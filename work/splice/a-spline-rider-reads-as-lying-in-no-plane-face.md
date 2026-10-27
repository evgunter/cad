---
id: a-spline-rider-reads-as-lying-in-no-plane-face
kind: issue
title: the slit pass's lies_in reads a spline or spiric edge as lying in no plane face, so a rider skips its EdgeInFaceGap refusal
status: open
opened: 2026-10-25
priority: P1
cost: E
refs: [delete-the-boolean-operand-edge-gate, nurbs-edge-crossing-rung-is-the-ring-composite]
---


Filed by `delete-the-boolean-operand-edge-gate` (its sweep for the
"curved carrier read as a line" class).

## Finding

`crates/topo/src/boolean/slit.rs` `lies_in` (called by `part_riders`)
answers whether an edge with both ends on a plane face lies in it. A
line and an in-plane conic are read; every other carrier falls to
`_ => return Ok(false)`, so a spiric or spline edge lying in the other
operand's plane face reads "not in it". `part_riders` then never
lists it, and it skips the `EdgeInFaceGap::Curved` refusal an
in-plane conic rider gets. The module doc leaves such an edge to the
result gate's `line_wedges`, which reads lines only, so the rider
would stand as one edge with no refusal.

**Latent, not live.** A spline edge claims no box
(`boxes::EdgeBoxRule::NoSoundBox`), so the sweep's planar arm meets it
with every plane face of the other operand and refuses it
(`EdgeCarrierUnsupported { site: PlanarCrossing }`) before the slit
pass runs; a spiric edge whose box meets a plane face refuses at the
same arm. It becomes live the day a crossing rung lets either kind
past the planar arm (`nurbs-edge-crossing-rung-is-the-ring-composite`).

## Fix

Give `lies_in` an arm for the two kinds that refuses typed (or reads
them, once a rung places a point on one), with a row driving
`part_riders` on a spline edge lying in a plane face.

## 2026-10-26 — re-pointed (CLEAVE, PR 5793)

`lies_in` is gone: the reading is `slit::holding`, and the carrier
question it asked is `slit::carrier_on_surface`, which reads a line or
a conic on a plane, cylinder, cone or sphere and answers every other
pair, a spline or spiric carrier among them, `Ok(false)` by its last
arm. The finding stands as written against that arm.
