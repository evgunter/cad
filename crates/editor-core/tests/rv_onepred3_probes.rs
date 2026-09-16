//! **Review probes for `edit/one-predicate-round-three`** (lane
//! `onepred3-rv`). Not part of the unit's acceptance — these exist to
//! falsify the PR's claims, and are kept so a fix pass can adopt them.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;

use editor_core::{
    Dimension, DocEdit, EditError, Expr, MeasureExpr, Node, PatternKind, PersistError, ProfileDoc,
    RecipeNodeId, SlotId, SnapshotError, apply, load, save,
};
use fixture::{insert, len, on_frame_keeping, scl, square};
use geom_core::Tol;

/// Wire surgery by path, as `load_door_slot_dimension::doctored`.
fn doctored(text: &str, edit: impl FnOnce(&mut serde_json::Value)) -> String {
    let split = text.find('{').expect("the JSON body follows the id header");
    let (header, body) = text.split_at(split);
    let mut wire: serde_json::Value = serde_json::from_str(body).expect("the body parses");
    edit(&mut wire);
    let out = format!("{header}{wire}");
    assert_ne!(out, text, "the corruption really landed");
    out
}

/// An extrude, patterned linearly — the pattern's `count` is a
/// Count-typed STRUCTURAL slot, the third node kind (after the extrude
/// and the frame datum the unit's own suite covers).
fn patterned() -> (ProfileDoc, RecipeNodeId) {
    let (doc, _, profile) = on_frame_keeping(
        ProfileDoc::empty(
            editor_core::DocumentId::derive("rv-onepred3"),
            Tol::witness(),
        ),
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![square(0.0, 0.0, 0.5)],
    );
    let (doc, extrude) = insert(
        doc,
        Node::Extrude {
            profile,
            distance: len(1.0),
        },
    );
    let (doc, pattern) = insert(
        doc,
        Node::Pattern {
            input: extrude,
            count: Expr::count(3),
            kind: PatternKind::Linear {
                direction: [scl(1.0), scl(0.0), scl(0.0)],
                spacing: len(3.0),
            },
        },
    );
    (doc, pattern)
}

/// PROBE 1 — a THIRD node kind with a typed slot, and a Count one:
/// the pattern's instance count. Claim 1 of the review brief.
#[test]
fn rv_a_retyped_pattern_count_is_refused_at_both_doors() {
    let (doc, pattern) = patterned();
    match apply(
        &doc,
        &DocEdit::SetStructuralParam {
            node: pattern,
            slot: SlotId::Count,
            expr: len(3.0),
        },
        Tol::witness(),
    ) {
        Err(EditError::SlotDimensionMismatch {
            slot,
            expected,
            found,
        }) => assert_eq!(
            (slot, expected, found),
            (SlotId::Count, Dimension::Count, Dimension::Length)
        ),
        other => panic!("the edit door must refuse a length count, got {other:?}"),
    }

    let text = save(&doc, &[], Tol::witness()).expect("the fixture saves");
    load(&text, Tol::witness()).expect("the fixture loads");
    eprintln!(
        "RV-PROBE1 count literal on the wire: {}",
        serde_json::to_string(
            &serde_json::from_str::<serde_json::Value>(&text[text.find('{').unwrap()..]).unwrap()
                ["snapshot"]["nodes"][pattern.0.to_string()]["Pattern"]["count"]
        )
        .unwrap()
    );
    // A Count expression is `{"Count": n}` on the wire, not a
    // `Literal`; the surgery swaps in a well-formed LENGTH literal, so
    // the only rule left to refuse it is the slot's own.
    let corrupt = doctored(&text, |wire| {
        let count = &mut wire["snapshot"]["nodes"][pattern.0.to_string()]["Pattern"]["count"];
        assert_eq!(*count, serde_json::json!({ "Count": 3 }));
        *count = serde_json::json!({
            "Literal": { "value": 3.0, "dim": "Length", "unit": "m" }
        });
    });
    match load(&corrupt, Tol::witness()) {
        Err(PersistError::Snapshot(SnapshotError::SlotDimension {
            node,
            slot,
            expected,
            found,
        })) => assert_eq!(
            (node, slot, expected, found),
            (pattern, SlotId::Count, Dimension::Count, Dimension::Length)
        ),
        other => panic!("the load door must refuse a length count, got {other:?}"),
    }
}

/// PROBE 2 — the residue the unit filed
/// (`load-door-does-not-check-payload-expression-param-refs`),
/// MEASURED: a measure's expression reading an undeclared parameter
/// loads clean while the edit door refuses the same node.
#[test]
fn rv_a_measure_expression_reading_an_undeclared_parameter_still_loads() {
    let name = editor_core::ParamName::new("depth");
    let (doc, _, profile) = on_frame_keeping(
        ProfileDoc::empty(
            editor_core::DocumentId::derive("rv-onepred3-payload"),
            Tol::witness(),
        ),
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![square(0.0, 0.0, 0.5)],
    );
    let (doc, _) = insert(
        doc,
        Node::Extrude {
            profile,
            distance: len(1.0),
        },
    );
    let doc = apply(
        &doc,
        &DocEdit::SetDocParam {
            name: name.clone(),
            value: editor_core::DocParam::continuous(Dimension::Length, 1.0),
        },
        Tol::witness(),
    )
    .expect("a well-formed length parameter declares")
    .doc;
    let (doc, measure) = insert(
        doc,
        Node::Measure {
            expr: MeasureExpr::value(Expr::param(name.clone(), Dimension::Length)),
            refs: Vec::new(),
        },
    );

    // The edit door refuses the same node when the name is undeclared.
    let missing = editor_core::ParamName::new("nowhere");
    match apply(
        &doc,
        &DocEdit::InsertNode {
            node: Node::Measure {
                expr: MeasureExpr::value(Expr::param(missing.clone(), Dimension::Length)),
                refs: Vec::new(),
            },
        },
        Tol::witness(),
    ) {
        Err(EditError::UnknownPayloadParam { name: n, .. }) => assert_eq!(n, missing),
        other => panic!("the edit door must refuse an undeclared payload param, got {other:?}"),
    }

    let text = save(&doc, &[], Tol::witness()).expect("the fixture saves");
    load(&text, Tol::witness()).expect("the fixture loads");
    let corrupt = doctored(&text, |wire| {
        let params = wire["snapshot"]["params"]
            .as_object_mut()
            .expect("the params are a map");
        assert!(params.remove(&name.0).is_some());
    });
    let verdict = load(&corrupt, Tol::witness());
    eprintln!("RV-PROBE2 measure node {measure:?}: load verdict {verdict:?}");
    assert!(
        verdict.is_ok(),
        "MEASURED: the residue is real — the load door admits a measure expression reading an \
         undeclared parameter, which the edit door refuses. Got {verdict:?}"
    );
}

/// PROBE 3 — the diagnostics shift the re-order could cause. A file
/// broken BOTH in a non-profile slot and in a way `validate_snapshot`
/// reports now reads the slot refusal; before the re-order it read the
/// structural one.
#[test]
fn rv_the_slot_walk_shadows_a_structural_refusal_it_did_not_shadow_before() {
    let (doc, pattern) = patterned();
    let text = save(&doc, &[], Tol::witness()).expect("the fixture saves");
    let corrupt = doctored(&text, |wire| {
        // (a) a non-profile slot retyped: spacing Length -> Angle.
        let lit = &mut wire["snapshot"]["nodes"][pattern.0.to_string()]["Pattern"]["kind"]
            ["Linear"]["spacing"]["Literal"];
        assert_eq!(lit["dim"], serde_json::json!("Length"));
        lit["dim"] = serde_json::json!("Angle");
        lit["unit"] = serde_json::json!("rad");
        // (b) the recorded ε broken too — `EpsilonInvalid`, a
        // `validate_snapshot` refusal.
        wire["snapshot"]["epsilon"] = serde_json::json!(-1.0);
    });
    let verdict = load(&corrupt, Tol::witness());
    eprintln!("RV-PROBE3 verdict {verdict:?}");
}
