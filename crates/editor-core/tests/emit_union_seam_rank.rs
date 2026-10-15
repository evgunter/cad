//! **A union's vertices that share a run of seams are ranked along a
//! seam in it whose pieces lie on one open carrier, a curved one
//! included, in every member order** (N2, `emit_union::rank_along_seam`).
//!
//! `x` is the unit disc extruded z ∈ [0, 1], `y` the block
//! y ∈ [0.2, 3] (|x| ≤ 3) under z = 0.5, and `w` the disc of radius 0.7
//! about (0, 0.6), extruded z ∈ [−0.25, 0.75]. `x`'s wall crosses the arc
//! seam where `w`'s wall leaves `y`'s top twice, at (±√(1 − 0.725²),
//! 0.725, 0.5); `x` covers the arc between, so what is left of it is one
//! open stretch of its circle, from one crossing to the other. No member
//! edge reaches either crossing, so each is named for the run of its
//! three seams, and the two share that name.
//!
//! With `y` cut back to y ≤ 1.1, every seam of the run is cut into pieces
//! that leave two gaps in their circle, or lie on two lines, so no seam
//! of it has a start of its own, and the two tie.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::blocks::{block, failure, run};
use crate::corpus::body_of;
use crate::emit_shared_rim_several::permutations;
use crate::emit_union_rim_piece_ranks::signature;
use crate::fixture::{frame, insert, len, table, union_over};

use editor_core::{
    EntityKey, Entry, Evaluation, ExtrudeSide, LoopProgram, Node, ProfileDoc, ProfileProgram,
    Qualifier, RecipeNodeId, RoleSeg, StableName,
};
use geom_core::{Point3, Tol};

/// The disc of radius `r` about `(cx, cy)`, extruded `dz` up from `z0`.
fn disc(
    doc: ProfileDoc,
    (cx, cy): (f64, f64),
    r: f64,
    z0: f64,
    dz: f64,
) -> (ProfileDoc, RecipeNodeId) {
    let (doc, plane) = insert(doc, frame([0.0, 0.0, z0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]));
    let (doc, p) = insert(
        doc,
        Node::Profile(ProfileProgram {
            frame: plane.into(),
            loops: vec![LoopProgram::circle(cx, cy, r).expect("a finite circle")],
            ids: Vec::new(),
        }),
    );
    insert(
        doc,
        Node::Extrude {
            profile: p.into(),
            distance: len(dz),
            side: ExtrudeSide::Along,
        },
    )
}

/// One evaluated union: its member order, the evaluation, and the union.
type Run = (String, Evaluation<f64>, RecipeNodeId);

/// `x`, `y` (over y from 0.2 to `y1`) and `w`, united in every order.
fn every_order(y1: f64) -> Vec<Run> {
    let doc = ProfileDoc::empty_derived("emit_union_seam_rank", Tol::witness());
    let (doc, x) = disc(doc, (0.0, 0.0), 1.0, 0.0, 1.0);
    let (doc, y) = block(doc, (-3.0, 3.0), (0.2, y1), 0.0, 0.5);
    let (doc, w) = disc(doc, (0.0, 0.6), 0.7, -0.25, 1.0);
    let members = [x, y, w];
    permutations(&[0, 1, 2])
        .into_iter()
        .map(|order| {
            let ids: Vec<RecipeNodeId> = order.iter().map(|&i| members[i]).collect();
            let (doc, union) = union_over(doc.clone(), &ids);
            let ev = run(&doc);
            let at = format!("order {order:?}");
            assert!(
                failure(&ev, union).is_none(),
                "{at}: {:?}",
                failure(&ev, union)
            );
            (at, ev, union)
        })
        .collect()
}

/// The rows naming a vertex at either crossing, the one at −x first:
/// each its name and whether it is tied.
fn crossings(ev: &Evaluation<f64>, union: RecipeNodeId) -> Vec<(StableName, bool)> {
    let body = body_of(ev, union);
    let at = |sx: f64| Point3::new(sx * 0.725f64.mul_add(-0.725, 1.0).sqrt(), 0.725, 0.5);
    let mut out: Vec<(f64, StableName, bool)> = Vec::new();
    for (name, entry) in table(ev, union).iter() {
        let (keys, tied) = match entry {
            Entry::Unique(e) => (vec![e.key], false),
            Entry::Tied(es) => (es.iter().map(|e| e.key).collect(), true),
        };
        for k in keys {
            let EntityKey::Vertex(v) = k else { continue };
            let p = *body.get_point(body.get_vertex(v).unwrap().point).unwrap();
            for sx in [-1.0, 1.0] {
                if (p - at(sx)).norm() < 1e-9 {
                    out.push((sx, name.clone(), tied));
                }
            }
        }
    }
    out.sort_by(|a, b| a.0.total_cmp(&b.0));
    out.into_iter().map(|(_, n, t)| (n, t)).collect()
}

/// `name` without a trailing `Fragment(OrderAlong { of: 2 })`, and its
/// rank there.
fn rank_of(name: &StableName) -> (StableName, Option<u32>) {
    let mut base = name.clone();
    match base.path.last() {
        Some(&RoleSeg::Fragment(Qualifier::OrderAlong { rank, of: 2 })) => {
            base.path.pop();
            (base, Some(rank))
        }
        _ => (base, None),
    }
}

/// **Two crossings of an arc seam's one open stretch rank along its
/// circle, alike in every member order.** Each crossing is the same run
/// of three seams, ranked 0 and 1 of 2; every order publishes one table
/// ([`signature`]), so each crossing takes one rank in all of them.
#[test]
fn crossings_of_one_open_arc_rank_along_it_in_every_member_order() {
    let runs = every_order(3.0);
    assert_eq!(runs.len(), 6, "every order of three members");
    let (first_at, first_ev, first_union) = &runs[0];
    let first = signature(first_ev, *first_union);
    for (at, ev, union) in &runs {
        assert_eq!(first, signature(ev, *union), "{first_at} against {at}");
        let rows = crossings(ev, *union);
        let [(a, false), (b, false)] = rows.as_slice() else {
            panic!("{at}: one untied row at each crossing, got {rows:?}");
        };
        let ((base_a, rank_a), (base_b, rank_b)) = (rank_of(a), rank_of(b));
        assert_eq!(base_a, base_b, "{at}: the two crossings share one name");
        assert!(
            matches!(base_a.path.as_slice(), [RoleSeg::EdgeCrossing(run)] if run.len() == 3),
            "{at}: the name is a run of three seams, got {base_a:?}"
        );
        let mut ranks = [rank_a, rank_b];
        ranks.sort_unstable();
        assert_eq!(
            ranks,
            [Some(0), Some(1)],
            "{at}: ranks 0 and 1 of 2, got {a:?} and {b:?}"
        );
    }
}

/// **Where every seam of the run leaves two gaps in its circle, or lies
/// on two lines, the crossings tie in every member order**: no seam of
/// the run has a start of its own to rank them from.
#[test]
fn crossings_of_seams_with_no_start_tie_in_every_member_order() {
    for (at, ev, union) in every_order(1.1) {
        let rows = crossings(&ev, union);
        let [(a, true), (b, true)] = rows.as_slice() else {
            panic!("{at}: the two crossings in one tied row, got {rows:?}");
        };
        assert_eq!(a, b, "{at}: one tied row");
        assert!(
            matches!(a.path.as_slice(), [RoleSeg::EdgeCrossing(run)] if run.len() == 3),
            "{at}: the name is a run of three seams, got {a:?}"
        );
    }
}
