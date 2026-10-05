//! **A record hop past a resolved face, half-edge or edge panics; a key
//! the caller carries keeps its typed or `None` answer** (D2 row 4),
//! one row per converted read path across `boolean/`. Each tears one
//! record a sound fixture's read passes through, and asserts the panic
//! names the link and the premise, with the body deep-unchanged
//! ([`assert_torn_op_panics`]); a row's sound answer is read first.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::body::Body;
use crate::entity::{EntityId, FaceKey, GeomRef, HalfEdgeKey, LoopBoundary};
use crate::live::OPERATORS_KEEP_LINKS;
use crate::review_d18::{ROW_FOUR, assert_torn_op_panics};
use crate::test_support_fixtures::{CylFrame, cyl_wall_sheet, geometric_cube};
use geom_core::{Band, Point3, Tol, Vec3};

fn band() -> Band {
    Band::linear(Tol::witness()).unwrap()
}

fn cube() -> Body<f64> {
    geometric_cube::<f64>(Tol::witness()).body
}

fn cyl_sheet() -> (Body<f64>, FaceKey) {
    let mut body = Body::<f64>::new();
    let face = cyl_wall_sheet(
        &mut body,
        CylFrame::canonical(1.0),
        None,
        (0.2, 1.4),
        (0.0, 1.0),
        Tol::witness(),
    );
    (body, face)
}

fn first_face(body: &Body<f64>) -> FaceKey {
    body.faces().next().map(|(k, _)| k).unwrap()
}

/// `face`'s outer loop members, in walk order.
fn outer_members(body: &Body<f64>, face: FaceKey) -> Vec<HalfEdgeKey> {
    let outer = body.get_face(face).unwrap().outer;
    let LoopBoundary::Cycle { first } = body.get_loop(outer).unwrap().boundary else {
        panic!("the fixture's outer loop is a cycle");
    };
    body.loop_cycle(first).unwrap()
}

/// Drops the curve of `he`'s edge, and names the link that now dangles.
fn drop_curve(body: &mut Body<f64>, he: HalfEdgeKey) -> String {
    let edge = body.get_half_edge(he).unwrap().edge;
    let curve = body.get_edge(edge).unwrap().curve;
    body.curves.remove(curve);
    format!(
        "{}'s curve names {}",
        EntityId::Edge(edge),
        GeomRef::Curve(curve)
    )
}

/// Drops `face`'s outer loop, and names the link that now dangles.
fn drop_outer(body: &mut Body<f64>, face: FaceKey) -> String {
    let outer = body.get_face(face).unwrap().outer;
    body.loops.remove(outer);
    format!("'s outer names {}", EntityId::Loop(outer))
}

/// Drops `face`'s surface, and names the link that now dangles.
fn drop_surface(body: &mut Body<f64>, face: FaceKey) -> String {
    let surface = body.get_face(face).unwrap().surface;
    body.surfaces.remove(surface);
    format!(
        "{}'s surface names {}",
        EntityId::Face(face),
        GeomRef::Surface(surface)
    )
}

/// `boxes.rs`: the window walk panics on a torn loop, where it answered
/// `None`, and the edge box on a torn curve, where it answered the
/// poison box an uncertified curve gets.
#[test]
fn the_box_reads_panic_on_a_torn_loop_and_a_torn_curve() {
    use super::boxes::{edge_box, face_window_steps};
    let mut body = cube();
    let face = first_face(&body);
    assert_eq!(face_window_steps(&body, face).len(), 1, "one loop");
    let he = outer_members(&body, face)[0];
    let edge = body.get_half_edge(he).unwrap().edge;
    assert!(
        edge_box(&body, edge, 0.0).min_x.is_finite(),
        "a certified box"
    );
    let mut torn = body.clone();
    let outer = torn.get_face(face).unwrap().outer;
    drop_outer(&mut torn, face);
    let named = format!("'s loop names {}", EntityId::Loop(outer));
    assert_torn_op_panics(
        "face_window_steps",
        &mut torn,
        &[&named, ROW_FOUR, OPERATORS_KEEP_LINKS],
        |b| face_window_steps(b, face).len(),
    );
    let named = drop_curve(&mut body, he);
    assert_torn_op_panics(
        "edge_box",
        &mut body,
        &[&named, ROW_FOUR, OPERATORS_KEEP_LINKS],
        |b| edge_box(b, edge, 0.0),
    );
}

/// `boxes.rs` `face_box`: a cylinder face's axial window panics on a
/// torn boundary curve, where it read as an edge with no claimable span.
#[test]
fn the_face_box_panics_on_a_torn_boundary_curve() {
    use super::boxes::face_box;
    let (mut body, face) = cyl_sheet();
    assert!(
        face_box(&body, face, 0.0, band())
            .unwrap()
            .min_x
            .is_finite(),
        "a certified box"
    );
    let he = outer_members(&body, face)[0];
    let named = drop_curve(&mut body, he);
    assert_torn_op_panics(
        "face_box",
        &mut body,
        &[&named, ROW_FOUR, OPERATORS_KEEP_LINKS],
        |b| face_box(b, face, 0.0, band()).map(|bx| bx.min_x.is_finite()),
    );
}

/// `finish.rs` `pinch_site`: a face round the pierce vertex whose loop
/// walk breaks panics, where it refused `JoinDesync` ("not walkable").
#[test]
fn the_pinch_site_panics_on_a_broken_walk_round_its_vertex() {
    let mut body = cube();
    let face = first_face(&body);
    let members = outer_members(&body, face);
    let start = |he| body.get_half_edge(he).unwrap().start;
    let (u, w) = (start(members[0]), start(members[2]));
    assert!(
        matches!(super::finish::pinch_site(&body, u, w, |_| true), Ok(Some((f, _))) if f == face),
        "two diagonal corners pinch across their one face"
    );
    // `members[1]` runs between the two corners after `u`: no step of
    // `u`'s orbit reads it, and the face's walk does.
    body.half_edges.remove(members[1]);
    assert_torn_op_panics(
        "pinch_site",
        &mut body,
        &["the loop walk from", ROW_FOUR],
        |b| super::finish::pinch_site(b, u, w, |_| true).map(|s| s.map(|(f, _)| f)),
    );
}

/// `ops.rs` `describe_edges`: a torn curve panics, where it read as an
/// edge with no existing description and was re-described.
#[test]
fn describe_edges_panics_on_a_torn_curve() {
    let mut body = cube();
    let he = outer_members(&body, first_face(&body))[0];
    let edge = body.get_half_edge(he).unwrap().edge;
    let tol = Tol::witness();
    let mut sound = body.clone();
    super::ops::describe_edges(&mut sound, [edge], &[], band(), tol).unwrap();
    let named = drop_curve(&mut body, he);
    assert_torn_op_panics(
        "describe_edges",
        &mut body,
        &[&named, ROW_FOUR, OPERATORS_KEEP_LINKS],
        |b| super::ops::describe_edges(b, [edge], &[], band(), tol).is_ok(),
    );
}

/// `solid_contain.rs` `point_in_face`: a torn outer loop panics, where
/// it read as an empty one and answered "not in the face".
#[test]
fn point_in_face_panics_on_a_torn_outer_loop() {
    let mut body = cube();
    let face = first_face(&body);
    let plane = super::reduce::face_plane(&body, face).expect("a cube face is planar");
    let members = outer_members(&body, face);
    let corner = |he| {
        let v = body.get_half_edge(he).unwrap().start;
        *body.get_point(body.get_vertex(v).unwrap().point).unwrap()
    };
    let p = corner(members[0]).lerp(corner(members[2]), 0.5);
    assert_eq!(
        super::solid_contain::point_in_face(&body, face, plane.normal, p, band()).unwrap(),
        Some(true),
        "the face's centre is in it"
    );
    let named = drop_outer(&mut body, face);
    assert_torn_op_panics(
        "point_in_face",
        &mut body,
        &[&named, ROW_FOUR, OPERATORS_KEEP_LINKS],
        |b| super::solid_contain::point_in_face(b, face, plane.normal, p, band()),
    );
}

/// `solid_contain.rs` `sphere_chart_trim`: a torn curve panics, where it
/// was stepped over as null scaffolding. The walk reads each edge's
/// curve before it asks anything of the sphere, so a cylinder sheet's
/// boundary reaches the read.
#[test]
fn the_sphere_trim_panics_on_a_torn_curve() {
    let (mut body, face) = cyl_sheet();
    let ask = |b: &Body<f64>| {
        super::solid_contain::sphere_chart_trim(
            b,
            face,
            Point3::origin(),
            1.0,
            Vec3::unit_z(),
            band(),
        )
        .map(|t| t.is_some())
    };
    assert!(ask(&body).is_ok(), "the sound sheet answers");
    let he = outer_members(&body, face)[0];
    let named = drop_curve(&mut body, he);
    assert_torn_op_panics(
        "sphere_chart_trim",
        &mut body,
        &[&named, ROW_FOUR, OPERATORS_KEEP_LINKS],
        |b| ask(b),
    );
}

/// `surface_group.rs` `unmated_boundary` (through `wrap_rims`): a torn
/// curve on an unmated edge panics, where it read as an edge that is
/// not a circle and answered "does not wrap".
#[test]
fn the_wrap_scan_panics_on_a_torn_curve() {
    use super::surface_group::{WrapRims, wrap_rims};
    let (mut body, face) = cyl_sheet();
    let rims = WrapRims::Coaxial {
        origin: Point3::origin(),
        axis: Vec3::unit_z(),
    };
    assert!(
        wrap_rims(&body, face, rims, band()).is_ok(),
        "the sound sheet answers"
    );
    let he = outer_members(&body, face)[0];
    let named = drop_curve(&mut body, he);
    assert_torn_op_panics(
        "wrap_rims",
        &mut body,
        &[&named, ROW_FOUR, OPERATORS_KEEP_LINKS],
        |b| wrap_rims(b, face, rims, band()).map(|r| r.is_some()),
    );
}

/// `reduce.rs` `face_plane`: a stale face reads `None` (the caller's
/// key); a torn surface panics, where it read as a face that is not a
/// plane.
#[test]
fn face_plane_answers_a_stale_face_none_and_panics_on_a_torn_surface() {
    let mut body = cube();
    let face = first_face(&body);
    assert!(super::reduce::face_plane(&body, face).is_some(), "a plane");
    let mut stale = body.clone();
    stale.faces.remove(face);
    assert!(
        super::reduce::face_plane(&stale, face).is_none(),
        "a face the caller passed that does not resolve reads None"
    );
    let named = drop_surface(&mut body, face);
    assert_torn_op_panics(
        "face_plane",
        &mut body,
        &[&named, ROW_FOUR, OPERATORS_KEEP_LINKS],
        |b| super::reduce::face_plane(b, face).is_some(),
    );
}

/// `reduce.rs` `gate_maximal_faces`: two faces across an edge on one
/// torn surface key panic, where the key read as a curved one and the
/// edge passed as a canonical curved seam. Every face is put on one key
/// first, so every edge is a same-key edge and none reaches the
/// distinct-key arm's own reads.
#[test]
fn the_maximal_faces_gate_panics_on_a_torn_shared_surface() {
    let mut body = cube();
    let faces: Vec<FaceKey> = body.faces().map(|(k, _)| k).collect();
    let shared = body.get_face(faces[0]).unwrap().surface;
    for &f in &faces {
        body.faces.get_mut(f).unwrap().surface = shared;
    }
    assert!(
        matches!(
            super::reduce::gate_maximal_faces(&body, super::Operand::A, band()),
            Err(super::BooleanError::NonMaximalFaces { .. })
        ),
        "planar faces on one key are not maximal"
    );
    body.surfaces.remove(shared);
    assert_torn_op_panics(
        "gate_maximal_faces",
        &mut body,
        &[
            &format!("'s surface names {}", GeomRef::Surface(shared)),
            ROW_FOUR,
            OPERATORS_KEEP_LINKS,
        ],
        |b| super::reduce::gate_maximal_faces(b, super::Operand::A, band()),
    );
}

/// `vtxfac.rs` `classify_vertex_on_face`: a contact face that does not
/// resolve refuses typed (the key the sweep carries); a torn surface on
/// one that does panics, where it read as the kind with no arm.
#[test]
fn the_vertex_on_face_classification_panics_on_a_torn_pierced_surface() {
    use super::{BooleanError, BooleanOp, ContactRecords, DeclaredPairs, Operand, VfContact};
    let piercing = cube();
    let vertex = piercing.vertices().next().map(|(k, _)| k).unwrap();
    let pierced = cube();
    let face = first_face(&pierced);
    let contact = VfContact { vertex, face };
    let classify = |a: &mut Body<f64>, b: &mut Body<f64>| {
        super::vtxfac::classify_vertex_on_face(
            a,
            b,
            Operand::A,
            contact,
            BooleanOp::Union,
            &DeclaredPairs::default(),
            &ContactRecords::default(),
            band(),
            Tol::witness(),
        )
        .map(drop)
    };
    let mut stale = pierced.clone();
    stale.faces.remove(face);
    assert!(
        matches!(
            classify(&mut piercing.clone(), &mut stale),
            Err(BooleanError::ClassificationInvariant { .. })
        ),
        "a contact face that does not resolve refuses typed"
    );
    let mut torn = pierced.clone();
    let named = drop_surface(&mut torn, face);
    assert_torn_op_panics(
        "classify_vertex_on_face",
        &mut torn,
        &[&named, ROW_FOUR, OPERATORS_KEEP_LINKS],
        |b| classify(&mut piercing.clone(), b),
    );
}
