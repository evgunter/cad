//! CERT-N3 blinded review (R1) — item 4 (pruning delta, both arms via
//! the `N3R1_BASE_ARM` switch planted in `edge_box`) and item 9 (the
//! conic-jet evaluation count per boolean, via the `N3R1_CONIC_JET`
//! counter planted in `neighborhood::chord`). Prints one line per corpus
//! case; run with `--nocapture` once per arm and diff. Probe branch only.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::sync::atomic::Ordering;
use geom_core::Tol;
use geom_core::{Affine3, Point2, Vec3};
use profile::RawLoop;
use profile::{Profile, ProfileLoop, ProfileVertex, SketchPlane};
use std::collections::BTreeSet;
use sweep::{Extrusion, extrude};
use topo::{Body, BooleanResult, SweepStrategy, SweepTrace, sweep_traces};

fn p2(x: f64, y: f64) -> Point2<f64> {
    Point2::new(x, y)
}

fn cylinder_at(cx: f64) -> Body<f64> {
    let b120 = (core::f64::consts::PI / 6.0).tan();
    let at = |deg: f64| {
        let th: f64 = deg.to_radians();
        p2(cx + 0.5 * th.cos(), 0.5 * th.sin())
    };
    let lp = ProfileLoop::new(vec![
        ProfileVertex::new(at(0.0), b120),
        ProfileVertex::new(at(120.0), b120),
        ProfileVertex::new(at(240.0), b120),
    ]);
    let profile = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(Tol::witness())
        .unwrap();
    extrude(&profile, Extrusion::Distance(1.0), Tol::witness())
        .unwrap()
        .body
}

fn cylinder() -> Body<f64> {
    cylinder_at(0.0)
}

fn small_box(cx: f64, h: f64, z0: f64) -> Body<f64> {
    let lp = ProfileLoop::new(
        [(cx - h, -h), (cx + h, -h), (cx + h, h), (cx - h, h)]
            .into_iter()
            .map(|(x, y)| ProfileVertex::new(p2(x, y), 0.0))
            .collect(),
    );
    let plane = SketchPlane::new(Affine3::translation(Vec3::new(0.0, 0.0, z0)));
    let profile = Profile::new(plane, vec![lp])
        .validate(Tol::witness())
        .unwrap();
    extrude(&profile, Extrusion::Distance(0.4), Tol::witness())
        .unwrap()
        .body
}

fn nested_box(cx: f64, h: f64) -> Body<f64> {
    small_box(cx, h, 0.3)
}

fn plate(x: (f64, f64), y: (f64, f64), z: (f64, f64)) -> Body<f64> {
    let lp = ProfileLoop::new(
        [(x.0, y.0), (x.1, y.0), (x.1, y.1), (x.0, y.1)]
            .into_iter()
            .map(|(a, b)| ProfileVertex::new(p2(a, b), 0.0))
            .collect(),
    );
    let plane = SketchPlane::new(Affine3::translation(Vec3::new(0.0, 0.0, z.0)));
    let profile = Profile::new(plane, vec![lp])
        .validate(Tol::witness())
        .unwrap();
    extrude(&profile, Extrusion::Distance(z.1 - z.0), Tol::witness())
        .unwrap()
        .body
}

fn rim_plate(x_max: f64) -> Body<f64> {
    plate((-0.9, x_max), (-0.15, 0.15), (-0.1, 0.1))
}

fn top_rim_plate(y_min: f64) -> Body<f64> {
    plate((-0.15, 0.15), (y_min, 0.9), (0.9, 1.1))
}

fn rounded_plate() -> Body<f64> {
    let pts = [
        ((-1.0, -0.4), 0.0),
        ((1.0, -0.4), 0.35),
        ((1.3, 0.0), 0.0),
        ((1.0, 0.4), 0.0),
        ((-1.0, 0.4), 0.35),
        ((-1.3, 0.0), 0.0),
    ];
    let lp = ProfileLoop::new(
        pts.iter()
            .map(|&((x, y), b)| ProfileVertex::new(p2(x, y), b))
            .collect(),
    );
    let profile = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(Tol::witness())
        .unwrap();
    extrude(&profile, Extrusion::Distance(0.8), Tol::witness())
        .unwrap()
        .body
}

fn corpus() -> Vec<(String, Body<f64>, Body<f64>)> {
    let cyl = cylinder();
    let rounded = rounded_plate();
    let mut v = vec![
        ("cylinder x nested box".to_string(), cyl.clone(), nested_box(-0.3, 0.05)),
        ("cylinder x box beside".to_string(), cyl.clone(), nested_box(3.0, 0.2)),
        ("cylinder x crossing box".to_string(), cyl.clone(), nested_box(0.45, 0.2)),
        ("cylinder x plate across x-extreme".to_string(), cyl.clone(), rim_plate(-0.499)),
        ("cylinder x plate at -0.45".to_string(), cyl.clone(), rim_plate(-0.45)),
        ("cylinder x top plate at 0.499".to_string(), cyl.clone(), top_rim_plate(0.499)),
        ("rounded x box clear of round".to_string(), rounded.clone(), plate((1.2, 1.6), (0.36, 0.6), (0.2, 0.5))),
        ("rounded x box grazing round".to_string(), rounded.clone(), plate((1.18, 1.6), (0.33, 0.6), (0.2, 0.5))),
        ("rounded x corner box".to_string(), rounded, plate((1.1, 1.6), (0.2, 0.6), (0.2, 0.5))),
        ("cylinder x cylinder 1e-3 apart".to_string(), cyl.clone(), cylinder_at(1.001)),
        ("cylinder x cylinder shifted 0.3".to_string(), cyl.clone(), cylinder_at(0.3)),
    ];
    for &x_max in &[-0.5003, -0.5006, -0.501, -0.502, -0.51, -0.6] {
        v.push((format!("cylinder x rim plate clear by {:.1e}", -0.5 - x_max), cyl.clone(), rim_plate(x_max)));
    }
    for &y_min in &[0.5006, 0.502, 0.51] {
        v.push((format!("cylinder x top rim plate clear by {:.1e}", y_min - 0.5), cyl.clone(), top_rim_plate(y_min)));
    }
    v
}

fn ex(t: &SweepTrace) -> BTreeSet<(topo::EdgeKey, topo::FaceKey)> {
    t.examined.iter().copied().collect()
}

fn digest(r: &Result<BooleanResult<f64>, topo::BooleanError>) -> String {
    match r {
        Ok(BooleanResult::Empty) => "Ok(Empty)".into(),
        Ok(BooleanResult::Body(b)) => format!(
            "Ok({:?} f={} e={} v={})",
            b.kind,
            b.body.faces().count(),
            b.body.edges().count(),
            b.body.vertices().count()
        ),
        Err(e) => format!("Err({e:?})").chars().take(80).collect(),
        #[allow(unreachable_patterns)]
        Ok(_) => "Ok(other)".into(),
    }
}

#[test]
fn n3r1_prune_and_jet_count() {
    let arm = if std::env::var_os("N3R1_BASE_ARM").is_some() { "BASE" } else { "HEAD" };
    let mut total_prune_pairs = 0usize;
    for (name, a, b) in corpus() {
        let real = sweep_traces(&a, &b, SweepStrategy::Realized, None, Tol::witness());
        let ideal = sweep_traces(&a, &b, SweepStrategy::Idealized, None, Tol::witness());
        let (rx, ry) = match &real {
            Ok((x, y)) => (ex(x).len(), ex(y).len()),
            Err(_) => (usize::MAX, usize::MAX),
        };
        let (ax, ay, lost) = match (&real, &ideal) {
            (Ok((x, y)), Ok((ix, iy))) => {
                let lx = ix.accepted.iter().filter(|p| !ex(x).contains(p)).count();
                let ly = iy.accepted.iter().filter(|p| !ex(y).contains(p)).count();
                (ix.accepted.len(), iy.accepted.len(), lx + ly)
            }
            _ => (usize::MAX, usize::MAX, 0),
        };
        let before = topo::N3R1_CONIC_JET.load(Ordering::Relaxed);
        let sub = topo::boolean::subtract(&a, &b, Tol::witness());
        let jets = topo::N3R1_CONIC_JET.load(Ordering::Relaxed) - before;
        if rx != usize::MAX { total_prune_pairs += rx + ry; }
        let ideal_s = if ideal.is_err() { "ideal REFUSED".to_string() } else { format!("accepted {ax}+{ay} lost {lost}") };
        eprintln!(
            "N3R1[{arm}] {name}: examined {rx}+{ry}; {ideal_s}; jets {jets}; subtract {}",
            digest(&sub)
        );
    }
    eprintln!("N3R1[{arm}] total examined pairs {total_prune_pairs}");
}
