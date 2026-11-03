"use client";

import Link from "next/link";
import { useSearchParams } from "next/navigation";
import { Suspense, useEffect, useState } from "react";
import { Stamp } from "@/components/Stamp";
import { getJSON } from "@/lib/api";
import { dateTime, decisionLabel, money, shortHash } from "@/lib/format";
import type { ReceiptResponse } from "@/lib/types";

function ReceiptInner() {
  const params = useSearchParams();
  const slug = params.get("slug") ?? "";
  const [data, setData] = useState<ReceiptResponse | null>(null);
  const [error, setError] = useState("");

  useEffect(() => {
    if (!slug) {
      setError("No receipt specified.");
      return;
    }
    getJSON<ReceiptResponse>(`/api/receipts/${slug}`)
      .then(setData)
      .catch((err: Error) => setError(err.message));
  }, [slug]);

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

  if (!data) {
    return <div className="wrap loading-block">Loading the receipt…</div>;
  }

  const final = data.verdict?.final_decision;
  const released = data.payment?.status === "settled";
  const stamp = released
    ? { tone: "green" as const, label: "Released" }
    : final === "bounce"
      ? { tone: "red" as const, label: "Blocked" }
      : final === "review"
        ? { tone: "amber" as const, label: "Held" }
        : { tone: "ink" as const, label: "Pending" };
  const settled = data.payment?.status === "settled";
  const moved = settled ? data.payment?.amount_cents ?? 0 : 0;

  return (
    <div className="receipt-page">
      <div className="wrap">
        <div className="breadcrumb">
          <Link href="/console">Console</Link>
          <span>/</span>
          <span>Receipt</span>
          <span>/</span>
          <span>{data.receipt.slug}</span>
        </div>

        <div className="receipt-wrap">
          <article className="receipt">
            <header className="receipt-top">
              <div>
                <div className="receipt-kicker">Tranche / proof receipt</div>
                <h1 className="receipt-title">
                  {released
                    ? "Payment released."
                    : stamp.label === "Blocked"
                      ? "Payment blocked."
                      : "Payment held."}
                </h1>
              </div>
              <div className="receipt-meta">
                <div>Receipt {data.receipt.slug}</div>
                <div>{dateTime(data.receipt.created_at)}</div>
                <div>{data.milestone.program_name}</div>
                <Stamp tone={stamp.tone}>{stamp.label}</Stamp>
              </div>
            </header>

            <div className="receipt-grid">
              <div>
                <div className="receipt-label">Amount moved</div>
                <div className="receipt-amount">
                  {(moved / 100).toLocaleString("en-US", {
                    minimumFractionDigits: 2,
                    maximumFractionDigits: 2,
                  })}
                  <small>{data.milestone.currency}</small>
                </div>
                <p className="receipt-note" style={{ marginTop: "10px" }}>
                  {settled
                    ? "Settled through the recorded rail. The reference below is the settlement instruction."
                    : "No funds moved. The acceptance policy did not release this tranche."}
                </p>
              </div>
              <div>
                <div className="receipt-label">Milestone</div>
                <div className="receipt-value">{data.milestone.title}</div>
                <div className="receipt-label" style={{ marginTop: "14px" }}>
                  Builder
                </div>
                <div className="receipt-value">{data.milestone.builder}</div>
                <div className="receipt-label" style={{ marginTop: "14px" }}>
                  Final decision
                </div>
                <div className="receipt-value">
                  {released && final === "review"
                    ? "review → released"
                    : decisionLabel(final)}{" "}
                  · assurance {data.verdict?.assurance ?? "n/a"}
                </div>
              </div>
            </div>

            <section className="receipt-section">
              <div className="receipt-section-title">Scope · criteria agreed before work</div>
              <p className="receipt-value">{data.milestone.criteria}</p>
            </section>

            <section className="receipt-section">
              <div className="receipt-section-title">Evidence</div>
              <div className="receipt-grid" style={{ padding: 0, border: 0 }}>
                <div>
                  <div className="receipt-label">Claim</div>
                  <p className="receipt-value">“{data.submission.evidence.claim}”</p>
                </div>
                <div>
                  <div className="receipt-label">Sources</div>
                  <div className="evidence-links">
                    <div>
                      <span>Repository</span>
                      <a
                        href={data.submission.evidence.repo_url}
                        target="_blank"
                        rel="noreferrer"
                      >
                        {data.submission.evidence.repo_url}
                      </a>
                    </div>
                    <div>
                      <span>Demo</span>
                      <a
                        href={data.submission.evidence.demo_url}
                        target="_blank"
                        rel="noreferrer"
                      >
                        {data.submission.evidence.demo_url}
                      </a>
                    </div>
                  </div>
                  <div className="hash-line">
                    SHA-256 {data.submission.evidence_hash}
                  </div>
                </div>
              </div>
            </section>

            <section className="receipt-section">
              <div className="receipt-section-title">Acceptance trace</div>
              <table className="receipt-table">
                <thead>
                  <tr>
                    <th>Node</th>
                    <th>Kind</th>
                    <th>Result</th>
                    <th>Latency</th>
                    <th>Cost</th>
                  </tr>
                </thead>
                <tbody>
                  {data.node_runs.map((node) => (
                    <tr key={node.id}>
                      <td>
                        {node.label}
                        <div style={{ color: "var(--muted)", marginTop: "2px" }}>
                          {node.detail}
                        </div>
                      </td>
                      <td className="mono">{node.kind}</td>
                      <td className="mono" style={{ textTransform: "uppercase" }}>
                        {node.status}
                      </td>
                      <td className="mono">{node.latency_ms ? `${node.latency_ms} ms` : "—"}</td>
                      <td className="mono">
                        {node.cost_usd ? `$${node.cost_usd.toFixed(4)}` : "$0.0000"}
                      </td>
                    </tr>
                  ))}
                </tbody>
              </table>
              <div className="hash-line">Graph SHA-256 · {data.graph_hash}</div>
            </section>

            <section className="receipt-section">
              <div className="receipt-section-title">Verdicts</div>
              <div className="verdict-grid">
                <div className="verdict-col">
                  <div className="verdict-head">
                    <span>SERV Reasoning</span>
                    <span>{data.verdict?.serv.engine ?? "not run"}</span>
                  </div>
                  <div className="decision-line">
                    <span className={`decision ${data.verdict?.serv.decision ?? "review"}`}>
                      {data.verdict?.serv.decision ?? "not run"}
                    </span>
                    <span className="score">{data.verdict?.serv.score ?? 0}/100</span>
                  </div>
                  <p className="verdict-summary">
                    {data.verdict?.serv.summary ?? "No reasoning verdict was recorded."}
                  </p>
                  <ul className="reasons">
                    {(data.verdict?.serv.reasons ?? []).map((reason, index) => (
                      <li key={index}>{reason}</li>
                    ))}
                  </ul>
                </div>
                <div className="verdict-col">
                  <div className="verdict-head">
                    <span>Reppo independent</span>
                    <span>{data.verdict?.reppo.mode ?? "not run"}</span>
                  </div>
                  <div className="decision-line">
                    <span
                      className={`decision ${data.verdict?.reppo.decision ?? "review"}`}
                    >
                      {data.verdict?.reppo.decision ?? data.verdict?.reppo.status ?? "not run"}
                    </span>
                    <span className="score">
                      {data.verdict?.reppo.score !== null &&
                      data.verdict?.reppo.score !== undefined
                        ? `${data.verdict.reppo.score}/100`
                        : "—"}
                    </span>
                  </div>
                  <p className="verdict-summary">
                    {data.verdict?.reppo.critique ?? "No independent critique was returned."}
                  </p>
                  <ul className="reasons">
                    <li>Status: {data.verdict?.reppo.status ?? "not run"}</li>
                    <li>Confidence: {data.verdict?.reppo.confidence ?? "not reported"}</li>
                    <li>Eval: {data.verdict?.reppo.eval_id ?? "not issued"}</li>
                  </ul>
                </div>
              </div>
            </section>

            <section className="receipt-section">
              <div className="receipt-section-title">Settlement</div>
              {data.payment ? (
                <div className="receipt-grid" style={{ padding: 0, border: 0 }}>
                  <div>
                    <div className="receipt-label">Rail / status</div>
                    <div className="receipt-value">
                      {data.payment.rail} · {data.payment.status}
                    </div>
                    <div className="receipt-label" style={{ marginTop: "12px" }}>
                      Reference
                    </div>
                    <div className="receipt-value mono" style={{ fontSize: "11px" }}>
                      {data.payment.explorer_url ? (
                        <a href={data.payment.explorer_url} target="_blank" rel="noreferrer">
                          {data.payment.tx_ref} ↗
                        </a>
                      ) : (
                        data.payment.tx_ref ?? "not issued"
                      )}
                    </div>
                  </div>
                  <div>
                    <div className="receipt-label">Operator note</div>
                    <p className="receipt-note">{data.payment.note}</p>
                  </div>
                </div>
              ) : (
                <p className="receipt-note">No settlement instruction was recorded.</p>
              )}
            </section>

            <footer className="receipt-foot">
              <p className="receipt-note">
                This receipt records an acceptance decision. Script nodes ran as
                deterministic code. The reasoning verdict was produced by SERV
                Reasoning. The independent verdict was requested from Reppo. The
                release policy is enforced in Rust and is not controlled by the
                model. Evidence is identified by SHA-256. Engine modes and the
                payment rail are labeled exactly as configured at verification
                time.
              </p>
              <div className="receipt-actions">
                <button className="button small ghost" onClick={() => window.print()}>
                  Print receipt
                </button>
                <Link className="button small" href="/console">
                  Release room
                </Link>
              </div>
            </footer>
          </article>
        </div>

        <div className="anatomy-notes" style={{ marginTop: "22px" }}>
          <span>Receipt {shortHash(data.receipt.id, 8)}</span>
          <span>Graph v{data.graph.version}</span>
          <span>{data.node_runs.length} nodes recorded</span>
        </div>
      </div>
    </div>
  );
}

export default function ReceiptPage() {
  return (
    <Suspense fallback={<div className="wrap loading-block">Loading the receipt…</div>}>
      <ReceiptInner />
    </Suspense>
  );
}
