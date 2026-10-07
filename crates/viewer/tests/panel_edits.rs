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

use crate::common;
use editor_core::ExtrudeSide;

use pncad::document::{Dimension, Distribution, DocEdit, EditError, FreeVar, SlotId, VarName};
use pncad::geom_core::Tol;
use pncad::prelude::MM;
use pncad::quantity::WrittenLength;
use viewer::props::{SlotDriver, SlotValue};
use viewer::session::{DocSession, Refusal, Selection, SessionOp};
use viewer::{props, tree};

#[test]
fn a_property_edit_emits_exactly_one_committed_docedit() {
    let tol = Tol::witness();
    let (doc, _profile, _extrude) = common::parametric_plate(tol);
    let mut session = DocSession::inline(doc, tol);
    let before = session.history().len();

    let outcome = session.perform(SessionOp::SetVariable {
        var: common::thickness_var(session.committed_doc()),
        value: SlotValue::Continuous(0.011),
    });
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    assert_eq!(outcome.committed.len(), 1);
    assert!(outcome.previewed.is_empty());
    // The VALUE door, not the whole-definition one: the panel is
    // moving a number, so it emits the edit that moves a number and
    // leaves the declaration alone.
    assert!(matches!(
        outcome.committed.first(),
        Some(DocEdit::SetVarValue { .. })
    ));
    assert_eq!(session.history().len(), before + 1, "one undo step");
}

#[test]
fn a_literal_slot_edit_routes_through_setparam_and_lands_in_the_document() {
    let tol = Tol::witness();
    let (doc, profile) = {
        let doc: pncad::document::Doc<pncad::document::ProfileProgram> =
            pncad::document::Doc::empty_derived("gui3-literal", tol);
        common::framed_square(&doc, 0.04, tol)
    };
    let (doc, extrude) = common::inserted(
        &doc,
        pncad::document::Node::Extrude {
            profile,
            distance: common::len(0.008),
            side: ExtrudeSide::Along,
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

    let before = session
        .committed_doc()
        .slot(extrude, SlotId::Distance)
        .expect("the extrude reads its distance");
    let outcome = session.perform(SessionOp::SetSlot {
        node: extrude,
        slot: SlotId::Distance,
        value: SlotValue::Continuous(0.012),
    });
    // A value TYPED at a slot mints an anonymous variable (VR6, D10),
    // through `SetParam`: the slot reads a new variable, and the one it
    // read before, read by nothing now, leaves the document (VR7).
    assert!(
        matches!(outcome.committed.as_slice(), [DocEdit::SetParam { .. }]),
        "{:?}",
        outcome.committed
    );
    let doc = session.committed_doc();
    let held = doc
        .slot(extrude, SlotId::Distance)
        .expect("the extrude reads its distance");
    assert_ne!(held, before, "the typed value is a new variable");
    assert!(doc.is_typed_value(held), "an anonymous free variable");
    assert!(doc.var(before).is_none(), "the unread one is gone");
    assert_eq!(
        props::slot_rows(doc, extrude)
            .into_iter()
            .find(|row| row.slot == SlotId::Distance)
            .expect("still there")
            .value,
        Ok(SlotValue::Continuous(0.012))
    );
}

/// One profile, one extrude with a LITERAL distance, and a pattern
/// over it whose count is a literal too — a continuous slot and a
/// structural `Count` slot in one document, so both directions of the
/// Count/continuous divide have a subject.
fn literal_and_pattern_doc(
    tol: Tol,
) -> (
    pncad::document::Doc<pncad::document::ProfileProgram>,
    pncad::document::RecipeNodeId,
    pncad::document::RecipeNodeId,
) {
    let doc: pncad::document::Doc<pncad::document::ProfileProgram> =
        pncad::document::Doc::empty_derived("gui3-literal-range", tol);
    let (doc, profile) = common::framed_square(&doc, 0.04, tol);
    let (doc, extrude) = common::inserted(
        &doc,
        pncad::document::Node::Extrude {
            profile,
            distance: common::len(0.008),
            side: ExtrudeSide::Along,
        },
        tol,
    );
    let (doc, pattern) = common::inserted(
        &doc,
        pncad::document::Node::Pattern {
            input: extrude,
            count: pncad::document::Formula::count(3),
            kind: pncad::document::PatternKind::Linear {
                direction: [common::scl(1.0), common::scl(0.0), common::scl(0.0)],
                spacing: common::len(0.03),
            },
        },
        tol,
    );
    (doc, extrude, pattern)
}

/// `text` with the variable id the first `"key":` holds replaced by
/// `id`: a slot on the wire is its variable's id, so this re-points one
/// slot at another variable — the one corruption a hand edit can make
/// of it.
fn repointed_slot(text: &str, key: &str, id: u64) -> String {
    let at = text
        .find(&format!("\"{key}\": "))
        .expect("the wire carries that key")
        + key.len()
        + 4;
    let digits = text[at..].bytes().take_while(u8::is_ascii_digit).count();
    assert!(digits > 0, "a stored slot holds its variable's id");
    let out = format!("{}{id}{}", &text[..at], &text[at + digits..]);
    assert_ne!(out, text, "the corruption really landed");
    out
}

/// **A slot the document holds a bare literal for always has a value**
/// — which is why the range button beside it is gated on the driver
/// alone, with no second conjunct on the value.
///
/// `props::slot_row` evaluates each slot with the branch
/// `SlotId::dimension` picks, so the only way a leaf carrying no
/// variable reference fails to evaluate is a Count/continuous
/// disagreement between the slot and its expression —
/// `CountExprInContinuousEval` one way, `ContinuousExprInCountEval`
/// the other. One predicate answers that disagreement for every door,
/// `Node::slot_dimension_fault` over `Node::slots()`.
///
/// This row reads the rows; the two below hold the doors, **enumerated
/// by the modality a document arrives through** rather than by code
/// path, because a claim about every document is only as good as its
/// list of ways in:
///
/// * **an edit** — both directions, `DocEdit::SetParam` and
///   `DocEdit::SetStructuralParam`, which is what
///   `SessionOp::SetSlot` and `SetSlotExpression` reach;
/// * **a file** — `pncad::document::load`, the door the viewer opens
///   every document through, over bytes a hand edit or another tool
///   wrote;
/// * **a hand-built `Node`** — refused when it is inserted, because
///   insertion is an edit: there is no door that puts a `Node` into a
///   `Doc` without `apply`.
///
/// The one way a row reaches the panel with an `Err` value and no
/// `EvalError` at all is `SlotFault::NoExpression`, and it closes the
/// other way: `props::slot_row` reports that row as DRIVEN with an
/// empty variable list, so the button refuses it as a driven slot
/// rather than offering it.
#[test]
fn a_literal_slot_always_has_a_value_because_every_door_fixes_its_dimension() {
    let tol = Tol::witness();
    let (doc, extrude, pattern) = literal_and_pattern_doc(tol);

    for node in [extrude, pattern] {
        let rows = props::slot_rows(&doc, node);
        assert!(
            rows.iter().any(|row| row.driver == SlotDriver::Literal),
            "node {} is about literal rows",
            node.0
        );
        for row in &rows {
            if row.driver == SlotDriver::Literal {
                assert!(
                    row.value.is_ok(),
                    "{} is a literal with no value: {:?}",
                    row.slot.label(),
                    row.value
                );
            }
        }
    }
    // The Count row really is one of them — otherwise the loop above
    // says nothing about the second direction of the divide.
    assert!(
        props::slot_rows(&doc, pattern)
            .iter()
            .any(|row| row.slot == SlotId::Count && row.driver == SlotDriver::Literal),
        "the pattern's count is a literal row"
    );
}

/// **The edit modality, both directions of the divide.** Each
/// expression below is the one whose row WOULD carry the matching
/// `EvalError`, refused rather than stored.
#[test]
fn the_edit_doors_refuse_both_directions_of_the_count_divide() {
    let tol = Tol::witness();
    let (doc, extrude, pattern) = literal_and_pattern_doc(tol);

    let count_into_continuous = pncad::document::apply(
        &doc,
        &DocEdit::SetParam {
            node: extrude,
            slot: SlotId::Distance,
            expr: pncad::document::Formula::count(3),
            fresh: Vec::new(),
        },
        tol,
        &pncad::document::RefusingReach,
    );
    match count_into_continuous {
        Err(pncad::document::EditError::SlotDimensionMismatch {
            slot,
            expected,
            found,
        }) => {
            // The payload, named rather than compared with another
            // reading of itself: this is the row that says WHICH
            // dimensions the door reported.
            assert_eq!(slot, SlotId::Distance);
            assert_eq!(expected, Dimension::Length);
            assert_eq!(found, Dimension::Count);
        }
        other => panic!("a Count literal in a Length slot must be refused, got {other:?}"),
    }

    let continuous_into_count = pncad::document::apply(
        &doc,
        &DocEdit::SetStructuralParam {
            node: pattern,
            slot: SlotId::Count,
            expr: common::len(0.03),
            fresh: Vec::new(),
        },
        tol,
        &pncad::document::RefusingReach,
    );
    match continuous_into_count {
        Err(pncad::document::EditError::SlotDimensionMismatch {
            slot,
            expected,
            found,
        }) => {
            assert_eq!(slot, SlotId::Count);
            assert_eq!(expected, Dimension::Count);
            assert_eq!(found, Dimension::Length);
        }
        other => panic!("a Length literal in a Count slot must be refused, got {other:?}"),
    }
}

/// **The file modality** — the door the viewer opens every document
/// through, over bytes no edit door wrote.
///
/// A hand edit or a foreign tool is the only way a `Count` variable can
/// be read at a `Length` slot, and it is the input the claim above
/// most needs: everything else in this suite reaches the document
/// through `apply`. The saved fixture is doctored in ONE field — the
/// extrude's distance re-pointed at the pattern count's variable — and
/// `load` is asked what it thinks.
#[test]
fn the_load_door_refuses_a_count_literal_in_a_continuous_slot() {
    let tol = Tol::witness();
    let (doc, extrude, pattern) = literal_and_pattern_doc(tol);
    let text = pncad::document::save(&doc, &[], tol).expect("the fixture saves");
    pncad::document::load(&text, tol).expect("and loads back as it was written");

    let count = doc
        .slot(pattern, SlotId::Count)
        .expect("a pattern reads its count");
    let corrupt = repointed_slot(&text, "distance", count.0);
    match pncad::document::load(&corrupt, tol) {
        Err(pncad::document::PersistError::Snapshot(
            pncad::document::SnapshotError::SlotVarKind {
                node,
                slot,
                declared,
                referenced,
                ..
            },
        )) => {
            assert_eq!(node.id(), extrude);
            assert_eq!(slot, SlotId::Distance);
            assert_eq!(declared, Dimension::Count);
            assert_eq!(referenced, Dimension::Length);
        }
        other => panic!("the load door must refuse a Count distance, got {other:?}"),
    }
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
        "thickness / 2, evaluated under the document's variables"
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
            variables,
            current,
            ..
        }) => {
            assert_eq!(node, extrude);
            assert_eq!(slot, SlotId::Distance);
            assert_eq!(
                variables,
                vec![
                    session
                        .committed_doc()
                        .spoken_var(common::thickness_var(session.committed_doc()))
                ],
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
    let Some(Refusal::DrivenByExpression { variables, .. }) = session
        .perform(SessionOp::SetSlot {
            node: extrude,
            slot: SlotId::Distance,
            value: SlotValue::Continuous(0.02),
        })
        .refusal
    else {
        panic!("expected the driven refusal");
    };
    let var = variables.first().expect("one driving variable").id();

    // The affordance's navigate half: selecting the variable is a
    // typed operation, and editing it there moves the slot the direct
    // edit refused to touch.
    session.perform(SessionOp::Select(Selection::Variable(var)));
    assert_eq!(session.selection(), &Selection::Variable(var));
    let outcome = session.perform(SessionOp::SetVariable {
        var,
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
        "the driven slot followed its variable"
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
        common::framed_square(&doc, 0.04, tol)
    };
    let (doc, extrude) = common::inserted(
        &doc,
        pncad::document::Node::Extrude {
            profile,
            distance: common::len(0.008),
            side: ExtrudeSide::Along,
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
            node: extrude,
            slot: SlotId::Distance,
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

    let outcome = session.perform(SessionOp::CommitGesture {
        node: extrude,
        slot: SlotId::Distance,
    });
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

/// **A dragged document VARIABLE is a gesture too.**
///
/// The affordance's "edit the variable" link lands a user on this
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
    let var = common::thickness_var(session.committed_doc());

    assert!(
        session
            .perform(SessionOp::BeginVariableGesture { var })
            .refusal
            .is_none()
    );
    let mut previews = 0usize;
    let mut last = 0.0;
    for step in 1..=5 {
        last = 0.008 + f64::from(step) * 0.002;
        let outcome = session.perform(SessionOp::PreviewVariableGesture { var, value: last });
        assert!(outcome.committed.is_empty(), "a preview commits nothing");
        previews += outcome.previewed.len();
        assert_eq!(session.history().len(), before, "and mints no history");
    }
    assert_eq!(previews, 5);

    let outcome = session.perform(SessionOp::CommitVariableGesture { var });
    assert_eq!(outcome.committed.len(), 1, "one edit for the whole drag");
    assert!(matches!(
        outcome.committed.first(),
        Some(DocEdit::SetVarValue { .. })
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
    let outcome = session.perform(SessionOp::BeginVariableGesture {
        var: pncad::document::VarId(0x6e6f_7375_6368),
    });
    assert!(matches!(outcome.refusal, Some(Refusal::NoSuchVariable(_))));
    assert!(matches!(
        session
            .perform(SessionOp::PreviewVariableGesture {
                var: pncad::document::VarId(0x6e6f_7375_6368),
                value: 1.0
            })
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
        SessionOp::PreviewGesture {
            node: extrude,
            slot: SlotId::Distance,
            value: 0.02,
        },
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

test_utils::f6_variants! {
    /// Every `Refusal` arm's identifier, as the ban list the six
    /// sampled renderings are held to.
    ///
    /// **The roster is the enum's, not the sample's.** A rendering that
    /// leaks a SIBLING arm's identifier is as much a dump as one that
    /// leaks its own, and a per-arm check cannot see it.
    const REFUSAL: Refusal = [
        DrivenByExpression,
        NoSuchSlot,
        NoSuchVariable,
        VariableIsDefined,
        NotOffered,
        ConstantRefused,
        EmptyName,
        WrongNodeKind,
        Duplicate,
        Contact,
        Edit,
        Dimension,
        Parse,
        NoGesture,
        GestureInFlight,
        WrongGesture,
        Io,
        NothingToDo,
        Display,
        SlotUnit,
        NoDocumentDirectory,
        Workspace,
        SelfInstance,
        ProfileEditStale,
    ];
}

/// The `Debug` punctuation that would be a dump in a `Refusal`
/// sentence: the two field names the payloads carry, and the quotation
/// mark a `{:?}` over a `String` or a `VarName` leaves behind.
///
/// `{` is [`test_utils::f6::assert_f6`]'s own and is banned whatever
/// this list says; the quotation mark is this row's extra clause, and
/// the doc comment below says why it is asserted of these six arms
/// rather than of the vocabulary.
const REFUSAL_FIELDS: &[&str] = &["node:", "name:", "\""];

/// **Six refusals a panel can provoke render as sentences** — six,
/// named, and not a claim about the vocabulary. Each is a real op
/// through a real door, so the rendering asserted is the one a person
/// reads.
///
/// **The universal is not asserted here, because a sample cannot hold
/// it.** One of `Refusal`'s arms is `Edit`, which forwards a whole
/// second vocabulary, so what decides the rendering is the payload's
/// variant one level down — `crates/pncad-py/src/
/// prose_census.rs` states exactly that failure mode, and a roster
/// that picks its own samples excludes the failing mode by
/// construction. The two vocabulary-wide halves live elsewhere, and
/// are named here so this row is not read as holding them:
///
/// * **that every arm renders at all** is the compiler's
///   (`crates/viewer/README.md`, *A policy over an enum names every
///   variant*), which is why there is no `Refusal::ALL` to walk.
/// * **that no rendering carries the field-brace fingerprint** is
///   `prose_census`'s, a census over SITES rather than samples.
///
/// The shape asserted below is F6's, through the one door that holds
/// it ([`test_utils::f6::assert_f6`]): no brace, no `Debug` field
/// punctuation, no variant identifier, and never simply the dump —
/// **plus a quotation mark**, which F6 does not list and this row
/// asserts anyway, passed as one more banned token.
///
/// **The identifier ban is the ENUM's roster, not each arm's own**
/// ([`REFUSAL`]). A rendering that leaks a sibling arm's identifier is
/// a dump as surely as one that leaks its own, and the per-arm check
/// this row used to spell could not see it.
///
/// That extra clause is the one that catches the case this row exists
/// for. A `{:?}` over a `String` or a `VarName` renders `"width"`:
/// no brace, no field punctuation, and the identifier it leaks is the
/// PAYLOAD's rather than the arm's, so every F6 clause passes over it
/// and so does `assert_ne!(rendered, format!("{:?}"))`, which compares
/// whole strings and cannot see a `Debug` fragment sitting inside
/// prose.
///
/// It is asserted OF THESE SIX ARMS and is not a rule over the
/// vocabulary — the distinction this row is built on. `EditError`'s
/// metadata arms quote a user's key on purpose and `MetaUnversioned`
/// names the D7 `"v"` field by writing it, so a census extended to a
/// blanket quote ban would red correct prose. A tripwire over named
/// samples can be stricter than the contract; a claim over a
/// vocabulary cannot.
#[test]
fn refusals_render_as_sentences() {
    let tol = Tol::witness();
    let (doc, profile, extrude) = common::parametric_plate(tol);
    let mut session = DocSession::inline(doc, tol);

    let io = session
        .perform(SessionOp::Open(
            std::env::temp_dir().join("gui3-no-such-document.pncad"),
        ))
        .refusal
        .expect("a missing file refuses");
    assert!(
        io.to_string().contains("cannot read the file"),
        "the io arm names what happened: {io}"
    );

    // The arm that motivated the widening: a value typed into the
    // field of a variable the document does not hold goes to the EDIT
    // door, whose sentence the status line renders verbatim.
    let absent = pncad::document::VarId(0x7461_7070_6572);
    let edit = session
        .perform(SessionOp::SetVariable {
            var: absent,
            value: SlotValue::Continuous(1.0),
        })
        .refusal
        .expect("an undeclared variable refuses");
    assert!(
        edit.to_string().contains(&absent.to_string()),
        "the edit arm names the variable: {edit}"
    );

    let lookup = session
        .perform(SessionOp::BeginVariableGesture { var: absent })
        .refusal
        .expect("dragging an absent variable refuses");
    assert!(
        lookup.to_string().contains(&absent.to_string()),
        "the lookup arm names the variable: {lookup}"
    );
    let kind = session
        .perform(SessionOp::AddExtrude {
            profile: extrude,
            distance: common::len(0.01),
        })
        .refusal
        .expect("an extrude of an extrude refuses");
    let slot = session
        .perform(SessionOp::SetSlot {
            node: profile,
            slot: SlotId::Radius,
            value: SlotValue::Continuous(1.0),
        })
        .refusal
        .expect("a profile has no radius slot");

    // A declare over a taken name, refused by the declare door and
    // forwarded through `Refusal::Edit` in that door's words.
    let exists = session
        .perform(SessionOp::DeclareVar {
            name: common::thickness_param(),
            value: pncad::document::FreeVar::continuous(pncad::document::Dimension::Angle, 1.0),
        })
        .refusal
        .expect("creating over a declared name refuses");
    let shown = exists.to_string();
    assert!(
        shown.contains("thickness"),
        "the refusal names the taken name: {shown}"
    );

    for refusal in [&io, &edit, &lookup, &kind, &slot, &exists] {
        test_utils::f6::assert_f6(refusal, &[], REFUSAL.identifiers(), REFUSAL_FIELDS);
    }

    // And the one mistake that reaches two doors reaches one recourse:
    // the typed route and the dragged route name the same thing to do.
    // Asserted against the CONST both renderings read, so the clause
    // cannot come back as a second literal without this row reddening.
    let recourse = editor_core::edit::UNKNOWN_VAR_RECOURSE;
    assert!(
        edit.to_string().contains(recourse) && lookup.to_string().contains(recourse),
        "typed {edit}\ndragged {lookup}"
    );
}

#[test]
fn an_abandoned_gesture_leaves_no_trace() {
    let tol = Tol::witness();
    let (doc, profile) = {
        let doc: pncad::document::Doc<pncad::document::ProfileProgram> =
            pncad::document::Doc::empty_derived("gui3-abandon", tol);
        common::framed_square(&doc, 0.04, tol)
    };
    let (doc, extrude) = common::inserted(
        &doc,
        pncad::document::Node::Extrude {
            profile,
            distance: common::len(0.008),
            side: ExtrudeSide::Along,
        },
        tol,
    );
    let mut session = DocSession::inline(doc, tol);
    let before = session.history().len();
    session.perform(SessionOp::BeginGesture {
        node: extrude,
        slot: SlotId::Distance,
    });
    session.perform(SessionOp::PreviewGesture {
        node: extrude,
        slot: SlotId::Distance,
        value: 0.03,
    });
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
            .perform(SessionOp::PreviewGesture {
                node: extrude,
                slot: SlotId::Distance,
                value: 1.0
            })
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
        common::row_of(&rows, extrude).depth,
        0,
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

/// **The create affordance's whole arc**: create → the variable
/// exists → an expression referencing it now parses → one undo
/// removes it.
#[test]
fn create_parameter_reference_it_and_one_undo_removes_it() {
    let tol = Tol::witness();
    let (doc, _profile, extrude) = common::parametric_plate(tol);
    let mut session = DocSession::inline(doc, tol);
    let margin = pncad::document::VarName::from_static("margin");

    // Before: an expression naming the undeclared variable refuses
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

    // Create: exactly one committed DeclareVar — the CREATE door
    // really is authoring a declaration — and one undo step.
    let outcome = session.perform(SessionOp::DeclareVar {
        name: margin.clone(),
        value: pncad::document::FreeVar::continuous(pncad::document::Dimension::Length, 0.005),
    });
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    assert_eq!(outcome.committed.len(), 1);
    assert!(matches!(
        outcome.committed.first(),
        Some(DocEdit::DeclareVar { .. })
    ));
    assert_eq!(session.history().len(), before + 1, "one undo step");
    let row = props::variable_rows(session.committed_doc())
        .into_iter()
        .find(|row| row.label.name() == Some(&margin))
        .expect("the variable exists");
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

    // Undo the expression edit, then ONE undo removes the variable.
    session.perform(SessionOp::Undo);
    assert!(
        props::variable_rows(session.committed_doc())
            .into_iter()
            .any(|row| row.label.name() == Some(&margin)),
        "the first undo returns only the expression edit"
    );
    session.perform(SessionOp::Undo);
    assert!(
        !props::variable_rows(session.committed_doc())
            .into_iter()
            .any(|row| row.label.name() == Some(&margin)),
        "one more undo removes the creation"
    );
}

/// **Create is not replace.** The panel's create door is the declare
/// door, which refuses a taken name itself (`EditError::VarNameTaken`,
/// naming the holder); the viewer forwards that refusal through
/// `Refusal::Edit` and adds no reading of its own. The replace act
/// stays spellable through the door that says so (`SetVariable`).
#[test]
fn the_create_door_forwards_the_declares_name_taken_and_setvariable_still_replaces() {
    let tol = Tol::witness();
    let (doc, _profile, _extrude) = common::parametric_plate(tol);
    let mut session = DocSession::inline(doc, tol);
    let before = session.history().len();
    let thickness = common::thickness_var(session.committed_doc());

    // Creating over "thickness" — even at a DIFFERENT dimension, the
    // riskier half of a silent replace — refuses typed and unchanged.
    let outcome = session.perform(SessionOp::DeclareVar {
        name: common::thickness_param(),
        value: pncad::document::FreeVar::Count { value: 3 },
    });
    match outcome.refusal.as_ref() {
        Some(Refusal::Edit(error)) => match error.as_ref() {
            EditError::VarNameTaken { name, holder } => {
                assert_eq!(name, &common::thickness_param());
                assert_eq!(
                    holder.id(),
                    thickness,
                    "the refusal names the variable already holding the name"
                );
            }
            other => panic!("expected the declare's name-taken refusal, got {other:?}"),
        },
        other => panic!("expected the forwarded edit refusal, got {other:?}"),
    }
    assert!(outcome.committed.is_empty(), "a refusal commits nothing");
    assert_eq!(session.history().len(), before, "and mints no history");
    assert_eq!(
        session
            .committed_doc()
            .free(thickness)
            .map(|free| free.dim()),
        Some(pncad::document::Dimension::Length),
        "the standing declaration is untouched"
    );

    // The REPLACE door still replaces — same underlying edit, spelled
    // as what it is.
    let outcome = session.perform(SessionOp::SetVariable {
        var: common::thickness_var(session.committed_doc()),
        value: SlotValue::Continuous(0.012),
    });
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    assert_eq!(outcome.committed.len(), 1);
}

/// **`50 mm` sets the value AND the notation, as one undo step.**
///
/// The text door reads both out of one literal — `parse_formula` applies
/// the unit factor once, on the way in — and commits them as one
/// action, so the history gains exactly one state and an undo puts
/// both halves back.
#[test]
fn a_unit_bearing_text_sets_the_value_and_the_notation_as_one_undo() {
    let tol = Tol::witness();
    let name = VarName::from_static("base_r");
    let mut session = DocSession::inline(
        common::declared(
            "auth2-written",
            &name,
            FreeVar::written_length(WrittenLength::in_unit(20.0, MM)),
            tol,
        ),
        tol,
    );
    let before = session.history().len();

    let outcome = session.perform(SessionOp::SetVariableText {
        var: common::var_of(session.committed_doc(), name.as_str()),
        text: "50 mm".to_owned(),
    });
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    assert_eq!(session.history().len(), before + 1, "one undo step");
    assert_eq!(
        outcome.committed.len(),
        1,
        "one action, whatever it is made of"
    );

    let row = variable_row(&session, &name);
    assert_eq!(
        row.value,
        SlotValue::Continuous(0.05),
        "fifty millimetres is 0.05 m — applied once, by the parser"
    );
    assert_eq!(row.unit.map(|u| u.symbol()), Some("mm"));
    assert_eq!(
        props::in_written(row.value.as_f64(), row.unit.expect("a length row")),
        50.0,
        "and the row reads back as 50 beside mm"
    );

    // One action, so ONE undo takes both halves back.
    session.perform(SessionOp::Undo);
    let row = variable_row(&session, &name);
    assert_eq!(row.value, SlotValue::Continuous(0.02));
    assert_eq!(row.unit.map(|u| u.symbol()), Some("mm"));
}

/// **A number that changes only the notation moves only the
/// notation**, and the same text a second time is not an edit at all:
/// the door submits the edits that change something and nothing else.
#[test]
fn text_that_says_what_the_declaration_already_says_is_not_an_edit() {
    let tol = Tol::witness();
    let name = VarName::from_static("base_r");
    let mut session = DocSession::inline(
        common::declared(
            "auth2-noop",
            &name,
            FreeVar::written_length(WrittenLength::in_unit(50.0, MM)),
            tol,
        ),
        tol,
    );
    let before = session.history().len();
    let outcome = session.perform(SessionOp::SetVariableText {
        var: common::var_of(session.committed_doc(), name.as_str()),
        text: "50 mm".to_owned(),
    });
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    assert!(outcome.committed.is_empty(), "nothing changed, so no edit");
    assert_eq!(session.history().len(), before, "and no undo step");

    // The same value, said in another notation: the notation moves and
    // the value does not.
    let outcome = session.perform(SessionOp::SetVariableText {
        var: common::var_of(session.committed_doc(), name.as_str()),
        text: "0.05 m".to_owned(),
    });
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    assert!(matches!(
        outcome.committed.as_slice(),
        [DocEdit::SetVarUnit { .. }]
    ));
    let row = variable_row(&session, &name);
    assert_eq!(row.unit.map(|u| u.symbol()), Some("m"));
    assert_eq!(row.value, SlotValue::Continuous(0.05));
}

/// **An expression typed into a variable DEFINES it**, keeping its
/// identity, and its panel row turns into its formula; a number typed
/// back makes it free again.
///
/// One reading itself is refused as the cycle it is, by the door, and
/// nothing moves. `base_r * 2` does not reach the door at all: `2` is
/// a count and the expression vocabulary refuses a count times a
/// length without an explicit promotion, so what a user reads there is
/// the parser's sentence about the multiply.
#[test]
fn an_expression_typed_into_a_parameter_defines_it() {
    let tol = Tol::witness();
    let name = VarName::from_static("base_r");
    let doc = common::declared(
        "auth2-expression",
        &name,
        FreeVar::written_length(WrittenLength::in_unit(50.0, MM)),
        tol,
    );
    let mut session = DocSession::inline(doc, tol);
    let declared = session.perform(SessionOp::DeclareVar {
        name: VarName::from_static("rim"),
        value: FreeVar::written_length(WrittenLength::in_unit(20.0, MM)),
    });
    assert!(declared.refusal.is_none(), "{:?}", declared.refusal);
    let base_r = common::var_of(session.committed_doc(), name.as_str());
    let before = session.history().len();
    for text in ["base_r * 2.0", "base_r"] {
        let refusal = session
            .perform(SessionOp::SetVariableText {
                var: base_r,
                text: text.to_owned(),
            })
            .refusal
            .expect("a definition reading itself is a cycle");
        assert!(
            matches!(&refusal, Refusal::Edit(error) if matches!(**error, EditError::DefinitionCycle { .. })),
            "{text}: {refusal:?}"
        );
    }
    let refusal = session
        .perform(SessionOp::SetVariableText {
            var: base_r,
            text: "base_r * 2".to_owned(),
        })
        .refusal
        .expect("a count times a length needs an explicit promotion");
    assert!(matches!(refusal, Refusal::Parse(_)), "{refusal:?}");
    assert_eq!(session.history().len(), before, "and nothing moved");

    let outcome = session.perform(SessionOp::SetVariableText {
        var: base_r,
        text: "rim * 2.0".to_owned(),
    });
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    assert!(
        matches!(outcome.committed.as_slice(), [DocEdit::DefineVar { .. }]),
        "{:?}",
        outcome.committed
    );
    assert_eq!(
        session.committed_doc().var_named("base_r"),
        Some(base_r),
        "the definition keeps the variable's identity"
    );
    let rows = props::defined_rows(session.doc());
    assert_eq!(rows.len(), 1, "{rows:?}");
    assert_eq!(rows[0].var, base_r);
    assert_eq!(rows[0].formula, "rim * 2.0");
    assert!(
        props::variable_rows(session.doc())
            .iter()
            .all(|row| row.var != base_r),
        "a defined variable has no value row"
    );

    let outcome = session.perform(SessionOp::SetVariableText {
        var: base_r,
        text: "30 mm".to_owned(),
    });
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    assert!(props::defined_rows(session.doc()).is_empty());
    let row = variable_row(&session, &name);
    assert_eq!(row.value, SlotValue::Continuous(0.03));
    assert_eq!(row.unit.map(|u| u.symbol()), Some("mm"));
}

/// **Constant text typed over a free variable is a VALUE**, folded as
/// a written quantity is: `SetVarValue`, so the variable keeps its
/// identity, its notation and its tolerance, and stays an analysis
/// axis. The reviewers' probe: a toleranced `base_r` given constant
/// arithmetic used to become a definition that read nothing.
#[test]
fn constant_text_over_a_toleranced_parameter_writes_its_value() {
    let tol = Tol::witness();
    let name = VarName::from_static("base_r");
    let toleranced = FreeVar::written_length(WrittenLength::in_unit(50.0, MM))
        .with_distribution(Some(Distribution::Normal { sigma: 0.0001 }))
        .expect("a valid distribution");
    let mut session = DocSession::inline(
        common::declared("literals-a-constant", &name, toleranced, tol),
        tol,
    );
    let base_r = common::var_of(session.committed_doc(), name.as_str());
    for (text, metres) in [
        ("50 mm + 1 mm", 0.051),
        ("-(5 mm)", -0.005),
        ("2.0 * 25.5 mm", 0.051),
    ] {
        let before = session.history().len();
        let outcome = session.perform(SessionOp::SetVariableText {
            var: base_r,
            text: text.to_owned(),
        });
        assert!(outcome.refusal.is_none(), "{text}: {:?}", outcome.refusal);
        assert!(
            matches!(outcome.committed.as_slice(), [DocEdit::SetVarValue { .. }]),
            "{text}: {:?}",
            outcome.committed
        );
        assert_eq!(session.history().len(), before + 1, "{text}: one edit");
        let doc = session.committed_doc();
        let free = doc.free(base_r).expect("still a free variable");
        let FreeVar::Continuous { value, .. } = *free else {
            panic!("{text}: a length is continuous: {free:?}")
        };
        assert!((value - metres).abs() < 1e-12, "{text}: {free:?}");
        assert_eq!(
            free.distribution(),
            Some(&Distribution::Normal { sigma: 0.0001 }),
            "{text}: the tolerance stands"
        );
        assert_eq!(
            variable_row(&session, &name).unit.map(|u| u.symbol()),
            Some("mm"),
            "{text}: the notation stands"
        );
    }
    // A constant that does not fold refuses with the evaluator's words.
    let before = session.history().len();
    let refusal = session
        .perform(SessionOp::SetVariableText {
            var: base_r,
            text: "1 mm / 0.0".to_owned(),
        })
        .refusal
        .expect("no finite value");
    assert!(
        matches!(refusal, Refusal::ConstantRefused { .. }),
        "{refusal:?}"
    );
    assert_eq!(session.history().len(), before, "nothing moved");
}

/// **A count typed over a length refuses in the value door's words** —
/// the typed value is a count and the variable a length — whether the
/// variable is free or defined, and a defined one keeps its definition.
#[test]
fn a_count_typed_over_a_length_refuses_as_a_value_of_another_kind() {
    let tol = Tol::witness();
    let name = VarName::from_static("base_r");
    let mut session = DocSession::inline(
        common::declared(
            "literals-a-count-over-length",
            &name,
            FreeVar::written_length(WrittenLength::in_unit(50.0, MM)),
            tol,
        ),
        tol,
    );
    let declared = session.perform(SessionOp::DeclareVar {
        name: VarName::from_static("rim"),
        value: FreeVar::written_length(WrittenLength::in_unit(20.0, MM)),
    });
    assert!(declared.refusal.is_none(), "{:?}", declared.refusal);
    let base_r = common::var_of(session.committed_doc(), name.as_str());
    let refused = |session: &mut DocSession| {
        let before = session.history().len();
        let refusal = session
            .perform(SessionOp::SetVariableText {
                var: base_r,
                text: "3".to_owned(),
            })
            .refusal
            .expect("a count is no length");
        assert_eq!(session.history().len(), before, "nothing moved");
        refusal
    };
    let free = refused(&mut session);
    assert!(
        matches!(&free, Refusal::Edit(error) if matches!(**error, EditError::VarValueKindMismatch { .. })),
        "{free:?}"
    );
    let defined = session.perform(SessionOp::SetVariableText {
        var: base_r,
        text: "rim * 2.0".to_owned(),
    });
    assert!(defined.refusal.is_none(), "{:?}", defined.refusal);
    let over_definition = refused(&mut session);
    assert_eq!(
        over_definition.to_string(),
        free.to_string(),
        "one mistake, one sentence"
    );
    let shown = over_definition.to_string();
    assert!(
        shown.contains("declared length") && shown.contains("offered a count"),
        "{shown}"
    );
    assert!(!shown.contains("definition offered"), "{shown}");
    assert_eq!(
        props::defined_rows(session.doc())[0].formula,
        "rim * 2.0",
        "the definition stands"
    );
}

/// **A defined variable's field is the way back** (spec §2 item 9): it
/// shows the formula, a formula committed through it redefines the
/// variable, and a number frees it. A drag over it is refused by the
/// value door, in its own words.
#[test]
fn a_defined_parameters_field_redefines_or_frees_it() {
    let tol = Tol::witness();
    let name = VarName::from_static("base_r");
    let mut session = DocSession::inline(
        common::declared(
            "literals-a-way-back",
            &name,
            FreeVar::written_length(WrittenLength::in_unit(50.0, MM)),
            tol,
        ),
        tol,
    );
    let declared = session.perform(SessionOp::DeclareVar {
        name: VarName::from_static("rim"),
        value: FreeVar::written_length(WrittenLength::in_unit(20.0, MM)),
    });
    assert!(declared.refusal.is_none(), "{:?}", declared.refusal);
    let base_r = common::var_of(session.committed_doc(), name.as_str());
    let text = |session: &mut DocSession, text: &str| {
        let outcome = session.perform(SessionOp::SetVariableText {
            var: base_r,
            text: text.to_owned(),
        });
        assert!(outcome.refusal.is_none(), "{text}: {:?}", outcome.refusal);
    };
    text(&mut session, "rim * 2.0");
    let row = &props::defined_rows(session.doc())[0];
    assert_eq!(row.formula, "rim * 2.0");
    assert_eq!(row.value, Some(SlotValue::Continuous(0.04)));

    let begun = session.perform(SessionOp::BeginVariableGesture { var: base_r });
    assert!(begun.refusal.is_none(), "{:?}", begun.refusal);
    let dragged = session
        .perform(SessionOp::PreviewVariableGesture {
            var: base_r,
            value: 0.05,
        })
        .refusal
        .expect("a defined variable holds no value to drag");
    assert!(
        matches!(&dragged, Refusal::Edit(error) if matches!(**error, EditError::NotAFreeVar { .. })),
        "{dragged:?}"
    );
    let _ = session.perform(SessionOp::CancelGesture);

    text(&mut session, "rim + 1 mm");
    assert_eq!(
        props::defined_rows(session.doc())[0].formula,
        "rim + 1 mm",
        "a formula redefines it"
    );
    assert_eq!(session.committed_doc().var_named("base_r"), Some(base_r));
    text(&mut session, "45 mm");
    assert!(
        props::defined_rows(session.doc()).is_empty(),
        "a number frees it"
    );
    let row = variable_row(&session, &name);
    assert_eq!(row.value, SlotValue::Continuous(0.045));
    assert_eq!(row.unit.map(|u| u.symbol()), Some("mm"));
}

/// **An unknown unit carries the parser's own refusal**, which names
/// the token and its offset — not a sentence re-composed at this door.
#[test]
fn an_unknown_unit_carries_the_parsers_own_wording() {
    let tol = Tol::witness();
    let name = VarName::from_static("base_r");
    let mut session = DocSession::inline(
        common::declared(
            "auth2-unknown-unit",
            &name,
            FreeVar::written_length(WrittenLength::in_unit(50.0, MM)),
            tol,
        ),
        tol,
    );
    let outcome = session.perform(SessionOp::SetVariableText {
        var: common::var_of(session.committed_doc(), name.as_str()),
        text: "50 furlong".to_owned(),
    });
    let refusal = outcome.refusal.expect("furlong is not a table row");
    assert!(matches!(refusal, Refusal::Parse(_)), "{refusal:?}");
    let shown = refusal.to_string();
    assert!(
        shown.contains("furlong") && shown.contains("is not a unit symbol"),
        "the parser's own sentence, naming the token: {shown}"
    );
    assert!(
        !shown.contains("holds a number, not an expression"),
        "and not this door's: {shown}"
    );
    assert_eq!(
        variable_row(&session, &name).value,
        SlotValue::Continuous(0.05)
    );
}

/// **A unit that does not measure the declared dimension is refused by
/// the edit door**, in the door's words, and the all-or-nothing action
/// means the value it was paired with does not land either.
#[test]
fn a_wrong_dimension_unit_refuses_the_whole_action() {
    let tol = Tol::witness();
    let name = VarName::from_static("sweep");
    let mut session = DocSession::inline(
        common::declared(
            "auth2-mismatch",
            &name,
            FreeVar::continuous(pncad::document::Dimension::Angle, 1.0),
            tol,
        ),
        tol,
    );
    let before = session.history().len();
    let outcome = session.perform(SessionOp::SetVariableText {
        var: common::var_of(session.committed_doc(), name.as_str()),
        text: "50 mm".to_owned(),
    });
    let refusal = outcome
        .refusal
        .expect("millimetres do not measure an angle");
    assert!(matches!(refusal, Refusal::Edit(_)), "{refusal:?}");
    let shown = refusal.to_string();
    assert!(
        shown.contains("declared angle") && shown.contains("measures length"),
        "the edit door names both dimensions: {shown}"
    );
    assert_eq!(session.history().len(), before, "nothing was recorded");
    let row = variable_row(&session, &name);
    assert_eq!(
        row.value,
        SlotValue::Continuous(1.0),
        "and the value the action carried did not land either"
    );
    assert_eq!(row.unit.map(|u| u.symbol()), Some("rad"));
}

/// **The row's unit picker moves the notation and nothing else** — one
/// `SetVarUnit`, and the stored value bit-identical.
#[test]
fn the_parameter_unit_picker_leaves_the_value_where_it_was() {
    let tol = Tol::witness();
    let name = VarName::from_static("base_r");
    let mut session = DocSession::inline(
        common::declared(
            "auth2-picker",
            &name,
            FreeVar::continuous(pncad::document::Dimension::Length, 0.05),
            tol,
        ),
        tol,
    );
    let outcome = session.perform(SessionOp::SetVariableUnit {
        var: common::var_of(session.committed_doc(), name.as_str()),
        unit: MM.def(),
    });
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    assert!(matches!(
        outcome.committed.as_slice(),
        [DocEdit::SetVarUnit { .. }]
    ));
    let row = variable_row(&session, &name);
    assert_eq!(row.unit.map(|u| u.symbol()), Some("mm"));
    assert_eq!(row.value, SlotValue::Continuous(0.05));

    // A count names no notation, and the door says so rather than
    // this one guessing.
    let holes = VarName::from_static("holes");
    let mut counted = DocSession::inline(
        common::declared(
            "auth2-picker-count",
            &holes,
            FreeVar::Count { value: 6 },
            tol,
        ),
        tol,
    );
    let refusal = counted
        .perform(SessionOp::SetVariableUnit {
            var: common::var_of(counted.committed_doc(), holes.as_str()),
            unit: MM.def(),
        })
        .refusal
        .expect("a count has no display unit");
    assert!(
        refusal.to_string().contains("no display unit to change"),
        "{refusal}"
    );
}

/// **A refusal names the half the user was editing.**
///
/// `8 mm` typed into a COUNT variable's field is a value edit with a
/// notation on it. The notation half has nothing to say about a count
/// — a count is an integer and names no unit under any declaration —
/// so the door submits the value edit alone and what the user reads
/// is the refusal of the thing they did: a count declared where a
/// continuous value was typed. Answering it in the notation's words
/// ("it has no display unit to change") would describe a change
/// nobody asked for.
#[test]
fn a_count_refuses_a_unit_bearing_value_in_the_values_words() {
    let tol = Tol::witness();
    let holes = VarName::from_static("holes");
    let mut session = DocSession::inline(
        common::declared("auth2-count-text", &holes, FreeVar::Count { value: 6 }, tol),
        tol,
    );
    let before = session.history().len();
    let refusal = session
        .perform(SessionOp::SetVariableText {
            var: common::var_of(session.committed_doc(), holes.as_str()),
            text: "8 mm".to_owned(),
        })
        .refusal
        .expect("a count takes no continuous value");
    let shown = refusal.to_string();
    assert!(
        shown.contains("declared count") && shown.contains("value edit"),
        "the sentence names the value half: {shown}"
    );
    assert!(
        !shown.contains("display unit"),
        "and not a notation change nobody asked for: {shown}"
    );
    assert_eq!(session.history().len(), before, "and nothing moved");
    assert_eq!(variable_row(&session, &holes).value, SlotValue::Count(6));
}

/// The panel row for `name`, as the panel reads it.
fn variable_row(session: &DocSession, name: &VarName) -> props::VariableRow {
    props::variable_rows(session.doc())
        .into_iter()
        .find(|row| row.label.name() == Some(name))
        .expect("the variable row")
}

/// **A typed `NaN` or `inf` in a Count slot is refused, by the name
/// the props module promises names it.**
///
/// `props::field_edit` reads `inf` and `NaN` as Numbers deliberately,
/// and says what pays for it: `Formula::literal`'s refusal names the
/// problem where the parser would only say the word is not a
/// variable. That promise had a hole exactly one dimension wide.
/// `SlotValue::of` splits on the dimension BEFORE any expression is
/// built, and `value as i64` is a saturating cast, not a conversion —
/// `NaN` is `0` and `inf` is `i64::MAX` — so a structural slot
/// committed an ordinary integer and the literal door was never asked.
///
/// **The error is read from BOTH sides rather than restated here.**
/// The claim is that the Count arm refuses what the continuous arm's
/// literal door refuses, so the expected value is that door's own
/// answer, taken by calling it. A row spelling `NonFiniteLiteral` as
/// a literal would be a third copy, agreeing with whichever side it
/// was written from.
///
/// **And the pair**: refusing everything would satisfy the first half.
/// The second is every legitimate count — the truncation toward zero
/// the door documents, at both signs and at zero — which has to come
/// back as the count it names.
#[test]
fn a_count_slot_refuses_a_value_that_is_not_a_number() {
    use pncad::document::{Dimension, Formula};

    let literal_door = Formula::literal(f64::NAN, Dimension::Length)
        .expect_err("a non-finite continuous literal is refused at construction");
    for poison in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
        let refused = SlotValue::of(Dimension::Count, poison).expect_err(
            "a Count slot took a value that is not a number; the saturating cast \
             commits an ordinary integer past the finiteness refusal",
        );
        assert_eq!(
            refused, literal_door,
            "a Count slot refused {poison} as {refused}, where the continuous \
             half's literal door says {literal_door}"
        );
    }
    // Every Count-dimensioned slot, so the refusal is the DIMENSION's
    // and not one slot's: `SlotId::is_structural` is defined as this
    // dimension, so these are exactly the structural slots.
    for slot in [
        SlotId::Count,
        SlotId::VDegree,
        SlotId::Stations,
        SlotId::Instance,
    ] {
        assert!(slot.is_structural(), "{slot:?} is not a structural slot");
        assert!(
            SlotValue::of(slot.dimension(), f64::INFINITY).is_err(),
            "{slot:?} took an infinite count"
        );
    }
    // The other half: a count the cast can carry comes back as itself,
    // truncated toward zero as the door documents.
    for (value, expected) in [(0.0, 0), (3.0, 3), (3.7, 3), (-3.7, -3), (-0.5, 0)] {
        assert_eq!(
            SlotValue::of(Dimension::Count, value).expect("a finite value is a count"),
            SlotValue::Count(expected),
            "a Count slot read {value} as something other than {expected}"
        );
    }
    // And the continuous arm is untouched: its value reaches the
    // literal door intact and is refused THERE, which is the
    // arrangement the Count arm has been brought into line with.
    // Compared by matching rather than by equality: `NaN` is equal to
    // nothing, itself included, so an `assert_eq!` here would be red
    // on a door that is right.
    let carried = SlotValue::of(Dimension::Length, f64::NAN).expect("the continuous arm carries");
    assert!(
        matches!(carried, SlotValue::Continuous(v) if v.is_nan()),
        "the continuous arm answered {carried:?} instead of carrying its value          to the literal door"
    );
}

/// **A rename moves the name and nothing keyed by the id.**
///
/// `RenameVar` commits exactly one `DocEdit::RenameVar` and one undo
/// step. The variables panel keys its rows by `VarId`, so the row for
/// the renamed variable is the same row — same id, same place in the
/// list, same value — under its new label; the selection on it stays
/// selected and still denotes; and the slot that reads it is written
/// by the new name, because a reader holds the id and not the name.
/// One undo puts the old name back on the same row.
#[test]
fn a_rename_keeps_the_parameter_row_and_its_selection_and_is_one_undo_step() {
    let tol = Tol::witness();
    let (doc, _profile, extrude) = common::parametric_plate(tol);
    // A second row, declared after `thickness` and named between the
    // old name and the new one: under a name sort the renamed row would
    // move from second to first, so "the same place" can go red.
    let (doc, _) = viewer::test_support::edited(
        &doc,
        DocEdit::DeclareVar {
            name: VarName::from_static("mid"),
            def: pncad::document::VarDecl::Free(FreeVar::continuous(Dimension::Length, 0.002)),
        },
        tol,
    );
    let mut session = DocSession::inline(doc, tol);
    let thickness = common::thickness_var(session.committed_doc());
    session.perform(SessionOp::Select(Selection::Variable(thickness)));
    let rows_before = props::variable_rows(session.committed_doc());
    assert!(rows_before.len() >= 2, "a re-sort has a row to move past");
    let at = rows_before
        .iter()
        .position(|row| row.var == thickness)
        .expect("the variable has a row");
    let before = session.history().len();
    let depth = VarName::from_static("depth");

    let outcome = session.perform(SessionOp::RenameVar {
        var: thickness,
        name: Some(depth.clone()),
    });
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    assert!(
        matches!(
            outcome.committed.as_slice(),
            [DocEdit::RenameVar { var, name: Some(name) }]
                if *var == thickness.into() && *name == depth
        ),
        "exactly one rename edit: {:?}",
        outcome.committed
    );
    assert_eq!(session.history().len(), before + 1, "one undo step");

    let rows = props::variable_rows(session.committed_doc());
    assert_eq!(rows.len(), rows_before.len(), "no row came or went");
    let row = &rows[at];
    assert_eq!(
        row.var, thickness,
        "the row at the same place is the same variable"
    );
    assert_eq!(row.label.name(), Some(&depth), "under its new name");
    assert_eq!(row.value, rows_before[at].value, "with its value untouched");
    assert_eq!(
        session.selection(),
        &Selection::Variable(thickness),
        "the selection rides the id"
    );
    assert!(
        matches!(
            session.standing(),
            viewer::session::Standing::Variable { ref var, present: true }
                if var.name() == Some(&depth)
        ),
        "and still denotes, spoken by the new name: {:?}",
        session.standing()
    );
    let source = props::slot_rows(session.committed_doc(), extrude)
        .into_iter()
        .find(|row| row.slot == SlotId::Distance)
        .and_then(|row| row.source)
        .expect("the driven slot shows its source");
    assert!(
        source.contains("depth") && !source.contains("thickness"),
        "the reader is written by the name its variable holds now: {source}"
    );

    session.perform(SessionOp::Undo);
    let row = &props::variable_rows(session.committed_doc())[at];
    assert_eq!(row.var, thickness);
    assert_eq!(
        row.label.name(),
        Some(&common::thickness_param()),
        "one undo returns the old name to the same row"
    );
}

/// **A delete removes the variable and leaves its readers unresolved,
/// typed.**
///
/// `DeleteVar` commits exactly one `DocEdit::DeleteVar`. The row goes,
/// and the slot that read the variable does not evaluate: its value is
/// `EvalError::UnresolvedVar` naming the deleted id, both in the
/// panel's row and in the evaluated node's own refusal.
#[test]
fn a_delete_removes_the_parameter_and_its_reader_refuses_unresolved() {
    let tol = Tol::witness();
    let (doc, _profile, extrude) = common::parametric_plate(tol);
    let mut session = DocSession::inline(doc, tol);
    session.pump();
    let thickness = common::thickness_var(session.committed_doc());
    let before = session.history().len();

    let outcome = session.perform(SessionOp::DeleteVar { var: thickness });
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    assert!(
        matches!(
            outcome.committed.as_slice(),
            [DocEdit::DeleteVar { var }] if *var == thickness.into()
        ),
        "exactly one delete edit: {:?}",
        outcome.committed
    );
    assert_eq!(session.history().len(), before + 1, "one undo step");
    assert!(
        props::variable_rows(session.committed_doc())
            .iter()
            .all(|row| row.var != thickness),
        "the row is gone"
    );

    let value = props::slot_rows(session.committed_doc(), extrude)
        .into_iter()
        .find(|row| row.slot == SlotId::Distance)
        .expect("the slot is listed")
        .value;
    assert_eq!(
        value,
        Err(props::SlotFault::Eval(
            pncad::document::EvalError::UnresolvedVar { var: thickness }
        )),
        "the reader's row says which variable it can no longer read"
    );

    session.pump();
    let eval = session.evaluation().expect("the inline seam landed");
    let error = eval
        .result(extrude)
        .and_then(pncad::document::NodeResult::error)
        .expect("the reader fails");
    // The slot reads the anonymous definition its formula lowered to,
    // whose refusal is the slot's own: the variable it cannot read.
    assert!(
        matches!(
            &error.kind,
            pncad::document::NodeErrorKind::Expr {
                slot: SlotId::Distance,
                source: pncad::document::EvalError::UnresolvedVar { var },
            } if *var == thickness
        ),
        "the evaluated node refuses typed, naming the variable: {:?}",
        error.kind
    );
}
