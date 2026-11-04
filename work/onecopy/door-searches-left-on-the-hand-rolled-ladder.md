---
id: door-searches-left-on-the-hand-rolled-ladder
kind: issue
title: Door searches still hand-rolling geom_brep::ladder::climb after onecopy/ladder-climb: each site, and why it was not ported there
status: dispatched
opened: 2026-11-03
priority: P1
cost: M
branch: onecopy/ladder-climb-2
---

The remainder of `witness-and-door-searches-hand-roll-the-ladder`.
Branches `onecopy/ladder-climb` (PR 6454) and `onecopy/ladder-climb-2` (tranche 2: `composite::cut`, `chart_singularity::place_on_boundary` and `Sides::holds`, `one_region::read_cycle`, `fate::rehome`, `slit::holding`, `side_reader::cast_aims`, the gap cell's branches, `pcurve_cache`'s meridian frame) ported the cause-dropping searches
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

(`edge_nurbs::measured_ends` also ranks differently: below.)

- **`topo::census::shared_run`** (`Some(Some(m)) if m.overlap`; still
  hand-rolled after tranche 2): the census derives the cause from its own
  reading log (`escalated_since` over `read`), not from a kept reading,
  and its two causes feed two different findings. A port needs the
  census's log to be the climb's record, which is a census design
  question. Original note:
  answers) prefers an `Undecided` cause over any other: its own
  ranking. The census is a detector, so check whether its ranking is
  the detector's posture or a third order.
- **`topo::pcurves`' stage chain** (the stated, fixed, then measured
  candidates of `probe_pick`, then the General image). `probe_pick`
  itself climbs (`first_within`). The chain is a lane choice whose
  fallback is an ANSWER (the General image), so nothing refuses on what
  a stage kept, and a `climb` over the stages would change no outcome and
  no record (`First` keeps them live, as now). Left as written unless a
  stage's escalation should refuse.
- (the row's original note:) **`topo::pcurves`' `probe_pick` loop** (the measured-row stage)
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

## The rest, each with what a port needs

- **`geom_brep::ssi::refine::fit_minimum`** (the halving round). Its
  refusal is a census over every rung: `ShortBranchUncertified` needs
  every gap in band, and the exhaustion refusals carry per-kind counts.
  `Evidence` keeps only the ranked first of each kind, so a port needs a
  counting `E` or a tally beside the ladder. Answering also mutates
  `states`, and the ladder repeats per round.
- **`topo::link::Cone::reads_given`** (the reference loop). `Any`. Each
  rung is itself a conjunction over every other sector, and a refusal on
  the `Out` reach is dropped while one on the `Near` reach is kept. A
  port folds the inner sector read into one rung, maps an `Out`-reach
  refusal to `Passes` (or chains two climbs with `Evidence::then`), and
  splits `Unread::Unsettled` into a limit and an in-band reading.
- **`topo::boolean::vtxfac` pierce pass** (`boolean_reduce_declared_strategy`
  in `boolean/mod.rs`, and `vtxfac::faces_holding_pierce`). This is a
  collect-every-`Ok` batch with side effects (its readings mint), not a
  search: only its refusals are ladder-shaped. A port would use `Ok` →
  `Passes` with results stashed through `FnMut`, a definite refusal →
  `Rung::Aborts`, and `Escalated` → in band, ideally after the
  read-then-mint split `faces_holding_pierce` already makes.
- **`geom_brep::edge_nurbs::measured_ends`**. It is identity-shaped (a
  miss answers), always reads both ends, and has a third kept kind
  (`bare`: in band with its enclosure reaching zero) ranked BELOW in band,
  which defers to the caller's `stall`. `Evidence` has no such rank, so a
  port needs a third kept kind or a flag beside the ladder.
## The `Any` premise is prose nothing checks

`whole_body_side` (every off-plane point of an un-cut body reads one
side), `stands::ladder` (the complex crosses no probed surface) and the
chord-join side ladders (a ring does not cross its curve, assumed on
`Ellipse`, `Spiric` and NURBS edges) each declare `Answering::Any` on a
premise no code checks. If a premise fails, the first decided reading
wins silently, as it did before `climb`. A debug check that reads one
more decided rung and asserts it agrees would make the premise a fact.
It costs a reading per ladder, and no current row reaches a violation.
Filed here as part of this row, not separately: a check is worth adding
where a port's premise is weakest (the NURBS ring edges first).

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
