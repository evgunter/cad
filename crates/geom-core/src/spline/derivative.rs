//! The span derivative's coefficient factor, spelled once: the
//! derivative of a degree-`p` B-spline has coefficients
//! `p·(c_{i+1} − c_i)/(u_{i+p+1} − u_{i+1})` (The NURBS Book Eq. 3.4).
//!
//! What a zero knot difference means is the caller's, so
//! [`span_derivative_scale`] takes the two knot values and checks
//! nothing.
//!
//! # The rational speed bound
//!
//! [`rational_speed_sup`] is the one statement of an upper bound
//! on a rational spline's derivative, the **pair-term bound**. On a
//! nonempty span of degree `p`, with `kᵢ = p/(u_{i+p+1} − u_{i+1})`,
//!
//! ```text
//! C′ = Σᵢ N_{i,p−1} · kᵢ · [ w_{i+1}(P_{i+1} − Pᵢ) − (w_{i+1} − wᵢ)(C − Pᵢ) ] / w
//! ```
//!
//! over the span's active pairs `i`. Each bracket is affine in `C`,
//! and `C` lies in the convex hull of the span's active points `P_j`
//! (positive weights), so its norm is largest at one of them; the
//! `N_{i,p−1}` sum to one and `w` is at least the least active weight.
//! So
//!
//! ```text
//! sup ‖C′‖ ≤ max_{i, j} kᵢ·‖w_{i+1}(P_{i+1} − Pᵢ) − (w_{i+1} − wᵢ)(P_j − Pᵢ)‖ / min_j w_j
//! ```
//!
//! with no centre to choose. At unit weights it is exactly the
//! polynomial control-difference bound `max kᵢ·‖P_{i+1} − Pᵢ‖`. A
//! domain's bound is the max over its nonempty spans, a window's over
//! the spans it meets.
//!
//! A tensor patch's `S_u` is, on a nonempty cell, the same sum over the
//! `u` pairs of each of the cell's active columns, with `S` in the hull
//! of the cell's active points: [`rational_stretch_sup`] is the
//! same bound per direction, per cell. Both read one
//! [`RationalPairTerm`].
//!
//! # The second order
//!
//! Write the first-order statement as `W·C′ = Σᵢ N_{i,p−1}·bᵢ(C)`, with
//! `bᵢ(S) = kᵢ·[w_{i+1}(P_{i+1} − Pᵢ) − (w_{i+1} − wᵢ)(S − Pᵢ)]` the pair
//! term and `W = Σ N_{i,p} wᵢ`. Its weight derivative is
//! `W′ = Σᵢ N_{i,p−1}·dᵢ` with `dᵢ = kᵢ·(w_{i+1} − wᵢ)`, the pair's
//! **weight rate**, and `d/dt bᵢ(C) = −dᵢ·C′`. Differentiating the
//! statement,
//!
//! ```text
//! W′·C′ + W·C″ = Σᵢ N′_{i,p−1}·bᵢ(C) − (Σᵢ N_{i,p−1}·dᵢ)·C′
//! W·C″        = Σⱼ N_{j,p−2}·k²ⱼ·(b_{j+1} − bⱼ)(C) − 2·W′·C′
//! ```
//!
//! with `k²ⱼ = (p − 1)/(u_{j+p+1} − u_{j+2})`, the first derivative's own
//! pair scale (Eq. 3.4 applied to the coefficients `bᵢ`), over the
//! span's second-difference window `j ∈ [s − p, s − 2]`. Each
//! `(b_{j+1} − bⱼ)` is affine in `C`, so with `C` in the hull of the
//! active points the first sum is a convex combination of its values at
//! them; `|W′| ≤ max |dᵢ|`; `‖C′‖` is at most the first-order bound
//! `S₁`; and `W ≥ min w`. So
//!
//! ```text
//! sup ‖C″‖ ≤ ( max_{j, l} k²ⱼ·‖(b_{j+1} − bⱼ)(P_l)‖ + 2·max_i |dᵢ|·S₁ ) / min w
//! ```
//!
//! `C′` enters only through `S₁`: the term `−2·W′·C′` is not affine in
//! a hull point, so it is bounded as a product. At unit weights every
//! `dᵢ` is zero and `bᵢ` is the constant `kᵢ(P_{i+1} − Pᵢ)`, leaving the
//! polynomial second-difference bound. At degree 1 the window is
//! empty (`N_{i,0}` is constant on the span) and `W·C″ = −2·W′·C′`
//! exactly; on a single rational segment the bound is then the true
//! sup. [`RationalSpanTerms::accel_sup`] is this bound.
//!
//! A tensor patch's cell reads the same identity per direction over the
//! `u` pairs of each active column (`v` symmetric):
//!
//! ```text
//! W·S_uu = Σ N_{j,p−2} N_b·k²ⱼ·(b_{j+1,b} − b_{j,b})(S) − 2·W_u·S_u
//! W·S_uv = Σ N_{i,p−1} N_{c,q−1}·k_c·(b_{i,c+1} − b_{i,c})(S) − W_v·S_u − W_u·S_v
//! ```
//!
//! the second from differentiating `W·S_u` in `v`, where
//! `Σ_b N′_{b,q}·y_b = Σ_c N_{c,q−1}·k_c·(y_{c+1} − y_c)`. Every first
//! sum is a convex combination of its affine terms at the cell's active
//! points ([`RationalCellTerms::second_vertices`]), `W_u` and `W_v` are
//! convex combinations of the weight rates, and `S_u`, `S_v` are what
//! the first-order statement gives: `W·S_u` is a convex combination of
//! [`RationalCellTerms::first_vertices`]. A consumer that needs a
//! signed box takes each of these hulls and divides by the cell's
//! weight range; one that needs a norm takes the vertex norms over the
//! least weight. Either way it reads ONE first-order statement for
//! both its `S_u` box and its `S_u` sup.
//!
//! **No zero knot difference is reachable.** A span is walked only
//! when it is nonempty, and its active pairs are `i ∈ [s − p, s − 1]`
//! ([`Span::first_derived_window`](super::Span::first_derived_window),
//! total): each pair's knot difference `u_{i+p+1} − u_{i+1}` spans
//! `[u_s, u_{s+1}]`, so it is positive. Nothing here checks it.

use core::ops::{Mul, Sub};

use crate::real::Real;

use super::knot_values::Knots;
use super::knots::Span;
use super::locate::SpanLocate;

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

/// The Euclidean norm of a `D`-vector, `√Σ xₖ²`.
fn norm<T: Real, const D: usize>(v: [T; D]) -> T {
    v.iter()
        .map(|x| x.powi(2))
        .fold(T::zero(), |a, b| a + b)
        .sqrt()
}

/// **One pair term** of the rational derivative (module docs): the pair
/// `(P₀, w₀) → (P₁, w₁)` with scale `k`, as the affine map
/// `S ↦ k·[lever − dw·(S − p0)]` with `lever = w₁(P₁ − P₀)` and
/// `dw = w₁ − w₀`. The one statement of the term: the sups below
/// evaluate it at points, and a reader that carries it through a rigid
/// map (`geom-brep`'s SSI enclosure) reads it about a centre
/// ([`Self::about`]).
#[derive(Clone, Copy, Debug)]
pub struct RationalPairTerm<T, const D: usize> {
    scale: T,
    lever: [T; D],
    dw: T,
    p0: [T; D],
}

impl<T: Real, const D: usize> RationalPairTerm<T, D> {
    /// The pair `(p0, w0) → (p1, w1)` at scale `scale`.
    #[must_use]
    pub fn new(scale: T, (p0, w0): ([T; D], T), (p1, w1): ([T; D], T)) -> Self {
        Self {
            scale,
            lever: core::array::from_fn(|c| w1 * (p1[c] - p0[c])),
            dw: w1 - w0,
            p0,
        }
    }

    /// The term as an affine function of the offset `q = S − c`:
    /// `(k, g)` with term `= k − g·q`, `k = scale·[lever + dw·(p0 − c)]`
    /// and `g = scale·dw`. Every point enters as a difference with
    /// another, so a translation moves `k` by its rounding width alone.
    #[must_use]
    pub fn about(&self, c: [T; D]) -> ([T; D], T) {
        (
            core::array::from_fn(|i| (self.lever[i] + self.dw * (self.p0[i] - c[i])) * self.scale),
            self.dw * self.scale,
        )
    }

    /// The term at the point `s`: [`Self::about`] `s` itself,
    /// `scale·[lever − dw·(s − p0)]`.
    #[must_use]
    pub fn at(&self, s: [T; D]) -> [T; D] {
        self.about(s).0
    }

    /// The weight step `w₁ − w₀`: exactly zero where the pair's weights
    /// agree, so the term does not read `S`.
    #[must_use]
    pub fn weight_step(&self) -> T {
        self.dw
    }

    /// The pair's weight rate `scale·(w₁ − w₀)`: its coefficient of the
    /// weight's derivative `W′` (module docs), and the term's slope in
    /// `S`.
    #[must_use]
    pub fn weight_rate(&self) -> T {
        self.dw * self.scale
    }
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

/// `max_{v} ‖v‖`, or poison when there is none.
fn norm_max<T: Real, const D: usize>(vs: impl Iterator<Item = [T; D]>) -> T {
    vs.map(norm)
        .reduce(T::max)
        .unwrap_or_else(|| T::from_f64(f64::NAN))
}

/// `max_{x} |x|`, or poison when there is none.
fn abs_max<T: Real>(xs: impl Iterator<Item = T>) -> T {
    xs.map(T::abs)
        .reduce(T::max)
        .unwrap_or_else(|| T::from_f64(f64::NAN))
}

/// `k·(b(s) − a(s))`: one second-difference term of the module docs'
/// second-order statement, from two adjacent pair terms `a`, `b`.
fn second_difference<T: Real, const D: usize>(
    k: T,
    a: &RationalPairTerm<T, D>,
    b: &RationalPairTerm<T, D>,
    s: [T; D],
) -> [T; D] {
    let (x, y) = (a.at(s), b.at(s));
    core::array::from_fn(|c| (y[c] - x[c]) * k)
}

/// The scale of second-difference pair `j` of a degree-`p` spline over
/// the `held` knot values, `(p − 1)/(u_{j+p+1} − u_{j+2})`: the first
/// derivative's own pair scale ([`pair_scale`] at degree `p − 1` over
/// the vector less its first knot). `None` past the vector.
fn second_scale<T: Real>(held: &[T], p: usize, j: usize) -> Option<T> {
    pair_scale(held.get(1..)?, p.checked_sub(1)?, j)
}

/// The second-difference scales of `span` (empty at degree 1), `None`
/// past the vector.
fn second_scales<T: Real>(span: Span<'_>, held: &[T]) -> Option<Vec<T>> {
    match span.derived_window(2) {
        None => Some(Vec::new()),
        Some(w) => w.map(|j| second_scale(held, span.degree(), j)).collect(),
    }
}

/// **One nonempty curve span's pair terms** (module docs) and the two
/// statements read off them: `W·C′` lies in the convex hull of
/// [`Self::first_vertices`], `W·C″ + 2·W′·C′` in that of
/// [`Self::second_vertices`], `W′` between the extremes of
/// [`Self::weight_rates`], and `W` is at least [`Self::least_weight`].
#[derive(Clone, Debug)]
pub struct RationalSpanTerms<T, const D: usize> {
    terms: Vec<RationalPairTerm<T, D>>,
    second: Vec<T>,
    active: Vec<[T; D]>,
    least: T,
}

impl<T: Real, const D: usize> RationalSpanTerms<T, D> {
    /// The terms of the nonempty `span`, for the rational spline with
    /// control `points` and `weights` over the `held` knot values of
    /// the vector `span` indexes. `None` on an index past an array.
    #[must_use]
    pub fn new(span: Span<'_>, held: &[T], points: &[[T; D]], weights: &[T]) -> Option<Self> {
        let p = span.degree();
        let active = points.get(span.window())?.to_vec();
        let least = weights.get(span.window())?.iter().copied().reduce(T::min)?;
        let terms = span
            .first_derived_window()
            .map(|i| {
                Some(RationalPairTerm::new(
                    pair_scale(held, p, i)?,
                    (*points.get(i)?, *weights.get(i)?),
                    (*points.get(i + 1)?, *weights.get(i + 1)?),
                ))
            })
            .collect::<Option<Vec<_>>>()?;
        Some(Self {
            terms,
            second: second_scales(span, held)?,
            active,
            least,
        })
    }

    /// Every pair term at every active point: `W·C′` is a convex
    /// combination of these.
    pub fn first_vertices(&self) -> impl Iterator<Item = [T; D]> + '_ {
        self.terms
            .iter()
            .flat_map(move |t| self.active.iter().map(move |q| t.at(*q)))
    }

    /// Every second-difference term `k²ⱼ·(b_{j+1} − bⱼ)` at every
    /// active point: `W·C″ + 2·W′·C′` is a convex combination of these.
    /// Empty at degree 1, where `W·C″ = −2·W′·C′` on the span.
    pub fn second_vertices(&self) -> impl Iterator<Item = [T; D]> + '_ {
        self.second.iter().enumerate().flat_map(move |(j, k)| {
            let (a, b) = (&self.terms[j], &self.terms[j + 1]);
            self.active
                .iter()
                .map(move |q| second_difference(*k, a, b, *q))
        })
    }

    /// Each pair's weight rate `kᵢ·(w_{i+1} − wᵢ)`: `W′` is a convex
    /// combination of these.
    pub fn weight_rates(&self) -> impl Iterator<Item = T> + '_ {
        self.terms.iter().map(RationalPairTerm::weight_rate)
    }

    /// The least active weight, which `W` is at least.
    #[must_use]
    pub fn least_weight(&self) -> T {
        self.least
    }

    /// The first-order bound, `max ‖first vertex‖ / least`.
    #[must_use]
    pub fn speed_sup(&self) -> T {
        norm_max(self.first_vertices()) / self.least
    }

    /// The second-order bound (module docs): `(max ‖second vertex‖ +
    /// 2·max |weight rate|·`[`Self::speed_sup`]`) / least`, the first
    /// maximum `0` at degree 1, where there is no second difference.
    #[must_use]
    pub fn accel_sup(&self) -> T {
        let curvature = if self.second.is_empty() {
            T::zero()
        } else {
            norm_max(self.second_vertices())
        };
        let two = T::from_f64(2.0);
        (curvature + two * abs_max(self.weight_rates()) * self.speed_sup()) / self.least
    }
}

/// Which second partial of a tensor patch.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SecondPartial {
    /// `S_uu`.
    Uu,
    /// `S_uv`.
    Uv,
    /// `S_vv`.
    Vv,
}

/// **One nonempty cell's pair terms** of a rational tensor patch
/// (module docs): the surface twin of [`RationalSpanTerms`], per
/// direction. `W·S_u` lies in the convex hull of
/// [`Self::first_vertices`]`(true)`, `W·S_uu + 2·W_u·S_u` in that of
/// [`Self::second_vertices`]`(Uu)`, `W·S_uv + W_v·S_u + W_u·S_v` in that
/// of `(Uv)`, `W_u` between the extremes of
/// [`Self::weight_rates`]`(true)`, and `W` is at least
/// [`Self::least_weight`]; `v` symmetric.
#[derive(Clone, Debug)]
pub struct RationalCellTerms<T, const D: usize> {
    /// `u` pairs, `[(i − i₀)·nb + (b − b₀)]`: pair `i` along `u` in
    /// column `b` of the cell's `v` window.
    u: Vec<RationalPairTerm<T, D>>,
    /// `v` pairs, `[(i − i₀)·na + (a − a₀)]`: pair `i` along `v` in row
    /// `a` of the cell's `u` window.
    v: Vec<RationalPairTerm<T, D>>,
    /// Second-difference scales along `u` (empty at degree 1).
    u2: Vec<T>,
    /// Second-difference scales along `v`.
    v2: Vec<T>,
    /// `v`'s pair scales, which weight the cross differences.
    v1: Vec<T>,
    /// The `u` window's width.
    na: usize,
    /// The `v` window's width.
    nb: usize,
    active: Vec<[T; D]>,
    least: T,
}

impl<T: Real, const D: usize> RationalCellTerms<T, D> {
    /// The terms of the nonempty cell `(su, sv)`, for the rational patch
    /// with row-major `points` and `weights` (`nv` per `u` row,
    /// `[iu·nv + iv]`) over the `held_u` / `held_v` knot values of the
    /// vectors the spans index. `None` on an index past an array.
    #[must_use]
    pub fn new(
        (su, sv): (Span<'_>, Span<'_>),
        (held_u, held_v): (&[T], &[T]),
        points: &[[T; D]],
        weights: &[T],
        nv: usize,
    ) -> Option<Self> {
        let at = |iu: usize, iv: usize| -> Option<([T; D], T)> {
            let k = iu.checked_mul(nv)?.checked_add(iv)?;
            Some((*points.get(k)?, *weights.get(k)?))
        };
        let (p, q) = (su.degree(), sv.degree());
        let mut active = Vec::with_capacity((p + 1) * (q + 1));
        let mut least: Option<T> = None;
        for iu in su.window() {
            for iv in sv.window() {
                let (pt, w) = at(iu, iv)?;
                active.push(pt);
                least = Some(least.map_or(w, |m| m.min(w)));
            }
        }
        let u = su
            .first_derived_window()
            .flat_map(|i| sv.window().map(move |b| (i, b)))
            .map(|(i, b)| {
                Some(RationalPairTerm::new(
                    pair_scale(held_u, p, i)?,
                    at(i, b)?,
                    at(i + 1, b)?,
                ))
            })
            .collect::<Option<Vec<_>>>()?;
        let v = sv
            .first_derived_window()
            .flat_map(|i| su.window().map(move |a| (i, a)))
            .map(|(i, a)| {
                Some(RationalPairTerm::new(
                    pair_scale(held_v, q, i)?,
                    at(a, i)?,
                    at(a, i + 1)?,
                ))
            })
            .collect::<Option<Vec<_>>>()?;
        Some(Self {
            u,
            v,
            u2: second_scales(su, held_u)?,
            v2: second_scales(sv, held_v)?,
            v1: sv
                .first_derived_window()
                .map(|c| pair_scale(held_v, q, c))
                .collect::<Option<Vec<_>>>()?,
            na: p + 1,
            nb: q + 1,
            active,
            least: least?,
        })
    }

    fn pairs(&self, along_u: bool) -> &[RationalPairTerm<T, D>] {
        if along_u { &self.u } else { &self.v }
    }

    /// Every pair term along `u` (`along_u`) or `v` at every active
    /// point: `W·S_u` (or `W·S_v`) is a convex combination of these.
    pub fn first_vertices(&self, along_u: bool) -> impl Iterator<Item = [T; D]> + '_ {
        self.pairs(along_u)
            .iter()
            .flat_map(move |t| self.active.iter().map(move |q| t.at(*q)))
    }

    /// The second-order terms of `which` at every active point (type
    /// docs): `k²ⱼ·(b_{j+1} − bⱼ)` over one direction's adjacent pairs
    /// in each line of the other, or, for `Uv`, `k_c·(b_{i,c+1} −
    /// b_{i,c})` over the `u` pairs of adjacent columns. Empty for a
    /// degree-1 direction's own second partial.
    #[must_use]
    pub fn second_vertices(&self, which: SecondPartial) -> Vec<[T; D]> {
        let mut out = Vec::new();
        let mut push = |k: T, a: &RationalPairTerm<T, D>, b: &RationalPairTerm<T, D>| {
            out.extend(self.active.iter().map(|q| second_difference(k, a, b, *q)));
        };
        match which {
            SecondPartial::Uu | SecondPartial::Vv => {
                let (pairs, scales, width) = if which == SecondPartial::Uu {
                    (&self.u, &self.u2, self.nb)
                } else {
                    (&self.v, &self.v2, self.na)
                };
                for (j, k) in scales.iter().enumerate() {
                    for o in 0..width {
                        push(*k, &pairs[j * width + o], &pairs[(j + 1) * width + o]);
                    }
                }
            }
            SecondPartial::Uv => {
                for i in 0..self.u.len() / self.nb {
                    for (c, k) in self.v1.iter().enumerate() {
                        push(*k, &self.u[i * self.nb + c], &self.u[i * self.nb + c + 1]);
                    }
                }
            }
        }
        out
    }

    /// Each pair's weight rate along `u` (`along_u`) or `v`: `W_u` (or
    /// `W_v`) is a convex combination of these.
    pub fn weight_rates(&self, along_u: bool) -> impl Iterator<Item = T> + '_ {
        self.pairs(along_u)
            .iter()
            .map(RationalPairTerm::weight_rate)
    }

    /// The least active weight, which `W` is at least.
    #[must_use]
    pub fn least_weight(&self) -> T {
        self.least
    }

    /// The first-order bound along one direction, `max ‖first vertex‖ /
    /// least`.
    #[must_use]
    pub fn stretch_sup(&self, along_u: bool) -> T {
        norm_max(self.first_vertices(along_u)) / self.least
    }
}

/// The pair-term bound on `sup ‖C′‖` (module docs) over the nonempty
/// spans `spans` of `knots` (indices into its vector, empty ones
/// skipped): their max, or poison when none is nonempty. The weights
/// are positive by construction; a refinement that rounds one to zero
/// divides by a zero least weight, which reads `+∞` or NaN, never a
/// low finite number.
#[must_use]
pub fn rational_speed_sup<T: SpanLocate, const D: usize>(
    knots: &Knots<T>,
    points: &[[T; D]],
    weights: &[T],
    spans: core::ops::RangeInclusive<usize>,
) -> T {
    let kv = knots.structure();
    spans
        .filter_map(|index| kv.span(index))
        .map(|span| {
            RationalSpanTerms::new(span, knots.values(), points, weights)
                .map_or_else(|| T::from_f64(f64::NAN), |t| t.speed_sup())
        })
        .reduce(T::max)
        .unwrap_or_else(|| T::from_f64(f64::NAN))
}

/// The pair-term bound on `sup ‖C″‖` (module docs, "The second
/// order") over the nonempty spans `spans` of `knots`: their max, or
/// poison when none is nonempty. As [`rational_speed_sup`], a zero
/// least weight reads `+∞` or NaN, never a low finite number.
#[must_use]
pub fn rational_accel_sup<T: SpanLocate, const D: usize>(
    knots: &Knots<T>,
    points: &[[T; D]],
    weights: &[T],
    spans: core::ops::RangeInclusive<usize>,
) -> T {
    let kv = knots.structure();
    spans
        .filter_map(|index| kv.span(index))
        .map(|span| {
            RationalSpanTerms::new(span, knots.values(), points, weights)
                .map_or_else(|| T::from_f64(f64::NAN), |t| t.accel_sup())
        })
        .reduce(T::max)
        .unwrap_or_else(|| T::from_f64(f64::NAN))
}

/// The pair-term bound on `sup ‖S_u‖` (`along_u`) or `sup ‖S_v‖` of
/// the rational tensor patch over `knots_u` × `knots_v` (module docs),
/// with row-major `points` and `weights` (`[iu·nv + iv]`): the max over
/// the nonempty cells, or poison when none is nonempty. A zero weight
/// reads as it does in [`rational_speed_sup`].
#[must_use]
pub fn rational_stretch_sup<T: Real, const D: usize>(
    (knots_u, knots_v): (&Knots<T>, &Knots<T>),
    points: &[[T; D]],
    weights: &[T],
    along_u: bool,
) -> T {
    let (ku, kv) = (knots_u.structure(), knots_v.structure());
    let nv = kv.control_count();
    let mut sup: Option<T> = None;
    for su in (ku.first_span()..=ku.last_span()).filter_map(|i| ku.span(i)) {
        for sv in (kv.first_span()..=kv.last_span()).filter_map(|i| kv.span(i)) {
            let b = RationalCellTerms::new(
                (su, sv),
                (knots_u.values(), knots_v.values()),
                points,
                weights,
                nv,
            )
            .map_or_else(|| T::from_f64(f64::NAN), |t| t.stretch_sup(along_u));
            sup = Some(sup.map_or(b, |m| m.max(b)));
        }
    }
    sup.unwrap_or_else(|| T::from_f64(f64::NAN))
}

#[cfg(test)]
#[allow(clippy::cast_precision_loss)]
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
