//! **A rigid motion whose angle or axis is an interval box moves a body
//! at `Interval`.** The motion is minted by construction
//! (`RigidMotion::rotation`), so `transform_rigid` asks nothing about
//! rigidity; the same map handed over as twelve numbers is refused by
//! the decided door, undecided, because `cos` and `sin` of the box are
//! independent brackets and the entries' orthogonality straddles the
//! band.
//!
//! The box's half-width is the run's ε, so at every ε row the entries'
//! orthogonality encloses about `±2ε`, wider than the band's zero, and
//! the cube is a centimetre at the axis, so what the box widens — a
//! point's position, by its distance from the axis times the box — is
//! a hundredth of ε.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common;

use geom_core::{Band, Interval, Real, RigidMotion, RigidMotionError, Tol, UnitVec3, Vec3};

fn cube() -> topo::Body<Interval> {
    common::brick::<Interval>(
        (-0.005, 0.005),
        (-0.005, 0.005),
        (-0.005, 0.005),
        Tol::witness(),
    )
}

fn band() -> Band {
    Band::linear(Tol::witness()).unwrap()
}

/// The box's half-width: the run's ε.
fn half_width() -> f64 {
    Tol::witness().get().eps
}

/// Every entry of the map the decided door is handed is the minted
/// motion's own, so the only difference between the two roads is who
/// asks about rigidity.
fn moves_where_its_entries_do_not_decide(motion: RigidMotion<Interval>, what: &str) {
    assert!(
        matches!(
            RigidMotion::check_rigid(&motion.to_affine(), band()),
            Err(RigidMotionError::Undecided { .. })
        ),
        "{what}: the box's entries cannot be decided orthonormal at the band"
    );
    let moved = topo::transform_rigid(&cube(), &motion, Tol::witness());
    assert!(
        moved.is_ok(),
        "{what}: the witness moves the cube: {:?}",
        moved.err()
    );
}

#[test]
fn a_widened_angle_moves_the_cube() {
    let angle = Interval::from_bounds(0.3 - half_width(), 0.3 + half_width());
    let motion = RigidMotion::rotation(UnitVec3::exact_z(), angle).expect("a finite angle");
    moves_where_its_entries_do_not_decide(motion, "angle box");
}

#[test]
fn a_widened_axis_moves_the_cube() {
    // The axis's tilt reaches the entries through `sin` of the angle
    // and `1 − cos`, both under one at 0.3 rad, so the axis box is a
    // few ε wide to straddle the band as the angle box does.
    let wobble = Interval::from_bounds(-4.0 * half_width(), 4.0 * half_width());
    let axis = UnitVec3::new(
        Vec3::new(wobble, wobble, Interval::from_f64(1.0)),
        "test_widened_axis",
        band(),
    )
    .expect("an axis box with a length");
    let motion = RigidMotion::rotation(axis, Interval::from_f64(0.3)).expect("a finite angle");
    moves_where_its_entries_do_not_decide(motion, "axis box");
}
