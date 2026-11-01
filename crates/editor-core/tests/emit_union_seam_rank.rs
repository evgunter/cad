//! **A union's vertices that share a run of seams are named apart by
//! their seams' senses, in every member order** (N2,
//! `emit_union::Flush::seam_sense`).
//!
//! Each scene unites `x`, the unit disc extruded z ∈ [0, 1], a block `y`
//! under z = 0.5, and a disc `w` extruded z ∈ [−0.25, 0.75]. `x`'s wall
//! crosses the seam where `w`'s wall leaves `y`'s top twice, and covers
//! the stretch of it between; no member edge reaches either crossing,
//! so each is named for the run of its three seams (that arc, the one
//! where `x`'s wall leaves `y`'s top, and the line where the two walls
//! meet). Each seam ends at the crossing, the result keeping it on one
//! side only, and the side it runs along into the member that trimmed
//! it is the side it is read as entering along: the arc enters `x` at
//! one crossing and leaves it at the other, so the two runs' senses
//! differ and the names come apart. Nothing ranks them.
//!
//! - [`OPEN_ARC`]: what is left of `w`'s arc is one open stretch from
//!   one crossing to the other.
//! - [`CUT_ARC`]: `y`'s side at x = −0.9 also cuts `w`'s arc, so its
//!   pieces leave two gaps; `x`'s arc leaves three.
//! - [`NO_START`]: `y` cut back to y ≤ 2.2 cuts `w`'s arc a third time.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::blocks::{block_on, failure, run};
use crate::corpus::body_of;
use crate::emit_shared_rim_several::permutations;
use crate::emit_union_rim_piece_ranks::signature;
use crate::fixture::{insert, len, slid, table, union_over};

use editor_core::{
    EntityKey, Entry, Evaluation, LoopProgram, Node, ProfileDoc, ProfileProgram, RecipeNodeId,
    RoleSeg, Sense, StableName,
};
use geom_core::{Point3, Tol};

/// The disc of radius `r` about `(cx, cy)` on its own xy, extruded
/// `dz`; its wall is two half cylinders, cut where `phase` says
/// ([`LoopProgram::circle_split`]), or where the kernel cuts a circle.
pub(crate) fn disc(
    doc: ProfileDoc,
    ((cx, cy), r, phase): Disc,
    dz: f64,
) -> (ProfileDoc, RecipeNodeId) {
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
            on: None,
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
pub(crate) type Disc = ((f64, f64), f64, Option<f64>);

/// One scene: `x` and `w`, `y`'s spans along x and y, and the height y
/// at which `x`'s wall crosses `w`'s.
pub(crate) struct Scene {
    pub(crate) x: Disc,
    pub(crate) w: Disc,
    pub(crate) y: ((f64, f64), (f64, f64)),
    cross_y: f64,
}

pub(crate) const OPEN_ARC: Scene = Scene {
    x: ((0.0, 0.0), 1.0, None),
    w: ((0.0, 0.6), 0.7, None),
    y: ((-3.0, 3.0), (0.2, 3.0)),
    cross_y: 0.725,
};

pub(crate) const CUT_ARC: Scene = Scene {
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

/// **The two crossings of `scene` name apart by their seams' senses,
/// alike in every member order.** Each crossing is a unique row, a run
/// of three seams; the two runs cite the same three lines, each seam
/// carrying the one side it enters along, and at least one seam reads
/// the other way at the other crossing. Every order publishes one table
/// ([`signature`]).
fn crossings_name_apart_in_every_member_order(scene: &Scene) {
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
        assert_ne!(a, b, "{at}: the two crossings name apart");
        let run = |n: &StableName| match n.path.as_slice() {
            [RoleSeg::EdgeCrossing(run)] if run.len() == 3 => run.clone(),
            _ => panic!("{at}: the name is a run of three seams, got {n:?}"),
        };
        let (ra, rb) = (run(a), run(b));
        for l in ra.iter().chain(&rb) {
            assert!(
                matches!(l.line.path.as_slice(), [RoleSeg::Seam { .. }]),
                "{at}: every line is a seam: {l:?}"
            );
            assert!(
                matches!(&l.sense, Sense::Through { entering } if entering.len() == 1),
                "{at}: each seam ends at the crossing, entering along one side: {l:?}"
            );
        }
        let lines =
            |r: &[editor_core::RunLine]| r.iter().map(|l| l.line.clone()).collect::<Vec<_>>();
        assert_eq!(lines(&ra), lines(&rb), "{at}: the same three seams");
    }
}

/// **Two crossings of an arc seam's one open stretch name apart by
/// sense** ([`OPEN_ARC`]).
#[test]
fn crossings_of_one_open_arc_name_apart_by_sense_in_every_member_order() {
    crossings_name_apart_in_every_member_order(&OPEN_ARC);
}

/// **Two crossings of an arc whose pieces leave two gaps name apart by
/// sense** ([`CUT_ARC`]).
#[test]
fn crossings_of_an_arc_cut_twice_name_apart_by_sense_in_every_member_order() {
    crossings_name_apart_in_every_member_order(&CUT_ARC);
}

/// **Two crossings of seams whose pieces leave several gaps name apart
/// by sense** ([`NO_START`]), where a rank along any of them would have
/// had no start to read from.
#[test]
fn crossings_of_seams_with_no_start_name_apart_by_sense_in_every_member_order() {
    crossings_name_apart_in_every_member_order(&NO_START);
}
