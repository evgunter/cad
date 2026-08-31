//! Turning a cursor into a selection: the per-generation pick index,
//! the id↔patch mapping the GPU pass rides on, and the highlight.
//!
//! # One index, three consumers
//!
//! [`PickIndex`] is built once per evaluation generation and answers
//! everything the viewport asks about what is under the cursor:
//!
//! - the **ray path** — [`PickIndex::pick_for`] (and its
//!   display-view-less wrapper [`PickIndex::pick`]) un-projects
//!   nothing itself; it takes a ray (from
//!   [`crate::Camera::ray_through`]) and hands it to the shipped
//!   `pick_face` service, whose answer is a `StableName`;
//! - the **id map** — [`PickIndex::ids`], the pure pair
//!   `id ↔ (node, body, patch)` that the GPU id-buffer pass writes and
//!   reads back;
//! - the **drawn mesh** — [`PickIndex::parts`] are the very
//!   tessellations the picture is built from, so what is drawn and
//!   what is picked are one tessellation rather than two that agree
//!   most of the time.
//!
//! # Why the index holds `NodePick`s and not meshes
//!
//! Arena keys collide numerically across sibling nodes, so a pick
//! index paired with the wrong `(node, body)` answers a plausible,
//! confidently wrong name instead of an error (issue #1098, and
//! `PickTarget`'s own contract). `NodePick` establishes that pairing
//! by construction and offers no other constructor, so the pairing
//! cannot drift as this cache grows a field. **Nothing here re-pairs a
//! mesh with a node by hand, and this type offers no door through
//! which it could** — said about `PickIndex` and about nothing else:
//! whether the FAÇADE hands a consumer the raw-assembly lane is a
//! separate question, answered in `pncad::select`'s own docs.
//!
//! # Staleness is by generation, and it is a discard
//!
//! The key is [`crate::Generation`] — the session's evaluation
//! generation. A [`PickIndex`] built under one generation is never
//! repaired against another: [`PickIndex::current_for`] answers
//! whether the index still describes the run on screen, and a stale
//! one is dropped and rebuilt whole. Re-pairing by hand is the
//! failure #1098 exists to name.

use std::collections::BTreeMap;

use pncad::document::{Doc, Evaluation, Frame, ParamName, ProfileProgram, RecipeNodeId};
use pncad::geom_core::{Point3, Tol};
use pncad::prelude::{NameOrigin, StableName, attribute};
use pncad::select::{HitTestError, NodePick, NodePickError, PickHit, PickTarget, Ray, pick_face};

use crate::camera::{Camera, CameraError};
use crate::display::DisplayView;
use crate::evalseam::Generation;
use crate::input::{PickAction, ViewportSize};
use crate::scene::{DisplayTolerance, SceneError, SceneMesh, ScenePart};
use crate::session::{DocSession, EdgeSelection, FaceSelection, Hovered, Selection, SessionOp};

/// One drawn face patch, addressed the way selection speaks: the node
/// whose body carries it, which output body, and the patch's position
/// in that body's tessellation.
///
/// The patch position is a position in a `Mesh`'s patch list — a
/// display coordinate, valid for one tessellation — and NOT an arena
/// key. It never leaves the generation it was minted under, which is
/// what [`IdMap`] is keyed by.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct PatchId {
    /// The node whose evaluated body carries the patch.
    pub node: RecipeNodeId,
    /// The output body index within that node's value.
    pub body: u32,
    /// The patch's position in that body's tessellation.
    pub patch: usize,
}

/// One drawn edge polyline, addressed the way selection speaks: the
/// node whose body carries it, which output body, and the polyline's
/// position in that body's tessellation.
///
/// The boundary position is a position in a `Mesh`'s boundary list —
/// a display coordinate, valid for one tessellation — and NOT an arena
/// key, for exactly the reason [`PatchId`] is not one. **This is what
/// keeps the edge hit test on the legal side of G1**: the geometry of
/// the test (project the polyline, measure pixels) is layer-3 work,
/// and the only thing that crosses back down is a position, which
/// `NodePick::boundary_names` inverts to a [`StableName`]. The
/// `topo::EdgeKey` behind it never leaves editor-core.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct EdgeId {
    /// The node whose evaluated body carries the edge.
    pub node: RecipeNodeId,
    /// The output body index within that node's value.
    pub body: u32,
    /// The polyline's position in that body's tessellation.
    pub boundary: usize,
}

/// **How close a cursor must come to a drawn edge for the edge to
/// beat the face behind it**, in physical pixels.
///
/// GQ7 leaves pick-priority — "which entity wins when a click hits
/// several" — a GUI question, and this is its first concrete
/// instance: a face fills the pixel an edge only borders, so an edge
/// is unreachable without a rule that lets it win near its own
/// boundary. The rule is *proximity in the picture*, because that is
/// the thing the user is aiming with, and the constant lives here —
/// one place, named — so a later instance of the same question can
/// cite it rather than inventing a second radius.
///
/// Six pixels is a cursor's-width aim: wide enough to hit a
/// hairline-thin edge on a dense model without a steady hand, narrow
/// enough that clicking the middle of a face still selects the face.
/// It is deliberately NOT scaled by device pixel ratio here — the
/// viewport hands this crate physical pixels (`crate::input`'s screen
/// convention) and a hi-dpi screen's finer cursor deserves the finer
/// radius that follows.
pub const EDGE_PICK_RADIUS_PX: f64 = 6.0;

/// The GPU id-buffer's alphabet: a bijection between the u32 an
/// offscreen pass can write into a pixel and the [`PatchId`] it names.
///
/// **A pure function pair, and the reason it is a value.** The GPU
/// half of picking cannot be tested without a GPU; this half can, and
/// it is the half where a wrong answer is silent — an id that inverts
/// to the wrong patch selects the wrong face with no error anywhere.
/// So the mapping is lifted out of the render code entirely and tested
/// for round-trip and collision-freedom in ordinary headless CI, and
/// what is left on the GPU is a pass that writes a number this table
/// gave it.
///
/// [`IdMap::NOTHING`] is reserved: an id buffer is cleared to it, so
/// "the cursor is over no geometry" is a value the pass produces
/// rather than a case the readback has to distinguish some other way.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct IdMap {
    /// Entry `i` is the patch of id `i + 1` — see [`IdMap::NOTHING`].
    entries: Vec<PatchId>,
    /// The inverse. Built once, because the forward direction is what
    /// the draw call needs per patch and a linear scan there is
    /// quadratic in the patch count.
    reverse: BTreeMap<PatchId, u32>,
}

impl IdMap {
    /// The id of "no geometry here". The id buffer's clear value.
    pub const NOTHING: u32 = 0;

    /// Assign ids to `keys`, in the order given.
    ///
    /// Duplicate keys are refused rather than deduplicated: two ids
    /// for one patch would make the round trip lossy in the direction
    /// nobody checks, and a caller that produced one has a pairing bug
    /// worth hearing about.
    ///
    /// # Errors
    ///
    /// [`IdMapError::Duplicate`] for a repeated key, and
    /// [`IdMapError::TooManyPatches`] when the count would exhaust the
    /// `u32` an id buffer pixel holds.
    pub fn build(keys: impl IntoIterator<Item = PatchId>) -> Result<Self, IdMapError> {
        let entries: Vec<PatchId> = keys.into_iter().collect();
        if u32::try_from(entries.len()).is_err() || entries.len() >= u32::MAX as usize {
            return Err(IdMapError::TooManyPatches {
                patches: entries.len(),
            });
        }
        let mut reverse = BTreeMap::new();
        for (index, key) in entries.iter().enumerate() {
            // `index + 1` cannot overflow: the length check above bounds
            // it below `u32::MAX`.
            let id = index as u32 + 1;
            if reverse.insert(*key, id).is_some() {
                return Err(IdMapError::Duplicate { key: *key });
            }
        }
        Ok(Self { entries, reverse })
    }

    /// The id naming `key`, or `None` when this map does not hold it.
    pub fn id_of(&self, key: PatchId) -> Option<u32> {
        self.reverse.get(&key).copied()
    }

    /// The patch `id` names, or `None` for [`IdMap::NOTHING`] and for
    /// any id this map did not assign.
    pub fn key_of(&self, id: u32) -> Option<PatchId> {
        let index = usize::try_from(id.checked_sub(1)?).ok()?;
        self.entries.get(index).copied()
    }

    /// How many patches carry an id.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Whether nothing is addressable.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Every assigned id, ascending — the ids a round-trip check walks.
    pub fn ids(&self) -> impl Iterator<Item = u32> + '_ {
        (1..=self.entries.len()).map(|i| i as u32)
    }
}

/// An id assignment that is not a bijection (closed enum, D4 ¶3).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IdMapError {
    /// One patch was offered twice.
    Duplicate {
        /// The repeated patch.
        key: PatchId,
    },
    /// More patches than a `u32` id buffer can address.
    TooManyPatches {
        /// How many were offered.
        patches: usize,
    },
}

/// Why a pick index could not be built (closed enum, D4 ¶3).
#[derive(Clone, Debug, PartialEq)]
pub enum PickIndexError {
    /// A root's bodies could not be tessellated or indexed. The node
    /// rides along because the payload names the body and not the
    /// root that owns it.
    Node {
        /// The root that refused.
        node: RecipeNodeId,
        /// The service's own refusal, unaltered.
        error: NodePickError,
    },
    /// The patch ids did not form a bijection.
    Ids(IdMapError),
}

impl core::fmt::Display for IdMapError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Duplicate { key } => write!(
                f,
                "patch {} of body {} on node {} was offered twice; an id assignment \
                 is a bijection",
                key.patch, key.body, key.node.0
            ),
            Self::TooManyPatches { patches } => write!(
                f,
                "{patches} patches is more than a 32-bit id buffer can address"
            ),
        }
    }
}

impl core::error::Error for IdMapError {}

impl core::fmt::Display for PickIndexError {
    /// The [`PickIndexError::Ids`] arm forwards to [`IdMapError`]'s own
    /// `Display`. The [`PickIndexError::Node`] arm cannot forward:
    /// `editor-core`'s `NodePickError` has no `Display`, so its value
    /// reaches a reader as a debug rendering until it grows one (issue
    /// #1111). The root it names is this layer's own contribution.
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Node { node, error } => write!(
                f,
                "root {}'s bodies could not be tessellated or indexed: {error:?}",
                node.0
            ),
            Self::Ids(error) => write!(f, "{error}"),
        }
    }
}

impl core::error::Error for PickIndexError {}

/// The pick index for one evaluation generation.
///
/// Built from the document's roots, one [`NodePick`] per output body,
/// in root order then payload order — the order the ids and the drawn
/// parts both follow, so a reader of either can predict the other.
#[derive(Debug)]
pub struct PickIndex {
    generation: Generation,
    delta: DisplayTolerance,
    parts: Vec<NodePick>,
    ids: IdMap,
    /// The assigned ids in key order — `1, 2, 3, …`. Materialized
    /// because [`PickIndex::scene`] hands each part a SLICE of its
    /// own patches' ids, and a slice needs storage to point at.
    id_slice: Vec<u32>,
    /// Every drawn patch's stable name, parallel to the id map's
    /// entries — `Err` for the loud unnamed-face bug arm, which is one
    /// patch's problem and not the index's.
    names: Vec<Result<StableName, HitTestError>>,
    /// The inverse of `names`: which ids a name is drawn as.
    ///
    /// **A `Vec` because a name can be drawn under several
    /// (node, body) pairs** — two `Transform` roots over one extrude is
    /// legal, a transform contributes no role segment, so both drawn
    /// copies carry the extrude's names. It is NOT a `Vec` because a
    /// name can repeat WITHIN one (node, body): a node's name table is
    /// bidirectional (N4), so one name denotes at most one entity of
    /// one body, and [`PickIndex::ids_in`] therefore narrows this list
    /// to at most one id. That is the whole reason a `FaceSelection`
    /// carries its `node` and `body` beside the name.
    by_name: BTreeMap<StableName, Vec<u32>>,
    /// Which ids belong to each drawn (node, body), as a contiguous
    /// window into `id_slice` — the parts are laid out in that order,
    /// so a window is all the bookkeeping this needs.
    ///
    /// The primitive the highlight is built on. Without it the only
    /// question this index could answer was "which ids share a name",
    /// which is the wrong question whenever a name is drawn twice.
    by_target: BTreeMap<(RecipeNodeId, u32), (usize, usize)>,
    /// Every drawn edge polyline, in part order then boundary order —
    /// the edge twin of the patch layout above, and laid out the same
    /// way so one body's edges are a contiguous run.
    edges: Vec<EdgeId>,
    /// Every drawn edge's stable name, parallel to [`PickIndex::edges`]
    /// — `Err` for the loud unnamed-entity bug arm, which is one
    /// polyline's problem and not the index's.
    edge_names: Vec<Result<StableName, HitTestError>>,
    /// Which entries of [`PickIndex::edges`] belong to each drawn
    /// (node, body), as a contiguous window.
    edges_by_target: BTreeMap<(RecipeNodeId, u32), (usize, usize)>,
}

impl PickIndex {
    /// Build the index for `generation` from the document's roots.
    ///
    /// Roots that denote no body at all (datums, profiles, mates) are
    /// skipped: they draw nothing and there is nothing to pick on
    /// them. Every other refusal is returned — a root that failed or
    /// would not tessellate is a picture the viewport cannot draw
    /// either, and swallowing it here would leave a viewport that
    /// silently picks nothing over part of the model.
    ///
    /// # Errors
    ///
    /// [`PickIndexError`], per arm.
    pub fn build(
        doc: &Doc<ProfileProgram>,
        eval: &Evaluation<f64>,
        generation: Generation,
        delta: DisplayTolerance,
        tol: Tol,
    ) -> Result<Self, PickIndexError> {
        let mut parts: Vec<NodePick> = Vec::new();
        for &node in doc.roots() {
            match NodePick::build_all(eval, node, delta.get(), tol) {
                Ok(built) => parts.extend(built),
                Err(NodePickError::NotABody { .. }) => {}
                Err(error) => return Err(PickIndexError::Node { node, error }),
            }
        }
        let mut keys: Vec<PatchId> = Vec::new();
        let mut names: Vec<Result<StableName, HitTestError>> = Vec::new();
        for part in &parts {
            for (patch, name) in part.patch_names(eval).into_iter().enumerate() {
                keys.push(PatchId {
                    node: part.node(),
                    body: part.body(),
                    patch,
                });
                names.push(name);
            }
        }
        let ids = IdMap::build(keys).map_err(PickIndexError::Ids)?;
        let mut by_name: BTreeMap<StableName, Vec<u32>> = BTreeMap::new();
        for (index, name) in names.iter().enumerate() {
            if let Ok(name) = name {
                // `index + 1` is the id `IdMap::build` assigned to the
                // key at the same position; the two lists are built in
                // one pass above, which is what keeps them parallel.
                by_name
                    .entry(name.clone())
                    .or_default()
                    .push(index as u32 + 1);
            }
        }
        let id_slice: Vec<u32> = ids.ids().collect();
        let mut by_target: BTreeMap<(RecipeNodeId, u32), (usize, usize)> = BTreeMap::new();
        let mut next = 0usize;
        for part in &parts {
            let patches = part.mesh().patches.len();
            by_target.insert((part.node(), part.body()), (next, patches));
            next += patches;
        }
        let mut edges: Vec<EdgeId> = Vec::new();
        let mut edge_names: Vec<Result<StableName, HitTestError>> = Vec::new();
        let mut edges_by_target: BTreeMap<(RecipeNodeId, u32), (usize, usize)> = BTreeMap::new();
        for part in &parts {
            let start = edges.len();
            for (boundary, name) in part.boundary_names(eval).into_iter().enumerate() {
                edges.push(EdgeId {
                    node: part.node(),
                    body: part.body(),
                    boundary,
                });
                edge_names.push(name);
            }
            edges_by_target.insert((part.node(), part.body()), (start, edges.len() - start));
        }
        Ok(Self {
            generation,
            delta,
            parts,
            ids,
            id_slice,
            names,
            by_name,
            by_target,
            edges,
            edge_names,
            edges_by_target,
        })
    }

    /// Whether this index still describes the run on screen.
    ///
    /// A `false` here means DISCARD: rebuild the index whole from the
    /// current evaluation. It never means repair.
    pub fn current_for(&self, generation: Option<Generation>, delta: DisplayTolerance) -> bool {
        Some(self.generation) == generation && self.delta == delta
    }

    /// The generation this index was built under.
    pub fn generation(&self) -> Generation {
        self.generation
    }

    /// The δ this index was tessellated at.
    pub fn delta(&self) -> DisplayTolerance {
        self.delta
    }

    /// The drawn parts, in id order.
    pub fn parts(&self) -> &[NodePick] {
        &self.parts
    }

    /// The id↔patch mapping.
    pub fn ids(&self) -> &IdMap {
        &self.ids
    }

    /// The name of the patch `id` denotes.
    ///
    /// `None` for [`IdMap::NOTHING`] and for an id this index did not
    /// assign; `Some(Err(_))` for the loud unnamed-face bug arm.
    pub fn name_of(&self, id: u32) -> Option<&Result<StableName, HitTestError>> {
        self.names.get(usize::try_from(id.checked_sub(1)?).ok()?)
    }

    /// The ids a name is drawn as — **the id map's inverse**, across
    /// every drawn (node, body). A name drawn twice answers two ids;
    /// which of them a SELECTION means is [`PickIndex::ids_of_target`].
    pub fn ids_of(&self, name: &StableName) -> &[u32] {
        self.by_name.get(name).map_or(&[], Vec::as_slice)
    }

    /// Every id drawn for one (node, body), ascending.
    ///
    /// The index lays its parts out in root-then-payload order and
    /// assigns ids in that same walk, so one body's ids are a
    /// contiguous run and this is a slice rather than a search.
    pub fn ids_in(&self, node: RecipeNodeId, body: u32) -> &[u32] {
        let Some(&(start, len)) = self.by_target.get(&(node, body)) else {
            return &[];
        };
        self.id_slice.get(start..start + len).unwrap_or_default()
    }

    /// Every id drawn for one NODE, across all its output bodies.
    ///
    /// The windows of a node's bodies are consecutive — the index lays
    /// its parts out in root-then-payload order — but this collects
    /// rather than slicing, because "consecutive across bodies" is a
    /// property of the build loop rather than a documented postcondition
    /// of the layout, and a highlight is not worth resting on it.
    pub fn ids_of_node(&self, node: RecipeNodeId) -> Vec<u32> {
        self.by_target
            .iter()
            .filter(|((drawn, _), _)| *drawn == node)
            .flat_map(|(_, &(start, len))| {
                self.id_slice.get(start..start + len).unwrap_or_default()
            })
            .copied()
            .collect()
    }

    /// The ids a face selection denotes: the ids of its NAME, narrowed
    /// to its own (node, body).
    ///
    /// **At most one**, by the name table's bidirectionality — but
    /// answered as a slice rather than an `Option` so a caller reads
    /// the narrowing rather than trusting it, and so a naming-emission
    /// bug that broke the bijection would show as a wider answer
    /// instead of a silently chosen one.
    pub fn ids_of_target(&self, face: &FaceSelection) -> Vec<u32> {
        let scope = self.ids_in(face.node, face.body);
        self.ids_of(&face.name)
            .iter()
            .copied()
            .filter(|id| scope.contains(id))
            .collect()
    }

    /// The drawable scene for this index: every part, carrying the ids
    /// its patches are drawn under.
    ///
    /// **The picture is built from the same tessellations the picks
    /// run against**, which is what `NodePick` carrying its mesh is
    /// for. A viewport that tessellated separately for display would
    /// be picking against a mesh nobody can see.
    ///
    /// # Errors
    ///
    /// Every arm of [`crate::SceneError`] the part path can reach.
    pub fn scene(&self) -> Result<SceneMesh, SceneError> {
        self.scene_for(&DisplayView::none())
    }

    /// The drawable scene under a display view: hidden instances'
    /// parts are omitted (the picture drops them; the ids and the
    /// document keep them), and a free-moved instance's parts are
    /// drawn under its probe frame, marked distinct.
    ///
    /// The id windows are walked over EVERY part, drawn or not, so an
    /// id names the same patch whatever is hidden — hide changes what
    /// is emitted, never what an id means.
    ///
    /// # Errors
    ///
    /// As [`PickIndex::scene`]. Hiding EVERYTHING is not an error:
    /// when the index has parts and the view hides all of them, the
    /// answer is [`SceneMesh::empty`] — an honest blank picture whose
    /// bounds are the hidden geometry's, so the camera keeps a real
    /// extent to frame against and the picture is never left stale
    /// behind a refusal.
    pub fn scene_for(&self, display: &DisplayView) -> Result<SceneMesh, SceneError> {
        self.scene_focused(display, &std::collections::BTreeSet::new())
    }

    /// [`PickIndex::scene_for`], marking the patches in `focus` — what
    /// the side panel is showing (see [`focus`]).
    ///
    /// # Errors
    ///
    /// As [`PickIndex::scene_for`].
    pub fn scene_focused(
        &self,
        display: &DisplayView,
        focus: &std::collections::BTreeSet<u32>,
    ) -> Result<SceneMesh, SceneError> {
        let mut parts: Vec<ScenePart<'_>> = Vec::with_capacity(self.parts.len());
        let mut next = 0usize;
        for part in &self.parts {
            let patches = part.mesh().patches.len();
            // The id list is contiguous per part in `IdMap` order,
            // because the keys were pushed part by part in exactly
            // this order — the same loop that built `names`.
            let ids = self.id_slice.get(next..next + patches).unwrap_or_default();
            next += patches;
            if display.hidden_roots.contains(&part.node()) {
                continue;
            }
            parts.push(ScenePart {
                mesh: part.mesh(),
                ids,
                probe: display.moved_roots.get(&part.node()).copied(),
            });
        }
        if parts.is_empty() && !self.parts.is_empty() {
            let bounds = bvh::Aabb::from_points(
                self.parts
                    .iter()
                    .flat_map(|part| part.mesh().positions.iter().copied()),
            )
            .ok_or(SceneError::EmptyMesh)?;
            return Ok(SceneMesh::empty(bounds, self.delta));
        }
        SceneMesh::build_parts_focused(&parts, self.delta, focus)
    }

    /// The nearest face a ray meets, as a stable name.
    ///
    /// A thin call into the shipped service: this offers every part as
    /// a pre-paired target and forwards the answer, adding no policy
    /// of its own. `Ok(None)` is the typed miss.
    ///
    /// # Errors
    ///
    /// [`HitTestError`], verbatim from `pick_face`.
    pub fn pick(&self, eval: &Evaluation<f64>, ray: &Ray) -> Result<Option<PickHit>, HitTestError> {
        self.pick_for(eval, ray, &DisplayView::none())
    }

    /// The nearest face a ray meets **under a display view**: hidden
    /// instances are not offered at all (a hidden part is out of the
    /// pick index exactly as it is out of the picture), and a
    /// free-moved instance is picked WHERE IT IS DRAWN — the ray is
    /// carried into that instance's display-local space through the
    /// probe frame's inverse, so the picture and the pick answer stay
    /// one tessellation even while the display displaces it.
    ///
    /// The comparison across groups is by the hit parameter `t`, which
    /// the display layer keeps comparable by admitting only rigid
    /// probe frames (lengths preserved, so `t` in units of `|dir|`
    /// means the same world distance in every group). Ties keep the
    /// first group considered: the unmoved batch first, then moved
    /// instances in node order — deterministic, stated, and reachable
    /// only by a graze across two instances' coincident triangles.
    ///
    /// # Errors
    ///
    /// [`HitTestError`], verbatim from `pick_face`.
    pub fn pick_for(
        &self,
        eval: &Evaluation<f64>,
        ray: &Ray,
        display: &DisplayView,
    ) -> Result<Option<PickHit>, HitTestError> {
        let visible = |part: &&NodePick| !display.hidden_roots.contains(&part.node());
        let unmoved: Vec<PickTarget<'_>> = self
            .parts
            .iter()
            .filter(visible)
            .filter(|part| !display.moved_roots.contains_key(&part.node()))
            .map(NodePick::target)
            .collect();
        let mut best = pick_face(eval, &unmoved, ray)?;
        for (&node, frame) in &display.moved_roots {
            if display.hidden_roots.contains(&node) {
                continue;
            }
            let targets: Vec<PickTarget<'_>> = self
                .parts
                .iter()
                .filter(|part| part.node() == node)
                .map(NodePick::target)
                .collect();
            if targets.is_empty() {
                continue;
            }
            let map = frame.affine::<f64>();
            let inverse = map.inverse();
            let local = Ray {
                origin: inverse.transform_point(ray.origin),
                dir: inverse.transform_vec(ray.dir),
            };
            if let Some(hit) = pick_face(eval, &targets, &local)? {
                // The hit's point is display-local; the answer the
                // caller compares against the picture is world.
                let world = PickHit {
                    point: map.transform_point(hit.point),
                    ..hit
                };
                let better = match &best {
                    None => true,
                    Some(b) => world.t < b.t,
                };
                if better {
                    best = Some(world);
                }
            }
        }
        Ok(best)
    }

    /// The face selection a ray denotes, or `None` for a miss.
    ///
    /// # Errors
    ///
    /// As [`PickIndex::pick`].
    pub fn face_at(
        &self,
        eval: &Evaluation<f64>,
        ray: &Ray,
    ) -> Result<Option<FaceSelection>, HitTestError> {
        self.face_at_for(eval, ray, &DisplayView::none())
    }

    /// [`PickIndex::face_at`] under a display view.
    ///
    /// # Errors
    ///
    /// As [`PickIndex::pick_for`].
    pub fn face_at_for(
        &self,
        eval: &Evaluation<f64>,
        ray: &Ray,
        display: &DisplayView,
    ) -> Result<Option<FaceSelection>, HitTestError> {
        Ok(self.pick_for(eval, ray, display)?.map(|hit| FaceSelection {
            name: hit.name,
            node: hit.node,
            body: hit.body,
        }))
    }

    /// Every drawn edge of one (node, body), in boundary order.
    ///
    /// A slice rather than a search for the reason
    /// [`PickIndex::ids_in`] is one: the index lays its parts out in
    /// root-then-payload order and walks each part's boundaries in
    /// order, so one body's edges are a contiguous run.
    pub fn edges_in(&self, node: RecipeNodeId, body: u32) -> &[EdgeId] {
        let Some(&(start, len)) = self.edges_by_target.get(&(node, body)) else {
            return &[];
        };
        self.edges.get(start..start + len).unwrap_or_default()
    }

    /// The name of the edge `id` denotes.
    ///
    /// `None` for an id this index did not assign; `Some(Err(_))` for
    /// the loud unnamed-entity bug arm.
    pub fn edge_name_of(&self, id: EdgeId) -> Option<&Result<StableName, HitTestError>> {
        let &(start, len) = self.edges_by_target.get(&(id.node, id.body))?;
        if id.boundary >= len {
            return None;
        }
        self.edge_names.get(start + id.boundary)
    }

    /// The drawn edges an edge selection denotes: the edges of its own
    /// (node, body) whose name is its name.
    ///
    /// **At most one**, by the name table's bidirectionality — and
    /// answered as a `Vec` for the reason [`PickIndex::ids_of_target`]
    /// is: a naming-emission bug that broke the bijection shows as a
    /// wider answer instead of a silently chosen one.
    pub fn edges_of_target(&self, edge: &EdgeSelection) -> Vec<EdgeId> {
        self.edges_in(edge.node, edge.body)
            .iter()
            .copied()
            .filter(|id| matches!(self.edge_name_of(*id), Some(Ok(name)) if *name == edge.name))
            .collect()
    }

    /// The tessellated part drawing one (node, body).
    fn part_of(&self, node: RecipeNodeId, body: u32) -> Option<&NodePick> {
        self.parts
            .iter()
            .find(|part| part.node() == node && part.body() == body)
    }

    /// The world polyline a drawn edge is drawn as, under a display
    /// view: the tessellation's own chord points, carried through the
    /// owning instance's free-move probe frame when it has one.
    ///
    /// Empty for an id this index did not assign and for an edge whose
    /// part is hidden — a hidden part is out of the picture, so there
    /// is nothing to mark, which is the same rule
    /// [`PickIndex::scene_for`] draws by.
    pub fn edge_polyline_for(&self, id: EdgeId, display: &DisplayView) -> Vec<Point3<f64>> {
        if display.hidden_roots.contains(&id.node) {
            return Vec::new();
        }
        let Some(part) = self.part_of(id.node, id.body) else {
            return Vec::new();
        };
        let mesh = part.mesh();
        let Some(boundary) = mesh.boundaries.get(id.boundary) else {
            return Vec::new();
        };
        let place = placement(display, id.node);
        boundary
            .points
            .iter()
            .filter_map(|&index| mesh.positions.get(index as usize).copied())
            .map(place)
            .collect()
    }

    /// **The nearest drawn edge to a cursor, when one is near enough
    /// to beat the face behind it** — the edge half of the pick path,
    /// as a typed layer-boundary service beside the ray→face one.
    ///
    /// # Why this one takes a cursor and not a ray
    ///
    /// An edge has no area on screen: a ray through a pixel misses it
    /// with probability one, so "what the cursor is aiming at" is a
    /// question about PROXIMITY IN THE PICTURE and cannot be asked of
    /// a ray. The measure is therefore pixels
    /// ([`EDGE_PICK_RADIUS_PX`]), which is also what makes the rule
    /// feel the same on a dense model and a coarse one.
    ///
    /// # The mechanism, and what it costs
    ///
    /// The cursor's ray picks a face first, and that hit SEEDS the
    /// search: only the edges of the body the cursor is actually over
    /// are measured. That is cheap (one body's polylines, not the
    /// document's) and it is what makes the answer an edge OF the face
    /// under the cursor rather than of whatever else happens to project
    /// nearby. The price, stated because it is a real limit rather
    /// than an oversight: an edge is reachable only where its own body
    /// is — a cursor just OUTSIDE a silhouette, over the background,
    /// picks nothing even when the silhouette edge is one pixel away.
    ///
    /// The nearest candidate is then checked for OCCLUSION, because
    /// screen distance alone cannot tell the near edge of a box from
    /// its far one: the ray through the candidate's own pixel is
    /// re-picked against everything drawn, and a candidate the surface
    /// hides is rejected rather than selected through the solid.
    ///
    /// Determinism: the candidates within the radius are ordered by
    /// `(pixel distance, boundary position, segment position)` — the
    /// same shape of total tie-break `pick_face` documents, so two
    /// edges meeting at the cursor answer the earlier one every time —
    /// and the answer is the first of them the solid does not hide.
    ///
    /// # Errors
    ///
    /// [`PickError`]: the camera's refusal for a cursor the viewport
    /// cannot un-project, or the hit-test service's.
    pub fn edge_at_for(
        &self,
        eval: &Evaluation<f64>,
        camera: &Camera,
        viewport: ViewportSize,
        cursor: [f64; 2],
        display: &DisplayView,
    ) -> Result<Option<EdgePick>, PickError> {
        let ray = camera
            .ray_through(cursor, viewport)
            .map_err(PickError::Camera)?;
        let Some(hit) = self
            .pick_for(eval, &ray, display)
            .map_err(PickError::HitTest)?
        else {
            return Ok(None);
        };
        self.edge_near(eval, camera, viewport, cursor, display, &hit)
    }

    /// [`PickIndex::edge_at_for`] with nothing hidden or free-moved.
    ///
    /// # Errors
    ///
    /// As [`PickIndex::edge_at_for`].
    pub fn edge_at(
        &self,
        eval: &Evaluation<f64>,
        camera: &Camera,
        viewport: ViewportSize,
        cursor: [f64; 2],
    ) -> Result<Option<EdgePick>, PickError> {
        self.edge_at_for(eval, camera, viewport, cursor, &DisplayView::none())
    }

    /// **What the cursor is over**: the edge when one is within
    /// [`EDGE_PICK_RADIUS_PX`], else the face, else nothing.
    ///
    /// The pick-priority rule lives here, in one place, so the hover
    /// op and the select op cannot disagree about which entity a
    /// cursor means.
    ///
    /// # Errors
    ///
    /// As [`PickIndex::edge_at_for`].
    pub fn hovered_for(
        &self,
        eval: &Evaluation<f64>,
        camera: &Camera,
        viewport: ViewportSize,
        cursor: [f64; 2],
        display: &DisplayView,
    ) -> Result<Option<Hovered>, PickError> {
        let ray = camera
            .ray_through(cursor, viewport)
            .map_err(PickError::Camera)?;
        let Some(hit) = self
            .pick_for(eval, &ray, display)
            .map_err(PickError::HitTest)?
        else {
            return Ok(None);
        };
        if let Some(edge) = self.edge_near(eval, camera, viewport, cursor, display, &hit)? {
            return Ok(Some(Hovered::Edge(edge.selection())));
        }
        Ok(Some(Hovered::Face(FaceSelection {
            name: hit.name,
            node: hit.node,
            body: hit.body,
        })))
    }

    /// [`PickIndex::hovered_for`] with nothing hidden or free-moved.
    ///
    /// # Errors
    ///
    /// As [`PickIndex::edge_at_for`].
    pub fn hovered_at(
        &self,
        eval: &Evaluation<f64>,
        camera: &Camera,
        viewport: ViewportSize,
        cursor: [f64; 2],
    ) -> Result<Option<Hovered>, PickError> {
        self.hovered_for(eval, camera, viewport, cursor, &DisplayView::none())
    }

    /// The nearest edge of the hit body within the pick radius, or
    /// `None`. See [`PickIndex::edge_at_for`] for the whole argument.
    fn edge_near(
        &self,
        eval: &Evaluation<f64>,
        camera: &Camera,
        viewport: ViewportSize,
        cursor: [f64; 2],
        display: &DisplayView,
        hit: &PickHit,
    ) -> Result<Option<EdgePick>, PickError> {
        // The caller un-projected a ray through this cursor already,
        // and that refuses first for a viewport with no area.
        let Some(aspect) = viewport.aspect() else {
            return Ok(None);
        };
        let Some(part) = self.part_of(hit.node, hit.body) else {
            return Ok(None);
        };
        let mesh = part.mesh();
        let place = placement(display, hit.node);
        // One candidate per drawn edge — its own nearest segment —
        // rather than one per segment: a curved edge's chords all lie
        // near the cursor together, and the question being asked is
        // which EDGE the cursor means.
        let mut candidates: Vec<Candidate> = Vec::new();
        for id in self.edges_in(hit.node, hit.body) {
            let Some(boundary) = mesh.boundaries.get(id.boundary) else {
                continue;
            };
            // Each chord point projected once, then the segments
            // walked over the result: a shared point is projected by
            // both of its segments otherwise, and the projection is
            // the expensive half.
            //
            // A chord point on or behind the eye plane has no pixel,
            // so a segment with such an endpoint is not offered:
            // measuring it would need clipping, and an edge running
            // through the eye is not something a cursor is aiming at.
            let mut projected: Vec<Option<(Point3<f64>, [f64; 2])>> =
                Vec::with_capacity(boundary.points.len());
            for &index in &boundary.points {
                let entry = match mesh.positions.get(index as usize) {
                    Some(&position) => {
                        let world = place(position);
                        camera
                            .project(world, aspect)
                            .map_err(PickError::Camera)?
                            .and_then(|ndc| viewport.cursor_of([ndc[0], ndc[1]]))
                            .map(|pixel| (world, pixel))
                    }
                    None => None,
                };
                projected.push(entry);
            }
            let mut best: Option<Candidate> = None;
            for (segment, pair) in projected.windows(2).enumerate() {
                let (Some((a, pixel_a)), Some((b, pixel_b))) = (pair[0], pair[1]) else {
                    continue;
                };
                let (distance, closest) = segment_distance_px(cursor, pixel_a, pixel_b);
                if distance > EDGE_PICK_RADIUS_PX {
                    continue;
                }
                // Strictly nearer only: a tie keeps the earlier
                // segment, which is what makes the answer total.
                if best.as_ref().is_none_or(|best| distance < best.distance) {
                    best = Some(Candidate {
                        distance,
                        boundary: id.boundary,
                        segment,
                        pixel: closest,
                        ends: [a, b],
                    });
                }
            }
            candidates.extend(best);
        }
        // The tie-break, stated once: nearest first, then the earlier
        // boundary, then the earlier segment — all integers after the
        // first, and the first is never NaN (a projected distance is
        // a finite pixel measure).
        candidates.sort_by(|left, right| {
            left.distance
                .partial_cmp(&right.distance)
                .unwrap_or(core::cmp::Ordering::Equal)
                .then((left.boundary, left.segment).cmp(&(right.boundary, right.segment)))
        });
        for candidate in candidates {
            // Occlusion: the ray through this candidate's own pixel,
            // picked against everything drawn. A candidate the solid
            // hides is not what the cursor is aiming at however near
            // it projects — and the walk CONTINUES past it, because
            // the edge behind a face and the edge in front of it are
            // both near the same cursor exactly when a body is seen
            // through its own silhouette.
            let ray = camera
                .ray_through(candidate.pixel, viewport)
                .map_err(PickError::Camera)?;
            let [a, b] = candidate.ends;
            let (t, point) = ray_segment_closest(&ray, a, b);
            let hidden = self
                .pick_for(eval, &ray, display)
                .map_err(PickError::HitTest)?
                .is_some_and(|front| front.t < t - OCCLUSION_SLACK_REL * t.abs());
            if hidden {
                continue;
            }
            let id = EdgeId {
                node: hit.node,
                body: hit.body,
                boundary: candidate.boundary,
            };
            // The loud unnamed-entity arm reaches the caller as a
            // refusal, not as a silent miss.
            let name = match self.edge_name_of(id) {
                Some(Ok(name)) => name.clone(),
                Some(Err(error)) => return Err(PickError::HitTest(*error)),
                None => return Ok(None),
            };
            return Ok(Some(EdgePick {
                name,
                node: id.node,
                body: id.body,
                boundary: candidate.boundary,
                distance_px: candidate.distance,
                point,
            }));
        }
        Ok(None)
    }

    /// **The whole cursor path, as one function**: a cursor action
    /// becomes the session operation it denotes.
    ///
    /// The chain is un-projection ([`Camera::ray_through`]) → the ray
    /// service → the edge-priority rule → a typed op. It is one
    /// function so that the shipped path and the tested path are the
    /// same path: a headless test names an action and reads the op,
    /// exactly as the viewport does.
    ///
    /// Which entity a cursor denotes is [`PickIndex::hovered_for`]'s
    /// answer, so hovering and clicking cannot disagree about it: an
    /// edge within [`EDGE_PICK_RADIUS_PX`] beats the face behind it,
    /// and everywhere else the face wins.
    ///
    /// A [`PickAction::Select`] that hits nothing clears the
    /// selection — clicking empty space is how a user says "nothing",
    /// and the alternative (a click that silently keeps the old
    /// selection) makes the highlight lie about what is selected.
    ///
    /// # Errors
    ///
    /// [`PickError`]: the camera's refusal for a cursor the viewport
    /// cannot un-project, or the hit-test service's.
    pub fn op_for(
        &self,
        eval: &Evaluation<f64>,
        camera: &Camera,
        viewport: ViewportSize,
        action: PickAction,
    ) -> Result<SessionOp, PickError> {
        self.op_under(eval, camera, viewport, action, &DisplayView::none())
    }

    /// [`PickIndex::op_for`] under a display view — the whole cursor
    /// path with hide and free-move applied, which is the one the
    /// application drives.
    ///
    /// # Errors
    ///
    /// As [`PickIndex::op_for`].
    pub fn op_under(
        &self,
        eval: &Evaluation<f64>,
        camera: &Camera,
        viewport: ViewportSize,
        action: PickAction,
        display: &DisplayView,
    ) -> Result<SessionOp, PickError> {
        let cursor = match action {
            PickAction::ClearHover => return Ok(SessionOp::Hover(None)),
            PickAction::Hover(cursor) | PickAction::Select(cursor) => cursor,
        };
        let hovered = self.hovered_for(eval, camera, viewport, cursor, display)?;
        Ok(match action {
            PickAction::Hover(_) => SessionOp::Hover(hovered),
            PickAction::Select(_) => SessionOp::Select(match hovered {
                Some(hovered) => hovered.selection(),
                None => Selection::None,
            }),
            PickAction::ClearHover => SessionOp::Hover(None),
        })
    }
}

/// How far, relative to its own distance from the eye, a candidate
/// edge may sit BEHIND the surface picked at its own pixel and still
/// count as visible.
///
/// It is a float-noise margin and nothing more. The edge polylines and
/// the face patches are one tessellation that shares its boundary
/// positions (the mesh crate's watertightness contract), so a visible
/// edge and the surface at its pixel agree to the last bits, while a
/// hidden edge sits a fraction of the model behind — the two
/// populations are many orders of magnitude apart and the threshold
/// only has to fall between them.
const OCCLUSION_SLACK_REL: f64 = 1.0e-6;

/// A successful edge pick: the stable name, where it was drawn, and
/// how near the cursor came. No arena key — the name IS the reference
/// selection state holds (G1).
///
/// No `PartialEq`: the hit point is float geometry, and `PickHit` — the
/// face pick this is the twin of — states the same by carrying none.
#[derive(Clone, Debug)]
pub struct EdgePick {
    /// The picked edge's stable name.
    pub name: StableName,
    /// The node whose body was hit.
    pub node: RecipeNodeId,
    /// The output body index within that node's value.
    pub body: u32,
    /// The polyline's position in that body's tessellation — a display
    /// coordinate, valid for the generation this index was built under.
    pub boundary: usize,
    /// How far the cursor was from the drawn edge, in physical pixels.
    /// At most [`EDGE_PICK_RADIUS_PX`], by construction.
    pub distance_px: f64,
    /// The point on the edge the cursor was aiming at.
    pub point: Point3<f64>,
}

impl EdgePick {
    /// This pick's drawn-edge address.
    pub fn id(&self) -> EdgeId {
        EdgeId {
            node: self.node,
            body: self.body,
            boundary: self.boundary,
        }
    }

    /// This pick as the selection value the session holds.
    pub fn selection(&self) -> EdgeSelection {
        EdgeSelection {
            name: self.name.clone(),
            node: self.node,
            body: self.body,
        }
    }
}

/// The best drawn edge segment a cursor found so far: how near it
/// came in pixels, which segment it was (the tie-break's coordinates),
/// the pixel to re-pick through for the occlusion check, and the
/// segment's world endpoints.
struct Candidate {
    distance: f64,
    boundary: usize,
    segment: usize,
    pixel: [f64; 2],
    ends: [Point3<f64>; 2],
}

/// Where a drawn part's geometry actually lands: its own coordinates,
/// or those coordinates through the free-move probe frame the display
/// view puts the owning instance under.
///
/// One home for the displacement, because a highlight drawn at the
/// tessellated placement while the picture draws the probed one is a
/// mark on empty space.
fn placement(display: &DisplayView, node: RecipeNodeId) -> impl Fn(Point3<f64>) -> Point3<f64> {
    let map = display
        .moved_roots
        .get(&node)
        .copied()
        .map(|frame: Frame| frame.affine::<f64>());
    move |point| match &map {
        Some(map) => map.transform_point(point),
        None => point,
    }
}

/// How far `cursor` is from the segment `a`–`b` in pixels, and the
/// point of the segment it is that far from.
fn segment_distance_px(cursor: [f64; 2], a: [f64; 2], b: [f64; 2]) -> (f64, [f64; 2]) {
    let (dx, dy) = (b[0] - a[0], b[1] - a[1]);
    let length2 = dx.powi(2) + dy.powi(2);
    // A segment whose endpoints project to one pixel is a point.
    let t = if length2 > 0.0 {
        (((cursor[0] - a[0]) * dx + (cursor[1] - a[1]) * dy) / length2).clamp(0.0, 1.0)
    } else {
        0.0
    };
    let closest = [a[0] + dx * t, a[1] + dy * t];
    let distance = ((cursor[0] - closest[0]).powi(2) + (cursor[1] - closest[1]).powi(2)).sqrt();
    (distance, closest)
}

/// The point of the segment `a`–`b` nearest `ray`, and its distance
/// along the ray.
///
/// The ray comes from the pixel the segment's own projection is
/// nearest, so the two very nearly meet; what this answers is where.
/// A segment pointing at the eye makes the pair near-parallel and the
/// crossing ill-conditioned — there the answer degrades to the
/// endpoint, which is the honest reading of "any point will do" rather
/// than a division by a number near zero.
fn ray_segment_closest(ray: &Ray, a: Point3<f64>, b: Point3<f64>) -> (f64, Point3<f64>) {
    let along = ray.dir;
    let segment = b - a;
    let offset = a - ray.origin;
    let (aa, ab, bb) = (along.dot(along), along.dot(segment), segment.dot(segment));
    let denominator = aa * bb - ab.powi(2);
    let t_segment = if denominator != 0.0 && bb > 0.0 {
        (ab * along.dot(offset) - aa * segment.dot(offset)) / denominator
    } else {
        0.0
    }
    .clamp(0.0, 1.0);
    let point = a + segment * t_segment;
    let t_ray = if aa > 0.0 {
        along.dot(point - ray.origin) / aa
    } else {
        0.0
    };
    (t_ray, point)
}

/// Why a cursor produced no answer (closed enum, D4 ¶3).
#[derive(Clone, Debug, PartialEq)]
pub enum PickError {
    /// The cursor could not be un-projected.
    Camera(CameraError),
    /// The hit test refused.
    HitTest(HitTestError),
}

impl core::fmt::Display for PickError {
    /// The rule this crate follows is that the layer which raised a
    /// failure names it, never a sentence composed here about somebody
    /// else's refusal. [`PickError::Camera`] forwards to
    /// [`CameraError`]'s own `Display`. [`PickError::HitTest`] cannot:
    /// `editor-core`'s `HitTestError` has no `Display`, so its value
    /// reaches a reader as a debug rendering until it grows one (issue
    /// #1111).
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Camera(error) => write!(f, "the cursor names no ray: {error}"),
            Self::HitTest(error) => write!(f, "the hit test refused: {error:?}"),
        }
    }
}

impl core::error::Error for PickError {}

/// Which drawn patches the viewport should mark, and how.
///
/// **A pure function of (index, selection, hover)** — see
/// [`highlight`]. Nothing is retained: the value is recomputed each
/// frame from state that lives in exactly one place, which is the
/// discipline the panels established and the reason no widget here
/// holds a "currently highlighted" field.
///
/// Both fields are [`IdMap::NOTHING`] when nothing is marked, so the
/// GPU consumes them as plain uniforms with no branch for absence.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Highlight {
    /// The selected patch's id, or [`IdMap::NOTHING`].
    pub selected: u32,
    /// The hovered patch's id, or [`IdMap::NOTHING`].
    pub hovered: u32,
}

/// The highlight for a selection and a hover, against the index that
/// describes what is drawn.
///
/// **Scoped to the selection's own (node, body)**, not merely to its
/// name. A name can be drawn twice — two `Transform` roots over one
/// extrude carry the same names on both copies — and marking "the
/// first id of the name" then lights the OTHER placement, which is the
/// deliverable failing at exactly the shape it is hardest to notice.
/// [`PickIndex::ids_of_target`] does the narrowing, and it narrows to
/// at most one id because a node's name table is a bijection.
///
/// A selection whose name is not drawn in this index — the vanished
/// case — yields [`IdMap::NOTHING`], which is how "nothing lights up"
/// falls out of the resolution-failure semantics rather than being a
/// second implementation of them. So does a selection whose name IS
/// drawn but not on the body it was picked from, which is the same
/// statement said about a stale index.
pub fn highlight(index: &PickIndex, selection: &Selection, hover: Option<&Hovered>) -> Highlight {
    let mark = |face: &FaceSelection| {
        index
            .ids_of_target(face)
            .first()
            .copied()
            .unwrap_or(IdMap::NOTHING)
    };
    Highlight {
        selected: selection.face().map_or(IdMap::NOTHING, mark),
        hovered: hover.and_then(Hovered::face).map_or(IdMap::NOTHING, mark),
    }
}

/// The edge marks a frame draws: the drawn polylines of the selected
/// and hovered edges, as line-list segment pairs in world space.
///
/// **A value, so the marking is checkable without pixels.** A test
/// asserts which segments a selection lights and where they are; what
/// colour they come out is the theme's answer and the shader's, and
/// neither is asserted here.
///
/// The buffers are `f32` because that is what a GPU consumes and this
/// is the display seam — the same cast, at the same boundary, that
/// [`crate::scene::SceneMesh`] makes.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct EdgeOverlay {
    /// The selected edge's segments, two positions per segment.
    pub selected: Vec<[f32; 3]>,
    /// The hovered edge's segments, two positions per segment.
    pub hovered: Vec<[f32; 3]>,
}

impl EdgeOverlay {
    /// Whether there is nothing to draw.
    pub fn is_empty(&self) -> bool {
        self.selected.is_empty() && self.hovered.is_empty()
    }

    /// How many line segments this overlay draws.
    pub fn segments(&self) -> usize {
        (self.selected.len() + self.hovered.len()) / 2
    }
}

/// **The edge half of the highlight**, as a pure function of (index,
/// display view, selection, hover) — the twin of [`highlight`], which
/// answers the face half by patch id.
///
/// Edges are drawn rather than tinted, so they cannot ride the id
/// comparison the face marks use: a face mark is a patch the shader
/// recognises, and an edge mark is geometry that has to be handed to
/// the renderer. What the two share is the RULE — the mark is scoped
/// to the selection's own (node, body), so an edge whose name is drawn
/// twice lights the copy it was picked from and not the other one, and
/// a selection whose name is not drawn at all lights nothing. That is
/// the resolution-failure semantics falling out of the same narrowing
/// rather than being implemented a second time.
///
/// A hover on the edge that is already selected draws only the
/// selected mark: selection is the state the user committed to, which
/// is the precedence the shader's face path already states.
pub fn edge_overlay(
    index: &PickIndex,
    display: &DisplayView,
    selection: &Selection,
    hover: Option<&Hovered>,
) -> EdgeOverlay {
    let mark = |edge: &EdgeSelection| -> Vec<[f32; 3]> {
        index
            .edges_of_target(edge)
            .first()
            .map(|id| segments_of(&index.edge_polyline_for(*id, display)))
            .unwrap_or_default()
    };
    let selected_edge = selection.edge();
    let hovered_edge = hover.and_then(Hovered::edge).filter(|edge| {
        // The one already marked as selected is not marked twice.
        selected_edge != Some(*edge)
    });
    EdgeOverlay {
        selected: selected_edge.map(mark).unwrap_or_default(),
        hovered: hovered_edge.map(mark).unwrap_or_default(),
    }
}

/// A polyline as the line-list pairs a GPU draws.
fn segments_of(polyline: &[Point3<f64>]) -> Vec<[f32; 3]> {
    let corner = |point: &Point3<f64>| [point.x as f32, point.y as f32, point.z as f32];
    polyline
        .windows(2)
        .flat_map(|pair| [corner(&pair[0]), corner(&pair[1])])
        .collect()
}

/// **What the picture marks because it is what the side panel is
/// showing.** The ids of every drawn patch the selection MADE.
///
/// Distinct from [`highlight`], and the distinction is the point.
/// `highlight` marks the ONE patch a pick landed on — an answer about
/// the cursor. This marks the whole extent of the thing being EDITED,
/// which for a feature is every face it made, and for a document
/// parameter is every face of every feature that parameter drives.
/// Selecting an extrude in the feature tree lights its walls; clicking
/// one of those walls lights the same set, with the picked patch
/// additionally tinted by `highlight` — and that holds however many
/// features later carried the wall, because a click resolves to the
/// feature that MADE the face (`FaceSelection::feature`) rather than
/// to whichever root drew it.
///
/// **Made, not merely drawn under.** A patch belongs to the node that
/// MINTED the entity its name denotes, which
/// [`pncad::select::attribute`] reads off the name's own
/// carry-through segments: a fillet's `FromTarget(f)` face is still
/// the target's face `f`, so a fillet's extent is the blends and
/// corners it created and nothing else. Which node DRAWS a patch is a
/// different question with a different answer — on a body whose whole
/// history ends in one outer feature, that feature draws every face
/// and made almost none of them.
///
/// **A node that made nothing drawn still focuses something**, in two
/// steps. A `Transform`, or a tool body a boolean consumed, mints no
/// drawn entity but drawn entities pass THROUGH it, and those are its
/// extent — the geometry built on top of it. Passing through is read
/// off the name where the op re-named what it carried, and off the
/// recipe where it did not: a `Transform` contributes no role segment
/// by construction, so what it carries is what was minted below it
/// (`display::derives_from`). Failing that, a node no
/// drawn name mentions at all — a profile, a datum plane, a sketch —
/// marks the drawn roots deriving from it
/// (`display::roots_deriving_from`): a profile's line and the wall it
/// swept are one thing seen twice. That last step is also where a name
/// the vocabulary walk cannot classify degrades to, so an
/// unclassified role costs the whole-body picture rather than an empty
/// one.
///
/// **What it does NOT do yet (issue 1182)**, stated so the gap is not
/// mistaken for a decision: the marking is per NODE, so selecting a
/// profile lights the whole body built from it rather than the walls of
/// the one segment being edited. Per-segment marking is expressible in
/// this type — the answer is a set of patch ids and nothing about the
/// shape assumes a whole node's worth — and wants the profile-step ↔
/// `RoleSeg::Lateral(ProfileEdgeRef)` correspondence established rather
/// than guessed: a slot's `step` is an index in the AUTHORING chain and
/// the name's `segment` an index in the LOWERED one, and one authored
/// step can lower to several segments. A wrong guess there lights a
/// confidently wrong face, silently.
///
/// A selection whose referent is not drawn — vanished, unevaluated,
/// hidden, or a feature that produces no body at all — answers the
/// empty set, which is how "nothing lights up" falls out of the same
/// rule rather than being a case.
pub fn focus(
    index: &PickIndex,
    doc: &Doc<ProfileProgram>,
    selection: &Selection,
) -> std::collections::BTreeSet<u32> {
    let nodes: Vec<RecipeNodeId> = match selection {
        Selection::None => Vec::new(),
        Selection::Node(node) => vec![*node],
        // The feature the face IS, not the root that drew it — the
        // same inversion the tree and the panel read
        // (`FaceSelection::feature`), so a click and a tree selection
        // of one feature mark one set.
        Selection::Face(face) => vec![face.feature()],
        // The feature the EDGE is, by the same inversion: an edge is
        // the same kind of picked entity a face is, and selecting one
        // shows the same feature's rows.
        Selection::Edge(edge) => vec![edge.feature()],
        // Every node the parameter drives. A parameter is the one
        // selection with no geometry of its own, and the useful
        // question about it is exactly "what does this number move".
        Selection::Param(name) => doc
            .order()
            .iter()
            .copied()
            .filter(|&id| drives(doc, id, name))
            .collect(),
    };
    if nodes.is_empty() {
        return std::collections::BTreeSet::new();
    }
    // One walk of the names per call, not one per selected node: a
    // parameter selection asks the same question of every node it
    // drives.
    let made: Vec<(u32, NameOrigin)> = index
        .ids()
        .ids()
        .filter_map(|id| Some((id, attribute(index.name_of(id)?.as_ref().ok()?))))
        .collect();
    let mut out = std::collections::BTreeSet::new();
    for node in nodes {
        out.extend(marked_for(index, doc, &made, node));
    }
    out
}

/// The patches ONE node is responsible for: what it minted, else what
/// passes through it, else the roots built from it (see [`focus`]).
fn marked_for(
    index: &PickIndex,
    doc: &Doc<ProfileProgram>,
    made: &[(u32, NameOrigin)],
    node: RecipeNodeId,
) -> std::collections::BTreeSet<u32> {
    let pick = |keep: &dyn Fn(&NameOrigin) -> bool| -> std::collections::BTreeSet<u32> {
        made.iter()
            .filter(|(_, at)| keep(at))
            .map(|(id, _)| *id)
            .collect()
    };
    let minted = pick(&|at| at.minted_by() == Some(node));
    if !minted.is_empty() {
        return minted;
    }
    // Passing through, by the name and by the recipe. A name records
    // the ops that RE-NAMED the entity, so an op that contributes no
    // role segment — a `Transform` — is invisible to the walk; the
    // entities it carries are the ones minted anywhere below it.
    let below: std::collections::BTreeSet<RecipeNodeId> = made
        .iter()
        .filter_map(|(_, at)| at.minted_by())
        .collect::<std::collections::BTreeSet<_>>()
        .into_iter()
        .filter(|&minter| crate::display::derives_from(doc, node, minter))
        .collect();
    let through =
        pick(&|at| at.passes_through(node) || at.minted_by().is_some_and(|m| below.contains(&m)));
    if !through.is_empty() {
        return through;
    }
    crate::display::roots_deriving_from(doc, node)
        .into_iter()
        .flat_map(|root| index.ids_of_node(root))
        .collect()
}

/// Whether any of `node`'s slot expressions reads the parameter
/// `name` — through `Expr::param_refs`, the public read side, so a
/// reference nested inside arithmetic counts exactly as a bare one
/// does.
fn drives(doc: &Doc<ProfileProgram>, node: RecipeNodeId, name: &ParamName) -> bool {
    let Some(recipe_node) = doc.node(node) else {
        return false;
    };
    recipe_node.slots().into_iter().any(|slot| {
        recipe_node.expr(slot).is_some_and(|expr| {
            let mut refs = Vec::new();
            expr.param_refs(&mut refs);
            refs.iter().any(|(referenced, _)| referenced == name)
        })
    })
}

/// The view-projection that puts ONE source pixel over the whole 1×1
/// target the GPU id pass renders into.
///
/// A pixel centred at `cursor_ndc` spans `2 / width` by `2 / height` of
/// normalized device space, so translating that point to the origin and
/// scaling by the viewport's pixel dimensions maps exactly that pixel
/// onto the target's `[−1, 1]²`. In a column-major clip-space matrix
/// the translation is a subtraction of `cursor · w`, which is why the
/// `w` row participates.
///
/// **It lives here, out of the render module, because it is the one
/// part of the id pass a machine with no GPU can check**: composed
/// with [`Camera::project`] it says that the world point the ray path
/// un-projects to is the point the id pass rasterizes at the centre of
/// its target. That composition is the headless half of "both picking
/// paths answer the same question".
pub fn cursor_projection(
    view_projection: &[[f32; 4]; 4],
    cursor_ndc: [f32; 2],
    viewport_px: [f32; 2],
) -> [[f32; 4]; 4] {
    let [cx, cy] = cursor_ndc;
    let [sx, sy] = viewport_px;
    let mut out = *view_projection;
    for column in &mut out {
        let w = column[3];
        column[0] = (column[0] - cx * w) * sx;
        column[1] = (column[1] - cy * w) * sy;
    }
    out
}

/// A [`PickIndex`] kept current with a session — **the rebuild-on-stale
/// loop, owned once**.
///
/// Two things made this a type rather than a habit. Ergonomics: a
/// consumer that only wanted to pick had to notice `current_for` said
/// stale, then rebuild with four arguments (document, evaluation,
/// generation, δ) it had to keep in step by hand, three of which come
/// from one `DocSession`. And correctness: the application's own
/// rebuild loop retried on **every repainted frame** whenever a build
/// refused — a failed or poisoned root is an ordinary editing state,
/// and each frame then re-tessellated every healthy root before
/// reaching the failing one, behind a picture that was already stale.
///
/// So the retry policy is stated once, here: **at most one attempt per
/// (landed generation, δ)**, success or failure. A failure is kept and
/// readable ([`PickCache::error`]) rather than retried into a stall.
#[derive(Debug, Default)]
pub struct PickCache {
    index: Option<PickIndex>,
    /// What the last attempt was for. `Some` after any attempt,
    /// successful or not — which is what stops the retry loop.
    attempted: Option<(Generation, DisplayTolerance)>,
    error: Option<PickIndexError>,
}

/// What one [`PickCache::sync`] did.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CacheStep {
    /// The held index already describes the run on screen.
    Current,
    /// Rebuilt for a new generation or δ.
    Rebuilt,
    /// The rebuild refused; the error is on the cache and will NOT be
    /// retried until the generation or δ moves.
    Refused,
    /// This attempt was already made and refused — nothing was done.
    Held,
    /// No evaluation has landed, so there is nothing to index.
    Nothing,
}

impl PickCache {
    /// An empty cache.
    pub fn new() -> Self {
        Self::default()
    }

    /// Bring the cache in line with the session's landed evaluation at
    /// `delta`, at most one build attempt per (generation, δ).
    ///
    /// **δ is built at, verbatim.** `scene::TRIANGLE_BUDGET` chooses
    /// the δ a document OPENS at (`app`'s `fit_delta_on_scene`), and
    /// that is the whole of the budget's authority: once a δ is in
    /// force it is the value someone asked for, and a cache that
    /// quietly built a different picture would make the View pane's δ
    /// field a control that does nothing.
    pub fn sync(&mut self, session: &DocSession, delta: DisplayTolerance) -> CacheStep {
        let Some(generation) = session.landed_generation() else {
            return CacheStep::Nothing;
        };
        if self
            .index
            .as_ref()
            .is_some_and(|index| index.current_for(Some(generation), delta))
        {
            return CacheStep::Current;
        }
        if self.attempted == Some((generation, delta)) {
            // Attempted and refused for this exact picture. Retrying
            // is the per-frame rebuild loop; the error is already
            // recorded and the caller has already seen it.
            return CacheStep::Held;
        }
        self.attempted = Some((generation, delta));
        self.index = None;
        let Some((doc, eval)) = session.landed_pair() else {
            return CacheStep::Nothing;
        };
        match PickIndex::build(doc, eval, generation, delta, session.tol()) {
            Ok(index) => {
                self.index = Some(index);
                self.error = None;
                CacheStep::Rebuilt
            }
            Err(error) => {
                self.error = Some(error);
                CacheStep::Refused
            }
        }
    }

    /// The held index, if the last attempt produced one.
    pub fn index(&self) -> Option<&PickIndex> {
        self.index.as_ref()
    }

    /// Why the last attempt refused, if it did.
    pub fn error(&self) -> Option<&PickIndexError> {
        self.error.as_ref()
    }
}
