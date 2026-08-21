//! M5 PR 10 §1 — `Loft` and `Sweep` as ORDINARY recipe nodes, and the
//! schema-v2 round trip that carries them.
//!
//! The vocabulary claims are structural, so the rows are structural:
//! named slots with the right dimension and structural flag, DAG edges
//! that are the profile refs, edit-door validation, and a save/load
//! round trip that reproduces the document bit-for-bit.
//!
//! Evaluation is exercised too — the §10.3/§10.4 construction genuinely
//! runs. Since M6-3 the LOFT evaluates to a Body (S9 flip below); the
//! SWEEP node still stops at its named sub-frontier
//! (`NodeErrorKind::CurvedSolidFrontier`, the joined-path lane). Never
//! a silent skip.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod fixture;

use editor_core::{
    CancelToken, Dimension, DocEdit, EvalOptions, Expr, Node, NodeErrorKind, NodeResult,
    ProfileDoc, RecipeNodeId, SlotId, evaluate, load, save,
};
use fixture::{desc, insert};
use geom_core::Tol;

/// A Count literal — `Expr::count`, because `Expr::literal` REFUSES
/// `Dimension::Count` on purpose (Count literals are integers).
fn count(v: i64) -> Expr {
    Expr::count(v)
}

/// A square section at height `z`, scaled by `s`.
fn section(z: f64, s: f64) -> editor_core::ProfileProgram {
    desc(
        [0.0, 0.0, z],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![
            (0.0, 0.0),
            (2.0 * s, 0.0),
            (2.0 * s, 1.0 * s),
            (0.0, 1.0 * s),
        ]],
    )
}

/// A three-section loft document; the middle section is scaled, so no
/// wall is ruled (§5's "at least one non-affine pair").
fn loft_doc() -> (ProfileDoc, RecipeNodeId, Vec<RecipeNodeId>) {
    let mut doc = ProfileDoc::empty_derived("m5_pr10_nodes", Tol::witness());
    let mut profiles = Vec::new();
    for (z, s) in [(0.0, 1.0), (1.0, 1.6), (2.0, 1.0)] {
        let (d, id) = insert(doc, Node::Profile(section(z, s)));
        doc = d;
        profiles.push(id);
    }
    let (doc, loft) = insert(
        doc,
        Node::Loft {
            profiles: profiles.clone(),
            v_degree: count(2),
        },
    );
    (doc, loft, profiles)
}

// ---------------------------------------------------------------------
// §1: the vocabulary
// ---------------------------------------------------------------------

#[test]
fn the_new_slots_are_structural_counts() {
    for slot in [SlotId::VDegree, SlotId::Stations] {
        assert_eq!(slot.dimension(), Dimension::Count, "{slot:?}");
        assert!(slot.is_structural(), "{slot:?}");
    }
}

#[test]
fn loft_inputs_are_its_profiles_in_order() {
    let (doc, loft, profiles) = loft_doc();
    let node = doc.node(loft).expect("the loft node");
    assert_eq!(node.inputs(), profiles);
    assert_eq!(node.slots(), vec![SlotId::VDegree]);
    assert!(node.expr(SlotId::VDegree).is_some());
    assert!(node.expr(SlotId::Stations).is_none());
    assert!(node.named_nodes().is_empty());
}

#[test]
fn sweep_inputs_are_profile_then_path_and_it_carries_both_slots() {
    let mut doc = ProfileDoc::empty_derived("m5_pr10_nodes", Tol::witness());
    let (d, profile) = insert(doc, Node::Profile(section(0.0, 1.0)));
    doc = d;
    let (d, path) = insert(doc, Node::Profile(section(0.0, 1.0)));
    doc = d;
    let (doc, sweep) = insert(
        doc,
        Node::Sweep {
            profile,
            path,
            stations: count(5),
            v_degree: count(3),
        },
    );
    let node = doc.node(sweep).expect("the sweep node");
    assert_eq!(node.inputs(), vec![profile, path]);
    assert_eq!(node.slots(), vec![SlotId::Stations, SlotId::VDegree]);
    for slot in [SlotId::Stations, SlotId::VDegree] {
        assert!(node.expr(slot).is_some(), "{slot:?}");
    }
}

/// The edit door's existing rule applies unchanged: an input ref that
/// names no node is refused typed, and the loft is not inserted.
#[test]
fn a_dangling_profile_ref_refuses_at_the_edit_door() {
    let (doc, ..) = loft_doc();
    let bogus = RecipeNodeId(9999);
    let err = doc
        .apply(&DocEdit::InsertNode {
            node: Node::Loft {
                profiles: vec![bogus],
                v_degree: count(1),
            },
        }, Tol::witness())
        .expect_err("a dangling input must refuse");
    assert!(format!("{err:?}").contains("9999"), "{err:?}");
}

/// A wrong-dimension expression in a structural slot is refused by the
/// same door every other slot uses (spec D6).
#[test]
fn a_length_expression_in_the_v_degree_slot_refuses() {
    let (doc, _, profiles) = loft_doc();
    let err = doc
        .apply(&DocEdit::InsertNode {
            node: Node::Loft {
                profiles,
                v_degree: Expr::literal(2.0, Dimension::Length).unwrap(),
            },
        }, Tol::witness())
        .expect_err("a Length in a Count slot must refuse");
    let text = format!("{err:?}");
    assert!(text.contains("Count") || text.contains("Length"), "{text}");
}

// ---------------------------------------------------------------------
// §4: the round trip, at schema v2
// ---------------------------------------------------------------------

#[test]
fn a_loft_document_round_trips_bit_identically_at_schema_v2() {
    let (doc, ..) = loft_doc();
    let text = save(&doc, &[], Tol::witness()).expect("saves");
    assert_eq!(
        text.lines().next().map(str::to_owned),
        Some(format!("schema: {}", editor_core::SCHEMA_VERSION))
    );
    match load(&text, Tol::witness()) {
        Ok(loaded) => {
            assert!(loaded.snapshot.bit_eq(&doc), "loft snapshot drifted");
            // Save∘load is a fixpoint on the BYTES too.
            assert_eq!(save(&loaded.doc, &loaded.edits, Tol::witness()).expect("re-saves"), text);
        }
        Err(editor_core::PersistError::ToleranceConflict { .. }) => {
            // Only reachable if a prior document pinned another ε in
            // this process; the bytes still parsed, validated and
            // replayed (the ε door is last).
        }
        Err(other) => panic!("loft document failed to load: {other:?}"),
    }
}

#[test]
fn a_sweep_document_round_trips_bit_identically_at_schema_v2() {
    let mut doc = ProfileDoc::empty_derived("m5_pr10_nodes", Tol::witness());
    let (d, profile) = insert(doc, Node::Profile(section(0.0, 1.0)));
    doc = d;
    let (d, path) = insert(doc, Node::Profile(section(0.0, 2.0)));
    doc = d;
    let (doc, _) = insert(
        doc,
        Node::Sweep {
            profile,
            path,
            stations: count(4),
            v_degree: count(2),
        },
    );
    let text = save(&doc, &[], Tol::witness()).expect("saves");
    assert_eq!(
        text.lines().next().map(str::to_owned),
        Some(format!("schema: {}", editor_core::SCHEMA_VERSION))
    );
    match load(&text, Tol::witness()) {
        Ok(loaded) => {
            assert!(loaded.snapshot.bit_eq(&doc), "sweep snapshot drifted");
            assert_eq!(save(&loaded.doc, &loaded.edits, Tol::witness()).expect("re-saves"), text);
        }
        Err(editor_core::PersistError::ToleranceConflict { .. }) => {}
        Err(other) => panic!("sweep document failed to load: {other:?}"),
    }
}

// ---------------------------------------------------------------------
// Evaluation: the construction runs, the frontier is named
// ---------------------------------------------------------------------

/// Evaluates and hands the node's TYPED failure to `inspect`.
/// (`NodeErrorKind` carries kernel errors unaltered and is not
/// `Clone`, so the inspection happens in place.)
fn with_failure<R>(
    doc: &ProfileDoc,
    id: RecipeNodeId,
    inspect: impl FnOnce(&NodeErrorKind) -> R,
) -> R {
    let out = evaluate::<f64>(doc, None, &CancelToken::new(), &EvalOptions::default(), Tol::witness());
    match out.nodes.get(&id).expect("the node has a result") {
        NodeResult::Failed(error) => inspect(&error.kind),
        other => panic!("expected a typed failure, got {other:?}"),
    }
}

/// S9 FLIP (M6-3) of `a_well_formed_loft_reaches_the_named_frontier`:
/// the same document now EVALUATES — the loft frontier is gone
/// (`wire_loft` runs `sweep::loft_body`, tiers 1–3 green at rest) and
/// the node yields a Body. The pre-flip row asserted
/// `CurvedSolidFrontier` naming `Surface::Nurbs`/`Unimplemented`;
/// that text retired with the frontier.
#[test]
fn a_well_formed_loft_evaluates_to_a_body() {
    let (doc, loft, _) = loft_doc();
    let out = evaluate::<f64>(&doc, None, &CancelToken::new(), &EvalOptions::default(), Tol::witness());
    match out.nodes.get(&loft).expect("the node has a result") {
        NodeResult::Ok(v) => {
            assert!(
                matches!(v.payload, editor_core::ValuePayload::Body(_)),
                "expected a Body payload, got {}",
                v.payload.kind_name()
            );
        }
        other => panic!("expected the loft to evaluate, got {other:?}"),
    }
}

/// A loft whose sections do not correspond refuses with the GEOMETRIC
/// door, not the frontier — the §2 compatibility refusal is live.
#[test]
fn mismatched_sections_refuse_before_the_frontier() {
    let mut doc = ProfileDoc::empty_derived("m5_pr10_nodes", Tol::witness());
    let (d, a) = insert(doc, Node::Profile(section(0.0, 1.0)));
    doc = d;
    // A triangle where the other section is a quad: no correspondence.
    let (d, b) = insert(
        doc,
        Node::Profile(desc(
            [0.0, 0.0, 1.0],
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            vec![vec![(0.0, 0.0), (2.0, 0.0), (1.0, 1.0)]],
        )),
    );
    doc = d;
    let (doc, loft) = insert(
        doc,
        Node::Loft {
            profiles: vec![a, b],
            v_degree: count(1),
        },
    );
    with_failure(&doc, loft, |kind| match kind {
        NodeErrorKind::Skin(e) => {
            let msg = e.to_string();
            assert!(msg.contains("segments"), "{msg}");
        }
        other => panic!("expected a Skin refusal, got {other:?}"),
    });
}

/// A v-degree the section count cannot carry refuses typed, before any
/// surface is built.
#[test]
fn an_unusable_v_degree_refuses_typed() {
    let (doc, _, profiles) = loft_doc();
    let (doc, loft) = insert(
        doc,
        Node::Loft {
            profiles,
            v_degree: count(3), // == the section count
        },
    );
    with_failure(&doc, loft, |kind| match kind {
        NodeErrorKind::Skin(sweep::SkinError::BadDegree { degree, sections }) => {
            assert_eq!((*degree, *sections), (3, 3));
        }
        other => panic!("expected BadDegree, got {other:?}"),
    });
}

/// An input that is not a profile node refuses typed at the operand
/// door (`Loft`'s refs resolve to existing nodes only, spec §1).
#[test]
fn a_non_profile_input_refuses_typed() {
    let (doc, _, profiles) = loft_doc();
    let (doc, datum) = insert(
        doc,
        Node::Datum(editor_core::Datum::Point {
            position: [fixture::len(0.0), fixture::len(0.0), fixture::len(0.0)],
        }),
    );
    let (doc, loft) = insert(
        doc,
        Node::Loft {
            profiles: vec![profiles[0], datum],
            v_degree: count(1),
        },
    );
    with_failure(&doc, loft, |kind| match kind {
        NodeErrorKind::WrongOperand { input, .. } => assert_eq!(*input, datum),
        other => panic!("expected WrongOperand, got {other:?}"),
    });
}

/// **F1**: the Sweep node's frontier is its OWN arm — since M6-3 the
/// ONLY `CurvedSolidFrontier` arm (the loft's retired with the body
/// assembly): no recipe-expressible path exists to sweep, and the
/// message says why rather than implying a single-segment case the
/// recipe layer cannot build.
#[test]
fn a_sweep_node_reaches_its_own_wider_frontier() {
    let mut doc = ProfileDoc::empty_derived("m5_pr10_nodes", Tol::witness());
    let (d, profile) = insert(doc, Node::Profile(section(0.0, 1.0)));
    doc = d;
    let (d, path) = insert(doc, Node::Profile(section(0.0, 2.0)));
    doc = d;
    let (doc, sweep) = insert(
        doc,
        Node::Sweep {
            profile,
            path,
            stations: count(4),
            v_degree: count(2),
        },
    );
    with_failure(&doc, sweep, |kind| match kind {
        NodeErrorKind::CurvedSolidFrontier { what } => {
            assert!(what.contains("joined-path composition lane"), "{what}");
            assert!(
                what.contains("closed chain of two or more segments"),
                "{what}"
            );
            assert!(!what.contains("multi-segment"), "{what}");
        }
        other => panic!("expected the sweep frontier, got {other:?}"),
    });
}

/// The structural slots are still doors: a Sweep missing its Count
/// reads as a slot error, not as a frontier story.
#[test]
fn a_sweeps_structural_slots_are_checked_before_the_frontier() {
    let mut doc = ProfileDoc::empty_derived("m5_pr10_nodes", Tol::witness());
    let (d, profile) = insert(doc, Node::Profile(section(0.0, 1.0)));
    doc = d;
    let (d, path) = insert(doc, Node::Profile(section(0.0, 2.0)));
    doc = d;
    let (doc, sweep) = insert(
        doc,
        Node::Sweep {
            profile,
            path,
            stations: count(-3),
            v_degree: count(2),
        },
    );
    with_failure(&doc, sweep, |kind| match kind {
        NodeErrorKind::NonPositiveCount { count } => assert_eq!(*count, -3),
        other => panic!("expected NonPositiveCount, got {other:?}"),
    });
}
