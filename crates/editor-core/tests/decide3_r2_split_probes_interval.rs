//! DECIDE-3 review probe (r2): the PER-PREDICATE split of the link, the
//! annulus and the bracket at the nominal, shipped against the shipped
//! set with rule G and the decision read shut — the acceptance's
//! "every per-predicate split moves UP or not at all in
//! `symbolic_zero + sign_gated + registered`", which the unit pinned
//! per DOCUMENT on these three. The pad is left out: its shape report
//! exhausts memory on this box.
#![cfg(feature = "interval")]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{SymRules, Tol};

use crate::m10_8_harness::split_at_the_nominal;

#[test]
#[ignore = "review evidence: three shape reports per document, minutes"]
fn r2_no_predicate_loses_a_decision_on_the_link_annulus_and_bracket() {
    let tol = Tol::witness();
    let before = SymRules {
        canonical_root: false,
        decision_read: false,
        ..SymRules::shipped()
    };
    let docs: [(&str, Box<dyn Fn() -> editor_core::ProfileDoc>); 3] = [
        (
            "r2_link",
            Box::new(move || crate::m10_9_r2_probes_interval::link(1.0, tol).0),
        ),
        (
            "r1_annulus",
            Box::new(move || crate::m10_8_r1_probes_interval::annulus(1.0, tol).0),
        ),
        (
            "r2_filleted_bracket",
            Box::new(move || crate::m10_7_r2_probes_interval::bracket(1.0, tol).0),
        ),
    ];
    let mut lost: Vec<String> = Vec::new();
    for (name, build) in &docs {
        let doc = build();
        let off = split_at_the_nominal(&doc, before, tol);
        let on = split_at_the_nominal(&doc, SymRules::shipped(), tol);
        let sum = |s: [u64; 4]| [s[0] + s[1] + s[2] + s[3], s[0], s[1], s[2], s[3]];
        let tot = |t: &std::collections::BTreeMap<&'static str, [u64; 4]>| {
            t.values().fold([0u64; 4], |a, s| {
                [a[0] + s[0], a[1] + s[1], a[2] + s[2], a[3] + s[3]]
            })
        };
        println!(
            "== {name}: G+reads off {:?} -> shipped {:?}",
            tot(&off),
            tot(&on)
        );
        for (p, b) in &off {
            let a = on.get(p).copied().unwrap_or([0; 4]);
            let moved = if a == *b { "" } else { "  <- moved" };
            println!("  {p}: {b:?} -> {a:?}{moved}");
            let (sb, sa) = (sum(*b), sum(a));
            if sa[1] < sb[1] || sa[1] + sa[2] + sa[3] < sb[1] + sb[2] + sb[3] {
                lost.push(format!("{name}/{p}: {b:?} -> {a:?}"));
            }
        }
        for (p, a) in &on {
            if !off.contains_key(p) {
                println!("  {p}: (absent) -> {a:?}  <- new");
            }
        }
    }
    assert!(lost.is_empty(), "a predicate LOST a decision: {lost:?}");
}

/// The link's `carrier_on_surface_2` under the four dial sets, to say
/// which of the two new dials moves six theorems into `registered`.
#[test]
#[ignore = "review evidence: four shape reports of the link, minutes"]
fn r2_which_dial_moves_the_links_carrier_on_surface_2() {
    let tol = Tol::witness();
    let doc = crate::m10_9_r2_probes_interval::link(1.0, tol).0;
    let sets = [
        ("shipped", SymRules::shipped()),
        ("G off", SymRules::without_canonical_root()),
        ("reads off", SymRules::without_the_reads()),
        (
            "G + reads off",
            SymRules {
                canonical_root: false,
                decision_read: false,
                ..SymRules::shipped()
            },
        ),
    ];
    for (name, rules) in sets {
        let t = split_at_the_nominal(&doc, rules, tol);
        let tot = t.values().fold([0u64; 4], |a, s| {
            [a[0] + s[0], a[1] + s[1], a[2] + s[2], a[3] + s[3]]
        });
        println!(
            "  {name}: total {tot:?}, carrier_on_surface_2 {:?}",
            t.get("carrier_on_surface_2").copied().unwrap_or([0; 4])
        );
    }
}
