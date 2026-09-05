//! **The ruled open band** — the open band on curved supports, cut off
//! at transverse caps. The other open band is [`super::planar`]; what
//! the two share, and the seam both rest on, is stated at [`super`].
//!
//!
//! A ruled link (`BlendArm::CylinderPlaneCylinder`,
//! `BlendArm::CylinderCylinderCylinder`) is a cylinder band about a
//! straight spine whose two trimlines are lines along the ruling. It
//! terminates where its supports do: at each end of the requested edge
//! the two unrequested edges lie in one plane face perpendicular to
//! the ruling — the CAP, classified `CornerConfig::TransverseCap` by
//! the battery's predicate 6 — and the band ends in that plane's
//! section of it, a circle of the band's radius about the spine's
//! crossing, of which the band's end is the arc between the two feet
//! (`RunOutPolicy::CutOffAtTransverseCap`). Exact and stored; no new
//! surface kind.
//!
//! # The walk, per link
//!
//! Plan ([`RuledPlan::plan`], read-only): per end, the cap face, the
//! two rim edges it shares with the supports, the feet — where each
//! support's trimline meets the cap plane — and the section circle's
//! centre. Every point is the stored trimline or spine carried along
//! the ruling to the stored cap plane; nothing is sampled or decided.
//!
//! Carve (`ruled_phase`): per end, split each rim edge at its foot
//! (`seam_split_param`, the ladder's one home for a split parameter)
//! and `mef` the cut-off arc across the cap between the two
//! feet — the corner region between the arc and the old vertex becomes
//! a SLIVER face. Per support, one trimline `mef` along the ruling
//! between the two feet carves the strip beside the crease. Then the
//! crease's `kef` merges the two strips, and at each end one `kef`
//! folds the sliver into the merged face and one `kev` retires the rim
//! remnant that is now a spur together with the old vertex. What is
//! left is the band face, bounded by two trimlines and two arcs; the
//! caps and supports keep their keys, surfaces, senses and rings.
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
//! chain — the cap face then GAINS the region under the arc rather
//! than losing it, which is what "the band adds material" means here.
//! Both sides are pinned through the extrude door: the rod with a flat
//! (convex, `ΔV = −2·A·L`) and a rod's section standing on a block's
//! top edge (concave, `ΔV = +2·A·L`;
//! `tests/review_fillet_h7_r1_probes.rs`). The boolean builds neither
//! concave fixture — the parallel-cylinder union refuses at its
//! curved-pierce door, the block ∪ cylinder at its join lane — which is
//! the boolean's ground and not this walk's.

use geom::Curve3;
use geom::Surface;
use geom_brep::EdgeCurveSpec;
use geom_core::{Bounds, Decide, Point3, Real, Tol};
use topo::{Body, EdgeKey, EntityId, FaceKey, FaceSurface, HalfEdgeKey, MefSite, VertexKey};

use super::chord_site;
use crate::blend::BlendError;
use crate::blend::admit::AdmittedOpen;
use crate::blend::battery::cap_incidence;
use crate::blend::naming::BlendNaming;
use crate::blend::surgery::{
    ContactCarrier, Described, SourceFaces, edge_touches, face_of_half, halves_of, loop_of_half,
    not_intact, op, point_of, seam_split_param, unbuilt_chain, unbuilt_geometry,
};

/// One transverse cap of a ruled link, as the plan read it.
struct CapEnd<T: Real> {
    /// The old vertex, which dies with the slivers.
    vertex: VertexKey,
    /// The cap face — the one face at `vertex` that is not a support.
    /// Its two rim edges are NOT stored: two ruled links may end on one
    /// cap and share a rim (the flat's chord between the rod's two
    /// creases), which the first carve splits, so the carve reads them
    /// live ([`cap_rims`]) and the plan only checks the shape.
    cap: FaceKey,
    /// The foot of `face_a`'s trimline on the cap — where that trimline
    /// meets the cap plane, on the rim edge the cap shares with
    /// `face_a`.
    foot_a: Point3<T>,
    /// Likewise for `face_b`.
    foot_b: Point3<T>,
    /// The section circle's centre: the spine's crossing of the cap
    /// plane. Its radius is the band's.
    center: Point3<T>,
}

/// **A ruled link whose two ends are transverse caps**, read off the
/// source body before any mutation. The battery classified each end
/// (`fillet3_cap_transverse`); this token reads the structure that
/// classification rests on and refuses where the body disagrees with
/// the verdict.
pub(in crate::blend) struct RuledPlan<'a, T: Real> {
    link: AdmittedOpen<'a, T>,
    ends: [CapEnd<T>; 2],
    /// The band's radius — the section circles' too.
    radius: T,
}

impl<'a, T: Decide + Bounds> RuledPlan<'a, T> {
    /// Plan one ruled link's carve.
    ///
    /// # Errors
    ///
    /// [`BlendError::UnsupportedGeometry`] when the link's band is not
    /// a cylinder or a trimline not a line; [`BlendError::UnsupportedChain`]
    /// when a support carries a ring or the crease is not on its
    /// support's outer cycle, or when a cap rim is itself requested;
    /// [`BlendError::BodyNotIntact`] when an end is not among the
    /// verdict's transverse caps, or its incidence is not the cap the
    /// verdict classified.
    pub(in crate::blend) fn plan(
        body: &Body<T>,
        link: AdmittedOpen<'a, T>,
        opens: &[AdmittedOpen<'_, T>],
        caps: &[VertexKey],
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
        let trim_origin = |trim: &Curve3<T>| -> Result<Point3<T>, BlendError> {
            match *trim {
                Curve3::Line { origin, .. } => Ok(origin),
                _ => Err(unbuilt_geometry(
                    EntityId::Edge(edge),
                    "an open link's trimline is not a line",
                )),
            }
        };
        let (q_a, q_b) = (
            trim_origin(&l.blend.trim_a.0)?,
            trim_origin(&l.blend.trim_b.0)?,
        );

        // The supports: ring-free, and carrying the crease on their
        // outer cycle. A ring on a curved support is not carried
        // through by this carve, and a crease on a ring would put the
        // trimline `mef` across a ring's loop. The CAP's rings are NOT
        // checked, deliberately: the cut-off `mef` runs on the cap's
        // outer cycle and leaves the old face's rings on the old face,
        // so a bored rod's cap keeps its bore
        // (`tests/review_fillet_h7_r1_probes.rs`, the cap-with-a-ring
        // row).
        let (hp, hm) = halves_of(body, edge)
            .ok_or_else(|| not_intact(EntityId::Edge(edge), "a ruled link's edge"))?;
        for (face, half) in [(l.face_a, hp), (l.face_b, hm)] {
            let fd = body
                .get_face(face)
                .ok_or_else(|| not_intact(EntityId::Face(face), "a ruled link's support"))?;
            if !fd.rings.is_empty() {
                return Err(unbuilt_chain(
                    edge,
                    "a ruled band's support face carries a ring; the band's carve on a curved \
                     support does not carry rings through",
                ));
            }
            if loop_of_half(body, half) != Some(fd.outer) {
                return Err(unbuilt_chain(
                    edge,
                    "a ruled band's edge is not on its support's outer cycle",
                ));
            }
            if face_of_half(body, half) != Some(face) {
                return Err(not_intact(
                    EntityId::Edge(edge),
                    "a ruled link's half-edges do not lie in the faces the verdict names",
                ));
            }
        }

        let mut ends = Vec::with_capacity(2);
        for v in [l.start, l.end] {
            // The battery's classification, read rather than re-made:
            // predicate 6 tagged this end `TransverseCap` and the
            // verdict carries it.
            if !caps.contains(&v) {
                return Err(not_intact(
                    EntityId::Vertex(v),
                    "a ruled link's end is not among the transverse caps the verdict classified",
                ));
            }
            let (rim_a, rim_b, cap) = cap_rims(body, v, edge, l.face_a, l.face_b)?;
            if opens.iter().any(|o| o.edge() == rim_a || o.edge() == rim_b) {
                return Err(unbuilt_chain(
                    edge,
                    "a ruled band's cap rim is itself requested; a cap whose rim edges are \
                     blended too is not implemented",
                ));
            }
            // The cap plane, from the STORED surface — the battery's
            // classification read the same one.
            let Some(Surface::Plane {
                origin: po,
                normal: n,
                ..
            }) = body.get_face(cap).and_then(|f| body.get_surface(f.surface))
            else {
                return Err(not_intact(
                    EntityId::Face(cap),
                    "a transverse cap's stored surface is not a plane",
                ));
            };
            // Carry a point along the ruling to the cap plane. The
            // ruling is transverse to the plane by the battery's
            // classification (its normal is parallel to `tau`), so the
            // quotient is total.
            let section = |p: Point3<T>| p + tau * ((*po - p).dot(*n) / tau.dot(*n));
            ends.push(CapEnd {
                vertex: v,
                cap,
                foot_a: section(q_a),
                foot_b: section(q_b),
                center: section(spine_origin),
            });
        }
        let Ok(ends) = <[CapEnd<T>; 2]>::try_from(ends) else {
            unreachable!("ruled plan: one cap per end of the link's two ends, pushed above")
        };
        Ok(Self { link, ends, radius })
    }

    /// The admitted link this plan carves.
    pub(in crate::blend) fn link(&self) -> AdmittedOpen<'a, T> {
        self.link
    }
}

/// **The two rim edges of a transverse cap at `vertex`**, and the cap
/// face, read off the LIVE body through the battery's one home for the
/// rule ([`cap_incidence`]): of the three edges at the
/// vertex, the two other than the crease each join one of the link's
/// supports to one third face, and that face — one face, shared — is
/// the cap. Returned as `(rim on face_a, rim on face_b, cap)`. Where the
/// battery reports an end that does not have this shape as
/// unclassifiable, the surgery reports it as a body that disagrees with
/// the verdict it was handed — the same fact, in each door's own words.
///
/// # Errors
///
/// [`BlendError::BodyNotIntact`]: the incidence is not the transverse
/// cap the battery classified.
fn cap_rims<T: Decide>(
    body: &Body<T>,
    vertex: VertexKey,
    crease: EdgeKey,
    face_a: FaceKey,
    face_b: FaceKey,
) -> Result<(EdgeKey, EdgeKey, FaceKey), BlendError> {
    cap_incidence(body, vertex, crease, face_a, face_b).ok_or_else(|| {
        not_intact(
            EntityId::Vertex(vertex),
            "a ruled link's end is not the transverse cap the verdict classified: its other \
             two edges do not each join one support to one shared cap face",
        )
    })
}

/// A split rim edge, as the carve needs it: the piece still touching
/// the old vertex (it dies with the sliver) and the foot vertex.
struct SplitRim {
    near: EdgeKey,
    foot: VertexKey,
}

/// Split one cap rim edge at the trimline's foot on it, recording the
/// foot and the surviving piece as births of this carve.
///
/// **The rim may already be a fragment.** Two creases on one cap share
/// the rim between them (the rod's two creases share the flat's chord),
/// so the second carve splits a piece the first one left — a key that is
/// either the SOURCE rim's or a fresh one, and in either case already
/// carries a `meridian_remnants` row. Provenance is read off that row:
/// the surviving piece is recorded as a fragment of the ORIGINAL source,
/// the stale fragment row is retired, and only a source key that dies is
/// a retirement (a minted piece that dies needs no row). Without this the
/// second split recorded the survivor twice, which the document layer's
/// emitter refuses as "the surgery recorded one entity twice".
#[allow(clippy::too_many_arguments)]
fn split_rim<T: Decide + Bounds>(
    body: &mut Body<T>,
    rim: EdgeKey,
    crease: EdgeKey,
    vertex: VertexKey,
    support: FaceKey,
    foot: Point3<T>,
    rec: &mut BlendNaming,
    tol: Tol,
) -> Result<SplitRim, BlendError> {
    let t = seam_split_param(body, rim, crease, foot)?;
    let created = body
        .split_edge(rim, t, tol)
        .map_err(|e| op("cap rim split", e))?;
    let (near, far) = if edge_touches(body, rim, vertex) {
        (rim, created.new_edge)
    } else {
        (created.new_edge, rim)
    };
    let source = rec
        .meridian_remnants
        .iter()
        .find(|(piece, _)| *piece == rim)
        .map_or(rim, |(_, source)| *source);
    rec.meridian_remnants.retain(|(piece, _)| *piece != rim);
    rec.feet.push((created.vertex, vertex, support));
    rec.meridian_remnants.push((far, source));
    if near == source {
        rec.dead.edges.push(source);
    }
    Ok(SplitRim {
        near,
        foot: created.vertex,
    })
}

/// **Carve one ruled link**: the band between its two transverse caps.
/// Returns the band face and the new edges awaiting their descriptions.
///
/// # Errors
///
/// [`BlendError::Op`] when an Euler operator refuses;
/// [`BlendError::UnsupportedChain`] / [`BlendError::UnsupportedGeometry`]
/// from the split parameter (a foot off its rim's span, an uncertified
/// rim); [`BlendError::BodyNotIntact`] where a cycle read disagrees
/// with the plan.
pub(in crate::blend) fn ruled_phase<T: Decide + Bounds>(
    body: &mut Body<T>,
    plan: &RuledPlan<'_, T>,
    sources: &SourceFaces,
    rec: &mut BlendNaming,
    tol: Tol,
) -> Result<(FaceKey, Described<T>), BlendError> {
    let l = plan.link.link();
    let crease = l.edge;
    let mut described: Described<T> = Vec::new();

    // ---- (1) Per cap: split both rims at their feet, then `mef` the
    // cut-off arc across the cap between the two feet. The run from
    // the first foot through the old vertex to the second is what
    // moves onto the new face, so the new face is the SLIVER and the
    // cap keeps its key, surface, sense and rings. ----
    let mut slivers: Vec<(SplitRim, SplitRim)> = Vec::with_capacity(2);
    for end in &plan.ends {
        let v = end.vertex;
        // Live, not planned: an earlier link's carve on the same cap
        // may have split the rim this end shares with it.
        let (rim_a, rim_b, cap) = cap_rims(body, v, crease, l.face_a, l.face_b)?;
        if cap != end.cap {
            return Err(not_intact(
                EntityId::Vertex(v),
                "a transverse cap's face is not the one the plan read",
            ));
        }
        let a = split_rim(body, rim_a, crease, v, l.face_a, end.foot_a, rec, tol)?;
        let b = split_rim(body, rim_b, crease, v, l.face_b, end.foot_b, rec, tol)?;
        // In the cap's cycle the half-edge ENDING at the old vertex
        // starts at one foot; two positions on, the half-edge starts
        // at the other.
        let ends_at_v = |body: &Body<T>, he: HalfEdgeKey| body.half_edge_end(he) == Some(v);
        let (he1, he2, x, y) = chord_site(body, end.cap, |row| ends_at_v(body, row.0), 0, 2)?;
        if !((x == a.foot && y == b.foot) || (x == b.foot && y == a.foot)) {
            return Err(not_intact(
                EntityId::Vertex(v),
                "a cap's cycle around the old vertex is not flanked by the two feet just split",
            ));
        }
        let (px, py) = (
            point_of(body, x).ok_or_else(|| not_intact(EntityId::Vertex(x), "a foot"))?,
            point_of(body, y).ok_or_else(|| not_intact(EntityId::Vertex(y), "a foot"))?,
        );
        // Scaffold chord now (the corner-arc precedent); the exact
        // section arc is attached in the description pass.
        let created = body
            .mef(
                MefSite::Chords { he1, he2 },
                EdgeCurveSpec::line_between(px, py),
                FaceSurface::Inherit,
                tol,
            )
            .map_err(|e| op("cap cut-off mef", e))?;
        described.push((
            created.edge,
            ContactCarrier::TransverseArc {
                center: end.center,
                radius: plan.radius,
            },
        ));
        rec.arcs.push((created.edge, v, crease));
        slivers.push((a, b));
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
        (l.face_a, hp, [slivers[0].0.foot, slivers[1].0.foot]),
        (l.face_b, hm, [slivers[0].1.foot, slivers[1].1.foot]),
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
        described.push((created.edge, ContactCarrier::TrimLine));
        rec.trims.push((created.edge, crease, face));
        trims.push(created.edge);
    }

    // ---- (3) Excise the crease across its two strips. ----
    sources.kef_minted(body, hp, "ruled crease kef")?;
    rec.dead.edges.push(crease);

    // ---- (4) Per cap: fold the sliver into the band across the
    // `face_a`-side rim remnant, then retire the `face_b`-side remnant
    // — now a spur — together with the old vertex. ----
    for (end, (a, b)) in plan.ends.iter().zip(&slivers) {
        let v = end.vertex;
        let (ahp, ahm) = halves_of(body, a.near)
            .ok_or_else(|| not_intact(EntityId::Edge(a.near), "a split rim's near piece"))?;
        // The sliver is on whichever side of the near piece is NOT the
        // merged strip face — which is the face the trimline of
        // `face_a` now bounds on its non-support side.
        let band_side = band_face(body, trims[0], l.face_a)?;
        let dying = if face_of_half(body, ahp) == Some(band_side) {
            ahm
        } else {
            ahp
        };
        sources.kef_minted(body, dying, "cap sliver kef")?;
        let (bhp, bhm) = halves_of(body, b.near)
            .ok_or_else(|| not_intact(EntityId::Edge(b.near), "a split rim's near piece"))?;
        let spur = if body.half_edge_end(bhm) == Some(v) {
            bhm
        } else {
            bhp
        };
        body.kev(spur).map_err(|e| op("cap vertex kev", e))?;
        rec.dead.vertices.push(v);
    }

    let band = band_face(body, trims[0], l.face_a)?;
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
