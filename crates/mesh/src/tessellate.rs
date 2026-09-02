//! The [`tessellate`] entry point: δ validation, deterministic mesh
//! vertex minting, the chord pass, and per-face dispatch.

use std::collections::HashMap;

use geom::Surface;
use geom_core::Tol;
use topo::Body;

use crate::chords::{compute_chords, edge_vertices};
use crate::curved::tessellate_curved;
use crate::nurbs_cert::FaceBounds;
use crate::planar::tessellate_planar;
use crate::sizing::{Eps, SizingTols, sizing_target};
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
/// garbage-in/garbage-out on the mesh *values*.
/// [`crate::validate::check_mesh`] is the backstop for that, and
/// **this function does not call it**: it is available to a caller,
/// and the acceptance suites run it, but nothing on this path does.
///
/// # Errors
///
/// [`TessellateError`] (closed enum): invalid δ, the `Nurbs`
/// placeholder, described NURBS faces outside the trimmed-NURBS
/// inventory (illegal-rational / C⁰-creased — `nurbs_cert`), unsupported
/// carriers, rings on curved faces, a curved face whose iso domain is
/// not its own UV rectangle, empty loops, dangling keys, resolution
/// overflow, certificate failure, CDT insertion failure.
pub fn tessellate(body: &Body<f64>, chordal: f64, tol: Tol) -> Result<Mesh, TessellateError> {
    if !(chordal.is_finite() && chordal > 0.0) {
        return Err(TessellateError::InvalidChordalTolerance { value: chordal });
    }
    let eps = Eps::at(tol);
    let delta_s = sizing_target(chordal);

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

    // Certified whole-patch NURBS bounds, assembled once per face and
    // shared by both passes that need them (`chords::FaceBounds`).
    let mut bounds = FaceBounds::new();

    // Chord pass: per-edge polylines, computed once (crate docs);
    // `chord_ts` is the matching parameter schedule (the trimmed lane
    // evaluates pcurves on it — one derivation, both consumers).
    let chords = compute_chords(body, delta_s, &vids, &mut positions, &mut bounds)?;
    let mut boundaries = Vec::new();
    for (ek, _) in body.edges() {
        let (start_vertex, end_vertex) = edge_vertices(body, ek)?;
        let points = chords
            .ids
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
        let tol = SizingTols {
            delta: chordal,
            delta_s,
            eps,
        };
        let triangles = match *surface {
            // Described NURBS faces route through the trimmed lane
            // unconditionally (M7 — the flip of the historical
            // first-arm refusal, whose record is on
            // [`TessellateError::UnsupportedSurface`]): a NURBS face
            // has no swept-rectangle chart, so the pcurve-driven walk
            // is its only lane. The placeholder still refuses typed
            // inside the lane; illegal-rational/C⁰ classes refuse
            // [`TessellateError::UnsupportedNurbsFace`] there too.
            // An approximating surface meshes through the SAME lane,
            // on its fit: the fit is the geometry, so the triangles it
            // produces are the face's own. The certificate's bound is
            // deliberately NOT folded into the mesh tolerance here —
            // widening `tol` by the fit's ε so the mesh certifies
            // against the DESCRIPTION is a separate statement, and
            // this pass makes the plain one.
            Surface::Nurbs(_) | Surface::Approx(_) => crate::trimmed::tessellate_trimmed(
                body,
                fk,
                surface,
                &chords,
                &mut positions,
                &tol,
                &mut bounds,
            )?,
            // The planar lane derives its chart frame from the face's
            // own boundary (planar.rs module docs, #284) — the stored
            // plane axes are deliberately not passed: imported axes
            // carry translator noise that projects valid boundaries
            // below spade's coordinate domain.
            Surface::Plane { .. } => tessellate_planar(body, fk, &chords.ids, &positions)?,
            // Structural routing (M5 PR 11): a conic/B-spline trim
            // carrier means the face is not an iso-rectangle — the
            // pcurve-driven trimmed lane takes it.
            //
            // The converse does NOT follow: this is a test on carrier
            // KINDS, and iso
            // carriers (`Line`, `Circle`) can bound a NON-rectangular
            // domain — a keyway or milled flat on a cylinder is exactly
            // that shape, and nothing on this path screens loop SHAPE.
            // So an iso boundary reaching `tessellate_curved` is a
            // routing decision, not a guarantee about the domain; the
            // domain itself is checked there
            // (`curved::require_swept_rectangle`, refusing
            // [`TessellateError::UnsupportedCurvedDomain`]).
            _ if crate::trimmed::has_trim_carrier(body, fk)? => crate::trimmed::tessellate_trimmed(
                body,
                fk,
                surface,
                &chords,
                &mut positions,
                &tol,
                &mut bounds,
            )?,
            _ => tessellate_curved(body, fk, surface, &chords.ids, &mut positions, &tol)?,
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
