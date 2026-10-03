//! **A curved carrier with no crossing lane, at the sweep's planar arm.**
//!
//! The operand is a planar sheet in `z = 0` bounded by a quadratic
//! Bézier arc `P(t) = (2t, 4t(1−t)·h, 0)` from `(0, 0, 0)` to
//! `(2, 0, 0)` (a NURBS carrier, peak height `h` at `t = ½`) and the
//! chord back along `y = 0`. The partner is a brick whose side face
//! `y = c` stands across the arc. `sweep_and_settle` runs directly, past
//! the operand gate that refuses the carrier first in the pipeline, so
//! the rows read what the planar arm itself does with it.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use super::coplanar_conic_rows::sweep;
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
fn sweep_a(a: &Body<f64>, b: &Body<f64>) -> Result<(Body<f64>, crate::boolean::ContactRecords), BooleanError> {
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

#[test]
fn measure() {
    for (name, mx, b) in [
        ("dip y=0.5", 1.0, brick_face((0.2, 1.8), (0.5, 3.5), Vec3::new(0.0, -1.0, 0.0))),
        ("cross x=1.5", 0.4, brick_face((1.5, 4.5), (-1.0, 3.0), Vec3::new(-1.0, 0.0, 0.0))),
    ] {
        let (a, e) = arc_sheet(mx, 1.0);
        let (b, g) = b;
        let plane = super::face_plane(&b, g).unwrap();
        match sweep_a(&a, &b) {
            Ok((x, c)) => {
                eprintln!("{name}: edge {e:?} face {g:?}: Ok; a_on_b {:?}", c.a_on_b);
                for vf in c.a_on_b.iter() {
                    let p = *x.get_point(x.get_vertex(vf.vertex).unwrap().point).unwrap();
                    eprintln!("   {vf:?} at {p:?}, off its face's plane by {:e}", (p - plane.origin).dot(plane.normal));
                }
                eprintln!("   vv {:?}", c.vv);
            }
            Err(err) => eprintln!("{name}: Err {err:?}"),
        }
        eprintln!("{name}: both directions: {:?}", sweep(&a, &b).map(|_| ()));
    }
}
