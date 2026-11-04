---
id: a-football-revolved-part-way-refuses-at-its-cap-meridians-certificate
kind: issue
title: A football revolved part way refuses at the meridian a cap meets the sheet on: the edge ends at both apexes, where Meters::Spindle's lever is not positive
status: open
opened: 2026-11-04
priority: P3
cost: M
---


(SPIRICARM, stage 2 of `horn-and-spindle-tori-have-no-representation`.)

## What

The football (an arc from one apex of a sheet's circle to the other,
closed along the axis) builds through `revolve(Full)`. Revolved
`Partial(θ)`, the two plane caps meet the sheet along meridians that end
at both apexes, and the edge's attachment refuses:
`RevolveError::Op` — "the analytic Intersection's rung-3 certificate
refused — the carrier's distance from the spindle has no certified
bound: its metre lever is not positive: it reaches the cone's apex or
other nappe, or a spindle torus's inner sheet". The lever is
`geom-brep`'s `projected::Meters::Spindle` (limb 2 on a sheet,
`pcurve_cache::limb2_offset_sup`), whose sheet factor `4|R|ρ` and side
check `|m − r|·|R| ≤ r·ρ` both read `ρ`, which is zero at an apex: an
edge that reaches an apex has no positive lever anywhere near it.

The full revolve does not mint such an edge between a plane and a sheet
(its meridians are sheet-sheet `Chart` edges), so it builds; a partial
revolve does. Pinned typed by
`crates/sweep/tests/the_football_revolves.rs`
`a_football_revolved_part_way_refuses_typed` (lemon and apple, θ = 2).

The message reads as a kernel defect ("an Euler operation refused")
rather than a lane not written.

## Done when

A lune of the football (`Partial(θ)`) builds, passes tier 3 and
measures its exact volume `θ/2π` of the football's, or refuses with a
typed lane-gap reason that names the apex.

The same refusal class: a horn's whole circle with its vertex at the
cusp, revolved part way, refuses `RevolveError::Op` (a stated interval
decided zero) rather than as the pinch the full revolve names
(`NonManifoldAxisContact`); `crates/sweep/tests/one_segment_loop.rs`
`a_one_segment_circle_on_or_against_the_axis_refuses` pins it.
