"use client";

import Link from "next/link";
import { useSearchParams } from "next/navigation";
import { Suspense, useCallback, useEffect, useState } from "react";
import { EngineStrip } from "@/components/EngineStrip";
import { Stamp, StateStamp } from "@/components/Stamp";
import { StatusMark, statusText, type Status } from "@/components/StatusMark";
import { getJSON, postJSON } from "@/lib/api";
import {
  dateTime,
  decisionLabel,
  money,
  shortHash,
} from "@/lib/format";
import type { Evidence, MilestoneDetail, NodeKind } from "@/lib/types";

const kindLabel: Record<NodeKind, string> = {
  script: "Script",
  agent: "Agent · SERV",
  external: "External · Reppo",
};

function defaultDetail(kind: NodeKind): string {
  switch (kind) {
    case "script":
      return "Deterministic check. Runs as code, with no model cost.";
    case "agent":
      return "SERV Reasoning reviews the claim against the criteria. PromptGuard screens the evidence first.";
    case "external":
      return "Reppo judges the verification trace independently and returns a confidence score.";
  }
}

function paymentStamp(status: string) {
  switch (status) {
    case "settled":
      return { label: "Settled", tone: "green" as const };
    case "blocked":
      return { label: "Blocked", tone: "red" as const };
    case "held":
      return { label: "Held", tone: "amber" as const };
    case "failed":
      return { label: "Failed", tone: "red" as const };
    default:
      return { label: "Pending", tone: "amber" as const };
  }
}

function MilestoneInner() {
  const params = useSearchParams();
  const id = params.get("id") ?? "";
  const [detail, setDetail] = useState<MilestoneDetail | null>(null);
  const [error, setError] = useState("");
  const [actionError, setActionError] = useState("");
  const [notice, setNotice] = useState("");
  const [busy, setBusy] = useState<string | null>(null);
  const [showForm, setShowForm] = useState(false);
  const [operatorToken, setOperatorToken] = useState("");
  const [form, setForm] = useState<Evidence>({
    repo_url: "",
    demo_url: "",
    readme_url: "",
    claim: "",
  });

  const load = useCallback(
    async (quiet = false) => {
      if (!id) return;
      try {
        const data = await getJSON<MilestoneDetail>(`/api/milestones/${id}`);
        setDetail(data);
        setError("");
      } catch (err) {
        if (!quiet) setError(err instanceof Error ? err.message : "Could not load milestone.");
      }
    },
    [id],
  );

  useEffect(() => {
    load();
  }, [load]);

  const state = detail?.milestone.state;

  useEffect(() => {
    if (state !== "verifying" && state !== "submitted") return;
    const timer = window.setInterval(() => load(true), 1400);
    return () => window.clearInterval(timer);
  }, [state, load]);

  function updateForm(key: keyof Evidence, value: string) {
    setForm((current) => ({ ...current, [key]: value }));
  }

  function useSample() {
    if (!detail?.sample) return;
    setForm({
      repo_url: detail.sample.repo_url,
      demo_url: detail.sample.demo_url,
      readme_url: detail.sample.readme_url,
      claim: detail.sample.claim,
    });
    setShowForm(true);
  }

  async function runSample() {
    setBusy("sample");
    setActionError("");
    setNotice("");
    try {
      await postJSON(`/api/milestones/${id}/sample`);
      setNotice("Sample submitted. The acceptance graph is running.");
      await load(true);
    } catch (err) {
      setActionError(err instanceof Error ? err.message : "Could not run the sample.");
    } finally {
      setBusy(null);
    }
  }

  async function submitEvidence() {
    setBusy("submit");
    setActionError("");
    setNotice("");
    try {
      const created = await postJSON<{ submission_id: string }>(
        `/api/milestones/${id}/submit`,
        form,
      );
      await postJSON(`/api/submissions/${created.submission_id}/verify`);
      setNotice("Evidence submitted. The acceptance graph is running.");
      setShowForm(false);
      await load(true);
    } catch (err) {
      setActionError(err instanceof Error ? err.message : "Could not submit evidence.");
    } finally {
      setBusy(null);
    }
  }

  async function verifyExisting() {    if (!detail?.submission) return;
    setBusy("verify");
    setActionError("");
    try {
      await postJSON(`/api/submissions/${detail.submission.id}/verify`);
      await load(true);
    } catch (err) {
      setActionError(err instanceof Error ? err.message : "Could not start verification.");
    } finally {
      setBusy(null);
    }
  }

  async function refreshReppo() {
    setBusy("refresh");
    setActionError("");
    try {
      await postJSON(`/api/milestones/${id}/refresh`);
      await load(true);
    } catch (err) {
      setActionError(err instanceof Error ? err.message : "Could not refresh the Reppo verdict.");
    } finally {
      setBusy(null);
    }
  }

  async function approve() {    if (!detail?.payment) return;
    const ok = window.confirm(
      "Approve this held release? The settlement rail will be instructed immediately.",
    );
    if (!ok) return;
    setBusy("approve");
    setActionError("");
    try {
      const headers = operatorToken.trim()
        ? { "X-Operator-Token": operatorToken.trim() }
        : undefined;
      await postJSON(`/api/payments/${detail.payment.id}/approve`, undefined, headers);
      setOperatorToken("");
      await load(true);
    } catch (err) {
      setActionError(err instanceof Error ? err.message : "Could not approve the release.");
    } finally {
      setBusy(null);
    }
  }

  if (!id) {
    return (
      <div className="wrap loading-block">
        No milestone selected. <Link href="/console">Back to the console.</Link>
      </div>
    );
  }

  if (error) {
    return (
      <div className="wrap" style={{ paddingTop: "48px" }}>
        <div className="error-box">{error}</div>
        <Link className="text-link" href="/console">
          ← Back to the console
        </Link>
      </div>
    );
  }

  if (!detail) {
    return <div className="wrap loading-block">Loading the release room…</div>;
  }

  const { milestone, graph } = detail;
  const submission = detail.submission;
  const canSubmit =
    milestone.state === "open" ||
    milestone.state === "bounced" ||
    milestone.state === "needs_review" ||
    milestone.state === "draft";
  const payment = detail.payment;
  const paymentInfo = payment ? paymentStamp(payment.status) : null;

  return (
    <div className="detail-page">
      <div className="wrap">
        <div className="breadcrumb">
          <Link href="/console">Console</Link>
          <span>/</span>
          <span>{milestone.program_name}</span>
          <span>/</span>
          <span>{milestone.id}</span>
        </div>

        <div className="detail-top">
          <div>
            <div className="page-kicker">Milestone · {milestone.builder}</div>
            <h1 className="detail-title">{milestone.title}</h1>
            <div className="detail-sub">
              <span>Tranche {money(milestone.amount_cents, milestone.currency)}</span>
              <span>Ceiling {money(milestone.ceiling_cents, milestone.currency)}</span>
              <span>{milestone.program_name}</span>
              <span>Opened {dateTime(milestone.created_at)}</span>
            </div>
          </div>
          <StateStamp state={milestone.state} />
        </div>

        <div className="detail-grid">
          <div className="detail-main">
            {actionError ? <div className="error-box">{actionError}</div> : null}
            {notice ? <div className="success-box">{notice}</div> : null}

            <section className="block">
              <div className="block-head">
                <h2 className="block-title">Acceptance graph</h2>
                <span className="block-note">
                  v{graph.version} · {graph.nodes.length} nodes · compiled before evidence
                </span>
              </div>
              <div className="graph">
                <div className="graph-head">
                  <span>Acceptance graph · v{graph.version}</span>
                  <span>{detail.node_runs.length ? "RUNNING / COMPLETE" : "AWAITING EVIDENCE"}</span>
                </div>
                <div className="graph-criteria">
                  <span>Criteria agreed before work</span>
                  {graph.criteria}
                </div>
                {graph.nodes.map((node, index) => {
                  const run = detail.node_runs.find((item) => item.node_id === node.id);
                  const status = (run?.status ?? "pending") as Status;
                  return (
                    <div className={`graph-node ${status}`} key={node.id}>
                      <span className="idx">{String(index + 1).padStart(2, "0")}</span>
                      <div>
                        <span className={`kind-tag ${node.kind}`}>{kindLabel[node.kind]}</span>
                        <div className="label">{node.label}</div>
                        <div className="detail">
                          {run?.detail || defaultDetail(node.kind)}
                          {run?.evidence_url ? (
                            <>
                              {" "}
                              <a href={run.evidence_url} target="_blank" rel="noreferrer">
                                open evidence ↗
                              </a>
                            </>
                          ) : null}
                        </div>
                      </div>
                      <span className="status">
                        <StatusMark status={status} />
                        {statusText(status)}
                      </span>
                      <span className="metrics">
                        {run && run.latency_ms > 0 ? `${run.latency_ms} ms` : "—"}
                        <br />
                        {run && run.cost_usd > 0
                          ? `$${run.cost_usd.toFixed(4)}`
                          : "no model cost"}
                      </span>
                    </div>
                  );
                })}
              </div>
            </section>

            <section className="block">
              <div className="block-head">
                <h2 className="block-title">Submission</h2>
                <span className="block-note">
                  {submission ? `Hash ${shortHash(submission.evidence_hash)}` : "No evidence yet"}
                </span>
              </div>

              {!submission ? (
                <>
                  {detail.sample ? (
                    <div className="sample-box">
                      <div>
                        <strong style={{ fontSize: "12px" }}>{detail.sample.label}</strong>
                        <p>
                          A seeded submission is attached to this milestone for the
                          demo. Running it submits the evidence and starts the graph.
                        </p>
                      </div>
                      <div style={{ display: "flex", gap: "8px", flexWrap: "wrap" }}>
                        <button
                          className="button small"
                          onClick={runSample}
                          disabled={busy !== null}
                        >
                          {busy === "sample" ? "Running…" : "Run seeded sample"}
                        </button>
                        <button className="button small ghost" onClick={useSample}>
                          Edit sample
                        </button>
                      </div>
                    </div>
                  ) : null}
                  {!showForm ? (
                    <button className="button ghost" onClick={() => setShowForm(true)}>
                      Submit evidence manually
                    </button>
                  ) : null}
                </>
              ) : (
                <div className="evidence-card">
                  <div className="claim">{submission.evidence.claim}</div>
                  <div className="evidence-links">
                    <div>
                      <span>Repository</span>
                      <a href={submission.evidence.repo_url} target="_blank" rel="noreferrer">
                        {submission.evidence.repo_url}
                      </a>
                    </div>
                    <div>
                      <span>Live demo</span>
                      <a href={submission.evidence.demo_url} target="_blank" rel="noreferrer">
                        {submission.evidence.demo_url}
                      </a>
                    </div>
                    {submission.evidence.readme_url ? (
                      <div>
                        <span>README</span>
                        <a href={submission.evidence.readme_url} target="_blank" rel="noreferrer">
                          {submission.evidence.readme_url}
                        </a>
                      </div>
                    ) : null}
                    <div>
                      <span>Submitted</span>
                      <span style={{ color: "var(--ink-soft)" }}>
                        {dateTime(submission.created_at)} by {submission.builder}
                      </span>
                    </div>
                  </div>
                  <div className="hash-line">SHA-256 · {submission.evidence_hash}</div>
                </div>
              )}

              {submission && milestone.state === "submitted" ? (
                <div className="form-actions">
                  <button
                    className="button"
                    onClick={verifyExisting}
                    disabled={busy !== null}
                  >
                    {busy === "verify" ? "Starting…" : "Run verification"}
                  </button>
                </div>
              ) : null}

              {milestone.state === "verifying" ? (
                <div className="notice amber" style={{ marginTop: "16px" }}>
                  The acceptance graph is running. Nodes update live. Do not close
                  this page if you are recording the demo.
                </div>
              ) : null}

              {canSubmit && submission ? (
                <div style={{ marginTop: "18px" }}>
                  {!showForm ? (
                    <button className="button ghost" onClick={() => setShowForm(true)}>
                      {detail.verdict?.serv.decision === "revise"
                        ? "Revise and resubmit"
                        : "Submit a revision"}
                    </button>
                  ) : null}
                  {detail.history && detail.history.length > 0 ? (
                    <div className="notice" style={{ marginTop: "12px" }}>
                      Previous attempts:{" "}
                      {detail.history
                        .map(
                          (attempt) =>
                            `${attempt.serv_decision} ${attempt.serv_score}/100 (${attempt.final_decision})`,
                        )
                        .join(" → ")}
                      {detail.history.length > 1
                        ? `. Latest: ${detail.history[detail.history.length - 1].serv_score}/100. Address the verdict reasons and resubmit to see the score move.`
                        : ". Each resubmission is scored again; the score history is kept on the receipt."}
                    </div>
                  ) : null}
                </div>
              ) : null}
            </section>

            {showForm ? (
              <section className="block">
                <div className="form">
                  <div className="form-title">Submit evidence</div>
                  <div className="form-sub">
                    The submission is hashed before verification. Hostile text is
                    treated as hostile text.
                  </div>
                  <div className="form-grid">
                    <div className="field">
                      <label htmlFor="repo">Repository URL</label>
                      <input
                        id="repo"
                        value={form.repo_url}
                        onChange={(e) => updateForm("repo_url", e.target.value)}
                        placeholder="https://github.com/owner/repo"
                      />
                    </div>
                    <div className="field">
                      <label htmlFor="demo">Live demo URL</label>
                      <input
                        id="demo"
                        value={form.demo_url}
                        onChange={(e) => updateForm("demo_url", e.target.value)}
                        placeholder="https://your-demo.example"
                      />
                    </div>
                  </div>
                  <div className="field">
                    <label htmlFor="readme">README URL (optional)</label>
                    <input
                      id="readme"
                      value={form.readme_url}
                      onChange={(e) => updateForm("readme_url", e.target.value)}
                      placeholder="Leave empty to use the repository README"
                    />
                  </div>
                  <div className="field">
                    <label htmlFor="claim">What did you ship?</label>
                    <textarea
                      id="claim"
                      value={form.claim}
                      onChange={(e) => updateForm("claim", e.target.value)}
                      placeholder="Describe the work in concrete terms. The claim is judged against the evidence, not on its own."
                    />
                    <div className="hint">
                      Minimum 20 characters. The claim is untrusted input; do not
                      paste instructions meant for the reviewer.
                    </div>
                  </div>
                  <div className="form-actions">
                    <button
                      className="button"
                      onClick={submitEvidence}
                      disabled={busy !== null}
                    >
                      {busy === "submit" ? "Submitting…" : "Submit and verify"}
                    </button>
                    <button
                      className="button ghost"
                      onClick={() => setShowForm(false)}
                      disabled={busy !== null}
                    >
                      Cancel
                    </button>
                  </div>
                </div>
              </section>
            ) : null}

            {detail.verdict ? (
              <section className="block">
                <div className="block-head">
                  <h2 className="block-title">Verdict</h2>
                  <span className="block-note">
                    Final:{" "}
                    {detail.verdict.final_decision === "review" &&
                    detail.payment?.status === "settled"
                      ? "review → released"
                      : decisionLabel(detail.verdict.final_decision)}{" "}
                    · assurance {detail.verdict.assurance}
                  </span>
                </div>
                <div className="verdict-grid">
                  <div className="verdict-col">
                    <div className="verdict-head">
                      <span>SERV Reasoning</span>
                      <span>{detail.verdict.serv.engine}</span>
                    </div>
                    <div className="decision-line">
                      <span className={`decision ${detail.verdict.serv.decision}`}>
                        {detail.verdict.serv.decision}
                      </span>
                      <span className="score">{detail.verdict.serv.score}/100</span>
                    </div>
                    <p className="verdict-summary">{detail.verdict.serv.summary}</p>
                    <ul className="reasons">
                      {detail.verdict.serv.reasons.map((reason, index) => (
                        <li key={index}>{reason}</li>
                      ))}
                    </ul>
                    {detail.verdict.serv.engine !== "serv_live" ? (
                      <div className="engine-note">
                        {detail.verdict.serv.engine === "fixture"
                          ? "Fixture evaluator. SERV_API_KEY is not configured, so this verdict is simulated and cannot release funds."
                          : "SERV did not complete. The release is held."}
                      </div>
                    ) : null}
                  </div>
                  <div className="verdict-col">
                    <div className="verdict-head">
                      <span>Reppo independent</span>
                      <span>{detail.verdict.reppo.mode}</span>
                    </div>
                    <div className="decision-line">
                      <span
                        className={`decision ${
                          detail.verdict.reppo.mode === "live_lite" &&
                          detail.verdict.reppo.status === "settled"
                            ? (detail.verdict.reppo.decision ?? "review")
                            : "advisory"
                        }`}
                      >
                        {detail.verdict.reppo.mode === "live_lite" &&
                        detail.verdict.reppo.status === "settled"
                          ? (detail.verdict.reppo.decision ?? detail.verdict.reppo.status)
                          : "advisory"}
                      </span>
                      <span className="score">
                        {detail.verdict.reppo.mode === "live_lite" &&
                        detail.verdict.reppo.status === "settled" &&
                        detail.verdict.reppo.score !== null
                          ? `${detail.verdict.reppo.score}/100`
                          : "not counted"}
                      </span>
                    </div>
                    <p className="verdict-summary">
                      {detail.verdict.reppo.critique ||
                        "No independent critique was returned."}
                    </p>
                    <ul className="reasons">
                      <li>Status: {detail.verdict.reppo.status}</li>
                      <li>
                        Confidence: {detail.verdict.reppo.confidence ?? "not reported"}
                      </li>
                      <li>
                        Eval: {detail.verdict.reppo.eval_id ?? "not issued"}
                      </li>
                    </ul>
                    {detail.verdict.reppo.mode !== "live_lite" ? (
                      <div className="engine-note">
                        {detail.verdict.reppo.mode === "sandbox"
                          ? "Sandbox verdict. It is a placeholder and never gates a payment."
                          : "Reppo is not configured. Assurance is reduced and the release policy accounts for it."}
                      </div>
                    ) : null}
                    {detail.verdict.reppo.status === "pending" ? (
                      <div className="engine-note">
                        Reppo is still evaluating. The payment is held; refresh when the
                        independent verdict lands.{" "}
                        <button
                          className="button small ghost"
                          onClick={refreshReppo}
                          disabled={busy !== null}
                          style={{ marginTop: "8px" }}
                        >
                          {busy === "refresh" ? "Checking…" : "Refresh verdict"}
                        </button>
                      </div>
                    ) : null}
                  </div>
                </div>
              </section>
            ) : null}
          </div>

          <aside className="detail-rail">
            <div className="rail-block">
              <div className="rail-label">Terms</div>
              <div className="rail-amount">
                {money(milestone.amount_cents, milestone.currency)}
              </div>
              <div className="rail-row">
                <span>Auto-release ceiling</span>
                <span className="mono">
                  {money(milestone.ceiling_cents, milestone.currency)}
                </span>
              </div>
              <div className="rail-row">
                <span>Policy</span>
                <span>
                  Release at or below the ceiling when every gate passes.
                  Otherwise a human decides.
                </span>
              </div>
              <div className="rail-row">
                <span>Graph</span>
                <span className="mono">v{graph.version}</span>
              </div>
            </div>

            <div className="rail-block">
              <div className="rail-label">Payment</div>
              {payment ? (
                <>
                  <div className="payment-stamp">
                    <Stamp tone={paymentInfo?.tone ?? "amber"}>
                      {paymentInfo?.label ?? payment.status}
                    </Stamp>
                  </div>
                  <div className="rail-amount" style={{ fontSize: "28px" }}>
                    {money(payment.amount_cents, payment.currency)}
                  </div>
                  <div className="rail-row">
                    <span>Rail</span>
                    <span className="mono">{payment.rail}</span>
                  </div>
                  <div className="rail-row">
                    <span>Reference</span>
                    <span className="mono">
                      {payment.explorer_url ? (
                        <a href={payment.explorer_url} target="_blank" rel="noreferrer">
                          {payment.tx_ref} ↗
                        </a>
                      ) : (
                        payment.tx_ref ?? "not issued"
                      )}
                    </span>
                  </div>
                  <p className="rail-note">{payment.note}</p>
                  {payment.status === "held" &&
                  detail.verdict?.serv.decision === "accept" ? (
                    <div className="approve-box">
                      <p>
                        The reasoning accepted the work, but the payment policy
                        did not auto-release it. Approving here settles through{" "}
                        {payment.rail}.
                      </p>
                      <div className="field">
                        <label htmlFor="operator-token">
                          Operator token (if configured)
                        </label>
                        <input
                          id="operator-token"
                          type="password"
                          value={operatorToken}
                          onChange={(e) => setOperatorToken(e.target.value)}
                          placeholder="X-Operator-Token"
                          autoComplete="off"
                        />
                      </div>
                      <button
                        className="button small"
                        onClick={approve}
                        disabled={busy !== null}
                      >
                        {busy === "approve" ? "Approving…" : "Approve release"}
                      </button>
                    </div>
                  ) : null}
                </>
              ) : (
                <p className="rail-note">
                  No settlement instruction yet. The payment is created by the
                  acceptance policy after verification.
                </p>
              )}
            </div>

            <div className="rail-block">
              <div className="rail-label">Proof</div>
              {detail.receipt_slug ? (
                <p className="rail-note">
                  <Link
                    href={`/receipt?slug=${detail.receipt_slug}`}
                    className="text-link"
                  >
                    Open public receipt <span aria-hidden="true">→</span>
                  </Link>
                </p>
              ) : (
                <p className="rail-note">
                  A receipt is issued when a submission is released or blocked.
                </p>
              )}
              {submission ? (
                <div className="rail-row">
                  <span>Evidence hash</span>
                  <span className="mono">{shortHash(submission.evidence_hash, 8)}</span>
                </div>
              ) : null}
            </div>

            <div className="rail-block">
              <div className="rail-label">Engines</div>
              <div className="rail-row">
                <span>SERV</span>
                <span>
                  {detail.engines.serv === "live"
                    ? `live · ${detail.engines.serv_model}`
                    : "not configured"}
                </span>
              </div>
              <div className="rail-row">
                <span>Reppo</span>
                <span>{detail.engines.reppo}</span>
              </div>
              <div className="rail-row">
                <span>Settlement</span>
                <span>{detail.engines.payment_rail}</span>
              </div>
              <div className="rail-row">
                <span>Assurance</span>
                <span>{detail.verdict?.assurance ?? "pending"}</span>
              </div>
            </div>

            <div className="rail-block">
              <div className="rail-label">Log</div>
              <ul className="event-list">
                {detail.events.map((event) => (
                  <li key={event.id}>
                    <span className="event-kind">{event.kind.replaceAll("_", " ")}</span>
                    {event.message}
                  </li>
                ))}
              </ul>
            </div>
          </aside>
        </div>

        <div style={{ marginTop: "24px" }}>
          <EngineStrip />
        </div>
      </div>
    </div>
  );
}

export default function MilestonePage() {
  return (
    <Suspense fallback={<div className="wrap loading-block">Loading the release room…</div>}>
      <MilestoneInner />
    </Suspense>
  );
}
