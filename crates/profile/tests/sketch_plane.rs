//! The canonical sketch planes, pinned by what they DO to points.
//!
//! `SketchPlane` is conventional data with zero geometry attached, so
//! the only thing worth pinning about `xy`/`yz`/`zx` is the map they
//! induce: where a sketch point lands in the world, and which way the
//! normal — the third placement column, the direction `extrude` runs —
//! points. The invariant these rows hold is the CYCLIC convention
//! x→y→z→x that the demo tour's letterform captions already speak
//! ("a yz sketch extruded +x", "a zx sketch extruded +y").
//!
//! Every comparison is EXACT: the canonical frames are made of 0 and
//! 1, and `to_world` is one affine apply, so an inexact answer would
//! be a real defect rather than float noise.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use geom_core::{Point2, Point3, Vec3};
use profile::SketchPlane;

/// Point3/Vec3 carry no `PartialEq` (a geometric type is not an
/// equatable value in this kernel), so rows compare the components.
fn pt(p: Point3<f64>) -> (f64, f64, f64) {
    (p.x, p.y, p.z)
}

fn vc(v: Vec3<f64>) -> (f64, f64, f64) {
    (v.x, v.y, v.z)
}

/// The plane's normal: the third column of the placement's linear
/// part, i.e. u × v (see the `SketchPlane` docs).
fn normal(plane: &SketchPlane<f64>) -> (f64, f64, f64) {
    vc(plane.placement.linear.c2)
}

#[test]
fn xy_maps_sketch_xy_to_world_xy_with_a_plus_z_normal() {
    let plane = SketchPlane::<f64>::xy();
    assert_eq!(pt(plane.to_world(Point2::new(3.0, 5.0))), (3.0, 5.0, 0.0));
    assert_eq!(normal(&plane), (0.0, 0.0, 1.0));
}

#[test]
fn yz_maps_sketch_xy_to_world_yz_with_a_plus_x_normal() {
    let plane = SketchPlane::<f64>::yz();
    // Sketch (x, y) ↦ world (0, x, y): u = ŷ, v = ẑ.
    assert_eq!(pt(plane.to_world(Point2::new(3.0, 5.0))), (0.0, 3.0, 5.0));
    assert_eq!(normal(&plane), (1.0, 0.0, 0.0));
}

#[test]
fn zx_maps_sketch_xy_to_world_zx_with_a_plus_y_normal() {
    let plane = SketchPlane::<f64>::zx();
    // Sketch (x, y) ↦ world (y, 0, x): u = ẑ, v = x̂.
    assert_eq!(pt(plane.to_world(Point2::new(3.0, 5.0))), (5.0, 0.0, 3.0));
    assert_eq!(normal(&plane), (0.0, 1.0, 0.0));
}

#[test]
fn each_canonical_plane_is_its_own_from_frame_spelling() {
    // ONE construction: the named planes are `from_frame` at the world
    // origin, so nothing can drift between the sugar and the general
    // door (the same seam the bindings reuse for `plane=`).
    let named = [
        SketchPlane::<f64>::xy(),
        SketchPlane::yz(),
        SketchPlane::zx(),
    ];
    let axes = [Vec3::unit_x(), Vec3::unit_y(), Vec3::unit_z()];
    for (k, plane) in named.iter().enumerate() {
        let spelled = SketchPlane::from_frame(Point3::origin(), axes[k], axes[(k + 1) % 3]);
        for p in [
            Point2::new(0.0, 0.0),
            Point2::new(1.0, 0.0),
            Point2::new(0.0, 1.0),
            Point2::new(-2.5, 7.25),
        ] {
            assert_eq!(pt(plane.to_world(p)), pt(spelled.to_world(p)));
        }
        assert_eq!(normal(plane), normal(&spelled));
    }
}

#[test]
fn the_three_planes_round_trip_the_sketch_basis_cyclically() {
    // The round trip that makes "cyclic" checkable rather than
    // asserted: each plane's (û, v̂) is the next pair in x→y→z→x, so
    // the sketch unit square lands on a different world coordinate
    // plane for each, and the three normals are the three world axes
    // exactly once.
    let planes = [
        SketchPlane::<f64>::xy(),
        SketchPlane::yz(),
        SketchPlane::zx(),
    ];
    let axes = [(1.0, 0.0, 0.0), (0.0, 1.0, 0.0), (0.0, 0.0, 1.0)];
    for (k, plane) in planes.iter().enumerate() {
        let u = plane.to_world(Point2::new(1.0, 0.0)) - Point3::origin();
        let v = plane.to_world(Point2::new(0.0, 1.0)) - Point3::origin();
        assert_eq!(vc(u), axes[k]);
        assert_eq!(vc(v), axes[(k + 1) % 3]);
        assert_eq!(normal(plane), axes[(k + 2) % 3]);
        assert_eq!(pt(plane.to_world(Point2::new(0.0, 0.0))), (0.0, 0.0, 0.0));
    }
}
