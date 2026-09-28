---
id: must-carry-lane-gate-hides-a-transverse-out-of-lane-pair
kind: issue
title: must_carry_over_edge answers UnderDetermined for a transverse edge on an out-of-lane pair: the lane gate runs before the first-order gate
status: open
opened: 2026-09-26
priority: P1
cost: D
---


## Finding

`crates/geom-brep/src/dihedral.rs`'s `must_carry_over_edge` asks
`tangent_certificate_lane` (`crates/geom-brep/src/tangent.rs`) before
anything else and returns `MustCarryVerdict::UnderDetermined` for a pair
outside it, so the per-station first-order gate (`classify_dihedral`,
added by PR 3295) never runs there. A transverse edge on an out-of-lane
pair therefore reads `UnderDetermined`, and a caller that acts on it stores
the conventional chart image for an edge whose honest description is
`Intersection`.

Measured in PR 3295's review: the plane `y = 0` crossing a 45° cone about
the z axis along a ruling (a `Line` carrier on a (Plane, Cone) pair, which
the lane refuses) reads `(UnderDetermined, UnderDetermined)` in the two
argument orders, while `classify_dihedral` reads `Transverse` at every
station.

Who can reach it today: `crates/sweep/src/extrude.rs`'s strut arm
(`sweep_loop`) and `crates/sweep/src/revolve/upgrade.rs`'s
`upgrade_intersection` classify the dihedral at the witness before calling
the rule, so a transverse pair never reaches it from those two.
`crates/sweep/src/blend/surgery.rs`'s `attach_contact` tangent branch has
no witness gate of its own; it relies on the carrier-kind routing to send
every crossing contact to the `Intersection` branch. A misrouted crossing
on an out-of-lane carrier (a `Line` against a cone or torus, any `Nurbs`
carrier or surface) would be stored conventionally with nothing
refusing it, where an in-lane one now refuses `SurgeryInvariant`.

## Question

Should the first-order gate run ahead of the lane gate? Doing so makes a
transverse pair answer `Transverse` in or out of lane, but it meters
`dihedral_arm`/`dihedral_wedge` on pairs that meter nothing today, which
moves the K stream (the K-REPORT runbook applies) and breaks the existing
row that pins an out-of-lane pair as metering nothing
(`crates/sweep/tests/must_carry_rule.rs`,
`the_rule_meters_the_schedules_interior_stations_and_the_gate_meters_nothing`).
`classify_dihedral` also escalates `Invalid` on a `Nurbs` surface's poison
implicit form, so an out-of-lane `Nurbs` pair would answer `InBand` where
it answers `UnderDetermined` today. The alternative is to leave the rule
as is and require every caller to gate first-order at its witness, as
extrude and revolve do.
