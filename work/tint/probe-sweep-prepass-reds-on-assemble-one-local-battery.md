---
id: probe-sweep-prepass-reds-on-assemble-one-local-battery
kind: issue
title: the probe sweep's plain pre-pass reds on assemble_one_local_battery (EmptyProduct), so k-lint (dev-probe) never reaches the lint
status: open
opened: 2026-10-10
priority: P1
cost: E
---



## What

`scripts/k_probe_sweep.sh` runs every rostered plain probe suite at
ε 1e-9 before any dump (`run_plain`). One of them fails on main, so the
script exits before the k-lint step and nightly `k-lint (dev-probe)`
lints nothing. Nightly run 38027799934 (2026-10-10, head `b9bbd645`),
job `k-lint (dev-probe)`:

```
test assemble_one_local_battery::the_assembly_gate_runs_the_census_over_the_gathers_verdict ... FAILED
panicked at crates/editor-core/tests/assemble_one_local_battery.rs:55:52:
the union gathers: EmptyProduct { unplaced: [VarId(..ordinal: 29..), VarId(..58..), VarId(..60..)] }
```

It reproduces locally at `af7a95b09` (`cargo test -p editor-core
--features probe --test all -- assemble_one_local_battery::`,
`CAD_TOLERANCE_EPS=1e-9`). The suite is `feature = "probe"` only, so
per-PR CI does not build it. `EmptyProduct { unplaced }` is INTENT's
stage-2 error, raised when nothing is placed in the world. Read, not bisected: the fixture
`two_blocks_and_their_union` appears to predate the explicit product list and
places nothing.

## Owed

Have the fixture place its union the way INTENT's `EmptyProduct`
recourse says (the `Part` pick), then re-run the sweep's pre-pass.
Found by GERM's `germ/circle-torus-slack-klint` lane, which needed the
lint's reading and had to take it locally.

## Progress (2026-10-11)

The `assemble_one_local_battery` cause is fixed by INTENT's
`the-probe-gated-assembly-battery-gathers-an-unplaced-fixture` (the
row's caller places the union). It is not the only red: running every
rostered plain suite locally at ε 1e-9 (the `run_plain` selection), two
more fail, so the sweep still stops before the k-lint step:

- `geom-brep::rim_dim_scale_twins` — filed as
  `mm-scale-sphere-rim-probe-no-longer-refuses`;
- `topo::rim_dim_review_probes` — already filed as
  `silent-fixed-predicates-probe-reds-under-all-features-on-main`.

This row closes when both are green and the pre-pass passes.

- 2026-10-12 (ENCL orchestrator, nightly triage): the test is red again on origin/main bcaed4f75a, with a NEW assertion, after PR 4679 fixed the EmptyProduct cause. Repro at `CAD_TOLERANCE_EPS=1e-9`:

  `cargo test -p editor-core --features probe --test all -- assemble_one_local_battery::`

  It panics at `assemble_one_local_battery.rs:70:5`: "the assembly gate recorded 0 samples, tier 3 2053 and tier 3′ 2533: the gate must be tier 3′ less one battery" (left 2053, right 2533). It is not bisected. The likely cause is INTENT's census moving into the gather: PR 4669 (`intent/s4-census-records-what-it-decides`) or PR 4733 (s4-j, ce834bd231), both merged 10-11. The fix is either to count the gather's samples into the gate or to re-state the equality. This still blocks nightly `k-lint (dev-probe)` through `scripts/k_probe_sweep.sh`'s `run_plain`. A note is posted on INTENT's log.
