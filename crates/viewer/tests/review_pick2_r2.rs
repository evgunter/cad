//! Review probes for EDIT-PICK2 (lane `pick2-r2`): the wide aim of
//! `review_pick_r2` re-walked under BOTH acceptances — `main`'s closed
//! comparison alone, restated here from the door's own intervals, and
//! the head's closed comparison ∧ INFORM (`ray_triangle`) — so the
//! tally's `+12` has a per-ray account, and rays whose answer moved
//! hit → miss or hit → another hit are counted, which the tally's four
//! columns cannot see. Also prints the ring example's numbers.
//!
//! Not a row: a one-shot measurement (`--nocapture`) with no pin.

#![allow(clippy::expect_used, clippy::panic, clippy::unwrap_used)]

use bvh::{Aabb, Bvh, Ray};
use editor_core::resolve::{crossing, ray_triangle};
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

/// `review_pick_r2`'s `ring_bump`, restated.
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

/// `main`'s acceptance (PR #2721's tree): the closed comparison on the
/// rounded `u`, `v`, `u + v` alone, with `t` from the hit point's
/// projection — restated over the head's own intervals so the only
/// difference from `ray_triangle` is INFORM.
fn ray_triangle_closed_only(ray: &Ray, tri: &[Point3<f64>; 3]) -> Option<f64> {
    let intervals = crossing(ray, tri)?.barycentrics;
    if !intervals.iter().all(|&(x, _)| (0.0..=1.0).contains(&x)) {
        return None;
    }
    let [(u, _), (v, _), _] = intervals;
    let e1: Vec3<f64> = tri[1] - tri[0];
    let e2: Vec3<f64> = tri[2] - tri[0];
    let hit = tri[0] + e1 * u + e2 * v;
    let t = (hit - ray.origin).dot(ray.dir) / ray.dir.dot(ray.dir);
    (t >= 0.0 && t.is_finite()).then_some(t)
}

#[derive(Default, Debug)]
struct Diff {
    rays: usize,
    grazes_main: usize,
    grazes_head: usize,
    same: usize,
    hit_to_miss: usize,
    miss_to_hit: usize,
    hit_to_other: usize,
    /// Of the rays whose answer moved: the head's answer is at the aim
    /// (|t − reach| < 1e-9) and main's was not.
    moved_to_aim: usize,
    /// Of the rays whose answer moved: main's was at the aim, the
    /// head's is not.
    moved_off_aim: usize,
    /// Candidates the head refuses at INFORM that main accepted.
    inform_refusals: usize,
    examples: Vec<String>,
}

fn sweep(name: &str, step: &str, index: &PickIndex, diff: &mut Diff) {
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
                    diff.rays += 1;
                    let ray = Ray {
                        origin: *v - dir * reach,
                        dir,
                    };
                    let mut best_main: Option<f64> = None;
                    let mut best_head: Option<f64> = None;
                    for flat in &parts {
                        for cand in flat.tree.ray(&ray) {
                            let tri = &flat.corners[cand.item];
                            let m = ray_triangle_closed_only(&ray, tri);
                            let h = ray_triangle(&ray, tri);
                            if m.is_some() && h.is_none() {
                                diff.inform_refusals += 1;
                            }
                            if let Some(t) = m
                                && best_main.is_none_or(|b| t < b)
                            {
                                best_main = Some(t);
                            }
                            if let Some(t) = h
                                && best_head.is_none_or(|b| t < b)
                            {
                                best_head = Some(t);
                            }
                        }
                    }
                    let at_aim = |t: Option<f64>| t.is_some_and(|t| (t - reach).abs() < 1e-9);
                    if at_aim(best_main) {
                        diff.grazes_main += 1;
                    }
                    if at_aim(best_head) {
                        diff.grazes_head += 1;
                    }
                    match (best_main, best_head) {
                        (None, None) => diff.same += 1,
                        (Some(a), Some(b)) if a.to_bits() == b.to_bits() => diff.same += 1,
                        (Some(_), None) => {
                            diff.hit_to_miss += 1;
                            diff.examples.push(format!(
                                "{name} after {step}: {dir:?} through {v:?} reach {reach}: main {best_main:?} → head miss"
                            ));
                        }
                        (None, Some(_)) => diff.miss_to_hit += 1,
                        (Some(a), Some(b)) => {
                            diff.hit_to_other += 1;
                            if at_aim(best_head) && !at_aim(best_main) {
                                diff.moved_to_aim += 1;
                            }
                            if at_aim(best_main) && !at_aim(best_head) {
                                diff.moved_off_aim += 1;
                            }
                            if diff.examples.len() < 40 {
                                diff.examples.push(format!(
                                    "{name} after {step}: {dir:?} through {v:?} reach {reach}: main t={a} → head t={b}"
                                ));
                            }
                        }
                    }
                }
            }
        }
    }
}

#[test]
fn the_wide_aim_under_both_acceptances() {
    let tol = Tol::witness();
    let mut diff = Diff::default();
    {
        let text = common::gallery_ring_at(tol);
        let loaded = pncad::document::load(&text, tol).expect("the gallery ring loads");
        let doc = loaded.snapshot;
        let bump = ring_bump(&doc);
        let mut session = DocSession::inline(doc, tol);
        session.pump();
        sweep("gallery_ring", "open", &fresh_index(&session), &mut diff);
        let outcome = session.perform(bump);
        assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
        session.pump();
        let index = fresh_index(&session);
        sweep("gallery_ring", "the first edit", &index, &mut diff);
        ring_example(&index);
    }
    for c in corpus::documents() {
        let Some(bump) = bump_op(&c) else {
            continue;
        };
        let mut session = DocSession::inline(c.doc.clone(), tol);
        session.pump();
        if session.evaluation().is_none() {
            continue;
        }
        let index = fresh_index(&session);
        sweep(c.name, "open", &index, &mut diff);
        if c.name == "cut_cylinder" {
            // The one physical ray of the wide aim whose at-the-aim
            // answer INFORM loses: +z up a ruling of the wall.
            let vertex = Point3::new(
                -0.484_291_580_564_315_5,
                0.124_344_943_582_427_67,
                0.059_515_284_073_164_7,
            );
            dump_candidates(
                "cut_cylinder open, +z through the ruling vertex",
                &index,
                vertex,
                Vec3::new(0.0, 0.0, 1.0),
                1.48,
            );
        }
        let outcome = session.perform(bump);
        if outcome.refusal.is_some() {
            continue;
        }
        session.pump();
        sweep(c.name, "the first edit", &fresh_index(&session), &mut diff);
    }
    let examples = std::mem::take(&mut diff.examples);
    println!("# review_pick2_r2 differential: {diff:?}");
    for e in &examples {
        println!("#   {e}");
    }
}

/// The ring example's numbers, from the door's own doors.
fn ring_example(index: &PickIndex) {
    let vertex = Point3::new(0.245_196_320_100_807_58, 0.0, 0.048_772_580_504_032_18);
    dump_candidates(
        "ring example",
        index,
        vertex,
        Vec3::new(0.0, -1.0, 0.0),
        1.48,
    );
}

/// Every certified candidate of the axis ray through `vertex`: its
/// determinant, conditioning, the three intervals, the door's `t`,
/// and which corner (if any) IS the aimed vertex.
fn dump_candidates(
    label: &str,
    index: &PickIndex,
    vertex: Point3<f64>,
    dir: Vec3<f64>,
    reach: f64,
) {
    let parts = flatten(index);
    let ray = Ray {
        origin: vertex - dir * reach,
        dir,
    };
    for flat in &parts {
        for cand in flat.tree.ray(&ray) {
            let tri = &flat.corners[cand.item];
            let Some(det) = crossing(&ray, tri).map(|c| c.det) else {
                continue;
            };
            let e1: Vec3<f64> = tri[1] - tri[0];
            let e2: Vec3<f64> = tri[2] - tri[0];
            let cond = det.abs() / (e1.norm() * e2.norm() * ray.dir.norm());
            let iv = crossing(&ray, tri).map(|c| c.barycentrics);
            let t = ray_triangle(&ray, tri);
            let corner = tri.iter().position(|p| {
                (p.x.to_bits(), p.y.to_bits(), p.z.to_bits())
                    == (vertex.x.to_bits(), vertex.y.to_bits(), vertex.z.to_bits())
            });
            println!(
                "# {label}: item {} det {det:e} conditioning {cond:e} intervals {iv:?} t {t:?} aimed vertex is corner {corner:?}",
                cand.item
            );
        }
    }
}
