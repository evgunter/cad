//! **The decision read's cost, per document** — where the time of
//! `signed::decision` (and of `signed::order` in front of it at
//! `min`/`max`) goes on the documents the plate's pin suite
//! (`m10_10_pins_interval`) drives: how often it is asked, what it
//! answers, why it declines, and what each cheap answer before the
//! enclosure would save (`geom_core::sym::profile::ReadProfile`).
//!
//! `decide_6_where_the_reads_cost_is` is `#[ignore]`d evidence that
//! prints and asserts nothing: it is the measurement, and the gates on
//! what the read decides are the pins it measures. It replays, with no
//! retry ladder (the pin suite's dials, `m10_8_harness::dials`), each
//! document's whole-box leaf at the certifying end of its pinned
//! bracket, and the plate at its nominal as the split row asks it:
//! timed unprofiled under the shipped rules, with the read shut, with
//! rule G shut and with both (the differential that says whose cost a
//! replay's time is), then once more under the shipped rules with the
//! profile installed. The pad's leaf is in it and returns in dev on a
//! four-core box (133.9 s at `2.4990e3·ε`, the certifying end
//! `m10_10_pins_interval` replays, with the per-node reduction
//! memoised), and a full run is five of those and change;
//! `CAD_DECIDE_6_DOCS` and `CAD_DECIDE_6_COLUMNS` (`shipped`,
//! `read off`, `rule G off`, `both off`, `profiled`) name subsets.
//! Run it:
//!
//! ```sh
//! cargo test -p editor-core --features interval --test all -- \
//!   decide_6_read_cost_interval:: --ignored --nocapture --test-threads 1
//! ```
//!
//! `decide_6_the_profile_decides_nothing` GATES: the instrument
//! re-encloses what the read enclosed, so a profiled replay must land
//! every decision where an unprofiled one does.
#![cfg(feature = "interval")]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

// Gated to the tier (the read and its instrument live in it), the
// shared harness and the plate's fixture.
test_utils::gated_to![
    "crates/geom-core/src/sym.rs",
    "crates/geom-core/src/sym/",
    "crates/editor-core/tests/m10_8_harness.rs",
    "crates/editor-core/tests/m10_8_arc_family_interval.rs",
    "crates/editor-core/tests/m10_7_plate.rs",
];

use std::time::Instant;

use editor_core::ProfileDoc;
use geom_core::{SymRules, Tol};

use crate::m10_8_harness::{dials, split_at_the_nominal, whole_box_leaf};

/// One measured replay: a label, and the replay itself under a rule
/// set, answering the receipt's four columns.
type Replay<'a> = (&'static str, Box<dyn Fn(SymRules) -> [u64; 4] + 'a>);

fn leaf(doc: &ProfileDoc, rules: SymRules, tol: Tol) -> [u64; 4] {
    let (_, c) = whole_box_leaf(doc, dials(rules), tol);
    [c.symbolic_zero, c.sign_gated, c.registered, c.numeric]
}

/// The rule sets each replay is timed under, unprofiled: the shipped
/// set; the read shut; rule G shut (with its conjunct dials, the exact
/// quotient among them); and both shut — `decide_3_split_rows_interval::before`'s
/// value, which leaves A0's `min`/`max` folds on, so it is not the
/// whole tier as it stood before the unit that added rule G and the
/// read.
fn columns() -> [(&'static str, SymRules); 4] {
    [
        ("shipped", SymRules::shipped()),
        ("read off", SymRules::without_the_reads()),
        ("rule G off", SymRules::without_canonical_root()),
        (
            "both off",
            SymRules {
                decision_read: false,
                ..SymRules::without_canonical_root()
            },
        ),
    ]
}

/// **The read's table on the plate's pin-suite documents.** Each
/// replay prints its receipt, its wall time, and the read's profile.
#[test]
#[ignore = "evidence-only: the decision read's calls, declines by cause, and time"]
fn decide_6_where_the_reads_cost_is() {
    let tol = Tol::witness();
    let eps = tol.eps();
    // The certifying ends of the pinned brackets at this ε row
    // (`m10_10_pins_interval`), plate and annulus in units of their
    // real studies, the link and the bracket in ε.
    let row = [1.0e-6, 1.0e-9, 1.0e-12]
        .iter()
        .position(|&e| (eps / e - 1.0).abs() < 1.0e-3)
        .expect("a measured ε row");
    let plate_s = [0.2368, 0.2630, 0.2630][row];
    let annulus_s = [0.6962, 0.8416, 0.8414][row];
    let replays: Vec<Replay<'_>> = vec![
        (
            "plate_nominal",
            Box::new(move |rules| {
                let doc = crate::m10_7_plate::plate(5.0e-5, 1.0e-5, tol).0;
                let split = split_at_the_nominal(&doc, rules, tol);
                split.values().fold([0; 4], |a, s| {
                    [a[0] + s[0], a[1] + s[1], a[2] + s[2], a[3] + s[3]]
                })
            }),
        ),
        (
            "plate_leaf",
            Box::new(move |rules| {
                leaf(
                    &crate::m10_7_plate::plate(5.0e-5 * plate_s, 1.0e-5 * plate_s, tol).0,
                    rules,
                    tol,
                )
            }),
        ),
        (
            "annulus_leaf",
            Box::new(move |rules| {
                leaf(
                    &crate::m10_8_r1_probes_interval::annulus(annulus_s, tol).0,
                    rules,
                    tol,
                )
            }),
        ),
        (
            "link_leaf",
            Box::new(move |rules| {
                leaf(
                    &crate::m10_9_r2_probes_interval::link(4.930e2 * eps, tol).0,
                    rules,
                    tol,
                )
            }),
        ),
        (
            "bracket_leaf",
            Box::new(move |rules| {
                leaf(
                    &crate::m10_7_r2_probes_interval::bracket(3.870e2 * eps, tol).0,
                    rules,
                    tol,
                )
            }),
        ),
        (
            "pad_leaf",
            Box::new(move |rules| {
                leaf(
                    &crate::m10_8_r2_probes_interval::pad(2.4990e3 * eps, tol).0,
                    rules,
                    tol,
                )
            }),
        ),
    ];
    let keep = |var: &str, label: &str| {
        std::env::var(var)
            .ok()
            .filter(|s| !s.trim().is_empty())
            .is_none_or(|l| l.split(',').any(|n| n.trim() == label))
    };
    for (label, run) in replays {
        if !keep("CAD_DECIDE_6_DOCS", label) {
            continue;
        }
        for (column, rules) in columns() {
            if !keep("CAD_DECIDE_6_COLUMNS", column) {
                continue;
            }
            let t = Instant::now();
            let receipt = run(rules);
            println!(
                "   {label:<14} {column:<10} receipt {receipt:?} in {:.3}s",
                t.elapsed().as_secs_f64()
            );
        }
        if !keep("CAD_DECIDE_6_COLUMNS", "profiled") {
            continue;
        }
        geom_core::sym::profile::start_profile();
        let t = Instant::now();
        let receipt = run(SymRules::shipped());
        let wall = t.elapsed();
        let p = geom_core::sym::profile::take_profile();
        println!(
            "== {label} (eps {eps:e}), profiled: receipt {receipt:?} in {wall:?}, {} sessions",
            p.sessions
        );
        print!("{}", p.read.render());
    }
}

/// **The profile decides nothing.** The plate at its nominal, split per
/// predicate under the shipped set with the profile installed and
/// without it: the same split, and the instrument ran (it saw the
/// plate's `min`/`max` reads, every one of which settles).
#[test]
fn decide_6_the_profile_decides_nothing() {
    let tol = Tol::witness();
    let doc = crate::m10_7_plate::plate(5.0e-5, 1.0e-5, tol).0;
    let bare = split_at_the_nominal(&doc, SymRules::shipped(), tol);
    geom_core::sym::profile::start_profile();
    let profiled = split_at_the_nominal(&doc, SymRules::shipped(), tol);
    let read = geom_core::sym::profile::take_profile().read;
    assert_eq!(profiled, bare, "the profiled split is the unprofiled one");
    assert!(
        read.order > 0 && read.settled > 0,
        "the instrument saw the plate's reads: {}",
        read.render()
    );
}
