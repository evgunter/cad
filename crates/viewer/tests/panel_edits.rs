//! **Event streams in, emitted edits out.**
//!
//! G1's testability rule made concrete for the document panels: a
//! synthetic stream of [`SessionOp`]s is replayed and the assertions
//! are on the `DocEdit`s that came out — one committed edit for a
//! property edit, a run of previews and exactly one commit for a
//! gesture, and a typed refusal where the ratified micro-decision says
//! there must be one.

// Panicking is a test's failure mechanism (workspace lint note).
#![allow(clippy::expect_used)]
#![allow(clippy::panic)]

mod common;

use pncad::document::{DocEdit, SlotId};
use pncad::geom_core::Tol;
use viewer::props::{SlotDriver, SlotValue};
use viewer::session::{DocSession, Refusal, Selection, SessionOp};
use viewer::{props, tree};

#[test]
fn a_property_edit_emits_exactly_one_committed_docedit() {
    let tol = Tol::witness();
    let (doc, _profile, _extrude) = common::parametric_plate(tol);
    let mut session = DocSession::inline(doc, tol);
    let before = session.history().len();

    let outcome = session.perform(SessionOp::SetParam {
        name: common::thickness_param(),
        value: SlotValue::Continuous(0.011),
    });
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    assert_eq!(outcome.committed.len(), 1);
    assert!(outcome.previewed.is_empty());
    // The VALUE door, not the create-or-replace one: the panel is
    // moving a number, so it emits the edit that moves a number and
    // leaves the declaration alone.
    assert!(matches!(
        outcome.committed.first(),
        Some(DocEdit::SetDocParamValue { .. })
    ));
    assert_eq!(session.history().len(), before + 1, "one undo step");
}

#[test]
fn a_literal_slot_edit_routes_through_setparam_and_lands_in_the_document() {
    let tol = Tol::witness();
    let (doc, profile) = {
        let doc: pncad::document::Doc<pncad::document::ProfileProgram> =
            pncad::document::Doc::empty_derived("gui3-literal", tol);
        common::inserted(&doc, common::square(0.04), tol)
    };
    let (doc, extrude) = common::inserted(
        &doc,
        pncad::document::Node::Extrude {
            profile,
            distance: common::len(0.008),
        },
        tol,
    );
    let mut session = DocSession::inline(doc, tol);
    session.perform(SessionOp::Select(Selection::Node(extrude)));

    let rows = session.slot_rows();
    let distance = rows
        .iter()
        .find(|row| row.slot == SlotId::Distance)
        .expect("an extrude carries a distance");
    assert_eq!(distance.driver, SlotDriver::Literal);
    assert!(!distance.structural);
    assert_eq!(distance.value, Ok(SlotValue::Continuous(0.008)));

    let outcome = session.perform(SessionOp::SetSlot {
        node: extrude,
        slot: SlotId::Distance,
        value: SlotValue::Continuous(0.012),
    });
    assert!(matches!(
        outcome.committed.first(),
        Some(DocEdit::SetParam {
            slot: SlotId::Distance,
            ..
        })
    ));
    assert_eq!(
        props::slot_rows(session.committed_doc(), extrude)
            .into_iter()
            .find(|row| row.slot == SlotId::Distance)
            .expect("still there")
            .value,
        Ok(SlotValue::Continuous(0.012))
    );
}

#[test]
fn an_expression_driven_dimension_refuses_with_the_affordance() {
    let tol = Tol::witness();
    let (doc, _profile, extrude) = common::parametric_plate(tol);
    let mut session = DocSession::inline(doc, tol);
    session.perform(SessionOp::Select(Selection::Node(extrude)));

    let rows = session.slot_rows();
    let distance = rows
        .iter()
        .find(|row| row.slot == SlotId::Distance)
        .expect("an extrude carries a distance");
    assert!(distance.driver.is_driven());
    assert_eq!(
        distance.value,
        Ok(SlotValue::Continuous(0.004)),
        "thickness / 2, evaluated under the document's parameters"
    );

    let before = session.history().len();
    let outcome = session.perform(SessionOp::SetSlot {
        node: extrude,
        slot: SlotId::Distance,
        value: SlotValue::Continuous(0.02),
    });
    assert!(outcome.committed.is_empty(), "a refusal commits nothing");
    assert_eq!(session.history().len(), before, "and mints no history");
    match outcome.refusal {
        Some(Refusal::DrivenByExpression {
            node,
            slot,
            params,
            current,
        }) => {
            assert_eq!(node, extrude);
            assert_eq!(slot, SlotId::Distance);
            assert_eq!(
                params,
                vec![common::thickness_param()],
                "the affordance names what to edit instead"
            );
            assert_eq!(current, Some(SlotValue::Continuous(0.004)));
        }
        other => panic!("expected the driven refusal, got {other:?}"),
    }
}

#[test]
fn the_affordance_navigates_to_the_driving_parameter_and_the_edit_lands_there() {
    let tol = Tol::witness();
    let (doc, _profile, extrude) = common::parametric_plate(tol);
    let mut session = DocSession::inline(doc, tol);
    session.perform(SessionOp::Select(Selection::Node(extrude)));
    let Some(Refusal::DrivenByExpression { params, .. }) = session
        .perform(SessionOp::SetSlot {
            node: extrude,
            slot: SlotId::Distance,
            value: SlotValue::Continuous(0.02),
        })
        .refusal
    else {
        panic!("expected the driven refusal");
    };
    let name = params.first().expect("one driving parameter").clone();

    // The affordance's navigate half: selecting the parameter is a
    // typed operation, and editing it there moves the slot the direct
    // edit refused to touch.
    session.perform(SessionOp::Select(Selection::Param(name.clone())));
    assert_eq!(session.selection(), &Selection::Param(name.clone()));
    let outcome = session.perform(SessionOp::SetParam {
        name,
        value: SlotValue::Continuous(0.020),
    });
    assert_eq!(outcome.committed.len(), 1);
    assert_eq!(
        props::slot_rows(session.committed_doc(), extrude)
            .into_iter()
            .find(|row| row.slot == SlotId::Distance)
            .expect("still there")
            .value,
        Ok(SlotValue::Continuous(0.010)),
        "the driven slot followed its parameter"
    );
}

#[test]
fn a_driven_slot_still_accepts_an_expression_through_the_text_door() {
    let tol = Tol::witness();
    let (doc, _profile, extrude) = common::parametric_plate(tol);
    let mut session = DocSession::inline(doc, tol);
    let outcome = session.perform(SessionOp::SetSlotExpression {
        node: extrude,
        slot: SlotId::Distance,
        text: "thickness * 2.0".to_owned(),
    });
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    assert_eq!(outcome.committed.len(), 1);
    assert_eq!(
        props::slot_rows(session.committed_doc(), extrude)
            .into_iter()
            .find(|row| row.slot == SlotId::Distance)
            .expect("still there")
            .value,
        Ok(SlotValue::Continuous(0.016))
    );

    // And unparseable text refuses typed, committing nothing.
    let before = session.history().len();
    let outcome = session.perform(SessionOp::SetSlotExpression {
        node: extrude,
        slot: SlotId::Distance,
        text: "thickness *".to_owned(),
    });
    assert!(matches!(outcome.refusal, Some(Refusal::Parse(_))));
    assert_eq!(session.history().len(), before);
}

#[test]
fn a_gesture_previews_against_scratch_state_and_commits_exactly_once() {
    let tol = Tol::witness();
    let (doc, profile) = {
        let doc: pncad::document::Doc<pncad::document::ProfileProgram> =
            pncad::document::Doc::empty_derived("gui3-gesture", tol);
        common::inserted(&doc, common::square(0.04), tol)
    };
    let (doc, extrude) = common::inserted(
        &doc,
        pncad::document::Node::Extrude {
            profile,
            distance: common::len(0.008),
        },
        tol,
    );
    let mut session = DocSession::inline(doc, tol);
    let before = session.history().len();

    assert!(
        session
            .perform(SessionOp::BeginGesture {
                node: extrude,
                slot: SlotId::Distance,
            })
            .refusal
            .is_none()
    );
    let mut previews = 0usize;
    for step in 1..=4 {
        let outcome = session.perform(SessionOp::PreviewGesture {
            value: 0.008 + f64::from(step) * 0.001,
        });
        assert!(outcome.committed.is_empty(), "a preview commits nothing");
        previews += outcome.previewed.len();
        assert_eq!(
            session.history().len(),
            before,
            "the history is untouched mid-gesture"
        );
    }
    assert_eq!(previews, 4);
    // Mid-gesture the panels show the scratch document, and the
    // committed one still says the starting value.
    assert_eq!(
        props::slot_rows(session.doc(), extrude)
            .into_iter()
            .find(|row| row.slot == SlotId::Distance)
            .expect("still there")
            .value,
        Ok(SlotValue::Continuous(0.012))
    );
    assert_eq!(
        props::slot_rows(session.committed_doc(), extrude)
            .into_iter()
            .find(|row| row.slot == SlotId::Distance)
            .expect("still there")
            .value,
        Ok(SlotValue::Continuous(0.008))
    );

    let outcome = session.perform(SessionOp::CommitGesture);
    assert_eq!(outcome.committed.len(), 1, "one edit for the whole drag");
    assert_eq!(session.history().len(), before + 1, "one undo step");

    // And one undo returns the whole gesture.
    session.perform(SessionOp::Undo);
    assert_eq!(
        props::slot_rows(session.committed_doc(), extrude)
            .into_iter()
            .find(|row| row.slot == SlotId::Distance)
            .expect("still there")
            .value,
        Ok(SlotValue::Continuous(0.008))
    );
}

/// **A dragged document PARAMETER is a gesture too.**
///
/// The affordance's "edit the parameter" link lands a user on this
/// widget, so it is a primary path — and it used to commit one edit,
/// one undo step and one re-evaluation per frame of a drag, where G1
/// ratifies exactly one commit on release. The rule is the slot rule
/// and this row is the slot row's twin.
#[test]
fn a_parameter_drag_previews_and_commits_exactly_once() {
    let tol = Tol::witness();
    let (doc, _profile, extrude) = common::parametric_plate(tol);
    let mut session = DocSession::inline(doc, tol);
    let before = session.history().len();
    let name = common::thickness_param();

    assert!(
        session
            .perform(SessionOp::BeginParamGesture { name: name.clone() })
            .refusal
            .is_none()
    );
    let mut previews = 0usize;
    let mut last = 0.0;
    for step in 1..=5 {
        last = 0.008 + f64::from(step) * 0.002;
        let outcome = session.perform(SessionOp::PreviewGesture { value: last });
        assert!(outcome.committed.is_empty(), "a preview commits nothing");
        previews += outcome.previewed.len();
        assert_eq!(session.history().len(), before, "and mints no history");
    }
    assert_eq!(previews, 5);

    let outcome = session.perform(SessionOp::CommitGesture);
    assert_eq!(outcome.committed.len(), 1, "one edit for the whole drag");
    assert!(matches!(
        outcome.committed.first(),
        Some(DocEdit::SetDocParamValue { .. })
    ));
    assert_eq!(session.history().len(), before + 1, "one undo step");
    // The last previewed value is the one recorded, and the driven
    // slot downstream followed it.
    assert_eq!(
        props::slot_rows(session.committed_doc(), extrude)
            .into_iter()
            .find(|row| row.slot == SlotId::Distance)
            .expect("still there")
            .value,
        Ok(SlotValue::Continuous(last / 2.0)),
        "the LAST previewed value is what the commit recorded"
    );

    session.perform(SessionOp::Undo);
    assert_eq!(session.history().len(), before + 1, "undo destroys nothing");
    assert_eq!(
        props::slot_rows(session.committed_doc(), extrude)
            .into_iter()
            .find(|row| row.slot == SlotId::Distance)
            .expect("still there")
            .value,
        Ok(SlotValue::Continuous(0.004)),
        "one undo returns the whole gesture"
    );
}

#[test]
fn a_gesture_on_an_absent_parameter_refuses_typed() {
    let tol = Tol::witness();
    let (doc, _profile, _extrude) = common::parametric_plate(tol);
    let mut session = DocSession::inline(doc, tol);
    let outcome = session.perform(SessionOp::BeginParamGesture {
        name: pncad::document::ParamName::new("no-such-parameter"),
    });
    assert!(matches!(outcome.refusal, Some(Refusal::NoSuchParam(_))));
    assert!(matches!(
        session
            .perform(SessionOp::PreviewGesture { value: 1.0 })
            .refusal,
        Some(Refusal::NoGesture)
    ));
}

/// **A frame's refusals have a precedence, and the affordance wins.**
///
/// Dragging an expression-driven slot queues `BeginGesture` (refused
/// with the ratified affordance) and `PreviewGesture` (refused
/// `NoGesture`, purely BECAUSE the first refusal stopped the gesture
/// from opening) in one frame. A chrome that keeps the last refusal
/// shows the bookkeeping one and buries the decision. This row replays
/// that exact batch and asserts on what a frame would display.
#[test]
fn the_affordance_outranks_the_bookkeeping_refusal_it_causes() {
    let tol = Tol::witness();
    let (doc, _profile, extrude) = common::parametric_plate(tol);
    let mut session = DocSession::inline(doc, tol);

    let batch = vec![
        SessionOp::BeginGesture {
            node: extrude,
            slot: SlotId::Distance,
        },
        SessionOp::PreviewGesture { value: 0.02 },
    ];
    let mut shown: Option<Refusal> = None;
    for op in batch {
        if let Some(next) = session.perform(op).refusal {
            shown = Refusal::preferred(shown, next);
        }
    }
    let shown = shown.expect("the batch refused");
    assert!(
        matches!(shown, Refusal::DrivenByExpression { .. }),
        "expected the affordance, got {shown:?}"
    );
    assert!(
        shown.to_string().contains("edit the expression?"),
        "and it renders the ratified wording: {shown}"
    );
    assert!(shown.rank() < Refusal::NoGesture.rank());
}

/// Every refusal renders through `Display`, not through a debug dump.
#[test]
fn refusals_render_as_sentences() {
    let tol = Tol::witness();
    let (doc, _profile, _extrude) = common::parametric_plate(tol);
    let mut session = DocSession::inline(doc, tol);
    let refusal = session
        .perform(SessionOp::Open(
            std::env::temp_dir().join("gui3-no-such-document.pncad"),
        ))
        .refusal
        .expect("a missing file refuses");
    let rendered = refusal.to_string();
    assert!(
        rendered.contains("cannot read the file"),
        "the io arm names what happened: {rendered}"
    );
    assert!(
        !rendered.contains('{') && !rendered.contains('"'),
        "and it is a sentence, not a debug dump: {rendered}"
    );
}

#[test]
fn an_abandoned_gesture_leaves_no_trace() {
    let tol = Tol::witness();
    let (doc, profile) = {
        let doc: pncad::document::Doc<pncad::document::ProfileProgram> =
            pncad::document::Doc::empty_derived("gui3-abandon", tol);
        common::inserted(&doc, common::square(0.04), tol)
    };
    let (doc, extrude) = common::inserted(
        &doc,
        pncad::document::Node::Extrude {
            profile,
            distance: common::len(0.008),
        },
        tol,
    );
    let mut session = DocSession::inline(doc, tol);
    let before = session.history().len();
    session.perform(SessionOp::BeginGesture {
        node: extrude,
        slot: SlotId::Distance,
    });
    session.perform(SessionOp::PreviewGesture { value: 0.03 });
    session.perform(SessionOp::CancelGesture);
    assert_eq!(session.history().len(), before);
    assert_eq!(
        props::slot_rows(session.doc(), extrude)
            .into_iter()
            .find(|row| row.slot == SlotId::Distance)
            .expect("still there")
            .value,
        Ok(SlotValue::Continuous(0.008)),
        "the panels are back on the committed document"
    );
}

#[test]
fn a_gesture_over_a_driven_slot_is_refused_before_it_starts() {
    let tol = Tol::witness();
    let (doc, _profile, extrude) = common::parametric_plate(tol);
    let mut session = DocSession::inline(doc, tol);
    let outcome = session.perform(SessionOp::BeginGesture {
        node: extrude,
        slot: SlotId::Distance,
    });
    assert!(matches!(
        outcome.refusal,
        Some(Refusal::DrivenByExpression { .. })
    ));
    assert!(matches!(
        session
            .perform(SessionOp::PreviewGesture { value: 1.0 })
            .refusal,
        Some(Refusal::NoGesture)
    ));
}

#[test]
fn the_tree_selects_a_node_and_the_property_panel_follows() {
    let tol = Tol::witness();
    let (doc, profile, extrude) = common::parametric_plate(tol);
    let mut session = DocSession::inline(doc, tol);
    session.pump();

    let rows = session.tree_rows();
    let ids: Vec<_> = rows.iter().map(|row| row.id).collect();
    assert!(ids.contains(&profile) && ids.contains(&extrude));
    assert_eq!(
        rows.iter()
            .find(|row| row.id == extrude)
            .map(|row| row.depth),
        Some(0),
        "the profile is the extrude's primary input, so the extrude \
         continues its line rather than indenting under it"
    );
    assert!(!tree::has_faults(&rows));

    assert!(session.slot_rows().is_empty(), "nothing selected yet");
    session.perform(SessionOp::Select(Selection::Node(extrude)));
    assert_eq!(session.slot_rows().len(), 1);
    session.perform(SessionOp::Select(Selection::Node(profile)));
    assert!(
        session
            .slot_rows()
            .iter()
            .all(|row| matches!(row.slot, SlotId::Profile { .. })),
        "a profile's slots are its program's"
    );
}

/// **The create affordance's whole arc**: create → the parameter
/// exists → an expression referencing it now parses → one undo
/// removes it.
#[test]
fn create_parameter_reference_it_and_one_undo_removes_it() {
    let tol = Tol::witness();
    let (doc, _profile, extrude) = common::parametric_plate(tol);
    let mut session = DocSession::inline(doc, tol);
    let margin = pncad::document::ParamName::new("margin");

    // Before: an expression naming the undeclared parameter refuses
    // typed at the parse door (deliberate typo-safety) and carries
    // the NAME — the payload the chrome's offer prefills from.
    let before = session.history().len();
    let outcome = session.perform(SessionOp::SetSlotExpression {
        node: extrude,
        slot: SlotId::Distance,
        text: "margin * 2.0".to_owned(),
    });
    match outcome.refusal {
        Some(Refusal::Parse(ref error)) => match error.as_ref() {
            pncad::document::ParseError::UnknownParam { name, .. } => {
                assert_eq!(name, "margin", "the refusal names the unknown");
            }
            other => panic!("expected the unknown-param refusal, got {other:?}"),
        },
        ref other => panic!("expected a parse refusal, got {other:?}"),
    }
    assert!(outcome.committed.is_empty(), "a refusal commits nothing");
    assert_eq!(session.history().len(), before, "and mints no history");

    // Create: exactly one committed SetDocParam — the CREATE door
    // really is authoring a declaration — and one undo step.
    let outcome = session.perform(SessionOp::CreateParam {
        name: margin.clone(),
        value: pncad::document::DocParam::continuous(pncad::document::Dimension::Length, 0.005),
    });
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    assert_eq!(outcome.committed.len(), 1);
    assert!(matches!(
        outcome.committed.first(),
        Some(DocEdit::SetDocParam { .. })
    ));
    assert_eq!(session.history().len(), before + 1, "one undo step");
    let row = props::param_rows(session.committed_doc())
        .into_iter()
        .find(|row| row.name == margin)
        .expect("the parameter exists");
    assert_eq!(row.dimension, pncad::document::Dimension::Length);
    assert_eq!(row.value, SlotValue::Continuous(0.005));

    // The same expression now parses, commits, and evaluates.
    let outcome = session.perform(SessionOp::SetSlotExpression {
        node: extrude,
        slot: SlotId::Distance,
        text: "margin * 2.0".to_owned(),
    });
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    assert_eq!(outcome.committed.len(), 1);
    assert_eq!(
        props::slot_rows(session.committed_doc(), extrude)
            .into_iter()
            .find(|row| row.slot == SlotId::Distance)
            .expect("still there")
            .value,
        Ok(SlotValue::Continuous(0.010))
    );

    // Undo the expression edit, then ONE undo removes the parameter.
    session.perform(SessionOp::Undo);
    assert!(
        props::param_rows(session.committed_doc())
            .into_iter()
            .any(|row| row.name == margin),
        "the first undo returns only the expression edit"
    );
    session.perform(SessionOp::Undo);
    assert!(
        !props::param_rows(session.committed_doc())
            .into_iter()
            .any(|row| row.name == margin),
        "one more undo removes the creation"
    );
}

/// **Create is not replace.** `DocEdit::SetDocParam` is
/// create-or-replace at the API; the panel's create door refuses an
/// already-declared name typed, with the existing declaration's
/// dimension in the payload — and the replace act stays spellable
/// through the door that says so (`SetParam`).
#[test]
fn the_create_door_refuses_an_existing_name_and_setparam_still_replaces() {
    let tol = Tol::witness();
    let (doc, _profile, _extrude) = common::parametric_plate(tol);
    let mut session = DocSession::inline(doc, tol);
    let before = session.history().len();

    // Creating over "thickness" — even at a DIFFERENT dimension, the
    // riskier half of a silent replace — refuses typed and unchanged.
    let outcome = session.perform(SessionOp::CreateParam {
        name: common::thickness_param(),
        value: pncad::document::DocParam::Count { value: 3 },
    });
    match outcome.refusal {
        Some(Refusal::ParamExists {
            ref name,
            dimension,
        }) => {
            assert_eq!(name, &common::thickness_param());
            assert_eq!(
                dimension,
                pncad::document::Dimension::Length,
                "the payload carries what already stands there"
            );
        }
        ref other => panic!("expected the already-exists refusal, got {other:?}"),
    }
    assert!(outcome.committed.is_empty(), "a refusal commits nothing");
    assert_eq!(session.history().len(), before, "and mints no history");
    let rendered = outcome.refusal.expect("asserted above").to_string();
    assert!(
        rendered.contains("already exists") && rendered.contains("edit it instead?"),
        "the refusal offers the edit door: {rendered}"
    );

    // The REPLACE door still replaces — same underlying edit, spelled
    // as what it is.
    let outcome = session.perform(SessionOp::SetParam {
        name: common::thickness_param(),
        value: SlotValue::Continuous(0.012),
    });
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    assert_eq!(outcome.committed.len(), 1);
}
