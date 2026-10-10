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
