//! R2 review probes for PR 1389 (issue 1362, walk band_u loop-area
//! anchor). Each probe prints its measured digits; the standalone
//! folds replicate `walk::loop_area`'s two spellings (the private fn
//! cannot be named from an integration test) so both can run in one
//! binary against the same band.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Point2, Point3, Tol, Vec2, Vec3};
use mesh::validate::{check_mesh, signed_volume};
use profile::{Profile, ProfileLoop, ProfileVertex, RawLoop, SketchPlane};
use sweep::{Revolution, RevolveAxis, revolve};

const TAU: f64 = core::f64::consts::TAU;

/// `raw + 2πk` nearest `prev` — `walk::unwrap_near`'s spelling.
fn unwrap_near(raw: f64, prev: f64) -> f64 {
    raw + TAU * ((prev - raw) / TAU).round()
}

/// The OLD spelling: world-origin-anchored area fold.
fn origin_area(pts: &[Point3<f64>]) -> Vec3<f64> {
    let mut area = Vec3::new(0.0, 0.0, 0.0);
    for (i, p) in pts.iter().enumerate() {
        let q = pts[(i + 1) % pts.len()];
        area = area + (*p - Point3::origin()).cross(q - Point3::origin());
    }
    area
}

/// The NEW spelling: bbox-centre-anchored fold (walk::loop_area).
fn local_area(pts: &[Point3<f64>]) -> Vec3<f64> {
    let first = pts[0];
    let (lo, hi) = pts
        .iter()
        .fold((first, first), |(lo, hi), &p| (lo.min(p), hi.max(p)));
    let o = lo + (hi - lo) * 0.5;
    let mut area = Vec3::new(0.0, 0.0, 0.0);
    for (i, p) in pts.iter().enumerate() {
        let q = pts[(i + 1) % pts.len()];
        area = area + (*p - o).cross(q - o);
    }
    area
}

/// The PR's band: south pole, up the meridian at `a0`, north pole,
/// down at `a1`; sphere radius `r` centred at `c` about +z.
fn band_cycle(r: f64, c: Point3<f64>, a0: f64, a1: f64, n: usize) -> Vec<Point3<f64>> {
    let on = |a: f64, t: f64| {
        Point3::new(
            c.x + r * a.cos() * t.sin(),
            c.y + r * a.sin() * t.sin(),
            c.z + r * t.cos(),
        )
    };
    let step = core::f64::consts::PI / (n as f64);
    let mut pts = vec![Point3::new(c.x, c.y, c.z - r)];
    pts.extend((1..n).map(|k| on(a0, core::f64::consts::PI - (k as f64) * step)));
    pts.push(Point3::new(c.x, c.y, c.z + r));
    pts.extend((1..n).map(|k| on(a1, (k as f64) * step)));
    pts
}

fn placed_band(d: f64) -> Vec<Point3<f64>> {
    band_cycle(
        1.3e-3,
        Point3::new(1.3 * d, -2.7 * d, 0.9 * d),
        0.37,
        2.27,
        8,
    )
}

/// Chart-frame azimuth of an area vector, u_ref = +x, v_ref = +y
/// (the z_chart the PR's rows use; sense_sign omitted as there).
fn az(area: Vec3<f64>) -> f64 {
    area.dot(Vec3::new(0.0, 1.0, 0.0))
        .atan2(area.dot(Vec3::new(1.0, 0.0, 0.0)))
}

/// Reproduce the PR's drift table with BOTH spellings in one run.
/// Asserts the direction of the red-first claim: origin-anchored over
/// budget at EVERY row (the PR says it misses the first row already),
/// whole-radians wrong at 1e6; loop-anchored within budget everywhere.
#[test]
fn r2_the_drift_table_reproduces() {
    let old0 = az(origin_area(&placed_band(0.0)));
    let new0 = az(local_area(&placed_band(0.0)));
    for d in [1e2, 1e3, 1e4, 1e5, 1e6, 1e7, 1e8] {
        let old_drift = (az(origin_area(&placed_band(d))) - old0).abs();
        let new_drift = (az(local_area(&placed_band(d))) - new0).abs();
        let budget = 1e-11 * d;
        println!("d {d:e}: origin {old_drift:.3e}  local {new_drift:.3e}  budget {budget:e}");
        assert!(
            old_drift > budget,
            "origin spelling within budget at {d:e}: {old_drift:e}"
        );
        assert!(
            new_drift < budget,
            "local spelling over budget at {d:e}: {new_drift:e}"
        );
        if d == 1e6 {
            assert!(
                old_drift > 3.0,
                "1e6 row not whole-radians red: {old_drift}"
            );
        }
    }
}

/// The consequence: at 1e6 the origin spelling's azimuth moves the
/// `unwrap_near` branch a whole 2π; the local spelling never moves it.
#[test]
fn r2_the_branch_flip_reproduces() {
    let old0 = az(origin_area(&placed_band(0.0)));
    let new0 = az(local_area(&placed_band(0.0)));
    let mut flipped = 0;
    for d in [1e2, 1e3, 1e4, 1e5, 1e6, 1e7, 1e8] {
        let old_far = az(origin_area(&placed_band(d)));
        let new_far = az(local_area(&placed_band(d)));
        for u_raw in [0.37, 2.27] {
            let old_gap = (unwrap_near(u_raw, old_far) - unwrap_near(u_raw, old0)).abs();
            if old_gap > 1.0 {
                flipped += 1;
                println!("d {d:e} u_raw {u_raw}: origin spelling {old_gap:.3} apart");
            }
            let new_gap = (unwrap_near(u_raw, new_far) - unwrap_near(u_raw, new0)).abs();
            assert!(new_gap < 1e-9, "local spelling flipped at {d:e}: {new_gap}");
        }
    }
    assert!(flipped > 0, "origin spelling never flipped a branch");
}

/// Conditioning-only at ordinary placements: where the old spelling
/// was well-conditioned, the two anchors give the same direction to
/// well under any decision margin. This is the PR's "no value moves
/// that was previously right" claim, executed.
#[test]
fn r2_ordinary_placements_agree_between_anchors() {
    for d in [0.0, 1e-3, 1.0, 10.0] {
        for (a0, a1) in [(0.37, 2.27), (0.0, 3.0), (-1.2, 1.9)] {
            let pts = band_cycle(1.3e-3, Point3::new(1.3 * d, -2.7 * d, 0.9 * d), a0, a1, 8);
            let gap = (az(origin_area(&pts)) - az(local_area(&pts))).abs();
            println!("d {d:e} band ({a0},{a1}): anchor gap {gap:.3e} rad");
            // Decision-relevant margin: a branch pick moves only past
            // ~π/2 of azimuth; 1e-6 rad is ~6 orders inside it. (At
            // d = 1 a mm band's origin fold already carries ~7e-11
            // rad of anchor noise — measured — so a bit-agreement
            // budget is not the right assertion here.)
            assert!(
                gap < 1e-6,
                "anchors disagree at ordinary placement: {gap:e}"
            );
        }
    }
}

/// End to end through the public API: a unit ball revolved about an
/// axis ~3e3 m from the world origin, tessellated, volume checked
/// against 4π/3. The ball's two faces are rimless pole-to-pole bands,
/// so this crosses the fixed fold on the real path.
#[test]
fn r2_e2e_placed_ball_volume_is_the_balls() {
    let (cx, cy) = (1.3e3, -2.7e3);
    let lp = ProfileLoop::new(vec![
        ProfileVertex::new(Point2::new(cx, cy - 1.0), 1.0),
        ProfileVertex::new(Point2::new(cx, cy + 1.0), 0.0),
    ]);
    let vp = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(Tol::witness())
        .expect("profile");
    let axis = RevolveAxis {
        origin: Point2::new(cx, cy),
        dir: Vec2::new(0.0, 1.0),
    };
    let t = revolve(&vp, axis, Revolution::Full, Tol::witness()).expect("revolve");
    let mesh = mesh::tessellate(&t.body, 0.01, Tol::witness()).expect("tessellate");
    assert_eq!(check_mesh(&mesh), Ok(()));
    let v = signed_volume(&mesh);
    let exact = 4.0 * core::f64::consts::PI / 3.0;
    println!("placed ball volume {v} exact {exact}");
    assert!(v > 0.0, "volume {v} not positive");
    assert!((v - exact).abs() < 0.1, "volume {v} vs exact {exact}");
}
