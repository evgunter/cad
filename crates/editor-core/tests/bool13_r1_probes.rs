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
        // The profile and the extrude, both.
        Ok(loaded) => assert_eq!(loaded.doc.ids().len(), 2),
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
        "`Place`",
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
/// {Profile, Extrude} and nothing newer, so the row still says that a
/// document lacking every later arm loads. It is re-frozen, by today's
/// writer, at each such break: last when a profile became 2-D and the
/// frame node it was drawn on retired, so the document is the profile
/// and the extrude alone, the extrude reading the profile's output; and
/// again when a construction came to define its space beside its body
/// (D′1), whose row the table now holds.
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
    assert_eq!(tags, ["Profile", "Extrude"]);
    let loaded = load(&text, Tol::witness()).expect("a minimal-vocabulary document loads");
    assert_eq!(loaded.doc.ids().len(), 2);
    assert_eq!(loaded.doc.epsilon().to_bits(), eps.to_bits());
}

// ---- A break: the same bytes from before outputs refuse ----

/// The minimal-vocabulary bytes in the shape an operation had before
/// it defined variables (INTENT stage 2): the same two nodes, no
/// output in the table, and the extrude naming the profile node
/// itself. A break, so they refuse typed with the regenerate recourse
/// rather than load. They are today's save with only that taken out,
/// so the bytes reach the outputs walk this row is about rather than
/// the reader's field check.
const PRE_OUTPUTS: &str = concat!(
    "id: c7e37d80a53df4cb1d21656a27d56c2d\n",
    "{\"snapshot\":{\"id\":\"c7e37d80a53df4cb1d21656a27d56c2d\",\"mint\":{\"chain\":\"60",
    "8252c10d916cfe5e1e6507e191371ccec1d1b04280e057679688bb077d4631\",\"log\":[{",
    "\"var\":\"1:b639d844bab8e826\"},{\"var\":\"2:74af9d633a64b77a\"},{\"var\":\"3:e49b8",
    "3e35d1cfaf8\"},{\"var\":\"4:9c83dccc6e32d7b3\"},{\"var\":\"5:d786bf1ceccb8f00\"},",
    "{\"var\":\"6:23ee116d75b4556b\"},{\"var\":\"7:455ba71f3c08cba1\"},{\"var\":\"8:4665",
    "cdc912ba37b9\"},{\"node\":\"9:b5f34a51587e847a\"},{\"step\":\"10:f91a0724eaf428d",
    "4\"},{\"step\":\"11:1727f41743e500c9\"},{\"step\":\"12:40ebf48fc1ce9abb\"},{\"step",
    "\":\"13:5515fee17d00383e\"},{\"step\":\"14:74e05563fbf673eb\"},{\"var\":\"15:11be5",
    "9149cf7b592\"},{\"var\":\"16:a2264ed7bb3bdcb7\"},{\"node\":\"17:4a08cd5c982f47a4",
    "\"},{\"var\":\"18:608252c10d916cfe\"}]},\"nodes\":{\"9:b5f34a51587e847a\":{\"Profi",
    "le\":{\"loops\":[{\"Chain\":[{\"At\":[\"1:b639d844bab8e826\",\"2:74af9d633a64b77a\"",
    "]},{\"LineTo\":{\"Point\":[\"3:e49b83e35d1cfaf8\",\"4:9c83dccc6e32d7b3\"]}},{\"Li",
    "neTo\":{\"Point\":[\"5:d786bf1ceccb8f00\",\"6:23ee116d75b4556b\"]}},{\"LineTo\":{",
    "\"Point\":[\"7:455ba71f3c08cba1\",\"8:4665cdc912ba37b9\"]}},{\"LineTo\":\"Start\"}",
    "]}],\"ids\":[[\"10:f91a0724eaf428d4\",\"11:1727f41743e500c9\",\"12:40ebf48fc1ce",
    "9abb\",\"13:5515fee17d00383e\",\"14:74e05563fbf673eb\"]]}},\"17:4a08cd5c982f47",
    "a4\":{\"Extrude\":{\"profile\":\"9:b5f34a51587e847a\",\"distance\":\"16:a2264ed7bb",
    "3bdcb7\"}}},\"vars\":{\"1:b639d844bab8e826\":{\"kind\":\"Length\",\"def\":{\"Free\":{",
    "\"Continuous\":{\"dim\":\"Length\",\"value\":0.0,\"display_unit\":\"m\"}}}},\"2:74af9",
    "d633a64b77a\":{\"kind\":\"Length\",\"def\":{\"Free\":{\"Continuous\":{\"dim\":\"Length",
    "\",\"value\":0.0,\"display_unit\":\"m\"}}}},\"3:e49b83e35d1cfaf8\":{\"kind\":\"Lengt",
    "h\",\"def\":{\"Free\":{\"Continuous\":{\"dim\":\"Length\",\"value\":1.0,\"display_unit",
    "\":\"m\"}}}},\"4:9c83dccc6e32d7b3\":{\"kind\":\"Length\",\"def\":{\"Free\":{\"Continuo",
    "us\":{\"dim\":\"Length\",\"value\":0.0,\"display_unit\":\"m\"}}}},\"5:d786bf1ceccb8f",
    "00\":{\"kind\":\"Length\",\"def\":{\"Free\":{\"Continuous\":{\"dim\":\"Length\",\"value\"",
    ":1.0,\"display_unit\":\"m\"}}}},\"6:23ee116d75b4556b\":{\"kind\":\"Length\",\"def\":",
    "{\"Free\":{\"Continuous\":{\"dim\":\"Length\",\"value\":1.0,\"display_unit\":\"m\"}}}}",
    ",\"7:455ba71f3c08cba1\":{\"kind\":\"Length\",\"def\":{\"Free\":{\"Continuous\":{\"dim",
    "\":\"Length\",\"value\":0.0,\"display_unit\":\"m\"}}}},\"8:4665cdc912ba37b9\":{\"kin",
    "d\":\"Length\",\"def\":{\"Free\":{\"Continuous\":{\"dim\":\"Length\",\"value\":1.0,\"dis",
    "play_unit\":\"m\"}}}},\"16:a2264ed7bb3bdcb7\":{\"kind\":\"Length\",\"def\":{\"Free\":",
    "{\"Continuous\":{\"dim\":\"Length\",\"value\":1.0,\"display_unit\":\"m\"}}}}},\"epsil",
    "on\":1e-09,\"witnesses\":{},\"metadata\":{},\"appearance\":[]},\"edits\":[]}",
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
    assert_eq!(tags, ["Profile", "Extrude"]);
    // Written before an operation defined variables, so its table holds
    // neither node's output: the first node's port refuses,
    // with the regenerate recourse, at the ambient ε rather than through
    // the tolerance door.
    let err = load(&text, Tol::witness()).unwrap_err();
    assert!(
        matches!(
            &err,
            PersistError::Snapshot(editor_core::SnapshotError::OutputSignature { fault, .. })
                if matches!(**fault, editor_core::OutputFault::Missing { port: "profile" })
        ),
        "{err:?}"
    );
    assert!(err.to_string().contains(REGENERATE_RECOURSE), "{err}");
}
