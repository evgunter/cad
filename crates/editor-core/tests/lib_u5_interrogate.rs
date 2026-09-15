//! **LIB-U5 — the name→geometry doors and their refusal ladder.**
//!
//! The happy path is exercised at the façade (`pncad::select`'s worked
//! example). What is pinned HERE is the part a doctest cannot reach
//! comfortably: every rung of [`InterrogateError`], and `edge_frame`
//! against a body whose edges are lines.
//!
//! Why the ladder deserves a test of its own: these doors are the
//! only route from a stored selection to a coordinate, and a stale
//! selection is the NORMAL case after an upstream edit. Each refusal
//! is a different fact about the model — the name is gone, the name
//! is tied, the node never evaluated, the node failed, the entity is
//! a whole body — and a caller that cannot tell them apart cannot
//! recover from any of them. An untested ladder is one where two
//! rungs silently collapse into each other.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;

use editor_core::{
    CancelToken, Dimension, EntityKind, EvalOptions, Expr, InterrogateError, Node, ProfileDoc,
    RecipeNodeId, RoleSeg, StableName, all_edges, all_faces, all_vertices, denotation, edge_frame,
    evaluate, face_frame, vertex_position,
};
use geom_core::Tol;

fn len(v: f64) -> Expr {
    Expr::literal(v, Dimension::Length).expect("a length literal")
}

fn eval(doc: &ProfileDoc) -> editor_core::Evaluation<f64> {
    evaluate::<f64>(
        doc,
        None,
        &CancelToken::new(),
        &EvalOptions::default(),
        Tol::witness(),
    )
}

/// A unit box as an extruded square, and its extrude node.
fn box_doc() -> (ProfileDoc, RecipeNodeId) {
    let (doc, p) = fixture::on_frame(
        ProfileDoc::empty_derived("lib_u5_interrogate", Tol::witness()),
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)]],
    );
    fixture::insert(
        doc,
        Node::Extrude {
            profile: p,
            distance: len(1.0),
        },
    )
}

/// **`edge_frame` answers, and a straight edge answers honestly.**
///
/// Every edge of a box is a line: a direction and NO distinguished
/// perpendicular. The door reports `u_ref: None` rather than
/// inventing one, and `v_ref()` follows it.
#[test]
fn edge_frame_reads_every_line_carrier_and_declines_to_invent_a_perpendicular() {
    let (doc, node) = box_doc();
    let ev = eval(&doc);
    let edges = all_edges(&ev, node);
    assert_eq!(edges.len(), 12);
    for name in &edges {
        let pose = edge_frame(&ev, node, name).expect("a certified line carrier");
        assert!(
            pose.u_ref.is_none() && pose.v_ref().is_none(),
            "a line fixes no reference perpendicular"
        );
        // The direction is a unit vector — the carrier's own, copied.
        let n = pose.axis;
        assert!((n.x.abs() + n.y.abs() + n.z.abs() - 1.0).abs() < 1e-12);
    }
}

/// **The doors are kind-checked, and a whole body is its own fact.**
#[test]
fn the_doors_refuse_the_wrong_kind_and_name_a_whole_body_separately() {
    let (doc, node) = box_doc();
    let ev = eval(&doc);
    let face = all_faces(&ev, node)[0].clone();
    let edge = all_edges(&ev, node)[0].clone();
    let vertex = all_vertices(&ev, node)[0].clone();

    assert!(matches!(
        face_frame(&ev, node, &edge),
        Err(InterrogateError::WrongKind {
            wanted: EntityKind::Face,
            found: EntityKind::Edge
        })
    ));
    assert!(matches!(
        edge_frame(&ev, node, &vertex),
        Err(InterrogateError::WrongKind {
            wanted: EntityKind::Edge,
            found: EntityKind::Vertex
        })
    ));
    assert!(matches!(
        vertex_position(&ev, node, &face),
        Err(InterrogateError::WrongKind {
            wanted: EntityKind::Vertex,
            found: EntityKind::Face
        })
    ));

    // A whole body has no single frame — a DIFFERENT fact from "you
    // asked for the wrong kind of entity", so it gets its own rung.
    let body = editor_core::all_bodies(&ev, node)[0].clone();
    assert!(matches!(
        face_frame(&ev, node, &body),
        Err(InterrogateError::WholeBody)
    ));
}

/// **A name nothing answers to is `NoSuchName`, not a panic and not
/// a zero value** — the stale-selection case, which is what happens
/// normally after an upstream edit.
#[test]
fn an_unknown_name_refuses_typed() {
    let (doc, node) = box_doc();
    let ev = eval(&doc);
    // Well-formed, and not in this node's table: `OutputBody` is a
    // BODY's role segment, so no FACE ever answers to this name.
    let stranger = StableName {
        kind: EntityKind::Face,
        node,
        path: vec![RoleSeg::OutputBody],
    };
    assert_eq!(
        face_frame(&ev, node, &stranger).unwrap_err(),
        InterrogateError::NoSuchName
    );
    assert_eq!(
        denotation(&ev, node, &stranger).unwrap_err(),
        InterrogateError::NoSuchName
    );
}

/// **A node with no result in this evaluation is `NodeNotEvaluated`**
/// — distinguishable from "the node evaluated and has no such name",
/// which is the distinction a caller recovers differently from.
#[test]
fn a_foreign_node_id_refuses_typed_and_differs_from_an_unknown_name() {
    let (doc, node) = box_doc();
    let ev = eval(&doc);
    let name = all_faces(&ev, node)[0].clone();
    let foreign = RecipeNodeId(4242);

    assert_eq!(
        face_frame(&ev, foreign, &name).unwrap_err(),
        InterrogateError::NodeNotEvaluated { node: foreign }
    );
    assert_eq!(
        denotation(&ev, foreign, &name).unwrap_err(),
        InterrogateError::NodeNotEvaluated { node: foreign }
    );
    // The two failures are NOT the same value: the ladder's rungs stay
    // apart.
    assert_ne!(
        face_frame(&ev, foreign, &name).unwrap_err(),
        InterrogateError::NoSuchName
    );
}

/// **`denotation` agrees with the doors**: every name the
/// materializers hand back resolves uniquely, and the geometry doors
/// answer for exactly those. (A tie would refuse `Ambiguous`; this
/// corpus mints none, which is itself worth pinning — the tie path is
/// N2's, not a routine outcome.)
#[test]
fn every_materialized_name_denotes_uniquely_and_answers() {
    let (doc, node) = box_doc();
    let ev = eval(&doc);
    for name in all_faces(&ev, node) {
        assert_eq!(
            denotation(&ev, node, &name),
            Ok(editor_core::Denotation::Unique)
        );
        assert!(face_frame(&ev, node, &name).is_ok());
    }
    for name in all_vertices(&ev, node) {
        assert!(vertex_position(&ev, node, &name).is_ok());
    }
}

/// **A read door asks what a name denotes before it asks how many
/// entities answer to it** — PORT-DOORS-1's rule
/// (`assembly::resolve_face`) at the body the five read doors share.
///
/// A face name handed to `edge_frame` is unreadable there however few
/// entities answer to it, so narrowing is no recourse and `WrongKind`
/// is the whole fault. Before the order changed, the answer depended
/// on whether the name happened to be tied: a unique face name got
/// `WrongKind`, a tied one got `Ambiguous` and an instruction to
/// narrow. The rows below pin that the two now agree, and that the
/// tie still refuses at the door that DOES read faces.
#[test]
fn a_read_door_refuses_a_tied_name_of_another_kind_by_its_kind() {
    use editor_core::Entry;

    // The symmetric U cutter's N2 tie.
    let (doc, sub) = fixture::u_cutter_tie(ProfileDoc::empty_derived(
        "lib_u5_interrogate_tie",
        Tol::witness(),
    ));
    let ev = eval(&doc);
    let table = &ev.value(sub).expect("the U subtract evaluates").name_table;
    let tied: StableName = table
        .iter()
        .find_map(|(n, e)| {
            (n.kind == EntityKind::Face && matches!(e, Entry::Tied(_))).then(|| n.clone())
        })
        .expect("the U fixture ties a face");
    let unique: StableName = table
        .iter()
        .find_map(|(n, e)| {
            (n.kind == EntityKind::Face && matches!(e, Entry::Unique(_))).then(|| n.clone())
        })
        .expect("the U subtract names a unique face");

    // The tie is REAL and the door's own kind is the one the name
    // does NOT denote: without both, the row below passes vacuously.
    let candidates = match denotation(&ev, sub, &tied) {
        Ok(editor_core::Denotation::Tied { candidates }) => candidates,
        other => panic!("the declared name must really be tied, got {other:?}"),
    };
    assert!(candidates >= 2, "a tie is two or more candidates");

    let wrong_kind = InterrogateError::WrongKind {
        wanted: EntityKind::Edge,
        found: EntityKind::Face,
    };
    assert_eq!(
        edge_frame(&ev, sub, &tied).err(),
        Some(wrong_kind),
        "an edge door handed a FACE name is not a door that must pick one — the tie is \
         not the fault and narrowing is no recourse"
    );
    assert_eq!(
        edge_frame(&ev, sub, &unique).err(),
        Some(wrong_kind),
        "and the unique name of the same kind gets the same word"
    );
    // The tie still refuses where the kind DOES match: this changes
    // the order of two questions, not whether a tie is referenceable.
    assert_eq!(
        face_frame(&ev, sub, &tied).err(),
        Some(InterrogateError::Ambiguous { candidates }),
        "a tied FACE name at the face door is still the N2 tie"
    );
}
