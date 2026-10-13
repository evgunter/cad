//! **The cone and cylinder iso parses decide a circle's incidence at
//! its true distance from the surface, and whether it is the iso rim as
//! the inventory's question.** A circle within band of the surface but
//! not the iso rim — tilted to second order on a cylinder or a needle
//! cone, its centre off the axis of a near-flat cone — lies on the
//! surface and is refused `NotIsoRectangle`; the same circle moved off
//! the surface is refused `OffSurface`. Each in-band row is sized from
//! the run's ε: the true distance at most `ε/10`, the deviation from the
//! iso rim at least `2·K·ε`.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::shared::tol::{band, eps};
use geom::{Curve3, Surface};
use geom_brep::props::{LoopEdge, PropsError, require_iso_rectangle};
use geom_core::{Point3, Vec3};

const TAU: f64 = core::f64::consts::TAU;
const HALF_PI: f64 = core::f64::consts::FRAC_PI_2;

/// A full rim about `+z`-ish: centre `(off, 0, z)`, radius `r`, its
/// axis tipped `tilt` toward `+x`.
fn rim(z: f64, r: f64, off: f64, tilt: f64, fwd: bool, tag: u32) -> LoopEdge<f64> {
    let (s, c) = tilt.sin_cos();
    LoopEdge::hand_built(
        Curve3::Circle {
            center: Point3::new(off, 0.0, z),
            axis: Vec3::new(s, 0.0, c),
            radius: r,
            u_ref: Vec3::new(c, 0.0, -s),
        },
        0.0,
        TAU,
        fwd,
        tag,
        tag,
    )
}

fn cylinder() -> Surface<f64> {
    Surface::Cylinder {
        origin: Point3::origin(),
        axis: Vec3::unit_z(),
        radius: 1.0,
        u_ref: Vec3::unit_x(),
    }
}

fn cone(alpha: f64) -> Surface<f64> {
    Surface::Cone {
        apex: Point3::origin(),
        axis: Vec3::unit_z(),
        half_angle: alpha,
        u_ref: Vec3::unit_x(),
    }
}

/// A band of `cone(alpha)` between the rims of radius 1 and 2, the
/// upper one tipped `tilt` and moved `off` off the axis.
fn cone_band(alpha: f64, off: f64, tilt: f64) -> Vec<LoopEdge<f64>> {
    let h = |r: f64| r / alpha.tan();
    vec![
        rim(h(1.0), 1.0, 0.0, 0.0, true, 0),
        rim(h(2.0), 2.0, off, tilt, false, 1),
    ]
}

/// A unit cylinder's band `z ∈ [0, 1]`, the upper rim tipped `tilt`.
fn cylinder_band(tilt: f64) -> Vec<LoopEdge<f64>> {
    vec![
        rim(0.0, 1.0, 0.0, 0.0, true, 0),
        rim(1.0, 1.0, 0.0, tilt, false, 1),
    ]
}

fn not_iso(what: &'static str) -> Result<(), PropsError> {
    Err(PropsError::NotIsoRectangle { what })
}

fn off_surface(what: &'static str) -> Result<(), PropsError> {
    Err(PropsError::OffSurface { what })
}

/// The untouched bands parse: the rows below perturb a face the parse
/// accepts.
#[test]
fn the_unperturbed_bands_parse() {
    assert_eq!(
        require_iso_rectangle(&cylinder(), &cylinder_band(0.0), band()),
        Ok(())
    );
    for alpha in [1e-3, HALF_PI - 1e-3] {
        assert_eq!(
            require_iso_rectangle(&cone(alpha), &cone_band(alpha, 0.0, 0.0), band()),
            Ok(()),
            "alpha = {alpha:e}"
        );
    }
}

/// A unit cylinder's rim tipped `θ = √(ε/20)`: off the cylinder by
/// `1 − cos θ ≈ ε/40`, off the iso rim by `sin θ`, past `2·K·ε` at
/// every committed ε. Tipped `0.05` it is off the cylinder.
#[test]
fn a_cylinder_rim_tipped_to_second_order_is_on_it_and_not_iso() {
    let theta = (eps() / 20.0).sqrt();
    assert_eq!(
        require_iso_rectangle(&cylinder(), &cylinder_band(theta), band()),
        not_iso("props_rim_axis_parallel"),
        "tipped by {theta:e}"
    );
    assert_eq!(
        require_iso_rectangle(&cylinder(), &cylinder_band(0.05), band()),
        off_surface("props_cylinder_circle_tilt"),
    );
}

/// Toward the needle (`α = 10⁻³`), a radius-2 rim tipped `θ = 20·ε`: off
/// the cone by `2·(θ·sin α + (1 − cos θ)·cos α) ≈ 0.04·ε`, off the iso
/// rim by `2·sin θ = 40·ε`. Tipped `0.05` it is off the cone.
#[test]
fn a_needle_cone_rim_tipped_within_band_is_on_it_and_not_iso() {
    let alpha = 1e-3;
    let theta = 20.0 * eps();
    assert_eq!(
        require_iso_rectangle(&cone(alpha), &cone_band(alpha, 0.0, theta), band()),
        not_iso("props_rim_axis_parallel"),
    );
    assert_eq!(
        require_iso_rectangle(&cone(alpha), &cone_band(alpha, 0.0, 0.05), band()),
        off_surface("props_cone_circle_tilt"),
    );
}

/// Toward the flat (`α = π/2 − 10⁻³`), a rim moved `o = 40·ε` off the
/// axis: off the cone by `o·cos α ≈ 0.04·ε`, off the iso rim by `o`.
/// Moved `0.05` it is off the cone.
#[test]
fn a_near_flat_cone_rim_off_axis_within_band_is_on_it_and_not_iso() {
    let alpha = HALF_PI - 1e-3;
    let o = 40.0 * eps();
    assert_eq!(
        require_iso_rectangle(&cone(alpha), &cone_band(alpha, o, 0.0), band()),
        not_iso("props_rim_center_on_axis"),
    );
    assert_eq!(
        require_iso_rectangle(&cone(alpha), &cone_band(alpha, 0.05, 0.0), band()),
        off_surface("props_cone_circle_off_axis"),
    );
}
