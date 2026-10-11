//! **The coincidence door runs at rest.** A landing draws at once and
//! its checks report reads the door *pending*; the job's findings
//! settle into that same report when it lands, an answer for a landing
//! since replaced is dropped, and the next request cancels the job.
//!
//! The seam here holds each job for the row to answer by hand, so what
//! is asserted is the session's bookkeeping, not a thread's timing.

// Panicking is a test's failure mechanism (workspace lint note).
#![allow(clippy::expect_used)]
#![allow(clippy::panic)]

use std::sync::{Arc, Mutex};

use pncad::document::{CancelToken, CheckId, coincide};
use pncad::geom_core::Tol;
use viewer::evalseam::{DoorDone, DoorRequest, DoorService, InlineEvaluator};
use viewer::frame::{Tone, checks_badge, pending_line};
use viewer::session::{DocSession, SessionOp};

use crate::corpus;

/// What the held seam saw, and the answers it hands back on `poll`.
#[derive(Default)]
struct Seen {
    requests: Vec<DoorRequest>,
    cancels: usize,
    answers: Vec<DoorDone>,
}

/// A door seam that runs nothing: it keeps each request, counts the
/// cancels, and answers what the row put in [`Seen::answers`].
#[derive(Clone, Default)]
struct Held(Arc<Mutex<Seen>>);

impl Held {
    fn seen(&self) -> std::sync::MutexGuard<'_, Seen> {
        self.0.lock().expect("an unpoisoned lock")
    }
}

impl DoorService for Held {
    fn submit(&mut self, request: DoorRequest) {
        self.seen().requests.push(request);
    }

    fn cancel(&mut self) {
        self.seen().cancels += 1;
    }

    fn poll(&mut self) -> Option<DoorDone> {
        self.seen().answers.pop()
    }

    fn busy(&self) -> bool {
        false
    }
}

/// The `unproven-coincidence` findings of the landed report.
fn unproven(session: &DocSession) -> usize {
    session
        .checks()
        .expect("the registry ran")
        .findings
        .iter()
        .filter(|f| f.check == CheckId::UnprovenCoincidence)
        .count()
}

#[test]
fn a_landings_door_reads_pending_until_its_own_answer_settles() {
    let tol = Tol::witness();
    let doc = corpus::documents()
        .into_iter()
        .find(|c| c.name == "reshaped_rod")
        .expect("a corpus document")
        .doc;
    let held = Held::default();
    let mut session = DocSession::new(doc, tol, Box::new(InlineEvaluator::new()))
        .with_door(Box::new(held.clone()));
    session.pump();

    let report = session.checks().expect("the registry ran");
    assert_eq!(report.pending, [CheckId::UnprovenCoincidence], "{report}");
    assert_eq!(unproven(&session), 0, "pending is not a finding");
    assert!(
        pending_line(report).is_some_and(|l| l.contains("unproven-coincidence")),
        "the Checks window says what is pending"
    );
    let badge = checks_badge(Some(report)).expect("a pending report badges");
    assert_eq!(
        (badge.label(), badge.tone()),
        ("checks: pending", Tone::Advisory),
        "pending alone is advisory"
    );
    let first = {
        let seen = held.seen();
        assert_eq!(seen.requests.len(), 1, "one landing, one door job");
        seen.requests[0].clone()
    };

    // An answer for a landing the session has since replaced is
    // dropped, and the request that replaced it canceled the job.
    let cancels = held.seen().cancels;
    let outcome = session.perform(SessionOp::Reevaluate);
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    assert_eq!(
        held.seen().cancels,
        cancels + 1,
        "the next request cancels the door"
    );
    session.pump();
    let second = held.seen().requests.last().expect("a second job").clone();
    assert_ne!(second.generation, first.generation);
    let stale = first
        .job
        .run(&first.doc, &CancelToken::new())
        .expect("an uncanceled door");
    held.seen().answers.push(DoorDone {
        generation: first.generation,
        findings: Ok(stale),
    });
    session.pump();
    let report = session.checks().expect("the registry ran");
    assert_eq!(
        report.pending,
        [CheckId::UnprovenCoincidence],
        "a stale answer settles nothing"
    );

    // A canceled answer for this landing leaves it pending.
    held.seen().answers.push(DoorDone {
        generation: second.generation,
        findings: Err(coincide::Canceled),
    });
    session.pump();
    assert_eq!(
        session.checks().expect("the registry ran").pending,
        [CheckId::UnprovenCoincidence]
    );

    // Its own answer settles: no longer pending, and the door's finding
    // is in the report.
    let found = second
        .job
        .run(&second.doc, &CancelToken::new())
        .expect("an uncanceled door");
    held.seen().answers.push(DoorDone {
        generation: second.generation,
        findings: Ok(found),
    });
    session.pump();
    let report = session.checks().expect("the registry ran");
    assert!(report.pending.is_empty(), "{report}");
    assert_eq!(
        unproven(&session),
        1,
        "the rod's support row is unproven: {report}"
    );
    assert!(pending_line(report).is_none());
}
