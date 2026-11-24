//! **The EXACT channel never contradicts its own form** — the measured
//! half of the partition SYM-11 draws.
//!
//! At `Sym<Interval>` a definite non-zero sign is a certified bracket
//! that excludes zero, so a form that is the zero polynomial under it
//! would mean one of the two channels does not contain its real. That
//! is a soundness defect; the decision asks the tier nothing there, so
//! the test instrument (`geom_core::sym::contradiction`) asks it after
//! the replay, and what this file measures is that it finds nothing on
//! the documents the tier is measured on past each one's ceiling, where
//! the residuals are the widest the drive ever sees. A symbolic `Zero`
//! whose enclosure excludes 0 is the other half, asserted inside the
//! replay, and a panic here is that assertion.
//!
//! This file is a whole-box replay of each of the five documents
//! `m10_9_no_registrant_lies_on_any_measured_document` replays.
//! It runs in the slow set: the bracket replays whole past its ceiling
//! and the pad reaches its extrude, about 20 s in a release build.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::sync::Arc;

use editor_core::ProfileDoc;
use editor_core::analysis::{AnalysisPolicy, ParamBox, analyzed_box};
use editor_core::{CancelToken, EvalOptions, NodeResult, evaluate};
use geom_core::sym::contradiction::{self, Report};
use geom_core::sym::revalue::record;
use geom_core::{SymCounts, SymRules, Tol};

use crate::m10_9_pins_interval::measured_studies;

/// **WHERE EACH REPLAY PAST ITS CEILING STOPS, AND WHAT IT MAY NOT
/// LOSE**, in `measured_studies`' order: the stop — `None` where it
/// replays whole, or the predicate its refusal names — then a floor on
/// `symbolic` and a ceiling on `frozen`. The claim below is over every
/// decision taken before the stop, so a replay that stops earlier
/// claims less, and reds.
///
/// The plate and the annulus refuse nowhere the bisection reaches, and
/// replay whole at [`PAST_NO_CEILING`]. The link replays whole too:
/// rule H proves the extrude's attachment gate
/// (`carrier_matches_mapped_source`) it stopped at. The bracket's and
/// the pad's over-band span readings (`arc_span`, `line_span`) are
/// settled by the segments' ends, which lie on the other carrier, so
/// the bracket and the pad replay whole.
///
/// **The floor and the ceiling are witnesses of two open rows**, which
/// ask whether moves of these columns were decisions lost:
/// `work/onehome/sym11s-symbolic-column-fell-on-four-documents-with-the-rigid-motion-move.md`
/// and `work/onedoor/sym11s-numeric-and-frozen-columns-rose-at-6869-the-wedge-fold-stage-2.md`.
/// A theorem that goes numeric, or a form that newly freezes, reds
/// here; a move the other way does not. They narrow to the stop alone
/// once both rows close.
///
/// The floors fell, and the pad's ceiling, when validation stopped
/// reading an adjacent line pair's shared vertex (PROFILE, `dbc608ffba`):
/// decisions no longer asked, not decisions answered differently.
///
/// The `frozen` ceilings are the decisions' own walks: a definite numeric
/// sign walks none.
///
/// They fell again with the profile's direction mint asking the chord's
/// expression whether it is zero (`geom_core::Expression::proves_zero`,
/// no decision) where it read a proved Zero as one: the plate 955 → 951,
/// the link 786 → 785, the bracket 1337 → 1329, the pad 1471 → 1455,
/// each a reading no longer taken. The link's and the pad's frozen
/// ceilings fell with the arcs after an axis hop departing along the
/// literal axis: 2175 → 1501 and 1303 → 127.
const PAST_THE_CEILING: [(&str, Option<&str>, u64, u64); 5] = [
    ("two_hole_plate", None, 951, 36),
    ("r1_annulus", None, 511, 36),
    ("r2_link", None, 785, 1501),
    ("r2_filleted_bracket", None, 1329, 53),
    ("r2_rounded_pad", None, 1455, 127),
];

/// The scale, in multiples of ε, a document with no measured refusal
/// replays at. Not the bisection's top, `1e6·ε`: at ε = 1e-6 that is a
/// scale of one, and the annulus's extrude refuses there geometrically
/// before the tier is asked what it is here to be asked.
const PAST_NO_CEILING: f64 = 1e5;

/// One whole-box replay at `Sym<Interval>`, ON ITS OWN THREAD: the
/// session's counts, the first node that refused and the instrument's
/// report, or the panic's own message when the enclosure check fired. A panic
/// inside the session leaves that thread's session installed and the
/// next replay would refuse to nest, so the thread is what makes the
/// next document runnable ([`test_utils::own_thread`], which keeps the
/// message — an `Err` that said only "it panicked" would send a reader
/// looking for the document that did it).
fn replay_on_a_thread(
    doc: ProfileDoc,
    box_: ParamBox,
    tol: Tol,
) -> Result<(Option<String>, SymCounts, Report), String> {
    test_utils::own_thread::caught(move || {
        let opts = EvalOptions {
            param_box: Some(Arc::new(box_)),
            ..EvalOptions::default()
        };
        let budget = geom_core::SymBudget {
            max_terms: editor_core::drive::DEFAULT_SYM_MAX_TERMS,
            max_degree: editor_core::drive::DEFAULT_SYM_MAX_DEGREE,
        };
        let ((refusal, report), counts) =
            geom_core::sym::with_session_rules(budget, SymRules::shipped(), || {
                let (refusal, decisions) = record(|| {
                    let ev: editor_core::Evaluation<geom_core::Sym<geom_core::Interval>> =
                        evaluate(&doc, None, &CancelToken::new(), &opts, tol);
                    ev.order.iter().find_map(|id| match ev.result(*id) {
                        Some(NodeResult::Failed(e)) => Some(format!("node {} — {}", id.0, e.kind)),
                        _ => None,
                    })
                });
                let report = contradiction::check(&decisions).expect("inside the session");
                (refusal, report)
            });
        (refusal, counts, report)
    })
}

/// **THE STOP CLAUSE'S ROW.** The five measured documents replayed at
/// `Sym<Interval>` past their ceilings — `Study::refuses_at`, the
/// scale the measured bracket says the drive refuses at, or
/// [`PAST_NO_CEILING`] for a document that refuses nowhere the
/// bisection reaches — where every leaf's residuals are the widest the
/// corpus produces. The instrument finds the exact witness contradicting
/// its own form at NONE of them, at any ε, and no symbolic zero's
/// enclosure excludes 0.
///
/// **What the zero speaks for is asserted beside it**: each replay
/// stops where [`PAST_THE_CEILING`] says, proves no fewer theorems and
/// freezes no more forms than it says, and the instrument asked
/// definite decisions it could read, at ε = 1e-6, 1e-9 and 1e-12.
///
/// **What runs it: the slow set** (`.config/nextest.toml`), on every PR
/// whose diff touches `editor-core`, and every night at every ε row.
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
            Ok((refusal, counts, report)) => {
                println!("   {name} at eps={eps:e}, ceiling + δ: {counts:?} -> {refusal:?}");
                println!(
                    "     instrument: {} definite decisions checked, {} absent",
                    report.checked, report.absent
                );
                let stopped = match (&refusal, want.1) {
                    (None, None) => true,
                    (Some(r), Some(predicate)) => r.contains(predicate),
                    _ => false,
                };
                if !stopped {
                    moved.push(format!(
                        "{name}: stops at {refusal:?}, where it stopped at {:?}",
                        want.1
                    ));
                }
                if counts.symbolic < want.2 || counts.frozen > want.3 {
                    moved.push(format!(
                        "{name}: symbolic {} under its floor {} or frozen {} over its ceiling \
                         {}: {counts:?}",
                        counts.symbolic, want.2, counts.frozen, want.3
                    ));
                }
                if report.checked == 0 || report.absent != 0 {
                    moved.push(format!(
                        "{name}: the instrument checked {} decisions and could not read {}",
                        report.checked, report.absent
                    ));
                }
                for c in &report.contradictions {
                    moved.push(format!(
                        "{name}: the tier proves `{}` zero ({:?}, {:?} rung, attempt {}) under a \
                         definite {:?}",
                        c.predicate, c.event, c.rung, c.attempt, c.sign
                    ));
                }
            }
            Err(message) => {
                println!("   {name} at eps={eps:e}, ceiling + δ: the ENCLOSURE CHECK FIRED");
                println!("     {message}");
                fired += 1;
            }
        }
    }
    assert_eq!(
        fired, 0,
        "a symbolic zero's certified enclosure excluded 0 on {fired} of the measured \
         documents — one of the two channels does not contain its real. That is a \
         soundness defect in a channel and a DIFFERENT unit: stop, file it with the \
         render, and report"
    );
    assert!(
        moved.is_empty(),
        "at eps={eps:e}, past the ceiling. A stop, floor or ceiling that moved is a \
         decision that moved; a contradiction is a soundness defect:\n{}",
        moved.join("\n")
    );
}
