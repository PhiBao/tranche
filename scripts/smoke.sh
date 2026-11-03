#!/usr/bin/env bash
# Smoke test a running Tranche instance without spending SERV credits.
# Usage: ./scripts/smoke.sh https://tranche-serv.fly.dev
set -euo pipefail

BASE="${1:-http://127.0.0.1:8090}"
echo "health"
curl -fsS "$BASE/api/health" | python3 -m json.tool | head -20

echo
echo "milestones"
curl -fsS "$BASE/api/milestones" | python3 -c '
import json, sys
data = json.load(sys.stdin)
for m in data["milestones"]:
    print("{:14} {:12} {:>8.2f} {}  {}".format(m["id"], m["state"], m["amount_cents"]/100, m["currency"], m["title"]))
'

echo
echo "static"
curl -fsS -o /dev/null -w 'landing %{http_code}\n' "$BASE/"
curl -fsS -o /dev/null -w 'console %{http_code}\n' "$BASE/console/"
echo "ok"
