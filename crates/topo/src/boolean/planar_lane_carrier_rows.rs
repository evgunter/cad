//! **A curved carrier with no crossing lane, at the sweep's planar arm.**
//!
//! The operand is a planar sheet in `z = 0` bounded by a quadratic
//! Bézier arc from `(0, 0, 0)` through the control point `(mx, 2h, 0)`
//! to `(2, 0, 0)` (a NURBS carrier) and the chord back along `y = 0`.
//! The partner is a brick with a side face across the arc, or a
//! cylinder wall. `A`'s direction of the sweep runs directly, past the
//! operand gate that refuses the carrier first in the pipeline, so the
//! rows read what the sweep's arms themselves do with it.
//!
//! The oracle is the Bézier in closed form,
//! `x(t) = 2t(1−t)·mx + 2t²`, `y(t) = 4t(1−t)·h`, never the kernel's
//! evaluator: each row first shows from it that the arc crosses the face
//! where an endpoint reading of the edge says something else.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::boolean::BooleanError;
use crate::euler::{FaceSurface, MefSite, MevSite};
use crate::test_support_fixtures::brick;
use crate::{Body, FaceKey};
use geom::Curve3;
use geom_brep::{EdgeCurveSpec, EdgeDescriptionSpec};
use geom_core::{Point3, Tol, Vec3};

/// The Bézier arc's control net, peak height `h`.
fn arc(mx: f64, h: f64) -> Curve3<f64> {
    let kv = geom_core::spline::KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
    Curve3::Nurbs(std::sync::Arc::new(
        geom::NurbsCurve3::new(
            kv,
            vec![
                Point3::new(0.0, 0.0, 0.0),
                Point3::new(mx, 2.0 * h, 0.0),
                Point3::new(2.0, 0.0, 0.0),
            ],
            vec![1.0; 3],
        )
        .unwrap(),
    ))
}

/// The arc's image in the plane chart (origin `0`, `u_ref = x̂`): the
/// same net, read in `(x, y)`.
fn arc_image(mx: f64, h: f64) -> geom_brep::Pcurve<f64> {
    let kv = geom_core::spline::KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
    geom_brep::Pcurve::Fitted(std::sync::Arc::new(
        geom::NurbsCurve2::new(
            kv,
            vec![
                geom_core::Point2::new(0.0, 0.0),
                geom_core::Point2::new(mx, 2.0 * h),
                geom_core::Point2::new(2.0, 0.0),
            ],
            vec![1.0; 3],
        )
        .unwrap(),
    ))
}

/// The sheet, and its arc edge.
fn arc_sheet(mx: f64, h: f64) -> (Body<f64>, crate::EdgeKey) {
    let tol = Tol::witness();
    let mut body = Body::<f64>::new();
    let (p0, p1) = (Point3::new(0.0, 0.0, 0.0), Point3::new(2.0, 0.0, 0.0));
    let seed = body.mvfs(p0, true).unwrap();
    let plane = body
        .set_face_surface(
            seed.face,
            FaceSurface::New {
                surface: geom::Surface::Plane {
                    origin: p0,
                    normal: Vec3::new(0.0, 0.0, 1.0),
                    u_ref: Vec3::new(1.0, 0.0, 0.0),
                },
                sense: true,
            },
        )
        .unwrap();
    let e = body
        .mev(
            MevSite::Lone {
                r#loop: seed.r#loop,
            },
            p1,
            EdgeCurveSpec {
                description: EdgeDescriptionSpec::Chart {
                    surface: plane,
                    image: Some(arc_image(mx, h)),
                    seam: false,
                    declared: None,
                },
                carrier: arc(mx, h),
                param_start: 0.0,
                param_end: 1.0,
            },
            tol,
        )
        .unwrap();
    body.mef(
        MefSite::Chords {
            he1: e.he_minus,
            he2: e.he_plus,
        },
        EdgeCurveSpec {
            description: EdgeDescriptionSpec::chart(plane),
            carrier: Curve3::Line {
                origin: p1,
                dir: Vec3::new(-1.0, 0.0, 0.0),
            },
            param_start: 0.0,
            param_end: 2.0,
        },
        FaceSurface::Shared {
            key: plane,
            sense: true,
        },
        tol,
    )
    .unwrap();
    crate::pcurves::mint_pcurves(&mut body, tol).unwrap();
    (body, e.edge)
}

/// A brick over `x ∈ [x0, x1]`, `y ∈ [y0, y1]`, `z ∈ [−1, 1]`, and its
/// face whose outward normal is `n`.
fn brick_face(x: (f64, f64), y: (f64, f64), n: Vec3<f64>) -> (Body<f64>, FaceKey) {
    let b: Body<f64> = brick(x, y, (-1.0, 1.0), Tol::witness());
    let g = b
        .faces()
        .map(|(k, _)| k)
        .find(|&f| super::face_plane(&b, f).is_some_and(|p| p.normal.dot(n) > 0.5))
        .expect("the brick has the face");
    (b, g)
}

/// `A`'s direction of the sweep alone: the swept sheet and its contacts.
fn sweep_a(
    a: &Body<f64>,
    b: &Body<f64>,
) -> Result<(Body<f64>, crate::boolean::ContactRecords), BooleanError> {
    let (mut x, mut y) = (a.clone(), b.clone());
    let mut acc = super::ContactAcc::default();
    let band = geom_core::Band::linear(Tol::witness()).unwrap();
    super::sweep_direction(
        &mut x,
        &mut y,
        crate::boolean::Operand::A,
        &crate::boolean::DeclaredPairs::default(),
        &mut acc,
        band,
        super::SweepStrategy::Realized,
        &super::SweepKnobs::default(),
        None,
        &mut Vec::new(),
        Tol::witness(),
    )?;
    Ok((x, acc.finish()))
}

/// The arc's closed form at `t`.
fn bezier(mx: f64, h: f64, t: f64) -> (f64, f64) {
    (
        2.0 * t * (1.0 - t) * mx + 2.0 * t * t,
        4.0 * t * (1.0 - t) * h,
    )
}

/// The refusal every row expects: the arc, at a face of `b`. The arc's
/// box is the whole space (a spline edge has no sound box of its own),
/// so the first face the sweep reads it against is the one named, and
/// a carrier with no lane cannot be cleared of any face.
fn assert_refused(
    got: Result<(Body<f64>, crate::boolean::ContactRecords), BooleanError>,
    edge: crate::EdgeKey,
    b: &Body<f64>,
) {
    match got.map(|(_, contacts)| contacts) {
        Err(BooleanError::CrossingCarrierUnsupported {
            operand: crate::boolean::Operand::A,
            edge: e,
            face,
        }) => {
            assert_eq!(e, edge, "the refusal names the arc");
            assert!(
                b.get_face(face).is_some(),
                "the refusal names a face of B: {face:?}"
            );
        }
        other => panic!("the arc has no crossing lane and must refuse at the sweep: {other:?}"),
    }
}

/// **An arc that dips through a plane face and back refuses.** Its ends
/// both lie at `y = 0`, below the brick's face `y = ½`, and the arc
/// (`mx = 1`, `h = 1`) crosses that face twice, at
/// `t = (1 ± √½)/2`, inside the face. Read as a line, same-side ends are
/// no crossing, and the sweep recorded none.
#[test]
fn an_arc_dipping_through_a_plane_face_refuses() {
    let (mx, h) = (1.0, 1.0);
    let (b, _) = brick_face((0.2, 1.8), (0.5, 3.5), Vec3::new(0.0, -1.0, 0.0));
    for t in [(1.0 - 0.5f64.sqrt()) / 2.0, (1.0 + 0.5f64.sqrt()) / 2.0] {
        let (x, y) = bezier(mx, h, t);
        assert!(
            (y - 0.5).abs() < 1e-12,
            "the oracle root t = {t} is on y = ½: {y}"
        );
        assert!(
            0.2 < x && x < 1.8,
            "the oracle root t = {t} lands inside the face: x = {x}"
        );
    }
    let (a, e) = arc_sheet(mx, h);
    assert_refused(sweep_a(&a, &b), e, &b);
}

/// **An arc crossing a plane face once refuses, rather than land its
/// crossing off the face.** Its ends straddle the brick's face
/// `x = 1.5`; the arc (`mx = 0.4`, `h = 1`) crosses it at `t = 5/6`, the
/// root of `1.2t² + 0.8t − 1.5`. The endpoint interpolation puts the
/// crossing at `t = 3/4`, which the closed form places `0.225` off the
/// face's plane, and the sweep recorded a vertex there as on the face.
#[test]
fn an_arc_crossing_a_plane_face_once_refuses() {
    let (mx, h) = (0.4, 1.0);
    let (b, _) = brick_face((1.5, 4.5), (-1.0, 3.0), Vec3::new(-1.0, 0.0, 0.0));
    let (x, y) = bezier(mx, h, 5.0 / 6.0);
    assert!(
        (x - 1.5).abs() < 1e-12,
        "the oracle root is on x = 1.5: {x}"
    );
    assert!(
        -1.0 < y && y < 3.0,
        "the oracle root lands inside the face: y = {y}"
    );
    let (x_interp, _) = bezier(mx, h, 0.75);
    assert!(
        (x_interp - 1.5).abs() > 0.2,
        "the endpoint interpolation lands off the plane: x = {x_interp}"
    );
    let (a, e) = arc_sheet(mx, h);
    assert_refused(sweep_a(&a, &b), e, &b);
}

/// **The same arc against a cylinder wall refuses with the same
/// variant.** The unit cylinder about `z` meets the arc (`mx = 1`,
/// `h = 1`) where `x² + y² = 1`: the arc starts inside it at the origin
/// and peaks at `(1, 1)`, outside it, so it crosses the wall. No
/// declaration settles a carrier with no crossing lane, so the refusal
/// offers none.
#[test]
fn an_arc_crossing_a_cylinder_wall_refuses() {
    use crate::test_support_fixtures::{CylFrame, cyl_wall_sheet};
    let (mx, h) = (1.0, 1.0);
    let (x, y) = bezier(mx, h, 0.5);
    assert!(
        x * x + y * y > 1.0,
        "the oracle puts the arc's peak outside the wall"
    );
    let mut b = Body::<f64>::new();
    cyl_wall_sheet(
        &mut b,
        CylFrame::canonical(1.0),
        None,
        (0.0, core::f64::consts::PI),
        (-1.0, 1.0),
        Tol::witness(),
    );
    let (a, e) = arc_sheet(mx, h);
    assert_refused(sweep_a(&a, &b), e, &b);
}
