---
id: a-spline-carrier-has-no-run-back-at-the-one-reversal
kind: issue
title: Curve3::reversed answers None for a spline, so each door that runs a spline edge back spells its own reversal or refuses: the chord lane reflects the net itself, a fate cut and two blend sites refuse
status: open
opened: 2026-10-28
priority: P2
cost: E
---


Found by SECT step 1b (`sect/cylinder-sphere-chord`, 2026-10-28), in
its sweep for doors that run an edge back.

## What

`geom::Curve3::reversed` (`crates/geom/src/curves.rs`) is the one
statement of a carrier run back, and answers `None` for a NURBS ("whose
knot vector a reversal would mirror"). The reflection exists and is
exact: `geom_brep::reversed_column` negates the knots
(`Knots::negated`, which does not round) and reverses the control
points and weights, so on `[−b, −a]` its point at `t` is the net's at
`−t`. Spline edges between two faces now exist (the cylinder × sphere
chord lane cuts them from the section's fitted loop), so the doors
that run an edge back meet them:

- `topo::chord_join::run_back`: the chord lane spells the spline arm
  itself (`reversed_column`), so `SegmentCurve::running_from` can mint
  a segment's second chord;
- `topo::boolean::fate::along_spec`: a fate cut's kept edge run back
  refuses `ClassificationInvariant` on a spline;
- `sweep::blend::surgery` (the `curve.reversed()` read near line 5512)
  and `sweep::blend::open::end_face` (`placed.reversed()`): refuse
  typed on a spline.

## Done when

`Curve3::reversed` answers a spline's reflection through 0, its doc
updated, `chord_join::run_back` retired onto it, and each `None`
reader above re-read (some may have relied on the refusal).
