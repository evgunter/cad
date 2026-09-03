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

#[path = "fixture/mod.rs"]
mod fixture;

use editor_core::UnitSym;
use editor_core::{
    BooleanOp, CancelToken, Dimension, DocEdit, DocParam, DocumentId, EvalOptions, Evaluation,
    Expr, Node, NodeResult, ParamName, ProfileDoc, ProfileProgram, RecipeNodeId, apply, evaluate,
};
use fixture::{ang, len, scl};
use geom_core::Tol;

fn push(doc: &ProfileDoc, edit: &DocEdit<ProfileProgram>) -> ProfileDoc {
    apply(doc, edit, Tol::witness())
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
    let p = RecipeNodeId(doc.len() as u64);
    let doc = push(
        doc,
        &DocEdit::InsertNode {
            node: Node::Profile(fixture::desc(
                [0.0, 0.0, z0],
                [1.0, 0.0, 0.0],
                [0.0, 1.0, 0.0],
                vec![vec![(x.0, y.0), (x.1, y.0), (x.1, y.1), (x.0, y.1)]],
            )),
        },
    );
    let e = RecipeNodeId(doc.len() as u64);
    let doc = push(
        &doc,
        &DocEdit::InsertNode {
            node: Node::Extrude {
                profile: p,
                distance: len(h),
            },
        },
    );
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
            name: ParamName::new("t"),
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
    let bp = RecipeNodeId(d2.len() as u64);
    let d3 = push(
        &d2,
        &DocEdit::InsertNode {
            node: Node::Profile(fixture::desc(
                [0.0, 0.0, 1.0],
                [1.0, 0.0, 0.0],
                [0.0, 1.0, 0.0],
                vec![vec![(0.5, 0.5), (1.5, 0.5), (1.5, 2.5), (0.5, 2.5)]],
            )),
        },
    );
    let b = RecipeNodeId(d3.len() as u64);
    let d4 = push(
        &d3,
        &DocEdit::InsertNode {
            node: Node::Extrude {
                profile: bp,
                distance: Expr::param(ParamName::new("t"), Dimension::Length),
            },
        },
    );
    let cut = RecipeNodeId(d4.len() as u64);
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
    let d6 = push(
        &d5,
        &DocEdit::InsertNode {
            node: Node::Transform {
                input: cut,
                translation: [len(1.0), len(2.0), len(3.0)],
                rotation_axis: [scl(0.0), scl(0.0), scl(1.0)],
                rotation_angle: ang(0.0),
            },
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
