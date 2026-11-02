//! **A selection's entity kind is refused at the doors that read what
//! the author wrote, never at evaluation.**
//!
//! What a name denotes is its own `kind`, and the name table makes every
//! entity answering to it agree, so a selection's kind is a fact about
//! the document. The edit door (`EditError::SelectionNameKind`, from
//! `Select::fault`'s per-name arm held to the seat's kind) and the load
//! door (`SnapshotError::SelectionShape`) hold a selection of kind K to
//! names of kind K; evaluation then resolves into keys of that kind and
//! has nothing left to say about it.
//!
//! The rows pin each reader of a selection a name can be authored at —
//! a shell's open faces and a blend's edges — and
//! under each fixed reader vary what was named across both wrong kinds,
//! so a door that answered a constant kind goes red. Two rows pin the
//! position: the first name of another kind is the one reported,
//! wherever in the list it sits. A saved document holding one refuses
//! at load. The tied name of the wrong kind, which refuses its kind
//! rather than its tie, is `m6_5_selection_refusals.rs`'s row, beside
//! the tie it is told apart from.
//!
//! The measure row is FORK-VTX's (`EditError::SlotVarKind`, the seat's
//! kind refusing the singleton the reference mints), which this suite
//! did not move: it is kept here as the remaining reader's door row, so
//! every reader of a selection has one in this file.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;

use editor_core::{
    CapEnd, EditError, EntityKind, Node, OperandSlot, PersistError, ProfileDoc, RecipeNodeId,
    SelectionFault, SnapshotError, StableName, load, save,
};
use fixture::{fname, insert, insert_refused, len, on_frame, square, wall};
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
        },
    );
    let face = fname(body, wall(&doc, body, 2));
    let edge = fixture::prism_edges(&doc, body, 4).remove(2);
    let vertex = end_cap_vertex(&doc, body, 0);
    (doc, body, face, edge, vertex)
}

/// The refusal an insert gave a name of another kind: the seat, the
/// name's position, what it denotes and what the seat reads.
fn name_kind(err: &EditError) -> (OperandSlot, usize, EntityKind, EntityKind) {
    match err {
        EditError::SelectionNameKind {
            slot: editor_core::SlotId::Operand(slot),
            at,
            found,
            expected,
            ..
        } => (*slot, *at, *found, *expected),
        other => panic!("expected a selection name-kind refusal, got {other:?}"),
    }
}

/// **The sentence an author reads**, with its node and its name spoken
/// from the refusal itself: the reader's position, the name, what it is
/// and what the seat reads, then the recourse naming the seat's kind.
fn sentence(err: &EditError, entry: &str) -> String {
    let EditError::SelectionNameKind {
        node,
        name,
        found,
        expected,
        ..
    } = err
    else {
        panic!("expected a selection name-kind refusal, got {err:?}")
    };
    format!(
        "{node}'s {entry} names {name}, which is {}, not {}. Recourse: name {} of the body the \
         selection reads",
        an(*found),
        an(*expected),
        an(*expected)
    )
}

/// One entity of `kind`, as the sentence says it — written out here
/// rather than borrowed from the crate, so the pin is a second reading.
fn an(kind: EntityKind) -> &'static str {
    match kind {
        EntityKind::Body => "a body",
        EntityKind::Face => "a face",
        EntityKind::Edge => "an edge",
        EntityKind::Vertex => "a vertex",
    }
}

/// **The shell's open designation names a FACE.** Two rows under one
/// reader: what was designated varies, so the pair fails a door that
/// answered a constant kind.
#[test]
fn a_shell_designation_of_another_kind_refuses_at_the_insert() {
    for (what, designate) in [
        ("an edge", EntityKind::Edge),
        ("a vertex", EntityKind::Vertex),
    ] {
        let (doc, body, _, edge, vertex) = solid();
        let name = if designate == EntityKind::Edge {
            edge
        } else {
            vertex
        };
        let err = insert_refused(&doc, Node::shell(body, len(0.1), vec![name]));
        assert_eq!(
            name_kind(&err),
            (OperandSlot::Open, 0, designate, EntityKind::Face),
            "{what}"
        );
        assert_eq!(err.to_string(), sentence(&err, "open face 0"), "{what}");
    }
}

/// **A blend's selection names an EDGE**: a face or a vertex refuses at
/// the insert, under each blend's own seat, and the sentence speaks the
/// face it was handed.
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
        for designate in [EntityKind::Face, EntityKind::Vertex] {
            let (doc, body, face, _, vertex) = solid();
            let name = if designate == EntityKind::Face {
                face
            } else {
                vertex
            };
            let err = insert_refused(&doc, node(body, len(0.1), vec![name]));
            assert_eq!(
                name_kind(&err),
                (OperandSlot::Selection, 0, designate, EntityKind::Edge),
                "{what} over {designate:?}"
            );
            assert_eq!(
                err.to_string(),
                sentence(&err, "edge 0"),
                "{what} over {designate:?}"
            );
        }
        // Byte-pinned once, so the spoken forms the helper reads off the
        // refusal are pinned too and not only agreed with.
        let (doc, body, face, _, _) = solid();
        let err = insert_refused(&doc, node(body, len(0.1), vec![face]));
        let EditError::SelectionNameKind { node: blend, .. } = &err else {
            panic!("{what}: {err:?}")
        };
        let (blend, body) = (
            test_utils::refusal::tag(blend.id().0.digest()),
            test_utils::refusal::tag(body.0.digest()),
        );
        let verb = if what == "fillet" {
            "Fillet"
        } else {
            "Chamfer"
        };
        assert_eq!(
            err.to_string(),
            format!(
                "{verb} {blend}'s edge 0 names the side wall over loop 0 step 3 of Extrude {body}, \
                 which is a face, not an edge. Recourse: name an edge of the body the selection \
                 reads"
            ),
            "{what}"
        );
    }
}

/// **The first name of another kind is the one reported, wherever it
/// sits**: the seat decides the selection's kind, not its first name,
/// so a face before an edge refuses at 0 and a face after one at 1.
#[test]
fn a_set_holding_one_name_of_another_kind_refuses_at_its_place() {
    for (face_first, at) in [(false, 1), (true, 0)] {
        let (doc, body, face, edge, _) = solid();
        let names = if face_first {
            vec![face, edge]
        } else {
            vec![edge, face]
        };
        let err = insert_refused(
            &doc,
            Node::Fillet {
                radius: len(0.1),
                selection: editor_core::Operand::select(body, names),
            },
        );
        assert_eq!(
            name_kind(&err),
            (
                OperandSlot::Selection,
                at,
                EntityKind::Face,
                EntityKind::Edge
            ),
            "the face at {at}"
        );
        assert_eq!(err.to_string(), sentence(&err, &format!("edge {at}")));
    }
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

/// **A clearance seat's kind is the door's, not evaluation's**
/// (FORK-VTX): a clearance reads a body or a face, and an edge or a
/// vertex refuses at insert as the seat's kind, before any evaluation
/// could ask what a name denotes. Two rows, one per found kind, so a seat
/// that admitted either goes red.
#[test]
fn a_clearance_seat_of_another_kind_refuses_at_the_door() {
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
        let err = crate::fixture::insert_refused(
            &doc,
            Node::clearance(
                crate::fixture::sited_ref(name),
                crate::fixture::sited_ref(face),
                editor_core::ClearanceBounds::AtLeast(crate::fixture::len(0.1)),
            ),
        );
        assert!(
            matches!(
                &err,
                editor_core::EditError::SlotVarKind {
                    found: f,
                    expected: editor_core::SlotKind::Cleared,
                    ..
                } if *f == found
            ),
            "{what}: the seat's kind refusal: {err:?}"
        );
    }
}
