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

Check 9 covers both shapes. None reaches `one_region` alone except in the cases named below.

- **Two distinct loops meeting at a point** are its contact arm 1
  (`RingMeetsOuter` / `RingMeetsRing` `Vertex`, on any carrier).
  Probe: `one_region::tests::a_ring_crossing_its_outer_loop_at_shared_points_is_check_9s`.
  A cylinder sheet's ring passes through two of its outer corners and
  crosses the outer loop there. `validate_geometric` refuses it with
  `RingMeetsOuter { contact: Vertex }`. Only arm 1's verdict is read:
  the ring's edges are straight chords, so the body is otherwise
  ill-formed.
- **Two surface cycles of one loop sharing a point key.** A cycle is a
  run of the loop's own half-edges, so a point key both cycles pass is
  one the loop passes at two or more corners. `pinch_corner_errors`
  groups exactly those (`loop_point_groups`, by point-key equality). It
  reads each corner's wedge against the loop's other sides there,
  including the seam or strut half-edges the surface boundary drops, on a
  curved face in its tangent plane. A crossing there is
  `PinchCornerCrossed`. A touch there is refused too, but not by arm 1's
  rule for two loops. The strut splits the loop so that each cycle's
  corner at the point pairs that cycle's own two edges, and the outer
  corner's wedge holds the other cycle's edges, so neither corner is a
  slice of the face. A legal pinch is one loop whose corners at the
  point pair the two lobes' edges, each a slice (Ev's rule, PR 4057). It
  passes: `validate::tests::a_figure_eight_hole_with_a_closed_arc_passes`.
  Probe: `one_region::tests::two_surface_cycles_of_one_loop_crossing_at_a_shared_point_are_check_9s`.
  - A cylinder sheet, its sense set so its loop bounds its rectangle,
    has an outer loop carrying a strut, from the corner after `A`, to an
    inner polygon closed by a membrane, with the strut kept. No polygon
    edge runs along the strut. A first version led the strut from `A`
    itself, so the polygon's edge to `A` lay along it, a degenerate
    encoding. That is one loop, which
    `surface_cycles` walks as two cycles (asserted).
  - With the cycles apart, nothing is refused at the outer corner.
  - With the polygon's corner moved onto the outer corner's point key,
    `validate_geometric` refuses `PinchCornerCrossed` there, whether the
    polygon crosses the outer cycle or touches it. Both come through the
    same mechanism, the outer corner's wedge holding the polygon's edges,
    so the row proves that no meeting of the two cycles at a shared key
    reaches `one_region`, not that the arm tells a crossing from a touch.
  - A first version of this probe read an unsensed sheet, which check 6
    refuses (`CurvedSenseInverted`). Its corners were complements, so
    the corner arm fired with the cycles apart too. The apart case now
    guards against that.
  - Only the corner arm's verdict at that point is read: the edges are
    straight chords, so the body is otherwise ill-formed.

A strut is an edge the face lies on both sides of. `surface_cycles` drops
it, so it adds no point the two cycles share.

**What the corner arm does not cover,** where `one_region` reads alone:
- `pinch_corner_errors`' stated residue (`validate::pinch_corner_errors`'
  doc): a cone's apex, which `one_region` does read (filed as
  `two-cycles-crossing-at-a-cones-apex-are-read-by-their-first-vertex`);
  a NURBS or `Approx` face; a side along a NURBS edge; two sides leaving
  along one tangent; a cusp corner.
- **Two distinct point keys within band** of one place, which
  `loop_point_groups` does not group. Measured: neither the corner arm
  nor arm 1 refuses it. That is the real residual, filed as
  `a-loop-through-one-place-at-two-point-keys-is-read-by-its-first-vertex`.
