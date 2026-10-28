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
on that route the parts restate gates decided apart and decide too, up
to rounding. Both routes' undecided readings therefore end as the class
not shown, a lane choice: "not yet" (`ChartClass`). That is the comment
at `unit_sign`.

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

Ev asked (on that PR) for a general principle behind the third answer,
since booleans decide their in-band cases differently and the lint is a
separate stage. Two more designers converged on one: a reading's
consumer is either a lane choice (it picks a form for one certified
statement, takes a lane that does not rest on the answer, and records
nothing) or a structural act (it changes the result's cells, records a
Zero for the lint and refuses in band). That paragraph goes in D3, with
a sentence in D10.

## The answer

Ev on `[ev]` PR 5113 (2026-10-14/15): decision 1 accepted (windings are structure the row stores and `certify` compares exactly); decision 3 replaced by the D3 principle (a class gate is a lane choice: in band or definite it takes a lane whose certificate does not rest on the class, records nothing, never ends as a defect; where no lane exists yet it refuses lever-only with "not yet", never a tolerance offer); the edge join records its class Zeros (D10). Decision 2 (the torus convention at admission) is split out to `a-torus-faces-convention-is-decided-where-the-face-is-admitted`, waiting on the import-validation conversation.

- 2026-10-20 (ENCL implementer, `encl/unsized-gate-word`): item 3's `unit_sign` reproduction is pinned whole as `pcurve_villarceau::a_tube_rate_gates_rejected_zero_renders_its_current_ending` (`crates/geom-brep/tests/pcurve_villarceau.rs`): "pcurve certification: ChartIncidence at sample 0 escalated: margin 9e-10 lies within the zero band (±1e-9). There is no way through: this is a kernel defect; report it". The pin records the ending this row says is false, so the fix the answer above schedules (a class gate is a lane choice, never a defect) moves it visibly; re-baseline it then. The gate's tag (`MarginDiag::rejected_sign`, Zero here) adds no word to the payload, which only a rejected nonzero sign does.
- 2026-10-27 (PCERT E1d, Ev's ruling on PR 5838): `Escalated{ChartIncidence}` no longer routes to `Unsized::Defect`. An incidence read in band ends in the geometry's lever at every door (`geom_brep::recourse::INCIDENCE_UNSHOWN`). So the pinned row above is re-baselined to "Recourse: the edge's curve is not shown on the face it bounds at this tolerance: move the geometry so it lies clearly on that face". The torus arm's `Escalated{ChartWinding}` arms are untouched and still end in the defect, and so is the class-gate half of this item.
- 2026-10-27 (PCERT E1d, review fix G2): the Villarceau tangent's parts are not an incidence reading, so they no longer escalate `ChartIncidence`. At `K ≤ 3` (the reproduction above) the tube-rate part's rejected Zero is a class reading and ends lever-only "not yet" (`ChartClass`, through the incidence test's witness); past `K = 3` it ended briefly as a defect check of its own (`ClassReread`), deleted on review (H6): nothing reaches it, so the both-apart route's readings end as the class reading too. The pinned row is renamed `pcurve_villarceau::a_tube_rate_gates_rejected_zero_is_a_class_reading` and re-baselined to "pcurve certification: ChartClass at sample 0 escalated: margin 9e-10 lies within the zero band (±1e-9). There is no way through yet".
