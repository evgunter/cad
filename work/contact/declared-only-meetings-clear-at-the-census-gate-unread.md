---
id: declared-only-meetings-clear-at-the-census-gate-unread
kind: issue
title: A pair whose only meetings are declared (v-on-f, v-v or face-pair records, no standing finding) is cleared at the census's extent gate without the touch analysis reading them
status: open
opened: 2026-09-26
priority: P1
cost: H
---

Filed by CONTACT-5. Arm 2 of `sweep_cross_solid_backstop`
(`crates/topo/src/census.rs`) clears a pair at the extent gate only
when no finding stands between the two solids (the `meets` closure);
a pair with one clears only when `blocks` reads every meeting as a
rest. A declared event leaves no finding, so a pair whose ONLY
meetings are declared is still cleared at the gate on the records'
word, with no side reading.

Two readings were measured and both refused ratified acceptance rows:

- Counting declared v-on-f and v-v records as meetings refused
  `sweep`'s `m5_pr9_boss_union::a_touching_curved_assembly_validates_declared_and_refuses_undeclared`
  (the M9-2 acceptance: a curved boss declared resting on a plate) as
  `Undecided::TouchUnreadable` — the vertex cone at an arc joint is
  curved, and the analysis cannot read it.
- Counting declared FACE-pair records (`Declared::faces`) refused
  every declared planar seat in the topo suite as
  `Undecided::DeclaredFacePair` — nineteen rows, among them
  `m9_c1_rest_face_rung::the_flush_seat_certifies_in_both_argument_orders`,
  `mate9_crossing_rung::the_declared_crossing_seat_certifies_both_ways`
  and `mate4a_ef_bound_rung::the_declared_straddle_seat_certifies` —
  because `blocks` has no reading for such a record except that
  refusal. The record backs the vertex events subordinate to it
  (`Declared::vv_face_backed`, `vf_face_backed`, `ve_face_backed`,
  `ef_bound_backed`, `ee_bound_backed`) by structural incidence, with
  no side, anywhere on the incident entities
  (`m9_c1_rest_face_rung::the_rung_backs_an_event_outside_the_declared_pairs_overlap_region`).

What bounds the gap: v-on-f and v-v records name vertices only, and a
vertex on an edge has no record type, so the half-overlapping cubes
cannot be declared finding-free (measured: every declaration round
leaves the `VertexOnEdge` and `EdgeEdgeOverlap` findings standing, and
the pair refuses as a mixed touch). A backed crossing is read for its
side (`ee_cross_backed`, opposite sides only), and the confirm pass
contradicts an aligned-sense planar face pair. No wrong clear has been
shown. The repair is to read each declared or face-pair-backed event
through the same `TouchSite::verdict` wherever the analysis can read
it, and to decide what a curved declared rest owes until the analysis
reads curved cones. Cost H.
