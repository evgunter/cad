//! **The oblique clip** — a base body intersected with a 3-unit cube
//! turned about an axis through `(1, 1, 1)`, whose corners are
//! genuinely oblique trihedra (no support's normal is parallel to the
//! other two's edge). [`f4`] is the one `m5_pr12_fix_pass`'s `f4` row
//! and `corner_patch_chart_poles` both fillet. Body authoring, so it
//! routes here (the module's routing rule).
//!
//! What this module deliberately did NOT absorb, as the whole list:
//!
//! - `general_circle_octant_dual.rs`'s `Dual64` copy of [`f4`] — the
//!   dual lane lifts every literal through `Dual::constant` and
//!   extrudes its own profiles, and a scalar-generic spelling here
//!   would put the lift on every `f64` consumer for one twin's
//!   benefit; the copy carries the marker at the site.

use geom_core::{Affine3, Point3, Tol, Vec3};
use sweep::test_support::{brick, finished};
use topo::Body;
use topo::boolean::{BooleanOp, SweepStrategy, boolean_op_with};

/// `base ∩` the cube `[0, 3]³` moved by `(−1.55, −1.55, −2.45)` and
/// turned `angle` about `axis` through `(1, 1, 1)`; `None` where the
/// boolean refuses or leaves no body.
pub fn clip(base: Body<f64>, angle: f64, axis: Vec3<f64>) -> Option<Body<f64>> {
    let tilted = topo::transform_rigid(
        &brick((0.0, 3.0), (0.0, 3.0), (0.0, 3.0), Tol::witness()),
        &geom_core::RigidMotion::translation(Vec3::new(-1.55, -1.55, -2.45)).unwrap(),
        Tol::witness(),
    )
    .unwrap();
    let tilted = topo::transform_rigid(
        &tilted,
        &geom_core::test_support::rigid(
            &Affine3::rotation_about_axis(Point3::new(1.0, 1.0, 1.0), axis.normalize(), angle),
            Tol::witness(),
        ),
        Tol::witness(),
    )
    .unwrap();
    let out = boolean_op_with(
        BooleanOp::Intersect,
        &finished("the base", base, Tol::witness()),
        &finished("the tilted cube", tilted, Tol::witness()),
        &topo::BooleanInputs::default(),
        SweepStrategy::Realized,
        Tol::witness(),
    )
    .ok()?;
    Some(out.body()?.body.clone().into_body())
}

/// The unit cube clipped by the cube turned `0.4` about `(1, −1, 0)`.
pub fn f4() -> Body<f64> {
    clip(
        brick((0.0, 1.0), (0.0, 1.0), (0.0, 1.0), Tol::witness()),
        0.4,
        Vec3::new(1.0, -1.0, 0.0),
    )
    .expect("the f4 oblique clip")
}
