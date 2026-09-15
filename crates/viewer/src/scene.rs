//! What the viewport draws: an evaluated document's product,
//! tessellated at a display tolerance.
//!
//! The whole path runs through the public authoring doors, in the
//! order an outside consumer would write it — author a `Doc`, hand it
//! to `evaluate`, gather the product, tessellate — because a viewport
//! fed by a hand-built mesh would be evidence about the viewport and
//! not about the library (`memories/demo-purpose.md`).
//!
//! # δ, and only δ
//!
//! [`DisplayTolerance`] is the fidelity lever: it is `mesh`'s chordal
//! δ, a per-call display parameter saying how far triangles may sag
//! from the exact surfaces. The kernel tolerance ε — what the model
//! *is* — never appears as a knob here. Coarsening the view is a
//! change of picture, never a change of model.
//!
//! # Flat shading, and why the mesh is expanded
//!
//! Each triangle gets its own three vertices carrying the triangle's
//! own geometric normal. That is deliberate rather than lazy: a
//! smoothed normal would blur the tessellation's real chordal error
//! into something prettier than the model, and the facets are exactly
//! what a δ reading should let you see.
//!
//! Module kind: **vocabulary** — it names no driver type and no
//! `app`-only crate (`crates/viewer/README.md`, Module boundaries).

use std::collections::BTreeSet;

use bvh::Aabb;
use pncad::document::{
    CancelToken, Datum, Dimension, Doc, DocEdit, EvalOptions, Expr, Frame, LoopProgram, Node,
    ProductError, ProfileProgram, RecipeNodeId, apply, evaluate, product,
};
use pncad::geom_core::{Affine3, Point3, Tol, Vec3};
use pncad::mesh::{Mesh, TessellateError, tessellate};
use pncad::topo::Body;

/// The chordal display tolerance δ: how far the drawn triangles may
/// sag from the exact surfaces.
///
/// A newtype rather than a bare `f64` so a caller cannot pass a
/// kernel ε where a display δ belongs. Finite and strictly positive
/// by construction, which is also `mesh::tessellate`'s own condition.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct DisplayTolerance(f64);

impl DisplayTolerance {
    /// A display tolerance, refused unless finite and strictly
    /// positive.
    ///
    /// **What this door checks is exactly that, and no more.** It is
    /// `mesh::tessellate`'s `InvalidChordalTolerance` condition —
    /// `chordal.is_finite() && chordal > 0.0` — hoisted so a caller
    /// meets it once at construction rather than at every call.
    ///
    /// **What it does NOT foreclose**, stated because the first
    /// version of this sentence implied it did: a δ that is a valid
    /// length but too fine for a *particular* body still refuses
    /// downstream, typed, as
    /// [`SceneError::NotTessellated`] — `f64::MIN_POSITIVE` is
    /// accepted here and produces `ResolutionOverflow` at
    /// tessellation. That residual is body-dependent (it is a function
    /// of δ against the body's own extent and curvature), so this
    /// door, which sees no body, cannot answer it. Answering it here
    /// would mean re-deriving the tessellator's sizing rule in a
    /// second place — a second opinion about another crate's refusal,
    /// which is worse than a narrower door with an honest doc.
    ///
    /// # Errors
    ///
    /// [`SceneError::InvalidDisplayTolerance`] for a δ that is not a
    /// finite, strictly positive length.
    pub fn new(delta: f64) -> Result<Self, SceneError> {
        if delta.is_finite() && delta > 0.0 {
            Ok(Self(delta))
        } else {
            Err(SceneError::InvalidDisplayTolerance { delta })
        }
    }

    /// The value, in world units.
    pub fn get(self) -> f64 {
        self.0
    }

    /// This δ in millimetres, as text a person reads.
    ///
    /// **The δ-facing door onto [`crate::readout::number`]**, which is
    /// the crate's one rule for a number a person reads: the shortest
    /// decimal spelling that reads back as this value, and a scientific
    /// one when no decimal spelling does. What this method adds is the
    /// millimetre conversion the δ field's own commit path uses, and
    /// nothing else.
    ///
    /// **No δ renders as `0.000`.** The rule refuses it without knowing
    /// anything about δ: a text reading zero is a hundred percent away
    /// from a strictly positive value, and the render's accuracy bound
    /// ([`crate::readout::REL_TOLERANCE`]) is five parts in ten
    /// thousand. So the thing that used to be a second predicate here —
    /// that the text read back as a δ [`DisplayTolerance::new`] accepts
    /// — is implied by the first for every δ this type can hold, and
    /// `no_delta_renders_as_a_number_a_delta_cannot_be` is where that
    /// implication is checked rather than restated.
    ///
    /// **What it is not is exact.** Four significant figures is what a
    /// ten-character bound buys, and a δ the triangle budget chose is
    /// `constant / TRIANGLE_BUDGET` — seventeen. The other thirteen
    /// figures are shown nowhere, which is why this render is a render
    /// and never a commit path: the number a δ moves to is the one a
    /// user types, never one the chrome echoed at them.
    pub fn render_mm(self) -> String {
        crate::readout::number(self.0 * 1.0e3)
    }

    /// This tolerance scaled by `factor` — the coarsen/refine step the
    /// chrome offers.
    ///
    /// # Errors
    ///
    /// As [`DisplayTolerance::new`], on the product.
    pub fn scaled(self, factor: f64) -> Result<Self, SceneError> {
        Self::new(self.0 * factor)
    }
}

/// A refusal on the way from a document to a drawable scene (closed
/// enum, D4 ¶3).
#[derive(Debug)]
pub enum SceneError {
    /// δ was not a finite, strictly positive length.
    InvalidDisplayTolerance {
        /// The offending value.
        delta: f64,
    },
    /// The document's roots did not gather into a product body: a
    /// failed or poisoned root, or a document denoting no body.
    NoProduct(ProductError),
    /// The body did not tessellate at this δ.
    NotTessellated(TessellateError),
    /// The tessellation was empty, or its positions gave no usable
    /// bounding box — nothing to look at, and nothing to frame a
    /// camera against.
    EmptyMesh,
    /// A part offered its patches' ids, but not one per patch.
    ///
    /// A part either carries no ids at all (nothing in it is
    /// pickable) or one per patch. A short or long list is a caller
    /// whose id assignment and whose tessellation disagree, and
    /// drawing it would put ids on the wrong triangles — the silent
    /// wrong answer the whole id mapping exists to make impossible.
    MispairedIds {
        /// How many ids were offered.
        ids: usize,
        /// How many patches the part has.
        patches: usize,
    },
    /// A face patch named a vertex index outside the mesh's shared
    /// position table.
    ///
    /// Its own arm because it is a **broken mesh**, not a display
    /// outcome: `EmptyMesh` says "this body drew nothing", which a
    /// caller might reasonably show as an empty viewport, while this
    /// says the tessellator's two halves disagree and nothing about
    /// the scene can be trusted. One arm for both made the code's own
    /// comment ("a broken mesh, not a display choice") a correction of
    /// the arm it was returning.
    BrokenPatchIndex {
        /// The out-of-range index.
        index: u32,
        /// How many positions the table actually holds.
        positions: usize,
    },
}

impl core::fmt::Display for SceneError {
    /// The payload-carrying arms forward to their payload's own
    /// `Display` — [`ProductError`] and `mesh`'s `TessellateError`
    /// each name their own failure, and this layer does not restate
    /// it.
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::InvalidDisplayTolerance { delta } => write!(
                f,
                "{delta} is not a finite, strictly positive display tolerance"
            ),
            Self::NoProduct(error) => write!(f, "{error}"),
            Self::NotTessellated(error) => {
                write!(
                    f,
                    "the body did not tessellate at this display tolerance: {error}"
                )
            }
            Self::EmptyMesh => f.write_str(
                "the tessellation drew nothing — there is no picture to show and no \
                 bounds to frame a camera against",
            ),
            Self::MispairedIds { ids, patches } => write!(
                f,
                "a part offered {ids} patch ids for {patches} patches; a part carries \
                 either no ids at all or exactly one per patch"
            ),
            Self::BrokenPatchIndex { index, positions } => write!(
                f,
                "a face patch names vertex {index}, but the mesh's shared position \
                 table holds only {positions} positions"
            ),
        }
    }
}

impl core::error::Error for SceneError {}

/// A drawable scene: triangles with flat normals, plus what they came
/// from.
///
/// The buffers are `f32` because that is what a GPU consumes; every
/// decision above them was taken at `f64` (D2's precision boundary
/// sits at the display seam, not inside it).
#[derive(Clone, Debug)]
pub struct SceneMesh {
    /// One entry per triangle corner: three per triangle, never
    /// shared (see the flat-shading note in the module docs).
    positions: Vec<[f32; 3]>,
    /// The owning triangle's outward unit normal, repeated per
    /// corner.
    normals: Vec<[f32; 3]>,
    /// `0, 1, 2, …` — kept explicit so the draw call is an indexed
    /// one and a future welded build changes only this module.
    indices: Vec<u32>,
    /// The id of the patch each corner belongs to, parallel to
    /// [`SceneMesh::positions`]. `IdMap::NOTHING` for a corner drawn
    /// from a part that carries no ids.
    ///
    /// It is a per-corner attribute rather than a per-draw uniform
    /// because the whole picture is one draw call and the id has to
    /// vary within it — the id-buffer pass writes this straight out,
    /// and the shaded pass compares it against the highlight.
    ids: Vec<u32>,
    /// Per-corner display flags, parallel to [`SceneMesh::positions`]:
    /// [`SceneMesh::FLAG_PROBE`] for a corner drawn from a free-moved
    /// part, `0` otherwise.
    ///
    /// **This is the G3 visual-distinctness requirement as a value.**
    /// A probed placement must be distinguishable from a mated or
    /// authored one — an honesty rule, not a styling choice — so the
    /// distinction is carried in the scene the shader consumes, where a
    /// headless row can assert its presence, rather than decided at
    /// paint time where nothing can.
    flags: Vec<u32>,
    bounds: Aabb,
    stats: SceneStats,
}

/// One piece of the drawn picture: a tessellation, and the id its
/// patches are drawn under.
///
/// `ids` is either empty — nothing in this part is pickable, which is
/// the gathered product's case, whose patches belong to the aggregate
/// and to no node — or exactly one id per patch of `mesh`, in patch
/// order.
#[derive(Clone, Copy, Debug)]
pub struct ScenePart<'a> {
    /// The tessellation to draw.
    pub mesh: &'a Mesh,
    /// The id of each patch, or empty for an unpickable part.
    pub ids: &'a [u32],
    /// The free-move PROBE this part is drawn under, if any: a display
    /// frame composed over the tessellated placement (applied to the
    /// positions at build, in `f64`, before the `f32` cast), and — by
    /// its very presence — the G3 distinctness marker: every corner of
    /// a probed part carries [`SceneMesh::FLAG_PROBE`]. One field for
    /// both facts, so a part cannot be displaced without being marked
    /// or marked without being displaced.
    pub probe: Option<Frame>,
}

impl<'a> ScenePart<'a> {
    /// A part drawn where its tessellation puts it, unprobed.
    pub fn plain(mesh: &'a Mesh, ids: &'a [u32]) -> Self {
        Self {
            mesh,
            ids,
            probe: None,
        }
    }
}

/// What a scene cost, for the chrome to show.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SceneStats {
    /// Faces in the tessellated body.
    pub faces: usize,
    /// Triangles drawn.
    pub triangles: usize,
    /// The δ this scene was built at.
    pub display_delta: f64,
    /// How many drawn parts are free-move probes (drawn displaced and
    /// marked distinct). The scene-level summary of the per-corner
    /// [`SceneMesh::flags`].
    pub probe_parts: usize,
    /// How many drawn PATCHES carry [`SceneMesh::FLAG_FOCUS`] — the
    /// same kind of scene-level summary, for the marking the side
    /// panel's selection drives. It is the number a headless row
    /// asserts on: "selecting this feature marks these many faces" is
    /// checkable, and the colour it is drawn in is not.
    pub focus_patches: usize,
}

impl SceneMesh {
    /// The per-corner flag marking a free-move probe's corners.
    pub const FLAG_PROBE: u32 = 1;

    /// The per-corner flag marking the corners of what the side panel
    /// is currently showing (`crate::marks::focus`).
    ///
    /// A second BIT rather than a second field: the two facts are
    /// independent — a probed part can be the selected one — and the
    /// flags word already travels to the shader.
    pub const FLAG_FOCUS: u32 = 2;

    /// The empty picture: what a scene where EVERYTHING is hidden
    /// draws. Zero triangles, legally — an honest blank viewport, not
    /// an error — with `bounds` carried from the geometry that exists
    /// but is not drawn, so a camera still has something real to frame
    /// against. Distinct from [`SceneError::EmptyMesh`], which remains
    /// the refusal for a document that has nothing to draw at all.
    pub fn empty(bounds: Aabb, delta: DisplayTolerance) -> Self {
        Self {
            positions: Vec::new(),
            normals: Vec::new(),
            indices: Vec::new(),
            ids: Vec::new(),
            flags: Vec::new(),
            bounds,
            stats: SceneStats {
                faces: 0,
                triangles: 0,
                display_delta: delta.get(),
                probe_parts: 0,
                focus_patches: 0,
            },
        }
    }

    /// **The picture of a document that denotes no geometry at all**
    /// — an empty recipe, or one holding only datums and profiles.
    ///
    /// [`SceneMesh::empty`]'s sibling, and the distinction between
    /// them is where the extent comes from: that one is drawn from
    /// geometry that EXISTS and is hidden, so it carries that
    /// geometry's box; this one has no geometry to take a box from,
    /// and says so with a degenerate one at the world origin. A
    /// camera asked to frame it refuses with
    /// [`crate::camera::CameraError::DegenerateScene`], which is the
    /// honest answer — inventing a scale for a document with no
    /// extent would put the user somewhere no fact chose.
    ///
    /// This is a legal outcome, not a refusal.
    /// [`SceneError::EmptyMesh`] stays what it always was: parts that
    /// exist and tessellated to nothing, which is a fault in the
    /// tessellation rather than a document with nothing in it.
    pub fn nothing(delta: DisplayTolerance) -> Self {
        // `from_points` answers `None` only for an EMPTY iterator, and
        // this one holds a point — but the absence is handled rather
        // than asserted away, because a panic in the picture of an
        // empty document would be the loudest possible answer to the
        // quietest possible state. The poison box is the fallback with
        // the same meaning the degenerate one has here: nothing a
        // camera can frame.
        let nowhere =
            Aabb::from_points([Point3::new(0.0_f64, 0.0, 0.0)]).unwrap_or_else(Aabb::poison);
        Self::empty(nowhere, delta)
    }

    /// Build a drawable scene from a tessellated body.
    ///
    /// Winding comes from `mesh::FacePatch`'s documented contract —
    /// counterclockwise seen from outside the material — so the
    /// triangle normal `(b − a) × (c − a)` points out of the solid
    /// and no per-consumer sense correction is applied (the contract
    /// says explicitly not to).
    ///
    /// # Errors
    ///
    /// [`SceneError::EmptyMesh`] when the tessellation carries no
    /// triangle, or no finite bounding box;
    /// [`SceneError::BrokenPatchIndex`] when a patch names a vertex
    /// the shared position table does not have.
    pub fn build(mesh: &Mesh, delta: DisplayTolerance) -> Result<Self, SceneError> {
        Self::build_parts(&[ScenePart::plain(mesh, &[])], delta)
    }

    /// Build a scene from several parts, concatenated in the order
    /// given.
    ///
    /// **This is the shape the viewport draws**: one part per
    /// (node, output body), each the tessellation its pick index was
    /// built from, so the picture and the pick answer come from one
    /// tessellation rather than two that happen to agree.
    ///
    /// # Errors
    ///
    /// As [`SceneMesh::build`], plus [`SceneError::MispairedIds`] for
    /// a part whose id list is neither empty nor one per patch.
    pub fn build_parts(
        parts: &[ScenePart<'_>],
        delta: DisplayTolerance,
    ) -> Result<Self, SceneError> {
        Self::build_parts_focused(parts, delta, &BTreeSet::new())
    }

    /// [`SceneMesh::build_parts`], marking the patches whose id is in
    /// `focus` with [`SceneMesh::FLAG_FOCUS`].
    ///
    /// **Why the marking is a per-corner attribute and not a shader
    /// uniform**, which is how the selected and hovered patches are
    /// marked: those are one patch each, so an id fits in a uniform
    /// slot; a focus is a SET, of no bounded size, and the only place a
    /// set of that shape can be tested per fragment without new GPU
    /// plumbing is the vertex data the picture is already carrying.
    ///
    /// The cost that buys is a scene rebuild when the selection moves.
    /// It is a real cost and it is a small one: this function walks
    /// tessellations that already exist — the pick index's, built once
    /// per evaluation — and copies vertex arrays. The free-move probe
    /// already rebuilds the scene on every frame of a drag through the
    /// same path, so a rebuild per selection CLICK is strictly cheaper
    /// than something that shipped.
    ///
    /// An id in `focus` that this scene does not draw is ignored rather
    /// than refused: a hidden instance's ids are legitimately absent,
    /// and a focus is a request to mark what is there.
    ///
    /// # Errors
    ///
    /// As [`SceneMesh::build_parts`].
    pub fn build_parts_focused(
        parts: &[ScenePart<'_>],
        delta: DisplayTolerance,
        focus: &BTreeSet<u32>,
    ) -> Result<Self, SceneError> {
        for part in parts {
            if !part.ids.is_empty() && part.ids.len() != part.mesh.patches.len() {
                return Err(SceneError::MispairedIds {
                    ids: part.ids.len(),
                    patches: part.mesh.patches.len(),
                });
            }
        }
        let triangles: usize = parts
            .iter()
            .flat_map(|part| part.mesh.patches.iter())
            .map(|p| p.triangles.len())
            .sum();
        if triangles == 0 {
            return Err(SceneError::EmptyMesh);
        }
        let mut positions = Vec::with_capacity(triangles * 3);
        let mut normals = Vec::with_capacity(triangles * 3);
        let mut ids = Vec::with_capacity(triangles * 3);
        let mut flags = Vec::with_capacity(triangles * 3);
        let mut faces = 0usize;
        let mut probe_parts = 0usize;
        let mut focus_patches = 0usize;
        for part in parts {
            let mesh = part.mesh;
            faces += mesh.patches.len();
            // The probe's display map, applied in f64 BEFORE the f32
            // cast: the displaced picture takes the same one rounding
            // step an undisplaced one does.
            let map: Option<Affine3<f64>> = part.probe.map(|frame| frame.affine());
            let flag = if part.probe.is_some() {
                probe_parts += 1;
                Self::FLAG_PROBE
            } else {
                0
            };
            for (index, patch) in mesh.patches.iter().enumerate() {
                // `IdMap::NOTHING` for a part that carries no ids —
                // the constant, not the literal it happens to be.
                let id = part
                    .ids
                    .get(index)
                    .copied()
                    .unwrap_or(crate::pickindex::IdMap::NOTHING);
                // The probe flag is the PART's; the focus flag is the
                // PATCH's, which is why it is computed here rather than
                // beside `flag` above.
                let flags_word = if id != crate::pickindex::IdMap::NOTHING && focus.contains(&id) {
                    focus_patches += 1;
                    flag | Self::FLAG_FOCUS
                } else {
                    flag
                };
                for corners in &patch.triangles {
                    let Some(mut corner_points) = fetch(&mesh.positions, corners) else {
                        // A patch index outside the shared position
                        // table is a broken mesh: refuse the whole
                        // scene, naming the index, rather than
                        // dropping a triangle.
                        return Err(SceneError::BrokenPatchIndex {
                            index: corners
                                .iter()
                                .copied()
                                .find(|i| *i as usize >= mesh.positions.len())
                                .unwrap_or_default(),
                            positions: mesh.positions.len(),
                        });
                    };
                    if let Some(map) = &map {
                        for p in &mut corner_points {
                            *p = map.transform_point(*p);
                        }
                    }
                    // Computed from the (possibly displaced) corners,
                    // so a probed part is lit by where it is drawn.
                    let normal = triangle_normal(&corner_points);
                    for p in corner_points {
                        positions.push([p.x as f32, p.y as f32, p.z as f32]);
                        normals.push(normal);
                        ids.push(id);
                        flags.push(flags_word);
                    }
                }
            }
        }
        let indices = (0..positions.len() as u32).collect();
        let bounds = Aabb::from_points(parts.iter().flat_map(|part| {
            let map: Option<Affine3<f64>> = part.probe.map(|frame| frame.affine());
            part.mesh
                .positions
                .iter()
                .map(move |p| map.as_ref().map_or(*p, |m| m.transform_point(*p)))
        }))
        .ok_or(SceneError::EmptyMesh)?;
        Ok(Self {
            positions,
            normals,
            indices,
            ids,
            flags,
            bounds,
            stats: SceneStats {
                faces,
                triangles,
                display_delta: delta.get(),
                probe_parts,
                focus_patches,
            },
        })
    }

    /// Triangle-corner positions, one per index.
    pub fn positions(&self) -> &[[f32; 3]] {
        &self.positions
    }

    /// Per-corner outward normals, parallel to
    /// [`SceneMesh::positions`].
    pub fn normals(&self) -> &[[f32; 3]] {
        &self.normals
    }

    /// The index buffer.
    pub fn indices(&self) -> &[u32] {
        &self.indices
    }

    /// The per-corner patch ids, parallel to
    /// [`SceneMesh::positions`].
    pub fn ids(&self) -> &[u32] {
        &self.ids
    }

    /// The per-corner display flags ([`SceneMesh::FLAG_PROBE`]),
    /// parallel to [`SceneMesh::positions`] — the distinctness value
    /// the shader paints and the headless rows assert.
    pub fn flags(&self) -> &[u32] {
        &self.flags
    }

    /// A bounding box of the scene, in world units — what a camera
    /// frames against.
    ///
    /// Taken over the tessellation's whole shared position table, not
    /// over the corners actually emitted into
    /// [`SceneMesh::positions`]. The two coincide whenever every mesh
    /// vertex is used by some patch, which is the tessellator's normal
    /// output; when they do not, this is the **superset**, so a camera
    /// framed on it still contains everything drawn. That is the safe
    /// direction, and it is stated rather than claimed as identity.
    pub fn bounds(&self) -> Aabb {
        self.bounds
    }

    /// What this scene cost.
    pub fn stats(&self) -> SceneStats {
        self.stats
    }
}

/// The spike's plate, in canonical metres: 60 × 40 × 8 mm.
///
/// **The one home for these numbers.** They are the plate's identity,
/// and a test fixture that restates them is a copy that goes on
/// testing a box the scene no longer has the day the plate changes.
/// [`plate_with_hole`] authors from these, and every consumer that
/// needs the plate's shape without evaluating it — a camera fixture,
/// an expected-bounds assertion — reads them here.
pub const PLATE_EXTENT: [f64; 3] = [0.060, 0.040, 0.008];

/// The radius of [`plate_with_hole`]'s through hole, canonical metres
/// (⌀24 mm). Same reason as [`PLATE_EXTENT`].
pub const PLATE_HOLE_RADIUS: f64 = 0.012;

/// The plate's box, without evaluating anything: the corner at the
/// origin and [`PLATE_EXTENT`] away from it.
///
/// The assembly [`PLATE_EXTENT`]'s own note asks for, given a home
/// rather than restated — "a camera fixture, an expected-bounds
/// assertion" is a six-line struct literal, and it had been hand-copied
/// into every suite that wanted a box to frame a camera on. One home
/// for the numbers and a different one for the box they make is half
/// the rule.
#[must_use]
pub fn plate_bounds() -> Aabb {
    let [width, depth, thickness] = PLATE_EXTENT;
    Aabb {
        min_x: 0.0,
        min_y: 0.0,
        min_z: 0.0,
        max_x: width,
        max_y: depth,
        max_z: thickness,
    }
}

/// The spike's document: a plate with a through hole.
///
/// Authored through the ordinary document doors — one profile node
/// carrying an outer rectangle and an inner circle, one extrude over
/// it. The hole is a profile ring rather than a boolean on purpose:
/// it puts a cylindrical face and a ring-triangulated planar face in
/// the very first frame, which is what makes a δ change visible at
/// all.
///
/// Dimensions come from [`PLATE_EXTENT`] and [`PLATE_HOLE_RADIUS`].
///
/// # A library finding, recorded at the site
///
/// Two ways to give a length live ten lines apart below:
/// `LoopProgram::polygon` takes bare `(f64, f64)` metres, while
/// `LoopProgram::Circle` takes `Expr::literal(x, Dimension::Length)`.
/// Both are canonical metres and both are correct; the asymmetry is
/// the profile-program vocabulary's, not this scene's, and a user
/// authoring their first ring meets it immediately. Recorded per
/// `memories/demo-purpose.md` (awkwardness met while authoring is a
/// library finding, never quietly worked around) — this unit does not
/// fix it, because widening `polygon` to expressions is a
/// `LoopProgram` decision with its own consumers.
///
/// # Errors
///
/// Never, as written: the expressions are literal lengths and the
/// node graph is well formed. The signature carries the `Result`
/// because every door it calls does — a scene that silently swallowed
/// an `EditError` would be a worse example than one that reports it.
pub fn plate_with_hole(tol: Tol) -> Result<(Doc<ProfileProgram>, RecipeNodeId), SceneDocError> {
    let [width, depth, thickness] = PLATE_EXTENT;
    let outline = LoopProgram::polygon([(0.0, 0.0), (width, 0.0), (width, depth), (0.0, depth)])
        .map_err(SceneDocError::Dimension)?;
    // The hole sits on the plate's centre.
    let hole = LoopProgram::Circle {
        centre: [length(width * 0.5)?, length(depth * 0.5)?],
        radius: length(PLATE_HOLE_RADIUS)?,
    };
    let doc: Doc<ProfileProgram> = Doc::empty_derived("gui-0-plate", tol);
    // The frame the plate is drawn on — the world xy frame, spelled as
    // a node because that is what a profile names now. It is the
    // document's first node, so the plate reads in the feature tree
    // the way it was authored: a frame, then a sketch on it.
    let (doc, frame) = insert(
        doc,
        Node::Datum(Datum::Frame {
            origin: [length(0.0)?, length(0.0)?, length(0.0)?],
            u: [scalar(1.0)?, scalar(0.0)?, scalar(0.0)?],
            v: [scalar(0.0)?, scalar(1.0)?, scalar(0.0)?],
        }),
        tol,
    )?;
    let profile = ProfileProgram {
        plane: frame,
        loops: vec![outline, hole],
    };
    let (doc, profile_node) = insert(doc, Node::Profile(profile), tol)?;
    let (doc, extrude) = insert(
        doc,
        Node::Extrude {
            profile: profile_node,
            distance: length(thickness)?,
        },
        tol,
    )?;
    Ok((doc, extrude))
}

/// A refusal while authoring the spike's document.
#[derive(Debug)]
pub enum SceneDocError {
    /// A literal was not a usable length.
    Dimension(pncad::document::DimensionError),
    /// An edit was refused.
    Edit(pncad::document::EditError),
    /// An insert did not mint a node id — an `apply` postcondition,
    /// carried as a value rather than asserted away.
    NoNodeMinted,
}

impl core::fmt::Display for SceneDocError {
    /// Both payload arms forward to the document layer's own
    /// `Display`; only the postcondition arm is this layer's sentence.
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Dimension(error) => write!(f, "{error}"),
            Self::Edit(error) => write!(f, "{error}"),
            Self::NoNodeMinted => f.write_str(
                "an insert minted no node id, so the authored node cannot be referred \
                 to",
            ),
        }
    }
}

impl core::error::Error for SceneDocError {}

/// Evaluate a document and gather its product body.
///
/// # Errors
///
/// [`SceneError::NoProduct`] for every way the roots fail to gather.
pub fn product_body(doc: &Doc<ProfileProgram>, tol: Tol) -> Result<Body<f64>, SceneError> {
    let cancel = CancelToken::new();
    let evaluation = evaluate::<f64>(doc, None, &cancel, &EvalOptions::default(), tol);
    product(doc, &evaluation, tol).map_err(SceneError::NoProduct)
}

/// **Gather the product of a pair the landing did not keep one for.**
///
/// The expensive door, and the only one in this module that gathers a
/// product for an evaluation someone else ran. It exists for exactly
/// one case: an assembly whose A5 gate REFUSED consumed the body the
/// landing would otherwise have kept
/// ([`crate::session::DocSession::landed_body`]), so a consumer that
/// needs one there has to pay for it. Naming that door rather than
/// hiding the gather inside a getter is what keeps the cost at the
/// call site, where a reader meets it.
///
/// # Errors
///
/// [`SceneError::NoProduct`] for every way the roots fail to gather.
pub fn product_of_evaluation(
    doc: &Doc<ProfileProgram>,
    evaluation: &pncad::document::Evaluation<f64>,
    tol: Tol,
) -> Result<Body<f64>, SceneError> {
    product(doc, evaluation, tol).map_err(SceneError::NoProduct)
}

/// The scene of a product SOMEONE ELSE gathered.
///
/// **Takes the aggregate, never the pair that would produce one.** A
/// gather is the expensive step on this path, and a landing has
/// already paid for one ([`crate::session::DocSession::landed_body`]);
/// a door that took `(doc, evaluation)` here would gather the same
/// product a second time. Not per frame — the drawn picture is built
/// by [`crate::pickindex::PickIndex`], per root, and never comes through
/// here — but once for every caller that asks, which is the shape the
/// landing already paid to avoid. [`scene_of`] is this function with a gather of
/// its own, kept for callers that have no seam — the two share every
/// step after the body exists, so a document drawn from a background
/// run and one drawn inline cannot differ.
///
/// # Errors
///
/// Every arm of [`SceneError`] except the δ and gather ones.
pub fn scene_of_body(
    body: &Body<f64>,
    delta: DisplayTolerance,
    tol: Tol,
) -> Result<SceneMesh, SceneError> {
    let mesh = tessellate(body, delta.get(), tol).map_err(SceneError::NotTessellated)?;
    SceneMesh::build(&mesh, delta)
}

/// **The picture's triangle budget**: the most a scene may carry
/// before the viewer draws it at a coarser δ than it was asked for.
///
/// # Why there is a budget at all
///
/// δ is a chord tolerance in metres, and nothing about an absolute
/// length knows how big a model is or how curved: the same 0.1 mm the
/// application starts at is a small picture of the startup plate and
/// a 1.6·10⁵-triangle picture of the tour's `hollowring`
/// (a torus of R = 0.30 m), and a body a few times larger or a δ a
/// decade finer asks for millions — seconds of tessellation and index
/// build with the window frozen, still showing the previous document,
/// which reads as "Open does nothing". A budget is what stops an
/// absolute δ from asking for a picture nobody can wait for.
///
/// # Why one million
///
/// Two independent anchors, and they agree, which is the argument:
///
/// - **The screen.** A viewport pane on a 1280×800 window is about
///   0.65 Mpx, and roughly half of a closed body's triangles face
///   away. One million is therefore already about one front-facing
///   triangle per pixel for a body filling the pane: past it the
///   tessellation is finer than the display can resolve, and the
///   detail is paid for and thrown away.
/// - **The corpus, by eye.** Measured on the tour's own scenes, the
///   fillet corners of `diefillet` read clean at δ ≈ 0.2–0.4 mm — the
///   δ this budget lands a curved gallery document on when it binds —
///   and visibly band one doubling coarser, so it is also the first
///   budget that keeps the demo documents looking right when it does
///   bind. It does not bind either gallery document at the starting
///   0.1 mm (`tests/display_budget.rs` holds the ring's side of that,
///   and asks the budget's own rows at 0.01 mm, where it does).
///
/// **The consequence worth stating**: if this number ever has to be
/// RAISED to make something look right, the fault is upstream in the
/// sizing, not here — a budget cannot buy detail the tessellator is
/// spending elsewhere. The ring at 0.1 mm is ~1.6·10⁵ triangles, which
/// is `mesh::sizing::torus_grid_steps`' doubly-curved chord bound spent
/// with no slack in its constant (2.9× the per-direction sagitta, and
/// that factor is proved necessary, not chosen); what remains is
/// TESS-BUDGET's question, and this constant is a safety net under it,
/// never its answer.
pub const TRIANGLE_BUDGET: usize = 1_000_000;

/// How much coarser than the δ it prices a cost probe runs.
///
/// Eight doublings-worth of cost is ~1/8 of the priced δ's
/// tessellation, which is what makes a probe affordable; and it is
/// close enough to the target that the 1/δ law below still holds
/// tightly (it softens at genuinely coarse δ, where planar faces have
/// stopped subdividing and only the curved ones still respond).
///
/// **What it is coarser THAN is the δ being priced, not the δ being
/// asked for.** Those are the same number only while the budget does
/// not move δ, and sizing a probe off the request is what let the
/// probe grow past the picture it sizes: a request the budget has to
/// coarsen by more than this factor is a request whose probe costs
/// more than the answer's whole picture ([`fit_delta`] says what
/// replaced that).
const PROBE_FACTOR: f64 = 8.0;

/// The δ the scale probe runs at: coarser than any body this viewer
/// opens, so nothing subdivides and the tessellation is the body's
/// FLOOR — the cheapest one it has at any δ, and the one whose points
/// say how big the body is.
///
/// A starting point, not a bound. A body larger than this still
/// tessellates here and the mesh is still no larger than the picture
/// (the count is non-increasing in δ); all that is lost is the
/// floor's tightness, and the ladder in [`fit_delta`] walks down from
/// wherever it starts.
const SCALE_PROBE_DELTA: f64 = 1.0e9;

/// What [`fit_delta`] decided, and why — a value, so the chrome can
/// say it and a row can assert it.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FittedDelta {
    /// The δ to open at. Equal to [`FittedDelta::requested`] unless
    /// the budget moved it.
    pub delta: DisplayTolerance,
    /// The δ that was asked for.
    pub requested: DisplayTolerance,
    /// The triangle count predicted at [`FittedDelta::delta`] — a
    /// prediction, not a measurement of the picture that gets built
    /// (`fit_delta` says why it is not verified), and the measured
    /// count itself when the ladder found the body flat
    /// ([`ProbeStop::Flat`]).
    ///
    /// **The error is two-sided, and small.** The big term is one-way:
    /// the 1/δ law describes the CURVED faces, planar ones stop
    /// subdividing and cost the same at every δ, so extrapolating them
    /// from a coarse rung OVER-counts — which is why a flat reading is
    /// taken as the count rather than run through the law. The other
    /// term is not one-way: the grid's per-direction step counts are
    /// `ceil`ed, so `C = triangles · δ` wobbles by a fraction of a
    /// percent with δ (`tube_ring` reads 64.296 at 4·10⁻⁴ and 64.106 at
    /// 8·10⁻⁴), and a reading off the low side of that wobble predicts
    /// a δ slightly finer than the budget wanted. The picture can
    /// therefore land just OVER `TRIANGLE_BUDGET`: measured at 1 002
    /// 536 and 1 001 088 triangles on two corpus rows, +0.25% at the
    /// worst, and `tests/display_budget.rs` holds the drawn count to
    /// the budget with a margin rather than as a cap. Whether the
    /// budget should be a cap that absorbs that error is
    /// `work/view/the-budgets-predicted-count-is-not-always-an-over-count.md`.
    pub predicted: usize,
    /// What the requested δ was predicted to cost, when the budget
    /// moved δ; `None` when the request was affordable and nothing
    /// was changed.
    pub requested_cost: Option<usize>,
    /// What finding this δ COST: triangles tessellated across every
    /// rung of the ladder.
    ///
    /// A sum, so it can exceed the picture — an all-planar body pays
    /// two rungs of the same small mesh. The bound that holds per
    /// probe is [`FittedDelta::largest_probe`].
    pub probe_triangles: usize,
    /// The largest single rung, which is the number the fit's
    /// invariant is about: **never more than the picture at
    /// [`FittedDelta::delta`]**.
    pub largest_probe: usize,
    /// Which rule ended the ladder.
    pub stop: ProbeStop,
}

impl FittedDelta {
    /// The verdict for a δ the budget did not have to move — the
    /// fallback when the fit itself refuses, and the shape every
    /// document under the budget lands on.
    pub fn as_requested(requested: DisplayTolerance) -> Self {
        Self {
            delta: requested,
            requested,
            predicted: 0,
            requested_cost: None,
            probe_triangles: 0,
            largest_probe: 0,
            stop: ProbeStop::Unprobed,
        }
    }

    /// The sentence the chrome shows, or `None` when the δ asked for
    /// was affordable and there is nothing to report.
    ///
    /// It ends by saying the budget is not a cap, because that is the
    /// question a reader has the moment they see a δ they did not
    /// choose, and a chosen default that read as a clamp would be
    /// worse than no default at all.
    ///
    /// **Both δ are rendered, not formatted.** A sentence whose whole
    /// job is to name the two δ in play is the last place a number may
    /// read as one δ cannot be, and a fixed `{:.3}` over millimetres
    /// carried both as `0.000` below half a micrometre
    /// ([`DisplayTolerance::render_mm`]). What that costs is that the
    /// sentence is as wide as the δ are awkward — a budget δ reads
    /// `0.0003746` where it used to read `0.000`, and a δ of a few
    /// picometres reads `4.000e-9` — which is the right trade for a
    /// status line, where a long true number is readable and a short
    /// false one is not.
    pub fn wording(&self) -> Option<String> {
        let requested = self.requested_cost?;
        let opened = self.delta.render_mm();
        let asked = self.requested.render_mm();
        Some(format!(
            "opened at δ = {opened} mm: {asked} mm needs about {requested} triangles, over the {TRIANGLE_BUDGET} budget. A finer δ typed in the View pane is still honoured — this is a starting point, not a cap"
        ))
    }
}

/// Why the ladder in [`fit_delta`] stopped, and so where the reading
/// it answered from came from.
///
/// A value rather than a comment because the cost argument turns on
/// it: [`ProbeStop::AtTheRequest`] is the rung a single probe would
/// have read, [`ProbeStop::Flat`] is the reading the 1/δ law is not
/// allowed to speak for, and a row can ask which one answered.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ProbeStop {
    /// No ladder ran: the value was built by
    /// [`FittedDelta::as_requested`], not by a fit.
    Unprobed,
    /// The last rung was [`PROBE_FACTOR`] × the request — the finest
    /// rung there is, and the one a fit with no ladder reads.
    AtTheRequest,
    /// A finer rung would not have moved the reading enough to pay
    /// for itself.
    Converged,
    /// Two rungs in a row counted the same at the request's own rung:
    /// the body does not subdivide across that span, so the count IS
    /// the prediction and the 1/δ law says nothing about it.
    Flat,
    /// A rung refused to tessellate. The reading is the rung before
    /// it, or — when the refusal was the first probe — the rung that
    /// prices the request.
    Refused,
}

/// Choose the δ to OPEN a document at: the requested one, or the
/// finest coarser one predicted to fit [`TRIANGLE_BUDGET`].
///
/// **A default, not a clamp.** The caller applies this once per
/// document that arrives
/// ([`crate::app::ViewerApp::fit_delta_on_scene`]); from there δ is
/// whatever the user types in the View pane, however fine, and nothing
/// re-reads it. A budget that bound every rebuild would disable that
/// field on exactly the documents someone would want it for.
///
/// # The method: probe coarse, solve, refine
///
/// A ladder that starts at the request — try δ, halve until it fits —
/// pays for the tessellation it then throws away, and the one it
/// throws away is the expensive one. So this ladder runs the other
/// way: it starts at the body's own extent, where nothing subdivides,
/// and each rung is a δ the rung above it has already PRICED at
/// `TRIANGLE_BUDGET / PROBE_FACTOR` triangles. It descends only while
/// a finer rung would buy a materially better reading of `C`, and it
/// stops at [`PROBE_FACTOR`] × the request, which is the rung that
/// prices the request itself.
///
/// The law it solves is `triangles ≈ C/δ`, which is what a chord-sized
/// grid over a fixed surface gives: each direction is cut ∝ 1/√δ, so
/// their product is ∝ 1/δ. It is not an assumption — measured on the
/// tour's `hollowring` the doubling ratios are 1.999, 1.996, 1.997,
/// 1.999 across four doublings, and on `diefillet` 1.99, 1.97, 1.98,
/// 1.94. So `C` is read off a rung and δ* = C / budget.
///
/// **A body that does not subdivide is not described by that law at
/// all**, and the ladder says so rather than extrapolating: two rungs
/// that count the same are a body whose count did not move across a
/// factor of at least two in δ, so the count itself is the prediction
/// ([`ProbeStop::Flat`], and [`FittedDelta::predicted`] is then the
/// measured count rather than `C/δ`). What that rule takes on trust is
/// that a body flat across one rung step is flat below it too — true
/// for every corpus document (`tests/display_budget.rs` asserts the
/// flat rows' prediction against the drawn picture, exactly) and
/// stated as a residue in
/// `work/view/a-flat-rung-pair-is-read-as-flat-below-it.md`.
///
/// # What it costs: no PROBE is larger than the picture it sizes
///
/// **Per probe, and that is the invariant.** The tessellator's
/// triangle count is non-increasing in δ, and no rung ever runs at a δ
/// finer than the committed one, so no single rung tessellates more
/// triangles than the picture it sizes
/// ([`FittedDelta::largest_probe`] against the drawn count, on every
/// row of `tests/display_budget.rs`'s table). Each rung is also placed
/// at `PROBE_FACTOR · C / TRIANGLE_BUDGET`, whose predicted cost is
/// `TRIANGLE_BUDGET / PROBE_FACTOR` — a bound the actual count meets
/// up to the law's own error, which is two-sided and small (see
/// [`FittedDelta::predicted`]).
///
/// **What the whole ladder costs is a sum, not that bound**
/// ([`FittedDelta::probe_triangles`]): rungs × what a rung costs. Two
/// rungs answer most documents and the total is 1.1–1.4 × the largest
/// of them; a body whose count does not respond to δ pays every rung
/// at the same count, which is what the flat rule above is for. The
/// largest total measured over the corpus at the application's δ and a
/// decade finer is 177 654 triangles.
///
/// **On a document the budget does not bind** the last rung is a probe
/// at [`PROBE_FACTOR`] × the request, so the δ committed and the cost
/// predicted for it are read off exactly the mesh a single probe there
/// reads them off; what the rungs above it add is 12–40% more
/// triangles on the curved gallery documents (26 058 → 29 824 on the
/// tour's die at 0.1 mm). An all-planar body pays two probes of a mesh
/// that never subdivides: `checks` and `heatsink` are 24 and 72
/// triangles at every δ.
///
/// # The count is the picture's, not an estimate of it
///
/// The probe tessellates the GATHERED product, while the picture is
/// built per root by `crate::pickindex::PickIndex`. Those are the same
/// number: measured across four δ on both multi-root gallery
/// documents, gathered and per-root triangle counts agree exactly
/// (0.000%), because the graft moves solids into one body without
/// re-cutting their faces.
///
/// # What it is handed
///
/// **The gathered body, not the pair it came from.** The landing this
/// fit follows already gathered the product once
/// ([`crate::session::DocSession::land`]), and a fit that gathered
/// again would pay a whole second gather on the one path a user reads
/// as "how long Open takes". Measured on a 165-root, 990-face
/// document (dev profile, this lane): 87 ms to gather, against 2.4 ms
/// to clone the body that gather produced. What that measurement
/// decides, and why it carries no guard, is stated where the decision
/// is (`session`'s `LandedRun::body`).
///
/// # Errors
///
/// [`SceneError::NotTessellated`] only when BOTH the scale probe and
/// the rung that prices the request refuse — a refusal anywhere else
/// falls back to a reading already taken, because a document that
/// opens un-budgeted is the freeze this exists to prevent.
/// [`SceneError::InvalidDisplayTolerance`] if the solved δ is not a
/// usable one.
pub fn fit_delta(
    body: &Body<f64>,
    requested: DisplayTolerance,
    tol: Tol,
) -> Result<FittedDelta, SceneError> {
    fit_from_probes(requested, |delta| {
        let mesh = tessellate(body, delta, tol)?;
        Ok(Probe {
            triangles: triangle_count(&mesh),
            extent: extent(&mesh),
        })
    })
}

/// What one rung of the ladder measured.
struct Probe {
    /// Its triangle count.
    triangles: usize,
    /// How far apart the mesh's points are — read only from the scale
    /// probe, where it is the body's own extent.
    extent: f64,
}

/// [`fit_delta`] over a probe door, which is what the ladder's own
/// rows drive: the policy is the same whatever tessellates.
fn fit_from_probes(
    requested: DisplayTolerance,
    mut probe: impl FnMut(f64) -> Result<Probe, TessellateError>,
) -> Result<FittedDelta, SceneError> {
    #[allow(clippy::cast_precision_loss)]
    let budget = TRIANGLE_BUDGET as f64;
    // The finest δ any rung runs at: PROBE_FACTOR coarser than the
    // request, which is the rung that prices the request itself.
    // Nothing below it is ever tessellated, because nothing below it
    // is ever needed — a request already inside the budget is the
    // answer, and one the budget moves is answered from a COARSER
    // rung.
    let finest = requested.scaled(PROBE_FACTOR)?;
    // The scale probe: the body's floor mesh, and the extent its
    // points span. Nothing subdivides anywhere between that extent and
    // this δ, so this count is the count at the extent too, and the
    // ladder starts there with a rung it has already paid for.
    //
    // A refusal here is answered by the rung that prices the request —
    // where a fit with no ladder would have started and ended. Only if
    // THAT refuses too is there no answer to give.
    let (mut triangles, mut probe_delta) = match probe(SCALE_PROBE_DELTA) {
        Ok(scale) => (scale.triangles, scale.extent.max(finest.get())),
        Err(_) => {
            let fallback = probe(finest.get()).map_err(SceneError::NotTessellated)?;
            return answer(
                requested,
                Reading {
                    triangles: fallback.triangles,
                    delta: finest.get(),
                    flat: false,
                    stop: ProbeStop::Refused,
                    largest: fallback.triangles,
                    total: fallback.triangles,
                },
            );
        }
    };
    let mut largest = triangles;
    let mut total = triangles;
    let mut flat = false;
    let stop = loop {
        // C, in triangle·metres. A body that tessellates to nothing at
        // this rung has no curvature to spend on, so it costs the same
        // at every δ and the request stands.
        #[allow(clippy::cast_precision_loss)]
        let constant = triangles as f64 * probe_delta;
        // The rung that prices the δ this one solves for — or the one
        // that prices the request, when the request is the answer and
        // there is nothing coarser to find. Its predicted cost is
        // TRIANGLE_BUDGET / PROBE_FACTOR by construction, which is
        // what makes descending to it affordable.
        //
        // A count that did not move has no such rung to offer: the law
        // is refuted for this body over this span, so the ladder goes
        // straight to the request's own rung rather than crawling down
        // in steps the law sizes from a constant it just disproved.
        let next = if flat {
            finest.get()
        } else {
            (PROBE_FACTOR * constant / budget).max(finest.get())
        };
        if next >= probe_delta {
            break if probe_delta <= finest.get() {
                if flat {
                    ProbeStop::Flat
                } else {
                    ProbeStop::AtTheRequest
                }
            } else {
                ProbeStop::Converged
            };
        }
        // Descend for a rung that pays for itself — a doubling finer,
        // or the finest probe there is, where an affordable request
        // gets its cost read off the very mesh a single probe would
        // have read it off. This terminates: `next` is bounded below
        // by `finest` and every step at least halves the δ.
        if !flat && next > probe_delta * 0.5 && next > finest.get() {
            break ProbeStop::Converged;
        }
        let Ok(rung) = probe(next) else {
            break ProbeStop::Refused;
        };
        flat = rung.triangles == triangles;
        triangles = rung.triangles;
        probe_delta = next;
        largest = largest.max(triangles);
        total += triangles;
    };
    answer(
        requested,
        Reading {
            triangles,
            delta: probe_delta,
            flat,
            stop,
            largest,
            total,
        },
    )
}

/// The finest rung's measurement, and what the ladder made of it.
struct Reading {
    triangles: usize,
    delta: f64,
    flat: bool,
    stop: ProbeStop,
    largest: usize,
    total: usize,
}

/// The verdict a reading implies.
fn answer(requested: DisplayTolerance, reading: Reading) -> Result<FittedDelta, SceneError> {
    #[allow(clippy::cast_precision_loss)]
    let budget = TRIANGLE_BUDGET as f64;
    #[allow(clippy::cast_precision_loss)]
    let constant = reading.triangles as f64 * reading.delta;
    // A count that does not respond to δ gives the budget no lever:
    // every δ draws the same picture, so the request stands and the
    // prediction is the count itself rather than the law's `C/δ`.
    if reading.flat {
        return Ok(FittedDelta {
            delta: requested,
            requested,
            predicted: reading.triangles,
            requested_cost: None,
            probe_triangles: reading.total,
            largest_probe: reading.largest,
            stop: reading.stop,
        });
    }
    let solved = constant / budget;
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let at_request = (constant / requested.get()) as usize;
    if solved <= requested.get() {
        return Ok(FittedDelta {
            delta: requested,
            requested,
            predicted: at_request,
            requested_cost: None,
            probe_triangles: reading.total,
            largest_probe: reading.largest,
            stop: reading.stop,
        });
    }
    Ok(FittedDelta {
        delta: DisplayTolerance::new(solved)?,
        requested,
        predicted: TRIANGLE_BUDGET,
        requested_cost: Some(at_request),
        probe_triangles: reading.total,
        largest_probe: reading.largest,
        stop: reading.stop,
    })
}

/// The triangles a tessellation carries.
fn triangle_count(mesh: &Mesh) -> usize {
    mesh.patches.iter().map(|patch| patch.triangles.len()).sum()
}

/// How far apart a mesh's points are: the diagonal of their bounding
/// box, and so a δ at which the body that produced them does not
/// subdivide at all.
///
/// Zero for a mesh with no points, and for one whose points are not a
/// finite box — a caller that starts a ladder here reads that as "no
/// rung coarser than the finest", which is the one place the answer is
/// a δ rather than a refusal.
fn extent(mesh: &Mesh) -> f64 {
    let Some(box_) = Aabb::from_points(mesh.positions.iter().copied()) else {
        return 0.0;
    };
    let diagonal = Vec3::new(
        box_.max_x - box_.min_x,
        box_.max_y - box_.min_y,
        box_.max_z - box_.min_z,
    )
    .norm();
    if diagonal.is_finite() { diagonal } else { 0.0 }
}

/// The whole path: document → evaluated product → tessellation at δ →
/// drawable scene.
///
/// # Errors
///
/// Every arm of [`SceneError`].
pub fn scene_of(
    doc: &Doc<ProfileProgram>,
    delta: DisplayTolerance,
    tol: Tol,
) -> Result<SceneMesh, SceneError> {
    let body = product_body(doc, tol)?;
    scene_of_body(&body, delta, tol)
}

fn length(metres: f64) -> Result<Expr, SceneDocError> {
    Expr::literal(metres, Dimension::Length).map_err(SceneDocError::Dimension)
}

fn scalar(v: f64) -> Result<Expr, SceneDocError> {
    Expr::literal(v, Dimension::Scalar).map_err(SceneDocError::Dimension)
}

fn insert(
    doc: Doc<ProfileProgram>,
    node: Node<ProfileProgram>,
    tol: Tol,
) -> Result<(Doc<ProfileProgram>, RecipeNodeId), SceneDocError> {
    let applied = apply(&doc, &DocEdit::InsertNode { node }, tol).map_err(SceneDocError::Edit)?;
    let minted = applied.record.minted.ok_or(SceneDocError::NoNodeMinted)?;
    Ok((applied.doc, minted))
}

/// The three corner points of one triangle, or `None` when an index
/// is out of range.
fn fetch(points: &[Point3<f64>], corners: &[u32; 3]) -> Option<[Point3<f64>; 3]> {
    let a = points.get(corners[0] as usize)?;
    let b = points.get(corners[1] as usize)?;
    let c = points.get(corners[2] as usize)?;
    Some([*a, *b, *c])
}

/// The unit normal of a triangle wound counterclockwise as seen from
/// the side the normal points to.
///
/// A degenerate (zero-area) triangle has no normal; it gets `+Z`
/// rather than a NaN, because a NaN in a vertex buffer poisons the
/// shading of everything the rasterizer blends it with, while a
/// wrong-facing sliver is invisible at the size a degenerate triangle
/// has.
fn triangle_normal(corners: &[Point3<f64>; 3]) -> [f32; 3] {
    let [a, b, c] = corners;
    let u = [b.x - a.x, b.y - a.y, b.z - a.z];
    let v = [c.x - a.x, c.y - a.y, c.z - a.z];
    let n = [
        u[1] * v[2] - u[2] * v[1],
        u[2] * v[0] - u[0] * v[2],
        u[0] * v[1] - u[1] * v[0],
    ];
    let len = (n[0] * n[0] + n[1] * n[1] + n[2] * n[2]).sqrt();
    if len > 0.0 && len.is_finite() {
        [
            (n[0] / len) as f32,
            (n[1] / len) as f32,
            (n[2] / len) as f32,
        ]
    } else {
        [0.0, 0.0, 1.0]
    }
}

/// **The ladder's own policy rows**, driven over a probe door rather
/// than a body: what a rung costs is a property of the tessellator,
/// but WHICH rungs are run, which refusal is survivable and which rule
/// ends the walk are properties of [`fit_from_probes`] alone, and a
/// synthetic law makes each of them a fact rather than a corpus
/// coincidence. The rows over real bodies are
/// `tests/display_budget.rs`.
#[cfg(test)]
mod tests {
    // Panicking is a test's failure mechanism (workspace lint note).
    #![allow(clippy::expect_used)]

    use super::{
        DisplayTolerance, PROBE_FACTOR, Probe, ProbeStop, SCALE_PROBE_DELTA, TRIANGLE_BUDGET,
        fit_from_probes,
    };
    use pncad::mesh::TessellateError;

    /// A body obeying the law exactly: `triangles = constant / δ`,
    /// never below a floor it reaches at coarse δ.
    fn curved(constant: f64, floor: usize) -> impl FnMut(f64) -> Result<Probe, TessellateError> {
        move |delta| {
            #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
            let law = (constant / delta) as usize;
            Ok(Probe {
                triangles: law.max(floor),
                extent: 1.0,
            })
        }
    }

    fn delta(value: f64) -> DisplayTolerance {
        DisplayTolerance::new(value).expect("a positive δ")
    }

    /// A body whose count never moves is answered by the count, not by
    /// the law — and in two probes, not the eleven a law-sized step
    /// would crawl through at this count (`8 · 62_500 / 10⁶` is a step
    /// of one halving).
    #[test]
    fn a_count_that_does_not_move_is_the_prediction() {
        let mut rungs: Vec<f64> = Vec::new();
        let flat = 62_500;
        let fitted = fit_from_probes(delta(1.0e-4), |d| {
            rungs.push(d);
            Ok(Probe {
                triangles: flat,
                extent: 1.0,
            })
        })
        .expect("a flat body fits");
        assert_eq!(fitted.stop, ProbeStop::Flat);
        assert_eq!(fitted.predicted, flat, "the count IS the prediction");
        assert_eq!(fitted.delta, fitted.requested, "and the request stands");
        assert_eq!(
            rungs.len(),
            3,
            "the scale probe, the one step that discovers the count did not move, and the \
             request's own rung: {rungs:?}"
        );
        let last = rungs.last().copied().expect("a rung");
        assert!(
            (last - 1.0e-4 * PROBE_FACTOR).abs() < f64::EPSILON,
            "the last rung prices the request: {rungs:?}"
        );
    }

    /// A flat body small enough that the law's own step already
    /// reaches the request's rung pays two probes, not three: the step
    /// that discovers the flatness IS the step to the request.
    #[test]
    fn a_small_flat_body_pays_two_probes() {
        let mut rungs: Vec<f64> = Vec::new();
        let fitted = fit_from_probes(delta(1.0e-4), |d| {
            rungs.push(d);
            Ok(Probe {
                triangles: 348,
                extent: 0.05,
            })
        })
        .expect("a flat body fits");
        assert_eq!(fitted.stop, ProbeStop::Flat);
        assert_eq!(fitted.predicted, 348);
        assert_eq!(rungs.len(), 2, "{rungs:?}");
    }

    /// The rung that prices the request is where an affordable request
    /// is answered from — the same mesh a fit with no ladder reads.
    #[test]
    fn an_affordable_request_is_answered_at_its_own_rung() {
        let fitted =
            fit_from_probes(delta(1.0e-4), curved(16.0, 256)).expect("the request is affordable");
        assert_eq!(fitted.stop, ProbeStop::AtTheRequest);
        assert_eq!(fitted.delta, fitted.requested);
        assert_eq!(fitted.requested_cost, None);
        // C/δ at the request, off the rung at PROBE_FACTOR × it.
        assert_eq!(fitted.predicted, 160_000);
    }

    /// A request the budget binds is answered from a rung
    /// `PROBE_FACTOR` coarser than the answer, and that rung is the
    /// largest — at most `TRIANGLE_BUDGET / PROBE_FACTOR`.
    #[test]
    fn a_bound_request_is_answered_from_a_rung_the_budget_can_afford() {
        let fitted = fit_from_probes(delta(1.0e-6), curved(16.0, 256)).expect("a fit");
        assert_eq!(fitted.stop, ProbeStop::Converged);
        assert!(
            fitted.delta.get() > fitted.requested.get(),
            "the budget moved δ"
        );
        // The per-rung bound, and the shape of its margin: a rung is
        // PLACED at `TRIANGLE_BUDGET / PROBE_FACTOR` triangles, and
        // what it actually counts meets that up to the law's error —
        // two-sided, so the margin is needed in this direction too
        // (this law is exact and still lands 8 triangles over, on
        // integer truncation alone).
        #[allow(clippy::cast_precision_loss)]
        let placed = TRIANGLE_BUDGET as f64 / PROBE_FACTOR;
        #[allow(clippy::cast_precision_loss)]
        let largest = fitted.largest_probe as f64;
        assert!(
            largest <= placed * 1.01,
            "the largest rung was {} triangles, past the {placed} it was placed at by more \
             than the law's error",
            fitted.largest_probe
        );
        assert!(
            fitted.probe_triangles >= fitted.largest_probe,
            "the total counts every rung"
        );
    }

    /// **A coarse probe that refuses never costs the document its
    /// budget.** The scale probe runs at a δ nothing else asks for, so
    /// its refusal falls back to the rung that prices the request —
    /// which is the whole fit a single-probe version would have done.
    #[test]
    fn a_refusal_at_the_scale_probe_falls_back_to_the_requests_own_rung() {
        let mut rungs: Vec<f64> = Vec::new();
        let mut law = curved(16.0, 256);
        let fitted = fit_from_probes(delta(1.0e-4), |d| {
            rungs.push(d);
            if d >= SCALE_PROBE_DELTA {
                return Err(TessellateError::InvalidChordalTolerance { value: d });
            }
            law(d)
        })
        .expect("the fit survives a coarse refusal");
        assert_eq!(fitted.stop, ProbeStop::Refused);
        assert_eq!(fitted.delta, fitted.requested);
        assert_eq!(fitted.predicted, 160_000, "priced off the request's rung");
        assert_eq!(rungs.len(), 2, "the refusal and the fallback: {rungs:?}");
    }

    /// A refusal further down keeps the reading it already has, rather
    /// than dropping the budget on the floor.
    #[test]
    fn a_refusal_below_the_first_rung_keeps_the_reading_above_it() {
        let mut seen = 0_usize;
        let mut law = curved(16.0, 256);
        let fitted = fit_from_probes(delta(1.0e-6), |d| {
            seen += 1;
            if seen > 2 {
                return Err(TessellateError::InvalidChordalTolerance { value: d });
            }
            law(d)
        })
        .expect("the fit survives a refusal mid-ladder");
        assert_eq!(fitted.stop, ProbeStop::Refused);
        assert!(
            fitted.delta.get() > fitted.requested.get(),
            "still budgeted"
        );
        assert_eq!(seen, 3, "it stopped at the refusal");
    }

    /// Both doors refusing is the one case with no answer to give.
    #[test]
    fn a_body_that_refuses_everywhere_refuses_the_fit() {
        let fitted = fit_from_probes(delta(1.0e-4), |d| {
            Err(TessellateError::InvalidChordalTolerance { value: d })
        });
        assert!(fitted.is_err());
    }
}
