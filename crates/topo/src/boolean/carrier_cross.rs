//! **Where an edge lying on a curved face's carrier crosses that face's
//! boundary in its own interior.**
//!
//! The declared-cover arms of [`super::reduce::curved_face_arm`] place an
//! on-carrier edge's two ENDPOINTS against the face. What happens between
//! them is a question about curves, not points: on the shared carrier the
//! edge and each boundary edge of the face are two curves of one surface,
//! and wherever they meet, the edge passes into or out of the face. A
//! shaft's seam ruling running through a full-turn bore crosses the bore's
//! two rim circles that way, with both of its ends past the bore and
//! neither rim's vertex anywhere near it.
//!
//! [`boundary_crossing`] finds such a meeting, and the sweep splits both
//! edges there as it splits a wall pierce landing on a boundary edge.
//! Where either crossing edge has a face transverse to the other (a
//! bore's rim and the flat cap beyond it, which a shaft ruling crosses)
//! the planar lane records the crossing as well. Where both edges have
//! the shared carrier on both sides, this is the only recorder: measured
//! on a bore split by a circle on its own carrier and crossed by a
//! shaft's seam rulings
//! (`full_turn_bore_mate::a_bore_split_on_its_own_carrier_unions_at_the_seam_azimuth`).
//! Its other job is the certificate behind
//! [`super::reduce`]'s all-`Elsewhere` reading: a span whose interior
//! meets the boundary nowhere, with both ends outside, lies outside. The candidate points are closed form per
//! carrier pair — every vertex of the face's boundary, and the
//! transverse meetings of the edge's carrier with each boundary edge's
//! carrier — and each is then asked the two questions the sweep already
//! has rows for: does it lie strictly inside the edge's span, and does
//! the face's boundary pre-pass put it ON the boundary. A candidate that
//! is not a meeting fails the second question, so the generation only
//! has to be complete, never exact. On iso-curves of one surface of
//! revolution the two kinds overlap (a boundary vertex projects onto a
//! coaxial curve at the meeting); off them only the closed forms find
//! it.
//!
//! **Complete over line, circle and ellipse carriers**, in either role.
//! Two lines or circles meet either at a point the pair's closed form
//! names (a line through a circle's plane; two non-parallel lines; two
//! circles whose planes cross, along the planes' common line; two
//! circles in one plane), or along a shared stretch whose ends are
//! vertices of one of them — the boundary vertices, which are always
//! candidates. An ellipse is a plane section, so whatever meets it meets
//! its plane: a line where it crosses the plane, a circle or another
//! ellipse on the two planes' common line. Two ellipses in parallel
//! planes meet nowhere or are one curve: an ellipse bounds a cylinder or
//! a cone here, which a plane cuts in one conic, so two of its ellipses
//! in one plane share a stretch. The band reads planes parallel that are
//! not, and a section stretches their tilt by its semi-axes' ratio, so
//! two conics read coplanar — two ellipses, or two circles (a sphere's
//! sections tilted within the band cross where the band reads them
//! parallel) — are taken as one curve only where they are one within
//! the band: in band they escalate, and two ellipses definitely apart
//! answer [`BoundaryCrossing::Unread`]. A line or circle lying in an ellipse's
//! plane would need the coplanar conic pair, which this module does not
//! hold, and on those carriers it cannot arise: a plane holding a ruling
//! cuts the carrier in rulings, and one holding a circle cuts it in that
//! circle. A spiric or spline carrier on either side, and a face holding
//! a lone vertex (a pierce ring), which has no boundary pre-pass, answer
//! [`BoundaryCrossing::Unread`], and the caller keeps its frontier door.

use geom_core::{Band, Decide, Margin, Point3, Sign, Vec3};

use super::contain::FaceContainment;
use super::{BooleanDecision, BooleanError, CrossingDecision, Operand};
use crate::body::Body;
use crate::entity::{EntityId, FaceKey};
use crate::live::BoundaryMember;
use crate::null::CurveGeom;
use crate::validate::decide;

/// What [`boundary_crossing`] found along an on-carrier edge.
#[derive(Debug, Clone, Copy)]
pub(super) enum BoundaryCrossing<T: geom_core::Real> {
    /// The edge meets the face's boundary at parameter `t`, strictly
    /// inside its span, at `p`; `at` is the boundary entity there.
    At {
        t: T,
        p: Point3<T>,
        at: FaceContainment,
        /// The margin that decided `p` on that boundary entity.
        margin: geom_core::MarginDiag,
    },
    /// Certified: the edge's interior meets the face's boundary nowhere.
    Clear,
    /// A carrier pair this module has no closed form for: nothing is
    /// known about the interior.
    Unread,
}

/// A point strictly inside `carrier`'s span `(t0, t1)` where it meets
/// `face`'s boundary. The span must lie on `face`'s carrier — the
/// caller's certificate, which this function does not re-ask.
///
/// # Errors
///
/// [`BooleanError::Escalated`] when a decision falls in band: whether
/// a candidate lies inside the span, whether two carriers are parallel,
/// or the boundary pre-pass's own rows.
/// [`BooleanError::ClassificationInvariant`] where `face`, a key the
/// caller passed, does not resolve.
///
/// # Panics
///
/// Where a record past `face` does not resolve or a loop walk does not
/// close (D2 row 4): its loops, a lone vertex's point, their members, a
/// member's start point, edge and curve. `y` is the reduction's working copy, whose links
/// hold mid-operation by [`crate::live::OPERATORS_KEEP_LINKS`].
pub(super) fn boundary_crossing<T: Decide>(
    y: &Body<T>,
    y_is: Operand,
    face: FaceKey,
    carrier: &geom::Curve3<T>,
    (t0, t1): (T, T),
    band: Band,
) -> Result<BoundaryCrossing<T>, BooleanError> {
    let mid = geom::mid_param(t0, t1);
    let Some(swept) = Span::of(carrier, (t0, t1)) else {
        return Ok(BoundaryCrossing::Unread);
    };
    let face_data = y
        .get_face(face)
        .ok_or(BooleanError::ClassificationInvariant {
            what: "on-carrier crossing: face lost",
        })?;
    let mut candidates: Vec<Point3<T>> = Vec::new();
    for member in y.face_boundary_linked(face, face_data) {
        let BoundaryMember::Edge { he, half, ek, edge } = member else {
            return Ok(BoundaryCrossing::Unread);
        };
        candidates.push(y.linked_vertex_point(half.start, EntityId::HalfEdge(he), "start"));
        let CurveGeom::Certified(other) = y.edge_curve_linked(ek, edge) else {
            return Ok(BoundaryCrossing::Unread);
        };
        let Some(partner) = Span::of(other.carrier(), other.params()) else {
            return Ok(BoundaryCrossing::Unread);
        };
        match meetings(swept, partner, band)? {
            Some(points) => candidates.extend(points),
            None => return Ok(BoundaryCrossing::Unread),
        }
    }
    // The first meeting in the boundary's walk order: the sweep splits
    // there and re-examines both fragments against this face, so every
    // other meeting is found again on them.
    // Per end, the metred gaps of the candidates that read at it.
    let mut at_end: [Vec<T>; 2] = [Vec::new(), Vec::new()];
    for q in candidates {
        let Some(t) = carrier.param_near(q, mid) else {
            return Ok(BoundaryCrossing::Unread);
        };
        let Some(speed) = geom_brep::carrier_speed::speed_at(carrier, t) else {
            return Ok(BoundaryCrossing::Unread);
        };
        let gaps = [(t - t0) * speed, (t1 - t) * speed];
        let signs = span_signs(gaps, band)?;
        for ((group, gap), sign) in at_end.iter_mut().zip(gaps).zip(signs) {
            if sign == Sign::Zero {
                group.push(gap);
            }
        }
        if signs != [Sign::Positive; 2] {
            continue;
        }
        let p = carrier.eval(t);
        if let Some((at, margin)) = super::contain::curved_boundary_containment(y, face, p, band)
            .map_err(|e| super::reduce::esc(e, y_is, face))?
        {
            return Ok(BoundaryCrossing::At { t, p, at, margin });
        }
    }
    for group in &at_end {
        one_point_at_end(group, band)?;
    }
    Ok(BoundaryCrossing::Clear)
}

/// Whether the candidates that read at one end of the span, with the
/// end itself, read as one point: every pair of them, as
/// [`super::vtxfac::ring_groups`] reads the pierces at one point. Each
/// reads at the end alone, which is not transitive: candidates for one
/// crossing a sliver of carriers spreads along the edge (a ruling tilted
/// within the band of its bore's axis meets each rim at a slightly
/// different turn) each read Zero at whichever of them the sweep split
/// at, while two of them lie in the band of each other. Which of them
/// is the split is the operand order's choice, so the pair's verdict
/// must not wait on it: a pair in band escalates however the edge was
/// split.
fn one_point_at_end<T: Decide>(gaps: &[T], band: Band) -> Result<(), BooleanError> {
    for (i, &a) in gaps.iter().enumerate() {
        for &b in std::iter::once(&T::zero()).chain(&gaps[i + 1..]) {
            if let Err(diag) = decide("bool_carrier_cross_in_span", Margin::of(a - b), band) {
                return Err(escalated(diag));
            }
        }
    }
    Ok(())
}

/// An in-band decision of this module: where a crossing lands along its
/// edge is undetermined.
fn escalated(diag: geom_core::Indeterminate) -> BooleanError {
    BooleanError::Escalated {
        decision: BooleanDecision::Crossing(CrossingDecision::OnEdge),
        diag,
    }
}

/// The signs of a candidate's gaps to the span's two ends, `t − t0`
/// and `t1 − t`, each metred by the carrier's speed at the candidate
/// ([`geom_brep::carrier_speed::speed_at`]): the chord from it to an end
/// is the gap times that speed to second order in the gap, and a gap
/// near the band is a few band widths long. The candidate is strictly
/// inside where both are Positive; an end is not inside, since an
/// endpoint's incidence is the endpoint arms' question.
fn span_signs<T: Decide>(gaps: [T; 2], band: Band) -> Result<[Sign; 2], BooleanError> {
    let sign = |gap| decide("bool_carrier_cross_in_span", Margin::of(gap), band).map_err(escalated);
    Ok([sign(gaps[0])?, sign(gaps[1])?])
}

/// Whether a dimensionless `x` (a sine), levered at `arm`, is
/// definitely nonzero.
fn transverse<T: Decide>(x: T, arm: T, band: Band) -> Result<bool, BooleanError> {
    match decide(
        "bool_carrier_cross_transverse",
        Margin::levered(x, arm),
        band,
    ) {
        Ok(Sign::Zero) => Ok(false),
        Ok(Sign::Positive | Sign::Negative) => Ok(true),
        Err(diag) => Err(escalated(diag)),
    }
}

/// One side of a [`meetings`] pair: a carrier, a bound on its edge's
/// length (the span at the carrier's greatest speed) and a point of the
/// edge.
#[derive(Clone, Copy)]
struct Span<'a, T: geom_core::Real> {
    carrier: &'a geom::Curve3<T>,
    reach: T,
    at: Point3<T>,
}

impl<'a, T: Decide> Span<'a, T> {
    /// The edge of `carrier` over `(t0, t1)`; `None` for a carrier with
    /// no closed-form speed.
    fn of(carrier: &'a geom::Curve3<T>, (t0, t1): (T, T)) -> Option<Self> {
        Some(Self {
            carrier,
            reach: (t1 - t0) * geom_brep::carrier_speed::speed_hi(carrier)?,
            at: carrier.eval(geom::mid_param(t0, t1)),
        })
    }
}

/// The isolated points where the swept edge `a` and the boundary edge
/// `b` can meet, beyond the boundary vertices the caller already holds:
/// empty where they meet only along a shared stretch or not at all,
/// `None` for a pair with no closed form here. A meeting lies on both
/// edges, so a line's tilt is read over its own edge's reach, and a line
/// read parallel is placed at its own edge's point; which edge is swept
/// does not change where two carriers meet, so each pair has one arm
/// and its mirror swaps the roles.
fn meetings<T: Decide>(
    a: Span<'_, T>,
    b: Span<'_, T>,
    band: Band,
) -> Result<Option<Vec<Point3<T>>>, BooleanError> {
    use geom::Curve3::{Circle, Ellipse, Line};
    let conics = (
        geom_brep::Conic::of(a.carrier),
        geom_brep::Conic::of(b.carrier),
    );
    Ok(Some(match (a.carrier, b.carrier, conics) {
        (
            &Line { origin, dir },
            &Circle {
                center,
                axis,
                radius,
                ..
            },
            _,
        )
        | (
            &Circle {
                center,
                axis,
                radius,
                ..
            },
            &Line { origin, dir },
            _,
        ) => {
            let den = dir.dot(axis);
            if !transverse(den, radius, band)? {
                // The line runs parallel to the circle's plane. On a
                // carrier holding both, that is a ruling beside a circle
                // that is not one of the carrier's rims, which no
                // carrier here has.
                return Ok(None);
            }
            let t = (center - origin).dot(axis) / den;
            vec![origin + dir * t]
        }
        (
            &Line {
                origin: o1,
                dir: d1,
            },
            &Line {
                origin: o2,
                dir: d2,
            },
            _,
        ) => {
            let n = d1.cross(d2);
            let s = n.norm();
            // Parallel lines meet along a stretch whose ends are
            // boundary vertices, or nowhere. The sine is read over the
            // swept edge's own length, the farthest it can carry a tilt.
            if !transverse(s, a.reach, band)? {
                return Ok(Some(Vec::new()));
            }
            let t = (o2 - o1).cross(d2).dot(n) / s.powi(2);
            vec![o1 + d1 * t]
        }
        (&Circle { .. }, &Circle { .. }, (Some(e), Some(f))) => {
            // A circle's speed is its radius `|r|`.
            if !transverse(
                e.axis.cross(f.axis).norm(),
                e.speed_hi().max(f.speed_hi()),
                band,
            )? {
                return Ok(Some(parallel_circles(&e, &f, band)?));
            }
            circle_meets_plane(e.center, e.axis, e.speed_hi(), f.center, f.axis, band)?
        }
        // An ellipse is a plane section, so whatever meets it meets its
        // plane: those meetings are the candidates, and the boundary
        // pre-pass drops any not on the ellipse. Every point of the
        // ellipse lies within its larger semi-axis of its centre. A
        // curve lying in that plane has no closed form here.
        //
        // A line is read parallel to the plane only where its tilt
        // carries it less than the band over its edge and the ellipse
        // alike; its edge then stays within the band of its distance at
        // its point, so a definite distance there certifies that the
        // edge misses the plane.
        (&Line { origin, dir }, &Ellipse { .. }, (_, Some(e))) => {
            let den = dir.dot(e.axis);
            if !transverse(den, a.reach.max(e.speed_hi()), band)? {
                return Ok(off_plane(a.at, e.center, e.axis, band)?.then(Vec::new));
            }
            let t = (e.center - origin).dot(e.axis) / den;
            vec![origin + dir * t]
        }
        (&Circle { .. }, &Ellipse { .. }, (Some(c), Some(e))) => {
            // Read parallel, every point of the circle stays within the
            // band of its centre's distance from the plane.
            let r = c.speed_hi();
            if !transverse(c.axis.cross(e.axis).norm(), r.max(e.speed_hi()), band)? {
                return Ok(off_plane(c.center, e.center, e.axis, band)?.then(Vec::new));
            }
            circle_meets_plane(c.center, c.axis, r, e.center, e.axis, band)?
        }
        (&Ellipse { .. }, &Line { .. } | &Circle { .. }, _) => return meetings(b, a, band),
        (&Ellipse { .. }, &Ellipse { .. }, (Some(e), Some(f))) => {
            let (c1, n1, c2, n2) = (e.center, e.axis, f.center, f.axis);
            // Read parallel, each ellipse stays within the band of its
            // centre's distance from the other's plane, so a definite
            // distance between the planes certifies that they share no
            // point.
            if !transverse(n1.cross(n2).norm(), e.speed_hi().max(f.speed_hi()), band)? {
                if off_plane(c2, c1, n1, band)? {
                    return Ok(Some(Vec::new()));
                }
                // In one plane: one curve is a shared stretch whose ends
                // are vertices. Two curves are two sections the band
                // reads coplanar, and they can cross where the band
                // cannot place it: a cylinder's section stretches the
                // plane's tilt by the semi-axes' ratio.
                return Ok(one_curve(&e, &f, band)?.then(Vec::new));
            }
            ellipse_meets_plane(&e, c2, n2, band)?
        }
        (&Circle { .. } | &Ellipse { .. }, _, (None, _))
        | (_, &Circle { .. } | &Ellipse { .. }, (_, None)) => {
            unreachable!("Conic::of reads every circle and ellipse")
        }
        _ => return Ok(None),
    }))
}

/// Whether two conics whose planes read parallel are, within the band,
/// one curve ([`one_curve_gap`]).
fn one_curve<T: Decide>(
    e: &geom_brep::Conic<T>,
    f: &geom_brep::Conic<T>,
    band: Band,
) -> Result<bool, BooleanError> {
    match decide(
        "bool_carrier_cross_one_curve",
        Margin::of(one_curve_gap(e, f)),
        band,
    ) {
        Ok(Sign::Zero) => Ok(true),
        Ok(Sign::Positive | Sign::Negative) => Ok(false),
        Err(diag) => Err(escalated(diag)),
    }
}

/// A bound on how far a point of either conic lies from the other,
/// tilt out of the plane included. Each conic is
/// `c + L·x` over the unit circle's `x`, with `L = [a·û, b·v̂]`. For any
/// orthogonal `Q`, pairing `c_e + L_e·x` with `c_f + L_f·Q·x` pairs every
/// point of each with a point of the other within
/// `|c_e − c_f| + ‖L_e − L_f·Q‖` ([`frame_gap`]). The stored order and
/// signs of the semi-axes change `L` by an orthogonal factor, which `Q`
/// absorbs. The gap read from either side is the same but for rounding,
/// and the larger is taken, so the gap does not depend on the order of
/// the pair.
fn one_curve_gap<T: Decide>(e: &geom_brep::Conic<T>, f: &geom_brep::Conic<T>) -> T {
    (e.center - f.center).norm() + frame_gap(e, f).max(frame_gap(f, e))
}

/// `‖L_e − L_f·Q‖`, the operator norm, for the `Q` that turns `f`'s
/// frame the way up `e`'s is (the sign of `(a·b·n̂)_e · (a·b·n̂)_f`,
/// near `±a·b·a′·b′` where the planes are parallel) and then by the
/// rotation that best matches the two, which maximizes
/// `tr(Qᵀ·L_fᵀ·L_e)`.
fn frame_gap<T: Decide>(e: &geom_brep::Conic<T>, f: &geom_brep::Conic<T>) -> T {
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

/// Whether `p` lies definitely off the plane through `on` with unit
/// normal `n`.
fn off_plane<T: Decide>(
    p: Point3<T>,
    on: Point3<T>,
    n: Vec3<T>,
    band: Band,
) -> Result<bool, BooleanError> {
    match decide(
        "bool_carrier_cross_plane_offset",
        Margin::of((p - on).dot(n)),
        band,
    ) {
        Ok(Sign::Zero) => Ok(false),
        Ok(Sign::Positive | Sign::Negative) => Ok(true),
        Err(diag) => Err(escalated(diag)),
    }
}

/// The common line of the planes through `c1` and `c2` with unit
/// normals `n1` and `n2`, decided transverse: its point nearest the
/// origin of the two-plane system, and its unit direction `n1 × n2`.
fn common_line<T: Decide>(
    c1: Point3<T>,
    n1: Vec3<T>,
    c2: Point3<T>,
    n2: Vec3<T>,
) -> (Point3<T>, Vec3<T>) {
    let m = n1.cross(n2);
    let s = m.norm();
    let (h1, h2) = (n1.dot(c1 - Point3::origin()), n2.dot(c2 - Point3::origin()));
    let c = n1.dot(n2);
    let k = s.powi(2);
    let p0 = Point3::origin() + (n1 * ((h1 - h2 * c) / k) + n2 * ((h2 - h1 * c) / k));
    (p0, m * (T::one() / s))
}

/// Where the circle (`c1`, unit `n1`, `r1`) meets the plane through
/// `c2` with unit normal `n2`, the two planes decided transverse: the
/// planes' [`common_line`]'s meetings with the circle's sphere, which,
/// in the circle's plane, are its meetings with the circle.
fn circle_meets_plane<T: Decide>(
    c1: Point3<T>,
    n1: Vec3<T>,
    r1: T,
    c2: Point3<T>,
    n2: Vec3<T>,
    band: Band,
) -> Result<Vec<Point3<T>>, BooleanError> {
    let (p0, u) = common_line(c1, n1, c2, n2);
    let w = p0 - c1;
    let b = w.dot(u);
    let disc = b.powi(2) - (w.dot(w) - r1.powi(2));
    Ok(
        match decide(
            "bool_carrier_cross_line_meets_circle",
            Margin::levered_inv(disc, r1 + r1),
            band,
        ) {
            Ok(Sign::Negative) => Vec::new(),
            Ok(Sign::Zero) => vec![p0 + u * (-b)],
            Ok(Sign::Positive) => {
                let root = disc.sqrt();
                vec![p0 + u * (-b - root), p0 + u * (-b + root)]
            }
            Err(diag) => return Err(escalated(diag)),
        },
    )
}

/// Where the ellipse `e` meets the plane through `c` with unit normal
/// `n`, the two planes decided transverse: the planes' [`common_line`]
/// `p0 + s·u`, then its meetings with `e` in `e`'s frame, the roots of
/// `qa·s² + 2qb·s + qc = 0` from `(x/a)² + (y/b)² = 1` scaled by `a²b²`.
///
/// The discriminant is `a²b²·qa·(1 − D²)`, `D` the line's distance from
/// the centre in the frame that takes the ellipse to the unit circle,
/// where the line's gap from tangency, along its normal in the plane, is
/// `√qa·(1 − D)` metres. Near tangency `1 − D² = 2(1 − D)` to first
/// order, so the discriminant over `2a²b²·√qa` is that gap: the
/// circle's row at `a = b = r`.
fn ellipse_meets_plane<T: Decide>(
    e: &geom_brep::Conic<T>,
    c: Point3<T>,
    n: Vec3<T>,
    band: Band,
) -> Result<Vec<Point3<T>>, BooleanError> {
    let (p0, u) = common_line(e.center, e.axis, c, n);
    let (u_ref, v_ref) = (e.u_ref, e.v_ref());
    let w = p0 - e.center;
    let (x0, y0, ux, uy) = (w.dot(u_ref), w.dot(v_ref), u.dot(u_ref), u.dot(v_ref));
    let (a2, b2) = (e.semi_u.powi(2), e.semi_v.powi(2));
    let qa = b2 * ux.powi(2) + a2 * uy.powi(2);
    let qb = b2 * x0 * ux + a2 * y0 * uy;
    let qc = b2 * x0.powi(2) + a2 * y0.powi(2) - a2 * b2;
    let disc = qb.powi(2) - qa * qc;
    let per = a2 * b2 * (qa.sqrt() + qa.sqrt());
    Ok(
        match decide(
            "bool_carrier_cross_line_meets_ellipse",
            Margin::levered_inv(disc, per),
            band,
        ) {
            Ok(Sign::Negative) => Vec::new(),
            Ok(Sign::Zero) => vec![p0 + u * (-qb / qa)],
            Ok(Sign::Positive) => {
                let root = disc.sqrt();
                vec![p0 + u * ((-qb - root) / qa), p0 + u * ((-qb + root) / qa)]
            }
            Err(diag) => return Err(escalated(diag)),
        },
    )
}

/// Two circles in planes read parallel: none where the planes are
/// distinct, where the circles are one within the band ([`one_curve`]),
/// or where they share a centre (nested ones), else the meetings of two
/// circles in one plane — two, one where they touch, none where they are
/// apart or one holds the other.
fn parallel_circles<T: Decide>(
    e: &geom_brep::Conic<T>,
    f: &geom_brep::Conic<T>,
    band: Band,
) -> Result<Vec<Point3<T>>, BooleanError> {
    // A circle's speed is its radius `|r|`.
    let (c1, n1, r1, c2, r2) = (e.center, e.axis, e.speed_hi(), f.center, f.speed_hi());
    let sign = |row, margin| decide(row, margin, band).map_err(escalated);
    if off_plane(c2, c1, n1, band)? || one_curve(e, f, band)? {
        return Ok(Vec::new());
    }
    let w = c2 - c1;
    if sign("bool_carrier_cross_concentric", Margin::norm3(w))? == Sign::Zero {
        return Ok(Vec::new());
    }
    let d = w.norm();
    // Apart, or one inside the other: no meeting. Touching: one.
    let apart = sign("bool_carrier_cross_circles_apart", Margin::of(r1 + r2 - d))?;
    let nested = sign(
        "bool_carrier_cross_circles_nested",
        Margin::of(d - (r1 - r2).abs()),
    )?;
    if apart == Sign::Negative || nested == Sign::Negative {
        return Ok(Vec::new());
    }
    let toward = w * (T::one() / d);
    let a = (d.powi(2) + r1.powi(2) - r2.powi(2)) / (d + d);
    let foot = c1 + toward * a;
    if apart == Sign::Zero || nested == Sign::Zero {
        return Ok(vec![foot]);
    }
    let h = (r1.powi(2) - a.powi(2)).sqrt();
    let across = n1.cross(toward);
    Ok(vec![foot + across * h, foot - across * h])
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod meetings_rows {
    //! The closed forms behind [`super::meetings`]: every isolated meeting
    //! of two carriers is among the candidates, a pair that meets only
    //! along a stretch or nowhere has none, and a pair with no closed
    //! form says so.
    use super::{Span, meetings, one_curve, one_curve_gap};
    use geom::Curve3;
    use geom_core::{Band, Point3, Tol, Vec3};

    /// One side read as a one-metre edge from its parameter `0` to `1`.
    fn edge(c: &Curve3<f64>) -> Span<'_, f64> {
        Span {
            carrier: c,
            reach: 1.0,
            at: c.eval(0.5),
        }
    }

    /// The meetings, both sides read as one-metre edges.
    fn meet(
        a: &Curve3<f64>,
        b: &Curve3<f64>,
        band: Band,
    ) -> Result<Option<Vec<Point3<f64>>>, crate::boolean::BooleanError> {
        meetings(edge(a), edge(b), band)
    }

    fn band() -> Band {
        Band::linear(Tol::witness()).unwrap()
    }

    fn line(o: [f64; 3], d: [f64; 3]) -> Curve3<f64> {
        Curve3::Line {
            origin: Point3::from_array(o),
            dir: Vec3::from_array(d).normalize(),
        }
    }

    fn circle(c: [f64; 3], axis: [f64; 3], r: f64) -> Curve3<f64> {
        let axis = Vec3::from_array(axis).normalize();
        let u_ref = if axis.x.abs() < 0.9 {
            Vec3::new(1.0, 0.0, 0.0)
        } else {
            Vec3::new(0.0, 1.0, 0.0)
        };
        let u_ref = (u_ref - axis * u_ref.dot(axis)).normalize();
        Curve3::Circle {
            center: Point3::from_array(c),
            axis,
            radius: r,
            u_ref,
        }
    }

    /// Whether `want` is among `got`.
    fn holds(got: &[Point3<f64>], want: [f64; 3]) -> bool {
        got.iter()
            .any(|p| p.distance(Point3::from_array(want)) < 1e-12)
    }

    #[test]
    fn every_isolated_meeting_is_a_candidate() {
        let b = band();
        // A cylinder's ruling through its rim's plane, both orders.
        let ruling = line([0.5, -3.0, 0.0], [0.0, 1.0, 0.0]);
        let rim = circle([0.0, 1.0, 0.0], [0.0, 1.0, 0.0], 0.5);
        for (a, c) in [(&ruling, &rim), (&rim, &ruling)] {
            let got = meet(a, c, b).unwrap().unwrap();
            assert!(holds(&got, [0.5, 1.0, 0.0]), "ruling × rim: {got:?}");
        }
        // Two skew-free lines crossing.
        let got = meet(
            &line([0.0, 0.0, 0.0], [1.0, 0.0, 0.0]),
            &line([1.0, -1.0, 0.0], [0.0, 1.0, 0.0]),
            b,
        )
        .unwrap()
        .unwrap();
        assert!(holds(&got, [1.0, 0.0, 0.0]), "line × line: {got:?}");
        // Two great circles of the unit sphere meet at two antipodes.
        let got = meet(
            &circle([0.0; 3], [0.0, 0.0, 1.0], 1.0),
            &circle([0.0; 3], [1.0, 0.0, 0.0], 1.0),
            b,
        )
        .unwrap()
        .unwrap();
        assert!(
            holds(&got, [0.0, 1.0, 0.0]) && holds(&got, [0.0, -1.0, 0.0]),
            "great circles: {got:?}"
        );
        // A torus's two meridians in one plane, either side of the axis,
        // and two overlapping circles in one plane.
        let got = meet(
            &circle([0.8, 0.0, 0.0], [0.0, 0.0, 1.0], 0.5),
            &circle([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], 0.5),
            b,
        )
        .unwrap()
        .unwrap();
        let y = 0.09f64.sqrt();
        assert!(
            holds(&got, [0.4, y, 0.0]) && holds(&got, [0.4, -y, 0.0]),
            "coplanar circles: {got:?}"
        );
        // A parallel of the unit sphere and a meridian: at latitude 30°.
        let (s, c) = (0.5, 0.75f64.sqrt());
        let got = meet(
            &circle([0.0, 0.0, s], [0.0, 0.0, 1.0], c),
            &circle([0.0; 3], [0.0, 1.0, 0.0], 1.0),
            b,
        )
        .unwrap()
        .unwrap();
        assert!(
            holds(&got, [c, 0.0, s]) && holds(&got, [-c, 0.0, s]),
            "parallel × meridian: {got:?}"
        );
    }

    fn ellipse(c: [f64; 3], axis: [f64; 3], major: f64, minor: f64) -> Curve3<f64> {
        let Curve3::Circle {
            center,
            axis,
            u_ref,
            ..
        } = circle(c, axis, 1.0)
        else {
            unreachable!("circle builds a circle")
        };
        Curve3::Ellipse {
            center,
            axis,
            semi_u: major,
            semi_v: minor,
            u_ref,
        }
    }

    /// **An ellipse is met where its plane is met**: the oblique section
    /// of the unit cylinder `z = 0.2·x`, against a swept ruling, rim and
    /// tilted circle. Each true meeting is among the candidates; the
    /// ruling's lies on the ellipse, and a circle's plane crossings need
    /// not.
    #[test]
    fn an_ellipse_is_met_at_its_plane() {
        let b = band();
        let k = 1.04f64.sqrt();
        let cut = ellipse([0.0; 3], [-0.2, 0.0, 1.0], k, 1.0);
        let ruling = line([0.6, 0.8, -3.0], [0.0, 0.0, 1.0]);
        let rim = circle([0.0, 0.0, 0.1], [0.0, 0.0, 1.0], 1.0);
        let tilted = circle([0.0, 0.0, 0.0], [0.0, 1.0, 0.0], 1.0);
        let got = meet(&ruling, &cut, b).unwrap().unwrap();
        assert!(holds(&got, [0.6, 0.8, 0.12]), "ruling × ellipse: {got:?}");
        for (a, c, want, what) in [
            (
                &rim,
                &cut,
                vec![[0.5, 0.75f64.sqrt(), 0.1], [0.5, -(0.75f64.sqrt()), 0.1]],
                "rim",
            ),
            (
                &tilted,
                &cut,
                vec![[1.0 / k, 0.0, 0.2 / k], [-1.0 / k, 0.0, -0.2 / k]],
                "tilted",
            ),
        ] {
            let got = meet(a, c, b).unwrap().unwrap();
            for w in &want {
                assert!(holds(&got, *w), "{what} × ellipse: {w:?} in {got:?}");
            }
        }
        // A line parallel to the ellipse's plane and off it, and a
        // circle in a parallel plane: no candidate. In the plane: unread.
        let lifted = |z: f64| line([0.0, 0.0, z], [1.0 / k, 0.0, 0.2 / k]);
        assert_eq!(
            meet(&lifted(1.0), &cut, b).unwrap().map(|v| v.len()),
            Some(0)
        );
        assert!(meet(&lifted(0.0), &cut, b).unwrap().is_none());
        let beside = |z: f64| circle([0.0, 0.0, z], [-0.2, 0.0, 1.0], 0.5);
        assert_eq!(
            meet(&beside(1.0), &cut, b).unwrap().map(|v| v.len()),
            Some(0)
        );
        assert!(meet(&beside(0.0), &cut, b).unwrap().is_none());
    }

    /// **A swept line's tilt is read over its whole reach.** With `z` the
    /// band's zero threshold: a line `100·z` above the plane of a
    /// unit-major ellipse at its origin and sloping down `z/2` crosses
    /// the plane at `(1, 0, 0)`, `200 m` on. Its tilt is under the band
    /// at the ellipse's size, but over a `300 m` swept edge it carries
    /// `150·z`, so the crossing is a candidate. The same line sloping
    /// `z/3000` carries `z/10` over that reach and is read parallel:
    /// about `100·z` off the plane at the span's middle, its span
    /// certainly misses it. (At the witness band the first line is the
    /// `1e-7` offset and `5e-10` slope.)
    #[test]
    fn a_swept_lines_tilt_is_read_over_its_reach() {
        let b = band();
        let z = b.zero();
        let cut = ellipse([0.0; 3], [0.0, 0.0, 1.0], 1.0, 0.5);
        let sloped = |slope: f64| {
            let dir = Vec3::new(1.0, 0.0, -slope).normalize();
            let origin = Point3::new(1.0, 0.0, 0.0) + dir * (-100.0 * z / slope);
            Curve3::Line { origin, dir }
        };
        let swept = |l: &Curve3<f64>| {
            let span = Span {
                carrier: l,
                reach: 300.0,
                at: l.eval(150.0),
            };
            meetings(span, edge(&cut), b)
        };
        let steep = sloped(z / 2.0);
        let Curve3::Line { origin, .. } = steep else {
            unreachable!("a line")
        };
        assert!(
            (origin.z - 100.0 * z).abs() < 1e-3 * z,
            "100·z above the plane: {origin:?}"
        );
        let got = swept(&steep).unwrap().unwrap();
        assert!(
            got.iter()
                .any(|p| p.distance(Point3::new(1.0, 0.0, 0.0)) < 1e-6),
            "the crossing 200 m on is a candidate: {got:?}"
        );
        let flat = line([0.0, 0.0, 100.0 * z], [1.0, 0.0, -z / 3000.0]);
        assert_eq!(
            swept(&flat).unwrap().map(|v| v.len()),
            Some(0),
            "read parallel and off the plane"
        );
    }

    #[test]
    fn a_stretch_or_a_miss_has_no_candidate_and_a_parallel_line_is_unread() {
        let b = band();
        let none = |a: &Curve3<f64>, c: &Curve3<f64>| meet(a, c, b).unwrap().map(|v| v.len());
        // Two parallel rulings, and one ruling along another.
        let r1 = line([0.5, 0.0, 0.0], [0.0, 1.0, 0.0]);
        let r2 = line([0.0, 0.0, 0.5], [0.0, 1.0, 0.0]);
        assert_eq!(none(&r1, &r2), Some(0), "parallel rulings");
        assert_eq!(none(&r1, &r1), Some(0), "one ruling");
        // Two rims of one cylinder, and one rim against itself.
        let low = circle([0.0, 1.0, 0.0], [0.0, 1.0, 0.0], 0.5);
        let high = circle([0.0, 2.0, 0.0], [0.0, 1.0, 0.0], 0.5);
        assert_eq!(none(&low, &high), Some(0), "two rims");
        assert_eq!(none(&low, &low), Some(0), "one rim");
        // Nested circles in one plane.
        let inner = circle([0.1, 1.0, 0.0], [0.0, 1.0, 0.0], 0.2);
        assert_eq!(none(&low, &inner), Some(0), "nested");
        // A line parallel to a circle's plane.
        let flat = line([0.0, 1.0, 0.0], [1.0, 0.0, 0.0]);
        assert_eq!(none(&flat, &low), None, "line in the rim's plane");
    }

    /// **A swept ellipse is met where its partner meets its plane**,
    /// which is the partner's own row with the roles swapped: the same
    /// candidates in either order. The oblique section of the unit
    /// cylinder `z = 0.2·x` against a ruling, a rim, a tilted circle and
    /// a second section of the same cylinder holds every true meeting.
    /// Two sections in parallel planes, one plane or two, have no
    /// candidate; a line or circle in the ellipse's plane is unread.
    #[test]
    fn a_swept_ellipse_is_met_where_its_plane_is_met() {
        let b = band();
        let k = 1.04f64.sqrt();
        let cut = ellipse([0.0; 3], [-0.2, 0.0, 1.0], k, 1.0);
        let ruling = line([0.6, 0.8, -3.0], [0.0, 0.0, 1.0]);
        let rim = circle([0.0, 0.0, 0.1], [0.0, 0.0, 1.0], 1.0);
        let tilted = circle([0.0, 0.0, 0.0], [0.0, 1.0, 0.0], 1.0);
        let other = ellipse([0.0, 0.0, 0.1], [0.3, 0.0, 1.0], 1.09f64.sqrt(), 1.0);
        let h = 0.75f64.sqrt();
        let g = 0.96f64.sqrt();
        for (c, want, what) in [
            (&ruling, vec![[0.6, 0.8, 0.12]], "ruling"),
            (&rim, vec![[0.5, h, 0.1], [0.5, -h, 0.1]], "rim"),
            (&tilted, vec![], "tilted"),
            (
                &other,
                vec![[0.2, g, 0.04], [0.2, -g, 0.04]],
                "other section",
            ),
        ] {
            let swept = meet(&cut, c, b).unwrap().unwrap();
            for w in &want {
                assert!(holds(&swept, *w), "ellipse × {what}: {w:?} in {swept:?}");
            }
            let partner = meet(c, &cut, b).unwrap().unwrap();
            if what == "other section" {
                for w in &want {
                    assert!(holds(&partner, *w), "the other order: {w:?} in {partner:?}");
                }
            } else {
                assert_eq!(
                    swept.iter().map(|p| p.to_array()).collect::<Vec<_>>(),
                    partner.iter().map(|p| p.to_array()).collect::<Vec<_>>(),
                    "ellipse × {what}: either order"
                );
            }
        }
        let count = |c: &Curve3<f64>| meet(&cut, c, b).unwrap().map(|v| v.len());
        let above = ellipse([0.0, 0.0, 0.5], [-0.2, 0.0, 1.0], k, 1.0);
        assert_eq!(count(&above), Some(0), "a parallel section");
        assert_eq!(count(&cut), Some(0), "the same section");
        let lifted = |z: f64| line([0.0, 0.0, z], [1.0 / k, 0.0, 0.2 / k]);
        assert_eq!(count(&lifted(1.0)), Some(0), "a line beside the plane");
        assert_eq!(count(&lifted(0.0)), None, "a line in the plane");
        let beside = |z: f64| circle([0.0, 0.0, z], [-0.2, 0.0, 1.0], 0.5);
        assert_eq!(count(&beside(1.0)), Some(0), "a circle beside the plane");
        assert_eq!(count(&beside(0.0)), None, "a circle in the plane");
    }

    /// **Two sections read coplanar are one curve only where they are.**
    /// The unit cylinder cut by `z = m·x` and `z = m(1 + d)·x`, `m = 100`:
    /// the two ellipses cross at `(0, ±1, 0)`. With `d` set so the
    /// planes' tilt carries `k·z` across the larger semi-axis (`z` the
    /// band's zero, `k` = 0.1, 0.5, 0.9), the band reads the planes
    /// parallel and through one point, while the larger semi-axes differ
    /// by about `m·k·z`. That is no certified absence of meetings: the
    /// pair answers unread, or at `k = 0.1`, where the semi-axes' gap
    /// sits at the band's edge, unread or escalated.
    #[test]
    fn two_sections_read_coplanar_in_band_are_one_curve_only_where_they_are() {
        let b = band();
        let z = b.zero();
        let m = 100.0f64;
        let section = |slope: f64| ellipse([0.0; 3], [-slope, 0.0, 1.0], slope.hypot(1.0), 1.0);
        let tilt_reach = |d: f64| {
            let (e, f) = (section(m), section(m * (1.0 + d)));
            let (
                Curve3::Ellipse { axis: n1, .. },
                Curve3::Ellipse {
                    axis: n2, semi_u, ..
                },
            ) = (e, f)
            else {
                unreachable!("two ellipses")
            };
            n1.cross(n2).norm() * semi_u
        };
        for k in [0.1, 0.5, 0.9] {
            let d = k * z * (k * z / tilt_reach(k * z));
            assert!(
                (tilt_reach(d) / z - k).abs() < 1e-3,
                "k = {k}: the tilt carries {} z",
                tilt_reach(d) / z
            );
            let (e, f) = (section(m), section(m * (1.0 + d)));
            for (x, y, order) in [(&e, &f, "e × f"), (&f, &e, "f × e")] {
                let got = meet(x, y, b);
                let unread = matches!(got, Ok(None));
                let escalated = matches!(got, Err(crate::boolean::BooleanError::Escalated { .. }));
                assert!(
                    unread || (k < 0.2 && escalated),
                    "k = {k}, {order}: two crossing sections are not one curve: {got:?}"
                );
            }
        }
    }

    /// **Two ellipses are one curve only where each lies within the band
    /// of the other**, a tilt out of the plane counted like any other
    /// move. An ellipse in a skew plane, of semi-axis ratio 1 to 100 at
    /// two sizes, against itself moved so that its farthest point leaves
    /// it by `d`: its centre within the plane and off it, each semi-axis,
    /// a turn in the plane (`d = (a − b)·φ`), and a tilt about either
    /// axis and the diagonal (`d` = the tilt times the extent across the
    /// hinge). Each is stored as built, with its semi-axes in the other
    /// order, and upside down. With `z` the band's zero, `d = 0.6·z` is
    /// one curve, `1.5·z` is not, and twice the escalation edge is
    /// definitely two, read the same in either order, the gap itself
    /// bit for bit. A circle turned in its plane is itself.
    #[test]
    fn two_ellipses_are_one_curve_only_within_the_band_of_each_other() {
        use crate::boolean::BooleanError;
        use geom_brep::Conic;
        let b = band();
        let z = b.zero();
        let turn = |x: Vec3<f64>, k: Vec3<f64>, t: f64| {
            let (s, c) = t.sin_cos();
            x * c + k.cross(x) * s + k * (k.dot(x) * (1.0 - c))
        };
        let n = Vec3::new(1.0, 2.0, 3.0).normalize();
        let u = n.cross(Vec3::new(0.0, 0.0, 1.0)).normalize();
        let v = n.cross(u);
        let read = |e: &Conic<f64>, f: &Conic<f64>| {
            assert_eq!(
                one_curve_gap(e, f).to_bits(),
                one_curve_gap(f, e).to_bits(),
                "the gap is the same either way round: {e:?} {f:?}"
            );
            let (ef, fe) = (one_curve(e, f, b), one_curve(f, e, b));
            let key = |r: &Result<bool, BooleanError>| match r {
                Ok(one) => Some(*one),
                Err(BooleanError::Escalated { .. }) => None,
                Err(other) => panic!("only a band reading refuses: {other:?}"),
            };
            assert_eq!(key(&ef), key(&fe), "either order: {e:?} {f:?}");
            key(&ef)
        };
        for ratio in [1.0, 1.001, 2.0, 10.0, 100.0] {
            for size in [0.01, 1.0] {
                let (major, minor) = (ratio * size, size);
                let e = Conic {
                    center: Point3::new(1.0, -2.0, 0.5),
                    axis: n,
                    u_ref: u,
                    semi_u: major,
                    semi_v: minor,
                };
                let tilt = |hinge: Vec3<f64>, d: f64| {
                    let across = n.cross(hinge);
                    let reach = (major * across.dot(u)).hypot(minor * across.dot(v));
                    Conic {
                        axis: turn(n, hinge, d / reach),
                        u_ref: turn(u, hinge, d / reach),
                        ..e
                    }
                };
                let pose = |what: &str, d: f64| match what {
                    "centre in the plane" => Conic {
                        center: e.center + (u + v).normalize() * d,
                        ..e
                    },
                    "centre off the plane" => Conic {
                        center: e.center + n * d,
                        ..e
                    },
                    "major" => Conic {
                        semi_u: major + d,
                        ..e
                    },
                    "minor" => Conic {
                        semi_v: minor - d,
                        ..e
                    },
                    "tilt about û" => tilt(u, d),
                    "tilt about v̂" => tilt(v, d),
                    "tilt about û + v̂" => tilt((u + v).normalize(), d),
                    "turn" => Conic {
                        u_ref: turn(u, n, d / (major - minor)),
                        ..e
                    },
                    other => panic!("no pose {other}"),
                };
                let mut poses = vec![
                    "centre in the plane",
                    "centre off the plane",
                    "major",
                    "minor",
                    "tilt about û",
                    "tilt about v̂",
                    "tilt about û + v̂",
                ];
                if ratio > 1.1 {
                    poses.push("turn");
                }
                for what in poses {
                    for (d, want) in [
                        (0.6 * z, Some(true)),
                        (1.5 * z, None),
                        (2.0 * b.escalate(), Some(false)),
                    ] {
                        let f = pose(what, d);
                        let stored = [
                            f,
                            Conic {
                                u_ref: f.v_ref(),
                                semi_u: f.semi_v,
                                semi_v: f.semi_u,
                                ..f
                            },
                            Conic { axis: -f.axis, ..f },
                        ];
                        for g in stored {
                            let got = read(&e, &g);
                            let fits = match want {
                                None => got != Some(true),
                                _ => got == want,
                            };
                            assert!(
                                fits,
                                "a/b = {ratio}, b = {size}, {what} by {} z: {got:?}",
                                d / z
                            );
                        }
                    }
                }
                if ratio == 1.0 {
                    for angle in [0.3, 2.0] {
                        let f = Conic {
                            u_ref: turn(u, n, angle),
                            ..e
                        };
                        assert_eq!(read(&e, &f), Some(true), "a circle turned {angle}");
                    }
                }
            }
        }
    }

    /// **Two of a sphere's circles the band reads coplanar are one only
    /// within it.** The unit sphere's great circle in `z = 0`, against
    /// its section by that plane turned `θ = k·z` about the line
    /// `x = 0.9` (`z` the band's zero): the two cross at
    /// `(0.9, ±√0.19, 0)` and lie about `1.9·θ` apart at `x = −1`. The
    /// planes' tilt (`0.9·θ` over the radius), the section's offset from
    /// `z = 0` and its centre's from the origin (each about `0.81·θ`) all
    /// read zero at `k = 0.9`, yet the circles are in band of one
    /// another, and the pair escalates; at `k = 0.2` they are one within
    /// the band; at `k = 30` they definitely cross, and both crossings
    /// are candidates.
    #[test]
    fn two_sphere_sections_read_coplanar_are_one_circle_only_within_the_band() {
        let b = band();
        let z = b.zero();
        let great = circle([0.0; 3], [0.0, 0.0, 1.0], 1.0);
        let section = |theta: f64| {
            let n = Vec3::new(-theta.sin(), 0.0, theta.cos());
            let centre = n * (-0.9 * theta.sin());
            circle(
                centre.to_array(),
                n.to_array(),
                (1.0 - (0.9 * theta.sin()).powi(2)).sqrt(),
            )
        };
        for (x, y) in [(&great, &section(0.9 * z)), (&section(0.9 * z), &great)] {
            assert!(
                matches!(
                    meet(x, y, b),
                    Err(crate::boolean::BooleanError::Escalated { .. })
                ),
                "k = 0.9: in band of one circle: {:?}",
                meet(x, y, b)
            );
        }
        assert_eq!(
            meet(&great, &section(0.2 * z), b).unwrap().map(|v| v.len()),
            Some(0)
        );
        let crossing = section(30.0 * z);
        let got = meet(&great, &crossing, b).unwrap().unwrap();
        let h = 0.19f64.sqrt();
        for w in [[0.9, h, 0.0], [0.9, -h, 0.0]] {
            assert!(
                got.iter().any(|p| p.distance(Point3::from_array(w)) < 1e-6),
                "k = 30: {w:?} in {got:?}"
            );
        }
    }

    /// **The common line meets an ellipse by its gap from tangency, in
    /// metres.** An ellipse of semi-axes `a` and `b` in `z = 0`, against
    /// an ellipse whose plane, tilted 45°, cuts `z = 0` along `y = b + δ`:
    /// with `z` the band's zero threshold, `δ = 100·z` misses,
    /// `δ = −100·z` crosses twice, and `δ = z/2` and `0.75·z` touch at
    /// `(0, b, 0)`. Read at `(a, b) = (4, 2)` and `(40, 0.5)`: the
    /// discriminant is `−2a²b²·b·δ` there, so a margin that did not
    /// divide out `2a²b²·√qa` reads the touch as a miss, and one
    /// overstated twice reads the `0.75·z` touch in band.
    #[test]
    fn the_common_line_meets_an_ellipse_at_its_gap_from_tangency() {
        let b = band();
        let z = b.zero();
        for (major, minor) in [(4.0, 2.0), (40.0, 0.5)] {
            let e = ellipse([0.0; 3], [0.0, 0.0, 1.0], major, minor);
            let partner =
                |delta: f64| ellipse([0.0, minor + delta, 0.0], [0.0, 1.0, 1.0], 0.6, 0.5);
            let got = |delta: f64| meet(&e, &partner(delta), b).unwrap().unwrap();
            let label = format!("({major}, {minor})");
            assert!(
                got(100.0 * z).is_empty(),
                "{label}: a miss: {:?}",
                got(100.0 * z)
            );
            assert_eq!(
                got(-100.0 * z).len(),
                2,
                "{label}: a crossing: {:?}",
                got(-100.0 * z)
            );
            for delta in [z / 2.0, 0.75 * z] {
                let touch = got(delta);
                assert!(
                    touch.len() == 1 && touch[0].distance(Point3::new(0.0, minor, 0.0)) < 1e-6,
                    "{label}, δ = {delta:e}: a touch at the minor vertex: {touch:?}"
                );
            }
        }
    }

    /// **A line's tilt against an ellipse's plane is read over the
    /// larger of the line's edge and the ellipse's semi-axis**, in
    /// either order. The ellipse of semi-axes `2` and `1` in `z = 0` and
    /// one-metre edges: a line through its major vertex `(2, 0, 0)`
    /// sloping `8·z` carries `16·z` across the semi-axis, so it is read
    /// transverse and its crossing is a candidate. A line `100·z` above
    /// the plane at its edge's point, sloping `z/100`, is read parallel
    /// and misses.
    #[test]
    fn a_lines_tilt_against_an_ellipses_plane_is_read_over_the_larger_reach() {
        let b = band();
        let z = b.zero();
        let e = ellipse([0.0; 3], [0.0, 0.0, 1.0], 2.0, 1.0);
        let steep = Curve3::Line {
            origin: Point3::new(2.0, 0.0, 0.0),
            dir: Vec3::new(1.0, 0.0, 8.0 * z).normalize(),
        };
        let flat = line([0.0, 0.0, 100.0 * z], [1.0, 0.0, z / 100.0]);
        for (x, y, order) in [(&e, &steep, "swept ellipse"), (&steep, &e, "swept line")] {
            let got = meet(x, y, b).unwrap().unwrap();
            assert!(
                holds(&got, [2.0, 0.0, 0.0]),
                "{order}: read transverse: {got:?}"
            );
        }
        for (x, y, order) in [(&e, &flat, "swept ellipse"), (&flat, &e, "swept line")] {
            assert_eq!(
                meet(x, y, b).unwrap().map(|v| v.len()),
                Some(0),
                "{order}: read parallel and off the plane"
            );
        }
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod crossing_rows {
    //! [`super::boundary_crossing`] against a face whose boundary vertices
    //! cannot name the meeting: a tilted circle through the interior of a
    //! wall sheet's rim, which no vertex projects onto. (Coaxial
    //! iso-curves are found by either kind of candidate; this is the row
    //! that needs the closed forms.)
    use super::{BoundaryCrossing, boundary_crossing};
    use crate::boolean::Operand;
    use crate::boolean::contain::FaceContainment;
    use crate::test_support_fixtures::{CylFrame, cyl_wall_sheet};
    use geom::Curve3;
    use geom_core::{Band, Point3, Tol, Vec3};

    #[test]
    fn a_tilted_circle_meets_a_rim_no_vertex_projects_onto() {
        let tol = Tol::witness();
        let band = Band::linear(tol).unwrap();
        let mut body = crate::body::Body::<f64>::new();
        let face = cyl_wall_sheet(
            &mut body,
            CylFrame::canonical(1.0),
            (0.2, 1.4),
            (0.0, 1.0),
            tol,
        );
        // The rim point at azimuth 0.8, and a circle of radius 0.3 through
        // it whose plane holds the radial there and is tilted off both
        // the rim's plane and the rulings' direction.
        let a: f64 = 0.8;
        let radial = Vec3::new(a.cos(), a.sin(), 0.0);
        let normal = (Vec3::new(-a.sin(), a.cos(), 0.0) + Vec3::unit_z() * 0.4).normalize();
        let p = Point3::origin() + radial;
        let out = radial * 0.6 + normal.cross(radial) * 0.8;
        let tilted = Curve3::Circle {
            center: p + out * 0.3,
            axis: normal,
            radius: 0.3,
            u_ref: -out,
        };
        match boundary_crossing(&body, Operand::B, face, &tilted, (-0.5, 0.5), band).unwrap() {
            BoundaryCrossing::At { t, p: q, at, .. } => {
                assert!(t.abs() < 1e-12, "the meeting is at the span's middle: {t}");
                assert!(q.distance(p) < 1e-12, "at the rim point: {q:?}");
                assert!(
                    matches!(at, FaceContainment::OnEdge(_)),
                    "on the rim edge: {at:?}"
                );
            }
            other => panic!("the rim meeting must be found: {other:?}"),
        }
    }

    /// **A swept ellipse finds the ruling and the rim it crosses.** The
    /// section of the wall sheet's cylinder by `z = 0.5 + 0.8·x`, whose
    /// parameter is the azimuth: from `1.0` to `1.6` it leaves the sheet
    /// across the ruling at azimuth `1.4`, from `0.5` to `1.2` it enters
    /// across the top rim where `cos θ = 0.625`, and from `1.0` to `1.3`
    /// it stays inside.
    #[test]
    fn a_swept_ellipse_meets_the_ruling_and_the_rim_it_crosses() {
        let tol = Tol::witness();
        let band = Band::linear(tol).unwrap();
        let mut body = crate::body::Body::<f64>::new();
        let face = cyl_wall_sheet(
            &mut body,
            CylFrame::canonical(1.0),
            (0.2, 1.4),
            (0.0, 1.0),
            tol,
        );
        let k = 1.64f64.sqrt();
        let section = Curve3::Ellipse {
            center: Point3::new(0.0, 0.0, 0.5),
            axis: Vec3::new(-0.8, 0.0, 1.0) * (1.0 / k),
            semi_u: k,
            semi_v: 1.0,
            u_ref: Vec3::new(1.0, 0.0, 0.8) * (1.0 / k),
        };
        let crossing = |span| match boundary_crossing(&body, Operand::B, face, &section, span, band)
            .unwrap()
        {
            BoundaryCrossing::At { t, p, at, .. } => {
                assert!(
                    p.distance(section.eval(t)) < 1e-12,
                    "{span:?}: on the ellipse"
                );
                assert!(
                    matches!(at, FaceContainment::OnEdge(_)),
                    "{span:?}: on an edge: {at:?}"
                );
                Some(t)
            }
            BoundaryCrossing::Clear => None,
            BoundaryCrossing::Unread => panic!("{span:?}: every carrier here is read"),
        };
        let at_ruling = crossing((1.0, 1.6)).expect("the ruling is crossed");
        assert!(
            (at_ruling - 1.4).abs() < 1e-12,
            "at the ruling: {at_ruling}"
        );
        let at_rim = crossing((0.5, 1.2)).expect("the rim is crossed");
        assert!(
            (at_rim - 0.625f64.acos()).abs() < 1e-12,
            "at the rim: {at_rim}"
        );
        assert_eq!(crossing((1.0, 1.3)), None, "inside the sheet");
    }

    /// **A swept span is metred by its carrier's speed at the meeting.**
    /// The unit cylinder's section by a plane of slope `m = 400` through
    /// `(cos θ*, sin θ*, 0.5)`, `θ* = 0.05`, whose parameter is the
    /// azimuth: its speed runs from `1` (the minor semi-axis) to about
    /// `400` (the major), and is about `20` at `θ*`, where it crosses the
    /// wall sheet's ruling. With `z` the band's zero, a span starting
    /// `0.8·z` of parameter before `θ*` carries the meeting `16·z` inside
    /// it, and finds it; read at the minor semi-axis that gap is `0.8·z`,
    /// the span's end. A span starting `0.04·z` before `θ*` carries it
    /// `0.8·z` in, at its end, and is clear; read at the major semi-axis
    /// that gap is `16·z`, inside.
    #[test]
    fn a_swept_span_is_metred_by_its_speed_at_the_meeting() {
        let tol = Tol::witness();
        let band = Band::linear(tol).unwrap();
        let z = band.zero();
        let theta: f64 = 0.05;
        let mut body = crate::body::Body::<f64>::new();
        let face = cyl_wall_sheet(
            &mut body,
            CylFrame::canonical(1.0),
            (theta, 1.0),
            (0.0, 1.0),
            tol,
        );
        let m = 400.0f64;
        let k = m.hypot(1.0);
        let section = Curve3::Ellipse {
            center: Point3::new(0.0, 0.0, 0.5 - m * theta.cos()),
            axis: Vec3::new(-m, 0.0, 1.0) * (1.0 / k),
            semi_u: k,
            semi_v: 1.0,
            u_ref: Vec3::new(1.0, 0.0, m) * (1.0 / k),
        };
        let speed = geom_brep::carrier_speed::speed_at(&section, theta).unwrap();
        assert!((speed - 20.0).abs() < 0.1, "about 20 at θ*: {speed}");
        let read = |gap: f64| {
            boundary_crossing(
                &body,
                Operand::B,
                face,
                &section,
                (theta - gap * z, theta + 0.01),
                band,
            )
            .unwrap()
        };
        match read(0.8) {
            BoundaryCrossing::At { t, at, .. } => {
                assert!((t - theta).abs() < 1e-12, "at the ruling: {t}");
                assert!(matches!(at, FaceContainment::OnEdge(_)), "on it: {at:?}");
            }
            other => panic!("16·z inside the span, the meeting is found: {other:?}"),
        }
        assert!(
            matches!(read(0.04), BoundaryCrossing::Clear),
            "0.8·z from the span's start, the meeting is its end: {:?}",
            read(0.04)
        );
    }

    /// A line along a boundary ruling that a vertex divides: parallel to
    /// the ruling, it has no closed-form meeting with it, and its span
    /// stops short of the rims, so only the vertex names the meeting.
    #[test]
    fn a_line_along_a_divided_ruling_meets_its_vertex() {
        let tol = Tol::witness();
        let band = Band::linear(tol).unwrap();
        let mut body = crate::body::Body::<f64>::new();
        let face = cyl_wall_sheet(
            &mut body,
            CylFrame::canonical(1.0),
            (0.2, 1.4),
            (0.0, 1.0),
            tol,
        );
        let a: f64 = 0.2;
        let foot = Point3::new(a.cos(), a.sin(), 0.0);
        let (ruling, origin) = body
            .edges()
            .find_map(|(k, e)| {
                let c = body.get_curve_geom(e.curve)?.certified()?;
                match *c.carrier() {
                    Curve3::Line { origin, dir }
                        if dir.z.abs() > 0.9
                            && Point3::new(origin.x, origin.y, 0.0).distance(foot) < 1e-12 =>
                    {
                        Some((k, origin))
                    }
                    _ => None,
                }
            })
            .expect("the ruling at azimuth 0.2");
        let dir = match *body
            .get_curve_geom(body.get_edge(ruling).unwrap().curve)
            .unwrap()
            .certified()
            .unwrap()
            .carrier()
        {
            Curve3::Line { dir, .. } => dir,
            _ => panic!("a ruling is a line"),
        };
        let mid = body
            .split_edge(ruling, (0.5 - origin.z) / dir.z, tol)
            .unwrap()
            .vertex;
        let along = Curve3::Line {
            origin: foot,
            dir: Vec3::unit_z(),
        };
        match boundary_crossing(&body, Operand::B, face, &along, (0.25, 0.75), band).unwrap() {
            BoundaryCrossing::At { t, at, .. } => {
                assert!((t - 0.5).abs() < 1e-12, "at the dividing vertex: {t}");
                assert_eq!(at, FaceContainment::OnVertex(mid), "the vertex itself");
            }
            other => panic!("the dividing vertex must be found: {other:?}"),
        }
    }

    /// **A torn hop past the face panics; the face itself stays the
    /// caller's.** The tilted-circle row with one boundary edge's curve
    /// dropped panics naming that curve, where a read of the miss as a
    /// carrier with no closed form would answer `Unread`; a face that
    /// does not resolve refuses typed.
    #[test]
    fn a_torn_boundary_curve_panics_and_a_stale_face_refuses_typed() {
        use crate::entity::{EntityId, GeomRef};
        use crate::live::OPERATORS_KEEP_LINKS;
        use crate::review_d18::{ROW_FOUR, assert_torn_op_panics};
        let tol = Tol::witness();
        let band = Band::linear(tol).unwrap();
        let mut body = crate::body::Body::<f64>::new();
        let face = cyl_wall_sheet(
            &mut body,
            CylFrame::canonical(1.0),
            (0.2, 1.4),
            (0.0, 1.0),
            tol,
        );
        let a: f64 = 0.8;
        let radial = Vec3::new(a.cos(), a.sin(), 0.0);
        let normal = (Vec3::new(-a.sin(), a.cos(), 0.0) + Vec3::unit_z() * 0.4).normalize();
        let out = radial * 0.6 + normal.cross(radial) * 0.8;
        let tilted = Curve3::Circle {
            center: Point3::origin() + radial + out * 0.3,
            axis: normal,
            radius: 0.3,
            u_ref: -out,
        };
        assert!(
            matches!(
                boundary_crossing(&body, Operand::B, face, &tilted, (-0.5, 0.5), band),
                Ok(BoundaryCrossing::At { .. })
            ),
            "the sound face meets the circle"
        );
        let mut stale = body.clone();
        stale.faces.remove(face);
        assert!(
            matches!(
                boundary_crossing(&stale, Operand::B, face, &tilted, (-0.5, 0.5), band),
                Err(crate::boolean::BooleanError::ClassificationInvariant { .. })
            ),
            "a face the caller passed that does not resolve refuses typed"
        );
        let mut torn = body.clone();
        let named = crate::review_d18::tear_ring(&mut torn, face);
        assert_torn_op_panics(
            "boundary_crossing (ring)",
            &mut torn,
            &[&named, ROW_FOUR, OPERATORS_KEEP_LINKS],
            |b| boundary_crossing(b, Operand::B, face, &tilted, (-0.5, 0.5), band),
        );
        // A member's start, on a span no candidate falls strictly inside:
        // no containment read follows, so the start is the only read that
        // passes through it.
        let clear = (0.4, 0.401);
        assert!(
            matches!(
                boundary_crossing(&body, Operand::B, face, &tilted, clear, band),
                Ok(BoundaryCrossing::Clear)
            ),
            "the sound face is clear of the circle there"
        );
        let mut torn = body.clone();
        let (vertex, _) = torn.vertices().next().expect("the sheet has vertices");
        torn.vertices.remove(vertex);
        let named = format!("'s start names {}", EntityId::Vertex(vertex));
        assert_torn_op_panics(
            "boundary_crossing (start)",
            &mut torn,
            &[&named, ROW_FOUR, OPERATORS_KEEP_LINKS],
            |b| boundary_crossing(b, Operand::B, face, &tilted, clear, band),
        );
        let (edge, curve) = body
            .edges()
            .map(|(k, e)| (k, e.curve))
            .next()
            .expect("the sheet has edges");
        body.curves.remove(curve);
        let named = format!(
            "{}'s curve names {}",
            EntityId::Edge(edge),
            GeomRef::Curve(curve)
        );
        assert_torn_op_panics(
            "boundary_crossing",
            &mut body,
            &[&named, ROW_FOUR, OPERATORS_KEEP_LINKS],
            |b| boundary_crossing(b, Operand::B, face, &tilted, (-0.5, 0.5), band),
        );
    }
}
