//! **R2 merge-base content-key differential** (M10-2 review claim 1).
//!
//! This file uses NO M10-2 API, so it compiles at the merge base and
//! at the unit's head alike. It prints the content key of every node
//! of a measure-free document; the reviewer runs it on both checkouts
//! and diffs the printed lines. A key that moved is claim 1 falsified.
//!
//! It asserts only that the document evaluated, so it cannot go red
//! for a reason unrelated to the differential.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;

use editor_core::UnitSym;
use editor_core::{
    BooleanOp, CancelToken, Dimension, DocEdit, DocParam, DocumentId, EvalOptions, Evaluation,
    Expr, Node, NodeResult, ParamName, ProfileDoc, ProfileProgram, RecipeNodeId, apply, evaluate,
};
use fixture::{ang, len, scl};
use geom_core::Tol;

fn push(doc: &editor_core::ProfileDoc, edit: &DocEdit<ProfileProgram>) -> ProfileDoc {
    apply(doc, edit, Tol::witness(), &editor_core::RefusingReach)
        .unwrap_or_else(|e| panic!("edit refused: {e}"))
        .doc
}

fn boxed(
    doc: &ProfileDoc,
    x: (f64, f64),
    y: (f64, f64),
    z0: f64,
    h: f64,
) -> (ProfileDoc, RecipeNodeId) {
    let doc = push(
        doc,
        &DocEdit::InsertNode {
            node: fixture::frame([0.0, 0.0, z0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]),
        },
    );
    let plane = crate::fixture::newest(&doc);
    let doc = push(
        &doc,
        &DocEdit::InsertNode {
            node: Node::Profile(fixture::desc(
                plane,
                vec![vec![(x.0, y.0), (x.1, y.0), (x.1, y.1), (x.0, y.1)]],
            )),
        },
    );
    let p = crate::fixture::newest(&doc);
    let doc = push(
        &doc,
        &DocEdit::InsertNode {
            node: Node::Extrude {
                profile: p,
                distance: len(h),
            },
        },
    );
    let e = crate::fixture::newest(&doc);
    (doc, e)
}

/// A measure-free document exercising the node kinds most likely to
/// move if a key input were added unconditionally: two profiles, two
/// extrudes (one parameter-driven), a boolean and a transform.
#[test]
fn r2_measure_free_content_keys() {
    let d0 = ProfileDoc::empty(DocumentId::derive("r2-keydiff"), Tol::witness());
    let d1 = push(
        &d0,
        &DocEdit::SetDocParam {
            name: ParamName::from_static("t"),
            value: DocParam::Continuous {
                dim: Dimension::Length,
                value: 0.125,
                display_unit: UnitSym::canonical_for(Dimension::Length),
                distribution: None,
            },
        },
    );
    let (d2, a) = boxed(&d1, (0.0, 1.0), (0.0, 2.0), 0.0, 3.0);
    // A parameter under a slot, so the parameter channel is live.
    let d2 = push(
        &d2,
        &DocEdit::InsertNode {
            node: fixture::frame([0.0, 0.0, 1.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]),
        },
    );
    let bplane = crate::fixture::newest(&d2);
    let d3 = push(
        &d2,
        &DocEdit::InsertNode {
            node: Node::Profile(fixture::desc(
                bplane,
                vec![vec![(0.5, 0.5), (1.5, 0.5), (1.5, 2.5), (0.5, 2.5)]],
            )),
        },
    );
    let bp = crate::fixture::newest(&d3);
    let d4 = push(
        &d3,
        &DocEdit::InsertNode {
            node: Node::Extrude {
                profile: bp,
                distance: Expr::param(ParamName::from_static("t"), Dimension::Length),
            },
        },
    );
    let b = crate::fixture::newest(&d4);
    let d5 = push(
        &d4,
        &DocEdit::InsertNode {
            node: Node::Boolean {
                op: BooleanOp::Subtract,
                a,
                b,
                declare: None,
            },
        },
    );
    let cut = crate::fixture::newest(&d5);
    let d6 = push(
        &d5,
        &DocEdit::InsertNode {
            node: Node::transform(
                cut,
                editor_core::Step::Rigid {
                    translation: [len(1.0), len(2.0), len(3.0)],
                    axis: [scl(0.0), scl(0.0), scl(1.0)],
                    angle: ang(0.0),
                },
            ),
        },
    );
    let ev: Evaluation<f64> = evaluate::<f64>(
        &d6,
        None,
        &CancelToken::new(),
        &EvalOptions::default(),
        Tol::witness(),
    );
    let mut ids: Vec<RecipeNodeId> = ev.nodes.keys().copied().collect();
    ids.sort();
    for id in ids {
        match ev.nodes.get(&id) {
            Some(NodeResult::Ok(v)) => println!("R2KEY {} {:032x}", id.0, v.content_key.0),
            other => println!("R2KEY {} FAILED {other:?}", id.0),
        }
    }
    assert!(!ev.nodes.is_empty(), "the document evaluated");
}
