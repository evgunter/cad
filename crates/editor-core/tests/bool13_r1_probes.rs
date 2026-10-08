//! BOOL-13 review probes (R1): the `Unreadable` / `Parse` seam attacked
//! from both sides, the name-carrying claim checked against REAL
//! older-build bytes (the goldens the unit deleted, re-staged under
//! `bool13_goldens/`), and the additive-growth LOAD row asserted at
//! the ambient ε so it cannot pass through the `ToleranceConflict`
//! escape hatch.
//!
//! Review artefact, not a unit deliverable: every row records what the
//! frozen head DOES, so a row going red on a later head is information,
//! not necessarily a defect.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;
use editor_core::ExtrudeSide;

use editor_core::{
    Node, PersistError, ProfileDoc, REGENERATE_RECOURSE, header_document_id, load, save,
};
use fixture::{insert, len, on_frame};
use geom_core::Tol;

/// A profile and an extrude (the same shape `unreadable_by_this_build`
/// mutates).
fn small() -> String {
    let doc = ProfileDoc::empty_derived("bool13-r1-probes", Tol::witness());
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
            side: ExtrudeSide::Along,
        },
    );
    save(&doc, &[], Tol::witness()).expect("saves")
}

fn split(text: &str) -> (String, serde_json::Value) {
    let (id_line, body) = text.split_once('\n').unwrap();
    (
        format!("{id_line}\n"),
        serde_json::from_str(body).expect("a save's body is JSON"),
    )
}

fn join(header: &str, v: &serde_json::Value) -> String {
    format!("{header}{}\n", serde_json::to_string_pretty(v).unwrap())
}

/// Asserts `Unreadable`, that `detail` carries `name`, and that the
/// rendered message carries the recourse exactly once; returns the
/// detail so the row can print it.
fn expect_unreadable_naming(label: &str, text: &str, name: &str) -> String {
    match load(text, Tol::witness()) {
        Err(err @ PersistError::Unreadable { .. }) => {
            let PersistError::Unreadable { detail, .. } = &err else {
                unreachable!()
            };
            assert!(
                detail.contains(name),
                "{label}: the refusal must name {name:?}: {detail}"
            );
            let msg = err.to_string();
            assert_eq!(
                msg.matches(REGENERATE_RECOURSE).count(),
                1,
                "{label}: {msg}"
            );
            assert!(msg.contains(name), "{label}: {msg}");
            eprintln!("[{label}] Unreadable: {detail}");
            detail.clone()
        }
        other => panic!("{label}: expected Unreadable, got {other:?}"),
    }
}

fn expect_parse(label: &str, text: &str) -> String {
    match load(text, Tol::witness()) {
        Err(err @ PersistError::Parse { .. }) => {
            let msg = err.to_string();
            assert!(!msg.contains(REGENERATE_RECOURSE), "{label}: {msg}");
            eprintln!("[{label}] Parse: {msg}");
            msg
        }
        other => panic!("{label}: expected Parse, got {other:?}"),
    }
}

// ---- Additive growth in the OTHER direction: a newer file, this build ----

/// A top-level key this build has no name for (a field a NEWER build
/// grew): `FileBody` is `deny_unknown_fields`, so it refuses Unreadable
/// naming the key. That is the ruled behaviour for this direction of
/// growth (the persist module docs, and the `Unreadable` variant's own
/// doc): a stale reader must not silently drop data. The recourse it
/// carries reads as advice about the wrong side of the seam for a
/// NEWER file — pre-release there is no such file, and the day there
/// is one is Band 4's.
#[test]
fn an_unknown_top_level_key_refuses_unreadable_naming_it() {
    let (header, mut v) = split(&small());
    v.as_object_mut()
        .unwrap()
        .insert("from_the_future".to_string(), serde_json::json!(1));
    expect_unreadable_naming("top-level key", &join(&header, &v), "from_the_future");
}

#[test]
fn an_unknown_snapshot_field_refuses_unreadable_naming_it() {
    let (header, mut v) = split(&small());
    v["snapshot"]
        .as_object_mut()
        .unwrap()
        .insert("from_the_future".to_string(), serde_json::json!(1));
    expect_unreadable_naming("snapshot field", &join(&header, &v), "from_the_future");
}

#[test]
fn a_missing_top_level_field_refuses_unreadable_naming_it() {
    let (header, mut v) = split(&small());
    v.as_object_mut().unwrap().remove("edits").unwrap();
    expect_unreadable_naming("missing edits", &join(&header, &v), "missing field `edits`");
}

// ---- The seam: inputs that sit between "not JSON" and "not this shape" ----

/// A repeated top-level key is grammatically valid JSON; serde's derive
/// refuses it as `duplicate field`, a `Data` error — so it lands on
/// Unreadable WITH the regenerate recourse, although no build ever
/// writes a duplicate key.
#[test]
fn a_duplicate_top_level_key_lands_on_unreadable_with_the_recourse() {
    let text = small();
    let (header, body) = text.split_once('\n').unwrap();
    let body = body.trim_end();
    assert!(body.ends_with('}'));
    let doubled = format!("{header}\n{}, \"edits\": []}}\n", &body[..body.len() - 1]);
    expect_unreadable_naming("duplicate edits", &doubled, "duplicate field `edits`");
}

/// `1e999` is a valid JSON number token that no f64 holds; serde_json
/// classifies it `Syntax` ("number out of range"), so it reports Parse
/// with no recourse, although the bytes are JSON.
#[test]
fn a_number_out_of_range_is_parse_without_recourse() {
    let text = small();
    let huge = text.replacen("\"value\": 1.0", "\"value\": 1e999", 1);
    assert_ne!(huge, text, "the fixture carries a 1.0 literal");
    expect_parse("1e999", &huge);
}

#[test]
fn a_nan_token_is_parse_without_recourse() {
    let text = small();
    let nan = text.replacen("\"value\": 1.0", "\"value\": NaN", 1);
    assert_ne!(nan, text);
    expect_parse("NaN", &nan);
}

/// `null` is valid JSON and not a document at all; serde classifies
/// the type mismatch `Data`, so it is Unreadable and carries the
/// recourse. Same for a bare number and a bare array.
#[test]
fn a_body_that_is_json_but_no_object_is_unreadable() {
    let (header, _) = split(&small());
    for body in ["null\n", "5\n", "[]\n", "\"a string\"\n"] {
        expect_unreadable_naming(
            &format!("body {body:?}"),
            &format!("{header}{body}"),
            "expected struct FileBody",
        );
    }
}

#[test]
fn an_empty_body_is_parse() {
    let (header, _) = split(&small());
    expect_parse("empty body", &header);
}

#[test]
fn trailing_garbage_after_a_valid_body_is_parse() {
    let text = small();
    let trailing = format!("{} trailing\n", text.trim_end());
    expect_parse("trailing", &trailing);
}

/// A nesting bomb within the load door's nesting limit meets a typed
/// visitor that refuses the first wrong-typed token, `Data` at depth
/// three, so it is Unreadable with the recourse. One past the limit
/// never reaches a type: the door's scan refuses it as the reader's
/// class, Parse, before anything descends into it.
#[test]
fn deep_nesting_is_unreadable_within_the_limit_and_unparsable_past_it() {
    let (header, _) = split(&small());
    let within = format!("{header}{{\"snapshot\": {}}}\n", "[".repeat(200));
    expect_unreadable_naming("deep nesting", &within, "invalid type: sequence");
    let past = format!("{header}{{\"snapshot\": {}}}\n", "[".repeat(100_000));
    let msg = expect_parse("a nesting bomb", &past);
    assert!(
        msg.contains("the body nests deeper than"),
        "the refusal names the nesting limit: {msg}"
    );
}

/// serde's derived struct visitor accepts a SEQUENCE as a struct (fields
/// positional), so a body spelled `[<snapshot>, <edits>]` is not
/// refused: it loads. Pre-existing serde behaviour, recorded because
/// the module docs describe the body as an object.
#[test]
fn a_positional_array_body_loads() {
    let (header, v) = split(&small());
    let arr = serde_json::json!([v["snapshot"], v["edits"]]);
    let text = join(&header, &arr);
    match load(&text, Tol::witness()) {
        Ok(loaded) => assert_eq!(loaded.doc.ids().len(), 3),
        Err(e) => panic!("recorded expectation: a positional body loads; got {e:?}"),
    }
}

#[test]
fn a_wrong_type_at_the_top_is_unreadable() {
    let (header, mut v) = split(&small());
    v["snapshot"] = serde_json::json!(5);
    expect_unreadable_naming("snapshot: 5", &join(&header, &v), "invalid type");
}

/// A duplicate key INSIDE a strict map (the `nodes` section) is refused
/// by the crate's own visitor, also `Data`; the message names the key
/// and section, and the recourse rides along.
#[test]
fn a_duplicate_node_key_names_the_section_and_carries_the_recourse() {
    let text = small();
    let (header, body) = text.split_once('\n').unwrap();
    // Duplicate the first node entry by re-parsing and
    // re-emitting the nodes object with the key twice (serde_json's
    // Value cannot hold duplicates, so splice text).
    let needle = "\"nodes\": {";
    let at = body.find(needle).expect("a nodes section") + needle.len();
    let v: serde_json::Value = serde_json::from_str(body).unwrap();
    let (key, node) = v["snapshot"]["nodes"]
        .as_object()
        .and_then(|nodes| nodes.iter().next())
        .expect("a node");
    let node = serde_json::to_string(node).unwrap();
    let spliced = format!("{header}\n{}\"{key}\": {node},{}", &body[..at], &body[at..]);
    expect_unreadable_naming(
        "duplicate node key",
        &spliced,
        "duplicate snapshot node key",
    );
}

// ---- Real older-build bytes: the goldens the unit deleted ----

const V1: &str = include_str!("bool13_goldens/v1_golden.cad");
const V4: &str = include_str!("bool13_goldens/v4_golden.cad");
const V9: &str = include_str!("bool13_goldens/v9_golden.cad");
const V12: &str = include_str!("bool13_goldens/v12_golden.cad");
const V15: &str = include_str!("bool13_goldens/v15_golden.cad");
const V17: &str = include_str!("bool13_goldens/v17_golden.cad");
const V19: &str = include_str!("bool13_goldens/v19_golden.cad");

/// Drops the `schema: N` line an older build wrote, leaving what the
/// head's format would have been had that build not written one.
fn sans_schema_line(golden: &str) -> &str {
    let (first, rest) = golden.split_once('\n').unwrap();
    assert!(first.starts_with("schema: "), "{first}");
    rest
}

/// Every id-line-bearing golden from v9 to v19 is a document written
/// by a real older build. None loads today (v20 made every literal name
/// its unit), and each must refuse Unreadable NAMING the vocabulary it
/// lacks — the deliverable-1 claim on bytes nobody hand-mutated.
#[test]
fn older_build_goldens_refuse_unreadable_and_name_the_break() {
    for (label, golden) in [
        ("v9", V9),
        ("v12", V12),
        ("v15", V15),
        ("v17", V17),
        ("v19", V19),
    ] {
        let text = sans_schema_line(golden);
        let id = header_document_id(text).expect("the id line still reads");
        assert!(text.contains(&format!("\"id\": \"{id}\"")), "{label}");
        let detail = match load(text, Tol::witness()) {
            Err(err @ PersistError::Unreadable { .. }) => {
                let msg = err.to_string();
                assert_eq!(
                    msg.matches(REGENERATE_RECOURSE).count(),
                    1,
                    "{label}: {msg}"
                );
                let PersistError::Unreadable { detail, .. } = err else {
                    unreachable!()
                };
                detail
            }
            other => panic!("{label}: an older build's document must be Unreadable, got {other:?}"),
        };
        eprintln!("[{label}] {detail}");
        // The name-carrying claim: the detail must contain a backticked
        // identifier (a field or variant name), not only a position.
        assert!(
            detail.contains('`'),
            "{label}: the refusal carries no identifier: {detail}"
        );
    }
}

/// Goldens older than the id line (v1–v4) never reach the body: the
/// header door refuses them first, in header terms.
#[test]
fn goldens_that_predate_the_id_line_refuse_at_the_header() {
    for (label, golden) in [("v1", V1), ("v4", V4)] {
        let text = sans_schema_line(golden);
        match load(text, Tol::witness()) {
            Err(PersistError::HeaderId { found }) => {
                eprintln!("[{label}] HeaderId found={found:?}");
            }
            other => panic!("{label}: expected HeaderId, got {other:?}"),
        }
    }
}

// ---- Plumbing: nothing truncates the deserializer's words ----

#[test]
fn display_carries_a_long_detail_untruncated() {
    let detail = format!("unknown variant `{}`", "x".repeat(20_000));
    let err = PersistError::Unreadable {
        line: 1,
        column: 1,
        detail: detail.clone(),
    };
    let msg = err.to_string();
    assert!(msg.contains(&detail));
    assert!(msg.ends_with(REGENERATE_RECOURSE));
}

/// The unknown-variant message enumerates every expected variant, so
/// its length grows with the enum; nothing in the plumbing cuts it.
#[test]
fn the_unknown_variant_detail_lists_the_vocabulary_in_full() {
    let (header, mut v) = split(&small());
    let node = v["snapshot"]["nodes"]
        .as_object_mut()
        .and_then(|nodes| nodes.values_mut().find(|n| n.get("Extrude").is_some()))
        .and_then(serde_json::Value::as_object_mut)
        .expect("the extrude");
    let payload = node.remove("Extrude").unwrap();
    node.insert("Extrudez".to_string(), payload);
    let detail = expect_unreadable_naming("Extrudez", &join(&header, &v), "Extrudez");
    for expected in [
        "`Profile`",
        "`Extrude`",
        "`Mate`",
        "`Measure`",
        "`Assertion`",
    ] {
        assert!(
            detail.contains(expected),
            "{expected} missing from {detail}"
        );
    }
}

// ---- The additive-growth LOAD row, with the ε escape hatch closed ----

/// The unit's frozen minimal-vocabulary bytes, verbatim.
///
/// RE-FROZEN when a profile's sketch plane became a document node.
/// The previous exemplar was written by a real earlier build of this
/// repository and carried a twelve-float placement object in the
/// profile's `plane`; that field is a node reference now, which is a
/// BREAKING wire change, so those bytes are `Unreadable` today. That
/// outcome is the ruling working — a breaking change refuses typed
/// rather than migrating — and it is asserted on the real historical
/// corpus by `bool13r2_probes::real_historical_documents_refuse_typed\
/// _and_name_what_they_lack`. What CANNOT be measured with bytes older
/// than the break is the additive half, so this exemplar is written by
/// today's writer and kept minimal: its node vocabulary is
/// {Datum, Profile, Extrude} and nothing newer, so the row still says
/// that a document lacking every later arm loads. It is re-frozen, by
/// today's writer, at each such break: last when an operation began
/// defining variables, so its table holds the three nodes' outputs.
const OLDER_SHAPED: &str = concat!(
    "id: 12c74470374c7c76269f22a931efab85\n",
    "{\"snapshot\":{\"id\":\"12c74470374c7c76269f22a931efab85\",\"mint\":{\"chain\":\"8bde3a68947a82a5",
    "e7ecd2ecb1681779cc9c25f3f1f03f564d300ff364441baf\",\"log\":[{\"var\":\"1:b639d844bab8e826\"},",
    "{\"var\":\"2:74af9d633a64b77a\"},{\"var\":\"3:fb5fef638c30912c\"},{\"var\":\"4:ee1d73a8dc8f8ad6\"}",
    ",{\"var\":\"5:f9047724cc168290\"},{\"var\":\"6:2fdd1f61b8b90439\"},{\"var\":\"7:5cdde09d996c5c61\"",
    "},{\"var\":\"8:af949e9d2cd2d001\"},{\"var\":\"9:135249424cb8e0e7\"},{\"node\":\"10:54a0180a832755",
    "75\"},{\"var\":\"11:7b9b7a031545bc84\"},{\"var\":\"12:aa1a1f25549ff844\"},{\"var\":\"13:ce0b2c4bfc",
    "e400e2\"},{\"var\":\"14:826a2b7c495117a7\"},{\"var\":\"15:74ce4ff170398ca6\"},{\"var\":\"16:4b36da",
    "afb7685c2b\"},{\"var\":\"17:c9a0e5f82154ce3e\"},{\"var\":\"18:73bc05541afded5a\"},{\"var\":\"19:e5",
    "1d8c264d945b32\"},{\"node\":\"20:d8a099f3b9fb93e6\"},{\"step\":\"21:958ec0488201536d\"},{\"step\"",
    ":\"22:bf987ae2315fc4e3\"},{\"step\":\"23:772d052fb02f7b5e\"},{\"step\":\"24:a99eac6cfba4aecc\"},",
    "{\"step\":\"25:9e75a6604942385f\"},{\"var\":\"26:5cf1574284334ae9\"},{\"var\":\"27:bcd2567cb989e9",
    "fc\"},{\"node\":\"28:2c488f530357dede\"},{\"var\":\"29:8bde3a68947a82a5\"}]},\"nodes\":{\"10:54a01",
    "80a83275575\":{\"Datum\":{\"Frame\":{\"origin\":[\"1:b639d844bab8e826\",\"2:74af9d633a64b77a\",\"3",
    ":fb5fef638c30912c\"],\"u\":[\"4:ee1d73a8dc8f8ad6\",\"5:f9047724cc168290\",\"6:2fdd1f61b8b90439",
    "\"],\"v\":[\"7:5cdde09d996c5c61\",\"8:af949e9d2cd2d001\",\"9:135249424cb8e0e7\"]}}},\"20:d8a099f",
    "3b9fb93e6\":{\"Profile\":{\"plane\":\"10:54a0180a83275575\",\"loops\":[{\"Chain\":[{\"At\":[\"12:aa1",
    "a1f25549ff844\",\"13:ce0b2c4bfce400e2\"]},{\"LineTo\":{\"Point\":[\"14:826a2b7c495117a7\",\"15:7",
    "4ce4ff170398ca6\"]}},{\"LineTo\":{\"Point\":[\"16:4b36daafb7685c2b\",\"17:c9a0e5f82154ce3e\"]}}",
    ",{\"LineTo\":{\"Point\":[\"18:73bc05541afded5a\",\"19:e51d8c264d945b32\"]}},{\"LineTo\":\"Start\"}",
    "]}],\"ids\":[[\"21:958ec0488201536d\",\"22:bf987ae2315fc4e3\",\"23:772d052fb02f7b5e\",\"24:a99e",
    "ac6cfba4aecc\",\"25:9e75a6604942385f\"]]}},\"28:2c488f530357dede\":{\"Extrude\":{\"profile\":\"2",
    "0:d8a099f3b9fb93e6\",\"distance\":\"27:bcd2567cb989e9fc\",\"side\":\"along\"}}},\"roots\":[\"28:2c",
    "488f530357dede\"],\"vars\":{\"1:b639d844bab8e826\":{\"kind\":\"Length\",\"def\":{\"Free\":{\"Continu",
    "ous\":{\"dim\":\"Length\",\"value\":0.0,\"display_unit\":\"m\"}}}},\"2:74af9d633a64b77a\":{\"kind\":\"",
    "Length\",\"def\":{\"Free\":{\"Continuous\":{\"dim\":\"Length\",\"value\":0.0,\"display_unit\":\"m\"}}}}",
    ",\"3:fb5fef638c30912c\":{\"kind\":\"Length\",\"def\":{\"Free\":{\"Continuous\":{\"dim\":\"Length\",\"va",
    "lue\":0.0,\"display_unit\":\"m\"}}}},\"4:ee1d73a8dc8f8ad6\":{\"kind\":\"Scalar\",\"def\":{\"Free\":{\"",
    "Continuous\":{\"dim\":\"Scalar\",\"value\":1.0,\"display_unit\":\"\"}}}},\"5:f9047724cc168290\":{\"k",
    "ind\":\"Scalar\",\"def\":{\"Free\":{\"Continuous\":{\"dim\":\"Scalar\",\"value\":0.0,\"display_unit\":\"",
    "\"}}}},\"6:2fdd1f61b8b90439\":{\"kind\":\"Scalar\",\"def\":{\"Free\":{\"Continuous\":{\"dim\":\"Scalar",
    "\",\"value\":0.0,\"display_unit\":\"\"}}}},\"7:5cdde09d996c5c61\":{\"kind\":\"Scalar\",\"def\":{\"Free",
    "\":{\"Continuous\":{\"dim\":\"Scalar\",\"value\":0.0,\"display_unit\":\"\"}}}},\"8:af949e9d2cd2d001\"",
    ":{\"kind\":\"Scalar\",\"def\":{\"Free\":{\"Continuous\":{\"dim\":\"Scalar\",\"value\":1.0,\"display_uni",
    "t\":\"\"}}}},\"9:135249424cb8e0e7\":{\"kind\":\"Scalar\",\"def\":{\"Free\":{\"Continuous\":{\"dim\":\"Sc",
    "alar\",\"value\":0.0,\"display_unit\":\"\"}}}},\"11:7b9b7a031545bc84\":{\"kind\":\"Frame\",\"def\":{\"",
    "Output\":{\"node\":\"10:54a0180a83275575\",\"port\":0}}},\"12:aa1a1f25549ff844\":{\"kind\":\"Lengt",
    "h\",\"def\":{\"Free\":{\"Continuous\":{\"dim\":\"Length\",\"value\":0.0,\"display_unit\":\"m\"}}}},\"13:",
    "ce0b2c4bfce400e2\":{\"kind\":\"Length\",\"def\":{\"Free\":{\"Continuous\":{\"dim\":\"Length\",\"value\"",
    ":0.0,\"display_unit\":\"m\"}}}},\"14:826a2b7c495117a7\":{\"kind\":\"Length\",\"def\":{\"Free\":{\"Con",
    "tinuous\":{\"dim\":\"Length\",\"value\":1.0,\"display_unit\":\"m\"}}}},\"15:74ce4ff170398ca6\":{\"ki",
    "nd\":\"Length\",\"def\":{\"Free\":{\"Continuous\":{\"dim\":\"Length\",\"value\":0.0,\"display_unit\":\"m",
    "\"}}}},\"16:4b36daafb7685c2b\":{\"kind\":\"Length\",\"def\":{\"Free\":{\"Continuous\":{\"dim\":\"Lengt",
    "h\",\"value\":1.0,\"display_unit\":\"m\"}}}},\"17:c9a0e5f82154ce3e\":{\"kind\":\"Length\",\"def\":{\"F",
    "ree\":{\"Continuous\":{\"dim\":\"Length\",\"value\":1.0,\"display_unit\":\"m\"}}}},\"18:73bc05541afd",
    "ed5a\":{\"kind\":\"Length\",\"def\":{\"Free\":{\"Continuous\":{\"dim\":\"Length\",\"value\":0.0,\"displa",
    "y_unit\":\"m\"}}}},\"19:e51d8c264d945b32\":{\"kind\":\"Length\",\"def\":{\"Free\":{\"Continuous\":{\"d",
    "im\":\"Length\",\"value\":1.0,\"display_unit\":\"m\"}}}},\"26:5cf1574284334ae9\":{\"kind\":\"Profile",
    "\",\"def\":{\"Output\":{\"node\":\"20:d8a099f3b9fb93e6\",\"port\":0}}},\"27:bcd2567cb989e9fc\":{\"ki",
    "nd\":\"Length\",\"def\":{\"Free\":{\"Continuous\":{\"dim\":\"Length\",\"value\":1.0,\"display_unit\":\"m",
    "\"}}}},\"29:8bde3a68947a82a5\":{\"kind\":\"Body\",\"def\":{\"Output\":{\"node\":\"28:2c488f530357ded",
    "e\",\"port\":0}}}},\"epsilon\":1e-09,\"witnesses\":{},\"metadata\":{},\"appearance\":[]},\"edits\":",
    "[]}",
    "\n"
);

/// The unit's row accepts `ToleranceConflict` under a non-default ε so
/// it can run on every CI row; that makes the LOAD half vacuous there.
/// This row re-records the document's ε as the process's, so the load
/// itself is asserted on every row. Also pins that the bytes' node
/// vocabulary is exactly {Profile, Extrude}, rather than only that six
/// named newer arms are absent.
#[test]
fn the_older_shaped_document_loads_at_the_ambient_eps() {
    let eps = Tol::witness().eps();
    let text = OLDER_SHAPED.replacen("\"epsilon\":1e-09", &format!("\"epsilon\":{eps:?}"), 1);
    assert_ne!(text, OLDER_SHAPED);
    let v: serde_json::Value = serde_json::from_str(text.split_once('\n').unwrap().1).unwrap();
    // In document order: the node map is keyed by minted id, which
    // orders as the nodes were inserted.
    let nodes = v["snapshot"]["nodes"].as_object().unwrap();
    let mut keyed: Vec<(editor_core::MintId, &serde_json::Value)> = nodes
        .iter()
        .map(|(id, node)| (editor_core::MintId::parse(id).unwrap(), node))
        .collect();
    keyed.sort_by_key(|&(id, _)| id);
    let tags: Vec<&String> = keyed
        .iter()
        .map(|(_, node)| node.as_object().unwrap().keys().next().unwrap())
        .collect();
    assert_eq!(tags, ["Datum", "Profile", "Extrude"]);
    let loaded = load(&text, Tol::witness()).expect("a minimal-vocabulary document loads");
    assert_eq!(loaded.doc.ids().len(), 3);
    assert_eq!(loaded.doc.epsilon().to_bits(), eps.to_bits());
}

// ---- A break: the same bytes from before outputs refuse ----

/// The minimal-vocabulary bytes as today's writer wrote them before
/// an operation defined variables (INTENT stage 2): the same three
/// nodes, and no output in the table. A break, so they refuse typed
/// with the regenerate recourse rather than load.
const PRE_OUTPUTS: &str = concat!(
    "id: 8ad37e1a750ae77132c0bf059acb322f\n",
    "{\"snapshot\":{\"id\":\"8ad37e1a750ae77132c0bf059acb322f\",\"mint\":{\"chain\":\"b4df12e4a4",
    "1c240fd51afc2c5f7c8122801137c78e464bee6240fe63ab5afab4\",\"log\":[{\"var\":\"1:b639d84",
    "4bab8e826\"},{\"var\":\"2:74af9d633a64b77a\"},{\"var\":\"3:fb5fef638c30912c\"},{\"var\":\"4:",
    "ee1d73a8dc8f8ad6\"},{\"var\":\"5:f9047724cc168290\"},{\"var\":\"6:2fdd1f61b8b90439\"},{\"v",
    "ar\":\"7:5cdde09d996c5c61\"},{\"var\":\"8:af949e9d2cd2d001\"},{\"var\":\"9:135249424cb8e0e",
    "7\"},{\"node\":\"10:54a0180a83275575\"},{\"var\":\"11:d05826096083a72f\"},{\"var\":\"12:3312",
    "b2f8504eeee2\"},{\"var\":\"13:67cd6e8f80d24d4e\"},{\"var\":\"14:fd8954268d1e7222\"},{\"var",
    "\":\"15:4e4dce13c4a56d92\"},{\"var\":\"16:e8e5f1ea6e7306c2\"},{\"var\":\"17:0ad5807e1b5f70",
    "0c\"},{\"var\":\"18:6d679bcadae726fe\"},{\"node\":\"19:f5ee2ea3890bdbd0\"},{\"step\":\"20:05",
    "3eb2ee5f95c60a\"},{\"step\":\"21:dd0913f0e561776c\"},{\"step\":\"22:5ca04880b5f26723\"},{",
    "\"step\":\"23:1c5d6d0a2e5f2352\"},{\"step\":\"24:e1a86d05a90489eb\"},{\"var\":\"25:bdc04106",
    "e3ffa2bc\"},{\"node\":\"26:b4df12e4a41c240f\"}]},\"nodes\":{\"10:54a0180a83275575\":{\"Dat",
    "um\":{\"Frame\":{\"origin\":[\"1:b639d844bab8e826\",\"2:74af9d633a64b77a\",\"3:fb5fef638c3",
    "0912c\"],\"u\":[\"4:ee1d73a8dc8f8ad6\",\"5:f9047724cc168290\",\"6:2fdd1f61b8b90439\"],\"v\"",
    ":[\"7:5cdde09d996c5c61\",\"8:af949e9d2cd2d001\",\"9:135249424cb8e0e7\"]}}},\"19:f5ee2ea",
    "3890bdbd0\":{\"Profile\":{\"plane\":\"10:54a0180a83275575\",\"loops\":[{\"Chain\":[{\"At\":[\"",
    "11:d05826096083a72f\",\"12:3312b2f8504eeee2\"]},{\"LineTo\":{\"Point\":[\"13:67cd6e8f80d",
    "24d4e\",\"14:fd8954268d1e7222\"]}},{\"LineTo\":{\"Point\":[\"15:4e4dce13c4a56d92\",\"16:e8",
    "e5f1ea6e7306c2\"]}},{\"LineTo\":{\"Point\":[\"17:0ad5807e1b5f700c\",\"18:6d679bcadae726f",
    "e\"]}},{\"LineTo\":\"Start\"}]}],\"ids\":[[\"20:053eb2ee5f95c60a\",\"21:dd0913f0e561776c\",",
    "\"22:5ca04880b5f26723\",\"23:1c5d6d0a2e5f2352\",\"24:e1a86d05a90489eb\"]]}},\"26:b4df12",
    "e4a41c240f\":{\"Extrude\":{\"profile\":\"19:f5ee2ea3890bdbd0\",\"distance\":\"25:bdc04106e",
    "3ffa2bc\",\"side\":\"along\"}}},\"roots\":[\"26:b4df12e4a41c240f\"],\"vars\":{\"1:b639d844ba",
    "b8e826\":{\"kind\":\"Length\",\"def\":{\"Free\":{\"Continuous\":{\"dim\":\"Length\",\"value\":0.0",
    ",\"display_unit\":\"m\"}}}},\"2:74af9d633a64b77a\":{\"kind\":\"Length\",\"def\":{\"Free\":{\"Co",
    "ntinuous\":{\"dim\":\"Length\",\"value\":0.0,\"display_unit\":\"m\"}}}},\"3:fb5fef638c30912c",
    "\":{\"kind\":\"Length\",\"def\":{\"Free\":{\"Continuous\":{\"dim\":\"Length\",\"value\":0.0,\"disp",
    "lay_unit\":\"m\"}}}},\"4:ee1d73a8dc8f8ad6\":{\"kind\":\"Scalar\",\"def\":{\"Free\":{\"Continuo",
    "us\":{\"dim\":\"Scalar\",\"value\":1.0,\"display_unit\":\"\"}}}},\"5:f9047724cc168290\":{\"kin",
    "d\":\"Scalar\",\"def\":{\"Free\":{\"Continuous\":{\"dim\":\"Scalar\",\"value\":0.0,\"display_uni",
    "t\":\"\"}}}},\"6:2fdd1f61b8b90439\":{\"kind\":\"Scalar\",\"def\":{\"Free\":{\"Continuous\":{\"di",
    "m\":\"Scalar\",\"value\":0.0,\"display_unit\":\"\"}}}},\"7:5cdde09d996c5c61\":{\"kind\":\"Scal",
    "ar\",\"def\":{\"Free\":{\"Continuous\":{\"dim\":\"Scalar\",\"value\":0.0,\"display_unit\":\"\"}}}",
    "},\"8:af949e9d2cd2d001\":{\"kind\":\"Scalar\",\"def\":{\"Free\":{\"Continuous\":{\"dim\":\"Scal",
    "ar\",\"value\":1.0,\"display_unit\":\"\"}}}},\"9:135249424cb8e0e7\":{\"kind\":\"Scalar\",\"def",
    "\":{\"Free\":{\"Continuous\":{\"dim\":\"Scalar\",\"value\":0.0,\"display_unit\":\"\"}}}},\"11:d0",
    "5826096083a72f\":{\"kind\":\"Length\",\"def\":{\"Free\":{\"Continuous\":{\"dim\":\"Length\",\"va",
    "lue\":0.0,\"display_unit\":\"m\"}}}},\"12:3312b2f8504eeee2\":{\"kind\":\"Length\",\"def\":{\"F",
    "ree\":{\"Continuous\":{\"dim\":\"Length\",\"value\":0.0,\"display_unit\":\"m\"}}}},\"13:67cd6e",
    "8f80d24d4e\":{\"kind\":\"Length\",\"def\":{\"Free\":{\"Continuous\":{\"dim\":\"Length\",\"value\"",
    ":1.0,\"display_unit\":\"m\"}}}},\"14:fd8954268d1e7222\":{\"kind\":\"Length\",\"def\":{\"Free\"",
    ":{\"Continuous\":{\"dim\":\"Length\",\"value\":0.0,\"display_unit\":\"m\"}}}},\"15:4e4dce13c4",
    "a56d92\":{\"kind\":\"Length\",\"def\":{\"Free\":{\"Continuous\":{\"dim\":\"Length\",\"value\":1.0",
    ",\"display_unit\":\"m\"}}}},\"16:e8e5f1ea6e7306c2\":{\"kind\":\"Length\",\"def\":{\"Free\":{\"C",
    "ontinuous\":{\"dim\":\"Length\",\"value\":1.0,\"display_unit\":\"m\"}}}},\"17:0ad5807e1b5f70",
    "0c\":{\"kind\":\"Length\",\"def\":{\"Free\":{\"Continuous\":{\"dim\":\"Length\",\"value\":0.0,\"di",
    "splay_unit\":\"m\"}}}},\"18:6d679bcadae726fe\":{\"kind\":\"Length\",\"def\":{\"Free\":{\"Conti",
    "nuous\":{\"dim\":\"Length\",\"value\":1.0,\"display_unit\":\"m\"}}}},\"25:bdc04106e3ffa2bc\":",
    "{\"kind\":\"Length\",\"def\":{\"Free\":{\"Continuous\":{\"dim\":\"Length\",\"value\":1.0,\"displa",
    "y_unit\":\"m\"}}}}},\"epsilon\":1e-09,\"witnesses\":{},\"metadata\":{},\"appearance\":[]},\"",
    "edits\":[]}",
    "\n"
);

#[test]
fn a_document_from_before_outputs_refuses_at_the_ambient_eps() {
    let eps = Tol::witness().eps();
    let text = PRE_OUTPUTS.replacen("\"epsilon\":1e-09", &format!("\"epsilon\":{eps:?}"), 1);
    assert_ne!(text, PRE_OUTPUTS);
    let v: serde_json::Value = serde_json::from_str(text.split_once('\n').unwrap().1).unwrap();
    // In document order: the node map is keyed by minted id, which
    // orders as the nodes were inserted.
    let nodes = v["snapshot"]["nodes"].as_object().unwrap();
    let mut keyed: Vec<(editor_core::MintId, &serde_json::Value)> = nodes
        .iter()
        .map(|(id, node)| (editor_core::MintId::parse(id).unwrap(), node))
        .collect();
    keyed.sort_by_key(|&(id, _)| id);
    let tags: Vec<&String> = keyed
        .iter()
        .map(|(_, node)| node.as_object().unwrap().keys().next().unwrap())
        .collect();
    assert_eq!(tags, ["Datum", "Profile", "Extrude"]);
    // Written before an operation defined variables, so its table holds
    // none of the three nodes' outputs: the first node's port refuses,
    // with the regenerate recourse, at the ambient ε rather than through
    // the tolerance door.
    let err = load(&text, Tol::witness()).unwrap_err();
    assert!(
        matches!(
            &err,
            PersistError::Snapshot(editor_core::SnapshotError::OutputSignature { fault, .. })
                if matches!(**fault, editor_core::OutputFault::Missing { port: "frame" })
        ),
        "{err:?}"
    );
    assert!(err.to_string().contains(REGENERATE_RECOURSE), "{err}");
}
