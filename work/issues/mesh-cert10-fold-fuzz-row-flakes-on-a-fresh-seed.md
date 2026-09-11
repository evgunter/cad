---
id: mesh-cert10-fold-fuzz-row-flakes-on-a-fresh-seed
kind: issue
title: mesh nurbs_cert_fuzz::cert10_the_fold_never_exceeds_the_whole_net_hull reds on a fresh seed (59 strict of 300) - a seeded row that gates
status: open
opened: 2026-09-06
refs: [1907]
---


## What

CI run 34082791499 (PR #1907's docs-only state-sync head `59e524d9e`,
job `test (eps = 1e-6, 1/2)`) reddened on
`crates/mesh/src/nurbs_cert_fuzz.rs:247`:

```
the sweep must keep producing STRICT gaps: 59 strict of 300 comparisons
— reproduce with CAD_FUZZ_SEED=0xdcc78227f392d565 CAD_FUZZ_EFFORT=1
```

The PR touches `topo/src/boolean/boxes.rs`, `census.rs` and demo/test
files — nothing under `crates/mesh/`; main's two most recent CI runs
on the same tree family are green. The row draws a random seed per
run, so this is a seed that falsifies the row's "strict gap" count, not
a regression the PR made. Per `memories/test-suite-cost.md`, a seeded
row that gates owes either a fixed seed with the property it pins, or
the property re-stated so a fresh seed cannot red it without a defect.

## Whose

S-MESH (`crates/mesh/*`). The failed jobs were re-run to draw a new
seed; the merge was annotated with this file. Filed by the CURVED
orchestrator.
