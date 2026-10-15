//! probe
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;
use editor_core::ExtrudeSide;
use editor_core::{
    CancelToken, EvalOptions, Evaluation, LoopProgram, Node, ProfileDoc, ProfileProgram,
    RecipeNodeId, evaluate,
};
use geom_core::Tol;

use fixture::{insert, len};

fn eval(doc: &ProfileDoc) -> Evaluation<f64> {
    evaluate::<f64>(
        doc,
        None,
        &CancelToken::new(),
        &EvalOptions::default(),
        Tol::witness(),
    )
}

fn cylinder(doc: ProfileDoc, z0: f64, r: f64, h: f64, az: f64) -> (ProfileDoc, RecipeNodeId) {
    let (c, s) = (az.cos(), az.sin());
    let (doc, plane) = insert(doc, fixture::frame([0.0, 0.0, z0], [c, s, 0.0], [-s, c, 0.0]));
    let (doc, profile) = insert(
        doc,
        Node::Profile(ProfileProgram {
            frame: plane.into(),
            loops: vec![LoopProgram::Circle {
                centre: [len(0.0), len(0.0)],
                radius: len(r),
            }],
            ids: Vec::new(),
        }),
    );
    insert(
        doc,
        Node::Extrude {
            profile: profile.into(),
            distance: len(h),
            side: ExtrudeSide::Along,
        },
    )
}

#[test]
fn probe() {
    for (lo_r, hi_r, az) in [(1.0, 1.0, 0.0), (1.0, 1.0, 2.0), (1.0, 0.5, 0.0), (0.5, 1.0, 0.0)] {
        let doc = ProfileDoc::empty_derived("probe", Tol::witness());
        let (doc, lo) = cylinder(doc, 0.0, lo_r, 1.0, 0.0);
        let (doc, hi) = cylinder(doc, 1.0, hi_r, 1.0, az);
        let nodes = [
            ("lo ∪ hi", Node::Union { members: editor_core::Bodies::Spelled(vec![lo.into(), hi.into()]) }),
            ("hi ∪ lo", Node::Union { members: editor_core::Bodies::Spelled(vec![hi.into(), lo.into()]) }),
            ("lo ∖ hi", Node::Subtract { from: lo.into(), tool: hi.into() }),
            ("hi ∖ lo", Node::Subtract { from: hi.into(), tool: lo.into() }),
        ];
        for (what, n) in nodes {
            let (d, id) = insert(doc.clone(), n);
            let ev = eval(&d);
            eprintln!("PROBE {lo_r} {hi_r} {az} {what}: {:?}", ev.nodes.get(&id).map(|r| format!("{r:?}").chars().take(400).collect::<String>()));
        }
    }
}
