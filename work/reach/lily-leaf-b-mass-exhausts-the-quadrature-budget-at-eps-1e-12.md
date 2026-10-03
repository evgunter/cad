---
id: lily-leaf-b-mass-exhausts-the-quadrature-budget-at-eps-1e-12
kind: issue
title: The k-probe sweep panics at eps 1e-12: lily_leaf_b's mass exhausts the quadrature budget (since #3838)
status: review
opened: 2026-10-02
priority: P1
pr: 3976
branch: reach/lily-leaf-1e12
---


Found by REACH while checking PR 3853 (the dev-probe leg). The nightly's
`k-lint (dev-probe)` row runs `scripts/k_probe_sweep.sh`, and its
demo-scene pass at ε 1e-12 panics at `demos/tour/src/probe.rs:73`:

```
lily_leaf_b: mass at Probe: Face { face: FaceKey(3v1), source: QuadratureBudget { width_len: 1.5433422007510428e-8, target_len: 1.024e-9, rounds: 1 } }
```

## Measured

`CAD_TOLERANCE_EPS=1e-12 cargo run --features probe -- k-probe <csv>` in
`demos/tour`:

| tree | result |
| --- | --- |
| `416ff76738^1` (main just before PR 3838) | every scene probes, `lily` and `lily_walls` included |
| `416ff76738` (PR 3838, the lanceolate blade sections) | panics as above |
| `ff6dfd40c9` (main, 2026-10-02 ~22:40 UTC) | panics as above |

So PR 3838 made the panic. Its leaf_b face 3 asks the quadrature for a
1.024e-9 m target width, and one round leaves it at 1.54e-8. The demo
k-probe pass at ε 1e-9 is green on all three trees. The PR gate runs no
probe pass, which is why this merged green (see
`work/ciw/tests-red-under-all-features-never-run-by-ci.md`).

## Wanted

Either the leaf's mass reaches its target at ε 1e-12, or the probe pass
treats a typed `QuadratureBudget` refusal as a sample rather than a
panic. Which of the two is SHOW's call, with QUAD's budget item
(`work/quad/`, filed with PR 3838) in view. Not REACH's ground, so not
fixed here.
