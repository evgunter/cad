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
//! ([`super::pieces::span_row`]), and halved at ½ by de Casteljau,
//! along each axis, as often as its schedule asks: each step a sum and
//! an exact halving, outward at `Interval`, so the halves enclose the
//! exact quotient's at any degree and the count has no exactness
//! ceiling. No piece reads a refined
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
//! # The measure quotients
//!
//! A quadrature of a rational patch's area `∫∫ |S_u×S_v|` and flux
//! `∫∫ S·(S_u×S_v)` reads three more quotients of the same shape
//! ([`cell_measure_quotients`]):
//!
//! ```text
//! ∂_d(M̃/W³)  = (M̃_d·W − 3·M̃·W_d) / W⁴
//! S·(S_u×S_v) = N/W³,   N = A·(A_u×A_v) + c·M̃
//! ∂_dd(N/W³) = (N_dd·W² − 6·N_d·W_d·W − 3·N·W_dd·W + 12·N·W_d²) / W⁵
//! ```
//!
//! — the flux because `S − c = A/W` in the frame `c` of the block's
//! channels, so `S·(S_u×S_v) = (A·M̃)/W⁴ + c·M̃/W³` and
//! `A·M̃ = W·A·(A_u×A_v)`, the two weight terms of `M̃` crossing `A` with
//! itself. Every term of the slope's numerator has bidegree
//! `(4p − 2, 4q − 1)` along `u` against `W⁴`'s `(4p, 4q)`, and of the
//! flux's `(5p − 3, 5q − 1)` against `W⁵`'s `(5p, 5q)` (`N` is
//! `(3p − 1, 3q − 1)`); `v` mirrors them. Each is raised to its weight
//! power's degree and read as the first order's convex combination,
//! clause for clause; the weight powers are products of the positive
//! weight row.
//!
//! `M̃_d`, `N_d` and `N_dd` are the derivatives of the grids formed over
//! the whole span, `n·(c_{k+1} − c_k)/width` along the axis, so a reader
//! that cuts the block restricts formed grids and differences none: the
//! derivative is taken before any piece's own `degree/width` scale
//! exists. `W_d` and `W_dd` are the weight's derivative rows, as the
//! second order reads them.
//!
//! # Degree ceiling
//!
//! The Bernstein product of rows of degrees `m` and `n` has coefficients
//! `c_k = Σ_{i+j=k} C(m,i)·C(n,j)·a_i·b_j / C(m+n, k)`, and elevation is
//! the product with a row of ones. Every pair product `C(m,i)·C(n,j)` is
//! at most `C(m+n, k)` (Vandermonde), so every factor the statement
//! forms is an exact `f64` integer while the result degree is at most
//! [`QUOTIENT_MAX_DEGREE`] ([`super::pieces::binomial`]); above it the
//! builders refuse (`None`). The measure quotients' `W⁵` meets it first,
//! at `5·max(p, q)`. At `Interval` each factor is a point and
//! every product and quotient rounds outward; at `f64` the same
//! expression rounds to nearest, and the reading can sit past the
//! bound by more than an ulp, growing with the weight spread (PR 4954
//! measured up to `5.3e-12` relative at a spread of `10³` on the
//! exact-weight cut the halving replaced; nothing re-measures it).
//! `Interval` is the certified lane.
//!
//! **The practical ceiling can sit lower on a wide weight spread.** The
//! weight power's coefficients are products of interval weights, and a
//! spread wide enough at a high enough degree lets their enclosures
//! reach zero: the ratios then do not certify and the reading is a
//! refusal, not a bound. The second order (`3p`) meets it first. Where
//! it falls is not pinned: PR 4954 placed it at degree 17 and a spread
//! near `10⁶`, but PR 5081's review probe there (degrees 17 and 18,
//! weights from 1 to `10⁶`) kept every `W³` coefficient certified
//! positive.
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
/// common degree whose binomials are all exact `f64`s.
pub const QUOTIENT_MAX_DEGREE: usize = super::pieces::BINOMIAL_EXACT_MAX;

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
/// by this module's builders ([`speed_pieces`], [`accel_pieces`],
/// [`cell_quotients`], [`cell_second_quotients`],
/// [`polynomial_area_pieces`], [`stretch_pieces`],
/// [`cell_measure_quotients`]), which hold both preconditions.
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

    /// The vector times the exact power of two `k`, skipped at one,
    /// through [`Real::scale_pow2`]: at the interval scalar an exponent
    /// shift, not a general product against a point.
    fn scaled(self, k: f64) -> Self {
        if k == 1.0 {
            self
        } else {
            Self(self.0.map(|x| x.scale_pow2(k)))
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
    ///
    /// One of restriction's three homes, kept as this module's halving:
    /// it is generic over `T: Real` on `Vector<T, D>` grids, where
    /// [`super::blossom::restrict_segment`], the door for a row read as
    /// it comes and the tightest of the three, reads certification
    /// scalars only.
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

    /// The grid's derivative along `u` (`along_u`) or `v`, the grid
    /// stated over the parameter interval `ends` along it: a degree-`n`
    /// line's `(c_{k+1} − c_k)·`[`span_derivative_scale`]`(n, ends)`.
    /// `None` along a degree-0 axis.
    fn derived(&self, along_u: bool, ends: (T, T)) -> Option<Self> {
        let n = if along_u {
            self.degree().0
        } else {
            self.degree().1
        };
        (n > 0).then_some(())?;
        let scale = span_derivative_scale(n, ends.0, ends.1);
        let diff = |line: &[Vector<T, D>]| -> Option<Vec<Vector<T, D>>> {
            Some(line.windows(2).map(|x| (x[1] - x[0]) * scale).collect())
        };
        if along_u {
            self.map_u(diff)
        } else {
            self.map_v(diff)
        }
    }

    /// Every coefficient mapped by `f`, at one shape.
    fn map<const E: usize>(&self, f: impl Fn([T; D]) -> [T; E]) -> Grid<T, E> {
        Grid {
            nv: self.nv,
            c: self.c.iter().map(|v| Vector(f(v.0))).collect(),
        }
    }
}

/// The largest power of two at most `splits`, never zero: how many
/// pieces halving cuts an axis into.
fn halving_count(splits: usize) -> usize {
    if splits == 0 { 1 } else { 1 << splits.ilog2() }
}

/// [`halving_count`] along each axis: the piece schedule `splits` read
/// as halvings, the one reading [`CellFirstQuotients::axis_pieces`] and
/// [`CellFirstQuotients::cut_pieces`] share.
fn halving_counts(splits: (usize, usize)) -> (usize, usize) {
    (halving_count(splits.0), halving_count(splits.1))
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
///
/// **`compose`'s `bern_mul_row` states the same product with the other
/// association**: a memoized outward quotient weight per term, on
/// univariate certification rows. Each association is the tighter one
/// for its own callers, so neither is folded into the other. Here,
/// exact integer numerators and a division after each coefficient's
/// sum keep the area, cross and raised terms tightest: a quotient per
/// term widens them by up to 1470× (`area_raised`), 208×
/// (`doubled_cross`) and 10× (`cell_pieces`), and on the `f64` lane it
/// would round every term. Taking this one in `compose` widens its
/// tensor powers by up to 6×. Both enclose the exact rational product
/// (`tests::both_bernstein_products_enclose_the_exact_rational_product`);
/// the measurement is in
/// `work/nurbs/binomial-bernstein-product-and-blossom-have-two-homes-each.md`.
fn product<T: Real, const A: usize, const B: usize, const C: usize>(
    a: &Grid<T, A>,
    b: &Grid<T, B>,
    mul: impl Fn([T; A], [T; B]) -> [T; C],
) -> Option<Grid<T, C>> {
    #[cfg(test)]
    tests::PRODUCTS.with(|n| n.set(n.get() + 1));
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

    /// The parameter interval the axis's Bernstein rows are stated
    /// over.
    fn ends(&self) -> (T, T) {
        match *self {
            Self::Span { ends, .. } => ends,
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
/// degree-1 `u` (identically zero there), `vv` likewise, and `uv`
/// absent unless [`Second::All`] asked for it.
struct SecondGrids<T> {
    uu: Option<(Grid<T, 3>, Grid<T, 1>)>,
    uv: Option<(Grid<T, 3>, Grid<T, 1>)>,
    vv: Option<(Grid<T, 3>, Grid<T, 1>)>,
}

/// Which second derivatives [`cell_whole`] forms.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Second {
    /// None.
    No,
    /// `uu` and `vv` alone (the measure quotients').
    Pure,
    /// `uu`, `uv` and `vv` (the second-order quotients').
    All,
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
        let uv = g.uv.as_ref()?;
        let (m, w3) = second_numerator(a, w, Some((&uv.0, &uv.1)), cross)?;
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
        Raised::of(
            &self.area_numerator(&product(&self.au, &self.av, cross)?)?,
            w3,
        )
    }

    /// `M̃ = W·(A_u×A_v) − W_v·(A_u×A) − W_u·(A×A_v)`, unraised, from
    /// the `A_u×A_v` grid `base`.
    fn area_numerator(&self, base: &Grid<T, 3>) -> Option<Grid<T, 3>> {
        let scaled = |w: &Grid<T, 1>, x: &Grid<T, 3>| product(w, x, weighted);
        scaled(&self.w, base)?
            .minus(&scaled(&self.wv, &product(&self.au, &self.a, cross)?)?)?
            .minus(&scaled(&self.wu, &product(&self.a, &self.av, cross)?)?)
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

    /// The quotient on `counts` equal pieces per axis (powers of two),
    /// as [`shared_pieces`] cuts it.
    fn pieces(&self, counts: (usize, usize)) -> Option<Vec<QuotientPiece<T, D>>> {
        let [pieces] = shared_pieces([&self.numer], &self.denom, counts)?;
        Some(pieces)
    }
}

/// `grid` on `counts` equal pieces per axis (powers of two), by halving
/// at ½ along `u` and then along `v`, row-major in `u` then `v`.
fn grid_pieces<T: Real, const D: usize>(
    grid: &Grid<T, D>,
    counts: (usize, usize),
) -> Option<Vec<Grid<T, D>>> {
    // A one-channel grid here is a weight power: counted where it is
    // halved, so a reader that halves one twice reads two.
    #[cfg(test)]
    if D == 1 {
        tests::WEIGHT_HALVINGS.with(|n| n.set(n.get() + 1));
    }
    let mut out = Vec::with_capacity(counts.0 * counts.1);
    for g in halvings(grid.clone(), counts.0, |g| g.halved(true))? {
        out.extend(halvings(g, counts.1, |g| g.halved(false))?);
    }
    Some(out)
}

/// The quotients `numers[i] / denom` on `counts` equal pieces per axis,
/// one list per numerator, each row-major in `u` then `v`. The weight
/// power is halved into its pieces once and each piece paired with
/// every numerator's: halving is deterministic, so a quotient that
/// halved its own copy of the same weight power would read the same
/// bits, and a weight power several quotients share costs one halving.
fn shared_pieces<T: Real, const D: usize, const N: usize>(
    numers: [&Grid<T, D>; N],
    denom: &Grid<T, 1>,
    counts: (usize, usize),
) -> Option<[Vec<QuotientPiece<T, D>>; N]> {
    let denoms: Vec<Vec<T>> = grid_pieces(denom, counts)?
        .iter()
        .map(|d| d.c.iter().map(|w| w.0[0]).collect())
        .collect();
    // Each numerator's list is filled or the whole reading refuses: no
    // slot is left at a default.
    let mut lists: [Vec<QuotientPiece<T, D>>; N] = core::array::from_fn(|_| Vec::new());
    for (list, numer) in lists.iter_mut().zip(numers) {
        *list = grid_pieces(numer, counts)?
            .iter()
            .zip(&denoms)
            .map(|(n, d)| QuotientPiece::new(n.arrays(), d.clone()))
            .collect::<Option<_>>()?;
    }
    Some(lists)
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
    for span in (kv.first_span()..=kv.last_span()).filter_map(|i| knots.span(i).map(|s| s.span())) {
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
    for span in (kv.first_span()..=kv.last_span()).filter_map(|i| knots.span(i).map(|s| s.span())) {
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
/// `splits.1` along `v`, row-major in `u` then `v`. `area` asks for
/// `S_u × S_v` as well. `None` on a degree below one, a block of the
/// wrong size, or a common degree above [`QUOTIENT_MAX_DEGREE`].
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
/// the piece count per direction (as [`cell_quotients`]'). `None` as
/// [`cell_quotients`], with the common degree `3·max(p, q)` against
/// [`QUOTIENT_MAX_DEGREE`].
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
    Raised { numer, denom }.pieces(halving_counts(splits))
}

#[allow(clippy::type_complexity)]
fn cell_pieces<T: Real>(
    axes: (CellAxis<'_, T>, CellAxis<'_, T>),
    block: CellBlock<'_, T>,
    splits: (usize, usize),
    area: bool,
    second: bool,
) -> Option<CellPieces<T>> {
    let whole = cell_whole(axes, block, if second { Second::All } else { Second::No })?;
    // Every quotient is formed once over the cell and halved into its
    // pieces, each weight power halved once for every quotient over it
    // ([`shared_pieces`]): `S_u` and `S_v` share `W²`, and the area and
    // the second partials share `W³`. With the second order the area is
    // formed whatever `area` asks: `CellSecondQuotients` readers take it
    // beside them.
    let counts = halving_counts(splits);
    let first = whole.first_raised(area || whole.second.is_some())?;
    let [u, v] = shared_pieces([&first.u.numer, &first.v.numer], &first.u.denom, counts)?;
    let (area, second) = match (&first.area, whole.second_raised()) {
        (Some(a), Some(s)) => {
            // Every `W³` is `W²·W` of the same `W` (`area_raised`,
            // `second_numerator`), so the area's serves all four.
            let [area, uu, uv, vv] = shared_pieces(
                [&a.numer, &s.uu.numer, &s.uv.numer, &s.vv.numer],
                &a.denom,
                counts,
            )?;
            let second = uu
                .into_iter()
                .zip(uv)
                .zip(vv)
                .map(|((uu, uv), vv)| Some(CellSecondQuotients { uu, uv, vv }))
                .collect();
            (area.into_iter().map(Some).collect(), second)
        }
        (Some(a), None) => {
            let [area] = shared_pieces([&a.numer], &a.denom, counts)?;
            (area.into_iter().map(Some).collect(), vec![None; u.len()])
        }
        (None, None) => (vec![None; u.len()], vec![None; u.len()]),
        (None, Some(_)) => unreachable!("the second order forms the area (`first_raised` above)"),
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
/// to `W²`'s bidegree. A reader takes one axis's pieces at a time:
/// [`Self::axis_pieces`] halves the whole block into a schedule's pieces,
/// as [`cell_quotients`] reads them, and at the interval scalar
/// [`CellFirstQuotients::cut_pieces`] first cuts the asked axes to a
/// sub-rectangle, restriction commuting with the ring operations that
/// formed them. [`Self::pieces`] and [`CellFirstQuotients::restrict`]
/// are the same readings of both axes together.
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
    let first = cell_whole(axes, block, Second::No)?.first_raised(false)?;
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
        // `S_u` and `S_v` share `W²` (`Homogeneous::first_raised`): one
        // halving of it serves both numerators.
        let [u, v] = shared_pieces(
            [&self.u.numer, &self.v.numer],
            &self.u.denom,
            halving_counts(splits),
        )?;
        Some(
            u.into_iter()
                .zip(v)
                .map(|(u, v)| CellQuotients { u, v, area: None })
                .collect(),
        )
    }

    /// The pieces of `S_u` (`along_u`) or `S_v` alone, as
    /// [`Self::pieces`] reads them, in the same order.
    #[must_use]
    pub fn axis_pieces(
        &self,
        along_u: bool,
        splits: (usize, usize),
    ) -> Option<Vec<QuotientPiece<T, 3>>> {
        if along_u { &self.u } else { &self.v }.pieces(halving_counts(splits))
    }
}

/// One axis's quotient pieces at [`Interval`](crate::interval::Interval),
/// as [`CellFirstQuotients::cut_pieces`] hands them back.
pub type IntervalPieces = Vec<QuotientPiece<crate::interval::Interval, 3>>;

impl CellFirstQuotients<crate::interval::Interval> {
    /// The quotients on the part of the block over `[a, b]` along each
    /// axis given a cut, the block spanning `[s0, s1]` along it: every
    /// line of every numerator and weight power restricted to the cut by
    /// two de Casteljau splits (`blossom::split_restrict`), each ratio an
    /// outward interval. An axis given `None` is kept whole. A cut out of
    /// its span refuses every coefficient it restricts.
    ///
    /// Restriction commutes with the ring operations, so the restricted
    /// grids are the cut block's own numerators and weight power at the
    /// same degrees, and the statement holds on the cut's pieces as on
    /// the cell's. Each restricted `W²` coefficient is a convex
    /// combination of the cell's, which are positive, so it is positive
    /// too (module docs, "The `b_k` are positive").
    #[must_use]
    pub fn restrict(&self, cuts: (Option<AxisCut>, Option<AxisCut>)) -> Self {
        let denom = self.u.denom.restricted(cuts);
        Self {
            u: restricted_over(&self.u, cuts, &denom),
            v: restricted_over(&self.v, cuts, &denom),
        }
    }

    /// The pieces of `S_u` (`axes.0`) and `S_v` (`axes.1`) on the part
    /// of the block a cut keeps ([`Self::restrict`]), each axis only
    /// where asked, `None` where not asked or refused: a reader of one
    /// axis restricts its numerator and `W²` alone. Bit for bit the
    /// pieces [`Self::restrict`] then [`Self::pieces`] read.
    ///
    /// One axis never refuses while the other reads: each numerator is
    /// raised to `W²`'s bidegree when formed ([`Raised::of`]), so both
    /// share `W²`'s grid shape, and restriction and halving refuse only
    /// on shape.
    #[must_use]
    pub fn cut_pieces(
        &self,
        cuts: (Option<AxisCut>, Option<AxisCut>),
        axes: (bool, bool),
        splits: (usize, usize),
    ) -> (Option<IntervalPieces>, Option<IntervalPieces>) {
        let counts = halving_counts(splits);
        match self.restrict_axes(cuts, axes) {
            // Both axes share the one restricted `W²`: halved once.
            (Some(u), Some(v)) => shared_pieces([&u.numer, &v.numer], &u.denom, counts)
                .map_or((None, None), |[u, v]| (Some(u), Some(v))),
            (u, v) => (
                u.and_then(|r| r.pieces(counts)),
                v.and_then(|r| r.pieces(counts)),
            ),
        }
    }

    /// The asked axes' quotients restricted; `S_u` and `S_v` share `W²`
    /// (`Homogeneous::first_raised`), so it is restricted once.
    fn restrict_axes(
        &self,
        cuts: (Option<AxisCut>, Option<AxisCut>),
        (want_u, want_v): (bool, bool),
    ) -> (
        Option<Raised<crate::interval::Interval, 3>>,
        Option<Raised<crate::interval::Interval, 3>>,
    ) {
        if !(want_u || want_v) {
            return (None, None);
        }
        let denom = self.u.denom.restricted(cuts);
        (
            want_u.then(|| restricted_over(&self.u, cuts, &denom)),
            want_v.then(|| restricted_over(&self.v, cuts, &denom)),
        )
    }
}

/// `q`'s numerator restricted to `cuts`, over `denom`: `W²` restricted
/// to the same cuts, which `S_u` and `S_v` share
/// (`Homogeneous::first_raised`), so a caller restricts it once.
fn restricted_over(
    q: &Raised<crate::interval::Interval, 3>,
    cuts: (Option<AxisCut>, Option<AxisCut>),
    denom: &Grid<crate::interval::Interval, 1>,
) -> Raised<crate::interval::Interval, 3> {
    Raised {
        numer: q.numer.restricted(cuts),
        denom: denom.clone(),
    }
}

/// One axis's cut: the span `(s0, s1)` the block covers and the part
/// `(a, b)` of it to keep.
pub type AxisCut = ((f64, f64), (f64, f64));

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
                for (k, x) in super::blossom::split_restrict(&row, span, ends)
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
    second: Second,
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
    let second = if second == Second::No {
        None
    } else {
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
        let uv = if second == Second::All {
            Some((
                cell_grid(&au_w.map_v(|r| av.derive_at(1, r))?, (&au, 1), (&av, 1))?,
                cell_grid(&wu_w.map_v(|r| av.derive_at(1, r))?, (&au, 1), (&av, 1))?,
            ))
        } else {
            None
        };
        Some(SecondGrids { uu, uv, vv })
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
    for su in (ku.first_span()..=ku.last_span()).filter_map(|i| knots_u.span(i).map(|s| s.span())) {
        for sv in
            (kv.first_span()..=kv.last_span()).filter_map(|i| knots_v.span(i).map(|s| s.span()))
        {
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

/// **A cell block's measure quotients, formed once** (module docs, "The
/// measure quotients"): what a quadrature of the area `∫∫ |S_u×S_v|` and
/// of the flux `∫∫ S·(S_u×S_v)` reads off a block, each a polynomial
/// over a power of `W` raised to that power's bidegree:
///
/// - the area element `S_u × S_v = M̃/W³`;
/// - its slopes `∂_d(M̃/W³) = (M̃_d·W − 3·M̃·W_d)/W⁴` along `u` and `v`;
/// - the flux integrand's second partials
///   `∂_dd(N/W³) = (N_dd·W² − 6·N_d·W_d·W − 3·N·W_dd·W + 12·N·W_d²)/W⁵`,
///   `N = A·(A_u×A_v) + c·M̃` for the block's frame `c`.
///
/// Read at [`Interval`](crate::interval::Interval) on the part of the
/// block a cut keeps ([`Self::area_on`], [`Self::flux_second_on`]).
#[derive(Clone, Debug)]
pub struct CellMeasureQuotients<T> {
    area: Raised<T, 3>,
    slope_u: Raised<T, 3>,
    slope_v: Raised<T, 3>,
    flux_uu: Raised<T, 1>,
    flux_vv: Raised<T, 1>,
}

/// The area element and its two slopes on one piece
/// ([`CellMeasureQuotients::area_on`]).
#[derive(Clone, Debug)]
pub struct AreaSlopes<T> {
    /// `S_u × S_v = M̃/W³`.
    pub area: QuotientPiece<T, 3>,
    /// `∂_u(S_u × S_v)`, over `W⁴`.
    pub u: QuotientPiece<T, 3>,
    /// `∂_v(S_u × S_v)`, over `W⁴`.
    pub v: QuotientPiece<T, 3>,
}

/// The measure quotients of the [`CellBlock::Points`] `block` along
/// `axes` (module docs, "The measure quotients"), the flux read in the
/// coordinates its points are stated in: the block's `origin` is the
/// frame `c` of `N = A·(A_u×A_v) + c·M̃`. `None` on a
/// [`CellBlock::Homogeneous`] block, whose channels name no frame, on a
/// degree below one, a block of the wrong size, or a common degree
/// `5·max(p, q)` above [`QUOTIENT_MAX_DEGREE`].
pub fn cell_measure_quotients<T: Real>(
    axes: (CellAxis<'_, T>, CellAxis<'_, T>),
    block: CellBlock<'_, T>,
) -> Option<CellMeasureQuotients<T>> {
    let CellBlock::Points { origin: frame, .. } = block else {
        return None;
    };
    let (au, av) = (Axis::of(axes.0)?, Axis::of(axes.1)?);
    let (ends_u, ends_v) = (au.ends(), av.ends());
    let h = cell_whole(axes, block, Second::Pure)?;
    let second = h.second.as_ref()?;
    let base = product(&h.au, &h.av, cross)?;
    let m = h.area_numerator(&base)?;
    let n = product(&h.a, &base, |a, b| [dot(a, b)])?.plus(&m.map(|x| [dot(frame, x)]))?;
    let w = &h.w;
    let w2 = product(w, w, weight_times)?;
    let w3 = product(&w2, w, weight_times)?;
    let w4 = product(&w2, &w2, weight_times)?;
    let w5 = product(&w4, w, weight_times)?;
    let slope = |along_u: bool, wd: &Grid<T, 1>| -> Option<Raised<T, 3>> {
        let md = m.derived(along_u, if along_u { ends_u } else { ends_v })?;
        let numer = product(&md, w, by_weight)?.minus(&product(&m, wd, by_weight)?.times(3.0)?)?;
        Raised::of(&numer, w4.clone())
    };
    let flux = |along_u: bool,
                wd: &Grid<T, 1>,
                wdd: Option<&(Grid<T, 3>, Grid<T, 1>)>|
     -> Option<Raised<T, 1>> {
        let ends = if along_u { ends_u } else { ends_v };
        let nd = n.derived(along_u, ends)?;
        let ndd = nd.derived(along_u, ends)?;
        let mut numer = product(&ndd, &w2, by_weight)?
            .minus(&x_wy(&nd, wd, w)?.times(6.0)?)?
            .plus(&a_yz(&n, wd, wd)?.times(12.0)?)?;
        if let Some((_, wdd)) = wdd {
            numer = numer.minus(&a_yz(&n, wdd, w)?.times(3.0)?)?;
        }
        Raised::of(&numer, w5.clone())
    };
    Some(CellMeasureQuotients {
        area: Raised::of(&m, w3)?,
        slope_u: slope(true, &h.wu)?,
        slope_v: slope(false, &h.wv)?,
        flux_uu: flux(true, &h.wu, second.uu.as_ref())?,
        flux_vv: flux(false, &h.wv, second.vv.as_ref())?,
    })
}

impl CellMeasureQuotients<crate::interval::Interval> {
    /// The area element and its slopes on the part of the block over
    /// `[a, b]` along each axis given a cut, one piece each: every
    /// numerator and weight power restricted as
    /// [`CellFirstQuotients::restrict`] restricts them. The slopes share
    /// one restricted `W⁴`. `None` on a refused restriction.
    #[must_use]
    pub fn area_on(
        &self,
        cuts: (Option<AxisCut>, Option<AxisCut>),
    ) -> Option<AreaSlopes<crate::interval::Interval>> {
        let w4 = self.slope_u.denom.restricted(cuts);
        Some(AreaSlopes {
            area: piece_on(&self.area, cuts, &self.area.denom.restricted(cuts))?,
            u: piece_on(&self.slope_u, cuts, &w4)?,
            v: piece_on(&self.slope_v, cuts, &w4)?,
        })
    }

    /// `∂_uu` and `∂_vv` of the flux integrand on the part of the block
    /// a cut keeps, one piece each, over one restricted `W⁵`.
    #[must_use]
    pub fn flux_second_on(
        &self,
        cuts: (Option<AxisCut>, Option<AxisCut>),
    ) -> Option<(
        QuotientPiece<crate::interval::Interval, 1>,
        QuotientPiece<crate::interval::Interval, 1>,
    )> {
        let w5 = self.flux_uu.denom.restricted(cuts);
        Some((
            piece_on(&self.flux_uu, cuts, &w5)?,
            piece_on(&self.flux_vv, cuts, &w5)?,
        ))
    }
}

/// `q`'s numerator restricted to `cuts`, as one piece over `denom`, the
/// weight power restricted to the same cuts.
fn piece_on<const D: usize>(
    q: &Raised<crate::interval::Interval, D>,
    cuts: (Option<AxisCut>, Option<AxisCut>),
    denom: &Grid<crate::interval::Interval, 1>,
) -> Option<QuotientPiece<crate::interval::Interval, D>> {
    QuotientPiece::new(
        q.numer.restricted(cuts).arrays(),
        denom.c.iter().map(|w| w.0[0]).collect(),
    )
}

#[cfg(test)]
#[allow(clippy::cast_precision_loss, clippy::expect_used)]
mod tests {
    use super::span_derivative_scale;
    use crate::Bounds;
    use crate::dual::Dual;
    use crate::interval::Interval;
    use crate::real::Real;
    use interval_transcendentals::Decoration;

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

    /// **The halving's scale is the exact power-of-two door.** A line
    /// whose outputs land between the smallest normal and the interval
    /// backend's 2Prod floor (`2^-960`) halves EXACTLY through
    /// [`Real::scale_pow2`] — each output the sum times `2⁻ʳ`, unpadded
    /// — where a general product against the point `2⁻ʳ` would pad every
    /// one a step outward. So a scale that went back to the general
    /// product reds here; nothing else does, since above the floor the
    /// two agree to the bit.
    #[test]
    fn the_halving_scales_through_the_exact_door() {
        let tiny = f64::MIN_POSITIVE * 1024.0;
        let row: Vec<super::Vector<Interval, 1>> = [3.0, 5.0, 7.0]
            .iter()
            .map(|&c| super::Vector([Interval::from_f64(c * tiny)]))
            .collect();
        let g = super::Grid::new(1, row).expect("a curve row");
        let (lo, hi) = g.halved(true).expect("halves");
        // The lower half's Bernstein row of [3, 5, 7]·tiny is
        // [3, 4, 5]·tiny, the upper's [5, 6, 7]·tiny: sums scaled by
        // 1, 1/2 and 1/4, every one exact.
        for (half, want) in [(&lo, [3.0, 4.0, 5.0]), (&hi, [5.0, 6.0, 7.0])] {
            for (got, w) in half.c.iter().zip(want) {
                let x = got.0[0];
                assert_eq!((x.lo(), x.hi()), (w * tiny, w * tiny), "{w}·tiny");
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

    std::thread_local! {
        /// How many Bernstein products this thread has formed.
        pub(super) static PRODUCTS: core::cell::Cell<usize> = const { core::cell::Cell::new(0) };
    }

    /// **A cut forms no product.** A cell's first-order quotients are
    /// formed once ([`super::cell_first_quotients`], which forms its
    /// products), and every cut of them after that is a restriction and
    /// halving of those products alone: red if a cut re-forms the
    /// quotients from the cut block.
    #[test]
    fn a_cut_of_formed_quotients_forms_no_product() {
        let (p, q) = (3, 2);
        let n = (p + 1) * (q + 1);
        let a: Vec<[Interval; 3]> = (0..n)
            .map(|k| {
                let (i, j) = ((k / (q + 1)) as f64, (k % (q + 1)) as f64);
                [i, j, 0.1 * i * j].map(Interval::from_f64)
            })
            .collect();
        let w: Vec<Interval> = (0..n)
            .map(|k| Interval::from_f64(1.0 + 0.25 * ((k * 7) % 5) as f64))
            .collect();
        let knots_u = [0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0].map(Interval::from_f64);
        let knots_v = [0.0, 0.0, 0.0, 1.0, 1.0, 1.0].map(Interval::from_f64);
        let ku = super::super::KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0], p)
            .expect("a Bézier vector");
        let kv = super::super::KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], q)
            .expect("a Bézier vector");
        let axes = (
            super::CellAxis::Span(ku.span(p).expect("the span"), &knots_u),
            super::CellAxis::Span(kv.span(q).expect("the span"), &knots_v),
        );
        let formed = PRODUCTS.with(core::cell::Cell::get);
        let whole =
            super::cell_first_quotients(axes, super::CellBlock::Homogeneous { a: &a, w: &w })
                .expect("a block under the ceiling");
        assert!(
            PRODUCTS.with(core::cell::Cell::get) > formed,
            "forming the quotients forms products"
        );
        let before = PRODUCTS.with(core::cell::Cell::get);
        let pieces = whole
            .restrict((
                Some(((0.0, 1.0), (0.25, 0.5))),
                Some(((0.0, 1.0), (0.5, 0.75))),
            ))
            .pieces((2, 2))
            .expect("the cut's pieces");
        assert_eq!(pieces.len(), 4, "a cut's 2×2 pieces");
        assert_eq!(
            PRODUCTS.with(core::cell::Cell::get),
            before,
            "a cut formed a product"
        );
    }

    /// **A flat block's cut keeps its exact derivative.** The bilinear
    /// plane `(x, y, z) = (u, v, 0.3·u − 0.7·v)` with unit weights over
    /// a cell has constant `N_u` and `N_v` and `W² = 1`: its whole-cell
    /// quotient pieces read them exactly (zero-width), and so does every
    /// cut at non-dyadic ends, so a zero-width whole box stays zero-width
    /// under the cut. Red with `split_restrict`'s equal-neighbour arm
    /// removed (the thin step widens constant rows by the ratio's width).
    #[test]
    fn a_flat_blocks_cut_keeps_its_exact_derivative() {
        let a: Vec<[Interval; 3]> = [(0.0, 0.0), (0.0, 1.0), (1.0, 0.0), (1.0, 1.0)]
            .iter()
            .map(|&(u, v)| [u, v, 0.3 * u - 0.7 * v].map(Interval::from_f64))
            .collect();
        let w = vec![Interval::from_f64(1.0); 4];
        let held = [0.0, 0.0, 1.0, 1.0].map(Interval::from_f64);
        let k = super::super::KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1)
            .expect("a Bézier vector");
        let axes = (
            super::CellAxis::Span(k.span(1).expect("the span"), &held),
            super::CellAxis::Span(k.span(1).expect("the span"), &held),
        );
        let whole =
            super::cell_first_quotients(axes, super::CellBlock::Homogeneous { a: &a, w: &w })
                .expect("a block under the ceiling");
        let zero_width = |pieces: &[super::CellQuotients<Interval>]| {
            pieces.iter().all(|q| {
                [&q.u, &q.v].iter().all(|r| {
                    r.numer()
                        .iter()
                        .flatten()
                        .chain(r.denom())
                        .all(|x| x.lo() == x.hi())
                })
            })
        };
        assert!(
            zero_width(&whole.pieces((2, 2)).expect("the whole pieces")),
            "the whole cell"
        );
        let cut = whole
            .restrict((
                Some(((0.0, 1.0), (0.1, 0.7))),
                Some(((0.0, 1.0), (0.3, 0.9))),
            ))
            .pieces((2, 2))
            .expect("the cut's pieces");
        assert!(zero_width(&cut), "the cut widened a constant quotient");
    }

    /// **A one-axis cut reads the two-axis cut's pieces to the bit.**
    /// [`super::CellFirstQuotients::cut_pieces`] restricts only the axes
    /// its reader asks for (a reader of `S_u` restricts `N_u` and `W²`
    /// alone); each asked axis's pieces equal, bit for bit and decoration
    /// for decoration, the pieces [`super::CellFirstQuotients::restrict`]
    /// then [`super::CellFirstQuotients::pieces`] read, and an unasked
    /// axis is `None`. Drawn rational blocks of degree 1–3 per axis with
    /// widened coefficients, on non-dyadic Bézier spans, cut along `u`,
    /// `v` or both at drawn ends, at 1×1 and 2×2 pieces and the
    /// asymmetric 1×2, 2×1 and 4×2, for every choice of axes. Red if a
    /// one-axis cut restricts `W²` differently from the two-axis cut,
    /// hands back the other axis's pieces, or reads the piece counts
    /// swapped.
    #[test]
    fn a_one_axis_cut_reads_the_two_axis_cuts_pieces_to_the_bit() {
        type Pieces = Vec<super::QuotientPiece<Interval, 3>>;
        let piece_bits = |pieces: &Pieces| -> Vec<(u64, u64, Decoration)> {
            pieces
                .iter()
                .flat_map(|q| q.numer().iter().flatten().chain(q.denom()).copied())
                .map(|x| (x.lo().to_bits(), x.hi().to_bits(), x.decoration()))
                .collect()
        };
        let mut rng = test_utils::fuzz::start("one-axis-cut");
        for _ in 0..test_utils::fuzz::scaled(60) {
            let (p, q) = (1 + rng.below(3), 1 + rng.below(3));
            let n = (p + 1) * (q + 1);
            let a: Vec<[Interval; 3]> = (0..n)
                .map(|_| core::array::from_fn(|_| coefficient(&mut rng)))
                .collect();
            let w: Vec<Interval> = (0..n)
                .map(|_| {
                    let lo = rng.range(0.5, 3.0);
                    Interval::from_bounds(lo, lo + rng.range(0.0, 1e-9))
                })
                .collect();
            let span = |rng: &mut test_utils::fuzz::Rng| {
                let s0 = rng.range(-1.0, 1.0);
                (s0, s0 + rng.range(0.3, 2.0))
            };
            let (du, dv) = (span(&mut rng), span(&mut rng));
            let vector = |d: usize, (s0, s1): (f64, f64)| {
                let k: Vec<f64> = core::iter::repeat_n(s0, d + 1)
                    .chain(core::iter::repeat_n(s1, d + 1))
                    .collect();
                super::super::KnotVector::clamped(k, d).expect("a Bézier vector")
            };
            let (ku, kv) = (vector(p, du), vector(q, dv));
            let held_u: Vec<Interval> = ku.knots().iter().map(|&k| Interval::from_f64(k)).collect();
            let held_v: Vec<Interval> = kv.knots().iter().map(|&k| Interval::from_f64(k)).collect();
            let axes = (
                super::CellAxis::Span(ku.span(p).expect("the span"), &held_u),
                super::CellAxis::Span(kv.span(q).expect("the span"), &held_v),
            );
            let whole =
                super::cell_first_quotients(axes, super::CellBlock::Homogeneous { a: &a, w: &w })
                    .expect("a block under the ceiling");
            let cut = |rng: &mut test_utils::fuzz::Rng, (s0, s1): (f64, f64)| {
                let (x, y) = (rng.range(s0, s1), rng.range(s0, s1));
                ((s0, s1), (x.min(y), x.max(y)))
            };
            let which = rng.below(3);
            let cuts = (
                (which != 1).then(|| cut(&mut rng, du)),
                (which != 0).then(|| cut(&mut rng, dv)),
            );
            for splits in [(1, 1), (2, 2), (1, 2), (2, 1), (4, 2)] {
                let both = whole
                    .restrict(cuts)
                    .pieces(splits)
                    .expect("the two-axis cut's pieces");
                let (ref_u, ref_v): (Pieces, Pieces) = both.into_iter().map(|c| (c.u, c.v)).unzip();
                for asked in [(true, true), (true, false), (false, true), (false, false)] {
                    let (u, v) = whole.cut_pieces(cuts, asked, splits);
                    for (got, want, on, axis) in
                        [(u, &ref_u, asked.0, "S_u"), (v, &ref_v, asked.1, "S_v")]
                    {
                        match got {
                            Some(got) => assert!(
                                on && piece_bits(&got) == piece_bits(want),
                                "{axis} asked {asked:?} at {splits:?} on cuts {cuts:?}: \
                                 not the two-axis cut's pieces"
                            ),
                            None => assert!(!on, "{axis} asked {asked:?} came back refused"),
                        }
                    }
                }
            }
        }
    }

    /// **A cut quotient holds the true derivative over its own region.**
    /// An independent oracle for [`super::CellFirstQuotients::cut_pieces`]:
    /// drawn rational blocks of degree 1–3 per axis, weights spread over
    /// `[0.2, 5]`, on non-dyadic Bézier spans, cut along both axes at drawn
    /// ends; over a 9×9 grid of the cut's own rectangle, `S_d` evaluated in
    /// `f64` by the quotient rule on the Bernstein form (no code under test)
    /// lies in the hull of every piece's coefficient ratios `a_k/b_k`.
    /// Red when the numerators and `W²` are cut over different regions
    /// (`W²` restricted along `u` only: the review of #5391's M3), at
    /// either restriction site or both. The bit-for-bit guard row above
    /// sees one site changed alone, since each side of it runs the other,
    /// but not both changed alike.
    #[test]
    fn a_cut_quotient_holds_the_sampled_derivative_over_its_region() {
        // `B_{n,i}(t)` and its derivative in `t`.
        fn bern(n: usize, i: usize, t: f64) -> f64 {
            if i > n {
                return 0.0;
            }
            let binom = super::super::pieces::binomial(n, i);
            binom * t.powi(i as i32) * (1.0 - t).powi((n - i) as i32)
        }
        fn dbern(n: usize, i: usize, t: f64) -> f64 {
            let left = if i == 0 { 0.0 } else { bern(n - 1, i - 1, t) };
            n as f64 * (left - bern(n - 1, i, t))
        }
        let mut rng = test_utils::fuzz::start("cut-oracle");
        for _ in 0..test_utils::fuzz::scaled(40) {
            let (p, q) = (1 + rng.below(3), 1 + rng.below(3));
            let n = (p + 1) * (q + 1);
            let pts: Vec<[f64; 3]> = (0..n)
                .map(|_| core::array::from_fn(|_| rng.range(-2.0, 2.0)))
                .collect();
            let wts: Vec<f64> = (0..n).map(|_| rng.range(0.2, 5.0)).collect();
            let span = |rng: &mut test_utils::fuzz::Rng| {
                let s0 = rng.range(-1.0, 1.0);
                (s0, s0 + rng.range(0.3, 2.0))
            };
            let (du, dv) = (span(&mut rng), span(&mut rng));
            let vector = |d: usize, (s0, s1): (f64, f64)| {
                let k: Vec<f64> = core::iter::repeat_n(s0, d + 1)
                    .chain(core::iter::repeat_n(s1, d + 1))
                    .collect();
                super::super::KnotVector::clamped(k, d).expect("a Bézier vector")
            };
            let (ku, kv) = (vector(p, du), vector(q, dv));
            let held_u: Vec<Interval> = ku.knots().iter().map(|&k| Interval::from_f64(k)).collect();
            let held_v: Vec<Interval> = kv.knots().iter().map(|&k| Interval::from_f64(k)).collect();
            let axes = (
                super::CellAxis::Span(ku.span(p).expect("the span"), &held_u),
                super::CellAxis::Span(kv.span(q).expect("the span"), &held_v),
            );
            let points: Vec<[Interval; 3]> =
                pts.iter().map(|x| x.map(Interval::from_f64)).collect();
            let weights: Vec<Interval> = wts.iter().copied().map(Interval::from_f64).collect();
            let block = super::CellBlock::Points {
                points: &points,
                weights: &weights,
                origin: [Interval::from_f64(0.0); 3],
            };
            let whole =
                super::cell_first_quotients(axes, block).expect("a block under the ceiling");
            let cut = |rng: &mut test_utils::fuzz::Rng, (s0, s1): (f64, f64)| {
                let (x, y) = (rng.range(s0, s1), rng.range(s0, s1));
                ((s0, s1), (x.min(y), x.max(y)))
            };
            let (cu, cv) = (cut(&mut rng, du), cut(&mut rng, dv));
            // `S_u` (`along_u`) or `S_v` at `(x, y)`, by the quotient rule.
            let truth = |along_u: bool, x: f64, y: f64| -> [f64; 3] {
                let (tu, tv) = ((x - du.0) / (du.1 - du.0), (y - dv.0) / (dv.1 - dv.0));
                let (mut nn, mut nd, mut w, mut wd) = ([0.0; 3], [0.0; 3], 0.0, 0.0);
                for i in 0..=p {
                    for j in 0..=q {
                        let k = i * (q + 1) + j;
                        let b = bern(p, i, tu) * bern(q, j, tv);
                        let db = if along_u {
                            dbern(p, i, tu) / (du.1 - du.0) * bern(q, j, tv)
                        } else {
                            bern(p, i, tu) * dbern(q, j, tv) / (dv.1 - dv.0)
                        };
                        w += wts[k] * b;
                        wd += wts[k] * db;
                        for c in 0..3 {
                            nn[c] += wts[k] * pts[k][c] * b;
                            nd[c] += wts[k] * pts[k][c] * db;
                        }
                    }
                }
                core::array::from_fn(|c| (nd[c] * w - nn[c] * wd) / (w * w))
            };
            for along_u in [true, false] {
                let (u, v) = whole.cut_pieces((Some(cu), Some(cv)), (along_u, !along_u), (2, 2));
                let pieces = if along_u { u } else { v }.expect("the cut's pieces");
                let hull: [(f64, f64); 3] = core::array::from_fn(|c| {
                    pieces
                        .iter()
                        .flat_map(|q| q.numer().iter().zip(q.denom()).map(move |(a, b)| a[c] / *b))
                        .fold((f64::INFINITY, f64::NEG_INFINITY), |(lo, hi), r| {
                            (lo.min(r.lo()), hi.max(r.hi()))
                        })
                });
                for i in 0..=8 {
                    for j in 0..=8 {
                        let x = cu.1.0 + (cu.1.1 - cu.1.0) * f64::from(i) / 8.0;
                        let y = cv.1.0 + (cv.1.1 - cv.1.0) * f64::from(j) / 8.0;
                        let s = truth(along_u, x, y);
                        for c in 0..3 {
                            let slack = 1e-9 * s[c].abs().max(1.0);
                            assert!(
                                hull[c].0 - slack <= s[c] && s[c] <= hull[c].1 + slack,
                                "S_{} component {c} at ({x}, {y}) = {} outside the cut's \
                                 hull [{}, {}] (cuts {cu:?} × {cv:?}, degrees {p}×{q})",
                                if along_u { "u" } else { "v" },
                                s[c],
                                hull[c].0,
                                hull[c].1
                            );
                        }
                    }
                }
            }
        }
    }

    /// **The measure quotients hold the sampled area element, its
    /// slopes and the flux integrand's second partials over a cut**
    /// (counterexample search): on a random rational Bézier block, read
    /// in a random frame and cut to a random sub-rectangle, every sampled
    /// `|S_u×S_v|` lies between the area piece's `inf_norm` and
    /// `sup_norm`, every `|∂_d(S_u×S_v)|` under its slope's `sup_norm`,
    /// and every `∂_dd(S·(S_u×S_v))` in its piece's ratio range. The truth
    /// is the patch's own Bernstein derivatives through the quotient and
    /// Leibniz rules, in absolute coordinates: no formed grid, no
    /// numerator formula of the statement's.
    #[test]
    fn the_measure_quotients_hold_the_sampled_area_and_flux_over_a_cut() {
        // `d^k/dt^k B_{n,i}(t)`.
        fn bern(n: usize, i: usize, k: usize, t: f64) -> f64 {
            if i > n {
                return 0.0;
            }
            if k == 0 {
                let binom = super::super::pieces::binomial(n, i);
                return binom * t.powi(i as i32) * (1.0 - t).powi((n - i) as i32);
            }
            if n == 0 {
                return 0.0;
            }
            let left = if i == 0 {
                0.0
            } else {
                bern(n - 1, i - 1, k - 1, t)
            };
            n as f64 * (left - bern(n - 1, i, k - 1, t))
        }
        fn cross3(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
            [
                a[1] * b[2] - a[2] * b[1],
                a[2] * b[0] - a[0] * b[2],
                a[0] * b[1] - a[1] * b[0],
            ]
        }
        fn dot3(a: [f64; 3], b: [f64; 3]) -> f64 {
            a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
        }
        fn norm3(a: [f64; 3]) -> f64 {
            dot3(a, a).sqrt()
        }
        let mut rng = test_utils::fuzz::start("measure-quotient-oracle");
        for _ in 0..test_utils::fuzz::scaled(30) {
            let (p, q) = (1 + rng.below(3), 1 + rng.below(3));
            let n = (p + 1) * (q + 1);
            let pts: Vec<[f64; 3]> = (0..n)
                .map(|_| core::array::from_fn(|_| rng.range(-2.0, 2.0)))
                .collect();
            let wts: Vec<f64> = (0..n).map(|_| rng.range(0.2, 5.0)).collect();
            let span = |rng: &mut test_utils::fuzz::Rng| {
                let s0 = rng.range(-1.0, 1.0);
                (s0, s0 + rng.range(0.3, 2.0))
            };
            let (du, dv) = (span(&mut rng), span(&mut rng));
            let vector = |d: usize, (s0, s1): (f64, f64)| {
                let k: Vec<f64> = core::iter::repeat_n(s0, d + 1)
                    .chain(core::iter::repeat_n(s1, d + 1))
                    .collect();
                super::super::KnotVector::clamped(k, d).expect("a Bézier vector")
            };
            let (ku, kv) = (vector(p, du), vector(q, dv));
            let held_u: Vec<Interval> = ku.knots().iter().map(|&k| Interval::from_f64(k)).collect();
            let held_v: Vec<Interval> = kv.knots().iter().map(|&k| Interval::from_f64(k)).collect();
            let axes = (
                super::CellAxis::Span(ku.span(p).expect("the span"), &held_u),
                super::CellAxis::Span(kv.span(q).expect("the span"), &held_v),
            );
            let points: Vec<[Interval; 3]> =
                pts.iter().map(|x| x.map(Interval::from_f64)).collect();
            let weights: Vec<Interval> = wts.iter().copied().map(Interval::from_f64).collect();
            // A frame away from every point: the flux reads `c·M̃` too.
            let frame: [f64; 3] = core::array::from_fn(|_| rng.range(-5.0, 5.0));
            let origin = frame.map(Interval::from_f64);
            let block = super::CellBlock::Points {
                points: &points,
                weights: &weights,
                origin,
            };
            let whole =
                super::cell_measure_quotients(axes, block).expect("a block under the ceiling");
            let cut = |rng: &mut test_utils::fuzz::Rng, (s0, s1): (f64, f64)| {
                let (x, y) = (rng.range(s0, s1), rng.range(s0, s1));
                ((s0, s1), (x.min(y), x.max(y)))
            };
            let (cu, cv) = (cut(&mut rng, du), cut(&mut rng, dv));
            let area = whole.area_on((Some(cu), Some(cv))).expect("the cut's area");
            let (fuu, fvv) = whole
                .flux_second_on((Some(cu), Some(cv)))
                .expect("the cut's flux");
            // `∂_u^i ∂_v^j` of `A = Σ B w P` and `W = Σ B w` at `(x, y)`.
            let at = |x: f64, y: f64| {
                let (tu, tv) = ((x - du.0) / (du.1 - du.0), (y - dv.0) / (dv.1 - dv.0));
                let (pts, wts) = (&pts, &wts);
                move |i: usize, j: usize| -> ([f64; 3], f64) {
                    let s = (du.1 - du.0).powi(i as i32) * (dv.1 - dv.0).powi(j as i32);
                    let (mut a, mut w) = ([0.0; 3], 0.0);
                    for ii in 0..=p {
                        for jj in 0..=q {
                            let k = ii * (q + 1) + jj;
                            let b = bern(p, ii, i, tu) * bern(q, jj, j, tv) / s;
                            w += wts[k] * b;
                            for c in 0..3 {
                                a[c] += wts[k] * pts[k][c] * b;
                            }
                        }
                    }
                    (a, w)
                }
            };
            let slack = |x: f64| 1e-8 * x.abs().max(1.0);
            let ratio_range = |r: (Interval, Interval)| (r.0.lo(), r.1.hi());
            for i in 0..=6 {
                for j in 0..=6 {
                    let x = cu.1.0 + (cu.1.1 - cu.1.0) * f64::from(i) / 6.0;
                    let y = cv.1.0 + (cv.1.1 - cv.1.0) * f64::from(j) / 6.0;
                    let d = at(x, y);
                    let (a, w) = d(0, 0);
                    let s: [f64; 3] = a.map(|c| c / w);
                    // `S_d = (A_d − S·W_d)/W`, and the second partials
                    // `S_ab = (A_ab − S_a·W_b − S_b·W_a − S·W_ab)/W`.
                    let first = |i: usize, j: usize| -> [f64; 3] {
                        let (ad, wd) = d(i, j);
                        core::array::from_fn(|c| (ad[c] - s[c] * wd) / w)
                    };
                    let (su, sv) = (first(1, 0), first(0, 1));
                    let second = |(ia, ja): (usize, usize), (ib, jb): (usize, usize)| {
                        let (aab, wab) = d(ia + ib, ja + jb);
                        let (sa, sb) = (first(ia, ja), first(ib, jb));
                        let (wa, wb) = (d(ia, ja).1, d(ib, jb).1);
                        core::array::from_fn::<f64, 3, _>(|c| {
                            (aab[c] - sa[c] * wb - sb[c] * wa - s[c] * wab) / w
                        })
                    };
                    let (suu, suv, svv) = (
                        second((1, 0), (1, 0)),
                        second((1, 0), (0, 1)),
                        second((0, 1), (0, 1)),
                    );
                    let n = cross3(su, sv);
                    let slope_u: [f64; 3] =
                        core::array::from_fn(|c| cross3(suu, sv)[c] + cross3(su, suv)[c]);
                    let slope_v: [f64; 3] =
                        core::array::from_fn(|c| cross3(suv, sv)[c] + cross3(su, svv)[c]);
                    let g = norm3(n);
                    assert!(
                        area.area.inf_norm().lo() - slack(g) <= g
                            && g <= area.area.sup_norm().hi() + slack(g),
                        "|S_u×S_v| = {g} at ({x}, {y}) outside [{:?}, {:?}] (degrees {p}×{q})",
                        area.area.inf_norm(),
                        area.area.sup_norm()
                    );
                    for (along, slope, piece) in [("u", slope_u, &area.u), ("v", slope_v, &area.v)]
                    {
                        let m = norm3(slope);
                        assert!(
                            m <= piece.sup_norm().hi() + slack(m),
                            "|∂_{along}(S_u×S_v)| = {m} at ({x}, {y}) over its sup_norm \
                             {:?} (degrees {p}×{q})",
                            piece.sup_norm()
                        );
                    }
                    // `f = N·W⁻³`, `N = A·(A_u×A_v)` absolute, and
                    // `f_dd = N_dd·g + 2·N_d·g_d + N·g_dd`, `g = W⁻³`; `N`'s
                    // partials by Leibniz over its three factors.
                    let flux_dd = |along_u: bool| -> f64 {
                        let step = |k: usize| if along_u { (k, 0) } else { (0, k) };
                        let dn = |k: usize| -> f64 {
                            let mut sum = 0.0;
                            for x in 0..=k {
                                for y in 0..=k - x {
                                    let z = k - x - y;
                                    let multi = [1.0, 1.0, 2.0][k]
                                        / ([1.0, 1.0, 2.0][x]
                                            * [1.0, 1.0, 2.0][y]
                                            * [1.0, 1.0, 2.0][z]);
                                    let (sx, sy, sz) = (step(x), step(y), step(z));
                                    let fa = d(sx.0, sx.1).0;
                                    let fb = d(sy.0 + 1, sy.1).0;
                                    let fc = d(sz.0, sz.1 + 1).0;
                                    sum += multi * dot3(fa, cross3(fb, fc));
                                }
                            }
                            sum
                        };
                        let (wd, wdd) = (d(step(1).0, step(1).1).1, d(step(2).0, step(2).1).1);
                        let g = w.powi(-3);
                        let gd = -3.0 * w.powi(-4) * wd;
                        let gdd = 12.0 * w.powi(-5) * wd * wd - 3.0 * w.powi(-4) * wdd;
                        dn(2) * g + 2.0 * dn(1) * gd + dn(0) * gdd
                    };
                    for (along_u, piece) in [(true, &fuu), (false, &fvv)] {
                        let truth = flux_dd(along_u);
                        let [r] = piece.coordinate_range();
                        let (lo, hi) = ratio_range(r);
                        assert!(
                            lo - slack(truth) <= truth && truth <= hi + slack(truth),
                            "f_{} = {truth} at ({x}, {y}) outside [{lo}, {hi}] (degrees {p}×{q}, \
                             frame {frame:?})",
                            if along_u { "uu" } else { "vv" }
                        );
                    }
                }
            }
        }
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

    std::thread_local! {
        /// How many weight powers this thread has halved into pieces.
        pub(super) static WEIGHT_HALVINGS: core::cell::Cell<usize> =
            const { core::cell::Cell::new(0) };
    }

    /// A rational bicubic-by-quadratic cell block, its weights uneven,
    /// at the interval scalar.
    struct RationalCell {
        a: Vec<[Interval; 3]>,
        w: Vec<Interval>,
        knots_u: [Interval; 8],
        knots_v: [Interval; 6],
        ku: super::super::KnotVector,
        kv: super::super::KnotVector,
    }

    fn rational_cell() -> RationalCell {
        let (p, q) = (3, 2);
        let n = (p + 1) * (q + 1);
        let a = (0..n)
            .map(|k| {
                let (i, j) = ((k / (q + 1)) as f64, (k % (q + 1)) as f64);
                [i, j, 0.1 * i * j + 0.3 * (k % 2) as f64].map(Interval::from_f64)
            })
            .collect();
        let w = (0..n)
            .map(|k| Interval::from_f64(1.0 + 0.25 * ((k * 7) % 5) as f64))
            .collect();
        let ku = super::super::KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0], p)
            .expect("a Bézier vector");
        let kv = super::super::KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], q)
            .expect("a Bézier vector");
        RationalCell {
            a,
            w,
            knots_u: [0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0].map(Interval::from_f64),
            knots_v: [0.0, 0.0, 0.0, 1.0, 1.0, 1.0].map(Interval::from_f64),
            ku,
            kv,
        }
    }

    /// **A cell halves each weight power once.** `S_u` and `S_v` share
    /// `W²`, and the area and the three second partials share `W³`, so a
    /// cell's pieces halve one weight power at the first order without
    /// the area and two otherwise, at any schedule: red if a quotient
    /// halves its own copy of a shared weight power again (6 with the
    /// second order).
    #[test]
    fn a_cell_halves_each_weight_power_once() {
        let RationalCell {
            a,
            w,
            knots_u,
            knots_v,
            ku,
            kv,
        } = rational_cell();
        let axes = || {
            (
                super::CellAxis::Span(ku.span(3).expect("the span"), &knots_u[..]),
                super::CellAxis::Span(kv.span(2).expect("the span"), &knots_v[..]),
            )
        };
        let block = || super::CellBlock::Homogeneous { a: &a, w: &w };
        let weight_halvings = |f: &dyn Fn() -> usize| {
            let before = WEIGHT_HALVINGS.with(core::cell::Cell::get);
            let pieces = f();
            assert!(pieces > 1, "the schedule cuts the cell");
            WEIGHT_HALVINGS.with(core::cell::Cell::get) - before
        };
        for splits in [(2, 1), (4, 2), (8, 8)] {
            let first = || {
                super::cell_quotients(axes(), block(), splits, false)
                    .expect("pieces")
                    .len()
            };
            let area = || {
                super::cell_quotients(axes(), block(), splits, true)
                    .expect("pieces")
                    .len()
            };
            let second = || {
                super::cell_second_quotients(axes(), block(), splits)
                    .expect("pieces")
                    .1
                    .len()
            };
            assert_eq!(weight_halvings(&first), 1, "W² once, splits {splits:?}");
            assert_eq!(
                weight_halvings(&area),
                2,
                "W² and W³ once each, splits {splits:?}"
            );
            assert_eq!(
                weight_halvings(&second),
                2,
                "W² and W³ once each, splits {splits:?}"
            );
        }
    }

    /// **A cut reader halves its weight power once.**
    /// [`super::CellFirstQuotients::pieces`] and the both-axes
    /// [`super::CellFirstQuotients::cut_pieces`] pair one halving of the
    /// shared `W²` with `S_u`'s and `S_v`'s numerators; a one-axis cut
    /// halves it once for its one numerator, and a cut that asks for no
    /// axis halves nothing. Counted where a weight power is halved, so red
    /// if a reader, or `shared_pieces` itself, halves the shared `W²` once
    /// per numerator.
    #[test]
    fn a_cut_reader_halves_its_weight_power_once() {
        let RationalCell {
            a,
            w,
            knots_u,
            knots_v,
            ku,
            kv,
        } = rational_cell();
        let axes = (
            super::CellAxis::Span(ku.span(3).expect("the span"), &knots_u[..]),
            super::CellAxis::Span(kv.span(2).expect("the span"), &knots_v[..]),
        );
        let whole =
            super::cell_first_quotients(axes, super::CellBlock::Homogeneous { a: &a, w: &w })
                .expect("a block under the ceiling");
        let halvings = |f: &dyn Fn()| {
            let before = WEIGHT_HALVINGS.with(core::cell::Cell::get);
            f();
            WEIGHT_HALVINGS.with(core::cell::Cell::get) - before
        };
        let cuts = (
            Some(((0.0, 1.0), (0.25, 0.5))),
            Some(((0.0, 1.0), (0.5, 0.75))),
        );
        for splits in [(1, 1), (2, 2), (4, 2)] {
            let whole_pieces = || {
                whole.pieces(splits).expect("the whole pieces");
            };
            assert_eq!(halvings(&whole_pieces), 1, "the whole cell, {splits:?}");
            for (asked, want) in [
                ((true, true), 1),
                ((true, false), 1),
                ((false, true), 1),
                ((false, false), 0),
            ] {
                let cut = || {
                    let _ = whole.cut_pieces(cuts, asked, splits);
                };
                assert_eq!(halvings(&cut), want, "a cut asking {asked:?}, {splits:?}");
            }
        }
    }

    /// **A shared halving reads each quotient's own halving to the
    /// bit.** Every piece [`super::cell_second_quotients`] serves (its
    /// `S_u`, `S_v`, area and three second partials, numerator and
    /// weight power) is the bits of that quotient halved as one,
    /// numerator and its own weight power together, the area's and the
    /// second partials' `W³` formed apart; and so is every piece
    /// [`super::cell_quotients`] serves with the area and without it.
    #[test]
    fn a_shared_weight_halving_is_each_quotients_own_to_the_bit() {
        let RationalCell {
            a,
            w,
            knots_u,
            knots_v,
            ku,
            kv,
        } = rational_cell();
        let axes = || {
            (
                super::CellAxis::Span(ku.span(3).expect("the span"), &knots_u[..]),
                super::CellAxis::Span(kv.span(2).expect("the span"), &knots_v[..]),
            )
        };
        let block = || super::CellBlock::Homogeneous { a: &a, w: &w };
        let ends = |x: &Interval| (x.lo().to_bits(), x.hi().to_bits(), x.is_certified());
        let same = |got: &super::QuotientPiece<Interval, 3>,
                    want: &super::QuotientPiece<Interval, 3>,
                    what: &str| {
            let flat = |p: &super::QuotientPiece<Interval, 3>| {
                (
                    p.numer().iter().flatten().map(ends).collect::<Vec<_>>(),
                    p.denom().iter().map(ends).collect::<Vec<_>>(),
                )
            };
            assert_eq!(flat(got), flat(want), "{what}");
        };
        for splits in [(1, 1), (2, 4), (8, 2)] {
            let counts = super::halving_counts(splits);
            let whole = super::cell_whole(axes(), block(), super::Second::All)
                .expect("a block under the ceiling");
            let first = whole.first_raised(true).expect("the first order");
            let area = first.area.as_ref().expect("the area");
            let second = whole.second_raised().expect("the second order");
            // The quotient halved as one: numerator and weight power
            // halved together, pair by pair, along `u` and then `v`.
            let own = |r: &super::Raised<Interval, 3>| -> Vec<super::QuotientPiece<Interval, 3>> {
                let halve = |along_u: bool| {
                    move |(n, d): &(super::Grid<Interval, 3>, super::Grid<Interval, 1>)| {
                        let (n0, n1) = n.halved(along_u)?;
                        let (d0, d1) = d.halved(along_u)?;
                        Some(((n0, d0), (n1, d1)))
                    }
                };
                super::halvings((r.numer.clone(), r.denom.clone()), counts.0, halve(true))
                    .expect("halves")
                    .into_iter()
                    .flat_map(|nd| super::halvings(nd, counts.1, halve(false)).expect("halves"))
                    .map(|(n, d)| {
                        super::QuotientPiece::new(n.arrays(), d.c.iter().map(|w| w.0[0]).collect())
                            .expect("a piece")
                    })
                    .collect()
            };
            let (u, v, ar) = (own(&first.u), own(&first.v), own(area));
            let (uu, uv, vv) = (own(&second.uu), own(&second.uv), own(&second.vv));
            let (got_counts, got) =
                super::cell_second_quotients(axes(), block(), splits).expect("pieces");
            assert_eq!(got_counts, counts);
            assert_eq!(got.len(), u.len());
            for (k, (f, s)) in got.iter().enumerate() {
                same(&f.u, &u[k], &format!("S_u piece {k}, splits {splits:?}"));
                same(&f.v, &v[k], &format!("S_v piece {k}, splits {splits:?}"));
                same(
                    f.area.as_ref().expect("the area"),
                    &ar[k],
                    &format!("area piece {k}"),
                );
                same(&s.uu, &uu[k], &format!("S_uu piece {k}, splits {splits:?}"));
                same(&s.uv, &uv[k], &format!("S_uv piece {k}, splits {splits:?}"));
                same(&s.vv, &vv[k], &format!("S_vv piece {k}, splits {splits:?}"));
            }
            for with_area in [true, false] {
                let got =
                    super::cell_quotients(axes(), block(), splits, with_area).expect("pieces");
                assert_eq!(got.len(), u.len());
                for (k, f) in got.iter().enumerate() {
                    let at = format!("piece {k}, splits {splits:?}, area {with_area}");
                    same(&f.u, &u[k], &format!("S_u {at}"));
                    same(&f.v, &v[k], &format!("S_v {at}"));
                    match &f.area {
                        Some(area) => same(area, &ar[k], &format!("area {at}")),
                        None => assert!(!with_area, "the area is absent at {at}"),
                    }
                    assert!(f.area.is_some() == with_area, "the area asked at {at}");
                }
            }
        }
    }

    /// **Two Bernstein products, one statement.** On integer rows, the
    /// exact product's coefficient `k` is the rational
    /// `Σ C(da,i)·C(db,k−i)·a_i·b_{k−i} / C(da+db, k)`. Both homes of
    /// the Bernstein product (this module's [`super::product`] on
    /// one-row grids, `compose`'s `bern_mul_row` on certification rows)
    /// enclose it, each at its own association (`product`'s docs).
    #[test]
    fn both_bernstein_products_enclose_the_exact_rational_product() {
        use super::super::compose::exact_q::Q;
        let q = |x: i128| Q::from_f64(x as f64);
        let choose = |n: usize, k: usize| Q::from_f64(super::binomial(n, k));
        let iv = |x: i128| Interval::from_f64(x as f64);
        let grid = |r: &[i128]| {
            super::Grid::new(1, r.iter().map(|&x| super::Vector([iv(x)])).collect())
                .expect("a one-row grid")
        };
        for (da, db) in [(1, 1), (3, 2), (5, 7), (12, 9), (20, 20)] {
            let a: Vec<i128> = (0..=da).map(|i| ((i * 7 + 3) % 11) as i128 - 5).collect();
            let b: Vec<i128> = (0..=db).map(|j| ((j * 5 + 1) % 9) as i128 - 4).collect();
            let rows = (
                a.iter().map(|&x| iv(x)).collect::<Vec<_>>(),
                b.iter().map(|&x| iv(x)).collect::<Vec<_>>(),
            );
            let compose = super::super::compose::bern_mul_row(&rows.0, &rows.1);
            let ours = super::product(
                &grid(&a),
                &grid(&b),
                |x: [Interval; 1], y: [Interval; 1]| [x[0] * y[0]],
            )
            .expect("a product under the ceiling");
            assert_eq!(compose.len(), da + db + 1);
            assert_eq!(ours.c.len(), da + db + 1);
            for k in 0..=da + db {
                let exact = (k.saturating_sub(db)..=k.min(da))
                    .map(|i| {
                        choose(da, i)
                            .mul(&choose(db, k - i))
                            .mul(&q(a[i]))
                            .mul(&q(b[k - i]))
                    })
                    .fold(q(0), |acc, t| acc.add(&t))
                    .div(&choose(da + db, k));
                for (home, x) in [("compose", compose[k]), ("derivative", ours.c[k].0[0])] {
                    use core::cmp::Ordering::{Greater, Less};
                    assert!(
                        exact.cmp_f64(x.lo()) != Less && exact.cmp_f64(x.hi()) != Greater,
                        "{home} misses {:e} at ({da}, {db}) k = {k}: [{:e}, {:e}]",
                        exact.to_f64_report(),
                        x.lo(),
                        x.hi()
                    );
                }
            }
        }
    }
}
