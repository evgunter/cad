//! **DECIDE-1's clause-1 `Invalid` census** — the dynamic half of the
//! self-dot straddle measurement (`docs/DECIDE-1-SPEC.md` §Phase 1.2).
//!
//! The question: does any clause-1 refusal (`Decide for Sym<T>`'s
//! `Invalid` arm — a domain violation the value channel answers on its
//! own, so the tier never asks the form) on a measured document descend
//! from a `sqrt` or a `sign_within` over an enclosure whose spurious
//! negative lower bound is a product of ONE straddling factor with
//! itself? A `v·v` spelled through `Vec::dot` is such a product; a
//! `norm_squared` is not, because it squares component-wise through the
//! tight `Real::powi(2)`.
//!
//! **THIS SUITE ASSERTS NOTHING AND CANNOT RED, BY DESIGN.** Its rows
//! are `#[ignore]`d evidence probes that print. A census that found
//! nothing has no state to gate, and an assertion that no runtime value
//! could make false is documentation rather than a check — so the
//! standing pins for the SOUND outcome on a hand-spelled product stay
//! where they are, in `geom-core`'s `m10_8_r1_sym_probes`
//! (`r1_rule_a_never_fires_on_a_straddling_argument` and
//! `r1_rule_a_decides_zero_at_every_width_and_off_it_widens`). What
//! nothing re-takes is the STATIC half, and that gap is a filed row
//! (`work/guard/self-dot-has-no-gate-the-interval-square-one-cannot-see-it`),
//! not a row here. Run these:
//!
//! ```sh
//! cargo test -p editor-core --features interval --test all -- \
//!   decide_1_self_dot_interval:: --ignored --nocapture --test-threads 1
//! ```
//!
//! **A REPLAY ESCALATES AT ITS FIRST BLOCKED PREDICATE AND STOPS**, so
//! every count here is over the decisions SEEN, which the rows print
//! beside the count along with the refusal that ended them. The pad is
//! measured on the CEILING instrument only: the shape report renders
//! the plain normal form of every blocked residual, and over that
//! document it does not fit in the memory of a box the size of this
//! lane's. Its bracket is measured; its `Invalid` count is not taken,
//! and the row says so.
#![cfg(feature = "interval")]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use editor_core::ProfileDoc;
use editor_core::analysis::{AnalysisPolicy, ParamBox, analyzed_box};
use geom_core::sym::report::{DecisionShape, ShapeOutcome, explain_depth};
use geom_core::{SymRules, Tol};

use crate::m10_8_arc_family_interval::replay;
use crate::m10_8_harness::{blocked, ceiling, head, nominal_box, split};
use crate::m10_9_pins_interval::measured_studies;

/// How many DAG levels below a blocked residual the report explains,
/// from `CAD_DECIDE_1_EXPLAIN`. **Zero by default, and the default is
/// what fits**: the report renders the plain normal form of every
/// blocked residual already, and asking it for the DAG as well costs
/// the pad's nominal replay more memory than this class of box has —
/// measured, SIGKILL. Four is the level that reaches the `Mul` under a
/// `Sqrt` under a difference, which is the shape this census looks
/// for, and it is affordable one document at a time
/// (`CAD_DECIDE_1_DOC`) once the census has named which one to ask.
fn explain_levels() -> usize {
    std::env::var("CAD_DECIDE_1_EXPLAIN")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(0)
}

/// The ceiling search's bracket and step count:
/// `CAD_DECIDE_1_LO`, `CAD_DECIDE_1_HI`, `CAD_DECIDE_1_STEPS`.
///
/// **BOTH ENDS ARE IN ONE UNIT — MULTIPLES OF THE DOCUMENT'S REAL
/// STUDY**, which is the unit `Study::at` consumes and the unit every
/// ceiling this suite prints is in (`certifies x2.63e-1` is 0.263 of
/// the study a user would ask for). The ε-relative view is printed
/// beside it and is derived, never entered.
///
/// The defaults are M10-10's evidence bench's — `1e-1 · ε` up to `1e1`,
/// ten times the real study, in sixteen steps — a search over eleven
/// decades, which is what a document whose ceiling is not already known
/// needs. It is not what the PAD needs: eighteen whole-box drives of
/// that document did not finish in an hour and a half on a box this
/// size at 46 s a probe, and its bracket is pinned to four digits
/// already (`m10_9_pins_interval::measured_studies`), so the pad's own
/// default is a five-step search of `[1e3 · ε, 1e4 · ε]` around that
/// number.
fn search(eps: f64, name: &str) -> (f64, f64, usize) {
    let num = |k: &str, d: f64| {
        std::env::var(k)
            .ok()
            .and_then(|v| v.parse::<f64>().ok())
            .unwrap_or(d)
    };
    let (lo, hi, steps) = if name == "r2_rounded_pad" {
        (1.0e3 * eps, 1.0e4 * eps, 5.0)
    } else {
        (1.0e-1 * eps, 1.0e1, 16.0)
    };
    (
        num("CAD_DECIDE_1_LO", lo),
        num("CAD_DECIDE_1_HI", hi),
        num("CAD_DECIDE_1_STEPS", steps) as usize,
    )
}

/// The one document to measure, from `CAD_DECIDE_1_DOC`; all six when
/// unset.
fn only_doc() -> Option<String> {
    std::env::var("CAD_DECIDE_1_DOC")
        .ok()
        .filter(|v| !v.is_empty())
}

/// The PREDICATE a refusal names, out of the refusal's own message —
/// the quoted name after `predicate `. The whole message is a
/// paragraph of recourse; what a census row wants is which predicate
/// ended the replay.
fn refusal_predicate(refusal: &str) -> String {
    refusal
        .split_once("predicate '")
        .and_then(|(_, rest)| rest.split_once('\''))
        .map_or_else(|| head(refusal, 60), |(name, _)| name.to_owned())
}

/// The census of one replay, printed: the counts, one line per blocked
/// predicate, the `transform_rigid_*` rows if any of them DECIDED
/// here, and — for a predicate that did refuse clause 1 — the first
/// such refusal's enclosure, its rendered residual and the DAG below
/// it, the three things that say whether the spurious lower bound is a
/// self-product.
///
/// The `transform_rigid_*` readout is R1's (the review's probe branch,
/// `r1_decide_1_review_probe.rs`): the blocked table keeps only what
/// the numeric channel could NOT decide, so it cannot show that a
/// predicate never ran — only a filter over [`split`], which keeps
/// every predicate that decided at all, can.
fn report_one(label: &str, shapes: &[DecisionShape], refusal: Option<&str>) {
    let table = blocked(shapes);
    let invalid: usize = table.values().map(|r| r.0).sum();
    println!(
        "{label:<42} decisions seen {:>5}  INVALID {invalid:>4}  blocked predicates {}  \
         stopped at {}",
        shapes.len(),
        table.len(),
        refusal.map_or_else(|| "nothing (ran to the end)".to_owned(), refusal_predicate)
    );
    for (pred, (invalid, indeterminate, all)) in &table {
        println!("    invalid {invalid:>4}  indeterminate {indeterminate:>4}  of {all:<5} {pred}");
    }
    let rigid: Vec<_> = split(shapes)
        .into_iter()
        .filter(|(p, _)| p.starts_with("transform_rigid"))
        .collect();
    println!(
        "    transform_rigid_* rows that DECIDED here: {}",
        if rigid.is_empty() {
            "none".to_owned()
        } else {
            format!("{rigid:?}")
        }
    );
    for pred in table.iter().filter(|(_, r)| r.0 > 0).map(|(p, _)| p) {
        let Some(s) = shapes
            .iter()
            .find(|s| s.predicate == *pred && matches!(s.outcome, ShapeOutcome::Invalid))
        else {
            continue;
        };
        println!("  --- {pred}: enclosure {:?}", s.enclosure);
        if let Some(form) = &s.form {
            println!("  plain form: {}", head(form, 600));
        }
        if let Some(early) = &s.early_form {
            println!("  early form: {}", head(early, 600));
        }
        if let Some(explain) = &s.explain {
            println!("  dag:\n{explain}");
        }
    }
}

/// One document at one scale and one box, replayed under the shipped
/// tier with the shape report installed.
fn census_at(label: &str, doc: &ProfileDoc, whole: bool, rules: SymRules, tol: Tol) {
    let analyzed = analyzed_box(doc, &AnalysisPolicy::default());
    let box_ = if whole {
        ParamBox::of(&analyzed)
    } else {
        nominal_box(&analyzed)
    };
    let (shapes, refusal, counts) = replay(doc, &box_, rules, tol);
    println!("{label:<46} {counts:?}");
    report_one(label, &shapes, refusal.as_deref());
}

/// **THE CLAUSE-1 `Invalid` CENSUS** on the six documents, at the
/// nominal (a degenerate box) and at ceiling + δ (the refusing end of
/// the bracket this run MEASURES, never one read off a table), under
/// the shipped tier. ε is the run's own (`CAD_TOLERANCE_EPS`), so the
/// three ε rows of the spec are three runs of this one probe.
#[test]
#[ignore = "evidence probe: prints the clause-1 Invalid census on the six documents"]
fn decide_1_the_clause_1_invalid_census_on_the_six_documents() {
    let tol = Tol::witness();
    let eps = tol.eps();
    let rules = SymRules::shipped();
    let levels = explain_levels();
    let only = only_doc();
    explain_depth(levels);
    println!("=== DECIDE-1 clause-1 Invalid census, eps = {eps:e}, explain {levels} ===");

    // The M10-3 slab: straight geometry, no arc, no ceiling — the
    // control. Both boxes are affordable.
    if only.as_deref().is_none_or(|d| d == "m10_3_slab") {
        let slab = crate::m10_3_driver_interval::slab(1.0, 0.25);
        census_at("m10_3_slab / nominal", &slab, false, rules, tol);
        census_at("m10_3_slab / whole box", &slab, true, rules, tol);
    }

    for study in measured_studies(tol) {
        if only.as_deref().is_some_and(|d| d != study.name) {
            continue;
        }
        // **THE CEILING IS MEASURED, NEVER READ OFF A TABLE.**
        // `measured_studies`' brackets are M10-9's tier's, and this
        // census runs the SHIPPED one, whose ceilings are elsewhere
        // (the pad's differ in the second digit); replaying at another
        // tier's refusing end is a replay that decides everything, and
        // a zero `Invalid` over one of those measures nothing. Same
        // call M10-10's evidence bench makes, over the bracket
        // `search` gives.
        let (s_lo, s_hi, steps) = search(eps, study.name);
        let (lo, hi, per) = ceiling(&*study.at, rules, tol, s_lo, s_hi, steps);
        println!(
            "{:<42} certifies x{lo:e}, refuses x{hi:e} ({per:.2}s/probe) [{:.4e}..{:.4e} eps]",
            study.name,
            lo / eps,
            hi / eps
        );
        // **THE PAD IS MEASURED ON THE CEILING INSTRUMENT ONLY.** Its
        // replay under the shape report — which renders the plain
        // normal form of every blocked residual — is killed for memory
        // on a box of this size, at the nominal as well as over the
        // whole box, so what this document contributes to the census is
        // its bracket and not an `Invalid` count. Said, not skipped
        // silently: a row nothing could run is not a row that measured
        // zero.
        if study.name == "r2_rounded_pad" {
            println!(
                "{:<42} NOT REPLAYED — the shape report over this document does not fit in the \
                 memory of a box this size; the bracket above is the ceiling instrument's",
                "r2_rounded_pad / nominal, ceiling + delta"
            );
            continue;
        }
        // **A BRACKET WITH NO CERTIFYING END IS NOT A CEILING.**
        // `ceiling` answers `(NaN, hi, _)` when even `lo` refuses, and
        // `hi` is then the top of the search bracket UNBISECTED — a
        // scale nothing measured. Replaying there is still worth doing
        // (it is the widest box in hand), but calling it "ceiling + δ"
        // would name it something it is not.
        let (nominal_scale, wide_scale, wide_label) = if lo.is_finite() {
            (lo, hi, "ceiling + delta")
        } else {
            println!(
                "{:<42} NO CERTIFYING END: even x{s_lo:e} refuses, so no ceiling was bisected \
                 and the rows below are at the search bracket's own ends",
                study.name
            );
            (s_lo, s_hi, "top of the search bracket (unbisected)")
        };
        census_at(
            &format!("{} / nominal", study.name),
            &(study.at)(nominal_scale),
            false,
            rules,
            tol,
        );
        if !wide_scale.is_finite() {
            println!(
                "{:<42} SKIPPED — no finite wide end in the search bracket",
                format!("{} / {wide_label}", study.name)
            );
            continue;
        }
        census_at(
            &format!("{} / {wide_label}", study.name),
            &(study.at)(wide_scale),
            true,
            rules,
            tol,
        );
    }
    explain_depth(0);
}

/// **THE SAME COUNT AT WIDER SCALES, up to and past the real study** —
/// adopted from R1's review probe (`decide/1-review-r1`,
/// `r1_decide_1_review_probe.rs`), which asked the question the census
/// above does not: is the zero `Invalid` an artefact of replaying at
/// ceiling + δ, the NARROWEST refusing box? The item's mechanism (a
/// spurious negative lower bound from a product of one enclosure with
/// itself) GROWS with width, so a zero at the narrowest refusing box
/// is the weakest place to find one.
///
/// `s = 1.0` IS the real study — the widths a user would actually ask
/// for — and `s = 2.0` is twice it, so these rows carry the verdict
/// past the ceiling rather than only up to it.
#[test]
#[ignore = "evidence probe: the clause-1 Invalid count at wider scales (R1's question)"]
fn decide_1_the_invalid_count_at_wider_scales() {
    let tol = Tol::witness();
    let rules = SymRules::shipped();
    explain_depth(0);
    println!(
        "=== DECIDE-1 Invalid at wider scales, eps = {:e} ===",
        tol.eps()
    );
    for s in [0.5_f64, 1.0, 2.0] {
        let doc = crate::m10_7_plate::plate(5.0e-5 * s, 1.0e-5 * s, tol).0;
        census_at(
            &format!("two_hole_plate / whole box / s={s}"),
            &doc,
            true,
            rules,
            tol,
        );
    }
    for s in [1.0_f64, 2.0] {
        let doc = crate::m10_8_r1_probes_interval::annulus(s, tol).0;
        census_at(
            &format!("r1_annulus / whole box / s={s}"),
            &doc,
            true,
            rules,
            tol,
        );
    }
}
