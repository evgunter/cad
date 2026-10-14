---
id: closed-in-face-section-loop-has-one-site
kind: issue
title: A closed operand conic lying in the partner's face makes a single-site closed section loop, which nothing in the join or the REST lane represents
status: closed
opened: 2026-10-02
closed: 2026-10-14
priority: P1
cost: H
branch: paths/in-face-section-self-loop
pr: 5159
refs: [3790]
---


Found by JOIN's in-face measurement (2026-10-02, probe branch
`join/inface-probe`).

## What

The tube of `an-edge-lying-in-a-cutter-face-past-its-end-wall-leaves-loose-ends-unpaired`'s
"closed in-face conic" measurement (`crates/sweep/tests/germ_coplanar_conic.rs`
`every_op_refuses_or_answers_its_closed_form`) produces two section
loops with ONE site each:

- the lone vertex `(1, 0, 0)` of the circle `ρ = 1` that lies in B's
  top face (A's face 5 on both slots of the record);
- `(0.5, 0, 0)` on the inner wall's transverse section circle (A's face
  3 on both slots).

The labels agree, but each germ's only partner is the other slot of
its own record. `find_match` skips `entry == cand`, and the REST lane
skips germs of the same pair, so both remain loose:
`Join(UnpairedLooseEnds { count: 4 })` for ∪, ∖ and ∩. The inner circle
is not an in-face edge at all, so this is not the flank class.

## What the taker owes

A representation of a closed section loop with one site, in the join
(and in the REST lane where one is declared), or a typed refusal
naming it before the loose ends are counted. Weigh it with the
designers' answer on the flank fork, which may cover it.

## Built (JOIN-1, branch `join/1-germ-locus`)

The typed-refusal half: both loops now refuse
`Join(SingleSiteSectionLoop { count: 2 })` under ∪, ∖ and ∩, before
the loose ends are counted; pinned by
`crates/sweep/tests/germ_coplanar_conic.rs`
`a_closed_section_loop_with_one_site_refuses_typed`. The self-loop
arm (a record matching itself) is not built.

## Also seen by CLEAVE (`cleave/tube-across-axis`, 2026-10-06)

A box `(−1.5, 1.5) × (0, 1) × (−1.5, 1.5)` minus the tube revolved a full
turn about `y` from `(0.3, −0.5)–(0.5, −0.5)–(0.5, 1.5)–(0.3, 1.5)`
refuses `Join(SingleSiteSectionLoop { count: 4 })`. The box's two faces
cut each tube wall across its one seam, so every section circle has one
site. The same box minus a revolved solid rod builds, because the rod's
wall is two faces with two seams.

The split's version of this, a one-site loop on a wall of one face, is
built on that branch. Its join already pairs a face's two halves at the
one site into a self-loop chord, and `chord_join::chord_spec` now gives
that chord the whole section conic instead of the scaffolding circle.
That may be the shape of the boolean's self-loop arm.

## Also seen by PATHS unit 3 (`one-segment-loop-through-builders`, 2026-10-06)

A one-segment circle (D1's full turn) extrudes to a cylinder of ONE
wall, its seam strut on the meridian through the profile vertex. Every
cap-parallel plane cuts that wall in a circle crossing the one seam
once. The cylinder r = 1, z ∈ [0, 2], minus or intersected with the slab
z ∈ [0.5, 1] refuses `Join(SingleSiteSectionLoop { count: 2 })`; the
same cylinder cut ALONG its wall (x ≥ 0 or y ≥ 0 removed, a bar united
through the seam) builds, tier 3 and closed-form volume. Pinned by
`crates/sweep/tests/one_segment_loop.rs`
`a_boolean_on_an_extruded_seam_wall_builds_along_it_and_refuses_across_it`.

This becomes common when PATHS unit 4 (`circle-lowers-to-one-segment`)
lowers `circle` to one segment: a pocket floor or a slab through any
circular boss is this cut, and today's two-arc cylinder (two seams, two
sites per section circle) builds it.

## Released from the D10 hold (2026-10-08)

Nothing D10 changes gates this row, so it is open: undeclared fixture (germ_coplanar_conic.rs); the missing self-loop arm is the join's topology, which D10 keeps (only the REST-lane half retires at stage 4). (INTENT's re-homing of the parked rows, `work/intent/log.md`.)

## The original witness is transverse (branch `join/wrap-edge-section-loop`, 2026-10-08)

At `872b33cc` the "strutted tube" of `germ_coplanar_conic.rs` revolves
to ONE outer wall face: its profile vertex at `(1, 0)` leaves no circle
edge at `ρ = 1`, `y = 0`. Both one-site loops of the fixture are
therefore transverse wrap-edge crossings (the outer wall's seam at
`(1, 0, 0)`, the inner wall's at `(0.5, 0, 0)`), and the boolean's
wrap-edge arm (`a-plane-across-a-one-face-wall-meets-its-wrap-edge-once`)
builds all three ops to their closed forms. The fixture is renamed
"the box top across both tube walls" and the refusal row
`a_closed_section_loop_with_one_site_refuses_typed` is deleted.

The class this row names, a conic lying in the partner's face, is
reached instead by the PR 4345 review's `F4d` fixture. It is pinned as
`crates/sweep/tests/a_plane_across_a_one_face_wall.rs`
`a_circle_edge_in_the_partners_face_refuses`:
- the fixture is a bored tube whose outer wall turns from a cylinder to
  a sphere at the circle `y = 0`, inside the box face `y = 0`;
- every op in both orders refuses `SingleSiteSectionLoop { count: 1 }`:
  the in-face circle refuses, and the inner wall's transverse crossing
  builds.

The arm takes a one-site record only where the site is a wrap edge of
one operand's face and a pierce of the other's planar face. The
in-face circle edge sits at its own site, so it keeps the refusal. The declared-REST lane, reached only when the
join refuses, reads `section_segments` alone and does not see the arm's
segments: a declared op whose join refuses for another reason with a
one-site loop on it falls back to that join refusal, as before.

## Also seen by GERM's U-T3 (`germ/plane-torus-chord-circles`, 2026-10-12)

With the plane × torus chord in, a torus whose equator `y = 0` is an
edge reaches this refusal against the half-space `y ≥ 0`: the one-face
torus of `germ_plane_torus_frame::one_face_torus` with its vertex on an
equator (profile phase 0 or π, so its wrap parallel is that equator),
and the torus seamed on `y = 0` (`seamed_torus`, both equators edges).
Every op in both orders refuses `SingleSiteSectionLoop` (`count: 1` on
the one-face torus, `count: 2` on the seamed one). The same poses at
`y ≥ 0.2`, `y ≥ −0.3` and the one-face torus with its vertex off the
equator build to their closed forms. Pinned by
`crates/sweep/tests/germ_plane_torus_chord.rs`
`a_section_circle_along_an_equator_edge_refuses_typed`. This is the
in-face circle class F4d names, on a torus: the equator edge lies in
the partner's face and sits at its own site.

## Measured by PATHS unit 4 (`paths/circle-lowers-to-one-segment`, 2026-10-14)

With `circle` lowered to one segment, the in-face class is the
ordinary boss on a plate. A `profile::circle` cylinder r = 0.3 standing
on the top face of the brick `[−1, 1]² × [−0.5, 0]` refuses
`plate ∪ boss` with `Join(BooleanJoin(SingleSiteSectionLoop { count: 1 }))`;
the same boss at `circle_split` n = 2 or 3 builds, and a through hole,
a blind hole and a sunk boss build at every n. In the workspace suite
the class turns these rows from building to refusing:
`pi_seam_and_kiss_through_the_boolean` (`a_cap_abutting_on_the_rim…`,
`a_puck_and_its_rounding_ring…`, `a_rim_inside_the_partners_face…`),
`a_tube_ending_on_a_ball::a_rim_inside_one_ball_face_holes_it`,
`cylinder_sphere_frame::a_ball_across_a_cylinder_wall…`,
`verbs_cylcyl_probe` (coaxial equal-radius union),
`verbs_cylsph_opening` (coaxial union), editor-core's
`peg_in_bore_union` and `intent_s5_b_interference::a_pin_bottoming…`,
and the tour's lily (`foot ∪ corm` at the socket). Unit 4 is held on
this row.

## Outcome

Built on `paths/in-face-section-self-loop` (PATHS took the row: it
blocked unit 4). The in-face loop is the transverse arm's sibling in
one owner, `boolean/join.rs` `one_site_segments` (renamed from
`wrap_site_segments`): a one-site record is one segment, its two ends
the record's two slots, where `closes_at_its_site` reads the site on
both operands (`site_cell`) as

- a wrap edge against a pierce of a planar face (the transverse arm,
  unchanged);
- a closed edge (both ends the site) against a pierce of a planar or
  spherical face, or against another closed edge through the site
  (`SegmentLane::AlongEdge`).

The chords (`chord_join.rs`):

- **the closed edge's side** copies the edge whole in every lane
  (`closed_edge_spec`, on the plan's closed edge, which `closed_at`
  reads once in `JoinPlan::of`). A self-loop's direction is free in
  topology, so it is read from the chord's `mef` site
  (`closed_chord_runs_forward`): the loop keeping the edge's half is the
  zero-area sliver, whose chord half runs against it. The copy is
  stated in the chart of the face it divides, because a whole-turn
  scaffold circle reads as a null edge's placeholder.
- **a ring holding the null edge and the closed edge** (a puck's floor
  rim in the ring's annulus) goes through `join_lone_ring` as a pierce
  ring does: promoted, joined, and the one face its chord walls off
  goes back as the old face's ring (`lone_ring` returns `LoneRing`).
- **a pierce of a sphere face** sides the two whole conics with
  `RimSide::Sphere`: a plane cuts the sphere in the conic alone, into
  two caps, and the old face's outer loop lies in one of them; each
  conic loop's region is the cap `n × t` points into, and the hole is
  the one whose cap holds the outer loop. `RimSide::of` is the one
  table of the faces a pierce is sided on, read by the join's
  `closes_at_its_site` and by `join_lone_ring` alike.

Witnesses: `crates/sweep/tests/a_closed_conic_in_the_partners_face.rs`
(a boss on and under a plate at four azimuths, beside a ring and over a
hole; coaxial equal cylinders; a rim inside a ball face; a puck and its
rounding ring), every op in both orders where the op builds on the
two-arc form. F4d (`a_plane_across_a_one_face_wall.rs`
`a_circle_edge_in_the_partners_face_builds`) and the equator tori
(`germ_plane_torus_chord.rs` `two_loops_on_one_face_pair_build_their_true_bodies`)
now build to their closed forms; their refusal rows are gone.

A closed edge lying in a cone, a cylinder or a torus face (whose plane
can cut the face in more than the conic) has no `RimSide` arm, and the
join would refuse it `SingleSiteSectionLoop`; no pose reaches the join
today, because the crossing layer refuses first
(`CurvedPierceUnsupported`, or `CurvedBooleanUnsupported` for a disc on
a torus's parallel). Filed as
`a-closed-edge-lying-in-a-cone-cylinder-or-torus-face-has-no-join-arm`,
and pinned at the crossing layer by the witness file. A plane tangent
at a wrap edge's end vertex still refuses `SingleSiteSectionLoop`. D10:
no declared contact is read or recorded by the arm, and the REST lane's
`section_segments` is unchanged.

The dual review found a wrong body next to this arm but not in it: a
boss over a hole tangent inside its rim, both vertices at the tangent
point, unites as two shells, on `main` as here
(`work/cleave/a-boss-over-a-hole-tangent-inside-its-rim-at-its-vertex-unites-as-two-shells.md`).
The germ-face blind spot the PR disclosed is
`a-closed-edges-germ-read-on-a-planar-face-would-take-the-placeholder-chord`.

