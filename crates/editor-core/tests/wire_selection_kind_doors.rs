//! **A selection's entity kind is refused at the doors that read what
//! the author wrote, never at evaluation.**
//!
//! What a name denotes is its own `kind`, and the name table makes every
//! entity answering to it agree, so a selection's kind is a fact about
//! the document. The edit door (the seat's selection kind, and
//! `Select::fault`'s per-name arm) and the load door
//! (`SnapshotError::SelectionShape`) hold a selection of kind K to names
//! of kind K; evaluation then resolves into keys of that kind and has
//! nothing left to say about it.
//!
//! The rows pin each reader of a selection — a shell's open faces, a
//! blend's edges, a derived frame's face, a measure's reference — and
//! vary what was named under a fixed reader, so a door that answered a
//! constant kind goes red. Two more rows pin what moved with the
//! question: a TIED name of the wrong kind refuses its kind rather than
//! its tie, and a saved document holding one refuses at load.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;
use editor_core::ExtrudeSide;

use editor_core::measure::MeasurePrimitive;
use editor_core::{
    CapEnd, Datum, EditError, EntityKind, Node, OperandSlot, PersistError, ProfileDoc,
    RecipeNodeId, SelectionFault, SitedRef, SnapshotError, StableName, load, save,
};
use fixture::{ang, fname, insert, insert_refused, len, on_frame, square, wall};
use geom_core::Tol;
/// A vertex name at `node` — the extrude's own END cap vertex on the
/// document's one outer loop, so the name RESOLVES and the refusal is
/// about its kind rather than about a name that names nothing.
fn end_cap_vertex(doc: &editor_core::ProfileDoc, node: RecipeNodeId, vertex: u32) -> StableName {
    fixture::cap_vertex(
        node,
        CapEnd::End,
        crate::fixture::vpiece(doc, node, 0, vertex as usize),
    )
}

/// A square prism and the four names every row below miswires with: a
/// face of it, one of its lateral edges, one of its cap vertices.
fn solid() -> (ProfileDoc, RecipeNodeId, StableName, StableName, StableName) {
    let doc = ProfileDoc::empty_derived("wire_selection_kind_doors", Tol::witness());
    let (doc, profile) = on_frame(
        doc,
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![square(0.0, 0.0, 1.0)],
    );
    let (doc, body) = insert(
        doc,
        Node::Extrude {
            profile: profile.into(),
            distance: len(1.0),
            side: ExtrudeSide::Along,
        },
    );
    let face = fname(body, wall(&doc, body, 2));
    let edge = fixture::prism_edges(&doc, body, 4).remove(2);
    let vertex = end_cap_vertex(&doc, body, 0);
    (doc, body, face, edge, vertex)
}

/// The seat and fault an insert refused a selection with.
fn shape(err: &EditError) -> (OperandSlot, &SelectionFault) {
    match err {
        EditError::SelectionShape {
            slot: editor_core::SlotId::Operand(slot),
            fault,
            ..
        } => (*slot, fault),
        other => panic!("expected a selection-shape refusal, got {other:?}"),
    }
}

/// **The shell's open designation names a FACE.** Two rows under one
/// reader: what was designated varies, so the pair fails a door that
/// answered a constant kind.
#[test]
fn a_shell_designation_of_another_kind_refuses_at_the_insert() {
    for (what, designate, said) in [
        (
            "an edge",
            EntityKind::Edge,
            "the seat reads no selection of an edge",
        ),
        (
            "a vertex",
            EntityKind::Vertex,
            "the seat reads no selection of a vertex",
        ),
    ] {
        let (doc, body, _, edge, vertex) = solid();
        let name = if designate == EntityKind::Edge {
            edge
        } else {
            vertex
        };
        let err = insert_refused(&doc, Node::shell(body, len(0.1), vec![name]));
        let (slot, fault) = shape(&err);
        assert_eq!(slot, OperandSlot::Open, "{what}: the shell's open faces");
        assert_eq!(*fault, SelectionFault::Seat { entity: designate }, "{what}");
        assert_eq!(fault.to_string(), said, "{what}");
    }
}

/// **A blend's selection names an EDGE**: a face refuses at the insert,
/// under the blend's own seat.
#[test]
fn a_blend_selection_of_another_kind_refuses_at_the_insert() {
    for (what, node) in [
        (
            "fillet",
            Node::fillet as fn(RecipeNodeId, editor_core::Formula, Vec<StableName>) -> _,
        ),
        (
            "chamfer",
            Node::chamfer as fn(RecipeNodeId, editor_core::Formula, Vec<StableName>) -> _,
        ),
    ] {
        let (doc, body, face, _, _) = solid();
        let err = insert_refused(&doc, node(body, len(0.1), vec![face]));
        let (slot, fault) = shape(&err);
        assert_eq!(slot, OperandSlot::Selection, "{what}: the blend's edges");
        assert_eq!(
            *fault,
            SelectionFault::Seat {
                entity: EntityKind::Face
            },
            "{what}"
        );
        assert_eq!(
            fault.to_string(),
            "the seat reads no selection of a face",
            "{what}"
        );
        let said = err.to_string();
        assert!(
            said.contains("the seat reads no selection of a face")
                && said.contains("name entities of the kind the seat reads"),
            "{what}: the sentence an author reads carries the fault and its recourse: {said}"
        );
    }
}

/// **A name of another kind AFTER the first** is the per-name arm's:
/// the first name decides the selection's kind, and every other is held
/// to it, so an edge set holding a face refuses at the face's place.
#[test]
fn a_set_holding_one_name_of_another_kind_refuses_at_its_place() {
    let (doc, body, face, edge, _) = solid();
    let err = insert_refused(
        &doc,
        Node::Fillet {
            radius: len(0.1),
            selection: editor_core::Operand::select(body, vec![edge, face]),
        },
    );
    let (slot, fault) = shape(&err);
    assert_eq!(slot, OperandSlot::Selection);
    assert_eq!(
        *fault,
        SelectionFault::Kind {
            at: 1,
            found: EntityKind::Face,
            expected: EntityKind::Edge,
        }
    );
    assert_eq!(fault.to_string(), "name 1 is a face, not an edge");
}

/// **A derived frame's face name names a FACE** — the same wanted kind
/// as the shell's row at another seat.
#[test]
fn a_derived_frame_named_on_another_kind_refuses_at_the_insert() {
    let (doc, body, _, edge, _) = solid();
    let err = insert_refused(
        &doc,
        Node::Datum(Datum::FaceFrame {
            face: editor_core::Operand::select(body, vec![edge]),
            spin: ang(0.0),
        }),
    );
    let (slot, fault) = shape(&err);
    assert_eq!(slot, OperandSlot::Face);
    assert_eq!(
        *fault,
        SelectionFault::Seat {
            entity: EntityKind::Edge
        }
    );
}

/// **A saved selection holding a name of another kind refuses at load**,
/// with the per-name arm the insert door asks: a fillet's one edge
/// re-spelled a face on the wire.
#[test]
fn a_saved_selection_of_another_kind_refuses_at_load() {
    let (doc, body, _, edge, _) = solid();
    let (doc, _) = insert(doc, Node::fillet(body, len(0.1), vec![edge]));
    let text = save(&doc, &[], Tol::witness()).expect("the fixture saves");
    load(&text, Tol::witness()).expect("the uncorrupted fixture loads");
    let sel = text
        .find("\"Select\"")
        .expect("the selection reaches the wire");
    let needle = "\"kind\": \"Edge\"";
    let at = sel
        + text[sel..]
            .find(needle)
            .expect("the selection names an edge");
    let corrupt = format!(
        "{}\"kind\": \"Face\"{}",
        &text[..at],
        &text[at + needle.len()..]
    );
    match load(&corrupt, Tol::witness()) {
        Err(PersistError::Snapshot(SnapshotError::SelectionShape { fault, .. })) => assert_eq!(
            fault,
            SelectionFault::Kind {
                at: 0,
                found: EntityKind::Face,
                expected: EntityKind::Edge,
            }
        ),
        other => panic!("a saved selection of another kind must refuse at load, got {other:?}"),
    }
}

/// **A measure's reference kind is the door's, not evaluation's**
/// (FORK-VTX): a `min_clearance` reads a body or a face, and an edge or
/// a vertex refuses at insert as the seat's kind, before any evaluation
/// could ask what a name denotes. Two rows, one per found kind, so a seat
/// that admitted either goes red.
#[test]
fn a_measure_reference_of_another_kind_refuses_at_the_door() {
    for (what, select, found) in [
        ("an edge", EntityKind::Edge, editor_core::VarKind::Edge),
        ("a vertex", EntityKind::Vertex, editor_core::VarKind::Vertex),
    ] {
        let (doc, _, face, edge, vertex) = solid();
        let name = if select == EntityKind::Edge {
            edge
        } else {
            vertex
        };
        let refused = editor_core::measure(
            &doc,
            &[MeasurePrimitive::MinClearance {
                a: SitedRef::at_mint(name),
                b: SitedRef::at_mint(face),
            }],
            Tol::witness(),
            &editor_core::RefusingReach,
        );
        let Err(err) = refused else {
            panic!("{what}: the insert refuses");
        };
        assert!(
            matches!(
                &err,
                editor_core::EditError::SlotVarKind {
                    found: f,
                    expected: editor_core::SlotKind::Measured(editor_core::MeasureVerb::MinClearance),
                    ..
                } if *f == found
            ),
            "{what}: the seat's kind refusal: {err:?}"
        );
    }
}
