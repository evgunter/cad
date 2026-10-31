//! Knot-algebra **plans**: insertion, refinement, removal, and degree
//! elevation, DECIDED as `f64` STRUCTURE on the weights' and knots'
//! value channel (new knot vectors, the combination schedule, and
//! every refusal), and applied to a net held at the caller's scalar by
//! a tiny fold ([`CurvePlan::apply_rational`]). Where a carrier's held
//! knots or weights are wider than a point, the channel is a stand-in
//! (`super::plan_inputs`, C6 S5b) after the carrier's parameter
//! decisions are certified for every member: the plan's structure and
//! schedule are the operation's, its values and its weight refusals the
//! stand-in's, and it is marked ([`CurvePlan::is_reference`]).
//!
//! # Why plans
//!
//! Every algorithm here (Book §5.2–§5.5) combines control points with
//! coefficients that depend only on knots and weights. Splitting each
//! operation into a structure-computed *plan* plus a generic *applier*
//! keeps the C6 boundary a module boundary: raw `f64` comparisons live
//! in the plan constructors, on the value channel, while the applier
//! forms each step's coefficient, weight and `λ` at its scalar from the
//! held knots and weights and combines (`lerp`, fixed association
//! `x + (y − x)·λ`) in plan order. One plan implementation serves 2-D
//! curves, 3-D curves, and each row/column of a surface net.
//!
//! # Projective form (rational)
//!
//! All combinations are homogeneous: a two-term combo
//! `Q = a·(wᵢPᵢ, wᵢ) + b·(wⱼPⱼ, wⱼ)` becomes the structure weight
//! `w_Q = a·wᵢ + b·wⱼ` and the **affine point combination**
//! `P_Q = lerp(Pⱼ, Pᵢ, λ)` with `λ = a·wᵢ / w_Q` — exact in ℝ because
//! the two projective coefficients sum to 1 by construction. A plan
//! records the step's form (`Form`) and the constructor takes it on
//! the `f64` channel ([`CurvePlan::weights`]); the applier takes the
//! same form on the held values.
//!
//! # One schedule, two arithmetics
//!
//! A plan (insertion, removal or elevation) is applied by
//! [`CurvePlan::apply_rational`] at an evaluation scalar
//! ([`ProjectiveScalar`]), by
//! [`CurvePlan::apply_points`] on its own `f64` channel, by
//! [`CurvePlan::apply_certified`] in the certification ring, and by
//! `CurvePlan::apply_homogeneous` (in `projective`, outside this
//! certification file) on `(w·P, w)` at any scalar, the curve's knot
//! operations at an enclosure scalar, off the SAME [`Step`] list — same targets, same sources, same
//! order. They differ in the coefficient the combination is taken
//! with, and they must: the projective appliers' `λ` is a quotient of
//! weights (formed at `T` from the held weights, or the plan's `f64`),
//! while the ring applier re-derives each step's coefficient from the
//! exact values it is made of ([`Step::Combo`]'s `ratio`: the Boehm
//! ratios `α = (u − U_j)/Δ` and `β = (U_{j+p} − u)/Δ`, a removal's `α_i`,
//! elevation's `i/(p + 1)`) so that it rounds OUTWARD. A ring consumer that took the stored `λ` and padded it by a
//! guessed number of ulps would be asserting a bound nobody derived;
//! re-deriving the ratios in certification arithmetic makes the insertion widen like
//! every other step of an enclosure. Interval arithmetic applier takes HOMOGENEOUS
//! coefficients, where the combination is the plain two-point one
//! ([`Certification::between`]) and no `λ` is needed.

use super::knots::{InteriorKnot, KnotVector, SplineError, find_span_in};
use super::projective::{ProjectivePoint, ProjectiveScalar};
use crate::interval::Interval;
use crate::interval::certification::Certification;
use crate::readable::Readable;
use crate::real::Bounds;

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
    /// At an enclosure scalar, whether a parameter the operation
    /// inserts or removes is a breakpoint, or in which span it lies, is
    /// not the same for every member: run `run`'s bracket holds `u`
    /// (C6 S5b). The margin is `u`'s distance to that bracket; knots
    /// narrow enough that it clears zero lift the refusal.
    ParameterUndecided {
        /// The parameter.
        u: f64,
        /// The run whose bracket holds it.
        run: usize,
        /// `u`'s distance to that run's bracket, rounded outward.
        margin: crate::MarginDiag,
    },
    /// A plan decided against a stand-in for wide held knots or weights
    /// (C6 S5b, [`CurvePlan::is_reference`]) reached an applier that
    /// reads its `f64` values, which are the stand-in's and no value of
    /// the carrier's.
    StandInPlan,
    /// A plan wrote a non-positive or non-finite weight: a removal
    /// chain left the positive-weight regime, or an insertion's or
    /// elevation's combination of two positive weights rounded to zero
    /// (both terms underflowing, on a net of subnormal weights). The
    /// operation is refused rather than returning an invalid curve.
    WeightCollapse {
        /// New-polygon index of the collapsed weight: in the polygon the
        /// writing pass builds. For a removal that is the removal's own
        /// polygon, or, when the bound's reinsertion
        /// ([`remove_knot_plan`] re-inserts `u` to measure the bound) is
        /// the writer, the reinserted polygon's slot.
        index: usize,
    },
    /// At an enclosure scalar, a control point or weight a plan wrote
    /// is not bounded: a removal's ratio, read from knots within
    /// rounding of each other, has an enclosure reaching `0` or `1`, and
    /// the unblend divides by it (C6 S4). Refused rather than handed to
    /// the carrier as an unbounded enclosure.
    UnboundedAtEnclosure {
        /// Index of the control point in the polygon the operation
        /// wrote.
        index: usize,
    },
}

impl core::fmt::Display for KnotAlgebraError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            KnotAlgebraError::Structure(e) => write!(f, "the knot edit refused: {e}"),
            KnotAlgebraError::ParameterOutsideDomain { u } => write!(
                f,
                "knot parameter {} is not strictly inside the domain. Recourse: ask for a \
                 finite parameter strictly between the clamped ends, whose multiplicity is \
                 already degree + 1",
                Readable(*u)
            ),
            KnotAlgebraError::MultiplicityOverflow { u, have, budget } => write!(
                f,
                "inserting knot {} (multiplicity {have}) exceeds the interior budget {budget}. \
                 Recourse: ask for at most {left} more cop{plural} of it, or raise the degree",
                Readable(*u),
                left = budget.saturating_sub(*have),
                plural = if budget.saturating_sub(*have) == 1 {
                    "y"
                } else {
                    "ies"
                },
            ),
            KnotAlgebraError::KnotNotPresent { u } => write!(
                f,
                "{} is not an interior knot. Recourse: ask for a value the knot vector \
                 carries — a clamped vector's end values are not removable",
                Readable(*u)
            ),
            KnotAlgebraError::RemovalExceedsMultiplicity { u, have, requested } => write!(
                f,
                "removing knot {} {requested} times exceeds its multiplicity {have}. \
                 Recourse: ask for at most {have} removals of it",
                Readable(*u)
            ),
            KnotAlgebraError::ParameterUndecided { u, run, margin } => write!(
                f,
                "whether knot parameter {} is a breakpoint, or in which span it lies, is \
                 undecided over the values the knots stand for: breakpoint {run}'s enclosure \
                 is {margin} from it. Recourse: supply knots narrow enough that the \
                 parameter clears that breakpoint, or a parameter that does",
                Readable(*u)
            ),
            KnotAlgebraError::StandInPlan => write!(
                f,
                "a knot operation's plan, decided against a stand-in for knots or weights \
                 wider than a point, reached an applier that reads its f64 values, which are \
                 the stand-in's. Recourse: apply it over the held values (the enclosure arm)"
            ),
            KnotAlgebraError::WeightCollapse { index } => write!(
                f,
                "the knot operation wrote weight {index} out of the positive regime. \
                 Recourse: drop this operation — the curve it would leave is not a \
                 valid rational one, so keep the curve as it is"
            ),
            KnotAlgebraError::UnboundedAtEnclosure { index } => write!(
                f,
                "the knot operation wrote control point {index} unbounded over the values it \
                 stands for: a removal's ratio, read from knots within rounding of each other, \
                 reaches 0 or 1. Recourse: evaluate at a point scalar, or keep the knot"
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

/// The exact `f64`s a combination's coefficient is made of,
/// `a = (inserted − lo)/(hi − lo)`, with `lo ≤ inserted ≤ hi` and
/// `lo < hi`:
/// - a Boehm insertion's `α`: `lo = U[j]`, `hi = U[j + p]` of the knot
///   vector the step was planned against, and the value inserted;
/// - a removal's `α_i`: `lo = U[i]`, `hi = U[i + p + 1]`, and the knot
///   removed;
/// - degree elevation's binomial `i/(p + 1)`: `lo = 0`, `hi = p + 1`,
///   `inserted = i`, small integers and so exact.
///
/// Carried as INGREDIENTS and not as values, for the ring applier
/// [`CurvePlan::apply_certified`], which takes one channel and no knots
/// and so needs them here, to form an outward-rounded ring quotient: a
/// stored `f64` ratio would leave it padding a rounded number by a
/// guess. The appliers at a scalar form the same coefficient from the
/// held knots instead, through the step's [`Coef`]; the two describe
/// one number, written together by each constructor.
#[derive(Clone, Copy, Debug)]
pub(super) struct Ratio {
    pub(super) inserted: f64,
    pub(super) lo: f64,
    pub(super) hi: f64,
}

/// Where a combination's coefficient `a` comes from, so an applier can
/// form it at its own scalar: a ratio of the input vector's knots,
/// `(u − U[lo])/(U[hi] − U[lo])`, or a combinatorial constant.
#[derive(Clone, Copy, Debug)]
pub(super) enum Coef {
    /// `(u − U[lo])/(U[hi] − U[lo])`, the knots read off the vector the
    /// step was planned against.
    Knots { u: KnotAt, lo: usize, hi: usize },
    /// Degree elevation's `i/(p + 1)`, a constant of the operation, as
    /// its two integers so an applier forms the quotient at its own
    /// scalar (exact at `Sym`; at `f64` the same bits as `i/(p + 1)`).
    Constant { num: f64, den: f64 },
}

/// The `u` of a [`Coef::Knots`] ratio: a knot of the input vector (a
/// removal's, or an insertion at a breakpoint already present, so the
/// ratio reads that breakpoint's one quantity), or a value the
/// insertion brings in.
#[derive(Clone, Copy, Debug)]
pub(super) enum KnotAt {
    Index(usize),
    Fresh(f64),
}

/// How a combination forms its weight `w_Q` and its projective `λ`
/// from the source weights `w_x`, `w_y` and its coefficient `a`; the
/// point is `lerp(x, y, λ)` in every form. The plan constructors take
/// these exact operations in `f64` on the weights' value channel, and
/// [`CurvePlan::apply_rational`] takes them again at the applier's
/// scalar, so at `f64` the two agree bit for bit.
#[derive(Clone, Copy, Debug)]
pub(super) enum Form {
    /// Insertion and elevation: `w_Q = a·w_y + (1 − a)·w_x`,
    /// `λ = a·w_y/w_Q`.
    Blend(Coef),
    /// Removal's forward chain: `w_Q = (w_y − (1 − a)·w_x)/a`,
    /// `λ = (w_y/a)/w_Q`.
    UnblendForward(Coef),
    /// Removal's backward chain: `w_Q = (w_y − a·w_x)/(1 − a)`,
    /// `λ = (w_y/(1 − a))/w_Q`.
    UnblendBackward(Coef),
}

/// One plan step: assign new-polygon slot `target`.
#[derive(Clone, Debug)]
enum Step {
    /// `new[target] = old[from]`.
    Carry { target: usize, from: usize },
    /// `new[target] = lerp(x, y, lambda)` — fixed association
    /// `x + (y − x)·λ` with the caller's lifted `λ`.
    ///
    /// `ratio` is the step's coefficient as its exact ingredients
    /// ([`Ratio`]), which [`CurvePlan::apply_certified`] forms in the
    /// ring and combines by `form`: an insertion's and an elevation's
    /// convex blend, a removal's unblend.
    Combo {
        target: usize,
        x: Src,
        y: Src,
        lambda: f64,
        ratio: Ratio,
        form: Form,
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
    /// Whether the plan's `f64` values (its knot vector's values, its
    /// weights, each step's `λ` and ring ingredients) are its inputs'
    /// own. A plan decided against a stand-in vector for wide held knots
    /// (`Knots::plan_proxy`, C6 S5b) has the right schedule and
    /// structure but a stand-in's values, so the appliers that read them
    /// ([`Self::apply_points`], [`Self::apply_certified`]) refuse it;
    /// the appliers at a scalar read the held values and run it.
    reference: bool,
}

impl CurvePlan {
    /// This plan marked as decided against a stand-in vector (the
    /// `reference` field's docs).
    pub(super) fn without_reference(mut self) -> Self {
        self.reference = false;
        self
    }

    /// Whether the plan's `f64` values are its inputs' own (the
    /// `reference` field's docs): a plan decided against point knots and
    /// weights.
    #[must_use]
    pub fn is_reference(&self) -> bool {
        self.reference
    }

    /// The knot vector after this plan. For a plan that is no reference
    /// ([`Self::is_reference`]) its structure is the operation's, its
    /// values a stand-in's.
    pub fn knots(&self) -> &KnotVector {
        &self.knots
    }

    /// The weights after this plan (all strictly positive — checked at
    /// plan construction). For a plan that is no reference
    /// ([`Self::is_reference`]) they are the unit stand-in's, so the
    /// check holds nothing about the carrier's weights: its door does.
    pub fn weights(&self) -> &[f64] {
        &self.weights
    }

    /// Applies the point schedule on the plan's own `f64` channel:
    /// `old` is the previous control polygon, `lerp(x, y, λ)` the
    /// caller's affine combination (`x + (y − x)·λ`, the fixed
    /// association every consumer documents) with the plan's `f64` `λ`,
    /// and `poison` the caller's poison point — the total fallback for
    /// a malformed plan, which the constructors rule out but the
    /// applier does not trust (D4: fail loud, never panic). It is the
    /// `f64` reference [`CurvePlan::apply_rational`] is pinned against;
    /// a carrier at any scalar is refined by that applier, over its
    /// held knots and weights, and the certification ring's applier is
    /// [`CurvePlan::apply_certified`]. `P` is a [`ProjectivePoint`]
    /// over `f64`, so ring enclosures cannot be combined with the plan's
    /// `f64` `λ` here either.
    pub fn apply_points<P: ProjectivePoint<f64>>(
        &self,
        old: &[P],
        poison: P,
        lerp: impl Fn(P, P, f64) -> P,
    ) -> Vec<P> {
        let n_new = self.knots.control_count();
        if !self.reference {
            return vec![poison; n_new];
        }
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

    /// **The schedule over a rational net held at `T`**: the points
    /// AND the weights combine at the caller's scalar, each step's
    /// coefficient formed in `T` from `knots`, the input vector's knot
    /// values (one per knot of the vector this plan was made against),
    /// and each new weight and projective `λ` formed in `T` from the
    /// source weights by the step's form — the operations, in the
    /// order, the plan constructor took on the weights' `f64` value
    /// channel. So at `f64` the result is [`Self::apply_points`] and
    /// [`Self::weights`] bit for bit, and at `Sym` a weight or a knot
    /// that is a function of a document's variables stays that function
    /// through the operation.
    ///
    /// The plan's structure (its steps, and every refusal its
    /// constructor ran on the value channel) is the `f64` decision; this
    /// applies it, and decides nothing.
    ///
    /// A value the step brings in that the input vector does not hold
    /// (an insertion of a knot not present) is `fresh(v)`: the caller's
    /// entry of it, which is the constant it is for an insertion at the
    /// caller's `f64` parameter, and the held value a removal took out
    /// when its bound reinserts it.
    ///
    /// Total: a malformed plan, a short channel or a knot index past
    /// `knots` leaves `poison` and `T`'s poison weight
    /// (`from_f64(NaN)`) in the targets it reaches.
    pub fn apply_rational<T: ProjectiveScalar, P: ProjectivePoint<T>>(
        &self,
        knots: &[T],
        old: &[P],
        old_w: &[T],
        (poison, fresh): (P, &dyn Fn(f64) -> T),
        lerp: impl Fn(P, P, T) -> P,
    ) -> (Vec<P>, Vec<T>) {
        let n_new = self.knots.control_count();
        let mut new: Vec<Option<(P, T)>> = vec![None; n_new];
        let fetch = |new: &[Option<(P, T)>], s: Src| -> Option<(P, T)> {
            match s {
                Src::Old(i) => Some((*old.get(i)?, *old_w.get(i)?)),
                Src::New(i) => new.get(i).copied().flatten(),
            }
        };
        let coef = |c: Coef| super::projective::form_coef(c, knots, fresh);
        let one = T::from_f64(1.0);
        for step in &self.steps {
            match *step {
                Step::Carry { target, from } => {
                    if target < n_new {
                        new[target] = fetch(&new, Src::Old(from));
                    }
                }
                Step::Combo {
                    target, x, y, form, ..
                } => {
                    if target < n_new {
                        new[target] = (|| {
                            let ((px, wx), (py, wy)) = (fetch(&new, x)?, fetch(&new, y)?);
                            let (wq, lambda) = match form {
                                Form::Blend(c) => {
                                    let a = coef(c)?;
                                    let wq = a * wy + (one - a) * wx;
                                    (wq, a * wy / wq)
                                }
                                Form::UnblendForward(c) => {
                                    let a = coef(c)?;
                                    let wq = (wy - (one - a) * wx) / a;
                                    (wq, (wy / a) / wq)
                                }
                                Form::UnblendBackward(c) => {
                                    let a = coef(c)?;
                                    let b = one - a;
                                    let wq = (wy - a * wx) / b;
                                    (wq, (wy / b) / wq)
                                }
                            };
                            Some((lerp(px, py, lambda), wq))
                        })();
                    }
                }
            }
        }
        let refused = T::from_f64(f64::NAN);
        new.into_iter()
            .map(|slot| slot.unwrap_or((poison, refused)))
            .unzip()
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
    /// # The step: [`Certification::between`] at a ratio from the knots
    ///
    /// (An insertion's and an elevation's blend; a removal's unblend is
    /// below.)
    ///
    /// With `Δ = U_{j+p} − U_j`, positive by the insertion precondition
    /// (`insert_once`'s band comment, so the quotient never refuses on a
    /// valid plan), each combination is
    /// `Certification::between(C_{j−1}, C_j, (u − U_j)/Δ)`: the ratio is
    /// a ring quotient of the knots it is made of, rounded outward, and
    /// the true ratio is in `[0, 1]` because `U_j ≤ u ≤ U_{j+p}` — the
    /// door's precondition. The door's docs are the one home of why the
    /// monotone form is the step and why it is met with its sources'
    /// hull; what is local here is only that the ratio is re-derived
    /// from the knots, never taken from the projective `λ`.
    ///
    /// **One width allowance per fixture.** This module's
    /// `the_ring_applier_stays_in_step_with_the_point_applier` allows
    /// `8.0·(plans.len() + 1)` ulps over equal-split plan chains; neither
    /// it nor the exact-rational rows in `compose`'s tests is a derived
    /// bound. They are ceilings a form that multiplies its width per
    /// step cannot meet.
    ///
    /// # Elevation and removal
    ///
    /// Degree elevation's binomial `i/(p + 1)` is a convex blend too,
    /// taken by the same step at the ring quotient of its two integers.
    /// A removal unblends: forward `Q = (C_y − (1 − a)·C_x)/a`, backward
    /// `Q = (C_y − a·C_x)/(1 − a)`, taken as `C_x + (C_y − C_x)/a` and
    /// `C_x + (C_y − C_x)/(1 − a)` (the same in ℝ; a constant column
    /// stays exact), plain ring arithmetic at the ratio
    /// `a = (u − U_i)/(U_{i+p+1} − U_i)` of the knots it is made of,
    /// which lies strictly inside `(0, 1)` for an interior knot
    /// (`remove_once`). It is not a convex combination, so it widens as
    /// the ring does; a ratio enclosure reaching `0` or `1` refuses
    /// through the division.
    ///
    /// **Total, and NaI is the refusal** (D4): a malformed plan or a
    /// channel of the wrong length refuses its targets. The refusal
    /// then flows through every hull the caller reads.
    pub fn apply_certified(&self, old: &[Interval]) -> Vec<Interval> {
        if !self.reference {
            return vec![Interval::refused(); self.knots.control_count()];
        }
        self.apply_ratios(old, Interval::refused(), |x, y, r, form| {
            Some(match form {
                Form::Blend(_) => boehm_step(x, y, r.lo, r.hi, r.inserted),
                Form::UnblendForward(_) => x + (y - x) / ring_ratio(r),
                Form::UnblendBackward(_) => x + (y - x) / (Interval::point(1.0) - ring_ratio(r)),
            })
        })
    }

    /// The fold both homogeneous appliers are: `Carry` copies, a
    /// combination combines through `step`, and a malformed plan or a
    /// short channel leaves `refused` in its target.
    pub(super) fn apply_ratios<P: Copy>(
        &self,
        old: &[P],
        refused: P,
        step: impl Fn(P, P, Ratio, Form) -> Option<P>,
    ) -> Vec<P> {
        let n_new = self.knots.control_count();
        let mut new: Vec<Option<P>> = vec![None; n_new];
        let fetch = |new: &[Option<P>], s: Src| -> Option<P> {
            match s {
                Src::Old(i) => old.get(i).copied(),
                Src::New(i) => new.get(i).copied().flatten(),
            }
        };
        for plan_step in &self.steps {
            match *plan_step {
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
                    form,
                    ..
                } => {
                    if target < n_new {
                        new[target] = match (fetch(&new, x), fetch(&new, y)) {
                            (Some(cx), Some(cy)) => step(cx, cy, ratio, form),
                            _ => None,
                        };
                    }
                }
            }
        }
        new.into_iter()
            .map(|slot| slot.unwrap_or(refused))
            .collect()
    }
}

/// One Boehm combination in the certification ring:
/// [`Certification::between`] at the ratio `(u − lo)/(hi − lo)`, a ring
/// quotient of the knots.
///
/// **Precondition: `lo`, `u` and `hi` are exact `f64` knots with
/// `lo ≤ u ≤ hi` and `lo < hi`.** The insertions place `u` strictly
/// inside the span `[U_j, U_{j+p}]` of the vector they plan against
/// (`insert_once`'s band), so the quotient never refuses and the true
/// ratio is in `[0, 1]`, which is the door's precondition.
pub(super) fn boehm_step(x: Interval, y: Interval, lo: f64, hi: f64, u: f64) -> Interval {
    Interval::between(
        x,
        y,
        ring_ratio(Ratio {
            inserted: u,
            lo,
            hi,
        }),
    )
}

/// A step's coefficient `(inserted − lo)/(hi − lo)` in the ring, from
/// its exact ingredients ([`Ratio`]), rounded outward.
fn ring_ratio(r: Ratio) -> Interval {
    let lo = Interval::point(r.lo);
    (Interval::point(r.inserted) - lo) / (Interval::point(r.hi) - lo)
}

/// Validates weights against a knot vector: count, then the weight
/// regime ([`super::knots::check_weight_values`]) — the shared
/// precondition of every plan constructor.
fn check_weights(kv: &KnotVector, weights: &[f64]) -> Result<(), KnotAlgebraError> {
    if weights.len() != kv.control_count() {
        return Err(KnotAlgebraError::Structure(
            SplineError::WeightCountMismatch {
                weights: weights.len(),
                control: kv.control_count(),
            },
        ));
    }
    super::knots::check_weight_values(weights).map_err(KnotAlgebraError::Structure)
}

/// **The one admission of a weight a plan writes**: in the weight
/// regime ([`super::knots::weight_in_regime`]), or
/// [`KnotAlgebraError::WeightCollapse`] at its new-polygon `index`.
/// [`check_weights`] holds a plan's INPUT to the same regime, but a
/// combination of two admitted weights can still leave it (an
/// insertion's convex blend rounds to zero when both terms underflow; a
/// removal's unblend can go negative), so every writer — insertion,
/// elevation, removal — passes what it writes through here.
fn written_weight(index: usize, w: f64) -> Result<f64, KnotAlgebraError> {
    if super::knots::weight_in_regime(w) {
        Ok(w)
    } else {
        Err(KnotAlgebraError::WeightCollapse { index })
    }
}

/// Single knot insertion (Book §5.2, Boehm), `times`-fold: returns a
/// chain of plans, each built on the previous plan's structure. The
/// resulting multiplicity must stay within the interior budget
/// (`degree`); the parameter must be strictly inside the domain.
///
/// # Errors
///
/// [`KnotAlgebraError`] on structure mismatch, out-of-domain `u`, or
/// multiplicity overflow, and [`KnotAlgebraError::WeightCollapse`] when
/// an inserted weight rounds out of the positive regime ([`written_weight`]).
/// `times == 0` is a no-op (empty chain).
pub fn insert_knot_plan(
    kv: &KnotVector,
    weights: &[f64],
    u: f64,
    times: usize,
) -> Result<Vec<CurvePlan>, KnotAlgebraError> {
    check_weights(kv, weights)?;
    let Some(knot) = kv.interior_knot(u) else {
        return Err(KnotAlgebraError::ParameterOutsideDomain { u });
    };
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
        let plan = insert_once(&cur_kv, &cur_w, knot)?;
        cur_kv = plan.knots.clone();
        cur_w = plan.weights.clone();
        plans.push(plan);
    }
    Ok(plans)
}

/// One insertion pass — preconditions established by the callers
/// (`u` strictly interior, multiplicity budget available, weights
/// validated). `u` was minted against the chain's first vector; an
/// insertion leaves both clamp runs alone, so it is interior to every
/// vector of the chain.
fn insert_once(
    kv: &KnotVector,
    weights: &[f64],
    knot: InteriorKnot,
) -> Result<CurvePlan, KnotAlgebraError> {
    let p = kv.degree();
    let knots = kv.knots();
    let k = find_span_in(knots, p, knot);
    let u = knot.value();
    let s = kv.multiplicity_of(u).map_or(0, |(s, _)| s);
    // A breakpoint already present is `knots[k]` (the span's start), so
    // the ratio reads its one quantity; a new one is the value inserted.
    let inserted = if s > 0 {
        KnotAt::Index(k)
    } else {
        KnotAt::Fresh(u)
    };
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
        let wq = written_weight(j, alpha * weights[j] + (1.0 - alpha) * weights[j - 1])?;
        let lambda = alpha * weights[j] / wq;
        new_w[j] = wq;
        steps.push(Step::Combo {
            target: j,
            x: Src::Old(j - 1),
            y: Src::Old(j),
            lambda,
            ratio: Ratio {
                inserted: u,
                lo: knots[j],
                hi: knots[j + p],
            },
            form: Form::Blend(Coef::Knots {
                u: inserted,
                lo: j,
                hi: j + p,
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
    Ok(CurvePlan {
        knots: new_kv,
        weights: new_w,
        steps,
        reference: true,
    })
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
        add.extend(split_points(kv.knots()[span], kv.knots()[span + 1], splits));
    }
    add
}

/// The points cutting `[lo, hi]` into `splits` equal pieces,
/// `lo + (hi − lo)·(k/splits)` for `0 < k < splits`, ascending, less any
/// that floating point collapses onto an end: [`equal_split_points`]'
/// schedule on one interval. On an interval a few ulps wide two points
/// may round together; a caller that needs them distinct dedups.
/// `splits` of 0 or 1 yields none.
pub fn split_points(lo: f64, hi: f64, splits: usize) -> impl Iterator<Item = f64> {
    (1..splits).filter_map(move |k| {
        #[allow(clippy::cast_precision_loss)]
        let u = lo + (hi - lo) * (k as f64 / splits as f64);
        (u > lo && u < hi).then_some(u)
    })
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

/// **A knot refinement in the certification ring**: the insertion chain
/// from a vector, and the vector it ends on. The one spelling of a ring
/// refinement. It owns the refined vector ([`RingRefinement::knots`],
/// the source itself when nothing is inserted) and the extent guard
/// ([`RingRefinement::refine`] refuses a channel that is not one entry
/// per source control point), so no caller re-derives either.
///
/// A channel is homogeneous (`w·P` per coordinate, or `w`) or
/// unit-weight, which a plain convex combination refines; a rational
/// net goes through it as its homogeneous channels and comes back
/// through [`dehomogenize`].
#[derive(Clone, Debug)]
pub struct RingRefinement {
    plans: Vec<CurvePlan>,
    source: usize,
    knots: KnotVector,
}

impl RingRefinement {
    /// `add` inserted into `kv` ([`refine_plan_homogeneous`]).
    ///
    /// # Errors
    ///
    /// As [`refine_plan`].
    pub fn new(kv: &KnotVector, add: &[f64]) -> Result<Self, KnotAlgebraError> {
        Ok(Self::of_chain(kv, refine_plan_homogeneous(kv, add)?))
    }

    /// Every nonempty span of `kv` split into `splits` equal pieces
    /// ([`equal_split_plan`]).
    ///
    /// # Errors
    ///
    /// As [`refine_plan`].
    pub fn equal_split(kv: &KnotVector, splits: usize) -> Result<Self, KnotAlgebraError> {
        Ok(Self::of_chain(kv, equal_split_plan(kv, splits)?))
    }

    fn of_chain(kv: &KnotVector, plans: Vec<CurvePlan>) -> Self {
        let knots = plans
            .last()
            .map_or_else(|| kv.clone(), |plan| plan.knots().clone());
        Self {
            plans,
            source: kv.control_count(),
            knots,
        }
    }

    /// The refined vector.
    #[must_use]
    pub fn knots(&self) -> &KnotVector {
        &self.knots
    }

    /// The insertion chain, one plan per inserted point: what the
    /// refinement tests replay plan by plan.
    #[cfg(test)]
    pub(crate) fn plans(&self) -> &[CurvePlan] {
        &self.plans
    }

    /// `channel` through every plan in order, each by
    /// [`CurvePlan::apply_certified`], so the result encloses the refined
    /// channel of the described net. A channel that is not one entry per
    /// source control point refuses: every entry of the refined length is
    /// refused.
    #[must_use]
    pub fn refine(&self, channel: &[Interval]) -> Vec<Interval> {
        if channel.len() != self.source {
            return vec![Interval::refused(); self.knots.control_count()];
        }
        self.plans
            .iter()
            .fold(channel.to_vec(), |c, plan| plan.apply_certified(&c))
    }
}

/// Every weight certified and its enclosure strictly positive: the
/// weights a de-homogenization may divide by, and the convex-combination
/// licence of a rational net's hulls. The one spelling of that guard.
#[must_use]
pub fn certainly_positive(weights: &[Interval]) -> bool {
    weights.iter().all(|w| w.is_certified() && w.lo() > 0.0)
}

/// A refined homogeneous channel `w·x` read back as `x`: the quotient by
/// the refined weights `w`, entry by entry. `None` unless the two are
/// one length and the weights are [`certainly_positive`].
#[must_use]
pub fn dehomogenize(channel: &[Interval], weights: &[Interval]) -> Option<Vec<Interval>> {
    (channel.len() == weights.len() && certainly_positive(weights))
        .then(|| channel.iter().zip(weights).map(|(a, w)| *a / *w).collect())
}

/// How close a grid point may come to a mandatory point (a knot, a
/// caller's cut) before it is dropped, as a FRACTION OF THE GRID'S OWN
/// SPACING `|hi − lo|/pieces`: the clearance of [`domain_grid_points`]
/// and [`range_grid_points`], spelled out by [`grid_clearance`].
///
/// Grid points only subdivide, so either choice is sound; each costs
/// width.
///
/// * **Skipping** one lets the span it would have closed run on to the
///   mandatory point: at most `(1 + f)` spacings, under 0.4% wider at
///   `f = 2⁻⁸`.
/// * **Inserting** one at a gap `g` opens a span `g` wide, and every
///   derivative read off it carries the inserted point's rounding over
///   `g`. That excess grows as `1/g` without bound as the gap closes,
///   which is why the clearance is a fraction of the spacing and not a
///   count of ulps.
///
/// A measurement, not a guarded contract: on the rational quarter
/// cylinder split a gap `g` above a point of the 16-span grid
/// (`geom-brep`'s `props::quad` row
/// `the_refine_grid_enclosure_has_no_cliff_at_any_knot_offset`), the
/// round-0 flux width with the point inserted exceeds the on-grid
/// `2.0e-2` by about `1.7e-15 / g` over `g` from `1.8e-15` (`47×`) to
/// `1e-8`; with it skipped the width is about 0.2% wider than inserted
/// at gaps of `1e-10` and more, and the two cross near `g ≈ 4e-11`. That
/// row asserts only that no offset leaves `1.25×` of on-grid, so the
/// law and the crossover are unguarded: they are the evidence for `f`,
/// and nothing reads them.
///
/// `2⁻⁸` skips every gap below `spacing/256` — far past that crossover,
/// paying the skip's few-tenths-of-a-percent there for decades of
/// margin over the `1/g` regime on grids and consumers the row does not
/// measure. It is exact in binary, so the clearance rounds once. It is
/// NOT a tolerance in the ε sense: no input's meaning depends on it,
/// only whether one optional subdivision is taken.
pub const GRID_CLEARANCE: f64 = 1.0 / 256.0;

/// The clearance of the `pieces`-span grid on `[lo, hi]`:
/// [`GRID_CLEARANCE`] of its spacing `|hi − lo|/pieces`. A grid point
/// within this of a mandatory point, inclusive, is dropped by
/// [`range_grid_points`].
#[must_use]
pub fn grid_clearance(lo: f64, hi: f64, pieces: usize) -> f64 {
    #[allow(clippy::cast_precision_loss)]
    let spacing = (hi - lo).abs() / pieces as f64;
    spacing * GRID_CLEARANCE
}

/// **The domain-uniform grid**: the interior points
/// `lo + (hi − lo)·k/pieces`, `0 < k < pieces`, of the vector's DOMAIN
/// `[lo, hi]`, ascending, minus every point within [`GRID_CLEARANCE`]
/// of the grid spacing of an interior knot — the `new_knots` a caller
/// hands [`refine_plan`] or [`refine_plan_homogeneous`] to refine to
/// about `pieces` spans over the whole domain, or a break list for a
/// per-span extraction.
///
/// **Not the equal-split schedule**: [`equal_split_points`] cuts each
/// nonempty SPAN into equal pieces, so its grid restarts at every knot
/// and a knot is a span end by construction. This grid is blind to
/// where the knots fall, which is why it needs a clearance and the
/// per-span schedule does not.
///
/// It refines ANY vector, however fine already. A caller for whom a
/// vector with `pieces + degree` or more control points is fine enough
/// tests that itself and does not call; the grid does not decide it.
/// `pieces` of 0 or 1 yields none.
#[must_use]
pub fn domain_grid_points(kv: &KnotVector, pieces: usize) -> Vec<f64> {
    let (lo, hi) = kv.domain();
    let knots: Vec<f64> = kv.interior_knots().map(|(k, _)| k).collect();
    range_grid_points(lo, hi, pieces, &knots)
}

/// **The range-uniform grid** under [`domain_grid_points`]: the
/// interior points `lo + (hi − lo)·k/pieces`, `0 < k < pieces`, of an
/// arbitrary range `[lo, hi]`, ascending, minus every point that falls
/// outside the open range or within [`grid_clearance`] of a point of
/// `mandatory`.
///
/// `mandatory` is whatever set the grid must defer to, and it is not
/// the grid's to widen: [`domain_grid_points`] passes a vector's
/// interior knots, and a caller cutting a sub-range on a raw knot
/// slice passes its whole cut set, which carries the range's ends
/// for completeness — the open-range test already keeps every grid
/// point off them. The test is against `mandatory` alone, never
/// against other grid points.
///
/// The clearance scales with the grid's spacing, so a coarser grid
/// clears more: a point a coarse grid shares with a finer one (the
/// counts dividing) is dropped by the finer grid only if the coarser
/// one drops it too. So doubling `pieces` keeps every point the
/// coarser grid kept.
///
/// `pieces` of 0 or 1 yields none.
#[must_use]
pub fn range_grid_points(lo: f64, hi: f64, pieces: usize, mandatory: &[f64]) -> Vec<f64> {
    let clearance = grid_clearance(lo, hi, pieces);
    (1..pieces)
        .filter_map(|k| {
            #[allow(clippy::cast_precision_loss)]
            let t = lo + (hi - lo) * (k as f64 / pieces as f64);
            (t > lo && t < hi && mandatory.iter().all(|m| (t - *m).abs() > clearance)).then_some(t)
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
/// positive-weight regime — the removal chain's written weights, or the
/// reinsertion's ([`insert_knot_plan`] of `u` into the removed curve,
/// which the bound is measured on), whose `index` is then the
/// reinserted polygon's slot — plus the shared structure refusals.
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
        let plan = remove_once(&cur_kv, &cur_w, u, ChainSplit::Midpoint)?;
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

/// Where a removal pass hands its unknowns from the forward chain to
/// the backward one ([`remove_knot_plan`] for the chains). Both solve
/// the same equations; they differ in which one equation goes unused
/// and in what each step divides by.
#[derive(Clone, Copy)]
enum ChainSplit {
    /// Piegl–Tiller's meet in the middle ([`midpoint_split`]). A lossy
    /// removal's residual lands on the middle equation, which is the one
    /// its bound is built around.
    Midpoint,
    /// The first `nf` unknowns forward. Sound only for a removal exact in
    /// ℝ, where every equation holds and which one goes unused is free:
    /// the elevation recomposition ([`conditioned_split`]).
    At(usize),
}

/// Piegl–Tiller's split of `n_unknown` unknowns: the first `⌈n/2⌉`
/// forward.
fn midpoint_split(n_unknown: usize) -> usize {
    n_unknown.div_ceil(2)
}

/// How much worse than the best split [`conditioned_split`] lets the
/// midpoint's estimated error be before it leaves the midpoint. The
/// estimate is first order, so a nearer split is no better on evidence
/// and the midpoint keeps its bits.
///
/// **Empirical.** Fitted on held-out knot families (uniform, graded,
/// geometric, jittered, with and without close pairs) at degrees 3–10;
/// 2 was at or within 2× of the best median on every family. Rows pin
/// both sides: 1 leaves the midpoint on uniform near-ties
/// (`uniform_knots_keep_the_midpoint_split_at_every_pass`), and 4 or more
/// keeps it on steeply graded knots where it is far worse
/// (`elevation_conditioning.rs`, `steeply_graded_knots_leave_the_midpoint`).
/// An exact tie at the margin goes to the midpoint (the product is
/// exact), and among equally ranked splits the one nearest the midpoint
/// wins. No row tells either tie rule from its inverse: inverting each
/// reads no worse on any family measured.
const MIDPOINT_MARGIN: f64 = 2.0;

/// The pass removing one copy of `u` (multiplicity `s`, last copy at
/// index `r`): the unknowns are `first..last` (`first = r − p`,
/// `last = r − s`).
fn removal_window(kv: &KnotVector, u: f64) -> (usize, usize, usize, usize) {
    let p = kv.degree();
    // Present with multiplicity s, last copy at index r (callers
    // establish presence; map_or keeps this total anyway).
    let (s, r) = kv.multiplicity_of(u).map_or((1, p + 1), |sr| sr);
    (s, r, r - p, r - s)
}

/// The split an exact removal of one copy of `u` should take, and the
/// error estimate rewritten to the removed curve's layout.
///
/// `err` holds one first-order rounding estimate per control point, in
/// units of one rounding of the point's scale. Each chain step's error
/// is its own rounding plus what its two inputs carry, scaled by what it
/// multiplies them by: forward `Â_i = (A_i − (1−α_i)·Â_{i−1})/α_i` from
/// the carried point below, backward
/// `Â_i = (A_{i+1} − α_{i+1}·Â_{i+1})/(1−α_{i+1})` from the one above.
/// Neither chain's values depend on where the other starts, so each is
/// read once. The split taken is the one whose worst point carries the
/// least, unless the midpoint is within [`MIDPOINT_MARGIN`] of it. The
/// estimate reads only the knots, so the choice is the same for every
/// curve on them.
fn conditioned_split(kv: &KnotVector, u: f64, err: &[f64]) -> (usize, Vec<f64>) {
    let (p, knots) = (kv.degree(), kv.knots());
    let (_, _, first, last) = removal_window(kv, u);
    let alpha = |i: usize| (u - knots[i]) / (knots[i + p + 1] - knots[i]);
    let n_unknown = last - first;
    let mut fwd = Vec::with_capacity(n_unknown);
    let mut e = err[first - 1];
    for (i, carried) in err.iter().enumerate().take(last).skip(first) {
        let a = alpha(i);
        e = (1.0 + carried + (1.0 - a) * e) / a;
        fwd.push(e);
    }
    let mut bwd = vec![0.0f64; n_unknown];
    let mut e = err[last + 1];
    for i in (first..last).rev() {
        let a = alpha(i + 1);
        e = (1.0 + err[i + 1] + a * e) / (1.0 - a);
        bwd[i - first] = e;
    }
    let worst = |k: usize| {
        fwd[..k]
            .iter()
            .chain(&bwd[k..])
            .copied()
            .fold(0.0f64, f64::max)
    };
    let mid = midpoint_split(n_unknown);
    let best = (0..=n_unknown)
        .min_by(|&x, &y| {
            worst(x)
                .total_cmp(&worst(y))
                .then(x.abs_diff(mid).cmp(&y.abs_diff(mid)))
        })
        .unwrap_or(mid);
    let nf = if worst(mid) <= MIDPOINT_MARGIN * worst(best) {
        mid
    } else {
        best
    };
    // The removed curve's layout: prefix carried, the chains' unknowns,
    // the suffix shifted down past the dropped slot.
    let mut next = Vec::with_capacity(err.len() - 1);
    next.extend_from_slice(&err[..first]);
    next.extend_from_slice(&fwd[..nf]);
    next.extend_from_slice(&bwd[nf..]);
    next.extend_from_slice(&err[last + 1..]);
    (nf, next)
}

/// One removal pass (fn docs on [`remove_knot_plan`] for the chain
/// derivation), its split as `split` names it. Its callers establish
/// the preconditions: valid weights, and `u` an interior knot.
fn remove_once(
    kv: &KnotVector,
    weights: &[f64],
    u: f64,
    split: ChainSplit,
) -> Result<CurvePlan, KnotAlgebraError> {
    let p = kv.degree();
    let knots = kv.knots();
    let (_, r, first, last) = removal_window(kv, u);
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
    // The same ratio as an applier forms it: `u` is the knot at `r`.
    let removal_ratio = |i: usize| Coef::Knots {
        u: KnotAt::Index(r),
        lo: i,
        hi: i + p + 1,
    };
    // And as its exact ingredients, for the ring applier.
    let ring_ratio = |i: usize| Ratio {
        inserted: u,
        lo: knots[i],
        hi: knots[i + p + 1],
    };

    let mut new_w = vec![0.0f64; n_new];
    let mut steps = Vec::with_capacity(n_new);
    for j in 0..first {
        steps.push(Step::Carry { target: j, from: j });
        new_w[j] = weights[j];
    }
    let n_unknown = last - first; // = p − s
    let nf = match split {
        ChainSplit::Midpoint => midpoint_split(n_unknown),
        ChainSplit::At(nf) => nf,
    };
    // Forward chain: Â_i = (A_i − (1−α_i)·Â_{i−1}) / α_i.
    for i in first..first + nf {
        let a = alpha(i);
        let (prev_src, prev_w) = if i == first {
            (Src::Old(first - 1), weights[first - 1])
        } else {
            (Src::New(i - 1), new_w[i - 1])
        };
        let wq = written_weight(i, (weights[i] - (1.0 - a) * prev_w) / a)?;
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
            ratio: ring_ratio(i),
            form: Form::UnblendForward(removal_ratio(i)),
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
        let wq = written_weight(i, (weights[i + 1] - a * next_w) / (1.0 - a))?;
        // λ overflow note: as in the forward chain above.
        let lambda = (weights[i + 1] / (1.0 - a)) / wq;
        steps.push(Step::Combo {
            target: i,
            x: next_src,
            y: Src::Old(i + 1),
            lambda,
            ratio: ring_ratio(i + 1),
            form: Form::UnblendBackward(removal_ratio(i + 1)),
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
        reference: true,
    })
}

/// Degree elevation by one (Book §5.5), via the Bézier route:
/// (1) refine every distinct interior knot to multiplicity `p`,
/// (2) elevate each Bézier segment with the binomial combination
/// `Q_i = c_i·A_{i−1} + (1−c_i)·A_i`, `c_i = i/(p+1)` (homogeneous;
/// projective form per the module docs), (3) remove each interior
/// breakpoint back down to its original multiplicity plus one. The
/// recomposition removals are **exact in ℝ** (the elevated curve has
/// full continuity there), so no bound is surfaced.
///
/// Being exact, a removal may leave any one of its equations unused,
/// and the choice decides its rounding. Piegl–Tiller's midpoint split
/// divides by `α ≈ g/span` next to a breakpoint whose gap `g` is small
/// beside its neighbours, and each removal pass feeds the next its
/// error, so the midpoint amplifies rounding by about `(span/g)^k` with
/// `k` growing with degree. Each pass here keeps the midpoint unless
/// another split's worst point carries clearly less first-order error,
/// given what its inputs already carry (`conditioned_split`). Any split
/// is exact in ℝ, so the choice moves only rounding.
///
/// **Measured agreement**, sup over the curve relative to its largest
/// control coordinate: within `1e-12` at degree ≤ 10 on every family the
/// rows and their held-out checks draw (close pairs to `1e-9`, graded,
/// geometric, jittered and uniform knots); about `1e-11` at degree 12 on
/// long graded or jittered knot vectors; `5e-11` at degree 25 with a
/// `1e-9` pair and wide weights. No bound is surfaced: past degree 10
/// the disagreement is unmeasured by any row and silent
/// (`work/nurbs/elevation-past-degree-ten-agrees-to-1e-11-with-no-bound`).
///
/// # Errors
///
/// The shared structure refusals; a [`KnotAlgebraError::WeightCollapse`]
/// from any step is possible only through floating-point degeneracy
/// (an underflowing blend in (1) or (2), a removal leaving the regime
/// in (3)) and is surfaced honestly rather than clamped.
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
    let stage = elevate_bezier_stage(&cur_kv, &cur_w)?;
    cur_kv = stage.knots.clone();
    cur_w = stage.weights.clone();
    plans.push(stage);
    // The split estimate's seed: one rounding per point. The
    // decomposition's insertions round each point a few times more, but
    // alike across a pass's candidates, so a uniform seed ranks them as
    // a counted one would.
    let mut err = vec![1.0f64; cur_w.len()];
    // (3) Recomposition: interior breakpoints from multiplicity p+1
    // down to original + 1.
    for (v, m) in &interior {
        for _ in 0..p - m {
            let (nf, next) = conditioned_split(&cur_kv, *v, &err);
            err = next;
            let plan = remove_once(&cur_kv, &cur_w, *v, ChainSplit::At(nf))?;
            cur_kv = plan.knots.clone();
            cur_w = plan.weights.clone();
            plans.push(plan);
        }
    }
    Ok(plans)
}

/// The Bézier-segment elevation stage: every interior knot is at
/// multiplicity `p` (established by [`elevate_plan`] step 1).
fn elevate_bezier_stage(kv: &KnotVector, weights: &[f64]) -> Result<CurvePlan, KnotAlgebraError> {
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
            #[allow(clippy::cast_precision_loss)]
            let (num, den) = (i as f64, (p + 1) as f64);
            let c = num / den;
            let wq = written_weight(o2 + i, c * weights[o + i - 1] + (1.0 - c) * weights[o + i])?;
            let lambda = c * weights[o + i - 1] / wq;
            steps.push(Step::Combo {
                target: o2 + i,
                x: Src::Old(o + i),
                y: Src::Old(o + i - 1),
                lambda,
                ratio: Ratio {
                    inserted: num,
                    lo: 0.0,
                    hi: den,
                },
                form: Form::Blend(Coef::Constant { num, den }),
            });
            new_w[o2 + i] = wq;
        }
        steps.push(Step::Carry {
            target: o2 + p + 1,
            from: o + p,
        });
        new_w[o2 + p + 1] = weights[o + p];
    }
    Ok(CurvePlan {
        knots: new_kv,
        weights: new_w,
        steps,
        reference: true,
    })
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;
    use crate::real::Bounds;
    use crate::spline::basis::basis_funs;

    /// On uniform knots the margin keeps the midpoint: at degree 10
    /// with 40 interior knots, every recomposition pass of the
    /// elevation splits its unknowns `⌈n/2⌉` forward. The estimate
    /// reads only the knots, so the choice is exact and the same for
    /// every curve on them. Red at a margin of 1, which leaves the
    /// midpoint on near-ties there and doubles the elevation's median
    /// rounding.
    #[test]
    fn uniform_knots_keep_the_midpoint_split_at_every_pass() {
        let p = 10;
        let mut ks = vec![0.0; p + 1];
        ks.extend((1..=40).map(|i| f64::from(i) / 41.0));
        ks.extend(core::iter::repeat_n(1.0, p + 1));
        let kv = KnotVector::from_algebra(ks, p);
        let plans = elevate_plan(&kv, &vec![1.0; kv.control_count()]).unwrap();
        let mut passes = 0;
        // The removal passes: every plan after the Bézier stage.
        let stage = plans
            .iter()
            .position(|pl| pl.knots.degree() == p + 1)
            .unwrap();
        for (k, plan) in plans[stage + 1..].iter().enumerate() {
            let (mut forward, mut unknowns) = (0usize, 0usize);
            for step in &plan.steps {
                if let Step::Combo { form, .. } = step {
                    unknowns += 1;
                    forward += usize::from(matches!(form, Form::UnblendForward(_)));
                }
            }
            assert_eq!(
                forward,
                unknowns.div_ceil(2),
                "pass {k}: {forward} of {unknowns} forward"
            );
            passes += 1;
        }
        assert_eq!(passes, 40 * (p - 1), "every breakpoint from p down to 2");
    }

    /// 1-D rational evaluation oracle: x(t) = Σ N w x / Σ N w — the
    /// plan machinery is dimension-agnostic, so scalar control points
    /// are a complete test bed.
    fn eval1(kv: &KnotVector, w: &[f64], x: &[f64], t: f64) -> f64 {
        let span = kv.span_at(t).expect("a numeric parameter");
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
    /// the spans, and drops a grid point within `GRID_CLEARANCE` of the
    /// grid's spacing of a knot. On `[0, 1]` in quarters (clearance
    /// `2⁻¹⁰`): a knot one ulp above `0.25` drops `0.25`; a knot exactly
    /// `2⁻¹⁰` below `0.75` drops it too (the clearance is inclusive);
    /// `0.5` is a knot. A vector already finer than the grid is still
    /// given it, and `pieces` of 0 or 1 yields none.
    #[test]
    fn domain_grid_points_clearance_and_no_cut_off() {
        let near = f64::from_bits(0.25f64.to_bits() + 1);
        let edge = 0.75f64 - 1.0 / 1024.0;
        let kv =
            KnotVector::clamped(vec![0.0, 0.0, 0.0, near, 0.5, edge, 1.0, 1.0, 1.0], 2).unwrap();
        assert!(domain_grid_points(&kv, 4).is_empty());
        // One ulp further from `0.75` and the grid point stands.
        let past = KnotVector::clamped(
            vec![0.0, 0.0, 0.0, near, 0.5, edge.next_down(), 1.0, 1.0, 1.0],
            2,
        )
        .unwrap();
        assert_eq!(domain_grid_points(&past, 4), [0.75]);
        // The clearance is a fraction of the SPACING, not of the domain:
        // on `[0, 4]` in quarters the spacing is `1`, so a knot `2⁻⁹`
        // off the grid point `1.0` is inside its `2⁻⁸` clearance; in
        // eighths (spacing `½`, clearance `2⁻⁹`) it is on the edge.
        let wide = KnotVector::clamped(vec![0.0, 0.0, 1.0 + 1.0 / 512.0, 4.0, 4.0], 1).unwrap();
        assert_eq!(domain_grid_points(&wide, 4), [2.0, 3.0]);
        assert_eq!(domain_grid_points(&wide, 8), [0.5, 1.5, 2.0, 2.5, 3.0, 3.5]);
        // Ten spans, a grid of four: every quarter is still offered.
        let fine = KnotVector::clamped(
            vec![
                0.0, 0.0, 0.05, 0.15, 0.35, 0.45, 0.55, 0.65, 0.8, 0.9, 0.95, 1.0, 1.0,
            ],
            1,
        )
        .unwrap();
        assert_eq!(fine.control_count(), 11);
        assert_eq!(domain_grid_points(&fine, 4), [0.25, 0.5, 0.75]);
        assert!(domain_grid_points(&kv, 1).is_empty());
        assert!(domain_grid_points(&kv, 0).is_empty());
        // A non-dyadic domain pins the grid ARITHMETIC: `lo + (hi − lo)·(k/n)`
        // rounds to these values, and the counted-from-the-top form
        // `hi − (hi − lo)·((n − k)/n)` to others (`0.15999999999999992`,
        // `0.39999999999999997`, …). The knot at `0.4` is the grid's own
        // point, so the clearance drops it.
        let odd = KnotVector::clamped(vec![0.1, 0.1, 0.4, 0.7, 0.7], 1).unwrap();
        assert_eq!(
            domain_grid_points(&odd, 10),
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
            domain_grid_points(&odd, 13),
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
        // A finite width near `f64::MAX`: `grid_clearance` divides
        // before it scales, so it is finite and a grid point
        // clear of the knot stands.
        let big = f64::MAX / 2.0;
        let huge = KnotVector::clamped(vec![-big, -big, 0.5 * big, big, big], 1).unwrap();
        assert_eq!(domain_grid_points(&huge, 4), [-0.5 * big, 0.0]);
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
    /// them, and with EQUAL adjacent coefficients it equals them. A form
    /// that rounds the ratio into the value (`β·c + α·c` with `α`, `β`
    /// rounded outward independently) brackets `c` instead; a constant
    /// column is the one input where any width at all is visible as an
    /// excursion.
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
    fn the_step_holds_a_constant_column_exactly() {
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

    /// **Elevation and removal run in the ring too, in step with the
    /// point applier (C6 S4).** A degree elevation chain (Bézier
    /// decomposition, the binomial blend, the recomposition removals)
    /// and a removal of a knot an insertion put in, each over one
    /// homogeneous channel of point coefficients of both signs: every
    /// slot certifies, and the point applier's answer lies within a
    /// per-step allowance of it. The removal undoes the insertion, so
    /// the ring encloses the original coefficients. Red where either
    /// step kind refuses, combines in the wrong orientation (a swapped
    /// `x`/`y` or `a`/`1 − a` moves a slot by the coefficients' own
    /// scale), or rounds its ratio into a guess.
    #[test]
    fn elevation_and_removal_run_in_the_ring_in_step_with_the_point_applier() {
        let in_step = |tag: &str, plans: &[CurvePlan], coeffs: &[f64], want: Option<&[f64]>| {
            let point = apply_chain(plans, coeffs);
            let mut ring: Vec<Interval> = coeffs.iter().copied().map(Interval::point).collect();
            for plan in plans {
                ring = plan.apply_certified(&ring);
            }
            assert_eq!(ring.len(), point.len(), "{tag}: extent");
            let scale = coeffs.iter().fold(1.0f64, |m, c| m.max(c.abs()));
            #[allow(clippy::cast_precision_loss)]
            let allowance = 64.0 * (plans.len() + 1) as f64 * scale * f64::EPSILON;
            for (i, r) in ring.iter().enumerate() {
                assert!(r.is_certified(), "{tag}: slot {i} refused");
                let f = point[i];
                let gap = (r.lo() - f).max(f - r.hi()).max(0.0);
                assert!(
                    gap <= allowance,
                    "{tag}: slot {i} = [{:e}, {:e}] is {gap:e} from the point applier's {f:e}",
                    r.lo(),
                    r.hi()
                );
                assert!(
                    r.width() <= allowance,
                    "{tag}: slot {i} is {:e} wide",
                    r.width()
                );
                if let Some(want) = want {
                    assert!(
                        r.lo() <= want[i] && want[i] <= r.hi(),
                        "{tag}: slot {i} = [{:e}, {:e}] misses the original {:e}",
                        r.lo(),
                        r.hi(),
                        want[i]
                    );
                }
            }
        };
        for (p, interior) in [
            (1usize, &[][..]),
            (2, &[1.0][..]),
            (2, &[1.0, 2.0][..]),
            (3, &[1.0, 1.0, 2.0][..]),
        ] {
            let mut knots = vec![0.0; p + 1];
            knots.extend_from_slice(interior);
            let top = interior.last().copied().unwrap_or(0.0) + 1.0;
            knots.extend(core::iter::repeat_n(top, p + 1));
            let kv = KnotVector::clamped(knots, p).unwrap();
            let n = kv.control_count();
            #[allow(clippy::cast_precision_loss)]
            let coeffs: Vec<f64> = (0..n)
                .map(|i| {
                    if i % 2 == 0 {
                        1.0 + i as f64
                    } else {
                        -(i as f64)
                    }
                })
                .collect();
            let unit = vec![1.0; n];
            let elevate = elevate_plan(&kv, &unit).unwrap();
            in_step(
                &format!("elevate p={p} {interior:?}"),
                &elevate,
                &coeffs,
                None,
            );
            // A knot put in and taken out again: exact in ℝ, so the ring
            // encloses the original coefficients.
            let u = 0.375 * top;
            let inserted = insert_knot_plan(&kv, &unit, u, 1).unwrap();
            let removed =
                remove_knot_plan(inserted[0].knots(), inserted[0].weights(), u, 1).unwrap();
            let chain: Vec<CurvePlan> = inserted
                .into_iter()
                .chain(removed.into_iter().map(|step| step.plan))
                .collect();
            in_step(
                &format!("remove p={p} {interior:?}"),
                &chain,
                &coeffs,
                Some(&coeffs),
            );
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
        // 309-digit positional expansion. The repair rides with it
        // (`every_knot_algebra_error_arm_names_a_recourse`).
        assert_eq!(
            insert_knot_plan(&kv, &w, 1e308, 1).unwrap_err().to_string(),
            "knot parameter 1e308 is not strictly inside the domain. Recourse: ask for a \
             finite parameter strictly between the clamped ends, whose multiplicity is \
             already degree + 1"
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

    /// Runs a chain through [`CurvePlan::apply_rational`] at `f64` and
    /// through [`CurvePlan::apply_points`] beside it, requiring the
    /// points and the weights to agree with the projective applier and
    /// the plans' own weights bit for bit at every link.
    fn rational_agrees_bitwise(
        plans: &[CurvePlan],
        kv: &KnotVector,
        w: &[f64],
        x: &[f64],
        what: &str,
    ) {
        let bits = |v: &[f64]| v.iter().map(|x| x.to_bits()).collect::<Vec<_>>();
        let (mut knots, mut xr, mut wr) = (kv.knots().to_vec(), x.to_vec(), w.to_vec());
        let mut xp = x.to_vec();
        for (n, plan) in plans.iter().enumerate() {
            (xr, wr) = plan.apply_rational(&knots, &xr, &wr, (f64::NAN, &|v| v), lerp1);
            xp = plan.apply_points(&xp, f64::NAN, lerp1);
            assert_eq!(bits(&xr), bits(&xp), "{what} link {n}: points");
            assert_eq!(bits(&wr), bits(plan.weights()), "{what} link {n}: weights");
            knots = plan.knots().knots().to_vec();
        }
    }

    /// The rational applier forms each step at the caller's scalar with
    /// the operations the plan constructor took on `f64`, so at `f64`
    /// it IS the projective applier and the plans' weights: insertion
    /// (fresh and at a present breakpoint), removal's two chains, and
    /// elevation's binomial stage.
    #[test]
    fn the_rational_applier_is_the_plan_at_f64() {
        let (kv, w, x) = fixture();
        rational_agrees_bitwise(
            &insert_knot_plan(&kv, &w, 1.4, 2).unwrap(),
            &kv,
            &w,
            &x,
            "insert fresh",
        );
        rational_agrees_bitwise(
            &insert_knot_plan(&kv, &w, 2.0, 1).unwrap(),
            &kv,
            &w,
            &x,
            "insert present",
        );
        rational_agrees_bitwise(
            &refine_plan(&kv, &w, &[0.3, 1.4, 1.4, 2.5]).unwrap(),
            &kv,
            &w,
            &x,
            "refine",
        );
        rational_agrees_bitwise(&elevate_plan(&kv, &w).unwrap(), &kv, &w, &x, "elevate");
        let cubic = KnotVector::clamped(
            vec![0.0, 0.0, 0.0, 0.0, 0.7, 0.7, 0.7, 2.0, 2.0, 2.0, 2.0],
            3,
        )
        .unwrap();
        let (wc, xc) = (
            vec![1.0, 0.8, 1.3, 2.0, 0.6, 1.1, 0.9],
            vec![0.0, 1.0, -2.0, 3.0, 0.5, 4.0, 1.0],
        );
        let removal: Vec<CurvePlan> = remove_knot_plan(&cubic, &wc, 0.7, 3)
            .unwrap()
            .into_iter()
            .map(|step| step.plan)
            .collect();
        rational_agrees_bitwise(&removal, &cubic, &wc, &xc, "remove");
        // Distinct knots around the removed one, so each chain's ratio
        // index is seen: `α(i) ≠ α(i + 1)` at every step of both chains.
        // The net is 0.7, absent from the base, inserted to full
        // multiplicity, so removing all three copies is exact (no weight
        // collapses) and the last pass has two unknowns: one forward
        // step and one backward step.
        let base = KnotVector::clamped(
            vec![0.0, 0.0, 0.0, 0.0, 0.2, 0.45, 1.3, 1.6, 2.0, 2.0, 2.0, 2.0],
            3,
        )
        .unwrap();
        let (wb, xb) = (
            vec![1.0, 0.8, 1.3, 2.0, 0.6, 1.1, 0.9, 1.4],
            vec![0.0, 1.0, -2.0, 3.0, 0.5, 4.0, 1.0, -1.0],
        );
        let ins = insert_knot_plan(&base, &wb, 0.7, 3).unwrap();
        let last = ins.last().unwrap();
        let (spread, ws) = (last.knots().clone(), last.weights().to_vec());
        let xs = apply_chain(&ins, &xb);
        let removal: Vec<CurvePlan> = remove_knot_plan(&spread, &ws, 0.7, 3)
            .unwrap()
            .into_iter()
            .map(|step| step.plan)
            .collect();
        rational_agrees_bitwise(&removal, &spread, &ws, &xs, "remove, distinct neighbours");
    }

    /// A weight held as a variable carries its derivative through every
    /// form: the refined weights and points are the `f64` ones in value
    /// and move with the variable in tangent (checked against a central
    /// difference of the `f64` plan's own weights).
    #[test]
    fn a_variable_weight_moves_through_the_rational_applier() {
        use crate::dual::Dual64;
        let (kv, w, x) = fixture();
        let lerp = |a: Dual64, b: Dual64, l: Dual64| a + (b - a) * l;
        for (what, chain) in [
            ("insert", insert_knot_plan(&kv, &w, 1.4, 2).unwrap()),
            ("elevate", elevate_plan(&kv, &w).unwrap()),
        ] {
            let mut knots: Vec<Dual64> = kv.knots().iter().map(|k| Dual64::constant(*k)).collect();
            let mut wd: Vec<Dual64> = w.iter().map(|v| Dual64::constant(*v)).collect();
            wd[2] = Dual64::variable(w[2]);
            let mut xd: Vec<Dual64> = x.iter().map(|v| Dual64::constant(*v)).collect();
            for plan in &chain {
                (xd, wd) = plan.apply_rational(
                    &knots,
                    &xd,
                    &wd,
                    (Dual64::constant(f64::NAN), &Dual64::constant),
                    lerp,
                );
                knots = plan
                    .knots()
                    .knots()
                    .iter()
                    .map(|k| Dual64::constant(*k))
                    .collect();
            }
            let last = chain.last().unwrap();
            let h = 1e-6;
            let at = |dw: f64| {
                let mut wv = w.clone();
                wv[2] += dw;
                let plans = if what == "insert" {
                    insert_knot_plan(&kv, &wv, 1.4, 2).unwrap()
                } else {
                    elevate_plan(&kv, &wv).unwrap()
                };
                plans.last().unwrap().weights().to_vec()
            };
            let (up, down) = (at(h), at(-h));
            for (i, d) in wd.iter().enumerate() {
                assert_eq!(
                    d.value.to_bits(),
                    last.weights()[i].to_bits(),
                    "{what} weight {i}: value"
                );
                let fd = (up[i] - down[i]) / (2.0 * h);
                assert!(
                    (d.deriv - fd).abs() < 1e-6,
                    "{what} weight {i}: {} vs {fd}",
                    d.deriv
                );
            }
            assert!(
                wd.iter().any(|d| d.deriv != 0.0),
                "{what}: the variable reached no weight"
            );
            assert!(
                xd.iter().any(|d| d.deriv != 0.0),
                "{what}: the variable reached no point"
            );
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

    /// **`KnotAlgebraError`'s recourse claim, made enforceable**: every
    /// arm names the lever the caller turns, exactly once and labelled.
    ///
    /// `Structure` is asserted TRANSITIVELY, because `SplineError` has
    /// an enforcement row of its own
    /// (`every_spline_error_arm_names_a_recourse`, in `super::knots`);
    /// this arm renders that carrier whole and adds four words of its
    /// own, so the labelled repair the reader sees is the carrier's.
    ///
    /// The algebra is reached from the public fit door through
    /// `geom::FitError::KnotAlgebra`, which renders this carrier whole,
    /// and this row is what lets
    /// `every_fit_error_arm_names_a_recourse` read that delegation
    /// transitively.
    #[test]
    fn every_knot_algebra_error_arm_names_a_recourse() {
        const RECOURSE_WORDS: &[&str] = &["supply", "ask", "drop", "raise", "keep"];
        let arms = [
            KnotAlgebraError::Structure(SplineError::ControlCountMismatch {
                control: 3,
                expected: 4,
            }),
            KnotAlgebraError::ParameterOutsideDomain { u: 1.5 },
            KnotAlgebraError::MultiplicityOverflow {
                u: 0.5,
                have: 3,
                budget: 3,
            },
            KnotAlgebraError::KnotNotPresent { u: 0.25 },
            KnotAlgebraError::RemovalExceedsMultiplicity {
                u: 0.5,
                have: 1,
                requested: 2,
            },
            KnotAlgebraError::WeightCollapse { index: 2 },
        ];
        assert_eq!(arms.len(), 6, "an arm was added without a row here");
        for arm in &arms {
            let msg = arm.to_string();
            assert_eq!(
                test_utils::refusal::recourse_markers(&msg),
                1,
                "not exactly one labelled repair: {msg}"
            );
            let lower = msg.to_lowercase();
            assert!(
                RECOURSE_WORDS.iter().any(|w| lower.contains(w)),
                "no recourse in: {msg}"
            );
        }
    }

    /// **A written weight that rounds out of the positive regime is
    /// refused, at every writer.** On a net of the least subnormal
    /// weight a half-and-half blend rounds each half to zero (a tie, to
    /// even), so the written weight is `0`: insertion at a midpoint,
    /// refinement (a fold of insertions) and degree-1 elevation (whose
    /// one binomial coefficient is `½`) each meet it, and each answers
    /// `WeightCollapse` at the slot it would have written.
    #[test]
    fn a_blend_that_rounds_a_weight_to_zero_is_refused() {
        let w = f64::from_bits(1);
        let quad = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
        let collapse = KnotAlgebraError::WeightCollapse { index: 1 };
        assert_eq!(
            insert_knot_plan(&quad, &[w, w, w], 0.5, 1).map(|_| ()),
            Err(collapse.clone()),
            "insertion"
        );
        assert_eq!(
            refine_plan(&quad, &[w, w, w], &[0.5]).map(|_| ()),
            Err(collapse.clone()),
            "refinement"
        );
        let line = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
        assert_eq!(
            elevate_plan(&line, &[w, w]).map(|_| ()),
            Err(collapse),
            "elevation"
        );
        // Weights a blend cannot underflow pass the same writers.
        assert!(insert_knot_plan(&quad, &[1.0; 3], 0.5, 1).is_ok());
        assert!(elevate_plan(&line, &[1.0; 2]).is_ok());
    }
}
