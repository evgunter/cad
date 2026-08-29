//! **R1 review probes for M10-1, GUI half**: the carry-forward class.
//!
//! `DocEdit::SetDocParam` is create-or-replace, so every GUI door that
//! rebuilds a `DocParam` from parts is a door that can silently DELETE
//! an existing distribution. The PR fixed `props::param_edit` and
//! reported the fix in prose; nothing in the tree pinned it. These
//! rows pin BOTH value-edit doors — the panel (`SetParam`) and the
//! drag gesture (`BeginParamGesture` → preview → commit) — against
//! that deletion, and the create door against dropping an annotation
//! it was handed.

#![allow(clippy::expect_used)]
#![allow(clippy::panic)]

mod common;

use pncad::document::{Dimension, Distribution, DocParam, ParamName};
use pncad::geom_core::Tol;
use viewer::props::SlotValue;
use viewer::session::{DocSession, SessionOp};

fn annotated_session() -> (DocSession, ParamName, Distribution) {
    let tol = Tol::witness();
    let (doc, _profile, _extrude) = common::parametric_plate(tol);
    let mut session = DocSession::inline(doc, tol);
    let name = ParamName::new("bore_r");
    let dist = Distribution::Normal { sigma: 5e-6 };
    let outcome = session.perform(SessionOp::CreateParam {
        name: name.clone(),
        value: DocParam::continuous_with(Dimension::Length, 0.004, dist),
    });
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    (session, name, dist)
}

fn distribution_of(session: &DocSession, name: &ParamName) -> Option<Distribution> {
    session
        .committed_doc()
        .params()
        .get(name)
        .expect("the parameter exists")
        .distribution()
        .copied()
}

/// The create door carries an annotation it was handed.
#[test]
fn create_param_carries_an_annotation() {
    let (session, name, dist) = annotated_session();
    let got = distribution_of(&session, &name).expect("annotated at creation");
    assert!(got.bit_eq(&dist));
}

/// **The reported defect, pinned**: a panel VALUE edit on an annotated
/// parameter changes the value and keeps the distribution.
#[test]
fn a_panel_value_edit_keeps_the_distribution() {
    let (mut session, name, dist) = annotated_session();
    let outcome = session.perform(SessionOp::SetParam {
        name: name.clone(),
        value: SlotValue::Continuous(0.005),
    });
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    let param = session
        .committed_doc()
        .params()
        .get(&name)
        .expect("still declared");
    match *param {
        DocParam::Continuous {
            value,
            distribution,
            ..
        } => {
            assert_eq!(value, 0.005, "the value moved");
            let got = distribution.expect("the annotation SURVIVED the value edit");
            assert!(got.bit_eq(&dist), "and survived bit for bit");
        }
        DocParam::Count { .. } => panic!("still continuous"),
    }
}

/// The same class through the OTHER value door: a drag gesture
/// (preview + commit) on an annotated parameter keeps the
/// distribution.
#[test]
fn a_param_drag_gesture_keeps_the_distribution() {
    let (mut session, name, dist) = annotated_session();
    let outcome = session.perform(SessionOp::BeginParamGesture { name: name.clone() });
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    let outcome = session.perform(SessionOp::PreviewGesture { value: 0.006 });
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    let outcome = session.perform(SessionOp::CommitGesture);
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    let got = distribution_of(&session, &name).expect("the annotation survived the gesture");
    assert!(got.bit_eq(&dist));
    match *session
        .committed_doc()
        .params()
        .get(&name)
        .expect("declared")
    {
        DocParam::Continuous { value, .. } => assert_eq!(value, 0.006, "the drag landed"),
        DocParam::Count { .. } => panic!("still continuous"),
    }
}
