//! **The pose of two carrier spans**, decided once: the one
//! classification the meeting outcome takes as input ([`Pose`]). It is
//! read for the pairs the table gives a closed form ([`super::Method`]):
//! lines, circles and ellipses. A pair with a spiric or spline side has
//! no closed pose and refuses typed ([`PoseRefusal::Walked`]); its
//! meetings are the walk's.
//!
//! # The arms
//!
//! Every length below is metres, and every closed form solves in one
//! carrier's own parameter: a line's, or a conic's angle.
//!
//! - **Two lines.** Their tilt, the sine between them levered by the
//!   longer span (`curve_pair_pose_tilt`). Read zero, the lines are
//!   parallel over both spans, and the second span's middle point's
//!   distance from the first line (`curve_pair_pose_offset`) decides one
//!   line ([`Pose::Coincident`]) or [`Pose::Parallel`]. Read definite, the
//!   candidate is the first line's point nearest the second.
//! - **A line and a conic.** The line's tilt to the conic's plane, `d̂·n̂`
//!   levered by the longer of the line's span and the conic's greatest
//!   speed. Definite: the line crosses the plane once, and the candidate
//!   is the closest pair polished from there (below). Zero: the line's
//!   height above the plane at its span's middle decides
//!   [`Pose::Parallel`], or, read zero, the line in the plane. There the
//!   conic's signed distance from the line, `(C(θ) − o)·m̂` with `m̂` the
//!   line's normal in the plane, is a first harmonic in the conic's angle
//!   (below).
//! - **Two conics.** Each conic's swing above the other's plane, the
//!   amplitude of its height there (`curve_pair_pose_swing`), the first
//!   conic's first. One read zero lies in a plane parallel to the
//!   other's: its height, constant to within that swing, decides
//!   [`Pose::Parallel`] (`curve_pair_pose_offset`), or, read zero, the
//!   conics in one plane to within the slant `|c₀|` plus the swing, below.
//!   Both definite: the first conic's height above the second's plane,
//!   `(C₁(θ) − c₂)·n̂₂`, is a first harmonic in its own angle whose roots
//!   are where it crosses that plane, and each candidate is the closest
//!   pair polished from one (below). In one plane, **one reading decides
//!   whether two conics are one curve**, whatever their kinds
//!   (`curve_pair_pose_one_curve`, `one_curve_gap`): each conic is
//!   `c + L·x` over the unit circle, `L = [a·û, b·v̂]`, and the centres'
//!   gap plus `‖L₁ − L₂Q‖` for the best rotation `Q` bounds how far a
//!   point of either lies from the other, a tilt out of the plane
//!   included. Read zero is [`Pose::Coincident`]. Past it:
//!   - **two circles**: their centres' distance `d`
//!     (`curve_pair_pose_centres`). Zero is one centre, and the radii's gap
//!     with `d` and the slant as its spread (`curve_pair_pose_radii`)
//!     decides [`Pose::Concentric`]; read zero all round, it contradicts
//!     the one-curve reading, which is at most `d + |Δr|` plus the slant
//!     ([`MeetFault::Contradiction`]). Definite, the first circle's distance from
//!     the second's centre runs between `|r₁ − d|` and `r₁ + d`, so its
//!     distance from the second circle has those extremes less `r₂`,
//!     exactly; and its square is a first harmonic in the first circle's
//!     angle, whose roots are read from the extremes (below);
//!   - **a circle and an ellipse**: the ellipse's distance from the
//!     circle, `|E(θ) − c| − r`, whose square less `r²` is of degree two in
//!     the ellipse's angle, goes to the certified subdivision
//!     ([`crate::half_angle::certified_subdivision`]);
//!   - **two ellipses** have no metre residual here, and refuse typed
//!     ([`MeetFault::Unsupported`]).
//!
//! # A first harmonic's roots, and a double root
//!
//! A residual `R(θ)` whose range is `[lo, hi]` is decided on its extremes
//! (`curve_pair_pose_extreme`), each read as the interval its rounding
//! spans ([`crate::roots::extreme_sign`]): definitely one-signed is
//! [`Pose::Apart`], definitely straddling is two crossings, an end in the
//! escalation gap escalates, and an extreme in the zero band is a touch
//! there. The roots are read from the extreme nearer zero
//! ([`crate::roots::arc_past`]).
//!
//! **A double root is decided by the gap between its roots along the
//! curve**, never by the margin of a centre or an extreme: the chord
//! between the two root points (`curve_pair_pose_root_gap`) decides one
//! touch or two crossings. Two near-concentric circles stay within
//! `d + |Δr|` of each other all the way round, so an extreme in the band
//! says nothing about where they meet; the root gap does. Where the
//! extreme is in the zero band, its crossing is read at the least its
//! rounding allows, the extreme less its noise: roots that only the
//! noise puts apart are a double root the band cannot split. Merged so,
//! two crossings can lie up to about `√(8R·noise)` apart along a carrier
//! of radius `R`, each within the zero band of the touch's point.
//!
//! **In one plane, a candidate admitted under a `Zero` reading is placed
//! by the exactly degenerate formula**: a touch is the extreme's own
//! point, `C(θ_hi)` or `C(θ_hi + π)`, on the first carrier exactly and
//! within the extreme's value of the second. An extreme read zero whose
//! value has not crossed (no real root) is that touch too. The residual
//! there is the distance to the other carrier itself, so the reading of
//! the candidate decides it.
//!
//! # Across planes: the closest pair
//!
//! **Where a curve crosses the other's plane is not where the two meet**
//! when they pass within the band without touching: the meeting lies
//! `h/σ` along the curve, `h` their distance and `σ` the sine of the
//! curve's angle to the plane. So a crossing (or a touch) of the other's
//! plane only starts the candidate: it is polished by Gauss–Newton to the
//! two carriers' closest pair, and that pair is read. Every point of the
//! curve within the band's escalation `e` of the other carrier is within
//! `e` of the other's plane, so it lies on the arc about a crossing or a
//! touch where the height is within `e`: `ℓ`, read off the harmonic (or
//! `e/σ` on a line). The pair the polish is after lies within
//! `2(ℓ + e)` of where it starts, on either carrier. Two meters guard it,
//! each refusing [`Unsettled::Meter`] unless definite:
//!
//! - **its basin** (`curve_pair_pose_basin`): over that reach each
//!   tangent turns by at most its carrier's curvature times the reach, so
//!   `sin φ − 2(κ₁ + κ₂)·2(ℓ + e)`, `φ` the angle between the tangents
//!   where the polish starts, levered as the tilt is, must read positive:
//!   the tangents then keep at least half the angle between them,
//!   the closest pair is the one stationary pair there, and the polish
//!   contracts toward it;
//! - **its convergence** (`curve_pair_pose_polish`): the polish's last
//!   step moves the pair within the band's zero.
//!
//! # The spans' ends
//!
//! A touch or a crossing is one point, but two curves near it stay
//! within the band over a run about it, long where they cross at a
//! shallow angle or touch with little relative curvature. Where that
//! point lies past a span and the run reaches into both, the spans meet
//! in the band at an end of one of them, which no isolated candidate
//! reads. So the outcome of a [`Pose::Tangent`] or [`Pose::Generic`]
//! also reads each span's ends against the other ([`end_candidates`]): a
//! span end within the band of the other span is a meeting, and one in
//! the band's gap escalates. Where the point lies at a span's end and
//! the run reaches into it, the meeting read at the end is not the only
//! one: points of the span clear of its end lie within the band of the
//! other too. So a meeting read at an end has its run read once more
//! (`run_candidates`), midway between the escalation and the arc where
//! the carriers part by half what the zero band leaves.
//!
//! # The band
//!
//! Every decision above that falls in the band's gap escalates
//! ([`MeetFault::Escalated`]); none prunes a candidate.

use geom::{Curve3, CurveKind};
use geom_core::k_stats::{decide, decide_reported, straddle};
use geom_core::{Band, Decide, Margin, Point3, Sign, Vec3};

use super::span::Span;
use super::{Arm, MeetFault, Method};
use crate::Conic;
use crate::half_angle::{DegreeTwo, SubdivisionFrame, SubdivisionRows, certified_subdivision};
use crate::roots::{Roots, Unsettled, arc_past, extreme_sign};

/// How two carriers read as one curve run: their parameters increase the
/// same way along it, or opposite ways.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sense {
    /// The same way.
    Same,
    /// Opposite ways.
    Opposite,
}

/// A candidate meeting: a point, and its parameter on each carrier (the
/// carrier's own on the one it was placed on, its foot on the other).
#[derive(Clone, Copy, Debug)]
pub struct Candidate<T: geom_core::Real> {
    /// The point.
    pub point: Point3<T>,
    /// Its parameter on the first carrier.
    pub ta: T,
    /// Its parameter on the second carrier.
    pub tb: T,
}

/// The pose of two spans' carriers (module docs).
#[derive(Clone, Debug)]
pub enum Pose<T: geom_core::Real> {
    /// One curve within the band over the spans. `witnesses` are points of
    /// the carrier that a positive-length overlap of the spans holds one
    /// of ([`Span::witness_params`]).
    Coincident {
        /// How the two parameters run.
        sense: Sense,
        /// The overlap's candidate points.
        witnesses: Vec<Candidate<T>>,
    },
    /// Two circles in one plane on one centre, their radii definitely
    /// apart: no point meets.
    Concentric,
    /// Parallel lines, or a line or conic parallel to the other's plane,
    /// definitely apart: no point meets.
    Parallel,
    /// Definitely apart otherwise: the residual's extremes one-signed.
    Apart,
    /// One touch: a double root the band cannot split, of the residual in
    /// one plane or of the height above the other's plane.
    Tangent(Candidate<T>),
    /// Isolated meetings, at most four, each a candidate to be read.
    Generic(Vec<Candidate<T>>),
}

/// Why [`pose`] has no pose to give.
#[derive(Clone, Copy, Debug)]
pub enum PoseRefusal {
    /// The pair has a spiric or spline side: its meetings are the walk's.
    Walked {
        /// The first carrier's kind.
        a: CurveKind,
        /// The second carrier's kind.
        b: CurveKind,
    },
    /// A closed-form arm could not read the pose ([`MeetFault`]).
    Closed(MeetFault),
}

/// The predicate rows the pose reads under.
mod row {
    pub(super) const TILT: &str = "curve_pair_pose_tilt";
    pub(super) const OFFSET: &str = "curve_pair_pose_offset";
    pub(super) const SENSE: &str = "curve_pair_pose_sense";
    pub(super) const SWING: &str = "curve_pair_pose_swing";
    pub(super) const EXTREME: &str = "curve_pair_pose_extreme";
    pub(super) const ROOT_GAP: &str = "curve_pair_pose_root_gap";
    pub(super) const CLEAR: &str = "curve_pair_pose_clear";
    pub(super) const BASIN: &str = "curve_pair_pose_basin";
    pub(super) const POLISH: &str = "curve_pair_pose_polish";
    pub(super) const CENTRES: &str = "curve_pair_pose_centres";
    pub(super) const RADII: &str = "curve_pair_pose_radii";
    pub(super) const ONE_CURVE: &str = "curve_pair_pose_one_curve";
    pub(super) const RUN: &str = "curve_pair_pose_run";
}

/// The coplanar circle × ellipse subdivision's rows.
const COPLANAR_ROWS: SubdivisionRows = SubdivisionRows {
    clear: "curve_pair_coplanar_sub_clear",
    monotone: "curve_pair_coplanar_sub_monotone",
    side: "curve_pair_coplanar_sub_side",
    width: "curve_pair_coplanar_sub_width",
};

/// **The pose of `a`'s and `b`'s carriers** (module docs), dispatched
/// through the table ([`super::method`]).
///
/// # Errors
///
/// [`PoseRefusal`], as its variants say.
pub fn pose<T: Decide>(
    a: &Span<'_, T>,
    b: &Span<'_, T>,
    band: Band,
) -> Result<Pose<T>, PoseRefusal> {
    let (ka, kb) = (a.carrier().kind(), b.carrier().kind());
    match super::method(ka, kb) {
        Method::Closed(arm) => closed(arm, a, b, band).map_err(PoseRefusal::Closed),
        Method::Walk => Err(PoseRefusal::Walked { a: ka, b: kb }),
    }
}

/// [`pose`] for a closed-form arm.
pub(super) fn closed<T: Decide>(
    arm: Arm,
    a: &Span<'_, T>,
    b: &Span<'_, T>,
    band: Band,
) -> Result<Pose<T>, MeetFault> {
    // Where one side is a circle and the other an ellipse, the ellipse is
    // parametrized, so the reading against the circle is at its exact
    // foot.
    match arm {
        Arm::LineLine => line_line(a, b, band),
        Arm::LineConic => line_conic(a, b, band),
        Arm::ConicLine => Ok(swapped(line_conic(b, a, band)?)),
        Arm::CircleCircle => conic_conic(a, b, Coplanar::Circles, band),
        Arm::EllipseCircle => conic_conic(a, b, Coplanar::EllipseCircle, band),
        Arm::CircleEllipse => Ok(swapped(conic_conic(b, a, Coplanar::EllipseCircle, band)?)),
        Arm::EllipseEllipse => conic_conic(a, b, Coplanar::Ellipses, band),
    }
}

/// A pose read with the spans' roles swapped, given back in their order.
fn swapped<T: geom_core::Real>(pose: Pose<T>) -> Pose<T> {
    let flip = |c: Candidate<T>| Candidate {
        point: c.point,
        ta: c.tb,
        tb: c.ta,
    };
    match pose {
        Pose::Coincident { sense, witnesses } => Pose::Coincident {
            sense,
            witnesses: witnesses.into_iter().map(flip).collect(),
        },
        Pose::Tangent(c) => Pose::Tangent(flip(c)),
        Pose::Generic(cs) => Pose::Generic(cs.into_iter().map(flip).collect()),
        Pose::Concentric => Pose::Concentric,
        Pose::Parallel => Pose::Parallel,
        Pose::Apart => Pose::Apart,
    }
}

/// A line carrier's origin and direction.
fn line_of<T: Decide>(c: &Curve3<T>) -> Result<(Point3<T>, Vec3<T>), MeetFault> {
    match *c {
        Curve3::Line { origin, dir } => Ok((origin, dir)),
        Curve3::Circle { .. }
        | Curve3::Ellipse { .. }
        | Curve3::Spiric { .. }
        | Curve3::Nurbs(_) => Err(MeetFault::Desync),
    }
}

/// A conic carrier's frame.
fn conic_of<T: Decide>(c: &Curve3<T>) -> Result<Conic<T>, MeetFault> {
    Conic::of(c).ok_or(MeetFault::Desync)
}

/// `v` scaled to unit length.
fn unit<T: Decide>(v: Vec3<T>) -> Vec3<T> {
    v * (T::one() / v.norm())
}

/// The metres a span runs at most: its parameter run at the carrier's
/// greatest speed.
fn reach<T: Decide>(s: &Span<'_, T>) -> T {
    let (t0, t1) = s.params();
    Margin::metered_sup((t1 - t0).abs(), s.speeds().1).value()
}

/// The span's middle point.
fn middle<T: Decide>(s: &Span<'_, T>) -> Point3<T> {
    let (t0, t1) = s.params();
    s.carrier().eval(geom::mid_param(t0, t1))
}

/// The conic's tangent `C′(θ)`.
fn tangent<T: Decide>(k: &Conic<T>, theta: T) -> Vec3<T> {
    let (sin, cos) = theta.sin_cos();
    k.u_ref * (T::zero() - k.semi_u * sin) + k.v_ref() * (k.semi_v * cos)
}

/// The conic's foot of `q` ([`Curve3::foot`], the one foot reading).
fn conic_foot<T: Decide>(k: &Conic<T>, q: Point3<T>) -> T {
    Curve3::Ellipse {
        center: k.center,
        axis: k.axis,
        semi_u: k.semi_u,
        semi_v: k.semi_v,
        u_ref: k.u_ref,
    }
    .foot(q, T::zero())
    .t()
}

/// A lower bound on `q`'s distance from the conic: the conic lies in its
/// plane between the circles of its semi-axes about its centre, so `q`
/// is at least its height above the plane, and its distance in the plane
/// outside that ring, apart from it. Exact on a circle.
fn conic_clearance<T: Decide>(k: &Conic<T>, q: Point3<T>) -> T {
    let w = q - k.center;
    let h = w.dot(k.axis);
    let rho = (w - k.axis * h).norm();
    let out = (rho - k.speed_hi()).max(k.speed_lo() - rho).max(T::zero());
    (h.powi(2) + out.powi(2)).sqrt()
}

/// The line's foot of `q` ([`Curve3::foot`], the one foot reading).
fn line_foot<T: Decide>((origin, dir): (Point3<T>, Vec3<T>), q: Point3<T>) -> T {
    Curve3::Line { origin, dir }.foot(q, T::zero()).t()
}

/// `q`'s foot on `s`'s carrier ([`Curve3::foot`]), seeded at the span's
/// middle.
fn foot<T: Decide>(s: &Span<'_, T>, q: Point3<T>) -> T {
    let (t0, t1) = s.params();
    s.carrier().foot(q, geom::mid_param(t0, t1)).t()
}

/// **Each span's ends, read against the other** (module docs, "The
/// spans' ends"): each end's point on its own carrier, with its foot on
/// the other's.
pub(super) fn end_candidates<T: Decide>(a: &Span<'_, T>, b: &Span<'_, T>) -> Vec<Candidate<T>> {
    let mut out = Vec::with_capacity(4);
    let (a0, a1) = a.params();
    for ta in [a0, a1] {
        let point = a.carrier().eval(ta);
        out.push(Candidate {
            point,
            ta,
            tb: foot(b, point),
        });
    }
    let (b0, b1) = b.params();
    for tb in [b0, b1] {
        let point = b.carrier().eval(tb);
        out.push(Candidate {
            point,
            ta: foot(a, point),
            tb,
        });
    }
    out
}

/// **The run about a meeting read at a span's end** (module docs, "The
/// spans' ends"): where the two carriers stay within the zero band of
/// each other past the end, into the span. About `c`, whose points are
/// `g` apart, they part by at most `sin φ·s + (κ₁ + κ₂)·s²/2` over an arc
/// `s` either way, `φ` the angle between their tangents and `κ` each
/// carrier's curvature bound. At the `s₂` where that reaches half of what
/// the zero band leaves, `(z − g)/2`, the run ends as far as this bound
/// can say. The meeting lies within the zero band `z` of the end, so a
/// run that reaches past `e + 2z`, the escalation and that slack twice
/// (`curve_pair_pose_run`, two of the bound's own lengths, read exactly),
/// holds points clear of the end: each carrier's points midway between
/// the two, `(s₂ + e + 2z)/2` either side of `c`, each with its foot on the
/// other, past the escalation from the end and within half the zero band
/// of the other carrier. None where the run is shorter.
pub(super) fn run_candidates<T: Decide>(
    a: &Span<'_, T>,
    b: &Span<'_, T>,
    c: &Candidate<T>,
    band: Band,
) -> Vec<Candidate<T>> {
    let (Some(ka), Some(kb)) = (
        geom::carrier_speed::curvature_hi(a.carrier()),
        geom::carrier_speed::curvature_hi(b.carrier()),
    ) else {
        return Vec::new();
    };
    let ((pa, da), (pb, db)) = (a.carrier().ders1(c.ta), b.carrier().ders1(c.tb));
    let two = T::from_f64(2.0);
    let left = (T::from_f64(band.zero()) - (pa - pb).norm()) / two;
    let sine = unit(da).cross(unit(db)).norm();
    let bend = ka + kb;
    // `sin φ·s + κ·s²/2 = left`, its positive root written so that it
    // does not cancel where `κ` vanishes.
    let s2 = left * two / (sine + (sine.powi(2) + bend * left * two).sqrt());
    // Two of the bound's own lengths compared, not a distance the band
    // resolves: read exactly (the least pair of positive `f64`s), and an
    // enclosure that straddles is read on.
    // The meeting is read within the zero band of the end, so a point the
    // run puts `o` from it lies at least `o − z` from the end: clear of it
    // past the escalation where `o > e + z`.
    let e = T::from_f64(band.escalate()) + T::from_f64(band.zero()) * two;
    let exact = Band::new(f64::from_bits(1), f64::from_bits(2));
    if exact.is_ok_and(|exact| {
        matches!(
            decide(row::RUN, Margin::of(s2 - e), exact),
            Ok(Sign::Negative | Sign::Zero)
        )
    }) {
        return Vec::new();
    }
    let s = (s2 + e) / two;
    let mut out = Vec::with_capacity(4);
    let dta = s / da.norm();
    for ta in [c.ta - dta, c.ta + dta] {
        let point = a.carrier().eval(ta);
        let tb = b.carrier().foot(point, c.tb).t();
        out.push(Candidate { point, ta, tb });
    }
    let dtb = s / db.norm();
    for tb in [c.tb - dtb, c.tb + dtb] {
        let point = b.carrier().eval(tb);
        let ta = a.carrier().foot(point, c.ta).t();
        out.push(Candidate { point, ta, tb });
    }
    out
}

/// How two carriers read as one curve run, from the cosine between
/// their directions or oriented normals levered by `lever`: definitely
/// signed wherever they are one curve.
///
/// # Errors
///
/// The band's escalation, and [`MeetFault::Unsupported`] where the cosine
/// reads zero: carriers the pose read as one curve whose runs read square
/// to each other, which only a degenerate frame gives.
fn sense<T: Decide>(cosine: T, lever: T, band: Band) -> Result<Sense, MeetFault> {
    match decide(row::SENSE, Margin::levered(cosine, lever), band)? {
        Sign::Positive => Ok(Sense::Same),
        Sign::Negative => Ok(Sense::Opposite),
        Sign::Zero => Err(MeetFault::Unsupported {
            pose: "one curve whose two runs read square to each other",
        }),
    }
}

/// Whether `margin` is definitely nonzero (`row`).
fn definite<T: Decide>(
    row: &'static str,
    margin: Margin<T>,
    band: Band,
) -> Result<bool, MeetFault> {
    Ok(match decide(row, margin, band)? {
        Sign::Zero => false,
        Sign::Positive | Sign::Negative => true,
    })
}

/// Whether a meter's margin reads definitely `want`; any other reading,
/// an escalation included, refuses [`Unsettled::Meter`].
fn metered<T: Decide>(
    row: &'static str,
    margin: Margin<T>,
    want: Sign,
    band: Band,
) -> Result<(), MeetFault> {
    match decide(row, margin, band) {
        Ok(sign) if sign == want => Ok(()),
        Ok(_) | Err(_) => Err(MeetFault::Unsettled(Unsettled::Meter)),
    }
}

/// The witnesses of a coincident pair ([`Span::witness_params`] on each
/// span), each placed on the carrier of the span it came from and given
/// its foot on the other by `foot_a`/`foot_b`.
fn witnesses<T: Decide>(
    a: &Span<'_, T>,
    b: &Span<'_, T>,
    foot_a: &impl Fn(Point3<T>) -> T,
    foot_b: &impl Fn(Point3<T>) -> T,
) -> Vec<Candidate<T>> {
    let ends = |s: &Span<'_, T>, foot: &dyn Fn(Point3<T>) -> T| {
        let (t0, t1) = s.params();
        [foot(s.carrier().eval(t0)), foot(s.carrier().eval(t1))]
    };
    let mut out = Vec::new();
    for ta in a.witness_params(ends(b, foot_a)) {
        let point = a.carrier().eval(ta);
        out.push(Candidate {
            point,
            ta,
            tb: foot_b(point),
        });
    }
    for tb in b.witness_params(ends(a, foot_b)) {
        let point = b.carrier().eval(tb);
        out.push(Candidate {
            point,
            ta: foot_a(point),
            tb,
        });
    }
    out
}

/// A carrier the polish reads: a line, or a conic's frame.
#[derive(Clone, Copy)]
enum Side<'k, T: geom_core::Real> {
    Line((Point3<T>, Vec3<T>)),
    Conic(&'k Conic<T>),
}

impl<T: Decide> Side<'_, T> {
    /// The point and tangent at `t`.
    fn ders(self, t: T) -> (Point3<T>, Vec3<T>) {
        match self {
            Self::Line((origin, dir)) => (origin + dir * t, dir),
            Self::Conic(k) => (k.point(t), tangent(k, t)),
        }
    }

    /// A bound on the curvature within `reach` metres of `t`: zero on a
    /// line; on a conic `ab/|C′|³` at its least speed over the angles
    /// `reach` at the least speed covers, `|C′|² = b² + (a² − b²)·sin²θ`.
    fn curvature_near(self, t: T, reach: T) -> T {
        let Self::Conic(k) = self else {
            return T::zero();
        };
        let (a, b) = (k.semi_u.abs(), k.semi_v.abs());
        let run = reach / k.speed_lo();
        let half = T::pi() / T::from_f64(2.0);
        // How near the arc comes to where `|C′|` is least: `θ ≡ 0 (mod π)`
        // where `a ≥ b`, `θ ≡ π/2` otherwise.
        let slow = (a - b).select_le_zero(t - half, t);
        let off = (slow.reduce_periodic_centred(T::pi()).abs() - run).max(T::zero());
        let near = off.min(half).sin().powi(2);
        let (long, short) = (a.max(b), a.min(b));
        let least = (short.powi(2) + (long.powi(2) - short.powi(2)) * near).sqrt();
        a * b / least.powi(3)
    }
}

/// A crossing or touch of the other's plane, polished to the carriers'
/// closest pair (module docs, "Across planes").
struct Polish<'k, T: geom_core::Real> {
    /// The first carrier.
    a: Side<'k, T>,
    /// The second.
    b: Side<'k, T>,
    /// Where the polish starts: the crossing on the first, its foot on
    /// the second.
    start: (T, T),
    /// `ℓ`: the metres along the first carrier, from `start`, where it is
    /// within the band's escalation of the second's plane.
    reach: T,
    /// A lower bound on the start point's distance from the second
    /// carrier ([`conic_clearance`]).
    clear: T,
    /// What the basin's sine is levered by.
    lever: T,
}

impl<T: Decide> Polish<'_, T> {
    /// The closest pair, metered, or `None` where no point of the arc is
    /// within the band of the second carrier (module docs).
    fn closest(&self, band: Band) -> Result<Option<Candidate<T>>, MeetFault> {
        let (s0, t0) = self.start;
        let ((p0, da), (q0, db)) = (self.a.ders(s0), self.b.ders(t0));
        if decide(row::CLEAR, Margin::of(self.clear - self.reach), band) == Ok(Sign::Positive) {
            return Ok(None);
        }
        let sine = unit(da).cross(unit(db)).norm();
        let e = T::from_f64(band.escalate());
        let region = (self.reach + e) * T::from_f64(2.0);
        let curvature = self.a.curvature_near(s0, region) + self.b.curvature_near(t0, region);
        let turn = T::from_f64(2.0) * curvature * region;
        metered(
            row::BASIN,
            Margin::levered(sine - turn, self.lever),
            Sign::Positive,
            band,
        )?;
        let (s, t, moved) =
            super::polish::pair(|s| self.a.ders(s), |t| self.b.ders(t), self.start, |_| true);
        let ((p, _), (q, _)) = (self.a.ders(s), self.b.ders(t));
        // Within its basin the polish contracts toward a pair inside the
        // region; one that left it twice over had none to contract to.
        let travel = (p - p0).norm() + (q - q0).norm();
        if decide(row::CLEAR, Margin::of(travel - region - region), band) == Ok(Sign::Positive) {
            return Ok(None);
        }
        match decide(row::POLISH, Margin::of(moved), band) {
            Ok(Sign::Zero | Sign::Negative) => {}
            Ok(Sign::Positive) | Err(_) => return Err(MeetFault::Unsettled(Unsettled::Meter)),
        }
        Ok(Some(Candidate {
            point: p,
            ta: s,
            tb: t,
        }))
    }
}

/// Two lines (module docs).
fn line_line<T: Decide>(
    a: &Span<'_, T>,
    b: &Span<'_, T>,
    band: Band,
) -> Result<Pose<T>, MeetFault> {
    let (la, lb) = (line_of(a.carrier())?, line_of(b.carrier())?);
    let ((o1, d1), (o2, d2)) = (la, lb);
    let (u1, u2) = (unit(d1), unit(d2));
    let lever = reach(a).max(reach(b));
    let n = u1.cross(u2);
    let s = n.norm();
    if !definite(row::TILT, Margin::levered(s, lever), band)? {
        let w = middle(b) - o1;
        let off = w - u1 * w.dot(u1);
        if definite(row::OFFSET, Margin::norm3(off), band)? {
            return Ok(Pose::Parallel);
        }
        return Ok(Pose::Coincident {
            sense: sense(u1.dot(u2), lever, band)?,
            witnesses: witnesses(a, b, &|q| line_foot(la, q), &|q| line_foot(lb, q)),
        });
    }
    let along = (o2 - o1).cross(u2).dot(n) / s.powi(2);
    let point = o1 + u1 * along;
    Ok(Pose::Generic(vec![Candidate {
        point,
        ta: line_foot(la, point),
        tb: line_foot(lb, point),
    }]))
}

/// A line `a` and a conic `b` (module docs).
fn line_conic<T: Decide>(
    a: &Span<'_, T>,
    b: &Span<'_, T>,
    band: Band,
) -> Result<Pose<T>, MeetFault> {
    let line = line_of(a.carrier())?;
    let k = conic_of(b.carrier())?;
    let (o, d) = line;
    let u = unit(d);
    let n = k.axis;
    let den = u.dot(n);
    let lever = reach(a).max(k.speed_hi());
    let origin = Point3::origin();
    let noise = crate::rounding_charge(
        (o - origin).norm() + (k.center - origin).norm() + lever + k.speed_hi(),
    );
    if definite(row::TILT, Margin::levered(den, lever), band)? {
        // The line's height above the plane runs `|d̂·n̂|` per metre, so
        // it is within the band's escalation `e/|d̂·n̂|` either side of
        // the crossing.
        let point = o + u * ((k.center - o).dot(n) / den);
        let e = T::from_f64(band.escalate()) + noise;
        let polish = Polish {
            a: Side::Line(line),
            b: Side::Conic(&k),
            start: (line_foot(line, point), conic_foot(&k, point)),
            reach: e / den.abs(),
            clear: conic_clearance(&k, point),
            lever,
        };
        return Ok(Pose::Generic(polish.closest(band)?.into_iter().collect()));
    }
    if definite(row::OFFSET, Margin::of((middle(a) - k.center).dot(n)), band)? {
        return Ok(Pose::Parallel);
    }
    // In the plane: the conic's signed distance from the line, along the
    // line's normal in the plane, a first harmonic in the conic's angle.
    let m = unit(n.cross(u));
    let c0 = (k.center - o).dot(m);
    let (cp, sp) = (k.semi_u * k.u_ref.dot(m), k.semi_v * k.v_ref().dot(m));
    let swing = (cp.powi(2) + sp.powi(2)).sqrt();
    if !definite(row::SWING, Margin::of(swing), band)? {
        return Err(MeetFault::Unsupported {
            pose: "a conic the band reads as thin as the line it lies beside",
        });
    }
    let range = Range::first_harmonic(c0, swing, sp.atan2(cp), noise);
    let place = |theta: T| {
        let point = k.point(theta);
        Candidate {
            point,
            ta: line_foot(line, point),
            tb: theta,
        }
    };
    Ok(match harmonic_roots(&k, &range, band)? {
        Found::Apart => Pose::Apart,
        Found::Touch(theta) => Pose::Tangent(place(theta)),
        Found::Cross(thetas) => Pose::Generic(thetas.into_iter().map(place).collect()),
    })
}

/// What two conics in one plane are, `a` the one parametrized: the arm's
/// kinds, fixed by the table.
#[derive(Clone, Copy)]
enum Coplanar {
    /// Two circles.
    Circles,
    /// An ellipse, then a circle.
    EllipseCircle,
    /// Two ellipses.
    Ellipses,
}

/// The conic `k`'s height above the plane through `on` with unit normal
/// `n`: its first harmonic's constant, cosine and sine parts, in metres.
fn height<T: Decide>(k: &Conic<T>, on: Point3<T>, n: Vec3<T>) -> (T, T, T) {
    (
        (k.center - on).dot(n),
        k.semi_u * k.u_ref.dot(n),
        k.semi_v * k.v_ref().dot(n),
    )
}

/// Two conics, `a` the one parametrized in one plane (module docs).
fn conic_conic<T: Decide>(
    a: &Span<'_, T>,
    b: &Span<'_, T>,
    coplanar: Coplanar,
    band: Band,
) -> Result<Pose<T>, MeetFault> {
    let (ka, kb) = (conic_of(a.carrier())?, conic_of(b.carrier())?);
    let lever = ka.speed_hi().max(kb.speed_hi());
    let origin = Point3::origin();
    let noise = crate::rounding_charge(
        (ka.center - origin).norm() + (kb.center - origin).norm() + lever + lever,
    );
    let ha = height(&ka, kb.center, kb.axis);
    let hb = height(&kb, ka.center, ka.axis);
    let swing = |(_, cp, sp): (T, T, T)| (cp.powi(2) + sp.powi(2)).sqrt();
    let (swing_a, swing_b) = (swing(ha), swing(hb));
    // A conic whose height above the other's plane is constant within
    // the band lies in a plane parallel to it: definitely off it, or in
    // it to within the slant `|c₀|` plus its swing.
    let mut slant = None;
    for (swing, (c0, _, _)) in [(swing_a, ha), (swing_b, hb)] {
        if definite(row::SWING, Margin::of(swing), band)? {
            continue;
        }
        if constant(row::OFFSET, c0, swing + noise, band)? {
            return Ok(Pose::Parallel);
        }
        slant = Some(c0.abs() + swing);
        break;
    }
    let Some(slant) = slant else {
        return transverse(&ka, &kb, ha, swing_a, noise, lever, band);
    };
    let oriented = |k: &Conic<T>| k.axis * (k.semi_u * k.semi_v);
    if one_curve(&ka, &kb, band)? {
        let (foot_a, foot_b) = (|q| conic_foot(&ka, q), |q| conic_foot(&kb, q));
        return Ok(Pose::Coincident {
            sense: sense(unit(oriented(&ka)).dot(unit(oriented(&kb))), lever, band)?,
            witnesses: witnesses(a, b, &foot_a, &foot_b),
        });
    }
    let place = |theta: T| {
        let point = ka.point(theta);
        Candidate {
            point,
            ta: theta,
            tb: conic_foot(&kb, point),
        }
    };
    match coplanar {
        Coplanar::Circles => coplanar_circles(&ka, &kb, (noise, slant), &place, band),
        Coplanar::EllipseCircle => coplanar_ellipse_circle(a, &ka, &kb, &place, band),
        Coplanar::Ellipses => Err(MeetFault::Unsupported {
            pose: "two ellipses in one plane that are not one curve",
        }),
    }
}

/// **Whether two conics read in one plane are one curve** (module docs):
/// [`one_curve_gap`] read zero.
///
/// # Errors
///
/// The band's escalation.
pub(super) fn one_curve<T: Decide>(
    e: &Conic<T>,
    f: &Conic<T>,
    band: Band,
) -> Result<bool, MeetFault> {
    Ok(!definite(
        row::ONE_CURVE,
        Margin::of(one_curve_gap(e, f)),
        band,
    )?)
}

/// **The one-curve reading's margin** (module docs): a bound on how far
/// a point of either conic lies from the other, a tilt out of the plane
/// included. Each conic is `c + L·x` over the unit circle's `x`, with
/// `L = [a·û, b·v̂]`. For any orthogonal `Q`, pairing `c_e + L_e·x` with
/// `c_f + L_f·Q·x` pairs every point of each with a point of the other
/// within `|c_e − c_f| + ‖L_e − L_f·Q‖` ([`frame_gap`]). The stored order
/// and signs of the semi-axes change `L` by an orthogonal factor, which
/// `Q` absorbs. The gap read from either side is the same but for
/// rounding, and the larger is taken, so the gap does not depend on the
/// order of the pair.
pub(super) fn one_curve_gap<T: Decide>(e: &Conic<T>, f: &Conic<T>) -> T {
    (e.center - f.center).norm() + frame_gap(e, f).max(frame_gap(f, e))
}

/// `‖L_e − L_f·Q‖`, the operator norm, for the `Q` that turns `f`'s
/// frame the way up `e`'s is (the sign of `(a·b·n̂)_e · (a·b·n̂)_f`) and
/// then by the rotation that best matches the two, which maximizes
/// `tr(Qᵀ·L_fᵀ·L_e)`.
fn frame_gap<T: Decide>(e: &Conic<T>, f: &Conic<T>) -> T {
    let ((e1, e2), (f1, f2)) = (e.semi_axis_vectors(), f.semi_axis_vectors());
    let up = e1.cross(e2).dot(f1.cross(f2));
    let f2 = f2 * (up / up.abs());
    let (x, y) = (f1.dot(e1) + f2.dot(e2), f2.dot(e1) - f1.dot(e2));
    let h = (x.powi(2) + y.powi(2)).sqrt();
    let (cos, sin) = (x / h, y / h);
    let d1 = e1 - (f1 * cos + f2 * sin);
    let d2 = e2 - (f2 * cos - f1 * sin);
    let half = T::from_f64(0.5);
    let (mid, off) = (
        (d1.dot(d1) + d2.dot(d2)) * half,
        (d1.dot(d1) - d2.dot(d2)) * half,
    );
    (mid + (off.powi(2) + d1.dot(d2).powi(2)).sqrt()).sqrt()
}

/// `ka`'s height above `kb`'s plane, `(c0, cp, sp)` with the swing read
/// definite (module docs): its crossings and touch, each polished to the
/// closest pair.
fn transverse<T: Decide>(
    ka: &Conic<T>,
    kb: &Conic<T>,
    (c0, cp, sp): (T, T, T),
    swing: T,
    noise: T,
    lever: T,
    band: Band,
) -> Result<Pose<T>, MeetFault> {
    let range = Range::first_harmonic(c0, swing, sp.atan2(cp), noise);
    let near = |theta: T| {
        let q = ka.point(theta);
        Polish {
            a: Side::Conic(ka),
            b: Side::Conic(kb),
            start: (theta, conic_foot(kb, q)),
            reach: ka.speed_hi() * range.band_arc(theta, band),
            clear: conic_clearance(kb, q),
            lever,
        }
        .closest(band)
    };
    Ok(match harmonic_roots(ka, &range, band)? {
        Found::Apart => Pose::Apart,
        Found::Touch(theta) => match near(theta)? {
            Some(c) => Pose::Tangent(c),
            None => Pose::Apart,
        },
        Found::Cross(thetas) => {
            let mut out = Vec::with_capacity(2);
            for theta in thetas {
                out.extend(near(theta)?);
            }
            Pose::Generic(out)
        }
    })
}

/// Whether a quantity known to within `spread` about `value` is
/// definitely clear of zero (`true`) or in the zero band all round
/// (`false`), read under `row`.
///
/// # Errors
///
/// An end in the band's gap, or ends that straddle the band.
fn constant<T: Decide>(
    row: &'static str,
    value: T,
    spread: T,
    band: Band,
) -> Result<bool, MeetFault> {
    let below = decide_reported(row, Margin::of(value - spread), band)?;
    let above = decide_reported(row, Margin::of(value + spread), band)?;
    match (below.sign, above.sign) {
        (Sign::Positive, _) | (_, Sign::Negative) => Ok(true),
        (Sign::Zero, Sign::Zero) => Ok(false),
        _ => Err(MeetFault::Escalated(straddle(row, below, above, band))),
    }
}

/// Two circles in one plane (module docs): `noise` bounds their
/// rounding, and `slant` how far they lie from one plane.
fn coplanar_circles<T: Decide>(
    ka: &Conic<T>,
    kb: &Conic<T>,
    (noise, slant): (T, T),
    place: &impl Fn(T) -> Candidate<T>,
    band: Band,
) -> Result<Pose<T>, MeetFault> {
    let (ra, rb) = (ka.semi_u.abs(), kb.semi_u.abs());
    let w = ka.center - kb.center;
    let w = w - ka.axis * w.dot(ka.axis);
    let d = w.norm();
    if !definite(row::CENTRES, Margin::norm3(w), band)? {
        // The one-curve gap is at most `d + |r₁ − r₂|` plus the slant, so
        // radii whose gap reads zero all round with the centres' are one
        // curve, which the one-curve reading has just denied.
        return if constant(row::RADII, ra - rb, d + noise + slant, band)? {
            Ok(Pose::Concentric)
        } else {
            Err(MeetFault::Contradiction {
                readings: "two circles read not one curve, on one centre, with radii one",
            })
        };
    }
    // `|C₁(θ) − c₂|` runs between `|r₁ − d|` and `r₁ + d`; its square
    // less `r₂²` is `c₀ + 2r₁d·cos(θ − θ_hi)`, a first harmonic whose
    // swing is `4r₁d`. `θ_hi` is the angle of `w`, half a turn on where
    // the stored radius is negative.
    let (near, far) = ((ra - d).abs(), ra + d);
    let (lo, hi) = (near - rb, far - rb);
    let swing = T::from_f64(4.0) * ra * d;
    let toward = w.dot(ka.v_ref()).atan2(w.dot(ka.u_ref));
    let range = Range {
        lo,
        hi,
        at_hi: ka.semi_u.select_le_zero(toward + T::pi(), toward),
        cross_lo: (T::zero() - lo) * (near + rb) / swing,
        cross_hi: hi * (far + rb) / swing,
        noise,
    };
    Ok(match harmonic_roots(ka, &range, band)? {
        Found::Apart => Pose::Apart,
        Found::Touch(theta) => Pose::Tangent(place(theta)),
        Found::Cross(thetas) => Pose::Generic(thetas.into_iter().map(place).collect()),
    })
}

/// An ellipse `ke` (the span `e`) and a circle `kc` in one plane, not one
/// curve (module docs): the ellipse's distance from the circle, through
/// the certified subdivision in the ellipse's angle.
fn coplanar_ellipse_circle<T: Decide>(
    e: &Span<'_, T>,
    ke: &Conic<T>,
    kc: &Conic<T>,
    place: &impl Fn(T) -> Candidate<T>,
    band: Band,
) -> Result<Pose<T>, MeetFault> {
    let r = kc.semi_u.abs();
    let w = ke.center - kc.center;
    let (a, b) = (ke.semi_u, ke.semi_v);
    let two = T::from_f64(2.0);
    // `|E(θ) − c|² − r²` in the ellipse's angle.
    let poly = DegreeTwo::new(
        w.norm_squared() + (a.powi(2) + b.powi(2)) / two - r.powi(2),
        two * a * w.dot(ke.u_ref),
        two * b * w.dot(ke.v_ref()),
        (a.powi(2) - b.powi(2)) / two,
        T::zero(),
    )
    .poly();
    let terms = (w.norm() + a.abs() + b.abs()).powi(2) + r.powi(2);
    let (t0, t1) = e.params();
    // `F = (ρ − r)(ρ + r)` in the distance `ρ` from the centre: `|F|` per
    // metre of `ρ − r` is `ρ + r`, at least `r`, and at most `3r` within
    // `r` of the circle.
    let frame = SubdivisionFrame {
        t0,
        t1,
        speed_hi: ke.speed_hi(),
        noise: crate::rounding_charge(terms),
        f_per_metre: r,
        f_per_metre_hi: T::from_f64(3.0) * r,
        residual_reach: Some(r),
    };
    let residual = |theta: T| (ke.point(theta) - kc.center).norm() - r;
    match certified_subdivision(&poly, &residual, &frame, &COPLANAR_ROWS, None, band) {
        Ok(Roots::Miss) => Ok(Pose::Apart),
        Ok(Roots::Certified(found)) => Ok(Pose::Generic(
            found.roots().iter().map(|&t| place(t)).collect(),
        )),
        Err(why) => Err(MeetFault::Unsettled(why)),
    }
}

/// A first harmonic's range along a conic, in metres, and where its roots
/// are read from (module docs): `lo` and `hi` are its extremes, `at_hi`
/// the angle of `hi` (`lo` is half a turn on), `cross_lo` and `cross_hi`
/// how far each extreme has crossed zero over the swing, in the units the
/// roots solve in (positive where it has crossed), and `noise` a bound on
/// each extreme's rounding.
struct Range<T> {
    lo: T,
    hi: T,
    at_hi: T,
    cross_lo: T,
    cross_hi: T,
    noise: T,
}

impl<T: Decide> Range<T> {
    /// `c₀ + A·cos(θ − at_hi)`, `A ≥ 0`, solved in itself.
    fn first_harmonic(c0: T, swing: T, at_hi: T, noise: T) -> Self {
        let (lo, hi) = (c0 - swing, c0 + swing);
        let two = swing + swing;
        Self {
            lo,
            hi,
            at_hi,
            cross_lo: (T::zero() - lo) / two,
            cross_hi: hi / two,
            noise,
        }
    }

    /// `ℓ` in radians (module docs, "Across planes"): how far from `theta`
    /// the arc about it runs on which the harmonic, a metre residual,
    /// stays within the band's escalation and its own rounding.
    fn band_arc(&self, theta: T, band: Band) -> T {
        let two = T::from_f64(2.0);
        let (c0, swing) = ((self.lo + self.hi) / two, (self.hi - self.lo) / two);
        let e = T::from_f64(band.escalate()) + self.noise;
        let unit_cos = |x: T| x.max(T::zero() - T::one()).min(T::one());
        let phi = (theta - self.at_hi).reduce_periodic_centred(T::tau()).abs();
        let inner = unit_cos((e - c0) / swing).acos();
        let outer = unit_cos((T::zero() - e - c0) / swing).acos();
        (phi - inner).abs().max((outer - phi).abs())
    }
}

/// What a first harmonic's extremes and root gap decide.
enum Found<T> {
    /// No root: the extremes definitely one-signed.
    Apart,
    /// A touch at this angle: a double root, or an extreme in the zero
    /// band that has not crossed.
    Touch(T),
    /// Two crossings.
    Cross([T; 2]),
}

/// A first harmonic's roots along `k` (module docs).
fn harmonic_roots<T: Decide>(
    k: &Conic<T>,
    r: &Range<T>,
    band: Band,
) -> Result<Found<T>, MeetFault> {
    let lo = extreme_sign(row::EXTREME, r.lo, r.noise, band)?;
    let hi = extreme_sign(row::EXTREME, r.hi, r.noise, band)?;
    // The angle from `hi` of the roots read from each extreme: none
    // crossed reads a double root at that extreme.
    let past = |cross: T| arc_past(cross.max(T::zero()).min(T::one()));
    let from_hi = past(r.cross_hi);
    let from_lo = T::pi() - past(r.cross_lo);
    let at_lo = r.at_hi + T::pi();
    // An extreme in the zero band has crossed by no more than its own
    // rounding can account for: its roots are read for the gap at the
    // least crossing that rounding allows, its value less its noise, so
    // two roots the noise alone puts apart read as the touch they are.
    let least = |cross: T, extreme: T| {
        let size = extreme.abs();
        cross * ((size - r.noise).max(T::zero()) / size.max(r.noise))
    };
    let (psi, gap_psi, at_near) = match (lo, hi) {
        (Sign::Positive, _) | (_, Sign::Negative) => return Ok(Found::Apart),
        (Sign::Zero, Sign::Zero) => return Err(MeetFault::Unsettled(Unsettled::Tangency)),
        (Sign::Zero, Sign::Positive) => (from_lo, T::pi() - past(least(r.cross_lo, r.lo)), at_lo),
        (Sign::Negative, Sign::Zero) => (from_hi, past(least(r.cross_hi, r.hi)), r.at_hi),
        // Two definite crossings: read from the extreme that has crossed
        // the less, to its own relative precision.
        (Sign::Negative, Sign::Positive) => {
            let nearer_hi = r.cross_hi - r.cross_lo;
            let psi = nearer_hi.select_le_zero(from_hi, from_lo);
            (psi, psi, nearer_hi.select_le_zero(r.at_hi, at_lo))
        }
    };
    let roots = [r.at_hi + psi, r.at_hi - psi];
    let gap = Margin::norm3(k.point(r.at_hi + gap_psi) - k.point(r.at_hi - gap_psi));
    Ok(match decide(row::ROOT_GAP, gap, band)? {
        Sign::Zero => Found::Touch(at_near),
        Sign::Positive | Sign::Negative => Found::Cross(roots),
    })
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod polish_rows {
    use super::*;

    /// **A polish that has not converged refuses.** Two unit circles
    /// crossing square at the origin, the polish started there and with
    /// the first `0.3` rad round: the first is the crossing; the second,
    /// inside its basin and its region, is still moving after its steps
    /// and refuses (`Unsettled::Meter`).
    #[test]
    fn an_unconverged_polish_refuses() {
        let band = Band::new(1e-9, 1e-8).unwrap();
        let (x, z) = (Vec3::new(1.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 1.0));
        let a = Conic::circle(Point3::new(0.0, -1.0, 0.0), z, 1.0, x);
        let b = Conic::circle(Point3::new(-1.0, 0.0, 0.0), z, 1.0, x);
        let polish = |off: f64| Polish {
            a: Side::Conic(&a),
            b: Side::Conic(&b),
            start: (core::f64::consts::FRAC_PI_2 + off, 0.0),
            reach: 0.11,
            clear: 0.0,
            lever: 1.0,
        };
        let got = polish(0.0).closest(band).unwrap().unwrap();
        assert!(
            got.point.to_array().iter().all(|c| c.abs() < 1e-15),
            "{got:?}"
        );
        let got = polish(0.3).closest(band);
        assert!(
            matches!(got, Err(MeetFault::Unsettled(Unsettled::Meter))),
            "{got:?}"
        );
    }
}
