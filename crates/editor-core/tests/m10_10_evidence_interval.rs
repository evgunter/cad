//! **M10-10's measurement bench** — the form-level mechanism (rule D:
//! trig of `atan` made exact; rules A/B per node made affordable) on
//! the five documents, read the way M10's closed
//! `first-refusal-at-twice-the-ceiling-is-an-order-artefact`
//! (`docs/DOC-LEDGER.md` sweep 13) prescribes: a bound is the OVER-BAND SET at ceiling + δ, never the
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

/// The five documents M10-9 measured, at the same scales, and the
/// three that author an arc at a bulge other than the circle kernel's
/// `1`: R1's circular-segment boss (a literal `bulge = 2`, a major
/// arc) and R2's D-tab twice (`bulge = 0.4` as a literal, and as a
/// document parameter). The fixtures stay beside the probe rows that
/// own them; this list is the index every env-driven row here reads
/// by name (`CAD_M10_10_DOC`, `CAD_M10_10_DOCS`), and the default set
/// the over-band rows run. The two dyadic CONTROLS are [`controls`],
/// reachable by name only.
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
        (
            "r1_segment_boss",
            Box::new(move |s: f64| crate::m10_10_r1_probes_interval::segment_boss(s, tol).0),
        ),
        (
            "r2_d_tab_literal",
            Box::new(move |s: f64| crate::m10_10_r2_probes_interval::d_tab(s, false, tol).0),
        ),
        (
            "r2_d_tab_parameter",
            Box::new(move |s: f64| crate::m10_10_r2_probes_interval::d_tab(s, true, tol).0),
        ),
    ]
}

/// The dyadic-bulge CONTROLS (`d_tab_at`'s docs): the D-tab at `bulge
/// = 0.5`, literal and parameter, whose sagitta coefficients fit the
/// ring, so what stands there is the sign alone. Not in [`documents`]:
/// the parameter control's whole-box replay costs ~8 s a probe in a
/// dev build, so the over-band rows take them only by name
/// (`CAD_M10_10_DOCS=r2_d_tab_parameter_dyadic`).
fn controls(tol: Tol) -> Vec<NamedStudy> {
    vec![
        (
            "r2_d_tab_literal_dyadic",
            Box::new(move |s: f64| {
                crate::m10_10_r2_probes_interval::d_tab_at(s, false, 0.5, tol).0
            }),
        ),
        (
            "r2_d_tab_parameter_dyadic",
            Box::new(move |s: f64| crate::m10_10_r2_probes_interval::d_tab_at(s, true, 0.5, tol).0),
        ),
    ]
}

/// The named study `CAD_M10_10_DOC` selects (default the plate), from
/// [`documents`] or [`controls`].
fn document_from_env(tol: Tol) -> NamedStudy {
    let name = std::env::var("CAD_M10_10_DOC").unwrap_or_else(|_| "two_hole_plate".into());
    documents(tol)
        .into_iter()
        .chain(controls(tol))
        .find(|(n, _)| *n == name)
        .unwrap_or_else(|| panic!("no document {name:?}"))
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
        // `tan(` is a suffix of `atan(`, so `atan(` matches are
        // subtracted; `atan(` never matches inside `atan2(` (the `2`
        // sits between), so nothing is subtracted there — the first
        // cut subtracted it anyway and the column underflowed to
        // 18446744073709548540 on the pad (R1 MIN-4, R2 m4).
        ("tan", count("tan(") - count("atan(")),
        ("atan", count("atan(")),
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

/// **§1 — THE RESIDUALS, RENDERED**, at one document's NOMINAL
/// (`CAD_M10_10_DOC`, default the plate) under the chosen tier
/// (`CAD_M10_10_RULES`): every decide site decides at a point, so this
/// is where every residual's form is built. Per predicate the outcome
/// split, and for the staged walk's four plus every other predicate
/// that kept a residual NUMERIC with a form, the first rendered forms
/// — PLAIN beside EARLY (A0 folded, the walk rule D runs in) — with
/// their sizes and the atom kinds each carries, and the DAG below each
/// explained to `CAD_M10_10_EXPLAIN` levels.
#[test]
#[ignore = "evidence-only: renders one document's identity residuals at the nominal"]
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
    let (name, at) = document_from_env(tol);
    let doc = at(1.0);
    let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
    let (shapes, refusal, counts) = replay(&doc, &nominal_box(&analyzed), rules, tol);
    println!("== {name} at the nominal, rules {rules:?}: {counts:?}; refusal {refusal:?}");
    print_split(&shapes);
    let show = std::env::var("CAD_M10_10_SHOW")
        .ok()
        .and_then(|v| v.parse::<usize>().ok())
        .unwrap_or(2);
    // The four first, in the walk's order, then any other predicate
    // that kept a residual numeric WITH a form (a decision the numeric
    // channel answered inside the band or could not, where the tier
    // had built the form and declined it).
    let mut preds: Vec<&'static str> = THE_FOUR.to_vec();
    for s in &shapes {
        if s.form.is_some() && !preds.contains(&s.predicate) {
            preds.push(s.predicate);
        }
    }
    // `CAD_M10_10_NEEDLES=a,b,…`: per predicate, how many of its
    // still-numeric decisions carry each substring in the PLAIN form
    // (nothing folded, so no freeze hides an atom) and in the EARLY
    // form (what stood after the rules) — a count of a route's reach
    // that renders nothing.
    if let Some(needles) = std::env::var("CAD_M10_10_NEEDLES")
        .ok()
        .filter(|s| !s.trim().is_empty())
    {
        for needle in needles.split(',').map(str::trim) {
            println!("-- decisions carrying {needle:?} (plain / early / numeric):");
            for pred in &preds {
                let (mut plain, mut early, mut all) = (0, 0, 0);
                for s in shapes.iter().filter(|s| s.predicate == *pred) {
                    let Some(f) = &s.form else { continue };
                    all += 1;
                    plain += usize::from(f.contains(needle));
                    early +=
                        usize::from(s.early_form.as_deref().is_some_and(|e| e.contains(needle)));
                }
                if all > 0 {
                    println!("   {pred:<34} {plain:>3} / {early:>3} / {all:<3}");
                }
            }
        }
    }
    for pred in preds {
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

/// The per-predicate outcome split of one replay
/// (`m10_8_harness::split`), printed.
fn print_split(shapes: &[DecisionShape]) {
    for (pred, row) in &crate::m10_8_harness::split(shapes) {
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
        // SYM-5's differential: the shipped set with rule E (the
        // quotient's common factor) shut, which is M10-10's tier bit
        // for bit.
        "no_e" => SymRules::without_rule_e(),
        // SYM-8's differential: the shipped set with rule F (the
        // manifest sign) SHUT, which is SYM-5's tier bit for bit.
        "no_f" => SymRules::without_rule_f(),
        // DECIDE-3's differential for the decision read: the shipped set
        // with the read shut and rule G on.
        "no_reads" => SymRules::without_the_reads(),
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
        // Rules A/B over the TOP residual only (`discharge`'s site,
        // consulted once the plain and early walks have declined),
        // without the per-node walk: the second site the A/B dials
        // switch on, disclosed as D17 and measured apart.
        "top_only" => SymRules {
            sqrt_square: true,
            pythagoras: true,
            ..SymRules::without_the_algebra()
        },
        // The shipped set WITHOUT the top-residual site's dials would
        // be `early_ab` alone — but the two share the dials, so the
        // top site's contribution is read as shipped minus `ab_only`
        // plus the per-node walk: `d_ab_pernode_only` is rule D with
        // per-node A/B and the top site OFF is not expressible; what
        // IS measurable is `top_only` (the top site alone) and
        // `d_top_only` (rule D with the top site, no per-node walk).
        "d_top_only" => SymRules {
            trig_of_atan: true,
            sqrt_square: true,
            pythagoras: true,
            ..SymRules::without_the_algebra()
        },
        other => panic!(
            "unknown rule set {other:?}: shipped | none | all | shut | off | no_e | no_f \
             | no_reads | d_only | ab_only | top_only | d_top_only"
        ),
    }
}

/// **The ceilings and the over-band set at ceiling + δ**, per document,
/// under a chosen rule set (`CAD_M10_10_RULES`, default shipped) —
/// the bracket at both ends of a 16-step log bisection, and every
/// predicate over the band at the refusing end with its enclosure.
/// `CAD_M10_10_DOCS` names a comma-separated subset of [`documents`]
/// and [`controls`] (the controls run only when named). Evidence-only;
/// the default set is eight documents and takes minutes. Run once per
/// ε row (the tolerance is a `OnceLock`).
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
    let defaults: Vec<&'static str> = documents(tol).iter().map(|(n, _)| *n).collect();
    for (name, at) in documents(tol).into_iter().chain(controls(tol)) {
        let wanted = match only.as_deref() {
            Some(list) => list.split(',').any(|n| n.trim() == name),
            None => defaults.contains(&name),
        };
        if !wanted {
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
    let (name, at) = document_from_env(tol);
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
    use editor_core::drive::{DriveConfig, drive};
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
                    symbolic: crate::m10_8_harness::dials(rules),
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
    // With `CAD_M10_10_CEILINGS` set: each fixture's whole-certifying
    // half-width under the shipped set, bisected between `ε/8` and the
    // authored radius — the number M10-4's rows are scaled against.
    if std::env::var("CAD_M10_10_CEILINGS").is_ok() {
        let whole = |doc: &ProfileDoc| {
            let analyzed = analyzed_box(doc, &AnalysisPolicy::default());
            let v = drive(
                doc,
                &analyzed,
                &DriveConfig {
                    max_leaves: 1,
                    ..DriveConfig::default()
                },
                tol,
            )
            .expect("builds");
            v.receipt().splits == 0 && v.receipt().certified == 1
        };
        let fixtures: [(&str, &dyn Fn(f64) -> ProfileDoc); 2] = [
            ("two_hole_plate (m10_4)", &|h: f64| {
                crate::m10_4_stackup_interval::plate(
                    Some(crate::m10_4_stackup_interval::uniform(h)),
                    Some(crate::m10_4_stackup_interval::uniform(h)),
                )
                .0
            }),
            ("bore_pin_fit", &|h: f64| {
                crate::m10_4_r2_probes_interval::fit(Some(
                    crate::m10_4_r2_probes_interval::uniform(-h, h),
                ))
                .0
            }),
        ];
        for (name, at) in fixtures {
            let (mut lo, mut hi) = (half, 0.1);
            println!(
                "   {name}: lo {lo:e} whole={} hi {hi:e} whole={}",
                whole(&at(lo)),
                whole(&at(hi))
            );
            for _ in 0..20 {
                let mid = 0.5 * (lo + hi);
                if whole(&at(mid)) {
                    lo = mid;
                } else {
                    hi = mid;
                }
            }
            println!(
                "   {name}: whole-certifying half-width bracket [{lo:e}, {hi:e}] at eps {eps:e}"
            );
        }
    }
}

/// **The plate's REAL study driven whole** — ±0.05 mm on the spacing,
/// σ = 0.01 mm on each radius, the tour's own drive (1024 leaves):
/// the receipt, the certified leaves, the refusals BY CLASS with the
/// predicates they name, and the stackup's answer. `CAD_M10_10_SCALE`
/// scales the study (default 1).
#[test]
#[ignore = "evidence-only: drives the plate's real study whole and prints the receipt"]
fn m10_10_the_plates_real_study_driven_whole() {
    use std::collections::BTreeMap;

    use editor_core::drive::{DriveConfig, drive};
    use editor_core::stackup::stackup;

    let tol = Tol::witness();
    let scale: f64 = std::env::var("CAD_M10_10_SCALE")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(1.0);
    let max_leaves: usize = std::env::var("CAD_M10_10_LEAVES")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(1024);
    let (doc, measure, _) = crate::m10_7_plate::plate(5.0e-5 * scale, 1.0e-5 * scale, tol);
    let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
    let t = std::time::Instant::now();
    let verdict = drive(
        &doc,
        &analyzed,
        &DriveConfig {
            max_leaves,
            ..DriveConfig::default()
        },
        tol,
    )
    .expect("the nominal builds");
    println!(
        "== plate x{scale:e} at {max_leaves} leaves: {:?} in {:.2}s; {} certified, {} refused",
        verdict.receipt(),
        t.elapsed().as_secs_f64(),
        verdict.certified().len(),
        verdict.refused().len()
    );
    let mut classes: BTreeMap<String, usize> = BTreeMap::new();
    for leaf in verdict.refused() {
        let key = format!("{:?}", leaf.reason);
        let key = key
            .split(['{', '('])
            .next()
            .unwrap_or(&key)
            .trim()
            .to_owned();
        *classes.entry(key).or_default() += 1;
    }
    println!("   refusals by class: {classes:?}");
    let mut shown = 0;
    for leaf in verdict.refused() {
        if shown < 6 {
            println!("   {:?}", leaf.reason);
            shown += 1;
        }
    }
    println!("{}", verdict.render(&analyzed));
    match stackup(&doc, measure, &analyzed, &verdict, None, true, tol) {
        Ok(report) => println!("   stackup OK:\n{}", report.render(&analyzed)),
        Err(e) => println!("   stackup refused: {e}"),
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
        // so the cost of each rule alone is one env var away. The two
        // rules columns run one attempt per rung (`m10_8_harness::dials`,
        // `SymRetry::none()`); the LADDER columns run the "on" rules with
        // the measured retry ladder (`SymRetry::kept_atom`) and with
        // its two masks in the other order, so what the ladder costs a
        // leaf is its own differential. `CAD_M10_10_TAKES` (default 1)
        // times each column that many times and prints the fastest.
        let takes: usize = std::env::var("CAD_M10_10_TAKES")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(1);
        let ladder = geom_core::SymRetry::kept_atom();
        let reversed = geom_core::SymRetry {
            without: [ladder.without[1], ladder.without[0]],
            ..ladder
        };
        let on = rules_from_env();
        for (label, dials) in [
            (
                "algebra OFF (M10-9)",
                crate::m10_8_harness::dials(SymRules::without_the_algebra()),
            ),
            ("algebra ON  (rules) ", crate::m10_8_harness::dials(on)),
            (
                "ON + the ladder     ",
                editor_core::drive::SymbolicDials {
                    retry: ladder,
                    ..crate::m10_8_harness::dials(on)
                },
            ),
            (
                "ON + ladder reversed",
                editor_core::drive::SymbolicDials {
                    retry: reversed,
                    ..crate::m10_8_harness::dials(on)
                },
            ),
            (
                "ON + rule-G mask only",
                editor_core::drive::SymbolicDials {
                    retry: geom_core::SymRetry {
                        without: [ladder.without[0], None],
                        ..ladder
                    },
                    ..crate::m10_8_harness::dials(on)
                },
            ),
            (
                "ON + rule-A mask only",
                editor_core::drive::SymbolicDials {
                    retry: geom_core::SymRetry {
                        without: [ladder.without[1], None],
                        ..ladder
                    },
                    ..crate::m10_8_harness::dials(on)
                },
            ),
        ] {
            // `CAD_M10_10_PROFILE` installs the tier's cost profile
            // around each take and prints what the retry memos held
            // (`SymProfile::retry_forms`) beside the DAG — the growth
            // guard's measurement. Its hooks cost time, so a timing
            // table is taken with it unset.
            let profiled = std::env::var("CAD_M10_10_PROFILE").is_ok();
            let mut best = f64::INFINITY;
            let mut leaf = (false, geom_core::SymCounts::default());
            let mut held = (Vec::new(), 0);
            for _ in 0..takes.max(1) {
                if profiled {
                    geom_core::sym::profile::start_profile();
                }
                let t = std::time::Instant::now();
                leaf = crate::m10_8_harness::whole_box_leaf(&doc, dials, tol);
                best = best.min(t.elapsed().as_secs_f64());
                if profiled {
                    let p = geom_core::sym::profile::take_profile();
                    held = (p.retry_forms, p.nodes / p.sessions.max(1));
                }
            }
            if profiled {
                println!(
                    "   {name:<20} {label}: retry memos {:?} (DAG {} nodes a session)",
                    held.0, held.1
                );
            }
            let d = leaf.1;
            println!(
                "   {name:<20} x{scale:<10.3e} {label}: certifies_whole={} in {best:.3}s \
                 [{}, {}, {}, {}] retried {}",
                leaf.0, d.symbolic_zero, d.sign_gated, d.registered, d.numeric, d.retried
            );
        }
    }
}

/// **The per-predicate split at the NOMINAL, per document, under a
/// chosen rule set** (`CAD_M10_10_RULES`, default shipped;
/// `CAD_M10_10_DOCS` names a subset of [`documents`] and [`controls`]).
/// The pins hold these tables one rule set at a time
/// (`m10_10_pins_interval`, `m10_bulge_interval`); this row prints them
/// all under any set, which is how a new dial's differential — every
/// predicate that moved, and in which column — is read on all eight
/// documents at once rather than one pin at a time.
#[test]
#[ignore = "evidence-only: the per-document splits at the nominal under a named rule set"]
fn m10_10_splits_at_the_nominal_under_a_rule_set() {
    let tol = Tol::witness();
    let rules = rules_from_env();
    println!("== rules {rules:?}");
    let only = std::env::var("CAD_M10_10_DOCS")
        .ok()
        .filter(|s| !s.trim().is_empty());
    let defaults: Vec<&'static str> = documents(tol).iter().map(|(n, _)| *n).collect();
    for (name, at) in documents(tol).into_iter().chain(controls(tol)) {
        let wanted = match only.as_deref() {
            Some(list) => list.split(',').any(|n| n.trim() == name),
            None => defaults.contains(&name),
        };
        if !wanted {
            continue;
        }
        let t = std::time::Instant::now();
        let table = crate::m10_8_harness::split_at_the_nominal(&at(1.0), rules, tol);
        println!("   {name} ({:.2}s)", t.elapsed().as_secs_f64());
        for (pred, row) in table {
            println!("      {pred:<36} {row:?}");
        }
    }
}
