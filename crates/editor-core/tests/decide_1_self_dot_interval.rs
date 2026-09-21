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
//! Every row here is an `#[ignore]`d evidence probe that prints and
//! asserts nothing a gate could read ([[test-suite-cost]]); the
//! standing pins for the SOUND outcome on a hand-spelled product are
//! `geom-core`'s `m10_8_r1_sym_probes` rows. Run them:
//!
//! ```sh
//! cargo test -p editor-core --features interval --test all -- \
//!   decide_1_self_dot_interval:: --ignored --nocapture --test-threads 1
//! ```
//!
//! The pad is measured at its NOMINAL only: its whole-box shape report
//! renders every blocked residual of a replay past the ceiling and does
//! not fit in the memory of the box this lane runs on.
#![cfg(feature = "interval")]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::collections::BTreeMap;

use editor_core::ProfileDoc;
use editor_core::analysis::{AnalysisPolicy, ParamBox, analyzed_box};
use geom_core::sym::report::{DecisionShape, ShapeOutcome, explain_depth};
use geom_core::{SymRules, Tol};

use crate::m10_8_arc_family_interval::replay;
use crate::m10_8_harness::{ceiling, nominal_box};
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

/// The one document to measure, from `CAD_DECIDE_1_DOC`; all six when
/// unset.
fn only_doc() -> Option<String> {
    std::env::var("CAD_DECIDE_1_DOC").ok().filter(|v| !v.is_empty())
}

/// `predicate -> (invalid, indeterminate, all)` over one replay, kept
/// for every predicate that was BLOCKED at all — the indeterminate
/// column is there so a row of zero `Invalid`s still says whether the
/// replay was stressed or simply decided everything.
fn invalid_census(shapes: &[DecisionShape]) -> BTreeMap<&'static str, (usize, usize, usize)> {
    let mut table: BTreeMap<&'static str, (usize, usize, usize)> = BTreeMap::new();
    for s in shapes {
        let row = table.entry(s.predicate).or_default();
        row.2 += 1;
        match s.outcome {
            ShapeOutcome::Invalid => row.0 += 1,
            ShapeOutcome::Indeterminate => row.1 += 1,
            _ => {}
        }
    }
    table.retain(|_, (invalid, indeterminate, _)| *invalid > 0 || *indeterminate > 0);
    table
}

/// The census of one replay, printed: one line per predicate with a
/// clause-1 refusal, then the FIRST such refusal's enclosure, its
/// rendered residual and the DAG below it — the three things that say
/// whether the spurious lower bound is a self-product.
fn report_one(label: &str, shapes: &[DecisionShape]) -> usize {
    let table = invalid_census(shapes);
    let invalid: usize = table.values().map(|r| r.0).sum();
    println!(
        "{label:<42} INVALID {invalid:>4} of {} decisions, blocked predicates: {}",
        shapes.len(),
        table.len()
    );
    for (pred, (invalid, indeterminate, all)) in &table {
        println!("    invalid {invalid:>4}  indeterminate {indeterminate:>4}  of {all:<5} {pred}");
    }
    for pred in table.iter().filter(|(_, r)| r.0 > 0).map(|(p, _)| p) {
        let Some(s) = shapes
            .iter()
            .find(|s| s.predicate == *pred && matches!(s.outcome, ShapeOutcome::Invalid))
        else {
            continue;
        };
        println!("  --- {pred}: enclosure {:?}", s.enclosure);
        if let Some(form) = &s.form {
            println!("  plain form: {}", truncate(form));
        }
        if let Some(early) = &s.early_form {
            println!("  early form: {}", truncate(early));
        }
        if let Some(explain) = &s.explain {
            println!("  dag:\n{explain}");
        }
    }
    table.len()
}

/// A rendered residual is a page-long rational function; the census
/// wants its head, not its text.
fn truncate(s: &str) -> String {
    let head: String = s.chars().take(600).collect();
    if head.len() < s.len() {
        format!("{head} …[{} chars]", s.len())
    } else {
        head
    }
}

/// One document at one scale and one box, replayed under the shipped
/// tier with the shape report installed.
fn census_at(label: &str, doc: &ProfileDoc, whole: bool, rules: SymRules, tol: Tol) -> usize {
    let analyzed = analyzed_box(doc, &AnalysisPolicy::default());
    let box_ = if whole {
        ParamBox::of(&analyzed)
    } else {
        nominal_box(&analyzed)
    };
    let (shapes, refusal, counts) = replay(doc, &box_, rules, tol);
    println!("{label:<46} {counts:?} -> {refusal:?}");
    report_one(label, &shapes)
}

/// **THE CLAUSE-1 `Invalid` CENSUS** on the six documents, at the
/// nominal (a degenerate box) and at ceiling + δ (the refusing end of
/// each study's pinned bisection bracket, `measured_studies`), under
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
        // **THE PAD IS OPT-IN**, by name. Its replay under the shape
        // report — at the nominal as well as over the whole box — is
        // killed for memory on a box of this size, and a process that
        // dies takes the rows after it with it. Asking for it by name
        // (`CAD_DECIDE_1_DOC=r2_rounded_pad`) is what says the run is
        // for the pad and may end that way.
        if study.name == "r2_rounded_pad" && only.is_none() {
            println!(
                "{:<42} SKIPPED — opt in with CAD_DECIDE_1_DOC=r2_rounded_pad; the shape report \
                 over this document does not fit in the memory of a box this size",
                "r2_rounded_pad"
            );
            continue;
        }
        // **THE CEILING IS MEASURED, NEVER READ OFF A TABLE.**
        // `measured_studies`' brackets are M10-9's tier's, and this
        // census runs the SHIPPED one, whose ceilings are elsewhere;
        // replaying at another tier's refusing end is a replay that
        // decides everything, and a zero `Invalid` over it measures
        // nothing. Same call M10-10's evidence bench makes.
        let (lo, hi, per) = ceiling(&*study.at, rules, tol, 1.0e-1 * eps, 1.0e1, 16);
        println!(
            "{:<42} certifies x{lo:e}, refuses x{hi:e} ({per:.2}s/probe) [{:.4e}..{:.4e} eps]",
            study.name,
            lo / eps,
            hi / eps
        );
        let nominal = (study.at)(if lo.is_finite() { lo } else { study.certifies_at * eps });
        census_at(
            &format!("{} / nominal", study.name),
            &nominal,
            false,
            rules,
            tol,
        );
        if !hi.is_finite() {
            println!(
                "{:<42} SKIPPED — no finite refusing end in [1e-1 eps, 1e1]",
                format!("{} / ceiling + delta", study.name)
            );
            continue;
        }
        census_at(
            &format!("{} / ceiling + delta", study.name),
            &(study.at)(hi),
            true,
            rules,
            tol,
        );
    }
    explain_depth(0);
}
