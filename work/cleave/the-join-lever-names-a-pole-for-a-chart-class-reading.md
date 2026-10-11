---
id: the-join-lever-names-a-pole-for-a-chart-class-reading
kind: issue
title: topo edge_join: JOIN_LEVER names a pole, apex or tangency for a chart-class reading, and the join's at-rest and door renders hand-spell the undecided joint
status: open
opened: 2026-10-11
---


(ENCL implementer, from the sweep in
`work/encl/whole-escalation-renders-outside-the-too-close-family.md`.)

## What

Two renders of an undecided join end in `JOIN_LEVER` ("move the vertex
clear of the pole, apex or tangency it sits near") for both of
`JoinReading`'s arms: `topo::JoinRefusal`'s Display (the
`undecided` closure, `boolean/edge_join.rs`) and
`ValidationError::JoinUndecidedAtRest` (`validate.rs`, through
`JoinUndecided::diag`). The lever is true of `JoinReading::Regularity`
(a vertex's distance from a chart singularity, the pair's wedge, the
tangent arm's second order). It is not true of
`JoinReading::ChartClass(IsoFamilyRefusal::Undecided)`, which
`geom_brep::chart_iso_family` raises where a cylinder carrier's
radial amplitude (`pcurve_chart_radial_moving`) is in the band: an edge
running nearly, but not exactly, along a ruling. No pole, apex or
tangency is near that vertex, and moving the vertex does not move the
edge's tilt. The geometry lever there is the edge's direction against
the ruling.

Both renders also spell the undecided shape by hand,
`"{JOIN_SUBJECT} is undecided ({payload}). {ending}"`, rather than
through `Indeterminate::undecided(JOIN_SUBJECT, ending)`
(`{subject} is undecided: {payload}. {ending}`), the shape
`too-close-to-call-remainder`'s rule gives a whole render.

The Boolean's own render of the same refusal (`JoinUndecided`'s
Display, under `BooleanError::JoinUndecided`) ends in
`COINCIDENCE_RECOURSE` ("move the geometry"), true of both arms.

## Repair shape

Give the chart-class arm a lever of its own (or one lever true of both
arms) and render both doors through `Indeterminate::undecided`,
re-baselining `edge_join.rs`'s `undecided_text` test and
`validate.rs`'s `JoinUndecidedAtRest` fixtures with literal text.
