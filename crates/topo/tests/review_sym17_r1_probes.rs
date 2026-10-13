//! SYM-17 review R1's probes of the rigid-motion witness, at the body
//! door. Each row asserts what the frozen head DOES; the review reads
//! the result against what the type's docs say it means.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common;

use geom_core::{
    Affine3, Band, Interval, Mat3, Point3, Real, RigidMotion, RigidMotionError, Tol, Vec3,
};

fn band() -> Band {
    Band::linear(Tol::witness()).unwrap()
}

/// A stretch along x whose column-norm margin `s² − 1 ≈ 8e-10` (and
/// determinant margin 4e-10) the door decides `Zero` at the default ε:
/// admitted.
fn near_band_scale() -> Affine3<f64> {
    let s = 1.0 + 4e-10;
    Affine3::from_parts(
        Mat3::from_cols(
            Vec3::new(s, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
            Vec3::new(0.0, 0.0, 1.0),
        ),
        Vec3::zero(),
    )
}

/// **Products of admitted motions are not decided again**, so a
/// door-admitted motion's defect compounds under `compose` without
/// bound: twenty squarings of an admitted 4e-10 scale are a 4.2e-4
/// scale, which the door refuses and the body door moves a cube by.
#[test]
fn r1_an_admitted_defect_compounds_under_compose() {
    let mut m = RigidMotion::check_rigid(&near_band_scale(), band()).expect("admitted");
    for _ in 0..20 {
        m = m.compose(m).unwrap();
    }
    let scale = m.to_affine().linear.c0.x;
    eprintln!("R1 compounded scale: {scale:.6e}");
    assert!(scale > 1.0 + 4e-4, "the product scales by {scale}");
    let door = RigidMotion::check_rigid(&m.to_affine(), band());
    eprintln!("R1 door on the product: {door:?}");
    assert!(matches!(door, Err(RigidMotionError::NotRigid { .. })));
    let cube = common::brick::<f64>((0.0, 1.0), (0.0, 1.0), (0.0, 1.0), Tol::witness());
    let moved = topo::transform_rigid(&cube, &m, Tol::witness());
    eprintln!(
        "R1 compounded scale through transform_rigid: {:?}",
        moved.as_ref().err()
    );
    let moved = moved.expect("the body door asks nothing about rigidity");
    let far = moved
        .vertex_points()
        .map(|(_, p)| (p - Point3::origin()).norm())
        .fold(0.0_f64, f64::max);
    let reach = moved
        .vertex_points()
        .map(|(_, p)| p.x)
        .fold(0.0_f64, f64::max);
    eprintln!("R1 the unit cube's far corner now sits at {far:.12}; its x extent is {reach:.12}");
    assert!(reach > 1.0 + 4e-4, "the cube is stretched along x");
}

/// **The transpose is not the inverse of an admitted motion**: `m ∘
/// m⁻¹` of the admitted stretch is `s²` along x, whose column margin
/// `s⁴ − 1 ≈ 1.6e-9` the same door does not admit.
#[test]
fn r1_m_then_its_inverse_is_refused_by_the_door_that_admitted_m() {
    let m = RigidMotion::check_rigid(&near_band_scale(), band()).expect("admitted");
    let back = m.compose(m.inverse().unwrap()).unwrap();
    let r = RigidMotion::check_rigid(&back.to_affine(), band());
    eprintln!("R1 door on m∘m⁻¹: {r:?}");
    assert!(r.is_err());
}

/// **What the mints' finiteness decides at `Interval`.** Printed, not
/// pinned: the review reads it.
#[test]
fn r1_interval_translation_finiteness() {
    let inf = Interval::from_f64(f64::INFINITY);
    let zero = Interval::from_f64(0.0);
    let minted = RigidMotion::translation(Vec3::new(inf, zero, zero));
    eprintln!("R1 Interval translation(+inf): {:?}", minted.as_ref().err());
    let big = Interval::from_f64(f64::MAX);
    let far = RigidMotion::translation(Vec3::new(big, zero, zero)).unwrap();
    let product = far.compose(far);
    eprintln!("R1 Interval MAX∘MAX: {:?}", product.as_ref().err());
    if let Ok(p) = product {
        eprintln!("R1 Interval MAX∘MAX origin x: {:?}", p.origin().x);
        let cube = common::brick::<Interval>(
            (-0.005, 0.005),
            (-0.005, 0.005),
            (-0.005, 0.005),
            Tol::witness(),
        );
        let moved = topo::transform_rigid(&cube, &p, Tol::witness());
        eprintln!(
            "R1 transform_rigid by MAX∘MAX at Interval: {:?}",
            moved.err()
        );
    }
    if let Ok(m) = minted {
        let cube = common::brick::<Interval>(
            (-0.005, 0.005),
            (-0.005, 0.005),
            (-0.005, 0.005),
            Tol::witness(),
        );
        let moved = topo::transform_rigid(&cube, &m, Tol::witness());
        eprintln!("R1 transform_rigid by +inf at Interval: {:?}", moved.err());
    }
}
