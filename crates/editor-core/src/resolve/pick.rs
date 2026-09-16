//! The hit-test service (G1: `ray → stable ref`, an editor-core
//! service on the mesh back-references and the [`super::hit`]
//! inversion).
//!
//! The chain: [`bvh::Bvh::ray`] over the patches' boxes, then over
//! the candidate patches' per-triangle boxes, merged into one
//! candidate sequence ([`MeshPick`]) → exact ray/triangle tests in
//! plain `f64`, each on a determinant certified non-zero AND on
//! barycentrics certified to say something — a candidate whose
//! rounding interval covers the whole admissible range is refused
//! rather than answered from ([`ray_triangle`], [`crossing`]) →
//! nearest hit by `t` with a total, documented tie-break
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
    /// The corners, in the mesh's winding — the exact test's operand.
    corners: [Point3<f64>; 3],
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
            // `from_points` is `None` only for an empty iterator;
            // three points always yield a box. The unreachable arm
            // degrades to poison — never pruned — rather than a
            // panic (fail-safe direction).
            boxes.push(Aabb::from_points(corners).unwrap_or_else(Aabb::poison));
            tris.push(PickTri { corners });
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
/// A pick is a function of the candidate SET: each candidate's exact
/// test ([`ray_triangle`]) reads the ray and the triangle and nothing
/// else, and the loop's early-out on the box entry prunes only
/// candidates that could not win, up to the rounding of a near-tie
/// ([`pick_face`]). What the order otherwise decides is the WORK —
/// which candidates are tested before the early-out fires — so
/// [`MeshPick::candidates`] reproduces the single-tree sequence
/// verbatim, and the two-level index costs what one tree would:
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
/// single-level reference that tests every candidate, tie-break row
/// included.
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
/// exact test.** Each test reads the ray and the triangle alone, and
/// a ray in a triangle's plane refuses at the determinant
/// ([`ray_triangle`]) rather than answering a noise `t`, so no
/// candidate's answer depends on which candidates were visited
/// before it. The traversal early-outs on
/// [`bvh::RayCandidate::t_enter`] for cost: candidates are visited in
/// ascending `t_enter` ([`MeshPick::candidates`], the single-tree
/// sequence), and a true hit's parameter is never below its own box's
/// entry, so once the confirmed best `t` is strictly below a
/// candidate's `t_enter` nothing further can beat it in exact
/// arithmetic. In `f64` the accepted `t` is rounded and can sit a few
/// ULP below its own entry (an axis-planar triangle's box has zero
/// extent along one axis, so the entry IS the hit's parameter
/// everywhere on it), so the early-out can break before a candidate
/// whose rounded `t` would have tied or beaten `best.t` by ULPs: the
/// tie-rounding class of the closed acceptance
/// (`work/edit/pick-closed-acceptance-loses-a-graze-to-rounding.md`),
/// which decides among triangles sharing a point, never between a
/// hit and a miss. A poisoned/NaN ray is legal input: the tree
/// returns everything, every exact test misses, and the answer is
/// the typed miss.
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
                // true hit in their box: nothing further can improve
                // beyond the rounding of a near-tie (docs).
                break;
            }
            let Some((tri, face)) = target.pick.triangle(&cand) else {
                // Unreachable: the trees were built over exactly the
                // patches' triangles.
                continue;
            };
            if let Some(t) = ray_triangle(ray, &tri.corners) {
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
/// `f64`): `Some(t)` iff the ray meets the CLOSED triangle at `t ≥ 0`
/// on a determinant certified non-zero ([`crossing`]),
/// with `t` the parameter of the hit point `a + u·e1 + v·e2` along
/// the ray rather than Möller–Trumbore's quotient `e2·q / det` — the
/// two agree to rounding on a well-conditioned crossing, and only
/// the projection survives a small determinant (a ray through a
/// corner of a triangle whose plane it all but contains: `u = v = 0`
/// exactly, the quotient off the corner by parts per thousand).
///
/// Boundary semantics: `u ∈ [0, 1]`, `v ∈ [0, 1]`, `u + v ∈ [0, 1]`,
/// `t ≥ 0` — all closed, so a hit exactly on a shared edge or vertex
/// is a hit for EVERY incident triangle (the caller's tie-break
/// disambiguates; watertight meshes never lose a graze to an open
/// boundary) and the hit point `a + u·e1 + v·e2` is a point OF the
/// closed triangle, which is what lets a caller stop its walk at a
/// candidate box the nearest hit already precedes.
///
/// A value inside those bounds is not enough. Each barycentric also
/// carries the forward bound on its own rounding ([`crossing`], where
/// the derivation lives), and a candidate is refused when that
/// interval COVERS the whole of `[0, 1]`: a quotient by a
/// certified-but-small determinant can land in range by chance while
/// its interval says nothing about where — or whether — the ray met
/// the triangle. Refusing it is the determinant's own posture one
/// level down, and **it is not free**. The bound is a function of
/// `|s|`, `|d|` and the edges, not of the numerator's value, so it
/// does not vanish where the barycentric does: a genuine graze
/// through a corner of a candidate at the certification's floor is
/// refused too, unless the ray's components happen to zero the
/// triple bound. Such a graze is answered by the candidate's
/// better-conditioned neighbours or not at all — the same trade the
/// certification already makes one level down, stated here because
/// the corpus pays it (`work/edit/pick-closed-acceptance-loses-a-graze-to-rounding`,
/// and the labelling asymmetry it exposes,
/// `work/edit/pick-a-corner-graze-verdict-depends-on-the-corner-labelling`).
/// A determinant that is
/// not certifiably non-zero — the ray parallel or near-parallel to
/// the plane, a degenerate triangle, any NaN — is a miss, and so is a
/// NON-FINITE `t`: a hit the service cannot place at a finite point is
/// never a `PickHit` whose `point` would be `0 · ∞ = NaN`. Every
/// acceptance is an affirmative comparison on the rounded value, which
/// a NaN fails.
///
/// This is the one exact test; the reference loop that pins the pick
/// (`viewer`'s `index_memo`) calls it rather than restating it, which
/// is why it is public at this module and nowhere else.
pub fn ray_triangle(ray: &Ray, tri: &[Point3<f64>; 3]) -> Option<f64> {
    let Crossing { barycentrics, .. } = crossing(ray, tri)?;
    if !barycentrics.iter().all(|&(x, err)| admits(x, err)) {
        return None;
    }
    let [(u, _), (v, _), _] = barycentrics;
    let e1: Vec3<f64> = tri[1] - tri[0];
    let e2: Vec3<f64> = tri[2] - tri[0];
    // The parameter of the hit POINT `a + u·e1 + v·e2` along the ray,
    // not Möller–Trumbore's `e2·q / det`: the quotient cancels
    // catastrophically when the determinant is small (a ray grazing
    // a triangle whose plane it nearly contains), while the point's
    // projection onto the ray is conditioned by `u` and `v` alone.
    let hit = tri[0] + e1 * u + e2 * v;
    let t = (hit - ray.origin).dot(ray.dir) / ray.dir.dot(ray.dir);
    let forward_and_finite = t >= 0.0 && t.is_finite();
    forward_and_finite.then_some(t)
}

/// Everything the arithmetic can certify about one ray/triangle
/// crossing, from one evaluation of it: the answer [`crossing`] gives
/// and the only door the exact test and the corpus rows read.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Crossing {
    /// Möller–Trumbore's determinant `e1 · (d × e2)`, computed. Its
    /// magnitude exceeds [`Crossing::bound_det`] — that is what makes
    /// it a `Crossing` at all — so its SIGN, and the fact that the ray
    /// crosses the plane, are certified.
    pub det: f64,
    /// The forward rounding bound on that evaluation of the
    /// determinant.
    pub bound_det: f64,
    /// `[(u, err_u), (v, err_v), (u + v, err_sum)]` — each barycentric
    /// as computed, with the forward bound on ITS OWN rounding.
    pub barycentrics: [(f64, f64); 3],
}

impl Crossing {
    /// `|det| / (|e1|·|e2|·|d|)` — up to a constant the sine of the
    /// angle between the ray and the plane, and so the conditioning of
    /// the whole test on this pair. Not derivable from the fields
    /// alone, which is why it takes the operands again.
    pub fn conditioning(ray: &Ray, tri: &[Point3<f64>; 3]) -> Option<f64> {
        let c = crossing(ray, tri)?;
        let e1: Vec3<f64> = tri[1] - tri[0];
        let e2: Vec3<f64> = tri[2] - tri[0];
        Some(c.det.abs() / (e1.norm() * e2.norm() * ray.dir.norm()))
    }
}

/// **The one door onto the crossing**: `None` when the determinant is
/// not certifiably non-zero — the ray parallel or within rounding of
/// the triangle's plane, a degenerate triangle, any NaN — where the
/// barycentrics and `t` would be quotients of rounding noise by
/// rounding noise, which is the answer [`ray_triangle`] refuses to
/// give. Otherwise the determinant, its bound, and the three
/// barycentrics with theirs, computed ONCE. [`ray_triangle`] reads
/// it, and so do the corpus rows that measure what the acceptance
/// refuses: one evaluation, one set of numbers, no second spelling to
/// drift.
///
/// # What the bounds bound
///
/// `err` bounds `|computed − true|` for that barycentric **given the
/// operands `e1`, `e2`, `d`, `s` as exact**. It is a bound on this
/// evaluation's rounding, not on the mesh's own coordinates: a
/// triangle whose corners are themselves approximations is a question
/// for whoever tessellated it, and nothing here can see it.
///
/// # The derivation (the one site; [`ray_triangle`] cites it)
///
/// `u = fl(fl(s·p) · fl(1/det))` with `p = d × e2`, so its error is
/// the numerator's forward bound, plus the determinant's scaled by
/// `|u|`, plus the two roundings of the quotient, all over the
/// smallest magnitude the true determinant can have. Writing `ũ`,
/// `Ñ`, `D̃` for the computed values and `u = N/D` for the true
/// quotient,
/// `ũ − N/D = (ũ − Ñ/D̃) + [(Ñ − N) − u·(D̃ − D)]/D̃`, and
/// `|u| ≤ |ũ| + |ũ − u|` closes the recursion into
/// `err = (γ₂|Ñ| + bound_N + |ũ|·bound_D) / (|D̃| − bound_D)`, whose
/// denominator is positive exactly because the determinant is
/// certified. `v`'s numerator `d·(s × e1)` is the same triple product
/// with different operands and takes the same bound
/// ([`DETERMINANT_ERROR_UNITS`]).
///
/// The sum carries both, plus one more term for the rounding of the
/// ADDITION `fl(u + v)` itself: that rounding is bounded by
/// `u_r·|fl(u + v)|` — relative to the computed sum, which is the
/// value in hand — hence [`SUM_ERROR_UNITS`] over `sum.abs()` and not
/// over the true sum.
pub fn crossing(ray: &Ray, tri: &[Point3<f64>; 3]) -> Option<Crossing> {
    let e1: Vec3<f64> = tri[1] - tri[0];
    let e2: Vec3<f64> = tri[2] - tri[0];
    let p = ray.dir.cross(e2);
    let (det, bound_det) = certify(e1, e2, ray.dir, p)?;
    let inv = 1.0 / det;
    let s = ray.origin - tri[0];
    let (u, err_u) = quotient(s.dot(p), triple_bound(s, ray.dir, e2), inv, det, bound_det);
    let q = s.cross(e1);
    let (v, err_v) = quotient(
        ray.dir.dot(q),
        triple_bound(ray.dir, s, e1),
        inv,
        det,
        bound_det,
    );
    let sum = u + v;
    let err_sum = err_u + err_v + SUM_ERROR_UNITS * f64::EPSILON * sum.abs();
    Some(Crossing {
        det,
        bound_det,
        barycentrics: [(u, err_u), (v, err_v), (sum, err_sum)],
    })
}

/// One barycentric from its numerator and the certified determinant,
/// with the bound [`crossing`] derives.
///
/// The bound is itself computed in `f64`, so it carries its own
/// roundings — at most four over the numerator's terms and one on the
/// division, `γ₅ < 11·u` relative. They are covered by the slack in
/// [`DETERMINANT_ERROR_UNITS`] and [`QUOTIENT_ERROR_UNITS`], which
/// round `5.01·u` up to `6·u` and `γ₂ < 2.0000000000000004·u` up to
/// `4·u`: a `1.2×` and a `2×` margin on terms that are already the
/// whole of `err`, against a `1 + 11·u` shortfall.
fn quotient(num: f64, bound_num: f64, inv: f64, det: f64, bound_det: f64) -> (f64, f64) {
    let x = num * inv;
    let err = (QUOTIENT_ERROR_UNITS * f64::EPSILON * num.abs() + bound_num + x.abs() * bound_det)
        / (det.abs() - bound_det);
    (x, err)
}

/// The ruling on one barycentric: admitted iff the computed value is
/// in the closed range `[0, 1]` AND its interval does not COVER that
/// range. The closed comparison is the boundary semantics
/// [`ray_triangle`] documents — a hit exactly on a shared edge or
/// vertex is a hit for every incident triangle. Not covering is the
/// demand that the number carry information: an interval spanning the
/// whole range is consistent with every point of the triangle and
/// with every point outside it, and so answers nothing. The two
/// compose: a value inside the range whose bound is `1` or more
/// necessarily covers it, so no admitted barycentric carries a bound
/// that wide. Both halves are affirmative comparisons, so a NaN is
/// refused.
fn admits(x: f64, err: f64) -> bool {
    let inside = (0.0..=1.0).contains(&x);
    let informs = x - err > 0.0 || x + err < 1.0;
    inside && informs
}

/// The triple product's constant, in units of `f64::EPSILON`, derived
/// from the operation count (`u = EPSILON / 2` is the unit roundoff,
/// `γ_n = n·u / (1 − n·u)` the standard bound on `n` roundings). It
/// bounds every `a · (b × c)` the test evaluates — the determinant at
/// `a = e1, b = d, c = e2`, `u`'s numerator at `a = s`, `v`'s at
/// `a = d, b = s, c = e1`:
///
/// - each component `w_i = fl(fl(b_j·c_k) − fl(b_k·c_j))` of the
///   cross product carries ≤ `γ_2 · S_i`, with
///   `S_i = |b_j·c_k| + |b_k·c_j|`;
/// - the dot `fl(Σ a_i·w_i)` (three products, two sums) carries
///   ≤ `γ_3 · Σ|a_i·w_i|` of its own, and `|w_i| ≤ (1 + γ_2)·S_i`;
/// - the propagated cross-product error is ≤ `γ_2 · Σ|a_i|·S_i`.
///
/// So `|fl − exact| ≤ (γ_2 + γ_3 + γ_2·γ_3) · M < 5.01·u · M` with
/// `M = Σ_i |a_i|·S_i` exact. `M` is itself computed with ≤ 6
/// roundings per term and the bound with one more, so the computed
/// bound is at least `(1 − γ_7)` of its exact value; `3 · EPSILON =
/// 6·u` clears `5.01·u / (1 − γ_7)` with margin. Not a tuned number:
/// change the arithmetic and re-count.
const DETERMINANT_ERROR_UNITS: f64 = 3.0;

/// The sum's constant, same units and same style: `fl(u + v)` is ONE
/// rounding, bounded by `u_r·|fl(u + v)|` with `u_r = EPSILON / 2`.
/// Not a tuned number: add the two differently and re-count.
const SUM_ERROR_UNITS: f64 = 0.5;

/// The quotient's constant, same units and same style: `fl(1/D̃)` and
/// then one multiply is two roundings, so the quotient's own
/// contribution is `≤ γ_2·|Ñ/D̃|` with
/// `γ_2 = 2u / (1 − 2u) < 4u = 2 · EPSILON`. Not a tuned number:
/// spell the division differently and re-count.
const QUOTIENT_ERROR_UNITS: f64 = 2.0;

/// The forward bound on the computed `a · (b × c)`:
/// `3 · EPSILON · Σ_i |a_i|·S_i`, per [`DETERMINANT_ERROR_UNITS`].
fn triple_bound(a: Vec3<f64>, b: Vec3<f64>, c: Vec3<f64>) -> f64 {
    let m = a.x.abs() * (b.y.abs() * c.z.abs() + b.z.abs() * c.y.abs())
        + a.y.abs() * (b.z.abs() * c.x.abs() + b.x.abs() * c.z.abs())
        + a.z.abs() * (b.x.abs() * c.y.abs() + b.y.abs() * c.x.abs());
    DETERMINANT_ERROR_UNITS * f64::EPSILON * m
}

/// The certification itself, over the operands [`ray_triangle`] has
/// in hand (`p = d × e2` computed once, shared with the barycentrics),
/// answering the determinant WITH its bound: the barycentrics divide
/// by the one and widen by the other.
fn certify(e1: Vec3<f64>, e2: Vec3<f64>, d: Vec3<f64>, p: Vec3<f64>) -> Option<(f64, f64)> {
    let det = e1.dot(p);
    let bound = triple_bound(e1, d, e2);
    // A NaN anywhere fails the comparison: poison is un-hittable.
    (det.abs() > bound).then_some((det, bound))
}

#[cfg(test)]
#[allow(clippy::expect_used, clippy::panic)]
mod tests {
    //! The table level's two arms that no picture reaches: a patch the
    //! patch memo holds no entry for, and the count guard. Everything
    //! else about [`PickMemo`] is pinned by `viewer`'s `index_memo`
    //! differential, which only ever drives the arms a successful
    //! build takes.

    use bvh::Ray;
    use geom_core::{Point2, Point3, Tol, Vec3};
    use mesh::tessellate_with;
    use profile::{Profile, ProfileLoop, RawLoop, SketchPlane};
    use sweep::{Extrusion, extrude};
    use test_utils::fuzz;
    use topo::Body;

    use super::{
        MeshPick, MeshPickError, PickMemo, PickTable, crossing, ray_triangle,
    };

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

    // ---------------- the exact test's own rows ----------------
    //
    // The rows below came in as review probes (branches
    // `review/pick-r1`, `review/pick-r2`); each now asserts the fixed
    // behaviour.

    /// A ray through `target` at parameter `reach`, so the expected
    /// hit is `t = reach` at `target`.
    fn ray_through(target: Point3<f64>, dir: Vec3<f64>, reach: f64) -> Ray {
        Ray {
            origin: target - dir * reach,
            dir,
        }
    }

    /// `|det| / (|e1|·|e2|·|d|)` — up to a constant the sine of the
    /// angle between the ray and the plane: the conditioning of the
    /// exact test on this pair.
    fn conditioning(ray: &Ray, tri: &[Point3<f64>; 3]) -> f64 {
        let e1: Vec3<f64> = tri[1] - tri[0];
        let e2: Vec3<f64> = tri[2] - tri[0];
        let det = e1.dot(ray.dir.cross(e2));
        det.abs() / (e1.norm() * e2.norm() * ray.dir.norm())
    }

    /// The hit a well-conditioned interior crossing owes: accepted,
    /// at `reach` to the test's own accuracy (`~u / conditioning`
    /// relative), which for conditioning ≥ 1e-7 is under 1e-8 —
    /// asserted at 1e-6 so the row reads the class, not the ULP.
    fn assert_interior_hit(ray: &Ray, tri: &[Point3<f64>; 3], reach: f64, what: &str) {
        let t = ray_triangle(ray, tri).unwrap_or_else(|| {
            panic!(
                "{what}: a well-conditioned interior hit refused; {ray:?} {tri:?}; {}",
                fuzz::replay()
            )
        });
        assert!(
            ((t - reach) / reach).abs() < 1e-6,
            "{what}: t = {t} for a hit at {reach}; {ray:?} {tri:?}; {}",
            fuzz::replay()
        );
    }

    /// The doors the rows below separate: the exact test's acceptance
    /// and the four wrong ones a fixture here catches. **Each is the
    /// REAL [`admits`] over a mis-read bound**, never a second
    /// spelling of the predicate — a door that ignores the interval
    /// reads every bound as `0`, a door that mis-derives it reads
    /// `err/2` or `2·err`, a door that applies INFORM to `u` and `v`
    /// but not to their sum reads the sum's as `0`. So the mutants
    /// cannot drift from the acceptance they mutate: the tie is by
    /// construction, not by an assertion.
    ///
    /// The fifth wrong door, MEET — accept when the interval REACHES
    /// `[0, 1]` rather than when the value is in it — is not a bound
    /// mis-read and has no arm here. It is killed by
    /// [`the_closed_boundaries_are_pinned_one_ulp_each_way`]'s misses:
    /// `u` one ULP above `1` carries a bound of eight ULP, so MEET
    /// admits it and the door does not.
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    enum Door {
        Exact,
        /// Take any value inside the range, however wide its interval.
        NoInform,
        /// The bound derived at half its size.
        HalfBound,
        /// The bound derived at twice its size.
        DoubleBound,
        /// INFORM on `u` and `v`, and not on their sum.
        NoSumInform,
    }

    impl Door {
        /// What this door reads the `i`th derived bound as.
        fn bound(self, i: usize, err: f64) -> f64 {
            match self {
                Door::Exact => err,
                Door::NoInform => 0.0,
                Door::HalfBound => err * 0.5,
                Door::DoubleBound => err * 2.0,
                Door::NoSumInform => {
                    if i == 2 {
                        0.0
                    } else {
                        err
                    }
                }
            }
        }
    }

    /// Whether `door` admits the crossing's three barycentrics.
    fn admits_under(door: Door, ray: &Ray, tri: &[Point3<f64>; 3]) -> bool {
        let Some(c) = crossing(ray, tri) else {
            return false;
        };
        c.barycentrics
            .into_iter()
            .enumerate()
            .all(|(i, (x, err))| super::admits(x, door.bound(i, err)))
    }

    /// **The closed boundaries, pinned one ULP each way.** An
    /// axis-aligned fixture whose Möller–Trumbore arithmetic is exact:
    /// `a = (1, 1, 1)`, `e1 = (4, 0, 0)`, `e2 = (0, 4, 0)`, `d = (0, 0,
    /// −1)`, so `p = (4, 0, 0)`, `det = 16`, and with `s = origin − a`
    /// the test computes `u = s_x / 4`, `v = s_y / 4`, `t = s_z`
    /// without a rounding. Every acceptance bound is then a hit AT the
    /// bound and a miss one ULP past it — the pin a mutant that opens
    /// `u ≥ 0`, `v ≥ 0`, `u + v ≤ 1` or `t ≥ 0` cannot survive. Nothing
    /// on an exact fixture is uninformative (the widest bound here is
    /// eight ULP of `1`, at `u = 1`), so INFORM admits every case in
    /// this row and the fixtures that catch it are the two below.
    ///
    /// A static witness (memories/test-suite-cost: shape 2), not a
    /// search.
    #[test]
    fn the_closed_boundaries_are_pinned_one_ulp_each_way() {
        let tri = [
            Point3::new(1.0, 1.0, 1.0),
            Point3::new(5.0, 1.0, 1.0),
            Point3::new(1.0, 5.0, 1.0),
        ];
        let dir = Vec3::new(0.0, 0.0, -1.0);
        let at = |u: f64, v: f64, t: f64| Ray {
            origin: Point3::new(1.0 + 4.0 * u, 1.0 + 4.0 * v, 1.0 + t),
            dir,
        };
        let hits = [
            ("u = 0", at(0.0, 0.5, 2.0)),
            ("u = 1", at(1.0, 0.0, 2.0)),
            ("v = 0", at(0.5, 0.0, 2.0)),
            ("u + v = 1", at(0.5, 0.5, 2.0)),
            ("t = 0", at(0.25, 0.25, 0.0)),
            ("the interior", at(0.25, 0.25, 2.0)),
        ];
        for (name, ray) in hits {
            let t = ray_triangle(&ray, &tri).unwrap_or_else(|| panic!("{name} is a hit"));
            assert_eq!(
                t.to_bits(),
                (ray.origin.z - 1.0).to_bits(),
                "{name}: t is exact"
            );
            assert!(
                admits_under(Door::Exact, &ray, &tri),
                "{name}: the row's spelling of the acceptance is the door's"
            );
        }
        // One ULP past each bound, spelled on the ORIGIN so the
        // subtraction `s = origin − a` stays exact (Sterbenz): an
        // origin coordinate one ULP off `1` or `5` puts `u`, `v` or
        // `t` one ULP (two for `u + v`, where a half-ULP would round
        // back to `1`) past its bound.
        let from = |x: f64, y: f64, z: f64| Ray {
            origin: Point3::new(x, y, z),
            dir,
        };
        let misses = [
            ("u one ULP below 0", from(1.0f64.next_down(), 3.0, 3.0)),
            ("u one ULP above 1", from(5.0f64.next_up(), 1.0, 3.0)),
            ("v one ULP below 0", from(3.0, 1.0f64.next_down(), 3.0)),
            (
                "u + v two ULP above 1",
                from(3.0, 3.0f64.next_up().next_up(), 3.0),
            ),
            ("t one ULP below 0", from(2.0, 2.0, 1.0f64.next_down())),
        ];
        for (name, ray) in misses {
            assert_eq!(ray_triangle(&ray, &tri), None, "{name} is a miss");
        }
        // `t`'s bound is the one the barycentrics do not carry: its
        // pin is a miss the acceptance above admits.
        assert!(
            admits_under(Door::Exact, &from(2.0, 2.0, 1.0f64.next_down()), &tri),
            "t one ULP below 0: the barycentrics themselves are admitted"
        );
        let in_plane = Ray {
            origin: Point3::new(0.0, 2.0, 1.0),
            dir: Vec3::new(1.0, 0.0, 0.0),
        };
        assert_eq!(
            crossing(&in_plane, &tri),
            None,
            "a ray in the plane has no certified determinant, so no crossing"
        );
        assert_eq!(ray_triangle(&in_plane, &tri), None);
    }

    /// A near-tangent crossing whose determinant is certified at
    /// `k / 6` of its own bound, over a triangle of area `0.5` that
    /// is not degenerate: `ζ = 2⁻²⁰` and `ξ = k` ULP of it,
    /// `e1 = (1, 0, ζ + ξ)`, `e2 = (0, 1, 0)`, `d = (1, 1, ζ)`, so
    /// `p = (−ζ, 0, 1)` and `det = ξ` exactly. The origin is placed a
    /// unit away and one unit off-axis, which makes `u = v = 0.5` and
    /// `u + v = 1` exactly at every `k` — all three INSIDE the closed
    /// range, so what happens to the candidate is INFORM's doing
    /// alone. The bounds are `9/(k − 6)`, `15/(k − 6)` and their sum,
    /// so `k` is the dial that moves the intervals without moving the
    /// values.
    fn near_tangent(k: f64) -> (Ray, [Point3<f64>; 3]) {
        let zeta = 2f64.powi(-20);
        let xi = k * zeta * f64::EPSILON;
        let tri = [
            Point3::new(0.0, 0.0, 0.0),
            Point3::new(1.0, 0.0, zeta + xi),
            Point3::new(0.0, 1.0, 0.0),
        ];
        let ray = Ray {
            origin: Point3::new(-1.0, -1.0, 0.5 * xi - zeta),
            dir: Vec3::new(1.0, 1.0, zeta),
        };
        (ray, tri)
    }

    /// **A candidate whose barycentrics carry no information is
    /// refused, though every one of them is inside the closed range.**
    /// At `k = 8` the [`near_tangent`] fixture computes
    /// `u = v = 0.5` with intervals `±4.5` and `±7.5` and
    /// `u + v = 1 ± 12`: each is consistent with every point of the
    /// triangle AND with every point outside it, and the ray misses
    /// the triangle's plane by `2e-21` while its origin sits a unit
    /// away, so the numbers are the quotient of that cancellation and
    /// nothing else. The closed comparison admits all three, so
    /// **this is the pin a door that dropped INFORM cannot survive**:
    /// that door takes the candidate and answers a `t` near `1.5`
    /// from numbers that say nothing.
    #[test]
    fn a_candidate_whose_barycentrics_carry_no_information_is_refused() {
        let (ray, tri) = near_tangent(8.0);
        let xi = 8.0 * 2f64.powi(-20) * f64::EPSILON;
        let c = crossing(&ray, &tri).expect("a certified determinant");
        assert_eq!(
            c.det, xi,
            "the determinant is certified, and is the fixture's ξ exactly"
        );
        let [(u, err_u), (v, err_v), (sum, err_sum)] = c.barycentrics;
        assert_eq!((u, v, sum), (0.5, 0.5, 1.0), "the fixture's barycentrics");
        for (what, x, err) in [("u", u, err_u), ("v", v, err_v), ("u + v", sum, err_sum)] {
            assert!(
                (0.0..=1.0).contains(&x),
                "{what} = {x} is inside the closed range, so the closed comparison admits it"
            );
            assert!(
                x - err <= 0.0 && x + err >= 1.0,
                "{what}: the interval {x} ± {err} covers the whole admissible range"
            );
        }
        assert_eq!(ray_triangle(&ray, &tri), None, "the candidate is refused");
        assert!(
            !admits_under(Door::Exact, &ray, &tri),
            "the row's spelling of the acceptance is the door's"
        );
        assert!(
            admits_under(Door::NoInform, &ray, &tri),
            "a door that dropped INFORM admits it — so INFORM is what refuses it"
        );
    }

    /// **The bound INFORM reads is the derived one, not half of it.**
    /// At `k = 32` the [`near_tangent`] fixture's `v` is `0.5` with an
    /// interval of `±15/26`, which covers `[0, 1]` by `0.077` at each
    /// end — so the candidate is refused — while half that interval,
    /// `±0.288`, covers nothing and the same candidate is taken. A
    /// bound quietly tightened is a door that answers from a number it
    /// cannot vouch for, and this is the pin that catches it. The
    /// other two barycentrics are informative at the full bound
    /// already (`u` by `0.154`, `u + v` by `0.077`), which is why the
    /// row names `v`.
    #[test]
    fn halving_the_derived_bound_admits_a_candidate_the_door_refuses() {
        let (ray, tri) = near_tangent(32.0);
        let [_, (v, err_v), _] = crossing(&ray, &tri)
            .expect("a certified determinant")
            .barycentrics;
        assert_eq!(v, 0.5, "the fixture's v");
        assert!(
            v - err_v <= 0.0 && v + err_v >= 1.0,
            "v's interval {v} ± {err_v} covers the admissible range"
        );
        assert!(
            v - 0.5 * err_v > 0.0,
            "half of it, {} ± {}, does not",
            v,
            0.5 * err_v
        );
        assert_eq!(ray_triangle(&ray, &tri), None, "the candidate is refused");
        assert!(
            !admits_under(Door::Exact, &ray, &tri),
            "the row's spelling of the acceptance is the door's"
        );
        assert!(
            admits_under(Door::HalfBound, &ray, &tri),
            "a door reading half the derived bound admits it"
        );
    }

    /// **The bound INFORM reads is the derived one, not twice it.**
    /// The bound is pinned from the loose side by the row above; this
    /// is the tight side. At `k = 64` the [`near_tangent`] fixture's
    /// intervals are `u = 0.5 ± 9/58`, `v = 0.5 ± 15/58` and
    /// `u + v = 1 ± 24/58`: every one informs, so the door takes the
    /// candidate — while at TWICE the derived bound `v`'s interval
    /// reaches `0.5 ± 15/29`, which covers `[0, 1]`, and a door
    /// reading the bound that loosely refuses a crossing the
    /// arithmetic can vouch for. A bound quietly inflated throws away
    /// answers, which is the same defect as one quietly tightened and
    /// was invisible to the rows until this one.
    #[test]
    fn doubling_the_derived_bound_refuses_a_candidate_the_door_admits() {
        let (ray, tri) = near_tangent(64.0);
        let [(u, err_u), (v, err_v), (sum, err_sum)] = crossing(&ray, &tri)
            .expect("a certified determinant")
            .barycentrics;
        assert_eq!((u, v, sum), (0.5, 0.5, 1.0), "the fixture's barycentrics");
        for (what, x, err) in [("u", u, err_u), ("v", v, err_v), ("u + v", sum, err_sum)] {
            assert!(
                x - err > 0.0 || x + err < 1.0,
                "{what}: the interval {x} ± {err} informs at the derived bound"
            );
        }
        assert!(
            v - 2.0 * err_v <= 0.0 && v + 2.0 * err_v >= 1.0,
            "at twice the bound, v's interval {v} ± {} covers the range",
            2.0 * err_v
        );
        assert_eq!(
            ray_triangle(&ray, &tri),
            Some(1.5),
            "the candidate is admitted, at the hit point's own parameter"
        );
        assert!(
            admits_under(Door::Exact, &ray, &tri),
            "the row's spelling of the acceptance is the door's"
        );
        assert!(
            !admits_under(Door::DoubleBound, &ray, &tri),
            "a door reading twice the derived bound refuses it"
        );
    }

    /// **INFORM on `u + v` is not implied by INFORM on `u` and `v`.**
    /// Adopted from review lane pick2-r2. The [`near_tangent`] shape
    /// at `k = 18` with the origin moved so the exact hit is near
    /// `(0.25, 0.125)`: `u` and `v` each inform — their intervals
    /// reach below `1` — while the sum is inside `[0, 1]` and its
    /// interval, which carries BOTH their bounds, covers it. So the
    /// sum is the deciding arm here, which `near_tangent`'s own
    /// `u = v = 0.5` fixtures cannot show: there the sum is the widest
    /// interval but never the only covering one. A door that applied
    /// INFORM to the two barycentrics and not to their sum takes this
    /// candidate.
    #[test]
    fn inform_on_the_sum_alone_refuses_a_candidate_whose_parts_inform() {
        let zeta = 2f64.powi(-20);
        let xi = 18.0 * zeta * f64::EPSILON;
        let tri = [
            Point3::new(0.0, 0.0, 0.0),
            Point3::new(1.0, 0.0, zeta + xi),
            Point3::new(0.0, 1.0, 0.0),
        ];
        let dir = Vec3::new(1.0, 1.0, zeta);
        let e1: Vec3<f64> = tri[1] - tri[0];
        let e2: Vec3<f64> = tri[2] - tri[0];
        let target = tri[0] + e1 * 0.25 + e2 * 0.125;
        let ray = Ray {
            origin: target - dir,
            dir,
        };
        let [(u, err_u), (v, err_v), (sum, err_sum)] = crossing(&ray, &tri)
            .expect("a certified determinant")
            .barycentrics;
        assert!(
            super::admits(u, err_u),
            "u = {u} ± {err_u} is admitted on its own"
        );
        assert!(
            super::admits(v, err_v),
            "v = {v} ± {err_v} is admitted on its own"
        );
        assert!(
            (0.0..=1.0).contains(&sum),
            "u + v = {sum} is inside the closed range"
        );
        assert!(
            sum - err_sum <= 0.0 && sum + err_sum >= 1.0,
            "u + v = {sum} ± {err_sum} covers the admissible range"
        );
        assert_eq!(
            ray_triangle(&ray, &tri),
            None,
            "the door refuses on the sum's interval alone"
        );
        assert!(
            admits_under(Door::NoSumInform, &ray, &tri),
            "a door that applied INFORM to u and v but not to their sum admits it"
        );
    }

    /// **The acceptance's bits at the ends.** Adopted from review lane
    /// pick2-r2. `admits` is read by the door on every candidate, so
    /// its edges are worth a static row of their own: a bound of
    /// exactly `1` covers `[0, 1]` from anywhere inside it and
    /// refuses; one ULP below `1` does not cover from either endpoint
    /// and admits; and a NaN or an infinite bound refuses, because
    /// both halves are affirmative comparisons.
    #[test]
    fn the_acceptance_at_its_ends() {
        assert!(!super::admits(0.0, 1.0));
        assert!(!super::admits(1.0, 1.0));
        assert!(!super::admits(0.5, 1.0));
        assert!(super::admits(0.0, 1.0f64.next_down()));
        assert!(super::admits(1.0, 1.0f64.next_down()));
        assert!(!super::admits(0.5, f64::NAN));
        assert!(!super::admits(f64::NAN, 0.0));
        assert!(!super::admits(0.5, f64::INFINITY));
    }

    /// **A corner graze's verdict depends on which corner the
    /// tessellator labelled `tri[0]`.** Adopted from review lane
    /// pick2-r2, and the measurement row for
    /// `work/edit/pick-a-corner-graze-verdict-depends-on-the-corner-labelling`.
    ///
    /// INFORM refuses a candidate whose interval covers the range, and
    /// the interval's width is `triple_bound(s, d, e2)`-driven — a
    /// function of `|s|`, `|d|` and the edges, NOT of the numerator.
    /// So it does not shrink to nothing where the barycentric does,
    /// and whether a genuine graze survives depends on which of `s`'s
    /// components the labelling happens to zero. On the
    /// [`near_tangent`] triangle at `k = 32` a ray through corner `b`
    /// is admitted labelled `(a, b, c)` and refused labelled
    /// `(b, c, a)`: one geometry, one determinant to the bit, two
    /// verdicts. At `k = 8` no labelling admits a corner graze at all.
    ///
    /// This row asserts the asymmetry rather than a fix, because the
    /// fix is a ruling (the filed row above). The runtime value that
    /// reds it is the asymmetry going away — which is the outcome the
    /// row wants.
    #[test]
    fn a_corner_graze_is_admitted_or_refused_by_its_label() {
        let zeta = 2f64.powi(-20);
        let xi = 32.0 * zeta * f64::EPSILON;
        let a = Point3::new(0.0, 0.0, 0.0);
        let b = Point3::new(1.0, 0.0, zeta + xi);
        let c = Point3::new(0.0, 1.0, 0.0);
        let dir = Vec3::new(1.0, 1.0, zeta);
        let ray = Ray {
            origin: b - dir,
            dir,
        };
        let abc = [a, b, c];
        let bca = [b, c, a];
        let det_abc = crossing(&ray, &abc).expect("certified").det;
        let det_bca = crossing(&ray, &bca).expect("certified").det;
        assert_eq!(
            det_abc.to_bits(),
            det_bca.to_bits(),
            "one determinant, two labellings"
        );
        let [(u, _), (v, _), _] = crossing(&ray, &abc).expect("certified").barycentrics;
        assert_eq!(
            (u, v),
            (1.0, 0.0),
            "the graze is corner b: u = 1, v = 0 exactly"
        );
        let [(u2, _), (v2, _), _] = crossing(&ray, &bca).expect("certified").barycentrics;
        assert_eq!(
            (u2, v2),
            (0.0, 0.0),
            "relabelled, the graze is corner a: u = v = 0 exactly"
        );
        assert_eq!(
            ray_triangle(&ray, &abc),
            Some(1.0),
            "labelled (a, b, c), the graze is admitted at t = 1"
        );
        assert_eq!(
            ray_triangle(&ray, &bca),
            None,
            "labelled (b, c, a), the same graze is refused at INFORM"
        );
        // And at k = 8 no labelling admits a graze at any corner.
        let xi = 8.0 * zeta * f64::EPSILON;
        let b8 = Point3::new(1.0, 0.0, zeta + xi);
        for (label, tri, corner) in [
            ("a", [a, b8, c], a),
            ("b", [a, b8, c], b8),
            ("c", [a, b8, c], c),
        ] {
            let ray = Ray {
                origin: corner - dir,
                dir,
            };
            assert!(crossing(&ray, &tri).is_some());
            assert_eq!(
                ray_triangle(&ray, &tri),
                None,
                "k = 8: the graze at corner {label} is refused"
            );
        }
    }

    /// **A well-conditioned interior hit is accepted, general
    /// position.** Random triangles; a ray aimed at an interior
    /// point (`u, v ∈ [0.15, 0.6]`, `u + v ≤ 0.85`) with an
    /// out-of-plane component from `1e-6` to `1e-2` of the normal, so
    /// the conditioning spans the near-tangent crossings a pick makes
    /// looking along a face. Every draw with conditioning ≥ 1e-7 is a
    /// hit at `reach`. Shape: counterexample search (varying seed,
    /// effort dial); the static witness is the fixture row above.
    #[test]
    fn a_well_conditioned_interior_hit_is_accepted() {
        let mut rng = fuzz::start("pick: interior hits in general position");
        for _ in 0..fuzz::scaled(20_000) {
            let pt = |r: &mut fuzz::Rng| {
                Point3::new(r.range(-1.0, 1.0), r.range(-1.0, 1.0), r.range(-1.0, 1.0))
            };
            let tri = [pt(&mut rng), pt(&mut rng), pt(&mut rng)];
            let (bu, bv) = (rng.range(0.15, 0.6), rng.range(0.15, 0.6));
            if bu + bv > 0.85 {
                continue;
            }
            let (e1, e2) = (tri[1] - tri[0], tri[2] - tri[0]);
            let target = tri[0] + e1 * bu + e2 * bv;
            let inplane = e1 * rng.range(-1.0, 1.0) + e2 * rng.range(-1.0, 1.0);
            let dir: Vec3<f64> = inplane + e1.cross(e2) * 10f64.powf(rng.range(-6.0, -2.0));
            let reach = rng.range(0.5, 4.0);
            let ray = ray_through(target, dir, reach);
            if conditioning(&ray, &tri) < 1e-7 {
                continue;
            }
            assert_interior_hit(&ray, &tri, reach, "general position");
        }
    }

    /// **A well-conditioned interior hit is accepted on an axis-planar
    /// triangle** — a flat cap, an extruded planar face, any
    /// sketch-plane geometry — whose own box has zero (or 1e-14 to
    /// 1e-6) extent along one axis. This is the geometry on which a
    /// guard comparing the rounded `t` against the box's entry refused
    /// 7% of interior hits; the certified determinant reads no box.
    #[test]
    fn a_flat_triangles_interior_hits_are_accepted() {
        let mut rng = fuzz::start("pick: interior hits on axis-planar triangles");
        for _ in 0..fuzz::scaled(20_000) {
            let zc = rng.range(-1.0, 1.0);
            let thick = if rng.below(2) == 0 {
                0.0
            } else {
                10f64.powf(rng.range(-14.0, -6.0))
            };
            let a = Point3::new(rng.range(-1.0, 1.0), rng.range(-1.0, 1.0), zc);
            let zb = zc + thick * rng.unit();
            let b = Point3::new(rng.range(-1.0, 1.0), rng.range(-1.0, 1.0), zb);
            let zcc = zc + thick * rng.unit();
            let c = Point3::new(rng.range(-1.0, 1.0), rng.range(-1.0, 1.0), zcc);
            let tri = [a, b, c];
            let (bu, bv) = (rng.range(0.15, 0.6), rng.range(0.15, 0.6));
            if bu + bv > 0.85 {
                continue;
            }
            let (e1, e2) = (b - a, c - a);
            let target = a + e1 * bu + e2 * bv;
            let inplane = e1 * rng.range(-1.0, 1.0) + e2 * rng.range(-1.0, 1.0);
            let dir: Vec3<f64> = inplane + e1.cross(e2) * 10f64.powf(rng.range(-8.0, -2.0));
            let reach = rng.range(0.5, 4.0);
            let ray = ray_through(target, dir, reach);
            if conditioning(&ray, &tri) < 1e-7 {
                continue;
            }
            assert_interior_hit(&ray, &tri, reach, "axis-planar");
        }
    }

    /// **A fan-triangulated cap accepts every interior hit**: the
    /// shape a tessellator emits for a round planar face — 64 thin
    /// axis-planar wedges at a non-dyadic height — picked nearly
    /// edge-on. A point in a wedge's OPEN interior is shared by no
    /// sibling, so a refusal there is a lost pick, not a moved one.
    #[test]
    fn a_fan_triangulated_cap_accepts_its_interior_hits() {
        let z = 0.5772156649015329;
        let n = 64usize;
        let centre = Point3::new(0.0, 0.0, z);
        let rim: Vec<Point3<f64>> = (0..n)
            .map(|i| {
                let a = (i as f64) * std::f64::consts::TAU / (n as f64);
                Point3::new(a.cos(), a.sin(), z)
            })
            .collect();
        let tris: Vec<[Point3<f64>; 3]> =
            (0..n).map(|i| [centre, rim[i], rim[(i + 1) % n]]).collect();
        let mut rng = fuzz::start("pick: interior hits on a fan-triangulated cap");
        for _ in 0..fuzz::scaled(20_000) {
            let tri = tris[rng.below(n)];
            let (bu, bv) = (rng.range(0.15, 0.6), rng.range(0.15, 0.6));
            if bu + bv > 0.85 {
                continue;
            }
            let target = tri[0] + (tri[1] - tri[0]) * bu + (tri[2] - tri[0]) * bv;
            let slope = 10f64.powf(rng.range(-6.0, -1.5));
            let theta = rng.range(0.0, std::f64::consts::TAU);
            let dir = Vec3::new(theta.cos(), theta.sin(), -slope);
            let reach = rng.range(0.5, 3.0);
            let ray = ray_through(target, dir, reach);
            if conditioning(&ray, &tri) < 1e-7 {
                continue;
            }
            assert_interior_hit(&ray, &tri, reach, "fan cap");
        }
    }

    /// **The prism's cap answers an edge-on pick**, through the
    /// service's own candidate loop over [`MeshPick`]: the unit prism
    /// tessellated at δ = 0.01 and moved to a non-dyadic place, a
    /// target in the interior of its top cap, a ray descending onto it
    /// at a slope from 1e-5 to 1e-1.5 — above the cap until it lands,
    /// so the cap is the first and only face on the way. The nearest
    /// accepted candidate is the cap at `reach`.
    #[test]
    fn the_prism_cap_answers_an_edge_on_pick() {
        let body = unit_prism();
        let mut pmemo = mesh::PatchMemo::new();
        let mut mesh = tessellate_with(&body, 0.01, Tol::witness(), &mut pmemo)
            .expect("the prism tessellates")
            .mesh;
        let off = Vec3::new(0.3141592653589793, 0.2718281828459045, 0.5772156649015329);
        for p in &mut mesh.positions {
            *p = *p + off;
        }
        let index = MeshPick::build(&mesh).expect("the prism indexes");
        let mut rng = fuzz::start("pick: the prism cap edge-on");
        for _ in 0..fuzz::scaled(5_000) {
            let target = Point3::new(
                rng.range(0.15, 0.85) + off.x,
                rng.range(0.15, 0.85) + off.y,
                1.0 + off.z,
            );
            let slope = 10f64.powf(rng.range(-5.0, -1.5));
            let theta = rng.range(0.0, std::f64::consts::TAU);
            let dir = Vec3::new(theta.cos(), theta.sin(), -slope);
            let reach = rng.range(0.5, 3.0);
            let ray = ray_through(target, dir, reach);
            let mut best: Option<f64> = None;
            for cand in index.candidates(&ray) {
                let (tri, _) = index.triangle(&cand).expect("a built candidate");
                if let Some(t) = ray_triangle(&ray, &tri.corners)
                    && best.is_none_or(|b| t < b)
                {
                    best = Some(t);
                }
            }
            let t = best.unwrap_or_else(|| {
                panic!(
                    "the cap is missed at {target:?} along {dir:?}; {}",
                    fuzz::replay()
                )
            });
            assert!(
                ((t - reach) / reach).abs() < 1e-6,
                "the nearest candidate is not the cap at {reach}: t = {t}; {}",
                fuzz::replay()
            );
        }
    }
}
