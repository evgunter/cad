//! `ProfileDesc::float_bits` key-space pins (#101 review MINOR-1 /
//! NOTE-1): the per-loop records are length-prefixed — no in-band
//! sentinel can be forged by data (the old encoding let a garbage
//! joint index of `usize::MAX` alias the loop marker) — and declared
//! tangent joints feed the key as their canonical set.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use editor_core::ProfileDesc;
use geom_core::Point2;
use profile::{Profile, ProfileLoop, ProfileVertex, SketchPlane};

fn v(x: f64, y: f64) -> ProfileVertex<f64> {
    ProfileVertex {
        pos: Point2::new(x, y),
        bulge: 0.0,
    }
}

fn mk(pts: &[(f64, f64)]) -> ProfileLoop<f64> {
    ProfileLoop::new(pts.iter().map(|&(x, y)| v(x, y)).collect())
}

fn desc(loops: Vec<ProfileLoop<f64>>) -> ProfileDesc {
    ProfileDesc(Profile::new(SketchPlane::xy(), loops))
}

/// The review's executed collision, inverted into the pin: a garbage
/// joint index `usize::MAX` must NOT alias the loop marker — the two
/// descriptions below keyed identically under the old encoding and
/// must differ now (neither can validate, but keys/equality are
/// computed on descriptions pre-validation).
#[test]
fn garbage_joint_index_cannot_forge_a_loop_boundary() {
    let v1 = [(0.0, 0.0), (1.0, 0.0), (0.0, 1.0)];
    let v2 = [(5.0, 5.0), (6.0, 5.0), (5.0, 6.0)];
    let mut a1 = mk(&v1);
    a1.tangent_joints = vec![usize::MAX];
    let a = desc(vec![a1, mk(&v2)]);

    let mut b1 = mk(&v1);
    let mut joints = vec![usize::MAX, usize::MAX];
    for &(x, y) in &v2 {
        joints.extend([
            x.to_bits() as usize,
            y.to_bits() as usize,
            0.0f64.to_bits() as usize,
        ]);
    }
    b1.tangent_joints = joints;
    let b = desc(vec![b1]);

    assert_ne!(a.float_bits(), b.float_bits(), "sentinel forged by data");
    assert_ne!(a, b);
}

/// Declarations are a SET: duplicates and order do not change the key
/// (review NOTE-1 — spurious recompute / bit-inequality noise).
#[test]
fn tangent_joint_set_semantics_key_identically() {
    let pts = [(0.0, 0.0), (2.0, 0.0), (2.0, 1.0), (0.0, 1.0)];
    let mut a = mk(&pts);
    a.tangent_joints = vec![1, 1, 3];
    let mut b = mk(&pts);
    b.tangent_joints = vec![3, 1];
    assert_eq!(desc(vec![a]).float_bits(), desc(vec![b]).float_bits());
    // ...and a different set is a different key.
    let mut c = mk(&pts);
    c.tangent_joints = vec![1];
    let mut d = mk(&pts);
    d.tangent_joints = vec![1, 3];
    assert_ne!(desc(vec![c]).float_bits(), desc(vec![d]).float_bits());
}

/// The original marker guarantee survives the re-encoding: loop
/// partitioning is key-distinct (2 loops of 2 vertices vs 1 loop of 4
/// with the same float stream), and declarations distinguish from
/// their absence.
#[test]
fn loop_partitioning_and_declarations_stay_key_distinct() {
    let two = desc(vec![
        mk(&[(0.0, 0.0), (1.0, 0.0)]),
        mk(&[(2.0, 0.0), (3.0, 0.0)]),
    ]);
    let one = desc(vec![mk(&[(0.0, 0.0), (1.0, 0.0), (2.0, 0.0), (3.0, 0.0)])]);
    assert_ne!(two.float_bits(), one.float_bits());

    let plain = mk(&[(0.0, 0.0), (1.0, 0.0), (0.0, 1.0)]);
    let mut declared = plain.clone();
    declared.tangent_joints = vec![0];
    assert_ne!(
        desc(vec![plain]).float_bits(),
        desc(vec![declared]).float_bits()
    );
}
