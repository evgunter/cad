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

## Closed: not reproducible at `unit_sign` (PCERT, 2026-10-14)

`unit_sign`'s magnitude gates read numbers the router has already
decided apart, on the same band, before it calls
`torus_oblique_circle`:

- The tube-rate part, levered at `R`, is `r`. `pcurve_torus_chart_axial`
  (the harmonic sup of `a·e` and `b·e`) reads the same `r` on a
  Villarceau circle.
- The orientation part is `√(R² − r²)`. `pcurve_torus_chart_meridian`
  (`(a × b)·e`, over `r`) reads the same `√(R² − r²)`.

The router reaches `torus_oblique_circle` only after it decides both
apart. So a decided Zero (`≤ ε`) here would contradict a reading
`≥ K·ε` of the same number. An in-band reading is possible only in the
rounding sliver between the two spellings.

Measured through `chart_pcurve`, on the Villarceau circle of each torus
(witness band (1e-9, 1e-8)):

| Torus | Result |
|---|---|
| `R = 2`, `r = 0.5ε` | imaged as a parallel (the axial gate decides Zero) |
| `R = 2`, `r = 3ε` | `Escalated{ChartWinding}` on `pcurve_torus_chart_axial`, margin `3e-9` |
| `R = 2`, `r = 30ε` | imaged as a focal section, both parts definite |
| `R = r = 2` | imaged as a meridian |
| `R = 2`, `R − r` = 4 ulps, then `2ε` | imaged as a focal section, both parts definite |
| `R = 1e-4`, `R − r = 1e-15` | imaged as a meridian |
| `R = 1e-4`, `R − r = 1e-13` | `Escalated{ChartWinding}` on `pcurve_torus_chart_meridian`, margin `4.47e-9` |
| `R = 1e-3`, `R − r = 1e-14` | the same, margin `4.47e-9` |

The orientation part is `√(R² − r²)`, not `R − r`. It reaches the band
at `R − r ≈ (K·ε)²/2R`, which is below an ulp of `R` at `R = 2`.

The gate stays as a fail-loud backstop: a Zero or in-band reading there
is two readings of one number disagreeing, a kernel defect, and that is
the ending `ChartIncidence` gives it. The site's comment says so.

What the item saw is real one level up. A tube, or a `√(R² − r²)`, at
the band's scale ends at the router's axial and meridian gates as
`ChartWinding` → `Unsized::Defect`, which is user geometry at a
tolerance's scale ending as a kernel defect. That is filed as
`work/pcert/a-torus-circles-class-gates-end-a-band-scale-tube-as-a-kernel-defect.md`.
