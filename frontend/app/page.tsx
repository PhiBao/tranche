import Link from "next/link";
import { EngineStrip } from "@/components/EngineStrip";
import { Stamp } from "@/components/Stamp";

function DossierNode({
  index,
  label,
  status,
  tone = "ok",
}: {
  index: string;
  label: string;
  status: string;
  tone?: "ok" | "wait";
}) {
  return (
    <div className="dossier-node">
      <span className="n">{index}</span>
      <span>{label}</span>
      <span className={`s ${tone}`}>{status}</span>
    </div>
  );
}

export default function HomePage() {
  return (
    <div>
      <section className="hero">
        <div className="wrap hero-grid">
          <div>
            <div className="eyebrow">
              <span className="dot dot-live" />
              Milestone payments / proof-gated
            </div>
            <h1 className="display">
              Pay when it&apos;s <em>proven.</em>
            </h1>
            <p className="hero-copy">
              Tranche turns a milestone rubric into an executable acceptance
              graph. Evidence is checked in code, judged by SERV Reasoning, and
              reviewed independently by Reppo. Kite moves the money — only after
              every gate passes.
            </p>
            <div className="hero-actions">
              <Link className="button" href="/console">
                Open the release room <span aria-hidden="true">↗</span>
              </Link>
              <Link className="text-link" href="/milestone?id=ms_blocked">
                See a blocked payment <span aria-hidden="true">→</span>
              </Link>
            </div>
            <div className="hero-meta">
              <span>Deterministic checks</span>
              <span>Independent verdict</span>
              <span>Public receipts</span>
            </div>
          </div>

          <aside className="dossier" aria-label="Illustration of a completed release">
            <div className="dossier-head">
              <span>Milestone 014 / Kite track</span>
              <span>USDC</span>
            </div>
            <div className="dossier-title">
              Proof-gated release for the SDK integration
            </div>
            <div className="dossier-amount">
              250.00 <small>USDC</small>
            </div>
            <DossierNode index="01" label="Repository is public" status="passed" />
            <DossierNode index="02" label="README documents SERV" status="passed" />
            <DossierNode index="03" label="Live demo responds" status="passed" />
            <DossierNode index="04" label="Claim safety screen" status="passed" />
            <DossierNode index="05" label="SERV acceptance review" status="accept" />
            <DossierNode index="06" label="Reppo independent verdict" status="accept" />
            <div className="dossier-stamp">
              <Stamp tone="green">Released</Stamp>
            </div>
            <div className="dossier-caption">
              Illustration of a completed release — not a live transaction.
            </div>
          </aside>
        </div>
      </section>

      <section className="section" id="how">
        <div className="wrap section-grid">
          <div>
            <div className="section-label">01 / The gap</div>
            <h2 className="h2">
              Kite answers <em>can</em> the agent pay. Tranche answers{" "}
              <em>should</em> it.
            </h2>
          </div>
          <div>
            <p className="lede">
              Agents can now hold an identity, carry a budget, and settle a
              payment in seconds. What they still cannot do is decide whether a
              piece of work deserves the money. That decision is either a human
              reading a spreadsheet for a week, or a model saying “looks good”
              with no evidence behind it.
            </p>
            <p className="lede">
              Tranche makes acceptance the executable part. The rubric is
              compiled into a graph before work starts. Deterministic checks run
              as code. The parts that need judgment go to SERV. An independent
              network reviews the result. Only then does the payment rail move.
            </p>
          </div>
        </div>

        <div className="wrap" style={{ marginTop: "42px" }}>
          <div className="layers">
            <div className="layer">
              <span className="num">01</span>
              <span className="what">Verifiable identity and spending limits</span>
              <span className="how">
                Passport gives the payer an identity, a budget, a per-transaction
                cap, and a receipt for every action.
              </span>
              <span className="who">Kite</span>
            </div>
            <div className="layer">
              <span className="num">02</span>
              <span className="what">Bounded reasoning</span>
              <span className="how">
                SERV shards the review into script, decision, and agent steps,
                with a predictable path and a logged cost.
              </span>
              <span className="who">SERV</span>
            </div>
            <div className="layer">
              <span className="num">03</span>
              <span className="what">Independent verdict</span>
              <span className="how">
                Reppo evaluates the trace with independent judges and returns a
                score, confidence, and written critiques.
              </span>
              <span className="who">Reppo</span>
            </div>
            <div className="layer">
              <span className="num">04</span>
              <span className="what">Release policy</span>
              <span className="how">
                Code compares the verdict, the evidence, and the caps. The model
                never moves money; a failed gate routes to a human.
              </span>
              <span className="who">Tranche</span>
            </div>
          </div>
        </div>
      </section>

      <section className="section" id="anatomy">
        <div className="wrap">
          <div className="section-label">02 / The mechanism</div>
          <h2 className="h2">A rubric that runs.</h2>
          <p className="lede" style={{ marginTop: "18px" }}>
            The operator writes the milestone in plain English. Tranche compiles
            it into typed nodes before a single submission arrives. The builder
            can see exactly what will be checked, what each check costs, and
            where a human decides.
          </p>

          <div className="anatomy">
            <div className="flow" role="img" aria-label="Acceptance flow: evidence, script checks, SERV review, Reppo verdict, policy gate, Kite payment">
              <div className="flow-step">
                <div className="k">Input</div>
                <div className="t">Evidence</div>
                <div className="d">Repo, live demo, README, and a written claim.</div>
              </div>
              <div className="flow-step">
                <div className="k">Script nodes</div>
                <div className="t">Check facts</div>
                <div className="d">HTTP, git, hashes, and required terms. No model.</div>
              </div>
              <div className="flow-step">
                <div className="k">Agent node</div>
                <div className="t">Judge the claim</div>
                <div className="d">SERV Reasoning with PromptGuard and a Shadow Agent.</div>
              </div>
              <div className="flow-step">
                <div className="k">External</div>
                <div className="t">Independent review</div>
                <div className="d">Reppo judges the trace and returns confidence.</div>
              </div>
              <div className="flow-step gate">
                <div className="k">Policy</div>
                <div className="t">Gate</div>
                <div className="d">Caps, assurance, and ceilings enforced in code.</div>
              </div>
              <div className="flow-step paid">
                <div className="k">Settlement</div>
                <div className="t">Pay</div>
                <div className="d">Kite releases the tranche or holds it for review.</div>
              </div>
            </div>
            <div className="anatomy-notes">
              <span>Script nodes carry no model cost</span>
              <span>Every node writes an audit record</span>
              <span>The graph version is hashed into the receipt</span>
            </div>
          </div>
        </div>
      </section>

      <section className="section" id="blocked">
        <div className="wrap">
          <div className="section-label">03 / The moment</div>
          <h2 className="h2">The payment that doesn&apos;t happen.</h2>
          <p className="lede" style={{ marginTop: "18px" }}>
            Most agent demos show money moving. The useful part is the refusal:
            a claim that outruns its evidence, a demo that returns 404, an
            instruction hidden inside the submission. Tranche makes the refusal
            as legible as the release.
          </p>

          <div className="contrast">
            <div className="contrast-col bad">
              <div className="contrast-head">
                <div>
                  <div className="contrast-title">Padded claim</div>
                  <div className="contrast-sub">Bounty #021 · 15,000 USDC requested</div>
                </div>
                <Stamp tone="red">Blocked</Stamp>
              </div>
              <ul className="reason-list">
                <li>The demo URL returned 404. The evidence does not exist.</li>
                <li>The claim states 9,000 users; no artifact supports the number.</li>
                <li>SERV: reject — the evidence contradicts the claim.</li>
                <li>Reppo: reject — the independent verdict agrees.</li>
                <li>Payment: 0.00 USDC moved. The receipt shows why.</li>
              </ul>
            </div>
            <div className="contrast-col good">
              <div className="contrast-head">
                <div>
                  <div className="contrast-title">Proven work</div>
                  <div className="contrast-sub">Reference build · 250.00 USDC requested</div>
                </div>
                <Stamp tone="green">Released</Stamp>
              </div>
              <ul className="reason-list">
                <li>The repository resolves to a public commit.</li>
                <li>The README documents the required SERV integration.</li>
                <li>The live demo responds with HTTP 200.</li>
                <li>SERV: accept. Reppo: accept, high confidence.</li>
                <li>Within the auto-release ceiling. Kite settles 250.00 USDC.</li>
              </ul>
            </div>
          </div>
        </div>
      </section>

      <section className="section">
        <div className="wrap">
          <div className="section-label">04 / Who pays</div>
          <h2 className="h2">Built for the people who release money.</h2>

          <div className="audience">
            <div className="audience-row">
              <span className="who">Grant programs</span>
              <span className="why">
                Tranches release on evidence, not on the next review meeting.
                Every decision carries a graph version and a public receipt.
              </span>
              <span className="metric">Slow closeouts</span>
            </div>
            <div className="audience-row">
              <span className="who">Bounty boards</span>
              <span className="why">
                Every submission gets the same checks and the same paper trail,
                so outcomes stop depending on who is on call that week.
              </span>
              <span className="metric">Manual triage</span>
            </div>
            <div className="audience-row">
              <span className="who">Hackathons</span>
              <span className="why">
                Proof gates the prize pool. Spam, dead demos, and padded claims
                bounce before a human reads them.
              </span>
              <span className="metric">70+ submissions</span>
            </div>
            <div className="audience-row">
              <span className="who">Agent marketplaces</span>
              <span className="why">
                A buyer defines acceptance once, hires an agent, and pays only
                for work that passes the same graph a human would apply.
              </span>
              <span className="metric">New category</span>
            </div>
          </div>
        </div>
      </section>

      <section className="section" id="stack">
        <div className="wrap">
          <div className="section-label">05 / The stack</div>
          <h2 className="h2">Live status, not a logo wall.</h2>
          <p className="lede" style={{ marginTop: "18px" }}>
            The demo reports what is actually configured. If an engine is
            missing, the interface says so and the policy lowers assurance. Every
            claim in this build is labeled live, sandbox, fixture, or demo ledger
            — including the payment rail.
          </p>
          <EngineStrip />
          <div className="anatomy-notes" style={{ marginTop: "16px" }}>
            <span>SERV: inference-api.openserv.ai</span>
            <span>Reppo: eval.reppo.xyz</span>
            <span>Kite: Agent Passport</span>
            <span>GitHub: public repo checks</span>
          </div>
        </div>
      </section>

      <section className="closing">
        <div className="wrap">
          <div className="eyebrow" style={{ justifyContent: "center" }}>
            <span className="dot dot-live" />
            The release room is open
          </div>
          <h2 className="display">
            Pay when it&apos;s <em>proven.</em>
          </h2>
          <div className="closing-actions">
            <Link className="button" href="/console">
              Open the console <span aria-hidden="true">↗</span>
            </Link>
            <Link className="button ghost" href="/new">
              Define a milestone
            </Link>
          </div>
        </div>
      </section>
    </div>
  );
}
