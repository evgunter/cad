//! Shared acceptance-body builders for the STL export suites (via the
//! public profile/sweep APIs only — the same shapes as the mesh
//! acceptance suites and the CI export example).
#![allow(dead_code)] // each consumer uses a subset

use geom_core::{Point2, Tolerance, Vec2};
use profile::{Profile, ProfileLoop, ProfileVertex, SketchPlane, ValidatedProfile};
use sweep::{Extrusion, Revolution, RevolveAxis, extrude, revolve};
use topo::Body;

pub fn p2(x: f64, y: f64) -> Point2<f64> {
    Point2::new(x, y)
}

pub fn validated(loops: Vec<ProfileLoop<f64>>) -> ValidatedProfile<f64> {
    Profile::new(SketchPlane::xy(), loops)
        .validate(Tolerance::get())
        .unwrap()
}

pub fn axis_y() -> RevolveAxis<f64> {
    RevolveAxis {
        origin: p2(0.0, 0.0),
        dir: Vec2::new(0.0, 1.0),
    }
}

/// The acceptance bodies, named, in fixed order (used by the export
/// tests and mirrored by the CI example).
pub fn acceptance_bodies() -> Vec<(&'static str, Body<f64>)> {
    vec![
        ("l_prism", l_prism()),
        ("holed_prism", holed_prism()),
        ("ball", ball()),
        ("cone", cone()),
        ("washer", washer()),
        ("donut", donut()),
        ("wedge", wedge()),
    ]
}

pub fn l_prism() -> Body<f64> {
    let lp = ProfileLoop::polygon([
        p2(0.0, 0.0),
        p2(2.0, 0.0),
        p2(2.0, 1.0),
        p2(1.0, 1.0),
        p2(1.0, 2.0),
        p2(0.0, 2.0),
    ]);
    extrude(&validated(vec![lp]), Extrusion::Distance(1.0))
        .unwrap()
        .body
}

/// 4×4 square with a centered circular hole (two-vertex closed
/// carrier), genus 1.
pub fn holed_prism() -> Body<f64> {
    let outer = ProfileLoop::polygon([p2(-2.0, -2.0), p2(2.0, -2.0), p2(2.0, 2.0), p2(-2.0, 2.0)]);
    let hole = ProfileLoop::new(vec![
        ProfileVertex {
            pos: p2(1.0, 0.0),
            bulge: 1.0,
        },
        ProfileVertex {
            pos: p2(-1.0, 0.0),
            bulge: 1.0,
        },
    ]);
    extrude(&validated(vec![outer, hole]), Extrusion::Distance(1.0))
        .unwrap()
        .body
}

pub fn ball() -> Body<f64> {
    let lp = ProfileLoop::new(vec![
        ProfileVertex {
            pos: p2(0.0, -1.0),
            bulge: 1.0,
        },
        ProfileVertex {
            pos: p2(0.0, 1.0),
            bulge: 0.0,
        },
    ]);
    revolve(&validated(vec![lp]), axis_y(), Revolution::Full)
        .unwrap()
        .body
}

pub fn cone() -> Body<f64> {
    let lp = ProfileLoop::polygon([p2(0.0, 0.0), p2(1.0, 0.0), p2(0.0, 1.0)]);
    revolve(&validated(vec![lp]), axis_y(), Revolution::Full)
        .unwrap()
        .body
}

pub fn washer() -> Body<f64> {
    let lp = ProfileLoop::polygon([p2(1.0, 0.0), p2(2.0, 0.0), p2(2.0, 1.0), p2(1.0, 1.0)]);
    revolve(&validated(vec![lp]), axis_y(), Revolution::Full)
        .unwrap()
        .body
}

pub fn donut() -> Body<f64> {
    let lp = ProfileLoop::new(vec![
        ProfileVertex {
            pos: p2(2.0, -0.5),
            bulge: 1.0,
        },
        ProfileVertex {
            pos: p2(2.0, 0.5),
            bulge: 1.0,
        },
    ]);
    revolve(&validated(vec![lp]), axis_y(), Revolution::Full)
        .unwrap()
        .body
}

pub fn wedge() -> Body<f64> {
    let lp = ProfileLoop::polygon([p2(1.0, 0.0), p2(2.0, 0.0), p2(2.0, 1.0), p2(1.0, 1.0)]);
    revolve(
        &validated(vec![lp]),
        axis_y(),
        Revolution::Partial(core::f64::consts::FRAC_PI_2),
    )
    .unwrap()
    .body
}
