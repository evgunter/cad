//! M4 PR 6 — the refusal CI row: garbled headers, corrupt payloads and
//! truncated files refuse TYPED (position info included); non-finite
//! floats refuse typed at SAVE naming the site; the D4
//! one-process-one-ε doors refuse loudly. No silent best-effort
//! loads, ever.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod fixture;

use editor_core::persist::SnapshotError;
use editor_core::{
    CancelToken, Dimension, DocEdit, DocParam, EvalOptions, Expr, MetaValue, Node, NodeErrorKind,
    NodeResult, ParamName, PersistError, ProfileDoc, RecipeNodeId, WitnessDatum, apply, evaluate,
    load, save,
};
use fixture::{insert, len, on_frame, xy_frame};
use geom_core::Tol;

/// A small valid document (profile + extrude + witness) and its save.
fn small() -> (ProfileDoc, String) {
    let doc = ProfileDoc::empty_derived("m4_pr6_refusal", Tol::witness());
    let (doc, p) = on_frame(
        doc,
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)]],
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
        Tol::witness(),
    )
    .expect("witness")
    .doc;
    let text = save(&doc, &[], Tol::witness()).expect("save");
    (doc, text)
}

#[test]
fn missing_or_garbled_header_refuses_typed() {
    for text in ["", "not a save file", "id: banana\n{}"] {
        match load(text, Tol::witness()) {
            Err(PersistError::HeaderId { .. }) => {}
            other => panic!("expected HeaderId refusal for {text:?}, got {other:?}"),
        }
    }
}

#[test]
fn truncated_file_refuses_typed_with_position() {
    let (_, text) = small();
    let cut = &text[..text.len() * 2 / 3];
    match load(cut, Tol::witness()) {
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

/// Valid JSON the typed shape rejects — a non-hex byte string, an
/// unknown field, an ill-dimensioned expression — is UNREADABLE by
/// this build (the deserializer's own rejection, recourse attached);
/// only bytes that are not JSON at all report as `Parse`.
#[test]
fn corrupt_payloads_refuse_typed() {
    let (_, text) = small();
    // Non-hex witness bytes.
    let bad_hex = text.replace("\"abcd\"", "\"abxd\"");
    assert_ne!(bad_hex, text, "fixture must contain the hex bytes");
    assert!(
        matches!(
            load(&bad_hex, Tol::witness()),
            Err(PersistError::Unreadable { .. })
        ),
        "non-hex byte string must refuse"
    );
    // An unknown field (deny_unknown_fields — no silent tolerance).
    let extra = text.replace("\"snapshot\":", "\"extra\": 1, \"snapshot\":");
    assert!(
        matches!(
            load(&extra, Tol::witness()),
            Err(PersistError::Unreadable { .. })
        ),
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
            matches!(
                load(&bad_expr, Tol::witness()),
                Err(PersistError::Unreadable { .. })
            ),
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
    // next_id below the live ids (a replay would re-mint id 2).
    let clipped = text.replace("\"next_id\": 3", "\"next_id\": 2");
    assert_ne!(clipped, text);
    match load(&clipped, Tol::witness()) {
        Err(PersistError::Snapshot(SnapshotError::IdBeyondCounter { id, next_id: 2 })) => {
            assert_eq!(id, RecipeNodeId(2));
        }
        other => panic!("expected IdBeyondCounter, got {other:?}"),
    }
    // order/nodes disagreement.
    let unordered = text.replace(
        "\"order\": [\n      0,\n      1,\n      2\n    ]",
        "\"order\": [0]",
    );
    if unordered != text {
        assert!(
            matches!(
                load(&unordered, Tol::witness()),
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
        value: DocParam::continuous(Dimension::Length, f64::NAN),
    };
    match save(&doc, &[nan_edit], Tol::witness()) {
        Err(PersistError::NonFinite {
            site: NonFiniteSite::Edit { index: 0, inner },
        }) => assert!(
            matches!(*inner, NonFiniteSite::DocParam { .. }),
            "site must name the doc param"
        ),
        other => panic!("expected NonFinite Edit site, got {other:?}"),
    }
    // A profile has no raw float left to smuggle one through: its
    // plane is a NODE and its programs are `Expr`s, so the non-finite
    // that used to reach the save door as a placement float is refused
    // a layer earlier, when the frame's slot is authored.
    assert!(
        matches!(
            Expr::literal(f64::INFINITY, Dimension::Length),
            Err(editor_core::DimensionError::NonFiniteLiteral)
        ),
        "the frame's origin slot refuses a non-finite at its literal door"
    );
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
    match save(&doc, &[meta_edit], Tol::witness()) {
        Err(PersistError::NonFinite {
            site: NonFiniteSite::Edit { inner, .. },
        }) => assert!(matches!(*inner, NonFiniteSite::Metadata { .. })),
        other => panic!("expected NonFinite Metadata site, got {other:?}"),
    }
}

#[test]
fn tolerance_conflict_refuses_on_load_and_at_evaluate() {
    let (doc, _) = small();
    let ambient = geom_core::Tol::witness().get().eps;
    // A recorded ε that disagrees with the committed process ε: the
    // LOAD door refuses (one process = one ε, D4).
    let other_eps = ambient * 2.0;
    let text = save(
        &doc,
        &[DocEdit::SetTolerance { eps: other_eps }],
        Tol::witness(),
    )
    .expect("save");
    match load(&text, Tol::witness()) {
        Err(PersistError::ToleranceConflict { process, document }) => {
            assert_eq!(process.to_bits(), ambient.to_bits());
            assert_eq!(document.to_bits(), other_eps.to_bits());
        }
        other => panic!("expected ToleranceConflict, got {other:?}"),
    }
    // The EVALUATE door refuses the same conflict per node, typed.
    let retol = apply(
        &doc,
        &DocEdit::SetTolerance { eps: other_eps },
        Tol::witness(),
    )
    .expect("SetTolerance applies as a pure doc edit")
    .doc;
    let ev = evaluate::<f64>(
        &retol,
        None,
        &CancelToken::new(),
        &EvalOptions::default(),
        Tol::witness(),
    );
    assert_eq!(ev.nodes.len(), 3);
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
        apply(&doc, &DocEdit::SetTolerance { eps: -1.0 }, Tol::witness()).is_err(),
        "non-positive ε must refuse"
    );
    assert!(
        apply(
            &doc,
            &DocEdit::SetTolerance { eps: f64::NAN },
            Tol::witness()
        )
        .is_err(),
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
        Tol::witness(),
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
        Tol::witness(),
    );
    assert!(
        matches!(scalar, Err(editor_core::EditError::MetaUnversioned { .. })),
        "non-map value must refuse, got {scalar:?}"
    );
}

#[test]
fn program_structure_doors_refuse_typed_at_load() {
    // v4 (LIB-SWITCH §4h): the stored-joint corruption class died with
    // stored joints; the program layer's corrupt-file classes are a
    // wrong-dimension argument ROLE and a lattice-violating step
    // order, both refused by the shared validator on the parsed
    // document. Craft a valid file, then mutate the JSON body.
    let (doc, plane) = insert(
        ProfileDoc::empty_derived("m4_pr6_refusal", Tol::witness()),
        xy_frame(),
    );
    let (doc, circle) = insert(
        doc,
        Node::Profile(editor_core::ProfileProgram {
            plane,
            loops: vec![editor_core::LoopProgram::circle(0.0, 0.0, 0.5).expect("finite")],
        }),
    );
    let text = save(&doc, &[], Tol::witness()).expect("save");
    // The header is the id line; split it off.
    let (header, body) = text.split_once('\n').expect("id line");
    let mut v: serde_json::Value = serde_json::from_str(body).expect("body parses");
    // (a) Wrong-dimension role: retype the circle's centre-x literal
    // as an Angle. The Expr door accepts an Angle literal per se; the
    // shared validator's DIMENSION WALK refuses it in the CenterX role.
    // The UNIT moves with the dim: since v20 a literal names its
    // notation, so leaving `"m"` beside an `Angle` dim would be caught
    // one door earlier as a display-unit mismatch, and this row is
    // about the SLOT's role dimension, not the literal's own coherence.
    v["snapshot"]["nodes"]["1"]["Profile"]["loops"][0]["Circle"]["centre"][0]["Literal"]["dim"] =
        serde_json::Value::String("Angle".into());
    v["snapshot"]["nodes"]["1"]["Profile"]["loops"][0]["Circle"]["centre"][0]["Literal"]["unit"] =
        serde_json::Value::String("rad".into());
    let mangled = format!("{header}\n{}\n", serde_json::to_string_pretty(&v).unwrap());
    match load(&mangled, Tol::witness()) {
        Err(PersistError::ProfileProgram {
            node,
            fault:
                editor_core::ProgramFault::SlotDimension {
                    expected: editor_core::Dimension::Length,
                    found: editor_core::Dimension::Angle,
                    ..
                },
        }) => assert_eq!(node, circle),
        other => panic!("wrong-dimension role must refuse typed at load, got {other:?}"),
    }
    // (b) Lattice violation: an unclosed chain (a step list that stops
    // mid-air) — reachable only from a hand-edited file, refused by
    // the replay PROBE with the Transition class.
    let (doc2, chain) = on_frame(
        ProfileDoc::empty_derived("m4_pr6_refusal", Tol::witness()),
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![(0.0, 0.0), (1.0, 0.0), (0.5, 1.0)]],
    );
    let text2 = save(&doc2, &[], Tol::witness()).expect("save");
    let (header2, body2) = text2.split_once('\n').expect("id line");
    let mut v2: serde_json::Value = serde_json::from_str(body2).expect("body parses");
    let steps = v2["snapshot"]["nodes"]["1"]["Profile"]["loops"][0]["Chain"]
        .as_array_mut()
        .expect("chain steps");
    steps.pop(); // drop the closing LineTo(Start)
    let n_left = steps.len() as u32;
    let mangled2 = format!(
        "{header2}\n{}\n",
        serde_json::to_string_pretty(&v2).unwrap()
    );
    match load(&mangled2, Tol::witness()) {
        Err(PersistError::ProfileProgram {
            node,
            fault:
                editor_core::ProgramFault::Lattice {
                    loop_: 0,
                    step,
                    verb: None,
                    ..
                },
        }) => {
            assert_eq!(node, chain);
            assert_eq!(step, n_left, "one past the end: the chain never closed");
        }
        other => panic!("an unclosed chain must refuse typed at load, got {other:?}"),
    }
    // The unmangled file loads and the program survives bit-exactly.
    let loaded = load(&text, Tol::witness()).expect("canonical program loads");
    let Some(Node::Profile(prog)) = loaded.doc.node(circle) else {
        panic!("profile lost");
    };
    assert_eq!(prog.loops.len(), 1);
}

#[test]
fn corrupt_program_refuses_at_the_edit_door_before_any_save() {
    // v4's stronger posture (MAJOR-DELTA-1's inversion, moved a layer
    // UP): the retired stale-joint attack corrupted the pub payload
    // and needed the SAVE door to catch it. A lattice-violating
    // program is refused already at `apply` (the VQ9 authoring-time
    // check runs resolve + replay), so the corrupt state never enters
    // a document at all — and the save-door twin (the shared
    // validator's replay probe) stays for parsed files.
    use editor_core::{EditError, LoopProgram, ProgramRefusal, ProgramStep};
    // The frame goes in first: the row is about the PROGRAM's refusal,
    // and a profile naming a plane the document does not have would be
    // turned away for that instead.
    let (doc, plane) = insert(
        ProfileDoc::empty_derived("m4_pr6_refusal", Tol::witness()),
        xy_frame(),
    );
    let unclosed = editor_core::ProfileProgram {
        plane,
        loops: vec![LoopProgram::Chain(vec![ProgramStep::Tangent])],
    };
    match editor_core::apply(
        &doc,
        &DocEdit::InsertNode {
            node: Node::Profile(unclosed),
        },
        Tol::witness(),
    ) {
        Err(EditError::ProfileProgramRefused {
            refusal:
                ProgramRefusal::Transition {
                    loop_: 0, step: 0, ..
                },
            ..
        }) => {}
        other => panic!("a lattice-violating program must refuse at apply, got {other:?}"),
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
    match save(&doc, &[bad], Tol::witness()) {
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
        save(&doc, &[orphan], Tol::witness()),
        Err(PersistError::EditReplay { index: 0, .. })
    ));
}
