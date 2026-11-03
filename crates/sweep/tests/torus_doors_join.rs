//! **A donut against a rod whose section the section value carries**:
//! the symmetric cylinder × torus poses whose components the value lists
//! and tells apart (a rod parallel to the torus axis, and a square rod
//! crossing the outer equator in one loop), which the door marches.
//! Each passes the join and stops at the result gate's volume, typed:
//! two chords meeting end to end on a section loop both crease there,
//! so the edge join leaves that vertex standing.

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
    /// The result gate's volume on a torus face whose trim the chart
    /// table has no lane for
    /// (`work/spiricarm/a-torus-wall-trimmed-by-a-spline-rim-has-no-volume-lane.md`).
    /// A rod's chorded wall measures through the rotational lane, so the
    /// torus face is the one the gate reports in any face order.
    ConicTrim,
    /// The edge join on a spline carrier beside another carrier, open or
    /// closed (`work/splice/a-spline-join-beside-another-carrier-has-no-far-parameter.md`),
    /// which no pose here reaches.
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
            ] if what.starts_with("a trimmed face with no volume lane on its chart") => {
                Stop::ConicTrim
            }
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
/// result gate's volume on the torus face its chords trim.
#[test]
fn a_rod_inside_the_tubes_shadow_stops_at_the_volume() {
    let got = topo::union(&donut(), &rod_z(2.0, 0.0, 0.2), Tol::witness());
    assert_eq!(stop(got), Stop::ConicTrim);
}

/// **The same rod off the axis line**: ∩ stops at the result gate's
/// volume on the torus face its chords trim.
// Pinned red at every ε row, on the torus face's conic trim:
// work/spiricarm/a-torus-wall-trimmed-by-a-spline-rim-has-no-volume-lane.md.
// When it lands, `stop` meets a body and the row is re-pinned there.
#[test]
fn a_rod_off_the_axis_line_stops_at_the_conic_trim() {
    let got = topo::intersect(&donut(), &rod_z(1.9, 0.5, 0.2), Tol::witness());
    assert_eq!(stop(got), Stop::ConicTrim);
}

/// **A rod across both bounds**: its two meridional loops carry; ∩
/// stops at the result gate's volume on the conic trim.
#[test]
fn a_rod_across_the_tube_stops_at_the_conic_trim() {
    let got = topo::intersect(&donut(), &rod_z(2.0, 0.0, 0.7), Tol::witness());
    assert_eq!(stop(got), Stop::ConicTrim);
}

/// **A rod across the inner bound**: its one null loop carries; ∩ stops
/// at the result gate's volume on the torus face its chord trims.
// Pinned red at every ε row, on the torus face's trim:
// work/spiricarm/a-torus-wall-trimmed-by-a-spline-rim-has-no-volume-lane.md.
#[test]
fn a_rod_across_the_inner_bound_stops_at_the_volume() {
    let got = topo::intersect(&donut(), &rod_z(1.5, 0.0, 0.2), Tol::witness());
    assert_eq!(stop(got), Stop::ConicTrim);
}

/// **A square rod straddling the outer equator**: its one loop carries;
/// ∖ stops at the result gate's volume on the torus face its chord trims.
// Pinned red at every ε row, on the torus face's trim:
// work/spiricarm/a-torus-wall-trimmed-by-a-spline-rim-has-no-volume-lane.md.
#[test]
fn a_square_rod_on_the_outer_equator_stops_at_the_volume() {
    let got = topo::subtract(&donut(), &rod_y(2.45, 0.1), Tol::witness());
    assert_eq!(stop(got), Stop::ConicTrim);
}
