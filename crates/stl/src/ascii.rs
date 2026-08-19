//! The ASCII STL writer (deterministic text layout, crate docs).

use std::io::Write;

use crate::{StlError, facets};

/// The constant solid name (deterministic; no run-dependent content,
/// no caller input — identical meshes give byte-identical files).
const NAME: &str = "cad-kernel";

/// Writes `mesh` as ASCII STL: the standard `solid <name>` grammar
/// with the constant name [`NAME`]. Every float is the **f32** value
/// (the same narrowing the binary writer performs) formatted with
/// Rust's default shortest-round-trip `Display` — deterministic, and
/// bit-exact under re-parse (`str::parse::<f32>` returns the identical
/// bits; the parse-back equivalence with the binary writer is under
/// test). Triangles stream in patch order then triangle order, exactly
/// as stored.
///
/// # Errors
///
/// [`StlError`] — degenerate triangle (no honest normal), corrupt
/// index, or an I/O failure.
pub fn write_ascii(mesh: &mesh::Mesh, out: &mut impl Write) -> Result<(), StlError> {
    let facets = facets(mesh)?;
    writeln!(out, "solid {NAME}")?;
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
    writeln!(out, "endsolid {NAME}")?;
    Ok(())
}
