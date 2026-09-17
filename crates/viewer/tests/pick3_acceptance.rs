//! The `t`-interval door over the corpus, through the REAL door.
//!
//! Two claims are pinned here, in ONE pass over every landing of the
//! parametric corpus and the gallery ring:
//!
//! - **`Pruned == Every`.** `PickIndex::pick` — `pick_face` over the
//!   picture's parts — answers what an EXHAUSTIVE walk of every
//!   candidate box the ray meets answers. The exhaustive walk calls
//!   `ray_triangle` and `TSpan::best_of`, the door's own two doors, so
//!   it restates neither the traversal nor the order: what it does not
//!   have is the early-out, which is exactly what is under test.
//! - **The closed acceptance loses no aimed vertex** that `main`'s
//!   rounded-`t` order answered, and gains some.
//!
//! **The acceptance question is CLOSED and no longer measured here.**
//! The ruling left "closed ∧ INFORM stays, or MEET ∧ INFORM returns"
//! to the unit; the unit measured it (2026-09-16, the PR body and the
//! unit's row carry the numbers: MEET reaches `0 Pruned ≠ Every` under
//! the clamp but loses 513 of the wide aim's aimed vertices against
//! `main`'s 141 106 and gains none, so closed stays). A measurement
//! that has been made is not a permanent gate, and keeping MEET here
//! cost this file a second copy of `t_span` and of the acceptance —
//! the two things a reviewer greps for. Both are gone.

#![allow(clippy::expect_used, clippy::panic, clippy::unwrap_used)]

use bvh::{Aabb, Bvh, Ray};
use editor_core::resolve::{TSpan, crossing, ray_triangle};
use editor_core::{
    Dimension, DocEdit, Evaluation, Expr, ProfileDoc, RecipeNodeId, SlotId, unparse,
};
use pncad::geom_core::{Point3, Tol, Vec3};
use viewer::pickindex::{PickIndex, PictureKey};
use viewer::scene::DisplayTolerance;
use viewer::session::{DocSession, SessionOp};

use crate::common;
use crate::corpus;

fn delta() -> DisplayTolerance {
    DisplayTolerance::new(2.0e-3).expect("a positive delta")
}

fn fresh_index(session: &DocSession) -> PickIndex {
    let (doc, eval) = session.landed_pair().expect("a landed pair");
    let generation = session
        .landed_generation()
        .expect("a landed evaluation has a generation");
    PickIndex::build(
        doc,
        eval,
        PictureKey::of(generation, delta()),
        session.tol(),
    )
    .expect("the document indexes")
}

fn bump_op(c: &corpus::CorpusDoc) -> Option<SessionOp> {
    let DocEdit::SetParam { node, slot, expr } = c.bump.clone() else {
        return None;
    };
    Some(SessionOp::SetSlotExpression {
        node,
        slot,
        text: unparse(&expr),
    })
}

fn ring_bump(doc: &ProfileDoc) -> SessionOp {
    let env = doc.param_env::<f64>();
    for &node in doc.order().iter().rev() {
        let (slot, expr): (SlotId, Expr) = match doc.node(node).expect("a node") {
            editor_core::Node::Extrude { distance, .. } => {
                let value = editor_core::eval(distance, &env).expect("a literal distance");
                (
                    SlotId::Distance,
                    Expr::literal(value * 1.03125, Dimension::Length).expect("a length literal"),
                )
            }
            editor_core::Node::Revolve { angle, .. } => {
                let value = editor_core::eval(angle, &env).expect("a literal angle");
                (
                    SlotId::RevolveAngle,
                    Expr::literal(value * 0.96875, Dimension::Angle).expect("an angle literal"),
                )
            }
            _ => continue,
        };
        let _: RecipeNodeId = node;
        return SessionOp::SetSlotExpression {
            node,
            slot,
            text: unparse(&expr),
        };
    }
    panic!("no extrude or revolve in the ring")
}

// ---------------------------------------------------------------
// The exhaustive walk: every candidate box the ray meets, through
// `ray_triangle` and `TSpan::best_of`. No early-out, no order of its
// own.
// ---------------------------------------------------------------

struct FlatPart {
    tree: Bvh,
    corners: Vec<[Point3<f64>; 3]>,
}

fn flatten(index: &PickIndex) -> Vec<FlatPart> {
    index
        .parts()
        .iter()
        .map(|part| {
            let mesh = part.mesh();
            let mut corners = Vec::new();
            let mut boxes = Vec::new();
            for p in &mesh.patches {
                for tri in &p.triangles {
                    let c = tri.map(|i| mesh.positions[i as usize]);
                    boxes.push(Aabb::from_points(c).expect("three points box"));
                    corners.push(c);
                }
            }
            FlatPart {
                tree: Bvh::build(&boxes),
                corners,
            }
        })
        .collect()
}

/// One admitted candidate, with where it came from.
#[derive(Clone, Copy, Debug)]
struct Seen {
    part: usize,
    item: usize,
    span: TSpan,
}

/// Every admitted candidate on `ray`, part by part and, within a part,
/// in flat triangle order — the order `pick_face` reads as its last
/// tie-break key, so [`TSpan::best_of`] over this slice decides the
/// certified tie the way the door does.
fn every(parts: &[FlatPart], ray: &Ray) -> Vec<Seen> {
    let mut seen = Vec::new();
    for (part, flat) in parts.iter().enumerate() {
        let mut items: Vec<usize> = flat.tree.ray(ray).into_iter().map(|c| c.item).collect();
        items.sort_unstable();
        for item in items {
            if let Some(span) = ray_triangle(ray, &flat.corners[item]) {
                seen.push(Seen { part, item, span });
            }
        }
    }
    seen
}

fn winner(seen: &[Seen]) -> Option<Seen> {
    let spans: Vec<TSpan> = seen.iter().map(|s| s.span).collect();
    TSpan::best_of(&spans).map(|i| seen[i])
}

/// `main`'s order: the rounded `t`, then position. The OLD door, kept
/// as the oracle the wide aim's `aim_lost` column is measured against
/// — not a second spelling of this one.
fn by_rounded_t(seen: &[Seen]) -> Option<Seen> {
    seen.iter().copied().reduce(|b, c| {
        if c.span.t < b.span.t || (c.span.t == b.span.t && (c.part, c.item) < (b.part, b.item)) {
            c
        } else {
            b
        }
    })
}

/// The widest barycentric bound the winning candidate carries.
fn widest_bound(parts: &[FlatPart], ray: &Ray, win: &Seen) -> f64 {
    crossing(ray, &parts[win.part].corners[win.item])
        .expect("an admitted candidate has a certified determinant")
        .barycentrics
        .iter()
        .fold(0.0f64, |w, &(_, err)| w.max(err))
}

// ---------------------------------------------------------------
// The two aims.
// ---------------------------------------------------------------

/// The tie-break aim: what the early-out column is measured over.
#[derive(Default, Debug)]
struct TieTable {
    rays: usize,
    /// The door's answer differs from the exhaustive walk's. The
    /// early-out is the only difference between them, so any count
    /// here is the early-out pruning a candidate the set rule keeps.
    pruned_differs: usize,
    /// The aimed point is not answered: the winner is beyond it, or
    /// the ray misses. Printed, not pinned — the corpus's shape, and
    /// the row above it is
    /// `work/edit/pick-closed-acceptance-loses-a-graze-to-rounding`.
    beyond_or_miss: usize,
    /// The winner carries a barycentric bound of `1` or more.
    /// **Structurally 0**: `admits` refuses `err >= 1` outright, so
    /// this column cannot red while INFORM stands. It is here as the
    /// statement of that, and it would red the day INFORM moved.
    wide_winners: usize,
    examples: Vec<String>,
}

/// The wide aim against `main`'s order over the same candidates.
#[derive(Default, Debug)]
struct WideTable {
    rays: usize,
    moved: usize,
    moved_farther: usize,
    aimed_main: usize,
    aimed_interval: usize,
    aim_lost: usize,
    aim_gained: usize,
    aim_lost_examples: Vec<String>,
}

fn dirs() -> [Vec3<f64>; 6] {
    [
        Vec3::new(1.0, 0.0, 0.0),
        Vec3::new(-1.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        Vec3::new(0.0, -1.0, 0.0),
        Vec3::new(0.0, 0.0, 1.0),
        Vec3::new(0.0, 0.0, -1.0),
    ]
}

fn tie_rays_for(index: &PickIndex) -> Vec<(Ray, f64)> {
    let mut ext = 0.0f64;
    let mut targets = Vec::new();
    for part in index.parts() {
        let mesh = part.mesh();
        for p in &mesh.positions {
            ext = ext.max(p.x.abs()).max(p.y.abs()).max(p.z.abs());
        }
        let stride = mesh.boundaries.len().div_ceil(40).max(1);
        for boundary in mesh.boundaries.iter().step_by(stride) {
            let pts: Vec<Point3<f64>> = boundary
                .points
                .iter()
                .map(|&i| mesh.positions[i as usize])
                .collect();
            if let Some(&first) = pts.first() {
                targets.push(first);
            }
            if let [a, b, ..] = pts[..] {
                targets.push(Point3::new(
                    (a.x + b.x) * 0.5,
                    (a.y + b.y) * 0.5,
                    (a.z + b.z) * 0.5,
                ));
            }
        }
    }
    let reach = 4.0 * ext.max(1e-3);
    let mut rays = Vec::new();
    for at in targets {
        for dir in dirs() {
            rays.push((
                Ray {
                    origin: at - dir * reach,
                    dir,
                },
                reach,
            ));
        }
    }
    rays
}

/// `Pruned == Every` over the tie-break aim, and the aim's own shape.
fn tie_sweep(
    name: &str,
    step: &str,
    index: &PickIndex,
    eval: &Evaluation<f64>,
    table: &mut TieTable,
) {
    let parts = flatten(index);
    for (ray, reach) in tie_rays_for(index) {
        table.rays += 1;
        let exhaustive = winner(&every(&parts, &ray));
        let door = index
            .pick(eval, &ray)
            .expect("the index is of this evaluation");
        let same = match (door.as_ref(), exhaustive) {
            (None, None) => true,
            (Some(d), Some(e)) => {
                (d.t.to_bits(), d.t_lo.to_bits(), d.t_hi.to_bits())
                    == (
                        e.span.t.to_bits(),
                        e.span.t_lo.to_bits(),
                        e.span.t_hi.to_bits(),
                    )
            }
            _ => false,
        };
        if !same {
            table.pruned_differs += 1;
            if table.examples.len() < 8 {
                table.examples.push(format!(
                    "{name}/{step}: PRUNED DIFFERS {:?} reach {reach}: door {:?} exhaustive {:?}",
                    ray.dir,
                    door.as_ref().map(|d| d.t),
                    exhaustive.map(|e| e.span.t)
                ));
            }
        }
        match exhaustive {
            None => table.beyond_or_miss += 1,
            Some(win) => {
                if win.span.t > reach + 1e-9 {
                    table.beyond_or_miss += 1;
                }
                if widest_bound(&parts, &ray, &win) >= 1.0 {
                    table.wide_winners += 1;
                }
            }
        }
    }
}

/// The wide aim: the door against `main`'s rounded-`t` order over the
/// same candidates.
fn wide_sweep(
    name: &str,
    step: &str,
    index: &PickIndex,
    eval: &Evaluation<f64>,
    table: &mut WideTable,
) {
    let parts = flatten(index);
    let mut ext = 0.0f64;
    for part in index.parts() {
        for p in &part.mesh().positions {
            ext = ext.max(p.x.abs()).max(p.y.abs()).max(p.z.abs());
        }
    }
    let reaches = [1.48, 3.0, 4.0 * ext.max(1e-3)];
    for part in index.parts() {
        let positions = &part.mesh().positions;
        let stride = positions.len().div_ceil(1500).max(1);
        for v in positions.iter().step_by(stride) {
            for dir in dirs() {
                for reach in reaches {
                    let ray = Ray {
                        origin: *v - dir * reach,
                        dir,
                    };
                    table.rays += 1;
                    let main = by_rounded_t(&every(&parts, &ray));
                    let here = index
                        .pick(eval, &ray)
                        .expect("the index is of this evaluation");
                    let aimed_main = main.is_some_and(|s| (s.span.t - reach).abs() < 1e-9);
                    let aimed_here = here.as_ref().is_some_and(|h| (h.t - reach).abs() < 1e-9);
                    table.aimed_main += usize::from(aimed_main);
                    table.aimed_interval += usize::from(aimed_here);
                    table.aim_gained += usize::from(aimed_here && !aimed_main);
                    if aimed_main && !aimed_here {
                        table.aim_lost += 1;
                        if table.aim_lost_examples.len() < 8 {
                            table.aim_lost_examples.push(format!(
                                "{name}/{step}: AIM LOST {dir:?} through {v:?} reach {reach}: \
                                 main {:?} door {:?}",
                                main.map(|s| s.span.t),
                                here.as_ref().map(|h| h.t)
                            ));
                        }
                    }
                    if let (Some(m), Some(h)) = (main, here.as_ref())
                        && m.span.t.to_bits() != h.t.to_bits()
                    {
                        table.moved += 1;
                        if h.t > m.span.t {
                            table.moved_farther += 1;
                        }
                    }
                }
            }
        }
    }
}

/// Every landing the aims run over: the gallery ring at open and after
/// its bump, then each parametric corpus document the same way. ONE
/// pass — both aims run per landing, because the landings are what
/// this file pays for.
fn over_every_landing(mut sweep: impl FnMut(&str, &str, &PickIndex, &Evaluation<f64>)) {
    let tol = Tol::witness();
    {
        let text = common::gallery_ring_at(tol);
        let loaded = pncad::document::load(&text, tol).expect("the gallery ring loads");
        let doc = loaded.snapshot;
        let bump = ring_bump(&doc);
        let mut session = DocSession::inline(doc, tol);
        session.pump();
        let index = fresh_index(&session);
        sweep(
            "gallery_ring",
            "open",
            &index,
            session.landed_pair().expect("a landed pair").1,
        );
        let outcome = session.perform(bump);
        assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
        session.pump();
        let index = fresh_index(&session);
        sweep(
            "gallery_ring",
            "the first edit",
            &index,
            session.landed_pair().expect("a landed pair").1,
        );
    }
    for doc in corpus::documents() {
        let Some(bump) = bump_op(&doc) else {
            continue;
        };
        let mut session = DocSession::inline(doc.doc.clone(), tol);
        session.pump();
        if session.evaluation().is_none() {
            continue;
        }
        let index = fresh_index(&session);
        sweep(
            doc.name,
            "open",
            &index,
            session.landed_pair().expect("a landed pair").1,
        );
        let outcome = session.perform(bump);
        if outcome.refusal.is_some() {
            continue;
        }
        session.pump();
        let index = fresh_index(&session);
        sweep(
            doc.name,
            "the first edit",
            &index,
            session.landed_pair().expect("a landed pair").1,
        );
    }
}

/// **The door answers what an exhaustive walk answers, and the closed
/// acceptance loses no aimed vertex.**
///
/// One pass over every landing, both aims.
///
/// `pruned_differs` is the property the early-out's proof claims:
/// `PickIndex::pick` runs `pick_face`, whose traversal skips a
/// candidate when the running bound is below its box entry by the
/// derived margin, and the exhaustive walk here runs the same two
/// doors over EVERY candidate box the ray meets. The two answers agree
/// on every ray or the margin is wrong.
///
/// `aim_lost` is the column the ruling read for the acceptance. It is
/// `0` and it can red: a door that reordered the candidates in a way
/// that answered a nearer face would lose aims here.
///
/// The counts are printed. Two are asserted at their value and the
/// third (`wide_winners`) is disclosed above as structurally zero.
#[test]
fn the_door_answers_the_exhaustive_walk_and_keeps_every_aimed_vertex() {
    let mut tie = TieTable::default();
    let mut wide = WideTable::default();
    over_every_landing(|name, step, index, eval| {
        tie_sweep(name, step, index, eval, &mut tie);
        wide_sweep(name, step, index, eval, &mut wide);
    });
    println!("# EDIT-PICK3 tie-break aim: {tie:#?}");
    println!("# EDIT-PICK3 wide aim: {wide:#?}");
    assert!(
        tie.rays > 19_000,
        "the tie-break aim is the whole corpus's, not a subset: {} rays",
        tie.rays
    );
    assert!(
        wide.rays > 400_000,
        "the wide aim is the whole corpus's: {} rays",
        wide.rays
    );
    assert_eq!(
        tie.pruned_differs, 0,
        "the door's answer differs from the exhaustive walk's on {} of {} rays — the early-out \
         is pruning a candidate the certified tie keeps",
        tie.pruned_differs, tie.rays
    );
    assert_eq!(
        tie.wide_winners, 0,
        "{} winners carry a barycentric bound of 1 or more, which INFORM refuses",
        tie.wide_winners
    );
    assert_eq!(
        wide.aim_lost, 0,
        "the door loses {} aimed vertices main's rounded-t order answered",
        wide.aim_lost
    );
    assert!(
        wide.aim_gained > 0,
        "the interval order gains no aimed vertex over main's, which is what makes it more than \
         a re-spelling: {wide:#?}"
    );
}
