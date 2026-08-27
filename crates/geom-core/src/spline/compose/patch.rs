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
//! subtract to ring rounding; what survives is the residual
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
//! disagree combine to **poison**, never to a silently misaligned
//! answer.
//!
//! # Degree budget
//!
//! Every product routes through `super::bern_mul_row`, whose
//! exactness cap (`BINOM_EXACT_MAX`) bounds the per-direction
//! degree a product may reach; beyond it the binomial row is
//! all-poison and every hull is `NaN`, which fails every `≤ ε`
//! certification loudly (D4 ¶2). Work per cell scales as
//! `(a+1)(c+1)` row products of length `O(b + d)`.
//!
//! # C6 and poison
//!
//! Structure (knots, degrees, break merges, cell counts) is `f64`;
//! every coefficient is a [`RingInterval`]. Nothing here evaluates or
//! samples anything.

use super::super::knots::KnotVector;
use super::{bern_mul_row, binom_row, to_bezier_spans_extra};
use crate::ring_interval::RingInterval;

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
    cells: Vec<Vec<Vec<RingInterval>>>,
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
    /// docs). Poison for an out-of-range cell. Fixed ascending fold
    /// order (D9).
    pub fn cell_hull(&self, su: usize, sv: usize) -> RingInterval {
        let Some(block) = self.cells.get(su).and_then(|r| r.get(sv)) else {
            return RingInterval::poison();
        };
        let mut acc = RingInterval::poison();
        for (n, c) in block.iter().enumerate() {
            acc = if n == 0 {
                *c
            } else {
                RingInterval::hull(acc, *c)
            };
        }
        acc
    }

    /// Tensor-product Bézier decomposition of one scalar channel of a
    /// spline whose control grid is **row-major `iu·nv + iv`**, with
    /// `extra_u`/`extra_v` break parameters injected in each direction
    /// so several channels land on one shared break list (module docs:
    /// alignment).
    ///
    /// Structure-filtered, never an error: an extra outside the open
    /// domain or duplicating a knot is dropped.
    pub fn decompose(
        ku: &KnotVector,
        kv: &KnotVector,
        grid: &[RingInterval],
        extra_u: &[f64],
        extra_v: &[f64],
    ) -> Self {
        let nu = ku.control_count();
        let nv = kv.control_count();
        if grid.len() != nu * nv {
            return Self::poisoned(ku.degree(), kv.degree());
        }
        // Stage 1 (u): one univariate decomposition per v-column;
        // identical structure across columns by construction.
        let mut breaks_u = Vec::new();
        let mut deg_u = ku.degree();
        // stage1[su][a][jv]
        let mut stage1: Vec<Vec<Vec<RingInterval>>> = Vec::new();
        for jv in 0..nv {
            let col: Vec<RingInterval> = (0..nu).map(|iu| grid[iu * nv + jv]).collect();
            let bs = to_bezier_spans_extra(ku, &col, extra_u);
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
        let mut cells: Vec<Vec<Vec<RingInterval>>> = Vec::new();
        for span_rows in &stage1 {
            let mut row_cells: Vec<Vec<RingInterval>> = Vec::new();
            for (a, vrow) in span_rows.iter().enumerate() {
                let bs = to_bezier_spans_extra(kv, vrow, extra_v);
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
    pub fn constant(&self, c: RingInterval) -> Self {
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

    /// A structurally poisoned channel of the given bidegree with no
    /// cells — the mismatch outcome of every combinator (total, D4).
    fn poisoned(deg_u: usize, deg_v: usize) -> Self {
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
    /// degree, which is exactly what `super::bern_mul_row`'s
    /// binomial-quotient product computes (and is exact in ℝ).
    /// Returns `self` unchanged when it is already there; poison when
    /// asked to LOWER a degree.
    pub fn elevated(&self, du: usize, dv: usize) -> Self {
        if du < self.deg_u || dv < self.deg_v {
            return Self::poisoned(du, dv);
        }
        if du == self.deg_u && dv == self.deg_v {
            return self.clone();
        }
        let one = RingInterval::one();
        let pad_u = du - self.deg_u;
        let pad_v = dv - self.deg_v;
        let cells = self
            .cells
            .iter()
            .map(|row| {
                row.iter()
                    .map(|block| elevate_block(block, self.deg_u, self.deg_v, pad_u, pad_v, one))
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
    /// bidegree. Poison on a structure mismatch.
    pub fn add(&self, other: &Self) -> Self {
        self.combine(other, false)
    }

    /// Cellwise difference (`self − other`), both operands first
    /// raised to the common bidegree. Poison on a structure mismatch.
    pub fn sub(&self, other: &Self) -> Self {
        self.combine(other, true)
    }

    fn combine(&self, other: &Self, subtract: bool) -> Self {
        if !self.aligned(other) {
            return Self::poisoned(self.deg_u.max(other.deg_u), self.deg_v.max(other.deg_v));
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
    /// Poison on a structure mismatch.
    pub fn mul(&self, other: &Self) -> Self {
        if !self.aligned(other) {
            return Self::poisoned(self.deg_u + other.deg_u, self.deg_v + other.deg_v);
        }
        let (a, b) = (self.deg_u, self.deg_v);
        let (c, d) = (other.deg_u, other.deg_v);
        let bin_a = binom_row(a);
        let bin_c = binom_row(c);
        let bin_ac = binom_row(a + c);
        let cells = self
            .cells
            .iter()
            .zip(other.cells.iter())
            .map(|(ra, rb)| {
                ra.iter()
                    .zip(rb.iter())
                    .map(|(ba, bb)| mul_block(ba, bb, (a, b), (c, d), &bin_a, &bin_c, &bin_ac))
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
    pub fn scale(&self, c: RingInterval) -> Self {
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

/// One cell's Bernstein degree elevation, `u` then `v`, each as a
/// product with the all-ones row of the padding degree.
fn elevate_block(
    block: &[RingInterval],
    deg_u: usize,
    deg_v: usize,
    pad_u: usize,
    pad_v: usize,
    one: RingInterval,
) -> Vec<RingInterval> {
    let nv = deg_v + 1;
    // v-direction first, per u index.
    let mut rows: Vec<Vec<RingInterval>> = (0..=deg_u)
        .map(|i| {
            let row = &block[i * nv..(i + 1) * nv];
            if pad_v == 0 {
                row.to_vec()
            } else {
                bern_mul_row(row, &vec![one; pad_v + 1])
            }
        })
        .collect();
    if pad_u > 0 {
        let width = rows.first().map_or(0, Vec::len);
        let ones = vec![one; pad_u + 1];
        let mut out: Vec<Vec<RingInterval>> =
            vec![vec![RingInterval::zero(); width]; deg_u + pad_u + 1];
        for k in 0..width {
            let col: Vec<RingInterval> = rows.iter().map(|r| r[k]).collect();
            let elevated = bern_mul_row(&col, &ones);
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
/// u-convolution slot `r = i + k` with the u-binomial quotient weight.
fn mul_block(
    ba: &[RingInterval],
    bb: &[RingInterval],
    (a, b): (usize, usize),
    (c, d): (usize, usize),
    bin_a: &[f64],
    bin_c: &[f64],
    bin_ac: &[f64],
) -> Vec<RingInterval> {
    let (nb, nd) = (b + 1, d + 1);
    let out_v = b + d + 1;
    let mut out = vec![RingInterval::zero(); (a + c + 1) * out_v];
    // Fixed ascending order throughout (D9).
    for i in 0..=a {
        let row_a = &ba[i * nb..(i + 1) * nb];
        for k in 0..=c {
            let row_b = &bb[k * nd..(k + 1) * nd];
            let prod = bern_mul_row(row_a, row_b);
            let r = i + k;
            let w = RingInterval::point(bin_a[i] * bin_c[k]) / RingInterval::point(bin_ac[r]);
            for (t, p) in prod.iter().enumerate() {
                out[r * out_v + t] = out[r * out_v + t] + *p * w;
            }
        }
    }
    out
}
