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
    // Every id a face can SHARE with another face is already minted:
    // topology vertices, then chord points, then (per face, below) that
    // face's own interior grid. So a shared id is exactly an id below
    // this mark, and the census at the end of this function tests it
    // with an integer compare rather than a lookup.
    #[cfg(debug_assertions)]
    #[allow(clippy::cast_possible_truncation)]
    let shared_below = positions.len() as u32;
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

    let mesh = Mesh {
        positions,
        patches,
        boundaries,
    };

    // D2 addendum row 5, and the CROSS-FACE half of the class
    // `curved`'s per-patch re-derivation cannot see: that census reads
    // ONE patch's identified edges, so a boundary the two adjacent
    // faces failed to identify with each other is outside its
    // footprint by construction (issue 897 says so, and it is right).
    // Re-derive it here, over the only ids two faces can share — the
    // chord segments of the body's own edges — and over nothing else.
    //
    // WHY NOT `check_mesh`, which is already the oracle for exactly
    // this: it was the first candidate and it was MEASURED against
    // this one, both switched into one binary on the tour corpus, at
    // all three ε rows and the byte instrument's three deltas.
    //
    // THE PRICE ARGUMENT IS NARROWER THAN IT LOOKS, and is stated at
    // its real width. On the sub-millisecond rows the same-binary
    // spread between rounds runs to ~98%, which swamps both columns;
    // there `check_mesh` measures CHEAPER than the census below on
    // several rows, and that is not evidence for it any more than
    // against it. The rows that decide are the donut's, where the
    // spread is 4-12% and the meshes are 7k-178k triangles per patch:
    // `check_mesh` +24% to +33% of `tessellate`, this census −8% to
    // +1%. That gap is the price argument, and it is the whole of it.
    //
    // The rest is FOOTPRINT, which does not depend on the clock:
    // `check_mesh` censuses every edge of every patch — overwhelmingly
    // patch-interior grid edges that no cross-face question is about —
    // and re-checks winding and degeneracy, which are other rows'
    // classes. This census reads the chord segments and nothing else,
    // so its footprint IS the class:
    // an unidentified shared boundary makes each side's copy a
    // one-use edge, which is what `n != 2` catches. The narrower guard
    // is not a second copy of the oracle; it is the class's own
    // question, and `check_mesh` remains available to a caller.
    #[cfg(debug_assertions)]
    {
        let polylines: Vec<&[u32]> = chords.ids.values().map(Vec::as_slice).collect();
        let patch_triangles: Vec<&[[u32; 3]]> = mesh
            .patches
            .iter()
            .map(|p| p.triangles.as_slice())
            .collect();
        let bad = unpaired_chord_segment(&polylines, &patch_triangles, shared_below);
        debug_assert!(
            bad.is_none(),
            "chord segment {:?} is used by {} face triangles rather than 2: the \
             faces meeting on that edge did not identify it (issue 897)",
            bad.map(|(e, _)| e),
            bad.map_or(0, |(_, n)| n)
        );
    }

    Ok(mesh)
}

/// The chord segment that is NOT used by exactly two face triangles,
/// if any — the cross-face identification re-derivation (issue 897).
///
/// Every edge of the body carries a chord polyline whose segments the
/// two faces meeting on that edge both insert as CDT constraints, so
/// in a watertight emission each segment is a triangle edge exactly
/// twice: once per side, or twice within one patch where a `Seam` edge
/// is traversed both ways by the same face. A count of 1 is the class
/// this guard exists for — the two sides emitted the segment under
/// DIFFERENT ids, so neither copy pairs up.
///
/// `shared_below` is the first id minted after the chord pass. Ids are
/// minted topology-vertices-then-chords-then-per-face-grid (D9's
/// determinism order, at the top of [`tessellate`]), so an id at or
/// above the mark is one face's private grid point and can never be a
/// chord segment endpoint. Testing that first is what keeps this scan
/// an integer compare on the overwhelming majority of triangle edges
/// rather than a map probe.
///
/// **PRECONDITION: the body is CLOSED, and that is the caller's, not
/// this census's.** [`tessellate`]'s contract says the input is a
/// closed solid at rest and that it does not re-validate; it never
/// calls `topo::validate_closed`. On an OPEN body — a tier-1-legal
/// scaffolding strut, say, which `topo::validate` accepts and
/// `validate_closed` rejects — a chord polyline exists that no face
/// triangle can use twice, and this census reports it. That firing is
/// a broken PRECONDITION, not the D2-row-5 kernel bug the assert is
/// worded for, and it is the one way the guard can be reached by input
/// rather than by defect. It stays a `debug_assert` on that basis: the
/// precondition is documented at the door, an open body is already
/// outside what `tessellate` promises anything about, and no shipped
/// build is made to panic by it that was not already garbage-in.
///
/// **The route is documented, not demonstrated, and the difference is
/// recorded rather than glossed.** A reviewer's probe
/// (`r2_mesh6_probes::r2_scaffold_strut_body_through_tessellate`)
/// mints the strut body through the Euler doors and calls
/// [`tessellate`] on it; the call refuses EARLIER and typed —
/// `UnsupportedSurface`, because faces assembled that way carry no
/// surface description — so the census is never reached and no open
/// body is yet known to reach it. The probe is kept as the record of
/// that attempt: it pins where the door actually stops, which is the
/// honest state of the precondition claim.
///
/// **This reads no tolerance.** It is a census of ids and counts;
/// `Eps` has no role in it, and a band would be the wrong instrument
/// for a question whose answer is an integer.
#[cfg(debug_assertions)]
fn unpaired_chord_segment(
    polylines: &[&[u32]],
    patch_triangles: &[&[[u32; 3]]],
    shared_below: u32,
) -> Option<((u32, u32), usize)> {
    let mut uses: HashMap<(u32, u32), usize> = HashMap::new();
    for ids in polylines {
        for w in ids.windows(2) {
            uses.insert(crate::walk::edge_key(w[0], w[1]), 0);
        }
    }
    for t in patch_triangles.iter().copied().flatten() {
        for k in 0..3 {
            let (a, b) = (t[k], t[(k + 1) % 3]);
            if a < shared_below
                && b < shared_below
                && let Some(n) = uses.get_mut(&crate::walk::edge_key(a, b))
            {
                *n += 1;
            }
        }
    }
    uses.iter().find(|&(_, &n)| n != 2).map(|(&e, &n)| (e, n))
}

// GATED ON THE GUARD IT TESTS, the way `walk`'s own row-5 test row is
// (`walk::tests::the_closure_detector_fires_when_the_gap_clears_the_
// spatial_bar`): every row here calls `unpaired_chord_segment`, which
// is `#[cfg(debug_assertions)]`, so with debug-assertions OFF the
// subject does not exist and neither should the rows. Without this the
// lib test target fails to COMPILE in that configuration.
#[cfg(all(test, debug_assertions))]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    //! The cross-face identification census (issue 897), red first.
    //!
    //! The class is the one the per-patch re-derivation in `curved`
    //! cannot see by construction: each patch is internally consistent
    //! — every one of its own edges used at most twice — and the
    //! failure is only visible when the two patches are read together.
    //! The rows below build exactly that mesh.

    use super::*;

    /// Every edge use within one patch, so a row can show the patch is
    /// internally clean while the pair of patches is not.
    fn per_patch_max_use(tris: &[[u32; 3]]) -> usize {
        let mut uses: HashMap<(u32, u32), usize> = HashMap::new();
        for t in tris {
            for k in 0..3 {
                let (a, b) = (t[k], t[(k + 1) % 3]);
                *uses.entry((a.min(b), a.max(b))).or_insert(0) += 1;
            }
        }
        uses.values().copied().max().unwrap_or(0)
    }

    #[test]
    fn a_shared_chord_segment_used_once_per_side_pairs_up() {
        // Chord polyline 0-1-2 on the edge between two faces; each
        // face emits both segments once. Ids 9+ are grid points.
        let poly: [&[u32]; 1] = [&[0, 1, 2]];
        let tris = [[0, 1, 9], [1, 2, 9], [1, 0, 10], [2, 1, 10]];
        assert_eq!(
            unpaired_chord_segment(&poly, &[&tris], 3),
            None,
            "two faces that identified the boundary leave every segment at two uses"
        );
    }

    #[test]
    fn a_seam_edge_traversed_twice_by_one_face_pairs_up() {
        // The full-2π case: ONE patch supplies both uses. The census
        // counts uses, not sides, which is what makes this legal.
        let poly: [&[u32]; 1] = [&[0, 1]];
        let tris = [[0, 1, 9], [1, 0, 10]];
        assert_eq!(unpaired_chord_segment(&poly, &[&tris], 2), None);
    }

    #[test]
    fn a_boundary_the_second_face_renumbered_is_caught() {
        // RED FIRST. The second face emits the same chord points under
        // its own ids (3, 4, 5) instead of the shared 0, 1, 2 — the
        // cross-face identification failure. Both patches stay
        // internally consistent, so nothing per-patch can see it.
        let a = [[0, 1, 9], [1, 2, 9]];
        let b = [[4, 3, 10], [5, 4, 10]];
        assert!(per_patch_max_use(&a) <= 2, "patch A is internally clean");
        assert!(per_patch_max_use(&b) <= 2, "patch B is internally clean");
        let poly: [&[u32]; 1] = [&[0, 1, 2]];
        let bad = unpaired_chord_segment(&poly, &[&a, &b], 9);
        assert!(
            matches!(bad, Some((_, 1))),
            "an unidentified shared boundary leaves each side's copy at ONE use, got {bad:?}"
        );
    }

    #[test]
    fn a_segment_no_face_emitted_at_all_is_caught() {
        // The other side of `n != 2`: a hole rather than a mismatch.
        let poly: [&[u32]; 1] = [&[0, 1]];
        assert_eq!(unpaired_chord_segment(&poly, &[&[]], 2), Some(((0, 1), 0)));
    }

    #[test]
    fn ids_at_or_above_the_mark_are_never_probed() {
        // The mark is what keeps the scan an integer compare on grid
        // edges. Below it the same pair IS probed and counted, so the
        // two halves of the row differ only in the mark.
        let poly: [&[u32]; 1] = [&[0, 1]];
        let tris = [[0, 1, 9]];
        assert_eq!(
            unpaired_chord_segment(&poly, &[&tris], 1),
            Some(((0, 1), 0))
        );
        assert_eq!(
            unpaired_chord_segment(&poly, &[&tris], 2),
            Some(((0, 1), 1))
        );
    }
}
