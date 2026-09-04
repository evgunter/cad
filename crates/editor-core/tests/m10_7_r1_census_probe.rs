//! **M10-7 R1: the census, attacked with a profile the PR never built.**
//! Two COLLINEAR consecutive walls — an edge split by a vertex whose
//! coordinates are the same expression of the parameter — so
//! `side_planes_cosurface` is a true identity, and the tier should
//! discharge it. Evidence-only rows print; the one gate asserts the
//! drive certifies the macroscopic box.
#![cfg(all(feature = "interval", feature = "probe"))]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;
use std::collections::BTreeMap;
use std::sync::Arc;

use editor_core::analysis::{AnalysisPolicy, BoxAxis, ParamBox, analyzed_box};
use editor_core::drive::{DriveConfig, SymbolicDials, drive};
use editor_core::{
    CancelToken, Datum, Dimension, Distribution, DocEdit, DocParam, EvalOptions, Expr,
    LoopProgram, Node, ParamName, ProfileDoc, ProfileLift, ProfileProgram, ProgramStep,
    ProgramTarget, UnitSym, evaluate,
};
use fixture::Recorder;
use geom_core::k_stats::{SampleOutcome, start_recording, take_samples};
use geom_core::Tol;

fn len(v: f64) -> Expr {
    Expr::literal(v, Dimension::Length).unwrap()
}
fn scl(v: f64) -> Expr {
    Expr::literal(v, Dimension::Scalar).unwrap()
}

/// A rectangle `[0, w] × [0, 1]` whose bottom edge is split at `w/2`,
/// extruded by 1: the two bottom walls WOULD be cosurface for every
/// `w` — but the profile door refuses a collinear `LineTo` junction as
/// `JunctionTangent` ("spell it line(len) off the directed point — no
/// junction exists there"), so the identity is unauthorable through
/// the chain vocabulary. `Err` carries the door's refusal.
fn split_rectangle(half: f64) -> Result<ProfileDoc, String> {
    let mut r = Recorder::new();
    r.push(DocEdit::SetDocParam {
        name: ParamName::new("w"),
        value: DocParam::Continuous {
            dim: Dimension::Length,
            value: 2.0,
            display_unit: UnitSym::canonical_for(Dimension::Length),
            distribution: Some(Distribution::Uniform {
                lo: -half,
                hi: half,
            }),
        },
    });
    let w = || Expr::param(ParamName::new("w"), Dimension::Length);
    let plane = r.insert(Node::Datum(Datum::Frame {
        origin: [len(0.0), len(0.0), len(0.0)],
        u: [scl(1.0), scl(0.0), scl(0.0)],
        v: [scl(0.0), scl(1.0), scl(0.0)],
    }));
    let pt = |x: Expr, y: Expr| ProgramStep::LineTo(ProgramTarget::Point([x, y]));
    let profile = Node::Profile(ProfileProgram {
        plane,
        loops: vec![LoopProgram::Chain(vec![
            ProgramStep::At([len(0.0), len(0.0)]),
            pt(Expr::div(w(), scl(2.0)).unwrap(), len(0.0)),
            pt(w(), len(0.0)),
            pt(w(), len(1.0)),
            pt(len(0.0), len(1.0)),
            ProgramStep::LineTo(ProgramTarget::Start),
        ])],
    });
    let applied = editor_core::apply(
        &r.doc,
        &DocEdit::InsertNode { node: profile },
        Tol::witness(),
    )
    .map_err(|e| format!("{e:?}"))?;
    let profile = applied.record.minted.unwrap();
    let mut r = Recorder::new();
    r.doc = applied.doc;
    r.insert(Node::Extrude {
        profile,
        distance: len(1.0),
    });
    Ok(r.doc)
}

fn split_at_point(doc: &ProfileDoc, tol: Tol) -> BTreeMap<&'static str, (u64, u64)> {
    let analyzed = analyzed_box(doc, &AnalysisPolicy::default());
    let nominal = ParamBox::from_axes(
        ParamBox::of(&analyzed)
            .axes()
            .keys()
            .map(|n| (n.clone(), BoxAxis::Fixed))
            .collect(),
    );
    let opts = EvalOptions {
        param_box: Some(Arc::new(nominal)),
        profile_lift: ProfileLift::Guided,
        ..EvalOptions::default()
    };
    start_recording();
    let (_, counts) = geom_core::sym::with_session(
        geom_core::SymBudget {
            max_terms: 4096,
            max_degree: 128,
        },
        || {
            let ev: editor_core::Evaluation<geom_core::Sym<geom_core::Probe>> =
                evaluate(doc, None, &CancelToken::new(), &opts, tol);
            ev
        },
    );
    println!("   session counts {counts:?}");
    let mut out: BTreeMap<&'static str, (u64, u64)> = BTreeMap::new();
    for s in take_samples() {
        let row = out.entry(s.predicate).or_default();
        match s.outcome {
            SampleOutcome::SymbolicZero => row.0 += 1,
            _ => row.1 += 1,
        }
    }
    out
}

/// **The census's `side_planes_cosurface` 0/8 claim, attacked and
/// upheld**: two collinear consecutive walls are the one profile on
/// which the predicate is a true identity, and the chain door refuses
/// to author them (`JunctionTangent`). If the door ever admits them,
/// this row reds and the tier's discharge of the predicate becomes
/// measurable.
#[test]
fn r1_collinear_walls_are_unauthorable_so_cosurface_stays_a_non_identity() {
    let tol = Tol::witness();
    match split_rectangle(0.5) {
        Err(e) => {
            println!(
                "the chain door refused the collinear junction: {}",
                &e[..e.len().min(300)]
            );
            assert!(e.contains("JunctionTangent"), "{e}");
        }
        Ok(doc) => {
            let rows = split_at_point(&doc, tol);
            for (pred, (s, n)) in &rows {
                println!("   {pred:<40} symbolic={s:<6} numeric={n}");
            }
            let cos = rows.get("side_planes_cosurface").copied().unwrap_or((0, 0));
            let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
            let v = drive(
                &doc,
                &analyzed,
                &DriveConfig {
                    max_leaves: 64,
                    symbolic: SymbolicDials::default(),
                    ..DriveConfig::default()
                },
                tol,
            )
            .unwrap();
            panic!(
                "the door now admits collinear junctions: side_planes_cosurface {cos:?}, \
                 drive certified={} refused={} — re-measure the census",
                v.receipt().certified,
                v.receipt().refused
            );
        }
    }
}

/// EVIDENCE-ONLY: does a chain with PARAMETRIC `LineTo` points lift at
/// `Interval` over a DEGENERATE box (every axis fixed)? If not, the
/// refusal is not a width effect at all.
#[test]
#[ignore = "evidence-only: the parametric chain at a point box"]
fn r1_parametric_chain_at_a_point_box() {
    let tol = Tol::witness();
    let (doc, _, _) = crate::m10_7_r1_probes_interval::bracket_pub(0.5e-3, false, tol);
    let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
    for fixed in [true, false] {
        let box_ = if fixed {
            ParamBox::from_axes(
                ParamBox::of(&analyzed)
                    .axes()
                    .keys()
                    .map(|n| (n.clone(), BoxAxis::Fixed))
                    .collect(),
            )
        } else {
            ParamBox::of(&analyzed)
        };
        for lift in [ProfileLift::Guided, ProfileLift::default()] {
            let opts = EvalOptions {
                param_box: Some(Arc::new(box_.clone())),
                profile_lift: lift,
                ..EvalOptions::default()
            };
            let ev: editor_core::Evaluation<geom_core::Interval> =
                evaluate(&doc, None, &CancelToken::new(), &opts, tol);
            let errs: Vec<String> = ev
                .order
                .iter()
                .filter_map(|id| {
                    ev.node_error(*id).map(|e| {
                        let s = format!("{:?}", e.kind);
                        format!("node {} {}", id.0, &s[..s.len().min(300)])
                    })
                })
                .collect();
            println!(
                "fixed={fixed} lift={lift:?}: {} errors: {:?}",
                errs.len(),
                errs.first()
            );
        }
        let ev: editor_core::Evaluation<f64> = evaluate(
            &doc,
            None,
            &CancelToken::new(),
            &EvalOptions {
                param_box: Some(Arc::new(box_.clone())),
                profile_lift: ProfileLift::Guided,
                ..EvalOptions::default()
            },
            tol,
        );
        let n = ev.order.iter().filter(|id| ev.node_error(**id).is_some()).count();
        println!("fixed={fixed} at f64 guided: {n} errors");
    }
}

/// EVIDENCE-ONLY: the per-predicate split on R1's bracket at the
/// nominal, and the FULL structure refusal its parametric rectangle
/// meets over a box — printed as Debug so the predicate is visible.
#[test]
#[ignore = "evidence-only: the bracket's census and its lift refusal"]
fn r1_bracket_census_and_lift_refusal() {
    let tol = Tol::witness();
    let (doc, _, _) = crate::m10_7_r1_probes_interval::bracket_pub(0.5e-3, false, tol);
    let rows = split_at_point(&doc, tol);
    println!("== R1 bracket at the nominal, Sym<Probe>:");
    for (pred, (s, n)) in &rows {
        println!("   {pred:<40} symbolic={s:<6} numeric={n}");
    }
    for (scale, literal) in [(1.0, false), (1e-2, false), (1e-4, false), (1.0, true), (1e-2, true), (1e-4, true)] {
        let (doc, _, _) = crate::m10_7_r1_probes_interval::bracket_pub(0.5e-3 * scale, literal, tol);
        let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
        let opts = EvalOptions {
            param_box: Some(Arc::new(ParamBox::of(&analyzed))),
            profile_lift: ProfileLift::Guided,
            ..EvalOptions::default()
        };
        let t0 = std::time::Instant::now();
        let (ev, counts) = geom_core::sym::with_session(
            geom_core::SymBudget {
                max_terms: 4096,
                max_degree: 128,
            },
            || {
                let ev: editor_core::Evaluation<geom_core::Sym<geom_core::Interval>> =
                    evaluate(&doc, None, &CancelToken::new(), &opts, tol);
                ev
            },
        );
        let t_on = t0.elapsed();
        let t0 = std::time::Instant::now();
        let _ev2: editor_core::Evaluation<geom_core::Interval> =
            evaluate(&doc, None, &CancelToken::new(), &opts, tol);
        let t_off = t0.elapsed();
        println!(
            "== bracket scale {scale:e} literal_plate={literal}: one leaf replay tier on {t_on:?} / off {t_off:?}; {counts:?}"
        );
        let t0 = std::time::Instant::now();
        let (_, huge) = geom_core::sym::with_session(
            geom_core::SymBudget {
                max_terms: 1_000_000,
                max_degree: 100_000,
            },
            || {
                let ev: editor_core::Evaluation<geom_core::Sym<geom_core::Interval>> =
                    evaluate(&doc, None, &CancelToken::new(), &opts, tol);
                ev
            },
        );
        println!("   with an unbounded budget: {:?} {huge:?}", t0.elapsed());
        for id in &ev.order {
            if let Some(e) = ev.node_error(*id) {
                let s = format!("{:?}", e.kind);
                println!("   node {} : {}", id.0, &s[..s.len().min(700)]);
            }
        }
    }
}
