//! The validated clamped knot vector — pure **structure** (C6): `f64`
//! knots and a `usize` degree, produced and consumed by the
//! deterministic f64 lane. Raw `f64` comparisons are legal throughout
//! this file (structure selection, never a topology decision).

/// A typed construction failure for spline structure — fail-loud per
/// D4: every invalid input is a named refusal, never a silent repair.
#[derive(Clone, Debug, PartialEq)]
pub enum SplineError {
    /// The knot vector fails the clamped-v1 contract; `reason` names
    /// the exact violation.
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
}

impl core::fmt::Display for SplineError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            SplineError::KnotVectorInvalid { reason } => {
                write!(f, "invalid clamped knot vector: {reason}")
            }
            SplineError::NonPositiveWeight { index, weight } => write!(
                f,
                "weight {index} is {weight}, not strictly positive (convex-hull invariant)"
            ),
            SplineError::NonFiniteWeight { index, weight } => {
                write!(f, "weight {index} is {weight}, not finite")
            }
            SplineError::ControlCountMismatch { control, expected } => write!(
                f,
                "control-point count {control} does not match the knot vector (expected {expected})"
            ),
            SplineError::WeightCountMismatch { weights, control } => write!(
                f,
                "weight count {weights} does not match control-point count {control}"
            ),
        }
    }
}

impl core::error::Error for SplineError {}

/// The exact structural violation behind
/// [`SplineError::KnotVectorInvalid`].
#[derive(Clone, Debug, PartialEq, Eq)]
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
            KnotVectorIssue::DegreeZero => f.write_str("degree 0 is unsupported"),
            KnotVectorIssue::TooShort => f.write_str("fewer than 2(degree+1) knots"),
            KnotVectorIssue::NonFinite { index } => write!(f, "knot {index} is not finite"),
            KnotVectorIssue::Decreasing { index } => write!(f, "knot {index} decreases"),
            KnotVectorIssue::StartNotClamped => {
                f.write_str("start multiplicity is not exactly degree+1")
            }
            KnotVectorIssue::EndNotClamped => {
                f.write_str("end multiplicity is not exactly degree+1")
            }
            KnotVectorIssue::InteriorMultiplicityTooHigh { index } => {
                write!(f, "interior knot {index} has multiplicity > degree")
            }
        }
    }
}

/// A validated **clamped** knot vector with its degree — the structural
/// half of every B-spline/NURBS entity (knots and degree are `f64`/
/// `usize` structure per C6; generic scalar types never appear here).
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
}

impl KnotVector {
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
        // Exact end multiplicities. Indexing justified: len ≥ 2(p+1) ≥
        // p + 2, so p + 1 and len − p − 2 are in range.
        let len = knots.len();
        let start_run = knots.iter().take_while(|k| **k == knots[0]).count();
        if start_run != degree + 1 {
            return issue(KnotVectorIssue::StartNotClamped);
        }
        let end_run = knots
            .iter()
            .rev()
            .take_while(|k| **k == knots[len - 1])
            .count();
        if end_run != degree + 1 {
            return issue(KnotVectorIssue::EndNotClamped);
        }
        // Interior multiplicity ≤ degree: scan runs strictly between the
        // clamp runs. (The end runs are exact, so interior values differ
        // from both end values.)
        let mut run_start = degree + 1;
        while run_start < len - degree - 1 {
            let mut run_end = run_start + 1;
            // Indexing justified: run_end ≤ len − degree − 1 < len.
            while run_end < len - degree - 1 && knots[run_end] == knots[run_start] {
                run_end += 1;
            }
            if run_end - run_start > degree {
                return issue(KnotVectorIssue::InteriorMultiplicityTooHigh { index: run_start });
            }
            run_start = run_end;
        }
        Ok(Self { knots, degree })
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

    /// The parameter domain `[knots[p], knots[len − 1 − p]]`.
    pub fn domain(&self) -> (f64, f64) {
        // Indexing justified: len ≥ 2(p+1) (construction invariant).
        (
            self.knots[self.degree],
            self.knots[self.knots.len() - 1 - self.degree],
        )
    }

    /// The index of the first (nonempty) span: `degree`.
    pub fn first_span(&self) -> usize {
        self.degree
    }

    /// The index of the last (nonempty) span: `len − degree − 2`.
    pub fn last_span(&self) -> usize {
        self.knots.len() - self.degree - 2
    }

    /// Locates the span containing `t`: the index `i` with
    /// `knots[i] ≤ t < knots[i+1]` (half-open), the **last** span
    /// closed — the fixed tie-break: at an interior knot value the
    /// returned span is the one *starting* there (the last copy's
    /// span, which is nonempty by the multiplicity invariant).
    ///
    /// Total on all of `f64`: `t` below the domain returns the first
    /// span, `t` at/above the domain end returns the last span (each
    /// then evaluates the span's polynomial extension — the documented
    /// garbage-out contract of `eval_in_span`), and **NaN returns the
    /// first span deterministically** (poison then propagates through
    /// the evaluation's arithmetic as a value, never a decision).
    // The `!(t > …)` guard is deliberate: the negated form routes NaN
    // to the first span (fn docs), where `t <= …` would be false for
    // NaN and fall through into the binary search with a broken
    // invariant.
    #[allow(clippy::neg_cmp_op_on_partial_ord)]
    pub fn find_span(&self, t: f64) -> usize {
        let first = self.first_span();
        let last = self.last_span();
        // NaN and below-domain both fail this test → first span.
        if !(t > self.knots[first]) {
            return first;
        }
        // Indexing justified: last + 1 = len − degree − 1 < len.
        if t >= self.knots[last + 1] {
            return last;
        }
        // Binary search over span indices [lo, hi] maintaining
        // knots[lo] ≤ t < knots[hi + 1]; both bounds were just
        // established. Terminates: the window shrinks every step.
        let (mut lo, mut hi) = (first, last);
        while lo < hi {
            let mid = lo + (hi - lo).div_ceil(2);
            // Indexing justified: first ≤ lo < mid ≤ hi ≤ last.
            if t < self.knots[mid] {
                hi = mid - 1;
            } else {
                lo = mid;
            }
        }
        lo
    }

    /// The inclusive span range overlapped by `[lo, hi]`, each end
    /// located by [`KnotVector::find_span`]'s tie-break. `lo ≤ hi` is
    /// the caller's contract ([`crate::Bounds`] brackets satisfy it);
    /// NaN ends land on the first span per `find_span`.
    pub fn span_range(&self, lo: f64, hi: f64) -> (usize, usize) {
        (self.find_span(lo), self.find_span(hi))
    }

    /// Whether `span` is a **nonempty** span (`knots[span] <
    /// knots[span+1]`). Interior knot multiplicities create empty
    /// spans; their basis denominators are zero, so evaluation treats
    /// them as invalid (poison). [`KnotVector::find_span`] never
    /// returns one — every parameter `t`, including a repeated knot
    /// value `u` itself, is assigned to the nonempty span *starting*
    /// at it — so multi-span hull iteration skips empty spans without
    /// discarding any parameter's span.
    pub fn span_is_nonempty(&self, span: usize) -> bool {
        span + 1 < self.knots.len() && self.knots[span] < self.knots[span + 1]
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

    /// The clamped single-segment (Bézier) vector on `[0, 1]`:
    /// `degree + 1` zeros followed by `degree + 1` ones. Infallible —
    /// statically valid for every `degree ≥ 1`.
    ///
    /// **`unit_segment(0)` silently yields the degree-1 vector** (the
    /// argument is clamped up with `max(1)`, not refused): the
    /// `DegreeZero` refusal is [`KnotVector::clamped`]'s job — the one
    /// validating door for externally supplied structure — while this
    /// constructor exists precisely to be panic- and error-free for
    /// the placeholder/fixture paths, which never ask for degree 0.
    /// Callers that must *distinguish* degree 0 go through `clamped`.
    pub fn unit_segment(degree: usize) -> Self {
        let p = degree.max(1);
        let mut knots = vec![0.0; p + 1];
        knots.extend(core::iter::repeat_n(1.0, p + 1));
        Self { knots, degree: p }
    }

    /// Crate-internal constructor for knot vectors produced by the
    /// knot-algebra plans, whose outputs preserve the invariants by
    /// construction (each op inserts/removes copies of interior values
    /// within the multiplicity budget, or re-clamps ends explicitly).
    /// Debug builds re-validate.
    pub(crate) fn from_algebra(knots: Vec<f64>, degree: usize) -> Self {
        #[cfg(debug_assertions)]
        {
            match Self::clamped(knots.clone(), degree) {
                Ok(kv) => kv,
                // A plan producing invalid structure is a kernel bug;
                // still total in release (validity re-checked by any
                // subsequent `clamped` round trip).
                Err(_) => Self { knots, degree },
            }
        }
        #[cfg(not(debug_assertions))]
        {
            Self { knots, degree }
        }
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;

    fn kv(knots: &[f64], p: usize) -> KnotVector {
        KnotVector::clamped(knots.to_vec(), p).unwrap()
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

    #[test]
    fn find_span_half_open_with_closed_last_span() {
        // Spans: [0,1) → 2, [1,2) → 4 (interior double knot at 1), [2,3] → 5.
        let k = kv(&[0.0, 0.0, 0.0, 1.0, 1.0, 2.0, 3.0, 3.0, 3.0], 2);
        assert_eq!(k.first_span(), 2);
        assert_eq!(k.last_span(), 5);
        assert_eq!(k.find_span(0.0), 2);
        assert_eq!(k.find_span(0.999), 2);
        // At the interior knot: the span STARTING there (last copy).
        assert_eq!(k.find_span(1.0), 4);
        assert_eq!(k.find_span(1.5), 4);
        assert_eq!(k.find_span(2.0), 5);
        // Last span closed; above-domain and below-domain totalize.
        assert_eq!(k.find_span(3.0), 5);
        assert_eq!(k.find_span(7.5), 5);
        assert_eq!(k.find_span(-1.0), 2);
        // NaN routes to the first span, deterministically.
        assert_eq!(k.find_span(f64::NAN), 2);
        // Range form.
        assert_eq!(k.span_range(0.5, 2.5), (2, 5));
        assert_eq!(k.span_range(1.25, 1.75), (4, 4));
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
}
