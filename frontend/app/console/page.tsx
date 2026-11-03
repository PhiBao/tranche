"use client";

import Link from "next/link";
import { useEffect, useState } from "react";
import { EngineStrip } from "@/components/EngineStrip";
import { StateStamp } from "@/components/Stamp";
import { getJSON } from "@/lib/api";
import { decisionLabel, money, relativeTime, stateLabel } from "@/lib/format";
import type { MilestoneListResponse } from "@/lib/types";

export default function ConsolePage() {
  const [data, setData] = useState<MilestoneListResponse | null>(null);
  const [error, setError] = useState("");

  useEffect(() => {
    getJSON<MilestoneListResponse>("/api/milestones")
      .then(setData)
      .catch((err: Error) => setError(err.message));
  }, []);

  const milestones = data?.milestones ?? [];
  const count = (state: string) => milestones.filter((m) => m.state === state).length;

  return (
    <div className="console-page">
      <div className="wrap">
        <div className="page-head">
          <div>
            <div className="page-kicker">Console / release room</div>
            <h1 className="page-title">Milestones</h1>
            <p className="page-sub">
              Every payout is a decision with a paper trail. Open a milestone to
              see the acceptance graph, the evidence, both verdicts, and the
              settlement instruction.
            </p>
          </div>
          <Link className="button" href="/new">
            New milestone <span aria-hidden="true">↗</span>
          </Link>
        </div>

        <div className="summary-strip">
          <span>
            Awaiting review <b>{count("needs_review")}</b>
          </span>
          <span>
            Open <b>{count("open")}</b>
          </span>
          <span>
            Verifying <b>{count("verifying")}</b>
          </span>
          <span>
            Released <b>{count("released")}</b>
          </span>
          <span>
            Blocked <b>{count("bounced")}</b>
          </span>
        </div>

        <EngineStrip />

        {error ? (
          <div className="error-box" style={{ marginTop: "24px" }}>
            Could not load milestones: {error}. Is the Rust API running on port
            8080?
          </div>
        ) : null}

        {!data && !error ? (
          <div className="loading-block">Loading the release room…</div>
        ) : null}

        {data && milestones.length === 0 ? (
          <div className="empty-state">
            No milestones yet. <Link href="/new">Define the first one.</Link>
          </div>
        ) : null}

        {milestones.length > 0 ? (
          <div className="release-list" role="table" aria-label="Milestones">
            <div className="release-row head" role="row">
              <span>State</span>
              <span>Milestone</span>
              <span>Proof</span>
              <span style={{ textAlign: "right" }}>Tranche</span>
              <span />
              <span />
            </div>
            {milestones.map((m) => (
              <Link
                key={m.id}
                href={`/milestone?id=${m.id}`}
                className="release-row"
                role="row"
              >
                <span className="release-state">
                  <StateStamp state={m.state} />
                </span>
                <span className="release-title-wrap">
                  <span className="release-title">{m.title}</span>
                  <span className="release-meta">
                    {m.builder} · {m.program_name} · {stateLabel(m.state)} ·{" "}
                    {relativeTime(m.updated_at)}
                  </span>
                </span>
                <span className="release-proof">
                  {m.submission_count === 0
                    ? "No submission yet"
                    : m.last_decision
                      ? `Verdict: ${
                          m.last_decision === "review" &&
                          m.last_payment_status === "settled"
                            ? "review → released"
                            : decisionLabel(m.last_decision)
                        }`
                      : "Verification in progress"}
                  <span className="mono">
                    {m.submission_count} submission{m.submission_count === 1 ? "" : "s"}
                    {m.last_payment_status ? ` · ${m.last_payment_status}` : ""}
                  </span>
                </span>
                <span className="release-amount">{money(m.amount_cents, m.currency)}</span>
                <span className="release-arrow" aria-hidden="true">
                  →
                </span>
                <span />
              </Link>
            ))}
          </div>
        ) : null}

        <div className="anatomy-notes" style={{ marginTop: "26px" }}>
          <span>Auto-release ceiling is enforced in code</span>
          <span>Blocked payments cannot be approved</span>
          <span>Every verdict has a public receipt</span>
        </div>
      </div>
    </div>
  );
}
