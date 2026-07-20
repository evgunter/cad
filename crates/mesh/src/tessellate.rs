//! The [`tessellate`] entry point: δ validation, deterministic mesh
//! vertex minting, the chord pass, and per-face dispatch.

use std::collections::HashMap;

use geom_core::Tolerance;
use geom_surfaces::Surface;
use topo::Body;

use crate::chords::{compute_chords, edge_vertices};
use crate::curved::tessellate_curved;
use crate::planar::tessellate_planar;
use crate::types::{BoundaryPolyline, FacePatch, Mesh, TessellateError};

/// Tessellates a closed body into a watertight [`Mesh`] within the
/// chordal tolerance `chordal` (δ, meters) of its exact surfaces.
///
/// δ is a per-call display/export parameter, deliberately not the
/// kernel ε — see the crate docs for the distinction, the
/// certified-conservative bound, the pure-function invariant, and the
/// determinism contract (byte-identical mesh for identical
/// `(body, chordal)`).
///
/// The input is expected to be a closed solid at rest (tier 2, with
/// tier-3 geometry); tessellation does not re-validate — corrupt input
/// surfaces as typed errors where cheaply detectable (dangling keys,
/// `Nurbs` placeholders, certificate failures) and is otherwise
/// garbage-in/garbage-out on the mesh *values* while
/// [`crate::validate::check_mesh`] stays available as the backstop.
///
/// # Errors
///
/// [`TessellateError`] (closed enum): invalid δ, `Nurbs`
/// surfaces/carriers, rings on curved faces, empty loops, dangling
/// keys, resolution overflow, certificate failure, CDT insertion
/// failure.
pub fn tessellate(body: &Body<f64>, chordal: f64) -> Result<Mesh, TessellateError> {
    if !(chordal.is_finite() && chordal > 0.0) {
        return Err(TessellateError::InvalidChordalTolerance { value: chordal });
    }
    let eps = Tolerance::get().eps;
    let delta_s = chordal * 0.5;

    // Mesh vertex ids: topology vertices first, arena order (D9).
    let mut positions = Vec::new();
    let mut vids = HashMap::new();
    for (vk, v) in body.vertices() {
        let p = body
            .get_point(v.point)
            .ok_or(TessellateError::MissingEntity {
                what: "vertex point",
            })?;
        #[allow(clippy::cast_possible_truncation)]
        vids.insert(vk, positions.len() as u32);
        positions.push(*p);
    }

    // Chord pass: per-edge polylines, computed once (crate docs).
    let chords = compute_chords(body, delta_s, &vids, &mut positions)?;
    let mut boundaries = Vec::new();
    for (ek, _) in body.edges() {
        let (start_vertex, end_vertex) = edge_vertices(body, ek)?;
        let points = chords
            .get(&ek)
            .ok_or(TessellateError::MissingEntity {
                what: "edge chords",
            })?
            .clone();
        boundaries.push(BoundaryPolyline {
            edge: ek,
            points,
            start_vertex,
            end_vertex,
        });
    }

    // Per-face dispatch, face-arena order.
    let mut patches = Vec::new();
    for (fk, face) in body.faces() {
        let surface = body
            .get_surface(face.surface)
            .ok_or(TessellateError::MissingEntity {
                what: "face surface",
            })?;
        let triangles = match *surface {
            Surface::Nurbs => return Err(TessellateError::UnsupportedSurface { face: fk }),
            Surface::Plane {
                origin,
                normal,
                u_ref,
            } => tessellate_planar(body, fk, origin, normal, u_ref, &chords, &positions)?,
            _ => tessellate_curved(
                body,
                fk,
                surface,
                &chords,
                &mut positions,
                &crate::curved::Tol {
                    delta: chordal,
                    delta_s,
                    eps,
                },
            )?,
        };
        patches.push(FacePatch {
            face: fk,
            triangles,
        });
    }

    Ok(Mesh {
        positions,
        patches,
        boundaries,
    })
}
