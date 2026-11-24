//! **A cut through a conventional vertex's point reads its edge's
//! interior** (`names/README.md`, "A conventional vertex"): the vertex
//! has no identity of its own, so the vertex a later cut leaves there is
//! named as the crossing of the circle a cut a hair away leaves, and
//! never for the conventional vertex it sits on. Member order moves
//! that vertex along the circle, so a fixed plane passes through it in
//! some orders and beside it in others; the names may not tell which.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::collections::BTreeSet;

use crate::blocks::{block, block_on, failure, run};
use crate::corpus::body_of;
use crate::emit_shared_rim_several::permutations;
use crate::emit_union_rim_piece_ranks::signature;
use editor_core::{EntityKey, Entry, Evaluation, ProfileDoc, RecipeNodeId, RoleSeg, StableName};
use geom_core::Tol;

/// The ball's turn about its own axis: its seam stays below the
/// blocks' top, and member order leaves the circle's vertex at more
/// than one point.
const SPIN: f64 = 0.7;

/// `emit_union_conventional_vertex_orders`' cap on two flush blocks,
/// the ball turned [`SPIN`] about its axis, in `order`; and, where
/// `cut` is given, the union less a block whose near face is the plane
/// x = `cut`.
fn document(order: &[usize], cut: Option<f64>) -> (ProfileDoc, RecipeNodeId, Option<RecipeNodeId>) {
    let doc = ProfileDoc::empty_derived("a_cut_through_a_conventional_vertex", Tol::witness());
    let (doc, a) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.5);
    let (doc, b) = block_on(doc, a, (1.0, 2.0), (0.0, 1.0), 0.0, 0.5);
    let (doc, ball) = crate::emit_sphere_cut_in_cap::ball(doc, 0.3, 0.5);
    let frame = editor_core::Frame::rotate_then_translate(
        [0.0, 1.0, 0.0],
        SPIN,
        [1.0, 0.0, 0.35],
        geom_core::Band::new(1e-9, 1e-6).unwrap(),
    )
    .unwrap();
    let (doc, ball) =
        crate::fixture::against(doc, a, editor_core::Operand::output(ball, 0), &frame);
    let members = [a, b, ball];
    let ordered: Vec<RecipeNodeId> = order.iter().map(|&i| members[i]).collect();
    let (doc, union) = crate::fixture::union_over(doc, &ordered);
    let Some(x) = cut else {
        return (doc, union, None);
    };
    let (doc, tool) = block_on(doc, a, (x, 3.0), (-1.0, 2.0), -1.0, 3.0);
    let (doc, out) = crate::fixture::insert(
        doc,
        editor_core::Node::Subtract {
            from: union.into(),
            tool: tool.into(),
        },
    );
    (doc, union, Some(out))
}

/// Where `out`'s table holds its one conventional vertex.
fn conventional_point(ev: &Evaluation<f64>, out: RecipeNodeId) -> geom_core::Point3<f64> {
    let body = body_of(ev, out);
    let at: Vec<_> = crate::fixture::table(ev, out)
        .iter()
        .filter_map(|(n, e)| match (n.path.as_slice(), e) {
            ([RoleSeg::ConventionalVertex(_)], Entry::Unique(e)) => match e.key {
                EntityKey::Vertex(v) => {
                    Some(*body.get_point(body.get_vertex(v).unwrap().point).unwrap())
                }
                _ => None,
            },
            _ => None,
        })
        .collect();
    let [p] = at.as_slice() else {
        panic!("one conventional vertex, not {at:?}")
    };
    *p
}

/// `out`'s published names, after it names, each spelled with every
/// node's digest left out: two documents that differ only in where the
/// cut stands mint their cut's nodes under other digests, at the same
/// ordinals.
fn names(doc: &ProfileDoc, out: RecipeNodeId, what: &str) -> (Evaluation<f64>, BTreeSet<String>) {
    let ev = run(doc);
    if let Some(e) = failure(&ev, out) {
        panic!("{what}: the cut names: {e}");
    }
    let names = crate::fixture::table(&ev, out)
        .iter()
        .map(|(n, _)| undigested(n))
        .collect();
    (ev, names)
}

/// `name`'s `Debug` with each `digest: <n>` dropped.
fn undigested(name: &StableName) -> String {
    let said = format!("{name:?}");
    let mut out = String::with_capacity(said.len());
    let mut rest = said.as_str();
    while let Some(at) = rest.find("digest: ") {
        out.push_str(&rest[..at]);
        rest = rest[at + "digest: ".len()..].trim_start_matches(|c: char| c.is_ascii_digit());
    }
    out.push_str(rest);
    out
}

/// **A plane through the vertex names as a plane a hair beside it**, in
/// every member order: the cut's table is one set of names at the
/// vertex's own x and 1e-7 past it. Red when the cut through the point
/// carries the conventional vertex's name onto the vertex it leaves.
#[test]
fn a_cut_through_the_circles_vertex_names_as_one_a_hair_beside_it() {
    for order in permutations(&[0, 1, 2]) {
        let at = format!("{order:?}");
        let (doc, union, _) = document(&order, None);
        let ev = run(&doc);
        assert!(failure(&ev, union).is_none(), "{at}: the union names");
        let x = conventional_point(&ev, union).x;
        let cut = |x: f64| {
            let (doc, _, out) = document(&order, Some(x));
            names(&doc, out.expect("a cut"), &at).1
        };
        let (through, beside) = (cut(x), cut(x + 1e-7));
        assert_eq!(
            through, beside,
            "{at}: through the vertex at x = {x} against 1e-7 past it"
        );
    }
}

/// **One plane names alike in every member order**, though it passes
/// through the circle's vertex in some orders and beside it in the
/// others: the plane through the vertex where `[0, 1, 2]` leaves it.
/// The cut's table is compared by what each name denotes
/// ([`signature`]).
#[test]
fn a_cut_where_one_order_leaves_the_vertex_names_alike_in_every_order() {
    let (doc, union, _) = document(&[0, 1, 2], None);
    let x = conventional_point(&run(&doc), union).x;
    let mut through = 0;
    let mut tables = Vec::new();
    for order in permutations(&[0, 1, 2]) {
        let at = format!("{order:?}");
        let (doc, union, out) = document(&order, Some(x));
        let out = out.expect("a cut");
        let (ev, _) = names(&doc, out, &at);
        through += usize::from(conventional_point(&ev, union).x == x);
        tables.push((at, signature(&ev, out)));
    }
    assert!(
        through > 0 && through < 6,
        "the plane passes through the vertex in some orders only: {through}"
    );
    let (first_at, first) = &tables[0];
    for (at, table) in &tables[1..] {
        assert_eq!(first, table, "{first_at} against {at}");
    }
}

/// **The ball ∪ tilted brick, less a block whose face passes through the
/// circle's vertex, names as the same less a block a hair beside it**
/// (`emit_ball_cut_by_brick`'s document, in the brick's space).
#[test]
fn a_cut_through_the_ball_and_bricks_circle_vertex_names_as_one_beside_it() {
    let (doc, [_, brick, union, _]) = crate::emit_ball_cut_by_brick::document("union", true);
    let ev = run(&doc);
    assert!(failure(&ev, union).is_none(), "the union names");
    let p = conventional_point(&ev, union);
    let cut = |x: f64| {
        let (doc, tool) = block_on(doc.clone(), brick, (x, x + 5.0), (-5.0, 5.0), -5.0, 10.0);
        let (doc, out) = crate::fixture::insert(
            doc,
            editor_core::Node::Subtract {
                from: union.into(),
                tool: tool.into(),
            },
        );
        names(&doc, out, &format!("x = {x}")).1
    };
    assert_eq!(
        cut(p.x),
        cut(p.x + 1e-7),
        "through the vertex at {p:?} against 1e-7 past it"
    );
}

/// **A union of the cap union alone, and a subtract of nothing from it,
/// name**: each carries the cap union's entities in through its read,
/// so the circle's vertex is the circle's again, under the carrying
/// node's own name for the circle. Red where a door publishes a table
/// without naming its conventional vertices.
#[test]
fn a_lone_member_union_and_a_toolless_subtract_name_the_circles_vertex() {
    let (doc, cap, _) = document(&[0, 1, 2], None);
    let (doc, lone) = crate::fixture::insert(
        doc,
        editor_core::Node::Union {
            members: editor_core::Bodies::spelled(vec![cap.into()]),
        },
    );
    let (doc, toolless) = crate::fixture::insert(
        doc,
        editor_core::Node::Subtract {
            from: cap.into(),
            tool: editor_core::Bodies::spelled(Vec::new()),
        },
    );
    let ev = run(&doc);
    for (what, node) in [
        ("the lone-member union", lone),
        ("the toolless subtract", toolless),
    ] {
        if let Some(e) = failure(&ev, node) {
            panic!("{what} names: {e}");
        }
        let vertices: Vec<_> = crate::fixture::table(&ev, node)
            .iter()
            .filter_map(|(n, _)| match n.path.as_slice() {
                [RoleSeg::ConventionalVertex(edge)] => Some(edge.node),
                _ => None,
            })
            .collect();
        assert_eq!(
            vertices,
            [node],
            "{what}: one vertex, named for its own name of the circle"
        );
        let _ = conventional_point(&ev, node);
    }
}

/// **A union with a block whose face passes through the cap union's
/// circle vertex names as one beside it**: the union reads its member's
/// vertices where a finished vertex sits on one (`emit_union::Flush`),
/// and a member's conventional vertex is none it may cite. Red where
/// the union names the finished vertex for the member's.
#[test]
fn a_union_through_a_members_circle_vertex_names_as_one_beside_it() {
    let (doc, cap, _) = document(&[0, 1, 2], None);
    let x = conventional_point(&run(&doc), cap).x;
    let unite = |x: f64| {
        let (doc, block) = block_on(doc.clone(), cap, (x, 3.0), (-1.0, 2.0), -1.0, 3.0);
        let (doc, out) = crate::fixture::insert(
            doc,
            editor_core::Node::Union {
                members: editor_core::Bodies::spelled(vec![cap.into(), block.into()]),
            },
        );
        names(&doc, out, &format!("x = {x}")).1
    };
    assert_eq!(
        unite(x),
        unite(x + 1e-7),
        "through the vertex at x = {x} against 1e-7 past it"
    );
}
