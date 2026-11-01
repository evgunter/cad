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

use crate::fixture;

use editor_core::{Node, PersistError, ProfileDoc, REGENERATE_RECOURSE, load, save};
use fixture::{insert, len, on_frame};
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
            profile: profile.into(),
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
///
/// The fixture is a sketch frame, the profile drawn on it, and the
/// extrude over that: the one node spelled `Extrude`.
fn extrude_mut(v: &mut serde_json::Value) -> &mut serde_json::Map<String, serde_json::Value> {
    v["snapshot"]["nodes"]
        .as_object_mut()
        .and_then(|nodes| nodes.values_mut().find(|n| n.get("Extrude").is_some()))
        .and_then(serde_json::Value::as_object_mut)
        .expect("the extrude is an object")
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

/// A snapshot of the wrong JSON type: the refusal names the type the
/// file spells, `Doc`, and carries the recourse once.
#[test]
fn a_snapshot_of_the_wrong_type_refuses_naming_doc() {
    let (header, mut v) = split(&small());
    v["snapshot"] = serde_json::Value::from(5);
    let mutated = join(&header, &v);
    match load(&mutated, Tol::witness()) {
        Err(err @ PersistError::Unreadable { .. }) => {
            let PersistError::Unreadable { detail, .. } = &err else {
                unreachable!()
            };
            assert!(
                detail.contains("invalid type: integer `5`, expected struct Doc"),
                "the refusal names the document's type: {detail}"
            );
            let msg = err.to_string();
            assert_eq!(msg.matches(REGENERATE_RECOURSE).count(), 1, "{msg}");
        }
        other => panic!("a non-object snapshot must refuse unreadable, got {other:?}"),
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
///
/// RE-FROZEN when a profile's sketch plane became a document node. The
/// previous exemplar carried a placement object in the profile's
/// `plane`, which is a node reference now — a BREAKING change, so those
/// bytes are `Unreadable` today rather than loadable. That is the
/// ruling working (a break refuses typed rather than migrating), and it
/// is measured on the real historical corpus by the `bool13r2_probes`
/// row above. The ADDITIVE half cannot be measured with bytes older
/// than a break, so this exemplar is written by today's writer and kept
/// minimal: nothing newer than {Profile, Extrude} appears in it.
/// It is re-frozen, by today's writer, at each such break: last when a
/// profile became 2-D and the frame node it was drawn on retired, so the
/// document is the profile and the extrude alone, the extrude reading
/// the profile's output and going up the profile's own +z; and again
/// when a construction came to define its space beside its body (D′1),
/// whose row the table now holds.
const OLDER_SHAPED: &str = concat!(
    "id: c7e37d80a53df4cb1d21656a27d56c2d\n",
    "{\"snapshot\":{\"id\":\"c7e37d80a53df4cb1d21656a27d56c2d\",\"mint\":{\"chain\":\"57d67edfcd",
    "80ce1c859a3e352266b90eae6ca0d1707aecedd4f5a3ed0490e3c2\",\"log\":[{\"var\":\"1:b639d84",
    "4bab8e826\"},{\"var\":\"2:74af9d633a64b77a\"},{\"var\":\"3:e49b83e35d1cfaf8\"},{\"var\":\"4:",
    "9c83dccc6e32d7b3\"},{\"var\":\"5:d786bf1ceccb8f00\"},{\"var\":\"6:23ee116d75b4556b\"},{\"v",
    "ar\":\"7:455ba71f3c08cba1\"},{\"var\":\"8:4665cdc912ba37b9\"},{\"node\":\"9:b5f34a51587e84",
    "7a\"},{\"step\":\"10:f91a0724eaf428d4\"},{\"step\":\"11:1727f41743e500c9\"},{\"step\":\"12:4",
    "0ebf48fc1ce9abb\"},{\"step\":\"13:5515fee17d00383e\"},{\"step\":\"14:74e05563fbf673eb\"},",
    "{\"var\":\"15:11be59149cf7b592\"},{\"var\":\"16:a2264ed7bb3bdcb7\"},{\"node\":\"17:4a08cd5c",
    "982f47a4\"},{\"var\":\"18:608252c10d916cfe\"},{\"var\":\"19:57d67edfcd80ce1c\"}]},\"nodes\"",
    ":{\"9:b5f34a51587e847a\":{\"Profile\":{\"loops\":[{\"Chain\":[{\"At\":[\"1:b639d844bab8e826",
    "\",\"2:74af9d633a64b77a\"]},{\"LineTo\":{\"Point\":[\"3:e49b83e35d1cfaf8\",\"4:9c83dccc6e3",
    "2d7b3\"]}},{\"LineTo\":{\"Point\":[\"5:d786bf1ceccb8f00\",\"6:23ee116d75b4556b\"]}},{\"Lin",
    "eTo\":{\"Point\":[\"7:455ba71f3c08cba1\",\"8:4665cdc912ba37b9\"]}},{\"LineTo\":\"Start\"}]}",
    "],\"ids\":[[\"10:f91a0724eaf428d4\",\"11:1727f41743e500c9\",\"12:40ebf48fc1ce9abb\",\"13:",
    "5515fee17d00383e\",\"14:74e05563fbf673eb\"]]}},\"17:4a08cd5c982f47a4\":{\"Extrude\":{\"p",
    "rofile\":\"15:11be59149cf7b592\",\"distance\":\"16:a2264ed7bb3bdcb7\"}}},\"vars\":{\"1:b63",
    "9d844bab8e826\":{\"kind\":\"Length\",\"def\":{\"Free\":{\"Continuous\":{\"dim\":\"Length\",\"val",
    "ue\":0.0,\"display_unit\":\"m\"}}}},\"2:74af9d633a64b77a\":{\"kind\":\"Length\",\"def\":{\"Fre",
    "e\":{\"Continuous\":{\"dim\":\"Length\",\"value\":0.0,\"display_unit\":\"m\"}}}},\"3:e49b83e35",
    "d1cfaf8\":{\"kind\":\"Length\",\"def\":{\"Free\":{\"Continuous\":{\"dim\":\"Length\",\"value\":1.",
    "0,\"display_unit\":\"m\"}}}},\"4:9c83dccc6e32d7b3\":{\"kind\":\"Length\",\"def\":{\"Free\":{\"C",
    "ontinuous\":{\"dim\":\"Length\",\"value\":0.0,\"display_unit\":\"m\"}}}},\"5:d786bf1ceccb8f0",
    "0\":{\"kind\":\"Length\",\"def\":{\"Free\":{\"Continuous\":{\"dim\":\"Length\",\"value\":1.0,\"dis",
    "play_unit\":\"m\"}}}},\"6:23ee116d75b4556b\":{\"kind\":\"Length\",\"def\":{\"Free\":{\"Continu",
    "ous\":{\"dim\":\"Length\",\"value\":1.0,\"display_unit\":\"m\"}}}},\"7:455ba71f3c08cba1\":{\"k",
    "ind\":\"Length\",\"def\":{\"Free\":{\"Continuous\":{\"dim\":\"Length\",\"value\":0.0,\"display_u",
    "nit\":\"m\"}}}},\"8:4665cdc912ba37b9\":{\"kind\":\"Length\",\"def\":{\"Free\":{\"Continuous\":{",
    "\"dim\":\"Length\",\"value\":1.0,\"display_unit\":\"m\"}}}},\"15:11be59149cf7b592\":{\"kind\":",
    "\"Profile\",\"def\":{\"Output\":{\"node\":\"9:b5f34a51587e847a\",\"port\":0}}},\"16:a2264ed7b",
    "b3bdcb7\":{\"kind\":\"Length\",\"def\":{\"Free\":{\"Continuous\":{\"dim\":\"Length\",\"value\":1.",
    "0,\"display_unit\":\"m\"}}}},\"18:608252c10d916cfe\":{\"kind\":\"Body\",\"def\":{\"Output\":{\"",
    "node\":\"17:4a08cd5c982f47a4\",\"port\":0}}},\"19:57d67edfcd80ce1c\":{\"kind\":\"Space\",\"d",
    "ef\":{\"Output\":{\"node\":\"17:4a08cd5c982f47a4\",\"port\":1}}}},\"epsilon\":1e-09,\"witnes",
    "ses\":{},\"metadata\":{},\"appearance\":[]},\"edits\":[]}",
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
    assert_eq!(loaded.doc.ids().len(), 2, "profile and extrude, as written");
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

/// **A document saved before an extrude carried its side refuses
/// typed, naming the first thing this build cannot read.** The bytes
/// are `crates/pncad/tests/plate_param.pncad` exactly as main held it
/// before the side became structural (Ev, #3551), frozen here so the
/// row reads real history rather than a mutation of today's save. They
/// predate ids as pairs too, and an id spelled as one integer is the
/// first thing the parser meets that this build cannot read, so the
/// refusal names it and carries the regenerate recourse once. It is kept as
/// `.cad`, as `bool13_goldens/`' older bytes are: refusal evidence, not
/// a member of the `*.pncad` corpus the load rows walk.
#[test]
fn a_document_from_before_the_extrude_side_refuses_unreadable() {
    let text = include_str!("before_extrude_side/plate_param.cad");
    assert!(
        !text.contains("\"side\""),
        "the frozen bytes predate the field"
    );
    let err = load(text, Tol::witness()).unwrap_err();
    assert!(
        matches!(err, PersistError::Unreadable { .. }),
        "an unknown variant is Unreadable: {err:?}"
    );
    let msg = err.to_string();
    // The first thing this build cannot read in bytes that old is a
    // slot written as an expression where a slot now holds its
    // variable's id.
    assert!(
        msg.contains("expected an id: its mint ordinal"),
        "the refusal names what it cannot read: {msg}"
    );
    assert_eq!(msg.matches(REGENERATE_RECOURSE).count(), 1, "{msg}");
}
