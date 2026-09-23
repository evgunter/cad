//! **A boolean's vertices are read in the key space of the operand
//! whose clone holds them.**
//!
//! A boolean result's arena is A's clone with B grafted in, or ONE
//! operand's clone when the other contributes nothing. The vertex pass
//! names a vertex out of the table of the operand its key belongs to,
//! and a vertex no operand table names — one the reduction minted on
//! an edge — out of its incident edges and the reduction's contact
//! records. Both reads have to know which operand a key belongs to;
//! the rows below build each layout in both operand orders and pin
//! that the two orders name the same geometry the same way, with the
//! sides swapped.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::corpus::body_of;
use crate::docm7_union_declare::{block, failure, run};
use crate::fixture::{ename, insert, len, on_frame, point, table, vertex_of, vname};

use editor_core::{
    BooleanOp, BooleanValue, CapEnd, EntityKind, Evaluation, NameRef, Node, ProfileDoc,
    ProfileVertexRef, RecipeNodeId, RoleSeg, StableName, ValuePayload,
};
use geom_core::Tol;
use topo::BooleanResultKind;

fn pv(vertex: u32) -> ProfileVertexRef {
    ProfileVertexRef {
        loop_index: 0,
        vertex,
    }
}

fn union(doc: ProfileDoc, a: RecipeNodeId, b: RecipeNodeId) -> (ProfileDoc, RecipeNodeId) {
    insert(
        doc,
        Node::Boolean {
            op: BooleanOp::Union,
            a,
            b,
            declare: None,
        },
    )
}

/// The kind of result `id` evaluated to, or a panic naming what it did
/// instead.
fn kind_of(ev: &Evaluation<f64>, id: RecipeNodeId) -> BooleanResultKind {
    if let Some(e) = failure(ev, id) {
        panic!("the union refused: {e}");
    }
    match &ev.value(id).expect("the union evaluated").payload {
        ValuePayload::Boolean(BooleanValue::Body { kind, .. }) => *kind,
        other => panic!("expected a boolean body, got {}", other.kind_name()),
    }
}

/// The name a vertex minted where operand `x`'s vertex touches operand
/// `y`'s edge gets, with `x` on side A or B as the boolean ordered them.
fn touch(node: RecipeNodeId, a: StableName, b: StableName) -> StableName {
    vname(
        node,
        RoleSeg::Seam {
            a: NameRef::new(a),
            b: NameRef::new(b),
        },
    )
}

fn assert_at(ev: &Evaluation<f64>, id: RecipeNodeId, n: &StableName, at: [f64; 3]) {
    let v = vertex_of(table(ev, id), "the touch vertex", n);
    let p = point(body_of(ev, id), v);
    let d = ((p.x - at[0]).powi(2) + (p.y - at[1]).powi(2) + (p.z - at[2]).powi(2)).sqrt();
    assert!(d < 1e-9, "{n:?} names a vertex at {p:?}, not at {at:?}");
}

/// **A result that is B's clone names B's vertices out of B's table.**
///
/// `small` sits strictly inside `big`, so their union is `big` alone
/// and the result's keys are `big`'s. Each corner is named `FromB` of
/// `big`'s own corner name, and stands where that corner stands — a
/// key read against A's table instead finds `small`'s corner at the
/// same slot and publishes `FromA` of a point inside the body.
#[test]
fn a_result_that_is_b_names_its_vertices_out_of_b() {
    let doc = ProfileDoc::empty_derived("emit_vertex_keys_b", Tol::witness());
    let (doc, big) = block(doc, (0.0, 2.0), (0.0, 2.0), 0.0, 2.0);
    let (doc, small) = block(doc, (0.5, 1.0), (0.5, 1.0), 0.5, 0.5);
    let (doc, u) = union(doc, small, big);
    let ev = run(&doc);
    assert_eq!(kind_of(&ev, u), BooleanResultKind::OperandB);

    let corners = [(0.0, 0.0), (2.0, 0.0), (2.0, 2.0), (0.0, 2.0)];
    for (end, z) in [(CapEnd::Start, 0.0), (CapEnd::End, 2.0)] {
        for (i, (x, y)) in corners.into_iter().enumerate() {
            let own = vname(big, RoleSeg::CapVertex(end, pv(i as u32)));
            let n = vname(u, RoleSeg::FromB(NameRef::new(own)));
            assert_at(&ev, u, &n, [x, y, z]);
        }
    }
    let from_a = table(&ev, u)
        .iter()
        .filter(|(n, _)| n.kind == EntityKind::Vertex)
        .filter(|(n, _)| matches!(n.path.first(), Some(RoleSeg::FromA(_))))
        .count();
    assert_eq!(
        from_a, 0,
        "a result with no A material named a vertex FromA"
    );
}

/// An L-shaped block with a reflex vertical edge at (1, 1), and a
/// triangular prism inside it whose apex touches that edge at
/// (1, 1, 0.5) and nowhere else.
fn ell_and_tip(doc: ProfileDoc) -> (ProfileDoc, RecipeNodeId, RecipeNodeId) {
    let r = std::f64::consts::FRAC_1_SQRT_2;
    let (doc, lp) = on_frame(
        doc,
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![
            (0.0, 0.0),
            (2.0, 0.0),
            (2.0, 1.0),
            (1.0, 1.0),
            (1.0, 2.0),
            (0.0, 2.0),
        ]],
    );
    let (doc, ell) = insert(
        doc,
        Node::Extrude {
            profile: lp,
            distance: len(1.0),
        },
    );
    // Drawn on the plane through the apex normal to (1, 1, 0), and
    // extruded along (-1, -1, 0) into the ell's material.
    let (doc, tp) = on_frame(
        doc,
        [1.0, 1.0, 0.5],
        [r, -r, 0.0],
        [0.0, 0.0, 1.0],
        vec![vec![(0.0, 0.0), (0.4, -0.2), (0.4, 0.2)]],
    );
    let (doc, tip) = insert(
        doc,
        Node::Extrude {
            profile: tp,
            distance: len(0.5),
        },
    );
    (doc, ell, tip)
}

/// **A vertex minted on B's edge where A's vertex touches it is named
/// by its A partner, in a result that is B's clone.**
///
/// The tip lies inside the ell, so the union is the ell with its
/// reflex edge split at the apex. The split vertex has one B parent
/// (the reflex edge, both halves) and no A edge or face: its A parent
/// is the apex, recorded by the reduction as the vertex's contact
/// partner. The other order is the same geometry through A's clone,
/// and names the same vertex with the sides swapped.
#[test]
fn a_b_edge_split_by_an_a_vertex_is_named_by_its_a_partner() {
    let doc = ProfileDoc::empty_derived("emit_vertex_keys_touch", Tol::witness());
    let (doc, ell, tip) = ell_and_tip(doc);
    let (doc, tip_first) = union(doc, tip, ell);
    let (doc, ell_first) = union(doc, ell, tip);
    let ev = run(&doc);

    let apex = vname(tip, RoleSeg::CapVertex(CapEnd::Start, pv(0)));
    let reflex = ename(ell, RoleSeg::LateralEdge(pv(3)));
    let at = [1.0, 1.0, 0.5];

    assert_eq!(kind_of(&ev, tip_first), BooleanResultKind::OperandB);
    let n = touch(tip_first, apex.clone(), reflex.clone());
    assert_at(&ev, tip_first, &n, at);

    assert_eq!(kind_of(&ev, ell_first), BooleanResultKind::OperandA);
    let n = touch(ell_first, reflex, apex);
    assert_at(&ev, ell_first, &n, at);
}

/// **The same touch, with both operands kept: B grafted in beside A.**
///
/// A wedge whose edge runs along (1, -1, 0) through the cube's corner
/// (1, 1, 0), its material strictly on the far side of x + y = 2, so
/// the two solids meet at that one point and the union keeps both.
/// The wedge's edge gains a vertex at the corner, named by the cube's
/// corner and the wedge's edge whichever operand the wedge is.
#[test]
fn an_assembly_names_the_touch_vertex_by_its_partner_in_either_order() {
    let r = std::f64::consts::FRAC_1_SQRT_2;
    let doc = ProfileDoc::empty_derived("emit_vertex_keys_assembly", Tol::witness());
    let (doc, cube) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, p) = on_frame(
        doc,
        [1.0 - r, 1.0 + r, 0.0],
        [r, r, 0.0],
        [0.0, 0.0, 1.0],
        vec![vec![(0.0, 0.0), (1.0, -1.0), (1.0, 1.0)]],
    );
    let (doc, wedge) = insert(
        doc,
        Node::Extrude {
            profile: p,
            distance: len(2.0),
        },
    );
    let (doc, cube_first) = union(doc, cube, wedge);
    let (doc, wedge_first) = union(doc, wedge, cube);
    let ev = run(&doc);

    let corner = vname(cube, RoleSeg::CapVertex(CapEnd::Start, pv(2)));
    let ridge = ename(wedge, RoleSeg::LateralEdge(pv(0)));
    let at = [1.0, 1.0, 0.0];

    assert_eq!(kind_of(&ev, cube_first), BooleanResultKind::Assembly);
    let n = touch(cube_first, corner.clone(), ridge.clone());
    assert_at(&ev, cube_first, &n, at);

    assert_eq!(kind_of(&ev, wedge_first), BooleanResultKind::Assembly);
    let n = touch(wedge_first, ridge, corner);
    assert_at(&ev, wedge_first, &n, at);
}
