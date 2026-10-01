---
id: rehome-rings-reads-an-arc-bearing-run-through-the-polygon-walk
kind: issue
title: chord_join::rehome_rings places a bystander ring against the run through point_in_loop with no loop_shape dispatch: on an arc-bearing run (a planar cylinder cap cut by a straight chord) a ring in the lune reads Out and stays on the wrong face, and the divided shell refuses as torn
status: dispatched
opened: 2026-09-24
priority: P0
cost: M
refs: [arc-aware-point-in-loop, three-answers-to-is-this-loop-inside-that-one]
branch: cleave/rehome-rings
---


**Reproduced 2026-10-01 (CLEAVE): a refusal at rest where a correct
answer exists, so P0.** The named case, built through the public doors
(a holed `Profile`, `extrude`, then `split` at `x = 0`), refuses
`SplitError::Finish(TornComponent)` in every orientation — two- and
four-arc discs, bore at `(±1.6, 0.8)`, plane normal `±x` — and so does
every `intersect`/`subtract` of the bored disc with a slab `x ≷ 0`
(`BooleanError::TornComponent`). The bore inside the polygon
(`(0.8, 0.4)`, `(−0.8, −0.4)`) splits and validates. Tier 3 never sees
the ring on the wrong half: the shell it is left on carries both halves'
section faces, and the finish refuses it first. With the run read through
`point_in_carrier_loop` every row splits, passes tier 3 and measures the
half-disc's volume less the bore on the half that holds it
(`crates/sweep/tests/rehome_rings_lune.rs`).

**Filed unreproduced — a hypothesis from reading the code, with a concrete
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
then refuses the result. (Check 9's nesting arm was silent on
`ArcParity` outer loops when this was filed; since 2026-09-25,
ATREST-12, it places rings on every planar outer loop with
`splitting::containment::point_in_carrier_loop`, so a ring left on the
wrong half should now be refused at rest as `RingOutsideOuter`.)

**What closes it.** The general arc-aware walk
(`work/tang/arc-aware-point-in-loop`, #1076) exists now as
`splitting::containment::point_in_carrier_loop` (`pub(crate)`, ATREST-9);
check 9's nesting arm reads every class through it with no shape
dispatch (ATREST-12), and `rehome_rings` can do the same. This is also one of the
three loop-in-loop answers `work/walks/three-answers-to-is-this-loop-inside-that-one`
tracks.

**Priority**: P1 while unreproduced; a measured wrong ring placement
from a normal split or boolean is a live wrong answer and moves it to
P0.
