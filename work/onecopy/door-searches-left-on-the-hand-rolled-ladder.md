---
id: door-searches-left-on-the-hand-rolled-ladder
kind: issue
title: "Door searches still hand-rolling geom_brep::ladder::climb after onecopy/ladder-climb: each site, and why it was not ported there"
status: open
opened: 2026-11-03
priority: P1
cost: M
---

The remainder of `witness-and-door-searches-hand-roll-the-ladder`.
Branch `onecopy/ladder-climb` ported the cause-dropping searches
(`splitting::finish::whole_body_side`, `geom_brep::half_angle`'s pole
candidates), the witness ladder (`topo::ray_walk::walk`,
`topo::stands::ladder`) and `topo::chord_join::first_decided` (retired;
its three callers read `climb`) onto `climb`. It also added the rung
helpers a door's own shapes need (`Rung::map_kept` for a refusal that
names an entity, `Rung::into_answer` for a reading gated on an earlier
one, `Rung::decide_magnitude`), and the gate
`scripts/gates/escalation-discards.sh`. Each site below still spells
the rule by hand. Port each to `climb`, declaring its `Answering`, or
say why its rule is different; where it ranks differently from
`ladder::Evidence`, decide the order from the semantics and pin it.

## Ranked differently from `Evidence`: settle the order first

- **`geom_brep::chart_singularity::place_on_boundary`** (the
  inside-edge loop) ranks an in-band edge (`Near::InBand`) ahead of an
  unsettled sampled carrier (`Near::Unsettled`), the reverse of
  `Evidence`'s order if `Unsettled` is a limit. It probably should not
  be reversed: tier 3 (`validate.rs`, `Undecided::of`) and step-import
  (`normalize/singularity.rs`, `foreign_carrier`) treat
  `Unread::Carrier` as EXEMPT, not as a refusal, so ranking it first
  would let a point with an edge in band pass validation unreported.
  The rule there is "a reportable reading ahead of an exempt one", so
  `Unsettled` is not `Evidence`'s limit. Port it with the exempt
  reading ranked last, say so, and pin a two-edge case (one edge in
  band, one unsettled, either order) so that `Err(Unread::Boundary)`
  holds. `Sides::holds` (the loop-side ladder) is the same file's second
  site.
- **`topo::census`'s interiors loop** (`Some(Some(m)) if m.overlap`
  answers) prefers an `Undecided` cause over any other: its own
  ranking. The census is a detector, so check whether its ranking is
  the detector's posture or a third order.
- **`topo::pcurves`' `probe_pick` loop** (the measured-row stage)
  keeps the LAST deferred cause, and a later stage runs only where
  nothing was deferred, so an earlier stage's in-band pick masks a
  later stage's decision. Both behaviours differ from `climb`; the
  second changes outcomes, not only the record.

## The rule plus an agreement check

- **`topo::boolean::join::seam_germ_side`** reads EVERY chord and
  refuses where two decided chords disagree; a region normal refused
  in band (`Err(_) => continue`, registered in
  `escalation-discards.counts`) and an in-band germ are dropped, and
  "no chord decides" is `Ok(None)`, which leaves the loop to the
  witness ladder's evidence. Porting it means returning its kept
  causes into that `Tally`, and they are `NormalAtError` and
  `Indeterminate` where the tally holds `PointInSolidError`.
- **`topo::boolean::sectors`' bounds loop** reads every bound, keeps the
  first escalation, and refuses where two decided bounds disagree.

## Ported by the same pattern, not reached in that PR

`geom_brep::curve_face::composite::cut` (the `SPLITS` `find_map`),
`geom_brep::pair_section::gap` (the cell's branch loop),
`geom_brep::ssi::refine` (the halving round), `topo::one_region`,
`topo::boolean::fate` (the ring vertex loop), `topo::boolean::slit` (the
piece loop), `topo::link` (the sector loop), `topo::side_reader` (the
aimed rays), `topo::boolean::vtxfac` (the pierce pass: a definite
refusal answers, escalations kept), and `geom_brep::edge_nurbs::measured_ends`
(the identity polarity over two ends, read whole, with a third "bare"
outcome).

## Not a site

`topo::boolean::conic_quadric::in_band_sweep`'s candidate loop is a
`#[test]` fuzz sweep (`no_pose_inside_the_band_is_answered`): its
`continue`s skip sampled poses the oracle does not judge. It is not a
door search.

## A reading still taken outside its rung

`chord_join::path_island_winding_closed_by` builds its `IslandReading`
(`IslandReading::closed_by`) before the ladder, and the chart arcs of
`path_ring_side` (`run_loop_arcs`) likewise. Their escalations end the
read before any rung is read, so nothing is superseded past them; noted
so a reader of the k-stats record does not look for them in the climb.
