# GOAL

## Mission

Ship **TRANCHE** — proof-gated milestone payments — as a submission-quality entry for
SERV Hackathon 02 (Nov 2–15, 2026), **Kite track** primary, Reppo track compatible,
Open track fallback.

**Pay when it's proven.**

## The one-sentence product

Tranche compiles a funder's milestone criteria into an executable acceptance graph,
verifies a builder's evidence node by node, gets an independent verdict from Reppo,
and releases payment through Kite Agent Passport only when the proof passes — blocking
or bouncing it with reasons when it does not.

## Winning conditions

1. A judge can open the live app, run the seeded milestone, and see a real verification
   trace: script nodes pass/fail deterministically, SERV Reasoning produces the
   judgment, Reppo provides the independent check, and payment is gated by policy.
2. The **blocked payment** is the memorable moment: a padded submission is refused with
   a stamped reason and $0 moves.
3. The repo is public, the README is submission-grade, the demo is reproducible, and
   every claim in the submission is backed by something visible.
4. The interface does not look like an AI-assembled dashboard. It looks like a release
   room: paper, ink, hard rules, stamps, one green signal.

## Non-negotiable engineering rules

- Rust backend (Axum/Tokio/SQLx). Next.js/TypeScript frontend. SQLite for the demo.
- Model proposes, code decides. No model output can move money around the policy engine.
- Every node run is persisted with input hash, output, cost, latency, and status.
- Never present simulated behavior as real. The UI and README label engine mode
  (`live`, `sandbox`, `fixture`, `not configured`) and payment rail (`kite_passport`,
  `ledger`) truthfully.
- Failure states route to a human. They never auto-release.
- No token, no custom contract, no fake on-chain link.

## Definition of done

- `backend/` builds and tests pass; `frontend/` builds.
- One command runs the stack locally.
- Seeded demo contains one passing submission, one blocked submission, one injection
  attempt, and an over-ceiling approval case.
- Public receipt page for every release.
- README, architecture diagram, acceptance-flow diagram, favicon, product mark.
- Submission write-up + X post draft + demo script in `docs/submission/`.
- Live deployment attempted and documented.

## Deadline

Submissions close **2026-11-15 23:59 UTC**. Build period started 2026-11-02.

## Status

- [x] Discovery complete, direction approved
- [x] Goal set, repository initialized
- [x] Rust backend (Axum, SQLx, SQLite) + 23 tests
- [x] Next.js frontend + static export served by the Rust binary
- [x] SERV Reasoning live integration (PromptGuard, Shadow Agent, structured verdict)
- [x] Reppo Evaluation API integration (labeled sandbox in the public demo)
- [x] Kite Passport payout adapter + labeled demo ledger
- [x] Seeded demo: release, blocked, injection, over-ceiling
- [x] Public proof receipts + live deployed demo
- [x] README, architecture/lifecycle diagrams, security model, screenshots
- [x] Cold start handled; Fly scales to zero
- [ ] Record the demo video
- [ ] Post on X and submit the Typeform
