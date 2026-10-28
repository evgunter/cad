//! **A union's names do not depend on the order of its members.**
//!
//! `Node::Union` folds its members pairwise, so each step's pair emitter
//! sees an A side and a B side, and the union's collapse rewrites every
//! fold-space name into member space. A name the collapse makes
//! order-independent — a `Seam` pair, canonicalized by name — must carry
//! a tail that is order-independent too: its pieces are told apart by
//! their ends, and a crossing of it names the side it enters along,
//! whichever way canonicalization writes the pair.
//!
//! The rows:
//! - two members in both orders, over the boolean suite's fixtures and a
//!   slab crossed by an inverted-U rib (a two-edge collinear seam chain
//!   on each rib cap): every face, edge and vertex name binds the same
//!   geometry in both orders;
//! - that rib's four ranked seam edges, pinned by name: each chain is
//!   ranked along `n(first) × n(second)`, the pair's outward normals in
//!   canonical (name) order;
//! - the same slab and rib with a cutter across every seam edge, in all
//!   six member orders: a seam line's pieces are a seam chain in some
//!   orders and a later step's cut of a whole seam in others, and no
//!   name binds a different entity in any two orders;
//! - two flush members and a bar across both, in both orders
//!   of the flush pair: the bar's cap pieces carry their `Borders`
//!   walls in member space, so each piece has one name;
//! - a block glued on a slab's top and a third block cutting its far
//!   end, in all six member orders, and with the block sunk into the
//!   slab or lifted clear: the edge along the glued block's bottom rim
//!   is the seam of the slab's top and its wall, not a piece of the rim.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::blocks::{block, block_on, run};
use crate::corpus::body_of;
use crate::emit_boolean_vertex_keys::{
    edge_touch_outside, ell_and_tip, face_touch, named_geometry, nested, seamed_touch,
};
use crate::emit_shared_rim_several::{construction, own_name};
use crate::fixture::{ends, fname, insert, len, member_face, on_frame, out, point, table};

use editor_core::{
    CapEnd, EntityKey, EntityKind, Entry, NameRef, Node, ProfileDoc, Qualifier, RecipeNodeId,
    RoleSeg, StableName,
};
use geom_core::Tol;

type Fixture = fn(ProfileDoc) -> (ProfileDoc, RecipeNodeId, RecipeNodeId);

/// Which two-operand boolean node a row builds.
#[derive(Clone, Copy)]
enum Op {
    Union,
    Subtract,
    Intersect,
}

impl Op {
    /// The node of this kind over `a` and `b`.
    fn node(
        self,
        a: impl Into<editor_core::Operand>,
        b: impl Into<editor_core::Operand>,
    ) -> editor_core::AuthoredNode {
        let (a, b) = (a.into(), b.into());
        match self {
            Self::Union => Node::Union {
                members: editor_core::Bodies::Spelled(vec![a.into(), b.into()]),
            },
            Self::Intersect => Node::Intersect {
                members: editor_core::Bodies::Spelled(vec![a.into(), b.into()]),
            },
            Self::Subtract => Node::Subtract {
                from: a.into(),
                tool: b.into(),
            },
        }
    }
}

/// A slab and an inverted-U rib standing in it: the rib's arms (x in
/// [0.5, 1] and [2, 2.5], y in [0.5, 1.5]) cross the slab's top at
/// z = 1, so each rib cap meets it in a two-edge collinear seam chain.
/// The rib is drawn on its own xy and placed against the slab on the
/// plane y = 1.5, extruded toward −y: the placement.
fn slab_rib(doc: ProfileDoc) -> (ProfileDoc, RecipeNodeId, RecipeNodeId) {
    let (doc, slab) = block(doc, (0.0, 3.0), (0.0, 2.0), 1.0);
    let (doc, p) = on_frame(
        doc,
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![
            (0.5, 0.5),
            (1.0, 0.5),
            (1.0, 1.5),
            (2.0, 1.5),
            (2.0, 0.5),
            (2.5, 0.5),
            (2.5, 2.0),
            (0.5, 2.0),
        ]],
    );
    let (doc, rib) = insert(
        doc,
        Node::Extrude {
            profile: p.into(),
            distance: len(1.0),
        },
    );
    let (doc, rib) = crate::fixture::against(doc, slab, rib, &rib_frame());
    (doc, slab, rib)
}

/// The rib's own frame in the slab's coordinates.
fn rib_frame() -> editor_core::Frame {
    crate::fixture::axes_at([0.0, 1.5, 0.0], [1.0, 0.0, 0.0], [0.0, 0.0, 1.0])
}

/// The union of `fixture`'s two bodies with its members in the order
/// `swap` picks, in a document of its own — both orders get the same
/// node ids, so their names compare directly.
fn union_of(
    fixture: Fixture,
    swap: bool,
) -> (
    ProfileDoc,
    editor_core::Evaluation<f64>,
    RecipeNodeId,
    [(RecipeNodeId, editor_core::VarId); 2],
) {
    let doc = ProfileDoc::empty_derived("emit_union_member_order", Tol::witness());
    let (doc, x, y) = fixture(doc);
    let members = if swap { vec![y, x] } else { vec![x, y] };
    let (doc, u) = crate::fixture::union_over(doc, &members);
    let (xr, yr) = (out(&doc, x), out(&doc, y));
    let ev = run(&doc);
    (doc, ev, u, [(x, xr), (y, yr)])
}

/// **Reordering a union's two members rebinds no name.**
///
/// Every name the union publishes — faces, edges, vertices, the body —
/// names the same geometry whichever member the fold reached first.
#[test]
fn reordering_a_unions_two_members_rebinds_no_name() {
    let fixtures: [(&str, Fixture); 6] = [
        ("slab_rib", slab_rib),
        ("seamed_touch", seamed_touch),
        ("nested", nested),
        ("ell_and_tip", ell_and_tip),
        ("face_touch", face_touch),
        ("edge_touch_outside", edge_touch_outside),
    ];
    for (what, fixture) in fixtures {
        let (_, ev_xy, u_xy, _) = union_of(fixture, false);
        let (_, ev_yx, u_yx, _) = union_of(fixture, true);
        let xy = named_geometry(&ev_xy, u_xy).expect("a union is never empty");
        let yx = named_geometry(&ev_yx, u_yx).expect("a union is never empty");
        let diff: Vec<_> = xy.symmetric_difference(&yx).collect();
        assert!(
            diff.is_empty(),
            "{what}: names bound to different geometry in the two member orders:\n{diff:#?}"
        );
    }
}

/// **A union's seam chain is named by its pieces' ends, one name per
/// piece in both member orders.**
///
/// The rib's caps meet the slab's top in two chains of two edges each,
/// x = 0.5..1.0 and x = 2.0..2.5. Each piece is the seam of the slab's
/// top and the rib's cap, in name order, with `Fragment(Ends)` over its
/// two end vertices; the two orders publish one name for each piece.
#[test]
fn a_unions_seam_chain_is_named_by_its_ends() {
    let mut named: std::collections::BTreeMap<(String, i64), StableName> =
        std::collections::BTreeMap::new();
    for swap in [false, true] {
        let (doc, ev, u, [(slab, slab_read), (rib, rib_read)]) = union_of(slab_rib, swap);
        let top = member_face(u, slab_read, fname(slab, RoleSeg::Cap(CapEnd::End)));
        let body = body_of(&ev, u);
        for cap in [CapEnd::End, CapEnd::Start] {
            let rib_cap = member_face(
                u,
                rib_read,
                own_name(&doc, rib, EntityKind::Face, vec![RoleSeg::Cap(cap)]),
            );
            let (a, b) = if top <= rib_cap {
                (top.clone(), rib_cap)
            } else {
                (rib_cap, top.clone())
            };
            let head = RoleSeg::Seam {
                a: NameRef::new(a),
                b: NameRef::new(b),
            };
            let mut found = 0;
            for (n, entry) in table(&ev, u).iter() {
                if n.path.first() != Some(&head) {
                    continue;
                }
                assert!(
                    matches!(
                        n.path.as_slice(),
                        [_, RoleSeg::Fragment(Qualifier::Ends(_))]
                    ),
                    "swap={swap}: a piece of the chain is named by its ends: {n:?}"
                );
                let Entry::Unique(r) = entry else {
                    panic!("swap={swap}: {n:?} is not uniquely named: {entry:?}");
                };
                let EntityKey::Edge(e) = r.key else {
                    panic!("swap={swap}: {n:?} names {:?}", r.key);
                };
                let [p, q] = ends(&body, e).map(|v| point(&body, v));
                let lo = (p.x.min(q.x) * 1e6).round() as i64;
                let key = (format!("{cap:?}"), lo);
                if let Some(was) = named.insert(key, n.clone()) {
                    assert_eq!(&was, n, "swap={swap}: {cap:?} at x = {lo}e-6");
                }
                found += 1;
            }
            assert_eq!(found, 2, "swap={swap}: {cap:?}'s chain is two pieces");
        }
    }
    assert_eq!(named.len(), 4, "{named:?}");
}

/// `slab_rib` and a cutter [0.7, 2.3] × [0.3, 1.7] × [0.95, 1.05] that
/// crosses every seam edge the slab and the rib make. Depending on
/// member order, a seam line is minted already cut (a seam chain) or
/// minted whole and cut by a later step (a descent chain of its pieces).
fn slab_rib_cutall(doc: ProfileDoc) -> (ProfileDoc, Vec<RecipeNodeId>) {
    let (doc, slab, rib) = slab_rib(doc);
    let (doc, cutter) = block_on(doc, slab, (0.7, 2.3), (0.3, 1.7), 0.95, 0.1);
    (doc, vec![slab, rib, cutter])
}

/// [`slab_rib_cutall`] and a block far from everything: in some orders
/// a seam passes through a step unchanged before a later one cuts it,
/// so the pieces' names descend through more than one wrapper.
fn slab_rib_cutall_far(doc: ProfileDoc) -> (ProfileDoc, Vec<RecipeNodeId>) {
    let (doc, mut m) = slab_rib_cutall(doc);
    let (doc, far) = block_on(doc, m[0], (5.0, 6.0), (0.0, 1.0), 0.0, 1.0);
    m.push(far);
    (doc, m)
}

/// [`slab_rib_cutall`] and a second small cutter across the first rib
/// arm's back seam: two later steps cut seam lines, in either order.
fn slab_rib_cutall_two(doc: ProfileDoc) -> (ProfileDoc, Vec<RecipeNodeId>) {
    let (doc, mut m) = slab_rib_cutall(doc);
    let (doc, second) = block_on(doc, m[0], (0.4, 0.6), (0.8, 1.0), 0.95, 0.1);
    m.push(second);
    (doc, m)
}

/// Every name of a union's table beside the geometry it binds, keyed by
/// name.
fn bindings(
    ev: &editor_core::Evaluation<f64>,
    u: RecipeNodeId,
) -> std::collections::BTreeMap<String, String> {
    let mut out = std::collections::BTreeMap::new();
    for line in named_geometry(ev, u).expect("a union is never empty") {
        let (n, g) = line.rsplit_once(" @ ").expect("a name @ geometry row");
        let prior = out.insert(n.to_string(), g.to_string());
        assert!(prior.is_none(), "{n} binds two entities");
    }
    out
}

/// Every order of `0..n`.
fn orders(n: usize) -> Vec<Vec<usize>> {
    if n == 0 {
        return vec![Vec::new()];
    }
    let mut out = Vec::new();
    for p in orders(n - 1) {
        for i in 0..=p.len() {
            let mut q = p.clone();
            q.insert(i, n - 1);
            out.push(q);
        }
    }
    out
}

/// **Every member order of a union whose seam lines are cut publishes
/// one table.**
///
/// A seam line's pieces are ranked by the seam-chain ranker in one
/// order and by the descent ranker in another — after one step or
/// several, through one wrapper or more — and a point where a cutter
/// crosses a seam is minted by whichever step met it. The union names
/// both from the finished body (N2), so every order publishes the same
/// names, each bound to the same geometry. Three fixtures, every order:
/// 6, 24 and 24.
#[test]
fn every_member_order_of_a_cut_seam_union_publishes_one_table() {
    type Members = fn(ProfileDoc) -> (ProfileDoc, Vec<RecipeNodeId>);
    let fixtures: [(&str, Members); 3] = [
        ("slab_rib_cutall", slab_rib_cutall),
        ("slab_rib_cutall_far", slab_rib_cutall_far),
        ("slab_rib_cutall_two", slab_rib_cutall_two),
    ];
    for (what, fixture) in fixtures {
        one_table(what, &fixture);
    }
}

/// The one table every member order of `fixture`'s union publishes,
/// each name beside the geometry it binds; a panic listing every name
/// two orders bind differently or only one order holds.
fn one_table(
    what: &str,
    fixture: &dyn Fn(ProfileDoc) -> (ProfileDoc, Vec<RecipeNodeId>),
) -> std::collections::BTreeMap<String, String> {
    let n = fixture(ProfileDoc::empty_derived("count", Tol::witness()))
        .1
        .len();
    let mut tables: Vec<_> = orders(n)
        .into_iter()
        .map(|p| {
            let doc = ProfileDoc::empty_derived("emit_union_member_order_cut", Tol::witness());
            let (doc, m) = fixture(doc);
            let (doc, u) = insert(
                doc,
                Node::Union {
                    members: editor_core::Bodies::Spelled(p.iter().map(|&i| m[i].into()).collect()),
                },
            );
            let ev = run(&doc);
            let t = bindings(&ev, u);
            (p, t)
        })
        .collect();
    let (p0, first) = tables.remove(0);
    let mut moved = Vec::new();
    for (p, t) in &tables {
        for (n, g) in t {
            match first.get(n) {
                Some(g0) if g0 == g => {}
                Some(g0) => moved.push(format!("{p:?}: {n} binds {g}, {p0:?} {g0}")),
                None => moved.push(format!("{p:?}: {n} is absent from {p0:?}")),
            }
        }
        for n in first.keys().filter(|n| !t.contains_key(*n)) {
            moved.push(format!("{p0:?}: {n} is absent from {p:?}"));
        }
    }
    assert!(
        moved.is_empty(),
        "{what}: {} names differ between orders:\n{moved:#?}",
        moved.len()
    );
    first
}

/// **A seam a split passed through, cut by a later boolean, is named by
/// its pieces' ends.**
///
/// `Union(slab, rib)`, split at x = 1.5, the lower half kept and then
/// cut by a small block across the rib arm's seam. The split renames
/// the faces it cut (`SplitFragment { parent }`) and keeps the seam
/// edge's name. Both the subtraction and the union name the result,
/// each piece of the seam the block cut by its two ends; no rule reads
/// the seam's faces.
#[test]
fn a_seam_passed_through_a_split_and_cut_later_is_named() {
    use editor_core::{Operand, SplitHalf};
    let doc = ProfileDoc::empty_derived("emit_union_member_order_split", Tol::witness());
    let (doc, slab, rib) = slab_rib(doc);
    let pair = |doc, op: Op, a: Operand, b: RecipeNodeId| insert(doc, op.node(a, b));
    let (doc, joined) = pair(doc, Op::Union, slab.into(), rib);
    let tool = crate::fixture::plane_on(&doc, slab, [1.5, 0.0, 0.0], [1.0, 0.0, 0.0]);
    let (doc, split) = insert(
        doc,
        Node::Split {
            target: joined.into(),
            tool,
        },
    );
    let below = Operand::output(split, SplitHalf::Below.port());
    let (doc, cut) = block_on(doc, slab, (0.7, 0.8), (0.3, 0.7), 0.95, 0.1);
    let (doc, minus) = pair(doc, Op::Subtract, below.clone(), cut);
    let (doc, plus) = pair(doc, Op::Union, below, cut);
    let ev = run(&doc);
    for (what, id) in [("subtract", minus), ("union", plus)] {
        if let Some(e) = crate::blocks::failure(&ev, id) {
            panic!("{what} refused: {e}");
        }
        let ranked = named_geometry(&ev, id)
            .expect("a body")
            .into_iter()
            .filter(|l| l.contains("Ends") && l.contains("Seam"))
            .count();
        assert!(
            ranked > 0,
            "{what}: no seam piece was named by its ends, so the row pins nothing"
        );
    }
}

/// **A seam between two placements of one prototype names.**
///
/// The U-rib and a copy of it turned a quarter about x, both placements
/// of the one rib construction: their seam chains' pieces are named by
/// their ends. Every pair boolean of the two, both union orders
/// included, names its result, as does a union node over them.
#[test]
fn a_seam_between_two_placements_of_one_prototype_is_named() {
    let doc = ProfileDoc::empty_derived("emit_union_member_order_same", Tol::witness());
    let (doc, slab, rib) = slab_rib(doc);
    let prototype = crate::emit_shared_rim_several::construction(&doc, rib);
    // The rib's own frame turned a quarter about x, then slid by
    // (0.1, 1.5, 0.3).
    let turned_frame = crate::fixture::axes_at([0.1, 1.5, 1.8], [1.0, 0.0, 0.0], [0.0, -1.0, 0.0]);
    let (doc, turned) = crate::fixture::against(doc, slab, prototype, &turned_frame);
    let mut doc = doc;
    let mut ids = Vec::new();
    for (what, op, a, b) in [
        ("union", Op::Union, rib, turned),
        ("union, swapped", Op::Union, turned, rib),
        ("subtract", Op::Subtract, rib, turned),
        ("intersect", Op::Intersect, rib, turned),
    ] {
        let (d, id) = insert(doc, op.node(a, b));
        doc = d;
        ids.push((what, id));
    }
    let (doc, node) = insert(
        doc,
        Node::Union {
            members: editor_core::Bodies::Spelled(vec![rib.into(), turned.into()]),
        },
    );
    ids.push(("union node", node));
    let ev = run(&doc);
    for (what, id) in ids {
        if let Some(e) = crate::blocks::failure(&ev, id) {
            panic!("{what} refused: {e}");
        }
        let seams = named_geometry(&ev, id)
            .expect("a body")
            .into_iter()
            .filter(|l| l.contains("Seam"))
            .count();
        assert!(
            seams > 0,
            "{what}: no seam was named, so the row pins nothing"
        );
    }
}

/// **A fragment's `Borders` walls are one spelling whatever the member
/// order** (the `abys` witness of
/// `name-ordered-positions-in-a-path-have-no-single-home`).
///
/// `a` and `b` overlap in x and are flush; the bar `y` crosses
/// both, so `y`'s start cap is cut into two pieces told apart by the
/// walls of `a` and `b` each borders. The pair emitter writes the walls
/// in the FOLD's space, where the first member is the A side; the
/// union names them again in member space. In `[a, b, y]` and
/// `[b, a, y]` every face piece carries the same name and binds the
/// same face.
#[test]
fn a_fragments_borders_walls_are_one_spelling_in_every_member_order() {
    let tables: Vec<std::collections::BTreeMap<String, String>> = [[0usize, 1, 2], [1, 0, 2]]
        .into_iter()
        .map(|p| {
            let doc = ProfileDoc::empty_derived("emit_union_member_order_abys", Tol::witness());
            let (doc, a) = block(doc, (0.0, 1.0), (0.0, 1.0), 1.0);
            let (doc, b) = block_on(doc, a, (0.5, 1.5), (0.0, 1.0), 0.0, 1.0);
            let (doc, y) = block_on(doc, a, (-1.0, 2.0), (0.3, 0.4), 0.5, 3.5);
            let m = [a, b, y];
            let members: Vec<RecipeNodeId> = p.iter().map(|&i| m[i]).collect();
            let (doc, u) = crate::fixture::union_over(doc, &members);
            let ev = run(&doc);
            bindings(&ev, u)
                .into_iter()
                .filter(|(n, _)| n.contains("Borders"))
                .collect()
        })
        .collect();
    assert!(
        !tables[0].is_empty(),
        "no fragment carries a Borders, so the row pins nothing"
    );
    assert_eq!(
        tables[0], tables[1],
        "the fragments' names depend on the member order"
    );
}

/// What `glued_rim(lift)` names, in member order `[a, b, c]`: the edge
/// along `b`'s y = 0.25 bottom rim from x = 0.5 to the x = 1.2 wall of
/// `c`, and the vertex where that wall cuts it, each with its name's
/// spelling as [`bindings`] writes it; and the member faces the names
/// are read against (`a`'s top, `b`'s y = 0.25 wall, `c`'s x = 1.2
/// wall) and `b`'s read.
struct GluedRim {
    edge: (StableName, String),
    vertex: (StableName, String),
    faces: [StableName; 3],
    b_read: editor_core::VarId,
}

fn glued_rim_names(lift: f64) -> GluedRim {
    let doc = ProfileDoc::empty_derived("emit_union_member_order_glued", Tol::witness());
    let (doc, m) = crate::fixture::glued_rim(lift)(doc);
    let [a, b, c] = [m[0], m[1], m[2]];
    let faces = [
        member_face_of(&doc, a, RoleSeg::Cap(CapEnd::End)),
        member_face_of(
            &doc,
            b,
            crate::fixture::wall(&doc, construction(&doc, b), 0),
        ),
        member_face_of(
            &doc,
            c,
            crate::fixture::wall(&doc, construction(&doc, c), 3),
        ),
    ];
    let b_read = out(&doc, b);
    let (doc, u) = crate::fixture::union_over(doc, &m);
    let faces = faces.map(|(read, of)| member_face(u, read, of));
    let ev = run(&doc);
    let body = body_of(&ev, u);
    let z = 1.0 + lift.max(0.0);
    let near = |p: geom_core::Point3<f64>, (x, y): (f64, f64)| {
        (p.x - x).abs() < 1e-9 && (p.y - y).abs() < 1e-9 && (p.z - z).abs() < 1e-9
    };
    let named = |hit: &dyn Fn(EntityKey) -> bool| {
        let found: Vec<StableName> = table(&ev, u)
            .iter()
            .filter(|(_, e)| matches!(e, Entry::Unique(r) if hit(r.key)))
            .map(|(n, _)| n.clone())
            .collect();
        let [n] = found.as_slice() else {
            panic!(
                "lift {lift}: {} names hold the entity: {found:?}",
                found.len()
            );
        };
        n.clone()
    };
    let edge = named(&|k| {
        let EntityKey::Edge(e) = k else { return false };
        let [p, q] = ends(&body, e).map(|v| point(&body, v));
        (near(p, (0.5, 0.25)) && near(q, (1.2, 0.25)))
            || (near(q, (0.5, 0.25)) && near(p, (1.2, 0.25)))
    });
    let vertex =
        named(&|k| matches!(k, EntityKey::Vertex(v) if near(point(&body, v), (1.2, 0.25))));
    let table = bindings(&ev, u);
    let spelled = |geo: &str| {
        let hits: Vec<&String> = table
            .iter()
            .filter(|(_, g)| *g == geo)
            .map(|(n, _)| n)
            .collect();
        let [n] = hits.as_slice() else {
            panic!("lift {lift}: {} names bind {geo}: {hits:?}", hits.len());
        };
        (*n).clone()
    };
    let at = |x: f64, y: f64| {
        let m = |v: f64| (v * 1e6).round() as i64;
        (m(x), m(y), m(z))
    };
    let mut e = [at(0.5, 0.25), at(1.2, 0.25)];
    e.sort_unstable();
    GluedRim {
        edge: (edge, spelled(&format!("{e:?}"))),
        vertex: (vertex, spelled(&format!("{:?}", at(1.2, 0.25)))),
        faces,
        b_read,
    }
}

/// `node`'s face `seg`, read at `node`'s output: the member read and
/// the name a union cites it under through that read.
fn member_face_of(
    doc: &ProfileDoc,
    node: RecipeNodeId,
    seg: RoleSeg,
) -> (editor_core::VarId, StableName) {
    (
        out(doc, node),
        own_name(doc, node, EntityKind::Face, vec![seg]),
    )
}

/// `spelled` with every mint id's digest erased, so ids compare by
/// ordinal alone.
fn by_ordinal(spelled: &str) -> String {
    let mut out = String::with_capacity(spelled.len());
    let mut rest = spelled;
    while let Some(i) = rest.find("digest: ") {
        let (head, tail) = rest.split_at(i + "digest: ".len());
        out.push_str(head);
        out.push('_');
        rest = tail.trim_start_matches(|c: char| c.is_ascii_digit());
    }
    out.push_str(rest);
    out
}

/// The `Seam` head over two member faces, in name order.
fn seam_of(x: &StableName, y: &StableName) -> RoleSeg {
    let (a, b) = if x <= y { (x, y) } else { (y, x) };
    RoleSeg::Seam {
        a: NameRef::new(a.clone()),
        b: NameRef::new(b.clone()),
    }
}

/// **An edge whose member face was glued away is the seam of the faces
/// it separates, in every member order** (the names README's flush
/// bullet, "lies within").
///
/// `b` stands on `a`, so `b`'s bottom face is glued to `a`'s top; the
/// union's edge along `b`'s y = 0.25 bottom rim, up to `c`'s x = 1.2
/// wall, lies on that rim but its faces are `a`'s top and `b`'s wall,
/// and `a`'s top descends from neither of the rim's faces. It is the
/// seam of `a`'s top and `b`'s wall; the vertex where `c`'s wall cuts
/// it is the run of the three seams through it (a.top/b.wall,
/// a.top/c.wall, b.wall/c.wall), none with a sense. All six member
/// orders publish one table.
///
/// Sunk 0.1 into `a`, `b`'s bottom is inside `a` and the same edge and
/// vertex carry the same names, node ids compared by mint ordinal
/// ([`by_ordinal`]): each fixture is built in a fresh document by the
/// same insertions, so each node has the same ordinal in all three,
/// while its digest hashes its content, `b`'s height among it. Lifted 0.1 clear, the
/// edge (now at z = 1.1) is a piece of `b`'s bottom rim, `c`'s wall
/// crosses that rim at the vertex, and the glued fixture's names are
/// gone.
#[test]
fn an_edge_whose_member_face_was_glued_away_is_a_seam_in_every_member_order() {
    let mut spelled = Vec::new();
    for (what, lift) in [("glued", 0.0), ("sunk", -0.1), ("lifted", 0.1)] {
        let all = one_table(what, &crate::fixture::glued_rim(lift));
        let r = glued_rim_names(lift);
        for (n, s) in [&r.edge, &r.vertex] {
            assert!(
                all.contains_key(s),
                "{what}: {n:?} is not the every-order table's {s}"
            );
        }
        let [top, wall, cut] = &r.faces;
        if lift > 0.0 {
            assert!(
                matches!(
                    r.edge.0.path.as_slice(),
                    [RoleSeg::From { read, .. }, RoleSeg::Fragment(Qualifier::Ends(_))]
                        if *read == r.b_read
                ),
                "{what}: the edge is a piece of b's rim: {:?}",
                r.edge.0
            );
            assert!(
                matches!(
                    r.vertex.0.path.as_slice(),
                    [RoleSeg::Crossing { edge, face, .. }]
                        if edge.path.first() == r.edge.0.path.first() && **face == *cut
                ),
                "{what}: the cut is b's rim crossing c's wall: {:?}",
                r.vertex.0
            );
        } else {
            assert_eq!(
                r.edge.0.path.first(),
                Some(&seam_of(top, wall)),
                "{what}: the edge is the seam of a's top and b's wall: {:?}",
                r.edge.0
            );
            let [RoleSeg::EdgeCrossing(run)] = r.vertex.0.path.as_slice() else {
                panic!("{what}: the cut is a run: {:?}", r.vertex.0);
            };
            let heads: std::collections::BTreeSet<_> = run
                .iter()
                .map(|l| {
                    assert!(
                        matches!(&l.sense, editor_core::Sense::Through { entering } if entering.len() == 1),
                        "{what}: a seam carries the side it enters along: {l:?}"
                    );
                    l.line.path.first().cloned()
                })
                .collect();
            let want: std::collections::BTreeSet<_> =
                [seam_of(top, wall), seam_of(top, cut), seam_of(wall, cut)]
                    .into_iter()
                    .map(Some)
                    .collect();
            assert_eq!(heads, want, "{what}: the run is the three seams");
            assert_eq!(run.len(), 3, "{what}: {run:?}");
        }
        spelled.push((what, by_ordinal(&r.edge.1), by_ordinal(&r.vertex.1)));
    }
    let [(_, ge, gv), (_, se, sv), (_, le, lv)] = spelled.as_slice() else {
        panic!("three variants")
    };
    assert_eq!(
        (ge, gv),
        (se, sv),
        "sinking b renames neither the edge nor the cut"
    );
    assert_ne!(ge, le, "lifted clear, the edge is no longer the seam");
    assert_ne!(gv, lv, "lifted clear, the cut is no longer the seams' run");
}
