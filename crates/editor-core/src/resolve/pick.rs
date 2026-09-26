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
//! the certified `t` INTERVAL each admitted candidate carries
//! ([`TSpan`]), ordered by that interval alone — candidates the
//! geometry cannot order are a certified tie, answered as one face
//! where they name one and REFUSED with all of them where they name
//! several ([`HitTestError::Ambiguous`])
//! → the answered patch's [`mesh::FacePatch::face`] back-reference →
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
//! order, an answer that is a function of the candidate SET, no
//! hashing — so the same pick against the same state answers
//! bit-identically.
//!
//! # Where the acceleration state lives, and when it dies
//!
//! [`MeshPick`] is per-mesh state, built once per tessellated mesh and
//! held inside the [`NodePick`] that built it: a consumer reaches an
//! index only by holding that `NodePick`, because a `MeshPick` is
//! minted only inside [`NodePick::build`] — or by the memoised door
//! [`MeshPick::build_with`], which serves each patch's table whole
//! from [`PickMemo`] where the tessellation reused that patch and
//! builds it where it did not — self-contained (it copies the
//! triangle geometry out of the mesh), and valid exactly as long
//! as the mesh it was built from is the one being displayed. A static
//! scene therefore never rebuilds per query. The obvious invalidator
//! is the evaluation epoch: a new [`Evaluation`] means new meshes,
//! so a consumer keys its `NodePick` cache by
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
//! the pick index are the same tessellation.
//!
//! **It is the only door a consumer has.** Raw assembly — a
//! [`MeshPick`] of one's own, declared to be of a document, a node and
//! a body — is the move that makes both halves a claim, so both of its
//! constructors (`MeshPick::build` and `PickTarget::new`) sit behind
//! this crate's `test-support` cargo feature and exist in no build that
//! does not ask for them. What the rows behind that feature measure is
//! written at [`PickTarget`]; what a consumer can hold is a target
//! whose document half [`pick_face`] checks and whose node, body and
//! mesh came from one tessellation.

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
    /// triangle order: the refusal list's coordinate.
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

/// Typed failure of the index build ([`MeshPick::build_every_table`]),
/// reached by a consumer as [`NodePickError::Index`] (closed; no
/// silent lanes).
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
/// refusal's list below is stated in ([`PickPatch::base`] plus the
/// position within the patch).
///
/// # Why the two-level query answers exactly as one tree over every
/// triangle would
///
/// A pick is a function of the candidate SET: each candidate's exact
/// test ([`ray_triangle`]) reads the ray and the triangle and nothing
/// else, and the loop's early-out on the box entry prunes only
/// candidates the certified order already drops — exactly, on a
/// derived margin, not up to a rounding ([`pick_face`],
/// [`early_out_margin`]). What the order otherwise decides is the WORK —
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
/// single-level reference that tests every candidate, the tied rows
/// included.
///
/// # The two doors, and why they answer the same index
///
/// [`MeshPick::build_every_table`] builds every patch's [`PickTable`];
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
    /// Flat position (patch-major) — the refusal list's coordinate.
    flat: usize,
    /// Position in [`MeshPick::patches`].
    patch: usize,
    /// Position in that patch's `tris`.
    tri: usize,
}

impl MeshPick {
    /// The raw index door — **test support**: it exists only in a build
    /// that asks for this crate's `test-support` feature. The body is
    /// [`MeshPick::build_every_table`]'s, which is where what it builds
    /// and what it refuses are written; this door only makes it
    /// nameable from outside the crate.
    ///
    /// A `MeshPick` is the pairing's other half: the index a
    /// [`PickTarget`] is built over. A consumer reaches one through
    /// [`NodePick`], which tessellates and indexes in one call so that
    /// the index and the `(document, node, body)` it is offered under
    /// come from one evaluation. Building one BY HAND is the move that
    /// makes a target's declaration a claim, so it is a fixture door,
    /// and [`MeshPick::build_every_table`] — the same body,
    /// crate-private — is what [`NodePick::build`] calls.
    ///
    /// **Not `#[doc(hidden)]`, and that has a cost.**
    /// `scripts/doc-gate.sh` renders this crate at `--all-features`, so
    /// in the rendered docs this mint sits among the public API with
    /// nothing marking it but the paragraphs above. The trade is
    /// deliberate: a hidden door is one a reader cannot find, and what
    /// keeps a consumer out is the feature gate rather than the docs —
    /// nothing written against this name compiles unless the build asks
    /// for the feature.
    ///
    /// # Errors
    ///
    /// As [`MeshPick::build_every_table`].
    #[cfg(any(test, feature = "test-support"))]
    pub fn build(mesh: &Mesh) -> Result<Self, MeshPickError> {
        Self::build_every_table(mesh)
    }

    /// Builds the index from a tessellated mesh: a [`PickTable`] per
    /// patch — corners, per-triangle boxes, tree, hull — and the
    /// top-level tree over the patches' hulls. Every table is built
    /// here; the memoised form is [`MeshPick::build_with`].
    ///
    /// Crate-private in every build: [`NodePick::build`] calls it
    /// after tessellating, which is how a consumer reaches an index
    /// without ever holding one. `MeshPick::build` is the same body
    /// under the `test-support` feature.
    ///
    /// # Errors
    ///
    /// [`MeshPickError::PositionOutOfRange`] when a triangle indexes
    /// outside [`Mesh::positions`] — corrupt input, never skipped.
    pub(crate) fn build_every_table(mesh: &Mesh) -> Result<Self, MeshPickError> {
        Self::assemble(mesh, |pi, patch| {
            Ok(Arc::new(PickTable::build(mesh, pi, patch)?))
        })
    }

    /// [`MeshPick::build_every_table`] over `memo`'s per-patch tables: the patch at
    /// position `i` is served the table the memo holds under the memo
    /// entry `keys` reports for it, and builds (and stores) one
    /// otherwise. Nothing per triangle runs on a served patch — no
    /// corner copy, no box, no tree — because the entry identity says
    /// the placed corners are the stored ones' bit for bit
    /// ([`mesh::StoredPatchId`]). The top-level
    /// tree is built every time. The index is the same, table for
    /// table and tree for tree, as [`MeshPick::build_every_table`]'s
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
    /// As [`MeshPick::build_every_table`].
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

/// One displayed mesh offered to a pick: which document and which
/// node/body the mesh renders, and its prebuilt index.
///
/// # Every half a consumer can reach is paired by construction (DI3, A2a)
///
/// [`NodePick::target`] is the only mint a consumer can reach, and it
/// takes nothing: the document, the node, the body and the mesh index
/// all come from the one tessellation [`NodePick::build`] performed, so
/// there is nothing for a caller to declare and nothing to declare
/// wrongly. The stamp matters because node ids are minted per document:
/// a twin recipe's evaluation satisfies every standing check and
/// answers every name lookup, about other geometry. So [`pick_face`]
/// refuses an evaluation of any OTHER document before it reads a
/// triangle — the one predicate every pairing door shares — and that
/// refusal is a statement about the TYPE, because every target a
/// consumer can mint carries a stamp it did not choose.
///
/// The fields are private, so a minted target cannot be taken apart and
/// re-stamped, and neither [`NodePick`] nor this type hands its
/// `MeshPick` out; a forged half would need a mesh index the forger
/// built, and the door that builds one is test support (below).
///
/// # The raw mint is TEST SUPPORT
///
/// `PickTarget::new` — the caller supplies a [`MeshPick`] of its own
/// and DECLARES which document, node and body it is of — exists only
/// under this crate's `test-support` feature, on a dev-dependency edge
/// no consumer's build graph carries. On that path every half is a
/// claim: the node half cannot be checked even in principle (arena keys
/// collide numerically across sibling nodes OF ONE DOCUMENT, so
/// [`pick_face`] resolves the hit triangle's face key against the wrong
/// node's table and answers a **plausible, confidently wrong name** —
/// the failure a selection consumer cannot detect, same convention
/// family as [`super::MeshPatchKey`]), and the document half is checked
/// against the handed evaluation rather than against the mesh, so a
/// caller that declares one document's mesh to be of another is taken
/// at its word. That is the whole of issue #1098's raw-assembly class,
/// and it is now closed AT THE API rather than documented: the class
/// lives where the feature does, which is the rows that measure it
/// (`edit_pair_apply_names::a_raw_target_is_a_claim_in_every_half` for
/// the document half, the ignored witness
/// `gui1_pick_r2::a_mesh_paired_with_the_wrong_node_does_not_answer_a_name`
/// for the node half) and the rows that need a mesh no tessellation
/// produces or a node [`NodePick::build`] refuses.
///
/// Re-stamping a minted target is a compile error, which is what makes
/// the paragraphs above a statement about the type rather than about
/// its callers:
///
/// ```compile_fail,E0451
/// use editor_core::{DocumentId, PickTarget};
/// fn forge<'a>(honest: PickTarget<'a>, other: DocumentId) -> PickTarget<'a> {
///     PickTarget { document: other, ..honest }
/// }
/// ```
///
/// `Debug` dumps the whole target, the mesh index included, which is
/// how a row compares two indexes table for table without a door that
/// hands the index itself out (one that did would re-open the mint
/// above).
#[derive(Clone, Copy, Debug)]
pub struct PickTarget<'a> {
    /// The document whose evaluation produced the displayed body —
    /// the half [`pick_face`] checks against the handed evaluation.
    document: DocumentId,
    /// The node whose evaluation produced the displayed body.
    node: RecipeNodeId,
    /// The output body index within that node's value.
    body: u32,
    /// The body's mesh index ([`MeshPick::build_every_table`]).
    pick: &'a MeshPick,
}

impl<'a> PickTarget<'a> {
    /// A target assembled BY HAND from a mesh index the caller built —
    /// **test support**: it exists only in a build that asks for this
    /// crate's `test-support` feature, which is what makes the rest of
    /// this type's contract a statement about the type.
    ///
    /// **Not `#[doc(hidden)]`, and that has a cost.**
    /// `scripts/doc-gate.sh` renders this crate at `--all-features`, so
    /// in the rendered docs this mint sits among the public API with
    /// nothing marking it but this paragraph and the one above. The
    /// trade is deliberate: a hidden door is one a reader cannot find,
    /// and what keeps a consumer out is the feature gate rather than
    /// the docs — nothing written against this name compiles unless the
    /// build asks for the feature.
    ///
    /// The document half is not an argument — it is read off `eval`,
    /// the evaluation `pick`'s mesh is claimed to have been
    /// tessellated from — so not even a row can name a document it has
    /// no evaluation of. Everything else is the caller's word: which
    /// node, which body, and which mesh the index was built over. What
    /// that is worth is this type's docs.
    ///
    /// What it exists for is the rows [`NodePick`] cannot express: a
    /// mesh scaled by hand so a ray oracle runs in exact integers, a
    /// target naming a node whose value failed or is absent (which
    /// [`NodePick::build`] refuses, and so mints nothing for), and the
    /// two witnesses of the raw-assembly class itself.
    #[cfg(any(test, feature = "test-support"))]
    pub fn new<T: Decide>(
        eval: &Evaluation<T>,
        node: RecipeNodeId,
        body: u32,
        pick: &'a MeshPick,
    ) -> Self {
        Self {
            document: eval.document,
            node,
            body,
            pick,
        }
    }
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
///
/// **The building evaluation's document is stamped too**, because the
/// pairing a constructor establishes has to survive the constructor:
/// the doors that take a SECOND evaluation —
/// [`NodePick::patch_names`], [`NodePick::boundary_names`], and
/// [`pick_face`] through [`NodePick::target`] — read THAT
/// evaluation's tables, and node ids are minted per document, so an
/// evaluation of a twin recipe answers every one of those lookups out
/// of its own tables, in patch order, with no refusal. Each door runs
/// `ident::mispaired` against this stamp before reading anything of
/// the evaluation (DI3, A2a). The version half is deliberately not
/// stamped — a later evaluation of the SAME document is admitted, and
/// what may be reused across it is the content keys' business.
///
/// **What admission costs a caller that went through [`PickMemo`]:
/// nothing.** An index is served back only under the evaluation memo's
/// own reuse condition, so a later run in which this node's value
/// MOVED misses the memo and rebuilds; the admitted case is reachable
/// only by holding an index across pictures by hand. The rows are
/// `edit_pair_apply_names::a_later_evaluation_of_the_same_document_is_admitted`
/// (what the doors answer) and `::what_the_admitted_later_evaluation_answers`
/// (what it costs, and the memo's miss).
#[derive(Debug, Clone)]
pub struct NodePick {
    document: DocumentId,
    node: RecipeNodeId,
    body: u32,
    mesh: Arc<Mesh>,
    pick: Arc<MeshPick>,
}

/// One memoised (node, body): what it was built for, and the pick.
///
/// The document half of the key is not a field here: it is the stamp
/// the memoised [`NodePick`] already carries, so the two cannot drift
/// into an entry whose pick is of one document and whose key says
/// another.
struct PickEntry {
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
/// **The document comparison is this memo's DI3 refusal**, and it is
/// the only half of the key that CAN refuse a prior of another
/// document: node ids are minted per document, so two documents of one
/// recipe carry the same ids AND the same content and naming keys for
/// the same node, and every other half of the key matches. The row is
/// `edit_pair_apply_names::the_memo_refuses_a_prior_of_another_document`
/// — the seam owns one memo across builds
/// (`viewer::evalseam::build_index`), so a second document's build
/// reaching the first document's entry is a live shape and not a
/// hypothetical one.
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
    /// positions the refusal's list is stated in.
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

/// The pairing for the three doors that take a second evaluation:
/// `None` when `eval` is of `expected`, else the typed refusal, which
/// is `HitTestError`'s `From<Mispaired>` and no second spelling of
/// which field goes where.
///
/// The comparison is `ident::mispaired`, the one predicate the pairing
/// doors share (A2a) — identity only, never a version, so a LATER
/// evaluation of the same document still pairs.
fn mispairing<T: Decide>(expected: DocumentId, eval: &Evaluation<T>) -> Option<HitTestError> {
    crate::ident::mispaired(expected, eval.document).map(HitTestError::from)
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
        let pick = MeshPick::build_every_table(&mesh).map_err(NodePickError::Index)?;
        Ok(Self {
            document: eval.document,
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
            && entry.pick.document == eval.document
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
            document: eval.document,
            node,
            body,
            mesh: Arc::new(tessellation.mesh),
            pick: Arc::new(pick),
        };
        memo.nodes.insert(
            (node, body),
            PickEntry {
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

    /// The pick target this index answers for — pre-paired in both
    /// halves ([`PickTarget`]), ready for [`pick_face`], and the only
    /// mint of a target a consumer can reach (the raw one is test
    /// support).
    pub fn target(&self) -> PickTarget<'_> {
        PickTarget {
            document: self.document,
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
    ///
    /// **`eval` must be an evaluation OF the document this index was
    /// built from** (DI3, A2a). The index is built from one evaluation
    /// and handed another here, and node ids and output-body indices
    /// are minted per document, so a twin recipe's evaluation answers
    /// these lookups out of ITS tables: other geometry's names, in
    /// patch order, with no `Unnamed` and no refusal. The pairing is
    /// checked against the building evaluation's stamp before any
    /// table is read.
    ///
    /// The refusal is of the CALL and sits OUTSIDE the vector, which
    /// is the shape of the fact: a mispairing is one thing that is
    /// wrong with the arguments, not one thing wrong with each patch,
    /// and a per-slot `Err` repeating it once per patch would read as
    /// `n` unnamed faces. The per-patch lane keeps its own meaning.
    ///
    /// A LATER evaluation of the SAME document is admitted, even one
    /// that re-tessellated this node: identity is what a pairing is
    /// about (DI3), and whether the patches still line up is the
    /// content keys' business, which is what [`PickMemo`] reads them
    /// for.
    ///
    /// # Errors
    ///
    /// [`HitTestError::EvaluationOfAnotherDocument`] — the only way
    /// the call as a whole refuses.
    pub fn patch_names(
        &self,
        eval: &Evaluation<f64>,
    ) -> Result<Vec<Result<StableName, HitTestError>>, HitTestError> {
        if let Some(refusal) = mispairing(self.document, eval) {
            return Err(refusal);
        }
        Ok(self
            .mesh
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
            .collect())
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
    ///
    /// The pairing is [`NodePick::patch_names`]', for the same reason
    /// and with the same boundary: `eval` must be an evaluation of the
    /// document this index was built from, a later evaluation of that
    /// document is admitted, and the refusal is of the call rather
    /// than of each polyline.
    ///
    /// # Errors
    ///
    /// [`HitTestError::EvaluationOfAnotherDocument`] — the only way
    /// the call as a whole refuses.
    pub fn boundary_names(
        &self,
        eval: &Evaluation<f64>,
    ) -> Result<Vec<Result<StableName, HitTestError>>, HitTestError> {
        if let Some(refusal) = mispairing(self.document, eval) {
            return Err(refusal);
        }
        Ok(self
            .mesh
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
            .collect())
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

/// A face pick's answer: the stable name plus where and what was
/// hit. No arena key — the name IS the reference selection state
/// holds (G1).
///
/// One of these is the door's success; a LIST of them is
/// [`HitTestError::Ambiguous`], the certified tie between faces, where
/// each one is equally true.
#[derive(Debug, Clone)]
pub struct PickHit {
    /// The picked face's stable name.
    pub name: StableName,
    /// The node whose body was hit.
    pub node: RecipeNodeId,
    /// The output body index within that node's value.
    pub body: u32,
    /// The winning [`TSpan`], spelled out: `t` is the rounded
    /// parameter of the hit along the ray, in units of `|ray.dir|`,
    /// and `[t_lo, t_hi]` is the interval the arithmetic certifies
    /// around it — what [`TSpan`] documents, not a second answer and
    /// not a second rule. The three are fields rather than one `span`
    /// because 43 call sites read them by name.
    ///
    /// **The parameter is of the ray this call was given.** A consumer
    /// that carries a hit across a transform converts all three, or
    /// none of them mean anything: the viewer's cross-part merge is
    /// `work/vgeom/pickindex-merges-parts-on-a-rounded-t-it-never-converts.md`.
    pub t: f64,
    /// See [`PickHit::t`].
    pub t_lo: f64,
    /// See [`PickHit::t`].
    pub t_hi: f64,
    /// The hit point, `origin + t · dir`.
    pub point: Point3<f64>,
}

/// **Field-wise equality, floats included** — written out because
/// [`Point3`] has none to derive from.
///
/// It exists for [`HitTestError`], whose [`HitTestError::Ambiguous`]
/// arm carries these: two refusals are the same refusal when they
/// name the same faces at the same parameters and the same points, and
/// nothing weaker would let a row pin a refusal at all. `==` on the
/// floats, so a hit carrying a NaN parameter equals nothing, itself
/// included — unreachable here ([`ray_triangle`] admits only a finite
/// span) and the fail-loud direction if it ever were not.
impl PartialEq for PickHit {
    fn eq(&self, other: &Self) -> bool {
        // Destructured exhaustively on BOTH sides: a field added to
        // `PickHit` is E0027 here rather than a field silently outside
        // equality.
        let Self {
            name,
            node,
            body,
            t,
            t_lo,
            t_hi,
            point,
        } = self;
        let Self {
            name: other_name,
            node: other_node,
            body: other_body,
            t: other_t,
            t_lo: other_t_lo,
            t_hi: other_t_hi,
            point: other_point,
        } = other;
        let xyz = |p: &Point3<f64>| [p.x, p.y, p.z];
        name == other_name
            && node == other_node
            && body == other_body
            && t == other_t
            && t_lo == other_t_lo
            && t_hi == other_t_hi
            && xyz(point) == xyz(other_point)
    }
}

/// The face pick: the nearest ray/triangle hit across `targets`,
/// resolved to a stable name.
///
/// `Ok(Some(hit))` is the nearest hit; **`Ok(None)` is the typed
/// miss** — the ray hits no offered triangle. Errors are never
/// flattened into a miss:
///
/// `Err(`[`HitTestError::Ambiguous`]`)` is the certified tie between
/// faces, below; it is a refusal about the GEOMETRY, not about the
/// targets, and it carries every tied face's own hit.
///
/// - every target must be OF the document `eval` is of — a target
///   stamped with another document answers
///   [`HitTestError::EvaluationOfAnotherDocument`] up front (first
///   offending target in slice order), before any standing is read,
///   because node ids are minted per document and a twin's evaluation
///   would answer every standing check and then name the hit out of
///   its own tables (DI3, A2a);
/// - every target's node must have an `Ok` value in `eval` — a target
///   whose node has no result / failed / was poisoned answers the
///   corresponding [`HitTestError`] up front (first offending target
///   in slice order), because a mesh being displayed for such a node
///   cannot belong to this evaluation;
/// - the winning face inverts through [`entity_name`]; an
///   evaluated-but-unnamed face is the loud
///   [`HitTestError::Unnamed`] bug report, propagated verbatim.
///
/// **Determinism and the certified tie (documented contract)**: an
/// admitted candidate answers a `t` INTERVAL ([`TSpan`]), and one
/// candidate is in front of another only when the WHOLE of its
/// interval is ([`TSpan::precedes`]). A candidate that some other
/// candidate precedes is out; the survivors overlap one another, so
/// the geometry does not order them at all — they are a CERTIFIED
/// TIE, and **the door does not break it**. There is no second key:
/// what the survivors NAME decides.
///
/// - Survivors naming ONE face — the same `(node, body, face)` met on
///   several of its own triangles, which is every ray across a
///   triangle diagonal or an in-face shared edge — are one answer.
///   The door answers that face with the HULL of the members'
///   intervals (each encloses the crossing of its own triangle, so
///   the hull encloses every crossing the tie holds) and, for `t` and
///   `point`, the member with the smallest rounded `t`: a function of
///   the set, and a point of the face.
/// - Survivors naming MORE THAN ONE face are refused, typed
///   ([`HitTestError::Ambiguous`]) — one [`PickHit`] per tied face,
///   each of them true, listed in the caller's target order and then
///   face-arena order. That is an order for a LIST and decides
///   nothing: neither the interval's width, nor the scene's placement,
///   nor the order the targets were offered in can reach a pick's
///   answer.
///
/// Triangle boundaries are CLOSED in the exact test, so an
/// edge/vertex graze is a hit for every incident triangle — which is
/// why a ray down a cube's shared edge names neither face and refuses
/// with both, while a ray down an edge INSIDE one face answers that
/// face.
///
/// **That rule is total, and it is a rule about the SET rather than a
/// pairwise fold.** `precedes` is a strict partial order, not a total
/// one, so a pairwise fold over it would answer whichever candidate
/// the traversal met first. Taking the survivors of `precedes` as one
/// set removes that: a candidate no other precedes is exactly one with
/// `t_lo ≤ min_j t_hi(j)`, and any two such candidates overlap each
/// other. Grouping that set by face is a function of the set alone.
///
/// **The answer is that winner over every candidate's exact test.**
/// Each test reads the ray and the triangle alone, and a ray in a
/// triangle's plane refuses at the determinant ([`ray_triangle`])
/// rather than answering a noise `t`, so no candidate's answer
/// depends on which candidates were visited before it.
///
/// **The early-out prunes nothing the set rule keeps** — the
/// invariant, and the whole of what the box is allowed to decide.
/// The traversal skips a candidate, without evaluating its crossing,
/// when `lowest_hi < t_enter − margin` for the margin
/// [`early_out_margin`] derives from that candidate's own triangle
/// and the ray. Four lines:
///
/// 1. `margin` bounds `t_enter − t_lo` for EVERY admitted candidate
///    of that triangle (`early_out_margin`, term by term).
/// 2. So the skip fires only where `lowest_hi < t_lo`, which is
///    `precedes`: the candidate holding `lowest_hi` precedes this
///    one, and [`TSpan::survivors`] drops it.
/// 3. A skipped candidate could not have lowered `lowest_hi` either,
///    since `t_hi ≥ t_lo > lowest_hi`.
/// 4. So the set of survivors — and therefore the answer, and the
///    refusal's LIST — are what an exhaustive walk of every candidate
///    answers. `Pruned == Every` is now what makes a refusal
///    COMPLETE as well as what makes a hit right: a pruned candidate
///    is a tied face that would have gone unlisted.
///
/// The margin's leading term is the triangle's own extent along the
/// ray, twice, so the skip still fires for every candidate whose box
/// is more than its own diameter behind the running bound — which is
/// where the cost was. A poisoned/NaN ray is legal input: the tree
/// returns everything, every exact test misses, and the answer is
/// the typed miss.
///
/// # Errors
///
/// [`HitTestError`] as above — the targets' pairing first, then their
/// standing, then the winning face's inversion.
pub fn pick_face<T: Decide>(
    eval: &Evaluation<T>,
    targets: &[PickTarget<'_>],
    ray: &Ray,
) -> Result<Option<PickHit>, HitTestError> {
    // The pairing, before any standing is read: a foreign
    // evaluation has an `Ok` value for these node ids too (docs).
    for target in targets {
        if let Some(refusal) = mispairing(target.document, eval) {
            return Err(refusal);
        }
    }

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

    // The survivors of the certified order: a candidate no other
    // candidate precedes (docs). Each one's identity rides along, so
    // nothing is re-looked-up after the scan.
    struct Cand {
        span: TSpan,
        target_pos: usize,
        tri_pos: usize,
        node: RecipeNodeId,
        body: u32,
        face: FaceKey,
    }
    // The smallest upper end seen. A candidate whose `t_lo` exceeds it
    // is preceded by whichever candidate achieved it, and every
    // survivor has `t_lo` at or below it.
    let mut lowest_hi = f64::INFINITY;
    let mut undecided: Vec<Cand> = Vec::new();
    for (target_pos, target) in targets.iter().enumerate() {
        for cand in target.pick.candidates(ray) {
            let Some((tri, face)) = target.pick.triangle(&cand) else {
                // Unreachable: the trees were built over exactly the
                // patches' triangles.
                continue;
            };
            if lowest_hi < cand.t_enter - early_out_margin(ray, &tri.corners) {
                // This candidate's interval cannot reach back to the
                // running bound, so the candidate holding that bound
                // precedes it and the set rule drops it: skipping the
                // crossing is a cost saving that changes no answer
                // ([`early_out_margin`], where the bound is derived).
                continue;
            }
            if let Some(span) = ray_triangle(ray, &tri.corners) {
                if span.t_lo > lowest_hi {
                    // Preceded by the candidate holding `lowest_hi`.
                    continue;
                }
                if span.t_hi < lowest_hi {
                    lowest_hi = span.t_hi;
                    undecided.retain(|c| c.span.t_lo <= lowest_hi);
                }
                undecided.push(Cand {
                    span,
                    target_pos,
                    tri_pos: cand.flat,
                    node: target.node,
                    body: target.body,
                    face,
                });
            }
        }
    }

    // The survivors of the certified order ([`TSpan::survivors`]),
    // offered in `(target position, flat triangle position)` order
    // because that is the order the refusal LISTS its hits in.
    undecided.sort_unstable_by_key(|c| (c.target_pos, c.tri_pos));

    // The survivors, grouped by FACE — several triangles of one face
    // are one answer, not a tie ([`answer_of`], the one spelling of
    // that half of the rule). The candidates are offered in the order
    // the refusal LISTS its faces in, which is target order and then
    // flat triangle order, so a group's first member says where its
    // face belongs in the list.
    let candidates: Vec<(TSpan, (RecipeNodeId, u32, FaceKey))> = undecided
        .iter()
        .map(|c| (c.span, (c.node, c.body, c.face)))
        .collect();
    let mut groups = answer_of(&candidates).faces();
    if groups.is_empty() {
        return Ok(None); // the typed miss
    }
    // Target order first, then face-arena order within one target:
    // the documented order of the LIST, which decides nothing.
    groups.sort_by_key(|group| (undecided[group.first].target_pos, group.face.2));

    let hit_of =
        |group: &FaceAnswer<(RecipeNodeId, u32, FaceKey)>| -> Result<PickHit, HitTestError> {
            let (node, body, face) = group.face;
            let name = entity_name(
                eval,
                node,
                EntityRef {
                    body,
                    key: EntityKey::Face(face),
                },
            )?;
            Ok(PickHit {
                name: name.clone(),
                node,
                body,
                t: group.span.t,
                t_lo: group.span.t_lo,
                t_hi: group.span.t_hi,
                point: ray.origin + ray.dir * group.span.t,
            })
        };

    let [only] = &groups[..] else {
        // More than one face, and nothing left to order them by: the
        // typed refusal, carrying every tied face's own true hit.
        let hits: Vec<PickHit> = groups
            .iter()
            .map(hit_of)
            .collect::<Result<_, HitTestError>>()?;
        return Err(HitTestError::Ambiguous { hits });
    };
    Ok(Some(hit_of(only)?))
}

/// The exact ray/triangle test (Möller–Trumbore, both-sided, plain
/// `f64`): `Some(`[`TSpan`]`)` iff the ray meets the CLOSED triangle at `t ≥ 0`
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
/// is a hit for EVERY incident triangle (what the caller does with
/// several is [`pick_face`]'s set rule — one face is one answer,
/// several are refused; watertight meshes never lose a graze to an
/// open boundary) and the hit point `a + u·e1 + v·e2` is a point OF the
/// closed triangle — exactly, after [`retract_to_simplex`], which is
/// what lets a caller bound how far below its own box's entry a
/// candidate's interval can reach ([`early_out_margin`]).
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
pub fn ray_triangle(ray: &Ray, tri: &[Point3<f64>; 3]) -> Option<TSpan> {
    let Crossing { barycentrics, .. } = crossing(ray, tri)?;
    if !barycentrics.iter().all(|&(x, err)| admits(x, err)) {
        return None;
    }
    let [(u, err_u), (v, err_v), _] = barycentrics;
    let e1: Vec3<f64> = tri[1] - tri[0];
    let e2: Vec3<f64> = tri[2] - tri[0];
    let (u, v) = retract_to_simplex(u, v);
    // The parameter of the hit POINT `a + u·e1 + v·e2` along the ray,
    // not Möller–Trumbore's `e2·q / det`: the quotient cancels
    // catastrophically when the determinant is small (a ray grazing
    // a triangle whose plane it nearly contains), while the point's
    // projection onto the ray is conditioned by `u` and `v` alone.
    let span = t_span(ray, tri[0], e1, e2, (u, err_u), (v, err_v));
    let forward_and_finite =
        span.t >= 0.0 && span.t.is_finite() && span.t_lo.is_finite() && span.t_hi.is_finite();
    forward_and_finite.then_some(span)
}

/// **The clamp**: the per-coordinate RETRACTION onto the simplex —
/// `u` into `[0, 1]`, then `v` into `[0, 1 − u]` with the clamped `u`.
///
/// It fixes every point of the simplex, which is all [`t_span`]'s
/// width derivation needs, and it is NOT the metric projection onto
/// the closed triangle: `(u, v) = (1, 1)` retracts to the corner
/// `(1, 0)`, while the nearest point of the triangle is `(0.5, 0.5)`.
/// It is what makes the answered point a point OF the triangle
/// whatever the acceptance admitted, and it never moves an admitted
/// point farther from a true crossing that is itself on the triangle.
///
/// `1 − u` is the EXACT bound here, not `fl(1 − u)`: for `u ≥ 0.5` the
/// subtraction is exact (Sterbenz), and for `u < 0.5` it can round UP
/// by half an ulp — at `u = 2⁻⁵⁴` it rounds to `1` — and admit a `v`
/// with `u + v > 1`, a point `2⁻⁵⁴·|e2|` off the triangle, which
/// would cost the sentence above its "OF". `1 − top < u` detects
/// exactly that case: when `top ≥ 0.5` that test is itself exact by
/// Sterbenz, and when `top < 0.5` then `u > 0.5`, so `top` was exact
/// and the test correctly declines.
fn retract_to_simplex(u: f64, v: f64) -> (f64, f64) {
    let u = u.clamp(0.0, 1.0);
    let top = 1.0 - u;
    let top = if 1.0 - top < u { top.next_down() } else { top };
    (u, v.clamp(0.0, top))
}

/// One admitted hit's parameter along the ray, as an INTERVAL.
///
/// `[t_lo, t_hi]` encloses the parameter of the true crossing whenever
/// that crossing is a point of the closed triangle, taking the corners
/// and the ray as exact; `t` is the rounded value inside it and is
/// what a caller reports. Every width here is the operation count's —
/// [`crossing`]'s bounds carried through the projection, plus the
/// projection's own rounding ([`t_span`], the one site where both are
/// derived). No tolerance, no chosen factor.
///
/// Two hits are ORDERED only when one interval lies wholly below the
/// other ([`TSpan::precedes`]); intervals that overlap are a CERTIFIED
/// tie — the geometry does not order them — and [`pick_face`] does not
/// break it. There is no second key: the survivors
/// ([`TSpan::survivors`]) are the answer when they name one face and
/// the refusal when they name more.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TSpan {
    /// The rounded parameter of the hit point, in units of `|ray.dir|`.
    pub t: f64,
    /// The interval's lower end.
    pub t_lo: f64,
    /// The interval's upper end.
    pub t_hi: f64,
}

impl TSpan {
    /// `t_hi − t_lo`: how little the arithmetic certifies about where
    /// along the ray the crossing is.
    ///
    /// **A measurement, never a key.** It is what the interval IS —
    /// the enclosure, and the quantity [`precedes`](TSpan::precedes)
    /// and [`early_out_margin`] are stated in — and nothing orders a
    /// certified tie by it: the width is not a function of the
    /// candidates' shapes alone ([`t_span`]'s `from_rounding` reads
    /// the coordinates' magnitudes), so a rule that read it would let
    /// translating the document change a pick's answer.
    pub fn width(&self) -> f64 {
        self.t_hi - self.t_lo
    }

    /// **The certified order**: this hit is in front of `other` only
    /// when the whole of its interval is, which is the only case the
    /// geometry decides. Not a total order — overlap is the tie.
    pub fn precedes(&self, other: &Self) -> bool {
        self.t_hi < other.t_lo
    }

    /// **The certified order's survivors**: the positions in `spans`
    /// of the hits no other hit [`precedes`](TSpan::precedes), in
    /// slice order. THE one spelling of the rule — [`pick_face`] calls
    /// it, and so does every reference loop and probe that pins it, so
    /// no row can pass by agreeing with a second copy of the door.
    ///
    /// The rule, in two lines: a span some other span precedes is out;
    /// the survivors are exactly those with `t_lo ≤ min_j t_hi(j)`,
    /// they pairwise overlap, and so they are ONE certified tie. The
    /// set is a function of the spans alone — no width, no position —
    /// so the order the candidates were met in cannot reach the
    /// answer. An empty slice has no survivors, which is the miss.
    ///
    /// **What the caller does with a tie is the caller's.**
    /// [`pick_face`] answers the face when every survivor names one
    /// and refuses with all of them when they name several; the slice
    /// order is the order the refusal lists them in, and decides
    /// nothing else.
    pub fn survivors(spans: &[Self]) -> Vec<usize> {
        let lowest_hi = spans.iter().map(|s| s.t_hi).fold(f64::INFINITY, f64::min);
        (0..spans.len())
            .filter(|&i| spans[i].t_lo <= lowest_hi)
            .collect()
    }
}

/// **One face's share of an answer**: the face its members name, the
/// hull of their intervals carrying the smallest rounded `t` among
/// them, and where in the offered slice those readings come from.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FaceAnswer<K> {
    /// The key every member of this group carries.
    pub face: K,
    /// The HULL of the members' intervals — each encloses the
    /// crossing of its own triangle, so the hull encloses every
    /// crossing the tie holds — with `t` the smallest rounded
    /// parameter among them.
    pub span: TSpan,
    /// The position, in the offered slice, of the member `span.t` was
    /// read from: the smallest rounded `t`, the earlier position at an
    /// equality. It is the member whose POINT the caller reports.
    pub member: usize,
    /// The position, in the offered slice, where this face was first
    /// met. Groups are listed in this order, which is the caller's
    /// own.
    pub first: usize,
    /// How many survivors named this face.
    pub members: usize,
}

/// **What the survivors of the certified order say**, once
/// [`answer_of`] has grouped them by face.
#[derive(Clone, Debug, PartialEq)]
pub enum Answer<K> {
    /// No candidate survived: the typed miss.
    Miss,
    /// Every survivor named one face, which is the answer.
    One(FaceAnswer<K>),
    /// The survivors named more than one face and nothing orders
    /// them: the certified tie, in first-met order.
    Ambiguous(Vec<FaceAnswer<K>>),
}

impl<K> Answer<K> {
    /// The faces this answer names, in the order it lists them: none
    /// for the miss, one for a hit, the whole tie for a refusal.
    ///
    /// The shape a caller comparing a door's answer against a
    /// reference's reads, because a refusal is an ANSWER about the ray
    /// — every hit in it is true — and the three arms differ only in
    /// how many faces it names.
    pub fn faces(self) -> Vec<FaceAnswer<K>> {
        match self {
            Self::Miss => Vec::new(),
            Self::One(one) => vec![one],
            Self::Ambiguous(faces) => faces,
        }
    }
}

/// **THE group rule, once**: the survivors of the certified order
/// ([`TSpan::survivors`]) grouped by the face they name.
///
/// The second half of [`pick_face`]'s documented contract, in one
/// callable beside the first, so that the door and every reference
/// loop that pins it run the same code: several triangles of ONE face
/// are one answer — the hull of their intervals at the smallest
/// rounded `t` among them — and several faces are the tie the door
/// refuses with. A reference keeps its own candidate ENUMERATION,
/// which is what makes it a reference; re-deriving the merge beside it
/// only makes a second door a row could agree with instead.
///
/// `candidates` is every admitted candidate, each with the key that
/// says which face it belongs to, in the order the caller wants the
/// answer LISTED in — for [`pick_face`] that is target order and then
/// flat triangle order. Keys are compared for equality alone: what
/// counts as one face is the caller's, and nothing here orders them.
///
/// Positions in the returned [`FaceAnswer`]s index `candidates`, not
/// the survivors, so a caller reads its own identity back without a
/// second lookup.
pub fn answer_of<K: Clone + PartialEq>(candidates: &[(TSpan, K)]) -> Answer<K> {
    let spans: Vec<TSpan> = candidates.iter().map(|(span, _)| *span).collect();
    let mut groups: Vec<FaceAnswer<K>> = Vec::new();
    for i in TSpan::survivors(&spans) {
        let (span, face) = &candidates[i];
        match groups.iter_mut().find(|group| group.face == *face) {
            Some(group) => {
                // The survivors arrive in the caller's order, so the
                // strict `<` keeps the earlier position at an equal
                // `t` — which decides which POINT of one face is
                // reported, and nothing else.
                if span.t < group.span.t {
                    group.span.t = span.t;
                    group.member = i;
                }
                group.span.t_lo = group.span.t_lo.min(span.t_lo);
                group.span.t_hi = group.span.t_hi.max(span.t_hi);
                group.members += 1;
            }
            None => groups.push(FaceAnswer {
                face: face.clone(),
                span: *span,
                member: i,
                first: i,
                members: 1,
            }),
        }
    }
    match <[FaceAnswer<K>; 1]>::try_from(groups) {
        Ok([only]) => Answer::One(only),
        Err(groups) if groups.is_empty() => Answer::Miss,
        Err(groups) => Answer::Ambiguous(groups),
    }
}

/// The hit point's parameter along the ray WITH the width the
/// arithmetic certifies for it: **the one site where that width is
/// derived**, as [`crossing`] is the one site for the barycentrics'.
///
/// # What the interval encloses
///
/// The parameter of the true crossing, given the corners and the ray
/// as exact, whenever that crossing is a point of the closed triangle.
/// Two sources, added:
///
/// **The barycentrics' own bounds, projected.** [`crossing`] bounds
/// `|ũ − u|` by `err_u` and `|ṽ − v|` by `err_v`. The clamp
/// [`ray_triangle`] applies is the per-coordinate RETRACTION onto the
/// simplex — not the metric projection onto the closed triangle,
/// which would move `(1, 1)` to `(0.5, 0.5)` where this moves it to
/// the corner `(1, 0)`. What the derivation needs is only that it
/// fixes every point of the simplex, so a true `(u, v)`
/// ON the triangle survives it: `|u_c − u| ≤ err_u` directly, and
/// `|v_c − v| ≤ max(err_u, err_v)` because `v`'s admissible range
/// `[0, 1 − u_c]` moves with the clamped `u` — where the clamp pushes
/// `v` down to `1 − u_c` past a true `v` above it, the gap is
/// `u_c − u ≤ err_u`. The hit point therefore moves by at most
/// `err_u·|e1| + max(err_u, err_v)·|e2|`, and a displacement `δ` of the
/// point moves `t = (p − o)·d / (d·d)` by at most
/// `|δ|·|d| / |d|² = |δ| / |d|`.
///
/// **This evaluation's own rounding**, [`PROJECTION_ERROR_UNITS`] over
/// the magnitude sum that constant's derivation counts.
///
/// Both ends are then rounded OUTWARD by one ulp ([`f64::next_down`],
/// [`f64::next_up`]), which covers the two roundings that form them —
/// `fl(from_barycentrics + from_rounding)` and `fl(t ∓ half)` — since
/// one ulp of the result is at least twice either's half-ulp.
///
/// # What it is NOT true of
///
/// The interval is centred on the CLAMPED point's parameter. When the
/// exact crossing is off the closed triangle, that point is not it,
/// and the enclosure claim above does not apply — nor should it: the
/// door answers a point of the triangle. Under the closed acceptance
/// the two are within the clamp's own step of each other, since every
/// admitted `u` is already in `[0, 1]` and every admitted
/// `fl(u + v)` is at most `1`, so the retraction can move `v` only
/// across the half-ulp between `fl(u + v) ≤ 1` and `u + v > 1` — at
/// most `2⁻⁵³·|e2|` of hit point, below the interval's own width.
fn t_span(
    ray: &Ray,
    a: Point3<f64>,
    e1: Vec3<f64>,
    e2: Vec3<f64>,
    (u, err_u): (f64, f64),
    (v, err_v): (f64, f64),
) -> TSpan {
    let hit = a + e1 * u + e2 * v;
    let dd = ray.dir.dot(ray.dir);
    let t = (hit - ray.origin).dot(ray.dir) / dd;
    let from_barycentrics = (err_u * e1.norm() + err_u.max(err_v) * e2.norm()) / ray.dir.norm();
    let o = ray.origin;
    let d = ray.dir;
    let m = (a.x.abs() + (e1.x * u).abs() + (e2.x * v).abs() + o.x.abs()) * d.x.abs()
        + (a.y.abs() + (e1.y * u).abs() + (e2.y * v).abs() + o.y.abs()) * d.y.abs()
        + (a.z.abs() + (e1.z * u).abs() + (e2.z * v).abs() + o.z.abs()) * d.z.abs();
    let from_rounding = PROJECTION_ERROR_UNITS * f64::EPSILON * m / dd;
    let half = from_barycentrics + from_rounding;
    TSpan {
        t,
        t_lo: (t - half).next_down(),
        t_hi: (t + half).next_up(),
    }
}

/// **How far below its own box's entry a candidate's interval can
/// reach** — the traversal's early-out bound, derived HERE from the
/// candidate's triangle and the ray alone, with every term counted.
///
/// [`pick_face`] drops a candidate without evaluating [`crossing`]
/// when `lowest_hi < t_enter − margin`. That is sound exactly when
/// `t_lo ≥ t_enter − margin` for every admitted candidate of this
/// triangle, which is what this function returns a bound for. Writing
/// `p` for the answered (clamped) point, `p'` for the entry point the
/// tree reports and `t(·)` for `(· − o)·d / (d·d)`:
///
/// - **the box's own spread**, `t_enter − t(p) ≤ diag(box)/|d|`.
///   [`bvh::RayCandidate::t_enter`] is a lower bound on the parameter
///   at which the ray ENTERS the box, and `p` is a point of the
///   triangle and so of the box, but a point of a box can project
///   before the ray's entry — by at most the projection's spread over
///   the box, `|x − y|/|d| ≤ diag(box)/|d|`. The box is the exact hull
///   of the three corners ([`PickTable`]), so componentwise
///   `diag_i ≤ |e1_i| + |e2_i|` and `diag ≤ |e1| + |e2|`.
/// - **the interval's own half-width**, `t(p) − t_lo ≤ half + ulp`,
///   with `half = from_barycentrics + from_rounding` ([`t_span`]) and
///   one ulp for the outward [`f64::next_down`]. An admitted
///   barycentric has `err < 1` — [`admits`] refuses `x − err > 0` and
///   `x + err < 1` alike once `err ≥ 1` — so `t_span`'s
///   `err_u·|e1| + max(err_u, err_v)·|e2|` is at most `|e1| + |e2|`
///   TERM BY TERM, and correct rounding is monotone, so the computed
///   `from_barycentrics` never exceeds the computed
///   `(|e1| + |e2|)/|d|` below. The same monotonicity covers
///   `from_rounding`: `|e1_i·u| ≤ |e1_i|` and `|e2_i·v| ≤ |e2_i|` for
///   clamped `u, v ∈ [0, 1]`, and `m` below is `t_span`'s own sum with
///   those substitutions and the same association.
/// - **this evaluation's own rounding**, `t(p) − t ≤ from_rounding`
///   again ([`PROJECTION_ERROR_UNITS`]).
///
/// So the exact deficit is at most `2·(|e1| + |e2|)/|d|` plus
/// `2·from_rounding` plus the `next_down` ulp, and
/// [`EARLY_OUT_ERROR_UNITS`] and [`EARLY_OUT_SLACK_UNITS`] carry the
/// last two. No tolerance and no chosen factor: the leading term is
/// the triangle's own extent, twice.
fn early_out_margin(ray: &Ray, tri: &[Point3<f64>; 3]) -> f64 {
    let a = tri[0];
    let e1: Vec3<f64> = tri[1] - tri[0];
    let e2: Vec3<f64> = tri[2] - tri[0];
    let o = ray.origin;
    let d = ray.dir;
    let dd = d.dot(d);
    let m = (a.x.abs() + e1.x.abs() + e2.x.abs() + o.x.abs()) * d.x.abs()
        + (a.y.abs() + e1.y.abs() + e2.y.abs() + o.y.abs()) * d.y.abs()
        + (a.z.abs() + e1.z.abs() + e2.z.abs() + o.z.abs()) * d.z.abs();
    let raw =
        2.0 * ((e1.norm() + e2.norm()) / d.norm()) + EARLY_OUT_ERROR_UNITS * f64::EPSILON * m / dd;
    raw * (1.0 + EARLY_OUT_SLACK_UNITS * f64::EPSILON)
}

/// The early-out margin's rounding constant, in units of
/// `f64::EPSILON` and counted the way its siblings are: TWICE
/// [`PROJECTION_ERROR_UNITS`], once for the half-width's own
/// `from_rounding` and once for `|t(p) − t|`, plus ONE unit for the
/// outward [`f64::next_down`], whose step is at most
/// `EPSILON·|t − half|` and whose `|t|` part is at most
/// `EPSILON·m/(d·d)`. `2·8 + 1 = 17`. Not a tuned number: change
/// [`t_span`] and re-count.
const EARLY_OUT_ERROR_UNITS: f64 = 17.0;

/// The early-out margin's relative slack, same units. It carries the
/// three things the term-by-term monotonicity above does not:
/// the `next_down` step's remaining part (`EPSILON·half`, at most
/// `2u` of the margin), the final addition and the widening multiply
/// (`u` each), and the margin's own evaluation of
/// `2·(|e1| + |e2|)/|d|` — two `norm`s (`γ₃/2 + u` each, the dot's
/// three products and two sums under a square root), one sum, one
/// division, `< γ₈ < 8.1u` relative. `16 · EPSILON = 32u` clears
/// `13u` with a 2.4× margin. Not a tuned number: change the
/// expression and re-count.
const EARLY_OUT_SLACK_UNITS: f64 = 16.0;

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
    /// the whole test on this pair. A method rather than a second
    /// door: it needs the operands' magnitudes, which the struct does
    /// not carry, but it must read THIS crossing's determinant and
    /// not recompute one.
    pub fn conditioning(&self, ray: &Ray, tri: &[Point3<f64>; 3]) -> f64 {
        let e1: Vec3<f64> = tri[1] - tri[0];
        let e2: Vec3<f64> = tri[2] - tri[0];
        self.det.abs() / (e1.norm() * e2.norm() * ray.dir.norm())
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
/// for whoever tessellated it, and nothing here can see it. That is
/// by design, not a gap: the pick is a question about the picture the
/// user sees, and the tessellation IS what is picked, so the mesh's
/// deviation from the surface it stands for is not a pick error
/// (ruled by Ev on `[ev]` PR 2889; the row
/// `pick-wide-candidate-needs-a-bound-over-mesh-coordinate-error`
/// closed by design).
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

/// The projection's constant, in units of `f64::EPSILON`, derived
/// from the operation count the way its siblings are (`u = EPSILON / 2`
/// the unit roundoff, `γ_n = n·u / (1 − n·u)`). It bounds the rounding
/// of `t = fl(fl(p̃ − o)·d) / fl(d·d)` relative to
/// `M = Σ_i (|a_i| + |e1_i·u| + |e2_i·v| + |o_i|)·|d_i|` over `d·d`:
///
/// - each component of `p̃ = fl(a + fl(e1·u) + fl(e2·v))` costs two
///   products and two sums, `≤ γ_3·(|a_i| + |e1_i u| + |e2_i v|)`;
/// - the subtraction `fl(p̃_i − o_i)` is one more rounding, relative to
///   `|p_i| + |o_i|`, so that component carries
///   `≤ γ_4·(|a_i| + |e1_i u| + |e2_i v| + |o_i|)`;
/// - the dot with `d` (three products, two sums) carries `γ_3` of its
///   own and propagates the above: `≤ (γ_3 + γ_4 + γ_3γ_4)·M < γ_7·M`;
/// - `fl(d·d)` carries `≤ γ_3·(d·d)`, which enters the quotient scaled
///   by `|t|`, and `|t|·(d·d) = |(p − o)·d| ≤ M`;
/// - the division is one rounding more, `≤ u·|t| ≤ u·M / (d·d)`.
///
/// So the whole is `≤ (γ_7 + γ_3 + u)/(1 − γ_3) · M/(d·d) < 11.2·u`
/// of `M/(d·d)`, and `8 · EPSILON = 16·u` clears it with a `1.4×`
/// margin — which also covers `M`'s own evaluation (≤ 8 roundings per
/// term). The cross term `γ_3γ_4` dropped from the third bullet is
/// below `u²·13`, immaterial beside the `4.8·u` the margin leaves
/// spare. The barycentrics' own roundings are NOT here: `err_u` and
/// `err_v` are forward bounds on `ũ` and `ṽ` themselves
/// ([`quotient`]), and [`t_span`] carries them through `|e1|` and
/// `|e2|` as a separate term. Not a tuned number: change the
/// arithmetic and re-count.
const PROJECTION_ERROR_UNITS: f64 = 8.0;

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
    use geom_core::{Point3, Tol, Vec3};
    use mesh::tessellate_with;
    use test_utils::fuzz;
    use topo::Body;

    use super::{
        MeshPick, MeshPickError, PickMemo, PickTable, TSpan, crossing, ray_triangle,
        retract_to_simplex,
    };

    fn unit_prism() -> Body<f64> {
        sweep::test_support::cube(1.0, Tol::witness())
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
    /// instead of desynchronising the flat positions the refusal's
    /// list is stated in.
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
        let t = ray_triangle(ray, tri)
            .unwrap_or_else(|| {
                panic!(
                    "{what}: a well-conditioned interior hit refused; {ray:?} {tri:?}; {}",
                    fuzz::replay()
                )
            })
            .t;
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
            let span = ray_triangle(&ray, &tri).unwrap_or_else(|| panic!("{name} is a hit"));
            assert_eq!(
                span.t.to_bits(),
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
    /// `u + v = 1 ± 12`: each covers the admissible range several
    /// times over. The ray misses the triangle's plane by `2e-21`
    /// while its origin sits a unit away, so the values are the
    /// quotient of that cancellation and nothing else. The closed
    /// comparison admits all three, so
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
            ray_triangle(&ray, &tri).map(|span| span.t),
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
        let ray = ray_through(target, dir, 1.0);
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
        let ray = ray_through(b, dir, 1.0);
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
            ray_triangle(&ray, &abc).map(|span| span.t),
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
            let ray = ray_through(corner, dir, 1.0);
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
                if let Some(span) = ray_triangle(&ray, &tri.corners)
                    && best.is_none_or(|b| span.t < b)
                {
                    best = Some(span.t);
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

    /// **The certified order is a rule about the SET.** `precedes` is a
    /// strict partial order, not a total one, so a pairwise fold over
    /// it answers whichever candidate it met first — the
    /// order-dependence the door's determinism contract forbids. The
    /// triple here is the cycle that shows it: `[−5, 0]` precedes
    /// `[1, 2.5]`, and neither of the other two pairs is ordered at
    /// all.
    ///
    /// [`TSpan::survivors`] takes the survivors of `precedes` as one
    /// set — they are pairwise overlapping, so they are one certified
    /// tie — and nothing orders them further. This row runs it over
    /// every arrival order of the triple and gets one set.
    #[test]
    fn the_certified_order_does_not_depend_on_the_arrival_order() {
        let span = |lo: f64, hi: f64| TSpan {
            t: 0.5 * (lo + hi),
            t_lo: lo,
            t_hi: hi,
        };
        // Named by their place in the cycle, not by the permutation.
        let cycle = [span(1.0, 2.5), span(-0.5, 1.5), span(-5.0, 0.0)];
        assert!(
            cycle[2].precedes(&cycle[0]),
            "the third interval lies wholly below the first"
        );
        assert!(
            cycle[0].width() < cycle[1].width() && cycle[1].width() < cycle[2].width(),
            "and the widths run the other way round the cycle, which decides nothing"
        );
        let mut door = std::collections::BTreeSet::new();
        for order in [
            [0, 1, 2],
            [0, 2, 1],
            [1, 0, 2],
            [1, 2, 0],
            [2, 0, 1],
            [2, 1, 0],
        ] {
            let spans: Vec<TSpan> = order.iter().map(|&i| cycle[i]).collect();
            let mut survived: Vec<usize> = TSpan::survivors(&spans)
                .into_iter()
                .map(|i| order[i])
                .collect();
            survived.sort_unstable();
            door.insert(survived);
        }
        assert_eq!(
            door.len(),
            1,
            "the door's rule answers one SET whatever order it met the candidates in: {door:?}"
        );
        assert_eq!(
            door.iter().next().map(Vec::as_slice),
            Some(&[1usize, 2][..]),
            "and it is the two the first is not in, the first being preceded: {door:?}"
        );
    }

    /// **A ray down a shared edge ties two narrow intervals, and
    /// nothing breaks the tie.** Two triangles mirrored across the
    /// segment from `(0, 0, 0)` to `(1, 0, 0)`, each carrying it as
    /// `(a, b)`, and a `−z` ray through its midpoint: both compute
    /// `u = 0.5`, `v = 0` and `t = 2` without a rounding, and their
    /// bounds are the same magnitudes, so the intervals are identical.
    /// Neither precedes the other, so both survive — in whichever
    /// order they arrive.
    ///
    /// **Both answers are true**, which is what makes the tie a tie:
    /// each triangle places the hit at the midpoint to the bit. What
    /// the door does with two survivors on two different FACES is the
    /// refusal, pinned through the real door by
    /// `pick3_early_out::a_ray_down_a_shared_edge_refuses_with_both_faces`;
    /// this row is the arithmetic underneath it.
    #[test]
    fn a_ray_down_a_shared_edge_ties_two_narrow_intervals_and_nothing_breaks_it() {
        let shared = [Point3::new(0.0, 0.0, 0.0), Point3::new(1.0, 0.0, 0.0)];
        let tris = [
            [shared[0], shared[1], Point3::new(0.5, 1.0, 0.0)],
            [shared[0], shared[1], Point3::new(0.5, -1.0, 0.0)],
        ];
        let midpoint = Point3::new(0.5, 0.0, 0.0);
        let ray = Ray {
            origin: Point3::new(midpoint.x, midpoint.y, 2.0),
            dir: Vec3::new(0.0, 0.0, -1.0),
        };
        let spans: Vec<TSpan> = tris
            .iter()
            .map(|tri| ray_triangle(&ray, tri).expect("the graze is a hit for both triangles"))
            .collect();
        for (i, span) in spans.iter().enumerate() {
            assert_eq!(span.t, 2.0, "triangle {i} answers the midpoint at t = 2");
            let at = ray.origin + ray.dir * span.t;
            assert_eq!(
                [at.x, at.y, at.z].map(f64::to_bits),
                [midpoint.x, midpoint.y, midpoint.z].map(f64::to_bits),
                "triangle {i}'s answer places the hit at the shared edge's midpoint"
            );
            assert!(
                span.width() < 1e-12,
                "triangle {i}'s interval is narrow: {span:?}"
            );
        }
        assert!(
            !spans[0].precedes(&spans[1]) && !spans[1].precedes(&spans[0]),
            "neither interval lies below the other, so the geometry does not order them: \
             {spans:?}"
        );
        assert_eq!(
            spans[0].width(),
            spans[1].width(),
            "the mirrored pair carries the same bound, which orders nothing in any case"
        );
        assert_eq!(
            TSpan::survivors(&spans),
            vec![0, 1],
            "both triangles are in the certified tie"
        );
        let reversed = [spans[1], spans[0]];
        assert_eq!(
            TSpan::survivors(&reversed),
            vec![0, 1],
            "and both again when the candidates arrive the other way round: the SET is the \
             answer, and it does not depend on the arrival order"
        );
    }

    /// **The clamp is a RETRACTION onto the simplex, not the nearest
    /// point of the closed triangle** — the word the door's docs used
    /// to carry. [`retract_to_simplex`] fixes the simplex, which is
    /// all [`t_span`]'s derivation needs, but it is not the metric
    /// projection onto it in the `(u, v)` plane or in the triangle's
    /// own: on the unit right triangle, where the parameter plane IS
    /// the isometry, `(1, 1)` retracts to the corner `(1, 0)` at
    /// distance `1` while the nearest point is `(0.5, 0.5)` at
    /// `0.707`.
    ///
    /// The distinction only bites under an acceptance that admits a
    /// barycentric outside the range; it is pinned because the docs
    /// name the map and a reader is owed the right name. Authored by
    /// review lane `pick3-r1`.
    #[test]
    fn the_clamp_is_a_retraction_onto_the_simplex_and_not_the_nearest_point() {
        let tri = [
            Point3::new(0.0, 0.0, 0.0),
            Point3::new(1.0, 0.0, 0.0),
            Point3::new(0.0, 1.0, 0.0),
        ];
        let e1: Vec3<f64> = tri[1] - tri[0];
        let e2: Vec3<f64> = tri[2] - tri[0];
        let at = |u: f64, v: f64| tri[0] + e1 * u + e2 * v;
        let (cu, cv) = retract_to_simplex(1.0, 1.0);
        assert_eq!(
            (cu, cv),
            (1.0, 0.0),
            "the retraction takes (1, 1) to a corner"
        );
        let dist = |p: Point3<f64>, q: Point3<f64>| (p - q).norm();
        let outside = at(1.0, 1.0);
        assert!(
            cu + cv <= 1.0,
            "it does land on the closed triangle, which is what the width derivation needs"
        );
        assert!(
            dist(at(0.5, 0.5), outside) < dist(at(cu, cv), outside),
            "and the nearest point of the closed triangle is a different point"
        );
    }

    /// **The retraction keeps `u + v ≤ 1` EXACTLY, not just in
    /// `f64`.** `fl(1 − u)` rounds up to `1` at `u = 2⁻⁵⁴`, so a `v`
    /// of `1` clamped against it would place the answered point
    /// `2⁻⁵⁴·|e2|` outside the closed triangle — admitted, because
    /// `fl(u + v) = 1`, and invisible to a membership check that is
    /// itself `f64`. [`retract_to_simplex`]'s exact bound is what
    /// keeps the door's "a point OF the triangle" true to the bit,
    /// which is the premise [`pick_face`]'s early-out proof rests on.
    /// Authored by review lane `pick3-r2`.
    #[test]
    fn the_retraction_keeps_u_plus_v_at_most_one_exactly() {
        let tiny = 2f64.powi(-54);
        assert_eq!(1.0 - tiny, 1.0, "the fixture: fl(1 - u) rounds up to 1");
        let (u, v) = retract_to_simplex(tiny, 1.0);
        assert_eq!(u, tiny, "the u arm leaves an in-range u alone");
        assert!(
            v < 1.0,
            "and the v arm refuses the value fl(1 - u) would have allowed: v = {v}"
        );
        assert!(
            1.0 - v >= u,
            "u + v <= 1 exactly: 1 - v = {} against u = {u:e}",
            1.0 - v
        );
        for (u, v) in [
            (0.0, 1.0),
            (0.25, 0.75),
            (0.5, 0.5),
            (1.0, 0.0),
            (0.75, 0.25),
        ] {
            assert_eq!(
                retract_to_simplex(u, v),
                (u, v),
                "the retraction fixes every point of the simplex"
            );
        }
    }

    /// **The clamp's law, and why no row here reds a door that drops
    /// it.** Under the closed acceptance every admitted `u` is already
    /// in `[0, 1]` and every admitted `fl(u + v)` is at most `1`, so
    /// [`retract_to_simplex`]'s `u` arm is a no-op by construction and
    /// its `v` arm can move `v` only across the half-ULP between
    /// `fl(u + v) ≤ 1` and `u + v > 1` — at most `2⁻⁵³·|e2|` of hit
    /// point, which is below the interval's own width on every ray. A
    /// row that reds when the clamp is dropped therefore cannot exist
    /// while the acceptance is closed; the clamp is here for what it
    /// makes TRUE — the answered point is a point OF the triangle,
    /// which is the premise the traversal's early-out rests on — and
    /// it becomes observable the day the acceptance admits a
    /// barycentric outside the range.
    ///
    /// **This row's own check is `f64` membership, not bits**: it asks
    /// whether the answered point's recovered barycentrics satisfy
    /// `bu + bv <= 1.0` as computed, which is the property a consumer
    /// can act on. The bit-exact statement is
    /// [`the_retraction_keeps_u_plus_v_at_most_one_exactly`], where
    /// `1 − u` rounding up is what the exact bound in
    /// [`retract_to_simplex`] is for.
    ///
    /// What IS pinned here is that law, over the acceptance's own
    /// boundary cases: the answered point lies on the closed triangle
    /// at each of `u = 0`, `u = 1`, `v = 0` and `u + v = 1`.
    #[test]
    fn every_admitted_hit_is_placed_on_the_closed_triangle() {
        let tri = [
            Point3::new(1.0, 1.0, 1.0),
            Point3::new(5.0, 1.0, 1.0),
            Point3::new(1.0, 5.0, 1.0),
        ];
        let dir = Vec3::new(0.0, 0.0, -1.0);
        let corners = [
            ("u = 0, v = 0", 0.0, 0.0),
            ("u = 1", 1.0, 0.0),
            ("v = 1", 0.0, 1.0),
            ("u + v = 1", 0.5, 0.5),
            ("the interior", 0.25, 0.25),
        ];
        for (name, u, v) in corners {
            let ray = Ray {
                origin: Point3::new(1.0 + 4.0 * u, 1.0 + 4.0 * v, 3.0),
                dir,
            };
            let span = ray_triangle(&ray, &tri).unwrap_or_else(|| panic!("{name} is a hit"));
            let at = ray.origin + ray.dir * span.t;
            let (bu, bv) = ((at.x - 1.0) / 4.0, (at.y - 1.0) / 4.0);
            assert!(
                (0.0..=1.0).contains(&bu) && (0.0..=1.0).contains(&bv) && bu + bv <= 1.0,
                "{name}: the answered point {at:?} is a point of the closed triangle \
                 (f64 membership, as a consumer computes it)"
            );
            assert_eq!(at.z, 1.0, "{name}: and it is in the triangle's own plane");
        }
    }
}
