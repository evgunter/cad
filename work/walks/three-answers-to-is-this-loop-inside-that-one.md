---
id: three-answers-to-is-this-loop-inside-that-one
kind: issue
title: three answers to "is this loop inside that one" with three boundary postures (shell::encloses, chord_join::rehome_rings, validate::ring_nesting), plus a fourth hand read of Surface::Plane's chart normal
status: open
opened: 2026-09-14
priority: P1
cost: D
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
   `boolean::contain::loop_shape` class** — `point_in_loop` for
   `Polygon`, `boolean::contain::disc_side` for `Disc` (made
   `pub(crate)` for it); `ArcParity` and `NoWalk` are silent. That
   widens the divergence this row tracks: `ring_nesting` and
   `boolean::contfp` now dispatch on the loop's shape, `rehome_rings`
   still does not
   (`work/reach/rehome-rings-reads-an-arc-bearing-run-through-the-polygon-walk`).

Two of the three agree on the instrument and disagree on the sample set
and on the boundary; the first agrees with neither and is not a
containment statement at all. None of the three is wrong at its own
call site today — what is missing is any statement of WHY the postures
differ, so a fourth caller has no way to choose and the obvious move
(reuse the nearest one) is the wrong one about half the time.

## The fourth copy: reading a plane's chart normal by hand

`rehome_rings` reaches its normal through `chord_join::face_plane_normal`,
which matches `Some(geom::Surface::Plane { normal, .. })` and refuses
typed otherwise. `validate::nesting_region` (named `nesting_normal` until
2026-09-24) does the same match inline, as do `validate`'s check-6 arm, `merge_faces`, `replace_face`,
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
