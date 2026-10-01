//! M6-5 — `Node::Fillet`'s required `selection` field on the wire
//! (Ev, #217). A fillet meaning "every edge of the target" names a
//! set that depends on an evaluation the FILE does not carry, so the
//! field has no honest default: it is on the wire under its own key,
//! canonical, and a body without it is unreadable by this build. (The
//! format carries no schema version — the persist module docs say why
//! — so there is no version pin here and no older golden to refuse.)

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;

use crate::fixture::len;
use editor_core::{PersistError, load};
use geom_core::Tol;

/// Two claims: the field is on the wire under its own key, and it is
/// on the wire CANONICAL. The fixture hands `Node::fillet` its two
/// names out of order; the bytes show them sorted.
#[test]
fn the_selection_reaches_the_wire_canonical() {
    use editor_core::{CapEnd, DocEdit, Node, ProfileDoc, RoleSeg, StableName, apply, save};

    let square =
        editor_core::LoopProgram::polygon([(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)])
            .expect("finite");
    let doc = ProfileDoc::empty_derived("m6_5_selection_wire", Tol::witness());
    let (doc, plane) = fixture::insert(doc, fixture::xy_frame());
    let (doc, profile) = fixture::insert(
        doc,
        Node::Profile(editor_core::ProfileProgram {
            plane,
            loops: vec![square],
            ids: Vec::new(),
        }),
    );
    let (mut doc, body) = fixture::insert(
        doc,
        Node::Extrude {
            profile,
            distance: len(1.0),
        },
    );
    let steps: Vec<u64> = (0..4)
        .map(|seg| match crate::fixture::piece(&doc, body, 0, seg) {
            editor_core::ProfileEdgeRef::Piece { step, .. } => step.0,
            other => panic!("a square's side is a step's piece, got {other:?}"),
        })
        .collect();
    let step_of = |seg: usize| steps[seg];
    // Canonical order is name order, which for two pieces of one
    // profile is their steps' id order. The selection is AUTHORED high
    // id first, so the stored order below differs from the authored one
    // whatever ids the chain drew.
    let (low, high) = if step_of(0) < step_of(2) {
        (0, 2)
    } else {
        (2, 0)
    };
    let rim = |seg: u32| StableName {
        kind: editor_core::EntityKind::Edge,
        node: body,
        path: vec![RoleSeg::RimEdge(
            CapEnd::End,
            crate::fixture::piece(&doc, body, 0, seg as usize),
        )],
    };
    doc = apply(
        &doc,
        &DocEdit::InsertNode {
            node: Box::new(Node::fillet(
                body,
                len(0.0625),
                vec![rim(high as u32), rim(low as u32)],
            )),
        },
        Tol::witness(),
        &editor_core::RefusingReach,
    )
    .expect("the fillet node inserts")
    .doc;

    let text = save(&doc, &[], Tol::witness()).expect("the fixture saves");
    assert!(text.contains("\"selection\""), "the field reaches the wire");
    let sel = text.find("\"selection\"").expect("the selection block");
    let spelled = |seg: usize| format!("\"step\": {}", step_of(seg));
    let at_low = text[sel..].find(&spelled(low)).expect("the lower id");
    let at_high = text[sel..].find(&spelled(high)).expect("the higher id");
    assert!(
        at_low < at_high,
        "stored in canonical order, not authoring order"
    );

    // A non-canonical selection on the wire is a CORRUPT file: refused
    // at the shared validator, never quietly re-sorted (a repair would
    // move the node's content key behind the caller's back). The form
    // is one predicate on `Node::input_fault`, so the load door names it
    // in the arm it names every other structural fault in;
    // `edit_blend_canonical` is where the two doors are pinned together.
    // The two pieces' steps swapped, so the list runs high to low.
    let corrupt = format!(
        "{}{}",
        &text[..sel],
        text[sel..]
            .replacen(&spelled(low), "@swap@", 1)
            .replacen(&spelled(high), &spelled(low), 1)
            .replacen("@swap@", &spelled(high), 1)
    );
    match load(&corrupt, Tol::witness()) {
        Err(PersistError::Snapshot(editor_core::SnapshotError::InputList {
            fault: editor_core::ListFault::SelectionNotCanonical { at: 0 },
            ..
        })) => {}
        other => panic!("a non-canonical selection must refuse typed, got {other:?}"),
    }

    // A fillet with no `selection` at all (an "every edge" fillet, the
    // shape before the field existed) cannot be promoted by hand: the
    // field has no default, so the body is unreadable by this build
    // either way. What `deny_unknown_fields` on `Node` buys is WHICH
    // NAME the refusal carries — the stand-in it met rather than the
    // field it wanted — which is what the assertion below reads, and
    // it is the only difference the attribute makes here.
    let unselected = text.replacen("\"selection\"", "\"unselection\"", 1);
    match load(&unselected, Tol::witness()) {
        Err(PersistError::Unreadable { detail, .. }) => {
            // The deserializer's message is the ONLY place the name
            // exists (serde exposes no structured accessor), so reading
            // it back is the assertion, not message sniffing; the
            // fuller phrase keeps a short word from matching by accident.
            assert!(detail.contains("unknown field `unselection`"), "{detail}");
        }
        other => panic!("a fillet without its selection must refuse unreadable, got {other:?}"),
    }
}
