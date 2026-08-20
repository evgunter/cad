//! The binary STL writer (deterministic byte layout, crate docs).

use std::io::Write;

use crate::{StlError, StlOptions, facets};

/// The 80-byte header written when the caller supplies none: constant
/// bytes — no timestamps, versions, or pointers (D9: byte-identical
/// output for identical inputs). A caller-supplied
/// [`StlOptions::header`] is the caller's own constant; the writer
/// reads no clock and no global state either way. Must not begin with
/// `solid` (some parsers sniff ASCII STL that way) — a constraint the
/// writer now enforces on every header rather than only satisfying on
/// this one. Padded to 80 bytes with zeros at write time.
pub(crate) const DEFAULT_HEADER: &str = "binary STL; CAD kernel tessellation export";

/// Writes `mesh` as binary STL: the 80-byte header from
/// [`StlOptions::header`], the little-endian `u32` triangle count, then
/// per triangle the f32 normal, three f32 vertices, and a zero `u16`
/// attribute byte count. All multi-byte values are written with
/// explicit little-endian `to_le_bytes` — never platform-dependent
/// memory dumps. Triangles stream in patch order then triangle order,
/// exactly as stored (zero writer-added nondeterminism; crate docs).
///
/// # Errors
///
/// [`StlError`] — a header over 80 bytes or one that would sniff as
/// ASCII STL (both refused before any byte is written), a degenerate
/// triangle (no honest normal), a corrupt index, a triangle count
/// exceeding `u32`, or an I/O failure.
pub fn write_binary(
    mesh: &mesh::Mesh,
    options: &StlOptions,
    out: &mut impl Write,
) -> Result<(), StlError> {
    options.check_header()?;
    let facets = facets(mesh)?;
    let count = u32::try_from(facets.len()).map_err(|_| StlError::TooManyTriangles {
        count: facets.len(),
    })?;

    let mut header = [0u8; 80];
    let text = options.header.as_bytes();
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
