//! **The one refusal that stays** (DESIGN.md, the Band 4 roadmap
//! line; the persist module docs): a document this build cannot read
//! refuses TYPED — `PersistError::Unreadable`, the deserializer's own
//! words naming the variant or field it could not place, and the
//! regenerate recourse — and a document that merely predates some of
//! today's vocabulary LOADS, because additive growth invalidates
//! nothing.
//!
//! There is no schema version to pin and no per-version golden to
//! refuse: this ONE generic family replaces the per-version suites.
//! The refusal rows mutate a fresh save, so they follow the wire
//! shape wherever it goes. The load row reads a SYNTHESIZED body that
//! names two arms of today's vocabulary and nothing else — the
//! additive-growth property by construction; its pair on REAL bytes
//! is the historical census in `bool13r2_probes.rs` (every document
//! an earlier build of this repo wrote, `schema:` line removed: none
//! loads, because the last format change before the demolition made
//! a literal's unit required, and every body refusal names `unit`).
//!
//! Some wire suites still derive their document ids from seeds that
//! spell an old version number (`"blend5-schema-v18"`,
//! `"asm-r2a-schema"`, `"asm-r2b-schema"`). A seed is an
//! id-derivation input, not prose, and stays; this is the one place
//! that says so.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod fixture;

use editor_core::{Node, PersistError, ProfileDoc, REGENERATE_RECOURSE, load, save};
use fixture::{desc, insert, len, on_frame};
use geom_core::Tol;

/// A profile and an extrude: the smallest recipe with a node whose
/// payload has a required field.
fn small() -> String {
    let doc = ProfileDoc::empty_derived("unreadable-by-this-build", Tol::witness());
    let (doc, profile) = on_frame(
        doc,
        [0.0; 3],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)]],
    );
    let (doc, _) = insert(
        doc,
        Node::Extrude {
            profile,
            distance: len(1.0),
        },
    );
    save(&doc, &[], Tol::witness()).expect("saves")
}

/// Splits a save into its header line and its body as a JSON value,
/// so a mutation lands on the STRUCTURE and not on a byte offset.
fn split(text: &str) -> (String, serde_json::Value) {
    let (id_line, body) = text.split_once('\n').unwrap();
    (
        format!("{id_line}\n"),
        serde_json::from_str(body).expect("a save's body is JSON"),
    )
}

/// The extrude node's externally-tagged object (`{"Extrude": {...}}`).
fn extrude_mut(v: &mut serde_json::Value) -> &mut serde_json::Map<String, serde_json::Value> {
    v["snapshot"]["nodes"]["1"]
        .as_object_mut()
        .expect("node 1 is an object")
}

fn join(header: &str, v: &serde_json::Value) -> String {
    format!("{header}{}\n", serde_json::to_string_pretty(v).unwrap())
}

/// A node tag no build has ever written: the shape of a document from
/// a build whose vocabulary this one lacks (or a typo). The refusal is
/// typed, names the variant, and carries the recourse exactly once.
#[test]
fn an_unknown_variant_refuses_naming_it() {
    let (header, mut v) = split(&small());
    let node = extrude_mut(&mut v);
    let payload = node.remove("Extrude").unwrap();
    node.insert("Extrudez".to_string(), payload);
    let mutated = join(&header, &v);
    match load(&mutated, Tol::witness()) {
        Err(err @ PersistError::Unreadable { .. }) => {
            let PersistError::Unreadable { detail, line, .. } = &err else {
                unreachable!()
            };
            assert!(
                detail.contains("unknown variant `Extrudez`"),
                "the refusal names the variant: {detail}"
            );
            assert!(*line >= 1, "the position is real");
            let msg = err.to_string();
            assert_eq!(msg.matches(REGENERATE_RECOURSE).count(), 1, "{msg}");
            assert!(msg.contains("Extrudez"), "{msg}");
        }
        other => panic!("an unknown variant must refuse unreadable, got {other:?}"),
    }
}

/// A field this build requires and the document lacks: the shape of a
/// document from before the field existed. Typed, named, recourse once.
#[test]
fn a_missing_required_field_refuses_naming_it() {
    let (header, mut v) = split(&small());
    let node = extrude_mut(&mut v);
    let removed = node["Extrude"].as_object_mut().unwrap().remove("distance");
    assert!(
        removed.is_some(),
        "the fixture's extrude carries `distance`"
    );
    let mutated = join(&header, &v);
    match load(&mutated, Tol::witness()) {
        Err(err @ PersistError::Unreadable { .. }) => {
            let PersistError::Unreadable { detail, .. } = &err else {
                unreachable!()
            };
            assert!(
                detail.contains("missing field `distance`"),
                "the refusal names the field: {detail}"
            );
            let msg = err.to_string();
            assert_eq!(msg.matches(REGENERATE_RECOURSE).count(), 1, "{msg}");
        }
        other => panic!("a missing required field must refuse unreadable, got {other:?}"),
    }
}

/// A SYNTHESIZED document in today's shape that names exactly two node
/// arms, profile and extrude, and nothing else. No build ever wrote
/// these bytes (the literals carry the `unit` key, which became
/// required only in the last format change before the demolition);
/// what the row proves is the additive-growth property BY
/// CONSTRUCTION: today's `Node` knows many more arms (the row checks a
/// few by name below, against the bytes), and a body that never names
/// them loads regardless, because nothing about an enum's other
/// variants is consulted when a tag it does name is read. This is the
/// property the ruling is FOR — the reason no version number needs to
/// stand between a format's growth and its files. Its pair on REAL
/// older-build bytes is `bool13r2_probes::
/// real_historical_documents_refuse_typed_and_name_what_they_lack`.
///
/// The bytes are compact rather than pretty because whitespace is not
/// part of the shape; the id line and the snapshot id agree by
/// construction, as a saved file's do; the recorded ε is rewritten to
/// the process's below so the LOAD is asserted on every CI ε row.
const OLDER_SHAPED: &str = concat!(
    "id: 403dad134a805e2f6ad6d453633789a4\n",
    "{\"snapshot\":{\"id\":\"403dad134a805e2f6ad6d453633789a4\",\"next_id\":2,",
    "\"nodes\":{\"0\":{\"Profile\":{\"plane\":{\"basis\":[[1.0,0.0,0.0],[0.0,1.0,0.0],[0.0,0.0,1.0]],",
    "\"origin\":[0.0,0.0,0.0]},\"loops\":[{\"Chain\":[{\"At\":[{\"Literal\":{\"value\":0.0,\"dim\":\"Length\",",
    "\"unit\":\"m\"}},{\"Literal\":{\"value\":0.0,\"dim\":\"Length\",\"unit\":\"m\"}}]},",
    "{\"LineTo\":{\"Point\":[{\"Literal\":{\"value\":1.0,\"dim\":\"Length\",\"unit\":\"m\"}},",
    "{\"Literal\":{\"value\":0.0,\"dim\":\"Length\",\"unit\":\"m\"}}]}},",
    "{\"LineTo\":{\"Point\":[{\"Literal\":{\"value\":1.0,\"dim\":\"Length\",\"unit\":\"m\"}},",
    "{\"Literal\":{\"value\":1.0,\"dim\":\"Length\",\"unit\":\"m\"}}]}},",
    "{\"LineTo\":{\"Point\":[{\"Literal\":{\"value\":0.0,\"dim\":\"Length\",\"unit\":\"m\"}},",
    "{\"Literal\":{\"value\":1.0,\"dim\":\"Length\",\"unit\":\"m\"}}]}},{\"LineTo\":\"Start\"}]}]}},",
    "\"1\":{\"Extrude\":{\"profile\":0,\"distance\":{\"Literal\":{\"value\":1.0,\"dim\":\"Length\",",
    "\"unit\":\"m\"}}}}},\"order\":[0,1],\"roots\":[1],\"placements\":{},\"params\":{},\"epsilon\":1e-09,",
    "\"witnesses\":{},\"metadata\":{},\"appearance\":[]},\"edits\":[]}",
    "\n"
);

#[test]
fn an_older_shaped_document_lacking_newer_vocabulary_loads() {
    for newer in [
        "Mate",
        "Measure",
        "Assertion",
        "Chamfer",
        "PlacedUnion",
        "InstantiatePart",
    ] {
        assert!(
            !OLDER_SHAPED.contains(&format!("\"{newer}\"")),
            "the frozen document must predate `{newer}` for this row to mean anything"
        );
    }
    // The bytes record an ε; a document refuses at the LAST door under
    // any other process ε, which would leave the LOAD half of this row
    // unproven on two of the three CI rows. So the recorded ε is the
    // process's — one replacement, the rest of the bytes untouched.
    let eps = Tol::witness().eps();
    let text = OLDER_SHAPED.replacen("\"epsilon\":1e-09", &format!("\"epsilon\":{eps:?}"), 1);
    assert_ne!(text, OLDER_SHAPED, "the ε rewrite must land");
    let loaded = load(&text, Tol::witness()).expect("an older-shaped document loads");
    assert_eq!(
        loaded.doc.order().len(),
        2,
        "profile and extrude, as written"
    );
    assert_eq!(loaded.doc.epsilon().to_bits(), eps.to_bits());
}

/// The seam's other side, so the split is pinned from both directions:
/// bytes that are not JSON at all are `Parse` (a corrupt or truncated
/// file, not a vocabulary problem) and do NOT carry the recourse.
///
/// Both non-JSON classes are covered: a syntax error and a truncated
/// object. (A body that IS JSON but opens with the wrong TYPE — say
/// `"snapshot": [` — is already a `Data` refusal at that token, before
/// any later truncation is reached; that is the Unreadable arm, and
/// `m4_pr6_refusal::corrupt_payloads_refuse_typed` pins it.)
#[test]
fn bytes_that_are_not_json_stay_parse() {
    let (header, _) = split(&small());
    for body in ["%%% not json %%%\n", "{\"snapshot\": {\"id\":\n"] {
        let text = format!("{header}{body}");
        match load(&text, Tol::witness()) {
            Err(err @ PersistError::Parse { .. }) => {
                assert!(!err.to_string().contains(REGENERATE_RECOURSE), "{err}");
            }
            other => panic!("{body:?} must refuse Parse, got {other:?}"),
        }
    }
}
