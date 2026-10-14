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
//!   speed. Definite: the candidate is where the line crosses the plane.
//!   Zero: the line's height above the plane at its span's middle decides
//!   [`Pose::Parallel`], or, read zero, the line in the plane. There the
//!   conic's signed distance from the line, `(C(θ) − o)·m̂` with `m̂` the
//!   line's normal in the plane, is a first harmonic in the conic's angle
//!   (below).
//! - **Two conics.** The tilt between their planes, `|n̂₁ × n̂₂|` levered by
//!   the larger conic's greatest speed. Definite: the first conic's height
//!   above the second's plane, `(C₁(θ) − c₂)·n̂₂`, is a first harmonic in
//!   the first conic's angle, and its roots are the candidates, each read
//!   against the second conic. Zero: the planes' offset decides
//!   [`Pose::Parallel`], or, read zero, the conics in one plane:
//!   - **two circles**: their centres' distance `d`
//!     (`curve_pair_pose_centres`). Zero is one centre, and the radii's gap
//!     with `d` as its spread (`curve_pair_pose_radii`) decides one curve
//!     or [`Pose::Concentric`]. Definite, the first circle's distance from
//!     the second's centre runs between `|r₁ − d|` and `r₁ + d`, so its
//!     distance from the second circle has those extremes less `r₂`,
//!     exactly; and its square is a first harmonic in the first circle's
//!     angle, whose roots are read from the extremes (below);
//!   - **a circle and an ellipse**: one curve when the bound
//!     `|Δc| + max(|a − r|, |b − r|)`, plus the planes' offset and tilt,
//!     reads zero (`curve_pair_pose_one_curve`); otherwise the ellipse's
//!     distance from the circle, `|E(θ) − c| − r`, whose square less `r²`
//!     is of degree two in the ellipse's angle, goes to the certified
//!     subdivision ([`crate::half_angle::certified_subdivision`]);
//!   - **two ellipses**: one curve when the frames' bound
//!     `|Δc| + min_Q ‖L₁ − L₂Q‖`, `L = [a·û, b·v̂]` over the eight signed
//!     permutations `Q`, reads zero. Two ellipses in one plane that are not
//!     one curve have no metre residual here, and refuse typed
//!     ([`PoseRefusal::Unsupported`]).
//!
//! # A first harmonic's roots, and a double root
//!
//! A residual `R(θ)` whose range is `[lo, hi]` is decided on its extremes
//! (`curve_pair_pose_extreme`), each read as the interval its rounding
//! spans: definitely one-signed is [`Pose::Apart`], definitely straddling
//! is two crossings, an end in the escalation gap escalates, and an
//! extreme in the zero band is a touch there. The roots are read from the
//! extreme nearer zero, `2·asin(√(|near|/swing))` past it, which reads
//! that extreme to its own relative precision.
//!
//! **A double root is decided by the gap between its roots along the
//! curve**, never by the margin of a centre or an extreme: the chord
//! between the two root points (`curve_pair_pose_root_gap`) decides one
//! touch or two crossings. Two near-concentric circles stay within
//! `d + |Δr|` of each other all the way round, so an extreme in the band
//! says nothing about where they meet; the root gap does.
//!
//! **A candidate admitted under a `Zero` reading is placed by the exactly
//! degenerate formula**: a touch is the extreme's own point, `C(θ_hi)` or
//! `C(θ_hi + π)`, on the first carrier exactly and within the extreme's
//! value of the second. An extreme read zero whose value has not crossed
//! (no real root) is that touch too.
//!
//! **A crossing of the other's plane is metered** (`curve_pair_pose_root_slack`):
//! a root's rounding moves its point along the first carrier by the
//! residual's noise over its slope, and that moves the point off the
//! second carrier by the sine of the angle between them. Not definitely
//! inside the band, the pose refuses ([`Unsettled::Meter`]): the meeting
//! of two curves in nearly parallel planes crossing at a wide angle is
//! placed by the planes' common line, which their rounding moves. In one
//! plane the residual is the distance to the other carrier itself, and
//! the reading of the candidate decides it.
//!
//! # The band
//!
//! Every decision above that falls in the band's gap escalates
//! ([`PoseRefusal::Escalated`]); none prunes a candidate.

use geom::{Curve3, CurveKind};
use geom_core::k_stats::{decide, decide_reported, straddle};
use geom_core::{Band, Decide, Decided, Indeterminate, Margin, Point3, Sign, Vec3};

use super::span::Span;
use super::{Arm, Method};
use crate::Conic;
use crate::half_angle::{SubdivisionFrame, SubdivisionRows, TrigPoly, certified_subdivision};
use crate::roots::{Roots, Unsettled};

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
    /// of: each span's middle, and the points halfway between each span's
    /// ends and the other's.
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
    /// One touch, a double root the band cannot split, placed by the
    /// degenerate formula.
    Tangent(Candidate<T>),
    /// Isolated meetings, at most four, each a candidate to be read.
    Generic(Vec<Candidate<T>>),
}

/// Why [`pose`] has no pose to give.
#[derive(Clone, Copy, Debug)]
pub enum PoseRefusal {
    /// A decision fell in the band's gap.
    Escalated(Indeterminate),
    /// A root finder or a meter could not certify the candidates.
    Unsettled(Unsettled),
    /// The pair has a spiric or spline side: its meetings are the walk's.
    Walked {
        /// The first carrier's kind.
        a: CurveKind,
        /// The second carrier's kind.
        b: CurveKind,
    },
    /// A pose this arm has no closed form for.
    Unsupported {
        /// The pose.
        pose: &'static str,
    },
    /// A carrier is not of the kind its arm reads.
    Desync,
}

impl From<Indeterminate> for PoseRefusal {
    fn from(diag: Indeterminate) -> Self {
        Self::Escalated(diag)
    }
}

/// The predicate rows the pose reads under.
mod row {
    pub(super) const TILT: &str = "curve_pair_pose_tilt";
    pub(super) const OFFSET: &str = "curve_pair_pose_offset";
    pub(super) const SENSE: &str = "curve_pair_pose_sense";
    pub(super) const SWING: &str = "curve_pair_pose_swing";
    pub(super) const CONSTANT: &str = "curve_pair_pose_constant";
    pub(super) const EXTREME: &str = "curve_pair_pose_extreme";
    pub(super) const ROOT_GAP: &str = "curve_pair_pose_root_gap";
    pub(super) const ROOT_SLACK: &str = "curve_pair_pose_root_slack";
    pub(super) const CENTRES: &str = "curve_pair_pose_centres";
    pub(super) const RADII: &str = "curve_pair_pose_radii";
    pub(super) const ONE_CURVE: &str = "curve_pair_pose_one_curve";
}

/// The coplanar circle × ellipse subdivision's rows.
const COPLANAR_ROWS: SubdivisionRows = SubdivisionRows {
    clear: "curve_pair_coplanar_sub_clear",
    monotone: "curve_pair_coplanar_sub_monotone",
    side: "curve_pair_coplanar_sub_side",
    width: "curve_pair_coplanar_sub_width",
};

/// The Gauss–Newton steps a foot on an ellipse is polished by.
const FOOT_STEPS: usize = 4;

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
        Method::Closed(arm) => closed(arm, a, b, band),
        Method::Walk => Err(PoseRefusal::Walked { a: ka, b: kb }),
    }
}

/// [`pose`] for a closed-form arm.
pub(super) fn closed<T: Decide>(
    arm: Arm,
    a: &Span<'_, T>,
    b: &Span<'_, T>,
    band: Band,
) -> Result<Pose<T>, PoseRefusal> {
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
fn line_of<T: Decide>(c: &Curve3<T>) -> Result<(Point3<T>, Vec3<T>), PoseRefusal> {
    match *c {
        Curve3::Line { origin, dir } => Ok((origin, dir)),
        Curve3::Circle { .. }
        | Curve3::Ellipse { .. }
        | Curve3::Spiric { .. }
        | Curve3::Nurbs(_) => Err(PoseRefusal::Desync),
    }
}

/// A conic carrier's frame.
fn conic_of<T: Decide>(c: &Curve3<T>) -> Result<Conic<T>, PoseRefusal> {
    Conic::of(c).ok_or(PoseRefusal::Desync)
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
    k.u_ref * (T::zero() - k.major * sin) + k.v_ref() * (k.minor * cos)
}

/// The conic's parameter nearest `q`: the angle of `q`'s unit
/// coordinates, exact on a circle and on any point of the ellipse,
/// polished by Gauss–Newton otherwise. A foot that is not the nearest
/// point only reads the point farther off ([`Span::contact`]).
fn conic_foot<T: Decide>(k: &Conic<T>, q: Point3<T>) -> T {
    let w = q - k.center;
    let mut t = (w.dot(k.v_ref()) / k.minor).atan2(w.dot(k.u_ref) / k.major);
    for _ in 0..FOOT_STEPS {
        let d = tangent(k, t);
        t = t - (k.point(t) - q).dot(d) / d.dot(d);
    }
    t
}

/// The line's parameter nearest `q`.
fn line_foot<T: Decide>((origin, dir): (Point3<T>, Vec3<T>), q: Point3<T>) -> T {
    (q - origin).dot(dir) / dir.dot(dir)
}

/// How two carriers read as one curve run, from the cosine between
/// their directions or oriented normals levered by `lever`: definitely
/// signed wherever they are one curve.
fn sense<T: Decide>(cosine: T, lever: T, band: Band) -> Result<Sense, PoseRefusal> {
    match decide(row::SENSE, Margin::levered(cosine, lever), band)? {
        Sign::Positive => Ok(Sense::Same),
        Sign::Negative => Ok(Sense::Opposite),
        Sign::Zero => Err(PoseRefusal::Unsettled(Unsettled::Degenerate)),
    }
}

/// Whether `x`, levered by `arm`, is definitely nonzero (`row`).
fn definite<T: Decide>(
    row: &'static str,
    margin: Margin<T>,
    band: Band,
) -> Result<bool, PoseRefusal> {
    Ok(match decide(row, margin, band)? {
        Sign::Zero => false,
        Sign::Positive | Sign::Negative => true,
    })
}

/// The witnesses of a coincident pair (each span's middle, and the points
/// halfway between each span's ends and the other's), each placed on the
/// carrier of the span it came from and given its foot on the other by
/// `foot_a`/`foot_b`.
fn witnesses<T: Decide>(
    a: &Span<'_, T>,
    b: &Span<'_, T>,
    foot_a: &impl Fn(Point3<T>) -> T,
    foot_b: &impl Fn(Point3<T>) -> T,
) -> Vec<Candidate<T>> {
    let mut out = Vec::new();
    let on = |s: &Span<'_, T>, other: &Span<'_, T>, foot: &dyn Fn(Point3<T>) -> T| {
        let (t0, t1) = s.params();
        let mut ts = vec![geom::mid_param(t0, t1)];
        let (o0, o1) = other.params();
        for q in [other.carrier().eval(o0), other.carrier().eval(o1)] {
            let t = s.branch(foot(q));
            ts.extend([geom::mid_param(t0, t), geom::mid_param(t, t1)]);
        }
        ts
    };
    for ta in on(a, b, foot_a) {
        let point = a.carrier().eval(ta);
        out.push(Candidate {
            point,
            ta,
            tb: foot_b(point),
        });
    }
    for tb in on(b, a, foot_b) {
        let point = b.carrier().eval(tb);
        out.push(Candidate {
            point,
            ta: foot_a(point),
            tb,
        });
    }
    out
}

/// Two lines (module docs).
fn line_line<T: Decide>(
    a: &Span<'_, T>,
    b: &Span<'_, T>,
    band: Band,
) -> Result<Pose<T>, PoseRefusal> {
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
) -> Result<Pose<T>, PoseRefusal> {
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
        let point = o + u * ((k.center - o).dot(n) / den);
        let tb = conic_foot(&k, point);
        // The crossing's rounding moves it along the line by `noise/|d̂·n̂|`,
        // which moves it off the conic by the sine between the line and
        // the conic there.
        let sine = u.cross(unit(tangent(&k, tb))).norm();
        let slack = noise / den.abs() * sine;
        if !matches!(
            decide(row::ROOT_SLACK, Margin::of(slack), band),
            Ok(Sign::Zero | Sign::Negative)
        ) {
            return Err(PoseRefusal::Unsettled(Unsettled::Meter));
        }
        return Ok(Pose::Generic(vec![Candidate {
            point,
            ta: line_foot(line, point),
            tb,
        }]));
    }
    if definite(row::OFFSET, Margin::of((middle(a) - k.center).dot(n)), band)? {
        return Ok(Pose::Parallel);
    }
    // In the plane: the conic's signed distance from the line, along the
    // line's normal in the plane, a first harmonic in the conic's angle.
    let m = unit(n.cross(u));
    let c0 = (k.center - o).dot(m);
    let (cp, sp) = (k.major * k.u_ref.dot(m), k.minor * k.v_ref().dot(m));
    let swing = (cp.powi(2) + sp.powi(2)).sqrt();
    if !definite(row::SWING, Margin::of(swing), band)? {
        return Err(PoseRefusal::Unsupported {
            pose: "a conic the band reads as thin as the line it lies beside",
        });
    }
    let (lo, hi) = (c0 - swing, c0 + swing);
    let two = swing + swing;
    let range = Range {
        lo,
        hi,
        at_hi: sp.atan2(cp),
        cross_lo: (T::zero() - lo) / two,
        cross_hi: hi / two,
        noise,
    };
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

/// Two conics, `a` the one parametrized (module docs).
fn conic_conic<T: Decide>(
    a: &Span<'_, T>,
    b: &Span<'_, T>,
    coplanar: Coplanar,
    band: Band,
) -> Result<Pose<T>, PoseRefusal> {
    let (ka, kb) = (conic_of(a.carrier())?, conic_of(b.carrier())?);
    let lever = ka.speed_hi().max(kb.speed_hi());
    let tilt = ka.axis.cross(kb.axis).norm();
    let origin = Point3::origin();
    let noise = crate::rounding_charge(
        (ka.center - origin).norm() + (kb.center - origin).norm() + lever + lever,
    );
    let place = |theta: T| {
        let point = ka.point(theta);
        Candidate {
            point,
            ta: theta,
            tb: conic_foot(&kb, point),
        }
    };
    if definite(row::TILT, Margin::levered(tilt, lever), band)? {
        return transverse(&ka, &kb, noise, &place, band);
    }
    let offset = (kb.center - ka.center).dot(ka.axis);
    if definite(row::OFFSET, Margin::of(offset), band)? {
        return Ok(Pose::Parallel);
    }
    // In one plane, to within the offset and the tilt over the conics.
    let slant = offset.abs() + tilt * lever;
    let oriented = |k: &Conic<T>| k.axis * (k.major * k.minor);
    let cosine = unit(oriented(&ka)).dot(unit(oriented(&kb)));
    let foot_a = |q: Point3<T>| conic_foot(&ka, q);
    let foot_b = |q: Point3<T>| conic_foot(&kb, q);
    let coincident = || -> Result<Pose<T>, PoseRefusal> {
        Ok(Pose::Coincident {
            sense: sense(cosine, lever, band)?,
            witnesses: witnesses(a, b, &foot_a, &foot_b),
        })
    };
    match coplanar {
        Coplanar::Circles => coplanar_circles(&ka, &kb, noise, &place, &coincident, band),
        Coplanar::EllipseCircle => {
            let w = ka.center - kb.center;
            let r = kb.major.abs();
            let bound =
                w.norm() + (ka.major.abs() - r).abs().max((ka.minor.abs() - r).abs()) + slant;
            if !definite(row::ONE_CURVE, Margin::of(bound), band)? {
                return coincident();
            }
            coplanar_ellipse_circle(a, &ka, &kb, &place, band)
        }
        Coplanar::Ellipses => {
            if !definite(
                row::ONE_CURVE,
                Margin::of(frame_gap(&ka, &kb) + slant),
                band,
            )? {
                return coincident();
            }
            Err(PoseRefusal::Unsupported {
                pose: "two ellipses in one plane that are not one curve",
            })
        }
    }
}

/// A bound on how far a point of one coplanar ellipse lies from the
/// other: the centres' gap plus the least, over the eight signed
/// permutations `Q` of the frame, Frobenius norm of `L₁ − L₂Q`,
/// `L = [a·û, b·v̂]`, which pairs each point `c₁ + L₁x` of the first with
/// the point `c₂ + L₂Qx` of the second.
fn frame_gap<T: Decide>(e: &Conic<T>, f: &Conic<T>) -> T {
    let (e1, e2) = (e.u_ref * e.major, e.v_ref() * e.minor);
    let (f1, f2) = (f.u_ref * f.major, f.v_ref() * f.minor);
    let pair = |p: Vec3<T>, q: Vec3<T>| ((e1 - p).norm_squared() + (e2 - q).norm_squared()).sqrt();
    let neg = |v: Vec3<T>| v * (T::zero() - T::one());
    let gaps = [
        pair(f1, f2),
        pair(neg(f1), neg(f2)),
        pair(f1, neg(f2)),
        pair(neg(f1), f2),
        pair(f2, f1),
        pair(neg(f2), neg(f1)),
        pair(f2, neg(f1)),
        pair(neg(f2), f1),
    ];
    let least = gaps.into_iter().fold(gaps[0], |x, y| x.min(y));
    (e.center - f.center).norm() + least
}

/// The first conic's height above the second's plane, a first harmonic
/// (module docs): its roots, each metered.
fn transverse<T: Decide>(
    ka: &Conic<T>,
    kb: &Conic<T>,
    noise: T,
    place: &impl Fn(T) -> Candidate<T>,
    band: Band,
) -> Result<Pose<T>, PoseRefusal> {
    let n = kb.axis;
    let c0 = (ka.center - kb.center).dot(n);
    let (cp, sp) = (ka.major * ka.u_ref.dot(n), ka.minor * ka.v_ref().dot(n));
    let swing = (cp.powi(2) + sp.powi(2)).sqrt();
    match decide(row::SWING, Margin::of(swing), band)? {
        Sign::Positive | Sign::Negative => {}
        Sign::Zero => {
            if constant(c0, swing + noise, band)? {
                return Ok(Pose::Apart);
            }
            return Err(PoseRefusal::Unsupported {
                pose: "a conic the band reads in the other's plane, the planes read crossing",
            });
        }
    }
    let (lo, hi) = (c0 - swing, c0 + swing);
    let two = swing + swing;
    let range = Range {
        lo,
        hi,
        at_hi: sp.atan2(cp),
        cross_lo: (T::zero() - lo) / two,
        cross_hi: hi / two,
        noise,
    };
    Ok(match harmonic_roots(ka, &range, band)? {
        Found::Apart => Pose::Apart,
        Found::Touch(theta) => Pose::Tangent(place(theta)),
        Found::Cross(thetas) => {
            let mut out = Vec::with_capacity(2);
            for theta in thetas {
                let c = place(theta);
                let (sin, cos) = theta.sin_cos();
                let slope = sp * cos - cp * sin;
                let along = tangent(ka, theta);
                let across = unit(along).cross(unit(tangent(kb, c.tb))).norm();
                let slack = along.norm() * noise / slope.abs() * across;
                if !matches!(
                    decide(row::ROOT_SLACK, Margin::of(slack), band),
                    Ok(Sign::Zero | Sign::Negative)
                ) {
                    return Err(PoseRefusal::Unsettled(Unsettled::Meter));
                }
                out.push(c);
            }
            Pose::Generic(out)
        }
    })
}

/// Whether a residual constant to within `spread` about `value` is
/// definitely clear of zero (`true`) or in the zero band all round
/// (`false`).
///
/// # Errors
///
/// An end in the band's gap, or ends that straddle the band.
fn constant<T: Decide>(value: T, spread: T, band: Band) -> Result<bool, PoseRefusal> {
    let below = decide_reported(row::CONSTANT, Margin::of(value - spread), band)?;
    let above = decide_reported(row::CONSTANT, Margin::of(value + spread), band)?;
    match (below.sign, above.sign) {
        (Sign::Positive, _) | (_, Sign::Negative) => Ok(true),
        (Sign::Zero, Sign::Zero) => Ok(false),
        _ => Err(straddled(row::CONSTANT, below, above, band)),
    }
}

/// Two decided readings of one quantity's ends straddling the band.
fn straddled(row: &'static str, below: Decided, above: Decided, band: Band) -> PoseRefusal {
    PoseRefusal::Escalated(straddle(row, below, above, band))
}

/// Two circles in one plane (module docs).
fn coplanar_circles<T: Decide>(
    ka: &Conic<T>,
    kb: &Conic<T>,
    noise: T,
    place: &impl Fn(T) -> Candidate<T>,
    coincident: &impl Fn() -> Result<Pose<T>, PoseRefusal>,
    band: Band,
) -> Result<Pose<T>, PoseRefusal> {
    let (ra, rb) = (ka.major.abs(), kb.major.abs());
    let w = ka.center - kb.center;
    let w = w - ka.axis * w.dot(ka.axis);
    let d = w.norm();
    if !definite(row::CENTRES, Margin::norm3(w), band)? {
        let gap = ra - rb;
        let spread = d + noise;
        let below = decide_reported(row::RADII, Margin::of(gap - spread), band)?;
        let above = decide_reported(row::RADII, Margin::of(gap + spread), band)?;
        return match (below.sign, above.sign) {
            (Sign::Positive, _) | (_, Sign::Negative) => Ok(Pose::Concentric),
            (Sign::Zero, Sign::Zero) => coincident(),
            _ => Err(straddled(row::RADII, below, above, band)),
        };
    }
    // `|C₁(θ) − c₂|` runs between `|r₁ − d|` and `r₁ + d`; its square
    // less `r₂²` is `c₀ + 2r₁d·cos(θ − θ_hi)`, a first harmonic whose
    // swing is `4r₁d`.
    let (near, far) = ((ra - d).abs(), ra + d);
    let (lo, hi) = (near - rb, far - rb);
    let swing = T::from_f64(4.0) * ra * d;
    let range = Range {
        lo,
        hi,
        at_hi: w.dot(ka.v_ref()).atan2(w.dot(ka.u_ref)),
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
) -> Result<Pose<T>, PoseRefusal> {
    let r = kc.major.abs();
    let w = ke.center - kc.center;
    let (a, b) = (ke.major, ke.minor);
    let two = T::from_f64(2.0);
    // `|E(θ) − c|² − r²` in the ellipse's angle.
    let c0 = w.norm_squared() + (a.powi(2) + b.powi(2)) / two - r.powi(2);
    let (c1, s1) = (two * a * w.dot(ke.u_ref), two * b * w.dot(ke.v_ref()));
    let c2 = (a.powi(2) - b.powi(2)) / two;
    let z = T::zero();
    let poly = TrigPoly {
        cos: [c0, c1, c2, z, z],
        sin: [z, s1, z, z, z],
        degree: 2,
    };
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
        Ok(Roots::Certified { count, roots }) => Ok(Pose::Generic(
            roots[..count].iter().map(|&t| place(t)).collect(),
        )),
        Err(why) => Err(PoseRefusal::Unsettled(why)),
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

/// The sign of an extreme read as `value` with error at most `noise`:
/// ends that disagree read `Zero`.
fn extreme_sign<T: Decide>(value: T, noise: T, band: Band) -> Result<Sign, PoseRefusal> {
    let below = decide(row::EXTREME, Margin::of(value - noise), band)?;
    let above = decide(row::EXTREME, Margin::of(value + noise), band)?;
    Ok(if below == above { below } else { Sign::Zero })
}

/// A first harmonic's roots along `k` (module docs).
fn harmonic_roots<T: Decide>(
    k: &Conic<T>,
    r: &Range<T>,
    band: Band,
) -> Result<Found<T>, PoseRefusal> {
    let lo = extreme_sign(r.lo, r.noise, band)?;
    let hi = extreme_sign(r.hi, r.noise, band)?;
    // The angle from `hi` of the roots read from each extreme: none
    // crossed reads a double root at that extreme.
    let past = |cross: T| {
        let two = T::from_f64(2.0);
        two * cross.max(T::zero()).min(T::one()).sqrt().asin()
    };
    let from_hi = past(r.cross_hi);
    let from_lo = T::pi() - past(r.cross_lo);
    let at_lo = r.at_hi + T::pi();
    let (psi, at_near) = match (lo, hi) {
        (Sign::Positive, _) | (_, Sign::Negative) => return Ok(Found::Apart),
        (Sign::Zero, Sign::Zero) => return Err(PoseRefusal::Unsettled(Unsettled::Tangency)),
        (Sign::Zero, Sign::Positive) => (from_lo, at_lo),
        (Sign::Negative, Sign::Zero) => (from_hi, r.at_hi),
        // Two definite crossings: read from the extreme that has crossed
        // the less, to its own relative precision.
        (Sign::Negative, Sign::Positive) => {
            let nearer_hi = r.cross_hi - r.cross_lo;
            (
                nearer_hi.select_le_zero(from_hi, from_lo),
                nearer_hi.select_le_zero(r.at_hi, at_lo),
            )
        }
    };
    let roots = [r.at_hi + psi, r.at_hi - psi];
    let gap = Margin::norm3(k.point(roots[0]) - k.point(roots[1]));
    Ok(match decide(row::ROOT_GAP, gap, band)? {
        Sign::Zero => Found::Touch(at_near),
        Sign::Positive | Sign::Negative => Found::Cross(roots),
    })
}
