//! **A donut against a rod whose section the section value carries**:
//! the symmetric cylinder × torus poses whose components the value lists
//! and tells apart (a rod parallel to the torus axis, and a rod square to the axis
//! crossing the outer equator in one loop), which the door marches.
//! Each passes the join and builds, its torus face measured through the
//! rotational lane, at the volume the donut sliced against the rod
//! reads.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common::donut_slices::{Op, Rod, volume};
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

/// What the slicing oracle's own quadrature may be off by (m³), far
/// below any width the enclosures read.
const ORACLE_SLACK: f64 = 1e-11;

/// **A run builds at its volume**: the result a body, which passed the
/// result gate (tier 3, its torus face measured through the rotational
/// lane), whose certified volume holds the donut sliced against the rod
/// by `op` ([`crate::common::donut_slices`]).
fn at_its_volume(
    label: &str,
    got: Result<topo::BooleanResult<f64>, topo::BooleanError>,
    op: Op,
    rod: Rod,
) {
    let tol = Tol::witness();
    let got = got.unwrap_or_else(|e| panic!("{label}: refused: {e:?}"));
    let body = got
        .body()
        .unwrap_or_else(|| panic!("{label}: an empty result"));
    let m = topo::mass_properties(&body.body, tol, topo::test_support::pinned_resolution(tol))
        .unwrap_or_else(|e| panic!("{label}: the body does not measure: {e:?}"))
        .enclosure;
    let truth = volume(op, rod);
    assert!(
        (m.volume - truth).abs() <= m.volume_pad + ORACLE_SLACK,
        "{label}: the volume {} ± {} misses the sliced {truth}",
        m.volume,
        m.volume_pad
    );
}

/// **A rod round the core, inside the tube's shadow**: the two loops
/// round the rod, told apart by the torus's plane, carry; ∪ builds at its
/// volume.
#[test]
fn a_rod_inside_the_tubes_shadow_builds_at_its_volume() {
    let got = topo::union(&donut(), &rod_z(2.0, 0.0, 0.2), Tol::witness());
    let rod = Rod::AlongZ {
        at: (2.0, 0.0),
        rc: 0.2,
    };
    at_its_volume("the rod in the shadow", got, Op::Union, rod);
}

/// **The same rod off the axis line**: ∩ builds at its volume, the
/// torus face its chords trim measured.
#[test]
fn a_rod_off_the_axis_line_builds_at_its_volume() {
    let got = topo::intersect(&donut(), &rod_z(1.9, 0.5, 0.2), Tol::witness());
    let rod = Rod::AlongZ {
        at: (1.9, 0.5),
        rc: 0.2,
    };
    at_its_volume("the rod off the axis line", got, Op::Intersect, rod);
}

/// **A rod across both bounds**: its two meridional loops carry; ∩ builds
/// at its volume.
#[test]
fn a_rod_across_the_tube_builds_at_its_volume() {
    let got = topo::intersect(&donut(), &rod_z(2.0, 0.0, 0.7), Tol::witness());
    let rod = Rod::AlongZ {
        at: (2.0, 0.0),
        rc: 0.7,
    };
    at_its_volume("the rod across the tube", got, Op::Intersect, rod);
}

/// **A rod across the inner bound**: its one null loop carries; ∩ builds
/// at its volume.
#[test]
fn a_rod_across_the_inner_bound_builds_at_its_volume() {
    let got = topo::intersect(&donut(), &rod_z(1.5, 0.0, 0.2), Tol::witness());
    let rod = Rod::AlongZ {
        at: (1.5, 0.0),
        rc: 0.2,
    };
    at_its_volume("the rod across the inner bound", got, Op::Intersect, rod);
}

/// **A rod along `y` across the outer equator**: its one loop carries;
/// ∖ builds at its volume. The rod along `y` through `(2.45, 0, 0)` is,
/// turned a quarter about the donut's axis, the oracle's rod along `x`
/// through `y = 2.45`.
#[test]
fn a_rod_across_the_outer_equator_builds_at_its_volume() {
    let got = topo::subtract(&donut(), &rod_y(2.45, 0.1), Tol::witness());
    let rod = Rod::AlongX {
        x: (-1.0, 1.0),
        at: (2.45, 0.0),
        rc: 0.1,
    };
    at_its_volume("the rod across the outer equator", got, Op::Subtract, rod);
}
