//! SHELL-7 review lane R1: a differential corpus chosen differently from
//! the lane's `shell7_dump.rs` — sf2b frustums at three radius pairs, a
//! sphere-zone vase, the tube door's ARC elbows (solid and hollow), a
//! three-quarter-turn wedge (two wall bands), the hollow vessel shelled
//! twice, and a bellied pot. Free of every symbol SHELL-7 adds so it
//! compiles at the merge base; the diff of the `[r1diff]` lines is the
//! verdict.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::{FRAC_PI_2, PI};

use geom_core::{Point2, Point3, Tol, Vec2, Vec3};
use profile::{Profile, ProfileLoop, ProfileVertex, SketchPlane};
use sweep::{Revolution, RevolveAxis, TubeWindow, revolve, tube_along_arc, tube_along_arc_hollow};
use topo::Body;

fn p2(x: f64, y: f64) -> Point2<f64> {
    Point2::new(x, y)
}
fn tol() -> Tol {
    Tol::witness()
}

fn polyline(pts: &[(f64, f64)], turn: Revolution<f64>) -> Body<f64> {
    let lp = ProfileLoop::new(pts.iter().map(|&(x, y)| ProfileVertex::new(p2(x, y), 0.0)).collect());
    let profile = Profile::new(SketchPlane::xy(), vec![lp]).validate(tol()).expect("validates");
    revolve(&profile, RevolveAxis { origin: p2(0.0, 0.0), dir: Vec2::new(0.0, 1.0) }, turn, tol())
        .expect("revolves")
        .body
}

fn dump(label: &str, body: &Body<f64>) {
    println!(
        "[r1diff] {label}: solids={} shells={} faces={} edges={} vertices={}",
        body.solids().count(), body.shells().count(), body.faces().count(), body.edges().count(), body.vertices().count()
    );
    for (k, f) in body.faces() {
        println!("[r1diff] {label}: face {k:?} sense={} surface={:?}", f.sense, body.get_surface(f.surface));
    }
    for (k, e) in body.edges() {
        let c = body.get_curve_geom(e.curve).and_then(|g| g.certified()).unwrap();
        println!("[r1diff] {label}: edge {k:?} carrier={:?} params={:?} description={:?}", c.carrier(), c.params(), c.description());
    }
    for (k, v) in body.vertices() {
        println!("[r1diff] {label}: vertex {k:?} point={:?}", body.get_point(v.point).unwrap());
    }
    match topo::mass_properties(body, tol()) {
        Ok(p) => println!("[r1diff] {label}: volume={:?} pad={:?} area={:?}", p.volume, p.volume_pad, p.surface_area),
        Err(e) => println!("[r1diff] {label}: props Err {e:?}"),
    }
    println!("[r1diff] {label}: tier3={:?}", topo::validate_geometric(body, tol()));
}

fn shelled(label: &str, body: &Body<f64>, t: f64) -> Option<Body<f64>> {
    match topo::shell(body, t, tol()) {
        Ok(s) => {
            dump(label, &s.body);
            Some(s.body)
        }
        Err(e) => {
            println!("[r1diff] {label}: shell Err {e}");
            None
        }
    }
}

#[test]
fn shell7_r1_diff_corpus() {
    let t128 = 1.0 / 128.0;
    let h = 8.0 / 64.0;
    for (r0, r1) in [(4.0 / 64.0, 2.0 / 64.0), (2.0 / 64.0, 4.0 / 64.0), (3.0 / 64.0, 3.0 / 64.0 + 1.0 / 256.0)] {
        shelled(&format!("frustum {r0} {r1}"), &polyline(&[(0.0, 0.0), (r0, 0.0), (r1, h), (0.0, h)], Revolution::Full), t128);
    }
    shelled("wedge 3/4 turn", &polyline(&[(0.0, 0.0), (1.0, 0.0), (1.0, 2.0), (0.0, 2.0)], Revolution::Partial(3.0 * FRAC_PI_2)), 0.05);
    shelled("wedge 3/4 turn frustum", &polyline(&[(0.0, 0.0), (1.0, 0.0), (0.5, 2.0), (0.0, 2.0)], Revolution::Partial(3.0 * FRAC_PI_2)), 0.05);
    shelled("pot", &polyline(&[(0.0, 0.0), (0.8, 0.0), (1.2, 1.0), (1.0, 2.0), (0.0, 2.0)], Revolution::Full), 0.15);
    if let Some(hollow) = shelled("vessel", &polyline(&[(0.0, 0.0), (1.0, 0.0), (1.0, 2.0), (0.0, 2.0)], Revolution::Full), 0.2) {
        if let Some(again) = shelled("vessel twice", &hollow, 0.05) {
            shelled("vessel thrice", &again, 0.01);
        }
    }
    let (c, a, u) = (Point3::new(0.0, 0.0, 0.0), Vec3::unit_y(), Vec3::unit_x());
    let arc = TubeWindow::Arc { t0: 0.0, t1: PI / 3.0 };
    match tube_along_arc::<f64>(c, a, u, 2.0, arc, 0.5, tol()) {
        Ok(b) => { dump("elbow solid operand", &b.body); shelled("elbow solid", &b.body, 0.05); }
        Err(e) => println!("[r1diff] elbow solid: build Err {e:?}"),
    }
    match tube_along_arc_hollow::<f64>(c, a, u, 2.0, TubeWindow::Arc { t0: 0.0, t1: PI / 3.0 }, 0.5, 0.125, tol()) {
        Ok(b) => { dump("elbow hollow operand", &b.body); shelled("elbow hollow", &b.body, 0.05); }
        Err(e) => println!("[r1diff] elbow hollow: build Err {e:?}"),
    }
}
