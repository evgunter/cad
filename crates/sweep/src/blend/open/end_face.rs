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
    ContactCarrier, Piece, SourceFaces, SplitFragments, chord_site, face_of_half, halves_of,
    not_intact, op, piece_along, piece_distance, point_of, retire_fragment, split_fragment,
    split_param_in_span, stored_piece, unbuilt_geometry, unbuilt_run_out,
};
use crate::blend::{BlendDecision, BlendError, BlendSite, classify};

/// The refusal for a cut-off whose foot does not land inside its rim's
/// span: the trimline runs past the end face's rim into the support,
/// which is a band running into a wall or a step.
pub(in crate::blend) const FOOT_INSIDE_A_FACE: &str = "a straight band's trimline meets the end \
     face's plane past its rim, so its foot lands inside a face rather than on that rim";

/// The band's section by the end face, between the two feet.
#[derive(Clone)]
pub(in crate::blend) enum EndCurve<T: Real> {
    /// A plane band's: the straight chord.
    Chord,
    /// A cylinder band's: the circle or ellipse the end face cuts from
    /// it ([`EndSection`]), about the spine's crossing of the end
    /// plane, its axis turned so the arc from `face_a`'s foot to
    /// `face_b`'s runs forward, the short way.
    Round(Curve3<T>),
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
    /// through `q_a` and `q_b` along `along`; `section` is what the
    /// battery read the band's section to be, and `spine` a cylinder
    /// band's spine point (still to be carried to the end face) and
    /// radius.
    ///
    /// Every point is a stored trimline or spine point carried along
    /// `along` to the stored end plane; nothing is sampled. `along` is
    /// transverse to the plane by the battery's classification of this
    /// end (independent face normals at the vertex, read by
    /// `fillet3_corner_independence` on a planar band's corner and on a
    /// ruled band's tilted cap, or a cap perpendicular to a cylinder
    /// band's spine), so the quotient is total.
    ///
    /// # Errors
    ///
    /// [`BlendError::UnsupportedRunOut`] when a foot does not land
    /// inside its rim's span ([`FOOT_INSIDE_A_FACE`]);
    /// [`BlendError::UnsupportedGeometry`] when a rim carries no
    /// certified line, circle or ellipse, or a round section has no
    /// cylinder band;
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
        section: EndSection<T>,
        spine: Option<(Point3<T>, T)>,
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
        let carry = |p: Point3<T>| p + along * ((*po - p).dot(*n) / along.dot(*n));
        let (foot_a, foot_b) = (carry(q_a), carry(q_b));
        // The feet must land on the rims, strictly inside their spans:
        // anywhere else the band runs into a face the cut-off does not
        // touch.
        for (rim, foot) in [(rim_a, foot_a), (rim_b, foot_b)] {
            foot_param(body, rim, vertex, foot)?;
        }
        // A round section is placed about the spine's crossing, and run
        // forward from `face_a`'s foot: the arc between the feet is under
        // half a turn, so its sense is the turn from one foot to the other.
        let round = |placed: Option<Curve3<T>>, center: Point3<T>| {
            let unbuilt = || unbuilt_geometry(EntityId::Vertex(vertex), "a cut-off's section");
            let placed = placed.ok_or_else(unbuilt)?;
            let turn = (foot_a - center).cross(foot_b - center);
            let backward = match placed {
                Curve3::Circle { axis, .. } | Curve3::Ellipse { axis, .. } => {
                    turn.dot(axis).hi() < 0.0
                }
                _ => return Err(unbuilt()),
            };
            Ok(EndCurve::Round(if backward {
                placed.reversed().ok_or_else(unbuilt)?
            } else {
                placed
            }))
        };
        let curve = match (section, spine) {
            (EndSection::Chord, _) => EndCurve::Chord,
            (EndSection::Circle, Some((spine, radius))) => {
                let center = carry(spine);
                let circle = Curve3::Circle {
                    center,
                    axis: n.normalize(),
                    radius,
                    u_ref: (foot_a - center).normalize(),
                };
                round(Some(circle), center)?
            }
            (EndSection::Ellipse(ellipse), Some((spine, _))) => {
                let center = carry(spine);
                let origin = Point3::new(T::zero(), T::zero(), T::zero());
                round(ellipse.translated(center - origin), center)?
            }
            (_, None) => {
                return Err(unbuilt_geometry(
                    EntityId::Vertex(vertex),
                    "a round cut-off's band is not a cylinder about its spine",
                ));
            }
        };
        let sliver = CapSliver::of(
            body,
            vertex,
            face,
            [(rim_a, foot_a), (rim_b, foot_b)],
            &curve,
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

    /// Each foot with the rim it lands on and the old vertex, as
    /// [`shared_rims_clear`] reads them.
    pub(in crate::blend) fn feet(&self) -> [(EdgeKey, VertexKey, Point3<T>); 2] {
        let [a, b] = self.sliver.rims;
        [(a, self.vertex, self.foot_a), (b, self.vertex, self.foot_b)]
    }

    /// The end curve's carrier, as the description pass reads it.
    fn carrier(&self) -> ContactCarrier<T> {
        match &self.curve {
            EndCurve::Chord => ContactCarrier::Chord,
            EndCurve::Round(curve) => ContactCarrier::Transverse(curve.clone()),
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
/// `Ω = { ‖p − center‖ ≤ reach } ∖ E ∩ ⋂ₖ { (p − center)·dₖ ≥ floorₖ }`:
///
/// - outside `E`, the inside of the band's section: on a round end the
///   section — a circle of the band's radius, or an ellipse whose minor
///   semi-axis it is — is convex, tangent to both rims, and lies on the
///   far side of `A` from `V` (in the material the band keeps on the
///   convex side, in the void it leaves on the concave side), and `S`
///   lies outside it; on a chord end `E` is empty;
/// - within `reach` and above every `floorₖ`, because `‖p − center‖` is
///   convex and `(p − center)·dₖ` linear, so over the compact `S` the
///   first is largest, and each second smallest, somewhere on `S`'s
///   boundary, which is `A` and the two rim pieces; `reach` and the
///   floors are those extremes over the three pieces, each in closed
///   form over its own window ([`piece_distance`], [`piece_along`]; an
///   ellipse's distance from its own centre is extreme at its window's
///   ends or at a major vertex).
///
/// The directions `dₖ` are free for soundness — every unit direction
/// gives a sound floor. They are `toward`, the unit direction from
/// `center` to `V`, which lays a half-plane's edge across the corner so
/// the part of the section on the far side of `center` from `V` lies
/// outside `Ω`; and on a round end both senses of the section's two
/// axes, which hold `Ω` to the box the sliver spans in the section's
/// own frame however far a tilted section's major axis reaches.
///
/// A round `A` is the section's arc from one foot to the other, run
/// forward ([`EndCurve::Round`]); a chord `A` is the segment between
/// the feet, and `center` its midpoint.
///
/// An edge that misses `Ω` misses `S`; the converse does not hold, and
/// that is the meter's conservative direction.
///
/// Two of its terms are pinned by no assembly row, only by the piece
/// meters' unit row: the arc's term of a floor, which binds only when a
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
    /// `E`, on a round end: the section's frame, and its semi-axes.
    inside: Option<SectionFrame<T>>,
    /// `Ω`'s outer radius.
    reach: T,
    /// Each direction `dₖ`, with the least `(p − center)·dₖ` over the
    /// sliver.
    floors: Vec<(Vec3<T>, T)>,
}

/// A round section's frame in the end plane: `p − center` is inside
/// it when `(d·u/major)² + (d·w/minor)² < 1`.
#[derive(Clone, Copy)]
struct SectionFrame<T: Real> {
    u: Vec3<T>,
    w: Vec3<T>,
    major: T,
    minor: T,
}

impl<T: Bounds> SectionFrame<T> {
    /// The section's scaled radius at `d`: below one inside it.
    fn scaled(self, d: Vec3<T>) -> T {
        let (x, y) = (d.dot(self.u) / self.major, d.dot(self.w) / self.minor);
        (x.powi(2) + y.powi(2)).sqrt()
    }

    /// **How far inside the section an edge stays**, in meters: positive
    /// when every point of `carrier` over `window` lies inside it. The
    /// scaled radius is convex in the point, so a segment's largest is
    /// at an end; a circle or ellipse of semi-axis at most `a` about
    /// `c` stays within `a/minor` of `c`'s, the scaling stretching no
    /// length by more than `1/minor`. A scaled radius `ρ < 1` is at
    /// least `minor·(1 − ρ)` from the section, which shrinks no length
    /// by less than `minor`. `None` for a carrier with no closed form.
    fn depth(self, carrier: &Curve3<T>, (ta, tb): (T, T), center: Point3<T>) -> Option<T> {
        let rho = match *carrier {
            Curve3::Line { origin, dir } => self
                .scaled(origin + dir * ta - center)
                .max(self.scaled(origin + dir * tb - center)),
            Curve3::Circle {
                center: c, radius, ..
            } => self.scaled(c - center) + radius / self.minor,
            Curve3::Ellipse {
                center: c, major, ..
            } => self.scaled(c - center) + major / self.minor,
            Curve3::Spiric { .. } | Curve3::Nurbs(_) => return None,
        };
        Some(self.minor * (T::one() - rho))
    }
}

impl<T: Bounds> CapSliver<T> {
    /// **How clear one end-face edge is of the sliver**: the `carrier`
    /// over `window` misses `Ω` when this is positive. `None` for a
    /// carrier with no closed form ([`piece_distance`]).
    ///
    /// A curved edge is read term by term ([`Self::whole_clearance`]),
    /// so one that misses `Ω` only by leaving it through different
    /// faces at different points reads not-clear — a conservative
    /// refusal, never a silent pass. A straight edge is also read point
    /// by point ([`Self::line_clearance`]) and the larger reading kept:
    /// the whole-edge one still carries an elliptic section's own depth,
    /// which the pointwise read replaces by the disc of its minor
    /// semi-axis.
    pub(in crate::blend) fn clearance(&self, carrier: &Curve3<T>, window: (T, T)) -> Option<T> {
        let whole = self.whole_clearance(carrier, window)?;
        Some(match *carrier {
            Curve3::Line { origin, dir } => whole.max(self.line_clearance(origin, dir, window)),
            _ => whole,
        })
    }

    /// The largest of how far the whole edge stays inside the section
    /// (or the disc of its minor semi-axis), beyond `reach`, and short
    /// of each floor: each term clears the whole edge on its own.
    fn whole_clearance(&self, carrier: &Curve3<T>, window: (T, T)) -> Option<T> {
        let (near, far) = piece_distance(carrier, window, self.center)?;
        let mut clear = near - self.reach;
        if let Some(section) = self.inside {
            // The disc of the minor semi-axis lies inside the section,
            // and `far` is exact where the section's own bound is not.
            clear =
                clear
                    .max(section.minor - far)
                    .max(section.depth(carrier, window, self.center)?);
        }
        for &(d, floor) in &self.floors {
            let (_, high) = piece_along(carrier, window, self.center, d)?;
            clear = clear.max(floor - high);
        }
        Some(clear)
    }

    /// **How clear a straight edge is of `Ω`, point by point**: the
    /// least over the segment of `G(p)`, the largest of `‖q‖ − reach`,
    /// `minor − ‖q‖` on a round end, and each `floorₖ − q·dₖ`, with
    /// `q = p − center`. Each term bounds from below `p`'s distance
    /// from a set containing `Ω` (the disc to `reach`, the outside of
    /// the disc of the minor semi-axis, a half-plane), so `G > 0`
    /// puts `p` outside `Ω`.
    ///
    /// In arc length `s` each term is affine, convex (`‖q‖ − reach`) or
    /// concave (`minor − ‖q‖`), so `G`'s least is at an end, at the
    /// foot of `center` on the line, or where two terms cross, each
    /// crossing closed-form. A candidate off any crossing is still a
    /// point of the segment, where `G` is no lower than its least.
    ///
    /// Against the sliver `S` itself only the ends carry soundness: a
    /// line meets the end curve at most twice, so a segment that meets
    /// `S` has an end in it. The rest of the candidates buy carves, and
    /// only a unit test against a sampled least can see one go missing.
    fn line_clearance(&self, origin: Point3<T>, dir: Vec3<T>, (ta, tb): (T, T)) -> T {
        let zero = T::zero();
        let one = T::one();
        let pa = origin + dir * ta;
        let chord = origin + dir * tb - pa;
        // An edge has positive length, so `len` is no divisor of zero.
        let len = chord.norm();
        let e = chord * (one / len);
        let q0 = pa - self.center;
        // `‖q‖² = (s + b)² + d2`, `d2` read off the foot's own offset
        // as [`piece_distance`] reads it, which does not cancel far
        // from `center` as `q0·q0 − b²` does.
        let b = q0.dot(e);
        let d2 = (q0 - e * b).norm().powi(2);
        let minor = self.inside.map(|s| s.minor);
        // Each floor as `a + k·s`.
        let affine = || {
            self.floors
                .iter()
                .map(move |&(d, floor)| (floor - q0.dot(d), zero - e.dot(d)))
        };
        let g = |s: T| {
            let s = s.max(zero).min(len);
            let rho = (q0 + e * s).norm();
            let v = minor.map_or(rho - self.reach, |m| (rho - self.reach).max(m - rho));
            affine().fold(v, |v, (a, k)| v.max(a + k * s))
        };
        // A quotient whose divisor's bracket meets zero is skipped: at
        // `f64` that is a divisor that vanishes (below), and at `Interval`
        // the candidate would spread over the segment and read `G` over
        // all of it. A skipped candidate can only raise the reading, and
        // never past the ends.
        let over = |x: T, y: T| (y.lo() > 0.0 || y.hi() < 0.0).then(|| x / y);
        // Both ends, though a clamped crossing would reach either: they
        // are what soundness against `S` reads, so no skipped quotient
        // may lose one.
        let mut least = g(zero).min(g(len)).min(g(zero - b));
        let mut at = |s: Option<T>| {
            if let Some(s) = s {
                least = least.min(g(s));
            }
        };
        // `‖q‖ = a + k·s`, `|k| ≤ 1`: with `τ = s + b` and `a′ = a − k·b`,
        // `(1 − k²)·τ² − 2·a′·k·τ + d2 − a′² = 0`, whose roots are
        // `(d2 − a′²)/(a′·k ∓ r)`, both signs taken where `topo`'s
        // `solid_contain::quadratic_roots` picks one. It never divides by
        // `1 − k²`, which vanishes on an edge along a floor's direction
        // and leaves the equation linear with the one root it takes. A
        // divisor vanishes alone only where `a′² = d2`: the other root
        // is then the foot, and `G` runs monotonically through the root
        // lost, since between the two crossings the larger term is the
        // one heading away from the foot's value.
        let mut radial = |a: T, k: T| {
            let a1 = a - k * b;
            let r = (a1.powi(2) - (one - k.powi(2)) * d2).max(zero).sqrt();
            for r in [r, zero - r] {
                at(over(d2 - a1.powi(2), a1 * k - r).map(|t| t - b));
            }
        };
        for (a, k) in affine() {
            radial(self.reach + a, k);
            if let Some(m) = minor {
                radial(m - a, zero - k);
            }
        }
        if let Some(m) = minor {
            let h = ((m + self.reach) / T::from_f64(2.0)).powi(2) - d2;
            let h = h.max(zero).sqrt();
            at(Some(h - b));
            at(Some(zero - h - b));
        }
        for (i, (ai, ki)) in affine().enumerate() {
            for (aj, kj) in affine().skip(i + 1) {
                // The two differ by `δ0` at the start and `δ1` at the end,
                // so where they cross inside the segment it is at
                // `len·|δ0|/(|δ0| + |δ1|)`.
                let d0 = (ai - aj).abs();
                let d1 = (ai - aj + (ki - kj) * len).abs();
                at(over(len * d0, d0 + d1));
            }
        }
        least
    }
}

impl<T: Decide + Bounds> CapSliver<T> {
    /// **The region a cut-off removes from its end face**, read off the
    /// source before any mutation. `rims` pairs each rim with its foot,
    /// `face_a`'s first. The end curves differ only in the curve's own
    /// terms: a round one is read over its window from `face_a`'s foot
    /// to `face_b`'s, a chord — `center` its midpoint, no section
    /// inside — at its two ends, where a segment's extremes are.
    ///
    /// # Errors
    ///
    /// [`BlendError::UnsupportedGeometry`] when a rim carries no
    /// certified line, circle or ellipse; [`BlendError::UnsupportedRunOut`] from
    /// a foot off its rim's span; [`BlendError::BodyNotIntact`] when the
    /// old vertex, a rim, or the end face's half of a rim does not
    /// resolve, or a round end curve does not run forward from one foot
    /// to the other.
    fn of(
        body: &Body<T>,
        vertex: VertexKey,
        cap: FaceKey,
        rims: [(EdgeKey, Point3<T>); 2],
        curve: &EndCurve<T>,
    ) -> Result<Self, BlendError> {
        let pv = point_of(body, vertex)
            .ok_or_else(|| not_intact(EntityId::Vertex(vertex), "a cut-off's old vertex"))?;
        let [(rim_a, foot_a), (rim_b, foot_b)] = rims;
        // The rims' pieces from their feet to the old vertex.
        let pieces = [
            rim_piece(body, rim_a, vertex, foot_a)?,
            rim_piece(body, rim_b, vertex, foot_b)?,
        ];
        // The end curve's own piece, its centre, and on a round end the
        // section's frame and its far extent from that centre.
        let (center, round) = match curve {
            EndCurve::Chord => (foot_a + (foot_b - foot_a) * T::from_f64(0.5), None),
            EndCurve::Round(arc) => {
                let (center, axis, major, minor, u) = match *arc {
                    Curve3::Circle {
                        center,
                        axis,
                        radius,
                        u_ref,
                    } => (center, axis, radius, radius, u_ref),
                    Curve3::Ellipse {
                        center,
                        axis,
                        major,
                        minor,
                        u_ref,
                    } => (center, axis, major, minor, u_ref),
                    Curve3::Line { .. } | Curve3::Spiric { .. } | Curve3::Nurbs(_) => {
                        return Err(unbuilt_geometry(
                            EntityId::Vertex(vertex),
                            "a cut-off's round end curve is neither a circle nor an ellipse",
                        ));
                    }
                };
                let backward = || {
                    not_intact(
                        EntityId::Vertex(vertex),
                        "a cut-off's round end curve does not run forward from one foot to \
                         the other",
                    )
                };
                let ta = arc.param_near(foot_a, T::zero()).ok_or_else(backward)?;
                let tb = arc.param_near(foot_b, ta).ok_or_else(backward)?;
                if (tb - ta).lo() <= 0.0 {
                    return Err(backward());
                }
                let frame = SectionFrame {
                    u,
                    w: axis.cross(u),
                    major,
                    minor,
                };
                // The distance from the centre at `t` is
                // `√(major²·cos²t + minor²·sin²t)`, largest at a window
                // end or at a major vertex `t = kπ` inside the window.
                let at = |p: Point3<T>| (p - center).norm();
                let first = T::zero() - (T::zero() - ta / T::pi()).floor();
                let vertex_inside = (first * T::pi() - tb).lo() <= 0.0;
                let far = if vertex_inside {
                    major
                } else {
                    at(foot_a).max(at(foot_b))
                };
                (center, Some((arc, (ta, tb), frame, far)))
            }
        };
        let toward = (pv - center).normalize();
        let mut directions = vec![toward];
        if let Some((_, _, frame, _)) = round {
            directions.extend([frame.u, -frame.u, frame.w, -frame.w]);
        }
        let mut reach = match round {
            Some((_, _, _, far)) => far,
            None => (foot_a - center).norm().max((foot_b - center).norm()),
        };
        let mut floors = Vec::with_capacity(directions.len());
        for d in directions {
            let low = match round {
                Some((arc, window, _, _)) => {
                    piece_along(arc, window, center, d)
                        .ok_or_else(|| {
                            unbuilt_geometry(EntityId::Vertex(vertex), "a cut-off's round section")
                        })?
                        .0
                }
                None => (foot_a - center).dot(d).min((foot_b - center).dot(d)),
            };
            floors.push((d, low));
        }
        for (i, (carrier, window)) in pieces.iter().enumerate() {
            let unsupported = || {
                unbuilt_geometry(
                    EntityId::Edge(rims[i].0),
                    "an end face's rim is neither a line, a circle nor an ellipse, the only \
                     rims the sliver's extent is closed-form over",
                )
            };
            reach = reach.max(
                piece_distance(carrier, *window, center)
                    .ok_or_else(unsupported)?
                    .1,
            );
            for (d, floor) in &mut floors {
                let (low, _) = piece_along(carrier, *window, center, *d).ok_or_else(unsupported)?;
                *floor = floor.min(low);
            }
        }
        Ok(Self {
            cap,
            rims: [rim_a, rim_b],
            center,
            inside: round.map(|(_, _, frame, _)| frame),
            reach,
            floors,
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
pub(in crate::blend) fn foot_param<T: Decide + Bounds>(
    body: &Body<T>,
    rim: EdgeKey,
    vertex: VertexKey,
    foot: Point3<T>,
) -> Result<T, BlendError> {
    split_param_in_span(body, rim, foot)?
        .ok_or_else(|| unbuilt_run_out(EntityId::Vertex(vertex), FOOT_INSIDE_A_FACE))
}

/// The refusal for two band ends — cut-offs, or a cut-off and a turn,
/// or two turns — whose feet on one shared rim cross or coincide.
pub(in crate::blend) const FEET_CROSS_ON_A_SHARED_RIM: &str = "two band ends' feet cross or \
     coincide on the rim they share, so the regions they take from the faces beside it meet";

/// **Two splits on one rim, metered before any mutation.** A rim
/// joins two vertices and a band may be cut off at each — two bands
/// on one end face share its rim, and so do two bands whose end faces
/// are each other's supports — and a turn splits its third edge at its
/// foot, which may be a cut-off's rim at its other end or another
/// turn's third edge. `feet` lists every such split as `(rim, the
/// vertex the split is taken beside, the foot)`. The first carve splits
/// the rim at its foot and the second splits the piece that is left, so
/// the second foot must lie on that piece: the two feet in order along
/// the rim,
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
pub(in crate::blend) fn shared_rims_clear<T: Decide + Bounds>(
    body: &Body<T>,
    feet: impl IntoIterator<Item = (EdgeKey, VertexKey, Point3<T>)>,
    band: Band,
) -> Result<(), BlendError> {
    let mut feet: Vec<(EdgeKey, VertexKey, Point3<T>)> = feet.into_iter().collect();
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
/// carrier and the window of the piece on it. The stored window runs
/// along `he_plus` (`topo`'s edge-direction invariant), so the vertex
/// sits at the window's start when `he_plus` starts there and at its
/// end otherwise; the foot's parameter is the one the carve splits the
/// rim at.
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
) -> Result<Piece<'_, T>, BlendError> {
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
    if body.get_half_edge(he_plus).map(|h| h.start) == Some(vertex) {
        Ok((carrier, (t0, t)))
    } else if body.half_edge_end(he_plus) == Some(vertex) {
        Ok((carrier, (t, t1)))
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

#[cfg(test)]
mod line_meter_fuzz;

#[cfg(test)]
mod tests {
    use super::{CapSliver, SectionFrame};
    use geom::Curve3;
    use geom_core::{Bounds, Interval, Point3, Real, Vec3};
    use topo::{EdgeKey, FaceKey};

    /// A sliver enclosure about the origin in the `z = 0` plane, spelled
    /// in plain numbers so one region builds at every scalar.
    pub(super) struct Region {
        /// The section's major axis's angle, its major and minor
        /// semi-axes; `None` on a chord end.
        pub(super) section: Option<(f64, f64, f64)>,
        pub(super) reach: f64,
        /// Each unit direction `dₖ` and its floor.
        pub(super) floors: Vec<((f64, f64), f64)>,
    }

    impl Region {
        pub(super) fn sliver<T: Real>(&self) -> CapSliver<T> {
            let f = T::from_f64;
            let v = |x: f64, y: f64| Vec3::new(f(x), f(y), f(0.0));
            CapSliver {
                cap: FaceKey::default(),
                rims: [EdgeKey::default(); 2],
                center: Point3::new(f(0.0), f(0.0), f(0.0)),
                inside: self.section.map(|(a, major, minor)| SectionFrame {
                    u: v(a.cos(), a.sin()),
                    w: v(-a.sin(), a.cos()),
                    major: f(major),
                    minor: f(minor),
                }),
                reach: f(self.reach),
                floors: self
                    .floors
                    .iter()
                    .map(|&((x, y), floor)| (v(x, y), f(floor)))
                    .collect(),
            }
        }

        /// The straight-edge meter on the segment `a → b`, at `f64` and at
        /// `Interval`.
        pub(super) fn meter(&self, a: (f64, f64), b: (f64, f64)) -> (f64, Interval) {
            let at = |x: f64, y: f64| Point3::new(x, y, 0.0);
            let dir = Vec3::new(b.0 - a.0, b.1 - a.1, 0.0);
            let iv = Interval::from_f64;
            (
                self.sliver::<f64>()
                    .line_clearance(at(a.0, a.1), dir, (0.0, 1.0)),
                self.sliver::<Interval>().line_clearance(
                    Point3::new(iv(a.0), iv(a.1), iv(0.0)),
                    Vec3::new(iv(dir.x), iv(dir.y), iv(0.0)),
                    (iv(0.0), iv(1.0)),
                ),
            )
        }

        /// `G` at `p`, from its definition.
        pub(super) fn g(&self, p: (f64, f64)) -> f64 {
            let rho = p.0.hypot(p.1);
            let v = self.section.map_or(rho - self.reach, |(_, _, minor)| {
                (rho - self.reach).max(minor - rho)
            });
            self.floors
                .iter()
                .fold(v, |v, &((x, y), floor)| v.max(floor - p.0 * x - p.1 * y))
        }

        /// Whether `p` is in `Ω`: within `reach`, outside the section
        /// itself (an ellipse, not its minor disc), and above every floor.
        fn holds(&self, p: (f64, f64)) -> bool {
            let outside = self.section.is_none_or(|(a, major, minor)| {
                let (x, y) = (p.0 * a.cos() + p.1 * a.sin(), p.1 * a.cos() - p.0 * a.sin());
                (x / major).powi(2) + (y / minor).powi(2) >= 1.0
            });
            p.0.hypot(p.1) <= self.reach
                && outside
                && self
                    .floors
                    .iter()
                    .all(|&((x, y), floor)| p.0 * x + p.1 * y >= floor)
        }
    }

    /// The distance from `p` to the segment `a → b`.
    fn to_segment(p: (f64, f64), a: (f64, f64), b: (f64, f64)) -> f64 {
        let (dx, dy) = (b.0 - a.0, b.1 - a.1);
        let t = (((p.0 - a.0) * dx + (p.1 - a.1) * dy) / (dx.powi(2) + dy.powi(2))).clamp(0.0, 1.0);
        (p.0 - a.0 - t * dx).hypot(p.1 - a.1 - t * dy)
    }

    /// **The straight-edge meter never reads a segment farther from `Ω`
    /// than it is**, on every segment between two points of a lattice
    /// over and around four regions, at `f64` and at `Interval`: below
    /// the segment's distance to each point of a grid over `Ω`, and not
    /// positive where a sample of the segment is in `Ω`. `Ω` is read off
    /// its own definition, with an elliptic section, so this goes red
    /// if a term of `G` stops bounding the distance to `Ω`.
    ///
    /// It is also the segment's least `G` (the candidate set is
    /// complete): never above a dense run of samples, and never more
    /// than their spacing below their least, `G` being 1-Lipschitz along
    /// the segment. The lattice holds segments along the box's floor
    /// directions, whose crossings with a radial term have a vanishing
    /// leading coefficient and at `Interval` one that straddles zero, and
    /// through the centre; one region repeats a direction, so two affine
    /// terms coincide.
    #[test]
    fn the_line_meter_is_the_least_of_g_and_never_reads_past_omega() {
        let (c, s) = (40f64.to_radians().cos(), 40f64.to_radians().sin());
        let box_floors = vec![
            ((c, s), 0.85),
            ((1.0, 0.0), 0.5),
            ((-1.0, 0.0), -1.3),
            ((0.0, 1.0), 0.1),
            ((0.0, -1.0), -1.25),
        ];
        let regions = [
            (
                "round end",
                Region {
                    section: Some((0.0, 1.0, 1.0)),
                    reach: 1.3,
                    floors: box_floors.clone(),
                },
            ),
            (
                "tilted elliptic end",
                Region {
                    section: Some((0.3, 1.4, 0.8)),
                    reach: 1.5,
                    floors: vec![((c, s), 0.2), ((0.0, -1.0), -1.4)],
                },
            ),
            (
                "chord end",
                Region {
                    section: None,
                    reach: 1.3,
                    floors: vec![((c, s), 0.4)],
                },
            ),
            (
                "a repeated direction",
                Region {
                    section: Some((0.0, 1.0, 1.0)),
                    reach: 1.3,
                    floors: vec![((1.0, 0.0), 0.5), ((1.0, 0.0), 0.5), ((0.0, 1.0), 0.1)],
                },
            ),
        ];
        let lattice: Vec<(f64, f64)> = (0..7)
            .flat_map(|i| (0..7).map(move |j| (i, j)))
            .map(|(i, j)| (-2.0 + f64::from(i) / 1.5, -2.0 + f64::from(j) / 1.5))
            .collect();
        const N: u32 = 400;
        for (what, region) in &regions {
            let omega: Vec<(f64, f64)> = (0..=80)
                .flat_map(|i| (0..=80).map(move |j| (i, j)))
                .map(|(i, j)| (-1.6 + f64::from(i) / 25.0, -1.6 + f64::from(j) / 25.0))
                .filter(|&p| region.holds(p))
                .collect();
            assert!(omega.len() > 100, "{what}: the grid meets Ω");
            let (mut handed_over, mut through) = (0usize, 0usize);
            for (k, &a) in lattice.iter().enumerate() {
                for &b in &lattice[k + 1..] {
                    let len = (b.0 - a.0).hypot(b.1 - a.1);
                    let (m, iv) = region.meter(a, b);
                    let near = omega
                        .iter()
                        .map(|&p| to_segment(p, a, b))
                        .fold(f64::INFINITY, f64::min);
                    let samples = (0..=N).map(|n| {
                        let t = f64::from(n) / f64::from(N);
                        (a.0 + (b.0 - a.0) * t, a.1 + (b.1 - a.1) * t)
                    });
                    let meets = samples.clone().any(|p| region.holds(p));
                    through += usize::from(meets);
                    let least = samples.map(|p| region.g(p)).fold(f64::INFINITY, f64::min);
                    for (scalar, read) in [("f64", m), ("Interval", iv.lo())] {
                        assert!(
                            read <= near + 1e-12 && !(meets && read > 0.0),
                            "{what}, {scalar}: {a:?} → {b:?} reads {read}, but is {near} from \
                             a point of Ω (meets it: {meets})"
                        );
                        assert!(
                            read <= least + 1e-12,
                            "{what}, {scalar}: {a:?} → {b:?}: {read} is above a sampled G {least}"
                        );
                        assert!(
                            read >= least - len / f64::from(2 * N) - 1e-12,
                            "{what}, {scalar}: {a:?} → {b:?}: {read} is below the least G {least}"
                        );
                    }
                    let carrier = Curve3::Line {
                        origin: Point3::new(a.0, a.1, 0.0),
                        dir: Vec3::new(b.0 - a.0, b.1 - a.1, 0.0),
                    };
                    let whole = region.sliver::<f64>().whole_clearance(&carrier, (0.0, 1.0));
                    handed_over += usize::from(m > 1e-3 && whole.is_some_and(|w| w <= 0.0));
                }
            }
            assert!(through > 0, "{what}: some segment runs through Ω");
            if *what == "round end" {
                assert!(
                    handed_over > 0,
                    "{what}: some segment the meter clears is cleared by no one term over its \
                     whole length"
                );
            }
        }
    }

    /// **Far from the centre the meter keeps its digits**: a segment
    /// `10⁴` from `center` at its start, passing `0.9` from it, between
    /// the disc of the minor semi-axis and `reach`. `G` there is least
    /// where the two radial terms cross, at `‖q‖ = 1.15`, and that least
    /// is `−0.15` exactly; the crossing is placed from the line's
    /// distance to `center`, which a spelling that squares `‖q‖` and
    /// subtracts reads `~10⁻⁸` off.
    #[test]
    fn a_segment_far_from_the_centre_reads_its_least_to_rounding() {
        let region = Region {
            section: Some((0.0, 1.0, 1.0)),
            reach: 1.3,
            floors: Vec::new(),
        };
        let t = 0.37f64;
        let (u, n) = ((t.cos(), t.sin()), (-t.sin(), t.cos()));
        let at = |s: f64| (0.9 * n.0 + s * u.0, 0.9 * n.1 + s * u.1);
        let (a, b) = (at(-1.0e4), at(1.0e4));
        let (m, iv) = region.meter(a, b);
        assert!((m + 0.15).abs() < 1e-11, "f64 reads {m}, not −0.15");
        assert!(
            iv.lo() <= -0.15 && iv.lo() > -0.15 - 1e-10,
            "Interval reads {iv:?}, not −0.15"
        );
    }
}
