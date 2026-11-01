//! The validated knot lists — pure **structure** (C6): the clamped
//! [`KnotVector`], and the [`LooseKnots`] a derivative steps onto where
//! no clamped vector holds it. `f64` knots and a `usize` degree,
//! produced and consumed by the
//! deterministic f64 lane. Raw `f64` comparisons are legal throughout
//! this file (structure selection, never a topology decision).

use super::range::{Param, ParamRange, last_at_or_below};
use crate::exact::two_sum;
use crate::readable::Readable;
use crate::real::Real;
use core::num::NonZeroUsize;

/// `hi − lo` for two `f64` knots of a bare [`KnotVector`] (one that
/// holds no values in a scalar; a carrier's held knots are subtracted
/// in their own scalar), the difference taken in `T`: each knot
/// lifted, then subtracted at the scalar. At `Interval` that is
/// an outward enclosure of the exact difference, which
/// `T::from_f64(hi - lo)` is not when the `f64` subtraction rounds (two
/// knots more than a factor of two apart, Sterbenz's lemma failing); at
/// `f64` the two are the same number. The one spelling of a knot
/// difference at a scalar.
#[must_use]
pub fn knot_difference<T: Real>(lo: f64, hi: f64) -> T {
    T::from_f64(hi) - T::from_f64(lo)
}

/// A typed construction failure for spline structure — fail-loud per
/// D4: every invalid input is a named refusal, never a silent repair.
#[derive(Clone, Debug, PartialEq)]
pub enum SplineError {
    /// The knot list fails its contract — [`KnotVector::clamped`]'s
    /// clamped-v1, or the end-run clauses alone of
    /// [`LooseKnots::new`]; `reason` names the exact violation.
    KnotVectorInvalid {
        /// The specific structural violation.
        reason: KnotVectorIssue,
    },
    /// A weight is not strictly positive (NaN weights land here too:
    /// `NaN > 0` is false). Positive weights are the convex-hull
    /// property every C9 hull bound stands on (Book p. 293); zero and
    /// negative weights are refused at construction, w = +∞ is refused
    /// as non-finite structure.
    NonPositiveWeight {
        /// Index of the offending weight.
        index: usize,
        /// The offending value.
        weight: f64,
    },
    /// A weight is `+∞` (passes `> 0` but is not usable structure).
    NonFiniteWeight {
        /// Index of the offending weight.
        index: usize,
        /// The offending value.
        weight: f64,
    },
    /// The control-point count does not match the knot vector
    /// (`control == knots.len() − degree − 1` is required).
    ControlCountMismatch {
        /// The supplied control-point count.
        control: usize,
        /// The count the knot vector requires.
        expected: usize,
    },
    /// The weight count does not match the control-point count.
    WeightCountMismatch {
        /// The supplied weight count.
        weights: usize,
        /// The control-point count it must equal.
        control: usize,
    },
    /// The domain a vector was asked to be re-expressed on
    /// ([`KnotVector::on_domain`]) is not a finite increasing interval
    /// of finite width: an end is NaN or ±∞, `hi ≤ lo` (collapsed or
    /// reversed), or `hi − lo` overflows. A defect of the REQUEST, not
    /// of any vector — which is why it is not a [`KnotVectorIssue`] —
    /// and named as the domain's own rather than as whichever clamp
    /// clause a vector built on it would trip first.
    DomainInvalid {
        /// The requested lower end.
        lo: f64,
        /// The requested upper end.
        hi: f64,
    },
    /// A breakpoint held in the scalar is poison: it stands for no
    /// value, so it names no place in the structure. (An enclosure wider
    /// than a point is a breakpoint, C6: it is held, and its runs'
    /// order is certified for every member.)
    KnotIsPoison {
        /// Index of the offending run.
        index: usize,
    },
    /// A breakpoint's value channel is not strictly above the one
    /// before it: equal, the two runs would merge into one and the
    /// structure would not be the runs written; below, the vector
    /// decreases.
    KnotRunsNotIncreasing {
        /// Index of the offending run.
        index: usize,
    },
    /// Whether a breakpoint lies above the one before it is not decided
    /// for every value the two stand for: their enclosures overlap, so
    /// some members' runs ascend and others' meet or cross (C6). The
    /// margin is the enclosure of the step `b − a`; inputs refined so
    /// that it clears zero lift the refusal.
    KnotRunsUndecided {
        /// Index of the offending run.
        index: usize,
        /// The step from the previous breakpoint, as read.
        margin: crate::MarginDiag,
    },
    /// A curve or surface was built over knots one of whose breakpoints
    /// is an enclosure wider than a point. The knots hold such a value
    /// (C6), but a carrier's evaluation, its span reads and its knot
    /// algebra still read its knots as `f64` points, so the carrier's
    /// door refuses it rather than hand them a vector they cannot read.
    CarrierKnotNotAPoint {
        /// Index of the offending run.
        index: usize,
    },
    /// A curve or surface was built over weights one of which is an
    /// enclosure wider than a point. Held weights take one (C6,
    /// [`super::Weights`]), but a carrier's knot algebra and its
    /// certificates still read its weights as `f64` points, so the
    /// carrier's door refuses it.
    CarrierWeightNotAPoint {
        /// Index of the offending weight.
        index: usize,
    },
    /// Whether a weight is strictly positive is not decided for every
    /// value it stands for: its enclosure reaches zero or below for some
    /// members and lies above zero for others (C6). The margin is the
    /// weight's enclosure; inputs refined so that it clears zero lift
    /// the refusal.
    WeightSignUndecided {
        /// Index of the offending weight.
        index: usize,
        /// The weight, as read.
        margin: crate::MarginDiag,
    },
}

impl core::fmt::Display for SplineError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            SplineError::KnotVectorInvalid { reason } => {
                write!(f, "invalid clamped knot vector: {reason}")
            }
            SplineError::NonPositiveWeight { index, weight } => write!(
                f,
                "weight {index} is {}, not strictly positive (convex-hull invariant), and \
                 every hull bound this kernel certifies stands on the convex-combination \
                 licence. Recourse: supply a strictly positive weight there rather than a \
                 zero, a negative or a NaN",
                Readable(*weight)
            ),
            SplineError::NonFiniteWeight { index, weight } => write!(
                f,
                "weight {index} is {}, not finite: an infinite weight passes `> 0` but \
                 is not usable structure. Recourse: supply a finite strictly positive weight \
                 there",
                Readable(*weight)
            ),
            SplineError::ControlCountMismatch { control, expected } => write!(
                f,
                "control-point count {control} does not match the knot vector (expected \
                 {expected}), and the two are one description. Recourse: supply {expected} \
                 control points, or a knot vector of control + degree + 1 knots"
            ),
            SplineError::WeightCountMismatch { weights, control } => write!(
                f,
                "weight count {weights} does not match control-point count {control}. Recourse: \
                 supply one weight per control point"
            ),
            SplineError::DomainInvalid { lo, hi } => write!(
                f,
                "the domain [{}, {}] is not a finite increasing interval of finite width, \
                 and the defect is the REQUEST's, not the vector's. Recourse: ask on a domain \
                 whose ends are finite with lo < hi and whose width does not overflow",
                Readable(*lo),
                Readable(*hi)
            ),
            SplineError::KnotIsPoison { index } => write!(
                f,
                "breakpoint {index} is poison, so it names no place for the spans to be \
                 located in. Recourse: supply a breakpoint that is a number there"
            ),
            SplineError::KnotRunsNotIncreasing { index } => write!(
                f,
                "breakpoint {index} does not lie strictly above the one before it, so the two \
                 would merge into one run or reverse the vector. Recourse: supply one \
                 breakpoint with the summed multiplicity where the two are one knot, or \
                 breakpoints that increase"
            ),
            SplineError::KnotRunsUndecided { index, margin } => write!(
                f,
                "whether breakpoint {index} lies above the one before it is undecided over \
                 the values they stand for: the step between them is {margin}, which does \
                 not clear zero, so some members' runs would merge or cross. Recourse: \
                 supply those breakpoints from inputs narrow enough that the step clears zero"
            ),
            SplineError::CarrierKnotNotAPoint { index } => write!(
                f,
                "breakpoint {index} is an enclosure wider than a point, and a curve or \
                 surface reads its knots as single numbers. The width may be rounding alone \
                 (a re-expressed or averaged breakpoint at an enclosure scalar), so narrower \
                 inputs need not lift it. Recourse: supply the carrier built at a point \
                 scalar (`f64`) and lifted, so its breakpoints stay the numbers it was \
                 built on"
            ),
            SplineError::CarrierWeightNotAPoint { index } => write!(
                f,
                "weight {index} is an enclosure wider than a point, and a curve or surface \
                 reads its weights as single numbers. The width may be rounding alone (an \
                 arc's cosine at an enclosure scalar), so narrower inputs need not lift it. \
                 Recourse: supply the carrier built at a point scalar (`f64`) and lifted, so \
                 its weights stay the numbers it was built on"
            ),
            SplineError::WeightSignUndecided { index, margin } => write!(
                f,
                "whether weight {index} is strictly positive is undecided over the values it \
                 stands for: it is {margin}, which does not clear zero, so some members' \
                 weights are not positive. Recourse: supply that weight from inputs narrow \
                 enough that it clears zero, and, for a curve or surface, narrow enough that \
                 it is a single number, which is all a carrier admits for now"
            ),
        }
    }
}

impl core::error::Error for SplineError {}

/// **The weight regime**: a rational net's weight is strictly positive
/// and finite. The one spelling of that predicate: every constructor's
/// check ([`check_weight_values`]), the knot-algebra plans' admission of
/// what they write, and the hull bound's licence read it. Written in
/// the positive form, so NaN (`NaN > 0` is false) is outside the regime.
#[must_use]
pub fn weight_in_regime(w: f64) -> bool {
    w > 0.0 && w.is_finite()
}

/// Every weight in [`weight_in_regime`], or the first that is not, named:
/// [`SplineError::NonPositiveWeight`] for zero, negative and NaN, then
/// [`SplineError::NonFiniteWeight`] for `+∞`. Counts are the caller's
/// check; this reads values only.
///
/// # Errors
///
/// As above, at the offending index.
pub fn check_weight_values(weights: &[f64]) -> Result<(), SplineError> {
    for (index, &weight) in weights.iter().enumerate() {
        check_weight_value(index, weight)?;
    }
    Ok(())
}

/// One weight in [`weight_in_regime`], or its refusal at `index`:
/// [`SplineError::NonPositiveWeight`] for zero, negative and NaN,
/// [`SplineError::NonFiniteWeight`] for `+∞`. The one spelling of a
/// point weight's positivity, which [`check_weight_values`] and
/// [`super::Weights::new`] share.
///
/// # Errors
///
/// As above.
pub fn check_weight_value(index: usize, weight: f64) -> Result<(), SplineError> {
    if weight_in_regime(weight) {
        return Ok(());
    }
    // Outside the regime and still `> 0` is `+∞`.
    Err(if weight > 0.0 {
        SplineError::NonFiniteWeight { index, weight }
    } else {
        SplineError::NonPositiveWeight { index, weight }
    })
}

/// The exact structural violation behind
/// [`SplineError::KnotVectorInvalid`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum KnotVectorIssue {
    /// Degree 0 is refused: a degree-0 "curve" is a step-function
    /// locus, not a curve — a designed absence until a consumer
    /// exists, like the periodic/unclamped forms (module docs).
    DegreeZero,
    /// Fewer than `2·(degree + 1)` knots.
    TooShort,
    /// A knot is NaN or ±∞.
    NonFinite {
        /// Index of the offending knot.
        index: usize,
    },
    /// `knots[index] < knots[index − 1]` — not non-decreasing.
    Decreasing {
        /// Index of the first knot that decreases.
        index: usize,
    },
    /// The first knot's multiplicity is not exactly `degree + 1`
    /// (clamped-v1: exact end multiplicity, so the first span is
    /// nonempty and the curve interpolates the first control point).
    StartNotClamped,
    /// The last knot's multiplicity is not exactly `degree + 1`.
    EndNotClamped,
    /// An interior knot's multiplicity exceeds `degree` (allowed up
    /// to `degree`, which drops continuity to C⁰ — never past it).
    InteriorMultiplicityTooHigh {
        /// Index of the first knot of the offending run.
        index: usize,
    },
}

impl core::fmt::Display for KnotVectorIssue {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            KnotVectorIssue::DegreeZero => f.write_str(
                "degree 0 is unsupported: a degree-0 locus is a step function rather than a \
                 curve, and the form is a designed absence until a consumer exists. Recourse: \
                 describe the geometry at degree 1 or above",
            ),
            KnotVectorIssue::TooShort => f.write_str(
                "fewer than 2(degree+1) knots, and a clamped vector carries degree+1 of them at \
                 each end. Recourse: supply the missing knots, or describe at the degree the \
                 knots you have can clamp",
            ),
            KnotVectorIssue::NonFinite { index } => write!(
                f,
                "knot {index} is not finite, and knots are structure, compared exactly and never \
                 to a tolerance. Recourse: supply a finite value there"
            ),
            KnotVectorIssue::Decreasing { index } => write!(
                f,
                "knot {index} decreases, which a knot vector never does. Recourse: supply the \
                 values in ascending order, repeating one to raise its multiplicity rather \
                 than stepping back"
            ),
            KnotVectorIssue::StartNotClamped => f.write_str(
                "start multiplicity is not exactly degree+1, and clamped-v1 pins both ends so \
                 the first span is nonempty and the curve interpolates the first control \
                 point. Recourse: repeat the first knot exactly degree+1 times (the periodic \
                 and unclamped forms are a designed absence, not this refusal)",
            ),
            KnotVectorIssue::EndNotClamped => f.write_str(
                "end multiplicity is not exactly degree+1, and clamped-v1 pins both ends so the \
                 last span is nonempty and the curve interpolates the last control point. \
                 Recourse: repeat the last knot exactly degree+1 times (the periodic and \
                 unclamped forms are a designed absence, not this refusal)",
            ),
            KnotVectorIssue::InteriorMultiplicityTooHigh { index } => write!(
                f,
                "interior knot {index} has multiplicity > degree; an interior value may \
                 repeat at most degree times (which drops continuity to C⁰). Recourse: drop \
                 the surplus copies there"
            ),
        }
    }
}

/// Why [`KnotVector::mirror_symmetric`] refuses: the vector is not its
/// own reflection about its domain, or the reflection cannot be
/// computed.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum KnotMirrorError {
    /// The reflection `lo + hi` overflows to an infinity, so the domain
    /// names no reflection and the test is not defined on it.
    ///
    /// Without the guard the arithmetic would not go WRONG: an
    /// overflowing head carries a NaN residual, a NaN compares equal to
    /// nothing, and the scan would refuse at index 0 with an
    /// [`Self::AsymmetricPair`] naming the clamp pair — a true verdict
    /// with a misleading reason, since that pair is the one pair the
    /// scan is guaranteed to hold. The guard names the DOMAIN's defect
    /// instead of an innocent pair. It also refuses a vector that IS
    /// its own reflection in ℝ but whose `lo + hi` overflows: the test
    /// that would admit it cannot be run.
    ///
    /// The same NaN residual settles the per-pair case the guard does
    /// not cover. A pair whose own head overflows while `lo + hi` stays
    /// finite is necessarily asymmetric — its real sum exceeds the
    /// finite `lo + hi` — and its NaN residual refuses it, which is the
    /// right answer for the right reason.
    ReflectionNotFinite {
        /// The domain's start.
        lo: f64,
        /// The domain's end.
        hi: f64,
    },
    /// Knots `index` and `mirror_index` do not sum to `lo + hi`, so the
    /// vector is not its own reflection.
    AsymmetricPair {
        /// The lower index of the offending pair.
        index: usize,
        /// Its partner, `m − index`, where `m` is the last knot index.
        mirror_index: usize,
        /// The knot at `index`.
        knot: f64,
        /// The knot at `mirror_index`.
        mirror_knot: f64,
        /// The domain's start.
        lo: f64,
        /// The domain's end.
        hi: f64,
    },
}

impl core::fmt::Display for KnotMirrorError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            KnotMirrorError::ReflectionNotFinite { lo, hi } => write!(
                f,
                "knot mirror: the domain [{}, {}] has no finite reflection sum, so whether \
                 the vector is its own reflection cannot be decided. Recourse: re-express \
                 the vector on a domain whose ends sum finitely and ask on that",
                Readable(*lo),
                Readable(*hi)
            ),
            KnotMirrorError::AsymmetricPair {
                index,
                mirror_index,
                knot,
                mirror_knot: _,
                lo,
                hi,
            } if index == mirror_index => write!(
                f,
                "knot mirror: the middle knot {index} ({}) is not the midpoint of [{}, {}]. \
                 Recourse: supply the midpoint there",
                Readable(*knot),
                Readable(*lo),
                Readable(*hi)
            ),
            KnotMirrorError::AsymmetricPair {
                index,
                mirror_index,
                knot,
                mirror_knot,
                lo,
                hi,
            } => write!(
                f,
                "knot mirror: knots {index} and {mirror_index} ({}, {}) do not sum to {} \
                 exactly. Recourse: supply partners whose sum is exactly that; dyadic \
                 fractions of the domain mirror exactly, while decimal pairs such as \
                 0.1 and 0.9 round apart",
                Readable(*knot),
                Readable(*mirror_knot),
                Readable(lo + hi)
            ),
        }
    }
}

impl core::error::Error for KnotMirrorError {}

/// A validated **clamped** knot vector with its degree — the structural
/// half of every B-spline/NURBS entity whose knots are points: the `f64`
/// vector of the knots a carrier holds in its scalar ([`super::Knots`]),
/// with its combinatorial [`KnotStructure`]. C6's interim reads it as
/// the knots' positions; generic scalar types never appear here.
///
/// # Invariants (established at construction, relied on by indexing)
///
/// - `degree ≥ 1`; `knots.len() ≥ 2·(degree + 1)`; all knots finite.
/// - Non-decreasing; first and last values have multiplicity **exactly**
///   `degree + 1` (clamped-v1 — periodic/unclamped forms are a designed
///   absence until a consumer exists); interior multiplicities ≤
///   `degree`.
/// - Consequently the first span (`[knots[p], knots[p+1])`) and the
///   last span are nonempty, and every parameter in the domain lies in
///   some nonempty span.
///
/// Multiplicity is **exact `f64` equality** — knots are structure, and
/// structure identity is bitwise-value identity, never a tolerance
/// question.
#[derive(Clone, Debug, PartialEq)]
pub struct KnotVector {
    knots: Vec<f64>,
    degree: usize,
    /// The combinatorial part, read off `knots` once (C6).
    structure: KnotStructure,
}

/// **A clamped knot vector's combinatorial part** (C6): its degree, its
/// knot count, and its runs of equal knots with their multiplicities, in
/// order. No knot position is here. A carrier whose knots are points has
/// this beside its `f64` [`KnotVector`]; a carrier whose knots are wide
/// enclosures has only this, certified for every member
/// ([`super::Knots::new`]), so whatever reads it reads structure and
/// never a position past the funnel.
///
/// The clamped contract is [`KnotVector`]'s: degree ≥ 1, end runs of
/// exactly `degree + 1`, interior runs of at most `degree`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct KnotStructure {
    degree: usize,
    /// For each knot index, the index of its run.
    run_of: Vec<usize>,
    /// Each run's multiplicity, ascending, clamp runs included.
    multiplicities: Vec<usize>,
}

impl KnotStructure {
    /// The structure of runs of `multiplicities` at degree `degree`,
    /// validated against the clamped contract. Each multiplicity is a
    /// run's, so nonzero: the callers hold them as `NonZeroUsize`.
    ///
    /// # Errors
    ///
    /// [`SplineError::KnotVectorInvalid`] naming the violated clause
    /// (the knot index of an interior run's first knot, as
    /// [`KnotVector::clamped`] names it).
    pub(crate) fn clamped(multiplicities: Vec<usize>, degree: usize) -> Result<Self, SplineError> {
        let issue = |reason| Err(SplineError::KnotVectorInvalid { reason });
        if degree == 0 {
            return issue(KnotVectorIssue::DegreeZero);
        }
        end_runs_clamped(&multiplicities, degree)?;
        // Indexing justified: exact end runs of `degree + 1` summing to
        // at least `2·(degree + 1)` are two runs at least.
        let mut index = degree + 1;
        for &m in &multiplicities[1..multiplicities.len() - 1] {
            if m > degree {
                return issue(KnotVectorIssue::InteriorMultiplicityTooHigh { index });
            }
            index += m;
        }
        Ok(Self::assembled(multiplicities, degree))
    }

    /// The structure of runs already validated.
    fn assembled(multiplicities: Vec<usize>, degree: usize) -> Self {
        let run_of = multiplicities
            .iter()
            .enumerate()
            .flat_map(|(run, &m)| core::iter::repeat_n(run, m))
            .collect();
        Self {
            degree,
            run_of,
            multiplicities,
        }
    }

    /// The degree `p`.
    pub fn degree(&self) -> usize {
        self.degree
    }

    /// The knot count.
    pub fn len(&self) -> usize {
        self.run_of.len()
    }

    /// Never: a clamped structure holds at least `2(p + 1)` knots.
    pub fn is_empty(&self) -> bool {
        self.run_of.is_empty()
    }

    /// The control-point count: `len − degree − 1`.
    pub fn control_count(&self) -> usize {
        self.len() - self.degree - 1
    }

    /// Each run's multiplicity, ascending, clamp runs included.
    pub fn multiplicities(&self) -> &[usize] {
        &self.multiplicities
    }

    /// The runs a spline on this structure may have no derivative
    /// across, by index into [`Self::multiplicities`]: its C⁰ creases,
    /// the interior runs of multiplicity at least the degree (every
    /// interior run at degree 1). The one statement of where a crease
    /// is: [`KnotVector::c1`] refuses exactly when this is nonempty,
    /// and a consumer locating the creases reads it too.
    pub fn crease_runs(&self) -> impl Iterator<Item = usize> + '_ {
        let last = self.multiplicities.len() - 1;
        (1..last).filter(move |&r| self.multiplicities[r] >= self.degree)
    }

    /// The run knot `index` belongs to.
    pub fn run_of(&self, index: usize) -> usize {
        self.run_of[index]
    }

    /// Whether knots `i` and `j` are one run: one breakpoint, so their
    /// difference is the zero a basis recursion's convention reads.
    pub fn same_run(&self, i: usize, j: usize) -> bool {
        self.run_of[i] == self.run_of[j]
    }

    /// The index of the first (nonempty) span: `degree`.
    pub fn first_span(&self) -> usize {
        self.degree
    }

    /// The index of the last (nonempty) span: `len − degree − 2`.
    pub fn last_span(&self) -> usize {
        self.len() - self.degree - 2
    }

    /// Whether `span` is nonempty: its two knots are two runs.
    pub fn span_is_nonempty(&self, span: usize) -> bool {
        span + 1 < self.len() && !self.same_run(span, span + 1)
    }

    /// The validated [`Span`] at `index`, as [`KnotVector::span`], over
    /// this structure alone: its [`Span::knots_f64_interim`] refuses.
    /// Crate-internal: [`super::Knots::span`] is the door, and it takes
    /// the vector's span where there is one.
    pub(crate) fn span(&self, index: usize) -> Option<Span<'_>> {
        self.span_over(None, index)
    }

    /// The span at `index` over this structure, beside `kv` when the
    /// knots are points.
    fn span_over<'a>(&'a self, kv: Option<&'a KnotVector>, index: usize) -> Option<Span<'a>> {
        if index < self.first_span() || index > self.last_span() || !self.span_is_nonempty(index) {
            return None;
        }
        Some(Span {
            structure: self,
            kv,
            index,
            first_control: index - self.degree,
            degree: self.degree,
        })
    }
}

/// A knot value **proven strictly interior** to the domain of the
/// [`KnotVector`] that minted it — the values knot insertion may be
/// asked to insert, as a type rather than as a precondition a caller
/// promises in prose.
///
/// Its field is private and its only constructors are
/// [`KnotVector::interior_knot_runs`] (the vector's own interior values,
/// interior by the construction invariant) and
/// [`KnotVector::interior_knot`] (the one filter for a value supplied
/// from outside), so an end value, an out-of-domain value or a NaN is
/// not a representable insertion point.
///
/// **What the type proves, and what privacy proves.** Unlike [`Span`],
/// which borrows the vector it indexes and so cannot be held beside
/// another one, it carries no borrow of its vector: a knot interior to
/// vector A handed to vector B's raw list IS representable, so the type
/// alone does not make the insertion guard unnecessary. What does is the type PLUS the
/// privacy of its consumers:
///
/// - [`super::algebra::insert_knot_plan`] and its siblings mint every
///   `InteriorKnot` from the vector they plan against.
/// - [`super::algebra::union_refinements`] reads several vectors' runs
///   and hands each vector the values it lacks from the OTHERS — the
///   one place a knot crosses vectors. It therefore returns plain
///   `f64`, not this type: the proof stops at the vector the knot came
///   from, and the insertion is re-validated by [`super::algebra::refine_plan`]
///   against the vector it lands in.
///
/// The type stays crate-private; a public one would need the borrow
/// [`Span`] carries, and — the second precedent — the one
/// [`super::hull::SplineCoeffs`] carries for a coefficient array.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct InteriorKnot(Param);

impl InteriorKnot {
    /// The knot value.
    pub(crate) fn value(self) -> f64 {
        self.0.get()
    }
}

/// A span index **proven** in range and nonempty for the knot vector it
/// indexes, carried **together with that vector** — a borrow, so the
/// span and the structure it is a proof about are one value.
///
/// The window it selects (`[index − degree, index]`) is computed once,
/// at construction, so `span − degree` never appears at a use site and
/// cannot underflow there.
///
/// **Branded to its knot structure by the borrow.** The fields are
/// private and the only constructors are [`KnotVector::span`]
/// (checked), [`KnotVector::span_of`] (total over a [`Param`]) and
/// [`KnotStructure::span`] (checked, over a structure with no `f64`
/// vector, whose [`Span::knots_f64_interim`] refuses), so an index
/// invalid for the structure it names is not a representable state —
/// and neither is a span held beside a *different* vector: every door
/// that consumes a `Span` reads its knots through the span
/// ([`Span::knots_f64_interim`], or the held values the same
/// [`super::KnotSpan`] pairs with it) and takes no second
/// [`KnotVector`] parameter to disagree with it. [`super::basis`] and [`super::hull`] therefore
/// index without a pairing guard and without a poison route for one.
/// (A span over knots wider than a point has no `f64` vector: its
/// [`Span::knots_f64_interim`] refuses loud, the reader's kernel bug,
/// and the held entry points read its values instead.)
/// `geom` carries the shape one level further: `CurveWindow{2,3}`
/// borrows the curve and `SurfaceWindow` the surface, so the doors
/// that read a control net get the net and the knots from one borrow
/// too.
///
/// **One level down, coefficients against knots take the same shape**:
/// a [`super::hull::SplineCoeffs`] (or a [`super::hull::RationalCoeffs`]
/// with the weights beside) borrows the vector its array was fitted
/// against and is minted only by [`KnotVector::with_coeffs`] and
/// [`KnotVector::with_rational_coeffs`] (the count relation, checked
/// once there), and every hull door reads through a window the pair
/// minted — a `Span` of ITS vector beside the pair. `InteriorKnot` is
/// the third member of the family and stays crate-private for it.
///
/// **Equality is address equality on the structure**, plus the indices:
/// a `Span` is a proof about *that* structure, and two bit-equal vectors
/// at different addresses are two structures a proof does not
/// transfer between. (`KnotVector` is not [`Eq`] — its knots are
/// `f64` — so a by-value derive is not available here in any case.)
///
/// # What does not typecheck
///
/// These are library doctests: they run under
/// `cargo test -p geom-core --doc`, so the claims below redden if the
/// borrow is undone rather than merely dating a comment.
///
/// **A door takes one vector, and it is the span's.** There is no
/// parameter through which a second one could arrive, so a span of one
/// vector cannot be evaluated against another.
///
/// ```compile_fail,E0061
/// use geom_core::spline::{KnotVector, basis};
/// let long = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 2.0, 3.0, 3.0, 3.0], 2).unwrap();
/// let short = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
/// let span = long.span(3).unwrap();
/// let _ = basis::basis_funs(&short, span, 0.5f64);
/// ```
///
/// Its twin, differing in one respect — the second vector is not
/// passed, because there is nowhere to pass it:
///
/// ```
/// use geom_core::spline::{KnotVector, basis};
/// let long = geom_core::spline::KnotVector::clamped(
///     vec![0.0, 0.0, 0.0, 1.0, 2.0, 3.0, 3.0, 3.0], 2).unwrap();
/// let span = long.span(3).unwrap();
/// let row = basis::basis_funs(span, 0.5f64);
/// assert_eq!(row.len(), long.degree() + 1);
/// ```
///
/// **A span cannot outlive its vector**, so it cannot be carried to
/// where another one is in scope:
///
/// ```compile_fail,E0597
/// use geom_core::spline::{KnotVector, basis};
/// let span = {
///     let kv = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
///     kv.span_at(0.5).unwrap()
/// };
/// let _ = basis::basis_funs(span, 0.5f64);
/// ```
///
/// The twin differs in one identifier — the vector is bound outside
/// the block, so the borrow lives as long as the span:
///
/// ```
/// use geom_core::spline::{KnotVector, basis};
/// let kv = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
/// let span = { kv.span_at(0.5).unwrap() };
/// let _ = basis::basis_funs(span, 0.5f64);
/// ```
///
/// **Nor can a span be held across a rebinding of its vector.** Every
/// knot-algebra door here is `&self -> Self`, so a refinement is a new
/// binding and a span of the original goes on naming the original:
///
/// ```compile_fail,E0506
/// use geom_core::spline::{KnotVector, basis};
/// let mut kv = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
/// let span = kv.span_at(0.5).unwrap();
/// kv = KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.5, 1.0, 1.0, 1.0], 2).unwrap();
/// let _ = basis::basis_funs(span, 0.5f64);
/// ```
///
/// The twin differs in one identifier — the refinement is a *new*
/// binding, which is what every knot-algebra door here returns:
///
/// ```
/// use geom_core::spline::{KnotVector, basis};
/// let kv = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
/// let span = kv.span_at(0.5).unwrap();
/// let refined = KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.5, 1.0, 1.0, 1.0], 2).unwrap();
/// let _ = basis::basis_funs(span, 0.5f64);
/// let _ = basis::basis_funs(refined.span_at(0.5).unwrap(), 0.5f64);
/// ```
///
/// **What these rows do and do not check.** Stable rustdoc checks only
/// that a `compile_fail` block fails to build; the `,E0061` /
/// `,E0597` / `,E0506` annotation beside it is **not** verified there
/// (that is a nightly rustdoc feature), so a row could be red for a
/// typo instead of for its subject. That is what each twin above is
/// for: it differs from its block in exactly one respect and it
/// compiles, so a typo shared by both would redden the twin. The
/// codes themselves were read off `rustc` directly on each snippet at
/// the pinned toolchain — and the first snippet emits `E0277`
/// (`Span<'_>: Real` unsatisfied, the knot vector landing in `t`'s
/// position) alongside its `E0061`, so the annotation names one of
/// two codes there rather than the only one.
#[derive(Clone, Copy)]
pub struct Span<'a> {
    structure: &'a KnotStructure,
    /// The `f64` vector, where the knots are points (C6's interim).
    kv: Option<&'a KnotVector>,
    index: usize,
    first_control: usize,
    degree: usize,
}

/// The borrow is printed as an ADDRESS, never followed: a derived
/// `Debug` would dump the whole knot vector through the reference at
/// every `{:?}`, which is the one cost a borrow-carrying token can
/// impose by accident. The address is also what equality reads.
///
/// **Both walks destructure `Self` exhaustively**, so a field added to
/// the declaration is an E0027 unbound-pattern error rather than a
/// value silently outside the dump and outside equality — an `Eq` that
/// misses a field answers wrong, where a `Debug` that misses one only
/// misleads. `finish` stands because every field IS shown; the borrow
/// is shown as its address, which is a summary of a field and not the
/// absence of one.
impl core::fmt::Debug for Span<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let Self {
            structure,
            kv,
            index,
            first_control,
            degree,
        } = self;
        f.debug_struct("Span")
            .field("structure", &core::ptr::from_ref(*structure))
            .field("kv", &kv.map(core::ptr::from_ref))
            .field("index", index)
            .field("first_control", first_control)
            .field("degree", degree)
            .finish()
    }
}

impl PartialEq for Span<'_> {
    fn eq(&self, other: &Self) -> bool {
        let Self {
            structure,
            kv: _,
            index,
            first_control,
            degree,
        } = self;
        let Self {
            structure: other_structure,
            kv: _,
            index: other_index,
            first_control: other_first_control,
            degree: other_degree,
        } = other;
        core::ptr::eq(*structure, *other_structure)
            && index == other_index
            && first_control == other_first_control
            && degree == other_degree
    }
}

impl Eq for Span<'_> {}

impl<'a> Span<'a> {
    /// The structure this span is a proof about.
    pub fn structure(self) -> &'a KnotStructure {
        self.structure
    }

    /// **C6's interim**: the `f64` [`KnotVector`] this span is a proof
    /// about, where the carrier's knots are points. A span over a
    /// carrier with a knot wider than a point has none, and reaching
    /// for it there is a kernel bug, refused loud with the reader's
    /// location: such a carrier's positions are read through its held
    /// values ([`super::KnotSpan`]), never past the funnel. Counted by
    /// `scripts/gates/knots-f64-interim.sh`; it goes in C6's S5.
    ///
    /// # Panics
    ///
    /// On a span over a carrier whose knots are not all points.
    #[track_caller]
    #[allow(clippy::panic)]
    pub fn knots_f64_interim(self) -> &'a KnotVector {
        // A `match`, not `unwrap_or_else`: `Location::caller()` inside a
        // closure names this file, not the reader.
        match self.kv {
            Some(kv) => kv,
            None => panic!(
                "a reader at {} took the f64 knot vector of a carrier whose knots are not \
                 all points (C6: its positions are its held values)",
                core::panic::Location::caller()
            ),
        }
    }

    /// The span index itself (the `i` of `knots[i] ≤ t < knots[i+1]`).
    pub fn index(self) -> usize {
        self.index
    }

    /// The degree this span was validated against.
    pub fn degree(self) -> usize {
        self.degree
    }

    /// The **first** control point of the window — `index − degree`,
    /// subtracted once at construction. This is what evaluation adds
    /// its basis-row offset to, so no use site performs the
    /// subtraction and none can underflow.
    pub fn first_control(self) -> usize {
        self.first_control
    }

    /// The inclusive control-point window the span selects:
    /// `[index − degree, index]`, always `degree + 1` entries.
    pub fn window(self) -> core::ops::RangeInclusive<usize> {
        self.first_control..=self.first_control + self.degree
    }

    /// The window active on the net obtained by **differencing `order`
    /// times**: `[index − degree, index − order]`, `degree + 1 − order`
    /// entries. Each differencing drops the top index, so this is
    /// [`Span::window`] shortened from the high end — the shape every
    /// derivative-coefficient hull needs.
    ///
    /// `None` when `order > degree`: a degree-`p` span has no active
    /// window on a net differenced more than `p` times, and that is a
    /// case the caller must answer — typically with the same
    /// `Option`/zero it already carries for the derived NET — rather
    /// than index into. The subtraction is inside the invariant
    /// (`order ≤ degree ≤ index`), so it cannot underflow here either.
    pub fn derived_window(self, order: usize) -> Option<core::ops::RangeInclusive<usize>> {
        (order <= self.degree).then(|| self.first_control..=self.index - order)
    }

    /// [`Span::derived_window`] at `order = 1`, **total**: a
    /// [`KnotVector`] refuses degree 0 at construction, so every span
    /// has `degree ≥ 1`, `index ≥ first_control + 1`, and the
    /// once-differenced window is nonempty.
    pub fn first_derived_window(self) -> core::ops::RangeInclusive<usize> {
        self.first_control..=self.index - 1
    }
}

impl KnotVector {
    /// The Bézier vector of degree `degree` on `[s0, s1]`: each end
    /// `degree + 1` times and no interior knot — the vector a bare
    /// Bernstein row on `[s0, s1]` is a proof about.
    ///
    /// # Errors
    ///
    /// As [`KnotVector::clamped`]: degree zero, or ends that are not
    /// finite and ordered.
    pub fn bezier(degree: usize, (s0, s1): (f64, f64)) -> Result<Self, SplineError> {
        let mut knots = vec![s0; degree + 1];
        knots.extend(core::iter::repeat_n(s1, degree + 1));
        Self::clamped(knots, degree)
    }

    /// Validates and wraps a clamped knot vector. See the type docs for
    /// the exact invariants.
    ///
    /// # Errors
    ///
    /// [`SplineError::KnotVectorInvalid`] naming the violated clause.
    pub fn clamped(knots: Vec<f64>, degree: usize) -> Result<Self, SplineError> {
        let issue = |reason| Err(SplineError::KnotVectorInvalid { reason });
        if degree == 0 {
            return issue(KnotVectorIssue::DegreeZero);
        }
        end_clamped(&knots, degree)?;
        // Interior multiplicity ≤ degree: the same run scan
        // [`KnotVector::interior_knots`] serves, one frame before the
        // type exists — which is why it goes through [`runs_in`] over
        // [`interior_of`] rather than through the methods. (The end
        // runs are exact, so interior values differ from both end
        // values.) `interior_of`'s precondition is the `TooShort`
        // clause, which `end_clamped` checks.
        let mut index = degree + 1;
        for (_, mult) in runs_in(interior_of(&knots, degree)) {
            if mult > degree {
                return issue(KnotVectorIssue::InteriorMultiplicityTooHigh { index });
            }
            index += mult;
        }
        Ok(Self::assemble(knots, degree))
    }

    /// A validated vector with its structure read off it.
    fn assemble(knots: Vec<f64>, degree: usize) -> Self {
        let multiplicities = runs_in(&knots).map(|(_, m)| m).collect();
        Self {
            structure: KnotStructure::assembled(multiplicities, degree),
            knots,
            degree,
        }
    }

    /// The combinatorial part (C6).
    pub fn structure(&self) -> &KnotStructure {
        &self.structure
    }

    /// The degree `p` this knot vector is clamped for.
    pub fn degree(&self) -> usize {
        self.degree
    }

    /// The knots, non-decreasing, ends at multiplicity `degree + 1`.
    pub fn knots(&self) -> &[f64] {
        &self.knots
    }

    /// The control-point count this knot vector requires:
    /// `knots.len() − degree − 1`.
    pub fn control_count(&self) -> usize {
        self.knots.len() - self.degree - 1
    }

    /// The parameter domain `[knots[p], knots[len − 1 − p]]`, strictly
    /// forward: the end runs are exactly `p + 1` long and the knots
    /// non-decreasing, so `knots[p] < knots[p + 1] ≤ knots[len − 1 − p]`.
    pub fn domain(&self) -> (f64, f64) {
        // Indexing justified: len ≥ 2(p+1) (construction invariant).
        (
            self.knots[self.degree],
            self.knots[self.knots.len() - 1 - self.degree],
        )
    }

    /// The same clamped structure re-expressed affinely on `[lo, hi]`:
    /// the two clamp runs are `lo` and `hi` **exactly** — assigned, not
    /// computed, so the result's [`KnotVector::domain`] is `(lo, hi)`
    /// bit for bit — and every interior knot `k` of this vector, whose
    /// domain is `[a, b]`, becomes `lo + (hi − lo)·((k − a)/(b − a))`.
    /// On the unit domain that is `lo + (hi − lo)·k`.
    ///
    /// Assigning the ends is what makes the door exact: `lo + (hi − lo)`
    /// is not `hi` in `f64` in general (an ulp off either way), and a
    /// vector whose last knot is an ulp past the interval it is meant
    /// to be expressed on is on some other interval.
    ///
    /// **Degree and knot count are unchanged, so
    /// [`KnotVector::control_count`] is unchanged** — the fact a curve
    /// carrying its net verbatim onto the result rests on.
    ///
    /// The map cannot itself produce a vector `clamped` refuses: equal
    /// source knots have equal images, and the map is monotone for
    /// `hi > lo` (each of `− a`, `/ (b − a)`, `· (hi − lo)`, `+ lo` is a
    /// monotone rounding of a monotone step), so the images keep the
    /// source's order and end runs of exactly `degree + 1` copies each
    /// of `lo < hi`; every knot is finite because `hi − lo` is (the
    /// guard refuses a width that overflows). What rounding can still
    /// do is collapse two DISTINCT interior knots onto one value
    /// (`InteriorMultiplicityTooHigh`), an interior knot onto an end
    /// value (`StartNotClamped`/`EndNotClamped`), or an interior knot
    /// PAST the pinned `hi` (`Decreasing`) — the last when the
    /// pull-back `(k − a)/(b − a)` of a knot within rounding of `b`
    /// rounds to exactly `1`, so the image is the computed
    /// `lo + (hi − lo)` the pin exists to avoid. The result goes
    /// through [`KnotVector::clamped`] precisely so that each collapse
    /// is refused by the clause that names it rather than minted.
    ///
    /// The interior is not round-trip stable: re-expressing on
    /// `[lo, hi]` and back re-rounds every interior knot twice, and a
    /// cubic interpolant's 13-knot vector comes back from
    /// `[0, 1] → [0.3, 0.9] → [0, 1]` with 2 of its knots (both
    /// interior) an ulp off; only the assigned ends survive.
    ///
    /// # Errors
    ///
    /// [`SplineError::DomainInvalid`] when `lo` or `hi` is not finite,
    /// `hi ≤ lo`, or `hi − lo` overflows; otherwise whatever clause of
    /// [`KnotVector::clamped`] a rounding collapse trips (above).
    pub fn on_domain(&self, lo: f64, hi: f64) -> Result<Self, SplineError> {
        let span = hi - lo;
        if !(lo.is_finite() && hi.is_finite() && lo < hi && span.is_finite()) {
            return Err(SplineError::DomainInvalid { lo, hi });
        }
        let p = self.degree;
        let (a, b) = self.domain();
        let interior = self
            .interior()
            .iter()
            .map(|k| lo + span * ((k - a) / (b - a)));
        let knots: Vec<f64> = core::iter::repeat_n(lo, p + 1)
            .chain(interior)
            .chain(core::iter::repeat_n(hi, p + 1))
            .collect();
        Self::clamped(knots, p)
    }

    /// Refuses unless this vector is its own reflection about its
    /// domain: `k_i + k_{m−i} = lo + hi` in ℝ for every `i`, with `m`
    /// the last knot index. Reflecting a knot vector (`k ↦ lo + hi − k`
    /// with the order reversed) carries each basis function to its
    /// mirror partner, so on a vector that passes, a net read backwards
    /// over the SAME knots is the original reparameterized by
    /// `t ↦ lo + hi − t`.
    ///
    /// # What the symmetric vectors actually are
    ///
    /// "Symmetric" means symmetric AFTER decimal-to-binary rounding,
    /// which is narrower than it reads: an interior pair a user types
    /// as mirrored — thirds, `0.1/0.9`, `0.2/0.8`, `0.3/0.7`,
    /// `0.45/0.55` — has a real sum that misses `lo + hi` by one 2Sum
    /// residual (±5.55e−17, or half that for `0.1/0.9`) and REFUSES,
    /// while `0.4/0.6` and every dyadic pair accept.
    ///
    /// # Why the test is exact, and why the clamp runs never trip it
    ///
    /// The condition is an identity between REAL numbers, and
    /// `fl(k_i + k_{m−i}) == fl(lo + hi)` does not decide it: on
    /// `lo = 0`, `hi = 1` the pair `(½, ½ + 2⁻⁵³)` rounds to `1.0` and
    /// would pass while reflecting an ulp away from its partner. So the
    /// two sums are compared AS EXACT SUMS — each held as a rounded
    /// head plus its exact residual ([`two_sum`]) and both components
    /// compared under IEEE equality, which is equality of the real sums
    /// and nothing weaker. (IEEE equality, not bit equality: `-0.0`
    /// equals `0.0`, so a vector carrying a `-0.0` where its partner
    /// carries `0.0` is accepted as the symmetric vector it really is.)
    /// Every step is one correctly-rounded binary64 operation, so the
    /// verdict is a function of the knots' bits alone: the same answer
    /// on every target and at every optimization level. The one case
    /// the arithmetic cannot decide is a `lo + hi` that overflows, and
    /// that is refused rather than answered.
    ///
    /// Knots are finite by construction; a NaN or infinite knot would
    /// in any case give a residual that compares equal to nothing, so
    /// the test would refuse it rather than admit it.
    ///
    /// The clamped ends never trip it. [`Self::domain`] reads `lo` and
    /// `hi` off `knots[p]` and `knots[m − p]`, and the end runs are
    /// equal under `==` (the comparison [`Self::clamped`] ran), so for
    /// every `i ≤ p` the pair `(k_i, k_{m−i})` equals `(lo, hi)` in
    /// value and the test compares a value against itself. Only the
    /// interior can refuse.
    ///
    /// # Errors
    ///
    /// [`KnotMirrorError::ReflectionNotFinite`] when `lo + hi`
    /// overflows; otherwise [`KnotMirrorError::AsymmetricPair`] naming
    /// the first pair, from the outside in, whose exact sum misses.
    ///
    /// [`two_sum`]: crate::exact::two_sum
    pub fn mirror_symmetric(&self) -> Result<(), KnotMirrorError> {
        let Some(index) = self.first_unreflected(self)? else {
            return Ok(());
        };
        let (lo, hi) = self.domain();
        let mirror_index = self.knots.len() - 1 - index;
        Err(KnotMirrorError::AsymmetricPair {
            index,
            mirror_index,
            knot: self.knots[index],
            mirror_knot: self.knots[mirror_index],
            lo,
            hi,
        })
    }

    /// Whether `other` is this vector reflected through a point: the
    /// same degree and length, and `k_i + k'_{m−i} = S` in ℝ for every
    /// `i`, with `S = lo + hi'` the sum of this vector's first knot and
    /// `other`'s last. A net over `other` read backwards is then a net
    /// over `self` reparameterized by `t ↦ S − t`. Reflected about its
    /// own domain a vector has `S = lo + hi`, and
    /// [`Self::mirror_symmetric`] is this test with `other = self`; its
    /// docs say why the sums are compared exact — rebuilding the
    /// mirror as `fl(lo + hi − k)` and comparing knots both admits a
    /// vector an ulp off the mirror and refuses an exact one.
    ///
    /// An `S` that overflows decides nothing, and reads as `false`.
    pub fn is_reflection_of(&self, other: &Self) -> bool {
        self.degree == other.degree
            && self.knots.len() == other.knots.len()
            && matches!(self.first_unreflected(other), Ok(None))
    }

    /// This vector reflected through 0, `k'_i = −k_{m−i}` on
    /// `[−hi, −lo]`: the one reflection every vector has exactly, since
    /// negation does not round. A net over it is this vector's net
    /// read backwards, reparameterized by `t ↦ −t`
    /// ([`Self::is_reflection_of`] with `S = 0`).
    #[must_use]
    pub fn negated(&self) -> Self {
        // `0.0 − k` rather than `−k`, so a zero knot stays `+0.0`.
        let knots = self.knots.iter().rev().map(|k| 0.0 - k).collect();
        Self::clamped(knots, self.degree)
            .unwrap_or_else(|e| unreachable!("a clamped vector reflected through 0: {e}"))
    }

    /// The first index `i` whose knot and `other`'s `m − i` do not sum
    /// exactly to `S`, this vector's first knot plus `other`'s last —
    /// the one reflection rule, read by [`Self::mirror_symmetric`] and
    /// [`Self::is_reflection_of`]. `other` has this vector's length.
    /// The refusal's `lo` and `hi` are those two knots, so for
    /// `other = self`, the only caller that surfaces it, they are the
    /// domain's ends.
    fn first_unreflected(&self, other: &Self) -> Result<Option<usize>, KnotMirrorError> {
        // Indexing justified: a clamped vector is never empty.
        let (lo, hi) = (self.knots[0], other.knots[other.knots.len() - 1]);
        let reflection = two_sum(lo, hi);
        if !reflection.0.is_finite() {
            return Err(KnotMirrorError::ReflectionNotFinite { lo, hi });
        }
        Ok(self
            .knots
            .iter()
            .zip(other.knots.iter().rev())
            .position(|(&k, &mirror)| two_sum(k, mirror) != reflection))
    }

    /// The index of the first (nonempty) span: `degree`.
    pub fn first_span(&self) -> usize {
        self.degree
    }

    /// The index of the last (nonempty) span: `len − degree − 2`.
    pub fn last_span(&self) -> usize {
        self.knots.len() - self.degree - 2
    }

    /// The whole domain as a [`ParamRange`], the range a window is
    /// clamped to.
    pub fn domain_range(&self) -> ParamRange {
        let (lo, hi) = self.domain();
        // `clamped` refuses a non-finite knot and a collapsed domain.
        ParamRange::new(lo, hi).unwrap_or_else(|| unreachable!("a clamped domain is ordered"))
    }

    /// The inclusive span range overlapped by `range`, each end located
    /// by [`KnotVector::span_at`]'s tie-break. Total: a [`ParamRange`]
    /// has no NaN end, and it is ordered, so `first ≤ last`.
    ///
    /// Both ends are [`Span`]s: locating is where span validity
    /// originates, and each end borrows this vector, so there is
    /// nothing for a caller to re-check and no second vector for either
    /// end to be paired against. Iterate the interior with
    /// `first.index() + 1 ..= last.index()` and [`KnotVector::span`],
    /// which refuses the empty spans in between.
    pub fn span_range(&self, range: ParamRange) -> (Span<'_>, Span<'_>) {
        let (start, end) = range.ends();
        (self.span_of(start), self.span_of(end))
    }

    /// Whether `span` is a **nonempty** span (`knots[span] <
    /// knots[span+1]`). Interior knot multiplicities create empty
    /// spans; their basis denominators are zero, so evaluation treats
    /// them as invalid (poison). [`KnotVector::span_at`] never
    /// returns one — every parameter `t`, including a repeated knot
    /// value `u` itself, is assigned to the nonempty span *starting*
    /// at it — so multi-span hull iteration skips empty spans without
    /// discarding any parameter's span.
    pub fn span_is_nonempty(&self, span: usize) -> bool {
        span + 1 < self.knots.len() && self.knots[span] < self.knots[span + 1]
    }

    /// The validated [`Span`] at `index`, or `None` when the index is
    /// out of range or names an **empty** span (interior knot
    /// multiplicity). This and the locators ([`KnotVector::span_of`],
    /// and [`KnotVector::span_at`] and [`KnotVector::span_range`] over
    /// it) are the only ways to obtain a `Span`.
    pub fn span(&self, index: usize) -> Option<Span<'_>> {
        if index < self.first_span() || index > self.last_span() || !self.span_is_nonempty(index) {
            return None;
        }
        // Justified once, here: `index >= first_span() == degree`, so
        // the subtraction cannot underflow, and `index <= last_span()`
        // puts `index + 1` inside the knot array. Every consumer of the
        // resulting `Span` inherits both facts.
        self.structure.span_over(Some(self), index)
    }

    /// Locates the span containing `t`: the index `i` with
    /// `knots[i] ≤ t < knots[i+1]` (half-open), the **last** span
    /// closed — the fixed tie-break: at an interior knot value the
    /// returned span is the one *starting* there (the last copy's
    /// span, which is nonempty by the multiplicity invariant).
    ///
    /// `None` exactly when `t` is NaN: a NaN names no place in the
    /// domain, so it has no span, and poison never chooses structure.
    /// Every number has one: `t` below the domain returns the first
    /// span and `t` at or above the domain end the last (each then
    /// evaluates the span's polynomial extension — the documented
    /// garbage-out contract of `eval_in_span`).
    pub fn span_at(&self, t: f64) -> Option<Span<'_>> {
        Param::new(t).map(|t| self.span_of(t))
    }

    /// [`KnotVector::span_at`] for a value that is a number by type:
    /// total, with the same clamping and tie-break.
    pub fn span_of(&self, t: Param) -> Span<'_> {
        self.span_from_offset(search_offset_in(&self.knots, self.degree, t))
    }

    /// The [`Span`] at a search result. The search runs in window
    /// coordinates, so its result *is* the window's first control
    /// point: there is no subtraction to check. Nonemptiness comes from
    /// the search's exits — the clamped ends are nonempty by the
    /// end-multiplicity invariant, and the search maintains
    /// `knots[i] ≤ t < knots[i + 1]` strictly.
    fn span_from_offset(&self, first_control: usize) -> Span<'_> {
        let index = first_control + self.degree;
        // Nonemptiness is the argument the basis denominators rest on:
        // an empty span here would divide by a zero knot difference and
        // poison silently, where the `span()` route returned `None`.
        debug_assert!(
            self.span_is_nonempty(index),
            "span_at located an empty span {index}"
        );
        Span {
            structure: &self.structure,
            kv: Some(self),
            index,
            first_control,
            degree: self.degree,
        }
    }

    /// The multiplicity of the exact value `u` among the knots (exact
    /// `f64` equality — structure identity), together with the index
    /// of the **last** knot equal to `u`; `None` if `u` is not a knot.
    pub fn multiplicity_of(&self, u: f64) -> Option<(usize, usize)> {
        let mut count = 0;
        let mut last = None;
        for (i, k) in self.knots.iter().enumerate() {
            if *k == u {
                count += 1;
                last = Some(i);
            }
        }
        last.map(|i| (count, i))
    }

    /// **The knot slice a once-differenced spline is built from**:
    /// this vector's knots minus one at each end.
    ///
    /// Differentiating a degree-`p` B-spline drops the outer knot pair
    /// and the degree by one (The NURBS Book Eq. 3.4), so every
    /// consumer that materialises a derivative structure starts here —
    /// whether it goes on to build a [`KnotVector`] (the clamped case)
    /// or a [`LooseKnots`] because the result has an interior
    /// multiplicity the clamped invariant refuses.
    ///
    /// A slice, not a vector: the raw-slice consumers difference level
    /// after level without a clone, and the ones that need an owned
    /// copy say `.to_vec()` at the site.
    #[must_use]
    pub fn derivative_knot_slice(&self) -> &[f64] {
        derivative_knot_slice(self.knots())
    }

    /// The knots strictly between the two clamp runs — `knots[p + 1..
    /// len − p − 1]`, with multiplicities kept, so a run scan over it
    /// yields exactly [`KnotVector::interior_knots`]. Empty exactly
    /// when there is one span. The slice every interior-only operation
    /// starts from: the multiplicity scan, the affine re-expression
    /// ([`KnotVector::on_domain`], whose ends are assigned rather than
    /// mapped) and a concatenation that mints its own ends.
    #[must_use]
    pub fn interior(&self) -> &[f64] {
        interior_of(&self.knots, self.degree)
    }

    /// The distinct **interior** knot values with their multiplicities,
    /// ascending — the query [`KnotVector::multiplicity_of`] cannot
    /// serve, because that one needs the value before it can answer.
    /// Exact `f64` equality throughout, the same structure-identity
    /// rule `multiplicity_of` uses: never a tolerance question.
    ///
    /// Total, and read-only: a single-span vector yields nothing, and
    /// the items are values, so no caller reaches a state
    /// [`KnotVector::clamped`] refuses. Two facts hold of every item by
    /// the construction invariants, and consumers may rely on them —
    /// each multiplicity is in `1..=degree`, and each value lies
    /// **strictly inside** [`KnotVector::domain`] (the end runs are
    /// exact, so no interior knot equals either end value).
    pub fn interior_knots(&self) -> impl DoubleEndedIterator<Item = (f64, usize)> + Clone + '_ {
        self.interior_knot_runs().map(|(k, m)| (k.value(), m))
    }

    /// [`KnotVector::interior_knots`] with each value carried as the
    /// [`InteriorKnot`] it is — the typed form, and the one the
    /// untyped form is defined over. The strictly-interior fact is
    /// this vector's construction invariant, so the items are minted
    /// without a check.
    pub(crate) fn interior_knot_runs(
        &self,
    ) -> impl DoubleEndedIterator<Item = (InteriorKnot, usize)> + Clone + '_ {
        // A clamped vector's knots are finite, so each is a `Param`.
        runs_in(self.interior()).filter_map(|(v, m)| Some((InteriorKnot(Param::new(v)?), m)))
    }

    /// `u` as an [`InteriorKnot`] of this vector — strictly inside
    /// [`KnotVector::domain`] — or `None`. **The one filter** for a
    /// value that did not come from the vector itself: an extra break
    /// a caller wants inserted goes through here, and NaN is refused
    /// because `u > lo` is false for it.
    pub(crate) fn interior_knot(&self, u: f64) -> Option<InteriorKnot> {
        let (lo, hi) = self.domain();
        let u = Param::new(u)?;
        (u.get() > lo && u.get() < hi).then_some(InteriorKnot(u))
    }

    /// Every knot run, **including the two clamps**, as
    /// `(value, multiplicity)` ascending — the whole-vector form of
    /// [`KnotVector::interior_knots`], which the interior slice cannot
    /// express. Same exact-`f64` identity, same totality; the first and
    /// last items always carry multiplicity `degree + 1`.
    ///
    /// This is the run-length encoding a serializer needs: STEP's
    /// `B_SPLINE_CURVE_WITH_KNOTS` takes exactly this pair of lists.
    /// Both methods are one line over [`runs_in`], so there is no second
    /// scan here to keep in agreement with the first.
    pub fn knot_runs(&self) -> impl DoubleEndedIterator<Item = (f64, usize)> + Clone + '_ {
        runs_in(&self.knots)
    }

    /// The clamped single-segment (Bézier) vector on `[0, 1]`:
    /// `degree + 1` zeros followed by `degree + 1` ones. Total, with
    /// nothing refused and nothing substituted: the clamped-v1 form
    /// represents every degree except 0, and the argument's type is
    /// exactly that set, so every spellable call names a vector
    /// [`KnotVector::clamped`] would accept. `NonZeroUsize::MIN` is the
    /// degree-1 segment the placeholder payloads ride.
    ///
    /// Degree 0 — the request `clamped` answers with `DegreeZero` when
    /// it arrives as data — is not a spellable argument here:
    ///
    /// ```compile_fail,E0308
    /// # use geom_core::spline::KnotVector;
    /// let _ = KnotVector::unit_segment(0);
    /// ```
    pub fn unit_segment(degree: NonZeroUsize) -> Self {
        let p = degree.get();
        let mut knots = vec![0.0; p + 1];
        knots.extend(core::iter::repeat_n(1.0, p + 1));
        Self::assemble(knots, p)
    }

    /// Crate-internal constructor for knot vectors produced by the
    /// knot-algebra plans, whose outputs preserve the invariants by
    /// construction (each op inserts/removes copies of interior values
    /// within the multiplicity budget, or re-clamps ends explicitly).
    ///
    /// **A plan whose output [`KnotVector::clamped`] would refuse is a
    /// kernel bug detectable only by re-deriving that validation — D2
    /// addendum row 5, whose mechanism is `debug_assert`, and this is
    /// it.** The refusal names the violation, so the assert reports
    /// which invariant the plan broke rather than that one did. Where
    /// debug assertions are compiled out the constructor is total and
    /// the structure is re-checked by any subsequent `clamped` round
    /// trip; the root `[profile.release]` sets `debug-assertions =
    /// true`, so today they are not compiled out anywhere.
    pub(crate) fn from_algebra(knots: Vec<f64>, degree: usize) -> Self {
        #[cfg(debug_assertions)]
        {
            let revalidated = Self::clamped(knots.clone(), degree);
            debug_assert!(
                revalidated.is_ok(),
                "a knot-algebra plan produced structure `clamped` refuses \
                 ({:?}): degree {degree}, knots {knots:?}",
                revalidated.as_ref().err(),
            );
            revalidated.unwrap_or_else(|_| Self::assemble(knots, degree))
        }
        #[cfg(not(debug_assertions))]
        {
            Self::assemble(knots, degree)
        }
    }
}

/// The **runs of equal values** in a sorted `f64` slice, ascending, as
/// `(value, multiplicity)`. A caller that needs each run's index into
/// a parent array accumulates the multiplicities, which is one line and
/// keeps this signature the one every caller actually wants.
///
/// The primitive under [`KnotVector::interior_knots`],
/// [`KnotVector::knot_runs`] and [`KnotVector::clamped`]'s own
/// interior-multiplicity check — the last of which runs *before* a
/// `KnotVector` exists, so it cannot go through either method, exactly
/// as the pre-`KnotVector` span search cannot go through
/// [`KnotVector::span_of`] and goes through [`search_offset_in`]
/// instead.
///
/// Runs are cut on **exact `f64` equality**: knots are structure, and
/// structure identity is bitwise-value identity, never a tolerance
/// question. `-0.0` and `+0.0` are therefore ONE run (`==` holds), which
/// is not how `total_cmp` orders them — the two rules meet only here,
/// and this one is the multiplicity rule.
///
/// `sorted` must be non-decreasing, which is what makes equal values
/// adjacent and a run-length walk equal to "count every equal element".
/// A violation is not unsound — the walk still terminates and still
/// yields a partition — it just splits one value into several runs, so
/// the caller's multiplicity is wrong rather than absent. Total on any
/// slice, empty included.
fn runs_in(sorted: &[f64]) -> impl DoubleEndedIterator<Item = (f64, usize)> + Clone + '_ {
    // Indexing justified: `chunk_by` never yields an empty run.
    sorted
        .chunk_by(|a, b| a == b)
        .map(|run| (run[0], run.len()))
}

/// The span **offset above `degree`** located for `t` in a clamped knot
/// list — the span search for **clamped** lists, shared by
/// [`KnotVector::span_of`], [`LooseKnots::span_of`] and by the knot-algebra paths that hold a
/// raw list mid-mutation and so have no [`KnotVector`] to ask.
///
/// It is [`last_at_or_below`] over the span starts
/// `knots[degree ..= degree + last]`, clamped to the first span below
/// the domain. At or above the domain end it lands on the last span
/// because the slice stops at that span's start; at an interior knot it
/// lands on the run's last copy, the nonempty span starting there.
/// It reads only the end-run clauses, which [`LooseKnots`] keeps
/// where a `KnotVector`'s interior clause does not hold.
///
/// **Preconditions, and what a violation costs.** Taking a slice rather
/// than `&self` moves two facts from *guaranteed by the type* to
/// *required of the caller*: `knots.len() ≥ 2·degree + 2` (a shorter
/// slice underflows `last`), and `knots` non-decreasing (otherwise the
/// answer is in range but arbitrary). Both are strictly weaker than
/// [`KnotVector`]'s construction invariant, so a `KnotVector`'s own
/// knots always satisfy them, as a [`LooseKnots`]'s do; the raw knot-algebra paths satisfy them
/// because they start from a `KnotVector`'s knots and only insert
/// interior values. A short slice underflows `usize` arithmetic — a
/// debug panic, a wrapped index in release — which is why the doors
/// over a raw slice ([`find_span_in`]) stay crate-internal. NaN is not
/// a precondition: `t` is a [`Param`].
fn search_offset_in(knots: &[f64], degree: usize, t: Param) -> usize {
    // `last` is the span count, len − 2·degree − 2: non-negative by
    // the construction invariant len ≥ 2(degree + 1).
    let (p, last) = (degree, knots.len() - 2 * degree - 2);
    // Indexing justified: p + last = len − degree − 2 < len.
    last_at_or_below(&knots[p..=p + last], t).unwrap_or(0)
}

/// [`KnotVector::span_at`] against a raw clamped knot list, at a value
/// proven strictly interior to its domain: the span index rather than
/// the offset, same tie-break (at an interior knot value, the span
/// *starting* there).
///
/// **This is not "the last index `i` with `knots[i] ≤ t`".** The two
/// coincide on `t ∈ [knots[degree], knots[len − degree − 1])` and
/// nowhere else: at or above the domain end this returns the last span
/// while that scan walks on into the trailing clamp. Substituting this
/// for such a scan is sound only under that half-open precondition,
/// which is the substituting frame's to state.
pub(crate) fn find_span_in(knots: &[f64], degree: usize, u: InteriorKnot) -> usize {
    search_offset_in(knots, degree, u.0) + degree
}

/// [`KnotVector::derivative_knot_slice`] on a raw knot slice — the
/// same "drop one knot at each end", for [`LooseKnots::derivative`].
///
/// Fewer than two knots answers the empty slice rather than panicking.
#[must_use]
pub(crate) fn derivative_knot_slice(knots: &[f64]) -> &[f64] {
    knots.get(1..knots.len().saturating_sub(1)).unwrap_or(&[])
}

/// The clauses a clamped knot list keeps at every degree, its interior
/// aside: at least `2·(degree + 1)` knots, all finite, non-decreasing,
/// and both end runs exactly `degree + 1` long ([`end_runs_clamped`]).
/// [`KnotVector::clamped`] adds its degree and interior clauses to
/// these; [`LooseKnots::new`] is these alone.
fn end_clamped(knots: &[f64], degree: usize) -> Result<(), SplineError> {
    let issue = |reason| Err(SplineError::KnotVectorInvalid { reason });
    // Ahead of the value clauses: a short list is refused as short.
    if knots.len() < 2 * (degree + 1) {
        return issue(KnotVectorIssue::TooShort);
    }
    for (index, k) in knots.iter().enumerate() {
        if !k.is_finite() {
            return issue(KnotVectorIssue::NonFinite { index });
        }
    }
    for index in 1..knots.len() {
        // Indexing justified: index ∈ [1, len), index − 1 ∈ [0, len).
        if knots[index] < knots[index - 1] {
            return issue(KnotVectorIssue::Decreasing { index });
        }
    }
    let multiplicities: Vec<usize> = runs_in(knots).map(|(_, m)| m).collect();
    end_runs_clamped(&multiplicities, degree)
}

/// The length and end-run clauses over a list's run multiplicities —
/// all a structure without positions has to read
/// ([`KnotStructure::clamped`]), and the last step of
/// [`end_clamped`]: at least `2·(degree + 1)` knots, then a first and a
/// last run of exactly `degree + 1`.
fn end_runs_clamped(multiplicities: &[usize], degree: usize) -> Result<(), SplineError> {
    let issue = |reason| Err(SplineError::KnotVectorInvalid { reason });
    if multiplicities.iter().sum::<usize>() < 2 * (degree + 1) {
        return issue(KnotVectorIssue::TooShort);
    }
    if multiplicities.first() != Some(&(degree + 1)) {
        return issue(KnotVectorIssue::StartNotClamped);
    }
    if multiplicities.last() != Some(&(degree + 1)) {
        return issue(KnotVectorIssue::EndNotClamped);
    }
    Ok(())
}

/// A knot list clamped at both ends that a [`KnotVector`] cannot hold:
/// its interior runs may be of any length and its degree may be 0. A
/// derivative's knots are this shape where they are no clamped vector
/// ([`super::Derived::Discontinuous`]: an interior run longer than the
/// derivative's degree; [`super::Derived::SpanConstant`]: degree 0),
/// and so is every derivative below one ([`LooseKnots::derivative`]).
///
/// Its ends are a clamped vector's (end runs of exactly `degree + 1`),
/// so it locates spans as one does ([`LooseKnots::span_of`]).
#[derive(Clone, Debug, PartialEq)]
pub struct LooseKnots {
    knots: Vec<f64>,
    degree: usize,
}

impl LooseKnots {
    /// `knots` at `degree`, validated as clamped at both ends.
    ///
    /// # Errors
    ///
    /// [`SplineError::KnotVectorInvalid`] naming the violated clause,
    /// as [`KnotVector::clamped`] names it: `TooShort`, `NonFinite`,
    /// `Decreasing`, `StartNotClamped` or `EndNotClamped`.
    pub fn new(knots: Vec<f64>, degree: usize) -> Result<Self, SplineError> {
        end_clamped(&knots, degree)?;
        Ok(Self { knots, degree })
    }

    /// The knots.
    pub fn knots(&self) -> &[f64] {
        &self.knots
    }

    /// The degree `p`.
    pub fn degree(&self) -> usize {
        self.degree
    }

    /// The coefficient count: `len − degree − 1`.
    pub fn control_count(&self) -> usize {
        self.knots.len() - self.degree - 1
    }

    /// The derivative's knots — one knot dropped at each end (as
    /// [`KnotVector::derivative_knot_slice`]) and the degree one less — or `None`
    /// at degree 0. Each end run loses one knot and the degree one, so
    /// the result is clamped at both ends again.
    #[must_use]
    pub fn derivative(&self) -> Option<Self> {
        let degree = self.degree.checked_sub(1)?;
        Some(Self {
            knots: derivative_knot_slice(&self.knots).to_vec(),
            degree,
        })
    }

    /// The derivative's knots of a clamped vector — [`KnotVector::derivative_knot_slice`]
    /// at degree `p − 1` — the loose shape a derivative takes where it is no
    /// clamped vector ([`super::Derived`]). Unchecked, as
    /// [`LooseKnots::derivative`] is: `kv`'s knots are finite and
    /// non-decreasing, its end runs are exactly `p + 1` with `p ≥ 1`, and
    /// dropping one knot at each end leaves end runs of `p` at degree `p − 1`
    /// in `len − 2 ≥ 2p` knots.
    pub(crate) fn derivative_of(kv: &KnotVector) -> Self {
        Self {
            knots: kv.derivative_knot_slice().to_vec(),
            degree: kv.degree() - 1,
        }
    }

    /// The index of the span `t` lies in, with [`KnotVector::span_of`]'s
    /// clamping and tie-break: below the domain the first span, at or
    /// above its end the last, and at a knot value the span starting
    /// there. The answer is in `degree ..= control_count − 1` and names
    /// a nonempty span: the search lands on a run's last copy, and the
    /// end runs are exact.
    pub fn span_of(&self, t: Param) -> usize {
        let index = search_offset_in(&self.knots, self.degree, t) + self.degree;
        // Nonemptiness is what a derivative across a long run rests on:
        // its coefficient over an empty span is a filler, never read.
        debug_assert!(
            self.knots[index] < self.knots[index + 1],
            "LooseKnots::span_of located an empty span {index}"
        );
        index
    }
}

/// [`KnotVector::interior`] on a raw knot slice — the one home of the
/// slice bounds. Precondition: `knots.len() ≥ 2·(degree + 1)`, which
/// gives `degree + 1 ≤ len − degree − 1` so the range is valid (empty
/// exactly when there is one span). That is [`KnotVector::clamped`]'s
/// `TooShort` clause, checked before the one call made on a vector
/// not yet validated, and the construction invariant everywhere else.
fn interior_of(knots: &[f64], degree: usize) -> &[f64] {
    &knots[degree + 1..knots.len() - degree - 1]
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use test_utils::vacuity::Exposure;

    use super::*;

    fn kv(knots: &[f64], p: usize) -> KnotVector {
        KnotVector::clamped(knots.to_vec(), p).unwrap()
    }

    /// **A knot difference at `Interval` encloses the exact difference**
    /// of its two `f64` knots, decided exactly: `hi − lo = s + e` with
    /// `s` the rounded difference and `e` its TwoSum error, and the
    /// enclosure's ends are floats, so each end is compared with `s` and
    /// `e`'s sign. On these non-dyadic knots the subtraction rounds, and
    /// the thin `T::from_f64(hi - lo)` the speed meter once took held
    /// `s` alone, missing `s + e`. At `f64` the difference is `s`.
    #[test]
    fn a_knot_difference_encloses_the_exact_difference_of_non_dyadic_knots() {
        use crate::real::Bounds;
        let mut inexact = 0;
        for (lo, hi) in [(0.1, 0.7), (0.1, 0.3), (1e-3, 0.9), (0.3, 0.9), (0.05, 0.7)] {
            let (s, e) = two_sum(hi, -lo);
            if e != 0.0 {
                inexact += 1;
            }
            let d: crate::Interval = knot_difference(lo, hi);
            let below = d.lo() < s || (d.lo() == s && e >= 0.0);
            let above = d.hi() > s || (d.hi() == s && e <= 0.0);
            assert!(
                below && above,
                "[{lo}, {hi}]: [{:e}, {:e}] misses {s:e} + {e:e}",
                d.lo(),
                d.hi()
            );
            assert_eq!(knot_difference::<f64>(lo, hi).to_bits(), s.to_bits());
        }
        assert!(inexact >= 4, "the row's knots must round: {inexact} did");
    }

    /// A domain stored reversed or collapsed is not mintable, so a
    /// consumer metering `d1 - d0` (certify's span meter) sees a positive
    /// width. The reversal here is in the END runs, which an interior
    /// order check alone would not reach; `on_domain`'s reversed request
    /// is `on_domain_refusals_are_typed`'s row.
    #[test]
    fn a_reversed_or_collapsed_domain_is_not_mintable() {
        let bad = |knots: &[f64]| KnotVector::clamped(knots.to_vec(), 1).unwrap_err();
        assert_eq!(
            bad(&[1.0, 1.0, 0.0, 0.0]),
            SplineError::KnotVectorInvalid {
                reason: KnotVectorIssue::Decreasing { index: 2 }
            },
            "reversed end runs"
        );
        assert_eq!(
            bad(&[0.5, 0.5, 0.5, 0.5]),
            SplineError::KnotVectorInvalid {
                reason: KnotVectorIssue::StartNotClamped
            },
            "collapsed domain"
        );
    }

    #[test]
    fn validation_refusals_are_typed_and_exact() {
        let bad = |knots: &[f64], p: usize| KnotVector::clamped(knots.to_vec(), p).unwrap_err();
        assert_eq!(
            bad(&[0.0, 0.0, 1.0, 1.0], 0),
            SplineError::KnotVectorInvalid {
                reason: KnotVectorIssue::DegreeZero
            }
        );
        assert_eq!(
            bad(&[0.0, 0.0, 1.0], 1),
            SplineError::KnotVectorInvalid {
                reason: KnotVectorIssue::TooShort
            }
        );
        assert_eq!(
            bad(&[0.0, 0.0, f64::NAN, 1.0, 1.0, 1.0], 2),
            SplineError::KnotVectorInvalid {
                reason: KnotVectorIssue::NonFinite { index: 2 }
            }
        );
        assert_eq!(
            bad(&[0.0, 0.0, 0.5, 0.25, 1.0, 1.0], 1),
            SplineError::KnotVectorInvalid {
                reason: KnotVectorIssue::Decreasing { index: 3 }
            }
        );
        assert_eq!(
            bad(&[0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 1),
            SplineError::KnotVectorInvalid {
                reason: KnotVectorIssue::StartNotClamped
            }
        );
        assert_eq!(
            bad(&[0.0, 0.0, 1.0, 1.0, 1.0], 1),
            SplineError::KnotVectorInvalid {
                reason: KnotVectorIssue::EndNotClamped
            }
        );
        // p = 2 with an interior triple knot: multiplicity 3 > 2.
        assert_eq!(
            bad(&[0.0, 0.0, 0.0, 0.5, 0.5, 0.5, 1.0, 1.0, 1.0], 2),
            SplineError::KnotVectorInvalid {
                reason: KnotVectorIssue::InteriorMultiplicityTooHigh { index: 3 }
            }
        );
    }

    /// The ends of a re-expressed vector are the requested ends BIT
    /// FOR BIT, on a pair where the affine image of the source's end is
    /// not: `0.3 + (0.9 − 0.3)` is one ulp above `0.9` in `f64`, so a
    /// computed end would put the domain an ulp past the interval asked
    /// for. Degree and count are unchanged, interior knots are the
    /// affine image, and a repeated source knot stays repeated.
    #[test]
    fn on_domain_pins_the_ends_exactly_and_maps_the_interior_affinely() {
        let (lo, hi) = (0.3_f64, 0.9_f64);
        let computed_end = lo + (hi - lo);
        assert_eq!(
            computed_end.to_bits(),
            hi.to_bits() + 1,
            "the pair no longer documents the pin: {computed_end:e} vs {hi:e}"
        );

        // Unit-domain source with a double interior knot.
        let src = kv(&[0.0, 0.0, 0.0, 0.25, 0.5, 0.5, 1.0, 1.0, 1.0], 2);
        let out = src.on_domain(lo, hi).unwrap();
        let (olo, ohi) = out.domain();
        assert_eq!((olo.to_bits(), ohi.to_bits()), (lo.to_bits(), hi.to_bits()));
        assert_eq!(out.degree(), src.degree());
        assert_eq!(out.knots().len(), src.knots().len());
        assert_eq!(out.control_count(), src.control_count());
        let span = hi - lo;
        let want: Vec<f64> = vec![
            lo,
            lo,
            lo,
            lo + span * 0.25,
            lo + span * 0.5,
            lo + span * 0.5,
            hi,
            hi,
            hi,
        ];
        let bits = |v: &[f64]| v.iter().map(|k| k.to_bits()).collect::<Vec<_>>();
        assert_eq!(bits(out.knots()), bits(&want));

        // A source off the unit domain maps through its own `[a, b]`.
        let src = kv(&[1.0, 1.0, 1.0, 2.0, 3.0, 3.0, 3.0], 2);
        let out = src.on_domain(lo, hi).unwrap();
        let want = vec![
            lo,
            lo,
            lo,
            lo + span * ((2.0 - 1.0) / (3.0 - 1.0)),
            hi,
            hi,
            hi,
        ];
        assert_eq!(bits(out.knots()), bits(&want));
        assert_eq!(out.control_count(), src.control_count());
    }

    /// A domain that is not a finite increasing interval refuses as
    /// the domain's own defect, whatever clause a vector built on it
    /// would have tripped first; an interior collapse under rounding
    /// refuses through the clause that names it, never minting.
    #[test]
    fn on_domain_refusals_are_typed() {
        let src = kv(&[0.0, 0.0, 0.5, 1.0, 1.0], 1);
        let domain = |lo: f64, hi: f64| src.on_domain(lo, hi).unwrap_err();
        for (lo, hi) in [
            (f64::NAN, 1.0),
            (0.0, f64::NAN),
            (f64::NEG_INFINITY, 1.0),
            (0.0, f64::INFINITY),
        ] {
            assert!(
                matches!(
                    domain(lo, hi),
                    SplineError::DomainInvalid { lo: l, hi: h }
                        if l.to_bits() == lo.to_bits() && h.to_bits() == hi.to_bits()
                ),
                "({lo}, {hi})"
            );
        }
        assert_eq!(
            domain(2.0, 2.0),
            SplineError::DomainInvalid { lo: 2.0, hi: 2.0 }
        );
        assert_eq!(
            domain(1.0, 0.0),
            SplineError::DomainInvalid { lo: 1.0, hi: 0.0 }
        );
        // A width that overflows is a domain whose ends sit near the
        // ceiling of the range, and the refusal names them readably.
        let e = domain(-1e308, 1e308);
        assert_eq!(
            e,
            SplineError::DomainInvalid {
                lo: -1e308,
                hi: 1e308
            }
        );
        assert!(
            e.to_string()
                .starts_with("the domain [-1e308, 1e308] is not a finite increasing interval"),
            "{e}"
        );

        // Two distinct interior knots whose images coincide: at 1e16
        // the ulp is 2, so `0.3·8` and `0.35·8` both round onto
        // `1e16 + 2` — strictly inside the pinned ends, so the clause
        // that fires is the multiplicity one (degree 1 admits one).
        let (lo, hi) = (1e16_f64, 1.000_000_000_000_000_8e16_f64);
        assert_eq!(hi.to_bits(), lo.to_bits() + 4, "hi is four ulps above lo");
        let src = kv(&[0.0, 0.0, 0.3, 0.35, 1.0, 1.0], 1);
        assert_eq!(
            src.on_domain(lo, hi).unwrap_err(),
            SplineError::KnotVectorInvalid {
                reason: KnotVectorIssue::InteriorMultiplicityTooHigh { index: 2 }
            }
        );
        // An interior knot whose image rounds onto the pinned start.
        let src = kv(&[0.0, 0.0, 1e-17, 1.0, 1.0], 1);
        assert_eq!(
            src.on_domain(lo, hi).unwrap_err(),
            SplineError::KnotVectorInvalid {
                reason: KnotVectorIssue::StartNotClamped
            }
        );
        // …and onto the pinned end: the largest double below 1 scales
        // onto `1e16 + 8` exactly (`8·(1 − 2⁻⁵³)` is an ulp below 8,
        // which `1e16 + …` rounds away).
        let below_one = f64::from_bits(1.0_f64.to_bits() - 1);
        assert_eq!(lo + (hi - lo) * below_one, hi, "the witness rounds onto hi");
        let src = kv(&[0.0, 0.0, below_one, 1.0, 1.0], 1);
        assert_eq!(
            src.on_domain(lo, hi).unwrap_err(),
            SplineError::KnotVectorInvalid {
                reason: KnotVectorIssue::EndNotClamped
            }
        );
        // PAST the pinned end: on a source `[−1, 1]` the same knot's
        // pull-back `(k + 1)/2` rounds to exactly 1, so its image is
        // the computed `0.3 + 0.6` — an ulp above the assigned `0.9`.
        let (lo, hi) = (0.3_f64, 0.9_f64);
        assert_eq!((below_one - -1.0) / (1.0 - -1.0), 1.0);
        assert_eq!((lo + (hi - lo)).to_bits(), hi.to_bits() + 1);
        let src = kv(&[-1.0, -1.0, below_one, 1.0, 1.0], 1);
        assert_eq!(
            src.on_domain(lo, hi).unwrap_err(),
            SplineError::KnotVectorInvalid {
                reason: KnotVectorIssue::Decreasing { index: 3 }
            }
        );
    }

    #[test]
    fn span_at_half_open_with_closed_last_span_and_no_span_at_nan() {
        // Spans: [0,1) → 2, [1,2) → 4 (interior double knot at 1), [2,3] → 5.
        let k = kv(&[0.0, 0.0, 0.0, 1.0, 1.0, 2.0, 3.0, 3.0, 3.0], 2);
        let at = |t: f64| k.span_at(t).map(Span::index);
        assert_eq!(k.first_span(), 2);
        assert_eq!(k.last_span(), 5);
        assert_eq!(at(0.0), Some(2));
        assert_eq!(at(0.999), Some(2));
        // At the interior knot: the span STARTING there (last copy).
        assert_eq!(at(1.0), Some(4));
        assert_eq!(at(1.5), Some(4));
        assert_eq!(at(2.0), Some(5));
        // Last span closed; every number out of the domain clamps.
        assert_eq!(at(3.0), Some(5));
        assert_eq!(at(7.5), Some(5));
        assert_eq!(at(-1.0), Some(2));
        assert_eq!(at(f64::INFINITY), Some(5));
        assert_eq!(at(f64::NEG_INFINITY), Some(2));
        // NaN names no place in the domain, so it has no span.
        assert_eq!(at(f64::NAN), None, "a NaN parameter located a span");
        // Range form — validated ends, compared as the indices they name.
        let range = |lo, hi| {
            let r = ParamRange::new(lo, hi).expect("an ordered pair");
            let (a, b): (Span<'_>, Span<'_>) = k.span_range(r);
            (a.index(), b.index())
        };
        assert_eq!(range(0.5, 2.5), (2, 5));
        assert_eq!(range(1.25, 1.75), (4, 4));
        assert_eq!(range(-4.0, 9.0), (2, 5), "out-of-domain ends clamp");
        let d = k.domain_range();
        assert_eq!((d.start(), d.end()), (0.0, 3.0));
    }

    #[test]
    fn accessors_and_multiplicity() {
        let k = kv(&[0.0, 0.0, 0.0, 0.5, 1.0, 1.0, 1.0], 2);
        assert_eq!(k.degree(), 2);
        assert_eq!(k.control_count(), 4);
        assert_eq!(k.domain(), (0.0, 1.0));
        assert_eq!(k.multiplicity_of(0.5), Some((1, 3)));
        assert_eq!(k.multiplicity_of(0.0), Some((3, 2)));
        assert_eq!(k.multiplicity_of(0.25), None);
    }

    /// **The span search against a definitional oracle**, at every exit
    /// its contract names.
    ///
    /// [`find_span_in`] and [`KnotVector::span_at`] are one
    /// search: both reduce to `search_offset_in(knots, degree, t) +
    /// degree`, so no probe can separate them and an assertion that
    /// they agree is satisfied by construction. What CAN go red is the
    /// search itself, so that is what this row drives — a linear scan
    /// written from the documented contract, independent of the binary
    /// search it checks:
    ///
    /// - at NaN, **no** span;
    /// - below the domain, the **first** span;
    /// - at or above the domain end, the **last** span;
    /// - inside, the unique `i` with `knots[i] ≤ t < knots[i+1]`, ties
    ///   broken toward the span *starting* at a repeated knot.
    ///
    /// Plus the divergence [`find_span_in`]'s own docs warn about and
    /// nothing else pinned: at or above the domain end it is **not**
    /// "the last index `i` with `knots[i] ≤ t`", which walks on into the
    /// trailing clamp. A refactor that quietly substituted such a scan
    /// would pass every in-domain probe.
    ///
    /// The probe classes are censused and floored, because a builder
    /// change that emptied one — no interior knot, no out-of-domain
    /// probe — would leave the row green over a contract it never
    /// exercised.
    #[test]
    fn the_span_search_matches_its_definitional_oracle_at_every_exit() {
        // The contract, written as a linear scan. `first`/`last` are
        // the span indices, not offsets.
        fn oracle(knots: &[f64], degree: usize, t: f64) -> Option<usize> {
            let (first, last) = (degree, knots.len() - degree - 2);
            if t.is_nan() {
                return None;
            }
            if t <= knots[first] {
                return Some(first);
            }
            if t >= knots[last + 1] {
                return Some(last);
            }
            let mut got = first;
            for i in first..=last {
                if knots[i] <= t && t < knots[i + 1] {
                    got = i;
                }
            }
            Some(got)
        }
        // "The last index `i` with `knots[i] ≤ t`" — the scan the
        // `find_span_in` docs say this is NOT.
        fn last_index_at_or_below(knots: &[f64], t: f64) -> Option<usize> {
            knots.iter().rposition(|&k| k <= t)
        }

        let cases = [
            (vec![0.0, 0.0, 1.0, 1.0], 1),
            (vec![0.0, 0.0, 0.0, 0.5, 1.0, 1.0, 1.0], 2),
            (vec![0.0, 0.0, 0.0, 0.5, 0.5, 1.0, 1.0, 1.0], 2),
            (
                vec![-2.0, -2.0, -2.0, -2.0, -0.5, 0.0, 0.0, 3.0, 3.0, 3.0, 3.0],
                3,
            ),
        ];
        let mut census = Exposure::new("knots: the span search against its oracle");
        for (knots, p) in cases {
            let k = kv(&knots, p);
            let (lo, hi) = k.domain();
            let interior: Vec<f64> = knots[p + 1..knots.len() - p - 1].to_vec();
            let mut probes = vec![lo, hi, lo - 1.0, hi + 1.0, f64::NAN, f64::INFINITY, -0.0];
            probes.extend(knots.iter().copied());
            let mut distinct = knots.clone();
            distinct.dedup();
            probes.extend(distinct.windows(2).map(|w| 0.5 * (w[0] + w[1])));
            for t in probes {
                census.note(if t.is_nan() {
                    "NaN"
                } else if t < lo {
                    "below the domain"
                } else if t > hi {
                    "above the domain end"
                } else if t == hi {
                    "at the domain end"
                } else if interior.contains(&t) {
                    "at an interior knot"
                } else {
                    "strictly inside a span"
                });
                let located = k.span_at(t).map(Span::index);
                assert_eq!(
                    located,
                    oracle(&knots, p, t),
                    "p{p} at {t}: the search left its documented contract"
                );
                let Some(got) = located else { continue };
                // Equal BY CONSTRUCTION today — both doors are one
                // search. Kept as one line so a future edit that gives
                // them separate bodies reds here; it is not this row's
                // evidence, which is the oracle above.
                if let Some(u) = k.interior_knot(t) {
                    assert_eq!(
                        got,
                        find_span_in(&knots, p, u),
                        "p{p} at {t}: the two doors diverged"
                    );
                }
                // The documented divergence, where it applies.
                if t >= hi {
                    let naive = last_index_at_or_below(&knots, t)
                        .expect("at or above the domain end some knot is ≤ t");
                    assert!(
                        got < naive,
                        "p{p} at {t}: the span search returned {got}, the same as the \
                         `knots[i] ≤ t` scan — the trailing-clamp divergence its docs \
                         warn about has been lost"
                    );
                }
            }
        }
        census.report();
        // Every exit the contract names must have been probed. A floor
        // of 1 is enough: these are enumerated, not sampled, so a class
        // that reaches 0 means the probe list stopped covering it.
        census.require_each(
            &[
                "NaN",
                "below the domain",
                "above the domain end",
                "at the domain end",
                "at an interior knot",
                "strictly inside a span",
            ],
            1,
            "the probe list no longer reaches this exit, so the contract it states is \
             unchecked here",
        );
    }

    /// Every rendering of a knot-vector violation names what to change
    /// in the description, not only what is wrong with it.
    ///
    /// The vocabulary is a vocabulary and not a part-of-speech test:
    /// an arm that names the thing a caller supplies satisfies the
    /// claim the same way an imperative does.
    #[test]
    fn every_knot_vector_issue_arm_names_a_recourse() {
        const RECOURSE_WORDS: &[&str] = &["supply", "describe", "repeat", "drop"];
        let arms = knot_vector_issue_arms();
        assert_eq!(arms.len(), 7, "an arm was added without a row here");
        for arm in &arms {
            let text = arm.to_string();
            assert_eq!(
                test_utils::refusal::recourse_markers(&text),
                1,
                "not exactly one labelled repair: {text}"
            );
            let msg = text.to_lowercase();
            assert!(
                RECOURSE_WORDS.iter().any(|w| msg.contains(w)),
                "no recourse in: {msg}"
            );
        }
    }

    /// The test decides the REAL identity, not the rounded one. This
    /// vector's interior pair sums to `1 + 2⁻⁵³`, which rounds to
    /// exactly `lo + hi` — a comparison of rounded sums would admit it.
    #[test]
    fn a_pair_whose_rounded_sum_hits_the_midline_still_refuses() {
        let half_up = f64::from_bits(0.5f64.to_bits() + 1);
        assert_eq!(
            0.5 + half_up,
            0.0 + 1.0,
            "the rounded sums agree, so only an exact test can tell these apart"
        );
        assert_ne!(half_up, 0.5, "the partner really is an ulp off the midline");
        let e = kv(&[0.0, 0.0, 0.0, 0.5, half_up, 1.0, 1.0, 1.0], 2)
            .mirror_symmetric()
            .unwrap_err();
        assert_eq!(
            e,
            KnotMirrorError::AsymmetricPair {
                index: 3,
                mirror_index: 4,
                knot: 0.5,
                mirror_knot: half_up,
                lo: 0.0,
                hi: 1.0,
            },
            "an exact-sum test refuses what a rounded-sum test would admit"
        );
        assert!(
            e.to_string().contains("do not sum to 1 exactly."),
            "the message states the reflection it wanted, evaluated: {e}"
        );
    }

    /// Two vectors are reflections of each other when their pairwise
    /// sums are `lo + hi` in ℝ, which the rounded mirror `lo + hi − k`
    /// misreads both ways: on `[0.1, 0.3]` the knots `0.25` and `0.15`
    /// sum to `0.4` exactly while `fl(0.1 + 0.3 − 0.25)` is not `0.15`,
    /// and on `[0, 1]` `fl(1 − 0.1)` is `0.9` while `0.1 + 0.9` is not 1.
    #[test]
    fn a_reflection_is_decided_on_exact_sums_not_on_a_rounded_mirror() {
        let row = kv(&[0.1, 0.1, 0.25, 0.3, 0.3], 1);
        let exact_mirror = kv(&[0.1, 0.1, 0.15, 0.3, 0.3], 1);
        assert_ne!(
            0.1 + 0.3 - 0.25,
            0.15,
            "the rounded mirror misses the exact one"
        );
        assert!(
            row.is_reflection_of(&exact_mirror) && exact_mirror.is_reflection_of(&row),
            "0.25 + 0.15 = 0.1 + 0.3 in ℝ, so each is the other's reflection"
        );
        let through_zero = row.negated();
        assert_eq!(through_zero.knots(), &[-0.3, -0.3, -0.25, -0.1, -0.1]);
        assert!(
            row.is_reflection_of(&through_zero) && !row.is_reflection_of(&row),
            "a vector is its own reflection through 0, and not about its domain"
        );
        let row = kv(&[0.0, 0.0, 0.1, 1.0, 1.0], 1);
        let rounded_mirror = kv(&[0.0, 0.0, 0.9, 1.0, 1.0], 1);
        assert_eq!(1.0 - 0.1, 0.9, "the rounded mirror reads as this vector");
        assert!(
            !row.is_reflection_of(&rounded_mirror),
            "0.1 + 0.9 misses 1 by a 2Sum residual, so this is not a reflection"
        );
        assert!(
            row.is_reflection_of(&row.negated()),
            "the reflection through 0 is exact where the one about [0, 1] is not"
        );
        assert!(
            !row.is_reflection_of(&kv(&[0.0, 0.0, 0.0, 0.9, 1.0, 1.0, 1.0], 2)),
            "a different degree is not a reflection"
        );
        let symmetric = kv(&[0.0, 0.0, 0.0, 0.25, 0.75, 1.0, 1.0, 1.0], 2);
        assert!(
            symmetric.is_reflection_of(&symmetric),
            "a mirror-symmetric vector is its own reflection"
        );
    }

    /// A self-paired middle knot is compared against itself, and its
    /// refusal reads as a midpoint claim rather than as a pair that does
    /// not sum to itself. The midpoint is accepted.
    #[test]
    fn a_self_paired_knot_refuses_as_a_midpoint() {
        assert_eq!(
            kv(&[0.0, 0.0, 0.0, 0.5, 1.0, 1.0, 1.0], 2).mirror_symmetric(),
            Ok(()),
            "the middle knot is the midpoint"
        );
        let e = kv(&[0.0, 0.0, 0.0, 0.25, 1.0, 1.0, 1.0], 2)
            .mirror_symmetric()
            .unwrap_err();
        assert_eq!(
            e,
            KnotMirrorError::AsymmetricPair {
                index: 3,
                mirror_index: 3,
                knot: 0.25,
                mirror_knot: 0.25,
                lo: 0.0,
                hi: 1.0,
            }
        );
        assert!(
            e.to_string().starts_with(
                "knot mirror: the middle knot 3 (0.25) is not the midpoint of [0, 1]."
            ),
            "{e}"
        );
    }

    /// A domain whose reflection overflows is refused rather than
    /// answered. Without the guard the scan would not pass vacuously —
    /// the overflowing head's residual is a NaN and compares equal to
    /// nothing — it would refuse at index 0 and blame the clamp pair;
    /// the guard names the domain's defect instead.
    #[test]
    fn a_domain_with_no_finite_reflection_refuses() {
        let (lo, hi): (f64, f64) = (1e308, 1.5e308);
        assert!(!(lo + hi).is_finite(), "the fixture's reflection overflows");
        assert!(
            two_sum(lo, hi).1.is_nan(),
            "so the residual is a NaN, and an unguarded scan would refuse at index 0"
        );
        let e = kv(&[lo, lo, lo, hi, hi, hi], 2)
            .mirror_symmetric()
            .unwrap_err();
        assert_eq!(
            e,
            KnotMirrorError::ReflectionNotFinite { lo, hi },
            "the test refuses a reflection it cannot compute"
        );
        assert!(
            e.to_string().starts_with(
                "knot mirror: the domain [1e308, 1.5e308] has no finite reflection sum"
            ),
            "the refusal names a domain at the ceiling of the range readably: {e}"
        );
        // And it refuses a vector that IS its own reflection in ℝ, for
        // the same reason: the test that would admit it cannot be run.
        assert_eq!(
            kv(&[lo, lo, lo, 1.25e308, hi, hi, hi], 2).mirror_symmetric(),
            Err(KnotMirrorError::ReflectionNotFinite { lo, hi }),
            "a genuinely symmetric vector whose lo + hi overflows is refused too"
        );
    }

    /// Every [`KnotMirrorError`] rendering names exactly one labelled
    /// repair, like the module's other refusals.
    #[test]
    fn every_knot_mirror_error_arm_names_a_recourse() {
        let pair = |index, mirror_index| KnotMirrorError::AsymmetricPair {
            index,
            mirror_index,
            knot: 0.25,
            mirror_knot: 0.5,
            lo: 0.0,
            hi: 1.0,
        };
        let arms = [
            KnotMirrorError::ReflectionNotFinite {
                lo: 1e308,
                hi: 1.5e308,
            },
            pair(3, 3),
            pair(3, 4),
        ];
        for arm in &arms {
            let text = arm.to_string();
            assert_eq!(
                test_utils::refusal::recourse_markers(&text),
                1,
                "not exactly one labelled repair: {text}"
            );
            let lower = text.to_lowercase();
            assert!(
                ["supply", "re-express"].iter().any(|w| lower.contains(w)),
                "no recourse in: {text}"
            );
        }
    }

    /// One of each [`KnotVectorIssue`], so the row above and the
    /// transitive arm of the row below read the same population.
    fn knot_vector_issue_arms() -> [KnotVectorIssue; 7] {
        [
            KnotVectorIssue::DegreeZero,
            KnotVectorIssue::TooShort,
            KnotVectorIssue::NonFinite { index: 2 },
            KnotVectorIssue::Decreasing { index: 3 },
            KnotVectorIssue::StartNotClamped,
            KnotVectorIssue::EndNotClamped,
            KnotVectorIssue::InteriorMultiplicityTooHigh { index: 4 },
        ]
    }

    /// Every `SplineError` rendering names a repair — and
    /// `KnotVectorInvalid`, which renders its carrier whole and
    /// contributes four words of its own, is asserted TRANSITIVELY: at
    /// every payload, not at one chosen because it happens to carry a
    /// clause. `KnotVectorIssue` has an enforcement row of its own
    /// directly above, which is what licenses the transitive reading
    /// here.
    #[test]
    fn every_spline_error_arm_names_a_recourse() {
        const RECOURSE_WORDS: &[&str] = &["supply", "ask", "describe", "repeat", "drop"];
        let arms = [
            SplineError::KnotVectorInvalid {
                reason: KnotVectorIssue::DegreeZero,
            },
            SplineError::NonPositiveWeight {
                index: 1,
                weight: 0.0,
            },
            SplineError::NonFiniteWeight {
                index: 1,
                weight: f64::INFINITY,
            },
            SplineError::ControlCountMismatch {
                control: 3,
                expected: 4,
            },
            SplineError::WeightCountMismatch {
                weights: 3,
                control: 4,
            },
            SplineError::DomainInvalid { lo: 1.0, hi: 1.0 },
            SplineError::KnotIsPoison { index: 1 },
            SplineError::KnotRunsNotIncreasing { index: 2 },
            SplineError::KnotRunsUndecided {
                index: 2,
                margin: crate::MarginDiag::enclosure(-0.25, 0.5),
            },
            SplineError::CarrierKnotNotAPoint { index: 1 },
            SplineError::CarrierWeightNotAPoint { index: 1 },
            SplineError::WeightSignUndecided {
                index: 1,
                margin: crate::MarginDiag::enclosure(-0.25, 0.5),
            },
        ];
        // Exhaustive by the compiler: a new arm fails to build here
        // until it is listed above.
        for e in &arms {
            match e {
                SplineError::KnotVectorInvalid { .. }
                | SplineError::NonPositiveWeight { .. }
                | SplineError::NonFiniteWeight { .. }
                | SplineError::ControlCountMismatch { .. }
                | SplineError::WeightCountMismatch { .. }
                | SplineError::DomainInvalid { .. }
                | SplineError::KnotIsPoison { .. }
                | SplineError::KnotRunsNotIncreasing { .. }
                | SplineError::KnotRunsUndecided { .. }
                | SplineError::CarrierKnotNotAPoint { .. }
                | SplineError::CarrierWeightNotAPoint { .. }
                | SplineError::WeightSignUndecided { .. } => {}
            }
        }
        assert_eq!(arms.len(), 12, "an arm was added without a row here");
        // The repair is labelled, once: a wrapper that forwards this
        // carrier whole and adds a `Recourse:` of its own is then two
        // to the checker, not one it cannot see.
        let check = |msg: &str| {
            assert_eq!(
                test_utils::refusal::recourse_markers(msg),
                1,
                "not exactly one labelled repair: {msg}"
            );
            let lower = msg.to_lowercase();
            assert!(
                RECOURSE_WORDS.iter().any(|w| lower.contains(w)),
                "no recourse in: {msg}"
            );
        };
        for arm in &arms {
            match arm {
                SplineError::KnotVectorInvalid { .. } => {
                    for reason in &knot_vector_issue_arms() {
                        let msg = SplineError::KnotVectorInvalid { reason: *reason }.to_string();
                        assert!(
                            msg.contains(&reason.to_string()),
                            "carrier not rendered whole: {msg}"
                        );
                        check(&msg);
                    }
                }
                _ => check(&arm.to_string()),
            }
        }
    }

    /// A clamped vector's knots read as [`LooseKnots`] locate every
    /// parameter in the vector's own span: below, inside, at each knot
    /// value and above the domain.
    #[test]
    fn loose_knots_locate_as_their_clamped_vector_does() {
        let kv = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 2.5, 4.0, 4.0, 4.0], 2).unwrap();
        let loose = LooseKnots::new(kv.knots().to_vec(), 2).unwrap();
        for t in [-1.0, 0.0, 0.5, 1.0, 2.0, 2.5, 3.9, 4.0, 9.0] {
            let t = Param::new(t).unwrap();
            assert_eq!(
                loose.span_of(t),
                kv.span_of(t).index(),
                "span of {t:?} disagrees with the vector's"
            );
        }
    }

    /// Across an interior run longer than the degree — the shape no
    /// clamped vector holds — the located span is the nonempty one
    /// starting at the run's last copy, at degree 1 and at degree 0.
    #[test]
    fn a_loose_span_is_the_nonempty_one_past_a_long_run() {
        let raw = LooseKnots::new(vec![0.0, 0.0, 1.0, 1.0, 1.0, 2.0, 2.0], 1).unwrap();
        let constant = LooseKnots::new(vec![0.0, 1.0, 1.0, 2.0], 0).unwrap();
        for (name, knots, table) in [
            (
                "degree 1",
                &raw,
                [
                    (-1.0, 1),
                    (0.0, 1),
                    (0.5, 1),
                    (1.0, 4),
                    (1.5, 4),
                    (2.0, 4),
                    (3.0, 4),
                ],
            ),
            (
                "degree 0",
                &constant,
                [
                    (-1.0, 0),
                    (0.0, 0),
                    (0.5, 0),
                    (1.0, 2),
                    (1.5, 2),
                    (2.0, 2),
                    (3.0, 2),
                ],
            ),
        ] {
            for (t, want) in table {
                assert_eq!(
                    knots.span_of(Param::new(t).unwrap()),
                    want,
                    "{name}: span of {t}"
                );
            }
        }
    }

    /// A list open at an end, or too short for its degree, is refused
    /// by the clause [`KnotVector::clamped`] names; the derivative of a
    /// loose list is the loose list one degree down, and degree 0 has
    /// none.
    #[test]
    fn loose_knots_refuse_an_unclamped_end_and_step_down_a_degree() {
        let invalid = |knots: Vec<f64>, degree| match LooseKnots::new(knots, degree) {
            Err(SplineError::KnotVectorInvalid { reason }) => reason,
            other => panic!("expected a refusal, got {other:?}"),
        };
        assert_eq!(invalid(vec![0.0], 0), KnotVectorIssue::TooShort);
        assert_eq!(
            invalid(vec![0.0, 0.0, 1.0, 1.0, 1.0], 1),
            KnotVectorIssue::EndNotClamped
        );
        assert_eq!(
            invalid(vec![0.0, 1.0, 1.0, 2.0, 2.0], 1),
            KnotVectorIssue::StartNotClamped
        );
        let quadratic =
            LooseKnots::new(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 2.0, 2.0, 2.0], 2).unwrap();
        let linear = quadratic.derivative().unwrap();
        assert_eq!(
            linear,
            LooseKnots::new(vec![0.0, 0.0, 1.0, 1.0, 1.0, 2.0, 2.0], 1).unwrap()
        );
        let constant = linear.derivative().unwrap();
        assert_eq!((constant.degree(), constant.control_count()), (0, 4));
        assert_eq!(constant.derivative(), None, "degree 0 has no derivative");
    }

    /// A structure refuses its ends by the clauses its vector does: one
    /// run long enough for two clamps is an unclamped start, and two
    /// clamp runs with nothing between are a valid single span.
    #[test]
    fn a_structure_refuses_its_ends_as_its_vector_does() {
        let reason = |e: SplineError| match e {
            SplineError::KnotVectorInvalid { reason } => reason,
            other => panic!("expected a knot-vector refusal, got {other:?}"),
        };
        for (multiplicities, knots) in [
            (vec![6], vec![0.0; 6]),
            (vec![2, 3], vec![0.0, 0.0, 1.0, 1.0, 1.0]),
            (vec![3, 2], vec![0.0, 0.0, 0.0, 1.0, 1.0]),
            (vec![3], vec![0.0; 3]),
        ] {
            let structure = KnotStructure::clamped(multiplicities.clone(), 2).unwrap_err();
            let vector = KnotVector::clamped(knots, 2).unwrap_err();
            assert_eq!(reason(structure), reason(vector), "runs {multiplicities:?}");
        }
        assert!(
            KnotStructure::clamped(vec![3, 3], 2).is_ok(),
            "one span, two clamps"
        );
    }
}
