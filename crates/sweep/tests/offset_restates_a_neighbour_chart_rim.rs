//! **An offset door restates a rim described in its held neighbour's
//! chart on the chart it re-mints.**
//!
//! A drum's top rim, re-described as an image in the top cap's chart;
//! the wall offset inward, the caps held (`offset_charts_together`
//! keeps a chart asked to move nothing). The rim's image names only the
//! cap, which no longer vouches for the moved wall's side, so the door
//! states the moved rim on the minted wall: as the section of the two
//! charts, or, where a declaration rides the image, as an image in the
//! wall's own chart.
//!
//! The planar door (`offset_planes_together`) takes the same shape on a
//! cube: its top edge re-described as an image in a side's chart, the
//! top moved and every other chart held.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use crate::common::approx::band;
use geom::Surface;
use geom_brep::{
    EdgeAuthority, EdgeCurve, EdgeDescription, EdgeDescriptionSpec, MappedCurve, SketchSegment,
};
use geom_core::{Affine3, Point2, Point3, Tol, Vec2, Vec3};
use profile::{Profile, SketchPlane, test_support::bulge_loop};
use sweep::{Revolution, RevolveAxis, revolve};
use topo::readback::edge_sides;
use topo::{
    Body, ChartMove, CurveGeom, EdgeKey, SurfaceKey, ValidationError, offset_charts_together,
    offset_planes_together, replace_faces_offset, validate_closed, validate_geometric,
};

const R: f64 = 3.0 / 64.0;
const H: f64 = 8.0 / 64.0;

/// A drum about `y`, its wall's key, and one rim re-described as an
/// image in its cap's chart (`declared` riding the image).
fn drum_with_a_rim_in_the_caps_chart(
    declared: Option<MappedCurve<f64>>,
) -> (Body<f64>, SurfaceKey, EdgeKey, SurfaceKey) {
    let profile = Profile::new(
        SketchPlane::xy(),
        vec![bulge_loop(vec![
            (Point2::new(0.0, 0.0), 0.0),
            (Point2::new(R, 0.0), 0.0),
            (Point2::new(R, H), 0.0),
            (Point2::new(0.0, H), 0.0),
        ])],
    )
    .validate(Tol::witness())
    .unwrap();
    let mut body = revolve(
        &profile,
        RevolveAxis {
            origin: Point2::new(0.0, 0.0),
            dir: Vec2::new(0.0, 1.0),
        },
        Revolution::Full,
        Tol::witness(),
    )
    .unwrap()
    .body;
    let kind = |key| body.get_surface(key).unwrap();
    let wall = body
        .faces()
        .find(|(_, f)| matches!(kind(f.surface), Surface::Cylinder { .. }))
        .map(|(_, f)| f.surface)
        .unwrap();
    let (rim, cap) = body
        .edges()
        .find_map(|(e, _)| match edge_sides(&body, e).unwrap().surfaces() {
            (s, cap) | (cap, s) if s == wall && cap != wall => Some((e, cap)),
            _ => None,
        })
        .unwrap();
    assert!(
        matches!(kind(cap), Surface::Plane { .. }),
        "a rim meets a cap"
    );
    let mut spec = description_of(&body, rim).restated_spec();
    let chart = EdgeDescriptionSpec::chart(cap);
    spec.description = match declared {
        Some(mc) => chart.declared_by(mc),
        None => chart,
    };
    body.set_edge_curve(rim, spec, Tol::witness())
        .expect("the rim lies in the cap's chart");
    (body, wall, rim, cap)
}

fn description_of(body: &Body<f64>, edge: EdgeKey) -> &EdgeCurve<f64> {
    body.get_edge(edge)
        .and_then(|e| body.get_curve_geom(e.curve))
        .and_then(CurveGeom::certified)
        .unwrap()
}

/// The wall offset inward by 1/64 with the caps held; the minted wall's
/// key.
fn offset_the_wall(body: &mut Body<f64>, wall: SurfaceKey) -> SurfaceKey {
    let moves: Vec<ChartMove<f64>> = crate::common::charts::charts(body)
        .into_iter()
        .map(|faces| ChartMove {
            distance: if body.get_face(faces[0]).unwrap().surface == wall {
                -1.0 / 64.0
            } else {
                0.0
            },
            faces,
        })
        .collect();
    let got = offset_charts_together(body, &moves, band(), Tol::witness());
    assert!(
        got.is_ok(),
        "the wall's offset, its rim in the cap's chart: {got:?}"
    );
    let minted = body
        .faces()
        .find(|(_, f)| matches!(body.get_surface(f.surface), Some(Surface::Cylinder { .. })))
        .map(|(_, f)| f.surface)
        .unwrap();
    assert_ne!(minted, wall, "the wall moved onto a minted chart");
    minted
}

#[test]
fn a_rim_in_the_caps_chart_is_restated_as_the_section_with_the_minted_wall() {
    let (mut body, wall, rim, cap) = drum_with_a_rim_in_the_caps_chart(None);
    validate_geometric(&body, Tol::witness()).expect("the re-described drum is tier-3 valid");
    let minted = offset_the_wall(&mut body, wall);
    validate_geometric(&body, Tol::witness()).expect("the offset drum is tier-3 valid");
    let description = description_of(&body, rim).description();
    assert!(
        matches!(
            *description,
            EdgeDescription::Intersection { s1, s2, .. } if (s1, s2) == (minted, cap)
        ),
        "the rim is the section of the minted wall and the held cap: {description:?}"
    );
}

/// The declaration is the rim's own revolve trajectory: the profile's
/// top corner turned half a revolution about `y`.
#[test]
fn a_declared_rim_in_the_caps_chart_keeps_its_declaration_on_the_minted_wall() {
    let (mut body, wall, rim, _) =
        drum_with_a_rim_in_the_caps_chart(Some(MappedCurve::RevolvedPoint {
            point: Point2::new(R, H),
            place: Affine3::identity(),
            axis_origin: Point3::new(0.0, 0.0, 0.0),
            axis_dir: Vec3::new(0.0, 1.0, 0.0),
            angle: std::f64::consts::PI,
        }));
    let minted = offset_the_wall(&mut body, wall);
    validate_closed(&body).expect("the offset drum is tier-2 valid");
    let curve = description_of(&body, rim);
    assert!(
        matches!(curve.description(), EdgeDescription::Chart(c) if c.surface == minted),
        "the declared rim is an image in the minted wall's chart: {:?}",
        curve.description()
    );
    assert!(
        matches!(
            curve.authority(),
            EdgeAuthority::Declared(MappedCurve::RevolvedPoint { point, .. })
                if (point.x - (R - 1.0 / 64.0)).abs() < 1e-12 && (point.y - H).abs() < 1e-12
        ),
        "the declaration is re-authored at the moved corner: {:?}",
        curve.authority()
    );
    // A declared transverse edge is what tier 3's prefer-intrinsic rule
    // names, before the offset and after it alike.
    let errors = validate_geometric(&body, Tol::witness()).unwrap_err();
    assert_eq!(
        errors,
        vec![ValidationError::TransverseNotIntrinsic { edge: rim }],
        "the declaration survived the offset"
    );
}

/// The per-chart door moves one chart and holds every other, so an
/// image in a neighbour's chart is this shape whenever it bounds the
/// moved face; the image itself draws the old locus.
#[test]
fn the_per_chart_door_restates_a_rim_in_the_caps_chart_as_the_section() {
    let (body, wall, rim, cap) = drum_with_a_rim_in_the_caps_chart(None);
    let faces: Vec<_> = body
        .faces()
        .filter(|(_, f)| f.surface == wall)
        .map(|(k, _)| k)
        .collect();
    for d in [-1.0 / 64.0, 1.0 / 64.0] {
        let mut moved = body.clone();
        let got = replace_faces_offset(&mut moved, &faces, d, Tol::witness());
        assert!(got.is_ok(), "the wall's offset by {d}: {got:?}");
        let valid = validate_geometric(&moved, Tol::witness());
        assert!(
            valid.is_ok(),
            "the drum offset by {d} is tier-3 valid: {valid:?}"
        );
        let minted = moved.get_face(faces[0]).unwrap().surface;
        let description = description_of(&moved, rim).description();
        assert!(
            matches!(
                *description,
                EdgeDescription::Intersection { s1, s2, .. } if (s1, s2) == (minted, cap)
            ),
            "offset by {d}, the rim is the section of the minted wall and the held cap: \
             {description:?}"
        );
    }
}

/// A declaration that translates rides into the moved face's own chart:
/// a cube's top edge, declared by its own segment in the top's plane and
/// described in a side's chart, with the top offset by `d`.
#[test]
fn the_per_chart_door_moves_a_declared_edge_into_the_moved_faces_chart() {
    let (mut body, edge, top, _) = cube_with_the_top_edge_in_the_sides_chart(true);
    let faces: Vec<_> = body
        .faces()
        .filter(|(_, f)| f.surface == top)
        .map(|(k, _)| k)
        .collect();
    let d = 1.0 / 64.0;
    let got = replace_faces_offset(&mut body, &faces, d, Tol::witness());
    assert!(got.is_ok(), "the top's offset: {got:?}");
    let minted = body.get_face(faces[0]).unwrap().surface;
    assert_declared_in_the_moved_top(&body, edge, minted, d);
}

/// The planar door takes moves of zero, which keep their chart: a
/// cube's top edge described in a side's chart, the top offset by `d`
/// and every other chart held.
#[test]
fn the_planar_door_restates_an_edge_in_a_held_sides_chart_as_the_section() {
    let (mut body, edge, top, side) = cube_with_the_top_edge_in_the_sides_chart(false);
    let side_face = body
        .faces()
        .find(|(_, f)| f.surface == side)
        .map(|(k, _)| k)
        .unwrap();
    let d = 1.0 / 64.0;
    let minted = offset_the_planes(&mut body, top, d);
    let valid = validate_geometric(&body, Tol::witness());
    assert!(valid.is_ok(), "the offset cube is tier-3 valid: {valid:?}");
    let description = description_of(&body, edge).description();
    let held = body.get_face(side_face).unwrap().surface;
    assert!(
        matches!(
            *description,
            EdgeDescription::Intersection { s1, s2, .. } if (s1, s2) == (minted, held)
        ),
        "the edge is the section of the minted top and the held side: {description:?}"
    );
}

#[test]
fn the_planar_door_moves_a_declared_edge_into_the_moved_faces_chart() {
    let (mut body, edge, top, _) = cube_with_the_top_edge_in_the_sides_chart(true);
    let d = 1.0 / 64.0;
    let minted = offset_the_planes(&mut body, top, d);
    assert_declared_in_the_moved_top(&body, edge, minted, d);
}

/// A cube, its top edge on `y = 0` re-described as an image in the
/// side's chart (declared by its own segment in the top's plane when
/// `declared`), and the edge, top and side keys.
fn cube_with_the_top_edge_in_the_sides_chart(
    declared: bool,
) -> (Body<f64>, EdgeKey, SurfaceKey, SurfaceKey) {
    let mut body: Body<f64> = sweep::test_support::cube(1.0, Tol::witness());
    let edge = body
        .edges()
        .map(|(e, _)| e)
        .find(|&e| {
            let c = description_of(&body, e);
            let (p0, p1) = (
                c.carrier().eval(c.params().0),
                c.carrier().eval(c.params().1),
            );
            (p0.z, p1.z, p0.y, p1.y) == (1.0, 1.0, 0.0, 0.0)
        })
        .expect("the cube's top edge on y = 0");
    let (a, b) = edge_sides(&body, edge).unwrap().surfaces();
    let (top, side) = if is_top(&body, a) { (a, b) } else { (b, a) };
    let mut spec = description_of(&body, edge).restated_spec();
    let (p0, p1) = (
        spec.carrier.eval(spec.param_start),
        spec.carrier.eval(spec.param_end),
    );
    let chart = EdgeDescriptionSpec::chart(side);
    spec.description = if declared {
        chart.declared_by(MappedCurve::PlacedSegment {
            segment: SketchSegment::Line {
                a: Point2::new(p0.x, p0.y),
                b: Point2::new(p1.x, p1.y),
            },
            place: Affine3::translation(Vec3::new(0.0, 0.0, 1.0)),
        })
    } else {
        chart
    };
    body.set_edge_curve(edge, spec, Tol::witness())
        .expect("the top edge lies in the side's chart");
    (body, edge, top, side)
}

/// `top` offset by `d` through the planar door with every other chart
/// held; the minted top's key.
fn offset_the_planes(body: &mut Body<f64>, top: SurfaceKey, d: f64) -> SurfaceKey {
    let moves: Vec<ChartMove<f64>> = crate::common::charts::charts(body)
        .into_iter()
        .map(|faces| ChartMove {
            distance: if body.get_face(faces[0]).unwrap().surface == top {
                d
            } else {
                0.0
            },
            faces,
        })
        .collect();
    let got = offset_planes_together(body, &moves, band(), Tol::witness());
    assert!(
        got.is_ok(),
        "the top's offset, its edge in the side's chart: {got:?}"
    );
    body.faces()
        .map(|(_, f)| f.surface)
        .find(|&k| is_top(body, k) && k != top)
        .expect("the top moved onto a minted chart")
}

/// The declared top edge is an image in the minted top's chart, its
/// declaration translated with the top, and tier 3 still names it.
fn assert_declared_in_the_moved_top(body: &Body<f64>, edge: EdgeKey, minted: SurfaceKey, d: f64) {
    let curve = description_of(body, edge);
    assert!(
        matches!(curve.description(), EdgeDescription::Chart(c) if c.surface == minted),
        "the declared edge is an image in the moved top's chart: {:?}",
        curve.description()
    );
    assert!(
        matches!(
            curve.authority(),
            EdgeAuthority::Declared(MappedCurve::PlacedSegment { place, .. })
                if (place.translation - Vec3::new(0.0, 0.0, 1.0 + d)).norm() < 1e-12
        ),
        "the declaration is translated with the top: {:?}",
        curve.authority()
    );
    assert_eq!(
        validate_geometric(body, Tol::witness()).unwrap_err(),
        vec![ValidationError::TransverseNotIntrinsic { edge }],
        "the declaration survived the offset"
    );
}

fn is_top(body: &Body<f64>, key: SurfaceKey) -> bool {
    matches!(body.get_surface(key), Some(Surface::Plane { normal, .. }) if normal.z.abs() == 1.0)
}

/// Where both of the edge's charts move, the edge slides within the
/// side's chart too, and its image there is derived afresh.
#[test]
fn the_planar_door_derives_the_image_afresh_when_both_charts_move() {
    for declared in [false, true] {
        let (mut body, edge, top, side) = cube_with_the_top_edge_in_the_sides_chart(declared);
        let side_face = body
            .faces()
            .find(|(_, f)| f.surface == side)
            .map(|(k, _)| k)
            .unwrap();
        let moves: Vec<ChartMove<f64>> = crate::common::charts::charts(&body)
            .into_iter()
            .map(|faces| {
                let k = body.get_face(faces[0]).unwrap().surface;
                ChartMove {
                    distance: if k == top || k == side {
                        1.0 / 64.0
                    } else {
                        0.0
                    },
                    faces,
                }
            })
            .collect();
        let got = offset_planes_together(&mut body, &moves, band(), Tol::witness());
        assert!(got.is_ok(), "declared {declared}: the offset: {got:?}");
        let minted = body.get_face(side_face).unwrap().surface;
        let curve = description_of(&body, edge);
        assert!(
            matches!(curve.description(), EdgeDescription::Chart(c) if c.surface == minted),
            "declared {declared}: the edge is an image in the minted side's chart: {:?}",
            curve.description()
        );
        let expected = if declared {
            Err(vec![ValidationError::TransverseNotIntrinsic { edge }])
        } else {
            Ok(())
        };
        assert_eq!(
            validate_geometric(&body, Tol::witness()),
            expected,
            "declared {declared}: tier 3 names the declaration and nothing else"
        );
    }
}
