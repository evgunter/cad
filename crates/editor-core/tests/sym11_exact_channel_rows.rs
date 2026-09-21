//! **The EXACT channel never contradicts its own form** — the measured
//! half of the partition SYM-11 draws.
//!
//! At `Sym<Interval>` a definite non-zero sign is a certified bracket
//! that excludes zero, so a form that is the zero polynomial under it
//! would mean one of the two channels does not contain its real. That
//! is a soundness defect and a panic is the honest answer; what this
//! file measures is that it never happens on the documents the tier is
//! measured on — at the scale each certifies whole at, and at the
//! scale past its ceiling, where the leaves refuse and the residuals
//! are the widest the drive ever sees.
//!
//! The nominal half is pinned by the gating row
//! `m10_9_no_registrant_lies_on_any_measured_document`, which replays
//! the same five documents and now reads the dispute column too. This
//! file is the ceiling-plus-δ half, which is a second whole-box replay
//! of each — too heavy for the gate and cheap to re-take by hand.
#![cfg(feature = "interval")]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::sync::Arc;

use editor_core::ProfileDoc;
use editor_core::analysis::{AnalysisPolicy, ParamBox, analyzed_box};
use editor_core::{CancelToken, EvalOptions, NodeResult, ProfileLift, evaluate};
use geom_core::{SymCounts, SymRules, Tol};

use crate::m10_9_pins_interval::measured_studies;

/// One whole-box replay at `Sym<Interval>`, ON ITS OWN THREAD: the
/// session's counts and the first node that refused, or `Err` when the
/// theorem-vs-numeric assertion fired. A panic inside the session
/// leaves that thread's session installed, and the next replay would
/// refuse to nest.
fn replay_on_a_thread(
    doc: ProfileDoc,
    box_: ParamBox,
    tol: Tol,
) -> Result<(Option<String>, SymCounts), ()> {
    std::thread::spawn(move || {
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(move || {
            let opts = EvalOptions {
                param_box: Some(Arc::new(box_)),
                profile_lift: ProfileLift::Guided,
                ..EvalOptions::default()
            };
            let budget = geom_core::SymBudget {
                max_terms: editor_core::drive::DEFAULT_SYM_MAX_TERMS,
                max_degree: editor_core::drive::DEFAULT_SYM_MAX_DEGREE,
            };
            geom_core::sym::with_session_rules(budget, SymRules::shipped(), || {
                let ev: editor_core::Evaluation<geom_core::Sym<geom_core::Interval>> =
                    evaluate(&doc, None, &CancelToken::new(), &opts, tol);
                ev.order.iter().find_map(|id| match ev.result(*id) {
                    Some(NodeResult::Failed(e)) => Some(format!("node {} — {}", id.0, e.kind)),
                    _ => None,
                })
            })
        }))
        .map_err(|_| ())
    })
    .join()
    .expect("the replay thread itself joins")
}

/// **THE STOP CLAUSE'S ROW.** The five measured documents replayed at
/// `Sym<Interval>` past their ceilings — `Study::refuses_at`, the
/// scale the measured bracket says the drive refuses at — where every
/// leaf's residuals are the widest the corpus produces. The exact
/// witness contradicts its own form at NONE of them, at any ε.
///
/// Phase 1's measurement, at ε = 1e-6, 1e-9 and 1e-12 alike: zero
/// contradictions on all five, and every replay's
/// `registrations_contradicted` is zero beside it. The receipts past
/// the ceiling, identical at all three ε
/// (`symbolic_zero / registered / numeric / frozen`):
/// plate `803 / 140 / 470 / 1044`, annulus `328 / 140 / 209 / 1056`,
/// link `214 / 76 / 175 / 556`, bracket `428 / 141 / 341 / 1096`,
/// pad `854 / 128 / 971 / 2750`.
///
/// `#[ignore]`d: five whole-box replays of the corpus on top of the
/// ones the gating rows already pay.
#[test]
#[ignore = "evidence-only: five whole-box replays past the measured ceilings"]
fn sym11_the_exact_channel_never_contradicts_past_the_ceiling() {
    let tol = Tol::witness();
    let eps = tol.eps();
    let mut fired = 0;
    for study in measured_studies(tol) {
        let name = study.name;
        let doc = (study.at)(study.refuses_at * eps);
        let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
        match replay_on_a_thread(doc.clone(), ParamBox::of(&analyzed), tol) {
            Ok((refusal, counts)) => {
                println!("   {name} at eps={eps:e}, ceiling + δ: {counts:?} -> {refusal:?}");
                assert_eq!(
                    counts.registrations_contradicted, 0,
                    "{name}: a registered identity was contradicted past the ceiling — \
                     that is an axiom a constructor stated and this box disproves: {counts:?}"
                );
            }
            Err(()) => {
                println!("   {name} at eps={eps:e}, ceiling + δ: the CONTRADICTION FIRED");
                fired += 1;
            }
        }
    }
    assert_eq!(
        fired, 0,
        "the exact witness contradicted its own form on {fired} of the measured documents — \
         a certified bracket that excludes zero and a form that is the zero polynomial \
         cannot both be right, so one of the two channels does not contain its real. That \
         is a soundness defect in a channel and a DIFFERENT unit: stop, file it with the \
         render, and report"
    );
}
