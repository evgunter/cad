//! **A whole ball cut by a tilted brick**: the unit ball and the brick
//! of [`emit_sphere_cut_in_cap`](crate::emit_sphere_cut_in_cap), whose
//! near face cuts a cap of height 0.015 off the ball.
//!
//! The ball is one closed sphere group, so the kernel re-charts it
//! (`topo::boolean::ops::apply_recuts`): it turns the ball about its
//! centre until a pole lies on the brick's axis, keeping every key. The
//! result's seam pieces and poles then descend from the ball's by key
//! but lie off the ball's own seam and poles. The subtract names them by
//! key; the union reads member vertices off the members' own bodies,
//! finds none at the turned pole, and reads it by the lines through it:
//! the ball's two meridians, which lie on one great circle and so are
//! one line, meeting no face of another member there. Which reading is
//! right is the open question of [`ROW`].
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::blocks::{failure, run};
use crate::emit_sphere_cut_in_cap::{ball, brick_toward, dir, world_in_brick};
use crate::fixture::insert;
use editor_core::{
    BooleanValue, EntityKey, EntityKind, Entry, Node, ProfileDoc, RecipeNodeId, RoleSeg,
    ValuePayload,
};
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

fn body_of(payload: &ValuePayload<f64>) -> &topo::Body<f64> {
    match payload {
        ValuePayload::Body(b) => b,
        ValuePayload::Boolean(BooleanValue::Body { body, .. }) => body,
        ValuePayload::Copies(copies) if copies.bodies.len() == 1 => &copies.bodies[0],
        other => panic!("expected a body, got {other:?}"),
    }
}

/// **Every op over the ball and the brick, in both orders**: the
/// intersect and the union refuse at the turned pole ([`ROW`]); the
/// subtract names, and cites one of the ball's poles at a point where
/// the ball holds no vertex, the turned pole.
#[test]
fn a_ball_cut_by_a_brick_refuses_at_its_turned_pole_where_a_member_vertex_is_read() {
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
        if op != "subtract" {
            let e = format!("{:?}", failure(&ev, out).expect("refuses"));
            assert!(
                e.contains("one member edge meets no face at a union vertex no seam reaches"),
                "{what}: refused other than at the turned pole ({ROW}): {e}"
            );
            continue;
        }
        assert!(failure(&ev, out).is_none(), "{what}: the subtract names");
        let ball = ev.value(b).expect("the ball's value");
        let poles: Vec<_> = body_of(&ball.payload)
            .vertex_points()
            .map(|(_, p)| p)
            .collect();
        let value = ev.value(out).expect("the subtract's value");
        let points: std::collections::BTreeMap<_, _> =
            body_of(&value.payload).vertex_points().collect();
        let turned: Vec<_> = crate::fixture::table(&ev, out)
            .iter()
            .filter_map(|(n, e)| match (n.path.as_slice(), e) {
                ([RoleSeg::From { of, .. }], Entry::Unique(r))
                    if of.kind == EntityKind::Vertex
                        && matches!(of.path.as_slice(), [RoleSeg::Placed { of }]
                            if matches!(of.path.as_slice(), [RoleSeg::Pole(_)])) =>
                {
                    match r.key {
                        EntityKey::Vertex(v) => Some(points[&v]),
                        _ => None,
                    }
                }
                _ => None,
            })
            .collect();
        assert_eq!(turned.len(), 1, "{what}: one of the ball's poles survives");
        assert!(
            poles.iter().all(|&p| (p - turned[0]).norm() > 0.1),
            "{what}: the subtract cites a ball pole at {:?}, off the ball's own poles {poles:?}",
            turned[0]
        );
    }
}
