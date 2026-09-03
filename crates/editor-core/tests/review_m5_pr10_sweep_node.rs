//! M5 PR 10 — the blinded review's Sweep-node reachability probe,
//! ADOPTED and EXTENDED into the PR (review MAJOR-1).
//!
//! The probe asked whether `wire_sweep`'s single-segment path lane
//! could ever run. It cannot: a `Node::Sweep`'s path operand is a
//! profile, a validated profile's loop is CLOSED, and a closed chain
//! has two or more segments — even the minimal two-vertex loop is two
//! half-turn arcs. The fix pass collapsed that lane to one honest
//! frontier arm; these rows pin the collapse.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;

use editor_core::{
    CancelToken, EvalOptions, Expr, Node, NodeErrorKind, NodeResult, ProfileDoc, evaluate,
};
use fixture::{desc, insert};
use geom_core::Tol;

#[test]
fn review_every_sweep_node_hits_the_one_collapsed_frontier_arm() {
    // The most single-segment-like path a validated profile can carry:
    // a two-vertex loop (two arc segments — a circle), and a plain
    // rectangle. Both refuse at the multi-segment arm; sweep_geometry
    // is unreachable from the recipe layer.
    let paths: Vec<(&str, editor_core::ProfileProgram)> = vec![
        (
            "rectangle path",
            desc(
                [0.0, 0.0, 0.0],
                [1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0],
                vec![vec![(0.0, 0.0), (3.0, 0.0), (3.0, 2.0), (0.0, 2.0)]],
            ),
        ),
        ("two-vertex circle path", {
            let mut d = desc(
                [0.0, 0.0, 0.0],
                [1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0],
                vec![vec![(0.0, 0.0), (3.0, 0.0)]],
            );
            // v4: the two half-turn arcs ARE the circle program form's
            // private lowering — a full circle of diameter (0,0)–(3,0).
            d.loops[0] = editor_core::LoopProgram::circle(1.5, 0.0, 1.5).unwrap();
            d
        }),
    ];
    for (name, path_desc) in paths {
        let mut doc = ProfileDoc::empty_derived("review_m5_pr10_sweep_node", Tol::witness());
        let (d, profile) = insert(
            doc,
            Node::Profile(desc(
                [0.0, 0.0, 0.0],
                [1.0, 0.0, 0.0],
                [0.0, 1.0, 0.0],
                vec![vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)]],
            )),
        );
        doc = d;
        let (d, path) = insert(doc, Node::Profile(path_desc));
        doc = d;
        let (doc, sweep) = insert(
            doc,
            Node::Sweep {
                profile,
                path,
                stations: Expr::count(4),
                v_degree: Expr::count(2),
            },
        );
        let out = evaluate::<f64>(
            &doc,
            None,
            &CancelToken::new(),
            &EvalOptions::default(),
            Tol::witness(),
        );
        match out.nodes.get(&sweep).expect("result") {
            NodeResult::Failed(e) => {
                println!("{name}: {:?}", e.kind);
                match &e.kind {
                    NodeErrorKind::CurvedSolidFrontier { what } => {
                        // ONE arm, and it names the missing lane
                        // without pretending a single-segment recipe
                        // path exists.
                        assert!(
                            what.contains("joined-path composition lane"),
                            "{name}: expected the collapsed sweep frontier, got: {what}"
                        );
                        assert!(
                            !what.contains("multi-segment"),
                            "{name}: the collapsed arm must not imply an expressible \
                             single-segment case: {what}"
                        );
                        assert!(
                            what.contains("closed chain of two or more segments"),
                            "{name}: the arm must say WHY no path is expressible: {what}"
                        );
                    }
                    other => panic!("{name}: NON-frontier refusal {other:?}"),
                }
            }
            other => panic!("{name}: sweep unexpectedly produced {other:?}"),
        }
    }
}

/// The RECIPE doors still run BEFORE the frontier: a Sweep whose path
/// is a datum is a recipe error and reads as one, never as a frontier
/// story about geometry it never reached.
#[test]
fn review_recipe_doors_precede_the_sweep_frontier() {
    let mut doc = ProfileDoc::empty_derived("review_m5_pr10_sweep_node", Tol::witness());
    let (d, profile) = insert(
        doc,
        Node::Profile(desc(
            [0.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            vec![vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)]],
        )),
    );
    doc = d;
    let (d, datum) = insert(
        doc,
        Node::Datum(editor_core::Datum::Point {
            position: [
                Expr::literal(0.0, editor_core::Dimension::Length).unwrap(),
                Expr::literal(0.0, editor_core::Dimension::Length).unwrap(),
                Expr::literal(0.0, editor_core::Dimension::Length).unwrap(),
            ],
        }),
    );
    doc = d;
    let (doc, sweep) = insert(
        doc,
        Node::Sweep {
            profile,
            path: datum,
            stations: Expr::count(4),
            v_degree: Expr::count(2),
        },
    );
    let out = evaluate::<f64>(
        &doc,
        None,
        &CancelToken::new(),
        &EvalOptions::default(),
        Tol::witness(),
    );
    match out.nodes.get(&sweep).expect("result") {
        NodeResult::Failed(e) => match &e.kind {
            NodeErrorKind::WrongOperand { input, .. } => assert_eq!(*input, datum),
            other => panic!("a datum path must be WrongOperand, got {other:?}"),
        },
        other => panic!("sweep unexpectedly produced {other:?}"),
    }
}
