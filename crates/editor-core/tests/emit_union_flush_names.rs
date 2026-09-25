//! **What a union's flush and parent rules may never publish.**
//!
//! The published table names a flush stretch, a member's corner, its
//! faces and a seam's sides from the finished body (`emit_union::Flush`,
//! `emit_union::name_by_parents`). These rows hold those rules to the
//! body they read:
//! - a vertex named for a member vertex sits at it and borders a face of
//!   that member it lies on, so a coincident vertex of another shell
//!   never takes its name;
//! - a face is its parent, or its parent and one `SideOf`, and a member
//!   face a merge lists is published by no face of its own;
//! - a seam edge's sides are the parents of the faces it lies between;
//! - a document whose finished body depends on member order (a vertex a
//!   declared merge left behind) still publishes, with no two rows
//!   sharing a name.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::collections::BTreeSet;

use crate::corpus::body_of;
use crate::docm7_union_declare::{failure, run};
use crate::emit_shared_rim_several::{Bx, document, permutations};
use crate::emit_union_rim_piece_ranks::{Case, cases, runs};
use crate::fixture::{face_vertices, insert, table};

use editor_core::{
    EntityKey, EntityKind, Entry, Evaluation, Node, NodeErrorKind, RecipeNodeId, RoleSeg,
    StableName,
};

/// The member faces a published face name cites: its own, or each of a
/// `Merged` set's, through any `Fragment`.
fn member_faces(name: &StableName, out: &mut BTreeSet<(RecipeNodeId, StableName)>) {
    match name.path.first() {
        Some(RoleSeg::FromMember { member, of }) if of.kind == EntityKind::Face => {
            out.insert((*member, (**of).clone()));
        }
        Some(RoleSeg::Merged(set)) => set.iter().for_each(|c| member_faces(c, out)),
        _ => {}
    }
}

/// The faces around vertex `v` of `body`.
fn faces_at(body: &topo::Body<f64>, v: topo::VertexKey) -> BTreeSet<topo::FaceKey> {
    body.half_edges()
        .filter(|(_, he)| he.start == v)
        .flat_map(|(h, he)| {
            let mate = body.mate(h).unwrap();
            [
                he.parent_loop,
                body.get_half_edge(mate).unwrap().parent_loop,
            ]
        })
        .map(|l| body.get_loop(l).unwrap().face)
        .collect()
}

/// The two faces edge `e` of `body` lies between.
fn faces_of(body: &topo::Body<f64>, e: topo::EdgeKey) -> [topo::FaceKey; 2] {
    let edge = body.get_edge(e).unwrap();
    [edge.he_plus, edge.he_minus].map(|he| {
        body.get_loop(body.get_half_edge(he).unwrap().parent_loop)
            .unwrap()
            .face
    })
}

fn point(body: &topo::Body<f64>, v: topo::VertexKey) -> geom_core::Point3<f64> {
    *body.get_point(body.get_vertex(v).unwrap().point).unwrap()
}

/// **Every vertex a union names for a member vertex is that vertex**:
/// at its point, and bordering a face of the member that the member
/// vertex lies on. Returns how many such vertices were checked.
fn member_vertices_hold(ev: &Evaluation<f64>, union: RecipeNodeId, at: &str) -> usize {
    let body = body_of(ev, union);
    let t = table(ev, union);
    let mut checked = 0;
    for (name, entry) in t.iter() {
        let (Entry::Unique(e), [RoleSeg::FromMember { member, of }]) =
            (entry, name.path.as_slice())
        else {
            continue;
        };
        let (EntityKey::Vertex(v), EntityKind::Vertex) = (e.key, of.kind) else {
            continue;
        };
        let (mbody, mtable) = (body_of(ev, *member), table(ev, *member));
        let Some(Entry::Unique(w)) = mtable.lookup(of) else {
            panic!("{at}: {name:?} names no vertex of its member")
        };
        let EntityKey::Vertex(w) = w.key else {
            panic!("{at}: {name:?} is not a member vertex")
        };
        assert!(
            (point(body, v) - point(mbody, w)).norm() < 1e-9,
            "{at}: {name:?} is not at its member vertex"
        );
        let mut cited = BTreeSet::new();
        for f in faces_at(body, v) {
            if let Some(fname) = t.name_of(&editor_core::EntityRef {
                body: 0,
                key: EntityKey::Face(f),
            }) {
                member_faces(fname, &mut cited);
            }
        }
        let borders = faces_at(mbody, w).into_iter().any(|g| {
            mtable
                .name_of(&editor_core::EntityRef {
                    body: 0,
                    key: EntityKey::Face(g),
                })
                .is_some_and(|g| cited.contains(&(*member, g.clone())))
        });
        assert!(
            borders,
            "{at}: {name:?} borders no face of its member that the member vertex lies on"
        );
        checked += 1;
    }
    checked
}

/// A published face's parent: its name without its trailing `Fragment`s.
fn parent_of(face: &StableName) -> StableName {
    let mut f = face.clone();
    while f.path.len() > 1 && matches!(f.path.last(), Some(RoleSeg::Fragment(_))) {
        f.path.pop();
    }
    f
}

/// **Every published seam edge's sides are exactly the parents of the
/// two faces it lies between** (N3), in name order. Returns how many
/// seam edges were checked.
fn seam_sides_hold(ev: &Evaluation<f64>, union: RecipeNodeId, at: &str) -> usize {
    let body = body_of(ev, union);
    let t = table(ev, union);
    let mut checked = 0;
    for (name, entry) in t.iter() {
        let (Entry::Unique(e), EntityKind::Edge, Some(RoleSeg::Seam { a, b })) =
            (entry, name.kind, name.path.first())
        else {
            continue;
        };
        let EntityKey::Edge(k) = e.key else { continue };
        let faces: Vec<StableName> = faces_of(body, k)
            .iter()
            .map(|&f| {
                t.name_of(&editor_core::EntityRef {
                    body: 0,
                    key: EntityKey::Face(f),
                })
                .cloned()
                .unwrap_or_else(|| panic!("{at}: a face beside {name:?} is unnamed"))
            })
            .collect();
        let mut parents: Vec<StableName> = faces.iter().map(parent_of).collect();
        parents.sort();
        assert_eq!(
            vec![(**a).clone(), (**b).clone()],
            parents,
            "{at}: {name:?}'s sides are not the parents of the faces it lies between: {faces:?}"
        );
        checked += 1;
    }
    checked
}

/// The member faces a parent name lists: its one member face, or each
/// of a flat `Merged` set's, and `None` for any other shape.
fn parent_members(parent: &StableName) -> Option<BTreeSet<StableName>> {
    let one = |n: &StableName| matches!(n.path.as_slice(), [RoleSeg::FromMember { of, .. }] if of.kind == EntityKind::Face);
    match parent.path.as_slice() {
        [RoleSeg::FromMember { .. }] if one(parent) => Some(BTreeSet::from([parent.clone()])),
        [RoleSeg::Merged(set)] if set.len() >= 2 && set.iter().all(one) => {
            Some(set.iter().cloned().collect())
        }
        _ => None,
    }
}

/// **Every published face is its parent, or its parent and one
/// `SideOf`** (N2, N3):
/// - a face's name is a member face, a flat `Merged` of member faces, or
///   one of those followed by exactly one `Fragment(SideOf)`;
/// - a parent is published bare only when it is held as one face, and
///   as fragments only when held as several;
/// - no member face is listed by two parents, so a constituent a merge
///   lists is never published by a face of its own;
/// - a `SideOf` partner is the parent of a published face.
///
/// Returns how many faces were checked.
fn faces_are_parents(ev: &Evaluation<f64>, union: RecipeNodeId, at: &str) -> usize {
    let t = table(ev, union);
    let faces: Vec<StableName> = t
        .iter()
        .filter(|(name, _)| name.kind == EntityKind::Face)
        .map(|(name, _)| name.clone())
        .collect();
    let parents: BTreeSet<StableName> = faces.iter().map(parent_of).collect();
    let mut listed: std::collections::BTreeMap<StableName, StableName> = Default::default();
    for p in &parents {
        let members = parent_members(p)
            .unwrap_or_else(|| panic!("{at}: {p:?} is not a member face or a flat merge of them"));
        for m in members {
            if let Some(other) = listed.insert(m.clone(), p.clone()) {
                panic!("{at}: {m:?} is listed by two parents, {other:?} and {p:?}");
            }
        }
    }
    for face in &faces {
        let parent = parent_of(face);
        let tail = &face.path[parent.path.len()..];
        let pieces = faces.iter().filter(|f| parent_of(f) == parent).count();
        match tail {
            [] => assert_eq!(
                pieces, 1,
                "{at}: {face:?} is bare but held as {pieces} faces"
            ),
            [RoleSeg::Fragment(editor_core::Qualifier::SideOf(v))] => {
                assert!(
                    pieces > 1,
                    "{at}: {face:?} is a fragment of a parent held whole"
                );
                for (partner, _) in v {
                    assert!(
                        parents.contains(partner),
                        "{at}: {face:?} cites {partner:?}, which is no published face's parent"
                    );
                }
            }
            _ => panic!("{at}: {face:?} is not its parent and one SideOf"),
        }
    }
    faces.len()
}

const A: Bx = ((0.0, 1.0), (0.0, 1.0), (0.0, 1.0));
const B: Bx = ((0.5, 1.5), (0.0, 1.0), (0.0, 1.0));
/// A slab 2e-3 wide centred on `b`'s x = 0.5 end, from z = 0.5 up: the
/// ZIP row's 2e-5 slab refuses at the 1e-6 tolerance row, and the width
/// is not what leaves the vertex.
const NEAR: Bx = ((0.499, 0.501), (-1.0, 2.0), (0.5, 2.0));

/// The ZIP row's shape
/// (`work/zip/a-declared-merge-leaves-a-collinear-valence-two-vertex-an-earlier-cut-made.md`):
/// `a`, `b` declared flush and a slab over `b`'s end.
fn near_slab() -> Case {
    Case::flat("near", vec![A, B, NEAR], vec![0, 1, 2], vec![(0, 1)])
}

/// **The corpus, its fixtures and the ZIP document publish no seam side
/// but the parents of the faces beside it, no face but a parent or a
/// fragment of one, and no member vertex another shell holds.**
#[test]
fn a_union_cites_only_what_the_finished_body_holds() {
    let (mut seams, mut faces, mut vertices) = (0, 0, 0);
    for case in cases().into_iter().chain([near_slab()]) {
        runs(&case, |at, ev, _, unions| {
            for &(tag, union) in unions {
                if failure(ev, union).is_none() {
                    let at = format!("{} {tag} {at}", case.label);
                    seams += seam_sides_hold(ev, union, &at);
                    faces += faces_are_parents(ev, union, &at);
                    vertices += member_vertices_hold(ev, union, &at);
                }
            }
        });
    }
    assert!(seams > 1000, "only {seams} seam edges checked");
    assert!(faces > 1000, "only {faces} faces checked");
    assert!(vertices > 1000, "only {vertices} member vertices checked");
}

/// The union-space name of member `m`'s face whose vertices all lie on
/// y = 0.
fn wall_at_y0(ev: &Evaluation<f64>, union: RecipeNodeId, m: RecipeNodeId) -> StableName {
    let (body, t) = (body_of(ev, m), table(ev, m));
    let on = t.iter().find_map(|(name, entry)| {
        let Entry::Unique(e) = entry else { return None };
        let EntityKey::Face(f) = e.key else {
            return None;
        };
        face_vertices(body, f)
            .into_iter()
            .all(|v| point(body, v).y.abs() < 1e-9)
            .then(|| name.clone())
    });
    StableName {
        kind: EntityKind::Face,
        node: union,
        path: vec![RoleSeg::FromMember {
            member: m,
            of: editor_core::NameRef::new(on.expect("a face at y = 0")),
        }],
    }
}

/// **A face cut and merged in one step publishes no piece under a
/// constituent's name** (N3). `fam012`: `a` = x 0..1 and `b` = x
/// 0.5..1.5, declared flush, and a slab `g` at x 0.3..0.4 through `a`'s
/// y = 0 wall. In `[b, g, a]` one step cuts `a`'s wall and merges its
/// right piece with `b`'s; in `[a, b, g]` the wall merges first and `g`
/// cuts the merge. Either way the wall is two faces, both fragments of
/// the one merge, under the same two names, and `a`'s wall is published
/// by no face.
#[test]
fn a_face_cut_and_merged_in_one_step_publishes_no_constituent() {
    let case = cases()
        .into_iter()
        .find(|c| c.label == "fam012")
        .expect("fam012");
    let mut spelled: std::collections::BTreeMap<String, BTreeSet<StableName>> = Default::default();
    runs(&case, |at, ev, ids, unions| {
        let union = unions[0].1;
        if failure(ev, union).is_some() {
            return;
        }
        let (wa, wb) = (wall_at_y0(ev, union, ids[0]), wall_at_y0(ev, union, ids[1]));
        let mut set = vec![wa.clone(), wb];
        set.sort();
        let merge = StableName {
            kind: EntityKind::Face,
            node: union,
            path: vec![RoleSeg::Merged(set)],
        };
        let t = table(ev, union);
        assert!(t.lookup(&wa).is_none(), "{at}: {wa:?} is published");
        let pieces: BTreeSet<StableName> = t
            .iter()
            .filter(|(name, _)| name.kind == EntityKind::Face && parent_of(name) == merge)
            .map(|(name, _)| name.clone())
            .collect();
        assert_eq!(
            pieces.len(),
            2,
            "{at}: pieces of the merged wall {pieces:?}"
        );
        assert!(pieces.iter().all(|p| p.path.len() == 2), "{at}: {pieces:?}");
        spelled.insert(at.to_string(), pieces);
    });
    assert!(
        spelled.contains_key("[1, 2, 0]"),
        "the one-step order [b, g, a] fuses"
    );
    let mut all = spelled.values();
    let first = all.next().expect("a fused order");
    assert!(all.all(|s| s == first), "{spelled:?}");
}

/// **A body that depends on member order still publishes.** In the ZIP
/// document, `[b, s, a]` and `[s, b, a]` keep a vertex on the slab's
/// bottom seam that the other orders do not, so the seam there is two
/// edges between the same two faces. They publish, every name once.
#[test]
fn a_seam_a_leftover_vertex_splits_is_published_twice_under_two_names() {
    let case = near_slab();
    let mut published = std::collections::BTreeMap::new();
    runs(&case, |at, ev, _, unions| {
        let union = unions[0].1;
        match failure(ev, union) {
            None => {
                published.insert(at.to_string(), body_of(ev, union).vertices().count());
                seam_sides_hold(ev, union, at);
            }
            Some(e @ NodeErrorKind::Naming(_)) => panic!("{at}: {e}"),
            Some(e) => eprintln!("{at}: refused {e:?}"),
        }
    });
    let count = |order: &str| {
        *published
            .get(order)
            .unwrap_or_else(|| panic!("{order} does not publish: {published:?}"))
    };
    // The shape under test: the orders that cut before they merge keep
    // two vertices the others do not.
    for (late, early) in [("[1, 2, 0]", "[0, 1, 2]"), ("[2, 1, 0]", "[1, 0, 2]")] {
        assert_eq!(count(late), count(early) + 2, "{late} against {early}");
    }
}

/// `a` = [0,1]³ and a block touching it along an edge or at a corner.
const TOUCH_EDGE: Bx = ((1.0, 2.0), (1.0, 2.0), (0.0, 1.0));
const TOUCH_CORNER: Bx = ((1.0, 2.0), (1.0, 2.0), (1.0, 1.0));
/// A block far from both, and a slab through `a`.
const FAR: Bx = ((5.0, 6.0), (0.0, 1.0), (0.0, 1.0));
const G: Bx = ((0.3, 0.4), (-1.0, 2.0), (0.5, 3.0));

/// **A member made of two shells that touch is named shell by shell.**
///
/// `U1` = `a` ∪ a block touching it along an edge, or at a corner: two
/// shells whose vertices coincide where they touch. `U1` then joins a
/// far block or a slab through `a`, in both orders. Each coincident
/// vertex keeps its own shell's name: a vertex of `a`'s shell is never
/// named for the touching block's vertex at the same point.
#[test]
fn a_member_of_two_touching_shells_names_each_shell_for_itself() {
    let mut fused = 0;
    for (touch, third) in [
        (TOUCH_EDGE, FAR),
        (TOUCH_EDGE, G),
        (TOUCH_CORNER, FAR),
        (TOUCH_CORNER, G),
    ] {
        let (doc, ids) = document(&[A, touch, third], &[0, 1, 2]);
        let (doc, u1) = insert(
            doc,
            Node::Union {
                members: vec![ids[0], ids[1]],
                declare: None,
            },
        );
        for order in permutations(&[0, 1]) {
            let members: Vec<_> = order.iter().map(|&i| [u1, ids[2]][i]).collect();
            let (docx, top) = insert(
                doc.clone(),
                Node::Union {
                    members,
                    declare: None,
                },
            );
            let ev = run(&docx);
            let at = format!("{touch:?} {third:?} {order:?}");
            for union in [u1, top] {
                assert!(
                    failure(&ev, union).is_none(),
                    "{at}: {:?}",
                    failure(&ev, union)
                );
                member_vertices_hold(&ev, union, &at);
                seam_sides_hold(&ev, union, &at);
            }
            fused += 1;
        }
    }
    assert_eq!(fused, 8);
}
