//! Adversarial e2e review artifact for M0 PR 6 (linalg layer,
//! 2026-07-15/16), salvaged from the M0 orchestrator session scratchpad
//! and promoted into CI 2026-07-16 (archived at
//! `references/review-artifacts-m0/pr6-review-demos/`). These are
//! **independent derivations** — do not "simplify" them to match shipped
//! fixtures; the independence is the regression value.
//!
//! M2-shaped consumer exercises: the generic functions are written
//! against `Real` alone — the way real M2 evaluation code will be — and
//! only the f64 call sites assert (raw comparisons are legal on concrete
//! f64 per the evaluation-code discipline).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Affine3, Mat3, Point2, Point3, Real, Vec3};

// ---------------------------------------------------------------- shared

/// Embeds a 2-D sketch point into 3-space through a sketch-plane frame:
/// `origin + u·x + v·y`. This is exactly the M2 "sketch plane" shape:
/// the frame is an Affine3 whose linear columns are the plane's in-plane
/// basis and normal, translation the plane origin.
fn embed<T: Real>(frame: Affine3<T>, p: Point2<T>) -> Point3<T> {
    frame.transform_point(Point3::new(p.x, p.y, T::zero()))
}

/// Triangle normal via cross of edge differences (Point Sub), normalized.
fn triangle_normal<T: Real>(a: Point3<T>, b: Point3<T>, c: Point3<T>) -> Vec3<T> {
    ((b - a).cross(c - a)).normalize()
}

fn assert_close(label: &str, got: f64, want: f64, tol: f64) {
    assert!(
        (got - want).abs() <= tol,
        "{label}: got {got}, want {want} (tol {tol})"
    );
}

// ------------------------------------------------------------ scenario 1

/// Extrude-like construction: an L-profile in the sketch plane, embedded
/// via a rigid frame (rotation about a skew axis + translation), then the
/// top cap translated along the extrude direction. Rigid maps must
/// preserve lengths and angles within rounding-scale bounds.
#[test]
fn scenario_extrude() {
    let profile: Vec<Point2<f64>> = vec![
        Point2::new(0.0, 0.0),
        Point2::new(0.04, 0.0),
        Point2::new(0.04, 0.01),
        Point2::new(0.01, 0.01),
        Point2::new(0.01, 0.03),
        Point2::new(0.0, 0.03),
    ];

    // Sketch plane: rotate 0.7 rad about a skew axis, then move out to
    // (0.5, -0.25, 1.0). Rigid, so an isometry on the sketch.
    let frame = Affine3::from_parts(
        Mat3::rotation_about(Vec3::new(1.0, 2.0, -0.5), 0.7),
        Vec3::new(0.5, -0.25, 1.0),
    );
    let bottom: Vec<Point3<f64>> = profile.iter().map(|&p| embed(frame, p)).collect();

    // Extrude direction: the sketch normal (pushforward of e_z), length 0.05.
    let dir = frame.transform_vec(Vec3::new(0.0, 0.0, 1.0)) * 0.05;
    let lift = Affine3::translation(dir);
    let top: Vec<Point3<f64>> = bottom.iter().map(|&p| lift.transform_point(p)).collect();

    // (a) Edge lengths preserved sketch -> bottom (rigid embed).
    for i in 0..profile.len() {
        let j = (i + 1) % profile.len();
        let l2 = profile[i].distance(profile[j]);
        let l3 = bottom[i].distance(bottom[j]);
        // coords ~1, ~10 roundings of EPSILON scale: 1e-12 is generous.
        assert_close(&format!("edge {i} length embed"), l3, l2, 1e-12);
    }
    // (b) Interior angles preserved (dot of unit edge vectors).
    for i in 0..profile.len() {
        let h = (i + profile.len() - 1) % profile.len();
        let j = (i + 1) % profile.len();
        let a2 = {
            let u = (profile[h] - profile[i]).normalize();
            let v = (profile[j] - profile[i]).normalize();
            u.dot(v)
        };
        let a3 = {
            let u = (bottom[h] - bottom[i]).normalize();
            let v = (bottom[j] - bottom[i]).normalize();
            u.dot(v)
        };
        assert_close(&format!("corner {i} cos angle"), a3, a2, 1e-9);
    }
    // (c) Lateral edges: |top_i - bottom_i| == 0.05 for every i, and
    // parallel to dir.
    let dn = dir.normalize();
    for i in 0..bottom.len() {
        let lat = top[i] - bottom[i];
        assert_close(&format!("lateral {i} length"), lat.norm(), 0.05, 1e-15);
        assert_close(
            &format!("lateral {i} parallelism"),
            lat.normalize().dot(dn),
            1.0,
            1e-12,
        );
    }
    // (d) Round trip through the inverse frame recovers the sketch.
    let back = frame.inverse();
    for (i, &b) in bottom.iter().enumerate() {
        let q = back.transform_point(b);
        assert_close(&format!("roundtrip {i} x"), q.x, profile[i].x, 1e-12);
        assert_close(&format!("roundtrip {i} y"), q.y, profile[i].y, 1e-12);
        assert_close(&format!("roundtrip {i} z"), q.z, 0.0, 1e-12);
    }
}

// ------------------------------------------------------------ scenario 2

/// Revolve-like construction: sweep a profile point about an axis through
/// an anchor point via Mat3::rotation_about at several angles.
#[test]
fn scenario_revolve() {
    let axis_dir = Vec3::new(0.3, 1.0, 0.2);
    let anchor = Point3::new(0.1, -0.2, 0.4);
    let profile_pt = Point3::new(0.6, 0.1, 0.15);

    let n = axis_dir.normalize();
    let arm = profile_pt - anchor;
    // Reference geometry of the swept circle.
    let axial = n * arm.dot(n); // component along the axis
    let radial = arm - axial;
    let radius = radial.norm();
    let height = arm.dot(n);

    let steps = 17u32;
    let mut swept = Vec::new();
    for k in 0..steps {
        let theta = f64::from(k) * (core::f64::consts::TAU / f64::from(steps));
        let rot = Mat3::rotation_about(axis_dir, theta);
        // rotate the arm about the axis at the anchor
        let p = anchor + rot * arm;
        swept.push(p);
    }
    for (k, &p) in swept.iter().enumerate() {
        let v = p - anchor;
        // (a) constant height along the axis => plane perpendicular to axis
        assert_close(&format!("step {k} axial height"), v.dot(n), height, 1e-13);
        // (b) constant distance to the axis line
        let d = (v - n * v.dot(n)).norm();
        assert_close(&format!("step {k} radius"), d, radius, 1e-13);
    }
    // (c) full turn returns to start
    let full = Mat3::rotation_about(axis_dir, core::f64::consts::TAU);
    let back = anchor + full * arm;
    assert_close("full-turn x", back.x, profile_pt.x, 1e-14);
    assert_close("full-turn y", back.y, profile_pt.y, 1e-14);
    assert_close("full-turn z", back.z, profile_pt.z, 1e-14);
}

// ------------------------------------------------------------ scenario 3

/// Tessellation-ish loop: subdivide a segment with lerp, transform, fold a
/// bbox with componentwise min/max; then poison a coordinate mid-stream
/// and confirm the NaN reaches the bbox (no laundering).
#[test]
fn scenario_tessellate_bbox() {
    let a = Point3::new(-0.3, 0.2, 0.9);
    let b = Point3::new(0.7, -0.4, 0.1);
    let map = Affine3::from_parts(
        Mat3::rotation_about(Vec3::new(0.0, 0.0, 1.0), 1.1),
        Vec3::new(10.0, 20.0, 30.0),
    );

    let n = 32;
    let pts: Vec<Point3<f64>> = (0..=n)
        .map(|i| {
            let t = f64::from(i) / f64::from(n);
            map.transform_point(a.lerp(b, t))
        })
        .collect();

    // Clean bbox first.
    let (mut lo, mut hi) = (pts[0], pts[0]);
    for &p in &pts[1..] {
        lo = lo.min(p);
        hi = hi.max(p);
    }
    assert!(lo.x <= hi.x && lo.y <= hi.y && lo.z <= hi.z);
    // Endpoint sanity: bbox contains images of a and b.
    let ia = map.transform_point(a);
    assert!(lo.x <= ia.x && ia.x <= hi.x);

    // Poison one coordinate mid-stream (as a degenerate normalize would).
    let mut poisoned = pts.clone();
    poisoned[13].y = Vec3::<f64>::zero().normalize().y; // NaN via the API itself
    let (mut plo, mut phi) = (poisoned[0], poisoned[0]);
    for &p in &poisoned[1..] {
        plo = plo.min(p);
        phi = phi.max(p);
    }
    assert!(
        plo.y.is_nan() && phi.y.is_nan(),
        "NaN must reach the bbox y bounds (no laundering)"
    );
    assert!(!plo.x.is_nan() && !phi.z.is_nan(), "other axes stay clean");
}

// ------------------------------------------------------------ scenario 4

/// Normal computation: triangle normal via cross of edge differences,
/// normalized; a collinear triangle yields a poison normal, and the
/// poison persists through dot/norm (predicate layer not on this branch).
#[test]
fn scenario_normals() {
    let a = Point3::new(0.0, 0.0, 0.0);
    let b = Point3::new(1.0, 0.0, 0.0);
    let c = Point3::new(0.0, 1.0, 0.0);
    let nrm = triangle_normal(a, b, c);
    assert_close("unit normal z", nrm.z, 1.0, 1e-15);
    assert_close("unit normal norm", nrm.norm(), 1.0, 1e-15);

    // Collinear triangle: b and c on the same ray from a.
    let c2 = Point3::new(2.0, 0.0, 0.0);
    let bad = triangle_normal(a, b, c2);
    assert!(
        bad.x.is_nan() && bad.y.is_nan() && bad.z.is_nan(),
        "collinear triangle must give poison normal"
    );
    // Poison persists through dot and norm — no laundering.
    assert!(bad.dot(Vec3::unit_x()).is_nan());
    assert!(bad.norm().is_nan());
    assert!(bad.norm_squared().is_nan());
    // ... and through min/max folds (Real::min NaN propagation).
    assert!(bad.min(Vec3::zero()).x.is_nan());
    assert!(bad.max(Vec3::zero()).x.is_nan());
}

// A generic consumer compiled at T = f64 only, proving the module is
// usable with no bounds beyond Real (evaluation-code discipline).
fn centroid_offset<T: Real>(pts: &[Point3<T>], anchor: Point3<T>) -> Vec3<T> {
    let mut acc = Vec3::zero();
    let mut count = T::zero();
    for &p in pts {
        acc = acc + (p - anchor);
        count = count + T::one();
    }
    acc / count
}

#[test]
fn generic_consumer_at_f64() {
    let pts = [
        Point3::new(1.0, 0.0, 0.0),
        Point3::new(0.0, 1.0, 0.0),
        Point3::new(0.0, 0.0, 1.0),
    ];
    let g = centroid_offset(&pts, Point3::origin());
    assert_close("centroid x", g.x, 1.0 / 3.0, 1e-15);
}

// --------------------------------------------- numeric spot-check probes

/// Rodrigues hand cases + zero-axis poison (the review's `probe` binary).
#[test]
fn rotation_spot_checks_and_degenerate_poison() {
    // 90 degrees about z: e_x -> e_y, e_y -> -e_x.
    let r = Mat3::rotation_about(Vec3::new(0.0, 0.0, 1.0), core::f64::consts::FRAC_PI_2);
    let ex = r * Vec3::unit_x();
    let ey = r * Vec3::unit_y();
    assert!((ex.x).abs() < 1e-15 && (ex.y - 1.0).abs() < 1e-15 && ex.z == 0.0);
    assert!((ey.x + 1.0).abs() < 1e-15 && (ey.y).abs() < 1e-15);

    // 120 degrees about (1,1,1)/sqrt(3): cyclic permutation x->y->z->x.
    let r = Mat3::rotation_about(Vec3::new(1.0, 1.0, 1.0), 2.0 * core::f64::consts::FRAC_PI_3);
    for (v, want, name) in [
        (Vec3::unit_x(), Vec3::unit_y(), "e_x -> e_y"),
        (Vec3::unit_y(), Vec3::unit_z(), "e_y -> e_z"),
        (Vec3::unit_z(), Vec3::unit_x(), "e_z -> e_x"),
    ] {
        let got = r * v;
        let d = (got - want).norm();
        assert!(d < 1e-15, "{name} err {d}");
    }

    // Zero axis: all-NaN matrix (documented poison).
    let bad = Mat3::rotation_about(Vec3::<f64>::zero(), 1.0);
    for c in [bad.c0, bad.c1, bad.c2] {
        assert!(c.x.is_nan() && c.y.is_nan() && c.z.is_nan());
    }

    // NaN angle: poison too.
    let bad2 = Mat3::rotation_about(Vec3::unit_z(), f64::NAN);
    assert!(bad2.c0.x.is_nan());
}

/// Singular inverse, det bit-identity, and inverse round-trip (the rest
/// of the review's `probe` binary).
#[test]
fn inverse_and_determinant_spot_checks() {
    // Singular-but-nonzero matrix: inverse has non-finite entries (doc
    // says "non-finite (+-inf, or NaN ...)"). Rank-2 example.
    let m = Mat3::from_cols(Vec3::unit_x(), Vec3::unit_y(), Vec3::new(1.0, 1.0, 0.0));
    let inv = m.inverse();
    let all = [
        inv.c0.x, inv.c0.y, inv.c0.z, inv.c1.x, inv.c1.y, inv.c1.z, inv.c2.x, inv.c2.y, inv.c2.z,
    ];
    assert!(all.iter().any(|e| e.is_infinite() || e.is_nan()));

    // det bit-identity claim: determinant() vs the det used inside inverse:
    // reconstruct inverse's det as c0.dot(c1.cross(c2)) and compare bits.
    let m = Mat3::from_cols(
        Vec3::new(1.1, 0.3, -0.7),
        Vec3::new(0.2, -1.9, 0.5),
        Vec3::new(-0.4, 0.8, 2.3),
    );
    let det = m.determinant();
    let det_inv_path = m.c0.dot(m.c1.cross(m.c2));
    assert_eq!(det.to_bits(), det_inv_path.to_bits());
    // Behavioral check: m * m.inverse() ~ I.
    let p = m * m.inverse();
    let idmax = [
        (p.c0.x - 1.0).abs(),
        (p.c1.y - 1.0).abs(),
        (p.c2.z - 1.0).abs(),
        p.c0.y.abs(),
        p.c0.z.abs(),
        p.c1.x.abs(),
        p.c1.z.abs(),
        p.c2.x.abs(),
        p.c2.y.abs(),
    ]
    .into_iter()
    .fold(0.0f64, f64::max);
    assert!(idmax < 1e-15, "m * m^-1 deviates from I by {idmax:e}");
}
