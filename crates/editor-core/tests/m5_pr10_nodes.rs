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

use crate::fixture;

use editor_core::{
    CancelToken, Dimension, DocEdit, EvalOptions, Formula, Node, NodeErrorKind, NodeResult,
    ProfileDoc, RecipeNodeId, SlotId, evaluate, load, save,
};
use fixture::{insert, len, on_frame};
use geom_core::Tol;

/// A Count literal — `Formula::count`, because `Formula::literal` REFUSES
/// `Dimension::Count` on purpose (Count literals are integers).
fn count(v: i64) -> Formula {
    Formula::count(v)
}

/// A square section scaled by `s`, drawn on its own plane: a loft's
/// station stands it at its height ([`lofted`]).
fn section(doc: ProfileDoc, s: f64) -> (ProfileDoc, RecipeNodeId) {
    on_frame(
        doc,
        [0.0, 0.0, 0.0],
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

/// A loft over `profiles`, section `i` standing `i` up the first's own
/// z: distinct planes are what a loft's sections are.
fn lofted(profiles: &[RecipeNodeId], v_degree: Formula) -> editor_core::AuthoredNode {
    Node::Loft {
        first: profiles[0].into(),
        then: profiles[1..]
            .iter()
            .zip(1u32..)
            .map(|(&profile, i)| editor_core::LoftSection {
                profile: profile.into(),
                station: vec![editor_core::StationMove::Slide {
                    along: editor_core::Axis3::Z,
                    by: fixture::len(f64::from(i)),
                }],
            })
            .collect(),
        v_degree,
    }
}

/// A three-section loft document; the middle section is scaled, so no
/// wall is ruled (§5's "at least one non-affine pair").
fn loft_doc() -> (ProfileDoc, RecipeNodeId, Vec<RecipeNodeId>) {
    let mut doc = ProfileDoc::empty_derived("m5_pr10_nodes", Tol::witness());
    let mut profiles = Vec::new();
    for s in [1.0, 1.6, 1.0] {
        let (d, id) = section(doc, s);
        doc = d;
        profiles.push(id);
    }
    let (doc, loft) = insert(doc, lofted(&profiles, count(2)));
    (doc, loft, profiles)
}

// ---------------------------------------------------------------------
// §1: the vocabulary
// ---------------------------------------------------------------------

#[test]
fn the_new_slots_are_structural_counts() {
    for slot in [SlotId::VDegree, SlotId::Stations] {
        assert_eq!(slot.dimension(), Some(Dimension::Count), "{slot:?}");
        assert!(slot.is_structural(), "{slot:?}");
    }
}

#[test]
fn loft_inputs_are_its_profiles_in_order() {
    let (doc, loft, profiles) = loft_doc();
    let node = doc.node(loft).expect("the loft node");
    assert_eq!(doc.upstream(loft), profiles);
    assert_eq!(
        node.slots(),
        vec![
            SlotId::StationSlide {
                section: 1,
                step: 0
            },
            SlotId::StationSlide {
                section: 2,
                step: 0
            },
            SlotId::VDegree,
        ],
        "each later section's one slide, then the degree"
    );
    assert!(node.expr(SlotId::VDegree).is_some());
    assert!(node.expr(SlotId::Stations).is_none());
    assert!(node.named_nodes().is_empty());
}

#[test]
fn sweep_inputs_are_profile_then_path_and_it_carries_both_slots() {
    let mut doc = ProfileDoc::empty_derived("m5_pr10_nodes", Tol::witness());
    let (d, profile) = section(doc, 1.0);
    doc = d;
    let (d, path) = section(doc, 1.0);
    doc = d;
    let (doc, sweep) = insert(
        doc,
        Node::Sweep {
            profile: profile.into(),
            path: path.into(),
            stations: count(5),
            v_degree: count(3),
        },
    );
    let node = doc.node(sweep).expect("the sweep node");
    assert_eq!(doc.upstream(sweep), vec![profile, path]);
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
    let bogus = RecipeNodeId::new(0, 9999);
    let err = doc
        .apply(
            &DocEdit::InsertNode {
                node: Box::new(lofted(&[bogus], count(1))),
                fresh: Vec::new(),
            },
            Tol::witness(),
        )
        .expect_err("a dangling input must refuse");
    assert!(format!("{err:?}").contains("9999"), "{err:?}");
}

/// A wrong-dimension expression in a structural slot is refused by the
/// same door every other slot uses (spec D6).
#[test]
fn a_length_expression_in_the_v_degree_slot_refuses() {
    let (doc, _, profiles) = loft_doc();
    let err = doc
        .apply(
            &DocEdit::InsertNode {
                node: Box::new(lofted(&profiles, len(2.0))),
                fresh: Vec::new(),
            },
            Tol::witness(),
        )
        .expect_err("a Length in a Count slot must refuse");
    let text = format!("{err:?}");
    assert!(text.contains("Count") || text.contains("Length"), "{text}");
}

// ---------------------------------------------------------------------
// §4: the round trip
// ---------------------------------------------------------------------

#[test]
fn a_loft_document_round_trips_bit_identically() {
    let (doc, ..) = loft_doc();
    let text = save(&doc, &[], Tol::witness()).expect("saves");
    match load(&text, Tol::witness()) {
        Ok(loaded) => {
            assert!(loaded.snapshot.bit_eq(&doc), "loft snapshot drifted");
            // Save∘load is a fixpoint on the BYTES too.
            assert_eq!(
                save(&loaded.doc, &loaded.edits, Tol::witness()).expect("re-saves"),
                text
            );
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
fn a_sweep_document_round_trips_bit_identically() {
    let mut doc = ProfileDoc::empty_derived("m5_pr10_nodes", Tol::witness());
    let (d, profile) = section(doc, 1.0);
    doc = d;
    let (d, path) = section(doc, 2.0);
    doc = d;
    let (doc, _) = insert(
        doc,
        Node::Sweep {
            profile: profile.into(),
            path: path.into(),
            stations: count(4),
            v_degree: count(2),
        },
    );
    let text = save(&doc, &[], Tol::witness()).expect("saves");
    match load(&text, Tol::witness()) {
        Ok(loaded) => {
            assert!(loaded.snapshot.bit_eq(&doc), "sweep snapshot drifted");
            assert_eq!(
                save(&loaded.doc, &loaded.edits, Tol::witness()).expect("re-saves"),
                text
            );
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
    let out = evaluate::<f64>(
        doc,
        None,
        &CancelToken::new(),
        &EvalOptions::default(),
        Tol::witness(),
    );
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
    let out = evaluate::<f64>(
        &doc,
        None,
        &CancelToken::new(),
        &EvalOptions::default(),
        Tol::witness(),
    );
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
    let (d, a) = section(doc, 1.0);
    doc = d;
    // A triangle where the other section is a quad: no correspondence.
    let (d, b) = on_frame(
        doc,
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![(0.0, 0.0), (2.0, 0.0), (1.0, 1.0)]],
    );
    doc = d;
    let (doc, loft) = insert(doc, lofted(&[a, b], count(1)));
    with_failure(&doc, loft, |kind| match kind {
        NodeErrorKind::Skin(e) => {
            let msg = e.to_string();
            assert!(msg.contains("segments"), "{msg}");
        }
        other => panic!("expected a Skin refusal, got {other:?}"),
    });
}

/// A v-degree the section count cannot carry refuses typed, before any
/// surface is built, naming the slot's own domain `1 ..= sections − 1`.
/// The domain has an upper end on every target, so this row pins the
/// bounded sentence everywhere. The edit door refuses such a degree as
/// written; one read off a variable moved after the insert reaches
/// evaluation, which refuses it here.
#[test]
fn an_unusable_v_degree_refuses_typed() {
    let (doc, _, profiles) = loft_doc();
    let (doc, _) = fixture::step(
        doc,
        editor_core::DocEdit::DeclareVar {
            name: editor_core::VarName::from_static("skin"),
            def: editor_core::VarDecl::Free(editor_core::FreeVar::Count { value: 2 }),
        },
    );
    let skin = Formula::named(
        editor_core::VarName::from_static("skin"),
        editor_core::Dimension::Count,
    );
    let (doc, loft) = insert(doc, lofted(&profiles, skin));
    let var = doc.var_named("skin").expect("declared");
    let (doc, _) = fixture::step(
        doc,
        editor_core::DocEdit::SetVarValue {
            var: var.into(),
            value: editor_core::FreeValue::Count(3), // == the section count
        },
    );
    with_failure(&doc, loft, |kind| match kind {
        NodeErrorKind::CountOutOfRange {
            slot,
            count,
            admits,
        } => {
            assert_eq!(*slot, editor_core::SlotId::VDegree);
            assert_eq!(*count, 3);
            assert_eq!(*admits, 1..=2, "three sections admit degrees 1 to 2");
            assert_eq!(
                kind.to_string(),
                "the v degree input evaluated to 3, and it admits 1 to 2. Recourse: make it \
                 evaluate within that range"
            );
        }
        other => panic!("expected CountOutOfRange, got {other:?}"),
    });
}

/// An input that is not a profile refuses typed at the edit door: a
/// section reads a `Profile` variable.
#[test]
fn a_non_profile_input_refuses_typed() {
    let (doc, _, profiles) = loft_doc();
    let (doc, body) = insert(
        doc,
        Node::Extrude {
            profile: profiles[0].into(),
            distance: len(1.0),
        },
    );
    let refusal = fixture::insert_refused(&doc, lofted(&[profiles[0], body], count(1)));
    assert!(
        matches!(
            &refusal,
            editor_core::EditError::SlotVarKind {
                slot: editor_core::SlotId::Operand(editor_core::OperandSlot::Section(1)),
                found: editor_core::VarKind::Body,
                ..
            }
        ),
        "a section reads a profile: {refusal:?}"
    );
}

/// **F1**: the Sweep node's frontier is its OWN arm — since M6-3 the
/// ONLY `CurvedSolidFrontier` arm (the loft's retired with the body
/// assembly): no recipe-expressible path exists to sweep, and the
/// message says why rather than implying a single-segment case the
/// recipe layer cannot build.
#[test]
fn a_sweep_node_reaches_its_own_wider_frontier() {
    let mut doc = ProfileDoc::empty_derived("m5_pr10_nodes", Tol::witness());
    let (d, profile) = section(doc, 1.0);
    doc = d;
    let (d, path) = section(doc, 2.0);
    doc = d;
    let (doc, sweep) = insert(
        doc,
        Node::Sweep {
            profile: profile.into(),
            path: path.into(),
            stations: count(4),
            v_degree: count(2),
        },
    );
    with_failure(&doc, sweep, |kind| match kind {
        NodeErrorKind::CurvedSolidFrontier { what } => {
            assert!(what.contains("joined-path composition lane"), "{what}");
            assert!(
                what.contains(
                    "a closed chain of segments, or a full circle as one segment at one vertex"
                ),
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
    let (d, profile) = section(doc, 1.0);
    doc = d;
    let (d, path) = section(doc, 2.0);
    doc = d;
    let (doc, sweep) = insert(
        doc,
        Node::Sweep {
            profile: profile.into(),
            path: path.into(),
            stations: count(-3),
            v_degree: count(2),
        },
    );
    with_failure(&doc, sweep, |kind| match kind {
        NodeErrorKind::CountOutOfRange {
            slot,
            count,
            admits,
        } => {
            assert_eq!(
                *slot,
                editor_core::SlotId::Stations,
                "the slot read is named"
            );
            assert_eq!(*count, -3, "the evaluated count is carried");
            assert_eq!(*admits.start(), 2, "a sweep needs at least two stations");
            // The upper end is what a `usize` holds: none an `i64`
            // cannot reach on a 64-bit target, `u32::MAX` on wasm32.
            let range = match i64::try_from(usize::MAX) {
                Ok(most) => format!("2 to {most}"),
                Err(_) => "at least 2".to_owned(),
            };
            assert_eq!(
                kind.to_string(),
                format!(
                    "the stations input evaluated to -3, and it admits {range}. Recourse: make it \
                     evaluate within that range"
                ),
                "the sentence names the slot it read, the value and the slot's domain"
            );
        }
        other => panic!("expected CountOutOfRange, got {other:?}"),
    });
}
