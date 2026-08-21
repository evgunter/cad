//! M4 PR 6 spec D6.2 — the ε-change diff CI row (goldened), the
//! first REAL two-ε diff (discharging PR 4 review Finding 6's wait).
//!
//! One process hosts ONE ε (`geom_core::Tolerance` commits once —
//! spec D4's standing constraint), so the two runs of a SetTolerance
//! audit can never share an address space. The audit's shape is
//! therefore persist-grade BY CONSTRUCTION: each run evaluates in its
//! own process, serializes its verdict populations
//! ([`editor_core::verdict_summary`]), and the report comes from
//! [`editor_core::diff_summaries`] — the same population math as the
//! in-process engine, via one shared core.
//!
//! The fixture is margin-thin by design: a profile segment with
//! sagitta 1e-6 (bulge 2e-6 on a unit chord). At ε = 1e-9 the
//! `segment_straightness` predicate decides POSITIVE (a real arc); at
//! ε = 1e-4 it decides ZERO (straight). Both ε values keep the margin
//! far outside the escalation band (K = 10), so both runs stay Ok;
//! the report — the exact net flips plus the arc→straight branch's
//! reshaped decision structure as loud divergence rows — is goldened
//! below, field by field.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod fixture;

use editor_core::{
    CancelToken, EvalOptions, Node, ProfileDoc, RecipeNodeId, RunStatus, evaluate, load, save,
    verdict_summary,
};
use fixture::{desc, insert};
use geom_core::Sign;
use geom_core::Tol;

/// The env var carrying the child probe's output path.
const PROBE_OUT: &str = "M4_PR6_EPS_PROBE_OUT";

/// The two audit ε values (chosen so the 1e-6 sagitta is definite on
/// both sides — no escalation-band contact at K = 10).
const EPS_OLD: &str = "1e-9";
const EPS_NEW: &str = "1e-4";

/// The margin-thin fixture: one profile whose third segment carries
/// bulge 2e-6 (sagitta 1e-6 on the unit chord) — a single
/// `segment_straightness` margin between the two audit ε values.
fn thin_profile_doc() -> ProfileDoc {
    let (doc, _) = insert(
        ProfileDoc::empty_derived("m4_pr6_eps_diff"),
        Node::Profile({
            // v4: the thin bulge authors as an arc_to step with its
            // AUTHORED bulge (the program stores exactly the value the
            // retired form stored on the vertex).
            use editor_core::{
                Dimension, Expr, LoopProgram, ProgramArcData, ProgramStep, ProgramTarget,
            };
            let lpt = |x: f64, y: f64| {
                [
                    Expr::literal(x, Dimension::Length).unwrap(),
                    Expr::literal(y, Dimension::Length).unwrap(),
                ]
            };
            let mut d = desc([0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0], vec![]);
            d.loops = vec![LoopProgram::Chain(vec![
                ProgramStep::At(lpt(0.0, 0.0)),
                ProgramStep::LineTo(ProgramTarget::Point(lpt(1.0, 0.0))),
                ProgramStep::LineTo(ProgramTarget::Point(lpt(1.0, 1.0))),
                // The (1,1) → (0,1) segment's thin bulge.
                ProgramStep::ArcTo(ProgramArcData::Bulge {
                    target: ProgramTarget::Point(lpt(0.0, 1.0)),
                    b: Expr::literal(2e-6, Dimension::Scalar).unwrap(),
                }),
                ProgramStep::LineTo(ProgramTarget::Start),
            ])];
            d
        }),
    );
    doc
}

/// CHILD MODE (no-op unless [`PROBE_OUT`] is set): evaluates the
/// fixture at THIS process's committed ε and writes its verdict
/// summary as JSON. The parent spawns this test twice with different
/// `CAD_TOLERANCE_EPS` values — the only way two ε values can exist
/// in one audit (module docs).
#[test]
fn child_eps_probe() {
    let Ok(out) = std::env::var(PROBE_OUT) else {
        return; // Normal suite run: the parent drives this test.
    };
    let doc = thin_profile_doc();
    let ev = evaluate::<f64>(&doc, None, &CancelToken::new(), &EvalOptions::default());
    let summary = verdict_summary(&ev);
    let json = serde_json::to_string(&summary).expect("summary serializes");
    std::fs::write(&out, json).expect("probe output writable");
}

fn spawn_probe(eps: &str, out: &std::path::Path) -> editor_core::VerdictSummary {
    let exe = std::env::current_exe().expect("test exe path");
    // Self re-exec: name the probe by MODULE PATH, not by bare fn name.
    // `tests/all.rs` aggregates every suite into one binary, so libtest sees
    // this probe as `<this_module>::child_eps_probe`. Stripping the leading crate name off
    // `module_path!()` yields the right filter in the aggregated layout AND in
    // a standalone one (where `module_path!()` has no `::` at all).
    let probe = match module_path!().split_once("::") {
        Some((_, m)) => format!("{m}::child_eps_probe"),
        None => "child_eps_probe".to_string(),
    };
    let status = std::process::Command::new(exe)
        .args([probe.as_str(), "--exact", "--nocapture"])
        .env("CAD_TOLERANCE_EPS", eps)
        .env(PROBE_OUT, out)
        .status()
        .expect("probe spawns");
    assert!(status.success(), "probe at eps={eps} failed");
    let json = std::fs::read_to_string(out).expect("probe wrote");
    serde_json::from_str(&json).expect("summary parses")
}

#[test]
fn eps_change_diff_reports_exactly_the_flipped_predicate() {
    let dir = std::env::temp_dir();
    let old = spawn_probe(
        EPS_OLD,
        &dir.join(format!("m4pr6_eps_old_{}", std::process::id())),
    );
    let new = spawn_probe(
        EPS_NEW,
        &dir.join(format!("m4pr6_eps_new_{}", std::process::id())),
    );

    let flips = editor_core::diff_summaries(&old, &new);
    // GOLDEN (update only on a ratified predicate-vocabulary or
    // fixture change — RE-PINNED once at the #101 merge: declared
    // tangency added validation probes, so carrier_line_circle and
    // chord_side instance counts grew; the FLIPS are unchanged):
    // exactly ONE differing node — the profile —
    // both runs Ok. The ε re-classification reports as EXACTLY these
    // net flips (the thin segment_straightness margin, twice decided
    // per validation pass, plus the line_span probes the collapsed
    // arc now answers at Zero), and the arc→straight branch change
    // reports its reshaped decision structure as loud DIVERGENCE
    // rows (arc-only predicates leaving, chord probes recounting) —
    // never absorbed, never guessed about (vdiff module docs).
    assert_eq!(
        flips.nodes.len(),
        1,
        "exactly one differing node: {flips:?}"
    );
    let delta = flips
        .nodes
        .get(&RecipeNodeId(0))
        .expect("profile node delta");
    let expected = editor_core::SummaryDelta {
        old_status: RunStatus::Ok,
        new_status: RunStatus::Ok,
        flips: vec![
            editor_core::SummaryFlip {
                predicate: "line_span".into(),
                from: Sign::Negative,
                to: Sign::Zero,
                count: 2,
            },
            editor_core::SummaryFlip {
                predicate: "segment_straightness".into(),
                from: Sign::Positive,
                to: Sign::Zero,
                count: 2,
            },
        ],
        diverged: vec![
            editor_core::SummaryDivergence {
                predicate: "arc_diameter_clearance".into(),
                old_count: 2,
                new_count: 0,
            },
            editor_core::SummaryDivergence {
                predicate: "arc_span".into(),
                old_count: 6,
                new_count: 0,
            },
            editor_core::SummaryDivergence {
                predicate: "carrier_line_circle".into(),
                old_count: 5,
                new_count: 0,
            },
            editor_core::SummaryDivergence {
                predicate: "chord_side".into(),
                old_count: 14,
                new_count: 28,
            },
            editor_core::SummaryDivergence {
                predicate: "contact_at_shared_vertex".into(),
                old_count: 6,
                new_count: 8,
            },
            editor_core::SummaryDivergence {
                predicate: "line_span".into(),
                old_count: 10,
                new_count: 8,
            },
        ],
    };
    assert_eq!(delta, &expected, "the ε audit's goldened report drifted");
    // The report surface: exactly the flipped predicates, in order.
    let report = flips.report();
    assert_eq!(report.len(), 2);
    assert!(report.iter().all(|(node, _)| *node == RecipeNodeId(0)));

    // The no-edit control: a summary diffs empty against itself.
    assert!(editor_core::diff_summaries(&old, &old).is_empty());
}

#[test]
fn set_tolerance_round_trips_and_gates_replay() {
    // The recorded-ε edit persists: a log ending in SetTolerance
    // loads ONLY in a process whose ε matches the new value (D4's
    // one-process-one-ε refusal is pinned in m4_pr6_refusal.rs; here
    // the matching side: a fresh child at the NEW ε loads the file
    // and reports the recorded value).
    let doc = thin_profile_doc();
    let text = save(&doc, &[editor_core::DocEdit::SetTolerance { eps: 1e-4 }]).expect("save");
    // In THIS process the recorded ε (ambient) plus the edit's 1e-4
    // conflicts unless ambient IS 1e-4 — assert the door's decision
    // matches the committed ε, whichever matrix row we run under.
    let ambient = geom_core::Tol::witness().get().eps;
    match load(&text) {
        Ok(loaded) => {
            assert_eq!(ambient.to_bits(), 1e-4f64.to_bits());
            assert_eq!(loaded.doc.epsilon().to_bits(), 1e-4f64.to_bits());
        }
        Err(editor_core::PersistError::ToleranceConflict { process, document }) => {
            assert_eq!(process.to_bits(), ambient.to_bits());
            assert_eq!(document.to_bits(), 1e-4f64.to_bits());
        }
        Err(other) => panic!("unexpected load refusal: {other:?}"),
    }
}
