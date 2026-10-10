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
//! - an edge one operand's vertex touches at an interior point, in a
//!   result that is one operand's clone and in an assembly, each in
//!   both orders: the edge stays whole, named for its own operand's
//!   edge, and no vertex is minted at the touch;
//! - the operand-swap row: six fixtures, union and intersection, both
//!   orders; every face, edge and vertex name of `x op y` is the name
//!   the same geometry gets in `y op x` — a member's entity is named
//!   `From` that member's READ, which no order changes. It guards
//!   SYMMETRY only — a consistent relabel would pass it — so the rows
//!   above are what pin which operand each name belongs to.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use editor_core::ExtrudeSide;
use std::collections::BTreeSet;

use crate::corpus::body_of;
use crate::docm7_union_declare::{block, declared_union, failure, flush_pairs, run};
use crate::fixture::{
    ang, ename, ends, face_vertices, insert, len, on_frame, out, point, scl, table, vertex_of,
    vname,
};

use editor_core::{
    BooleanValue, CapEnd, EntityKey, EntityKind, Entry, Evaluation, NameRef, Node, ProfileDoc,
    ProfileVertexRef, Qualifier, RecipeNodeId, RoleSeg, StableName, ValuePayload,
};
use geom_core::Tol;
use topo::BooleanResultKind;

fn pv(doc: &editor_core::ProfileDoc, node: RecipeNodeId, vertex: u32) -> ProfileVertexRef {
    crate::fixture::vpiece(doc, node, 0, vertex as usize)
}

/// Which symmetric two-member boolean node a row builds.
#[derive(Clone, Copy, Debug)]
enum Op {
    Union,
    Intersect,
}

fn boolean(
    doc: ProfileDoc,
    op: Op,
    a: RecipeNodeId,
    b: RecipeNodeId,
) -> (ProfileDoc, RecipeNodeId) {
    let members = editor_core::Bodies::Spelled(vec![a.into(), b.into()]);
    let declare = Vec::new();
    insert(
        doc,
        match op {
            Op::Union => Node::Union { members, declare },
            Op::Intersect => Node::Intersect { members, declare },
        },
    )
}

fn union(doc: ProfileDoc, a: RecipeNodeId, b: RecipeNodeId) -> (ProfileDoc, RecipeNodeId) {
    boolean(doc, Op::Union, a, b)
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

fn assert_at(ev: &Evaluation<f64>, id: RecipeNodeId, n: &StableName, at: [f64; 3]) {
    let v = vertex_of(table(ev, id), "the touch vertex", n);
    let p = point(body_of(ev, id), v);
    let d = ((p.x - at[0]).powi(2) + (p.y - at[1]).powi(2) + (p.z - at[2]).powi(2)).sqrt();
    assert!(d < 1e-9, "{n:?} names a vertex at {p:?}, not at {at:?}");
}

pub(crate) fn nested(doc: ProfileDoc) -> (ProfileDoc, RecipeNodeId, RecipeNodeId) {
    let (doc, big) = block(doc, (0.0, 2.0), (0.0, 2.0), 0.0, 2.0);
    let (doc, small) = block(doc, (0.5, 1.0), (0.5, 1.0), 0.5, 0.5);
    (doc, big, small)
}

/// **The operand that survives names the result's corners, whichever
/// side it is.**
///
/// `small` sits strictly inside `big`: their union is `big` and their
/// intersection `small`, each a clone of one operand. In each of the
/// four orders every corner of the survivor is named `From` the
/// survivor's read of its own corner name, at that corner's point, and
/// no vertex is named from the absent operand. A B-clone key read against A's
/// table instead finds the other block's corner at the same slot and
/// publishes its name at a point where that corner is not.
#[test]
fn the_surviving_operand_names_the_corners_in_every_order() {
    let doc = ProfileDoc::empty_derived("emit_vertex_keys_nested", Tol::witness());
    let (doc, big, small) = nested(doc);
    let cases = [
        (
            Op::Union,
            small,
            big,
            big,
            BooleanResultKind::OperandB,
        ),
        (
            Op::Union,
            big,
            small,
            big,
            BooleanResultKind::OperandA,
        ),
        (
            Op::Intersect,
            big,
            small,
            small,
            BooleanResultKind::OperandB,
        ),
        (
            Op::Intersect,
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
        let survivor_read = out(&doc, survivor);
        for (end, h) in [(CapEnd::Start, z), (CapEnd::End, z + dz)] {
            for (i, (px, py)) in corners.into_iter().enumerate() {
                let own = NameRef::new(vname(
                    survivor,
                    RoleSeg::CapVertex(end, pv(&doc, survivor, i as u32)),
                ));
                let seg = RoleSeg::From {
                    read: survivor_read,
                    of: own,
                };
                assert_at(&ev, id, &vname(id, seg), [px, py, h]);
            }
        }
        let absent = table(&ev, id)
            .iter()
            .filter(|(n, _)| n.kind == EntityKind::Vertex)
            .filter(|(n, _)| {
                matches!(n.path.first(), Some(RoleSeg::From { read, .. }) if *read != survivor_read)
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
pub(crate) fn ell_and_tip(doc: ProfileDoc) -> (ProfileDoc, RecipeNodeId, RecipeNodeId) {
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
            profile: lp.into(),
            distance: len(1.0),
            side: ExtrudeSide::Along,
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
            profile: tp.into(),
            distance: len(0.5),
            side: ExtrudeSide::Along,
        },
    );
    (doc, ell, tip)
}

/// **An edge another operand's vertex touches stays whole, named for
/// its own operand's edge, in a result that is one operand's clone.**
///
/// The tip lies inside the ell, so the union is the ell. The apex
/// touches the reflex edge at one interior point, and the output stage
/// joins the vertex a touch would mint there (maximal edges): the
/// contact is the record (apex, reflex edge), and the edge is the
/// ell's reflex edge whole, `From` the ell's read of it whether the
/// result is B's clone or A's.
#[test]
fn an_edge_touched_by_the_other_operands_vertex_stays_whole_in_a_clone() {
    let doc = ProfileDoc::empty_derived("emit_vertex_keys_touch", Tol::witness());
    let (doc, ell, tip) = ell_and_tip(doc);
    let (doc, tip_first) = union(doc, tip, ell);
    let (doc, ell_first) = union(doc, ell, tip);
    let ev = run(&doc);

    let reflex = NameRef::new(ename(ell, RoleSeg::LateralEdge(pv(&doc, ell, 3))));
    let ell_read = out(&doc, ell);
    assert_eq!(kind_of(&ev, tip_first), BooleanResultKind::OperandB);
    assert_whole(
        &ev,
        tip_first,
        RoleSeg::From {
            read: ell_read,
            of: reflex.clone(),
        },
    );
    assert_eq!(kind_of(&ev, ell_first), BooleanResultKind::OperandA);
    assert_whole(
        &ev,
        ell_first,
        RoleSeg::From {
            read: ell_read,
            of: reflex,
        },
    );
}

/// The boolean `id` publishes `head` as one whole edge, and no vertex
/// that is not an operand's own.
fn assert_whole(ev: &Evaluation<f64>, id: RecipeNodeId, head: RoleSeg) {
    let t = table(ev, id);
    let name = StableName {
        kind: EntityKind::Edge,
        node: id,
        path: vec![head],
    };
    assert!(
        matches!(t.lookup(&name), Some(Entry::Unique(_))),
        "{name:?} is not one whole edge"
    );
    let minted: Vec<_> = t
        .iter()
        .filter(|(n, _)| n.kind == EntityKind::Vertex)
        .filter(|(n, _)| {
            !matches!(n.path.first(), Some(RoleSeg::From { .. }))
        })
        .map(|(n, _)| n.clone())
        .collect();
    assert!(minted.is_empty(), "a touch minted vertices: {minted:?}");
}

/// **The same touch, with both operands kept: B grafted in beside A.**
///
/// A wedge whose edge runs along (1, -1, 0) through the cube's corner
/// (1, 1, 0), its material strictly on the far side of x + y = 2, so
/// the two solids meet at that one point and the union keeps both.
/// The wedge's edge stays whole through the corner, named for the
/// wedge's edge whichever operand the wedge is.
#[test]
fn an_assembly_keeps_the_touched_edge_whole_in_either_order() {
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
            profile: p.into(),
            distance: len(2.0),
            side: ExtrudeSide::Along,
        },
    );
    let (doc, cube_first) = union(doc, cube, wedge);
    let (doc, wedge_first) = union(doc, wedge, cube);
    let ev = run(&doc);

    let ridge = NameRef::new(ename(wedge, RoleSeg::LateralEdge(pv(&doc, wedge, 0))));
    let wedge_read = out(&doc, wedge);
    assert_eq!(kind_of(&ev, cube_first), BooleanResultKind::Assembly);
    assert_whole(
        &ev,
        cube_first,
        RoleSeg::From {
            read: wedge_read,
            of: ridge.clone(),
        },
    );
    assert_eq!(kind_of(&ev, wedge_first), BooleanResultKind::Assembly);
    assert_whole(
        &ev,
        wedge_first,
        RoleSeg::From {
            read: wedge_read,
            of: ridge,
        },
    );
}

/// A tip whose apex touches the top face of a block at an interior
/// point, hanging down into the block.
pub(crate) fn face_touch(doc: ProfileDoc) -> (ProfileDoc, RecipeNodeId, RecipeNodeId) {
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
            profile: tp.into(),
            distance: len(0.4),
            side: ExtrudeSide::Along,
        },
    );
    (doc, bl, tip)
}

/// A tip whose apex touches a block's vertical edge from outside.
pub(crate) fn edge_touch_outside(doc: ProfileDoc) -> (ProfileDoc, RecipeNodeId, RecipeNodeId) {
    let r = std::f64::consts::FRAC_1_SQRT_2;
    let (doc, bl) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, tp) = on_frame(
        doc,
        [1.0, 1.0, 0.5],
        [r, -r, 0.0],
        [0.0, 0.0, 1.0],
        vec![vec![(0.0, 0.0), (0.4, -0.2), (0.4, 0.2)]],
    );
    // The frame normal points into the block; the tip extrudes
    // against it, away from the block.
    let (doc, tip) = insert(
        doc,
        Node::Extrude {
            profile: tp.into(),
            distance: len(0.5),
            side: ExtrudeSide::Against,
        },
    );
    (doc, bl, tip)
}

/// A SEAMED result that also carries an unzipped touch: an L-prism
/// whose convex corner (1, 1, 0) is touched by a tilted wedge's ridge,
/// the wedge crossing the L's long arm elsewhere — a zip in the body,
/// none at the touch.
pub(crate) fn seamed_touch(doc: ProfileDoc) -> (ProfileDoc, RecipeNodeId, RecipeNodeId) {
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
            profile: lp.into(),
            distance: len(1.0),
            side: ExtrudeSide::Along,
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
            profile: p.into(),
            distance: len(2.0),
            side: ExtrudeSide::Along,
        },
    );
    (doc, ell, wedge)
}

/// A bar — the declared union of two flush, x-offset placements of one
/// unit block, whose merged front and top faces meet along one joined
/// top-front edge — and a small prism inside it whose apex touches
/// that line at (0.6, 0, 1).
fn bar_and_tip(doc: ProfileDoc) -> (ProfileDoc, RecipeNodeId, RecipeNodeId) {
    let (doc, proto) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let place = |doc: ProfileDoc, dx: f64| {
        insert(
            doc,
            Node::transform(
                proto,
                editor_core::Step::Rigid {
                    translation: [len(dx), len(0.0), len(0.0)],
                    axis: [scl(0.0), scl(0.0), scl(1.0)],
                    angle: ang(0.0),
                },
            ),
        )
    };
    let (doc, m1) = place(doc, 0.0);
    let (doc, m2) = place(doc, 0.5);
    let pairs = flush_pairs(&doc, (m1, proto), (m2, proto));
    let (doc, bar) = declared_union(doc, &[m1, m2], pairs);
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
            profile: tp.into(),
            distance: len(0.3),
            side: ExtrudeSide::Along,
        },
    );
    (doc, bar, tip)
}

/// **An edge in a result that is B's clone descends to B's edge.**
///
/// The tip lies inside the bar, so `tip ∪ bar` is the bar — B's clone —
/// and its apex touches the bar's top-front line. That line is one
/// edge: the bar is a declared flush union, and its output stage
/// joined the line into one edge across both blocks. The touch mints
/// no vertex on it, so the line is named `From` the bar's read of the
/// bar's edge, exactly as `bar ∪ tip` (A's clone) names it.
#[test]
fn a_b_edge_in_a_b_clone_descends_to_its_b_edge() {
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

    let bar_read = out(&doc, bar);
    for (id, kind) in [
        (tip_first, BooleanResultKind::OperandB),
        (bar_first, BooleanResultKind::OperandA),
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
            1,
            "{kind:?}: the line is one edge: {halves:?}"
        );
        let want = RoleSeg::From {
            read: bar_read,
            of: NameRef::new(root.clone()),
        };
        assert_eq!(halves[0].path, vec![want], "{kind:?}: {halves:?}");
    }
}

/// A name with its node erased. An edge piece's ends are the node's
/// own vertex names, so they are spelled the same way and put back in
/// order.
fn spelled(n: &StableName) -> String {
    format!("{:?}", respelled(n))
}

fn respelled(n: &StableName) -> StableName {
    let mut n = n.clone();
    n.node = RecipeNodeId::new(0, 0);
    if let Some(RoleSeg::Fragment(Qualifier::Ends(ends))) = n.path.last_mut() {
        let mut out: Vec<StableName> = ends.iter().map(respelled).collect();
        out.sort();
        *ends = out;
    }
    n
}

fn micro(x: f64) -> i64 {
    (x * 1e6).round() as i64
}

/// Every name of `id`'s table, beside the geometry it names (a
/// vertex's point, an edge's two end points, a face's boundary vertex
/// points, the body), or `None` when the result is the empty value.
pub(crate) fn named_geometry(ev: &Evaluation<f64>, id: RecipeNodeId) -> Option<BTreeSet<String>> {
    if let Some(e) = failure(ev, id) {
        panic!("the boolean refused: {e}");
    }
    match &ev.value(id).expect("the boolean evaluated").payload {
        ValuePayload::Boolean(BooleanValue::Body { .. }) | ValuePayload::Body(_) => {}
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
            out.insert(format!("{} @ {geo}", spelled(n)));
        }
    }
    Some(out)
}

/// **Swapping a union's or an intersection's operands changes no
/// name.**
///
/// Union and intersection are symmetric in their operands, so `y op x`
/// is the same body as `x op y` with A and B exchanged. Each fixture is
/// built once and combined in both orders; the result kinds must mirror
/// (`OperandA` ↔ `OperandB`), and every face, edge and vertex name of one
/// order must name the same geometry in the other: a member's entity is
/// named `From` the member's read, whichever position it holds.
/// The fixtures cover a zip, a nest, and a vertex touching an edge or a
/// face from inside, from outside and beside a zip, and an edge split
/// where its two faces share a second rim — the layouts where the
/// emitter's A and B sides take different key reads.
#[test]
fn swapping_the_operands_changes_no_name() {
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
        for op in [Op::Union, Op::Intersect] {
            let doc = ProfileDoc::empty_derived("emit_vertex_keys_swap", Tol::witness());
            let (doc, x, y) = fixture(doc);
            let (doc, xy) = boolean(doc, op, x, y);
            let (doc, yx) = boolean(doc, op, y, x);
            let ev = run(&doc);
            let (fwd, back) = (
                named_geometry(&ev, xy),
                named_geometry(&ev, yx),
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
