//! **The declaration class on the wire** (M9-1 spec PR-2;
//! CONTACT-DESIGN C4, ratified #178).
//!
//! `Node::Declare`'s pairs each carry the contact class they assert.
//! The class is persisted per pair as a stable spelling, never
//! defaulted on read: C4's invariant is that no path exists from "the
//! numbers look equal" to a glued contact without a structural or
//! declared rung, and a reader that filled in `rest` for a pair
//! lacking one would be that path with extra steps. So the round trip
//! pins two DIFFERENT classes, and an unknown spelling refuses naming
//! it. (The format carries no schema version — the persist module
//! docs say why — so there is no version pin here and no older golden
//! to refuse.)

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use editor_core::{CapEnd, ContactClass, DocEdit, Node, ProfileDoc, RoleSeg, load, save};
use geom_core::Tol;

mod fixture;

/// A class-bearing document round-trips with its declaration CLASSES
/// intact.
#[test]
fn a_declaration_round_trips_carrying_its_class() {
    let (doc, decl) = declaring_doc();
    let text = save(&doc, &[], Tol::witness()).expect("saves");

    let back: ProfileDoc = load(&text, Tol::witness())
        .expect("the saved text loads")
        .doc;
    let Some(Node::Declare { pairs }) = back.node(decl) else {
        panic!("the Declare node survives the round trip");
    };
    assert_eq!(pairs.len(), 2);
    assert_eq!(pairs[0].1, ContactClass::Rest);
    assert_eq!(
        pairs[1].1,
        ContactClass::Tangent,
        "the class is persisted per pair, not defaulted on read"
    );
}

/// The wire spelling is a stable STRING, and an unknown one refuses
/// typed rather than being read as some other class. A discriminant
/// would have re-mapped silently the day a variant landed between the
/// two.
#[test]
fn an_unknown_class_spelling_refuses_typed() {
    let (doc, _) = declaring_doc();
    let text = save(&doc, &[], Tol::witness()).expect("saves");
    assert!(
        text.contains("\"rest\"") && text.contains("\"tangent\""),
        "classes ride the wire as stable spellings: {text}"
    );
    let tampered = text.replace("\"tangent\"", "\"fit\"");
    let err = load(&tampered, Tol::witness()).expect_err("an unknown class spelling must refuse");
    // The deserializer's message is the ONLY place the name exists
    // (serde exposes no structured accessor), so reading it back is the
    // assertion, not message sniffing; the fuller phrase keeps a short
    // word from matching by accident.
    assert!(
        err.to_string().contains("unknown contact class 'fit'"),
        "the refusal names the spelling it could not read: {err}"
    );
}

/// A document with one `Rest` pair and one `Tangent` pair, plus the
/// Declare node's id. The classes differ ON PURPOSE: a round trip that
/// only ever saw one class could not tell "persisted" from
/// "defaulted on read".
fn declaring_doc() -> (ProfileDoc, editor_core::RecipeNodeId) {
    let doc = ProfileDoc::empty_derived("m9_1_declaration_wire", Tol::witness());
    let (doc, a) = block(doc, (0.0, 2.0), (0.0, 2.0), 0.0, 1.0);
    let (doc, b) = block(doc, (0.0, 2.0), (0.0, 2.0), 1.0, 1.0);
    let cap = |node, end| fixture::fname(node, RoleSeg::Cap(end));
    let node: Node<editor_core::ProfileProgram> = Node::Declare {
        pairs: vec![
            (
                (cap(a, CapEnd::Top), cap(b, CapEnd::Bottom)),
                ContactClass::Rest,
            ),
            (
                (cap(a, CapEnd::Bottom), cap(b, CapEnd::Top)),
                ContactClass::Tangent,
            ),
        ],
    };
    let applied = doc
        .apply(&DocEdit::InsertNode { node }, Tol::witness())
        .expect("the Declare inserts");
    let id = applied.record.minted.expect("an id is minted");
    (applied.doc, id)
}

fn block(
    doc: ProfileDoc,
    x: (f64, f64),
    y: (f64, f64),
    z0: f64,
    dz: f64,
) -> (ProfileDoc, editor_core::RecipeNodeId) {
    let (doc, p) = fixture::insert(
        doc,
        Node::Profile(fixture::desc(
            [0.0, 0.0, z0],
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            vec![vec![(x.0, y.0), (x.1, y.0), (x.1, y.1), (x.0, y.1)]],
        )),
    );
    fixture::insert(
        doc,
        Node::Extrude {
            profile: p,
            distance: fixture::len(dz),
        },
    )
}
