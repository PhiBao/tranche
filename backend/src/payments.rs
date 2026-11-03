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
            note: "Kite payout command timed out after 90 seconds.".into(),
        },
        Ok(Err(error)) => Settlement {
            rail: "kite_passport".into(),
            status: "failed".into(),
            tx_ref: None,
            explorer_url: None,
            session_ref: None,
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
                    note: format!(
                        "Kite payout exited with {}. {}",
                        output.status,
                        stderr.chars().take(400).collect::<String>()
                    ),
                };
            }
            let receipt: serde_json::Value =
                serde_json::from_str(&stdout).unwrap_or(serde_json::Value::Null);
            let tx_ref = receipt
                .get("tx_ref")
                .or_else(|| receipt.get("txHash"))
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
            Settlement {
                rail: "kite_passport".into(),
                status: "settled".into(),
                tx_ref,
                explorer_url: receipt
                    .get("explorer_url")
                    .and_then(serde_json::Value::as_str)
                    .map(str::to_string),
                session_ref: receipt
                    .get("session_ref")
                    .or_else(|| receipt.get("sessionId"))
                    .and_then(serde_json::Value::as_str)
                    .map(str::to_string),
                note: "Settled through the configured Kite Agent Passport payout command.".into(),
            }
        }
    }
}
