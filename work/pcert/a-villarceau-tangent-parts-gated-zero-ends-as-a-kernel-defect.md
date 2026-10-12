---
id: a-villarceau-tangent-parts-gated-zero-ends-as-a-kernel-defect
kind: issue
title: torus_oblique_circle's decide_positive gate on a Villarceau tangent part ends a decided zero as ChartIncidence's Unsized::Defect
status: open
opened: 2026-10-12
priority: P3
cost: E
---

(Found in review of ENCL's PR 4874, `unsized-fit-reads-a-rejected-zero-on-the-undecided-arm-as-the-last-resort`. That PR first added "no unsized decision is a gate" to `Unsized`'s doc, with a `debug_assert!` on the gate tag. This site falsifies both, so both came out.)

## What

`geom_brep::pcurve_cache` `torus_oblique_circle` (about pcurve_cache.rs:8247), closure `unit_sign`, runs `geom_core::k_stats::decide_positive` on each Villarceau tangent part's magnitude, `Margin::levered(part.abs(), big)`. A gate rejects a decided zero by returning `Err(Indeterminate)`, and the margin carries the rejection tag (`MarginDiag::rejected_sign() == Some(Sign::Zero)`). The error is wrapped by `.map_err(incidence_escalated)` (about pcurve_cache.rs:8382) as `PcurveCertifyError::Escalated { check: PcurveCheck::ChartIncidence, .. }`.

`PcurveCheck::recourse` routes `ChartIncidence` to `Unsized::Defect.recourse(RefusedArm::Undecided(cause), reading)`, which gives the defect ending. The site's own comment says what that magnitude reaches zero on: the tube, or `R − r`, at the band's own scale (a near-horn torus), where "the class is undecided at this tolerance". That is the user's geometry at a scale a tolerance decides, not a kernel defect. Yet the decided zero ends as a defect. It also rides the `Undecided` arm, so `SizedDecision`'s `at_zero` tag reading never sees it.

## Question

Give the magnitude gate its own decision, sized on the tube and `R − r` with the tolerance offer, and read the tag the way `SizedDecision::recourse` does. Or state why a defect is the right ending for it. Separately, consider whether `Unsized::recourse` should refuse a gate-tagged arm by type rather than by doc.
