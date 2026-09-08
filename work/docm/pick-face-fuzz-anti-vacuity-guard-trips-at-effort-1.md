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
