//! **The ruled open band** — the open band on curved supports, cut off
//! at its plane caps. The other open band is [`super::planar`]; what
//! the two share, and the seam both rest on, is stated at [`super`].
//!
//! A ruled link (`BlendArm::CylinderPlaneCylinder`,
//! `BlendArm::CylinderCylinderCylinder`) is a cylinder band about a
//! straight spine whose two trimlines are lines along the ruling. It
//! terminates where its supports do: at each end of the requested edge
//! the two unrequested edges lie in one plane face — the CAP,
//! classified `CornerConfig::EndFace` by the battery's predicate 6 —
//! and the band is cut off there ([`super::end_face`]) in the cap's
//! section of it about the spine's crossing: an arc of the band's
//! radius where the cap is perpendicular to the ruling, of an ellipse
//! where it is oblique (`RunOutPolicy::CutOffAtEndFace`). Exact and
//! stored; no new surface kind.
//!
//! # The walk, per link
//!
//! Plan ([`RuledPlan::plan`], read-only): per end, the cut-off
//! ([`EndCut::plan`]), the stored trimlines and spine carried along the
//! ruling to the stored cap plane. Carve (`ruled_phase`): per end the
//! cut-off's first step ([`cut_off`]); per support, one trimline `mef`
//! along the ruling between the two feet carves the strip beside the
//! crease; the crease's `kef` merges the two strips; and per end the
//! cut-off's last step ([`fold_sliver`]). What is left is the band face,
//! bounded by two trimlines and two end arcs; the caps and supports keep
//! their keys, surfaces, senses and rings, and what the carve removes
//! from a cap is metered first ([`CapSliver`]).
//!
//! **No strut is minted.** On a curved support a chord between two
//! surface points is a secant, which is the scaffolding-door
//! escalation the planar strut carries; here every new vertex sits on
//! an EXISTING edge and every new edge runs along a ruling or lies in
//! the cap plane, so each is described exactly at rest — the trimlines
//! as the band's tangent contact with the support, the arcs as its
//! transverse intersection with the cap.
//!
//! **Either material side.** The walk reads no convexity: the arm's
//! feet already fold the chain's verdict (`Convexity::ball_side` in
//! the sheet reduction), the band's sense bit folds it once more at
//! the surface pass, and the combinatorics are the same on a concave
//! chain. The cap face loses the region under the arc on either side:
//! a convex band cuts it away, a concave band's fill covers it, which
//! is what "the band adds material" means here.
//! Both sides are pinned through the extrude door: a D-profile rod
//! (convex, `ΔV = −2·A·L`,
//! `fillet_h7_transverse_cap::the_d_profile_rod_carves_through_a_cap_arc_past_pi`)
//! and a rod's section standing on a block's top edge (concave,
//! `ΔV = +2·A·L`,
//! `review_fillet_h7_r1_probes::a_sunk_rod_has_concave_ruled_creases_that_add_material`).
//!
//! **Either cap cycle.** The cut-off arc spans whichever of the cap's
//! cycles carries the old vertex — its outer cycle, or a ring where the
//! crease runs along a through-hole — on either material side: a
//! D-shaped hole's concave creases (`ΔV = +2·A·L`,
//! `band_ruled_d_hole::a_d_hole_fillets_both_creases_at_the_rod_closed_form`)
//! and a keyhole's convex ones (`ΔV = −2·A·L`,
//! `review_band_ruled_ring_probes::a_keyhole_fillets_its_convex_ring_creases_at_the_closed_form`).
//! The convex side has a boolean-built twin beside it — the rod with a
//! flat, `rod ∖ box`, at
//! `fillet_h7_transverse_cap::the_rod_with_a_flat_fillets_both_creases_at_the_prism_closed_form`.
//! The boolean builds neither
//! concave fixture — the parallel-cylinder union refuses at its
//! curved-pierce door, the block ∪ cylinder at its join lane — which is
//! the boolean's ground and not this walk's.

use geom::Surface;
use geom_core::{Bounds, Decide, Real, Tol};
use topo::{Body, EdgeKey, EntityId, FaceKey, FaceSurface, MefSite, VertexKey};

use super::end_face::{CapSliver, CutRims, EndCut, cut_off, end_rims, fold_sliver};
use crate::blend::BlendError;
use crate::blend::admit::AdmittedOpen;
use crate::blend::battery::EndSection;
use crate::blend::naming::BlendNaming;
use crate::blend::surgery::{
    ContactCarrier, Described, SourceFaces, chord_site, face_of_half, halves_of, not_intact, op,
    open_trimline, point_of, unbuilt_chain, unbuilt_geometry,
};
use geom_brep::EdgeCurveSpec;

/// **A ruled link whose two ends are plane caps**, read off the
/// source body before any mutation. The battery classified each end
/// (`fillet3_cap_transverse`); this token reads the structure that
/// classification rests on and refuses where the body disagrees with
/// the verdict.
pub(in crate::blend) struct RuledPlan<'a, T: Real> {
    link: AdmittedOpen<'a, T>,
    ends: [EndCut<T>; 2],
}

impl<'a, T: Decide + Bounds> RuledPlan<'a, T> {
    /// Plan one ruled link's carve.
    ///
    /// # Errors
    ///
    /// [`BlendError::UnsupportedGeometry`] when the link's band is not
    /// a cylinder, a trimline not a line, or a cap rim neither a line
    /// nor a circle ([`EndCut::plan`]);
    /// [`BlendError::UnsupportedChain`]
    /// when a curved support carries a ring, or when a cap rim is itself
    /// requested; [`BlendError::UnsupportedRunOut`] when a foot lands
    /// off its rim; [`BlendError::BodyNotIntact`] when the crease's
    /// halves do not lie in the supports the verdict names, an end is
    /// not among the verdict's end faces, or its incidence is not the
    /// cap the verdict classified.
    pub(in crate::blend) fn plan(
        body: &Body<T>,
        link: AdmittedOpen<'a, T>,
        opens: &[AdmittedOpen<'_, T>],
        end_faces: &[(VertexKey, EndSection<T>)],
    ) -> Result<Self, BlendError> {
        let l = link.link();
        let edge = l.edge;
        let Surface::Cylinder {
            origin: spine_origin,
            axis: tau,
            radius,
            ..
        } = l.blend.surface
        else {
            return Err(unbuilt_geometry(
                EntityId::Edge(edge),
                "a ruled link's band is not a cylinder about its ruling",
            ));
        };
        let (q_a, q_b) = (open_trimline(l, l.face_a)?.0, open_trimline(l, l.face_b)?.0);

        // The supports: each carries its half of the crease, and a
        // CURVED one is ring-free — a ring on a curved support is not
        // carried through by this carve. A plane support's rings stay
        // on it: the trimline `mef` ([`chord_site`]) hangs in the cycle
        // carrying the crease and leaves the face's other cycles on the
        // support, and the surgery's ring carry-through pass meters each
        // against the trimline. The CAP's other cycles are not refused
        // here either: the cut-off `mef` leaves each on the cap, so each
        // is metered against the region the cut removes ([`CapSliver`])
        // by the same pass.
        let (hp, hm) = halves_of(body, edge)
            .ok_or_else(|| not_intact(EntityId::Edge(edge), "a ruled link's edge"))?;
        for (face, half) in [(l.face_a, hp), (l.face_b, hm)] {
            let fd = body
                .get_face(face)
                .ok_or_else(|| not_intact(EntityId::Face(face), "a ruled link's support"))?;
            if face_of_half(body, half) != Some(face) {
                return Err(not_intact(
                    EntityId::Edge(edge),
                    "a ruled link's half-edges do not lie in the faces the verdict names",
                ));
            }
            let planar = matches!(body.get_surface(fd.surface), Some(Surface::Plane { .. }));
            if !planar && !fd.rings.is_empty() {
                return Err(unbuilt_chain(
                    edge,
                    "a ruled band's support face carries a ring, which its curved support cannot \
             carry through",
                ));
            }
        }

        let mut ends = Vec::with_capacity(2);
        for v in [l.start, l.end] {
            // The battery's classification, read rather than re-made:
            // predicate 6 tagged this end `EndFace` and the verdict
            // carries it.
            let Some((_, section)) = end_faces.iter().find(|(e, _)| *e == v) else {
                return Err(not_intact(
                    EntityId::Vertex(v),
                    "a ruled link's end is not among the end faces the verdict classified",
                ));
            };
            let (rim_a, rim_b, _) = end_rims(body, v, edge, l.face_a, l.face_b)?;
            if opens.iter().any(|o| o.edge() == rim_a || o.edge() == rim_b) {
                return Err(unbuilt_chain(
                    edge,
                    "a ruled band's cap rim is itself requested, and blending it too is not \
             implemented",
                ));
            }
            // Carried along the ruling, which the battery's
            // classification makes transverse to the cap.
            ends.push(EndCut::plan(
                body,
                v,
                edge,
                (l.face_a, l.face_b),
                (q_a, q_b),
                tau,
                section.clone(),
                Some((spine_origin, radius)),
            )?);
        }
        let Ok(ends) = <[EndCut<T>; 2]>::try_from(ends) else {
            unreachable!("ruled plan: one cap per end of the link's two ends, pushed above")
        };
        Ok(Self { link, ends })
    }

    /// The admitted link this plan carves.
    pub(in crate::blend) fn link(&self) -> AdmittedOpen<'a, T> {
        self.link
    }

    /// The link's two cut-off ends.
    pub(in crate::blend) fn ends(&self) -> &[EndCut<T>; 2] {
        &self.ends
    }

    /// The slivers this link's cut-offs remove from its caps, one per
    /// end on either side: a concave link's fill covers its caps'
    /// slivers as a convex link's cut takes them away.
    pub(in crate::blend) fn slivers(&self) -> impl Iterator<Item = &CapSliver<T>> {
        self.ends.iter().map(|e| &e.sliver)
    }
}

/// **Carve one ruled link**: the band between its two plane caps.
/// Returns the band face and the new edges awaiting their descriptions.
///
/// # Errors
///
/// [`BlendError::Op`] when an Euler operator refuses;
/// [`BlendError::UnsupportedRunOut`] / [`BlendError::UnsupportedGeometry`]
/// from a cut-off ([`cut_off`]); [`BlendError::BodyNotIntact`] where a
/// cycle read disagrees with the plan.
pub(in crate::blend) fn ruled_phase<T: Decide + Bounds + topo::AtRestPolicy>(
    body: &mut Body<T>,
    plan: &RuledPlan<'_, T>,
    sources: &SourceFaces,
    rec: &mut BlendNaming,
    tol: Tol,
) -> Result<(FaceKey, Described<T>), BlendError> {
    let l = plan.link.link();
    let crease = l.edge;
    let mut described: Described<T> = Vec::new();

    // ---- (1) Per cap: the cut-off ([`cut_off`]) — both rims split at
    // their feet and the arc `mef`'d across the cap, in the cycle that
    // carries the old vertex: its outer cycle, or a ring where the
    // crease runs along a through-hole. ----
    let mut cuts: Vec<CutRims> = Vec::with_capacity(2);
    for end in &plan.ends {
        let (rims, row) = cut_off(body, end, crease, (l.face_a, l.face_b), rec, tol)?;
        described.push(row);
        cuts.push(rims);
    }

    // ---- (2) Per support: the trimline `mef` along the ruling between
    // the two feet. The crease's half on that support is the middle of
    // the moved run — the near rim piece into the old vertex, the
    // crease, the near rim piece out of the other — so the new face is
    // the STRIP and the support keeps its key. ----
    let (hp, hm) = halves_of(body, crease)
        .ok_or_else(|| not_intact(EntityId::Edge(crease), "the crease being carved"))?;
    let mut trims: Vec<EdgeKey> = Vec::with_capacity(2);
    for (face, half, feet) in [
        (l.face_a, hp, [cuts[0].a.vertex, cuts[1].a.vertex]),
        (l.face_b, hm, [cuts[0].b.vertex, cuts[1].b.vertex]),
    ] {
        let (he1, he2, x, y) = chord_site(body, face, |row| row.0 == half, 1, 2)?;
        if !((x == feet[0] && y == feet[1]) || (x == feet[1] && y == feet[0])) {
            return Err(not_intact(
                EntityId::Face(face),
                "a support's cycle around the crease is not flanked by its two feet",
            ));
        }
        let (px, py) = (
            point_of(body, x).ok_or_else(|| not_intact(EntityId::Vertex(x), "a foot"))?,
            point_of(body, y).ok_or_else(|| not_intact(EntityId::Vertex(y), "a foot"))?,
        );
        let created = body
            .mef(
                MefSite::Chords { he1, he2 },
                EdgeCurveSpec::line_between(px, py),
                FaceSurface::Inherit,
                tol,
            )
            .map_err(|e| op("ruled trimline mef", e))?;
        described.push((created.edge, ContactCarrier::TrimLine, crease));
        rec.trims.push((created.edge, crease, face));
        trims.push(created.edge);
    }

    // ---- (3) Excise the crease across its two strips. ----
    sources.kef_minted(body, hp, "ruled crease kef", tol)?;
    rec.dead.edges.push(crease);

    // ---- (4) Per cap: fold the sliver into the band ([`fold_sliver`]). ----
    let band = band_face(body, trims[0], l.face_a)?;
    for (end, rims) in plan.ends.iter().zip(&cuts) {
        fold_sliver(body, sources, band, end.vertex, rims, rec, tol)?;
    }

    rec.blends.push((band, crease));
    Ok((band, described))
}

/// The face a trimline bounds on its non-support side: the band (once
/// the strips have merged) or the strip (before).
fn band_face<T: Decide>(
    body: &Body<T>,
    trim: EdgeKey,
    support: FaceKey,
) -> Result<FaceKey, BlendError> {
    let (hp, hm) = halves_of(body, trim)
        .ok_or_else(|| not_intact(EntityId::Edge(trim), "a trimline this carve minted"))?;
    match (face_of_half(body, hp), face_of_half(body, hm)) {
        (Some(f1), Some(f2)) if f1 == support => Ok(f2),
        (Some(f1), Some(_)) => Ok(f1),
        _ => Err(not_intact(
            EntityId::Edge(trim),
            "both halves of a trimline this carve minted bound a face",
        )),
    }
}
