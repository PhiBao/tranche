# Submission checklist

Deadline: **2026-11-15 23:59 UTC**.

## Product

- [x] Public repository: https://github.com/PhiBao/tranche
- [x] Live demo: https://tranche-serv.fly.dev
- [x] Live SERV Reasoning (`serv` in `/api/health` reports `live`)
- [x] Reppo live lite verdicts (credit token funded and set as a Fly secret)
- [x] Live Kite Passport settlement: 0.10 USDC on Base,
      tx `0xe5f507ebceb3ca54f78ba5815b9e069e0703c022bd96a8426c2e570016bea301`
      ([receipt](https://tranche-serv.fly.dev/receipt/?slug=c6ac72kq2rc4))
- [x] Labeled demo ledger for self-serve runs (Passport needs the operator passkey)
- [x] Four seeded milestones: release, blocked, over-ceiling, injection
- [x] Public receipts for the Kite-settled release and the blocked submissions
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

- [x] Record a 2-minute demo: `docs/submission/tranche-demo.mp4`
      (synthetic voiceover; re-record with your own voice if preferred).
- [ ] Capture the X post from `docs/submission/x-post.md` with:
  - the Kite-settled receipt (`docs/screenshots/live-receipt-kite.png`),
  - the 2-minute walkthrough (`docs/submission/tranche-demo.mp4`),
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
- [ ] Open the live Kite-settled receipt (`c6ac72kq2rc4`), the BaseScan tx, and
      the live blocked receipt in a private browser window.
- [ ] Re-run the reference sample once to confirm the release path after any
      infrastructure change.
- [ ] Confirm the demo still works after the machine has been idle (cold start).
- [ ] Do not rotate the SERV key until after the submission is verified.

## Known limitations to disclose

- Reppo live verdicts consume prepaid credits; if the balance runs out the UI labels the node and holds the payment.
- Self-serve demo settlement is the labeled ledger. One real Kite Passport
  settlement on Base is linked on the receipt; each additional live payout needs
  the operator passkey.
- No user accounts or per-IP rate limiting on the public demo.
- Evidence is public URLs and text; no builder signatures or private repos.
