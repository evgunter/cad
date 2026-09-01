//! **The measurement vocabulary on the wire** (ERROR-DESIGN E3/E10,
//! CONTACT-DESIGN C5) — schema **v17**, and the gate that refuses
//! everything older.
//!
//! Before v17 the node enum had no `Measure` and no `Assertion`. A v16
//! reader handed either meets a variant its `deny_unknown_fields` node
//! enum has no name for and dies inside serde rather than at the
//! version door — which is exactly the direction the gate buys. The
//! other direction is forgiving by construction (a v16 file contains
//! neither node), so the disposition is the family's: the older file
//! refuses TYPED with the regenerate recourse, and the migration table
//! stays empty.
//!
//! **Why 17.** Read by eye from main's constant at the final re-merge
//! (`git show origin/main:crates/editor-core/src/persist/mod.rs | grep
//! SCHEMA_VERSION`), because units have repeatedly had a same-number
//! claim merge CLEAN — both sides write the identical line, so git
//! never conflicts.
//!
//! **What the frozen golden does NOT carry, and why it lives here.**
//! `m4_pr6_golden.rs`'s document must evaluate green, and this
//! document's only well-known reference is a whole BODY, which no
//! primitive has a closed form for. So the golden pins the node
//! shapes, the reference list and the arithmetic leaves, and the three
//! PRIMITIVE leaves are pinned here instead, by save-load-`bit_eq`
//! round trip over a document that carries all of them.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use editor_core::UnitSym;
use editor_core::{
    AssertionDir, Datum, Dimension, DocEdit, DocParam, DocumentId, EditError, EntityKind, Expr,
    MeasureExpr, MeasureNodeFault, MeasurePrimitive, MeasureRef, Node, ParamName, PersistError,
    ProfileDoc, REGENERATE_RECOURSE, RecipeNodeId, RoleSeg, SCHEMA_VERSION, SnapshotError,
    StableName, apply, load, save,
};
use geom_core::Tol;

fn len(v: f64) -> Expr {
    Expr::literal(v, Dimension::Length).expect("finite")
}

/// Two datum points, so the measure's references name nodes that
/// EXIST — the insert door checks that, and a schema fixture must pass
/// the same doors a real document does. Nothing here is evaluated:
/// these rows are about the wire.
fn two_named_nodes(doc: &ProfileDoc) -> ProfileDoc {
    let mut doc = doc.clone();
    for x in [0.0, 1.0] {
        doc = apply(
            &doc,
            &DocEdit::InsertNode {
                node: Node::Datum(Datum::Point {
                    position: [len(x), len(0.0), len(0.0)],
                }),
            },
            Tol::witness(),
        )
        .expect("a datum point inserts")
        .doc;
    }
    doc
}

/// The prior live golden, kept as the REFUSAL fixture: a break nobody
/// can demonstrate is a break nobody can trust.
const V16: &str = include_str!("golden/v16_golden.cad");
/// One further back, to show the gate has no notion of "nearly
/// current".
const V15: &str = include_str!("golden/v15_golden.cad");

#[test]
fn schema_version_is_current() {
    assert_eq!(SCHEMA_VERSION, 20);
}

#[test]
fn the_checked_in_older_goldens_are_really_older() {
    assert_eq!(V16.lines().next(), Some("schema: 16"));
    assert_eq!(V15.lines().next(), Some("schema: 15"));
}

/// The break, demonstrated in the direction that matters: a v16 file
/// refuses TYPED at the version door, naming the version found, the
/// version supported, and the step that does not exist.
#[test]
fn v16_refuses_too_old() {
    match load(V16, Tol::witness()) {
        Err(PersistError::SchemaTooOld {
            found,
            supported,
            missing,
        }) => {
            assert_eq!(found, 16);
            assert_eq!(supported, SCHEMA_VERSION);
            assert_eq!(
                missing, 16,
                "the 16 -> 17 step is the one that does not exist"
            );
        }
        other => panic!("v16 must refuse SchemaTooOld, got {other:?}"),
    }
}

#[test]
fn the_refusal_carries_the_regenerate_recourse() {
    for (label, bytes) in [("v16", V16), ("v15", V15)] {
        let msg = match load(bytes, Tol::witness()) {
            Err(e) => e.to_string(),
            Ok(_) => panic!("{label} must refuse"),
        };
        assert!(msg.contains(REGENERATE_RECOURSE), "{label}: {msg}");
    }
}

fn name(node: u64) -> MeasureRef {
    // Read at the minting node: these fixtures are about the WIRE, and
    // none of them places geometry.
    MeasureRef::at_mint(StableName {
        kind: EntityKind::Face,
        node: RecipeNodeId(node),
        path: vec![RoleSeg::OutputBody],
    })
}

/// A document carrying a measure with every primitive leaf and every
/// arithmetic arm, plus an assertion over it.
fn every_form() -> ProfileDoc {
    let mut doc = ProfileDoc::empty(DocumentId::derive("m10-2-schema"), Tol::witness());
    let push = |d: &ProfileDoc, e: &DocEdit<editor_core::ProfileProgram>| {
        apply(d, e, Tol::witness())
            .expect("a valid edit applies")
            .doc
    };
    doc = push(
        &doc,
        &DocEdit::SetDocParam {
            name: ParamName::new("pad"),
            value: DocParam::Continuous {
                dim: Dimension::Length,
                value: 0.001,
                display_unit: UnitSym::canonical_for(Dimension::Length),
                distribution: None,
            },
        },
    );
    let prim = |p: MeasurePrimitive| MeasureExpr::primitive(p);
    let scalar = |v: f64| MeasureExpr::value(Expr::literal(v, Dimension::Scalar).expect("finite"));
    // distance - gap, halved, floored by a parameter and ceilinged by a
    // literal: every arithmetic arm and all three primitives at once.
    let expr = MeasureExpr::max(
        MeasureExpr::min(
            MeasureExpr::div(
                MeasureExpr::mul(
                    MeasureExpr::add(
                        MeasureExpr::sub(
                            prim(MeasurePrimitive::Distance { a: 0, b: 1 }),
                            prim(MeasurePrimitive::Gap { outer: 1, inner: 0 }),
                        )
                        .expect("Length - Length"),
                        MeasureExpr::neg(MeasureExpr::value(Expr::param(
                            ParamName::new("pad"),
                            Dimension::Length,
                        ))),
                    )
                    .expect("Length + Length"),
                    scalar(2.0),
                )
                .expect("Length * Scalar"),
                scalar(4.0),
            )
            .expect("Length / Scalar"),
            MeasureExpr::value(Expr::literal(1.0, Dimension::Length).expect("finite")),
        )
        .expect("Length min Length"),
        MeasureExpr::value(Expr::literal(-0.0, Dimension::Length).expect("finite")),
    )
    .expect("Length max Length");
    doc = two_named_nodes(&doc);
    doc = push(
        &doc,
        &DocEdit::InsertNode {
            node: Node::measure(expr, vec![name(0), name(1)]).expect("indices in range"),
        },
    );
    doc = push(
        &doc,
        &DocEdit::InsertNode {
            node: Node::Assertion {
                measure: MEASURE,
                bound: Expr::literal(0.0005, Dimension::Length).expect("finite"),
                dir: AssertionDir::AtMost,
            },
        },
    );
    doc
}

/// Both fixtures put their measure at the same id: two datum points
/// come first so the references name live nodes.
const MEASURE: RecipeNodeId = RecipeNodeId(2);
const ASSERTION: RecipeNodeId = RecipeNodeId(3);

/// The angular half, separately: an `angle` primitive is an `Angle`
/// measure and therefore takes an `Angle` bound. One document proves
/// the dimension rides the expression rather than being fixed per node.
fn angular() -> ProfileDoc {
    let mut doc = ProfileDoc::empty(DocumentId::derive("m10-2-angle"), Tol::witness());
    let push = |d: &ProfileDoc, e: &DocEdit<editor_core::ProfileProgram>| {
        apply(d, e, Tol::witness())
            .expect("a valid edit applies")
            .doc
    };
    doc = two_named_nodes(&doc);
    doc = push(
        &doc,
        &DocEdit::InsertNode {
            node: Node::measure(
                MeasureExpr::primitive(MeasurePrimitive::Angle { a: 0, b: 1 }),
                vec![name(0), name(1)],
            )
            .expect("indices in range"),
        },
    );
    doc = push(
        &doc,
        &DocEdit::InsertNode {
            node: Node::Assertion {
                measure: MEASURE,
                bound: Expr::literal(0.5, Dimension::Angle).expect("finite"),
                dir: AssertionDir::AtLeast,
            },
        },
    );
    doc
}

/// Every wire arm round-trips, and BIT-exactly: the `-0.0` literal in
/// the measured expression is the point — a value-blind comparator
/// would pass here with `0.0` on the wire.
#[test]
fn every_measure_form_round_trips_at_v17() {
    for doc in [every_form(), angular()] {
        let text = save(&doc, &[], Tol::witness()).expect("the document saves");
        assert_eq!(
            text.lines().next(),
            Some(&format!("schema: {SCHEMA_VERSION}")[..])
        );
        let back = load(&text, Tol::witness()).expect("its own bytes load").doc;
        for &id in doc.order() {
            let (mine, theirs) = (
                doc.node(id).expect("live"),
                back.node(id).expect("every node survives the round trip"),
            );
            assert!(
                mine.bit_eq(theirs),
                "node {id:?} did not round-trip bit-exactly"
            );
        }
        let again = save(&back, &[], Tol::witness()).expect("the reloaded document saves");
        assert_eq!(text, again, "save . load is not a fixpoint");
    }
}

/// The measured expression's dimension is what the payload reports —
/// the E3 claim, on the wire rather than only in memory.
#[test]
fn the_quantity_kind_rides_the_expression() {
    let length = every_form();
    let angle = angular();
    let dim_of = |doc: &ProfileDoc, id: u64| match doc.node(RecipeNodeId(id)) {
        Some(Node::Measure { expr, .. }) => expr.dim(),
        other => panic!("expected a measure, got {other:?}"),
    };
    assert_eq!(dim_of(&length, MEASURE.0), Dimension::Length);
    assert_eq!(dim_of(&angle, MEASURE.0), Dimension::Angle);
}

/// **The load door is the construction door.** A file whose measure
/// indexes past its reference list is CORRUPT — `Rebind` cannot repair
/// an index — so it refuses at the snapshot check rather than reaching
/// evaluation.
#[test]
fn a_measure_indexing_past_its_refs_refuses_at_the_load_door() {
    // Corrupt the BYTES, not a value: the construction and edit doors
    // both refuse this state, so the only way a document reaches the
    // loader carrying it is a file nothing in this build wrote — which
    // is exactly the input the load-door re-check exists for.
    let text = save(&angular(), &[], Tol::witness()).expect("the document saves");
    let corrupt = text.replace("\"b\": 1", "\"b\": 7");
    assert_ne!(corrupt, text, "the corruption must actually land");
    match load(&corrupt, Tol::witness()) {
        Err(PersistError::Snapshot(SnapshotError::MeasureRefs { node, fault })) => {
            assert_eq!(node, MEASURE);
            assert!(
                matches!(
                    fault,
                    MeasureNodeFault::RefIndexOutOfRange {
                        index: 7,
                        refs: 2,
                        ..
                    }
                ),
                "got {fault:?}"
            );
        }
        other => panic!("a past-the-end index must refuse MeasureRefs, got {other:?}"),
    }
}

/// The same fault at the EDIT door, which is where an author meets it.
#[test]
fn a_measure_indexing_past_its_refs_refuses_at_the_edit_door() {
    let err = Node::<editor_core::ProfileProgram>::measure(
        MeasureExpr::primitive(MeasurePrimitive::Gap { outer: 0, inner: 3 }),
        vec![name(0)],
    )
    .expect_err("index 3 addresses nothing");
    assert!(matches!(
        err,
        MeasureNodeFault::RefIndexOutOfRange {
            index: 3,
            refs: 1,
            ..
        }
    ));
}

/// An assertion's bound must be dimensioned like its measure — refused
/// at the edit door, so a document never carries a comparison of
/// metres with radians.
#[test]
fn a_dimension_mismatched_bound_refuses_at_the_edit_door() {
    let doc = angular();
    let err = apply(
        &doc,
        &DocEdit::InsertNode {
            node: Node::Assertion {
                measure: MEASURE,
                bound: Expr::literal(0.5, Dimension::Length).expect("finite"),
                dir: AssertionDir::AtLeast,
            },
        },
        Tol::witness(),
    )
    .expect_err("a Length bound on an Angle measure is refused");
    assert!(
        matches!(
            err,
            EditError::AssertionDimension {
                measured: Dimension::Angle,
                bound: Dimension::Length,
                ..
            }
        ),
        "got {err:?}"
    );
}

/// An assertion over something that is not a measure at all.
#[test]
fn an_assertion_over_a_non_measure_refuses() {
    let doc = angular();
    let err = apply(
        &doc,
        &DocEdit::InsertNode {
            node: Node::Assertion {
                // An assertion is not a measure.
                measure: ASSERTION,
                bound: Expr::literal(0.5, Dimension::Angle).expect("finite"),
                dir: AssertionDir::AtLeast,
            },
        },
        Tol::witness(),
    )
    .expect_err("an assertion constrains a measurement");
    assert!(
        matches!(err, EditError::AssertionTarget { .. }),
        "got {err:?}"
    );
}
