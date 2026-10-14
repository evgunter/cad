---
id: band-scale-torus-geometry-ends-as-a-kernel-defect
kind: issue
title: chart_pcurve's torus arm ends band-scale geometry as Unsized::Defect: a thin tube or near-horn torus at the class gates, a near-horn Villarceau circle at its gates and incidence test, and the parallel route's unit_sign at K ≤ 3
status: open
opened: 2026-10-14
priority: P3
cost: M
design: true
refs: [a-villarceau-tangent-parts-gated-zero-ends-as-a-kernel-defect]
needs_ev: true
---

## What

`chart_pcurve`'s torus arm (`crates/geom-brep/src/pcurve_cache.rs`)
refuses a circle with `Escalated{ChartWinding}` or
`Escalated{ChartIncidence}`. `PcurveCheck::recourse` (:2007) routes
both checks to `Unsized::Defect`: "There is no way through: this is a
kernel defect; report it". Every pose below is user geometry at a
tolerance's scale, and every one ends that way.

1. **The router's class gates.**
   - `pcurve_torus_chart_axial` (:7950) reads the harmonic sup of the
     axial parts of `a` and `b`: `R·sin θ`, which is `r` on a
     Villarceau circle.
   - `pcurve_torus_chart_meridian` (:8005) reads `(a × b)·e / r`, which
     is `(R/r)·R·cos θ` and so `(R/r)·√(R² − r²)` on a Villarceau
     circle.

   A tube, or a `√(R² − r²)`, at the band's scale escalates there as
   `ChartWinding`. Measured on each torus's Villarceau circle at the
   witness band `(1e-9, 1e-8)`:
   - `R = 2`, `r = 3ε`: escalates on `_axial`, margin `3e-9`.
   - `R = 1e-4`, `R − r = 1e-13`: escalates on `_meridian`, margin
     `4.47e-9`.
   - `R = 1e-3`, `R − r = 1e-14`: escalates on `_meridian`, margin
     `4.47e-9`.
2. **The Villarceau gates and the incidence test, near a horn torus**
   (PR 5089's second review). Take `R = 2`, `R − r = 1e-6`, band
   `(1e-9, 1e-8)`, and the Villarceau circle moved along its own
   normal:
   - by `3ε` or `10ε` it escalates as `ChartIncidence` on
     `pcurve_torus_villarceau_plane`;
   - at the pinch phase off a sample, it escalates on
     `pcurve_torus_chart_incident` at `30ε` and `100ε`;
   - at `R − r = 1e-4`, it escalates on `pcurve_torus_chart_incident`
     from `3ε` to `30ε`.

   `pcurve_villarceau::a_plane_gate_in_band_near_the_horn_escalates_on_it`
   pins the `_plane` rows.
3. **`unit_sign`, through the router's parallel route, at `K ≤ 3`**
   (PR 5089's first review). The parallel arm (:7963) sends a circle to
   `torus_oblique_circle` on `_axial` Zero (`R·sin θ ≤ ε`) and
   `pcurve_torus_chart_centered` apart (`‖w_r‖ ≥ K·ε`). Passing the
   Villarceau gates then needs `‖w_r‖ ≤ r + ε` (`_offset`) and
   `r ≤ R·sin θ + ε` (`_tilt`), so `‖w_r‖ ≤ 3ε`. That is reachable only
   at `K ≤ 3`, and there the tube-rate part decides Zero, and the gate's
   rejected Zero ends as a defect. The reproduction:
   - `R = 2`, `r = 1.8e-9`, band `(1e-9, 2e-9)`;
   - a circle of radius 2 centred `(2.5e-9, 0, 0)`, its normal
     `(0, −s, √(1 − s²))` with `s = 0.9e-9/2`;
   - result: `Escalated{ChartIncidence}` on
     `pcurve_torus_villarceau_tube_rate`, margin `9e-10`, tagged
     rejected Zero.

   At `K = 3` (`r = 1.9e-9`, `h = 0.95e-9`, `δ = 2.9e-9`), and at
   `K = 10`, it escalates earlier, on `_centered`.

On the both-apart route `unit_sign`'s magnitudes restate the class
gates. The tube-rate part is `_axial`'s own number. The orientation
part is `_meridian`'s divided by `R/r`. `R·cos θ` at the band's scale
forces `R − r ≤ (K·ε)²/2R`, where `R/r` is one to below rounding, so
there a Zero or in-band `unit_sign` reading would be two readings
disagreeing: a defect, rightly. That is the comment at `unit_sign`.

## The design question

The obvious ending is a sized decision with the tolerance offer.

- **The tube's half** is `TorusConvention::Tube.sized()`.
- **The meridian half** reads `(R/r)·√(R² − r²)`, which is neither
  `R − r` nor `√(R² − r²)`. It wants its own noun, with
  `TorusConvention::Ring`'s lever.
- **Each gate's in-band arm has two readings:**
  - `_axial`'s is a circle tilted off the equator by a band-scale angle
    as well as a thin tube;
  - `_meridian`'s is a circle whose plane sits a band-scale angle off
    the axis as well as a near-horn torus;
  - the near-horn Villarceau rows are a circle moved a band-scale
    distance off the class as well as a torus whose class is
    ill-conditioned there.

  So the sized noun and the lever have to fit both readings, or the arm
  has to tell them apart: for example, by deciding the torus's own `r`
  and `R − r` (`geom::torus_tube`, `geom::ring_torus`) before the class
  gates read them.

That needs a designer's look before a lane builds it.

## The question for Ev

One question covers this item and its sibling
(`chart-winding-escalates-on-the-first-candidate-at-a-band-sized-lever`,
`band-scale-torus-geometry-ends-as-a-kernel-defect`): what certification
owes a row whose angular or curvature-scale quantities sit at a
band-sized lever, where today the refusal ends as "a kernel defect;
report it". Two designers weighed it and converged after one round: on
a periodic chart the winding is structure the row stores and `certify`
compares exactly, never decided in metres (the near-apex child row then
certifies); the torus convention is decided where a torus face is
admitted; a closed-form class gate in band means "class not shown" and
the carrier takes the general (projected) lane. The C4 text the answer
changes is in the `[ev]` PR that carries this question.
