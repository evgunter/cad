//! **A spline or spiric carrier at the sweep's two arms**: the plane
//! arm reads a spline edge through the curve × face table's
//! ring-composite arm, and every other pairing here has no crossing lane
//! and refuses.
//!
//! Two operands, each a planar sheet bounded by one curved edge and the
//! chord back:
//!
//! - in `z = 0`, a quadratic Bézier arc (a NURBS carrier) from
//!   `(0, 0, 0)` through the control point `(mx, 2h, 0)` to `(2, 0, 0)`;
//! - in `x = ½`, the spiric of the torus `R = 2`, `r = 1` about `ẑ` over
//!   `v ∈ [−π/2, π/2]`.
//!
//! The partner is a brick with a side face across the curve, or a
//! cylinder wall. `A`'s direction of the sweep runs directly, so the
//! rows read what the sweep's arms themselves do with it: where it split
//! the edge, or which arm refused (`EdgeCarrierSite`). Each sheet is built fresh-keyed
//! (`fresh_keyed_body`), so the face a refusal names resolves in one
//! operand only.
//!
//! The oracles are closed forms, never the kernel's evaluator: the
//! Bézier's `x(t) = 2t(1−t)·mx + 2t²`, `y(t) = 4t(1−t)·h`, and the
//! spiric's `P(v) = (½, √(ρ² − ¼), sin v)` with `ρ = 2 + cos v`. Each row
//! first shows from one that the curve crosses the face where an
//! endpoint reading of the edge says something else.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::boolean::{BooleanError, EdgeCarrierSite};
use crate::euler::{FaceSurface, MefSite, MevSite};
use crate::test_support_fixtures::{brick, fresh_keyed_body};
use crate::{Body, FaceKey};
use geom::{Curve3, Surface};
use geom_brep::{EdgeCurveSpec, EdgeDescriptionSpec, SurfacePair};
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
pub(crate) fn arc_sheet(mx: f64, h: f64) -> (Body<f64>, crate::EdgeKey) {
    let tol = Tol::witness();
    let mut body = fresh_keyed_body::<f64>();
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
                    wrap: false,
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

/// The spiric cap in `x = ½`: the arc of the torus `R = 2`, `r = 1`
/// about `ẑ` over `v ∈ [−π/2, π/2]`, described as the torus's
/// intersection with the cap's plane, and the chord back. The sheet, and
/// its arc edge.
pub(crate) fn spiric_cap() -> (Body<f64>, crate::EdgeKey) {
    use core::f64::consts::FRAC_PI_2;
    let tol = Tol::witness();
    let spiric = Curve3::Spiric {
        center: Point3::new(0.0, 0.0, 0.0),
        axis: Vec3::new(0.0, 0.0, 1.0),
        u_ref: Vec3::new(1.0, 0.0, 0.0),
        major_radius: 2.0,
        minor_radius: 1.0,
        offset: 0.5,
    };
    let (p0, p1) = (spiric.eval(-FRAC_PI_2), spiric.eval(FRAC_PI_2));
    let mut body = fresh_keyed_body::<f64>();
    let seed = body.mvfs(p0, true).unwrap();
    let plane = body
        .set_face_surface(
            seed.face,
            FaceSurface::New {
                surface: Surface::Plane {
                    origin: Point3::new(0.5, 0.0, 0.0),
                    normal: Vec3::new(1.0, 0.0, 0.0),
                    u_ref: Vec3::new(0.0, 1.0, 0.0),
                },
                sense: true,
            },
        )
        .unwrap();
    let torus = body.add_surface(Surface::Torus {
        center: Point3::new(0.0, 0.0, 0.0),
        axis: Vec3::new(0.0, 0.0, 1.0),
        major_radius: 2.0,
        minor_radius: 1.0,
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    });
    let e = body
        .mev(
            MevSite::Lone {
                r#loop: seed.r#loop,
            },
            p1,
            EdgeCurveSpec {
                description: EdgeDescriptionSpec::Intersection {
                    pair: SurfacePair::new(torus, plane),
                    selector: geom_brep::CurveSelector::Witness(spiric.eval(0.0)),
                },
                carrier: spiric,
                param_start: -FRAC_PI_2,
                param_end: FRAC_PI_2,
            },
            tol,
        )
        .unwrap();
    body.mef(
        MefSite::Chords {
            he1: e.he_minus,
            he2: e.he_plus,
        },
        EdgeCurveSpec::line_between(p1, p0),
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

/// The spiric cap's closed form at `v`: `(½, √(ρ² − ¼), sin v)`,
/// `ρ = 2 + cos v`.
fn spiric_at(v: f64) -> (f64, f64, f64) {
    let rho = 2.0 + v.cos();
    (0.5, (rho * rho - 0.25).sqrt(), v.sin())
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
) -> Result<(Body<f64>, crate::boolean::ReductionContacts), BooleanError> {
    let (mut x, mut y) = (a.clone(), b.clone());
    let mut acc = super::ContactAcc::default();
    let band = geom_core::Band::linear(Tol::witness()).unwrap();
    super::sweep_direction(
        (&mut x, &crate::sector_shape::SlitTips::of(a)),
        &mut y,
        crate::boolean::Operand::A,
        &crate::boolean::GluedPairs::default(),
        &mut acc,
        band,
        super::SweepStrategy::Realized,
        &super::SweepKnobs::default(),
        None,
        &mut Vec::new(),
        &crate::boolean::separating::OperandAxes::new(crate::boolean::boxes::axis_key),
        Tol::witness(),
    )?;
    Ok((x, acc.finish().0))
}

/// The arc's closed form at `t`.
fn bezier(mx: f64, h: f64, t: f64) -> (f64, f64) {
    (
        2.0 * t * (1.0 - t) * mx + 2.0 * t * t,
        4.0 * t * (1.0 - t) * h,
    )
}

/// The refusal a row with no lane expects: the curved edge, at a face of
/// `b`. The face named is the first one whose box the edge's box meets,
/// and a carrier with no lane cannot be cleared of any face.
fn assert_refused(
    got: Result<(Body<f64>, crate::boolean::ReductionContacts), BooleanError>,
    edge: crate::EdgeKey,
    [a, b]: [&Body<f64>; 2],
    at: EdgeCarrierSite,
) {
    match got.map(|(_, contacts)| contacts) {
        Err(BooleanError::EdgeCarrierUnsupported {
            operand: crate::boolean::Operand::A,
            edge: e,
            face,
            face_of: crate::boolean::Operand::B,
            site,
        }) => {
            assert_eq!(site, at, "the refusal names the arm that read the edge");
            assert_eq!(e, edge, "the refusal names the curved edge");
            // A's keys are fresh-keyed (`fresh_keyed_body`), so a face
            // read in the wrong body resolves in neither.
            assert!(
                b.get_face(face).is_some() && a.get_face(face).is_none(),
                "the refusal names a face of B, not of A: {face:?}"
            );
        }
        other => panic!("the curve has no crossing lane and must refuse at the sweep: {other:?}"),
    }
}

/// The points at which the sweep put a vertex of `A` on `face` of `B`.
fn on_face(
    got: Result<(Body<f64>, crate::boolean::ReductionContacts), BooleanError>,
    face: FaceKey,
) -> Vec<Point3<f64>> {
    let (x, contacts) = got.unwrap_or_else(|e| panic!("the sweep reads the arc: {e:?}"));
    contacts
        .a_on_b
        .iter()
        .filter(|c| c.record.face == face)
        .map(|c| {
            x.vertex_points()
                .find_map(|(v, p)| (v == c.record.vertex).then_some(p))
                .expect("a contact's vertex is in the swept sheet")
        })
        .collect()
}

/// Whether `got` holds a point within `1e-9` of each of `want`, and no
/// other.
fn lands_at(got: &[Point3<f64>], want: &[(f64, f64)]) -> bool {
    got.len() == want.len()
        && want.iter().all(|&(x, y)| {
            got.iter()
                .any(|p| (p.x - x).abs() < 1e-9 && (p.y - y).abs() < 1e-9 && p.z.abs() < 1e-9)
        })
}

/// **An arc that dips through a plane face and back is split at both
/// crossings.** Its ends both lie at `y = 0`, below the brick's face
/// `y = ½`, and the arc (`mx = 1`, `h = 1`) crosses that face twice, at
/// `t = (1 ± √½)/2`, inside the face. Read as a line, same-side ends are
/// no crossing; read through the ring composite, the sweep puts a vertex
/// of the arc on the face at each.
#[test]
fn an_arc_dipping_through_a_plane_face_is_split_at_both_crossings() {
    let (mx, h) = (1.0, 1.0);
    let (b, face) = brick_face((0.2, 1.8), (0.5, 3.5), Vec3::new(0.0, -1.0, 0.0));
    let mut want = Vec::new();
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
        want.push((x, y));
    }
    let (a, _) = arc_sheet(mx, h);
    let got = on_face(sweep_a(&a, &b), face);
    assert!(lands_at(&got, &want), "on the face at {want:?}: {got:?}");
}

/// **An arc crossing a plane face once is split where it crosses, not
/// where its ends interpolate.** Its ends straddle the brick's face
/// `x = 1.5`; the arc (`mx = 0.4`, `h = 1`) crosses it at `t = 5/6`, the
/// root of `1.2t² + 0.8t − 1.5`. The endpoint interpolation puts the
/// crossing at `t = 3/4`, which the closed form places `0.225` off the
/// face's plane. The sheet's chord back crosses the face too, at
/// `(1.5, 0)`.
#[test]
fn an_arc_crossing_a_plane_face_once_is_split_where_it_crosses() {
    let (mx, h) = (0.4, 1.0);
    let (b, face) = brick_face((1.5, 4.5), (-1.0, 3.0), Vec3::new(-1.0, 0.0, 0.0));
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
    let (a, _) = arc_sheet(mx, h);
    let got = on_face(sweep_a(&a, &b), face);
    let want = [(x, y), (1.5, 0.0)];
    assert!(lands_at(&got, &want), "on the face at {want:?}: {got:?}");
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
        (0.0, core::f64::consts::PI),
        (-1.0, 1.0),
        Tol::witness(),
    );
    let (a, e) = arc_sheet(mx, h);
    assert_refused(
        sweep_a(&a, &b),
        e,
        [&a, &b],
        EdgeCarrierSite::CurvedCrossing,
    );
}

/// **A spiric that dips through a plane face and back refuses.** Its ends
/// (`v = ±π/2`) lie at `y = √3.75 ≈ 1.94`, below the brick's face
/// `y = 2.5`, and its middle (`v = 0`) at `y = √8.75 ≈ 2.96`, above it;
/// the closed form crosses `y = 2.5` where `ρ = √6.5`, at
/// `z = ±√(1 − (√6.5 − 2)²) ≈ ±0.82`, inside the face.
#[test]
fn a_spiric_dipping_through_a_plane_face_refuses() {
    let (_, y_end, _) = spiric_at(core::f64::consts::FRAC_PI_2);
    let (_, y_mid, _) = spiric_at(0.0);
    assert!(
        y_end < 2.5 && y_mid > 2.5,
        "the oracle dips through y = 2.5: ends {y_end}, middle {y_mid}"
    );
    let z = (1.0 - (6.5f64.sqrt() - 2.0).powi(2)).sqrt();
    assert!(
        z < 1.2,
        "the oracle's crossings land inside the face: z = ±{z}"
    );
    let (b, _) = brick_face((0.0, 1.0), (2.5, 4.0), Vec3::new(0.0, -1.0, 0.0));
    let (a, e) = spiric_cap();
    assert_refused(
        sweep_a(&a, &b),
        e,
        [&a, &b],
        EdgeCarrierSite::PlanarCrossing,
    );
}

/// **A spiric crossing a cylinder wall refuses with the same variant**,
/// the curved arm's spiric half. The spiric's distance from the axis,
/// `|(x, y)| = ρ`, runs `2 → 3 → 2`, so it crosses the wall `ρ = 2.5`
/// twice, at `v = ±π/3`: `(½, √6, ±√3/2)`, inside the half-wall
/// `y > 0`, `|z| ≤ 2`.
#[test]
fn a_spiric_crossing_a_cylinder_wall_refuses() {
    use crate::test_support_fixtures::{CylFrame, cyl_wall_sheet};
    let (x, y, z) = spiric_at(core::f64::consts::FRAC_PI_3);
    assert!(
        ((x * x + y * y).sqrt() - 2.5).abs() < 1e-12 && y > 0.0 && z.abs() < 2.0,
        "the oracle crosses the wall inside the sheet: ({x}, {y}, {z})"
    );
    let mut b = Body::<f64>::new();
    cyl_wall_sheet(
        &mut b,
        CylFrame::canonical(2.5),
        (0.0, core::f64::consts::PI),
        (-2.0, 2.0),
        Tol::witness(),
    );
    let (a, e) = spiric_cap();
    assert_refused(
        sweep_a(&a, &b),
        e,
        [&a, &b],
        EdgeCarrierSite::CurvedCrossing,
    );
}
