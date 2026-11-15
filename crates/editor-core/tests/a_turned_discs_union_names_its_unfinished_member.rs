//! **A union whose member does not finish at its take names that member**
//! (`NodeErrorKind::UnfinishedLeaf`), at a box scalar: a disc extruded
//! and turned in the world by a boxed angle (`theta` = 30° ± 1° about
//! `y`), united with an unturned copy standing clear of it.
//!
//! The union is built in its first member's frame, the turned copy's, so
//! the member carried through the boxed turn is the CLEAR copy. Its
//! cylinder's axis, read through the turn, is the datum left undecided,
//! and the refusal names that copy as its input. That the named input is
//! the copy that was not turned is
//! `work/onehome/an-unfinished-leaf-names-the-copy-that-was-not-turned.md`;
//! this row pins what the evaluation does today.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture::{self, Recorder, len};
use editor_core::analysis::{AnalysisPolicy, ParamBox, analyzed_box};
use editor_core::{
    Bodies, CancelToken, Dimension, DocEdit, EvalOptions, Evaluation, Formula, FreeVar,
    LoopProgram, Node, ProfileDoc, ProfileProgram, RecipeNodeId, TargetAxis, UnitSym, VarName,
    evaluate,
};
use geom_core::{Interval, Tol};

/// The document: the turned copy, the clear copy and their union.
fn turned_disc_union() -> (ProfileDoc, [RecipeNodeId; 3]) {
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
    let turned = r.in_world_by(
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
    let union = r.insert(Node::Union {
        members: Bodies::Spelled(vec![turned.into(), clear.into()]),
    });
    r.assemble_world_copies();
    (r.doc, [turned, clear, union])
}

#[test]
fn a_turned_discs_union_names_the_member_carried_through_the_turn() {
    let (doc, [turned, clear, union]) = turned_disc_union();
    let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
    let opts = EvalOptions {
        param_box: Some(std::sync::Arc::new(ParamBox::of(&analyzed))),
        ..EvalOptions::default()
    };
    let ev: Evaluation<Interval> = evaluate(&doc, None, &CancelToken::new(), &opts, Tol::witness());
    assert!(ev.node_error(turned).is_none(), "the turned copy builds");
    assert!(ev.node_error(clear).is_none(), "the clear copy builds");
    let refusal = ev.node_error(union).expect("the union refuses");
    let editor_core::NodeErrorKind::UnfinishedLeaf { input, findings } = &refusal.kind else {
        panic!(
            "the union refuses its unfinished member: {:?}",
            refusal.kind
        );
    };
    assert_eq!(
        *input, clear,
        "the refusal names the member carried through the boxed turn"
    );
    assert!(
        findings.as_slice().iter().any(|f| matches!(
            f,
            topo::ValidationError::SurfaceDatumUndecided {
                kind: geom::SurfaceKind::Cylinder,
                ..
            }
        )),
        "its finding is the turned cylinder's datum: {findings:?}"
    );
    let text = refusal.kind.to_string();
    assert!(
        text.contains("does not finish") && !text.contains("extru"),
        "the text names the input, not how it was made: {text}"
    );
}
