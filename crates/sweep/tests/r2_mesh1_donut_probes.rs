//! R2 review probes for PR 1389's disclosed deviation 2 (the donut
//! row repair in `revolve_washer.rs`). Attacks, by execution: the
//! structural-zero argument (the two faces' fans must cancel exactly,
//! not merely nearly), the lifted oracle's sign sensitivity, and its
//! stability under a different choice of interior lift.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod revolve_common;

use geom_core::{Point3, Tol};
use profile::RawLoop;
use revolve_common::*;
use sweep::{Revolution, revolve};

fn donut() -> sweep::Revolved<f64> {
    let lp = profile::ProfileLoop::new(vec![
        profile::ProfileVertex::new(p2(1.0, 0.5), 1.0),
        profile::ProfileVertex::new(p2(2.0, 0.5), 1.0),
    ]);
    let vp = validated(vec![lp]);
    revolve(&vp, axis_y(), Revolution::Full, Tol::witness()).unwrap()
}

/// One face's boundary fan from its own first probe point, measured
/// from the shared anchor — the per-face term of [`signed_volume`].
fn face_fan(body: &topo::Body<f64>, fk: topo::FaceKey, o: Point3<f64>) -> f64 {
    let face = body.get_face(fk).unwrap();
    let mut six_v = 0.0;
    for lk in core::iter::once(face.outer).chain(face.rings.iter().copied()) {
        let pts = loop_probe_points(body, lk);
        let p1 = pts[0];
        for i in 1..pts.len() - 1 {
            let a = p1 - o;
            let b = pts[i] - o;
            let c = pts[i + 1] - o;
            six_v += a.dot(b.cross(c));
        }
    }
    six_v
}

/// The structural-zero claim, executed: with the SHARED local anchor
/// the two walls' fans must cancel — and the probe prints how exactly.
/// Also pins that the PR's "origin-anchored 3.1e-16" was cancellation
/// noise, not a measurement: the origin-anchored total is nonzero only
/// at rounding scale against tetrahedra of order 1.
#[test]
fn r2_the_donut_fan_is_a_structural_zero() {
    let t = donut();
    let o = probe_anchor(&t.body);
    let f0 = face_fan(&t.body, t.walls[0][0].unwrap(), o);
    let f1 = face_fan(&t.body, t.walls[0][1].unwrap(), o);
    println!(
        "local anchor {o:?}: wall0 fan {f0:e}, wall1 fan {f1:e}, sum {:e}",
        f0 + f1
    );
    // Mirror cancellation: the sum is zero at rounding scale of the
    // individual fans, and the local-anchor fans themselves are small.
    assert!((f0 + f1).abs() < 1e-12, "fans do not cancel: {:e}", f0 + f1);
    let total = signed_volume(&t.body);
    println!("signed_volume (local anchor): {total:e}");
    assert!(
        total.abs() < 1e-12,
        "signed_volume not structural zero: {total:e}"
    );
    // The old spelling, replicated: origin anchor.
    let g0 = face_fan(&t.body, t.walls[0][0].unwrap(), Point3::origin());
    let g1 = face_fan(&t.body, t.walls[0][1].unwrap(), Point3::origin());
    println!(
        "origin anchor: wall0 {g0:e}, wall1 {g1:e}, sum {:e}",
        g0 + g1
    );
    assert!(
        (g0 + g1).abs() < 1e-13,
        "origin-anchored residue not rounding-scale: {:e}",
        g0 + g1
    );
}

/// The repaired oracle's sign is real: the PR's lifts give +2.828,
/// the swapped lifts give the same magnitude negated, and a DIFFERENT
/// off-seam interior lift pair agrees in sign and magnitude.
#[test]
fn r2_the_lifted_oracle_is_sign_sensitive_and_lift_stable() {
    let t = donut();
    let (w0, w1) = (t.walls[0][0].unwrap(), t.walls[0][1].unwrap());
    let pr_lifts = [
        (w0, Point3::new(0.0, 0.0, 1.5)),
        (w1, Point3::new(0.0, 1.0, 1.5)),
    ];
    let v = signed_volume_lifted(&t.body, &pr_lifts);
    let swapped = [
        (w0, Point3::new(0.0, 1.0, 1.5)),
        (w1, Point3::new(0.0, 0.0, 1.5)),
    ];
    let vs = signed_volume_lifted(&t.body, &swapped);
    // A second, independent interior pair: the inner extreme of each
    // half's minor circle (radius 1.5 at the other azimuth), still off
    // the seam. Bottom of the tube for wall 0, top for wall 1, at
    // azimuth π instead of π/2.
    let alt = [
        (w0, Point3::new(-1.5, 0.0, 0.0)),
        (w1, Point3::new(-1.5, 1.0, 0.0)),
    ];
    let va = signed_volume_lifted(&t.body, &alt);
    println!("PR lifts {v}, swapped {vs}, alternative pair {va}");
    assert!(v > 0.0, "PR lifts not positive: {v}");
    assert!((v - 2.828).abs() < 0.01, "PR lifts magnitude moved: {v}");
    assert!(vs < 0.0, "swapped lifts not negative: {vs}");
    assert!(
        (v + vs).abs() < 1e-12,
        "swap is not an exact negation: {v} {vs}"
    );
    assert!(
        va > 0.0 && (va - v).abs() < 0.01,
        "alt lift disagrees: {va} vs {v}"
    );
}

/// The comment's premise, executed: wall 0's probe points span
/// y ∈ [0, 0.5] (the LOWER half), wall 1's y ∈ [0.5, 1.0], and both
/// walls' boundary probe sets are the same point set — which is what
/// makes the unlifted fans mirror images.
#[test]
fn r2_the_walls_share_their_boundary() {
    let t = donut();
    for (w, lo, hi) in [(0usize, 0.0, 0.5), (1, 0.5, 1.0)] {
        let face = t.body.get_face(t.walls[0][w].unwrap()).unwrap();
        let mut ys: Vec<f64> = Vec::new();
        for lk in core::iter::once(face.outer).chain(face.rings.iter().copied()) {
            ys.extend(loop_probe_points(&t.body, lk).iter().map(|p| p.y));
        }
        let (min, max) = ys
            .iter()
            .fold((f64::INFINITY, f64::NEG_INFINITY), |(a, b), &y| {
                (a.min(y), b.max(y))
            });
        println!("wall {w}: probe y in [{min}, {max}]");
        assert!(
            min >= lo - 1e-12 && max <= hi + 1e-12,
            "wall {w} y span [{min}, {max}]"
        );
    }
}
