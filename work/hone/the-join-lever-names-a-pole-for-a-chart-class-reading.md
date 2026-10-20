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

`PcurveCertifyError::ending(reading)` (`pcurve_cache.rs`) gives the
ending for the arms that are a decision's refused arm: `Escalated`,
`ResidualExceeded`, `IntervalNotForward`, `AzimuthPeriodExceeded`,
`TubePeriodExceeded` and `FittedEscalated`. Neither the Boolean's
`Image` arm nor `JoinRefusal`'s calls it. `chart_iso_family` reaches
`derive_harmonic`, which reaches `derive_class`, and a read of
`derive_class` shows `Escalated` is reachable through that path:
- `incidence::class_escalated` (`PcurveCheck::ChartClass`, PCERT E1's
  rename of `winding_escalated`), where a class reading is in band on a
  carrier the incidence test does not show off the chart;
- `incidence_escalated`, through `chart_incidence`, `incidence::incident`
  (which every general-lane arm runs since PCERT E1),
  `torus_oblique_circle` and `cone_conic_incidence`.

So an incidence or winding margin in the band arrives at the join as
`Image(Escalated)`. Today it renders as "undecided" with the bare
`PcurveCertifyError` text at the Boolean, and as a kernel or file
defect at the join door, while it is an undecided margin with an
ending of its own. `derive_class`'s body was not seen to raise
`ResidualExceeded` or the period arms: no grep hit, and the helpers it
calls were not followed further.

## Repair shape

- Give the chart-class arm a lever true of it at every door, ending
  `LeverOnly`.
- Render both doors through `Indeterminate::undecided`.
- Move `Image` out of `JoinReading` (or out of "undecided") so that
  every door gives it one verdict, ending it through
  `PcurveCertifyError::ending` where it is a decision's arm.
- Re-baseline `undecided_text::a_chart_image_that_did_not_derive_ends_without_a_recourse_today`.
- Re-baseline, with literal text, `edge_join.rs`'s `undecided_text`
  tests and `validate.rs`'s `JoinUndecidedAtRest` fixtures.
