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
//! meets the boundary nowhere, with both ends outside, lies outside.
//!
//! **It reads, it does not solve.** The edge and each boundary edge are
//! two carrier spans, and whether and where they meet is
//! [`geom_brep::curve_pair::meet`]'s one reading. A meeting it places on
//! the edge's interior is a crossing; one at the edge's end is the
//! endpoint arms' question. Where the two read as one curve, the shared
//! run's ends are vertices of the boundary or the edge's own ends, so a
//! boundary vertex the point-on-span reading ([`Span::contact`]) puts on
//! the edge's interior is a crossing too. Each crossing is then placed on
//! the boundary entity it lands on by the face's boundary pre-pass, which
//! names that entity; a crossing the pre-pass puts off every boundary
//! entity is two readings that disagree, and answers
//! [`BoundaryCrossing::Unread`]. So does every refusal `meet` types past
//! the band's own escalation, and a face holding a lone vertex (a pierce
//! ring), which has no boundary pre-pass: the caller keeps its frontier
//! door. A refusal's decided meetings are read like an answer's: a
//! crossing among them is split, and the fragments are read again.

use geom_brep::curve_pair::{Contact, Hit, MeetFault, Span, meet};
use geom_core::{Band, Decide, Point3};

use super::contain::FaceContainment;
use super::{BooleanDecision, BooleanError, CrossingDecision, Operand};
use crate::body::Body;
use crate::entity::{EntityId, FaceKey};
use crate::live::BoundaryMember;
use crate::null::CurveGeom;

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
    /// No reading answers: nothing is known about the interior (module
    /// docs).
    Unread,
}

/// A point strictly inside `carrier`'s span `(t0, t1)` where it meets
/// `face`'s boundary. The span must lie on `face`'s carrier — the
/// caller's certificate, which this function does not re-ask.
///
/// # Errors
///
/// [`BooleanError::Escalated`] when a decision falls in band: where a
/// boundary vertex sits against the span, a decision of the meeting
/// reading, or the boundary pre-pass's own rows.
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
    let swept = Span::new(carrier, (t0, t1));
    let face_data = y
        .get_face(face)
        .ok_or(BooleanError::ClassificationInvariant {
            what: "on-carrier crossing: face lost",
        })?;
    // Each crossing's parameter on the edge.
    let mut crossings: Vec<T> = Vec::new();
    for member in y.face_boundary_linked(face, face_data) {
        let BoundaryMember::Edge { he, half, ek, edge } = member else {
            return Ok(BoundaryCrossing::Unread);
        };
        let vertex = y.linked_vertex_point(half.start, EntityId::HalfEdge(he), "start");
        let Some(t) = carrier.param_near(vertex, mid) else {
            return Ok(BoundaryCrossing::Unread);
        };
        match swept.contact(vertex, t, band).map_err(escalated)? {
            Contact::On { t, .. } => crossings.push(t),
            Contact::Off | Contact::Carrier | Contact::Past | Contact::End => {}
        }
        let CurveGeom::Certified(other) = y.edge_curve_linked(ek, edge) else {
            return Ok(BoundaryCrossing::Unread);
        };
        let met = match meet(&swept, &Span::new(other.carrier(), other.params()), band) {
            Ok(met) => met,
            // A crossing decided before the refusal is a crossing: the
            // sweep splits there and reads both fragments again.
            Err(refusal)
                if refusal.shared.is_none()
                    && refusal
                        .found
                        .iter()
                        .any(|m| !m.run && matches!(m.on_a, Hit::On { .. })) =>
            {
                crossings.extend(refusal.found.iter().filter(|m| !m.run).filter_map(
                    |m| match m.on_a {
                        Hit::On { t, .. } => Some(t),
                        Hit::End => None,
                    },
                ));
                continue;
            }
            Err(refusal) => {
                return match refusal.fault {
                    MeetFault::Escalated(diag) => Err(escalated(diag)),
                    MeetFault::Unsettled(_) | MeetFault::Unsupported { .. } => {
                        Ok(BoundaryCrossing::Unread)
                    }
                    MeetFault::Desync => Err(BooleanError::ClassificationInvariant {
                        what: "on-carrier crossing: a carrier the meeting table misread",
                    }),
                    MeetFault::Contradiction { .. } => Err(BooleanError::ClassificationInvariant {
                        what: "on-carrier crossing: two readings of the meeting pose contradict",
                    }),
                };
            }
        };
        if met.shared.is_some() {
            continue;
        }
        // A run's points stand beside a meeting at an end, which is the
        // endpoint arms' question: no crossing of their own.
        for m in met.points.iter().filter(|m| !m.run) {
            match m.on_a {
                Hit::On { t, .. } => crossings.push(t),
                Hit::End => {}
            }
        }
    }
    // The first crossing in the boundary's walk order: the sweep splits
    // there and re-examines both fragments against this face, so every
    // other one is found again on them.
    let Some(&t) = crossings.first() else {
        return Ok(BoundaryCrossing::Clear);
    };
    let p = carrier.eval(t);
    Ok(
        match super::contain::curved_boundary_containment(y, face, p, band)
            .map_err(|e| super::reduce::esc(e, y_is, face))?
        {
            Some((at, margin)) => BoundaryCrossing::At { t, p, at, margin },
            None => BoundaryCrossing::Unread,
        },
    )
}

/// An in-band decision of this module: where a crossing lands along its
/// edge is undetermined.
fn escalated(diag: geom_core::Indeterminate) -> BooleanError {
    BooleanError::Escalated {
        decision: BooleanDecision::Crossing(CrossingDecision::OnEdge),
        diag,
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod crossing_rows {
    //! [`super::boundary_crossing`] against a face whose boundary vertices
    //! cannot name the meeting: a tilted circle through the interior of a
    //! wall sheet's rim, which no vertex projects onto. (Coaxial
    //! iso-curves are found by either kind of candidate; this is the row
    //! that needs the meeting reading.)
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

    /// **A swept span's end is read by its chord to the meeting.** The
    /// unit cylinder's section by a plane of slope `m = 400` through
    /// `(cos θ*, sin θ*, 0.5)`, `θ* = 0.05`, whose parameter is the
    /// azimuth: its speed runs from `1` (the minor semi-axis) to about
    /// `400` (the major), and is about `20` at `θ*`, where it crosses the
    /// wall sheet's ruling. With `z` the band's zero, a span starting
    /// `0.8·z` of parameter before `θ*` carries the meeting a chord of
    /// `16·z` inside it, and finds it. A span starting `0.04·z` before
    /// `θ*` carries it a chord of `0.8·z` in, at its end, and is clear.
    /// Read by the parameter at either semi-axis, each would be the
    /// other's answer.
    #[test]
    fn a_swept_spans_end_is_read_by_its_chord_to_the_meeting() {
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

    /// **A swept span crossing a rim at a shallow angle at its own end
    /// crosses nothing inside.** The section of the unit cylinder by
    /// `z = c + ½·cos(θ − 0.8)`, its top `δ = ½(1 − cos 0.01)` above the
    /// sheet's top rim, crosses the rim at its own parameters `±t_c`,
    /// `t_c ≈ 0.01`, at about `0.005` rad. A span from `t_c` inward meets
    /// the rim at its end, which the endpoint arms place. The run of its
    /// points within the band of the rim, about `5·10⁻⁸` long, is no
    /// crossing of its own, so the interior is clear. A span through both
    /// crossings is crossed at one of them.
    ///
    /// (A section tangent to the rim reads `Unread`: across planes the
    /// basin meter refuses a crossing angle of zero,
    /// `work/fuse/curve-pair-polish-meters-refuse-shallow-meetings.md`.)
    #[test]
    fn a_shallow_crossing_at_the_swept_spans_end_crosses_nothing_inside() {
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
        let (m, a) = (0.5f64, 0.8f64);
        let c = 1.0 + m * (1.0 - 0.01f64.cos()) - m;
        let k = m.hypot(1.0);
        let section = Curve3::Ellipse {
            center: Point3::new(0.0, 0.0, c),
            axis: Vec3::new(-m * a.cos(), -m * a.sin(), 1.0) * (1.0 / k),
            semi_u: k,
            semi_v: 1.0,
            u_ref: Vec3::new(a.cos(), a.sin(), m) * (1.0 / k),
        };
        let tc = ((1.0 - c) / m).acos();
        assert!(
            (section.eval(tc).z - 1.0).abs() < 1e-12,
            "t_c is on the top rim"
        );
        let read = |span| boundary_crossing(&body, Operand::B, face, &section, span, band).unwrap();
        for span in [(tc, tc + 0.3), (-tc - 0.3, -tc)] {
            assert!(
                matches!(read(span), BoundaryCrossing::Clear),
                "{span:?}: starts at the crossing: {:?}",
                read(span)
            );
        }
        match read((-0.3, 0.3)) {
            BoundaryCrossing::At { t, at, .. } => {
                assert!((t.abs() - tc).abs() < 1e-6, "at a crossing: {t}");
                assert!(matches!(at, FaceContainment::OnEdge(_)), "{at:?}");
            }
            other => panic!("through both crossings: {other:?}"),
        }
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
    /// dropped panics naming that curve, where a read of the miss as an
    /// unread pair would answer `Unread`; a face that
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
