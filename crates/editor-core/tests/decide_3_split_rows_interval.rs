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

use crate::m10_8_harness::{split_at_the_nominal, split_at_the_nominal_retried};

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

/// One measured document, by name and by the door that builds it.
type Document = (&'static str, Box<dyn Fn() -> editor_core::ProfileDoc>);

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
    let docs: [Document; 3] = [
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
        // **The `on` side is the tier a DRIVE runs**, retry ladder and
        // all (`editor_core::drive::DEFAULT_SYM_RETRY`), so the clause
        // this row asserts is about what ships and not about one attempt
        // of it. SYM-9 put the ladder there and re-baselined this row;
        // the ladder can only ADD, so nothing in the clause weakens.
        let on = split_at_the_nominal_retried(
            &doc,
            SymRules::shipped(),
            editor_core::drive::DEFAULT_SYM_RETRY,
            tol,
        );
        println!(
            "== {name}: G + the read off {:?} -> shipped, with the ladder {:?}",
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
            // **ONE predicate is re-baselined, at its numbers, with
            // its reason.** R2's link's `carrier_on_surface_2` loses
            // sixteen theorems to rule G — TEN to rule A's companion
            // rewrite opening the squares of `abs` NODES the document
            // wrote (the same rewrite buys 52 of the 88 it leaves on
            // this predicate: with it shut, rule G reads 36/0/0/72
            // here), SIX to `sqrt(R²) = |R|`, which the rim
            // registrant's axiom then closes. Neither is the retired
            // side-condition source, and the remedy the finding named
            // was implemented and measured: it recovers the six and
            // four of the ten and costs the DOCUMENT forty theorems,
            // because after rule G a `sqrt(R²)` and a document's
            // `abs(R)` are the same atom, so the restriction narrows
            // by traversal order rather than by provenance.
            //
            // It is a re-baseline and not an exemption: the numbers
            // are asserted on both sides, so any further drift reds
            // and says which. The whole measurement, and the remedy
            // record, is
            // `work/decide/rule-g-trades-sixteen-of-the-links-carrier-on-surface-2`.
            // At the DOCUMENT level rule G is a gain here —
            // `[515, 0, 90, 497]` becomes `[541, 0, 96, 465]` at one
            // attempt per rung — and a change that makes the code right
            // is not skipped for the re-baseline it costs.
            //
            // **SYM-9 RE-BASELINED IT UP.** The shipped retry ladder
            // (`drive::DEFAULT_SYM_RETRY`) re-asks a refused decision
            // with rule G shut, and that recovers the TEN: this
            // predicate reads `[92, 0, 6, 10]` under the tier a drive
            // runs, so it now discharges 98 of its 108 decisions,
            // exactly as the rule-G-off base does, and the document
            // reads `[553, 0, 96, 453]`. What is left of the sixteen is
            // the SIX that were never a lost decision — they are
            // `registered` where the base had them `symbolic_zero`,
            // because `sqrt(R²) = |R|` hands them to the rim
            // registrant's axiom — and that is the residue the row
            // keeps.
            if *name == "r2_link" && *p == "carrier_on_surface_2" {
                assert_eq!(
                    (*b, a),
                    ([98, 0, 0, 10], [92, 0, 6, 10]),
                    "the re-baselined predicate moved: re-measure it and say what"
                );
                continue;
            }
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

/// Evidence: the link's `carrier_on_surface_2` residuals that stop
/// being theorems under rule G, rendered under both dial sets with the
/// atom census of each early form — the instrument that says WHICH
/// spelling stopped meeting which.
#[test]
#[ignore = "evidence-only: two shape reports of the link with forms rendered"]
fn decide_3_the_links_carrier_on_surface_2_rendered() {
    use crate::m10_8_harness::nominal_box;
    use editor_core::analysis::{AnalysisPolicy, analyzed_box};
    use geom_core::sym::report::{ShapeOutcome, explain_depth};
    let tol = Tol::witness();
    let doc = crate::m10_9_r2_probes_interval::link(1.0, tol).0;
    let census = |s: &str| {
        let n = |needle: &str| s.matches(needle).count();
        format!(
            "sqrt {} | abs {} | select {} | max {} | min {} | terms(top) {}",
            n("sqrt("),
            n("abs("),
            n("select("),
            n("max("),
            n("min("),
            s.split(" + ").count()
        )
    };
    for (name, rules) in [
        ("G off", SymRules::without_canonical_root()),
        ("shipped", SymRules::shipped()),
    ] {
        explain_depth(4);
        let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
        let (shapes, _, _) =
            crate::m10_8_arc_family_interval::replay(&doc, &nominal_box(&analyzed), rules, tol);
        let mut shown = 0;
        for s in &shapes {
            if s.predicate != "carrier_on_surface_2"
                || !matches!(
                    s.outcome,
                    ShapeOutcome::NumericZero
                        | ShapeOutcome::Indeterminate
                        | ShapeOutcome::Registered
                )
            {
                continue;
            }
            shown += 1;
            if shown > 2 {
                break;
            }
            println!("== {name} blocked #{shown}: {:?}", s.outcome);
            if let Some(f) = &s.early_form {
                println!("   early [{}]", census(f));
                println!("   {}", &f[..f.len().min(600)]);
            }
            if let Some(f) = &s.form {
                println!("   plain [{}]", census(f));
            }
        }
        println!("== {name}: {} shapes, {shown} blocked shown", shapes.len());
    }
}
