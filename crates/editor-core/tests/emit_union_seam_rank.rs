//! **A union's vertices that share a run of seams are ranked along a
//! seam in it whose pieces lie on one open carrier, a curved one
//! included, in every member order** (N2, `emit_union::rank_along_seam`).
//!
//! Each scene unites `x`, the unit disc extruded z ∈ [0, 1], a block `y`
//! under z = 0.5, and a disc `w` extruded z ∈ [−0.25, 0.75]. `x`'s wall
//! crosses the seam where `w`'s wall leaves `y`'s top twice, and covers
//! the stretch of it between; no member edge reaches either crossing,
//! so each is named for the run of its three seams (that arc, the one
//! where `x`'s wall leaves `y`'s top, and the line where the two walls
//! meet), and the two share that name. The two walls meet on two
//! vertical lines, so that seam never ranks; which arc does is the
//! scene's.
//!
//! - [`OPEN_ARC`]: what is left of `w`'s arc is one open stretch from
//!   one crossing to the other.
//! - [`CUT_ARC`]: `y`'s side at x = −0.9 also cuts `w`'s arc, so its
//!   pieces leave two gaps, of which only the one `x` swallows is
//!   bounded by the crossings: the arc reads from the other.
//!   `x`'s arc leaves three gaps, two of them, at `y`'s side and its
//!   end at y = −0.95, free of the crossings, so it has no start.
//! - [`NO_START`]: `y` cut back to y ≤ 2.2 cuts `w`'s arc a third time,
//!   so it too leaves two gaps free of the crossings, and the two tie.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::blocks::{block_on, failure, run};
use crate::corpus::body_of;
use crate::emit_shared_rim_several::permutations;
use crate::emit_union_rim_piece_ranks::signature;
use crate::fixture::{insert, len, slid, table, union_over};

use editor_core::{
    EntityKey, Entry, Evaluation, LoopProgram, Node, ProfileDoc, ProfileProgram, Qualifier,
    RecipeNodeId, RoleSeg, StableName,
};
use geom_core::{Point3, Tol};

/// The disc of radius `r` about `(cx, cy)` on its own xy, extruded
/// `dz`; its wall is two half cylinders, cut where `phase` says
/// ([`LoopProgram::circle_split`]), or where the kernel cuts a circle.
fn disc(doc: ProfileDoc, ((cx, cy), r, phase): Disc, dz: f64) -> (ProfileDoc, RecipeNodeId) {
    let (doc, p) = insert(
        doc,
        Node::Profile(ProfileProgram {
            loops: vec![
                match phase {
                    Some(phase) => LoopProgram::circle_split(cx, cy, r, 2, phase),
                    None => LoopProgram::circle(cx, cy, r),
                }
                .expect("a finite circle"),
            ],
            ids: Vec::new(),
        }),
    );
    insert(
        doc,
        Node::Extrude {
            profile: p.into(),
            distance: len(dz),
        },
    )
}

/// A disc's centre, radius, and where its wall is cut in two.
type Disc = ((f64, f64), f64, Option<f64>);

/// One scene: `x` and `w`, `y`'s spans along x and y, and the height y
/// at which `x`'s wall crosses `w`'s.
struct Scene {
    x: Disc,
    w: Disc,
    y: ((f64, f64), (f64, f64)),
    cross_y: f64,
}

const OPEN_ARC: Scene = Scene {
    x: ((0.0, 0.0), 1.0, None),
    w: ((0.0, 0.6), 0.7, None),
    y: ((-3.0, 3.0), (0.2, 3.0)),
    cross_y: 0.725,
};

const CUT_ARC: Scene = Scene {
    x: ((0.0, 0.0), 1.0, Some(0.6)),
    w: ((0.0, 1.5), 1.0, Some(0.0)),
    y: ((-0.9, 3.0), (-0.95, 3.0)),
    cross_y: 0.75,
};

const NO_START: Scene = Scene {
    w: ((0.0, 1.5), 1.0, Some(2.5)),
    ..CUT_ARC
};

/// One evaluated union: its member order, the evaluation, and the union.
type Run = (String, Evaluation<f64>, RecipeNodeId);

/// `x`, `y` and `w`, united in every order.
fn every_order(scene: &Scene) -> Vec<Run> {
    let doc = ProfileDoc::empty_derived("emit_union_seam_rank", Tol::witness());
    let (doc, x) = disc(doc, scene.x, 1.0);
    let (doc, y) = block_on(doc, x, scene.y.0, scene.y.1, 0.0, 0.5);
    let (doc, w) = disc(doc, scene.w, 1.0);
    let (doc, w) = slid(doc, x, w, [0.0, 0.0, -0.25]);
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

/// The rows naming a vertex at either crossing of `scene`, the one at
/// −x first: each its name and whether it is tied.
fn crossings(scene: &Scene, ev: &Evaluation<f64>, union: RecipeNodeId) -> Vec<(StableName, bool)> {
    let body = body_of(ev, union);
    let y = scene.cross_y;
    let at = |sx: f64| Point3::new(sx * y.mul_add(-y, 1.0).sqrt(), y, 0.5);
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

/// **The two crossings of `scene` rank along one arc of the run,
/// alike in every member order.** Each crossing is the same run of
/// three seams, the one at −x ranked `minus_x` of 2 and the other the
/// other rank; every order publishes one table ([`signature`]). Read
/// from the gap between the crossings instead, the ranks swap.
fn crossings_rank_in_every_member_order(scene: &Scene, minus_x: u32) {
    let runs = every_order(scene);
    assert_eq!(runs.len(), 6, "every order of three members");
    let (first_at, first_ev, first_union) = &runs[0];
    let first = signature(first_ev, *first_union);
    for (at, ev, union) in &runs {
        assert_eq!(first, signature(ev, *union), "{first_at} against {at}");
        let rows = crossings(scene, ev, *union);
        let [(a, false), (b, false)] = rows.as_slice() else {
            panic!("{at}: one untied row at each crossing, got {rows:?}");
        };
        let ((base_a, rank_a), (base_b, rank_b)) = (rank_of(a), rank_of(b));
        assert_eq!(base_a, base_b, "{at}: the two crossings share one name");
        assert!(
            matches!(base_a.path.as_slice(), [RoleSeg::EdgeCrossing(run)] if run.len() == 3),
            "{at}: the name is a run of three seams, got {base_a:?}"
        );
        assert_eq!(
            [rank_a, rank_b],
            [Some(minus_x), Some(1 - minus_x)],
            "{at}: the crossing at −x ranked {minus_x} of 2, got {a:?} and {b:?}"
        );
    }
}

/// **Two crossings of an arc seam's one open stretch rank along its
/// circle** ([`OPEN_ARC`]).
#[test]
fn crossings_of_one_open_arc_rank_along_it_in_every_member_order() {
    crossings_rank_in_every_member_order(&OPEN_ARC, 1);
}

/// **Two crossings of an arc whose pieces leave two gaps rank from the
/// gap they leave free** ([`CUT_ARC`]): the only seam of the run that
/// reads is `w`'s arc, read from where `y`'s side cuts it.
#[test]
fn crossings_of_an_arc_cut_twice_rank_from_the_gap_they_leave_free_in_every_member_order() {
    crossings_rank_in_every_member_order(&CUT_ARC, 1);
}

/// **Where every seam of the run leaves several gaps free of the
/// crossings, or lies on two lines, the crossings tie in every member
/// order** ([`NO_START`]): no seam of the run has a start of its own to
/// rank them from.
#[test]
fn crossings_of_seams_with_no_start_tie_in_every_member_order() {
    for (at, ev, union) in every_order(&NO_START) {
        let rows = crossings(&NO_START, &ev, union);
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
