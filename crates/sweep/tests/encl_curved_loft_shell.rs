//! **Bodies with NURBS walls, against the shell verb and the offset
//! door it runs per face.**
//!
//! The twisted loft (`common::approx::twisted_loft`) is lofted through
//! `sweep::loft_body` between a square and the same square turned
//! 0.3 rad: two planar caps and four bilinear SADDLE walls, so every
//! wall's offset is genuinely not a NURBS and has to be fitted. It is
//! the natural operand for the question "what does a shell of a
//! spline-walled body cost at the run's ε", and these rows pin why that
//! cost cannot be taken yet, at the thickness a user would ask for:
//! `topo::shell` refuses before any wall is fitted.
//!
//! A cap's offset moves the cap's corners, so each seam between two
//! walls that ends at a moved corner is re-anchored on its lofted
//! spline carrier. The twist slants those seams, and the per-chart
//! door moves the cap rigidly along its normal, so the moved corner
//! leaves a slanted seam by the thickness times the slant's sine: the
//! oblique-junction refusal, met here on a spline wall. The straight
//! prism's seams are parallel to the cap normal, so its re-anchor
//! holds and the moved rim lands on an interior row of each wall.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Dual64, Tol, Vec3};
use topo::{Body, EdgeKey, FaceKey, ReplaceFaceError, ShellError};

use crate::common::approx::{nurbs_walls, prism, twisted_loft};
use sweep::test_support::finished;

/// The wall thickness these rows shell at, in metres: 2.5% of the
/// 2 m section, a thickness a user would ask for.
const THICKNESS: f64 = 0.05;

fn is_spline_wall(walls: &[(FaceKey, impl Sized)], face: FaceKey) -> bool {
    walls.iter().any(|(k, _)| *k == face)
}

fn is_cap<T: geom_core::Real>(body: &Body<T>, face: FaceKey) -> bool {
    matches!(
        body.get_face(face)
            .and_then(|f| body.get_surface(f.surface)),
        Some(geom::Surface::Plane { .. })
    )
}

/// The `z = 1` cap, whose chart normal points out of the body.
fn top_cap<T: geom_core::Real + geom_core::Bounds>(body: &Body<T>) -> FaceKey {
    body.faces()
        .find(|(_, f)| {
            matches!(
                body.get_surface(f.surface),
                Some(geom::Surface::Plane { origin, normal, .. })
                    if normal.z.lo() > 0.5 && origin.z.lo() > 0.5
            )
        })
        .map(|(k, _)| k)
        .expect("the loft has a top cap")
}

/// `edge` is a seam between two spline walls.
fn assert_wall_seam(body: &Body<f64>, edge: EdgeKey, context: &str) {
    let walls = nurbs_walls(body);
    let halves = body.get_edge(edge).expect("the refused edge resolves");
    let sides: Vec<FaceKey> = [halves.he_plus, halves.he_minus]
        .into_iter()
        .filter_map(|he| body.face_of_half_edge(he))
        .collect();
    assert!(
        sides.len() == 2 && sides.iter().all(|k| is_spline_wall(&walls, *k)),
        "{context}: {edge:?} is not a seam between two spline walls"
    );
}

/// The chord of `edge`'s carrier between its two ends.
fn seam_chord(body: &Body<f64>, edge: EdgeKey) -> Vec3<f64> {
    let data = body.get_edge(edge).expect("the seam resolves");
    let curve = body
        .get_curve_geom(data.curve)
        .and_then(topo::CurveGeom::certified)
        .expect("the seam carries a certified curve");
    let (t0, t1) = curve.params();
    curve.carrier().eval(t1) - curve.carrier().eval(t0)
}

/// The shell of the curved loft refuses at a CAP, re-anchoring a
/// wall-to-wall seam, before the fit of any wall runs: the seam is
/// re-anchored on its spline carrier, and the cap's corner, moved
/// along the cap normal, is off the slanted seam by exactly the
/// thickness times the sine of the seam's slant.
#[test]
fn shelling_the_curved_loft_refuses_at_the_oblique_cap_corner_before_any_fit() {
    let body = twisted_loft(0.3);
    let e = topo::shell(
        &finished("the operand", body.clone(), Tol::witness()),
        THICKNESS,
        Tol::witness(),
    )
    .expect_err("a spline-walled body does not shell today");
    let ShellError::Face { face, error } = &e else {
        panic!("expected a per-face offset refusal, got {e}");
    };
    assert!(is_cap(&body, *face), "the refusing face is not a cap: {e}");
    let ReplaceFaceError::ReanchorOffCarrier { edge, gap } = error.as_ref() else {
        panic!("expected the oblique corner's re-anchor refusal, got {e}");
    };
    assert_wall_seam(&body, *edge, "the oblique corner");
    // The bilinear walls' seams are straight, so the moved corner's
    // distance from the seam is the cap's displacement across it.
    let chord = seam_chord(&body, *edge);
    let sine = chord.cross(Vec3::unit_z()).norm() / chord.norm();
    assert!(
        sine > 0.3,
        "the twist slants the seam (sine {sine}); a straight seam would re-anchor"
    );
    assert!(
        (gap - THICKNESS * sine).abs() < 1e-12,
        "the gap {gap} is the cap's displacement across the seam, {}",
        THICKNESS * sine
    );
}

/// An OUTWARD cap offset runs each seam's corner past the end of the
/// wall patch that carries it, by the offset itself: the foot of the
/// moved corner is the carrier's domain end, and the door names that
/// rather than a point off the carrier.
#[test]
fn an_outward_cap_offset_runs_past_the_seam_patchs_end() {
    for (name, body) in [("prism", prism()), ("twisted", twisted_loft(0.3))] {
        let mut moved = body.clone();
        let e = topo::replace_face_offset(&mut moved, top_cap(&body), THICKNESS, Tol::witness())
            .expect_err("the walls end at the cap and do not extend");
        let ReplaceFaceError::ReanchorPastCarrierEnd { edge, gap } = e else {
            panic!("{name}: expected the past-the-end refusal, got {e}");
        };
        assert_wall_seam(&body, edge, name);
        assert!(
            (gap - THICKNESS).abs() < 1e-12,
            "{name}: the corner is the offset past the seam's end, got {gap}"
        );
    }
}

/// The straight prism's seams are parallel to the cap normal, so an
/// INWARD cap offset re-anchors every one of them, and the moved cap's
/// rim runs along an interior row of each wall's chart, where the
/// pcurve mint places it exactly: an iso line at the moved height.
#[test]
fn the_prisms_inward_cap_offset_mints_its_rim_on_the_walls_interior_row() {
    let body = prism();
    let walls = nurbs_walls(&body);
    let mut moved = body.clone();
    let cap = top_cap(&body);
    topo::replace_face_offset(&mut moved, cap, -THICKNESS, Tol::witness())
        .expect("the prism's cap moves inward");
    let mut rims = 0;
    for (he, cache) in moved.pcurves() {
        let Some(face) = moved.face_of_half_edge(he) else {
            continue;
        };
        if !is_spline_wall(&walls, face) {
            continue;
        }
        let geom_brep::Pcurve::IsoLine { p0, pl } = *cache.pcurve() else {
            continue;
        };
        // A row: `u` moves and `v` is fixed, strictly inside the chart.
        if pl.y == 0.0 && p0.y > 0.0 && p0.y < 1.0 {
            assert!(
                (p0.y - (1.0 - THICKNESS)).abs() < 1e-12,
                "the rim's row is the moved cap's height, got v = {}",
                p0.y
            );
            rims += 1;
        }
    }
    assert_eq!(rims, 4, "each of the four walls carries the moved rim on an interior row");
}

/// A scalar that holds no NURBS lane cannot read a spline seam's foot,
/// and says so by name rather than re-anchoring on a guess.
#[test]
fn a_dual_scalar_refuses_the_spline_seam_re_anchor_by_name() {
    let v = |x: f64, y: f64| (geom_core::Point2::new(x, y), 0.0);
    let square = || {
        vec![profile::test_support::bulge_loop(vec![
            v(0.0, 0.0),
            v(2.0, 0.0),
            v(2.0, 2.0),
            v(0.0, 2.0),
        ])]
    };
    let places = [0.0, 1.0]
        .iter()
        .map(|z| geom_core::Affine3::translation(Vec3::new(0.0, 0.0, *z)))
        .collect::<Vec<_>>();
    let body = sweep::loft_body::<Dual64>(&[square(), square()], &places, 1, Tol::witness())
        .expect("the square prism lofts at a dual")
        .body;
    let mut moved = body.clone();
    let cap = top_cap(&body);
    let Err(ReplaceFaceError::NurbsLaneUnsupported { scalar, .. }) = topo::replace_face_offset(
        &mut moved,
        cap,
        <Dual64 as geom_core::Real>::from_f64(-THICKNESS),
        Tol::witness(),
    ) else {
        panic!("expected the lane refusal at a dual");
    };
    assert_eq!(scalar, <Dual64 as geom_core::Real>::NAME);
}

/// The circular vase: three circle sections of radius 1, 1.3 and 1 at
/// heights 0, 1 and 2, lofted at degree 2 — two spline walls whose
/// seams leave each cap at a slant.
fn vase() -> Body<f64> {
    let circle = |r: f64| {
        vec![profile::test_support::bulge_loop(vec![
            (geom_core::Point2::new(r, 0.0), 1.0),
            (geom_core::Point2::new(-r, 0.0), 1.0),
        ])]
    };
    let places = [0.0, 1.0, 2.0]
        .iter()
        .map(|z| geom_core::Affine3::translation(Vec3::new(0.0, 0.0, *z)))
        .collect::<Vec<_>>();
    sweep::loft_body::<f64>(
        &[circle(1.0), circle(1.3), circle(1.0)],
        &places,
        2,
        Tol::witness(),
    )
    .expect("the vase lofts")
    .body
}

/// The vase's seams are curved, so its corner's gap is the twisted
/// loft's to first order: the thickness times the sine of the seam's
/// slant where it leaves the refusing cap.
#[test]
fn shelling_the_vase_refuses_at_its_oblique_cap_corner() {
    let body = vase();
    let e = topo::shell(
        &finished("the vase", body.clone(), Tol::witness()),
        THICKNESS,
        Tol::witness(),
    )
    .expect_err("a spline-walled body does not shell today");
    let ShellError::Face { face, error } = &e else {
        panic!("expected a per-face offset refusal, got {e}");
    };
    assert!(is_cap(&body, *face), "the refusing face is not a cap: {e}");
    let ReplaceFaceError::ReanchorOffCarrier { edge, gap } = error.as_ref() else {
        panic!("expected the oblique corner's re-anchor refusal, got {e}");
    };
    assert_wall_seam(&body, *edge, "the vase's corner");
    let data = body.get_edge(*edge).expect("the seam resolves");
    let curve = body
        .get_curve_geom(data.curve)
        .and_then(topo::CurveGeom::certified)
        .expect("the seam carries a certified curve");
    let (t0, t1) = curve.params();
    let cap_z = match body
        .get_face(*face)
        .and_then(|f| body.get_surface(f.surface))
    {
        Some(geom::Surface::Plane { origin, .. }) => origin.z,
        _ => unreachable!("the refusing face is a cap"),
    };
    let at_cap = if (curve.carrier().eval(t0).z - cap_z).abs() < 1e-9 {
        t0
    } else {
        t1
    };
    let tangent = curve.carrier().deriv(at_cap);
    let sine = tangent.cross(Vec3::unit_z()).norm() / tangent.norm();
    let first_order = THICKNESS * sine;
    assert!(
        (gap - first_order).abs() < 0.05 * first_order,
        "the gap {gap} is the cap's displacement across the seam to first order, {first_order}"
    );
}
