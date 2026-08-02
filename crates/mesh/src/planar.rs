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

    // Loop id cycles (outer first, then rings in face order), each
    // paired with its projection into the chart.
    let mut loops: Vec<Vec<u32>> = Vec::with_capacity(1 + face.rings.len());
    let mut polygons: Vec<Vec<[f64; 2]>> = Vec::with_capacity(1 + face.rings.len());
    for lk in core::iter::once(face.outer).chain(face.rings.iter().copied()) {
        let ids = loop_ids(body, fk, lk, chords)?;
        polygons.push(ids.iter().map(|&id| project(id)).collect());
        loops.push(ids);
    }

    triangulate_chart(fk, &loops, &polygons)
}

/// Triangulates a face already reduced to its chart: `loops[i][j]` is
/// the mesh id of `polygons[i][j]`, outer loop first. Split out from
/// [`tessellate_planar`] so that a face's exact projected coordinates
/// can be replayed in isolation (the #111 regression is a property of
/// the chart alone).
fn triangulate_chart(
    fk: FaceKey,
    loops: &[Vec<u32>],
    polygons: &[Vec<[f64; 2]>],
) -> Result<Vec<[u32; 3]>, TessellateError> {
    // CDT: every loop's points first, then the boundary constraints.
    // The two passes must not interleave: inserting a vertex that lands
    // exactly on an existing constraint edge splits it, which would
    // invalidate the crossing bookkeeping built below.
    let mut cdt: ConstrainedDelaunayTriangulation<SpadePoint<f64>> =
        ConstrainedDelaunayTriangulation::new();
    let mut meta: Vec<u32> = Vec::new(); // handle index -> mesh id
    let mut handles: Vec<Vec<FixedVertexHandle>> = Vec::new();
    for (ids, poly) in loops.iter().zip(polygons) {
        let mut hs = Vec::with_capacity(ids.len());
        for (&id, &[u, v]) in ids.iter().zip(poly) {
            let h = cdt
                .insert(SpadePoint::new(u, v))
                .map_err(|_| TessellateError::Triangulation { face: fk })?;
            if h.index() == meta.len() {
                meta.push(id);
            }
            hs.push(h);
        }
        handles.push(hs);
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

    // Outer-loop orientation in the chart frame decides the flip. This
    // is an unreified float sign, but not an in-band one: getting it
    // wrong needs a face whose projected area is ~0, and it flips the
    // whole patch at once, so `check_mesh` catches it as
    // `MismatchedWinding` against the neighbouring faces — unlike the
    // #111 per-triangle decision, which could fail on one sliver and
    // leak past every downstream check but `check_mesh`'s edge census.
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
pub(crate) fn edge_key(
    e: DirectedEdgeHandle<'_, SpadePoint<f64>, (), spade::CdtEdge<()>, ()>,
) -> (usize, usize) {
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
pub(crate) fn classify_faces(
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
        let e = if hull.face().is_outer() {
            hull.rev()
        } else {
            hull
        };
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
pub(crate) fn shoelace2(poly: &[[f64; 2]]) -> f64 {
    let mut s = 0.0;
    for (i, p) in poly.iter().enumerate() {
        let q = poly[(i + 1) % poly.len()];
        s += p[0] * q[1] - q[0] * p[1];
    }
    s
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

    use super::{shoelace2, triangulate_chart};
    use topo::FaceKey;

    /// Face `19v3` of the issue-#111 A×Z intersect (A's left inner-leg
    /// slant), captured as the exact projected chart the tessellator
    /// builds — bit patterns, because the defect lives in the last ulp.
    ///
    /// Boundary walk `48 → 34 → 1 → 2 → 13 → 52 → 24 → 48`. Vertices
    /// `2`, `13`, `48` are exactly collinear (A's leg-top carrier);
    /// vertex `24`, the boolean's crossing of that carrier with Z's
    /// slope-3/5 diagonal, is exactly 5/8 in real arithmetic but lands
    /// 1 ulp off the carrier, and `13 → 52 → 24` is the #93 seam
    /// hexagon's notch dipping in between. The CDT therefore builds a
    /// needle triangle `[24, 13, 48]` of twice-area 4e-18 that lies
    /// OUTSIDE the face — verified present in the triangulation of this
    /// very chart, and the exact triangle the predecessor kept: forming
    /// a centroid rounds it by ~5e-17, ten times the needle's ~3e-18
    /// half-thickness, so the constructed point crosses to the interior
    /// side of boundary edge `24→48` and every point-in-polygon test,
    /// f64 or exact, then answers "inside".
    const FACE_19V3: [(u64, u64); 7] = [
        (4581431797661461788, 4602803313439314012),  // 48
        (4601278341968785480, 13826922315894092423), // 34
        (4606183720244804921, 13827710779582984058), // 1
        (4607527494452543132, 4599358972061524235),  // 2
        (4603999415142092400, 4600811405172640406),  // 13
        (4603471649560759920, 4590705563545250395),  // 52
        (4602000164000415209, 4601433876505975908),  // 24
    ];
    const FACE_19V3_IDS: [u32; 7] = [48, 34, 1, 2, 13, 52, 24];

    fn chart(bits: &[(u64, u64)]) -> Vec<[f64; 2]> {
        bits.iter()
            .map(|&(u, v)| [f64::from_bits(u), f64::from_bits(v)])
            .collect()
    }

    fn sorted(t: [u32; 3]) -> [u32; 3] {
        let mut t = t;
        t.sort_unstable();
        t
    }

    /// Twice the signed area of a triangle in the chart.
    fn tri2(poly: &[[f64; 2]], ids: &[u32], t: [u32; 3]) -> f64 {
        let at = |id: u32| poly[ids.iter().position(|&x| x == id).unwrap()];
        let (a, b, c) = (at(t[0]), at(t[1]), at(t[2]));
        (b[0] - a[0]) * (c[1] - a[1]) - (b[1] - a[1]) * (c[0] - a[0])
    }

    /// Every boundary segment must be an edge of exactly one emitted
    /// triangle — the per-face half of watertightness, and precisely
    /// what the centroid-parity filter broke (the shared segment went
    /// to 3 uses, its neighbours to 1).
    fn assert_covers_boundary(loops: &[Vec<u32>], tris: &[[u32; 3]]) {
        let mut uses: std::collections::HashMap<(u32, u32), u32> = std::collections::HashMap::new();
        for t in tris {
            for k in 0..3 {
                let (a, b) = (t[k], t[(k + 1) % 3]);
                *uses.entry((a.min(b), a.max(b))).or_insert(0) += 1;
            }
        }
        for ids in loops {
            for i in 0..ids.len() {
                let (a, b) = (ids[i], ids[(i + 1) % ids.len()]);
                if a == b {
                    continue;
                }
                let key = (a.min(b), a.max(b));
                assert_eq!(
                    uses.get(&key).copied().unwrap_or(0),
                    1,
                    "boundary segment {key:?} used {:?} times, want 1",
                    uses.get(&key)
                );
            }
        }
    }

    #[test]
    fn issue111_needle_on_the_az_leg_carrier_is_not_emitted() {
        let poly = chart(&FACE_19V3);
        let loops = vec![FACE_19V3_IDS.to_vec()];
        let tris = triangulate_chart(FaceKey::default(), &loops, std::slice::from_ref(&poly))
            .expect("face 19v3 triangulates");

        // The exterior needle the old centroid test kept.
        assert!(
            !tris.iter().any(|&t| sorted(t) == [13, 24, 48]),
            "REGRESSION (issue #111): exterior needle [24, 13, 48] emitted; \
             triangles = {tris:?}"
        );
        assert_covers_boundary(&loops, &tris);

        // The kept triangles must tile the region exactly: signed areas
        // sum to the loop's own signed area.
        let total: f64 = tris.iter().map(|&t| tri2(&poly, &FACE_19V3_IDS, t)).sum();
        let want = shoelace2(&poly);
        assert!(
            (total.abs() - want.abs()).abs() < 1e-12,
            "emitted area {total} does not tile the loop area {want}"
        );
    }

    #[test]
    fn slit_traversed_twice_does_not_toggle_the_region() {
        // A unit square with a slit cut in from the left edge to its
        // centre, traversed out and back — the revolve-annulus seam
        // pattern in miniature. The slit segment carries crossing
        // multiplicity 2, so the fill must NOT treat it as a boundary;
        // the whole square stays interior.
        let poly = vec![
            [0.0, 0.0],
            [1.0, 0.0],
            [1.0, 1.0],
            [0.0, 1.0],
            [0.0, 0.5],
            [0.5, 0.5],
            [0.0, 0.5],
        ];
        let ids = vec![0_u32, 1, 2, 3, 4, 5, 4];
        let loops = vec![ids];
        let tris = triangulate_chart(FaceKey::default(), &loops, std::slice::from_ref(&poly))
            .expect("slit square triangulates");
        let total: f64 = tris
            .iter()
            .map(|&t| {
                let at = |id: u32| poly[id as usize];
                let (a, b, c) = (at(t[0]), at(t[1]), at(t[2]));
                ((b[0] - a[0]) * (c[1] - a[1]) - (b[1] - a[1]) * (c[0] - a[0])).abs()
            })
            .sum();
        assert!(
            (total - 2.0).abs() < 1e-12,
            "slit square tiled to twice-area {total}, want 2 (the full square)"
        );
    }
}
