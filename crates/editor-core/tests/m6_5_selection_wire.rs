//! M6-5 — `Node::Fillet`'s required `selection` field on the wire
//! (Ev, #217). A fillet meaning "every edge of the target" names a
//! set that depends on an evaluation the FILE does not carry, so the
//! field has no honest default: it is on the wire under its own key,
//! canonical, and a body without it is unreadable by this build. (The
//! format carries no schema version — the persist module docs say why
//! — so there is no version pin here and no older golden to refuse.)

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod fixture;

use editor_core::{PersistError, load};
use geom_core::Tol;

/// Two claims: the field is on the wire under its own key, and it is
/// on the wire CANONICAL. The fixture hands `Node::fillet` its two
/// names out of order; the bytes show them sorted.
#[test]
fn the_selection_reaches_the_wire_canonical() {
    use editor_core::{
        CapEnd, Dimension, DocEdit, Expr, Node, ProfileDoc, ProfileEdgeRef, RoleSeg, StableName,
        apply, save,
    };

    let square =
        editor_core::LoopProgram::polygon([(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)])
            .expect("finite");
    let len = |v: f64| Expr::literal(v, Dimension::Length).expect("a length literal");
    let mut doc = ProfileDoc::empty_derived("m6_5_selection_wire", Tol::witness());
    for edit in [
        DocEdit::InsertNode {
            node: fixture::xy_frame(),
        },
        DocEdit::InsertNode {
            node: Node::Profile(editor_core::ProfileProgram {
                plane: editor_core::RecipeNodeId(0),
                loops: vec![square],
            }),
        },
        DocEdit::InsertNode {
            node: Node::Extrude {
                profile: editor_core::RecipeNodeId(1),
                distance: len(1.0),
            },
        },
    ] {
        doc = apply(&doc, &edit, Tol::witness())
            .expect("the fixture builds")
            .doc;
    }
    let rim = |seg: u32| StableName {
        kind: editor_core::EntityKind::Edge,
        node: editor_core::RecipeNodeId(2),
        path: vec![RoleSeg::RimEdge(
            CapEnd::Top,
            ProfileEdgeRef {
                loop_index: 0,
                segment: seg,
            },
        )],
    };
    doc = apply(
        &doc,
        &DocEdit::InsertNode {
            node: Node::fillet(
                editor_core::RecipeNodeId(2),
                len(0.0625),
                vec![rim(2), rim(0)],
            ),
        },
        Tol::witness(),
    )
    .expect("the fillet node inserts")
    .doc;

    let text = save(&doc, &[], Tol::witness()).expect("the fixture saves");
    assert!(text.contains("\"selection\""), "the field reaches the wire");
    let sel = text.find("\"selection\"").expect("the selection block");
    let zero = text[sel..].find("\"segment\": 0").expect("segment 0");
    let two = text[sel..].find("\"segment\": 2").expect("segment 2");
    assert!(zero < two, "stored in canonical order, not authoring order");

    // A non-canonical selection on the wire is a CORRUPT file: refused
    // at the shared validator, never quietly re-sorted (a repair would
    // move the node's content key behind the caller's back).
    let corrupt = text.replacen("\"segment\": 0", "\"segment\": 9", 1);
    match load(&corrupt, Tol::witness()) {
        Err(PersistError::Snapshot(editor_core::SnapshotError::BlendSelectionNotCanonical {
            ..
        })) => {}
        other => panic!("a non-canonical selection must refuse typed, got {other:?}"),
    }

    // A fillet with no `selection` at all (an "every edge" fillet, the
    // shape before the field existed) cannot be promoted by hand: the
    // field has no default and `deny_unknown_fields` admits no
    // stand-in, so the body is unreadable by this build and the
    // refusal names the field it met instead.
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
