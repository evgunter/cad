---
id: declared-only-meetings-clear-at-the-census-gate-unread
kind: issue
title: A pair whose only meetings are declared is probed but its records are not side-read, so an overlap they hide with no vertex strictly inside clears
status: open
opened: 2026-09-26
priority: P0
cost: H
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
