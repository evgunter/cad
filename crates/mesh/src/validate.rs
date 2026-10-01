//! Mesh validity: the watertightness/orientation checker and the
//! divergence-theorem signed volume (the acceptance oracles; PR 7's
//! STL exporter and mass properties consume the same triangle data).

use std::collections::BTreeMap;

use crate::types::Mesh;
use topo::FaceKey;

/// Typed mesh-validity failure (closed enum).
#[derive(Clone, Debug, PartialEq)]
pub enum MeshError {
    /// The mesh carries no triangles at all — no patch, or every patch
    /// empty. Not the mesh of a solid: a solid has surface, and every
    /// closure condition below is universal over edges, so a mesh of
    /// nothing satisfies all of them by having none, [`signed_volume`]
    /// zero beside it.
    ///
    /// **Reachable by input, and invalid (D2 addendum row 1).**
    /// [`Mesh`]'s fields are public, so a caller may hold any triangle
    /// set at all; `tessellate` reaches the state too, most plainly on
    /// a body with no faces (`topo::validate`'s own doc says the empty
    /// body validates vacuously, and this lane has no face-count
    /// guard), and on a curved face whose walk collapses to a
    /// degenerate domain where debug assertions are off. Not row 4: a
    /// validator is handed meshes of unknown provenance by contract, so
    /// it answers typed rather than panics.
    ///
    /// **Invalid AGAINST THIS CONTRACT**, and that is the whole of what
    /// it says — an empty body is a valid body, exactly as a single
    /// triangle is a fine triangle, and neither is a solid's boundary.
    /// A caller who needs to tell "nothing to show" from "a hole"
    /// cannot learn it here: asked of the mesh alone the two are the
    /// same bytes.
    ///
    /// Row 0 (can the state be made unrepresentable?) is answered no:
    /// a non-emptiness guarantee on [`Mesh`] means a private triangle
    /// buffer and a fallible constructor, which costs exactly the
    /// hand-built broken mesh this validator exists to catch — and the
    /// per-face emptiness below cannot be typed away at all without a
    /// non-empty vector in [`crate::FacePatch`], whose whole public
    /// surface is that vector.
    ///
    /// Rows: `survives_checkmesh_refuses_the_empty_mesh` and
    /// `survives_the_empty_body_meshes_to_nothing_and_the_validator_says_so`.
    NoTriangles,
    /// One face's patch carries no triangles beside patches that do —
    /// a hole where a face is. Also not the mesh of a solid, and the
    /// same state as [`Self::NoTriangles`] one level down.
    ///
    /// **Why this validator names it, and `tessellate` does not.** A
    /// REFUSAL is decided on structure and reads no count: TESS-1
    /// ruled that (`work/tess/TESS-1.md`, `## Closed` — "no float
    /// decides it"; the spec itself is deleted) and
    /// [`crate::TessellateError::MeridianFreeCurvedFace`] is built that
    /// way. This is the other thing: a property RE-DERIVED from the
    /// emitted mesh, where the count is the only evidence there is.
    /// `tessellate` re-derives it too, in the cross-face census, which
    /// is `debug_assertions`-only; in release this arm is the whole of
    /// what sees a hole whose structural fact no guard in front of the
    /// walk has found yet.
    ///
    /// The state is not otherwise unnamed — an empty patch leaves its
    /// face's chord segments used once by each neighbour, so the edge
    /// census below reports [`Self::BoundaryEdge`] on one of them (a
    /// rim-only sphere cap closed by a disc measured exactly that).
    /// That names an edge and blames the wrong side; this arm names the
    /// face that emitted nothing, which is the fact. Row:
    /// `survives_checkmesh_names_the_face_of_an_empty_patch`.
    EmptyPatch {
        /// The face whose patch is empty.
        face: FaceKey,
    },
    /// A triangle references a position index out of range.
    IndexOutOfRange {
        /// The offending index.
        index: u32,
    },
    /// A triangle repeats a vertex (degenerate in 3-D).
    DegenerateTriangle {
        /// The offending triangle.
        triangle: [u32; 3],
    },
    /// An undirected edge is used by more than two triangles.
    NonManifoldEdge {
        /// Edge endpoints (ascending).
        edge: (u32, u32),
        /// How many triangles use it.
        count: u32,
    },
    /// An undirected edge is used by exactly one triangle — the mesh
    /// has a boundary (a closed body's mesh must not).
    BoundaryEdge {
        /// Edge endpoints (ascending).
        edge: (u32, u32),
    },
    /// An undirected edge is used twice **with the same direction** —
    /// its two triangles disagree on winding.
    MismatchedWinding {
        /// Edge endpoints (ascending).
        edge: (u32, u32),
    },
}

/// Checks that the mesh is **the mesh of a solid**: a closed,
/// consistently wound 2-manifold with surface — every patch carrying
/// triangles, every triangle index valid and non-degenerate, every
/// undirected edge shared by exactly two triangles traversing it in
/// opposite directions.
///
/// **Surface is part of the contract, not a consequence of it.** The
/// 2-manifold conditions are all universal over edges, so a mesh of no
/// triangles satisfies every one of them vacuously; read as "closed
/// 2-manifold" this function would therefore accept a mesh of nothing,
/// and so would every caller, each of which asks it whether it is
/// holding a solid's boundary ([`MeshError::NoTriangles`] lists the
/// producers). An empty patch beside filled ones is the same state per
/// face ([`MeshError::EmptyPatch`]). Both are refused before the edge
/// census runs, so the report names the emptiness rather than a
/// neighbour's dangling edge, and before the per-triangle checks, so
/// an empty patch outranks a bad index in another one.
///
/// The check is **combinatorial only** — it inspects indices and
/// counts, never positions: geometrically-zero-area slivers (distinct
/// indices, coincident points) and globally-inverted shells both pass;
/// [`signed_volume`] is the orientation backstop.
///
/// # Errors
///
/// The first failure in deterministic order (the whole mesh's
/// emptiness, then the first empty patch in patch order, then
/// triangles in patch order, then edges in ascending index order), as
/// a typed [`MeshError`].
pub fn check_mesh(mesh: &Mesh) -> Result<(), MeshError> {
    if triangle_count(mesh) == 0 {
        return Err(MeshError::NoTriangles);
    }
    if let Some(patch) = mesh.patches.iter().find(|p| p.triangles.is_empty()) {
        return Err(MeshError::EmptyPatch { face: patch.face });
    }
    let n = mesh.positions.len();
    // (min, max) -> (uses, direction balance: +1 for min->max).
    let mut edges: BTreeMap<(u32, u32), (u32, i64)> = BTreeMap::new();
    for patch in &mesh.patches {
        for tri in &patch.triangles {
            for &i in tri {
                if (i as usize) >= n {
                    return Err(MeshError::IndexOutOfRange { index: i });
                }
            }
            if tri[0] == tri[1] || tri[1] == tri[2] || tri[0] == tri[2] {
                return Err(MeshError::DegenerateTriangle { triangle: *tri });
            }
            for k in 0..3 {
                let (a, b) = (tri[k], tri[(k + 1) % 3]);
                let key = crate::walk::edge_key(a, b);
                let entry = edges.entry(key).or_insert((0, 0));
                entry.0 += 1;
                entry.1 += if a < b { 1 } else { -1 };
            }
        }
    }
    for (edge, (count, balance)) in edges {
        match count {
            1 => return Err(MeshError::BoundaryEdge { edge }),
            2 => {
                if balance != 0 {
                    return Err(MeshError::MismatchedWinding { edge });
                }
            }
            _ => return Err(MeshError::NonManifoldEdge { edge, count }),
        }
    }
    Ok(())
}

/// The mesh's signed volume by the divergence theorem,
/// `V = (1/6)·Σ det[a−o, b−o, c−o]` over all triangles, with `o` the
/// positions' bbox centre — positive for outward-wound closed meshes
/// (the global orientation invariant).
///
/// Two distinct properties, with distinct premises:
///
/// * **Translation invariance holds unconditionally** — open or
///   closed: `o` is derived from the positions, so translating the
///   mesh translates `o` with it and every operand `p − o` is
///   unchanged (equivariance).
/// * **Equality to the enclosed volume needs a CLOSED mesh.** Over ℝ
///   the sum is anchor-independent exactly when the anchor's extra
///   terms assemble the surface's total area vector — zero iff the
///   mesh is closed — so for a closed mesh any `o` measures the
///   enclosed volume. For an OPEN mesh the value is the signed volume
///   of the cone from `o` over the triangles: placement-stable (see
///   above) but anchor-dependent, and NOT an enclosed volume.
///
/// Numerically, the position-derived anchor keeps the fold's operands
/// at the body's own scale (body-SCALE, not necessarily interior — a
/// non-convex body's bbox centre can sit outside the material), where
/// a world-origin anchor pays cancellation proportional to the
/// placement distance. The bbox folds over ALL positions rather than
/// only triangle-referenced ones: in meshes this crate mints every
/// position lies on the body's surface (topology vertices, edge chord
/// points, face grid points), so the two sets share a bbox — and any
/// body-scale anchor serves regardless.
///
/// Validity is [`check_mesh`]'s contract, not this fold's: a corrupt
/// mesh whose triangles reference a missing position gets
/// `IndexOutOfRange` there, while here it panics on the index — except
/// the empty cases, which return a quiet 0.0: no position (the `else`
/// below) and no triangle over positions that exist (the fold runs
/// over nothing). Both are [`MeshError::NoTriangles`] at the
/// validator.
pub fn signed_volume(mesh: &Mesh) -> f64 {
    let Some(&first) = mesh.positions.first() else {
        return 0.0;
    };
    let (lo, hi) = mesh
        .positions
        .iter()
        .fold((first, first), |(lo, hi), &p| (lo.min(p), hi.max(p)));
    let o = lo + (hi - lo) * 0.5;
    let mut six_v = 0.0;
    for patch in &mesh.patches {
        for tri in &patch.triangles {
            let a = mesh.positions[tri[0] as usize] - o;
            let b = mesh.positions[tri[1] as usize] - o;
            let c = mesh.positions[tri[2] as usize] - o;
            six_v += a.dot(b.cross(c));
        }
    }
    six_v / 6.0
}

/// Total triangle count across all patches.
pub fn triangle_count(mesh: &Mesh) -> usize {
    mesh.patches.iter().map(|p| p.triangles.len()).sum()
}
