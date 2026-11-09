"use client";

import Link from "next/link";
import { useRouter } from "next/navigation";
import { useEffect, useState } from "react";
import { postJSON } from "@/lib/api";
import type { AcceptanceGraph } from "@/lib/types";

interface Preview {
  graph: AcceptanceGraph;
  graph_summary: {
    nodes: number;
    script_nodes: number;
    agent_nodes: number;
    external_nodes: number;
    estimated_review_cost_usd: number;
  };
}

const kindLabel: Record<string, string> = {
  script: "Script · code",
  agent: "Agent · SERV",
  external: "External · Reppo",
};

export default function NewMilestonePage() {
  const router = useRouter();
  const [title, setTitle] = useState("");
  const [builder, setBuilder] = useState("");
  const [amount, setAmount] = useState("250");
  const [ceiling, setCeiling] = useState("100");
  const [criteria, setCriteria] = useState(
    "The repository is public and its README documents SERV Reasoning. The live demo responds. The claim is supported by the linked evidence.",
  );
  const [busy, setBusy] = useState(false);
  const [error, setError] = useState("");
  const [preview, setPreview] = useState<Preview | null>(null);
  const [previewError, setPreviewError] = useState("");
  const [reviewed, setReviewed] = useState(false);

  useEffect(() => {
    setReviewed(false);
    if (criteria.trim().length < 20) {
      setPreview(null);
      setPreviewError("");
      return;
    }
    const timer = window.setTimeout(() => {
      postJSON<Preview>("/api/milestones/preview", { criteria })
        .then((data) => {
          setPreview(data);
          setPreviewError("");
        })
        .catch((err: Error) => {
          setPreview(null);
          setPreviewError(err.message);
        });
    }, 450);
    return () => window.clearTimeout(timer);
  }, [criteria]);

  async function submit() {
    setBusy(true);
    setError("");
    try {
      const amountCents = Math.round(Number(amount) * 100);
      const ceilingCents = Math.round(Number(ceiling) * 100);
      if (!Number.isFinite(amountCents) || !Number.isFinite(ceilingCents)) {
        throw new Error("Amount and ceiling must be numbers.");
      }
      const created = await postJSON<{ id: string }>("/api/milestones", {
        title,
        builder,
        criteria,
        amount_cents: amountCents,
        ceiling_cents: ceilingCents,
      });
      router.push(`/milestone?id=${created.id}`);
    } catch (err) {
      setError(err instanceof Error ? err.message : "Could not create the milestone.");
      setBusy(false);
    }
  }

  return (
    <div className="new-page">
      <div className="wrap">
        <div className="breadcrumb">
          <Link href="/console">Console</Link>
          <span>/</span>
          <span>New milestone</span>
        </div>

        <div className="page-head">
          <div>
            <div className="page-kicker">Define the acceptance</div>
            <h1 className="page-title">Publish the graph.</h1>
            <p className="page-sub">
              Write the milestone in plain English. Tranche compiles the rubric
              into typed nodes before any work is submitted. The builder sees the
              same graph the payment will be judged against.
            </p>
          </div>
          <Link className="button ghost" href="/console">
            Cancel
          </Link>
        </div>

        <div className="detail-grid" style={{ paddingTop: "32px" }}>
          <div className="detail-main">
            {error ? <div className="error-box">{error}</div> : null}
            <div className="form">
              <div className="form-title">Milestone terms</div>
              <div className="form-sub">
                The tranche is funded by the operator. The ceiling is the largest
                amount code may release without a human.
              </div>

              <div className="field">
                <label htmlFor="title">Milestone title</label>
                <input
                  id="title"
                  value={title}
                  onChange={(e) => setTitle(e.target.value)}
                  placeholder="Kite Passport checkout for the marketplace"
                />
              </div>

              <div className="form-grid">
                <div className="field">
                  <label htmlFor="builder">Builder handle</label>
                  <input
                    id="builder"
                    value={builder}
                    onChange={(e) => setBuilder(e.target.value)}
                    placeholder="@builder"
                  />
                </div>
                <div className="field">
                  <label htmlFor="program">Program</label>
                  <input id="program" value="SERV Hackathon 02 — Kite Track" readOnly />
                </div>
              </div>

              <div className="form-grid">
                <div className="field">
                  <label htmlFor="amount">Tranche amount (USDC)</label>
                  <input
                    id="amount"
                    value={amount}
                    onChange={(e) => setAmount(e.target.value)}
                    inputMode="decimal"
                  />
                </div>
                <div className="field">
                  <label htmlFor="ceiling">Auto-release ceiling (USDC)</label>
                  <input
                    id="ceiling"
                    value={ceiling}
                    onChange={(e) => setCeiling(e.target.value)}
                    inputMode="decimal"
                  />
                  <div className="hint">
                    Above this line, an accepted verdict waits for the operator.
                  </div>
                </div>
              </div>

              <div className="field">
                <label htmlFor="criteria">Acceptance criteria</label>
                <textarea
                  id="criteria"
                  value={criteria}
                  onChange={(e) => setCriteria(e.target.value)}
                  style={{ minHeight: "130px" }}
                />
                <div className="hint">
                  The compiler looks for required terms (SERV, Kite, Reppo,
                  Dolphin), repository and demo checks, and optional commit
                  recency. Every generated node is visible on the milestone page.
                </div>
              </div>

              <div className="form-actions">
                <button
                  className="button"
                  onClick={submit}
                  disabled={busy || !preview || !reviewed}
                >
                  {busy ? "Compiling…" : "Publish approved graph"}
                </button>
                <span className="block-note">No funds move on publish.</span>
              </div>

              <label className="review-check">
                <input
                  type="checkbox"
                  checked={reviewed}
                  onChange={(e) => setReviewed(e.target.checked)}
                  disabled={!preview}
                />
                <span>
                  I reviewed the compiled acceptance graph and its estimated
                  review cost before publishing.
                </span>
              </label>
              {!preview && !previewError ? (
                <div className="hint" style={{ marginTop: "8px" }}>
                  Compiling the graph from your criteria…
                </div>
              ) : null}
              {previewError ? (
                <div className="error-box" style={{ marginTop: "12px" }}>
                  Could not compile the graph: {previewError}
                </div>
              ) : null}
            </div>
          </div>

          <aside className="detail-rail">
            <div className="rail-block">
              <div className="rail-label">Compiled graph · review before publish</div>
              {preview ? (
                <>
                  {preview.graph.nodes.map((node, index) => (
                    <div className="rail-row" key={node.id}>
                      <span>
                        {String(index + 1).padStart(2, "0")} · {node.label}
                      </span>
                      <span>{kindLabel[node.kind] ?? node.kind}</span>
                    </div>
                  ))}
                  <div className="rail-row">
                    <span>Estimated review cost</span>
                    <span className="mono">
                      ${preview.graph_summary.estimated_review_cost_usd.toFixed(4)}
                    </span>
                  </div>
                  <div className="rail-row">
                    <span>Script nodes</span>
                    <span>{preview.graph_summary.script_nodes} · no model cost</span>
                  </div>
                </>
              ) : (
                <p className="rail-note">
                  {previewError ||
                    "The compiled graph appears here as you write the criteria. Nothing is published until you approve it."}
                </p>
              )}
            </div>
            <div className="rail-block">
              <div className="rail-label">Policy</div>
              <p className="rail-note">
                A failed script node blocks the payment. A model verdict cannot
                override a script failure. A rejected independent verdict blocks
                the payment even if SERV accepted.
              </p>
            </div>
            <div className="rail-block">
              <div className="rail-label">Evidence</div>
              <p className="rail-note">
                Submissions are hashed before verification. The hash and the graph
                version appear on the public receipt.
              </p>
            </div>
          </aside>
        </div>
      </div>
    </div>
  );
}
