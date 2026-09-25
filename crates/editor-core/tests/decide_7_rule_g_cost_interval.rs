//! **Rule G's leaf cost, per part** — where inside the canonical root
//! (`geom_core::sym::root`) the time of the link's and the pad's leaves
//! goes, against the same leaves with rule G shut
//! (`SymRules::without_canonical_root`): the mint site's own time by
//! branch and part (`geom_core::sym::profile::RootProfile`), and the
//! same DAG nodes' forms and times under each rule set
//! (`SymProfile::node_delta`).
//!
//! `decide_7_where_rule_gs_time_goes` is `#[ignore]`d evidence that
//! prints and asserts nothing: it is the measurement, and the gates on
//! what rule G decides are the pins it measures. It replays, with no
//! retry ladder (`m10_8_harness::dials`), the link's and the pad's
//! whole-box leaves at the certifying ends `m10_10_pins_interval`
//! replays: timed unprofiled under the shipped set, with rule G shut,
//! and with each of rule G's conjunct dials shut alone (the companion
//! `|X|² = X²`, the magnitude door, the exact quotient) — which say
//! whose cost a replay's time is — then profiled under the shipped set
//! and under rule G shut, with the node join between the two. The
//! pad's leaf returns in dev on a four-core box (133.9 s shipped with
//! the per-node reduction memoised), and a full run is seven pad
//! replays and the rest; `CAD_DECIDE_7_DOCS` (`link_leaf`, `pad_leaf`,
//! `bracket_leaf`) and `CAD_DECIDE_7_COLUMNS` (`shipped`, `G off`,
//! `abs_square off`, `magnitude off`, `quotient off`, `profiled`) name
//! subsets. Run it:
//!
//! ```sh
//! cargo test -p editor-core --features interval --test all -- \
//!   decide_7_rule_g_cost_interval:: --ignored --nocapture --test-threads 1
//! ```
//!
//! `decide_7_the_profile_decides_nothing` GATES: a profiled replay must
//! land every decision where an unprofiled one does, and the rule-G
//! clock must have seen the plate's roots.
#![cfg(feature = "interval")]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

// Gated to the tier (rule G and its instrument live in it), the shared
// harness and the documents' fixtures.
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

fn leaf(doc: &ProfileDoc, rules: SymRules, tol: Tol) -> [u64; 4] {
    let (_, c) = whole_box_leaf(doc, dials(rules), tol);
    [c.symbolic_zero, c.sign_gated, c.registered, c.numeric]
}

/// The rule sets each replay is timed under, unprofiled: the shipped
/// set; rule G shut with its conjunct dials; and each conjunct dial
/// shut alone, rule G otherwise on.
fn columns() -> [(&'static str, SymRules); 5] {
    let shipped = SymRules::shipped();
    [
        ("shipped", shipped),
        ("G off", SymRules::without_canonical_root()),
        (
            "abs_square off",
            SymRules {
                abs_square: false,
                ..shipped
            },
        ),
        (
            "magnitude off",
            SymRules {
                root_magnitude: false,
                ..shipped
            },
        ),
        ("quotient off", SymRules::without_root_quotient()),
    ]
}

/// **Rule G's table on the link's and the pad's leaves.** Each replay
/// prints its receipt and its wall time; the profiled pair prints the
/// walks, rule G's own table, and the node join.
#[test]
#[ignore = "evidence-only: rule G's own time by branch and part, and the node join against rule G shut"]
fn decide_7_where_rule_gs_time_goes() {
    let tol = Tol::witness();
    let eps = tol.eps();
    type Replay<'a> = (&'static str, Box<dyn Fn(SymRules) -> [u64; 4] + 'a>);
    let replays: Vec<Replay<'_>> = vec![
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
        if !keep("CAD_DECIDE_7_DOCS", label) {
            continue;
        }
        for (column, rules) in columns() {
            if !keep("CAD_DECIDE_7_COLUMNS", column) {
                continue;
            }
            let t = Instant::now();
            let receipt = run(rules);
            println!(
                "   {label:<14} {column:<15} receipt {receipt:?} in {:.3}s",
                t.elapsed().as_secs_f64()
            );
        }
        if !keep("CAD_DECIDE_7_COLUMNS", "profiled") {
            continue;
        }
        let mut profiles = Vec::new();
        for (column, rules) in [
            ("shipped", SymRules::shipped()),
            ("G off", SymRules::without_canonical_root()),
        ] {
            geom_core::sym::profile::start_profile();
            let t = Instant::now();
            let receipt = run(rules);
            let wall = t.elapsed();
            let p = geom_core::sym::profile::take_profile();
            println!(
                "== {label} (eps {eps:e}) {column}, profiled: receipt {receipt:?} in {wall:?}, \
                 {} sessions",
                p.sessions
            );
            print!("{}", p.render());
            print!("{}", p.root.render());
            profiles.push(p);
        }
        println!("== {label}: the same nodes, shipped / G off");
        print!("{}", profiles[0].node_delta(&profiles[1]));
    }
}

/// **The profile decides nothing.** The plate at its nominal, split per
/// predicate under the shipped set with the profile installed and
/// without it: the same split, and rule G's clock ran (it saw the
/// plate's roots and answered some of them).
#[test]
fn decide_7_the_profile_decides_nothing() {
    let tol = Tol::witness();
    let doc = crate::m10_7_plate::plate(5.0e-5, 1.0e-5, tol).0;
    let bare = split_at_the_nominal(&doc, SymRules::shipped(), tol);
    geom_core::sym::profile::start_profile();
    let profiled = split_at_the_nominal(&doc, SymRules::shipped(), tol);
    let root = geom_core::sym::profile::take_profile().root;
    assert_eq!(profiled, bare, "the profiled split is the unprofiled one");
    assert!(
        root.own.calls > 0 && root.answered > 0,
        "rule G's clock saw the plate's roots: {}",
        root.render()
    );
}
