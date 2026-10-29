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
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::collections::BTreeMap;

use geom_core::{SymRules, Tol};

use crate::m10_8_harness::split_at_the_nominal;

/// The tier as it stood before this unit: the shipped set with rule G
/// and the decision read shut. Rule G's conjunct dials (its companion
/// rewrite, its magnitude door, its exact quotient) are spelled shut
/// with it through [`SymRules::without_canonical_root`], so this is one
/// `SymRules` value with the base tier and not a tier that never
/// existed.
fn before() -> SymRules {
    SymRules {
        decision_read: false,
        ..SymRules::without_canonical_root()
    }
}

/// One measured document, by name and by the door that builds it.
type Document = (&'static str, Box<dyn Fn() -> editor_core::ProfileDoc>);

fn total(t: &BTreeMap<&'static str, [u64; 2]>) -> [u64; 2] {
    t.values()
        .fold([0u64; 2], |a, s| [a[0] + s[0], a[1] + s[1]])
}

/// **Per predicate, the discharged count never falls** — the
/// acceptance's own clause, asserted per predicate and printed whole,
/// so a re-baseline reads the row rather than the reviewer's
/// arithmetic — and the two predicates the dials move are pinned
/// exactly, on both sides.
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
        let on = split_at_the_nominal(&doc, SymRules::shipped(), tol);
        println!(
            "== {name}: G + the read off {:?} -> shipped {:?}",
            total(&off),
            total(&on)
        );
        for (p, b) in &off {
            let a = on.get(p).copied().unwrap_or([0; 2]);
            println!(
                "  {p}: {b:?} -> {a:?}{}",
                if a == *b { "" } else { "  <- moved" }
            );
            // **TWO predicates are pinned at their numbers on R2's link,
            // and one on the bracket, with their reasons**,
            // `[symbolic, numeric]` on both sides, so any further drift
            // reds and says which. The link's lose no decision; they are
            // the ones this unit's dials MOVE, and a pin on both sides is
            // what tells a re-baseline from a drift. The bracket's is the
            // one trade, and its reason is beside it.
            //
            // `carrier_on_surface_1`/`_2` is rule G's trade: the tangent
            // arc's quarter-tangent `2w / sqrt(4w²)`, which rule G
            // re-keys as a sign quotient the tier does not hold, so the
            // shipped side reaches through the door what the G-shut side
            // proves outright, and four of each slot's twelve numeric
            // decisions go symbolic. The two slots are the pair's key
            // order (`geom_brep::SurfacePair`), so the trade lands half
            // on each. The whole measurement and its remedy record is
            // `work/decide/rule-g-trades-sixteen-of-the-links-carrier-on-surface-2`.
            //
            // `carrier_matches_mapped_source` is the other side of the
            // arc's span spelled from the decided turn (`sweep`'s
            // `arc_span`): the carrier's span meets the pushforward's
            // atom on the shipped side, four more decisions out of
            // `numeric`.
            //
            // The sums are the split columns of the day before DECIDE-10
            // made them one (`[96, 0, 0, 12] -> [92, 0, 8, 8]` and
            // `[108, 0, 44, 28] -> [144, 0, 12, 24]` as theorem / gated /
            // registered / numeric). Both sides run one attempt per rung
            // (`split_at_the_nominal`, no ladder); what the ladder does
            // is pinned where the ladder is
            // (`sym_9_retry_interval::sym_9_the_kept_atom_ladder_recovers_what_phase_1_measured`).
            //
            // The link's placement reads a declared or toleranced
            // variable, so it runs its map at the nominal (C-5 narrowed,
            // the orchestrator's ruling) and the copy is re-certified
            // after its transform:
            // +36 symbolic on each `carrier_on_surface_*` slot and +18 on
            // `carrier_matches_mapped_source`, on both sides alike, so
            // each trade is the one it was. Rule H's angle sums take
            // eight more of `carrier_matches_mapped_source` on both
            // sides alike (`[170, 28] -> [178, 20]`, `[174, 24] ->
            // [182, 16]`).
            let pinned: Option<([u64; 2], [u64; 2])> = match (*name, *p) {
                ("r2_link", "carrier_on_surface_1" | "carrier_on_surface_2") => {
                    Some(([132, 12], [136, 8]))
                }
                ("r2_link", "carrier_matches_mapped_source") => Some(([178, 20], [182, 16])),
                // **The one decision rule G trades, against rule H**: on
                // the bracket rule H's angle sums take six of
                // `carrier_matches_mapped_source` with rule G shut and two
                // with it on (`[339, 12]` on both sides before rule H),
                // the canonical root re-keying four of the roots the
                // rewrite's sums would meet. One attempt per rung is what
                // this row runs; the drive's kept-atom ladder, whose first
                // attempt shuts rule G, takes them back and two more
                // (`[347, 4]` under `editor_core::drive::DEFAULT_SYM_RETRY`).
                ("r2_filleted_bracket", "carrier_matches_mapped_source") => {
                    Some(([345, 6], [341, 10]))
                }
                _ => None,
            };
            let traded = (*name, *p) == ("r2_filleted_bracket", "carrier_matches_mapped_source");
            if let Some(expected) = pinned {
                assert_eq!(
                    (*b, a),
                    expected,
                    "{name}/{p}: the pinned predicate moved: re-measure it and say what"
                );
            }
            if a[0] < b[0] && !traded {
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
            t.get("carrier_on_surface_2").copied().unwrap_or([0; 2])
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
                    ShapeOutcome::NumericZero | ShapeOutcome::Indeterminate
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
                // The first 600 characters: a rendered form carries
                // multi-byte glyphs, so a byte bound can split one.
                let cut = f.char_indices().nth(600).map_or(f.len(), |(i, _)| i);
                println!("   {}", &f[..cut]);
            }
            if let Some(f) = &s.form {
                println!("   plain [{}]", census(f));
            }
        }
        println!("== {name}: {} shapes, {shown} blocked shown", shapes.len());
    }
}
