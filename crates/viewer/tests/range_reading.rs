//! **`range?` is the certified search**, as the session runs it and the
//! panel reads it: the search on its own seam, its stop, the discard an
//! edit makes, the admission that greys the control, and the words of
//! each reading.
//!
//! The documents are straight-edged, so the leaf protocol certifies
//! them quickly (`editor-core`'s `docm9_range` is where their shapes
//! come from): a square slid sideways, which proves its whole seed, and
//! a profile whose one vertex moves, which builds on another branch or
//! stops building past a point.

// Panicking is a test's failure mechanism (workspace lint note).
#![allow(clippy::expect_used)]
#![allow(clippy::panic)]

use crate::common;

use pncad::analysis::{CertifiedRange, RangeRefusal, RangeSeed};
use pncad::document::{
    Dimension, Distribution, Doc, DocEdit, Formula, FreeVar, LoopProgram, Node, ProfileProgram,
    RecipeNodeId, SlotId, VarDecl, VarName,
};
use pncad::geom_core::Tol;
use viewer::evalseam::{RangeRequest, RangeRun, RangeService, RangeState, ThreadRanger};
use viewer::props::{self, SlotValue};
use viewer::session::{DocSession, RangeReading, RangeTarget, Refusal, SessionOp};

fn tol() -> Tol {
    Tol::witness()
}

fn param(n: &'static str) -> Formula {
    Formula::named(VarName::from_static(n), Dimension::Length)
}

/// `a + b`, two lengths.
fn plus(a: Formula, b: f64) -> Formula {
    Formula::add(a, common::len(b)).expect("a length plus a length")
}

/// `doc` with the length `n` declared at `value`.
fn declared(doc: &Doc<ProfileProgram>, n: &'static str, value: f64) -> Doc<ProfileProgram> {
    common::edited(
        doc,
        DocEdit::DeclareVar {
            name: VarName::from_static(n),
            def: VarDecl::Free(FreeVar::continuous(Dimension::Length, value)),
        },
        tol(),
    )
    .0
}

/// `doc` with `points` as a polygon profile, extruded by a unit
/// literal. Answers the document and the extrude.
fn extruded<const N: usize>(
    doc: &Doc<ProfileProgram>,
    points: [[Formula; 2]; N],
) -> (Doc<ProfileProgram>, RecipeNodeId) {
    let (doc, profile) = common::inserted(
        doc,
        Node::Profile(ProfileProgram {
            loops: vec![LoopProgram::polygon_expr(points)],
            ids: Vec::new(),
            on: None,
        }),
        tol(),
    );
    common::inserted(
        &doc,
        Node::Extrude {
            profile: profile.into(),
            distance: common::len(1.0),
        },
        tol(),
    )
}

/// **The slid square**: a unit square whose left side sits at `x`,
/// extruded. Sliding it decides nothing differently, so the search
/// proves its whole seed.
fn slid(x: f64) -> Doc<ProfileProgram> {
    let doc = declared(&Doc::empty_derived("range-slid", tol()), "x", x);
    extruded(
        &doc,
        [
            [param("x"), common::len(0.0)],
            [plus(param("x"), 1.0), common::len(0.0)],
            [plus(param("x"), 1.0), common::len(1.0)],
            [param("x"), common::len(1.0)],
        ],
    )
    .0
}

/// **The notch**: a unit square whose bottom side has a vertex at its
/// midpoint, at height `h − below`, extruded. Past height zero the
/// vertex is a notch and the walls beside it turn the other way: the
/// document builds, on a branch the witness's decisions do not
/// describe. Past height one it crosses the top side, and the profile
/// stops building.
fn notch(label: &str, h: f64, below: f64) -> (Doc<ProfileProgram>, RecipeNodeId) {
    let doc = declared(&Doc::empty_derived(label, tol()), "h", h);
    extruded(
        &doc,
        [
            [common::len(0.0), common::len(0.0)],
            [common::len(0.5), plus(param("h"), -below)],
            [common::len(1.0), common::len(0.0)],
            [common::len(1.0), common::len(1.0)],
            [common::len(0.0), common::len(1.0)],
        ],
    )
}

/// **The slab**: a unit square extruded by `d`. Its depth refuses at
/// zero, which is where `range?`'s seed ends below, so the low side
/// halves toward zero until its depth floor and says more time will not
/// help; the high side proves its whole seed.
fn slab(d: f64) -> Doc<ProfileProgram> {
    let doc = declared(&Doc::empty_derived("range-slab", tol()), "d", d);
    let (doc, profile) = common::inserted(&doc, common::square(1.0), tol());
    common::inserted(
        &doc,
        Node::Extrude {
            profile: profile.into(),
            distance: param("d"),
        },
        tol(),
    )
    .0
}

/// A session over `doc`, landed, with `range?` asked of the variable `n`.
fn ranging(doc: Doc<ProfileProgram>, n: &str) -> (DocSession, RangeTarget) {
    let mut session = DocSession::inline(doc, tol());
    session.pump();
    let target = RangeTarget::Variable {
        var: common::var_of(session.committed_doc(), n),
    };
    let outcome = session.perform(SessionOp::ProveRange {
        target: target.clone(),
    });
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    assert!(outcome.committed.is_empty(), "a run commits nothing");
    (session, target)
}

/// Pump until `done` holds of the reading or the run ends, at most
/// `cap` pumps: the inline seam takes one step a pump.
fn pump_until(session: &mut DocSession, cap: usize, done: impl Fn(&RangeReading) -> bool) {
    for _ in 0..cap {
        let reading = session.range().expect("the run's reading stands");
        if done(reading) || !reading.proving() {
            return;
        }
        session.pump();
    }
    panic!("the run did not get there in {cap} pumps");
}

/// The reading's answer, which a run that built its witness has.
fn answer(session: &DocSession) -> CertifiedRange {
    session
        .range()
        .and_then(RangeReading::answer)
        .cloned()
        .expect("an answer")
}

/// The reading's lines, as the panel draws them.
fn lines(session: &DocSession) -> Vec<String> {
    session.range().expect("a reading").lines(session.doc())
}

/// A number as the reading writes it: in metres, the unit the
/// fixtures' variables are declared in.
fn m(value: f64) -> String {
    props::shown_text(Some(pncad::prelude::M.def()), value)
}

/// A proven end as the reading writes it: rounded toward the current
/// value.
fn end(value: f64, nominal: f64) -> String {
    props::shown_text_toward(Some(pncad::prelude::M.def()), value, nominal)
}

/// A point proven different as the reading writes it: rounded away
/// from the current value.
fn point(value: f64, nominal: f64) -> String {
    let away = if value > nominal {
        f64::INFINITY
    } else {
        f64::NEG_INFINITY
    };
    props::shown_text_toward(Some(pncad::prelude::M.def()), value, away)
}

/// `doc` with `n`'s spread declared, so a run of another field pins it.
fn spread(doc: &Doc<ProfileProgram>, n: &'static str) -> Doc<ProfileProgram> {
    common::edited(
        doc,
        DocEdit::SetVarDistribution {
            var: VarName::from_static(n).into(),
            distribution: Some(Distribution::Band {
                lo: -0.01,
                hi: 0.01,
            }),
        },
        tol(),
    )
    .0
}

/// **A whole reading, in order: what is proven, how far it looked, and
/// what it held** — on a field nothing reads, whose every replay
/// certifies, so the row is cheap.
#[test]
fn a_finished_reading_says_what_it_proved_how_far_it_looked_and_what_it_held() {
    let doc = declared(&Doc::empty_derived("range-idle", tol()), "x", 0.5);
    let doc = spread(&declared(&doc, "spare", 0.25), "spare");
    let (mut session, _) = ranging(doc, "x");
    pump_until(&mut session, 16, |_| false);
    let reading = session.range().expect("the reading stands");
    assert!(
        matches!(reading.state, RangeState::Finished(_)),
        "{:?}",
        reading.state
    );
    assert_eq!(
        lines(&session),
        [
            format!("proven {}…{}", end(0.0, 0.5), end(1.0, 0.5)),
            format!("checked to {}…{}", end(0.0, 0.5), end(1.0, 0.5)),
            "holding spare at their current values".to_owned(),
        ],
        "the proof, how far it looked, and what it held, in that order"
    );
}

/// **A slid square is proven over its whole seed**: real geometry the
/// leaf protocol certifies, at the seed `range?` chooses (`±|x|`).
#[test]
fn a_slid_square_is_proven_over_its_seed() {
    let (mut session, _) = ranging(slid(0.5), "x");
    pump_until(&mut session, 64, |_| false);
    let r = answer(&session);
    assert!(
        r.lo().reached_seed_end() && r.hi().reached_seed_end(),
        "{r:?}"
    );
    assert_eq!(
        lines(&session),
        [
            format!("proven {}…{}", end(0.0, 0.5), end(1.0, 0.5)),
            format!("checked to {}…{}", end(0.0, 0.5), end(1.0, 0.5)),
        ]
    );
}

/// **Stop keeps what is proven, and the notch builds differently past
/// undecided ground.** The run is stopped once the high side has proven
/// a difference; what the stopped run reads is the answer it had, the
/// ground between the proof and the difference is "not decided" and
/// never valid, and more proving may move both sides.
#[test]
fn stop_keeps_the_proof_and_the_notch_builds_differently_past_undecided_ground() {
    let (doc, _) = notch("range-notch", 0.375, 0.5);
    let (mut session, _) = ranging(doc, "h");
    pump_until(&mut session, 64, |reading| {
        reading
            .answer()
            .is_some_and(|r| r.hi().different().is_some())
    });
    let before = answer(&session);
    let outcome = session.perform(SessionOp::StopRange);
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    assert!(outcome.committed.is_empty(), "a stop commits nothing");
    session.pump();
    let reading = session.range().expect("a stopped run keeps its reading");
    assert!(
        matches!(reading.state, RangeState::Stopped(_)),
        "{:?}",
        reading.state
    );
    assert!(!session.ranging(), "nothing is stepping");
    let after = answer(&session);
    assert_eq!(
        after.proven_interval(),
        before.proven_interval(),
        "the stop keeps the proof"
    );
    let (lo, hi) = after.proven_interval();
    let at = after.absolute(after.hi().different().expect("the difference stands").at());
    let steps = after.steps();
    assert_eq!(
        lines(&session),
        [
            format!("proven {}…{}", end(lo, 0.375), end(hi, 0.375)),
            format!("checked to {}…{}", end(0.0, 0.375), end(0.75, 0.375)),
            format!(
                "below: not decided past {}: stopped after {steps} replays; more proving may go \
                 further",
                end(lo, 0.375)
            ),
            format!(
                "above: not decided past {}: stopped after {steps} replays; more proving may go \
                 further",
                end(hi, 0.375)
            ),
            format!("above: at {} it builds, differently", point(at, 0.375)),
        ],
    );
    assert!(
        hi < 0.5 && 0.5 < at,
        "the vertex crosses the line at h = 0.5, between the proof and the difference"
    );
}

/// **A side the search has finished with says more time will not
/// help**: the slab's low side halves toward its zero depth until its
/// depth floor; its high side proves its seed.
#[test]
fn a_settled_side_says_more_time_will_not_help() {
    let (mut session, _) = ranging(slab(1.0), "d");
    pump_until(&mut session, 512, |_| false);
    let r = answer(&session);
    assert!(r.hi().reached_seed_end(), "{r:?}");
    let gap = r.lo().not_decided().expect("the low side is not decided");
    assert!(!gap.more_time_helps(), "{gap:?}");
    let said = lines(&session);
    let prefix = format!(
        "below: not decided past {}: the search halved this side as finely as it may (",
        end(r.proven_interval().0, 1.0)
    );
    assert!(
        said.iter().any(|line| line.starts_with(&prefix)
            && line.ends_with(" halvings); more time will not help")),
        "{said:?}"
    );
    assert_eq!(said.len(), 3, "proven, checked, and the low side: {said:?}");
}

/// **Nothing proven around the current value**, each side named: a run
/// stopped before its first replay says so and never draws an interval;
/// one stopped while its witness was built says it proved nothing.
#[test]
fn a_run_stopped_before_it_proves_anything_says_so() {
    let (mut session, _) = ranging(slid(0.5), "x");
    // One pump builds the witness: the run is proving, nothing replayed.
    session.pump();
    assert_eq!(
        lines(&session),
        [format!("proving… nothing proven around {} so far", m(0.5))]
    );
    session.perform(SessionOp::StopRange);
    session.pump();
    let gap = |side: &str| {
        format!(
            "{side}: not decided past {}: stopped after 0 replays; more proving may go further",
            m(0.5)
        )
    };
    assert_eq!(
        lines(&session),
        [
            format!("nothing proven around {}", m(0.5)),
            format!("checked to {}…{}", end(0.0, 0.5), end(1.0, 0.5)),
            gap("below"),
            gap("above"),
        ]
    );

    let (mut session, _) = ranging(slid(0.5), "x");
    session.perform(SessionOp::StopRange);
    session.pump();
    assert!(!session.ranging());
    assert_eq!(lines(&session), ["stopped before anything was proven"]);
}

/// **While it runs, the line says what is proven so far**, and the
/// range seam's work is not the picture's.
#[test]
fn a_running_search_says_what_it_has_proven_so_far() {
    let (mut session, _) = ranging(slid(0.5), "x");
    assert_eq!(lines(&session), ["proving…"], "before the witness is built");
    assert!(session.ranging(), "the range seam has work");
    assert!(!session.busy(), "and the picture does not wait on it");
    pump_until(&mut session, 64, |reading| {
        reading.answer().is_some_and(|r| !r.nothing_proven())
    });
    let (lo, hi) = answer(&session).proven_interval();
    assert_eq!(
        lines(&session),
        [format!(
            "proving… proven {}…{} so far",
            end(lo, 0.5),
            end(hi, 0.5)
        )]
    );
}

/// **A slot that reads a named variable is that variable's range**, and
/// the reading says so: widening it moves every slot that reads it.
#[test]
fn a_slot_reading_a_named_variable_says_the_range_is_the_variables() {
    let mut session = DocSession::inline(slab(1.0), tol());
    session.pump();
    let extrude = session
        .tree_rows()
        .iter()
        .map(|row| row.id)
        .next_back()
        .expect("the slab has its extrude");
    let outcome = session.perform(SessionOp::ProveRange {
        target: RangeTarget::Slot {
            node: extrude,
            slot: SlotId::Distance,
        },
    });
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    session.pump();
    assert_eq!(
        lines(&session),
        [
            format!("proving… nothing proven around {} so far", m(1.0)),
            "the range of d, which moves every slot that reads it".to_owned(),
        ]
    );
}

/// **Every door that changes or replaces the shown document stops the
/// run and drops its reading**: an edit, a value gesture, an undo, a
/// redo, Open and New — and another `range?` replaces it. After each,
/// nothing is stepping.
#[test]
fn every_door_that_changes_the_document_stops_the_run() {
    let (mut session, target) = ranging(slid(0.5), "x");
    let var = common::var_of(session.committed_doc(), "x");
    let restart = |session: &mut DocSession| {
        let outcome = session.perform(SessionOp::ProveRange {
            target: RangeTarget::Variable {
                var: common::var_of(session.committed_doc(), "x"),
            },
        });
        assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
        session.pump();
        assert!(session.ranging(), "the run is stepping before the door");
    };
    let stopped = |session: &mut DocSession, door: &str| {
        assert!(session.range().is_none(), "{door} drops the reading");
        for _ in 0..4 {
            session.pump();
        }
        assert!(
            session.range().is_none(),
            "{door}: and the run's reports with it"
        );
        assert!(!session.ranging(), "{door}: the run is still stepping");
    };
    session.pump();
    assert!(session.ranging());
    let outcome = session.perform(SessionOp::SetVariable {
        var,
        value: SlotValue::Continuous(0.25),
    });
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    stopped(&mut session, "an edit");

    restart(&mut session);
    session.perform(SessionOp::BeginVariableGesture { var });
    session.perform(SessionOp::PreviewVariableGesture { var, value: 0.3 });
    stopped(&mut session, "a value gesture");
    session.perform(SessionOp::CancelGesture);
    session.pump();

    restart(&mut session);
    session.perform(SessionOp::Undo);
    stopped(&mut session, "an undo");
    restart(&mut session);
    session.perform(SessionOp::Redo);
    stopped(&mut session, "a redo");

    let dir = common::tempdir("range-open");
    let file = dir.join("slid.pncad");
    assert!(
        session
            .perform(SessionOp::Save(file.clone()))
            .refusal
            .is_none()
    );
    session.pump();
    restart(&mut session);
    let opened = session.perform(SessionOp::Open(file));
    assert!(opened.refusal.is_none(), "{:?}", opened.refusal);
    stopped(&mut session, "Open");
    session.pump();

    restart(&mut session);
    let new = session.perform(SessionOp::NewDocument {
        name: "n".to_owned(),
    });
    assert!(new.refusal.is_none(), "{:?}", new.refusal);
    stopped(&mut session, "New");
    std::fs::remove_dir_all(&dir).expect("the fixture directory is removable");

    let (mut session, _) = ranging(slid(0.5), "x");
    session.pump();
    let first = session.range().expect("a reading").run;
    session.perform(SessionOp::ProveRange { target });
    let second = session.range().expect("a reading");
    assert_ne!(second.run, first, "another `range?` is another run");
    assert!(
        matches!(second.state, RangeState::Starting),
        "whose reading starts empty"
    );
}

/// **Admission greys `range?` before the click, in the refusal's own
/// words, and runs no geometry**: a slot a formula defines has no
/// interval of its own, and a document that does not build now has no
/// witness, which admission reads off the session's own landed build.
/// A refused request changes nothing: the run already in flight stands.
#[test]
fn admission_refuses_ahead_of_the_click_in_the_kernels_words() {
    let tol = tol();
    let (doc, _profile, extrude) = common::parametric_plate(tol);
    let mut session = DocSession::inline(doc, tol);
    session.pump();
    let thickness = RangeTarget::Variable {
        var: common::thickness_var(session.committed_doc()),
    };
    assert!(
        session.range_refusal(&thickness).is_none(),
        "the variable the formula reads is offered"
    );
    session.perform(SessionOp::ProveRange {
        target: thickness.clone(),
    });
    let running = session.range().expect("a run on the variable").run;
    let target = RangeTarget::Slot {
        node: extrude,
        slot: SlotId::Distance,
    };
    let greyed = session.range_refusal(&target).expect("the control greys");
    assert!(
        matches!(&greyed, Refusal::Range(r) if matches!(**r, RangeRefusal::SlotIsDefined { .. })),
        "{greyed:?}"
    );
    let refused = session
        .perform(SessionOp::ProveRange { target })
        .refusal
        .expect("the click is refused");
    assert_eq!(
        refused.to_string(),
        greyed.to_string(),
        "one door, one sentence"
    );
    let standing = session.range().expect("the run in flight stands");
    assert_eq!(
        (standing.run, &standing.target),
        (running, &thickness),
        "a refused request neither sets a reading nor clears the one there"
    );
    assert!(session.ranging(), "and its run goes on");

    // A depth of zero does not build: admission says so from the
    // session's own build, before any search.
    let mut session = DocSession::inline(slab(0.0), tol);
    session.pump();
    let target = RangeTarget::Variable {
        var: common::var_of(session.committed_doc(), "d"),
    };
    let greyed = session.range_refusal(&target).expect("the control greys");
    assert!(
        matches!(&greyed, Refusal::Range(r) if matches!(**r, RangeRefusal::Drive(_))),
        "{greyed:?}"
    );
    // **The build admission reads is the picture's only while the
    // picture answers the document.** Fixed and not yet evaluated, the
    // depth is admitted: the broken build no longer answers it.
    let outcome = session.perform(SessionOp::SetVariable {
        var: common::var_of(session.committed_doc(), "d"),
        value: SlotValue::Continuous(1.0),
    });
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    assert!(session.busy(), "the fix has not landed");
    assert_eq!(
        session.range_refusal(&target).map(|r| r.to_string()),
        None,
        "a build of the broken document does not grey the fixed one"
    );
}

/// **The session hears only the run it is waiting on**: a seam that
/// hands back a report of some other run moves nothing. A scripted seam
/// stands in for one that would.
#[test]
fn a_report_of_another_run_moves_no_reading() {
    #[derive(Default)]
    struct Scripted(std::rc::Rc<std::cell::RefCell<Vec<viewer::evalseam::RangeReport>>>);
    impl RangeService for Scripted {
        fn submit(&mut self, _: RangeRequest) {}
        fn stop(&mut self) {}
        fn poll(&mut self) -> Option<viewer::evalseam::RangeReport> {
            self.0.borrow_mut().pop()
        }
        fn busy(&self) -> bool {
            !self.0.borrow().is_empty()
        }
    }
    let queue = std::rc::Rc::default();
    let mut session = DocSession::inline(slid(0.5), tol())
        .with_ranger(Box::new(Scripted(std::rc::Rc::clone(&queue))));
    session.pump();
    session.perform(SessionOp::ProveRange {
        target: RangeTarget::Variable {
            var: common::var_of(session.committed_doc(), "x"),
        },
    });
    let run = session.range().expect("a run").run;
    let other = declared(&Doc::empty_derived("range-other", tol()), "x", 0.25);
    let field = pncad::analysis::RangeField::Param(common::var_of(&other, "x"));
    let seed = RangeSeed::of(&other, &field).expect("a field");
    let foreign = pncad::analysis::certified_range(
        &other,
        &field,
        seed,
        &pncad::analysis::RangeBudget::for_seed(seed, tol(), 2),
        None,
        &pncad::document::Stop::default(),
        tol(),
    )
    .expect("the other document builds");
    queue.borrow_mut().push(viewer::evalseam::RangeReport {
        run: RangeRun(run.0 + 1),
        state: RangeState::Finished(foreign),
    });
    session.pump();
    assert!(
        matches!(
            session.range().expect("the reading stands").state,
            RangeState::Starting
        ),
        "another run's report is not this run's answer"
    );
}

// ------------------------------------------------- the threaded seam

/// How long a threaded row waits for its worker before it fails.
const PATIENCE: std::time::Duration = std::time::Duration::from_secs(60);

/// A session whose `range?` runs on the application's own worker.
fn threaded(doc: Doc<ProfileProgram>) -> DocSession {
    let mut session = DocSession::inline(doc, tol())
        .with_ranger(Box::new(ThreadRanger::spawn().expect("the worker starts")));
    session.pump();
    session
}

/// Pump `session` until `done` holds, failing after [`PATIENCE`]; every
/// reading seen on the way is handed to `seen`.
fn pump_threaded(
    session: &mut DocSession,
    mut seen: impl FnMut(&RangeReading),
    done: impl Fn(&DocSession) -> bool,
) {
    let start = std::time::Instant::now();
    while !done(session) {
        assert!(start.elapsed() < PATIENCE, "the worker did not get there");
        std::thread::sleep(std::time::Duration::from_millis(2));
        session.pump();
        if let Some(reading) = session.range() {
            seen(reading);
        }
    }
}

/// **On the worker, every answer drawn is the current document's**: a
/// run on the notch at one height, an edit while it steps, and a run at
/// the new height — no answer the panel can draw is of the old height.
/// Repeated, since the race it pins is a matter of timing.
#[test]
fn on_the_worker_no_answer_of_an_earlier_run_is_drawn() {
    for round in 0..5 {
        let (doc, _) = notch("range-threaded", 0.375, 0.5);
        let mut session = threaded(doc);
        let h = common::var_of(session.committed_doc(), "h");
        session.perform(SessionOp::ProveRange {
            target: RangeTarget::Variable { var: h },
        });
        pump_threaded(
            &mut session,
            |_| {},
            |s| {
                s.range()
                    .and_then(RangeReading::answer)
                    .is_some_and(|r| r.steps() > round)
            },
        );
        session.perform(SessionOp::SetVariable {
            var: h,
            value: SlotValue::Continuous(0.3),
        });
        session.pump();
        session.perform(SessionOp::ProveRange {
            target: RangeTarget::Variable { var: h },
        });
        let mut drawn = 0;
        pump_threaded(
            &mut session,
            |reading| {
                if let Some(r) = reading.answer() {
                    drawn += 1;
                    assert_eq!(r.nominal(), 0.3, "round {round}: an earlier run was drawn");
                }
            },
            |s| {
                s.range()
                    .and_then(RangeReading::answer)
                    .is_some_and(|r| r.steps() >= 3)
            },
        );
        assert!(drawn > 0, "round {round}: the new run was drawn");
        session.perform(SessionOp::StopRange);
        pump_threaded(&mut session, |_| {}, |s| !s.ranging());
    }
}

/// **On the worker, stop keeps the proof and the run ends**: after the
/// stop the reading is the stopped run's, with at least what it had
/// proven, and nothing is stepping.
#[test]
fn on_the_worker_stop_keeps_the_proof_and_ends_the_run() {
    for _ in 0..3 {
        let (doc, _) = notch("range-threaded-stop", 0.375, 0.5);
        let mut session = threaded(doc);
        let h = common::var_of(session.committed_doc(), "h");
        session.perform(SessionOp::ProveRange {
            target: RangeTarget::Variable { var: h },
        });
        pump_threaded(
            &mut session,
            |_| {},
            |s| {
                s.range()
                    .and_then(RangeReading::answer)
                    .is_some_and(|r| !r.nothing_proven())
            },
        );
        let before = answer(&session).proven_offsets();
        session.perform(SessionOp::StopRange);
        pump_threaded(&mut session, |_| {}, |s| !s.ranging());
        session.pump();
        let reading = session.range().expect("a stopped run keeps its reading");
        let RangeState::Stopped(after) = &reading.state else {
            panic!("the run stopped: {:?}", reading.state);
        };
        let (lo, hi) = after.proven_offsets();
        assert!(
            lo <= before.0 && before.1 <= hi,
            "the stop keeps the proof: {before:?} then {:?}",
            (lo, hi)
        );
    }
}

/// **A finished run leaves nothing to poll**: its last report clears
/// the step report in its cell, so the seam answers nothing after it.
#[test]
fn on_the_worker_a_finished_run_clears_its_cell() {
    let doc = declared(&Doc::empty_derived("range-cell", tol()), "x", 0.5);
    let field = pncad::analysis::RangeField::Param(common::var_of(&doc, "x"));
    let seed = RangeSeed::of(&doc, &field).expect("a field");
    for round in 0..5 {
        let mut seam = ThreadRanger::spawn().expect("the worker starts");
        seam.submit(RangeRequest {
            run: RangeRun(round),
            doc: std::sync::Arc::new(doc.clone()),
            field: field.clone(),
            seed,
            resolver: None,
            tol: tol(),
        });
        let start = std::time::Instant::now();
        let last = loop {
            assert!(start.elapsed() < PATIENCE, "the worker did not finish");
            match seam.poll() {
                Some(report) if !report.state.proving() => break report,
                _ => std::thread::sleep(std::time::Duration::from_millis(1)),
            }
        };
        assert!(
            matches!(last.state, RangeState::Finished(_)),
            "{:?}",
            last.state
        );
        assert!(!seam.busy(), "round {round}: the worker is done");
        std::thread::sleep(std::time::Duration::from_millis(20));
        assert!(
            seam.poll().is_none(),
            "round {round}: a step report outlived its run's last report"
        );
    }
}
