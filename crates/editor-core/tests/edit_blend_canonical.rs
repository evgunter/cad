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
    CancelToken, CapEnd, DocEdit, EditError, EntityKind, EvalOptions, InputFault, Node,
    NodeErrorKind, NodeResult, PersistError, ProfileDoc, ProfileEdgeRef, ProfileProgram,
    RecipeNodeId, RoleSeg, SnapshotError, StableName, apply, evaluate, load, save,
};
use geom_core::Tol;
use sweep::blend::BlendKind;

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

/// A hand-built `Node::Fillet` over the prism's END-cap rims, by
/// profile segment — the shape `Node::fillet` would have
/// canonicalized, handed to a door raw.
fn raw_fillet(solid: RecipeNodeId, segments: &[u32]) -> Node<ProfileProgram> {
    Node::Fillet {
        target: solid,
        radius: fixture::len(0.0625),
        selection: segments.iter().map(|s| edge(solid, *s)).collect(),
    }
}

/// The saved text of a prism carrying one canonical fillet over the
/// named segments, which reach the wire in that order.
fn saved_fillet(segments: &[u32]) -> String {
    let (doc, solid) = prism();
    let doc = apply(
        &doc,
        &DocEdit::InsertNode {
            node: Node::fillet(
                solid,
                fixture::len(0.0625),
                segments.iter().map(|s| edge(solid, *s)).collect(),
            ),
        },
        Tol::witness(),
        &editor_core::RefusingReach,
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
    let raw = raw_fillet(solid, &[2, 0]);
    match apply(
        &doc,
        &DocEdit::InsertNode { node: raw },
        Tol::witness(),
        &editor_core::RefusingReach,
    ) {
        Err(EditError::SelectionNotCanonical { at: 0, .. }) => {}
        other => panic!("an unsorted selection must refuse typed, got {other:?}"),
    }
    // Sorted, the same two names are accepted — so the refusal above is
    // the order's and not the fixture's.
    apply(
        &doc,
        &DocEdit::InsertNode {
            node: raw_fillet(solid, &[0, 2]),
        },
        Tol::witness(),
        &editor_core::RefusingReach,
    )
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
    match apply(
        &doc,
        &DocEdit::InsertNode { node: raw },
        Tol::witness(),
        &editor_core::RefusingReach,
    ) {
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
    let raw = raw_fillet(solid, &[0, 0, 2]);
    match apply(
        &doc,
        &DocEdit::InsertNode { node: raw },
        Tol::witness(),
        &editor_core::RefusingReach,
    ) {
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
    let text = saved_fillet(&[0, 2]);
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
    let text = saved_fillet(&[0, 2]);
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
/// a blend of nothing is an unfinished recipe, which EVALUATION names,
/// not a corrupt form the authoring door can diagnose. The row admits
/// it at the insert door and then takes the refusal it does get, so
/// the sentence in `input_fault`'s comment is evidence and not a
/// claim.
#[test]
fn an_empty_selection_is_canonical() {
    let (doc, solid) = prism();
    let empty: Node<ProfileProgram> = Node::Fillet {
        target: solid,
        radius: fixture::len(0.0625),
        selection: Vec::new(),
    };
    assert!(empty.input_fault().is_none());
    let doc = apply(
        &doc,
        &DocEdit::InsertNode { node: empty },
        Tol::witness(),
        &editor_core::RefusingReach,
    )
    .expect("an empty selection is not this door's refusal")
    .doc;
    let fillet = *doc.order().last().expect("the fillet is the last node");
    let ev = evaluate::<f64>(
        &doc,
        None,
        &CancelToken::new(),
        &EvalOptions::default(),
        Tol::witness(),
    );
    match ev.nodes.get(&fillet) {
        Some(NodeResult::Failed(e)) => assert!(
            matches!(
                e.kind,
                NodeErrorKind::BlendSelectionEmpty {
                    verb: BlendKind::Fillet
                }
            ),
            "the empty blend refuses at evaluation, not at a door: {:?}",
            e.kind
        ),
        other => panic!("expected an evaluation refusal, got {other:?}"),
    }
}

/// The refusal's PROSE **as each door actually renders it**, over a
/// real node each door actually refuses, at a position that is not
/// zero. A door that forwarded the wrong `at` — its own loop index, a
/// hard-coded zero, the successor's position — reds here, which is
/// what building the fault by hand could not catch.
#[test]
fn both_doors_forward_one_sentence() {
    // `[0, 4, 2]`: the break is between entries 1 and 2.
    let expected = InputFault::SelectionNotCanonical { at: 1 }.to_string();
    assert!(
        expected.contains("entry 1") && expected.contains("entry 2"),
        "the sentence names the entry and its successor: {expected}"
    );

    let (doc, solid) = prism();
    let at_edit = match apply(
        &doc,
        &DocEdit::InsertNode {
            node: raw_fillet(solid, &[0, 4, 2]),
        },
        Tol::witness(),
        &editor_core::RefusingReach,
    ) {
        Err(e @ EditError::SelectionNotCanonical { .. }) => e.to_string(),
        other => panic!("expected the edit door's refusal, got {other:?}"),
    };
    assert!(
        at_edit.contains(&expected),
        "the edit door forwards the fault's own sentence: {at_edit}"
    );

    // The same break, through the load door: `[0, 2, 4]` → `[0, 9, 4]`.
    let text = saved_fillet(&[0, 2, 4]);
    let at_load = match load(&corrupt_selection(&text, 2, 9), Tol::witness()) {
        Err(e @ PersistError::Snapshot(SnapshotError::InputList { .. })) => e.to_string(),
        other => panic!("expected the load door's refusal, got {other:?}"),
    };
    assert!(
        at_load.contains(&expected),
        "the load door forwards the same sentence: {at_load}"
    );
}

// ---------------------------------------------------------------
// Adopted from the style review's probe lane (`review/blend-rv`):
// every row above named `at: 0`, so the computed index was never
// compared against a non-zero expectation and the mutant `at: 0`
// survived the suite. These rows kill it.
// ---------------------------------------------------------------

/// `input_fault` directly, at each position of a three-name selection,
/// and with both faults present at once — the answer is the FIRST
/// break, whichever kind it is.
#[test]
fn at_names_each_position() {
    let solid = RecipeNodeId(2);
    let cases: &[(&str, Vec<u32>, Option<u32>)] = &[
        ("canonical", vec![0, 2, 4], None),
        ("swap at 0", vec![2, 0, 4], Some(0)),
        ("swap at 1", vec![0, 4, 2], Some(1)),
        ("repeat at 0", vec![0, 0, 4], Some(0)),
        ("repeat at 1", vec![0, 2, 2], Some(1)),
        // Both faults present, the repeat LATER than the swap.
        ("swap at 0 + repeat at 2", vec![2, 0, 4, 4], Some(0)),
        // Both faults present, the repeat EARLIER than the swap.
        ("repeat at 0 + swap at 2", vec![0, 0, 4, 2], Some(0)),
    ];
    for (what, segs, want) in cases {
        let got = match raw_fillet(solid, segs).input_fault() {
            Some(InputFault::SelectionNotCanonical { at }) => Some(at),
            None => None,
            other => panic!("{what}: unexpected fault {other:?}"),
        };
        assert_eq!(got, *want, "{what}");
    }
}

/// The same non-zero position, reported by the INSERT door.
#[test]
fn the_insert_door_reports_a_non_zero_position() {
    let (doc, solid) = prism();
    match apply(
        &doc,
        &DocEdit::InsertNode {
            node: raw_fillet(solid, &[0, 4, 2]),
        },
        Tol::witness(),
        &editor_core::RefusingReach,
    ) {
        Err(EditError::SelectionNotCanonical { at, .. }) => {
            assert_eq!(at, 1, "the break is between entries 1 and 2");
        }
        other => panic!("expected a typed refusal, got {other:?}"),
    }
}

/// The same non-zero position, reported by the LOAD door.
#[test]
fn the_load_door_reports_a_non_zero_position() {
    let text = saved_fillet(&[0, 2, 4]);
    load(&text, Tol::witness()).expect("the canonical fixture loads");
    // `[0, 2, 4]` → `[0, 9, 4]`: the break moves to entry 1.
    match load(&corrupt_selection(&text, 2, 9), Tol::witness()) {
        Err(PersistError::Snapshot(SnapshotError::InputList {
            fault: InputFault::SelectionNotCanonical { at },
            ..
        })) => assert_eq!(at, 1, "the load door names the same entry"),
        other => panic!("expected a typed refusal, got {other:?}"),
    }
}

/// **`Rebind` re-establishes the form it repairs.** The rewrite goes
/// through the same canonicalizer the construction doors use, so a
/// rebind onto an already-selected edge shrinks the set by one and
/// what it writes answers `input_fault` with `None` — the repair
/// cannot leave behind a shape a door would refuse.
#[test]
fn a_rebind_leaves_a_canonical_selection() {
    let (doc, solid) = prism();
    let (doc, fillet) = {
        let applied = apply(
            &doc,
            &DocEdit::InsertNode {
                node: Node::fillet(
                    solid,
                    fixture::len(0.0625),
                    vec![edge(solid, 0), edge(solid, 2), edge(solid, 4)],
                ),
            },
            Tol::witness(),
            &editor_core::RefusingReach,
        )
        .expect("a canonical three-edge fillet inserts");
        let id = applied.record.minted.expect("the fillet is minted");
        (applied.doc, id)
    };
    // Entry 2 onto entry 0's name: the set shrinks to two and re-sorts.
    let doc = apply(
        &doc,
        &DocEdit::Rebind {
            from: edge(solid, 4),
            to: edge(solid, 0),
        },
        Tol::witness(),
        &editor_core::RefusingReach,
    )
    .expect("the rebind applies")
    .doc;
    let Some(Node::Fillet { selection, .. }) = doc.node(fillet) else {
        panic!("the fillet survives the rebind")
    };
    assert_eq!(
        selection,
        &vec![edge(solid, 0), edge(solid, 2)],
        "the repair re-establishes the canonical form, shrinking by one"
    );
    let node = doc.node(fillet).expect("the fillet is live");
    assert!(
        node.input_fault().is_none(),
        "so the repaired node passes the predicate every door asks"
    );
}
