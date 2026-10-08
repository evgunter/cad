//! **The measurement vocabulary on the wire** (ERROR-DESIGN E3/E10,
//! CONTACT-DESIGN C5): `Node::Measure` and `Node::Assertion`, every
//! primitive leaf and every arithmetic arm, round-tripped bit for bit,
//! and their door refusals. (The format carries no schema version —
//! the persist module docs say why — so there is no version pin here
//! and no older golden to refuse.)
//!
//! **What the frozen golden does NOT carry, and why it lives here.**
//! `m4_pr6_golden.rs`'s document must evaluate green, and this
//! document's only well-known reference is a whole BODY, which no
//! primitive has a closed form for. So the golden pins the node
//! shapes, the reference list and the arithmetic leaves, and the three
//! PRIMITIVE leaves are pinned here instead, by save-load-`bit_eq`
//! round trip over a document that carries all of them.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture::{ang, len, scl};
use editor_core::UnitSym;
use editor_core::expr::DimensionError;
use editor_core::{
    AssertionDir, Datum, Dimension, DocEdit, DocumentId, EditError, EntityKind, Formula, FreeVar,
    MeasureExpr, MeasureNodeFault, MeasurePrimitive, Node, PersistError, ProfileDoc, RecipeNodeId,
    RoleSeg, SitedRef, SnapshotError, StableName, VarName, apply, load, save,
};
use geom_core::Tol;

/// Two datum points, so the measure's references name nodes that
/// EXIST — the insert door checks that, and a wire fixture must pass
/// the same doors a real document does. Nothing here is evaluated:
/// these rows are about the wire.
fn two_named_nodes(doc: &ProfileDoc) -> ProfileDoc {
    let mut doc = doc.clone();
    for x in [0.0, 1.0] {
        doc = apply(
            &doc,
            &DocEdit::InsertNode {
                node: Box::new(Node::Datum(Datum::Point {
                    position: [len(x), len(0.0), len(0.0)],
                })),
                fresh: Vec::new(),
            },
            Tol::witness(),
            &editor_core::RefusingReach,
        )
        .expect("a datum point inserts")
        .doc;
    }
    doc
}

fn name(node: RecipeNodeId) -> SitedRef {
    // Read at the minting node: these fixtures are about the WIRE, and
    // none of them places geometry.
    SitedRef::at_mint(StableName {
        kind: EntityKind::Face,
        node,
        path: vec![RoleSeg::OutputBody],
    })
}

/// A document carrying a measure with every primitive leaf and every
/// arithmetic arm, plus an assertion over it. Adding a
/// `MeasurePrimitive` variant without adding it here leaves that
/// variant's wire form unexercised, so this fixture is where the
/// table's growth is checked.
fn every_form() -> ProfileDoc {
    let mut doc = ProfileDoc::empty(DocumentId::derive("m10-2-measure-wire"), Tol::witness());
    let push = |d: &ProfileDoc, e: &DocEdit<editor_core::ProfileProgram>| {
        apply(d, e, Tol::witness(), &editor_core::RefusingReach)
            .expect("a valid edit applies")
            .doc
    };
    doc = push(
        &doc,
        &DocEdit::DeclareVar {
            name: VarName::from_static("pad"),
            def: editor_core::VarDecl::Free(FreeVar::Continuous {
                dim: Dimension::Length,
                value: 0.001,
                display_unit: UnitSym::canonical_for(Dimension::Length),
                distribution: None,
            }),
        },
    );
    let prim = |p: MeasurePrimitive<u32>| MeasureExpr::primitive(p);
    let scalar = |v: f64| MeasureExpr::value(scl(v));
    // distance - gap + min_clearance, halved, floored by a parameter and
    // ceilinged by a literal: every arithmetic arm and every Length
    // primitive at once. `min_clearance` (M10-6) is here for the same
    // reason the other three are — this fixture IS the populated wire
    // golden for the primitive table, so a primitive absent from it
    // round-trips under no test at all.
    let expr = MeasureExpr::max(
        MeasureExpr::min(
            MeasureExpr::div(
                MeasureExpr::mul(
                    MeasureExpr::add(
                        MeasureExpr::sub(
                            MeasureExpr::add(
                                prim(MeasurePrimitive::Distance { a: 0, b: 1 }),
                                prim(MeasurePrimitive::MinClearance { a: 0, b: 1 }),
                            )
                            .expect("Length + Length"),
                            prim(MeasurePrimitive::Gap { outer: 1, inner: 0 }),
                        )
                        .expect("Length - Length"),
                        MeasureExpr::neg(MeasureExpr::value(Formula::named(
                            VarName::from_static("pad"),
                            Dimension::Length,
                        )))
                        .expect("a shallow negation"),
                    )
                    .expect("Length + Length"),
                    scalar(2.0),
                )
                .expect("Length * Scalar"),
                scalar(4.0),
            )
            .expect("Length / Scalar"),
            MeasureExpr::value(len(1.0)),
        )
        .expect("Length min Length"),
        MeasureExpr::value(len(-0.0)),
    )
    .expect("Length max Length");
    doc = two_named_nodes(&doc);
    let refs = [name(doc.ids()[0]), name(doc.ids()[1])];
    let (mut doc, measured) = crate::fixture::measure(doc, &expr, &refs);
    doc = push(
        &doc,
        &DocEdit::InsertNode {
            node: Box::new(Node::Assertion {
                value: measured.value,
                bound: len(0.0005),
                dir: AssertionDir::AtMost,
            }),
            fresh: Vec::new(),
        },
    );
    doc
}

/// Both fixtures put their first measure third and their assertion
/// last: two datum points come first so the references name live
/// nodes.
fn measure(doc: &ProfileDoc) -> RecipeNodeId {
    doc.ids()[2]
}

fn assertion(doc: &ProfileDoc) -> RecipeNodeId {
    *doc.ids().last().expect("an assertion")
}

/// The angular half, separately: an `angle` primitive is an `Angle`
/// measure and therefore takes an `Angle` bound. One document proves
/// the dimension rides the expression rather than being fixed per node.
fn angular() -> ProfileDoc {
    let mut doc = ProfileDoc::empty(DocumentId::derive("m10-2-angle"), Tol::witness());
    let push = |d: &ProfileDoc, e: &DocEdit<editor_core::ProfileProgram>| {
        apply(d, e, Tol::witness(), &editor_core::RefusingReach)
            .expect("a valid edit applies")
            .doc
    };
    doc = two_named_nodes(&doc);
    let refs = [name(doc.ids()[0]), name(doc.ids()[1])];
    let (mut doc, measured) = crate::fixture::measure(
        doc,
        &MeasureExpr::primitive(MeasurePrimitive::Angle { a: 0, b: 1 }),
        &refs,
    );
    doc = push(
        &doc,
        &DocEdit::InsertNode {
            node: Box::new(Node::Assertion {
                value: measured.value,
                bound: ang(0.5),
                dir: AssertionDir::AtLeast,
            }),
            fresh: Vec::new(),
        },
    );
    doc
}

/// Every wire arm round-trips, and BIT-exactly: the `-0.0` literal in
/// the measured value's definition is the point — a value-blind
/// comparator would pass here with `0.0` on the wire.
#[test]
fn every_measure_form_round_trips() {
    for doc in [every_form(), angular()] {
        let text = save(&doc, &[], Tol::witness()).expect("the document saves");
        let back = load(&text, Tol::witness()).expect("its own bytes load").doc;
        for id in doc.ids() {
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

/// A measure's dimension is its primitive's, and an assertion's value
/// is read at its variable's — the E3 claim, on the wire rather than
/// only in memory.
#[test]
fn the_quantity_kind_rides_the_primitive() {
    let length = every_form();
    let angle = angular();
    let dim_of = |doc: &ProfileDoc| match doc.node(measure(doc)) {
        Some(Node::Measure { primitive }) => primitive.dim(),
        other => panic!("expected a measure, got {other:?}"),
    };
    assert_eq!(dim_of(&length), Dimension::Length);
    assert_eq!(dim_of(&angle), Dimension::Angle);
    let value_dim = |doc: &ProfileDoc| {
        let value = crate::fixture::assertion_value(doc, assertion(doc));
        doc.var(value).and_then(|v| v.kind().dimension())
    };
    assert_eq!(value_dim(&length), Some(Dimension::Length));
    assert_eq!(value_dim(&angle), Some(Dimension::Angle));
}

/// **Measure arithmetic is a definition**, so a saved one the
/// dimension checker refuses crosses the load door the way any
/// definition does: whole, as `PersistError::Dimension`, carrying the
/// refusal rather than a sentence about it.
///
/// The tamper wraps the assertion's OWN value definition rather than
/// spelling one out, so the needle follows the expression wire.
#[test]
fn a_dimension_refusal_in_a_measured_definition_crosses_the_load_door_whole() {
    let doc = every_form();
    let value = crate::fixture::assertion_value(&doc, assertion(&doc));
    let text = save(&doc, &[], Tol::witness()).expect("the document saves");
    let (header, body_text) = text.split_once('\n').expect("a header line");
    let mut body: serde_json::Value = serde_json::from_str(body_text).expect("a JSON body");
    let held = &mut body["snapshot"]["vars"][value.0.to_string().as_str()]["def"]["Defined"];
    assert!(!held.is_null(), "the web is a definition");
    // A `Length` measurement plus an `Angle` reader: the arithmetic
    // constructor refuses it, exactly as it would at the edit door.
    let inner = held.take();
    *held = serde_json::json!({
        "Add": [
            inner,
            { "Var": { "var": value.0.to_string(), "dim": "Angle" } },
        ]
    });
    let corrupt = format!(
        "{header}\n{}",
        serde_json::to_string(&body).expect("re-emit")
    );
    match load(&corrupt, Tol::witness()) {
        Err(PersistError::Dimension { error, .. }) => assert_eq!(
            error,
            DimensionError::Mismatch {
                op: "add",
                left: Dimension::Length,
                right: Dimension::Angle,
            },
        ),
        other => panic!("an ill-dimensioned definition must refuse typed, got {other:?}"),
    }
}

/// An authored measurement indexing past the references it is handed
/// refuses at its builder, before anything is applied.
#[test]
fn a_measure_indexing_past_its_refs_refuses_at_the_builder() {
    let doc = two_named_nodes(&ProfileDoc::empty(
        DocumentId::derive("m10-2-index"),
        Tol::witness(),
    ));
    let err = editor_core::measure(
        &doc,
        &MeasureExpr::primitive(MeasurePrimitive::Gap { outer: 0, inner: 3 }),
        &[name(doc.ids()[0])],
        Tol::witness(),
        &editor_core::RefusingReach,
    )
    .expect_err("index 3 addresses nothing");
    assert!(matches!(
        err,
        EditError::MeasureMalformed {
            fault: MeasureNodeFault::RefIndexOutOfRange {
                index: 3,
                refs: 1,
                ..
            }
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
            node: Box::new(Node::Assertion {
                value: crate::fixture::value_of(&doc, measure(&doc)),
                bound: len(0.5),
                dir: AssertionDir::AtLeast,
            }),
            fresh: Vec::new(),
        },
        Tol::witness(),
        &editor_core::RefusingReach,
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

/// An assertion over something that is not a scalar at all: a datum
/// point's output is a pose, read at the bound's dimension.
#[test]
fn an_assertion_over_a_non_scalar_refuses() {
    let doc = angular();
    let point = doc.output(doc.ids()[0], 0).expect("a point defines its pose");
    let err = apply(
        &doc,
        &DocEdit::InsertNode {
            node: Box::new(Node::Assertion {
                value: Formula::var(point, Dimension::Angle),
                bound: ang(0.5),
                dir: AssertionDir::AtLeast,
            }),
            fresh: Vec::new(),
        },
        Tol::witness(),
        &editor_core::RefusingReach,
    )
    .expect_err("an assertion compares a value");
    assert!(
        matches!(err, EditError::PayloadVarKind { .. }),
        "a pose is no value to bound: {err:?}"
    );
}
