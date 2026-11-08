---
id: a-spline-rider-reads-as-lying-in-no-plane-face
kind: issue
title: the slit pass's carrier_on_surface reads a spline or spiric edge as lying in no plane face, so a rider skips its EdgeInFaceGap refusal
status: closed
opened: 2026-10-25
priority: P1
cost: E
refs: [delete-the-boolean-operand-edge-gate, nurbs-edge-crossing-rung-is-the-ring-composite]
branch: splice/kind-reads
closed: 2026-11-03
pr: 6437
---


Filed by `delete-the-boolean-operand-edge-gate` (its sweep for the
"curved carrier read as a line" class).

## Finding

`crates/topo/src/boolean/slit.rs` `carrier_on_surface` (through
`holding`, `riders` and `part_riders`) answers whether an edge's
carrier lies on a face's surface. On a plane face a line and an
in-plane conic are read; every other carrier falls to
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

Give `carrier_on_surface` an arm for the two kinds that refuses typed (or reads
them, once a rung places a point on one), with a row driving
`part_riders` on a spline edge lying in a plane face.

Moved from SPLICE to SPLICESEAM on 2026-10-28, by `git mv` in SPLICE's split along its seams (Track size; Ev, in chat, 2026-10-28).

## 2026-10-26 — re-pointed (CLEAVE, PR 5793)

`lies_in` is gone: the reading is `slit::holding`, and the carrier
question it asked is `slit::carrier_on_surface`, which reads a line or
a conic on a plane, cylinder, cone or sphere and answers every other
pair, a spline or spiric carrier among them, `Ok(false)` by its last
arm. The finding stands as written against that arm.

Moved from SPLICESEAM to SPLICE on 2026-11-02, by `git mv`, when SPLICESEAM merged back into SPLICE: the track budget now counts only the P0, P1 and unbanded rows a sitting dispatches (Ev, in chat, 2026-11-02), so the four tracks of SPLICE's 2026-10-28 split came in under one budget.

## Closed (SPLICE, PR 6437, 2026-11-03)

`slit::holding` reads where a carrier lies on a face as `Placed::{On, Off, Unread}`. A spline carrier reads `Off` only when its whole control net clears the plane by more than the band (the hull, certified), and is never read `On`. A spiric carrier on a plane reads as a conic does, and a line on a sphere or ring torus reads `Off`. An unread carrier refuses `EdgeInFaceGap::CarrierUnread` wherever any record relates it to the face (an end on it, a crossing of its loops, or an edge-on-face record), never on a sampled point; `sectors::closed_reach` refuses on unread. The review's notch and keel cases are rows. Four builds now refuse rather than ship on a sampled read (the lofted cylinder's far `O ∪ B` and `B ∪ O`, around `B ∖ O`, and the tilted rod's β 0.7 union). Their way back is `an-unread-rider-related-only-by-its-own-vertices-refuses` (P0).
