---
id: the-join-lever-names-a-pole-for-a-chart-class-reading
kind: issue
title: topo edge_join: JOIN_LEVER names a pole, apex or tangency for a chart-class reading, the join's at-rest and door renders hand-spell the undecided joint, and an underived chart image is classed undecided
status: open
opened: 2026-10-11
priority: P3
---


(ENCL implementer, from the sweep in
`work/encl/whole-escalation-renders-outside-the-too-close-family.md`,
PR 4756, and its review.)

## What

**A lever untrue at the site.** Two renders of an undecided join end
in `JOIN_LEVER` ("move the vertex clear of the pole, apex or tangency
it sits near") on both of `JoinReading`'s arms:
`topo::JoinRefusal`'s Display (its `undecided` closure,
`boolean/edge_join.rs`) and `ValidationError::JoinUndecidedAtRest`
(`validate.rs`, through `JoinUndecided::diag`). The lever is true of
`JoinReading::Regularity`, which covers a vertex's distance from a
chart singularity, the pair's wedge and the tangent arm's second
order. It is not true of
`JoinReading::ChartClass(IsoFamilyRefusal::Undecided)`, which
`geom_brep::chart_iso_family` raises where a cylinder carrier's radial
amplitude (`pcurve_chart_radial_moving`) is in the band: an edge
running nearly, but not exactly, along a ruling. No pole, apex or
tangency is near that vertex, and moving the vertex does not change
the edge's tilt. That reading is also a form selection (the mint's
structure tie-break), so `recourse.rs`'s `Unsized` classing makes it
`LeverOnly`, and no tolerance arm belongs on it. Both doors give it
`Indeterminate::ending`'s valued arm today.

**Hand-spelled joints.** Both doors write
`"{JOIN_SUBJECT} is undecided ({payload}). {ending}"` by hand: in
`edge_join.rs`'s `JoinRefusal` Display (its `undecided` closure) and
in `validate.rs`'s `JoinUndecidedAtRest` arm. The shared form is
`Indeterminate::undecided(JOIN_SUBJECT, ending)`
(`{subject} is undecided: {payload}. {ending}`). The Boolean's own
render, `JoinUndecided`'s Display, now goes through that form after
PR 4756: regularity under `ending(COINCIDENCE_RECOURSE)`, chart class
under `LeverOnly { lever: COINCIDENCE_RECOURSE }`.

**An underived chart image classed as undecided.**
`JoinReading::ChartClass(IsoFamilyRefusal::Image(e))` is a refusal of
the chart image's derivation (`PcurveCertifyError`), not a margin in
the band. It still sits under `JoinUndecided`, a refusal that says
"undecided":
- `JoinUndecided`'s Display (`BooleanError::JoinUndecided`) renders it
  "{JOIN_SUBJECT} is undecided: {e}", with whatever ending `e` carries.
  For `UnsupportedChart` that is none.
- The join door (`JoinRefusal`) calls the same reading a kernel or file
  defect (`KERNEL_OR_FILE_DEFECT_ENDING`).

One reading, two verdicts. `JoinUndecided::diag` already returns
`None` for it.

## Repair shape

- Give the chart-class arm a lever true of it at every door, ending
  `LeverOnly`.
- Render both doors through `Indeterminate::undecided`.
- Move `Image` out of `JoinReading` (or out of "undecided") so that
  every door gives it one verdict.
- Re-baseline, with literal text, `edge_join.rs`'s `undecided_text`
  tests and `validate.rs`'s `JoinUndecidedAtRest` fixtures.
