//! Review probe for EDIT-PICK (PR 2721, lane `pick-r2`, frozen head
//! `428431ab`): the box-entry guard re-measured over the corpus with a
//! WIDER aim than the tie-break row — every mesh vertex (subsampled),
//! six axis directions, three reaches — asking whether a genuine
//! graze the guard refuses is always answered by a sibling triangle
//! (the PR's claim from 9 ★ rows), and re-taking the "moved answers"
//! count with the unguarded predicate (main's kernel) against the
//! guarded one on the same candidates.
//!
//! A LOSS is a ray whose unguarded nearest is the aimed vertex (a true
//! hit at exactly `reach`) and whose guarded nearest is beyond it or a
//! miss. Red = the guard turned a hit into a miss somewhere on the
//! corpus geometry, which the tie-break row's aim did not reach.

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

fn delta() -> DisplayTolerance {
    DisplayTolerance::new(2.0e-3).expect("a positive delta")
}

fn fresh_index(session: &DocSession) -> PickIndex {
    let (doc, eval) = session.landed_pair().expect("a landed pair");
    let generation = session
        .landed_generation()
        .expect("a landed evaluation has a generation");
    PickIndex::build(doc, eval, PictureKey::of(generation, delta()), session.tol())
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

/// `index_memo`'s `first_length_slot`, restated: the gallery ring's bump.
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
    patch: Vec<usize>,
}

fn flatten(index: &PickIndex) -> Vec<FlatPart> {
    index
        .parts()
        .iter()
        .map(|part| {
            let mesh = part.mesh();
            let mut corners = Vec::new();
            let mut patch = Vec::new();
            let mut boxes = Vec::new();
            for (pi, p) in mesh.patches.iter().enumerate() {
                for tri in &p.triangles {
                    let c = tri.map(|i| mesh.positions[i as usize]);
                    boxes.push(Aabb::from_points(c).expect("three points box"));
                    corners.push(c);
                    patch.push(pi);
                }
            }
            FlatPart {
                tree: Bvh::build(&boxes),
                corners,
                patch,
            }
        })
        .collect()
}

fn det(ray: &Ray, tri: &[Point3<f64>; 3]) -> f64 {
    let e1: Vec3<f64> = tri[1] - tri[0];
    let e2: Vec3<f64> = tri[2] - tri[0];
    e1.dot(ray.dir.cross(e2))
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Hit {
    t: f64,
    part: usize,
    item: usize,
    patch: usize,
}

/// Every candidate of every part tested, no early-out; the minimum of
/// `(t, part, item)`. `guarded` passes each candidate's own entry,
/// unguarded passes `−∞` (main's kernel). Also answers whether some
/// candidate's GENUINE graze at `reach` (|det| ≥ 1e-15, unguarded `t`
/// within 1e-9 of `reach`) was refused by the guard.
fn nearest(parts: &[FlatPart], ray: &Ray, guarded: bool, reach: f64) -> (Option<Hit>, bool) {
    let mut best: Option<Hit> = None;
    let mut genuine_refused = false;
    for (pi, flat) in parts.iter().enumerate() {
        for cand in flat.tree.ray(ray) {
            let tri = &flat.corners[cand.item];
            let floor = if guarded { cand.t_enter } else { f64::NEG_INFINITY };
            let un = ray_triangle(ray, tri, f64::NEG_INFINITY);
            let gu = ray_triangle(ray, tri, cand.t_enter);
            if matches!(un, Some(t) if (t - reach).abs() < 1e-9)
                && gu.is_none()
                && det(ray, tri).abs() >= 1e-15
            {
                genuine_refused = true;
            }
            let Some(t) = ray_triangle(ray, tri, floor) else {
                continue;
            };
            let hit = Hit {
                t,
                part: pi,
                item: cand.item,
                patch: flat.patch[cand.item],
            };
            if best.is_none_or(|b| (t, pi, cand.item) < (b.t, b.part, b.item)) {
                best = Some(hit);
            }
        }
    }
    (best, genuine_refused)
}

fn key(h: Option<Hit>) -> Option<(u64, usize, usize)> {
    h.map(|h| (h.t.to_bits(), h.part, h.item))
}

struct Tally {
    rays: usize,
    grazes: usize,
    moved: usize,
    moved_noise: usize,
    genuine_refused: usize,
    rescued: usize,
    lost: Vec<String>,
}

fn sweep(
    name: &str,
    step: &str,
    session: &DocSession,
    index: &PickIndex,
    tally: &mut Tally,
    show: bool,
) {
    let (_, eval) = session.landed_pair().expect("a landed pair");
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
                    tally.rays += 1;
                    let ray = Ray {
                        origin: *v - dir * reach,
                        dir,
                    };
                    let (un, _) = nearest(&parts, &ray, false, reach);
                    let (gu, refused) = nearest(&parts, &ray, true, reach);
                    if key(un) != key(gu) {
                        tally.moved += 1;
                        let un_det = un.map_or(0.0, |h| det(&ray, &parts[h.part].corners[h.item]));
                        if un_det.abs() < 1e-15 {
                            tally.moved_noise += 1;
                        }
                        if show && un_det.abs() >= 1e-15 {
                            println!(
                                "# {name} after {step}: {dir:?} through {v:?} reach {reach}: \
                                 unguarded {un:?} (det {un_det:e}) -> guarded {gu:?}"
                            );
                        }
                    }
                    let Some(u) = un else {
                        continue;
                    };
                    if (u.t - reach).abs() > 1e-9 {
                        continue;
                    }
                    tally.grazes += 1;
                    if refused {
                        tally.genuine_refused += 1;
                    }
                    match gu {
                        Some(g) if (g.t - reach).abs() < 1e-9 => {
                            if refused {
                                tally.rescued += 1;
                            }
                        }
                        other => tally.lost.push(format!(
                            "{name} after {step}: {dir:?} through {v:?} reach {reach}: unguarded \
                             {u:?} (det {:e}) -> guarded {other:?} (det {:e}); the service answers \
                             {:?}",
                            det(&ray, &parts[u.part].corners[u.item]),
                            other.map_or(f64::NAN, |g| det(&ray, &parts[g.part].corners[g.item])),
                            index.pick(eval, &ray)
                        )),
                    }
                }
            }
        }
    }
}

#[test]
fn a_genuine_graze_the_guard_refuses_is_answered_by_a_sibling_over_the_corpus() {
    let tol = Tol::witness();
    let mut tally = Tally {
        rays: 0,
        grazes: 0,
        moved: 0,
        moved_noise: 0,
        genuine_refused: 0,
        rescued: 0,
        lost: Vec::new(),
    };
    // The gallery ring at open and after its bump (the item's landing).
    {
        let text = common::gallery_ring_at(tol);
        let loaded = pncad::document::load(&text, tol).expect("the gallery ring loads");
        let doc = loaded.snapshot;
        let bump = ring_bump(&doc);
        let mut session = DocSession::inline(doc, tol);
        session.pump();
        sweep(
            "gallery_ring",
            "open",
            &session,
            &fresh_index(&session),
            &mut tally,
            true,
        );
        let outcome = session.perform(bump);
        assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
        session.pump();
        sweep(
            "gallery_ring",
            "the first edit",
            &session,
            &fresh_index(&session),
            &mut tally,
            true,
        );
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
        let show = matches!(c.name, "cut_cylinder" | "tube_arc" | "hollow_tube_elbow");
        sweep(c.name, "open", &session, &fresh_index(&session), &mut tally, show);
        let outcome = session.perform(bump);
        if outcome.refusal.is_some() {
            continue;
        }
        session.pump();
        sweep(
            c.name,
            "the first edit",
            &session,
            &fresh_index(&session),
            &mut tally,
            show,
        );
    }
    println!(
        "# {} rays; {} graze the aimed vertex unguarded; {} answers moved ({} from a noise \
         determinant); {} rays had a genuine graze refused by the guard, {} of them answered at \
         the same point by a sibling; {} LOST",
        tally.rays,
        tally.grazes,
        tally.moved,
        tally.moved_noise,
        tally.genuine_refused,
        tally.rescued,
        tally.lost.len()
    );
    assert!(tally.grazes > 1000, "the probe reached the graze class");
    assert!(
        tally.lost.is_empty(),
        "{} grazes lost to the guard with no sibling answering:\n{}",
        tally.lost.len(),
        tally.lost.join("\n")
    );
}
