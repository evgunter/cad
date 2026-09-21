//! **Rule G and the decision read, per PREDICATE** — the acceptance
//! this unit is measured against is "no decision LOST", and a decision
//! is lost at a predicate, not at a document total. A sum that holds
//! while one predicate trades theorems for axioms and another gains
//! more than it lost reads as "nothing moved"; these rows read the
//! predicates.
//!
//! The pad is not here. Its shape report exhausts memory on the boxes
//! this suite runs on (DECIDE-1 measured it), so the pad is held per
//! document by `m10_9_pins_interval` and on the ceiling instrument,
//! and that is a gap this row states rather than hides.
#![cfg(feature = "interval")]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::collections::BTreeMap;

use geom_core::{SymRules, Tol};

use crate::m10_8_harness::split_at_the_nominal;

/// The tier as it stood before this unit: the shipped set with rule G
/// and the decision read shut. Rule G's companion rewrite in rule A
/// rides rule G's dial, so this IS the base tier and not a tier that
/// never existed.
fn before() -> SymRules {
    SymRules {
        canonical_root: false,
        decision_read: false,
        ..SymRules::shipped()
    }
}

fn total(t: &BTreeMap<&'static str, [u64; 4]>) -> [u64; 4] {
    t.values().fold([0u64; 4], |a, s| {
        [a[0] + s[0], a[1] + s[1], a[2] + s[2], a[3] + s[3]]
    })
}

/// **Per predicate, `symbolic_zero` never falls and the discharged
/// count never falls.** The first is the direction the receipt may
/// never move in (a theorem re-labelled as a gated read or as an
/// axiom is a claim WEAKENED); the second is the acceptance's own
/// clause. Both are asserted per predicate and printed whole, so a
/// re-baseline reads the row rather than the reviewer's arithmetic.
#[test]
fn decide_3_no_predicate_loses_a_decision() {
    let tol = Tol::witness();
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
        let off = split_at_the_nominal(&doc, before(), tol);
        let on = split_at_the_nominal(&doc, SymRules::shipped(), tol);
        println!(
            "== {name}: G + the read off {:?} -> shipped {:?}",
            total(&off),
            total(&on)
        );
        for (p, b) in &off {
            let a = on.get(p).copied().unwrap_or([0; 4]);
            println!(
                "  {p}: {b:?} -> {a:?}{}",
                if a == *b { "" } else { "  <- moved" }
            );
            let discharged = |s: [u64; 4]| s[0] + s[1] + s[2];
            if a[0] < b[0] {
                lost.push(format!("{name}/{p}: THEOREMS fell {b:?} -> {a:?}"));
            } else if discharged(a) < discharged(*b) {
                lost.push(format!("{name}/{p}: discharges fell {b:?} -> {a:?}"));
            }
        }
        for (p, a) in &on {
            if !off.contains_key(p) {
                println!("  {p}: (absent) -> {a:?}  <- new");
            }
        }
    }
    assert!(
        lost.is_empty(),
        "a predicate lost a decision to this unit: {lost:?}"
    );
}

/// Which of the two dials moves what, on the document where the first
/// cut of this unit traded six theorems to the registered-identity
/// door. Evidence-only: the row above is what gates.
#[test]
#[ignore = "evidence-only: four shape reports of the link, minutes"]
fn decide_3_which_dial_moves_the_links_split() {
    let tol = Tol::witness();
    let doc = crate::m10_9_r2_probes_interval::link(1.0, tol).0;
    for (name, rules) in [
        ("shipped      ", SymRules::shipped()),
        ("G off        ", SymRules::without_canonical_root()),
        ("reads off    ", SymRules::without_the_reads()),
        ("G + reads off", before()),
    ] {
        let t = split_at_the_nominal(&doc, rules, tol);
        println!(
            "  {name}: total {:?}, carrier_on_surface_2 {:?}",
            total(&t),
            t.get("carrier_on_surface_2").copied().unwrap_or([0; 4])
        );
    }
}
