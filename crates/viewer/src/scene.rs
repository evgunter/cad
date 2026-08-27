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

use bvh::Aabb;
use pncad::document::{
    CancelToken, Dimension, Doc, DocEdit, EvalOptions, Expr, LoopProgram, Node, ProfileProgram,
    ProductError, RecipeNodeId, apply, evaluate, product,
};
use pncad::geom_core::{Point3, Tol};
use pncad::mesh::{Mesh, TessellateError, tessellate};
use pncad::profile::SketchPlane;
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
    /// # Errors
    ///
    /// [`SceneError::InvalidDisplayTolerance`] — the same condition
    /// `mesh::tessellate` refuses, checked at the door instead of
    /// four call sites later.
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
}

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
    bounds: Aabb,
    stats: SceneStats,
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
}

impl SceneMesh {
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
    /// triangle, or no finite bounding box.
    pub fn build(mesh: &Mesh, delta: DisplayTolerance) -> Result<Self, SceneError> {
        let triangles: usize = mesh.patches.iter().map(|p| p.triangles.len()).sum();
        if triangles == 0 {
            return Err(SceneError::EmptyMesh);
        }
        let mut positions = Vec::with_capacity(triangles * 3);
        let mut normals = Vec::with_capacity(triangles * 3);
        for patch in &mesh.patches {
            for corners in &patch.triangles {
                let Some(corner_points) = fetch(&mesh.positions, corners) else {
                    // A patch index outside the shared position table
                    // is a broken mesh, not a display choice: drop the
                    // whole scene rather than a silent triangle.
                    return Err(SceneError::EmptyMesh);
                };
                let normal = triangle_normal(&corner_points);
                for p in corner_points {
                    positions.push([p.x as f32, p.y as f32, p.z as f32]);
                    normals.push(normal);
                }
            }
        }
        let indices = (0..positions.len() as u32).collect();
        let bounds = Aabb::from_points(mesh.positions.iter().copied())
            .ok_or(SceneError::EmptyMesh)?;
        Ok(Self {
            positions,
            normals,
            indices,
            bounds,
            stats: SceneStats {
                faces: mesh.patches.len(),
                triangles,
                display_delta: delta.get(),
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

    /// The scene's bounding box, in world units — what a camera
    /// frames against.
    pub fn bounds(&self) -> Aabb {
        self.bounds
    }

    /// What this scene cost.
    pub fn stats(&self) -> SceneStats {
        self.stats
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
/// # Errors
///
/// Never, as written: the expressions are literal lengths and the
/// node graph is well formed. The signature carries the `Result`
/// because every door it calls does — a scene that silently swallowed
/// an `EditError` would be a worse example than one that reports it.
pub fn plate_with_hole(tol: Tol) -> Result<(Doc<ProfileProgram>, RecipeNodeId), SceneDocError> {
    // 60 × 40 × 8 mm, hole ⌀24 mm on centre. Canonical metres.
    let outline = LoopProgram::polygon([
        (0.0, 0.0),
        (0.060, 0.0),
        (0.060, 0.040),
        (0.0, 0.040),
    ])
    .map_err(SceneDocError::Dimension)?;
    let hole = LoopProgram::Circle {
        centre: [length(0.030)?, length(0.020)?],
        radius: length(0.012)?,
    };
    let profile = ProfileProgram {
        plane: SketchPlane::xy(),
        loops: vec![outline, hole],
    };
    let doc: Doc<ProfileProgram> = Doc::empty_derived("gui-0-plate", tol);
    let (doc, profile_node) = insert(doc, Node::Profile(profile), tol)?;
    let (doc, extrude) = insert(
        doc,
        Node::Extrude {
            profile: profile_node,
            distance: length(0.008)?,
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
