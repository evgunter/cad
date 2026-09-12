//! **The intersecting equal-radius cylinder pair** — the germ lane's
//! fixture (`verbs_germarms2`) and the parameter-identity channel's
//! (`seat6_germ_channel`), which reads the same pair at the same door.
//! Body authoring, so it routes here (the module's routing rule).
//!
//! What this module deliberately did NOT absorb, as the whole list:
//!
//! - `verbs_germarms2_interval.rs`'s `Interval`-typed copy of the same
//!   five functions — the interval twin builds `Body<Interval>` through
//!   `Interval::from_f64` lifts at every literal, and a scalar-generic
//!   spelling here would put the lift bounds on every f64 consumer for
//!   one twin's benefit; the copy carries the marker at the site;
//! - `topo`'s `verbs_cylsph_*` SURFACE fixtures — surfaces, not bodies,
//!   and another crate's suites.

use core::f64::consts::PI;

use geom_core::{Affine3, Point2, Point3, Tol, Vec3};
use profile::{Profile, SketchPlane};
use sweep::{Extrusion, extrude};
use topo::Body;

/// A cylinder about `z`, radius `r`, `z ∈ [−h, h]`, through the public
/// extrude door.
pub fn cyl(r: f64, h: f64) -> Body<f64> {
    let tol = Tol::witness();
    let lp = profile::circle(Point2::new(0.0, 0.0), r, tol).unwrap();
    let plane = SketchPlane::new(Affine3::translation(Vec3::new(0.0, 0.0, -h)));
    let profile = Profile::new(plane, vec![lp.into()]).validate(tol).unwrap();
    extrude(&profile, Extrusion::Distance(2.0 * h), tol)
        .unwrap()
        .body
}

pub fn spin(b: &Body<f64>, axis: Vec3<f64>, angle: f64) -> Body<f64> {
    topo::transform_rigid(
        b,
        &Affine3::rotation_about_axis(Point3::new(0.0, 0.0, 0.0), axis, angle),
        Tol::witness(),
    )
    .unwrap()
}

/// **The re-pose**: a rotation about `(1,2,3)` by 0.7 rad followed by a
/// translation off every axis plane. Nothing about the configuration
/// changes — the same two solids, the same contacts — so every row's
/// re-posed twin must answer exactly what its direct copy answers.
pub fn repose(b: &Body<f64>) -> Body<f64> {
    let r = Affine3::rotation_about_axis(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(1.0, 2.0, 3.0).normalize(),
        0.7,
    );
    topo::transform_rigid(
        b,
        &Affine3::from_parts(r.linear, r.translation + Vec3::new(0.3, -0.45, 0.6)),
        Tol::witness(),
    )
    .unwrap()
}

/// The classic Steinmetz pair: equal radii, perpendicular intersecting
/// axes, both seams on the pinch points.
pub fn steinmetz(h: f64) -> (Body<f64>, Body<f64>) {
    (
        cyl(1.0, h),
        spin(&cyl(1.0, h), Vec3::new(1.0, 0.0, 0.0), PI / 2.0),
    )
}

/// The same SURFACES with both seams turned off the pinch: each
/// operand is spun about its own axis, which a cylinder of revolution
/// is invariant under. Only the charts move.
pub fn seams_off_the_pinch(h: f64, phi: f64) -> (Body<f64>, Body<f64>) {
    let (a, b) = steinmetz(h);
    (
        spin(&a, Vec3::new(0.0, 0.0, 1.0), phi),
        spin(&b, Vec3::new(0.0, 1.0, 0.0), phi),
    )
}
