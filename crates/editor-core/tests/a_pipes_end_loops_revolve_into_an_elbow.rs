//! **A pipe's end loops revolve into an elbow about an axis beside
//! them, whichever way the axis heads.** The pipe's two circles drawn
//! on its end cap, revolved a quarter turn about a line in the cap's
//! plane 2 clear of the outer circle: the axis through `(±3, 0)`,
//! heading `±y`, each builds.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use editor_core::test_support::{ang, len};
use editor_core::{
    BodyRead, CancelToken, CapEnd, DocEdit, EntityKind, EvalOptions, LoopProgram, Node, OnFace,
    Operand, ProfileDoc, ProfileProgram, RecipeNodeId, RoleSeg, StableName, evaluate,
};
use geom_core::Tol;

fn insert(
    doc: ProfileDoc,
    node: Node<ProfileProgram<editor_core::Formula>, editor_core::Formula>,
) -> (ProfileDoc, RecipeNodeId) {
    let applied = doc
        .apply(
            &DocEdit::InsertNode {
                node: Box::new(node),
                fresh: Vec::new(),
            },
            Tol::witness(),
        )
        .unwrap();
    (applied.doc, applied.record.minted.unwrap())
}

fn circle(r: f64) -> LoopProgram<editor_core::Formula> {
    LoopProgram::Circle {
        centre: [len(0.0), len(0.0)],
        radius: len(r),
    }
}

#[test]
fn a_pipes_end_loops_revolve_into_an_elbow_for_either_heading() {
    let tol = Tol::witness();
    let (opts, cancel) = (EvalOptions::default(), CancelToken::new());
    let doc = ProfileDoc::empty_derived("elbow", tol);
    let (doc, ring) = insert(
        doc,
        Node::Profile(ProfileProgram {
            loops: vec![circle(1.0), circle(0.6)],
            ids: Vec::new(),
            on: None,
        }),
    );
    let (doc, pipe) = insert(
        doc,
        Node::Extrude {
            profile: ring.into(),
            distance: len(2.0),
        },
    );
    let cap = StableName {
        kind: EntityKind::Face,
        node: pipe,
        path: vec![RoleSeg::Cap(CapEnd::End)],
    };
    let drawn = editor_core::draw_on_face(
        &doc,
        BodyRead::plain(Operand::output(pipe, 0)),
        cap,
        vec![circle(1.0), circle(0.6)],
        OnFace::nearest(),
        &opts,
        &cancel,
        tol,
    )
    .unwrap();
    let q = core::f64::consts::FRAC_PI_2;
    let mut doc = drawn.doc;
    let mut elbows = Vec::new();
    for (x, heading) in [(3.0, q), (3.0, -q), (-3.0, q), (-3.0, -q)] {
        let (next, elbow) = insert(
            doc,
            Node::Revolve {
                profile: drawn.profile.into(),
                axis_through: [len(x), len(0.0)],
                axis_heading: ang(heading),
                angle: ang(q),
            },
        );
        doc = next;
        elbows.push((x, heading, elbow));
    }
    let ev = evaluate::<f64>(&doc, None, &cancel, &opts, tol);
    for (x, heading, elbow) in elbows {
        assert!(
            ev.node_error(elbow).is_none() && ev.value(elbow).is_some(),
            "the elbow about the axis through ({x}, 0) heading {heading}: {:?}",
            ev.node_error(elbow).map(|e| e.kind.to_string())
        );
    }
}
