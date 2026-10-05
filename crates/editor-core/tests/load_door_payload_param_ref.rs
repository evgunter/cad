//! **A PAYLOAD expression reads a minted variable, and a live one at
//! its kind, at EVERY door** (spec D6, `Doc::var_read_faults`).
//!
//! The expressions no slot addresses — a `Node::Measure`'s
//! `MeasureExpr` value leaves and a `Node::Assertion`'s bound
//! (`node::payload_exprs`) — ask the same one predicate the slot
//! expressions ask. The two doors only name its answer: the edit door
//! as `EditError::PayloadUnknownVarName` (a name it cannot lower) and
//! `EditError::PayloadVarKind`, the load door as
//! `SnapshotError::ReaderOfUnmintedVar` and
//! `SnapshotError::PayloadVarKind`. So a file cannot carry a
//! payload expression an edit door would have refused.
//!
//! **One row per FACT, naming both doors' refusals for it**: a
//! predicate dropped at either door reds the fact's row and the panic
//! says which door let the document through.
//!
//! What is NOT here: the payload expressions' DIMENSIONS, which are
//! fixed at construction — a `MeasureExpr` runs the F1 checker at every
//! constructor, and an assertion's bound is checked against its
//! measure's dimension (`Node::assertion_bound_fault`, the load door's
//! `SnapshotError::AssertionBound`). This suite is the param TABLE's
//! half of the same address, and nothing here re-checks those.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;
use editor_core::ExtrudeSide;

use crate::wire::doctored;
use editor_core::{
    Dimension, DocEdit, EditError, Expr, FreeVar, MeasureExpr, Node, PersistError, ProfileDoc,
    RecipeNodeId, SlotId, SnapshotError, VarName, apply, load, save,
};
use fixture::{insert, len, on_frame, square};
use geom_core::Tol;

/// A frame, a profile, an extrude and a LENGTH document parameter
/// `depth` — the ground every row below builds its payload node on,
/// with the EXTRUDE's id, which only the rows that need a SLOT to
/// break read.
fn with_depth_and_extrude() -> (ProfileDoc, VarName, RecipeNodeId) {
    let name = VarName::from_static("depth");
    let (doc, profile) = on_frame(
        ProfileDoc::empty(
            editor_core::DocumentId::derive("payload-param-ref"),
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
            side: ExtrudeSide::Along,
        },
    );
    let doc = apply(
        &doc,
        &DocEdit::DeclareVar {
            name: name.clone(),
            def: editor_core::VarDecl::Free(FreeVar::continuous(Dimension::Length, 1.0)),
        },
        Tol::witness(),
        &editor_core::RefusingReach,
    )
    .expect("a well-formed length parameter declares")
    .doc;
    (doc, name, extrude)
}

/// The same ground for the rows whose fault is in a PAYLOAD and needs
/// no slot address — the one place the extrude's id is dropped, so no
/// row below writes `_` for a value it was handed.
fn with_depth() -> (ProfileDoc, VarName) {
    let (doc, name, _) = with_depth_and_extrude();
    (doc, name)
}

/// [`with_depth`] plus a measure whose expression reads `depth` — and
/// carries a LITERAL beside it, `-0.0`.
///
/// The literal is what gives the round-trip row something only
/// `bit_eq` can see: `-0.0 == 0.0` is true in IEEE arithmetic and the
/// two have different bits, so a payload channel whose comparison had
/// silently degraded to `==`, or a writer that normalised the sign of
/// zero on the way out, passes on every other value and fails on this
/// one. `MeasureExpr::add` keeps both leaves at `Length`, so the F1
/// checker admits it and the value the measure reports is unchanged.
fn measuring_depth() -> (ProfileDoc, VarName, RecipeNodeId) {
    let (doc, name) = with_depth();
    let expr = MeasureExpr::add(
        MeasureExpr::value(Expr::named(name.clone(), Dimension::Length)),
        MeasureExpr::value(len(-0.0)),
    )
    .expect("two length leaves add");
    let (doc, measure) = insert(
        doc,
        Node::Measure {
            expr,
            refs: Vec::new(),
        },
    );
    (doc, name, measure)
}

/// The variable removed from the wire and from its mint log, out from
/// under whatever reads it.
fn unmint(text: &str, name: &VarName) -> String {
    doctored(text, |wire| {
        crate::wire::wire_unmint(wire, name.as_str());
    })
}

/// The id `name` holds in `doc`.
fn id_of(doc: &ProfileDoc, name: &VarName) -> editor_core::VarId {
    doc.var_named(name.as_str()).expect("a declared variable")
}

/// The declaration RETYPED on the wire, its display unit moved with it
/// so the document is broken in exactly one way: the pairing between a
/// declaration and the dimension an expression reads it at.
fn retype_to_angle(text: &str, name: &VarName) -> String {
    doctored(text, |wire| {
        crate::wire::wire_retype(wire, name.as_str(), "Length", "Angle", "rad");
    })
}

/// **A measured expression reading an undeclared parameter — both
/// doors.** The edit door refuses the node as it is written; the load
/// door refuses the file whose declaration is gone from under it.
///
/// This row is the flip of the measurement that disclosed the gap
/// (`work/edit/load-door-does-not-check-payload-expression-param-refs`):
/// the same saved document loaded clean while the edit door refused
/// the same node, and this is that sentence with the load door's
/// answer in it.
#[test]
fn a_measure_expression_reading_an_undeclared_parameter_refuses_to_load() {
    let (doc, name, measure) = measuring_depth();

    // The edit door, over the node as written.
    let missing = VarName::from_static("nowhere");
    match apply(
        &doc,
        &DocEdit::InsertNode {
            node: Box::new(Node::Measure {
                expr: MeasureExpr::value(Expr::named(missing.clone(), Dimension::Length)),
                refs: Vec::new(),
            }),
        },
        Tol::witness(),
        &editor_core::RefusingReach,
    ) {
        Err(EditError::PayloadUnknownVarName { name: n, .. }) => assert_eq!(n, missing),
        other => panic!("the edit door must refuse an undeclared payload param, got {other:?}"),
    }

    // The load door, over the file whose variable was never minted.
    let text = save(&doc, &[], Tol::witness()).expect("the fixture saves");
    load(&text, Tol::witness()).expect("the fixture loads");
    match load(&unmint(&text, &name), Tol::witness()) {
        Err(PersistError::Snapshot(SnapshotError::ReaderOfUnmintedVar { node, var })) => {
            assert_eq!((node.id(), var), (measure, id_of(&doc, &name)));
        }
        other => panic!("the load door must refuse an unminted payload reader, got {other:?}"),
    }
}

/// **A measured expression reading a parameter at another dimension
/// than it is declared with — both doors.** The edit door cannot write
/// the pairing at all: a variable's kind is fixed, so the retyping is
/// what it refuses; the load door reads the broken pairing off a file
/// whose declaration was retyped after the fact.
#[test]
fn a_measure_expression_reading_a_parameter_at_the_wrong_dimension_refuses_to_load() {
    let (doc, name, measure) = measuring_depth();

    match apply(
        &doc,
        &DocEdit::DefineVar {
            var: name.clone().into(),
            def: editor_core::VarDecl::Free(FreeVar::continuous(Dimension::Angle, 1.0)),
        },
        Tol::witness(),
        &editor_core::RefusingReach,
    ) {
        Err(EditError::VarKindFixed { var, kind, offered }) => {
            assert_eq!(var.name(), Some(&name));
            assert_eq!(
                (kind, offered),
                (editor_core::VarKind::Length, editor_core::VarKind::Angle)
            );
        }
        other => panic!("the edit door must refuse the retyping, got {other:?}"),
    }

    let text = save(&doc, &[], Tol::witness()).expect("the fixture saves");
    match load(&retype_to_angle(&text, &name), Tol::witness()) {
        Err(PersistError::Snapshot(SnapshotError::PayloadVarKind {
            node,
            var,
            declared,
            referenced,
        })) => {
            assert_eq!((node.id(), var.id()), (measure, id_of(&doc, &name)));
            assert_eq!(
                (declared, referenced),
                (Dimension::Angle, Dimension::Length)
            );
        }
        other => panic!("the load door must refuse the broken pairing, got {other:?}"),
    }
}

/// **An assertion's BOUND is a payload expression too — both doors.**
/// The second of the two expressions no slot addresses, so the walk
/// that misses it misses half the vocabulary.
#[test]
fn an_assertion_bound_reading_an_undeclared_parameter_refuses_to_load() {
    let (doc, name) = with_depth();
    let (doc, measure) = insert(
        doc,
        Node::Measure {
            expr: MeasureExpr::value(len(1.0)),
            refs: Vec::new(),
        },
    );
    let bound = |n: &VarName| Node::Assertion {
        measure,
        bound: Expr::named(n.clone(), Dimension::Length),
        dir: editor_core::AssertionDir::AtLeast,
    };
    let (doc, assertion) = insert(doc, bound(&name));

    let missing = VarName::from_static("nowhere");
    match apply(
        &doc,
        &DocEdit::InsertNode {
            node: Box::new(bound(&missing)),
        },
        Tol::witness(),
        &editor_core::RefusingReach,
    ) {
        Err(EditError::PayloadUnknownVarName { name: n, .. }) => assert_eq!(n, missing),
        other => panic!("the edit door must refuse an undeclared bound param, got {other:?}"),
    }

    let text = save(&doc, &[], Tol::witness()).expect("the fixture saves");
    load(&text, Tol::witness()).expect("the fixture loads");
    match load(&unmint(&text, &name), Tol::witness()) {
        Err(PersistError::Snapshot(SnapshotError::ReaderOfUnmintedVar { node, var })) => {
            assert_eq!((node.id(), var), (assertion, id_of(&doc, &name)));
        }
        other => panic!("the load door must refuse an unminted bound reader, got {other:?}"),
    }
}

/// **A well-formed payload reference round-trips.** The walk refuses a
/// broken pairing and nothing else: a measure reading a declared
/// parameter at its declared dimension saves, loads, and comes back
/// bit for bit.
///
/// The payload carries `-0.0` beside the reference ([`measuring_depth`])
/// so that the BITS are what this row reads. Signed zero is the one
/// f64 value for which `==` and bit equality disagree, so a `bit_eq`
/// that had degraded to `==` on the payload channel — or a writer that
/// normalised the sign away — is green over any other literal and red
/// here. The row asserts the sign survived on the loaded document too,
/// so the panic says WHICH half broke.
#[test]
fn a_payload_reference_the_table_answers_round_trips() {
    let (doc, _, measure) = measuring_depth();
    let text = save(&doc, &[], Tol::witness()).expect("the fixture saves");
    let loaded = load(&text, Tol::witness()).expect("the fixture loads").doc;
    assert!(
        signed_zero_leaf(&loaded, measure),
        "the payload's `-0.0` came back as `+0.0`: the saved text is {text}"
    );
    assert!(
        loaded.bit_eq(&doc),
        "a payload expression the param table answers round-trips"
    );
}

/// Whether the measure node's payload still carries a NEGATIVE zero —
/// read off the loaded document rather than off the wire, because the
/// claim is about the value that reaches memory, and by BITS, which is
/// the only comparison that can tell `-0.0` from `0.0`.
fn signed_zero_leaf(doc: &editor_core::ProfileDoc, measure: RecipeNodeId) -> bool {
    let node = doc.node(measure).expect("the measure survived");
    editor_core::node::payload_exprs(node)
        .into_iter()
        .flatten()
        .any(|expr| {
            let mut bits = Vec::new();
            expr.literal_bits(&mut bits);
            bits.contains(&(-0.0f64).to_bits())
        })
}

/// **The walk ORDER, pinned**: a document broken in a SLOT expression
/// and in a PAYLOAD expression at once reads the SLOT refusal, because
/// the slot read walk runs first (`persist::check::Walk::ORDER`).
///
/// Moving the payload walk ahead of the slot walk changes the
/// diagnosis of every file broken both ways, and this row is what says
/// so out loud.
#[test]
fn a_document_broken_in_a_slot_and_in_a_payload_reads_the_slot_refusal() {
    let (doc, name, extrude) = with_depth_and_extrude();
    let doc = apply(
        &doc,
        &DocEdit::SetParam {
            node: extrude,
            slot: SlotId::Distance,
            expr: Expr::named(name.clone(), Dimension::Length),
        },
        Tol::witness(),
        &editor_core::RefusingReach,
    )
    .expect("a length parameter drives a length slot")
    .doc;
    let (doc, _) = insert(
        doc,
        Node::Measure {
            expr: MeasureExpr::value(Expr::named(name.clone(), Dimension::Length)),
            refs: Vec::new(),
        },
    );

    let text = save(&doc, &[], Tol::witness()).expect("the fixture saves");
    match load(&retype_to_angle(&text, &name), Tol::witness()) {
        Err(PersistError::Snapshot(SnapshotError::SlotVarKind {
            node, slot, var, ..
        })) => assert_eq!(
            (node.id(), slot, var.id()),
            (extrude, SlotId::Distance, id_of(&doc, &name))
        ),
        other => panic!(
            "a file broken in a slot AND in a payload must read the slot walk's refusal — the \
             walk order `validate_document` documents. Got {other:?}"
        ),
    }
}

/// **The walk order again, at the other edge**: a node broken in a
/// PAYLOAD expression and STRUCTURALLY at once reads the PAYLOAD
/// refusal, because both param-ref walks run before
/// `persist::check::Walk::Snapshot`.
///
/// The fixture is an assertion whose bound reads `depth` and whose
/// target is the EXTRUDE — a node that is not a measure, which
/// `Node::assertion_bound_fault` refuses as
/// `SnapshotError::AssertionTarget` — with `depth` undeclared in the
/// same file. Both faults are real and only one sentence comes back;
/// this row says which, so moving the payload walk behind the
/// structural walk changes a diagnosis with a row on it rather than
/// silently.
///
/// It also says the edit door could not have produced the file: the
/// same assertion offered to `InsertNode` is refused, at the target.
#[test]
fn an_assertion_bound_on_a_non_measure_reads_the_payload_refusal() {
    let (doc, name, extrude) = with_depth_and_extrude();
    let (doc, measure) = insert(
        doc,
        Node::Measure {
            expr: MeasureExpr::value(len(1.0)),
            refs: Vec::new(),
        },
    );
    let (doc, assertion) = insert(
        doc,
        Node::Assertion {
            measure,
            bound: Expr::named(name.clone(), Dimension::Length),
            dir: editor_core::AssertionDir::AtLeast,
        },
    );

    // The edit door, over the node the wire surgery below forges.
    match apply(
        &doc,
        &DocEdit::InsertNode {
            node: Box::new(Node::Assertion {
                measure: extrude,
                bound: Expr::named(name.clone(), Dimension::Length),
                dir: editor_core::AssertionDir::AtLeast,
            }),
        },
        Tol::witness(),
        &editor_core::RefusingReach,
    ) {
        Err(EditError::AssertionTarget { .. }) => {}
        other => panic!("the edit door must refuse an assertion on a non-measure, got {other:?}"),
    }

    let text = save(&doc, &[], Tol::witness()).expect("the fixture saves");
    let corrupt = doctored(&text, |wire| {
        let field = &mut wire["snapshot"]["nodes"][assertion.0.to_string()]["Assertion"]["measure"];
        assert_eq!(
            *field,
            serde_json::json!(measure.0),
            "the surgery is aimed at the assertion's target"
        );
        *field = serde_json::json!(extrude.0);
        crate::wire::wire_unmint(wire, name.as_str());
    });

    match load(&corrupt, Tol::witness()) {
        Err(PersistError::Snapshot(SnapshotError::ReaderOfUnmintedVar { node, var })) => {
            assert_eq!((node.id(), var), (assertion, id_of(&doc, &name)));
        }
        other => panic!(
            "a node broken in a payload AND structurally must read the PAYLOAD walk's refusal — \
             both read walks run before `Walk::Snapshot`. Got {other:?}"
        ),
    }
}
