//! Reviewer row (PR 6959): a candidate fixture for the coverage the
//! stated-ends unit's re-pins dropped — an `Interval` leaf that is built
//! but does not finish at its taker. A disc extruded and turned in the
//! world by a boxed angle (`theta` = 30° ± 1° about `y`), then united
//! with an unturned copy standing clear of it. Ported from the M3 probe
//! row `m3_turned_disc` (`boxfinish/probe-switch-off`).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture::{self, Recorder, len};
use editor_core::analysis::{AnalysisPolicy, ParamBox, analyzed_box};
use editor_core::{
    Bodies, CancelToken, CheckEvidence, ChecksConfig, Dimension, DocEdit, EvalOptions, Evaluation,
    Formula, FreeVar, LoopProgram, Node, ProfileDoc, ProfileProgram, RecipeNodeId, TargetAxis,
    UnitSym, VarName, evaluate, run_checks,
};
use geom_core::{Interval, Tol};

fn turned_disc_union(with_union: bool) -> (ProfileDoc, [RecipeNodeId; 3]) {
    let (theta, delta, radius) = (30f64.to_radians(), 1f64.to_radians(), 5e-3);
    let mut r = Recorder::new();
    r.push(DocEdit::DeclareVar {
        name: VarName::from_static("theta"),
        def: editor_core::VarDecl::Free(FreeVar::Continuous {
            dim: Dimension::Angle,
            value: theta,
            display_unit: UnitSym::canonical_for(Dimension::Angle),
            distribution: Some(editor_core::Distribution::Uniform {
                lo: -delta,
                hi: delta,
            }),
        }),
    });
    let p = r.insert(Node::Profile(ProfileProgram {
        loops: vec![LoopProgram::Circle {
            centre: [len(0.0), len(0.0)],
            radius: len(radius),
        }],
        ids: Vec::new(),
        on: None,
    }));
    let disc = r.insert(Node::Extrude {
        profile: p.into(),
        distance: len(radius),
    });
    let copy = r.in_world_by(
        disc,
        fixture::turned_to(
            [0.0, 0.0, 0.0],
            &[(
                TargetAxis::Y,
                Formula::named(VarName::from_static("theta"), Dimension::Angle),
            )],
        ),
    );
    let clear = r.in_world_by(disc, fixture::turned_to([20.0 * radius, 0.0, 0.0], &[]));
    let u = if with_union {
        r.insert(Node::Union {
            members: Bodies::Spelled(vec![copy.into(), clear.into()]),
        })
    } else {
        clear
    };
    r.assemble_world_copies();
    (r.doc, [copy, clear, u])
}

#[test]
fn review_a_turned_discs_union_names_the_copy_that_does_not_finish() {
    let (doc, [copy, clear, union]) = turned_disc_union(true);
    let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
    let opts = EvalOptions {
        param_box: Some(std::sync::Arc::new(ParamBox::of(&analyzed))),
        ..EvalOptions::default()
    };
    let ev: Evaluation<Interval> = evaluate(&doc, None, &CancelToken::new(), &opts, Tol::witness());
    eprintln!("[review] copy: {:?}", ev.node_error(copy));
    eprintln!("[review] clear: {:?}", ev.node_error(clear));
    let refusal = ev.node_error(union);
    eprintln!("[review] union: {refusal:?}");
    let cfg = ChecksConfig {
        at_rest: editor_core::Severity::Off,
        chart_coherence: editor_core::Advisory::Off,
        unproven_coincidence: editor_core::Advisory::Off,
        unused: editor_core::Advisory::Off,
        ..ChecksConfig::default()
    };
    let report = run_checks(
        &doc,
        &ev,
        &cfg,
        &editor_core::Stop::default(),
        Tol::witness(),
    );
    let not_at_rest: Vec<_> = report
        .iter()
        .flat_map(|r| r.findings.iter())
        .filter(|f| matches!(f.evidence, CheckEvidence::NotAtRest { .. }))
        .map(|f| format!("{:?}", f.subject))
        .collect();
    eprintln!("[review] NotAtRest findings: {not_at_rest:?}");
    let refusal = refusal.expect("the union refuses");
    let editor_core::NodeErrorKind::UnfinishedLeaf { input, findings } = &refusal.kind else {
        panic!(
            "the union refuses its unfinished member: {:?}",
            refusal.kind
        );
    };
    // The union is built in its first member's frame (the turned copy's),
    // so the member that meets the boxed turn is the CLEAR copy: its
    // cylinder's axis, carried through the turn, is the datum left
    // undecided. The refusal names that member.
    assert_eq!(
        *input, clear,
        "the refusal names the member carried through the boxed turn"
    );
    let text = format!("{findings:?}");
    assert!(
        text.contains("SurfaceDatumUndecided") && text.contains("Cylinder"),
        "its finding is the turned cylinder's datum: {text}"
    );
}
