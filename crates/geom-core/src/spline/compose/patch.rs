//! **Tensor-product Bernstein patch algebra** — the two-parameter
//! sibling of [`super`]'s curve composites.
//!
//! Where [`super::tensor`] composes a surface with a *curve* on one
//! shared parameter, this module keeps both surface parameters and
//! serves the shape a **surface-against-surface** residual needs: a
//! tensor-product scalar channel in per-cell Bernstein coefficient
//! form, closed under `+`, `−`, `×`, and degree elevation, with a
//! certified per-cell hull read off the coefficients.
//!
//! # Why coefficient algebra rather than enclosure algebra
//!
//! Two surfaces that agree to `ε` still each move by their whole
//! geometric variation across a cell. Enclosing them separately and
//! subtracting the enclosures reports the SUM of those variations —
//! a bound that scales like the cell, no matter how small the true
//! residual is (the retirement recorded in [`super::tensor`]'s docs,
//! at one parameter). Forming the difference's **coefficients**
//! instead makes the large correlated parts the same numbers, which
//! subtract to outward rounding; what survives is the residual
//! polynomial's own coefficient net, and the convexity fact (a
//! Bernstein polynomial lies in the hull of its coefficients, in each
//! parameter, hence in the tensor hull) turns it into a sup bound at
//! the residual's own scale.
//!
//! That is the whole reason this module exists: the offset residual
//! `S_fit − (S + d·n)` cannot be formed as one polynomial (the unit
//! normal's square root is exactly why an offset is not a NURBS), but
//! its **rationalized parts** can — and each of those is a product of
//! tensor-product splines, which is what is built here.
//!
//! # Alignment is the caller's, and it is structural
//!
//! Every operand must be decomposed on ONE break list per direction
//! ([`PatchSpans::decompose`] takes extra breaks for exactly that,
//! and knot insertion is exact in ℝ). Two patches whose break lists
//! disagree combine to a **refusal**, never to a silently misaligned
//! answer.
//!
//! # Degree budget
//!
//! Every product routes through `super::bern_mul_row_into` (and
//! `super::bern_mul_row_with` for degree elevation) over the weight
//! tables `super::bern_weights` serves, whose exactness cap
//! (`BINOM_EXACT_MAX`) bounds the per-direction degree a product may
//! reach; beyond it the binomial row is all-NaN and every hull is
//! `NaN`, which fails every `≤ ε` certification loudly (D4 ¶2). Work
//! per cell scales as `(a+1)(c+1)` row products of length `O(b + d)`.
//!
//! Those tables are functions of the degrees alone, so each entry
//! point looks them up ONCE for the whole patch and hands them down
//! to the per-cell loop; the table carries the degree pair it was
//! built for, so a table reaching the wrong direction announces
//! itself rather than answering.
//!
//! # C6 and refusal
//!
//! Structure (knots, degrees, break merges, cell counts) is `f64`;
//! every coefficient is a [`Interval`]. Nothing here evaluates or
//! samples anything.

use super::super::net::TensorCoeffs;
use super::{
    BernWeights, bern_mul_row_into, bern_mul_row_with, bern_weights, to_bezier_spans_extra,
};
use crate::interval::Interval;
use crate::interval::certification::Certification;
use std::borrow::Cow;

/// One scalar channel of a tensor-product spline in per-cell Bernstein
/// form: `cell(su, sv)` holds `(deg_u + 1)·(deg_v + 1)` ring
/// coefficients, row-major in `u`, of the polynomial on
/// `[breaks_u[su], breaks_u[su+1]] × [breaks_v[sv], breaks_v[sv+1]]`.
#[derive(Clone, Debug)]
pub struct PatchSpans {
    deg_u: usize,
    deg_v: usize,
    breaks_u: Vec<f64>,
    breaks_v: Vec<f64>,
    cells: Vec<Vec<Vec<Interval>>>,
}

impl PatchSpans {
    /// The Bernstein bidegree shared by every cell.
    pub fn degree(&self) -> (usize, usize) {
        (self.deg_u, self.deg_v)
    }

    /// The break parameters per direction: cell `(su, sv)` covers
    /// `[breaks_u[su], breaks_u[su+1]] × [breaks_v[sv], breaks_v[sv+1]]`.
    pub fn breaks(&self) -> (&[f64], &[f64]) {
        (&self.breaks_u, &self.breaks_v)
    }

    /// The cell counts `(nu, nv)`.
    pub fn cell_counts(&self) -> (usize, usize) {
        (self.cells.len(), self.cells.first().map_or(0, Vec::len))
    }

    /// The certified enclosure of the channel's values on cell
    /// `(su, sv)` — the hull of its Bernstein coefficients (module
    /// docs). Refused for an out-of-range cell. Fixed ascending fold
    /// order (D9).
    pub fn cell_hull(&self, su: usize, sv: usize) -> Interval {
        let Some(block) = self.cells.get(su).and_then(|r| r.get(sv)) else {
            return Interval::refused();
        };
        let mut acc = Interval::refused();
        for (n, c) in block.iter().enumerate() {
            acc = if n == 0 { *c } else { Interval::hull(acc, *c) };
        }
        acc
    }

    /// A certified upper bound on `‖(c₀, c₁, c₂)‖` over cell `(su, sv)`
    /// for the VECTOR form whose three coordinates are `channels`: the
    /// largest norm of one vector coefficient, which bounds the norm
    /// because the norm is convex and the form lies in the hull of its
    /// coefficients. It is a function of the coefficient vectors, so a
    /// rotation of the coordinates moves it only by rounding, where
    /// three [`Self::cell_hull`]s folded into one norm read a box that
    /// can be `√3`× the vector's own reach.
    ///
    /// [`super::tensor::coefficient_norm_sup`] on the cell's
    /// coefficients. `NaN` when the channels do not share one cell
    /// structure and bidegree, or the cell is out of range.
    pub fn cell_norm_sup(channels: [&Self; 3], su: usize, sv: usize) -> f64 {
        let [a, b, c] = channels;
        if !a.aligned(b) || !a.aligned(c) || a.degree() != b.degree() || a.degree() != c.degree() {
            return f64::NAN;
        }
        let (Some(na), Some(nb), Some(nc)) = (a.block(su, sv), b.block(su, sv), c.block(su, sv))
        else {
            return f64::NAN;
        };
        super::tensor::coefficient_norm_sup([na, nb, nc])
    }

    /// A certified LOWER bound on `‖(c₀, c₁, c₂)‖` over cell `(su, sv)`
    /// for the VECTOR form whose three coordinates are `channels` — the
    /// inf-side twin of [`Self::cell_norm_sup`], read from the coefficient
    /// vectors themselves rather than from three hulls folded into a norm.
    ///
    /// The form lies in the convex hull of its coefficient vectors `cₖ`,
    /// so for any direction `n ≠ 0` its norm is at least
    /// `minₖ (cₖ·n)/‖n‖`, read here over the coefficient enclosures with
    /// certified arithmetic. The direction is a CHOICE (any finite one is
    /// sound), and the largest reading over two kinds is kept: the mean of
    /// the coefficient vectors' midpoints, which already reads at least the ball
    /// `‖c̄‖ − maxₖ ‖cₖ − c̄‖` about it, and the iterates of two descents
    /// toward the hull's point nearest the origin on the midpoints: Wolfe's
    /// min-norm-point algorithm, whose last iterate is that point where its
    /// affine systems are well posed, and Frank–Wolfe, which carries the
    /// reading where rounding leaves them singular. Along it the reading is the hull's
    /// distance from the origin, which no per-coordinate box's mignitude
    /// exceeds; the row `the_hull_floor_is_never_above_a_hull_point`
    /// measures the reading against sampled hull points and against the
    /// box. Every step commutes with a rotation, so the bound is a function
    /// of the coefficient vectors and a rotation moves it only by rounding
    /// (DESIGN `[tolerance.residuals]`). `0.0` where no direction proves
    /// the hull clear of the origin; `NaN` as [`Self::cell_norm_sup`] is,
    /// and where a product along a direction does not certify.
    pub fn cell_norm_inf(channels: [&Self; 3], su: usize, sv: usize) -> f64 {
        use crate::real::Bounds as _;
        let [a, b, c] = channels;
        if !a.aligned(b) || !a.aligned(c) || a.degree() != b.degree() || a.degree() != c.degree() {
            return f64::NAN;
        }
        let (Some(na), Some(nb), Some(nc)) = (a.block(su, sv), b.block(su, sv), c.block(su, sv))
        else {
            return f64::NAN;
        };
        let blocks = [na, nb, nc];
        if na.is_empty()
            || blocks
                .iter()
                .flat_map(|b| b.iter())
                .any(|x| !x.is_certified())
        {
            return f64::NAN;
        }
        let mids: Vec<[f64; 3]> = (0..na.len())
            .map(|k| {
                core::array::from_fn(|i| crate::mid_param(blocks[i][k].lo(), blocks[i][k].hi()))
            })
            .collect();
        let dot = |p: &[f64; 3], q: &[f64; 3]| p[0] * q[0] + p[1] * q[1] + p[2] * q[2];
        // The certified reading along `n`: `minₖ (cₖ·n)` over the
        // coefficient enclosures, over `‖n‖` from above.
        let along = |n: &[f64; 3]| -> f64 {
            let n_iv = n.map(Interval::point);
            let len = (n_iv[0].sqr() + n_iv[1].sqr() + n_iv[2].sqr()).sqrt();
            let mut lo = f64::INFINITY;
            for ((x, y), z) in blocks[0].iter().zip(blocks[1]).zip(blocks[2]) {
                let p = *x * n_iv[0] + *y * n_iv[1] + *z * n_iv[2];
                if !p.is_certified() {
                    // A product that does not certify reads nothing: the
                    // whole reading refuses rather than skip it.
                    return f64::NAN;
                }
                lo = lo.min(p.lo());
            }
            if !len.is_certified() {
                return f64::NAN;
            }
            let hi = len.hi();
            if lo > 0.0 && hi > 0.0 && hi.is_finite() {
                crate::interval::div_down(lo, hi)
            } else {
                0.0
            }
        };
        // The reading along the mean of the coefficient vectors' midpoints
        // first: it already
        // reads at least the ball about that centre.
        #[allow(clippy::cast_precision_loss)]
        let count = mids.len() as f64;
        let mean: [f64; 3] =
            core::array::from_fn(|i| mids.iter().map(|m| m[i]).sum::<f64>() / count);
        let mut best = 0.0_f64;
        let mut read = |n: &[f64; 3]| -> bool {
            if !n.iter().all(|v| v.is_finite()) || dot(n, n) == 0.0 {
                return true;
            }
            let r = along(n);
            if r.is_nan() {
                return false;
            }
            best = best.max(r);
            true
        };
        if !read(&mean) {
            return f64::NAN;
        }
        // Then the hull's point nearest the origin, two ways, each read at
        // every iterate: Wolfe's min-norm-point algorithm, exact where its
        // corral's affine systems are well posed, and a Frank–Wolfe
        // descent, which needs none and so carries the reading where
        // rounding leaves a near-flat corral singular.
        for x in min_norm_point_iterates(&mids)
            .into_iter()
            .chain(frank_wolfe_iterates(&mids, mean))
        {
            if !read(&x) {
                return f64::NAN;
            }
        }
        best
    }

    /// Cell `(su, sv)`'s coefficients, row-major in `u`, `None` out of
    /// range: one coefficient's enclosure, which [`Self::cell_hull`]
    /// folds into the cell's.
    #[cfg(any(test, feature = "test-support"))]
    pub fn cell_coeffs(&self, su: usize, sv: usize) -> Option<&[Interval]> {
        self.block(su, sv)
    }

    /// Cell `(su, sv)`'s coefficients, `None` out of range.
    fn block(&self, su: usize, sv: usize) -> Option<&[Interval]> {
        self.cells
            .get(su)
            .and_then(|r| r.get(sv))
            .map(Vec::as_slice)
    }

    /// Tensor-product Bézier decomposition of one scalar channel of a
    /// spline, its control grid paired with both knot vectors, with
    /// `extra_u`/`extra_v` break parameters injected in each direction
    /// so several channels land on one shared break list (module docs:
    /// alignment).
    ///
    /// Structure-filtered, never an error: an extra outside the open
    /// domain or duplicating a knot is dropped.
    pub fn decompose(grid: &TensorCoeffs<'_>, extra_u: &[f64], extra_v: &[f64]) -> Self {
        let (ku, kv, net) = (grid.knots_u(), grid.knots_v(), grid.net());
        // Stage 1 (u): one univariate decomposition per v-column;
        // identical structure across columns by construction.
        let mut breaks_u = Vec::new();
        let mut deg_u = ku.degree();
        // stage1[su][a][jv]
        let mut stage1: Vec<Vec<Vec<Interval>>> = Vec::new();
        for jv in 0..kv.control_count() {
            let bs = ku.with_coeffs_from_fn(
                |iu| net.get(iu, jv),
                |pair| to_bezier_spans_extra(pair, extra_u),
            );
            if jv == 0 {
                breaks_u = bs.breaks().to_vec();
                deg_u = bs.degree();
                stage1 = bs
                    .spans()
                    .iter()
                    .map(|row| row.iter().map(|c| vec![*c]).collect())
                    .collect();
            } else {
                for (su, row) in bs.spans().iter().enumerate() {
                    for (a, c) in row.iter().enumerate() {
                        stage1[su][a].push(*c);
                    }
                }
            }
        }
        // Stage 2 (v): per u-cell and u-index, decompose the v-row.
        let mut breaks_v = Vec::new();
        let mut deg_v = kv.degree();
        let mut cells: Vec<Vec<Vec<Interval>>> = Vec::new();
        for span_rows in &stage1 {
            let mut row_cells: Vec<Vec<Interval>> = Vec::new();
            for (a, vrow) in span_rows.iter().enumerate() {
                // `vrow` holds one entry per v-column: `kv`'s control count.
                let bs = kv.with_coeffs_from_fn(
                    |jv| vrow[jv],
                    |pair| to_bezier_spans_extra(pair, extra_v),
                );
                if a == 0 {
                    breaks_v = bs.breaks().to_vec();
                    deg_v = bs.degree();
                    row_cells = vec![vec![]; bs.spans().len()];
                }
                for (sv, row) in bs.spans().iter().enumerate() {
                    row_cells[sv].extend(row.iter().copied());
                }
            }
            cells.push(row_cells);
        }
        Self {
            deg_u,
            deg_v,
            breaks_u,
            breaks_v,
            cells,
        }
    }

    /// A constant channel on this patch's cell structure — the
    /// degree-`(0, 0)` form of `c`, used both as a literal and as the
    /// degree-elevation multiplicand ([`Self::elevated`]).
    pub fn constant(&self, c: Interval) -> Self {
        Self {
            deg_u: 0,
            deg_v: 0,
            breaks_u: self.breaks_u.clone(),
            breaks_v: self.breaks_v.clone(),
            cells: self
                .cells
                .iter()
                .map(|row| row.iter().map(|_| vec![c]).collect())
                .collect(),
        }
    }

    /// A structurally refused channel of the given bidegree with no
    /// cells — the mismatch outcome of every combinator (total, D4).
    fn refused(deg_u: usize, deg_v: usize) -> Self {
        Self {
            deg_u,
            deg_v,
            breaks_u: Vec::new(),
            breaks_v: Vec::new(),
            cells: Vec::new(),
        }
    }

    /// Whether two channels share a cell structure (same breaks, same
    /// counts) — the precondition of every combinator.
    fn aligned(&self, other: &Self) -> bool {
        !self.cells.is_empty()
            && self.breaks_u == other.breaks_u
            && self.breaks_v == other.breaks_v
            && self.cell_counts() == other.cell_counts()
    }

    /// This channel raised to bidegree `(du, dv)` — Bernstein degree
    /// elevation as multiplication by the constant `1` at the missing
    /// degree, which is exactly what `super::bern_mul_row_with`'s
    /// binomial-quotient product computes (and is exact in ℝ).
    /// Returns `self` unchanged when it is already there; refused when
    /// asked to LOWER a degree.
    pub fn elevated(&self, du: usize, dv: usize) -> Self {
        if du < self.deg_u || dv < self.deg_v {
            return Self::refused(du, dv);
        }
        if du == self.deg_u && dv == self.deg_v {
            return self.clone();
        }
        // The multipliers are functions of the degrees alone, so one
        // set of them serves every cell of this elevation — the same
        // lever `mul` pulls one altitude up.
        let pads = ElevationPads::new((self.deg_u, self.deg_v), (du - self.deg_u, dv - self.deg_v));
        let cells = self
            .cells
            .iter()
            .map(|row| {
                row.iter()
                    .map(|block| elevate_block(block, self.deg_u, self.deg_v, &pads))
                    .collect()
            })
            .collect();
        Self {
            deg_u: du,
            deg_v: dv,
            breaks_u: self.breaks_u.clone(),
            breaks_v: self.breaks_v.clone(),
            cells,
        }
    }

    /// Cellwise sum, both operands first raised to the common
    /// bidegree. Refused on a structure mismatch.
    pub fn add(&self, other: &Self) -> Self {
        self.combine(other, false)
    }

    /// Cellwise difference (`self − other`), both operands first
    /// raised to the common bidegree. Refused on a structure mismatch.
    pub fn sub(&self, other: &Self) -> Self {
        self.combine(other, true)
    }

    fn combine(&self, other: &Self, subtract: bool) -> Self {
        if !self.aligned(other) {
            return Self::refused(self.deg_u.max(other.deg_u), self.deg_v.max(other.deg_v));
        }
        let du = self.deg_u.max(other.deg_u);
        let dv = self.deg_v.max(other.deg_v);
        let a = self.elevated(du, dv);
        let b = other.elevated(du, dv);
        let cells = a
            .cells
            .iter()
            .zip(b.cells.iter())
            .map(|(ra, rb)| {
                ra.iter()
                    .zip(rb.iter())
                    .map(|(ba, bb)| {
                        ba.iter()
                            .zip(bb.iter())
                            .map(|(x, y)| if subtract { *x - *y } else { *x + *y })
                            .collect()
                    })
                    .collect()
            })
            .collect();
        Self {
            deg_u: du,
            deg_v: dv,
            breaks_u: self.breaks_u.clone(),
            breaks_v: self.breaks_v.clone(),
            cells,
        }
    }

    /// Cellwise tensor-product Bernstein product: bidegrees add, and
    /// the coefficients are the separable double convolution with the
    /// binomial quotient weights in each direction (module docs).
    /// Refused on a structure mismatch.
    pub fn mul(&self, other: &Self) -> Self {
        if !self.aligned(other) {
            return Self::refused(self.deg_u + other.deg_u, self.deg_v + other.deg_v);
        }
        let (a, b) = (self.deg_u, self.deg_v);
        let (c, d) = (other.deg_u, other.deg_v);
        // The two weight tables are functions of the bidegrees alone,
        // so one pair of them serves every cell of this product.
        let wu = bern_weights(a, c);
        let wv = bern_weights(b, d);
        let cells = self
            .cells
            .iter()
            .zip(other.cells.iter())
            .map(|(ra, rb)| {
                ra.iter()
                    .zip(rb.iter())
                    .map(|(ba, bb)| mul_block(ba, bb, (a, b), (c, d), &wu, &wv))
                    .collect()
            })
            .collect();
        Self {
            deg_u: a + c,
            deg_v: b + d,
            breaks_u: self.breaks_u.clone(),
            breaks_v: self.breaks_v.clone(),
            cells,
        }
    }

    /// Cellwise scaling by a ring constant (exact bidegree, one ring
    /// product per coefficient).
    pub fn scale(&self, c: Interval) -> Self {
        Self {
            deg_u: self.deg_u,
            deg_v: self.deg_v,
            breaks_u: self.breaks_u.clone(),
            breaks_v: self.breaks_v.clone(),
            cells: self
                .cells
                .iter()
                .map(|row| {
                    row.iter()
                        .map(|block| block.iter().map(|x| *x * c).collect())
                        .collect()
                })
                .collect(),
        }
    }
}

/// Degree elevation's per-direction multiplier: the all-ones row of
/// the padding degree and the weight table its product reads, or
/// `None` in a direction with no padding, which multiplies by nothing.
///
/// Both are functions of the degrees alone, so they are built once per
/// patch rather than once per cell (and not at all in an unpadded
/// direction). Nothing about the arithmetic moves: the same rows and
/// the same table entries reach the same fold in the same order.
type ElevationPad = Option<(Vec<Interval>, Cow<'static, BernWeights>)>;

/// The two directions' multipliers for one elevation.
struct ElevationPads {
    u: ElevationPad,
    v: ElevationPad,
}

impl ElevationPads {
    fn new((deg_u, deg_v): (usize, usize), (pad_u, pad_v): (usize, usize)) -> Self {
        let dir = |deg: usize, pad: usize| {
            (pad > 0).then(|| (vec![Interval::one(); pad + 1], bern_weights(deg, pad)))
        };
        Self {
            u: dir(deg_u, pad_u),
            v: dir(deg_v, pad_v),
        }
    }
}

/// One cell's Bernstein degree elevation, `u` then `v`, each as a
/// product with the all-ones row of the padding degree.
fn elevate_block(
    block: &[Interval],
    deg_u: usize,
    deg_v: usize,
    pads: &ElevationPads,
) -> Vec<Interval> {
    let nv = deg_v + 1;
    // v-direction first, per u index.
    let mut rows: Vec<Vec<Interval>> = (0..=deg_u)
        .map(|i| {
            let row = &block[i * nv..(i + 1) * nv];
            match &pads.v {
                None => row.to_vec(),
                Some((ones, w)) => bern_mul_row_with(row, ones, w),
            }
        })
        .collect();
    if let Some((ones, w)) = &pads.u {
        let width = rows.first().map_or(0, Vec::len);
        // `ones.len() == pad_u + 1`, so the elevated u-extent is
        // `deg_u + pad_u + 1`.
        let mut out: Vec<Vec<Interval>> = vec![vec![Interval::zero(); width]; deg_u + ones.len()];
        for k in 0..width {
            let col: Vec<Interval> = rows.iter().map(|r| r[k]).collect();
            let elevated = bern_mul_row_with(&col, ones, w);
            for (j, e) in elevated.iter().enumerate() {
                out[j][k] = *e;
            }
        }
        rows = out;
    }
    rows.concat()
}

/// One cell's tensor Bernstein product (module docs): the v-direction
/// row product for every `(i, k)` u-index pair, accumulated into the
/// u-convolution slot `r = i + k` with the u-binomial quotient weight,
/// both read out of the tables the whole product shares.
fn mul_block(
    ba: &[Interval],
    bb: &[Interval],
    (a, b): (usize, usize),
    (c, d): (usize, usize),
    wu: &BernWeights,
    wv: &BernWeights,
) -> Vec<Interval> {
    let (nb, nd) = (b + 1, d + 1);
    let out_v = b + d + 1;
    let mut out = vec![Interval::zero(); (a + c + 1) * out_v];
    // One row buffer for the whole block: the v-product overwrites it
    // per `(i, k)` pair and is consumed before the next pair runs.
    let mut prod: Vec<Interval> = Vec::with_capacity(out_v);
    // Fixed ascending order throughout (D9).
    for i in 0..=a {
        let row_a = &ba[i * nb..(i + 1) * nb];
        for k in 0..=c {
            let row_b = &bb[k * nd..(k + 1) * nd];
            bern_mul_row_into(row_a, row_b, wv, &mut prod);
            let r = i + k;
            // `at` carries the `lo(r)` convention with the table, so
            // this lookup cannot disagree with the fold's keying.
            // Non-negative, as every binomial quotient:
            // [`Interval::mul_nonneg`].
            let w = wu.at(r, i);
            for (t, p) in prod.iter().enumerate() {
                out[r * out_v + t] = out[r * out_v + t] + p.mul_nonneg(w);
            }
        }
    }
    out
}

/// The iterates of Wolfe's min-norm-point algorithm on `points`
/// (P. Wolfe, "Finding the nearest point in a polytope", 1976): each
/// major iterate is the nearest point of the affine hull of a corral of
/// at most four of the points, and the last is the convex hull's point
/// nearest the origin, up to the arithmetic. Only a DIRECTION is taken
/// from these ([`PatchSpans::cell_norm_inf`] certifies its own reading
/// along each), so an iterate that rounding leaves short of the optimum
/// costs tightness, never soundness. Every step commutes with a rotation
/// of the points. Capped at 64 major iterates.
fn min_norm_point_iterates(points: &[[f64; 3]]) -> Vec<[f64; 3]> {
    let dot = |p: &[f64; 3], q: &[f64; 3]| p[0] * q[0] + p[1] * q[1] + p[2] * q[2];
    let combine = |set: &[usize], w: &[f64]| -> [f64; 3] {
        core::array::from_fn(|i| set.iter().zip(w).map(|(&k, a)| a * points[k][i]).sum())
    };
    let scale = points.iter().map(|p| dot(p, p)).fold(0.0f64, f64::max);
    let Some(first) = (0..points.len())
        .min_by(|&a, &b| dot(&points[a], &points[a]).total_cmp(&dot(&points[b], &points[b])))
    else {
        return Vec::new();
    };
    let mut set = vec![first];
    let mut w = vec![1.0];
    let mut x = points[first];
    let mut out = vec![x];
    for _ in 0..64 {
        let Some(j) =
            (0..points.len()).min_by(|&a, &b| dot(&points[a], &x).total_cmp(&dot(&points[b], &x)))
        else {
            break;
        };
        // Optimal: no point lies further toward the origin than `x`.
        if dot(&x, &x) - dot(&points[j], &x) <= 1e-15 * scale || set.contains(&j) {
            break;
        }
        set.push(j);
        w.push(0.0);
        // Minor cycles: move toward the corral's affine minimizer,
        // dropping the points whose weight it drives to zero.
        for _ in 0..8 {
            let Some(lambda) = affine_minimizer(points, &set) else {
                break;
            };
            if lambda.iter().all(|&l| l > 0.0) {
                w = lambda;
                break;
            }
            let theta = w
                .iter()
                .zip(&lambda)
                .filter(|(_, l)| **l <= 0.0)
                .map(|(a, l)| a / (a - l))
                .fold(1.0f64, f64::min);
            for (a, l) in w.iter_mut().zip(&lambda) {
                *a = (1.0 - theta) * *a + theta * l;
            }
            let kept: Vec<(usize, f64)> = set
                .iter()
                .zip(&w)
                .filter(|(_, a)| **a > 1e-14)
                .map(|(&k, &a)| (k, a))
                .collect();
            let total: f64 = kept.iter().map(|(_, a)| a).sum();
            if kept.is_empty() || total.is_nan() || total <= 0.0 {
                return out;
            }
            set = kept.iter().map(|(k, _)| *k).collect();
            w = kept.iter().map(|(_, a)| a / total).collect();
        }
        let next = combine(&set, &w);
        if !next.iter().all(|v| v.is_finite()) || dot(&next, &next) >= dot(&x, &x) {
            break;
        }
        x = next;
        out.push(x);
    }
    out
}

/// The iterates of a Frank–Wolfe descent toward the convex hull of
/// `points`' point nearest the origin, from `start`, with exact line
/// search: each moves toward the point furthest along `−x`. Only a
/// DIRECTION is taken from these (as [`min_norm_point_iterates`]'s), so
/// the cap trades tightness for time, never soundness. Capped at 64
/// steps.
fn frank_wolfe_iterates(points: &[[f64; 3]], start: [f64; 3]) -> Vec<[f64; 3]> {
    let dot = |p: &[f64; 3], q: &[f64; 3]| p[0] * q[0] + p[1] * q[1] + p[2] * q[2];
    let mut x = start;
    let mut out = Vec::new();
    for _ in 0..64 {
        if !x.iter().all(|v| v.is_finite()) || dot(&x, &x) == 0.0 {
            break;
        }
        let Some(s) = points
            .iter()
            .min_by(|p, q| dot(p, &x).total_cmp(&dot(q, &x)))
        else {
            break;
        };
        let step: [f64; 3] = core::array::from_fn(|i| s[i] - x[i]);
        let len2 = dot(&step, &step);
        if len2.is_nan() || len2 <= 0.0 {
            break;
        }
        let gamma = (-dot(&x, &step) / len2).clamp(0.0, 1.0);
        if gamma.is_nan() || gamma <= 0.0 {
            break;
        }
        x = core::array::from_fn(|i| x[i] + gamma * step[i]);
        out.push(x);
    }
    out
}

/// The weights `λ` (summing to 1) of the point of `set`'s affine hull
/// nearest the origin, from the bordered Gram system; `None` when it is
/// singular (the corral is affinely dependent).
fn affine_minimizer(points: &[[f64; 3]], set: &[usize]) -> Option<Vec<f64>> {
    let n = set.len();
    let dot = |p: &[f64; 3], q: &[f64; 3]| p[0] * q[0] + p[1] * q[1] + p[2] * q[2];
    // [G 1; 1ᵀ 0] [λ; μ] = [0; 1], solved by elimination with pivoting.
    let m = n + 1;
    let mut a = vec![vec![0.0f64; m + 1]; m];
    for (r, &i) in set.iter().enumerate() {
        for (c, &j) in set.iter().enumerate() {
            a[r][c] = dot(&points[i], &points[j]);
        }
        a[r][n] = 1.0;
        a[n][r] = 1.0;
    }
    a[n][m] = 1.0;
    let norm = a
        .iter()
        .flat_map(|r| r.iter())
        .fold(0.0f64, |s, v| s.max(v.abs()));
    for col in 0..m {
        let piv = (col..m).max_by(|&p, &q| a[p][col].abs().total_cmp(&a[q][col].abs()))?;
        if a[piv][col].abs().is_nan() || a[piv][col].abs() <= 1e-13 * norm {
            return None;
        }
        a.swap(col, piv);
        let pivot_row = a[col].clone();
        for (r, row) in a.iter_mut().enumerate() {
            if r != col {
                let f = row[col] / pivot_row[col];
                for (cell, p) in row.iter_mut().zip(&pivot_row).skip(col) {
                    *cell -= f * p;
                }
            }
        }
    }
    let lambda: Vec<f64> = (0..n).map(|r| a[r][m] / a[r][r]).collect();
    lambda.iter().all(|l| l.is_finite()).then_some(lambda)
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod tests {
    use super::super::binom_row;
    use super::super::tests::{bern_mul_row_base, same_bits};
    use super::*;
    use crate::real::Bounds;
    use crate::spline::KnotVector;

    /// A patch of bidegree `(du, dv)` with one interior break in each
    /// direction, decomposed. Every operand here shares the same two
    /// break values whatever its degrees, so any two are aligned.
    fn decomposed_at(du: usize, dv: usize, seed: f64) -> PatchSpans {
        let clamped = |d: usize, interior: f64| {
            let mut k = vec![0.0; d + 1];
            k.push(interior);
            k.resize(2 * d + 3, 1.0);
            KnotVector::clamped(k, d).unwrap()
        };
        let ku = clamped(du, 0.4);
        let kv = clamped(dv, 0.6);
        let nv = kv.control_count();
        let grid = TensorCoeffs::from_fn(&ku, &kv, |i, j| {
            let c = ((i * nv + j) as f64 - 5.0) * seed / 3.0;
            Interval::from_bounds(c - 2e-14, c + 5e-14)
        });
        PatchSpans::decompose(&grid, &[], &[])
    }

    /// [`mul_block`] as it was before the weight tables existed: the
    /// pre-memo convolution in the v direction, and the u weight
    /// rebuilt as a ring quotient per `(i, k)` pair — the retired
    /// spelling, verbatim, so the rows below pin the hoisted tables
    /// against the code they replaced.
    fn mul_block_base(
        ba: &[Interval],
        bb: &[Interval],
        (a, b): (usize, usize),
        (c, d): (usize, usize),
    ) -> Vec<Interval> {
        let bin_a = binom_row(a);
        let bin_c = binom_row(c);
        let bin_ac = binom_row(a + c);
        let (nb, nd) = (b + 1, d + 1);
        let out_v = b + d + 1;
        let mut out = vec![Interval::zero(); (a + c + 1) * out_v];
        for i in 0..=a {
            let row_a = &ba[i * nb..(i + 1) * nb];
            for k in 0..=c {
                let row_b = &bb[k * nd..(k + 1) * nd];
                let prod = bern_mul_row_base(row_a, row_b);
                let r = i + k;
                let w = Interval::point(bin_a[i] * bin_c[k]) / Interval::point(bin_ac[r]);
                for (t, p) in prod.iter().enumerate() {
                    out[r * out_v + t] = out[r * out_v + t] + *p * w;
                }
            }
        }
        out
    }

    /// [`elevate_block`] as it was before the weight tables existed.
    fn elevate_block_base(
        block: &[Interval],
        deg_u: usize,
        deg_v: usize,
        pad_u: usize,
        pad_v: usize,
    ) -> Vec<Interval> {
        let one = Interval::one();
        let nv = deg_v + 1;
        let mut rows: Vec<Vec<Interval>> = (0..=deg_u)
            .map(|i| {
                let row = &block[i * nv..(i + 1) * nv];
                if pad_v == 0 {
                    row.to_vec()
                } else {
                    bern_mul_row_base(row, &vec![one; pad_v + 1])
                }
            })
            .collect();
        if pad_u > 0 {
            let width = rows.first().map_or(0, Vec::len);
            let ones = vec![one; pad_u + 1];
            let mut out: Vec<Vec<Interval>> =
                vec![vec![Interval::zero(); width]; deg_u + pad_u + 1];
            for k in 0..width {
                let col: Vec<Interval> = rows.iter().map(|r| r[k]).collect();
                let elevated = bern_mul_row_base(&col, &ones);
                for (j, e) in elevated.iter().enumerate() {
                    out[j][k] = *e;
                }
            }
            rows = out;
        }
        rows.concat()
    }

    /// The product hoists ONE weight table per direction to the whole
    /// patch, so the two tables have to stay told apart — and on a
    /// symmetric bidegree they are the same table, which no comparison
    /// against itself can see. Asymmetric bidegrees in both operands,
    /// against the pre-memo convolution: every coefficient of every
    /// cell, bitwise.
    #[test]
    fn an_asymmetric_products_coefficients_are_the_pre_memo_convolution() {
        for ((au, av), (bu, bv)) in [((1, 3), (3, 1)), ((2, 4), (4, 2)), ((3, 1), (1, 2))] {
            let a = decomposed_at(au, av, 1.0);
            let b = decomposed_at(bu, bv, -0.7);
            let p = a.mul(&b);
            let (nu, nv) = p.cell_counts();
            assert!(nu > 1 && nv > 1, "the fixture must have interior breaks");
            assert_eq!(
                (p.deg_u, p.deg_v),
                (au + bu, av + bv),
                "product bidegree of ({au}, {av}) × ({bu}, {bv})"
            );
            for su in 0..nu {
                for sv in 0..nv {
                    let want =
                        mul_block_base(&a.cells[su][sv], &b.cells[su][sv], (au, av), (bu, bv));
                    let got = &p.cells[su][sv];
                    assert_eq!(
                        got.len(),
                        want.len(),
                        "block width at ({su}, {sv}) of ({au}, {av}) × ({bu}, {bv})"
                    );
                    for (n, (g, w)) in got.iter().zip(want.iter()).enumerate() {
                        assert!(
                            same_bits(*g, *w),
                            "({au}, {av}) × ({bu}, {bv}) cell ({su}, {sv}) coefficient {n}: \
                             {g:?} vs {w:?}"
                        );
                    }
                }
            }
        }
    }

    /// The same claim for degree elevation, whose two tables are
    /// hoisted to the whole patch as well: asymmetric degrees and
    /// asymmetric pads, both directions, including a pad of zero in
    /// each — every coefficient of every cell against the pre-memo
    /// convolution, bitwise.
    #[test]
    fn an_asymmetric_elevations_coefficients_are_the_pre_memo_convolution() {
        for ((du, dv), (pad_u, pad_v)) in [
            ((1, 3), (3, 1)),
            ((2, 1), (1, 4)),
            ((3, 2), (2, 0)),
            ((2, 3), (0, 2)),
        ] {
            let p = decomposed_at(du, dv, 0.9);
            let e = p.elevated(du + pad_u, dv + pad_v);
            let (nu, nv) = e.cell_counts();
            assert!(nu > 1 && nv > 1, "the fixture must have interior breaks");
            assert_eq!(
                (e.deg_u, e.deg_v),
                (du + pad_u, dv + pad_v),
                "elevated bidegree of ({du}, {dv}) by ({pad_u}, {pad_v})"
            );
            for su in 0..nu {
                for sv in 0..nv {
                    let want = elevate_block_base(&p.cells[su][sv], du, dv, pad_u, pad_v);
                    let got = &e.cells[su][sv];
                    assert_eq!(
                        got.len(),
                        want.len(),
                        "block width at ({su}, {sv}) of ({du}, {dv}) by ({pad_u}, {pad_v})"
                    );
                    for (n, (g, w)) in got.iter().zip(want.iter()).enumerate() {
                        assert!(
                            same_bits(*g, *w),
                            "({du}, {dv}) by ({pad_u}, {pad_v}) cell ({su}, {sv}) \
                             coefficient {n}: {g:?} vs {w:?}"
                        );
                    }
                }
            }
        }
    }

    /// The one cell `(su, sv)` of `p`, as a patch in its own right.
    fn single_cell(p: &PatchSpans, su: usize, sv: usize) -> PatchSpans {
        PatchSpans {
            deg_u: p.deg_u,
            deg_v: p.deg_v,
            breaks_u: vec![p.breaks_u[su], p.breaks_u[su + 1]],
            breaks_v: vec![p.breaks_v[sv], p.breaks_v[sv + 1]],
            cells: vec![vec![p.cells[su][sv].clone()]],
        }
    }

    fn decomposed(du: usize, dv: usize, seed: f64) -> PatchSpans {
        let ku = KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.4, 1.0, 1.0, 1.0], du).unwrap();
        let kv = KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.6, 1.0, 1.0, 1.0], dv).unwrap();
        let nv = kv.control_count();
        let grid = TensorCoeffs::from_fn(&ku, &kv, |i, j| {
            let c = ((i * nv + j) as f64 - 5.0) * seed / 3.0;
            Interval::from_bounds(c - 2e-14, c + 5e-14)
        });
        PatchSpans::decompose(&grid, &[], &[])
    }

    /// The tensor product is separable over cells: a cell's
    /// coefficients are a function of that cell's two operand blocks
    /// alone, bit for bit. This is what licenses forming a product
    /// only on the cells that read it.
    #[test]
    fn a_products_cell_is_the_same_formed_whole_patch_or_alone() {
        let a = decomposed(2, 2, 1.0);
        let b = decomposed(2, 2, -0.7);
        let whole = a.mul(&b);
        let (nu, nv) = whole.cell_counts();
        assert!(nu > 1 && nv > 1, "the fixture must have interior breaks");
        for su in 0..nu {
            for sv in 0..nv {
                let alone = single_cell(&a, su, sv).mul(&single_cell(&b, su, sv));
                let (got, want) = (&alone.cells[0][0], &whole.cells[su][sv]);
                assert_eq!(got.len(), want.len(), "block width at ({su}, {sv})");
                for (n, (g, w)) in got.iter().zip(want.iter()).enumerate() {
                    assert!(
                        g.lo().to_bits() == w.lo().to_bits()
                            && g.hi().to_bits() == w.hi().to_bits(),
                        "cell ({su}, {sv}) coefficient {n}: {g:?} vs {w:?}"
                    );
                }
                // The hull the bound actually reads follows.
                let (gh, wh) = (alone.cell_hull(0, 0), whole.cell_hull(su, sv));
                assert!(
                    gh.lo().to_bits() == wh.lo().to_bits()
                        && gh.hi().to_bits() == wh.hi().to_bits(),
                    "cell hull at ({su}, {sv}): {gh:?} vs {wh:?}"
                );
            }
        }
    }

    /// `cell_norm_sup` reads the largest norm of one coefficient
    /// VECTOR, bit for bit, where a box of the three cell hulls would
    /// pair one channel's extreme with another's; a channel triple that
    /// does not share a bidegree, or a cell out of range, refuses.
    #[test]
    fn the_cell_norm_is_the_largest_coefficient_vectors_not_the_boxes() {
        // `y = 1 − x` peaks where `x` is least, so the two channels'
        // extremes sit on different coefficients.
        let x = decomposed(2, 2, 1.0);
        let y = x.constant(Interval::point(1.0)).sub(&x);
        let z = x.mul(&y);
        let (x, y) = (x.elevated(4, 4), y.elevated(4, 4));
        let (nu, nv) = z.cell_counts();
        let mut below_box = 0usize;
        for su in 0..nu {
            for sv in 0..nv {
                let got = PatchSpans::cell_norm_sup([&x, &y, &z], su, sv);
                let want = (0..x.cells[su][sv].len())
                    .map(|k| {
                        crate::interval::norm_sup(&[
                            x.cells[su][sv][k],
                            y.cells[su][sv][k],
                            z.cells[su][sv][k],
                        ])
                    })
                    .fold(0.0f64, f64::max);
                assert_eq!(got.to_bits(), want.to_bits(), "cell ({su}, {sv})");
                let boxed = crate::interval::norm_sup(&[
                    x.cell_hull(su, sv),
                    y.cell_hull(su, sv),
                    z.cell_hull(su, sv),
                ]);
                assert!(
                    got <= boxed,
                    "cell ({su}, {sv}): {got:e} above the box {boxed:e}"
                );
                if got < boxed {
                    below_box += 1;
                }
            }
        }
        assert!(
            below_box > 0,
            "no cell separates the coefficient norm from the box"
        );
        let low = decomposed(2, 2, 0.3);
        assert!(PatchSpans::cell_norm_sup([&x, &y, &low], 0, 0).is_nan());
        assert!(PatchSpans::cell_norm_sup([&x, &y, &z], nu, 0).is_nan());
    }

    /// A deterministic draw in `[-1, 1)` (an LCG, so the row is replayable).
    fn draw(state: &mut u64) -> f64 {
        *state = state
            .wrapping_mul(6_364_136_223_846_793_005)
            .wrapping_add(1_442_695_040_888_963_407);
        #[allow(clippy::cast_precision_loss)]
        let unit = (*state >> 11) as f64 / (1u64 << 53) as f64;
        2.0 * unit - 1.0
    }

    /// One `(2, 2)` cell per channel from coefficient VECTORS: `vectors[k]`
    /// is coefficient `k`'s point, each coordinate widened by `pad`.
    fn cell_of(vectors: &[[f64; 3]], pad: f64) -> [PatchSpans; 3] {
        let deg = match vectors.len() {
            9 => 2,
            16 => 3,
            25 => 4,
            n => panic!("no square bidegree holds {n} coefficients"),
        };
        core::array::from_fn(|c| PatchSpans {
            deg_u: deg,
            deg_v: deg,
            breaks_u: vec![0.0, 1.0],
            breaks_v: vec![0.0, 1.0],
            cells: vec![vec![
                vectors
                    .iter()
                    .map(|v| Interval::from_bounds(v[c] - pad, v[c] + pad))
                    .collect(),
            ]],
        })
    }

    /// **`cell_norm_inf` never reads above a point of the coefficient
    /// hull** (the form lies in that hull, so this is the stronger
    /// oracle), over 400 drawn cells: clusters a chosen distance off the
    /// origin, spread from tight to wider than that distance, flat and
    /// collinear clusters, one straddling the origin, and coordinates
    /// near overflow. Each is read against 2000 sampled hull points
    /// (every vertex, every pairwise midpoint, random convex weights),
    /// and against the per-coordinate box's mignitude, which the hull's
    /// true distance is never below: the row counts the cells where the
    /// capped descent reads under the box and pins how far.
    #[test]
    fn the_hull_floor_is_never_above_a_hull_point() {
        let mut state = 0x5eed_u64;
        let mut under_box = 0usize;
        let mut worst_under = 0.0f64;
        for case in 0..400 {
            let kind = case % 8;
            let dir = {
                let d = [draw(&mut state), draw(&mut state), draw(&mut state)];
                let n = (d[0] * d[0] + d[1] * d[1] + d[2] * d[2]).sqrt().max(1e-3);
                d.map(|x| x / n)
            };
            let off = match kind {
                6 => 0.0,
                7 => 1e150,
                _ => 0.5 + draw(&mut state).abs(),
            };
            let spread = match kind {
                0 => 1e-3,
                1 => 0.1,
                2 => 0.5,
                3 => 1.2,
                7 => 1e149,
                _ => 0.3,
            };
            let count = [9, 16, 25][case % 3];
            let turning = case % 5 == 4 && kind != 7;
            let vectors: Vec<[f64; 3]> = (0..count)
                .map(|k| {
                    if turning {
                        // `E ≈ d·n` over a cell whose normal turns: points
                        // on a sphere of radius `off` across a cap of
                        // half-angle up to ~1.4 rad, plus a little noise.
                        #[allow(clippy::cast_precision_loss)]
                        let (a, b) = ((k % 5) as f64 / 4.0 - 0.5, (k / 5) as f64 / 4.0 - 0.5);
                        let cap = 2.8 * draw(&mut state).abs().max(0.1);
                        let (th, ph) = (cap * a, cap * b);
                        let n = [th.sin() * ph.cos(), ph.sin(), th.cos() * ph.cos()];
                        return core::array::from_fn(|i| {
                            off * n[i] + 1e-3 * off * draw(&mut state)
                        });
                    }
                    let mut e = [draw(&mut state), draw(&mut state), draw(&mut state)];
                    if kind == 4 {
                        e[2] = 0.0; // flat
                    }
                    if kind == 5 {
                        let t = e[0];
                        e = [t, t, t]; // collinear
                    }
                    core::array::from_fn(|i| off * dir[i] + spread * e[i])
                })
                .collect();
            let [x, y, z] = cell_of(&vectors, 1e-15 * off.max(1.0));
            let got = PatchSpans::cell_norm_inf([&x, &y, &z], 0, 0);
            assert!(got.is_finite() && got >= 0.0, "case {case}: {got:e}");
            let norm = |p: [f64; 3]| (p[0] * p[0] + p[1] * p[1] + p[2] * p[2]).sqrt();
            let mut least = f64::INFINITY;
            let mut sample = |w: &[f64]| {
                let total: f64 = w.iter().sum();
                let p: [f64; 3] = core::array::from_fn(|i| {
                    w.iter().zip(&vectors).map(|(a, v)| a * v[i]).sum::<f64>() / total
                });
                least = least.min(norm(p));
            };
            for k in 0..count {
                let mut w = vec![0.0; count];
                w[k] = 1.0;
                sample(&w);
                for l in k + 1..count {
                    let mut w = vec![0.0; count];
                    w[k] = 1.0;
                    w[l] = 1.0;
                    sample(&w);
                }
            }
            for _ in 0..2000 {
                let w: Vec<f64> = (0..count).map(|_| draw(&mut state).abs().powi(4)).collect();
                sample(&w);
            }
            assert!(
                got <= least * (1.0 + 1e-12),
                "case {case}: the floor {got:e} is above a hull point at {least:e}"
            );
            let boxed = {
                let mig = |p: &PatchSpans| {
                    let h = p.cell_hull(0, 0);
                    h.lo().max(-h.hi()).max(0.0)
                };
                (mig(&x).powi(2) + mig(&y).powi(2) + mig(&z).powi(2)).sqrt()
            };
            if got < boxed {
                under_box += 1;
                worst_under = worst_under.max(1.0 - got / boxed);
            }
        }
        // The hull's distance is never below the box's mignitude; the
        // reading lands on it up to the arithmetic. Measured: 6 cells under
        // the box, by at most 1.1e-15 of it (a Frank-Wolfe descent capped at
        // 64 steps, the reading this replaced, was 27% under on a turning
        // normal's cell).
        assert!(
            worst_under <= 1e-14,
            "{under_box} cells read under the box, the worst by {worst_under:e} of it"
        );
    }
}
