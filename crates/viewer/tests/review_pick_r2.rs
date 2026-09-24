//! The certified determinant measured over the corpus with a WIDER
//! aim than the tie-break row: every mesh vertex (subsampled), six
//! axis directions, three reaches, on the gallery ring and every
//! parametric corpus document at open and after the first edit. The
//! row came in as a review probe (branch `review/pick-r2`) against a
//! box-entry guard the unit withdrew; it now asserts the behaviour of
//! the certified determinant and pins what that mechanism refuses on
//! this corpus.
//!
//! Three claims, one row:
//!
//! 1. **No genuine determinant is refused.** A candidate whose
//!    conditioning `|det| / (|e1|·|e2|·|d|)` is at or above `1e-12`
//!    — four orders above the certification's own bound — is never
//!    refused at the determinant. Red = the mechanism refuses a
//!    crossing it could certify.
//! 2. **The refusals are pinned.** The count of candidates refused at
//!    the determinant (a non-zero determinant the certification
//!    cannot vouch for) and the count of rays carrying one are pinned
//!    to the corpus as it is: a change in either is a change in the
//!    class the mechanism refuses — or a retessellation — and is
//!    re-derived by `cargo test -p viewer --test all --
//!    review_pick_r2 --nocapture`, which prints the tally.
//! 3. **The aim reaches the graze class**: the count of rays answered
//!    at the aimed vertex is pinned the same way.
//! 4. **No winner's barycentric bound reaches `1`.** A value inside
//!    `[0, 1]` whose rounding interval is that wide covers the range,
//!    and `ray_triangle` refuses it, so zero is the only count this
//!    can have. Asserted rather than pinned — the count is derivable
//!    from the acceptance and a pinned `0` would read as a baseline —
//!    and asserted HERE as well as in `index_memo` because this is
//!    the aim that walks 441 126 rays.
//!
//! What is NOT pinned here, and why: the answers that moved against
//! `main`'s kernel before this unit. That predicate is the copy the
//! unit removed from the tree, and an independent oracle of another
//! algebra differs from it at rounding level in exactly the class
//! being counted, so the moved-answer table is a one-shot measurement
//! (the PR's), not a row.

#![allow(clippy::expect_used, clippy::panic, clippy::unwrap_used)]

use bvh::{Aabb, Bvh, Ray};
use editor_core::resolve::{crossing, ray_triangle};
use editor_core::{DocEdit, Expr, ProfileDoc, RecipeNodeId, SlotId, unparse};
use pncad::geom_core::{Point3, Tol, Vec3};
use viewer::pickindex::PickIndex;
use viewer::session::{DocSession, SessionOp};

use crate::common;
use crate::common::corpus_index;
use crate::corpus;

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
                (SlotId::Distance, common::len(value * 1.03125))
            }
            editor_core::Node::Revolve { angle, .. } => {
                let value = editor_core::eval(angle, &env).expect("a literal angle");
                (SlotId::RevolveAngle, common::ang(value * 0.96875))
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

/// Möller–Trumbore's determinant, uncertified, and its conditioning.
fn det_and_conditioning(ray: &Ray, tri: &[Point3<f64>; 3]) -> (f64, f64) {
    let e1: Vec3<f64> = tri[1] - tri[0];
    let e2: Vec3<f64> = tri[2] - tri[0];
    let det = e1.dot(ray.dir.cross(e2));
    (det, det.abs() / (e1.norm() * e2.norm() * ray.dir.norm()))
}

#[derive(Default)]
struct Tally {
    rays: usize,
    grazes: usize,
    refused_candidates: usize,
    rays_with_a_refusal: usize,
    genuine_refused: Vec<String>,
    /// Winners whose barycentric bounds are not all below `1`. A
    /// value inside `[0, 1]` with a bound that wide covers the range
    /// and the exact test refuses it, so this is empty by
    /// construction — over the WIDE aim as well as `index_memo`'s,
    /// which is the point of asserting it in both places.
    wide_winners: Vec<String>,
    /// The best-conditioned candidate refused at the determinant —
    /// the class the mechanism refuses, at its edge (reported, not
    /// pinned).
    worst_refused_conditioning: f64,
}

/// The pinned tally over the aim below (docs: re-derive with
/// `--nocapture`).
const PINNED: (usize, usize, usize, usize) = (442_782, 141_983, 12_786, 6_882);

fn sweep(name: &str, step: &str, index: &PickIndex, tally: &mut Tally) {
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
                    let mut best: Option<(f64, [f64; 3])> = None;
                    let mut refused_here = 0usize;
                    for flat in &parts {
                        for cand in flat.tree.ray(&ray) {
                            let tri = &flat.corners[cand.item];
                            let cross = crossing(&ray, tri);
                            if cross.is_none() {
                                let (det, cond) = det_and_conditioning(&ray, tri);
                                if det != 0.0 {
                                    refused_here += 1;
                                    tally.worst_refused_conditioning =
                                        tally.worst_refused_conditioning.max(cond);
                                }
                                if cond >= 1e-12 {
                                    tally.genuine_refused.push(format!(
                                        "{name} after {step}: {dir:?} through {v:?}: det {det:e} \
                                         (conditioning {cond:e}) refused on {tri:?}"
                                    ));
                                }
                            }
                            if let Some(span) = ray_triangle(&ray, tri)
                                && best.is_none_or(|(b, _)| span.t < b)
                            {
                                let t = span.t;
                                let bounds = cross
                                    .expect("an admitted candidate has a certified determinant")
                                    .barycentrics
                                    .map(|(_, err)| err);
                                best = Some((t, bounds));
                            }
                        }
                    }
                    tally.refused_candidates += refused_here;
                    if refused_here > 0 {
                        tally.rays_with_a_refusal += 1;
                    }
                    if let Some((t, bounds)) = best {
                        if (t - reach).abs() < 1e-9 {
                            tally.grazes += 1;
                        }
                        // A NaN bound is not a bound and must red
                        // this row, not slip through a comparison it
                        // fails.
                        if bounds.iter().any(|&b| b.is_nan() || b >= 1.0) {
                            tally.wide_winners.push(format!(
                                "{name} after {step}: {dir:?} through {v:?} at reach {reach}: \
                                 winner at t {t} with bounds {bounds:?}"
                            ));
                        }
                    }
                }
            }
        }
    }
}

#[test]
fn the_certified_determinant_refuses_no_genuine_crossing_over_the_corpus() {
    let tol = Tol::witness();
    let mut tally = Tally::default();
    {
        let text = common::gallery_ring_at(tol);
        let loaded = pncad::document::load(&text, tol).expect("the gallery ring loads");
        let doc = loaded.snapshot;
        let bump = ring_bump(&doc);
        let mut session = DocSession::inline(doc, tol);
        session.pump();
        sweep("gallery_ring", "open", &corpus_index(&session), &mut tally);
        let outcome = session.perform(bump);
        assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
        session.pump();
        sweep(
            "gallery_ring",
            "the first edit",
            &corpus_index(&session),
            &mut tally,
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
        sweep(c.name, "open", &corpus_index(&session), &mut tally);
        let outcome = session.perform(bump);
        if outcome.refusal.is_some() {
            continue;
        }
        session.pump();
        sweep(
            c.name,
            "the first edit",
            &corpus_index(&session),
            &mut tally,
        );
    }
    let counts = (
        tally.rays,
        tally.grazes,
        tally.refused_candidates,
        tally.rays_with_a_refusal,
    );
    println!(
        "# review_pick_r2 tally (rays, answered at the aimed vertex, candidates refused at the \
         determinant, rays with a refusal): {counts:?}; best conditioning refused {:e}",
        tally.worst_refused_conditioning
    );
    assert!(
        tally.wide_winners.is_empty(),
        "{} winners over the wide aim carry a barycentric bound of 1 or more, which covers the \
         admissible range and the exact test refuses:\n{}",
        tally.wide_winners.len(),
        tally.wide_winners.join("\n")
    );
    assert!(
        tally.genuine_refused.is_empty(),
        "{} candidates with a genuine determinant refused at the certification:\n{}",
        tally.genuine_refused.len(),
        tally.genuine_refused.join("\n")
    );
    assert_eq!(
        counts, PINNED,
        "the tally over the corpus moved from its pin; if the corpus or the certification \
         changed on purpose, re-derive with --nocapture and re-pin"
    );
}
