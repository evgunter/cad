//! STL export from certified tessellations (M2 PR 7): binary and
//! ASCII writers over [`mesh::Mesh`].
//!
//! # What is exported, and what is promised
//!
//! The writers consume `Mesh::positions` and each patch's triangles —
//! outward winding is guaranteed by the tessellator (M2 PR 6) — and
//! drop the back-reference keys (faces/edges/vertices mean nothing to
//! STL). Triangles are emitted in patch order, then triangle order,
//! exactly as stored: **no snapping, no deduplication, no reordering**
//! — the writer adds zero nondeterminism on top of the mesh (no hash
//! iteration anywhere on the write path), so the mesh's bitwise
//! determinism (verified across debug/release and the ε rows in the
//! PR 6 suites) carries through to **byte-identical STL output for
//! identical inputs** (D9).
//!
//! # The f32 narrowing is an honest display-layer loss
//!
//! STL stores f32. The certified δ+ε export promise (per-triangle
//! deviation certificates, `mesh::cert`) is about the **f64 mesh**;
//! casting each coordinate to f32 adds at most 1 ulp of relative
//! rounding per coordinate on top (about 6e-8 relative — far below
//! any practical δ, but not covered by the certificates). The cast is
//! `as f32` (round-to-nearest-even), deterministic.
//!
//! # Normals
//!
//! Each facet normal is computed in f64 from the triangle's vertex
//! winding (`(b−a)×(c−a)`, normalized), then cast to f32. Mesh
//! triangles are non-degenerate by construction (the tessellator drops
//! degenerates), so a zero cross product is a defect upstream — it is
//! a typed [`StlError::DegenerateTriangle`], never a silently zeroed
//! or guessed normal (fail loud, D4/D9).
//!
//! # Choosing δ for export
//!
//! Tessellation wall-clock is ~quadratic in per-face point count (see
//! the performance note in `mesh`'s crate docs): each 100× tightening
//! of δ costs ~100× more triangles and ~10⁴× more CDT time. The
//! acceptance exports use moderate δ (≈ 1e-3 of the model scale);
//! consumers wanting fine δ should budget accordingly — the mesh's
//! `ResolutionOverflow` cap bounds allocation, not wall-clock.

mod ascii;
mod binary;

pub use ascii::write_ascii;
pub use binary::write_binary;

/// Typed export failure (closed enum, D4 ¶3).
#[derive(Debug)]
pub enum StlError {
    /// A triangle's winding cross product is exactly zero — the mesh
    /// violated its non-degeneracy contract; no honest normal exists.
    DegenerateTriangle {
        /// The triangle's position indices.
        triangle: [u32; 3],
    },
    /// A triangle references a position index out of range — corrupt
    /// mesh, surfaced rather than trusted.
    IndexOutOfRange {
        /// The offending index.
        index: u32,
    },
    /// The mesh holds more than `u32::MAX` triangles — unrepresentable
    /// in binary STL's 32-bit facet count.
    TooManyTriangles {
        /// The actual triangle count.
        count: usize,
    },
    /// An I/O failure from the output sink.
    Io(std::io::Error),
}

impl core::fmt::Display for StlError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::DegenerateTriangle { triangle } => write!(
                f,
                "stl export: triangle {triangle:?} has a zero winding cross product \
                 (degenerate; the mesh's non-degeneracy contract is violated)"
            ),
            Self::IndexOutOfRange { index } => {
                write!(f, "stl export: position index {index} out of range")
            }
            Self::TooManyTriangles { count } => write!(
                f,
                "stl export: {count} triangles exceed binary STL's u32 facet count"
            ),
            Self::Io(e) => write!(f, "stl export: io error: {e}"),
        }
    }
}

impl std::error::Error for StlError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io(e) => Some(e),
            _ => None,
        }
    }
}

impl From<std::io::Error> for StlError {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e)
    }
}

/// One facet as both writers see it: the f64 unit normal and the three
/// f64 vertices, in patch-then-triangle order. Shared by the binary
/// and ASCII writers so both emit the identical triangle stream.
pub(crate) struct Facet {
    pub normal: [f64; 3],
    pub vertices: [[f64; 3]; 3],
}

/// Iterate the mesh's facets in the fixed export order, computing each
/// winding normal in f64 (module docs). This is the single triangle
/// walk both writers share.
pub(crate) fn facets(mesh: &mesh::Mesh) -> Result<Vec<Facet>, StlError> {
    let n = mesh.positions.len();
    let mut out = Vec::new();
    for patch in &mesh.patches {
        for tri in &patch.triangles {
            for &i in tri {
                if (i as usize) >= n {
                    return Err(StlError::IndexOutOfRange { index: i });
                }
            }
            let [a, b, c] = [
                mesh.positions[tri[0] as usize],
                mesh.positions[tri[1] as usize],
                mesh.positions[tri[2] as usize],
            ];
            let u = b - a;
            let v = c - a;
            let cross = u.cross(v);
            let len = cross.norm();
            if len == 0.0 || !len.is_finite() {
                return Err(StlError::DegenerateTriangle { triangle: *tri });
            }
            out.push(Facet {
                normal: [cross.x / len, cross.y / len, cross.z / len],
                vertices: [[a.x, a.y, a.z], [b.x, b.y, b.z], [c.x, c.y, c.z]],
            });
        }
    }
    Ok(out)
}
