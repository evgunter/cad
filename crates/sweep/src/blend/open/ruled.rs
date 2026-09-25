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
//! (`surgery::split_fragment`, the one home of a band split and its
//! provenance, over `seam_split_param`'s split parameter)
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
//! **What the carve removes from a cap, metered first.** On the convex
//! side the cut-off takes the sliver out of the cap, and the `mef` that
//! does it leaves every OTHER cycle of the cap where it was. So before
//! any mutation each such cycle — the cap's rings, and its outer cycle
//! when the cut runs in a ring — is metered against the region the
//! sliver can occupy ([`CapSliver`], decided by the surgery's ring
//! carry-through pass under `fillet3_ring_clearance`); one that is not
//! definitely clear of it refuses `RingClearance` at the cap. On the
//! concave side the sliver is void of the source, so no cycle of the
//! cap can lie in it, and there is nothing to meter.
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

use geom::Curve3;
use geom::Surface;
use geom_brep::EdgeCurveSpec;
use geom_core::{Bounds, Decide, Point3, Real, Tol};
use topo::{
    Body, EdgeKey, EntityId, FaceKey, FaceSurface, HalfEdgeKey, LoopKey, MefSite, VertexKey,
};

use crate::blend::BlendError;
use crate::blend::admit::AdmittedOpen;
use crate::blend::battery::{Convexity, cap_incidence};
use crate::blend::naming::BlendNaming;
use crate::blend::surgery::{
    ContactCarrier, Described, SourceFaces, SplitFragments, chord_site, face_of_half, halves_of,
    loop_of_half, not_intact, op, point_of, retire_fragment, seam_split_param, split_fragment,
    unbuilt_chain, unbuilt_geometry,
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
    /// The cap's cycle that carries `vertex` — the one the cut-off
    /// `mef` runs in, its outer cycle or a ring.
    cut: LoopKey,
    /// The farthest the sliver reaches from `center` ([`rim_reach`]).
    reach: T,
}

/// **The region a convex ruled cut-off can remove from one cap**, as
/// the surgery's ring carry-through pass meters it.
///
/// The sliver is bounded by the cut-off arc (on the section circle,
/// radius `radius` about `center`) and the two rim pieces from the feet
/// to the old vertex. It lies in the closed annulus
/// `radius ≤ ‖p − center‖ ≤ reach`: outside the section circle's open
/// disc, because that disc is the ball's section, which is tangent to
/// both rims and lies in the material the band keeps; and within
/// `reach`, because the distance to `center` is convex, so its maximum
/// over the sliver is taken on the sliver's boundary, and `reach` is
/// that maximum over each boundary piece. A cycle of the cap clear of
/// the annulus is clear of the sliver; the converse does not hold, and
/// that is the meter's conservative direction.
pub(in crate::blend) struct CapSliver<T: Real> {
    /// The cap face the sliver is cut from.
    pub(in crate::blend) cap: FaceKey,
    /// The cycle the cut runs in; every OTHER cycle of `cap` is
    /// metered.
    pub(in crate::blend) cut: LoopKey,
    /// The section circle's centre.
    pub(in crate::blend) center: Point3<T>,
    /// The band's radius: the annulus's inner radius.
    pub(in crate::blend) radius: T,
    /// The annulus's outer radius.
    pub(in crate::blend) reach: T,
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
    /// a cylinder, a trimline not a line, or a cap rim neither a line
    /// nor a circle ([`rim_reach`]); [`BlendError::UnsupportedChain`]
    /// when a support carries a ring, or when a cap rim is itself
    /// requested; [`BlendError::BodyNotIntact`] when the crease's
    /// halves do not lie in the supports the verdict names, an end is
    /// not among the verdict's transverse caps, or its incidence is not
    /// the cap the verdict classified.
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

        // The supports: each carries its half of the crease, and is
        // ring-free — a ring on a curved support is not carried
        // through by this carve. Together the two put the crease on
        // the support's OUTER cycle (a half-edge's loop is a cycle of
        // its face, and a ring-free face has one), which is where the
        // trimline `mef` hangs ([`chord_site`]). The CAP's other cycles
        // are not refused here: the cut-off `mef` leaves each on the
        // cap, so each is metered against the region the cut removes
        // ([`CapSliver`]) by the surgery's ring carry-through pass.
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
            if !fd.rings.is_empty() {
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
                    "a ruled band's cap rim is itself requested, and blending it too is not \
             implemented",
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
            let (foot_a, foot_b, center) = (section(q_a), section(q_b), section(spine_origin));
            let pv = point_of(body, v)
                .ok_or_else(|| not_intact(EntityId::Vertex(v), "a transverse cap's old vertex"))?;
            let reach = rim_reach(body, rim_a, edge, v, pv, foot_a, center)?
                .max(rim_reach(body, rim_b, edge, v, pv, foot_b, center)?);
            // The cut cycle: the cap's loop through the rim's cap-side
            // half — a cycle of the cap, carrying the old vertex.
            let (rh1, rh2) = halves_of(body, rim_a)
                .ok_or_else(|| not_intact(EntityId::Edge(rim_a), "a transverse cap's rim"))?;
            let cut = [rh1, rh2]
                .into_iter()
                .find(|&h| face_of_half(body, h) == Some(cap))
                .and_then(|h| loop_of_half(body, h))
                .ok_or_else(|| {
                    not_intact(
                        EntityId::Edge(rim_a),
                        "a transverse cap's rim has no half in the cap",
                    )
                })?;
            ends.push(CapEnd {
                vertex: v,
                cap,
                foot_a,
                foot_b,
                center,
                cut,
                reach,
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

    /// The slivers this link's cut-offs REMOVE from its caps: one per
    /// end on a convex link, none on a concave one, whose slivers are
    /// void of the source and are added to the caps rather than taken
    /// from them.
    pub(in crate::blend) fn removed_slivers(&self) -> Vec<CapSliver<T>> {
        match self.link.convexity() {
            Convexity::Concave => Vec::new(),
            Convexity::Convex => self
                .ends
                .iter()
                .map(|e| CapSliver {
                    cap: e.cap,
                    cut: e.cut,
                    center: e.center,
                    radius: self.radius,
                    reach: e.reach,
                })
                .collect(),
        }
    }
}

/// **How far one rim piece reaches from the section centre**: the
/// maximum of `‖p − center‖` over the piece of `rim` between the foot
/// and the old vertex, closed form over the stored carrier.
///
/// The distance to a point is convex along a line, so on a LINE rim its
/// maximum over the piece is at an end. On a CIRCLE rim of centre `o`
/// and radius `R` it is largest at the circle's point opposite
/// `center`, `‖o − center‖ + R`, and it rises monotonically from the
/// point nearest `center` to that one. The foot is where the band
/// touches the support, so `center` lies on the rim's normal at the
/// foot and the foot is the circle's nearest or farthest point from
/// it; a piece that starts there and spans less than half a turn
/// therefore peaks at an end. Its span is read off the stored window —
/// the split parameter of the foot against the window end at the old
/// vertex (`topo`'s edge split runs the window along `he_plus`) — as a
/// BOUNDS-lane representation read: where the span is not certainly
/// under half a turn the reach is the whole circle's, which holds
/// either way, so neither answer is unsound.
///
/// # Errors
///
/// [`BlendError::UnsupportedGeometry`] when the rim carries no
/// certified line or circle, or from the foot's split parameter;
/// [`BlendError::BodyNotIntact`] when the rim does not resolve.
fn rim_reach<T: Decide + Bounds>(
    body: &Body<T>,
    rim: EdgeKey,
    crease: EdgeKey,
    vertex: VertexKey,
    pv: Point3<T>,
    foot: Point3<T>,
    center: Point3<T>,
) -> Result<T, BlendError> {
    let ends = (foot - center).norm().max((pv - center).norm());
    let ed = body
        .get_edge(rim)
        .ok_or_else(|| not_intact(EntityId::Edge(rim), "a transverse cap's rim"))?;
    let Some(c) = body.get_curve_geom(ed.curve).and_then(|g| g.certified()) else {
        return Err(unbuilt_geometry(
            EntityId::Edge(rim),
            "a transverse cap's rim carries no certified carrier",
        ));
    };
    match *c.carrier() {
        Curve3::Line { .. } => Ok(ends),
        Curve3::Circle {
            center: o, radius, ..
        } => {
            let t = seam_split_param(body, rim, crease, foot)?;
            let (t0, t1) = c.params();
            let starts_at_v = body.get_half_edge(ed.he_plus).map(|h| h.start) == Some(vertex);
            let span = if starts_at_v { t - t0 } else { t1 - t };
            if (T::pi() - span).lo() > 0.0 {
                Ok(ends)
            } else {
                Ok(ends.max((o - center).norm() + radius))
            }
        }
        _ => Err(unbuilt_geometry(
            EntityId::Edge(rim),
            "a transverse cap's rim is neither a line nor a circle, the only rims the \
             sliver's reach is closed-form over",
        )),
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

/// Split one cap rim edge at the trimline's foot on it, recording the
/// foot and the surviving piece as births of this carve.
///
/// The split's provenance — which piece is a fragment of which source,
/// and whether the dying piece is a retirement — is
/// [`split_fragment`]'s and [`retire_fragment`]'s, shared with the
/// ladder rim phase's meridian splits, and the answer travels in that
/// type rather than in a cap-rim re-wrap of it. The `near` piece always
/// dies here: it is the remnant the cap's `kev` folds away with the
/// sliver.
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
) -> Result<SplitFragments, BlendError> {
    let t = seam_split_param(body, rim, crease, foot)?;
    let frag = split_fragment(body, rim, vertex, t, rec, "cap rim split", tol)?;
    rec.feet.push((frag.vertex, vertex, support));
    retire_fragment(rec, frag.near, frag.source);
    Ok(frag)
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
    // cut-off arc across the cap between the two feet, in the cap's
    // cycle that carries the old vertex — its outer cycle, or a ring
    // where the crease runs along a through-hole. The run from the
    // first foot through the old vertex to the second is what moves
    // onto the new face, so the new face is the SLIVER and the cap
    // keeps its key, surface, sense, the cut cycle's outer/ring
    // designation, and its other cycles. ----
    let mut slivers: Vec<(SplitFragments, SplitFragments)> = Vec::with_capacity(2);
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
        if !((x == a.vertex && y == b.vertex) || (x == b.vertex && y == a.vertex)) {
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
            crease,
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
        (l.face_a, hp, [slivers[0].0.vertex, slivers[1].0.vertex]),
        (l.face_b, hm, [slivers[0].1.vertex, slivers[1].1.vertex]),
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
