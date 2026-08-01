//! M4 PR 6 spec D6.3 — the refusal CI row: unknown schema versions
//! and corrupt/truncated files refuse TYPED (position info included);
//! non-finite floats refuse typed at SAVE naming the site; the D4
//! one-process-one-ε doors refuse loudly. No silent best-effort
//! loads, ever.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod fixture;

use editor_core::persist::SnapshotError;
use editor_core::{
    CancelToken, Dimension, DocEdit, DocParam, EvalOptions, MetaValue, Node, NodeErrorKind,
    NodeResult, ParamName, PersistError, ProfileDoc, RecipeNodeId, SCHEMA_VERSION,
    WitnessDatum, apply, evaluate, load, save,
};
use fixture::{desc, insert, len};

/// A small valid document (profile + extrude + witness) and its save.
fn small() -> (ProfileDoc, String) {
    let doc = ProfileDoc::empty();
    let (doc, p) = insert(
        doc,
        Node::Profile(desc(
            [0.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            vec![vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)]],
        )),
    );
    let (doc, _) = insert(
        doc,
        Node::Extrude {
            profile: p,
            distance: len(1.0),
        },
    );
    let doc = apply(
        &doc,
        &DocEdit::ReWitness {
            node: p,
            witness: WitnessDatum {
                schema: 1,
                bytes: vec![0xab, 0xcd],
            },
        },
    )
    .expect("witness")
    .doc;
    let text = save(&doc, &[]).expect("save");
    (doc, text)
}

#[test]
fn unknown_schema_version_refuses_typed() {
    let (_, text) = small();
    let body = text.split_once('\n').expect("header").1;
    // Forward-only (D6.3): a version NEWER than this build is
    // `UnknownSchema` — held one past `SCHEMA_VERSION` so the row
    // survives every schema bump (M5 PR 10 moved it from a literal 2).
    let newer = format!("schema: {}\n{body}", SCHEMA_VERSION + 1);
    match load(&newer) {
        Err(PersistError::UnknownSchema { found, newest }) => {
            assert_eq!(found, u64::from(SCHEMA_VERSION) + 1);
            assert_eq!(newest, SCHEMA_VERSION);
        }
        other => panic!("expected UnknownSchema, got {other:?}"),
    }
    match load("schema: 0\n{}") {
        Err(PersistError::UnknownSchema { found: 0, .. }) => {}
        other => panic!("expected UnknownSchema for v0, got {other:?}"),
    }
}

#[test]
fn missing_or_garbled_header_refuses_typed() {
    for text in ["", "not a save file", "schema: banana\n{}"] {
        match load(text) {
            Err(PersistError::Header { .. }) => {}
            other => panic!("expected Header refusal for {text:?}, got {other:?}"),
        }
    }
}

#[test]
fn truncated_file_refuses_typed_with_position() {
    let (_, text) = small();
    let cut = &text[..text.len() * 2 / 3];
    match load(cut) {
        Err(PersistError::Parse {
            line,
            column,
            message,
        }) => {
            // serde_json's 1-based position; column is 0 exactly at a
            // line boundary (where an eps-dependent file length can
            // legitimately land the cut), so only the line is bounded.
            assert!(line >= 1, "position info must be real: {line}:{column}");
            assert!(!message.is_empty());
        }
        other => panic!("expected Parse with position, got {other:?}"),
    }
}

#[test]
fn corrupt_payloads_refuse_typed() {
    let (_, text) = small();
    // Non-hex witness bytes.
    let bad_hex = text.replace("\"abcd\"", "\"abxd\"");
    assert_ne!(bad_hex, text, "fixture must contain the hex bytes");
    assert!(
        matches!(load(&bad_hex), Err(PersistError::Parse { .. })),
        "non-hex byte string must refuse"
    );
    // An unknown field (deny_unknown_fields — no silent tolerance).
    let extra = text.replace("\"snapshot\":", "\"extra\": 1, \"snapshot\":");
    assert!(
        matches!(load(&extra), Err(PersistError::Parse { .. })),
        "unknown field must refuse"
    );
    // An ill-dimensioned expression tree (Length + Angle) — the
    // rebuild-through-constructors door.
    let bad_expr = text.replace(
        "{\"Literal\":{\"value\":1.0,\"dim\":\"Length\"}}",
        "{\"Add\":[{\"Literal\":{\"value\":1.0,\"dim\":\"Length\"}},{\"Literal\":{\"value\":1.0,\"dim\":\"Angle\"}}]}",
    );
    if bad_expr != text {
        assert!(
            matches!(load(&bad_expr), Err(PersistError::Parse { .. })),
            "ill-dimensioned expression must refuse"
        );
    } else {
        // Pretty-printed layout differs: fall back to a direct probe.
        let json = "{\"Add\":[{\"Literal\":{\"value\":1.0,\"dim\":\"Length\"}},{\"Literal\":{\"value\":1.0,\"dim\":\"Angle\"}}]}";
        let refused: Result<editor_core::Expr, _> = serde_json::from_str(json);
        assert!(refused.is_err(), "ill-dimensioned expression must refuse");
    }
}

#[test]
fn snapshot_invariant_violations_refuse_typed() {
    let (_, text) = small();
    // next_id below the live ids (a replay would re-mint id 1).
    let clipped = text.replace("\"next_id\": 2", "\"next_id\": 1");
    assert_ne!(clipped, text);
    match load(&clipped) {
        Err(PersistError::Snapshot(SnapshotError::IdBeyondCounter { id, next_id: 1 })) => {
            assert_eq!(id, RecipeNodeId(1));
        }
        other => panic!("expected IdBeyondCounter, got {other:?}"),
    }
    // order/nodes disagreement.
    let unordered = text.replace("\"order\": [\n      0,\n      1\n    ]", "\"order\": [0]");
    if unordered != text {
        assert!(
            matches!(
                load(&unordered),
                Err(PersistError::Snapshot(SnapshotError::OrderMismatch))
            ),
            "order mismatch must refuse"
        );
    }
}

#[test]
fn non_finite_floats_refuse_at_save_naming_the_site() {
    use editor_core::persist::NonFiniteSite;
    let (doc, _) = small();
    // A NaN smuggled through an UNAPPLIED edit log (a log is data).
    let nan_edit = DocEdit::SetDocParam {
        name: ParamName::new("bad"),
        value: DocParam::Continuous {
            dim: Dimension::Length,
            value: f64::NAN,
        },
    };
    match save(&doc, &[nan_edit]) {
        Err(PersistError::NonFinite {
            site: NonFiniteSite::Edit { index: 0, inner },
        }) => assert!(
            matches!(*inner, NonFiniteSite::DocParam { .. }),
            "site must name the doc param"
        ),
        other => panic!("expected NonFinite Edit site, got {other:?}"),
    }
    // A NaN inside a profile payload (opaque to apply's doors).
    let (bad_doc, p) = insert(
        doc.clone(),
        Node::Profile(desc(
            [f64::INFINITY, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            vec![vec![(0.0, 0.0), (1.0, 0.0), (0.5, 1.0)]],
        )),
    );
    match save(&bad_doc, &[]) {
        Err(PersistError::NonFinite {
            site: NonFiniteSite::Profile { node, .. },
        }) => assert_eq!(node, p),
        other => panic!("expected NonFinite Profile site, got {other:?}"),
    }
    // A NaN inside a metadata tree carried by an unapplied edit.
    let mut m = std::collections::BTreeMap::new();
    m.insert("v".to_owned(), MetaValue::Int(1));
    m.insert("x".to_owned(), MetaValue::Float(f64::NAN));
    let meta_edit = DocEdit::SetAppearanceMeta {
        name: editor_core::StableName {
            kind: editor_core::EntityKind::Body,
            node: RecipeNodeId(1),
            path: vec![editor_core::RoleSeg::OutputBody],
        },
        key: "k".into(),
        value: MetaValue::Map(m),
    };
    match save(&doc, &[meta_edit]) {
        Err(PersistError::NonFinite {
            site: NonFiniteSite::Edit { inner, .. },
        }) => assert!(matches!(*inner, NonFiniteSite::Metadata { .. })),
        other => panic!("expected NonFinite Metadata site, got {other:?}"),
    }
}

#[test]
fn tolerance_conflict_refuses_on_load_and_at_evaluate() {
    let (doc, _) = small();
    let ambient = geom_core::Tolerance::get().eps;
    // A recorded ε that disagrees with the committed process ε: the
    // LOAD door refuses (one process = one ε, D4).
    let other_eps = ambient * 2.0;
    let text = save(&doc, &[DocEdit::SetTolerance { eps: other_eps }]).expect("save");
    match load(&text) {
        Err(PersistError::ToleranceConflict { process, document }) => {
            assert_eq!(process.to_bits(), ambient.to_bits());
            assert_eq!(document.to_bits(), other_eps.to_bits());
        }
        other => panic!("expected ToleranceConflict, got {other:?}"),
    }
    // The EVALUATE door refuses the same conflict per node, typed.
    let retol = apply(&doc, &DocEdit::SetTolerance { eps: other_eps })
        .expect("SetTolerance applies as a pure doc edit")
        .doc;
    let ev = evaluate::<f64>(&retol, None, &CancelToken::new(), &EvalOptions::default());
    assert_eq!(ev.nodes.len(), 2);
    for result in ev.nodes.values() {
        assert!(
            matches!(
                result,
                NodeResult::Failed(e) if matches!(e.kind, NodeErrorKind::ToleranceConflict { .. })
            ),
            "every node must refuse typed on an ε conflict, got {result:?}"
        );
    }
    // And SetTolerance itself validates its value.
    assert!(
        apply(&doc, &DocEdit::SetTolerance { eps: -1.0 }).is_err(),
        "non-positive ε must refuse"
    );
    assert!(
        apply(&doc, &DocEdit::SetTolerance { eps: f64::NAN }).is_err(),
        "NaN ε must refuse"
    );
}

#[test]
fn metadata_convention_doors_refuse_typed() {
    let (doc, _) = small();
    let name = editor_core::StableName {
        kind: editor_core::EntityKind::Body,
        node: RecipeNodeId(1),
        path: vec![editor_core::RoleSeg::OutputBody],
    };
    // No "v" field → refused at the edit door (D7 convention).
    let mut m = std::collections::BTreeMap::new();
    m.insert("x".to_owned(), MetaValue::Int(3));
    let no_v = apply(
        &doc,
        &DocEdit::SetAppearanceMeta {
            name: name.clone(),
            key: "k".into(),
            value: MetaValue::Map(m),
        },
    );
    assert!(
        matches!(no_v, Err(editor_core::EditError::MetaUnversioned { .. })),
        "missing v field must refuse, got {no_v:?}"
    );
    // Non-map top level → refused.
    let scalar = apply(
        &doc,
        &DocEdit::SetAppearanceMeta {
            name,
            key: "k".into(),
            value: MetaValue::Int(1),
        },
    );
    assert!(
        matches!(scalar, Err(editor_core::EditError::MetaUnversioned { .. })),
        "non-map value must refuse, got {scalar:?}"
    );
}

#[test]
fn tangent_joint_doors_refuse_typed_at_load() {
    // A profile whose second loop-joint is declared tangent (#101):
    // craft the file, then mangle the declaration each illegal way.
    let mut lp = profile::ProfileLoop::new(
        [(0.0, 0.0), (1.0, 0.0), (2.0, 0.0), (2.0, 2.0), (0.0, 2.0)]
            .into_iter()
            .map(|(x, y)| profile::ProfileVertex {
                pos: geom_core::Point2::new(x, y),
                bulge: 0.0,
            })
            .collect(),
    );
    lp.tangent_joints = vec![1];
    let (doc, _) = insert(
        ProfileDoc::empty(),
        Node::Profile(editor_core::ProfileDesc(profile::Profile::new(
            profile::SketchPlane::xy(),
            vec![lp],
        ))),
    );
    let text = save(&doc, &[]).expect("save");
    let needle = "\"tangent_joints\": [\n                1\n              ]";
    assert!(text.contains(needle), "fixture must contain the joint");
    for (bad, expect) in [
        ("[\n                9\n              ]", "out of range"),
        (
            "[\n                1, 1\n              ]",
            "strictly increasing",
        ),
        (
            "[\n                3, 1\n              ]",
            "strictly increasing",
        ),
    ] {
        let crafted = text.replace(needle, &format!("\"tangent_joints\": {bad}"));
        assert_ne!(crafted, text);
        match load(&crafted) {
            Err(PersistError::Parse { message, .. }) => assert!(
                message.contains(expect),
                "expected {expect:?} in refusal: {message}"
            ),
            other => panic!("mangled joints ({bad}) must refuse typed, got {other:?}"),
        }
    }
    // The canonical declaration itself loads and survives bit-exactly.
    let loaded = load(&text).expect("canonical joints load");
    let Some(Node::Profile(desc)) = loaded.doc.node(RecipeNodeId(0)) else {
        panic!("profile lost");
    };
    assert_eq!(desc.0.loops[0].tangent_joints, vec![1]);
}

#[test]
fn stale_tangent_joint_refuses_at_save_before_any_byte() {
    use editor_core::persist::JointSite;
    // The payload is pub: a stale joint (as after deleting a vertex
    // in the payload) reaches the document without passing any edit
    // door. Save must refuse — the load side would (MAJOR-DELTA-1's
    // inversion: a save that succeeds but cannot load).
    let mut lp = profile::ProfileLoop::new(
        [(0.0, 0.0), (2.0, 0.0), (1.0, 1.0)]
            .into_iter()
            .map(|(x, y)| profile::ProfileVertex {
                pos: geom_core::Point2::new(x, y),
                bulge: 0.0,
            })
            .collect(),
    );
    lp.tangent_joints = vec![7]; // 3 vertices: out of range
    let bad = editor_core::ProfileDesc(profile::Profile::new(profile::SketchPlane::xy(), vec![lp]));
    // Snapshot site.
    let (doc, node) = insert(ProfileDoc::empty(), Node::Profile(bad.clone()));
    match save(&doc, &[]) {
        Err(PersistError::TangentJointOutOfRange {
            site: JointSite::Profile { node: at },
            loop_index: 0,
            joint: 7,
            vertex_count: 3,
        }) => assert_eq!(at, node),
        other => panic!("stale joint in snapshot must refuse at save, got {other:?}"),
    }
    // Edit-log site (an unapplied log is data; the walk runs before
    // the replay-verification pass so the site names the edit).
    match save(
        &ProfileDoc::empty(),
        &[DocEdit::InsertNode {
            node: Node::Profile(bad),
        }],
    ) {
        Err(PersistError::TangentJointOutOfRange {
            site: JointSite::InsertedProfile { index: 0 },
            loop_index: 0,
            joint: 7,
            vertex_count: 3,
        }) => {}
        other => panic!("stale joint in edit log must refuse at save, got {other:?}"),
    }
}

#[test]
fn unreplayable_edit_log_refuses_at_save() {
    // Save/load symmetry for the LOG: load replays through apply's
    // doors, so a log that refuses there refuses at save too — e.g. a
    // D7 metadata value without its "v" field.
    let (doc, _) = small();
    let mut m = std::collections::BTreeMap::new();
    m.insert("x".to_owned(), MetaValue::Int(3));
    let bad = DocEdit::SetAppearanceMeta {
        name: editor_core::StableName {
            kind: editor_core::EntityKind::Body,
            node: RecipeNodeId(1),
            path: vec![editor_core::RoleSeg::OutputBody],
        },
        key: "k".into(),
        value: MetaValue::Map(m),
    };
    match save(&doc, &[bad]) {
        Err(PersistError::EditReplay { index: 0, error }) => assert!(
            matches!(error, editor_core::EditError::MetaUnversioned { .. }),
            "expected the apply door's refusal, got {error:?}"
        ),
        other => panic!("unreplayable log must refuse at save, got {other:?}"),
    }
    // And a log referencing a node the snapshot lacks.
    let orphan = DocEdit::SetParam {
        node: RecipeNodeId(77),
        slot: editor_core::SlotId::Distance,
        expr: len(1.0),
    };
    assert!(matches!(
        save(&doc, &[orphan]),
        Err(PersistError::EditReplay { index: 0, .. })
    ));
}
