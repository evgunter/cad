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

use pncad::document::{Dimension, DocEdit, DocParam, ParamName, SlotId};
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
        common::framed_square(&doc, 0.04, tol)
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
        },
        tol,
    );
    let (doc, pattern) = common::inserted(
        &doc,
        pncad::document::Node::Pattern {
            input: extrude,
            count: pncad::document::Expr::count(3),
            kind: pncad::document::PatternKind::Linear {
                direction: [common::scl(1.0), common::scl(0.0), common::scl(0.0)],
                spacing: common::len(0.03),
            },
        },
        tol,
    );
    (doc, extrude, pattern)
}

/// Replace the object a saved document's first `"<key>":` holds, by
/// matching braces — the file-modality surgery, in the one place this
/// suite needs it.
///
/// Deliberately NOT a parse-and-re-serialize: a round trip through a
/// JSON value would rewrite bytes this suite has not asked about, and
/// the point of the row below is that ONE field was hand-edited into
/// something no door would have written.
fn retyped_field(text: &str, key: &str, replacement: &str) -> String {
    let at = text
        .find(&format!("\"{key}\":"))
        .expect("the wire carries that key");
    let start = at + text[at..].find('{').expect("its value is an object");
    let mut depth = 0usize;
    let mut end = None;
    for (i, c) in text[start..].char_indices() {
        match c {
            '{' => depth += 1,
            '}' => {
                depth -= 1;
                if depth == 0 {
                    end = Some(start + i + 1);
                    break;
                }
            }
            _ => {}
        }
    }
    let end = end.expect("the object closes");
    let out = format!("{}{replacement}{}", &text[..start], &text[end..]);
    assert_ne!(out, text, "the corruption really landed");
    out
}

/// **A slot the document holds a bare literal for always has a value**
/// — which is why the range button beside it is gated on the driver
/// alone, with no second conjunct on the value.
///
/// `props::slot_row` evaluates each slot with the branch
/// `SlotId::dimension` picks, so the only way a leaf carrying no
/// parameter reference fails to evaluate is a Count/continuous
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
/// empty parameter list, so the button refuses it as a driven slot
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
            expr: pncad::document::Expr::count(3),
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
/// A hand edit or a foreign tool is the only way a Count literal can
/// be sitting in a `Length` slot, and it is the input the claim above
/// most needs: everything else in this suite reaches the document
/// through `apply`. The saved fixture is doctored in ONE field and
/// `load` is asked what it thinks.
#[test]
fn the_load_door_refuses_a_count_literal_in_a_continuous_slot() {
    let tol = Tol::witness();
    let (doc, extrude, _pattern) = literal_and_pattern_doc(tol);
    let text = pncad::document::save(&doc, &[], tol).expect("the fixture saves");
    pncad::document::load(&text, tol).expect("and loads back as it was written");

    // A `CountLiteral` on the wire is `{"Count": n}`; the extrude's
    // distance is a `Length` slot.
    let corrupt = retyped_field(
        &text,
        "distance",
        "{\n              \"Count\": 3\n            }",
    );
    match pncad::document::load(&corrupt, tol) {
        Err(pncad::document::PersistError::Snapshot(
            pncad::document::SnapshotError::SlotDimension {
                node,
                slot,
                expected,
                found,
            },
        )) => {
            assert_eq!(node, extrude);
            assert_eq!(slot, SlotId::Distance);
            assert_eq!(expected, Dimension::Length);
            assert_eq!(found, Dimension::Count);
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
        common::framed_square(&doc, 0.04, tol)
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
        let outcome = session.perform(SessionOp::PreviewParamGesture {
            name: name.clone(),
            value: last,
        });
        assert!(outcome.committed.is_empty(), "a preview commits nothing");
        previews += outcome.previewed.len();
        assert_eq!(session.history().len(), before, "and mints no history");
    }
    assert_eq!(previews, 5);

    let outcome = session.perform(SessionOp::CommitParamGesture { name: name.clone() });
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
            .perform(SessionOp::PreviewParamGesture {
                name: pncad::document::ParamName::new("no-such-parameter"),
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
    /// sampled renderings are held to. The `match` the macro writes is
    /// exhaustive, so an arm added to `Refusal` stops this file
    /// compiling until it is listed here and the ban covers it. No
    /// count is written down: the `match` is what holds the roster
    /// complete, and a number beside it would be a second claim with
    /// nothing checking it.
    ///
    /// **The roster is the enum's, not the sample's.** A rendering that
    /// leaks a SIBLING arm's identifier is as much a dump as one that
    /// leaks its own, and a per-arm check cannot see it.
    const REFUSAL: Refusal = [
        DrivenByExpression,
        NoSuchSlot,
        NoSuchParam,
        ParamNotANumber,
        ParamExists,
        EmptyName,
        WrongNodeKind,
        Duplicate,
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
        ProfileRestructure,
        ProfileEditOrder,
        ProfileEditOrderCapped,
        ProfileEditStale,
    ];
}

/// The `Debug` punctuation that would be a dump in a `Refusal`
/// sentence: the two field names the payloads carry, and the quotation
/// mark a `{:?}` over a `String` or a `ParamName` leaves behind.
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
/// * **that every arm renders at all** is the compiler's.
///   `Display for Refusal` and `Refusal::rank` are exhaustive matches
///   with no wildcard, so a nineteenth arm reds both until it is
///   given a sentence and a rank. That is the obligation an `ALL`
///   over this vocabulary was wanted for, and the type has it already
///   — which is why there is no `Refusal::ALL` to walk.
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
/// for. A `{:?}` over a `String` or a `ParamName` renders `"width"`:
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

    // The arm that motivated the widening: an undeclared name typed
    // into the value field goes to the EDIT door, whose sentence the
    // status line renders verbatim.
    let edit = session
        .perform(SessionOp::SetParam {
            name: pncad::document::ParamName::new("tapper"),
            value: SlotValue::Continuous(1.0),
        })
        .refusal
        .expect("an undeclared parameter refuses");
    assert!(
        edit.to_string().contains("tapper"),
        "the edit arm names the parameter: {edit}"
    );

    let lookup = session
        .perform(SessionOp::BeginParamGesture {
            name: pncad::document::ParamName::new("tapper"),
        })
        .refusal
        .expect("dragging an absent parameter refuses");
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

    // The arm whose sentence is composed OUTSIDE `Display` — through
    // `Refusal::exists_wording`, shared with the add-parameter form's
    // pre-click notice — and therefore outside the source census,
    // which reads `impl Display` bodies.
    //
    // **The ASKED-for dimension differs from the declared one**, and
    // it has to: this arm exists to name what already stands there
    // (`create_param` reads `existing.dim()`), and a fixture that
    // asks for the dimension it declared cannot tell that apart from
    // an arm forwarding the request — which is the one mistake the
    // refusal guards, `SetDocParam` being create-or-replace at the
    // API. `thickness` is declared a length; this asks for an angle.
    let exists = session
        .perform(SessionOp::CreateParam {
            name: common::thickness_param(),
            value: pncad::document::DocParam::continuous(pncad::document::Dimension::Angle, 1.0),
        })
        .refusal
        .expect("creating over a declared name refuses");
    let shown = exists.to_string();
    assert!(
        shown.contains("(length)"),
        "the dimension is the quantity's noun, not its variant identifier: {shown}"
    );
    assert!(
        !shown.contains("angle"),
        "and it is the EXISTING declaration's, not the one asked for: {shown}"
    );

    for refusal in [&io, &edit, &lookup, &kind, &slot, &exists] {
        test_utils::f6::assert_f6(refusal, &[], REFUSAL.identifiers(), REFUSAL_FIELDS);
    }

    // And the one mistake that reaches two doors reaches one recourse:
    // the typed route and the dragged route name the same thing to do.
    // Asserted against the CONST both renderings read, so the clause
    // cannot come back as a second literal without this row reddening.
    let recourse = editor_core::edit::UNDECLARED_PARAM_RECOURSE;
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

/// **`50 mm` sets the value AND the notation, as one undo step.**
///
/// The text door reads both out of one literal — `parse_expr` applies
/// the unit factor once, on the way in — and commits them as one
/// action, so the history gains exactly one state and an undo puts
/// both halves back.
#[test]
fn a_unit_bearing_text_sets_the_value_and_the_notation_as_one_undo() {
    let tol = Tol::witness();
    let name = ParamName::new("base_r");
    let mut session = DocSession::inline(
        common::declared(
            "auth2-written",
            &name,
            DocParam::written_length(WrittenLength::in_unit(20.0, MM)),
        ),
        tol,
    );
    let before = session.history().len();

    let outcome = session.perform(SessionOp::SetParamText {
        name: name.clone(),
        text: "50 mm".to_owned(),
    });
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    assert_eq!(session.history().len(), before + 1, "one undo step");
    assert_eq!(
        outcome.committed.len(),
        1,
        "one action, whatever it is made of"
    );

    let row = param_row(&session, &name);
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
    let row = param_row(&session, &name);
    assert_eq!(row.value, SlotValue::Continuous(0.02));
    assert_eq!(row.unit.map(|u| u.symbol()), Some("mm"));
}

/// **A number that changes only the notation moves only the
/// notation**, and the same text a second time is not an edit at all:
/// the door submits the edits that change something and nothing else.
#[test]
fn text_that_says_what_the_declaration_already_says_is_not_an_edit() {
    let tol = Tol::witness();
    let name = ParamName::new("base_r");
    let mut session = DocSession::inline(
        common::declared(
            "auth2-noop",
            &name,
            DocParam::written_length(WrittenLength::in_unit(50.0, MM)),
        ),
        tol,
    );
    let before = session.history().len();
    let outcome = session.perform(SessionOp::SetParamText {
        name: name.clone(),
        text: "50 mm".to_owned(),
    });
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    assert!(outcome.committed.is_empty(), "nothing changed, so no edit");
    assert_eq!(session.history().len(), before, "and no undo step");

    // The same value, said in another notation: the notation moves and
    // the value does not.
    let outcome = session.perform(SessionOp::SetParamText {
        name: name.clone(),
        text: "0.05 m".to_owned(),
    });
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    assert!(matches!(
        outcome.committed.as_slice(),
        [DocEdit::SetDocParamUnit { .. }]
    ));
    let row = param_row(&session, &name);
    assert_eq!(row.unit.map(|u| u.symbol()), Some("m"));
    assert_eq!(row.value, SlotValue::Continuous(0.05));
}

/// **A document parameter holds a number, not an expression** — the
/// refusal says so, by name, and nothing moves.
///
/// **Two spellings, because the layer that refuses differs.**
/// `base_r * 2` does not reach this door at all: `2` is a count and
/// the expression vocabulary refuses a count times a length without an
/// explicit promotion, so what a user reads there is the parser's
/// sentence about the multiply. `base_r * 2.0` and a bare `base_r`
/// both parse, and those are the texts this door has to answer for.
#[test]
fn an_expression_typed_into_a_parameter_is_refused_with_a_sentence() {
    let tol = Tol::witness();
    let name = ParamName::new("base_r");
    let mut session = DocSession::inline(
        common::declared(
            "auth2-expression",
            &name,
            DocParam::written_length(WrittenLength::in_unit(50.0, MM)),
        ),
        tol,
    );
    let before = session.history().len();
    for text in ["base_r * 2.0", "base_r"] {
        let outcome = session.perform(SessionOp::SetParamText {
            name: name.clone(),
            text: text.to_owned(),
        });
        let refusal = outcome.refusal.expect("a parameter takes no expression");
        assert!(
            matches!(refusal, Refusal::ParamNotANumber { .. }),
            "{text}: {refusal:?}"
        );
        let shown = refusal.to_string();
        assert!(
            shown.contains("holds a number, not an expression"),
            "the sentence says what a parameter is: {shown}"
        );
    }
    // The count-promotion spelling is refused one layer earlier, by
    // the parser, and carries the parser's own sentence.
    let refusal = session
        .perform(SessionOp::SetParamText {
            name: name.clone(),
            text: "base_r * 2".to_owned(),
        })
        .refusal
        .expect("a count times a length needs an explicit promotion");
    assert!(matches!(refusal, Refusal::Parse(_)), "{refusal:?}");
    assert_eq!(session.history().len(), before, "and nothing moved");
    assert_eq!(
        param_row(&session, &name).value,
        SlotValue::Continuous(0.05)
    );
}

/// **An unknown unit carries the parser's own refusal**, which names
/// the token and its offset — not a sentence re-composed at this door.
#[test]
fn an_unknown_unit_carries_the_parsers_own_wording() {
    let tol = Tol::witness();
    let name = ParamName::new("base_r");
    let mut session = DocSession::inline(
        common::declared(
            "auth2-unknown-unit",
            &name,
            DocParam::written_length(WrittenLength::in_unit(50.0, MM)),
        ),
        tol,
    );
    let outcome = session.perform(SessionOp::SetParamText {
        name: name.clone(),
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
        param_row(&session, &name).value,
        SlotValue::Continuous(0.05)
    );
}

/// **A unit that does not measure the declared dimension is refused by
/// the edit door**, in the door's words, and the all-or-nothing action
/// means the value it was paired with does not land either.
#[test]
fn a_wrong_dimension_unit_refuses_the_whole_action() {
    let tol = Tol::witness();
    let name = ParamName::new("sweep");
    let mut session = DocSession::inline(
        common::declared(
            "auth2-mismatch",
            &name,
            DocParam::continuous(pncad::document::Dimension::Angle, 1.0),
        ),
        tol,
    );
    let before = session.history().len();
    let outcome = session.perform(SessionOp::SetParamText {
        name: name.clone(),
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
    let row = param_row(&session, &name);
    assert_eq!(
        row.value,
        SlotValue::Continuous(1.0),
        "and the value the action carried did not land either"
    );
    assert_eq!(row.unit.map(|u| u.symbol()), Some("rad"));
}

/// **The row's unit picker moves the notation and nothing else** — one
/// `SetDocParamUnit`, and the stored value bit-identical.
#[test]
fn the_parameter_unit_picker_leaves_the_value_where_it_was() {
    let tol = Tol::witness();
    let name = ParamName::new("base_r");
    let mut session = DocSession::inline(
        common::declared(
            "auth2-picker",
            &name,
            DocParam::continuous(pncad::document::Dimension::Length, 0.05),
        ),
        tol,
    );
    let outcome = session.perform(SessionOp::SetParamUnit {
        name: name.clone(),
        unit: MM.def(),
    });
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    assert!(matches!(
        outcome.committed.as_slice(),
        [DocEdit::SetDocParamUnit { .. }]
    ));
    let row = param_row(&session, &name);
    assert_eq!(row.unit.map(|u| u.symbol()), Some("mm"));
    assert_eq!(row.value, SlotValue::Continuous(0.05));

    // A count names no notation, and the door says so rather than
    // this one guessing.
    let holes = ParamName::new("holes");
    let mut counted = DocSession::inline(
        common::declared("auth2-picker-count", &holes, DocParam::Count { value: 6 }),
        tol,
    );
    let refusal = counted
        .perform(SessionOp::SetParamUnit {
            name: holes,
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
/// `8 mm` typed into a COUNT parameter's field is a value edit with a
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
    let holes = ParamName::new("holes");
    let mut session = DocSession::inline(
        common::declared("auth2-count-text", &holes, DocParam::Count { value: 6 }),
        tol,
    );
    let before = session.history().len();
    let refusal = session
        .perform(SessionOp::SetParamText {
            name: holes.clone(),
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
    assert_eq!(param_row(&session, &holes).value, SlotValue::Count(6));
}

/// The panel row for `name`, as the panel reads it.
fn param_row(session: &DocSession, name: &ParamName) -> props::ParamRow {
    props::param_rows(session.doc())
        .into_iter()
        .find(|row| &row.name == name)
        .expect("the parameter row")
}

/// **A typed `NaN` or `inf` in a Count slot is refused, by the name
/// the props module promises names it.**
///
/// `props::field_edit` reads `inf` and `NaN` as Numbers deliberately,
/// and says what pays for it: `Expr::literal`'s refusal names the
/// problem where the parser would only say the word is not a
/// parameter. That promise had a hole exactly one dimension wide.
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
    use pncad::document::{Dimension, Expr};

    let literal_door = Expr::literal(f64::NAN, Dimension::Length)
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
