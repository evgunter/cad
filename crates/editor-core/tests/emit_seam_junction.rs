//! **Where a slab crosses the merged rim of two flush-declared members,
//! a union names the point by the rim and the slab, whatever the member
//! order.**
//!
//! The pair emitter names a seam JUNCTION — the vertex where k ≥ 2 seam
//! lines meet and no operand edge does — by the sorted run of those
//! lines' `Seam` segments. A declared union's fold reaches that shape
//! whenever a slab crosses the merged edge of two flush-declared members
//! before the second of them is folded in; folded after them, it meets a
//! piece of the rim instead. The merged edge lies along a member's rim,
//! and the published table names the point from the finished body
//! (`emit_union::cite_member_edges`): that rim, crossed by the slab's
//! wall.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::collections::BTreeSet;

use crate::corpus::body_of;
use crate::docm7_union_declare::{block, declared_union, failure, flush_pairs, member_face, run};
use crate::fixture::{ename, fname, member_entity, table, vertex_of, wall};

use editor_core::{
    CapEnd, EntityKind, NameTable, NamingError, NodeErrorKind, ProfileDoc, RecipeNodeId, RoleSeg,
    StableName,
};
use geom_core::Tol;

/// Every ordering of `items`.
fn permutations(items: &[RecipeNodeId]) -> Vec<Vec<RecipeNodeId>> {
    if items.len() <= 1 {
        return vec![items.to_vec()];
    }
    let mut out = Vec::new();
    for (i, &head) in items.iter().enumerate() {
        let mut rest = items.to_vec();
        rest.remove(i);
        for mut tail in permutations(&rest) {
            tail.insert(0, head);
            out.push(tail);
        }
    }
    out
}

fn volume(body: &topo::Body<f64>) -> f64 {
    topo::mass_properties(body, Tol::witness())
        .expect("mass")
        .volume
}

/// A junction row: a vertex named by a run of two or more seam lines.
fn junctions(t: &NameTable) -> BTreeSet<StableName> {
    t.iter()
        .map(|(n, _)| n)
        .filter(|n| {
            n.kind == EntityKind::Vertex
                && n.path.len() >= 2
                && n.path.iter().all(|s| matches!(s, RoleSeg::Seam { .. }))
        })
        .cloned()
        .collect()
}

/// The row's document: `a` and `b` meet flush along x with all four
/// families declared, so their caps and y-walls merge; `g` is a slab
/// through `a` alone, rising out of the top cap and through both
/// y-walls; `h` a second slab, over the merged overlap's far end.
/// `(z0, dz)` places `g`, so the same shape is had through the bottom
/// cap too.
struct Fixture {
    doc: ProfileDoc,
    a: RecipeNodeId,
    b: RecipeNodeId,
    g: RecipeNodeId,
    h: Option<RecipeNodeId>,
}

fn fixture(g_z: (f64, f64), with_h: bool) -> Fixture {
    let doc = ProfileDoc::empty_derived("emit_seam_junction", Tol::witness());
    let (doc, a) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, b) = block(doc, (0.5, 1.5), (0.0, 1.0), 0.0, 1.0);
    let (doc, g) = block(doc, (0.3, 0.4), (-1.0, 2.0), g_z.0, g_z.1);
    let (doc, h) = if with_h {
        let (doc, h) = block(doc, (1.0, 1.1), (-1.0, 2.0), 0.5, 3.0);
        (doc, Some(h))
    } else {
        (doc, None)
    };
    Fixture { doc, a, b, g, h }
}

/// The name every fused order gives the point where `g`'s `x = 0.3`
/// wall crosses `a`'s `cap` / `y = 1` rim: that rim and that wall, in
/// name order.
fn crossing(
    doc: &ProfileDoc,
    union: RecipeNodeId,
    a: RecipeNodeId,
    g: RecipeNodeId,
    cap: CapEnd,
) -> StableName {
    let rim = member_entity(
        union,
        a,
        ename(
            a,
            RoleSeg::RimEdge(cap, crate::fixture::piece(doc, a, 0, 2)),
        ),
        EntityKind::Edge,
    );
    let g_x0 = member_face(union, g, fname(g, wall(doc, g, 3)));
    let (lo, hi) = if rim < g_x0 { (rim, g_x0) } else { (g_x0, rim) };
    StableName {
        kind: EntityKind::Vertex,
        node: union,
        path: vec![RoleSeg::Seam {
            a: lo.into(),
            b: hi.into(),
        }],
    }
}

/// Where the vertex named `name` sits in `ev`'s union.
fn point_of(ev: &editor_core::Evaluation<f64>, union: RecipeNodeId, name: &StableName) -> [f64; 3] {
    let body = body_of(ev, union);
    let vx = vertex_of(table(ev, union), "the crossing", name);
    let p = body
        .get_vertex(vx)
        .and_then(|vd| body.get_point(vd.point))
        .copied()
        .expect("the crossing has a point");
    [p.x, p.y, p.z]
}

/// **The row's fixture, `[b, g, a, h]`, fuses, and the point where `g`
/// crosses the merged top / `y = 1` edge is named by `a`'s rim and `g`'s
/// wall, not as a junction of three seams.**
///
/// At `(0.3, 1, 1)`, `g`'s `x = 0.3` wall crosses `a`'s top cap and
/// `a`'s `y = 1` wall, and the merged top / `y = 1` edge of `a` and `b`
/// passes through. No operand edge reaches the point at the step that
/// makes it, so the fold names it by its three seam lines. The merged
/// edge lies along `a`'s rim there, and the published table names what
/// the finished body holds: `a`'s rim, crossed by `g`'s wall.
#[test]
fn a_slab_crossing_a_merged_rim_is_named_by_the_rim_and_the_slab() {
    let Fixture { doc, a, b, g, h } = fixture((0.5, 3.0), true);
    let h = h.unwrap();
    let pairs = flush_pairs(&doc, (a, a), (b, b));
    let (docx, union, _) = declared_union(doc, &[b, g, a, h], pairs);
    let ev = run(&docx);
    assert!(failure(&ev, union).is_none(), "{:?}", failure(&ev, union));
    let v = volume(body_of(&ev, union));
    // a ∪ b is 1.5; g adds 0.1 × 3 × 3 less its 0.1 × 1 × 0.5 inside
    // a, and h adds the same less its share inside b.
    assert!((v - (1.5 + 2.0 * (0.9 - 0.05))).abs() < 1e-9, "volume {v}");
    assert_eq!(junctions(table(&ev, union)), BTreeSet::new());
    let p = point_of(&ev, union, &crossing(&docx, union, a, g, CapEnd::End));
    assert!(
        (p[0] - 0.3).abs() < 1e-12 && (p[1] - 1.0).abs() < 1e-12 && (p[2] - 1.0).abs() < 1e-12,
        "the crossing sits where g's wall meets the rim, not at {p:?}"
    );
}

/// **Every order of every such document names the crossing the same.**
///
/// Three documents reach it: the row's four members, the same without
/// `h`, and `g` lowered through the bottom cap (the crossing then on
/// `a`'s bottom / `y = 1` rim). Orders that refuse for reasons of their
/// own (a declaration that no longer resolves, an undeclared contact)
/// are other rows' subjects; what is asserted of them is only that none
/// of them is an `Emission`.
///
/// Whether `a` and `b` fold before `g` decides what the fold meets: a
/// piece of `a`'s rim, or three seam lines. The published name is the
/// same either way, and no order publishes a junction.
#[test]
fn a_crossing_of_a_merged_rim_is_named_the_same_in_every_order_that_fuses() {
    for (label, g_z, with_h, cap, z) in [
        ("four members", (0.5, 3.0), true, CapEnd::End, 1.0),
        ("three members", (0.5, 3.0), false, CapEnd::End, 1.0),
        (
            "through the bottom cap",
            (-0.5, 1.0),
            false,
            CapEnd::Start,
            0.0,
        ),
    ] {
        let Fixture { doc, a, b, g, h } = fixture(g_z, with_h);
        let members: Vec<_> = [a, b, g].into_iter().chain(h).collect();
        let (mut ab_first, mut g_between) = (0, 0);
        for order in permutations(&members) {
            let (docx, union, _) =
                declared_union(doc.clone(), &order, flush_pairs(&doc, (a, a), (b, b)));
            let ev = run(&docx);
            match failure(&ev, union) {
                None => {
                    assert_eq!(
                        junctions(table(&ev, union)),
                        BTreeSet::new(),
                        "{label} {order:?}"
                    );
                    let p = point_of(&ev, union, &crossing(&docx, union, a, g, cap));
                    assert!(
                        (p[0] - 0.3).abs() < 1e-12
                            && (p[1] - 1.0).abs() < 1e-12
                            && (p[2] - z).abs() < 1e-12,
                        "{label} {order:?}: the crossing is at {p:?}"
                    );
                    if order[..2].contains(&a) && order[..2].contains(&b) {
                        ab_first += 1;
                    } else {
                        g_between += 1;
                    }
                }
                Some(e @ NodeErrorKind::Naming(NamingError::Emission { .. })) => {
                    panic!("{label} {order:?}: {e}")
                }
                Some(_) => {}
            }
        }
        assert!(
            ab_first >= 1 && g_between >= 2,
            "{label}: {ab_first} orders fold a and b first, {g_between} do not"
        );
    }
}
