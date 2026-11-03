use crate::config::{ReppoConfig, ReppoMode};
use crate::domain::ReppoVerdict;
use reqwest::Client;
use serde_json::{json, Value};

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
        ReppoMode::Sandbox => sandbox(cfg, http, milestone_title, criteria, trace).await,
        ReppoMode::LiveLite => live_lite(cfg, http, milestone_title, criteria, trace).await,
    }
}

async fn live_lite(
    cfg: &ReppoConfig,
    http: &Client,
    milestone_title: &str,
    criteria: &str,
    trace: &str,
) -> ReppoVerdict {
    let Some(token) = cfg.credit_token.as_deref() else {
        return ReppoVerdict::unavailable(
            "live_lite",
            "REPPO_CREDIT_TOKEN is missing; the independent verdict cannot be requested.",
        );
    };

    let payload = build_payload(milestone_title, criteria, trace);
    let url = format!("{}/api/v1/x402/evals/lite", cfg.base_url);
    let response = http
        .post(&url)
        .bearer_auth(token)
        .timeout(std::time::Duration::from_secs(150))
        .json(&payload)
        .send()
        .await;

    match response {
        Ok(response) => {
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
            parse_live(&raw)
        }
        Err(error) => ReppoVerdict {
            eval_id: None,
            status: "error".into(),
            score: None,
            decision: None,
            confidence: None,
            critique: Some(format!("Reppo request failed: {error}")),
            mode: "live_lite".into(),
        },
    }
}

async fn sandbox(
    cfg: &ReppoConfig,
    http: &Client,
    milestone_title: &str,
    criteria: &str,
    trace: &str,
) -> ReppoVerdict {
    let payload = build_payload(milestone_title, criteria, trace);
    let url = format!("{}/api/v1/x402/evals/lite", cfg.base_url);
    let response = http
        .post(&url)
        .header("Sandbox", "true")
        .timeout(std::time::Duration::from_secs(60))
        .json(&payload)
        .send()
        .await;

    match response {
        Ok(response) => {
            let status = response.status();
            let raw = response.text().await.unwrap_or_default();
            let eval_id = serde_json::from_str::<Value>(&raw)
                .ok()
                .and_then(|v| {
                    v.get("evalId")
                        .or_else(|| v.get("id"))
                        .and_then(Value::as_str)
                        .map(str::to_string)
                });
            if !status.is_success() {
                return ReppoVerdict {
                    eval_id,
                    status: "error".into(),
                    score: None,
                    decision: None,
                    confidence: None,
                    critique: Some(format!(
                        "Reppo sandbox returned HTTP {status}: {}",
                        raw.chars().take(300).collect::<String>()
                    )),
                    mode: "sandbox".into(),
                };
            }
            ReppoVerdict {
                eval_id,
                status: "sandbox".into(),
                score: Some(0),
                decision: Some("revise".into()),
                confidence: Some("placeholder".into()),
                critique: Some(
                    "Reppo sandbox mode returns a placeholder verdict. It is shown for integration proof and never gates a payment."
                        .into(),
                ),
                mode: "sandbox".into(),
            }
        }
        Err(error) => ReppoVerdict {
            eval_id: None,
            status: "error".into(),
            score: None,
            decision: None,
            confidence: None,
            critique: Some(format!("Reppo sandbox request failed: {error}")),
            mode: "sandbox".into(),
        },
    }
}

fn build_payload(milestone_title: &str, criteria: &str, trace: &str) -> Value {
    let truncated = trace.chars().take(48_000).collect::<String>();
    json!({
        "type": "trace",
        "payload": truncated,
        "criteria": [criteria.chars().take(1900).collect::<String>()],
        "context": format!("Milestone: {milestone_title}. This is an acceptance decision for a milestone payment. The verdict must judge whether the submitted evidence supports the claim and the milestone criteria.")
    })
}

fn parse_live(raw: &str) -> ReppoVerdict {
    let parsed: Value = match serde_json::from_str(raw) {
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

    let eval_id = parsed
        .get("evalId")
        .or_else(|| parsed.get("id"))
        .and_then(Value::as_str)
        .map(str::to_string);
    let status = parsed
        .get("status")
        .and_then(Value::as_str)
        .unwrap_or("unknown")
        .to_ascii_lowercase();
    let result = parsed.get("result").cloned().unwrap_or(Value::Null);
    let verdict_obj = result.get("verdict").unwrap_or(&result);

    let score = verdict_obj
        .get("score")
        .and_then(Value::as_i64)
        .or_else(|| result.get("score").and_then(Value::as_i64));
    let decision = verdict_obj
        .get("decision")
        .and_then(Value::as_str)
        .or_else(|| result.get("decision").and_then(Value::as_str))
        .map(|s| s.to_ascii_lowercase());
    let confidence = verdict_obj
        .get("confidence")
        .and_then(Value::as_str)
        .map(str::to_string);

    let critique = result
        .get("critiques")
        .and_then(Value::as_array)
        .map(|items| {
            items
                .iter()
                .filter_map(|item| {
                    item.get("critique")
                        .or_else(|| item.get("summary"))
                        .and_then(Value::as_str)
                })
                .collect::<Vec<_>>()
                .join(" ")
        })
        .filter(|s| !s.is_empty())
        .or_else(|| {
            verdict_obj
                .get("critique")
                .and_then(Value::as_str)
                .map(str::to_string)
        });

    ReppoVerdict {
        eval_id,
        status,
        score,
        decision,
        confidence,
        critique,
        mode: "live_lite".into(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_settled_lite_response() {
        let raw = r#"{"evalId":"eval_1","status":"SETTLED","result":{"score":88,"decision":"accept","confidence":"high","critiques":[{"critique":"Evidence supports the claim."}]}}"#;
        let verdict = parse_live(raw);
        assert_eq!(verdict.eval_id.as_deref(), Some("eval_1"));
        assert_eq!(verdict.score, Some(88));
        assert_eq!(verdict.decision.as_deref(), Some("accept"));
        assert!(verdict.accepted_live());
    }
}
