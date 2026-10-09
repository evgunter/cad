---
id: closed-in-face-section-loop-has-one-site
kind: issue
title: A closed operand conic lying in the partner's face makes a single-site closed section loop, which nothing in the join or the REST lane represents
status: open
opened: 2026-10-02
priority: P1
cost: H
refs: [an-edge-lying-in-a-cutter-face-past-its-end-wall-leaves-loose-ends-unpaired]
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
