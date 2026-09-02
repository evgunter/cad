//! BOOL-13 review probes (R1): the `Unreadable` / `Parse` seam attacked
//! from both sides, the name-carrying claim checked against REAL
//! older-build bytes (the goldens the unit deleted, re-staged under
//! `bool13r1_goldens/`), and the additive-growth LOAD row asserted at
//! the ambient ε so it cannot pass through the `ToleranceConflict`
//! escape hatch.
//!
//! Review artefact, not a unit deliverable: every row records what the
//! frozen head DOES, so a row going red on a later head is information,
//! not necessarily a defect.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod fixture;

use editor_core::{
    Node, PersistError, ProfileDoc, REGENERATE_RECOURSE, header_document_id, load, save,
};
use fixture::{desc, insert, len};
use geom_core::Tol;

/// A profile and an extrude (the same shape `unreadable_by_this_build`
/// mutates).
fn small() -> String {
    let doc = ProfileDoc::empty_derived("bool13-r1-probes", Tol::witness());
    let (doc, profile) = insert(
        doc,
        Node::Profile(desc(
            [0.0; 3],
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            vec![vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)]],
        )),
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
/// naming the key — and hands the reader the "regenerate with a current
/// build" recourse, which for a newer file is advice about the wrong
/// side of the seam. Recorded, not judged here.
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

/// Deep nesting never reaches serde_json's recursion limit (a `Syntax`
/// class): the typed visitor meets the first wrong-typed token and
/// refuses `Data` at depth three. So a nesting bomb is Unreadable, with
/// the recourse, not Parse.
#[test]
fn deep_nesting_is_unreadable_because_the_type_fails_first() {
    let (header, _) = split(&small());
    let deep = format!("{header}{{\"snapshot\": {}}}\n", "[".repeat(300));
    expect_unreadable_naming("deep nesting", &deep, "invalid type: sequence");
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
        Ok(loaded) => assert_eq!(loaded.doc.order().len(), 2),
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
    // Duplicate the whole `"1": {...}` node entry by re-parsing and
    // re-emitting the nodes object with the key twice (serde_json's
    // Value cannot hold duplicates, so splice text).
    let needle = "\"nodes\": {";
    let at = body.find(needle).expect("a nodes section") + needle.len();
    let v: serde_json::Value = serde_json::from_str(body).unwrap();
    let node0 = serde_json::to_string(&v["snapshot"]["nodes"]["0"]).unwrap();
    let spliced = format!("{header}\n{}\"0\": {node0},{}", &body[..at], &body[at..]);
    expect_unreadable_naming(
        "duplicate node key",
        &spliced,
        "duplicate snapshot node key",
    );
}

// ---- Real older-build bytes: the goldens the unit deleted ----

const V1: &str = include_str!("bool13r1_goldens/v1_golden.cad");
const V4: &str = include_str!("bool13r1_goldens/v4_golden.cad");
const V9: &str = include_str!("bool13r1_goldens/v9_golden.cad");
const V12: &str = include_str!("bool13r1_goldens/v12_golden.cad");
const V15: &str = include_str!("bool13r1_goldens/v15_golden.cad");
const V17: &str = include_str!("bool13r1_goldens/v17_golden.cad");
const V19: &str = include_str!("bool13r1_goldens/v19_golden.cad");

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
    let node = v["snapshot"]["nodes"]["1"].as_object_mut().unwrap();
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

/// The unit's frozen older-shaped bytes, verbatim.
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
    let tags: Vec<&String> = v["snapshot"]["nodes"]
        .as_object()
        .unwrap()
        .values()
        .map(|n| n.as_object().unwrap().keys().next().unwrap())
        .collect();
    assert_eq!(tags, ["Profile", "Extrude"]);
    let loaded = load(&text, Tol::witness()).expect("an older-shaped document loads");
    assert_eq!(loaded.doc.order().len(), 2);
    assert_eq!(loaded.doc.epsilon().to_bits(), eps.to_bits());
}
