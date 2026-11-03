//! **A rim arc lying on a cylinder, beside a spline wall.** A cylinder
//! `r = 0.5`, `z ∈ [0, 1]`, and a tube of the same section swept from its
//! top: the tube's flat start cap sits flush on the cylinder's top disc,
//! so each arc of the cap's rim lies on the cylinder's wall, and one of
//! the arc's two faces is the tube's spline wall. The boolean reads the
//! arc on the wall (`LiesOn`) and takes it when both its faces are
//! decided distinct from the wall; the spline one is read distinct where
//! every Bézier piece of its net has a point off the cylinder
//! (`reduce::spline_off_carrier`).
//!
//! The union then stops further on, describing the result's rim arc
//! between the cylinder and the spline wall: the wall's chart is two
//! rational arcs, and the arc's ends sit on their tangent-continuous
//! joint, which the describer reads as a crease
//! (`work/splice/a-closed-charts-seam-continuity-has-no-home.md`).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;

use geom::NurbsCurve3;
use geom_core::spline::KnotVector;
use geom_core::{Arc2, Point2, Point3, Tol};
use profile::{Profile, ProfileLoop, RawLoop, Segment, SketchPlane};
use sweep::{ExtrudeSide, Extrusion, extrude, sweep_body};
use topo::{AtRestBody, BooleanError, EdgeDescribeFailure};

use crate::common::normal_start_place;

const R: f64 = 0.5;

fn tol() -> Tol {
    Tol::witness()
}

/// The circle of radius [`R`] about the sketch origin, as two half arcs.
fn circle() -> ProfileLoop<f64> {
    RawLoop::new((0..2).map(|k| {
        let at = PI * f64::from(k);
        (
            Point2::new(R * at.cos(), R * at.sin()),
            Segment::Arc(Arc2 {
                centre: Point2::new(0.0, 0.0),
                radius: R,
                sweep: PI,
            }),
        )
    }))
}

/// The cylinder `r = R`, `z ∈ [0, 1]`.
fn cylinder() -> AtRestBody<f64> {
    let profile = Profile::new(SketchPlane::<f64>::xy(), vec![circle()])
        .validate(tol())
        .unwrap();
    let body = extrude(
        &profile,
        Extrusion::Distance {
            depth: 1.0,
            side: ExtrudeSide::Along,
        },
        tol(),
    )
    .unwrap()
    .body;
    topo::test_support::finished("the cylinder", body, tol())
}

/// The quadratic spine from `(0, 0, 1)` leaving along `+z`, through
/// `(0, 0, 2)` toward `end`.
fn spine(end: Point3<f64>) -> NurbsCurve3<f64> {
    let kv = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
    let control = vec![Point3::new(0.0, 0.0, 1.0), Point3::new(0.0, 0.0, 2.0), end];
    NurbsCurve3::new(kv, control, vec![1.0; 3]).unwrap()
}

/// The circle swept along [`spine`] to `end`, at nine stations, cubic
/// along the spine.
fn tube(end: Point3<f64>) -> AtRestBody<f64> {
    let path = spine(end);
    let body = sweep_body::<f64>(&[circle()], normal_start_place(&path), &path, 9, 3, tol())
        .unwrap()
        .body;
    topo::test_support::finished("the tube", body, tol())
}

/// The tube bends toward `+x`, so its wall leaves the cylinder above
/// the rim on every piece: the rim arc is taken, and the union goes on.
/// Where it stops depends on the band. At a band of `1e-6` the tube's
/// edge 4v1, which leaves the rim tangent to the cylinder, is read in
/// band of the wall (an honest escalation, a second-order departure
/// inside a wide band). Elsewhere it stops describing the result's rim
/// arc on the wall's arc joint.
#[test]
fn a_rim_arc_beside_a_bent_spline_wall_is_taken_on_the_cylinder() {
    let got = topo::union(&cylinder(), &tube(Point3::new(1.5, 0.0, 3.0)), tol());
    assert!(
        !matches!(got, Err(BooleanError::CurvedPierceUnsupported { .. })),
        "the cap's rim arc lies on the cylinder beside a spline wall off it: {got:?}"
    );
    let key = |k: &dyn core::fmt::Debug| format!("{k:?}");
    let wide = geom_core::Band::linear(tol()).unwrap().zero() > 1e-7;
    if wide {
        assert!(
            matches!(
                &got,
                Err(BooleanError::CurvedPierceEscalated { operand: topo::Operand::B, face, edge, diag, .. })
                    if key(face) == "FaceKey(3v1)"
                        && key(edge) == "EdgeKey(4v1)"
                        && diag.predicate == Some("spline_composite_past_the_band")
            ),
            "at a wide band the tube's tangent edge escalates against the wall: {got:?}"
        );
    } else {
        assert!(
            matches!(
                &got,
                Err(BooleanError::EdgeUndescribed {
                    edge,
                    failure: EdgeDescribeFailure::Crease,
                    cause: None,
                    fitted: false,
                }) if key(edge) == "EdgeKey(10v1)"
            ),
            "the union stops describing the result's rim arc on the wall's arc joint: {got:?}"
        );
    }
}
