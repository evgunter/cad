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

/// A span index **proven** in range and nonempty for the knot vector it
/// was drawn from, carrying the control-point window it selects.
///
/// Its fields are private and its only constructors are
/// [`KnotVector::span`] (checked) and [`KnotVector::span_at`] (total),
/// so "invalid span index" is not a representable state: evaluation
/// needs no guard and has no poison-on-bad-index path. The window is
/// computed once, at construction, so `span − degree` never appears at
/// a use site and cannot underflow there.
///
/// **Not branded to its knot vector.** A `Span` from one `KnotVector`
/// used with another of the same degree yields an in-range but wrong
/// window; from a **longer** one it can index past the shorter vector's
/// arrays entirely, and since the consumers' range guards are gone that
/// is now a panic rather than the poison D4 asks for. Every consumer
/// today draws the span from the same vector it evaluates, one
/// statement apart.
///
/// Making it a type-level fact wants one of two shapes, neither paid
/// for yet: the `Span` **holding** its vector (`Span<'a>` with a
/// `&'a KnotVector`), which lets the entry points drop their own `kv`
/// parameter so the mismatch is unrepresentable — at the cost of a
/// lifetime on `Span`, [`super::SpanSet`] and `SpanLocate` — or an
/// invariant-lifetime **brand**, which keeps the values plain but needs
/// a scoped constructor. Both are design changes, not refactors.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Span {
    index: usize,
    first_control: usize,
    degree: usize,
}

impl Span {
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
        // Interior multiplicity ≤ degree: the same run scan
        // [`KnotVector::interior_knots`] serves, one frame before the
        // type exists — which is why it goes through [`runs_in`] rather
        // than through the method. (The end runs are exact, so interior
        // values differ from both end values.) Slicing justified:
        // len ≥ 2(degree + 1) gives degree + 1 ≤ len − degree − 1, both
        // checked above.
        let mut index = degree + 1;
        for (_, mult) in runs_in(&knots[degree + 1..len - degree - 1]) {
            if mult > degree {
                return issue(KnotVectorIssue::InteriorMultiplicityTooHigh { index });
            }
            index += mult;
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
    pub fn find_span(&self, t: f64) -> usize {
        self.span_at(t).index()
    }

    /// The located span as an **offset above [`KnotVector::first_span`]**
    /// — which, since `first_span() == degree`, is exactly the first
    /// control point of the span's window. Searching in this coordinate
    /// is what lets [`KnotVector::span_at`] build a [`Span`] with no
    /// `index − degree` subtraction to underflow and no validity check
    /// to discharge: the search starts at 0 and never leaves
    /// the span count, `len − 2·degree − 2`.
    ///
    /// Semantics are [`KnotVector::find_span`]'s, unchanged: same
    /// comparisons in the same order against the same knots — it is
    /// [`span_offset_in`], the module's only span search.
    fn span_offset(&self, t: f64) -> usize {
        span_offset_in(&self.knots, self.degree, t)
    }

    /// The inclusive span range overlapped by `[lo, hi]`, each end
    /// located by [`KnotVector::span_at`]'s tie-break. `lo ≤ hi` is
    /// the caller's contract ([`crate::Bounds`] brackets satisfy it);
    /// NaN ends land on the first span per `span_at`.
    ///
    /// Both ends are [`Span`]s: locating is where span validity
    /// originates, and `span_at` is total, so there is nothing for a
    /// caller to re-check. Iterate the interior with
    /// `first.index() + 1 ..= last.index()` and [`KnotVector::span`],
    /// which refuses the empty spans in between.
    pub fn span_range(&self, lo: f64, hi: f64) -> (Span, Span) {
        (self.span_at(lo), self.span_at(hi))
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

    /// The validated [`Span`] at `index`, or `None` when the index is
    /// out of range or names an **empty** span (interior knot
    /// multiplicity). This and [`KnotVector::span_at`] are the only
    /// ways to obtain a `Span`.
    pub fn span(&self, index: usize) -> Option<Span> {
        if index < self.first_span() || index > self.last_span() || !self.span_is_nonempty(index) {
            return None;
        }
        // Justified once, here: `index >= first_span() == degree`, so
        // the subtraction cannot underflow, and `index <= last_span()`
        // puts `index + 1` inside the knot array. Every consumer of the
        // resulting `Span` inherits both facts.
        Some(Span {
            index,
            first_control: index - self.degree,
            degree: self.degree,
        })
    }

    /// [`KnotVector::find_span`] as a validated [`Span`] — total on all
    /// of `f64` for exactly the reasons `find_span` is (see its docs:
    /// out-of-domain clamps to an end span, NaN lands on the first).
    pub fn span_at(&self, t: f64) -> Span {
        // The search runs in window coordinates, so its result *is* the
        // window's first control point: there is no subtraction to
        // check and no `Option` to discharge. Nonemptiness comes from
        // the same three exits `find_span` documents — the clamped ends
        // are nonempty by the end-multiplicity invariant, and the
        // search maintains `knots[i] ≤ t < knots[i + 1]` strictly.
        let first_control = self.span_offset(t);
        let index = first_control + self.degree;
        // In-range is structural above; nonemptiness is still an
        // argument, and it is the one the basis denominators rest on.
        // Keep it a postcondition with teeth: an empty span here would
        // otherwise divide by a zero knot difference and poison
        // silently, where the `span()` route returned `None`.
        debug_assert!(
            self.span_is_nonempty(index),
            "span_at located an empty span {index}"
        );
        Span {
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
        let p = self.degree;
        // Slicing justified: len ≥ 2(degree + 1) gives
        // degree + 1 ≤ len − degree − 1, so the range is valid for
        // every knot vector — empty exactly when there is one span.
        runs_in(&self.knots[p + 1..self.knots.len() - p - 1])
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
/// [`KnotVector::find_span`] and goes through [`span_offset_in`]
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
/// list — the one span search for **clamped** vectors, shared by
/// [`KnotVector::span_at`] and by the knot-algebra paths that hold a
/// raw list mid-mutation and so have no [`KnotVector`] to ask.
///
/// It is not the tree's only span search, and the other one is not a
/// duplicate: `geom-brep`'s `props::quad::raw_span` locates spans in
/// knot lists a `KnotVector` **cannot represent** — a derivative
/// direction whose interior multiplicity exceeds its own degree, which
/// [`KnotVector::clamped`] refuses. The preconditions below do not hold
/// there, and the answers genuinely differ: this search maintains a
/// bracket that may name an empty span, where `raw_span` skips empty
/// spans by construction and clamps into a coefficient-count-derived
/// range. Two searches, two domains, one of them outside this type.
///
/// **Preconditions, and what a violation costs.** Taking a slice rather
/// than `&self` moves two facts from *guaranteed by the type* to
/// *required of the caller*, and they are the facts the indexing rests
/// on: `knots.len() ≥ 2·degree + 2` (a shorter slice underflows
/// `last`), and `knots` non-decreasing (otherwise the search's
/// maintained bracket is meaningless and the answer is arbitrary — in
/// range, but wrong). Both are strictly weaker than [`KnotVector`]'s
/// construction invariant, so a `KnotVector`'s own knots always satisfy
/// them; the raw knot-algebra paths satisfy them because they start
/// from a `KnotVector`'s knots and only insert interior values.
///
/// **A violation of the first one panics**, and it is worth naming what
/// kind: `knots.len() - 2·degree - 2` is `usize` arithmetic, so a short
/// slice underflows — a debug panic, and in release (the workspace sets
/// no `overflow-checks`) it wraps to a huge span count and the very
/// next index runs off the end. That is neither poison nor a typed
/// refusal, which is the concrete price of holding by convention what
/// the type was holding by construction. It is why the two doors below
/// are crate-internal rather than a matter of documentation. This is
/// not a public door — [`find_span_in`] is `pub(crate)` and
/// `span_offset_in` is private — so the obligation cannot escape the
/// crate. Widening either to `pub` is what would change that, and would
/// want the `Span`-holds-its-vector shape [`Span`] already documents.
///
/// Total on all of `f64` with [`KnotVector::find_span`]'s three
/// documented behaviours — below-domain and NaN give the first span, at
/// or above the domain end gives the last — because it *is* that
/// function's body.
// The `!(t > …)` guard is deliberate: the negated form routes NaN to
// the first span, where `t <= …` would be false for NaN and fall
// through into the binary search with a broken invariant.
#[allow(clippy::neg_cmp_op_on_partial_ord)]
fn span_offset_in(knots: &[f64], degree: usize, t: f64) -> usize {
    // `last` is the span count, len − 2·degree − 2: non-negative by
    // the construction invariant len ≥ 2(degree + 1).
    let (p, last) = (degree, knots.len() - 2 * degree - 2);
    // NaN and below-domain both fail this test → first span.
    if !(t > knots[p]) {
        return 0;
    }
    // Indexing justified: p + last + 1 = len − degree − 1 < len.
    if t >= knots[p + last + 1] {
        return last;
    }
    // Binary search over span offsets [lo, hi] maintaining
    // knots[p + lo] ≤ t < knots[p + hi + 1]; both bounds were just
    // established. Terminates: the window shrinks every step.
    let (mut lo, mut hi) = (0, last);
    while lo < hi {
        let mid = lo + (hi - lo).div_ceil(2);
        // Indexing justified: 0 ≤ lo < mid ≤ hi ≤ last.
        if t < knots[p + mid] {
            hi = mid - 1;
        } else {
            lo = mid;
        }
    }
    lo
}

/// [`KnotVector::find_span`] against a raw clamped knot list: the span
/// index rather than the offset, same tie-break (at an interior knot
/// value, the span *starting* there), same totality.
///
/// **This is not "the last index `i` with `knots[i] ≤ t`".** The two
/// coincide on `t ∈ [knots[degree], knots[len − degree − 1])` and
/// nowhere else: at or above the domain end this returns the last span
/// while that scan walks on into the trailing clamp, and below the
/// domain — or at NaN — this returns the first span while that scan
/// returns whatever it was initialised with. Substituting this for such
/// a scan is sound only under that half-open precondition, which is the
/// substituting frame's to state.
pub(crate) fn find_span_in(knots: &[f64], degree: usize, t: f64) -> usize {
    span_offset_in(knots, degree, t) + degree
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use test_utils::vacuity::Exposure;

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
        // Range form — validated ends, compared as the indices they name.
        let range = |lo, hi| {
            let (a, b): (Span, Span) = k.span_range(lo, hi);
            (a.index(), b.index())
        };
        assert_eq!(range(0.5, 2.5), (2, 5));
        assert_eq!(range(1.25, 1.75), (4, 4));
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
    /// [`find_span_in`] and [`KnotVector::find_span`] are one
    /// expression: both reduce to `span_offset_in(knots, degree, t) +
    /// degree`, so no probe can separate them and an assertion that
    /// they agree is satisfied by construction. What CAN go red is the
    /// search itself, so that is what this row drives — a linear scan
    /// written from the documented contract, independent of the binary
    /// search it checks:
    ///
    /// - below the domain, and at NaN, the **first** span;
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
        // the span indices, not offsets. The negated comparison is the
        // NaN route, exactly as in `span_offset_in`.
        #[allow(clippy::neg_cmp_op_on_partial_ord)]
        fn oracle(knots: &[f64], degree: usize, t: f64) -> usize {
            let (first, last) = (degree, knots.len() - degree - 2);
            if !(t > knots[first]) {
                return first;
            }
            if t >= knots[last + 1] {
                return last;
            }
            let mut got = first;
            for i in first..=last {
                if knots[i] <= t && t < knots[i + 1] {
                    got = i;
                }
            }
            got
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
                let got = find_span_in(&knots, p, t);
                assert_eq!(
                    got,
                    oracle(&knots, p, t),
                    "p{p} at {t}: the search left its documented contract"
                );
                // Equal BY CONSTRUCTION today — both doors are one
                // expression. Kept as one line so a future edit that
                // gives them separate bodies reds here; it is not this
                // row's evidence, which is the oracle above.
                assert_eq!(got, k.find_span(t), "p{p} at {t}: the two doors diverged");
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
}
