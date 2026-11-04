---
id: limb-2s-floor-on-a-sheet-reads-the-whole-torus
kind: issue
title: Limb 2's floor on a spindle sheet reads the whole torus's distance, so a carrier on the other sheet refuses as the kernel's limit rather than a measured miss
status: open
opened: 2026-11-04
priority: P3
cost: M
---

(SPIRICARM, found by `a-football-revolved-part-way-refuses-at-its-cap-meridians-certificate`.)

## What

Limb 2 reads a certified floor of a carrier's distance from its operand at
each break (`geom-brep`'s `projected::distance_floor`, over
`Meters::divisor`), and a floor past the band is the measured miss
(`AnalyticRung3Refusal::Limb`). On a spindle sheet the divisor is the
whole torus's (`Meters::divisor`'s `Torus | Spindle` arm): `|f|` vanishes
on both sheets, so a carrier on the other sheet has floor zero everywhere.
Its bound (this sheet's, `Meters::over` with the side check, or the
distance from an apex) does not clear, and limb 2 refuses
`KernelLimit` ("the kernel's approximation limit") for a carrier that is
off the sheet by `√3` at `R = −½, r = 1`. Witnessed by
`crates/geom-brep/tests/closed_form_limb2.rs`
`limb_2_against_a_sheet_reads_that_sheet_up_to_its_apexes`, which pins
that `KernelLimit`.

At the build doors limb 1's samples read the sheet's own residual and
refuse such a carrier first, so this is the refusal's kind, not its
soundness.

## Done when

A floor on a sheet is this sheet's distance (the circle's `|m − r|` where
the side holds, else the distance from the nearer apex, each from below),
so a carrier on the other sheet refuses as a measured miss, and the row
above pins `Limb`.
