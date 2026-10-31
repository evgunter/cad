---
id: the-chart-and-pair-tubes-are-held-by-their-coarsest-box
kind: issue
title: The chart and four-dimensional tubes are held to the rung of their coarsest box, as the R3 tube was
status: open
opened: 2026-10-28
priority: P3
cost: M
---


Found by the sweep of `main-red-cylinder-sphere-section-runs-out-of-cells-near-the-node-at-1e-6`
(SECT, 2026-10-28). Unmeasured on these lanes.

## The shape

Limb 3 takes one ladder rung for a whole chain
(`crates/geom-brep/src/ssi/certify.rs` `limb_three`), so the chain's
coarsest box sets the tube everywhere, and the exhaustiveness
accounting then subdivides the whole domain to that rung. On the ℝ³
analytic lane this held a cylinder × sphere loop near its node at ε =
1e-6 to a rung of 1.7e-4 (the default ε's was 5.5e-3) and spent
`CellBudget { budget: 200000 }`: the march's samples, spaced as
`ε^¼`, left boxes up to 3.4e-2 wide where the surfaces cross at a
small angle. That lane now refines the samples that hold its tube
(`ssi::refine::refine_for_tube`, called from `ssi::finish_r3`).

The plane × NURBS chart lane (`probe_tube_chart`, accounted by
`exhaust::account_chart_plane` through `ssi::ends::Ends::finish`) and
the NURBS × NURBS ℝ⁴ lane (`probe_tube_pair`, accounted by
`exhaust::account_pair`) take their rung the same way and are not
refined for it.

## What closing it takes

Measure whether a coarse ε holds either lane's tube below its default-ε
rung on a near-tangent fixture. If it does, give it the same
refinement (the `coarse` reading is a chain box wider than the next
rung up whose padded box is not a graph); if not, close it with the
measurement.
