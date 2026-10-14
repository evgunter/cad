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

use geom_core::{
    Affine3, Band, Bounds, Interval, Point3, Real, RigidMotion, RigidMotionError, Tol, UnitVec3,
    Vec3,
};

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
/// asks about rigidity. Then the moved cube is read against the maps
/// at `samples`, points of the box built at point intervals by
/// `Affine3::rotation_about_axis` (the normalizing construction, not
/// the mint): every moved vertex must enclose each sample's image of
/// it, so a witness that moved the cube by the wrong motion, or by a
/// box that misses part of its own, reds here and not only one that
/// refused; and each moved coordinate must be no wider than the box
/// widens it, so a witness that widened its own box reds too.
fn moves_where_its_entries_do_not_decide(
    motion: RigidMotion<Interval>,
    samples: &[Affine3<Interval>],
    what: &str,
) {
    assert!(
        matches!(
            RigidMotion::check_rigid(&motion.to_affine(), band()),
            Err(RigidMotionError::Undecided { .. })
        ),
        "{what}: the box's entries cannot be decided orthonormal at the band"
    );
    let cube = cube();
    let moved = topo::transform_rigid(&cube, &motion, Tol::witness())
        .unwrap_or_else(|e| panic!("{what}: the witness moves the cube: {e:?}"));
    let contains =
        |outer: Interval, inner: Interval| outer.lo() <= inner.lo() && inner.hi() <= outer.hi();
    let mut read = 0;
    for ((k, p), (k_moved, q)) in cube.vertex_points().zip(moved.vertex_points()) {
        assert_eq!(k, k_moved, "{what}: the map keeps every vertex key");
        for sample in samples {
            let want = sample.transform_point(p);
            assert!(
                contains(q.x, want.x) && contains(q.y, want.y) && contains(q.z, want.z),
                "{what}: vertex {k:?} moved to {q:?}, which misses a sample's image {want:?}"
            );
            read += 1;
        }
        // The box widens a corner by its distance from the axis times the
        // box, about a hundredth of ε (the module doc; the widest row
        // measures 0.027 ε at every ε row): a witness wider than it has to
        // be passes the containment above, so its width is bounded too.
        for c in [q.x, q.y, q.z] {
            assert!(
                c.hi() - c.lo() <= 0.04 * half_width(),
                "{what}: vertex {k:?} moved to {q:?}, wider than the box widens it"
            );
        }
    }
    assert_eq!(
        read,
        8 * samples.len(),
        "{what}: every corner read at every sample"
    );
}

#[test]
fn a_widened_angle_moves_the_cube() {
    let angle = Interval::from_bounds(0.3 - half_width(), 0.3 + half_width());
    let motion = RigidMotion::rotation(UnitVec3::exact_z(), angle).expect("a finite angle");
    let samples = [0.3 - half_width(), 0.3, 0.3 + half_width()].map(|theta| {
        Affine3::rotation_about_axis(
            Point3::origin(),
            Vec3::unit_z().map(Interval::from_f64),
            Interval::from_f64(theta),
        )
    });
    moves_where_its_entries_do_not_decide(motion, &samples, "angle box");
}

#[test]
fn a_widened_axis_moves_the_cube() {
    // The axis's tilt reaches the entries through `sin` of the angle
    // and `1 − cos`, both under one at 0.3 rad, so the axis box is a
    // few ε wide to straddle the band as the angle box does.
    let w = 4.0 * half_width();
    let wobble = Interval::from_bounds(-w, w);
    let turn_about = |x: Interval, y: Interval| {
        let axis = UnitVec3::new(
            Vec3::new(x, y, Interval::from_f64(1.0)),
            "test_widened_axis",
            band(),
        )
        .expect("an axis with a length");
        RigidMotion::rotation(axis, Interval::from_f64(0.3)).expect("a finite angle")
    };
    let motion = turn_about(wobble, wobble);
    let samples = [(-w, -w), (-w, w), (0.0, 0.0), (w, -w), (w, w)].map(|(x, y)| {
        Affine3::rotation_about_axis(
            Point3::origin(),
            Vec3::new(x, y, 1.0).map(Interval::from_f64),
            Interval::from_f64(0.3),
        )
    });
    moves_where_its_entries_do_not_decide(motion, &samples, "axis box");
}
