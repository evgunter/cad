---
id: elevation-past-degree-ten-agrees-to-1e-11-with-no-bound
kind: issue
title: Degree elevation past degree 10 agrees to about 1e-11 of scale and surfaces no bound
status: open
opened: 2026-10-20
priority: P3
design: true
---


**P3.** Found by #5459's FULL review (MINOR-3,
https://github.com/evgunter/cad/pull/5459#issuecomment-6290152933).

**What.** `algebra::elevate_plan` (`crates/geom-core/src/spline/algebra.rs`)
recomposes by exact removals and surfaces no bound. After #5459 its
measured agreement, relative to the curve's largest control coordinate,
is within 1e-12 at degree ≤ 10 on every family the rows and their
held-out checks draw. Past degree 10 it is larger, and no row measures
it:

| input | relative error |
|---|---|
| degree 11, 200 near-uniform knots | 2e-12 |
| degree 12, 200 near-uniform knots | 1e-11 |
| degree 12, graded or jittered vectors | 1–2e-11 |
| degree 25, a `1e-9` pair, weights 1e-3–1e3 | 5e-11 |

On ±10 coordinates that is about 1e-10 absolute, 100× ε at
`CAD_TOLERANCE_EPS=1e-12`. The callers that elevate user geometry to
a common degree (`compose_chain` in `crates/geom/src/curves/compose.rs`,
`skin` in `crates/sweep/src/skin.rs`) take any degree the user gives
them, so a section of degree ≥ 11 reaches it silently. (Main is far worse
on the same inputs: it refuses, or reads 1e-5 to 1e+44.)

**Done when** `elevate_plan` surfaces a bound, or its callers refuse
at ε. The split estimate already computes one per pass
(`conditioned_split`'s `worst(nf)`, times unit roundoff, times the
coordinates' scale). It is first-order, so "steering" grade as `fit.rs`
calls its bound unless the chain is interval-evaluated, and whether
`compose_chain` and `skin` should refuse at ε is a design fork: this
form goes through the designer pass when it is picked up.

**Not a live hazard today, as far as checked.** Every degree-≥11 path
is user geometry handed to `compose_chain` or `skin`, and no test row
outside these elevation rows builds one. Whether any tour scene does
was not checked.

## Pinned (2026-10-20, `nurbs/elevation-envelope-rows`)

The measured envelope is pinned by rows, each capped at about 3× its
worst of 90 curves (CI's test profile), and stated at both callers'
docs. Relative to the curve's largest control coordinate:

| row | shapes | worst | cap |
|---|---|---|---|
| `degree_eleven_elevates_within_its_measured_envelope` | 50 uniform / graded / jittered knots | 4.4e-12 / 2.4e-12 / 8.1e-12 | 2.5e-11 |
| `degree_twelve_elevates_within_its_measured_envelope` | the same | 1.8e-11 / 1.5e-11 / 3.6e-11 | 1.1e-10 |
| `high_degree_clusters_elevate_within_their_measured_envelope` | degree 16 / 20 / 25, clusters of 2 and 4 knots `1e-9` apart | 1e-13 / 4e-13 / 5.9e-12 | 2e-11 |

- **Red on the midpoint split**, which refuses or reads 9e-5 and up on
  every row.
- **Margin 8** turns the cluster row red (2–4e-11 at degree 25).
- **Margin 1** reads alike on these shapes.
- **Timing:** each row takes ≤ 0.15 s. Shapes with 200 knots cost about
  0.6 s per elevation at degree 11–12 and are left out of the fast set
  (degree 12 with 200 knots measured 1–6e-11).

The rows guard against regression; they are not a bound, so this item
stays open on the form above.

## Step 0: which bound (2026-10-21, NURBS build A, scratch)

Both candidate bounds were measured against the real disagreement,
the sup of `|C − E|` at nine points per span, as the envelope rows
read it. Up to 30 curves per shape, polynomial and rational (weights
0.2–5), coordinates ±10, elevated once.

**(A) Certified: the plan chain in the certification ring
(`CurvePlan::apply_certified` on the homogeneous channels).**
Unusable past degree about 4. Each removal's unblend
`x + (y − x)/a` adds its inputs' widths where the `f64` values
cancel, compounding over the recomposition:

| shape | bound / real |
|---|---|
| p ≤ 3 | 1–12 (polynomial), to 2·10² (rational) |
| p = 5, uniform knots | 3·10⁸ |
| p = 8 / 10, uniform knots | 10¹⁴ / 10¹⁸ |
| p = 11–12, 50 knots | 10⁸⁵–10⁹⁵ (overflow) |
| p = 16–25, clusters | 10⁶–10¹⁴ |

Rational weight enclosures reach 0 from p = 8 on uniform knots, so
the projective bound is infinite there.

**(B) Estimated: `conditioned_split`'s first-order estimate** (its
final worst point, times `u/2`, times the largest control
coordinate). It tracks the real disagreement to within 10–10³:

| shape | estimate / scale (worst) | real / scale (worst) | estimate / real, median / max |
|---|---|---|---|
| p = 2, 3 | 1–6·10⁻¹⁶ | 4–6·10⁻¹⁶ | 0.4–1.6 / 3 |
| p = 5 | 2·10⁻¹⁵ | 6·10⁻¹⁶ | 5–8 / 12 |
| p = 8 | 7·10⁻¹⁴ | 3·10⁻¹⁵ | 13–85 / 190 |
| p = 10, uniform | 3.5·10⁻¹² | 2·10⁻¹⁴ | 400–760 / 1.2·10³ |
| p = 11, 50 knots | 1.2–4.2·10⁻¹⁰ | 0.7–7·10⁻¹² | 110–250 / 540 |
| p = 12, 50 knots | 0.7–2.3·10⁻⁹ | 0.4–3.7·10⁻¹¹ | 110–550 / 690 |
| p = 16–25, clusters | 0.8·10⁻¹²–3·10⁻¹⁰ | 3·10⁻¹⁴–5·10⁻¹² | 30–120 / 680 |

Below a ratio of 1 (p = 2, 3) the real figure is the evaluation's
own rounding, not elevation error. On no shape was the estimate below
the real disagreement past that floor.

**An ordinary compose leg** (a quarter arc or full circle 2 → 3, a
line 1 → 3) at ε `1e-12`:
- under (A), it is inside ε to about 100 m from the origin, and
  outside it at 1 km (bound `2.4·10⁻¹²` m, real `1.1·10⁻¹³` m);
- under (B), it is inside ε at every offset to 1 km (estimate at most
  `1.2·10⁻¹³` m).

At `10⁵` m the real disagreement is the coordinates' ulp
(`1.5·10⁻¹¹` m), past what ε `1e-12` resolves under either form.

**Ruled (NURBS orchestrator):** (B) is the bound's form, worded
"estimated", never "certified". Its rows pin estimate ≥ real over the
envelope shapes, against their ratio floor.

**Open (design).** Should `compose_chain` and `skin` refuse on the
estimate or carry it, and how does `skin` get a `Tol`?
- Refusing on the estimate refuses honest p ≥ 12 legs at the default
  ε: about `10⁻⁸` m estimated against `4·10⁻¹⁰` m real on ±10
  coordinates.
- Carrying it has no reader today.
- `make_compatible` takes no `Tol`. `loft_geometry` holds one.

It goes to a designer pair when a caller needs it or the program
reaches it.
