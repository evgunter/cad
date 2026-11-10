---
id: a-football-revolved-part-way-refuses-at-its-cap-meridians-certificate
kind: issue
title: A football revolved part way refuses at the meridian a cap meets the sheet on: the edge ends at both apexes, where Meters::Spindle's lever is not positive
status: dispatched
opened: 2026-11-04
priority: P1
cost: M
branch: spiricarm/spindle-partway
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
revolve does. The lune is now
`crates/sweep/tests/the_football_revolves.rs`
`a_football_revolved_part_way_is_its_lune`.

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

## Dispatched (2026-11-04)

Branch `spiricarm/spindle-partway`. Review tier: **single FULL review**. It changes an edge certificate near an apex: a wrong lever certifies an edge off its sheet, and believing the fix takes more than reading it, though it is narrower than a stage. Look first at the precedent: how a cone's partial revolve certifies the meridian that ends at its apex (`Meters`' cone arm, `apex_root`). Ask whether the sheet's apex wants the same treatment.

Every new or changed guard ships with a row, or a seeded fuzz, that goes red when its load-bearing check is removed (the stage 1 and stage 2 reviews). A wrong-X witness must reach the gate it names.
