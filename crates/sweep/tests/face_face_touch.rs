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
