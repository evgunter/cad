//! Geometry attachment setters: certified re-attachment of face
//! surfaces and edge curves on an existing body (M2 PR 3).
//!
//! The Euler operators attach geometry at mint time (op parameters —
//! the preferred path); these two setters cover the attachments that
//! **cannot** happen at mint time, both consequences of construction
//! order:
//!
//! - [`Body::set_face_surface`] — `mvfs`'s seed face is born before its
//!   eventual cap surface exists (the seed's surface is the honest
//!   `Surface::Nurbs` "not yet described" state); the sweep attaches
//!   the real plane once profile data determines it.
//! - [`Body::set_edge_curve`] — an intrinsic
//!   ([`geom_brep::EdgeDescription::Intersection`]) description references
//!   its two adjacent faces' surfaces *by key*, and a swept edge is
//!   necessarily minted **before** the side faces it will bound
//!   (struts precede `mef`): the sweep mints with a conventional
//!   description and upgrades to the intrinsic one here, once both
//!   surfaces exist (the prefer-intrinsic rule lands at rest, D2).
//!
//! Both setters are **certified mutations**: the same D4 ¶2/¶3 gates as
//! the operators (`EdgeCurve::certify`; typed errors, body untouched on
//! failure), plus — since both adjacent faces exist by the time an
//! edge is re-described — the **description-adjacency coherence** check
//! the mint-time gate cannot run: an `Intersection`'s two surfaces must
//! be exactly the edge's two faces' surfaces, and a `Seam`'s surface
//! must be on both sides ([`EulerOpError::DescriptionNotAdjacent`]).
//! Neither setter is an Euler operator (no topology changes — the D1
//! "exclusively Euler" rule governs *topology*); both preserve tier 1
//! (geometry arenas stay reference-coherent through the orphan-hygiene
//! paths) and re-run the tier-1 debug postcondition.
//!
//! Replacement is by **fresh insertion** (new key, old removed iff
//! orphaned): overwriting in place could silently retarget another
//! referent of a shared slot, while key churn is harmless — replay
//! determinism (D9) is about identical histories minting identical
//! keys, and a setter call is part of the history.

use geom_brep::EdgeCurveSpec;
use geom_core::Decide;

use crate::body::Body;
use crate::entity::{EdgeKey, EntityId, FaceKey};
use crate::euler::{EulerOpError, FaceSurface};
use crate::geometry::{CurveKey, SurfaceKey};
use geom_core::Tol;

impl<T: Decide> Body<T> {
    /// Replaces `face`'s surface per the [`FaceSurface`] spec,
    /// returning the face's (possibly unchanged) surface key.
    /// `Inherit` is a no-op (the current surface); `New` mints; `Shared`
    /// reuses an existing key. The old surface is removed iff nothing
    /// else references it (faces or edge descriptions).
    ///
    /// No residual certification runs here: a face's surface has no
    /// independent cache to certify — the *edges'* certifications and
    /// the tier-3 planar-face residuals are what tie a face's surface
    /// to the body's points, and re-attaching a surface is exactly what
    /// tier 3 re-checks at rest. (In particular, replacing a surface
    /// can invalidate an adjacent edge's certification; tier 3 reports
    /// it — attach surfaces before upgrading edge descriptions.)
    ///
    /// # Errors
    ///
    /// [`EulerOpError::StaleKey`] if `face` does not resolve;
    /// [`EulerOpError::StaleGeometry`] if a `Shared` key does not
    /// resolve. The body is untouched on `Err`.
    pub fn set_face_surface(
        &mut self,
        face: FaceKey,
        surface: FaceSurface<T>,
    ) -> Result<SurfaceKey, EulerOpError> {
        let face_data = self.get_face(face).ok_or(EulerOpError::StaleKey {
            key: EntityId::Face(face),
        })?;
        let old = face_data.surface;
        self.check_face_surface(&surface)?;

        // ---- Mutation (infallible from here on). ----
        let new = self.mint_face_surface(surface, old);
        if new != old {
            let Some(f) = self.get_face_mut(face) else {
                unreachable!(
                    "set_face_surface: `face` resolved in the plan phase and minting a \
                     surface kills no face"
                )
            };
            f.surface = new;
            self.remove_surface_if_orphaned(old);
        }

        #[cfg(debug_assertions)]
        debug_assert_eq!(
            crate::validate::validate(self),
            Ok(()),
            "set_face_surface postcondition: result is not tier-1 valid (kernel bug)",
        );
        Ok(new)
    }

    /// Sets `face`'s orientation sense ([`crate::Face::sense`]) — the
    /// constructor-facing writer of the S10 orientation bit, opened in
    /// M5 S11.
    ///
    /// An Euler operator mints `sense: true` on a face it puts on a
    /// NEW surface, because the material side is not op-level
    /// knowledge: `mef` sees two chords, not the profile. Whether a
    /// swept wall's material lies with or against its surface's chart
    /// normal is the **constructor's**
    /// knowledge, decided from exact stored structure (a concave arc
    /// segment's turn sign against its loop's canonical winding — the
    /// profile's material-left rule), so the constructor attaches the
    /// honest bit here right after the mint, exactly as
    /// [`Body::set_face_surface`] attaches a surface the mint could not
    /// know. Callers must keep the two encodings of orientation
    /// coherent (the bit and the loop winding — tier 3's check 6
    /// falsifies planar disagreement at rest); the test-only hand-flip
    /// door [`Body::flipped_face_sense_for_tests`] is the deliberate
    /// exception.
    ///
    /// Not an Euler operator (no topology changes) and not a numeric
    /// decision (a `bool` is written, nothing compared); tier 1 is
    /// trivially preserved.
    ///
    /// **Splitting inherits the bit exactly where the fragment is the
    /// same region.** A `mef` or `mfkrh` re-mint that keeps the
    /// parent's surface takes the parent's `sense` — a piece of a
    /// reversed wall is the same surface region with the same material
    /// side — and stamps `true` only when the fragment lands somewhere
    /// that is NOT the parent's surface (a fresh one, or a foreign
    /// shared key), which is not the parent's region at all and whose
    /// honest bit is this door's to attach.
    /// `Body::mint_face_surface_and_sense`
    /// owns that rule; the boolean's chord re-mints (`chord_join.rs`)
    /// pass `FaceSurface::Inherit` and so inherit, while
    /// `splitting/finish.rs`'s section promotion is the live case of
    /// the mint. Guard: sweep's `m5_s12_curved_ops.rs`, the row named
    /// `a_boolean_that_splits_a_reversed_wall_inherits_the_parent_bit`.
    ///
    /// # Errors
    ///
    /// [`EulerOpError::StaleKey`] if `face` does not resolve. The body
    /// is untouched on `Err`.
    pub fn set_face_sense(&mut self, face: FaceKey, sense: bool) -> Result<(), EulerOpError> {
        let f = self.get_face_mut(face).ok_or(EulerOpError::StaleKey {
            key: EntityId::Face(face),
        })?;
        f.sense = sense;
        Ok(())
    }

    /// Replaces `edge`'s curve geometry with a freshly certified
    /// attachment of `curve`, returning the new curve key. The
    /// certification endpoints are the edge's own vertices' points in
    /// `he_plus` forward order; intrinsic/seam descriptions must also
    /// be **adjacency-coherent** (module docs). The old curve is
    /// removed iff no other edge references it.
    ///
    /// # Errors
    ///
    /// [`EulerOpError::StaleKey`] / [`EulerOpError::StaleGeometry`] on
    /// unresolvable topology/points;
    /// [`EulerOpError::DescriptionNotAdjacent`] on an
    /// `Intersection`/`Seam` description whose surfaces are not the
    /// edge's faces' surfaces; [`EulerOpError::Certification`] on a
    /// failed gate. The body is untouched on `Err`.
    pub fn set_edge_curve(
        &mut self,
        edge: EdgeKey,
        curve: EdgeCurveSpec<T>,
        tol: Tol,
    ) -> Result<CurveKey, EulerOpError> {
        self.set_edge_curve_via(edge, curve, Self::certify_edge_spec, tol)
    }

    /// [`Body::set_edge_curve`] with the certification door supplied by
    /// the caller — the one axis on which the two attach doors differ
    /// (the plane × NURBS lane, M7-8). Every precondition, every
    /// adjacency rule and every mutation below is shared verbatim, so
    /// the doors cannot drift apart.
    pub(crate) fn set_edge_curve_via(
        &mut self,
        edge: EdgeKey,
        curve: EdgeCurveSpec<T>,
        certify: impl FnOnce(
            &Self,
            EdgeCurveSpec<T>,
            geom_core::Point3<T>,
            geom_core::Point3<T>,
            Tol,
        ) -> Result<geom_brep::EdgeCurve<T>, EulerOpError>,
        tol: Tol,
    ) -> Result<CurveKey, EulerOpError> {
        let edge_data = self.get_edge(edge).ok_or(EulerOpError::StaleKey {
            key: EntityId::Edge(edge),
        })?;
        let old = edge_data.curve;
        let he_plus = edge_data.he_plus;
        let he_minus = edge_data.he_minus;
        let plus_data = self.resolve_half_edge(he_plus)?;
        let end_vertex = self.half_edge_end(he_plus).ok_or(EulerOpError::StaleKey {
            key: EntityId::HalfEdge(he_plus),
        })?;
        let p_start = self.resolve_vertex_point(plus_data.start)?;
        let p_end = self.resolve_vertex_point(end_vertex)?;

        // Adjacency coherence (module docs): resolvable now that both
        // faces exist.
        let face_surface = |body: &Self, he: crate::entity::HalfEdgeKey| {
            let he_data = body.resolve_half_edge(he)?;
            let loop_data = body
                .get_loop(he_data.parent_loop)
                .ok_or(EulerOpError::StaleKey {
                    key: EntityId::Loop(he_data.parent_loop),
                })?;
            let face_data = body
                .get_face(loop_data.face)
                .ok_or(EulerOpError::StaleKey {
                    key: EntityId::Face(loop_data.face),
                })?;
            Ok::<SurfaceKey, EulerOpError>(face_data.surface)
        };
        let fs_plus = face_surface(self, he_plus)?;
        let fs_minus = face_surface(self, he_minus)?;
        match curve.description {
            // Both intrinsic variants carry the same adjacency
            // obligation: the described pair IS the faces' pair
            // (M5 PR 9 — TangentIntersection mirrors Intersection).
            geom_brep::EdgeDescriptionSpec::Intersection { s1, s2, .. }
            | geom_brep::EdgeDescriptionSpec::TangentIntersection { s1, s2, .. } => {
                let matches_pair =
                    (s1 == fs_plus && s2 == fs_minus) || (s1 == fs_minus && s2 == fs_plus);
                if !matches_pair {
                    return Err(EulerOpError::DescriptionNotAdjacent { edge });
                }
            }
            // A chart image names ONE of the edge's two adjacent
            // faces' surfaces (M6-3: a wall–wall seam is the
            // u-boundary iso of either wall; the minted convention
            // picks one, and adjacency accepts either side — the
            // M5-LOG item 6(iii) reading). A chart image that claims
            // to BE the chart's parameterization seam owes more: both
            // sides of a seam are the SAME surface, by what a seam is.
            geom_brep::EdgeDescriptionSpec::Chart { surface, seam, .. } => {
                let adjacent = if seam {
                    surface == fs_plus && surface == fs_minus
                } else {
                    surface == fs_plus || surface == fs_minus
                };
                if !adjacent {
                    return Err(EulerOpError::DescriptionNotAdjacent { edge });
                }
            }
            // The scaffolding door names no surface — there is none.
            geom_brep::EdgeDescriptionSpec::Scaffold(_) => {}
        }

        let certified = certify(self, curve, p_start, p_end, tol)?;

        // ---- Mutation (infallible from here on). ----
        let new = self.add_curve(certified);
        let Some(e) = self.get_edge_mut(edge) else {
            unreachable!(
                "set_edge_curve: `edge` resolved in the plan phase and adding a curve \
                 kills no edge"
            )
        };
        e.curve = new;
        self.remove_curve_if_orphaned(old);

        #[cfg(debug_assertions)]
        debug_assert_eq!(
            crate::validate::validate(self),
            Ok(()),
            "set_edge_curve postcondition: result is not tier-1 valid (kernel bug)",
        );
        Ok(new)
    }
}
