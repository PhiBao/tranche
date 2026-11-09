#!/usr/bin/env bash
# Polls a Kite Passport wallet-send approval request.
#
# Tranche calls this with TRANCHE_REQUEST_ID after the operator says they
# approved the payout. Prints the CLI JSON so the backend can record the tx.

set -euo pipefail

if ! command -v kpass >/dev/null 2>&1; then
  echo "kpass is not installed." >&2
  exit 127
fi

if [ -z "${TRANCHE_REQUEST_ID:-}" ]; then
  echo "TRANCHE_REQUEST_ID is required." >&2
  exit 2
fi

KPASS_NO_BROWSER=1 kpass wallet send-status \
  --no-interactive \
  --request-id "$TRANCHE_REQUEST_ID" \
  --wait \
  --timeout "${KITE_APPROVAL_TIMEOUT_SECONDS:-30}" \
  --output json
