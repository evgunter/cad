//! **Where the E12 tier's time goes** — the cost profile of the
//! symbolic normal form on the M10-3 slab and the two-hole plate
//! (`work/sym/symbolic-tier-costs-95-percent-of-the-m10-3-drive`).
//!
//! The `#[ignore]`d rows are EVIDENCE-ONLY: each
//! prints the tier's structural profile (`geom_core::sym::profile`,
//! behind the test-only `sym-profile-testing` feature the
//! dev-dependency edge turns on) for one replay or one drive, and
//! asserts nothing about the numbers — they are the record, read into
//! the item body and the tier's `# Cost` section, and re-taken by
//! re-running the row. The fixtures are the ones the M10-3 suite
//! drives (`bounded_chamber`, the row S-TCOST bisected on) and the
//! plate the tier was built for, through the same doors.
//!
//! The gating rows read the same instrument for the one thing in it
//! that is not a clock: the walk ledger — per walk and origin the
//! calls, the forms, the frozen, and the digest chain of the forms
//! built ([`walk_ledger`]). What they hold of it is what no intended
//! change moves: a replay builds the same ledger every time, the slab
//! freezes nothing, no form passes the growth guard, and no dial of
//! the early walk moves a line of the plain walk.
//!
//! The one row without the profile installed,
//! [`sym_profile_callgrind_replay`], is the callgrind target: it
//! replays one box a chosen number of times and nothing else, so an
//! instruction count over it is the tier's and not the instrument's.
//! The command that takes it is recorded in the item body.
//!
//! **This file is ungated**: it carries no `gated_to!` marker, so every
//! row here outside the `ci` slow set runs on every PR whose closure
//! reaches editor-core. A new fast row inherits that.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::collections::BTreeMap;
use std::time::Instant;

use editor_core::ProfileDoc;
use editor_core::analysis::{AnalysisPolicy, BoxAxis, ParamBox, analyzed_box};
use editor_core::drive::{DriveConfig, drive};
use geom_core::sym::profile::{start_profile, take_profile};
use geom_core::sym::report::DecisionShape;
use geom_core::{SymRules, Tol};

use crate::m10_3_r1_probes_interval::{CHAMBER_LEAVES, bounded_chamber};
use crate::m10_7_plate::plate;
use crate::m10_8_arc_family_interval::replay;
use crate::m10_8_harness::nominal_box;
use editor_core::drive::{DEFAULT_SYM_MAX_DEGREE, DEFAULT_SYM_MAX_TERMS};
use editor_core::{CancelToken, EvalOptions, evaluate};
use geom_core::SymBudget;
use geom_core::sym::with_session_rules;
use std::sync::Arc;

/// One replay at `Sym<Interval>` over `box_` with NOTHING installed —
/// no shape report (which renders every blocked residual through the
/// walks, ~1 % of a replay's instructions) — so a callgrind count over
/// it is the tier's and the numeric channel's alone.
fn bare_replay(doc: &editor_core::ProfileDoc, box_: &ParamBox, tol: Tol) -> geom_core::SymCounts {
    let opts = EvalOptions {
        param_box: Some(Arc::new(box_.clone())),
        ..EvalOptions::default()
    };
    let budget = SymBudget {
        max_terms: DEFAULT_SYM_MAX_TERMS,
        max_degree: DEFAULT_SYM_MAX_DEGREE,
    };
    let (_, counts) = with_session_rules(budget, SymRules::shipped(), || {
        let ev: editor_core::Evaluation<geom_core::Sym<geom_core::Interval>> =
            evaluate(doc, None, &CancelToken::new(), &opts, tol);
        ev.order.len()
    });
    counts
}

fn eps() -> f64 {
    Tol::witness().eps()
}

/// **The M10-3 slab**: the bounded chamber at the dimensions S-TCOST
/// bisected the tier's cost on.
fn slab() -> ProfileDoc {
    bounded_chamber(60.0 * eps(), 30.0 * eps(), 100.0 * eps())
}

/// **The two-hole plate at its real study** (the M10-8 nominal that
/// froze 1,056 forms).
fn the_plate(tol: Tol) -> ProfileDoc {
    plate(5.0e-5, 1.0e-5, tol).0
}

/// The three scales a replay is read at: the degenerate box at the
/// nominal (every form built); a LEAF-sized box — every axis `± ε`
/// about its nominal, the width at which the M10-3 drive's leaves
/// certify, so the evaluation runs to the end and the forms are
/// built only where the numeric channel does not answer; and the
/// whole analyzed box, which on the slab refuses at its second node
/// (the extrusion vector straddles zero over it) and is kept as the
/// record of that.
fn boxes(doc: &ProfileDoc) -> [(&'static str, ParamBox); 3] {
    let analyzed = analyzed_box(doc, &AnalysisPolicy::default());
    let root = ParamBox::of(&analyzed);
    let leaf = ParamBox::from_axes(
        root.axes()
            .iter()
            .map(|(n, a)| {
                let m = a.midpoint();
                (
                    *n,
                    BoxAxis::Varying {
                        lo: m - eps(),
                        hi: m + eps(),
                    },
                )
            })
            .collect(),
    );
    [
        ("nominal", nominal_box(&analyzed)),
        ("leaf", leaf),
        ("root", root),
    ]
}

fn outcomes(shapes: &[DecisionShape]) -> BTreeMap<String, usize> {
    let mut out = BTreeMap::new();
    for s in shapes {
        *out.entry(format!("{:?}", s.outcome)).or_default() += 1;
    }
    out
}

/// One replay over `box_` with the profile installed; prints the
/// decision split, the counts and the profile.
fn profiled_replay(label: &str, doc: &ProfileDoc, box_: &ParamBox, tol: Tol) {
    start_profile();
    let t0 = Instant::now();
    let (shapes, refusal, counts) = replay(doc, box_, SymRules::shipped(), tol);
    let wall = t0.elapsed();
    let profile = take_profile();
    println!("=== {label}: wall {wall:?}");
    println!("decisions {:?}", outcomes(&shapes));
    println!("counts {counts:?}");
    if let Some(r) = refusal {
        println!("first refusal: {r}");
    }
    assert_eq!(profile.unnoted(), 0, "a refusal site without a note");
    print!("{}", profile.render());
}

/// **The slab, one replay at each scale** — the nominal (every form
/// built), a leaf-sized box, and the whole analyzed box ([`boxes`]).
#[test]
#[ignore = "evidence-only: prints the tier's cost profile of one slab replay at three scales"]
fn sym_profile_slab_replays() {
    let tol = Tol::witness();
    let doc = slab();
    for (scale, box_) in boxes(&doc) {
        profiled_replay(&format!("slab {scale}"), &doc, &box_, tol);
    }
}

/// **The slab, driven whole** at the M10-3 row's own leaf budget,
/// sequentially, with the profile on for every leaf — the population
/// over the entire row S-TCOST measured rather than one replay of it.
#[test]
#[ignore = "evidence-only: prints the tier's cost profile summed over the M10-3 chamber drive"]
fn sym_profile_slab_drive() {
    let tol = Tol::witness();
    let doc = slab();
    let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
    let config = DriveConfig {
        max_leaves: CHAMBER_LEAVES,
        ..DriveConfig::default()
    };
    start_profile();
    let t0 = Instant::now();
    let v = drive(&doc, &analyzed, &config, &editor_core::Stop::default(), tol).unwrap();
    let wall = t0.elapsed();
    let profile = take_profile();
    println!(
        "=== slab drive at {CHAMBER_LEAVES} leaves: wall {wall:?} receipt {:?} decisions {:?}",
        v.receipt(),
        v.decisions()
    );
    assert_eq!(profile.unnoted(), 0, "a refusal site without a note");
    print!("{}", profile.render());
}

/// **The plate at its nominal**, the freeze population only — the
/// comparison point on the document the tier was built for.
#[test]
#[ignore = "evidence-only: prints the tier's cost profile of one plate replay at the nominal"]
fn sym_profile_plate_nominal() {
    let tol = Tol::witness();
    let doc = the_plate(tol);
    let (_, nominal) = boxes(&doc).into_iter().next().unwrap();
    profiled_replay("plate nominal", &doc, &nominal, tol);
}

/// **The callgrind target**: `CAD_SYM_PROFILE_DOC` (`slab` | `plate`,
/// default `slab`) replayed over `CAD_SYM_PROFILE_BOX` (`nominal` |
/// `leaf` | `root`, default `nominal`) `CAD_SYM_PROFILE_REPEATS` times
/// (default 1), neither the profile nor the shape report installed
/// ([`bare_replay`]), so an instruction count over the process is the
/// tier's own. Prints the counts per replay.
#[test]
#[ignore = "evidence-only: the callgrind target — replays one box, nothing else"]
fn sym_profile_callgrind_replay() {
    let tol = Tol::witness();
    let doc = match std::env::var("CAD_SYM_PROFILE_DOC").as_deref() {
        Ok("plate") => the_plate(tol),
        _ => slab(),
    };
    let which = std::env::var("CAD_SYM_PROFILE_BOX").unwrap_or_else(|_| "nominal".into());
    let repeats: usize = std::env::var("CAD_SYM_PROFILE_REPEATS")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(1);
    let (scale, box_) = boxes(&doc)
        .into_iter()
        .find(|(s, _)| *s == which)
        .expect("CAD_SYM_PROFILE_BOX is `nominal`, `leaf` or `root`");
    for i in 0..repeats {
        let t0 = Instant::now();
        let counts = bare_replay(&doc, &box_, tol);
        println!(
            "replay {i} {scale}: wall {:?} counts {counts:?}",
            t0.elapsed()
        );
    }
}

/// **The growth guard**: the largest form (numerator plus denominator
/// terms) any op may build on each document at its nominal, at twice
/// the largest measured. A representation change that blows a form up
/// reds here in seconds, where the walks would first run for minutes;
/// a form that shrinks, or grows by less than its measured size, moves
/// nothing.
const SLAB_MAX_TERMS: usize = 8;
const PLATE_MAX_TERMS: usize = 76;

/// One replay of `doc` at its nominal with the profile installed: its
/// walk ledger — per walk and origin the calls, the forms, the frozen,
/// and the digest chain of the forms built (`WalkProfile::digest`) —
/// and the largest form any op built.
fn walk_ledger(name: &str, doc: &ProfileDoc, tol: Tol) -> (String, usize) {
    let (_, nominal) = boxes(doc).into_iter().next().unwrap();
    start_profile();
    let _ = replay(doc, &nominal, SymRules::shipped(), tol);
    let profile = take_profile();
    let ledger = profile.walk_ledger();
    let largest = profile
        .ops
        .values()
        .map(|o| o.max_terms_out)
        .max()
        .unwrap_or(0);
    println!(
        "\nLEDGER {name} eps {:e} largest form {largest}\n{ledger}",
        tol.eps()
    );
    (ledger, largest)
}

/// The frozen count of each line of a walk ledger, by its line's
/// `walk/origin` label.
fn frozen_by_line(ledger: &str) -> Vec<(String, u64)> {
    ledger
        .lines()
        .filter_map(|l| {
            let label = l.split_whitespace().next()?.to_owned();
            let frozen = l.split(" frozen ").nth(1)?.split_whitespace().next()?;
            Some((label, frozen.parse().ok()?))
        })
        .collect()
}

/// **The slab's walks are a function of the replay, freeze nothing,
/// and stay under the growth guard.** Two replays of the slab at its
/// nominal build the same ledger, digest chains included: a form's
/// storage may change, and the forms a replay builds, in the order it
/// builds them, may not differ from one replay to the next. The slab is
/// a polygon, so no form on any walk passes the budget and freezes.
/// One replay is cheap enough for the per-PR gate, so it stands in
/// there for [`the_plates_largest_form_is_under_the_growth_guard`] and
/// [`the_plains_ledger_lines_are_the_same_under_every_dial_set`], which
/// replay the plate and run nightly.
#[test]
fn the_slabs_walks_build_one_ledger_and_freeze_nothing() {
    let tol = Tol::witness();
    let (first, largest) = walk_ledger("slab", &slab(), tol);
    let (again, _) = walk_ledger("slab", &slab(), tol);
    assert_eq!(
        first,
        again,
        "two replays of the slab at eps {:e} built different forms",
        tol.eps()
    );
    let frozen = frozen_by_line(&first);
    assert!(
        !frozen.is_empty() && frozen.iter().all(|(_, n)| *n == 0),
        "the slab's walks freeze nothing at eps {:e}: {frozen:?}",
        tol.eps()
    );
    assert!(
        largest <= SLAB_MAX_TERMS,
        "the slab's largest form has {largest} terms, past the growth guard {SLAB_MAX_TERMS}"
    );
}

/// The plate's largest form under the growth guard ([`PLATE_MAX_TERMS`]).
/// The holes' plane mates place by a reach read off the faces' cover
/// (`topo::cover`), and the largest form is one of its norms, so a
/// change to how a cover is read is what moves it.
///
/// **And the cap apex's lines freeze nothing** — the witness of an open
/// row, not a baseline: rule D folds `sin` and `cos` of `q·atan X` but
/// not `tan`, and a cap apex read through `tan(|Δθ|/4)` froze 104 nodes
/// on each of `Early/Assertion` and `Door/Decision`
/// (`work/sym/rule-d-leaves-tan-of-atan-opaque-and-the-cap-apex-mints-it.md`).
/// The apex reads `Arc2::quarter_tan` now; a reader that mints the
/// `tan` atom again reds here. It goes when that row closes.
#[test]
fn the_plates_largest_form_is_under_the_growth_guard() {
    let tol = Tol::witness();
    let (ledger, largest) = walk_ledger("plate", &the_plate(tol), tol);
    let apex: Vec<_> = frozen_by_line(&ledger)
        .into_iter()
        .filter(|(label, _)| label == "Early/Assertion" || label == "Door/Decision")
        .collect();
    assert!(
        apex.len() == 2 && apex.iter().all(|(_, n)| *n == 0),
        "the cap apex's lines freeze nothing on the plate: {apex:?}"
    );
    assert!(
        largest <= PLATE_MAX_TERMS,
        "the plate's largest form has {largest} terms, past the growth guard {PLATE_MAX_TERMS}"
    );
}

/// **The walk ledger on the documents the gating rows do not
/// measure** — the M10-10 evidence set at scale 1 (the plate, the two
/// brackets, the annulus, the pad, the link), at the nominal and over
/// a leaf-sized box — printed, not asserted, so a merge-base build and
/// a head build of one row can be diffed: every counter and every
/// digest chain must agree. From SYM-4's reviews (both reviewers ran
/// this differential: `origin/sym/4-review-r2`'s row, with
/// `origin/sym/4-review-r1`'s four-document twin), which is the
/// coverage the unit's own record did not claim.
#[test]
#[ignore = "evidence-only: the walk ledger on six documents, for a merge-base differential"]
fn the_walk_ledger_on_the_unmeasured_documents() {
    let tol = Tol::witness();
    let docs: Vec<(&str, ProfileDoc)> = vec![
        ("two_hole_plate", the_plate(tol)),
        (
            "r2_filleted_bracket",
            crate::m10_7_r2_probes_interval::bracket(1.0, tol).0,
        ),
        (
            "r1_bracket",
            crate::m10_8_arc_family_interval::documents(tol)
                .pop()
                .map(|(_, d)| d)
                .unwrap(),
        ),
        (
            "r1_annulus",
            crate::m10_8_r1_probes_interval::annulus(1.0, tol).0,
        ),
        (
            "r2_rounded_pad",
            crate::m10_8_r2_probes_interval::pad(1.0, tol).0,
        ),
        ("r2_link", crate::m10_9_r2_probes_interval::link(1.0, tol).0),
    ];
    for (name, doc) in &docs {
        for (scale, box_) in boxes(doc) {
            if scale == "root" {
                continue;
            }
            start_profile();
            let t0 = Instant::now();
            let (shapes, refusal, counts) = replay(doc, &box_, SymRules::shipped(), tol);
            let wall = t0.elapsed();
            let prof = take_profile();
            println!(
                "LEDGER {name} {scale} eps {:e} counts {counts:?} refusal {refusal:?} outcomes {:?} nodes {} atoms {} freezes {} rat_ops {} big_ops {} promotions {} widest {} refused {} (wall {wall:?}, local)\n{}",
                tol.eps(),
                outcomes(&shapes),
                prof.nodes,
                prof.atoms,
                prof.freezes.len(),
                prof.rat_ops,
                prof.big_ops,
                prof.promotions,
                prof.widest_bits,
                prof.widest_refused_bits,
                prof.walk_ledger()
            );
        }
    }
}

/// **No dial of the early walk moves a line of the plain walk.** The
/// row replays the plate under each dial set and asserts that the PLAIN
/// lines of its ledger — calls, forms, frozen and digest chain — are
/// identical across all of them, so a claim that a rule of the early
/// walk moved a plain form reds here first.
#[test]
fn the_plains_ledger_lines_are_the_same_under_every_dial_set() {
    let tol = Tol::witness();
    let doc = the_plate(tol);
    let (_, nominal) = boxes(&doc).into_iter().next().unwrap();
    // `Plain/Report` is not a form the plain walk builds: it counts the
    // residuals rendered because they BLOCKED, and which ones block is
    // the early walk's rules' to decide. So it is the one plain line a
    // dial may move: the sets with the canonical root render none, and
    // the sets without it render the certificate's blocked reads.
    let is_report = |l: &str| l.trim_start().starts_with("Plain/Report");
    let plain_lines = |ledger: &str| {
        ledger
            .lines()
            .filter(|l| l.trim_start().starts_with("Plain/") && !is_report(l))
            .map(|l| l.trim().to_owned())
            .collect::<Vec<_>>()
    };
    let report_calls = |ledger: &str| {
        ledger
            .lines()
            .find(|l| is_report(l))
            .and_then(|l| l.split_whitespace().nth(2))
            .and_then(|n| n.parse::<u64>().ok())
    };
    let sets: [(&str, SymRules, bool); 4] = [
        ("shipped", SymRules::shipped(), false),
        (
            "without_canonical_root",
            SymRules::without_canonical_root(),
            true,
        ),
        ("without_the_reads", SymRules::without_the_reads(), false),
        (
            "both new dials off",
            SymRules {
                decision_read: false,
                ..SymRules::without_canonical_root()
            },
            true,
        ),
    ];
    let mut seen: Option<(&str, Vec<String>)> = None;
    for (name, rules, blocked) in sets {
        start_profile();
        let _ = replay(&doc, &nominal, rules, tol);
        let p = take_profile();
        let largest = p.ops.values().map(|o| o.max_terms_out).max().unwrap_or(0);
        let ledger = p.walk_ledger();
        let lines = plain_lines(&ledger);
        println!(
            "  {name}: largest form {largest}, Plain/Report {:?}\n    {}",
            report_calls(&ledger),
            lines.join("\n    ")
        );
        assert_eq!(
            report_calls(&ledger).is_some_and(|n| n > 0),
            blocked,
            "{name}: the plain walk renders blocked residuals where the canonical root is off, \
             and none where it is on: {:?}",
            report_calls(&ledger)
        );
        match &seen {
            None => seen = Some((name, lines)),
            Some((first, want)) => assert_eq!(
                &lines, want,
                "{name} builds a different PLAIN walk from {first}: the plain form reads no \
                 dial of the early walk, so a difference here is a defect and not a \
                 re-baseline"
            ),
        }
    }
}
