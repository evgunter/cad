//! **A donut against a rod whose section the section value carries**:
//! the symmetric cylinder × torus poses whose components the value lists
//! and tells apart (a rod parallel to the torus axis, and a square rod
//! crossing the outer equator in one loop), which the door marches.
//! Each reaches the join and stops there, typed.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common::germ_pair::cyl;
use crate::revolve_common::{axis_y, validated};
use core::f64::consts::FRAC_PI_2;
use geom_core::{Affine3, Point3, Tol, Vec3};
use sweep::test_support::finished;
use sweep::{Revolution, revolve};
use topo::{AtRestBody, Body};

/// The donut `R = 2`, `r = 1/2`, turned onto `z`.
fn donut() -> AtRestBody<f64> {
    let vp = validated(vec![crate::revolve_common::donut_profile()]);
    let d = revolve(&vp, axis_y(), Revolution::Full, Tol::witness())
        .expect("the donut revolves")
        .body;
    finished(
        "the donut",
        placed(&d, Vec3::unit_x(), FRAC_PI_2, Vec3::new(0.0, 0.0, 0.0)),
        Tol::witness(),
    )
}

fn placed(b: &Body<f64>, axis: Vec3<f64>, angle: f64, t: Vec3<f64>) -> Body<f64> {
    let r = Affine3::rotation_about_axis(Point3::origin(), axis, angle);
    topo::transform_rigid(
        b,
        &geom_core::test_support::rigid(
            &Affine3::from_parts(r.linear, r.translation + t),
            Tol::witness(),
        ),
        Tol::witness(),
    )
    .unwrap()
}

/// A rod of radius `rc`, `|z| ≤ 1` along `z` through `(x, y)`.
fn rod_z(x: f64, y: f64, rc: f64) -> AtRestBody<f64> {
    finished(
        "the rod",
        placed(&cyl(rc, 1.0), Vec3::unit_z(), 0.0, Vec3::new(x, y, 0.0)),
        Tol::witness(),
    )
}

/// A rod of radius `rc`, `|y| ≤ 1` along `y` through `(x, 0, 0)`.
fn rod_y(x: f64, rc: f64) -> AtRestBody<f64> {
    finished(
        "the rod",
        placed(
            &cyl(rc, 1.0),
            Vec3::unit_x(),
            FRAC_PI_2,
            Vec3::new(x, 0.0, 0.0),
        ),
        Tol::witness(),
    )
}

/// **Where a run stops past the join**, by variant.
#[derive(Debug, PartialEq, Eq)]
enum Stop {
    /// The result gate's volume on a face whose fitted boundary has no
    /// harmonic form on the analytic chart
    /// (`work/spiricarm/a-projected-boundary-on-an-analytic-chart-has-no-volume-lane.md`).
    Projected,
    /// The result gate's volume on a torus face trimmed by a conic.
    ConicTrim,
    /// The edge join on a spline carrier, open or closed.
    Join { closed: bool },
}

fn stop(got: Result<topo::BooleanResult<f64>, topo::BooleanError>) -> Stop {
    use geom_brep::PropsError;
    use topo::{BooleanError, MassPropsError, ValidationError};
    match got {
        Err(BooleanError::ResultInvalid { errors }) => match errors.as_slice() {
            [
                ValidationError::VolumeUncomputable {
                    source:
                        MassPropsError::Face {
                            source: PropsError::QuadratureUnsupported { what },
                            ..
                        },
                    ..
                },
            ] if what.starts_with(
                "curved-cut face half-edge carries a pcurve with no harmonic form",
            ) =>
            {
                Stop::Projected
            }
            [
                ValidationError::VolumeUncomputable {
                    source:
                        MassPropsError::Face {
                            source: PropsError::QuadratureUnsupported { what },
                            ..
                        },
                    ..
                },
            ] if what.starts_with("conic trim on a cone/sphere/torus chart") => Stop::ConicTrim,
            other => panic!("an unpinned result gate: {other:?}"),
        },
        Err(BooleanError::JoinCarrierUnsupported {
            carrier: geom::CurveKind::Nurbs,
            closed,
            ..
        }) => Stop::Join { closed },
        other => panic!(
            "an unpinned stop: {:?}",
            other.map(|r| r.body().map(|b| b.kind))
        ),
    }
}

/// **A rod round the core, inside the tube's shadow**: the two loops
/// round the rod, told apart by the torus's plane, carry; ∪ stops at the
/// result gate's volume.
#[test]
fn a_rod_inside_the_tubes_shadow_stops_at_the_volume() {
    let got = topo::union(&donut(), &rod_z(2.0, 0.0, 0.2), Tol::witness());
    assert_eq!(stop(got), Stop::Projected);
}

/// **The same rod off the axis line**: ∩ stops at the edge join.
#[test]
fn a_rod_off_the_axis_line_stops_at_the_join() {
    let got = topo::intersect(&donut(), &rod_z(1.9, 0.5, 0.2), Tol::witness());
    assert_eq!(stop(got), Stop::Join { closed: false });
}

/// **A rod across both bounds**: its two meridional loops carry; ∩
/// stops at the result gate's volume on the conic trim.
#[test]
fn a_rod_across_the_tube_stops_at_the_conic_trim() {
    let got = topo::intersect(&donut(), &rod_z(2.0, 0.0, 0.7), Tol::witness());
    assert_eq!(stop(got), Stop::ConicTrim);
}

/// **A rod across the inner bound**: its one null loop carries; ∩ stops
/// at the edge join on the closed loop.
#[test]
fn a_rod_across_the_inner_bound_stops_at_the_join() {
    let got = topo::intersect(&donut(), &rod_z(1.5, 0.0, 0.2), Tol::witness());
    assert_eq!(stop(got), Stop::Join { closed: true });
}

/// **A square rod straddling the outer equator**: its one loop carries;
/// ∖ stops at the edge join.
#[test]
fn a_square_rod_on_the_outer_equator_stops_at_the_join() {
    // Pinned red at 1e-6 since PR 6011: the germ section refuses its
    // winding before the join
    // (`work/sect/main-red-five-torus-section-rows-refuse-at-1e-6-since-components-by-foot.md`).
    test_utils::pinned_red::row(
        Tol::witness().eps(),
        &[1e-6],
        "the square rod on the outer equator",
        &[
            "an unpinned stop: Err(GermSectionUnbuilt { a_face: FaceKey(4v1), a_kind: Torus, b_face: FaceKey(3v1), b_kind: Cylinder, why: Winds(Unseparated) })",
        ],
        a_square_rod_on_the_outer_equator_stops_at_the_join_row,
    );
}

/// The row [`a_square_rod_on_the_outer_equator_stops_at_the_join`] runs, under its pin.
fn a_square_rod_on_the_outer_equator_stops_at_the_join_row() {
    let got = topo::subtract(&donut(), &rod_y(2.45, 0.1), Tol::witness());
    assert_eq!(stop(got), Stop::Join { closed: false });
}
