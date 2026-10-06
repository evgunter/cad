//! **The cut-off** — how a straight band ENDS at a vertex where the
//! request names its edge alone: in the end face's plane section of it
//! (`CornerConfig::EndFace`, `RunOutPolicy::CutOffAtEndFace`). Both open
//! bands end this way, the ruled band at each of its caps and the
//! planar band at each end the request does not close with a corner
//! patch, so the end's plan, its two carve steps and the region it takes
//! from the end face live here once.
//!
//! At the old vertex `V` the two edges other than the band's own are the
//! RIMS the end face `C` shares with the band's supports; each support's
//! trimline meets `C`'s plane at a FOOT on that support's rim. The band's
//! section by `C` is the END CURVE between the two feet: a chord for a
//! plane band (the chamfer, at any angle), an arc of the band's radius
//! about the spine's crossing for a cylinder band whose spine is
//! perpendicular to `C` (the plane–plane fillet's, and the ruled band's
//! transverse cap).
//!
//! The carve, per end:
//!
//! 1. [`cut_off`]: split each rim at its foot and `mef` the end curve
//!    across `C`, in the cycle that carries `V`. The run from one foot
//!    through `V` to the other moves onto the new face — the SLIVER — so
//!    `C` keeps its key, surface, sense, its cut cycle's designation and
//!    its other cycles.
//! 2. The band's carve between its ends (the supports' trimlines, the
//!    crease's `kef`), which is each band's own.
//! 3. [`fold_sliver`]: `kef` the sliver into the band across the
//!    `face_a`-side rim remnant, and `kev` the `face_b`-side remnant —
//!    now a spur — together with `V`.
//!
//! **What the cut-off takes from `C`, metered first.** On the convex side
//! the sliver leaves `C`, and every other edge of `C` stays where it was,
//! so each is metered before any mutation against a region that encloses
//! the sliver ([`CapSliver`]); on the concave side the sliver is void of
//! the source and `C` gains it, so there is nothing to meter.

use geom::Curve3;
use geom::Surface;
use geom_brep::EdgeCurveSpec;
use geom_core::{Bounds, Decide, Point3, Real, Tol, Vec3};
use topo::{Body, EdgeKey, EntityId, FaceKey, FaceSurface, HalfEdgeKey, MefSite, VertexKey};

use crate::blend::BlendError;
use crate::blend::battery::{Convexity, cap_incidence};
use crate::blend::naming::BlendNaming;
use crate::blend::surgery::{
    CircleFrame, ContactCarrier, Piece, SourceFaces, SplitFragments, chord_site, face_of_half,
    halves_of, not_intact, op, piece_along, piece_distance, point_of, retire_fragment,
    seam_split_param, split_fragment, split_param_in_span, stored_piece, unbuilt_geometry,
    unbuilt_run_out,
};

/// The refusal for a cut-off whose foot does not land inside its rim's
/// span: the trimline runs past the end face's rim into the support,
/// which is a band running into a wall or a step.
pub(in crate::blend) const FOOT_INSIDE_A_FACE: &str = "a straight band's trimline meets the end \
     face's plane past its rim, so its foot lands inside a face rather than on that rim";

/// The band's section by the end face, between the two feet.
#[derive(Clone, Copy)]
pub(in crate::blend) enum EndCurve<T: Real> {
    /// A plane band's: the straight chord.
    Chord,
    /// A cylinder band's at an end face perpendicular to its spine: the
    /// arc of the band's radius about the spine's crossing.
    Arc {
        /// The spine's crossing of the end face's plane.
        center: Point3<T>,
        /// The band's radius.
        radius: T,
    },
}

/// One cut-off end of a band, as the plan read it off the source body.
pub(in crate::blend) struct EndCut<T: Real> {
    /// The old vertex, which dies with the sliver.
    pub(in crate::blend) vertex: VertexKey,
    /// The end face — the one face at `vertex` that is not a support.
    /// Its two rims are NOT stored: two bands may end on one end face
    /// and share a rim (the flat's chord between a rod's two creases,
    /// a box face's edge between two cut-offs), which the first carve
    /// splits, so the carve reads them live ([`end_rims`]).
    pub(in crate::blend) face: FaceKey,
    /// The foot of `face_a`'s trimline, on the rim the end face shares
    /// with `face_a`.
    pub(in crate::blend) foot_a: Point3<T>,
    /// Likewise for `face_b`.
    pub(in crate::blend) foot_b: Point3<T>,
    /// The end curve between the feet.
    curve: EndCurve<T>,
    /// The region the cut-off removes from the end face, on a convex
    /// band; `None` on a concave one, whose sliver the end face gains.
    pub(in crate::blend) sliver: Option<CapSliver<T>>,
}

impl<T: Decide + Bounds> EndCut<T> {
    /// **Plan one cut-off end** of the band over `crease` between
    /// `face_a` and `face_b`, whose trimlines on those faces pass
    /// through `q_a` and `q_b` along `along`; `curve` says what the
    /// band's section is, its arc centre still to be carried to the end
    /// face (`spine` is a point of the spine, read only for an arc).
    ///
    /// Every point is a stored trimline or spine point carried along
    /// `along` to the stored end plane; nothing is sampled. `along` is
    /// transverse to the plane by the battery's classification of this
    /// end (independent support normals for a plane band, a
    /// perpendicular end face for a cylinder band), so the quotient is
    /// total.
    ///
    /// # Errors
    ///
    /// [`BlendError::UnsupportedRunOut`] when a foot does not land
    /// inside its rim's span ([`FOOT_INSIDE_A_FACE`]);
    /// [`BlendError::UnsupportedGeometry`] when a rim carries no
    /// certified line or circle;
    /// [`BlendError::BodyNotIntact`] when the end is not the incidence
    /// the verdict classified or the end face is not a plane.
    #[allow(clippy::too_many_arguments)] // the band's own reads, one per argument.
    pub(in crate::blend) fn plan(
        body: &Body<T>,
        vertex: VertexKey,
        crease: EdgeKey,
        (face_a, face_b): (FaceKey, FaceKey),
        (q_a, q_b): (Point3<T>, Point3<T>),
        along: Vec3<T>,
        curve: EndCurve<T>,
        convexity: Convexity,
    ) -> Result<Self, BlendError> {
        let (rim_a, rim_b, face) = end_rims(body, vertex, crease, face_a, face_b)?;
        // The end plane, from the STORED surface — the battery's
        // classification read the same one.
        let Some(Surface::Plane {
            origin: po,
            normal: n,
            ..
        }) = body
            .get_face(face)
            .and_then(|f| body.get_surface(f.surface))
        else {
            return Err(not_intact(
                EntityId::Face(face),
                "a cut-off's end face's stored surface is not a plane",
            ));
        };
        let section = |p: Point3<T>| p + along * ((*po - p).dot(*n) / along.dot(*n));
        let (foot_a, foot_b) = (section(q_a), section(q_b));
        // The feet must land on the rims, strictly inside their spans:
        // anywhere else the band runs into a face the cut-off does not
        // touch.
        for (rim, foot) in [(rim_a, foot_a), (rim_b, foot_b)] {
            if split_param_in_span(body, rim, foot)?.is_none() {
                return Err(unbuilt_run_out(
                    EntityId::Vertex(vertex),
                    FOOT_INSIDE_A_FACE,
                ));
            }
        }
        let curve = match curve {
            EndCurve::Chord => EndCurve::Chord,
            EndCurve::Arc { center, radius } => EndCurve::Arc {
                center: section(center),
                radius,
            },
        };
        let sliver = match convexity {
            Convexity::Concave => None,
            Convexity::Convex => {
                let rims = [(rim_a, foot_a), (rim_b, foot_b)];
                Some(match curve {
                    EndCurve::Chord => CapSliver::of_chord(body, crease, vertex, face, rims)?,
                    EndCurve::Arc { center, radius } => {
                        CapSliver::of_arc(body, crease, vertex, face, rims, center, radius)?
                    }
                })
            }
        };
        Ok(Self {
            vertex,
            face,
            foot_a,
            foot_b,
            curve,
            sliver,
        })
    }

    /// The end curve's carrier, as the description pass reads it.
    fn carrier(&self) -> ContactCarrier<T> {
        match self.curve {
            EndCurve::Chord => ContactCarrier::Chord,
            EndCurve::Arc { center, radius } => ContactCarrier::TransverseArc { center, radius },
        }
    }
}

/// **A region that encloses what a convex cut-off removes from its end
/// face**, as the surgery's ring carry-through pass meters it.
///
/// The sliver `S` is bounded by the end curve `A` from one foot to the
/// other and the two rim pieces from the feet to the old vertex `V`. It
/// lies in the region
///
/// `Ω = { radius ≤ ‖p − center‖ ≤ reach } ∩ { (p − center)·toward ≥ floor }`:
///
/// - outside the disc of `radius` about `center`: on an arc end, that
///   disc is the band's section, which is tangent to both rims and lies
///   in the material the band keeps; on a chord end `radius` is zero and
///   the clause is empty;
/// - within `reach` and above `floor`, because `‖p − center‖` is convex
///   and `(p − center)·toward` linear, so over the compact `S` the first
///   is largest, and the second smallest, somewhere on `S`'s boundary,
///   which is `A` and the two rim pieces; `reach` and `floor` are those
///   extremes over the three pieces, each in closed form over its own
///   window ([`piece_distance`], [`piece_along`]).
///
/// `toward` is the unit direction from `center` to `V`. Its choice is
/// free for soundness — every unit direction gives a sound `floor` —
/// and this one lays the half-plane's edge across the corner, so the
/// part of the disc on the far side of `center` from `V`, which is kept
/// material, lies outside `Ω`.
///
/// An arc `A` is not an edge of the source, so the plan describes it:
/// at a foot the rim and the section circle are tangent and `S` is the
/// cusp between them, so `A` leaves the foot in the direction the rim
/// piece leaves it towards `V` — the arc from that foot turning about
/// `(foot − center) × tangent` to the other foot. A chord `A` is the
/// segment between the feet, and `center` its midpoint.
///
/// An edge that misses `Ω` misses `S`; the converse does not hold, and
/// that is the meter's conservative direction.
///
/// Two of its terms are pinned by no assembly row, only by the piece
/// meters' unit row: the arc's term of `floor`, which binds only when a
/// rim piece spans more than π, and the whole-circle arm of an arc
/// extreme on a cap edge (work item
/// `cap-sliver-floor-arc-term-and-whole-circle-arm-are-unpinned`).
pub(in crate::blend) struct CapSliver<T: Real> {
    /// The end face the sliver is cut from.
    pub(in crate::blend) cap: FaceKey,
    /// The two rim edges the cut shortens, which the meter skips: they
    /// bound the sliver rather than lie across it. Each joins the end
    /// face to one support, so neither appears in any cycle of the end
    /// face but the one the cut runs in.
    pub(in crate::blend) rims: [EdgeKey; 2],
    center: Point3<T>,
    /// `Ω`'s inner radius: the band's on an arc end, zero on a chord.
    radius: T,
    /// `Ω`'s outer radius.
    reach: T,
    /// The unit direction from `center` to the old vertex.
    toward: Vec3<T>,
    /// The least `(p − center)·toward` over the sliver.
    floor: T,
}

impl<T: Bounds> CapSliver<T> {
    /// **How clear one end-face edge is of the sliver**: the `carrier`
    /// over `window` misses `Ω` when this is positive, being the largest
    /// of how far the edge stays inside the inner disc, beyond `reach`,
    /// and short of `floor`. `None` for a carrier with no closed form
    /// ([`piece_distance`]).
    ///
    /// Each term clears the whole edge on its own, so an edge that
    /// misses `Ω` only by leaving it through different faces at
    /// different points reads not-clear — a conservative refusal, never
    /// a silent pass.
    pub(in crate::blend) fn clearance(&self, carrier: &Curve3<T>, window: (T, T)) -> Option<T> {
        let (near, far) = piece_distance(carrier, window, self.center)?;
        let (_, high) = piece_along(carrier, window, self.center, self.toward)?;
        Some(
            (self.radius - far)
                .max(near - self.reach)
                .max(self.floor - high),
        )
    }
}

impl<T: Decide + Bounds> CapSliver<T> {
    /// The rims' pieces from their feet to the old vertex, read off the
    /// source, with the far extent of each from `center` and its least
    /// height along `toward`.
    fn rim_extremes(
        body: &Body<T>,
        crease: EdgeKey,
        vertex: VertexKey,
        rims: [(EdgeKey, Point3<T>); 2],
        center: Point3<T>,
        toward: Vec3<T>,
    ) -> Result<([T; 2], [T; 2], Vec3<T>), BlendError> {
        let unsupported = |rim: EdgeKey| {
            unbuilt_geometry(
                EntityId::Edge(rim),
                "an end face's rim is neither a line nor a circle, the only rims the sliver's \
                 extent is closed-form over",
            )
        };
        let mut far = [T::zero(); 2];
        let mut low = far;
        let mut leaves = Vec3::new(T::zero(), T::zero(), T::zero());
        for (i, (rim, foot)) in rims.into_iter().enumerate() {
            let ((carrier, window), leaving) = rim_piece(body, rim, crease, vertex, foot)?;
            far[i] = piece_distance(carrier, window, center)
                .ok_or_else(|| unsupported(rim))?
                .1;
            low[i] = piece_along(carrier, window, center, toward)
                .ok_or_else(|| unsupported(rim))?
                .0;
            if i == 0 {
                leaves = leaving;
            }
        }
        Ok((far, low, leaves))
    }

    /// **The region a cut-off on an arc removes**, read off the source
    /// before any mutation. `rims` pairs each rim with its foot,
    /// `face_a`'s first.
    ///
    /// # Errors
    ///
    /// [`BlendError::UnsupportedGeometry`] when a rim carries no
    /// certified line or circle, or from a foot's split parameter;
    /// [`BlendError::BodyNotIntact`] when the old vertex, a rim, or the
    /// end face's half of a rim does not resolve.
    fn of_arc(
        body: &Body<T>,
        crease: EdgeKey,
        vertex: VertexKey,
        cap: FaceKey,
        rims: [(EdgeKey, Point3<T>); 2],
        center: Point3<T>,
        radius: T,
    ) -> Result<Self, BlendError> {
        let pv = point_of(body, vertex)
            .ok_or_else(|| not_intact(EntityId::Vertex(vertex), "a cut-off's old vertex"))?;
        let toward = (pv - center).normalize();
        let (far, low, leaves_a) = Self::rim_extremes(body, crease, vertex, rims, center, toward)?;
        let [(rim_a, foot_a), (rim_b, foot_b)] = rims;
        // The cut-off arc, from `face_a`'s foot along the rim's tangent
        // there (the type's docs), to `face_b`'s: its span read in
        // `(0, τ]` past the start.
        let from = foot_a - center;
        let arc = CircleFrame {
            center,
            axis: from.cross(leaves_a).normalize(),
            radius,
            u_ref: from.normalize(),
        };
        let (low_arc, _) = arc.along((T::zero(), arc.past(T::zero(), foot_b)), center, toward);
        Ok(Self {
            cap,
            rims: [rim_a, rim_b],
            center,
            radius,
            reach: radius.max(far[0]).max(far[1]),
            toward,
            floor: low_arc.min(low[0]).min(low[1]),
        })
    }

    /// **The region a cut-off on a chord removes** — the triangle of the
    /// two feet and the old vertex, its rims straight: `center` the
    /// chord's midpoint, no inner disc, and the extremes over the two
    /// rim pieces and the chord, whose own extremes are at its ends.
    ///
    /// # Errors
    ///
    /// As [`Self::of_arc`].
    fn of_chord(
        body: &Body<T>,
        crease: EdgeKey,
        vertex: VertexKey,
        cap: FaceKey,
        rims: [(EdgeKey, Point3<T>); 2],
    ) -> Result<Self, BlendError> {
        let pv = point_of(body, vertex)
            .ok_or_else(|| not_intact(EntityId::Vertex(vertex), "a cut-off's old vertex"))?;
        let [(rim_a, foot_a), (rim_b, foot_b)] = rims;
        let half = T::from_f64(0.5);
        let center = foot_a + (foot_b - foot_a) * half;
        let toward = (pv - center).normalize();
        let (far, low, _) = Self::rim_extremes(body, crease, vertex, rims, center, toward)?;
        let chord_far = (foot_a - center).norm().max((foot_b - center).norm());
        let chord_low = (foot_a - center)
            .dot(toward)
            .min((foot_b - center).dot(toward));
        Ok(Self {
            cap,
            rims: [rim_a, rim_b],
            center,
            radius: T::zero(),
            reach: chord_far.max(far[0]).max(far[1]),
            toward,
            floor: chord_low.min(low[0]).min(low[1]),
        })
    }
}

/// **One rim's piece from its foot to the old vertex**: the rim's stored
/// carrier, the window of the piece on it, and the unit tangent at the
/// foot pointing along the piece towards the vertex. The stored window
/// runs along `he_plus` (`topo`'s edge-direction invariant), so the
/// vertex sits at the window's start when `he_plus` starts there and at
/// its end otherwise; the foot's parameter is the one the carve splits
/// the rim at.
///
/// # Errors
///
/// [`BlendError::UnsupportedGeometry`] when the rim carries no certified
/// carrier, or from the foot's split parameter;
/// [`BlendError::BodyNotIntact`] when the rim does not resolve or does
/// not end at `vertex`.
fn rim_piece<T: Decide + Bounds>(
    body: &Body<T>,
    rim: EdgeKey,
    crease: EdgeKey,
    vertex: VertexKey,
    foot: Point3<T>,
) -> Result<(Piece<'_, T>, Vec3<T>), BlendError> {
    let he_plus = body
        .get_edge(rim)
        .ok_or_else(|| not_intact(EntityId::Edge(rim), "an end face's rim"))?
        .he_plus;
    let Some((carrier, (t0, t1))) = stored_piece(body, rim)? else {
        return Err(unbuilt_geometry(
            EntityId::Edge(rim),
            "an end face's rim carries no certified carrier",
        ));
    };
    let t = seam_split_param(body, rim, crease, foot)?;
    let forward = carrier.ders1(t).1.normalize();
    if body.get_half_edge(he_plus).map(|h| h.start) == Some(vertex) {
        Ok(((carrier, (t0, t)), -forward))
    } else if body.half_edge_end(he_plus) == Some(vertex) {
        Ok(((carrier, (t, t1)), forward))
    } else {
        Err(not_intact(
            EntityId::Edge(rim),
            "an end face's rim does not end at the band's old vertex",
        ))
    }
}

/// **The two rims of an end face at `vertex`**, and the end face, read
/// off the LIVE body through the battery's one home for the rule
/// ([`cap_incidence`]): of the three edges at the vertex, the two other
/// than the crease each join one of the band's supports to one third
/// face, and that face — one face, shared — is the end face. Returned as
/// `(rim on face_a, rim on face_b, end face)`. Where the battery reports
/// an end that does not have this shape as unclassifiable, the surgery
/// reports it as a body that disagrees with the verdict it was handed —
/// the same fact, in each door's own words.
///
/// # Errors
///
/// [`BlendError::BodyNotIntact`]: the incidence is not the end face the
/// battery classified.
pub(in crate::blend) fn end_rims<T: Decide>(
    body: &Body<T>,
    vertex: VertexKey,
    crease: EdgeKey,
    face_a: FaceKey,
    face_b: FaceKey,
) -> Result<(EdgeKey, EdgeKey, FaceKey), BlendError> {
    cap_incidence(body, vertex, crease, face_a, face_b).ok_or_else(|| {
        not_intact(
            EntityId::Vertex(vertex),
            "a band's end is not the end face the verdict classified: its other two edges do \
             not each join one support to one shared end face",
        )
    })
}

/// Split one rim at the trimline's foot on it, recording the foot and
/// the surviving piece as births of this carve.
///
/// The split's provenance — which piece is a fragment of which source,
/// and whether the dying piece is a retirement — is [`split_fragment`]'s
/// and [`retire_fragment`]'s, shared with the ladder rim phase's
/// meridian splits. The `near` piece always dies here: it is the remnant
/// the end face's `kev` folds away with the sliver.
#[allow(clippy::too_many_arguments)]
fn split_rim<T: Decide + Bounds + topo::AtRestPolicy>(
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
    let frag = split_fragment(body, rim, vertex, t, None, rec, "end rim split", tol)?;
    rec.feet.push((frag.vertex, vertex, support));
    retire_fragment(rec, frag.near, frag.source);
    Ok(frag)
}

/// The two split rims of one cut-off end — `face_a`'s, then `face_b`'s —
/// as [`cut_off`] leaves them for [`fold_sliver`].
pub(in crate::blend) struct CutRims {
    pub(in crate::blend) a: SplitFragments,
    pub(in crate::blend) b: SplitFragments,
}

/// **Carve step 1 at one end**: split both rims at their feet, then
/// `mef` the end curve across the end face between them, in the end
/// face's cycle that carries the old vertex. Returns the split rims and
/// the end curve's edge with the carrier the description pass attaches
/// (a scaffold chord until then).
///
/// # Errors
///
/// [`BlendError::Op`] when an Euler operator refuses;
/// [`BlendError::UnsupportedChain`] / [`BlendError::UnsupportedGeometry`]
/// from a split parameter; [`BlendError::BodyNotIntact`] where a cycle
/// read disagrees with the plan.
pub(in crate::blend) fn cut_off<T: Decide + Bounds + topo::AtRestPolicy>(
    body: &mut Body<T>,
    end: &EndCut<T>,
    crease: EdgeKey,
    (face_a, face_b): (FaceKey, FaceKey),
    rec: &mut BlendNaming,
    tol: Tol,
) -> Result<(CutRims, (EdgeKey, ContactCarrier<T>, EdgeKey)), BlendError> {
    let v = end.vertex;
    // Live, not planned: an earlier carve on the same end face may have
    // split the rim this end shares with it.
    let (rim_a, rim_b, face) = end_rims(body, v, crease, face_a, face_b)?;
    if face != end.face {
        return Err(not_intact(
            EntityId::Vertex(v),
            "a cut-off's end face is not the one the plan read",
        ));
    }
    let a = split_rim(body, rim_a, crease, v, face_a, end.foot_a, rec, tol)?;
    let b = split_rim(body, rim_b, crease, v, face_b, end.foot_b, rec, tol)?;
    // In the end face's cycle the half-edge ENDING at the old vertex
    // starts at one foot; two positions on, the half-edge starts at the
    // other.
    let ends_at_v = |body: &Body<T>, he: HalfEdgeKey| body.half_edge_end(he) == Some(v);
    let (he1, he2, x, y) = chord_site(body, end.face, |row| ends_at_v(body, row.0), 0, 2)?;
    if !((x == a.vertex && y == b.vertex) || (x == b.vertex && y == a.vertex)) {
        return Err(not_intact(
            EntityId::Vertex(v),
            "an end face's cycle around the old vertex is not flanked by the two feet just split",
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
        .map_err(|e| op("end cut-off mef", e))?;
    rec.arcs.push((created.edge, v, crease));
    Ok((CutRims { a, b }, (created.edge, end.carrier(), crease)))
}

/// **Carve step 3 at one end**: fold the sliver into `band` across the
/// `face_a`-side rim remnant, then retire the `face_b`-side remnant —
/// now a spur — together with the old vertex.
///
/// # Errors
///
/// [`BlendError::Op`] when an Euler operator refuses;
/// [`BlendError::BodyNotIntact`] when a remnant does not resolve;
/// [`BlendError::SurgeryInvariant`] from the face-destroying door.
pub(in crate::blend) fn fold_sliver<T: Decide>(
    body: &mut Body<T>,
    sources: &SourceFaces,
    band: FaceKey,
    vertex: VertexKey,
    rims: &CutRims,
    rec: &mut BlendNaming,
    tol: Tol,
) -> Result<(), BlendError> {
    let (ahp, ahm) = halves_of(body, rims.a.near)
        .ok_or_else(|| not_intact(EntityId::Edge(rims.a.near), "a split rim's near piece"))?;
    // The sliver is on whichever side of the near piece is NOT the band.
    let dying = if face_of_half(body, ahp) == Some(band) {
        ahm
    } else {
        ahp
    };
    sources.kef_minted(body, dying, "end sliver kef", tol)?;
    let (bhp, bhm) = halves_of(body, rims.b.near)
        .ok_or_else(|| not_intact(EntityId::Edge(rims.b.near), "a split rim's near piece"))?;
    let spur = if body.half_edge_end(bhm) == Some(vertex) {
        bhm
    } else {
        bhp
    };
    // The sliver's `kef` left the near piece a spur at the old vertex,
    // so it has valence one and the keys-only kill merges no fan.
    debug_assert!(
        body.kev_merged_members(spur).is_ok_and(|m| m.is_empty()),
        "end vertex kev: the near piece is a spur at the old vertex"
    );
    body.kev(spur).map_err(|e| op("end vertex kev", e))?;
    rec.dead.vertices.push(vertex);
    Ok(())
}
