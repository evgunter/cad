//! NURBS curves — [`NurbsCurve2`] (the future pcurve substrate; not
//! wired into any enum) and [`NurbsCurve3`] (the [`crate::curves::Curve3::Nurbs`]
//! payload), M5 PR 3.
//!
//! # Data model (binding conventions)
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
//!   a caller-supplied [`Span`] — ring ops + `from_f64` only, total for
//!   every scalar. For `t` outside the span's knot interval the result
//!   is the span's **polynomial extension** (documented garbage-out —
//!   detecting it would need the comparison [`Real`] deliberately
//!   lacks). A [`Span`] is validated against the vector it was drawn
//!   from and carries its own control window, so the window base is an
//!   addition rather than a `span − degree` that could underflow — but
//!   it carries no borrow of that vector, so each evaluator asks
//!   [`KnotVector::admits`] and answers a span this curve's knots do
//!   not admit with an **all-poison** point/derivative triple rather
//!   than an out-of-bounds index (D9: the kernel never panics on any
//!   input).
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

use geom_core::spline::{self, KnotAlgebraError, KnotVector, Span, SpanLocate, SplineError};
use geom_core::{Point2, Point3, Real, RingInterval, Vec2, Vec3};

use crate::net;

/// The rational speed meter's **fixed refinement schedule** (D9: a
/// structure choice, never a decision): before the per-span scan of
/// `speed_lower_bound`'s rational arm, every nonempty span is split
/// into this many equal pieces by knot insertion. Knot insertion is
/// evaluation-invariant, so the curve is unchanged; only the hulls the
/// bound is assembled from shrink. The constant is a measured
/// trade-off, not a tuning knob — see the `rational_speed_lower_bound`
/// docs and the adversarial rows in `tests/curves/m5_pr7_speed_meter.rs`.
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
                net::validate_counts(knots.control_count(), control.len(), &weights)?;
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

            /// The point at `t`, evaluated **in the given span** — the
            /// generic core (module docs: the span contract; the fixed
            /// single-ascending-pass association).
            ///
            /// A span this curve's knot vector does not admit
            /// ([`KnotVector::admits`]) yields the **all-poison**
            /// point; `t` outside the span's interval still yields the
            /// span's polynomial extension (documented garbage-out).
            pub fn eval_in_span(&self, span: Span, t: T) -> $Point<T> {
                // The pairing check, before any indexing: a `Span` of
                // some other vector is a representable input, and
                // `admits` is what makes `base + j` below in range for
                // THIS curve's arrays.
                if !self.knots.admits(span) {
                    return net::poison_point::<T, $Point<T>>();
                }
                let basis = spline::basis::basis_funs(&self.knots, span, t);
                // The window's base, subtracted once inside `Span` — so
                // the underflow-prone `span − p` is gone from here, and
                // what remains is an addition. Indexing (not `zip`)
                // deliberately: `basis` is `degree + 1` long and the
                // window is `degree + 1` wide, one `degree` by the
                // check above, and if that ever ceased to hold this
                // must PANIC rather than silently drop control points.
                let base = span.first_control();
                $(let mut $c = T::zero();)+
                let mut w_acc = T::zero();
                for (j, nj) in basis.iter().enumerate() {
                    let i = base + j;
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
            /// Same totality contract as [`Self::eval_in_span`]: a span
            /// this curve's knot vector does not admit yields an
            /// all-poison triple.
            pub fn ders_in_span(&self, span: Span, t: T) -> ($Point<T>, $Vector<T>, $Vector<T>) {
                // The pairing check, as in [`Self::eval_in_span`].
                if !self.knots.admits(span) {
                    $(let $c = T::from_f64(f64::NAN);)+
                    let poison = $Vector::new($($c),+);
                    return (net::poison_point::<T, $Point<T>>(), poison, poison);
                }
                let ders = spline::basis::ders_basis_funs(&self.knots, span, t, 2);
                // Indexed off the window base, exactly as
                // [`Self::eval_in_span`]. A `zip` against a window slice
                // would be the wrong shape here: `ders`' row length and
                // the window's length are two derivations of the same
                // `degree`, and a `zip` would answer a disagreement by
                // silently dropping control points where indexing
                // panics.
                let base = span.first_control();
                $(let mut $c = [T::zero(), T::zero(), T::zero()];)+
                let mut w_hom = [T::zero(), T::zero(), T::zero()];
                for (k, row) in ders.iter().enumerate() {
                    for (j, nkj) in row.iter().enumerate() {
                        let i = base + j;
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
            pub fn deriv_in_span(&self, span: Span, t: T) -> $Vector<T> {
                self.ders_in_span(span, t).1
            }

            /// Second derivative in the given span (the last component
            /// of [`Self::ders_in_span`]).
            pub fn deriv2_in_span(&self, span: Span, t: T) -> $Vector<T> {
                self.ders_in_span(span, t).2
            }

            /// Applies a chain of structure plans to this curve's
            /// control polygon (points via `lerp(x, y, from_f64(λ))`,
            /// the fixed association; knots/weights from the final
            /// plan). Empty chain ⇒ clone.
            fn apply_plans(&self, plans: &[spline::CurvePlan]) -> Self {
                let mut control = self.control.clone();
                for plan in plans {
                    control = plan.apply_points(&control, net::poison_point::<T, $Point<T>>(), |x, y, l| {
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
                    bound = bound + net::removal_pass_bound(
                        (&cur.control, &cur.weights),
                        (&reinserted.control, &reinserted.weights),
                    );
                    cur = removed;
                }
                Ok((cur, bound))
            }

            /// A certified sup-norm bound on `|C_self − C_other|` for
            /// two curves **sharing one knot vector** (same degree,
            /// same control count; weights may differ): the
            /// `net::removal_pass_bound` formula, which only uses
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
            /// `‖C′‖ ≥ d·C′ ≥ minᵢ (d·Qᵢ)` over the `Qᵢ` active where
            /// `d` is applied. Since M8-14 (#222) the arm runs **two
            /// independent assemblies** of that inequality and states
            /// their join:
            ///
            /// 1. the **global-chord** assembly — the retired original
            ///    arm, verbatim: one direction (first→last control
            ///    point, the chord a monotone carrier advances along),
            ///    min over ALL `Qᵢ`;
            /// 2. the **per-span** assembly — the M8-2 rational
            ///    template carried over: each nonempty span projects
            ///    its ACTIVE `Qᵢ` (`i ∈ span−p .. span`) on the span's
            ///    own control chord `P_span − P_{span−p}`, and the
            ///    whole domain is the ascending `Real::min` fold over
            ///    spans. A per-span direction is legitimate because
            ///    `‖C′(t)‖ ≥ d_s·C′(t)` holds for *every* unit `d_s`,
            ///    so the min over spans of per-span bounds still
            ///    bounds the whole domain (the M8-2 review's
            ///    soundness argument, unchanged).
            ///
            /// **The join**: an assembly whose direction collapsed
            /// (poison) abstains; if both abstain the answer is
            /// poison; if both are real the answer is their `max` —
            /// sound because each is independently a lower bound on
            /// the same `inf‖C′‖`. Every cell of that lattice — both
            /// abstentions, both-poison, and the poisoned-INPUT
            /// no-laundering claim below — is EXERCISED on
            /// bitwise-exact fixtures by the adopted review probes
            /// (`tests/curves/lt_r1_probes.rs::r1_join_abstention_logic`,
            /// `tests/curves/r2_lt_probes.rs::the_join_lattice_is_pinned_cell_by_cell`),
            /// and the same suites' randomized fuzz kills the unsound
            /// near-neighbors of the scan (active window shifted, last
            /// span dropped, either arm deleted from the join) that
            /// the smooth-interpolant corpus alone cannot detect. The
            /// join is therefore **never
            /// below the retired single-chord arm** on any carrier
            /// that arm bounded (the M8-14 corpus pins exactly that,
            /// green rows as floors), while a long-turn carrier (a
            /// helix past half a revolution, a closed loop) whose
            /// speed never drops no longer collapses the meter merely
            /// because its tangent leaves the global chord's
            /// half-space — that collapse was a MEASUREMENT artifact
            /// of assembly 1, and assembly 2 retires it.
            ///
            /// The result may still be zero or negative — a genuine
            /// stationary point (cusp, turn-around) defeats every
            /// direction on its own span — and that is reported
            /// honestly, so the margin collapses and the caller's
            /// trilean escalates rather than guessing.
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
            /// Both arms project per span (the integral arm since
            /// M8-14 — before that its single global direction also
            /// collapsed on any curve that turns away from its chord,
            /// which is what refused every ≥ half-turn sweep path).
            /// Both are sound lower bounds on `‖C′‖`; neither is a
            /// claim about the curve's shape beyond its speed.
            ///
            /// The arm is chosen on **f64 structure** (`w_j == 1.0`
            /// exactly), never on an evaluation scalar.
            ///
            /// # Rounding posture
            ///
            /// The chord directions are `chord/‖chord‖` — unit only to
            /// rounding at `f64`, so the plain-f64 reading is a bound
            /// up to about a relative ulp (both review harnesses
            /// measured the worst case one ulp on the SOUND side).
            /// This is the kernel-wide posture shared with the
            /// rational arm; the `Interval` instantiation is the
            /// certified lane, and the bracket row in
            /// `tests/curves/m5_pr7_speed_meter.rs` pins containment.
            ///
            /// # Poison (total, D4 ¶2)
            ///
            /// A zero degree, fewer than two control points, a
            /// non-positive difference of the knots framing any
            /// derivative coefficient (`u_{i+p+1} ≤ u_{i+1}` — a
            /// structural violation the old arm turned into ±∞/NaN
            /// arithmetic instead of naming), a knot vector with no
            /// nonempty span, or BOTH assemblies abstaining — every
            /// one yields NaN. A bound is never fabricated. The
            /// knot-difference clause is DEFENSIVE: it needs an
            /// interior multiplicity of `p + 1`, which Clamped-v1
            /// validation forbids (interior multiplicity ≤ `p`, end
            /// multiplicity exactly `p + 1` with nonempty end spans),
            /// so no validated constructor reaches it — it guards
            /// future unvalidated paths, and is an untestable-by-
            /// construction claim, stated as such rather than pinned.
            ///
            /// Structural misses INSIDE the per-span scan (an active
            /// window past the control net, an index underflow) poison
            /// the WHOLE meter rather than abstaining the one
            /// assembly — deliberate asymmetry: chord collapse is a
            /// fact about a well-formed curve, a range miss is a
            /// construction-invariant break, and fail-loud beats
            /// recovering around corrupted structure.
            ///
            /// The join's one-sided recovery is NOT poison
            /// laundering: an assembly abstains only when its own
            /// chord DIRECTION collapsed (`0/0`), a structural fact
            /// about which projections exist, while a poisoned INPUT
            /// (a non-finite control point) poisons the projections
            /// of every assembly whose active set touches it — the
            /// global assembly's min covers all `Qᵢ` and the span
            /// containing the point poisons the per-span fold, so
            /// corrupted data still reaches the caller as poison
            /// through both arms at once. `Real::is_poison` here
            /// discriminates assembly structure, never geometry: the
            /// geometric decision (is the bound positive?) stays with
            /// the caller's trilean.
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
                // Derivative coefficients, once for the curve:
                // `Qᵢ = p·(Pᵢ₊₁ − Pᵢ)/(uᵢ₊ₚ₊₁ − uᵢ₊₁)`. The knot
                // difference is checked POSITIVE on f64 structure —
                // the totality clause above — so every coefficient
                // below is finite arithmetic on finite structure.
                let mut coeffs = Vec::with_capacity(self.control.len() - 1);
                for i in 0..(self.control.len() - 1) {
                    let (Some(a), Some(b)) = (self.control.get(i), self.control.get(i + 1)) else {
                        return poison;
                    };
                    let (Some(&lo), Some(&hi)) = (knots.get(i + 1), knots.get(i + p + 1)) else {
                        return poison;
                    };
                    let du = hi - lo;
                    #[allow(clippy::neg_cmp_op_on_partial_ord)]
                    if !(du > 0.0) {
                        return poison;
                    }
                    #[allow(clippy::cast_precision_loss)]
                    let scale = T::from_f64(p as f64) / T::from_f64(du);
                    coeffs.push((*b - *a) * scale);
                }
                // ---- Assembly 1: the global chord (the retired
                // original arm, verbatim — same direction, same fold
                // order, bit-identical where it was defined). ----
                let (Some(first), Some(last)) = (self.control.first(), self.control.last()) else {
                    return poison;
                };
                let global = {
                    let chord = *last - *first;
                    // A collapsed chord makes this 0/0 ⇒ poison ⇒ the
                    // assembly abstains at the join (the rational
                    // arm's chord treatment).
                    let d = chord / chord.norm();
                    let mut acc: Option<T> = None;
                    for q in &coeffs {
                        let v = d.dot(*q);
                        acc = Some(match acc {
                            None => v,
                            // `Real::min`/`max` are NaN-propagating
                            // (poison in, poison out) and total — no
                            // comparison here or below.
                            Some(m) => m.min(v),
                        });
                    }
                    acc.unwrap_or(poison)
                };
                // ---- Assembly 2: per-span chords (the M8-2 rational
                // template), fixed ascending span order (D9): the min
                // of `d_span·Qᵢ` over the ACTIVE coefficients
                // (`i ∈ span−p .. span`), folded over spans. ----
                let perspan = {
                    let mut acc: Option<T> = None;
                    for index in self.knots.first_span()..=self.knots.last_span() {
                        // Emptiness check and span validation are one step.
                        let Some(span) = self.knots.span(index) else {
                            continue;
                        };
                        // The window's base, subtracted once inside
                        // `Span` — the `span − p` that used to need a
                        // `checked_sub` here.
                        let (lo_i, span) = (span.first_control(), span.index());
                        // The remaining range miss is the one a `Span`
                        // cannot speak to: it bounds its window by the
                        // KNOT vector's control count, not by this
                        // curve's array. A mismatch poisons the WHOLE
                        // meter (early return), not just this assembly
                        // — a construction-invariant break fails loud
                        // (doc: "Poison", the stated asymmetry with
                        // chord-collapse abstention).
                        if span >= self.control.len() {
                            return poison;
                        }
                        let Some(active) = coeffs.get(lo_i..span) else {
                            return poison;
                        };
                        // The span's own control chord, as unit
                        // direction; collapse ⇒ 0/0 ⇒ this span
                        // poisons THIS assembly (which then abstains
                        // at the join — the other assembly still
                        // covers the same span soundly).
                        let (Some(a), Some(b)) =
                            (self.control.get(lo_i), self.control.get(span))
                        else {
                            return poison;
                        };
                        let chord = *b - *a;
                        let d = chord / chord.norm();
                        for q in active {
                            let v = d.dot(*q);
                            acc = Some(match acc {
                                None => v,
                                Some(m) => m.min(v),
                            });
                        }
                    }
                    acc.unwrap_or(poison)
                };
                // ---- The join (doc: "The join"). ----
                match (global.is_poison(), perspan.is_poison()) {
                    (true, true) => poison,
                    (true, false) => perspan,
                    (false, true) => global,
                    (false, false) => global.max(perspan),
                }
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
            /// choice — dividing by the min-weight floor — is the
            /// direction for an UPPER bound on the derivative, and
            /// would be unsound here.) Which case applies is a
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
            /// whole domain. The integral arm adopted the same
            /// per-span scan in M8-14 (#222), joined with its
            /// original global chord — see [`Self::speed_lower_bound`].
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
            /// `tests/curves/m5_pr7_speed_meter.rs`'s bracket row pins that
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
            /// `tests/curves/m5_pr7_speed_meter.rs` pin where it currently
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
                    // A plain emptiness filter: this loop builds knot
                    // VALUES and constructs no window, so there is no
                    // span validation here to fuse with (the shape
                    // `mesh`'s `rational_split_points` shares).
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
                for index in self.knots.first_span()..=self.knots.last_span() {
                    // Emptiness check and span validation are one step.
                    let Some(span) = self.knots.span(index) else {
                        continue;
                    };
                    // The active window, computed once at the `Span`'s
                    // construction — the `checked_sub(p)` that used to
                    // stand here is not a reachable refusal any more.
                    let (first, last) = (span.first_control(), span.index());
                    // The one refusal a `Span` cannot make on the
                    // caller's behalf: its window is bounded by the KNOT
                    // vector's control count, not by this curve's array.
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
                net::removal_pass_bound(
                    (&self.control, &self.weights),
                    (&other.control, &other.weights),
                )
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
                // `spans.first` arrives already validated — the locator
                // is where span validity originates, so there is
                // nothing to re-check and no `expect` here.
                let mut acc = self.eval_in_span(spans.first, t);
                for s in (spans.first.index() + 1)..=spans.last.index() {
                    // Skip empty spans (interior multiplicity):
                    // find_span assigns every parameter — a repeated
                    // knot value included — to the nonempty span
                    // starting at it, which this loop's range always
                    // covers, so nothing is discarded (containment
                    // preserved); an empty span itself would only
                    // contribute poison (zero basis denominators).
                    // The emptiness check and the span's validation are
                    // now the same operation.
                    let Some(span) = self.knots.span(s) else { continue };
                    let q = self.eval_in_span(span, t);
                    acc = $Point::new($(acc.$c.enclosure_hull(q.$c)),+);
                }
                acc
            }

            /// The first derivative at `t` (span selection as
            /// [`Self::eval`]; the `Dual` kink convention at knots is
            /// the seam's — the derivative of the program as evaluated).
            pub fn deriv(&self, t: T) -> $Vector<T> {
                let spans = t.locate_spans(&self.knots);
                // `spans.first` arrives already validated — the locator
                // is where span validity originates, so there is
                // nothing to re-check and no `expect` here.
                let mut acc = self.deriv_in_span(spans.first, t);
                for s in (spans.first.index() + 1)..=spans.last.index() {
                    // Empty-span skip: see `eval`'s note.
                    // The emptiness check and the span's validation are
                    // now the same operation.
                    let Some(span) = self.knots.span(s) else { continue };
                    let q = self.deriv_in_span(span, t);
                    acc = $Vector::new($(acc.$c.enclosure_hull(q.$c)),+);
                }
                acc
            }

            /// The second derivative at `t` (contract as
            /// [`Self::deriv`]).
            pub fn deriv2(&self, t: T) -> $Vector<T> {
                let spans = t.locate_spans(&self.knots);
                // `spans.first` arrives already validated — the locator
                // is where span validity originates, so there is
                // nothing to re-check and no `expect` here.
                let mut acc = self.deriv2_in_span(spans.first, t);
                for s in (spans.first.index() + 1)..=spans.last.index() {
                    // Empty-span skip: see `eval`'s note.
                    // The emptiness check and the span's validation are
                    // now the same operation.
                    let Some(span) = self.knots.span(s) else { continue };
                    let q = self.deriv2_in_span(span, t);
                    acc = $Vector::new($(acc.$c.enclosure_hull(q.$c)),+);
                }
                acc
            }
        }
    };
}

nurbs_curve!(NurbsCurve2, Point2, Vec2, x, y);
nurbs_curve!(NurbsCurve3, Point3, Vec3, x, y, z);

impl<T: geom_core::CertifiedBounds> NurbsCurve3<T> {
    /// The control coordinates lifted to ring points — the data-in
    /// shape of `geom_core::spline::compose`: channel `d`, point `i`,
    /// as `[x, y, z]` channels of ring enclosures. Pair with
    /// [`Self::knots`] and [`Self::weights`] to build a `CurveRingData`
    /// for composite bounds. The bracket seam this reads the net
    /// through is the shared one (`net::ring_coords`).
    pub fn ring_coords(&self) -> Vec<Vec<RingInterval>> {
        net::ring_coords(&self.control)
    }
}

impl<T: geom_core::CertifiedBounds> NurbsCurve2<T> {
    /// [`NurbsCurve3::ring_coords`] at two channels: `[x, y]` channels
    /// of ring enclosures, through the same bracket seam and the same
    /// body.
    pub fn ring_coords(&self) -> Vec<Vec<RingInterval>> {
        net::ring_coords(&self.control)
    }
}

impl<T: Real> NurbsCurve3<T> {
    /// The "no description yet" placeholder payload for
    /// [`crate::curves::Curve3::Nurbs`]: a structurally valid degree-1 segment
    /// whose control points are all-poison, so every evaluation yields
    /// the all-poison point — bit-for-bit the totality behavior the
    /// former unit placeholder variant had (representable ≠ described;
    /// fails every downstream certification loudly, D4 ¶2).
    pub fn placeholder() -> Self {
        let p = net::poison_point::<T, Point3<T>>();
        Self {
            // Structurally valid by construction: clamped degree-1
            // vector, two positive weights, two control points.
            knots: KnotVector::unit_segment(1),
            control: vec![p, p],
            weights: vec![1.0, 1.0],
        }
    }

    /// Is this payload the [`NurbsCurve3::placeholder`] — the "no
    /// description yet" state — rather than a described curve?
    ///
    /// The discriminator and the reason it is `all` and not
    /// `any` are the crate docs' totality-and-poison section;
    /// the surface and curve halves answer it identically.
    pub fn is_placeholder(&self) -> bool {
        net::is_placeholder(&self.control)
    }
}
