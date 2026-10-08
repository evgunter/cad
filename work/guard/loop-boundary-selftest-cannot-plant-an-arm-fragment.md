---
id: loop-boundary-selftest-cannot-plant-an-arm-fragment
kind: issue
title: loop-boundary-discards --selftest reds on its own register: an arm fragment ending in return is planted as a let-else
status: open
priority: P3
cost: E
opened: 2026-10-06
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
