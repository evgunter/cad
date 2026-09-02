//! Reviewer probes for the schema demolition's one door.
//!
//! The unit's own family asserts the door on documents this build's
//! serializer produced (mutated) and on one frozen hand-written body.
//! These rows attack the same seam from directions that family does
//! not reach:
//!
//! - REAL historical documents, byte-for-byte as earlier builds of
//!   this repo wrote them, with only their `schema:` line removed —
//!   the load side of "additive growth invalidates nothing" measured
//!   against actual history rather than against a body written today.
//! - The OTHER direction of growth: a document naming a field this
//!   build does not have (a newer file read by an older build).
//! - The classification seam's edges: duplicate keys, a non-optional
//!   field removed, deep nesting, an out-of-range float literal.
//! - serde's own missing-field semantics, which is the premise the
//!   persist module docs rest "a new optional field invalidates
//!   nothing" on.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod fixture;

use editor_core::{Node, PersistError, ProfileDoc, REGENERATE_RECOURSE, load, save};
use fixture::{insert, len, on_frame};
use geom_core::Tol;

/// A profile + extrude save, the smallest body with a required field.
fn small() -> String {
    let doc = ProfileDoc::empty_derived("bool13r2-probes", Tol::witness());
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

fn fixtures_dir() -> std::path::PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("bool13_goldens")
}

/// Documents this repository's own earlier builds wrote, byte for
/// byte, with only their `schema:` line removed — the load side of
/// "additive growth invalidates nothing" measured against actual
/// history rather than against a body written today.
///
/// The row does not require them to load, and as of this head none of
/// the nineteen does (the three kept here are representatives; the
/// census was run over all of them). TWO breaking changes account for
/// that, and either one is enough on its own: a literal's display unit
/// became non-optional, so every pre-`v20` document misses a required
/// field, and a profile's plane became a NODE REFERENCE, so every
/// document that carries a placement object there is unreadable too.
/// The four oldest predate the `id:` header line entirely. What it
/// pins is the property the ruling
/// actually bought — that the outcome is TYPED and self-describing on
/// real bytes: the body refusals name the field they could not place
/// and carry the recourse exactly once, the header refusals say the
/// header is the problem, and nothing lands on `Parse` (these are
/// valid JSON; a `Parse` here would mean the seam mis-classifies real
/// history) or panics.
///
/// It goes red if a refusal stops carrying the offending NAME —
/// which is the whole premise deliverable 1 verified and the whole
/// reason no version number is needed.
#[test]
fn real_historical_documents_refuse_typed_and_name_what_they_lack() {
    let dir = fixtures_dir();
    let mut entries: Vec<_> = std::fs::read_dir(&dir)
        .unwrap_or_else(|e| panic!("{}: {e}", dir.display()))
        .map(|e| e.expect("dir entry").path())
        .filter(|p| p.extension().is_some_and(|x| x == "cad"))
        .collect();
    entries.sort();
    assert!(!entries.is_empty(), "the fixture directory is populated");
    let mut named = 0usize;
    for path in &entries {
        let raw = std::fs::read_to_string(path).expect("readable");
        let text = raw.strip_prefix("schema: ").map_or(raw.clone(), |rest| {
            rest.split_once('\n').expect("a header line").1.to_string()
        });
        let name = path.file_name().unwrap().to_string_lossy().to_string();
        match load(&text, Tol::witness()) {
            // Additive growth would put a historical document here.
            // The recorded ε is the document's, so under a non-default
            // ε row the parse, the validator and the replay all ran and
            // only the LAST door refused — the same evidence.
            Ok(_) | Err(PersistError::ToleranceConflict { .. }) => {}
            Err(err @ PersistError::Unreadable { .. }) => {
                let msg = err.to_string();
                // The NAME, not which name: two different fields do
                // the naming across this set (a missing `unit`, and a
                // `plane` that is a node id now), and pinning one of
                // them would make the row about the format's history
                // rather than about the refusal being self-describing.
                assert!(
                    msg.contains('`'),
                    "{name}: the refusal must name the vocabulary it could not place: {msg}"
                );
                assert_eq!(msg.matches(REGENERATE_RECOURSE).count(), 1, "{name}: {msg}");
                named += 1;
            }
            // The oldest shapes predate the `id:` line: the header
            // door refuses them in header terms, and — since the
            // review — with the same recourse the body door gives,
            // because a pre-id document is a file this build cannot
            // read too.
            Err(err @ PersistError::HeaderId { .. }) => {
                let msg = err.to_string();
                assert!(msg.contains("id:"), "{name}: {msg}");
                assert_eq!(msg.matches(REGENERATE_RECOURSE).count(), 1, "{name}: {msg}");
            }
            Err(other) => panic!("{name}: a historical document refused untyped: {other:?}"),
        }
    }
    assert!(
        named >= 1,
        "at least one historical document exercises the naming refusal"
    );
}

/// Growth in the OTHER direction: a document naming a field this
/// build has no place for — what an older build meets when it is
/// handed a newer file. It refuses `Unreadable` with the recourse,
/// naming the field; it is never silently ignored.
#[test]
fn a_field_this_build_does_not_know_refuses_rather_than_being_ignored() {
    let (header, mut v) = split(&small());
    v.as_object_mut()
        .unwrap()
        .insert("provenance".into(), serde_json::json!("a newer build"));
    let outer = join(&header, &v);
    match load(&outer, Tol::witness()) {
        Err(err @ PersistError::Unreadable { .. }) => {
            let msg = err.to_string();
            assert!(msg.contains("provenance"), "{msg}");
            assert_eq!(msg.matches(REGENERATE_RECOURSE).count(), 1, "{msg}");
        }
        other => panic!("an unknown top-level field must refuse unreadable, got {other:?}"),
    }
    // And one level down, inside the snapshot.
    let (header, mut v) = split(&small());
    v["snapshot"]
        .as_object_mut()
        .unwrap()
        .insert("annotations".into(), serde_json::json!([]));
    let inner = join(&header, &v);
    match load(&inner, Tol::witness()) {
        Err(err @ PersistError::Unreadable { .. }) => {
            assert!(err.to_string().contains("annotations"), "{err}");
        }
        other => panic!("an unknown snapshot field must refuse unreadable, got {other:?}"),
    }
}

/// A non-optional STRUCT field removed — the shape of a document
/// written before that field existed. This is the half of "an older
/// document" that does NOT load, and the door names the field, which
/// is what the ruling asks of a breaking change.
#[test]
fn a_document_predating_a_non_optional_field_refuses_naming_it() {
    let (header, mut v) = split(&small());
    let removed = v["snapshot"].as_object_mut().unwrap().remove("roots");
    assert!(removed.is_some(), "the fixture carries `roots`");
    let text = join(&header, &v);
    match load(&text, Tol::witness()) {
        Err(err @ PersistError::Unreadable { .. }) => {
            let msg = err.to_string();
            assert!(msg.contains("missing field `roots`"), "{msg}");
            assert_eq!(msg.matches(REGENERATE_RECOURSE).count(), 1, "{msg}");
        }
        other => panic!("a missing non-optional field must refuse unreadable, got {other:?}"),
    }
}

/// The premise the module docs rest "a new OPTIONAL field invalidates
/// nothing" on is serde's, not this crate's: a missing `Option<T>`
/// field deserializes to `None` even under `deny_unknown_fields`,
/// while a missing plain field does not. Pinned here because the
/// sentence in `persist/mod.rs` is load-bearing for the whole
/// no-version argument and nothing else in the tree checks it.
#[test]
fn serdes_missing_field_semantics_are_what_the_module_docs_assume() {
    #[derive(serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    struct Grown {
        kept: u8,
        added: Option<u8>,
        #[serde(default)]
        defaulted: u8,
    }
    let older: Grown = serde_json::from_str(r#"{"kept":1}"#).expect("older shape loads");
    assert_eq!((older.kept, older.added, older.defaulted), (1, None, 0));

    #[derive(Debug, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    #[allow(dead_code)]
    struct GrownPlain {
        kept: u8,
        added: u8,
    }
    let err = serde_json::from_str::<GrownPlain>(r#"{"kept":1}"#).expect_err("plain field is not");
    assert_eq!(err.classify(), serde_json::error::Category::Data);
    assert!(err.to_string().contains("missing field `added`"), "{err}");
}

/// The classification seam's edges, each pinned to the arm it lands
/// on. A duplicate key is `Data` (valid JSON, refused by the types);
/// nesting past serde_json's recursion limit and a bare `NaN` token
/// are not JSON at all.
#[test]
fn the_classification_seam_holds_at_its_edges() {
    let (header, v) = split(&small());
    let pretty = serde_json::to_string_pretty(&v).unwrap();

    // Duplicate top-level key: `deny_unknown_fields` makes it a
    // duplicate-field refusal, not a last-wins silent accept.
    assert!(
        pretty.starts_with("{\n  \"edits\""),
        "serde_json orders keys: {pretty:.40}"
    );
    let dup = format!(
        "{header}{}\n",
        pretty.replacen('{', "{\n  \"edits\": [],", 1)
    );
    match load(&dup, Tol::witness()) {
        Err(err @ PersistError::Unreadable { .. }) => {
            assert!(err.to_string().contains("duplicate field"), "{err}");
        }
        other => panic!("a duplicate key must refuse unreadable, got {other:?}"),
    }

    // Deep nesting: serde_json's recursion limit is a SYNTAX class.
    let deep = format!("{}{}{}\n", header, "[".repeat(600), "]".repeat(600));
    match load(&deep, Tol::witness()) {
        Err(PersistError::Parse { .. } | PersistError::Unreadable { .. }) => {}
        other => panic!("deep nesting must refuse typed, got {other:?}"),
    }

    // Non-finite tokens are not JSON: they refuse before the types.
    for body in [
        "{\"snapshot\": NaN, \"edits\": []}",
        "{\"snapshot\": Infinity, \"edits\": []}",
    ] {
        let text = format!("{header}{body}\n");
        match load(&text, Tol::witness()) {
            Err(err @ PersistError::Parse { .. }) => {
                assert!(!err.to_string().contains(REGENERATE_RECOURSE), "{err}");
            }
            other => panic!("{body:?} must refuse Parse, got {other:?}"),
        }
    }
}

/// The module docs say a non-finite float is "unreachable post-parse
/// — JSON carries no non-finite tokens". JSON carries no non-finite
/// TOKENS, but it carries decimal literals that OVERFLOW `f64`, and
/// nothing in the tree checked what happens to one. This row is that
/// check, in the recorded-ε slot.
#[test]
fn an_overflowing_float_literal_lands_somewhere_typed() {
    // A sentinel rather than the recorded ε's own spelling: the ε a
    // save records is the AMBIENT one and its rendering differs per
    // CI tolerance row.
    let (header, mut v) = split(&small());
    v["snapshot"]["epsilon"] = serde_json::json!(-98765.0);
    let pretty = serde_json::to_string_pretty(&v).unwrap();
    let text = format!("{header}{}\n", pretty.replacen("-98765.0", "1e999", 1));
    assert!(text.contains("1e999"), "the literal is in the body");
    match load(&text, Tol::witness()) {
        // serde_json refuses the literal at the TOKEN ("number out of
        // range"), classified `Syntax`, so the module docs' claim
        // holds: no non-finite value survives the parser, and
        // `NonFinite` really is unreachable post-parse. If a future
        // parser change made an overflow round to infinity instead,
        // this row goes red and the sentence needs revisiting.
        Err(err @ PersistError::Parse { .. }) => {
            assert!(err.to_string().contains("out of range"), "{err}");
        }
        other => panic!("an overflowing literal must refuse at the token, got {other:?}"),
    }
}

/// The recourse rides exactly the two arms a stale document can land
/// on — `Unreadable` (the body) and `HeaderId` (a pre-id header) —
/// EXACTLY once each, and no other arm carries it: not `Parse` (the
/// reader refused the bytes; regeneration is not the diagnosis) and
/// not `IdMismatch` (a tampered file).
#[test]
fn only_the_stale_document_arms_carry_the_recourse_and_carry_it_once() {
    let (header, v) = split(&small());
    let body = serde_json::to_string_pretty(&v).unwrap();
    let cases: Vec<String> = vec![
        // No header at all.
        body.clone(),
        // Header, non-JSON body.
        format!("{header}not json\n"),
        // Header id that no snapshot claims.
        format!("id: {}\n{body}\n", "0".repeat(32)),
    ];
    for text in cases {
        if let Err(err) = load(&text, Tol::witness()) {
            let msg = err.to_string();
            let count = msg.matches(REGENERATE_RECOURSE).count();
            match err {
                PersistError::Unreadable { .. } | PersistError::HeaderId { .. } => {
                    assert_eq!(count, 1, "{msg}");
                }
                _ => assert_eq!(count, 0, "{msg}"),
            }
        }
    }
}
