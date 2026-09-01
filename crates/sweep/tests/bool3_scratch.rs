//! SCRATCH — deleted before the PR.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Point2, Tol};
use profile::{Profile, ProfileLoop, ProfileVertex, RawLoop, SketchPlane};
use topo::Body;

fn p2(x: f64, y: f64) -> Point2<f64> {
    Point2::new(x, y)
}

fn donut() -> Body<f64> {
    use sweep::{Revolution, RevolveAxis, revolve};
    let lp = ProfileLoop::new(vec![
        ProfileVertex::new(p2(0.5, 1.10), 1.0),
        ProfileVertex::new(p2(0.5, 1.40), 1.0),
    ]);
    let vp = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(Tol::witness())
        .unwrap();
    let axis = RevolveAxis {
        origin: p2(0.0, 0.0),
        dir: geom_core::Vec2::new(0.0, 1.0),
    };
    revolve(&vp, axis, Revolution::Full, Tol::witness())
        .unwrap()
        .body
}

#[test]
fn scratch_report_donut_structure() {
    let b = donut();
    println!("faces: {}", b.faces().count());
    for (k, f) in b.faces() {
        let s = b.get_surface(f.surface);
        let kind = match s {
            Some(geom::Surface::Torus { .. }) => "Torus",
            Some(geom::Surface::Plane { .. }) => "Plane",
            Some(geom::Surface::Cylinder { .. }) => "Cylinder",
            Some(geom::Surface::Sphere { .. }) => "Sphere",
            Some(geom::Surface::Cone { .. }) => "Cone",
            _ => "other",
        };
        println!(
            "face {k:?} surface {:?} kind {kind} rings {} sense {}",
            f.surface,
            f.rings.len(),
            f.sense
        );
        let topo::entity::LoopBoundary::Cycle { first } = b.get_loop(f.outer).unwrap().boundary
        else {
            println!("  NOT A CYCLE");
            continue;
        };
        for he in b.loop_cycle(first).unwrap() {
            let h = b.get_half_edge(he).unwrap();
            let e = b.get_edge(h.edge).unwrap();
            let mate_face = b
                .mate(he)
                .and_then(|m| b.get_half_edge(m))
                .and_then(|x| b.get_loop(x.parent_loop))
                .map(|l| l.face);
            let carrier = match b.get_curve_geom(e.curve) {
                Some(topo::null::CurveGeom::Certified(c)) => {
                    format!("{:?} params {:?}", c.carrier(), c.params())
                }
                _ => "uncertified".into(),
            };
            println!("  he {he:?} mate_face {mate_face:?} carrier {carrier}");
        }
    }
}
