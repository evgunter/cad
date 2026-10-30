//! Review probes for PR 6088 (not for merge).
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
use core::f64::consts::{PI, TAU};
use geom_core::{Arc2, Band, Point2, Real, Tol};
use profile::{Profile, ProfileLoop, RawLoop, Segment, SketchPlane};
use sweep::Revolution;

fn circle<T: Real>(cx: f64, cy: f64, r: f64) -> ProfileLoop<T> {
    let f = T::from_f64;
    RawLoop::new([(
        Point2::new(f(cx + r), f(cy)),
        Segment::Arc(Arc2 { centre: Point2::new(f(cx), f(cy)), radius: f(r), sweep: f(TAU) }),
    )])
}

/// whole_surface(torus, sense) against the full torus face's own Green reading.
#[test]
fn probe_whole_torus_sign_matches_full_torus_face() {
    let (cx, r) = (3.0, 0.5);
    let vp = Profile::new(SketchPlane::<f64>::xy(), vec![circle(cx, 0.25, r)]).validate(Tol::witness()).unwrap();
    let axis = sweep::RevolveAxis { origin: Point2::new(0.0, 0.0), dir: geom_core::Vec2::new(0.0, 1.0) };
    let body = sweep::revolve(&vp, axis, Revolution::Full, Tol::witness()).unwrap().body;
    for (face, f) in body.faces() {
        let surface = body.get_surface(f.surface).unwrap();
        let reading = topo::test_support::face_reading(&body, face, Tol::witness(), 1e-6).unwrap();
        let whole = geom_brep::props::whole_surface(surface, f.sense).unwrap();
        eprintln!("PROBE torus sense={} reading flux={} area={} whole flux={} area={} pappus V*3={}",
            f.sense, reading.flux, reading.area, whole.flux, whole.area, 3.0 * TAU * PI * r * r * cx);
        assert!((reading.flux - whole.flux).abs() < 1e-9 * whole.flux.abs(), "torus whole-surface sign/magnitude");
        // An empty loop passed inside a non-empty list: the empty rule keys on the outer slice.
        let band = Band::linear(Tol::witness()).unwrap();
        let empty: &[geom_brep::props::LoopEdge<f64>] = &[];
        let r1 = geom_brep::props::curved_face(surface, empty, f.sense, band);
        let r2 = geom_brep::props::curved_face_loops(surface, &[empty], f.sense, band);
        eprintln!("PROBE curved_face(torus, []) = {r1:?}; curved_face_loops(torus, &[[]]) = {r2:?}");
    }
}
