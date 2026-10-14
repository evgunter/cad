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

use geom_core::{Affine3, Dual64, Interval, Mat3, Point2, Point3, Real, RigidMotion, Vec3};
use profile::SketchPlane;

use crate::common::frame_of;

/// Point3/Vec3 carry no `PartialEq` (a geometric type is not an
/// equatable value in this kernel), so rows compare the components.
fn pt(p: Point3<f64>) -> (f64, f64, f64) {
    (p.x, p.y, p.z)
}

fn vc(v: Vec3<f64>) -> (f64, f64, f64) {
    (v.x, v.y, v.z)
}

/// The plane's normal: the third column of the placement's linear
/// part, the frame's third axis (see the `SketchPlane` docs).
fn normal(plane: &SketchPlane<f64>) -> (f64, f64, f64) {
    vc(plane.placement().linear.c2)
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
    let spelled_all = [
        SketchPlane::from_frame(RigidMotion::axes_xy(Point3::origin())),
        SketchPlane::from_frame(RigidMotion::axes_yz(Point3::origin())),
        SketchPlane::from_frame(RigidMotion::axes_zx(Point3::origin())),
    ];
    for (k, plane) in named.iter().enumerate() {
        let spelled = spelled_all[k];
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
    // The accessors are the stored frame's own origin and axes, not a
    // recomputation, so the round trip is bitwise — including the
    // signed zero.
    let o = Point3::new(-0.0, 2.0, 3.5);
    let frame = RigidMotion::axes_yz(o);
    let plane = SketchPlane::from_frame(frame);

    assert_eq!(plane.origin().x.to_bits(), (-0.0f64).to_bits());
    assert_eq!(pt(plane.origin()), (-0.0, 2.0, 3.5));
    assert_eq!(vc(plane.u().get()), (0.0, 1.0, 0.0));
    assert_eq!(vc(plane.v().get()), (0.0, 0.0, 1.0));
    assert_eq!(vc(plane.normal().get()), normal(&plane));

    // The accessors are the frame's own three axes and its origin.
    assert_eq!(vc(plane.u().get()), vc(frame.u().get()));
    assert_eq!(vc(plane.v().get()), vc(frame.v().get()));
    assert_eq!(vc(plane.normal().get()), vc(frame.w().get()));
    assert!(plane.bit_eq(&SketchPlane::from_frame(frame)));
}

#[test]
fn plane_equality_is_bit_exact_and_the_two_zeros_differ() {
    // The `Doc::bit_eq` precedent (spec D7): a sketch plane carries no
    // ε, so the only honest equality it can offer compares BITS —
    // `-0.0` keeps its own identity rather than being folded away.
    let frame = |x: f64| SketchPlane::from_frame(RigidMotion::axes_xy(Point3::new(x, 0.0, 0.0)));
    assert!(frame(0.0).bit_eq(&SketchPlane::xy()));
    assert!(!frame(0.0).bit_eq(&frame(-0.0)));
    assert!(frame(-0.0).bit_eq(&frame(-0.0)));
}

#[test]
fn the_partial_eq_impl_is_bit_eq_and_answers_the_same_on_the_two_zeros() {
    // `==` delegates to `bit_eq`, so the two never disagree — the
    // claim a bug could break is a delegation that read some other
    // comparison, which the `-0.0` row below is exactly the case for:
    // any tolerant or IEEE equality calls those two planes the same
    // and `bit_eq` calls them different.
    let frame = |x: f64| SketchPlane::from_frame(RigidMotion::axes_xy(Point3::new(x, 0.0, 0.0)));
    let rows = [
        frame(0.0),
        frame(-0.0),
        SketchPlane::xy(),
        SketchPlane::yz(),
    ];
    for left in rows {
        for right in rows {
            assert_eq!(left == right, left.bit_eq(&right));
        }
    }
    assert_eq!(frame(0.0), SketchPlane::xy());
    assert_ne!(frame(0.0), frame(-0.0));
    assert_eq!(frame(-0.0), frame(-0.0));
}

/// The twelve stored components of a placement, as bits — the
/// comparison `bit_eq` makes, read through the same door so a row can
/// hold an `Affine3` against a `SketchPlane`. The door's bit-identity
/// corpus (every sign pattern of zeros, extremes, a generated sweep)
/// lives with the door, in `geom-core`'s `affine.rs` tests; this suite
/// keeps the delegation row on a handful of frames and no corpus.
fn bits(a: Affine3<f64>) -> [u64; 12] {
    a.components().map(f64::to_bits)
}

/// The canonical planes, one general triple, one frame of signed
/// zeros — enough to pin that the plane IS the door, not to sweep it.
const FRAMES: [(Point3<f64>, Vec3<f64>, Vec3<f64>); 5] = [
    (
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(1.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
    ),
    (
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        Vec3::new(0.0, 0.0, 1.0),
    ),
    (
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 1.0),
        Vec3::new(1.0, 0.0, 0.0),
    ),
    (
        Point3::new(-7.0, 0.5, 2.0),
        Vec3::new(0.3, -1.2, 2.5),
        Vec3::new(-4.0, 0.25, 1.0e-3),
    ),
    (
        Point3::new(-0.0, 0.0, -0.0),
        Vec3::new(-0.0, 1.0, 0.0),
        Vec3::new(0.0, -0.0, 1.0),
    ),
];

#[test]
fn the_placement_is_the_witness_conversion_bit_for_bit() {
    // One home: `SketchPlane::placement` IS `RigidMotion::to_affine` —
    // a derivation, so the twelve components agree by BITS, and the
    // columns are the frame's own three axes. The conversion's own
    // identity with the hand spelling, over the wide corpus, is pinned
    // beside the type in `geom-core`.
    for (o, u, v) in FRAMES {
        let frame = frame_of(o, u, v);
        let plane = SketchPlane::from_frame(frame);
        assert_eq!(bits(plane.placement()), bits(frame.to_affine()));
        let hand = Affine3::from_parts(
            Mat3::from_cols(frame.u().get(), frame.v().get(), frame.w().get()),
            o - Point3::origin(),
        );
        assert_eq!(bits(plane.placement()), bits(hand));
    }
}

#[test]
fn point_value_reads_a_plane_minted_at_a_lane_scalar_back_bit_for_bit() {
    // A plane exists at a lane scalar only by being minted there. Minted
    // at the decided door from the `f64` plane's twelve entries as point
    // values, it holds those entries verbatim, so `point_value` returns
    // every bit at `Interval` and through `Dual64`'s value channel, the
    // signed-zero frame included.
    let band = geom_core::Band::linear(geom_core::Tol::witness()).unwrap();
    for (o, u, v) in FRAMES {
        let plane = SketchPlane::from_frame(frame_of(o, u, v));
        let at_interval = SketchPlane::from_frame(
            RigidMotion::check_rigid(&plane.placement().map(Interval::from_f64), band)
                .expect("a rigid f64 frame is rigid as point intervals"),
        );
        let back = at_interval.point_value().expect("a point plane reads back");
        assert!(back.bit_eq(&plane), "Interval round trip at {o:?}");
        let at_dual = SketchPlane::from_frame(
            RigidMotion::check_rigid(&plane.placement().map(Dual64::from_f64), band)
                .expect("a rigid f64 frame is rigid at Dual64"),
        );
        let back = at_dual
            .point_value()
            .expect("a dual plane's value channel reads back");
        assert!(back.bit_eq(&plane), "Dual64 round trip at {o:?}");
    }
}

#[test]
fn point_value_refuses_a_plane_wider_than_a_point() {
    // A frame minted at `Interval` from a WIDE first axis has no single
    // `f64` frame to read: `point_value` answers `None` rather than a
    // midpoint, which would invent a frame the enclosure does not name.
    let band = geom_core::Band::linear(geom_core::Tol::witness()).unwrap();
    let wide = RigidMotion::gram_schmidt(
        Point3::new(
            Interval::from_f64(0.0),
            Interval::from_f64(0.0),
            Interval::from_f64(0.0),
        ),
        Vec3::new(
            Interval::from_bounds(0.9, 1.1),
            Interval::from_f64(0.0),
            Interval::from_f64(0.0),
        ),
        Vec3::new(
            Interval::from_f64(0.0),
            Interval::from_f64(1.0),
            Interval::from_f64(0.0),
        ),
        "sketch_plane_test_axis",
        band,
    )
    .expect("a wide pair that spans a plane");
    assert!(SketchPlane::from_frame(wide).point_value().is_none());
    let point = SketchPlane::<Interval>::xy();
    assert!(
        point.point_value().is_some(),
        "the control: a point plane reads"
    );
}
