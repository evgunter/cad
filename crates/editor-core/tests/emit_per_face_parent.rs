//! **A boolean's face is named for exactly the member faces it holds**
//! (names README N2, N3): the designers' worked example, its two edits,
//! and a subtract whose tool face lies over a face of `from`.
//!
//! Tiles `t1` (x 0 to 1) and `t2` (x 1 to 2) stand flush, tops at
//! z = 1, and a bar `g` (x 0.4 to 0.6) rises through their tops. The
//! finished top is two faces: L, west of the bar, holds `t1`'s top
//! alone; R, east of it, holds both tiles' tops. Each is qualified by
//! the bar's wall it borders, since `t1`'s top is held by both.
#![allow(clippy::expect_used, clippy::panic)]

use crate::blocks::{block, block_on, failure, run};
use crate::corpus::body_of;
use crate::fixture::{insert, point, table};
use crate::union_member_orders_decide_alike::orders;
use editor_core::{
    EntityKey, EntityKind, Entry, Evaluation, Node, ProfileDoc, Qualifier, RecipeNodeId, RoleSeg,
    StableName, VarId,
};
use geom_core::Tol;
use std::collections::BTreeSet;

/// The tiles and the bar, with the bar at x `g0` to `g0 + 0.2`, and a
/// flush tile `t0` (x −1 to 0) where `with_t0`.
struct Tiles {
    doc: ProfileDoc,
    t0: Option<RecipeNodeId>,
    t1: RecipeNodeId,
    t2: RecipeNodeId,
    g: RecipeNodeId,
}

fn tiles(label: &str, g0: f64, with_t0: bool) -> Tiles {
    let doc = ProfileDoc::empty_derived(label, Tol::witness());
    let (doc, t1) = block(doc, (0.0, 1.0), (0.0, 1.0), 1.0);
    let (doc, t2) = block_on(doc, t1, (1.0, 2.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, g) = block_on(doc, t1, (g0, g0 + 0.2), (-0.5, 1.5), 0.5, 1.5);
    let (doc, t0) = if with_t0 {
        let (doc, t0) = block_on(doc, t1, (-1.0, 0.0), (0.0, 1.0), 0.0, 1.0);
        (doc, Some(t0))
    } else {
        (doc, None)
    };
    Tiles { doc, t0, t1, t2, g }
}

impl Tiles {
    /// The read member `node` comes into the union through.
    fn read(&self, node: RecipeNodeId) -> VarId {
        self.doc.output(node, 0).expect("a block defines its body")
    }

    fn members(&self) -> Vec<RecipeNodeId> {
        self.t0
            .into_iter()
            .chain([self.t1, self.t2, self.g])
            .collect()
    }

    /// The union of the members in `order`, evaluated.
    fn union(&self, order: &[RecipeNodeId]) -> (Evaluation<f64>, RecipeNodeId) {
        let (doc, union) = crate::fixture::union_over(self.doc.clone(), order);
        let ev = run(&doc);
        if let Some(refusal) = failure(&ev, union) {
            panic!("the union of {order:?} names, got {refusal:?}");
        }
        (ev, union)
    }
}

/// The faces of `node`'s body lying in the plane z = 1 and facing up,
/// by their x span, each with its name.
fn tops(ev: &Evaluation<f64>, node: RecipeNodeId) -> Vec<((f64, f64), StableName)> {
    let body = body_of(ev, node);
    let mut out = Vec::new();
    for (name, entry) in table(ev, node).iter() {
        let Entry::Unique(e) = entry else { continue };
        let EntityKey::Face(f) = e.key else { continue };
        let up = topo::face_normal::face_outward_normal(&body, f).is_some_and(|n| n.vec().z > 0.5);
        let xs: Vec<(f64, f64)> = face_points(&body, f)
            .into_iter()
            .map(|p| (p.x, p.z))
            .collect();
        if up && xs.iter().all(|&(_, z)| (z - 1.0).abs() < 1e-9) {
            let lo = xs.iter().map(|&(x, _)| x).fold(f64::INFINITY, f64::min);
            let hi = xs.iter().map(|&(x, _)| x).fold(f64::NEG_INFINITY, f64::max);
            out.push(((lo, hi), name.clone()));
        }
    }
    out.sort_by(|a, b| a.0.0.total_cmp(&b.0.0));
    out
}

/// Whether two x spans agree to the fixture's rounding.
fn near(a: (f64, f64), b: (f64, f64)) -> bool {
    (a.0 - b.0).abs() < 1e-9 && (a.1 - b.1).abs() < 1e-9
}

/// The points of face `f`'s vertices.
pub(crate) fn face_points(body: &topo::Body<f64>, f: topo::FaceKey) -> Vec<geom_core::Point3<f64>> {
    body.half_edges()
        .filter(|(_, h)| body.get_loop(h.parent_loop).is_some_and(|l| l.face == f))
        .map(|(_, h)| point(body, h.start))
        .collect()
}

/// The reads `name`'s base cites: one for a member's face, each
/// constituent's for a `Merged` set.
fn cited_members(name: &StableName) -> Vec<VarId> {
    let read = |seg: &RoleSeg| match seg {
        RoleSeg::From { read, .. } => *read,
        other => panic!("a member face is cited through its read: {other:?}"),
    };
    match name.path.first() {
        Some(RoleSeg::Merged(set)) => set.iter().map(|c| read(&c.path[0])).collect(),
        Some(seg) => vec![read(seg)],
        None => panic!("an empty name"),
    }
}

/// The `Borders` walls `name` is qualified by, each by its member's
/// read; `None` for an unqualified name.
fn borders(name: &StableName) -> Option<Vec<VarId>> {
    match name.path.last() {
        Some(RoleSeg::Fragment(Qualifier::Borders(walls))) => {
            Some(walls.iter().flat_map(cited_members).collect())
        }
        _ => None,
    }
}

/// `name` said by tag with `union`'s tag replaced by `NODE`, so one
/// name over two unions compares.
fn modulo(name: &StableName, union: RecipeNodeId) -> String {
    name.to_string().replace(&union.to_string(), "NODE")
}

/// Every order of `members` names one table, each said modulo its union.
fn one_table(label: &str, fixture: &Tiles) {
    let members = fixture.members();
    let orders: Vec<Vec<RecipeNodeId>> = orders(members.len())
        .into_iter()
        .map(|o| o.into_iter().map(|i| members[i]).collect())
        .collect();
    let tables: Vec<BTreeSet<String>> = orders
        .iter()
        .map(|order| {
            let (ev, union) = fixture.union(order);
            crate::emit_union_borders::names_modulo(&ev, union)
        })
        .collect();
    crate::emit_union_borders::one_table(label, &orders, &tables);
}

/// **The worked example**: L is `t1`'s top bordering the bar's west
/// wall; R is `Merged` of both tiles' tops bordering its east wall.
#[test]
fn a_bar_across_one_tile_of_two_flush_names_each_top_for_what_it_holds() {
    let f = tiles("per-face-worked", 0.4, false);
    let (ev, union) = f.union(&f.members());
    let [(l_span, l), (r_span, r)] = &tops(&ev, union)[..] else {
        panic!("the top is two faces: {:?}", tops(&ev, union));
    };
    assert!(
        near(*l_span, (0.0, 0.4)) && near(*r_span, (0.6, 2.0)),
        "L and R: {l_span:?}, {r_span:?}"
    );
    assert_eq!(
        cited_members(l),
        vec![f.read(f.t1)],
        "L holds t1's top alone: {l}"
    );
    assert_eq!(
        borders(l),
        Some(vec![f.read(f.g)]),
        "L borders the bar: {l}"
    );
    let mut held = cited_members(r);
    held.sort();
    let mut tiles = vec![f.read(f.t1), f.read(f.t2)];
    tiles.sort();
    assert_eq!(held, tiles, "R holds both tiles' tops: {r}");
    assert_eq!(
        borders(r),
        Some(vec![f.read(f.g)]),
        "R borders the bar: {r}"
    );
    assert_ne!(
        l.path.last(),
        r.path.last(),
        "L and R border two walls of the bar"
    );
}

/// **The worked example names one table in every member order.**
#[test]
fn the_worked_example_names_one_table_in_every_member_order() {
    one_table("worked", &tiles("per-face-worked", 0.4, false));
}

/// **A flush tile beside `t1` renames only the faces gaining its
/// area**: L now holds `t0`'s top beside `t1`'s, and the bottom and the
/// two long side walls hold `t0`'s too. Every face that gains none of
/// `t0`'s area names alike, R and every edge bounding it among them, and
/// the one face that loses its area, `t1`'s wall against `t0`, is gone.
#[test]
fn a_flush_tile_beside_t1_renames_only_the_faces_gaining_its_area() {
    let before = tiles("per-face-t0", 0.4, false);
    let after = tiles("per-face-t0", 0.4, true);
    let (ev0, u0) = before.union(&before.members());
    let (ev1, u1) = after.union(&after.members());
    let (tops0, tops1) = (tops(&ev0, u0), tops(&ev1, u1));
    let ([(_, l0), (_, r0)], [(l_span, l1), (_, r1)]) = (&tops0[..], &tops1[..]) else {
        panic!("two tops each: {tops0:?}, {tops1:?}");
    };
    assert!(near(*l_span, (-1.0, 0.4)), "L reaches over t0: {l_span:?}");
    let t0 = after.t0.expect("the tile is in");
    let mut held = cited_members(l1);
    held.sort();
    let mut want = vec![after.read(t0), after.read(after.t1)];
    want.sort();
    assert_eq!(held, want, "L holds t0's and t1's tops: {l1}");
    assert_ne!(modulo(l0, u0), modulo(l1, u1), "L is renamed");
    assert_eq!(modulo(r0, u0), modulo(r1, u1), "R is not");
    // R's boundary, by name.
    let around = |ev: &Evaluation<f64>, union: RecipeNodeId, r: &StableName| {
        let body = body_of(ev, union);
        let t = table(ev, union);
        let Some(Entry::Unique(e)) = t.lookup(r) else {
            panic!("R names one face");
        };
        let EntityKey::Face(f) = e.key else {
            panic!("R names a face")
        };
        let edges: BTreeSet<topo::EdgeKey> = body
            .half_edges()
            .filter(|(_, h)| body.get_loop(h.parent_loop).is_some_and(|l| l.face == f))
            .map(|(_, h)| h.edge)
            .collect();
        t.iter()
            .filter(|(_, entry)| {
                let keys: Vec<EntityKey> = match entry {
                    Entry::Unique(e) => vec![e.key],
                    Entry::Tied(es) => es.iter().map(|e| e.key).collect(),
                };
                keys.iter()
                    .any(|k| matches!(k, EntityKey::Edge(k) if edges.contains(k)))
            })
            .map(|(name, _)| modulo(name, union))
            .collect::<BTreeSet<String>>()
    };
    assert_eq!(
        around(&ev0, u0, r0),
        around(&ev1, u1, r1),
        "R's edges are named alike"
    );
    // The face table's whole diff: each name only the union with `t0`
    // has holds `t0`'s area, or borders a face that does (a wall is
    // cited by the base of the face across it); each name only the
    // union without it has is one that gained `t0`'s area or a wall
    // that did, or is `t1`'s wall against `t0`.
    let faces = |ev: &Evaluation<f64>, union: RecipeNodeId| {
        table(ev, union)
            .iter()
            .filter(|(name, _)| name.kind == EntityKind::Face)
            .map(|(name, _)| (modulo(name, union), name.clone()))
            .collect::<std::collections::BTreeMap<String, StableName>>()
    };
    let (faces0, faces1) = (faces(&ev0, u0), faces(&ev1, u1));
    let gained: Vec<&StableName> = faces1
        .iter()
        .filter(|(said, _)| !faces0.contains_key(*said))
        .map(|(_, name)| name)
        .collect();
    let t0_read = after.read(t0);
    let moved = |name: &StableName| {
        let walls_moved = match name.path.last() {
            Some(RoleSeg::Fragment(Qualifier::Borders(walls))) => walls
                .iter()
                .any(|w| cited_members(w).contains(&t0_read)),
            _ => false,
        };
        cited_members(name).contains(&t0_read) || walls_moved
    };
    for name in faces1.values() {
        assert_eq!(
            moved(name),
            gained.contains(&name),
            "a face is renamed exactly where it or a wall it borders holds t0's area: {name}"
        );
    }
    let body0 = body_of(&ev0, u0);
    for (said, name) in faces0.iter().filter(|(said, _)| !faces1.contains_key(*said)) {
        let mut kept = cited_members(name);
        kept.sort();
        let mut was = kept.clone();
        was.push(t0_read);
        was.sort();
        let renamed = gained.iter().any(|now| {
            let mut held = cited_members(now);
            held.sort();
            held == was || (held == kept && moved(now))
        });
        let Some(Entry::Unique(e)) = table(&ev0, u0).lookup(name) else {
            panic!("{said} names one face");
        };
        let EntityKey::Face(f) = e.key else {
            panic!("{said} names a face")
        };
        let against_t0 = face_points(&body0, f).iter().all(|p| p.x.abs() < 1e-9);
        assert!(
            renamed || against_t0,
            "a face no longer named gained t0's area or lies against t0: {said}"
        );
    }
    one_table("with t0", &after);
}

/// **Moving the bar across x = 1 renames only the faces beside it**:
/// every face that is not a top and not the bar's names alike, and the
/// two tops swap which holds both tiles.
#[test]
fn moving_the_bar_across_the_tiles_joint_renames_only_the_faces_beside_it() {
    let west = tiles("per-face-move", 0.4, false);
    let east = tiles("per-face-move", 1.4, false);
    assert_eq!(
        (west.t1, west.t2),
        (east.t1, east.t2),
        "the tiles are one in both documents"
    );
    let (ev0, u0) = west.union(&west.members());
    let (ev1, u1) = east.union(&east.members());
    let [(_, l), (_, r)] = &tops(&ev1, u1)[..] else {
        panic!("the top is two faces: {:?}", tops(&ev1, u1));
    };
    assert_eq!(cited_members(l).len(), 2, "L now holds both tiles: {l}");
    assert_eq!(
        cited_members(r),
        vec![east.read(east.t2)],
        "R holds t2 alone: {r}"
    );
    let rest = |ev: &Evaluation<f64>, union: RecipeNodeId, g: VarId| {
        let tops: BTreeSet<StableName> = tops(ev, union).into_iter().map(|(_, n)| n).collect();
        table(ev, union)
            .iter()
            .filter(|(name, _)| {
                name.kind == EntityKind::Face
                    && !tops.contains(name)
                    && !cited_members(name).contains(&g)
            })
            .map(|(name, _)| modulo(name, union))
            .collect::<BTreeSet<String>>()
    };
    assert_eq!(
        rest(&ev0, u0, west.read(west.g)),
        rest(&ev1, u1, east.read(east.g)),
        "the faces away from the bar name alike"
    );
    one_table("moved", &east);
}

/// **A subtract's face holds the tool face lying over it**: a tool
/// standing under `from`'s bottom and past its x = 2 wall, touching the
/// bottom, has a top that, turned, lies over the bottom where the two
/// meet, so the bottom holds it and is `Merged` of the two (N2: one rule
/// for the union, the intersect and the subtract).
#[test]
fn a_subtracts_face_holds_the_tool_face_lying_over_it() {
    let doc = ProfileDoc::empty_derived("per-face-subtract", Tol::witness());
    let (doc, from) = block(doc, (0.0, 2.0), (0.0, 2.0), 1.0);
    let (doc, tool) = block_on(doc, from, (0.5, 2.5), (0.5, 1.5), -1.0, 1.0);
    let (doc, cut) = insert(
        doc,
        Node::Subtract {
            from: from.into(),
            tool: tool.into(),
        },
    );
    let ev = run(&doc);
    if let Some(refusal) = failure(&ev, cut) {
        panic!("the subtract names, got {refusal:?}");
    }
    let body = body_of(&ev, cut);
    let bottom: Vec<StableName> = table(&ev, cut)
        .iter()
        .filter_map(|(name, entry)| {
            let Entry::Unique(e) = entry else { return None };
            let EntityKey::Face(f) = e.key else {
                return None;
            };
            let down =
                topo::face_normal::face_outward_normal(&body, f).is_some_and(|n| n.vec().z < -0.5);
            let flat = face_points(&body, f).iter().all(|p| p.z.abs() < 1e-9);
            (down && flat).then(|| name.clone())
        })
        .collect();
    let [bottom] = &bottom[..] else {
        panic!("one bottom face: {bottom:?}");
    };
    let Some(RoleSeg::Merged(set)) = bottom.path.first() else {
        panic!("the bottom is Merged of from's bottom and the tool's top: {bottom}");
    };
    assert_eq!(set.len(), 2, "two faces: {bottom}");
    assert_eq!(
        bottom.path.len(),
        1,
        "held by one face, unqualified: {bottom}"
    );
}

/// The plane face `f` of `body` lies on: a point of it and its outward
/// unit normal.
fn plane_of(body: &topo::Body<f64>, f: topo::FaceKey) -> (geom_core::Point3<f64>, [f64; 3]) {
    let n = topo::face_normal::face_outward_normal(body, f)
        .expect("a brick's face has a normal")
        .vec();
    let p = face_points(body, f)[0];
    (p, [n.x, n.y, n.z])
}

/// **A subtract from an already-cut plate names each face for member
/// faces lying under it**: the plate less a slot, then less each of
/// three bricks whose walls meet the slot's own (a notch over the
/// slot's west half, one over its east half, a pocket past the plate's
/// west side). Each face's base cites only member faces on its own
/// plane, read off the members' bodies; a substitution row read through
/// the join's null scaffolding once made the slot's wall or floor cite a
/// tool's side wall perpendicular to it.
#[test]
fn a_second_subtract_cites_only_member_faces_lying_under_each_face() {
    for (what, x, y, z0) in [
        ("notch west", (0.0, 2.0), (1.0, 2.0), 0.5),
        ("notch east", (1.0, 3.0), (0.0, 1.0), 0.25),
        ("pocket", (-1.0, 2.0), (1.0, 2.0), 0.5),
    ] {
        let doc = ProfileDoc::empty_derived("per-face-second-cut", Tol::witness());
        let (doc, plate) = block(doc, (0.0, 3.0), (0.0, 3.0), 1.0);
        let (doc, slot) = block_on(doc, plate, (1.0, 2.0), (-1.0, 4.0), 0.5, 1.0);
        let (doc, cut) = insert(
            doc,
            Node::Subtract {
                from: plate.into(),
                tool: slot.into(),
            },
        );
        let (doc, tool) = block_on(doc, plate, x, y, z0, 1.5 - z0);
        let (doc, again) = insert(
            doc,
            Node::Subtract {
                from: cut.into(),
                tool: tool.into(),
            },
        );
        let ev = run(&doc);
        if let Some(refusal) = failure(&ev, again) {
            panic!("{what}: the second subtract names, got {refusal:?}");
        }
        let body = body_of(&ev, again);
        let operand = |read: VarId| {
            [cut, tool]
                .into_iter()
                .find(|&n| doc.output(n, 0) == Some(read))
                .unwrap_or_else(|| panic!("{what}: read {read:?} is an operand's"))
        };
        let mut cited = 0;
        for (name, entry) in table(&ev, again).iter() {
            let Entry::Unique(e) = entry else { continue };
            let EntityKey::Face(f) = e.key else { continue };
            let (p, n) = plane_of(&body, f);
            let constituents: Vec<&StableName> = match name.path.first() {
                Some(RoleSeg::Merged(set)) => set.iter().collect(),
                _ => vec![name],
            };
            for c in constituents {
                let Some(RoleSeg::From { read, of }) = c.path.first() else {
                    panic!("{what}: a face cites an operand face through its read: {name}");
                };
                let node = operand(*read);
                let Some(Entry::Unique(m)) = table(&ev, node).lookup(of.name()) else {
                    panic!("{what}: {of} names one face of its operand");
                };
                let EntityKey::Face(mf) = m.key else {
                    panic!("{what}: {of} names a face");
                };
                let (q, k) = plane_of(&body_of(&ev, node), mf);
                let dot = n[0] * k[0] + n[1] * k[1] + n[2] * k[2];
                let off = (q.x - p.x) * n[0] + (q.y - p.y) * n[1] + (q.z - p.z) * n[2];
                assert!(
                    (dot.abs() - 1.0).abs() < 1e-9 && off.abs() < 1e-9,
                    "{what}: {name} cites {of}, which lies off its plane"
                );
                cited += 1;
            }
        }
        assert!(cited > 10, "{what}: {cited} member faces cited");
    }
}

/// **A union's wrap edge is named as a piece of the member edge its
/// fold row names**: the corpus document `revolves_on_one_axis` unions
/// two stacked rings, whose bores and walls each glue across the joint
/// into one face holding a seam edge with that face on both sides. No
/// cover is read for such an edge (it lies between no two faces), so it
/// keeps the member edge its fold row names, and names a piece of that
/// edge: no `Seam`, which needs two faces.
#[test]
fn a_unions_wrap_edge_is_a_piece_of_its_fold_rows_member_edge() {
    let corpus = crate::corpus::revolves_on_one_axis::document();
    let union = corpus.result.expect("the document unions the rings");
    let ev = run(&corpus.doc);
    if let Some(refusal) = failure(&ev, union) {
        panic!("the union names, got {refusal:?}");
    }
    let body = body_of(&ev, union);
    let mut wraps = 0;
    for (name, entry) in table(&ev, union).iter() {
        let Entry::Unique(e) = entry else { continue };
        let EntityKey::Edge(k) = e.key else { continue };
        let edge = body.get_edge(k).expect("a named edge resolves");
        let face = |h| body.face_of_half_edge(h);
        if face(edge.he_plus) != face(edge.he_minus) {
            continue;
        }
        wraps += 1;
        assert!(
            matches!(name.path.first(), Some(RoleSeg::From { .. })),
            "a wrap edge is a piece of a member's edge: {name}"
        );
    }
    assert!(wraps >= 2, "the glued bore and wall each hold a wrap edge: {wraps}");
}
