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
//! Every product routes through `super::bern_mul_row_into` (and
//! `super::bern_mul_row_with` for degree elevation) over the weight
//! tables `super::bern_weights` serves, whose exactness cap
//! (`BINOM_EXACT_MAX`) bounds the per-direction degree a product may
//! reach; beyond it the binomial row is all-poison and every hull is
//! `NaN`, which fails every `≤ ε` certification loudly (D4 ¶2). Work
//! per cell scales as `(a+1)(c+1)` row products of length `O(b + d)`.
//!
//! Those tables are functions of the degrees alone, so each entry
//! point looks them up ONCE for the whole patch and hands them down
//! to the per-cell loop; the table carries the degree pair it was
//! built for, so a table reaching the wrong direction announces
//! itself rather than answering.
//!
//! # C6 and poison
//!
//! Structure (knots, degrees, break merges, cell counts) is `f64`;
//! every coefficient is a [`RingInterval`]. Nothing here evaluates or
//! samples anything.

use super::super::knots::KnotVector;
use super::{
    BernWeights, bern_mul_row_into, bern_mul_row_with, bern_weights, to_bezier_spans_extra,
};
use crate::ring_interval::RingInterval;
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
    /// degree, which is exactly what `super::bern_mul_row_with`'s
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

/// Degree elevation's per-direction multiplier: the all-ones row of
/// the padding degree and the weight table its product reads, or
/// `None` in a direction with no padding, which multiplies by nothing.
///
/// Both are functions of the degrees alone, so they are built once per
/// patch rather than once per cell (and not at all in an unpadded
/// direction). Nothing about the arithmetic moves: the same rows and
/// the same table entries reach the same fold in the same order.
type ElevationPad = Option<(Vec<RingInterval>, Cow<'static, BernWeights>)>;

/// The two directions' multipliers for one elevation.
struct ElevationPads {
    u: ElevationPad,
    v: ElevationPad,
}

impl ElevationPads {
    fn new((deg_u, deg_v): (usize, usize), (pad_u, pad_v): (usize, usize)) -> Self {
        let dir = |deg: usize, pad: usize| {
            (pad > 0).then(|| (vec![RingInterval::one(); pad + 1], bern_weights(deg, pad)))
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
    block: &[RingInterval],
    deg_u: usize,
    deg_v: usize,
    pads: &ElevationPads,
) -> Vec<RingInterval> {
    let nv = deg_v + 1;
    // v-direction first, per u index.
    let mut rows: Vec<Vec<RingInterval>> = (0..=deg_u)
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
        let mut out: Vec<Vec<RingInterval>> =
            vec![vec![RingInterval::zero(); width]; deg_u + ones.len()];
        for k in 0..width {
            let col: Vec<RingInterval> = rows.iter().map(|r| r[k]).collect();
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
    ba: &[RingInterval],
    bb: &[RingInterval],
    (a, b): (usize, usize),
    (c, d): (usize, usize),
    wu: &BernWeights,
    wv: &BernWeights,
) -> Vec<RingInterval> {
    let (nb, nd) = (b + 1, d + 1);
    let out_v = b + d + 1;
    let mut out = vec![RingInterval::zero(); (a + c + 1) * out_v];
    // One row buffer for the whole block: the v-product overwrites it
    // per `(i, k)` pair and is consumed before the next pair runs.
    let mut prod: Vec<RingInterval> = Vec::with_capacity(out_v);
    // Fixed ascending order throughout (D9).
    for i in 0..=a {
        let row_a = &ba[i * nb..(i + 1) * nb];
        for k in 0..=c {
            let row_b = &bb[k * nd..(k + 1) * nd];
            bern_mul_row_into(row_a, row_b, wv, &mut prod);
            let r = i + k;
            // `at` carries the `lo(r)` convention with the table, so
            // this lookup cannot disagree with the fold's keying.
            let w = wu.at(r, i);
            for (t, p) in prod.iter().enumerate() {
                out[r * out_v + t] = out[r * out_v + t] + *p * w;
            }
        }
    }
    out
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod tests {
    use super::super::binom_row;
    use super::super::tests::{bern_mul_row_base, same_bits};
    use super::*;

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
        let (nu, nv) = (ku.control_count(), kv.control_count());
        let grid: Vec<RingInterval> = (0..nu * nv)
            .map(|n| {
                let c = (n as f64 - 5.0) * seed / 3.0;
                RingInterval::from_bounds(c - 2e-14, c + 5e-14)
            })
            .collect();
        PatchSpans::decompose(&ku, &kv, &grid, &[], &[])
    }

    /// [`mul_block`] as it was before the weight tables existed: the
    /// pre-memo convolution in the v direction, and the u weight
    /// rebuilt as a ring quotient per `(i, k)` pair — the retired
    /// spelling, verbatim, so the rows below pin the hoisted tables
    /// against the code they replaced.
    fn mul_block_base(
        ba: &[RingInterval],
        bb: &[RingInterval],
        (a, b): (usize, usize),
        (c, d): (usize, usize),
    ) -> Vec<RingInterval> {
        let bin_a = binom_row(a);
        let bin_c = binom_row(c);
        let bin_ac = binom_row(a + c);
        let (nb, nd) = (b + 1, d + 1);
        let out_v = b + d + 1;
        let mut out = vec![RingInterval::zero(); (a + c + 1) * out_v];
        for i in 0..=a {
            let row_a = &ba[i * nb..(i + 1) * nb];
            for k in 0..=c {
                let row_b = &bb[k * nd..(k + 1) * nd];
                let prod = bern_mul_row_base(row_a, row_b);
                let r = i + k;
                let w = RingInterval::point(bin_a[i] * bin_c[k]) / RingInterval::point(bin_ac[r]);
                for (t, p) in prod.iter().enumerate() {
                    out[r * out_v + t] = out[r * out_v + t] + *p * w;
                }
            }
        }
        out
    }

    /// [`elevate_block`] as it was before the weight tables existed.
    fn elevate_block_base(
        block: &[RingInterval],
        deg_u: usize,
        deg_v: usize,
        pad_u: usize,
        pad_v: usize,
    ) -> Vec<RingInterval> {
        let one = RingInterval::one();
        let nv = deg_v + 1;
        let mut rows: Vec<Vec<RingInterval>> = (0..=deg_u)
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
            let mut out: Vec<Vec<RingInterval>> =
                vec![vec![RingInterval::zero(); width]; deg_u + pad_u + 1];
            for k in 0..width {
                let col: Vec<RingInterval> = rows.iter().map(|r| r[k]).collect();
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
        let (nu, nv) = (ku.control_count(), kv.control_count());
        let grid: Vec<RingInterval> = (0..nu * nv)
            .map(|n| {
                let c = (n as f64 - 5.0) * seed / 3.0;
                RingInterval::from_bounds(c - 2e-14, c + 5e-14)
            })
            .collect();
        PatchSpans::decompose(&ku, &kv, &grid, &[], &[])
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
}
