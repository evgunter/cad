//! The hit-test service (G1: `ray → stable ref`, an editor-core
//! service on the mesh back-references and the [`super::hit`]
//! inversion).
//!
//! The chain: [`bvh::Bvh::ray`] over the patches' boxes, then over
//! the candidate patches' per-triangle boxes, merged into one
//! candidate sequence ([`MeshPick`]) → exact ray/triangle tests in
//! plain `f64` → nearest hit by `t` with a total, documented tie-break
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
/// [`mesh::StoredPatchId`](mesh::StoredPatchId), which says the placed
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
/// A pick is a function of the candidate SEQUENCE the tree hands the
/// exact test — not only of the set: the loop early-outs on the
/// conservative entry parameter, and the exact test can answer a `t`
/// outside a grazed triangle's own box (a ray in the triangle's
/// plane), so which such answers the loop reaches is decided by the
/// order. [`MeshPick::candidates`] therefore reproduces the single-
/// tree sequence verbatim:
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
///   of the same triangle box.
/// - **Same order.** The merged list is sorted by
///   `(t_enter under total_cmp, flat position)`, the single tree's
///   documented order.
///
/// So the loop in [`pick_face`] sees the sequence one tree over all
/// the triangles would give it, and answers bit-identically — the
/// invariant `viewer`'s `index_memo` differential pins against a
/// single-level reference, tie-break row included.
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
    /// ([`mesh::StoredPatchId`](mesh::StoredPatchId)). The top-level
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
/// ([`mesh::StoredPatchId`](mesh::StoredPatchId)), and that identity
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

impl core::fmt::Debug for PickMemo {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("PickMemo")
            .field("nodes", &self.nodes.len())
            .field("patches", &self.patches)
            .field("tables", &self.tables.len())
            .field("picture", &self.picture)
            .field("node_hits", &self.node_hits)
            .field("node_misses", &self.node_misses)
            .field("table_hits", &self.table_hits)
            .field("table_misses", &self.table_misses)
            .finish()
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
/// The traversal early-outs on [`bvh::RayCandidate::t_enter`] (a
/// conservative lower bound on any hit in that box): candidates are
/// visited in ascending `t_enter` ([`MeshPick::candidates`], the
/// single-tree sequence), and once the confirmed best `t` is strictly
/// below a candidate's `t_enter` the rest of that target's list cannot
/// improve on it. A poisoned/NaN ray is legal input: the
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
        for cand in target.pick.candidates(ray) {
            if let Some(b) = &best
                && b.t < cand.t_enter
            {
                // Candidates ascend in t_enter, a lower bound on any
                // hit in their box: nothing further can improve.
                break;
            }
            let Some((tri, face)) = target.pick.triangle(&cand) else {
                // Unreachable: the trees were built over exactly the
                // patches' triangles.
                continue;
            };
            if let Some(t) = ray_triangle(ray, tri) {
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
