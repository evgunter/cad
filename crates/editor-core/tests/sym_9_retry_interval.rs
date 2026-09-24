//! **SYM-9's measurement: where the refusals are, and what a retry
//! recovers** — the two Phase 1 tables of
//! `work/sym/coefficient-ring-width-is-not-monotone-in-reach`'s unit,
//! taken at the nominal on five of the six measured documents (R2's
//! rounded pad does not return a nominal replay on a four-core box:
//! `work/sym/the-pads-nominal-replay-is-not-takeable-on-a-four-core-box`;
//! the leaf instrument in `m10_10_evidence_interval` takes it).
//!
//! The instrument is the RETRY LADDER itself (`geom_core::sym::SymRetry`)
//! at its dials, not a reverted patch, and the reason is what a ladder
//! is: a retry is asked only where every rung of the first attempt has
//! already declined, so the first attempt's answers are identical with
//! the ladder installed and without it, and the difference between two
//! replays IS "what the retry recovers", per decision, with nothing to
//! subtract. A whole-tier re-run at a wider ring cannot answer the same
//! question — it moves the FIRST attempt, which is the non-monotonicity
//! this unit's item records.
//!
//! **Two rows here GATE** — [`sym_9_the_drive_writes_the_ladders_receipt`],
//! which drives one whole-box leaf of the bracket at the drive's default
//! dials and reads both receipts' ladder clauses, and
//! [`sym_9_the_kept_atom_ladder_recovers_what_phase_1_measured`],
//! which pins what the measured ladder (`SymRetry::kept_atom`) recovers
//! on the two documents that gain from it and pins ZERO on the three
//! that do not. Together they cost about four minutes in a dev build
//! and they are `gated_to!` the tier, its dials and those
//! documents' fixture doors, so a change elsewhere does not pay them. The
//! rest are `#[ignore]`d evidence probes that print and assert nothing
//! a gate could read ([[test-suite-cost]]). Run one document's evidence:
//!
//! ```sh
//! CAD_SYM_9_DOC=r2_link cargo test -p editor-core \
//!   --features interval,geom-core/sym-profile-testing --test all -- \
//!   sym_9_retry_interval:: --ignored --nocapture
//! ```
#![cfg(feature = "interval")]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

// Gated to the ladder and to the documents it is measured on: the tier
// and its rules, the drive's retry dial, the leaf lane that carries it, and the
// fixture doors the two documents that gain from it are built through.
test_utils::gated_to![
    "crates/geom-core/src/sym.rs",
    "crates/geom-core/src/sym/",
    "crates/editor-core/src/drive.rs",
    "crates/editor-core/src/eval/mod.rs",
    "crates/editor-core/tests/m10_7_r2_probes_interval.rs",
    "crates/editor-core/tests/m10_8_arc_family_interval.rs",
    "crates/editor-core/tests/m10_8_harness.rs",
    "crates/editor-core/tests/m10_9_r2_probes_interval.rs",
];

use std::collections::BTreeMap;
use std::sync::Arc;
use std::time::Instant;

use editor_core::analysis::{AnalysisPolicy, ParamBox, analyzed_box};
use editor_core::drive::{DEFAULT_SYM_MAX_DEGREE, DEFAULT_SYM_MAX_TERMS};
use editor_core::{CancelToken, EvalOptions, NodeResult, ProfileDoc, ProfileLift, evaluate};
use geom_core::sym::report::{
    DecisionShape, ShapeOutcome, name_param, start_shape_report, take_shape_report,
};
use geom_core::sym::{SymRetry, with_session_retry};
use geom_core::{SymBudget, SymCounts, SymRules, Tol};

use crate::m10_8_harness::{nominal_box, split};

fn budget() -> SymBudget {
    SymBudget {
        max_terms: DEFAULT_SYM_MAX_TERMS,
        max_degree: DEFAULT_SYM_MAX_DEGREE,
    }
}

/// **The retry SHAPES Phase 1 measures**, each as the ladder that
/// offers exactly it — the spec's two, plus amendment A1's two, plus
/// the rule-by-rule kept-atom shapes.
///
/// `signed_root` is absent on purpose and is a DEVIATION disclosed in
/// the unit's PR: the spec's "re-run with rule C off" is the shipped
/// tier, because `SymRules::shipped` has `signed_root: false` already,
/// so that shape is inert by construction rather than by measurement.
fn shapes() -> Vec<(&'static str, SymRetry)> {
    let all = SymRules::all();
    let without = |f: fn(&mut SymRules)| {
        let mut m = all;
        f(&mut m);
        SymRetry {
            without: [Some(m), None],
            ..SymRetry::none()
        }
    };
    vec![
        (
            "ring_512",
            SymRetry {
                bits: Some(512),
                ..SymRetry::none()
            },
        ),
        (
            "ring_1024",
            SymRetry {
                bits: Some(1024),
                ..SymRetry::none()
            },
        ),
        ("no_abs_square", without(|m| m.abs_square = false)),
        ("no_root_magnitude", without(|m| m.root_magnitude = false)),
        ("no_rule_a", without(|m| m.sqrt_square = false)),
        ("no_rule_e", without(|m| m.common_factor = false)),
        ("no_rule_f", without(|m| m.manifest_sign = false)),
        ("no_rule_g", without(|m| m.canonical_root = false)),
        // **The two that recover, together** — rule A off (the
        // bracket's six) and rule G off (the link's twelve) in ONE
        // kept-atom attempt: the joint mask, measured beside the two
        // single ones to say whether one attempt can carry both.
        (
            "no_rule_a_g",
            without(|m| {
                m.sqrt_square = false;
                m.canonical_root = false;
            }),
        ),
        // **The measured ladder** (`geom_core::SymRetry::kept_atom`),
        // and the same two masks in the other order — the order is a
        // cost, paid on every refusal the first mask does not close.
        ("kept_atom_ladder", SymRetry::kept_atom()),
        (
            "kept_atom_reversed",
            SymRetry {
                without: [
                    SymRetry::kept_atom().without[1],
                    SymRetry::kept_atom().without[0],
                ],
                ..SymRetry::kept_atom()
            },
        ),
        // The measured ladder with a 512-bit ring attempt behind it.
        (
            "kept_atom_then_512",
            SymRetry {
                bits: Some(512),
                ..SymRetry::kept_atom()
            },
        ),
    ]
}

/// The named study `CAD_SYM_9_DOC` selects, from
/// `m10_10_evidence_interval`'s index (default the plate), at the scale
/// `CAD_SYM_9_SCALE` names (default 1).
fn document_from_env(tol: Tol) -> (String, ProfileDoc) {
    let name = std::env::var("CAD_SYM_9_DOC").unwrap_or_else(|_| "two_hole_plate".into());
    let scale: f64 = std::env::var("CAD_SYM_9_SCALE")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(1.0);
    let doc = document(&name, scale, tol);
    (name, doc)
}

/// One named study at one scale — the index the rows here share, the
/// same eight `m10_10_evidence_interval` carries.
fn document(name: &str, scale: f64, tol: Tol) -> ProfileDoc {
    match name {
        "two_hole_plate" => crate::m10_7_plate::plate(5.0e-5 * scale, 1.0e-5 * scale, tol).0,
        "r2_filleted_bracket" => crate::m10_7_r2_probes_interval::bracket(scale, tol).0,
        "r1_annulus" => crate::m10_8_r1_probes_interval::annulus(scale, tol).0,
        "r2_rounded_pad" => crate::m10_8_r2_probes_interval::pad(scale, tol).0,
        "r2_link" => crate::m10_9_r2_probes_interval::link(scale, tol).0,
        "r1_segment_boss" => crate::m10_10_r1_probes_interval::segment_boss(scale, tol).0,
        "r2_d_tab_literal" => crate::m10_10_r2_probes_interval::d_tab(scale, false, tol).0,
        "r2_d_tab_parameter" => crate::m10_10_r2_probes_interval::d_tab(scale, true, tol).0,
        other => panic!("no document {other:?}"),
    }
}

/// One replay at `Sym<Interval>` over `box_` with `retry`'s ladder
/// installed, the shape report on when `report` is.
///
/// The gating row runs with the report off: it reads the receipt and
/// nothing else, and the report renders the plain and early forms of
/// every blocked residual. The evidence rows run with it on unless
/// `CAD_SYM_9_NO_REPORT` is set — the door for R2's rounded pad, whose
/// report the unit's dispatch recorded as exhausting a four-core box's
/// memory (this lane's own two pad runs did not return a first replay
/// with the report on or off:
/// `work/sym/the-pads-nominal-replay-is-not-takeable-on-a-four-core-box`).
/// The counts are the same either way: the report is a recorder, the
/// receipt is the session's own.
fn replay(
    doc: &ProfileDoc,
    box_: &ParamBox,
    retry: SymRetry,
    report: bool,
    tol: Tol,
) -> (Vec<DecisionShape>, SymCounts) {
    for name in box_.axes().keys() {
        name_param(&name.0);
    }
    let opts = EvalOptions {
        param_box: Some(Arc::new(box_.clone())),
        profile_lift: ProfileLift::Guided,
        ..EvalOptions::default()
    };
    if report {
        start_shape_report();
    }
    let (_, counts) = with_session_retry(budget(), SymRules::shipped(), retry, || {
        let ev: editor_core::Evaluation<geom_core::Sym<geom_core::Interval>> =
            evaluate(doc, None, &CancelToken::new(), &opts, tol);
        ev.order.iter().for_each(|id| {
            if let Some(NodeResult::Failed(e)) = ev.result(*id) {
                println!("   (node {} refused — {})", id.0, e.kind);
            }
        });
    });
    (
        if report {
            take_shape_report()
        } else {
            Vec::new()
        },
        counts,
    )
}

/// Whether an evidence row renders the shape report
/// (`CAD_SYM_9_NO_REPORT` unset).
fn reported() -> bool {
    std::env::var("CAD_SYM_9_NO_REPORT").is_err()
}

/// The receipt's four decision columns, in the split's order — the
/// same totals [`totals`] sums out of the shape report, from the
/// session's own counts, so a replay with the report off still prints
/// them.
fn receipt(c: &SymCounts) -> [u64; 4] {
    [c.symbolic_zero, c.sign_gated, c.registered, c.numeric]
}

/// The document's whole per-predicate split, as one printable line per
/// predicate, and its totals.
fn totals(shapes: &[DecisionShape]) -> [u64; 4] {
    let mut out = [0; 4];
    for row in split(shapes).values() {
        for (a, b) in out.iter_mut().zip(row) {
            *a += b;
        }
    }
    out
}

/// **The `numeric` column, taken apart** — `Definite` (the numeric
/// channel certified a non-zero sign and the tier was never asked),
/// `NumericZero` (zero inside the band, the tier asked and declined),
/// `Indeterminate` (the band could not classify, the tier asked and
/// declined) and `Invalid` (a domain violation, the tier never asked).
///
/// It is the split the retry ladder's subject needs: a retry can only
/// reach a decision the tier was ASKED, so the middle two columns are
/// the population and the outer two are not.
fn asked(shapes: &[DecisionShape]) -> [u64; 4] {
    let mut out = [0; 4];
    for s in shapes {
        match s.outcome {
            ShapeOutcome::Definite(_) => out[0] += 1,
            ShapeOutcome::NumericZero => out[1] += 1,
            ShapeOutcome::Indeterminate => out[2] += 1,
            ShapeOutcome::Invalid => out[3] += 1,
            _ => {}
        }
    }
    out
}

/// **PHASE 1, TABLE 1 — where the refusals are**: the rung and attempt
/// that answered every decision of one document at its nominal, and for
/// the decisions every rung refused, the freeze causes their own walks
/// made.
///
/// The profile is `geom_core::sym::profile`, behind the test-only
/// `sym-profile-testing` feature this crate's dev-dependency edge on
/// `geom-core` turns on unconditionally — so the attribution is here
/// in every test build and nowhere in a shipped one.
#[test]
#[ignore = "evidence-only: the rung funnel and the freeze causes of one document's refusals"]
fn sym_9_where_the_refusals_are() {
    let tol = Tol::witness();
    let (name, doc) = document_from_env(tol);
    let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
    let box_ = nominal_box(&analyzed);
    geom_core::sym::profile::start_profile();
    let (shapes, counts) = replay(&doc, &box_, SymRetry::none(), reported(), tol);
    println!("== {name} at the nominal, the shipped tier, no ladder");
    println!(
        "   totals (theorem/gated/registered/numeric) {:?} (report {:?})",
        receipt(&counts),
        totals(&shapes)
    );
    println!("   receipt {counts:?}");
    println!(
        "   the numeric column (definite/numeric-zero/indeterminate/invalid) {:?}",
        asked(&shapes)
    );
    for (pred, row) in &split(&shapes) {
        println!("   {pred:<34} {row:?}");
    }
    let p = geom_core::sym::profile::take_profile();
    println!("-- the rung funnel");
    print!("{}", p.rung_table());
}

/// **PHASE 1, TABLE 2 — what each retry recovers, and what it costs**:
/// one document at its nominal, replayed once with no ladder and once
/// per shape of [`shapes`], with the per-predicate split, the `retried`
/// column and the wall time of each.
///
/// The recovery is read as a DIFFERENCE and that is exact: a retry is
/// asked only into the first attempt's silence, so `numeric` can only
/// fall and every other column can only rise, decision for decision.
#[test]
#[ignore = "evidence-only: what each retry shape recovers on one document, and its cost"]
fn sym_9_what_each_retry_recovers() {
    let tol = Tol::witness();
    let (name, doc) = document_from_env(tol);
    let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
    let box_ = nominal_box(&analyzed);
    let t0 = Instant::now();
    // The profile is installed on EVERY replay here, the base included,
    // so its hooks cost both sides of each ratio alike; what it is read
    // for is `retry_forms`, the retry memos' size.
    geom_core::sym::profile::start_profile();
    let (base_shapes, base_counts) = replay(&doc, &box_, SymRetry::none(), reported(), tol);
    let dag = geom_core::sym::profile::take_profile().nodes;
    let base_time = t0.elapsed();
    let base = split(&base_shapes);
    println!("== {name} at the nominal");
    println!(
        "   no ladder: totals {:?}  retried {}  {:?}",
        receipt(&base_counts),
        base_counts.retried,
        base_time
    );
    println!(
        "   the numeric column (definite/numeric-zero/indeterminate/invalid) {:?}",
        asked(&base_shapes)
    );
    // `CAD_SYM_9_SHAPES` names a comma-separated subset of [`shapes`].
    let only = std::env::var("CAD_SYM_9_SHAPES").ok();
    for (label, retry) in shapes() {
        if only
            .as_deref()
            .is_some_and(|l| !l.split(',').any(|n| n.trim() == label))
        {
            continue;
        }
        let t0 = Instant::now();
        geom_core::sym::profile::start_profile();
        let (shapes, counts) = replay(&doc, &box_, retry, reported(), tol);
        let held = geom_core::sym::profile::take_profile().retry_forms;
        let dt = t0.elapsed();
        let table = split(&shapes);
        let moved: BTreeMap<&str, ([u64; 4], [u64; 4])> = table
            .iter()
            .filter(|(p, row)| base.get(*p).is_none_or(|b| b != *row))
            .map(|(p, row)| (*p, (*base.get(p).unwrap_or(&[0; 4]), *row)))
            .collect();
        println!(
            "   {label:<18} totals {:?}  retried {}  {:?} ({:.2}x)  retry memos {held:?} (DAG {dag})",
            receipt(&counts),
            counts.retried,
            dt,
            dt.as_secs_f64() / base_time.as_secs_f64().max(f64::MIN_POSITIVE)
        );
        for (pred, (was, now)) in &moved {
            println!("      {pred:<32} {was:?} -> {now:?}");
        }
    }
}

/// **THE LADDER'S PIN**: what `SymRetry::kept_atom` recovers, per
/// document, asserted on both sides — and on R2's link, the two
/// predicates it recovers, at their rows: `carrier_on_surface_2`
/// `[82, 0, 6, 20]` → `[92, 0, 6, 10]` (the ten decisions rule G costs,
/// `decide_3_split_rows_interval` pins that trade one attempt per rung)
/// and `witness_on_surface_2` `[14, 0, 0, 2]` → `[16, 0, 0, 0]`.
///
/// It pins the two things the acceptance asks for and nothing else. On
/// the two documents that gain, the whole split with the ladder against
/// the same replay without it, so a decision that moved DOWN reds; and
/// the `retried` column at its measured count, so a retry that stops
/// carrying them reds even if something else picks them up. On the
/// three that do not gain, `retried` is pinned at ZERO — which is the
/// claim that the ladder is not quietly paying for itself somewhere
/// unmeasured.
///
/// The `numeric` column can only FALL and the other three can only
/// rise: a retry is asked only into the first attempt's silence
/// (`geom_core::SymRetry`). A row here that moved the other way is a
/// defect in the ladder, not a re-baseline.
#[test]
fn sym_9_the_kept_atom_ladder_recovers_what_phase_1_measured() {
    let tol = Tol::witness();
    let ladder = SymRetry::kept_atom();
    // `(document, the receipt without the ladder, with it, retried)`.
    let expected: [(&str, [u64; 4], [u64; 4], u64); 5] = [
        ("two_hole_plate", [811, 0, 140, 462], [811, 0, 140, 462], 0),
        ("r1_annulus", [328, 0, 140, 209], [328, 0, 140, 209], 0),
        ("r1_segment_boss", [374, 2, 96, 234], [374, 2, 96, 234], 0),
        (
            "r2_filleted_bracket",
            [1104, 7, 144, 766],
            [1104, 7, 150, 760],
            6,
        ),
        ("r2_link", [541, 0, 96, 465], [553, 0, 96, 453], 12),
    ];
    let mut moved: Vec<String> = Vec::new();
    for (name, want_off, want_on, want_retried) in expected {
        let doc = document(name, 1.0, tol);
        let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
        let box_ = nominal_box(&analyzed);
        let (_, off) = replay(&doc, &box_, SymRetry::none(), false, tol);
        // The shape report is on for ONE replay, the link's with the
        // ladder: the predicate rule G trades is pinned here at the row
        // the ladder recovers it to, and a split is read off the report.
        let link = name == "r2_link";
        let (shapes, on) = replay(&doc, &box_, ladder, link, tol);
        if link {
            let t = split(&shapes);
            for (pred, want) in [
                ("carrier_on_surface_2", [92, 0, 6, 10]),
                ("witness_on_surface_2", [16, 0, 0, 0]),
            ] {
                let got = t.get(pred).copied().unwrap_or([0; 4]);
                if got != want {
                    moved.push(format!("{name}/{pred} with the ladder {want:?} -> {got:?}"));
                }
            }
        }
        println!(
            "== {name}: {:?} -> {:?} (retried {})",
            receipt(&off),
            receipt(&on),
            on.retried
        );
        if receipt(&off) != want_off {
            moved.push(format!(
                "{name} without the ladder {want_off:?} -> {:?}",
                receipt(&off)
            ));
        }
        if receipt(&on) != want_on {
            moved.push(format!(
                "{name} with the ladder {want_on:?} -> {:?}",
                receipt(&on)
            ));
        }
        if on.retried != want_retried {
            moved.push(format!("{name} retried {want_retried} -> {}", on.retried));
        }
        assert_eq!(
            off.retried, 0,
            "{name}: `SymRetry::none()` installs no ladder, so nothing can be retried"
        );
    }
    assert!(
        moved.is_empty(),
        "the ladder's measured recovery moved (theorem/gated/registered/numeric). A row that \
         moved UP is re-baselined and said; one that moved DOWN is a retry taking a decision \
         away, which `SymRetry` says cannot happen: {moved:?}"
    );
}

/// **The DRIVE carries the ladder, and its receipt says so**: one
/// whole-box leaf of R2's filleted bracket at `1e1·ε` under
/// `SymbolicDials::default()` — the shipped ladder, through the drive's
/// own door (`sym::with_session_memo_retry`, with the drive's plain
/// memo installed) — against the same leaf with `SymRetry::none()`.
///
/// It reads both receipts the drive writes: the goldening line carries
/// `retried=6` after the discharge columns, and the human form names
/// the six as discharges a second attempt reached, not as a clause of
/// `registered`'s. Without the ladder neither appears and the line is
/// the one a drive wrote before the ladder existed. The six themselves
/// are the rule-A attempt's, and they are `registered` (144 → 150).
#[test]
fn sym_9_the_drive_writes_the_ladders_receipt() {
    let tol = Tol::witness();
    let doc = document("r2_filleted_bracket", 1.0e1 * tol.eps(), tol);
    let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
    let run = |symbolic: editor_core::drive::SymbolicDials| {
        editor_core::drive::drive(
            &doc,
            &analyzed,
            &editor_core::drive::DriveConfig {
                max_depth: 0,
                max_leaves: 1,
                symbolic,
                ..editor_core::drive::DriveConfig::default()
            },
            tol,
        )
        .expect("the bracket drives")
    };
    let shipped = run(editor_core::drive::SymbolicDials::default());
    let bare = run(editor_core::drive::SymbolicDials {
        retry: SymRetry::none(),
        ..editor_core::drive::SymbolicDials::default()
    });
    let (line, human) = (shipped.serialize(), shipped.render(&analyzed));
    println!("{line}\n{human}");
    assert_eq!(shipped.receipt().certified, 1, "the leaf certifies whole");
    assert_eq!(
        bare.receipt().certified,
        1,
        "with the ladder and without it"
    );
    let d = shipped.decisions();
    assert_eq!(
        [
            d.symbolic_zero,
            d.sign_gated,
            d.registered,
            d.numeric,
            d.retried
        ],
        [1104, 7, 150, 760, 6],
        "the shipped ladder's leaf receipt"
    );
    assert!(
        line.contains("registered=150 retried=6\n"),
        "the goldening line carries `retried=` after the discharge columns: {line}"
    );
    assert!(
        human.contains("; 6 of those discharges reached only by a second attempt"),
        "the human form names the six as the ladder's: {human}"
    );
    let b = bare.decisions();
    assert_eq!(
        [
            b.symbolic_zero,
            b.sign_gated,
            b.registered,
            b.numeric,
            b.retried
        ],
        [1104, 7, 144, 766, 0]
    );
    assert!(
        !bare.serialize().contains("retried="),
        "no ladder, no column"
    );
    assert!(!bare.render(&analyzed).contains("second attempt"));
}
