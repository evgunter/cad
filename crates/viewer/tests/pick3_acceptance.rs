//! MEASUREMENT (EDIT-PICK3): the acceptance the `t`-interval ruling
//! left to the unit — **closed ∧ INFORM stays, or MEET ∧ INFORM
//! returns** — measured over both of the door's aims under the
//! interval order, and asserted at the outcome the measurement found.
//!
//! EDIT-PICK2 measured MEET and could not take it: MEET admits a
//! barycentric outside `[0, 1]`, the hit point `a + u·e1 + v·e2` then
//! left the closed triangle, and the traversal's early-out stopped
//! before a candidate that would have won — 2 rays of the tie-break
//! aim's 19 296 answered differently with the early-out than without
//! it. The clamp this unit adds is supposed to remove that mechanism.
//! This probe is what says whether it did, and at what cost to the
//! other three columns.
//!
//! The two aims are `review_pick_r2`'s and `review_pick2_r1`'s, whose
//! private helpers are restated here so the measurement does not edit
//! the files it is measured against.

#![allow(clippy::expect_used, clippy::panic, clippy::unwrap_used)]

use bvh::{Aabb, Bvh, Ray};
use editor_core::resolve::{TSpan, crossing, ray_triangle};
use editor_core::{Dimension, DocEdit, Expr, ProfileDoc, RecipeNodeId, SlotId, unparse};
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

/// The acceptance the ruling left open.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Acceptance {
    /// The landed one: the barycentric's rounded value inside the
    /// closed range, and its interval informing.
    Closed,
    /// The candidate: the barycentric's interval REACHING the closed
    /// range, and informing.
    Meet,
}

/// `ray_triangle` under `acceptance`. `Closed` IS the door, called;
/// `Meet` is the door's own derivation with its comparison widened,
/// restated here because what this file measures is whether to ship
/// it. The clamp and the width are spelled the same way as
/// `ray_triangle` and `t_span`, which is what makes the two columns
/// comparable — only `admits` moves.
fn under(acceptance: Acceptance, ray: &Ray, tri: &[Point3<f64>; 3]) -> Option<TSpan> {
    if acceptance == Acceptance::Closed {
        return ray_triangle(ray, tri);
    }
    let c = crossing(ray, tri)?;
    let meets = |x: f64, err: f64| x + err >= 0.0 && x - err <= 1.0;
    let informs = |x: f64, err: f64| x - err > 0.0 || x + err < 1.0;
    if !c
        .barycentrics
        .iter()
        .all(|&(x, err)| meets(x, err) && informs(x, err))
    {
        return None;
    }
    let [(u, err_u), (v, err_v), _] = c.barycentrics;
    let e1: Vec3<f64> = tri[1] - tri[0];
    let e2: Vec3<f64> = tri[2] - tri[0];
    let u = u.clamp(0.0, 1.0);
    let v = v.clamp(0.0, 1.0 - u);
    let a = tri[0];
    let hit = a + e1 * u + e2 * v;
    let dd = ray.dir.dot(ray.dir);
    let t = (hit - ray.origin).dot(ray.dir) / dd;
    let from_barycentrics = (err_u * e1.norm() + err_u.max(err_v) * e2.norm()) / ray.dir.norm();
    let (o, d) = (ray.origin, ray.dir);
    let m = (a.x.abs() + (e1.x * u).abs() + (e2.x * v).abs() + o.x.abs()) * d.x.abs()
        + (a.y.abs() + (e1.y * u).abs() + (e2.y * v).abs() + o.y.abs()) * d.y.abs()
        + (a.z.abs() + (e1.z * u).abs() + (e2.z * v).abs() + o.z.abs()) * d.z.abs();
    let half = from_barycentrics + 8.0 * f64::EPSILON * m / dd;
    let span = TSpan {
        t,
        t_lo: (t - half).next_down(),
        t_hi: (t + half).next_up(),
    };
    (span.t >= 0.0 && span.t.is_finite() && span.t_lo.is_finite() && span.t_hi.is_finite())
        .then_some(span)
}

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

impl Seen {
    fn key(seen: Option<Self>) -> Option<(usize, usize, u64)> {
        seen.map(|s| (s.part, s.item, s.span.t.to_bits()))
    }
}

/// `pick_face`'s order, restated: the candidates no other candidate
/// PRECEDES, then the narrower interval, then position.
fn by_interval(seen: &[Seen]) -> Option<Seen> {
    let lowest_hi = seen
        .iter()
        .map(|s| s.span.t_hi)
        .fold(f64::INFINITY, f64::min);
    seen.iter()
        .filter(|s| s.span.t_lo <= lowest_hi)
        .copied()
        .reduce(|b, c| {
            if c.span.width() < b.span.width()
                || (c.span.width() == b.span.width() && (c.part, c.item) < (b.part, b.item))
            {
                c
            } else {
                b
            }
        })
}

/// `main`'s order: the rounded `t`, then position. Over the closed
/// acceptance this is `main`'s answer to the bit — the clamp is a
/// no-op where every admitted barycentric is already in range.
fn by_rounded_t(seen: &[Seen]) -> Option<Seen> {
    seen.iter().copied().reduce(|b, c| {
        if c.span.t < b.span.t || (c.span.t == b.span.t && (c.part, c.item) < (b.part, b.item)) {
            c
        } else {
            b
        }
    })
}

/// Every admitted candidate on `ray`, in tree order. `prune` runs the
/// traversal's early-out — the walk `pick_face` runs — against the
/// smallest upper end seen.
fn admitted(parts: &[FlatPart], ray: &Ray, acceptance: Acceptance, prune: bool) -> Vec<Seen> {
    let mut lowest_hi = f64::INFINITY;
    let mut seen = Vec::new();
    for (part, flat) in parts.iter().enumerate() {
        for cand in flat.tree.ray(ray) {
            if prune && lowest_hi < cand.t_enter {
                break;
            }
            let Some(span) = under(acceptance, ray, &flat.corners[cand.item]) else {
                continue;
            };
            lowest_hi = lowest_hi.min(span.t_hi);
            seen.push(Seen {
                part,
                item: cand.item,
                span,
            });
        }
    }
    seen
}

/// The widest barycentric bound the winning candidate carries.
fn widest_bound(parts: &[FlatPart], ray: &Ray, win: &Seen) -> f64 {
    crossing(ray, &parts[win.part].corners[win.item])
        .expect("an admitted candidate has a certified determinant")
        .barycentrics
        .iter()
        .fold(0.0f64, |w, &(_, err)| w.max(err))
}

/// The three-rule table over the tie-break aim, one acceptance.
#[derive(Default, Debug)]
struct TieTable {
    rays: usize,
    /// The aimed point is not answered: the winner is beyond it, or
    /// the ray misses.
    beyond_or_miss: usize,
    /// The early-out changed the answer.
    pruned_differs: usize,
    /// The winner carries a barycentric bound of `1` or more.
    wide_winners: usize,
    examples: Vec<String>,
}

/// The wide aim's contingency, one acceptance against `main`'s order
/// over the same candidates.
#[derive(Default, Debug)]
struct WideTable {
    rays: usize,
    moved: usize,
    moved_farther: usize,
    lost_entirely: usize,
    aimed_main: usize,
    aimed_interval: usize,
    aim_lost: usize,
    aim_gained: usize,
    aim_lost_examples: Vec<String>,
    aim_gained_examples: Vec<String>,
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

fn tie_sweep(
    name: &str,
    step: &str,
    index: &PickIndex,
    acceptance: Acceptance,
    table: &mut TieTable,
) {
    let parts = flatten(index);
    for (ray, reach) in tie_rays_for(index) {
        table.rays += 1;
        let every = admitted(&parts, &ray, acceptance, false);
        let pruned = admitted(&parts, &ray, acceptance, true);
        let win = by_interval(&every);
        if Seen::key(by_interval(&pruned)) != Seen::key(win) {
            table.pruned_differs += 1;
            if table.examples.len() < 8 {
                table.examples.push(format!(
                    "{name}/{step} {acceptance:?}: PRUNED DIFFERS {:?} reach {reach}: pruned \
                     {:?} every {:?}",
                    ray.dir,
                    Seen::key(by_interval(&pruned)),
                    Seen::key(win)
                ));
            }
        }
        match win {
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

fn wide_sweep(name: &str, step: &str, index: &PickIndex, tables: &mut [WideTable; 2]) {
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
                    // Two walks per ray, not four: `main`'s answer is
                    // the CLOSED acceptance under the rounded order,
                    // so it reads the same candidates the closed
                    // column does.
                    let closed = admitted(&parts, &ray, Acceptance::Closed, false);
                    let meet = admitted(&parts, &ray, Acceptance::Meet, false);
                    let main = by_rounded_t(&closed);
                    for (table, seen) in tables.iter_mut().zip([&closed, &meet]) {
                        table.rays += 1;
                        let here = by_interval(seen);
                        let aimed =
                            |s: Option<Seen>| s.is_some_and(|s| (s.span.t - reach).abs() < 1e-9);
                        let (am, ah) = (aimed(main), aimed(here));
                        table.aimed_main += usize::from(am);
                        table.aimed_interval += usize::from(ah);
                        table.aim_lost += usize::from(am && !ah);
                        table.aim_gained += usize::from(ah && !am);
                        if ah && !am && table.aim_gained_examples.len() < 8 {
                            table.aim_gained_examples.push(format!(
                                "{name}/{step}: AIM GAINED {dir:?} through {v:?} reach {reach}: \
                                 main {:?} interval {:?}",
                                main.map(|s| s.span.t),
                                here.map(|s| s.span.t)
                            ));
                        }
                        if am && !ah && table.aim_lost_examples.len() < 8 {
                            table.aim_lost_examples.push(format!(
                                "{name}/{step}: AIM LOST {dir:?} through {v:?} reach {reach}: \
                                 main {:?} interval {:?}",
                                main.map(|s| s.span.t),
                                here.map(|s| s.span.t)
                            ));
                        }
                        match (main, here) {
                            (Some(_), None) => table.lost_entirely += 1,
                            (Some(m), Some(h)) if m.span.t.to_bits() != h.span.t.to_bits() => {
                                table.moved += 1;
                                if h.span.t > m.span.t {
                                    table.moved_farther += 1;
                                }
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
    }
}

/// Every landing the two aims run over: the gallery ring at open and
/// after its bump, then each parametric corpus document the same way.
fn over_every_landing(mut sweep: impl FnMut(&str, &str, &PickIndex)) {
    let tol = Tol::witness();
    {
        let text = common::gallery_ring_at(tol);
        let loaded = pncad::document::load(&text, tol).expect("the gallery ring loads");
        let doc = loaded.snapshot;
        let bump = ring_bump(&doc);
        let mut session = DocSession::inline(doc, tol);
        session.pump();
        sweep("gallery_ring", "open", &fresh_index(&session));
        let outcome = session.perform(bump);
        assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
        session.pump();
        sweep("gallery_ring", "the first edit", &fresh_index(&session));
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
        sweep(doc.name, "open", &fresh_index(&session));
        let outcome = session.perform(bump);
        if outcome.refusal.is_some() {
            continue;
        }
        session.pump();
        sweep(doc.name, "the first edit", &fresh_index(&session));
    }
}

/// **The three-rule table, both acceptances, over the tie-break aim.**
///
/// The rule the ruling set for taking MEET: `0 Pruned ≠ Every` and no
/// lost aim. The clamp removes the mechanism EDIT-PICK2 measured — an
/// admitted candidate whose projection preceded its box entry — and
/// this row is what says so, over every landing of the corpus and the
/// gallery ring.
///
/// What it asserts is the OUTCOME, not the counts: the early-out is
/// order-independent under both acceptances (that is the property the
/// door's docs claim, and it is the column that decided EDIT-PICK2),
/// and no winner under either acceptance carries a barycentric bound
/// of `1` or more, which INFORM makes impossible and is asserted here
/// rather than pinned as a number. The two `beyond_or_miss` counts are
/// printed, not pinned: they are the corpus's shape, and the row above
/// them is `work/edit/pick-closed-acceptance-loses-a-graze-to-rounding`.
#[test]
fn the_three_rule_table_over_the_tie_break_aim_for_both_acceptances() {
    for acceptance in [Acceptance::Closed, Acceptance::Meet] {
        let mut table = TieTable::default();
        over_every_landing(|name, step, index| {
            tie_sweep(name, step, index, acceptance, &mut table)
        });
        println!("# EDIT-PICK3 tie-break aim, {acceptance:?}: {table:#?}");
        assert!(
            table.rays > 19_000,
            "{acceptance:?}: the aim is the whole corpus's, not a subset: {} rays",
            table.rays
        );
        assert_eq!(
            table.pruned_differs, 0,
            "{acceptance:?}: the traversal's early-out changed the answer on {} of {} rays — the \
             clamp is what is supposed to make it order-independent",
            table.pruned_differs, table.rays
        );
        assert_eq!(
            table.wide_winners, 0,
            "{acceptance:?}: {} winners carry a barycentric bound of 1 or more, which INFORM \
             refuses",
            table.wide_winners
        );
    }
}

/// **The wide aim, both acceptances, against `main`'s order over the
/// same candidates.** `main` answered the rounded `t`; the door
/// answers the interval. The column the ruling reads is `aim_lost` —
/// an aimed vertex `main` answered and this acceptance does not — and
/// it is what decides the acceptance: `0` for the closed comparison,
/// and hundreds for MEET, which is why the closed comparison stays.
///
/// MEET's losses are its own mechanism, not the interval's. It admits
/// a candidate whose barycentrics are outside the range, the clamp
/// then places the answer on the nearest point OF that triangle, and
/// that point is a real point of the mesh at a smaller `t` than the
/// vertex the ray was aimed at — so the ray answers a face it passes
/// beside instead of the one it passes through. The clamp is what
/// makes those answers points of the triangle at all, which is why
/// MEET now costs nothing in the early-out column and everything in
/// this one.
///
/// The counts are printed, not pinned: they are the corpus's shape and
/// the PR body carries them with their date. What is asserted is the
/// two facts the ruling reads — that the closed comparison loses no
/// aim and answers on every ray `main` answered, and that MEET loses
/// aims, which is the measurement that keeps the closed comparison.
#[test]
fn the_wide_aim_for_both_acceptances() {
    let mut tables = [WideTable::default(), WideTable::default()];
    over_every_landing(|name, step, index| wide_sweep(name, step, index, &mut tables));
    let [closed, meet] = &tables;
    println!("# EDIT-PICK3 wide aim, Closed: {closed:#?}");
    println!("# EDIT-PICK3 wide aim, Meet: {meet:#?}");
    assert!(
        closed.rays > 400_000,
        "the aim is the whole corpus's: {} rays",
        closed.rays
    );
    assert_eq!(
        closed.lost_entirely, 0,
        "the interval order answers nothing on {} rays main answered — it reorders the same          candidates, it does not refuse them",
        closed.lost_entirely
    );
    assert_eq!(
        closed.aim_lost, 0,
        "the closed comparison under the interval order loses {} aimed vertices main answered",
        closed.aim_lost
    );
    assert!(
        meet.aim_lost > 0,
        "MEET lost no aim, which is the one thing that would reopen the acceptance the unit \
         measured closed: {meet:#?}"
    );
}
