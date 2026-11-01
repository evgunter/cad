//! **A tilted non-convex cap is pierced where it pierces**: a prism's
//! cap with a V notch, a spike, a slot or a shallow reflex vertex,
//! turned about a random axis, its stored origin moved along its plane,
//! crossed by a grazing edge aimed near the feature's tip. Each crossing
//! is measured exactly and read against the profile; beside it, a sweep
//! that reads every slope at origins out to 100 km for panics.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use super::*;
use geom_core::{Interval, RigidMotion, Tol, UnitVec3};
use test_utils::fuzz::Rng;

/// A profile with a feature point to aim at.
fn profile(kind: usize, w: f64) -> (Vec<(f64, f64)>, (f64, f64)) {
    match kind {
        // V notch, tip at (0.5, 0.5).
        0 => (
            vec![
                (0.0, 0.0),
                (0.5 - w, 0.0),
                (0.5, 0.5),
                (0.5 + w, 0.0),
                (1.0, 0.0),
                (1.0, 1.0),
                (0.0, 1.0),
            ],
            (0.5, 0.5),
        ),
        // A spike of material, tip at (0.5, 0.9).
        1 => (
            vec![
                (0.0, 0.0),
                (1.0, 0.0),
                (1.0, 0.4),
                (0.5 + w, 0.4),
                (0.5, 0.9),
                (0.5 - w, 0.4),
                (0.0, 0.4),
            ],
            (0.5, 0.9),
        ),
        // A slot of width 2w from the top, its bottom at y = 0.3.
        2 => (
            vec![
                (0.0, 0.0),
                (1.0, 0.0),
                (1.0, 1.0),
                (0.5 + w, 1.0),
                (0.5 + w, 0.3),
                (0.5 - w, 0.3),
                (0.5 - w, 1.0),
                (0.0, 1.0),
            ],
            (0.5 + w, 0.3),
        ),
        // A shallow reflex vertex at (0.5, 0.5 - w): a vertex the span can hold.
        _ => (
            vec![
                (0.0, 0.0),
                (1.0, 0.0),
                (1.0, 0.5),
                (0.5, 0.5 - w),
                (0.0, 0.5),
            ],
            (0.5, 0.5 - w),
        ),
    }
}

/// Signed distance of `(x, y)` into the polygon (negative outside).
fn signed(poly: &[(f64, f64)], (x, y): (f64, f64)) -> f64 {
    let mut inside = false;
    let mut d = f64::INFINITY;
    for i in 0..poly.len() {
        let (a, b) = (poly[i], poly[(i + 1) % poly.len()]);
        if (a.1 > y) != (b.1 > y) && x < a.0 + (y - a.1) * (b.0 - a.0) / (b.1 - a.1) {
            inside = !inside;
        }
        let (ux, uy) = (b.0 - a.0, b.1 - a.1);
        let t = (((x - a.0) * ux + (y - a.1) * uy) / (ux * ux + uy * uy)).clamp(0.0, 1.0);
        d = d.min(((x - a.0 - t * ux).powi(2) + (y - a.1 - t * uy).powi(2)).sqrt());
    }
    if inside { d } else { -d }
}

#[derive(Clone, Copy, Debug)]
struct Pose {
    kind: usize,
    w: f64,
    axis: [f64; 3],
    angle: f64,
    far: (f64, f64),
    aim: (f64, f64),
    theta: f64,
    half: f64,
    off: f64,
}

fn local_ends(p: &Pose) -> [Point3<f64>; 2] {
    let (c, s) = (p.theta.cos(), p.theta.sin());
    [
        Point3::new(p.aim.0 - p.half * c, p.aim.1 - p.half * s, 1.0 + p.off),
        Point3::new(p.aim.0 + p.half * c, p.aim.1 + p.half * s, 1.0 - p.off),
    ]
}

fn motion<T: Decide>(p: &Pose) -> RigidMotion<T> {
    let band = Band::new(1e-9, 1e-8).unwrap();
    let ax = Vec3::new(p.axis[0], p.axis[1], p.axis[2]).map(T::from_f64);
    let ax = UnitVec3::new(ax, "tilted cap", band).unwrap();
    RigidMotion::rotation(ax, T::from_f64(p.angle)).unwrap()
}

/// What a pose read: the pair's finding, the world ends, and the
/// stored plane's origin and normal.
type PoseRead<T> = (Found, [Point3<T>; 2], Point3<T>, Vec3<T>);

/// The pair read at `T`, the world ends, and the stored plane; `None`
/// where the turned prism does not certify at the run's ε (at `Interval`,
/// whose rotation's enclosures pass 1e-12).
fn read_pose<T: Decide + crate::props::AtRestPolicy>(p: &Pose, band: Band) -> Option<PoseRead<T>> {
    let (poly, _) = profile(p.kind, p.w);
    let prism = crate::test_support_fixtures::prism_z::<T>(&poly, 0.0, 1.0, Tol::witness());
    let m = motion::<T>(p);
    let mut body = crate::transform_rigid(&prism.body, &m, Tol::witness()).ok()?;
    let face = prism.top_face;
    let o = m.transform_point(Point3::new(p.far.0, p.far.1, 1.0).map(T::from_f64));
    move_origin(&mut body, face, o);
    let ends = local_ends(p).map(|q| m.transform_point(q.map(T::from_f64)));
    let pair = Pair::new(&body, face, ends);
    let n = pair.f().normal;
    Some((found(&pair.read(band)), ends, o, n))
}

/// The exact crossing of the stored plane with the stored ends, in the
/// profile's local coordinates (to a few ulps), and its signed distance
/// into the polygon.
fn truth(p: &Pose, ends: [Point3<f64>; 2], o: Point3<f64>, n: Vec3<f64>) -> f64 {
    let x = exact_crossing(o, n, ends).expect("an edge across the plane") - Point3::origin();
    let m = motion::<f64>(p);
    let col = |v: Vec3<f64>| {
        let c = m.transform_point(Point3::origin() + v) - Point3::origin();
        c.dot(x)
    };
    let local = (col(Vec3::new(1.0, 0.0, 0.0)), col(Vec3::new(0.0, 1.0, 0.0)));
    signed(&profile(p.kind, p.w).0, local)
}

fn draw(r: &mut Rng, eps: f64, far_max: f64) -> Pose {
    let sign = |r: &mut Rng| if r.below(2) == 0 { 1.0 } else { -1.0 };
    let kind = r.below(4);
    let w = r.log_uniform(1e-5, 1e-2);
    let (_, tip) = profile(kind, w);
    let axis = {
        let v = [r.range(-1.0, 1.0), r.range(-1.0, 1.0), r.range(-1.0, 1.0)];
        let l = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
        [v[0] / l, v[1] / l, v[2] / l]
    };
    let scale = r.log_uniform(1e-3, 30.0) * w;
    Pose {
        kind,
        w,
        axis,
        angle: r.range(0.1, 3.0),
        far: (
            sign(r) * r.log_uniform(0.1, far_max),
            sign(r) * r.log_uniform(0.1, far_max),
        ),
        aim: (
            tip.0 + sign(r) * r.range(0.0, scale),
            tip.1 + sign(r) * r.range(0.0, scale),
        ),
        theta: r.range(0.0, std::f64::consts::TAU),
        half: r.range(0.05, 0.3),
        off: r.log_uniform(1.5 * 10.0 * eps, 1e3 * 10.0 * eps),
    }
}

/// **A grazing edge across a tilted non-convex cap is pierced where it
/// pierces**: every pose drawn, at each ε, read right or refused at
/// `f64` and at [`Interval`]. A pose within `2Kε` of the profile's
/// boundary is not read.
#[test]
fn a_grazing_edge_across_a_tilted_non_convex_cap_is_read_pierced_where_it_pierces() {
    let label = "census_tilted_cap_pierce";
    let mut r = test_utils::fuzz::start(label);
    let mut bad = Vec::new();
    for eps in EPS {
        let band = Band::new(eps, K * eps).unwrap();
        let (mut read, mut held, mut held_iv, mut unbuilt) = (0usize, 0usize, 0usize, 0usize);
        for _ in 0..test_utils::fuzz::scaled(COUNT / 2) {
            let pose = draw(&mut r, eps, 40.0);
            let Some((f, ends, o, n)) = read_pose::<f64>(&pose, band) else {
                unbuilt += 1;
                continue;
            };
            let inside = truth(&pose, ends, o, n);
            let margin = 2.0 * band.escalate();
            let want = if inside >= margin {
                Found::Pierce
            } else if inside <= -margin {
                Found::Nothing
            } else {
                continue;
            };
            let iv = match read_pose::<Interval>(&pose, band) {
                Some((iv, ..)) => iv,
                None => {
                    unbuilt += 1;
                    Found::Refused
                }
            };
            read += 1;
            held += usize::from(f == Found::Refused);
            held_iv += usize::from(iv == Found::Refused);
            let wrong = |got: Found| got != Found::Refused && got != want;
            if wrong(f) || wrong(iv) {
                bad.push(format!(
                    "ε {eps:e} {pose:?}: {inside:e} inside, f64 {f:?}, Interval {iv:?}"
                ));
            }
        }
        println!(
            "[fuzz] {label} at ε {eps:e}: {read} poses read, {held} refused at f64, {held_iv} at \
             Interval; {unbuilt} turned prisms that do not certify"
        );
    }
    assert!(
        bad.is_empty(),
        "{label}: {} readings wrong; {}:\n{}",
        bad.len(),
        test_utils::fuzz::replay(),
        bad[..bad.len().min(8)].join("\n")
    );
}

/// **No pose panics either scalar**: the same caps, origins out to
/// 100 km, the edge in the plane, within the band of it, grazing,
/// steep, or near-normal, at every ε.
#[test]
fn a_tilted_cap_read_at_any_slope_and_origin_does_not_panic() {
    let label = "census_tilted_cap_no_panic";
    let mut r = test_utils::fuzz::start(label);
    for eps in EPS {
        let band = Band::new(eps, K * eps).unwrap();
        for _ in 0..test_utils::fuzz::scaled(COUNT / 4) {
            let mut pose = draw(&mut r, eps, 1e5);
            pose.off = match r.below(5) {
                0 => 0.0,
                1 => r.log_uniform(1e-18, eps),
                2 => r.log_uniform(eps, 20.0 * eps),
                3 => r.log_uniform(1e-6, 0.3),
                _ => r.log_uniform(0.3, 1e3),
            } * if r.below(2) == 0 { 1.0 } else { -1.0 };
            // A panic fails the row; the seed is in the log above.
            let _ = read_pose::<f64>(&pose, band);
            let _ = read_pose::<Interval>(&pose, band);
        }
    }
    println!("[fuzz] {label}: no panic; {}", test_utils::fuzz::replay());
}
