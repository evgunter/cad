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
//! 1. Lift the homogeneous channels in the ring: per coordinate `d`,
//!    `G_d,i = w_i·(x_d,i − c_d)` (center-shifted **before** any
//!    product — forming `|P|² − 2P·c + …` instead would cancel
//!    catastrophically in the coefficients, and an interval bound
//!    reports that cancellation as width rather than hiding it), plus
//!    the weight channel `W_i = w_i`.
//! 2. Bézier-decompose each channel: knot insertion to full interior
//!    multiplicity, **structure** (positions, counts) read from the
//!    `f64` knot vector, **coefficients** combined in the ring with the
//!    insertion `α` formed as a ring quotient of knot enclosures (an
//!    `f64`-rounded `α` would silently drop its rounding error).
//! 3. Per span, exact Bernstein products: degree `da × db → da + db`
//!    with the binomial weights `C(da,i)·C(db,j)/C(da+db,k)` computed
//!    as ring quotients (several are not `f64`-representable).
//! 4. Bounds: per-span coefficient hulls of numerator and denominator,
//!    the quotient per span (the ring refuses a zero-touching divisor,
//!    so a degenerate denominator poisons loudly), hulled across spans.
//!
//! # Scaling conventions (what the bound means)
//!
//! With unit `normal`/`axis` data the composites are the standard
//! signed residuals: meters for [`ImplicitSurface::Plane`], meters² for
//! sphere/cylinder/cone, meters⁴ for the torus. Axis normalization for
//! the quadratic terms is **exact in the ring** — `(Q·a)²` enters as
//! `T²/|a|²` with `|a|²` a ring enclosure — so a non-unit axis changes
//! nothing for those terms; the plane's linear normalization would need
//! a square root, which the ring deliberately lacks, so the plane
//! composite is `n·(P − p₀)` as given (meters only for unit `n`).
//!
//! # C6 and the poison posture
//!
//! Structure (knots, weights, degrees, binomials) is `f64`; everything
//! coefficient-valued is [`RingInterval`]. Checkable structural errors
//! at the entry points are typed refusals; anything downstream (zero
//! axis, degenerate weights) poisons the bound, which fails every
//! `≤ ε` comparison (D4 ¶2).

use super::knots::{KnotVector, SplineError};
use crate::ring_interval::RingInterval;

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
}

impl core::fmt::Display for ComposeError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            ComposeError::Structure(e) => write!(f, "compose: {e}"),
            ComposeError::DimensionMismatch { dims, expected } => {
                write!(f, "compose: {dims} coordinate channels, need {expected}")
            }
            ComposeError::ChannelOutOfRange { channel, dims } => {
                write!(f, "compose: channel {channel} out of range ({dims} channels)")
            }
        }
    }
}

impl core::error::Error for ComposeError {}

/// A NURBS curve's structure plus ring-lifted control coordinates —
/// the data-in shape every composite consumes. `coords[d][i]` is the
/// `d`-th coordinate of control point `i` as a ring enclosure (a plain
/// `f64` control point lifts via [`RingInterval::point`]; a perturbed
/// or interval-valued one via [`RingInterval::from_bounds`]).
#[derive(Clone, Debug)]
pub struct CurveRingData<'a> {
    kv: &'a KnotVector,
    weights: &'a [f64],
    coords: &'a [Vec<RingInterval>],
}

// `!(w > 0)` is deliberate (NaN-catching): see `algebra::check_weights`.
#[allow(clippy::neg_cmp_op_on_partial_ord)]
impl<'a> CurveRingData<'a> {
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
        coords: &'a [Vec<RingInterval>],
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
                return Err(ComposeError::Structure(
                    SplineError::ControlCountMismatch {
                        control: ch.len(),
                        expected: n,
                    },
                ));
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
        let coeffs: Vec<RingInterval> =
            self.weights.iter().map(|w| RingInterval::point(*w)).collect();
        to_bezier_spans(self.kv, &coeffs)
    }

    /// The unshifted weighted channel `w_i·x_d,i`, Bézier-decomposed.
    /// Caller guarantees `d < dims()`.
    fn weighted_channel(&self, d: usize) -> BernsteinSpans {
        let coeffs: Vec<RingInterval> = self.coords[d]
            .iter()
            .zip(self.weights.iter())
            .map(|(x, w)| RingInterval::point(*w) * *x)
            .collect();
        to_bezier_spans(self.kv, &coeffs)
    }

    /// The shifted weighted channel `w_i·(x_d,i − shift)` (module docs
    /// step 1: shift before any product), Bézier-decomposed. Caller
    /// guarantees `d < dims()`.
    fn shifted_channel(&self, d: usize, shift: f64) -> BernsteinSpans {
        let s = RingInterval::point(shift);
        let coeffs: Vec<RingInterval> = self.coords[d]
            .iter()
            .zip(self.weights.iter())
            .map(|(x, w)| RingInterval::point(*w) * (*x - s))
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
    spans: Vec<Vec<RingInterval>>,
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
    pub fn spans(&self) -> &[Vec<RingInterval>] {
        &self.spans
    }

    /// Per-span coefficient hulls (the convexity fact of
    /// [`super::hull`]: each is a certified enclosure of the channel's
    /// values on that span). Fixed ascending fold order (D9).
    pub fn span_hulls(&self) -> Vec<RingInterval> {
        self.spans
            .iter()
            .map(|row| {
                let mut acc = RingInterval::poison();
                for (n, c) in row.iter().enumerate() {
                    acc = if n == 0 { *c } else { RingInterval::hull(acc, *c) };
                }
                acc
            })
            .collect()
    }
}

// ---------------------------------------------------------------------
// Bézier decomposition (module docs step 2)
// ---------------------------------------------------------------------

/// The distinct interior knot values of `kv` with their multiplicities
/// (ascending — structure scan, exact `f64` identity).
fn interior_values(kv: &KnotVector) -> Vec<(f64, usize)> {
    let p = kv.degree();
    let knots = kv.knots();
    let mut out = Vec::new();
    let mut idx = p + 1;
    while idx < knots.len() - p - 1 {
        let v = knots[idx];
        let mut m = 1;
        while idx + m < knots.len() - p - 1 && knots[idx + m] == v {
            m += 1;
        }
        out.push((v, m));
        idx += m;
    }
    out
}

/// One Boehm insertion of `u` into the raw knot list `knots` (degree
/// `p`, current multiplicity `s` of `u`), coefficients combined in the
/// ring: `Q_i = c_{i−1} + (c_i − c_{i−1})·α_i` with
/// `α_i = (u − U_i)/(U_{i+p} − U_i)` formed as a **ring quotient** of
/// knot enclosures (module docs step 2). Fixed ascending index order.
fn insert_once_ring(knots: &mut Vec<f64>, p: usize, s: usize, coeffs: &mut Vec<RingInterval>, u: f64) {
    // Span k: the last index with knots[k] ≤ u (interior u of a valid
    // clamped vector, so 0 < k < len − 1).
    let mut k = 0;
    for (i, kn) in knots.iter().enumerate() {
        if *kn <= u {
            k = i;
        }
    }
    let n_old = coeffs.len();
    let mut out = Vec::with_capacity(n_old + 1);
    let up = RingInterval::point(u);
    for i in 0..=n_old {
        if i + p <= k {
            // Q_i = c_i (carry below the affected window).
            out.push(coeffs[i]);
        } else if i + s <= k {
            // Window k−p+1 ..= k−s: the ring combination.
            let alpha = (up - RingInterval::point(knots[i]))
                / (RingInterval::point(knots[i + p]) - RingInterval::point(knots[i]));
            out.push(coeffs[i - 1] + (coeffs[i] - coeffs[i - 1]) * alpha);
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
fn to_bezier_spans(kv: &KnotVector, coeffs: &[RingInterval]) -> BernsteinSpans {
    let p = kv.degree();
    let mut knots = kv.knots().to_vec();
    let mut c = coeffs.to_vec();
    let interior = interior_values(kv);
    for (v, m) in &interior {
        for step in *m..p {
            insert_once_ring(&mut knots, p, step, &mut c, *v);
        }
    }
    // Breaks: domain ends plus the distinct interior values (ascending).
    let (lo, hi) = kv.domain();
    let mut breaks = Vec::with_capacity(interior.len() + 2);
    breaks.push(lo);
    breaks.extend(interior.iter().map(|(v, _)| *v));
    breaks.push(hi);
    // Full multiplicity everywhere: control count = nseg·p + 1; span j
    // owns coefficients j·p ..= j·p + p.
    let nseg = breaks.len() - 1;
    let spans = (0..nseg)
        .map(|j| c[j * p..=j * p + p].to_vec())
        .collect();
    BernsteinSpans {
        degree: p,
        breaks,
        spans,
    }
}

// ---------------------------------------------------------------------
// Bernstein algebra (module docs step 3) — fixed association orders
// ---------------------------------------------------------------------

/// Binomial row `C(n, 0..=n)` at `f64` — exact for every degree this
/// module can produce from kernel-sized splines (`C(n, k) < 2⁵³` holds
/// through n = 56; composite degrees are ≤ 4·p).
fn binom_row(n: usize) -> Vec<f64> {
    let mut row = vec![1.0f64; n + 1];
    for k in 1..=n {
        row[k] = row[k - 1] * ((n - k + 1) as f64) / (k as f64);
    }
    row
}

/// Bernstein product of two coefficient rows (degrees from lengths):
/// `(ab)_k = Σ_{i+j=k} [C(da,i)·C(db,j)/C(da+db,k)] · a_i·b_j`, the
/// binomial weight formed as a ring quotient. Fixed association (D9):
/// ascending `k`, ascending `i`, terms exactly as written
/// (`acc = acc + a_i·b_j·w`).
fn bern_mul_row(a: &[RingInterval], b: &[RingInterval]) -> Vec<RingInterval> {
    let da = a.len() - 1;
    let db = b.len() - 1;
    let bin_a = binom_row(da);
    let bin_b = binom_row(db);
    let bin_ab = binom_row(da + db);
    let mut out = Vec::with_capacity(da + db + 1);
    for (k, bk) in bin_ab.iter().enumerate() {
        let mut acc = RingInterval::zero();
        for (i, ai) in a.iter().enumerate() {
            let Some(j) = k.checked_sub(i) else { continue };
            if j > db {
                continue;
            }
            let w = RingInterval::point(bin_a[i] * bin_b[j]) / RingInterval::point(*bk);
            acc = acc + *ai * b[j] * w;
        }
        out.push(acc);
    }
    out
}

/// A structurally-poisoned channel: the shape mismatch outcome for the
/// span-wise combinators (never produced by the entry-point builders,
/// which share one decomposition structure; total anyway, D4).
fn poison_like(a: &BernsteinSpans, degree: usize) -> BernsteinSpans {
    BernsteinSpans {
        degree,
        breaks: a.breaks.clone(),
        spans: a
            .spans
            .iter()
            .map(|_| vec![RingInterval::poison(); degree + 1])
            .collect(),
    }
}

/// Span-wise sum (equal degrees and span counts required; mismatch
/// poisons). Ascending span, ascending coefficient (D9).
fn ch_add(a: &BernsteinSpans, b: &BernsteinSpans) -> BernsteinSpans {
    if a.degree != b.degree || a.spans.len() != b.spans.len() {
        return poison_like(a, a.degree);
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
        return poison_like(a, a.degree);
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
        return poison_like(a, degree);
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
fn ch_scale_left(c: RingInterval, a: &BernsteinSpans) -> BernsteinSpans {
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
fn ch_scale_right(a: &BernsteinSpans, c: RingInterval) -> BernsteinSpans {
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
            .map(|_| vec![RingInterval::zero(); degree + 1])
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
    /// The ring refuses a zero-touching divisor, so a degenerate
    /// denominator yields a poisoned (NaN-bracket) entry — fail-loud.
    pub fn span_bounds(&self) -> Vec<RingInterval> {
        self.num
            .span_hulls()
            .into_iter()
            .zip(self.den.span_hulls())
            .map(|(n, d)| n / d)
            .collect()
    }

    /// The whole-domain enclosure: the hull of [`Self::span_bounds`]
    /// (fixed ascending fold, D9). Poison if any span poisons.
    pub fn bound(&self) -> RingInterval {
        let mut acc = RingInterval::poison();
        for (n, b) in self.span_bounds().into_iter().enumerate() {
            acc = if n == 0 { b } else { RingInterval::hull(acc, b) };
        }
        acc
    }

    /// A certified upper bound on `|f ∘ C|` over the whole domain —
    /// C2.2's sup-norm honesty limb as one number. `NaN` on every
    /// poison path (fails `≤ ε` in every direction, D4 ¶2).
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
    /// (meters²; the `|a|²` normalization is exact in the ring).
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

/// `Σ_d a_d²` as a ring enclosure (ascending `d`, `acc + p·p`).
fn axis_norm2(axis: &[f64; 3]) -> RingInterval {
    let mut acc = RingInterval::zero();
    for a in axis {
        let p = RingInterval::point(*a);
        acc = acc + p * p;
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
        acc = ch_add(&acc, &ch_scale_right(gd, RingInterval::point(*cd)));
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
/// structure errors were caught at [`CurveRingData::new`]).
pub fn implicit_composite(
    data: &CurveRingData<'_>,
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
            let r = RingInterval::point(*radius);
            let r2 = r * r;
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
            let r = RingInterval::point(*radius);
            let r2 = r * r;
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
            let tg = RingInterval::point(*tan_half_angle);
            let k = RingInterval::one() + tg * tg;
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
            let rr = RingInterval::point(*major_radius);
            let rm = RingInterval::point(*minor_radius);
            let c1 = rr * rr - rm * rm; // R² − r²
            let c4 = RingInterval::point(4.0) * (rr * rr); // 4R²
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
    data: &CurveRingData<'_>,
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
    data: &CurveRingData<'_>,
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
        acc = ch_add(&acc, &ch_scale_right(&ad, RingInterval::point(*cd)));
    }
    acc = ch_add(&acc, &ch_scale_right(&w, RingInterval::point(offset)));
    Ok(CompositeForm { num: acc, den: w })
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;
    use crate::spline::basis;

    /// f64 rational-curve oracle: `x_d(t)` via A2.2 basis values —
    /// independent of every ring/decomposition code path.
    fn rat_eval(kv: &KnotVector, w: &[f64], coords: &[Vec<f64>], t: f64) -> Vec<f64> {
        let span = kv.find_span(t);
        let p = kv.degree();
        let n = basis::basis_funs(kv, span, t);
        let mut den = 0.0;
        let mut num = vec![0.0; coords.len()];
        for (j, nj) in n.iter().enumerate() {
            let i = span - p + j;
            let cw = nj * w[i];
            den += cw;
            for (d, ch) in coords.iter().enumerate() {
                num[d] += cw * ch[i];
            }
        }
        num.iter().map(|v| v / den).collect()
    }

    fn lift(coords: &[Vec<f64>]) -> Vec<Vec<RingInterval>> {
        coords
            .iter()
            .map(|ch| ch.iter().map(|x| RingInterval::point(*x)).collect())
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
        assert!(b.is_finite(), "poisoned bound");
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
        let data = CurveRingData::new(&kv, &w, &ring).unwrap();
        let cone = ImplicitSurface::Cone {
            apex: [0.0, 0.0, 0.0],
            axis: [0.0, 0.0, 1.0],
            tan_half_angle: tg,
        };
        let form = implicit_composite(&data, &cone).unwrap();
        let worst = assert_sound(&form, &kv, &w, &coords, |p| {
            let q2 = p[0] * p[0] + p[1] * p[1] + p[2] * p[2];
            q2 - (1.0 + tg * tg) * p[2] * p[2]
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
        let data = CurveRingData::new(&kv, &w, &ring).unwrap();
        // Non-unit axis on purpose: the |a|² normalization is exact in
        // the ring, so the bound is still the meters² residual.
        let cyl = ImplicitSurface::Cylinder {
            point: [0.0, 0.0, 1.0],
            axis: [0.0, 0.0, 3.0],
            radius: r,
        };
        let form = implicit_composite(&data, &cyl).unwrap();
        assert_sound(&form, &kv, &w, &coords, |p| {
            p[0] * p[0] + p[1] * p[1] - r * r
        });
        assert!(form.sup_bound() < 1e-13, "bound {:e}", form.sup_bound());
    }

    /// The §7.3 rational-quadratic full circle of radius `rad` in the
    /// xy-plane about the origin (exact-on-locus in ℝ).
    fn circle_fixture(rad: f64) -> (KnotVector, Vec<f64>, Vec<Vec<f64>>) {
        let s = core::f64::consts::FRAC_1_SQRT_2;
        let kv = KnotVector::clamped(
            vec![0.0, 0.0, 0.0, 0.25, 0.25, 0.5, 0.5, 0.75, 0.75, 1.0, 1.0, 1.0],
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
        let data = CurveRingData::new(&kv, &w, &ring).unwrap();
        let torus = ImplicitSurface::Torus {
            center: [0.0, 0.0, 0.0],
            axis: [0.0, 0.0, 2.0], // non-unit on purpose
            major_radius: big_r,
            minor_radius: small_r,
        };
        let form = implicit_composite(&data, &torus).unwrap();
        assert_sound(&form, &kv, &w, &coords, |p| {
            let q2 = p[0] * p[0] + p[1] * p[1] + p[2] * p[2];
            let c = q2 + big_r * big_r - small_r * small_r;
            c * c - 4.0 * big_r * big_r * (q2 - p[2] * p[2])
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
        let coords = vec![
            vec![0.0, 0.7, 1.3, 2.0],
            vec![1.0, -0.4, 0.9, -1.5],
        ];
        let w = vec![1.0, 0.8, 1.3, 1.0];
        let ring = lift(&coords);
        let data = CurveRingData::new(&kv, &w, &ring).unwrap();

        let prod = coordinate_product(&data, 0, 1).unwrap();
        assert_sound(&prod, &kv, &w, &coords, |p| p[0] * p[1]);

        let lin = linear_composite(&data, &[2.0, -1.0], 3.0).unwrap();
        assert_sound(&lin, &kv, &w, &coords, |p| 2.0 * p[0] - p[1] + 3.0);

        // Tightness sanity: the linear bound is within the coefficient
        // hull's honest reach (not vacuously wide).
        let worst = assert_sound(&lin, &kv, &w, &coords, |p| 2.0 * p[0] - p[1] + 3.0);
        assert!(lin.sup_bound() <= 4.0 * worst.max(1.0));
    }

    #[test]
    fn typed_refusals_and_poison_paths() {
        let kv = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
        let coords2 = lift(&[vec![0.0, 1.0], vec![0.0, 1.0]]);
        let w = vec![1.0, 1.0];
        let data2 = CurveRingData::new(&kv, &w, &coords2).unwrap();
        // Implicit surfaces need 3 channels.
        let sphere = ImplicitSurface::Sphere {
            center: [0.0; 3],
            radius: 1.0,
        };
        assert!(matches!(
            implicit_composite(&data2, &sphere),
            Err(ComposeError::DimensionMismatch { dims: 2, expected: 3 })
        ));
        assert!(matches!(
            coordinate_product(&data2, 0, 2),
            Err(ComposeError::ChannelOutOfRange { channel: 2, dims: 2 })
        ));
        assert!(matches!(
            linear_composite(&data2, &[1.0], 0.0),
            Err(ComposeError::DimensionMismatch { .. })
        ));
        // Bad weights are typed at construction.
        assert!(matches!(
            CurveRingData::new(&kv, &[1.0, -1.0], &coords2),
            Err(ComposeError::Structure(SplineError::NonPositiveWeight { .. }))
        ));
        // A zero axis reaches the denominator as a zero-touching
        // divisor: the ring refuses, the bound poisons (NaN), and NaN
        // fails every ≤ ε certification (D4 ¶2).
        let coords3 = lift(&[vec![0.0, 1.0], vec![0.0, 1.0], vec![0.0, 1.0]]);
        let data3 = CurveRingData::new(&kv, &w, &coords3).unwrap();
        let cyl = ImplicitSurface::Cylinder {
            point: [0.0; 3],
            axis: [0.0; 3],
            radius: 1.0,
        };
        let form = implicit_composite(&data3, &cyl).unwrap();
        assert!(form.sup_bound().is_nan());
    }

    #[test]
    fn bit_replay_composites_are_deterministic() {
        let (kv, w, coords) = circle_fixture(2.5);
        let ring = lift(&coords);
        let data = CurveRingData::new(&kv, &w, &ring).unwrap();
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
