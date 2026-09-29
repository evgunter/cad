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

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::sync::Arc;

use editor_core::ProfileDoc;
use editor_core::analysis::{AnalysisPolicy, ParamBox, analyzed_box};
use editor_core::{CancelToken, EvalOptions, NodeResult, ProfileLift, evaluate};
use geom_core::{SymCounts, SymRules, Tol};

use crate::m10_9_pins_interval::measured_studies;

/// **THE RECEIPTS PAST EACH CEILING**, in `measured_studies`' order,
/// as `[symbolic_zero, registered, numeric, frozen]`. Identical at
/// ε = 1e-6, 1e-9 and 1e-12: the atoms a residual carries do not
/// depend on the band, and neither does what the tier proves about
/// them.
const PAST_THE_CEILING: [(&str, [u64; 4]); 5] = [
    ("two_hole_plate", [803, 140, 470, 1044]),
    ("r1_annulus", [328, 140, 209, 1056]),
    ("r2_link", [214, 76, 175, 556]),
    ("r2_filleted_bracket", [428, 141, 341, 1096]),
    // +28 `symbolic_zero` and +84 `numeric` from the must-carry rule's
    // per-station dihedral gate (16 edges x 7 stations of
    // `dihedral_wedge`).
    ("r2_rounded_pad", [882, 128, 1055, 2750]),
];

/// One whole-box replay at `Sym<Interval>`, ON ITS OWN THREAD: the
/// session's counts and the first node that refused, or the panic's
/// own message when the theorem-vs-numeric assertion fired. A panic
/// inside the session leaves that thread's session installed and the
/// next replay would refuse to nest, so the thread is what makes the
/// next document runnable ([`test_utils::own_thread`], which keeps the
/// message — an `Err` that said only "it panicked" would send a reader
/// looking for the document that did it).
fn replay_on_a_thread(
    doc: ProfileDoc,
    box_: ParamBox,
    tol: Tol,
) -> Result<(Option<String>, SymCounts), String> {
    test_utils::own_thread::caught(move || {
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
    })
}

/// **THE STOP CLAUSE'S ROW.** The five measured documents replayed at
/// `Sym<Interval>` past their ceilings — `Study::refuses_at`, the
/// scale the measured bracket says the drive refuses at — where every
/// leaf's residuals are the widest the corpus produces. The exact
/// witness contradicts its own form at NONE of them, at any ε.
///
/// **The receipts are ASSERTED, not recorded in prose**, so a re-take
/// reds on drift instead of quietly printing a different table:
/// [`PAST_THE_CEILING`] is the measurement, identical at ε = 1e-6,
/// 1e-9 and 1e-12.
///
/// **WHEN IT IS RE-TAKEN: by hand, at each SYM unit's close**, because
/// nothing schedules it — it is `#[ignore]`d (five whole-box replays
/// of the corpus on top of the ones the gating rows already pay, about
/// two minutes) and no sweep names it. The register of that obligation
/// is the unit item, not this comment:
/// `work/sym/sym-f64-far-placement-trips-the-theorem-vs-numeric-assert`
/// while it is open, and the SYM log after it closes.
#[test]
#[ignore = "evidence-only: five whole-box replays past the measured ceilings; re-taken by hand at each SYM unit's close"]
fn sym11_the_exact_channel_never_contradicts_past_the_ceiling() {
    let tol = Tol::witness();
    let eps = tol.eps();
    let mut fired = 0;
    for (study, want) in measured_studies(tol).into_iter().zip(PAST_THE_CEILING) {
        let name = study.name;
        assert_eq!(
            name, want.0,
            "PAST_THE_CEILING is read positionally against `measured_studies`, and the two \
             have gone out of order"
        );
        let doc = (study.at)(study.refuses_at * eps);
        let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
        match replay_on_a_thread(doc, ParamBox::of(&analyzed), tol) {
            Ok((refusal, counts)) => {
                println!("   {name} at eps={eps:e}, ceiling + δ: {counts:?} -> {refusal:?}");
                assert_eq!(
                    [
                        counts.symbolic_zero,
                        counts.registered,
                        counts.numeric,
                        counts.frozen
                    ],
                    want.1,
                    "{name} at eps={eps:e}: the receipt past the ceiling moved. This row is \
                     the measurement SYM-11 argued the dispute column's zero from, so a \
                     number that moved is a decision that moved and is a finding, not a \
                     table to refresh: {counts:?}"
                );
                assert_eq!(
                    counts.registrations_contradicted, 0,
                    "{name}: a registered identity was contradicted past the ceiling — \
                     that is an axiom a constructor stated and this box disproves: {counts:?}"
                );
                assert_eq!(
                    counts.theorems_disputed, 0,
                    "{name}: this lane's witness is EXACT, so the theorem-vs-numeric \
                     contradiction is routed to the `debug_assert!` and never to this \
                     column — a count here is `Interval::WITNESS` having moved, not a \
                     document behaving badly: {counts:?}"
                );
            }
            Err(message) => {
                println!("   {name} at eps={eps:e}, ceiling + δ: the CONTRADICTION FIRED");
                println!("     {message}");
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
