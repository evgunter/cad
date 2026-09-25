//! REVIEW PROBE (DECIDE-6 review): the pad's dev leaf at the certifying
//! end of its pinned bracket (`m10_10_pins_interval`'s ceiling row
//! replays it there twice), and the link's, timed under the shipped
//! rules and with each of DECIDE-3's and DECIDE-4's additions shut, so
//! whose cost the pin suite's replays are is measured in the suite's
//! own regime (dev, one attempt per rung). Prints and asserts nothing.
//!
//! `CAD_D6R_DOCS=pad,link` and `CAD_D6R_COLS=shipped,no_g,...` select.
#![cfg(feature = "interval")]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::time::Instant;

use geom_core::{SymRules, Tol};

use crate::m10_8_harness::{dials, whole_box_leaf};

fn pick(var: &str, label: &str) -> bool {
    std::env::var(var)
        .ok()
        .filter(|s| !s.trim().is_empty())
        .is_none_or(|l| l.split(',').any(|n| n.trim() == label))
}

#[test]
#[ignore = "review probe: dev leaf times under rule differentials"]
fn decide_6_review_probe_leaf_times() {
    let tol = Tol::witness();
    let eps = tol.eps();
    let docs: [(&str, editor_core::ProfileDoc); 2] = [
        (
            "pad",
            crate::m10_8_r2_probes_interval::pad(2.4990e3 * eps, tol).0,
        ),
        (
            "link",
            crate::m10_9_r2_probes_interval::link(4.930e2 * eps, tol).0,
        ),
    ];
    let cols: [(&str, SymRules); 4] = [
        ("shipped", SymRules::shipped()),
        ("no_reads", SymRules::without_the_reads()),
        ("no_g", SymRules::without_canonical_root()),
        ("no_q", SymRules::without_root_quotient()),
    ];
    for (name, doc) in &docs {
        if !pick("CAD_D6R_DOCS", name) {
            continue;
        }
        for (label, rules) in cols {
            if !pick("CAD_D6R_COLS", label) {
                continue;
            }
            let t = Instant::now();
            let (ok, c) = whole_box_leaf(doc, dials(rules), tol);
            println!(
                "   {name:<5} {label:<9} certified {ok} receipt {:?} in {:.3}s",
                [c.symbolic_zero, c.sign_gated, c.registered, c.numeric],
                t.elapsed().as_secs_f64()
            );
        }
    }
}
