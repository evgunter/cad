//! **Review probes for `edit/load-door-payload-refs`** (lane
//! `payloadrefs-rv`): the claims the unit makes about the payload
//! param-ref walk, each row measuring one thing no row in
//! `load_door_payload_param_ref` measures.
//!
//! - The walk's DOMAIN is complete only relative to `payload_exprs`.
//!   A node's `Expr` that neither `slots()` nor `payload_exprs`
//!   reaches exists — a `Node::Pattern`'s `count` under an `Explicit`
//!   rule — and PROBE 1 measures what the load door does with one:
//!   the file is refused, but by the STRUCTURAL walk and in another
//!   vocabulary, not by either param-ref walk.
//! - Which door refuses a measure's dimension fault first (PROBE 2):
//!   the F1 checker at CONSTRUCTION for the arithmetic, and the
//!   param-ref doors for the param TABLE, which are different faults.
//! - The dimension arm's rendered prose names the node (PROBE 3) —
//!   ADOPTED: the fact is now a word of the F6 case for
//!   `SnapshotError::PayloadDocParamDimension` in `display_contract`,
//!   which is the census that owns rendered prose, so the probe is
//!   gone rather than kept as a second copy of it.
//! - The noun both payload arms use (PROBE 4): it said "measurement
//!   payload" of an assertion's BOUND, which is the wrong noun for
//!   half the walk's domain. The wording is repaired at both doors,
//!   and the probe below pins the repaired sentence instead of the
//!   defect it measured.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;

use crate::wire::doctored;
use editor_core::{
    Dimension, DocEdit, DocParam, EditError, Expr, MeasureExpr, Node, ParamName, PatternKind,
    PersistError, ProfileDoc, RecipeNodeId, SnapshotError, apply, load, save,
};
use fixture::{ang, insert, len, on_frame, scl, square};
use geom_core::Tol;

/// A frame, a profile, an extrude and a COUNT document parameter
/// `howmany`,
/// plus a linear pattern whose count READS that parameter.
fn patterned_on_a_count_param() -> (ProfileDoc, ParamName, RecipeNodeId) {
    let name = ParamName::new("howmany");
    let (doc, profile) = on_frame(
        ProfileDoc::empty(
            editor_core::DocumentId::derive("rv-payloadrefs"),
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
    let doc = apply(
        &doc,
        &DocEdit::SetDocParam {
            name: name.clone(),
            value: DocParam::Count { value: 3 },
        },
        Tol::witness(),
        &editor_core::RefusingReach,
    )
    .expect("a count parameter declares")
    .doc;
    let (doc, pattern) = insert(
        doc,
        Node::Pattern {
            input: extrude,
            count: Expr::param(name.clone(), Dimension::Count),
            kind: PatternKind::Linear {
                direction: [scl(1.0), scl(0.0), scl(0.0)],
                spacing: len(3.0),
            },
        },
    );
    (doc, name, pattern)
}

/// **PROBE 1 — the walk's domain, at its edge.** `Node::slots()` gives
/// a pattern a `SlotId::Count` only while its rule is a STEPPED one
/// (`node::rule_slots`), and `payload_exprs` returns `None` for a
/// pattern at every rule. So a file carrying an `Explicit` rule AND a
/// count expression holds an `Expr` that NEITHER param-ref walk reads
/// — and this row measures what the load door does with one whose
/// parameter the document does not declare.
///
/// The answer is that the file is refused, by `Walk::Snapshot`'s
/// `placement_rule_fault` (the two-spellings-of-the-count state), so
/// the unit's "every payload expression is walked" claim holds in the
/// sense that matters: no document reaches memory carrying an
/// unchecked parameter reference. It does NOT hold in the sense the
/// arms' prose suggests — the refusal names neither the parameter nor
/// the reference, and it would still be `PlacementRule` if the
/// parameter were declared and well-typed.
#[test]
fn rv_an_expression_no_walk_reads_is_refused_structurally_not_as_a_param_ref() {
    let (doc, name, pattern) = patterned_on_a_count_param();
    let text = save(&doc, &[], Tol::witness()).expect("the fixture saves");
    load(&text, Tol::witness()).expect("the fixture loads");

    let corrupt = doctored(&text, |wire| {
        // The rule becomes EXPLICIT, which takes the count's slot away
        // without taking the count expression away.
        wire["snapshot"]["nodes"][pattern.0.to_string()]["Pattern"]["kind"] = serde_json::json!({
            "Explicit": [{
                "columns": [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]],
                "translation": [0.0, 0.0, 0.0]
            }]
        });
        // ... and the parameter it reads leaves the table.
        let params = wire["snapshot"]["params"]
            .as_object_mut()
            .expect("the params are a map");
        assert!(params.remove(&name.0).is_some());
    });

    let verdict = load(&corrupt, Tol::witness());
    match &verdict {
        Err(PersistError::Snapshot(SnapshotError::PlacementRule { .. })) => {}
        other => panic!(
            "a count expression under an `Explicit` rule is read by no param-ref walk; the file \
             must still be refused, and it is the STRUCTURAL walk that does it. Got {other:?}"
        ),
    }
    let rendered = format!("{}", verdict.expect_err("refused"));
    assert!(
        !rendered.contains(&name.0),
        "the refusal for an unreadable parameter reference does not name the parameter — that is \
         the cost of the domain edge, and this is the assertion that says so: {rendered}"
    );
}

/// **PROBE 2 — which door refuses a measure's dimension fault, and
/// which fault it is.** The two are not the same question and the
/// unit's walk doc says so; this row measures both halves.
///
/// The F1 checker runs at CONSTRUCTION and refuses ARITHMETIC over
/// mismatched dimensions — a length added to an angle — before any
/// document exists. It says nothing about the param table: a leaf
/// reading a declared LENGTH parameter as an ANGLE constructs clean,
/// and is refused at the EDIT door (and, per
/// `load_door_payload_param_ref`, at the load door) by the unit's two
/// arms.
#[test]
fn rv_the_f1_checker_refuses_arithmetic_and_the_param_table_refuses_the_reading() {
    let name = ParamName::new("depth");
    // The F1 checker, at construction, with no document in sight.
    let fault = MeasureExpr::add(
        MeasureExpr::value(Expr::param(name.clone(), Dimension::Length)),
        MeasureExpr::value(ang(1.0)),
    );
    assert!(
        fault.is_err(),
        "a length added to an angle is refused by the F1 checker at construction"
    );

    // The param TABLE, which construction never asks: reading a
    // declared LENGTH parameter as an ANGLE builds fine.
    let leaf = MeasureExpr::value(Expr::param(name.clone(), Dimension::Angle));
    let (doc, profile) = on_frame(
        ProfileDoc::empty(
            editor_core::DocumentId::derive("rv-payloadrefs-f1"),
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
            value: DocParam::continuous(Dimension::Length, 1.0),
        },
        Tol::witness(),
        &editor_core::RefusingReach,
    )
    .expect("a length parameter declares")
    .doc;

    match apply(
        &doc,
        &DocEdit::InsertNode {
            node: Node::Measure {
                expr: leaf,
                refs: Vec::new(),
            },
        },
        Tol::witness(),
        &editor_core::RefusingReach,
    ) {
        Err(EditError::PayloadDocParamDimension {
            declared,
            referenced,
            ..
        }) => assert_eq!(
            (declared, referenced),
            (Dimension::Length, Dimension::Angle)
        ),
        other => panic!(
            "the EDIT door is the first door to see a payload reading a declared parameter at \
             another dimension — construction admits it. Got {other:?}"
        ),
    }
}

/// **PROBE 4 — the payload refusals' noun covers BOTH payload
/// expressions.** One sentence serves a measure's expression and an
/// assertion's bound alike, so the noun it uses has to be true of
/// both: `Node::Assertion`'s bound is a bound, not a measurement, and
/// "measurement payload" — the wording this probe first measured —
/// was the measure's noun applied to both.
///
/// The repaired sentence says "payload expression", at the load door
/// here and at the edit door's twins (`EditError::PayloadUnknownDocParam`
/// / `PayloadDocParamDimension`). This row pins it over an
/// ASSERTION's refusal, which is the half nothing else renders: the F6
/// cases in `display_contract.rs` build the arms directly and never
/// reach an assertion fixture, so a regression to the measure's noun
/// reds here and nowhere else.
#[test]
fn rv_the_payload_refusal_names_a_noun_that_covers_an_assertion_bound() {
    let rendered = format!(
        "{}",
        SnapshotError::PayloadUnknownDocParam {
            node: RecipeNodeId(7),
            name: ParamName::new("depth"),
        }
    );
    assert!(
        rendered.contains("its payload expression reads"),
        "the noun has to be true of an assertion's bound as well as a measure's expression: \
         {rendered}"
    );
    assert!(
        !rendered.contains("measurement"),
        "an assertion's bound is not a measurement: {rendered}"
    );

    // The edit door's twin, the same noun.
    let edit = format!(
        "{}",
        EditError::PayloadUnknownDocParam {
            name: ParamName::new("depth"),
            node: RecipeNodeId(7),
        }
    );
    assert!(
        edit.contains("payload expression") && !edit.contains("measurement"),
        "the two doors spell the payload's noun the same way: {edit}"
    );
}
