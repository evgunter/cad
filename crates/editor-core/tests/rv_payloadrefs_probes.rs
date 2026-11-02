//! **Review probes for `edit/load-door-payload-refs`** (lane
//! `payloadrefs-rv`): the claims the unit makes about the payload
//! param-ref walk, each row measuring one thing no row in
//! `load_door_payload_param_ref` measures.
//!
//! - Which door refuses a measure's dimension fault first (PROBE 2):
//!   the F1 checker at CONSTRUCTION for the arithmetic, and the
//!   param-ref doors for the param TABLE, which are different faults.
//! - The dimension arm's rendered prose names the node (PROBE 3) —
//!   ADOPTED: the fact is now a word of the F6 case for
//!   `SnapshotError::PayloadVarKind` in `display_contract`,
//!   which is the census that owns rendered prose, so the probe is
//!   gone rather than kept as a second copy of it.
//! - The noun both payload arms use (PROBE 4): it said "measurement
//!   payload" of an assertion's BOUND, which is the wrong noun for
//!   half the walk's domain. The wording is repaired at both doors,
//!   and the probe below pins the repaired sentence instead of the
//!   defect it measured.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;

use editor_core::{
    Dimension, DocEdit, EditError, Formula, FreeVar, Node, ProfileDoc, RecipeNodeId, SnapshotError,
    VarName, apply,
};
use fixture::{ang, insert, len, on_frame, square};
use geom_core::Tol;

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
    let name = VarName::from_static("depth");
    // The F1 checker, at construction, with no document in sight.
    let fault = Formula::add(Formula::named(name.clone(), Dimension::Length), ang(1.0));
    assert!(
        fault.is_err(),
        "a length added to an angle is refused by the F1 checker at construction"
    );

    // The param TABLE, which construction never asks: reading a
    // declared LENGTH parameter as an ANGLE builds fine.
    let leaf = Formula::named(name.clone(), Dimension::Angle);
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
            profile: profile.into(),
            distance: len(1.0),
        },
    );
    let doc = apply(
        &doc,
        &DocEdit::DeclareVar {
            name: name.clone(),
            def: editor_core::VarDecl::Free(FreeVar::continuous(Dimension::Length, 1.0)),
        },
        Tol::witness(),
    )
    .expect("a length parameter declares")
    .doc;

    match apply(
        &doc,
        &DocEdit::InsertNode {
            node: Box::new(Node::Assertion(editor_core::Claim::Value {
                value: leaf,
                bound: ang(0.0),
                relation: editor_core::AssertionRelation::AtLeast,
            })),
            fresh: Vec::new(),
        },
        Tol::witness(),
    ) {
        Err(EditError::PayloadVarKind {
            declared,
            referenced,
            ..
        }) => assert_eq!(
            (declared, referenced),
            (editor_core::VarKind::Length, Dimension::Angle)
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
/// here and at the edit door's twins (`EditError::PayloadUnknownVarName`
/// / `PayloadVarKind`). This row pins it over an
/// ASSERTION's refusal, which is the half nothing else renders: the F6
/// cases in `display_contract.rs` build the arms directly and never
/// reach an assertion fixture, so a regression to the measure's noun
/// reds here and nowhere else.
#[test]
fn rv_the_payload_refusal_names_a_noun_that_covers_an_assertion_bound() {
    let rendered = format!(
        "{}",
        SnapshotError::PayloadVarKind {
            node: editor_core::SpokenNode::absent(RecipeNodeId::new(0, 7)),
            var: editor_core::SpokenVar::new(
                editor_core::VarId::new(0, 3),
                Some(VarName::from_static("depth")),
            ),
            declared: editor_core::VarKind::Angle,
            referenced: editor_core::Dimension::Length,
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
        EditError::PayloadUnknownVarName {
            name: VarName::from_static("depth"),
            node: editor_core::SpokenNode::absent(RecipeNodeId::new(0, 7)),
        }
    );
    assert!(
        edit.contains("payload expression") && !edit.contains("measurement"),
        "the two doors spell the payload's noun the same way: {edit}"
    );
}
