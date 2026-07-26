//! Planar-face tessellation: projection into the face plane, CDT with
//! boundary constraints, even-odd region classification (rings and
//! non-convex boundaries), outward winding from the outer loop's
//! projected orientation.
//!
//! The plane is affine, so no deviation certificate is needed
//! (deviation 0 in exact arithmetic; boundary chord points carry the
//! kernel's ≤ ε carrier residual, crate docs). Interior sampling is
//! unnecessary — boundary points alone triangulate the region.
//!
//! # Region classification is combinatorial, not geometric
//!
//! Which CDT triangles are *inside* the face is decided by a flood fill
//! over the triangulation's face-adjacency graph, seeded at the outer
//! face (outside every loop) and toggling only when the walk crosses a
//! boundary constraint edge. This computes exactly the even-odd rule —
//! parity of boundary crossings on a path from infinity — but as a
//! graph traversal, with **no floating-point decision at all**.
//!
//! The predecessor tested each triangle's centroid against the boundary
//! polylines by ray casting. That is unsound for needle triangles
//! riding the boundary, and issue #111 caught it in the wild: an A×Z
//! boolean face carries four boundary vertices on one carrier line, one
//! of them 1 ulp off it (an exact 5/8 crossing reached through
//! non-dyadic slope-3/5 arithmetic). The needle the CDT correctly
//! builds there is ~3e-18 thick, while merely *forming* a centroid
//! rounds its coordinates by ~5e-17 — the constructed point lands on
//! the wrong side of the boundary edge before any predicate runs, so no
//! amount of exactness in the test could rescue it. The classification
//! had to stop depending on a constructed point.
//!
//! Two structural consequences, both load-bearing:
//!
//! * Across a boundary edge the classification necessarily *differs*,
//!   so every boundary segment is an edge of exactly one kept triangle
//!   on this face — watertightness by construction rather than by
//!   numerical luck.
//! * A degenerate needle may still be kept (a zero-area sliver inside
//!   the region) or dropped (outside it), but never emitted on the
//!   outside — which is the defect that leaks meshes.
//!
//! Slit note: a full-2π revolve's annulus wall is a *slit* polygon —
//! one loop traversing its seam segment twice with bitwise-identical
//! projected points. The CDT dedupes the repeated points to the same
//! handles (same positions ⇒ same ids), so both traversals land on the
//! same constraint edges. Crossing multiplicity is therefore counted,
//! not merely recorded: the fill toggles only on **odd** multiplicity,
//! so the slit's doubled traversal cancels and classification proceeds
//! exactly as if the slit were absent — the same cancellation the
//! even-odd crossing count used to get, now exact.

use std::collections::HashMap;

use geom_core::{Point3, Vec3};
use spade::{
    ConstrainedDelaunayTriangulation, Point2 as SpadePoint, Triangulation,
    handles::{DirectedEdgeHandle, FixedFaceHandle, FixedVertexHandle, InnerTag},
};
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

    // CDT: every loop's points first, then the boundary constraints.
    // The two passes must not interleave: inserting a vertex that lands
    // exactly on an existing constraint edge splits it, which would
    // invalidate the crossing bookkeeping built below.
    let mut cdt: ConstrainedDelaunayTriangulation<SpadePoint<f64>> =
        ConstrainedDelaunayTriangulation::new();
    let mut meta: Vec<u32> = Vec::new(); // handle index -> mesh id
    let mut polygons: Vec<Vec<[f64; 2]>> = Vec::new();
    let mut handles: Vec<Vec<FixedVertexHandle>> = Vec::new();
    for ids in &loops {
        let mut hs = Vec::with_capacity(ids.len());
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
            hs.push(h);
        }
        handles.push(hs);
        polygons.push(poly);
    }

    // Boundary constraints, counting how often each resulting CDT edge
    // is traversed. A loop segment may be realised by several constraint
    // edges (an intervening vertex splits it), and the slit's two
    // traversals return the same edges twice — both handled by keying on
    // the sub-edges spade reports rather than on the segment endpoints.
    let mut crossings: HashMap<(usize, usize), u32> = HashMap::new();
    for hs in &handles {
        for i in 0..hs.len() {
            let (a, b) = (hs[i], hs[(i + 1) % hs.len()]);
            if a == b {
                continue; // slit-collapsed segment: no boundary at all
            }
            // `try_add_constraint` reports the constraint edges joining
            // `a` and `b`, including pre-existing ones, and leaves the
            // triangulation untouched (returning empty) when the segment
            // would cross an existing constraint — corrupt input
            // geometry, kept a typed failure rather than a panic.
            let realised = cdt.try_add_constraint(a, b);
            if realised.is_empty() {
                return Err(TessellateError::Triangulation { face: fk });
            }
            for e in realised {
                let e = cdt.directed_edge(e);
                *crossings.entry(edge_key(e)).or_insert(0) += 1;
            }
        }
    }

    // Outer-loop orientation in the chart frame decides the flip.
    let flip = shoelace2(&polygons[0]) < 0.0;

    let inside = classify_faces(&cdt, &crossings);

    // Emit the interior triangles.
    let mut triangles = Vec::new();
    for f in cdt.inner_faces() {
        if !inside[f.fix().index()] {
            continue;
        }
        let vs = f.vertices();
        let ids = [
            meta[vs[0].fix().index()],
            meta[vs[1].fix().index()],
            meta[vs[2].fix().index()],
        ];
        if ids[0] == ids[1] || ids[1] == ids[2] || ids[0] == ids[2] {
            continue; // slit-degenerate sliver
        }
        triangles.push(if flip { [ids[0], ids[2], ids[1]] } else { ids });
    }
    Ok(triangles)
}

/// A CDT edge's identity as an unordered pair of vertex indices.
fn edge_key(e: DirectedEdgeHandle<'_, SpadePoint<f64>, (), spade::CdtEdge<()>, ()>) -> (usize, usize) {
    let (p, q) = (e.from().fix().index(), e.to().fix().index());
    (p.min(q), p.max(q))
}

/// Even-odd interior flag per CDT face, indexed by face index (0 is
/// spade's outer face, which is outside every loop).
///
/// A breadth-first walk of the face-adjacency graph from the outer face,
/// toggling the flag exactly when it steps across a boundary segment
/// traversed an odd number of times. Purely combinatorial: the result
/// depends on the triangulation's connectivity and on integer crossing
/// counts, never on a coordinate comparison.
fn classify_faces(
    cdt: &ConstrainedDelaunayTriangulation<SpadePoint<f64>>,
    crossings: &HashMap<(usize, usize), u32>,
) -> Vec<bool> {
    let toggles = |e| crossings.get(&edge_key(e)).is_some_and(|n| n % 2 == 1);
    let mut inside = vec![false; cdt.num_all_faces()];
    let mut seen = vec![false; cdt.num_all_faces()];
    let mut queue: Vec<FixedFaceHandle<InnerTag>> = Vec::new();

    // Seed: step inwards across every convex-hull edge. The hull bounds
    // the outer face, which is outside every loop; a hull edge that is
    // itself a boundary segment toggles on the way in.
    for hull in cdt.convex_hull() {
        let e = if hull.face().is_outer() { hull.rev() } else { hull };
        let Some(f) = e.face().as_inner() else {
            continue; // both sides outer: a degenerate, edge-only CDT
        };
        let i = f.fix().index();
        if !seen[i] {
            seen[i] = true;
            inside[i] = toggles(e);
            queue.push(f.fix());
        }
    }

    let mut head = 0;
    while head < queue.len() {
        let f = queue[head];
        head += 1;
        let here = inside[f.index()];
        for e in cdt.face(f).adjacent_edges() {
            let Some(g) = e.rev().face().as_inner() else {
                continue; // the outer face, already the walk's origin
            };
            let j = g.fix().index();
            if !seen[j] {
                seen[j] = true;
                inside[j] = here ^ toggles(e);
                queue.push(g.fix());
            }
        }
    }
    inside
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
