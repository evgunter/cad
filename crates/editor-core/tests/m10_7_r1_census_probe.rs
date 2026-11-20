//! **M10-7 R1: the census, attacked with a profile the PR never built.**
//! Two COLLINEAR consecutive walls — an edge split by a vertex whose
//! coordinates are the same expression of the parameter — so
//! `side_planes_cosurface` would be a true identity for the tier to
//! discharge. Evidence-only rows print; the one gate asserts that the
//! `LineTo` spelling of such walls cannot be built.
//!
//! Every test here is behind the whole-file `#![cfg(feature = "probe")]`,
//! so the per-PR gate does not run them; the nightly's all-features job
//! does, and two of the three are `#[ignore]`d besides. They are a
//! reviewer's counter-fixture: a profile the unit never built, run to
//! check what the census claims about it. The claim they check is
//! recorded in `work/tier/symbolic-tier-census.md`; the rows are how it
//! was reached, kept so the next lane can re-run them rather than
//! re-derive them.
#![cfg(feature = "probe")]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;
use std::collections::BTreeMap;
use std::sync::Arc;

use editor_core::analysis::{AnalysisPolicy, ParamBox, analyzed_box};
use editor_core::{
    CancelToken, Dimension, Distribution, DocEdit, EvalOptions, Formula, FreeVar, LoopProgram,
    Node, ProfileDoc, ProfileProgram, ProgramStep, ProgramTarget, RecipeNodeId, UnitSym, VarName,
    evaluate,
};
use fixture::{Recorder, len, scl};
use geom_core::Tol;
use geom_core::k_stats::{SampleOutcome, start_recording, take_samples};

/// A rectangle `[0, w] × [0, 1]` whose bottom edge is split at `w/2`,
/// extruded by 1: the two bottom walls would be cosurface for every
/// `w`. The edit door reads no value, so it admits the document; the
/// collinear `LineTo` junction is refused when the profile evaluates.
/// Returns the document and the profile's id.
fn split_rectangle(half: f64) -> (ProfileDoc, RecipeNodeId) {
    let mut r = Recorder::new();
    r.push(DocEdit::DeclareVar {
        name: VarName::from_static("w"),
        def: editor_core::VarDecl::Free(FreeVar::Continuous {
            dim: Dimension::Length,
            value: 2.0,
            display_unit: UnitSym::canonical_for(Dimension::Length),
            distribution: Some(Distribution::Uniform {
                lo: -half,
                hi: half,
            }),
        }),
    });
    let w = || Formula::named(VarName::from_static("w"), Dimension::Length);
    let pt = |x: Formula, y: Formula| ProgramStep::LineTo(ProgramTarget::Point([x, y]));
    let profile = Node::Profile(ProfileProgram {
        loops: vec![LoopProgram::Chain(vec![
            ProgramStep::At([len(0.0), len(0.0)]),
            pt(Formula::div(w(), scl(2.0)).unwrap(), len(0.0)),
            pt(w(), len(0.0)),
            pt(w(), len(1.0)),
            pt(len(0.0), len(1.0)),
            ProgramStep::LineTo(ProgramTarget::Start),
        ])],
        ids: Vec::new(),
        on: None,
    });
    let applied = editor_core::apply(
        &r.doc,
        &DocEdit::InsertNode {
            node: Box::new(profile),
            fresh: Vec::new(),
        },
        Tol::witness(),
    )
    .expect("the door reads no value");
    let profile = applied.record.minted.expect("the insert mints the profile");
    let mut r = Recorder::new();
    r.doc = applied.doc;
    r.insert(Node::Extrude {
        profile: profile.into(),
        distance: len(1.0),
    });
    (r.doc, profile)
}

fn split_at_point(doc: &editor_core::ProfileDoc, tol: Tol) -> BTreeMap<&'static str, (u64, u64)> {
    let analyzed = analyzed_box(doc, &AnalysisPolicy::default());
    let nominal = ParamBox::of(&analyzed).at_nominal();
    let opts = EvalOptions {
        param_box: Some(Arc::new(nominal)),
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
            SampleOutcome::Symbolic => row.0 += 1,
            _ => row.1 += 1,
        }
    }
    out
}

/// Two collinear consecutive walls are the one profile on which
/// `side_planes_cosurface` is a true identity. Spelled with `LineTo`,
/// the junction refuses as `JunctionTangent` at evaluation; the declared
/// spelling, `ContinueTo`, builds them, and
/// `m10_7_r2_probes_interval::r2_collinear_walls_should_discharge_side_planes_cosurface`
/// measures the tier there. If the `LineTo` spelling ever builds, this
/// row reds.
#[test]
fn r1_collinear_walls_are_unauthorable_so_cosurface_stays_a_non_identity() {
    let (doc, id) = split_rectangle(0.5);
    let ev = evaluate::<f64>(
        &doc,
        None,
        &CancelToken::new(),
        &EvalOptions::default(),
        Tol::witness(),
    );
    match ev.node_error(id).map(|e| &e.kind) {
        Some(editor_core::NodeErrorKind::ProfileReplay {
            loop_: 0,
            error:
                profile::ReplayError {
                    step: 2,
                    kind: profile::ReplayErrorKind::Path(e),
                },
        }) if e.kind() == profile::PathErrorKind::JunctionTangent => {}
        other => panic!(
            "the collinear junction refuses as tangent at step 2; got {other:?}: \
             if the walls now build, re-measure the census"
        ),
    }
}

/// EVIDENCE-ONLY: does a chain with PARAMETRIC `LineTo` points lift at
/// `Interval` over a DEGENERATE box (every axis spanning `[0, 0]`, the
/// nominal as an analysis — `ParamBox::at_nominal`)? If not, the
/// refusal is not a width effect at all.
#[test]
#[ignore = "evidence-only: the parametric chain at a point box"]
fn r1_parametric_chain_at_a_point_box() {
    let tol = Tol::witness();
    let (doc, _, _) = crate::m10_7_r1_probes_interval::bracket_pub(0.5e-3, false, tol);
    let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
    for point in [true, false] {
        let box_ = if point {
            ParamBox::of(&analyzed).at_nominal()
        } else {
            ParamBox::of(&analyzed)
        };
        {
            let opts = EvalOptions {
                param_box: Some(Arc::new(box_.clone())),
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
            println!("point={point}: {} errors: {:?}", errs.len(), errs.first());
        }
        let ev: editor_core::Evaluation<f64> = evaluate(
            &doc,
            None,
            &CancelToken::new(),
            &EvalOptions {
                param_box: Some(Arc::new(box_.clone())),
                ..EvalOptions::default()
            },
            tol,
        );
        let n = ev
            .order
            .iter()
            .filter(|id| ev.node_error(**id).is_some())
            .count();
        println!("point={point} at f64: {n} errors");
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
    for (scale, literal) in [
        (1.0, false),
        (1e-2, false),
        (1e-4, false),
        (1.0, true),
        (1e-2, true),
        (1e-4, true),
    ] {
        let (doc, _, _) =
            crate::m10_7_r1_probes_interval::bracket_pub(0.5e-3 * scale, literal, tol);
        let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
        let opts = EvalOptions {
            param_box: Some(Arc::new(ParamBox::of(&analyzed))),
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
