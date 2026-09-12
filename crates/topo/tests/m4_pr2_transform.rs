//! `transform_rigid` acceptance (M4 PR 2, spec D3's Transform clause):
//! key stability, exact dyadic translation, rotation validity through
//! re-certification, composition against booleans, and refusal doors.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common;

use std::f64::consts::FRAC_PI_2;

use common::prism_z;
use geom_core::Tol;
use geom_core::{Affine3, Point3, Vec3};
use topo::{
    Body, BooleanResult, mass_properties, transform_rigid, validate, validate_closed,
    validate_geometric,
};

/// A dyadic brick `[x0,x1]×[y0,y1]×[0,h]` (the M3 fixture builder).
fn brick(x: (f64, f64), y: (f64, f64), h: f64) -> Body<f64> {
    prism_z::<f64>(&[(x.0, y.0), (x.1, y.0), (x.1, y.1), (x.0, y.1)], 0.0, h).body
}

fn tiers_ok(b: &Body<f64>) {
    assert_eq!(validate(b), Ok(()));
    assert_eq!(validate_closed(b), Ok(()));
    assert_eq!(validate_geometric(b, Tol::witness()), Ok(()));
}

#[test]
fn translation_is_exact_and_key_stable() {
    let b = brick((0.0, 2.0), (0.0, 1.0), 0.5);
    let keys: Vec<_> = b.points().map(|(k, _)| k).collect();
    let t = transform_rigid(
        &b,
        &Affine3::translation(Vec3::new(0.25, -1.5, 8.0)),
        Tol::witness(),
    )
    .unwrap();
    tiers_ok(&t);
    // Same keys, exactly translated coordinates (dyadic in, dyadic out).
    for k in keys {
        let p0 = *b.get_point(k).unwrap();
        let p1 = *t.get_point(k).unwrap();
        assert_eq!((p1.x, p1.y, p1.z), (p0.x + 0.25, p0.y - 1.5, p0.z + 8.0));
    }
    let (m0, m1) = (
        mass_properties(&b, Tol::witness()).unwrap(),
        mass_properties(&t, Tol::witness()).unwrap(),
    );
    assert_eq!(m0.volume.to_bits(), m1.volume.to_bits());
    assert_eq!(m0.surface_area.to_bits(), m1.surface_area.to_bits());
}

#[test]
fn zero_angle_rotation_is_exact_identity() {
    let b = brick((0.0, 1.0), (0.0, 1.0), 1.0);
    let map =
        Affine3::rotation_about_axis(Point3::new(0.5, 0.5, 0.0), Vec3::new(0.0, 0.0, 1.0), 0.0);
    let t = transform_rigid(&b, &map, Tol::witness()).unwrap();
    for (k, p0) in b.points() {
        let p1 = t.get_point(k).unwrap();
        assert_eq!(
            (p0.x.to_bits(), p0.y.to_bits(), p0.z.to_bits()),
            (p1.x.to_bits(), p1.y.to_bits(), p1.z.to_bits())
        );
    }
}

#[test]
fn quarter_turn_recertifies_and_preserves_mass_properties_close() {
    let b = brick((0.0, 2.0), (0.0, 1.0), 0.5);
    let map = Affine3::rotation_about_axis(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 1.0),
        FRAC_PI_2,
    );
    let t = transform_rigid(&b, &map, Tol::witness()).unwrap();
    tiers_ok(&t);
    let (m0, m1) = (
        mass_properties(&b, Tol::witness()).unwrap(),
        mass_properties(&t, Tol::witness()).unwrap(),
    );
    assert!((m0.volume - m1.volume).abs() < 1e-12);
    assert!((m0.surface_area - m1.surface_area).abs() < 1e-12);
}

#[test]
fn transformed_tool_subtracts_exactly() {
    // The M3 pocket pattern: translate a dyadic tool onto a face and
    // subtract — the moved body composes with the boolean pipeline.
    let base = brick((0.0, 2.0), (0.0, 2.0), 2.0);
    let tool = brick((-0.125, 0.125), (-0.125, 0.125), 0.25);
    // Place the tool so it embeds in the top face: pocket at (1, 1).
    let map = Affine3::translation(Vec3::new(1.0, 1.0, 1.75));
    let placed = transform_rigid(&tool, &map, Tol::witness()).unwrap();
    tiers_ok(&placed);
    let out = match topo::subtract_with(
        &base,
        &placed,
        &common::flush_declarations(&base, &placed),
        Tol::witness(),
    )
    .unwrap()
    {
        BooleanResult::Body(b) => b.body,
        BooleanResult::Empty => panic!("nonempty subtract"),
    };
    tiers_ok(&out);
    let vol = mass_properties(&out, Tol::witness()).unwrap().volume;
    assert_eq!(vol, 8.0 - 0.25 * 0.25 * 0.25);
}

#[test]
fn non_rigid_maps_are_refused_at_the_door() {
    let b = brick((0.0, 1.0), (0.0, 1.0), 1.0);
    // Uniform scale: affinely self-consistent on planar bodies (re-
    // certification alone would PASS it) — the decided rigidity door
    // is what refuses it.
    let scale = Affine3::from_parts(
        geom_core::Mat3::from_cols(
            Vec3::new(3.0, 0.0, 0.0),
            Vec3::new(0.0, 3.0, 0.0),
            Vec3::new(0.0, 0.0, 3.0),
        ),
        Vec3::zero(),
    );
    // Shear: unit columns, non-orthogonal pair.
    let shear = Affine3::from_parts(
        geom_core::Mat3::from_cols(
            Vec3::new(1.0, 0.0, 0.0),
            Vec3::new(0.5, 1.0, 0.0).normalize(),
            Vec3::new(0.0, 0.0, 1.0),
        ),
        Vec3::zero(),
    );
    // Reflection: orthonormal but determinant −1.
    let mirror = Affine3::from_parts(
        geom_core::Mat3::from_cols(
            Vec3::new(-1.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
            Vec3::new(0.0, 0.0, 1.0),
        ),
        Vec3::zero(),
    );
    for (name, m) in [("scale", scale), ("shear", shear), ("mirror", mirror)] {
        match transform_rigid(&b, &m, Tol::witness()) {
            Err(topo::TransformError::NotRigid { .. }) => {}
            other => panic!("{name}: expected NotRigid refusal, got {other:?}"),
        }
    }
}

// ------------------------------------------------------------------ //
// The M7-8 class: an `Intersection` between a plane and a DESCRIBED   //
// NURBS wall — the one carrier class that certifies only through the  //
// injected plane × NURBS lane.                                        //
// ------------------------------------------------------------------ //

/// A DESCRIBED (non-placeholder) degree-2 NURBS patch on the plane
/// `y = 0`, u along +x, v along +z — the unit cube's front wall
/// restated as a net rather than as the plane it exactly is, so the
/// only thing separating it from that plane is that it is a
/// `Surface::Nurbs`.
fn nurbs_wall() -> geom::Surface<f64> {
    let k = geom_core::spline::KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
    let ticks = [-1.0, 0.5, 2.0];
    let (mut control, mut weights) = (Vec::new(), Vec::new());
    for &x in &ticks {
        for &z in &ticks {
            control.push(Point3::new(x, 0.0, z));
            weights.push(1.0);
        }
    }
    let n = geom::NurbsSurface::new(k.clone(), k, control, weights).unwrap();
    assert!(
        !n.is_placeholder(),
        "the wall is DESCRIBED — the placeholder is a different refusal"
    );
    geom::Surface::Nurbs(std::sync::Arc::new(n))
}

fn face_surface_of_he(body: &Body<f64>, he: topo::HalfEdgeKey) -> topo::SurfaceKey {
    let he_data = body.get_half_edge(he).unwrap();
    let loop_data = body.get_loop(he_data.parent_loop).unwrap();
    body.get_face(loop_data.face).unwrap().surface
}

/// The unit cube with its front wall restated as a described NURBS net
/// and that wall's four edges re-described as plane × NURBS
/// `Intersection`s through `Body::set_edge_curve_nurbs_lane` — the
/// M7-8 class, minted through the door that mints it.
fn m7_8_cube() -> Body<f64> {
    let cube = common::geometric_cube::<f64>();
    let mut body = cube.body;
    let wall = body
        .set_face_surface(cube.mefs[1].face, topo::FaceSurface::New(nurbs_wall()))
        .unwrap();
    let edges: Vec<_> = body.edges().map(|(k, e)| (k, e.clone())).collect();
    let mut lane_edges = 0;
    for (edge_key, edge) in edges {
        let s1 = face_surface_of_he(&body, edge.he_plus);
        let s2 = face_surface_of_he(&body, edge.he_minus);
        if s1 != wall && s2 != wall {
            continue;
        }
        let start = body.get_half_edge(edge.he_plus).unwrap().start;
        let end = body.half_edge_end(edge.he_plus).unwrap();
        let p0 = *body
            .get_point(body.get_vertex(start).unwrap().point)
            .unwrap();
        let p1 = *body.get_point(body.get_vertex(end).unwrap().point).unwrap();
        let kv = geom_core::spline::KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
        let carrier = geom::Curve3::Nurbs(std::sync::Arc::new(
            geom::NurbsCurve3::new(kv, vec![p0, p1], vec![1.0, 1.0]).unwrap(),
        ));
        body.set_edge_curve_nurbs_lane(
            edge_key,
            geom_brep::EdgeCurveSpec {
                description: geom_brep::EdgeDescriptionSpec::Intersection {
                    s1,
                    s2,
                    witness: p0.lerp(p1, 0.5),
                },
                carrier,
                param_start: 0.0,
                param_end: 1.0,
            },
            Tol::witness(),
        )
        .unwrap();
        lane_edges += 1;
    }
    assert_eq!(lane_edges, 4, "the front wall has four M7-8 edges");
    body
}

/// The number of check-2 findings the certified at-rest door raises on
/// `body`. A described-NURBS face has no certified flux lane, so the
/// composed door answers `VolumeUncomputable` at check 7 whatever
/// happens at check 2; counting the edge-certification arm is what
/// isolates the carrier question from that.
fn edge_findings(body: &Body<f64>) -> usize {
    match topo::validate_pseudomanifold_certified(
        body,
        &topo::ContactRecords::default(),
        Tol::witness(),
    ) {
        Ok(()) => 0,
        Err(errs) => errs
            .iter()
            .filter(|e| matches!(e, topo::ValidationError::EdgeCertification { .. }))
            .count(),
    }
}

/// **A body the certified at-rest door calls valid is a body the
/// kernel can move** — through the door that names the lane. Which
/// door a caller takes decides whether the map is attempted at all,
/// and the lane-free door's refusal is a fact about that door's
/// rights, never about the body.
#[test]
fn an_m7_8_body_validates_at_rest_and_moves_through_the_lane() {
    let body = m7_8_cube();
    assert_eq!(
        edge_findings(&body),
        0,
        "the certified at-rest door re-derives all four M7-8 certificates"
    );
    let map = Affine3::translation(Vec3::new(0.25, -1.5, 8.0));

    // The lane-free door refuses, TYPED, naming the class.
    match transform_rigid(&body, &map, Tol::witness()) {
        Err(topo::TransformError::Certify {
            source: geom_brep::CertifyError::Unimplemented,
            ..
        }) => {}
        other => panic!("lane-free door: expected Unimplemented, got {other:?}"),
    }

    // The same body and the same map, through the door that supplies
    // the lane: it moves, and re-derives at rest afterwards.
    let moved = topo::transform_rigid_via(
        &body,
        &map,
        Tol::witness(),
        Some(&geom_brep::plane_nurbs_limbs::<f64>),
    )
    .expect("an M7-8 body moves when the caller names the lane");
    assert_eq!(
        edge_findings(&moved),
        0,
        "and re-derives all four certificates after the map"
    );
    // Topology and keys are untouched, exactly as for every other
    // carrier class.
    assert_eq!(validate_closed(&moved), Ok(()));
    let before: Vec<_> = body.points().map(|(k, _)| k).collect();
    let after: Vec<_> = moved.points().map(|(k, _)| k).collect();
    assert_eq!(before, after, "point keys are stable across the map");
}

/// The lane changes ONE thing and the negative row is what says so: a
/// body with no M7-8 edge maps identically through both doors, so the
/// injected lane is not a second code path for the ordinary classes.
#[test]
fn the_lane_changes_nothing_for_a_body_that_does_not_carry_the_class() {
    let b = brick((0.0, 2.0), (0.0, 1.0), 0.5);
    let map = Affine3::translation(Vec3::new(0.25, -1.5, 8.0));
    let plain = transform_rigid(&b, &map, Tol::witness()).unwrap();
    let laned = topo::transform_rigid_via(
        &b,
        &map,
        Tol::witness(),
        Some(&geom_brep::plane_nurbs_limbs::<f64>),
    )
    .unwrap();
    let pts = |body: &Body<f64>| -> Vec<_> { body.points().map(|(_, p)| *p).collect() };
    assert_eq!(pts(&plain), pts(&laned));
    tiers_ok(&laned);
}
