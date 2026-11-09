use crate::config::{ReppoConfig, ReppoMode};
use crate::domain::ReppoVerdict;
use reqwest::Client;
use serde_json::{json, Value};
use std::time::Duration;

const POLL_INTERVAL: Duration = Duration::from_secs(4);
const MAX_POLLS: u32 = 45; // ~180s, above the documented 30-90s settle window

pub async fn evaluate(
    cfg: &ReppoConfig,
    http: &Client,
    milestone_title: &str,
    criteria: &str,
    trace: &str,
) -> ReppoVerdict {
    match cfg.mode {
        ReppoMode::Unconfigured => ReppoVerdict::unavailable(
            "unconfigured",
            "Reppo is not configured. The independent verdict is missing, so assurance is reduced.",
        ),
        ReppoMode::Sandbox => ReppoVerdict::unavailable(
            "sandbox",
            "Reppos sandbox endpoint was retired in the current API. Configure REPPO_CREDIT_TOKEN for a live independent verdict. This node is advisory only and never gates a payment.",
        ),
        ReppoMode::LiveLite => live(cfg, http, milestone_title, criteria, trace).await,
    }
}

async fn live(
    cfg: &ReppoConfig,
    http: &Client,
    _milestone_title: &str,
    criteria: &str,
    trace: &str,
) -> ReppoVerdict {
    let Some(token) = cfg.credit_token.as_deref() else {
        return ReppoVerdict::unavailable(
            "live_lite",
            "REPPO_CREDIT_TOKEN is missing; the independent verdict cannot be requested.",
        );
    };

    let input = format!(
        "You are an independent evaluator for a milestone payment. Judge the CLAIM against the evidence, not the prose style of the response.\n\nMilestone criteria:\n{criteria}\n\nEvidence bundle (JSON):\n{trace}\n\nQuestion: Does the evidence support the claim and satisfy the milestone criteria? Return a verdict on the claim."
    );
    let truncated = input.chars().take(48_000).collect::<String>();
    let payload = json!({
        "taskType": "claim",
        "input": truncated,
    });

    let submit_url = format!("{}/api/v1/x402/evals", cfg.base_url);
    let response = http
        .post(&submit_url)
        .bearer_auth(token)
        .timeout(Duration::from_secs(30))
        .json(&payload)
        .send()
        .await;

    let response = match response {
        Ok(response) => response,
        Err(error) => {
            return ReppoVerdict {
                eval_id: None,
                status: "error".into(),
                score: None,
                decision: None,
                confidence: None,
                critique: Some(format!("Reppo submit failed: {error}")),
                mode: "live_lite".into(),
            }
        }
    };

    let status = response.status();
    let raw = response.text().await.unwrap_or_default();
    if !status.is_success() {
        return ReppoVerdict {
            eval_id: None,
            status: "error".into(),
            score: None,
            decision: None,
            confidence: None,
            critique: Some(format!(
                "Reppo returned HTTP {status}: {}",
                raw.chars().take(300).collect::<String>()
            )),
            mode: "live_lite".into(),
        };
    }

    let submitted: Value = match serde_json::from_str(&raw) {
        Ok(value) => value,
        Err(error) => {
            return ReppoVerdict {
                eval_id: None,
                status: "error".into(),
                score: None,
                decision: None,
                confidence: None,
                critique: Some(format!("Reppo returned invalid JSON: {error}")),
                mode: "live_lite".into(),
            }
        }
    };
    let eval_id = submitted
        .get("id")
        .and_then(Value::as_str)
        .map(str::to_string);
    let Some(eval_id) = eval_id else {
        return ReppoVerdict {
            eval_id: None,
            status: "error".into(),
            score: None,
            decision: None,
            confidence: None,
            critique: Some("Reppo submit did not return an evaluation id.".into()),
            mode: "live_lite".into(),
        };
    };

    let poll_url = format!("{}/api/v1/x402/evals/{eval_id}", cfg.base_url);
    for _ in 0..MAX_POLLS {
        tokio::time::sleep(POLL_INTERVAL).await;
        let poll = http
            .get(&poll_url)
            .bearer_auth(token)
            .timeout(Duration::from_secs(30))
            .send()
            .await;
        let raw = match poll {
            Ok(response) => response.text().await.unwrap_or_default(),
            Err(_) => continue,
        };
        let Ok(value) = serde_json::from_str::<Value>(&raw) else {
            continue;
        };
        let state = value
            .get("status")
            .and_then(Value::as_str)
            .unwrap_or("")
            .to_ascii_uppercase();
        match state.as_str() {
            "COMPLETED" => return parse_completed(&value, &eval_id),
            "FAILED" => {
                return ReppoVerdict {
                    eval_id: Some(eval_id),
                    status: "failed".into(),
                    score: None,
                    decision: None,
                    confidence: None,
                    critique: Some("The evaluation failed and was not charged. Resubmit to try again.".into()),
                    mode: "live_lite".into(),
                }
            }
            _ => continue,
        }
    }

    ReppoVerdict {
        eval_id: Some(eval_id),
        status: "pending".into(),
        score: None,
        decision: None,
        confidence: None,
        critique: Some("The evaluation is still queued after ~2 minutes. It was not charged; a refresh will not recover it — resubmit to retry.".into()),
        mode: "live_lite".into(),
    }
}

/// One-shot poll for a previously submitted evaluation. Used by the refresh
/// endpoint when a verdict was still queued at the end of the inline window.
pub async fn poll_once(cfg: &ReppoConfig, http: &Client, eval_id: &str) -> Option<ReppoVerdict> {
    let Some(token) = cfg.credit_token.as_deref() else {
        return None;
    };
    let url = format!("{}/api/v1/x402/evals/{eval_id}", cfg.base_url);
    let response = http
        .get(&url)
        .bearer_auth(token)
        .timeout(Duration::from_secs(30))
        .send()
        .await
        .ok()?;
    let raw = response.text().await.ok()?;
    let value: Value = serde_json::from_str(&raw).ok()?;
    let state = value
        .get("status")
        .and_then(Value::as_str)
        .unwrap_or("")
        .to_ascii_uppercase();
    match state.as_str() {
        "COMPLETED" => Some(parse_completed(&value, eval_id)),
        "FAILED" => Some(ReppoVerdict {
            eval_id: Some(eval_id.to_string()),
            status: "failed".into(),
            score: None,
            decision: None,
            confidence: None,
            critique: Some("The evaluation failed and was not charged.".into()),
            mode: "live_lite".into(),
        }),
        _ => None,
    }
}

fn parse_completed(value: &Value, eval_id: &str) -> ReppoVerdict {
    let result = value.get("result").cloned().unwrap_or(Value::Null);
    let score = result
        .get("score")
        .and_then(Value::as_i64)
        .or_else(|| value.get("score").and_then(Value::as_i64));
    let decision = result
        .get("decision")
        .and_then(Value::as_str)
        .map(|d| d.to_ascii_lowercase());
    let confidence = result
        .get("confidence")
        .and_then(Value::as_str)
        .map(str::to_string);
    let route = result
        .get("route")
        .and_then(Value::as_str)
        .map(str::to_string);
    let critique = result
        .get("reason")
        .and_then(Value::as_str)
        .map(str::to_string)
        .or_else(|| {
            result
                .get("explain")
                .and_then(|explain| explain.get("route_reasons"))
                .and_then(Value::as_array)
                .map(|items| {
                    items
                        .iter()
                        .filter_map(|item| item.as_str())
                        .collect::<Vec<_>>()
                        .join(" ")
                })
        });

    ReppoVerdict {
        eval_id: Some(eval_id.to_string()),
        status: "settled".into(),
        score,
        decision: match decision.as_deref() {
            Some("accept") => Some("accept".into()),
            Some("reject") => Some("reject".into()),
            _ => match route.as_deref() {
                Some("ship") => Some("accept".into()),
                Some("block") => Some("reject".into()),
                _ => Some("revise".into()),
            },
        },
        confidence,
        critique,
        mode: "live_lite".into(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_completed_eval() {
        let value = json!({
            "status": "COMPLETED",
            "result": {
                "score": 82,
                "decision": "accept",
                "confidence": "high",
                "route": "ship",
                "reason": "Evidence supports the claim."
            }
        });
        let verdict = parse_completed(&value, "eval_1");
        assert_eq!(verdict.eval_id.as_deref(), Some("eval_1"));
        assert_eq!(verdict.score, Some(82));
        assert_eq!(verdict.decision.as_deref(), Some("accept"));
        assert!(verdict.accepted_live());
    }

    #[test]
    fn block_route_maps_to_reject() {
        let value = json!({
            "status": "COMPLETED",
            "result": {"score": 12, "route": "block", "reason": "Unsupported numbers."}
        });
        let verdict = parse_completed(&value, "eval_2");
        assert_eq!(verdict.decision.as_deref(), Some("reject"));
        assert!(verdict.rejected());
    }
}
