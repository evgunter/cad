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
    /// empty. Not the mesh of a solid: a solid has surface, and its
    /// mesh is watertight only vacuously (no edge, so every edge is
    /// shared twice) with a [`signed_volume`] of zero.
    ///
    /// **Reachable by input, and invalid (D2 addendum row 1).**
    /// [`Mesh`]'s fields are public, so the caller of this validator
    /// may hold any triangle set at all — an empty one included, which
    /// is what the hand-built rows of
    /// `mesh/tests/review_m2_pr6_checkmesh_audit.rs` exercise. Three
    /// producers reach it through [`fn@crate::tessellate`]:
    ///
    /// * **The empty body.** `topo::Body::new()` is public and every
    ///   arena of it is empty; `topo::validate`'s own doc says the
    ///   empty body validates vacuously, so it is tier-1 and tier-2
    ///   VALID, and this lane has no face-count guard — it meshes to
    ///   zero patches and `Ok`. A boolean that annihilates its operands
    ///   does NOT go this way: `topo::BooleanResult::Empty` is a typed
    ///   empty success carrying no body at all, so ∅ never arrives here
    ///   wearing a solid's clothes. A caller meshing a body it did not
    ///   author distinguishes "nothing to show" from "a hole" before
    ///   this validator, by the body's own face count; asked of the
    ///   mesh alone the two states are the same bytes, and this arm is
    ///   the honest answer to the question this function was asked.
    /// * **A curved face that walks to a degenerate domain**, with
    ///   debug assertions off, where the cross-face census that
    ///   otherwise catches it is compiled out: a loop of rims only
    ///   walks to zero HEIGHT and a loop whose meridians all stand on
    ///   one column to zero WIDTH, and either triangulates to nothing.
    ///   The first is refused typed at the walk
    ///   ([`crate::TessellateError::MeridianFreeCurvedFace`]); the
    ///   second is open
    ///   (`work/tess/rim-free-loop-on-a-poleless-chart-meshes-as-a-hole.md`)
    ///   and today hands a two-face torus a mesh of two empty patches.
    /// * **The planar lane's own shape of it**, unmeasured: `planar`'s
    ///   `classify_faces` seeds the inside-walk across the convex hull,
    ///   and a CDT whose every hull edge has the outer face on both
    ///   sides marks nothing inside, so the lane emits nothing and
    ///   answers `Ok`. Reaching it wants every inserted point collinear
    ///   in the chart, and the chart frame is derived from that same
    ///   loop's area vector — zero for a collinear loop, which makes
    ///   the frame non-finite and refuses at `spade`'s insert
    ///   (`Triangulation`). So the arm is believed unreachable rather
    ///   than measured unreachable, and it is listed because it is the
    ///   planar shape of a class whose curved shape is live.
    ///
    /// It is not row 4: a validator is handed meshes of unknown
    /// provenance by contract, so it answers typed rather than panics.
    /// "Invalid" is invalid AGAINST THIS CONTRACT and says nothing
    /// about the body — an empty body is a valid body, exactly as a
    /// single triangle is a fine triangle, and neither is a solid's
    /// boundary.
    ///
    /// Row 0 (can the state be made unrepresentable?) is answered no:
    /// a non-emptiness guarantee on [`Mesh`] means a private triangle
    /// buffer and a fallible constructor, which costs exactly the
    /// hand-built broken mesh this validator exists to catch — and the
    /// per-face emptiness below cannot be typed away at all without a
    /// non-empty vector in [`crate::FacePatch`], whose whole public
    /// surface is that vector.
    NoTriangles,
    /// One face's patch carries no triangles beside patches that do —
    /// a hole where a face is. Also not the mesh of a solid, and the
    /// same state as [`Self::NoTriangles`] one level down.
    ///
    /// **Why this validator names it, and `tessellate` does not.** A
    /// REFUSAL is decided on structure and reads no triangle count —
    /// [`crate::TessellateError::MeridianFreeCurvedFace`]'s doc states
    /// that rule and is decided that way. This is the other thing: a
    /// property RE-DERIVED from the emitted mesh, where the count is
    /// the only evidence there is. `tessellate` re-derives it too, in
    /// the cross-face census, which is `debug_assertions`-only; in
    /// release this arm is the whole of what sees a hole whose
    /// structural fact no guard in front of the walk has found yet.
    ///
    /// The state is not otherwise unnamed — an empty patch leaves its
    /// face's chord segments used once by each neighbour, so the edge
    /// census below reports [`Self::BoundaryEdge`] on one of them (a
    /// rim-only sphere cap closed by a disc measured exactly that).
    /// That names an edge and blames the wrong side; this arm names the
    /// face that emitted nothing, which is the fact.
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
/// neighbour's dangling edge. (The crate already refuses a vacuous
/// verdict once, one predicate over: `nurbs_cert`'s `Domination::new`
/// asserts its component list non-empty rather than let `holds` answer
/// `true` over none.)
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
/// the empty-positions case, which returns a quiet 0.0.
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
