---
id: one-region-reads-a-cycle-crossing-at-a-shared-vertex-by-its-first-vertex
kind: issue
title: one_region::read_cycle reads two surface cycles of one loop (split by a seam or strut) that cross at a vertex they share by its first placed vertex (Answering::Any), so a CycleOffFaceSide can pass unreported
status: closed
opened: 2026-11-09
priority: P3
cost: M
closed: 2026-11-09
---


## Finding

Filed by `onecopy/pinch-premise` (PR 6605's review). `one_region::read_cycle`
reads a curved face's cycle against the face's other cycles at each of
its vertices, through an `Answering::Any` ladder, and the first vertex
placed answers. A vertex on a cycle of the cut is set aside.

The premise that every placed vertex reads one side holds where the cycle
does not cross the cut, and a touch at a shared vertex switches no side
(`chord_join::paths_rung`'s premise). It is not checked here, and this is
a validator: nothing upstream joins the face's own cycles.

- A crossing inside two edges is the census's `EdgeEdgeCross`.
- Two distinct loops meeting at a point are check 9's contact arm 1
  (`validate::ring_outer_contact_about`, any carrier, shared keys not
  exempt: `RingMeetsOuter` / `RingMeetsRing` `Vertex`, per Ev's rule at
  PR 4057).
- What may survive is **two surface cycles of ONE loop**, split by a seam
  or a strut, that share a vertex and cross there. It is not shown to be
  any check's.

Such a cycle has placed vertices on both sides. Which one answers depends
on walk order, so `CycleOffFaceSide` can be missed.

**Probe:** a curved face (a cylinder sheet with a seam) whose one loop
walks as two surface cycles sharing a vertex that cross there, read by `region_errors` in
every walk order. If another check refuses the body, record which check
and close this row. If not, make the site read every vertex (the agreeing
check, `a-ladder-that-checks-its-answers-agree-has-no-home`, after
`[ev]` PR 6592) or refuse a crossing at the shared vertex.

## Closed (`onecopy/shared-vertex-crossing`): check 9 catches it

Check 9 covers both shapes.

- **Two distinct loops meeting at a point** are its contact arm 1
  (`RingMeetsOuter` / `RingMeetsRing` `Vertex`, on any carrier).
  Probe: `one_region::tests::a_ring_crossing_its_outer_loop_at_shared_points_is_check_9s`.
  A cylinder sheet's ring passes through two of its outer corners and
  crosses the outer loop there. `validate_geometric` refuses it with
  `RingMeetsOuter { contact: Vertex }`.
- **Two surface cycles of one loop sharing a point.** A cycle is a run
  of the loop's own half-edges, so a point both cycles pass is a point
  the loop passes at two or more corners. `pinch_corner_errors` groups
  exactly those (`loop_point_groups`, by point key) and reads each
  corner's wedge against the loop's other sides there, the dropped
  seam or strut half-edges among them, on a curved face in its tangent
  plane. A crossing there is `PinchCornerCrossed`.
  Probe: `one_region::tests::two_surface_cycles_of_one_loop_crossing_at_a_shared_point_are_check_9s`.
  - A cylinder sheet's outer loop carries a strut to an inner polygon
    closed by a membrane, the strut kept. That is one loop, which
    `surface_cycles` walks as two cycles (asserted).
  - One polygon corner is moved onto the outer corner's point, with its
    neighbours one inside the sheet and one outside, so the cycles cross
    there.
  - `validate_geometric` refuses it with `PinchCornerCrossed` at that
    point.

**Not this row's:** two cycles at one location through distinct point
keys. `loop_point_groups` groups by key, so the corner arm does not read
them. They are a vertex–vertex contact, which tier 3′'s census reads
(`UnrecordedContact`, then `BoundaryCrossesItself` where a record
admits it).
