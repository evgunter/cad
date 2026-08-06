//! TEMPORARY measurement probe (montage-v2, Evan's #218 follow-up):
//! what path classes does `sweep::sweep_body` accept today, measured
//! by building the bodies — not asserted. Deleted before the PR; the
//! findings live in the PR body.
//!
//! Classes probed:
//!   1. planar, continuously varying curvature (parabola — no arc
//!      segment anywhere)
//!   2. non-planar, constant curvature+torsion (circular helix, two
//!      pitch angles — torsion is what no assembly of revolves has)
//!   3. non-planar, continuously varying curvature AND torsion
//!      (conical helix)
//! Plus the frame-behaviour measurement: station-to-station rotation
//! of the path-following frame (the minimal-rotation-from-start
//! frame's near-antipode swing), and the loft-pair spacing candidates
//! for item 3.

#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::cast_precision_loss
)]

use geom_core::{Affine3, Point2, Point3, Vec3};
use geom_curves::NurbsCurve3;
use mesh::validate::{check_mesh, signed_volume, triangle_count};
use profile::SketchPlane;
use sweep::skin::SectionSegments;
use sweep::{SketchSegment, sweep_places};

fn quad(h: f64) -> SectionSegments {
    let p = |x: f64, y: f64| Point2::new(x, y);
    let seg = |a: Point2<f64>, b: Point2<f64>| SketchSegment::Line { a, b };
    vec![vec![
        seg(p(-h, -h), p(h, -h)),
        seg(p(h, -h), p(h, h)),
        seg(p(h, h), p(-h, h)),
        seg(p(-h, h), p(-h, -h)),
    ]]
}

/// Profile placement normal to the path's start tangent (the
/// narration's frame recipe, verbatim).
fn place_normal_to(path: &NurbsCurve3<f64>) -> Affine3<f64> {
    let (lo, _) = path.domain();
    let d = path.deriv(lo);
    let n = d / d.norm();
    let helper = if n.z.abs() < 0.9 {
        Vec3::unit_z()
    } else {
        Vec3::unit_x()
    };
    let u = helper.cross(n);
    let u = u / u.norm();
    SketchPlane::from_frame(path.eval(lo), u, n.cross(u)).placement
}

/// Arc length of the interpolant by composite Simpson on |dC/dt|.
fn curve_length(path: &NurbsCurve3<f64>, n: usize) -> f64 {
    let (lo, hi) = path.domain();
    let h = (hi - lo) / n as f64;
    let f = |t: f64| path.deriv(t).norm();
    let mut s = f(lo) + f(hi);
    for i in 1..n {
        let w = if i % 2 == 1 { 4.0 } else { 2.0 };
        s += w * f((i as f64).mul_add(h, lo));
    }
    s * h / 3.0
}

/// Relative rotation angle (deg) between consecutive station frames.
fn frame_steps(places: &[Affine3<f64>]) -> Vec<f64> {
    let axes = [Vec3::unit_x(), Vec3::unit_y(), Vec3::unit_z()];
    places
        .windows(2)
        .map(|w| {
            let tr: f64 = axes
                .iter()
                .map(|a| w[0].transform_vec(*a).dot(w[1].transform_vec(*a)))
                .sum();
            (((tr - 1.0) / 2.0).clamp(-1.0, 1.0)).acos().to_degrees()
        })
        .collect()
}

fn probe_sweep(name: &str, points: &[Point3<f64>], stations: usize, h: f64, expect_len: f64) {
    println!(
        "\n== {name} ({} interp points, {stations} stations) ==",
        points.len()
    );
    let path = NurbsCurve3::interpolate(points, 3).expect("path interpolates");
    let place = place_normal_to(&path);
    match sweep_places(place, &path, stations) {
        Ok(places) => {
            let steps = frame_steps(&places);
            let max = steps.iter().cloned().fold(0.0f64, f64::max);
            let sum: f64 = steps.iter().sum();
            println!("   frame: max station-to-station turn {max:.1} deg, total {sum:.1} deg");
        }
        Err(e) => {
            println!("   sweep_places REFUSED: {e:?}");
            return;
        }
    }
    match sweep::sweep_body::<f64>(&quad(h), place, &path, stations, 3) {
        Ok(lofted) => {
            let body = lofted.body;
            let t1 = topo::validate(&body).is_ok();
            let t2 = topo::validate_closed(&body).is_ok();
            let t3 = topo::validate_geometric(&body);
            let props = topo::mass_properties(&body).expect("mass properties");
            let len = curve_length(&path, 2000);
            let al = (2.0 * h) * (2.0 * h) * len;
            println!(
                "   body BUILDS: tier1 {t1}, tier2 {t2}, tier3 {:?}",
                t3.map(|()| "ok")
            );
            println!(
                "   V = {:.6} (pad {:.1e}); A*L(interpolant) = {al:.6} ({:+.3}%); \
                 L = {len:.6} vs analytic {expect_len:.6}",
                props.volume,
                props.volume_pad,
                (props.volume - al) / al * 100.0
            );
            match mesh::tessellate(&body, 5e-3) {
                Ok(m) => {
                    let cm = check_mesh(&m).is_ok();
                    let vm = signed_volume(&m);
                    println!(
                        "   mesh: {} tris, check_mesh {cm}, V_mesh = {vm:.6} \
                         ({:+.3}% off exact)",
                        triangle_count(&m),
                        (vm - props.volume) / props.volume * 100.0
                    );
                }
                Err(e) => println!("   tessellate REFUSED: {e:?}"),
            }
            match step_export::step_string(&body, &step_export::StepOptions::default()) {
                Ok(doc) => println!("   STEP exports ({} bytes)", doc.len()),
                Err(e) => println!("   STEP REFUSED: {e:?}"),
            }
        }
        Err(e) => println!("   sweep_body REFUSED: {e:?}"),
    }
}

fn loft_candidate(zmid: f64) {
    use sweep::skin::loft_geometry;
    let quad4 = |pts: [(f64, f64); 4]| -> SectionSegments {
        let seg = |a: (f64, f64), b: (f64, f64)| SketchSegment::Line {
            a: Point2::new(a.0, a.1),
            b: Point2::new(b.0, b.1),
        };
        vec![vec![
            seg(pts[0], pts[1]),
            seg(pts[1], pts[2]),
            seg(pts[2], pts[3]),
            seg(pts[3], pts[0]),
        ]]
    };
    let square = [(-1.0, -1.0), (1.0, -1.0), (1.0, 1.0), (-1.0, 1.0)];
    let trap = [(-1.375, -1.0), (1.375, -1.0), (1.0, 1.0), (-1.0, 1.0)];
    let sections = vec![quad4(square), quad4(trap), quad4(square)];
    let places: Vec<Affine3<f64>> = [0.0, zmid, 2.0]
        .iter()
        .map(|z| Affine3::translation(Vec3::new(0.0, 0.0, *z)))
        .collect();
    let geo = loft_geometry(&sections, &places, 2).expect("loft geometry");
    let t = geo.section_params[1];
    // Closed form: V = 4H + dH/(3 t(1-t)), H = 2, d = 0.375.
    let v_closed = 0.25f64.mul_add((t * (1.0 - t)).recip(), 8.0);
    // Silhouette numbers: bulge peak half-width 1 + d/(4 t(1-t)) at
    // z(1/2) with z(v) the quadratic through (0,0),(t,zmid),(1,2).
    let a = (zmid - 2.0 * t) / (t * (t - 1.0));
    let b = 2.0 - a;
    let peak_w = 1.0 + 0.375 * 0.25 / (t * (1.0 - t));
    let peak_z = a / 4.0 + b / 2.0;
    match sweep::loft_body::<f64>(&sections, &places, 2) {
        Ok(l) => {
            let props = topo::mass_properties(&l.body).expect("props");
            println!(
                "z_mid = {zmid}: t = {t:.17}, V_closed = {v_closed:.12}, V_meas = {:.12} \
                 (pad {:.1e}); peak half-width {peak_w:.4} at z = {peak_z:.4} \
                 ({:.1}% height); z'(0) = {b:.4}",
                props.volume,
                props.volume_pad,
                peak_z / 2.0 * 100.0
            );
        }
        Err(e) => println!("z_mid = {zmid}: loft_body REFUSED: {e:?}"),
    }
}

fn main() {
    // ---- (1) planar, continuously varying curvature: parabola ----
    // z = x^2 / 2, x in [-1.5, 1.5]; curvature 1/(1+x^2)^{3/2} peaks
    // at the apex and decays — no arc anywhere.
    let parabola: Vec<Point3<f64>> = (0..=16)
        .map(|k| {
            let x = 3.0f64.mul_add(f64::from(k) / 16.0, -1.5);
            Point3::new(x, 0.0, x * x / 2.0)
        })
        .collect();
    // analytic length of parabola z = x^2/2: integral sqrt(1+x^2)
    let asinh_l = |x: f64| 0.5 * (x * (1.0 + x * x).sqrt() + x.asinh());
    probe_sweep(
        "parabola (planar, varying curvature)",
        &parabola,
        13,
        0.25,
        asinh_l(1.5) - asinh_l(-1.5),
    );

    // ---- (2) circular helix, shallow (pitch angle ~14.9 deg) ----
    let helix = |r: f64, c: f64, turns: f64, n: usize| -> Vec<Point3<f64>> {
        (0..=n)
            .map(|k| {
                let th = turns * core::f64::consts::TAU * f64::from(k as u32) / n as f64;
                Point3::new(r * th.cos(), r * th.sin(), c * th)
            })
            .collect()
    };
    let len = |r: f64, c: f64, turns: f64| (r * r + c * c).sqrt() * turns * core::f64::consts::TAU;
    probe_sweep(
        "helix r=1.5 c=0.4 (alpha 14.9deg), 1.25 turns",
        &helix(1.5, 0.4, 1.25, 40),
        21,
        0.25,
        len(1.5, 0.4, 1.25),
    );

    // ---- (2b) circular helix, steep (pitch angle 30 deg) ----
    let c30 = 1.2 * (30.0f64.to_radians()).tan();
    probe_sweep(
        "helix r=1.2 alpha=30deg, 1.25 turns",
        &helix(1.2, c30, 1.25, 40),
        21,
        0.25,
        len(1.2, c30, 1.25),
    );

    // ---- (2c) the chord-meter boundary: alpha=20deg, turn sweep ----
    // speed_lower_bound projects derivative control points on the
    // corner path's own chord: a FULL turn's chord is vertical and
    // every tangent climbs, so exactly-one-turn should pass while
    // partial turns beyond a half-space excursion fail.
    let c20 = 1.4 * (20.0f64.to_radians()).tan();
    for (turns, st) in [
        (0.4, 13),
        (0.5, 15),
        (0.6, 17),
        (0.75, 17),
        (1.0, 25),
        (1.1, 25),
        (1.25, 25),
    ] {
        probe_sweep(
            &format!("helix r=1.4 alpha=20deg, {turns} turns"),
            &helix(1.4, c20, turns, 40),
            st,
            0.25,
            len(1.4, c20, turns),
        );
    }

    // ---- (3) conical helix, ONE turn: curvature+torsion vary ----
    let cone: Vec<Point3<f64>> = (0..=40)
        .map(|k| {
            let th = core::f64::consts::TAU * f64::from(k as u32) / 40.0;
            let r = 0.0955f64.mul_add(th, 1.0);
            Point3::new(r * th.cos(), r * th.sin(), 0.509 * th)
        })
        .collect();
    probe_sweep(
        "conical helix r=1.0..1.6 c=0.509, 1 turn",
        &cone,
        25,
        0.25,
        f64::NAN,
    );

    // ---- cell candidates ----
    // B. twisted cubic (t, t^2, t^3) scaled: curvature AND torsion
    // both continuously varying, no arc anywhere, non-planar.
    let cubic: Vec<Point3<f64>> = (0..=32)
        .map(|k| {
            let t = 2.0f64.mul_add(f64::from(k as u32) / 32.0, -1.0);
            Point3::new(2.2 * t, 1.3 * t * t, 1.5 * t * t * t)
        })
        .collect();
    probe_sweep(
        "twisted cubic (2.2t, 1.3t^2, 1.5t^3)",
        &cubic,
        17,
        0.25,
        f64::NAN,
    );

    // C. conical spiral arc, 0.4 turns, radius 1.0 -> 1.8.
    let spiral: Vec<Point3<f64>> = (0..=32)
        .map(|k| {
            let th = 0.4 * core::f64::consts::TAU * f64::from(k as u32) / 32.0;
            let r = 0.318f64.mul_add(th, 1.0);
            Point3::new(r * th.cos(), r * th.sin(), 0.55 * th)
        })
        .collect();
    probe_sweep(
        "conical spiral 0.4 turns r=1.0..1.8",
        &spiral,
        17,
        0.25,
        f64::NAN,
    );

    // ---- item 3: loft-pair spacing candidates ----
    println!("\n== loft-pair spacing candidates (H = 2, d = 0.375) ==");
    for z in [1.0, 0.3, 0.2, 0.15, 0.1] {
        loft_candidate(z);
    }
}
