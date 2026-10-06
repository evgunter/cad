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
//! plane band (the chamfer, at any angle); for a cylinder band (the
//! plane–plane fillet, the ruled band) an arc about the spine's
//! crossing, of a circle of the band's radius where the spine is
//! perpendicular to `C` and of an ellipse where it is oblique — the
//! kind `fillet3_cap_transverse` picked ([`EndSection`]).
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
//! **What the cut-off takes from `C`, metered first.** On either side
//! `C` loses the sliver: on the convex side it is cut away with the
//! material beyond the band, on the concave side the band's fill covers
//! it. Every other edge of `C` stays where it was, so each is metered
//! before any mutation against a region that encloses the sliver
//! ([`CapSliver`]), whichever the band's convexity.

use geom::Curve3;
use geom::Surface;
use geom_brep::EdgeCurveSpec;
use geom_core::{Band, Bounds, Decide, InfSpeed, Margin, Point3, Real, Sign, Tol, Vec3};
use topo::{Body, EdgeKey, EntityId, FaceKey, FaceSurface, HalfEdgeKey, MefSite, VertexKey};

use crate::blend::battery::{EndSection, cap_incidence};
use crate::blend::naming::BlendNaming;
use crate::blend::surgery::{
    CircleFrame, ContactCarrier, Piece, SourceFaces, SplitFragments, chord_site, face_of_half,
    halves_of, not_intact, op, piece_along, piece_distance, point_of, retire_fragment,
    split_fragment, split_param_in_span, stored_piece, unbuilt_geometry, unbuilt_run_out,
};
use crate::blend::{BlendDecision, BlendError, BlendSite, classify};

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
    /// A cylinder band's at an oblique end face: the arc of the end
    /// plane's section of the band, the ellipse about the spine's
    /// crossing with the band's radius for its minor semi-axis.
    Ellipse {
        /// The spine's crossing of the end face's plane.
        center: Point3<T>,
        /// The end face's unit normal.
        axis: Vec3<T>,
        /// The semi-major axis.
        major: T,
        /// The semi-minor axis: the band's radius.
        minor: T,
        /// The unit semi-major direction.
        u_major: Vec3<T>,
    },
}

impl<T: Real> EndCurve<T> {
    /// The end curve of a band whose section the battery picked:
    /// `spine` is a point of a cylinder band's spine (still to be
    /// carried to the end face) and `radius` its radius, read only for
    /// the round kinds.
    pub(in crate::blend) fn of(section: EndSection<T>, spine: Point3<T>, radius: T) -> Self {
        match section {
            EndSection::Chord => Self::Chord,
            EndSection::Circle => Self::Arc {
                center: spine,
                radius,
            },
            EndSection::Ellipse {
                major,
                u_major,
                normal,
            } => Self::Ellipse {
                center: spine,
                axis: normal,
                major,
                minor: radius,
                u_major,
            },
        }
    }

    /// The round kinds' centre and inner radius: the spine's crossing,
    /// and the radius of the disc about it the band's section bounds —
    /// on an ellipse its minor semi-axis, the disc its section contains.
    fn round(self) -> Option<(Point3<T>, T)> {
        match self {
            Self::Chord => None,
            Self::Arc { center, radius } => Some((center, radius)),
            Self::Ellipse { center, minor, .. } => Some((center, minor)),
        }
    }
}

/// One cut-off end of a band, as the plan read it off the source body.
pub(in crate::blend) struct EndCut<T: Real> {
    /// The old vertex, which dies with the sliver.
    pub(in crate::blend) vertex: VertexKey,
    /// The end face — the one face at `vertex` that is not a support.
    /// The carve reads its two rims live ([`end_rims`]) rather than off
    /// the sliver's source keys: two bands may end on one end face and
    /// share a rim (the flat's chord between a rod's two creases, a box
    /// face's edge between two cut-offs), which the first carve splits.
    pub(in crate::blend) face: FaceKey,
    /// The foot of `face_a`'s trimline, on the rim the end face shares
    /// with `face_a`.
    pub(in crate::blend) foot_a: Point3<T>,
    /// Likewise for `face_b`.
    pub(in crate::blend) foot_b: Point3<T>,
    /// The end curve between the feet.
    curve: EndCurve<T>,
    /// The region the cut-off removes from the end face.
    pub(in crate::blend) sliver: CapSliver<T>,
}

impl<T: Decide + Bounds> EndCut<T> {
    /// **Plan one cut-off end** of the band over `crease` between
    /// `face_a` and `face_b`, whose trimlines on those faces pass
    /// through `q_a` and `q_b` along `along`; `curve` says what the
    /// band's section is ([`EndCurve::of`]), its centre still to be
    /// carried to the end face.
    ///
    /// Every point is a stored trimline or spine point carried along
    /// `along` to the stored end plane; nothing is sampled. `along` is
    /// transverse to the plane by the battery's classification of this
    /// end (independent face normals at the vertex, or an end face
    /// perpendicular to a cylinder band's spine), so the quotient is
    /// total.
    ///
    /// # Errors
    ///
    /// [`BlendError::UnsupportedRunOut`] when a foot does not land
    /// inside its rim's span ([`FOOT_INSIDE_A_FACE`]);
    /// [`BlendError::UnsupportedGeometry`] when a rim carries no
    /// certified line, circle or ellipse;
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
            foot_param(body, rim, vertex, foot)?;
        }
        let curve = match curve {
            EndCurve::Chord => EndCurve::Chord,
            EndCurve::Arc { center, radius } => EndCurve::Arc {
                center: section(center),
                radius,
            },
            EndCurve::Ellipse {
                center,
                axis,
                major,
                minor,
                u_major,
            } => EndCurve::Ellipse {
                center: section(center),
                axis,
                major,
                minor,
                u_major,
            },
        };
        let sliver = CapSliver::of(
            body,
            vertex,
            face,
            [(rim_a, foot_a), (rim_b, foot_b)],
            curve,
        )?;
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
            EndCurve::Ellipse {
                center,
                major,
                minor,
                u_major,
                ..
            } => ContactCarrier::TransverseEllipse {
                center,
                major,
                minor,
                u_major,
            },
        }
    }
}

/// **A region that encloses what a cut-off removes from its end face**,
/// on either side, as the surgery's ring carry-through pass meters it.
///
/// The sliver `S` is bounded by the end curve `A` from one foot to the
/// other and the two rim pieces from the feet to the old vertex `V`. It
/// lies in the region
///
/// `Ω = { radius ≤ ‖p − center‖ ≤ reach } ∩ { (p − center)·toward ≥ floor }`:
///
/// - outside the disc of `radius` about `center`: on a round end the
///   band's section — a circle of the band's radius, or an ellipse
///   whose minor semi-axis it is, so it contains that disc — is tangent
///   to both rims and lies on the far side of `A` from `V` (in the
///   material the band keeps on the convex side, in the void it leaves
///   on the concave side), and `S` lies outside it; on a chord end
///   `radius` is zero and the clause is empty;
/// - within `reach` and above `floor`, because `‖p − center‖` is convex
///   and `(p − center)·toward` linear, so over the compact `S` the first
///   is largest, and the second smallest, somewhere on `S`'s boundary,
///   which is `A` and the two rim pieces; `reach` and `floor` are those
///   extremes over the three pieces, each in closed form over its own
///   window ([`piece_distance`], [`piece_along`]) — but for an ellipse
///   `A`'s term of `reach`, which is its semi-major axis, a bound on
///   the extreme rather than the extreme.
///
/// `toward` is the unit direction from `center` to `V`. Its choice is
/// free for soundness — every unit direction gives a sound `floor` —
/// and this one lays the half-plane's edge across the corner, so the
/// part of the disc on the far side of `center` from `V`, which the
/// cut-off does not touch, lies outside `Ω`.
///
/// A round `A` is not an edge of the source, so the plan describes it:
/// at a foot the rim and the section are tangent and `S` is the cusp
/// between them, so `A` leaves the foot in the direction the rim piece
/// leaves it towards `V` — the arc from that foot turning about
/// `(foot − center) × tangent` to the other foot. An ellipse is read as
/// the affine image of the unit circle in its own frame, where a linear
/// function keeps its form. A chord `A` is the segment between the
/// feet, and `center` its midpoint.
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
    /// `Ω`'s inner radius: the band's on a round end, zero on a chord.
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
    /// **The region a cut-off removes from its end face**, read off the
    /// source before any mutation. `rims` pairs each rim with its foot,
    /// `face_a`'s first. The end curves differ only in the curve's own
    /// term: a round one is read over its span from `face_a`'s foot, a
    /// chord — `center` its midpoint, no inner disc — at its two ends,
    /// where a segment's extremes are.
    ///
    /// # Errors
    ///
    /// [`BlendError::UnsupportedGeometry`] when a rim carries no
    /// certified line, circle or ellipse; [`BlendError::UnsupportedRunOut`] from
    /// a foot off its rim's span; [`BlendError::BodyNotIntact`] when the
    /// old vertex, a rim, or the end face's half of a rim does not
    /// resolve.
    fn of(
        body: &Body<T>,
        vertex: VertexKey,
        cap: FaceKey,
        rims: [(EdgeKey, Point3<T>); 2],
        curve: EndCurve<T>,
    ) -> Result<Self, BlendError> {
        let pv = point_of(body, vertex)
            .ok_or_else(|| not_intact(EntityId::Vertex(vertex), "a cut-off's old vertex"))?;
        let [(rim_a, foot_a), (rim_b, foot_b)] = rims;
        let (center, radius) = curve.round().unwrap_or((
            foot_a + (foot_b - foot_a) * T::from_f64(0.5),
            T::zero(),
        ));
        let toward = (pv - center).normalize();
        // The rims' pieces from their feet to the old vertex: each one's
        // far extent from `center` and least height along `toward`.
        let pieces = [
            rim_piece(body, rim_a, vertex, foot_a)?,
            rim_piece(body, rim_b, vertex, foot_b)?,
        ];
        let mut far = [T::zero(); 2];
        let mut low = far;
        for (i, ((carrier, window), _)) in pieces.iter().enumerate() {
            let unsupported = || {
                unbuilt_geometry(
                    EntityId::Edge(rims[i].0),
                    "an end face's rim is neither a line, a circle nor an ellipse, the only \
                     rims the sliver's extent is closed-form over",
                )
            };
            far[i] = piece_distance(carrier, *window, center)
                .ok_or_else(unsupported)?
                .1;
            low[i] = piece_along(carrier, *window, center, toward)
                .ok_or_else(unsupported)?
                .0;
        }
        let (curve_far, curve_low) = match curve {
            EndCurve::Chord => (
                (foot_a - center).norm().max((foot_b - center).norm()),
                (foot_a - center)
                    .dot(toward)
                    .min((foot_b - center).dot(toward)),
            ),
            // The arc, from `face_a`'s foot along the tangent its rim
            // piece leaves that foot by (the type's docs), to `face_b`'s:
            // its span read in `(0, τ]` past the start.
            EndCurve::Arc { .. } => {
                let from = foot_a - center;
                let arc = CircleFrame {
                    center,
                    axis: from.cross(pieces[0].1).normalize(),
                    radius,
                    u_ref: from.normalize(),
                };
                let (low_arc, _) =
                    arc.along((T::zero(), arc.past(T::zero(), foot_b)), center, toward);
                (radius, low_arc)
            }
            // The ellipse as the unit circle's image `center + major·cos t·u
            // + minor·sin t·w`, its frame turned as the arc's is: `toward`'s
            // height over it is the height of the circle point at `t` along
            // `major·(u·toward)·u + minor·(w·toward)·w`.
            EndCurve::Ellipse {
                major,
                minor,
                u_major,
                ..
            } => {
                let axis = (foot_a - center).cross(pieces[0].1).normalize();
                let w = axis.cross(u_major);
                let on_circle = |p: Point3<T>| {
                    let d = p - center;
                    center + u_major * (d.dot(u_major) / major) + w * (d.dot(w) / minor)
                };
                let unit = CircleFrame {
                    center,
                    axis,
                    radius: T::one(),
                    u_ref: u_major,
                };
                let from = on_circle(foot_a) - center;
                let start = from.dot(w).atan2(from.dot(u_major));
                let span = unit.past(start, on_circle(foot_b));
                let lifted = u_major * (major * u_major.dot(toward)) + w * (minor * w.dot(toward));
                let (low_arc, _) = unit.along((start, start + span), center, lifted);
                (major, low_arc)
            }
        };
        Ok(Self {
            cap,
            rims: [rim_a, rim_b],
            center,
            radius,
            reach: curve_far.max(far[0]).max(far[1]),
            toward,
            floor: curve_low.min(low[0]).min(low[1]),
        })
    }
}

/// **Where a foot lands on its rim**: the parameter the carve splits the
/// rim at, strictly inside the rim's stored span — or the one refusal
/// for a foot that is not ([`FOOT_INSIDE_A_FACE`]), read at the plan on
/// the source rim and again by the carve on the live one. Between the
/// two reads the only thing that changes a rim is another cut-off's
/// split from its other end, which [`shared_rims_clear`] meters first.
///
/// # Errors
///
/// [`BlendError::UnsupportedRunOut`] for a foot off its rim's span;
/// [`BlendError::UnsupportedGeometry`] / [`BlendError::BodyNotIntact`]
/// from the read ([`split_param_in_span`]).
fn foot_param<T: Decide + Bounds>(
    body: &Body<T>,
    rim: EdgeKey,
    vertex: VertexKey,
    foot: Point3<T>,
) -> Result<T, BlendError> {
    split_param_in_span(body, rim, foot)?
        .ok_or_else(|| unbuilt_run_out(EntityId::Vertex(vertex), FOOT_INSIDE_A_FACE))
}

/// The refusal for two cut-offs whose feet on one shared rim cross or
/// coincide.
pub(in crate::blend) const FEET_CROSS_ON_A_SHARED_RIM: &str = "two cut-offs' feet cross or \
     coincide on the rim they share, so the regions they take from the end faces meet";

/// **Two cut-offs on one rim, metered before any mutation.** A rim
/// joins two vertices and a band may be cut off at each — two bands
/// on one end face share its rim, and so do two bands whose end faces
/// are each other's supports. The first carve splits the rim at its
/// foot and the second splits the piece that is left, so the second
/// foot must lie on that piece: the two feet in order along the rim,
/// each nearer its own cut-off's vertex, apart by a margin
/// (`fillet3_cut_off_feet`, the span between them metered into meters
/// as `topo`'s edge split meters its interior test) decided definitely
/// positive. Feet that cross or coincide refuse typed here, naming the
/// shared rim; feet apart only within the band escalate.
///
/// # Errors
///
/// [`BlendError::UnsupportedRunOut`] ([`FEET_CROSS_ON_A_SHARED_RIM`],
/// or [`FOOT_INSIDE_A_FACE`] from a foot's read);
/// [`BlendError::Escalated`] for feet apart within the band;
/// [`BlendError::UnsupportedGeometry`] / [`BlendError::BodyNotIntact`]
/// from a rim's read.
pub(in crate::blend) fn shared_rims_clear<'e, T: Decide + Bounds + 'e>(
    body: &Body<T>,
    ends: impl IntoIterator<Item = &'e EndCut<T>>,
    band: Band,
) -> Result<(), BlendError> {
    let mut feet: Vec<(EdgeKey, VertexKey, Point3<T>)> = ends
        .into_iter()
        .flat_map(|e| {
            let [a, b] = e.sliver.rims;
            [(a, e.vertex, e.foot_a), (b, e.vertex, e.foot_b)]
        })
        .collect();
    feet.sort_by_key(|&(rim, v, _)| (rim, v));
    for pair in feet.windows(2) {
        let [(rim, v0, f0), (r1, v1, f1)] = [pair[0], pair[1]];
        if rim != r1 {
            continue;
        }
        let refuse = || unbuilt_run_out(EntityId::Edge(rim), FEET_CROSS_ON_A_SHARED_RIM);
        let (t0, t1) = (
            foot_param(body, rim, v0, f0)?,
            foot_param(body, rim, v1, f1)?,
        );
        let Some((carrier, _)) = stored_piece(body, rim)? else {
            return Err(unbuilt_geometry(
                EntityId::Edge(rim),
                "an end face's rim carries no certified carrier",
            ));
        };
        let rate = match *carrier {
            Curve3::Line { .. } => InfSpeed::new(T::one()),
            Curve3::Circle { radius, .. } => InfSpeed::new(radius),
            // An ellipse's speed is never below its minor semi-axis.
            Curve3::Ellipse { major, minor, .. } => InfSpeed::new(major.min(minor)),
            Curve3::Spiric { .. } | Curve3::Nurbs(_) => {
                return Err(unbuilt_geometry(
                    EntityId::Edge(rim),
                    "a rim two cut-offs share is neither a line, a circle nor an ellipse",
                ));
            }
        };
        let he_plus = body
            .get_edge(rim)
            .ok_or_else(|| not_intact(EntityId::Edge(rim), "a shared rim"))?
            .he_plus;
        // The stored window runs along `he_plus`, so the foot of the
        // cut-off at its start must come first.
        let span = if body.get_half_edge(he_plus).map(|h| h.start) == Some(v0) {
            t1 - t0
        } else {
            t0 - t1
        };
        let margin = Margin::metered(span, rate);
        match classify(
            BlendSite::Link { edge: rim },
            BlendDecision::CutOffFeet,
            margin,
            band,
        )? {
            Sign::Positive => {}
            Sign::Zero | Sign::Negative => return Err(refuse()),
        }
    }
    Ok(())
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
/// carrier; [`BlendError::UnsupportedRunOut`] from the foot's parameter
/// ([`foot_param`]); [`BlendError::BodyNotIntact`] when the rim does not
/// resolve or does not end at `vertex`.
fn rim_piece<T: Decide + Bounds>(
    body: &Body<T>,
    rim: EdgeKey,
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
    let t = foot_param(body, rim, vertex, foot)?;
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
    vertex: VertexKey,
    support: FaceKey,
    foot: Point3<T>,
    rec: &mut BlendNaming,
    tol: Tol,
) -> Result<SplitFragments, BlendError> {
    let t = foot_param(body, rim, vertex, foot)?;
    let frag = split_fragment(body, rim, vertex, t, None, rec, "end rim split", tol)?;
    rec.feet.push((frag.vertex, vertex, support));
    retire_fragment(rec, frag.near, frag.source);
    Ok(frag)
}

/// One new edge awaiting its description, as
/// [`Described`](crate::blend::surgery::Described) holds it.
pub(in crate::blend) type DescribedEdge<T> = (EdgeKey, ContactCarrier<T>, EdgeKey);

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
/// [`BlendError::UnsupportedRunOut`] for a foot off its live rim's span
/// ([`foot_param`]), which the plan's reads and [`shared_rims_clear`]
/// leave no route to; [`BlendError::UnsupportedGeometry`] from a rim's
/// read; [`BlendError::BodyNotIntact`] where a cycle read disagrees with
/// the plan.
pub(in crate::blend) fn cut_off<T: Decide + Bounds + topo::AtRestPolicy>(
    body: &mut Body<T>,
    end: &EndCut<T>,
    crease: EdgeKey,
    (face_a, face_b): (FaceKey, FaceKey),
    rec: &mut BlendNaming,
    tol: Tol,
) -> Result<(CutRims, DescribedEdge<T>), BlendError> {
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
    let a = split_rim(body, rim_a, v, face_a, end.foot_a, rec, tol)?;
    let b = split_rim(body, rim_b, v, face_b, end.foot_b, rec, tol)?;
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
