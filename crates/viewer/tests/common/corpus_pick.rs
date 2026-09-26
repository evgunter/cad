//! **The corpus pick walk**: the landings the corpus pick suites sweep,
//! the single-level reference they read the door against, and the two
//! aims they fire.
//!
//! `index_memo`, `pick3_acceptance`, `review_pick_r2` and
//! `review_pick2_r1` all open every parametric corpus document and the
//! tour's gallery ring, index each landing at [`super::corpus_delta`],
//! and walk the same rays over it. This is that walk, spelled once.
//!
//! # Which door here carries an oracle
//!
//! Asked door by door, as `tests/common`'s header asks it:
//!
//! - [`FlatReference`] **does.** It is the second implementation
//!   `index_memo` and `pick3_acceptance` compare `PickIndex::pick`
//!   against, and it shares nothing with the door but the door's two
//!   leaf callables (`ray_triangle` for the exact test, `answer_of` for
//!   the certified tie's face grouping): it reads the index's MESHES and
//!   none of its trees, tables or early-out. So a suite reading it still
//!   compares the door with something the door does not read, and a
//!   defect in it — a dropped triangle, a wrong owner, an early-out of
//!   its own — makes the door and the reference disagree on the rows
//!   that read both, rather than hiding in them.
//! - The aims ([`tie_rays_for`], [`wide_aim`]) and the landings
//!   ([`over_every_landing`], [`ring_bump`]) **do not**: they choose which
//!   rays are fired at which documents, and decide nothing about an
//!   answer.

use bvh::{Aabb, Bvh, Ray};
use editor_core::resolve::{TSpan, answer_of, crossing, ray_triangle};
use editor_core::{DocEdit, Evaluation, Expr, ProfileDoc, RecipeNodeId, SlotId, unparse};
use pncad::geom_core::{Point3, Tol, Vec3};
use viewer::pickindex::PickIndex;
use viewer::session::{DocSession, SessionOp};

use crate::corpus;
use crate::fixture::pick::{AXES, aimed};

// ---------------------------------------------------------------
// The single-level reference.
// ---------------------------------------------------------------

/// **The single-level reference.** One flat tree per part over EVERY
/// triangle of its mesh, patch-major in the mesh's patch order, and the
/// exact test on every candidate the tree hands back — no early-out.
///
/// Whatever shape the production index takes — one tree per mesh, a
/// tree per patch under a tree over the patches — its answer to every
/// ray is this one's, hit for hit: the minimum over the per-triangle
/// tests, each of which reads the ray and the triangle alone. The
/// door's early-out on the conservative box entry is a cost measure,
/// and this is what says, ray by ray, that it did not change the
/// answer.
pub struct FlatReference {
    /// One flattened part per part of the index, in the index's order.
    pub parts: Vec<FlatPart>,
}

/// One part of the index, flattened.
pub struct FlatPart {
    /// The tree over every triangle's box, in flat order.
    pub tree: Bvh,
    /// Each flat triangle's corners.
    pub corners: Vec<[Point3<f64>; 3]>,
    /// Flat triangle position → (patch position, triangle position):
    /// the door groups its survivors by FACE, so the reference has to
    /// know the grouping too.
    pub owner: Vec<(usize, usize)>,
}

/// One admitted candidate, or one face of an answer: which part, which
/// flat triangle, which patch, over what `t` interval.
#[derive(Clone, Copy, Debug)]
pub struct FlatHit {
    pub part: usize,
    /// Flat triangle position within the part — for a face of an
    /// answer, its min-`t` member's, which is the triangle the point
    /// comes from.
    pub item: usize,
    pub patch: usize,
    pub span: TSpan,
}

impl FlatHit {
    /// The rounded parameter — what the door reports.
    pub fn t(&self) -> f64 {
        self.span.t
    }
}

impl FlatReference {
    /// `index`'s meshes, flattened part by part.
    pub fn of(index: &PickIndex) -> Self {
        let parts = index
            .parts()
            .iter()
            .map(|part| {
                let mesh = part.mesh();
                let mut corners = Vec::new();
                let mut owner = Vec::new();
                let mut boxes = Vec::new();
                for (pi, patch) in mesh.patches.iter().enumerate() {
                    for (ti, tri) in patch.triangles.iter().enumerate() {
                        let c = tri.map(|i| mesh.positions[i as usize]);
                        boxes.push(Aabb::from_points(c).expect("three points box"));
                        corners.push(c);
                        owner.push((pi, ti));
                    }
                }
                FlatPart {
                    tree: Bvh::build(&boxes),
                    corners,
                    owner,
                }
            })
            .collect();
        Self { parts }
    }

    /// **Every admitted candidate on `ray`**: EVERY candidate box the
    /// ray meets is tested, part by part and, within a part, in flat
    /// triangle order — the order `pick_face` lists a refusal's faces
    /// in, so the certified tie over this list answers the way the door
    /// does.
    pub fn every(&self, ray: &Ray) -> Vec<FlatHit> {
        let mut hits = Vec::new();
        for (part, flat) in self.parts.iter().enumerate() {
            let mut items: Vec<usize> = flat.tree.ray(ray).into_iter().map(|c| c.item).collect();
            items.sort_unstable();
            for item in items {
                let Some(span) = ray_triangle(ray, &flat.corners[item]) else {
                    continue;
                };
                let (patch, _) = flat.owner[item];
                hits.push(FlatHit {
                    part,
                    item,
                    patch,
                    span,
                });
            }
        }
        hits
    }

    /// **What the door answers for**: one hit per FACE of the certified
    /// tie over [`Self::every`], in the order the door lists them, and
    /// how many candidates the tie holds — which is what says a ray
    /// actually tied. Empty is the miss; one face is the hit; several
    /// are the refusal.
    ///
    /// The FACES, not the triangles: several triangles of one face are
    /// one answer, at the HULL of their intervals and the smallest
    /// rounded `t` among them. Neither half of the RULE is restated —
    /// the candidates are offered to [`answer_of`] in `(part, flat
    /// position)` order, and that callable is the door's own, order and
    /// face grouping together. What is this reference's own is the
    /// enumeration.
    pub fn pick(&self, ray: &Ray) -> (Vec<FlatHit>, usize) {
        let hits = self.every(ray);
        let candidates: Vec<(TSpan, (usize, usize))> = hits
            .iter()
            .map(|hit| (hit.span, (hit.part, hit.patch)))
            .collect();
        let faces = answer_of(&candidates).faces();
        let tied = faces.iter().map(|face| face.members).sum();
        let per_face = faces
            .into_iter()
            .map(|face| FlatHit {
                span: face.span,
                ..hits[face.member]
            })
            .collect();
        (per_face, tied)
    }
}

/// **An answered candidate's barycentric error bounds, when they are
/// too wide for it to BE an answer** — or `None` when they are not.
///
/// A value inside `[0, 1]` whose rounding interval is `1` or wider
/// covers the admissible range, and the exact test (`ray_triangle`'s
/// INFORM half) refuses it; so over every candidate the door or the
/// reference answered with, this is `None`, and a row counts or
/// asserts the ones that are not. A NaN bound is not a bound, and is
/// answered as too wide rather than slipping through a `>= 1.0` that a
/// NaN fails.
///
/// # Panics
///
/// If the candidate's determinant is not certified: an answered
/// candidate's always is.
pub fn too_wide(ray: &Ray, tri: &[Point3<f64>; 3]) -> Option<[f64; 3]> {
    let bounds = crossing(ray, tri)
        .expect("an answered candidate's determinant is certified")
        .barycentrics
        .map(|(_, err)| err);
    bounds
        .iter()
        .any(|&b| b.is_nan() || b >= 1.0)
        .then_some(bounds)
}

// ---------------------------------------------------------------
// The two aims.
// ---------------------------------------------------------------

/// The largest absolute coordinate of any position of any part — the
/// scale both aims set their standoff by.
pub fn extent(index: &PickIndex) -> f64 {
    let mut ext = 0.0f64;
    for part in index.parts() {
        for p in &part.mesh().positions {
            ext = ext.max(p.x.abs()).max(p.y.abs()).max(p.z.abs());
        }
    }
    ext
}

/// **The tie-break aim**: rays aimed exactly at the points two or more
/// patches share — a boundary polyline's first point (a vertex or chord
/// point every incident face's triangles have as a corner) and the
/// midpoint of its first segment (a point on the shared edge) — along
/// the six [`AXES`] from outside the picture, each with the `reach` it
/// was fired from. A hit there is a hit for every incident triangle at
/// one `t`, across patches and, where bodies touch, across parts: the
/// case the door's set rule is about, and the one it refuses on.
///
/// At most ~40 boundaries per part, spread over the polyline list.
pub fn tie_rays_for(index: &PickIndex) -> Vec<(Ray, f64)> {
    let mut targets = Vec::new();
    for part in index.parts() {
        let mesh = part.mesh();
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
    let reach = 4.0 * extent(index).max(1e-3);
    let mut rays = Vec::new();
    for at in targets {
        for dir in AXES {
            rays.push((aimed(at, dir, reach), reach));
        }
    }
    rays
}

/// One ray of the wide aim, with where it was aimed.
#[derive(Clone, Copy, Debug)]
pub struct Aim {
    /// The part whose vertex it is aimed at.
    pub part: usize,
    /// The vertex's position in that part's mesh.
    pub vertex: usize,
    /// The vertex itself.
    pub at: Point3<f64>,
    pub dir: Vec3<f64>,
    /// The parameter the vertex is at along [`Self::ray`].
    pub reach: f64,
}

impl Aim {
    /// The ray: [`aimed`] at the vertex from `reach` back.
    pub fn ray(&self) -> Ray {
        aimed(self.at, self.dir, self.reach)
    }
}

/// **The wide aim**: every mesh vertex (subsampled to at most ~1500 a
/// part), along the six [`AXES`], from three reaches — two fixed, one
/// clear of the whole picture — in part, vertex, direction, reach
/// order.
pub fn wide_aim(index: &PickIndex) -> impl Iterator<Item = Aim> + '_ {
    let reaches = [1.48, 3.0, 4.0 * extent(index).max(1e-3)];
    index.parts().iter().enumerate().flat_map(move |(part, p)| {
        let positions = &p.mesh().positions;
        let stride = positions.len().div_ceil(1500).max(1);
        positions
            .iter()
            .enumerate()
            .step_by(stride)
            .flat_map(move |(vertex, &at)| {
                AXES.into_iter().flat_map(move |dir| {
                    reaches.into_iter().map(move |reach| Aim {
                        part,
                        vertex,
                        at,
                        dir,
                        reach,
                    })
                })
            })
    })
}

// ---------------------------------------------------------------
// The landings.
// ---------------------------------------------------------------

/// **The gallery ring's own bump**: the LAST extrude distance or
/// revolve angle in `doc`, scaled — a distance by `1.03125`, an angle
/// by `0.96875`, both exact in binary.
///
/// # Panics
///
/// If `doc` has neither.
pub fn ring_bump(doc: &ProfileDoc) -> (RecipeNodeId, SlotId, Expr) {
    let env = doc.param_env::<f64>();
    for &node in doc.order().iter().rev() {
        match doc.node(node).expect("a node") {
            editor_core::Node::Extrude { distance, .. } => {
                let value = editor_core::eval(distance, &env).expect("a literal distance");
                return (node, SlotId::Distance, super::len(value * 1.03125));
            }
            editor_core::Node::Revolve { angle, .. } => {
                let value = editor_core::eval(angle, &env).expect("a literal angle");
                return (node, SlotId::RevolveAngle, super::ang(value * 0.96875));
            }
            _ => {}
        }
    }
    panic!("no extrude or revolve in the document")
}

/// A slot write as the session's op spells it.
fn set_slot(node: RecipeNodeId, slot: SlotId, expr: &Expr) -> SessionOp {
    SessionOp::SetSlotExpression {
        node,
        slot,
        text: unparse(expr),
    }
}

/// **Every landing the corpus pick suites sweep**: the gallery ring at
/// open and after its [`ring_bump`], then each parametric corpus
/// document the same way through its own bump edit — `sweep(name,
/// step, index, evaluation)` once per landing, `step` being `"open"` or
/// `"the first edit"`.
///
/// A corpus document with no bump edit, one that does not land an
/// evaluation at open, or one whose bump is refused contributes what it
/// reached and no more; the gallery ring's bump is asserted to land.
pub fn over_every_landing(mut sweep: impl FnMut(&str, &str, &PickIndex, &Evaluation<f64>)) {
    let tol = Tol::witness();
    let mut landed = |name: &str, step: &str, session: &DocSession| {
        let index = super::corpus_index(session);
        let (_, eval) = session.landed_pair().expect("a landed pair");
        sweep(name, step, &index, eval);
    };
    {
        let text = super::gallery_ring_at(tol);
        let loaded = pncad::document::load(&text, tol).expect("the gallery ring loads");
        let doc = loaded.snapshot;
        let (node, slot, expr) = ring_bump(&doc);
        let mut session = DocSession::inline(doc, tol);
        session.pump();
        landed("gallery_ring", "open", &session);
        let outcome = session.perform(set_slot(node, slot, &expr));
        assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
        session.pump();
        landed("gallery_ring", "the first edit", &session);
    }
    for doc in corpus::documents() {
        let DocEdit::SetParam { node, slot, expr } = doc.bump.clone() else {
            continue;
        };
        let mut session = DocSession::inline(doc.doc.clone(), tol);
        session.pump();
        if session.evaluation().is_none() {
            continue;
        }
        landed(doc.name, "open", &session);
        let outcome = session.perform(set_slot(node, slot, &expr));
        if outcome.refusal.is_some() {
            continue;
        }
        session.pump();
        landed(doc.name, "the first edit", &session);
    }
}
