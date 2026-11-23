---
id: three-answers-to-is-this-loop-inside-that-one
kind: issue
title: three answers to "is this loop inside that one" with three boundary postures (shell::encloses, chord_join::rehome_rings, validate::ring_nesting), plus a fourth hand read of Surface::Plane's chart normal
status: closed
opened: 2026-09-14
priority: P1
cost: M
branch: walks/loop-in-loop
closed: 2026-11-23
---

Filed by `tier3-accepts-a-ring-outside-its-outer-loop`'s fix pass, from
its R1 review's class finding. The unit itself retired one copy of a
smaller duplicate (its `outer_loop_is_a_polygon` gate, now
`boolean::loop_shape`); this row is the class that duplicate belonged
to, and it is not this unit's to sweep.

## Three answers to one question

`crates/topo` decides "does loop A lie inside loop B, both coplanar"
in three places, by three instruments, with three different postures on
the boundary case:

1. **`shell::encloses`** — mean radius of each loop's sampled points
   about the pair's common centroid, one `decide("shell_rim_nesting")`.
   Its own doc scopes it to loops "concentric by construction", which
   is true of its callers (a slit chart's two sides, a hole and its own
   offset) and false in general: a thin cross with a ring threading one
   arm is genuinely nested and reads un-nested here, measured
   (`validate::tests::mean_radius_nesting_is_not_a_containment_statement`).
   Boundary posture: none — the decide's `Zero` is simply "not
   enclosing".
2. **`chord_join::rehome_rings`** — `splitting::point_in_loop` on one
   RING REPRESENTATIVE point against the survivor's outer loop.
   Boundary posture: `OnBoundary` is a typed refusal
   (`SplitJoinError::RingHomingAmbiguous`).
3. **`validate::ring_nesting`** (this unit) — every ring vertex in
   cycle order, first definite verdict wins. Boundary posture:
   `OnBoundary` settles nothing and the walk moves on, because check
   9's contact half owns that question. **2026-09-24 (ATREST-5, PR
   #3179): two instruments now, dispatched on the outer loop's
   `contain::loop_shape` class** — `point_in_loop` for
   `Polygon`, `contain::disc_side` for `Disc` (made
   `pub(crate)` for it); `ArcParity` and `NoWalk` are silent. That
   widens the divergence this row tracks: `ring_nesting` and
   `boolean::contfp` now dispatch on the loop's shape, `rehome_rings`
   still does not
   (`work/cleave/rehome-rings-reads-an-arc-bearing-run-through-the-polygon-walk`).
   **2026-09-25 (ATREST-12): one instrument again, and no shape
   dispatch** — `splitting::containment::point_in_carrier_loop` (the
   walk that reads each edge on its own carrier, ATREST-9) on every
   planar outer loop; `NestingRegion` and the `loop_shape` projection
   are deleted, `disc_side` is no longer a check-9 instrument, and the
   arm is silent only where that walk returns no answer (a spiric or
   spline outer edge, the query inside the ball holding the loop).
   `ring_nesting` now shares its walk with `solid_contain::point_in_face`;
   `contfp` still dispatches on `loop_shape` and `rehome_rings` on
   nothing. **2026-10-01 (CLEAVE,
   `rehome-rings-reads-an-arc-bearing-run-through-the-polygon-walk`):
   `rehome_rings` reads the run through `point_in_carrier_loop` too**,
   so the two now share the instrument and differ on the sample set
   (one representative against every vertex) and on the boundary
   posture, which is this row's question.

Two of the three agree on the instrument and disagree on the sample set
and on the boundary; the first agrees with neither and is not a
containment statement at all. None of the three is wrong at its own
call site today — what is missing is any statement of WHY the postures
differ, so a fourth caller has no way to choose and the obvious move
(reuse the nearest one) is the wrong one about half the time.

## The fourth copy: reading a plane's chart normal by hand

`rehome_rings` reaches its normal through `chord_join::face_plane_normal`,
which matches `Some(geom::Surface::Plane { normal, .. })` and refuses
typed otherwise. `validate::nesting_normal` (briefly `nesting_region`,
2026-09-24 to 2026-09-25) does the same match inline, as do `validate`'s check-6 arm, `merge_faces`, `replace_face`,
`revert`, `boolean::join` and `face_normal` — nine sites in
`crates/topo/src`, three of which multiply by `sense_sign` and the rest
of which do not. `face_normal.rs` already owns the OUTWARD normal door
and inventories every hand multiply of the ±1
(`every_hand_multiply_of_the_face_sign_is_inventoried`); there is no
equivalent door, and no inventory, for the CHART normal read that comes
first.

## What this row asks

Not a rewrite. A decision, then whichever of these follows from it:

- whether the chart-normal read belongs beside `face_normal`'s outward
  door as a second named door, with the same inventory row over its
  hand-written copies;
- whether `rehome_rings` and `ring_nesting` should share one
  loop-in-loop helper parameterised on the boundary posture, or whether
  the two postures are genuinely different questions and each should
  say so where it asks;
- whether `shell::encloses` keeps its name. "Encloses" reads as a
  containment statement and is not one; a name that said "concentric
  order" would not have needed a validator unit to discover the gap it
  leaves.

## Sweep and its blind spot

The hit list above is a grep over `crates/topo/src` for
`Surface::Plane { normal` and for the three named functions, plus a
reading of every `point_in_loop` caller. It cannot match a containment
claim made through a helper that already returns a normal, one stated
only in prose naming none of those symbols, or any site outside
`crates/topo/src`.

## Staleness audit (2026-11-22, WALKS, origin/main 7709b0ea)

Premise moved; renames: `point_in_carrier_loop` → `splitting::containment::point_in_loop`; `SplitJoinError::RingHomingAmbiguous` → chord_join `JoinError`; `nesting_normal` → `validate::plane_chart_normal`; `Surface::Plane` carries a `frame`, so the row's grep matches nothing. `rehome_rings` no longer reads one point: `ring_side` reads every vertex with `Answering::Any`. `loop_side.rs`'s header now states the posture split (validators read every vertex; construction sites the first, on the operation's premise). Residual drift: on a ring on the boundary at every vertex, `rehome_rings` refuses or defers while `ring_nesting` accepts. Unlisted siblings: section_loops `nest`'s `encloses` closure (one representative point; OnBoundary and Err read as not enclosed), finish `nest_slit`, fate `rehome`, one_region. Chart-normal reads: chord_join `face_plane_normal`, validate `plane_chart_normal`, solid_contain `chart_normal` (via `implicit_gradient`). `ring_nesting`'s stale "first definite verdict" doc is fixed by WALKS's E-bundle. Being weighed as fork F6.

## Decided

- `crates/topo/src/loop_side.rs` is the one home of "is this loop
  inside that one" read vertex by vertex. Two readings, each named for
  who vouches that the loop does not cross the boundary:
  `inside_at_every_vertex` (the validator: every vertex, a crossing is a
  finding) and `side_of_a_loop_the_division_does_not_cross` (a
  construction site: the first vertex that places, the operation's own
  division the premise). `of_loop` is the one map from `point_in_loop`'s
  reading to the per-vertex rung; `chord_join::vertex_side` is gone and
  check 9 reads through it. `boolean::fate::rehome` reads a curved
  face's placement, not a `LoopContainment`, so it keeps its own rung
  and climbs the construction reading.
- `EveryVertex` tells `OnBoundary { at }` (every vertex on the
  boundary) from `NoAnswer` (the instrument's silence).
- Check 9: a lone-vertex ring is read by contact arms 1 and 2 as the
  point it stands at, and a ring on its outer loop at every vertex is
  `RingMeetsOuter { VertexOnEdge }` at the edge the walk's boundary pass
  placed it on (`contain::loop_placement`, the face door's boundary pass on one loop). No new public
  variant. `one_region::read_cycle` keeps exempting a cycle on the cut,
  with the pinch reason written there.
- `Body::loop_vertices` is the one walk of a loop's vertices, a lone
  vertex's included (chord_join, fate, check 9's two arms).
  `one_region::read_cycle` walks a surface cycle of half-edges, not a
  loop, so it does not take it.
- `shell::encloses` is `is_inner_of_concentric_pair`, out of the
  containment family; instrument unchanged.
- `point_in_loop` takes the loop's plane as a `geom::Frame`.
  `nest_slit` hands the split plane's frame with no per-side flip;
  `section_loops::nest`'s callers hand `section_frame(plane, side)`,
  which keeps the side's outward sign for the contact arms they also
  run. `chord_join::face_plane_normal` is deleted (both callers already
  matched the `Plane`); `validate::plane_chart_normal` is
  `validate::plane_gate`, returning the frame.
- `section_loops::nest`'s one-point read says why one point is enough;
  `nest_slit`'s doc already did.

## Closed

2026-11-23: PR 7475 merged — `loop_side.rs` is the one home (the validator reading and the construction reading named for its premise); check 9 reports a ring on its outer loop at every vertex as meeting it; `point_in_loop` takes the plane's `Frame`; `shell::encloses` renamed to the concentric-pair order it decides.
