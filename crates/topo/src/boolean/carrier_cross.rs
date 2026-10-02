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
//! [`boundary_crossing`] finds the first such meeting along the edge, so
//! the sweep can split both edges there exactly as it splits a wall
//! pierce that lands on a boundary edge. The candidate points are closed
//! form per carrier pair — every vertex of the face's boundary, and the
//! transverse meetings of the edge's carrier with each boundary edge's
//! carrier — and each is then asked the two questions the sweep already
//! has rows for: does it lie strictly inside the edge's span, and does the
//! face's boundary pre-pass put it ON the boundary. A candidate that is
//! not a meeting fails the second question, so the generation only has to
//! be complete, never exact.
//!
//! **Complete over line and circle carriers.** Two of them meet either at
//! a point the pair's closed form names (a line through a circle's plane;
//! two non-parallel lines; two circles whose planes cross, along the
//! planes' common line; two circles in one plane), or along a shared
//! stretch whose ends are vertices
//! of one of them — the boundary vertices, which are always candidates.
//! A spiric or spline carrier on either side has no closed form here: it
//! answers [`BoundaryCrossing::Unread`], and the caller keeps its
//! frontier door.

use geom_core::{Band, Decide, Margin, Point3, Sign, Vec3};

use super::contain::FaceContainment;
use super::{BooleanDecision, BooleanError, CrossingDecision, Operand};
use crate::body::Body;
use crate::entity::{FaceKey, LoopBoundary};
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
    },
    /// Certified: the edge's interior meets the face's boundary nowhere.
    Clear,
    /// A carrier pair this module has no closed form for: nothing is
    /// known about the interior.
    Unread,
}

/// A point strictly inside `curve`'s span where it meets `face`'s
/// boundary. `curve` must lie on `face`'s
/// carrier — the caller's certificate, which this function does not
/// re-ask.
///
/// # Errors
///
/// [`BooleanError::Escalated`] when a decision falls in band: whether
/// a candidate lies inside the span, whether two carriers are parallel,
/// or the boundary pre-pass's own rows.
pub(super) fn boundary_crossing<T: Decide>(
    y: &Body<T>,
    y_is: Operand,
    face: FaceKey,
    curve: &geom_brep::EdgeCurve<T>,
    band: Band,
) -> Result<BoundaryCrossing<T>, BooleanError> {
    let carrier = curve.carrier();
    let (t0, t1) = curve.params();
    let mid = geom::mid_param(t0, t1);
    // A parameter step as arc length, so every span decision is in metres.
    let metres_per_param = match *carrier {
        geom::Curve3::Line { .. } => T::one(),
        geom::Curve3::Circle { radius, .. } => radius,
        _ => return Ok(BoundaryCrossing::Unread),
    };
    let face_data = y
        .get_face(face)
        .ok_or(BooleanError::ClassificationInvariant {
            what: "on-carrier crossing: face lost",
        })?;
    let mut candidates: Vec<Point3<T>> = Vec::new();
    for lk in core::iter::once(face_data.outer).chain(face_data.rings.iter().copied()) {
        let first = match y.get_loop(lk).map(|l| l.boundary) {
            Some(LoopBoundary::Cycle { first }) => first,
            // A lone vertex is boundary too, and a candidate like any
            // other.
            Some(LoopBoundary::Empty { vertex }) => {
                let p = y
                    .get_vertex(vertex)
                    .and_then(|v| y.get_point(v.point))
                    .ok_or(BooleanError::ClassificationInvariant {
                        what: "on-carrier crossing: ring vertex lost",
                    })?;
                candidates.push(*p);
                continue;
            }
            None => {
                return Err(BooleanError::ClassificationInvariant {
                    what: "on-carrier crossing: boundary loop lost",
                });
            }
        };
        let cycle = y
            .loop_cycle(first)
            .ok_or(BooleanError::ClassificationInvariant {
                what: "on-carrier crossing: boundary loop does not close",
            })?;
        for he in cycle {
            let half = y
                .get_half_edge(he)
                .ok_or(BooleanError::ClassificationInvariant {
                    what: "on-carrier crossing: half-edge lost",
                })?;
            if let Some(p) = y.get_vertex(half.start).and_then(|v| y.get_point(v.point)) {
                candidates.push(*p);
            }
            let Some(CurveGeom::Certified(other)) = y
                .get_edge(half.edge)
                .and_then(|e| y.get_curve_geom(e.curve))
            else {
                return Ok(BoundaryCrossing::Unread);
            };
            match meetings(carrier, other.carrier(), band)? {
                Some(points) => candidates.extend(points),
                None => return Ok(BoundaryCrossing::Unread),
            }
        }
    }
    // The first meeting in the boundary's walk order: the sweep splits
    // there and re-examines both fragments against this face, so every
    // other meeting is found again on them.
    for q in candidates {
        let Some(t) = carrier.param_near(q, mid) else {
            return Ok(BoundaryCrossing::Unread);
        };
        if !strictly_inside(t, t0, t1, metres_per_param, band)? {
            continue;
        }
        let p = carrier.eval(t);
        if let Some(at) = super::contain::curved_boundary_containment(y, face, p, band)
            .map_err(|e| super::reduce::esc(e, y_is))?
        {
            return Ok(BoundaryCrossing::At { t, p, at });
        }
    }
    Ok(BoundaryCrossing::Clear)
}

/// Whether `t` lies strictly inside `[t0, t1]`, each gap metred as arc
/// length. An end is not inside: an endpoint's incidence is the
/// endpoint arms' question.
fn strictly_inside<T: Decide>(
    t: T,
    t0: T,
    t1: T,
    metres_per_param: T,
    band: Band,
) -> Result<bool, BooleanError> {
    let mut inside = true;
    for gap in [t - t0, t1 - t] {
        match decide(
            "bool_carrier_cross_in_span",
            Margin::of(gap * metres_per_param),
            band,
        ) {
            Ok(Sign::Positive) => {}
            Ok(Sign::Zero | Sign::Negative) => inside = false,
            Err(diag) => {
                return Err(BooleanError::Escalated {
                    decision: BooleanDecision::Crossing(CrossingDecision::OnEdge),
                    diag,
                });
            }
        }
    }
    Ok(inside)
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
        Err(diag) => Err(BooleanError::Escalated {
            decision: BooleanDecision::Crossing(CrossingDecision::OnEdge),
            diag,
        }),
    }
}

/// The isolated points where carriers `a` and `b` can meet, beyond the
/// boundary vertices the caller already holds: empty where they meet
/// only along a shared stretch or not at all, `None` for a pair with no
/// closed form here.
fn meetings<T: Decide>(
    a: &geom::Curve3<T>,
    b: &geom::Curve3<T>,
    band: Band,
) -> Result<Option<Vec<Point3<T>>>, BooleanError> {
    use geom::Curve3::{Circle, Line};
    Ok(Some(match (a, b) {
        (
            &Line { origin, dir },
            &Circle {
                center,
                axis,
                radius,
                ..
            },
        )
        | (
            &Circle {
                center,
                axis,
                radius,
                ..
            },
            &Line { origin, dir },
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
        ) => {
            let n = d1.cross(d2);
            let s = n.norm();
            // Parallel lines meet along a stretch whose ends are
            // boundary vertices, or nowhere. The lever is unit: a sine
            // over a metre, the scale the span rows read.
            if !transverse(s, T::one(), band)? {
                return Ok(Some(Vec::new()));
            }
            let t = (o2 - o1).cross(d2).dot(n) / s.powi(2);
            vec![o1 + d1 * t]
        }
        (
            &Circle {
                center: c1,
                axis: n1,
                radius: r1,
                ..
            },
            &Circle {
                center: c2,
                axis: n2,
                radius: r2,
                ..
            },
        ) => {
            let m = n1.cross(n2);
            let s = m.norm();
            if !transverse(s, r1.max(r2), band)? {
                return Ok(Some(parallel_circles(c1, n1, r1, c2, r2, band)?));
            }
            // The planes' common line, through the point of it nearest
            // the origin of the two-plane system, then its meetings with
            // the first circle's sphere — which, in its plane, are its
            // meetings with the circle.
            let (h1, h2) = (n1.dot(c1 - Point3::origin()), n2.dot(c2 - Point3::origin()));
            let c = n1.dot(n2);
            let k = s.powi(2);
            let p0 = Point3::origin() + (n1 * ((h1 - h2 * c) / k) + n2 * ((h2 - h1 * c) / k));
            let u = m * (T::one() / s);
            let w = p0 - c1;
            let b = w.dot(u);
            let disc = b.powi(2) - (w.dot(w) - r1.powi(2));
            match decide(
                "bool_carrier_cross_disc",
                Margin::levered_inv(disc, r1 + r1),
                band,
            ) {
                Ok(Sign::Negative) => Vec::new(),
                Ok(Sign::Zero) => vec![p0 + u * (-b)],
                Ok(Sign::Positive) => {
                    let root = disc.sqrt();
                    vec![p0 + u * (-b - root), p0 + u * (-b + root)]
                }
                Err(diag) => {
                    return Err(BooleanError::Escalated {
                        decision: BooleanDecision::Crossing(CrossingDecision::OnEdge),
                        diag,
                    });
                }
            }
        }
        _ => return Ok(None),
    }))
}

/// Two circles in parallel planes: none where the planes are distinct
/// or the circles share a centre (one circle, or nested ones), else
/// the meetings of two circles in one plane — two, one where they
/// touch, none where they are apart or one holds the other.
fn parallel_circles<T: Decide>(
    c1: Point3<T>,
    n1: Vec3<T>,
    r1: T,
    c2: Point3<T>,
    r2: T,
    band: Band,
) -> Result<Vec<Point3<T>>, BooleanError> {
    let escalate = |diag| BooleanError::Escalated {
        decision: BooleanDecision::Crossing(CrossingDecision::OnEdge),
        diag,
    };
    let sign = |row, margin| decide(row, margin, band).map_err(escalate);
    if sign(
        "bool_carrier_cross_plane_offset",
        Margin::of((c2 - c1).dot(n1)),
    )? != Sign::Zero
    {
        return Ok(Vec::new());
    }
    let w = c2 - c1;
    if sign("bool_carrier_cross_concentric", Margin::norm3(w))? == Sign::Zero {
        return Ok(Vec::new());
    }
    let d = w.norm();
    // Apart, or one inside the other: no meeting. Touching: one.
    let apart = sign("bool_carrier_cross_disc", Margin::of(r1 + r2 - d))?;
    let nested = sign("bool_carrier_cross_disc", Margin::of(d - (r1 - r2).abs()))?;
    if apart == Sign::Negative || nested == Sign::Negative {
        return Ok(Vec::new());
    }
    let e = w * (T::one() / d);
    let a = (d.powi(2) + r1.powi(2) - r2.powi(2)) / (d + d);
    let foot = c1 + e * a;
    if apart == Sign::Zero || nested == Sign::Zero {
        return Ok(vec![foot]);
    }
    let h = (r1.powi(2) - a.powi(2)).sqrt();
    let f = n1.cross(e);
    Ok(vec![foot + f * h, foot - f * h])
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod meetings_rows {
    //! The closed forms behind [`super::meetings`]: every isolated meeting
    //! of two carriers is among the candidates, a pair that meets only
    //! along a stretch or nowhere has none, and a pair with no closed
    //! form says so.
    use super::meetings;
    use geom::Curve3;
    use geom_core::{Band, Point3, Tol, Vec3};

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
            let got = meetings(a, c, b).unwrap().unwrap();
            assert!(holds(&got, [0.5, 1.0, 0.0]), "ruling × rim: {got:?}");
        }
        // Two skew-free lines crossing.
        let got = meetings(
            &line([0.0, 0.0, 0.0], [1.0, 0.0, 0.0]),
            &line([1.0, -1.0, 0.0], [0.0, 1.0, 0.0]),
            b,
        )
        .unwrap()
        .unwrap();
        assert!(holds(&got, [1.0, 0.0, 0.0]), "line × line: {got:?}");
        // Two great circles of the unit sphere meet at two antipodes.
        let got = meetings(
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
        let got = meetings(
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
        let got = meetings(
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

    #[test]
    fn a_stretch_or_a_miss_has_no_candidate_and_a_parallel_line_is_unread() {
        let b = band();
        let none = |a: &Curve3<f64>, c: &Curve3<f64>| meetings(a, c, b).unwrap().map(|v| v.len());
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
}
