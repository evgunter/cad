//! The binary STL writer (deterministic byte layout, crate docs).

use std::io::Write;

use crate::{StlError, facets};

/// The fixed 80-byte header: constant bytes — no timestamps, versions,
/// or pointers (D9: byte-identical output for identical inputs). Must
/// not begin with `solid` (some parsers sniff ASCII STL that way).
/// Padded to 80 bytes with zeros at write time.
const HEADER: &[u8] = b"binary STL; CAD kernel M2 tessellation export";

/// Writes `mesh` as binary STL: the 80-byte constant header, the
/// little-endian `u32` triangle count, then per triangle the f32
/// normal, three f32 vertices, and a zero `u16` attribute byte count.
/// All multi-byte values are written with explicit little-endian
/// `to_le_bytes` — never platform-dependent memory dumps. Triangles
/// stream in patch order then triangle order, exactly as stored (zero
/// writer-added nondeterminism; crate docs).
///
/// # Errors
///
/// [`StlError`] — degenerate triangle (no honest normal), corrupt
/// index, a triangle count exceeding `u32`, or an I/O failure.
pub fn write_binary(mesh: &mesh::Mesh, out: &mut impl Write) -> Result<(), StlError> {
    let facets = facets(mesh)?;
    let count = u32::try_from(facets.len()).map_err(|_| StlError::TooManyTriangles {
        count: facets.len(),
    })?;

    let mut header = [0u8; 80];
    header[..HEADER.len()].copy_from_slice(HEADER);
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
