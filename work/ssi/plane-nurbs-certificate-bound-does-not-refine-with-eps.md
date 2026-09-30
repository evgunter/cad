---
id: plane-nurbs-certificate-bound-does-not-refine-with-eps
kind: issue
title: Limb 2's between-samples bound is a function of a FIXED sample schedule, so an exact intensional description refuses at small enough epsilon
status: open
opened: 2026-09-15
priority: P1
cost: H
---


(S-TINT orchestrator) Filed from S-TINT's `D70`, which is a test-suite
row about thirteen silent stand-downs. Eleven of them stand down on this
certificate, and the question *"why is this fixture ε-conditional"* has
a kernel answer rather than a test answer. Filed here because
`crates/geom-brep/src/edge_nurbs.rs` is TRIM's territory
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
envelope findings. **Whether that is the right fix is TRIM's call**, and
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
