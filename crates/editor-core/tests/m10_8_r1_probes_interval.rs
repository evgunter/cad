//! **R1's independent probes of M10-8** (the arc family, measured before
//! mechanism) — the unit's negative claim attacked with two variants of
//! its own algebra that the unit did not measure:
//!
//! - **A0, the exact constant fold** (`SymRules::const_fold`): `sqrt(c)`
//!   / `abs(c)` of a CONSTANT form whose value is an exact rational
//!   root. The unit's own blocking residuals print `sqrt(1)^58` and
//!   `sqrt(1152921504606847²·2^-114)` — atoms of literal arguments that
//!   cost degree the budget then freezes on. No value is read.
//! - **the EARLY reduction, bounded and ALONGSIDE** (`SymRules::early`):
//!   rules A/B per DAG node in a second memo beside the plain form, at
//!   most 8 substitutions per node, falling back to the un-reduced form
//!   — so it can neither run away nor downgrade a plain-form theorem.
//!
//! EVERY ROW IS EVIDENCE-ONLY (`#[ignore]`d, prints, asserts nothing a
//! gate could read — [[test-suite-cost]]). Run:
//!
//! ```sh
//! cargo test -p editor-core --features interval --test all -- \
//!   m10_8_r1_probes_interval:: --ignored --nocapture
//! ```
#![cfg(feature = "interval")]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
#![allow(dead_code)]

use std::collections::BTreeMap;
use std::time::Instant;

use editor_core::analysis::{AnalysisPolicy, BoxAxis, ParamBox, analyzed_box};
use editor_core::drive::{DriveConfig, SymbolicDials, drive};
use editor_core::{
    Datum, Dimension, Distribution, DocEdit, DocParam, EntityKind, Expr, GeomPred, LoopProgram,
    MeasureExpr, MeasurePrimitive, NamePat, Node, ParamName, ProfileDoc, ProfileProgram,
    RecipeNodeId, Selector, SitedRef, SurfaceKindSet, UnitSym, select_where,
};
use geom_core::sym::report::{DecisionShape, ShapeOutcome};
use geom_core::{SymRules, Tol};

use crate::fixture::Recorder;
use crate::m10_7_plate::plate;
use crate::m10_7_r2_probes_interval::bracket as r2_bracket;
use crate::m10_8_arc_family_interval::replay;

/// The variants, named. `none` is M10-7's tier; `A_top` is the unit's
/// shipped-but-off rule A over the top residual; the rest are R1's.
fn variants() -> Vec<(&'static str, SymRules)> {
    let n = SymRules::none();
    vec![
        ("none", n),
        (
            "A_top",
            SymRules {
                sqrt_square: true,
                ..n
            },
        ),
        (
            "A0",
            SymRules {
                const_fold: true,
                ..n
            },
        ),
        (
            "A0+A_top",
            SymRules {
                const_fold: true,
                sqrt_square: true,
                ..n
            },
        ),
        (
            "AB_early",
            SymRules {
                sqrt_square: true,
                pythagoras: true,
                early: true,
                ..n
            },
        ),
        (
            "A0+AB_early",
            SymRules {
                const_fold: true,
                sqrt_square: true,
                pythagoras: true,
                early: true,
                ..n
            },
        ),
        // A0 ALONGSIDE the plain form (the early memo carries the fold,
        // the plain form is M10-7's): can it lose a theorem?
        (
            "A0_alongside",
            SymRules {
                const_fold: true,
                early: true,
                ..n
            },
        ),
    ]
}

fn dials(rules: SymRules) -> SymbolicDials {
    SymbolicDials {
        rules,
        ..SymbolicDials::default()
    }
}

fn nominal_box(analyzed: &editor_core::analysis::AnalyzedBox) -> ParamBox {
    ParamBox::from_axes(
        ParamBox::of(analyzed)
            .axes()
            .keys()
            .map(|n| (n.clone(), BoxAxis::Fixed))
            .collect(),
    )
}

fn theorems_by_predicate(shapes: &[DecisionShape]) -> BTreeMap<&'static str, (u64, u64)> {
    let mut out: BTreeMap<&'static str, (u64, u64)> = BTreeMap::new();
    for s in shapes {
        let e = out.entry(s.predicate).or_default();
        match s.outcome {
            ShapeOutcome::Theorem | ShapeOutcome::SignGated => e.0 += 1,
            _ => e.1 += 1,
        }
    }
    out
}

fn short(s: &Option<String>) -> String {
    s.as_deref()
        .map_or("-".to_owned(), |s| s.chars().take(150).collect::<String>())
}

/// The documents at their REAL study: the plate, R2's bracket, and R1's
/// own eccentric annulus (below).
fn documents(tol: Tol) -> Vec<(&'static str, ProfileDoc)> {
    vec![
        ("two_hole_plate", plate(5.0e-5, 1.0e-5, tol).0),
        ("r2_filleted_bracket", r2_bracket(1.0, tol).0),
        ("r1_annulus", annulus(1.0, tol).0),
    ]
}

/// **Claim 2/3: the whole-box replay at the real study under each
/// variant** — counts, first refusal, wall time per leaf replay; then
/// the per-predicate theorem split at the nominal against `none`, with
/// the HURT list (a predicate that lost theorems = a downgrade).
///
/// A warm-up replay precedes the table because the shape report
/// attributes decisions made outside any predicate scope to the LAST
/// predicate name set on the thread (the unit's table shows this as the
/// `assert_bound` / `witness_at_mid_parameter` 9-count shift).
///
/// EVIDENCE-ONLY.
#[test]
#[ignore = "evidence-only: R1's variants on the three documents"]
fn r1_variants_whole_box_and_nominal_split() {
    let tol = Tol::witness();
    for (name, doc) in documents(tol) {
        let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
        let real = ParamBox::of(&analyzed);
        let nominal = nominal_box(&analyzed);
        println!("== {name}: WHOLE-BOX replay at the real study, per variant");
        for (label, rules) in variants() {
            let t = Instant::now();
            let (shapes, refusal, counts) = replay(&doc, &real, rules, tol);
            let dt = t.elapsed().as_secs_f64();
            let th: u64 = shapes
                .iter()
                .filter(|s| matches!(s.outcome, ShapeOutcome::Theorem))
                .count() as u64;
            println!(
                "   {label:<12} {dt:>7.2}s counts={counts:?} theorems(report)={th}; first refusal: {}",
                short(&refusal)
            );
        }
        println!(
            "== {name}: NOMINAL split per variant (theorem/numeric), HURT = lost theorems vs none"
        );
        let _ = replay(&doc, &nominal, SymRules::none(), tol);
        let base = theorems_by_predicate(&replay(&doc, &nominal, SymRules::none(), tol).0);
        for (label, rules) in variants() {
            let t = Instant::now();
            let (shapes, _, counts) = replay(&doc, &nominal, rules, tol);
            let dt = t.elapsed().as_secs_f64();
            let split = theorems_by_predicate(&shapes);
            let mut helped = vec![];
            let mut hurt = vec![];
            for (p, (th, nu)) in &split {
                let (bth, bnu) = base.get(p).copied().unwrap_or((0, 0));
                if *th > bth {
                    helped.push(format!("{p}:{bth}/{bnu}->{th}/{nu}"));
                }
                if *th < bth {
                    hurt.push(format!("{p}:{bth}/{bnu}->{th}/{nu}"));
                }
            }
            println!(
                "   {label:<12} {dt:>7.2}s counts={counts:?}\n      helped {helped:?}\n      HURT   {hurt:?}"
            );
        }
    }
}

/// Whether `doc` certifies its WHOLE analyzed box in one leaf.
fn certifies_whole(doc: &ProfileDoc, rules: SymRules, tol: Tol) -> bool {
    let analyzed = analyzed_box(doc, &AnalysisPolicy::default());
    drive(
        doc,
        &analyzed,
        &DriveConfig {
            max_depth: 0,
            max_leaves: 1,
            symbolic: dials(rules),
            ..DriveConfig::default()
        },
        tol,
    )
    .is_ok_and(|v| v.receipt().certified == 1)
}

/// A coarse log-bisection of the widest whole-certifying scale between
/// `lo` and `hi`, with the wall time per probe.
fn ceiling(
    doc_at: &dyn Fn(f64) -> ProfileDoc,
    rules: SymRules,
    tol: Tol,
    lo: f64,
    hi: f64,
    steps: usize,
) -> (f64, f64, f64) {
    let (mut lo, mut hi) = (lo, hi);
    let mut probes = 0.0;
    let mut spent = 0.0;
    let mut probe = |s: f64| {
        let t = Instant::now();
        let ok = certifies_whole(&doc_at(s), rules, tol);
        spent += t.elapsed().as_secs_f64();
        probes += 1.0;
        ok
    };
    if !probe(lo) {
        return (f64::NAN, hi, spent / probes);
    }
    if probe(hi) {
        return (hi, f64::INFINITY, spent / probes);
    }
    for _ in 0..steps {
        let mid = (0.5 * (lo.ln() + hi.ln())).exp();
        if probe(mid) {
            lo = mid;
        } else {
            hi = mid;
        }
    }
    (lo, hi, spent / probes)
}

/// **Claim 2: does the whole-certifying box MOVE under R1's variants?**
/// The bracket and the plate, `none` against each variant, with the
/// cost per probe.
///
/// EVIDENCE-ONLY.
#[test]
#[ignore = "evidence-only: R1's ceilings per variant"]
fn r1_ceilings_per_variant() {
    let tol = Tol::witness();
    let eps = tol.eps();
    let plate_at = |scale: f64| plate(5.0e-5 * scale, 1.0e-5 * scale, tol).0;
    let bracket_at = |scale: f64| r2_bracket(scale, tol).0;
    let annulus_at = |scale: f64| annulus(scale, tol).0;
    let docs: [(&str, &dyn Fn(f64) -> ProfileDoc, f64, f64); 3] = [
        ("two_hole_plate", &plate_at, eps, eps * 1.0e7),
        ("r2_filleted_bracket", &bracket_at, 1.0e-9, 1.0e-1),
        ("r1_annulus", &annulus_at, 1.0e-9, 1.0e1),
    ];
    for (name, doc_at, lo, hi) in docs {
        for (label, rules) in variants() {
            let (c_lo, c_hi, per) = ceiling(doc_at, rules, tol, lo, hi, 12);
            println!(
                "   {name:<20} {label:<12} ceiling in [{c_lo:.3e}, {c_hi:.3e}] of the study; {per:.2}s per probe"
            );
        }
    }
}

// ------------------------------------------------ R1's own document

/// **R1's own arc-bearing document: an ECCENTRIC ANNULUS** — a disc
/// (radius `outer_r`, uniform) with an off-centre bore (centre offset
/// `offset`, uniform; radius `bore_r`, normal), extruded through a
/// DIVISION of a parameter, measuring the wall between the two
/// cylindrical faces and asserting a floor on it. Every wall is an arc
/// carrier: the pure arc family, no straight edge in the profile at all.
///
/// `scale` multiplies every tolerance; `1.0` is the study a user would
/// ask for.
pub(crate) fn annulus(scale: f64, tol: Tol) -> (ProfileDoc, RecipeNodeId, RecipeNodeId) {
    let len = |v: f64| Expr::literal(v, Dimension::Length).expect("finite length");
    let scl = |v: f64| Expr::literal(v, Dimension::Scalar).expect("finite scalar");
    let plen = |n: &str| Expr::param(ParamName::new(n), Dimension::Length);
    let mut r = Recorder::new();
    let declare = |r: &mut Recorder, n: &str, value: f64, distribution: Distribution| {
        r.push(DocEdit::SetDocParam {
            name: ParamName::new(n),
            value: DocParam::Continuous {
                dim: Dimension::Length,
                value,
                display_unit: UnitSym::canonical_for(Dimension::Length),
                distribution: Some(distribution),
            },
        });
    };
    declare(
        &mut r,
        "outer_r",
        4.0e-3,
        Distribution::Uniform {
            lo: -5.0e-5 * scale,
            hi: 5.0e-5 * scale,
        },
    );
    declare(
        &mut r,
        "offset",
        1.0e-3,
        Distribution::Uniform {
            lo: -5.0e-5 * scale,
            hi: 5.0e-5 * scale,
        },
    );
    declare(
        &mut r,
        "bore_r",
        0.75e-3,
        Distribution::Normal {
            sigma: 1.0e-5 * scale,
        },
    );
    let plane = r.insert(Node::Datum(Datum::Frame {
        origin: [len(0.0), len(0.0), len(0.0)],
        u: [scl(1.0), scl(0.0), scl(0.0)],
        v: [scl(0.0), scl(1.0), scl(0.0)],
    }));
    let thickness = Expr::div(plen("outer_r"), scl(5.0)).expect("Length / Scalar");
    let disc_profile = r.insert(Node::Profile(ProfileProgram {
        plane,
        loops: vec![LoopProgram::Circle {
            centre: [len(0.0), len(0.0)],
            radius: plen("outer_r"),
        }],
    }));
    let disc = r.insert(Node::Extrude {
        profile: disc_profile,
        distance: thickness.clone(),
    });
    let bore_profile = r.insert(Node::Profile(ProfileProgram {
        plane,
        loops: vec![LoopProgram::Circle {
            centre: [plen("offset"), len(0.0)],
            radius: plen("bore_r"),
        }],
    }));
    let bore = r.insert(Node::Extrude {
        profile: bore_profile,
        distance: thickness,
    });
    let refs = {
        let ev: editor_core::Evaluation<f64> = editor_core::evaluate(
            &r.doc,
            None,
            &editor_core::CancelToken::new(),
            &editor_core::EvalOptions::default(),
            tol,
        );
        let env = r.doc.param_env::<f64>();
        let wall = |node: RecipeNodeId| {
            let mut faces = select_where(
                &ev,
                node,
                &Selector::of(NamePat::of_kind(EntityKind::Face)),
                &[GeomPred::SurfaceKind(SurfaceKindSet::just(
                    geom_brep::SurfaceKind::Cylinder,
                ))],
                &env,
                tol,
            )
            .expect("the surface-kind atom is exact");
            faces.sort();
            assert!(!faces.is_empty(), "a circle extrude has a cylindrical wall");
            SitedRef::new(node, faces.remove(0))
        };
        vec![wall(disc), wall(bore)]
    };
    // wall = distance(outer wall, bore wall) = outer_r − offset − bore_r.
    let wall = MeasureExpr::primitive(MeasurePrimitive::Distance { a: 0, b: 1 });
    let measure = r.insert(Node::measure(wall, refs).expect("both indices in range"));
    let assertion = r.insert(Node::Assertion {
        measure,
        bound: len(2.25e-3 - 1.0e-4),
        dir: editor_core::AssertionDir::AtLeast,
    });
    (r.doc, measure, assertion)
}

/// **THE E2E EXERCISE**: R1's annulus driven with the tier on at each
/// rule set, and off — receipts, decisions, the render, the stackup and
/// the assertion, at the real study and two narrower ones.
///
/// EVIDENCE-ONLY.
#[test]
#[ignore = "evidence-only: R1's own end-to-end arc study"]
fn r1_annulus_end_to_end() {
    let tol = Tol::witness();
    for scale in [1.0_f64, 1.0e-3, 1.0e-6] {
        let (doc, measure, assertion) = annulus(scale, tol);
        let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
        let mut sets: Vec<(&str, SymbolicDials)> = vec![("TIER OFF", SymbolicDials::off())];
        for (l, r) in variants() {
            sets.push((l, dials(r)));
        }
        for (name, d) in sets {
            let t = Instant::now();
            let v = drive(
                &doc,
                &analyzed,
                &DriveConfig {
                    symbolic: d,
                    max_leaves: 128,
                    ..DriveConfig::default()
                },
                tol,
            );
            let dt = t.elapsed().as_secs_f64();
            match v {
                Err(e) => println!("== scale {scale:e} {name:<12} {dt:>6.1}s: REFUSED {e}"),
                Ok(v) => {
                    println!(
                        "== scale {scale:e} {name:<12} {dt:>6.1}s: receipt={:?} decisions={:?}",
                        v.receipt(),
                        v.decisions()
                    );
                    if scale == 1.0 || name == "TIER OFF" {
                        println!("{}", v.render(&analyzed));
                    }
                    let stack = editor_core::stackup::stackup(
                        &doc, measure, &analyzed, &v, None, false, tol,
                    );
                    println!("   stackup: {stack:?}");
                    let a = editor_core::drive::assertion_at(&doc, assertion, v.root(), d, tol);
                    println!("   assertion at the root box: {a:?}");
                }
            }
        }
        // The first refusal at the real box, with the blocking shapes.
        let (shapes, refusal, counts) =
            replay(&doc, &ParamBox::of(&analyzed), SymRules::none(), tol);
        println!(
            "   whole-box replay (none): {counts:?}; first refusal {}",
            short(&refusal)
        );
        let mut seen = BTreeMap::new();
        for s in shapes.iter().filter(|s| {
            matches!(
                s.outcome,
                ShapeOutcome::Indeterminate | ShapeOutcome::Invalid | ShapeOutcome::NumericZero
            )
        }) {
            let n: &mut usize = seen.entry(s.predicate).or_default();
            if *n < 1 {
                *n += 1;
                let f = s.form.as_deref().unwrap_or("-");
                println!(
                    "   [{:?}] {}: {}",
                    s.outcome,
                    s.predicate,
                    f.chars().take(300).collect::<String>()
                );
            }
        }
    }
}
