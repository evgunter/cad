---
id: pick-face-fuzz-anti-vacuity-guard-trips-at-effort-1
kind: issue
title: review_gui1_r1's random-ray oracle sweep asserts "some draw hit the cube" over 120 integer rays at effort 1 — a probabilistic guard that reds CI on a bad seed
status: open
opened: 2026-09-08
refs: [1617]
---

Filed by the S-MESH orchestrator from MESH-12's landing run (PR 1617, run
34171106411, job `test (interval, eps = 1e-6, 1/2)`), on DOCM's slate
because the file is `crates/editor-core/tests/review_gui1_r1.rs`.

`random_integer_rays_match_the_exact_oracle` draws `fuzz::scaled(120)`
integer rays (origins in [-3, 3]³, directions in [-2, 2]³) against a
unit cube and, after the sweep, asserts `hits_seen > 0` with the message
"no draw hit the cube — generator shape broke". At effort 1 that is a
probabilistic claim, and seed `0x2870e278e5a1ef24` falsifies it:

```
CAD_FUZZ_SEED=0x2870e278e5a1ef24 CAD_TOLERANCE_EPS=1e-6 \
  cargo nextest run -p editor-core --features interval \
  -E 'test(random_integer_rays_match_the_exact_oracle)'
# → panicked at review_gui1_r1.rs:498: no draw hit the cube
```

Reproduces at default eps with the same seed; passes at
`CAD_FUZZ_EFFORT=2`; passes for 24 other seeds. The row's own comment
says anti-vacuity is "structural, not searched (the battery row covers
guaranteed hits)", which is the right rule — the searched guard
contradicts it. The fix is the comment's: drop the searched assertion,
or make the sweep's first draw a known hit so the guard is structural.
`test-utils::fuzz`'s header says effort "can never make the test fail";
here a LOWER effort can, which is the same contract broken from below.

Not MESH-12's fence (the unit touches no editor-core file); disclosed on
PR 1617 and left to its owner.

## Second instance, a second program, a different seed (2026-09-13, WIRE)

Added as evidence rather than a second row. This guard reddened **WIRE
PR 2518's state-sync run** — run `34783289811`, job
`test (eps = 1e-6, 2/2)`, `review_gui1_r1.rs:498`, same message:

```
no draw hit the cube — generator shape broke;
reproduce with CAD_FUZZ_SEED=0x1a9e0f26198e881b CAD_FUZZ_EFFORT=1
```

**A different seed from the one this row was filed on** — `0x1a9e0f26…`
against the original `0x2870e278…` — and a different eps row, which is
what a probabilistic guard looks like from the outside: it is not one bad
seed, it is a distribution.

**The commit it reddened was two markdown files.** The previous run on
the same code (`34782064566`, head `01d8dbca8`) was green; the only diff
to the red head was two `work/wire/*.md` closing sections. **Re-running
the failed job on identical code passed**, which is the decisive
instrument: the variable is the seed, not the tree.

What that cost, and why it is worth recording on the row rather than
just in a log: the red arrived on a **docs-only state-sync commit at the
end of a unit**, which is the point where an orchestrator is deciding
whether to merge. A guard that reds there spends a reviewer's attention
on an already-answered question, and the honest reading of a red on a
two-markdown-file diff is "something is wrong with the gate", which is
exactly the wrong lesson to teach.

The tree already carries the rule this row asks for:
`crates/test-utils/src/vacuity.rs` says an anti-vacuity claim is
**"stated against the floor of the dial, never"** above it — so the fix
has a written convention to land on, not just a judgement call.

## Re-homed (2026-09-13)

Moved from `work/docm/` to `work/tint/` at DOCM's exit sweep (`docs/DOC-LEDGER.md`,
sweep 14): a probabilistic or once-flaky test guard is S-TINT's (test-suite integrity). Id, body and header are unchanged; the directory is the
claim (`work/README.md`). Any `## Home` section above is superseded by
this line and is kept as the record of why the file was where it was.
