//! STL export from certified tessellations: binary and
//! ASCII writers over [`mesh::Mesh`].
//!
//! # What is exported, and what is promised
//!
//! The writers consume `Mesh::positions` and each patch's triangles —
//! outward winding is guaranteed by the tessellator — and
//! drop the back-reference keys (faces/edges/vertices mean nothing to
//! STL). Everything a file carries that is *not* triangles — the ASCII
//! solid name, the binary 80-byte header — comes from the caller,
//! through [`AsciiOptions`] and [`BinaryOptions`]: one options type per
//! writer, each carrying only what its own format reads, and each
//! field a validated newtype so an unwritable value cannot be built
//! (see [`options`]). Triangles are emitted in patch order, then
//! triangle order, exactly as stored: **no snapping, no deduplication,
//! no reordering** — the writer adds zero nondeterminism on top of the
//! mesh (no hash iteration anywhere on the write path), so the mesh's
//! bitwise determinism (verified across debug/release and the ε rows)
//! carries through: **same mesh value + same options ⇒ byte-identical
//! STL output** (D9).
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
//! winding (`(b−a)×(c−a)`, normalized), then cast to f32. The
//! tessellator drops index-degenerate triangles, so a zero cross
//! product is a defect — it is a typed
//! [`StlError::DegenerateTriangle`], never a silently zeroed or
//! guessed normal (fail loud, D4/D9). Known live case: at coarse δ
//! a cone's apex fan can emit triangles whose
//! three points are exactly collinear along a generator (distinct
//! indices, zero 3-D area — invisible to the id-degenerate drop and
//! to the combinatorial `check_mesh`); the writer refuses those
//! meshes, and the export tests pin the behavior — pick a finer δ
//! (the acceptance set uses δ = 1e-2 for the cone).
//!
//! The normal's *direction* is therefore whatever the winding says,
//! and STL's own rule (facet normal ≈ outward, agreeing with the
//! right-hand vertex order) holds here **transitively**: it is
//! inherited from `mesh::FacePatch::triangles`, whose contract is
//! outward winding for either value of the face orientation sense.
//! This writer never reads `topo::Face::sense` — it has no access to
//! the body at all — and it must not: the sense is already baked into
//! the triangle order upstream, so applying it again would invert
//! every facet on a reversed face.
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
pub mod options;

pub use ascii::write_ascii;
pub use binary::write_binary;
pub use options::{
    AsciiOptions, BinaryHeader, BinaryHeaderError, BinaryOptions, SolidName, SolidNameError,
};

/// Typed export failure from the writers (closed enum, D4 ¶3).
///
/// **The mesh and the sink only.** Everything an option could get
/// wrong is refused at construction by [`SolidName::new`] and
/// [`BinaryHeader::new`], each with its own error type, so no arm here
/// is about a value the caller supplied through options.
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
            // The normal is computed from the f64 vertices — the
            // honest normal of the certified tessellation. Its
            // ORIENTATION comes from the triangle order and nothing
            // else: outwardness is the mesh's guarantee, already
            // sense-correct (module docs).
            // (Not computed from the f32-narrowed vertices:
            // apex-fan slivers become EXACTLY collinear
            // under f32 rounding at every practical δ, so an
            // "as-written" normal doesn't exist for them; external
            // checkers recomputing normals from the file's f32
            // vertices may report small disagreements on such slivers
            // — a narrowing artifact, not an orientation defect.)
            let vertex = |i: u32| {
                let p = mesh.positions[i as usize];
                [p.x, p.y, p.z]
            };
            let [a, b, c] = [vertex(tri[0]), vertex(tri[1]), vertex(tri[2])];
            let u = [b[0] - a[0], b[1] - a[1], b[2] - a[2]];
            let v = [c[0] - a[0], c[1] - a[1], c[2] - a[2]];
            let cross = [
                u[1] * v[2] - u[2] * v[1],
                u[2] * v[0] - u[0] * v[2],
                u[0] * v[1] - u[1] * v[0],
            ];
            let len = (cross[0] * cross[0] + cross[1] * cross[1] + cross[2] * cross[2]).sqrt();
            if len == 0.0 || !len.is_finite() {
                return Err(StlError::DegenerateTriangle { triangle: *tri });
            }
            out.push(Facet {
                normal: [cross[0] / len, cross[1] / len, cross[2] / len],
                vertices: [a, b, c],
            });
        }
    }
    Ok(out)
}
