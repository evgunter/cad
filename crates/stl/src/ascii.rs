//! The ASCII STL writer (deterministic text layout, crate docs).

use std::io::Write;

use crate::{AsciiOptions, StlError, facets};

/// The solid name written into an ASCII export the caller supplies no
/// name for: constant in this build, with no run-dependent content, so
/// identical meshes give byte-identical files. A caller-supplied
/// [`SolidName`](crate::SolidName) is written verbatim and the writer
/// still reads no clock, no environment and no global state, so
/// byte-identity follows from the pair `(mesh, options)` for every
/// name, not only this one.
///
/// It is a **generic part name**, not a producer or project identity:
/// `solid <name>` names the solid the file describes, so the producer
/// belongs in the binary header instead. This also keeps the exported
/// bytes independent of Q9.
pub(crate) const DEFAULT_SOLID_NAME: &str = "part";

/// Writes `mesh` as ASCII STL: the standard `solid <name>` grammar with
/// [`AsciiOptions::solid_name`], closed by the matching `endsolid`. Every
/// float is the **f32** value (the same narrowing the binary writer
/// performs) formatted with Rust's default shortest-round-trip
/// `Display` — deterministic, and bit-exact under re-parse
/// (`str::parse::<f32>` returns the identical bits; the parse-back
/// equivalence with the binary writer is under test). Triangles stream
/// in patch order then triangle order, exactly as stored.
///
/// # Errors
///
/// [`StlError`] — a degenerate triangle (no honest normal), a corrupt
/// index, or an I/O failure. **Nothing about the name**: a
/// [`SolidName`](crate::SolidName) that exists is already writable.
pub fn write_ascii(
    mesh: &mesh::Mesh,
    options: &AsciiOptions,
    out: &mut impl Write,
) -> Result<(), StlError> {
    let name = options.solid_name.as_str();
    let facets = facets(mesh)?;
    writeln!(out, "solid {name}")?;
    for facet in &facets {
        let [nx, ny, nz] = facet.normal.map(|v| v as f32);
        writeln!(out, "facet normal {nx} {ny} {nz}")?;
        writeln!(out, "  outer loop")?;
        for v in &facet.vertices {
            let [x, y, z] = v.map(|c| c as f32);
            writeln!(out, "    vertex {x} {y} {z}")?;
        }
        writeln!(out, "  endloop")?;
        writeln!(out, "endfacet")?;
    }
    writeln!(out, "endsolid {name}")?;
    Ok(())
}
