//! Planar-face tessellation: projection into the face plane, CDT with
//! boundary constraints, even-odd parity filtering (rings and
//! non-convex boundaries), outward winding from the outer loop's
//! projected orientation.
//!
//! The plane is affine, so no deviation certificate is needed
//! (deviation 0 in exact arithmetic; boundary chord points carry the
//! kernel's ≤ ε carrier residual, crate docs). Interior sampling is
//! unnecessary — boundary points alone triangulate the region.
//!
//! Parity note: a full-2π revolve's annulus wall is a *slit* polygon —
//! one loop traversing its seam segment twice with bitwise-identical
//! projected points. The CDT dedupes the repeated points to the same
//! handles (same positions ⇒ same ids), duplicate constraints are
//! no-ops, and the slit's two opposite traversals cancel in the
//! even-odd crossing count, so parity classifies exactly as if the
//! slit were absent.

use std::collections::HashMap;

use geom_core::{Point3, Vec3};
use spade::{ConstrainedDelaunayTriangulation, Point2 as SpadePoint, Triangulation};
use topo::{Body, EdgeKey, FaceKey, LoopKey};

use crate::types::TessellateError;
use crate::walk::loop_edges;

/// Tessellates one planar face into outward-wound triangles.
pub(crate) fn tessellate_planar(
    body: &Body<f64>,
    fk: FaceKey,
    origin: Point3<f64>,
    normal: Vec3<f64>,
    u_ref: Vec3<f64>,
    chords: &HashMap<EdgeKey, Vec<u32>>,
    positions: &[Point3<f64>],
) -> Result<Vec<[u32; 3]>, TessellateError> {
    let face = body
        .get_face(fk)
        .ok_or(TessellateError::MissingEntity { what: "face" })?;
    let v_ref = normal.cross(u_ref);
    let project = |id: u32| -> [f64; 2] {
        let w = positions[id as usize] - origin;
        [w.dot(u_ref), w.dot(v_ref)]
    };

    // Loop id cycles (outer first, then rings in face order).
    let mut loops: Vec<Vec<u32>> = Vec::with_capacity(1 + face.rings.len());
    for lk in core::iter::once(face.outer).chain(face.rings.iter().copied()) {
        loops.push(loop_ids(body, fk, lk, chords)?);
    }

    // CDT: points in loop order, then boundary constraints.
    let mut cdt: ConstrainedDelaunayTriangulation<SpadePoint<f64>> =
        ConstrainedDelaunayTriangulation::new();
    let mut meta: Vec<u32> = Vec::new(); // handle index -> mesh id
    let mut polygons: Vec<Vec<[f64; 2]>> = Vec::new();
    for ids in &loops {
        let mut handles = Vec::with_capacity(ids.len());
        let mut poly = Vec::with_capacity(ids.len());
        for &id in ids {
            let [u, v] = project(id);
            poly.push([u, v]);
            let h = cdt
                .insert(SpadePoint::new(u, v))
                .map_err(|_| TessellateError::Triangulation { face: fk })?;
            if h.index() == meta.len() {
                meta.push(id);
            }
            handles.push(h);
        }
        for i in 0..handles.len() {
            let (a, b) = (handles[i], handles[(i + 1) % handles.len()]);
            if a != b && !cdt.exists_constraint(a, b) {
                // `add_constraint` panics on crossing constraints
                // (corrupt input geometry) — pre-check to keep failure
                // typed. The slit's second traversal is skipped by the
                // `exists_constraint` guard.
                if !cdt.can_add_constraint(a, b) {
                    return Err(TessellateError::Triangulation { face: fk });
                }
                cdt.add_constraint(a, b);
            }
        }
        polygons.push(poly);
    }

    // Outer-loop orientation in the chart frame decides the flip.
    let flip = shoelace2(&polygons[0]) < 0.0;

    // Emit parity-interior triangles.
    let mut triangles = Vec::new();
    for f in cdt.inner_faces() {
        let vs = f.vertices();
        let ids = [
            meta[vs[0].fix().index()],
            meta[vs[1].fix().index()],
            meta[vs[2].fix().index()],
        ];
        if ids[0] == ids[1] || ids[1] == ids[2] || ids[0] == ids[2] {
            continue; // slit-degenerate sliver
        }
        let c = f.center();
        if !parity_inside(c.x, c.y, &polygons) {
            continue;
        }
        triangles.push(if flip { [ids[0], ids[2], ids[1]] } else { ids });
    }
    Ok(triangles)
}

/// The loop's chord ids in walk order (each traversal contributes its
/// points minus the last, which the next traversal repeats).
fn loop_ids(
    body: &Body<f64>,
    fk: FaceKey,
    lk: LoopKey,
    chords: &HashMap<EdgeKey, Vec<u32>>,
) -> Result<Vec<u32>, TessellateError> {
    let mut out = Vec::new();
    for (ek, forward) in loop_edges(body, lk, fk)? {
        let ids = chords.get(&ek).ok_or(TessellateError::MissingEntity {
            what: "edge chords",
        })?;
        if forward {
            out.extend_from_slice(&ids[..ids.len() - 1]);
        } else {
            out.extend(ids[1..].iter().rev());
        }
    }
    Ok(out)
}

/// Twice the signed area of a 2-D polygon.
fn shoelace2(poly: &[[f64; 2]]) -> f64 {
    let mut s = 0.0;
    for (i, p) in poly.iter().enumerate() {
        let q = poly[(i + 1) % poly.len()];
        s += p[0] * q[1] - q[0] * p[1];
    }
    s
}

/// Even-odd point-in-region test against every boundary polygon
/// (half-open crossing rule; deterministic f64 arithmetic).
fn parity_inside(x: f64, y: f64, polygons: &[Vec<[f64; 2]>]) -> bool {
    let mut inside = false;
    for poly in polygons {
        for (i, a) in poly.iter().enumerate() {
            let b = poly[(i + 1) % poly.len()];
            if (a[1] > y) != (b[1] > y) {
                let x_int = a[0] + (y - a[1]) * (b[0] - a[0]) / (b[1] - a[1]);
                if x_int > x {
                    inside = !inside;
                }
            }
        }
    }
    inside
}
