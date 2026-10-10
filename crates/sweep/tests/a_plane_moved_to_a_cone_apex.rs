//! **A plane face moved to within the band of a cone's apex still cuts
//! the cone.** A frustum cut across by a plane leaning nearly along a
//! generator has an elliptical rim shared by its cone wall and the
//! plane face. Offsetting that face toward the apex moves the plane
//! parallel to itself, and parallel planes cut the cone in ellipses
//! homothetic about the apex: the moved plane, its apex gap Zero, still
//! cuts an ellipse whose farthest point is `|δ|/|D|` from the apex (`δ`
//! the gap, `D` the plane-through-apex discriminant), past the band
//! when `D` is small. That ellipse is the rim's section; the apex point
//! is not, and the door must not read it as a plane that no longer
//! meets its neighbour.
//!
//! A plane through the apex holding two generators, moved into the band
//! of it, cuts a hyperbola instead, whose branches start `|δ|/D` from
//! the apex. On a needle that is past the band, and the door must not
//! keep the generator through the apex as the section there.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::revolve_common::{axis_y, validated};
use geom::Surface;
use geom_core::{Point2, Point3, Tol, Vec3};
use profile::{ProfileLoop, RawLoop};
use sweep::{Revolution, revolve};
use topo::{Body, FaceKey};

/// The cone's half-angle.
const ALPHA: f64 = core::f64::consts::FRAC_PI_3;
/// The cut plane's discriminant `sin α·‖a×n‖ − cos α·|a·n|`: negative
/// (the plane through the apex clears the cone) and small, so the
/// ellipse is long against the plane's gap.
const DISCR: f64 = -1.0 / 256.0;
/// The cut plane's gap from the apex.
const GAP: f64 = 1.0 / 512.0;

/// A frustum of the cone with apex at the origin and axis `+y`, from
/// `y = 1/2048` to `y = 3/8`, cut by the plane `GAP` above the apex
/// whose normal leans off the axis toward `+x` until its discriminant
/// is `DISCR`; the part away from the apex. Its rim on the cone wall is
/// the ellipse the plane cuts (reach `GAP/|DISCR|` along a generator,
/// inside the frustum's slant window).
fn cut_frustum() -> Body<f64> {
    let (y0, y1, t) = (1.0 / 2048.0, 3.0 / 8.0, ALPHA.tan());
    let meridian = ProfileLoop::polygon([
        Point2::new(0.0, y0),
        Point2::new(y0 * t, y0),
        Point2::new(y1 * t, y1),
        Point2::new(0.0, y1),
    ]);
    let frustum = revolve(
        &validated(vec![meridian]),
        axis_y(),
        Revolution::Full,
        Tol::witness(),
    )
    .expect("the meridian revolves")
    .body;
    let frustum = topo::test_support::finished("the frustum", frustum, Tol::witness());
    let lean = (-DISCR).acos() - ALPHA;
    let n = Vec3::new(lean.sin(), lean.cos(), 0.0);
    let plane = topo::test_support::split_plane(Point3::origin() + n * GAP, n, Tol::witness());
    let split = topo::splitting::split(&frustum, &plane, Tol::witness()).expect("the cut splits");
    let topo::splitting::SplitPart::Body(above) = split.above else {
        panic!("the part away from the apex carries material");
    };
    above
}

fn surface_of(body: &Body<f64>, face: FaceKey) -> Surface<f64> {
    body.get_surface(body.get_face(face).unwrap().surface)
        .unwrap()
        .clone()
}

/// **Every rim arc of the moved face plans its section.** The cut face
/// is offset toward the apex by its gap less `0.9·ε`, so the moved
/// plane's apex gap is in the zero band while the ellipse it cuts
/// reaches `0.9·ε/|D|` (`≈ 230·ε`) from the apex. Each rim arc between
/// the face and the cone wall is planned (`Ok(None)`): a deferred
/// `NoBranch` refusal is the apex point read as the section.
#[test]
fn a_plane_moved_to_the_apex_band_plans_the_ellipse_it_cuts() {
    let tol = Tol::witness();
    let (eps, k) = (tol.eps(), tol.k());
    let body = cut_frustum();
    let face = body
        .faces()
        .map(|(f, _)| f)
        .find(|&f| {
            matches!(surface_of(&body, f), Surface::Plane { normal, .. } if normal.x.abs() > 0.1)
        })
        .expect("the cut face");
    let Surface::Plane { origin, normal, .. } = surface_of(&body, face) else {
        unreachable!("the cut face is a plane")
    };
    let Some(Surface::Cone {
        apex,
        axis,
        half_angle,
        ..
    }) = body
        .faces()
        .map(|(f, _)| surface_of(&body, f))
        .find(|s| matches!(s, Surface::Cone { .. }))
    else {
        panic!("the frustum's wall is a cone");
    };

    // The face's outward normal faces the apex, so a positive offset
    // closes the gap.
    let gap = (apex - origin).dot(normal);
    assert!(
        (gap - GAP).abs() < 1e-12,
        "the cut face stands {gap} off the apex"
    );
    let d = gap - 0.9 * eps;
    let moved_gap = (apex - (origin + normal * d)).dot(normal);
    let discr =
        half_angle.sin() * axis.cross(normal).norm() - half_angle.cos() * axis.dot(normal).abs();
    assert!(
        moved_gap.abs() <= eps,
        "the moved plane's apex gap {moved_gap:e} is in the zero band"
    );
    assert!(
        moved_gap.abs() / discr.abs() > k * eps,
        "the moved plane's ellipse reaches {:e}, past the band",
        moved_gap.abs() / discr.abs()
    );

    let plans = topo::offset_edge_plans_for_tests(&body, face, d, tol);
    assert!(!plans.is_empty(), "the cut face has rim edges");
    for (edge, plan) in plans {
        let carrier = body
            .get_curve_geom(body.get_edge(edge).unwrap().curve)
            .and_then(topo::CurveGeom::certified)
            .unwrap()
            .carrier()
            .clone();
        assert!(
            matches!(carrier, geom::Curve3::Ellipse { .. }),
            "{edge:?}: the rim is the cut's ellipse, got {carrier:?}"
        );
        assert!(
            matches!(plan, Ok(None)),
            "{edge:?}: the moved plane's ellipse is the rim's section, got {plan:?}"
        );
    }
}

/// A quarter wedge of the cone of half-angle `atan(1/64)` with its apex
/// at the origin and axis `+y`, from the apex to `y = 3/8`: its two caps
/// are planes containing the axis, each meeting the wall along a
/// generator from the apex vertex.
fn needle_wedge() -> Body<f64> {
    let t = 1.0 / 64.0;
    let meridian = ProfileLoop::polygon([
        Point2::new(0.0, 0.0),
        Point2::new(0.375 * t, 0.375),
        Point2::new(0.0, 0.375),
    ]);
    revolve(
        &validated(vec![meridian]),
        axis_y(),
        Revolution::Partial(core::f64::consts::FRAC_PI_2),
        Tol::witness(),
    )
    .expect("the meridian revolves")
    .body
}

/// **A cap moved off a needle's apex does not keep the generator its
/// plane no longer holds.** A cap of [`needle_wedge`] moved by `±0.9·ε`
/// stands in the zero band of the apex, and its plane cuts the wall in
/// a hyperbola whose branches start `0.9·ε/D` (`D = sin α ≈ 1/64`, so
/// `≈ 58·ε`) from the apex, past the band. The cap's wall edge runs from
/// the apex vertex, so the line pair through the apex is not its
/// section there: the edge refuses typed, naming the hyperbola (R1),
/// where the door had planned the old generator. The cap's other edges,
/// against planes, plan.
#[test]
fn a_cap_moved_off_a_needle_apex_refuses_the_hyperbola() {
    let tol = Tol::witness();
    let (eps, k) = (tol.eps(), tol.k());
    let body = needle_wedge();
    let Some(Surface::Cone {
        apex,
        axis,
        half_angle,
        ..
    }) = body
        .faces()
        .map(|(f, _)| surface_of(&body, f))
        .find(|s| matches!(s, Surface::Cone { .. }))
    else {
        panic!("the wedge's wall is a cone");
    };
    let caps: Vec<FaceKey> = body
        .faces()
        .map(|(f, _)| f)
        .filter(|&f| {
            matches!(surface_of(&body, f), Surface::Plane { normal, .. } if normal.y.abs() < 0.5)
        })
        .collect();
    assert_eq!(caps.len(), 2, "the wedge has two caps");
    for face in caps {
        let Surface::Plane { origin, normal, .. } = surface_of(&body, face) else {
            unreachable!("a cap is a plane")
        };
        for d in [0.9 * eps, -0.9 * eps] {
            let moved_gap = (apex - (origin + normal * d)).dot(normal);
            let discr = half_angle.sin() * axis.cross(normal).norm()
                - half_angle.cos() * axis.dot(normal).abs();
            assert!(
                moved_gap.abs() <= eps && moved_gap.abs() / discr > k * eps,
                "{face:?} by {d:e}: the moved gap {moved_gap:e} is in the zero band and the \
                 hyperbola starts {:e} off the apex, past it",
                moved_gap.abs() / discr
            );
            let mut walls = 0;
            for (edge, plan) in topo::offset_edge_plans_for_tests(&body, face, d, tol) {
                match plan {
                    Err(topo::ReplaceFaceError::NeighborPoseUnroutable {
                        other_kind: geom::SurfaceKind::Cone,
                        why,
                        ..
                    }) => {
                        assert!(
                            why.contains("HYPERBOLA"),
                            "{face:?} by {d:e}, {edge:?}: refuses naming the hyperbola, got {why}"
                        );
                        walls += 1;
                    }
                    Ok(None) => {}
                    other => panic!("{face:?} by {d:e}, {edge:?}: got {other:?}"),
                }
            }
            assert_eq!(
                walls, 1,
                "{face:?} by {d:e}: the cap's one wall edge refuses"
            );
        }
    }
}
