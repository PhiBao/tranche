# Submission checklist

Deadline: **2026-11-15 23:59 UTC**.

## Product

- [x] Public repository: https://github.com/PhiBao/tranche
- [x] Live demo: https://tranche-serv.fly.dev
- [x] Live SERV Reasoning (`serv` in `/api/health` reports `live`)
- [x] Reppo live lite verdicts (credit token funded and set as a Fly secret)
- [x] Labeled demo ledger (Kite Passport adapter ready; live settlement requires
      the operator's passkey and a funded wallet)
- [x] Four seeded milestones: release, blocked, over-ceiling, injection
- [x] Public receipts for released and blocked submissions
- [x] README, architecture diagram, lifecycle diagram, security model
- [x] Favicon and product mark
- [x] Cold-start retry and a scale-to-zero Fly configuration

## Eligibility

- [ ] Open https://console.openserv.ai/settings/organization and enable data
      collection. This was an explicit eligibility requirement in Edition 01.
- [ ] Confirm the OpenServ account/API key used by the deployment is the one
      registered for the hackathon.
- [ ] Confirm the repository is public and the license is present.

## Submission artifacts

- [ ] Record a 2–3 minute demo using `docs/submission/demo-script.md`.
- [ ] Capture the X post from `docs/submission/x-post.md` with:
  - the blocked-payment receipt (`docs/screenshots/live-receipt-blocked.png`),
  - the live release room (`docs/screenshots/live-milestone-released.png`),
  - the architecture diagram (`docs/diagrams/architecture.png`).
- [ ] Post on X and tag `@openservai`, `@GoKiteAI`, and `@reppo`.
- [ ] Copy the X post URL into the submission form:
      https://openserv.typeform.com/submissions02
- [ ] Select the **Kite** track.
- [ ] Paste the write-up from `docs/submission/writeup.md`.
- [ ] Include the live demo and public repository links.
- [ ] Submit before 23:59 UTC on November 15, 2026.

## Final verification

- [ ] Run `scripts/smoke.sh https://tranche-serv.fly.dev`.
- [ ] Open the live release receipt and the live blocked receipt in a private
      browser window.
- [ ] Re-run the reference sample once to confirm the release path after any
      infrastructure change.
- [ ] Confirm the demo still works after the machine has been idle (cold start).
- [ ] Do not rotate the SERV key until after the submission is verified.

## Known limitations to disclose

- Reppo live verdicts consume prepaid credits; if the balance runs out the UI labels the node and holds the payment.
- Settlement is the labeled demo ledger, not a live Kite transfer.
- No user accounts or per-IP rate limiting on the public demo.
- Evidence is public URLs and text; no builder signatures or private repos.
