//! **Tensor-product Bernstein composition** (M5 PR 7b): the surface
//! side of the compose contract — given a tensor-product NURBS
//! surface's structure and ring-lifted control net, a 2-D parameter
//! curve `P(t) = (u(t), v(t))` and a 3-D carrier `C(t)` on **one shared
//! parameter** (the OQ4 identity), build the per-coordinate composite
//! residual `S(P(t)) − C(t)` in ring-coefficient Bernstein form and
//! read a certified sup-norm bound off its coefficient hulls. Data in,
//! bounds out — nothing here evaluates or samples anything (C2.2, OQ2).
//!
//! # The pipeline (the curve module's, one dimension up)
//!
//! 1. **Center-shift, then homogeneous lift.** Every spatial channel is
//!    shifted by one common center **before any product** (the curve
//!    module's step 1, one dimension up): the surface as
//!    `(w·(x−c_x), w·(y−c_y), w·(z−c_z), w)` on the control net, the
//!    carrier as the same-shifted `(A_x, A_y, A_z, W_C)`; the parameter
//!    curve `(U, V, W_P) = (w·u, w·v, w)` is chart-valued and carries
//!    no center. The residual is shift-invariant in exact arithmetic,
//!    but the ring's outward rounding scales with coefficient
//!    magnitude — unshifted, a wall 1e6 m from the origin costs six
//!    orders of bound (the PR 7b review's executed witness). No
//!    division happens until the very last per-span quotient, so every
//!    intermediate is polynomial and the ring's products stay
//!    exact-up-to-rounding.
//! 2. **Bézier decomposition, tensor-product.** The surface is
//!    decomposed by knot insertion **in u and in v** — the tensor
//!    product of the two univariate decompositions: structure
//!    (positions, counts) from the `f64` knot vectors, the insertion
//!    `α` formed as a **ring quotient of knot enclosures** in both
//!    directions (an `f64`-rounded `α` would silently drop its rounding
//!    error). The two curves are decomposed on a **merged break list**
//!    (each other's interior knots plus any caller-supplied refinement
//!    breaks injected via exact knot insertion), so a span index means
//!    the same parameter window on every channel by construction.
//! 3. **Composition.** Per `t`-span and per surface Bézier cell
//!    `[u_a,u_b]×[v_a,v_b]`, the cell's Bernstein basis evaluated along
//!    the curve clears denominators exactly:
//!    `W_P^{m_u}·B_i^{m_u}(ũ(t)) = C(m_u,i)·G_u^i·H_u^{m_u−i} / (u_b−u_a)^{m_u}`
//!    with `G_u = U − u_a·W_P`, `H_u = u_b·W_P − U` — Bernstein rows of
//!    the shared span, powered and multiplied by [`super::bern_mul_row`]'s
//!    exact binomial-quotient products. The composed channels
//!    `N_c = Σ_{ij} F_{c,ij}·C(m_u,i)C(m_v,j)·G_u^iH_u^{m_u−i}·G_v^jH_v^{m_v−j}`
//!    (one per homogeneous surface channel `c`) then satisfy
//!    `N_c = s·W_P^{m_u+m_v}·(F_c ∘ P)` with one **common** positive
//!    factor `s·W_P^{m_u+m_v}` — common, so it cancels in every
//!    quotient below and is never formed.
//! 4. **The difference at the coefficient level, then hulls.** Per
//!    coordinate `d`: `num_d = N_d·W_C − A_d·N_w`, `den = N_w·W_C`, and
//!    `num_d/den = S(P(t))_d − C_d(t)` **exactly** (the common factor
//!    divides out). Per-span coefficient hulls of numerator and
//!    denominator, the rational quotient per span (the ring refuses a
//!    zero-touching divisor, so a degenerate denominator poisons
//!    loudly), hulled across spans.
//!
//! # Why the cancellation survives composition-then-hull
//!
//! The reviewable content of `S(P(t)) = C(t)` is a **cancellation**:
//! over one span both `S(P(t))` and `C(t)` move by the span's whole
//! geometric variation (~`‖derivative‖ × span width`), while their
//! difference stays at the fit residual, many orders smaller.
//! Hull-then-difference — enclosing `S∘P` and `C` separately and
//! subtracting the enclosures (PR 7's first-order limb, and any
//! midpoint-plus-radii variant) — necessarily reports the *sum of the
//! two variations*, because interval subtraction cannot see that the
//! two enclosures are correlated: the bound scales like the span
//! width (~1e-2 m on the M5 wall fixture) no matter how small the true
//! residual (~1e-10 m) is. Composition-then-hull instead forms the
//! coefficients of the **single polynomial** `num_d = N_d·W_C − A_d·N_w`
//! in the ring: the large, correlated parts of the two products are the
//! *same numbers* and subtract to ring rounding, so the surviving
//! coefficients are the residual polynomial's own — small because the
//! residual is small — and the convexity fact (a Bernstein polynomial
//! lies in the hull of its coefficients) turns them into a sup bound at
//! the residual's own scale. The only losses are outward rounding
//! (~1e-16 relative, accumulated over the composition's few hundred
//! ring ops) and the Bernstein hull's own overshoot, which shrinks
//! quadratically as the caller injects refinement breaks.
//!
//! # Degree and size budget
//!
//! With curve degrees `q` (both curves, the OQ4 pair share one fit
//! degree) and surface degrees `(m_u, m_v)`, the composite numerator
//! degree is `q·(m_u + m_v) + q`. Every Bernstein product routes
//! through [`super::binom_row`], whose exactness cap
//! ([`super::BINOM_EXACT_MAX`] = 54) therefore bounds what this module
//! serves: `q·(m_u + m_v + 1) ≤ 54`. For the SSI fit degrees actually
//! in use (`SSI_FIT_DEGREE` = 3 for carrier and pcurve) that is
//! `m_u + m_v ≤ 17` — a bicubic×bicubic wall lands at degree 21 of the
//! 54 budget, and even a degree (9,8) surface fits. Beyond the cap the
//! binomial row is all-poison and the bound is `NaN`, which fails every
//! `≤ ε` certification loudly (D4 ¶2) — never a silently rounded
//! weight. Work scales as
//! `spans × cells-touched × (m_u+1)(m_v+1)` products of rows of length
//! `O(q·(m_u+m_v))`.
//!
//! # Scaling conventions
//!
//! The composite is the **coordinatewise residual in meters** (the
//! operands are position-valued; no implicit form and no m²/m⁴ channel
//! exists on this side of the table — the curve module's m/m²/m⁴
//! conventions apply to its implicit composites only).
//! [`SurfaceResidual::sup_bound`] folds the three coordinate bounds
//! Euclidean, so it is a certified upper bound on
//! `sup_t |S(P(t)) − C(t)|` in meters.
//!
//! # Domain posture
//!
//! Cells are selected by the parameter curve's per-span **window** (the
//! quotient of its span coefficient hulls — the convex-hull property
//! for positive weights). A window that leaves the surface's knot
//! domain is served by the boundary cell's polynomial extension —
//! exactly the spline's own clamped extension semantics (see the
//! [`super::super`] span contract) — and a window that straddles a knot
//! line is bounded on **every** cell it touches and hulled, which is
//! sound on each cell's own territory and conservative off it.
//!
//! # C6 and the poison posture
//!
//! Structure (knots, degrees, binomials, break merges, cell selection)
//! is `f64`; everything coefficient-valued is [`RingInterval`].
//! Checkable structural errors at the entry point are typed
//! ([`ComposeError`], closed per D3); anything downstream (degenerate
//! weights, budget overrun, zero-touching denominator) poisons the
//! bound, which fails every `≤ ε` comparison (D4 ¶2).

use super::super::knots::{KnotVector, SplineError};
use super::{
    BernsteinSpans, ComposeError, CurveRingData, bern_mul_row, binom_row, to_bezier_spans_extra,
};
use crate::ring_interval::RingInterval;

// ---------------------------------------------------------------------
// Data-in: the surface's structure + ring-lifted control net
// ---------------------------------------------------------------------

/// A tensor-product NURBS surface's structure plus ring-lifted control
/// coordinates — the surface-side data-in shape. `coords[d][i]` is the
/// `d`-th coordinate (`d < 3`) of control point `i` in the **row-major
/// `iu·nv + iv` layout** as a ring enclosure.
#[derive(Clone, Debug)]
pub struct SurfaceRingData<'a> {
    ku: &'a KnotVector,
    kv: &'a KnotVector,
    weights: &'a [f64],
    coords: &'a [Vec<RingInterval>],
}

// `!(w > 0)` is deliberate (NaN-catching): see `algebra::check_weights`.
#[allow(clippy::neg_cmp_op_on_partial_ord)]
impl<'a> SurfaceRingData<'a> {
    /// Validated construction: weight count/positivity/finiteness
    /// against the tensor control count `nu·nv`, exactly three
    /// coordinate channels, and per-channel coordinate counts.
    ///
    /// # Errors
    ///
    /// [`ComposeError`] naming the exact violation.
    pub fn new(
        ku: &'a KnotVector,
        kv: &'a KnotVector,
        weights: &'a [f64],
        coords: &'a [Vec<RingInterval>],
    ) -> Result<Self, ComposeError> {
        let n = ku.control_count() * kv.control_count();
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
        if coords.len() != 3 {
            return Err(ComposeError::DimensionMismatch {
                dims: coords.len(),
                expected: 3,
            });
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
            ku,
            kv,
            weights,
            coords,
        })
    }
}

// ---------------------------------------------------------------------
// Bounds-out: the composite residual's per-span enclosures
// ---------------------------------------------------------------------

/// The composite residual `S(P(t)) − C(t)`, bounded: per shared
/// `t`-span, one certified ring enclosure per coordinate. Where the
/// curve module's [`super::CompositeForm`] is one global rational form,
/// this is per-span data — the surface's Bézier **cell** serving a span
/// changes along the curve, so no single rational form spans the
/// domain; the bounds-out reading is the same.
#[derive(Clone, Debug)]
pub struct SurfaceResidual {
    breaks: Vec<f64>,
    spans: Vec<[RingInterval; 3]>,
}

impl SurfaceResidual {
    /// The shared break parameters: span `j` covers
    /// `[breaks[j], breaks[j+1]]`.
    pub fn breaks(&self) -> &[f64] {
        &self.breaks
    }

    /// Per-span certified enclosures of the residual, `[x, y, z]`.
    pub fn span_bounds(&self) -> &[[RingInterval; 3]] {
        &self.spans
    }

    /// The whole-domain per-coordinate enclosure: the hull of the span
    /// bounds (fixed ascending fold, D9). Poison if any span poisons.
    pub fn bound(&self) -> [RingInterval; 3] {
        let mut acc = [RingInterval::poison(); 3];
        for (n, row) in self.spans.iter().enumerate() {
            for d in 0..3 {
                acc[d] = if n == 0 {
                    row[d]
                } else {
                    RingInterval::hull(acc[d], row[d])
                };
            }
        }
        acc
    }

    /// A certified upper bound on `sup_t |S(P(t)) − C(t)|` in meters —
    /// the three coordinate bounds folded Euclidean (`f64` squares of
    /// nonnegative magnitudes; `NaN` on every poison path, which fails
    /// every `≤ ε` comparison, D4 ¶2).
    pub fn sup_bound(&self) -> f64 {
        let b = self.bound();
        let mut acc = 0.0f64;
        for e in b {
            let m = e.mag();
            acc += m.powi(2);
        }
        acc.sqrt()
    }
}

// ---------------------------------------------------------------------
// Tensor Bézier decomposition (pipeline step 2)
// ---------------------------------------------------------------------

/// One homogeneous surface channel, Bézier-decomposed in both
/// directions: `patches[su][sv]` holds the `(deg_u+1)·(deg_v+1)`
/// cell coefficients, row-major `a·(deg_v+1) + b`.
struct TensorSpans {
    deg_u: usize,
    deg_v: usize,
    breaks_u: Vec<f64>,
    breaks_v: Vec<f64>,
    patches: Vec<Vec<Vec<RingInterval>>>,
}

/// Tensor-product Bézier decomposition of one scalar channel over the
/// control grid (`nu·nv`, row-major): the u-direction univariate
/// decomposition applied per v-column, then the v-direction one per
/// (u-span, u-index) row — the tensor product of the two univariate
/// insertions, `α` a ring quotient in **both** directions.
fn tensor_channel(ku: &KnotVector, kv: &KnotVector, grid: &[RingInterval]) -> TensorSpans {
    let nv = kv.control_count();
    // Stage 1 (u): one decomposition per v-column; identical structure.
    let mut breaks_u = Vec::new();
    let mut deg_u = ku.degree();
    // stage1[su][a][jv]
    let mut stage1: Vec<Vec<Vec<RingInterval>>> = Vec::new();
    for jv in 0..nv {
        let col: Vec<RingInterval> = (0..ku.control_count())
            .map(|iu| grid[iu * nv + jv])
            .collect();
        let bs = to_bezier_spans_extra(ku, &col, &[]);
        if jv == 0 {
            breaks_u = bs.breaks.clone();
            deg_u = bs.degree;
            stage1 = bs
                .spans
                .iter()
                .map(|row| row.iter().map(|c| vec![*c]).collect())
                .collect();
        } else {
            for (su, row) in bs.spans.iter().enumerate() {
                for (a, c) in row.iter().enumerate() {
                    stage1[su][a].push(*c);
                }
            }
        }
    }
    // Stage 2 (v): per u-span and u-index, decompose the v-row.
    let mut breaks_v = Vec::new();
    let mut deg_v = kv.degree();
    let mut patches: Vec<Vec<Vec<RingInterval>>> = Vec::new();
    for span_rows in &stage1 {
        let mut cells: Vec<Vec<RingInterval>> = Vec::new();
        for (a, vrow) in span_rows.iter().enumerate() {
            let bs = to_bezier_spans_extra(kv, vrow, &[]);
            if a == 0 {
                breaks_v = bs.breaks.clone();
                deg_v = bs.degree;
                cells = vec![vec![]; bs.spans.len()];
            }
            for (sv, row) in bs.spans.iter().enumerate() {
                cells[sv].extend(row.iter().copied());
            }
        }
        patches.push(cells);
    }
    TensorSpans {
        deg_u,
        deg_v,
        breaks_u,
        breaks_v,
        patches,
    }
}

// ---------------------------------------------------------------------
// Row algebra (fixed association orders, D9)
// ---------------------------------------------------------------------

/// `a_k − c·b_k` elementwise (equal degrees by construction; the
/// association is exactly as written).
fn row_sub_scaled(a: &[RingInterval], c: f64, b: &[RingInterval]) -> Vec<RingInterval> {
    let cc = RingInterval::point(c);
    a.iter().zip(b.iter()).map(|(x, y)| *x - cc * *y).collect()
}

/// `c·b_k − a_k` elementwise.
fn row_scaled_sub(c: f64, b: &[RingInterval], a: &[RingInterval]) -> Vec<RingInterval> {
    let cc = RingInterval::point(c);
    a.iter().zip(b.iter()).map(|(x, y)| cc * *y - *x).collect()
}

/// `a_k − b_k` elementwise.
fn row_sub(a: &[RingInterval], b: &[RingInterval]) -> Vec<RingInterval> {
    a.iter().zip(b.iter()).map(|(x, y)| *x - *y).collect()
}

/// The coefficient hull of one row (ascending fold, D9).
fn row_hull(row: &[RingInterval]) -> RingInterval {
    let mut acc = RingInterval::poison();
    for (n, c) in row.iter().enumerate() {
        acc = if n == 0 {
            *c
        } else {
            RingInterval::hull(acc, *c)
        };
    }
    acc
}

/// The table `T_i = g^i·h^{m−i}` for `i = 0..=m`, every entry of one
/// degree `m·deg(g)` (ascending powers built by repeated
/// [`bern_mul_row`], fixed order).
fn power_table(g: &[RingInterval], h: &[RingInterval], m: usize) -> Vec<Vec<RingInterval>> {
    let one = vec![RingInterval::one()];
    let mut gp: Vec<Vec<RingInterval>> = vec![one.clone()];
    let mut hp: Vec<Vec<RingInterval>> = vec![one];
    for i in 1..=m {
        gp.push(bern_mul_row(&gp[i - 1], g));
        hp.push(bern_mul_row(&hp[i - 1], h));
    }
    (0..=m).map(|i| bern_mul_row(&gp[i], &hp[m - i])).collect()
}

// ---------------------------------------------------------------------
// Composition (pipeline step 3) and the per-cell bound (step 4)
// ---------------------------------------------------------------------

/// One `t`-span's homogeneous curve rows: the parameter curve's
/// `(U, V, W_P)` and the carrier's `(A_x, A_y, A_z, W_C)`.
struct SpanRows<'a> {
    u: &'a [RingInterval],
    v: &'a [RingInterval],
    wp: &'a [RingInterval],
    ac: [&'a [RingInterval]; 3],
    wc: &'a [RingInterval],
}

/// The residual enclosure of one `t`-span against one surface cell:
/// the module-docs composition, the difference formed at the
/// coefficient level, then hull quotients. `[RingInterval; 3]`, poison
/// entries wherever the ring poisons (budget, zero-touching
/// denominator).
fn cell_residual(
    surf: &[&TensorSpans; 4],
    su: usize,
    sv: usize,
    rows: &SpanRows<'_>,
) -> [RingInterval; 3] {
    let (mu, mv) = (surf[0].deg_u, surf[0].deg_v);
    // The one budget case the automatic row poison cannot reach: a
    // degree-0 curve pair composes to degree-0 rows whose binomials
    // never overflow, while `C(m_u,i)·C(m_v,j)` still could. Poison
    // explicitly rather than round silently.
    if mu + mv > super::BINOM_EXACT_MAX {
        return [RingInterval::poison(); 3];
    }
    let (ua, ub) = (surf[0].breaks_u[su], surf[0].breaks_u[su + 1]);
    let (va, vb) = (surf[0].breaks_v[sv], surf[0].breaks_v[sv + 1]);
    // G/H rows (module docs step 3), then the power tables.
    let gu = row_sub_scaled(rows.u, ua, rows.wp);
    let hu = row_scaled_sub(ub, rows.wp, rows.u);
    let gv = row_sub_scaled(rows.v, va, rows.wp);
    let hv = row_scaled_sub(vb, rows.wp, rows.v);
    let tu = power_table(&gu, &hu, mu);
    let tv = power_table(&gv, &hv, mv);
    let bmu = binom_row(mu);
    let bmv = binom_row(mv);
    // The four composed channels N_c, degree q_p·(m_u+m_v): ascending
    // (i, j), the cell coefficient times its exact binomial pair on the
    // left of the basis row (fixed association).
    let deg_n = tu[0].len() - 1 + tv[0].len() - 1;
    let mut n: [Vec<RingInterval>; 4] =
        core::array::from_fn(|_| vec![RingInterval::zero(); deg_n + 1]);
    for i in 0..=mu {
        for j in 0..=mv {
            let basis = bern_mul_row(&tu[i], &tv[j]);
            let w = RingInterval::point(bmu[i] * bmv[j]);
            for (c, ch) in n.iter_mut().enumerate() {
                let f = surf[c].patches[su][sv][i * (mv + 1) + j];
                let fw = f * w;
                for (k, b) in basis.iter().enumerate() {
                    ch[k] = ch[k] + fw * *b;
                }
            }
        }
    }
    // The difference at the coefficient level (module docs step 4),
    // then hull quotients: num_d/den = S(P(t))_d − C_d(t) exactly.
    //
    // Banked observation (PR 7b review NOTE 2, 2026-07-31): the
    // hull-then-quotient step's looseness grows with the CURVE
    // weights' dynamic range — the numerator and denominator hulls
    // decorrelate when `W_P`/`W_C` swing hard across a span (the
    // review forced a 108× worst case with adversarial weights). SSI
    // fit weights are ≈1 today, so nothing rides on it; revisit
    // (per-span de-rationalization or a joint quotient) when
    // genuinely rational pcurves arrive.
    let den = row_hull(&bern_mul_row(&n[3], rows.wc));
    core::array::from_fn(|d| {
        let num = row_sub(
            &bern_mul_row(&n[d], rows.wc),
            &bern_mul_row(rows.ac[d], &n[3]),
        );
        row_hull(&num) / den
    })
}

// ---------------------------------------------------------------------
// Cell selection (structure, f64) and the entry point
// ---------------------------------------------------------------------

/// The cell indices of the break list `breaks` whose closed interval
/// meets the window `[lo, hi]`, clamped so an out-of-domain window is
/// served by the boundary cell's polynomial extension (module docs,
/// domain posture). Returns the inclusive index range.
///
/// Banked observation (PR 7b review NOTE 1, 2026-07-31): the overlap
/// test is **closed**, so a window whose endpoint lands exactly on a
/// knot pulls in the neighbor cell whose contribution is a
/// single-point agreement (the two patch polynomials agree at the
/// knot). A strict-interior test would be tighter and still sound —
/// a future tightening, banked rather than slipped into a fix pass.
fn cells_touched(breaks: &[f64], lo: f64, hi: f64) -> (usize, usize) {
    let last = breaks.len() - 2;
    let mut first_cell = last;
    let mut last_cell = 0;
    for cell in 0..=last {
        if breaks[cell + 1] >= lo && breaks[cell] <= hi {
            first_cell = first_cell.min(cell);
            last_cell = last_cell.max(cell);
        }
    }
    if first_cell > last_cell {
        // The window misses the domain entirely: the nearest edge cell.
        if hi < breaks[0] { (0, 0) } else { (last, last) }
    } else {
        (first_cell, last_cell)
    }
}

/// The composite residual `S(P(t)) − C(t)` per coordinate, in certified
/// per-span ring enclosures (module docs: the pipeline, the
/// cancellation note, and the degree budget).
///
/// `pcurve` is the 2-channel parameter curve `P(t) = (u(t), v(t))` in
/// the surface's parameter square; `carrier` is the 3-channel curve the
/// composite is compared against, on the **same knot domain** (the OQ4
/// identity — exact `f64` equality, typed refusal otherwise). Both are
/// decomposed onto the merged break list of their interior knots plus
/// `extra_breaks` (exact knot insertion; each extra strictly inside the
/// domain, duplicates structure-filtered), so a caller wanting tighter
/// hulls injects refinement here rather than refitting anything.
///
/// # Errors
///
/// [`ComposeError::DimensionMismatch`] unless `pcurve` has 2 and
/// `carrier` 3 channels; [`ComposeError::DomainMismatch`] when the two
/// curves' knot domains differ.
pub fn surface_curve_residual(
    surface: &SurfaceRingData<'_>,
    pcurve: &CurveRingData<'_>,
    carrier: &CurveRingData<'_>,
    extra_breaks: &[f64],
) -> Result<SurfaceResidual, ComposeError> {
    if pcurve.dims() != 2 {
        return Err(ComposeError::DimensionMismatch {
            dims: pcurve.dims(),
            expected: 2,
        });
    }
    if carrier.dims() != 3 {
        return Err(ComposeError::DimensionMismatch {
            dims: carrier.dims(),
            expected: 3,
        });
    }
    if pcurve.kv.domain() != carrier.kv.domain() {
        return Err(ComposeError::DomainMismatch {
            a: pcurve.kv.domain(),
            b: carrier.kv.domain(),
        });
    }
    // The merged break list: each curve is decomposed with the OTHER's
    // interior knots (plus the caller's refinement) injected, so every
    // channel lands on one shared span structure.
    let mut merged: Vec<f64> = extra_breaks.to_vec();
    merged.extend(pcurve.kv.interior_knots().map(|(v, _)| v));
    merged.extend(carrier.kv.interior_knots().map(|(v, _)| v));
    merged.sort_by(f64::total_cmp);
    merged.dedup();

    // Pipeline step 1: **center-shift before any product** — the curve
    // module's precedent, one dimension up. The residual is
    // shift-invariant in exact arithmetic (a common center cancels in
    // `S(P(t)) − C(t)`), but the RING is not: outward rounding scales
    // with coefficient MAGNITUDE, so an unshifted wall 1e6 m from the
    // origin costs six orders of bound (review finding 2026-07-31,
    // executed witness pinned in `review_m5_pr7b_tensor.rs`:
    // 1.128e-12 near the origin vs 1.866e-6 unshifted at 1e6 m). The
    // center is structure (C6): the carrier's first control
    // coordinate, one exact `f64` per coordinate, applied identically
    // to the surface's and the carrier's spatial channels. The chart
    // channels are parameter-valued and carry no center.
    let center: [f64; 3] = core::array::from_fn(|d| carrier.coords[d][0].lo());

    // Homogeneous channels on the shared breaks, the weight channel
    // carried; spatial channels center-shifted at the lift.
    let homog = |data: &CurveRingData<'_>, d: usize, shift: f64| -> BernsteinSpans {
        let s = RingInterval::point(shift);
        let coeffs: Vec<RingInterval> = data.coords[d]
            .iter()
            .zip(data.weights.iter())
            .map(|(x, w)| RingInterval::point(*w) * (*x - s))
            .collect();
        to_bezier_spans_extra(data.kv, &coeffs, &merged)
    };
    let weight = |data: &CurveRingData<'_>| -> BernsteinSpans {
        let coeffs: Vec<RingInterval> = data
            .weights
            .iter()
            .map(|w| RingInterval::point(*w))
            .collect();
        to_bezier_spans_extra(data.kv, &coeffs, &merged)
    };
    let (pu, pv, pw) = (homog(pcurve, 0, 0.0), homog(pcurve, 1, 0.0), weight(pcurve));
    let (ax, ay, az, cw) = (
        homog(carrier, 0, center[0]),
        homog(carrier, 1, center[1]),
        homog(carrier, 2, center[2]),
        weight(carrier),
    );

    // The surface's four homogeneous channels, tensor-decomposed —
    // spatial channels shifted by the SAME center.
    let lift = |f: &dyn Fn(usize) -> RingInterval| -> Vec<RingInterval> {
        (0..surface.weights.len()).map(f).collect()
    };
    let schan: Vec<TensorSpans> = (0..3)
        .map(|d| {
            let c = RingInterval::point(center[d]);
            let grid =
                lift(&|i| RingInterval::point(surface.weights[i]) * (surface.coords[d][i] - c));
            tensor_channel(surface.ku, surface.kv, &grid)
        })
        .collect();
    let swt = tensor_channel(
        surface.ku,
        surface.kv,
        &lift(&|i| RingInterval::point(surface.weights[i])),
    );
    let surf: [&TensorSpans; 4] = [&schan[0], &schan[1], &schan[2], &swt];

    // Per shared t-span: the parameter window, the cells it touches,
    // and the hull across cells (ascending, D9).
    let breaks = pu.breaks.clone();
    let mut spans: Vec<[RingInterval; 3]> = Vec::with_capacity(breaks.len() - 1);
    for s in 0..breaks.len() - 1 {
        let rows = SpanRows {
            u: &pu.spans[s],
            v: &pv.spans[s],
            wp: &pw.spans[s],
            ac: [&ax.spans[s], &ay.spans[s], &az.spans[s]],
            wc: &cw.spans[s],
        };
        let wden = row_hull(rows.wp);
        let wu = row_hull(rows.u) / wden;
        let wv = row_hull(rows.v) / wden;
        if wu.is_poison() || wv.is_poison() {
            spans.push([RingInterval::poison(); 3]);
            continue;
        }
        let (u0, u1) = cells_touched(&swt.breaks_u, wu.lo(), wu.hi());
        let (v0, v1) = cells_touched(&swt.breaks_v, wv.lo(), wv.hi());
        let mut acc: Option<[RingInterval; 3]> = None;
        for su in u0..=u1 {
            for sv in v0..=v1 {
                let b = cell_residual(&surf, su, sv, &rows);
                acc = Some(match acc {
                    None => b,
                    Some(prev) => core::array::from_fn(|d| RingInterval::hull(prev[d], b[d])),
                });
            }
        }
        // `cells_touched` always returns at least one cell.
        spans.push(acc.unwrap_or([RingInterval::poison(); 3]));
    }
    Ok(SurfaceResidual { breaks, spans })
}
