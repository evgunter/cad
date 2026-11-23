//! **A whole ball cut by a tilted brick**: the unit ball and the brick
//! of [`emit_sphere_cut_in_cap`](crate::emit_sphere_cut_in_cap), whose
//! near face cuts a cap of height 0.015 off the ball.
//!
//! The ball is one closed sphere group, so the kernel re-charts it
//! (`topo::boolean::ops::apply_recuts`): it turns the ball about its
//! centre until a pole lies on the brick's axis, keeping every key. The
//! result's seam pieces and poles then lie off the ball's own seam and
//! poles, and descend from no cell of the ball, so every op refuses to
//! name them ([`ROW`], whose way through is to cut the sphere instead of
//! turning it).
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::blocks::{failure, run};
use crate::emit_sphere_cut_in_cap::{ball, brick_toward, dir, world_in_brick};
use crate::fixture::insert;
use editor_core::{NamingError, Node, NodeErrorKind, ProfileDoc, RecipeNodeId};
use geom_core::Tol;

const ROW: &str =
    "work/borders/a-whole-ball-cut-by-a-tilted-brick-refuses-an-unclassified-member-edge-side.md";

/// The ball (its copy beside the brick), the brick, and `op` over
/// them, ball first or not.
fn document(op: &str, ball_first: bool) -> (ProfileDoc, [RecipeNodeId; 3]) {
    let doc = ProfileDoc::empty_derived("ball_cut_by_brick", Tol::witness());
    let (doc, b) = ball(doc, 1.0, 0.0);
    let (doc, brick, stands) = brick_toward(doc, dir(68.0, 50.0), 0.985, 0.2, 0.3);
    let (doc, b) = world_in_brick(doc, brick, &stands, editor_core::Operand::output(b, 0));
    let ball_body = editor_core::Operand::from(b);
    let brick_body = editor_core::Operand::from(brick);
    let (x, y) = if ball_first {
        (ball_body, brick_body)
    } else {
        (brick_body, ball_body)
    };
    let node = match op {
        "subtract" => Node::Subtract {
            from: x.into(),
            tool: y.into(),
        },
        "union" => Node::Union {
            members: editor_core::Bodies::Spelled(vec![x.into(), y.into()]),
        },
        _ => Node::Intersect {
            members: editor_core::Bodies::Spelled(vec![x.into(), y.into()]),
        },
    };
    let (doc, out) = insert(doc, node);
    (doc, [b, brick, out])
}

/// **Every op over the ball and the brick, in both orders, refuses at
/// a piece of the turned ball's seam**: no name form is true of a cell
/// the re-chart turned, so none is published under the ball's names.
/// Every result keeps a turned seam piece, and edges are read before
/// vertices, so each refuses on an edge.
#[test]
fn a_ball_cut_by_a_brick_refuses_to_name_its_turned_seam_or_pole() {
    for (op, ball_first) in [
        ("intersect", true),
        ("intersect", false),
        ("union", true),
        ("union", false),
        ("subtract", true),
        ("subtract", false),
    ] {
        let what = format!("{op}, ball first: {ball_first}");
        let (doc, [b, brick, out]) = document(op, ball_first);
        let ev = run(&doc);
        assert!(failure(&ev, b).is_none(), "{what}: the ball names");
        assert!(failure(&ev, brick).is_none(), "{what}: the brick names");
        let cell = match failure(&ev, out) {
            Some(NodeErrorKind::Naming(NamingError::UnsupportedTurnedCell { cell, .. })) => *cell,
            other => {
                panic!("{what}: refused other than at a turned seam or pole ({ROW}): {other:?}")
            }
        };
        assert!(
            matches!(cell, topo::Cell::Edge(_)),
            "{what}: refused on {cell:?}, not on a piece of the turned seam, which the pass \
             reads before the turned pole"
        );
    }
}
