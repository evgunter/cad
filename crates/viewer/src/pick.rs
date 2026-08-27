//! Turning a cursor into a selection: the per-generation pick index,
//! the id↔patch mapping the GPU pass rides on, and the highlight.
//!
//! # One index, three consumers
//!
//! [`PickIndex`] is built once per evaluation generation and answers
//! everything the viewport asks about what is under the cursor:
//!
//! - the **ray path** — [`PickIndex::pick`] un-projects nothing itself;
//!   it takes a ray (from [`crate::Camera::ray_through`]) and hands it
//!   to the shipped `pick_face` service, whose answer is a
//!   `StableName`;
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
//! mesh with a node by hand, and there is no door through which it
//! could.**
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

use pncad::document::{Doc, Evaluation, ProfileProgram, RecipeNodeId};
use pncad::geom_core::Tol;
use pncad::prelude::StableName;
use pncad::select::{HitTestError, NodePick, NodePickError, PickHit, PickTarget, Ray, pick_face};

use crate::camera::{Camera, CameraError};
use crate::evalseam::Generation;
use crate::input::{PickAction, ViewportSize};
use crate::scene::{DisplayTolerance, SceneError, SceneMesh, ScenePart};
use crate::session::{FaceSelection, Selection, SessionOp};

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
    /// The inverse of `names`: which ids a name is drawn as. A `Vec`
    /// because a name CAN be drawn more than once — a body appearing
    /// under two roots draws the same face twice — and the highlight
    /// should light every occurrence rather than the first.
    by_name: BTreeMap<StableName, Vec<u32>>,
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
        let id_slice = ids.ids().collect();
        Ok(Self {
            generation,
            delta,
            parts,
            ids,
            id_slice,
            names,
            by_name,
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

    /// The ids a name is drawn as — **the id map's inverse**, and what
    /// the highlight and the ray/id agreement check both read.
    pub fn ids_of(&self, name: &StableName) -> &[u32] {
        self.by_name.get(name).map_or(&[], Vec::as_slice)
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
        let mut parts: Vec<ScenePart<'_>> = Vec::with_capacity(self.parts.len());
        let mut next = 0usize;
        for part in &self.parts {
            let patches = part.mesh().patches.len();
            // The id list is contiguous per part in `IdMap` order,
            // because the keys were pushed part by part in exactly
            // this order — the same loop that built `names`.
            let ids = self.id_slice.get(next..next + patches).unwrap_or_default();
            next += patches;
            parts.push(ScenePart {
                mesh: part.mesh(),
                ids,
            });
        }
        SceneMesh::build_parts(&parts, self.delta)
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
        let targets: Vec<PickTarget<'_>> = self.parts.iter().map(NodePick::target).collect();
        pick_face(eval, &targets, ray)
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
        Ok(self.pick(eval, ray)?.map(|hit| FaceSelection {
            name: hit.name,
            node: hit.node,
            body: hit.body,
        }))
    }

    /// **The whole cursor path, as one function**: a cursor action
    /// becomes the session operation it denotes.
    ///
    /// The chain is un-projection ([`Camera::ray_through`]) → the ray
    /// service → a typed op. It is one function so that the shipped
    /// path and the tested path are the same path: a headless test
    /// names an action and reads the op, exactly as the viewport does.
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
        let cursor = match action {
            PickAction::ClearHover => return Ok(SessionOp::Hover(None)),
            PickAction::Hover(cursor) | PickAction::Select(cursor) => cursor,
        };
        let ray = camera
            .ray_through(cursor, viewport)
            .map_err(PickError::Camera)?;
        let face = self.face_at(eval, &ray).map_err(PickError::HitTest)?;
        Ok(match action {
            PickAction::Hover(_) => SessionOp::Hover(face),
            PickAction::Select(_) => SessionOp::Select(match face {
                Some(face) => Selection::Face(face),
                None => Selection::None,
            }),
            PickAction::ClearHover => SessionOp::Hover(None),
        })
    }
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
    /// Both payloads show a debug rendering, because neither
    /// `CameraError` nor `HitTestError` has a `Display` — and the rule
    /// this crate follows is that the layer which raised a failure
    /// names it, never a sentence composed here about somebody else's
    /// refusal. `CameraError`'s absence is this crate's own and
    /// pre-existing; `HitTestError`'s is `editor-core`'s, recorded as
    /// a finding rather than papered over with wording invented here.
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Camera(error) => write!(f, "the cursor names no ray: {error:?}"),
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
/// A selection whose name is not drawn in this index — the vanished
/// case — yields [`IdMap::NOTHING`], which is how "nothing lights up"
/// falls out of the resolution-failure semantics rather than being a
/// second implementation of them. A name drawn more than once lights
/// its FIRST id: the uniform block carries one, and the ids of one
/// name are ascending, so the choice is deterministic rather than
/// whichever the map iterated to first.
pub fn highlight(
    index: &PickIndex,
    selection: &Selection,
    hover: Option<&FaceSelection>,
) -> Highlight {
    let first = |name: &StableName| {
        index
            .ids_of(name)
            .first()
            .copied()
            .unwrap_or(IdMap::NOTHING)
    };
    Highlight {
        selected: selection.face().map_or(IdMap::NOTHING, |f| first(&f.name)),
        hovered: hover.map_or(IdMap::NOTHING, |f| first(&f.name)),
    }
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
