use std::path::PathBuf;

#[derive(Clone, Debug)]
pub struct Config {
    pub bind_addr: String,
    pub database_url: String,
    pub database_max_connections: u32,
    pub public_base_url: String,
    pub frontend_dir: Option<PathBuf>,
    pub serv: ServConfig,
    pub reppo: ReppoConfig,
    pub kite: KiteConfig,
    pub github_token: Option<String>,
    pub operator_token: Option<String>,
    pub allow_medium_assurance: bool,
    pub allow_fixture_release: bool,
    pub serv_daily_budget_usd: f64,
}

#[derive(Clone, Debug)]
pub struct ServConfig {
    pub api_key: Option<String>,
    pub base_url: String,
    pub model: String,
}

impl ServConfig {
    pub fn is_live(&self) -> bool {
        self.api_key.as_deref().map(|k| !k.trim().is_empty()).unwrap_or(false)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ReppoMode {
    LiveLite,
    Sandbox,
    Unconfigured,
}

#[derive(Clone, Debug)]
pub struct ReppoConfig {
    pub mode: ReppoMode,
    pub api_key: Option<String>,
    pub credit_token: Option<String>,
    pub base_url: String,
}

impl ReppoConfig {
    pub fn label(&self) -> &'static str {
        match self.mode {
            ReppoMode::LiveLite => "live_lite",
            ReppoMode::Sandbox => "sandbox",
            ReppoMode::Unconfigured => "unconfigured",
        }
    }

    pub fn is_live(&self) -> bool {
        self.mode == ReppoMode::LiveLite
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum KiteMode {
    Passport,
    Ledger,
}

#[derive(Clone, Debug)]
pub struct KiteConfig {
    pub mode: KiteMode,
    pub payee: Option<String>,
    pub payout_command: Option<String>,
}

impl KiteConfig {
    pub fn label(&self) -> &'static str {
        match self.mode {
            KiteMode::Passport => "kite_passport",
            KiteMode::Ledger => "ledger",
        }
    }
}

fn env_or(key: &str, default: &str) -> String {
    std::env::var(key).unwrap_or_else(|_| default.to_string())
}

fn env_opt(key: &str) -> Option<String> {
    match std::env::var(key) {
        Ok(v) if !v.trim().is_empty() => Some(v),
        _ => None,
    }
}

fn env_bool(key: &str, default: bool) -> bool {
    match std::env::var(key) {
        Ok(v) => matches!(v.trim().to_ascii_lowercase().as_str(), "1" | "true" | "yes" | "on"),
        Err(_) => default,
    }
}

impl Config {
    pub fn from_env() -> Self {
        let _ = dotenvy::dotenv();

        let reppo_api_key = env_opt("REPPO_API_KEY");
        let reppo_credit_token = env_opt("REPPO_CREDIT_TOKEN");
        let requested = env_or("REPPO_MODE", "auto").to_ascii_lowercase();

        let reppo_mode = match requested.as_str() {
            "live" | "live_lite" => {
                if reppo_credit_token.is_some() {
                    ReppoMode::LiveLite
                } else {
                    // The standard API path settles in 10+ minutes; the demo cannot wait.
                    // Treat it as configured-but-not-inline and fall back to sandbox honestly.
                    ReppoMode::Sandbox
                }
            }
            "sandbox" => ReppoMode::Sandbox,
            "off" | "unconfigured" | "none" => ReppoMode::Unconfigured,
            _ => {
                if reppo_credit_token.is_some() {
                    ReppoMode::LiveLite
                } else if reppo_api_key.is_some() {
                    ReppoMode::Sandbox
                } else {
                    ReppoMode::Unconfigured
                }
            }
        };

        let kite_requested = env_or("KITE_MODE", "ledger").to_ascii_lowercase();
        let kite_mode = if kite_requested == "passport" {
            KiteMode::Passport
        } else {
            KiteMode::Ledger
        };

        let frontend_dir = env_opt("FRONTEND_DIR").map(PathBuf::from);

        Self {
            bind_addr: env_or("BIND_ADDR", "0.0.0.0:8080"),
            database_url: env_or("DATABASE_URL", "sqlite://data/tranche.db?mode=rwc"),
            database_max_connections: env_or("DATABASE_MAX_CONNECTIONS", "5")
                .parse()
                .unwrap_or(5),
            public_base_url: env_or("PUBLIC_BASE_URL", "http://localhost:8080")
                .trim_end_matches('/')
                .to_string(),
            frontend_dir,
            serv: ServConfig {
                api_key: env_opt("SERV_API_KEY"),
                base_url: env_or("SERV_BASE_URL", "https://inference-api.openserv.ai/v1")
                    .trim_end_matches('/')
                    .to_string(),
                model: env_or("SERV_MODEL", "gpt-5.4-mini"),
            },
            reppo: ReppoConfig {
                mode: reppo_mode,
                api_key: reppo_api_key,
                credit_token: reppo_credit_token,
                base_url: env_or("REPPO_BASE_URL", "https://eval.reppo.xyz")
                    .trim_end_matches('/')
                    .to_string(),
            },
            kite: KiteConfig {
                mode: kite_mode,
                payee: env_opt("KITE_PAYEE"),
                payout_command: env_opt("KITE_PAYOUT_COMMAND"),
            },
            github_token: env_opt("GITHUB_TOKEN"),
            operator_token: env_opt("OPERATOR_TOKEN"),
            allow_medium_assurance: env_bool("ALLOW_MEDIUM_ASSURANCE", true),
            allow_fixture_release: env_bool("ALLOW_FIXTURE_RELEASE", false),
            serv_daily_budget_usd: env_or("SERV_DAILY_BUDGET_USD", "2.0")
                .parse()
                .unwrap_or(2.0),
        }
    }

    pub fn engine_report(&self) -> serde_json::Value {
        serde_json::json!({
            "serv": if self.serv.is_live() { "live" } else { "unconfigured" },
            "serv_model": self.serv.model,
            "reppo": self.reppo.label(),
            "payment_rail": self.kite.label(),
            "assurance_policy": {
                "medium_may_release": self.allow_medium_assurance,
                "fixture_may_release": self.allow_fixture_release
            }
        })
    }
}
