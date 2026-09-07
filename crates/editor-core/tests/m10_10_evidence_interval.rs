//! **M10-10's measurement bench** — the form-level mechanism (rule D:
//! trig of `atan` made exact; rules A/B per node made affordable) on
//! the five documents, read the way
//! `work/m10/first-refusal-at-twice-the-ceiling-is-an-order-artefact`
//! prescribes: a bound is the OVER-BAND SET at ceiling + δ, never the
//! first name a drive reports at a multiple of the ceiling.
//!
//! Every row here is an `#[ignore]`d evidence probe that prints and
//! asserts nothing a gate could read ([[test-suite-cost]]); the
//! positive pins live in `m10_10_pins_interval.rs`. Run them:
//!
//! ```sh
//! cargo test -p editor-core --features interval --test all -- \
//!   m10_10_evidence_interval:: --ignored --nocapture --test-threads 1
//! ```
#![cfg(feature = "interval")]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::collections::BTreeMap;

use editor_core::ProfileDoc;
use editor_core::analysis::{AnalysisPolicy, ParamBox, analyzed_box};
use geom_core::sym::report::{DecisionShape, ShapeOutcome};
use geom_core::{SymRules, Tol};

use crate::m10_8_arc_family_interval::replay;
use crate::m10_8_harness::nominal_box;

/// A named study: a document as a function of the SCALE of its real
/// study, so a ceiling is a multiple of the study a user would ask for.
type NamedStudy = (&'static str, Box<dyn Fn(f64) -> ProfileDoc>);

/// The five documents M10-9 measured, at the same scales.
fn documents(tol: Tol) -> Vec<NamedStudy> {
    vec![
        (
            "two_hole_plate",
            Box::new(move |s: f64| crate::m10_7_plate::plate(5.0e-5 * s, 1.0e-5 * s, tol).0)
                as Box<dyn Fn(f64) -> ProfileDoc>,
        ),
        (
            "r2_filleted_bracket",
            Box::new(move |s: f64| crate::m10_7_r2_probes_interval::bracket(s, tol).0),
        ),
        (
            "r1_annulus",
            Box::new(move |s: f64| crate::m10_8_r1_probes_interval::annulus(s, tol).0),
        ),
        (
            "r2_rounded_pad",
            Box::new(move |s: f64| crate::m10_8_r2_probes_interval::pad(s, tol).0),
        ),
        (
            "r2_link",
            Box::new(move |s: f64| crate::m10_9_r2_probes_interval::link(s, tol).0),
        ),
    ]
}

/// The four identity residuals the staged walk names, in its order.
const THE_FOUR: [&str; 4] = [
    "carrier_matches_mapped_source",
    "carrier_on_surface_2",
    "pcurve_map_residual",
    "witness_on_surface_2",
];

/// The atom KINDS a rendered form carries, counted by name.
fn atoms_of(rendered: &str) -> BTreeMap<&'static str, usize> {
    let mut out = BTreeMap::new();
    let count = |kind: &str| rendered.matches(kind).count();
    for (name, n) in [
        ("sin", count("sin(")),
        ("cos", count("cos(")),
        // `tan(` is a suffix of `atan(`; `atan(` of `atan2(`.
        ("tan", count("tan(") - count("atan(")),
        ("atan", count("atan(") - count("atan2(")),
        ("atan2", count("atan2(")),
        ("sqrt", count("sqrt(")),
        ("abs", count("abs(")),
    ] {
        if n > 0 {
            out.insert(name, n);
        }
    }
    out
}

/// **§1 — THE FOUR RESIDUALS, RENDERED**, at the plate's NOMINAL under
/// the shipped tier: every decide site decides at a point, so this is
/// where every residual's form is built. Per predicate the outcome
/// split, and for each of the four the first rendered forms — PLAIN
/// beside EARLY (A0 folded, the walk rule D runs in) — with their
/// sizes and the atom kinds each carries.
#[test]
#[ignore = "evidence-only: renders the plate's four identity residuals at the nominal"]
fn m10_10_the_four_residuals_rendered_at_the_nominal() {
    let tol = Tol::witness();
    let rules = rules_from_env();
    // `CAD_M10_10_EXPLAIN=<levels>` renders the DAG below each shown
    // residual to that depth, naming the node the early walk froze.
    geom_core::sym::report::explain_depth(
        std::env::var("CAD_M10_10_EXPLAIN")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(0),
    );
    let doc = crate::m10_7_plate::plate(5.0e-5, 1.0e-5, tol).0;
    let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
    let (shapes, refusal, counts) = replay(&doc, &nominal_box(&analyzed), rules, tol);
    println!("== plate at the nominal, rules {rules:?}: {counts:?}; refusal {refusal:?}");
    print_split(&shapes);
    let show = std::env::var("CAD_M10_10_SHOW")
        .ok()
        .and_then(|v| v.parse::<usize>().ok())
        .unwrap_or(2);
    for pred in THE_FOUR {
        // The outcomes in evaluation order — nine per curve, so the
        // sample index of each numeric decision can be read off.
        let sequence: String = shapes
            .iter()
            .filter(|s| s.predicate == pred)
            .map(|s| match s.outcome {
                ShapeOutcome::Theorem => 'T',
                ShapeOutcome::SignGated => 'G',
                ShapeOutcome::Registered => 'R',
                _ => 'n',
            })
            .collect();
        println!("-- {pred} in evaluation order: {sequence}");
        let mut seen = 0;
        for s in shapes.iter().filter(|s| s.predicate == pred) {
            if matches!(
                s.outcome,
                ShapeOutcome::Theorem | ShapeOutcome::SignGated | ShapeOutcome::Registered
            ) {
                continue;
            }
            seen += 1;
            if seen > show {
                continue;
            }
            println!("-- {pred} #{seen} [{:?}] sizes {:?}", s.outcome, s.sizes);
            if let Some(f) = &s.form {
                println!("   PLAIN atoms {:?}\n   {f}", atoms_of(f));
            }
            if let Some(f) = &s.early_form {
                println!("   EARLY atoms {:?}\n   {f}", atoms_of(f));
            }
            if let Some(x) = &s.explain {
                println!("   EXPLAIN\n{x}");
            }
        }
        println!("   ({seen} numeric {pred} decisions)");
    }
}

/// The per-predicate outcome split of one replay, printed.
fn print_split(shapes: &[DecisionShape]) {
    let mut table: BTreeMap<&'static str, [u64; 4]> = BTreeMap::new();
    for s in shapes {
        let row = table.entry(s.predicate).or_default();
        let k = match s.outcome {
            ShapeOutcome::Theorem => 0,
            ShapeOutcome::SignGated => 1,
            ShapeOutcome::Registered => 2,
            _ => 3,
        };
        row[k] += 1;
    }
    for (pred, row) in &table {
        println!("   {pred:<34} {row:?} (theorem/gated/registered/numeric)");
    }
}

/// The rule set `CAD_M10_10_RULES` names (the shipped set unset).
fn rules_from_env() -> SymRules {
    std::env::var("CAD_M10_10_RULES").map_or(SymRules::shipped(), |v| rules_named(&v))
}

/// A rule set by name, for the env-driven rows.
fn rules_named(name: &str) -> SymRules {
    match name.trim() {
        "shipped" => SymRules::shipped(),
        "none" => SymRules::none(),
        "all" => SymRules::all(),
        "shut" => SymRules::shipped_without_the_door(),
        other => panic!("unknown rule set {other:?}: shipped | none | all | shut"),
    }
}

/// **The ceilings and the over-band set at ceiling + δ**, per document,
/// under a chosen rule set (`CAD_M10_10_RULES`, default shipped) —
/// the bracket at both ends of a 16-step log bisection, and every
/// predicate over the band at the refusing end with its enclosure.
/// `CAD_M10_10_DOCS` names a comma-separated subset. Run once per ε
/// row (the tolerance is a `OnceLock`).
#[test]
#[ignore = "evidence-only: the ceilings and the over-band set at ceiling + delta"]
fn m10_10_ceilings_and_the_over_band_set() {
    let tol = Tol::witness();
    let eps = tol.eps();
    let rules = rules_from_env();
    println!("== eps = {eps:e}, rules {rules:?}");
    let only = std::env::var("CAD_M10_10_DOCS")
        .ok()
        .filter(|s| !s.trim().is_empty());
    for (name, at) in documents(tol) {
        if only
            .as_deref()
            .is_some_and(|l| !l.split(',').any(|n| n.trim() == name))
        {
            continue;
        }
        let (lo, hi, per) =
            crate::m10_8_harness::ceiling(&*at, rules, tol, 1.0e-1 * eps, 1.0e1, 16);
        println!(
            "   {name:<20}: certifies x{lo:e}, refuses x{hi:e} ({per:.2}s/probe) \
             [= {:.4e}·eps .. {:.4e}·eps]",
            lo / eps,
            hi / eps
        );
        if !(lo.is_finite() && hi.is_finite()) {
            continue;
        }
        let doc = at(hi);
        let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
        let (shapes, _, counts) = replay(&doc, &ParamBox::of(&analyzed), rules, tol);
        println!("      at ceiling+delta ({:.4e}·eps): {counts:?}", hi / eps);
        println!(
            "{}",
            crate::m10_8_harness::render_over_band(&crate::m10_8_harness::over_band_set(&shapes))
        );
    }
}
