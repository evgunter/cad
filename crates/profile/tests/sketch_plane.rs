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

use core::cell::Cell;
use core::convert::Infallible;

use geom_core::{Affine3, Mat3, OrthoFrame, Point2, Point3, Vec3};
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
    let spelled_all = [
        SketchPlane::from_frame(OrthoFrame::axes_xy(Point3::origin())),
        SketchPlane::from_frame(OrthoFrame::axes_yz(Point3::origin())),
        SketchPlane::from_frame(OrthoFrame::axes_zx(Point3::origin())),
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
    // The accessors are PROJECTIONS of the stored placement, not a
    // recomputation, so the round trip is bitwise — including the
    // signed zero, which is why `origin` transcribes the translation
    // rather than adding it to the coordinate origin.
    let o = Point3::new(-0.0, 2.0, 3.5);
    let frame = OrthoFrame::axes_yz(o);
    let plane = SketchPlane::from_frame(frame);

    assert_eq!(plane.origin().x.to_bits(), (-0.0f64).to_bits());
    assert_eq!(pt(plane.origin()), (-0.0, 2.0, 3.5));
    assert_eq!(vc(plane.u()), (0.0, 1.0, 0.0));
    assert_eq!(vc(plane.v()), (0.0, 0.0, 1.0));
    assert_eq!(vc(plane.normal()), normal(&plane));

    // The accessors are the frame's own three axes and its origin,
    // read off the placement they were written into.
    assert_eq!(vc(plane.u()), vc(frame.u().get()));
    assert_eq!(vc(plane.v()), vc(frame.v().get()));
    assert_eq!(vc(plane.normal()), vc(frame.w().get()));
    assert!(plane.bit_eq(&SketchPlane::from_frame(frame)));
}

#[test]
fn plane_equality_is_bit_exact_and_the_two_zeros_differ() {
    // The `Doc::bit_eq` precedent (spec D7): a sketch plane carries no
    // ε, so the only honest equality it can offer compares BITS —
    // `-0.0` keeps its own identity rather than being folded away.
    let frame = |x: f64| SketchPlane::from_frame(OrthoFrame::axes_xy(Point3::new(x, 0.0, 0.0)));
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
    let frame = |x: f64| SketchPlane::from_frame(OrthoFrame::axes_xy(Point3::new(x, 0.0, 0.0)));
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
/// comparison `bit_eq` makes, spelled out so a row can hold an
/// `Affine3` against a `SketchPlane`. The door's bit-identity corpus
/// (every sign pattern of zeros, extremes, a generated sweep) lives
/// with the door, in `geom-core`'s `affine.rs` tests; it is not
/// reachable from here without a new dev-dependency, so this suite
/// keeps the delegation row on a handful of frames and no corpus.
fn bits(a: Affine3<f64>) -> [u64; 12] {
    let (l, t) = (a.linear, a.translation);
    [
        l.c0.x, l.c0.y, l.c0.z, l.c1.x, l.c1.y, l.c1.z, l.c2.x, l.c2.y, l.c2.z, t.x, t.y, t.z,
    ]
    .map(f64::to_bits)
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
fn from_frame_is_the_witness_conversion_bit_for_bit() {
    // One home: `SketchPlane::from_frame` IS `OrthoFrame::to_affine` —
    // a delegation, so the twelve stored components agree by BITS, and
    // the columns are the frame's own three axes. The conversion's own
    // identity with the hand spelling, over the wide corpus, is pinned
    // beside the type in `geom-core`.
    for (o, u, v) in FRAMES {
        let frame = frame_of(o, u, v);
        let plane = SketchPlane::from_frame(frame);
        assert_eq!(bits(plane.placement), bits(frame.to_affine()));
        assert!(plane.bit_eq(&SketchPlane::new(frame.to_affine())));
        let hand = Affine3::from_parts(
            Mat3::from_cols(frame.u().get(), frame.v().get(), frame.w().get()),
            o - Point3::origin(),
        );
        assert_eq!(bits(plane.placement), bits(hand));
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
    for (o, u, v) in FRAMES {
        let frame = frame_of(o, u, v);
        let plane = SketchPlane::from_frame(frame);
        assert!(plane.map(|x| x).bit_eq(&plane));
        let neg = plane.map(|x: f64| -x);
        let want = bits(plane.placement)
            .map(f64::from_bits)
            .map(|x| (-x).to_bits());
        assert_eq!(bits(neg.placement), want);
        // The lifted normal is the SOURCE frame's `u × v` negated; the
        // cross product of the two negated axes is that same product
        // unnegated, which is the difference the two lift spellings in
        // the doc are about.
        let n = frame.w().get();
        let crossed_after = (-frame.u().get()).cross(-frame.v().get());
        assert_eq!(vc(neg.normal()), (-n.x, -n.y, -n.z));
        assert_eq!(vc(crossed_after), (n.x, n.y, n.z));
    }
}

#[test]
fn try_map_is_the_fallible_direction_of_map_over_the_same_twelve_components() {
    // `try_map` is `Affine3::try_map` on the placement exactly as `map`
    // is `Affine3::map`: under an `f` that cannot refuse, the twelve
    // stored components come back where `map` puts them, bit for bit,
    // the signed-zero frame included. Nothing is recomputed — the
    // stored normal is carried as a value, which is what the doc's two
    // lift spellings are about and is unchanged in this direction.
    for (o, u, v) in FRAMES {
        let plane = SketchPlane::from_frame(frame_of(o, u, v));
        let walk = plane
            .try_map(|x: f64| Ok::<f64, Infallible>(x))
            .unwrap_or_else(|never| match never {});
        assert_eq!(bits(walk.placement), bits(plane.map(|x| x).placement));
        assert_eq!(bits(walk.placement), bits(plane.placement));
    }
}

#[test]
fn try_map_returns_the_first_refusal_in_its_place_and_builds_no_plane() {
    // A refusal is the whole answer: no `SketchPlane` is constructed
    // around a partly-walked placement. The walk is twelve components
    // long and short-circuits, so refusing from the k-th on yields the
    // k-th refusal after exactly `k + 1` calls — a plane door that
    // collected all twelve and picked would report eleven and twelve.
    //
    // The refusal carries the component's own VALUE, over a placement
    // whose twelve components are all distinct, so the loop
    // discriminates PLACEMENT and not only order: a transposed column
    // changes which value comes out at which k. (The canonical planes
    // could not do this — their components are zeros and ones, so a
    // transposition leaves the refused value unchanged.)
    let plane = SketchPlane::new(Affine3::from_parts(
        Mat3::from_cols(
            Vec3::new(1.0, 2.0, 3.0),
            Vec3::new(4.0, 5.5, -6.0),
            Vec3::new(-7.25, 0.5, 8.0),
        ),
        Vec3::new(10.0, 11.0, 12.0),
    ));
    let want = bits(plane.placement);
    for (k, expected) in want.iter().enumerate() {
        let calls = Cell::new(0usize);
        let got = plane.try_map(|x: f64| {
            let i = calls.get();
            calls.set(i + 1);
            if i < k { Ok(x) } else { Err(x.to_bits()) }
        });
        assert_eq!(
            got.err(),
            Some(*expected),
            "the first refusal carries component {k}'s own value"
        );
        assert_eq!(
            calls.get(),
            k + 1,
            "nothing after component {k} is consulted"
        );
    }
    // And with no refusal anywhere, all twelve are visited once.
    let calls = Cell::new(0usize);
    let all = plane.try_map(|x: f64| {
        calls.set(calls.get() + 1);
        Ok::<f64, ()>(x)
    });
    assert_eq!(bits(all.unwrap().placement), want);
    assert_eq!(calls.get(), 12);
}
