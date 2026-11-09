---
id: door-searches-left-on-the-hand-rolled-ladder
kind: issue
title: The Any ports' unreached record rows, left after four tranches onto geom_brep::ladder::climb
status: open
opened: 2026-11-03
priority: P1
cost: E
refs: [6495, 6598]
---

The remainder of `witness-and-door-searches-hand-roll-the-ladder`, after
four tranches onto `geom_brep::ladder::climb`: PR 6598
(`onecopy/ladder-climb-4`, `topo::boolean`'s pierce pass, split into
`vtxfac::read_vertex_on_face` and `vtxfac::mint_pierce`), PR 6454
(`onecopy/ladder-climb`), PR 6495 (`onecopy/ladder-climb-2`) and
`onecopy/ladder-climb-3`, which ported census's four interior-meeting
searches, `offset_axial`'s corner pairs, `ssi::refine::fit_minimum`,
`edge_nurbs::measured_ends` and `vtxfac::faces_holding_pierce`. Each site
below still spells the rule by hand, with what a port needs.

## Rerouted to their own rows

- `topo::boolean::join::seam_germ_side` and `topo::boolean::sectors::tangent_lump`'s
  bounds read every rung and refuse where two answers disagree, a shape
  `climb` does not hold:
  `work/onecopy/a-ladder-that-checks-its-answers-agree-has-no-home.md`,
  which ports both onto `ladder::climb_agreeing`. The debug check the
  `Any` ports' premises want went to the `Any` design fork's `[ev]` PR.
- `topo::link::Cone::reads_given`'s reference loop waits on its
  `Unsettled` variants being classed:
  `work/onecopy/cone-reads-given-reference-loop-waits-on-its-unsettled-classes.md`.

## Left

- **The `Any` record paths are unreached.** In the full review of PR 6495,
  no `Any` port kept a reading and then answered: over 2,090 rows (topo,
  geom-brep, sweep, step-import), with the ported ladders instrumented.
  So the superseded record that `Any` buys is exercised only by
  `ladder.rs`'s own rows and `ray_walk`'s. A row needing a kept reading
  ahead of a deciding one is owed for each port. The cheapest is
  `side_reader::cast_aims` with a `Pair` ahead of a deciding ray, and no
  fixture builds one today. These rows travel with the debug check on
  the `Any` ports' premises, in
  `work/onecopy/a-ladder-that-checks-its-answers-agree-has-no-home.md`.

- **The class beyond this list.** The loops that keep a first refusal
  and that no row named are listed, undispositioned, in
  `work/onecopy/kept-refusal-loops-the-ladder-row-never-listed.md`.

## Not ported, by design

- **`topo::pcurves`' stage chain** (the stated, fixed, then measured
  candidates of `probe_pick`, then the General image). `probe_pick`
  itself climbs (`first_within`). The chain is a lane choice (D3) whose
  last stage is an ANSWER, the General image, so no stage's kept reading
  refuses anything. A `climb` over the stages would change no outcome
  and no record (`First` keeps each stage's escalation live, which is
  what a serial read does). It is not a copy of the rule. It becomes one
  only if a stage's escalation should refuse.
- **`topo::boolean::conic_quadric::in_band_sweep`'s candidate loop** is a
  `#[test]` fuzz sweep (`no_pose_inside_the_band_is_answered`). Its
  `continue`s skip sampled poses the oracle does not judge.

## A reading still taken outside its rung

`chord_join::path_island_winding_closed_by` builds its `IslandReading`
(`IslandReading::closed_by`) before the ladder, and the chart arcs of
`path_ring_side` (`run_loop_arcs`) likewise. Their escalations end the
read before any rung is read, so nothing is superseded past them. This
is noted so a reader of the k-stats record does not look for them in
the climb. `census::arc_interiors_meet`'s cuts are the same shape. They
are read before its piece ladder, and kept into its `Evidence`
(`Evidence::keep`) but spliced live, so a piece that answers supersedes
only the pieces' own kept readings.

## Where a reading's rank lives

What a consumer EXEMPTS is the consumer's rule: tier 3 exempts a pole an
unsettled edge or an unread walk meets, and the import pass refuses
both. Whether a reading is a LIMIT is a property of where the reading is
minted, so it lives on each type, beside its variants, in one
classification, `ladder::Refusal` (`Result<Refusal, Aborts>` for a type
that can abort; `ladder`'s module docs):
- `Unread::refusal` and `census::InteriorsUnread::refusal` (`Refusal`);
- `PointInLoopError::refusal`, `PointInSolidError::refusal` and
  `ContainError::refusal` (`Result<Refusal, Aborts>`).

`census`'s fold of `MeetFault::Unsettled` into `InteriorsUnread::Unbuilt`
is a consumer-side rank of that kind, filed as
`work/splitdoor/census-interiors-fold-a-meetings-fault-into-unbuilt.md` (its 2026-11-05 section).
