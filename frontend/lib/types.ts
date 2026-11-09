export type MilestoneState =
  | "draft"
  | "open"
  | "submitted"
  | "verifying"
  | "needs_review"
  | "released"
  | "bounced";

export type NodeKind = "script" | "agent" | "external";

export interface GraphNode {
  id: string;
  kind: NodeKind;
  label: string;
  check?: string | null;
  tokens?: string[];
  hint?: string | null;
}

export interface AcceptanceGraph {
  version: number;
  criteria: string;
  nodes: GraphNode[];
}

export interface MilestoneSummary {
  id: string;
  title: string;
  builder: string;
  amount_cents: number;
  ceiling_cents: number;
  currency: string;
  state: MilestoneState;
  created_at: string;
  updated_at: string;
  program_name: string;
  submission_count: number;
  last_decision: string | null;
  last_payment_status: string | null;
  last_receipt_slug: string | null;
}

export interface EngineReport {
  serv: "live" | "unconfigured";
  serv_model: string;
  reppo: "live_lite" | "sandbox" | "unconfigured";
  payment_rail: "kite_passport" | "ledger";
  assurance_policy: {
    medium_may_release: boolean;
    fixture_may_release: boolean;
  };
}

export interface NodeRun {
  id: string;
  submission_id: string;
  node_id: string;
  seq: number;
  kind: NodeKind;
  label: string;
  status: "pending" | "running" | "pass" | "fail" | "warn" | "skipped" | "blocked";
  detail: string;
  evidence_url: string | null;
  cost_usd: number;
  latency_ms: number;
  output_json: string | null;
  created_at: string;
}

export interface Evidence {
  repo_url: string;
  demo_url: string;
  readme_url: string;
  claim: string;
  label?: string;
}

export interface VerdictView {
  id: string;
  serv: {
    decision: string;
    score: number;
    summary: string;
    reasons: string[];
    engine: string;
  };
  reppo: {
    eval_id: string | null;
    status: string;
    score: number | null;
    decision: string | null;
    confidence: string | null;
    critique: string | null;
    mode: string;
  };
  final_decision: "release" | "review" | "bounce";
  assurance: "high" | "medium" | "low";
  created_at: string;
}

export interface PaymentView {
  id: string;
  submission_id: string;
  amount_cents: number;
  currency: string;
  rail: string;
  status: string;
  tx_ref: string | null;
  explorer_url: string | null;
  session_ref: string | null;
  approval_url: string | null;
  note: string;
  created_at: string;
}

export interface EventView {
  id: string;
  milestone_id: string;
  kind: string;
  message: string;
  created_at: string;
}

export interface MilestoneDetail {
  milestone: {
    id: string;
    program_id: string;
    program_name: string;
    title: string;
    builder: string;
    criteria: string;
    amount_cents: number;
    ceiling_cents: number;
    currency: string;
    state: MilestoneState;
    created_at: string;
    updated_at: string;
  };
  graph: AcceptanceGraph;
  sample: Evidence | null;
  submission: {
    id: string;
    builder: string;
    evidence: Evidence;
    evidence_hash: string;
    created_at: string;
  } | null;
  history: Array<{
    submission_id: string;
    created_at: string;
    serv_decision: string;
    serv_score: number;
    final_decision: string;
  }>;
  node_runs: NodeRun[];
  verdict: VerdictView | null;
  payment: PaymentView | null;
  receipt_slug: string | null;
  events: EventView[];
  engines: EngineReport;
  public_base_url: string;
  policy: {
    auto_release_ceiling_cents: number;
    tranche_cents: number;
    currency: string;
  };
}

export interface MilestoneListResponse {
  milestones: MilestoneSummary[];
  engines: EngineReport;
}

export interface ReceiptResponse {
  receipt: { id: string; submission_id: string; slug: string; created_at: string };
  graph_hash: string;
  milestone: {
    id: string;
    title: string;
    builder: string;
    criteria: string;
    amount_cents: number;
    currency: string;
    program_name: string;
  };
  submission: {
    id: string;
    evidence: Evidence;
    evidence_hash: string;
    created_at: string;
  };
  graph: AcceptanceGraph;
  node_runs: NodeRun[];
  verdict: VerdictView | null;
  payment: PaymentView | null;
  engines: EngineReport;
  public_base_url: string;
}

export interface HealthResponse {
  ok: boolean;
  service: string;
  version: string;
  time: string;
  engines: EngineReport;
}
