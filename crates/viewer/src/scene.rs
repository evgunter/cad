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

use std::collections::BTreeSet;

use bvh::Aabb;
use pncad::document::{
    CancelToken, Datum, Dimension, Doc, DocEdit, EvalOptions, Expr, Frame, LoopProgram, Node,
    ProductError, ProfileProgram, RecipeNodeId, apply, evaluate, product,
};
use pncad::geom_core::{Affine3, Point3, Tol};
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
    /// [`SceneError::NoProduct`] forwards to [`ProductError`]'s own
    /// `Display`: the layer that raised a failure names it.
    /// [`SceneError::NotTessellated`] cannot — `mesh`'s
    /// `TessellateError` has no `Display`, so its value reaches a
    /// reader as a debug rendering until it grows one (issue #1111).
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
                    "the body did not tessellate at this display tolerance: {error:?}"
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
    /// is currently showing (`crate::pick::focus`).
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
                    .unwrap_or(crate::pick::IdMap::NOTHING);
                // The probe flag is the PART's; the focus flag is the
                // PATCH's, which is why it is computed here rather than
                // beside `flag` above.
                let flags_word = if id != crate::pick::IdMap::NOTHING && focus.contains(&id) {
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

/// The scene of a document under an evaluation SOMEONE ELSE ran.
///
/// The door the evaluation seam feeds: a result DAG arrives from
/// wherever it was computed, and the picture is gathered and
/// tessellated from it. [`scene_of`] is this function with an
/// evaluation of its own, kept for callers that have no seam — the two
/// share every step after the result exists, so a document drawn from
/// a background run and one drawn inline cannot differ.
///
/// # Errors
///
/// Every arm of [`SceneError`] except the δ one.
pub fn scene_of_evaluation(
    doc: &Doc<ProfileProgram>,
    evaluation: &pncad::document::Evaluation<f64>,
    delta: DisplayTolerance,
    tol: Tol,
) -> Result<SceneMesh, SceneError> {
    let body = product(doc, evaluation, tol).map_err(SceneError::NoProduct)?;
    let mesh = tessellate(&body, delta.get(), tol).map_err(SceneError::NotTessellated)?;
    SceneMesh::build(&mesh, delta)
}

/// **The picture's triangle budget**: the most a scene may carry
/// before the viewer draws it at a coarser δ than it was asked for.
///
/// # Why there is a budget at all
///
/// δ is a chord tolerance in metres, and nothing about an absolute
/// length knows how big a model is or how curved. The application
/// starts at 0.1 mm, which is a fine picture of the startup plate and
/// a 4·10⁶-triangle picture of the tour's `hollowring` (a torus of
/// R = 0.30 m) — 13 s of tessellation and index build with the window
/// frozen, still showing the previous document, which is what "Open
/// does nothing" looked like. A budget is what stops an absolute δ
/// from asking for a picture nobody can wait for.
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
/// - **The corpus, by eye.** At this budget both curved gallery
///   documents draw at δ ≈ 0.2–0.4 mm. Measured on the tour's own
///   scenes, the fillet corners of `diefillet` read clean there and
///   visibly band one doubling coarser, so it is also the first
///   budget that keeps the demo documents looking right.
///
/// **The consequence worth stating**: if this number ever has to be
/// RAISED to make something look right, the fault is upstream in the
/// sizing, not here — a budget cannot buy detail the tessellator is
/// spending elsewhere. The ring's 4·10⁶ triangles at 0.1 mm are about
/// 65× what the per-direction sagitta asks for
/// (`mesh::sizing::torus_grid_step` sizes both chart directions off
/// one conservative step); that is TESS-BUDGET's question, and this
/// constant is a safety net under it, never its answer.
pub const TRIANGLE_BUDGET: usize = 1_000_000;

/// How much coarser than the requested δ the cost probe runs.
///
/// Eight doublings-worth of cost is ~1/8 of the requested δ's
/// tessellation, which is what makes the probe affordable; and it is
/// close enough to the target that the 1/δ law below still holds
/// tightly (it softens at genuinely coarse δ, where planar faces have
/// stopped subdividing and only the curved ones still respond).
const PROBE_FACTOR: f64 = 8.0;

/// What [`fit_delta`] decided, and why — a value, so the chrome can
/// say it and a row can assert it.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FittedDelta {
    /// The δ to open at. Equal to [`FittedDelta::requested`] unless
    /// the budget moved it.
    pub delta: DisplayTolerance,
    /// The δ that was asked for.
    pub requested: DisplayTolerance,
    /// The triangle count predicted at [`FittedDelta::delta`] — an
    /// UPPER bound, not a measurement of the picture that gets built
    /// (`fit_delta` says why it is not verified).
    ///
    /// Upper in both of the ways it is wrong, which is the direction
    /// that makes a budget safe. The 1/δ law describes the CURVED
    /// faces; planar ones stop subdividing and then cost the same at
    /// every δ, so extrapolating from a coarse probe over-counts them
    /// — the tour's all-planar `heatsink` probes at 72 triangles and
    /// this reads 576, against 72 actually drawn. On the curved
    /// documents, where the number is load-bearing because the budget
    /// binds, it is over by 0.4% (`hollowring`) and 2.6%
    /// (`diefillet`). Over-counting cost means choosing a δ slightly
    /// coarser than needed; under-counting would mean drawing a
    /// picture over budget, and this cannot do that by this route.
    pub predicted: usize,
    /// What the requested δ was predicted to cost, when the budget
    /// moved δ; `None` when the request was affordable and nothing
    /// was changed.
    pub requested_cost: Option<usize>,
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
        }
    }

    /// The sentence the chrome shows, or `None` when the δ asked for
    /// was affordable and there is nothing to report.
    ///
    /// It ends by saying the budget is not a cap, because that is the
    /// question a reader has the moment they see a δ they did not
    /// choose, and a chosen default that read as a clamp would be
    /// worse than no default at all.
    pub fn wording(&self) -> Option<String> {
        let requested = self.requested_cost?;
        let opened = self.delta.get() * 1.0e3;
        let asked = self.requested.get() * 1.0e3;
        Some(format!(
            "opened at δ = {opened:.3} mm: {asked:.3} mm needs about {requested} triangles, over the {TRIANGLE_BUDGET} budget. A finer δ typed in the View pane is still honoured — this is a starting point, not a cap"
        ))
    }
}

/// Choose the δ to OPEN a document at: the requested one, or the
/// finest coarser one predicted to fit [`TRIANGLE_BUDGET`].
///
/// **A default, not a clamp.** The caller applies this once per
/// document that arrives (`app`'s `fit_delta_on_scene`); from there δ
/// is whatever the user types in the View pane, however fine, and
/// nothing re-reads it. A budget that bound every rebuild would
/// disable that field on exactly the documents someone would want it
/// for.
///
/// # The method: predict, do not ladder
///
/// A ladder — try δ, halve until it fits — pays for the tessellation
/// it then throws away, and the one it throws away is the expensive
/// one. So this probes ONCE, at [`PROBE_FACTOR`] × the request, and
/// solves.
///
/// The law it solves is `triangles ≈ C/δ`, which is what a chord-sized
/// grid over a fixed surface gives: each direction is cut ∝ 1/√δ, so
/// their product is ∝ 1/δ. It is not an assumption — measured on the
/// tour's `hollowring` the doubling ratios are 1.999, 1.996, 1.997,
/// 1.999 across four doublings, and on `diefillet` 1.99, 1.97, 1.98,
/// 1.94. So `C` is read off the probe and δ* = C / budget.
///
/// **The prediction is not verified, and that is deliberate.**
/// Verifying costs a second tessellation of the accepted size, which
/// is the whole expense this function exists to avoid, to correct an
/// error the measurements above bound at a few percent — and an error
/// whose SIGN is the safe one: the law describes curved faces, planar
/// ones stop subdividing and are therefore over-counted from a coarse
/// probe, so the fit errs coarse ([`FittedDelta::predicted`] carries
/// the measurements). Drawn against a 10⁶ budget the two curved
/// gallery documents land at 998 576 and 974 526 triangles.
///
/// # What it costs, and what it costs on a document that fits
///
/// One tessellation of the gathered product at the probe δ — about an
/// eighth of the request's. On a document already inside the budget
/// the prediction returns a δ* below the request, the request is kept,
/// and the probe was a tessellation of a small mesh: `checks` and
/// `heatsink` are 24 and 72 triangles at every δ, because an
/// all-planar body never subdivides.
///
/// # The count is the picture's, not an estimate of it
///
/// The probe tessellates the GATHERED product, while the picture is
/// built per root by `crate::pick::PickIndex`. Those are the same
/// number: measured across four δ on both multi-root gallery
/// documents, gathered and per-root triangle counts agree exactly
/// (0.000%), because the graft moves solids into one body without
/// re-cutting their faces.
///
/// # Errors
///
/// [`SceneError::NoProduct`] if the roots do not gather,
/// [`SceneError::NotTessellated`] if the probe refuses, and
/// [`SceneError::InvalidDisplayTolerance`] if the solved δ is not a
/// usable one.
pub fn fit_delta(
    doc: &Doc<ProfileProgram>,
    evaluation: &pncad::document::Evaluation<f64>,
    requested: DisplayTolerance,
    tol: Tol,
) -> Result<FittedDelta, SceneError> {
    let body = product(doc, evaluation, tol).map_err(SceneError::NoProduct)?;
    let probe_delta = requested.scaled(PROBE_FACTOR)?;
    let probe = tessellate(&body, probe_delta.get(), tol).map_err(SceneError::NotTessellated)?;
    let probe_triangles: usize = probe
        .patches
        .iter()
        .map(|patch| patch.triangles.len())
        .sum();
    // C, in triangle·metres. A body that tessellates to nothing at the
    // probe δ has no curvature to spend on, so it costs the same at
    // every δ and the request stands.
    let constant = probe_triangles as f64 * probe_delta.get();
    #[allow(clippy::cast_precision_loss)]
    let budget = TRIANGLE_BUDGET as f64;
    let solved = constant / budget;
    if solved <= requested.get() {
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        let predicted = (constant / requested.get()) as usize;
        return Ok(FittedDelta {
            delta: requested,
            requested,
            predicted,
            requested_cost: None,
        });
    }
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let requested_cost = (constant / requested.get()) as usize;
    Ok(FittedDelta {
        delta: DisplayTolerance::new(solved)?,
        requested,
        predicted: TRIANGLE_BUDGET,
        requested_cost: Some(requested_cost),
    })
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
    let mesh = tessellate(&body, delta.get(), tol).map_err(SceneError::NotTessellated)?;
    SceneMesh::build(&mesh, delta)
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
