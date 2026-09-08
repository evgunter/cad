//! R1 review probes for BOOL-10 (PR #2135, frozen head 3f8163dd8) —
//! the wire half: what `Step::ArcTo { spec, splits }` did to the
//! persisted shape, measured on the committed corpus document.
//!
//! Review artefact, not a unit deliverable: every row records what the
//! frozen head DOES, so a row going red on a later head is
//! information, not necessarily a defect.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::path::Path;

use editor_core::{PersistError, load};
use geom_core::Tol;

/// The committed die-tool document, whose one arc leg carries the new
/// shape.
fn die_tool() -> String {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/corpus/die_tool.pncad");
    std::fs::read_to_string(&path).expect("the committed die-tool document reads")
}

fn split_header(text: &str) -> (String, serde_json::Value) {
    let (id_line, body) = text.split_once('\n').unwrap();
    (
        format!("{id_line}\n"),
        serde_json::from_str(body).expect("a save's body is JSON"),
    )
}

fn join(header: &str, v: &serde_json::Value) -> String {
    format!("{header}{}\n", serde_json::to_string_pretty(v).unwrap())
}

/// Rewrites every `"ArcTo": {"spec": S, "splits": N}` back to the
/// pre-BOOL-10 newtype shape `"ArcTo": S`, in place.
fn to_pre_bool10_shape(v: &mut serde_json::Value, hits: &mut usize) {
    match v {
        serde_json::Value::Object(map) => {
            if let Some(serde_json::Value::Object(inner)) = map.get("ArcTo")
                && inner.contains_key("spec")
                && inner.contains_key("splits")
            {
                let spec = inner.get("spec").unwrap().clone();
                map.insert("ArcTo".to_owned(), spec);
                *hits += 1;
                return;
            }
            for (_, child) in map.iter_mut() {
                to_pre_bool10_shape(child, hits);
            }
        }
        serde_json::Value::Array(items) => {
            for child in items {
                to_pre_bool10_shape(child, hits);
            }
        }
        _ => {}
    }
}

/// Drops the `"splits"` field, keeping the struct shape — the question
/// "is the new field OPTIONAL on read".
fn drop_splits(v: &mut serde_json::Value, hits: &mut usize) {
    match v {
        serde_json::Value::Object(map) => {
            if let Some(serde_json::Value::Object(inner)) = map.get_mut("ArcTo")
                && inner.remove("splits").is_some()
            {
                *hits += 1;
                return;
            }
            for (_, child) in map.iter_mut() {
                drop_splits(child, hits);
            }
        }
        serde_json::Value::Array(items) => {
            for child in items {
                drop_splits(child, hits);
            }
        }
        _ => {}
    }
}

/// **The committed document loads, and its arc leg carries `splits`.**
/// The baseline the two mutations below are measured against.
#[test]
fn the_committed_document_carries_the_split_count_and_loads() {
    let text = die_tool();
    assert!(
        text.contains("\"splits\": 1"),
        "the committed die-tool document should carry the new field"
    );
    load(&text, Tol::witness()).expect("the committed document loads at this head");
}

/// **`splits` is REQUIRED on read, not optional.** A document whose
/// `ArcTo` keeps the struct shape but omits the field does not load.
/// This is the answer to "is the field append-only-optional": it is
/// not — the load refuses typed rather than defaulting to the plain
/// leg.
#[test]
fn a_document_without_the_splits_field_does_not_load() {
    let (header, mut body) = split_header(&die_tool());
    let mut hits = 0;
    drop_splits(&mut body, &mut hits);
    assert_eq!(hits, 1, "the fixture has exactly one arc leg");
    let text = join(&header, &body);
    match load(&text, Tol::witness()) {
        Err(err @ PersistError::Unreadable { .. }) => {
            eprintln!("[no splits] {err}");
        }
        other => panic!("a document missing `splits` must refuse Unreadable, got {other:?}"),
    }
}

/// **A pre-BOOL-10 document does not load either.** The variant went
/// from a newtype to a struct, so every document an earlier build
/// wrote with an arc leg is unreadable at this head — the change is a
/// FORMAT BREAK, not an additive field. Nothing in tree still carries
/// the old shape (the corpus was re-authored), so this row is the
/// record of what the break costs, on bytes derived from a real
/// committed document.
#[test]
fn a_pre_bool10_arc_to_shape_does_not_load() {
    let (header, mut body) = split_header(&die_tool());
    let mut hits = 0;
    to_pre_bool10_shape(&mut body, &mut hits);
    assert_eq!(hits, 1, "the fixture has exactly one arc leg");
    let text = join(&header, &body);
    match load(&text, Tol::witness()) {
        Err(err @ PersistError::Unreadable { .. }) => {
            eprintln!("[pre-BOOL-10 shape] {err}");
        }
        other => panic!("the pre-BOOL-10 arc shape must refuse Unreadable, got {other:?}"),
    }
}
