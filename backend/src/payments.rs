use crate::config::{KiteConfig, KiteMode};
use serde::Serialize;
use sha2::{Digest, Sha256};

#[derive(Debug, Clone, Serialize)]
pub struct Settlement {
    pub rail: String,
    pub status: String,
    pub tx_ref: Option<String>,
    pub explorer_url: Option<String>,
    pub session_ref: Option<String>,
    pub approval_url: Option<String>,
    pub note: String,
}

pub async fn settle(
    cfg: &KiteConfig,
    payment_id: &str,
    amount_cents: i64,
    currency: &str,
    recipient: &str,
    milestone_title: &str,
) -> Settlement {
    match cfg.mode {
        KiteMode::Ledger => {
            let mut hasher = Sha256::new();
            hasher.update(payment_id.as_bytes());
            hasher.update(amount_cents.to_le_bytes());
            let hash = hex::encode(hasher.finalize());
            Settlement {
                rail: "ledger".into(),
                status: "settled".into(),
                tx_ref: Some(format!("ledger:{}", &hash[..20])),
                explorer_url: None,
                session_ref: None,
                approval_url: None,
                note: "Settled on the internal demo ledger. No on-chain transfer was made. Set KITE_MODE=passport and KITE_PAYOUT_COMMAND to settle through Kite Agent Passport.".into(),
            }
        }
        KiteMode::Passport => match cfg.payout_command.as_deref() {
            Some(command) if !command.trim().is_empty() => {
                run_payout_command(
                    command,
                    cfg,
                    payment_id,
                    amount_cents,
                    currency,
                    recipient,
                    milestone_title,
                )
                .await
            }
            _ => Settlement {
                rail: "kite_passport".into(),
                status: "failed".into(),
                tx_ref: None,
                explorer_url: None,
                session_ref: None,
                approval_url: None,
                note: "Kite Passport is selected but KITE_PAYOUT_COMMAND is not configured. The release is held for an operator.".into(),
            },
        },
    }
}

async fn run_payout_command(
    command: &str,
    cfg: &KiteConfig,
    payment_id: &str,
    amount_cents: i64,
    currency: &str,
    recipient: &str,
    milestone_title: &str,
) -> Settlement {
    let mut child = tokio::process::Command::new(command);
    child
        .env("TRANCHE_PAYMENT_ID", payment_id)
        .env("TRANCHE_AMOUNT_CENTS", amount_cents.to_string())
        .env("TRANCHE_CURRENCY", currency)
        .env("TRANCHE_RECIPIENT", recipient)
        .env("TRANCHE_MILESTONE", milestone_title)
        .env("TRANCHE_PAYEE", cfg.payee.clone().unwrap_or_default())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped());

    match tokio::time::timeout(std::time::Duration::from_secs(90), child.output()).await {
        Err(_) => Settlement {
            rail: "kite_passport".into(),
            status: "failed".into(),
            tx_ref: None,
            explorer_url: None,
            session_ref: None,
            approval_url: None,
            note: "Kite payout command timed out after 90 seconds.".into(),
        },
        Ok(Err(error)) => Settlement {
            rail: "kite_passport".into(),
            status: "failed".into(),
            tx_ref: None,
            explorer_url: None,
            session_ref: None,
            approval_url: None,
            note: format!("Kite payout command could not start: {error}"),
        },
        Ok(Ok(output)) => {
            let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
            let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
            if !output.status.success() {
                return Settlement {
                    rail: "kite_passport".into(),
                    status: "failed".into(),
                    tx_ref: None,
                    explorer_url: None,
                    session_ref: None,
                    approval_url: None,
                    note: format!(
                        "Kite payout exited with {}. {}",
                        output.status,
                        stderr.chars().take(400).collect::<String>()
                    ),
                };
            }
            parse_payout_output(&stdout, &cfg.chain)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pending_passport_send_is_not_treated_as_settled() {
        let out = r#"{"status":"human_action_required","request_id":"req_123","approval_url":"https://agentpassport.ai/approve/req_123"}"#;
        let settlement = parse_payout_output(out, "base");
        assert_eq!(settlement.status, "pending_approval");
        assert_eq!(settlement.session_ref.as_deref(), Some("req_123"));
        assert!(settlement.approval_url.is_some());
        assert!(settlement.tx_ref.is_none() || settlement.tx_ref.as_deref() == Some("req_123"));
    }

    #[test]
    fn completed_passport_send_records_tx() {
        let out = r#"{"status":"success","transactionHash":"0xabc123","explorer_url":"https://basescan.org/tx/0xabc123"}"#;
        let settlement = parse_payout_output(out, "base");
        assert_eq!(settlement.status, "settled");
        assert_eq!(settlement.tx_ref.as_deref(), Some("0xabc123"));
        assert!(settlement.explorer_url.is_some());
    }

    #[test]
    fn hash_only_passport_send_gets_a_base_explorer_link() {
        let hash = "0xe5f507ebceb3ca54f78ba5815b9e069e0703c022bd96a8426c2e570016bea301";
        let out = format!(r#"{{"status":"success","transaction_hash":"{hash}"}}"#);
        let settlement = parse_payout_output(&out, "base");
        assert_eq!(settlement.status, "settled");
        assert_eq!(
            settlement.explorer_url.as_deref(),
            Some("https://basescan.org/tx/0xe5f507ebceb3ca54f78ba5815b9e069e0703c022bd96a8426c2e570016bea301")
        );
    }
}

fn parse_payout_output(stdout: &str, chain: &str) -> Settlement {
    let receipt: serde_json::Value =
        serde_json::from_str(stdout).unwrap_or(serde_json::Value::Null);
    let approval_url = receipt
        .get("approval_url")
        .or_else(|| receipt.get("approvalUrl"))
        .and_then(serde_json::Value::as_str)
        .map(str::to_string);
    let request_id = receipt
        .get("request_id")
        .or_else(|| receipt.get("requestId"))
        .or_else(|| receipt.get("id"))
        .and_then(serde_json::Value::as_str)
        .map(str::to_string);
    let haystack = stdout.to_ascii_lowercase();
    let pending = approval_url.is_some()
        || haystack.contains("human_action_required")
        || haystack.contains("pending")
        || haystack.contains("awaiting approval");
    if pending {
        return Settlement {
            rail: "kite_passport".into(),
            status: "pending_approval".into(),
            tx_ref: request_id.clone(),
            explorer_url: None,
            session_ref: request_id,
            approval_url,
            note: "Payout created. Approve it with the operator passkey in Kite Passport, then check the approval status.".into(),
        };
    }
    let tx_ref = receipt
        .get("tx_ref")
        .or_else(|| receipt.get("txHash"))
        .or_else(|| receipt.get("tx_hash"))
        .or_else(|| receipt.get("transactionHash"))
        .or_else(|| receipt.get("transaction_hash"))
        .or_else(|| receipt.get("hash"))
        .and_then(serde_json::Value::as_str)
        .map(str::to_string)
        .or_else(|| {
            if stdout.is_empty() {
                None
            } else {
                Some(stdout.lines().next().unwrap_or("").to_string())
            }
        });
    let explorer_url = receipt
        .get("explorer_url")
        .and_then(serde_json::Value::as_str)
        .map(str::to_string)
        .or_else(|| tx_ref.as_deref().and_then(|tx| explorer_for(chain, tx)));
    Settlement {
        rail: "kite_passport".into(),
        status: "settled".into(),
        tx_ref,
        explorer_url,
        session_ref: request_id,
        approval_url: None,
        note: "Settled through the configured Kite Agent Passport payout command.".into(),
    }
}

/// Best-effort explorer link when the payout command only returns a hash.
fn explorer_for(chain: &str, tx_ref: &str) -> Option<String> {
    if tx_ref.len() != 66 || !tx_ref.starts_with("0x") {
        return None;
    }
    let base = match chain.to_ascii_lowercase().as_str() {
        "base" => "https://basescan.org/tx/",
        "polygon" => "https://polygonscan.com/tx/",
        "avalanche" => "https://snowtrace.io/tx/",
        _ => return None,
    };
    Some(format!("{base}{tx_ref}"))
}

/// Polls the configured Kite status command for a pending payout.
pub async fn poll_status(cfg: &KiteConfig, request_id: &str) -> Settlement {
    let Some(command) = cfg.status_command.as_deref().filter(|c| !c.trim().is_empty()) else {
        return Settlement {
            rail: "kite_passport".into(),
            status: "failed".into(),
            tx_ref: None,
            explorer_url: None,
            session_ref: Some(request_id.into()),
            approval_url: None,
            note: "KITE_STATUS_COMMAND is not configured; cannot poll the Passport approval.".into(),
        };
    };
    let mut child = tokio::process::Command::new(command);
    child
        .env("TRANCHE_REQUEST_ID", request_id)
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped());
    match tokio::time::timeout(std::time::Duration::from_secs(45), child.output()).await {
        Err(_) => Settlement {
            rail: "kite_passport".into(),
            status: "failed".into(),
            tx_ref: None,
            explorer_url: None,
            session_ref: Some(request_id.into()),
            approval_url: None,
            note: "Kite approval check timed out.".into(),
        },
        Ok(Err(error)) => Settlement {
            rail: "kite_passport".into(),
            status: "failed".into(),
            tx_ref: None,
            explorer_url: None,
            session_ref: Some(request_id.into()),
            approval_url: None,
            note: format!("Kite approval check could not start: {error}"),
        },
        Ok(Ok(output)) => {
            let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if !output.status.success() {
                return Settlement {
                    rail: "kite_passport".into(),
                    status: "failed".into(),
                    tx_ref: None,
                    explorer_url: None,
                    session_ref: Some(request_id.into()),
                    approval_url: None,
                    note: format!(
                        "Kite approval check exited with {}. {}",
                        output.status,
                        String::from_utf8_lossy(&output.stderr).chars().take(300).collect::<String>()
                    ),
                };
            }
            let mut parsed = parse_payout_output(&stdout, &cfg.chain);
            if parsed.status == "pending_approval" {
                parsed.session_ref = Some(request_id.into());
            }
            parsed
        }
    }
}
