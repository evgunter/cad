//! REVIEW PROBE (lane `blend-rv`): does `at` name the entry the doc
//! says it names, at a position other than zero, at BOTH doors?
//!
//! Every row in `edit_blend_canonical` asserts `at: 0`, and
//! `both_doors_forward_one_sentence` builds its fault by hand, so the
//! computed index is never compared against a non-zero expectation.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;

use editor_core::{
    CapEnd, DocEdit, EditError, EntityKind, InputFault, Node, PersistError, ProfileDoc,
    ProfileEdgeRef, ProfileProgram, RecipeNodeId, RoleSeg, SnapshotError, StableName, apply, load,
    save,
};
use geom_core::Tol;

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

/// `input_fault` directly, at each position of a three-name selection.
#[test]
fn probe_at_names_each_position() {
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
        let node: Node<ProfileProgram> = Node::Fillet {
            target: solid,
            radius: fixture::len(0.0625),
            selection: segs.iter().map(|s| edge(solid, *s)).collect(),
        };
        let got = match node.input_fault() {
            Some(InputFault::SelectionNotCanonical { at }) => Some(at),
            None => None,
            other => panic!("{what}: unexpected fault {other:?}"),
        };
        assert_eq!(got, *want, "{what}");
    }
}

/// The same non-zero position, reported by the INSERT door.
#[test]
fn probe_insert_door_reports_position_one() {
    let (doc, solid) = prism();
    let raw: Node<ProfileProgram> = Node::Fillet {
        target: solid,
        radius: fixture::len(0.0625),
        selection: vec![edge(solid, 0), edge(solid, 4), edge(solid, 2)],
    };
    match apply(&doc, &DocEdit::InsertNode { node: raw }, Tol::witness()) {
        Err(EditError::SelectionNotCanonical { at, .. }) => {
            assert_eq!(at, 1, "the break is between entries 1 and 2")
        }
        other => panic!("expected a typed refusal, got {other:?}"),
    }
}

/// The same non-zero position, reported by the LOAD door.
#[test]
fn probe_load_door_reports_position_one() {
    let (doc, solid) = prism();
    let doc = apply(
        &doc,
        &DocEdit::InsertNode {
            node: Node::fillet(
                solid,
                fixture::len(0.0625),
                vec![edge(solid, 0), edge(solid, 2), edge(solid, 4)],
            ),
        },
        Tol::witness(),
    )
    .expect("a canonical three-edge fillet inserts")
    .doc;
    let text = save(&doc, &[], Tol::witness()).expect("the fixture saves");
    load(&text, Tol::witness()).expect("the canonical fixture loads");
    // `[0, 2, 4]` -> `[0, 9, 4]`: the break moves to entry 1.
    let sel = text
        .find("\"selection\"")
        .expect("the selection is on the wire");
    let needle = "\"segment\": 2";
    let at = sel + text[sel..].find(needle).expect("entry 1 is on the wire");
    let corrupt = format!(
        "{}\"segment\": 9{}",
        &text[..at],
        &text[at + needle.len()..]
    );
    match load(&corrupt, Tol::witness()) {
        Err(PersistError::Snapshot(SnapshotError::InputList {
            fault: InputFault::SelectionNotCanonical { at },
            ..
        })) => assert_eq!(at, 1, "the load door names the same entry"),
        other => panic!("expected a typed refusal, got {other:?}"),
    }
}
