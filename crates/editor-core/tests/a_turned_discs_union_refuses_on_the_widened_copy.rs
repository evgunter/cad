//! **A union over a member carried through a boxed turn refuses in the
//! Boolean, on the member's widened data**, at a box scalar: a disc
//! extruded and turned in the world by a boxed angle (`theta` = 30° ± 1°
//! about `y`), united with an unturned copy standing clear of it.
//!
//! The union computes in its first member's frame, the turned copy's, so
//! the clear copy is the one carried through the boxed turn. Each member
//! is finished in its own frame, where the clear copy's cylinder decides
//! exactly, and the n-ary door moves it into the turned frame
//! (`topo::nary_placed`), carrying that verdict: every finding family but
//! the session box is a fact of the shape, which a rigid motion keeps.
//! The image's data is the source's widened by the boxed turn, and the
//! Boolean's ray against the cylinder's disc does not decide on it: the
//! union refuses `ShellWitnessExhausted` at `bool_ray_cylinder_disc`.
//! Before members were placed, the clear copy's image was finished in the
//! turned frame, and its cylinder's datum, read through the turn,
//! refused at the member's take (`UnfinishedLeaf`). The refusal no longer
//! names the member
//! (`work/onehome/a-boolean-refusing-on-a-boxed-members-widened-data-does-not-name-the-member.md`).

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
fn a_turned_discs_union_refuses_on_the_widened_copy_in_the_boolean() {
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
    let editor_core::NodeErrorKind::Boolean(
        error @ topo::BooleanError::ShellWitnessExhausted { .. },
    ) = &refusal.kind
    else {
        panic!(
            "the union refuses in the Boolean, every member taken: {:?}",
            refusal.kind
        );
    };
    let said = format!("{error:?}");
    assert!(
        said.contains("bool_ray_cylinder_disc"),
        "at the ray against the carried copy's cylinder disc: {said}"
    );
}
