//! Review 2 probes of the edge–face pierce: tilted non-convex caps
//! (notch tips, spikes, slots, a vertex on the span) read at `f64`
//! against the exact crossing, and a panic sweep at both scalars with
//! origins out to 100 km. Run with
//! `cargo nextest run -p topo review2 --run-ignored all --no-capture`.
use super::*;
use geom_core::{Interval, RigidMotion, Tol, UnitVec3};
use num_bigint::BigInt;
use test_utils::fuzz::Rng;

use super::super::segment_fuzz::{big, big_dot, ratio};

fn classify(errors: &[ValidationError]) -> Found {
    if errors.iter().any(|e| {
        matches!(
            e,
            ValidationError::CensusEscalated { .. } | ValidationError::CensusUnsupported { .. }
        )
    }) {
        Found::Refused
    } else if errors.is_empty() {
        Found::Nothing
    } else {
        Found::Pierce
    }
}

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
    let ax = UnitVec3::new(ax, "review2", band).unwrap();
    RigidMotion::rotation(ax, T::from_f64(p.angle)).unwrap()
}

/// The pair read at `T`, the world ends, and the stored plane (at `f64`).
fn read<T: Decide + crate::props::AtRestPolicy>(
    p: &Pose,
    band: Band,
) -> (Found, [Point3<T>; 2], Point3<T>, Vec3<T>) {
    let (poly, _) = profile(p.kind, p.w);
    let prism = crate::test_support_fixtures::prism_z::<T>(&poly, 0.0, 1.0, Tol::witness());
    let m = motion::<T>(p);
    let mut body = crate::transform_rigid(&prism.body, &m, Tol::witness()).unwrap();
    let face = prism.top_face;
    let s = body.get_face(face).unwrap().surface;
    let (o, n) = match body.surfaces.get_mut(s).unwrap() {
        geom::Surface::Plane { origin, normal, .. } => {
            *origin = m.transform_point(Point3::new(p.far.0, p.far.1, 1.0).map(T::from_f64));
            (*origin, *normal)
        }
        _ => panic!("a planar cap"),
    };
    let ends = local_ends(p).map(|q| m.transform_point(q.map(T::from_f64)));
    let geo = snapshot(&body);
    let f = geo.faces.iter().find(|f| f.key == face).unwrap();
    let e = segment((ends[0], ends[1]));
    let declared = Declared::index(&geo, &ContactRecords::default());
    let mut errors = Vec::new();
    pair_edge_face(&body, &geo, &declared, band, None, &e, f, &mut errors);
    (classify(&errors), ends, o, n)
}

/// The exact crossing of the stored plane with the stored ends, in the
/// profile's local coordinates (to a few ulps), and its signed distance
/// into the polygon.
fn truth(p: &Pose, [p0, p1]: [Point3<f64>; 2], o: Point3<f64>, n: Vec3<f64>) -> f64 {
    let bn = [n.x, n.y, n.z].map(big);
    let r = |q: Point3<f64>| big_dot(&[q.x, q.y, q.z].map(big), &bn) - big_dot(&[o.x, o.y, o.z].map(big), &bn);
    let (r0, r1) = (r(p0), r(p1));
    let mut den = &r0 - &r1;
    let mut r0 = r0;
    if den < BigInt::from(0) {
        den = -den;
        r0 = -r0;
    }
    let x = [0, 1, 2].map(|i| {
        let (a, b) = ([p0.x, p0.y, p0.z][i], [p1.x, p1.y, p1.z][i]);
        let num = big(a) * &den + (big(b) - big(a)) * &r0;
        let d = &den << 1074_usize;
        if num < BigInt::from(0) { -ratio(&-num, &d) } else { ratio(&num, &d) }
    });
    let m = motion::<f64>(p);
    let col = |v: Vec3<f64>| {
        let c = m.transform_point(Point3::origin() + v) - Point3::origin();
        c.x * x[0] + c.y * x[1] + c.z * x[2]
    };
    let local = (
        col(Vec3::new(1.0, 0.0, 0.0)),
        col(Vec3::new(0.0, 1.0, 0.0)),
    );
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
        far: (sign(r) * r.log_uniform(0.1, far_max), sign(r) * r.log_uniform(0.1, far_max)),
        aim: (tip.0 + sign(r) * r.range(0.0, scale), tip.1 + sign(r) * r.range(0.0, scale)),
        theta: r.range(0.0, std::f64::consts::TAU),
        half: r.range(0.05, 0.3),
        off: r.log_uniform(1.5 * 10.0 * eps, 1e3 * 10.0 * eps),
    }
}

/// Verdicts at `f64` on tilted non-convex caps, against the exact crossing.
#[test]
#[ignore = "review 2 probe"]
fn review2_tilted_nonconvex_caps_read_right_at_f64() {
    let label = "review2_tilted";
    let mut r = test_utils::fuzz::start(label);
    let mut bad = Vec::new();
    let k: f64 = std::env::var("REVIEW2_K").ok().and_then(|v| v.parse().ok()).unwrap_or(10.0);
    for eps in [1e-6, 1e-9, 1e-12] {
        let band = Band::new(eps, k * eps).unwrap();
        let (mut n, mut held, mut skip, mut iv_bad) = (0, 0, 0, 0);
        for _ in 0..test_utils::fuzz::scaled(200) {
            let pose = draw(&mut r, eps, 40.0);
            let (got, ends, o, nn) = read::<f64>(&pose, band);
            let inside = truth(&pose, ends, o, nn);
            let margin = 2.0 * band.escalate() + 1e-13;
            let want = if inside >= margin {
                Found::Pierce
            } else if inside <= -margin {
                Found::Nothing
            } else {
                skip += 1;
                continue;
            };
            n += 1;
            held += usize::from(got == Found::Refused);
            if got != Found::Refused && got != want {
                bad.push(format!("ε {eps:e} {pose:?}: {inside:e} inside, f64 {got:?}"));
            }
            let (iv, ..) = read::<Interval>(&pose, band);
            if iv != Found::Refused && iv != want {
                iv_bad += 1;
            }
        }
        println!("[review2] ε {eps:e}: {n} read, {held} refused, {skip} near, Interval disagreeing with truth {iv_bad}");
    }
    assert!(bad.is_empty(), "{} wrong; {}:\n{}", bad.len(), test_utils::fuzz::replay(), bad.join("\n"));
}

/// No panic at either scalar: origins to 100 km, edges in the plane,
/// at every grazing slope, steep, and at all three ε.
#[test]
#[ignore = "review 2 probe"]
fn review2_no_panic_far_origins_any_slope() {
    let label = "review2_panic";
    let mut r = test_utils::fuzz::start(label);
    let mut panics = Vec::new();
    let mut tally = std::collections::BTreeMap::new();
    for eps in [1e-6, 1e-9, 1e-12] {
        let band = Band::new(eps, 10.0 * eps).unwrap();
        for _ in 0..test_utils::fuzz::scaled(150) {
            let mut pose = draw(&mut r, eps, 1e5);
            pose.off = match r.below(5) {
                0 => 0.0,
                1 => r.log_uniform(1e-18, eps),
                2 => r.log_uniform(eps, 20.0 * eps),
                3 => r.log_uniform(1e-6, 0.3),
                _ => r.log_uniform(0.3, 1e3),
            } * if r.below(2) == 0 { 1.0 } else { -1.0 };
            for scalar in 0..2 {
                let got = std::panic::catch_unwind(|| {
                    if scalar == 0 {
                        read::<f64>(&pose, band).0
                    } else {
                        read::<Interval>(&pose, band).0
                    }
                });
                match got {
                    Ok(f) => *tally.entry((scalar, format!("{f:?}"))).or_insert(0) += 1,
                    Err(_) => panics.push(format!("ε {eps:e} scalar {scalar} {pose:?}")),
                }
            }
        }
    }
    println!("[review2] tally {tally:?}");
    assert!(panics.is_empty(), "{} panics; {}:\n{}", panics.len(), test_utils::fuzz::replay(), panics.join("\n"));
}

/// A square hole narrower than the span: the block `[0, 3] × [0, 2] ×
/// [0, 2]` holed at `x ∈ [1, 2]`, `y ∈ [0.5, 1.5]`, its top's stored
/// origin 1 km off, an 8 m edge grazing the top across the hole and
/// crossing it at the hole's centre. Both span ends stand on material
/// (`In`); the crossing is in the hole, so no pierce.
#[test]
fn review2_a_hole_narrower_than_the_span_is_no_pierce() {
    let probe = crate::test_support_fixtures::holed_block::<f64>(3.0, &[1.5], Tol::witness());
    let top = probe
        .faces()
        .find(|(_, f)| {
            matches!(probe.get_surface(f.surface),
                Some(geom::Surface::Plane { origin, normal, .. })
                    if normal.z.abs() > 0.5 && origin.z == 2.0)
        })
        .map(|(k, _)| k)
        .unwrap();
    fn go<T: Decide + crate::props::AtRestPolicy>(top: FaceKey, off: f64) -> (Found, Vec<String>) {
        let band = Band::new(1e-12, 1e-11).unwrap();
        let mut body = crate::test_support_fixtures::holed_block::<T>(3.0, &[1.5], Tol::witness());
        let s = body.get_face(top).unwrap().surface;
        match body.surfaces.get_mut(s).unwrap() {
            geom::Surface::Plane { origin, .. } => {
                *origin = Point3::new(700.0, -700.0, 2.0).map(T::from_f64);
            }
            _ => panic!(),
        }
        let geo = snapshot(&body);
        let f = geo.faces.iter().find(|f| f.key == top).unwrap();
        assert!(f.polygon && f.loops.len() == 2, "a polygon with one ring");
        let e = segment((
            Point3::new(-2.5, 1.0, 2.0 + off).map(T::from_f64),
            Point3::new(5.5, 1.0, 2.0 - off).map(T::from_f64),
        ));
        let declared = Declared::index(&geo, &ContactRecords::default());
        let mut errors = Vec::new();
        pair_edge_face(&body, &geo, &declared, band, None, &e, f, &mut errors);
        let said = errors.iter().map(|e| format!("{e:?}").chars().take(160).collect()).collect();
        (classify(&errors), said)
    }
    for off in [1.2e-11, 1.5e-11, 2e-11, 3e-11] {
        let (a, ea) = go::<f64>(top, off);
        let (b, eb) = go::<Interval>(top, off);
        println!("[review2-hole] off {off:e}: f64 {a:?} {ea:?}\n   Interval {b:?} {eb:?}");
        assert!(a != Found::Pierce && b != Found::Pierce, "a pierce read inside the hole at off {off:e}");
    }
}
