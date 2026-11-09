#!/usr/bin/env bash
set -euo pipefail

# Optional Kite Passport login for live settlement. The JSON is stored as a
# Fly secret, never baked into the image.
if [ -n "${KITE_CONFIG_JSON:-}" ]; then
  mkdir -p /root/.kite-passport
  printf '%s' "$KITE_CONFIG_JSON" > /root/.kite-passport/config.json
  chmod 600 /root/.kite-passport/config.json
fi

exec /app/tranche
