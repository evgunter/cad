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
//! - **The router** (two lines, and a line against a conic's plane):
//!   where the pair can meet is the overlap of the two spans'
//!   projections on the first line (for a conic, the line's span with
//!   the conic's centre ± its larger semi-axis), length `L`, and the
//!   escalation `e` either side. The pair's offset there is the second
//!   line's distance from the overlap's middle point on the first, or the
//!   line's height above the conic's plane at that point. **The parallel
//!   argument runs first, whatever the tilt reads**: where the offset less
//!   its most variation over the overlap and `e` either side,
//!   `rate·(L/2 + e)`, decides positive (`curve_pair_pose_parallel`), the
//!   pair is [`Pose::Parallel`], `rate` being `s/c` for two lines (`s`
//!   the sine between them, `c = |û₁·û₂|`) and `|d̂·n̂|` for a line and a
//!   plane. **Why that proves no meeting anywhere on either span:** a
//!   meeting is two points, one per span, within the band's zero `z` of
//!   each other, so their projections on the first line lie within `z`
//!   of each other, each inside its own span's projection (for a conic,
//!   inside its extent's): the meeting projects within `z < e` of the
//!   overlap, even where the overlap is empty (the projections then part
//!   by less than `z`) or shorter than the band. Over that interval the
//!   offset moves at most `rate` per metre from its value at the
//!   overlap's middle, so it stays past the band's escalation and no
//!   point of one lies within `z` of the other. The parts of either span
//!   outside that interval lie farther than `z` from the other by their
//!   projections alone. **Split invariance follows by construction**: a
//!   piece of a span projects inside the whole's projection, so a
//!   piece's overlap is a sub-interval of the whole's, and its offset at
//!   its own middle less its own variation is at least the whole's
//!   margin; a pair read Parallel whole is read Parallel in every split.
//!   **The gap between the projections reads next**
//!   (`curve_pair_pose_gap`), before the tilt or the offset is read: by
//!   the same projection argument, projections apart by a gap that
//!   decides positive hold no meeting, whatever the lines' distance
//!   reads, and the pair is [`Pose::Apart`]. A piece's gap is at least
//!   the whole's, so a pair read Apart whole is read Parallel or Apart in
//!   every split. A gap read zero or in the escalation gap says nothing,
//!   and the reading goes on. Both screens' margins are charged with the
//!   rounding the projections and the offset carry, sized by the
//!   carriers' origins and the spans' ends as evaluated, so a coordinate
//!   large by its parameter is charged as one large by its origin.
//!   Only where neither decides does the tilt route: levered
//!   by the longer span (`curve_pair_pose_tilt`), definite is a crossing;
//!   read zero over the overlap `s·(L + 2e)`, or zero over the longer
//!   span, the pair is level where it can meet, and the offset at the
//!   overlap's middle (`curve_pair_pose_offset`) decides: zero is level,
//!   and otherwise the parallel margin having not decided refuses
//!   [`Unsettled::Meter`]. A tilt over the longer span in its band
//!   escalates.
//! - **Two lines.** Crossing, the candidate is the first line's point
//!   nearest the second. Level, the lines are [`Pose::Coincident`] only
//!   where each span's two ends lie within the zero band of the other's
//!   carrier (`curve_pair_pose_offset`): a line's distance from another is
//!   affine along it, so its ends bound it over the whole span, and
//!   `Coincident` claims one curve over both spans, not only where they
//!   can meet. Two lines level at a shallow corner, their overlap one
//!   point and an end apart past the band, are read as the crossing the
//!   longer span's tilt decides, and refuse [`Unsettled::Meter`] where it
//!   reads zero.
//! - **A line and a conic.** Crossing, the line crosses the plane once,
//!   and the candidate is the closest pair polished from there (below).
//!   Level, the line lies in the plane where it can meet, and there the
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
//!   gap plus `‖L₁ − L₂Q‖` for the best rotation `Q`, its rounding
//!   charged, bounds how far a point of either lies from the other, a
//!   tilt out of the plane included. Read zero is [`Pose::Coincident`]. Past it:
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
//!   - **two ellipses**: the first's level on the second, of degree two
//!     in the first's angle, goes to the certified subdivision in the
//!     metres of the lower bound on a point's distance from an ellipse
//!     ([`geom::ellipse_level_clearance`], [`Curve3::clearance`]'s), with
//!     a ceiling on the level per metre of it (`coplanar_ellipses`).
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
//! # A touch in the subdivision
//!
//! **The coplanar subdivisions place a touch as the circle pair does**
//! ([`crate::half_angle::certified_subdivision_touching`]). Two conics
//! that touch stay within the band of each other over an arc much wider
//! than the band, so no cut there reads a definite sign. Before that
//! arc is cut, the piece holding it reads convex toward its ends' common
//! side (`curve_pair_coplanar_sub_convex`): `F″` one-signed, so `F` has
//! one extremum there and at most two roots, both about it. The
//! extremum is the candidate. Read on the carrier in the zero band, the
//! arc that holds every root of the piece, at the least depth `F`'s
//! rounding allows, decides (`curve_pair_coplanar_sub_touch`): zero is
//! one touch standing for them all, as the circle pair's root gap read
//! zero is; in the gap it escalates; roots the band can part are left to
//! the walk. The touch is the extremum's own point, on the first
//! carrier exactly, and is read on both spans like any candidate.
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
//! `2(ℓ + e)` of where it starts, on either carrier. Three meters guard it,
//! each refusing [`Unsettled::Meter`] unless definite:
//!
//! - **its basin** (`curve_pair_pose_basin`): over that reach each
//!   tangent turns by at most its carrier's curvature times the reach, so
//!   `sin φ − 2(κ₁ + κ₂)·2(ℓ + e)`, `φ` the angle between the tangents
//!   where the polish starts, levered as the tilt is, must read positive:
//!   the tangents then keep at least half the angle between them,
//!   the closest pair is the one stationary pair there, and the polish
//!   contracts toward it;
//! - **its travel** (`curve_pair_pose_travel`): the polished pair lies
//!   within twice the region of where it started, as the basin says;
//! - **its convergence** (`curve_pair_pose_polish`): the polish's last
//!   step moves the pair within the band's zero.
//!
//! # The spans' ends, and deflation
//!
//! A touch or a crossing is one point, but two curves near it stay
//! within the band over a stretch about it, long where they cross at a
//! shallow angle or touch with little relative curvature. Where that
//! point lies past a span and the stretch reaches into both, the spans
//! meet in the band at an end of one of them, which no isolated
//! candidate reads. So each span's ends are read against the other
//! first ([`end_candidates`]): a span end within the band of the other
//! span is a meeting, and one in the band's gap escalates.
//!
//! **A meeting so found is a root the pose already holds where the
//! arm's equation is the meeting's own**, and there the arm divides it
//! out rather than solving for it again: re-derived near a tangent, a
//! root comes from a square root whose rounding is about `√ulp` of the
//! radius, and its gap from its twin escalates where the found meeting
//! decides it exactly. The arms whose equation is the meeting's are the
//! in-plane ones: a circle's or an
//! ellipse's signed distance from a line in its plane, and a circle's
//! distance from a circle in its plane. Their first harmonics,
//! `c₀ + A·cos(θ − θ_hi)`, have their two roots symmetric about `θ_hi`,
//! so the other root is `2θ_hi − θ_J`, linear in the found one's angle
//! `θ_J`: the line's second meeting with a conic in its plane (Vieta),
//! and the reflection of a meeting of two circles in one plane across
//! their line of centres. Two found meetings leave a first harmonic
//! none. Two lines' crossing is solved linearly, with no square root to
//! condition, so it lands inside a found joint's ring and is not read
//! again: that form divides nothing out.
//!
//! **A crossing of the other's plane is not a meeting**, so the arms
//! across a plane divide nothing out. A conic's or a line's root on the
//! other's plane marks a window, about `e/|d̂·n̂|` of arc either side,
//! where it lies within the band's escalation of that plane, and that
//! window can hold more than one meeting, each in a stretch of its own.
//! A conic's plane root whose window holds a found meeting is read
//! instead (`window`): where the window certainly holds no meeting but
//! the found ones, the root is not polished. Every other plane root is
//! polished and metered as it would be with no meeting found; a candidate
//! it gives nearer a found meeting than the escalation is that meeting
//! again (C5's ring rule), and a meter that cannot place one refuses. The coplanar subdivisions divide nothing
//! out: their roots nearer a found meeting than the escalation are not
//! read again.
//!
//! # The band
//!
//! A decision the reading rests on escalates in the band's gap
//! ([`MeetFault::Escalated`]). A decision read as a screen, for the one
//! verdict that lets the reading stop or take a shorter road, passes any
//! other reading on, an escalation included, to the readings after it,
//! which decide the pair or escalate on their own: the router's parallel
//! margin (`curve_pair_pose_parallel`), its gap (`curve_pair_pose_gap`)
//! and its tilt over the overlap (`curve_pair_pose_tilt`), the polish's
//! clearance (`curve_pair_pose_clear`), and the like in the window's
//! readings and the branch compares. A meter's escalation refuses
//! [`Unsettled::Meter`]. No escalation prunes a candidate.

mod window;

use geom::{Curve3, CurveKind};
use geom_core::k_stats::{decide, straddle};
use geom_core::one_line::{OnOneLine, on_one_line};
use geom_core::{Band, Decide, Margin, Point3, Sign, Vec3};

use super::span::Span;
use super::{Arm, MeetFault, Method};
use crate::Conic;
use crate::half_angle::{
    DegreeTwo, SubdivisionFrame, SubdivisionRows, TouchRows, certified_subdivision_touching,
};
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
    /// Definitely apart otherwise: the spans' projections on a line
    /// apart past the band, or the residual's extremes one-signed.
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
    pub(super) const PARALLEL: &str = "curve_pair_pose_parallel";
    pub(super) const GAP: &str = "curve_pair_pose_gap";
    pub(super) const SENSE: &str = "curve_pair_pose_sense";
    pub(super) const SWING: &str = "curve_pair_pose_swing";
    pub(super) const EXTREME: &str = "curve_pair_pose_extreme";
    pub(super) const ROOT_GAP: &str = "curve_pair_pose_root_gap";
    pub(super) const CLEAR: &str = "curve_pair_pose_clear";
    pub(super) const TRAVEL: &str = "curve_pair_pose_travel";
    pub(super) const BASIN: &str = "curve_pair_pose_basin";
    pub(super) const POLISH: &str = "curve_pair_pose_polish";
    pub(super) const CENTRES: &str = "curve_pair_pose_centres";
    pub(super) const RADII: &str = "curve_pair_pose_radii";
    pub(super) const ONE_CURVE: &str = "curve_pair_pose_one_curve";
    pub(super) const BRANCH: &str = "curve_pair_pose_branch";
}

/// The coplanar circle × ellipse subdivision's rows.
const COPLANAR_ROWS: SubdivisionRows = SubdivisionRows {
    clear: "curve_pair_coplanar_sub_clear",
    monotone: "curve_pair_coplanar_sub_monotone",
    side: "curve_pair_coplanar_sub_side",
    width: "curve_pair_coplanar_sub_width",
};

/// The coplanar subdivision's touch rows (module docs, "A touch in the
/// subdivision").
const COPLANAR_TOUCH: TouchRows = TouchRows {
    convex: "curve_pair_coplanar_sub_convex",
    gap: "curve_pair_coplanar_sub_touch",
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
        Method::Closed(arm) => closed(arm, a, b, &[], band).map_err(PoseRefusal::Closed),
        Method::Walk => Err(PoseRefusal::Walked { a: ka, b: kb }),
    }
}

/// [`pose`] for a closed-form arm.
pub(super) fn closed<T: Decide>(
    arm: Arm,
    a: &Span<'_, T>,
    b: &Span<'_, T>,
    known: &[Candidate<T>],
    band: Band,
) -> Result<Pose<T>, MeetFault> {
    // Where one side is a circle and the other an ellipse, the ellipse is
    // parametrized, so the reading against the circle is at its exact
    // foot.
    let flipped: Vec<_> = known.iter().copied().map(flip).collect();
    match arm {
        Arm::LineLine => line_line(a, b, band),
        Arm::LineConic => line_conic(a, b, known, band),
        Arm::ConicLine => Ok(swapped(line_conic(b, a, &flipped, band)?)),
        Arm::CircleCircle => conic_conic(a, b, Coplanar::Circles, known, band),
        Arm::EllipseCircle => conic_conic(a, b, Coplanar::EllipseCircle, known, band),
        Arm::CircleEllipse => Ok(swapped(conic_conic(
            b,
            a,
            Coplanar::EllipseCircle,
            &flipped,
            band,
        )?)),
        Arm::EllipseEllipse => conic_conic(a, b, Coplanar::Ellipses, known, band),
    }
}

/// A candidate with its two carriers' roles swapped.
fn flip<T: geom_core::Real>(c: Candidate<T>) -> Candidate<T> {
    Candidate {
        point: c.point,
        ta: c.tb,
        tb: c.ta,
    }
}

/// A pose read with the spans' roles swapped, given back in their order.
fn swapped<T: geom_core::Real>(pose: Pose<T>) -> Pose<T> {
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

/// **The angle of `(x, y)`, read clear of its branch cut**: `atan2(y, x)`
/// where `x` does not read negative, and otherwise the same angle a
/// turn on, `atan2(−y, −x) + π`, whose cut lies half a turn from it. An
/// enclosure of a direction near the negative `x` axis then stays as
/// narrow as the direction, where the principal value would wrap the
/// whole turn. Every reader takes the angle modulo a turn, so either
/// branch serves.
fn angle<T: Decide>(y: T, x: T) -> T {
    if super::compare(row::BRANCH, x, T::zero()) == Some(Sign::Negative) {
        (T::zero() - y).atan2(T::zero() - x) + T::pi()
    } else {
        y.atan2(x)
    }
}

/// **How far round the turn `theta` lies from `at`**, in `[0, π]`, read
/// clear of the reduction's cut: from `at` where it reads within a
/// quarter turn of it, and otherwise as half a turn less how far it
/// lies from `at + π`. An enclosure about the antipode then stays as
/// narrow as `theta`, where the reduction about `at` would read the
/// whole half turn.
fn separation<T: Decide>(theta: T, at: T) -> T {
    let from = |anchor: T| (theta - anchor).reduce_periodic_centred(T::tau()).abs();
    let near = from(at);
    if super::compare(row::BRANCH, near, T::pi() / T::from_f64(2.0)) == Some(Sign::Negative) {
        near
    } else {
        T::pi() - from(at + T::pi())
    }
}

/// The metres a span runs at most: its parameter run at the carrier's
/// greatest speed.
fn reach<T: Decide>(s: &Span<'_, T>) -> T {
    let (t0, t1) = s.params();
    Margin::metered_sup((t1 - t0).abs(), s.speeds().1).value()
}

/// The line's foot of `q` ([`Curve3::foot`], the one foot reading).
fn line_foot<T: Decide>((origin, dir): (Point3<T>, Vec3<T>), q: Point3<T>) -> T {
    Curve3::Line { origin, dir }.foot(q, T::zero()).t()
}

/// A lower bound on `q`'s distance from the carrier
/// ([`Curve3::clearance`], the one bound), zero where it has none in
/// closed form.
fn clearance<T: Decide>(c: &Curve3<T>, q: Point3<T>) -> T {
    c.clearance(q).unwrap_or_else(T::zero)
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
    /// The point and tangent at `t`: the carrier's own
    /// [`Curve3::ders1`].
    fn ders(self, t: T) -> (Point3<T>, Vec3<T>) {
        match self {
            Self::Line((origin, dir)) => Curve3::Line { origin, dir }.ders1(t),
            Self::Conic(k) => Curve3::Ellipse {
                center: k.center,
                axis: k.axis,
                semi_u: k.semi_u,
                semi_v: k.semi_v,
                u_ref: k.u_ref,
            }
            .ders1(t),
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
    /// carrier ([`Curve3::clearance`]).
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
        // Within its basin the polish stays in the region; one that left
        // it twice over is a polish the basin meter did not certify.
        let travel = (p - p0).norm() + (q - q0).norm();
        metered(
            row::TRAVEL,
            Margin::of(region + region - travel),
            Sign::Positive,
            band,
        )?;
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

/// The metres `s`'s two ends lie from the origin, as evaluated.
fn ends<T: Decide>(s: &Span<'_, T>) -> T {
    let (t0, t1) = s.params();
    let from = |t: T| (s.carrier().eval(t) - Point3::origin()).norm();
    from(t0) + from(t1)
}

/// The projection of `s`'s ends on the line `(o, u)`, `u` a unit: their
/// distances along it from `o`.
fn on_line<T: Decide>((o, u): (Point3<T>, Vec3<T>), s: &Span<'_, T>) -> (T, T) {
    let (t0, t1) = s.params();
    (
        (s.carrier().eval(t0) - o).dot(u),
        (s.carrier().eval(t1) - o).dot(u),
    )
}

/// Where the router sends a line's pair (module docs, "The router").
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Route {
    /// The projections definitely apart: no place to meet.
    Apart,
    /// Definitely apart over every place they can meet.
    Parallel,
    /// Crossing: the tilt definite over the longer span.
    Crossing,
    /// Level over where they can meet, the offset there read zero.
    Level,
}

/// **The router** of two lines, or of a line and a conic's plane (module
/// docs, "The router"): `tilt` the sine of the pair's angle, `rate` the
/// most metres the offset moves per metre along the first line, `offset`
/// the second's distance (or the plane's height) at the overlap's middle,
/// `at` the overlap ([`OnOneLine`], carrying its own rounding), `lever`
/// the longer span's metres and `noise` the rounding the offset carries,
/// charged against the parallel screen's margin.
///
/// The parallel argument runs first: where the offset less its most
/// variation over the overlap and the escalation either side decides
/// positive (`curve_pair_pose_parallel`), no point meets, whatever the
/// tilt reads. Then the gap: where the projections are disjoint and
/// their gap decides positive (`curve_pair_pose_gap`), no point meets.
/// Only otherwise does the tilt route: definite over the
/// longer span, a crossing; zero over the overlap, or over the longer
/// span, level where the offset reads zero, and [`Unsettled::Meter`]
/// where it does not, the parallel margin having not decided.
///
/// # Errors
///
/// The band's escalation of the tilt over the longer span or of the
/// offset, and the Meter refusal above.
fn route<T: Decide>(
    tilt: T,
    rate: T,
    offset: T,
    at: OnOneLine<T>,
    (lever, noise): (T, T),
    band: Band,
) -> Result<Route, MeetFault> {
    let length = at.extent().max(T::zero());
    let e = T::from_f64(band.escalate());
    let half = length * T::from_f64(0.5) + e;
    let parallel = Margin::of(offset - rate * half - noise);
    if decide(row::PARALLEL, parallel, band) == Ok(Sign::Positive) {
        return Ok(Route::Parallel);
    }
    if decide(row::GAP, Margin::of(at.apart()), band) == Ok(Sign::Positive) {
        return Ok(Route::Apart);
    }
    let level = decide(row::TILT, Margin::levered(tilt, length + e + e), band) == Ok(Sign::Zero)
        || !definite(row::TILT, Margin::levered(tilt, lever), band)?;
    if !level {
        return Ok(Route::Crossing);
    }
    if definite(row::OFFSET, Margin::of(offset), band)? {
        return Err(MeetFault::Unsettled(Unsettled::Meter));
    }
    Ok(Route::Level)
}

/// Whether two lines are one curve within the band over both whole spans
/// (`curve_pair_pose_offset`): each span's ends lie within the zero band
/// of the other's carrier. A line's distance from another is affine in
/// its parameter, so its ends bound it over the span.
fn one_line<T: Decide>(
    (a, la): (&Span<'_, T>, (Point3<T>, Vec3<T>)),
    (b, lb): (&Span<'_, T>, (Point3<T>, Vec3<T>)),
    band: Band,
) -> Result<bool, MeetFault> {
    let off = |(o, u): (Point3<T>, Vec3<T>), q: Point3<T>| {
        let w = q - o;
        (w - u * w.dot(u)).norm()
    };
    for (s, other) in [(a, lb), (b, la)] {
        let (t0, t1) = s.params();
        for t in [t0, t1] {
            if definite(
                row::OFFSET,
                Margin::of(off(other, s.carrier().eval(t))),
                band,
            )? {
                return Ok(false);
            }
        }
    }
    Ok(true)
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
    let n = u1.cross(u2);
    let s = n.norm();
    // Read from `a`'s span start along its carrier, never from either
    // carrier's origin, and charged for how each end was evaluated:
    // `origin + dir·t` rounds at `|origin| + |dir|·|t|`, which a far
    // origin makes far larger than the end's own distance
    // (`geom_core::one_line`, "The charge").
    let ends_of = |s: &Span<'_, T>| {
        let (t0, t1) = s.params();
        (s.carrier().eval(t0), s.carrier().eval(t1))
    };
    let produced = |(o, d): (Point3<T>, Vec3<T>), s: &Span<'_, T>| {
        let (t0, t1) = s.params();
        (o - Point3::origin()).norm() * T::from_f64(2.0) + d.norm() * (t0.abs() + t1.abs())
    };
    let (a0, a1) = ends_of(a);
    let at = on_one_line(u1, (a0, a1), ends_of(b), produced(la, a) + produced(lb, b));
    let w = a0 + u1 * at.mid() - o2;
    let off = (w - u2 * w.dot(u2)).norm();
    let lever = reach(a).max(reach(b));
    let origin = Point3::origin();
    let noise = geom_core::rounding_charge(
        (o1 - origin).norm() + (o2 - origin).norm() + ends(a) + ends(b) + lever + lever,
    );
    let crossing = || {
        let along = (o2 - o1).cross(u2).dot(n) / s.powi(2);
        let point = o1 + u1 * along;
        Pose::Generic(vec![Candidate {
            point,
            ta: line_foot(la, point),
            tb: line_foot(lb, point),
        }])
    };
    match route(s, s / u1.dot(u2).abs(), off, at, (lever, noise), band)? {
        Route::Apart => Ok(Pose::Apart),
        Route::Parallel => Ok(Pose::Parallel),
        Route::Crossing => Ok(crossing()),
        Route::Level if one_line((a, (o1, u1)), (b, (o2, u2)), band)? => Ok(Pose::Coincident {
            sense: sense(u1.dot(u2), lever, band)?,
            witnesses: witnesses(a, b, &|q| line_foot(la, q), &|q| line_foot(lb, q)),
        }),
        // Level where they can meet but apart past the band along a span:
        // a shallow corner, read as the crossing the longer span's tilt
        // decides.
        Route::Level if definite(row::TILT, Margin::levered(s, lever), band)? => Ok(crossing()),
        Route::Level => Err(MeetFault::Unsettled(Unsettled::Meter)),
    }
}

/// A line `a` and a conic `b` (module docs).
fn line_conic<T: Decide>(
    a: &Span<'_, T>,
    b: &Span<'_, T>,
    known: &[Candidate<T>],
    band: Band,
) -> Result<Pose<T>, MeetFault> {
    let line = line_of(a.carrier())?;
    let k = conic_of(b.carrier())?;
    let (o, d) = line;
    let u = unit(d);
    let n = k.axis;
    let den = u.dot(n);
    // Where they can meet (module docs): the line's span and the conic's
    // extent, both projected on the line.
    let c = (k.center - o).dot(u);
    let lever = reach(a).max(k.speed_hi());
    let origin = Point3::origin();
    let noise = geom_core::rounding_charge(
        (o - origin).norm() + (k.center - origin).norm() + lever + k.speed_hi(),
    );
    let screened = noise + geom_core::rounding_charge(ends(a));
    // A conic's extent is no second span: its projection is read from the
    // line's origin, charged as the router's screens are. That anchor
    // charges the carrier origin's metres once, where a read from the
    // span's start would charge both ends' evaluation, twice as many
    // (`work/cpair/the-line-conic-router-reads-its-projections-from-the-lines-carrier-origin`).
    let at = OnOneLine::of(
        on_line((o, u), a),
        (c - k.speed_hi(), c + k.speed_hi()),
        screened,
    );
    let height = (o + u * at.mid() - k.center).dot(n);
    match route(den, den.abs(), height.abs(), at, (lever, screened), band)? {
        Route::Apart => return Ok(Pose::Apart),
        Route::Parallel => return Ok(Pose::Parallel),
        Route::Crossing => {
            // The line's height above the plane runs `|d̂·n̂|` per metre,
            // so it is within the band's escalation `e/|d̂·n̂|` either side
            // of the crossing.
            let point = o + u * ((k.center - o).dot(n) / den);
            let e = T::from_f64(band.escalate()) + noise;
            let polish = Polish {
                a: Side::Line(line),
                b: Side::Conic(&k),
                start: (line_foot(line, point), foot(b, point)),
                reach: e / den.abs(),
                clear: clearance(b.carrier(), point),
                lever,
            };
            return Ok(Pose::Generic(polish.closest(band)?.into_iter().collect()));
        }
        Route::Level => {}
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
    let range = Range::first_harmonic(c0, swing, angle(sp, cp), noise);
    let place = |theta: T| {
        let point = k.point(theta);
        Candidate {
            point,
            ta: line_foot(line, point),
            tb: theta,
        }
    };
    let found: Vec<T> = known.iter().map(|c| c.tb).collect();
    Ok(match owed(&k, &range, &found, band)? {
        Harmonic::Apart => Pose::Apart,
        Harmonic::Touch(theta) => Pose::Tangent(place(theta)),
        Harmonic::Cross(thetas) => Pose::Generic(thetas.into_iter().map(place).collect()),
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
pub(super) fn height<T: Decide>(k: &Conic<T>, on: Point3<T>, n: Vec3<T>) -> (T, T, T) {
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
    known: &[Candidate<T>],
    band: Band,
) -> Result<Pose<T>, MeetFault> {
    let (ka, kb) = (conic_of(a.carrier())?, conic_of(b.carrier())?);
    let lever = ka.speed_hi().max(kb.speed_hi());
    let origin = Point3::origin();
    let noise = geom_core::rounding_charge(
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
        return transverse((a, &ka, &kb, b), ha, swing_a, (noise, lever), known, band);
    };
    let oriented = |k: &Conic<T>| k.axis * (k.semi_u * k.semi_v);
    if one_curve(&ka, &kb, band)? {
        let (foot_a, foot_b) = (|q| foot(a, q), |q| foot(b, q));
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
            tb: foot(b, point),
        }
    };
    match coplanar {
        Coplanar::Circles => coplanar_circles(&ka, &kb, (noise, slant), &place, known, band),
        Coplanar::EllipseCircle => coplanar_ellipse_circle(a, &ka, &kb, &place, band),
        Coplanar::Ellipses => coplanar_ellipses(a, &ka, &kb, &place, band),
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
///
/// **The gap carries its own rounding charge**, so no reading of it is
/// bare. With `m` a conic's larger semi-axis magnitude, the computed
/// semi-axis vectors are off by `√17·u·m` (`v̂ = n̂ × û` by `4u`), the
/// computed `Q`'s columns are unit to within about `4.5u`, and its
/// products and the norms round by `4u·m` and `5u·g`: within
/// `12.1u·(m_e + m_f) + 5u·g` of a gap the bound holds for, inside
/// [`geom_core::rounding_charge`]`(m_e + m_f + g)`. A degenerate side (a zero
/// semi-axis, or `a·b·a′·b′` underflowing) has no way up, and its gap
/// is NaN: the reading escalates.
pub(super) fn one_curve_gap<T: Decide>(e: &Conic<T>, f: &Conic<T>) -> T {
    let reach = |k: &Conic<T>| k.semi_u.abs().max(k.semi_v.abs());
    let gap = (e.center - f.center).norm() + frame_gap(e, f).max(frame_gap(f, e));
    gap + geom_core::rounding_charge(reach(e) + reach(f) + gap)
}

/// `‖L_e − L_f·Q‖`, the operator norm, for the `Q` that turns `f`'s
/// frame the way up `e`'s is (the sign of `(a·b·n̂)_e · (a·b·n̂)_f`) and
/// then by the rotation that best matches the two, which maximizes
/// `tr(Qᵀ·L_fᵀ·L_e)`. The square is the larger eigenvalue of a Gram
/// matrix, never negative, so its enclosure is clamped at zero: two
/// coincident conics' `d1` and `d2` enclose zero, and their self-dots
/// read as interval products reach below it.
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
    (mid + (off.powi(2) + d1.dot(d2).powi(2)).sqrt())
        .max(T::from_f64(0.0))
        .sqrt()
}

/// `ka`'s height above `kb`'s plane, `(c0, cp, sp)` with the swing read
/// definite (module docs): its crossings and touch, each polished to the
/// closest pair.
fn transverse<T: Decide>(
    (a, ka, kb, b): (&Span<'_, T>, &Conic<T>, &Conic<T>, &Span<'_, T>),
    (c0, cp, sp): (T, T, T),
    swing: T,
    (noise, lever): (T, T),
    known: &[Candidate<T>],
    band: Band,
) -> Result<Pose<T>, MeetFault> {
    let range = Range::first_harmonic(c0, swing, angle(sp, cp), noise);
    let window = window::Window {
        a: ka,
        span_a: a.params(),
        b: kb,
        carrier_b: b.carrier(),
        height: (c0, cp, sp),
        noise,
    };
    let joints: Vec<_> = known
        .iter()
        .map(|c| window::Joint {
            point: c.point,
            ta: c.ta,
        })
        .collect();
    let near = |theta: T| {
        let arc = range.band_arc(theta, band);
        if !joints.is_empty() && window.holds_only((theta - arc, theta + arc), &joints, band) {
            return Ok(None);
        }
        let q = ka.point(theta);
        Polish {
            a: Side::Conic(ka),
            b: Side::Conic(kb),
            start: (theta, foot(b, q)),
            reach: ka.speed_hi() * arc,
            clear: clearance(b.carrier(), q),
            lever,
        }
        .closest(band)
    };
    Ok(match harmonic_roots(ka, &range, band)? {
        Harmonic::Apart => Pose::Apart,
        Harmonic::Touch(theta) => match near(theta)? {
            Some(c) => Pose::Tangent(c),
            None => Pose::Apart,
        },
        Harmonic::Cross(thetas) => {
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
    let (below, above) = crate::roots::spread_readings(row, value, spread, band)?;
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
    known: &[Candidate<T>],
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
    let toward = angle(w.dot(ka.v_ref()), w.dot(ka.u_ref));
    let range = Range {
        lo,
        hi,
        at_hi: ka.semi_u.select_le_zero(toward + T::pi(), toward),
        cross_lo: (T::zero() - lo) * (near + rb) / swing,
        cross_hi: hi * (far + rb) / swing,
        noise,
    };
    let found: Vec<T> = known.iter().map(|c| c.ta).collect();
    Ok(match owed(ka, &range, &found, band)? {
        Harmonic::Apart => Pose::Apart,
        Harmonic::Touch(theta) => Pose::Tangent(place(theta)),
        Harmonic::Cross(thetas) => Pose::Generic(thetas.into_iter().map(place).collect()),
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
        noise: geom_core::rounding_charge(terms),
        f_per_metre: r,
        f_per_metre_hi: T::from_f64(3.0) * r,
        residual_reach: Some(r),
        jet: None,
    };
    let residual = |theta: T| (ke.point(theta) - kc.center).norm() - r;
    subdivided(&poly, &residual, &frame, place, band)
}

/// A coplanar pair's roots and touches from the certified subdivision
/// (module docs, "A touch in the subdivision"), each placed as a
/// candidate: none is [`Pose::Apart`], one touch alone is
/// [`Pose::Tangent`], and a refusal is typed.
fn subdivided<T: Decide>(
    poly: &crate::half_angle::TrigPoly<T>,
    residual: &impl Fn(T) -> T,
    frame: &SubdivisionFrame<T>,
    place: &impl Fn(T) -> Candidate<T>,
    band: Band,
) -> Result<Pose<T>, MeetFault> {
    let found =
        certified_subdivision_touching(poly, residual, frame, (&COPLANAR_ROWS, &COPLANAR_TOUCH), band)?;
    let roots: &[T] = match &found.roots {
        Roots::Miss => &[],
        Roots::Certified(r) => r.roots(),
        Roots::On(never) => match *never {},
    };
    Ok(match (roots, &found.touches[..]) {
        ([], []) => Pose::Apart,
        ([], &[touch]) => Pose::Tangent(place(touch)),
        _ => Pose::Generic(
            roots
                .iter()
                .chain(&found.touches)
                .map(|&t| place(t))
                .collect(),
        ),
    })
}

/// **Two ellipses in one plane** (module docs): the first's level on
/// the second, `F(θ) = X²/A² + Y²/B² − 1` at the first's point `(X, Y)`
/// in the second's frame, is of degree two in the first's angle, and goes
/// to the certified subdivision in the metres of
/// [`geom::ellipse_level_clearance`], the lower bound on a point's
/// distance from the second that [`Curve3::clearance`] reads, signed as
/// `F`.
///
/// **The ceiling on `|F|` per metre of that bound.** `X²/A⁴ + Y²/B⁴` is
/// at most `(F + 1)/s²`, `s` the second's smaller semi-axis, so `g = |∇F|`
/// is at most `2√(1 + |F|)/s`, and the bound
/// `2|F|/(g + √(g² + 4|F|/s²))` is at least
/// `|F|·s/(√(1 + |F|) + √(1 + 2|F|))`. Where `|F| ≤ 1` that is at least
/// `|F|·s/(√2 + √3) > |F|·s/4`; past it, increasing in `|F|`, at least
/// `s/(√2 + √3) > s/4`. So the bound is at least `min(s/4, |F|/(4/s))`
/// everywhere: the ceiling `4/s`, reaching to `s/4`.
fn coplanar_ellipses<T: Decide>(
    e: &Span<'_, T>,
    ka: &Conic<T>,
    kb: &Conic<T>,
    place: &impl Fn(T) -> Candidate<T>,
    band: Band,
) -> Result<Pose<T>, MeetFault> {
    let (big_a, big_b) = (kb.semi_u, kb.semi_v);
    let (ub, vb) = (kb.u_ref, kb.v_ref());
    let (poly, terms) = level_on(ka, kb);
    let small = big_a.abs().min(big_b.abs());
    let (t0, t1) = e.params();
    let (ceiling, reach) = level_ceiling(small);
    let frame = SubdivisionFrame {
        t0,
        t1,
        speed_hi: ka.speed_hi(),
        noise: geom_core::rounding_charge(terms),
        // `g` vanishes at the second's centre: no floor but zero holds.
        f_per_metre: T::zero(),
        f_per_metre_hi: ceiling,
        residual_reach: Some(reach),
        jet: None,
    };
    let residual = |theta: T| {
        let d = ka.point(theta) - kb.center;
        geom::ellipse_level_clearance((d.dot(ub), d.dot(vb)), (big_a, big_b))
    };
    subdivided(&poly, &residual, &frame, place, band)
}

/// **`b`'s level along `a`**: `F(θ) = X²/A² + Y²/B² − 1` at the image of
/// `a(θ)` in `b`'s plane, `(X, Y)` in `b`'s frame and `(A, B)` its
/// semi-axes, of degree two in `a`'s angle (each of `X` and `Y` is a
/// first harmonic); and the term bound its rounding is charged on. The
/// height of `a(θ)` above `b`'s plane plays no part.
pub(super) fn level_on<T: Decide>(
    ka: &Conic<T>,
    kb: &Conic<T>,
) -> (crate::half_angle::TrigPoly<T>, T) {
    let (big_a, big_b) = (kb.semi_u, kb.semi_v);
    let (ub, vb) = (kb.u_ref, kb.v_ref());
    let (ua, va) = (ka.u_ref * ka.semi_u, ka.v_ref() * ka.semi_v);
    let w = ka.center - kb.center;
    // `X = x₀ + p cos θ + q sin θ`, `Y = y₀ + r cos θ + t sin θ`.
    let (x0, p, q) = (w.dot(ub), ua.dot(ub), va.dot(ub));
    let (y0, r, t) = (w.dot(vb), ua.dot(vb), va.dot(vb));
    let (a2, b2) = (big_a.powi(2), big_b.powi(2));
    let two = T::from_f64(2.0);
    let poly = DegreeTwo::new(
        (x0.powi(2) + (p.powi(2) + q.powi(2)) / two) / a2
            + (y0.powi(2) + (r.powi(2) + t.powi(2)) / two) / b2
            - T::one(),
        two * (x0 * p / a2 + y0 * r / b2),
        two * (x0 * q / a2 + y0 * t / b2),
        (p.powi(2) - q.powi(2)) / (two * a2) + (r.powi(2) - t.powi(2)) / (two * b2),
        p * q / a2 + r * t / b2,
    )
    .poly();
    let small = big_a.abs().min(big_b.abs());
    let terms = ((w.norm() + ka.semi_u.abs() + ka.semi_v.abs()) / small).powi(2) + T::one();
    (poly, terms)
}

/// **The ceiling on an ellipse's level per metre of
/// [`geom::ellipse_level_clearance`], and how far it reaches**
/// (`coplanar_ellipses`'s docs): `(4/s, s/4)`, `s` the smaller semi-axis.
pub(super) fn level_ceiling<T: Decide>(small: T) -> (T, T) {
    let four = T::from_f64(4.0);
    (four / small, small / four)
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
        let phi = separation(theta, self.at_hi);
        let inner = unit_cos((e - c0) / swing).acos();
        let outer = unit_cos((T::zero() - e - c0) / swing).acos();
        (phi - inner).abs().max((outer - phi).abs())
    }
}

/// What a first harmonic's extremes and root gap decide.
enum Harmonic<T> {
    /// No root: the extremes definitely one-signed.
    Apart,
    /// A touch at this angle: a double root, or an extreme in the zero
    /// band that has not crossed.
    Touch(T),
    /// Crossings: two, or those left once the found meetings' roots are
    /// divided out (module docs, "deflation").
    Cross(Vec<T>),
}

/// **A first harmonic's roots along `k` still owed**, `found` the angles
/// of the meetings already found (module docs, "deflation"): none
/// found, every root ([`harmonic_roots`]); one, the other, `2θ_hi − θ_J`;
/// two, none.
fn owed<T: Decide>(
    k: &Conic<T>,
    r: &Range<T>,
    found: &[T],
    band: Band,
) -> Result<Harmonic<T>, MeetFault> {
    Ok(match found {
        [] => harmonic_roots(k, r, band)?,
        [theta] => Harmonic::Cross(vec![r.at_hi + r.at_hi - *theta]),
        _ => Harmonic::Cross(Vec::new()),
    })
}

/// A first harmonic's roots along `k` (module docs).
fn harmonic_roots<T: Decide>(
    k: &Conic<T>,
    r: &Range<T>,
    band: Band,
) -> Result<Harmonic<T>, MeetFault> {
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
        (Sign::Positive, _) | (_, Sign::Negative) => return Ok(Harmonic::Apart),
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
        Sign::Zero => Harmonic::Touch(at_near),
        Sign::Positive | Sign::Negative => Harmonic::Cross(roots.to_vec()),
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
