---
id: plane-nurbs-tube-straddles-a-curved-dome-at-coarse-eps
kind: issue
title: ssi: limb 3 refuses TubeStraddles on the plane × curved-dome cuts at ε 1e-6
status: open
opened: 2026-10-02
priority: P1
cost: M
---


Found by `ssi-a-seed-refined-off-the-chart-is-marched`'s lane. This is
one of the two refusals that
`plane-nurbs-ssi-does-not-certify-a-curved-dome` leaves unexplained.

## What

At ε 1e-6, limb 3 refuses the tilt cut of the dome `W(d)` with
`TubeStraddles { verdict: Zero(margin 0.0) }` at every d measured from
0.5 to 3 (52 to 137 boxes). This holds once the seed off the chart no
longer reaches the fit. Before that fix it showed at d = 0.5 alone,
because the other rows refused limb 1 first. The level loop refuses
the same way at every d. At ε 1e-9 the same cuts get past limb 3 and
refuse limb 2 instead.

So limb 3's transversality enclosure over the tube chain straddles
zero on a branch that the march traced without any
`ssi_transversality` refusal. It is not yet measured, but the likely
reading is that
the tube boxes at ε 1e-6 are wide enough that the wall-normal interval
over one box spans the plane normal's direction. If so, it is a
resolution limit of limb 3 at coarse ε, not a sliver (F6), and the
refusal's text misnames it.

## Repro

`crates/geom-brep/tests/m5_pr7_ssi.rs`,
`a_seed_settled_off_the_walls_chart_is_no_branch`, at
`CAD_TOLERANCE_EPS=1e-6`. It pins this refusal at d = 1 and d = 2.

## Next

Measure the straddling box (its chart extent, and both normal
enclosures) before choosing between refining the tube chain where it
straddles and renaming the refusal.
