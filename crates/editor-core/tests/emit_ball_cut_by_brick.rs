//! **A whole ball cut by a tilted brick**: the unit ball and the brick
//! of [`emit_sphere_cut_in_cap`](crate::emit_sphere_cut_in_cap), whose
//! near face cuts a cap of height 0.015 off the ball.
//!
//! The brick's near face meets the ball's sphere in a circle no edge
//! reaches, so the kernel cuts the ball's face along its own chart's
//! meridian through the circle before the op runs
//! (`topo::boolean::ops::apply_cut_ins`), and the ball's seam and poles
//! stay where the ball has them. The output stage closes the cut again,
//! and the circle keeps one conventional vertex, named for the circle, as
//! the lens of `emit_sphere_cut_in_cap` does.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::blocks::{failure, run};
use crate::emit_sphere_cut_in_cap::{ball, brick_toward, circle_vertex_cites, dir, world_in_brick};
use crate::fixture::insert;
use editor_core::{Node, ProfileDoc, RecipeNodeId};
use geom_core::Tol;

/// The ball (its copy beside the brick), the brick, `op` over them,
/// ball first or not, and the revolve that mints the ball's names.
pub(crate) fn document(op: &str, ball_first: bool) -> (ProfileDoc, [RecipeNodeId; 4]) {
    let doc = ProfileDoc::empty_derived("ball_cut_by_brick", Tol::witness());
    let (doc, revolve) = ball(doc, 1.0, 0.0);
    let (doc, brick, stands) = brick_toward(doc, dir(68.0, 50.0), 0.985, 0.2, 0.3);
    let (doc, b) = world_in_brick(
        doc,
        brick,
        &stands,
        editor_core::Operand::output(revolve, 0),
    );
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
            members: editor_core::Bodies::spelled(vec![x.into(), y.into()]),
        },
        _ => Node::Intersect {
            members: editor_core::Bodies::spelled(vec![x.into(), y.into()]),
        },
    };
    let (doc, out) = insert(doc, node);
    (doc, [b, brick, out, revolve])
}

/// **Every op over the ball and the brick, in both orders, names**, the
/// circle's conventional vertex for the circle, the seam between the
/// ball and the brick. The ball and the brick name.
#[test]
fn every_op_over_a_ball_cut_by_a_brick_names() {
    for (op, ball_first) in [
        ("intersect", true),
        ("intersect", false),
        ("subtract", false),
        ("union", true),
        ("union", false),
        ("subtract", true),
    ] {
        let what = format!("{op}, ball first: {ball_first}");
        let (doc, [b, brick, out, revolve]) = document(op, ball_first);
        let ev = run(&doc);
        assert!(failure(&ev, b).is_none(), "{what}: the ball names");
        assert!(failure(&ev, brick).is_none(), "{what}: the brick names");
        if let Some(e) = failure(&ev, out) {
            panic!("{what}: the op names: {e}");
        }
        circle_vertex_cites(&doc, &ev, out, [revolve, brick], &what);
    }
}
