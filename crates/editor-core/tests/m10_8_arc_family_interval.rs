//! **The arc family, measured before its mechanism** (ERROR-DESIGN
//! E12's reserve clause, executed as measurement first): per decide
//! site, on the three documents where the symbolic tier was seen to
//! miss — the tour's two-hole plate, R2's filleted L-bracket with two
//! bores and R1's parametric bracket — which of the BUILDABLE
//! atom-algebra rules (`geom_core::SymRules`: A `sqrt(X)² = X`,
//! B `sin² + cos² = 1`; rule C, the clause-3 sign fold, is filed
//! unbuilt) discharges it, and for the sites that stay numeric with
//! every rule on, the SHAPE of the residual that blocks them.
//!
//! Two replays per document, because they answer different questions:
//!
//! - **the point replay** at the nominal, per rule set — every decide
//!   site decides (no node refuses at a point), and the identity test
//!   asks the same question there it asks over a box, so this is the
//!   complete per-site table: theorem / sign-gated / numeric under
//!   none, A, B, A+B;
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

use editor_core::analysis::{AnalysisPolicy, ParamBox, analyzed_box};
use editor_core::drive::{DEFAULT_SYM_MAX_DEGREE, DEFAULT_SYM_MAX_TERMS, SymbolicDials};
use editor_core::{CancelToken, EvalOptions, NodeResult, ProfileDoc, ProfileLift, evaluate};
use geom_core::sym::report::{
    DecisionShape, ShapeOutcome, name_param, start_shape_report, take_shape_report,
};
use geom_core::sym::with_session_rules;
use geom_core::{Sign, SymBudget, SymRules, Tol};

use crate::m10_7_plate::plate;
use crate::m10_7_r1_probes_interval::bracket_pub as r1_bracket;
use crate::m10_7_r2_probes_interval::bracket as r2_bracket;
use crate::m10_8_harness::nominal_box;

fn budget() -> SymBudget {
    SymBudget {
        max_terms: DEFAULT_SYM_MAX_TERMS,
        max_degree: DEFAULT_SYM_MAX_DEGREE,
    }
}

/// The rule LADDER the table is cut against, each rung adding one
/// mechanism to the last: `none` is M10-7's tier; `A0` the constant
/// fold in the plain form; `A0+AB_top` rules A/B over the top residual;
/// `A0+C_early` rule C's fold in the early walk (the SHIPPED set);
/// `+AB_top` both. Reading the columns left to right says what each
/// mechanism reaches that the previous ones did not. The per-node A/B
/// reduction (`early_ab`) is not a rung: minutes per replay on the
/// BigInt ring (`m10_8_r1_probes_interval::slow_variants`).
fn rule_sets() -> [(&'static str, SymRules); 5] {
    let n = SymRules::none();
    [
        ("none", n),
        (
            "A0",
            SymRules {
                const_fold: true,
                ..n
            },
        ),
        (
            "A0+AB_top",
            SymRules {
                const_fold: true,
                sqrt_square: true,
                pythagoras: true,
                ..n
            },
        ),
        ("A0+C_early", SymRules::shipped()),
        (
            "+AB_top",
            SymRules {
                sqrt_square: true,
                pythagoras: true,
                ..SymRules::shipped()
            },
        ),
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
            assert!(
                refusal.is_none(),
                "{name} refuses at its own nominal: {refusal:?}"
            );
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
                print!(
                    " {:>12}",
                    format!("{}/{}/{}", s.theorem, s.gated, s.numeric())
                );
            }
            println!();
        }
        // Per rung, which predicates it moved relative to `none` — the
        // WHOLE split compared (theorem, gated, numeric), so a theorem
        // re-labelled as gated, or a numeric decision that became a
        // theorem, both show. `helped` is more discharged; `HURT` is a
        // theorem lost (fewer theorems, or fewer discharged in all).
        for (i, (label, _)) in rule_sets().iter().enumerate().skip(1) {
            let cell = |s: &Split| format!("{}/{}/{}", s.theorem, s.gated, s.numeric());
            let helped: Vec<String> = table
                .iter()
                .filter(|(_, c)| c[i].theorem + c[i].gated > c[0].theorem + c[0].gated)
                .map(|(p, c)| format!("{p}:{}->{}", cell(&c[0]), cell(&c[i])))
                .collect();
            let hurt: Vec<String> = table
                .iter()
                .filter(|(_, c)| {
                    c[i].theorem < c[0].theorem
                        || c[i].theorem + c[i].gated < c[0].theorem + c[0].gated
                })
                .map(|(p, c)| format!("{p}:{}->{}", cell(&c[0]), cell(&c[i])))
                .collect();
            println!("   rung {label:<12} helped {helped:?}\n        HURT   {hurt:?}");
        }

        println!("== {name}: WHOLE-BOX replay at the real study, the SHIPPED rule set");
        let (shapes, refusal, counts) = replay(&doc, &real, SymRules::shipped(), tol);
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
/// (`max_depth = 0`), by bisection of the log of the scale
/// (`m10_8_harness::ceiling`), and the first refusal beyond it.
pub(crate) fn ceiling(
    doc_at: &dyn Fn(f64) -> ProfileDoc,
    dials: SymbolicDials,
    tol: Tol,
) -> (f64, Option<String>) {
    let (lo, hi, _) = crate::m10_8_harness::ceiling(doc_at, dials.rules, tol, 1.0e-14, 1.0e3, 40);
    if lo.is_nan() || hi.is_infinite() {
        return (lo, None);
    }
    // What refuses first beyond it, from a whole-box replay at twice
    // the ceiling.
    let doc = doc_at(lo * 2.0);
    let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
    let (_, refusal, _) = replay(&doc, &ParamBox::of(&analyzed), dials.rules, tol);
    (lo, refusal)
}

/// **Claim 7 — the cost per LEAF, per rule set**: one whole-box replay
/// of the plate and of R2's bracket at a scale each certifies, timed
/// under the plain tier (M10-7's D17 baseline), A0 alone, the early
/// walk without rule C, and the shipped set. EVIDENCE-ONLY (prints).
#[test]
#[ignore = "evidence-only: prints the leaf cost per rule set"]
fn m10_8_leaf_cost_per_rule_set() {
    let tol = Tol::witness();
    let eps = tol.eps();
    let n = SymRules::none();
    let sets = [
        ("none", n),
        (
            "A0",
            SymRules {
                const_fold: true,
                ..n
            },
        ),
        (
            "A0+early",
            SymRules {
                const_fold: true,
                early: true,
                ..n
            },
        ),
        ("shipped", SymRules::shipped()),
    ];
    let docs: [(&str, ProfileDoc); 2] = [
        (
            "plate x1e2·eps",
            plate(5.0e-5 * 1.0e2 * eps, 1.0e-5 * 1.0e2 * eps, tol).0,
        ),
        ("bracket x1e1·eps", r2_bracket(1.0e1 * eps, tol).0),
    ];
    for (name, doc) in &docs {
        for (label, rules) in sets {
            let t = std::time::Instant::now();
            let ok = crate::m10_8_harness::certifies_whole(doc, rules, tol);
            println!(
                "   {name:<18} {label:<10} certifies_whole={ok} in {:.2}s",
                t.elapsed().as_secs_f64()
            );
        }
    }
}

/// **The ceilings, per rule set**, on the plate and on R2's bracket.
#[test]
#[ignore = "evidence-only: prints the ceilings per rule set and the first refusal beyond each"]
fn m10_8_ceilings_per_rule_set() {
    let tol = Tol::witness();
    let plate_at = |scale: f64| plate(5.0e-5 * scale, 1.0e-5 * scale, tol).0;
    let bracket_at = |scale: f64| r2_bracket(scale, tol).0;
    let docs: [(&str, &dyn Fn(f64) -> ProfileDoc); 2] = [
        ("two_hole_plate", &plate_at),
        ("r2_filleted_bracket", &bracket_at),
    ];
    // The decisive sets — the ceiling drive is expensive: `none` is the
    // pre-algebra tier, `A` the square reduction, `all` both buildable
    // rules.
    let sets = [
        ("none", SymRules::none()),
        (
            "A",
            SymRules {
                sqrt_square: true,
                pythagoras: false,
                ..SymRules::none()
            },
        ),
        ("all", SymRules::all()),
    ];
    for (name, doc_at) in docs {
        for (label, rules) in sets {
            let dials = SymbolicDials {
                rules,
                ..SymbolicDials::default()
            };
            let (c, refusal) = ceiling(doc_at, dials, tol);
            println!(
                "   {name} rules={label:<6} ceiling x{c:e} of the real study; beyond: {refusal:?}"
            );
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
    let docs: [(&str, &dyn Fn(f64) -> ProfileDoc); 2] = [
        ("two_hole_plate", &plate_at),
        ("r2_filleted_bracket", &bracket_at),
    ];
    for (name, doc_at) in docs {
        let (c, _) = ceiling(doc_at, SymbolicDials::default(), tol);
        for factor in [2.0, 1.0 / c] {
            let doc = doc_at(c * factor);
            let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
            let (shapes, refusal, counts) =
                replay(&doc, &ParamBox::of(&analyzed), SymRules::all(), tol);
            println!(
                "== {name} at x{:e} (ceiling x{c:e}): {counts:?}; first refusal {refusal:?}",
                c * factor
            );
            for s in shapes.iter().filter(|s| {
                matches!(
                    s.outcome,
                    ShapeOutcome::Indeterminate | ShapeOutcome::Invalid
                )
            }) {
                println!(
                    "   [{:?}] {}: {}",
                    s.outcome,
                    s.predicate,
                    s.form.as_deref().unwrap_or("-")
                );
            }
            let _ = Sign::Zero;
        }
    }
}
