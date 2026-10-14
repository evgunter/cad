//! The span derivative's coefficient factor, spelled once: the
//! derivative of a degree-`p` B-spline has coefficients
//! `p·(c_{i+1} − c_i)/(u_{i+p+1} − u_{i+1})` (The NURBS Book Eq. 3.4).
//!
//! What a zero knot difference means is the caller's, so
//! [`span_derivative_scale`] takes the two knot values and checks
//! nothing.
//!
//! # The quotient bound
//!
//! Every first-order quantity of a rational spline is a polynomial over
//! a power of its weight. With `A = Σ N wP` the homogeneous numerator
//! and `W = Σ N w` its weight,
//!
//! ```text
//! C′      = N / W²,    N   = A′·W − A·W′
//! S_u     = N_u / W²,  N_u = A_u·W − A·W_u      (S_v likewise)
//! S_u×S_v = M̃ / W³,    M̃   = W·(A_u×A_v) − W_v·(A_u×A) − W_u·(A×A_v)
//! ```
//!
//! — the last because `W·S_u = A_u − W_u·S` and `W·S_v = A_v − W_v·S`
//! cross to `W²·(S_u×S_v) = A_u×A_v − W_v·(A_u×S) − W_u·(S×A_v)`, the
//! `S×S` term vanishing, and `S = A/W`.
//!
//! On a Bézier piece numerator and weight power are Bernstein
//! polynomials. **Raise both to one degree** `n` (elevation does not
//! change a polynomial), with coefficients `a_k` and `b_k`. Then
//!
//! ```text
//! Q = Σ_k B_k a_k / Σ_k B_k b_k = Σ_k λ_k · (a_k / b_k),   λ_k = B_k b_k / Σ_j B_j b_j
//! ```
//!
//! and with every `b_k > 0` the `λ_k` are non-negative and sum to one:
//! `Q` is a convex combination of the ratios `a_k/b_k`. So for any unit
//! `d`, `‖Q‖ ≥ Q·d ≥ min_k (a_k·d)/b_k`, and since the norm is convex,
//! `‖Q‖ ≤ max_k ‖a_k‖/b_k` — [`QuotientPiece::inf_norm`] (along the
//! piece's own direction) and [`QuotientPiece::sup_norm`].
//!
//! Each clause carries the proof. The ratio pairs `a_k` with **its own**
//! `b_k`: the `λ_k` weigh one index on both sides. Both sides sit at
//! **one degree**: a ratio of coefficients in two bases is no
//! combination. `d` is any unit vector, so each piece **picks its own**
//! ([`QuotientPiece::inf_norm`]: the direction of `Σ_k a_k`). The `b_k`
//! are positive because the weight power's coefficients on a piece are
//! the Bernstein product of the weight's own piece coefficients — sums
//! of products with non-negative factors — and those are convex
//! combinations of the positive control weights. There is no weight
//! range, no sign split and no control-point hull.
//!
//! **The pieces.** A curve span's or a cell's quotient is formed once
//! over the span or cell, from its Bernstein rows read off the blossom
//! ([`super::pieces::span_row`]), and halved at ½ by de Casteljau, along each axis, as often as its
//! schedule asks: each step a sum and an exact halving, outward at
//! `Interval`, so the halves enclose the exact quotient's at any degree
//! and the count has no exactness ceiling. No piece reads a refined
//! knot vector: as the pieces shrink, the coefficient ratios converge
//! on the quantity's range.
//! Each derivative row is read off the B-spline derivative net (its
//! knot differences span the whole support), not off a piece's own
//! differences, whose `degree/width` scale would amplify their rounding.
//!
//! **The weights are read over their greatest** per span or block: every
//! quotient is homogeneous of degree zero in the weights, so the scaling
//! is exact in ℝ (outward at `Interval`), and it keeps the weight
//! powers' products clear of underflow.
//!
//! **At equal weights** the curve meter passes the polynomial
//! derivative over the constant one ([`speed_pieces`] with no weights):
//! every `b_k` is exactly one, and the reading is the polynomial piece
//! row's own.
//!
//! # The second order
//!
//! The same statement, one order up. Differentiate `C′ = N/W²`:
//!
//! ```text
//! C″ = (N′·W − 2·N·W′) / W³,   N′ = A″·W + A′·W′ − A′·W′ − A·W″ = A″·W − A·W″
//! C″ = M / W³,   M = A″·W² − A·W″·W − 2·A′·W·W′ + 2·A·W′²
//! ```
//!
//! On a degree-`p` piece every term of `M` has degree `3p − 2`
//! (`A″` is `p − 2`, `A′` and `W′` are `p − 1`, `A` and `W` are `p`), and
//! `W³` has degree `3p`. A tensor patch reads it per direction, and
//! the mixed partial by differentiating `S_u = N_u/W²` along `v`:
//!
//! ```text
//! S_uu = M_uu / W³,  M_uu = A_uu·W² − A·W_uu·W − 2·A_u·W·W_u + 2·A·W_u²   (S_vv likewise)
//! S_uv = M_uv / W³,  M_uv = A_uv·W² − A·W_uv·W − A_u·W·W_v − A_v·W·W_u + 2·A·W_u·W_v
//! ```
//!
//! from `∂_v N_u = A_uv·W + A_u·W_v − A_v·W_u − A·W_uv` and
//! `S_uv = (∂_v N_u·W − 2·N_u·W_v)/W³`. Every term of `M_uu` has
//! bidegree `(3p − 2, 3q)`, of `M_uv` `(3p − 1, 3q − 1)`, against `W³`'s
//! `(3p, 3q)`.
//!
//! Then the first-order argument, clause by clause:
//! - **Raised to one degree.** `M` is raised to `W³`'s degree by
//!   elevation, which changes no polynomial, so `Σ B_k m_k / Σ B_k b_k`
//!   is `C″` itself and the `λ_k = B_k b_k / Σ_j B_j b_j` are its
//!   weights.
//! - **Each `m_k` over its own `b_k`.** `C″ = Σ_k λ_k·(m_k/b_k)` with
//!   `λ_k ≥ 0` summing to one, so `‖C″‖ ≤ max_k ‖m_k‖/b_k`, and each
//!   coordinate lies between its ratios'
//!   ([`QuotientPiece::coordinate_range`]).
//! - **The `b_k` are positive.** `W³`'s coefficients are Bernstein
//!   products of the piece's weight coefficients, which are convex
//!   combinations of the positive control weights.
//! - **Every term of `M` is kept.** A dropped term is a different
//!   polynomial: `A″·W²` alone is the polynomial bound, and the three
//!   weight terms are what make a rational curve's `C″` differ from it.
//!   At degree 1, `A″` and `W″` are identically zero and `M` is the
//!   doubled cross term alone.
//!
//! `C″` needs no first-order bound in between: `C′` enters `M` through
//! `A′` and `W′`, polynomially. The derivative rows are the B-spline's
//! derivative nets read twice ([`derivative_net`]'s scale at degree
//! `p − 1` over the vector from its second knot for the second
//! differences), each into the span's Bernstein row at its own degree,
//! and `M` and `W³` are formed once over the span and halved into the
//! pieces like the first order's.
//!
//! # Degree ceiling
//!
//! The Bernstein product of rows of degrees `m` and `n` has coefficients
//! `c_k = Σ_{i+j=k} C(m,i)·C(n,j)·a_i·b_j / C(m+n, k)`, and elevation is
//! the product with a row of ones. Every pair product `C(m,i)·C(n,j)` is
//! at most `C(m+n, k)` (Vandermonde), so every factor the statement
//! forms is an exact `f64` integer while the result degree is at most
//! [`QUOTIENT_MAX_DEGREE`] ([`super::pieces::binomial`]); above it the
//! builders refuse (`None`). At `Interval` each factor is a point and
//! every product and quotient rounds outward; at `f64` the same
//! expression rounds to nearest, and the reading can sit past the
//! bound by more than an ulp: measured up to `5.3e-12` relative at a
//! weight spread of `10³`, growing with the spread. `Interval` is the
//! certified lane.
//!
//! **The practical ceiling sits lower on a wide weight spread.** The
//! weight power's coefficients are products of interval weights, and at
//! degree 17 and above a spread near `10⁶` lets their enclosures reach
//! zero: the ratios then do not certify and the reading is a refusal,
//! not a bound. The second order (`3p`) meets it first.
//!
//! **No zero knot difference is reachable.** A span is walked only when
//! it is nonempty, and its active pairs are `i ∈ [s − p, s − 1]`
//! ([`Span::first_derived_window`], total): each pair's knot difference
//! `u_{i+p+1} − u_{i+1}` spans `[u_s, u_{s+1}]`, so it is positive.
//! Nothing here checks it.

use core::ops::{Add, Mul, Sub};

use crate::real::Real;

use super::knot_values::Knots;
use super::knots::Span;
use super::pieces::{binomial, span_row};

/// The degree ceiling of the quotient bound (module docs): the highest
/// common degree whose binomials are all exact `f64`s, `C(56, 28) < 2⁵³`.
pub const QUOTIENT_MAX_DEGREE: usize = 56;

/// `p / (hi − lo)` in `T`, for the pair whose knot difference runs from
/// `lo = u_{i+1}` to `hi = u_{i+p+1}`.
#[must_use]
pub fn span_derivative_scale<T: Real>(p: usize, lo: T, hi: T) -> T {
    #[allow(clippy::cast_precision_loss)]
    let p = T::from_f64(p as f64);
    p / (hi - lo)
}

/// The derivative net of the degree-`p` coefficients `net`, whose first
/// entry is coefficient `first`: entry `k` is
/// `(net[k+1] − net[k])·`[`span_derivative_scale`] with `i = first + k`.
/// The knot difference is checked positive on the structure `knots` and
/// formed in `T` over the `held` knots. `None` on a non-positive
/// difference or a knot index past the vector.
pub fn derivative_net<T: Real, P: Copy + Sub<Output = V>, V: Mul<T, Output = V>>(
    net: &[P],
    knots: &[f64],
    held: &[T],
    p: usize,
    first: usize,
) -> Option<Vec<V>> {
    net.windows(2)
        .enumerate()
        .map(|(k, pair)| {
            let i = first + k;
            let du = *knots.get(i + p + 1)? - *knots.get(i + 1)?;
            #[allow(clippy::neg_cmp_op_on_partial_ord)]
            if !(du > 0.0) {
                return None;
            }
            Some((pair[1] - pair[0]) * pair_scale(held, p, i)?)
        })
        .collect()
}

/// The derivative scale of pair `i` of a degree-`p` spline over the
/// `held` knot values: [`span_derivative_scale`] over
/// `u_{i+1} … u_{i+p+1}`, `None` past the vector.
fn pair_scale<T: Real>(held: &[T], p: usize, i: usize) -> Option<T> {
    Some(span_derivative_scale(
        p,
        *held.get(i + 1)?,
        *held.get(i + p + 1)?,
    ))
}

fn poison<T: Real>() -> T {
    T::from_f64(f64::NAN)
}

/// The Euclidean norm of a `D`-vector, `√Σ xₖ²`.
fn norm<T: Real, const D: usize>(v: [T; D]) -> T {
    v.iter()
        .map(|x| x.powi(2))
        .fold(T::zero(), |a, b| a + b)
        .sqrt()
}

fn dot<T: Real, const D: usize>(a: [T; D], b: [T; D]) -> T {
    a.iter().zip(b).fold(T::zero(), |s, (x, y)| s + *x * y)
}

fn cross<T: Real>(a: [T; 3], b: [T; 3]) -> [T; 3] {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}

/// **The quotient bound on one piece** (module docs): a quantity
/// `Σ B_k a_k / Σ B_k b_k`, its numerator's coefficients `a_k` and its
/// weight power's `b_k > 0` at one degree and one layout. Minted only
/// by this module's builders ([`speed_pieces`], [`cell_quotients`],
/// [`stretch_pieces`]), which hold both preconditions.
#[derive(Clone, Debug)]
pub struct QuotientPiece<T, const D: usize> {
    numer: Vec<[T; D]>,
    denom: Vec<T>,
}

impl<T: Real, const D: usize> QuotientPiece<T, D> {
    /// The piece with numerator coefficients `numer` and weight-power
    /// coefficients `denom`, paired by index; `None` when the counts
    /// differ or there is none. Private: one degree on both sides and a
    /// positive `denom` (module docs) are what the builders here hold,
    /// raising the numerator to the weight power's degree and forming
    /// the weight power as a product of positive weights.
    fn new(numer: Vec<[T; D]>, denom: Vec<T>) -> Option<Self> {
        (!numer.is_empty() && numer.len() == denom.len()).then_some(Self { numer, denom })
    }

    /// The numerator's coefficients.
    #[must_use]
    pub fn numer(&self) -> &[[T; D]] {
        &self.numer
    }

    /// The weight power's coefficients.
    #[must_use]
    pub fn denom(&self) -> &[T] {
        &self.denom
    }

    /// `max_k ‖a_k‖/b_k`, an upper bound on the quantity's norm over
    /// the piece.
    #[must_use]
    pub fn sup_norm(&self) -> T {
        self.ratios(norm).reduce(T::max).unwrap_or_else(poison)
    }

    /// `min_k (a_k·d)/b_k`, a lower bound on the quantity's projection
    /// on `d` over the piece; for a unit `d`, on its norm.
    #[must_use]
    pub fn floor_along(&self, d: [T; D]) -> T {
        self.ratios(|a| dot(a, d))
            .reduce(T::min)
            .unwrap_or_else(poison)
    }

    /// The piece's own direction: `Σ_k a_k` over its norm, poison when
    /// the sum is zero.
    #[must_use]
    pub fn direction(&self) -> [T; D] {
        let sum = self
            .numer
            .iter()
            .fold([T::zero(); D], |s, a| core::array::from_fn(|c| s[c] + a[c]));
        let n = norm(sum);
        sum.map(|x| x / n)
    }

    /// A lower bound on the quantity's norm over the piece:
    /// [`Self::floor_along`] its own [`Self::direction`]. Signed: a
    /// piece whose ratios straddle the direction reads at most zero;
    /// poison where the direction is.
    #[must_use]
    pub fn inf_norm(&self) -> T {
        self.floor_along(self.direction())
    }

    /// Per coordinate, `(min_k a_k^c/b_k, max_k a_k^c/b_k)`: each
    /// coordinate of the quantity is the same convex combination of its
    /// coordinate ratios (module docs), so it lies between them over the
    /// piece. A signed box; poison when there is no coefficient.
    #[must_use]
    pub fn coordinate_range(&self) -> [(T, T); D] {
        core::array::from_fn(|c| {
            let mut r = self.ratios(move |a| a[c]);
            let first = r.next().unwrap_or_else(poison);
            r.fold((first, first), |(lo, hi), x| (lo.min(x), hi.max(x)))
        })
    }

    fn ratios<'a>(&'a self, f: impl Fn([T; D]) -> T + 'a) -> impl Iterator<Item = T> + 'a {
        self.numer
            .iter()
            .zip(&self.denom)
            .map(move |(a, b)| f(*a) / *b)
    }
}

/// The three first-order quotients of a tensor patch on one piece:
/// `S_u`, `S_v` and, when asked for, `S_u × S_v`.
#[derive(Clone, Debug)]
pub struct CellQuotients<T> {
    /// `S_u = N_u/W²`.
    pub u: QuotientPiece<T, 3>,
    /// `S_v = N_v/W²`.
    pub v: QuotientPiece<T, 3>,
    /// `S_u × S_v = M̃/W³`, when the builder was asked for it.
    pub area: Option<QuotientPiece<T, 3>>,
}

/// A `D`-vector with the ring operations a blossom and a restriction
/// combine with; a scalar row is `D = 1`.
#[derive(Clone, Copy, Debug)]
struct Vector<T, const D: usize>([T; D]);

impl<T: Real, const D: usize> Add for Vector<T, D> {
    type Output = Self;
    fn add(self, o: Self) -> Self {
        Self(core::array::from_fn(|c| self.0[c] + o.0[c]))
    }
}

impl<T: Real, const D: usize> Sub for Vector<T, D> {
    type Output = Self;
    fn sub(self, o: Self) -> Self {
        Self(core::array::from_fn(|c| self.0[c] - o.0[c]))
    }
}

impl<T: Real, const D: usize> Mul<T> for Vector<T, D> {
    type Output = Self;
    fn mul(self, k: T) -> Self {
        Self(self.0.map(|x| x * k))
    }
}

impl<T: Real, const D: usize> Vector<T, D> {
    /// The vector times the exact integer `k`, skipped at one.
    fn times(self, k: f64) -> Self {
        if k == 1.0 {
            self
        } else {
            self * T::from_f64(k)
        }
    }

    /// The vector times the exact power of two `k`, skipped at one.
    fn scaled(self, k: f64) -> Self {
        if k == 1.0 {
            self
        } else {
            self * T::from_f64(k)
        }
    }

    /// The vector over the exact integer `k`, skipped at one.
    fn over(self, k: f64) -> Self {
        if k == 1.0 {
            self
        } else {
            let k = T::from_f64(k);
            Self(self.0.map(|x| x / k))
        }
    }
}

/// A tensor coefficient grid, `nu × nv`, row-major `[i·nv + j]` with
/// `i` along `u`; a curve's row is one column (`nv = 1`). Never empty.
#[derive(Clone, Debug)]
struct Grid<T, const D: usize> {
    nv: usize,
    c: Vec<Vector<T, D>>,
}

impl<T: Real, const D: usize> Grid<T, D> {
    /// The grid from its row-major coefficients, `None` unless `nv`
    /// divides a nonempty count.
    fn new(nv: usize, c: Vec<Vector<T, D>>) -> Option<Self> {
        (nv > 0 && !c.is_empty() && c.len().is_multiple_of(nv)).then_some(Self { nv, c })
    }

    /// A column of `n` coefficients.
    fn column(c: Vec<Vector<T, D>>) -> Option<Self> {
        Self::new(1, c)
    }

    fn nu(&self) -> usize {
        self.c.len() / self.nv
    }

    /// The degrees `(nu − 1, nv − 1)`.
    fn degree(&self) -> (usize, usize) {
        (self.nu() - 1, self.nv - 1)
    }

    fn get(&self, i: usize, j: usize) -> Option<Vector<T, D>> {
        (j < self.nv).then_some(())?;
        self.c.get(i.checked_mul(self.nv)?.checked_add(j)?).copied()
    }

    /// The columns (along `u`), each `nu` long.
    fn columns(&self) -> Option<Vec<Vec<Vector<T, D>>>> {
        (0..self.nv)
            .map(|j| (0..self.nu()).map(|i| self.get(i, j)).collect())
            .collect()
    }

    /// The grid whose columns are `cols`, all one length.
    fn from_columns(cols: &[Vec<Vector<T, D>>]) -> Option<Self> {
        let nu = cols.first()?.len();
        cols.iter().all(|c| c.len() == nu).then_some(())?;
        let c = (0..nu)
            .flat_map(|i| cols.iter().map(move |col| col.get(i).copied()))
            .collect::<Option<Vec<_>>>()?;
        Self::new(cols.len(), c)
    }

    /// The grid whose rows are `rows`, all one length.
    fn from_rows(rows: &[Vec<Vector<T, D>>]) -> Option<Self> {
        let nv = rows.first()?.len();
        rows.iter().all(|r| r.len() == nv).then_some(())?;
        Self::new(nv, rows.concat())
    }

    /// Every column mapped by `f` (along `u`).
    fn map_u(&self, f: impl Fn(&[Vector<T, D>]) -> Option<Vec<Vector<T, D>>>) -> Option<Self> {
        let cols: Vec<Vec<Vector<T, D>>> = self
            .columns()?
            .iter()
            .map(|c| f(c))
            .collect::<Option<_>>()?;
        Self::from_columns(&cols)
    }

    /// Every row mapped by `f` (along `v`).
    fn map_v(&self, f: impl Fn(&[Vector<T, D>]) -> Option<Vec<Vector<T, D>>>) -> Option<Self> {
        let rows: Vec<Vec<Vector<T, D>>> = self
            .c
            .chunks_exact(self.nv)
            .map(&f)
            .collect::<Option<_>>()?;
        Self::from_rows(&rows)
    }

    /// `self + other`, coefficient by coefficient, at one shape.
    fn plus(&self, other: &Self) -> Option<Self> {
        (self.nv == other.nv && self.c.len() == other.c.len()).then_some(())?;
        Self::new(
            self.nv,
            self.c.iter().zip(&other.c).map(|(a, b)| *a + *b).collect(),
        )
    }

    /// The grid times the exact integer `k`.
    fn times(&self, k: f64) -> Option<Self> {
        Self::new(self.nv, self.c.iter().map(|v| v.times(k)).collect())
    }

    /// `self − other`, coefficient by coefficient, at one shape.
    fn minus(&self, other: &Self) -> Option<Self> {
        (self.nv == other.nv && self.c.len() == other.c.len()).then_some(())?;
        Self::new(
            self.nv,
            self.c.iter().zip(&other.c).map(|(a, b)| *a - *b).collect(),
        )
    }

    /// The grid halved along `u` (`along_u`) or `v` at the parameter's
    /// midpoint, by de Casteljau: the two halves' Bernstein grids, lower
    /// first. Each line is summed unscaled and every output scaled once
    /// by its exact power `2⁻ʳ`, which reads every halving step's sums
    /// to the bit wherever no step leaves the normal range: a sum's
    /// rounding commutes with a power-of-two scale.
    fn halved(&self, along_u: bool) -> Option<(Self, Self)> {
        let (nu, nv) = (self.nu(), self.nv);
        let (n, lines) = if along_u { (nu, nv) } else { (nv, nu) };
        let at = |line: usize, k: usize| {
            if along_u {
                k * nv + line
            } else {
                line * nv + k
            }
        };
        let (mut lo, mut hi) = (self.c.clone(), self.c.clone());
        let mut tri = Vec::with_capacity(n);
        for line in 0..lines {
            tri.clear();
            tri.extend((0..n).map(|k| self.c[at(line, k)]));
            let mut scale = 1.0;
            for r in 0..n {
                if r > 0 {
                    for i in 0..n - r {
                        tri[i] = tri[i] + tri[i + 1];
                    }
                    scale *= 0.5;
                }
                *lo.get_mut(at(line, r))? = tri[0].scaled(scale);
                *hi.get_mut(at(line, n - 1 - r))? = tri[n - 1 - r].scaled(scale);
            }
        }
        Some((Self { nv, c: lo }, Self { nv, c: hi }))
    }

    /// The grid raised to the degrees `(du, dv)` by elevation: its
    /// product with a grid of ones.
    fn raised_to(&self, (du, dv): (usize, usize)) -> Option<Self> {
        let (mu, mv) = self.degree();
        let ones = Grid::new(
            dv.checked_sub(mv)? + 1,
            vec![Vector([T::one()]); (du.checked_sub(mu)? + 1) * (dv - mv + 1)],
        )?;
        product(self, &ones, |a, _| a)
    }

    fn arrays(&self) -> Vec<[T; D]> {
        self.c.iter().map(|v| v.0).collect()
    }
}

/// The largest power of two at most `splits`, never zero: how many
/// pieces halving cuts an axis into.
fn halving_count(splits: usize) -> usize {
    if splits == 0 { 1 } else { 1 << splits.ilog2() }
}

/// `whole` cut into `count` equal pieces along one axis by repeated
/// halving at ½ (`halve`), in ascending order. `count` is a power of two.
fn halvings<X>(whole: X, count: usize, halve: impl Fn(&X) -> Option<(X, X)>) -> Option<Vec<X>> {
    let mut out = vec![whole];
    while out.len() < count {
        out = out
            .iter()
            .map(&halve)
            .collect::<Option<Vec<_>>>()?
            .into_iter()
            .flat_map(|(lo, hi)| [lo, hi])
            .collect();
    }
    Some(out)
}

/// The Bernstein product of the grids `a` and `b` under the bilinear
/// `mul` (module docs: the degree ceiling): coefficient `(k, l)` is
/// `Σ C(ma,i)C(mb,k−i)·C(na,j)C(nb,l−j)·mul(a_ij, b_{k−i,l−j})` over
/// `C(ma+mb, k)·C(na+nb, l)`, folded in ascending `(i, j)` then
/// ascending `b` index (D9). `None` above [`QUOTIENT_MAX_DEGREE`].
fn product<T: Real, const A: usize, const B: usize, const C: usize>(
    a: &Grid<T, A>,
    b: &Grid<T, B>,
    mul: impl Fn([T; A], [T; B]) -> [T; C],
) -> Option<Grid<T, C>> {
    let ((ma, na), (mb, nb)) = (a.degree(), b.degree());
    let (mu, mv) = (ma + mb, na + nb);
    if mu > QUOTIENT_MAX_DEGREE || mv > QUOTIENT_MAX_DEGREE {
        return None;
    }
    let row = |m: usize| (0..=m).map(|i| binomial(m, i)).collect::<Vec<_>>();
    let (ca, cb, da, db) = (row(ma), row(mb), row(na), row(nb));
    let mut acc: Vec<Option<Vector<T, C>>> = vec![None; (mu + 1) * (mv + 1)];
    for (i, (x_row, ci)) in a.c.chunks_exact(na + 1).zip(&ca).enumerate() {
        for (j, (x, dj)) in x_row.iter().zip(&da).enumerate() {
            for (k, (y_row, ck)) in b.c.chunks_exact(nb + 1).zip(&cb).enumerate() {
                let slots = acc.get_mut((i + k) * (mv + 1) + j..)?;
                for ((y, dl), slot) in y_row.iter().zip(&db).zip(slots) {
                    let term = Vector(mul(x.0, y.0)).times(ci * ck).times(dj * dl);
                    *slot = Some(slot.map_or(term, |s| s + term));
                }
            }
        }
    }
    let (cu, cv) = (row(mu), row(mv));
    let c = acc
        .into_iter()
        .zip(cu.iter().flat_map(|u| cv.iter().map(move |v| (u, v))))
        .map(|(s, (u, v))| Some(s?.over(*u).over(*v)))
        .collect::<Option<Vec<_>>>()?;
    Grid::new(mv + 1, c)
}

fn scale_by<T: Real, const D: usize>(v: Vector<T, D>, w: Vector<T, 1>) -> Vector<T, D> {
    v * w.0[0]
}

/// The weights `ws` over their greatest. Every quotient is homogeneous
/// of degree zero in the weights (`N/W²` and `M̃/W³` scale alike), so
/// this is exact in ℝ and changes no bound, while the weight powers'
/// products stay clear of underflow and overflow wherever the weights'
/// spread allows: a block of equal subnormal weights reads as one of
/// ones. `None` when there is none.
fn normalised<T: Real>(ws: &[T]) -> Option<(Vec<T>, T)> {
    let most = ws.iter().copied().reduce(T::max)?;
    Some((ws.iter().map(|w| *w / most).collect(), most))
}

/// One axis of a cell block: a span of a knot vector, whose window
/// the blossom reads into the span's Bernstein row.
#[derive(Clone, Copy, Debug)]
pub enum CellAxis<'a, T> {
    /// The window of `span`, over the `held` knot values of the vector
    /// it indexes.
    Span(Span<'a>, &'a [T]),
}

/// A [`CellAxis`] resolved: its degree and how it reads a window.
#[derive(Clone, Copy)]
enum Axis<'a, T> {
    Span {
        held: &'a [T],
        p: usize,
        first: usize,
        ends: (T, T),
    },
}

impl<'a, T: Real> Axis<'a, T> {
    fn of(axis: CellAxis<'a, T>) -> Option<Self> {
        Some(match axis {
            CellAxis::Span(span, held) => {
                let s = span.index();
                Self::Span {
                    held,
                    p: span.degree(),
                    first: span.first_control(),
                    ends: (*held.get(s)?, *held.get(s + 1)?),
                }
            }
        })
    }

    fn p(&self) -> usize {
        match *self {
            Self::Span { p, .. } => p,
        }
    }

    /// The span's Bernstein row of a window of this axis's degree less
    /// `order`: of its `order`-th derivative's window. The derived
    /// spline of degree `p − r` reads the same knot vector from index
    /// `first + r` (Eq. 3.4).
    fn row<const D: usize>(&self, w: &[Vector<T, D>], order: usize) -> Option<Vec<Vector<T, D>>> {
        match *self {
            Self::Span {
                held, first, ends, ..
            } => span_row(w, held, first + order, ends),
        }
    }

    /// The derivative scale of the window's pair `k`: the span's
    /// [`pair_scale`] (its knot difference positive by the span
    /// contract, module docs).
    fn scale(&self, k: usize) -> Option<T> {
        self.scale_at(1, k)
    }

    /// The derivative scale of pair `k` of the window differenced
    /// `order − 1` times: the degree-`p − order + 1` spline's own
    /// [`pair_scale`], read over the knot vector from index `order − 1`
    /// (on a span, `(p − r)/(u_{i+p+1} − u_{i+r+1})` with `r = order − 1`;
    /// positive by the same span contract).
    fn scale_at(&self, order: usize, k: usize) -> Option<T> {
        let r = order.checked_sub(1)?;
        match *self {
            Self::Span { held, p, first, .. } => {
                pair_scale(held.get(r..)?, p.checked_sub(r)?, first + k)
            }
        }
    }

    /// A window already differenced `order − 1` times, differenced once
    /// more: each pair's difference times its [`Self::scale_at`].
    fn derive_at<const D: usize>(
        &self,
        order: usize,
        w: &[Vector<T, D>],
    ) -> Option<Vec<Vector<T, D>>> {
        w.windows(2)
            .enumerate()
            .map(|(k, x)| Some((x[1] - x[0]) * self.scale_at(order, k)?))
            .collect()
    }

    /// The window's derivative window: each pair's difference times
    /// its [`Self::scale`].
    fn derive<const D: usize>(&self, w: &[Vector<T, D>]) -> Option<Vec<Vector<T, D>>> {
        w.windows(2)
            .enumerate()
            .map(|(k, x)| Some((x[1] - x[0]) * self.scale(k)?))
            .collect()
    }

    /// The derivative windows of the homogeneous `w·(P − c)` and of
    /// `w`, read off the points and weights in the pair form
    /// `w₁·(P₁ − P₀) + (w₁ − w₀)·(P₀ − c)`: every point enters as a
    /// difference with another, so equal points with equal weights read
    /// an exact zero at any scalar, and a translation moves a term by
    /// its rounding alone.
    #[allow(clippy::type_complexity)]
    fn derive_pairs<const D: usize>(
        &self,
        pts: &[Vector<T, D>],
        ws: &[Vector<T, 1>],
        c: Vector<T, D>,
    ) -> Option<(Vec<Vector<T, D>>, Vec<Vector<T, 1>>)> {
        pts.windows(2)
            .zip(ws.windows(2))
            .enumerate()
            .map(|(k, (p, w))| {
                let s = self.scale(k)?;
                let dw = w[1] - w[0];
                Some((
                    (scale_by(p[1] - p[0], w[1]) + scale_by(p[0] - c, dw)) * s,
                    dw * s,
                ))
            })
            .collect::<Option<Vec<_>>>()
            .map(|v| v.into_iter().unzip())
    }
}

/// The span's Bernstein grid of a window grid, along `u` with `du`'s
/// degree less the derivative order `du`, and along `v` with `dv`'s.
fn cell_grid<T: Real, const D: usize>(
    g: &Grid<T, D>,
    (au, du): (&Axis<'_, T>, usize),
    (av, dv): (&Axis<'_, T>, usize),
) -> Option<Grid<T, D>> {
    g.map_u(|col| au.row(col, du))?.map_v(|row| av.row(row, dv))
}

/// A cell's homogeneous Bernstein grids on one piece: `A`, `W` and
/// their `u` and `v` derivatives, and, when asked for, their second
/// ones.
struct Homogeneous<T> {
    a: Grid<T, 3>,
    w: Grid<T, 1>,
    au: Grid<T, 3>,
    wu: Grid<T, 1>,
    av: Grid<T, 3>,
    wv: Grid<T, 1>,
    second: Option<SecondGrids<T>>,
}

/// The second derivatives of a cell's `A` and `W`: `uu` absent along a
/// degree-1 `u` (identically zero there), `vv` likewise.
struct SecondGrids<T> {
    uu: Option<(Grid<T, 3>, Grid<T, 1>)>,
    uv: (Grid<T, 3>, Grid<T, 1>),
    vv: Option<(Grid<T, 3>, Grid<T, 1>)>,
}

/// The three second-order quotients of a tensor patch on one piece,
/// each over `W³` (module docs, "The second order").
#[derive(Clone, Debug)]
pub struct CellSecondQuotients<T> {
    /// `S_uu`.
    pub uu: QuotientPiece<T, 3>,
    /// `S_uv`.
    pub uv: QuotientPiece<T, 3>,
    /// `S_vv`.
    pub vv: QuotientPiece<T, 3>,
}

impl<T: Real> Homogeneous<T> {
    /// The second-order quotients on this piece (module docs, "The
    /// second order"):
    /// `M_uu = A_uu·W² − A·W_uu·W − 2·A_u·W·W_u + 2·A·W_u²` and `M_vv`
    /// likewise, `M_uv = A_uv·W² − A·W_uv·W − A_u·W·W_v − A_v·W·W_u +
    /// 2·A·W_u·W_v`, each over `W³`. `None` when the grids were not
    /// built.
    fn second_raised(&self) -> Option<SecondRaised<T>> {
        let g = self.second.as_ref()?;
        let (a, w) = (&self.a, &self.w);
        let diagonal = |d: &Grid<T, 3>, dw: &Grid<T, 1>, xy: Option<&(Grid<T, 3>, Grid<T, 1>)>| {
            let cross = doubled_cross(a, w, d, dw)?;
            let (m, w3) = second_numerator(a, w, xy.map(|(x, y)| (x, y)), cross)?;
            Raised::of(&m, w3)
        };
        let uu = diagonal(&self.au, &self.wu, g.uu.as_ref())?;
        let vv = diagonal(&self.av, &self.wv, g.vv.as_ref())?;
        let cross = a_yz(a, &self.wu, &self.wv)?
            .times(2.0)?
            .minus(&x_wy(&self.au, w, &self.wv)?)?
            .minus(&x_wy(&self.av, w, &self.wu)?)?;
        let (m, w3) = second_numerator(a, w, Some((&g.uv.0, &g.uv.1)), cross)?;
        let uv = Raised::of(&m, w3)?;
        Some(SecondRaised { uu, uv, vv })
    }

    /// The first-order quotients over the block (module docs):
    /// `N_u = A_u·W − A·W_u` and `N_v` over `W²`, and `M̃` over `W³` when
    /// `area`, each raised to its weight power's degrees.
    fn first_raised(&self, area: bool) -> Option<FirstRaised<T>> {
        let first = |d: &Grid<T, 3>, dw: &Grid<T, 1>| -> Option<Grid<T, 3>> {
            product(d, &self.w, by_weight)?.minus(&product(&self.a, dw, by_weight)?)
        };
        let w2 = product(&self.w, &self.w, weight_times)?;
        let u = Raised::of(&first(&self.au, &self.wu)?, w2.clone())?;
        let v = Raised::of(&first(&self.av, &self.wv)?, w2.clone())?;
        let area = if area {
            Some(self.area_raised()?)
        } else {
            None
        };
        Some(FirstRaised { u, v, area })
    }

    /// `S_u × S_v = M̃/W³` before it is cut.
    fn area_raised(&self) -> Option<Raised<T, 3>> {
        let w3 = product(
            &product(&self.w, &self.w, weight_times)?,
            &self.w,
            weight_times,
        )?;
        let scaled = |w: &Grid<T, 1>, x: &Grid<T, 3>| product(w, x, weighted);
        let m = scaled(&self.w, &product(&self.au, &self.av, cross)?)?
            .minus(&scaled(&self.wv, &product(&self.au, &self.a, cross)?)?)?
            .minus(&scaled(&self.wu, &product(&self.a, &self.av, cross)?)?)?;
        Raised::of(&m, w3)
    }
}

/// A quotient before it is cut into pieces: its numerator raised to its
/// weight power's degrees, and the weight power, formed once over the
/// block. Restricting both to a piece gives the piece's own
/// coefficients at that degree (restriction commutes with the ring
/// operations and preserves degree), so a quotient raised once and cut
/// is the statement on each piece.
#[derive(Clone, Debug)]
struct Raised<T, const D: usize> {
    numer: Grid<T, D>,
    denom: Grid<T, 1>,
}

impl<T: Real, const D: usize> Raised<T, D> {
    fn of(numer: &Grid<T, D>, denom: Grid<T, 1>) -> Option<Self> {
        Some(Self {
            numer: numer.raised_to(denom.degree())?,
            denom,
        })
    }

    /// The quotient halved along `u` (`along_u`) or `v`: numerator and
    /// weight power halved alike, so each half is the statement on its
    /// half of the block.
    fn halved(&self, along_u: bool) -> Option<(Self, Self)> {
        let (n0, n1) = self.numer.halved(along_u)?;
        let (d0, d1) = self.denom.halved(along_u)?;
        Some((
            Self {
                numer: n0,
                denom: d0,
            },
            Self {
                numer: n1,
                denom: d1,
            },
        ))
    }

    /// The quotient on `counts` equal pieces per axis (powers of two),
    /// by halving at ½ along `u` and then along `v`, row-major in `u`
    /// then `v`.
    fn pieces(&self, counts: (usize, usize)) -> Option<Vec<QuotientPiece<T, D>>> {
        let along_u = halvings(self.clone(), counts.0, |r| r.halved(true))?;
        let mut out = Vec::with_capacity(counts.0 * counts.1);
        for r in along_u {
            for piece in halvings(r, counts.1, |r| r.halved(false))? {
                out.push(piece.piece()?);
            }
        }
        Some(out)
    }

    fn piece(&self) -> Option<QuotientPiece<T, D>> {
        QuotientPiece::new(
            self.numer.arrays(),
            self.denom.c.iter().map(|w| w.0[0]).collect(),
        )
    }
}

/// [`CellQuotients`] before it is cut.
struct FirstRaised<T> {
    u: Raised<T, 3>,
    v: Raised<T, 3>,
    area: Option<Raised<T, 3>>,
}

/// [`CellSecondQuotients`] before it is cut.
struct SecondRaised<T> {
    uu: Raised<T, 3>,
    uv: Raised<T, 3>,
    vv: Raised<T, 3>,
}

/// **The curve statement's pieces** (module docs): on each of the
/// nonempty spans of the spline with control `points` over `knots`, its
/// derivative's quotient on each of the largest power of two at most
/// `splits` equal pieces, at every degree, halved from the quotient
/// formed once over the span, in ascending span then piece order. With
/// `weights`, `C′ = N/W²` (`N` of degree `2p − 1` raised to `2p`);
/// without, the polynomial derivative over one (degree `p − 1`, every
/// `b_k` exactly one).
/// `None` on a degree below one, an index past an array, or a common
/// degree above [`QUOTIENT_MAX_DEGREE`]; empty when no span is
/// nonempty.
pub fn speed_pieces<T: Real, const D: usize>(
    knots: &Knots<T>,
    points: &[[T; D]],
    weights: Option<&[T]>,
    splits: usize,
) -> Option<Vec<QuotientPiece<T, D>>> {
    let kv = knots.structure();
    let mut out = Vec::new();
    for span in (kv.first_span()..=kv.last_span()).filter_map(|i| kv.span(i)) {
        let (axis, p, pts) = span_window(knots, span, points)?;
        let raised = match weights {
            None => {
                let numer = Grid::column(axis.row(&axis.derive(&pts)?, 1)?)?;
                let denom = Grid::column(vec![Vector([T::one()]); p])?;
                Raised { numer, denom }
            }
            Some(ws) => {
                let rows = HomogeneousSpan::of(&axis, &pts, ws.get(span.window())?)?.rows;
                let (a, w) = (Grid::column(rows.0)?, Grid::column(rows.1)?);
                let (da, dw) = (Grid::column(rows.2)?, Grid::column(rows.3)?);
                let n = product(&da, &w, by_weight)?.minus(&product(&a, &dw, by_weight)?)?;
                Raised::of(&n, product(&w, &w, weight_times)?)?
            }
        };
        out.extend(raised.pieces((halving_count(splits), 1))?);
    }
    Some(out)
}

/// `x·w` for a vector row `x` and a scalar row `w`: the bilinear map of
/// a weight-scaled [`product`].
fn by_weight<T: Real, const D: usize>(x: [T; D], w: [T; 1]) -> [T; D] {
    x.map(|c| c * w[0])
}

/// One nonempty span's resolved axis, degree and control window: the
/// preamble of every curve quotient. `None` on a degree below one or an
/// index past `points`.
#[allow(clippy::type_complexity)]
fn span_window<'k, T: Real, const D: usize>(
    knots: &'k Knots<T>,
    span: Span<'k>,
    points: &[[T; D]],
) -> Option<(Axis<'k, T>, usize, Vec<Vector<T, D>>)> {
    let axis = Axis::of(CellAxis::Span(span, knots.values()))?;
    let p = axis.p();
    (p > 0).then_some(())?;
    let pts = points
        .get(span.window())?
        .iter()
        .map(|x| Vector(*x))
        .collect();
    Some((axis, p, pts))
}

/// One span's homogeneous setup, which every rational curve quotient
/// reads: the weights over their greatest, `A = w·(P − c)` in the frame
/// of the span's first point, the pair-form derivative windows of `A`
/// and `w` ([`Axis::derive_pairs`]), and the Bernstein rows of all four.
struct HomogeneousSpan<T, const D: usize> {
    da: Vec<Vector<T, D>>,
    dw: Vec<Vector<T, 1>>,
    rows: RowSet<T, D>,
}

/// The Bernstein rows of `A`, `W`, `A′` and `W′` on one span.
type RowSet<T, const D: usize> = (
    Vec<Vector<T, D>>,
    Vec<Vector<T, 1>>,
    Vec<Vector<T, D>>,
    Vec<Vector<T, 1>>,
);

impl<T: Real, const D: usize> HomogeneousSpan<T, D> {
    fn of(axis: &Axis<'_, T>, pts: &[Vector<T, D>], ws: &[T]) -> Option<Self> {
        let w: Vec<Vector<T, 1>> = normalised(ws)?.0.into_iter().map(|w| Vector([w])).collect();
        let c = *pts.first()?;
        let a = pts
            .iter()
            .zip(&w)
            .map(|(x, w)| scale_by(*x - c, *w))
            .collect::<Vec<_>>();
        let (da, dw) = axis.derive_pairs(pts, &w, c)?;
        let rows = (
            axis.row(&a, 0)?,
            axis.row(&w, 0)?,
            axis.row(&da, 1)?,
            axis.row(&dw, 1)?,
        );
        Some(Self { da, dw, rows })
    }
}

/// `w·v` for two scalar rows.
fn weight_times<T: Real>(w: [T; 1], v: [T; 1]) -> [T; 1] {
    [w[0] * v[0]]
}

/// [`by_weight`] with the weight row first.
fn weighted<T: Real, const D: usize>(w: [T; 1], x: [T; D]) -> [T; D] {
    by_weight(x, w)
}

/// `A·Y·Z` for scalar grids `Y`, `Z`.
fn a_yz<T: Real, const D: usize>(
    a: &Grid<T, D>,
    y: &Grid<T, 1>,
    z: &Grid<T, 1>,
) -> Option<Grid<T, D>> {
    product(a, &product(y, z, weight_times)?, by_weight)
}

/// `X·W·Y` for a grid `X` and scalar grids `W`, `Y`.
fn x_wy<T: Real, const D: usize>(
    x: &Grid<T, D>,
    w: &Grid<T, 1>,
    y: &Grid<T, 1>,
) -> Option<Grid<T, D>> {
    product(x, &product(w, y, weight_times)?, by_weight)
}

/// `2·(A·W′² − A′·W·W′)`: the doubled cross term of a pure second
/// derivative's numerator `M = A″·W² − A·W″·W − 2·A′·W·W′ + 2·A·W′²`
/// (module docs, "The second order"), the curve's and each surface
/// direction's.
fn doubled_cross<T: Real, const D: usize>(
    a: &Grid<T, D>,
    w: &Grid<T, 1>,
    da: &Grid<T, D>,
    dw: &Grid<T, 1>,
) -> Option<Grid<T, D>> {
    a_yz(a, dw, dw)?.minus(&x_wy(da, w, dw)?)?.times(2.0)
}

/// The second-order numerator over `W³` (module docs, "The second
/// order"): `M = X·W² − A·Y·W − 2·Xa·W·Ya + 2·A·Ya·Yb`, at one degree,
/// for the second partial along `(a, b)`: `X = A_ab`, `Y = W_ab`
/// (absent along a degree-1 direction's own second partial, where both
/// are identically zero), `Xa = A_a`, `Ya = W_a`, `Yb = W_b`. At `a = b`
/// the `Xa·W·Ya` term is the one doubled cross term; at `a ≠ b` the
/// caller passes the two halves of it as the cross form. Returns the
/// numerator and `W³`.
#[allow(clippy::many_single_char_names)]
fn second_numerator<T: Real, const D: usize>(
    a: &Grid<T, D>,
    w: &Grid<T, 1>,
    xy: Option<(&Grid<T, D>, &Grid<T, 1>)>,
    cross: Grid<T, D>,
) -> Option<(Grid<T, D>, Grid<T, 1>)> {
    let w2 = product(w, w, weight_times)?;
    let w3 = product(&w2, w, weight_times)?;
    let m = match xy {
        Some((x, y)) => product(x, &w2, by_weight)?
            .minus(&a_yz(a, y, w)?)?
            .plus(&cross)?,
        None => cross,
    };
    Some((m, w3))
}

/// **The curve's second-order pieces** (module docs, "The second
/// order"): on each nonempty span of the RATIONAL spline with control
/// `points` and `weights` over `knots`, `C″ = M/W³` on each of the
/// largest power of two at most `splits` equal pieces, halved from `M`
/// and `W³` formed once over the span, in ascending span then piece
/// order, `M` of degree `3p − 2` raised to `3p`. `None` on a
/// degree below one, an index past an array, or a common degree above
/// [`QUOTIENT_MAX_DEGREE`]; empty when no span is nonempty.
pub fn accel_pieces<T: Real, const D: usize>(
    knots: &Knots<T>,
    points: &[[T; D]],
    weights: &[T],
    splits: usize,
) -> Option<Vec<QuotientPiece<T, D>>> {
    let kv = knots.structure();
    let mut out = Vec::new();
    for span in (kv.first_span()..=kv.last_span()).filter_map(|i| kv.span(i)) {
        let (axis, p, pts) = span_window(knots, span, points)?;
        let h = HomogeneousSpan::of(&axis, &pts, weights.get(span.window())?)?;
        let second = if p >= 2 {
            Some((
                axis.row(&axis.derive_at(2, &h.da)?, 2)?,
                axis.row(&axis.derive_at(2, &h.dw)?, 2)?,
            ))
        } else {
            None
        };
        let rows = h.rows;
        let (a, w) = (Grid::column(rows.0)?, Grid::column(rows.1)?);
        let (da, dw) = (Grid::column(rows.2)?, Grid::column(rows.3)?);
        let cross = doubled_cross(&a, &w, &da, &dw)?;
        let xy = match second {
            Some((dda, ddw)) => Some((Grid::column(dda)?, Grid::column(ddw)?)),
            None => None,
        };
        let (m, w3) = second_numerator(&a, &w, xy.as_ref().map(|(x, y)| (x, y)), cross)?;
        out.extend(Raised::of(&m, w3)?.pieces((halving_count(splits), 1))?);
    }
    Some(out)
}

/// A cell block's control net, row-major `(p + 1) × (q + 1)`.
#[derive(Clone, Copy, Debug)]
pub enum CellBlock<'a, T> {
    /// Cartesian points and weights, read in the frame of `origin`:
    /// the derivatives in the pair form `w₁·(P₁ − P₀) + (w₁ − w₀)·(P₀ − c)`,
    /// so equal points with equal weights read an exact zero.
    Points {
        /// The control points.
        points: &'a [[T; 3]],
        /// Their weights.
        weights: &'a [T],
        /// The frame `c`, any one point.
        origin: [T; 3],
    },
    /// The homogeneous channels `w·(P − c)` and `w`, for one frame `c`:
    /// the derivatives are their differences.
    Homogeneous {
        /// `w·(P − c)`.
        a: &'a [[T; 3]],
        /// `w`.
        w: &'a [T],
    },
}

/// **The surface statement's pieces on one cell block** (module docs):
/// the `block` along `axes`, its quotients formed once and halved into
/// the largest power of two at most `splits.0` pieces along `u` and
/// `splits.1` along `v`, row-major in `u` then `v`. `area` asks for `S_u × S_v` as well. `None` on a degree
/// below one, a block of the wrong size, or a common degree above
/// [`QUOTIENT_MAX_DEGREE`].
pub fn cell_quotients<T: Real>(
    axes: (CellAxis<'_, T>, CellAxis<'_, T>),
    block: CellBlock<'_, T>,
    splits: (usize, usize),
    area: bool,
) -> Option<Vec<CellQuotients<T>>> {
    Some(
        cell_pieces(axes, block, splits, area, false)?
            .1
            .into_iter()
            .map(|(first, _)| first)
            .collect(),
    )
}

/// **The surface statement's pieces at both orders** on one cell block
/// (module docs, "The second order"): [`cell_quotients`] with
/// `S_u × S_v`, and beside each piece its [`CellSecondQuotients`], with
/// the piece count per direction (as [`cell_quotients`]'). `None` as [`cell_quotients`], with
/// the common degree `3·max(p, q)` against [`QUOTIENT_MAX_DEGREE`].
#[allow(clippy::type_complexity)]
pub fn cell_second_quotients<T: Real>(
    axes: (CellAxis<'_, T>, CellAxis<'_, T>),
    block: CellBlock<'_, T>,
    splits: (usize, usize),
) -> Option<(
    (usize, usize),
    Vec<(CellQuotients<T>, CellSecondQuotients<T>)>,
)> {
    let (counts, pieces) = cell_pieces(axes, block, splits, true, true)?;
    let pieces = pieces
        .into_iter()
        .map(|(first, second)| Some((first, second?)))
        .collect::<Option<_>>()?;
    Some((counts, pieces))
}

/// **The integral area's pieces** on one cell block: with unit
/// weights `S_u × S_v = A_u × A_v`, a polynomial of degree
/// `(2p − 1, 2q − 1)`, read as a quotient over one (every `b_k` exactly
/// one), halved into pieces as [`cell_quotients`]' are, row-major
/// in `u` then `v`. The rational form's `M̃/W³` would carry degree `3p`
/// for nothing, so an integral patch reaches `p = 28` here, where the
/// rational area stops at `18`. `None` on a degree below one, a block
/// of the wrong size, or a degree above [`QUOTIENT_MAX_DEGREE`].
pub fn polynomial_area_pieces<T: Real>(
    axes: (CellAxis<'_, T>, CellAxis<'_, T>),
    points: &[[T; 3]],
    splits: (usize, usize),
) -> Option<Vec<QuotientPiece<T, 3>>> {
    let (au, av) = (Axis::of(axes.0)?, Axis::of(axes.1)?);
    let (p, q) = (au.p(), av.p());
    if p == 0 || q == 0 || points.len() != (p + 1) * (q + 1) {
        return None;
    }
    let c = Vector(*points.first()?);
    let g = Grid::new(q + 1, points.iter().map(|x| Vector(*x) - c).collect())?;
    let du = cell_grid(&g.map_u(|col| au.derive(col))?, (&au, 1), (&av, 0))?;
    let dv = cell_grid(&g.map_v(|row| av.derive(row))?, (&au, 0), (&av, 1))?;
    let numer = product(&du, &dv, cross)?;
    let (mu, mv) = numer.degree();
    let denom = Grid::new(mv + 1, vec![Vector([T::one()]); (mu + 1) * (mv + 1)])?;
    Raised { numer, denom }.pieces((halving_count(splits.0), halving_count(splits.1)))
}

#[allow(clippy::type_complexity)]
fn cell_pieces<T: Real>(
    axes: (CellAxis<'_, T>, CellAxis<'_, T>),
    block: CellBlock<'_, T>,
    splits: (usize, usize),
    area: bool,
    second: bool,
) -> Option<CellPieces<T>> {
    let whole = cell_whole(axes, block, second)?;
    // Every quotient is formed once over the cell and halved into its
    // pieces. With the second order the area is formed whatever `area`
    // asks: it shares `W³` with the second partials, and
    // `CellSecondQuotients` readers take it beside them.
    let counts = (halving_count(splits.0), halving_count(splits.1));
    let first = whole.first_raised(area || whole.second.is_some())?;
    let (u, v) = (first.u.pieces(counts)?, first.v.pieces(counts)?);
    let area: Vec<_> = match &first.area {
        Some(a) => a.pieces(counts)?.into_iter().map(Some).collect(),
        None => vec![None; u.len()],
    };
    let second: Vec<_> = match whole.second_raised() {
        Some(s) => {
            let (uu, uv, vv) = (
                s.uu.pieces(counts)?,
                s.uv.pieces(counts)?,
                s.vv.pieces(counts)?,
            );
            uu.into_iter()
                .zip(uv)
                .zip(vv)
                .map(|((uu, uv), vv)| Some(CellSecondQuotients { uu, uv, vv }))
                .collect()
        }
        None => vec![None; u.len()],
    };
    let out = u
        .into_iter()
        .zip(v)
        .zip(area)
        .zip(second)
        .map(|(((u, v), area), second)| (CellQuotients { u, v, area }, second))
        .collect();
    Some((counts, out))
}

/// **A cell block's first-order quotients, formed once** (module docs):
/// `S_u = N_u/W²` and `S_v = N_v/W²` over the whole block, each raised
/// to `W²`'s bidegree. [`Self::pieces`] halves them into a schedule's
/// pieces, as [`cell_quotients`] reads them; at the interval scalar
/// [`CellFirstQuotients::restrict`] first cuts them to a sub-rectangle,
/// restriction commuting with the ring operations that formed them.
#[derive(Clone, Debug)]
pub struct CellFirstQuotients<T> {
    u: Raised<T, 3>,
    v: Raised<T, 3>,
}

/// The first-order quotients of `block` along `axes`, formed once over
/// the block. `None` as [`cell_quotients`].
pub fn cell_first_quotients<T: Real>(
    axes: (CellAxis<'_, T>, CellAxis<'_, T>),
    block: CellBlock<'_, T>,
) -> Option<CellFirstQuotients<T>> {
    let first = cell_whole(axes, block, false)?.first_raised(false)?;
    Some(CellFirstQuotients {
        u: first.u,
        v: first.v,
    })
}

impl<T: Real> CellFirstQuotients<T> {
    /// `S_u` and `S_v` on the largest power of two at most `splits.0`
    /// pieces along `u` and `splits.1` along `v`, row-major in `u` then
    /// `v`, the area absent.
    #[must_use]
    pub fn pieces(&self, splits: (usize, usize)) -> Option<Vec<CellQuotients<T>>> {
        let counts = (halving_count(splits.0), halving_count(splits.1));
        let (u, v) = (self.u.pieces(counts)?, self.v.pieces(counts)?);
        Some(
            u.into_iter()
                .zip(v)
                .map(|(u, v)| CellQuotients { u, v, area: None })
                .collect(),
        )
    }
}

impl CellFirstQuotients<crate::interval::Interval> {
    /// The quotients on the part of the block over `[a, b]` along each
    /// axis given a cut, the block spanning `[s0, s1]` along it: every
    /// line of every numerator and weight power restricted to the cut
    /// ([`super::restrict_segment`]), at the exact cut ends. An axis
    /// given `None` is kept whole. A cut out of its span refuses every
    /// coefficient it restricts.
    #[must_use]
    pub fn restrict(&self, cuts: (Option<AxisCut>, Option<AxisCut>)) -> Self {
        Self {
            u: self.u.restricted(cuts),
            v: self.v.restricted(cuts),
        }
    }
}

/// One axis's cut: the span `(s0, s1)` the block covers and the part
/// `(a, b)` of it to keep.
pub type AxisCut = ((f64, f64), (f64, f64));

impl<const D: usize> Raised<crate::interval::Interval, D> {
    fn restricted(&self, cuts: (Option<AxisCut>, Option<AxisCut>)) -> Self {
        Self {
            numer: self.numer.restricted(cuts),
            denom: self.denom.restricted(cuts),
        }
    }
}

impl<const D: usize> Grid<crate::interval::Interval, D> {
    /// The grid restricted along each axis given a cut.
    fn restricted(&self, (u, v): (Option<AxisCut>, Option<AxisCut>)) -> Self {
        let g = u.map_or_else(|| self.clone(), |c| self.restricted_along(true, c));
        match v {
            Some(c) => g.restricted_along(false, c),
            None => g,
        }
    }

    /// Every line along `u` (`along_u`) or `v` restricted to the cut,
    /// channel by channel.
    fn restricted_along(&self, along_u: bool, (span, ends): AxisCut) -> Self {
        let (nu, nv) = (self.nu(), self.nv);
        let (n, lines) = if along_u { (nu, nv) } else { (nv, nu) };
        let at = |line: usize, k: usize| {
            if along_u {
                k * nv + line
            } else {
                line * nv + k
            }
        };
        let mut c = self.c.clone();
        for line in 0..lines {
            for d in 0..D {
                let row: Vec<_> = (0..n).map(|k| self.c[at(line, k)].0[d]).collect();
                for (k, x) in super::restrict_segment(&row, span, ends)
                    .into_iter()
                    .enumerate()
                {
                    c[at(line, k)].0[d] = x;
                }
            }
        }
        Self { nv, c }
    }
}

/// A cell block's homogeneous grids over the whole block, with the
/// second derivatives when `second`.
fn cell_whole<T: Real>(
    axes: (CellAxis<'_, T>, CellAxis<'_, T>),
    block: CellBlock<'_, T>,
    second: bool,
) -> Option<Homogeneous<T>> {
    let (au, av) = (Axis::of(axes.0)?, Axis::of(axes.1)?);
    let (p, q) = (au.p(), av.p());
    let count = (p + 1) * (q + 1);
    if p == 0 || q == 0 {
        return None;
    }
    let grid3 = |x: &[[T; 3]]| Grid::new(q + 1, x.iter().map(|v| Vector(*v)).collect());
    let grid1 = |x: &[T]| Grid::new(q + 1, x.iter().map(|v| Vector([*v])).collect());
    let (a, w, (au_w, wu_w), (av_w, wv_w)) = match block {
        CellBlock::Points {
            points,
            weights,
            origin,
        } => {
            if points.len() != count || weights.len() != count {
                return None;
            }
            let (pg, wg, c) = (
                grid3(points)?,
                grid1(&normalised(weights)?.0)?,
                Vector(origin),
            );
            let a = Grid::new(
                q + 1,
                pg.c.iter()
                    .zip(&wg.c)
                    .map(|(x, w)| scale_by(*x - c, *w))
                    .collect(),
            )?;
            let (pc, wc) = (pg.columns()?, wg.columns()?);
            let along_u = pc
                .iter()
                .zip(&wc)
                .map(|(x, w)| au.derive_pairs(x, w, c))
                .collect::<Option<Vec<_>>>()?;
            let along_v =
                pg.c.chunks_exact(q + 1)
                    .zip(wg.c.chunks_exact(q + 1))
                    .map(|(x, w)| av.derive_pairs(x, w, c))
                    .collect::<Option<Vec<_>>>()?;
            let (ua, uw): (Vec<_>, Vec<_>) = along_u.into_iter().unzip();
            let (va, vw): (Vec<_>, Vec<_>) = along_v.into_iter().unzip();
            (
                a,
                wg,
                (Grid::from_columns(&ua)?, Grid::from_columns(&uw)?),
                (Grid::from_rows(&va)?, Grid::from_rows(&vw)?),
            )
        }
        CellBlock::Homogeneous { a, w } => {
            if a.len() != count || w.len() != count {
                return None;
            }
            let (w, most) = normalised(w)?;
            let a: Vec<[T; 3]> = a.iter().map(|x| x.map(|c| c / most)).collect();
            let (a, w) = (grid3(&a)?, grid1(&w)?);
            let u = (a.map_u(|c| au.derive(c))?, w.map_u(|c| au.derive(c))?);
            let v = (a.map_v(|r| av.derive(r))?, w.map_v(|r| av.derive(r))?);
            (a, w, u, v)
        }
    };
    let second = if second {
        let uu = if p >= 2 {
            Some((
                cell_grid(&au_w.map_u(|c| au.derive_at(2, c))?, (&au, 2), (&av, 0))?,
                cell_grid(&wu_w.map_u(|c| au.derive_at(2, c))?, (&au, 2), (&av, 0))?,
            ))
        } else {
            None
        };
        let vv = if q >= 2 {
            Some((
                cell_grid(&av_w.map_v(|r| av.derive_at(2, r))?, (&au, 0), (&av, 2))?,
                cell_grid(&wv_w.map_v(|r| av.derive_at(2, r))?, (&au, 0), (&av, 2))?,
            ))
        } else {
            None
        };
        let uv = (
            cell_grid(&au_w.map_v(|r| av.derive_at(1, r))?, (&au, 1), (&av, 1))?,
            cell_grid(&wu_w.map_v(|r| av.derive_at(1, r))?, (&au, 1), (&av, 1))?,
        );
        Some(SecondGrids { uu, uv, vv })
    } else {
        None
    };
    Some(Homogeneous {
        second,
        a: cell_grid(&a, (&au, 0), (&av, 0))?,
        w: cell_grid(&w, (&au, 0), (&av, 0))?,
        au: cell_grid(&au_w, (&au, 1), (&av, 0))?,
        wu: cell_grid(&wu_w, (&au, 1), (&av, 0))?,
        av: cell_grid(&av_w, (&au, 0), (&av, 1))?,
        wv: cell_grid(&wv_w, (&au, 0), (&av, 1))?,
    })
}

/// A cell's pieces per direction, and its pieces row-major in `u`
/// then `v`.
type CellPieces<T> = ((usize, usize), PiecePairs<T>);

/// A cell's pieces, each with its second order when asked for.
type PiecePairs<T> = Vec<(CellQuotients<T>, Option<CellSecondQuotients<T>>)>;

/// **The surface statement's pieces** over every nonempty cell of the
/// rational tensor patch over `knots_u` × `knots_v` with row-major
/// `points` and `weights` (`[iu·nv + iv]`): [`cell_quotients`] per cell
/// in ascending `(su, sv)`, each cell's window read in the frame of its
/// first active point. `None` as [`cell_quotients`] or on an index past an
/// array; empty when no cell is nonempty.
pub fn stretch_pieces<T: Real>(
    (knots_u, knots_v): (&Knots<T>, &Knots<T>),
    points: &[[T; 3]],
    weights: &[T],
    splits: (usize, usize),
    area: bool,
) -> Option<Vec<CellQuotients<T>>> {
    let (ku, kv) = (knots_u.structure(), knots_v.structure());
    let nv = kv.control_count();
    let mut out = Vec::new();
    for su in (ku.first_span()..=ku.last_span()).filter_map(|i| ku.span(i)) {
        for sv in (kv.first_span()..=kv.last_span()).filter_map(|i| kv.span(i)) {
            let mut block = Vec::with_capacity((su.degree() + 1) * (sv.degree() + 1));
            for iu in su.window() {
                for iv in sv.window() {
                    let k = iu.checked_mul(nv)?.checked_add(iv)?;
                    block.push((*points.get(k)?, *weights.get(k)?));
                }
            }
            let (points, weights): (Vec<[T; 3]>, Vec<T>) = block.into_iter().unzip();
            out.extend(cell_quotients(
                (
                    CellAxis::Span(su, knots_u.values()),
                    CellAxis::Span(sv, knots_v.values()),
                ),
                CellBlock::Points {
                    origin: *points.first()?,
                    points: &points,
                    weights: &weights,
                },
                splits,
                area,
            )?);
        }
    }
    Some(out)
}

#[cfg(test)]
#[allow(clippy::cast_precision_loss, clippy::expect_used)]
mod tests {
    use super::span_derivative_scale;
    use crate::Bounds;
    use crate::dual::Dual;
    use crate::interval::Interval;
    use crate::real::Real;

    /// Knot values at every edge the quotient has: equal (a zero
    /// difference), reversed, signed zeros, subnormal and huge
    /// differences, exact and inexact quotients.
    const KNOTS: [f64; 12] = [
        -1.0,
        -0.0,
        0.0,
        5e-324,
        1e-300,
        0.1,
        1.0 / 3.0,
        0.5,
        1.0,
        3.0,
        1e300,
        f64::MAX,
    ];
    const DEGREES: [usize; 5] = [1, 2, 3, 5, 7];

    /// Per-step de Casteljau at ½, each step a sum and a halving: the
    /// reference [`super::Grid::halved`]'s unscaled sums are read against.
    fn halved_by_steps(
        g: &super::Grid<Interval, 3>,
        along_u: bool,
    ) -> (super::Grid<Interval, 3>, super::Grid<Interval, 3>) {
        let half = |x: super::Vector<Interval, 3>| x * Interval::from_f64(0.5);
        let row = |r: &[super::Vector<Interval, 3>]| {
            let n = r.len();
            let mut tri = r.to_vec();
            let (mut lo, mut hi) = (vec![tri[0]], vec![tri[n - 1]]);
            for step in 1..n {
                for i in 0..n - step {
                    tri[i] = half(tri[i] + tri[i + 1]);
                }
                lo.push(tri[0]);
                hi.push(tri[n - 1 - step]);
            }
            hi.reverse();
            (lo, hi)
        };
        if along_u {
            let (lo, hi): (Vec<_>, Vec<_>) =
                g.columns().expect("columns").iter().map(|c| row(c)).unzip();
            (
                super::Grid::from_columns(&lo).expect("a grid"),
                super::Grid::from_columns(&hi).expect("a grid"),
            )
        } else {
            let (lo, hi): (Vec<_>, Vec<_>) = g.c.chunks_exact(g.nv).map(row).unzip();
            (
                super::Grid::from_rows(&lo).expect("a grid"),
                super::Grid::from_rows(&hi).expect("a grid"),
            )
        }
    }

    /// **Halving sums unscaled and scales once, reading every step's
    /// value to the bit**: at the interval scalar, in the normal range,
    /// each half of a random grid along either axis is the per-step de
    /// Casteljau's own.
    #[test]
    fn halving_reads_each_steps_sums_to_the_bit() {
        let mut rng = test_utils::fuzz::start("derivative::halving_steps");
        for case in 0..test_utils::fuzz::scaled(40) {
            let deg = (rng.below(9), rng.below(9));
            let g = grid::<3>(&mut rng, deg);
            for along_u in [true, false] {
                let (lo, hi) = g.halved(along_u).expect("halves");
                let (want_lo, want_hi) = halved_by_steps(&g, along_u);
                assert_eq!(
                    (grid_bits(&lo), grid_bits(&hi)),
                    (grid_bits(&want_lo), grid_bits(&want_hi)),
                    "case {case}: degree {deg:?}, along_u {along_u} — {}",
                    test_utils::fuzz::replay()
                );
            }
        }
    }

    /// **A cell is halved into the pieces its schedule asks for at any
    /// degree.** A degree-5 cell's second order sits at degree 15, past
    /// the exact-weight tables' reach at 16 pieces (degree 12), where it
    /// read 8 per axis; halving reads 16, and every piece's weight
    /// power stays positive.
    #[test]
    fn a_cell_past_the_exact_tables_reads_every_piece_it_asks_for() {
        let (p, q) = (5, 5);
        let n = (p + 1) * (q + 1);
        let a: Vec<[f64; 3]> = (0..n)
            .map(|k| {
                let (i, j) = ((k / (q + 1)) as f64, (k % (q + 1)) as f64);
                [i, j, 0.1 * i * j - 0.05 * i * i]
            })
            .collect();
        let w: Vec<f64> = (0..n).map(|k| 1.0 + 0.25 * ((k * 7) % 5) as f64).collect();
        let a: Vec<[f64; 3]> = a.iter().zip(&w).map(|(x, w)| x.map(|c| c * w)).collect();
        let kv = super::super::KnotVector::bezier(p, (0.0, 1.0)).expect("a Bézier vector");
        let held = kv.knots().to_vec();
        let span = kv.span(kv.first_span()).expect("its one span");
        let axes = (
            super::CellAxis::Span(span, &held),
            super::CellAxis::Span(span, &held),
        );
        let (counts, pieces) = super::cell_second_quotients(
            axes,
            super::CellBlock::Homogeneous { a: &a, w: &w },
            (16, 16),
        )
        .expect("a degree-15 second order is under the ceiling");
        assert_eq!(counts, (16, 16), "the pieces per axis");
        assert_eq!(pieces.len(), 256, "the pieces read");
        assert!(
            pieces
                .iter()
                .all(|(_, s)| s.uu.denom().iter().all(|&b| b > 0.0)),
            "a piece's W³ coefficient is not positive"
        );
    }

    /// **A curve span is halved into the pieces its schedule asks for at
    /// any degree.** At degree 13 the exact-weight tables reached 32
    /// pieces only to degree 9 and 16 to degree 12, so the integral arm
    /// (counted for degree 12) read 16 and the rational arm and the
    /// acceleration (counted for 13) read 8. Halving reads 32 on each,
    /// and every piece's weight power stays positive.
    #[test]
    fn a_span_past_the_exact_tables_reads_every_piece_it_asks_for() {
        let p = 13;
        let knots = super::Knots::from(
            super::super::KnotVector::bezier(p, (0.0, 1.0)).expect("a Bézier vector"),
        );
        let points: Vec<[f64; 3]> = (0..=p)
            .map(|i| {
                let t = i as f64;
                [t, (0.7 * t).sin(), 0.1 * t * t]
            })
            .collect();
        let w: Vec<f64> = (0..=p).map(|k| 1.0 + 0.25 * ((k * 7) % 5) as f64).collect();
        let integral = super::speed_pieces(&knots, &points, None, 32).expect("the integral arm");
        assert_eq!(integral.len(), 32, "the integral arm's pieces");
        let rational =
            super::speed_pieces(&knots, &points, Some(&w), 32).expect("the rational arm");
        assert_eq!(rational.len(), 32, "the rational arm's pieces");
        let accel = super::accel_pieces(&knots, &points, &w, 32).expect("a degree-39 second order");
        assert_eq!(accel.len(), 32, "the second order's pieces");
        assert!(
            rational
                .iter()
                .chain(&accel)
                .all(|s| s.denom().iter().all(|&b| b > 0.0)),
            "a piece's weight power is not positive"
        );
    }

    /// An interval's two ends and its certification, as bits.
    fn bits(x: Interval) -> (u64, u64, bool) {
        (x.lo().to_bits(), x.hi().to_bits(), x.is_certified())
    }

    /// A coefficient with a width, so every product rounds outward.
    fn coefficient(rng: &mut test_utils::fuzz::Rng) -> Interval {
        let lo = rng.range(-2.0, 2.0);
        Interval::from_bounds(lo, lo + rng.range(0.0, 1e-9))
    }

    fn grid<const D: usize>(
        rng: &mut test_utils::fuzz::Rng,
        (m, n): (usize, usize),
    ) -> super::Grid<Interval, D> {
        let c = (0..(m + 1) * (n + 1))
            .map(|_| super::Vector(core::array::from_fn(|_| coefficient(rng))))
            .collect();
        super::Grid::new(n + 1, c).expect("a nonempty grid")
    }

    fn grid_bits<const D: usize>(g: &super::Grid<Interval, D>) -> Vec<(u64, u64, bool)> {
        g.c.iter().flat_map(|v| v.0.map(bits)).collect()
    }

    /// [`super::product`]'s fold before its binomials were tabled: the
    /// association it keeps.
    fn product_reference<const A: usize, const B: usize, const C: usize>(
        a: &super::Grid<Interval, A>,
        b: &super::Grid<Interval, B>,
        mul: impl Fn([Interval; A], [Interval; B]) -> [Interval; C],
    ) -> Option<super::Grid<Interval, C>> {
        use super::{Vector, binomial};
        let ((ma, na), (mb, nb)) = (a.degree(), b.degree());
        let (mu, mv) = (ma + mb, na + nb);
        let mut acc: Vec<Option<Vector<Interval, C>>> = vec![None; (mu + 1) * (mv + 1)];
        for i in 0..=ma {
            for j in 0..=na {
                let x = a.get(i, j)?.0;
                for k in 0..=mb {
                    for l in 0..=nb {
                        let term = Vector(mul(x, b.get(k, l)?.0))
                            .times(binomial(ma, i) * binomial(mb, k))
                            .times(binomial(na, j) * binomial(nb, l));
                        let slot = acc.get_mut((i + k) * (mv + 1) + j + l)?;
                        *slot = Some(slot.map_or(term, |s| s + term));
                    }
                }
            }
        }
        let c = acc
            .into_iter()
            .enumerate()
            .map(|(n, s)| {
                Some(
                    s?.over(binomial(mu, n / (mv + 1)))
                        .over(binomial(mv, n % (mv + 1))),
                )
            })
            .collect::<Option<Vec<_>>>()?;
        super::Grid::new(mv + 1, c)
    }

    /// **The Bernstein product folds as it always did, bit for bit**:
    /// the same terms, multiplied by the same binomials in the same
    /// association, summed in the same order, at the interval scalar,
    /// where any change of association or order moves an outward
    /// rounding.
    #[test]
    fn the_product_folds_as_its_reference() {
        let mut rng = test_utils::fuzz::start("derivative::product_reference");
        for case in 0..test_utils::fuzz::scaled(40) {
            let da = (rng.below(5), rng.below(5));
            let db = (rng.below(5), rng.below(5));
            let (a, b) = (grid::<3>(&mut rng, da), grid::<1>(&mut rng, db));
            let got = super::product(&a, &b, super::by_weight).expect("in the ceiling");
            let want = product_reference(&a, &b, super::by_weight).expect("in the ceiling");
            assert_eq!(
                grid_bits(&got),
                grid_bits(&want),
                "case {case}: {da:?} × {db:?} — {}",
                test_utils::fuzz::replay()
            );
        }
    }

    /// The spelling every routed site wrote before it called the helper.
    fn literal<T: Real>(p: usize, lo: T, hi: T) -> T {
        T::from_f64(p as f64) / (hi - lo)
    }

    /// **The helper is the sites' own spelling, bit for bit**, at the
    /// three scalars they run at: `p` entered first and the knot
    /// difference divided into it, never `p·(1/Δu)` or any other
    /// association. The goldens downstream do not see the difference,
    /// so this row is what keeps the association from drifting.
    #[test]
    fn the_scale_is_the_sites_spelling_at_every_scalar() {
        let mut asked = 0;
        for p in DEGREES {
            for lo in KNOTS {
                for hi in KNOTS {
                    let (a, b) = (span_derivative_scale(p, lo, hi), literal(p, lo, hi));
                    assert_eq!(a.to_bits(), b.to_bits(), "f64: p = {p}, [{lo}, {hi}]");
                    let (li, hi_i) = (Interval::from_f64(lo), Interval::from_f64(hi));
                    let (x, y) = (span_derivative_scale(p, li, hi_i), literal(p, li, hi_i));
                    let ends = |v: Interval| (v.lo().to_bits(), v.hi().to_bits(), v.is_certified());
                    assert_eq!(ends(x), ends(y), "Interval: p = {p}, [{lo}, {hi}]");
                    let (ld, hd) = (Dual::new(lo, 1.0), Dual::new(hi, 0.5));
                    let (x, y) = (span_derivative_scale(p, ld, hd), literal(p, ld, hd));
                    assert_eq!(
                        (x.value.to_bits(), x.deriv.to_bits()),
                        (y.value.to_bits(), y.deriv.to_bits()),
                        "Dual: p = {p}, [{lo}, {hi}]"
                    );
                    asked += 1;
                }
            }
        }
        assert_eq!(asked, DEGREES.len() * KNOTS.len() * KNOTS.len());
    }
}
