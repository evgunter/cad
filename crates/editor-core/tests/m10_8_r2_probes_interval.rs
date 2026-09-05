//! **R2's end-to-end probes of M10-8** — the arc-family measurement
//! re-derived on a document the unit never built, and two attacks on
//! the measurement instrument itself.
//!
//! The document is R2's own and is NEITHER of the unit's three: a
//! **rounded-corner pad with a central bore**, whose four corner arcs
//! are authored as `Fillet` steps on a chain (so the swept-arc carrier
//! family the unit targets stands between the profile and every side
//! wall, four times over rather than once), with the corner radius and
//! the bore radius both distributed. The study measures the web from
//! the bore wall to a corner wall and asserts a floor on it.
//!
//! Two rows ASSERT; the rest print. The asserting rows are:
//!
//! - `r2_the_shape_report_attributes_by_a_stale_predicate_name`, which
//!   shows that the per-predicate split §1 reports moves between two
//!   replays of the SAME rule set — so the "every rule set is
//!   identical" reading of that table is an artifact of replay ORDER,
//!   not a fact about the rules;
//! - `r2_the_bracket_pin_passes_with_both_sides_certifying`, which
//!   shows the shipped bracket-ceiling pin cannot see the failure it
//!   is written against.
//!
//! ITS PROBE-GATED CODE IS NOT EXECUTED BY CI.

#![cfg(feature = "interval")]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
#![allow(dead_code)]

use editor_core::analysis::{AnalysisPolicy, ParamBox, analyzed_box};
use editor_core::drive::{DriveConfig, SymbolicDials, drive};
use editor_core::{
    Datum, Dimension, Distribution, DocEdit, DocParam, EntityKind, Expr, GeomPred, LoopProgram,
    MeasureExpr, MeasurePrimitive, NamePat, Node, ParamName, ProfileDoc, ProfileProgram,
    ProgramStep, ProgramTarget, RecipeNodeId, Selector, SitedRef, SurfaceKindSet, UnitSym,
    select_where,
};
use geom_core::{SymRules, Tol};

use crate::fixture::Recorder;
use crate::m10_8_arc_family_interval::replay;
use crate::m10_8_harness::nominal_box;

fn len(v: f64) -> Expr {
    Expr::literal(v, Dimension::Length).expect("finite length")
}

fn scl(v: f64) -> Expr {
    Expr::literal(v, Dimension::Scalar).expect("finite scalar")
}

fn plen(n: &str) -> Expr {
    Expr::param(ParamName::new(n), Dimension::Length)
}

/// The pad's half-width and half-height, in metres.
const HALF_W: f64 = 4.0e-3;
const HALF_H: f64 = 2.5e-3;
/// The nominal corner radius — the ARC parameter.
const CORNER: f64 = 0.8e-3;
/// The nominal bore radius.
const BORE: f64 = 1.0e-3;

/// **R2's own arc-bearing document**: a rounded-corner pad (four
/// parametric fillet arcs on one chain) with a central bore, measuring
/// the web from the bore wall to one corner wall.
///
/// `scale` multiplies every tolerance, so `1.0` is the study a user
/// would ask for.
pub(crate) fn pad(scale: f64, tol: Tol) -> (ProfileDoc, RecipeNodeId, RecipeNodeId) {
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
        "corner_r",
        CORNER,
        Distribution::Uniform {
            lo: -2.0e-5 * scale,
            hi: 2.0e-5 * scale,
        },
    );
    declare(
        &mut r,
        "bore_r",
        BORE,
        Distribution::Normal {
            sigma: 1.0e-5 * scale,
        },
    );

    let plane = r.insert(Node::Datum(Datum::Frame {
        origin: [len(0.0), len(0.0), len(0.0)],
        u: [scl(1.0), scl(0.0), scl(0.0)],
        v: [scl(0.0), scl(1.0), scl(0.0)],
    }));

    // The rounded rectangle: four straight legs joined by four
    // PARAMETRIC fillet arcs, all sharing one radius parameter.
    let pt = |x: f64, y: f64| ProgramTarget::Point([len(x), len(y)]);
    let pad_loop = LoopProgram::Chain(vec![
        ProgramStep::At([len(-HALF_W), len(0.0)]),
        ProgramStep::LineTo(pt(-HALF_W, HALF_H - 2.0 * CORNER)),
        ProgramStep::Fillet(plen("corner_r")),
        ProgramStep::Toward {
            dx: scl(1.0),
            dy: scl(0.0),
        },
        ProgramStep::FarEndTo([len(HALF_W - 2.0 * CORNER), len(HALF_H)]),
        ProgramStep::Fillet(plen("corner_r")),
        ProgramStep::Toward {
            dx: scl(0.0),
            dy: scl(-1.0),
        },
        ProgramStep::FarEndTo([len(HALF_W), len(-HALF_H + 2.0 * CORNER)]),
        ProgramStep::Fillet(plen("corner_r")),
        ProgramStep::Toward {
            dx: scl(-1.0),
            dy: scl(0.0),
        },
        ProgramStep::FarEndTo([len(-HALF_W + 2.0 * CORNER), len(-HALF_H)]),
        ProgramStep::Fillet(plen("corner_r")),
        ProgramStep::CloseTo,
    ]);
    let profile = r.insert(Node::Profile(ProfileProgram {
        plane,
        loops: vec![pad_loop],
    }));
    let thickness = len(1.2e-3);
    let body = r.insert(Node::Extrude {
        profile,
        distance: thickness.clone(),
    });

    let bore_profile = r.insert(Node::Profile(ProfileProgram {
        plane,
        loops: vec![LoopProgram::Circle {
            centre: [len(0.0), len(0.0)],
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
            assert!(!faces.is_empty(), "a rounded pad has cylindrical walls");
            SitedRef::new(node, faces.remove(0))
        };
        // Index 0: a CORNER ARC wall of the pad. Index 1: the bore wall.
        vec![wall(body), wall(bore)]
    };

    // web = distance(corner wall, bore wall) − corner_r − bore_r.
    let web = MeasureExpr::sub(
        MeasureExpr::primitive(MeasurePrimitive::Distance { a: 0, b: 1 }),
        MeasureExpr::add(
            MeasureExpr::value(plen("corner_r")),
            MeasureExpr::value(plen("bore_r")),
        )
        .expect("Length + Length"),
    )
    .expect("Length - Length");
    let measure = r.insert(Node::measure(web, refs).expect("both indices in range"));
    let assertion = r.insert(Node::Assertion {
        measure,
        bound: len(0.0),
        dir: editor_core::AssertionDir::AtLeast,
    });
    (r.doc, measure, assertion)
}

// ------------------------------------------------- the asserting rows

/// **The shape report attributes each decision to its own door.** R2's
/// review found `report::record` charging a decision to
/// `k_stats::current_predicate()`, a thread-local the last named
/// `classify` set and nothing reset — so each replay's first decisions
/// were charged to the previous replay's last predicate (`assert_bound`
/// read 1 decision on the first replay and 10 on the second, and the
/// unit's §1 table was cut from that). The name is scoped now
/// (`k_stats::classify` restores it on the way out), and two identical
/// replays attribute identically; decisions taken outside any named
/// door land under the unnamed default, which this row prints.
#[test]
fn r2_the_shape_report_attributes_each_decision_to_its_own_door() {
    let tol = Tol::witness();
    let doc = crate::m10_7_plate::plate(5.0e-5, 1.0e-5, tol).0;
    let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
    let nominal = nominal_box(&analyzed);
    let split = |run: usize| {
        let (shapes, _, _) = replay(&doc, &nominal, SymRules::none(), tol);
        let mut n = 0usize;
        let mut unnamed = 0usize;
        for s in &shapes {
            if s.predicate == "assert_bound" {
                n += 1;
            }
            if s.predicate == "<unnamed>" {
                unnamed += 1;
            }
        }
        println!(
            "   replay {run} under `none`: assert_bound recorded {n} decision(s), {unnamed} outside any named door"
        );
        n
    };
    let first = split(1);
    let second = split(2);
    assert_eq!(
        first, second,
        "the per-predicate attribution is stable across two identical replays"
    );
}

/// **The bracket at `1e2 · ε` of its study certifies under the shipped
/// tier and NOT under M10-7's.** R2's review found the first cut of the
/// bracket pin asserting only that two rule sets AGREE at one scale,
/// both `false` — a `false == false` that would have stayed green had
/// the mechanism moved. It moved: A0 (the constant fold, shipped) lifts
/// the bracket's whole-certifying ceiling from `3.7e1 · ε` to
/// `3.9e2 · ε`, so at `1e2 · ε` (the first cut's `1e-7` at the default
/// epsilon, made ε-relative) the two sides DIFFER at every ε row, and
/// that difference is what this row pins. The ceilings themselves are
/// pinned in `m10_8_pins_interval`.
#[test]
fn r2_the_bracket_between_the_two_ceilings_certifies_under_the_shipped_tier_only() {
    let tol = Tol::witness();
    let doc = crate::m10_7_r2_probes_interval::bracket(1.0e2 * tol.eps(), tol).0;
    let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
    let whole = |rules: SymRules| {
        drive(
            &doc,
            &analyzed,
            &DriveConfig {
                max_depth: 0,
                max_leaves: 1,
                symbolic: SymbolicDials {
                    rules,
                    ..SymbolicDials::default()
                },
                ..DriveConfig::default()
            },
            tol,
        )
        .is_ok_and(|v| v.receipt().certified == 1)
    };
    let (shipped, off) = (whole(SymRules::shipped()), whole(SymRules::none()));
    println!("   bracket at x1e2·eps: shipped {shipped}, M10-7's tier {off}");
    assert!(
        shipped,
        "the shipped tier certifies the bracket whole at 1e2 · eps"
    );
    assert!(
        !off,
        "M10-7's tier does not — the mechanism moved the ceiling"
    );
}

// -------------------------------------------------- the evidence rows

/// **THE E2E EXERCISE**: R2's rounded-corner pad, driven with the tier
/// on under each rule set, with the receipts and the first refusal.
#[test]
#[ignore = "evidence-only: prints R2's own end-to-end arc study"]
fn r2_end_to_end_rounded_pad_study() {
    let tol = Tol::witness();
    for scale in [1.0_f64, 1.0e-3, 1.0e-6] {
        let (doc, _, _) = pad(scale, tol);
        let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
        for (label, rules) in [
            ("none", SymRules::none()),
            (
                "A",
                SymRules {
                    sqrt_square: true,
                    pythagoras: false,
                    ..SymRules::none()
                },
            ),
            ("A+B", SymRules::all()),
        ] {
            let verdict = drive(
                &doc,
                &analyzed,
                &DriveConfig {
                    max_leaves: 64,
                    symbolic: SymbolicDials {
                        rules,
                        ..SymbolicDials::default()
                    },
                    ..DriveConfig::default()
                },
                tol,
            );
            match verdict {
                Ok(v) => println!(
                    "   pad x{scale:e} rules={label:<4}: {:?} decisions {:?}",
                    v.receipt(),
                    v.decisions()
                ),
                Err(e) => println!("   pad x{scale:e} rules={label:<4}: REFUSED {e}"),
            }
        }
        let (_, refusal, counts) = replay(&doc, &ParamBox::of(&analyzed), SymRules::all(), tol);
        println!("   pad x{scale:e} whole-box replay: {counts:?}; first refusal {refusal:?}");
    }
}

/// The pad's per-predicate split under each rule set — R2's own §1
/// table, on a document the unit never measured.
#[test]
#[ignore = "evidence-only: prints the pad's per-predicate table"]
fn r2_the_pad_per_predicate_table() {
    let tol = Tol::witness();
    let (doc, _, _) = pad(1.0, tol);
    let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
    let nominal = nominal_box(&analyzed);
    for (label, rules) in [
        ("none", SymRules::none()),
        ("none(2)", SymRules::none()),
        ("A+B", SymRules::all()),
    ] {
        let (shapes, refusal, counts) = replay(&doc, &nominal, rules, tol);
        println!(
            "   pad rules={label:<8}: {counts:?} refusal {refusal:?} shapes {}",
            shapes.len()
        );
    }
}

/// **The bounded EARLY reduction on the real documents** — R2's own
/// answer to "the runaway is intrinsic". Set `CAD_M10_8_R2_EARLY` to
/// the per-node term cap; `0` (unset) is the shipped behaviour.
#[test]
#[ignore = "evidence-only: needs CAD_M10_8_R2_EARLY; prints what the early pass discharges and what it costs"]
fn r2_the_bounded_early_reduction_on_the_documents() {
    let tol = Tol::witness();
    let cap = std::env::var("CAD_M10_8_R2_EARLY").unwrap_or_else(|_| "0".into());
    let docs: [(&str, ProfileDoc); 3] = [
        ("plate", crate::m10_7_plate::plate(5.0e-5, 1.0e-5, tol).0),
        (
            "r2_bracket",
            crate::m10_7_r2_probes_interval::bracket(1.0, tol).0,
        ),
        ("r2_pad", pad(1.0, tol).0),
    ];
    for (name, doc) in docs {
        let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
        let start = std::time::Instant::now();
        let (_, refusal, counts) = replay(&doc, &ParamBox::of(&analyzed), SymRules::none(), tol);
        println!(
            "   cap={cap} {name}: {counts:?} in {:?}; first refusal {refusal:?}",
            start.elapsed()
        );
    }
}

/// The whole-certifying ceiling of the plate and R2's bracket with the
/// bounded early reduction on, against the same measurement with it
/// off — the number that says whether the mechanism was landable.
#[test]
#[ignore = "evidence-only: needs CAD_M10_8_R2_EARLY; bisects the ceiling with the early pass on"]
fn r2_the_ceiling_with_the_bounded_early_reduction() {
    let tol = Tol::witness();
    let cap = std::env::var("CAD_M10_8_R2_EARLY").unwrap_or_else(|_| "0".into());
    let plate_at = |s: f64| crate::m10_7_plate::plate(5.0e-5 * s, 1.0e-5 * s, tol).0;
    let bracket_at = |s: f64| crate::m10_7_r2_probes_interval::bracket(s, tol).0;
    let pad_at = |s: f64| pad(s, tol).0;
    let docs: [(&str, &dyn Fn(f64) -> ProfileDoc); 3] = [
        ("plate", &plate_at),
        ("r2_bracket", &bracket_at),
        ("r2_pad", &pad_at),
    ];
    for (name, at) in docs {
        let whole = |s: f64| {
            drive(
                &at(s),
                &analyzed_box(&at(s), &AnalysisPolicy::default()),
                &DriveConfig {
                    max_depth: 0,
                    max_leaves: 1,
                    symbolic: SymbolicDials::default(),
                    ..DriveConfig::default()
                },
                tol,
            )
            .is_ok_and(|v| v.receipt().certified == 1)
        };
        let start = std::time::Instant::now();
        let (mut lo, mut hi) = (1.0e-14_f64, 1.0e3_f64);
        if !whole(lo) {
            println!("   cap={cap} {name}: NOTHING certifies (factor 1.0)");
            continue;
        }
        for _ in 0..24 {
            let mid = (0.5 * (lo.ln() + hi.ln())).exp();
            if whole(mid) { lo = mid } else { hi = mid }
        }
        println!(
            "   cap={cap} {name}: ceiling x{lo:e} in {:?}",
            start.elapsed()
        );
    }
}
