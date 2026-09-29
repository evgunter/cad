//! M4 PR 6 — the refusal CI row: garbled headers, corrupt payloads and
//! truncated files refuse TYPED (position info included); non-finite
//! floats refuse typed at SAVE naming the site; the D4
//! one-process-one-ε doors refuse loudly. No silent best-effort
//! loads, ever.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;

use editor_core::expr::DimensionError;
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
        &editor_core::RefusingReach,
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
/// unknown field — is UNREADABLE by this build (the deserializer's own
/// rejection, recourse attached); only bytes that are not JSON at all
/// report as `Parse`. An ill-dimensioned expression is the one `Data`
/// failure that is neither: it has a typed refusal to carry and
/// carries it ([`dimension_refusals_cross_the_load_door_whole`]).
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
}

/// The save's text back out with the extrude's distance expression
/// replaced by `wire`.
///
/// The tamper is STRUCTURAL rather than a string replacement: an
/// expression's spelling carries whatever fields `WireExpr` has today
/// (a `unit` arrived after this file was written) and the layout is
/// whatever `to_string_pretty` produces, so a needle written out in
/// full stops matching without stopping the test, and an assertion
/// nothing reaches is documentation. This one fails if the slot it
/// aims at is not there.
fn with_distance(text: &str, wire: serde_json::Value) -> String {
    let (header, body_text) = text.split_once('\n').expect("a header line");
    let mut body: serde_json::Value = serde_json::from_str(body_text).expect("a JSON body");
    let nodes = body["snapshot"]["nodes"]
        .as_object_mut()
        .expect("a node map");
    let mut swapped = 0;
    for node in nodes.values_mut() {
        if let Some(extrude) = node.get_mut("Extrude") {
            extrude["distance"] = wire.clone();
            swapped += 1;
        }
    }
    assert_eq!(swapped, 1, "the fixture has exactly one extrude to tamper");
    format!(
        "{header}\n{}",
        serde_json::to_string(&body).expect("re-emit")
    )
}

/// A literal wire expression of `dim` written in `unit`.
fn wire_literal(dim: &str, unit: &str) -> serde_json::Value {
    serde_json::json!({ "Literal": { "value": 1.0, "dim": dim, "unit": unit } })
}

/// An expression the document layer's dimension checker refuses crosses
/// the load door WHOLE: `PersistError::Dimension` carries the
/// `DimensionError` itself, not a sentence about it.
///
/// The refusal is the one the AUTHORING constructors raise for the same
/// tree, because the rebuild runs those constructors.
#[test]
fn dimension_refusals_cross_the_load_door_whole() {
    let (_, text) = small();
    let length = wire_literal("Length", "m");
    let angle = wire_literal("Angle", "rad");
    let cases: [(serde_json::Value, DimensionError); 4] = [
        (
            serde_json::json!({ "Add": [length.clone(), angle.clone()] }),
            DimensionError::Mismatch {
                op: "add",
                left: Dimension::Length,
                right: Dimension::Angle,
            },
        ),
        (
            serde_json::json!({ "Mul": [length.clone(), length.clone()] }),
            DimensionError::MulNeedsScalar {
                left: Dimension::Length,
                right: Dimension::Length,
            },
        ),
        (
            serde_json::json!({ "Sin": length.clone() }),
            DimensionError::TrigNeedsAngle {
                op: "sin",
                found: Dimension::Length,
            },
        ),
        (
            wire_literal("Length", "furlong"),
            DimensionError::UnknownDisplayUnit {
                symbol: "furlong".to_owned(),
            },
        ),
    ];
    for (wire, expected) in cases {
        let tampered = with_distance(&text, wire.clone());
        match load(&tampered, Tol::witness()) {
            Err(PersistError::Dimension {
                line,
                column,
                error,
            }) => {
                assert_eq!(error, expected, "the refusal crosses whole: {wire}");
                assert!(
                    line >= 1,
                    "serde_json's position rides along: {line}:{column}"
                );
            }
            other => panic!("expected a Dimension refusal for {wire}, got {other:?}"),
        }
    }
}

/// The recourse belongs to a file this build cannot READ, and an
/// ill-dimensioned expression is not one: regenerating it produces the
/// same refusal, because what is wrong is the expression.
#[test]
fn a_dimension_refusal_does_not_advise_regenerating_the_file() {
    let (_, text) = small();
    let tampered = with_distance(
        &text,
        serde_json::json!({ "Add": [wire_literal("Length", "m"), wire_literal("Angle", "rad")] }),
    );
    let refusal = load(&tampered, Tol::witness()).expect_err("must refuse");
    let message = refusal.to_string();
    assert!(
        !message.contains("regenerate"),
        "a dimension refusal carries no regenerate recourse: {message}"
    );
    assert!(
        message.contains("cannot apply `add` to length and angle"),
        "the checker's own prose is the human half: {message}"
    );
}

/// **One fault, one arm, whichever route it takes.** An off-table
/// display-unit symbol reaches the load door two ways — on an
/// expression literal, through `WireExpr::rebuild`'s closed-table
/// lookup, and on a document PARAMETER, through `UnitSym`'s own
/// `Deserialize` — and it is the same fault both times.
///
/// This is the row that would red if the two ever diverged again. They
/// did: the parameter route used to answer `Unreadable` with the
/// regenerate recourse, which is advice that reproduces the refusal,
/// while the expression route answered typed with none.
#[test]
fn an_off_table_display_unit_refuses_the_same_way_on_either_route() {
    let (doc, _) = small();
    // A document parameter carries a display unit of its own, so its
    // symbol is on the wire beside the expression literals'.
    let doc = apply(
        &doc,
        &DocEdit::SetDocParam {
            name: ParamName::new("bore"),
            value: DocParam::continuous(Dimension::Length, 0.01),
        },
        Tol::witness(),
        &editor_core::RefusingReach,
    )
    .expect("the parameter applies")
    .doc;
    let text = save(&doc, &[], Tol::witness()).expect("save");
    let unknown = DimensionError::UnknownDisplayUnit {
        symbol: "furlong".to_owned(),
    };

    // Route 1: the parameter's own display unit, refused at the token.
    let on_param = text.replace(r#""display_unit": "m""#, r#""display_unit": "furlong""#);
    assert_ne!(on_param, text, "the parameter's unit must be on the wire");
    match load(&on_param, Tol::witness()) {
        Err(PersistError::Dimension { error, .. }) => assert_eq!(error, unknown),
        other => panic!("a parameter's off-table unit must refuse typed, got {other:?}"),
    }

    // Route 2: an expression literal's, refused at the rebuild.
    let on_literal = with_distance(&text, wire_literal("Length", "furlong"));
    match load(&on_literal, Tol::witness()) {
        Err(PersistError::Dimension { error, .. }) => assert_eq!(error, unknown),
        other => panic!("a literal's off-table unit must refuse typed, got {other:?}"),
    }

    // And neither sends the caller to regenerate a file whose symbol
    // would be just as absent from the table the second time.
    for bad in [&on_param, &on_literal] {
        let message = load(bad, Tol::witness())
            .expect_err("must refuse")
            .to_string();
        assert!(!message.contains("regenerate"), "{message}");
    }
}

/// **The sibling route, and what it costs one rung up.** A save file's
/// `edits` list is DATA, so a hand-edited one can carry an edit whose
/// replay the document layer's dimension checker refuses. That reaches
/// the load door too — as `EditReplay` wrapping the `EditError` the
/// replay raised, which for `SetExpression` is `EditError::Dimension`
/// carrying the same `DimensionError` the `Dimension` arm above does.
///
/// It has to be TAMPERED rather than handed to `save`: the save door
/// replays the log through the same apply doors
/// ([`unreplayable_edit_log_refuses_at_save`]), so a log this bad never
/// gets written. The hand-edited file is the only way in, which is
/// exactly what the load-door re-check is for.
///
/// The kernel loses nothing — both levels are on the value, matchable.
/// What is one rung short is the PYTHON projection: `pncad-py`'s
/// `persist_err` gives `PersistError.inner_variant` the `EditError`'s
/// own word (`dimension`) and drops the check's (`mismatch`), which
/// `crate::tags::edit_inner_variant_tag` would supply and which it
/// calls at its other two sites. Filed as
/// `work/lib/persist-inner-variant-stops-one-rung-above-the-check.md`;
/// this row is the kernel-side fact that one rests on.
#[test]
fn a_replayed_edits_dimension_refusal_reaches_the_load_door() {
    use editor_core::{EditError, ExprPath, SlotId};
    let tol = Tol::witness();
    // A distance that is an OPERATOR node, so replacing one leaf can
    // make the rebuild ill-dimensioned. A bare literal could not.
    let doc = ProfileDoc::empty_derived("m4_pr6_replay_dimension", tol);
    let (doc, p) = on_frame(
        doc,
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)]],
    );
    let (doc, extrude) = insert(
        doc,
        Node::Extrude {
            profile: p,
            distance: Expr::add(len(1.0), len(1.0)).expect("Length + Length"),
        },
    );
    // A log the save door accepts: same dimension, so the replay is
    // clean and the file is written.
    let legal = DocEdit::SetExpression {
        path: ExprPath {
            node: extrude,
            slot: SlotId::Distance,
            path: vec![1],
        },
        expr: len(2.0),
    };
    let text = save(&doc, &[legal.into()], tol).expect("a replayable log is written");

    // Now the hand edit: the logged replacement becomes an Angle.
    let (header, body_text) = text.split_once('\n').expect("a header line");
    let mut body: serde_json::Value = serde_json::from_str(body_text).expect("a JSON body");
    let edits = body["edits"].as_array_mut().expect("an edit list");
    assert_eq!(edits.len(), 1, "the fixture logs exactly one edit");
    // `pointer_mut`, not `[..]` indexing: indexing a missing key would
    // INSERT it, and the tamper would then refuse for the stray key
    // rather than reach the dimension checker.
    *edits[0]
        .pointer_mut("/edit/SetExpression/expr")
        .expect("the logged edit's expression slot") =
        serde_json::json!({ "Literal": { "value": 1.0, "dim": "Angle", "unit": "rad" } });
    let tampered = format!(
        "{header}\n{}",
        serde_json::to_string(&body).expect("re-emit")
    );

    match load(&tampered, tol) {
        Err(PersistError::EditReplay {
            index: 0,
            error: EditError::Dimension(inner),
        }) => assert_eq!(
            inner,
            DimensionError::Mismatch {
                op: "add",
                left: Dimension::Length,
                right: Dimension::Angle,
            },
            "the replay refuses with the checker's own value, two levels deep"
        ),
        other => panic!("expected an EditReplay dimension refusal, got {other:?}"),
    }
}

/// A `Deserialize` impl driven from somewhere other than the load door
/// records NOTHING, because the refusal frame belongs to a parse and
/// there is no parse open.
///
/// Without that, a bare `from_str` of one wire type would arm the
/// channel for whatever this thread loaded next — the hazard the guard
/// exists to make unreachable rather than to document.
#[test]
fn a_refusal_outside_a_parse_arms_nothing() {
    let bad = r#"{"Add":[{"Literal":{"value":1.0,"dim":"Length","unit":"m"}},"#.to_owned()
        + r#"{"Literal":{"value":1.0,"dim":"Angle","unit":"rad"}}]}"#;
    let refused: Result<Expr, _> = serde_json::from_str(&bad);
    assert!(refused.is_err(), "the tree is ill-dimensioned");
    // Now a clean load on the same thread. If that refusal had been
    // recorded anywhere a later parse could read, this would answer a
    // dimension refusal for a document that has nothing wrong with it.
    let (_, text) = small();
    load(&text, Tol::witness()).expect("the clean document still loads");
}

/// A load after one that refused does not inherit the refusal: the
/// channel the structure crosses on belongs to ONE parse
/// (`persist::refusal`), and a slot left dirty would make the next load
/// answer with the last one's fault.
#[test]
fn a_recorded_refusal_does_not_reach_the_next_load() {
    let (_, text) = small();
    let tampered = with_distance(
        &text,
        serde_json::json!({ "Add": [wire_literal("Length", "m"), wire_literal("Angle", "rad")] }),
    );
    assert!(matches!(
        load(&tampered, Tol::witness()),
        Err(PersistError::Dimension { .. })
    ));
    load(&text, Tol::witness()).expect("the clean document still loads");
    // And an unrelated `Data` failure after one is still `Unreadable`,
    // rather than the earlier refusal wearing a new position.
    let extra = text.replace("\"snapshot\":", "\"extra\": 1, \"snapshot\":");
    assert_ne!(extra, text, "the fixture must carry the snapshot key");
    assert!(matches!(
        load(&extra, Tol::witness()),
        Err(PersistError::Unreadable { .. })
    ));
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
    match save(
        &doc,
        &[editor_core::LoggedEdit::bare(nan_edit)],
        Tol::witness(),
    ) {
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
    match save(
        &doc,
        &[editor_core::LoggedEdit::bare(meta_edit)],
        Tol::witness(),
    ) {
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
        &[editor_core::LoggedEdit::bare(DocEdit::SetTolerance {
            eps: other_eps,
        })],
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
        &editor_core::RefusingReach,
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
        apply(
            &doc,
            &DocEdit::SetTolerance { eps: -1.0 },
            Tol::witness(),
            &editor_core::RefusingReach
        )
        .is_err(),
        "non-positive ε must refuse"
    );
    assert!(
        apply(
            &doc,
            &DocEdit::SetTolerance { eps: f64::NAN },
            Tol::witness(),
            &editor_core::RefusingReach
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
        &editor_core::RefusingReach,
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
        &editor_core::RefusingReach,
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
    // wrong-dimension argument ROLE — decided for every node kind by
    // the shared slot walk — and a lattice-violating step order, both
    // refused by the shared validator on the parsed document. Craft a
    // valid file, then mutate the JSON body.
    let (doc, plane) = insert(
        ProfileDoc::empty_derived("m4_pr6_refusal", Tol::witness()),
        xy_frame(),
    );
    let (doc, circle) = insert(
        doc,
        Node::Profile(editor_core::ProfileProgram {
            plane,
            loops: vec![editor_core::LoopProgram::circle(0.0, 0.0, 0.5).expect("finite")],
            ids: Vec::new(),
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
    // A program slot is a slot like any other, so the document-wide
    // slot walk decides it — the same `Node::slot_dimension_fault` the
    // edit doors ask, in the load door's vocabulary.
    match load(&mangled, Tol::witness()) {
        Err(PersistError::Snapshot(editor_core::SnapshotError::SlotDimension {
            node,
            expected: editor_core::Dimension::Length,
            found: editor_core::Dimension::Angle,
            ..
        })) => assert_eq!(node, circle),
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
        ids: Vec::new(),
    };
    match editor_core::apply(
        &doc,
        &DocEdit::InsertNode {
            node: Node::Profile(unclosed),
        },
        Tol::witness(),
        &editor_core::RefusingReach,
    ) {
        Err(EditError::ProfileProgramRefused { refusal, .. })
            if matches!(
                *refusal,
                ProgramRefusal::Transition {
                    loop_: 0,
                    step: 0,
                    ..
                }
            ) => {}
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
    match save(&doc, &[editor_core::LoggedEdit::bare(bad)], Tol::witness()) {
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
        save(
            &doc,
            &[editor_core::LoggedEdit::bare(orphan)],
            Tol::witness()
        ),
        Err(PersistError::EditReplay { index: 0, .. })
    ));
}
