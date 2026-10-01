//! **Ring-coefficient composites** `f ∘ C` (M5 PR 4): the reusable
//! promotion of the PR 2 rehearsal (`hull_circle_rehearsal.rs`) from
//! test-local code to certification substrate. Given a NURBS curve's
//! structure and ring-lifted control data, build the ring-coefficient
//! Bernstein form of polynomial composites — products of coordinate
//! splines, linear functionals, and the quadric-family implicit
//! composites (plane, sphere, cylinder, cone, torus — the torus is
//! degree 4 in the point, still polynomial) — and read **certified
//! sup-norm bounds** off their coefficient hulls. Data in, bounds out:
//! nothing here evaluates or samples anything (C2.2, OQ2: hull bounds
//! are the entry requirement for fitted-cache certification).
//!
//! # The pipeline (the rehearsal's, generalized)
//!
//! 1. Lift the homogeneous channels in certification arithmetic: per coordinate `d`,
//!    `G_d,i = w_i·(x_d,i − c_d)` (center-shifted **before** any
//!    product — forming `|P|² − 2P·c + …` instead would cancel
//!    catastrophically in the coefficients, and an interval bound
//!    reports that cancellation as width rather than hiding it), plus
//!    the weight channel `W_i = w_i`.
//! 2. Bézier-decompose each channel: knot insertion to full interior
//!    multiplicity, **structure** (positions, counts) read from the
//!    `f64` knot vector, **coefficients** combined in certification arithmetic in the
//!    convex form `c_{i−1}·β + c_i·α`, with BOTH barycentric ratios
//!    formed as ring quotients of knot enclosures (an `f64`-rounded
//!    ratio would silently drop its rounding error, and the lerp form
//!    `c_{i−1} + (c_i − c_{i−1})·α` would read `c_{i−1}` twice and
//!    multiply its dust up once per insertion), met with the hull of
//!    `c_{i−1}` and `c_i` so a constant channel stays exact.
//! 3. Per span, exact Bernstein products: degree `da × db → da + db`
//!    with the binomial weights `C(da,i)·C(db,j)/C(da+db,k)` computed
//!    as ring quotients (several are not `f64`-representable).
//! 4. Bounds: per-span coefficient hulls of numerator and denominator,
//!    the quotient per span (interval arithmetic refuses a zero-touching divisor,
//!    so a degenerate denominator is refused loudly), hulled across spans.
//!
//! # Scaling conventions (what the bound means)
//!
//! With unit `normal`/`axis` data the composites are the standard
//! signed residuals: meters for [`ImplicitSurface::Plane`], meters² for
//! sphere/cylinder/cone, meters⁴ for the torus. Axis normalization for
//! the quadratic terms is **exact in certification arithmetic** — `(Q·a)²` enters as
//! `T²/|a|²` with `|a|²` a certification enclosure — so a non-unit axis changes
//! nothing for those terms; the plane's linear normalization would need
//! a square root, which certification arithmetic deliberately does not
//! take (C9), so the plane
//! composite is `n·(P − p₀)` as given (meters only for unit `n`).
//!
//! # C6 and the refusal posture
//!
//! Structure (knots, weights, degrees, binomials) is `f64`; everything
//! coefficient-valued is [`Interval`]. Checkable structural errors
//! at the entry points are typed refusals; anything downstream (zero
//! axis, degenerate weights) refuses the bound, which fails every
//! `≤ ε` comparison (D4 ¶2).

use super::algebra::convex_step;
use super::knots::{InteriorKnot, KnotVector, SplineError, find_span_in};
use crate::interval::Interval;
use crate::interval::certification::Certification;
use crate::readable::Readable;
use std::borrow::Cow;

pub mod patch;
pub mod tensor;

/// A typed compose refusal (entry-point structure validation).
#[derive(Clone, Debug, PartialEq)]
pub enum ComposeError {
    /// Weight/coordinate counts or weight positivity fail basic spline
    /// structure validation.
    Structure(SplineError),
    /// The curve data has the wrong number of coordinate channels for
    /// the requested composite (implicit surfaces need 3).
    DimensionMismatch {
        /// Channels supplied.
        dims: usize,
        /// Channels required.
        expected: usize,
    },
    /// A coordinate channel index is out of range.
    ChannelOutOfRange {
        /// The offending channel index.
        channel: usize,
        /// The channel count.
        dims: usize,
    },
    /// A curve pair fed to a shared-parameter composite lives on two
    /// different knot domains, so "the same `t`" is not a statement
    /// (the OQ4 identity is the entry requirement, exact in `f64`).
    DomainMismatch {
        /// The first curve's `(lo, hi)` domain.
        a: (f64, f64),
        /// The second curve's `(lo, hi)` domain.
        b: (f64, f64),
    },
}

impl core::fmt::Display for ComposeError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            ComposeError::Structure(e) => write!(f, "compose: {e}"),
            ComposeError::DimensionMismatch { dims, expected } => {
                write!(f, "compose: {dims} coordinate channels, need {expected}")
            }
            ComposeError::ChannelOutOfRange { channel, dims } => {
                write!(
                    f,
                    "compose: channel {channel} out of range ({dims} channels)"
                )
            }
            ComposeError::DomainMismatch { a, b } => {
                write!(
                    f,
                    "compose: a shared-parameter composite needs one knot domain, \
                     got [{}, {}] and [{}, {}] — refit the pair on one \
                     parameterization (the shared-parameter identity) before composing",
                    Readable(a.0),
                    Readable(a.1),
                    Readable(b.0),
                    Readable(b.1)
                )
            }
        }
    }
}

impl core::error::Error for ComposeError {}

/// A NURBS curve in certification form: its structure plus its control
/// coordinates — the data-in shape every composite consumes. `coords[d][i]`
/// is the `d`-th coordinate of control point `i` as a certification enclosure (a plain
/// `f64` control point lifts via [`Interval::point`]; a perturbed
/// or interval-valued one via [`Interval::from_bounds`]).
#[derive(Clone, Debug)]
pub struct CurveCertData<'a> {
    kv: &'a KnotVector,
    weights: &'a [f64],
    coords: &'a [Vec<Interval>],
}

// `!(w > 0)` is deliberate (NaN-catching): see `algebra::check_weights`.
#[allow(clippy::neg_cmp_op_on_partial_ord)]
impl<'a> CurveCertData<'a> {
    /// Validated construction: weight count/positivity/finiteness and
    /// per-channel coordinate counts against the knot vector. The
    /// channel count (2-D pcurves, 3-D curves, …) is free; composites
    /// that need a specific dimension check it themselves.
    ///
    /// # Errors
    ///
    /// [`ComposeError::Structure`] naming the exact violation.
    pub fn new(
        kv: &'a KnotVector,
        weights: &'a [f64],
        coords: &'a [Vec<Interval>],
    ) -> Result<Self, ComposeError> {
        let n = kv.control_count();
        if weights.len() != n {
            return Err(ComposeError::Structure(SplineError::WeightCountMismatch {
                weights: weights.len(),
                control: n,
            }));
        }
        for (index, w) in weights.iter().enumerate() {
            if !(*w > 0.0) {
                return Err(ComposeError::Structure(SplineError::NonPositiveWeight {
                    index,
                    weight: *w,
                }));
            }
            if !w.is_finite() {
                return Err(ComposeError::Structure(SplineError::NonFiniteWeight {
                    index,
                    weight: *w,
                }));
            }
        }
        for ch in coords {
            if ch.len() != n {
                return Err(ComposeError::Structure(SplineError::ControlCountMismatch {
                    control: ch.len(),
                    expected: n,
                }));
            }
        }
        Ok(Self {
            kv,
            weights,
            coords,
        })
    }

    /// The number of coordinate channels.
    pub fn dims(&self) -> usize {
        self.coords.len()
    }

    /// The weight channel `W_i = w_i`, Bézier-decomposed.
    fn weight_channel(&self) -> BernsteinSpans {
        let coeffs: Vec<Interval> = self.weights.iter().map(|w| Interval::point(*w)).collect();
        to_bezier_spans(self.kv, &coeffs)
    }

    /// The unshifted weighted channel `w_i·x_d,i`, Bézier-decomposed.
    /// Caller guarantees `d < dims()`.
    fn weighted_channel(&self, d: usize) -> BernsteinSpans {
        let coeffs: Vec<Interval> = self.coords[d]
            .iter()
            .zip(self.weights.iter())
            .map(|(x, w)| Interval::point(*w) * *x)
            .collect();
        to_bezier_spans(self.kv, &coeffs)
    }

    /// The shifted weighted channel `w_i·(x_d,i − shift)` (module docs
    /// step 1: shift before any product), Bézier-decomposed. Caller
    /// guarantees `d < dims()`.
    fn shifted_channel(&self, d: usize, shift: f64) -> BernsteinSpans {
        let s = Interval::point(shift);
        let coeffs: Vec<Interval> = self.coords[d]
            .iter()
            .zip(self.weights.iter())
            .map(|(x, w)| Interval::point(*w) * (*x - s))
            .collect();
        to_bezier_spans(self.kv, &coeffs)
    }
}

/// The per-span Bernstein form of one scalar channel: `spans[j]` holds
/// the `degree + 1` ring coefficients of the polynomial on
/// `[breaks[j], breaks[j+1]]`.
#[derive(Clone, Debug)]
pub struct BernsteinSpans {
    degree: usize,
    breaks: Vec<f64>,
    spans: Vec<Vec<Interval>>,
}

impl BernsteinSpans {
    /// The Bernstein degree shared by every span.
    pub fn degree(&self) -> usize {
        self.degree
    }

    /// The break parameters: span `j` covers `[breaks[j], breaks[j+1]]`.
    pub fn breaks(&self) -> &[f64] {
        &self.breaks
    }

    /// The per-span coefficient rows.
    pub fn spans(&self) -> &[Vec<Interval>] {
        &self.spans
    }

    /// Per-span coefficient hulls (the convexity fact of
    /// [`super::hull`]: each is a certified enclosure of the channel's
    /// values on that span). Fixed ascending fold order (D9).
    pub fn span_hulls(&self) -> Vec<Interval> {
        self.spans
            .iter()
            .map(|row| {
                let mut acc = Interval::refused();
                for (n, c) in row.iter().enumerate() {
                    acc = if n == 0 { *c } else { Interval::hull(acc, *c) };
                }
                acc
            })
            .collect()
    }
}

// ---------------------------------------------------------------------
// Bézier decomposition (module docs step 2)
// ---------------------------------------------------------------------

/// One Boehm insertion of `u` into the raw knot list `knots` (degree
/// `p`, current multiplicity `s` of `u`), coefficients combined in the
/// ring in the **convex form** `Q_i = c_{i−1}·β_i + c_i·α_i`, with
/// `Δ_i = U_{i+p} − U_i` and both barycentric coefficients formed as
/// **ring quotients** of knot enclosures from the knots they are made
/// of (module docs step 2):
///
/// ```text
/// α_i = (u − U_i)/Δ_i        β_i = (U_{i+p} − u)/Δ_i
/// ```
///
/// Fixed ascending index order.
///
/// **The argument for this form — why `β` comes from the knots and not
/// from `1 − α`, why the combination still encloses the true refined
/// coefficient, why reading each coefficient once is the whole width
/// saving, and why the step is met with the hull of its two sources —
/// has one home in this crate and it is
/// [`super::algebra::CurvePlan::apply_certified`]'s docs.** Both
/// combine through the one `algebra::convex_step`. What belongs here
/// is only what is local:
///
/// - `Δ_i > 0` because `U_i < u` (`i ≤ k − s`, below the copy run) and
///   `U_{i+p} ≥ U_{k+1} > u` (span `k` is nonempty), so neither
///   quotient refuses.
/// - Containment through a WHOLE fold of these steps, against exact
///   rational arithmetic, is
///   this module's `the_ring_fold_encloses_the_exact_refined_net`; the
///   width the fold accumulates is
///   `the_convex_form_does_not_inflate_the_fold`.
///   (Both are `#[cfg(test)]`, so these are names and not links —
///   rustdoc does not document a test module.)
/// - [`to_bezier_spans_extra`] inserts each interior knot to full
///   multiplicity, so the fold here is `p − m` deep for a knot of
///   existing multiplicity `m` (`for step in m..p`) — `p` deep for a
///   fresh break, one step for a knot already at `p − 1`. That is the
///   depth the lerp form multiplied a coefficient's dust up over.
///
/// **The Boehm structure is shared with `algebra::insert_once` (private
/// there, so this is a name and not a link), and so is the coefficient
/// arithmetic.** Both derive the span through the same search
/// ([`super::knots::find_span_in`] here, `find_span` there), both
/// insert one knot at `k + 1`, and both combine in the convex form.
/// What separates them is the SHAPE of the schedule: this one folds
/// interval coefficients in place over a RAW knot list, to full
/// interior multiplicity, deliberately never rebuilding a
/// [`super::KnotVector`] per step, where `insert_once` builds a
/// replayable `CurvePlan` of `Step`s and rebuilds a vector per
/// insertion. There are also no weights at all here — one homogeneous
/// channel — where `insert_once` forms the projective applier's `λ`
/// from them. Unifying the two is filed, not argued away
/// (`insert_once`'s docs name the row).
fn insert_once_ring(
    knots: &mut Vec<f64>,
    p: usize,
    s: usize,
    coeffs: &mut Vec<Interval>,
    u: InteriorKnot,
) {
    // Strictly interior by type plus privacy (`InteriorKnot`'s docs):
    // the span search below is total and would answer an END span for
    // an outside value, which is why it may only run on a proven one.
    let u = u.value();
    // Span k: knots[k] ≤ u < knots[k+1], the last copy's span, so
    // 0 < k < len − 1.
    let k = find_span_in(knots, p, u);
    let n_old = coeffs.len();
    let mut out = Vec::with_capacity(n_old + 1);
    for i in 0..=n_old {
        if i + p <= k {
            // Q_i = c_i (carry below the affected window).
            out.push(coeffs[i]);
        } else if i + s <= k {
            // Window k−p+1 ..= k−s: interval arithmetic combination.
            out.push(convex_step(
                coeffs[i - 1],
                coeffs[i],
                knots[i],
                knots[i + p],
                u,
            ));
        } else {
            // Q_i = c_{i−1} (carry above the window; i ≥ 1 here because
            // k ≥ s for an interior u with multiplicity s).
            out.push(coeffs[i - 1]);
        }
    }
    knots.insert(k + 1, u);
    *coeffs = out;
}

/// Bézier-decomposes one scalar channel: knot insertion to full
/// interior multiplicity (structure from `kv`, coefficients in the
/// ring), then the per-span coefficient rows read off by chunks.
fn to_bezier_spans(kv: &KnotVector, coeffs: &[Interval]) -> BernsteinSpans {
    to_bezier_spans_extra(kv, coeffs, &[])
}

/// [`to_bezier_spans`] with **extra break parameters** injected: each
/// `extra` value strictly inside the domain and not already a knot is
/// inserted to full multiplicity, so two channels decomposed with each
/// other's knots as extras land on one shared break list (the tensor
/// composite's alignment substrate, and knot insertion is exact in ℝ —
/// the represented function is unchanged). Values outside the open
/// domain or duplicating a knot are structure-filtered, not errors.
fn to_bezier_spans_extra(kv: &KnotVector, coeffs: &[Interval], extra: &[f64]) -> BernsteinSpans {
    let p = kv.degree();
    let mut knots = kv.knots().to_vec();
    let mut c = coeffs.to_vec();
    let (lo, hi) = kv.domain();
    // Merge the existing interior values (multiplicity from the vector)
    // with the fresh extras (multiplicity 0), ascending, exact-`f64`
    // dedup — pure structure (C6's f64 lane). An extra becomes an
    // insertion point only through `interior_knot`, the one filter to
    // the open domain.
    let mut interior: Vec<(InteriorKnot, usize)> = kv.interior_knot_runs().collect();
    for &v in extra {
        if let Some(k) = kv.interior_knot(v)
            && !interior.iter().any(|(w, _)| w.value() == v)
        {
            interior.push((k, 0));
        }
    }
    interior.sort_by(|a, b| a.0.value().total_cmp(&b.0.value()));
    // `knots` is a valid clamped vector for `p` at every step, which is
    // what lets the raw span search be called on it: each insertion
    // places one copy of a strictly interior `v` inside the existing
    // order (so still non-decreasing, and both clamp runs untouched),
    // and `step` stops at `p`, so no interior multiplicity exceeds the
    // degree. Rebuilding a `KnotVector` per step would re-establish
    // that by validation, at a clone and an O(n) re-check each — the
    // reason this path is raw at all.
    for (v, m) in &interior {
        for step in *m..p {
            insert_once_ring(&mut knots, p, step, &mut c, *v);
        }
    }
    // Breaks: domain ends plus the distinct interior values (ascending).
    let mut breaks = Vec::with_capacity(interior.len() + 2);
    breaks.push(lo);
    breaks.extend(interior.iter().map(|(v, _)| v.value()));
    breaks.push(hi);
    // Full multiplicity everywhere: control count = nseg·p + 1; span j
    // owns coefficients j·p ..= j·p + p.
    let nseg = breaks.len() - 1;
    let spans = (0..nseg).map(|j| c[j * p..=j * p + p].to_vec()).collect();
    BernsteinSpans {
        degree: p,
        breaks,
        spans,
    }
}

// ---------------------------------------------------------------------
// Bernstein algebra (module docs step 3) — fixed association orders
// ---------------------------------------------------------------------

/// The largest row degree [`binom_row`] serves exactly. **Exactness is
/// the recurrence's, not the values'**: at `n = 55` the intermediate
/// product `C(55, 25)·31` exceeds 2⁵³ and rounds even though
/// `C(55, 26)` itself is representable, so the honest cap is `n ≤ 54`
/// (differentially pinned against `u128` arithmetic in the tests).
const BINOM_EXACT_MAX: usize = 54;

/// Binomial row `C(n, 0..=n)` at `f64` — exact for `n ≤`
/// [`BINOM_EXACT_MAX`], which covers every degree this module can
/// produce from kernel-sized splines (composite degrees are ≤ 4·p).
/// Beyond the cap the row is **all-NaN**, which refuses every
/// composite coefficient built from it and fails every `≤ ε`
/// certification loudly (D4 ¶2) — a rounded weight would instead be a
/// silently unsound enclosure. (Forming the weights as ring quotients
/// was considered and rejected: interval arithmetic's products widen outward
/// unconditionally, which would break the rehearsal's ratified
/// bit-identity pin for the exact small-degree cases.)
fn binom_row(n: usize) -> Vec<f64> {
    binom_table()
        .get(n)
        .cloned()
        .unwrap_or_else(|| vec![f64::NAN; n + 1])
}

/// The binomial rows `0 ..= BINOM_EXACT_MAX`, built once. A memo, not
/// a second spelling: the rows are exactly the recurrence
/// [`binom_row`] used to run inline, bit for bit. It exists because
/// the tensor composites ask for the same few rows hundreds of
/// thousands of times per bound.
fn binom_table() -> &'static [Vec<f64>] {
    static TABLE: std::sync::OnceLock<Vec<Vec<f64>>> = std::sync::OnceLock::new();
    TABLE.get_or_init(|| {
        (0..=BINOM_EXACT_MAX)
            .map(|n| {
                let mut row = vec![1.0f64; n + 1];
                for k in 1..=n {
                    // The association is the original's, exactly:
                    // `(row[k−1] · (n−k+1)) / k`, NOT
                    // `row[k−1] · ((n−k+1)/k)`. The two differ in
                    // rounding, and these rows feed bounds with
                    // bit-identity pins.
                    row[k] = row[k - 1] * ((n - k + 1) as f64) / (k as f64);
                }
                row
            })
            .collect()
    })
}

/// Bernstein product of two coefficient rows (degrees from lengths):
/// `(ab)_k = Σ_{i+j=k} [C(da,i)·C(db,j)/C(da+db,k)] · a_i·b_j`, the
/// binomial weight formed as a ring quotient. Fixed association (D9):
/// ascending `k`, ascending `i`, terms exactly as written
/// (`acc = acc + a_i·b_j·w`). The `f64` numerator product is exact
/// whenever the output row is served at all: by Vandermonde,
/// `C(da,i)·C(db,j) ≤ C(da+db, i+j)`, and `da + db >`
/// [`BINOM_EXACT_MAX`] is already refused through `binom_row`.
fn bern_mul_row(a: &[Interval], b: &[Interval]) -> Vec<Interval> {
    let w = bern_weights(a.len() - 1, b.len() - 1);
    bern_mul_row_with(a, b, &w)
}

/// The Bernstein product's weight rows for ONE degree pair, carrying
/// that pair: `row(k)[i − lo(k)] = C(da,i)·C(db,k−i)/C(da+db,k)` as
/// interval arithmetic quotient, over the `i` the convolution actually visits
/// (`lo(k) = k − db` clamped at zero, up to `min(k, da)`), in the
/// ascending order [`bern_mul_row_into`] folds them in.
///
/// **The degrees are part of the value because the shape is not.** A
/// table built for `(db, da)` has exactly the same row widths as one
/// built for `(da, db)`, so a table handed to the wrong operands
/// indexes perfectly and answers with the wrong numbers — the failure
/// no index check can catch. Carrying `(da, db)` with the rows is what
/// lets [`bern_mul_row_into`] check the table against the operands it
/// folds it over, and gives the `lo(k)` convention ONE home rather
/// than one spelling per reader: the builder, the fold, the tensor
/// product's u-lookup and the tests all read it here.
#[derive(Clone)]
struct BernWeights {
    da: usize,
    db: usize,
    rows: Vec<Vec<Interval>>,
}

impl BernWeights {
    /// The first `i` that row `k` holds: `k − db`, clamped at zero.
    fn lo(&self, k: usize) -> usize {
        k.saturating_sub(self.db)
    }

    /// Row `k`, over `i ∈ lo(k) ..= min(k, da)`, ascending.
    fn row(&self, k: usize) -> &[Interval] {
        &self.rows[k]
    }

    /// The weight at `(k, i)`, `i` being an operand index rather than
    /// an offset into the row.
    fn at(&self, k: usize, i: usize) -> Interval {
        self.rows[k][i - self.lo(k)]
    }

    /// The number of output coefficients the table serves,
    /// `da + db + 1`.
    fn row_count(&self) -> usize {
        self.rows.len()
    }
}

/// The [`BernWeights`] of one degree pair, from the binomial rows.
///
/// The weight depends on the two DEGREES alone — pure structure — so
/// there is exactly one value per `(da, db, k, i)` for the life of the
/// process, while the loop that reads it runs once per coefficient
/// pair of every span of every product. This is a memo, not a second
/// spelling: each entry is the same two `f64` binomials multiplied and
/// the same ring division, in the same order, bit for bit. Giving the
/// table a type moves no arithmetic — `lo(k)` is the same
/// `k.saturating_sub(db)` the loop spelled, read from one place now.
fn build_bern_weights(da: usize, db: usize) -> BernWeights {
    let bin_a = binom_row(da);
    let bin_b = binom_row(db);
    let bin_ab = binom_row(da + db);
    let mut out = BernWeights {
        da,
        db,
        rows: Vec::with_capacity(da + db + 1),
    };
    for (k, bk) in bin_ab.iter().enumerate() {
        // `point(*bk)` is a pure constructor of the same divisor the
        // inline loop rebuilt per term; the quotients below are that
        // loop's, term for term.
        let den = Interval::point(*bk);
        let row = (out.lo(k)..=k.min(da))
            .map(|i| Interval::point(bin_a[i] * bin_b[k - i]) / den)
            .collect();
        out.rows.push(row);
    }
    out
}

/// [`build_bern_weights`], memoized on the degree pair: the table is
/// built on first ask and shared thereafter.
///
/// **A refused table is not what the grid excludes.** Every pair with
/// `da + db >` [`BINOM_EXACT_MAX`] has an all-refused table, and plenty
/// of those pairs are INSIDE the grid — `(27, 27)` is a slotted pair
/// whose every entry is `NaN`. That is the correct answer for the pair
/// and it is memoized like any other. What is served fresh rather than
/// given a slot is a pair with a SINGLE degree past the cap, and that
/// is a bound on the grid rather than a statement about the values: no
/// kernel-sized spline reaches such a degree (composite degrees are
/// ≤ 4·p), and keeping it out keeps the grid square and bounded
/// without a second builder.
///
/// **Retained memory**, all of it for the life of the process and
/// never freed: `(BINOM_EXACT_MAX + 1)² = 55 × 55` `OnceLock` slots,
/// plus, for each pair actually asked for, that pair's table —
/// `da + db + 1` rows holding `(da + 1)·(db + 1)` certification values in
/// total. A structural memo of a pure function, like [`binom_table`]
/// one level down.
fn bern_weights(da: usize, db: usize) -> Cow<'static, BernWeights> {
    const N: usize = BINOM_EXACT_MAX + 1;
    if da >= N || db >= N {
        return Cow::Owned(build_bern_weights(da, db));
    }
    static GRID: std::sync::OnceLock<Vec<std::sync::OnceLock<BernWeights>>> =
        std::sync::OnceLock::new();
    let grid = GRID.get_or_init(|| (0..N * N).map(|_| std::sync::OnceLock::new()).collect());
    Cow::Borrowed(grid[da * N + db].get_or_init(|| build_bern_weights(da, db)))
}

/// [`bern_mul_row`] over a weight table the caller already holds —
/// the entry point for the tensor product, where one degree pair
/// serves every cell of a whole patch.
fn bern_mul_row_with(a: &[Interval], b: &[Interval], w: &BernWeights) -> Vec<Interval> {
    let mut out = Vec::with_capacity(w.row_count());
    bern_mul_row_into(a, b, w, &mut out);
    out
}

/// [`bern_mul_row_with`] into a caller-owned buffer (overwritten), so
/// a loop over cells allocates one output row rather than one per
/// coefficient block.
///
/// The table must be the one built for these two operands. A table for
/// the transposed pair has the same row widths, so the mismatch is a
/// wrong answer rather than an out-of-range index; the pair travels
/// with the rows so that the check below can exist at all, and a
/// hoisted lookup handed to the wrong direction announces itself here
/// (D2 addendum row 5).
fn bern_mul_row_into(a: &[Interval], b: &[Interval], w: &BernWeights, out: &mut Vec<Interval>) {
    debug_assert!(
        w.da == a.len() - 1 && w.db == b.len() - 1,
        "weight table for degrees ({}, {}) folded over operands of degree ({}, {})",
        w.da,
        w.db,
        a.len() - 1,
        b.len() - 1
    );
    out.clear();
    out.reserve(w.row_count());
    for k in 0..w.row_count() {
        let lo = w.lo(k);
        let mut acc = Interval::zero();
        for (t, wt) in w.row(k).iter().enumerate() {
            let i = lo + t;
            acc = acc + a[i] * b[k - i] * *wt;
        }
        out.push(acc);
    }
}

/// A structurally refused channel: the shape mismatch outcome for the
/// span-wise combinators (never produced by the entry-point builders,
/// which share one decomposition structure; total anyway, D4).
fn refused_like(a: &BernsteinSpans, degree: usize) -> BernsteinSpans {
    BernsteinSpans {
        degree,
        breaks: a.breaks.clone(),
        spans: a
            .spans
            .iter()
            .map(|_| vec![Interval::refused(); degree + 1])
            .collect(),
    }
}

/// Span-wise sum (equal degrees and span counts required; mismatch
/// is refused). Ascending span, ascending coefficient (D9).
fn ch_add(a: &BernsteinSpans, b: &BernsteinSpans) -> BernsteinSpans {
    if a.degree != b.degree || a.spans.len() != b.spans.len() {
        return refused_like(a, a.degree);
    }
    BernsteinSpans {
        degree: a.degree,
        breaks: a.breaks.clone(),
        spans: a
            .spans
            .iter()
            .zip(b.spans.iter())
            .map(|(ra, rb)| ra.iter().zip(rb.iter()).map(|(x, y)| *x + *y).collect())
            .collect(),
    }
}

/// Span-wise difference (contract as [`ch_add`]).
fn ch_sub(a: &BernsteinSpans, b: &BernsteinSpans) -> BernsteinSpans {
    if a.degree != b.degree || a.spans.len() != b.spans.len() {
        return refused_like(a, a.degree);
    }
    BernsteinSpans {
        degree: a.degree,
        breaks: a.breaks.clone(),
        spans: a
            .spans
            .iter()
            .zip(b.spans.iter())
            .map(|(ra, rb)| ra.iter().zip(rb.iter()).map(|(x, y)| *x - *y).collect())
            .collect(),
    }
}

/// Span-wise Bernstein product (degrees add; span counts must match).
fn ch_mul(a: &BernsteinSpans, b: &BernsteinSpans) -> BernsteinSpans {
    let degree = a.degree + b.degree;
    if a.spans.len() != b.spans.len() {
        return refused_like(a, degree);
    }
    BernsteinSpans {
        degree,
        breaks: a.breaks.clone(),
        spans: a
            .spans
            .iter()
            .zip(b.spans.iter())
            .map(|(ra, rb)| bern_mul_row(ra, rb))
            .collect(),
    }
}

/// Span-wise left scale `c·x` (constant on the left — the rehearsal's
/// `r²·W²` association, kept verbatim).
fn ch_scale_left(c: Interval, a: &BernsteinSpans) -> BernsteinSpans {
    BernsteinSpans {
        degree: a.degree,
        breaks: a.breaks.clone(),
        spans: a
            .spans
            .iter()
            .map(|row| row.iter().map(|x| c * *x).collect())
            .collect(),
    }
}

/// Span-wise right scale `x·c` (constant on the right — the rehearsal's
/// `g_d,i·n_d` association, kept verbatim).
fn ch_scale_right(a: &BernsteinSpans, c: Interval) -> BernsteinSpans {
    BernsteinSpans {
        degree: a.degree,
        breaks: a.breaks.clone(),
        spans: a
            .spans
            .iter()
            .map(|row| row.iter().map(|x| *x * c).collect())
            .collect(),
    }
}

/// An all-zero channel shaped like `a` at the given degree — the
/// accumulator seed (`0 + first term` is the rehearsal's association,
/// kept for bit-identity).
fn ch_zero_like(a: &BernsteinSpans, degree: usize) -> BernsteinSpans {
    BernsteinSpans {
        degree,
        breaks: a.breaks.clone(),
        spans: a
            .spans
            .iter()
            .map(|_| vec![Interval::zero(); degree + 1])
            .collect(),
    }
}

// ---------------------------------------------------------------------
// Composites (module docs steps 1 + 3) and their bounds (step 4)
// ---------------------------------------------------------------------

/// A composite residual in rational Bernstein form: `num/den` span by
/// span, both in ring coefficients on shared breaks. Public data —
/// consumers may hull it, subdivide it, or feed the numerator to
/// further algebra; [`CompositeForm::span_bounds`] is the standard
/// bounds-out reading.
#[derive(Clone, Debug)]
pub struct CompositeForm {
    /// The numerator channel.
    pub num: BernsteinSpans,
    /// The denominator channel (a power of the weight function, times
    /// an exact axis-normalization constant where the surface has an
    /// axis — strictly positive for valid data).
    pub den: BernsteinSpans,
}

impl CompositeForm {
    /// Per-span certified enclosures of the composite's values: the
    /// numerator hull divided by the denominator hull, span by span.
    /// Interval arithmetic refuses a zero-touching divisor, so a degenerate
    /// denominator yields a refused (NaN-bracket) entry — fail-loud.
    pub fn span_bounds(&self) -> Vec<Interval> {
        self.num
            .span_hulls()
            .into_iter()
            .zip(self.den.span_hulls())
            .map(|(n, d)| n / d)
            .collect()
    }

    /// The whole-domain enclosure: the hull of [`Self::span_bounds`]
    /// (fixed ascending fold, D9). Refused if any span is refused.
    pub fn bound(&self) -> Interval {
        let mut acc = Interval::refused();
        for (n, b) in self.span_bounds().into_iter().enumerate() {
            acc = if n == 0 { b } else { Interval::hull(acc, b) };
        }
        acc
    }

    /// A certified upper bound on `|f ∘ C|` over the whole domain —
    /// C2.2's sup-norm honesty limb as one number. `NaN` on every
    /// refusal path (fails `≤ ε` in every direction, D4 ¶2).
    pub fn sup_bound(&self) -> f64 {
        self.bound().mag()
    }
}

/// The analytic implicit surfaces whose composites `f ∘ C` this module
/// builds (C11's certification shapes for PR 5/7). All parameter data
/// is `f64` structure; see the module docs for the scaling conventions
/// (axis-bearing surfaces are normalized by `|axis|²` exactly in the
/// ring; the plane is not normalized).
#[derive(Clone, Copy, Debug)]
pub enum ImplicitSurface {
    /// `f(P) = n·(P − p₀)` — signed distance for unit `n`.
    Plane {
        /// A point `p₀` on the plane.
        point: [f64; 3],
        /// The plane normal `n` (unit for meters semantics).
        normal: [f64; 3],
    },
    /// `f(P) = |P − c|² − r²` (meters²).
    Sphere {
        /// The center `c`.
        center: [f64; 3],
        /// The radius `r` in meters.
        radius: f64,
    },
    /// `f(P) = |Q|² − (Q·â)² − r²` with `Q = P − c`, `â = a/|a|`
    /// (meters²; the `|a|²` normalization is exact in certification arithmetic).
    Cylinder {
        /// A point `c` on the axis.
        point: [f64; 3],
        /// The axis direction `a` (any nonzero length).
        axis: [f64; 3],
        /// The radius `r` in meters.
        radius: f64,
    },
    /// `f(P) = |Q|² − (1 + tan²γ)·(Q·â)²` with `Q = P − apex`
    /// (meters²) — zero on the double cone of half-angle `γ` about the
    /// axis through the apex.
    Cone {
        /// The apex.
        apex: [f64; 3],
        /// The axis direction `a` (any nonzero length).
        axis: [f64; 3],
        /// `tan γ` of the half-angle.
        tan_half_angle: f64,
    },
    /// `f(P) = (|Q|² + R² − r²)² − 4R²·(|Q|² − (Q·â)²)` with
    /// `Q = P − c` (meters⁴) — degree 4 in the point, still polynomial.
    Torus {
        /// The torus center `c`.
        center: [f64; 3],
        /// The axis direction `a` (any nonzero length).
        axis: [f64; 3],
        /// The major radius `R` in meters.
        major_radius: f64,
        /// The minor radius `r` in meters.
        minor_radius: f64,
    },
}

/// `Σ_d a_d²` as a certification enclosure (ascending `d`, `acc + p·p`).
fn axis_norm2(axis: &[f64; 3]) -> Interval {
    let mut acc = Interval::zero();
    for a in axis {
        let p = Interval::point(*a);
        acc = acc + p.sqr();
    }
    acc
}

/// `Σ_d G_d·G_d` — the squared-distance numerator channel `S`
/// (ascending `d`, accumulator seeded at zero: the rehearsal's
/// association, kept verbatim).
fn sum_of_squares(g: &[BernsteinSpans]) -> BernsteinSpans {
    let Some(first) = g.first() else {
        return BernsteinSpans {
            degree: 0,
            breaks: Vec::new(),
            spans: Vec::new(),
        };
    };
    let mut acc = ch_zero_like(first, first.degree * 2);
    for gd in g {
        acc = ch_add(&acc, &ch_mul(gd, gd));
    }
    acc
}

/// `Σ_d G_d·c_d` — a linear functional channel (ascending `d`,
/// coefficient on the right: the rehearsal's association).
fn dot_channel(g: &[BernsteinSpans], c: &[f64; 3]) -> BernsteinSpans {
    let Some(first) = g.first() else {
        return BernsteinSpans {
            degree: 0,
            breaks: Vec::new(),
            spans: Vec::new(),
        };
    };
    let mut acc = ch_zero_like(first, first.degree);
    for (gd, cd) in g.iter().zip(c.iter()) {
        acc = ch_add(&acc, &ch_scale_right(gd, Interval::point(*cd)));
    }
    acc
}

/// The composite `f ∘ C` for an analytic implicit surface along a 3-D
/// NURBS curve, in rational Bernstein form (module docs: pipeline and
/// scaling conventions). Requires exactly 3 coordinate channels.
///
/// # Errors
///
/// [`ComposeError::DimensionMismatch`] unless `data.dims() == 3` (the
/// structure errors were caught at [`CurveCertData::new`]).
pub fn implicit_composite(
    data: &CurveCertData<'_>,
    surface: &ImplicitSurface,
) -> Result<CompositeForm, ComposeError> {
    if data.dims() != 3 {
        return Err(ComposeError::DimensionMismatch {
            dims: data.dims(),
            expected: 3,
        });
    }
    let w = data.weight_channel();
    let shifted = |c: &[f64; 3]| -> [BernsteinSpans; 3] {
        [
            data.shifted_channel(0, c[0]),
            data.shifted_channel(1, c[1]),
            data.shifted_channel(2, c[2]),
        ]
    };
    let form = match surface {
        ImplicitSurface::Plane { point, normal } => {
            let g = shifted(point);
            CompositeForm {
                num: dot_channel(&g, normal),
                den: w,
            }
        }
        ImplicitSurface::Sphere { center, radius } => {
            let g = shifted(center);
            let w2 = ch_mul(&w, &w);
            let r = Interval::point(*radius);
            let r2 = r.sqr();
            let s = sum_of_squares(&g);
            CompositeForm {
                num: ch_sub(&s, &ch_scale_left(r2, &w2)),
                den: w2,
            }
        }
        ImplicitSurface::Cylinder {
            point,
            axis,
            radius,
        } => {
            let g = shifted(point);
            let w2 = ch_mul(&w, &w);
            let a2 = axis_norm2(axis);
            let r = Interval::point(*radius);
            let r2 = r.sqr();
            let s = sum_of_squares(&g);
            let t = dot_channel(&g, axis);
            // (A2·S − T·T) − (r²·A2)·W², over A2·W².
            let num = ch_sub(
                &ch_sub(&ch_scale_left(a2, &s), &ch_mul(&t, &t)),
                &ch_scale_left(r2 * a2, &w2),
            );
            CompositeForm {
                num,
                den: ch_scale_left(a2, &w2),
            }
        }
        ImplicitSurface::Cone {
            apex,
            axis,
            tan_half_angle,
        } => {
            let g = shifted(apex);
            let w2 = ch_mul(&w, &w);
            let a2 = axis_norm2(axis);
            let tg = Interval::point(*tan_half_angle);
            let k = Interval::one() + tg.sqr();
            let s = sum_of_squares(&g);
            let t = dot_channel(&g, axis);
            // A2·S − k·(T·T), over A2·W².
            let num = ch_sub(&ch_scale_left(a2, &s), &ch_scale_left(k, &ch_mul(&t, &t)));
            CompositeForm {
                num,
                den: ch_scale_left(a2, &w2),
            }
        }
        ImplicitSurface::Torus {
            center,
            axis,
            major_radius,
            minor_radius,
        } => {
            let g = shifted(center);
            let w2 = ch_mul(&w, &w);
            let a2 = axis_norm2(axis);
            let rr = Interval::point(*major_radius);
            let rm = Interval::point(*minor_radius);
            let c1 = rr.sqr() - rm.sqr(); // R² − r²
            let c4 = Interval::point(4.0) * rr.sqr(); // 4R²
            let s = sum_of_squares(&g);
            let t = dot_channel(&g, axis);
            // A2·(S + (R²−r²)·W²)² − 4R²·(A2·S − T·T)·W², over A2·W⁴.
            let s2 = ch_add(&s, &ch_scale_left(c1, &w2));
            let term1 = ch_scale_left(a2, &ch_mul(&s2, &s2));
            let inner = ch_sub(&ch_scale_left(a2, &s), &ch_mul(&t, &t));
            let term2 = ch_scale_left(c4, &ch_mul(&inner, &w2));
            CompositeForm {
                num: ch_sub(&term1, &term2),
                den: ch_scale_left(a2, &ch_mul(&w2, &w2)),
            }
        }
    };
    Ok(form)
}

/// The product of two coordinate splines `x_i(t)·x_j(t)` in rational
/// Bernstein form: numerator `A_i·A_j` (weighted, unshifted channels),
/// denominator `W²`.
///
/// # Errors
///
/// [`ComposeError::ChannelOutOfRange`] for a bad channel index.
pub fn coordinate_product(
    data: &CurveCertData<'_>,
    i: usize,
    j: usize,
) -> Result<CompositeForm, ComposeError> {
    let dims = data.dims();
    for ch in [i, j] {
        if ch >= dims {
            return Err(ComposeError::ChannelOutOfRange { channel: ch, dims });
        }
    }
    let w = data.weight_channel();
    let ai = data.weighted_channel(i);
    let aj = data.weighted_channel(j);
    Ok(CompositeForm {
        num: ch_mul(&ai, &aj),
        den: ch_mul(&w, &w),
    })
}

/// A linear functional along the curve, `Σ_d c_d·x_d(t) + offset`, in
/// rational Bernstein form: numerator `Σ_d A_d·c_d + W·offset`
/// (ascending `d`, then the offset term), denominator `W`. Works at
/// any channel count (2-D pcurves included).
///
/// # Errors
///
/// [`ComposeError::DimensionMismatch`] unless
/// `coeffs.len() == data.dims()`.
pub fn linear_composite(
    data: &CurveCertData<'_>,
    coeffs: &[f64],
    offset: f64,
) -> Result<CompositeForm, ComposeError> {
    if coeffs.len() != data.dims() {
        return Err(ComposeError::DimensionMismatch {
            dims: data.dims(),
            expected: coeffs.len(),
        });
    }
    let w = data.weight_channel();
    let mut acc = ch_zero_like(&w, w.degree);
    for (d, cd) in coeffs.iter().enumerate() {
        let ad = data.weighted_channel(d);
        acc = ch_add(&acc, &ch_scale_right(&ad, Interval::point(*cd)));
    }
    acc = ch_add(&acc, &ch_scale_right(&w, Interval::point(offset)));
    Ok(CompositeForm { num: acc, den: w })
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;
    use crate::real::Bounds;
    use crate::spline::basis;
    use num_bigint::BigInt;
    use num_integer::Integer;
    use num_traits::ToPrimitive;

    /// f64 rational-curve oracle: `x_d(t)` via A2.2 basis values —
    /// independent of every ring/decomposition code path.
    fn rat_eval(kv: &KnotVector, w: &[f64], coords: &[Vec<f64>], t: f64) -> Vec<f64> {
        let span = kv.span_at(t);
        let n = basis::basis_funs(span, t);
        let mut den = 0.0;
        let mut num = vec![0.0; coords.len()];
        for (j, nj) in n.iter().enumerate() {
            let i = span.first_control() + j;
            let cw = nj * w[i];
            den += cw;
            for (d, ch) in coords.iter().enumerate() {
                num[d] += cw * ch[i];
            }
        }
        num.iter().map(|v| v / den).collect()
    }

    /// The raw insertion path's span derivation, at **every step of a
    /// real multi-insertion decomposition**, against the linear scan it
    /// replaced. The integration differential
    /// (`tests/knot_queries_differential.rs`) covers the two searches on
    /// static vectors; this one covers the thing that static test
    /// cannot — that the knot list stays a valid clamped vector as it is
    /// mutated in place, so the binary search is entitled to run on it.
    #[test]
    fn the_raw_span_search_tracks_the_linear_scan_through_every_insertion() {
        let retired = |knots: &[f64], u: f64| {
            let mut k = 0;
            for (i, kn) in knots.iter().enumerate() {
                if *kn <= u {
                    k = i;
                }
            }
            k
        };
        // Degree 3, three interior values at multiplicities 1, 2, 3 —
        // so the decomposition inserts at a fresh value, at a partly
        // filled one, and skips the already-full one.
        let kv = KnotVector::clamped(
            vec![
                0.0, 0.0, 0.0, 0.0, 0.2, 0.5, 0.5, 0.8, 0.8, 0.8, 1.0, 1.0, 1.0, 1.0,
            ],
            3,
        )
        .unwrap();
        let p = kv.degree();
        let mut knots = kv.knots().to_vec();
        let mut coeffs: Vec<Interval> = (0..kv.control_count())
            .map(|i| Interval::point(i as f64))
            .collect();
        let interior: Vec<(InteriorKnot, usize)> = kv.interior_knot_runs().collect();
        assert_eq!(
            interior
                .iter()
                .map(|(k, m)| (k.value(), *m))
                .collect::<Vec<_>>(),
            vec![(0.2, 1), (0.5, 2), (0.8, 3)]
        );
        for (k, m) in &interior {
            let v = k.value();
            for step in *m..p {
                // Checked BEFORE the insertion, against the list as it
                // stands at this step — the state a static fixture has
                // no way to reach.
                assert_eq!(
                    find_span_in(&knots, p, v),
                    retired(&knots, v),
                    "step {step} at {v}: the binary search left the linear scan"
                );
                assert!(
                    KnotVector::clamped(knots.clone(), p).is_ok(),
                    "step {step} at {v}: the raw list stopped being a valid clamped vector"
                );
                insert_once_ring(&mut knots, p, step, &mut coeffs, *k);
            }
        }
        // Full multiplicity everywhere: four segments over degree 3.
        assert_eq!(knots.len(), 2 * (p + 1) + 3 * p);
        assert_eq!(coeffs.len(), 4 * p + 1);
    }

    // -----------------------------------------------------------------
    // The insertion combination: containment against exact rationals
    // -----------------------------------------------------------------

    /// An exact rational, `num/den` with `den > 0`, normalized. Only the
    /// operations the Boehm combination needs — the point is that the
    /// SHADOW of the ring fold carries no rounding at all, so a ring
    /// bound that fails to contain it is unsound by construction.
    ///
    /// **Why this is not [`crate::sym::Rat`]**, which is this crate's
    /// own exact rational over the same `num-bigint`: `Rat` is BOUNDED
    /// at `sym::rational`'s `COEFF_BITS` (256) and freezes the form it
    /// belongs to when a coefficient would exceed it. That is correct
    /// for the symbolic tier, whose budget is the point, and fatal
    /// here: the shadow's denominator picks up a factor of `Δ` per
    /// insertion, so a fold to full multiplicity over a clustered
    /// vector runs past 256 bits well before it ends, and a freeze
    /// would silently stop the oracle being exact — the one property it
    /// exists for.
    #[derive(Clone, Debug, PartialEq, Eq)]
    struct Q {
        num: BigInt,
        den: BigInt,
    }

    impl Q {
        fn new(num: BigInt, den: BigInt) -> Self {
            assert!(
                den != BigInt::from(0),
                "exact rational with zero denominator"
            );
            let (num, den) = if den < BigInt::from(0) {
                (-num, -den)
            } else {
                (num, den)
            };
            let g = num.gcd(&den);
            if g == BigInt::from(0) {
                return Self { num, den };
            }
            Self {
                num: num / &g,
                den: den / &g,
            }
        }

        /// Exact: an `f64` IS a dyadic rational, so this loses nothing.
        fn from_f64(x: f64) -> Self {
            assert!(x.is_finite(), "exact rational from {x}");
            if x == 0.0 {
                return Self {
                    num: BigInt::from(0),
                    den: BigInt::from(1),
                };
            }
            let bits = x.to_bits();
            let sign = if bits >> 63 == 1 { -1i32 } else { 1 };
            let raw_exp = ((bits >> 52) & 0x7ff) as i32;
            let frac = bits & ((1u64 << 52) - 1);
            // Subnormals carry no implicit leading bit and sit one
            // exponent up; the fixture families below reach them.
            let (mant, exp) = if raw_exp == 0 {
                (frac, -1074i32)
            } else {
                (frac | (1u64 << 52), raw_exp - 1075)
            };
            let m = BigInt::from(mant) * BigInt::from(sign);
            if exp >= 0 {
                Self::new(m << (exp as usize), BigInt::from(1))
            } else {
                Self::new(m, BigInt::from(1) << ((-exp) as usize))
            }
        }

        fn add(&self, o: &Self) -> Self {
            Self::new(&self.num * &o.den + &o.num * &self.den, &self.den * &o.den)
        }

        fn sub(&self, o: &Self) -> Self {
            Self::new(&self.num * &o.den - &o.num * &self.den, &self.den * &o.den)
        }

        fn mul(&self, o: &Self) -> Self {
            Self::new(&self.num * &o.num, &self.den * &o.den)
        }

        fn div(&self, o: &Self) -> Self {
            assert!(o.num != BigInt::from(0), "exact division by zero");
            Self::new(&self.num * &o.den, &self.den * &o.num)
        }

        fn cmp_f64(&self, x: f64) -> core::cmp::Ordering {
            let y = Self::from_f64(x);
            (&self.num * &y.den).cmp(&(&y.num * &self.den))
        }

        /// A reported-only `f64` rendering, total over the magnitudes
        /// this shadow reaches. Dividing `num.to_f64()` by
        /// `den.to_f64()` is NOT total: both overflow to `inf` once the
        /// fold's denominator passes 1024 bits, and `inf / inf` is
        /// `NaN` — which silently dropped 45 of the 487 slack samples
        /// from the figure the row reports. Scaling the numerator first
        /// and dividing in `BigInt` keeps the ratio.
        ///
        /// Never read by an assertion about containment: that is
        /// [`Q::cmp_f64`], which is exact.
        fn to_f64_report(&self) -> f64 {
            #[allow(clippy::cast_possible_wrap)]
            let (bn, bd) = (self.num.bits() as i64, self.den.bits() as i64);
            let shift = (64 - (bn - bd)).max(0);
            let q = (&self.num << usize::try_from(shift).unwrap_or(0)) / &self.den;
            let Some(v) = q.to_f64() else {
                return f64::NAN;
            };
            v * 2f64.powi(i32::try_from(-shift).unwrap_or(i32::MIN))
        }
    }

    /// One coefficient's TRUE value set: an exact rational interval.
    /// The Boehm combination is affine with both barycentric
    /// coefficients in `(0, 1)`, so it is monotone in each source and
    /// the extremes of the true set are attained at the sources' own
    /// endpoints — which is why two rationals describe it exactly.
    #[derive(Clone, Debug)]
    struct QInt {
        lo: Q,
        hi: Q,
    }

    /// The exact shadow of one [`insert_once_ring`] step, over the same
    /// `f64` knot list the ring fold reads: `λ` is the exact ratio of
    /// exact knot differences, and nothing here rounds.
    fn insert_once_exact(knots: &[f64], p: usize, s: usize, coeffs: &[QInt], u: f64) -> Vec<QInt> {
        let k = find_span_in(knots, p, u);
        let n_old = coeffs.len();
        let uq = Q::from_f64(u);
        let one = Q::new(BigInt::from(1), BigInt::from(1));
        let mut out = Vec::with_capacity(n_old + 1);
        for i in 0..=n_old {
            if i + p <= k {
                out.push(coeffs[i].clone());
            } else if i + s <= k {
                let lo = Q::from_f64(knots[i]);
                let hi = Q::from_f64(knots[i + p]);
                let lam = uq.sub(&lo).div(&hi.sub(&lo));
                let co = one.sub(&lam);
                out.push(QInt {
                    lo: co.mul(&coeffs[i - 1].lo).add(&lam.mul(&coeffs[i].lo)),
                    hi: co.mul(&coeffs[i - 1].hi).add(&lam.mul(&coeffs[i].hi)),
                });
            } else {
                out.push(coeffs[i - 1].clone());
            }
        }
        out
    }

    /// The fixture families, chosen to BREAK containment rather than to
    /// flatter it: `(name, degree, knot vector, coefficient endpoints)`.
    /// Each name says what it attacks.
    #[allow(clippy::type_complexity)]
    fn containment_fixtures() -> Vec<(&'static str, usize, Vec<f64>, Vec<(f64, f64)>)> {
        let pts = |v: &[f64]| -> Vec<(f64, f64)> { v.iter().map(|x| (*x, *x)).collect() };
        let mut out: Vec<(&'static str, usize, Vec<f64>, Vec<(f64, f64)>)> = Vec::new();

        // λ is 1/3 and 2/3 — not representable in f64, so the quotient
        // rounds on every step and outwardness is the only thing
        // keeping the truth inside.
        out.push((
            "non-dyadic ratios",
            3,
            vec![0.0, 0.0, 0.0, 0.0, 3.0, 6.0, 9.0, 9.0, 9.0, 9.0],
            pts(&[1.0, -2.0, 4.0, 8.0, -16.0, 32.0]),
        ));

        // Knots one ulp apart at 1.0: Δ is exact by Sterbenz but tiny,
        // so the quotient's relative rounding is at its worst.
        let e = f64::EPSILON;
        out.push((
            "ulp-scale knot spans",
            2,
            vec![
                1.0,
                1.0,
                1.0,
                1.0 + e,
                1.0 + 2.0 * e,
                1.0 + 3.0 * e,
                1.0 + 3.0 * e,
                1.0 + 3.0 * e,
            ],
            pts(&[1.0, -1.0, 1.0, -1.0, 1.0]),
        ));

        // Cancelling coefficients of huge magnitude: the combination's
        // true value is ~0 while its inputs are ~1e17, so any width the
        // form invents is astronomically larger than the answer — and
        // any width it FAILS to carry loses containment outright.
        out.push((
            "catastrophic cancellation",
            3,
            vec![0.0, 0.0, 0.0, 0.0, 1.0 / 3.0, 2.0 / 3.0, 1.0, 1.0, 1.0, 1.0],
            pts(&[1e17, -1e17, 1e17, -1e17, 1e17, -1e17]),
        ));

        // λ within an ulp of 0 and of 1: the near-degenerate ends of
        // the barycentric range, where `1 − α` and a knot-derived `β`
        // differ most.
        out.push((
            "ratios at the ends",
            3,
            vec![0.0, 0.0, 0.0, 0.0, 1e-300, 1.0 - 1e-16, 1.0, 1.0, 1.0, 1.0],
            pts(&[3.0, 5.0, 7.0, 11.0, 13.0, 17.0]),
        ));

        // Subnormal coefficients: the one regime where an ulp is
        // absolute rather than relative.
        out.push((
            "subnormal coefficients",
            2,
            vec![0.0, 0.0, 0.0, 0.25, 0.5, 0.75, 1.0, 1.0, 1.0],
            pts(&[5e-324, -1e-320, 3e-322, 0.0, 7e-323, -5e-324]),
        ));

        // Already-WIDE inputs, of both signs and mixed magnitudes: the
        // fold then has real dust to carry, which is the case the lerp
        // form multiplies up and the case a too-clever contraction
        // would lose.
        out.push((
            "wide inputs",
            3,
            vec![0.0, 0.0, 0.0, 0.0, 0.1, 0.3, 0.7, 1.0, 1.0, 1.0, 1.0],
            vec![
                (-1.0, 1.0),
                (0.999_999_999, 1.000_000_001),
                (-1e8, 1e8),
                (2.0, 2.0),
                (-3e-7, 5e-7),
                (1e10, 1.000_000_1e10),
                (-7.0, -6.999_999),
            ],
        ));

        // Degree 5 over an irregular, clustered knot vector at a knot
        // that already has multiplicity 2 — a deep fold with the window
        // arm entered at several `s`, which is where the accumulated
        // dust is largest.
        out.push((
            "deep irregular fold",
            5,
            vec![
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.017, 0.017, 0.33, 0.9, 0.999_9, 1.0, 1.0, 1.0, 1.0,
                1.0, 1.0,
            ],
            pts(&[
                1.0, -0.5, 0.25, -0.125, 6.0, -7.0, 8.0, -9.0, 10.0, 11.0, -12.0,
            ]),
        ));

        // Spans that overflow, go subnormal, or whose numerator
        // underflows: the regimes where the two quotients are least
        // like each other and a refusal is the sound answer. A REFUSAL
        // is fine (loud); an escape is not, and the row below treats
        // them differently.
        out.push((
            "overflowing knot span",
            2,
            vec![-1e308, -1e308, -1e308, 0.0, 1e308, 1e308, 1e308],
            pts(&[1.0, -1.0, 2.0, 3.0]),
        ));
        out.push((
            "subnormal knot span",
            2,
            vec![0.0, 0.0, 0.0, 5e-324, 1e-323, 1.5e-323, 1.5e-323, 1.5e-323],
            pts(&[1.0, -1.0, 1.0, -1.0, 1.0]),
        ));
        out.push((
            "numerator underflow across 1e300",
            3,
            vec![
                0.0, 0.0, 0.0, 0.0, 1e-308, 1e300, 1e308, 1e308, 1e308, 1e308,
            ],
            pts(&[1.0, -2.0, 4.0, -8.0, 16.0, -32.0]),
        ));

        // Wide coefficients that straddle zero at the SAME magnitude as
        // their neighbour — the cancellation the lerp form hides inside
        // its `c_i − c_{i−1}`.
        {
            let mut knots = vec![0.0; 5];
            knots.extend([0.125, 0.25, 0.375, 0.5, 0.625, 0.75, 0.875]);
            knots.extend(core::iter::repeat_n(1.0, 5));
            #[allow(clippy::cast_precision_loss)]
            let ends: Vec<(f64, f64)> = (0..12)
                .map(|i| {
                    let m = 1e16_f64 * if i % 2 == 0 { 1.0 } else { -1.0 };
                    (m - 1.0, m + 1.0)
                })
                .collect();
            out.push(("equal-magnitude straddling wide inputs", 4, knots, ends));
        }

        // Degree 7 over a vector clustered at an ulp of 0.5 — the
        // deepest fold the row runs, at the degree where the window arm
        // is entered most often.
        {
            let mut knots = vec![0.0; 8];
            knots.extend([
                1e-9,
                0.5 - f64::EPSILON,
                0.5,
                0.5 + f64::EPSILON,
                1.0 - 1e-9,
            ]);
            knots.extend(core::iter::repeat_n(1.0, 8));
            #[allow(clippy::cast_precision_loss)]
            let ends: Vec<(f64, f64)> = (0..13)
                .map(|i| {
                    let x = (i as f64) * 0.5 - 3.0;
                    (x, x + 1e-13)
                })
                .collect();
            out.push(("degree 7 clustered deep fold", 7, knots, ends));
        }

        out
    }

    /// **The claim this unit exists to establish: the convex form still
    /// ENCLOSES the true inserted coefficient.**
    ///
    /// The failure mode is a bound that comes out TIGHTER THAN TRUTH,
    /// which reads as an improvement and is an unsound certificate. A
    /// narrower bound is the expected outcome of the change and is
    /// therefore not evidence for it; containment is, and containment
    /// is what this asserts — against exact rational arithmetic, on
    /// families chosen to break it (non-dyadic ratios, ulp-scale knot
    /// spans, cancelling 1e17 coefficients, ratios an ulp from either
    /// end, subnormals, already-wide inputs, a degree-5 fold over a
    /// clustered vector, spans that overflow or go subnormal, a
    /// numerator that underflows across 1e300, equal-magnitude
    /// straddling wide inputs, and a degree-7 fold clustered at an ulp
    /// of 0.5).
    ///
    /// The shadow folds the SAME schedule the ring fold does, over the
    /// same `f64` knot list, with `λ` the exact ratio of exact knot
    /// differences. Its per-coefficient value set is exactly an
    /// interval because the combination is affine with coefficients in
    /// `(0, 1)`, so it is monotone in each source; the extremes are
    /// therefore attained at the sources' endpoints and two rationals
    /// describe the set with nothing left over.
    ///
    /// **What the shadow does NOT test, stated so nobody reads it as
    /// more than it is.** It is an oracle of the ARITHMETIC, not of the
    /// structure: it calls the same [`find_span_in`] and repeats the
    /// same `i + p <= k` / `i + s <= k` window predicates as the subject,
    /// so a wrong span or a mis-placed window boundary is invisible here
    /// — both folds would make the same wrong choice and agree. Those
    /// are covered separately, by
    /// `the_raw_span_search_tracks_the_linear_scan_through_every_insertion`
    /// (the span, against an independent linear scan, at every step of a
    /// real decomposition) and by the module's end-to-end soundness
    /// rows. What the shadow proves is that given the schedule, the
    /// ring combination encloses the real one.
    ///
    /// **A REFUSAL is sound and is treated as such.** On the overflow
    /// and subnormal families a quotient can legitimately refuse (D4);
    /// the row counts those and requires that a slot which does NOT
    /// refuse contains the truth. A row that demanded certification
    /// everywhere would be asserting that interval arithmetic never
    /// refuses, which is a different and false claim.
    #[test]
    fn the_ring_fold_encloses_the_exact_refined_net() {
        let mut worst_slack_ulps = f64::INFINITY;
        let (mut checked, mut refused, mut tie_lo, mut tie_hi) = (0usize, 0usize, 0usize, 0usize);
        let (mut unreported, mut no_ulp_scale) = (0usize, 0usize);
        for (name, p, knot_list, coeff_ends) in containment_fixtures() {
            let kv = KnotVector::clamped(knot_list, p).unwrap();
            assert_eq!(
                kv.control_count(),
                coeff_ends.len(),
                "{name}: fixture coefficient count does not match the knot vector"
            );
            let mut knots = kv.knots().to_vec();
            let mut ring: Vec<Interval> = coeff_ends
                .iter()
                .map(|(lo, hi)| Interval::from_bounds(*lo, *hi))
                .collect();
            let mut exact: Vec<QInt> = coeff_ends
                .iter()
                .map(|(lo, hi)| QInt {
                    lo: Q::from_f64(*lo),
                    hi: Q::from_f64(*hi),
                })
                .collect();
            let interior: Vec<(InteriorKnot, usize)> = kv.interior_knot_runs().collect();
            for (v, m) in &interior {
                for step in *m..p {
                    exact = insert_once_exact(&knots, p, step, &exact, v.value());
                    insert_once_ring(&mut knots, p, step, &mut ring, *v);
                    assert_eq!(ring.len(), exact.len(), "{name}: the two folds left step");
                    for (i, (r, x)) in ring.iter().zip(exact.iter()).enumerate() {
                        if !r.is_certified() {
                            // Loud, and outward: a refusal encloses
                            // everything, so it cannot be tighter than
                            // truth. Counted, not asserted away.
                            refused += 1;
                            continue;
                        }
                        // Containment, in exact arithmetic: the ring
                        // bracket must reach at or past the true set on
                        // BOTH sides. A tighter bound fails here, which
                        // is the whole point of the row.
                        assert!(
                            x.lo.cmp_f64(r.lo()) != core::cmp::Ordering::Less,
                            "{name}: slot {i} after inserting {} (step {step}) is TIGHTER \
                             THAN TRUTH below — ring lo {:e} is above the exact lo {:e}",
                            v.value(),
                            r.lo(),
                            x.lo.to_f64_report()
                        );
                        assert!(
                            x.hi.cmp_f64(r.hi()) != core::cmp::Ordering::Greater,
                            "{name}: slot {i} after inserting {} (step {step}) is TIGHTER \
                             THAN TRUTH above — ring hi {:e} is below the exact hi {:e}",
                            v.value(),
                            r.hi(),
                            x.hi.to_f64_report()
                        );
                        checked += 1;
                        // Is the reported "0.00 ulps of slack" a REAL
                        // zero — the ring bracket exactly on the truth
                        // in exact arithmetic — or a rendering of a
                        // small positive slack? Counted exactly, so the
                        // headline figure cannot be a rounding.
                        if x.lo.cmp_f64(r.lo()) == core::cmp::Ordering::Equal {
                            tie_lo += 1;
                        }
                        if x.hi.cmp_f64(r.hi()) == core::cmp::Ordering::Equal {
                            tie_hi += 1;
                        }
                        // How much room the enclosure has left over the
                        // truth, in ulps of the slot's own scale. It is
                        // reported, not asserted tight: this row is
                        // about the sound direction only. What IS
                        // asserted is that every certified slot reaches
                        // the reported figure — a metric that silently
                        // drops its hard cases says nothing.
                        let scale = r.lo().abs().max(r.hi().abs());
                        let ulp = scale * f64::EPSILON;
                        #[allow(clippy::neg_cmp_op_on_partial_ord)]
                        if !(ulp > 0.0) || !ulp.is_finite() {
                            // The slot has no representable ulp scale:
                            // it is exactly `{0}`, or it sits at a
                            // subnormal magnitude where `scale·ε`
                            // underflows to zero. Legitimate, and
                            // distinguished from a sample the metric
                            // LOST — the exact asserts above cover
                            // these slots like any other, so a truth
                            // outside one would have red there.
                            no_ulp_scale += 1;
                        } else {
                            let slack = (r.hi() - x.hi.to_f64_report())
                                .min(x.lo.to_f64_report() - r.lo())
                                / ulp;
                            if slack.is_finite() {
                                worst_slack_ulps = worst_slack_ulps.min(slack);
                            } else {
                                unreported += 1;
                            }
                        }
                    }
                }
            }
            println!("{name}: contained through the fold to full multiplicity");
        }
        assert!(
            checked > 400,
            "only {checked} slot comparisons — the fixture families stopped folding"
        );
        // The slack figure is reported over EVERY certified comparison
        // or it is not reported. `to_f64_report`'s predecessor divided
        // two overflowed `BigInt::to_f64()`s and lost 45 of 487 samples
        // to `inf / inf`; a zero-scale slot (a true zero coefficient
        // held exactly) is the only legitimate exclusion.
        assert!(
            unreported == 0,
            "{unreported} of {checked} certified comparisons never reached the reported \
             slack figure, so '{worst_slack_ulps:.2} ulps' is taken over a subset the row \
             does not name ({no_ulp_scale} slots with no representable ulp scale are \
             excluded by construction and are not these)"
        );
        // The zero is a real zero: some slot's bracket lands exactly on
        // the truth in exact arithmetic. If this ever reads 0 while the
        // slack prints 0.00, the headline is a rendering and not a
        // measurement.
        assert!(
            tie_lo > 0 && tie_hi > 0,
            "no slot is exactly on the truth ({tie_lo} lo, {tie_hi} hi), so the reported \
             {worst_slack_ulps:.2} ulps of slack is a rounded positive number"
        );
        println!(
            "{checked} exact containment comparisons ({refused} slots refused, which is \
             outward; {no_ulp_scale} carry no representable ulp scale); tightest \
             side left {worst_slack_ulps:.2} ulps of slack over the truth, and it is an \
             EXACT tie on {tie_lo} slots below and {tie_hi} above"
        );
    }

    /// **The width the lerp form gives away, asserted as a ceiling the
    /// lerp form cannot meet.**
    ///
    /// With point inputs the only width in the answer is the ratios'
    /// own outward rounding, so it accumulates ADDITIVELY — a few ulps
    /// of the coefficient scale per insertion. The lerp form reads
    /// `c_{i−1}` twice, so its dust is multiplied by `1 + α` per step
    /// and grows GEOMETRICALLY with the fold's depth. The ceiling is
    /// therefore stated against the insertion COUNT: a per-step growth
    /// factor blows through it, an additive one does not.
    ///
    /// [`to_bezier_spans_extra`] inserts each interior knot to full
    /// multiplicity, `p − m` steps for an existing multiplicity `m`, so
    /// the deep fold this pins is the one the composite bounds actually
    /// pay: degree 6, sixteen interior knots already present once, five
    /// insertions each, 80 in all.
    ///
    /// **What this ceiling can and cannot catch, said at the claim
    /// site.** The allowance has a lot of room: 7.0x at `p = 2` over 30
    /// insertions, widening to about 15x at `p = 6` over 200, because
    /// the convex form's width per insertion FALLS with depth while the
    /// line's slope is constant. So it cannot catch a regression under
    /// roughly 7x, and the 0.06-ulps-per-insertion and 100x figures the
    /// row prints are evidence for a reader, not gates. Tightening it to
    /// bite harder would mean a per-degree ceiling fitted to measured
    /// values, i.e. a second baseline to re-derive whenever any
    /// coefficient of the assembly changes; the claim this row is for —
    /// the fold does not INFLATE, additive against geometric — is a
    /// claim about the shape of the growth, and a linear allowance is
    /// the honest statement of it. The lerp form misses it by 431.7
    /// ulps, which is the size of regression it exists to catch.
    #[test]
    fn the_convex_form_does_not_inflate_the_fold() {
        // (degree, interior count). The first is TESS-2's measuring
        // shape — degree 2, thirty insertions; the last is the deep
        // full-multiplicity fold the decomposition actually runs.
        let cases: &[(usize, usize)] = &[(2, 30), (3, 16), (6, 16)];
        let mut worst_excess = f64::NEG_INFINITY;
        for &(p, m) in cases {
            let mut knot_list = vec![0.0; p + 1];
            #[allow(clippy::cast_precision_loss)]
            for j in 1..=m {
                knot_list.push(j as f64 / (m + 1) as f64);
            }
            knot_list.extend(core::iter::repeat_n(1.0, p + 1));
            let kv = KnotVector::clamped(knot_list, p).unwrap();
            let n = kv.control_count();
            // Both signs and O(1) magnitudes, so a sign error in either
            // ratio shows as an escape rather than as width.
            #[allow(clippy::cast_precision_loss)]
            let coeffs: Vec<f64> = (0..n)
                .map(|i| {
                    if i % 2 == 0 {
                        1.0 + i as f64
                    } else {
                        -(1.0 + i as f64)
                    }
                })
                .collect();
            let scale = coeffs.iter().fold(0.0f64, |a, c| a.max(c.abs()));
            let mut knots = kv.knots().to_vec();
            let mut ring: Vec<Interval> = coeffs.iter().copied().map(Interval::point).collect();
            let mut insertions = 0usize;
            for (v, s) in kv.interior_knot_runs().collect::<Vec<_>>() {
                for step in s..p {
                    insert_once_ring(&mut knots, p, step, &mut ring, v);
                    insertions += 1;
                }
            }
            let worst = ring.iter().fold(0.0f64, |a, r| {
                assert!(r.is_certified(), "p={p}: refused slot in the fold");
                a.max(r.hi() - r.lo())
            });
            // The inputs are points, but the refined coefficients are
            // combinations at non-dyadic ratios of neighbours that
            // differ, so their true values are not `f64`s and no slot
            // may come out a point: a fold that held every slot exactly
            // would be reporting a tighter enclosure than its own
            // rounding licenses.
            assert!(
                worst > 0.0,
                "p={p}: every slot of a {insertions}-insertion fold held a point, so the \
                 ratios stopped rounding outward"
            );
            let ulps = worst / (scale * f64::EPSILON);
            #[allow(clippy::cast_precision_loss)]
            let allowance = 2.0 + 0.5 * insertions as f64;
            worst_excess = worst_excess.max(ulps - allowance);
            #[allow(clippy::cast_precision_loss)]
            let per_insertion = ulps / insertions as f64;
            println!(
                "p={p}, {insertions} insertions: widest slot {worst:.3e} ({ulps:.1} ulps of \
                 the coefficient scale, {per_insertion:.2} per insertion, allowance \
                 {allowance:.1})"
            );
        }
        // The allowance is LINEAR in the insertion count, and that is
        // the whole content of the claim: two outward-rounded quotients
        // and one combination add a fraction of an ulp per step, so a
        // line through them holds at any depth. A form that reads a
        // coefficient twice multiplies its dust by `1 + α` per step
        // instead, and no line holds a geometric series — the lerp form
        // clears this by 473.7 ulps against 42.0 on the p=6 row.
        assert!(
            worst_excess < 0.0,
            "a fold ran {worst_excess:.1} ulps of width past a linear-in-depth allowance, so \
             the width is no longer the two ratios rounding outward once per step — a form \
             that reads a coefficient twice multiplies its dust up instead of adding to it"
        );
    }

    fn lift(coords: &[Vec<f64>]) -> Vec<Vec<Interval>> {
        coords
            .iter()
            .map(|ch| ch.iter().map(|x| Interval::point(*x)).collect())
            .collect()
    }

    /// The composite's whole-domain bound contains every densely
    /// sampled residual value (soundness by falsification).
    fn assert_sound(
        form: &CompositeForm,
        kv: &KnotVector,
        w: &[f64],
        coords: &[Vec<f64>],
        f: impl Fn(&[f64]) -> f64,
    ) -> f64 {
        let b = form.sup_bound();
        assert!(b.is_finite(), "refused bound");
        let (lo, hi) = kv.domain();
        let mut worst = 0.0f64;
        for k in 0..=1024 {
            let t = lo + (hi - lo) * (f64::from(k) / 1024.0);
            let r = f(&rat_eval(kv, w, coords, t)).abs();
            assert!(r <= b, "sample {r:e} at t = {t} exceeds bound {b:e}");
            worst = worst.max(r);
        }
        worst
    }

    #[test]
    fn cone_composite_vanishes_on_a_ruling_line() {
        // Line through the apex at half-angle γ: exactly on the cone.
        let kv = KnotVector::clamped(vec![0.0, 0.0, 3.0, 3.0], 1).unwrap();
        let (tg, l) = (0.75, 4.0);
        let coords = vec![
            vec![0.0, tg * l], // x = tanγ·z direction, |axis| slope
            vec![0.0, 0.0],
            vec![0.0, l],
        ];
        let w = vec![1.0, 1.0];
        let ring = lift(&coords);
        let data = CurveCertData::new(&kv, &w, &ring).unwrap();
        let cone = ImplicitSurface::Cone {
            apex: [0.0, 0.0, 0.0],
            axis: [0.0, 0.0, 1.0],
            tan_half_angle: tg,
        };
        let form = implicit_composite(&data, &cone).unwrap();
        let worst = assert_sound(&form, &kv, &w, &coords, |p| {
            let q2 = p[0] * p[0] + p[1] * p[1] + p[2] * p[2];
            q2 - (1.0 + tg.powi(2)) * p[2] * p[2]
        });
        assert!(form.sup_bound() < 1e-12, "bound {:e}", form.sup_bound());
        assert!(worst <= form.sup_bound());
    }

    #[test]
    fn cylinder_composite_vanishes_on_a_parallel_line_and_scales_off_axis() {
        let kv = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
        let r = 1.25;
        let coords = vec![vec![r, r], vec![0.0, 0.0], vec![-2.0, 5.0]];
        let w = vec![1.0, 1.0];
        let ring = lift(&coords);
        let data = CurveCertData::new(&kv, &w, &ring).unwrap();
        // Non-unit axis on purpose: the |a|² normalization is exact in
        // interval arithmetic, so the bound is still the meters² residual.
        let cyl = ImplicitSurface::Cylinder {
            point: [0.0, 0.0, 1.0],
            axis: [0.0, 0.0, 3.0],
            radius: r,
        };
        let form = implicit_composite(&data, &cyl).unwrap();
        assert_sound(&form, &kv, &w, &coords, |p| {
            p[0] * p[0] + p[1] * p[1] - r.powi(2)
        });
        assert!(form.sup_bound() < 1e-13, "bound {:e}", form.sup_bound());
    }

    /// The §7.3 rational-quadratic full circle of radius `rad` in the
    /// xy-plane about the origin (exact-on-locus in ℝ).
    fn circle_fixture(rad: f64) -> (KnotVector, Vec<f64>, Vec<Vec<f64>>) {
        let s = core::f64::consts::FRAC_1_SQRT_2;
        let kv = KnotVector::clamped(
            vec![
                0.0, 0.0, 0.0, 0.25, 0.25, 0.5, 0.5, 0.75, 0.75, 1.0, 1.0, 1.0,
            ],
            2,
        )
        .unwrap();
        let x = vec![rad, rad, 0.0, -rad, -rad, -rad, 0.0, rad, rad];
        let y = vec![0.0, rad, rad, rad, 0.0, -rad, -rad, -rad, 0.0];
        let z = vec![0.0; 9];
        let w = vec![1.0, s, 1.0, s, 1.0, s, 1.0, s, 1.0];
        (kv, w, vec![x, y, z])
    }

    #[test]
    fn torus_composite_vanishes_on_the_outer_equator() {
        let (big_r, small_r) = (2.0, 0.5);
        let (kv, w, coords) = circle_fixture(big_r + small_r);
        let ring = lift(&coords);
        let data = CurveCertData::new(&kv, &w, &ring).unwrap();
        let torus = ImplicitSurface::Torus {
            center: [0.0, 0.0, 0.0],
            axis: [0.0, 0.0, 2.0], // non-unit on purpose
            major_radius: big_r,
            minor_radius: small_r,
        };
        let form = implicit_composite(&data, &torus).unwrap();
        assert_sound(&form, &kv, &w, &coords, |p| {
            let q2 = p[0] * p[0] + p[1] * p[1] + p[2] * p[2];
            let c = q2 + big_r.powi(2) - small_r.powi(2);
            c.powi(2) - 4.0 * big_r.powi(2) * (q2 - p[2] * p[2])
        });
        // Exact locus: the bound is pure fp scale (meters⁴ at
        // magnitudes ~ (R+r)⁴ ≈ 39).
        assert!(form.sup_bound() < 1e-11, "bound {:e}", form.sup_bound());
    }

    #[test]
    fn products_and_linear_functionals_are_sound_across_real_decomposition() {
        // Degree-2 spline with a single-multiplicity interior knot —
        // the Bézier decomposition actually inserts here.
        let kv = KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.5, 1.0, 1.0, 1.0], 2).unwrap();
        let coords = vec![vec![0.0, 0.7, 1.3, 2.0], vec![1.0, -0.4, 0.9, -1.5]];
        let w = vec![1.0, 0.8, 1.3, 1.0];
        let ring = lift(&coords);
        let data = CurveCertData::new(&kv, &w, &ring).unwrap();

        let prod = coordinate_product(&data, 0, 1).unwrap();
        assert_sound(&prod, &kv, &w, &coords, |p| p[0] * p[1]);

        let lin = linear_composite(&data, &[2.0, -1.0], 3.0).unwrap();
        assert_sound(&lin, &kv, &w, &coords, |p| 2.0 * p[0] - p[1] + 3.0);

        // Tightness sanity: the linear bound is within the coefficient
        // hull's honest reach (not vacuously wide).
        let worst = assert_sound(&lin, &kv, &w, &coords, |p| 2.0 * p[0] - p[1] + 3.0);
        assert!(lin.sup_bound() <= 4.0 * worst.max(1.0));
    }

    /// **The convolution this unit replaced, kept verbatim**: the body
    /// [`bern_mul_row`] had before the weight table existed, with the
    /// binomial rows read and interval arithmetic quotient formed inside the
    /// coefficient loop. It is the oracle for every bit-identity row
    /// here and in [`super::patch`], so those rows pin the memo
    /// against the code it retired rather than against a second typing
    /// of the code that replaced it — a re-typed oracle agrees with a
    /// mis-typed builder.
    pub(super) fn bern_mul_row_base(a: &[Interval], b: &[Interval]) -> Vec<Interval> {
        let da = a.len() - 1;
        let db = b.len() - 1;
        let bin_a = binom_row(da);
        let bin_b = binom_row(db);
        let bin_ab = binom_row(da + db);
        let mut out = Vec::with_capacity(da + db + 1);
        for (k, bk) in bin_ab.iter().enumerate() {
            let mut acc = Interval::zero();
            for (i, ai) in a.iter().enumerate() {
                let Some(j) = k.checked_sub(i) else { continue };
                if j > db {
                    continue;
                }
                let w = Interval::point(bin_a[i] * bin_b[j]) / Interval::point(*bk);
                acc = acc + *ai * b[j] * w;
            }
            out.push(acc);
        }
        out
    }

    /// A coefficient row of degree `n` with mixed sign, magnitude and
    /// width, so a dropped, reordered or mis-weighted term shows in
    /// the endpoints.
    pub(super) fn sample_row(n: usize, seed: usize) -> Vec<Interval> {
        (0..=n)
            .map(|i| {
                let c = (i as f64 - 3.5) * (1.0 + seed as f64) / 7.0;
                Interval::from_bounds(c - 1e-13, c + 3e-13)
            })
            .collect()
    }

    /// Bitwise identity of a certification value: NaN endpoints compare equal
    /// to each other and to nothing else, which is what a refused
    /// weight has to preserve.
    pub(super) fn same_bits(x: Interval, y: Interval) -> bool {
        x.lo().to_bits() == y.lo().to_bits() && x.hi().to_bits() == y.hi().to_bits()
    }

    #[test]
    fn the_memoized_weight_table_is_the_recurrence_it_replaces_bitwise() {
        // Every degree pair the kernel can produce, and then the
        // straddle: `da + db` past the exactness cap (all-refused rows)
        // and a single degree past it (the un-slotted path).
        let mut pairs: Vec<(usize, usize)> = Vec::new();
        for da in 0..=12 {
            for db in 0..=12 {
                pairs.push((da, db));
            }
        }
        pairs.extend([
            (BINOM_EXACT_MAX, 0),
            (BINOM_EXACT_MAX - 1, 1),
            (27, 27),
            (BINOM_EXACT_MAX, 1),
            (BINOM_EXACT_MAX, BINOM_EXACT_MAX),
            (BINOM_EXACT_MAX + 1, 0),
            (BINOM_EXACT_MAX + 1, 3),
            (80, 2),
        ]);
        for (da, db) in pairs {
            let table = bern_weights(da, db);
            // The table knows which pair it is for, and its keying is
            // the convolution's: `lo(k) = k − db` clamped at zero.
            assert_eq!((table.da, table.db), (da, db), "pair at ({da}, {db})");
            assert_eq!(table.row_count(), da + db + 1, "row count at ({da}, {db})");
            let refused = da + db > BINOM_EXACT_MAX;
            for k in 0..table.row_count() {
                let lo = table.lo(k);
                assert_eq!(lo, k.saturating_sub(db), "lo({k}) at ({da}, {db})");
                assert_eq!(
                    table.row(k).len(),
                    k.min(da) + 1 - lo,
                    "row {k} width at ({da}, {db})"
                );
                for (t, w) in table.row(k).iter().enumerate() {
                    // `at` is the one door the u-lookup reads through;
                    // it must land on the entry the fold folds.
                    assert!(
                        same_bits(*w, table.at(k, lo + t)),
                        "at({k}, {}) at ({da}, {db})",
                        lo + t
                    );
                    // Past the cap the row is all-refused, and the memo
                    // must carry the refusal rather than a rounded
                    // weight (`binom_row`'s contract).
                    assert_eq!(
                        w.lo().is_nan() && w.hi().is_nan(),
                        refused,
                        "refusal at ({da}, {db}) k={k} i={}",
                        lo + t
                    );
                }
            }
            // The arithmetic, against the loop the table replaced: a
            // product driven through the memo equals the pre-memo
            // convolution, coefficient for coefficient, bitwise. A
            // memo whose entries were merely re-typed from the builder
            // would pass every assertion above and fail this one.
            let (a, b) = (sample_row(da, da + 1), sample_row(db, db + 3));
            let (got, want) = (bern_mul_row(&a, &b), bern_mul_row_base(&a, &b));
            assert_eq!(got.len(), want.len(), "row length at ({da}, {db})");
            for (k, (g, w)) in got.iter().zip(want.iter()).enumerate() {
                assert!(
                    same_bits(*g, *w),
                    "memoized product ({da}, {db}) coefficient {k}: {g:?} vs {w:?}"
                );
            }
        }
    }

    #[test]
    fn the_row_product_is_the_inline_convolution_bitwise() {
        for da in 0..=9 {
            for db in 0..=9 {
                let a = sample_row(da, da + 1);
                let b = sample_row(db, db + 3);
                // The pre-memo spelling, term for term.
                let want = bern_mul_row_base(&a, &b);
                let got = bern_mul_row(&a, &b);
                assert_eq!(got.len(), want.len(), "row length at ({da}, {db})");
                for (k, (g, w)) in got.iter().zip(want.iter()).enumerate() {
                    assert!(
                        same_bits(*g, *w),
                        "product ({da}, {db}) coefficient {k}: {g:?} vs {w:?}"
                    );
                }
                // A buffer carrying another product's row must leave
                // no residue: the scratch entry point overwrites.
                let mut buf = bern_mul_row(&b, &a);
                buf.push(Interval::point(7.0));
                let w = bern_weights(da, db);
                bern_mul_row_into(&a, &b, &w, &mut buf);
                assert_eq!(buf.len(), want.len(), "reused buffer length ({da}, {db})");
                for (k, (g, w)) in buf.iter().zip(want.iter()).enumerate() {
                    assert!(
                        same_bits(*g, *w),
                        "reused buffer ({da}, {db}) coefficient {k}: {g:?} vs {w:?}"
                    );
                }
            }
        }
    }

    #[test]
    fn typed_refusals_and_refusal_paths() {
        let kv = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
        let coords2 = lift(&[vec![0.0, 1.0], vec![0.0, 1.0]]);
        let w = vec![1.0, 1.0];
        let data2 = CurveCertData::new(&kv, &w, &coords2).unwrap();
        // Implicit surfaces need 3 channels.
        let sphere = ImplicitSurface::Sphere {
            center: [0.0; 3],
            radius: 1.0,
        };
        assert!(matches!(
            implicit_composite(&data2, &sphere),
            Err(ComposeError::DimensionMismatch {
                dims: 2,
                expected: 3
            })
        ));
        assert!(matches!(
            coordinate_product(&data2, 0, 2),
            Err(ComposeError::ChannelOutOfRange {
                channel: 2,
                dims: 2
            })
        ));
        assert!(matches!(
            linear_composite(&data2, &[1.0], 0.0),
            Err(ComposeError::DimensionMismatch { .. })
        ));
        // Bad weights are typed at construction.
        assert!(matches!(
            CurveCertData::new(&kv, &[1.0, -1.0], &coords2),
            Err(ComposeError::Structure(
                SplineError::NonPositiveWeight { .. }
            ))
        ));
        // A zero axis reaches the denominator as a zero-touching
        // divisor: interval arithmetic refuses, the bound is refused (NaN), and NaN
        // fails every ≤ ε certification (D4 ¶2).
        let coords3 = lift(&[vec![0.0, 1.0], vec![0.0, 1.0], vec![0.0, 1.0]]);
        let data3 = CurveCertData::new(&kv, &w, &coords3).unwrap();
        let cyl = ImplicitSurface::Cylinder {
            point: [0.0; 3],
            axis: [0.0; 3],
            radius: 1.0,
        };
        let form = implicit_composite(&data3, &cyl).unwrap();
        assert!(form.sup_bound().is_nan());
    }

    /// MINOR-1 pin (adversarial review): the recurrence is exact through
    /// `BINOM_EXACT_MAX` (differential vs `u128` integer arithmetic) and
    /// refuses — all-NaN, never a rounded weight — beyond it. The
    /// first inexact recurrence row is n = 55, where the intermediate
    /// product exceeds 2⁵³ although `C(55, 26)` is representable.
    #[test]
    fn binom_rows_are_exact_through_the_cap_and_nan_beyond_it() {
        for n in 0..=BINOM_EXACT_MAX {
            let row = binom_row(n);
            let mut exact: u128 = 1;
            for (k, v) in row.iter().enumerate() {
                if k > 0 {
                    exact = exact * (n as u128 - k as u128 + 1) / k as u128;
                }
                #[allow(clippy::cast_precision_loss)]
                let e = exact as f64;
                assert!(exact < (1u128 << 53), "C({n},{k}) not representable");
                assert_eq!(v.to_bits(), e.to_bits(), "C({n},{k}): {v} vs {exact}");
            }
        }
        assert!(binom_row(BINOM_EXACT_MAX + 1).iter().all(|v| v.is_nan()));
        assert!(binom_row(80).iter().all(|v| v.is_nan()));
    }

    #[test]
    fn bit_replay_composites_are_deterministic() {
        let (kv, w, coords) = circle_fixture(2.5);
        let ring = lift(&coords);
        let data = CurveCertData::new(&kv, &w, &ring).unwrap();
        let sphere = ImplicitSurface::Sphere {
            center: [0.1, -0.2, 0.3],
            radius: 2.5,
        };
        let a = implicit_composite(&data, &sphere).unwrap();
        let b = implicit_composite(&data, &sphere).unwrap();
        assert_eq!(a.sup_bound().to_bits(), b.sup_bound().to_bits());
        for (ba, bb) in a.span_bounds().iter().zip(b.span_bounds().iter()) {
            assert_eq!(ba.lo().to_bits(), bb.lo().to_bits());
            assert_eq!(ba.hi().to_bits(), bb.hi().to_bits());
        }
    }
}
