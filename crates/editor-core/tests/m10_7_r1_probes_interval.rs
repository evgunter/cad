//! **M10-7 R1 probes at the driver** — independent derivations for
//! claims 1, 5, 6, 8 and the required end-to-end exercise, through the
//! public doors only.
//!
//! Sweep shape ([[test-suite-cost]]): static fixtures, no seed. Rows
//! marked EVIDENCE-ONLY print and gate nothing.
#![cfg(feature = "interval")]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;
use std::sync::Arc;

use editor_core::analysis::{AnalysisPolicy, ParamBox, analyzed_box};
use editor_core::drive::{DriveConfig, RefusalReason, SymbolicDials, assertion_at, drive};
use editor_core::report::MassBudget;
use editor_core::stackup::stackup;
use editor_core::{
    Datum, Dimension, Distribution, DocEdit, DocParam, EntityKind, EvalOptions, Expr, GeomPred,
    LoopProgram, MeasureExpr, MeasurePrimitive, MeasureRef, NamePat, Node, NodeResult, ParamName,
    ProfileDoc, ProfileLift, ProfileProgram, RecipeNodeId, Selector, SurfaceKindSet, UnitSym,
    evaluate, select_where,
};
use fixture::Recorder;
use geom_core::Tol;

use crate::m10_3_driver_interval::{sliver_axis, slab};
use crate::m10_7_plate::plate;

fn len(v: f64) -> Expr {
    Expr::literal(v, Dimension::Length).unwrap()
}
fn scl(v: f64) -> Expr {
    Expr::literal(v, Dimension::Scalar).unwrap()
}
fn param(n: &str) -> Expr {
    Expr::param(ParamName::new(n), Dimension::Length)
}

/// Every `Failed` node of a leaf replay, with its kind — the first is
/// what refuses.
fn failures(doc: &ProfileDoc, box_: ParamBox, dials: SymbolicDials, tol: Tol) -> Vec<String> {
    let opts = EvalOptions {
        param_box: Some(Arc::new(box_)),
        profile_lift: ProfileLift::Guided,
        ..EvalOptions::default()
    };
    let read = |ev: &editor_core::Evaluation<geom_core::Sym<geom_core::Interval>>| {
        ev.order
            .iter()
            .filter_map(|id| match ev.result(*id) {
                Some(NodeResult::Failed(e)) => Some(format!("node {} — {}", id.0, e.kind)),
                _ => None,
            })
            .collect::<Vec<_>>()
    };
    if dials.enabled {
        let (out, counts) = geom_core::sym::with_session(
            geom_core::SymBudget {
                max_terms: dials.max_terms,
                max_degree: dials.max_degree,
            },
            || {
                let ev: editor_core::Evaluation<geom_core::Sym<geom_core::Interval>> =
                    evaluate(doc, None, &editor_core::CancelToken::new(), &opts, tol);
                read(&ev)
            },
        );
        let mut out = out;
        out.push(format!("counts {counts:?}"));
        out
    } else {
        let ev: editor_core::Evaluation<geom_core::Interval> =
            evaluate(doc, None, &editor_core::CancelToken::new(), &opts, tol);
        ev.order
            .iter()
            .filter_map(|id| match ev.result(*id) {
                Some(NodeResult::Failed(e)) => Some(format!("node {} — {}", id.0, e.kind)),
                _ => None,
            })
            .collect()
    }
}

// ------------------------------------------------ claim 6: the slab ceiling

/// EVIDENCE-ONLY. Re-derives the slab's whole-certifying half-width and
/// names what refuses first beyond it — the PR says "the flip at a zero
/// extrusion distance", which lives at `half = 1.0`, while the measured
/// ceiling is `0.488`.
#[test]
#[ignore = "evidence-only: prints the slab ceiling and the first refusal beyond it"]
fn r1_slab_ceiling_and_first_refusal() {
    let tol = Tol::witness();
    let whole = |half: f64| {
        let doc = slab(1.0, half);
        let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
        let v = drive(
            &doc,
            &analyzed,
            &DriveConfig {
                max_depth: 0,
                max_leaves: 1,
                ..DriveConfig::default()
            },
            tol,
        )
        .unwrap();
        (
            v.receipt().certified == 1,
            v.refused().iter().map(|l| format!("{:?}", l.reason)).collect::<Vec<_>>(),
            v.decisions(),
        )
    };
    let (mut lo, mut hi) = (1e-3, 1.0);
    for _ in 0..30 {
        let mid = 0.5 * (lo + hi);
        if whole(mid).0 {
            lo = mid;
        } else {
            hi = mid;
        }
    }
    println!("R1 slab ceiling: half-width {lo:.6} certifies whole, {hi:.6} does not");
    for half in [lo, hi, 0.5, 0.75, 0.999] {
        let (ok, reasons, d) = whole(half);
        println!("  half={half:.6} whole={ok} decisions={d:?} refused={reasons:?}");
        let doc = slab(1.0, half);
        let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
        for f in failures(&doc, ParamBox::of(&analyzed), SymbolicDials::default(), tol) {
            println!("     {f}");
        }
    }
}

// ------------------------------------------------ claim 1: the differential

/// Dumps the serialized verdicts of the M10 fixtures with the tier OFF
/// to `CAD_R1_OUT`; the same dump is produced at the merge base by the
/// twin probe there, and the two files are diffed in the shell.
#[test]
#[ignore = "evidence-only: writes the tier-off dump for the merge-base differential"]
fn r1_tier_off_dump() {
    let tol = Tol::witness();
    let e = tol.eps();
    let off = || DriveConfig {
        symbolic: SymbolicDials::off(),
        ..DriveConfig::default()
    };
    let mut s = String::new();
    let docs: Vec<(&str, ProfileDoc, DriveConfig)> = vec![
        (
            "planted_flip",
            slab(20.0 * e, 40.0 * e),
            DriveConfig {
                max_leaves: 4096,
                ..off()
            },
        ),
        (
            "terminal_sliver",
            sliver_axis(),
            DriveConfig {
                max_leaves: 4096,
                ..off()
            },
        ),
        (
            "macroscopic_slab",
            slab(1.0, 0.05),
            DriveConfig {
                max_leaves: 32,
                ..off()
            },
        ),
        (
            "narrow_slab",
            slab(1.0, e / 4.0),
            DriveConfig {
                max_leaves: 64,
                ..off()
            },
        ),
        (
            "plate_1e-7",
            plate(5.0e-5 * 1e-7, 1.0e-5 * 1e-7, tol).0,
            DriveConfig {
                max_leaves: 64,
                ..off()
            },
        ),
        (
            "plate_1e-3",
            plate(5.0e-5 * 1e-3, 1.0e-5 * 1e-3, tol).0,
            DriveConfig {
                max_leaves: 64,
                ..off()
            },
        ),
    ];
    for (name, doc, cfg) in docs {
        let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
        let v = drive(&doc, &analyzed, &cfg, tol).unwrap();
        s.push_str(&format!("== {name}\n{}", v.serialize()));
        s.push_str(&format!("key {:?}\n", v.content_key()));
        s.push_str(&MassBudget::of(v.accounting(), &analyzed).serialize());
    }
    let path = std::env::var("CAD_R1_OUT").expect("CAD_R1_OUT");
    std::fs::write(path, s).unwrap();
}

// -------------------------------------------------------- claim 5: D9

/// Bit-identity across repeats and the rayon schedule with the tier ON,
/// on a drive with certified leaves, refusals and splits.
#[test]
fn r1_d9_tier_on_repeats_and_schedules() {
    let tol = Tol::witness();
    let e = tol.eps();
    let docs = vec![
        ("planted_flip", slab(20.0 * e, 40.0 * e), 512usize),
        ("plate", plate(5.0e-5 * 1e-6, 1.0e-5 * 1e-6, tol).0, 64),
        ("macro", slab(1.0, 0.75), 64),
    ];
    for (name, doc, leaves) in docs {
        let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
        let run = |parallel: bool| {
            let v = drive(
                &doc,
                &analyzed,
                &DriveConfig {
                    max_leaves: leaves,
                    parallel,
                    ..DriveConfig::default()
                },
                tol,
            )
            .unwrap();
            (v.serialize(), v.content_key(), v.decisions())
        };
        let a = run(false);
        assert!(a.2.symbolic_zero > 0, "{name}: the tier is live: {:?}", a.2);
        for i in 0..3 {
            let b = run(i % 2 == 1);
            assert_eq!(a.0, b.0, "{name}: serialization differs on repeat {i}");
            assert_eq!(a.1, b.1, "{name}: key differs on repeat {i}");
            assert_eq!(a.2, b.2, "{name}: decisions differ on repeat {i}");
        }
        let p = run(true);
        assert_eq!(a.0, p.0, "{name}: parallel schedule differs");
        assert_eq!(a.2, p.2, "{name}: parallel decisions differ");
    }
}

// ------------------------------------------------- D10's third row: budget 0

/// EVIDENCE-ONLY. With the tier ON, does `max_leaves: 0` still starve the
/// drive (an empty certified set) — the plant D10 says no leaf budget
/// can produce?
#[test]
#[ignore = "evidence-only"]
fn r1_max_leaves_zero_with_the_tier_on() {
    let doc = slab(1.0, 0.05);
    let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
    for leaves in [0usize, 1] {
        let v = drive(
            &doc,
            &analyzed,
            &DriveConfig {
                max_leaves: leaves,
                ..DriveConfig::default()
            },
            Tol::witness(),
        );
        match v {
            Ok(v) => println!(
                "max_leaves={leaves}: certified={} refused={} reasons={:?}",
                v.receipt().certified,
                v.receipt().refused,
                v.refused().iter().map(|l| &l.reason).collect::<Vec<_>>()
            ),
            Err(e) => println!("max_leaves={leaves}: refused {e}"),
        }
    }
}

// ------------------------------------------------------ D2: the mate lever

/// The mate lever is the datum's own extent "wherever it has one": a
/// datum a nanometre from the origin has one, and is levered at a
/// nanometre.
#[test]
fn r1_mate_lever_is_discontinuous_at_zero_extent() {
    use editor_core::mate::{Alignment, AxisSense, MateFrame, MatePrimitive};
    let frame = |origin: [f64; 3]| MateFrame {
        origin,
        axis: [0.0, 0.0, 1.0],
        reference: [1.0, 0.0, 0.0],
    };
    let at = |o: f64| Alignment {
        a: frame([o, 0.0, 0.0]),
        b: frame([0.0, 0.0, 0.0]),
        primitive: MatePrimitive::Coaxial,
        sense: AxisSense::Aligned,
        clocking: None,
    };
    println!(
        "lever at origin 0: {}  at 1e-9: {}  at 1e-3: {}",
        at(0.0).lever_arm(),
        at(1e-9).lever_arm(),
        at(1e-3).lever_arm()
    );
    assert_eq!(at(0.0).lever_arm(), 1.0);
    assert_eq!(at(1e-9).lever_arm(), 1e-9);
}

// ------------------------------------------------------------ e2e

/// **R1's own study**: a bracket plate of width `w` (Uniform ±0.5 mm on
/// 20 mm) with two holes whose radius is `w / 16` (a DIVISION of the
/// parameter) at `±w / 4`, extruded by `w / 10`; the web between the
/// holes is measured and asserted. Arcs (the hole rims), a division, a
/// macroscopic box.
pub(crate) fn bracket_pub(
    half_width: f64,
    literal_plate: bool,
    tol: Tol,
) -> (ProfileDoc, RecipeNodeId, RecipeNodeId) {
    bracket_with(half_width, literal_plate, tol)
}

fn bracket(half_width: f64, tol: Tol) -> (ProfileDoc, RecipeNodeId, RecipeNodeId) {
    bracket_with(half_width, false, tol)
}

/// `literal_plate`: the plate's corners as literals (20 mm × 10 mm),
/// leaving only the holes parametric.
fn bracket_with(
    half_width: f64,
    literal_plate: bool,
    tol: Tol,
) -> (ProfileDoc, RecipeNodeId, RecipeNodeId) {
    let mut r = Recorder::new();
    r.push(DocEdit::SetDocParam {
        name: ParamName::new("w"),
        value: DocParam::Continuous {
            dim: Dimension::Length,
            value: 20.0e-3,
            display_unit: UnitSym::canonical_for(Dimension::Length),
            distribution: Some(Distribution::Uniform {
                lo: -half_width,
                hi: half_width,
            }),
        },
    });
    let w = || param("w");
    let div = |a: Expr, k: f64| Expr::div(a, scl(k)).unwrap();
    let plane = r.insert(Node::Datum(Datum::Frame {
        origin: [len(0.0), len(0.0), len(0.0)],
        u: [scl(1.0), scl(0.0), scl(0.0)],
        v: [scl(0.0), scl(1.0), scl(0.0)],
    }));
    let half = |k: f64| div(w(), k);
    // The plate: (−w/2, −w/4) .. (w/2, w/4), parametric corners.
    let plate_loop = if literal_plate {
        LoopProgram::polygon([
            (-10.0e-3, -5.0e-3),
            (10.0e-3, -5.0e-3),
            (10.0e-3, 5.0e-3),
            (-10.0e-3, 5.0e-3),
        ])
        .unwrap()
    } else {
        LoopProgram::Chain(vec![
            editor_core::ProgramStep::At([Expr::neg(half(2.0)), Expr::neg(half(4.0))]),
            editor_core::ProgramStep::LineTo(editor_core::ProgramTarget::Point([
                half(2.0),
                Expr::neg(half(4.0)),
            ])),
            editor_core::ProgramStep::LineTo(editor_core::ProgramTarget::Point([
                half(2.0),
                half(4.0),
            ])),
            editor_core::ProgramStep::LineTo(editor_core::ProgramTarget::Point([
                Expr::neg(half(2.0)),
                half(4.0),
            ])),
            editor_core::ProgramStep::LineTo(editor_core::ProgramTarget::Start),
        ])
    };
    let plate_profile = r.insert(Node::Profile(ProfileProgram {
        plane,
        loops: vec![plate_loop],
    }));
    let _plate = r.insert(Node::Extrude {
        profile: plate_profile,
        distance: div(w(), 10.0),
    });
    let hole = |r: &mut Recorder, cx: Expr| {
        let profile = r.insert(Node::Profile(ProfileProgram {
            plane,
            loops: vec![LoopProgram::Circle {
                centre: [cx, len(0.0)],
                radius: div(w(), 16.0),
            }],
        }));
        r.insert(Node::Extrude {
            profile,
            distance: div(w(), 10.0),
        })
    };
    let hole_a = hole(&mut r, Expr::neg(half(4.0)));
    let hole_b = hole(&mut r, half(4.0));
    let refs = {
        let ev: editor_core::Evaluation<f64> = evaluate(
            &r.doc,
            None,
            &editor_core::CancelToken::new(),
            &EvalOptions::default(),
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
            .unwrap();
            faces.sort();
            MeasureRef::new(node, faces.remove(0))
        };
        vec![wall(hole_a), wall(hole_b)]
    };
    // web = distance(axes) − 2·(w/16) = w/2 − w/8 = 3w/8 = 7.5 mm nominal.
    let web = MeasureExpr::sub(
        MeasureExpr::primitive(MeasurePrimitive::Distance { a: 0, b: 1 }),
        MeasureExpr::value(div(w(), 8.0)),
    )
    .unwrap();
    let measure = r.insert(Node::measure(web, refs).unwrap());
    let assertion = r.insert(Node::Assertion {
        measure,
        bound: len(7.0e-3),
        dir: editor_core::AssertionDir::AtLeast,
    });
    (r.doc, measure, assertion)
}

#[test]
#[ignore = "evidence-only: the R1 end-to-end study"]
fn r1_e2e_bracket_study() {
    let tol = Tol::witness();
    let literal = std::env::var("CAD_R1_LITERAL_PLATE").is_ok();
    for scale in [1.0, 1e-2, 1e-4, 1e-6, 1e-8] {
        let half = 0.5e-3 * scale;
        let (doc, measure, assertion) = bracket_with(half, literal, tol);
        let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
        for dials in [SymbolicDials::default(), SymbolicDials::off()] {
            let t0 = std::time::Instant::now();
            let v = drive(
                &doc,
                &analyzed,
                &DriveConfig {
                    max_leaves: 256,
                    symbolic: dials,
                    ..DriveConfig::default()
                },
                tol,
            )
            .unwrap();
            println!(
                "== bracket(literal_plate={literal}) ±{half:e} tier={} ({:?}) certified={} refused={} splits={} decisions={:?}",
                dials.enabled,
                t0.elapsed(),
                v.receipt().certified,
                v.receipt().refused,
                v.receipt().splits,
                v.decisions()
            );
            let mut classes = std::collections::BTreeMap::new();
            for l in v.refused() {
                *classes
                    .entry(match &l.reason {
                        RefusalReason::Budget(_) => "budget".to_owned(),
                        other => format!("{other:?}").chars().take(60).collect(),
                    })
                    .or_insert(0usize) += 1;
            }
            println!("   refusals: {classes:?}");
            println!("{}", MassBudget::of(v.accounting(), &analyzed).render());
            if dials.enabled {
                let fails = failures(&doc, ParamBox::of(&analyzed), dials, tol);
                for f in fails.iter().take(3) {
                    println!("   whole-box replay: {f}");
                }
            }
            match stackup(&doc, measure, &analyzed, &v, None, true, tol) {
                Ok(report) => println!("{}", report.render(&analyzed)),
                Err(e) => println!("   stackup refused: {e}"),
            }
            let mut holds = (0, 0, 0);
            for leaf in v.certified() {
                match assertion_at(&doc, assertion, &leaf.box_, v.symbolic(), tol)
                    .and_then(|a| a.holds())
                {
                    Some(true) => holds.0 += 1,
                    Some(false) => holds.1 += 1,
                    None => holds.2 += 1,
                }
            }
            println!("   assertion over certified leaves (holds, violated, unevaluated) = {holds:?}");
        }
    }
}

/// The bracket at the nominal builds and measures what its algebra says.
#[test]
fn r1_e2e_bracket_nominal_measures_three_eighths_w() {
    let tol = Tol::witness();
    let (doc, measure, _) = bracket(0.5e-3, tol);
    let ev: editor_core::Evaluation<f64> = evaluate(
        &doc,
        None,
        &editor_core::CancelToken::new(),
        &EvalOptions::default(),
        tol,
    );
    match ev.result(measure) {
        Some(NodeResult::Ok(v)) => match &v.payload {
            editor_core::ValuePayload::Measure { value, .. } => {
                assert!((value - 7.5e-3).abs() < 1e-9, "web {value}");
            }
            other => panic!("{}", other.kind_name()),
        },
        _ => panic!("{:?}", ev.node_error(measure).map(|e| e.kind.to_string())),
    }
}
