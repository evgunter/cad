//! The ASCII STL writer (deterministic text layout, crate docs).

use std::io::Write;

use crate::{StlError, StlOptions, facets};

/// The solid name written into an ASCII export the caller supplies no
/// name for: constant in this build, with no run-dependent content, so
/// identical meshes give byte-identical files. A caller who sets
/// [`StlOptions::solid_name`] supplies a constant of its own — the
/// writer reads no clock, no environment and no global state, so the
/// same byte-identity holds for every name, not only this one.
pub(crate) const DEFAULT_SOLID_NAME: &str = "cad-kernel";

/// Writes `mesh` as ASCII STL: the standard `solid <name>` grammar with
/// [`StlOptions::solid_name`], closed by the matching `endsolid`. Every
/// float is the **f32** value (the same narrowing the binary writer
/// performs) formatted with Rust's default shortest-round-trip
/// `Display` — deterministic, and bit-exact under re-parse
/// (`str::parse::<f32>` returns the identical bits; the parse-back
/// equivalence with the binary writer is under test). Triangles stream
/// in patch order then triangle order, exactly as stored.
///
/// # Errors
///
/// [`StlError`] — a solid name outside the single-line grammar's
/// printable ASCII (refused before any byte is written), a degenerate
/// triangle (no honest normal), a corrupt index, or an I/O failure.
pub fn write_ascii(
    mesh: &mesh::Mesh,
    options: &StlOptions,
    out: &mut impl Write,
) -> Result<(), StlError> {
    options.check_solid_name()?;
    let name = &options.solid_name;
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
