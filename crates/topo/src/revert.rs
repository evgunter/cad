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
//! - Loops, faces, shells, solids, points, curves, provenance, and F9
//!   null records are copied unchanged (`Cycle::first` still names a
//!   member of its cycle; outer/ring designation is a maintained
//!   designation and survives; null-entity sides refer to the
//!   splitting surface, not the body's orientation).
//!
//! **Planar-only, and after M5 S10 that is a WIRING bound, not a
//! representational one.** Originally (F5) non-`Plane` surfaces could
//! not represent their orientation-reversed side at all: D3's enum is
//! closed and the analytic variants' chart normals have a fixed
//! parity, so there was nothing for this function to write. S10 closed
//! that by moving the reversal onto the FACE —
//! [`crate::entity::Face::sense`] — where flipping it is exact
//! structure. What S10 deliberately did not do is make this function
//! write the bit: every constructor still mints `sense: true`, and the
//! flip-every-face pass (with its involution, determinism and
//! curved-boolean rows) is the follow-on unit. So the refusal below
//! stands for exactly one more unit, and it now refuses on
//! not-yet-wired grounds rather than on unrepresentability. Curved
//! subtract/intersect keep their typed front-door refusal meanwhile.
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

use geom_core::Real;
use geom_surfaces::Surface;

use crate::body::Body;
use crate::entity::HalfEdgeKey;
use crate::geometry::SurfaceKey;

/// A failed [`Body::revert`] precondition (closed enum, D3 style); the
/// source body is never touched (revert is `&self`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RevertError {
    /// A surface is not a `Plane`: this operator does not yet write the
    /// face-sense flip a reversed curved face needs (module docs).
    ///
    /// **Status after M5 S10 (2026-08-02): the contract gap is CLOSED;
    /// the wiring is not.** The gap PR 9c returned was that a face's
    /// outward normal *was* its surface's chart normal, so `revert` had
    /// no representation to write on a curved face (the parity argument
    /// below, still the reason no amount of work inside `revert` could
    /// have closed it). S10 landed the ratified fix — option (a),
    /// [`crate::entity::Face::sense`], the orientation bit, with the
    /// full outward-normal consumer audit threaded behind it. Reverting
    /// a curved face is now a `sense` flip and nothing more. **This
    /// refusal survives only because S10 deliberately stopped at the
    /// contract plus the audit**: every constructor still mints
    /// `sense: true`, and WIRING `revert` to flip the bit (with its
    /// involution, determinism, and curved subtract/intersect rows) is
    /// the immediately following unit. Until that lands, curved
    /// subtract/intersect keep their typed front-door refusal
    /// (`BooleanError::CurvedOpUnsupported`) and this stays the
    /// operator-level statement.
    ///
    /// **The parity finding (M5 PR 9c, executed 2026-08-01; scoped by
    /// the PR 9c review's F1)**, retained as the record of *why* the
    /// representation had to change rather than the code:
    ///
    /// - **Cylinder, cone, torus**: the chart normal is ODD in the
    ///   radius (`∂u × ∂v = r·radial(u)` for the cylinder, and the
    ///   analogous odd form for the other two), so it is OUTWARD for
    ///   either sign — a negative radius moves the point to
    ///   `radial(u + π)` and the normal with it. Negating `axis` flips
    ///   `v_ref = axis × u_ref` too and merely reparameterizes
    ///   `u ↦ −u`. Nothing to write.
    /// - **Sphere**: outward under the ratified `radius > 0`
    ///   convention. The chart normal `∂u × ∂v = r²·cos v·n̂` is EVEN
    ///   in the radius (the PR 9c review's executed probe:
    ///   `Sphere { radius: -1.0, .. }` evaluates with an INWARD chart
    ///   normal, while a cylinder at `r = -1` stays outward). So a
    ///   negative-radius sphere IS a de facto reversed sphere — and it
    ///   is REJECTED as a representation, not adopted: it violates the
    ///   variant's stated `radius > 0` convention, and every consumer
    ///   that meters a sphere residual by `2r` reads the resulting
    ///   sign backwards (including this build's own
    ///   `point_in_solid` sphere arm, whose boundary residual and
    ///   `bool_ray_sphere_disc` metering both divide by `2r`).
    ///
    /// So there was nothing for this function to WRITE *in the surface*
    /// on a reverted cylinder, cone, sphere, or torus — which is why
    /// the fix had to be a ratified representation change, and why it
    /// landed on the FACE (S10 above) rather than in `Surface`.
    UnsupportedSurface {
        /// The non-plane surface.
        surface: SurfaceKey,
    },
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
            Self::UnsupportedSurface { surface } => write!(
                f,
                "revert: surface {surface:?} is not a plane — this operator \
                 still reverses planar bodies only, and the remaining gap is \
                 WIRING, no longer representation. The representation gap is \
                 CLOSED: M5 S10 landed Face::sense (the ratified option (a)), \
                 so a reverted curved face is a sense flip and the outward-\
                 normal consumers are audited and threaded behind the bit. What \
                 S10 deliberately did NOT do is make revert write it — every \
                 constructor still mints sense: true, and the follow-on unit \
                 wires this arm (flip every face's sense, keep the involution \
                 and determinism rows) and unblocks curved subtract/intersect, \
                 which meanwhile keep their typed front-door refusal \
                 (BooleanError::CurvedOpUnsupported). The parity finding that \
                 FORCED the representation change is retained verbatim, because \
                 it is the reason no surface-side fix ever existed (M5 PR 9c, \
                 executed; scoped per kind by that review's F1): the \
                 cylinder/cone/torus chart normal is ODD in the radius \
                 (r·radial(u) and its analogues), so it is outward for either \
                 sign and neither an axis flip nor a negative radius moves it; \
                 the sphere's is EVEN in the radius (r²·cos v·n̂), so it is \
                 outward exactly under the ratified radius > 0 convention — \
                 which makes a negative-radius sphere a de facto reversed \
                 sphere, REJECTED as a representation because it breaks that \
                 convention and inverts every consumer metering a sphere \
                 residual by 2r, this build's own point_in_solid sphere arm \
                 included"
            ),
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
    /// map, the planar-only refusal, the involution/determinism
    /// contract). The source is untouched.
    ///
    /// # Errors
    ///
    /// [`RevertError::UnsupportedSurface`] on the first (surface-arena
    /// order) non-plane surface; [`RevertError::Corrupt`] on
    /// tier-1-invalid input the reversal map cannot follow. All checks
    /// precede construction of the result.
    pub fn revert(&self) -> Result<Self, RevertError> {
        // ---- Preconditions (read-only). ----
        for (surface_key, surface) in self.surfaces.iter() {
            if !matches!(surface, Surface::Plane { .. }) {
                return Err(RevertError::UnsupportedSurface {
                    surface: surface_key,
                });
            }
        }
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
        for (he_key, start) in new_starts {
            if let Some(he) = out.get_half_edge_mut(he_key) {
                he.start = start;
                core::mem::swap(&mut he.next, &mut he.prev);
            }
        }
        for (edge_key, _) in self.edges.iter() {
            if let Some(edge) = out.get_edge_mut(edge_key) {
                core::mem::swap(&mut edge.he_plus, &mut edge.he_minus);
            }
        }
        for (vertex_key, anchor) in new_anchors {
            if let Some(vertex) = out.get_vertex_mut(vertex_key) {
                vertex.emanating = Some(anchor);
            }
        }
        for (_, surface) in out.surfaces.iter_mut() {
            if let Surface::Plane { normal, .. } = surface {
                *normal = -*normal;
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
    use super::*;
    use crate::fixtures::ops_cube;

    /// Non-plane surfaces refuse: ops_cube's faces share the mvfs
    /// `Nurbs` placeholder, and this operator does not yet write the
    /// face-sense flip (the follow-on unit does — M5 S10 landed the
    /// representation, not the writer). (The full
    /// involution/determinism/tier pins run on the geometric cube in
    /// `tests/m3_pr1_surgery.rs`.)
    #[test]
    fn revert_refuses_non_plane_surfaces() {
        let cube = ops_cube();
        let err = cube.body.revert().unwrap_err();
        let (surface_key, _) = cube.body.surfaces().next().unwrap();
        assert_eq!(
            err,
            RevertError::UnsupportedSurface {
                surface: surface_key,
            }
        );
    }
}
