# M5 PR 3 spec (binding): NURBS substrate part 1

Status: BINDING. Deviations reported, never improvised. Authority:
CURVED-DESIGN C11 (scope), C6 (f64-structure/generic-certification
split), C9 (ring ops only), C12.8 (no new `Real` methods); M5-PLAN
PR 3 as amended (linalg LSQ/SVD land with their consumers in PR
4/PR 7, NOT here).

## What this PR is

The NURBS types, evaluators, and knot algebra the rest of M5 rides
on: B-spline/NURBS curves (2-D and 3-D) and surfaces, de Boor
evaluation + derivatives generic over `Real`, and the knot-algebra
toolkit (insertion, refinement, removal with Tiller bounds, degree
elevation). NO fitting, NO projection/inversion (PR 4), NO pcurve
wiring (PR 6), NO certification of NURBS carriers (certify.rs
keeps refusing them — that refusal is pinned, not removed), NO
Ellipse (PR 5), NO tessellation/props changes.

## Homes and layering (binding)

- `geom-core::spline` (new module): knot-vector type + validation,
  span/basis machinery, and every algorithm that manipulates
  STRUCTURE (knots, degrees, weights, alpha coefficients). Both
  geom-curves and geom-surfaces consume it (they do not depend on
  each other — geom-surfaces/src/lib.rs:10-13).
- `geom-curves::nurbs`: `NurbsCurve2<T>` (Point2 control points —
  the future pcurve substrate; NOT wired into any enum yet) and
  `NurbsCurve3<T>`.
- `geom-surfaces::nurbs`: `NurbsSurface<T>`.
- `Curve3::Nurbs` (geom-curves/src/lib.rs:138) and
  `Surface::Nurbs` (geom-surfaces/src/lib.rs:237) — today unit
  placeholder variants — gain payloads `Nurbs(Arc<NurbsCurve3<T>>)`
  / `Nurbs(Arc<NurbsSurface<T>>)`. **Consequence accepted and
  binding: `Curve3`/`Surface` lose `Copy`, stay `Clone` (cheap via
  Arc; immutable payload, D9-clean).** The compiler then walks you
  through every dispatch site; each existing `Nurbs` arm KEEPS its
  current behavior (poison in evaluators until this PR implements
  them; typed refusal in certify/splitting/boolean gates —
  unchanged semantics, verify the refusal tests still pass). If
  Arc causes genuine trait friction, plain inline payload
  (Clone-only) is the fallback — report which shipped.
- Evaluator arms in `Curve3::{eval,deriv,deriv2}` and
  `Surface::{eval,deriv_u,deriv_v,normal,deriv_uu,deriv_uv,
  deriv_vv}` route to the payload's methods (replacing
  poison_point/poison_vec). `geom-brep::implicit` Nurbs arms STAY
  poison (no implicit form — that is C2.1's foot-point story, PR
  4); update their comments to say so.

## Data model (binding)

- Knots: `Vec<f64>`, weights: `Vec<f64>`, degree: `usize` — these
  are STRUCTURE (C6: cache shape is an f64-lane artifact; topology
  never reads it). Control points: `Vec<Point{2,3}<T>>`.
- Construction is validated, typed-error, fail-loud:
  `NonPositiveWeight` (w > 0 enforced at construction — the
  convex-hull property every C9 hull bound stands on, Book p. 293),
  `KnotVectorInvalid` (non-decreasing; CLAMPED required in v1 —
  end-knot multiplicity p+1; interior multiplicity ≤ p allowed and
  documented). Periodic/unclamped forms are deliberately out of
  scope until a consumer exists (report as designed absence, not a
  gap). f64 comparisons in construction/validation are legal —
  this is structure selection, not topology (C6); route NOTHING
  through k_stats here (no topology decision exists in this PR).
- Rational evaluation: homogeneous de Boor — combine (wᵢ·Pᵢ, wᵢ)
  with basis coefficients lifted via `T::from_f64`, single
  ascending-index pass, fixed association documented at the fn,
  then one division. Ring ops + from_f64 ONLY in generic code: no
  comparisons, no new `Real` methods (C12.8), no fused ops, no
  order-implicit reductions (the geom-curves :72-79 discipline
  verbatim).

## The evaluation contract (binding — the span question)

Generic code cannot compare `t` against knots (Real is
comparison-free by design). The split:
- **Core, generic**: `eval_in_span(span, t)` (and derivative
  versions) — de Boor restricted to a caller-supplied span index;
  pure ring ops; total for every `T`. Caller contract documented:
  `t` must lie in the span's knot interval; outside it the result
  is the span's polynomial extension (documented garbage-out, like
  Mat inverse on singular input — NOT poison, because detecting it
  would need a comparison the trait forbids).
- **Convenience, f64-structural**: `eval(t: f64)` etc. on the
  concrete types locate the span by binary search on the f64 knot
  vector (fixed, documented tie-break at knot values: the span
  whose half-open interval [uᵢ, uᵢ₊₁) contains t, last span closed)
  and call the core. The enum evaluator arms (`Curve3::eval` at
  `T`) route through the VALUE CHANNEL span of... no — they cannot
  read a `T`'s value. Binding resolution: the enum arms take the
  parameter as `T` but the NURBS payload carries NO span oracle
  for generic `T`, so `Curve3::eval` Nurbs arms are implemented
  via a spec'd `SpanLocate` seam: a small private trait
  implemented for f64 (binary search), Interval (hull of the spans
  overlapped by [lo,hi] — sound containment; uses Bounds), Probe
  (via its f64), and Dual<T: SpanLocate> (value channel — kink
  convention: at a knot, the span tie-break above, matching the
  "differentiates the program as evaluated" rule). This trait
  lives in geom-curves/geom-surfaces (NOT on Real — C12.8 stands;
  it is a structure-selection seam exactly like floor/copysign's
  per-instantiation kink handling, cite them). If Dual<Interval>
  blanket-impl friction arises, restrict to the instantiations the
  kernel actually evaluates and report.
- Interval containment obligation (tested): for an interval `t`
  inside one span, eval(t: Interval) ⊇ eval(t: f64) for every f64
  in it; for a knot-straddling interval, the hull-of-spans result
  still contains every pointwise value (continuity — assert on
  sampled probes).

## Knot algebra (binding scope)

All operate structure-first (f64 knots/alphas) with control-point
combination generic (from_f64-lifted alphas, fixed association):
- Insertion §5.2 (single + to-multiplicity — the future
  `split_edge` substrate), refinement §5.3.
- Removal §5.4 returning the Tiller error BOUND (Eq. 9.81 shape)
  to the caller — removal is bounded, never silent; a
  planted-perturbation test verifies the bound is honest
  (|C - Ĉ| ≤ bound on a dense sample).
- Degree elevation §5.5.
- Each: evaluation-invariance tests (pre/post agree within a tight
  tolerance at f64 — NOT bit-equal, floating point moves; state
  the tolerance and why — plus Interval containment of the f64
  results on shared samples).

## Tests / acceptance (all rows foreground, one at a time)

1. Closed-form differential suite: circle as exact rational
   quadratic (§7.3, w₁ = cos θ) vs `Curve3::Circle` — eval +
   deriv + deriv2 agree within stated tight tolerances across a
   dense schedule; a Bézier special case (all-1 weights, single
   span) vs binomial closed form; surface case: cylinder patch as
   NURBS vs `Surface::Cylinder`.
2. Determinism: same inputs ⇒ bit-identical outputs across
   repeated evaluation and across knot-algebra round trips
   (structure equality is Vec equality; value bit-equality via
   to_bits on coordinates).
3. Positive-weights + knot-validation refusals pinned (typed
   errors, exact variants).
4. Interval lane: containment tests above; poison propagation
   (poisoned control point/parameter ⇒ poisoned result, never a
   decision).
5. Full battery green at 3ε + Interval suites (the new tests join
   the existing lanes — no new CI matrix rows, floor stays 18).
6. Every existing test that matched unit `Nurbs` variants (typed
   refusals, poison arms) still passes with unchanged semantics.
7. `cargo clippy` clean per workspace lints; missing_docs
   satisfied (module docs carry the conventions: clamped-v1,
   span contract, fixed association orders).

## Out of scope (repeat, binding)

Fitting/A9.10, point projection/inversion, LSQ/SVD, pcurve types
in any enum, certification changes, persistence/schema, Ellipse,
sweeps/lofts, tessellation, BVH, demos. The `SpanLocate` seam is
crate-private — no public API commitment beyond the types and
algorithms above.
