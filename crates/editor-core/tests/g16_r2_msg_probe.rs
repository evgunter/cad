//! REVIEW PROBE (lib-g16-r2): the FILLET refusal Display strings,
//! printed for byte comparison across trees (claim: the error-family
//! rework left every fillet message byte-identical).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod fixture;

use editor_core::{
    CancelToken, EvalOptions, Node, NodeResult, ProfileDoc, ProfileProgram, RecipeNodeId, evaluate,
};
use geom_core::Tol;

fn cube_doc() -> (ProfileDoc, RecipeNodeId) {
    let doc = ProfileDoc::empty_derived("g16-r2-msg", Tol::witness());
    let (doc, profile) = fixture::insert(
        doc,
        Node::Profile(fixture::desc(
            [0.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            vec![vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)]],
        )),
    );
    let (doc, cube) = fixture::insert(
        doc,
        Node::Extrude {
            profile,
            distance: fixture::len(1.0),
        },
    );
    (doc, cube)
}

fn msg_of(doc: &ProfileDoc, node: RecipeNodeId) -> String {
    let ev = evaluate::<f64>(
        doc,
        None,
        &CancelToken::new(),
        &EvalOptions::default(),
        Tol::witness(),
    );
    match ev.nodes.get(&node) {
        Some(NodeResult::Failed(e)) => e.kind.to_string(),
        other => panic!("expected a refusal, got {other:?}"),
    }
}

#[test]
fn g16_r2_fillet_messages() {
    // (a) empty selection
    let (doc, cube) = cube_doc();
    let (doc_a, f) = fixture::insert(doc, Node::fillet(cube, fixture::len(0.1), Vec::new()));
    println!("G16R2MSG empty |{}|", msg_of(&doc_a, f));

    // (b) selection naming a FACE
    let (doc, cube) = cube_doc();
    let face = fixture::ename(cube, fixture::wall(0));
    let (doc_b, f) = fixture::insert(doc, Node::fillet(cube, fixture::len(0.1), vec![face]));
    println!("G16R2MSG kind |{}|", msg_of(&doc_b, f));

    // (c) selection naming an edge of a node that never minted it —
    // wall(7) as an EDGE name resolves Vanished through N5.
    let (doc, cube) = cube_doc();
    let ghost = editor_core::StableName {
        kind: editor_core::EntityKind::Edge,
        node: cube,
        path: vec![fixture::wall(7)],
    };
    let (doc_c, f) = fixture::insert(doc, Node::fillet(cube, fixture::len(0.1), vec![ghost]));
    println!("G16R2MSG resolve |{}|", msg_of(&doc_c, f));

    // (d) the op itself refusing: a radius far too large for the cube.
    let (doc, cube) = cube_doc();
    let (doc_d, f) = fixture::insert(
        doc,
        Node::fillet(cube, fixture::len(0.9), fixture::prism_edges(cube, 4)),
    );
    println!("G16R2MSG op |{}|", msg_of(&doc_d, f));
}
