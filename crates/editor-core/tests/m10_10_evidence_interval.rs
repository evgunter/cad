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
        "off" => SymRules::without_the_algebra(),
        // The cost breakdown: rule D alone, and rules A/B per node alone.
        "d_only" => SymRules {
            trig_of_atan: true,
            ..SymRules::without_the_algebra()
        },
        "ab_only" => SymRules {
            early_ab: true,
            sqrt_square: true,
            pythagoras: true,
            ..SymRules::without_the_algebra()
        },
        other => panic!(
            "unknown rule set {other:?}: shipped | none | all | shut | off | d_only | ab_only"
        ),
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

/// **What stands in the way, rendered**: one document
/// (`CAD_M10_10_DOC`, default the plate) replayed at one scale of its
/// real study (`CAD_M10_10_SCALE`, a multiple of ε unless
/// `CAD_M10_10_ABSOLUTE` is set), the over-band set, and for each
/// over-band predicate the first blocked decision's early form with
/// the DAG below it explained to `CAD_M10_10_EXPLAIN` levels — the row
/// that says which atom a residual still carries and which node the
/// early walk froze.
#[test]
#[ignore = "evidence-only: renders what bounds one document at one scale"]
fn m10_10_what_stands_rendered() {
    let tol = Tol::witness();
    let eps = tol.eps();
    let rules = rules_from_env();
    let name = std::env::var("CAD_M10_10_DOC").unwrap_or_else(|_| "two_hole_plate".into());
    let scale: f64 = std::env::var("CAD_M10_10_SCALE")
        .ok()
        .and_then(|v| v.parse().ok())
        .expect("CAD_M10_10_SCALE names the scale of the real study");
    let scale = if std::env::var("CAD_M10_10_ABSOLUTE").is_ok() {
        scale
    } else {
        scale * eps
    };
    geom_core::sym::report::explain_depth(
        std::env::var("CAD_M10_10_EXPLAIN")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(0),
    );
    let (_, at) = documents(tol)
        .into_iter()
        .find(|(n, _)| *n == name)
        .unwrap_or_else(|| panic!("no document {name:?}"));
    let doc = at(scale);
    let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
    let (shapes, refusal, counts) = replay(&doc, &ParamBox::of(&analyzed), rules, tol);
    println!(
        "== {name} at x{scale:e} ({:.4e}·eps): {counts:?}",
        scale / eps
    );
    println!("   the drive stops at {refusal:?}");
    let set = crate::m10_8_harness::over_band_set(&shapes);
    println!("{}", crate::m10_8_harness::render_over_band(&set));
    for e in &set {
        let Some(s) = shapes.iter().find(|s| {
            s.predicate == e.predicate
                && matches!(
                    s.outcome,
                    ShapeOutcome::Indeterminate | ShapeOutcome::Invalid
                )
        }) else {
            continue;
        };
        println!(
            "-- {} [{:?}] enclosure {:?} sizes {:?}",
            e.predicate, s.outcome, s.enclosure, s.sizes
        );
        if let Some(f) = &s.early_form {
            println!("   EARLY atoms {:?}\n   {f}", atoms_of(f));
        }
        if let Some(x) = &s.explain {
            println!("   EXPLAIN\n{x}");
        }
    }
}

/// **The stackup hulls under both rule sets** — the two ε-scale
/// studies whose worst-case padding M10-4 pinned per half-width (the
/// two-hole plate at `ε/8`, the bore/pin fit at `ε/8`): the drive's
/// leaves and the certified worst-case hull with the algebra off and
/// on, so a padding that moved is read against the LEAVES that moved
/// it. The hull is the union of per-leaf interval enclosures, and an
/// enclosure's dependency padding is proportional to its leaf's width:
/// a tier that certifies a box in fewer, wider leaves certifies a wider
/// hull, and that is the whole of what this row measures.
#[test]
#[ignore = "evidence-only: the stackup hulls with the algebra on and off"]
fn m10_10_the_stackup_hulls_under_both_rule_sets() {
    use editor_core::drive::{DriveConfig, SymbolicDials, drive};
    use editor_core::stackup::stackup;

    let tol = Tol::witness();
    let eps = tol.eps();
    let half = eps / 8.0;
    let plate = crate::m10_4_stackup_interval::plate(
        Some(crate::m10_4_stackup_interval::uniform(half)),
        Some(crate::m10_4_stackup_interval::uniform(half)),
    );
    let fit = crate::m10_4_r2_probes_interval::fit(Some(crate::m10_4_r2_probes_interval::uniform(
        -half, half,
    )));
    let docs: [(&str, &ProfileDoc, editor_core::RecipeNodeId, f64); 2] = [
        ("two_hole_plate eps/8", &plate.0, plate.1, 4.0 * half),
        ("bore_pin_fit eps/8", &fit.0, fit.1, 2.0 * half),
    ];
    for (name, doc, measure, true_range) in docs {
        let analyzed = analyzed_box(doc, &AnalysisPolicy::default());
        for (label, rules) in [
            ("algebra OFF (M10-9)", SymRules::without_the_algebra()),
            ("algebra ON  (rules) ", rules_from_env()),
        ] {
            let verdict = drive(
                doc,
                &analyzed,
                &DriveConfig {
                    max_leaves: 1024,
                    symbolic: SymbolicDials {
                        rules,
                        ..SymbolicDials::default()
                    },
                    ..DriveConfig::default()
                },
                tol,
            )
            .expect("the document builds");
            let report = stackup(doc, measure, &analyzed, &verdict, None, false, tol)
                .unwrap_or_else(|e| panic!("{e}"));
            let wc = report.worst_case;
            println!(
                "   {name:<22} {label}: {:?}; hull [{:.6e}, {:.6e}] over {} leaves; padding {:.3} half-widths",
                verdict.receipt(),
                wc.lo,
                wc.hi,
                wc.leaves,
                ((wc.hi - wc.lo) - true_range) / half
            );
            for leaf in verdict.certified() {
                let spans: Vec<String> = leaf
                    .box_
                    .axes()
                    .iter()
                    .map(|(n, a)| format!("{}={:?}", n.0, a.span()))
                    .collect();
                println!("      leaf {spans:?}");
            }
        }
    }
}

/// **The cost per leaf** — one whole-box replay of each document at a
/// scale it certifies with the algebra OFF, timed with the algebra on
/// and off (`SymRules::shipped` against `SymRules::without_the_algebra`),
/// plus the plate at its REAL study (scale 1), which is the
/// affordability line's own number. `CAD_M10_10_DOCS` names a subset.
#[test]
#[ignore = "evidence-only: prints the leaf cost with the algebra on and off"]
fn m10_10_leaf_cost_with_and_without_the_algebra() {
    let tol = Tol::witness();
    let eps = tol.eps();
    let only = std::env::var("CAD_M10_10_DOCS")
        .ok()
        .filter(|s| !s.trim().is_empty());
    let scales: [(&str, f64); 6] = [
        ("two_hole_plate", 1.0e2 * eps),
        ("two_hole_plate", 1.0),
        ("r2_filleted_bracket", 1.0e1 * eps),
        ("r1_annulus", 1.0e1 * eps),
        ("r2_rounded_pad", 1.0e2 * eps),
        ("r2_link", 1.0e1 * eps),
    ];
    let docs = documents(tol);
    for (name, scale) in scales {
        if only
            .as_deref()
            .is_some_and(|l| !l.split(',').any(|n| n.trim() == name))
        {
            continue;
        }
        let at = &docs
            .iter()
            .find(|(n, _)| *n == name)
            .expect("a named document")
            .1;
        let doc = at(scale);
        // The "on" column is `CAD_M10_10_RULES` (the shipped set unset),
        // so the cost of each rule alone is one env var away.
        for (label, rules) in [
            ("algebra OFF (M10-9)", SymRules::without_the_algebra()),
            ("algebra ON  (rules) ", rules_from_env()),
        ] {
            let t = std::time::Instant::now();
            let ok = crate::m10_8_harness::certifies_whole(&doc, rules, tol);
            println!(
                "   {name:<20} x{scale:<10.3e} {label}: certifies_whole={ok} in {:.3}s",
                t.elapsed().as_secs_f64()
            );
        }
    }
}
