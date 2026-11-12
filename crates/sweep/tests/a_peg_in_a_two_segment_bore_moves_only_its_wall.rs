//! **A peg glued into one segment of a two-segment bore moves only its
//! own wall** (`docs/DESIGN.md` `[coincidence.door]`; `topo`'s README
//! `[topo.one-record.kept]`).
//!
//! The bore is two full-turn collars stacked on one axis, `y ∈ [0, 1]`
//! and `[1, 6]`, their union leaving its wall as two faces on one record
//! joined by a circle: the lower segment `F2` and the upper `X`. A solid
//! peg of the bore's radius, `y ∈ [0.25, 0.75]`, turned `0.5 ε` about
//! `z` through its middle, rests against `F2`. Its wall reads one
//! carrier with `F2` over the two of them, and in band with the bore's
//! wall over the whole bore (both asserted: the witness has teeth).
//!
//! The bore's record is also worn by `X`, which no verdict decided, so it
//! is kept in both operand orders: the peg's wall alone moves, the glue's
//! verdict is read over the peg and `F2` only, and the union builds. A
//! first-written rule would keep the peg's record where the peg comes
//! first, move `F2` and `X` with it, and read the verdict over the whole
//! bore, in band.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::mate2_common::{boolean_body, full_turn_collar, volume};
use geom_core::{Point2, Point3, Tol, Vec2, Vec3};
use profile::{Profile, ProfileLoop, RawLoop, SketchPlane};
use sweep::test_support::finished;
use sweep::{Revolution, RevolveAxis, revolve};
use topo::{AtRestBody, Body};

/// The bore's radius.
const R: f64 = 0.5;

/// A solid rod of radius `r` along `y`, `y ∈ [y0, y1]`: one wall face.
fn rod(r: f64, (y0, y1): (f64, f64)) -> Body<f64> {
    let lp = ProfileLoop::polygon([
        Point2::new(0.0, y0),
        Point2::new(r, y0),
        Point2::new(r, y1),
        Point2::new(0.0, y1),
    ]);
    let vp = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(Tol::witness())
        .unwrap();
    let axis = RevolveAxis {
        origin: Point2::new(0.0, 0.0),
        dir: Vec2::new(0.0, 1.0),
    };
    revolve(&vp, axis, Revolution::Full, Tol::witness())
        .unwrap()
        .body
        .into_body()
}

/// The faces of `body` on a cylinder of radius [`R`].
fn walls(body: &Body<f64>) -> Vec<topo::FaceKey> {
    body.faces()
        .filter(|&(f, _)| {
            matches!(
                body.face_surface(f),
                Some(geom::Surface::Cylinder { radius, .. }) if (radius - R).abs() < 1e-9
            )
        })
        .map(|(f, _)| f)
        .collect()
}

/// The `y` span of `face`'s loops.
fn y_span(body: &Body<f64>, face: topo::FaceKey) -> (f64, f64) {
    body.face(face)
        .unwrap()
        .loops()
        .filter_map(|(_, l, _)| match l.boundary {
            topo::LoopBoundary::Cycle { first } => body.loop_cycle(first),
            topo::LoopBoundary::Empty { .. } => None,
        })
        .flatten()
        .filter_map(|he| body.half_edge_start_point(he))
        .fold((f64::INFINITY, f64::NEG_INFINITY), |(lo, hi), p| {
            (lo.min(p.y), hi.max(p.y))
        })
}

/// The ball enclosing the cylinder of radius [`R`] about `y` over
/// `[y0, y1]`.
fn ball(y0: f64, y1: f64) -> topo::ConsumedExtent<f64> {
    topo::ConsumedExtent::new(geom_brep::ExtentBall::of_box(
        Point3::new(-R, y0, -R),
        Point3::new(R, y1, R),
    ))
}

#[test]
fn a_peg_in_one_segment_of_a_bore_moves_only_its_wall_in_both_orders() {
    let tol = Tol::witness();
    let eps = tol.eps();
    let band = geom_core::Band::linear(tol).unwrap();
    let lo = finished(
        "the lower collar",
        full_turn_collar(R, 1.5, (0.0, 1.0)),
        tol,
    );
    let hi = finished(
        "the upper collar",
        full_turn_collar(R, 1.5, (1.0, 6.0)),
        tol,
    );
    let bore: AtRestBody<f64> = boolean_body(topo::union(&lo, &hi, tol).unwrap()).body;
    let bore_walls = walls(&bore);
    assert_eq!(bore_walls.len(), 2, "the bore's wall is two faces");
    let records: Vec<_> = bore_walls
        .iter()
        .map(|&f| bore.get_face(f).unwrap().surface)
        .collect();
    assert_eq!(records[0], records[1], "on one record");
    let f2 = *bore_walls
        .iter()
        .find(|&&f| y_span(&bore, f) == (0.0, 1.0))
        .expect("the lower segment");

    let pivot = Point3::new(0.0, 0.5, 0.0);
    let turn = |t: Vec3<f64>| {
        geom_core::RigidMotion::rotation_then_translation(
            geom_core::UnitVec3::exact_z(),
            0.5 * eps,
            t,
        )
        .unwrap()
    };
    let moved = turn(Vec3::zero()).transform_point(pivot);
    let peg = finished(
        "the peg",
        topo::transform_rigid(&rod(R, (0.25, 0.75)), &turn(pivot - moved)).unwrap(),
        tol,
    );
    let peg_wall = walls(&peg)[0];

    let (cp, cb) = (
        topo::face_carrier(&peg, peg_wall).unwrap(),
        topo::face_carrier(&bore, f2).unwrap(),
    );
    assert!(
        matches!(
            topo::carrier_eq(&cp, &cb, &ball(0.0, 1.0), band),
            Ok(topo::CarrierRelation::SameOriented | topo::CarrierRelation::SameOpposite)
        ),
        "the peg's wall is one carrier with F2 over the two of them"
    );
    assert!(
        topo::carrier_eq(&cp, &cb, &ball(0.0, 6.0), band).is_err(),
        "the peg's wall is in band of the bore's over the whole bore"
    );

    let sum = volume(&bore) + volume(&peg);
    for (order, x, y) in [("peg ∪ bore", &peg, &bore), ("bore ∪ peg", &bore, &peg)] {
        let u = boolean_body(
            topo::union(x, y, tol).unwrap_or_else(|e| panic!("{order}: the union builds: {e:?}")),
        )
        .body;
        let v = volume(&u);
        assert!(
            (v - sum).abs() <= eps,
            "{order}: {v} against the operands' {sum}"
        );
        // Every bore wall face left wears the bore's record: its axis is
        // the bore's own `y`, not the peg's turned one.
        let left = walls(&u);
        assert!(!left.is_empty(), "{order}: the bore's wall survives");
        for f in left {
            let Some(geom::Surface::Cylinder { axis, .. }) = u.face_surface(f) else {
                unreachable!("a wall face is a cylinder");
            };
            assert_eq!(
                (axis.x, axis.z),
                (0.0, 0.0),
                "{order}: {f:?} spanning {:?} keeps the bore's record",
                y_span(&u, f)
            );
        }
    }
}
