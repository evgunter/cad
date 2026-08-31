//! Mesh validity: the watertightness/orientation checker and the
//! divergence-theorem signed volume (the acceptance oracles; PR 7's
//! STL exporter and mass properties consume the same triangle data).

use std::collections::BTreeMap;

use crate::types::Mesh;

/// Typed mesh-validity failure (closed enum).
#[derive(Clone, Debug, PartialEq)]
pub enum MeshError {
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

/// Checks that the mesh is a closed, consistently wound 2-manifold:
/// every triangle index valid and non-degenerate, every undirected
/// edge shared by exactly two triangles traversing it in opposite
/// directions.
///
/// The check is **combinatorial only** — it inspects indices, never
/// positions: geometrically-zero-area slivers (distinct indices,
/// coincident points) and globally-inverted shells both pass;
/// [`signed_volume`] is the orientation backstop.
///
/// # Errors
///
/// The first failure in deterministic order (triangles in patch order,
/// then edges in ascending index order), as a typed [`MeshError`].
pub fn check_mesh(mesh: &Mesh) -> Result<(), MeshError> {
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
                let key = (a.min(b), a.max(b));
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
/// Recentring on `o` is exact over ℝ: for a closed mesh the sum is
/// translation-invariant (the anchor's extra terms assemble the
/// surface's total area vector, which is zero), so any `o` measures
/// the same volume — and a body-interior `o` keeps the fold's
/// operands at the body's own scale, where a world-origin anchor
/// pays cancellation proportional to the placement distance.
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
