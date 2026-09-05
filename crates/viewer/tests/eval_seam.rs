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

use crate::common;

use std::sync::Arc;

use pncad::document::{CancelToken, EvalOptions, EvalOutcome, ProfileProgram, SlotId, evaluate};
use pncad::geom_core::Tol;
use viewer::evalseam::{
    EvalDone, EvalRequest, EvalService, Generation, IndexDone, IndexRequest, IndexService,
    InlineEvaluator, InlineIndexer,
};
use viewer::props::SlotValue;
use viewer::scene::DisplayTolerance;
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

/// **A canceled run never becomes the picture.**
///
/// The prefix a cancel returns answers a document nobody asked to see
/// half of: rendered, it is a tree of `Unevaluated` rows and a product
/// that gathers to nothing. So the session keeps the last good
/// evaluation, and `busy()` goes on saying the picture is older than
/// the document — with `running()` false, which is the state
/// `Reevaluate` exists to leave.
#[test]
fn a_cancel_keeps_the_last_good_picture_and_reevaluate_recovers_it() {
    let tol = Tol::witness();
    let (doc, _profile, _extrude) = common::parametric_plate(tol);
    let mut session = DocSession::inline(doc, tol);
    session.pump();
    let good = Arc::clone(session.evaluation_arc().expect("the first result landed"));

    session.perform(SessionOp::SetParam {
        name: common::thickness_param(),
        value: SlotValue::Continuous(0.010),
    });
    session.perform(SessionOp::CancelEvaluation);
    assert_eq!(session.pump(), vec![Landing::Canceled]);

    assert!(session.busy(), "the picture is older than the document");
    assert!(!session.running(), "and nothing is working on it");
    assert!(
        Arc::ptr_eq(
            session.evaluation_arc().expect("a picture is still shown"),
            &good
        ),
        "the canceled prefix did not replace the last good evaluation"
    );
    assert!(
        !viewer::tree::has_faults(&session.tree_rows()),
        "the tree still shows the good run, not a blank one"
    );

    // The recovery op: ask again, and the picture comes back.
    session.perform(SessionOp::Reevaluate);
    assert!(session.running());
    assert_eq!(session.pump(), vec![Landing::Landed]);
    assert!(!session.busy());
    assert!(!session.running());
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
        resolver: None,
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
        resolver: None,
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
        resolver: None,
    });
    let second = Generation::FIRST.next();
    seam.submit(EvalRequest {
        generation: second,
        doc,
        tol,
        resolver: None,
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
        resolver: None,
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
        resolver: None,
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
        resolver: None,
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
    let mut seam = viewer::evalseam::ThreadEvaluator::spawn().expect("the worker starts");
    seam.submit(EvalRequest {
        generation: Generation::FIRST,
        doc,
        tol,
        resolver: None,
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
/// **The threaded lane coalesces too** — the property the module doc
/// states for the seam and not for one implementation of it.
///
/// The inline row above proves it for the seam the tests drive; this
/// proves it for the seam the APPLICATION drives, which is the lane
/// where "N submits queue N jobs" would actually have cost work. Two
/// submits, one result, carrying the newer generation: the superseded
/// request dies inside the seam rather than travelling up to be
/// discarded by generation.
#[cfg(not(target_family = "wasm"))]
#[test]
fn the_threaded_seam_coalesces_two_submits_into_one_result() {
    let tol = Tol::witness();
    let (doc, _profile, _extrude) = common::parametric_plate(tol);
    let mut seam = viewer::ThreadEvaluator::spawn().expect("the worker starts");
    seam.submit(EvalRequest {
        generation: Generation::FIRST,
        doc: doc.clone(),
        tol,
        resolver: None,
    });
    let second = Generation::FIRST.next();
    seam.submit(EvalRequest {
        generation: second,
        doc,
        tol,
        resolver: None,
    });

    let mut results = Vec::new();
    for _ in 0..10_000 {
        while let Some(done) = seam.poll() {
            results.push(done.generation);
        }
        if !seam.busy() {
            break;
        }
        std::thread::sleep(std::time::Duration::from_millis(1));
    }
    assert_eq!(
        results,
        vec![second],
        "two submits must answer once, for the newer request"
    );
    assert!(!seam.busy(), "and the seam is idle afterwards");
}

/// **A cancel raised against a threaded seam with a job WAITING** —
/// the window the per-job token exists for, and the one no row reached
/// before.
///
/// `cancel()` names the newest submitted job's token whether that job
/// is running or waiting, so the cancelation cannot be lost in the
/// hand-off between the two. What comes back is a `Canceled` outcome
/// (or nothing, if the seam had already finished) — never a completed
/// run for a request the user stopped.
#[cfg(not(target_family = "wasm"))]
#[test]
fn a_cancel_reaches_a_threaded_seams_waiting_job() {
    let tol = Tol::witness();
    let (doc, _profile, _extrude) = common::parametric_plate(tol);
    let mut seam = viewer::ThreadEvaluator::spawn().expect("the worker starts");
    seam.submit(EvalRequest {
        generation: Generation::FIRST,
        doc: doc.clone(),
        tol,
        resolver: None,
    });
    let second = Generation::FIRST.next();
    seam.submit(EvalRequest {
        generation: second,
        doc,
        tol,
        resolver: None,
    });
    // The second job is waiting behind the first; cancel names it.
    seam.cancel();

    let mut results = Vec::new();
    for _ in 0..10_000 {
        while let Some(done) = seam.poll() {
            results.push(done);
        }
        if !seam.busy() {
            break;
        }
        std::thread::sleep(std::time::Duration::from_millis(1));
    }
    assert!(!seam.busy());
    for done in &results {
        assert_eq!(
            done.generation, second,
            "only the newest request is ever answered"
        );
        assert!(
            !done.completed(),
            "a canceled job must not answer as a completed run"
        );
    }
}

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

// --- the index seam -------------------------------------------------

/// The δ the plate indexes at in this suite. Coarse on purpose: these
/// rows are about the seam's bookkeeping, not about tessellation.
fn index_delta() -> DisplayTolerance {
    DisplayTolerance::new(2.0e-4).expect("a positive delta")
}

fn index_request(session: &DocSession, generation: Generation) -> IndexRequest {
    let (doc, _) = session.landed_pair().expect("a landed pair");
    IndexRequest {
        generation,
        delta: index_delta(),
        doc: doc.clone(),
        evaluation: Arc::clone(session.evaluation_arc().expect("a landed run")),
        tol: session.tol(),
    }
}

/// **The index seam answers with the key it was asked with**, and the
/// index it carries was built under that same generation — the pair a
/// consumer matches against its own request.
#[test]
fn the_index_seam_answers_with_the_key_it_was_asked_with() {
    let tol = Tol::witness();
    let (doc, _profile, _extrude) = common::parametric_plate(tol);
    let mut session = DocSession::inline(doc, tol);
    session.pump();
    let generation = session.landed_generation().expect("a landed generation");

    let mut seam = InlineIndexer::new();
    assert!(!seam.busy());
    seam.submit(index_request(&session, generation));
    assert!(seam.busy(), "asked, and not yet answered");
    let done = seam.poll().expect("the inline seam answers inside poll");
    assert!(!seam.busy());
    assert_eq!(done.generation, generation);
    assert_eq!(done.delta, index_delta());
    let index = done.index.expect("the plate indexes");
    assert_eq!(
        index.generation(),
        generation,
        "the index is stamped with the generation the answer is filed under",
    );
    assert!(index.current_for(Some(generation), index_delta()));
    assert!(seam.poll().is_none(), "and there is nothing else to take");
}

/// **Restart without cancel, and the wasted build is real.** A submit
/// while a build is in flight does not stop it; the seam lets it
/// finish and drops its answer, so two submits produce exactly one
/// result and it is the newer one. This is the promise the evaluation
/// seam above makes STRONGER: there, the superseded run is stopped at
/// its next node.
#[cfg(not(target_family = "wasm"))]
#[test]
fn the_threaded_index_seam_restarts_without_canceling_and_answers_once() {
    let tol = Tol::witness();
    let (doc, _profile, _extrude) = common::parametric_plate(tol);
    let mut session = DocSession::inline(doc, tol);
    session.pump();
    let first = session.landed_generation().expect("a landed generation");
    let second = first.next();

    let mut seam = viewer::evalseam::ThreadIndexer::spawn().expect("the worker starts");
    seam.submit(index_request(&session, first));
    seam.submit(index_request(&session, second));

    let mut results = Vec::new();
    for _ in 0..10_000 {
        while let Some(done) = seam.poll() {
            results.push(done);
        }
        if !seam.busy() && !results.is_empty() {
            break;
        }
        std::thread::sleep(std::time::Duration::from_millis(1));
    }
    assert!(!seam.busy());
    assert_eq!(
        results.len(),
        1,
        "the superseded build dies inside the seam rather than travelling \
         up to be discarded by key",
    );
    assert_eq!(results[0].generation, second);
    assert!(results[0].index.is_ok());
}

/// **A δ moved away and back does not pay for a second build of an
/// answer already in hand.** The waiting request and the finished one
/// name the same picture, so the finished one IS the answer: dropping
/// it by position rather than by key would dispatch an identical
/// build, and on the fine-δ row that is thirteen seconds for nothing.
///
/// **How the row can tell which build answered.** Two builds of one
/// key are indistinguishable by their results — which is the whole
/// difficulty — so the waiting request here carries a BROKEN document
/// under the key the worker is already building the good one for.
/// Production never mints two payloads for one key; this row does, so
/// that "the seam kept the answer it had" and "the seam rebuilt" have
/// different observable answers instead of the same one.
#[cfg(not(target_family = "wasm"))]
#[test]
fn the_threaded_index_seam_keeps_an_answer_a_waiting_request_asks_for() {
    let tol = Tol::witness();
    let (doc, extrude) = viewer::scene::plate_with_hole(tol).expect("the plate authors");
    let mut session = DocSession::inline(doc.clone(), tol);
    session.pump();
    let generation = session.landed_generation().expect("a landed generation");

    let mut broken = DocSession::inline(doc, tol);
    broken.perform(SessionOp::SetSlot {
        node: extrude,
        slot: SlotId::Distance,
        value: SlotValue::Continuous(0.0),
    });
    broken.pump();

    let mut seam = viewer::evalseam::ThreadIndexer::spawn().expect("the worker starts");
    seam.submit(index_request(&session, generation));
    // A second submit while the first is with the worker, so the third
    // is only WAITING rather than dispatched — and the third asks for
    // the picture the worker is already building.
    let mut other = index_request(&session, generation);
    other.delta = index_delta().scaled(2.0).expect("a positive delta");
    seam.submit(other);
    seam.submit(index_request(&broken, generation));

    let mut results = Vec::new();
    for _ in 0..10_000 {
        while let Some(done) = seam.poll() {
            results.push(done);
        }
        if !seam.busy() && !results.is_empty() {
            break;
        }
        std::thread::sleep(std::time::Duration::from_millis(1));
    }
    assert!(!seam.busy());
    assert_eq!(results.len(), 1, "one answer for one picture");
    assert_eq!(results[0].generation, generation);
    assert_eq!(results[0].delta, index_delta());
    assert!(
        results[0].index.is_ok(),
        "the answer in hand was kept, not thrown away and rebuilt",
    );
}

/// The index seam's traffic is `Send` too — checked here as well as by
/// the compile-time assertion in the module, because the threaded
/// implementation that would otherwise force it is absent on wasm.
#[test]
fn the_index_seams_traffic_is_send() {
    fn assert_send<T: Send>() {}
    assert_send::<IndexRequest>();
    assert_send::<IndexDone>();
    assert_send::<viewer::PickIndex>();
}
