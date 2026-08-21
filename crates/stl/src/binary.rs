//! The binary STL writer (deterministic byte layout, crate docs).

use std::io::Write;

use crate::{BinaryOptions, StlError, facets};

/// The 80-byte header written when the caller supplies none: constant
/// bytes — no timestamps, versions, or pointers (D9: same mesh + same
/// options ⇒ byte-identical output). A caller-supplied
/// [`BinaryHeader`](crate::BinaryHeader) is written verbatim; the
/// writer reads no clock and no global state either way.
///
/// This is the format's **free text**, conventionally the producer,
/// which is why this default names the writer while the ASCII solid
/// name defaults to a part. It must not read as the ASCII-STL `solid`
/// keyword (some parsers sniff that way) — a constraint
/// [`BinaryHeader::new`](crate::BinaryHeader::new) **enforces on
/// every header**, over the whole class a whitespace-skipping,
/// case-folding sniffer recognises. Padded to 80 bytes with zeros at
/// write time.
pub(crate) const DEFAULT_HEADER: &str = "binary STL; CAD kernel tessellation export";

/// Writes `mesh` as binary STL: the 80-byte header from
/// [`BinaryOptions::header`], the little-endian `u32` triangle count, then
/// per triangle the f32 normal, three f32 vertices, and a zero `u16`
/// attribute byte count. All multi-byte values are written with
/// explicit little-endian `to_le_bytes` — never platform-dependent
/// memory dumps. Triangles stream in patch order then triangle order,
/// exactly as stored (zero writer-added nondeterminism; crate docs).
///
/// # Errors
///
/// [`StlError`] — a degenerate triangle (no honest normal), a corrupt
/// index, a triangle count exceeding `u32`, or an I/O failure.
/// **Nothing about the header**: a
/// [`BinaryHeader`](crate::BinaryHeader) that exists already fits the
/// field and already does not sniff.
pub fn write_binary(
    mesh: &mesh::Mesh,
    options: &BinaryOptions,
    out: &mut impl Write,
) -> Result<(), StlError> {
    let facets = facets(mesh)?;
    let count = u32::try_from(facets.len()).map_err(|_| StlError::TooManyTriangles {
        count: facets.len(),
    })?;

    let mut header = [0u8; 80];
    let text = options.header.as_str().as_bytes();
    header[..text.len()].copy_from_slice(text);
    out.write_all(&header)?;
    out.write_all(&count.to_le_bytes())?;
    for facet in &facets {
        for &n in &facet.normal {
            out.write_all(&(n as f32).to_le_bytes())?;
        }
        for v in &facet.vertices {
            for &c in v {
                out.write_all(&(c as f32).to_le_bytes())?;
            }
        }
        out.write_all(&0u16.to_le_bytes())?;
    }
    Ok(())
}
