//! M5 PR 9c, scope item 1 (containment/pierce half): the SPHERE doors
//! of `topo::boolean::point_in_solid`.
//!
//! Before this unit `point_in_solid::face_geo` resolved `{Plane,
//! Cylinder}` only and refused every other kind as `CorruptFace` — so
//! a sphere operand could not be classified at all, which is the door
//! the cylinder×sphere boolean arm stands on. The arm this unit lands
//! covers the CLOSED sphere class: the faces sharing one sphere
//! surface close on each other, so their union is the whole chart and
//! no per-face chart trim is needed. That is exactly the ball the M5
//! inventory mints (`revolve_ball`'s V2 E2 F2: two half-bands on one
//! sphere key, joined along the seam meridian and its angle-π copy).
//!
//! Rows here:
//!
//! - **containment**: centre `In`, far point `Out`, a point ON the
//!   sphere `OnBoundary` — through the ray sweep, on a body whose only
//!   faces are spherical (so every row exercises the new arm and
//!   nothing else);
//! - **the group-closure refusal**: a PARTIAL revolve leaves the
//!   sphere band bounded by planar fan walls, so the group is not
//!   closed and the door refuses `PartialSphereFace` — the
//!   construction row for the chart-trim frontier this unit does NOT
//!   retire (the S9 flip pattern: every refusal is re-pinned as its
//!   construction row);
//! - **multi-ε honesty**: every probe offset is derived from the
//!   RESOLVED band (`Tolerance::get().eps`), never a literal, so the
//!   rows mean the same thing in each ε lane.
//!
//! The Interval lane lives in `m5_pr9c_sphere_doors_interval.rs`.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod revolve_common;

use geom_core::{Point3, Tolerance};
use profile::{ProfileLoop, ProfileVertex};
use revolve_common::*;
use sweep::{Revolution, revolve};
use topo::boolean::{PointInSolidError, SolidContainment, point_in_solid};

/// The half-disc of the `ball` acceptance: a unit semicircle from
/// (0, −1) through (1, 0) to (0, 1), closed by the on-axis diameter.
fn half_disc() -> ProfileLoop<f64> {
    ProfileLoop::new(vec![
        ProfileVertex {
            pos: p2(0.0, -1.0),
            bulge: 1.0,
        },
        ProfileVertex {
            pos: p2(0.0, 1.0),
            bulge: 0.0,
        },
    ])
}

/// The unit ball: two half-sphere bands on ONE sphere surface.
fn ball() -> topo::Body<f64> {
    let vp = validated(vec![half_disc()]);
    revolve(&vp, axis_y(), Revolution::Full).unwrap().body
}

fn band() -> geom_core::Band {
    geom_core::Band::linear().unwrap()
}

/// The probe offset: a multiple of the RESOLVED band, so each ε lane
/// probes at its own scale rather than at a hard-coded distance.
fn away() -> f64 {
    (1e6 * Tolerance::get().eps).max(0.25)
}

#[test]
fn sphere_door_classifies_the_ball_interior_exterior_and_boundary() {
    let body = ball();
    assert_all_tiers(&body);
    // Nothing but sphere faces — every row below is the new arm.
    assert_eq!(body.surfaces().count(), 1);
    let b = band();

    // Interior: the centre.
    assert_eq!(
        point_in_solid(&body, Point3::origin(), b).unwrap(),
        SolidContainment::In
    );
    // Interior: a band-scaled step inside the wall, off every axis so
    // no schedule ray runs along a symmetry.
    let inside = 1.0 - away();
    let d = inside / 3.0_f64.sqrt();
    assert_eq!(
        point_in_solid(&body, Point3::new(d, d, d), b).unwrap(),
        SolidContainment::In
    );
    // Exterior: the same direction, a band-scaled step outside.
    let outside = 1.0 + away();
    let e = outside / 3.0_f64.sqrt();
    assert_eq!(
        point_in_solid(&body, Point3::new(e, e, e), b).unwrap(),
        SolidContainment::Out
    );
    // Far exterior: the at-infinity side, reached with no crossing at
    // all on some schedule members.
    assert_eq!(
        point_in_solid(&body, Point3::new(10.0, 7.0, 3.0), b).unwrap(),
        SolidContainment::Out
    );
    // ON the sphere: the boundary pre-pass, before any ray is cast.
    for p in [
        Point3::new(1.0, 0.0, 0.0),
        Point3::new(0.0, 1.0, 0.0),
        Point3::new(0.0, 0.0, -1.0),
    ] {
        assert_eq!(
            point_in_solid(&body, p, b).unwrap(),
            SolidContainment::OnBoundary,
            "on-sphere probe {p:?}"
        );
    }
}

/// The pierce arm's own row: a query the boundary pre-pass cannot
/// answer (definitely off every face) whose verdict comes from the
/// ray/sphere quadratic — checked in BOTH directions across the wall
/// along a direction that is not in the schedule.
#[test]
fn sphere_pierce_reads_the_material_side_from_the_discriminant() {
    let body = ball();
    let b = band();
    let r = away();
    // A tangentially-placed pair: just inside / just outside the wall
    // at a generic point of the sphere.
    let n = Point3::new(0.6, 0.48, 0.64); // |n| = 1 exactly enough
    let s = n.distance(Point3::origin());
    let unit = (n - Point3::origin()) / s;
    let inner = Point3::origin() + unit * (1.0 - r);
    let outer = Point3::origin() + unit * (1.0 + r);
    assert_eq!(
        point_in_solid(&body, inner, b).unwrap(),
        SolidContainment::In
    );
    assert_eq!(
        point_in_solid(&body, outer, b).unwrap(),
        SolidContainment::Out
    );
}

/// The construction row for the frontier this unit does NOT retire:
/// a TRIMMED sphere face. A partial revolve caps the sphere band with
/// planar fan walls, so the sphere-surface face group has a boundary
/// against another surface and the door refuses typed rather than
/// guessing where the chart trim runs.
#[test]
fn trimmed_sphere_face_refuses_typed_partial_sphere_face() {
    let vp = validated(vec![half_disc()]);
    let t = revolve(
        &vp,
        axis_y(),
        Revolution::Partial(std::f64::consts::FRAC_PI_2),
    )
    .unwrap();
    let err = point_in_solid(&t.body, Point3::new(0.1, 0.1, 0.1), band()).unwrap_err();
    let PointInSolidError::PartialSphereFace { .. } = err else {
        panic!("expected PartialSphereFace, got {err:?}");
    };
    // The refusal states its own two-tolerance posture: a STRUCTURAL
    // arm with no in-band twin (S9 — definite arms say so too).
    let msg = err.to_string();
    assert!(msg.contains("STRUCTURAL"), "{msg}");
    assert!(msg.contains("no in-band twin"), "{msg}");
    assert!(msg.contains("Recourse"), "{msg}");
}
