//! Knot-algebra **plans**: insertion, refinement, removal, and degree
//! elevation computed entirely as `f64` STRUCTURE (new knot vectors,
//! new weights, and affine combination schedules), applied to
//! generically-typed control points by a tiny fold
//! ([`CurvePlan::apply_points`]).
//!
//! # Why plans
//!
//! Every algorithm here (Book §5.2–§5.5) combines control points with
//! coefficients that depend only on knots and weights — both `f64`
//! structure (C6). Splitting each operation into a structure-computed
//! *plan* plus a generic *applier* keeps the C6 boundary a module
//! boundary: raw `f64` comparisons live in the plan constructors,
//! while the applier does nothing but `from_f64`-lifted two-point
//! affine combinations (`lerp`, fixed association `x + (y − x)·λ`) in
//! plan order. One plan implementation serves 2-D curves, 3-D curves,
//! and each row/column of a surface net.
//!
//! # Projective form (rational)
//!
//! All combinations are homogeneous: a two-term combo
//! `Q = a·(wᵢPᵢ, wᵢ) + b·(wⱼPⱼ, wⱼ)` becomes the structure weight
//! `w_Q = a·wᵢ + b·wⱼ` and the **affine point combination**
//! `P_Q = lerp(Pⱼ, Pᵢ, λ)` with `λ = a·wᵢ / w_Q` — exact in ℝ because
//! the two projective coefficients sum to 1 by construction. Weights
//! stay `f64` forever; only points are generic.
//!
//! # One schedule, two arithmetics
//!
//! An insertion plan is applied by [`CurvePlan::apply_points`] in the
//! caller's scalar and by [`CurvePlan::apply_certified`] in the certification
//! ring, off the SAME [`Step`] list — same targets, same sources, same
//! order. The two differ in the coefficient the combination is taken
//! with, and they must: the projective applier's `λ` is an `f64`
//! quotient of weights, while interval arithmetic applier re-derives the Boehm
//! ratios `α = (u − U_j)/Δ` and `β = (U_{j+p} − u)/Δ` from the knots
//! they are made of ([`Step::Combo`]'s `ratio`) so that they round
//! OUTWARD. A ring consumer that took the stored `λ` and padded it by a
//! guessed number of ulps would be asserting a bound nobody derived;
//! re-deriving the ratios in certification arithmetic makes the insertion widen like
//! every other step of an enclosure. Interval arithmetic applier takes HOMOGENEOUS
//! coefficients, where the combination is the plain convex one and no
//! `λ` is needed.

use super::knots::{InteriorKnot, KnotVector, SplineError};
use crate::interval::Interval;
use crate::interval::certification::Certification;
use crate::readable::Readable;

/// A typed knot-algebra refusal (fail-loud; the kernel never panics).
#[derive(Clone, Debug, PartialEq)]
pub enum KnotAlgebraError {
    /// The inputs fail basic spline structure validation (weight
    /// count/positivity/finiteness against the knot vector).
    Structure(SplineError),
    /// Insertion parameter not strictly inside the knot domain (or
    /// not finite). Boundary insertion is meaningless for a clamped
    /// vector (end multiplicity is already `degree + 1`).
    ParameterOutsideDomain {
        /// The offending parameter.
        u: f64,
    },
    /// Inserting would push an interior multiplicity past `degree`.
    MultiplicityOverflow {
        /// The value whose multiplicity would overflow.
        u: f64,
        /// Its current multiplicity.
        have: usize,
        /// The interior budget (`degree`).
        budget: usize,
    },
    /// Removal of a value that is not an interior knot (exact `f64`
    /// equality — structure identity; end values are not removable
    /// from a clamped vector).
    KnotNotPresent {
        /// The requested value.
        u: f64,
    },
    /// Removing more copies than the knot's multiplicity.
    RemovalExceedsMultiplicity {
        /// The value being removed.
        u: f64,
        /// Its current multiplicity.
        have: usize,
        /// The requested removal count.
        requested: usize,
    },
    /// A removal chain produced a non-positive or non-finite weight —
    /// the candidate polygon leaves the positive-weight regime, so the
    /// removal is refused rather than returning an invalid curve.
    WeightCollapse {
        /// New-polygon index of the collapsed weight.
        index: usize,
    },
}

impl core::fmt::Display for KnotAlgebraError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            KnotAlgebraError::Structure(e) => write!(f, "the knot edit refused: {e}"),
            KnotAlgebraError::ParameterOutsideDomain { u } => write!(
                f,
                "knot parameter {} is not strictly inside the domain",
                Readable(*u)
            ),
            KnotAlgebraError::MultiplicityOverflow { u, have, budget } => write!(
                f,
                "inserting knot {} (multiplicity {have}) exceeds the interior budget {budget}",
                Readable(*u)
            ),
            KnotAlgebraError::KnotNotPresent { u } => {
                write!(f, "{} is not an interior knot", Readable(*u))
            }
            KnotAlgebraError::RemovalExceedsMultiplicity { u, have, requested } => write!(
                f,
                "removing knot {} {requested} times exceeds its multiplicity {have}",
                Readable(*u)
            ),
            KnotAlgebraError::WeightCollapse { index } => write!(
                f,
                "removing a knot collapsed weight {index} out of the positive regime"
            ),
        }
    }
}

impl core::error::Error for KnotAlgebraError {}

/// A source operand of a plan step: an index into the old polygon or
/// into the already-built portion of the new polygon.
#[derive(Clone, Copy, Debug)]
enum Src {
    Old(usize),
    New(usize),
}

/// The knots one Boehm insertion's ratios are made of: `lo = U[j]` and
/// `hi = U[j + p]` of the knot vector the step was planned against, and
/// the value inserted. They give `α = (inserted − lo)/(hi − lo)` and
/// its complement `β = (hi − inserted)/(hi − lo)`.
///
/// Carried as INGREDIENTS and not as values, because the two appliers
/// need them at two precisions: `f64`, folded into the projective `λ`
/// below, and outward-rounded ring quotients for
/// [`CurvePlan::apply_certified`]. A stored `f64` ratio would leave interval arithmetic
/// applier padding a rounded number by a guess.
#[derive(Clone, Copy, Debug)]
struct Ratio {
    inserted: f64,
    lo: f64,
    hi: f64,
}

/// One plan step: assign new-polygon slot `target`.
#[derive(Clone, Debug)]
enum Step {
    /// `new[target] = old[from]`.
    Carry { target: usize, from: usize },
    /// `new[target] = lerp(x, y, lambda)` — fixed association
    /// `x + (y − x)·λ` with the caller's lifted `λ`.
    ///
    /// `ratio` is `Some` exactly for a KNOT-INSERTION step, whose `λ`
    /// is the projective form of a Boehm ratio. Removal and degree
    /// elevation combine with coefficients that are not ratios of
    /// knots at all, so they carry `None` and
    /// [`CurvePlan::apply_certified`] refuses them.
    Combo {
        target: usize,
        x: Src,
        y: Src,
        lambda: f64,
        ratio: Option<Ratio>,
    },
}

/// One structure-computed polygon rewrite: the new knot vector, the
/// new weights, and the point combination schedule. Produced only by
/// this module's constructors (invariants: every `target` is assigned
/// exactly once, `New` sources refer to already-assigned slots).
#[derive(Clone, Debug)]
pub struct CurvePlan {
    knots: KnotVector,
    weights: Vec<f64>,
    steps: Vec<Step>,
}

impl CurvePlan {
    /// The knot vector after this plan.
    pub fn knots(&self) -> &KnotVector {
        &self.knots
    }

    /// The weights after this plan (all strictly positive — checked at
    /// plan construction).
    pub fn weights(&self) -> &[f64] {
        &self.weights
    }

    /// Applies the point schedule: `old` is the previous control
    /// polygon, `lerp(x, y, λ)` the caller's affine combination
    /// (`x + (y − x)·λ` with `λ` lifted via `from_f64` — the fixed
    /// association every consumer documents), and `poison` the
    /// caller's poison point — the total fallback for a malformed
    /// plan, which the constructors rule out but the applier does not
    /// trust (D4: fail loud, never panic).
    pub fn apply_points<P: Copy>(
        &self,
        old: &[P],
        poison: P,
        lerp: impl Fn(P, P, f64) -> P,
    ) -> Vec<P> {
        let n_new = self.knots.control_count();
        let mut new: Vec<Option<P>> = vec![None; n_new];
        let fetch = |new: &[Option<P>], s: Src| -> Option<P> {
            match s {
                Src::Old(i) => old.get(i).copied(),
                Src::New(i) => new.get(i).copied().flatten(),
            }
        };
        for step in &self.steps {
            match *step {
                Step::Carry { target, from } => {
                    if target < n_new {
                        new[target] = old.get(from).copied();
                    }
                }
                Step::Combo {
                    target,
                    x,
                    y,
                    lambda,
                    ..
                } => {
                    if target < n_new {
                        let combined = match (fetch(&new, x), fetch(&new, y)) {
                            (Some(px), Some(py)) => Some(lerp(px, py, lambda)),
                            _ => None,
                        };
                        new[target] = combined;
                    }
                }
            }
        }
        new.into_iter().map(|slot| slot.unwrap_or(poison)).collect()
    }

    /// **The same schedule, applied in the certification ring to one
    /// HOMOGENEOUS coefficient channel** — the weight net `w`, or one
    /// spatial channel of `w·P`. Module docs, "One schedule, two
    /// arithmetics".
    ///
    /// Homogeneous is what makes this a plain affine combination:
    /// insertion on `(w·P, w)` is `Q_j = C_{j−1}·β_j + C_j·α_j` with the
    /// Boehm ratios themselves, where the PROJECTIVE applier above needs
    /// [`CurvePlan::weights`]' quotient `λ` to combine de-homogenized
    /// points. So no weight is read here, and the result is an
    /// enclosure of the refined homogeneous net of the DESCRIBED
    /// curve: `α` and `β` are ring quotients of knot enclosures,
    /// outward rounded, and every coefficient the caller handed in is
    /// widened by them rather than re-rounded to `f64`.
    ///
    /// # The convex form, and why `β` comes from the knots
    ///
    /// **This section is the ONE home of that argument in this crate.**
    /// `compose`'s `insert_once_ring` combines the same way and cites
    /// this method rather than restating it — a change whose subject is
    /// duplicated width has no business shipping a duplicated argument.
    ///
    /// With `Δ = U_{j+p} − U_j`, positive by the insertion precondition
    /// (`insert_once`'s band comment, so the quotients never refuse on a
    /// valid plan):
    ///
    /// ```text
    /// α = (u − U_j)/Δ        β = (U_{j+p} − u)/Δ
    /// ```
    ///
    /// **`β` is derived from the knots, NOT as `1 − α`, and the two are
    /// not required to sum to exactly one.** Each encloses its own true
    /// ratio by outward rounding, which is the whole of what the
    /// enclosure needs: with `λ` the exact real ratio, `α ∋ λ` and
    /// `β ∋ 1 − λ`, and interval `*` and `+` are inclusion-monotone, so
    /// the combination contains `(1 − λ)·x + λ·y` for every `x`, `y` in
    /// the input enclosures. `1 − α` would inherit `α`'s rounding and
    /// add its own, and route the argument through a subtraction rather
    /// than through the knots.
    ///
    /// **The convex form reads each coefficient ONCE, and that is
    /// WIDTH.** `x + (y − x)·α` reads `x` TWICE, so an interval `x`
    /// enters the width with coefficient `1 + α` and a fold of
    /// insertions multiplies its dust up step by step. Read once each,
    /// the width grows only by the ratios' own rounding — measured at 16
    /// ulps of the coefficient scale over 30 insertions against 355 for
    /// the lerp form on this applier, and at 4.7 against 473.7 over 80
    /// insertions on `insert_once_ring`'s fold
    /// (`compose`'s `the_convex_form_does_not_inflate_the_fold`).
    ///
    /// **The step is met with the hull of its two sources**, so a
    /// constant column stays its point; why that meet is sound is
    /// `convex_step`'s docs (private, so a name and not a link).
    ///
    /// **Two width allowances, one claim.** This module's
    /// `the_ring_applier_stays_in_step_with_the_point_applier`
    /// allows `8.0·(plans.len() + 1)` ulps where `compose`'s row allows
    /// `2 + 0.5·insertions` — a 16x difference in slope over the same
    /// quantity. Neither is a derived bound; both are ceilings set so
    /// that a form which multiplies its width per step cannot meet them
    /// while the convex form can, and they were set against different
    /// fixtures (equal-split plan chains here, full-multiplicity
    /// decomposition there). The looser one is this module's, and it is
    /// the one to tighten first if either is ever asked to catch a
    /// small regression.
    ///
    /// **Total, and NaI is the refusal** (D4): a plan step with no
    /// insertion ratio — degree elevation, knot removal — refuses its
    /// target, as does a malformed plan or a channel of the wrong
    /// length. The refusal then flows through every hull the caller
    /// reads.
    pub fn apply_certified(&self, old: &[Interval]) -> Vec<Interval> {
        let n_new = self.knots.control_count();
        let mut new: Vec<Option<Interval>> = vec![None; n_new];
        let fetch = |new: &[Option<Interval>], s: Src| -> Option<Interval> {
            match s {
                Src::Old(i) => old.get(i).copied(),
                Src::New(i) => new.get(i).copied().flatten(),
            }
        };
        for step in &self.steps {
            match *step {
                Step::Carry { target, from } => {
                    if target < n_new {
                        new[target] = old.get(from).copied();
                    }
                }
                Step::Combo {
                    target,
                    x,
                    y,
                    ratio,
                    ..
                } => {
                    if target < n_new {
                        new[target] = match (fetch(&new, x), fetch(&new, y), ratio) {
                            (Some(cx), Some(cy), Some(r)) => {
                                Some(convex_step(cx, cy, r.lo, r.hi, r.inserted))
                            }
                            _ => None,
                        };
                    }
                }
            }
        }
        new.into_iter()
            .map(|slot| slot.unwrap_or_else(Interval::refused))
            .collect()
    }
}

/// One Boehm combination in the certification ring, shared by
/// [`CurvePlan::apply_certified`] and `compose`'s `insert_once_ring`:
/// `β·x + α·y` with `Δ = hi − lo`, `α = (u − lo)/Δ` and
/// `β = (hi − u)/Δ`, BOTH ring quotients of the knots (fixed
/// association, D9), met with the hull of `x` and `y`. Why the convex
/// form and why `β` comes from the knots is
/// [`CurvePlan::apply_certified`]'s docs.
///
/// **Precondition: `lo`, `u` and `hi` are exact `f64` knots with
/// `lo < u < hi`.** Both callers insert `u` strictly inside the span
/// `[U_j, U_{j+p}]` of the vector they plan against (`insert_once`'s
/// band, `insert_once_ring`'s window), so `Δ > 0` and neither quotient
/// refuses.
///
/// **Why the hull meet is sound.** `α` and `β` round outward
/// INDEPENDENTLY, so `α_hi + β_hi > 1` and `β·x + α·y` alone reaches a
/// little past both sources: a constant column, whose source hull is a
/// point, would come out as a bracket around it. But the knots are
/// exact, so the true ratio `λ = (u − lo)/(hi − lo)` is a real number
/// in `[0, 1]` by the precondition, and `(1 − λ)·x + λ·y` lies between
/// `x` and `y` for every `x ∈ X`, `y ∈ Y`. The hull of `X` and `Y` is
/// therefore a second enclosure of the same value, by an argument that
/// never reads `α` or `β`, and two sound enclosures of one value meet
/// ([`Certification::meet`]). The result is variation-diminishing in
/// the sense the reals give: a constant column stays its point, so an
/// exactly-represented weight channel (a polynomial curve's `w ≡ 1`)
/// stays exact through any fold. `the_convex_form_holds_a_constant_column_exactly`
/// pins that; `compose`'s `the_ring_fold_encloses_the_exact_refined_net`
/// pins containment against exact rationals (both `#[cfg(test)]`).
pub(super) fn convex_step(x: Interval, y: Interval, lo: f64, hi: f64, u: f64) -> Interval {
    let (lo, hi) = (Interval::point(lo), Interval::point(hi));
    let u = Interval::point(u);
    let span = hi - lo;
    let alpha = (u - lo) / span;
    let beta = (hi - u) / span;
    (x * beta + y * alpha).meet(Interval::hull(x, y))
}

/// Validates weights against a knot vector: count, positivity,
/// finiteness (the shared precondition of every plan constructor).
// `!(x > 0)`-shaped guards below are deliberate: the negated form is
// NaN-catching (`NaN > 0` is false, so NaN refuses), where `x <= 0`
// would silently pass NaN through — the fail-loud direction.
#[allow(clippy::neg_cmp_op_on_partial_ord)]
fn check_weights(kv: &KnotVector, weights: &[f64]) -> Result<(), KnotAlgebraError> {
    if weights.len() != kv.control_count() {
        return Err(KnotAlgebraError::Structure(
            SplineError::WeightCountMismatch {
                weights: weights.len(),
                control: kv.control_count(),
            },
        ));
    }
    for (index, w) in weights.iter().enumerate() {
        if !(*w > 0.0) {
            return Err(KnotAlgebraError::Structure(
                SplineError::NonPositiveWeight { index, weight: *w },
            ));
        }
        if !w.is_finite() {
            return Err(KnotAlgebraError::Structure(SplineError::NonFiniteWeight {
                index,
                weight: *w,
            }));
        }
    }
    Ok(())
}

/// Single knot insertion (Book §5.2, Boehm), `times`-fold: returns a
/// chain of plans, each built on the previous plan's structure. The
/// resulting multiplicity must stay within the interior budget
/// (`degree`); the parameter must be strictly inside the domain.
///
/// # Errors
///
/// [`KnotAlgebraError`] on structure mismatch, out-of-domain `u`, or
/// multiplicity overflow. `times == 0` is a no-op (empty chain).
// NaN-catching negated comparisons — see `check_weights`' note.
#[allow(clippy::neg_cmp_op_on_partial_ord)]
pub fn insert_knot_plan(
    kv: &KnotVector,
    weights: &[f64],
    u: f64,
    times: usize,
) -> Result<Vec<CurvePlan>, KnotAlgebraError> {
    check_weights(kv, weights)?;
    let (lo, hi) = kv.domain();
    if !u.is_finite() || !(u > lo) || !(u < hi) {
        return Err(KnotAlgebraError::ParameterOutsideDomain { u });
    }
    let have = kv.multiplicity_of(u).map_or(0, |(s, _)| s);
    if have + times > kv.degree() {
        return Err(KnotAlgebraError::MultiplicityOverflow {
            u,
            have,
            budget: kv.degree(),
        });
    }
    let mut plans = Vec::with_capacity(times);
    let mut cur_kv = kv.clone();
    let mut cur_w = weights.to_vec();
    for _ in 0..times {
        let plan = insert_once(&cur_kv, &cur_w, u);
        cur_kv = plan.knots.clone();
        cur_w = plan.weights.clone();
        plans.push(plan);
    }
    Ok(plans)
}

/// One insertion pass — preconditions established by the callers
/// (`u` strictly interior, multiplicity budget available, weights
/// validated).
///
/// **The Boehm structure is shared with [`super::compose`]'s
/// `insert_once_ring`, and the two are now a FILED duplication rather
/// than an argued one.** That function's own docs still argue the split
/// on the ground that it "folds `Interval` coefficients with an
/// outward-rounding quotient and has no weights to form `λ` from" —
/// which is a description of [`CurvePlan::apply_certified`], so the argument
/// no longer separates them, and neither does the coefficient
/// arithmetic any more: both combine in the convex form with `α` and
/// `β` re-derived from their knots, and
/// [`CurvePlan::apply_certified`]'s docs are the one home of that
/// argument. What still separates them is the SHAPE of the schedule
/// each needs: that one inserts to full interior multiplicity over a
/// raw knot list, deliberately never rebuilding a [`KnotVector`] per
/// step, where a plan chain rebuilds one per insertion. Filed on PROPS'
/// `f64-refinement-inside-an-enclosure-has-five-more-sites`; not done
/// here.
fn insert_once(kv: &KnotVector, weights: &[f64], u: f64) -> CurvePlan {
    let p = kv.degree();
    let knots = kv.knots();
    let k = kv.find_span(u);
    let s = kv.multiplicity_of(u).map_or(0, |(s, _)| s);
    let n_old = kv.control_count();
    let n_new = n_old + 1;

    let mut new_knots = Vec::with_capacity(knots.len() + 1);
    new_knots.extend_from_slice(&knots[..=k]);
    new_knots.push(u);
    new_knots.extend_from_slice(&knots[k + 1..]);
    let new_kv = KnotVector::from_algebra(new_knots, p);

    let mut new_w = vec![0.0f64; n_new];
    let mut steps = Vec::with_capacity(n_new);
    // Prefix carries: Q_j = A_j for j ≤ k − p.
    for j in 0..=(k - p) {
        steps.push(Step::Carry { target: j, from: j });
        new_w[j] = weights[j];
    }
    // The combined band: Q_j = α_j A_j + (1−α_j) A_{j−1},
    // α_j = (u − U[j]) / (U[j+p] − U[j]) ∈ (0, 1) — denominator > 0
    // because U[j] < u (j ≤ k − s, below the copy run) and
    // U[j+p] ≥ U[k+1] > u (nonempty span k). Projective form: module
    // docs.
    for j in (k - p + 1)..=(k - s) {
        let alpha = (u - knots[j]) / (knots[j + p] - knots[j]);
        let wq = alpha * weights[j] + (1.0 - alpha) * weights[j - 1];
        let lambda = alpha * weights[j] / wq;
        new_w[j] = wq;
        steps.push(Step::Combo {
            target: j,
            x: Src::Old(j - 1),
            y: Src::Old(j),
            lambda,
            ratio: Some(Ratio {
                inserted: u,
                lo: knots[j],
                hi: knots[j + p],
            }),
        });
    }
    // Suffix carries: Q_j = A_{j−1} for j ≥ k − s + 1.
    for j in (k - s + 1)..n_new {
        steps.push(Step::Carry {
            target: j,
            from: j - 1,
        });
        new_w[j] = weights[j - 1];
    }
    CurvePlan {
        knots: new_kv,
        weights: new_w,
        steps,
    }
}

/// Knot refinement (Book §5.3) as a fold of single insertions in
/// ascending parameter order (ties inserted consecutively) — a
/// deliberately simple, deterministic composition of §5.2 rather than
/// the one-pass A5.4 (documented implementation choice; the
/// evaluation-invariance obligations are identical).
///
/// # Errors
///
/// As [`insert_knot_plan`], evaluated against the *cumulative*
/// structure (earlier insertions count toward multiplicity budgets).
pub fn refine_plan(
    kv: &KnotVector,
    weights: &[f64],
    new_knots: &[f64],
) -> Result<Vec<CurvePlan>, KnotAlgebraError> {
    check_weights(kv, weights)?;
    let mut sorted = new_knots.to_vec();
    // Structure sort: refuse NaN up front, then total order is the
    // plain f64 order.
    for u in &sorted {
        if !u.is_finite() {
            return Err(KnotAlgebraError::ParameterOutsideDomain { u: *u });
        }
    }
    sorted.sort_by(f64::total_cmp);
    let mut plans = Vec::with_capacity(sorted.len());
    let mut cur_kv = kv.clone();
    let mut cur_w = weights.to_vec();
    for u in sorted {
        let mut chain = insert_knot_plan(&cur_kv, &cur_w, u, 1)?;
        // insert_knot_plan(times = 1) returns exactly one plan.
        if let Some(plan) = chain.pop() {
            cur_kv = plan.knots.clone();
            cur_w = plan.weights.clone();
            plans.push(plan);
        }
    }
    Ok(plans)
}

/// [`refine_plan`] for a **POLYNOMIAL** (unit-weight) coefficient
/// sequence — the shape a HOMOGENEOUS net has, since `w` and each `w·P`
/// channel of a rational description are themselves polynomial
/// B-splines. The schedule is the arm a ring consumer wants
/// ([`CurvePlan::apply_certified`]): the plan's own `λ` is then the
/// `f64`-rounded insertion ratio, which interval arithmetic applier does not read.
///
/// Unit weights are the net's real weights and not a stand-in, and what
/// they buy is the positivity precondition for free — nothing else, since
/// [`CurvePlan::apply_certified`] reads neither the plan's weights nor its
/// `λ`.
///
/// # Errors
///
/// As [`refine_plan`].
pub fn refine_plan_homogeneous(
    kv: &KnotVector,
    new_knots: &[f64],
) -> Result<Vec<CurvePlan>, KnotAlgebraError> {
    refine_plan(kv, &vec![1.0; kv.control_count()], new_knots)
}

/// **The equal-split refinement schedule**: the interior points that
/// cut every nonempty span of `kv` into `splits` equal pieces — the
/// `new_knots` a caller hands [`refine_plan`] or
/// [`refine_plan_homogeneous`] to refine uniformly within spans
/// ([`equal_split_plan`] is the latter composition).
///
/// A point floating point collapses onto a span end is skipped rather
/// than inserted: refinement is a tightening, never a correctness
/// condition, so a dropped point costs a wider piece and never an
/// invalid one, while an inserted collapse would raise an end knot's
/// multiplicity. Points come out in ascending span order, ascending
/// within a span; `splits` of 0 or 1 yields none.
#[must_use]
pub fn equal_split_points(kv: &KnotVector, splits: usize) -> Vec<f64> {
    let mut add = Vec::new();
    for span in kv.first_span()..=kv.last_span() {
        if !kv.span_is_nonempty(span) {
            continue;
        }
        // `span_is_nonempty` has just checked `span + 1` is in range.
        let (lo, hi) = (kv.knots()[span], kv.knots()[span + 1]);
        for k in 1..splits {
            #[allow(clippy::cast_precision_loss)]
            let f = k as f64 / splits as f64;
            let u = lo + (hi - lo) * f;
            if u > lo && u < hi {
                add.push(u);
            }
        }
    }
    add
}

/// **The equal-split refinement chain**: the [`refine_plan_homogeneous`]
/// plans that insert [`equal_split_points`]`(kv, splits)` — every
/// nonempty span of `kv` cut into `splits` equal pieces, built from
/// structure alone. One plan per inserted point, so the chain's length
/// is the insertion count. A caller that needs the refined vector takes
/// the last plan's knots, or `kv` when the chain is empty.
///
/// # Errors
///
/// As [`refine_plan`].
pub fn equal_split_plan(
    kv: &KnotVector,
    splits: usize,
) -> Result<Vec<CurvePlan>, KnotAlgebraError> {
    refine_plan_homogeneous(kv, &equal_split_points(kv, splits))
}

/// How close a grid point may come to a knot before it is dropped
/// instead of minting a hairline span or cell, in ulps of the range's
/// own width — the clearance [`GridSkip::WithinUlps`] is spelled with.
///
/// A few ulps, because that is the whole width of the defect: the
/// grid point and the knot are describing the same place, and the
/// span between them is arithmetic noise rather than geometry. It is
/// deliberately NOT a tolerance in the ε sense — no input's meaning
/// depends on it, only whether one redundant subdivision is taken.
pub const SLIVER_CLEARANCE_ULPS: u32 = 8;

/// When a uniform grid ([`domain_grid_points`], [`range_grid_points`])
/// counts a grid point as already one of the MANDATORY points it
/// defers to — a vector's interior knots, or a caller's cut set — and
/// skips it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GridSkip {
    /// Skip a grid point that IS a mandatory point: `f64` equality, so
    /// a knot one ulp off a grid point does not suppress it and both
    /// reach the output's consumer. The same rule as `WithinUlps(0)`,
    /// named for the reader.
    BitEqual,
    /// Skip a grid point within `ulps · ε · |hi − lo|` of a mandatory
    /// point, `[lo, hi]` the grid's range: the mandatory point stands
    /// and the hairline span the grid point would open beside it is
    /// never minted. [`SLIVER_CLEARANCE_ULPS`] is the clearance the tree
    /// uses.
    WithinUlps(u32),
}

/// **The domain-uniform grid**: the interior points
/// `lo + (hi − lo)·k/pieces`, `0 < k < pieces`, of the vector's DOMAIN
/// `[lo, hi]`, ascending, minus every point `skip` finds on an interior
/// knot — the `new_knots` a caller hands [`refine_plan`] or
/// [`refine_plan_homogeneous`] to refine to at least `pieces` spans
/// over the whole domain, or a break list for a per-span extraction.
///
/// **Not the equal-split schedule**: [`equal_split_points`] cuts each
/// nonempty SPAN into equal pieces, so its grid restarts at every knot
/// and a knot is a span end by construction. This grid is blind to
/// where the knots fall, which is why it takes a `skip` rule and the
/// per-span schedule does not.
///
/// It refines ANY vector, however fine already. A caller for whom a
/// vector with `pieces + degree` or more control points is fine enough
/// tests that itself and does not call; the grid does not decide it.
/// `pieces` of 0 or 1 yields none.
#[must_use]
pub fn domain_grid_points(kv: &KnotVector, pieces: usize, skip: GridSkip) -> Vec<f64> {
    let (lo, hi) = kv.domain();
    let knots: Vec<f64> = kv.interior_knots().map(|(k, _)| k).collect();
    range_grid_points(lo, hi, pieces, skip, &knots)
}

/// **The range-uniform grid** under [`domain_grid_points`]: the
/// interior points `lo + (hi − lo)·k/pieces`, `0 < k < pieces`, of an
/// arbitrary range `[lo, hi]`, ascending, minus every point that falls
/// outside the open range or that `skip` finds on a point of
/// `mandatory` — `skip`'s clearance scaled by `|hi − lo|`.
///
/// `mandatory` is whatever set the grid must defer to, and it is not
/// the grid's to widen: [`domain_grid_points`] passes a vector's
/// interior knots, and a caller cutting a sub-range on a raw knot
/// slice passes its whole cut set, which carries the range's ends
/// for completeness — the open-range test already keeps every grid
/// point off them. The test is against `mandatory` alone, never
/// against other grid points, so a point two grids share (a coarse
/// grid's point is a fine grid's when the counts divide) is kept by
/// both or dropped by both.
///
/// The clearance is `|hi − lo|·(ulps·ε)`: `ulps·ε` is exact, so the
/// product rounds once and a finite width never overflows it to `∞`
/// (which would drop every grid point beside any mandatory one).
///
/// `pieces` of 0 or 1 yields none.
#[must_use]
pub fn range_grid_points(
    lo: f64,
    hi: f64,
    pieces: usize,
    skip: GridSkip,
    mandatory: &[f64],
) -> Vec<f64> {
    let sliver = match skip {
        GridSkip::BitEqual => None,
        GridSkip::WithinUlps(ulps) => Some((hi - lo).abs() * (f64::from(ulps) * f64::EPSILON)),
    };
    (1..pieces)
        .filter_map(|k| {
            #[allow(clippy::cast_precision_loss)]
            let t = lo + (hi - lo) * (k as f64 / pieces as f64);
            (t > lo
                && t < hi
                && mandatory.iter().all(|m| match sliver {
                    None => *m != t,
                    Some(sliver) => (t - *m).abs() > sliver,
                }))
            .then_some(t)
        })
        .collect()
}

/// **Knot merging (Book §5.3), the structure half: per vector, the
/// copies [`refine_plan`] must insert to land it on the UNION of
/// `vectors`.** The union knot vector carries every distinct interior
/// value at the GREATEST multiplicity any input gives it; entry `i` of
/// the result lists, ascending with ties consecutive, what vector `i`
/// still lacks — empty when it is already there. Refining every input
/// by its entry puts them all on one bit-identical knot vector, which
/// is what "compatible" means for a skin and what a same-structure
/// deviation bound needs of its two operands.
///
/// Interior values are compared on exact `f64` identity — the
/// multiplicity rule of [`KnotVector::interior_knots`], never a
/// tolerance — and the union is read through the typed
/// [`KnotVector::interior_knot_runs`], so every value returned is
/// interior to the vector it was read from. It is NOT thereby interior
/// to every OTHER vector: inputs on different domains, or at different
/// degrees, are representable here, and the entries then name
/// insertions [`refine_plan`] refuses typed (`ParameterOutsideDomain`,
/// `MultiplicityOverflow`) when applied. That is why the entries are
/// plain `f64` and not [`InteriorKnot`]s — the type's proof stops at
/// its own vector, and the boundary where it stops is this return.
/// A caller making sections compatible elevates to one degree and
/// checks one domain first; a caller comparing two curves of one
/// pipeline has both by construction.
///
/// Total, and structure only: no control point is read. Empty input
/// yields empty output.
pub fn union_refinements(vectors: &[&KnotVector]) -> Vec<Vec<f64>> {
    let mut union: Vec<(InteriorKnot, usize)> = Vec::new();
    for kv in vectors {
        for (knot, mult) in kv.interior_knot_runs() {
            match union.iter_mut().find(|(k, _)| k.value() == knot.value()) {
                Some((_, m)) => *m = (*m).max(mult),
                None => union.push((knot, mult)),
            }
        }
    }
    union.sort_by(|a, b| a.0.value().total_cmp(&b.0.value()));
    vectors
        .iter()
        .map(|kv| {
            let own: Vec<(InteriorKnot, usize)> = kv.interior_knot_runs().collect();
            let mut add: Vec<f64> = Vec::new();
            for (knot, want) in &union {
                let have = own
                    .iter()
                    .find(|(k, _)| k.value() == knot.value())
                    .map_or(0, |(_, m)| *m);
                add.extend(core::iter::repeat_n(
                    knot.value(),
                    want.saturating_sub(have),
                ));
            }
            add
        })
        .collect()
}

/// One bounded-removal pass: the removal plan plus the **reinsertion**
/// plan that puts the removed copy back on the *new* structure. The
/// reinsertion is exact in ℝ, so applying `plan` then `reinsert` yields
/// a polygon on the ORIGINAL knot vector whose pointwise difference
/// from the original polygon bounds `|C − Ĉ|` by partition of unity —
/// the Eq. 9.81 mechanism; the projected bound itself is computed by
/// the curve/surface layer (it needs control-point norms at `T`).
#[derive(Clone, Debug)]
pub struct RemovalStep {
    /// The removal rewrite (one multiplicity step down).
    pub plan: CurvePlan,
    /// The exact reinsertion of the removed copy, built on
    /// `plan`'s structure.
    pub reinsert: CurvePlan,
}

/// Bounded knot removal (Book §5.4), `times`-fold: each pass removes
/// one copy of `u` and pairs the rewrite with its reinsertion plan for
/// the caller's error-bound computation ([`RemovalStep`]). Removal is
/// **total on the arithmetic and bounded, never silent**: no
/// removability tolerance test happens here — the caller receives the
/// rewritten polygon and the data to bound its deviation, and decides.
///
/// The chain solves the reinsertion equations
/// `A_i = α_i·Â_i + (1−α_i)·Â_{i−1}` (i = r−p ..= r−s,
/// `α_i = (u − U[i])/(U[i+p+1] − U[i]) ∈ (0,1)`) forward for the first
/// `⌈(p−s)/2⌉` unknowns and backward for the rest — the one leftover
/// equation's residual is exactly what the returned bound captures.
///
/// # Errors
///
/// [`KnotAlgebraError::KnotNotPresent`] if `u` is not an interior knot
/// (exact `f64` identity), [`KnotAlgebraError::RemovalExceedsMultiplicity`],
/// [`KnotAlgebraError::WeightCollapse`] if a pass leaves the
/// positive-weight regime, plus the shared structure refusals.
pub fn remove_knot_plan(
    kv: &KnotVector,
    weights: &[f64],
    u: f64,
    times: usize,
) -> Result<Vec<RemovalStep>, KnotAlgebraError> {
    check_weights(kv, weights)?;
    let (lo, hi) = kv.domain();
    if u == lo || u == hi {
        return Err(KnotAlgebraError::KnotNotPresent { u });
    }
    let have = kv.multiplicity_of(u).map_or(0, |(s, _)| s);
    if have == 0 {
        return Err(KnotAlgebraError::KnotNotPresent { u });
    }
    if times > have {
        return Err(KnotAlgebraError::RemovalExceedsMultiplicity {
            u,
            have,
            requested: times,
        });
    }
    let mut steps = Vec::with_capacity(times);
    let mut cur_kv = kv.clone();
    let mut cur_w = weights.to_vec();
    for _ in 0..times {
        let plan = remove_once(&cur_kv, &cur_w, u)?;
        let mut re = insert_knot_plan(&plan.knots, &plan.weights, u, 1)?;
        cur_kv = plan.knots.clone();
        cur_w = plan.weights.clone();
        // insert_knot_plan(times = 1) returns exactly one plan.
        if let Some(reinsert) = re.pop() {
            steps.push(RemovalStep { plan, reinsert });
        }
    }
    Ok(steps)
}

/// One removal pass (fn docs on [`remove_knot_plan`] for the chain
/// derivation; preconditions established there).
// NaN-catching negated comparisons — see `check_weights`' note.
#[allow(clippy::neg_cmp_op_on_partial_ord)]
fn remove_once(kv: &KnotVector, weights: &[f64], u: f64) -> Result<CurvePlan, KnotAlgebraError> {
    let p = kv.degree();
    let knots = kv.knots();
    // Present with multiplicity s, last copy at index r (caller
    // checked presence; map_or keeps this total anyway).
    let (s, r) = kv.multiplicity_of(u).map_or((1, p + 1), |sr| sr);
    let first = r - p;
    let last = r - s;
    let n_old = kv.control_count();
    let n_new = n_old - 1;

    let mut new_knots = Vec::with_capacity(knots.len() - 1);
    new_knots.extend_from_slice(&knots[..r]);
    new_knots.extend_from_slice(&knots[r + 1..]);
    let new_kv = KnotVector::from_algebra(new_knots, p);

    // α_i ∈ (0,1): U[i] < u for i ≤ r−s (below the copy run) and
    // U[i+p+1] ≥ U[r+1] > u for i ≥ r−p (above it) — validated
    // structure, so both divisions below are safe.
    let alpha = |i: usize| (u - knots[i]) / (knots[i + p + 1] - knots[i]);

    let mut new_w = vec![0.0f64; n_new];
    let mut steps = Vec::with_capacity(n_new);
    for j in 0..first {
        steps.push(Step::Carry { target: j, from: j });
        new_w[j] = weights[j];
    }
    let n_unknown = last - first; // = p − s
    let nf = n_unknown.div_ceil(2);
    // Forward chain: Â_i = (A_i − (1−α_i)·Â_{i−1}) / α_i.
    for i in first..first + nf {
        let a = alpha(i);
        let (prev_src, prev_w) = if i == first {
            (Src::Old(first - 1), weights[first - 1])
        } else {
            (Src::New(i - 1), new_w[i - 1])
        };
        let wq = (weights[i] - (1.0 - a) * prev_w) / a;
        if !(wq > 0.0) || !wq.is_finite() {
            return Err(KnotAlgebraError::WeightCollapse { index: i });
        }
        // λ overflow note: wq passed the guard, but a subnormal-
        // positive wq can still overflow `…/wq` to +∞ here; the lifted
        // infinite λ then poisons the combined point (NaN through
        // `x + (y − x)·λ`) rather than raising a typed error. Accepted:
        // validated inputs (finite weights ≥ DBL_MIN-scale, α ∈ (0,1))
        // cannot reach a subnormal wq without first tripping the
        // WeightCollapse guard in a preceding pass, and poison fails
        // certification loudly downstream (D4 ¶2) if they somehow do.
        let lambda = (weights[i] / a) / wq;
        steps.push(Step::Combo {
            target: i,
            x: prev_src,
            y: Src::Old(i),
            lambda,
            // Removal's λ is not a ratio of knots (module docs on
            // `Step::Combo`): no ring applier reads this step.
            ratio: None,
        });
        new_w[i] = wq;
    }
    // Backward chain: Â_i = (A_{i+1} − α_{i+1}·Â_{i+1}) / (1−α_{i+1}).
    for i in (first + nf..last).rev() {
        let a = alpha(i + 1);
        let (next_src, next_w) = if i == last - 1 {
            (Src::Old(last + 1), weights[last + 1])
        } else {
            (Src::New(i + 1), new_w[i + 1])
        };
        let wq = (weights[i + 1] - a * next_w) / (1.0 - a);
        if !(wq > 0.0) || !wq.is_finite() {
            return Err(KnotAlgebraError::WeightCollapse { index: i });
        }
        // λ overflow note: as in the forward chain above.
        let lambda = (weights[i + 1] / (1.0 - a)) / wq;
        steps.push(Step::Combo {
            target: i,
            x: next_src,
            y: Src::Old(i + 1),
            lambda,
            // As the forward chain: not a knot ratio.
            ratio: None,
        });
        new_w[i] = wq;
    }
    // Suffix carries (the dropped slot is `last`; everything above
    // shifts down by one).
    for j in last..n_new {
        steps.push(Step::Carry {
            target: j,
            from: j + 1,
        });
        new_w[j] = weights[j + 1];
    }
    Ok(CurvePlan {
        knots: new_kv,
        weights: new_w,
        steps,
    })
}

/// Degree elevation by one (Book §5.5), via the Bézier route:
/// (1) refine every distinct interior knot to multiplicity `p`,
/// (2) elevate each Bézier segment with the binomial combination
/// `Q_i = c_i·A_{i−1} + (1−c_i)·A_i`, `c_i = i/(p+1)` (homogeneous;
/// projective form per the module docs), (3) remove each interior
/// breakpoint back down to its original multiplicity plus one. The
/// recomposition removals are **exact in ℝ** (the elevated curve has
/// full continuity there), so no bound is surfaced; the
/// evaluation-invariance tests pin the floating-point agreement.
///
/// # Errors
///
/// The shared structure refusals; a [`KnotAlgebraError::WeightCollapse`]
/// from step (3) is possible only through floating-point degeneracy and
/// is surfaced honestly rather than clamped.
pub fn elevate_plan(kv: &KnotVector, weights: &[f64]) -> Result<Vec<CurvePlan>, KnotAlgebraError> {
    check_weights(kv, weights)?;
    let p = kv.degree();
    // Collected, not iterated: `cur_kv` below is rebuilt inside the
    // loop, so the list must outlive the borrow of `kv`.
    let interior: Vec<(f64, usize)> = kv.interior_knots().collect();

    let mut plans: Vec<CurvePlan> = Vec::new();
    let mut cur_kv = kv.clone();
    let mut cur_w = weights.to_vec();
    // (1) Bézier decomposition.
    for (v, m) in &interior {
        if *m < p {
            for plan in insert_knot_plan(&cur_kv, &cur_w, *v, p - m)? {
                cur_kv = plan.knots.clone();
                cur_w = plan.weights.clone();
                plans.push(plan);
            }
        }
    }
    // (2) Per-segment elevation.
    let stage = elevate_bezier_stage(&cur_kv, &cur_w);
    cur_kv = stage.knots.clone();
    cur_w = stage.weights.clone();
    plans.push(stage);
    // (3) Recomposition: interior breakpoints from multiplicity p+1
    // down to original + 1.
    for (v, m) in &interior {
        let excess = p - m;
        if excess > 0 {
            for step in remove_knot_plan(&cur_kv, &cur_w, *v, excess)? {
                cur_kv = step.plan.knots.clone();
                cur_w = step.plan.weights.clone();
                plans.push(step.plan);
            }
        }
    }
    Ok(plans)
}

/// The Bézier-segment elevation stage: every interior knot is at
/// multiplicity `p` (established by [`elevate_plan`] step 1).
fn elevate_bezier_stage(kv: &KnotVector, weights: &[f64]) -> CurvePlan {
    let p = kv.degree();
    let knots = kv.knots();
    // Distinct values in order (ends included).
    let mut values: Vec<f64> = Vec::new();
    for k in knots {
        if values.last() != Some(k) {
            values.push(*k);
        }
    }
    let nseg = values.len() - 1;
    let n_new = nseg * (p + 2) - (nseg - 1);

    let mut new_knots = Vec::with_capacity(knots.len() + values.len());
    for (vi, v) in values.iter().enumerate() {
        let mult = if vi == 0 || vi == nseg { p + 2 } else { p + 1 };
        for _ in 0..mult {
            new_knots.push(*v);
        }
    }
    let new_kv = KnotVector::from_algebra(new_knots, p + 1);

    let mut new_w = vec![0.0f64; n_new];
    let mut steps = Vec::with_capacity(n_new);
    steps.push(Step::Carry { target: 0, from: 0 });
    new_w[0] = weights[0];
    for seg in 0..nseg {
        let o = seg * p; // old segment offset (Bézier points o ..= o+p)
        let o2 = seg * (p + 1); // new segment offset
        for i in 1..=p {
            let c = i as f64 / (p + 1) as f64;
            let wq = c * weights[o + i - 1] + (1.0 - c) * weights[o + i];
            let lambda = c * weights[o + i - 1] / wq;
            steps.push(Step::Combo {
                target: o2 + i,
                x: Src::Old(o + i),
                y: Src::Old(o + i - 1),
                lambda,
                // Elevation's λ comes from a binomial ratio, not from
                // knots (module docs on `Step::Combo`).
                ratio: None,
            });
            new_w[o2 + i] = wq;
        }
        steps.push(Step::Carry {
            target: o2 + p + 1,
            from: o + p,
        });
        new_w[o2 + p + 1] = weights[o + p];
    }
    CurvePlan {
        knots: new_kv,
        weights: new_w,
        steps,
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;
    use crate::real::Bounds;
    use crate::spline::basis::basis_funs;

    /// 1-D rational evaluation oracle: x(t) = Σ N w x / Σ N w — the
    /// plan machinery is dimension-agnostic, so scalar control points
    /// are a complete test bed.
    fn eval1(kv: &KnotVector, w: &[f64], x: &[f64], t: f64) -> f64 {
        let span = kv.span_at(t);
        let n = basis_funs(span, t);
        let (mut num, mut den) = (0.0, 0.0);
        for (j, nj) in n.iter().enumerate() {
            let i = span.first_control() + j;
            num += nj * w[i] * x[i];
            den += nj * w[i];
        }
        num / den
    }

    fn lerp1(x: f64, y: f64, l: f64) -> f64 {
        x + (y - x) * l
    }

    fn fixture() -> (KnotVector, Vec<f64>, Vec<f64>) {
        let kv = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 2.0, 3.0, 3.0, 3.0], 2).unwrap();
        let w = vec![1.0, 2.0, 0.5, 1.5, 1.0];
        let x = vec![0.0, 1.0, 4.0, 2.0, -1.0];
        (kv, w, x)
    }

    /// The equal-split schedule's sliver guard: on the one-ulp span
    /// `[1, tiny]` every `lo + (hi − lo)·k/n` rounds onto an end, and
    /// none of those collapses reaches the output. The rest of the row
    /// pins count and order — `n − 1` points per span wider than a
    /// sliver, ascending, the grid restarting at every knot — against
    /// values written out by hand rather than re-derived from the
    /// implementation's expression.
    #[test]
    fn equal_split_points_skips_slivers_and_keeps_count_and_order() {
        let tiny = f64::from_bits(1.0f64.to_bits() + 1);
        let kv = KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.5, 0.5, 1.0, tiny, 2.0, 2.0, 2.0], 2)
            .unwrap();
        let got = equal_split_points(&kv, 4);
        // Three spans wider than a sliver — [0, 0.5], [0.5, 1], [tiny, 2] —
        // at three points each.
        assert_eq!(got.len(), 9, "{got:?}");
        assert_eq!(got[..6], [0.125, 0.25, 0.375, 0.625, 0.75, 0.875]);
        for (g, want) in got[6..].iter().zip([1.25, 1.5, 1.75]) {
            assert!((g - want).abs() <= f64::EPSILON, "{g} vs {want}");
        }
        assert!(got.windows(2).all(|w| w[0] < w[1]), "{got:?}");
        // A collapsed sliver point would land on `1.0` or `tiny`: no knot
        // value is ever a split point.
        assert!(got.iter().all(|u| !kv.knots().contains(u)), "{got:?}");
        assert!(equal_split_points(&kv, 1).is_empty());
        assert!(equal_split_points(&kv, 0).is_empty());
    }

    /// The equal-split chain inserts exactly the schedule's points, one
    /// plan each, and lands on the vector that carries them: its last
    /// plan's interior is the described interior merged with the
    /// schedule, written out by hand. An empty schedule is an empty
    /// chain.
    #[test]
    fn equal_split_plan_is_one_plan_per_schedule_point() {
        let kv = KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.5, 1.0, 1.0, 1.0], 2).unwrap();
        let plans = equal_split_plan(&kv, 4).unwrap();
        assert_eq!(plans.len(), 6);
        let last = plans.last().expect("six insertions");
        assert_eq!(
            last.knots.knots(),
            [
                0.0, 0.0, 0.0, 0.125, 0.25, 0.375, 0.5, 0.625, 0.75, 0.875, 1.0, 1.0, 1.0
            ]
        );
        assert!(equal_split_plan(&kv, 1).unwrap().is_empty());
    }

    /// The domain-uniform grid runs over the whole domain, blind to
    /// the spans: on `[0, 1]` with knots at `0.5` and one ulp above
    /// `0.25`, the quarters grid is `0.25, 0.75` under the bit-equal
    /// skip (`0.5` is a knot; `0.25` is not) and `0.75` alone under an
    /// 8-ulp clearance. A vector already finer than the grid is still
    /// given it, and `pieces` of 0 or 1 yields none.
    #[test]
    fn domain_grid_points_skip_rules_and_no_cut_off() {
        let near = f64::from_bits(0.25f64.to_bits() + 1);
        let kv = KnotVector::clamped(vec![0.0, 0.0, 0.0, near, 0.5, 1.0, 1.0, 1.0], 2).unwrap();
        assert_eq!(domain_grid_points(&kv, 4, GridSkip::BitEqual), [0.25, 0.75]);
        assert_eq!(
            domain_grid_points(&kv, 4, GridSkip::WithinUlps(SLIVER_CLEARANCE_ULPS)),
            [0.75]
        );
        // The clearance scales with the domain's width: on `[0, 4]` a
        // knot 4 ulps of `1.0` off the grid point `1.0` is inside it.
        let off = f64::from_bits(1.0f64.to_bits() + 4);
        let wide = KnotVector::clamped(vec![0.0, 0.0, off, 4.0, 4.0], 1).unwrap();
        assert_eq!(
            domain_grid_points(&wide, 4, GridSkip::BitEqual),
            [1.0, 2.0, 3.0]
        );
        assert_eq!(
            domain_grid_points(&wide, 4, GridSkip::WithinUlps(SLIVER_CLEARANCE_ULPS)),
            [2.0, 3.0]
        );
        // Ten spans, a grid of four: every quarter is still offered.
        let fine = KnotVector::clamped(
            vec![
                0.0, 0.0, 0.05, 0.15, 0.35, 0.45, 0.55, 0.65, 0.8, 0.9, 0.95, 1.0, 1.0,
            ],
            1,
        )
        .unwrap();
        assert_eq!(fine.control_count(), 11);
        assert_eq!(
            domain_grid_points(&fine, 4, GridSkip::BitEqual),
            [0.25, 0.5, 0.75]
        );
        assert!(domain_grid_points(&kv, 1, GridSkip::BitEqual).is_empty());
        assert!(domain_grid_points(&kv, 0, GridSkip::BitEqual).is_empty());
        // A non-dyadic domain pins the grid ARITHMETIC: `lo + (hi − lo)·(k/n)`
        // rounds to these values, and the counted-from-the-top form
        // `hi − (hi − lo)·((n − k)/n)` to others (`0.15999999999999992`,
        // `0.39999999999999997`, …). The knot at `0.4` is the grid's own
        // point, so the bit-equal skip drops it.
        let odd = KnotVector::clamped(vec![0.1, 0.1, 0.4, 0.7, 0.7], 1).unwrap();
        assert_eq!(
            domain_grid_points(&odd, 10, GridSkip::BitEqual),
            [
                0.16,
                0.22,
                0.28,
                0.339_999_999_999_999_97,
                0.459_999_999_999_999_96,
                0.52,
                0.58,
                0.64
            ]
        );
        // n = 13 separates the step forms that n = 10 does not:
        // `lo + ((hi − lo)·k)/n` and `lo + k·((hi − lo)/n)` give
        // `0.1923076923076923` at k = 2, `0.23846153846153845` at k = 3
        // and `0.6538461538461537` at k = 12.
        assert_eq!(
            domain_grid_points(&odd, 13, GridSkip::BitEqual),
            [
                0.146_153_846_153_846_16,
                0.192_307_692_307_692_32,
                0.238_461_538_461_538_47,
                0.284_615_384_615_384_6,
                0.330_769_230_769_230_8,
                0.376_923_076_923_076_9,
                0.423_076_923_076_923,
                0.469_230_769_230_769_23,
                0.515_384_615_384_615_3,
                0.561_538_461_538_461_5,
                0.607_692_307_692_307_6,
                0.653_846_153_846_153_9
            ]
        );
        // A finite width near `f64::MAX`: the clearance is
        // `|hi − lo|·(ulps·ε)`, finite, so a grid point clear of the
        // knot stands. Spelled `(|hi − lo|·ulps)·ε` the product
        // overflows to `∞` first and every point is dropped.
        let big = f64::MAX / 2.0;
        let huge = KnotVector::clamped(vec![-big, -big, 0.5 * big, big, big], 1).unwrap();
        assert_eq!(
            domain_grid_points(&huge, 4, GridSkip::WithinUlps(SLIVER_CLEARANCE_ULPS)),
            [-0.5 * big, 0.0]
        );
        // `WithinUlps(0)` is the bit-equal rule.
        assert_eq!(
            domain_grid_points(&odd, 10, GridSkip::WithinUlps(0)),
            domain_grid_points(&odd, 10, GridSkip::BitEqual)
        );
    }

    fn apply_chain(plans: &[CurvePlan], x: &[f64]) -> Vec<f64> {
        let mut cur = x.to_vec();
        for plan in plans {
            cur = plan.apply_points(&cur, f64::NAN, lerp1);
        }
        cur
    }

    /// **The two arithmetics stay in step, and the interval one stays where
    /// the point one lands.** The interval applier reads the SAME
    /// [`Step`] list as the point applier, so "same targets, same
    /// sources" is structural rather than tested; what a row can break
    /// is the arithmetic that hangs off it, and these four claims are
    /// the ones a wrong ratio or a wrong association falsifies.
    ///
    /// 1. **Same extent.** The two appliers answer the same length, at
    ///    every degree and every split count.
    /// 2. **A carry is the coefficient itself**, bitwise: nothing is
    ///    combined, so nothing rounds, and an enclosure wider than a
    ///    point would mean interval arithmetic applier had touched a carry.
    /// 3. **Each slot sits where the point applier lands.** Both
    ///    appliers compute the same refined coefficient, the point one
    ///    to a few ulps per insertion, so every `f64` answer lies within
    ///    claim 4's allowance of its interval slot. A ratio of the wrong
    ///    sign or a swapped association moves the slot by the
    ///    coefficients' own scale (they alternate in sign), which this
    ///    sees and a hull test does not: the step's meet with its
    ///    sources' hull keeps every slot inside the described hull by
    ///    construction.
    /// 4. **The fold does not inflate.** With point inputs, the widths
    ///    the whole chain accumulates come only from the ratios' own
    ///    rounding, so they stay at the scale of the coefficients times
    ///    a few ulps per insertion — not a multiple per insertion. The
    ///    ceiling is stated against the insertion COUNT for that
    ///    reason; a per-step growth factor blows through it at once, and
    ///    **this is the claim the lerp form reds on** (355 ulps against
    ///    the 248 this allows, at `p=2`, 30 insertions), not claim 3.
    #[test]
    fn the_ring_applier_stays_in_step_with_the_point_applier() {
        let cases: &[(usize, &[f64])] = &[
            (1, &[]),
            (2, &[1.0]),
            (2, &[1.0, 2.0]),
            (3, &[1.0, 1.0, 2.0]),
            (4, &[1.0, 2.0, 2.0, 3.0]),
        ];
        let mut worst_width_ulps = 0.0f64;
        let mut worst_gap_ulps = 0.0f64;
        for &(p, interior) in cases {
            let mut knots = vec![0.0; p + 1];
            knots.extend_from_slice(interior);
            let top = interior.last().copied().unwrap_or(0.0) + 1.0;
            knots.extend(core::iter::repeat_n(top, p + 1));
            let kv = KnotVector::clamped(knots, p).unwrap();
            let n = kv.control_count();
            // Coefficients of one homogeneous channel, O(1) and of both
            // signs so a sign error in a ratio escapes the hull.
            #[allow(clippy::cast_precision_loss)]
            let coeffs: Vec<f64> = (0..n)
                .map(|i| if i % 2 == 0 { i as f64 } else { -(i as f64) })
                .collect();
            let input: Vec<Interval> = coeffs.iter().copied().map(Interval::point).collect();
            let scale = coeffs.iter().fold(0.0f64, |m, c| m.max(c.abs())).max(1.0);
            for splits in [2usize, 3, 8, 16] {
                let plans = equal_split_plan(&kv, splits).unwrap();
                let f64_out = apply_chain(&plans, &coeffs);
                let mut ring_out = input.clone();
                for plan in &plans {
                    ring_out = plan.apply_certified(&ring_out);
                }
                let tag = format!("p={p} interior={interior:?} splits={splits}");
                assert_eq!(ring_out.len(), f64_out.len(), "{tag}: extent");
                // Claims 3 and 4's allowance: the width a non-inflating fold
                // may accumulate over `plans.len()` insertions (one plan
                // per insertion), in ulps of the coefficient scale.
                #[allow(clippy::cast_precision_loss)]
                let ceiling_ulps = 8.0 * (plans.len() + 1) as f64;
                for (i, r) in ring_out.iter().enumerate() {
                    assert!(r.is_certified(), "{tag}: slot {i} refused");
                    let f = f64_out[i];
                    let gap = (r.lo() - f).max(f - r.hi()).max(0.0) / (scale * f64::EPSILON);
                    worst_gap_ulps = worst_gap_ulps.max(gap);
                    assert!(
                        gap <= ceiling_ulps,
                        "{tag}: slot {i} = [{:.17e}, {:.17e}] is {gap:.1} ulps of scale from \
                         the point applier's {f:.17e}, above {ceiling_ulps:.0}",
                        r.lo(),
                        r.hi(),
                    );
                    worst_width_ulps = worst_width_ulps.max(r.width() / (scale * f64::EPSILON));
                }
                // Claim 2: a carried slot is still a point. The clamped
                // ends are carried by every insertion, so slot 0 and the
                // last slot are carries at every case here.
                for i in [0, ring_out.len() - 1] {
                    assert_eq!(ring_out[i].width(), 0.0, "{tag}: carry {i} widened");
                }
                // Claim 4, against the insertion count rather than a
                // fixed number: a fold that inflated per step would be
                // exponential in `plans.len()` and blow through this.
                let worst = ring_out
                    .iter()
                    .fold(0.0f64, |m, r| m.max(r.width() / (scale * f64::EPSILON)));
                assert!(
                    worst <= ceiling_ulps,
                    "{tag}: widest refined coefficient is {worst:.1} ulps of the \
                     coefficient scale over {} insertions, above the {ceiling_ulps:.0} a \
                     non-inflating fold allows",
                    plans.len()
                );
            }
        }
        println!(
            "ring refinement: widest coefficient {worst_width_ulps:.2} ulps of scale, \
             farthest point answer {worst_gap_ulps:.2}"
        );
    }

    /// **A constant column stays its point.** A refined coefficient is a
    /// convex combination of two described ones, so in ℝ it lies between
    /// them, and with EQUAL adjacent coefficients it equals them. `α` and
    /// `β` round outward independently, so `β·c + α·c` alone is a
    /// bracket straddling `c`; the step's meet with its sources' hull is
    /// what brings it back to `c`, and a constant column is the one
    /// input where any width at all is visible as an excursion.
    ///
    /// That exactness is load-bearing downstream: a polynomial curve's
    /// weight channel is `w ≡ 1`, and the tensor composite's residual
    /// `N_d·W_C − A_d·N_w` cancels against it. A `W` that came out of
    /// the fold as a bracket loosened the plane × NURBS envelope by
    /// 0.5% at a 1e-12 m residual.
    ///
    /// Both shapes are here for a reason. The degree-1, 15-insertion
    /// column is the deep fold; the degree-2, 2-insertion one is the
    /// shallow fold at three coefficient magnitudes, including a
    /// negative, where a sign error in either ratio would show instead.
    #[test]
    fn the_convex_form_holds_a_constant_column_exactly() {
        // (degree, interior knots, splits, coefficient)
        let cases: &[(usize, &[f64], usize, f64)] = &[
            (1, &[], 16, 0.5),
            (2, &[], 3, 1.0),
            (2, &[], 3, 0.5),
            (2, &[], 3, -3.0),
            (3, &[0.5], 4, 7.25),
        ];
        for &(p, interior, splits, c) in cases {
            let mut knots = vec![0.0; p + 1];
            knots.extend_from_slice(interior);
            knots.extend(core::iter::repeat_n(1.0, p + 1));
            let kv = KnotVector::clamped(knots, p).unwrap();
            let plans = equal_split_plan(&kv, splits).unwrap();
            let mut out: Vec<Interval> = vec![Interval::point(c); kv.control_count()];
            for plan in &plans {
                out = plan.apply_certified(&out);
            }
            for (i, r) in out.iter().enumerate() {
                assert!(r.is_certified(), "p={p} c={c}: refused slot {i}");
                assert!(
                    r.lo() == c && r.hi() == c,
                    "p={p} c={c}, {} insertions: slot {i} is [{:e}, {:e}], not the point \
                     {c} — the step reached past the hull of its two sources",
                    plans.len(),
                    r.lo(),
                    r.hi()
                );
            }
        }
    }

    #[test]
    fn insertion_is_evaluation_invariant() {
        let (kv, w, x) = fixture();
        for (u, times) in [(0.5, 1), (1.0, 1), (2.5, 2), (1.5, 2)] {
            let plans = insert_knot_plan(&kv, &w, u, times).unwrap();
            let x2 = apply_chain(&plans, &x);
            let last = plans.last().unwrap();
            assert_eq!(x2.len(), kv.control_count() + times);
            for i in 0..=60 {
                let t = 3.0 * f64::from(i) / 60.0;
                let a = eval1(&kv, &w, &x, t);
                let b = eval1(last.knots(), last.weights(), &x2, t);
                // Tight-but-not-bitwise: floating point moves under
                // re-association; the values agree to ~1e-13 of the
                // O(1) coordinate scale here.
                assert!(
                    (a - b).abs() < 1e-12,
                    "u={u} times={times} t={t}: {a} vs {b}"
                );
            }
        }
    }

    #[test]
    fn insertion_refusals_are_typed() {
        let (kv, w, x) = fixture();
        let _ = x;
        assert_eq!(
            insert_knot_plan(&kv, &w, 3.5, 1).unwrap_err(),
            KnotAlgebraError::ParameterOutsideDomain { u: 3.5 }
        );
        assert_eq!(
            insert_knot_plan(&kv, &w, 0.0, 1).unwrap_err(),
            KnotAlgebraError::ParameterOutsideDomain { u: 0.0 }
        );
        // The refusal names the parameter as a number a reader can
        // see: at the ceiling of the range that is `1e308`, not the
        // 309-digit positional expansion.
        assert_eq!(
            insert_knot_plan(&kv, &w, 1e308, 1).unwrap_err().to_string(),
            "knot parameter 1e308 is not strictly inside the domain"
        );
        assert_eq!(
            insert_knot_plan(&kv, &w, 1.0, 2).unwrap_err(),
            KnotAlgebraError::MultiplicityOverflow {
                u: 1.0,
                have: 1,
                budget: 2
            }
        );
        assert_eq!(
            insert_knot_plan(&kv, &[1.0, 1.0], 0.5, 1).unwrap_err(),
            KnotAlgebraError::Structure(SplineError::WeightCountMismatch {
                weights: 2,
                control: 5
            })
        );
        assert_eq!(
            insert_knot_plan(&kv, &[1.0, -1.0, 1.0, 1.0, 1.0], 0.5, 1).unwrap_err(),
            KnotAlgebraError::Structure(SplineError::NonPositiveWeight {
                index: 1,
                weight: -1.0
            })
        );
    }

    #[test]
    fn refinement_is_evaluation_invariant_and_sorts() {
        let (kv, w, x) = fixture();
        let plans = refine_plan(&kv, &w, &[2.5, 0.5, 1.5, 0.5]).unwrap();
        let x2 = apply_chain(&plans, &x);
        let last = plans.last().unwrap();
        assert_eq!(last.knots().knots().len(), kv.knots().len() + 4);
        for i in 0..=60 {
            let t = 3.0 * f64::from(i) / 60.0;
            let a = eval1(&kv, &w, &x, t);
            let b = eval1(last.knots(), last.weights(), &x2, t);
            assert!((a - b).abs() < 1e-12, "t={t}: {a} vs {b}");
        }
    }

    #[test]
    fn insert_then_remove_round_trips_evaluation() {
        let (kv, w, x) = fixture();
        let ins = insert_knot_plan(&kv, &w, 1.4, 2).unwrap();
        let xi = apply_chain(&ins, &x);
        let last_ins = ins.last().unwrap();
        let rem = remove_knot_plan(last_ins.knots(), last_ins.weights(), 1.4, 2).unwrap();
        let mut xr = xi;
        let mut final_kv = last_ins.knots().clone();
        let mut final_w = last_ins.weights().to_vec();
        for step in &rem {
            xr = step.plan.apply_points(&xr, f64::NAN, lerp1);
            final_kv = step.plan.knots().clone();
            final_w = step.plan.weights().to_vec();
        }
        assert_eq!(final_kv.knots(), kv.knots());
        for i in 0..=60 {
            let t = 3.0 * f64::from(i) / 60.0;
            let a = eval1(&kv, &w, &x, t);
            let b = eval1(&final_kv, &final_w, &xr, t);
            assert!((a - b).abs() < 1e-10, "t={t}: {a} vs {b}");
        }
    }

    #[test]
    fn removal_refusals_are_typed() {
        let (kv, w, _x) = fixture();
        assert_eq!(
            remove_knot_plan(&kv, &w, 0.25, 1).unwrap_err(),
            KnotAlgebraError::KnotNotPresent { u: 0.25 }
        );
        assert_eq!(
            remove_knot_plan(&kv, &w, 0.0, 1).unwrap_err(),
            KnotAlgebraError::KnotNotPresent { u: 0.0 }
        );
        assert_eq!(
            remove_knot_plan(&kv, &w, 1.0, 2).unwrap_err(),
            KnotAlgebraError::RemovalExceedsMultiplicity {
                u: 1.0,
                have: 1,
                requested: 2
            }
        );
    }

    #[test]
    fn elevation_is_evaluation_invariant() {
        let (kv, w, x) = fixture();
        let plans = elevate_plan(&kv, &w).unwrap();
        let x2 = apply_chain(&plans, &x);
        let last = plans.last().unwrap();
        assert_eq!(last.knots().degree(), kv.degree() + 1);
        // Elevation adds one copy of every distinct value: interior
        // count 2 (values 1, 2) + both ends ⇒ +4 knots, +3 control.
        assert_eq!(last.knots().knots().len(), kv.knots().len() + 4);
        assert_eq!(x2.len(), x.len() + 3);
        for i in 0..=90 {
            let t = 3.0 * f64::from(i) / 90.0;
            let a = eval1(&kv, &w, &x, t);
            let b = eval1(last.knots(), last.weights(), &x2, t);
            assert!((a - b).abs() < 1e-10, "t={t}: {a} vs {b}");
        }
    }

    #[test]
    fn determinism_bitwise_across_repeats() {
        let (kv, w, x) = fixture();
        let p1 = insert_knot_plan(&kv, &w, 1.7, 2).unwrap();
        let p2 = insert_knot_plan(&kv, &w, 1.7, 2).unwrap();
        let x1 = apply_chain(&p1, &x);
        let x2 = apply_chain(&p2, &x);
        let bits = |v: &[f64]| v.iter().map(|f| f.to_bits()).collect::<Vec<_>>();
        assert_eq!(bits(&x1), bits(&x2));
        assert_eq!(
            bits(p1.last().unwrap().weights()),
            bits(p2.last().unwrap().weights())
        );
        assert_eq!(p1.last().unwrap().knots(), p2.last().unwrap().knots());
    }
}
