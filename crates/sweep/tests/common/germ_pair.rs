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
use sweep::ExtrudeSide;

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
    extrude(
        &profile,
        Extrusion::Distance {
            depth: 2.0 * h,
            side: ExtrudeSide::Along,
        },
        tol,
    )
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

/// **Whether two refusals of one configuration in two poses are one
/// door** — the re-pose rows' comparison, for both scalar lanes. Their
/// `Debug` agrees, except where a refusal carries a decided margin
/// (the pierce curvature's, `CurvedSectorSideUnsupported`, and an
/// undeclared coincidence's, `UndeclaredCoincidence`): each pose
/// reads it off its own coordinates, so the twins' margins agree to
/// within the zero band the verdict was classified against rather than
/// bit for bit. A verdict of the other sign, or a margin a band-width
/// away, is another door.
pub fn same_door(a: &topo::BooleanError, b: &topo::BooleanError) -> bool {
    use geom_brep::recourse::Refused;
    use geom_core::{ErrorTextReading, MarginDiag};
    let zero = geom_core::Band::linear(Tol::witness())
        .expect("a linear band")
        .zero();
    let near = |x: MarginDiag, y: MarginDiag| match (
        x.diagnostic_f64_for_error_text(),
        y.diagnostic_f64_for_error_text(),
    ) {
        (ErrorTextReading::Value(p), ErrorTextReading::Value(q)) => (p - q).abs() <= zero,
        (
            ErrorTextReading::Enclosure { lo: l1, hi: h1 },
            ErrorTextReading::Enclosure { lo: l2, hi: h2 },
        ) => (l1 - l2).abs() <= zero && (h1 - h2).abs() <= zero,
        _ => false,
    };
    match (a, b) {
        (
            topo::BooleanError::CurvedSectorSideUnsupported { verdict: va },
            topo::BooleanError::CurvedSectorSideUnsupported { verdict: vb },
        ) => match (va, vb) {
            (Refused::Zero(x), Refused::Zero(y)) => near(x.margin, y.margin),
            (Refused::Negative { margin: x }, Refused::Negative { margin: y }) => near(*x, *y),
            _ => false,
        },
        (
            topo::BooleanError::UndeclaredCoincidence {
                diag: x,
                pair: pa,
                relation: ra,
            },
            topo::BooleanError::UndeclaredCoincidence {
                diag: y,
                pair: pb,
                relation: rb,
            },
        ) => pa == pb && ra == rb && x.predicate == y.predicate && near(x.margin, y.margin),
        _ => format!("{a:?}") == format!("{b:?}"),
    }
}
