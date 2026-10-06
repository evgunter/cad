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
//! file is the ceiling-plus-δ half, a second whole-box replay of each,
//! and it runs where the gating row does: in `.config/nextest.toml`'s
//! slow set.

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
///
/// Each value is SYM-11's measurement moved by the merges named beside
/// it (`sz`, `reg`, `num`: the first three columns), each bisected on
/// `main` and re-run at its parent:
/// - #3257, `path_junction_side` asked of every declared tangent joint;
/// - #3270, `tangent_jet`'s curvatures over their own gradient norms;
/// - #3313, the must-carry rule's per-station `dihedral_wedge`;
/// - #3254, the arc carrier's span from the stored sweep;
/// - #3266, the run outs read against their arrival carriers;
/// - #3455 and #3594, the id mints
///   (`work/rules/the-pads-frozen-set-moves-with-the-documents-id-mint`);
/// - #3612 and #3697, the schedule assigning its ends and its middle;
/// - #3645, one `mid_point`;
/// - #2468, DECIDE-3's fold, rule G and the decision read;
/// - #3527, copied arc carriers;
/// - #3807, DECIDE-9;
/// - #3759, the pcurve mint's checks;
/// - #3981, check 5's escape;
/// - #4037, the closing joint decided as every joint.
const PAST_THE_CEILING: [(&str, [u64; 4]); 5] = [
    // sz +8 #2468, +136 #3759, +8 #3981; reg +8 #3759; num −8 #2468,
    // +236 #3759, −8 #3981, +14 #4037; frozen +96 #3645, −96 #3697,
    // +24 #3759.
    ("two_hole_plate", [955, 148, 704, 1069]),
    // sz +112 #3759; reg +8 #3759; num +236 #3759, −8 #3981, +14 #4037;
    // frozen +96 #3645, −96 #3697, +24 #3759.
    ("r1_annulus", [440, 148, 451, 1080]),
    // sz +68 #3759, +4 #3981; reg +9 #3254, −5 #2468, +4 #3759; num +4
    // #3257, −9 #3254, +5 #2468, +118 #3759, −4 #3981, +7 #4037; frozen
    // +48 #3645, −48 #3697, +12 #3759.
    ("r2_link", [286, 84, 296, 568]),
    // sz +1 #3266, +7 #2468, +112 #3759; reg +8 #3759; num +2 #3257, +1
    // #3266, −12 #2468, +236 #3759, −8 #3981, +14 #4037; frozen −3
    // #3612, +96 #3645, −96 #3697, −4 #3527, +24 #3759.
    ("r2_filleted_bracket", [548, 149, 574, 1113]),
    // sz +28 #3313, +3 #3266, +8 #2468, +32 #3807, +87 #3759, +4 #3981;
    // reg +22 #2468, −2 #3527, +4 #3759; num +8 #3257, +84 #3313, +3
    // #3266, −64 #2468, +2 #3527, +176 #3759, −4 #3981, +10 #4037;
    // frozen −28 #3270, −130 #3254, −15 #3455, −48 #3612, +168 #3645,
    // −168 #3697, +15 #3594, −34 #3527, +77 #3759.
    ("r2_rounded_pad", [1016, 152, 1186, 2587]),
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
/// **WHAT RUNS IT: the slow set** (`.config/nextest.toml`): every
/// night at every ε row, and on every PR whose diff seeds `editor-core`.
/// It is five whole-box replays on top of the gating row's, so it stays
/// off the per-PR fast set.
#[test]
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
