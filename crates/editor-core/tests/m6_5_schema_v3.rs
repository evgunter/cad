//! M6-5 — the schema-v3 CLEAN BREAK, pinned.
//!
//! The call (Evan, #217): `Node::Fillet` grew a required `selection`
//! field, and no `migrate` step is written for 2 → 3. A v2 fillet
//! meant "every edge of the target" — a set that depends on an
//! evaluation the FILE does not carry — so there is no honest default
//! to migrate to, and none is invented. A v2 file refuses TYPED with
//! the regenerate recourse, exactly as v1 does; the v2 BYTES stay
//! checked in as the refusal fixture, because a break nobody can
//! demonstrate is a break nobody can trust (the M5 PR 10 precedent,
//! verbatim).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use editor_core::{PersistError, REGENERATE_RECOURSE, SCHEMA_VERSION, load};

/// The pre-break bytes, kept verbatim as the refusal fixture.
const V2: &str = include_str!("golden/v2_golden.cad");
/// The regenerated live golden.
const V3: &str = include_str!("golden/v3_golden.cad");

#[test]
fn schema_version_is_three() {
    assert_eq!(SCHEMA_VERSION, 3);
}

#[test]
fn the_checked_in_v3_file_is_really_v3() {
    assert_eq!(V3.lines().next(), Some("schema: 3"));
}

/// The break, demonstrated: a v2 file refuses TYPED at the version
/// door, naming the version found, the version supported, and the
/// step that does not exist.
#[test]
fn v2_refuses_too_old() {
    match load(V2) {
        Err(PersistError::SchemaTooOld {
            found,
            supported,
            missing,
        }) => {
            assert_eq!(found, 2);
            assert_eq!(supported, SCHEMA_VERSION);
            assert_eq!(missing, 2, "the 2 → 3 step is the one that does not exist");
        }
        other => panic!("v2 must refuse SchemaTooOld, got {other:?}"),
    }
}

/// The message names all three facts and ends on the ONE shared
/// recourse carrier, composed exactly once.
#[test]
fn the_too_old_message_names_the_recourse_exactly_once() {
    let msg = PersistError::SchemaTooOld {
        found: 2,
        supported: SCHEMA_VERSION,
        missing: 2,
    }
    .to_string();
    assert_eq!(msg.matches(REGENERATE_RECOURSE).count(), 1, "{msg}");
    assert!(msg.contains("v2"), "{msg}");
    assert!(msg.contains(&format!("v{SCHEMA_VERSION}")), "{msg}");
}

/// The version door still precedes the body parse: a v2 header over
/// nonsense reports the version, never a parse position.
#[test]
fn too_old_beats_a_broken_body() {
    let text = "schema: 2\nnot json at all\n";
    assert!(
        matches!(load(text), Err(PersistError::SchemaTooOld { found: 2, .. })),
        "version door must precede the body parse"
    );
}

/// **The selection's wire bytes, pinned.** The shared golden
/// (`m4_pr6_golden.rs`) deliberately carries no fillet — a golden must
/// EVALUATE green, and a green fillet needs a whole polyhedron of
/// geometry that golden does not have — so the new field is pinned
/// here instead, on a minimal document saved through the same door.
///
/// Two claims: the field is on the wire under its own key, and it is
/// on the wire CANONICAL. The fixture hands `Node::fillet` its two
/// names out of order; the bytes show them sorted.
#[test]
fn the_selection_reaches_the_wire_canonical() {
    use editor_core::{
        CapEnd, Dimension, DocEdit, Expr, Node, ProfileDesc, ProfileDoc, ProfileEdgeRef, RoleSeg,
        StableName, apply, save,
    };

    let square = profile::ProfileLoop::new(
        [(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)]
            .into_iter()
            .map(|(x, y)| profile::ProfileVertex {
                pos: geom_core::Point2::new(x, y),
                bulge: 0.0,
            })
            .collect(),
    );
    let len = |v: f64| Expr::literal(v, Dimension::Length).expect("a length literal");
    let mut doc = ProfileDoc::empty();
    for edit in [
        DocEdit::InsertNode {
            node: Node::Profile(ProfileDesc(profile::Profile::new(
                profile::SketchPlane::xy(),
                vec![square],
            ))),
        },
        DocEdit::InsertNode {
            node: Node::Extrude {
                profile: editor_core::RecipeNodeId(0),
                distance: len(1.0),
            },
        },
    ] {
        doc = apply(&doc, &edit).expect("the fixture builds").doc;
    }
    let rim = |seg: u32| StableName {
        kind: editor_core::EntityKind::Edge,
        node: editor_core::RecipeNodeId(1),
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
                editor_core::RecipeNodeId(1),
                len(0.0625),
                vec![rim(2), rim(0)],
            ),
        },
    )
    .expect("the fillet node inserts")
    .doc;

    let text = save(&doc, &[]).expect("the fixture saves");
    assert_eq!(
        text.lines().next(),
        Some(&format!("schema: {SCHEMA_VERSION}")[..])
    );
    assert!(text.contains("\"selection\""), "the field reaches the wire");
    let sel = text.find("\"selection\"").expect("the selection block");
    let zero = text[sel..].find("\"segment\": 0").expect("segment 0");
    let two = text[sel..].find("\"segment\": 2").expect("segment 2");
    assert!(zero < two, "stored in canonical order, not authoring order");

    // A non-canonical selection on the wire is a CORRUPT file: refused
    // at the shared validator, never quietly re-sorted (a repair would
    // move the node's content key behind the caller's back).
    let corrupt = text.replacen("\"segment\": 0", "\"segment\": 9", 1);
    match load(&corrupt) {
        Err(PersistError::Snapshot(editor_core::SnapshotError::FilletSelectionNotCanonical {
            ..
        })) => {}
        other => panic!("a non-canonical selection must refuse typed, got {other:?}"),
    }

    // And the concrete reason 2 → 3 has NO migration step: a v2-shaped
    // fillet — one with no `selection` at all — cannot be promoted by
    // hand, because the field has no default and `deny_unknown_fields`
    // admits no stand-in. Refusing is the honest answer.
    let v2_shaped = text.replacen("\"selection\"", "\"unselection\"", 1);
    match load(&v2_shaped) {
        Err(PersistError::Parse { .. }) => {}
        other => panic!("a fillet without its selection must refuse at the body, got {other:?}"),
    }
}
