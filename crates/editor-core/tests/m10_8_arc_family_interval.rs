//! **The arc family, measured before its mechanism** (ERROR-DESIGN
//! E12's reserve clause, executed as measurement first): per decide
//! site, on the three documents where the symbolic tier was seen to
//! miss — the tour's two-hole plate, R2's filleted L-bracket with two
//! bores and R1's parametric bracket — which of the atom-algebra rules
//! (`geom_core::SymRules`: A `sqrt(X)² = X`, B `sin² + cos² = 1`, C
//! `sqrt(Q²) = Q` by a certified sign) discharges it, and for the sites
//! that stay numeric with every rule on, the SHAPE of the residual that
//! blocks them.
//!
//! Two replays per document, because they answer different questions:
//!
//! - **the point replay** at the nominal, per rule set — every decide
//!   site decides (no node refuses at a point), and the identity test
//!   asks the same question there it asks over a box, so this is the
//!   complete per-site table: theorem / sign-gated / numeric under
//!   none, A, A+B, A+B+C;
//! - **the whole-box replay** at the real study, every rule on — what
//!   actually BOUNDS the box: the outcome per predicate up to the first
//!   refusal in each node, and the rendered normal form of every
//!   residual the numeric channel could not decide.
//!
//! Then the ceilings: the widest whole-certifying box of the plate and
//! the bracket per rule set, and what refuses first beyond each.
//!
//! **NO TEST IN THIS FILE IS EXECUTED BY CI** — every row is an
//! `#[ignore]`d evidence probe that prints and asserts nothing a gate
//! could read ([[test-suite-cost]]); the positive pins the measurement
//! justifies live in `m10_8_pins_interval.rs`. Run them:
//!
//! ```sh
//! cargo test -p editor-core --features interval --test all -- \
//!   m10_8_arc_family_interval:: --ignored --nocapture
//! ```
#![cfg(feature = "interval")]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::collections::BTreeMap;
use std::sync::Arc;

use editor_core::analysis::{AnalysisPolicy, BoxAxis, ParamBox, analyzed_box};
use editor_core::drive::{
    DEFAULT_SYM_MAX_DEGREE, DEFAULT_SYM_MAX_TERMS, DriveConfig, SymbolicDials, drive,
};
use editor_core::{CancelToken, EvalOptions, NodeResult, ProfileDoc, ProfileLift, evaluate};
use geom_core::sym::report::{
    DecisionShape, ShapeOutcome, name_param, start_shape_report, take_shape_report,
};
use geom_core::sym::with_session_rules;
use geom_core::{Sign, SymBudget, SymRules, Tol};

use crate::m10_7_plate::plate;
use crate::m10_7_r1_probes_interval::bracket_pub as r1_bracket;
use crate::m10_7_r2_probes_interval::bracket as r2_bracket;

fn budget() -> SymBudget {
    SymBudget {
        max_terms: DEFAULT_SYM_MAX_TERMS,
        max_degree: DEFAULT_SYM_MAX_DEGREE,
    }
}

/// The rule sets the table is cut against. Cumulative A / A+B / A+B+C
/// answers "which rule adds what", and the isolated A / B / C answer
/// "which rule needs which" — rule C's fold of the normalized frame's
/// `sqrt(v·v)²` for instance depends on rule A having reduced it first,
/// while rule A's eager expansion can COST a higher `sqrt(Xa)−sqrt(Xb)`
/// cancellation, so the two have to be seen apart as well as together.
fn rule_sets() -> [(&'static str, SymRules); 6] {
    let only = |sqrt_square, pythagoras, signed_root| SymRules {
        sqrt_square,
        pythagoras,
        signed_root,
    };
    [
        ("none", SymRules::none()),
        ("A", only(true, false, false)),
        ("B", only(false, true, false)),
        ("C", only(false, false, true)),
        ("A+B", only(true, true, false)),
        ("A+B+C", SymRules::all()),
    ]
}

/// The three documents, each at its REAL study.
pub(crate) fn documents(tol: Tol) -> Vec<(&'static str, ProfileDoc)> {
    vec![
        ("two_hole_plate", plate(5.0e-5, 1.0e-5, tol).0),
        ("r2_filleted_bracket", r2_bracket(1.0, tol).0),
        ("r1_bracket", r1_bracket(0.5e-3, false, tol).0),
    ]
}

/// The degenerate box at the nominal.
fn nominal_box(analyzed: &editor_core::analysis::AnalyzedBox) -> ParamBox {
    ParamBox::from_axes(
        ParamBox::of(analyzed)
            .axes()
            .keys()
            .map(|n| (n.clone(), BoxAxis::Fixed))
            .collect(),
    )
}

/// One replay at `Sym<Interval>` over `box_` with the shape report on:
/// every decision recorded, and the first refusal, if any.
pub(crate) fn replay(
    doc: &ProfileDoc,
    box_: &ParamBox,
    rules: SymRules,
    tol: Tol,
) -> (Vec<DecisionShape>, Option<String>, geom_core::SymCounts) {
    for name in box_.axes().keys() {
        name_param(&name.0);
    }
    let opts = EvalOptions {
        param_box: Some(Arc::new(box_.clone())),
        profile_lift: ProfileLift::Guided,
        ..EvalOptions::default()
    };
    start_shape_report();
    let (first_refusal, counts) = with_session_rules(budget(), rules, || {
        let ev: editor_core::Evaluation<geom_core::Sym<geom_core::Interval>> =
            evaluate(doc, None, &CancelToken::new(), &opts, tol);
        ev.order.iter().find_map(|id| match ev.result(*id) {
            Some(NodeResult::Failed(e)) => Some(format!("node {} — {}", id.0, e.kind)),
            _ => None,
        })
    });
    (take_shape_report(), first_refusal, counts)
}

#[derive(Default, Clone, Copy)]
struct Split {
    theorem: u64,
    gated: u64,
    definite: u64,
    numeric_zero: u64,
    indeterminate: u64,
    invalid: u64,
}

impl Split {
    fn add(&mut self, o: ShapeOutcome) {
        match o {
            ShapeOutcome::Theorem => self.theorem += 1,
            ShapeOutcome::SignGated => self.gated += 1,
            ShapeOutcome::Definite(_) => self.definite += 1,
            ShapeOutcome::NumericZero => self.numeric_zero += 1,
            ShapeOutcome::Indeterminate => self.indeterminate += 1,
            ShapeOutcome::Invalid => self.invalid += 1,
        }
    }

    /// Decisions the symbolic tier did NOT answer.
    fn numeric(self) -> u64 {
        self.definite + self.numeric_zero + self.indeterminate + self.invalid
    }
}

fn split_by_predicate(shapes: &[DecisionShape]) -> BTreeMap<&'static str, Split> {
    let mut out: BTreeMap<&'static str, Split> = BTreeMap::new();
    for s in shapes {
        out.entry(s.predicate).or_default().add(s.outcome);
    }
    out
}

/// **§1's table.** Per document, per predicate: the theorem /
/// sign-gated / numeric split at the nominal under each rule set, then
/// the whole-box outcome split with every rule on and the residual
/// shapes of what the numeric channel could not decide there.
#[test]
#[ignore = "evidence-only: prints the per-predicate table under each rule set"]
fn m10_8_table_per_predicate_under_each_rule_set() {
    let tol = Tol::witness();
    for (name, doc) in documents(tol) {
        let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
        let nominal = nominal_box(&analyzed);
        let real = ParamBox::of(&analyzed);

        println!("== {name}: POINT replay at the nominal, per rule set (theorem/gated/numeric)");
        let mut table: BTreeMap<&'static str, Vec<Split>> = BTreeMap::new();
        for (_, rules) in rule_sets() {
            let (shapes, refusal, _) = replay(&doc, &nominal, rules, tol);
            assert!(refusal.is_none(), "{name} refuses at its own nominal: {refusal:?}");
            for (pred, split) in split_by_predicate(&shapes) {
                table.entry(pred).or_default().push(split);
            }
        }
        let labels: Vec<&str> = rule_sets().iter().map(|(l, _)| *l).collect();
        print!("   {:<34}", "predicate (theorem/gated/numeric)");
        for l in &labels {
            print!(" {l:>12}");
        }
        println!();
        for (pred, cols) in &table {
            print!("   {pred:<34}");
            for s in cols {
                print!(" {:>12}", format!("{}/{}/{}", s.theorem, s.gated, s.numeric()));
            }
            println!();
        }
        // Per rule, which predicates it moved OFF the numeric channel
        // relative to `none`, and which it moved ONTO it (a regression).
        for (i, (label, _)) in rule_sets().iter().enumerate().skip(1) {
            let helped: Vec<&str> = table
                .iter()
                .filter(|(_, c)| c[i].numeric() < c[0].numeric())
                .map(|(p, _)| *p)
                .collect();
            let hurt: Vec<&str> = table
                .iter()
                .filter(|(_, c)| c[i].numeric() > c[0].numeric())
                .map(|(p, _)| *p)
                .collect();
            println!("   rule {label:<6} helped {helped:?}  HURT {hurt:?}");
        }

        println!("== {name}: WHOLE-BOX replay at the real study, rules A+B+C");
        let (shapes, refusal, counts) = replay(&doc, &real, SymRules::all(), tol);
        println!("   counts {counts:?}; first refusal: {refusal:?}");
        println!(
            "   {:<34} {:>8} {:>8} {:>8} {:>8} {:>8} {:>8}",
            "predicate", "theorem", "gated", "definite", "num0", "indet", "invalid"
        );
        for (pred, s) in split_by_predicate(&shapes) {
            println!(
                "   {pred:<34} {:>8} {:>8} {:>8} {:>8} {:>8} {:>8}",
                s.theorem, s.gated, s.definite, s.numeric_zero, s.indeterminate, s.invalid
            );
        }
        println!("   BLOCKING RESIDUALS (numeric, not definite), first two shapes per predicate:");
        let mut seen: BTreeMap<&'static str, usize> = BTreeMap::new();
        for s in &shapes {
            if matches!(s.outcome, ShapeOutcome::Definite(_))
                || matches!(s.outcome, ShapeOutcome::Theorem | ShapeOutcome::SignGated)
            {
                continue;
            }
            let n = seen.entry(s.predicate).or_default();
            if *n >= 2 {
                continue;
            }
            *n += 1;
            println!(
                "   [{:?}] {}: {}",
                s.outcome,
                s.predicate,
                s.form.as_deref().unwrap_or("<no form>")
            );
        }
    }
}

/// The widest scale of a document's real study that certifies WHOLE
/// (`max_depth = 0`), by bisection of the log of the scale, and the
/// first refusal beyond it.
pub(crate) fn ceiling(
    doc_at: &dyn Fn(f64) -> ProfileDoc,
    dials: SymbolicDials,
    tol: Tol,
) -> (f64, Option<String>) {
    let certifies_whole = |scale: f64| {
        let doc = doc_at(scale);
        let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
        drive(
            &doc,
            &analyzed,
            &DriveConfig {
                max_depth: 0,
                max_leaves: 1,
                symbolic: dials,
                ..DriveConfig::default()
            },
            tol,
        )
        .is_ok_and(|v| v.receipt().certified == 1)
    };
    let (mut lo, mut hi) = (1.0e-14, 1.0e3);
    if !certifies_whole(lo) {
        return (f64::NAN, None);
    }
    if certifies_whole(hi) {
        return (f64::INFINITY, None);
    }
    for _ in 0..40 {
        let mid = (0.5 * (lo.ln() + hi.ln())).exp();
        if certifies_whole(mid) {
            lo = mid;
        } else {
            hi = mid;
        }
    }
    // What refuses first beyond it, from a whole-box replay at twice
    // the ceiling.
    let doc = doc_at(lo * 2.0);
    let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
    let (_, refusal, _) = replay(&doc, &ParamBox::of(&analyzed), dials.rules, tol);
    (lo, refusal)
}

/// **The ceilings, per rule set**, on the plate and on R2's bracket.
#[test]
#[ignore = "evidence-only: prints the ceilings per rule set and the first refusal beyond each"]
fn m10_8_ceilings_per_rule_set() {
    let tol = Tol::witness();
    let plate_at = |scale: f64| plate(5.0e-5 * scale, 1.0e-5 * scale, tol).0;
    let bracket_at = |scale: f64| r2_bracket(scale, tol).0;
    let docs: [(&str, &dyn Fn(f64) -> ProfileDoc); 2] =
        [("two_hole_plate", &plate_at), ("r2_filleted_bracket", &bracket_at)];
    // The decisive sets only — the ceiling drive is expensive, and
    // these four separate the questions: `none` is the pre-algebra
    // tier, `C` isolates the sign fold, `A` the square reduction it
    // depends on, `all` both together.
    let sets = [
        ("none", SymRules::none()),
        ("A", SymRules { sqrt_square: true, pythagoras: false, signed_root: false }),
        ("C", SymRules { sqrt_square: false, pythagoras: false, signed_root: true }),
        ("all", SymRules::all()),
    ];
    for (name, doc_at) in docs {
        for (label, rules) in sets {
            let dials = SymbolicDials {
                rules,
                ..SymbolicDials::default()
            };
            let (c, refusal) = ceiling(doc_at, dials, tol);
            println!("   {name} rules={label:<6} ceiling x{c:e} of the real study; beyond: {refusal:?}");
        }
        let (c, refusal) = ceiling(doc_at, SymbolicDials::off(), tol);
        println!("   {name} tier OFF     ceiling x{c:e} of the real study; beyond: {refusal:?}");
    }
}

/// The blocking shapes just beyond a ceiling, named with their
/// predicate and enclosure — the numbers the caption and the deviations
/// table quote.
#[test]
#[ignore = "evidence-only: prints what bounds each document just past its ceiling"]
fn m10_8_what_bounds_each_document_past_its_ceiling() {
    let tol = Tol::witness();
    let plate_at = |scale: f64| plate(5.0e-5 * scale, 1.0e-5 * scale, tol).0;
    let bracket_at = |scale: f64| r2_bracket(scale, tol).0;
    let docs: [(&str, &dyn Fn(f64) -> ProfileDoc); 2] =
        [("two_hole_plate", &plate_at), ("r2_filleted_bracket", &bracket_at)];
    for (name, doc_at) in docs {
        let (c, _) = ceiling(doc_at, SymbolicDials::default(), tol);
        for factor in [2.0, 1.0 / c] {
            let doc = doc_at(c * factor);
            let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
            let (shapes, refusal, counts) =
                replay(&doc, &ParamBox::of(&analyzed), SymRules::all(), tol);
            println!("== {name} at x{:e} (ceiling x{c:e}): {counts:?}; first refusal {refusal:?}", c * factor);
            for s in shapes.iter().filter(|s| {
                matches!(s.outcome, ShapeOutcome::Indeterminate | ShapeOutcome::Invalid)
            }) {
                println!("   [{:?}] {}: {}", s.outcome, s.predicate, s.form.as_deref().unwrap_or("-"));
            }
            let _ = Sign::Zero;
        }
    }
}
