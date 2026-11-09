---
id: a-corner-reads-only-its-own-sectors
kind: unit
title: a corner meeting an unglued pair reads only its own sectors: no carrier re-read, unglued_coincidence and unglued_tilt retire, an off far bound ends Coincide::Sectors
status: open
opened: 2026-11-09
priority: P1
cost: M
parent: glue-disagreements-and-held-contradictions-after-stage-4
---


Step 4 of the parent's design (fork `glue-disagreements-carry`, 2026-11-09). Step 1 gave `bounds_off` its `Decided` (`Result<Decided, Indeterminate>`), and this row was opened when step 1 closed.

## What

Today a corner that meets a pair the glue door did not glue re-reads the carrier pair over its own arm: `carrier_eq_reading` in recl.rs and vtxfac.rs. `unglued_tilt` then decides `bool_plane_parallel` a third time. All of it serves only to build a refusal, and C22 (`unglued_coincidence`) is that refusal.

- **`recl` and `vtxfac` stop calling the carrier ladder.** These go with it: `unglued_coincidence`, `unglued_tilt` and both "ON sector with definitely-distinct carriers" invariants.
- **The glue door keeps no verdict table.** No corner branches on the door's Apart; it would feed only wording.
- **Precedence, as today (`unglued_corner`):**
  1. the bounds' reading, where it offers a tolerance (`Coincide::Sectors`);
  2. else the door's in-band reading (`door_refusal`);
  3. else the sector's own funnel reading.
- **The open case: `ReadOn::Faces`.** The near arm meets within band (`fan::meeting`'s `bool_faces_parallel` decided Zero at the shorter arm), while `bounds_off` decides the far bound off.
  - These are two readings of the sector over two extents. The case is reachable on valid input: a 1 mm and a 10 m bound tilted by θ with 1 mm·θ < ε and 10 m·θ > Kε.
  - It ends `Coincide::Sectors` with `Off(Decided)` (the far bound's departure), told by the Sectors lever.
  - `gate_rejection` is not available here: `Sectors` passes `AnySign`, so the door's `unreachable!` would fire.
- **`ReadOn::FirstBounds`** reads one quantity twice (`BoundSupport::departure`). It ends the same way, retiring its `ClassificationInvariant`, unless a probe shows the two reads cannot disagree.
- topo README C4's sentence "An unglued pair is read at a corner by the ladder over the corner's own arm, where every verdict but `Distinct` refuses" retires with the code. It is agent text (`c7571919b0`, PR 6079) with no ratification found.

## To probe before filing anything

- On the `Faces` path, a tolerance that `bounds_off` offers above `bool_faces_parallel`'s margin at the short arm may still read Coincident and refuse again.
- Whether a fold step's corner reaches this arm.

## Done when

No corner reads a carrier pair. Both `ReadOn` arms end on the sector's own reading, pinned by a scene of the 1 mm / 10 m shape.
