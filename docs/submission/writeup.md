# Tranche — submission write-up

**Track:** Kite
**One line:** Proof-gated milestone payments. Pay when it's proven.
**Live demo:** https://tranche-serv.fly.dev
**Released receipt:** https://tranche-serv.fly.dev/receipt/?slug=p6nv6vd92fsk
**Blocked receipt:** https://tranche-serv.fly.dev/receipt/?slug=xx3mw7cradtx
**Repository:** https://github.com/PhiBao/tranche
**Demo video:** _to be recorded_

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

The public demo runs a **labeled demo ledger** because live Passport settlement
requires the operator's passkey and a funded USDC.e wallet. The UI never calls the
ledger "Kite"; the adapter is `scripts/kite_payout.sh` and the setup is documented
in the README. The deployed demo runs SERV Reasoning live and Reppo in labeled
sandbox mode.

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

## What is real vs. labeled

The README contains the full table. In short: script checks, hashes, policy,
receipts, and the console are real; SERV is live when a key is configured and
labeled `FIXTURE` otherwise; Reppo is live lite with a credit token, labeled
`sandbox` otherwise; settlement is Kite Passport when configured and a labeled
demo ledger otherwise.
