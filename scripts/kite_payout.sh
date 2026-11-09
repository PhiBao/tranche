#!/usr/bin/env bash
# Kite Agent Passport payout adapter for Tranche.
#
# Tranche calls this script with:
#   TRANCHE_PAYMENT_ID, TRANCHE_AMOUNT_CENTS, TRANCHE_CURRENCY,
#   TRANCHE_RECIPIENT (the builder handle), TRANCHE_MILESTONE, TRANCHE_PAYEE
#
# Requirements:
#   - `kpass` installed and the operator signed in WITH A PASSKEY
#   - KITE_RECIPIENT_ADDRESS: the builder's wallet on the payout chain
#   - KITE_CHAIN (default: base)
#
# This script only *creates* the Passport approval request. It prints the
# approval URL and the request id. The operator approves with their passkey,
# then Tranche calls KITE_STATUS_COMMAND (see kite_status.sh) to settle and
# record the transaction.

set -euo pipefail

if ! command -v kpass >/dev/null 2>&1; then
  echo "kpass is not installed. Install Kite Passport: curl -fsSL https://cli.gokite.ai/install.sh | bash" >&2
  exit 127
fi

if [ -z "${KITE_RECIPIENT_ADDRESS:-}" ]; then
  echo "KITE_RECIPIENT_ADDRESS is required for a Passport payout." >&2
  exit 2
fi

cents="${TRANCHE_AMOUNT_CENTS:?TRANCHE_AMOUNT_CENTS is required}"
amount="$(printf '%d.%02d' $((cents / 100)) $((cents % 100)))"

KPASS_NO_BROWSER=1 kpass wallet send \
  --no-interactive \
  --chain "${KITE_CHAIN:-base}" \
  --to "$KITE_RECIPIENT_ADDRESS" \
  --amount "$amount" \
  --asset "${TRANCHE_CURRENCY:-USDC}" \
  --output json
