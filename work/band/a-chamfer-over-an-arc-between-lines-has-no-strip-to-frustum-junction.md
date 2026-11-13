---
id: a-chamfer-over-an-arc-between-lines-has-no-strip-to-frustum-junction
kind: issue
title: A chamfer over a rounded outline refuses at its arc: no junction joins a strip to a frustum
status: open
opened: 2026-11-13
priority: P1
cost: H
---


Found by the plane–cylinder chamfer lane (band-borecham, PR 6904), which built the chamfer's frustum arm over WHOLE plane–cylinder rims and measured the case below without building it.

A rounded outline's top edges (lines and arcs meeting tangentially) chamfered all round, or an open run of them, is one chain whose arcs are plane–cylinder links (`BlendArm::CylinderPlaneCone`) beside plane–plane ones (`BlendArm::PlanePlaneStrip`). The battery refuses such a chain at its first frustum link, `ChamferArmUnsupported` (`run_battery_for`, the gate right after `walk_chains`, `crates/sweep/src/blend/battery.rs`), pinned by `band_chamfer_frustum::an_arc_between_lines_refuses_at_the_arc`, `band_closed_mixed_chain::a_rounded_plate_chamfered_all_round_refuses_at_its_arcs` and `band_open_mixed_chain::an_open_mixed_chain_chamfered_refuses_at_its_arc`.

The geometry is exact. At a tangent junction the strip's feet and the frustum's lie at the same two points (one setback along each support's inward unit), so the strip's section and the frustum's generator there are one segment, and the two bands share their tangent plane along it (both contain the segment and the edge's common tangent). So the two meet G1 along a straight SECTION, as the fillet's cylinder and torus meet along the ball's circle.

What it would take, measured against the fillet's mixed rim and open mixed chain:

- **The section** (`crates/sweep/src/blend/junction.rs`): `junction_plan` and `rim_phase_mixed` read the section as `arms::BallSection` (a circle about the spine), which is `None` for a strip and a frustum. A chamfer needs a straight section between the two feet, its carrier a line, described as the shared row of a plane and a cone (a generator, `u` constant on the cone), which `geom_brep::analytic_shared_row` does not take today (`ContactCarrier::Section` in `crates/sweep/src/blend/surgery.rs`).
- **Predicate 4** (`broken_at_turns` and the band reading at a junction of links on different supports, `crates/sweep/src/blend/battery.rs`): it reads the two balls' section centres; for a chamfer it would read the two feet pairs, which coincide by construction.
- **The rest is shared**: the open band's admission (`OpenBand::admit` in `admit.rs` admits a torus between two junctions; its `between` would read `BlendArm::is_coaxial` instead), the frustum's windowed reach (`reach::circular_reach` already takes a frustum and windows an arc between sections) and its windowed support-boundary meter (`StripLevel::of` reads any coaxial band's circle trims), the junction moves, and the `BandSection` name.
- Remove the battery gate, and pin a rounded plate chamfered all round and an L-plate's rounded corner run at Pappus (the strips' prisms and the frustum's quarter rings `π·s²·(R ± s/3)/2` each).

P1 for the reason the plane–cylinder row was: "chamfer all round" on an ordinary part — a rounded outline — is the chamfer twin of the fillet's built mixed rim. H: a new section kind and its certified description.
