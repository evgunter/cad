//! **A union's names do not depend on the order of its members.**
//!
//! `Node::Union` folds its members pairwise, so each step's pair emitter
//! sees an A side and a B side, and the union's collapse rewrites every
//! fold-space name into member space. A name the collapse makes
//! order-independent — a `Seam` pair, canonicalized by name — must carry
//! a tail that is order-independent too: a seam chain's `OrderAlong`
//! rank was taken along the pair emitter's A-first line, so wherever
//! canonicalization swaps the pair, the rank is read from the other end.
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
//! - two declared-flush members and a bar across both, in both orders
//!   of the flush pair: the bar's cap fragments carry their `SideOf`
//!   partners in member-space name order, so each fragment has one
//!   name.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::corpus::body_of;
use crate::docm7_union_declare::{block, run};
use crate::emit_boolean_vertex_keys::{
    edge_touch_outside, ell_and_tip, face_touch, named_geometry, nested, seamed_touch,
};
use crate::fixture::{ends, fname, insert, len, member_face, on_frame, point, table};

use editor_core::{
    CapEnd, EntityKey, EntityKind, Entry, NameRef, Node, ProfileDoc, Qualifier, RecipeNodeId,
    RoleSeg, StableName,
};
use geom_core::Tol;

type Fixture = fn(ProfileDoc) -> (ProfileDoc, RecipeNodeId, RecipeNodeId);

/// A slab and an inverted-U rib standing in it: the rib's arms (x in
/// [0.5, 1] and [2, 2.5], y in [0.5, 1.5]) cross the slab's top at
/// z = 1, so each rib cap meets it in a two-edge collinear seam chain.
fn slab_rib(doc: ProfileDoc) -> (ProfileDoc, RecipeNodeId, RecipeNodeId) {
    let (doc, slab) = block(doc, (0.0, 3.0), (0.0, 2.0), 0.0, 1.0);
    let (doc, p) = on_frame(
        doc,
        [0.0, 1.5, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0],
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
            profile: p,
            distance: len(1.0),
        },
    );
    (doc, slab, rib)
}

/// The union of `fixture`'s two bodies with its members in the order
/// `swap` picks, in a document of its own — both orders get the same
/// node ids, so their names compare directly.
fn union_of(
    fixture: Fixture,
    swap: bool,
) -> (
    editor_core::Evaluation<f64>,
    RecipeNodeId,
    [RecipeNodeId; 2],
) {
    let doc = ProfileDoc::empty_derived("emit_union_member_order", Tol::witness());
    let (doc, x, y) = fixture(doc);
    let members = if swap { vec![y, x] } else { vec![x, y] };
    let (doc, u) = insert(
        doc,
        Node::Union {
            members,
            declare: None,
        },
    );
    (run(&doc), u, [x, y])
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
        let (ev_xy, u_xy, _) = union_of(fixture, false);
        let (ev_yx, u_yx, _) = union_of(fixture, true);
        let xy = named_geometry(&ev_xy, u_xy, false).expect("a union is never empty");
        let yx = named_geometry(&ev_yx, u_yx, false).expect("a union is never empty");
        let diff: Vec<_> = xy.symmetric_difference(&yx).collect();
        assert!(
            diff.is_empty(),
            "{what}: names bound to different geometry in the two member orders:\n{diff:#?}"
        );
    }
}

/// **A union's seam chain is ranked along the canonical pair.**
///
/// The rib's caps meet the slab's top in two chains of two edges each.
/// The slab's top has outward normal +z; the rib's end cap −y, its start
/// cap +y. In canonical (name) order the slab's face comes first, so
/// the end-cap chain runs along +z × −y = +x and the start-cap chain
/// along +z × +y = −x. Each rank names the edge at that place along
/// its chain, in both member orders.
#[test]
fn a_unions_seam_chain_is_ranked_along_the_canonical_pair() {
    for swap in [false, true] {
        let (ev, u, [slab, rib]) = union_of(slab_rib, swap);
        let top = member_face(u, slab, fname(slab, RoleSeg::Cap(CapEnd::End)));
        let body = body_of(&ev, u);
        for (cap, rank, x) in [
            (CapEnd::End, 0, (0.5, 1.0)),
            (CapEnd::End, 1, (2.0, 2.5)),
            (CapEnd::Start, 0, (2.0, 2.5)),
            (CapEnd::Start, 1, (0.5, 1.0)),
        ] {
            let rib_cap = member_face(u, rib, fname(rib, RoleSeg::Cap(cap)));
            let (a, b) = if top <= rib_cap {
                (top.clone(), rib_cap)
            } else {
                (rib_cap, top.clone())
            };
            let n = StableName {
                kind: EntityKind::Edge,
                node: u,
                path: vec![
                    RoleSeg::Seam {
                        a: NameRef::new(a),
                        b: NameRef::new(b),
                    },
                    RoleSeg::Fragment(Qualifier::OrderAlong { rank, of: 2 }),
                ],
            };
            let e = match table(&ev, u).lookup(&n) {
                Some(Entry::Unique(r)) => match r.key {
                    EntityKey::Edge(e) => e,
                    other => panic!("swap={swap}: {n:?} names {other:?}"),
                },
                other => panic!("swap={swap}: {n:?} is not uniquely named: {other:?}"),
            };
            let [p, q] = ends(body, e).map(|v| point(body, v));
            let (lo, hi) = (p.x.min(q.x), p.x.max(q.x));
            assert!(
                (lo - x.0).abs() < 1e-9 && (hi - x.1).abs() < 1e-9,
                "swap={swap}: {cap:?} rank {rank} binds x in [{lo}, {hi}], not {x:?}"
            );
        }
    }
}

/// `slab_rib` and a cutter [0.7, 2.3] × [0.3, 1.7] × [0.95, 1.05] that
/// crosses every seam edge the slab and the rib make. Depending on
/// member order, a seam line is minted already cut (a seam chain) or
/// minted whole and cut by a later step (a descent chain of its pieces).
fn slab_rib_cutall(doc: ProfileDoc) -> (ProfileDoc, Vec<RecipeNodeId>) {
    let (doc, slab, rib) = slab_rib(doc);
    let (doc, cutter) = block(doc, (0.7, 2.3), (0.3, 1.7), 0.95, 0.1);
    (doc, vec![slab, rib, cutter])
}

/// [`slab_rib_cutall`] and a block far from everything: in some orders
/// a seam passes through a step unchanged before a later one cuts it,
/// so the pieces' names descend through more than one wrapper.
fn slab_rib_cutall_far(doc: ProfileDoc) -> (ProfileDoc, Vec<RecipeNodeId>) {
    let (doc, mut m) = slab_rib_cutall(doc);
    let (doc, far) = block(doc, (5.0, 6.0), (0.0, 1.0), 0.0, 1.0);
    m.push(far);
    (doc, m)
}

/// [`slab_rib_cutall`] and a second small cutter across the first rib
/// arm's back seam: two later steps cut seam lines, in either order.
fn slab_rib_cutall_two(doc: ProfileDoc) -> (ProfileDoc, Vec<RecipeNodeId>) {
    let (doc, mut m) = slab_rib_cutall(doc);
    let (doc, second) = block(doc, (0.4, 0.6), (0.8, 1.0), 0.95, 0.1);
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
    for line in named_geometry(ev, u, false).expect("a union is never empty") {
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

/// **No name binds different geometry in any two member orders of a
/// union whose seam lines are cut.**
///
/// A seam line's pieces are ranked by the seam-chain ranker in one
/// order and by the descent ranker in another — after one step or
/// several, through one wrapper or more. Every ranker along a seam line
/// must use one orientation, or one name lands on another piece. The
/// tables need not be identical — which step cut the line shows in a
/// name's structure — but a name present in two orders names the same
/// entity in both. Three fixtures, every order: 6, 24 and 24.
#[test]
fn no_name_rebinds_across_the_member_orders_of_a_cut_seam_union() {
    type Members = fn(ProfileDoc) -> (ProfileDoc, Vec<RecipeNodeId>);
    let fixtures: [(&str, Members); 3] = [
        ("slab_rib_cutall", slab_rib_cutall),
        ("slab_rib_cutall_far", slab_rib_cutall_far),
        ("slab_rib_cutall_two", slab_rib_cutall_two),
    ];
    for (what, fixture) in fixtures {
        let n = fixture(ProfileDoc::empty_derived("count", Tol::witness()))
            .1
            .len();
        let tables: Vec<_> = orders(n)
            .into_iter()
            .map(|p| {
                let doc = ProfileDoc::empty_derived("emit_union_member_order_cut", Tol::witness());
                let (doc, m) = fixture(doc);
                let (doc, u) = insert(
                    doc,
                    Node::Union {
                        members: p.iter().map(|&i| m[i]).collect(),
                        declare: None,
                    },
                );
                let ev = run(&doc);
                let t = bindings(&ev, u);
                (p, t)
            })
            .collect();
        let mut rebound = Vec::new();
        for (i, (pi, a)) in tables.iter().enumerate() {
            for (pj, b) in &tables[i + 1..] {
                for (n, ga) in a {
                    if let Some(gb) = b.get(n)
                        && ga != gb
                    {
                        rebound.push(format!("{pi:?} vs {pj:?}: {n} binds {ga} vs {gb}"));
                    }
                }
            }
        }
        assert!(
            rebound.is_empty(),
            "{what}: {} rebinds:\n{rebound:#?}",
            rebound.len()
        );
    }
}

/// **A seam a split passed through, cut by a later boolean, is ranked
/// along its line.**
///
/// `Union(slab, rib)`, split at x = 1.5, the lower half kept and then
/// cut by a small block across the rib arm's seam. The split renames
/// the faces it cut (`SplitFragment { parent }`) and keeps the seam
/// edge's name, so finding the seam pair's two faces has to see
/// through the split's renaming. Both the subtraction and the union
/// name the result, and each seam piece the block cut lies on its
/// parent seam's line.
#[test]
fn a_seam_passed_through_a_split_and_cut_later_is_named() {
    use crate::fixture::scl;
    use editor_core::{BooleanOp, Datum, PartSelect, SplitHalf};
    let doc = ProfileDoc::empty_derived("emit_union_member_order_split", Tol::witness());
    let (doc, slab, rib) = slab_rib(doc);
    let pair = |doc, op, a, b| {
        insert(
            doc,
            Node::Boolean {
                op,
                a,
                b,
                declare: None,
            },
        )
    };
    let (doc, joined) = pair(doc, BooleanOp::Union, slab, rib);
    let (doc, tool) = insert(
        doc,
        Node::Datum(Datum::Plane {
            origin: [len(1.5), len(0.0), len(0.0)],
            normal: [scl(1.0), scl(0.0), scl(0.0)],
        }),
    );
    let (doc, split) = insert(
        doc,
        Node::Split {
            target: joined,
            tool,
        },
    );
    let (doc, below) = insert(
        doc,
        Node::Part {
            of: split,
            select: PartSelect::SplitHalf(SplitHalf::Below),
        },
    );
    let (doc, cut) = block(doc, (0.7, 0.8), (0.3, 0.7), 0.95, 0.1);
    let (doc, minus) = pair(doc, BooleanOp::Subtract, below, cut);
    let (doc, plus) = pair(doc, BooleanOp::Union, below, cut);
    let ev = run(&doc);
    for (what, id) in [("subtract", minus), ("union", plus)] {
        if let Some(e) = crate::docm7_union_declare::failure(&ev, id) {
            panic!("{what} refused: {e}");
        }
        let ranked = named_geometry(&ev, id, false)
            .expect("a body")
            .into_iter()
            .filter(|l| l.contains("OrderAlong") && l.contains("Seam"))
            .count();
        assert!(
            ranked > 0,
            "{what}: no seam piece was ranked, so the row pins nothing"
        );
    }
}

/// **A seam between two placements of one prototype names.**
///
/// A transform adds no segment (N1), so the U-rib and a rotated copy of
/// it carry identical tables and their seams are `Seam { a: x, b: x }`.
/// Such a pair names no side; its seam chains take their sides from
/// the pair's structure, and a later cut of one ranks along the edge's
/// own carrier. Every pair boolean of the two, both union orders
/// included, names its result, as does a union node over them.
#[test]
fn a_seam_between_two_placements_of_one_prototype_is_named() {
    use crate::fixture::{ang, scl};
    use editor_core::BooleanOp;
    let doc = ProfileDoc::empty_derived("emit_union_member_order_same", Tol::witness());
    let (doc, _slab, rib) = slab_rib(doc);
    let (doc, turned) = insert(
        doc,
        Node::Transform {
            input: rib,
            translation: [len(0.1), len(1.5), len(0.3)],
            rotation_axis: [scl(1.0), scl(0.0), scl(0.0)],
            rotation_angle: ang(std::f64::consts::FRAC_PI_2),
        },
    );
    let mut doc = doc;
    let mut ids = Vec::new();
    for (what, op, a, b) in [
        ("union", BooleanOp::Union, rib, turned),
        ("union, swapped", BooleanOp::Union, turned, rib),
        ("subtract", BooleanOp::Subtract, rib, turned),
        ("intersect", BooleanOp::Intersect, rib, turned),
    ] {
        let (d, id) = insert(
            doc,
            Node::Boolean {
                op,
                a,
                b,
                declare: None,
            },
        );
        doc = d;
        ids.push((what, id));
    }
    let (doc, node) = insert(
        doc,
        Node::Union {
            members: vec![rib, turned],
            declare: None,
        },
    );
    ids.push(("union node", node));
    let ev = run(&doc);
    for (what, id) in ids {
        if let Some(e) = crate::docm7_union_declare::failure(&ev, id) {
            panic!("{what} refused: {e}");
        }
        let seams = named_geometry(&ev, id, false)
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

/// **A fragment's `SideOf` partners are in name order whatever the
/// member order** (the `abys` witness of
/// `name-ordered-positions-in-a-path-have-no-single-home`).
///
/// `a` and `b` overlap in x and are declared flush; the bar `y` crosses
/// both, so `y`'s start cap is cut into two fragments told apart by
/// their side of `a`'s and `b`'s faces. The pair emitter writes the
/// partners sorted in the FOLD's space, where the first member is the
/// A side; the collapse has to sort them again in member space. In
/// `[a, b, y]` and `[b, a, y]` every face fragment carries the same
/// name and binds the same face.
#[test]
fn a_fragments_side_of_partners_are_one_order_in_every_member_order() {
    use crate::docm7_union_declare::{declared_union, flush_pairs};
    let tables: Vec<std::collections::BTreeMap<String, String>> = [[0usize, 1, 2], [1, 0, 2]]
        .into_iter()
        .map(|p| {
            let doc = ProfileDoc::empty_derived("emit_union_member_order_abys", Tol::witness());
            let (doc, a) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
            let (doc, b) = block(doc, (0.5, 1.5), (0.0, 1.0), 0.0, 1.0);
            let (doc, y) = block(doc, (-1.0, 2.0), (0.3, 0.4), 0.5, 3.5);
            let m = [a, b, y];
            let members: Vec<RecipeNodeId> = p.iter().map(|&i| m[i]).collect();
            let (doc, u, _) = declared_union(doc, &members, flush_pairs((a, a), (b, b)));
            let ev = run(&doc);
            bindings(&ev, u)
                .into_iter()
                .filter(|(n, _)| n.contains("SideOf"))
                .collect()
        })
        .collect();
    assert!(
        !tables[0].is_empty(),
        "no fragment carries a SideOf, so the row pins nothing"
    );
    assert_eq!(
        tables[0], tables[1],
        "the fragments' names depend on the member order"
    );
}
