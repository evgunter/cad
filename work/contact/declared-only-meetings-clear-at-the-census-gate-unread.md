---
id: declared-only-meetings-clear-at-the-census-gate-unread
kind: issue
title: A pair whose only meetings are declared is probed but its records are not side-read, so an overlap they hide with no vertex strictly inside clears
status: dispatched
opened: 2026-09-26
priority: P0
cost: H
parent: CONTACT-13
---

Filed by CONTACT-5 and re-scoped by its fix pass. Arm 2 of
`sweep_cross_solid_backstop` (`crates/topo/src/census.rs`) now counts a
declared record naming one entity of each solid — v-on-f, v-v, or a
curve or patch face pair (`meets_declared`) — as a meeting, and a pair
that meets has every vertex probed against the other's material in
both orderings. That closes both demonstrated wrong clears:

- a prism standing on a wall with a point dipping 0.1 into it, its
  four resting corners declared v-on-f
  (`contact5_gate_and_beam::a_declared_rest_with_a_dipping_point_is_probed`);
- a seat declared as two patch records with a V keel hanging 0.5 into
  the block (`contact5_gate_and_beam::a_declared_seat_with_a_keel_is_probed`).

Both passed `validate_pseudomanifold` with `Ok(())` before the fix and
now refuse as `InstanceInterference`.

**What stays open.** A pair whose only meetings are declared is probed
whether or not it reaches, and `blocks` reads its records with
`records_on_their_word`: a declared v-on-f or v-v touch whose cones
decidedly cross refuses as `MixedTouch`. Two kinds of meeting are still
taken on the records' word:

- a declared touch the analysis cannot read, such as a curved corner;
- the vertex events a declared face pair backs by structural
  incidence (`Declared::vv_face_backed`, `vf_face_backed`,
  `ve_face_backed`, `ef_bound_backed`, `ee_bound_backed`).

An overlap those meetings hide with no vertex strictly inside therefore
clears. No such pose has been built. The half-overlap shape cannot be
declared free of findings with v-on-f and v-v records alone, because a
vertex on an edge has no record type.

Running `blocks` over declared-only pairs is not the repair. It
refuses ratified acceptance rows:

- `sweep` `m5_pr9_boss_union::a_touching_curved_assembly_validates_declared_and_refuses_undeclared`,
  the M9-2 declared curved boss, refuses as `TouchUnreadable`;
- nineteen declared planar seats refuse as `DeclaredFacePair`, among
  them `m9_c1_rest_face_rung::the_flush_seat_certifies_in_both_argument_orders`
  and `mate9_crossing_rung::the_declared_crossing_seat_certifies_both_ways`.

The repair is to read each face-pair-backed event through
`TouchSite::verdict` where the sweeps back it, and to decide what a
curved declared rest owes until the analysis reads curved cones.
Cost H.

## Note from CONTACT-7's designers (2026-09-28)

On the declared-only path, `blocks` with `records_on_their_word`
refuses a record only on `MixedTouch`. So a lenient NON-Crossing
reading (`Unanalysed` or in band), not just a lenient Rest, clears
there. Whether the Crossing verdict is complete, never missing a real
crossing, is therefore load-bearing on this path, and this row's repair
has to answer it.

Under CONTACT-7's piece reading, a Rest speaks only for the ball
around the touch point that its pieces cover. Coverage beyond that ball
rests on the vertex probe and on completeness (every meeting stands as
a finding). That coverage is exactly what this path takes on the
records' word.

## A built wrong clear (2026-09-29, a designer lane on `contact/land-7`, now main)

"No such pose has been built" is no longer true.

The pose:
- **The part:** an I-profile, `[(0,-1),(2,-1),(2,0),(1.5,0),(1.5,1),(2,1),(2,2),(0,2),(0,1),(0.5,1),(0.5,0),(0,0)]`, `prism_z(.., 0, 2)`.
- **The slab:** un-holed, `prism_z([(-5,0),(5,0),(5,1),(-5,1)], -1, 3)`.
- The waist `x∈[0.5,1.5], y∈[0,1]` passes through the slab. The head is seated on the slab's `y=1` face and the tail on `y=0`.
- No vertex is strictly inside either solid.

Results:
- **Undeclared:** it refuses, with 32 `UndeclaredContact` and a decided crossing at an edge-in-face site.
- **Declared as four `PatchContact`s** (the seat faces and the tail tops; each verifies): `validate_pseudomanifold` returns `Ok(())`.
- **Declared as sixteen `VfContact`s:** `Ok(())`. The vertex stars are saddles, so they read `Unanalysed`, and the declared-only path refuses only on `MixedTouch`.

This confirms CONTACT-7's designers' note: the Crossing verdict is incomplete, so "refuse only a decided crossing" is unsound even for vertex records. The repair lane commits the three poses as rows.

A second designer lane built a curved wrong clear on the same branch.
- **The pose:** a wall, `brick((0,4),(0,1),(0,4))`, and a horizontal
  log, `extruded(sketch_at(1.0), [bulge_loop([(1.6,1),(2.4,1),(2,1.8)])
  with bulge(·,·,(2,1.3)) on each arc], 2.0)`. The log's lower arc lies
  0.2 m inside the wall.
- **Declared** with four v-on-f records at (1.6|2.4, 1, 1|3), it returns
  `Ok(())`. A true rest (the same log with a flat bottom) also returns
  `Ok(())`, and the records cannot tell the two apart.
- **Arm 1 skips the log's curved wall** against the wall face because a
  v-on-f record names that face, so one point's record clears a whole
  curved face.

The same lane measured the nineteen planar seats: every event a face
pair backs, read strictly through `TouchSite::verdict`, reads Rest
(topo, sweep and editor-core unchanged).

The design fork is on its `[ev]` PR.

## Ev's ruling (PR 3422, 2026-09-29)

"For 1, nice find! Sounds good. For 2, the recommendation makes
sense."

- **Decision 1:** a declaration licenses a coincidence, never a side.
  Every meeting is read by the one touch analysis, and only a Rest
  clears.
- **Decision 2:** C1. A curved face in a touch's star is read through
  its certified reach box, which can certify a Rest or refuse but never
  decides a crossing. This keeps the M9-2 boss.
- **Later:** reading curved cones would be tighter than the box, so it
  is filed as a row:
  `the-touch-analysis-reads-curved-cones-tighter-than-the-reach-box`.
