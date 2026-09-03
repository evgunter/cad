//! ASM-1 acceptance — document identity + content pins (spec rows
//! 1–5 and 8, each an executable falsifier).
//!
//! The pin is the SHA-256 of the canonical bytes (spec D-3 as
//! AMENDED 2026-08-10, include-by-default): the full replayed
//! snapshot with exactly two exclusions — the edit log and the
//! document id. Metadata and appearance are INCLUDED (rows 4e/4f).
//! The rows below falsify each inclusion and each exclusion
//! separately.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;

use editor_core::{
    Attr, CapEnd, Dimension, DocEdit, DocParam, DocRef, DocumentId, EntityKind, MetaValue, Node,
    ParamName, PersistError, ProfileDoc, Rgba8, RoleSeg, StableName, WitnessDatum, content_pin,
    header_document_id, load, save,
};
use fixture::{desc, insert, len, on_frame, step};
use geom_core::Tol;

/// The shared exemplar: a block (profile + extrude) and a document
/// param, under a derived id.
fn exemplar(
    label: &str,
) -> (
    ProfileDoc,
    editor_core::RecipeNodeId,
    editor_core::RecipeNodeId,
) {
    let doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    let (doc, profile) = on_frame(
        doc,
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![(0.0, 0.0), (2.0, 0.0), (2.0, 1.0), (0.0, 1.0)]],
    );
    let (doc, extrude) = insert(
        doc,
        Node::Extrude {
            profile,
            distance: len(0.5),
        },
    );
    let (doc, _) = step(
        doc,
        DocEdit::SetDocParam {
            name: ParamName::new("depth"),
            value: DocParam::continuous(Dimension::Length, 0.75),
        },
    );
    (doc, profile, extrude)
}

/// Row 1 — pin determinism: the same construction twice hashes to
/// the same pin.
#[test]
fn row1_same_construction_same_pin() {
    let (a, _, _) = exemplar("asm1-row1");
    let (b, _, _) = exemplar("asm1-row1");
    assert_eq!(
        content_pin(&a, Tol::witness()).unwrap(),
        content_pin(&b, Tol::witness()).unwrap()
    );
}

/// Row 2a — history independence: two DIFFERENT edit paths reaching
/// one snapshot pin identically (the log is excluded by
/// construction: the pin is a function of the replayed state, and
/// save/load replays before pinning).
#[test]
fn row2_two_edit_paths_one_snapshot_equal_pins() {
    let (base, _, _) = exemplar("asm1-row2");
    // Path A: set the param to 0.9 in one step.
    let (a, _) = step(
        base.clone(),
        DocEdit::SetDocParam {
            name: ParamName::new("depth"),
            value: DocParam::continuous(Dimension::Length, 0.9),
        },
    );
    // Path B: wander through 0.1 first, then land on 0.9.
    let (b, _) = step(
        base,
        DocEdit::SetDocParam {
            name: ParamName::new("depth"),
            value: DocParam::continuous(Dimension::Length, 0.1),
        },
    );
    let (b, _) = step(
        b,
        DocEdit::SetDocParam {
            name: ParamName::new("depth"),
            value: DocParam::continuous(Dimension::Length, 0.9),
        },
    );
    assert_eq!(
        content_pin(&a, Tol::witness()).unwrap(),
        content_pin(&b, Tol::witness()).unwrap()
    );
    // And through the persistence door: the two saves carry DIFFERENT
    // edit logs over one origin; both load-replay to the same pin.
    let (origin, _, _) = exemplar("asm1-row2");
    let log_a = vec![DocEdit::SetDocParam {
        name: ParamName::new("depth"),
        value: DocParam::continuous(Dimension::Length, 0.9),
    }];
    let mut log_b = vec![DocEdit::SetDocParam {
        name: ParamName::new("depth"),
        value: DocParam::continuous(Dimension::Length, 0.1),
    }];
    log_b.extend(log_a.clone());
    let loaded_a = load(
        &save(&origin, &log_a, Tol::witness()).unwrap(),
        Tol::witness(),
    )
    .unwrap();
    let loaded_b = load(
        &save(&origin, &log_b, Tol::witness()).unwrap(),
        Tol::witness(),
    )
    .unwrap();
    assert_ne!(loaded_a.edits, loaded_b.edits, "the histories differ");
    assert_eq!(
        content_pin(&loaded_a.doc, Tol::witness()).unwrap(),
        content_pin(&loaded_b.doc, Tol::witness()).unwrap()
    );
    // The loaded pin is the EDITED state's pin, not the origin's — a
    // load that skipped replay would degrade both branches to the
    // origin pin and the equality above would hold vacuously (R1
    // NOTE-1); this line is what catches it.
    assert_eq!(
        content_pin(&loaded_a.doc, Tol::witness()).unwrap(),
        content_pin(&a, Tol::witness()).unwrap(),
        "the replayed load pins as the 0.9 state, never the origin"
    );
    assert_ne!(
        content_pin(&loaded_a.doc, Tol::witness()).unwrap(),
        content_pin(&origin, Tol::witness()).unwrap()
    );
}

/// Row 2b — an undone edit leaves the pin unchanged (undo is
/// applying the inverse value; history must not move pins).
#[test]
fn row2_undone_edit_pin_unchanged() {
    let (doc, _, _) = exemplar("asm1-row2b");
    let before = content_pin(&doc, Tol::witness()).unwrap();
    let (edited, _) = step(
        doc,
        DocEdit::SetDocParam {
            name: ParamName::new("depth"),
            value: DocParam::continuous(Dimension::Length, 0.9),
        },
    );
    assert_ne!(
        content_pin(&edited, Tol::witness()).unwrap(),
        before,
        "the edit moved it"
    );
    let (undone, _) = step(
        edited,
        DocEdit::SetDocParam {
            name: ParamName::new("depth"),
            value: DocParam::continuous(Dimension::Length, 0.75),
        },
    );
    assert_eq!(content_pin(&undone, Tol::witness()).unwrap(), before);
}

/// Row 3 (and row 5) — id retarget: equal content under two ids
/// keeps its pin; the ids stay distinct. Identity is not content.
#[test]
fn row3_row5_id_excluded_from_pin() {
    let (a, _, _) = exemplar("asm1-id-a");
    let (b, _, _) = exemplar("asm1-id-b");
    assert_ne!(a.id(), b.id(), "distinct ids");
    assert_eq!(
        content_pin(&a, Tol::witness()).unwrap(),
        content_pin(&b, Tol::witness()).unwrap(),
        "equal content, equal pins — the id answers \"which part\", \
         the pin \"which version\""
    );
}

/// Row 4a — a node edit moves the pin.
#[test]
fn row4_node_edit_moves_pin() {
    let (doc, _, extrude) = exemplar("asm1-row4a");
    let before = content_pin(&doc, Tol::witness()).unwrap();
    let (edited, _) = step(
        doc,
        DocEdit::SetParam {
            node: extrude,
            slot: editor_core::SlotId::Distance,
            expr: len(0.625),
        },
    );
    assert_ne!(content_pin(&edited, Tol::witness()).unwrap(), before);
}

/// Row 4b — a document-param edit moves the pin.
#[test]
fn row4_param_edit_moves_pin() {
    let (doc, _, _) = exemplar("asm1-row4b");
    let before = content_pin(&doc, Tol::witness()).unwrap();
    let (edited, _) = step(
        doc,
        DocEdit::SetDocParam {
            name: ParamName::new("depth"),
            value: DocParam::continuous(Dimension::Length, 0.8),
        },
    );
    assert_ne!(content_pin(&edited, Tol::witness()).unwrap(), before);
}

/// Row 4c — an ε change moves the pin (ε is semantic: the A2 seam).
#[test]
fn row4_epsilon_change_moves_pin() {
    let (doc, _, _) = exemplar("asm1-row4c");
    let before = content_pin(&doc, Tol::witness()).unwrap();
    let eps = doc.epsilon();
    let (edited, _) = step(doc, DocEdit::SetTolerance { eps: eps * 2.0 });
    assert_ne!(content_pin(&edited, Tol::witness()).unwrap(), before);
}

/// Row 4e — an appearance edit moves the pin (D-3 as amended:
/// include-by-default; the spec states the consequence honestly —
/// an appearance-only update re-verifies trivially, accepted v1
/// noise).
#[test]
fn row4_appearance_edit_moves_pin() {
    let (doc, _, extrude) = exemplar("asm1-row4e");
    let before = content_pin(&doc, Tol::witness()).unwrap();
    let cap = StableName {
        kind: EntityKind::Face,
        node: extrude,
        path: vec![RoleSeg::Cap(CapEnd::Top)],
    };
    let (painted, _) = step(
        doc,
        DocEdit::SetAppearance {
            name: cap,
            attr: Attr::Color(Rgba8::opaque(200, 30, 30)),
        },
    );
    assert!(!painted.appearance().is_empty(), "the edit landed");
    assert_ne!(content_pin(&painted, Tol::witness()).unwrap(), before);
}

/// Row 4f — a metadata edit moves the pin (D-3 as amended). The
/// document-level metadata MAP has no edit door in v1, so the
/// executable metadata edit is the appearance record's D7 metadata
/// (`SetAppearanceMeta`) — REPORTED in the PR.
#[test]
fn row4_metadata_edit_moves_pin() {
    let (doc, _, extrude) = exemplar("asm1-row4f");
    let before = content_pin(&doc, Tol::witness()).unwrap();
    let body = StableName {
        kind: EntityKind::Body,
        node: extrude,
        path: vec![],
    };
    let mut m = std::collections::BTreeMap::new();
    m.insert("v".to_owned(), MetaValue::Int(1));
    let (annotated, _) = step(
        doc,
        DocEdit::SetAppearanceMeta {
            name: body,
            key: "tool.example/pin-row".into(),
            value: MetaValue::Map(m),
        },
    );
    assert_ne!(content_pin(&annotated, Tol::witness()).unwrap(), before);
}

/// Row 4d — a witness change moves the pin (branch selection is
/// geometry).
#[test]
fn row4_witness_change_moves_pin() {
    let (doc, profile, _) = exemplar("asm1-row4d");
    let before = content_pin(&doc, Tol::witness()).unwrap();
    let (edited, _) = step(
        doc,
        DocEdit::ReWitness {
            node: profile,
            witness: WitnessDatum {
                schema: 1,
                bytes: b"assignment-v1".to_vec(),
            },
        },
    );
    assert_ne!(content_pin(&edited, Tol::witness()).unwrap(), before);
}

// ---- Door pins beyond the numbered rows ----

/// The save header is the `id: <32 hex>` line, the scan helper reads
/// the id back, and load verifies header/snapshot agreement (tamper
/// refuses typed).
#[test]
fn header_id_line_round_trips_and_tamper_refuses() {
    let (doc, _, _) = exemplar("asm1-header");
    let text = save(&doc, &[], Tol::witness()).unwrap();
    let mut lines = text.lines();
    assert_eq!(lines.next(), Some(&format!("id: {}", doc.id())[..]));
    assert_eq!(header_document_id(&text).unwrap(), doc.id());
    assert_eq!(load(&text, Tol::witness()).unwrap().doc.id(), doc.id());

    // Tamper the header id line: typed IdMismatch naming both ids.
    let other = DocumentId::derive("asm1-header-tampered");
    let tampered = text.replace(&format!("id: {}", doc.id()), &format!("id: {other}"));
    match load(&tampered, Tol::witness()) {
        Err(PersistError::IdMismatch { header, snapshot }) => {
            assert_eq!(header, other);
            assert_eq!(snapshot, doc.id());
        }
        got => panic!("a tampered header id must refuse IdMismatch, got {got:?}"),
    }

    // A missing id line refuses in header terms.
    let headerless = text.replacen(&format!("id: {}\n", doc.id()), "", 1);
    match load(&headerless, Tol::witness()) {
        Err(PersistError::HeaderId { .. }) => {}
        got => panic!("a missing id line must refuse HeaderId, got {got:?}"),
    }
}

/// `DocumentId::derive` is deterministic and label-sensitive; the
/// hex forms are strict-canonical both ways.
#[test]
fn document_id_derive_and_hex_are_canonical() {
    let a = DocumentId::derive("part-a");
    assert_eq!(a, DocumentId::derive("part-a"));
    assert_ne!(a, DocumentId::derive("part-b"));
    assert_eq!(a.hex().len(), 32);
    assert_eq!(DocumentId::parse_hex(&a.hex()), Some(a));
    // Non-canonical spellings refuse: wrong length, uppercase.
    assert_eq!(DocumentId::parse_hex("abc"), None);
    assert_eq!(DocumentId::parse_hex(&a.hex().to_uppercase()), None);
}

/// `DocRef` displays as `id@pin-prefix` and serde round-trips.
#[test]
fn doc_ref_display_and_serde() {
    let (doc, _, _) = exemplar("asm1-docref");
    let r = DocRef {
        id: doc.id(),
        pin: content_pin(&doc, Tol::witness()).unwrap(),
    };
    let shown = r.to_string();
    assert_eq!(shown, format!("{}@{}", doc.id(), &r.pin.hex()[..12]));
    let json = serde_json::to_string(&r).unwrap();
    let back: DocRef = serde_json::from_str(&json).unwrap();
    assert_eq!(back, r);
}

/// Doc-level metadata is IN the preimage (row 4 as amended; the
/// dual review's converged finding: silently dropping the key from
/// the canonical form passed the whole suite). The map has no edit
/// door in v1, so the falsifier goes through the persistence door —
/// a crafted save whose body carries non-empty `"metadata"` loads
/// clean and pins DIFFERENTLY from its metadata-empty twin.
#[test]
fn row4_doc_metadata_in_preimage_via_crafted_save() {
    let (doc, _, _) = exemplar("asm1-row4-meta");
    let text = save(&doc, &[], Tol::witness()).unwrap();
    let needle = "\"metadata\": {}";
    assert_eq!(
        text.matches(needle).count(),
        1,
        "exactly the snapshot's one empty metadata map"
    );
    let crafted = text.replace(needle, "\"metadata\": {\"units\": \"mm\"}");
    let twin = load(&text, Tol::witness()).unwrap();
    let loaded = load(&crafted, Tol::witness()).unwrap();
    assert_eq!(
        loaded.doc.metadata().get("units").map(String::as_str),
        Some("mm"),
        "the crafted map survived the load doors"
    );
    assert_ne!(
        content_pin(&loaded.doc, Tol::witness()).unwrap(),
        content_pin(&twin.doc, Tol::witness()).unwrap(),
        "metadata is content: dropping it from the preimage must fail HERE"
    );
}

/// The spec's stated `next_id` consequence (D-3 as amended; R2
/// MINOR-3, ruled compliant): an undone INSERT moves the pin —
/// delete never decrements the monotone counter, and the counter is
/// document state in the include-by-default preimage. "Undo must not
/// move pins" holds exactly for value edits (row 2b); structural
/// insert/delete pairs leave counter residue. Documented behavior,
/// pinned so a silent preimage change is caught in both directions.
#[test]
fn stated_consequence_undone_insert_moves_pin() {
    let (doc, _, _) = exemplar("asm1-next-id");
    let nodes_before = doc.len();
    let before = content_pin(&doc, Tol::witness()).unwrap();
    // One node in, one node out: the profile alone, on a frame the
    // document already carries, so the delete restores the count.
    // The exemplar's own frame: node 0, ahead of its profile.
    let plane = doc.order()[0];
    let (with_extra, extra) = insert(
        doc,
        Node::Profile(desc(
            plane,
            vec![vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)]],
        )),
    );
    let (undone, _) = step(with_extra, DocEdit::DeleteNode { id: extra });
    assert_eq!(undone.len(), nodes_before, "the node itself is gone");
    assert_ne!(
        content_pin(&undone, Tol::witness()).unwrap(),
        before,
        "counter residue pins as a new version — the spec's stated consequence"
    );
}
