//! **The EXACT channel never contradicts its own form** — the measured
//! half of the partition SYM-11 draws.
//!
//! At `Sym<Interval>` a definite non-zero sign is a certified bracket
//! that excludes zero, so a form that is the zero polynomial under it
//! would mean one of the two channels does not contain its real. That
//! is a soundness defect and a panic is the honest answer; what this
//! file measures is that it never happens on the documents the tier is
//! measured on — at the scale each certifies whole at, and past its
//! ceiling, where the residuals are the widest the drive ever sees.
//!
//! The nominal half is pinned by the gating row
//! `m10_9_no_registrant_lies_on_any_measured_document`, which replays
//! the same five documents and now reads the dispute column too. This
//! file is the ceiling-plus-δ half, a second whole-box replay of each.
//! It runs on the per-PR fast set: since #3774 its replays stop early
//! and it costs under the slow set's 1 s bar (0.86 s hosted).

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
/// The plate and the annulus refuse nowhere the bisection reaches, and
/// replay whole at [`PAST_NO_CEILING`]. The link stops at the
/// extrude's attachment gate (`carrier_matches_mapped_source`). The
/// bracket and the pad stop at the replayed profile's validation
/// (`arc_span`, `line_span`), so theirs is the validation prefix's
/// receipt, and nothing in that prefix reaches the door. How each value
/// moved since SYM-11 (2026-09-21), merge by merge, is the attribution
/// table in
/// `work/sym/ignored-sym-receipt-rows-drifted-red-on-main-unattributed`.
///
/// Only the `frozen` column moved when an intrinsic edge description's
/// surfaces became a set (`geom_brep::SurfacePair`): +29, +24, +12, +24,
/// +12 in this order, and no decision column. The cause is the
/// transversality wedge (`geom_brep::dihedral::wedge_decided`'s
/// `n1.cross(n2)`), which the certificate now takes in key order where
/// the extrude took it in builder order: the cross product is symmetric
/// in value up to sign but not in form, so a document that reads one
/// edge's wedge in both orientations builds two hash-consed forms where
/// it built one. The residual checks' order contributes nothing.
const PAST_THE_CEILING: [(&str, [u64; 4]); 5] = [
    ("two_hole_plate", [1103, 0, 704, 641]),
    ("r1_annulus", [588, 0, 451, 828]),
    ("r2_link", [373, 9, 284, 496]),
    ("r2_filleted_bracket", [644, 0, 516, 830]),
    ("r2_rounded_pad", [368, 0, 302, 314]),
];

/// The scale, in multiples of ε, a document with no measured refusal
/// replays at. Not the bisection's top, `1e6·ε`: at ε = 1e-6 that is a
/// scale of one, and the annulus's extrude refuses there geometrically
/// before the tier is asked what it is here to be asked.
const PAST_NO_CEILING: f64 = 1e5;

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
/// scale the measured bracket says the drive refuses at, or
/// [`PAST_NO_CEILING`] for a document that refuses nowhere the
/// bisection reaches — where every leaf's residuals are the widest the
/// corpus produces. The exact
/// witness contradicts its own form at NONE of them, at any ε.
///
/// **The receipts are ASSERTED, not recorded in prose**, so a re-take
/// reds on drift instead of quietly printing a different table:
/// [`PAST_THE_CEILING`] is the measurement, identical at ε = 1e-6,
/// 1e-9 and 1e-12.
///
/// **What runs it: the per-PR fast set**, at the default ε on every PR
/// whose test scope includes `editor-core` (a change to it or to a crate
/// it depends on), and every night at every ε row. It
/// costs 0.86 s hosted, under `.config/nextest.toml`'s 1 s slow-set bar,
/// so it is not in the slow set.
#[test]
fn sym11_the_exact_channel_never_contradicts_past_the_ceiling() {
    let tol = Tol::witness();
    let eps = tol.eps();
    let mut fired = 0;
    let mut moved = Vec::new();
    for (study, want) in measured_studies(tol).into_iter().zip(PAST_THE_CEILING) {
        let name = study.name;
        assert_eq!(
            name, want.0,
            "PAST_THE_CEILING is read positionally against `measured_studies`, and the two \
             have gone out of order"
        );
        let past = study.refuses_at.unwrap_or(PAST_NO_CEILING);
        let doc = (study.at)(past * eps);
        let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
        match replay_on_a_thread(doc, ParamBox::of(&analyzed), tol) {
            Ok((refusal, counts)) => {
                println!("   {name} at eps={eps:e}, ceiling + δ: {counts:?} -> {refusal:?}");
                let got = [
                    counts.symbolic_zero,
                    counts.registered,
                    counts.numeric,
                    counts.frozen,
                ];
                if got != want.1 {
                    moved.push(format!("{name}: {got:?}, stored {:?}", want.1));
                }
                if counts.registrations_contradicted != 0 {
                    moved.push(format!(
                        "{name}: a registered identity was contradicted past the ceiling — \
                         that is an axiom a constructor stated and this box disproves: {counts:?}"
                    ));
                }
                if counts.theorems_disputed != 0 {
                    moved.push(format!(
                        "{name}: this lane's witness is EXACT, so the theorem-vs-numeric \
                         contradiction is routed to the `debug_assert!` and never to this \
                         column — a count here is `Interval::WITNESS` having moved, not a \
                         document behaving badly: {counts:?}"
                    ));
                }
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
    assert!(
        moved.is_empty(),
        "at eps={eps:e}, past the ceiling (a receipt is [symbolic_zero, registered, \
         numeric, frozen]). This row is the measurement SYM-11 argued the dispute column's \
         zero from, so a number that moved is a decision that moved and is a finding, not a \
         table to refresh:\n{}",
        moved.join("\n")
    );
}
