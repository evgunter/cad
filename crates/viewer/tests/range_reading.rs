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

use pncad::analysis::{CertifiedRange, RangeRefusal};
use pncad::document::{
    Dimension, Distribution, Doc, DocEdit, Formula, FreeVar, LoopProgram, Node, ProfileProgram,
    RecipeNodeId, SlotId, VarDecl, VarName,
};
use pncad::geom_core::Tol;
use viewer::evalseam::RangeProgress;
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
        .and_then(|reading| reading.answer.clone())
        .expect("an answer")
        .expect("the run was admitted and its witness builds")
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

/// **A side that proves its whole seed, the seed it was checked to, and
/// the parameters held**: the slid square proves `0…1`, which is the
/// seed `range?` chooses (`±|x|`), and a second variable with a declared
/// spread is held at its value.
#[test]
fn a_slid_square_is_proven_over_its_seed_holding_its_other_spread() {
    let doc = slid(0.5);
    let (doc, _) = common::edited(
        &declared(&doc, "spare", 0.25),
        DocEdit::SetVarDistribution {
            var: VarName::from_static("spare").into(),
            distribution: Some(Distribution::Band {
                lo: -0.01,
                hi: 0.01,
            }),
        },
        tol(),
    );
    let (mut session, _) = ranging(doc, "x");
    pump_until(&mut session, 64, |_| false);
    let reading = session.range().expect("the reading stands");
    assert_eq!(reading.progress, RangeProgress::Finished);
    let r = answer(&session);
    assert!(
        r.lo().reached_seed_end() && r.hi().reached_seed_end(),
        "{r:?}"
    );
    assert_eq!(
        lines(&session),
        [
            format!("proven {}…{}", m(0.0), m(1.0)),
            format!("checked to {}…{}", m(0.0), m(1.0)),
            "holding spare at their current values".to_owned(),
        ],
        "the proof, how far it looked, and what it held"
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
    pump_until(
        &mut session,
        64,
        |reading| matches!(&reading.answer, Some(Ok(r)) if r.hi().different().is_some()),
    );
    let before = answer(&session);
    let outcome = session.perform(SessionOp::StopRange);
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    assert!(outcome.committed.is_empty(), "a stop commits nothing");
    session.pump();
    let reading = session.range().expect("a stopped run keeps its reading");
    assert_eq!(reading.progress, RangeProgress::Stopped);
    assert!(!session.ranging(), "nothing is stepping");
    let after = answer(&session);
    assert_eq!(
        after.proven_interval(),
        before.proven_interval(),
        "the stop keeps the proof"
    );
    let (lo, hi) = after.proven_interval();
    let different = after.hi().different().expect("the difference stands");
    let steps = after.steps();
    assert_eq!(
        lines(&session),
        [
            format!("proven {}…{}", m(lo), m(hi)),
            format!("checked to {}…{}", m(0.0), m(0.75)),
            format!(
                "not decided past {}: stopped after {steps} replays; more proving may go further",
                m(lo)
            ),
            format!(
                "not decided past {}: stopped after {steps} replays; more proving may go further",
                m(hi)
            ),
            format!(
                "at {} it builds, differently",
                m(after.absolute(different.at()))
            ),
        ],
    );
    assert!(
        hi < 0.5 && 0.5 < after.absolute(different.at()),
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
    let low = said
        .iter()
        .find(|line| line.starts_with("not decided past "))
        .unwrap_or_else(|| panic!("the low side is said: {said:?}"));
    assert!(
        low.starts_with(&format!(
            "not decided past {}: the search halved this side its ",
            m(r.proven_interval().0)
        )) && low.ends_with(" times; more time will not help"),
        "{low}"
    );
    assert_eq!(said.len(), 3, "proven, checked, and the low side: {said:?}");
}

/// **Nothing proven around the current value**: a run stopped before its
/// first replay says so, and never draws an interval.
#[test]
fn a_run_stopped_before_its_first_replay_proves_nothing_around_the_value() {
    let (mut session, _) = ranging(slid(0.5), "x");
    // One pump builds the witness: the run is proving, nothing replayed.
    session.pump();
    assert_eq!(
        lines(&session),
        [format!("proving… nothing proven around {} so far", m(0.5))]
    );
    session.perform(SessionOp::StopRange);
    session.pump();
    let gap = format!(
        "not decided past {}: stopped after 0 replays; more proving may go further",
        m(0.5)
    );
    assert_eq!(
        lines(&session),
        [
            format!("nothing proven around {}", m(0.5)),
            format!("checked to {}…{}", m(0.0), m(1.0)),
            gap.clone(),
            gap,
        ]
    );
}

/// **While it runs, the line says what is proven so far.**
#[test]
fn a_running_search_says_what_it_has_proven_so_far() {
    let (mut session, _) = ranging(slid(0.5), "x");
    assert_eq!(lines(&session), ["proving…"], "before the witness is built");
    assert!(session.ranging(), "the range seam has work");
    assert!(!session.busy(), "and the picture does not wait on it");
    pump_until(
        &mut session,
        64,
        |reading| matches!(&reading.answer, Some(Ok(r)) if !r.nothing_proven()),
    );
    let (lo, hi) = answer(&session).proven_interval();
    assert_eq!(
        lines(&session),
        [format!("proving… proven {}…{} so far", m(lo), m(hi))]
    );
}

/// **An edit, a value gesture or another `range?` stops the run and
/// drops its reading**: a range is a statement about one document, and
/// one run.
#[test]
fn an_edit_a_gesture_or_another_range_drops_the_run() {
    let (mut session, target) = ranging(slid(0.5), "x");
    session.pump();
    let var = common::var_of(session.committed_doc(), "x");
    let outcome = session.perform(SessionOp::SetVariable {
        var,
        value: SlotValue::Continuous(0.25),
    });
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    assert!(session.range().is_none(), "an edit drops the reading");
    session.pump();
    assert!(session.range().is_none(), "and the run's reports with it");
    assert!(!session.ranging(), "and the run stops");

    session.perform(SessionOp::ProveRange {
        target: target.clone(),
    });
    session.pump();
    session.perform(SessionOp::BeginVariableGesture { var });
    session.perform(SessionOp::PreviewVariableGesture { var, value: 0.3 });
    assert!(session.range().is_none(), "a value gesture drops it");
    session.perform(SessionOp::CancelGesture);
    session.pump();

    session.perform(SessionOp::ProveRange {
        target: target.clone(),
    });
    session.pump();
    let first = session.range().expect("a reading").run;
    session.perform(SessionOp::ProveRange { target });
    let second = session.range().expect("a reading");
    assert_ne!(second.run, first, "another `range?` is another run");
    assert!(second.answer.is_none(), "whose reading starts empty");
}

/// **Admission greys `range?` before the click, in the refusal's own
/// words, and runs no geometry**: a slot a formula defines has no
/// interval of its own, and a document that does not build now has no
/// witness, which admission reads off the session's own landed build.
#[test]
fn admission_refuses_ahead_of_the_click_in_the_kernels_words() {
    let tol = tol();
    let (doc, _profile, extrude) = common::parametric_plate(tol);
    let mut session = DocSession::inline(doc, tol);
    session.pump();
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
    assert!(session.range().is_none(), "a refused run leaves no reading");
    assert!(
        session
            .range_refusal(&RangeTarget::Variable {
                var: common::thickness_var(session.committed_doc()),
            })
            .is_none(),
        "the variable the formula reads is offered"
    );

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
}
