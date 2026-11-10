# Tranche — submission write-up

**Track:** Kite
**One line:** Proof-gated milestone payments. Pay when it's proven.
**Live demo:** https://tranche-serv.fly.dev
**Kite-settled receipt:** https://tranche-serv.fly.dev/receipt/?slug=c6ac72kq2rc4
**Kite settlement tx:** https://basescan.org/tx/0xe5f507ebceb3ca54f78ba5815b9e069e0703c022bd96a8426c2e570016bea301
**Blocked receipt:** https://tranche-serv.fly.dev/receipt/?slug=afbq4eejfzhb
**Repository:** https://github.com/PhiBao/tranche
**Demo video:** https://github.com/PhiBao/tranche/raw/main/docs/submission/tranche-demo.mp4 (2:02 walkthrough, synthetic voiceover)

## The problem

Agents can now identify themselves, hold a spending limit, and settle a payment.
The layer that is still missing is acceptance: deciding whether the work actually
met the criteria before any money moves. Today that decision is a human reading a
spreadsheet for a week, or a model saying "looks good" with no evidence and no
audit trail.

## What we built

Tranche compiles a milestone rubric into an executable acceptance graph before
work starts. A submission is hashed, then verified node by node:

- **Script nodes** check facts as Rust code: repository visibility, README terms,
  live-demo reachability, commit recency. No model cost.
- **A SERV agent node** judges the claim against the criteria with PromptGuard and
  a Shadow Agent.
- **A Reppo external node** requests an independent verdict on the trace.
- **A Rust policy gate** combines the results, the assurance level, and the
  operator's auto-release ceiling. The model has no settlement authority.

The output is a public Proof Receipt with the graph hash, evidence hash, every
node run, both verdicts, and the settlement reference. A failed gate produces a
receipt too: `0.00 USDC moved` is a first-class product outcome.

## How SERV Reasoning powers it

The public SERV Reasoning API is the judgment layer. Each agent node calls
`/v1/chat/completions` with a versioned system prompt, a small model, low
reasoning effort, structured JSON, `serv_prompt_guard` on the untrusted evidence,
and `serv_shadow_agent` with a domain-specific hint. The UI shows the SERV model,
decision, score, reasons, latency, and per-node cost.

The graph itself is compiled and executed in Rust. Tranche applies SERV v3's
script/decision/agent sharding discipline at the application layer on top of the
public v2 API; it does not claim to call a beta v3 endpoint.

## How Kite fits

The payment is the product, so the rail matters. In Passport mode, Tranche invokes
a configured `kpass` payout command with the amount, currency, recipient, and
payment reference, and records the returned reference on the receipt. The operator
sets a tranche amount and an auto-release ceiling; above the ceiling a human must
approve, and a blocked payment can never be approved.

The deployed app has executed a real payout: **0.10 USDC on Base** from the
Passport wallet to the builder wallet, approved with a passkey and recorded as
[`0xe5f507…ea301`](https://basescan.org/tx/0xe5f507ebceb3ca54f78ba5815b9e069e0703c022bd96a8426c2e570016bea301).
The receipt is public:
[`c6ac72kq2rc4`](https://tranche-serv.fly.dev/receipt/?slug=c6ac72kq2rc4).

Self-serve milestones run a **labeled demo ledger** because every live Passport
payout requires the operator's passkey and a funded wallet. The UI never calls
the ledger "Kite"; the adapter is `scripts/kite_payout.sh`, the status check is
`scripts/kite_status.sh`, and the setup is documented in the README. The
deployed demo runs both SERV Reasoning and the Reppo Evaluation API live.

## What was built during the hackathon

Everything in this repository: the Rust acceptance engine, the graph compiler,
the script and agent nodes, the SERV and Reppo clients, the payment adapter, the
operator console, the builder submission flow, the public receipt, the seeded demo
flows, tests, diagrams, and deployment.

## What makes it different

Most agent-payment demos show money moving. The useful part is the refusal. A
padded claim, a dead demo, or an instruction hidden in a submission should not be
able to reach a payment rail. Tranche makes the refusal as legible as the release,
and gives both sides a receipt.

## Economic model

Tranche charges 1–2% of released tranches, or a per-verification fee for programs
that prefer to keep settlement in their existing rail. The buyer is the program
operator; the cost is justified against the manual review time and the risk of
paying for unverified work. Every grant program, bounty board, and hackathon is a
distribution channel, and each public receipt is a shareable proof artifact.

## What is real vs. labeled

The README contains the full table. In short: script checks, hashes, policy,
receipts, the console, and one Kite Passport settlement on Base are real; SERV
Reasoning and the Reppo Evaluation API run live on the deployed demo; self-serve
settlement is a labeled demo ledger because each Passport payout needs the
operator's passkey.
