//! **A blend's canonical selection is one rule with two doors.**
//!
//! `Node::Fillet` and `Node::Chamfer` store their selection sorted and
//! deduplicated, so two recipes that pick the same edges are
//! bit-identical. `Node::fillet`/`Node::chamfer` are the construction
//! doors that put it in that form; the variants are public, so a
//! hand-built one can be handed to `DocEdit::InsertNode` and a corrupt
//! file can be handed to `load`. Both doors ask the one predicate
//! (`Node::input_fault`) and refuse alike, never repairing — a repair
//! would move the node's content key behind the caller's back.
//!
//! Each door gets its own row, so dropping the predicate at either one
//! is caught by that door's row rather than by its twin.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;

use editor_core::{
    CapEnd, DocEdit, EditError, EntityKind, InputFault, Node, PersistError, ProfileDoc,
    ProfileEdgeRef, ProfileProgram, RecipeNodeId, RoleSeg, SnapshotError, StableName, apply, load,
    save,
};
use geom_core::Tol;

/// A square prism: an XY frame (0), a unit-square profile (1) and the
/// extrude (2) whose edges the blends name.
fn prism() -> (ProfileDoc, RecipeNodeId) {
    let mut r = fixture::Recorder::new();
    let profile = r.profile(
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![fixture::square(0.0, 0.0, 0.5)],
    );
    let solid = r.insert(Node::Extrude {
        profile,
        distance: fixture::len(1.0),
    });
    (r.doc, solid)
}

/// One END-cap rim edge of the prism, by profile segment — the names
/// sort by segment, so `edge(n, 0) < edge(n, 2)`.
fn edge(node: RecipeNodeId, segment: u32) -> StableName {
    StableName {
        kind: EntityKind::Edge,
        node,
        path: vec![RoleSeg::RimEdge(
            CapEnd::End,
            ProfileEdgeRef {
                loop_index: 0,
                segment,
            },
        )],
    }
}

/// The saved text of a prism carrying one canonical two-edge fillet,
/// plus the two segment spellings that reach the wire in that order.
fn saved_fillet() -> String {
    let (doc, solid) = prism();
    let doc = apply(
        &doc,
        &DocEdit::InsertNode {
            node: Node::fillet(
                solid,
                fixture::len(0.0625),
                vec![edge(solid, 0), edge(solid, 2)],
            ),
        },
        Tol::witness(),
    )
    .expect("a canonical fillet inserts")
    .doc;
    save(&doc, &[], Tol::witness()).expect("the fixture saves")
}

/// Rewrites the FIRST `"segment": <from>` at or after the `"selection"`
/// key to `<to>`, leaving the rest of the document alone: the selection
/// is the only list this suite corrupts, and the caller reads the
/// refusal that comes back.
fn corrupt_selection(text: &str, from: u32, to: u32) -> String {
    let sel = text
        .find("\"selection\"")
        .expect("the selection reaches the wire");
    let needle = format!("\"segment\": {from}");
    let at = sel
        + text[sel..]
            .find(&needle)
            .expect("the selection names the segment");
    format!(
        "{}\"segment\": {to}{}",
        &text[..at],
        &text[at + needle.len()..]
    )
}

/// **The insert door refuses an UNSORTED selection**, naming the
/// position at which the order breaks. The variant is public, so this
/// is a node the construction door would never have produced reaching
/// the document's one authoring door.
#[test]
fn an_unsorted_selection_is_refused_at_the_insert_door() {
    let (doc, solid) = prism();
    let raw: Node<ProfileProgram> = Node::Fillet {
        target: solid,
        radius: fixture::len(0.0625),
        selection: vec![edge(solid, 2), edge(solid, 0)],
    };
    match apply(&doc, &DocEdit::InsertNode { node: raw }, Tol::witness()) {
        Err(EditError::SelectionNotCanonical { at: 0, .. }) => {}
        other => panic!("an unsorted selection must refuse typed, got {other:?}"),
    }
    // Sorted, the same two names are accepted — so the refusal above is
    // the order's and not the fixture's.
    let raw: Node<ProfileProgram> = Node::Fillet {
        target: solid,
        radius: fixture::len(0.0625),
        selection: vec![edge(solid, 0), edge(solid, 2)],
    };
    apply(&doc, &DocEdit::InsertNode { node: raw }, Tol::witness())
        .expect("a canonical selection inserts");
}

/// **The chamfer's twin**: the rule reads the selection, not the blend,
/// so the other variant refuses at the same door with the same fault.
#[test]
fn an_unsorted_chamfer_selection_is_refused_at_the_insert_door() {
    let (doc, solid) = prism();
    let raw: Node<ProfileProgram> = Node::Chamfer {
        target: solid,
        distance: fixture::len(0.0625),
        selection: vec![edge(solid, 2), edge(solid, 0)],
    };
    match apply(&doc, &DocEdit::InsertNode { node: raw }, Tol::witness()) {
        Err(EditError::SelectionNotCanonical { at: 0, .. }) => {}
        other => panic!("an unsorted chamfer selection must refuse typed, got {other:?}"),
    }
}

/// **A REPEATED entry is refused at the insert door too**: "canonical"
/// is one predicate — strictly increasing — and a repeat breaks it at
/// the position where the two equal entries meet.
#[test]
fn a_repeated_selection_entry_is_refused_at_the_insert_door() {
    let (doc, solid) = prism();
    let raw: Node<ProfileProgram> = Node::Fillet {
        target: solid,
        radius: fixture::len(0.0625),
        selection: vec![edge(solid, 0), edge(solid, 0), edge(solid, 2)],
    };
    match apply(&doc, &DocEdit::InsertNode { node: raw }, Tol::witness()) {
        Err(EditError::SelectionNotCanonical { at: 0, .. }) => {}
        other => panic!("a repeated selection entry must refuse typed, got {other:?}"),
    }
}

/// **The load door refuses the same document with the same fault**,
/// through `SnapshotError::InputList` — the arm every other
/// `Node::input_fault` answer already loads through, so the blend's
/// canonical form has no refusal of its own any more.
#[test]
fn an_unsorted_selection_is_refused_at_the_load_door() {
    let text = saved_fillet();
    // The uncorrupted text loads, so the refusal below is the surgery's
    // and not the fixture's.
    load(&text, Tol::witness()).expect("the canonical fixture loads");
    // `[seg 0, seg 2]` → `[seg 9, seg 2]`: still two distinct names, no
    // longer sorted.
    let corrupt = corrupt_selection(&text, 0, 9);
    match load(&corrupt, Tol::witness()) {
        Err(PersistError::Snapshot(SnapshotError::InputList {
            fault: InputFault::SelectionNotCanonical { at: 0 },
            ..
        })) => {}
        other => panic!("an unsorted selection must refuse typed at load, got {other:?}"),
    }
}

/// **A repeat is refused at the load door too** — the second half of
/// "a duplicate is refused at both doors".
#[test]
fn a_repeated_selection_entry_is_refused_at_the_load_door() {
    let text = saved_fillet();
    // `[seg 0, seg 2]` → `[seg 0, seg 0]`.
    let corrupt = corrupt_selection(&text, 2, 0);
    match load(&corrupt, Tol::witness()) {
        Err(PersistError::Snapshot(SnapshotError::InputList {
            fault: InputFault::SelectionNotCanonical { at: 0 },
            ..
        })) => {}
        other => panic!("a repeated selection must refuse typed at load, got {other:?}"),
    }
}

/// **The construction doors still canonicalize**: the refusals above
/// are what a caller who bypassed these doors gets, not a new burden on
/// the caller who uses them.
#[test]
fn the_construction_doors_canonicalize() {
    let solid = RecipeNodeId(2);
    let unruly = vec![edge(solid, 2), edge(solid, 0), edge(solid, 2)];
    let canonical = vec![edge(solid, 0), edge(solid, 2)];

    let fillet: Node<ProfileProgram> = Node::fillet(solid, fixture::len(0.0625), unruly.clone());
    let Node::Fillet { selection, .. } = &fillet else {
        panic!("the door builds a fillet")
    };
    assert_eq!(selection, &canonical, "sorted and deduplicated");
    assert!(fillet.input_fault().is_none(), "and therefore canonical");

    let chamfer: Node<ProfileProgram> = Node::chamfer(solid, fixture::len(0.0625), unruly);
    let Node::Chamfer { selection, .. } = &chamfer else {
        panic!("the door builds a chamfer")
    };
    assert_eq!(selection, &canonical, "sorted and deduplicated");
    assert!(chamfer.input_fault().is_none(), "and therefore canonical");
}

/// **An EMPTY selection is canonical**, and stays a different refusal:
/// a blend of nothing is an unfinished recipe, which evaluation names
/// (`NodeErrorKind::BlendSelectionEmpty`), not a corrupt form the
/// authoring door can diagnose.
#[test]
fn an_empty_selection_is_canonical() {
    let (doc, solid) = prism();
    let empty: Node<ProfileProgram> = Node::Fillet {
        target: solid,
        radius: fixture::len(0.0625),
        selection: Vec::new(),
    };
    assert!(empty.input_fault().is_none());
    apply(&doc, &DocEdit::InsertNode { node: empty }, Tol::witness())
        .expect("an empty selection is not this door's refusal");
}

/// The refusal's PROSE, at both doors: the load door forwards the
/// fault's own sentence and the edit door frames it, so the two say the
/// same thing about the same document and neither invents a second
/// vocabulary for it.
#[test]
fn both_doors_forward_one_sentence() {
    let fault = InputFault::SelectionNotCanonical { at: 3 };
    let said = fault.to_string();
    assert!(said.contains("entry 3"), "it names the position: {said}");

    let at_load = SnapshotError::InputList {
        node: RecipeNodeId(7),
        fault,
    }
    .to_string();
    assert!(at_load.contains(&said), "the load door forwards: {at_load}");

    let at_edit = EditError::SelectionNotCanonical {
        node: RecipeNodeId(7),
        at: 3,
    }
    .to_string();
    assert!(at_edit.contains(&said), "the edit door forwards: {at_edit}");
}
