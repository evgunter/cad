//! M9-2 PR-2 blinded-review probe (R2), frozen head a1b78954: the
//! cross-instance curved CONFORMAL touch. A boss inserted through a
//! same-radius hole touches wall-on-wall (gap = 0, opposed material)
//! across two instances whose cylinder carriers are value-equal but
//! DISTINCT keys — the class the shared-key conformal arm cannot see
//! and the stated envelope (curved TANGENT touching / curved
//! transverse interference) does not name. The probe records what the
//! 3' gate actually answers.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Affine3, Point2, Tolerance, Vec3};
use profile::RawLoop;
use profile::{Profile, ProfileLoop, ProfileVertex, SketchPlane};
use sweep::{Extrusion, extrude};
use topo::Body;
use geom_core::Tol;

fn p2(x: f64, y: f64) -> Point2<f64> {
    Point2::new(x, y)
}

/// The plate with a radius-0.5 hole at (2, 2): 4x4x1, z in [0, 1];
/// the hole is two semicircular arcs with joints at 0 and 180 deg.
fn holed_plate() -> Body<f64> {
    let outer = ProfileLoop::new(
        [(0.0, 0.0), (4.0, 0.0), (4.0, 4.0), (0.0, 4.0)]
            .map(|(x, y)| ProfileVertex::new(p2(x, y), 0.0))
            .to_vec(),
    );
    let hole = ProfileLoop::new(vec![
        ProfileVertex::new(p2(2.5, 2.0), 1.0),
        ProfileVertex::new(p2(1.5, 2.0), 1.0),
    ]);
    let profile = Profile::new(SketchPlane::xy(), vec![outer, hole])
        .validate(Tol::witness())
        .unwrap();
    extrude(&profile, Extrusion::Distance(1.0), Tol::witness()).unwrap().body
}

/// The through-boss: radius 0.5 at (2, 2), three 120-deg arcs with
/// joints at 90/210/330 deg (angles chosen OFF the hole's rim
/// vertices so no vertex-vertex or vertex-on-line coincidence backs
/// the interface by accident), z in [-0.2, 1.4].
fn through_boss() -> Body<f64> {
    let b120 = (core::f64::consts::PI / 6.0).tan();
    let at = |deg: f64| {
        let th = deg.to_radians();
        p2(2.0 + 0.5 * th.cos(), 2.0 + 0.5 * th.sin())
    };
    let lp = ProfileLoop::new(vec![
        ProfileVertex::new(at(90.0), b120),
        ProfileVertex::new(at(210.0), b120),
        ProfileVertex::new(at(330.0), b120),
    ]);
    let plane = SketchPlane::new(Affine3::translation(Vec3::new(0.0, 0.0, -0.2)));
    let profile = Profile::new(plane, vec![lp])
        .validate(Tol::witness())
        .unwrap();
    extrude(&profile, Extrusion::Distance(1.6), Tol::witness()).unwrap().body
}

#[test]
fn probe_cross_instance_conformal_wall_touch_at_three_prime() {
    let plate = holed_plate();
    let boss = through_boss();
    assert_eq!(topo::validate_geometric(&plate, Tol::witness()), Ok(()), "plate tier 3");
    assert_eq!(topo::validate_geometric(&boss, Tol::witness()), Ok(()), "boss tier 3");
    let mut body = plate.clone();
    topo::graft_disjoint(&mut body, &boss, Tol::witness()).unwrap();
    let verdict = topo::validate_pseudomanifold(&body, &topo::ContactRecords::default(), Tol::witness());
    println!("cross-instance conformal wall touch, undeclared: {verdict:?}");
    // OBSERVED (a1b78954): the gate refuses LOUDLY — but not through
    // any conformal arm (the walls' keys are distinct, so the arm
    // never pairs them). The boss's wall joint-line edges cross the
    // plate's top/bottom face planes at points exactly ON the hole's
    // rim circle (the faces' trim boundary), and contfp classifies
    // those on-boundary grazes as In, so the touch surfaces as
    // `EdgeFacePierce` — a class the PR itself calls categorically
    // UNDECLARABLE. Consequences pinned here: (1) A5 holds — the
    // undeclared touch is never blessed; (2) the finding's CLASS is
    // wrong — the geometry touches (gap == 0), it does not cross, and
    // a legitimate boss-in-hole assembly has no declarable recourse;
    // (3) the census envelope statement does not name this
    // distinct-carrier conformal class either way.
    let errs = verdict.expect_err("the undeclared cross-instance touch must not bless");
    assert!(
        errs.iter().any(|e| matches!(
            e,
            topo::ValidationError::UndeclaredContact {
                contact: topo::CensusContact::EdgeFacePierce { .. },
                ..
            }
        )),
        "observed refusal shape (grazing joint edges as pierces): {errs:?}"
    );
    assert!(
        !errs.iter().any(|e| matches!(
            e,
            topo::ValidationError::UndeclaredContact {
                contact: topo::CensusContact::ConformalPatch { .. },
                ..
            }
        )),
        "the conformal arm cannot see distinct-key pairs (shared-key scope): {errs:?}"
    );
}
