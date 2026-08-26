//! **`replace_face_offset` at the body**: one face's surface becomes
//! its certified offset, the face's own boundary is re-described
//! against the moved chart, and the result rests tier-3 valid.
//!
//! The analytic fixture is a revolved rectangular annulus — a tube.
//! Four faces (two coaxial cylinder walls, two annular caps), eight
//! edges, four vertices, and every description class the door has a
//! lane for: a `Seam` on each wall, an `Intersection` on each rim, a
//! `MappedCurve` on each cap's radial seam. Offsetting the outer wall
//! exercises all three transport lanes AND the re-anchoring of the two
//! cap seams whose far end moved with it; offsetting a cap exercises
//! the plane's translation and the two wall seams' re-anchoring.
//!
//! The `Approx` rows sit on the loft prism the OFF-C consumer built,
//! now driven THROUGH the door rather than through the attach layer by
//! hand — the fit is minted from the wall itself rather than from a
//! pulled-back base, so the face genuinely moves and every carrier is
//! re-derived.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::{Curve3, Surface};
use geom_core::{Band, Point2, Tol, Vec2};
use profile::{Profile, ProfileLoop, ProfileVertex, RawLoop, SketchPlane};
use sweep::{Revolution, RevolveAxis, revolve};
use topo::{Body, CurveGeom, FaceKey, ReplaceFaceError};

mod common;
use common::approx::{FIT_DEGREE, prism};

fn p2(x: f64, y: f64) -> Point2<f64> {
    Point2::new(x, y)
}

fn band() -> Band {
    Band::linear(Tol::witness()).unwrap()
}

/// The fit tolerance the `Approx` rows mint at — the OFF-C consumer's.
const FIT_TOL: f64 = 1e-6;

/// Revolves the closed `(r, y)` polygon a full turn about the `y` axis.
fn revolved(points: &[(f64, f64)]) -> Body<f64> {
    let lp = ProfileLoop::new(
        points
            .iter()
            .map(|(r, y)| ProfileVertex::new(p2(*r, *y), 0.0))
            .collect(),
    );
    let profile = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(Tol::witness())
        .expect("the fixture polygon is a valid profile");
    revolve(
        &profile,
        RevolveAxis {
            origin: p2(0.0, 0.0),
            dir: Vec2::new(0.0, 1.0),
        },
        Revolution::Full,
        Tol::witness(),
    )
    .expect("the fixture polygon revolves")
    .body
}

/// The tube: outer wall `r = 0.8`, inner wall `r = 0.4`, annular caps
/// at `y = 0` and `y = 0.6`.
fn tube() -> Body<f64> {
    revolved(&[(0.4, 0.0), (0.8, 0.0), (0.8, 0.6), (0.4, 0.6)])
}

/// A tube whose outer wall is a cylinder BELOW and a cone ABOVE — the
/// pair `(cone, cylinder)` has no route arm, which is what the C5 row
/// needs, and the cone's own `v`-window is what the apex row decides
/// over.
fn coned_tube() -> Body<f64> {
    revolved(&[(0.4, 0.0), (0.8, 0.0), (0.8, 0.3), (0.4, 0.6)])
}

/// The face whose cylinder carries `radius`.
fn cylinder_face(body: &Body<f64>, radius: f64) -> FaceKey {
    body.faces()
        .find(|(_, f)| {
            matches!(body.get_surface(f.surface), Some(Surface::Cylinder { radius: r, .. }) if (*r - radius).abs() < 1e-9)
        })
        .map(|(k, _)| k)
        .unwrap_or_else(|| panic!("no cylinder face at r = {radius}"))
}

/// The planar face whose origin sits at height `y`.
fn plane_face(body: &Body<f64>, y: f64) -> FaceKey {
    body.faces()
        .find(|(_, f)| {
            matches!(body.get_surface(f.surface), Some(Surface::Plane { origin, .. }) if (origin.y - y).abs() < 1e-9)
        })
        .map(|(k, _)| k)
        .unwrap_or_else(|| panic!("no planar face at y = {y}"))
}

/// The body's one cone face.
fn cone_face(body: &Body<f64>) -> FaceKey {
    body.faces()
        .find(|(_, f)| matches!(body.get_surface(f.surface), Some(Surface::Cone { .. })))
        .map(|(k, _)| k)
        .expect("the fixture has a cone face")
}

/// Every non-placeholder spline wall of `body`.
fn spline_walls(body: &Body<f64>) -> Vec<FaceKey> {
    body.faces()
        .filter(|(_, f)| {
            matches!(body.get_surface(f.surface), Some(Surface::Nurbs(n)) if !n.is_placeholder())
        })
        .map(|(k, _)| k)
        .collect()
}

fn radius_of(body: &Body<f64>, face: FaceKey) -> f64 {
    match body.get_surface(body.get_face(face).unwrap().surface) {
        Some(Surface::Cylinder { radius, .. }) => *radius,
        other => panic!("expected a cylinder, got {other:?}"),
    }
}

/// The radii of every `Circle` carrier in the body, sorted — the rims'
/// own record of where the walls now are.
fn circle_radii(body: &Body<f64>) -> Vec<f64> {
    let mut out: Vec<f64> = body
        .edges()
        .filter_map(|(_, e)| {
            match body
                .get_curve_geom(e.curve)
                .and_then(CurveGeom::certified)?
                .carrier()
            {
                Curve3::Circle { radius, .. } => Some(*radius),
                _ => None,
            }
        })
        .collect();
    out.sort_by(f64::total_cmp);
    out
}

// ---------------------------------------------------------------------
// The analytic rows
// ---------------------------------------------------------------------

/// **The headline row, both signs.** The outer wall's surface becomes
/// its offset, its `Seam` and both `Intersection` rims are
/// re-described against the moved cylinder, the two cap seams whose far
/// end moved with it are re-anchored, and the body rests tier-3 valid.
#[test]
fn the_cylinder_wall_offsets_at_both_signs() {
    for d in [0.05_f64, -0.05] {
        let mut body = tube();
        let face = cylinder_face(&body, 0.8);
        topo::replace_face_offset(&mut body, face, d, FIT_TOL, band(), Tol::witness())
            .unwrap_or_else(|e| panic!("d = {d}: the outer wall's offset must land: {e}"));

        assert!(
            (radius_of(&body, face) - (0.8 + d)).abs() < 1e-15,
            "d = {d}: the wall's radius is the offset one"
        );
        assert_eq!(
            circle_radii(&body),
            vec![0.4, 0.4, 0.8 + d, 0.8 + d],
            "d = {d}: both outer rims moved with the wall and neither inner rim did"
        );
        assert_eq!(
            topo::validate_geometric(&body, Tol::witness()),
            Ok(()),
            "d = {d}: tier 3 on the re-described tube"
        );
    }
}

/// The cap seams' PARAMETER ranges follow the moved wall: a radial
/// segment from `r = 0.4` to `r = 0.8` now ends at `r = 0.8 + d`, and
/// its authoritative sketch datum says so — the door re-states the
/// segment rather than patching the carrier around it.
#[test]
fn the_untouched_cap_seams_are_re_anchored() {
    let d = 0.05;
    let mut body = tube();
    let face = cylinder_face(&body, 0.8);
    topo::replace_face_offset(&mut body, face, d, FIT_TOL, band(), Tol::witness()).unwrap();

    let mut spans: Vec<f64> = body
        .edges()
        .filter_map(|(_, e)| {
            let c = body
                .get_curve_geom(e.curve)
                .and_then(CurveGeom::certified)?;
            matches!(c.description(), geom_brep::EdgeGeometry::MappedCurve(_)).then(|| {
                let (t0, t1) = c.params();
                (t1 - t0).abs()
            })
        })
        .collect();
    spans.sort_by(f64::total_cmp);
    assert!(
        spans.len() == 2 && spans.iter().all(|s| (s - (0.4 + d)).abs() < 1e-15),
        "both cap seams span the wider annulus, got {spans:?}"
    );

    let far: Vec<f64> = body
        .edges()
        .filter_map(|(_, e)| {
            let c = body
                .get_curve_geom(e.curve)
                .and_then(CurveGeom::certified)?;
            let geom_brep::EdgeGeometry::MappedCurve(geom_brep::MappedCurve::PlacedSegment {
                segment: geom_brep::SketchSegment::Line { a, b },
                ..
            }) = c.description()
            else {
                return None;
            };
            Some(a.x.max(b.x))
        })
        .collect();
    assert!(
        far.iter().all(|x| (x - (0.8 + d)).abs() < 1e-15),
        "the sketch segments' far endpoints followed the wall: {far:?}"
    );
}

/// **The planar row.** A cap's plane translates by `d·n`, its two rims
/// translate with it, its `MappedCurve` seam's PLACEMENT absorbs the
/// same translation, and the two wall seams are re-anchored at their
/// moved end.
#[test]
fn a_planar_cap_offsets_at_both_signs() {
    for d in [0.05_f64, -0.05] {
        let mut body = tube();
        let face = plane_face(&body, 0.6);
        topo::replace_face_offset(&mut body, face, d, FIT_TOL, band(), Tol::witness())
            .unwrap_or_else(|e| panic!("d = {d}: the cap's offset must land: {e}"));

        let Some(Surface::Plane { origin, .. }) =
            body.get_surface(body.get_face(face).unwrap().surface)
        else {
            panic!("the cap is still a plane")
        };
        assert!(
            (origin.y - (0.6 + d)).abs() < 1e-15,
            "d = {d}: the plane moved along its own normal, got {origin:?}"
        );
        assert_eq!(
            topo::validate_geometric(&body, Tol::witness()),
            Ok(()),
            "d = {d}: tier 3 on the re-described tube"
        );
    }
}

/// A whole tube shelled one face at a time: every face replaced by its
/// inward offset, ONE body, tier-3 valid at the end. This is the
/// composition PR-2 rides on, exercised here at the primitive.
#[test]
fn every_face_of_a_tube_offsets_in_turn() {
    let mut body = tube();
    let faces: Vec<FaceKey> = body.faces().map(|(k, _)| k).collect();
    for face in faces {
        let sense = body.get_face(face).unwrap().sense;
        // Inward is against the chart normal on a positively-sensed
        // face and with it on a reversed one.
        let d = if sense { -0.02 } else { 0.02 };
        topo::replace_face_offset(&mut body, face, d, FIT_TOL, band(), Tol::witness())
            .unwrap_or_else(|e| panic!("{face:?} at d = {d}: {e}"));
    }
    assert_eq!(
        topo::validate_geometric(&body, Tol::witness()),
        Ok(()),
        "tier 3 on the tube with every face replaced"
    );
}

// ---------------------------------------------------------------------
// The planted reds
// ---------------------------------------------------------------------

/// **Collapse.** The inner wall's inward offset reaches its own axis:
/// the offset door's realized-radius floor refuses, and the door
/// carries that refusal verbatim.
#[test]
fn the_radius_floor_refuses_typed() {
    let mut body = tube();
    let face = cylinder_face(&body, 0.4);
    let before = body.clone();
    let e = topo::replace_face_offset(&mut body, face, -0.5, FIT_TOL, band(), Tol::witness())
        .expect_err("an offset past the axis must not mint");
    assert!(
        matches!(
            e,
            ReplaceFaceError::Offset {
                face: f,
                error: geom_brep::OffsetError::RadiusFloor { .. },
            } if f == face
        ),
        "expected the offset door's radius floor naming {face:?}, got {e}"
    );
    assert_eq!(
        circle_radii(&body),
        circle_radii(&before),
        "the body is untouched on Err"
    );
}

/// **The C5 boundary.** A cone's rims are intersections with the walls
/// either side, and `cone × cylinder` has no route arm: the moved
/// cone cannot be re-stated against its untouched neighbour, so the
/// door refuses naming the pair.
#[test]
fn an_undescribable_neighbor_pair_refuses_typed() {
    let mut body = coned_tube();
    let face = cone_face(&body);
    let e = topo::replace_face_offset(&mut body, face, 0.05, FIT_TOL, band(), Tol::witness())
        .expect_err("the cone's neighbours have no route arm");
    assert!(
        matches!(
            e,
            ReplaceFaceError::NeighborPairUnroutable {
                kind: geom_brep::SurfaceKind::Cone,
                other_kind: geom_brep::SurfaceKind::Cylinder,
                ..
            }
        ),
        "expected the C5 refusal naming (cone, cylinder), got {e}"
    );
}

/// **The apex window.** The cone's `v`-window, shifted by the offset's
/// `d·cot α`, reaches the apex: the mint would put the face's own
/// window on the mirror nappe, so the door refuses BEFORE the boundary
/// is even planned (the C5 refusal above is on the same face at a
/// smaller `d`, so the order is what this row also pins).
#[test]
fn an_apex_window_crossing_refuses_typed() {
    let mut body = coned_tube();
    let face = cone_face(&body);
    let e = topo::replace_face_offset(&mut body, face, 1.5, FIT_TOL, band(), Tol::witness())
        .expect_err("a window shifted across the apex must not be called this face's offset");
    assert!(
        matches!(e, ReplaceFaceError::ApexWindow { face: f, .. } if f == face),
        "expected the apex-window refusal naming {face:?}, got {e}"
    );
}

// ---------------------------------------------------------------------
// The `Approx` row
// ---------------------------------------------------------------------

/// **The fitted lane, through the door.** A lofted prism's wall face
/// carries a NURBS surface, so the door reaches the fit door and mints
/// the certified `Approx` — and then refuses, by name, at the first
/// boundary edge the fit's own rows do not carry.
///
/// **That refusal is the honest one, and it is structural rather than
/// tolerance-shaped.** A fitted chart covers exactly its own parameter
/// window. Move the face, and the seam it shares with the next wall is
/// no longer a row of EITHER chart: the neighbour would have to extend
/// to meet it, which a bounded chart cannot do. No `d` makes that
/// close, so the door decides it before it mutates rather than letting
/// the pcurve lane discover it after.
///
/// Every edge of the wall's boundary refuses for that one reason — the
/// mapped rims because a `v`-row is not an `IsoCurve` (which is
/// `u`-const by definition) and so cannot be re-described at all, the
/// seams because the chart on the other side is bounded too. The row
/// pins the variant rather than which edge the loop walk reaches
/// first, which is bookkeeping.
#[test]
fn the_fitted_lane_refuses_at_a_shared_bounded_chart() {
    for d in [5e-10_f64, -5e-10] {
        let mut body = prism();
        let wall = *spline_walls(&body)
            .first()
            .expect("the prism has spline walls");
        let before = body.clone();
        let e = topo::replace_face_offset(&mut body, wall, d, FIT_TOL, band(), Tol::witness())
            .expect_err("a fitted face's shared seam has nowhere to go");
        assert!(
            matches!(e, ReplaceFaceError::FittedBoundaryUnsupported { .. }),
            "d = {d}: expected the bounded-chart refusal, got {e}"
        );
        assert_eq!(
            format!("{:?}", body.faces().collect::<Vec<_>>()),
            format!("{:?}", before.faces().collect::<Vec<_>>()),
            "d = {d}: the body is untouched on Err"
        );
    }
}

/// **The spline-space obligation, discharged by extraction.** The
/// carrier the door hands a fitted chart's iso seam is that chart's own
/// boundary row, so it carries the FIT's degree and the fit's knot
/// vector — the elevation and the refinement both, and neither by
/// transforming the old carrier.
///
/// The row asserts against the fit itself rather than a constant, and
/// it asserts the SEED is not already there: the loft wall's carrier is
/// degree 1 over a two-knot vector, so a lane that forgot either
/// operation would be visible here.
#[test]
fn a_fitted_charts_iso_row_carries_the_fits_spline_space() {
    let d = 5e-10;
    let body = prism();
    let wall = *spline_walls(&body).first().unwrap();
    let Some(Surface::Nurbs(base)) = body.get_surface(body.get_face(wall).unwrap().surface) else {
        panic!("the wall carries a spline")
    };
    let seed_degree = base.knots_v().degree();
    let seed_knots = base.knots_v().knots().to_vec();

    let approx = geom_brep::approx_offset_surface(base.clone(), d, FIT_TOL, band())
        .expect("the wall's own offset fits");
    let Surface::Approx(a) = &approx else {
        panic!("the fit door mints the variant")
    };
    let row = geom_brep::boundary_iso_u(a.fit(), false).expect("the fit's u = 0 row extracts");

    assert_eq!(
        row.knots().degree(),
        FIT_DEGREE,
        "the row carries the fit's degree"
    );
    assert_eq!(
        row.knots().knots(),
        a.fit().knots_v().knots(),
        "the row carries the fit's knot vector, interior knots included"
    );
    assert!(
        seed_degree < FIT_DEGREE,
        "the seed carrier is below the fit's degree, so elevation is a real step \
         (seed degree {seed_degree})"
    );
    assert!(
        row.knots().knots().len() > seed_knots.len(),
        "the fit refined past the seed grid, so refinement is a real step too (seed {seed_knots:?}, \
         fit {:?})",
        row.knots().knots()
    );
}

// ---------------------------------------------------------------------
// The negative space
// ---------------------------------------------------------------------

/// A body the door was not called on is bit-identical: no arena churn,
/// no re-mint, nothing.
#[test]
fn a_body_the_door_did_not_touch_is_bit_identical() {
    let a = tube();
    let b = tube();
    assert_eq!(
        format!("{:?}", a.faces().collect::<Vec<_>>()),
        format!("{:?}", b.faces().collect::<Vec<_>>())
    );
    let mut moved = a.clone();
    let face = cylinder_face(&moved, 0.8);
    topo::replace_face_offset(&mut moved, face, 0.05, FIT_TOL, band(), Tol::witness()).unwrap();
    assert_ne!(
        radius_of(&moved, face),
        radius_of(&a, face),
        "the door moved the face it was called on"
    );
    assert_eq!(
        radius_of(&a, cylinder_face(&a, 0.4)),
        radius_of(&moved, cylinder_face(&moved, 0.4)),
        "and no other face"
    );
}
