---
id: witness-and-door-searches-hand-roll-the-ladder
kind: issue
title: Witness and door searches hand-roll the ladder's rule beside geom_brep::ladder::climb, each with its own recording and ranking
status: open
opened: 2026-11-02
priority: P1
cost: M
---



Found by the sweep of `identity-data-reads-its-ladder-from-geom-brep`
(branch `onehome/identity-ladder`, 2026-11-02).

## What

`geom_brep::ladder::climb` now states the ladder's rule once: the first
reading that answers decides; a reading the band does not decide is
kept, and refuses only where none answers, ranked by
`ladder::Evidence` (a limit ahead of an in-band reading). It also owns
the recording: a kept reading's run is spliced superseded when a later
one answers. The identity ladders whose rungs are banded data now read
it. These searches spell the same rule by hand, each over door
readings or witnesses rather than banded data:

- `topo::ray_walk::walk` and `topo::stands::ladder` (the witness
  ladder): `Evidence`'s ranking, but no held recording, so a set-aside
  ray's or witness's escalation is recorded as one the verdict rests on.
- `topo::chord_join::first_decided`: a second generic engine (reversed
  polarity, first escalation kept, no ranking, no held recording). Its
  readings are computed in the iterator its callers hand it, so a
  `climb` port moves the reading into the reader.
- `geom_brep::chart_singularity` (the inside-edge loop and
  `holds`): the inside-edge loop ranks an in-band reading AHEAD of an
  unsettled carrier, the reverse of `Evidence`'s order if an unsettled
  carrier is a limit; check which is right.
- `topo::pcurves`' `probe_pick` loop (the measured-row stage): keeps the
  LAST deferred cause, and a later stage runs only where nothing was
  deferred, so an earlier stage's in-band pick masks a later stage.
- `topo::census`'s interiors loop (`Some(Some(m)) if m.overlap` answers)
  prefers an `Undecided` cause over any other: its own ranking.
- `geom_brep::curve_face::composite::cut` (the `SPLITS` `find_map`),
  `geom_brep::pair_section::gap` (the cell's branch loop),
  `geom_brep::ssi::refine` (the halving round), `topo::one_region`,
  `topo::boolean::fate` (the ring vertex loop), `topo::boolean::slit`
  (the piece loop), `topo::link` (the sector loop),
  `topo::side_reader` (the aimed rays), `topo::boolean::vtxfac` (the
  pierce pass: a definite refusal answers, escalations kept).
- `geom_brep::edge_nurbs::measured_ends`: the identity polarity over two
  ends, read whole, with a third "bare" outcome.

## Why not ported with the identity ladders

Each kept cause is a door's typed error with its own limit / in-band
split, some have a fatal arm, and porting moves the k-stats record: a
kept reading's escalation becomes superseded where a later one answers.
On `ray_walk::walk` that is every containment query's record, a
k-report change to look at on its own. Several rank differently from
`Evidence` (above); each needs the check of which order is right, and a
pin.

## Fix

Port each to `climb` (a fatal arm is an answer of `Err`), or say why it
is a different rule. Where a search's readings happen in an iterator,
move them into the reader so the held recording covers them.
