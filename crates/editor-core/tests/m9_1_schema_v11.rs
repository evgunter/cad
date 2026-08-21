//! **The declaration-class clean break** (M9-1 spec PR-2;
//! CONTACT-DESIGN C4, ratified #178) — schema **v11**, and the gate
//! that refuses everything older.
//!
//! `Node::Declare`'s pairs each gained the contact class they assert,
//! so a v10 pair (a bare name pair) and a v11 pair (a name pair plus a
//! class) are different shapes at the wire in both directions. With
//! `deny_unknown_fields` there is no forgiving read of either from the
//! other.
//!
//! **Why the migration table stays empty.** A migration could write
//! `rest` into every v8 pair — that is what they meant — but it would
//! be authoring the one datum the break exists to stop being assumed,
//! silently, on files whose author never made the choice. C4's
//! invariant is that no path exists from "the numbers look equal" to a
//! glued contact without a structural or declared rung; a migration
//! that authors the rung on the user's behalf is that path with extra
//! steps. So older files refuse TYPED with the regenerate recourse.
//!
//! **Why 11**: this unit claimed 10 at dispatch (LIB-RESPELL held 9
//! and was open), then #531 merged with 9 and ASM-UPD merged with 10
//! while this branch was open, so the claim moved twice. Each shift
//! was a conscious resolve; see the `persist/mod.rs` ledger, including
//! what the second re-merge showed — both sides had written `10`, so
//! the CONSTANT merged cleanly and only the prose collision caught it.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use editor_core::{
    CapEnd, ContactClass, DocEdit, Node, PersistError, ProfileDoc, REGENERATE_RECOURSE, RoleSeg,
    SCHEMA_VERSION, load, save,
};
use geom_core::Tol;

mod fixture;

/// The prior live goldens, kept as REFUSAL fixtures. v10 is what the
/// build immediately before this one wrote (ASM-UPD's
/// `UpdateReference` arm); v9 and v8 are the two before that. All
/// refuse identically — the gate has no notion of "nearly current",
/// which is the property that makes the version door decidable.
const V10: &str = include_str!("golden/v10_golden.cad");
const V9: &str = include_str!("golden/v9_golden.cad");
const V8: &str = include_str!("golden/v8_golden.cad");
/// The oldest fixture in the tree — the far end of the same gate.
const V1: &str = include_str!("golden/v1_golden.cad");

#[test]
fn schema_version_is_current() {
    // Named for the PROPERTY, not the number (the `lbret_schema_v8`
    // precedent): M9-1's own bump was v11; LIB-PLACEDUNION took v12
    // ASM-R2a v13 and ASM-R2b v14, and the number is exactly what keeps moving.
    assert_eq!(SCHEMA_VERSION, 14);
}

/// The IMMEDIATE prior version refuses, from the real file. Named
/// separately from the v8/v1 rows because it is the one a user is
/// actually likely to be holding.
#[test]
fn v10_golden_refuses_too_old() {
    assert_eq!(
        V10.lines().next(),
        Some("schema: 10"),
        "the immediate-prior fixture must be the version it is named for"
    );
    match load(V10, Tol::witness()) {
        Err(PersistError::SchemaTooOld { found, missing, .. }) => {
            assert_eq!(found, 10);
            assert_eq!(missing, 10);
        }
        other => panic!("v10 must refuse SchemaTooOld, got {other:?}"),
    }
}

/// And so does the version before it, from ITS real file.
#[test]
fn v9_golden_refuses_too_old() {
    assert_eq!(V9.lines().next(), Some("schema: 9"));
    match load(V9, Tol::witness()) {
        Err(PersistError::SchemaTooOld { found, missing, .. }) => {
            assert_eq!(found, 9);
            assert_eq!(missing, 9);
        }
        other => panic!("v9 must refuse SchemaTooOld, got {other:?}"),
    }
}

#[test]
fn the_checked_in_v8_file_is_really_v8() {
    assert_eq!(
        V8.lines().next(),
        Some("schema: 8"),
        "the refusal fixture must still be the version it is named for"
    );
}

/// The immediately-prior version refuses, naming the step that does
/// not exist. `missing` is 8 and not 9: the loader walks the chain
/// from the file's version upward and names the FIRST gap.
#[test]
fn v8_refuses_too_old() {
    match load(V8, Tol::witness()) {
        Err(PersistError::SchemaTooOld {
            found,
            supported,
            missing,
        }) => {
            assert_eq!(found, 8);
            assert_eq!(supported, SCHEMA_VERSION);
            assert_eq!(
                missing, 8,
                "the 8 → 9 step is the first that does not exist"
            );
        }
        other => panic!("v8 must refuse SchemaTooOld, got {other:?}"),
    }
}

/// The far end of the same gate: v1 refuses identically. Both ends
/// matter — a gate that only rejects the neighbour would let a v1 file
/// through a chain that no longer exists.
#[test]
fn v1_refuses_too_old() {
    match load(V1, Tol::witness()) {
        Err(PersistError::SchemaTooOld { found, missing, .. }) => {
            assert_eq!(found, 1);
            assert_eq!(missing, 1);
        }
        other => panic!("v1 must refuse SchemaTooOld, got {other:?}"),
    }
}

/// The OTHER direction: a file NEWER than this build is `UnknownSchema`,
/// never `SchemaTooOld`. The two refusals must not be confusable —
/// "regenerate from source" is right for one and wrong for the other.
#[test]
fn newer_than_this_build_is_unknown_not_too_old() {
    let text = format!("schema: {}\n{{}}\n", SCHEMA_VERSION + 1);
    match load(&text, Tol::witness()) {
        Err(PersistError::UnknownSchema { found, newest }) => {
            assert_eq!(found, u64::from(SCHEMA_VERSION) + 1);
            assert_eq!(newest, SCHEMA_VERSION);
        }
        other => panic!("a newer schema must be UnknownSchema, got {other:?}"),
    }
}

/// The refusal composes the shared recourse EXACTLY once — the
/// two-tolerance message discipline applied to the persistence door.
#[test]
fn the_too_old_message_names_the_recourse_exactly_once() {
    let msg = load(V8, Tol::witness()).expect_err("v8 refuses").to_string();
    assert_eq!(msg.matches(REGENERATE_RECOURSE).count(), 1, "{msg}");
    assert!(msg.contains("v8"), "the message names the version: {msg}");
}

/// The version gate runs BEFORE the body is parsed: a too-old file
/// with an unparseable body still refuses on the VERSION, so the
/// diagnostics name the real problem instead of whatever the stale
/// bytes happen to fail as.
#[test]
fn too_old_beats_a_broken_body() {
    let text = "schema: 8\nnot json at all\n";
    match load(text, Tol::witness()) {
        Err(PersistError::SchemaTooOld { found, .. }) => assert_eq!(found, 8),
        other => panic!("the version gate runs first, got {other:?}"),
    }
}

/// A saved document announces the CURRENT version in its header, and a
/// class-bearing document round-trips with its declaration CLASSES
/// intact — the actual content of this break.
#[test]
fn a_declaration_round_trips_carrying_its_class() {
    let (doc, decl) = declaring_doc();
    let text = save(&doc, &[], Tol::witness()).expect("saves");
    assert_eq!(
        text.lines().next(),
        Some(format!("schema: {SCHEMA_VERSION}").as_str())
    );

    let back: ProfileDoc = load(&text, Tol::witness()).expect("the current version loads").doc;
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
    assert!(
        err.to_string().contains("fit"),
        "the refusal names the spelling it could not read: {err}"
    );
}

/// A document with one `Rest` pair and one `Tangent` pair, plus the
/// Declare node's id. The classes differ ON PURPOSE: a round trip that
/// only ever saw one class could not tell "persisted" from
/// "defaulted on read".
fn declaring_doc() -> (ProfileDoc, editor_core::RecipeNodeId) {
    let doc = ProfileDoc::empty_derived("m9_1_schema_v10", Tol::witness());
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

/// The IMMEDIATE prior version refuses — and since LIB-RESPELL (#531)
/// merged while this branch was open, that version is now v9 and a
/// real v9 document exists in the tree (`golden/v9_golden.cad`).
///
/// This row arrived as a review probe against a HYPOTHETICAL v9 (the
/// number was claimed but unmerged), which is why it reads a bare
/// header rather than the golden; it is kept in that form on purpose,
/// because a header-only document proves the gate runs before the
/// body. The `v9_golden_refuses_too_old` row below is the same claim
/// against the real file.
#[test]
fn probe_v9_header_refuses_too_old() {
    match load("schema: 9\n{}\n", Tol::witness()) {
        Err(PersistError::SchemaTooOld { found, missing, .. }) => {
            assert_eq!(found, 9);
            assert_eq!(missing, 9);
        }
        other => panic!("v9 must refuse SchemaTooOld, got {other:?}"),
    }
}
