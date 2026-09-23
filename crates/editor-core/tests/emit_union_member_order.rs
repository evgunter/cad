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
//!   canonical (name) order.
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
