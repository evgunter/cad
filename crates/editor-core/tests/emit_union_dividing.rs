//! **A union's `SideOf` partners are the seams that divide a face**
//! (N2's splitting features), not every neighbour of its pieces.
//!
//! A parent the finished body holds as several faces is qualified
//! against the faces whose seams divide it: a partner's oriented plane
//! has pieces strictly on both sides of it. A neighbour that stands on a
//! piece, or notches or trims one, divides nothing, so it is no partner,
//! whether it is curved (it has no plane at all) or planar. These rows
//! hold that on the shapes that tell the two apart, and hold a split's
//! names still under a value edit that moves the splitting feature.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::collections::{BTreeMap, BTreeSet};

use crate::corpus::body_of;
use crate::docm7_union_declare::{block, failure, run};
use crate::emit_shared_rim_several::permutations;
use crate::fixture::{ang, face_vertices, frame, insert, len, on_frame, scl, step, table};

use editor_core::{
    Axis3, BooleanOp, DocEdit, EntityKey, EntityKind, Entry, Evaluation, LoopProgram, Node,
    ProfileDoc, ProfileProgram, RecipeNodeId, RoleSeg, SlotId, StableName,
};
use geom_core::Tol;

/// `input` behind a rigid `Transform` at the identity, so an edit can
/// move it.
fn movable(doc: ProfileDoc, input: RecipeNodeId) -> (ProfileDoc, RecipeNodeId) {
    insert(
        doc,
        Node::Transform {
            input,
            translation: [len(0.0), len(0.0), len(0.0)],
            rotation_axis: [scl(0.0), scl(0.0), scl(1.0)],
            rotation_angle: ang(0.0),
        },
    )
}

/// `doc` with the transform `tr` translated to `to` along `axis`.
fn moved(doc: ProfileDoc, tr: RecipeNodeId, axis: Axis3, to: f64) -> ProfileDoc {
    step(
        doc,
        DocEdit::SetParam {
            node: tr,
            slot: SlotId::Translation(axis),
            expr: len(to),
        },
    )
    .0
}

/// A union of `members` in the order `order` picks.
fn union_of(
    doc: ProfileDoc,
    members: &[RecipeNodeId],
    order: &[usize],
) -> (ProfileDoc, RecipeNodeId) {
    insert(
        doc,
        Node::Union {
            members: order.iter().map(|&i| members[i]).collect(),
            declare: None,
        },
    )
}

/// A published face's parent: its name without its trailing `Fragment`s.
fn parent_of(face: &StableName) -> StableName {
    let mut f = face.clone();
    while f.path.len() > 1 && matches!(f.path.last(), Some(RoleSeg::Fragment(_))) {
        f.path.pop();
    }
    f
}

/// Each uniquely named face of `union`'s table → the centroid of its
/// vertices.
fn face_centroids(ev: &Evaluation<f64>, union: RecipeNodeId) -> BTreeMap<StableName, [f64; 3]> {
    let body = body_of(ev, union);
    let mut out = BTreeMap::new();
    for (name, entry) in table(ev, union).iter() {
        let (EntityKind::Face, Entry::Unique(e)) = (name.kind, entry) else {
            continue;
        };
        let EntityKey::Face(f) = e.key else { continue };
        let vs = face_vertices(body, f);
        let mut c = [0.0; 3];
        for &v in &vs {
            let p = body.get_point(body.get_vertex(v).unwrap().point).unwrap();
            c[0] += p.x;
            c[1] += p.y;
            c[2] += p.z;
        }
        let n = vs.len() as f64;
        out.insert(name.clone(), [c[0] / n, c[1] / n, c[2] / n]);
    }
    out
}

/// **A curved neighbour that divides nothing is no partner.** A plate
/// `[0,3]² × [0,1]`; a slab at x 0.5..0.7 through its top, dividing it in
/// two; and a cylinder boss standing through the top away from the slab.
/// The boss's curved walls border one piece and divide nothing, so the
/// top's pieces are sided against the slab alone. Every member order
/// publishes except the two the pair boolean already refuses, and the
/// ones that publish publish one table.
#[test]
fn a_curved_neighbour_that_divides_nothing_is_no_partner() {
    let doc = ProfileDoc::empty_derived("union-dividing-boss", Tol::witness());
    let (doc, plate) = block(doc, (0.0, 3.0), (0.0, 3.0), 0.0, 1.0);
    let (doc, slab) = block(doc, (0.5, 0.7), (-1.0, 4.0), 0.5, 1.5);
    let (doc, plane) = insert(
        doc,
        frame([0.0, 0.0, 0.6], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]),
    );
    let (doc, disc) = insert(
        doc,
        Node::Profile(ProfileProgram {
            plane,
            loops: vec![LoopProgram::circle_split(2.0, 1.5, 0.3, 2, 0.0).unwrap()],
            ids: Vec::new(),
        }),
    );
    let (doc, boss) = insert(
        doc,
        Node::Extrude {
            profile: disc,
            distance: len(1.0),
        },
    );
    let members = [plate, slab, boss];
    let mut tables: Vec<(String, BTreeSet<StableName>)> = Vec::new();
    let mut refused = Vec::new();
    for order in permutations(&[0, 1, 2]) {
        let (docx, u) = union_of(doc.clone(), &members, &order);
        let ev = run(&docx);
        if let Some(e) = failure(&ev, u) {
            refused.push(format!("{order:?}: {e:?}"));
            continue;
        }
        let names: BTreeSet<StableName> = table(&ev, u).iter().map(|(n, _)| n.clone()).collect();
        // The plate's top is two pieces, each one `SideOf` against the
        // slab's walls alone.
        let pieces: Vec<&StableName> = names
            .iter()
            .filter(|n| {
                n.kind == EntityKind::Face
                    && matches!(n.path.first(), Some(RoleSeg::FromMember { member, .. }) if *member == plate)
                    && n.path.len() == 2
            })
            .collect();
        assert_eq!(pieces.len(), 2, "{order:?}: {pieces:?}");
        for p in &pieces {
            let Some(RoleSeg::Fragment(editor_core::Qualifier::SideOf(v))) = p.path.last() else {
                panic!("{order:?}: {p:?}")
            };
            assert!(
                v.iter().all(|(q, _)| matches!(q.path.first(), Some(RoleSeg::FromMember { member, .. }) if *member == slab)),
                "{order:?}: {p:?} cites a partner other than the slab"
            );
        }
        tables.push((format!("{order:?}"), names));
    }
    // The orders that fold the plate last split its top in the pair
    // emitter's own step, which still sides against every seam neighbour
    // and refuses on the boss
    // (`work/emit/the-pair-boolean-sides-a-split-face-against-every-seam-neighbour.md`);
    // the union adds no refusal of its own.
    assert_eq!(
        refused,
        [
            "[1, 2, 0]: Naming(Emission { what: \"face_plane: non-planar carrier in planar pipeline\" })",
            "[2, 1, 0]: Naming(Emission { what: \"face_plane: non-planar carrier in planar pipeline\" })",
        ],
        "the orders that refuse"
    );
    let (first_at, first) = &tables[0];
    for (at, t) in &tables[1..] {
        assert_eq!(first, t, "{first_at} against {at}");
    }
}

/// The tied-prongs block (`resolve_group_membership`'s fixture): a
/// 4×4×4 block with a U-shaped fork cut through it, whose two slot
/// ceilings are ONE face of the fork, so the block's table ties them.
fn tied_prongs() -> (ProfileDoc, RecipeNodeId) {
    let doc = ProfileDoc::empty_derived("union-dividing-tie", Tol::witness());
    let (doc, a) = block(doc, (0.0, 4.0), (0.0, 4.0), 0.0, 4.0);
    let (doc, p) = on_frame(
        doc,
        [0.0, 0.0, 1.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![
            (2.0, 1.0),
            (6.0, 1.0),
            (6.0, 3.0),
            (2.0, 3.0),
            (2.0, 2.5),
            (5.0, 2.5),
            (5.0, 1.5),
            (2.0, 1.5),
        ]],
    );
    let (doc, u) = insert(
        doc,
        Node::Extrude {
            profile: p,
            distance: len(2.0),
        },
    );
    insert(
        doc,
        Node::Boolean {
            op: BooleanOp::Subtract,
            a,
            b: u,
            declare: None,
        },
    )
}

/// The faces of `union`'s table that descend from a face `member`'s own
/// table ties, as (name, whether the row is tied).
fn from_tied(
    ev: &Evaluation<f64>,
    union: RecipeNodeId,
    member: RecipeNodeId,
) -> Vec<(StableName, bool)> {
    let own = table(ev, member);
    table(ev, union)
        .iter()
        .filter(|(n, _)| n.kind == EntityKind::Face)
        .filter(|(n, _)| {
            matches!(parent_of(n).path.as_slice(), [RoleSeg::FromMember { member: m, of }]
                if *m == member && matches!(own.lookup(of), Some(Entry::Tied(_))))
        })
        .map(|(n, e)| (n.clone(), matches!(e, Entry::Tied(_))))
        .collect()
}

/// **A notch that divides neither of two tied faces leaves them tied.**
/// A bar at x 3.9..4.1 closes the far end of one slot, notching one of
/// the two tied slot ceilings and dividing neither. Nothing divides the
/// tied parent, so both ceilings stay the tie's candidates under one
/// name, in both member orders. Moving the bar to the other slot (a y
/// edit) must not hand either ceiling a unique name: before the fix,
/// each got one against the bar's wall, and the same spelling denoted
/// the upper ceiling before the edit and the lower one after it.
#[test]
fn a_notch_that_divides_no_tied_face_leaves_the_tie() {
    let (doc, sub) = tied_prongs();
    let (doc, bar) = block(doc, (3.9, 4.1), (0.8, 1.7), 2.5, 1.0);
    let (doc, tr) = movable(doc, bar);
    for order in permutations(&[0, 1]) {
        let (doc1, u) = union_of(doc.clone(), &[sub, tr], &order);
        let doc2 = moved(doc1.clone(), tr, Axis3::Y, 1.5);
        let mut seen = Vec::new();
        for (label, d) in [("before", &doc1), ("after", &doc2)] {
            let ev = run(d);
            assert!(
                failure(&ev, u).is_none(),
                "{order:?} {label}: {:?}",
                failure(&ev, u)
            );
            let rows = from_tied(&ev, u, sub);
            assert!(
                !rows.is_empty(),
                "{order:?} {label}: no face descends from the tie"
            );
            for (n, tied) in &rows {
                assert!(tied, "{order:?} {label}: {n:?} is unique");
            }
            seen.push(rows.into_iter().map(|(n, _)| n).collect::<BTreeSet<_>>());
        }
        assert_eq!(seen[0], seen[1], "{order:?}: the edit renamed the tie");
    }
}

/// **A split's names hold under an edit that moves its splitting
/// feature.** A block `a` = `[0,1]³` and two slabs through its top (x
/// 0.2..0.3 and 0.6..0.7), the first behind a transform. Sliding the
/// first slab 0.05 along x keeps every piece: in every member order,
/// the same face names are published before and after, and each denotes
/// the same piece, moved by no more than the slide.
#[test]
fn a_split_keeps_its_names_when_the_splitting_feature_moves() {
    let doc = ProfileDoc::empty_derived("union-dividing-edit", Tol::witness());
    let (doc, a) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, s1) = block(doc, (0.2, 0.3), (-1.0, 2.0), 0.5, 2.5);
    let (doc, s2) = block(doc, (0.6, 0.7), (-1.0, 2.0), 0.5, 2.5);
    let (doc, tr) = movable(doc, s1);
    let mut checked = 0;
    for order in permutations(&[0, 1, 2]) {
        let (doc1, u) = union_of(doc.clone(), &[a, tr, s2], &order);
        let doc2 = moved(doc1.clone(), tr, Axis3::X, 0.05);
        let (ev1, ev2) = (run(&doc1), run(&doc2));
        for ev in [&ev1, &ev2] {
            assert!(failure(ev, u).is_none(), "{order:?}: {:?}", failure(ev, u));
        }
        let (c1, c2) = (face_centroids(&ev1, u), face_centroids(&ev2, u));
        assert_eq!(
            c1.keys().collect::<Vec<_>>(),
            c2.keys().collect::<Vec<_>>(),
            "{order:?}: the edit changed the published faces"
        );
        for (n, p) in &c1 {
            let q = c2[n];
            let d = ((p[0] - q[0]).powi(2) + (p[1] - q[1]).powi(2) + (p[2] - q[2]).powi(2)).sqrt();
            assert!(d <= 0.05 + 1e-9, "{order:?}: {n:?} moved {d}");
            checked += 1;
        }
        // The top is three pieces, each one `SideOf` against both slabs.
        let top_pieces = c1
            .keys()
            .filter(|n| {
                matches!(n.path.first(), Some(RoleSeg::FromMember { member, .. }) if *member == a)
                    && n.path.len() == 2
            })
            .count();
        assert!(
            top_pieces >= 3,
            "{order:?}: {top_pieces} pieces of a's faces"
        );
    }
    assert!(checked > 0);
}
