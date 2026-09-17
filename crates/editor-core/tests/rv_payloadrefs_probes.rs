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
//! - The dimension arm's rendered prose names the node (PROBE 3): the
//!   F6 case in `display_contract` does not read that word, so this
//!   row is what reds when the address the arm exists to carry is
//!   dropped from its `Display`.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;

use editor_core::{
    Dimension, DocEdit, DocParam, EditError, Expr, MeasureExpr, Node, ParamName, PatternKind,
    PersistError, ProfileDoc, RecipeNodeId, SnapshotError, apply, load, save,
};
use fixture::{ang, insert, len, on_frame, scl, square};
use geom_core::Tol;

/// Wire surgery by path, as `load_door_payload_param_ref::doctored`.
fn doctored(text: &str, edit: impl FnOnce(&mut serde_json::Value)) -> String {
    let split = text.find('{').expect("the JSON body follows the id header");
    let (header, body) = text.split_at(split);
    let mut wire: serde_json::Value = serde_json::from_str(body).expect("the body parses");
    edit(&mut wire);
    let out = format!("{header}{wire}");
    assert_ne!(out, text, "the corruption really landed");
    out
}

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
    ) {
        Err(EditError::PayloadParamDimensionMismatch {
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

/// **PROBE 3 — the dimension arm's prose names the NODE.** The arm
/// exists because the address of a payload expression is its node
/// (`persist::check`'s `PayloadDocParamDimension` doc), and its
/// `Display` writes that address — but the F6 case for it in
/// `display_contract.rs` reads only `measurement payload`, `depth`,
/// `as a length` and `declared angle`, so a `Display` that dropped the
/// node renders a refusal with no address at all and no row reds.
/// This is that row.
#[test]
fn rv_the_payload_dimension_refusal_names_the_node_it_addresses() {
    let rendered = format!(
        "{}",
        SnapshotError::PayloadDocParamDimension {
            node: RecipeNodeId(5),
            name: ParamName::new("depth"),
            declared: Dimension::Angle,
            referenced: Dimension::Length,
        }
    );
    assert!(
        rendered.contains("node 5"),
        "the payload refusals' whole reason for being a second pair of arms is that their \
         address is the NODE; the dimension arm must render it: {rendered}"
    );
}

/// **PROBE 4, MEASURED — the payload refusals call an ASSERTION's
/// bound a "measurement payload".** Both arms render one sentence for
/// both payload expressions, and the noun that sentence uses is the
/// measure's. An assertion node whose BOUND reads an undeclared
/// parameter is reported as "node N: its measurement payload reads …",
/// which is the wrong noun for the only other expression the walk can
/// be looking at: `Node::Assertion`'s bound is a bound, and the edit
/// door's twin (`EditError::UnknownPayloadParam`) says "payload"
/// without claiming which kind.
///
/// This row records what the prose says today and reds if the wording
/// is repaired, which is the point — nothing else reads these words
/// for an assertion, because the F6 case in `display_contract.rs`
/// builds the arm directly and never reaches an assertion fixture.
#[test]
fn rv_the_payload_refusal_calls_an_assertion_bound_a_measurement() {
    let rendered = format!(
        "{}",
        SnapshotError::PayloadUnknownDocParam {
            node: RecipeNodeId(7),
            name: ParamName::new("depth"),
        }
    );
    assert!(
        rendered.contains("its measurement payload reads"),
        "MEASURED: one sentence serves both payload expressions: {rendered}"
    );
}
