//! **A boolean names each entity out of the operand whose key it
//! carries, and swapping the operands swaps the sides of every name.**
//!
//! A boolean result's arena is A's clone with B grafted in, or ONE
//! operand's clone when the other contributes nothing. The emitter reads
//! every result key through one layout read to find the operand it
//! belongs to; a vertex no operand table names — one the reduction
//! minted on an edge — is named out of its incident edges and the
//! reduction's contact records, on whichever side its key belongs to.
//!
//! The rows:
//! - a nested pair in all four union/intersect orders: the surviving
//!   operand's corners are named out of ITS table, whichever side it is;
//! - a vertex minted on one operand's edge where the other's vertex
//!   touches it, in a result that is one operand's clone and in an
//!   assembly, each in both orders, pinned by name;
//! - the operand-swap row: five fixtures, union and intersection, both
//!   orders; every face, edge and vertex name of `x op y`, with `FromA` and
//!   `FromB` exchanged and each `Seam{a, b}` read as `Seam{b, a}`, is the
//!   name the same geometry gets in `y op x`. It guards SYMMETRY only —
//!   a consistent A/B relabel would pass it — so the rows above are what
//!   pin which side each name belongs to.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::collections::BTreeSet;

use crate::corpus::body_of;
use crate::docm7_union_declare::{block, declared_union, failure, flush_pairs, run};
use crate::fixture::{
    ang, ename, ends, face_vertices, insert, len, on_frame, point, scl, table, vertex_of, vname,
};

use editor_core::{
    BooleanOp, BooleanValue, CapEnd, EntityKey, EntityKind, Entry, Evaluation, NameRef, Node,
    ProfileDoc, ProfileVertexRef, RecipeNodeId, RoleSeg, StableName, ValuePayload,
};
use geom_core::Tol;
use topo::BooleanResultKind;

fn pv(vertex: u32) -> ProfileVertexRef {
    ProfileVertexRef {
        loop_index: 0,
        vertex,
    }
}

fn boolean(
    doc: ProfileDoc,
    op: BooleanOp,
    a: RecipeNodeId,
    b: RecipeNodeId,
) -> (ProfileDoc, RecipeNodeId) {
    insert(
        doc,
        Node::Boolean {
            op,
            a,
            b,
            declare: None,
        },
    )
}

fn union(doc: ProfileDoc, a: RecipeNodeId, b: RecipeNodeId) -> (ProfileDoc, RecipeNodeId) {
    boolean(doc, BooleanOp::Union, a, b)
}

/// The kind of result `id` evaluated to, or a panic naming what it did
/// instead.
fn kind_of(ev: &Evaluation<f64>, id: RecipeNodeId) -> BooleanResultKind {
    if let Some(e) = failure(ev, id) {
        panic!("the boolean refused: {e}");
    }
    match &ev.value(id).expect("the boolean evaluated").payload {
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

fn nested(doc: ProfileDoc) -> (ProfileDoc, RecipeNodeId, RecipeNodeId) {
    let (doc, big) = block(doc, (0.0, 2.0), (0.0, 2.0), 0.0, 2.0);
    let (doc, small) = block(doc, (0.5, 1.0), (0.5, 1.0), 0.5, 0.5);
    (doc, big, small)
}

/// **The operand that survives names the result's corners, whichever
/// side it is.**
///
/// `small` sits strictly inside `big`: their union is `big` and their
/// intersection `small`, each a clone of one operand. In each of the
/// four orders every corner of the survivor is named `From<its side>`
/// of the survivor's own corner name, at that corner's point, and no
/// vertex is named from the absent side. A B-clone key read against A's
/// table instead finds the other block's corner at the same slot and
/// publishes its name at a point where that corner is not.
#[test]
fn the_surviving_operand_names_the_corners_in_every_order() {
    let doc = ProfileDoc::empty_derived("emit_vertex_keys_nested", Tol::witness());
    let (doc, big, small) = nested(doc);
    let cases = [
        (
            BooleanOp::Union,
            small,
            big,
            big,
            BooleanResultKind::OperandB,
        ),
        (
            BooleanOp::Union,
            big,
            small,
            big,
            BooleanResultKind::OperandA,
        ),
        (
            BooleanOp::Intersect,
            big,
            small,
            small,
            BooleanResultKind::OperandB,
        ),
        (
            BooleanOp::Intersect,
            small,
            big,
            small,
            BooleanResultKind::OperandA,
        ),
    ];
    let mut doc = doc;
    let mut ids = Vec::new();
    for (op, a, b, _, _) in cases {
        let (d, id) = boolean(doc, op, a, b);
        doc = d;
        ids.push(id);
    }
    let ev = run(&doc);
    for ((op, _, _, survivor, kind), id) in cases.into_iter().zip(ids) {
        assert_eq!(kind_of(&ev, id), kind, "{op:?}");
        let (x, y, z, dz) = if survivor == big {
            ((0.0, 2.0), (0.0, 2.0), 0.0, 2.0)
        } else {
            ((0.5, 1.0), (0.5, 1.0), 0.5, 0.5)
        };
        let corners = [(x.0, y.0), (x.1, y.0), (x.1, y.1), (x.0, y.1)];
        for (end, h) in [(CapEnd::Start, z), (CapEnd::End, z + dz)] {
            for (i, (px, py)) in corners.into_iter().enumerate() {
                let own = NameRef::new(vname(survivor, RoleSeg::CapVertex(end, pv(i as u32))));
                let seg = if kind == BooleanResultKind::OperandA {
                    RoleSeg::FromA(own)
                } else {
                    RoleSeg::FromB(own)
                };
                assert_at(&ev, id, &vname(id, seg), [px, py, h]);
            }
        }
        let absent = table(&ev, id)
            .iter()
            .filter(|(n, _)| n.kind == EntityKind::Vertex)
            .filter(|(n, _)| match n.path.first() {
                Some(RoleSeg::FromA(_)) => kind == BooleanResultKind::OperandB,
                Some(RoleSeg::FromB(_)) => kind == BooleanResultKind::OperandA,
                _ => false,
            })
            .count();
        assert_eq!(
            absent, 0,
            "{op:?}: a vertex was named from the absent operand"
        );
    }
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

/// A tip whose apex touches the top face of a block at an interior
/// point, hanging down into the block.
fn face_touch(doc: ProfileDoc) -> (ProfileDoc, RecipeNodeId, RecipeNodeId) {
    let (doc, bl) = block(doc, (0.0, 2.0), (0.0, 2.0), 0.0, 1.0);
    let (doc, tp) = on_frame(
        doc,
        [1.0, 0.8, 1.0],
        [0.0, 0.0, -1.0],
        [1.0, 0.0, 0.0],
        vec![vec![(0.0, 0.0), (0.4, -0.2), (0.4, 0.2)]],
    );
    let (doc, tip) = insert(
        doc,
        Node::Extrude {
            profile: tp,
            distance: len(0.4),
        },
    );
    (doc, bl, tip)
}

/// A tip whose apex touches a block's vertical edge from outside.
fn edge_touch_outside(doc: ProfileDoc) -> (ProfileDoc, RecipeNodeId, RecipeNodeId) {
    let r = std::f64::consts::FRAC_1_SQRT_2;
    let (doc, bl) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, tp) = on_frame(
        doc,
        [1.0, 1.0, 0.5],
        [r, -r, 0.0],
        [0.0, 0.0, 1.0],
        vec![vec![(0.0, 0.0), (0.4, -0.2), (0.4, 0.2)]],
    );
    // The frame normal points into the block; a negative distance
    // extrudes the tip away from it.
    let (doc, tip) = insert(
        doc,
        Node::Extrude {
            profile: tp,
            distance: len(-0.5),
        },
    );
    (doc, bl, tip)
}

/// A SEAMED result that also carries an unzipped touch: an L-prism
/// whose convex corner (1, 1, 0) is touched by a tilted wedge's ridge,
/// the wedge crossing the L's long arm elsewhere — a zip in the body,
/// none at the touch.
fn seamed_touch(doc: ProfileDoc) -> (ProfileDoc, RecipeNodeId, RecipeNodeId) {
    let (doc, lp) = on_frame(
        doc,
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![
            (0.0, 0.0),
            (3.0, 0.0),
            (3.0, 0.5),
            (1.0, 0.5),
            (1.0, 1.0),
            (0.0, 1.0),
        ]],
    );
    let (doc, ell) = insert(
        doc,
        Node::Extrude {
            profile: lp,
            distance: len(1.0),
        },
    );
    let n = (1.0f64 + 1.0 + 0.09).sqrt();
    let d = [1.0 / n, -1.0 / n, -0.3 / n];
    let r = std::f64::consts::FRAC_1_SQRT_2;
    let u = [r, r, 0.0];
    let v = [
        d[1] * u[2] - d[2] * u[1],
        d[2] * u[0] - d[0] * u[2],
        d[0] * u[1] - d[1] * u[0],
    ];
    let s = 0.7;
    let o = [1.0 - s * d[0], 1.0 - s * d[1], -s * d[2]];
    let (doc, p) = on_frame(
        doc,
        o,
        u,
        v,
        vec![vec![(0.0, 0.0), (1.0, -1.0), (1.0, 1.0)]],
    );
    let (doc, wedge) = insert(
        doc,
        Node::Extrude {
            profile: p,
            distance: len(2.0),
        },
    );
    (doc, ell, wedge)
}

/// A bar — the declared union of two flush, x-offset placements of one
/// unit block, whose merged front and top faces keep two collinear
/// top-front edges, so the two faces share more than one rim — and a
/// small prism inside it whose apex touches that line at (0.6, 0, 1).
fn bar_and_tip(doc: ProfileDoc) -> (ProfileDoc, RecipeNodeId, RecipeNodeId) {
    let (doc, proto) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let place = |doc: ProfileDoc, dx: f64| {
        insert(
            doc,
            Node::Transform {
                input: proto,
                translation: [len(dx), len(0.0), len(0.0)],
                rotation_axis: [scl(0.0), scl(0.0), scl(1.0)],
                rotation_angle: ang(0.0),
            },
        )
    };
    let (doc, m1) = place(doc, 0.0);
    let (doc, m2) = place(doc, 0.5);
    let (doc, bar, _) = declared_union(doc, &[m1, m2], flush_pairs((m1, proto), (m2, proto)));
    // The tip's frame: normal (1, 1, -1)/sqrt3 pointing into the bar.
    let s3 = 3f64.sqrt();
    let n = [1.0 / s3, 1.0 / s3, -1.0 / s3];
    let l = 1.5f64.sqrt();
    let e1 = [-1.0 / l, 0.5 / l, -0.5 / l];
    let e2 = [
        n[1] * e1[2] - n[2] * e1[1],
        n[2] * e1[0] - n[0] * e1[2],
        n[0] * e1[1] - n[1] * e1[0],
    ];
    let (doc, tp) = on_frame(
        doc,
        [0.6, 0.0, 1.0],
        e1,
        e2,
        vec![vec![(0.0, 0.0), (0.3, -0.1), (0.3, 0.1)]],
    );
    let (doc, tip) = insert(
        doc,
        Node::Extrude {
            profile: tp,
            distance: len(0.3),
        },
    );
    (doc, bar, tip)
}

/// **An edge split in a result that is B's clone descends to B's edge,
/// even where its two faces share a second rim.**
///
/// The tip lies inside the bar, so `tip ∪ bar` is the bar — B's clone —
/// with its top-front line split at the apex. Each half is named `FromB`
/// of the bar edge it lies on, exactly as each half of `bar ∪ tip` (A's
/// clone) is named `FromA` of it. The root is read off the split
/// lineage, not re-derived from the adjacent faces: the bar's merged
/// front and top faces share two collinear edges, so "the one rim the
/// two faces share" has no answer here.
#[test]
fn a_b_edge_split_in_a_b_clone_descends_to_its_b_edge() {
    let doc = ProfileDoc::empty_derived("emit_vertex_keys_bar", Tol::witness());
    let (doc, bar, tip) = bar_and_tip(doc);
    let (doc, tip_first) = union(doc, tip, bar);
    let (doc, bar_first) = union(doc, bar, tip);
    let ev = run(&doc);

    // The bar's own name for the top-front edge through x = 0.6.
    let on_line = |body: &topo::Body<f64>, e| {
        let [p, q] = ends(body, e).map(|v| point(body, v));
        let on = |p: geom_core::Point3<f64>| p.y.abs() < 1e-9 && (p.z - 1.0).abs() < 1e-9;
        on(p) && on(q) && p.x.min(q.x) < 0.6 + 1e-9 && p.x.max(q.x) > 0.6 - 1e-9
    };
    let bar_body = body_of(&ev, bar);
    let roots: Vec<StableName> = table(&ev, bar)
        .iter()
        .filter_map(|(n, e)| match e {
            Entry::Unique(r) => match r.key {
                EntityKey::Edge(k) if on_line(bar_body, k) => Some(n.clone()),
                _ => None,
            },
            Entry::Tied(_) => None,
        })
        .collect();
    let [root] = roots.as_slice() else {
        panic!("the bar has no one edge through (0.6, 0, 1): {roots:?}");
    };

    for (id, kind, side) in [
        (tip_first, BooleanResultKind::OperandB, false),
        (bar_first, BooleanResultKind::OperandA, true),
    ] {
        assert_eq!(kind_of(&ev, id), kind);
        let body = body_of(&ev, id);
        let halves: Vec<StableName> = table(&ev, id)
            .iter()
            .flat_map(|(n, e)| {
                let keys: Vec<EntityKey> = match e {
                    Entry::Unique(r) => vec![r.key],
                    Entry::Tied(rs) => rs.iter().map(|r| r.key).collect(),
                };
                keys.into_iter().filter_map(move |k| match k {
                    EntityKey::Edge(k) if on_line(body, k) => Some(n.clone()),
                    _ => None,
                })
            })
            .collect();
        assert_eq!(
            halves.len(),
            2,
            "{kind:?}: the line is not split once at the apex: {halves:?}"
        );
        let want = if side {
            RoleSeg::FromA(NameRef::new(root.clone()))
        } else {
            RoleSeg::FromB(NameRef::new(root.clone()))
        };
        for h in &halves {
            assert_eq!(h.path.first(), Some(&want), "{kind:?}: {h:?}");
        }
    }
}

/// A name with its node erased, optionally with its sides exchanged:
/// `FromA` ↔ `FromB` and `Seam{a, b}` → `Seam{b, a}` at the head.
fn spelled(n: &StableName, swap: bool) -> String {
    let mut n = n.clone();
    n.node = RecipeNodeId(0);
    if swap && let Some(h) = n.path.first_mut() {
        *h = match h.clone() {
            RoleSeg::FromA(x) => RoleSeg::FromB(x),
            RoleSeg::FromB(x) => RoleSeg::FromA(x),
            RoleSeg::Seam { a, b } => RoleSeg::Seam { a: b, b: a },
            o => o,
        };
    }
    format!("{n:?}")
}

fn micro(x: f64) -> i64 {
    (x * 1e6).round() as i64
}

/// Every name of `id`'s table, beside the geometry it names (a
/// vertex's point, an edge's two end points, a face's boundary vertex
/// points, the body), or `None` when the result is the empty value.
fn named_geometry(ev: &Evaluation<f64>, id: RecipeNodeId, swap: bool) -> Option<BTreeSet<String>> {
    if let Some(e) = failure(ev, id) {
        panic!("the boolean refused: {e}");
    }
    match &ev.value(id).expect("the boolean evaluated").payload {
        ValuePayload::Boolean(BooleanValue::Body { .. }) => {}
        ValuePayload::Boolean(BooleanValue::Empty) => return None,
        other => panic!("expected a boolean value, got {}", other.kind_name()),
    }
    let body = body_of(ev, id);
    let at = |v| {
        let p = point(body, v);
        (micro(p.x), micro(p.y), micro(p.z))
    };
    let mut out = BTreeSet::new();
    for (n, e) in table(ev, id).iter() {
        let keys: Vec<EntityKey> = match e {
            Entry::Unique(r) => vec![r.key],
            Entry::Tied(rs) => rs.iter().map(|r| r.key).collect(),
        };
        for k in keys {
            let geo = match k {
                EntityKey::Vertex(v) => format!("{:?}", at(v)),
                EntityKey::Edge(e) => {
                    let mut s = ends(body, e).map(at);
                    s.sort_unstable();
                    format!("{s:?}")
                }
                // A face by the points of its boundary vertices — its
                // extent, which no other face of a sound body shares.
                EntityKey::Face(f) => {
                    let mut s: Vec<_> = face_vertices(body, f).into_iter().map(at).collect();
                    s.sort_unstable();
                    format!("face {s:?}")
                }
                EntityKey::Body => "body".to_string(),
            };
            out.insert(format!("{} @ {geo}", spelled(n, swap)));
        }
    }
    Some(out)
}

/// **Swapping a union's or an intersection's operands swaps the sides
/// of every name, and changes nothing else.**
///
/// Union and intersection are symmetric in their operands, so `y op x`
/// is the same body as `x op y` with A and B exchanged. Each fixture is
/// built once and combined in both orders; the result kinds must mirror
/// (`OperandA` ↔ `OperandB`), and every face, edge and vertex name of one
/// order, sides exchanged, must name the same geometry in the other.
/// The fixtures cover a zip, a nest, and a vertex touching an edge or a
/// face from inside, from outside and beside a zip, and an edge split
/// where its two faces share a second rim — the layouts where the
/// emitter's A and B sides take different key reads.
#[test]
fn swapping_the_operands_swaps_the_sides_of_every_name() {
    type Fixture = fn(ProfileDoc) -> (ProfileDoc, RecipeNodeId, RecipeNodeId);
    let fixtures: [(&str, Fixture); 6] = [
        ("seamed_touch", seamed_touch),
        ("bar_and_tip", bar_and_tip),
        ("nested", nested),
        ("ell_and_tip", ell_and_tip),
        ("face_touch", face_touch),
        ("edge_touch_outside", edge_touch_outside),
    ];
    let mirror = |k: BooleanResultKind| match k {
        BooleanResultKind::OperandA => BooleanResultKind::OperandB,
        BooleanResultKind::OperandB => BooleanResultKind::OperandA,
        k => k,
    };
    let mut compared = 0;
    for (what, fixture) in fixtures {
        for op in [BooleanOp::Union, BooleanOp::Intersect] {
            let doc = ProfileDoc::empty_derived("emit_vertex_keys_swap", Tol::witness());
            let (doc, x, y) = fixture(doc);
            let (doc, xy) = boolean(doc, op, x, y);
            let (doc, yx) = boolean(doc, op, y, x);
            let ev = run(&doc);
            let (fwd, back) = (
                named_geometry(&ev, xy, false),
                named_geometry(&ev, yx, true),
            );
            match (fwd, back) {
                (None, None) => {}
                (Some(fwd), Some(back)) => {
                    assert_eq!(
                        kind_of(&ev, yx),
                        mirror(kind_of(&ev, xy)),
                        "{what} {op:?}: the result kinds do not mirror"
                    );
                    let diff: Vec<_> = fwd.symmetric_difference(&back).collect();
                    assert!(
                        diff.is_empty(),
                        "{what} {op:?}: names that do not survive the swap:\n{diff:#?}"
                    );
                    compared += 1;
                }
                (f, b) => panic!(
                    "{what} {op:?}: one order is empty and the other is not ({}, {})",
                    f.is_some(),
                    b.is_some()
                ),
            }
        }
    }
    // Every fixture yields a body under union; a row that compared
    // nothing would pass vacuously.
    assert!(compared >= fixtures.len(), "only {compared} pairs compared");
}
