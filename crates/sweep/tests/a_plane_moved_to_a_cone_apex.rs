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
