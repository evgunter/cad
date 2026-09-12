//! **The raw scalar-lift door, rung by rung**: `ProfileVertex::map`,
//! `ProfileLoop::map_scalar`, `Profile::map_scalar`.
//!
//! The claim under test is that the lift is STRUCTURAL — every stored
//! scalar goes through `f` and nothing is computed, every count and
//! index carried — so it is exact wherever `f` is, and at `f64` it is
//! the identity down to the sign of a zero.
//!
//! **The expected values are written here, not derived.** Each row
//! names the source constants the fixture was authored from and
//! compares the lifted form against `from_f64` of those same
//! constants, so no row can pass by agreeing with the door about what
//! the door does.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Affine3, Dual64, Point2, Real, Vec3};
use profile::{Profile, ProfileLoop, ProfileVertex, RawLoop, SketchPlane};

/// The fixture's vertices as `(x, y, bulge)`, authored here and read
/// by every row as the source of truth: a negative coordinate, a
/// signed zero, bulges with no exact binary representation, and two
/// straight legs.
const VERTS: [(f64, f64, f64); 4] = [
    (-0.5, -0.0, 0.1),
    (2.25, 0.0, -0.3),
    (2.25, 1.75, 0.0),
    (-0.5, 1.75, 0.0),
];

/// The declared-tangent joints of the fixture loop — indices, which a
/// scalar lift must carry rather than map.
const JOINTS: [usize; 2] = [1, 2];

/// The fixture plane's translation, authored here.
const ORIGIN: (f64, f64, f64) = (-1.5, 0.25, 3.0);

fn source_loop() -> ProfileLoop<f64> {
    ProfileLoop::new(
        VERTS
            .iter()
            .map(|&(x, y, b)| ProfileVertex::new(Point2::new(x, y), b))
            .collect(),
    )
    .with_tangent_joints(JOINTS.to_vec())
}

fn source_profile() -> Profile<f64> {
    Profile::new(
        SketchPlane::new(Affine3::translation(Vec3::new(
            ORIGIN.0, ORIGIN.1, ORIGIN.2,
        ))),
        vec![source_loop(), source_loop()],
    )
}

/// A `Dual64` that is `from_f64(want)`: the value channel's bits are
/// `want`'s — bit equality, so a laundered `-0.0` is caught — and the
/// derivative channel is zero, a lift seeding no tangent.
fn is_lift_of(got: Dual64, want: f64, what: &str) {
    assert!(
        got.value.to_bits() == want.to_bits(),
        "{what}: value channel is {}, want {want}",
        got.value
    );
    assert!(
        got.deriv == 0.0,
        "{what}: a lifted scalar carries no tangent, got {}",
        got.deriv
    );
}

#[test]
fn the_vertex_rung_carries_each_scalar_through_f() {
    for &(x, y, b) in &VERTS {
        let lifted: ProfileVertex<Dual64> =
            ProfileVertex::new(Point2::new(x, y), b).map(Dual64::from_f64);
        is_lift_of(lifted.pos().x, x, "x");
        is_lift_of(lifted.pos().y, y, "y");
        is_lift_of(lifted.bulge(), b, "bulge");
    }
}

#[test]
fn the_loop_rung_carries_the_vertex_order_and_the_joint_set() {
    let lifted: ProfileLoop<Dual64> = source_loop().map_scalar(Dual64::from_f64);
    assert_eq!(lifted.vertices().len(), VERTS.len());
    for (v, &(x, y, b)) in lifted.vertices().iter().zip(VERTS.iter()) {
        is_lift_of(v.pos().x, x, "x");
        is_lift_of(v.pos().y, y, "y");
        is_lift_of(v.bulge(), b, "bulge");
    }
    assert_eq!(
        lifted.tangent_joints(),
        JOINTS.as_slice(),
        "the declared joints are indices and travel unchanged"
    );
}

#[test]
fn the_profile_rung_carries_the_plane_and_the_loop_order() {
    let lifted: Profile<Dual64> = source_profile().map_scalar(Dual64::from_f64);
    assert_eq!(lifted.loops.len(), 2, "the loop list is carried in order");
    let m = &lifted.plane.placement;
    is_lift_of(m.translation.x, ORIGIN.0, "origin x");
    is_lift_of(m.translation.y, ORIGIN.1, "origin y");
    is_lift_of(m.translation.z, ORIGIN.2, "origin z");
    is_lift_of(m.linear.c0.x, 1.0, "u x");
    is_lift_of(m.linear.c1.y, 1.0, "v y");
    is_lift_of(m.linear.c2.z, 1.0, "n z");
    for lp in &lifted.loops {
        assert_eq!(lp.vertices().len(), VERTS.len());
        assert_eq!(lp.tangent_joints(), JOINTS.as_slice());
        for (v, &(x, y, b)) in lp.vertices().iter().zip(VERTS.iter()) {
            is_lift_of(v.pos().x, x, "x");
            is_lift_of(v.pos().y, y, "y");
            is_lift_of(v.bulge(), b, "bulge");
        }
    }
}

#[test]
fn the_lift_to_f64_is_the_identity_down_to_the_sign_of_a_zero() {
    let lifted: Profile<f64> = source_profile().map_scalar(f64::from_f64);
    for lp in &lifted.loops {
        for (v, &(x, y, b)) in lp.vertices().iter().zip(VERTS.iter()) {
            assert!(v.pos().x.to_bits() == x.to_bits(), "x");
            assert!(
                v.pos().y.to_bits() == y.to_bits(),
                "the signed zero survives: got {}, want {y}",
                v.pos().y
            );
            assert!(v.bulge().to_bits() == b.to_bits(), "bulge");
        }
    }
}

/// At `Interval` every lifted scalar is a POINT enclosure of its
/// source: the lift performs no arithmetic, so there is nothing to
/// round outward.
#[cfg(feature = "interval")]
#[test]
fn the_lift_to_interval_is_point_wide() {
    use geom_core::{Bounds, Interval};
    let lifted: Profile<Interval> = source_profile().map_scalar(Interval::from_f64);
    for lp in &lifted.loops {
        for (v, &(x, y, b)) in lp.vertices().iter().zip(VERTS.iter()) {
            for (got, want) in [(v.pos().x, x), (v.pos().y, y), (v.bulge(), b)] {
                assert!(
                    got.lo().to_bits() == want.to_bits() && got.hi().to_bits() == want.to_bits(),
                    "[{}, {}] is not the point enclosure of {want}",
                    got.lo(),
                    got.hi()
                );
            }
        }
    }
}
