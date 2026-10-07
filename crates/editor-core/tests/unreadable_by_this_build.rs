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
use editor_core::ExtrudeSide;

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
            profile,
            distance: len(1.0),
            side: ExtrudeSide::Along,
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
/// minimal: nothing newer than {Datum, Profile, Extrude} appears in it.
/// Re-frozen again when a profile's steps gained minted ids (the
/// program's `ids` and the document's step counter), the same kind of
/// break, and again when the counter became the step mint's chain and
/// log, again when node ids moved onto that mint, and again when an
/// extrude's side became a required field, and again when a slot
/// came to hold its variable's id.
const OLDER_SHAPED: &str = concat!(
    "id: 8ad37e1a750ae77132c0bf059acb322f\n{\"snapshot\":{\"id\":\"8ad37e1a750ae77132c0bf",
    "059acb322f\",\"mint\":{\"chain\":\"0316a8e5f987300fb63dbac217e149d3ac1a39a289aa4d0de",
    "502fe43be30e09f\",\"log\":[{\"node\":222550937288781839},{\"var\":687067507121259641",
    "},{\"var\":1692951550161305783},{\"var\":3992818210703844589},{\"var\":4557121810070",
    "918341},{\"var\":4720270004154464002},{\"var\":5217081412177518420},{\"step\":642089",
    "1478810644163},{\"node\":6952591527216186421},{\"var\":6958480090943866564},{\"var\"",
    ":8004427572517704354},{\"var\":9933765765270439050},{\"step\":10270049170496020773},",
    "{\"var\":10636643242840588584},{\"var\":11508714693452263776},{\"var\":1152016280368",
    "8111428},{\"var\":12921966442148625044},{\"var\":13437228041663008622},{\"var\":1496",
    "9638619857046222},{\"var\":15645373147172179756},{\"step\":16054304900290762691},{\"",
    "var\":16300829493895992422},{\"var\":16922366612452288273},{\"node\":174034797324733",
    "40673},{\"step\":17434746612444774629},{\"step\":18381077563867063357}]},\"nodes\":{",
    "\"222550937288781839\":{\"Extrude\":{\"profile\":6952591527216186421,\"distance\":12",
    "921966442148625044,\"side\":\"along\"}},\"6952591527216186421\":{\"Profile\":{\"plan",
    "e\":17403479732473340673,\"loops\":[{\"Chain\":[{\"At\":[10636643242840588584,800442",
    "7572517704354]},{\"LineTo\":{\"Point\":[3992818210703844589,16922366612452288273]}},",
    "{\"LineTo\":{\"Point\":[14969638619857046222,4720270004154464002]}},{\"LineTo\":{\"P",
    "oint\":[13437228041663008622,4557121810070918341]}},{\"LineTo\":\"Start\"}]}],\"ids\"",
    ":[[17434746612444774629,6420891478810644163,18381077563867063357,1605430490029076269",
    "1,10270049170496020773]]}},\"17403479732473340673\":{\"Datum\":{\"Frame\":{\"origin\"",
    ":[16300829493895992422,9933765765270439050,1692951550161305783],\"u\":[1564537314717",
    "2179756,6958480090943866564,5217081412177518420],\"v\":[11508714693452263776,1152016",
    "2803688111428,687067507121259641]}}}},\"order\":[17403479732473340673,69525915272161",
    "86421,222550937288781839],\"roots\":[222550937288781839],\"vars\":{\"687067507121259",
    "641\":{\"kind\":\"Scalar\",\"def\":{\"Free\":{\"Continuous\":{\"dim\":\"Scalar\",\"v",
    "alue\":0.0,\"display_unit\":\"\"}}}},\"1692951550161305783\":{\"kind\":\"Length\",\"",
    "def\":{\"Free\":{\"Continuous\":{\"dim\":\"Length\",\"value\":0.0,\"display_unit\":\"",
    "m\"}}}},\"3992818210703844589\":{\"kind\":\"Length\",\"def\":{\"Free\":{\"Continuous",
    "\":{\"dim\":\"Length\",\"value\":1.0,\"display_unit\":\"m\"}}}},\"455712181007091834",
    "1\":{\"kind\":\"Length\",\"def\":{\"Free\":{\"Continuous\":{\"dim\":\"Length\",\"val",
    "ue\":1.0,\"display_unit\":\"m\"}}}},\"4720270004154464002\":{\"kind\":\"Length\",\"d",
    "ef\":{\"Free\":{\"Continuous\":{\"dim\":\"Length\",\"value\":1.0,\"display_unit\":\"",
    "m\"}}}},\"5217081412177518420\":{\"kind\":\"Scalar\",\"def\":{\"Free\":{\"Continuous",
    "\":{\"dim\":\"Scalar\",\"value\":0.0,\"display_unit\":\"\"}}}},\"6958480090943866564",
    "\":{\"kind\":\"Scalar\",\"def\":{\"Free\":{\"Continuous\":{\"dim\":\"Scalar\",\"valu",
    "e\":0.0,\"display_unit\":\"\"}}}},\"8004427572517704354\":{\"kind\":\"Length\",\"def",
    "\":{\"Free\":{\"Continuous\":{\"dim\":\"Length\",\"value\":0.0,\"display_unit\":\"m\"",
    "}}}},\"9933765765270439050\":{\"kind\":\"Length\",\"def\":{\"Free\":{\"Continuous\":",
    "{\"dim\":\"Length\",\"value\":0.0,\"display_unit\":\"m\"}}}},\"10636643242840588584\"",
    ":{\"kind\":\"Length\",\"def\":{\"Free\":{\"Continuous\":{\"dim\":\"Length\",\"value\"",
    ":0.0,\"display_unit\":\"m\"}}}},\"11508714693452263776\":{\"kind\":\"Scalar\",\"def\"",
    ":{\"Free\":{\"Continuous\":{\"dim\":\"Scalar\",\"value\":0.0,\"display_unit\":\"\"}}",
    "}},\"11520162803688111428\":{\"kind\":\"Scalar\",\"def\":{\"Free\":{\"Continuous\":{",
    "\"dim\":\"Scalar\",\"value\":1.0,\"display_unit\":\"\"}}}},\"12921966442148625044\":",
    "{\"kind\":\"Length\",\"def\":{\"Free\":{\"Continuous\":{\"dim\":\"Length\",\"value\"",
    ":1.0,\"display_unit\":\"m\"}}}},\"13437228041663008622\":{\"kind\":\"Length\",\"def\"",
    ":{\"Free\":{\"Continuous\":{\"dim\":\"Length\",\"value\":0.0,\"display_unit\":\"m\"}",
    "}}},\"14969638619857046222\":{\"kind\":\"Length\",\"def\":{\"Free\":{\"Continuous\":",
    "{\"dim\":\"Length\",\"value\":1.0,\"display_unit\":\"m\"}}}},\"15645373147172179756\"",
    ":{\"kind\":\"Scalar\",\"def\":{\"Free\":{\"Continuous\":{\"dim\":\"Scalar\",\"value\"",
    ":1.0,\"display_unit\":\"\"}}}},\"16300829493895992422\":{\"kind\":\"Length\",\"def\"",
    ":{\"Free\":{\"Continuous\":{\"dim\":\"Length\",\"value\":0.0,\"display_unit\":\"m\"}",
    "}}},\"16922366612452288273\":{\"kind\":\"Length\",\"def\":{\"Free\":{\"Continuous\":",
    "{\"dim\":\"Length\",\"value\":0.0,\"display_unit\":\"m\"}}}}},\"var_order\":[1630082",
    "9493895992422,9933765765270439050,1692951550161305783,15645373147172179756,695848009",
    "0943866564,5217081412177518420,11508714693452263776,11520162803688111428,68706750712",
    "1259641,10636643242840588584,8004427572517704354,3992818210703844589,169223666124522",
    "88273,14969638619857046222,4720270004154464002,13437228041663008622,4557121810070918",
    "341,12921966442148625044],\"epsilon\":1e-09,\"witnesses\":{},\"metadata\":{},\"appea",
    "rance\":[]},\"edits\":[]}",
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
        3,
        "frame, profile and extrude, as written"
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

/// **A document saved before an extrude carried its side refuses
/// typed, naming the first thing this build cannot read.** The bytes
/// are `crates/pncad/tests/plate_param.pncad` exactly as main held it
/// before the side became structural (Ev, #3551), frozen here so the
/// row reads real history rather than a mutation of today's save. They
/// predate readers by id too, and a reader by name is the first thing
/// the parser meets that this build cannot read, so the refusal names
/// it (`Param`) and carries the regenerate recourse once. It is kept as
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
        msg.contains("invalid type: map, expected u64"),
        "the refusal names what it cannot read: {msg}"
    );
    assert_eq!(msg.matches(REGENERATE_RECOURSE).count(), 1, "{msg}");
}
