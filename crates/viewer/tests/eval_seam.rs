//! **The evaluation seam, without a thread in the assertions.**
//!
//! What the seam owes the layer above it is four things, and each has
//! a row here: a busy state the chrome can read as a value, a
//! cancelation that reaches the shipped `CancelToken`, results that
//! land by generation with stale ones discarded, and a memo that makes
//! the second evaluation of an edited document cheaper than the first.
//!
//! Every row drives [`InlineEvaluator`], which runs the evaluation
//! inside `poll`. That is not a weaker test of the seam — it is the
//! seam's other implementation, the one the browser will use, and the
//! fact that these assertions hold against it is the evidence that
//! nothing above the boundary assumes a thread.

// Panicking is a test's failure mechanism (workspace lint note).
#![allow(clippy::expect_used)]
#![allow(clippy::panic)]

mod common;

use std::sync::Arc;

use pncad::document::{CancelToken, EvalOptions, EvalOutcome, ProfileProgram, SlotId, evaluate};
use pncad::geom_core::Tol;
use viewer::evalseam::{EvalDone, EvalRequest, EvalService, Generation, InlineEvaluator};
use viewer::props::SlotValue;
use viewer::session::{DocSession, Landing, SessionOp};

#[test]
fn busy_is_a_value_the_chrome_reads_and_it_clears_when_the_result_lands() {
    let tol = Tol::witness();
    let (doc, _profile, _extrude) = common::parametric_plate(tol);
    let mut session = DocSession::inline(doc, tol);

    assert!(session.busy(), "the first evaluation is outstanding");
    assert!(session.evaluation().is_none());
    assert_eq!(session.pump(), vec![Landing::Landed]);
    assert!(!session.busy());
    assert!(session.evaluation().is_some());

    // An edit makes it busy again, and only the pump clears it.
    session.perform(SessionOp::SetParam {
        name: common::thickness_param(),
        value: SlotValue::Continuous(0.010),
    });
    assert!(session.busy());
    session.pump();
    assert!(!session.busy());
}

#[test]
fn a_stale_result_is_discarded_by_generation() {
    let tol = Tol::witness();
    let (doc, _profile, _extrude) = common::parametric_plate(tol);
    let mut session = DocSession::inline(doc, tol);
    session.pump();
    let landed = Arc::clone(session.evaluation_arc().expect("the first result landed"));

    // Move the session two documents on, then hand it a result for
    // the generation it started at. The seam cannot produce this
    // ordering by itself; the rule that rejects it is a comparison of
    // two integers, and this is that rule under test.
    session.perform(SessionOp::SetParam {
        name: common::thickness_param(),
        value: SlotValue::Continuous(0.010),
    });
    let current = session.generation();
    assert_eq!(
        session.land(EvalDone {
            generation: Generation::FIRST,
            evaluation: Arc::clone(&landed),
        }),
        Landing::Stale
    );
    assert!(session.busy(), "a discarded result does not clear busy");
    assert_eq!(
        session.land(EvalDone {
            generation: current,
            evaluation: landed,
        }),
        Landing::Landed
    );
    assert!(!session.busy());
}

#[test]
fn cancel_reaches_the_shipped_token_and_the_prefix_is_typed_canceled() {
    let tol = Tol::witness();
    let (doc, _profile, _extrude) = common::parametric_plate(tol);
    let mut seam = InlineEvaluator::new();
    seam.submit(EvalRequest {
        generation: Generation::FIRST,
        doc: doc.clone(),
        tol,
    });
    assert!(seam.busy());
    seam.cancel();
    let done = seam.poll().expect("a canceled run still answers");
    assert!(!seam.busy());
    assert_eq!(
        done.evaluation.outcome,
        EvalOutcome::Canceled,
        "the cancelation is the shipped token's, reported as the shipped outcome"
    );
    assert!(!done.completed());
    assert!(
        done.evaluation.nodes.is_empty(),
        "a run canceled before its first node returns an empty prefix"
    );

    // And the seam recovers: the next request evaluates normally.
    seam.submit(EvalRequest {
        generation: Generation::FIRST.next(),
        doc,
        tol,
    });
    let done = seam.poll().expect("the next run answers");
    assert!(done.completed());
}

#[test]
fn an_edit_during_an_evaluation_cancels_and_restarts_rather_than_queueing() {
    let tol = Tol::witness();
    let (doc, _profile, _extrude) = common::parametric_plate(tol);
    let mut seam = InlineEvaluator::new();
    seam.submit(EvalRequest {
        generation: Generation::FIRST,
        doc: doc.clone(),
        tol,
    });
    let second = Generation::FIRST.next();
    seam.submit(EvalRequest {
        generation: second,
        doc,
        tol,
    });
    let done = seam.poll().expect("a result");
    assert_eq!(
        done.generation, second,
        "the newer document replaced the older request"
    );
    assert!(
        seam.poll().is_none(),
        "the superseded request produced no second result to discard"
    );
}

#[test]
fn the_memo_makes_an_edited_documents_re_evaluation_incremental() {
    let tol = Tol::witness();
    let (doc, _profile, extrude) = common::parametric_plate(tol);
    let mut seam = InlineEvaluator::new();
    seam.submit(EvalRequest {
        generation: Generation::FIRST,
        doc: doc.clone(),
        tol,
    });
    let first = seam.poll().expect("a result");
    assert!(first.completed());
    assert_eq!(first.evaluation.reused, 0, "nothing to reuse on a cold run");

    // Edit the LAST node only: the profile below it is unchanged, so
    // its content key matches and the memo reuses it.
    let edited = pncad::document::apply(
        &doc,
        &pncad::document::DocEdit::SetParam {
            node: extrude,
            slot: SlotId::Distance,
            expr: common::len(0.02),
        },
        tol,
    )
    .expect("the edit applies")
    .doc;
    seam.submit(EvalRequest {
        generation: Generation::FIRST.next(),
        doc: edited,
        tol,
    });
    let second = seam.poll().expect("a result");
    assert!(second.completed());
    assert!(
        second.evaluation.reused > 0,
        "the unchanged prefix was reused, not recomputed"
    );
    assert!(
        second.evaluation.recomputed < second.evaluation.order.len(),
        "an edit recomputes its downstream cone, not the document"
    );
}

#[test]
fn the_seams_result_agrees_with_a_direct_evaluation() {
    // The seam is plumbing, and plumbing that quietly evaluated
    // something else would pass every row above. This one pins it to
    // the door an ordinary consumer would call.
    let tol = Tol::witness();
    let (doc, _profile, extrude) = common::parametric_plate(tol);
    let mut seam = InlineEvaluator::new();
    seam.submit(EvalRequest {
        generation: Generation::FIRST,
        doc: doc.clone(),
        tol,
    });
    let through_seam = seam.poll().expect("a result");
    let direct = evaluate::<f64>(
        &doc,
        None,
        &CancelToken::new(),
        &EvalOptions::default(),
        tol,
    );
    assert_eq!(through_seam.evaluation.order, direct.order);
    assert_eq!(through_seam.evaluation.outcome, direct.outcome);
    assert!(
        direct.value(extrude).is_some() && through_seam.evaluation.value(extrude).is_some(),
        "both evaluations produced the extrude's body"
    );
}

#[cfg(not(target_family = "wasm"))]
#[test]
fn the_threaded_seam_answers_the_same_generations() {
    // The native implementation of the same trait. The row is
    // deliberately about the CONTRACT — a result per submitted
    // generation — and not about timing: it polls until the answer
    // arrives rather than asserting when it does.
    let tol = Tol::witness();
    let (doc, _profile, _extrude) = common::parametric_plate(tol);
    let mut seam = viewer::evalseam::ThreadEvaluator::spawn();
    seam.submit(EvalRequest {
        generation: Generation::FIRST,
        doc,
        tol,
    });
    let mut done: Option<EvalDone> = None;
    for _ in 0..10_000 {
        if let Some(result) = seam.poll() {
            done = Some(result);
            break;
        }
        std::thread::sleep(std::time::Duration::from_millis(1));
    }
    let done = done.expect("the worker answered");
    assert_eq!(done.generation, Generation::FIRST);
    assert!(done.completed());
    assert!(!seam.busy());
}

/// The seam never sends anything but values across the boundary — the
/// property that makes a Worker-backed sibling possible without
/// changing a line above it.
#[test]
fn the_seams_traffic_is_send() {
    fn assert_send<T: Send>() {}
    assert_send::<EvalRequest>();
    assert_send::<EvalDone>();
    assert_send::<Arc<pncad::document::Evaluation<f64>>>();
    assert_send::<pncad::document::Doc<ProfileProgram>>();
}
