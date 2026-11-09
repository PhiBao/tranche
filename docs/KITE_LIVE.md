# Live Kite Passport settlement — operator runbook

**Status: completed.** The deployed app settled a real 0.10 USDC payout on Base
through Kite Passport. The public self-serve demo now runs the labeled ledger;
the live proof remains available at
<https://tranche-serv.fly.dev/receipt/?slug=c6ac72kq2rc4>.

## Completed live settlement

- Milestone: `ms_9137b9edc58b453fae0468ec476b700e` — “Kite Passport live settlement”
- Amount: **0.10 USDC** on Base
- From: `0x2f0E613BaF0e6148C29695f8480981e7970a55DD` (Passport wallet)
- To: `0xDbFe5c4d0256D2459bB216caB905999Ca41bE349` (demo builder wallet)
- Request: `wallet_send_req_01a1c2bb-fe2e-7bc7-9e4b-2aaffbaa36d8`
- Transaction: [`0xe5f507ebceb3ca54f78ba5815b9e069e0703c022bd96a8426c2e570016bea301`](https://basescan.org/tx/0xe5f507ebceb3ca54f78ba5815b9e069e0703c022bd96a8426c2e570016bea301)
- Receipt: <https://tranche-serv.fly.dev/receipt/?slug=c6ac72kq2rc4>
- Verdict: SERV accept, Reppo accept (100, settled), assurance high

The payout was created by the deployed app (`POST /api/payments/.../approve`),
approved with the operator passkey, then finalized with
`POST /api/payments/.../kite-status`. The backend recorded the tx only after
`kpass wallet send-status` returned success.

## Accounts and wallet

- Passport account: `kiter0211@gmail.com`
- Passport wallet: `0x2f0E613BaF0e6148C29695f8480981e7970a55DD`
- Builder payout address: `0xDbFe5c4d0256D2459bB216caB905999Ca41bE349`
- `kpass` 6.13.1 is installed in the Fly image and logged in through the
  `KITE_CONFIG_JSON` secret.

## 1. Create the passkey

1. Open <https://agentpassport.ai/overview> on the device that will hold the
   passkey.
2. Sign in with `kiter0211@gmail.com`.
3. Create a passkey when prompted (Touch ID / Face ID / Windows Hello / hardware
   key). This passkey controls every Passport approval.
4. Confirm locally:

```bash
kpass status
kpass wallet balance --output json
```

## 2. Smoke-test one tiny send

From a terminal with `kpass` signed in:

```bash
kpass wallet send \
  --chain base \
  --to 0xDbFe5c4d0256D2459bB216caB905999Ca41bE349 \
  --amount 0.01 \
  --asset USDC \
  --output json
```

Open the returned approval URL, approve with the passkey, then:

```bash
kpass wallet send-status --request-id <request_id> --wait --output json
```

A BaseScan transaction hash confirms the rail. This costs 0.01 USDC plus gas.

## 3. Flip the deployed demo to Kite

In `fly.toml`:

```toml
KITE_MODE = "passport"
```

The image already has `KITE_PAYOUT_COMMAND`, `KITE_STATUS_COMMAND`, and
`KITE_RECIPIENT_ADDRESS` configured. `OPERATOR_TOKEN` is set as a Fly secret;
the operator can read it from `.operator-token` (gitignored) or rotate it.

```bash
fly deploy --ha=false
```

## 4. Run a real release

1. Open the reference milestone and run the seeded sample.
2. SERV and Reppo settle. The policy tries to release through Passport.
3. The payment becomes **Awaiting approval** and shows an **Approve in Kite
   Passport** link.
4. Open the link, approve with the passkey.
5. Return to the milestone and press **Check approval**.
6. The receipt records the BaseScan transaction hash; the milestone becomes
   Released.

If approval is pending, the UI never reports the payment as settled. If the
approval fails or expires, the milestone stays in review.

## Safety

- Keep the Passport wallet funded only with a small demo balance.
- Do not expose the JWT or `.operator-token`.
- The builder payout address is a demo wallet; use a real builder address for a
  live program.
- A pending approval is not a settlement. The status check is authoritative.
