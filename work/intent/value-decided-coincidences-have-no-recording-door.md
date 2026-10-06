---
id: value-decided-coincidences-have-no-recording-door
kind: issue
title: D10: a coincidence decided from values has no door to be recorded at; the blend's isosceles-turn verdict is held on BatteryVerdict::coincidences, read by nothing
status: open
opened: 2026-10-06
---


Filed by BAND (step 4 of
`work/band/a-plane-plane-blend-cannot-end-at-an-unrequested-corner.md`,
the mitre), under Ev's answer on
`isosceles-mitre-reads-as-an-unproven-coincidence-on-every-box`: land the
mitre under option (b), recording its isosceles verdict as a
value-decided coincidence.

## The gap

D10 says every coincidence the kernel infers from values is recorded
at the one door where structure is decided, for the
`unproven-coincidence` lint. That door is stage 4 of
`work/intent/plan.md` and is not built: no type in the tree is a
record of a value-decided coincidence (the nearest names,
`topo::contact::BooleanCoincidence` and the boolean's declaration
reads, are the pre-D10 declared-pair system stage 4 retires).

## The seam BAND left

The battery decides `fillet3_turn_isosceles` Zero at every isosceles
turn and records it as a typed value:
`sweep::blend::battery::DecidedCoincidence::IsoscelesTurn { vertex, reading }`,
held in `BatteryVerdict::coincidences` (one row per turn, in vertex
order; `crates/sweep/src/blend/battery.rs`, `turn_at` and
`run_battery_for`). Nothing reads it but the row that pins it
(`crates/sweep/tests/band_planar_mitre.rs`,
`an_isosceles_turn_is_recorded_as_a_value_decided_coincidence`). It
does not leave the battery: `Blended` does not carry it, so the
document layer never sees it.

## What closing it takes

The stage-4 door, and this seam routed into it: the verdict's records
carried out of the blend into whatever the door's input is, with the
vertex resolved to a name. Ev's answer says how the box mitre is then
proven structural (output definitions plus rung 3), so the lint stays
quiet on it.
