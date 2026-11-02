---
id: plane-nurbs-certificate-bound-does-not-refine-with-eps
kind: issue
title: Limb 2's between-samples bound is a function of a FIXED sample schedule, so an exact intensional description refuses at small enough epsilon
status: open
opened: 2026-09-15
priority: P1
cost: H
design: true
---


(S-TINT orchestrator) Filed from S-TINT's `D70`, which is a test-suite
row about thirteen silent stand-downs. Eleven of them stand down on this
certificate, and the question *"why is this fixture ε-conditional"* has
a kernel answer rather than a test answer. Filed here because
`crates/geom-brep/src/edge_nurbs.rs` is ISO's territory
(`work.py territory`). **Nothing is proposed and no fix is scheduled** —
this row states a defect and the measurement that would settle it.

## The claim

**A valid intensional description should stay valid as ε decreases.**
`Intersection { s1, s2, witness }` names a locus; it is exact and says
nothing about any tolerance. So a description that certifies at
ε = 1e-9 and refuses at ε = 1e-12 is reporting something about the
CERTIFIER, not about the geometry — unless the residual it measures is
genuinely at ulp scale, which at metre-scale coordinates and ε = 1e-12
it is not (ulp(1.0) ≈ 2.2e-16; the tolerance is ~4500 ulps).

The plane × NURBS lane does exactly that, and its own constants say why.

## The mechanism

Limb 2 bounds `sup_t |S(P(t)) − C(t)|` over the whole span. `C(t)` is
the declared carrier. **`P(t)` is not in the file** — the lane derives
it, as a degree-1 (piecewise-linear) interpolant through foot points at
a fixed schedule:

- `edge_nurbs.rs`'s `PXN_FIT_SAMPLES` = **33**, a `const`.
- `certify.rs`'s `CERT_SAMPLES` = **9**, a `const`.
- `edge_nurbs.rs`'s `PXN_IMAGE_DEGREE` = **1**, a `const`, with its own
  doc naming the two error sources: the image's deviation from the true
  foot path, `O(h²·κ_uv)` at degree 1, and the ring's per-span widening.

**Both error sources are functions of `h` (the schedule step) and of the
model's scale. Neither is a function of ε.** The bound is therefore a
constant of the fixture, and the certificate compares that constant
against a tolerance that varies by three decades across the CI matrix.
Nothing refines when ε tightens.

The module's own class statement is the same fact read forwards:
*"the edges this lane certifies are those whose foot path is straight in
the wall's chart to within `ε` over a schedule step"*. With the step
fixed, the admissible curvature falls as ε falls — **so the same edge
changes class as ε changes**, which the sentence does not say and which
is the part worth noticing.

## The fixture that shows it, and why it is a clean instance

`crates/sweep/tests/m8_4_intersection_iso.rs` (S-TINT's `D70`) lofts an
offset square prism, restates the exactly-planar `y = −1` wall as the
plane it is, and re-describes the flat/bowed seam intrinsically as
`Intersection { plane, bowed }`. That seam's certified between-samples
sup is **6.217e-12 m**, so ε = 1e-12 refuses typed and eleven rows in
that file stand down.

**The carrier is exact, by construction, which is what makes this a
clean instance rather than an argument about fit quality.** Both walls
come out of the same `loft_body` call; the seam edge is the loft of a
profile vertex and is simultaneously a boundary iso-curve of the bowed
patch — the same curve, from the same construction, not a fit to it. And
every one of that corner's control points has `y = −scale` exactly, so
it lies in the restated plane exactly. The true deviation of `C` from
the locus is zero. **The entire 6.217e-12 is the certificate's own
internal approximation.**

Which internal term dominates is NOT established here and is the first
thing a taker measures: the degree-1 image's `O(h²·κ_uv)` deviation from
the true foot path, or the foot points' own projection residual
(`NurbsSurface::project`), or the ring's per-span widening. The doc
records `1.1e-13 m` for the piecewise-linear image on the certifying
fixture, so 6.217e-12 is ~56× that and the same order of phenomenon.
**The claim above holds whichever dominates**, because all three are
fixed-schedule quantities.

## Why it matters beyond one test file

The tree's only current lever is to shrink the model until the fixed
bound fits inside ε. That lever is already pulled once, deliberately and
with its reasoning written down —
`m8_4_intersection_iso.rs`'s `INTERIOR_COLUMN_SCALE = 1.0 / 1024.0`,
taken under #1167 to stop a row asserting nothing at one of the three ε
the matrix draws. It works, and it is the wrong shape: it buys an ε-table
that exercises one thing everywhere by testing the kernel on a
millimetre-scale prism, and it generalises to *"this kernel certifies
intersection edges only on models small enough for a 33-sample schedule"*
— which, if true, is a statement about the kernel that belongs in its
documentation rather than in a test fixture's scale constant.

A cheap arm exists if the image term dominates: at degree 1 the error
falls as `h²`, so clearing 6.217e-12 → under 1e-12 needs `h` smaller by
~2.5×, i.e. a schedule of roughly 81 samples rather than 33 — refinement,
not the algebraic route `PXN_IMAGE_DEGREE`'s docs bank with #264's
envelope findings. **Whether that is the right fix is ISO's call**, and
it may not be: the ring-widening term does not obviously improve with
more spans, and an ε-driven schedule is a different contract from a
fixed one. What this row asserts is only that the present contract
compares a fixed number against a varying one and calls the result a
property of the edge.

## What would settle it

Measure the three terms separately on this fixture at the three ε rows,
and report whether refining `PXN_FIT_SAMPLES` moves the bound as `h²`.
If it does, the bound is refinable and the class boundary is an artifact.
If it does not, the ring widening is the floor and the class boundary is
real — in which case it should be stated in metres per model scale,
where a reader can find it.

## Measured (SSI measurement lane, 2026-10-01, head 04095ca9)

**The premise is half right.** The bound is ε-independent: the same
bits come out at 1e-6, 1e-9 and 1e-12. But it does not refine as `h²`.
It is **ring widening from interval Boehm insertion** in
`to_bezier_spans_extra` → `geom_core::spline::compose`'s
`insert_once_ring`, whose lerp form reads `c₋₁` twice, and it grows as
**N³** under refinement.

On the m8_4 seam at scale 1, N = 32, the certified bound is 1.6926e-12 m
(the row's 6.217e-12 is stale). It splits as:

| term | value (m) | share |
|---|---|---|
| (a) degree-1 image against the true foot path | 0 (structurally: the seam is the wall's `u = 1` column, so the foot path is straight) | 0 |
| (b) foot projection residual | 2.54e-14 | 1.5% |
| (c) ring widening | 1.67e-12 | 98.5% |

The widening piles up along `t`: 1.3e-16 in the first span, 1.53e-12 in
the last two.

Refining (N is set by `PXN_FIT_SAMPLES` and `SSI_CERT_SPANS` together;
`CERT_SAMPLES` plays no part in limb 2, nor does `PXN_WALL_SPANS`):

| N | lerp (shipped) | convex form |
|---|---|---|
| 8 | 3.00e-14 | 2.04e-14 |
| 32 | 1.69e-12 | 5.47e-14 |
| 128 | 1.08e-10 | 1.53e-13 |
| 256 | 8.64e-10 | 2.60e-13 |

Per model scale at N = 32, the shipped code costs 1.69e-12 m per metre,
so it certifies only while L < 0.59 m at ε = 1e-12. The convex form
costs ≈ 3.4e-14 m/m plus a ≈ 3e-14 m absolute floor from the projection.

`INTERIOR_COLUMN_SCALE = 1/1024` is still load-bearing on shipped code.
Under the convex form at scale 1, every row passes at all three ε, and
the four "refuses below 1e-9" pins would be re-baselined (the seam then
attaches). Today 4 rows stand down at 1e-12, not 11: three in
`m8_4_intersection_iso.rs` and `review_probes_m8_4.rs::probe_e_…`.

**What follows.**
1. The fix for the dominant term is the convex form at `insert_once_ring`,
   which PROPS' `f64-refinement-inside-an-enclosure-has-five-more-sites`
   already carries (PR 3524, open). This row parks on it, and the
   measurement is added there as evidence.
2. Once that lands, the **design question** on this row is the
   schedule contract. Three options:
   - (i) a fixed N with the residual floor documented in metres per metre
     of edge extent;
   - (ii) per-span adaptive refinement, driven by the enclosure's midpoint
     (the true residual) rather than its radius;
   - (iii) N driven by ε.
   (iii) is argued against: refinement raises (c) under either form.
   Weighing (i) against (ii) needs term (a) measured on a fixture whose
   foot path is curved in the chart (an oblique plane through the bowed
   wall), which this fixture cannot show.

Off-question findings were filed on PROPS
(`project-eps-point-is-absolute-so-a-km-model-refuses-off-geometry`)
and on NURBS (`an-exact-pcurve-image-certifies-worse-than-an-interpolated-one`),
and the stale numbers are noted on TINT's `D70`.

## Measured: the image term, and why degree alone does not fix it (PCERT, 2026-10-14)

From `work/pcert/a-limb-two-refusal-of-an-exact-closed-form-reads-as-an-off-surface-carrier`.

**Fixture.** `sweep/tests/an_arc_rim_picks_the_row_it_lies_on.rs`'s
`an_intersection_circle_on_a_spline_wall_is_refused_at_the_door`:
- an exact circle, radius 1, restated as the cap × wall `Intersection`
  on a loft wall;
- the wall's chart is rational quadratic in `u`, with an interior knot
  at `u = 0.5`.

At `Tol::witness` it refuses at limb 2, at a break of the composite.
The circle's own foot is on the wall at that break (this PR's
`SsiLimb::ImageMiss`), so the whole miss is the image's.

**How the miss scales.** The fixture was re-run with `PXN_IMAGE_DEGREE`
and `PXN_FIT_SAMPLES` edited locally:

| image | 33 feet | 129 feet | 513 feet |
|---|---|---|---|
| degree 1 | 4.6e-4 m | 3.1e-5 m | — |
| degree 3 | 1.9e-4 m | 1.1e-5 m | 7.1e-7 m, still refused |

- **Degree 1** falls as `h²`, so on this fixture the image term is
  refinable.
- **Degree 3** also falls only as `h²`, not `h⁴`. The foot path's chart
  coordinate has a slope break where the circle crosses the chart's
  knot: `du/dt` jumps 0.332 → 0.350 at `t = π/2`, which is `u = 0.5`.
  The schedule is uniform in `t` and has no break there, so a
  higher-degree interpolant buys nothing past that break.

**What a finer image needs, then:**
1. a schedule that breaks at the foot path's chart-knot crossings (find
   where the foot's `u(t)` or `v(t)` meets an interior knot);
2. a higher degree per piece between those breaks.

Only that pair escapes `h²`. Raising the degree alone does not.
`PXN_IMAGE_DEGREE`'s own doc fixes "one structure" and banks curved loci
on the algebraic route. The schedule, the image and the doc are all in
`edge_nurbs`, which is ISO's territory per `work.py territory`.

## Un-parked (band-rule triage, 2026-10-25)

This row parked on `f64-refinement-inside-an-enclosure-has-five-more-sites` and PR 3524 for the convex form at `insert_once_ring` (term (c) above). PR 3524 closed that site. The one site still open on that row, `crates/mesh/src/chords.rs`, plays no part in limb 2. So the design question in "What follows" item 2 can be taken up now. Before weighing (i) against (ii), re-take the 2026-10-01 table: its numbers are from the lerp form.

## Measured on two spline charts: lily wall 8 (SPLICE, 2026-10-27)

That wall 8 was the leaf's sheath grafted on its blade; since 2026-10-29 the scene builds the leaf as one loft, and the sweep row `crates/sweep/tests/a_sheath_grafted_on_its_blade.rs` builds the same graft.

From `work/splice/the-results-seam-between-two-spline-walls-is-read-by-no-dihedral`.

Once the result's seam between two spline walls reads its dihedral
(`geom_brep::classify_seam`), lily wall 8 (the sheath grafted on its
blade) gets one step further and stops at this row's limb, on the
NURBS × NURBS lane (`edge_nurbs::nurbs_pair_limbs`) rather than the plane
× NURBS one: `describe_intersection` on the shared rim, a `Circle` edge
between a rational loft wall of each body, refuses
`CertifyError::PlaneNurbs(Limb { limb: ImageMiss, margin: 9.007e-6 })`
at `Tol::witness`, in both operand orders. Fixture:
`crates/sweep/tests/a_sheath_grafted_on_its_blade.rs`
`a_sheath_on_its_blade_stops_where_its_walls_touch_on_the_rim` (5 stations); the
tour's `lily.rs` wall 8 (9 stations) stops at the same refusal, its
margin 4.527e-6.

The circle is an exact boundary row of each wall: `topo::pcurves::pcurve_of`
derives its image there as an exact `Pcurve::IsoArc` (the arc-rim class),
which the seam's dihedral reads its chart parameters off. The pair lane
re-derives a degree-1 image from feet instead (`chart_image`), and that
image's miss is the whole refusal. So on this class a third route sits
beside (i) and (ii) above: certify against the exact image the chart's
description already states, where one exists.

**The two margins' scaling (review of PR 5868).** The two fixtures differ
in station count, 5 against 9 (4 against 8 spans along the loft), and the
miss falls from 9.0e-6 to 4.53e-6: about `1/(n − 1)`, not the `h²` a
chord's sagitta would give. Recorded here, not acted on.

**At ε 1e-6 (2026-10-28).** The sweep row's 9.0e-6 miss lands inside
the band `[1e-6, 1e-5)` and escalates (`Escalated { PlaneNurbsHullValue }`,
`"ssi_hull_value_chart"`) rather than refusing; the tour's lily wall 8
escalates the pair's transversality in band at sample 12 (margin 8.3e-6)
before it. Both pins name the site, the rim's certification, not the
cause.
