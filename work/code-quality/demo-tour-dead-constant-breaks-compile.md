---
id: demo-tour-dead-constant-breaks-compile
kind: issue
title: demos/tour: dead constant SAME_CARRIER breaks demo-tour compile (from d485124ca) — invisible to pull_request runs
status: closed
opened: 2026-09-01
closed: 2026-09-03
github: 1449
refs: [1353]
---

## Closed — the constant went live again, from another direction

**Closed by commit `abbc0d8`** ("the twins retire: test-common and the
tour's declarer run the library doors; twopeg picks its mating
finding"), which is not the routing this file asked for: no lane took
the one-line fix as a fix. `abbc0d8` rewrote `twopeg`'s mating-finding
so it needs the tolerance again, and the dead constant became a used
one as a side effect.

Verified in the tree at close: `SAME_CARRIER` is declared at
`demos/tour/src/twopeg.rs:90` and read at `:406-408` and `:657`, and
`cargo check --all-targets` in `demos/tour` finishes clean — the crate
the dispatch-only `demos tour suite` step (`.github/workflows/ci.yml:3198`,
`cd demos/tour && cargo test --release`) could not compile now compiles.

The routing rule this file invoked (the `0c2172e03` ruling: the causing
lane owes the fix) was therefore never exercised, and the fourth-face
observation it rests on — that a `pull_request`-only run cannot see this
red — is untouched by the close.

## From GitHub issue 1449

Opened 2026-09-01; 0 comments.

Found by the VERBS-GERMARMS PR-2 fix pass (#1353): both its workflow-dispatch CI runs red at `k-lint (gate)`'s `demos tour suite` step with

```
error: constant SAME_CARRIER is never used → could not compile demo-tour
```

- `git diff origin/main -- demos/` on the PR head is **empty** — the constant went dead in main commit `d485124ca` ("Style review fallout…").
- It does NOT surface on `pull_request` runs (their filter skips the step), which is why main looks green — the fourth-face family again: the red exists only on the dispatch path.
- Fix is one line in `demos/tour/src/twopeg.rs` (remove or use the constant).

Per the `0c2172e03` ruling the causing lane owes the fix — routing to whoever owns `d485124ca` (style-review fallout lane). VERBS did not repair it cross-program.

🤖 Generated with [Claude Code](https://claude.com/claude-code)

https://claude.ai/code/session_016pYMaeU4woYZN8YGdTLfSK

## Home

`work/code-quality/` — `demos/` is SMELL Track X's fence (as issue 1434 records) and the causing style-review-fallout lane is no open program's; the register owns Tracks K–X.
