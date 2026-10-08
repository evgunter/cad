//! **`replace_face_offset` at the body**: one face's surface becomes
//! its certified offset, the face's own boundary is re-described
//! against the moved chart, and the result rests tier-3 valid.
//!
//! The analytic fixture is a revolved rectangular annulus — a tube.
//! Four faces (two coaxial cylinder walls, two annular caps, each
//! carrying its bore as a ring), six edges, four vertices: a `Seam` on
//! each wall and an `Intersection` on each rim. Offsetting the outer
//! wall exercises both transport lanes; offsetting a cap exercises the
//! plane's translation and the two wall seams' re-anchoring.
//!
//! The `Approx` rows sit on the loft prism the OFF-C consumer built,
//! now driven THROUGH the door rather than through the attach layer by
//! hand — the fit is minted from the wall itself rather than from a
//! pulled-back base, so the face genuinely moves and every carrier is
//! re-derived.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::{Curve3, Surface};
use geom_core::{Point2, Tol, Vec2};
use profile::{Profile, SketchPlane, test_support::bulge_loop};
use sweep::ExtrudeSide;
use sweep::{Revolution, RevolveAxis, revolve};
use topo::{Body, CurveGeom, FaceKey, ReplaceFaceError};

use crate::common;
use crate::common::approx::band;
use crate::common::shell_operands::tube;
use common::approx::{FIT_DEGREE, prism};

/// The target these fixtures hand the fit ENGINE, and no longer a door's
/// argument: since the shell chain took the `Tol` witness, the only
/// tolerance a kernel door accepts is the run's ε, and a chosen number
/// reaches the fit only through `geom-brep`'s `_at` instrument. 1e-6 is
/// what these planar pull-backs were always fitted at and the value is
/// unchanged; what moved is what it means.
const ENGINE_FIT_TARGET: f64 = 1e-6;

/// Revolves the closed `(r, y)` polygon a full turn about the `y` axis.
fn revolved(points: &[(f64, f64)]) -> Body<f64> {
    let lp = bulge_loop(
        points
            .iter()
            .map(|(r, y)| (Point2::new(*r, *y), 0.0))
            .collect(),
    );
    let profile = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(Tol::witness())
        .expect("the fixture polygon is a valid profile");
    revolve(
        &profile,
        RevolveAxis {
            origin: Point2::new(0.0, 0.0),
            dir: Vec2::new(0.0, 1.0),
        },
        Revolution::Full,
        Tol::witness(),
    )
    .expect("the fixture polygon revolves")
    .body
}

/// A tube whose outer wall is a cylinder BELOW and a cone ABOVE. The
/// pair `(cone, cylinder)` is ROUTED (the coaxial closed form), so this
/// is the fixture that reaches past C5, and the cone's own `v`-window
/// is what the apex row decides over.
fn coned_tube() -> Body<f64> {
    revolved(&[(0.4, 0.0), (0.8, 0.0), (0.8, 0.3), (0.4, 0.6)])
}

/// A tube whose outer wall is TWO cones of different half-angles: the
/// rim between them is `(cone, cone)`, a pair with no route arm and no
/// closed form claimed for it. This is the C5 row's fixture — the
/// gate's own refusal, on a pair whose general-rung arm has not
/// retired.
fn double_coned_tube() -> Body<f64> {
    revolved(&[(0.4, 0.0), (0.8, 0.0), (0.6, 0.3), (0.4, 0.7)])
}

/// The cone face whose half-angle carries `tan α` — the fixture above
/// has two, and a row that means one of them must say which.
fn cone_face_with_tan(body: &Body<f64>, tan_a: f64) -> FaceKey {
    body.faces()
        .find(|(_, f)| {
            matches!(
                body.get_surface(f.surface),
                Some(Surface::Cone { half_angle, .. }) if (half_angle.tan() - tan_a).abs() < 1e-9
            )
        })
        .map(|(k, _)| k)
        .unwrap_or_else(|| panic!("no cone face with tan alpha = {tan_a}"))
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
/// re-described against the moved cylinder, and the body rests tier-3
/// valid.
#[test]
fn the_cylinder_wall_offsets_at_both_signs() {
    for d in [0.05_f64, -0.05] {
        let mut body = tube(0.4, 0.8, 0.6);
        let face = cylinder_face(&body, 0.8);
        topo::replace_face_offset(&mut body, face, d, Tol::witness())
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

/// The untouched wall's declared meridians' PARAMETER range follows the
/// moved cap: the vessel's angle-0 and angle-π meridians — segments from
/// `y = 0` to `y = 0.4`, each parting the wall's two π-bands and so
/// each a declared image at rest (no wrap edge, D1) — now end at
/// `y = 0.4 + d`, and their authoritative sketch data say so — the door
/// re-states the segment rather than patching the carrier around it.
#[test]
fn the_untouched_walls_declared_meridian_is_re_anchored() {
    let d = 0.05;
    let mut body = common::shell_operands::vessel(0.5, 0.4);
    let cap = plane_face(&body, 0.4);
    topo::replace_face_offset(&mut body, cap, d, Tol::witness()).unwrap();

    // Selected on the sketch datum the door re-states, so the span and
    // the far endpoint are read off the SAME edges.
    let meridians: Vec<(f64, f64)> = body
        .edges()
        .filter_map(|(_, e)| {
            let c = body
                .get_curve_geom(e.curve)
                .and_then(CurveGeom::certified)?;
            let geom_brep::EdgeAuthority::Declared(geom_brep::MappedCurve::PlacedSegment {
                segment: geom_brep::SketchSegment::Line { a, b },
                ..
            }) = c.authority()
            else {
                return None;
            };
            let (t0, t1) = c.params();
            Some(((t1 - t0).abs(), a.y.max(b.y)))
        })
        .collect();
    assert_eq!(
        meridians.len(),
        2,
        "the wall's two declared meridians, got {meridians:?}"
    );
    assert!(
        meridians
            .iter()
            .all(|(span, _)| (span - (0.4 + d)).abs() < 1e-15),
        "the meridian spans the taller wall, got {meridians:?}"
    );
    assert!(
        meridians
            .iter()
            .all(|(_, far)| (far - (0.4 + d)).abs() < 1e-15),
        "the sketch segment's far endpoint followed the cap: {meridians:?}"
    );
}

/// **The planar row.** A cap's plane translates by `d·n`, its two rims
/// translate with it, and the two wall seams are re-anchored at their
/// moved end.
#[test]
fn a_planar_cap_offsets_at_both_signs() {
    for d in [0.05_f64, -0.05] {
        let mut body = tube(0.4, 0.8, 0.6);
        let face = plane_face(&body, 0.6);
        topo::replace_face_offset(&mut body, face, d, Tol::witness())
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
    let mut body = tube(0.4, 0.8, 0.6);
    let faces: Vec<FaceKey> = body.faces().map(|(k, _)| k).collect();
    for face in faces {
        let sense = body.get_face(face).unwrap().sense;
        // Inward is against the chart normal on a positively-sensed
        // face and with it on a reversed one.
        let d = if sense { -0.02 } else { 0.02 };
        topo::replace_face_offset(&mut body, face, d, Tol::witness())
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
    let mut body = tube(0.4, 0.8, 0.6);
    let face = cylinder_face(&body, 0.4);
    let before = body.clone();
    let e = topo::replace_face_offset(&mut body, face, -0.5, Tol::witness())
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
/// either side, and `cone × cone` has no route arm: the moved cone
/// cannot be re-stated against its untouched neighbour, so the door
/// refuses naming the pair.
///
/// The fixture is a pair of cones because `cone × cylinder` — this
/// row's operand until the coaxial arm landed — is now routed. What the
/// row holds is the GATE, not the pair: a neighbour the C5 table
/// declines stops the door before any mutation, and the refusal names
/// which pair declined.
#[test]
fn an_undescribable_neighbor_pair_refuses_typed() {
    let mut body = double_coned_tube();
    let face = cone_face_with_tan(&body, 0.2 / 0.3);
    let e = topo::replace_face_offset(&mut body, face, 0.05, Tol::witness())
        .expect_err("the cone's neighbours have no route arm");
    assert!(
        matches!(
            e,
            ReplaceFaceError::NeighborPairUnroutable {
                kind: geom::SurfaceKind::Cone,
                other_kind: geom::SurfaceKind::Cone,
                ..
            }
        ),
        "expected the C5 refusal naming (cone, cone), got {e}"
    );
}

/// **What the coaxial arm bought, measured at the door.** With
/// `cone × cylinder` routed, the coned tube's cone reaches PAST the C5
/// gate, and the per-chart door derives each rim as the moved cone's
/// coaxial section with the cylinder that did NOT move: a circle of
/// that cylinder's own radius, its corner where the cone's seam meets
/// it. The shared vertex slides along the untouched cylinder rather
/// than following the cone's normal off it.
#[test]
fn the_routed_cone_reaches_past_c5_and_its_rims_stay_on_the_cylinders() {
    let mut body = coned_tube();
    let face = cone_face(&body);
    topo::replace_face_offset(&mut body, face, 0.05, Tol::witness())
        .expect("the cone moves between its untouched cylinders");
    let cylinders: Vec<f64> = body
        .faces()
        .filter_map(|(_, f)| match body.get_surface(f.surface) {
            Some(geom::Surface::Cylinder { radius, .. }) => Some(*radius),
            _ => None,
        })
        .collect();
    let mut rims = 0;
    for he in body
        .half_edges()
        .map(|(he, _)| he)
        .filter(|he| body.face_of_half_edge(*he) == Some(face))
        .collect::<Vec<_>>()
    {
        let edge = body.get_half_edge(he).expect("a live half-edge").edge;
        let curve = body
            .get_curve_geom(body.get_edge(edge).expect("a live edge").curve)
            .and_then(topo::CurveGeom::certified)
            .expect("a certified rim");
        if let geom::Curve3::Circle { radius, .. } = *curve.carrier() {
            assert!(
                cylinders.iter().any(|c| (c - radius).abs() < 1e-12),
                "a rim of radius {radius} is not on an untouched cylinder {cylinders:?}"
            );
            rims += 1;
        }
    }
    assert_eq!(rims, 2, "the moved cone keeps both rims");
}

/// **The apex window.** The cone's `v`-window, shifted by the offset's
/// `d·cot α`, reaches the apex: the mint would put the face's own
/// window on the mirror nappe, so the door refuses BEFORE the boundary
/// is even planned (the row above is the same face at a smaller `|d|`,
/// which moves).
///
/// This wall sits BELOW its apex, and `d` is along the chart normal at
/// the face, so the offset that reaches the apex is the negative one —
/// the door turns it for the mint (`topo::face_nappe`) and the window
/// is read on the face's own nappe.
#[test]
fn an_apex_window_crossing_refuses_typed() {
    let mut body = coned_tube();
    let face = cone_face(&body);
    let e = topo::replace_face_offset(&mut body, face, -1.5, Tol::witness())
        .expect_err("a window shifted across the apex must not be called this face's offset");
    assert!(
        matches!(e, ReplaceFaceError::ApexWindow { face: f, .. } if f == face),
        "expected the apex-window refusal naming {face:?}, got {e}"
    );
}

/// **A shared chart is a multi-face operand, and the door says so
/// before it computes anything.** An extruded disc's two wall faces
/// carry ONE cylinder surface (the profile is two semicircular arcs),
/// which is the natural form of the sharing `step-import`'s adoption
/// also produces. Replacing one wall would re-point the shared vertical
/// seams at a fresh key while the other wall kept the old chart.
#[test]
fn a_shared_surface_key_refuses_typed() {
    let v = |x: f64, y: f64| (Point2::new(x, y), 1.0);
    let lp = bulge_loop(vec![v(-0.5, 0.0), v(0.5, 0.0)]);
    let profile = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(Tol::witness())
        .expect("a disc is a valid profile");
    let mut body = sweep::extrude(
        &profile,
        sweep::Extrusion::Distance {
            depth: 1.0,
            side: ExtrudeSide::Along,
        },
        Tol::witness(),
    )
    .expect("the disc extrudes")
    .body;
    let wall = body
        .faces()
        .find(|(_, f)| matches!(body.get_surface(f.surface), Some(Surface::Cylinder { .. })))
        .map(|(k, _)| k)
        .expect("the extruded disc has cylinder walls");
    let shared = body.get_face(wall).unwrap().surface;
    assert_eq!(
        body.faces().filter(|(_, f)| f.surface == shared).count(),
        2,
        "the fixture's two wall faces really do share one surface"
    );
    let before = format!("{body:?}");
    let e = topo::replace_face_offset(&mut body, wall, 0.05, Tol::witness())
        .expect_err("a shared chart is a multi-face operand");
    assert!(
        matches!(e, ReplaceFaceError::SharedSurfaceKey { face: f, .. } if f == wall),
        "expected the shared-key refusal naming {wall:?}, got {e}"
    );
    assert_eq!(
        format!("{body:?}"),
        before,
        "the refusal is decided before anything is computed or written"
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
        let e = topo::replace_face_offset(&mut body, wall, d, Tol::witness())
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

    let approx = geom_brep::approx_offset_surface_at(base.clone(), d, ENGINE_FIT_TARGET, band())
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
    let a = tube(0.4, 0.8, 0.6);
    let b = tube(0.4, 0.8, 0.6);
    assert_eq!(
        format!("{:?}", a.faces().collect::<Vec<_>>()),
        format!("{:?}", b.faces().collect::<Vec<_>>())
    );
    let mut moved = a.clone();
    let face = cylinder_face(&moved, 0.8);
    topo::replace_face_offset(&mut moved, face, 0.05, Tol::witness()).unwrap();
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
