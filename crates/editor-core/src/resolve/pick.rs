//! The hit-test service (G1: `ray → stable ref`, an editor-core
//! service on the mesh back-references and the [`super::hit`]
//! inversion).
//!
//! The chain: [`bvh::Bvh::ray`] over per-triangle boxes → exact
//! ray/triangle tests in plain `f64` → nearest hit by `t` with a
//! total, documented tie-break → the winning triangle's
//! [`mesh::FacePatch::face`] back-reference → [`super::hit::entity_name`]
//! → [`StableName`]. **No arena key crosses the layer-2/3 boundary as
//! a selection value**: the service's public answer is a name (plus
//! node id, `t`, and hit point) or a typed error; the
//! [`topo::FaceKey`]s live inside the private [`MeshPick`] state and
//! the private winning-triangle lookup. The one stated exception is
//! the [`HitTestError::Unnamed`] BUG arm, whose payload (inherited
//! verbatim from [`super::hit`]) carries the unnamed [`EntityRef`] —
//! that is a naming-emission diagnostic for a kernel bug report,
//! never a selection value, and typed beats stringly even there.
//!
//! Picking is a UI concern with no D9 predicate obligation (GQ6
//! re-survey §3): everything here is plain `f64` with conservative
//! comparisons, and what IS kept is determinism — fixed iteration
//! order, a total tie-break, no hashing — so the same pick against the
//! same state answers bit-identically.
//!
//! # Where the acceleration state lives, and when it dies
//!
//! [`MeshPick`] is per-mesh state the CONSUMER holds: built once per
//! tessellated mesh by [`MeshPick::build`], self-contained (it copies
//! the triangle geometry out of the mesh), and valid exactly as long
//! as the mesh it was built from is the one being displayed. A static
//! scene therefore never rebuilds per query. The obvious invalidator
//! is the evaluation epoch: a new [`Evaluation`] means new meshes,
//! so a consumer keys its `MeshPick` cache by
//! ([`Evaluation::epoch`], node, body) and drops entries whose epoch
//! is stale — exactly the staleness discipline the epoch exists for.
//! That keying is a CONSUMER obligation this module cannot check;
//! [`NodePick`] below is the door that discharges it by construction.
//!
//! # Provenance: the one confident-wrong-answer lane, and its door
//!
//! Arena keys collide numerically across sibling nodes, so a
//! [`PickTarget`] whose `(node, body)` is not the pair its mesh was
//! tessellated from makes [`pick_face`] invert the hit face's key
//! against the WRONG node's table — a plausible, confidently wrong
//! [`StableName`], not an error ([`PickTarget`]'s contract). The
//! typed door that closes the lane is [`NodePick`]: it fetches the
//! body from the evaluation payload itself (through the same
//! output-body indexing the name tables key by), tessellates and
//! indexes in one call, and hands back the mesh alongside — so the
//! pairing is established by construction and the display mesh and
//! the pick index are the same tessellation. Raw [`PickTarget`]
//! assembly remains for consumers that already hold a mesh, and
//! carries the loud contract.

use bvh::{Aabb, Bvh, Ray};
use geom_core::{Decide, Point3, Tol, Vec3};
use mesh::{Mesh, TessellateError};
use topo::FaceKey;

use super::hit::{HitTestError, entity_name};
use crate::eval::{Evaluation, NodeResult};
use crate::names::{EntityKey, EntityRef, StableName};
use crate::node::RecipeNodeId;
use crate::product::sources_of;

/// One triangle of the pick index: its corner geometry (copied out of
/// the mesh's position buffer, so the index cannot drift out of sync
/// with a mesh it merely borrowed) and its face back-reference.
#[derive(Clone, Copy, Debug)]
struct PickTri {
    /// First corner.
    a: Point3<f64>,
    /// Second corner.
    b: Point3<f64>,
    /// Third corner.
    c: Point3<f64>,
    /// The owning face ([`mesh::FacePatch::face`]) — private: this key
    /// never leaves the service.
    face: FaceKey,
}

/// Typed failure of [`MeshPick::build`] (closed; no silent lanes).
///
/// Deliberately arena-key-free: the offending site is named by patch
/// position and triangle position within the mesh value, which is the
/// vocabulary a layer-3 consumer holding that mesh can act on.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MeshPickError {
    /// A triangle indexes outside the mesh's position buffer — the
    /// mesh violates its own invariant (corrupt input, surfaced rather
    /// than skipped).
    PositionOutOfRange {
        /// Position of the offending patch in [`Mesh::patches`].
        patch: usize,
        /// Position of the offending triangle within that patch.
        triangle: usize,
        /// The out-of-range position index.
        index: u32,
    },
}

/// The per-mesh picking acceleration state: a triangle [`Bvh`] plus
/// the flat triangle table it indexes (module docs: consumer-held,
/// epoch-invalidated).
///
/// Triangles are stored patch-major in [`Mesh::patches`] order (face
/// arena order), triangles within a patch in their emitted order —
/// the **flat order** the pick tie-break below is stated in.
#[derive(Debug, Clone)]
pub struct MeshPick {
    /// Tree over the triangles' exact vertex-hull boxes.
    tree: Bvh,
    /// The flat triangle table (tree item `i` ↔ `tris[i]`).
    tris: Vec<PickTri>,
}

impl MeshPick {
    /// Builds the index from a tessellated mesh: one box per triangle
    /// (the exact hull of its three corners — a triangle is inside its
    /// corners' hull, so no padding is needed; the ray query's own
    /// conservative slab test supplies the rounding margin).
    ///
    /// A NaN position poisons its triangle's box, which the tree then
    /// never prunes (fail-safe); the exact ray test refuses NaN
    /// triangles, so poisoned geometry is un-hittable but never
    /// silently un-pruned.
    ///
    /// # Errors
    ///
    /// [`MeshPickError::PositionOutOfRange`] when a triangle indexes
    /// outside [`Mesh::positions`] — corrupt input, never skipped.
    pub fn build(mesh: &Mesh) -> Result<Self, MeshPickError> {
        let mut tris = Vec::new();
        let mut boxes = Vec::new();
        for (pi, patch) in mesh.patches.iter().enumerate() {
            for (ti, tri) in patch.triangles.iter().enumerate() {
                let mut corners = [Point3::new(0.0, 0.0, 0.0); 3];
                for (slot, &index) in corners.iter_mut().zip(tri) {
                    *slot = *mesh.positions.get(index as usize).ok_or(
                        MeshPickError::PositionOutOfRange {
                            patch: pi,
                            triangle: ti,
                            index,
                        },
                    )?;
                }
                let [a, b, c] = corners;
                // `from_points` is `None` only for an empty iterator;
                // three points always yield a box. The unreachable arm
                // degrades to poison — never pruned — rather than a
                // panic (fail-safe direction).
                boxes.push(Aabb::from_points([a, b, c]).unwrap_or_else(Aabb::poison));
                tris.push(PickTri {
                    a,
                    b,
                    c,
                    face: patch.face,
                });
            }
        }
        Ok(Self {
            tree: Bvh::build(&boxes),
            tris,
        })
    }
}

/// One displayed mesh offered to a pick: which node/body the mesh
/// renders, and its prebuilt index.
///
/// # The provenance contract (loud, unenforceable here)
///
/// **`(node, body)` MUST be the pair `pick`'s mesh was tessellated
/// from.** This module cannot verify it: arena keys collide
/// numerically across sibling nodes, so a mismatched pairing does not
/// error — [`pick_face`] resolves the hit triangle's face key against
/// the wrong node's table and answers a **plausible, confidently
/// wrong name** (the failure a selection consumer cannot detect;
/// same convention family as [`super::MeshPatchKey`]). Assemble raw
/// targets only from state that carries the pairing — e.g. a cache
/// keyed by ([`Evaluation::epoch`], node, body) holding the mesh and
/// its index together — or use [`NodePick`], which establishes the
/// pairing by construction and cannot be mis-assembled.
#[derive(Clone, Copy)]
pub struct PickTarget<'a> {
    /// The node whose evaluation produced the displayed body.
    pub node: RecipeNodeId,
    /// The output body index within that node's value.
    pub body: u32,
    /// The body's mesh index ([`MeshPick::build`]).
    pub pick: &'a MeshPick,
}

/// Typed failure of [`NodePick::build`] (closed; no silent lanes).
#[derive(Debug, Clone, PartialEq)]
pub enum NodePickError {
    /// The node has no `Ok` value in this evaluation — the same
    /// standing vocabulary [`pick_face`] answers.
    Standing(HitTestError),
    /// The node's value denotes no output body at all (datum,
    /// profile, declarations, mate).
    NotABody {
        /// The non-body node.
        node: RecipeNodeId,
    },
    /// The node's value has no output body at this index (out of
    /// range, an empty boolean, or an empty split side).
    NoSuchBody {
        /// The queried node.
        node: RecipeNodeId,
        /// The absent output-body index.
        body: u32,
    },
    /// Tessellation refused — the kernel's typed error, unaltered.
    Tessellate(TessellateError),
    /// The tessellated mesh failed indexing (corrupt back-references).
    Index(MeshPickError),
}

/// A pick index whose `(node, body)` ↔ mesh pairing is TRUE BY
/// CONSTRUCTION: [`NodePick::build`] fetches the body from the
/// evaluation payload itself — through the same output-body indexing
/// the name tables key by — then tessellates and indexes it in one
/// call. The fields are private and there is no other constructor,
/// so a `NodePick` cannot assert a pairing it does not have (the
/// closure of [`PickTarget`]'s provenance contract).
///
/// The tessellated mesh rides along ([`NodePick::mesh`]) so a viewer
/// can display exactly what it picks against — one tessellation, one
/// source of truth. Cache a `NodePick` per displayed (node, body) and
/// drop it when [`Evaluation::epoch`] moves.
#[derive(Debug, Clone)]
pub struct NodePick {
    node: RecipeNodeId,
    body: u32,
    mesh: Mesh,
    pick: MeshPick,
}

impl NodePick {
    /// Tessellates and indexes output body `body` of `node` at
    /// chordal tolerance `delta`, against `eval`'s own payload.
    ///
    /// # Errors
    ///
    /// [`NodePickError`], each arm typed: node standing, non-body or
    /// absent-body payloads, tessellation refusals (unaltered), and
    /// mesh-index refusals.
    pub fn build(
        eval: &Evaluation<f64>,
        node: RecipeNodeId,
        body: u32,
        delta: f64,
        tol: Tol,
    ) -> Result<Self, NodePickError> {
        let value = match eval.nodes.get(&node) {
            Some(NodeResult::Ok(v)) => v,
            Some(NodeResult::Failed(_)) => {
                return Err(NodePickError::Standing(HitTestError::NodeFailed { node }));
            }
            Some(NodeResult::Poisoned { through }) => {
                return Err(NodePickError::Standing(HitTestError::NodePoisoned {
                    node,
                    through: *through,
                }));
            }
            None => {
                return Err(NodePickError::Standing(HitTestError::NodeNotEvaluated {
                    node,
                }));
            }
        };
        // The payload's body-denoting sources, tagged with the SAME
        // output-body indices the node's name table keys its rows by
        // (`product::sources_of` — the one shipped enumeration; using
        // anything else here would re-mint the pairing this door
        // exists to guarantee).
        let Some(sources) = sources_of(value) else {
            return Err(NodePickError::NotABody { node });
        };
        let Some((_, body_arc, _)) = sources.into_iter().find(|(ix, _, _)| *ix == body) else {
            return Err(NodePickError::NoSuchBody { node, body });
        };
        let mesh = mesh::tessellate(&body_arc, delta, tol).map_err(NodePickError::Tessellate)?;
        let pick = MeshPick::build(&mesh).map_err(NodePickError::Index)?;
        Ok(Self {
            node,
            body,
            mesh,
            pick,
        })
    }

    /// The pick target this index answers for — pre-paired, ready for
    /// [`pick_face`].
    pub fn target(&self) -> PickTarget<'_> {
        PickTarget {
            node: self.node,
            body: self.body,
            pick: &self.pick,
        }
    }

    /// The tessellation this index was built from — the mesh to
    /// display so that what is drawn is what is picked.
    pub fn mesh(&self) -> &Mesh {
        &self.mesh
    }

    /// The node this index answers for.
    pub fn node(&self) -> RecipeNodeId {
        self.node
    }

    /// The output-body index this index answers for.
    pub fn body(&self) -> u32 {
        self.body
    }
}

/// A successful face pick: the stable name plus where and what was
/// hit. No arena key — the name IS the reference selection state
/// holds (G1).
#[derive(Debug, Clone)]
pub struct PickHit {
    /// The picked face's stable name.
    pub name: StableName,
    /// The node whose body was hit.
    pub node: RecipeNodeId,
    /// The output body index within that node's value.
    pub body: u32,
    /// The ray parameter of the hit (units of `|ray.dir|`).
    pub t: f64,
    /// The hit point, `origin + t · dir`.
    pub point: Point3<f64>,
}

/// The face pick: the nearest ray/triangle hit across `targets`,
/// resolved to a stable name.
///
/// `Ok(Some(hit))` is the nearest hit; **`Ok(None)` is the typed
/// miss** — the ray hits no offered triangle. Errors are never
/// flattened into a miss:
///
/// - every target's node must have an `Ok` value in `eval` — a target
///   whose node has no result / failed / was poisoned answers the
///   corresponding [`HitTestError`] up front (first offending target
///   in slice order), because a mesh being displayed for such a node
///   cannot belong to this evaluation;
/// - the winning face inverts through [`entity_name`]; an
///   evaluated-but-unnamed face is the loud
///   [`HitTestError::Unnamed`] bug report, propagated verbatim.
///
/// **Determinism and the tie-break (documented contract)**: the
/// winner minimizes `(t, target position in `targets`, flat triangle
/// position)` lexicographically — `t` compared as plain `f64` (no hit
/// has NaN `t`; the exact test refuses those), positions as integers.
/// A ray down the shared edge of two faces therefore resolves to the
/// earlier target, then the earlier patch in face-arena order, every
/// time. Triangle boundaries are CLOSED in the exact test, so an
/// edge/vertex graze is a hit for every incident triangle and the
/// tie-break, not chance, picks the answer.
///
/// The traversal early-outs on [`bvh::RayCandidate::t_enter`] (a
/// conservative lower bound on any hit in that box): candidates are
/// visited in ascending `t_enter`, and once the confirmed best `t` is
/// strictly below a candidate's `t_enter` the rest of that target's
/// list cannot improve on it. A poisoned/NaN ray is legal input: the
/// tree returns everything, every exact test misses, and the answer
/// is the typed miss.
///
/// # Errors
///
/// [`HitTestError`] as above — target standing first, then the
/// winning face's inversion.
pub fn pick_face<T: Decide>(
    eval: &Evaluation<T>,
    targets: &[PickTarget<'_>],
    ray: &Ray,
) -> Result<Option<PickHit>, HitTestError> {
    // Target standing, up front (docs: an error, never a silent miss).
    for target in targets {
        match eval.nodes.get(&target.node) {
            Some(NodeResult::Ok(_)) => {}
            Some(NodeResult::Failed(_)) => {
                return Err(HitTestError::NodeFailed { node: target.node });
            }
            Some(NodeResult::Poisoned { through }) => {
                return Err(HitTestError::NodePoisoned {
                    node: target.node,
                    through: *through,
                });
            }
            None => {
                return Err(HitTestError::NodeNotEvaluated { node: target.node });
            }
        }
    }

    // The nearest hit, minimizing (t, target position, flat triangle
    // position) lexicographically — the documented tie-break. The
    // winner's identity rides along, so nothing is re-looked-up after
    // the scan.
    struct Best {
        t: f64,
        target_pos: usize,
        tri_pos: usize,
        node: RecipeNodeId,
        body: u32,
        face: FaceKey,
    }
    let mut best: Option<Best> = None;
    for (target_pos, target) in targets.iter().enumerate() {
        for cand in target.pick.tree.ray(ray) {
            if let Some(b) = &best
                && b.t < cand.t_enter
            {
                // Candidates ascend in t_enter, a lower bound on any
                // hit in their box: nothing further can improve.
                break;
            }
            let Some(tri) = target.pick.tris.get(cand.item) else {
                // Unreachable: the tree was built over exactly `tris`.
                continue;
            };
            if let Some(t) = ray_triangle(ray, tri) {
                let better = match &best {
                    None => true,
                    // Plain f64 compare is total here: `t` is never
                    // NaN (the exact test refuses non-finite hits).
                    Some(b) => {
                        t < b.t || (t == b.t && (target_pos, cand.item) < (b.target_pos, b.tri_pos))
                    }
                };
                if better {
                    best = Some(Best {
                        t,
                        target_pos,
                        tri_pos: cand.item,
                        node: target.node,
                        body: target.body,
                        face: tri.face,
                    });
                }
            }
        }
    }

    let Some(win) = best else {
        return Ok(None); // the typed miss
    };
    let name = entity_name(
        eval,
        win.node,
        EntityRef {
            body: win.body,
            key: EntityKey::Face(win.face),
        },
    )?;
    Ok(Some(PickHit {
        name: name.clone(),
        node: win.node,
        body: win.body,
        t: win.t,
        point: ray.origin + ray.dir * win.t,
    }))
}

/// The exact ray/triangle test (Möller–Trumbore, both-sided, plain
/// `f64`): `Some(t)` iff the ray meets the CLOSED triangle at `t ≥ 0`.
///
/// Boundary semantics, stated: `u ∈ [0, 1]`, `v ≥ 0`, `u + v ≤ 1`,
/// `t ≥ 0` — all closed, so a hit exactly on a shared edge or vertex
/// is a hit for EVERY incident triangle (the caller's tie-break
/// disambiguates; watertight meshes therefore never lose a graze to
/// an open boundary). A zero determinant (ray parallel to the plane,
/// or a degenerate triangle) is a miss, as is any NaN anywhere: every
/// acceptance condition is an affirmative comparison, which NaN
/// fails — poisoned geometry is un-hittable, never mis-hit. A
/// NON-FINITE `t` (the `e2·q × inv` product overflowing on a
/// near-degenerate determinant) is refused by the final guard: a hit
/// the service cannot place at a finite point is a miss, never a
/// `PickHit` whose `point` would be `0 · ∞ = NaN`.
fn ray_triangle(ray: &Ray, tri: &PickTri) -> Option<f64> {
    let e1: Vec3<f64> = tri.b - tri.a;
    let e2: Vec3<f64> = tri.c - tri.a;
    let p = ray.dir.cross(e2);
    let det = e1.dot(p);
    if det == 0.0 {
        // Parallel or degenerate. (A NaN det passes THIS check and is
        // refused two comparisons down, at `u_inside`.)
        return None;
    }
    let inv = 1.0 / det;
    let s = ray.origin - tri.a;
    let u = s.dot(p) * inv;
    let u_inside = (0.0..=1.0).contains(&u);
    if !u_inside {
        return None;
    }
    let q = s.cross(e1);
    let v = ray.dir.dot(q) * inv;
    let v_inside = v >= 0.0 && u + v <= 1.0;
    if !v_inside {
        return None;
    }
    let t = e2.dot(q) * inv;
    let forward_and_finite = t >= 0.0 && t.is_finite();
    forward_and_finite.then_some(t)
}
