---
id: rehome-rings-reads-an-arc-bearing-run-through-the-polygon-walk
kind: issue
title: chord_join::rehome_rings places a bystander ring against the run through point_in_loop with no loop_shape dispatch: on an arc-bearing run (a planar cylinder cap cut by a straight chord) a ring in the lune reads Out and stays on the wrong face (unreproduced)
status: open
opened: 2026-09-24
priority: P1
cost: D
refs: [arc-aware-point-in-loop, three-answers-to-is-this-loop-inside-that-one]
---


**Unreproduced — a hypothesis from reading the code, with a concrete
case to try.** Found by ATREST-5's sweep (PR #3179) of every caller of
`splitting::point_in_loop` for the shape "a loop's region read off the
polygon walk with no `boolean::contain::loop_shape` dispatch".

**The site.** `chord_join::rehome_rings` (the `laringmv` re-homing
after a face-dividing join) tests each bystander ring's anchor vertex
(`ring_representative`) against the RUN — `newf`'s outer loop — with
`point_in_loop`. That walk's contract is the planar POLYGON through the
loop's vertices; on a loop bearing arcs it answers about a different
region. `OnBoundary` refuses typed (`SplitJoinError::RingHomingAmbiguous`),
so only the `Out` direction is silent: a ring the run encloses is left
on the old face.

**Which class is reachable.** A run always contains the chord the `mef`
just minted, so it is almost never `loop_shape`'s `Disc` class (every
edge an arc of one circle). On a planar face the chord is straight in
the `Planar` and `Split` lanes (`chord_spec` returns `None`); only the
boolean's `BoolPlanar` lane mints a conic arc there
(`bool_planar_chord_spec`). So the arcs a planar run carries come
mostly from the divided face's OWN boundary, and the reachable class is
`ArcParity` — arcs over three or more vertices, where an arc bowing
outward leaves region between the polygon and the boundary and a point
there reads `Out` when it is in. `NoWalk` (fewer than three vertices)
is reachable only for a run of one arc and one chord.

**The case to try** (from ATREST-5's review): an R = 2 planar cylinder
cap split by the straight chord x = 0. Each half's run is arcs plus the
chord over three vertices (`ArcParity`). A bore in the lune between an
arc and the polygon — hole centre (1.6, 0.8), radius 0.15 — has its
anchor outside the polygon through the run's vertices, reads `Out`,
and stays on the face it no longer lies in. What the row should
measure: the ring's owning face after the split, and whether tier 3
then refuses the result (check 9's nesting arm is silent on `ArcParity`
outer loops, so it may not).

**What closes it.** `loop_shape` dispatch the way `boolean::contfp`
does: `Disc` through `boolean::contain::disc_side` (crate-visible since
ATREST-5), `Polygon` through the walk, and `ArcParity`/`NoWalk` either
refused typed or answered by the general arc-aware walk
(`work/tang/arc-aware-point-in-loop`, #1076). This is also one of the
three loop-in-loop answers `work/walks/three-answers-to-is-this-loop-inside-that-one`
tracks.

**Priority**: P1 while unreproduced; a measured wrong ring placement
from a normal split or boolean is a live wrong answer and moves it to
P0.
