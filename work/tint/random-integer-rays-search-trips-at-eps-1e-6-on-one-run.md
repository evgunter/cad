---
id: random-integer-rays-search-trips-at-eps-1e-6-on-one-run
kind: issue
title: review_gui1_r1::random_integer_rays_match_the_exact_oracle failed once at eps = 1e-6 on a commit that passed it before and after
status: open
opened: 2026-09-09
---


Filed by LIB (2026-09-09) from LIB-LOOPS's CI (`lib/loops`, PR #2268,
commit `ca64828d9`): the varying-seed counterexample search at
`crates/editor-core/tests/review_gui1_r1.rs:420` failed in the
`test (eps = 1e-6, 2/2)` shard on the second of two full runs of the
same commit — green on the first run, and green six times locally at
`CAD_TOLERANCE_EPS=1e-6`. The subject is ray picking on an extruded
cube; the PR's diff is four role-name builder signatures and their
call sites, which that test does not reach. Filed here rather than
in LIB's fence because `work/docm/pick-face-fuzz-anti-vacuity-guard-trips-at-effort-1.md`
already carries a sibling flake in the same file, so the seed
discipline of that search looks like one question. LIB did not
re-run or touch the test.

## Re-homed (2026-09-13)

Moved from `work/docm/` to `work/tint/` at DOCM's exit sweep (`docs/DOC-LEDGER.md`,
sweep 14): a probabilistic or once-flaky test guard is S-TINT's (test-suite integrity). Id, body and header are unchanged; the directory is the
claim (`work/README.md`). Any `## Home` section above is superseded by
this line and is kept as the record of why the file was where it was.
