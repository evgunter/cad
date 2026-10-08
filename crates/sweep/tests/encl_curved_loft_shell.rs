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

/// The cap rims of the curved loft are the cap and wall's
/// `Intersection`, and an INWARD cap offset derives them: each rim is
/// the moved plane's section of its wall — an interior row, minted
/// exactly — and each corner is the moved plane's root along its
/// slanted seam, so it slides along the seam rather than along the cap
/// normal.
#[test]
fn the_curved_lofts_cap_moves_its_corners_along_the_slanted_seams() {
    let body = twisted_loft(0.3);
    let walls = nurbs_walls(&body);
    let cap = top_cap(&body);
    let rims = |b: &Body<f64>| {
        b.edges()
            .filter_map(|(_, e)| {
                b.get_curve_geom(e.curve)
                    .and_then(topo::CurveGeom::certified)
            })
            .filter(|c| {
                matches!(
                    c.description(),
                    geom_brep::EdgeDescription::Intersection { .. }
                )
            })
            .count()
    };
    assert_eq!(
        rims(&body),
        8,
        "every cap rim of the loft is an Intersection at rest"
    );
    let mut moved = body.clone();
    topo::replace_face_offset(&mut moved, cap, -THICKNESS, Tol::witness())
        .expect("the curved loft's cap moves inward");
    assert_eq!(
        rims(&moved),
        8,
        "the moved rims are still the cap's sections"
    );
    let z = 1.0 - THICKNESS;
    // Each seam ends at a moved corner on the moved plane, and on the
    // seam's own (untouched) carrier.
    let mut corners = 0;
    for (edge, data) in moved.edges() {
        let Some(curve) = moved
            .get_curve_geom(data.curve)
            .and_then(topo::CurveGeom::certified)
        else {
            continue;
        };
        let (t0, t1) = curve.params();
        let (a, b) = (curve.carrier().eval(t0), curve.carrier().eval(t1));
        if (a.z - b.z).abs() < 0.5 {
            continue;
        }
        assert_wall_seam(&moved, edge, "a re-anchored seam");
        let top = if a.z > b.z { a } else { b };
        assert!(
            (top.z - z).abs() < 1e-9,
            "the seam ends on the moved plane, at z = {}",
            top.z
        );
        corners += 1;
    }
    assert_eq!(corners, 4, "four seams meet the moved cap");
    let rows = moved
        .pcurves()
        .filter(|(he, _)| {
            moved
                .face_of_half_edge(*he)
                .is_some_and(|f| is_spline_wall(&walls, f))
        })
        .filter(|(_, c)| {
            matches!(*c.pcurve(), geom_brep::Pcurve::IsoLine { p0, pl }
                if pl.y == 0.0 && (p0.y - z).abs() < 1e-9)
        })
        .count();
    assert_eq!(
        rows, 4,
        "each wall carries the moved rim on its interior row v = {z}"
    );
}

/// With its caps derived, the curved loft's shell moves on to the walls
/// and refuses at the first one's offset fit, at the run's ε.
#[test]
fn shelling_the_curved_loft_refuses_at_a_walls_fit() {
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
    assert!(
        is_spline_wall(&nurbs_walls(&body), *face),
        "the refusing face is not a wall: {e}"
    );
    assert!(
        matches!(error.as_ref(), ReplaceFaceError::Fit { .. }),
        "expected the wall's offset fit to refuse, got {e}"
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
    assert_eq!(
        rims, 4,
        "each of the four walls carries the moved rim on an interior row"
    );
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

/// The vase's walls are rational, so its cap rims keep their declared
/// image in the cap's chart, and moving the cap tilts that declared
/// edge against a wall the move does not carry onto itself: the door
/// refuses it by name rather than re-stating the sketch's record on a
/// section it does not describe.
#[test]
fn shelling_the_vase_refuses_its_declared_cap_rim() {
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
    let ReplaceFaceError::DeclaredEdgeTilted { edge } = error.as_ref() else {
        panic!("expected the declared rim's refusal, got {e}");
    };
    let data = body.get_edge(*edge).expect("the rim resolves");
    let curve = body
        .get_curve_geom(data.curve)
        .and_then(topo::CurveGeom::certified)
        .expect("the rim carries a certified curve");
    assert!(
        curve.authority().is_declared(),
        "the refused rim is the sketch's declared one"
    );
    let walls = nurbs_walls(&body);
    assert!(
        [data.he_plus, data.he_minus]
            .into_iter()
            .filter_map(|he| body.face_of_half_edge(he))
            .any(|f| is_spline_wall(&walls, f)),
        "the refused rim bounds a spline wall"
    );
}
