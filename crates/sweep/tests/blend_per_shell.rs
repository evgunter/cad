//! **A blend carves each chain inside the shell it lies in**, whatever
//! else the body holds: a disjoint union is two solids of one shell
//! each, a sealed cavity is one solid of an outer and a void shell, and
//! a request over either carves its own shells and carries every other
//! one through entity for entity.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common::cavity::{brick, edges_with_corners};
use crate::common::oracles::{chamfered_cube_removed, chamfered_cube_volume, rounded_box_volume};
use geom_core::{Point3, Tol};
use sweep::blend::build::{Blended, fillet_edges};
use sweep::chamfer::chamfer_edges;
use sweep::test_support::finished;
use topo::{AtRestBody, Body, EdgeKey, FaceKey, ShellKey};

/// The blend size, meters.
const R: f64 = 0.1;

/// Two unit boxes a unit apart along `x`, unioned: two solids.
fn two_boxes() -> Body<f64> {
    let tol = Tol::witness();
    let a = finished(
        "box a",
        brick(Point3::new(0.0, 0.0, 0.0), Point3::new(1.0, 1.0, 1.0)),
        tol,
    );
    let b = finished(
        "box b",
        brick(Point3::new(2.0, 0.0, 0.0), Point3::new(3.0, 1.0, 1.0)),
        tol,
    );
    topo::union(&a, &b, tol)
        .expect("a disjoint union succeeds")
        .body()
        .expect("the union keeps material")
        .body
        .clone()
        .into_body()
}

/// The shell of edge `e`'s `plus` side.
fn shell_of_edge(body: &Body<f64>, e: EdgeKey) -> ShellKey {
    let (f, _) = topo::readback::edge_sides(body, e)
        .expect("an edge has two sides")
        .faces();
    body.get_face(f).expect("live").shell
}

/// Box `a` (`x < 1.5`) or box `b`: its shell and its edges, found by
/// position.
fn box_at(body: &Body<f64>, a_side: bool) -> (ShellKey, Vec<EdgeKey>) {
    let edges = edges_with_corners(body, |p| (p.x < 1.5) == a_side);
    (shell_of_edge(body, edges[0]), edges)
}

/// **The shells not asked about are the source's, entity for entity**:
/// every face and edge of `kept` survives under its own key, in the
/// same shell, between the same points; and the body has as many
/// solids and shells as it had.
fn carried_through(what: &str, source: &Body<f64>, out: &Blended<f64>, kept: ShellKey) {
    assert!(
        !out.shells.contains(&kept),
        "{what}: the carried shell is not reported carved"
    );
    assert_eq!(
        out.body.solids().count(),
        source.solids().count(),
        "{what}: the solid count stands"
    );
    assert_eq!(
        out.body.shells().count(),
        source.shells().count(),
        "{what}: the shell count stands"
    );
    let faces: Vec<FaceKey> = source.get_shell(kept).expect("live").faces.clone();
    assert_eq!(
        out.body
            .get_shell(kept)
            .expect("the carried shell lives")
            .faces,
        faces,
        "{what}: the carried shell owns the same faces"
    );
    let ends = |b: &Body<f64>, e: EdgeKey| {
        let edge = b.get_edge(e)?;
        let start = b.get_half_edge(edge.he_plus)?.start;
        let end = b.half_edge_end(edge.he_plus)?;
        let pt = |v| b.get_vertex(v).and_then(|x| b.get_point(x.point)).copied();
        Some((pt(start)?.to_array(), pt(end)?.to_array()))
    };
    let kept_edges: Vec<EdgeKey> = topo::query::all_edges(source)
        .into_iter()
        .filter(|e| shell_of_edge(source, *e) == kept)
        .collect();
    assert!(
        !kept_edges.is_empty(),
        "{what}: the carried shell has edges"
    );
    for e in kept_edges {
        assert_eq!(
            ends(&out.body, e),
            ends(source, e),
            "{what}: carried edge {e:?} survives between the same points"
        );
        assert_eq!(
            shell_of_edge(&out.body, e),
            kept,
            "{what}: carried edge {e:?} stays in its shell"
        );
    }
}

fn volume(what: &str, body: &Body<f64>) -> f64 {
    let tol = Tol::witness();
    AtRestBody::validate(body.clone(), tol)
        .unwrap_or_else(|e| panic!("{what}: the result is a finished body: {e:?}"));
    topo::mass_properties(body, tol)
        .expect("closed-form mass properties")
        .volume
}

fn assert_volume(what: &str, got: f64, want: f64) {
    assert!(
        (got - want).abs() <= 1e-12 * want,
        "{what}: V = {got}, closed form {want}"
    );
}

/// **One box of a disjoint union, filleted whole**: its twelve edges
/// round at the Steiner form and the other box rides through.
#[test]
fn one_box_of_a_disjoint_union_fillets_and_the_other_rides_through() {
    let body = two_boxes();
    assert_eq!(body.solids().count(), 2, "a disjoint union is two solids");
    let (a, a_edges) = box_at(&body, true);
    let (b, _) = box_at(&body, false);
    assert_eq!(a_edges.len(), 12, "box a's twelve edges");
    let out = fillet_edges(
        &sweep::test_support::at_rest(&body),
        &a_edges,
        R,
        Tol::witness(),
    )
    .expect("box a fillets");
    assert_eq!(out.shells, vec![a], "box a's shell is the one carved");
    carried_through("fillet", &body, &out, b);
    assert_volume(
        "a filleted box beside a unit box",
        volume("fillet", &out.body),
        rounded_box_volume(1.0 - 2.0 * R, R) + 1.0,
    );
}

/// **Both boxes in one request**: each chain carves its own shell, and
/// both are reported.
#[test]
fn both_boxes_of_a_disjoint_union_fillet_in_one_request() {
    let body = two_boxes();
    let (a, mut edges) = box_at(&body, true);
    let (b, b_edges) = box_at(&body, false);
    edges.extend(b_edges);
    let out = fillet_edges(
        &sweep::test_support::at_rest(&body),
        &edges,
        R,
        Tol::witness(),
    )
    .expect("both boxes fillet");
    let mut want = vec![a, b];
    want.sort_unstable();
    assert_eq!(out.shells, want, "both shells are carved");
    assert_volume(
        "two filleted boxes",
        volume("both", &out.body),
        2.0 * rounded_box_volume(1.0 - 2.0 * R, R),
    );
}

/// **The chamfer takes the same door.**
#[test]
fn one_box_of_a_disjoint_union_chamfers_and_the_other_rides_through() {
    let body = two_boxes();
    let (_, a_edges) = box_at(&body, true);
    let (b, _) = box_at(&body, false);
    let out = chamfer_edges(
        &sweep::test_support::at_rest(&body),
        &a_edges,
        R,
        Tol::witness(),
    )
    .expect("box a chamfers");
    carried_through("chamfer", &body, &out, b);
    assert_volume(
        "a chamfered box beside a unit box",
        volume("chamfer", &out.body),
        chamfered_cube_volume(1.0, R) + 1.0,
    );
}

/// **A sealed cavity — one solid, an outer and a void shell.** The
/// void's twelve edges are concave; filleted, the void becomes the
/// Steiner form of its shrunk box, and the outer shell rides through.
/// Chamfered, the void loses what a chamfered cube of its side loses.
#[test]
fn a_sealed_void_blends_inside_its_own_shell() {
    let tol = Tol::witness();
    let block = finished(
        "block",
        brick(Point3::new(0.0, 0.0, 0.0), Point3::new(4.0, 4.0, 4.0)),
        tol,
    );
    let void = finished(
        "void",
        brick(Point3::new(1.0, 1.0, 1.0), Point3::new(3.0, 3.0, 3.0)),
        tol,
    );
    let body = topo::subtract(&block, &void, tol)
        .expect("the cut succeeds")
        .body()
        .expect("the cut keeps material")
        .body
        .clone()
        .into_body();
    assert_eq!(body.solids().count(), 1, "a sealed cavity is one solid");
    assert_eq!(body.shells().count(), 2, "of an outer and a void shell");
    let inner = |p: Point3<f64>| p.to_array().iter().all(|c| (1.0..=3.0).contains(c));
    let void_edges = edges_with_corners(&body, inner);
    assert_eq!(void_edges.len(), 12, "the void's twelve edges");
    let outer_edges = edges_with_corners(&body, |p| !inner(p));
    assert_eq!(outer_edges.len(), 12, "the block's twelve edges");
    let outer = shell_of_edge(&body, outer_edges[0]);
    assert_ne!(
        outer,
        shell_of_edge(&body, void_edges[0]),
        "the void is a shell of its own"
    );

    let r = 0.25;
    let out = fillet_edges(&sweep::test_support::at_rest(&body), &void_edges, r, tol)
        .expect("the void fillets");
    carried_through("void fillet", &body, &out, outer);
    assert_volume(
        "a block with a filleted void",
        volume("void fillet", &out.body),
        64.0 - rounded_box_volume(2.0 - 2.0 * r, r),
    );

    let out = chamfer_edges(&sweep::test_support::at_rest(&body), &void_edges, r, tol)
        .expect("the void chamfers");
    carried_through("void chamfer", &body, &out, outer);
    assert_volume(
        "a block with a chamfered void",
        volume("void chamfer", &out.body),
        64.0 - 8.0 + chamfered_cube_removed(2.0, r),
    );
}
