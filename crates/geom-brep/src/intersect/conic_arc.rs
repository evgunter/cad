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
//! `X = −L·sinh²(t/2)/K`, `η = b·sinh t` (`b = |L|/(2·√|K|·cos α)`), and
//! a window end's parameter is `t = asinh(η/b)`. The tangents of
//! `[t₀, t₁]` meet at
//! `(−L·sinh(t₀/2)·sinh(t₁/2)/K, b·sinh((t₀ + t₁)/2))/cosh h`,
//! `h = (t₁ − t₀)/2`, and the span is the rational quadratic Bézier on
//! the two ends and that shoulder with weights `(1, cosh h, 1)`: the
//! weight that puts its mid-parameter point on the section's point at
//! `(t₀ + t₁)/2`. **A window is cut into `2^k` spans equal in `t`, for
//! the least `k` that brings each span's weight to at most
//! [`SPAN_WEIGHT_MAX`]** (`√2`, the reciprocal of a quarter circle's
//! weight). The spans carry one weight, and the hyperbola's boosts map
//! each to the next, so with unit knot spacing the arc is `C¹` across
//! its double interior knots.
//!
//! The arc is minted at `f64`.
//!
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

use geom::{Curve3, NurbsCurve3};
use geom_core::{Band, Indeterminate, KnotVector, Margin, Point3, Real, Sign, SplineError, Vec3};

use crate::dihedral::decide;
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
        /// The distance, in metres.
        gap: f64,
    },
    /// The window's two ends are one point of the branch.
    EmptyWindow,
    /// The closed form read a value that is not finite.
    NotFinite,
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
                "the window's {} end stands {gap:e} m off the section's branch",
                match end {
                    WindowEnd::From => "first",
                    WindowEnd::To => "last",
                }
            ),
            Self::EmptyWindow => write!(f, "the window's two ends are one point of the section"),
            Self::NotFinite => write!(f, "the open conic's closed form read a non-finite value"),
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
struct Span {
    ends: (Point3<f64>, Point3<f64>),
    shoulder: Point3<f64>,
    weight: f64,
}

impl ConicBranch<f64> {
    /// The hyperbola's semi-conjugate axis `b = |L|/(2·√|K|·cos α)`.
    fn conjugate(&self) -> f64 {
        self.l.abs() / (2.0 * (-self.k).sqrt() * self.cos2.sqrt())
    }

    /// The hyperbola's point at hyperbolic parameter `t`:
    /// `X = −L·sinh²(t/2)/K`, `η = b·sinh t`.
    fn hyperbolic_point(&self, t: f64) -> Point3<f64> {
        let sh = (0.5 * t).sinh();
        self.vertex
            + self.u * (-self.l * sh.powi(2) / self.k)
            + self.v * (self.conjugate() * t.sinh())
    }

    /// The hyperbola's span `[t₀, t₁]`: the end tangents meet at
    /// `X = −L·sinh(t₀/2)·sinh(t₁/2)/(K·cosh h)`,
    /// `η = b·sinh((t₀ + t₁)/2)/cosh h`, `h = (t₁ − t₀)/2`, and the
    /// weight is `cosh h`.
    fn hyperbolic_span(&self, t0: f64, t1: f64) -> Span {
        let ch = (0.5 * (t1 - t0)).cosh();
        let x = -self.l * (0.5 * t0).sinh() * (0.5 * t1).sinh() / (self.k * ch);
        let eta = self.conjugate() * (0.5 * (t0 + t1)).sinh() / ch;
        Span {
            ends: (self.hyperbolic_point(t0), self.hyperbolic_point(t1)),
            shoulder: self.vertex + self.u * x + self.v * eta,
            weight: ch,
        }
    }

    /// The parabola's span `[μ₀, μ₁]`: the numerator's blossom over the
    /// denominator's, `cos²α`, at unit weight.
    fn parabolic_span(&self, m0: f64, m1: f64) -> Span {
        Span {
            ends: (self.point(m0), self.point(m1)),
            shoulder: self.vertex
                + self.u * (self.l * m0 * m1 / self.cos2)
                + self.v * (self.l * (m0 + m1) * 0.5 / self.cos2),
            weight: 1.0,
        }
    }

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
    /// [`ConicArcError::NotFinite`] and [`ConicArcError::Carrier`].
    pub fn arc(
        &self,
        from: Point3<f64>,
        to: Point3<f64>,
        band: Band,
    ) -> Result<Curve3<f64>, ConicArcError> {
        let spans: Vec<Span> = match self.conic {
            OpenConic::Parabola => {
                let (m0, m1) = (self.slope_of(from), self.slope_of(to));
                window_on((from, to), (self.point(m0), self.point(m1)), band)?;
                vec![self.parabolic_span(m0, m1)]
            }
            OpenConic::Hyperbola => {
                let b = self.conjugate();
                let height = |p: Point3<f64>| ((p - self.vertex).dot(self.v) / b).asinh();
                let (t0, t1) = (height(from), height(to));
                if !(t0.is_finite() && t1.is_finite()) {
                    return Err(ConicArcError::NotFinite);
                }
                window_on(
                    (from, to),
                    (self.hyperbolic_point(t0), self.hyperbolic_point(t1)),
                    band,
                )?;
                let mut count = 1_u32;
                while (0.5 * (t1 - t0) / f64::from(count)).cosh() > SPAN_WEIGHT_MAX {
                    count *= 2;
                }
                let step = (t1 - t0) / f64::from(count);
                (0..count)
                    .map(|i| {
                        let a = t0 + step * f64::from(i);
                        let z = if i + 1 == count { t1 } else { a + step };
                        self.hyperbolic_span(a, z)
                    })
                    .collect()
            }
        };
        let mut control = vec![spans[0].ends.0];
        let mut weights = vec![1.0];
        let mut knots = vec![0.0; 3];
        for (i, span) in spans.iter().enumerate() {
            control.extend([span.shoulder, span.ends.1]);
            weights.extend([span.weight, 1.0]);
            #[allow(clippy::cast_precision_loss)] // a span count far below 2⁵³
            let k = (i + 1) as f64;
            knots.extend(if i + 1 == spans.len() {
                vec![k; 3]
            } else {
                vec![k; 2]
            });
        }
        if control
            .iter()
            .any(|p| !(p.x.is_finite() && p.y.is_finite() && p.z.is_finite()))
            || weights.iter().any(|w| !w.is_finite())
        {
            return Err(ConicArcError::NotFinite);
        }
        let knots = KnotVector::clamped(knots, 2).map_err(ConicArcError::Carrier)?;
        let curve = NurbsCurve3::new(knots, control, weights).map_err(ConicArcError::Carrier)?;
        Ok(Curve3::Nurbs(std::sync::Arc::new(curve)))
    }
}

/// The window's ends `given` against the branch's points `read` at their
/// heights: each within the band of its own, and the two apart.
fn window_on(
    given: (Point3<f64>, Point3<f64>),
    read: (Point3<f64>, Point3<f64>),
    band: Band,
) -> Result<(), ConicArcError> {
    for (end, p, q) in [
        (WindowEnd::From, given.0, read.0),
        (WindowEnd::To, given.1, read.1),
    ] {
        let gap = q.distance(p);
        match decide("pn_conic_window_on", Margin::of(gap), band)
            .map_err(ConicArcError::Escalated)?
        {
            Sign::Zero => {}
            Sign::Positive | Sign::Negative => {
                return Err(ConicArcError::WindowOffSection { end, gap });
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
