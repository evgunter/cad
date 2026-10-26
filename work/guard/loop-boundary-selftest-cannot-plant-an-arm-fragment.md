---
id: loop-boundary-selftest-cannot-plant-an-arm-fragment
kind: issue
title: loop-boundary-discards --selftest reds on its own register: an arm fragment ending in return is planted as a let-else
status: closed
priority: P3
cost: E
opened: 2026-10-06
closed: 2026-10-26
---


## What

`bash scripts/gates/loop-boundary-discards.sh --selftest` fails on
`origin/main` (212ccfa1) as well as on the branch that found it: "the
gate FAILED on a clean fixture", with

```
UNREG|crates/topo/src/splitting/section_loops.rs|4|loop_edges|let LoopBoundary::Cycle { first } = … else { return
MISCOUNT|crates/topo/src/splitting/section_loops.rs|loop_edges|LoopBoundary::Empty { .. } => return|pinned 1|matched 0
```

The register entry `section_loops.rs|loop_edges|LoopBoundary::Empty {
.. } => return|1|…` carries an ARM fragment. `gate_plant_clean`
(`loop-boundary-discards.sh`) picks the planted form from the
fragment's last word only (`*" return") form=wrapped-return`), and
`wrapped-return` plants a let-else, whose head never contains
`LoopBoundary::Empty { .. } => return`. The header's own rule — "a
fragment the fixture cannot reproduce is one the tree may not
reproduce either" — is what fires, but the tree does reproduce it: the
live gate passes. The planter needs an arm form keyed on a fragment
that starts with the arm's pattern.

CI never runs the gates' `--selftest` (`ci.yml`'s lint job runs each
gate plain; `nightly.yml` runs only `doc-gate.sh --selftest` and
`criterion-emit.py --selftest`), so nothing reds on it — which is a
second finding: no gate's self-test is exercised anywhere.

## Closed (2026-10-26, tracker sweep)

**Fixed in this sweep**, and the failure was still live at main
`bb25fa4f` (reproduced before the change: UNREG/MISCOUNT on
`section_loops.rs` `loop_edges`). `gate_plant_clean` now maps a fragment
ending ` => return` to a new `arm-fragment` form
(`scripts/gates/loop-boundary-discards.sh`, `gate_plant_site`'s
`arm-fragment)` arm, ~:484), which plants the fragment itself as a match
arm, so the head the matcher reads carries it verbatim. The planter's
header comment says so. `--selftest` passes and the plain gate still
passes (71 discards, 68 entries).

Absorbs `loop-boundary-discards-selftest-fails-on-a-clean-tree`
(ENCL, 2026-10-14), the same failure filed a second time; that row's
attribution to TANG's `8f0b0ffd25` was wrong — this row reproduced the
failure on `212ccfa1` on 2026-10-06, before that edit.

**The second finding is not closed by this.** That no CI job runs any
gate's `--selftest` is its own row now:
`gate-selftests-run-in-no-ci-job`.
