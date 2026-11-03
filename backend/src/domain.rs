use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MilestoneState {
    Draft,
    Open,
    Submitted,
    Verifying,
    NeedsReview,
    Released,
    Bounced,
}

impl MilestoneState {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Draft => "draft",
            Self::Open => "open",
            Self::Submitted => "submitted",
            Self::Verifying => "verifying",
            Self::NeedsReview => "needs_review",
            Self::Released => "released",
            Self::Bounced => "bounced",
        }
    }

    pub fn from_str(value: &str) -> Self {
        match value {
            "open" => Self::Open,
            "submitted" => Self::Submitted,
            "verifying" => Self::Verifying,
            "needs_review" => Self::NeedsReview,
            "released" => Self::Released,
            "bounced" => Self::Bounced,
            _ => Self::Draft,
        }
    }

    pub fn accepts_submission(self) -> bool {
        matches!(
            self,
            Self::Draft | Self::Open | Self::Bounced | Self::NeedsReview
        )
    }
}

impl fmt::Display for MilestoneState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NodeKind {
    Script,
    Agent,
    External,
}

impl NodeKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Script => "script",
            Self::Agent => "agent",
            Self::External => "external",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphNode {
    pub id: String,
    pub kind: NodeKind,
    pub label: String,
    #[serde(default)]
    pub check: Option<String>,
    #[serde(default)]
    pub tokens: Vec<String>,
    #[serde(default)]
    pub hint: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AcceptanceGraph {
    pub version: u32,
    pub criteria: String,
    pub nodes: Vec<GraphNode>,
}

impl AcceptanceGraph {
    pub fn script_count(&self) -> usize {
        self.nodes.iter().filter(|n| n.kind == NodeKind::Script).count()
    }
}

/// Compiles plain-English criteria into a bounded acceptance graph.
///
/// This is deliberately deterministic: the shape of the process is code, not a
/// model output. SERV is used inside the Agent node for judgment, not for
/// deciding which checks exist. Operators can see and audit the graph before
/// any work is submitted.
pub fn compile_criteria(criteria: &str) -> AcceptanceGraph {
    let lower = criteria.to_ascii_lowercase();
    let mut tokens: Vec<String> = Vec::new();

    for keyword in ["serv", "kite", "reppo", "dolphin"] {
        if lower.contains(keyword) {
            tokens.push(keyword.to_string());
        }
    }

    let mut nodes = vec![
        GraphNode {
            id: "repo".into(),
            kind: NodeKind::Script,
            label: "Repository resolves and is public".into(),
            check: Some("repo_public".into()),
            tokens: vec![],
            hint: None,
        },
        GraphNode {
            id: "readme".into(),
            kind: NodeKind::Script,
            label: if tokens.is_empty() {
                "README is readable".into()
            } else {
                format!(
                    "README documents {}",
                    tokens
                        .iter()
                        .map(|t| title_case(t))
                        .collect::<Vec<_>>()
                        .join(" + ")
                )
            },
            check: Some("readme_mentions".into()),
            tokens,
            hint: None,
        },
        GraphNode {
            id: "demo".into(),
            kind: NodeKind::Script,
            label: "Live demo responds".into(),
            check: Some("demo_url_live".into()),
            tokens: vec![],
            hint: None,
        },
        GraphNode {
            id: "review".into(),
            kind: NodeKind::Agent,
            label: "Claim is supported by the evidence".into(),
            check: Some("criteria_review".into()),
            tokens: vec![],
            hint: Some(
                "The verdict must cite at least one concrete piece of evidence and must not accept a claim that the linked evidence does not support."
                    .into(),
            ),
        },
        GraphNode {
            id: "independent".into(),
            kind: NodeKind::External,
            label: "Independent verdict".into(),
            check: Some("reppo_eval".into()),
            tokens: vec![],
            hint: None,
        },
    ];

    // Optional stricter checks recognized from the criteria text.
    if lower.contains("commit") || lower.contains("during the hackathon") {
        nodes.insert(
            1,
            GraphNode {
                id: "commit".into(),
                kind: NodeKind::Script,
                label: "Repository has a recent commit".into(),
                check: Some("repo_recent_commit".into()),
                tokens: vec![],
                hint: None,
            },
        );
    }

    AcceptanceGraph {
        version: 1,
        criteria: criteria.trim().to_string(),
        nodes,
    }
}

fn title_case(value: &str) -> String {
    let mut chars = value.chars();
    match chars.next() {
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
        None => String::new(),
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Evidence {
    #[serde(default)]
    pub repo_url: String,
    #[serde(default)]
    pub demo_url: String,
    #[serde(default)]
    pub readme_url: String,
    #[serde(default)]
    pub claim: String,
}

impl Evidence {
    pub fn is_complete(&self) -> bool {
        !self.repo_url.trim().is_empty()
            && !self.demo_url.trim().is_empty()
            && self.claim.trim().len() >= 20
    }

    pub fn canonical_json(&self) -> String {
        serde_json::to_string(self).unwrap_or_default()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Assurance {
    High,
    Medium,
    Low,
}

impl Assurance {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::High => "high",
            Self::Medium => "medium",
            Self::Low => "low",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FinalDecision {
    Release,
    Review,
    Bounce,
}

impl FinalDecision {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Release => "release",
            Self::Review => "review",
            Self::Bounce => "bounce",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServVerdict {
    pub decision: String,
    pub score: i64,
    pub summary: String,
    pub reasons: Vec<String>,
}

impl ServVerdict {
    pub fn normalized_decision(&self) -> FinalDecision {
        match self.decision.to_ascii_lowercase().as_str() {
            "accept" | "release" => FinalDecision::Release,
            "reject" | "bounce" => FinalDecision::Bounce,
            _ => FinalDecision::Review,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReppoVerdict {
    pub eval_id: Option<String>,
    pub status: String,
    pub score: Option<i64>,
    pub decision: Option<String>,
    pub confidence: Option<String>,
    pub critique: Option<String>,
    pub mode: String,
}

impl ReppoVerdict {
    pub fn unavailable(mode: &str, reason: &str) -> Self {
        Self {
            eval_id: None,
            status: "unavailable".into(),
            score: None,
            decision: None,
            confidence: None,
            critique: Some(reason.into()),
            mode: mode.into(),
        }
    }

    pub fn rejected(&self) -> bool {
        self.mode == "live_lite"
            && self
                .decision
                .as_deref()
                .map(|d| d.eq_ignore_ascii_case("reject"))
                .unwrap_or(false)
    }

    pub fn accepted_live(&self) -> bool {
        self.mode == "live_lite"
            && self
                .decision
                .as_deref()
                .map(|d| d.eq_ignore_ascii_case("accept"))
                .unwrap_or(false)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphSummary {
    pub nodes: usize,
    pub script_nodes: usize,
    pub agent_nodes: usize,
    pub external_nodes: usize,
    pub estimated_review_cost_usd: f64,
}

impl AcceptanceGraph {
    pub fn summary(&self) -> GraphSummary {
        GraphSummary {
            nodes: self.nodes.len(),
            script_nodes: self.script_count(),
            agent_nodes: self.nodes.iter().filter(|n| n.kind == NodeKind::Agent).count(),
            external_nodes: self
                .nodes
                .iter()
                .filter(|n| n.kind == NodeKind::External)
                .count(),
            estimated_review_cost_usd: 0.004,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compiles_serv_and_kite_tokens() {
        let graph = compile_criteria("Public repo. README documents SERV Reasoning and Kite Passport.");
        let readme = graph.nodes.iter().find(|n| n.id == "readme").unwrap();
        assert_eq!(readme.tokens, vec!["serv", "kite"]);
        assert_eq!(graph.version, 1);
        assert!(graph.nodes.iter().any(|n| n.id == "review"));
    }

    #[test]
    fn adds_commit_check_when_asked() {
        let graph = compile_criteria("Repository must have a commit during the hackathon and a live demo.");
        assert!(graph.nodes.iter().any(|n| n.id == "commit"));
    }

    #[test]
    fn state_machine_accepts_revisions_after_bounce() {
        assert!(MilestoneState::Bounced.accepts_submission());
        assert!(!MilestoneState::Verifying.accepts_submission());
    }
}
