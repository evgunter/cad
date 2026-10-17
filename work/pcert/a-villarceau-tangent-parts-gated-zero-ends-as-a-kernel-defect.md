---
id: a-villarceau-tangent-parts-gated-zero-ends-as-a-kernel-defect
kind: issue
title: torus_oblique_circle's decide_positive gate on a Villarceau tangent part ends a decided zero as ChartIncidence's Unsized::Defect
status: closed
opened: 2026-10-12
priority: P3
cost: E
closed: 2026-10-14
pr: 5089
---

(Found in review of ENCL's PR 4874, `unsized-fit-reads-a-rejected-zero-on-the-undecided-arm-as-the-last-resort`. That PR first added "no unsized decision is a gate" to `Unsized`'s doc, with a `debug_assert!` on the gate tag. This site falsifies both, so both came out.)

## What

`geom_brep::pcurve_cache` `torus_oblique_circle` (about pcurve_cache.rs:8247), closure `unit_sign`, runs `geom_core::k_stats::decide_positive` on each Villarceau tangent part's magnitude, `Margin::levered(part.abs(), big)`. A gate rejects a decided zero by returning `Err(Indeterminate)`, and the margin carries the rejection tag (`MarginDiag::rejected_sign() == Some(Sign::Zero)`). The error is wrapped by `.map_err(incidence_escalated)` (about pcurve_cache.rs:8382) as `PcurveCertifyError::Escalated { check: PcurveCheck::ChartIncidence, .. }`.

`PcurveCheck::recourse` routes `ChartIncidence` to `Unsized::Defect.recourse(RefusedArm::Undecided(cause), reading)`, which gives the defect ending. The site's own comment says what that magnitude reaches zero on: the tube, or `R − r`, at the band's own scale (a near-horn torus), where "the class is undecided at this tolerance". That is the user's geometry at a scale a tolerance decides, not a kernel defect. Yet the decided zero ends as a defect. It also rides the `Undecided` arm, so `SizedDecision`'s `at_zero` tag reading never sees it.

## Question

Give the magnitude gate its own decision, sized on the tube and `R − r` with the tolerance offer, and read the tag the way `SizedDecision::recourse` does. Or state why a defect is the right ending for it. Whether `Unsized::recourse` should refuse a gate-tagged arm by type is a separate, ENCL question: `work/encl/unsized-recourse-takes-a-gate-tagged-arm-and-renders-no-gate-word.md`.

## Closed: merged into `band-scale-torus-geometry-ends-as-a-kernel-defect` (PCERT, 2026-10-14)

**Reproduced.** The router reaches `torus_oblique_circle` by two routes:
- both class gates apart;
- the parallel arm: `pcurve_torus_chart_axial` Zero and
  `pcurve_torus_chart_centered` apart.

On the parallel route the router has not read the tube-rate part apart.
Passing the Villarceau gates there bounds `‖w_r‖ ≤ 3ε`, so the route
reaches `unit_sign` at `K ≤ 3`, where the tube-rate magnitude's gate
decides Zero and ends as `ChartIncidence`'s defect.

PR 5089's first review reproduced it:
- `R = 2`, `r = 1.8e-9`, band `(1e-9, 2e-9)`;
- a radius-2 circle centred `(2.5e-9, 0, 0)`, its normal tilted
  `0.9e-9/2` off the axis;
- result: `Escalated{ChartIncidence}` on
  `pcurve_torus_villarceau_tube_rate`, margin `9e-10`, rejected Zero.

At `K = 3` and `K = 10` the same family escalates earlier, on
`_centered`.

On the both-apart route the two magnitudes restate the class gates'
numbers:
- `_axial` reads `R·sin θ`, the tube-rate part itself;
- `_meridian` reads `(a × b)·e / r = (R/r)·R·cos θ`, the orientation
  part times `R/r`, which is one to below rounding wherever `R·cos θ` is
  at the band's scale.

So there a Zero reading would be a disagreement, and a defect is
right. The `unit_sign` comment states both routes and the `K` bound.

**Merged.** This defect is one case of a wider class: band-scale torus
geometry ending as `Unsized::Defect`.
- at the router's axial and meridian gates;
- at a near-horn Villarceau circle's gates and incidence test;
- here, at `K ≤ 3`.

That class is
`work/pcert/band-scale-torus-geometry-ends-as-a-kernel-defect.md`
(`design: true`), which carries both reviewers' poses and the design
question of the sized ending. This item closes into it.
