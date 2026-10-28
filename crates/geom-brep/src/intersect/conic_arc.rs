//! **The plane×cone parabola and hyperbola** (C1): no curve kind holds
//! either, so the section is kept in closed form and each arc of it is
//! minted per window as an exact rational-quadratic `Curve3::Nurbs`,
//! never marched.
//!
//! # The section in the plane
//!
//! Write the plane's unit normal `n`, the cone's unit axis `a`, apex `A`
//! and half-angle `α`; `c = a·n`, `s = ‖a × n‖`, `δ = (A − q)·n` the apex
//! gap, `D = sin α·s − cos α·|c|` and `M = sin α·s + cos α·|c|` the
//! conic-type discriminant and its partner, and `K = c² − sin²α = −D·M`
//! ([`ConeTilt`], which the ellipse arm reads too). In the plane's frame
//! `û = (a × n)/s × n`, `v̂ = n × û`, about the apex's foot
//! `F = A − δ·n`, a point `F + ξ·û + η·v̂` is on the cone (both nappes)
//! exactly where
//!
//! ```text
//! −K·ξ² + 2δcs·ξ + δ²(c² − cos²α) − cos²α·η² = 0,
//! ```
//!
//! since `a = c·n − s·û`. Its roots at `η = 0` are the vertices, and the
//! discriminant of that quadratic is `(2|δ|·sin α·cos α)²` exactly, so
//! both are read without cancellation. With `σ = copysign(1, c)` (`−1`
//! at `c = −0`; at `c = ±0` the vertices are mirror images about `F` and
//! either sign names them consistently) and the centre at `ξ_C = cδs/K`,
//!
//! ```text
//! near vertex  ξₙ = δσ·(s·cos α − |c|·sin α) / M      (finite at K = 0)
//! far vertex   ξ_f = ξ_C + δσ·sin α·cos α / K         (hyperbola only)
//! ```
//!
//! About a vertex `V`, with `X = ξ − ξ_V`, the curve is
//! `−K·X² + L·X = cos²α·η²`, `L = ±2δσ·sin α·cos α` (`+` at the near
//! vertex): the slope `L` of the quadratic at its root.
//!
//! # The arc
//!
//! **A parabola** (`K = 0`) is `X = L·μ²/cos²α`, `η = L·μ/cos²α` in the
//! slope `μ = X/η` of the line from its vertex, and the parameter of a
//! window end `P` is `μ = η·cos²α/L` at its height `η = (P − V)·v̂`. Over
//! `[μ₀, μ₁]` that is one polynomial quadratic, `w = 1`, whose middle
//! control point is the numerator's blossom, where the end tangents meet:
//! one span at any length. ([`ConicBranch::point`] and
//! [`ConicBranch::slope_of`] read the same family
//! `X = L·μ²/(K·μ² + cos²α)` on either conic.)
//!
//! **A hyperbola** is read in its hyperbolic parameter `t`:
//! `X = −L·sinh²(t/2)/K`, `η = b·sinh t` (`b = |L|/(2·√|K|·cos α)`),
//! carried as `E = e^{t/2}`, so every quantity below is algebraic and
//! the arc is minted at any scalar: a window end at height `η` has
//! `e^t = S + √(1 + S²)` at `S = η/b ≥ 0` and `1/(√(1 + S²) − S)` below.
//! The tangents of `[t₀, t₁]` meet at
//! `(−L·sinh(t₀/2)·sinh(t₁/2)/K, b·sinh((t₀ + t₁)/2))/cosh h`,
//! `h = (t₁ − t₀)/2`, `cosh h = (E₁/E₀ + E₀/E₁)/2`, and the span is the
//! rational quadratic Bézier on the two ends and that shoulder with
//! weights `(1, cosh h, 1)`: the weight that puts its mid-parameter
//! point on the section's point at `(t₀ + t₁)/2`. **A window is cut
//! into `2^k` spans equal in `t`, for the least `k` that brings each
//! span's weight to at most [`SPAN_WEIGHT_MAX`]** (`√2`, the reciprocal
//! of a quarter circle's weight), each break's `E` the geometric mean of
//! its neighbours'. The spans carry one weight, and the hyperbola's
//! boosts map each to the next, so with unit knot spacing the arc is
//! `C¹` across its double interior knots.

//! # The window
//!
//! Each minted end is metered against the end it was given
//! (`pn_conic_window_on`), and the window refuses where they stand
//! apart. The meter reads an absolute gap in metres, so a window so far
//! down a branch that the closed form's rounding there exceeds the band
//! refuses too, although its ends are on the section.
//!
//! **A parabola decided Zero is a hyperbola or an ellipse whose `D` lies
//! in the zero band**, minted with `K` taken as zero. At a given height
//! the minted point and the section's stand apart by an amount that
//! grows with `|η|` (the dropped `K·X²` term, `X` growing with `|η|`),
//! so over a window it is greatest at the end of greatest `|η|`, one of
//! the two ends, and the ends are metered: a window whose ends pass the
//! meter has every point within it.

use core::num::NonZeroUsize;
use geom::{Curve3, NurbsCurve3};

use geom_core::{
    Band, Decided, Indeterminate, Margin, MarginDiag, Point3, Real, Sign, SplineError, Vec3,
};

use crate::dihedral::{decide, decide_reported};
use geom_core::Decide;

/// The largest weight a hyperbolic span is minted with (module docs).
pub const SPAN_WEIGHT_MAX: f64 = core::f64::consts::SQRT_2;

/// Which open conic a plane×cone section is.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OpenConic {
    /// The plane lies parallel to a generator.
    Parabola,
    /// The plane meets both nappes.
    Hyperbola,
}

impl OpenConic {
    /// The conic's name, with its article.
    #[must_use]
    pub const fn named(self) -> &'static str {
        match self {
            Self::Parabola => "a parabola",
            Self::Hyperbola => "a hyperbola",
        }
    }
}

/// **A plane×cone parabola or hyperbola, in closed form** (module docs).
/// [`OpenConicSection::branch_at`] picks a branch by a seed, and
/// [`ConicBranch::arc`] mints the arc over a window of it.
#[derive(Clone, Debug)]
pub struct OpenConicSection<T: Real> {
    conic: OpenConic,
    /// The apex's foot on the plane.
    foot: Point3<T>,
    /// The plane's in-plane axis direction `û`.
    u: Vec3<T>,
    /// `n × û`.
    v: Vec3<T>,
    /// The apex gap `δ`, and `δ` times the sign `σ` of `a·n`.
    delta: T,
    delta_sigma: T,
    /// The tilt's closed form, its `K` zero exactly on a parabola.
    tilt: ConeTilt<T>,
}

/// **The closed form of a plane×cone section off its tilt** (module
/// docs): `c = a·n`, `s = ‖a × n‖`, the half-angle's sine and cosine,
/// `M = sin α·s + cos α·|c|` and `K = −D·M`. The ellipse arm and the
/// open conic both read their axes and centre here.
#[derive(Clone, Copy, Debug)]
pub(super) struct ConeTilt<T: Real> {
    pub(super) c: T,
    pub(super) s: T,
    pub(super) sin_a: T,
    pub(super) cos_a: T,
    pub(super) m: T,
    /// `K = c² − sin²α`, read as `−D·M`: where the plane is near square
    /// to the axis or the cone near flat, `c` and `sin α` sit near 1 and
    /// carry the tilt and the aperture's complement only to `u` over
    /// those small angles, which the difference of squares hands to `K`
    /// whole.
    pub(super) k: T,
}

impl<T: Real> ConeTilt<T> {
    /// The tilt of `c`, `s` and the half-angle, whose conic-type
    /// discriminant is `discr = sin α·s − cos α·|c|`.
    pub(super) fn new(c: T, s: T, (sin_a, cos_a): (T, T), discr: T) -> Self {
        let m = sin_a * s + cos_a * c.abs();
        Self {
            c,
            s,
            sin_a,
            cos_a,
            m,
            k: (T::zero() - discr) * m,
        }
    }

    /// The centre's offset `c·δ·s/K` along `û` from the apex's foot.
    pub(super) fn centre(&self, delta: T) -> T {
        self.c * delta * self.s / self.k
    }

    /// The centre-to-vertex distance `δ·sin α·cos α/K` along `û` (the
    /// ellipse's semi-major axis at `δ = |δ|`).
    pub(super) fn half_axis(&self, delta: T) -> T {
        delta * self.sin_a * self.cos_a / self.k
    }

    /// The near vertex `δσ·(s·cos α − |c|·sin α)/M` along `û`, the
    /// centre less the half axis read without their cancellation, finite
    /// at `K = 0`.
    pub(super) fn near_vertex(&self, delta_sigma: T) -> T {
        delta_sigma * (self.s * self.cos_a - self.c.abs() * self.sin_a) / self.m
    }
}

/// One branch of an [`OpenConicSection`] about its vertex: the curve
/// `−K·X² + L·X = cos²α·η²` in the frame `(û, v̂)` at `vertex`.
#[derive(Clone, Debug)]
pub struct ConicBranch<T: Real> {
    conic: OpenConic,
    vertex: Point3<T>,
    u: Vec3<T>,
    v: Vec3<T>,
    l: T,
    k: T,
    cos2: T,
}

/// Which end of a window a refusal is about.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WindowEnd {
    /// The window's first point.
    From,
    /// The window's last point.
    To,
}

/// A refusal of the open-conic arc doors.
#[derive(Clone, Debug, PartialEq)]
pub enum ConicArcError {
    /// A decision landed in the ambiguity band (F6).
    Escalated(Indeterminate),
    /// The seed stands on the hyperbola's conjugate axis, between its two
    /// branches, so it names neither.
    SeedBetweenBranches,
    /// A window end does not stand on the branch: the branch's point at
    /// that end's height is `gap` metres from it.
    WindowOffSection {
        /// Which end.
        end: WindowEnd,
        /// The distance, in metres, as the classifier saw it.
        gap: MarginDiag,
    },
    /// The window's two ends are one point of the branch.
    EmptyWindow,
    /// The closed form read a value that is not finite.
    NotFinite,
    /// The hyperbolic window's span weight was not read at or under
    /// [`SPAN_WEIGHT_MAX`] within the halvings any finite window needs
    /// (an infinite or unreadable `cosh` of its half span).
    SpansUnsettled,
    /// The NURBS constructor refused the minted net.
    Carrier(SplineError),
}

impl core::fmt::Display for ConicArcError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Escalated(diag) => write!(
                f,
                "the open conic's arc is ill-conditioned at this tolerance: {diag}"
            ),
            Self::SeedBetweenBranches => write!(
                f,
                "the seed stands between the hyperbola's two branches, so it names neither"
            ),
            Self::WindowOffSection { end, gap } => write!(
                f,
                "the window's {} end stands {gap} m off the section's branch",
                match end {
                    WindowEnd::From => "first",
                    WindowEnd::To => "last",
                }
            ),
            Self::EmptyWindow => write!(f, "the window's two ends are one point of the section"),
            Self::NotFinite => write!(f, "the open conic's closed form read a non-finite value"),
            Self::SpansUnsettled => write!(
                f,
                "the open conic's window is too long for its spans' weight to be read: its \
                 half span's cosh is not finite"
            ),
            Self::Carrier(e) => write!(f, "the open conic's arc refused as a NURBS curve: {e}"),
        }
    }
}

impl std::error::Error for ConicArcError {}

impl<T: Real> OpenConicSection<T> {
    /// The section of a plane `(q, n)` and a cone with apex `apex` at
    /// `tilt`, whose conic-type discriminant has been decided zero
    /// (`conic` the parabola, `K` taken as zero) or positive (the
    /// hyperbola). `u` is the plane's in-plane axis direction
    /// ([`super::tilted_section_major`]).
    pub(super) fn new(
        conic: OpenConic,
        (q, n): (Point3<T>, Vec3<T>),
        apex: Point3<T>,
        tilt: ConeTilt<T>,
        u: Vec3<T>,
    ) -> Self {
        let delta = (apex - q).dot(n);
        let tilt = match conic {
            OpenConic::Parabola => ConeTilt {
                k: T::zero(),
                ..tilt
            },
            OpenConic::Hyperbola => tilt,
        };
        Self {
            conic,
            foot: apex - n * delta,
            u,
            v: n.cross(u),
            delta,
            delta_sigma: delta * T::one().copysign(tilt.c),
            tilt,
        }
    }

    /// Which conic the section is.
    #[must_use]
    pub fn conic(&self) -> OpenConic {
        self.conic
    }

    /// The branch about the near vertex: the parabola's only one, and
    /// the hyperbola's branch that stays finite as the plane tilts to
    /// a generator.
    fn near(&self) -> ConicBranch<T> {
        self.branch(self.tilt.near_vertex(self.delta_sigma), T::one())
    }

    /// The hyperbola's other branch, a half axis past the centre on the
    /// side away from the near vertex.
    fn far(&self) -> ConicBranch<T> {
        let xi = self.tilt.centre(self.delta) + self.tilt.half_axis(self.delta_sigma);
        self.branch(xi, T::zero() - T::one())
    }

    /// The branch about the vertex at `xi` along `û`, with
    /// `L = side·2δσ·sin α·cos α`.
    fn branch(&self, xi: T, side: T) -> ConicBranch<T> {
        let ConeTilt {
            sin_a, cos_a, k, ..
        } = self.tilt;
        ConicBranch {
            conic: self.conic,
            vertex: self.foot + self.u * xi,
            u: self.u,
            v: self.v,
            l: side * self.delta_sigma * sin_a * cos_a * T::from_f64(2.0),
            k,
            cos2: cos_a.powi(2),
        }
    }
}

impl<T: Decide> OpenConicSection<T> {
    /// **The branch `seed` stands beside**: the parabola's one branch,
    /// or the hyperbola's on `seed`'s side of its conjugate axis, read
    /// as `seed`'s signed distance from that axis in the plane, in
    /// metres. The seed need not be on the section.
    ///
    /// # Errors
    ///
    /// [`ConicArcError::Escalated`] where that distance is in the band,
    /// and [`ConicArcError::SeedBetweenBranches`] where it is zero.
    pub fn branch_at(&self, seed: Point3<T>, band: Band) -> Result<ConicBranch<T>, ConicArcError> {
        if self.conic == OpenConic::Parabola {
            return Ok(self.near());
        }
        // The near vertex lies on the side `sign(δσ)` of the centre.
        let centre = self.tilt.centre(self.delta);
        let side = ((seed - self.foot).dot(self.u) - centre) * T::one().copysign(self.delta_sigma);
        match decide("pn_conic_branch", Margin::of(side), band).map_err(ConicArcError::Escalated)? {
            Sign::Positive => Ok(self.near()),
            Sign::Negative => Ok(self.far()),
            Sign::Zero => Err(ConicArcError::SeedBetweenBranches),
        }
    }
}

impl<T: Real> ConicBranch<T> {
    /// Which conic the branch is of.
    #[must_use]
    pub fn conic(&self) -> OpenConic {
        self.conic
    }

    /// The branch's vertex.
    #[must_use]
    pub fn vertex(&self) -> Point3<T> {
        self.vertex
    }

    /// The branch's point at slope `μ` from its vertex (module docs).
    #[must_use]
    pub fn point(&self, mu: T) -> Point3<T> {
        let d = self.k * mu.powi(2) + self.cos2;
        self.vertex + self.u * (self.l * mu.powi(2) / d) + self.v * (self.l * mu / d)
    }

    /// The slope `μ` of the branch's point at `p`'s height
    /// `η = (p − V)·v̂`: the root of `η·K·μ² − L·μ + η·cos²α = 0` finite
    /// at `η = 0`, `μ = 2η·cos²α / (L + sign(L)·√(L² − 4K·η²·cos²α))`, a
    /// sum of like signs for `K ≤ 0`.
    #[must_use]
    pub fn slope_of(&self, p: Point3<T>) -> T {
        let eta = (p - self.vertex).dot(self.v);
        let root = (self.l.powi(2) - T::from_f64(4.0) * self.k * eta.powi(2) * self.cos2).sqrt();
        T::from_f64(2.0) * eta * self.cos2 / (self.l + root.copysign(self.l))
    }
}

/// A span of a branch as a rational quadratic Bézier segment with
/// weights `(1, weight, 1)`.
struct Span<T: Real> {
    ends: (Point3<T>, Point3<T>),
    shoulder: Point3<T>,
    weight: T,
}

impl<T: Real> ConicBranch<T> {
    /// The hyperbola's semi-conjugate axis `b = |L|/(2·√|K|·cos α)`.
    fn conjugate(&self) -> T {
        self.l.abs() / (T::from_f64(2.0) * (T::zero() - self.k).sqrt() * self.cos2.sqrt())
    }

    /// `E = e^{t/2}` of the hyperbola's point at `p`'s height
    /// `η = (p − V)·v̂ = b·sinh t`: `e^t = S + √(1 + S²)` at `S = η/b ≥ 0`
    /// and `1/(√(1 + S²) − S)` below, each a sum of like signs.
    fn half_exp(&self, p: Point3<T>) -> T {
        let s = (p - self.vertex).dot(self.v) / self.conjugate();
        let c = (T::one() + s.powi(2)).sqrt();
        s.select_le_zero(T::one() / (c - s), s + c).sqrt()
    }

    /// The hyperbola's point at `E = e^{t/2}`:
    /// `X = −L·sinh²(t/2)/K`, `η = b·sinh t`.
    fn hyperbolic_point(&self, e: T) -> Point3<T> {
        let two = T::from_f64(2.0);
        let sh = (e - T::one() / e) / two;
        self.vertex
            + self.u * (T::zero() - self.l * sh.powi(2) / self.k)
            + self.v * (self.conjugate() * (e.powi(2) - T::one() / e.powi(2)) / two)
    }

    /// The hyperbola's span between `E₀ = e^{t₀/2}` and `E₁ = e^{t₁/2}`:
    /// the end tangents meet at
    /// `X = −L·sinh(t₀/2)·sinh(t₁/2)/(K·cosh h)`,
    /// `η = b·sinh((t₀ + t₁)/2)/cosh h`, `h = (t₁ − t₀)/2`, and the
    /// weight is `cosh h = (E₁/E₀ + E₀/E₁)/2`.
    fn hyperbolic_span(&self, e0: T, e1: T) -> Span<T> {
        let two = T::from_f64(2.0);
        let ch = (e1 / e0 + e0 / e1) / two;
        let (sh0, sh1) = ((e0 - T::one() / e0) / two, (e1 - T::one() / e1) / two);
        let x = T::zero() - self.l * sh0 * sh1 / (self.k * ch);
        let eta = self.conjugate() * ((e0 * e1 - T::one() / (e0 * e1)) / two) / ch;
        Span {
            ends: (self.hyperbolic_point(e0), self.hyperbolic_point(e1)),
            shoulder: self.vertex + self.u * x + self.v * eta,
            weight: ch,
        }
    }

    /// The parabola's span `[μ₀, μ₁]`: the numerator's blossom over the
    /// denominator's, `cos²α`, at unit weight.
    fn parabolic_span(&self, m0: T, m1: T) -> Span<T> {
        Span {
            ends: (self.point(m0), self.point(m1)),
            shoulder: self.vertex
                + self.u * (self.l * m0 * m1 / self.cos2)
                + self.v * (self.l * (m0 + m1) * T::from_f64(0.5) / self.cos2),
            weight: T::one(),
        }
    }
}

impl<T: Decide> ConicBranch<T> {
    /// **The arc of the branch from `from` to `to`** (module docs): a
    /// degree-2 rational `Curve3::Nurbs` with weights `(1, w, 1)` per
    /// span, on the knot domain `[0, spans]`, running from the branch's
    /// point at `from`'s height to its point at `to`'s.
    ///
    /// # Errors
    ///
    /// [`ConicArcError::WindowOffSection`] where an end stands off the
    /// branch, [`ConicArcError::EmptyWindow`] where the ends are one
    /// point, [`ConicArcError::Escalated`] where either is in the band,
    /// [`ConicArcError::NotFinite`], [`ConicArcError::SpansUnsettled`]
    /// and [`ConicArcError::Carrier`].
    pub fn arc(
        &self,
        from: Point3<T>,
        to: Point3<T>,
        band: Band,
    ) -> Result<Curve3<T>, ConicArcError> {
        let spans: Vec<Span<T>> = match self.conic {
            OpenConic::Parabola => {
                let (m0, m1) = (self.slope_of(from), self.slope_of(to));
                window_on((from, to), (self.point(m0), self.point(m1)), band)?;
                vec![self.parabolic_span(m0, m1)]
            }
            OpenConic::Hyperbola => {
                let (e0, e1) = (self.half_exp(from), self.half_exp(to));
                if !(e0.is_bounded() && e1.is_bounded()) {
                    return Err(ConicArcError::NotFinite);
                }
                window_on(
                    (from, to),
                    (self.hyperbolic_point(e0), self.hyperbolic_point(e1)),
                    band,
                )?;
                // The least `k` whose `2^k` equal spans each weigh at
                // most `√2`: `cosh(h/2) = √((cosh h + 1)/2)`. A count is
                // structure, read on the sign alone, and a reading the
                // scalar cannot place halves once more, which costs a
                // span and never exactness.
                let mut weight = (e1 / e0 + e0 / e1) / T::from_f64(2.0);
                let settled = |weight: T| {
                    matches!(
                        decide(
                            "pn_conic_span_weight",
                            Margin::of(weight - T::from_f64(SPAN_WEIGHT_MAX)),
                            Band::least(),
                        ),
                        Ok(Sign::Zero | Sign::Negative)
                    )
                };
                let mut halvings = 0_u32;
                while !settled(weight) {
                    if halvings == MAX_HALVINGS {
                        return Err(ConicArcError::SpansUnsettled);
                    }
                    weight = ((weight + T::one()) / T::from_f64(2.0)).sqrt();
                    halvings += 1;
                }
                // The `2^k + 1` breaks equal in `t`: each midpoint's
                // `E` is the geometric mean of its neighbours'.
                let mut breaks = vec![e0, e1];
                for _ in 0..halvings {
                    breaks = breaks
                        .windows(2)
                        .flat_map(|w| [w[0], (w[0] * w[1]).sqrt()])
                        .chain(breaks.last().copied())
                        .collect();
                }
                breaks
                    .windows(2)
                    .map(|w| self.hyperbolic_span(w[0], w[1]))
                    .collect()
            }
        };
        let mut control = vec![spans[0].ends.0];
        let mut weights = vec![T::one()];
        // The knots `0, 1, …, spans`, ends of multiplicity three and
        // interior ones double, counted up in `T`.
        let mut at = T::zero();
        let three = NonZeroUsize::MIN.saturating_add(2);
        let mut runs = vec![(at, three)];
        for (i, span) in spans.iter().enumerate() {
            control.extend([span.shoulder, span.ends.1]);
            weights.extend([span.weight, T::one()]);
            at = at + T::one();
            runs.push((
                at,
                if i + 1 == spans.len() {
                    three
                } else {
                    NonZeroUsize::MIN.saturating_add(1)
                },
            ));
        }
        if control
            .iter()
            .any(|p| !(p.x.is_bounded() && p.y.is_bounded() && p.z.is_bounded()))
            || weights.iter().any(|w| !w.is_bounded())
        {
            return Err(ConicArcError::NotFinite);
        }
        let knots = geom_core::spline::Knots::new(runs, 2).map_err(ConicArcError::Carrier)?;
        let curve = NurbsCurve3::new(knots, control, weights).map_err(ConicArcError::Carrier)?;
        Ok(Curve3::Nurbs(std::sync::Arc::new(curve)))
    }
}

/// The most halvings a hyperbolic window takes: a window whose ends'
/// `E = e^{t/2}` are finite has `|t₁ − t₀| < 1420`, and `2¹⁶` spans of
/// weight `√2` cover `2¹⁶·2·acosh √2 > 10⁵` of `t`. A weight still not
/// read at or under `√2` past them is not a finite window's
/// ([`ConicArcError::SpansUnsettled`]).
const MAX_HALVINGS: u32 = 16;

/// The window's ends `given` against the branch's points `read` at their
/// heights: each within the band of its own, and the two apart.
fn window_on<T: Decide>(
    given: (Point3<T>, Point3<T>),
    read: (Point3<T>, Point3<T>),
    band: Band,
) -> Result<(), ConicArcError> {
    for (end, p, q) in [
        (WindowEnd::From, given.0, read.0),
        (WindowEnd::To, given.1, read.1),
    ] {
        match decide_reported("pn_conic_window_on", Margin::of(q.distance(p)), band)
            .map_err(ConicArcError::Escalated)?
        {
            Decided {
                sign: Sign::Zero, ..
            } => {}
            Decided { margin, .. } => {
                return Err(ConicArcError::WindowOffSection { end, gap: margin });
            }
        }
    }
    match decide(
        "pn_conic_window_span",
        Margin::of(read.0.distance(read.1)),
        band,
    )
    .map_err(ConicArcError::Escalated)?
    {
        Sign::Positive => Ok(()),
        Sign::Zero | Sign::Negative => Err(ConicArcError::EmptyWindow),
    }
}
