---
id: a-torus-circles-class-gates-end-a-band-scale-tube-as-a-kernel-defect
kind: issue
title: chart_pcurve's torus-circle class gates end a tube, or a √(R² − r²), at the band's scale as ChartWinding's kernel defect
status: open
opened: 2026-10-14
priority: P3
cost: M
design: true
---


## What

`chart_pcurve`'s torus arm routes a circle by two class gates
(`crates/geom-brep/src/pcurve_cache.rs`):

- `pcurve_torus_chart_axial` (:7867) reads the harmonic sup of the
  axial parts of `a` and `b`. Zero means a parallel.
- `pcurve_torus_chart_meridian` (:7922) reads `(a × b)·e` over `r`.
  Zero means a meridian; apart sends the circle to
  `torus_oblique_circle`.

On a Villarceau circle these are the tube `r` and `√(R² − r²)`. When
either is at the band's scale the gate escalates as
`Escalated{ChartWinding}`, and `PcurveCheck::recourse` (:1959) routes
`ChartWinding` to `Unsized::Defect`: "There is no way through: this is a
kernel defect; report it". That is user geometry at a tolerance's scale
ending as a kernel defect.

Measured through `chart_pcurve`, on each torus's Villarceau circle
(witness band (1e-9, 1e-8)), in
`a-villarceau-tangent-parts-gated-zero-ends-as-a-kernel-defect`'s
closure:

- `R = 2`, `r = 3ε`: escalates on `_axial`, margin `3e-9`.
- `R = 1e-4`, `R − r = 1e-13`: escalates on `_meridian`, margin
  `4.47e-9`.
- `R = 1e-3`, `R − r = 1e-14`: escalates on `_meridian`, margin
  `4.47e-9`.

## The design question

The obvious ending is a sized decision with the tolerance offer.

- The tube's half is `TorusConvention::Tube.sized()`.
- The meridian half's size is `√(R² − r²)`, not `R − r`, so it would
  want its own noun, with `TorusConvention::Ring`'s lever.

But both gates also answer the class question for every circle on a
torus. `_axial`'s in-band arm is a circle tilted off the equator by a
band-scale angle as well as a thin tube, and `_meridian`'s is a circle
whose plane is at a band-scale angle off the axis as well as a
near-horn torus. So the sized noun, and the lever, have to fit both
readings, or the arm has to tell them apart: for example by deciding
the torus's own `r` and `R − r` (`geom::torus_tube`,
`geom::ring_torus`) before the class gates read them. That needs a
designer's look before a lane builds it.
