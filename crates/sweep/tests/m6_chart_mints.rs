//! **M6-3 Leg E: the walk-row-4 sentence, executed** — the corpus's
//! revolve bodies (ball, cone, donut) and the die's eight sphere
//! octants carry STORED certified pcurves at rest, on every curved
//! chart (cone/sphere/torus mint since M6-3, exactly as cylinders
//! have since M5 PR 6).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::Surface;
use geom_core::{Point2, Tolerance, Vec2};
use profile::RawLoop;
use profile::{Profile, ProfileLoop, ProfileVertex, SketchPlane};
use sweep::{Extrusion, Revolution, RevolveAxis, extrude, revolve};
use topo::Body;
use geom_core::Tol;

fn validated(lp: ProfileLoop<f64>) -> profile::ValidatedProfile<f64> {
    Profile::new(SketchPlane::xy(), vec![lp])
        .validate(Tol::witness())
        .unwrap()
}

fn axis_y() -> RevolveAxis<f64> {
    RevolveAxis {
        origin: Point2::new(0.0, 0.0),
        dir: Vec2::new(0.0, 1.0),
    }
}

/// Every half-edge of every NON-PLANAR face carries a stored cache
/// (planar faces keep derive-on-demand — C4), and tier 3 is green.
fn assert_curved_faces_fully_minted(name: &str, body: &Body<f64>) {
    assert_eq!(topo::validate_geometric(body, Tol::witness()), Ok(()), "{name} tier 3");
    let mut curved_hes = 0usize;
    for (_, face) in body.faces() {
        let surface = body.get_surface(face.surface).expect("surface resolves");
        if matches!(surface, Surface::Plane { .. }) {
            continue;
        }
        for lk in core::iter::once(face.outer).chain(face.rings.iter().copied()) {
            let lp = body.get_loop(lk).expect("loop resolves");
            let topo::LoopBoundary::Cycle { first } = lp.boundary else {
                continue;
            };
            for he in body.loop_cycle(first).expect("cycle closes") {
                curved_hes += 1;
                assert!(
                    body.pcurve(he).is_some(),
                    "{name}: half-edge {he:?} of a {:?}-chart face carries no stored \
                     cache at rest",
                    match surface {
                        Surface::Cylinder { .. } => "cylinder",
                        Surface::Cone { .. } => "cone",
                        Surface::Sphere { .. } => "sphere",
                        Surface::Torus { .. } => "torus",
                        _ => "other",
                    }
                );
            }
        }
    }
    assert!(curved_hes > 0, "{name}: the fixture has curved faces");
}

/// The ball: two half-sphere faces, pole-to-pole seam meridians.
#[test]
fn the_ball_carries_stored_sphere_pcurves_at_rest() {
    let lp = ProfileLoop::new(vec![
        ProfileVertex::new(Point2::new(0.0, -1.0), 1.0),
        ProfileVertex::new(Point2::new(0.0, 1.0), 0.0),
    ]);
    let t = revolve::<f64>(&validated(lp), axis_y(), Revolution::Full, Tol::witness()).unwrap();
    assert_curved_faces_fully_minted("ball", &t.body);
}

/// The cone: rim circles (polar class) + apex + mirror-free rulings.
#[test]
fn the_cone_carries_stored_cone_pcurves_at_rest() {
    let lp = ProfileLoop::polygon([
        Point2::new(0.0, 0.0),
        Point2::new(1.0, 0.0),
        Point2::new(0.0, 1.0),
    ]);
    let t = revolve::<f64>(&validated(lp), axis_y(), Revolution::Full, Tol::witness()).unwrap();
    assert_curved_faces_fully_minted("cone", &t.body);
}

/// The donut: parallel rims + seam meridian circles, the torus chart's
/// two closed-form families in one body.
#[test]
fn the_donut_carries_stored_torus_pcurves_at_rest() {
    let lp = ProfileLoop::new(vec![
        ProfileVertex::new(Point2::new(1.5, 0.0), 1.0),
        ProfileVertex::new(Point2::new(2.5, 0.0), 1.0),
    ]);
    let t = revolve::<f64>(&validated(lp), axis_y(), Revolution::Full, Tol::witness()).unwrap();
    assert_curved_faces_fully_minted("donut", &t.body);
}

/// The die blank's EIGHT sphere octants (plus its twelve
/// quarter-cylinders): the fillet pipeline's mint pass now covers
/// every chart — each octant's three great-circle arcs are meridian /
/// polar class relative to its stored chart axis, and the walk closes
/// through the pole with the zero local azimuth lever.
#[test]
fn the_die_octants_carry_stored_sphere_pcurves_at_rest() {
    let tol = Tol::witness().get();
    let band = geom_core::Band::new(tol.eps, tol.k * tol.eps).expect("band");
    let lp = ProfileLoop::polygon([
        Point2::new(0.0, 0.0),
        Point2::new(1.0, 0.0),
        Point2::new(1.0, 1.0),
        Point2::new(0.0, 1.0),
    ]);
    let blank = extrude(&validated(lp), Extrusion::Distance(1.0), Tol::witness())
        .unwrap()
        .body;
    let rims: Vec<topo::EdgeKey> = blank.edges().map(|(k, _)| k).collect();
    let filleted = sweep::fillet::fillet_edges(&blank, &rims, 0.12, band, Tol::witness())
        .expect("the die blank fillets")
        .body;
    let mut spheres = 0usize;
    for (_, face) in filleted.faces() {
        if matches!(
            filleted.get_surface(face.surface).expect("surface"),
            Surface::Sphere { .. }
        ) {
            spheres += 1;
        }
    }
    assert_eq!(spheres, 8, "the eight corner octants");
    assert_curved_faces_fully_minted("filleted die", &filleted);
}
