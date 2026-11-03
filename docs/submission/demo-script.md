# Demo script — 2:30

Record at 1440×900. Use the deployed app with a SERV key configured. If SERV is
not configured, the UI labels the verdict `FIXTURE`; do not hide that in the video.

## 0:00 — The hook

Open the landing page. Pause on the headline.

> "Agents can pay now. The hard question is whether they should. Tranche makes a
> milestone rubric executable, verifies the evidence, and releases the money only
> when the proof passes. Built for the SERV Hackathon 02, Kite track."

## 0:20 — The graph

Click **Open console** → **Bounty — Kite Passport checkout**.

Scroll the acceptance graph.

> "The operator writes the criteria in plain English. Tranche compiles five typed
> nodes before any work is submitted: script checks that cost nothing, a SERV
> judgment, and an independent Reppo verdict. The builder can see exactly what
> will be checked."

## 0:40 — The refusal (the memorable moment)

Click **Run seeded sample**.

Narrate as nodes complete:

> "The repository check passes. The README passes. The demo returns 404."

Pause on the red node and the verdict.

> "SERV rejects the claim. Reppo has no verdict because the evidence already
> failed. The policy blocks the payment. Zero USDC moved."

Open the public receipt.

> "The receipt records every node, both verdicts, the hashes, and the reason money
> did not move. This is the artifact a grant program can hand to an auditor."

## 1:15 — The release

Go back → **Reference build — proof-gated release** → **Run seeded sample**.

> "Same graph. A real public repository, a live demo, a claim that matches the
> evidence."

Show the SERV panel, then the Reppo panel, then the payment stamp.

> "SERV accepts. If Reppo is in live mode, its independent verdict also accepts.
> The amount is at the operator's ceiling, so the policy releases it. In the
> public demo Reppo is a labeled sandbox, and the UI says that too."

Click **Open public receipt**, show the graph hash, evidence hash, settlement
reference.

> "The receipt is public. The builder can share it; the operator can audit it."

## 1:50 — The ceiling

Go to **Grant tranche — independent review** → **Run seeded sample**.

> "The same passing work, but the tranche is above the auto-release ceiling. The
> policy holds it. A human has to approve."

Click **Approve release**. Show the receipt's `review → released`.

> "The model cannot move money. The ceiling is code, and the approval is a
> deliberate operator action."

## 2:15 — The close

Return to the console.

> "Tranche is the acceptance layer for agent payments. Kite gives the agent an
> identity and a spending limit. SERV gives it bounded judgment. Reppo gives it an
> independent check. The result is a payment that can be refused — with a receipt.

> Pay when it's proven."

## Backup plan

- Pre-run the blocked and reference flows once so the database has receipts.
- If SERV is slow, the page shows node status live; do not cut the wait, it is the
  product.
- If Reppo is not configured, say so on camera and show the `sandbox` or
  `not configured` label. Never imply a live independent verdict when there is
  none.
- If the demo ledger is used, say "demo ledger" on camera. Do not call it Kite.
