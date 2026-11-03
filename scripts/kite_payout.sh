#!/usr/bin/env bash
# Kite Agent Passport payout adapter for Tranche.
#
# Tranche calls this script with:
#   TRANCHE_PAYMENT_ID, TRANCHE_AMOUNT_CENTS, TRANCHE_CURRENCY,
#   TRANCHE_RECIPIENT (the builder handle), TRANCHE_MILESTONE, TRANCHE_PAYEE
#
# Requirements:
#   - `kpass` installed and signed in on the operator machine
#   - KITE_RECIPIENT_ADDRESS set to the builder's Kite-chain wallet
#
# The script prints a JSON object. Tranche records tx_ref / txHash / hash and
# explorer_url if present. Nothing is printed to stderr on success.

set -euo pipefail

if ! command -v kpass >/dev/null 2>&1; then
  echo "kpass is not installed. Install Kite Passport: curl -fsSL https://cli.gokite.ai/install.sh | bash" >&2
  exit 127
fi

if [ -z "${KITE_RECIPIENT_ADDRESS:-}" ]; then
  echo "KITE_RECIPIENT_ADDRESS is required for a Passport payout." >&2
  exit 2
fi

amount="$(python3 - <<'PY'
import os
cents = int(os.environ["TRANCHE_AMOUNT_CENTS"])
print(f"{cents / 100:.2f}")
PY
)"

kpass wallet send \
  --to "$KITE_RECIPIENT_ADDRESS" \
  --amount "$amount" \
  --asset "${TRANCHE_CURRENCY:-USDC}" \
  --output json
