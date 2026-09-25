//! **A seam junction under a declared union is named by its lines, in
//! member space, whatever the member order.**
//!
//! The pair emitter names a seam JUNCTION — the vertex where k ≥ 2 seam
//! lines meet and no operand edge does — by the sorted run of those
//! lines' `Seam` segments. A declared union reaches that shape whenever
//! a slab crosses the merged edge of two flush-declared members before
//! the second of them is folded in, and the union's collapse reads the
//! run as ONE head: each line rewritten into member space and
//! canonicalized, the run re-sorted.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::collections::BTreeSet;

use crate::corpus::body_of;
use crate::docm7_union_declare::{block, declared_union, failure, flush_pairs, member_face, run};
use crate::fixture::{fname, table, vertex_of, wall};

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

/// **The row's fixture, `[b, g, a, h]`, fuses, and its junction is
/// the three lines meeting at `(0.3, 1, 1)`.**
///
/// There, `g`'s `x = 0.3` wall crosses `a`'s top cap and `a`'s `y = 1`
/// wall, and the merged top/`y = 1` edge of `a` and `b` passes through:
/// no operand edge reaches the point, so the three lines name it. Each
/// line's sides are member faces, one wrapper deep.
#[test]
fn a_seam_junction_in_a_declared_union_is_named_by_its_member_space_lines() {
    let Fixture { doc, a, b, g, h } = fixture((0.5, 3.0), true);
    let h = h.unwrap();
    let pairs = flush_pairs(&doc, (a, a), (b, b));
    let (docx, union, _) = declared_union(doc, &[b, g, a, h], pairs);
    let ev = run(&docx);
    assert!(failure(&ev, union).is_none(), "{:?}", failure(&ev, union));
    let body = body_of(&ev, union);
    let v = volume(body);
    // a ∪ b is 1.5; g adds 0.1 × 3 × 3 less its 0.1 × 1 × 0.5 inside
    // a, and h adds the same less its share inside b.
    assert!((v - (1.5 + 2.0 * (0.9 - 0.05))).abs() < 1e-9, "volume {v}");

    let line = |x: StableName, y: StableName| RoleSeg::Seam {
        a: x.into(),
        b: y.into(),
    };
    let a_top = member_face(union, a, fname(a, RoleSeg::Cap(CapEnd::End)));
    let a_y1 = member_face(union, a, fname(a, wall(&docx, a, 2)));
    let b_y1 = member_face(union, b, fname(b, wall(&docx, b, 2)));
    let g_x0 = member_face(union, g, fname(g, wall(&docx, g, 3)));
    let want = StableName {
        kind: EntityKind::Vertex,
        node: union,
        path: vec![
            line(a_top.clone(), b_y1),
            line(a_top, g_x0.clone()),
            line(a_y1, g_x0),
        ],
    };
    let t = table(&ev, union);
    assert_eq!(junctions(t), BTreeSet::from([want.clone()]));
    let vx = vertex_of(t, "the junction", &want);
    let p = body
        .get_vertex(vx)
        .and_then(|vd| body.get_point(vd.point))
        .copied()
        .expect("the junction vertex has a point");
    assert!(
        (p.x - 0.3).abs() < 1e-12 && (p.y - 1.0).abs() < 1e-12 && (p.z - 1.0).abs() < 1e-12,
        "the junction sits where the three lines meet, not at {p:?}"
    );
}

/// **Every order of every junction document: no emission bug, and the
/// orders that name a junction name it identically.**
///
/// Three documents reach the junction: the row's four members, the
/// same without `h`, and `g` lowered through the bottom cap. Orders
/// that refuse for reasons of their own (a declaration that no longer
/// resolves, an undeclared contact, a missing rule) are other rows'
/// subjects; what is asserted of them is only that none of them is an
/// `Emission`. At least two orders of each document name a junction,
/// so the equality is not vacuous.
///
/// Some fusing orders publish NO junction row. When `a` and `b` fold
/// before `g`, `g` cuts a chord out of one piece of `a`'s top/y-wall
/// rim, the chord is named as that piece, and the point where the
/// three lines meet is a vertex on that rim edge — named from the edge
/// and `g`'s wall, not as a junction of three seams. That is the
/// member-order dependence
/// `work/emit/declared-flush-union-edge-and-vertex-names-follow-member-order.md`
/// owns; those orders refused before the rim piece had a rule.
#[test]
fn a_junction_is_named_the_same_in_every_order_that_fuses() {
    for (label, g_z, with_h) in [
        ("four members", (0.5, 3.0), true),
        ("three members", (0.5, 3.0), false),
        ("through the bottom cap", (-0.5, 1.0), false),
    ] {
        let Fixture { doc, a, b, g, h } = fixture(g_z, with_h);
        let members: Vec<_> = [a, b, g].into_iter().chain(h).collect();
        let mut named = BTreeSet::new();
        let mut fused = 0;
        let mut rim_named = 0;
        for order in permutations(&members) {
            let (docx, union, _) = declared_union(doc.clone(), &order, flush_pairs(&doc, (a, a), (b, b)));
            let ev = run(&docx);
            match failure(&ev, union) {
                None => {
                    let js = junctions(table(&ev, union));
                    let ab_first = order[..2].contains(&a) && order[..2].contains(&b);
                    assert_eq!(
                        js.is_empty(),
                        ab_first,
                        "{label} {order:?}: a junction row is missing exactly when a and b fold first"
                    );
                    if js.is_empty() {
                        rim_named += 1;
                    } else {
                        named.insert(js);
                        fused += 1;
                    }
                }
                Some(e @ NodeErrorKind::Naming(NamingError::Emission { .. })) => {
                    panic!("{label} {order:?}: {e}")
                }
                Some(_) => {}
            }
        }
        assert!(fused >= 2, "{label}: {fused} orders named a junction");
        assert!(
            rim_named >= 1,
            "{label}: no order folds a and b first and fuses"
        );
        assert_eq!(
            named.len(),
            1,
            "{label}: the junction's name moved with the member order: {named:?}"
        );
    }
}
