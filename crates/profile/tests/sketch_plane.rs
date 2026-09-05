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

use geom_core::{Affine3, Mat3, Point2, Point3, Vec3};
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

#[test]
fn the_frame_accessors_read_back_exactly_what_from_frame_wrote() {
    // The accessors are PROJECTIONS of the stored placement, not a
    // recomputation, so the round trip is bitwise — including the
    // signed zero, which is why `origin` transcribes the translation
    // rather than adding it to the coordinate origin.
    let o = Point3::new(-0.0, 2.0, 3.5);
    let (u, v) = (Vec3::new(0.0, 1.0, 0.0), Vec3::new(0.0, 0.0, 1.0));
    let plane = SketchPlane::from_frame(o, u, v);

    assert_eq!(plane.origin().x.to_bits(), (-0.0f64).to_bits());
    assert_eq!(pt(plane.origin()), (-0.0, 2.0, 3.5));
    assert_eq!(vc(plane.u()), (0.0, 1.0, 0.0));
    assert_eq!(vc(plane.v()), (0.0, 0.0, 1.0));
    assert_eq!(vc(plane.normal()), normal(&plane));

    let rebuilt = SketchPlane::from_frame(plane.origin(), plane.u(), plane.v());
    assert!(plane.bit_eq(&rebuilt));
}

#[test]
fn plane_equality_is_bit_exact_and_the_two_zeros_differ() {
    // The `Doc::bit_eq` precedent (spec D7): a sketch plane carries no
    // ε, so the only honest equality it can offer compares BITS —
    // `-0.0` keeps its own identity rather than being folded away.
    let frame = |x: f64| {
        SketchPlane::from_frame(
            Point3::new(x, 0.0, 0.0),
            Vec3::new(1.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
        )
    };
    assert!(frame(0.0).bit_eq(&SketchPlane::xy()));
    assert!(!frame(0.0).bit_eq(&frame(-0.0)));
    assert!(frame(-0.0).bit_eq(&frame(-0.0)));
}

/// The twelve stored components of a placement, as bits — the
/// comparison `bit_eq` makes, spelled out so a row can hold an
/// `Affine3` against a `SketchPlane`.
fn bits(a: Affine3<f64>) -> [u64; 12] {
    let (l, t) = (a.linear, a.translation);
    [
        l.c0.x, l.c0.y, l.c0.z, l.c1.x, l.c1.y, l.c1.z, l.c2.x, l.c2.y, l.c2.z, t.x, t.y, t.z,
    ]
    .map(f64::to_bits)
}

/// Frames for the bit-identity rows: the canonical planes, a general
/// (non-orthonormal — unchecked by construction) triple, and every
/// signed-zero placement of the origin, which is the component the
/// `origin()` accessor's transcription exists to keep.
fn frame_corpus() -> Vec<(Point3<f64>, Vec3<f64>, Vec3<f64>)> {
    let mut corpus = vec![
        (Point3::origin(), Vec3::unit_x(), Vec3::unit_y()),
        (Point3::origin(), Vec3::unit_y(), Vec3::unit_z()),
        (Point3::origin(), Vec3::unit_z(), Vec3::unit_x()),
        (
            Point3::new(1.5, -2.25, 3.0e3),
            Vec3::unit_y(),
            Vec3::unit_z(),
        ),
        (
            Point3::new(-7.0, 0.5, 2.0),
            Vec3::new(0.3, -1.2, 2.5),
            Vec3::new(-4.0, 0.25, 1.0e-3),
        ),
    ];
    for sx in [0.0, -0.0] {
        for sy in [0.0, -0.0] {
            for sz in [0.0, -0.0] {
                corpus.push((
                    Point3::new(sx, sy, sz),
                    Vec3::unit_z(),
                    Vec3::new(-0.0, 1.0, 0.0),
                ));
            }
        }
    }
    corpus
}

#[test]
fn from_frame_stores_bit_for_bit_what_the_placement_door_builds() {
    // One home: `SketchPlane::from_frame` IS `Affine3::from_frame`, and
    // both are the explicit spelling `from_cols(u, v, u × v)` with the
    // origin's displacement from the chart base — the same operations
    // in the same order, so the twelve stored components agree by BITS
    // over the corpus, signed zeros included. This row is also the
    // measurement for a consumer that wants only the placement: it
    // reads the door directly rather than a plane's `.placement`, and
    // gets the same bits.
    for (o, u, v) in frame_corpus() {
        let plane = SketchPlane::from_frame(o, u, v);
        let door = Affine3::from_frame(o, u, v);
        let explicit = Affine3::from_parts(Mat3::from_cols(u, v, u.cross(v)), o - Point3::origin());
        assert_eq!(bits(plane.placement), bits(door));
        assert_eq!(bits(plane.placement), bits(explicit));
        assert!(plane.bit_eq(&SketchPlane::new(door)));
    }
}

#[test]
fn map_lifts_the_stored_frame_componentwise_without_recomputing_it() {
    // `map` is `Affine3::map` on the placement: twelve components
    // through `f`, no arithmetic. Under the identity every bit survives
    // (the signed zeros too); under negation every component is the
    // negated bit — and the normal is the SOURCE frame's `u × v`
    // negated, not the cross product of the negated axes (which would
    // be `u × v` again). That difference is what the two lift spellings
    // in the doc are about.
    for (o, u, v) in frame_corpus() {
        let plane = SketchPlane::from_frame(o, u, v);
        assert!(plane.map(|x| x).bit_eq(&plane));
        let neg = plane.map(|x: f64| -x);
        let want = bits(plane.placement)
            .map(f64::from_bits)
            .map(|x| (-x).to_bits());
        assert_eq!(bits(neg.placement), want);
        let rebuilt = SketchPlane::from_frame(
            Point3::new(-o.x, -o.y, -o.z),
            Vec3::new(-u.x, -u.y, -u.z),
            Vec3::new(-v.x, -v.y, -v.z),
        );
        let n = u.cross(v);
        assert_eq!(vc(rebuilt.normal()), (n.x, n.y, n.z));
        assert_eq!(vc(neg.normal()), (-n.x, -n.y, -n.z));
    }
}
