//! REVIEW PROBE (lane `pick2-r1`, EDIT-PICK2): what the INFORM half
//! costs and buys over the wide aim, measured against `main`'s
//! acceptance restated here.
//!
//! `review_pick_r2`'s tally moved one column (`141 094 → 141 106`) and
//! the claim is that the move is a GAIN. The tally cannot see a loss:
//! its `grazes` column counts rays whose best accepted `t` is within
//! `1e-9` of the aimed reach, so a ray that answered a DIFFERENT face
//! before and answers nothing now, or answers a farther face now, is
//! invisible to it. This probe walks the same aim with both
//! acceptances side by side and prints the full contingency.

#![allow(clippy::expect_used, clippy::panic, clippy::unwrap_used)]

use bvh::{Aabb, Bvh, Ray};
use editor_core::resolve::ray_triangle;
use editor_core::{Dimension, DocEdit, Expr, ProfileDoc, RecipeNodeId, SlotId, unparse};
use pncad::geom_core::{Point3, Tol, Vec3};
use viewer::pickindex::{PickIndex, PictureKey};
use viewer::scene::DisplayTolerance;
use viewer::session::{DocSession, SessionOp};

use crate::common;
use crate::corpus;

// `review_pick_r2`'s private helpers, restated so this probe does not
// edit the file under review.
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

/// `main`'s exact test, restated verbatim from
/// `git show e97a13fdb:crates/editor-core/src/resolve/pick.rs`: the
/// closed comparison alone, no INFORM half.
fn main_ray_triangle(ray: &Ray, tri: &[Point3<f64>; 3]) -> Option<f64> {
    let e1: Vec3<f64> = tri[1] - tri[0];
    let e2: Vec3<f64> = tri[2] - tri[0];
    let p = ray.dir.cross(e2);
    let det = e1.dot(p);
    let m = e1.x.abs() * (ray.dir.y.abs() * e2.z.abs() + ray.dir.z.abs() * e2.y.abs())
        + e1.y.abs() * (ray.dir.z.abs() * e2.x.abs() + ray.dir.x.abs() * e2.z.abs())
        + e1.z.abs() * (ray.dir.x.abs() * e2.y.abs() + ray.dir.y.abs() * e2.x.abs());
    let bound = 3.0 * f64::EPSILON * m;
    // `main` spelled this `(det.abs() > bound).then_some(det)?`; the
    // affirmative form is the same test and a NaN fails it either way.
    if det.abs() <= bound || det.is_nan() {
        return None;
    }
    let inv = 1.0 / det;
    let s = ray.origin - tri[0];
    let u = s.dot(p) * inv;
    if !(0.0..=1.0).contains(&u) {
        return None;
    }
    let q = s.cross(e1);
    let v = ray.dir.dot(q) * inv;
    if !(v >= 0.0 && u + v <= 1.0) {
        return None;
    }
    let hit = tri[0] + e1 * u + e2 * v;
    let t = (hit - ray.origin).dot(ray.dir) / ray.dir.dot(ray.dir);
    (t >= 0.0 && t.is_finite()).then_some(t)
}

#[derive(Default, Debug)]
struct Contingency {
    rays: usize,
    /// main answers, the landed door answers nothing at all.
    lost_entirely: usize,
    /// both answer, at different `t` bits.
    moved: usize,
    /// both answer, landed `t` is strictly FARTHER.
    moved_farther: usize,
    /// landed answers where main did not — impossible by inclusion.
    gained_impossible: usize,
    /// answered within 1e-9 of the aimed reach, each door.
    aimed_main: usize,
    aimed_landed: usize,
    /// rays aimed-answered by main and not by the landed door.
    aim_lost: usize,
    aim_gained: usize,
    /// of `moved`, those whose relative move exceeds 1e-9 — a
    /// different surface, not a rounding of the same one.
    moved_materially: usize,
    widest_relative_move: f64,
    examples: Vec<String>,
    aim_lost_examples: Vec<String>,
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

fn sweep(name: &str, step: &str, index: &PickIndex, c: &mut Contingency) {
    let parts = flatten(index);
    let mut ext = 0.0f64;
    for part in index.parts() {
        for p in &part.mesh().positions {
            ext = ext.max(p.x.abs()).max(p.y.abs()).max(p.z.abs());
        }
    }
    let reaches = [1.48, 3.0, 4.0 * ext.max(1e-3)];
    let dirs = [
        Vec3::new(1.0, 0.0, 0.0),
        Vec3::new(-1.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        Vec3::new(0.0, -1.0, 0.0),
        Vec3::new(0.0, 0.0, 1.0),
        Vec3::new(0.0, 0.0, -1.0),
    ];
    for part in index.parts() {
        let positions = &part.mesh().positions;
        let stride = positions.len().div_ceil(1500).max(1);
        for v in positions.iter().step_by(stride) {
            for dir in dirs {
                for reach in reaches {
                    c.rays += 1;
                    let ray = Ray {
                        origin: *v - dir * reach,
                        dir,
                    };
                    let mut best_landed: Option<f64> = None;
                    let mut best_main: Option<f64> = None;
                    for flat in &parts {
                        for cand in flat.tree.ray(&ray) {
                            let tri = &flat.corners[cand.item];
                            if let Some(span) = ray_triangle(&ray, tri)
                                && best_landed.is_none_or(|b| span.t < b)
                            {
                                best_landed = Some(span.t);
                            }
                            if let Some(t) = main_ray_triangle(&ray, tri)
                                && best_main.is_none_or(|b| t < b)
                            {
                                best_main = Some(t);
                            }
                        }
                    }
                    let aimed = |t: Option<f64>| t.is_some_and(|t| (t - reach).abs() < 1e-9);
                    let (am, al) = (aimed(best_main), aimed(best_landed));
                    c.aimed_main += usize::from(am);
                    c.aimed_landed += usize::from(al);
                    c.aim_lost += usize::from(am && !al);
                    c.aim_gained += usize::from(al && !am);
                    if am && !al && c.aim_lost_examples.len() < 8 {
                        c.aim_lost_examples.push(format!(
                            "{name}/{step}: AIM LOST {dir:?} through {v:?} reach {reach}: main \
                             {best_main:?} landed {best_landed:?}"
                        ));
                    }
                    match (best_main, best_landed) {
                        (Some(_), None) => {
                            c.lost_entirely += 1;
                            if c.examples.len() < 12 {
                                c.examples.push(format!(
                                    "{name}/{step}: LOST {dir:?} through {v:?} reach {reach}: \
                                     main {:?} landed none",
                                    best_main
                                ));
                            }
                        }
                        (Some(m), Some(l)) if m.to_bits() != l.to_bits() => {
                            c.moved += 1;
                            if l > m {
                                c.moved_farther += 1;
                            }
                            let rel = ((l - m) / m.abs().max(f64::MIN_POSITIVE)).abs();
                            c.widest_relative_move = c.widest_relative_move.max(rel);
                            if rel > 1e-9 {
                                c.moved_materially += 1;
                            }
                            if rel > 1e-9 && c.examples.len() < 12 {
                                c.examples.push(format!(
                                    "{name}/{step}: MOVED {dir:?} through {v:?} reach {reach}: \
                                     main {m} landed {l}"
                                ));
                            }
                        }
                        (None, Some(_)) => c.gained_impossible += 1,
                        _ => {}
                    }
                }
            }
        }
    }
}

#[test]
fn what_the_inform_half_costs_and_buys_over_the_wide_aim() {
    let tol = Tol::witness();
    let mut c = Contingency::default();
    {
        let text = common::gallery_ring_at(tol);
        let loaded = pncad::document::load(&text, tol).expect("the gallery ring loads");
        let doc = loaded.snapshot;
        let bump = ring_bump(&doc);
        let mut session = DocSession::inline(doc, tol);
        session.pump();
        sweep("gallery_ring", "open", &fresh_index(&session), &mut c);
        let outcome = session.perform(bump);
        assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
        session.pump();
        sweep(
            "gallery_ring",
            "the first edit",
            &fresh_index(&session),
            &mut c,
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
        sweep(doc.name, "open", &fresh_index(&session), &mut c);
        let outcome = session.perform(bump);
        if outcome.refusal.is_some() {
            continue;
        }
        session.pump();
        sweep(doc.name, "the first edit", &fresh_index(&session), &mut c);
    }
    println!("# pick2-r1 contingency (wide aim): {c:#?}");
    assert_eq!(
        c.gained_impossible, 0,
        "the landed acceptance answered a ray main's did not — it is not a narrowing"
    );
}

// ---------------------------------------------------------------
// The tie-break aim, where the amendment's table reports `149 → 149`
// for "aimed rays answering beyond the aim or missing". A count is
// not a set: this measures the SET difference between the two doors.

#[derive(Default, Debug)]
struct TieAim {
    rays: usize,
    beyond_or_miss_main: usize,
    beyond_or_miss_landed: usize,
    /// aimed under main, beyond-or-miss under the landed door.
    newly_beyond_or_miss: usize,
    /// beyond-or-miss under main, aimed under the landed door.
    newly_aimed: usize,
    examples: Vec<String>,
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
        for dir in [
            Vec3::new(1.0, 0.0, 0.0),
            Vec3::new(-1.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
            Vec3::new(0.0, -1.0, 0.0),
            Vec3::new(0.0, 0.0, 1.0),
            Vec3::new(0.0, 0.0, -1.0),
        ] {
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

fn tie_sweep(name: &str, step: &str, index: &PickIndex, a: &mut TieAim) {
    let parts = flatten(index);
    for (ray, reach) in tie_rays_for(index) {
        a.rays += 1;
        let mut best_landed: Option<f64> = None;
        let mut best_main: Option<f64> = None;
        for flat in &parts {
            for cand in flat.tree.ray(&ray) {
                let tri = &flat.corners[cand.item];
                if let Some(span) = ray_triangle(&ray, tri)
                    && best_landed.is_none_or(|b| span.t < b)
                {
                    best_landed = Some(span.t);
                }
                if let Some(t) = main_ray_triangle(&ray, tri)
                    && best_main.is_none_or(|b| t < b)
                {
                    best_main = Some(t);
                }
            }
        }
        let beyond = |t: Option<f64>| !t.is_some_and(|t| t <= reach + 1e-9);
        let (bm, bl) = (beyond(best_main), beyond(best_landed));
        a.beyond_or_miss_main += usize::from(bm);
        a.beyond_or_miss_landed += usize::from(bl);
        if !bm && bl {
            a.newly_beyond_or_miss += 1;
            if a.examples.len() < 8 {
                a.examples.push(format!(
                    "{name}/{step}: NEWLY BEYOND {:?} reach {reach}: main {best_main:?} landed \
                     {best_landed:?}",
                    ray.dir
                ));
            }
        }
        if bm && !bl {
            a.newly_aimed += 1;
        }
    }
}

#[test]
fn the_tie_break_aims_unchanged_count_is_not_an_unchanged_set() {
    let tol = Tol::witness();
    let mut a = TieAim::default();
    {
        let text = common::gallery_ring_at(tol);
        let loaded = pncad::document::load(&text, tol).expect("the gallery ring loads");
        let doc = loaded.snapshot;
        let bump = ring_bump(&doc);
        let mut session = DocSession::inline(doc, tol);
        session.pump();
        tie_sweep("gallery_ring", "open", &fresh_index(&session), &mut a);
        let outcome = session.perform(bump);
        assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
        session.pump();
        tie_sweep(
            "gallery_ring",
            "the first edit",
            &fresh_index(&session),
            &mut a,
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
        tie_sweep(doc.name, "open", &fresh_index(&session), &mut a);
        let outcome = session.perform(bump);
        if outcome.refusal.is_some() {
            continue;
        }
        session.pump();
        tie_sweep(doc.name, "the first edit", &fresh_index(&session), &mut a);
    }
    println!("# pick2-r1 tie-break aim: {a:#?}");
    // The claim the probe was written to make: the amendment's
    // `149 -> 149` is the same SET, not two counts that happen to
    // agree. A ray that swapped sides would leave the total where it
    // is and still be a change in what the door answers on the graze
    // class — which is exactly the class this unit moves.
    assert_eq!(
        (a.newly_beyond_or_miss, a.newly_aimed),
        (0, 0),
        "aimed rays swapped sides between the two acceptances while the total stayed at {}: {a:#?}",
        a.beyond_or_miss_landed
    );
}
