//! `revert` — whole-body orientation reversal (M3 PR 1): the ch. 15
//! `revert(b)` that boolean difference needs (`A ∖ B ≡ A ∩ revert(B)`).
//!
//! A reverted body bounds the **complementary** volume: every loop's
//! half-edge cycle runs backwards, so by the interior-left rule the
//! outward side flips. The surgery is a pure per-entity map — no keys
//! are minted or killed, so the reverted body's arenas are key-for-key
//! those of the source (D9: a deterministic function of the input, and
//! a bitwise **involution** — `revert ∘ revert` is the identity,
//! pinned by test):
//!
//! - **Half-edges**: `start ← end(he)` (each half now runs the other
//!   way), `next ↔ prev` (cycles reverse).
//! - **Edges**: `he_plus ↔ he_minus`. This is what keeps every curve
//!   valid *unchanged*: the old minus half already traverses the
//!   carrier forward, and after reversal it is exactly the
//!   forward-running half — so the he_plus forward contract holds with
//!   zero curve mutation (bitwise involution for free).
//! - **Vertices**: `emanating ← mate(emanating)` (the old anchor no
//!   longer starts here; its mate does — and `mate ∘ mate = id` keeps
//!   the involution). Lone vertices (`None`) are untouched.
//! - **Surfaces**: every `Plane`'s `normal` is negated (`u_ref` and
//!   `origin` unchanged — the frame stays right-handed with `v_ref`
//!   flipping alongside), re-satisfying the convention that a face's
//!   outward normal is its plane's stored normal. Negation is a
//!   bitwise involution.
//! - **Face senses** (M5 S12): every face whose surface is **not** a
//!   `Plane` has [`crate::entity::Face::sense`] flipped. This is the
//!   curved arm, and it is the *same* statement as the plane bullet —
//!   each face's outward normal is negated exactly once — written in
//!   whichever of the two encodings the surface class admits:
//!   - a `Plane` can carry its own reversal (the normal is stored, and
//!     negating it is exact), so the plane arm stays where M3 put it
//!     and stays bit-for-bit what the planar battery pins;
//!   - the analytic charts cannot (D1's S10 amendment: cylinder, cone
//!     and torus normals are ODD in the radius, the sphere's is EVEN
//!     and the `radius > 0` convention fixes it outward), and a NURBS
//!     chart's parameterization is not ours to rewrite — so those
//!     faces flip the S10 bit instead.
//!
//!   The two arms are **exclusive by surface kind**, so no face is
//!   flipped twice, and both are exact structure: a `bool` negation
//!   and an IEEE sign flip are each bitwise involutions, so `revert ∘
//!   revert` stays bit-identical at every scalar backend (D1: "exact
//!   structure, never a decide"). The sense flip is applied to every
//!   face on a non-plane surface — including one already carrying an
//!   honest `false` from S11's concave/inward constructors, which is
//!   the whole point: reverting a body with mixed senses must flip
//!   each of them, not stamp a constant.
//! - Loops, faces, shells, solids, points, curves, provenance, and F9
//!   null records are copied unchanged (`Cycle::first` still names a
//!   member of its cycle; outer/ring designation is a maintained
//!   designation and survives; null-entity sides refer to the
//!   splitting surface, not the body's orientation).
//!
//! **No longer planar-only (M5 S12).** Originally (F5) non-`Plane`
//! surfaces could not represent their orientation-reversed side at
//! all: D3's enum is closed and the analytic variants' chart normals
//! have a fixed parity, so there was nothing for this function to
//! write, and it refused `RevertError::UnsupportedSurface`. S10 closed
//! that by moving the reversal onto the FACE —
//! [`crate::entity::Face::sense`] — where flipping it is exact
//! structure; S11's constructors made the incoming bits honest (a
//! concave wall already reads `false`, so there is a real bit to
//! flip rather than a uniform lie); S12 (here) writes it. The refusal
//! is **retired**: the sense flip is uniform over every non-plane
//! surface class, so there is no per-class residue left inside this
//! operator. What remains gated is downstream and belongs to the
//! *boolean*, not to `revert` — a curved subtract still needs a join
//! lane for its seam, and the classes that lack one refuse typed at
//! their own doors naming their own blocker.
//!
//! Functional style (the plan's assumption, made concrete): `revert`
//! takes `&self` and returns a **new body value** — the operand is
//! untouched, both bodies remain usable (Problem 15.7's
//! both-results-free, inherited by ∖'s use of revert).
//!
//! **Validity class**: a reverted body is **tier-2 currency** — every
//! structural invariant and every certification survives the map — but
//! deliberately NOT tier-3: it bounds the complement, so the +V
//! invariant fails (exactly `NegativeVolume`, pinned by test). That is
//! correct, not a defect: `revert(B)` is ∖'s transient operand, never
//! an at-rest solid handed across the API.
//!
//! Serves ch. 15 `setopfinish` (difference reverts `B`'s kept
//! component) and the `A ∖ B ≡ A ∩ revert(B)` oracle (M3 PR 5).

use core::fmt;
use std::collections::BTreeSet;

use geom::Surface;
use geom_core::Real;

use crate::body::Body;
use crate::entity::HalfEdgeKey;
use crate::geometry::SurfaceKey;
use geom_core::Tol;

/// A failed [`Body::revert`] precondition (closed enum, D3 style); the
/// source body is never touched (revert is `&self`).
///
/// **Retired variant — `UnsupportedSurface`** (M3 PR 1 → M5 S12).
/// Retired, not left unreachable: a closed enum that can no longer
/// produce one of its variants is a lie about the frontier. The record
/// is kept here, and the refusal pin it carried is re-pinned as a
/// CONSTRUCTION row (the S9 pattern) in
/// `crates/sweep/tests/m5_s12_curved_ops.rs` (the curved arm needs the
/// sweep constructors to build a curved body at all).
///
/// - **What it said**: a surface is not a `Plane`, so this operator has
///   no representation to write for the reversed side of a curved face.
/// - **Why it is gone**: the reversal moved onto the FACE.
///   [`Body::revert`] flips [`crate::entity::Face::sense`] on every face
///   carried by a non-plane surface — exact structure, uniform over
///   cylinder, cone, sphere, torus and NURBS alike — so no per-class
///   residue is left *inside* `revert`. What is still gated is
///   downstream and belongs to the boolean: a curved subtract needs a
///   JOIN lane for its seam, and the classes lacking one refuse typed at
///   their own doors, naming their own blocker.
/// - **The parity finding it carried** (M5 PR 9c, executed 2026-08-01;
///   scoped per kind by that review's F1), retained because it is the
///   reason no surface-side fix ever existed, and hence why D1's S10
///   amendment had to be *ratified* rather than coded around:
///   - **Cylinder, cone, torus**: the chart normal is ODD in the radius
///     (`∂u × ∂v = r·radial(u)` for the cylinder, analogously for the
///     other two), so it is OUTWARD for either sign — a negative radius
///     moves the point to `radial(u + π)` and the normal with it, and
///     negating `axis` merely reparameterizes `u ↦ −u`. Nothing to
///     write.
///   - **Sphere**: `∂u × ∂v = r²·cos v·n̂` is EVEN in the radius, so the
///     chart normal is outward exactly under the ratified `radius > 0`
///     convention. A negative-radius sphere is therefore a de facto
///     reversed sphere — REJECTED as a representation, not adopted: it
///     breaks that convention, and every consumer metering a sphere
///     residual by `2r` reads the sign backwards, this build's own
///     `point_in_solid` sphere arm included.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RevertError {
    /// A half-edge's derived end or mate does not resolve —
    /// tier-1-invalid input, surfaced typed (D9: never a panic).
    Corrupt {
        /// The half-edge whose reversal data is unresolvable.
        he: HalfEdgeKey,
    },
}

impl fmt::Display for RevertError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Corrupt { he } => write!(
                f,
                "revert: half-edge {he:?}'s end or mate does not resolve \
                 (malformed body)"
            ),
        }
    }
}

impl std::error::Error for RevertError {}

impl<T: Real> Body<T> {
    /// The orientation-reversed body value (module docs: the per-entity
    /// map, the two exclusive orientation-reversal encodings, the
    /// involution/determinism contract). Every surface class is
    /// supported as of M5 S12. The source is untouched.
    ///
    /// # Errors
    ///
    /// [`RevertError::Corrupt`] on tier-1-invalid input the reversal map
    /// cannot follow. All checks precede construction of the result.
    pub fn revert(&self) -> Result<Self, RevertError> {
        // ---- Preconditions (read-only). ----
        // Which surfaces carry their own reversal (`Plane`: the normal
        // is negated in the surface — the M3 encoding) and which push it
        // onto the face's `sense` bit (every other class — module docs).
        // Read from the SOURCE: revert is key-for-key, so the
        // classification is valid for the result too.
        let plane_surfaces: BTreeSet<SurfaceKey> = self
            .surfaces
            .iter()
            .filter(|(_, surface)| matches!(surface, Surface::Plane { .. }))
            .map(|(key, _)| key)
            .collect();
        // Resolve every half-edge's new start and every vertex's new
        // anchor from the SOURCE before building the result (the map
        // must read pre-reversal adjacency throughout).
        let mut new_starts = Vec::with_capacity(self.half_edges.len());
        for (he_key, _) in self.half_edges.iter() {
            let end = self
                .half_edge_end(he_key)
                .ok_or(RevertError::Corrupt { he: he_key })?;
            new_starts.push((he_key, end));
        }
        let mut new_anchors = Vec::new();
        for (vertex_key, vertex) in self.vertices.iter() {
            if let Some(emanating) = vertex.emanating {
                let mate = self
                    .mate(emanating)
                    .ok_or(RevertError::Corrupt { he: emanating })?;
                new_anchors.push((vertex_key, mate));
            }
        }

        // ---- The map (infallible from here on). ----
        let mut out = self.clone();
        // The two keyed loops below carry a value the plan phase
        // derived per key, so they look their key up; `out` is a clone
        // of `self` and cloning a slotmap preserves its keys, so every
        // lookup resolves and the map removes nothing. The edge loop
        // carries no such value and therefore does not look anything
        // up — it walks the arena directly, like the surface and face
        // loops below.
        for (he_key, start) in new_starts {
            let Some(he) = out.get_half_edge_mut(he_key) else {
                unreachable!("revert: `he_key` was iterated out of the arena `out` clones")
            };
            he.start = start;
            core::mem::swap(&mut he.next, &mut he.prev);
        }
        for (_, edge) in out.edges.iter_mut() {
            core::mem::swap(&mut edge.he_plus, &mut edge.he_minus);
        }
        for (vertex_key, anchor) in new_anchors {
            let Some(vertex) = out.get_vertex_mut(vertex_key) else {
                unreachable!("revert: `vertex_key` was iterated out of the arena `out` clones")
            };
            vertex.emanating = Some(anchor);
        }
        for (_, surface) in out.surfaces.iter_mut() {
            if let Surface::Plane { normal, .. } = surface {
                *normal = -*normal;
            }
        }
        // The curved arm (M5 S12): the reversal a non-plane chart
        // cannot express goes on the FACE. Exclusive with the plane
        // negation above — a face is flipped in exactly one encoding —
        // and it is a `bool` negation, so it is exact structure at every
        // backend and a bitwise involution.
        for (_, face) in out.faces.iter_mut() {
            if !plane_surfaces.contains(&face.surface) {
                face.sense = !face.sense;
            }
        }
        // N6: `revert` flips every surface source's orientation tag
        // (`rev ∘ rev = id`) — the negated description is the SAME
        // recipe source seen from the other side. Curve and point
        // records are untouched (their descriptions are).
        for (_, gs) in out.surface_sources.iter_mut() {
            *gs = gs.reverted();
        }

        #[cfg(debug_assertions)]
        debug_assert_eq!(
            crate::validate::validate(&out),
            Ok(()),
            "revert postcondition: result is not tier-1 valid (kernel bug)",
        );
        Ok(out)
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use geom_core::Tol;
    use crate::fixtures::ops_cube;

    /// **CONSTRUCTION row, flipped from the M3 refusal pin** (S9
    /// pattern; the retired `UnsupportedSurface` record is on
    /// [`RevertError`]). `ops_cube`'s faces all share the `mvfs`
    /// `Nurbs` placeholder surface — the exact shape that refused
    /// before S12 — and now revert: every face's `sense` flips, and
    /// nothing else about a non-plane surface moves. (The full
    /// involution/determinism/tier pins run on the geometric cube in
    /// `tests/m3_pr1_surgery.rs` and on real analytic surfaces in
    /// `crates/sweep/tests/m5_s12_curved_ops.rs`.)
    #[test]
    fn revert_flips_sense_on_non_plane_faces_instead_of_refusing() {
        let cube = ops_cube(Tol::witness());
        let before: Vec<bool> = cube.body.faces().map(|(_, f)| f.sense).collect();
        assert!(before.iter().all(|s| *s), "mvfs/mef mint sense: true");
        let reverted = cube.body.revert().expect("S12: curved revert is wired");
        let after: Vec<bool> = reverted.faces().map(|(_, f)| f.sense).collect();
        assert!(after.iter().all(|s| !*s), "every non-plane face flipped");
        // The surfaces themselves are untouched (the reversal is on the
        // face, not the chart), and the involution is bitwise.
        assert_eq!(
            format!("{:?}", reverted.surfaces().collect::<Vec<_>>()),
            format!("{:?}", cube.body.surfaces().collect::<Vec<_>>()),
        );
        assert_eq!(
            format!("{:?}", reverted.revert().unwrap()),
            format!("{:?}", cube.body),
        );
    }
}
