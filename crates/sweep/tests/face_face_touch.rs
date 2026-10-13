//! A solid touching another, or itself, along a tangent line through
//! two faces' interiors: a face–face record of the two faces, with no
//! edge on the line (`crates/topo/README.md` C7).
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::SurfaceKind;
use geom_core::{Point2, Tol};
use profile::test_support::bulge_loop;
use profile::{Profile, ProfileLoop};
use sweep::ExtrudeSide;
use sweep::test_support::{brick, finished, sketch_at};
use sweep::{Extrusion, extrude};
use topo::boolean::BooleanResult;
use topo::{AtRestBody, Body};

fn tol() -> Tol {
    Tol::witness()
}

/// A circle of radius `r` about `(cx, cy)` with its two vertices at
/// angles `phase` and `phase + π`.
fn circle(cx: f64, cy: f64, r: f64, phase: f64) -> ProfileLoop<f64> {
    let at = |t: f64| Point2::new(cx + r * t.cos(), cy + r * t.sin());
    bulge_loop(vec![
        (at(phase), 1.0),
        (at(phase + core::f64::consts::PI), 1.0),
    ])
}

fn ext(what: &str, loops: Vec<ProfileLoop<f64>>, z0: f64, h: f64) -> AtRestBody<f64> {
    let profile = Profile::new(sketch_at(z0), loops).validate(tol()).unwrap();
    let body: Body<f64> = extrude(
        &profile,
        Extrusion::Distance {
            depth: h,
            side: ExtrudeSide::Along,
        },
        tol(),
    )
    .unwrap()
    .body;
    finished(what, body, tol())
}

fn point(body: &Body<f64>, v: topo::VertexKey) -> [f64; 3] {
    let (_, p) = body.vertex_points().find(|(k, _)| *k == v).unwrap();
    [p.x, p.y, p.z]
}

/// Each record's two cells' carrier kinds, faces only.
fn face_pairs(r: &topo::BooleanBody<f64>) -> Vec<(SurfaceKind, SurfaceKind)> {
    let kind = |f: topo::FaceKey| {
        let face = r.body.get_face(f).unwrap();
        r.body.get_surface(face.surface).unwrap().kind()
    };
    r.contacts
        .face_face()
        .map(|(x, y)| {
            let (p, q) = (kind(x), kind(y));
            if p == SurfaceKind::Plane {
                (p, q)
            } else {
                (q, p)
            }
        })
        .collect()
}

/// A unit disc about (1, 0) inside a disc of radius 2 about the origin,
/// tangent from inside along x = 2, y = 0, each circle's vertices off
/// that line, caps coplanar or the inner disc run past both: every op
/// refuses `CurvedPierceUnsupported`, the outer disc's cap circles
/// touching the inner wall mid-arc. So `A ∖ B`, a solid pinched along
/// the line, is not reached this way.
#[test]
fn a_nested_disc_with_its_vertices_off_the_line_refuses_its_mid_arc_touch() {
    let half_pi = core::f64::consts::FRAC_PI_2;
    let a = ext("fat", vec![circle(0.0, 0.0, 2.0, half_pi)], 0.0, 1.0);
    let b = ext("thin", vec![circle(1.0, 0.0, 1.0, half_pi)], 0.0, 1.0);
    let tall = ext("tall thin", vec![circle(1.0, 0.0, 1.0, half_pi)], -1.0, 3.0);
    for (what, got) in [
        ("A ∖ B", topo::subtract(&a, &b, tol())),
        ("A ∪ B", topo::union(&a, &b, tol())),
        ("B ∖ A", topo::subtract(&b, &a, tol())),
        ("A ∩ B", topo::intersect(&a, &b, tol())),
        ("A ∖ tall B", topo::subtract(&a, &tall, tol())),
    ] {
        assert!(
            matches!(got, Err(topo::BooleanError::CurvedPierceUnsupported { .. })),
            "{what}: {:?}",
            got.map(|_| ())
        );
    }
}

/// A rod resting on a plate's side face along a ruling through both
/// faces' interiors, its vertices off the ruling. The union is two
/// solids carrying the face–face record of the plate's face and the
/// rod's wall. A union with a box bridging the rod's top to the plate's
/// joins them into one solid, and the record, carried as the first
/// union's, lands on the plate's face and the piece of the rod's wall
/// still on it; the tier-3′ pass confirms it. Carried with the first
/// union's edge–face records as well, the bridge refuses: the record
/// of a cap circle touching the plate's face at a point is one the
/// census's confirm cannot read
/// (`work/restread/an-edge-face-record-off-a-line-and-a-plane-has-no-curve-touch-lane.md`).
#[test]
fn a_rod_on_a_plate_records_its_touch_and_a_bridge_lands_it() {
    let pi = core::f64::consts::PI;
    let plate = finished(
        "plate",
        brick::<f64>((-2.0, 2.0), (-2.0, 2.0), (0.0, 1.0), tol()),
        tol(),
    );
    let rod = ext("rod", vec![circle(0.0, 2.5, 0.5, 0.0)], 0.25, 0.5);
    let u = match topo::union(&plate, &rod, tol()) {
        Ok(BooleanResult::Body(u)) => u,
        other => panic!("plate ∪ rod: {:?}", other.map(|_| ())),
    };
    assert_eq!(u.body.solids().count(), 2, "plate ∪ rod: two solids");
    let v = topo::mass_properties(&u.body, tol()).unwrap().volume;
    assert!(
        (v - (16.0 + 0.125 * pi)).abs() < 1e-9,
        "plate ∪ rod: volume {v}"
    );
    assert_eq!(
        face_pairs(&u),
        [(SurfaceKind::Plane, SurfaceKind::Cylinder)],
        "plate ∪ rod: the face–face record of the plate's face and the rod's wall"
    );
    let (side, _) = u.contacts.face_face().next().unwrap();
    let plane = |f: topo::FaceKey| {
        let face = u.body.get_face(f).unwrap();
        u.body.get_surface(face.surface).unwrap().kind() == SurfaceKind::Plane
    };
    let side = if plane(side) {
        side
    } else {
        u.contacts.face_face().next().unwrap().1
    };
    // Each cap circle touches the plate's face at one point inside its
    // arc: an edge–face record of the circle and that face.
    let ef: Vec<_> = u.contacts.edge_face().collect();
    let heights: Vec<f64> = ef
        .iter()
        .map(|&(e, f)| {
            assert_eq!(f, side, "plate ∪ rod: a cap record names the plate's face");
            let zs: Vec<f64> = u
                .body
                .half_edges()
                .filter(|(_, h)| h.edge == e)
                .map(|(_, h)| point(&u.body, h.start)[2])
                .collect();
            assert!(
                zs.windows(2).all(|w| w[0] == w[1]),
                "plate ∪ rod: a cap record's edge lies at one height: {zs:?}"
            );
            zs[0]
        })
        .collect();
    assert_eq!(
        heights,
        [0.25, 0.75],
        "plate ∪ rod: the bottom and top cap circles' edge–face records"
    );
    // The face–face record cites the one row the glue door recorded for
    // its tangency.
    let tangent: Vec<usize> = u
        .coincidences
        .iter()
        .enumerate()
        .filter(|(_, c)| c.relation == topo::Relation::SurfaceTangent)
        .map(|(i, _)| i)
        .collect();
    assert_eq!(
        tangent.len(),
        1,
        "plate ∪ rod: one tangency row: {:?}",
        u.coincidences
    );
    assert_eq!(
        u.coincidences[tangent[0]].site,
        topo::DecisionSite::TangentWitness,
        "plate ∪ rod: the tangency row is the glue door's"
    );
    let cites: Vec<_> = u
        .contacts
        .rows()
        .iter()
        .find(|r| matches!(r.kind(), topo::ContactKind::FaceFace(..)))
        .unwrap()
        .cites
        .iter()
        .collect();
    assert_eq!(
        cites,
        [topo::Backing::Decided(u32::try_from(tangent[0]).unwrap())],
        "plate ∪ rod: the face–face record cites the tangency row"
    );
    let bridge = finished(
        "bridge",
        brick::<f64>((-1.0, 1.0), (1.0, 2.5), (0.6, 1.6), tol()),
        tol(),
    );
    let bridged = |records_a| {
        topo::boolean_op_with(
            topo::BooleanOp::Union,
            &u.body,
            &bridge,
            &topo::BooleanInputs {
                records_a,
                ..topo::BooleanInputs::default()
            },
            topo::SweepStrategy::Realized,
            tol(),
        )
    };
    let ff = topo::OperandRecords {
        ff: u.contacts.carried().ff,
        ..topo::OperandRecords::default()
    };
    match bridged(ff) {
        Ok(BooleanResult::Body(r)) => {
            assert_eq!(r.body.solids().count(), 1, "∪ bridge: one solid");
            let v = topo::mass_properties(&r.body, tol()).unwrap().volume;
            let want = 18.2 + 0.85 * pi / 8.0;
            assert!((v - want).abs() < 1e-9, "∪ bridge: volume {v}, want {want}");
            assert_eq!(
                face_pairs(&r),
                [(SurfaceKind::Plane, SurfaceKind::Cylinder)],
                "∪ bridge: the record lands on the plate's face and the rod's wall"
            );
            assert_eq!(
                topo::validate_pseudomanifold(&r.body, &r.contacts, tol()),
                Ok(()),
                "∪ bridge: tier 3′ with the landed record"
            );
        }
        other => panic!("∪ bridge: {:?}", other.map(|_| ())),
    }
    assert!(
        matches!(
            bridged(u.contacts.carried()),
            Err(topo::BooleanError::CarryRefused {
                refusal: topo::CarryRefusal::Unread { .. },
                ..
            })
        ),
        "∪ bridge carrying every record: the cap circle's edge–face record is unread"
    );
}

/// **A carried edge–face or face–face row is validated at the door**,
/// as its siblings are: a key that does not resolve, a face–face row
/// naming one face twice, and an edge–face row whose edge bounds its
/// face (no record: their interiors cannot meet) each refuse there.
#[test]
fn a_carried_curve_touch_row_with_a_bad_key_refuses_at_the_door() {
    let plate = finished(
        "plate",
        brick::<f64>((-2.0, 2.0), (-2.0, 2.0), (0.0, 1.0), tol()),
        tol(),
    );
    let rod = ext("rod", vec![circle(0.0, 2.5, 0.5, 0.0)], 0.25, 0.5);
    let u = match topo::union(&plate, &rod, tol()) {
        Ok(BooleanResult::Body(u)) => u,
        other => panic!("plate ∪ rod: {:?}", other.map(|_| ())),
    };
    let far = finished(
        "far",
        brick::<f64>((10.0, 11.0), (10.0, 11.0), (0.0, 1.0), tol()),
        tol(),
    );
    let (edge, face) = u.contacts.edge_face().next().unwrap();
    let (fa, _) = u.contacts.face_face().next().unwrap();
    let rim = u
        .body
        .half_edges()
        .find(|&(he, _)| u.body.face_of_half_edge(he) == Some(face))
        .map(|(_, h)| h.edge)
        .unwrap();
    let ef = |edge, face| topo::OperandRecords {
        ef: vec![topo::CarriedRecord {
            contact: topo::EfContact { edge, face },
            record: 0,
        }],
        ..topo::OperandRecords::default()
    };
    let ff = |a, b| topo::OperandRecords {
        ff: vec![topo::CarriedRecord {
            contact: topo::FfContact { a, b },
            record: 0,
        }],
        ..topo::OperandRecords::default()
    };
    for (records_a, what) in [
        (
            ef(topo::EdgeKey::default(), face),
            "carried e-f edge key does not resolve",
        ),
        (
            ef(edge, topo::FaceKey::default()),
            "carried e-f face key does not resolve",
        ),
        (ef(rim, face), "carried e-f edge bounds its face"),
        (
            ff(fa, topo::FaceKey::default()),
            "carried f-f face key does not resolve",
        ),
        (ff(fa, fa), "carried f-f pair names one face twice"),
    ] {
        let got = topo::boolean_op_with(
            topo::BooleanOp::Union,
            &u.body,
            &far,
            &topo::BooleanInputs {
                records_a,
                ..topo::BooleanInputs::default()
            },
            topo::SweepStrategy::Realized,
            tol(),
        )
        .map(|_| ());
        assert!(
            matches!(
                got,
                Err(topo::BooleanError::InvalidInput {
                    operand: topo::Operand::A,
                    what: w,
                }) if w == what
            ),
            "{what}: {got:?}"
        );
    }
}
