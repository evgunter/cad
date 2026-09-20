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
    EvalDone, EvalRequest, EvalService, FitDone, FitRequest, FitService, FitSubject, IndexDone,
    IndexRequest, IndexService, InlineEvaluator, InlineFitter, InlineIndexer,
};
use viewer::generation::Generation;
use viewer::pickindex::PictureKey;
use viewer::props::SlotValue;
use viewer::scene::DisplayTolerance;
use viewer::session::{DocSession, Landing, Outstanding, SessionOp};

/// The two doors every threaded row below needs, and the only two the
/// three seam traits share.
///
/// [`EvalService`], [`IndexService`] and [`FitService`] have no common
/// supertrait on purpose — each offers only the doors its own seam can
/// honestly answer, which is why the index and fit seams have no
/// `cancel` — so a harness that drives all three names the two doors
/// they do share and nothing else.
#[cfg(not(target_family = "wasm"))]
trait Drainable {
    /// What one answer is.
    type Done;
    /// [`EvalService::poll`] and its two siblings.
    fn take(&mut self) -> Option<Self::Done>;
    /// [`EvalService::busy`] and its two siblings.
    fn working(&self) -> bool;
}

#[cfg(not(target_family = "wasm"))]
macro_rules! drainable {
    ($seam:ty, $done:ty) => {
        impl Drainable for $seam {
            type Done = $done;
            fn take(&mut self) -> Option<$done> {
                self.poll()
            }
            fn working(&self) -> bool {
                self.busy()
            }
        }
    };
}

#[cfg(not(target_family = "wasm"))]
drainable!(viewer::evalseam::ThreadEvaluator, EvalDone);
#[cfg(not(target_family = "wasm"))]
drainable!(viewer::evalseam::ThreadIndexer, IndexDone);
#[cfg(not(target_family = "wasm"))]
drainable!(viewer::evalseam::ThreadFitter, FitDone);

/// Poll `seam` to a standstill: drain every answer it has, and stop
/// once it is idle holding at least `at_least` of them.
///
/// **Written once for every threaded row in this file**, because a
/// per-row copy of a spin loop is a per-row chance to spin on the
/// wrong condition — and because what the loop is FOR is one sentence
/// that belongs in one place: a threaded seam answers when its worker
/// does, so a row either waits or asserts about a race.
///
/// Ten thousand millisecond naps is a ceiling and not a schedule. It is
/// long enough that a loaded box does not fail the row, and finite so
/// that a seam which never answers fails the row instead of hanging the
/// suite. `at_least` is what separates a row that must see an answer
/// from one whose seam is allowed to have none: a cancel can leave a
/// seam idle with nothing to hand back, and `0` says so.
#[cfg(not(target_family = "wasm"))]
fn drained<S: Drainable>(seam: &mut S, at_least: usize) -> Vec<S::Done> {
    let mut results = Vec::new();
    for _ in 0..10_000 {
        while let Some(done) = seam.take() {
            results.push(done);
        }
        if !seam.working() && results.len() >= at_least {
            return results;
        }
        std::thread::sleep(std::time::Duration::from_millis(1));
    }
    results
}

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
///
/// That pair is `Outstanding::Canceled`, and this row is where the fold
/// from the two reads to the one value is covered.
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
    assert_eq!(
        session.outstanding(),
        Outstanding::Evaluating,
        "the edit is submitted and the seam has it",
    );
    session.perform(SessionOp::CancelEvaluation);
    assert_eq!(session.pump(), vec![Landing::Canceled]);

    assert!(session.busy(), "the picture is older than the document");
    assert!(!session.running(), "and nothing is working on it");
    assert_eq!(
        session.outstanding(),
        Outstanding::Canceled,
        "and the one value the chrome reads says so without being told \
         which of the two answers came first",
    );
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
    assert_eq!(session.outstanding(), Outstanding::Evaluating);
    assert_eq!(session.pump(), vec![Landing::Landed]);
    assert!(!session.busy());
    assert!(!session.running());
    assert_eq!(session.outstanding(), Outstanding::Current);
}

/// An [`InlineEvaluator`] that never reports itself idle.
///
/// `DocSession` holds a `Box<dyn EvalService>`, so a seam is free to
/// say it has work when the session's own generations say the picture
/// is current. Both shipped seams cannot reach that combination, which
/// is exactly why the answer for it needs a seam written to.
struct NeverIdle(InlineEvaluator);

impl EvalService for NeverIdle {
    fn submit(&mut self, request: EvalRequest) {
        self.0.submit(request);
    }

    fn cancel(&mut self) {
        self.0.cancel();
    }

    fn poll(&mut self) -> Option<EvalDone> {
        self.0.poll()
    }

    fn busy(&self) -> bool {
        true
    }
}

/// **The eighth combination is answered, not merely commented.**
///
/// `!busy() && running()` — the picture current while the seam claims
/// work — is unreachable through both shipped seams, so it is the one
/// point of `outstanding()`'s domain no ordinary session reaches. The
/// mapping is total regardless; what this row buys is that the answer
/// is executed rather than asserted only by a doc comment, which is
/// where the tree's statement about this case used to live before the
/// case moved down here.
#[test]
fn a_current_picture_reads_current_even_when_the_seam_claims_work() {
    let tol = Tol::witness();
    let (doc, _profile, _extrude) = common::parametric_plate(tol);
    let mut session = DocSession::new(doc, tol, Box::new(NeverIdle(InlineEvaluator::new())));
    session.pump();

    assert!(!session.busy(), "the first result landed");
    assert!(
        session.running(),
        "and this seam reports work outstanding anyway",
    );
    assert_eq!(
        session.outstanding(),
        Outstanding::Current,
        "the picture is what the chrome describes, so a current picture \
         is Current whatever the seam says about itself",
    );
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
        &pncad::document::RefusingReach,
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
    let mut results = drained(&mut seam, 1);
    assert_eq!(results.len(), 1, "one submit is answered once");
    let done = results.remove(0);
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

    let results: Vec<Generation> = drained(&mut seam, 0)
        .into_iter()
        .map(|done| done.generation)
        .collect();
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

    // Zero, not one: a cancel may leave the seam idle with nothing to
    // hand back, which is one of the two answers this row accepts.
    let results = drained(&mut seam, 0);
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

fn index_request(session: &DocSession, generation: Generation) -> IndexRequest {
    let (doc, _) = session.landed_pair().expect("a landed pair");
    IndexRequest {
        key: PictureKey::of(generation, common::plate_delta()),
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
    assert_eq!(done.key, PictureKey::of(generation, common::plate_delta()));
    let index = done.index.expect("the plate indexes");
    assert_eq!(
        index.generation(),
        generation,
        "the index is stamped with the generation the answer is filed under",
    );
    assert!(index.current_for(Some(PictureKey::of(generation, common::plate_delta()))));
    assert!(seam.poll().is_none(), "and there is nothing else to take");
}

/// **Two submits, one answer, and it is the newer one.**
///
/// Named for what it measures. The row does NOT measure that the
/// superseded build ran to completion — a seam that cancelled the
/// first build would satisfy every assertion here identically — and
/// nothing needs to: "restart without cancel" is structural rather
/// than behavioural, because [`IndexService`] offers no cancel door
/// for anything to call. What is left to check is that the caller
/// sees one answer for its latest ask, which is the part a queue or a
/// lost `waiting` slot would break.
#[cfg(not(target_family = "wasm"))]
#[test]
fn the_threaded_index_seam_answers_only_the_newest_of_two_submits() {
    let tol = Tol::witness();
    let (doc, _profile, _extrude) = common::parametric_plate(tol);
    let mut session = DocSession::inline(doc, tol);
    session.pump();
    let first = session.landed_generation().expect("a landed generation");
    let second = first.next();

    let mut seam = viewer::evalseam::ThreadIndexer::spawn().expect("the worker starts");
    seam.submit(index_request(&session, first));
    seam.submit(index_request(&session, second));

    let results = drained(&mut seam, 1);
    assert!(!seam.busy());
    assert_eq!(
        results.len(),
        1,
        "the superseded build dies inside the seam rather than travelling \
         up to be discarded by key",
    );
    assert_eq!(results[0].key.generation(), second);
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
    // **The premise, asserted where it is used.** The discrimination
    // below is `Ok` against `Err`, so a broken document that started
    // indexing cleanly would leave the row passing in both directions
    // instead of going red. It is checked here rather than inherited
    // from a row in another file.
    let mut probe = InlineIndexer::new();
    probe.submit(index_request(&broken, generation));
    assert!(
        probe
            .poll()
            .expect("the inline seam answers inside poll")
            .index
            .is_err(),
        "a zero-distance extrude must refuse to index, or this row \
         measures nothing",
    );

    let mut seam = viewer::evalseam::ThreadIndexer::spawn().expect("the worker starts");
    seam.submit(index_request(&session, generation));
    // A second submit while the first is with the worker, so the third
    // is only WAITING rather than dispatched — and the third asks for
    // the picture the worker is already building.
    let mut other = index_request(&session, generation);
    other.key = PictureKey::of(
        generation,
        common::plate_delta().scaled(2.0).expect("a positive delta"),
    );
    seam.submit(other);
    seam.submit(index_request(&broken, generation));

    let results = drained(&mut seam, 1);
    assert!(!seam.busy());
    assert_eq!(results.len(), 1, "one answer for one picture");
    assert_eq!(
        results[0].key,
        PictureKey::of(generation, common::plate_delta())
    );
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

// --- the display budget's fit seam ----------------------------------

/// The δ this suite's fit rows ask for. Finer than the plate needs, so
/// the ladder has somewhere to descend from rather than answering off
/// its first rung.
fn fit_delta_request() -> DisplayTolerance {
    DisplayTolerance::new(1.0e-5).expect("a positive delta")
}

/// **The fit seam answers with the key it was asked with, and answers
/// what the function it wraps answers.**
///
/// The second half is the one that matters here: moving the ladder
/// behind a seam moved the WORK and must not have moved the NUMBER, so
/// the row prices the same body twice — once through the seam, once
/// through `scene::fit_delta` directly — and compares.
#[test]
fn the_fit_seam_answers_with_the_key_it_was_asked_with() {
    let tol = Tol::witness();
    let (doc, _profile, _extrude) = common::parametric_plate(tol);
    let mut session = DocSession::inline(doc, tol);
    session.pump();
    let generation = session.landed_generation().expect("a landed generation");
    let body = session.landed_body().expect("the plate gathers");
    let direct = viewer::scene::fit_delta(body, fit_delta_request(), tol).expect("the plate fits");

    let mut seam = InlineFitter::new();
    assert!(!seam.busy());
    seam.submit(
        session
            .fit_request(fit_delta_request())
            .expect("a landing with a body to price"),
    );
    assert!(seam.busy(), "asked, and not yet answered");
    let done = seam.poll().expect("the inline seam answers inside poll");
    assert!(!seam.busy());
    assert_eq!(done.generation, generation);
    assert_eq!(done.requested, fit_delta_request());
    let fitted = done.fit.expect("the plate fits behind the seam too");
    assert_eq!(
        fitted.delta, direct.delta,
        "the seam's answer is the function's answer",
    );
    assert_eq!(fitted.predicted, direct.predicted);
    assert_eq!(fitted.probe_triangles, direct.probe_triangles);
    assert!(seam.poll().is_none(), "and there is nothing else to take");
}

/// **The gathering arm gathers behind the seam, and answers the same
/// δ.**
///
/// [`FitSubject::Ungathered`] is the landing shape with no body to
/// share — an assembly whose A5 gate consumed the product it judged —
/// and the point of the arm is that the gather it needs is paid on the
/// worker rather than on the frame. Driven here by handing the seam
/// the pair directly, because what is under test is the arm, not the
/// landing that produces it.
#[test]
fn the_fit_seams_gathering_arm_answers_what_the_shared_body_answers() {
    let tol = Tol::witness();
    let (doc, _profile, _extrude) = common::parametric_plate(tol);
    let mut session = DocSession::inline(doc, tol);
    session.pump();
    let generation = session.landed_generation().expect("a landed generation");
    let shared = session
        .fit_request(fit_delta_request())
        .expect("a landing with a body to price");
    assert!(
        matches!(shared.subject, FitSubject::Landed(_)),
        "a part document's landing keeps its body, or this row's two \
         arms are the same arm",
    );
    let (pair_doc, _) = session.landed_pair().expect("a landed pair");
    let gathering = FitRequest {
        generation,
        requested: fit_delta_request(),
        subject: FitSubject::Ungathered {
            doc: Arc::new(pair_doc.clone()),
            evaluation: Arc::clone(session.evaluation_arc().expect("a landed run")),
        },
        tol,
    };

    let mut seam = InlineFitter::new();
    seam.submit(shared);
    let from_body = seam.poll().expect("the seam answers inside poll");
    seam.submit(gathering);
    let from_pair = seam.poll().expect("the seam answers inside poll");
    assert_eq!(
        from_body.fit.expect("the shared body fits").delta,
        from_pair.fit.expect("the gathered body fits").delta,
        "the arm decides who gathers, not what the answer is",
    );
}

/// **Two submits, one answer, and it is the newer one** — the index
/// seam's row over this seam, for its reason: the fit has no cancel
/// either, so what is left to check is that the caller sees one answer
/// for its latest ask.
#[cfg(not(target_family = "wasm"))]
#[test]
fn the_threaded_fit_seam_answers_only_the_newest_of_two_submits() {
    let tol = Tol::witness();
    let (doc, _profile, _extrude) = common::parametric_plate(tol);
    let mut session = DocSession::inline(doc, tol);
    session.pump();
    let mut seam = viewer::evalseam::ThreadFitter::spawn().expect("the worker starts");
    seam.submit(
        session
            .fit_request(common::plate_delta())
            .expect("a landing to price"),
    );
    seam.submit(
        session
            .fit_request(fit_delta_request())
            .expect("a landing to price"),
    );

    let results: Vec<FitDone> = drained(&mut seam, 1);
    assert!(!seam.busy());
    assert_eq!(results.len(), 1, "one answer for one ask");
    assert_eq!(
        results[0].requested,
        fit_delta_request(),
        "and it is the newest ask, not the first",
    );
}

/// The fit seam's traffic is `Send` too — checked here as well as by
/// the compile-time assertion in the module, because the threaded
/// implementation that would otherwise force it is absent on wasm.
#[test]
fn the_fit_seams_traffic_is_send() {
    fn assert_send<T: Send>() {}
    assert_send::<FitRequest>();
    assert_send::<FitDone>();
    assert_send::<FitSubject>();
}

/// **A panic raised inside an egui frame is not swallowed** — the fact
/// the crash ruling rests on, executed rather than assumed.
///
/// A crashed worker is announced by panicking on the UI thread, at the
/// point of detection (`evalseam`'s coalescing machine). That site is
/// inside `<ViewerApp as eframe::App>::ui`, which runs inside
/// `egui::Context::run`, which runs inside eframe's winit event loop.
/// **If anything up that stack caught the unwind, the loudest thing
/// this crate does would be a no-op** — strictly worse than the silence
/// it replaced, because the loudness would be a lie.
///
/// This row executes the layer nearest the panic: it plants one inside
/// a panel closure and asserts the unwind leaves `Context::run` rather
/// than being absorbed by egui's own frame bookkeeping. **What it does
/// NOT execute** is eframe and winit, which have no headless door here;
/// those were established by reading, at the pinned versions the
/// manifest names: `egui`, `eframe`, `egui-winit` and `egui-wgpu`
/// 0.36.1 contain no `catch_unwind` at all (eframe's only panic
/// machinery is `web/panic_handler.rs`, a `set_hook` on the wasm
/// build), and `winit` 0.30.13 has none on the linux backends this
/// crate builds against — it catches on macOS and Windows only, and
/// both re-raise (`macos/event_loop.rs`'s two `resume_unwind` sites,
/// `windows/event_loop.rs`'s one).
///
/// So the runtime value that would make this row false is a toolkit
/// UPGRADE that adds a catch, which is exactly the change that would
/// make the crash announcement worthless and exactly what nothing else
/// here would notice.
#[cfg(feature = "app")]
#[test]
fn a_panic_inside_an_egui_frame_is_not_swallowed() {
    // `Context::run_ui` is eframe's own per-frame call, at this
    // version, and its closure argument is where `eframe::App::ui` —
    // and so `ViewerApp::ui`, and so the seam read — is invoked:
    // `eframe-0.36.1/src/native/epi_integration.rs`'s
    // `self.egui_ctx.run_ui(raw_input, |ui| …)`, mirrored in the wgpu
    // and glow integrations and in the web runner. So this is the real
    // door and not a door-shaped stand-in.
    let ctx = egui::Context::default();
    let escaped = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let _ = ctx.run_ui(egui::RawInput::default(), |_ui| {
            panic!("the planted panic");
        });
    }));
    let payload = escaped.expect_err("egui must not absorb a panic raised inside a frame");
    assert_eq!(
        payload.downcast_ref::<&str>().copied(),
        Some("the planted panic"),
        "and it must be the SAME panic, not one egui re-raised of its own",
    );
}
