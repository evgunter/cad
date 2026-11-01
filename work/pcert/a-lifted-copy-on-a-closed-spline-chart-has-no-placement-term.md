---
id: a-lifted-copy-on-a-closed-spline-chart-has-no-placement-term
kind: issue
title: On a spline chart closed in a channel, the iso and fitted lanes carry no placement term, so whether a row stored a knot period off its placed image certifies is unpinned
status: open
opened: 2026-10-19
priority: P3
cost: M
---

Filed by PCERT B2b's second fix pass (G7, from the delta review of
PR 5343).

## What

C4: a row is its placed image, and a lifted copy, the row with a deck
element, is never a row of its own. B2b makes check 4 refuse a lifted
copy on every closed-form lane of an **analytic** chart: the harmonic
lane by its fidelity offsets, the focal-section, spiric and projected
lanes by their placement terms (`placement_terms`, read off
`angle_periods`).

A **spline** chart closed in a channel has a period too: the knot
domain's length, decided closed by `topo` (`chart_u_period` in `u`, `chart_v_period`
in `v`, both through `net_closes` in `crates/topo/src/pcurves.rs`). Its rows certify through the iso
lane (`IsoLine`, `IsoArc`: the control-difference hull) or the fitted
and general lanes (the hull of the composed net). None carries a
placement term, and `angle_periods` answers `None` there. A spline
chart's map is not periodic as a formula: past its knot domain the net
is extrapolated, or refused. So a row a whole period over may refuse
for that reason, or may certify against the extrapolated net. Nothing
pins which.

## What to decide

Whether each spline lane refuses a lifted copy, and as which check:
- probe `IsoLine` / `IsoArc` rows translated by the knot period on a net
  closed in `u` (`pole_slit_tests::prism_patch_across_the_seam`), and on
  a net closed in `v` (`pole_slit_tests::prism_patch_across_the_v_seam`),
  at `f64` and at `Interval`;
- then either pin the refusal, or give those lanes a placement term read
  off the chart's own periods (which needs the band-decided closure from
  `topo`, or a placement the mint and check 4 share).

`work/pcert/chart-boundary-recertifies-a-lifted-iso-line-off-the-knot-domain.md`
is the same question asked of `chart_boundary`'s re-certification.
