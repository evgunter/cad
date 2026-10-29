---
id: the-torus-gaps-rounding-charge-is-first-order-and-reads-the-world-origin
kind: issue
title: The torus gap's rounding charge is a first-order estimate measured from the world origin, so a pose decides by where it sits, and G1 near-miss saddles escalate
status: open
opened: 2026-10-28
priority: P2
cost: M
refs: [each-torus-pair-decides-tangency-on-one-pose]
---


Filed by SECT's orchestrator from the confirmation passes of PR 5924's
dual review (2026-10-28).

## Measured

- **The charge is not a certified bound.** `pair_section/gap.rs`'s
  `Rounding` is charged by `running::rounding_charge` (`SHORT_CHAIN_ULPS`),
  which is a first-order estimate of the readings' rounding. It held
  against every probe both reviewers ran, but nothing proves it bounds
  the rounding.
- **It reads the world origin.** The charge's `size` is measured from the
  world origin (`gap.rs` `read`), not from the pose. So a pose translated
  far from the origin escalates at gaps 10–100× the escalation width
  where the same pose near the origin decides. R1 measured this for G1
  joints at k ≥ 30, where δ from 1e-7 to 1e-6 mostly escalates.
- **G1 near-miss saddles escalate.** These are two equal-tube bends whose
  cores pass a few ε apart, with gap ≤ 0.5ε. They are genuine
  tangencies that the previous head decided Zero, and now they escalate:
  61 poses in R2's probe. That is sound, but the shipped default loses a
  decision on a realistic pose.

## What a fix has to supply

- A rounding bound that is certified, either in the certification
  arithmetic or derived.
- That bound measured at the pose's own scale: from the pair's own centre
  or pivot, never the world origin.
- Then a fresh reading of which near-miss saddles can decide.

## It reads the torus's `u_ref` too (SECT, 2026-10-28)

Measured on `sect/torus-doors` at `CAD_TOLERANCE_EPS=1e-12`: the
straight pipe entering its bend (torus `R = 1`, `r = 1/4` about `z` at
the origin; a cylinder of radius `1/4` along `y` through `(1, 0, 0)`,
the tube-circle tangency, the pose `torus_gap.rs`
`the_tube_circle_tangency_decides_zero` decides Zero at `R = 2`)
decides Zero (`ct_critical_gap`, margin `2.8e-13`) when the torus's
`u_ref` is `+x`, the tangency at the sweep's first cell edge, and
escalates (margin `9.82e-12`, inside `[1e-12, 1e-11]`) for `u_ref` `+y`,
`(0.51, 0.86, 0)` and `(−0.86, 0.51, 0)`. The torus is the same surface
in each; only its parameter origin moves, and with it where the sweep's
cells fall about the critical point. At the default ε every `u_ref`
decides Zero. So the gap's verdict at a tight band depends on the chart
origin as well as the world origin, the frame-dependence this row's fix
has to remove. `crates/geom-brep/tests/torus_doors_ssi.rs`
`a_tube_circle_tangency_refuses_at_each_door` reads the pose with
`u_ref = +x` for that reason.

## Evidence: a square pipe at scale reads tangent far past the band (SECT, 2026-10-29)

`topo` `section_cert::square_search::the_square_wall_arm_agrees_with_the_traced_section`
at ε 1e-12 (`CAD_FUZZ_SEED=0x5bd605a63756e366 CAD_FUZZ_EFFORT=1`), found
on `sect/components-by-foot`, whose diff does not touch the tangency
path: "a pipe along the core's tangent", `R 46.22`, `r 34.60`,
`(e, z0, rc) = (−46.22141309604856, 3.26e-10, 34.60196814898672)`, tilt
`2.8e-20`, answers R-tan on `ct_critical_gap` where the traced
degeneracy stands `2.67e-10` from tangency, past the search's `16 ×
escalate` allowance. The pose's scale (`R + r ≈ 81` from the origin)
fits the charge being read from the world origin.
