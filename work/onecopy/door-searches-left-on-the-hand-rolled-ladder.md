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

(`pcurve_cache`'s meridian frame, `side_reader::cast_aims` and
`edge_nurbs::measured_ends` also rank or default differently: below.)

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

## The rest, each with what a port needs

- **`geom_brep::curve_face::composite::cut`** (the `SPLITS` `find_map`).
  A straight port: `Answering::First` (the share that answers is where
  the piece is cut). `Reading::Past` answers, `Near(Some)` is in band,
  and `Near(None)` or a share outside the piece passes. Map
  `Climb::Passed` to `Unsettled::Tangency` and `Undecided` to
  `Escalated` (`touch_or_tangency`). The floor refusal ahead of the loop
  stays outside. It was not reached in the first tranche only because
  the tranche was the cause-dropping searches.
- **`geom_brep::pair_section::gap::critical_gap`** (the cell's branch
  loop). Only the inner loop over one or two branches is ladder-shaped
  (First, `Open` in band). The outer search is a worklist: a kept cell is
  bisected, not refused, and there are budget and floor refusals. `climb`
  has no "a kept rung spawns more rungs" mode, so the worklist stays
  hand-written.
- **`geom_brep::ssi::refine::fit_minimum`** (the halving round). Its
  refusal is a census over every rung: `ShortBranchUncertified` needs
  every gap in band, and the exhaustion refusals carry per-kind counts.
  `Evidence` keeps only the ranked first of each kind, so a port needs a
  counting `E` or a tally beside the ladder. Answering also mutates
  `states`, and the ladder repeats per round.
- **`topo::one_region::read_cycle`** (with `door_side`). A small port:
  `Any`, with the in-band diag carrying its vertex through
  `Rung::map_kept`. `door_side` deliberately turns its limits
  (`RayExhausted`, `EdgeCarrierUnsupported`, `CorruptFace`) into passes
  (exempt), and a port must keep them as `Passes`, not `Blocked`, or
  `Read::Exempt` becomes a refusal.
- **`topo::boolean::fate::rehome`** (the ring vertex loop). `Any`.
  `ContainError::Escalated` is in band and every other error passes.
  Note that limits pass silently today. `Passed` maps to
  `Limit::RingUnplaced`, and a corrupt point lookup aborts. The loop over
  rings around it is a batch, not a ladder.
- **`topo::boolean::slit::holding`** (the piece loop). `Any`. It keeps
  the first of `Unread` (limit-like) and an escalation, unranked. Under
  `climb` a later `Unread` would outrank an earlier escalation, which
  changes the named refusal. Decide whether `Unread` is a limit first.
  `unplaced`'s `crossed_only` post-filter, which can turn the refusal into
  "not in", stays after the ladder.
- **`topo::link::Cone::reads_given`** (the reference loop). `Any`. Each
  rung is itself a conjunction over every other sector, and a refusal on
  the `Out` reach is dropped while one on the `Near` reach is kept. A
  port folds the inner sector read into one rung, maps an `Out`-reach
  refusal to `Passes` (or chains two climbs with `Evidence::then`), and
  splits `Unread::Unsettled` into a limit and an in-band reading.
- **`topo::side_reader::cast_aims`** (the aimed rays). `Any` (the family
  docs say so). Its ranking is the reverse of `Evidence`'s, and that is
  deliberate: an in-band `Pair` refuses, while a set-aside ray,
  including `SetAsideWhy::Limit`, falls through to the volume reader as
  `Rays::Unread`. A port maps `SetAside` to `Passes` with a side list. A
  limit mapped to `Blocked` would outrank the pair and refuse where the
  volume should answer.
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
- **`geom_brep::pcurve_cache`'s sphere meridian frame**
  (`toward(u_ref).or_else(|| toward(cv)).unwrap_or(false)`). A two-rung
  `First` ladder whose in-band reading is dropped like a Zero, defaulting
  to the chart's side (a D9 tie-break) and never refusing. A port is
  `climb(First, [u_ref, cv], …)` with `Passed` and `Undecided` both mapped
  to `false`. Someone has to decide whether recording the swallowed
  in-band reading live is wanted: it moves the record at every sphere
  chart frame.

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
