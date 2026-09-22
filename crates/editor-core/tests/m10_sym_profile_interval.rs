//! **Where the E12 tier's time goes** — the cost profile of the
//! symbolic normal form on the M10-3 slab and the two-hole plate
//! (`work/sym/symbolic-tier-costs-95-percent-of-the-m10-3-drive`).
//!
//! Every row here but one is EVIDENCE-ONLY and `#[ignore]`d: each
//! prints the tier's structural profile (`geom_core::sym::profile`,
//! behind the test-only `sym-profile-testing` feature the
//! dev-dependency edge turns on) for one replay or one drive, and
//! asserts nothing about the numbers — they are the record, read into
//! the item body and the tier's `# Cost` section, and re-taken by
//! re-running the row. The fixtures are the ones the M10-3 suite
//! drives (`bounded_chamber`, the row S-TCOST bisected on) and the
//! plate the tier was built for, through the same doors.
//!
//! The one gating row,
//! [`the_forms_the_walks_build_are_pinned_per_eps_row`], reads the
//! same instrument for the one thing in it that is not a clock: the
//! walk ledger — per walk and origin the calls, the forms, the frozen,
//! and the digest chain of the forms built — so what the tier BUILDS
//! on both documents is a pinned number, over and above what the
//! pins say it DECIDES.
//!
//! The one row without the profile installed,
//! [`sym_profile_callgrind_replay`], is the callgrind target: it
//! replays one box a chosen number of times and nothing else, so an
//! instruction count over it is the tier's and not the instrument's.
//! The command that takes it is recorded in the item body.
#![cfg(feature = "interval")]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

// Gated to the code it profiles: the normal form and its rules, the
// driver that replays leaves through it, and the fixtures — the shared
// tree and the four sibling suites whose doors build the documents and
// run the replay.
test_utils::gated_to![
    "crates/geom-core/src/sym.rs",
    "crates/geom-core/src/sym/",
    "crates/editor-core/src/drive.rs",
    "crates/editor-core/tests/fixture/",
    "crates/editor-core/tests/m10_3_r1_probes_interval.rs",
    "crates/editor-core/tests/m10_7_plate.rs",
    "crates/editor-core/tests/m10_8_arc_family_interval.rs",
    "crates/editor-core/tests/m10_8_harness.rs",
];

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
use editor_core::{CancelToken, EvalOptions, ProfileLift, evaluate};
use geom_core::SymBudget;
use geom_core::sym::with_session_rules;
use std::sync::Arc;

/// One replay at `Sym<Interval>` over `box_` with NOTHING installed —
/// no shape report (which renders every blocked residual through the
/// walks, ~1 % of a replay's instructions) — so a callgrind count over
/// it is the tier's and the numeric channel's alone.
fn bare_replay(doc: &ProfileDoc, box_: &ParamBox, tol: Tol) -> geom_core::SymCounts {
    let opts = EvalOptions {
        param_box: Some(Arc::new(box_.clone())),
        profile_lift: ProfileLift::Guided,
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

/// **The M10-3 slab**: the bounded chamber exactly as
/// `the_driven_chamber_replays_bit_identically_names_both_wall_flips_and_reports_containment`
/// builds it — the row S-TCOST bisected the tier's cost on.
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
                    n.clone(),
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
    let v = drive(&doc, &analyzed, &config, tol).unwrap();
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

/// The ε row this run is on, as the index into a three-row table
/// (`1e-6`, `1e-9`, `1e-12`); any other ε has no captured row and
/// fails loud rather than reading a neighbour's.
fn eps_row(eps: f64) -> usize {
    [1.0e-6, 1.0e-9, 1.0e-12]
        .iter()
        .position(|&e| (eps / e - 1.0).abs() < 1.0e-3)
        .unwrap_or_else(|| panic!("no captured row at eps = {eps:e}: capture one and add it"))
}

/// The slab's walk ledger at its nominal, per ε row.
const SLAB_LEDGER: [&str; 3] = [
    "\
     Plain/Decision calls 980 forms 9686 frozen 0 digest 4c206fa8091829f2e72e255bfcf8cb34\n\
     Plain/Assertion calls 510 forms 918 frozen 0 digest 9a5a90ce2fb285a663e9cb3773b3fb8d\n\
     Early/Decision calls 16 forms 36 frozen 0 digest 6e3af4a8ba2d62d438237e2adb6a8a9d\n\
     Early/Assertion calls 510 forms 1958 frozen 0 digest ec472ae73ea4c7420d838e1560bb36d0",
    "\
     Plain/Decision calls 980 forms 9686 frozen 0 digest 68a31dec794118be1e5494c295c01a77\n\
     Plain/Assertion calls 510 forms 918 frozen 0 digest dc273a096929ffb480ee3ac3734fcf6e\n\
     Early/Decision calls 16 forms 36 frozen 0 digest 6e3af4a8ba2d62d438237e2adb6a8a9d\n\
     Early/Assertion calls 510 forms 1958 frozen 0 digest e83eae7723869354725ac1ce959e7302",
    "\
     Plain/Decision calls 980 forms 9686 frozen 0 digest b2316116afff13c352e218269a06ec67\n\
     Plain/Assertion calls 510 forms 918 frozen 0 digest 03d710606e809b65dc34948ac3a0d5b9\n\
     Early/Decision calls 16 forms 36 frozen 0 digest 6e3af4a8ba2d62d438237e2adb6a8a9d\n\
     Early/Assertion calls 510 forms 1958 frozen 0 digest 171de8349a6fabdbdc04441d2abb73b7",
];

/// The largest form (numerator plus denominator terms) any op built
/// on each document at its nominal — the growth guard: a
/// representation change that grows forms reds here in seconds,
/// where the ledger alone would first run for minutes.
///
/// **Both moved with SYM-5's rule E** (the quotient's common factor),
/// and in opposite directions, which is the rule's shape: the slab's
/// largest 10 → 6, because the rule divides the shared factor out of
/// every form it is given and can only SHRINK that form; the plate's
/// 90 → 288, because a form the rule brings back under the budget is
/// one that no longer FREEZES to a one-term indeterminate, and its
/// consumers then build what the freeze used to cut off (the plate's
/// `Early/Decision` frozen falls 48 → 8 in the ledger below). The rule
/// never grows a form; the WALK's largest form grows because fewer
/// forms are cut short.
///
/// **The plate's 288 → 252 is NOT this unit's**, and the row above is
/// what says so: the plain walk consults no dial of the early walk,
/// the plate's largest form is 252 under `shipped`, under
/// `without_canonical_root`, under `without_the_reads` and under both
/// shut, and the two `Plain/*` digests are identical across all four.
/// Both numbers were stale against this pin before DECIDE-3's branch
/// was cut — its own Phase 1 baseline records this row RED on the base
/// with "every count identical, every digest moved" — so what changed
/// them is a base change between the pin's last capture and the cut,
/// and DECIDE-3 re-pins them without claiming them.
///
/// **What IS rule G's, by the same differential**: the plate's
/// `Early/Assertion` and `Door/Decision` freezes fall 104 → 0, and the
/// `Early/*` and `Door/*` digests move. **What A0's new fold is**: the
/// slab's `Early/*` digests, and the `*/Report` rows absent on both
/// documents — the sites the shape report used to be called on are
/// comparisons of two rational CONSTANTS, and A0 decides those exactly
/// now (`work/decide/a0-leaves-max-and-min-of-constants-opaque`), so
/// there is nothing left for the instrument to report.
const SLAB_MAX_TERMS: usize = 6;
const PLATE_MAX_TERMS: usize = 252;

/// The plate's walk ledger at its nominal — one row, because the
/// plate's nominal reads no ε (its dimensions are literals, not
/// multiples of ε) and the captures at the three rows agree.
const PLATE_LEDGER: &str = "\
     Plain/Decision calls 951 forms 15030 frozen 672 digest aa581ad9960b7ef704f978c1bb2d7ce3\n\
     Plain/Assertion calls 462 forms 2594 frozen 372 digest 702ce928fd05aeeb766afa8c0ae157bb\n\
     Early/Decision calls 320 forms 7979 frozen 8 digest 5274cafc352ceecaf7e8fef294baa3f0\n\
     Early/Assertion calls 462 forms 3406 frozen 0 digest 4bcf903191f8a0a9db825a2798908440\n\
     Door/Decision calls 330 forms 11884 frozen 0 digest 90c99c9e033c9e0adf0032dacb84aee1\n\
     Door/Assertion calls 190 forms 0 frozen 0 digest 00000000000000000000000000000000";

/// **What the walks BUILD is pinned, not only what the tier decides.**
/// For the slab and the plate at their nominals, every (walk, origin)
/// line of the profile's walk ledger — calls, forms, frozen, and the
/// digest chain of the forms themselves (`WalkProfile::digest`: each
/// memoized form's canonical digest, the key an atom over it is minted
/// under, chained in build order). The ledger was captured on the tree
/// that held a form as a `BTreeMap` of terms and is asserted since: a
/// form's storage may change, and what it SAYS — every coefficient,
/// every term in the map's order, hence every atom key and every
/// decision — may not. The pins hold the decisions; this row holds
/// the forms behind them, so a swapped term order or a coefficient
/// normalised differently reads here even where the outcome survives
/// it. The slab's ledger is per ε row, because its literals are
/// multiples of ε.
#[test]
fn the_forms_the_walks_build_are_pinned_per_eps_row() {
    let tol = Tol::witness();
    let row = eps_row(tol.eps());
    let slab_doc = slab();
    let plate_doc = the_plate(tol);
    // Both ledgers are taken before either is read, so a red on the
    // slab still shows the plate's.
    let taken: Vec<_> = [
        ("slab", &slab_doc, SLAB_LEDGER[row], SLAB_MAX_TERMS),
        ("plate", &plate_doc, PLATE_LEDGER, PLATE_MAX_TERMS),
    ]
    .into_iter()
    .map(|(name, doc, expected, max_terms)| {
        let (_, nominal) = boxes(doc).into_iter().next().unwrap();
        start_profile();
        let (_, _, counts) = replay(doc, &nominal, SymRules::shipped(), tol);
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
        (name, ledger, expected, counts, largest, max_terms)
    })
    .collect();
    for (name, ledger, expected, counts, largest, max_terms) in taken {
        assert_eq!(
            largest,
            max_terms,
            "{name} at eps {:e}: the largest form built has {largest} terms, pinned {max_terms}",
            tol.eps()
        );
        assert_eq!(
            ledger.trim(),
            expected.trim(),
            "{name} at eps {:e}: the walks built different forms (counts {counts:?})",
            tol.eps()
        );
    }
}

/// **The walk ledger on the documents the pinned row does not
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

/// **Which dial moves which line of the plate's ledger.** The unit
/// that shipped rule G re-baselined the plate's `PLATE_MAX_TERMS` and
/// both of its `Plain/*` digests and said rule G had moved them; it
/// had not — the plain walk consults neither new dial, and the two
/// numbers were stale against the pin before this branch was cut. This
/// row is what keeps the attribution honest: it replays the plate under
/// each dial set and asserts that the PLAIN lines are identical across
/// all of them, so a future claim that a rule of the early walk moved
/// a plain form reds here first.
#[test]
fn the_plains_ledger_lines_are_the_same_under_every_dial_set() {
    let tol = Tol::witness();
    let doc = the_plate(tol);
    let (_, nominal) = boxes(&doc).into_iter().next().unwrap();
    let plain_lines = |ledger: &str| {
        ledger
            .lines()
            .filter(|l| l.trim_start().starts_with("Plain/"))
            .map(|l| l.trim().to_owned())
            .collect::<Vec<_>>()
    };
    let sets: [(&str, SymRules); 4] = [
        ("shipped", SymRules::shipped()),
        ("without_canonical_root", SymRules::without_canonical_root()),
        ("without_the_reads", SymRules::without_the_reads()),
        (
            "both new dials off",
            SymRules {
                canonical_root: false,
                decision_read: false,
                ..SymRules::shipped()
            },
        ),
    ];
    let mut seen: Option<(&str, Vec<String>)> = None;
    for (name, rules) in sets {
        start_profile();
        let _ = replay(&doc, &nominal, rules, tol);
        let p = take_profile();
        let largest = p.ops.values().map(|o| o.max_terms_out).max().unwrap_or(0);
        let lines = plain_lines(&p.walk_ledger());
        println!(
            "  {name}: largest form {largest}\n    {}",
            lines.join("\n    ")
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
