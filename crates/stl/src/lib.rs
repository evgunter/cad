//! STL export from certified tessellations: binary and
//! ASCII writers over [`mesh::Mesh`].
//!
//! # What is exported, and what is promised
//!
//! The writers consume `Mesh::positions` and each patch's triangles —
//! outward winding is guaranteed by the tessellator — and
//! drop the back-reference keys (faces/edges/vertices mean nothing to
//! STL). Everything a file carries that is *not* triangles — the ASCII
//! solid name, the binary 80-byte header — comes from the caller's
//! [`StlOptions`]. Triangles are emitted in patch order, then triangle
//! order, exactly as stored: **no snapping, no deduplication, no
//! reordering** — the writer adds zero nondeterminism on top of the
//! mesh (no hash iteration anywhere on the write path), so the mesh's
//! bitwise determinism (verified across debug/release and the ε rows)
//! carries through: **same mesh value + same [`StlOptions`] ⇒
//! byte-identical STL output** (D9).
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
    /// The requested ASCII solid name carries a character outside the
    /// printable-ASCII range the `solid <name>` grammar admits — a
    /// newline would make `endsolid <name>` unmatchable and the file
    /// unparseable, so the name is refused rather than sanitized.
    UnrepresentableSolidName {
        /// The offending character.
        character: char,
    },
    /// The requested binary header does not fit binary STL's 80-byte
    /// header field. Refused rather than truncated.
    HeaderTooLong {
        /// The requested header's length in bytes.
        len: usize,
    },
    /// The requested binary header reads as the ASCII-STL `solid`
    /// keyword, which makes the file sniff as ASCII STL in the parsers
    /// that decide by it. The recognised class is wider than a
    /// byte-exact prefix — leading whitespace is skipped and case is
    /// folded, because sniffers in the wild do both — so
    /// `" Solid v2"` is refused as well as `"solid v2"`.
    HeaderSniffsAscii,
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
            Self::UnrepresentableSolidName { character } => write!(
                f,
                "stl export: solid name contains {character:?}, outside the printable \
                 ASCII the single-line `solid <name>` grammar admits"
            ),
            Self::HeaderTooLong { len } => write!(
                f,
                "stl export: {len}-byte header exceeds binary STL's 80-byte header field"
            ),
            Self::HeaderSniffsAscii => write!(
                f,
                "stl export: a binary header reading as the `solid` keyword (leading \
                 whitespace skipped, case folded) sniffs as ASCII STL"
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

/// Export options: everything that lands in the file besides the mesh.
///
/// The two things an STL file carries that are not triangles — the
/// ASCII `solid <name>` name and the binary format's 80-byte header.
/// They name **different kinds of thing**: the solid name is the
/// *part*, the header is free text the format leaves to the writer and
/// which conventionally identifies the *producer*. That split is why
/// the defaults differ in kind, and it is the whole content of the
/// pair; a caller who cares about only one format sets only that
/// format's field.
///
/// Each writer reads the field its format has and validates that field
/// only, so a header longer than 80 bytes is a refusal from
/// [`write_binary`] and not from [`write_ascii`]. **A field the other
/// format does not read is inert, not defaulted-away**: exporting
/// binary does not consult [`Self::solid_name`] at all.
///
/// Determinism (D9): each writer is a pure function of
/// `(mesh, options)` — equal inputs give byte-identical files. The
/// writers read no clock, no environment variable and no global state,
/// and neither default carries one.
#[derive(Clone, Debug, PartialEq)]
pub struct StlOptions {
    /// The ASCII writer's `solid <name>` name, closed by the matching
    /// `endsolid <name>`. **This names the solid the file describes**,
    /// not the software that wrote it — the producer belongs in
    /// [`Self::header`], which is the field the format leaves free.
    ///
    /// Printable ASCII (`0x20..=0x7E`) only — the grammar is one line,
    /// so anything else is a typed refusal rather than a silently
    /// mangled file. The empty string is accepted and writes
    /// `solid` followed by a trailing space; the format admits it and
    /// nothing downstream objects.
    ///
    /// Defaults to a generic part name (`ascii::DEFAULT_SOLID_NAME`),
    /// which carries no project or producer identity — so the exported
    /// bytes do not depend on Q9.
    pub solid_name: String,
    /// The binary writer's header text, zero-padded to the format's
    /// 80-byte header field. Free text by the format; conventionally
    /// the producer, which is why the default names this writer rather
    /// than the part.
    ///
    /// At most 80 bytes (measured in **bytes**, not characters), and it
    /// may not read as the ASCII-STL `solid` keyword — see
    /// [`StlError::HeaderSniffsAscii`] for how wide that check is and
    /// why. Both are typed refusals. The empty string is accepted and
    /// writes 80 zero bytes.
    ///
    /// Defaults to a constant description of the producer
    /// (`binary::DEFAULT_HEADER`) carrying no timestamp, version, or
    /// path.
    pub header: String,
}

impl Default for StlOptions {
    fn default() -> Self {
        Self {
            solid_name: ascii::DEFAULT_SOLID_NAME.to_owned(),
            header: binary::DEFAULT_HEADER.to_owned(),
        }
    }
}

impl StlOptions {
    /// The ASCII writer's precondition on [`Self::solid_name`], checked
    /// before any byte is written so a refusal never leaves a partial
    /// file.
    pub(crate) fn check_solid_name(&self) -> Result<(), StlError> {
        // The bound is BOTH-SIDED on purpose: below 0x20 are the
        // control characters that break the one-line grammar, and at or
        // above 0x7F are DEL and everything non-ASCII, which no STL
        // reader agrees on the encoding of.
        match self.solid_name.chars().find(|c| !(' '..='~').contains(c)) {
            Some(character) => Err(StlError::UnrepresentableSolidName { character }),
            None => Ok(()),
        }
    }

    /// The binary writer's precondition on [`Self::header`], checked
    /// before any byte is written so a refusal never leaves a partial
    /// file.
    pub(crate) fn check_header(&self) -> Result<(), StlError> {
        if self.header.len() > 80 {
            return Err(StlError::HeaderTooLong {
                len: self.header.len(),
            });
        }
        if sniffs_as_ascii(&self.header) {
            return Err(StlError::HeaderSniffsAscii);
        }
        Ok(())
    }
}

/// Whether `header` would make a binary file read as ASCII STL in a
/// reader that decides by the `solid` keyword.
///
/// **Wider than a byte-exact prefix, deliberately.** Sniffers in the
/// wild skip leading whitespace and fold case before comparing, so a
/// header of `" Solid v2"` is misread by some readers and not others —
/// which is worse than either. The check covers the whole class the
/// keyword can be recognised through, and the cost is that a header
/// which merely *starts* with those five letters (`SolidWorks export`)
/// is refused too. A refusal the caller can see beats a file another
/// tool silently reads as text.
fn sniffs_as_ascii(header: &str) -> bool {
    let head = header.trim_start().as_bytes();
    head.len() >= 5 && head[..5].eq_ignore_ascii_case(b"solid")
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
            // (Computing
            // it from the f32-narrowed vertices instead was tried and
            // rejected: apex-fan slivers become EXACTLY collinear
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
