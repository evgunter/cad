//! NURBS curves — [`NurbsCurve2`] (the future pcurve substrate; not
//! wired into any enum) and [`NurbsCurve3`] (the [`crate::Curve3::Nurbs`]
//! payload), M5 PR 3.
//!
//! # Data model (binding conventions, `docs/M5-PR3-SPEC.md`)
//!
//! Knots, weights, and degree are **f64 structure** (C6); control
//! points are the only generically-typed data. Construction is
//! validated and typed-error: **clamped-v1** knot vectors
//! ([`geom_core::spline::KnotVector`] carries the exact invariants),
//! **strictly positive weights** (the convex-hull property every C9
//! hull bound stands on), and count coherence. Unlike the analytic
//! variants' public conventional fields, these invariants are
//! *load-bearing for indexing safety*, so fields are private and
//! construction goes through [`NurbsCurve3::new`] /
//! [`NurbsCurve2::new`].
//!
//! # Evaluation contract
//!
//! - **Core, generic** (`*_in_span`): rational evaluation restricted to
//!   a caller-supplied span — ring ops + `from_f64` only, total for
//!   every scalar. For `t` outside the span's knot interval the result
//!   is the span's **polynomial extension** (documented garbage-out —
//!   detecting it would need the comparison [`Real`] deliberately
//!   lacks); an invalid span *index* (checkable structure) is poison.
//! - **Full evaluators** (`eval`/`deriv`/`deriv2`): span selection via
//!   the sealed [`SpanLocate`] seam (per-instantiation semantics
//!   documented in `geom_core::spline::locate`), then the core per
//!   overlapped span, hulled channel-independently for interval-natured
//!   scalars.
//!
//! # Fixed association (D9)
//!
//! Homogeneous combination is a **single ascending-index pass**: for
//! each basis index `j` (ascending), `cw = N_j · from_f64(w_j)`, then
//! each homogeneous accumulator adds its term in the written order
//! (`acc = acc + cw · coord`), followed by **one division per
//! coordinate** by the accumulated weight. Rational derivative
//! corrections are evaluated exactly as parenthesized at each method.
//! Knot-algebra point combinations are `lerp(x, y, λ) = x + (y − x)·λ`
//! with `λ` lifted once per combination.

use geom_core::spline::{self, KnotAlgebraError, KnotVector, SpanLocate, SplineError};
use geom_core::{Bounds, Point2, Point3, Real, RingInterval, Vec2, Vec3};

/// Shared constructor validation: counts and weight positivity/
/// finiteness (the knot vector validates itself at construction).
// `!(w > 0)` is deliberate (NaN-catching — NaN refuses; `w <= 0`
// would pass NaN). Same note as geom-core::spline::algebra.
#[allow(clippy::neg_cmp_op_on_partial_ord)]
fn validate_counts(knots: &KnotVector, control: usize, weights: &[f64]) -> Result<(), SplineError> {
    if control != knots.control_count() {
        return Err(SplineError::ControlCountMismatch {
            control,
            expected: knots.control_count(),
        });
    }
    if weights.len() != control {
        return Err(SplineError::WeightCountMismatch {
            weights: weights.len(),
            control,
        });
    }
    for (index, w) in weights.iter().enumerate() {
        if !(*w > 0.0) {
            return Err(SplineError::NonPositiveWeight { index, weight: *w });
        }
        if !w.is_finite() {
            return Err(SplineError::NonFiniteWeight { index, weight: *w });
        }
    }
    Ok(())
}

/// The rational speed meter's **fixed refinement schedule** (D9: a
/// structure choice, never a decision): before the per-span scan of
/// `speed_lower_bound`'s rational arm, every nonempty span is split
/// into this many equal pieces by knot insertion. Knot insertion is
/// evaluation-invariant, so the curve is unchanged; only the hulls the
/// bound is assembled from shrink. The constant is a measured
/// trade-off, not a tuning knob — see the `rational_speed_lower_bound`
/// docs and the adversarial rows in `tests/m5_pr7_speed_meter.rs`.
const RATIONAL_METER_SPLITS: usize = 16;

macro_rules! nurbs_curve {
    ($Curve:ident, $Point:ident, $Vector:ident, $($c:ident),+) => {
        /// A validated NURBS curve (module docs: data model, evaluation
        /// contract, fixed association orders). Immutable after
        /// construction; every knot-algebra operation returns a new
        /// curve (D9-clean value semantics).
        #[derive(Clone, Debug)]
        pub struct $Curve<T: Real> {
            knots: KnotVector,
            control: Vec<$Point<T>>,
            weights: Vec<f64>,
        }

        impl<T: Real> $Curve<T> {
            /// Validated construction (module docs for the invariants).
            ///
            /// # Errors
            ///
            /// [`SplineError`] naming the exact violation: count
            /// mismatches or a non-positive/non-finite weight. (Knot
            /// vector violations are refused earlier, by
            /// [`KnotVector::clamped`].)
            pub fn new(
                knots: KnotVector,
                control: Vec<$Point<T>>,
                weights: Vec<f64>,
            ) -> Result<Self, SplineError> {
                validate_counts(&knots, control.len(), &weights)?;
                Ok(Self { knots, control, weights })
            }

            /// The validated clamped knot vector.
            pub fn knots(&self) -> &KnotVector {
                &self.knots
            }

            /// The control points.
            pub fn control(&self) -> &[$Point<T>] {
                &self.control
            }

            /// The weights (strictly positive, f64 structure).
            pub fn weights(&self) -> &[f64] {
                &self.weights
            }

            /// The degree `p` (carried by the knot vector).
            pub fn degree(&self) -> usize {
                self.knots.degree()
            }

            /// The parameter domain (first knot at multiplicity
            /// `p + 1` to last).
            pub fn domain(&self) -> (f64, f64) {
                self.knots.domain()
            }

            /// The all-poison point (every coordinate the scalar's
            /// poison) — the total outcome for invalid span indices.
            fn poison_point() -> $Point<T> {
                let nan = T::from_f64(f64::NAN);
                $Point::new($({ let _ = stringify!($c); nan }),+)
            }

            /// The all-poison vector.
            fn poison_vector() -> $Vector<T> {
                let nan = T::from_f64(f64::NAN);
                $Vector::new($({ let _ = stringify!($c); nan }),+)
            }

            /// The point at `t`, evaluated **in the given span** — the
            /// generic core (module docs: the span contract; the fixed
            /// single-ascending-pass association). Total: an invalid
            /// span index yields the all-poison point; `t` outside the
            /// span's interval yields the span's polynomial extension.
            pub fn eval_in_span(&self, span: usize, t: T) -> $Point<T> {
                if span < self.knots.first_span()
                    || span > self.knots.last_span()
                    || !self.knots.span_is_nonempty(span)
                {
                    return Self::poison_point();
                }
                let p = self.knots.degree();
                let basis = spline::basis::basis_funs(&self.knots, span, t);
                $(let mut $c = T::zero();)+
                let mut w_acc = T::zero();
                for (j, nj) in basis.iter().enumerate() {
                    // Indexing justified: span valid ⇒ i = span − p + j
                    // ∈ [span − p, span] ⊆ [0, control_count).
                    let i = span - p + j;
                    let cw = *nj * T::from_f64(self.weights[i]);
                    let pt = self.control[i];
                    $($c = $c + cw * pt.$c;)+
                    w_acc = w_acc + cw;
                }
                $Point::new($($c / w_acc),+)
            }

            /// Point, first, and second derivative at `t` in the given
            /// span — one homogeneous pass (orders 0..=2 of the basis),
            /// then the rational corrections, exactly as written:
            /// `C = N⁰/w⁰`, `C′ = (N¹ − C·w¹)/w⁰`,
            /// `C″ = (N² − C·w² − C′·w¹·2)/w⁰`.
            /// Same totality contract as [`Self::eval_in_span`].
            pub fn ders_in_span(&self, span: usize, t: T) -> ($Point<T>, $Vector<T>, $Vector<T>) {
                if span < self.knots.first_span()
                    || span > self.knots.last_span()
                    || !self.knots.span_is_nonempty(span)
                {
                    return (Self::poison_point(), Self::poison_vector(), Self::poison_vector());
                }
                let p = self.knots.degree();
                let ders = spline::basis::ders_basis_funs(&self.knots, span, t, 2);
                $(let mut $c = [T::zero(), T::zero(), T::zero()];)+
                let mut w_hom = [T::zero(), T::zero(), T::zero()];
                for (k, row) in ders.iter().enumerate() {
                    for (j, nkj) in row.iter().enumerate() {
                        // Indexing justified as in `eval_in_span`.
                        let i = span - p + j;
                        let cw = *nkj * T::from_f64(self.weights[i]);
                        let pt = self.control[i];
                        $($c[k] = $c[k] + cw * pt.$c;)+
                        w_hom[k] = w_hom[k] + cw;
                    }
                }
                let two = T::from_f64(2.0);
                $(let $c = {
                    let hom = $c;
                    let c0 = hom[0] / w_hom[0];
                    let c1 = (hom[1] - c0 * w_hom[1]) / w_hom[0];
                    let c2 = (hom[2] - c0 * w_hom[2] - c1 * w_hom[1] * two) / w_hom[0];
                    (c0, c1, c2)
                };)+
                (
                    $Point::new($($c.0),+),
                    $Vector::new($($c.1),+),
                    $Vector::new($($c.2),+),
                )
            }

            /// First derivative in the given span (the middle component
            /// of [`Self::ders_in_span`]).
            pub fn deriv_in_span(&self, span: usize, t: T) -> $Vector<T> {
                self.ders_in_span(span, t).1
            }

            /// Second derivative in the given span (the last component
            /// of [`Self::ders_in_span`]).
            pub fn deriv2_in_span(&self, span: usize, t: T) -> $Vector<T> {
                self.ders_in_span(span, t).2
            }

            /// Applies a chain of structure plans to this curve's
            /// control polygon (points via `lerp(x, y, from_f64(λ))`,
            /// the fixed association; knots/weights from the final
            /// plan). Empty chain ⇒ clone.
            fn apply_plans(&self, plans: &[spline::CurvePlan]) -> Self {
                let mut control = self.control.clone();
                for plan in plans {
                    control = plan.apply_points(&control, Self::poison_point(), |x, y, l| {
                        x.lerp(y, T::from_f64(l))
                    });
                }
                match plans.last() {
                    Some(last) => Self {
                        knots: last.knots().clone(),
                        control,
                        weights: last.weights().to_vec(),
                    },
                    None => self.clone(),
                }
            }

            /// Knot insertion (§5.2), single value `times`-fold — the
            /// future `split_edge` substrate. Evaluation-invariant in ℝ.
            ///
            /// # Errors
            ///
            /// [`KnotAlgebraError`]: out-of-domain `u`, interior
            /// multiplicity budget (`degree`) exceeded, or structure
            /// mismatch.
            pub fn insert_knot(&self, u: f64, times: usize) -> Result<Self, KnotAlgebraError> {
                let plans = spline::algebra::insert_knot_plan(&self.knots, &self.weights, u, times)?;
                Ok(self.apply_plans(&plans))
            }

            /// Splits the curve at interior parameter `u` into two
            /// clamped curves covering `[t₀, u]` and `[u, t₁]` — knot
            /// insertion to full interior multiplicity, then the
            /// control/knot partition (§5.2; C12.3: the NURBS
            /// `split_edge` substrate, M5 PR 9). Evaluation-invariant
            /// in ℝ, and each child keeps the PARENT's parameter (the
            /// split value is not re-normalized), so a caller's
            /// `[t₀, u]` interval on the child means exactly what it
            /// meant on the parent.
            ///
            /// # Errors
            ///
            /// [`KnotAlgebraError`]: `u` outside the open knot domain
            /// (boundary splits are refused — a clamped end already
            /// has full multiplicity and an empty child is not a
            /// curve), or the insertion path's structure refusals.
            pub fn split_at(&self, u: f64) -> Result<(Self, Self), KnotAlgebraError> {
                let p = self.knots.degree();
                let (d0, d1) = self.knots.domain();
                if !u.is_finite() || !(u > d0 && u < d1) {
                    return Err(KnotAlgebraError::ParameterOutsideDomain { u });
                }
                let have = self.knots.multiplicity_of(u).map_or(0, |(m, _)| m);
                let full = if have < p {
                    self.insert_knot(u, p - have)?
                } else {
                    self.clone()
                };
                // First occurrence of `u` in the saturated vector
                // (multiplicity is exactly `p` there). Present by
                // construction; `ok_or` keeps the path total.
                let knots = full.knots.knots();
                let f = knots
                    .iter()
                    .position(|k| *k == u)
                    .ok_or(KnotAlgebraError::KnotNotPresent { u })?;
                // Child 1: every knot below `u` plus `p + 1` copies of
                // `u`; control points 0..f (C(u) is control index
                // f − 1 of the saturated curve, shared by both).
                let mut k1: Vec<f64> = knots[..f + p].to_vec();
                k1.push(u);
                let c1 = Self::new(
                    KnotVector::clamped(k1, p).map_err(KnotAlgebraError::Structure)?,
                    full.control[..f].to_vec(),
                    full.weights[..f].to_vec(),
                )
                .map_err(KnotAlgebraError::Structure)?;
                // Child 2: one extra copy of `u`, then every knot from
                // index f on; control points f − 1 onward.
                let mut k2: Vec<f64> = vec![u];
                k2.extend_from_slice(&knots[f..]);
                let c2 = Self::new(
                    KnotVector::clamped(k2, p).map_err(KnotAlgebraError::Structure)?,
                    full.control[f - 1..].to_vec(),
                    full.weights[f - 1..].to_vec(),
                )
                .map_err(KnotAlgebraError::Structure)?;
                Ok((c1, c2))
            }

            /// Knot refinement (§5.3): inserts every value of `add`
            /// (ascending, ties consecutive). Evaluation-invariant in ℝ.
            ///
            /// # Errors
            ///
            /// As [`Self::insert_knot`], against the cumulative
            /// structure.
            pub fn refine_knots(&self, add: &[f64]) -> Result<Self, KnotAlgebraError> {
                let plans = spline::algebra::refine_plan(&self.knots, &self.weights, add)?;
                Ok(self.apply_plans(&plans))
            }

            /// Bounded knot removal (§5.4): removes `times` copies of
            /// the interior knot `u` and returns the rewritten curve
            /// **with a sup-norm error bound** `B` such that
            /// `|C(t) − Ĉ(t)| ≤ B` over the whole domain — removal is
            /// bounded, never silent. The bound is the Eq. 9.81
            /// mechanism in projective form: each pass reinserts the
            /// removed copy exactly and bounds the polygon perturbation
            /// through partition of unity and the positive-weight
            /// convex hull —
            /// `B_pass = (Cmax·Bw + Bwp) / w̃min`, where `Cmax` bounds
            /// `|C|` by the control hull, `Bw`/`Bwp` are the max weight
            /// and weighted-point perturbations, and `w̃min` lower-bounds
            /// the reinserted weight function; passes add (triangle
            /// inequality). Conservative: the per-basis sup factor is
            /// relaxed to 1. At interval scalars the bound is itself an
            /// enclosure (containment composes).
            ///
            /// # Errors
            ///
            /// [`KnotAlgebraError`]: `u` not an interior knot (exact
            /// f64 identity), removing past its multiplicity, or a
            /// weight collapsing out of the positive regime.
            pub fn remove_knot(&self, u: f64, times: usize) -> Result<(Self, T), KnotAlgebraError> {
                let steps = spline::algebra::remove_knot_plan(&self.knots, &self.weights, u, times)?;
                let mut cur = self.clone();
                let mut bound = T::zero();
                for step in &steps {
                    let removed = cur.apply_plans(core::slice::from_ref(&step.plan));
                    let reinserted = removed.apply_plans(core::slice::from_ref(&step.reinsert));
                    bound = bound + Self::removal_pass_bound(&cur, &reinserted);
                    cur = removed;
                }
                Ok((cur, bound))
            }

            /// A certified sup-norm bound on `|C_self − C_other|` for
            /// two curves **sharing one knot vector** (same degree,
            /// same control count; weights may differ): the
            /// [`Self::removal_pass_bound`] formula, which only uses
            /// that sharing — `(Cmax·Bw + Bwp)/w̃min` through partition
            /// of unity and the positive-weight convex hull. Poison
            /// (NaN) when the structures do not match — total, never
            /// a fabricated bound. Crate-internal: the fitting stack's
            /// deviation measurements (M5 PR 4) ride it.
            /// A **certified lower bound** on `‖C′(t)‖` over the whole
            /// domain, in meters per parameter unit — the "meter" a
            /// parameter-space margin must be multiplied by to become a
            /// length (D4 ¶1).
            ///
            /// This is the rung-3 analogue of the conic lane's
            /// conservative meters (`Circle` ⇒ radius, `Ellipse` ⇒ the
            /// MINOR semi-axis): without it, a fitted SSI carrier
            /// reaching `split_edge` has no honest way to state
            /// "definitely interior in meters", and the parameter gate
            /// would either poison or, far worse, use an *upper* bound
            /// and accept a split that is not clear of the endpoints.
            ///
            /// # How it is certified
            ///
            /// The derivative of a clamped B-spline is a B-spline of
            /// degree `p − 1` over the derivative control points
            /// `Qᵢ = p·(Pᵢ₊₁ − Pᵢ)/(uᵢ₊ₚ₊₁ − uᵢ₊₁)`, so on every span
            /// `C′(t)` is a **convex combination** of the local `Qᵢ`.
            /// Fix any unit direction `d`: then
            /// `‖C′‖ ≥ d·C′ ≥ minᵢ (d·Qᵢ)`. The direction taken is the
            /// curve's own chord (first to last control point), which
            /// is the one a monotone carrier advances along. The result
            /// may be zero or negative — this arm's single global
            /// direction bounds nothing when the curve turns away from
            /// its own chord — and that is reported honestly, so the
            /// margin collapses and the caller's trilean escalates
            /// rather than guessing.
            ///
            /// **Rational carriers** take the second arm,
            /// [`Self::rational_speed_lower_bound`] — the derivative of
            /// a rational spline is *not* a convex combination of any
            /// control net, so the argument above does not apply
            /// directly and a quotient-rule assembly stands in. The
            /// contract (whole domain, m/param, honestly non-positive
            /// when the curve gives the assembly nothing to stand on,
            /// poison when the structure refuses) is the same on both
            /// arms.
            ///
            /// # What the bound does and does not certify
            ///
            /// It certifies **speed**, and through speed, **arc
            /// length**: over any parameter interval `[a, b]` inside
            /// the domain, `(b − a)·bound ≤ ∫ₐᵇ ‖C′‖`. That is exactly
            /// what every consumer asks of it — `interval_span_forward`
            /// converts a parameter span to metres of arc,
            /// `split_edge_param_interior` converts a distance-to-
            /// endpoint the same way.
            ///
            /// It certifies **nothing about injectivity, turning, or
            /// monotone advance along any direction**. A carrier may
            /// reverse, loop, or return arbitrarily close to a point it
            /// has already visited and still meter positively, provided
            /// its speed never collapses — reversal is not
            /// disqualifying, only a genuine stationary point (a cusp,
            /// a turn-around, a degenerate span) is. Callers that need
            /// non-self-intersection or a bounded turn must obtain it
            /// elsewhere; this number will not supply it, and reading it
            /// as if it did would be reading an arc-length rate as a
            /// chord-distance rate.
            ///
            /// The two arms differ only in how conservative they are
            /// about that: the integral arm projects on ONE global
            /// direction and therefore also happens to collapse on a
            /// curve that turns away from its chord, while the rational
            /// arm projects per span and does not. Both are sound lower
            /// bounds on `‖C′‖`; neither is a claim about the curve's
            /// shape beyond its speed.
            ///
            /// The arm is chosen on **f64 structure** (`w_j == 1.0`
            /// exactly), never on an evaluation scalar, so the integral
            /// path is bit-for-bit what it always was.
            pub fn speed_lower_bound(&self) -> T {
                let poison = T::from_f64(f64::NAN);
                // Rational ⇒ the convexity argument does not hold
                // directly; the quotient-rule arm takes over.
                if self.weights.iter().any(|w| *w != 1.0) {
                    return self.rational_speed_lower_bound();
                }
                let p = self.knots.degree();
                if p == 0 || self.control.len() < 2 {
                    return poison;
                }
                let knots = self.knots.knots();
                // The chord direction (first to last control point);
                // a clamped curve interpolates both, so this is the
                // curve's own end-to-end direction.
                let Some((first, last)) = self.control.first().zip(self.control.last()) else {
                    return poison;
                };
                let chord = *last - *first;
                let n = chord.norm();
                let d = chord / n;
                let mut acc: Option<T> = None;
                for i in 0..(self.control.len() - 1) {
                    let (Some(a), Some(b)) = (self.control.get(i), self.control.get(i + 1)) else {
                        return poison;
                    };
                    let (Some(&lo), Some(&hi)) = (knots.get(i + 1), knots.get(i + p + 1)) else {
                        return poison;
                    };
                    #[allow(clippy::cast_precision_loss)]
                    let scale = T::from_f64(p as f64) / T::from_f64(hi - lo);
                    let q = (*b - *a) * scale;
                    let v = d.dot(q);
                    acc = Some(match acc {
                        None => v,
                        // `Real::min` is NaN-propagating (poison in,
                        // poison out) and total — no comparison here.
                        Some(m) => m.min(v),
                    });
                }
                acc.unwrap_or(poison)
            }

            /// The **rational arm** of [`Self::speed_lower_bound`]: a
            /// certified lower bound on `‖C′(t)‖` over the whole domain
            /// for a carrier with non-unit weights, in m/param.
            ///
            /// # The bound (the invariant this function computes)
            ///
            /// Write `C = A/w` with `A = Σ N_j w_j P_j` and
            /// `w = Σ N_j w_j`. For any fixed point `c`, the translate
            /// `Ã = A − c·w` is the B-spline with coefficients
            /// `a_j = w_j·(P_j − c)`, `C − c = Ã/w`, and the quotient
            /// rule gives the identity everything here rests on:
            ///
            /// ```text
            /// C′ = (Ã′ − (C − c)·w′) / w
            /// ```
            ///
            /// Fix a **unit** direction `d`. Then `‖C′‖ ≥ d·C′` and
            ///
            /// ```text
            /// d·C′  ≥  ( min_i (d·Q_i)  −  sup|C − c|·sup|w′| ) / w
            /// ```
            ///
            /// where `Q_i = p·(a_{i+1} − a_i)/(u_{i+p+1} − u_{i+1})` are
            /// `Ã′`'s coefficients (the knot-difference formula of
            /// `geom_core::spline::hull::derivative_coeffs`, applied to
            /// the HOMOGENEOUS coefficients — hull.rs deliberately has
            /// no rational derivative path, because this assembly
            /// belongs with the consumer that owns the homogeneous
            /// form). Each ingredient is a hull over the coefficients
            /// active on the span, licensed by the same convexity fact
            /// as the integral arm plus **strictly positive weights**
            /// (checked here, poison otherwise — without it neither the
            /// rational basis nor `w`'s own hull is a convex
            /// combination):
            ///
            /// - `min_i (d·Q_i)` over the active `Q`, since `Ã′` is a
            ///   degree-`p−1` B-spline in the `Q_i`;
            /// - `sup|C − c| ≤ max_j ‖P_j − c‖` — `C − c` is a convex
            ///   combination of the `P_j − c` (positive weights) and
            ///   the norm is convex;
            /// - `sup|w′| ≤ max_i |q_i|` over the weight spline's own
            ///   derivative coefficients, taken through
            ///   [`spline::hull::derivative_coeffs`] so the knot
            ///   difference is rounded in the ring, not at `f64`.
            ///
            /// **The denominator is `w_max`, not `w_min`.** `w` itself
            /// is a convex combination of the active weights, so
            /// `w ∈ [w_min, w_max]`. For a *non-negative* numerator the
            /// conservative division is by the LARGEST denominator; for
            /// a negative one it is by the smallest. (The opposite
            /// choice — dividing by the min-weight floor, as
            /// `geom_surfaces::recognize`'s conic-derivative work does
            /// — is the direction for an UPPER bound on the derivative,
            /// and would be unsound here.) Which case applies is a
            /// question about a `Real`, which this code may not ask, so
            /// it takes the **lattice min of both divisions**: that is
            /// `L/w_max` exactly when `L ≥ 0` and `L/w_min` exactly
            /// when `L < 0`, with no comparison and no branch.
            ///
            /// # Schedule (D9: structure, never a decision)
            ///
            /// The above is evaluated **per nonempty span** — active
            /// coefficients only, with the span's own control centroid
            /// as `c` and the span's own control chord
            /// `P_span − P_{span−p}` as `d` — and the whole-domain
            /// answer is the ascending `Real::min` fold over spans.
            /// This is a fixed constant schedule read off the knot
            /// vector (f64 structure), and it is what keeps the
            /// `sup|C − c|` term span-sized rather than curve-sized: a
            /// per-span direction is legitimate because
            /// `‖C′(t)‖ ≥ d_s·C′(t)` holds for *every* unit `d_s`, so
            /// the min over spans of per-span bounds still bounds the
            /// whole domain. The integral arm keeps its single global
            /// chord, bit-identically.
            ///
            /// A per-span direction bounds SPEED and nothing else — see
            /// [`Self::speed_lower_bound`]'s "what the bound does and
            /// does not certify". Successive spans may point anywhere,
            /// so this arm meters a carrier that reverses, as it should:
            /// only a genuine stationary point drives the answer
            /// non-positive.
            ///
            /// # Poison (total, D4 ¶2)
            ///
            /// A zero degree, fewer than two control points, any
            /// non-positive or non-finite weight, a non-positive knot
            /// difference, a span whose control chord collapses, or a
            /// knot vector with no nonempty span — every one yields
            /// NaN. A bound is never fabricated.
            ///
            /// # Rounding posture
            ///
            /// At `f64` the assembly runs in nearest rounding, like
            /// every other `Real`-generic bound in the kernel: the
            /// weight-derivative hulls come through the ring (correctly
            /// rounded), but the chord normalisation and the hull folds
            /// do not, so the `f64` reading is a bound only up to about
            /// a relative ulp. **The `Interval` instantiation is the
            /// certified lane** — it encloses the same expression, and
            /// `tests/m5_pr7_speed_meter.rs`'s bracket row pins that
            /// the interval answer contains the `f64` one. This is the
            /// kernel-wide posture, not a property of this bound.
            ///
            /// # Conservatism
            ///
            /// The answer is a bound, not an estimate, and the gap can
            /// be wide. It measures at 0.86–0.97 of the true minimum on
            /// ordinary and adversarial carriers, but a curve that
            /// turns hard, or a high degree with alternating extreme
            /// weights, can refuse outright while its true speed is
            /// comfortably positive. Refusal is always sound and only
            /// ever a usability cost; the frontier rows in
            /// `tests/m5_pr7_speed_meter.rs` pin where it currently
            /// falls, so [`RATIONAL_METER_SPLITS`] cannot be changed
            /// without the trade-off becoming visible.
            fn rational_speed_lower_bound(&self) -> T {
                let poison = T::from_f64(f64::NAN);
                let p = self.knots.degree();
                if p == 0 || self.control.len() < 2 {
                    return poison;
                }
                // The convex-combination licence, re-checked here on
                // f64 STRUCTURE (never on an evaluation scalar):
                // `!(w > 0.0)` catches NaN too.
                #[allow(clippy::neg_cmp_op_on_partial_ord)]
                if self.weights.iter().any(|w| !(*w > 0.0) || !w.is_finite()) {
                    return poison;
                }
                // The refinement schedule (D9 structure, a fixed
                // constant): every nonempty span is split into
                // `RATIONAL_METER_SPLITS` equal pieces before the scan.
                // Knot insertion is evaluation-invariant, so this
                // changes no geometry — it only shrinks every hull the
                // bound is assembled from, which is what buys a
                // POSITIVE answer on steep weight ratios where the
                // one-span assembly is dominated by `sup‖C − c‖·sup|w′|`.
                let mut add = Vec::new();
                for span in self.knots.first_span()..=self.knots.last_span() {
                    if !self.knots.span_is_nonempty(span) {
                        continue;
                    }
                    let (Some(&lo), Some(&hi)) =
                        (self.knots.knots().get(span), self.knots.knots().get(span + 1))
                    else {
                        return poison;
                    };
                    for k in 1..RATIONAL_METER_SPLITS {
                        #[allow(clippy::cast_precision_loss)]
                        let f = k as f64 / RATIONAL_METER_SPLITS as f64;
                        let u = lo + (hi - lo) * f;
                        // Skip a split point that floating point has
                        // collapsed onto a span end — refinement is a
                        // tightening, never a correctness condition.
                        if u > lo && u < hi {
                            add.push(u);
                        }
                    }
                }
                let Ok(refined) = self.refine_knots(&add) else {
                    return poison;
                };
                refined.rational_span_scan()
            }

            /// The per-span scan of [`Self::rational_speed_lower_bound`],
            /// run on the refined curve: the ascending `Real::min` fold
            /// of [`Self::rational_span_bound`] over nonempty spans.
            fn rational_span_scan(&self) -> T {
                let poison = T::from_f64(f64::NAN);
                let p = self.knots.degree();
                // Re-checked on the REFINED weights: knot insertion
                // keeps positivity in ℝ, and this function may not
                // assume floating point did.
                #[allow(clippy::neg_cmp_op_on_partial_ord)]
                if p == 0 || self.weights.iter().any(|w| !(*w > 0.0) || !w.is_finite()) {
                    return poison;
                }
                let knots = self.knots.knots();
                // `w′`'s coefficient enclosures, once for the curve:
                // index `i` holds `q_i`, poison for a bad knot
                // difference (which then poisons this bound).
                let dw = spline::hull::derivative_coeffs(&self.knots, &self.weights);
                let origin = $Point::new($({ let _ = stringify!($c); T::zero() }),+);
                let mut acc: Option<T> = None;
                // Fixed ascending span order (D9).
                for span in self.knots.first_span()..=self.knots.last_span() {
                    if !self.knots.span_is_nonempty(span) {
                        continue;
                    }
                    let (Some(first), last) = (span.checked_sub(p), span) else {
                        return poison;
                    };
                    if last >= self.control.len() {
                        return poison;
                    }
                    let b = self.rational_span_bound(knots, &dw, origin, first, last);
                    acc = Some(match acc {
                        None => b,
                        // NaN-propagating lattice fold — poison in,
                        // poison out, no comparison.
                        Some(m) => m.min(b),
                    });
                }
                acc.unwrap_or(poison)
            }

            /// One span's arm of [`Self::rational_speed_lower_bound`]
            /// (the derivation lives there). `first ..= last` are the
            /// coefficient indices active on the span, already range-
            /// checked; `dw` holds the weight spline's derivative
            /// coefficient enclosures.
            fn rational_span_bound(
                &self,
                knots: &[f64],
                dw: &[RingInterval],
                origin: $Point<T>,
                first: usize,
                last: usize,
            ) -> T {
                let poison = T::from_f64(f64::NAN);
                let p = self.knots.degree();
                let Some(active) = self.control.get(first..=last) else {
                    return poison;
                };
                #[allow(clippy::cast_precision_loss)]
                let count = T::from_f64(active.len() as f64);
                // The span's own control centroid — the translation
                // that keeps `sup‖C − c‖` span-sized.
                let mut sum = origin - origin;
                for pt in active {
                    sum = sum + (*pt - origin);
                }
                let c = origin + sum / count;
                // The span's control chord, as unit direction.
                let (Some(a), Some(b)) = (active.first(), active.last()) else {
                    return poison;
                };
                let chord = *b - *a;
                let d = chord / chord.norm();
                // The SIGNED hull of `d·(C − c)` on the span — the
                // rational value hull (`hull::span_hull_rational`'s
                // fact: positive weights make the rational basis a
                // nonnegative partition of unity) read through `d`.
                // Ascending `Real::min`/`Real::max` folds.
                let mut s_lo: Option<T> = None;
                let mut s_hi: Option<T> = None;
                for pt in active {
                    let s = d.dot(*pt - c);
                    s_lo = Some(match s_lo {
                        None => s,
                        Some(m) => m.min(s),
                    });
                    s_hi = Some(match s_hi {
                        None => s,
                        Some(m) => m.max(s),
                    });
                }
                let (Some(s_lo), Some(s_hi)) = (s_lo, s_hi) else {
                    return poison;
                };
                // `w`'s hull on the span (f64 structure comparisons on
                // f64 weights — the `removal_pass_bound` precedent).
                let Some(w_active) = self.weights.get(first..=last) else {
                    return poison;
                };
                let mut w_min = f64::INFINITY;
                let mut w_max = 0.0f64;
                for w in w_active {
                    if *w < w_min {
                        w_min = *w;
                    }
                    if *w > w_max {
                        w_max = *w;
                    }
                }
                // The numerator's two terms over the active derivative
                // indices `[first, last)`.
                let mut num: Option<T> = None;
                let (mut wp_lo, mut wp_hi) = (f64::INFINITY, f64::NEG_INFINITY);
                for i in first..last {
                    let (Some(&lo), Some(&hi)) = (knots.get(i + 1), knots.get(i + p + 1)) else {
                        return poison;
                    };
                    let du = hi - lo;
                    #[allow(clippy::neg_cmp_op_on_partial_ord)]
                    if !(du > 0.0) {
                        return poison;
                    }
                    let (Some(pi), Some(pj)) = (self.control.get(i), self.control.get(i + 1))
                    else {
                        return poison;
                    };
                    let (Some(&wi), Some(&wj)) = (self.weights.get(i), self.weights.get(i + 1))
                    else {
                        return poison;
                    };
                    #[allow(clippy::cast_precision_loss)]
                    let scale = T::from_f64(p as f64) / T::from_f64(du);
                    // Homogeneous, centroid-translated: a_j = w_j·(P_j − c).
                    let ai = (*pi - c) * T::from_f64(wi);
                    let aj = (*pj - c) * T::from_f64(wj);
                    let v = d.dot((aj - ai) * scale);
                    num = Some(match num {
                        None => v,
                        Some(m) => m.min(v),
                    });
                    // `w′`'s SIGNED hull, from the ring-rounded
                    // coefficients (`!(a >= b)` so a poisoned
                    // coefficient poisons the hull rather than being
                    // skipped by a false comparison).
                    let Some(q) = dw.get(i) else {
                        return poison;
                    };
                    #[allow(clippy::neg_cmp_op_on_partial_ord)]
                    if !(q.lo() >= wp_lo) {
                        wp_lo = q.lo();
                    }
                    #[allow(clippy::neg_cmp_op_on_partial_ord)]
                    if !(q.hi() <= wp_hi) {
                        wp_hi = q.hi();
                    }
                }
                let Some(num) = num else {
                    return poison;
                };
                // `sup (d·(C − c))·w′` over the two signed hulls: the
                // ascending `Real::max` fold of the four corner
                // products (a magnitude product `sup|·|·sup|·|` would
                // be sound but needlessly loose — it throws away the
                // sign correlation that steep weight ramps live in).
                let (lo, hi) = (T::from_f64(wp_lo), T::from_f64(wp_hi));
                let corner = (s_lo * lo).max(s_lo * hi).max(s_hi * lo).max(s_hi * hi);
                let l = num - corner;
                // `min(L/w_max, L/w_min)` — the correct division in
                // both numerator signs, without asking the sign.
                (l / T::from_f64(w_max)).min(l / T::from_f64(w_min))
            }

            pub(crate) fn same_structure_deviation_bound(&self, other: &Self) -> T {
                if self.knots != other.knots || self.control.len() != other.control.len() {
                    return T::from_f64(f64::NAN);
                }
                Self::removal_pass_bound(self, other)
            }

            /// One removal pass's projected perturbation bound
            /// (derivation at [`Self::remove_knot`]); `orig` and `re`
            /// share a knot vector by construction. Reductions are
            /// ascending-index `Real::max` folds (lattice value ops,
            /// never control flow).
            fn removal_pass_bound(orig: &Self, re: &Self) -> T {
                let origin = $Point::new($({ let _ = stringify!($c); T::zero() }),+);
                let mut c_max = T::zero();
                let mut bwp = T::zero();
                let mut bw = 0.0f64;
                let mut w_min = f64::INFINITY;
                for (i, pt) in orig.control.iter().enumerate() {
                    c_max = c_max.max((*pt - origin).norm());
                    // Indexing justified: `re` has the same structure
                    // (reinsertion restores the original knot vector,
                    // hence the original control count).
                    let (rp, rw) = (re.control[i], re.weights[i]);
                    let dw = (orig.weights[i] - rw).abs();
                    if dw > bw {
                        bw = dw;
                    }
                    if rw < w_min {
                        w_min = rw;
                    }
                    let d = (*pt - origin) * T::from_f64(orig.weights[i])
                        - (rp - origin) * T::from_f64(rw);
                    bwp = bwp.max(d.norm());
                }
                (c_max * T::from_f64(bw) + bwp) * T::from_f64(1.0 / w_min)
            }

            /// Degree elevation (§5.5) by `raise` (≥ 1), via the Bézier
            /// route (`geom_core::spline::algebra::elevate_plan`):
            /// decompose, elevate each segment binomially, recompose
            /// with exact removals. Evaluation-invariant in ℝ.
            ///
            /// # Errors
            ///
            /// [`KnotAlgebraError`] (structure refusals; a floating-
            /// point weight collapse in recomposition surfaces
            /// honestly).
            pub fn elevate_degree(&self, raise: usize) -> Result<Self, KnotAlgebraError> {
                let mut cur = self.clone();
                for _ in 0..raise {
                    let plans = spline::algebra::elevate_plan(&cur.knots, &cur.weights)?;
                    cur = cur.apply_plans(&plans);
                }
                Ok(cur)
            }
        }

        impl<T: SpanLocate> $Curve<T> {
            /// The point at `t` — span selection through the sealed
            /// [`SpanLocate`] seam (per-instantiation semantics in
            /// `geom_core::spline::locate`), the generic core per
            /// overlapped span, channel-independent hulls across spans
            /// for interval-natured scalars.
            pub fn eval(&self, t: T) -> $Point<T> {
                let spans = t.locate_spans(&self.knots);
                let mut acc = self.eval_in_span(spans.first, t);
                for s in (spans.first + 1)..=spans.last {
                    // Skip empty spans (interior multiplicity):
                    // find_span assigns every parameter — a repeated
                    // knot value included — to the nonempty span
                    // starting at it, which this loop's range always
                    // covers, so nothing is discarded (containment
                    // preserved); an empty span itself would only
                    // contribute poison (zero basis denominators).
                    if !self.knots.span_is_nonempty(s) {
                        continue;
                    }
                    let q = self.eval_in_span(s, t);
                    acc = $Point::new($(acc.$c.enclosure_hull(q.$c)),+);
                }
                acc
            }

            /// The first derivative at `t` (span selection as
            /// [`Self::eval`]; the `Dual` kink convention at knots is
            /// the seam's — the derivative of the program as evaluated).
            pub fn deriv(&self, t: T) -> $Vector<T> {
                let spans = t.locate_spans(&self.knots);
                let mut acc = self.deriv_in_span(spans.first, t);
                for s in (spans.first + 1)..=spans.last {
                    // Empty-span skip: see `eval`'s note.
                    if !self.knots.span_is_nonempty(s) {
                        continue;
                    }
                    let q = self.deriv_in_span(s, t);
                    acc = $Vector::new($(acc.$c.enclosure_hull(q.$c)),+);
                }
                acc
            }

            /// The second derivative at `t` (contract as
            /// [`Self::deriv`]).
            pub fn deriv2(&self, t: T) -> $Vector<T> {
                let spans = t.locate_spans(&self.knots);
                let mut acc = self.deriv2_in_span(spans.first, t);
                for s in (spans.first + 1)..=spans.last {
                    // Empty-span skip: see `eval`'s note.
                    if !self.knots.span_is_nonempty(s) {
                        continue;
                    }
                    let q = self.deriv2_in_span(s, t);
                    acc = $Vector::new($(acc.$c.enclosure_hull(q.$c)),+);
                }
                acc
            }
        }
    };
}

nurbs_curve!(NurbsCurve2, Point2, Vec2, x, y);
nurbs_curve!(NurbsCurve3, Point3, Vec3, x, y, z);

impl<T: Bounds> NurbsCurve3<T> {
    /// The control coordinates lifted to ring points — the data-in
    /// shape of `geom_core::spline::compose` (M5 PR 4): channel `d`,
    /// point `i`, as `[x, y, z]` channels of ring enclosures. Pair with
    /// [`Self::knots`] and [`Self::weights`] to build a `CurveRingData`
    /// for composite bounds.
    ///
    /// **The bracket seam (M6-2).** Knots, weights and degree are `f64`
    /// structure (module docs), so the only scalar-typed data in a
    /// carrier is the control net — and a control point enters the C9
    /// ring through its own bracket, never through an evaluation. At
    /// `f64` the bracket is the value (`lo` = `hi`), so this is bitwise
    /// what the `f64`-only form produced; at the interval scalar each
    /// coefficient carries its enclosure into the hull, which is what
    /// makes a composite bound over a lifted carrier honest.
    pub fn ring_coords(&self) -> Vec<Vec<RingInterval>> {
        let lift = |f: fn(&Point3<T>) -> T| -> Vec<RingInterval> {
            self.control
                .iter()
                .map(|p| {
                    let c = f(p);
                    RingInterval::from_bounds(c.lo(), c.hi())
                })
                .collect()
        };
        vec![lift(|p| p.x), lift(|p| p.y), lift(|p| p.z)]
    }
}

impl<T: Bounds> NurbsCurve2<T> {
    /// The 2-D counterpart of [`NurbsCurve3::ring_coords`]: `[x, y]`
    /// channels of ring enclosures, through the same bracket seam.
    pub fn ring_coords(&self) -> Vec<Vec<RingInterval>> {
        let lift = |f: fn(&Point2<T>) -> T| -> Vec<RingInterval> {
            self.control
                .iter()
                .map(|p| {
                    let c = f(p);
                    RingInterval::from_bounds(c.lo(), c.hi())
                })
                .collect()
        };
        vec![lift(|p| p.x), lift(|p| p.y)]
    }
}

impl<T: Real> NurbsCurve3<T> {
    /// The "no description yet" placeholder payload for
    /// [`crate::Curve3::Nurbs`]: a structurally valid degree-1 segment
    /// whose control points are all-poison, so every evaluation yields
    /// the all-poison point — bit-for-bit the totality behavior the
    /// former unit placeholder variant had (representable ≠ described;
    /// fails every downstream certification loudly, D4 ¶2).
    pub fn placeholder() -> Self {
        let nan = T::from_f64(f64::NAN);
        let p = Point3::new(nan, nan, nan);
        Self {
            // Structurally valid by construction: clamped degree-1
            // vector, two positive weights, two control points.
            knots: KnotVector::unit_segment(1),
            control: vec![p, p],
            weights: vec![1.0, 1.0],
        }
    }
}
