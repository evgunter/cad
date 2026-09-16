//! The hit-test service (G1: `ray → stable ref`, an editor-core
//! service on the mesh back-references and the [`super::hit`]
//! inversion).
//!
//! The chain: [`bvh::Bvh::ray`] over the patches' boxes, then over
//! the candidate patches' per-triangle boxes, merged into one
//! candidate sequence ([`MeshPick`]) → exact ray/triangle tests in
//! plain `f64`, each refused below its own box's certified entry
//! ([`ray_triangle`]) → nearest hit by `t` with a total, documented
//! tie-break
//! → the winning patch's [`mesh::FacePatch::face`] back-reference →
//! [`super::hit::entity_name`] → [`StableName`]. **No arena key crosses the layer-2/3 boundary as
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
//! tessellated mesh — by [`MeshPick::build`], or by the memoised door
//! [`MeshPick::build_with`], which serves each patch's table whole
//! from [`PickMemo`] where the tessellation reused that patch and
//! builds it where it did not — self-contained (it copies the
//! triangle geometry out of the mesh), and valid exactly as long
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

use std::collections::HashMap;
use std::sync::Arc;

use bvh::{Aabb, Bvh, Ray};
use geom_core::{Decide, Point3, Tol, Vec3};
use mesh::{Mesh, PatchKeys, PatchMemo, StoredPatchId, TessellateError, tessellate_with};
use topo::FaceKey;

use super::hit::{HitTestError, entity_name};
use crate::eval::{ContentKey, Evaluation, NamingKey, NodeResult, NodeValue};
use crate::ident::DocumentId;
use crate::names::{EntityKey, EntityRef, StableName};
use crate::node::RecipeNodeId;
use crate::product::sources_of;

/// One triangle of the pick index: its corner geometry, copied out of
/// the mesh's position buffer so the index cannot drift out of sync
/// with a mesh it merely borrowed. Its face is its patch's
/// ([`PickPatch::face`]).
#[derive(Clone, Copy, Debug)]
struct PickTri {
    /// First corner.
    a: Point3<f64>,
    /// Second corner.
    b: Point3<f64>,
    /// Third corner.
    c: Point3<f64>,
}

/// One patch's pick table: its triangles' corners copied out of the
/// mesh's position buffer, the tree over their boxes, and the hull of
/// those boxes.
///
/// **Everything here is a function of the patch's placed corners and
/// nothing else** — the boxes are each triangle's exact corner hull,
/// the tree is `bvh`'s arena-order build over them, the hull is the
/// fold over them left to right. That is what lets [`PickMemo`] serve
/// the whole table across pictures under
/// [`mesh::StoredPatchId`], which says the placed
/// corners are bit-identical: same corners, same table, with nothing
/// per triangle to recompute or compare. The `Arc` is the memo's
/// handle and this index's, one build.
#[derive(Debug)]
struct PickTable {
    /// The patch's triangles, in emitted order.
    tris: Vec<PickTri>,
    /// Tree over the triangles' exact vertex-hull boxes (tree item `i`
    /// ↔ `tris[i]`).
    tree: Bvh,
    /// The hull of those boxes, folded left to right (fixed
    /// association order) — the patch's box in the top-level tree. A
    /// patch with no triangles is poison: never pruned, and its empty
    /// tree answers nothing.
    hull: Aabb,
}

/// One patch of the pick index: its table, and the two things the
/// table cannot carry because neither is a function of the patch's own
/// geometry — the face it belongs to (an arena key, lineage-scoped)
/// and where its triangles start in the mesh's patch-major order.
#[derive(Clone, Debug)]
struct PickPatch {
    /// The corners, the tree and the hull ([`PickTable`]).
    table: Arc<PickTable>,
    /// The owning face ([`mesh::FacePatch::face`]) — private: this key
    /// never leaves the service.
    face: FaceKey,
    /// The flat position of `table.tris[0]` in the mesh's patch-major
    /// triangle order: the tie-break's coordinate.
    base: usize,
}

impl PickTable {
    /// The table of the patch at position `pi`: one box per triangle
    /// (the exact hull of its three corners — a triangle is inside its
    /// corners' hull, so no padding is needed; the ray query's own
    /// conservative slab test supplies the rounding margin), the tree
    /// over those boxes, and their hull.
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
    fn build(mesh: &Mesh, pi: usize, patch: &mesh::FacePatch) -> Result<Self, MeshPickError> {
        let mut tris = Vec::with_capacity(patch.triangles.len());
        let mut boxes = Vec::with_capacity(patch.triangles.len());
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
            tris.push(PickTri { a, b, c });
        }
        let hull = boxes
            .iter()
            .fold(None::<Aabb>, |acc, b| Some(acc.map_or(*b, |a| a.hull(b))))
            .unwrap_or_else(Aabb::poison);
        Ok(Self {
            tree: Bvh::build(&boxes),
            tris,
            hull,
        })
    }
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

// The human-readable rendering: the one arm states the PROBLEM in the
// type's own arena-key-free vocabulary — patch position, triangle
// position, the out-of-range index — and calls the violation what it
// is: a mesh that breaks its own invariant is corrupt input to report,
// never geometry to guess at.
impl core::fmt::Display for MeshPickError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::PositionOutOfRange {
                patch,
                triangle,
                index,
            } => write!(
                f,
                "pick index: triangle {triangle} of patch {patch} references position {index}, \
                 outside the mesh's position buffer — the mesh violates its own invariant, so \
                 this is corrupt input, surfaced rather than skipped"
            ),
        }
    }
}

impl core::error::Error for MeshPickError {}

/// The per-mesh picking acceleration state (module docs: consumer-
/// held, epoch-invalidated): **two levels of tree**. One [`Bvh`] per
/// patch over that patch's triangles, and one top-level [`Bvh`] over
/// the patches' boxes, in [`Mesh::patches`] order (face arena order).
///
/// Triangles are addressed patch-major in patch order, triangles
/// within a patch in their emitted order — the **flat order** the
/// pick tie-break below is stated in ([`PickPatch::base`] plus the
/// position within the patch).
///
/// # Why the two-level query answers exactly as one tree over every
/// triangle would
///
/// A pick is a function of the candidate SET and of each candidate's
/// own box: the exact test refuses a `t` below the box's certified
/// entry ([`ray_triangle`]), so the loop's early-out on that entry
/// prunes only candidates that could not win ([`pick_face`]), and
/// the answer is the same in every candidate order. What the order
/// still decides is the WORK — which candidates are tested before
/// the early-out fires — so [`MeshPick::candidates`] reproduces the
/// single-tree sequence verbatim, and the two-level index costs what
/// one tree would:
///
/// - **Same set.** `bvh`'s contract: a query's candidates are a
///   function of the item boxes alone, independent of tree shape, and
///   an item is a candidate iff its own box passes [`Ray::slab_enter`].
///   That test is monotone in the box — every per-axis endpoint is a
///   correctly rounded subtraction then division, so a box's slab
///   interval contains that of any box it contains — hence a
///   triangle whose box passes has a patch box that passes: the top
///   level never prunes a patch holding a single-tree candidate, and
///   within a candidate patch the per-triangle test is the same test
///   on the same box. (The one seam is `slab_t`'s exact recompute of
///   an OVERFLOWED subtraction, which can disagree with the direct
///   formula by ~2 ULP; it is reached only when a bound and the ray
///   origin differ by more than `f64::MAX`, which no mesh does.)
/// - **Same values.** Each candidate's `t_enter` is the same function
///   of the same triangle box — and it is the bound the exact test
///   refuses below, so each candidate's test answers the same.
/// - **Same order.** The merged list is sorted by
///   `(t_enter under total_cmp, flat position)`, the single tree's
///   documented order.
///
/// So the loop in [`pick_face`] sees the sequence one tree over all
/// the triangles would give it, and answers bit-identically — the
/// invariant `viewer`'s `index_memo` differential pins against a
/// single-level reference that tests every candidate in every order,
/// tie-break row included.
///
/// # The two doors, and why they answer the same index
///
/// [`MeshPick::build`] builds every patch's [`PickTable`];
/// [`MeshPick::build_with`] serves the tables [`PickMemo`] holds for
/// the patches this tessellation reused and builds the rest. The
/// second answers table for table and tree for tree what the first
/// would — a table is a function of its patch's placed corners alone,
/// and the memo's key says those corners are bit-identical — which is
/// the row the same differential asserts after every landing.
#[derive(Debug, Clone)]
pub struct MeshPick {
    /// Tree over the patches' boxes (item `i` ↔ `patches[i]`); a patch
    /// with no triangles carries the poison box, so it is never pruned
    /// and its empty tree answers nothing.
    top: Bvh,
    /// The patches, in [`Mesh::patches`] order.
    patches: Vec<PickPatch>,
}

/// One merged candidate of [`MeshPick::candidates`].
#[derive(Clone, Copy, Debug)]
struct Candidate {
    /// The triangle box's conservative entry parameter
    /// ([`bvh::RayCandidate::t_enter`]).
    t_enter: f64,
    /// Flat position (patch-major) — the tie-break's coordinate.
    flat: usize,
    /// Position in [`MeshPick::patches`].
    patch: usize,
    /// Position in that patch's `tris`.
    tri: usize,
}

impl MeshPick {
    /// Builds the index from a tessellated mesh: a [`PickTable`] per
    /// patch — corners, per-triangle boxes, tree, hull — and the
    /// top-level tree over the patches' hulls. Every table is built
    /// here; the memoised form is [`MeshPick::build_with`].
    ///
    /// # Errors
    ///
    /// [`MeshPickError::PositionOutOfRange`] when a triangle indexes
    /// outside [`Mesh::positions`] — corrupt input, never skipped.
    pub fn build(mesh: &Mesh) -> Result<Self, MeshPickError> {
        Self::assemble(mesh, |pi, patch| {
            Ok(Arc::new(PickTable::build(mesh, pi, patch)?))
        })
    }

    /// [`MeshPick::build`] over `memo`'s per-patch tables: the patch at
    /// position `i` is served the table the memo holds under the memo
    /// entry `keys` reports for it, and builds (and stores) one
    /// otherwise. Nothing per triangle runs on a served patch — no
    /// corner copy, no box, no tree — because the entry identity says
    /// the placed corners are the stored ones' bit for bit
    /// ([`mesh::StoredPatchId`]). The top-level
    /// tree is built every time. The index is the same, table for
    /// table and tree for tree, as [`MeshPick::build`]'s
    /// ([`PickMemo`]'s table level).
    ///
    /// `keys` is the tessellation's, one row per patch in the same
    /// order. Both come from one `tessellate_with`, which emits one
    /// patch and one row per face, so a count mismatch is a state only
    /// a kernel bug reaches: it panics (D9 — a bug announces itself),
    /// in every build.
    ///
    /// # Errors
    ///
    /// As [`MeshPick::build`].
    pub(crate) fn build_with(
        mesh: &Mesh,
        keys: &PatchKeys,
        memo: &mut PickMemo,
    ) -> Result<Self, MeshPickError> {
        if keys.len() != mesh.patches.len() {
            unreachable!(
                "pick index: {} keys for {} patches — a tessellation carries one key per face",
                keys.len(),
                mesh.patches.len()
            );
        }
        memo.open();
        Self::assemble(mesh, |pi, patch| {
            memo.table(keys.stored(pi), patch.triangles.len(), || {
                PickTable::build(mesh, pi, patch)
            })
        })
    }

    /// The walk both doors share: per patch, `table_for(patch
    /// position, patch)`, then the top-level tree over the tables'
    /// hulls.
    ///
    /// The walk itself is O(#patches): a patch's triangles are touched
    /// only inside [`PickTable::build`], which the memoised door calls
    /// only on a miss.
    fn assemble(
        mesh: &Mesh,
        mut table_for: impl FnMut(usize, &mesh::FacePatch) -> Result<Arc<PickTable>, MeshPickError>,
    ) -> Result<Self, MeshPickError> {
        let mut patches = Vec::with_capacity(mesh.patches.len());
        let mut hulls = Vec::with_capacity(mesh.patches.len());
        let mut base = 0;
        for (pi, patch) in mesh.patches.iter().enumerate() {
            let table = table_for(pi, patch)?;
            hulls.push(table.hull);
            patches.push(PickPatch {
                table,
                face: patch.face,
                base,
            });
            base += patch.triangles.len();
        }
        Ok(Self {
            top: Bvh::build(&hulls),
            patches,
        })
    }

    /// The ray's candidates, in the single-tree order (type docs): the
    /// top level's candidate patches, each patch's candidate
    /// triangles, merged and sorted by `(t_enter, flat position)`.
    fn candidates(&self, ray: &Ray) -> Vec<Candidate> {
        let mut out = Vec::new();
        for pc in self.top.ray(ray) {
            let Some(patch) = self.patches.get(pc.item) else {
                // Unreachable: the top level was built over exactly
                // `patches`.
                continue;
            };
            out.extend(patch.table.tree.ray(ray).into_iter().map(|c| Candidate {
                t_enter: c.t_enter,
                flat: patch.base + c.item,
                patch: pc.item,
                tri: c.item,
            }));
        }
        // Flat positions are distinct, so the key is a strict total
        // order and the sort is a permutation with no ties to break.
        out.sort_unstable_by(|a, b| a.t_enter.total_cmp(&b.t_enter).then(a.flat.cmp(&b.flat)));
        out
    }

    /// The triangle at a candidate, with its patch's face.
    fn triangle(&self, cand: &Candidate) -> Option<(&PickTri, FaceKey)> {
        let patch = self.patches.get(cand.patch)?;
        Some((patch.table.tris.get(cand.tri)?, patch.face))
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

// The human-readable rendering (LIB-DOORS F6 shape): the two arms this
// door owns state the PROBLEM in the pick-build vocabulary — which
// node, and why its payload offers no body to index — and keep the
// two payload distinctions the enum draws ("never draws" vs "draws
// nothing today"). The wrapping arms FORWARD the payload's own
// `Display` verbatim rather than paraphrasing a vocabulary another
// door owns: `Standing` is the hit-test door's standing prose about
// the same evaluation, and `Tessellate`/`Index` each carry their own
// door's words, prefix included.
impl core::fmt::Display for NodePickError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Standing(error) => write!(f, "{error}"),
            Self::NotABody { node } => write!(
                f,
                "pick: node {}'s value is not body-denoting (a datum, profile, declaration, or \
                 mate), so there is nothing to tessellate and index — offer a body-producing \
                 node instead",
                node.0
            ),
            Self::NoSuchBody { node, body } => write!(
                f,
                "pick: node {}'s value has no output body at index {} — the index is stale, or \
                 that body is currently empty (an annihilated boolean, an empty split side)",
                node.0, body
            ),
            Self::Tessellate(error) => write!(f, "{error}"),
            Self::Index(error) => write!(f, "{error}"),
        }
    }
}

impl core::error::Error for NodePickError {}

/// One node's `Ok` value, or the standing refusal that says why there
/// is none.
///
/// The ladder [`NodePick::build`] and [`NodePick::build_all`] both
/// climb, written once: two spellings of one standing vocabulary is
/// how the two doors come to disagree about a poisoned node.
fn standing_value<T: Decide>(
    eval: &Evaluation<T>,
    node: RecipeNodeId,
) -> Result<&NodeValue<T>, NodePickError> {
    match eval.nodes.get(&node) {
        Some(NodeResult::Ok(value)) => Ok(value),
        Some(NodeResult::Failed(_)) => {
            Err(NodePickError::Standing(HitTestError::NodeFailed { node }))
        }
        Some(NodeResult::Poisoned { through }) => {
            Err(NodePickError::Standing(HitTestError::NodePoisoned {
                node,
                through: *through,
            }))
        }
        None => Err(NodePickError::Standing(HitTestError::NodeNotEvaluated {
            node,
        })),
    }
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
/// source of truth. The mesh and the index are shared, so a clone is
/// a handle: [`PickMemo`] keeps one per displayed (node, body) across
/// pictures, and the index the viewer holds is another.
#[derive(Debug, Clone)]
pub struct NodePick {
    node: RecipeNodeId,
    body: u32,
    mesh: Arc<Mesh>,
    pick: Arc<MeshPick>,
}

/// One memoised (node, body): what it was built for, and the pick.
struct PickEntry {
    document: DocumentId,
    content_key: ContentKey,
    naming_key: NamingKey,
    /// δ and the ambient ε and k, by bit pattern.
    tolerances: [u64; 3],
    pick: NodePick,
    keys: PatchKeys,
    picture: u64,
}

/// What the pick index reuses across pictures: the previous picture's
/// [`NodePick`]s, by (node, body), the per-face patch memo under them,
/// and beside it the per-patch pick tables under the entry
/// identities that memo reports.
///
/// **Node level.** The key is the evaluation memo's own reuse
/// condition, exactly — [`NodeValue::content_key`] AND
/// [`NodeValue::naming_key`] (`eval_node`'s memo hit, whose doc carries
/// the argument: the content key proves the body's bits, the naming
/// key its names and so the face keys the id map is built on) — plus
/// the document's identity (node ids are minted per document) and
/// `(δ, ε, k)`. Under that key the previous picture's `NodePick` IS this
/// picture's, BVH included, because a `NodePick` is a pure function of
/// what the key names.
///
/// **Face level.** A node that was recomputed is tessellated through
/// [`mesh::tessellate_with`] over the patch memo, which answers every
/// face whose inputs did not change; a node reused at node level keeps
/// its faces alive in that memo ([`PatchMemo::keep`]) without looking
/// them up.
///
/// **Table level.** A patch's whole pick table — its triangles'
/// corners, the boxes, the tree over them, their hull ([`PickTable`])
/// — is a function of the patch's PLACED CORNERS alone, and lives
/// here rather than in `mesh`, which stays free of `bvh`. The entry is
/// found by the identity the tessellation reports for the patch
/// ([`mesh::StoredPatchId`]), and that identity
/// is the proof: the patch memo mints one per stored entry and reports
/// it only where it answered a face from that entry — after comparing
/// the entry's FULL key bytes — so two patches under one id have the
/// same key bytes, and the key covers every input their corners are a
/// function of. What the key covers, and why byte-equal keys mean
/// bit-identical placed corners, is stated where the key lives:
/// `mesh::memo`'s module docs, `FaceInputs`, and `StoredPatchId`'s own
/// type docs. Nothing is compared here, per triangle or at all —
/// which is the point: a hit is O(1) per patch, so the only per-build
/// work left is the top-level tree over `#patches` hulls and the
/// patches that missed. A node reused at node level keeps its tables
/// alive without looking them up, as it keeps its patches.
///
/// The two levels are STAMPED in different places, though — a patch
/// entry in `PatchMemo::record`, a table here — so they do not evict
/// in lockstep in one case: a tessellation that refuses partway has
/// counted and stamped its patch hits and never reaches this level for
/// them, so the picture it is closed in keeps those entries with no
/// tables beside them and the next picture rebuilds those tables. A
/// wasted rebuild after a refused picture, never a wrong table.
///
/// **Memory.** The table map holds `Arc`s to the same tables the
/// node-level entries' [`NodePick`]s hold, so its marginal cost is one
/// map slot per patch (an id, a stamp and a pointer — about 3 KB on
/// the tour die's 89 faces), not the tables; the corners, boxes and
/// trees are the index's own, retained by the node level for a picture
/// either way. Moving the corners behind that `Arc` added no copy of
/// them: the index held exactly one before and holds exactly one now.
///
/// **Lifetime.** Whoever builds pictures owns the memo and calls
/// [`PickMemo::end_picture`] after each: entries not used in the
/// picture just built are dropped, at all three levels, so the memo
/// holds exactly one picture's worth.
///
/// The picture/open/close counter machinery here re-spells
/// [`PatchMemo`]'s, twice over (the node level and the table level);
/// the consolidation is
/// `work/perf/fnv-digest-and-memo-machinery-copies.md`.
#[derive(Default)]
pub struct PickMemo {
    nodes: HashMap<(RecipeNodeId, u32), PickEntry>,
    patches: PatchMemo,
    /// The per-patch pick tables, by the memo entry their patch was
    /// placed from (the table level above). Stamped with this memo's
    /// picture, evicted with its nodes.
    tables: HashMap<StoredPatchId, TableEntry>,
    picture: u64,
    /// As [`PatchMemo`]'s: the counts describe the closed picture until
    /// the next build starts.
    closed: bool,
    node_hits: usize,
    node_misses: usize,
    table_hits: usize,
    table_misses: usize,
}

/// One memoised per-patch table and the picture it was last used in.
struct TableEntry {
    table: Arc<PickTable>,
    picture: u64,
}

/// **The dump is held to the declaration**: `Self` is destructured
/// exhaustively, so a field added to [`PickMemo`] is an E0027
/// unbound-pattern error rather than a value silently absent from every
/// dump. `nodes` and `tables` are carried as their COUNTS — the fact a
/// dump is asked for, where the maps themselves are every patch's
/// corners, boxes and trees.
///
/// `closed` is not carried, so this ends in `finish_non_exhaustive`:
/// `finish` claims every field is shown. It is the bit the four
/// counters beside it are qualified by, and a dump that named it would
/// be the better dump — `work/mesh/memo-dumps-hide-the-closed-bit-the-counters-depend-on.md`,
/// which holds [`PatchMemo`]'s identical omission with this one.
impl core::fmt::Debug for PickMemo {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let Self {
            nodes,
            patches,
            tables,
            picture,
            closed: _,
            node_hits,
            node_misses,
            table_hits,
            table_misses,
        } = self;
        f.debug_struct("PickMemo")
            .field("nodes", &nodes.len())
            .field("patches", patches)
            .field("tables", &tables.len())
            .field("picture", picture)
            .field("node_hits", node_hits)
            .field("node_misses", node_misses)
            .field("table_hits", table_hits)
            .field("table_misses", table_misses)
            .finish_non_exhaustive()
    }
}

impl PickMemo {
    /// An empty memo.
    pub fn new() -> Self {
        Self::default()
    }

    /// How many (node, body) picks the memo holds.
    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    /// Whether the memo holds nothing.
    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    /// The face-level memo under the node-level one.
    pub fn patches(&self) -> &PatchMemo {
        &self.patches
    }

    /// (node, body)s answered whole from the memo in the current
    /// picture — or, once [`PickMemo::end_picture`] has closed it and
    /// no build has started the next, in that closed picture.
    pub fn node_hits(&self) -> usize {
        self.node_hits
    }

    /// (node, body)s tessellated (through the patch memo), counted as
    /// [`PickMemo::node_hits`].
    pub fn node_misses(&self) -> usize {
        self.node_misses
    }

    /// How many per-patch pick tables the memo holds (the table
    /// level's [`PickMemo::len`]).
    pub fn table_len(&self) -> usize {
        self.tables.len()
    }

    /// Patches whose whole pick table was served from the memo, over
    /// the same picture [`PickMemo::node_hits`] counts.
    pub fn table_hits(&self) -> usize {
        self.table_hits
    }

    /// Patches whose pick table was built, over the same picture.
    pub fn table_misses(&self) -> usize {
        self.table_misses
    }

    /// The first build after a close starts the next picture's counts.
    fn open(&mut self) {
        if self.closed {
            self.closed = false;
            self.node_hits = 0;
            self.node_misses = 0;
            self.table_hits = 0;
            self.table_misses = 0;
        }
    }

    /// The table level's half of one patch: the stored table under
    /// `stored`, else `build()`, remembered under that identity.
    ///
    /// `stored` is `None` for a patch the patch memo holds no entry
    /// for (the tessellation ran without a memo, or the face's key
    /// could not name one): nothing to serve and nothing to store, so
    /// the table is built and dropped with the index.
    ///
    /// `triangles` is what the mesh says this patch has. A served
    /// table with a different count would mean the identity named
    /// another patch's geometry — a broken threading rather than a
    /// state, so it panics (D9) instead of desynchronising the flat
    /// positions the tie-break is taken on.
    fn table(
        &mut self,
        stored: Option<StoredPatchId>,
        triangles: usize,
        build: impl FnOnce() -> Result<PickTable, MeshPickError>,
    ) -> Result<Arc<PickTable>, MeshPickError> {
        let picture = self.picture;
        if let Some(id) = stored
            && let Some(entry) = self.tables.get_mut(&id)
        {
            if entry.table.tris.len() != triangles {
                unreachable!(
                    "pick index: the memo's table for {id:?} has {} triangles and the mesh's \
                     patch has {triangles}",
                    entry.table.tris.len()
                );
            }
            entry.picture = picture;
            self.table_hits += 1;
            return Ok(Arc::clone(&entry.table));
        }
        self.table_misses += 1;
        let table = Arc::new(build()?);
        if let Some(id) = stored {
            self.tables.insert(
                id,
                TableEntry {
                    table: Arc::clone(&table),
                    picture,
                },
            );
        }
        Ok(table)
    }

    /// Mark `keys`' tables as part of the picture being built (the
    /// table level's [`PatchMemo::keep`]): the caller reused a whole
    /// index and its tables were never looked up.
    fn keep_tables(&mut self, keys: &PatchKeys) {
        let picture = self.picture;
        for id in keys.stored_ids() {
            if let Some(entry) = self.tables.get_mut(&id) {
                entry.picture = picture;
            }
        }
    }

    /// Close the picture at all three levels: drop every entry not
    /// used in it. The counts then describe the picture just closed
    /// until the next build starts.
    ///
    /// The caller's obligation to call this after every picture is
    /// what bounds the memo — the node map, the patch memo AND the
    /// table map: a picture never closed keeps every table it looked
    /// up or built, alive beside the index that holds it.
    pub fn end_picture(&mut self) {
        let picture = self.picture;
        self.nodes.retain(|_, entry| entry.picture == picture);
        self.tables.retain(|_, entry| entry.picture == picture);
        self.picture += 1;
        self.closed = true;
        self.patches.end_picture();
    }
}

fn tolerance_bits(delta: f64, tol: Tol) -> [u64; 3] {
    let ambient = tol.get();
    [delta.to_bits(), ambient.eps.to_bits(), ambient.k.to_bits()]
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
        let value = standing_value(eval, node)?;
        // The payload's body-denoting sources, tagged with the SAME
        // output-body indices the node's name table keys its rows by
        // (`product::sources_of` — the one shipped enumeration; using
        // anything else here would re-mint the pairing this door
        // exists to guarantee).
        let Some(sources) = sources_of(value) else {
            return Err(NodePickError::NotABody { node });
        };
        let Some((_, body_arc, _, _)) = sources.into_iter().find(|(ix, _, _, _)| *ix == body)
        else {
            return Err(NodePickError::NoSuchBody { node, body });
        };
        let mesh = mesh::tessellate(&body_arc, delta, tol).map_err(NodePickError::Tessellate)?;
        let pick = MeshPick::build(&mesh).map_err(NodePickError::Index)?;
        Ok(Self {
            node,
            body,
            mesh: Arc::new(mesh),
            pick: Arc::new(pick),
        })
    }

    /// [`NodePick::build`] over `memo`: the previous picture's pick for
    /// this (node, body) when the node's content and naming keys, the
    /// document and `(delta, tol)` all match, else a build whose
    /// tessellation goes through the patch memo. The answer is byte-identical to
    /// [`NodePick::build`]'s either way ([`PickMemo`]).
    ///
    /// # Errors
    ///
    /// As [`NodePick::build`].
    pub fn build_with(
        eval: &Evaluation<f64>,
        node: RecipeNodeId,
        body: u32,
        delta: f64,
        tol: Tol,
        memo: &mut PickMemo,
    ) -> Result<Self, NodePickError> {
        let value = standing_value(eval, node)?;
        let Some(sources) = sources_of(value) else {
            return Err(NodePickError::NotABody { node });
        };
        let Some((_, body_arc, _, _)) = sources.into_iter().find(|(ix, _, _, _)| *ix == body)
        else {
            return Err(NodePickError::NoSuchBody { node, body });
        };
        memo.open();
        let tolerances = tolerance_bits(delta, tol);
        let picture = memo.picture;
        if let Some(entry) = memo.nodes.get_mut(&(node, body))
            && entry.document == eval.document
            && entry.content_key == value.content_key
            && entry.naming_key == value.naming_key
            && entry.tolerances == tolerances
        {
            entry.picture = picture;
            memo.patches.keep(&entry.keys);
            let keys = entry.keys.clone();
            let pick = entry.pick.clone();
            memo.keep_tables(&keys);
            memo.node_hits += 1;
            return Ok(pick);
        }
        memo.node_misses += 1;
        let tessellation = tessellate_with(&body_arc, delta, tol, &mut memo.patches)
            .map_err(NodePickError::Tessellate)?;
        let pick = MeshPick::build_with(&tessellation.mesh, &tessellation.keys, memo)
            .map_err(NodePickError::Index)?;
        let pick = Self {
            node,
            body,
            mesh: Arc::new(tessellation.mesh),
            pick: Arc::new(pick),
        };
        memo.nodes.insert(
            (node, body),
            PickEntry {
                document: eval.document,
                content_key: value.content_key,
                naming_key: value.naming_key,
                tolerances,
                pick: pick.clone(),
                keys: tessellation.keys,
                picture,
            },
        );
        Ok(pick)
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

    /// Every output body of `node`, each tessellated and indexed —
    /// the enumerating form of [`NodePick::build`].
    ///
    /// A consumer that wants to offer a whole node to a pick cannot
    /// ask "how many bodies does it have": the payload's body indices
    /// are the ones the product gather's `sources_of` assigns and they
    /// are not a dense range (a split with one empty half occupies
    /// index 1 and not index 0). Probing `build` at 0, 1, 2, … until
    /// it refuses is precisely the by-hand pairing this type exists to
    /// remove. So the enumeration is taken HERE, from the same
    /// function the name tables key by, and each element is a
    /// `NodePick` with its pairing established the ordinary way.
    ///
    /// The bodies come back in payload order.
    ///
    /// **An empty vector is a legal answer**, and it means something
    /// narrower than it looks: the node's payload IS body-denoting but
    /// currently denotes none — a boolean that annihilated, a split
    /// whose sides are both empty. A node whose payload is not
    /// body-denoting at all (datum, profile, declarations, mate)
    /// answers [`NodePickError::NotABody`] instead, because "this kind
    /// of node never draws" and "this node draws nothing today" are
    /// different states and only the second changes under an edit.
    ///
    /// # Errors
    ///
    /// As [`NodePick::build`]: node standing, non-body payloads,
    /// tessellation refusals (the first body that refuses stops the
    /// whole enumeration — a partial answer would be a partial
    /// picture), and mesh-index refusals.
    pub fn build_all(
        eval: &Evaluation<f64>,
        node: RecipeNodeId,
        delta: f64,
        tol: Tol,
    ) -> Result<Vec<Self>, NodePickError> {
        let value = standing_value(eval, node)?;
        let Some(sources) = sources_of(value) else {
            return Err(NodePickError::NotABody { node });
        };
        sources
            .into_iter()
            .map(|(ix, _, _, _)| ix)
            .collect::<Vec<_>>()
            .into_iter()
            .map(|body| Self::build(eval, node, body, delta, tol))
            .collect()
    }

    /// [`NodePick::build_all`] over `memo` — each body through
    /// [`NodePick::build_with`].
    ///
    /// # Errors
    ///
    /// As [`NodePick::build_all`].
    pub fn build_all_with(
        eval: &Evaluation<f64>,
        node: RecipeNodeId,
        delta: f64,
        tol: Tol,
        memo: &mut PickMemo,
    ) -> Result<Vec<Self>, NodePickError> {
        let value = standing_value(eval, node)?;
        let Some(sources) = sources_of(value) else {
            return Err(NodePickError::NotABody { node });
        };
        sources
            .into_iter()
            .map(|(ix, _, _, _)| ix)
            .collect::<Vec<_>>()
            .into_iter()
            .map(|body| Self::build_with(eval, node, body, delta, tol, memo))
            .collect()
    }

    /// The tessellation this index was built from — the mesh to
    /// display so that what is drawn is what is picked.
    pub fn mesh(&self) -> &Mesh {
        &self.mesh
    }

    /// The stable name of every face patch of [`NodePick::mesh`], in
    /// patch order (one entry per [`Mesh::patches`] element).
    ///
    /// **The inversion a DISPLAY consumer needs**, and the reason it
    /// lives here: a patch's identity in the mesh is its
    /// [`mesh::FacePatch::face`] arena key, and G1 forbids that key
    /// crossing into layer 3 — so a viewer cannot ask "which name is
    /// this patch" without either handling a key or re-deriving the
    /// pairing this type owns. Both are the failure [`PickTarget`]'s
    /// contract describes. Here the key never leaves: the patch index
    /// goes in, the name comes out.
    ///
    /// Total, per patch, and honest about the loud arm: an
    /// evaluated-but-unnamed face is [`HitTestError::Unnamed`] in ITS
    /// OWN slot rather than a refusal of the whole call, because one
    /// naming-emission bug should not cost a consumer the names of
    /// every other patch it is drawing.
    pub fn patch_names(&self, eval: &Evaluation<f64>) -> Vec<Result<StableName, HitTestError>> {
        self.mesh
            .patches
            .iter()
            .map(|patch| {
                entity_name(
                    eval,
                    self.node,
                    EntityRef {
                        body: self.body,
                        key: EntityKey::Face(patch.face),
                    },
                )
                .cloned()
            })
            .collect()
    }

    /// The stable name of every boundary polyline of
    /// [`NodePick::mesh`], in polyline order (one entry per
    /// [`Mesh::boundaries`] element).
    ///
    /// The edge twin of [`NodePick::patch_names`], and it exists for
    /// the same reason: a polyline's identity in the mesh is its
    /// [`mesh::BoundaryPolyline::edge`] arena key, and G1 forbids that
    /// key crossing into layer 3. A consumer that wants to hit-test
    /// against the drawn edge polylines therefore addresses them by
    /// POSITION — a display coordinate valid for one tessellation —
    /// and reads the name out of here; the key never leaves.
    ///
    /// Total, per polyline, with the loud arm in its own slot: an
    /// evaluated-but-unnamed edge is [`HitTestError::Unnamed`] for
    /// that polyline alone, because one naming-emission bug should not
    /// cost a consumer the names of every other edge it is drawing.
    pub fn boundary_names(&self, eval: &Evaluation<f64>) -> Vec<Result<StableName, HitTestError>> {
        self.mesh
            .boundaries
            .iter()
            .map(|boundary| {
                entity_name(
                    eval,
                    self.node,
                    EntityRef {
                        body: self.body,
                        key: EntityKey::Edge(boundary.edge),
                    },
                )
                .cloned()
            })
            .collect()
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
/// **The answer is the lexicographic minimum over every candidate's
/// exact test, and nothing else.** The traversal early-outs on
/// [`bvh::RayCandidate::t_enter`] for cost alone: candidates are
/// visited in ascending `t_enter` ([`MeshPick::candidates`], the
/// single-tree sequence), and every accepted hit satisfies
/// `t ≥ t_enter` of its own box ([`ray_triangle`] refuses the rest),
/// so once the confirmed best `t` is strictly below a candidate's
/// `t_enter`, every remaining candidate's accepted `t` is at least
/// that `t_enter` — strictly worse, never a tie — and the rest of
/// that target's list cannot change the answer. Without the guard a
/// grazing ray's noise `t` (a candidate whose exact test answers
/// below its own box) could win or lose by where the loop broke,
/// which is what made the answer a function of the candidate order.
/// A poisoned/NaN ray is legal input: the tree returns everything,
/// every exact test misses, and the answer is the typed miss.
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
        for cand in target.pick.candidates(ray) {
            if let Some(b) = &best
                && b.t < cand.t_enter
            {
                // Candidates ascend in t_enter, and every accepted
                // hit is at or above its own: nothing further can
                // reach `b.t`, let alone improve on it.
                break;
            }
            let Some((tri, face)) = target.pick.triangle(&cand) else {
                // Unreachable: the trees were built over exactly the
                // patches' triangles.
                continue;
            };
            if let Some(t) = ray_triangle(ray, &[tri.a, tri.b, tri.c], cand.t_enter) {
                let better = match &best {
                    None => true,
                    // Plain f64 compare is total here: `t` is never
                    // NaN (the exact test refuses non-finite hits).
                    Some(b) => {
                        t < b.t || (t == b.t && (target_pos, cand.flat) < (b.target_pos, b.tri_pos))
                    }
                };
                if better {
                    best = Some(Best {
                        t,
                        target_pos,
                        tri_pos: cand.flat,
                        node: target.node,
                        body: target.body,
                        face,
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

/// The exact ray/triangle test the pick runs (Möller–Trumbore,
/// both-sided, plain `f64`), under the triangle's own box: `Some(t)`
/// iff the ray meets the CLOSED triangle `[a, b, c]` at `t ≥ 0` and
/// `t` is not below `t_enter`, the certified lower bound on any
/// point of the ray inside the triangle's box
/// ([`bvh::RayCandidate::t_enter`] of the box [`bvh::Aabb::from_points`]
/// spans over the corners).
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
///
/// **The box-entry guard, `t < t_enter` on the bits — no tolerance,
/// no `≤`.** A ray lying in the triangle's plane has a determinant
/// that is rounding noise (~1e-19 on a unit-scale mesh), and `u`, `v`
/// and `t` are then noise too; a noise `t` can pass every closed
/// acceptance above while lying OUTSIDE the triangle's own box. Every
/// true hit lies inside that box, and `t_enter` never exceeds the
/// parameter of any point of the ray inside it, so a `t` strictly
/// below `t_enter` is already proven wrong and is refused; a `t` at
/// or above it is left to the acceptance conditions, exactly as
/// before, so an edge or vertex graze at the box's own entry keeps
/// its closed-boundary answer. This is what makes [`pick_face`]'s
/// answer a function of the per-triangle tests and not of the order
/// its early-out visits them in.
///
/// Two things the guard does not do. A noise `t` that lands above
/// `t_enter` and inside the box is not refused here; the
/// order-independence row of `viewer`'s `index_memo` differential is
/// where that would show. And the guard reads the ROUNDED `t`: a
/// genuine graze at the box's own entry face whose test is
/// ill-conditioned (a near-tangent hit, determinant ~1e-4 on a
/// unit-scale mesh) can round below the entry by more than the
/// entry's own widening and is refused too; the point is then
/// answered by another triangle sharing it, at the same `t` to
/// within ULPs. A graze every sharing triangle refuses falls through
/// — the loss the closed acceptance's own rounding in `u` and `v`
/// already produces on such hits, which the guard neither causes nor
/// cures.
///
/// Public at this module, not lifted to the crate root: the reference
/// loop that pins the pick runs the same predicate rather than
/// restating it, and a consumer with a `Bvh` over triangle boxes has
/// everything it takes; it is not a door of the façade.
pub fn ray_triangle(ray: &Ray, tri: &[Point3<f64>; 3], t_enter: f64) -> Option<f64> {
    let e1: Vec3<f64> = tri[1] - tri[0];
    let e2: Vec3<f64> = tri[2] - tri[0];
    let p = ray.dir.cross(e2);
    let det = e1.dot(p);
    if det == 0.0 {
        // Parallel or degenerate. (A NaN det passes THIS check and is
        // refused two comparisons down, at `u_inside`.)
        return None;
    }
    let inv = 1.0 / det;
    let s = ray.origin - tri[0];
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
    // The box-entry guard (docs): a true hit is never below the
    // certified entry of the triangle's own box. `t` is finite here
    // and `t_enter` is never NaN (`bvh`'s contract), so this is
    // exactly the refusal of `t < t_enter`.
    let inside_its_box = t >= t_enter;
    (forward_and_finite && inside_its_box).then_some(t)
}

#[cfg(test)]
#[allow(clippy::expect_used, clippy::panic)]
mod tests {
    //! The table level's two arms that no picture reaches: a patch the
    //! patch memo holds no entry for, and the count guard. Everything
    //! else about [`PickMemo`] is pinned by `viewer`'s `index_memo`
    //! differential, which only ever drives the arms a successful
    //! build takes.

    use geom_core::{Point2, Tol};
    use mesh::tessellate_with;
    use profile::{Profile, ProfileLoop, RawLoop, SketchPlane};
    use sweep::{Extrusion, extrude};
    use topo::Body;

    use super::{MeshPickError, PickMemo, PickTable, ray_triangle};

    fn unit_prism() -> Body<f64> {
        let square = ProfileLoop::polygon([
            Point2::new(0.0, 0.0),
            Point2::new(1.0, 0.0),
            Point2::new(1.0, 1.0),
            Point2::new(0.0, 1.0),
        ]);
        let profile = Profile::new(SketchPlane::xy(), vec![square])
            .validate(Tol::witness())
            .expect("a square validates");
        extrude(&profile, Extrusion::Distance(1.0), Tol::witness())
            .expect("a square extrudes")
            .body
    }

    /// A patch the memo holds no entry for is built every time and
    /// stored nowhere: there is no identity to serve it under, so a
    /// second ask builds a second table rather than finding the first.
    #[test]
    fn a_patch_with_no_entry_identity_is_built_and_never_stored() {
        let mut memo = PickMemo::new();
        let build = || {
            Ok::<_, MeshPickError>(PickTable {
                tris: Vec::new(),
                tree: bvh::Bvh::build(&[]),
                hull: bvh::Aabb::poison(),
            })
        };
        let first = memo.table(None, 0, build).expect("the build succeeds");
        let second = memo.table(None, 0, build).expect("the build succeeds");
        assert!(
            !std::sync::Arc::ptr_eq(&first, &second),
            "with no identity there is nothing to serve: two builds, two tables"
        );
        assert_eq!(memo.table_len(), 0, "and nothing was stored");
        assert_eq!((memo.table_hits(), memo.table_misses()), (0, 2));
    }

    /// The stored table IS served — the build closure is never called
    /// on a hit — and it is served under the identity the tessellation
    /// reported, not under the digest.
    #[test]
    fn a_stored_table_is_served_without_building() {
        let body = unit_prism();
        let mut memo = PickMemo::new();
        let tess = tessellate_with(&body, 0.5, Tol::witness(), &mut memo.patches)
            .expect("the prism meshes");
        let id = tess.keys.stored(0).expect("the first face was stored");
        let patch = &tess.mesh.patches[0];
        let count = patch.triangles.len();
        let built = memo
            .table(Some(id), count, || PickTable::build(&tess.mesh, 0, patch))
            .expect("the build succeeds");
        let served = memo
            .table(Some(id), count, || {
                panic!("a hit must not build");
            })
            .expect("the memo answers");
        assert!(std::sync::Arc::ptr_eq(&built, &served));
        assert_eq!((memo.table_hits(), memo.table_misses()), (1, 1));
    }

    /// A served table whose triangle count is not the mesh patch's
    /// would mean the identity named another patch's geometry — a
    /// broken threading, not a state, so it announces itself (D9)
    /// instead of desynchronising the flat positions the tie-break is
    /// taken on.
    #[test]
    #[should_panic(expected = "triangles and the mesh's patch has")]
    fn a_served_table_that_is_not_this_patchs_panics() {
        let body = unit_prism();
        let mut memo = PickMemo::new();
        let tess = tessellate_with(&body, 0.5, Tol::witness(), &mut memo.patches)
            .expect("the prism meshes");
        let id = tess.keys.stored(0).expect("the first face was stored");
        let patch = &tess.mesh.patches[0];
        let count = patch.triangles.len();
        memo.table(Some(id), count, || PickTable::build(&tess.mesh, 0, patch))
            .expect("the build succeeds");
        let _ = memo.table(Some(id), count + 1, || {
            panic!("the guard fires before the build")
        });
    }

    // ---------------- review probes (review/pick-r1) ----------------

    /// The unguarded exact test, byte-for-byte `ray_triangle` without
    /// the box-entry guard: what `main` answers.
    fn unguarded(
        ray: &bvh::Ray,
        tri: &[geom_core::Point3<f64>; 3],
    ) -> Option<(f64, f64, f64, f64)> {
        use geom_core::Vec3;
        let e1: Vec3<f64> = tri[1] - tri[0];
        let e2: Vec3<f64> = tri[2] - tri[0];
        let p = ray.dir.cross(e2);
        let det = e1.dot(p);
        if det == 0.0 {
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
        let t = e2.dot(q) * inv;
        (t >= 0.0 && t.is_finite()).then_some((t, u, v, det))
    }

    struct Lcg(u64);
    impl Lcg {
        fn next_f64(&mut self) -> f64 {
            self.0 = self
                .0
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            ((self.0 >> 11) as f64) / ((1u64 << 53) as f64)
        }
        fn range(&mut self, lo: f64, hi: f64) -> f64 {
            lo + (hi - lo) * self.next_f64()
        }
    }

    /// **Review probe (not for merge).** Does the box-entry guard
    /// refuse a genuine hit at a point in the triangle's INTERIOR —
    /// where no sibling triangle of a watertight mesh shares the
    /// point, so the answer is lost outright rather than answered by
    /// a neighbour?
    #[test]
    fn probe_guard_refuses_a_genuine_interior_hit() {
        use geom_core::{Point3, Vec3};
        let mut rng = Lcg(0x5eed_1234_9abc_def0);
        let mut found = 0usize;
        let mut worst = String::new();
        let mut worst_ulps = 0i64;
        let mut interior_hits = 0usize;
        for _case in 0..400_000 {
            let mut pt = |r: &mut Lcg| {
                Point3::new(r.range(-1.0, 1.0), r.range(-1.0, 1.0), r.range(-1.0, 1.0))
            };
            let a = pt(&mut rng);
            let b = pt(&mut rng);
            let c = pt(&mut rng);
            let (bu, bv) = (rng.range(0.15, 0.6), rng.range(0.15, 0.6));
            if bu + bv > 0.85 {
                continue;
            }
            let target = a + (b - a) * bu + (c - a) * bv;
            let n = (b - a).cross(c - a);
            let inplane = (b - a) * rng.range(-1.0, 1.0) + (c - a) * rng.range(-1.0, 1.0);
            let scale = 10f64.powf(rng.range(-6.0, -2.0));
            let dir: Vec3<f64> = inplane + n * scale;
            let reach = rng.range(0.5, 4.0);
            let origin = target - dir * reach;
            let ray = bvh::Ray { origin, dir };
            let bx = bvh::Aabb::from_points([a, b, c]).unwrap();
            let Some(t_enter) = ray.slab_enter(&bx) else {
                continue;
            };
            let Some((t, u, v, det)) = unguarded(&ray, &[a, b, c]) else {
                continue;
            };
            if !(u > 0.05 && v > 0.05 && u + v < 0.95) {
                continue;
            }
            interior_hits += 1;
            if t >= t_enter {
                continue;
            }
            found += 1;
            let mut x = t;
            let mut ulps = 0i64;
            while x < t_enter && ulps < 10_000 {
                x = x.next_up();
                ulps += 1;
            }
            if ulps > worst_ulps {
                worst_ulps = ulps;
                worst = format!(
                    "t={t:e} t_enter={t_enter:e} ({ulps} ulp below), u={u:e} v={v:e} det={det:e}\n  a={a:?}\n  b={b:?}\n  c={c:?}\n  origin={origin:?}\n  dir={dir:?}"
                );
            }
            assert_eq!(
                ray_triangle(&ray, &[a, b, c], t_enter),
                None,
                "the guard refuses it"
            );
        }
        println!("PROBE counts: found={found} interior_hits={interior_hits} worst={worst}");
        assert_eq!(
            found, 0,
            "PROBE: {found} genuine INTERIOR hits (of {interior_hits} interior hits drawn) refused by the box-entry guard.\nworst: {worst}"
        );
    }

    /// **Review probe (not for merge).** The same question as
    /// [`probe_guard_refuses_a_genuine_interior_hit`], but for a
    /// triangle whose own box is THIN along one axis — an
    /// axis-planar triangle (a flat cap, an extruded planar face, any
    /// sketch-plane geometry), where the box's entry parameter is the
    /// hit's own parameter for EVERY point of the triangle, not just
    /// for points on the entry face. Only WELL-CONDITIONED hits count:
    /// `|det| / (|e1||e2||d|)` is (up to a constant) the sine of the
    /// angle between the ray and the plane, so a draw above `1e-7` is
    /// a genuine crossing and not plane-noise.
    #[test]
    fn probe_guard_refuses_an_interior_hit_on_a_flat_box() {
        use geom_core::{Point3, Vec3};
        let mut rng = Lcg(0xabcd_0001_2345_6789);
        let mut found = 0usize;
        let mut interior_hits = 0usize;
        let mut worst = String::new();
        let mut max_cond = 0f64;
        for _case in 0..400_000 {
            let zc = rng.range(-1.0, 1.0);
            let thick = if rng.next_f64() < 0.5 {
                0.0
            } else {
                10f64.powf(rng.range(-14.0, -6.0))
            };
            let pt = |r: &mut Lcg, z: f64| Point3::new(r.range(-1.0, 1.0), r.range(-1.0, 1.0), z);
            let a = pt(&mut rng, zc);
            let zb = zc + thick * rng.next_f64();
            let b = pt(&mut rng, zb);
            let zcc = zc + thick * rng.next_f64();
            let c = pt(&mut rng, zcc);
            let (bu, bv) = (rng.range(0.15, 0.6), rng.range(0.15, 0.6));
            if bu + bv > 0.85 {
                continue;
            }
            let target = a + (b - a) * bu + (c - a) * bv;
            let n = (b - a).cross(c - a);
            let inplane = (b - a) * rng.range(-1.0, 1.0) + (c - a) * rng.range(-1.0, 1.0);
            let scale = 10f64.powf(rng.range(-8.0, -2.0));
            let dir: Vec3<f64> = inplane + n * scale;
            let reach = rng.range(0.5, 4.0);
            let origin = target - dir * reach;
            let ray = bvh::Ray { origin, dir };
            let bx = bvh::Aabb::from_points([a, b, c]).unwrap();
            let Some(t_enter) = ray.slab_enter(&bx) else {
                continue;
            };
            let Some((t, u, v, det)) = unguarded(&ray, &[a, b, c]) else {
                continue;
            };
            if !(u > 0.05 && v > 0.05 && u + v < 0.95) {
                continue;
            }
            let (ee1, ee2) = (b - a, c - a);
            let cond = det.abs() / (ee1.norm() * ee2.norm() * dir.norm());
            if cond < 1e-7 {
                continue;
            }
            interior_hits += 1;
            if t >= t_enter {
                continue;
            }
            found += 1;
            let mut x = t;
            let mut ulps = 0i64;
            while x < t_enter && ulps < 100_000 {
                x = x.next_up();
                ulps += 1;
            }
            if cond > max_cond {
                max_cond = cond;
                worst = format!(
                    "t={t:e} t_enter={t_enter:e} ({ulps} ulp below), u={u:e} v={v:e} det={det:e} cond={cond:e} thick={thick:e}\n  a={a:?}\n  b={b:?}\n  c={c:?}\n  origin={origin:?}\n  dir={dir:?}"
                );
            }
            assert_eq!(
                ray_triangle(&ray, &[a, b, c], t_enter),
                None,
                "the guard refuses it"
            );
        }
        println!(
            "PROBE-FLAT: found={found} well_conditioned_interior_hits={interior_hits} max_cond_among_refusals={max_cond:e}\nbest-conditioned refusal: {worst}"
        );
        assert_eq!(found, 0, "genuine interior hits refused on a flat box");
    }

    /// **Review probe (not for merge).** The same loss on REAL mesh
    /// geometry: the unit prism's axis-planar cap, picked nearly
    /// edge-on. Runs `pick_face`'s own candidate loop twice over
    /// `MeshPick` — once with the shipped guarded predicate, once with
    /// `main`'s unguarded one — and reports rays whose answer the
    /// guard moved from a well-conditioned INTERIOR hit to a
    /// different face or a miss.
    #[test]
    fn probe_the_prism_cap_loses_an_edge_on_pick() {
        use geom_core::{Point3, Vec3};
        let body = unit_prism();
        let mut pmemo = mesh::PatchMemo::new();
        let mesh = tessellate_with(&body, 0.01, Tol::witness(), &mut pmemo)
            .expect("the prism tessellates")
            .mesh;
        // The same prism, standing at a non-dyadic place in the
        // document — the only difference from the axis-exact one.
        let mut mesh = mesh;
        let off = Vec3::new(0.3141592653589793, 0.2718281828459045, 0.5772156649015329);
        for p in &mut mesh.positions {
            *p = *p + off;
        }
        let index = super::MeshPick::build(&mesh).expect("the prism indexes");
        let mut rng = Lcg(0x1357_9bdf_2468_ace0);
        let mut moved = 0usize;
        let mut aimed = 0usize;
        let mut worst = String::new();
        let mut max_cond = 0f64;
        for _case in 0..200_000 {
            // A point in the interior of the prism's top cap.
            let target = Point3::new(rng.range(0.15, 0.85) + off.x, rng.range(0.15, 0.85) + off.y, 1.0 + off.z);
            // Nearly edge-on: mostly in the z = 1 plane.
            let slope = 10f64.powf(rng.range(-6.0, -1.5));
            let theta = rng.range(0.0, 6.283_185_307_179_586);
            let dir = Vec3::new(theta.cos(), theta.sin(), -slope);
            let reach = rng.range(0.5, 3.0);
            let origin = target - dir * reach;
            let ray = bvh::Ray { origin, dir };
            // pick_face's loop, guarded (shipped) and unguarded (main).
            let mut best_g: Option<(f64, usize)> = None;
            let mut best_u: Option<(f64, usize, f64, f64, f64)> = None;
            for cand in index.candidates(&ray) {
                let Some((tri, _)) = index.triangle(&cand) else {
                    continue;
                };
                let corners = [tri.a, tri.b, tri.c];
                if let Some(t) = ray_triangle(&ray, &corners, cand.t_enter)
                    && best_g.as_ref().is_none_or(|b| t < b.0)
                {
                    best_g = Some((t, cand.flat));
                }
                if let Some((t, u, v, det)) = unguarded(&ray, &corners)
                    && best_u.as_ref().is_none_or(|b| t < b.0)
                {
                    best_u = Some((t, cand.flat, u, v, det));
                }
            }
            let Some((tu, fu, u, v, det)) = best_u else {
                continue;
            };
            // The unguarded winner is a well-conditioned interior hit.
            let Some((tri, _)) = index.triangle(&super::Candidate {
                t_enter: 0.0,
                flat: fu,
                patch: 0,
                tri: fu,
            }) else {
                continue;
            };
            let (ee1, ee2) = (tri.b - tri.a, tri.c - tri.a);
            let cond = det.abs() / (ee1.norm() * ee2.norm() * dir.norm());
            if !(u > 0.05 && v > 0.05 && u + v < 0.95) || cond < 1e-7 {
                continue;
            }
            aimed += 1;
            let same = best_g.as_ref().is_some_and(|b| b.0 == tu && b.1 == fu);
            if same {
                continue;
            }
            moved += 1;
            if cond > max_cond {
                max_cond = cond;
                worst = format!(
                    "unguarded: t={tu:e} flat={fu} u={u:e} v={v:e} det={det:e} cond={cond:e}\n  guarded: {best_g:?}\n  origin={origin:?}\n  dir={dir:?}"
                );
            }
        }
        println!(
            "PROBE-PRISM: moved={moved} of {aimed} well-conditioned interior cap hits; max_cond_among_moved={max_cond:e}\nbest-conditioned move: {worst}"
        );
        assert_eq!(moved, 0, "the guard moved a real mesh pick");
    }

    /// **Review probe (not for merge).** The flat-box loss on the
    /// shape a CAD tessellator actually emits for a round planar face:
    /// a fan-triangulated disc at a non-dyadic height, picked nearly
    /// edge-on. Every triangle of the fan is axis-planar (its box has
    /// zero z-extent, so the box's entry IS the hit's own parameter
    /// everywhere on it) and thin (the fan's apex angle). Reports hits
    /// in the OPEN interior of a fan triangle — no sibling triangle of
    /// a watertight mesh shares such a point — that the guard refuses.
    #[test]
    fn probe_a_fan_triangulated_cap_loses_an_interior_hit() {
        use geom_core::{Point3, Vec3};
        let z = 0.5772156649015329;
        let n = 64usize;
        let centre = Point3::new(0.0, 0.0, z);
        let rim: Vec<Point3<f64>> = (0..n)
            .map(|i| {
                let a = (i as f64) * std::f64::consts::TAU / (n as f64);
                Point3::new(a.cos(), a.sin(), z)
            })
            .collect();
        let tris: Vec<[Point3<f64>; 3]> = (0..n)
            .map(|i| [centre, rim[i], rim[(i + 1) % n]])
            .collect();
        let mut rng = Lcg(0x0f0f_1234_5678_9abc);
        let mut refused = 0usize;
        let mut interior = 0usize;
        let mut worst = String::new();
        let mut max_cond = 0f64;
        for _case in 0..200_000 {
            let k = (rng.next_f64() * (n as f64)) as usize % n;
            let tri = tris[k];
            let (bu, bv) = (rng.range(0.15, 0.6), rng.range(0.15, 0.6));
            if bu + bv > 0.85 {
                continue;
            }
            let target = tri[0] + (tri[1] - tri[0]) * bu + (tri[2] - tri[0]) * bv;
            let slope = 10f64.powf(rng.range(-6.0, -1.5));
            let theta = rng.range(0.0, std::f64::consts::TAU);
            let dir = Vec3::new(theta.cos(), theta.sin(), -slope);
            let reach = rng.range(0.5, 3.0);
            let origin = target - dir * reach;
            let ray = bvh::Ray { origin, dir };
            let bx = bvh::Aabb::from_points(tri).unwrap();
            let Some(t_enter) = ray.slab_enter(&bx) else {
                continue;
            };
            let Some((t, u, v, det)) = unguarded(&ray, &tri) else {
                continue;
            };
            if !(u > 0.05 && v > 0.05 && u + v < 0.95) {
                continue;
            }
            let (ee1, ee2) = (tri[1] - tri[0], tri[2] - tri[0]);
            let cond = det.abs() / (ee1.norm() * ee2.norm() * dir.norm());
            if cond < 1e-7 {
                continue;
            }
            interior += 1;
            if t >= t_enter {
                continue;
            }
            refused += 1;
            if cond > max_cond {
                max_cond = cond;
                worst = format!(
                    "t={t:e} t_enter={t_enter:e} u={u:e} v={v:e} det={det:e} cond={cond:e} wedge={k}\n  origin={origin:?}\n  dir={dir:?}"
                );
            }
            assert_eq!(ray_triangle(&ray, &tri, t_enter), None);
        }
        println!(
            "PROBE-FAN: refused={refused} of {interior} well-conditioned interior hits; max_cond={max_cond:e}\nbest-conditioned refusal: {worst}"
        );
        assert_eq!(refused, 0, "a fan cap loses an interior hit to the guard");
    }

    /// **Review probe (not for merge).** The residue the spec asked to
    /// measure: a NOISE hit (the ray all but in the triangle's plane,
    /// `|det|` at rounding level) whose noise `t` lands at or ABOVE
    /// its own box's entry and so survives the guard. General-position
    /// triangles; the out-of-plane component is drawn down to `1e-18`.
    #[test]
    fn probe_noise_hits_that_survive_the_guard() {
        use geom_core::{Point3, Vec3};
        let mut rng = Lcg(0x2222_3333_4444_5555);
        let mut accepted = 0usize;
        let mut refused = 0usize;
        let mut worst = String::new();
        for _case in 0..400_000 {
            let pt = |r: &mut Lcg| {
                Point3::new(r.range(-1.0, 1.0), r.range(-1.0, 1.0), r.range(-1.0, 1.0))
            };
            let a = pt(&mut rng);
            let b = pt(&mut rng);
            let c = pt(&mut rng);
            let (bu, bv) = (rng.range(0.1, 0.6), rng.range(0.1, 0.6));
            if bu + bv > 0.85 {
                continue;
            }
            let target = a + (b - a) * bu + (c - a) * bv;
            let n = (b - a).cross(c - a);
            let inplane = (b - a) * rng.range(-1.0, 1.0) + (c - a) * rng.range(-1.0, 1.0);
            let scale = 10f64.powf(rng.range(-20.0, -15.0));
            let dir: Vec3<f64> = inplane + n * scale;
            let reach = rng.range(0.5, 3.0);
            let origin = target - dir * reach;
            let ray = bvh::Ray { origin, dir };
            let bx = bvh::Aabb::from_points([a, b, c]).unwrap();
            let Some(t_enter) = ray.slab_enter(&bx) else {
                continue;
            };
            let Some((t, u, v, det)) = unguarded(&ray, &[a, b, c]) else {
                continue;
            };
            let (ee1, ee2) = (b - a, c - a);
            let cond = det.abs() / (ee1.norm() * ee2.norm() * dir.norm());
            if cond > 1e-14 {
                continue;
            }
            if ray_triangle(&ray, &[a, b, c], t_enter).is_some() {
                accepted += 1;
                if worst.is_empty() {
                    worst = format!(
                        "t={t:e} t_enter={t_enter:e} u={u:e} v={v:e} det={det:e} cond={cond:e}\n  a={a:?}\n  b={b:?}\n  c={c:?}\n  origin={origin:?}\n  dir={dir:?}"
                    );
                }
            } else {
                refused += 1;
            }
        }
        println!(
            "PROBE-NOISE: noise hits accepted_by_the_guard={accepted} refused={refused}\nfirst survivor: {worst}"
        );
        assert_eq!(accepted, 0, "a noise hit survived the guard");
    }
}
