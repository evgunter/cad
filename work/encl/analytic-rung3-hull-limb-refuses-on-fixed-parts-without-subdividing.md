---
id: analytic-rung3-hull-limb-refuses-on-fixed-parts-without-subdividing
kind: issue
title: geom-brep: the AnalyticRung3 hull limb (net_offset_sup) refuses on its fixed INCIDENCE_CUTS parts, with no adaptive subdivision and no measured value at a break
status: open
opened: 2026-10-10
priority: P3
cost: H
---

Filed by `encl/hull-bound-refine` (row `hull-bound-refusals-could-refine-the-composite-before-refusing`), which made the two SSI hull limbs (`ssi_hull_sup`, `ssi_hull_sup_chart` in `crates/geom-brep/src/ssi/certify.rs`) subdivide their composite under a budget before refusing.

## What

The third Bernstein control-hull limb is not in that change. `edge_nurbs::analytic_rung3` decides `pcurve_cache::projected::net_offset_sup` once: its bound comes from `pcurve_cache::projected_hull_lane`, which cuts each knot span into a fixed `projected::INCIDENCE_CUTS` parts, and `projected::net_incidence` converts each part's composite bound to metres. A part that does not clear refuses the limb (`AnalyticHull`, `Unsized::Bound`) with no further halving, and no value at a break is read.

## Why it was not taken with the SSI limbs

- `projected_hull_lane` is shared with the projected pcurve row's check 4 (`projected_envelope`), and it is reached through `FittedLane`'s function pointer, so adaptive cuts change that door's signature and both consumers.
- A measured miss needs a LOWER-bound metres conversion per chart kind. `net_incidence` converts an upper bound (`f / (R + max(ρ, R − f/R))` on the cylinder and sphere, root-based on the cone and torus); a lower bound needs `ρ`'s upper end and new certified arithmetic per kind.

## Repair shape

Pass the cuts through `projected_hull_lane` and halve the parts whose converted bound does not clear, under the same budget as `ssi::certify::subdivide` (`SSI_HULL_ROUNDS`, `SSI_HULL_CUTS`). Read the canonical composite's value at a break (`CompositeForm::break_values`) as the measured residual, once a lower-bound conversion exists for the kind.
