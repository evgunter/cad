//! **The rigid poses a re-posed row asks its question at**: the torax
//! rows' one re-pose ([`torax_pose`]), and the six-pose set ([`poses`])
//! a `point_in_solid` row puts every fixture through, because the pose
//! decides which schedule ray is the first to answer and a misread face
//! shows at some poses and hides at others. What a suite drives a door
//! WITH, so it routes here beside [`super::charts`] ([`super`]'s
//! routing rule).
//!
//! **Deliberately not absorbed**, and the whole of it: [`super::charts`],
//! which a pose would join as something a suite drives a door with and
//! does not — a pose moves the whole body rigidly, a chart move offsets
//! faces of it.

use geom_core::{Affine3, Point3, Vec3};

/// **The torax rows' re-pose**: `0.7` rad about `x` through
/// `(1/4, −1/2, 1/8)` — a turn that moves the revolve axis, so a door
/// that read a global axis would answer differently after it.
pub fn torax_pose() -> Affine3<f64> {
    Affine3::rotation_about_axis(
        Point3::new(0.25, -0.5, 0.125),
        Vec3::new(1.0, 0.0, 0.0),
        0.7,
    )
}

/// The identity, the torax row's own re-pose, and four more — two of
/// which (a quarter turn about `x`, a turn about `y` that keeps the
/// revolve axis) leave the schedule meeting the fixtures as it meets
/// them unposed.
pub fn poses() -> Vec<(&'static str, Affine3<f64>)> {
    let about = |pivot: [f64; 3], axis: Vec3<f64>, angle: f64| {
        Affine3::rotation_about_axis(Point3::new(pivot[0], pivot[1], pivot[2]), axis, angle)
    };
    vec![
        ("identity", about([0.0; 3], Vec3::new(1.0, 0.0, 0.0), 0.0)),
        ("0.7 about x through (1/4, -1/2, 1/8)", torax_pose()),
        (
            "0.7 about z",
            about([0.0; 3], Vec3::new(0.0, 0.0, 1.0), 0.7),
        ),
        (
            "0.3 about y through (0.1, 0.2, 0.3)",
            about([0.1, 0.2, 0.3], Vec3::new(0.0, 1.0, 0.0), 0.3),
        ),
        (
            "1.1 about (1,2,3) through (-0.2, 0.1, 0.4)",
            about([-0.2, 0.1, 0.4], Vec3::new(1.0, 2.0, 3.0).normalize(), 1.1),
        ),
        (
            "pi/2 about x",
            about(
                [0.0; 3],
                Vec3::new(1.0, 0.0, 0.0),
                core::f64::consts::FRAC_PI_2,
            ),
        ),
    ]
}
