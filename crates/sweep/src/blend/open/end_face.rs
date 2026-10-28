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
//! ([`CapSliver`]), whichever the band's convexity, and a cycle not
//! clear of it is read against the sliver itself: wholly inside it,
//! crossing it, or neither certified.

use geom::Curve3;
use geom::Surface;
use geom_brep::EdgeCurveSpec;
use geom_core::{Band, Bounds, Decide, InfSpeed, Margin, Point3, Real, Sign, Tol, Vec3};
use topo::construction::Construct;
use topo::construction::{FaceSurface, MefSite};
use topo::{Body, EdgeKey, EntityId, FaceKey, HalfEdgeKey, VertexKey};

use crate::blend::battery::{EndSection, cap_incidence};
use crate::blend::naming::BlendNaming;
use crate::blend::reach::{EDGE_SAMPLES, sure};
use crate::blend::surgery::{
    ContactCarrier, Piece, SourceFaces, SplitFragments, arc_window, chord_site, disagrees,
    face_of_half, halves_of, op, piece_along, piece_distance, point_of, retire_fragment,
    split_fragment, split_param_in_span, stored_piece, unbuilt_geometry, unbuilt_run_out,
    unresolved,
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
        let stored = body
            .get_face(face)
            .and_then(|f| body.get_surface(f.surface))
            .ok_or_else(|| {
                unresolved(
                    EntityId::Face(face),
                    "a cut-off's end face's stored surface",
                )
            })?;
        let Surface::Plane {
            origin: po,
            normal: n,
            ..
        } = stored
        else {
            return Err(disagrees(
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
    pub(in crate::blend) fn carrier(&self) -> ContactCarrier<T> {
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
///   boundary, which is `A` and the two rim pieces. `A` adds no
///   extreme of its own: its ends are the feet, which end the rim
///   pieces too; a chord is extreme only at its ends; and inside its
///   window an arc of the convex `E` is farthest from `center`, or
///   lowest along `dₖ`, only where `E`'s outward normal is the
///   direction in which the function keeps growing (the reach) or
///   falling (a floor); `S` lies outside `E` there — and `center`,
///   `E`'s own centre, inside it — so `S` runs on past that point. So `reach` and the floors are those extremes over the two
///   rim pieces, each in closed form over its own window
///   ([`piece_distance`], [`piece_along`]).
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
/// that is the meter's conservative direction. So `Ω` answers only
/// "clear": a cycle wholly inside `S`, or crossing its boundary, is read
/// against that boundary itself ([`Exact`]). That reading certifies
/// inside or crossing, never outside, so a cycle not clear of `Ω` that
/// it certifies neither way is a bound — on a chord end too, where the
/// exact triangle could certify it outside (work item
/// `a-chord-ends-triangle-could-read-a-cycle-outside`).
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
    /// What the reading against `S` itself takes
    /// ([`Self::ring_inside`], [`Self::reaches_in`]).
    exact: Exact<T>,
}

/// **`S` itself, as the reading against it takes it**: one per end's
/// shape.
enum Exact<T: Real> {
    /// A round end: the sliver's own boundary, read with `E`.
    Round(SliverBoundary<T>),
    /// A chord end whose rims are lines: the triangle of the old vertex
    /// and the two feet, as its three sides, each a point on it and the
    /// unit normal into `S` in the end plane ([`Exact::triangle`]).
    Triangle([(Point3<T>, Vec3<T>); 3]),
    /// Neither: `Ω` alone, which reads nothing inside `S`.
    Unread,
}

impl<T: Real> Exact<T> {
    /// The triangle `v, foot_a, foot_b`, which runs counter-clockwise
    /// about `n = (foot_a − v) × (foot_b − v)`: each side's inward
    /// normal is `n × (its run)`.
    fn triangle(v: Point3<T>, foot_a: Point3<T>, foot_b: Point3<T>) -> Self {
        let n = (foot_a - v).cross(foot_b - v);
        Self::Triangle(
            [(v, foot_a), (foot_a, foot_b), (foot_b, v)]
                .map(|(from, to)| (from, n.cross(to - from).normalize())),
        )
    }
}

/// **How a cycle of an end face not clear of `Ω` reads against `S`**
/// ([`CapSliver::read_cycle`]).
#[derive(Clone, Copy, Debug, PartialEq)]
pub(in crate::blend) enum CycleReading<T> {
    /// A witness certainly inside `S` and another certainly outside: the
    /// inside one's depth, negative.
    Crossing(T),
    /// Wholly inside `S`, in closed form: the cycle's depth, negative.
    Inside(T),
    /// Neither certified.
    Bound,
}

/// **The sliver's own boundary on a round end**: the end curve's two
/// feet, the axis the arc runs forward about from `face_a`'s foot to
/// `face_b`'s (under half a turn), and the two rim pieces from the feet
/// to the old vertex. With the arc they close the Jordan curve that
/// bounds `S`.
struct SliverBoundary<T: Real> {
    feet: [Point3<T>; 2],
    axis: Vec3<T>,
    rims: [(Curve3<T>, (T, T)); 2],
}

/// A round section's frame in the end plane: `p − center` is inside
/// it when `(d·u/major)² + (d·w/minor)² < 1`. `u` is along the larger
/// semi-axis, so `major ≥ minor > 0` and `1/minor` bounds the scaling's
/// stretch ([`Self::depth`], [`Self::outside`]).
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
                center: c,
                semi_u,
                semi_v,
                ..
            } => self.scaled(c - center) + Curve3::semi_axes_by_size(semi_u, semi_v).0 / self.minor,
            Curve3::Spiric { .. } | Curve3::Nurbs(_) => return None,
        };
        Some(self.minor * (T::one() - rho))
    }

    /// **How far outside the section an edge stays**, in meters:
    /// positive when every point of `carrier` over `window` lies outside
    /// it. The least scaled radius of a segment is its distance from the
    /// origin in the scaled frame, the scaling being linear; a circle or
    /// an ellipse of semi-axis at most `a` about `c` stays at least
    /// `a/minor` short of `c`'s. A scaled radius `ρ > 1` is at least
    /// `minor·(ρ − 1)` from the section, as in [`Self::depth`]. `None`
    /// for a carrier with no closed form.
    fn outside(self, carrier: &Curve3<T>, (ta, tb): (T, T), center: Point3<T>) -> Option<T> {
        let rho = match *carrier {
            Curve3::Line { origin, dir } => {
                let at = |p: Point3<T>| {
                    let d = p - center;
                    (d.dot(self.u) / self.major, d.dot(self.w) / self.minor)
                };
                let (a, b) = (at(origin + dir * ta), at(origin + dir * tb));
                let (ex, ey) = (b.0 - a.0, b.1 - a.1);
                let s = (T::zero() - (a.0 * ex + a.1 * ey) / (ex.powi(2) + ey.powi(2)))
                    .max(T::zero())
                    .min(T::one());
                ((a.0 + ex * s).powi(2) + (a.1 + ey * s).powi(2)).sqrt()
            }
            Curve3::Circle {
                center: c, radius, ..
            } => self.scaled(c - center) - radius / self.minor,
            Curve3::Ellipse {
                center: c,
                semi_u,
                semi_v,
                ..
            } => self.scaled(c - center) - Curve3::semi_axes_by_size(semi_u, semi_v).0 / self.minor,
            Curve3::Spiric { .. } | Curve3::Nurbs(_) => return None,
        };
        Some(self.minor * (rho - T::one()))
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

    /// **`G` at `q = p − center`**: the largest of `‖q‖ − reach`,
    /// `minor − ‖q‖` on a round end, and each `floorₖ − q·dₖ`. Each term
    /// bounds from below `p`'s distance from a set containing `Ω` (the
    /// disc to `reach`, the outside of the disc of the minor semi-axis, a
    /// half-plane), so `G > 0` puts `p` outside `Ω`.
    fn beyond(&self, q: Vec3<T>) -> T {
        let rho = q.norm();
        let v = self
            .inside
            .map_or(rho - self.reach, |s| (rho - self.reach).max(s.minor - rho));
        self.floors
            .iter()
            .fold(v, |v, &(d, floor)| v.max(floor - q.dot(d)))
    }

    /// **How clear a straight edge is of `Ω`, point by point**: the
    /// least over the segment of `G` ([`Self::beyond`]).
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
        // `G` along the segment, each term reading `s` once. Read at the
        // point `q0 + e·s`, every coordinate carries `s`, and at
        // `Interval` a candidate's spread then lowers even a term flat
        // along the segment by about the spread itself.
        let g = |s: T| {
            let s = s.max(zero).min(len);
            let rho = ((s + b).powi(2) + d2).sqrt();
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
        // `geom_brep::quartic::quadratic_roots` picks one. It never divides by
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

/// **A lower bound on the distance between two pieces lying in one
/// plane** of unit normal `n`: positive certifies that they do not meet;
/// `None` for a carrier with no closed form here.
///
/// A circle is read whole: every point of the other piece is
/// `‖x − o‖ ∈ [near, far]` from its centre ([`piece_distance`]), so at
/// least `near − radius` from it outside and `radius − far` inside. An
/// ellipse is read the same way through the disc of its major
/// semi-axis, which holds it, and the disc of its minor, which it holds.
/// Two segments are read by each one's line: two segments that do not
/// meet always leave one of them wholly on one side of the other's line.
fn apart<T: Bounds>(a: Piece<'_, T>, b: Piece<'_, T>, n: Vec3<T>) -> Option<T> {
    let round = |c: &Curve3<T>| match *c {
        Curve3::Circle { center, radius, .. } => Some((center, radius, radius)),
        Curve3::Ellipse {
            center,
            semi_u,
            semi_v,
            ..
        } => {
            let (outer, inner) = Curve3::semi_axes_by_size(semi_u, semi_v);
            Some((center, outer, inner))
        }
        _ => None,
    };
    let about = |(o, outer, inner): (Point3<T>, T, T), (carrier, window): Piece<'_, T>| {
        let (near, far) = piece_distance(carrier, window, o)?;
        Some((near - outer).max(inner - far))
    };
    let side = |(x, _): Piece<'_, T>, (y, window): Piece<'_, T>| {
        let Curve3::Line { origin, dir } = *x else {
            return None;
        };
        let (low, high) = piece_along(y, window, origin, n.cross(dir).normalize())?;
        Some(low.max(T::zero() - high))
    };
    match (round(a.0), round(b.0)) {
        (Some(ra), Some(rb)) => Some(about(ra, b)?.max(about(rb, a)?)),
        (Some(ra), None) => about(ra, b),
        (None, Some(rb)) => about(rb, a),
        (None, None) => Some(side(a, b)?.max(side(b, a)?)),
    }
}

impl<T: Decide + Bounds> CapSliver<T> {
    /// **How surely the point `p` of the end plane lies inside `S`**:
    /// positive when it certainly does; `None` on a chord end whose rims
    /// are not lines, or where a rim has no closed form.
    ///
    /// `S` is the region the Jordan curve `A ∪ Pa ∪ Pb` bounds (the arc
    /// and the two rim pieces), and the section's centre `c` is not in
    /// it: the section is convex and `S` lies outside it. The segment
    /// `[c, p]` meets the section's boundary once, and that point is on
    /// `A` when `p` lies outside the section and strictly inside the
    /// wedge `A` subtends at `c`, the arc running under half a turn. If
    /// the segment also misses both rim pieces it crosses the curve once,
    /// so `p` is inside. The reading is the least of `p`'s distance
    /// outside the section (`minor·(ρ − 1)`, `ρ` its scaled radius), its
    /// distance inside each of the wedge's two lines, and the segment's
    /// clearance from each rim piece ([`apart`]).
    ///
    /// On a chord end `S` is the triangle ([`Exact::Triangle`]), and the
    /// reading is `p`'s least distance inside its three sides.
    fn inside_point(&self, p: Point3<T>) -> Option<T> {
        let b = match &self.exact {
            Exact::Round(b) => b,
            Exact::Triangle(sides) => {
                return sides
                    .iter()
                    .map(|(o, m)| (p - *o).dot(*m))
                    .reduce(|a, b| a.min(b));
            }
            Exact::Unread => return None,
        };
        let section = self.inside?;
        let q = p - self.center;
        let [ua, ub] = b.feet.map(|f| f - self.center);
        let mut least = (section.minor * (section.scaled(q) - T::one()))
            .min(b.axis.cross(ua).normalize().dot(q))
            .min(ub.cross(b.axis).normalize().dot(q));
        let ray = Curve3::Line {
            origin: self.center,
            dir: q,
        };
        let span = (T::zero(), T::one());
        for (rim, window) in &b.rims {
            least = least.min(apart((&ray, span), (rim, *window), b.axis)?);
        }
        Some(least)
    }

    /// **Whether `S` is read as its exact triangle**, which is convex.
    pub(in crate::blend) fn convex(&self) -> bool {
        matches!(self.exact, Exact::Triangle(_))
    }

    /// **How surely the point `p` of the end plane lies outside `S`**:
    /// positive when it certainly does — outside `Ω` ([`Self::beyond`]),
    /// or inside the section, which `S` lies outside of
    /// (`minor·(1 − ρ)`, as [`SectionFrame::depth`] reads it), or on a
    /// chord end past a side of the triangle.
    fn outside_point(&self, p: Point3<T>) -> T {
        let q = p - self.center;
        let beyond = self.beyond(q);
        if let Exact::Triangle(sides) = &self.exact {
            return sides
                .iter()
                .map(|(o, m)| T::zero() - (p - *o).dot(*m))
                .fold(beyond, |a, b| a.max(b));
        }
        self.inside.map_or(beyond, |section| {
            beyond.max(section.minor * (T::one() - section.scaled(q)))
        })
    }

    /// [`EDGE_SAMPLES`] places along the `carrier` over `window`, ends
    /// included: the witnesses the reading against `S` decides, exactly
    /// where each is read, so how many there are decides only how
    /// readily a crossing is told from a bound.
    fn places(carrier: &Curve3<T>, (ta, tb): (T, T)) -> impl Iterator<Item = Point3<T>> + '_ {
        (0..EDGE_SAMPLES)
            .map(move |i| carrier.eval(geom_brep::schedule_param(ta, tb, i, EDGE_SAMPLES)))
    }

    /// **A cycle of the end face read against `S` itself**, three ways
    /// ([`CycleReading`]): `pieces` is every edge of the cycle but the
    /// two rims the cut shortens, and `cut` whether the cycle carries
    /// them. A crossing needs a witness on each side; the cut cycle always
    /// has points outside, its rims running on past the feet, and is
    /// never read inside. The witnesses are samples, so a cycle that
    /// crosses with none on one side reads as a bound, never as inside.
    pub(in crate::blend) fn read_cycle(
        &self,
        pieces: &[Piece<'_, T>],
        cut: bool,
        band: Band,
    ) -> CycleReading<T> {
        if !cut && let Some(depth) = self.ring_inside(pieces).filter(|d| sure(*d, band)) {
            return CycleReading::Inside(T::zero() - depth);
        }
        let deepest = pieces
            .iter()
            .filter_map(|&(carrier, window)| self.reaches_in(carrier, window))
            .filter(|x| sure(*x, band))
            .reduce(|m, x| m.max(x));
        let reaches_out = cut
            || pieces.iter().any(|&(carrier, window)| {
                self.reaches_out(carrier, window)
                    .is_some_and(|x| sure(x, band))
            });
        match deepest.filter(|_| reaches_out) {
            Some(depth) => CycleReading::Crossing(T::zero() - depth),
            None => CycleReading::Bound,
        }
    }

    /// **How surely some point of an end-face edge lies inside `S`**:
    /// positive when one of its witnesses does ([`Self::inside_point`]).
    fn reaches_in(&self, carrier: &Curve3<T>, window: (T, T)) -> Option<T> {
        Self::places(carrier, window).try_fold(None, |most: Option<T>, p| {
            let x = self.inside_point(p)?;
            Some(Some(most.map_or(x, |m| m.max(x))))
        })?
    }

    /// **How surely some point of an end-face edge lies outside `S`**:
    /// positive when one of its witnesses does ([`Self::outside_point`]).
    fn reaches_out(&self, carrier: &Curve3<T>, window: (T, T)) -> Option<T> {
        Self::places(carrier, window)
            .map(|p| self.outside_point(p))
            .reduce(|m, x| m.max(x))
    }

    /// **How surely a whole cycle of the end face lies inside `S`**:
    /// positive when it certainly does, and then a lower bound on its
    /// distance from `∂S`; `None` on a chord end whose rims are not
    /// lines, or for a piece with no closed form. `pieces` is every edge
    /// of the cycle.
    ///
    /// A connected curve that misses `∂S` lies wholly inside `S` or
    /// wholly outside it, so it is inside when it misses the arc, misses
    /// both rim pieces and has one point inside. Each piece misses the
    /// arc by lying outside the section ([`SectionFrame::outside`]), and
    /// the cycle's first point is read by [`Self::inside_point`]. A cycle
    /// that carries no rim misses the rim pieces already, a face's cycles
    /// being disjoint; their clearance ([`apart`]) is read for the depth,
    /// which it bounds where the cycle is nearest a rim.
    ///
    /// On a chord end `S` is the triangle, the intersection of three
    /// half-planes, so a cycle lies inside it when every piece stays
    /// inside each side, its least reach along the side's inward normal
    /// ([`piece_along`]) positive; the least of those is its depth.
    fn ring_inside(&self, pieces: &[Piece<'_, T>]) -> Option<T> {
        let b = match &self.exact {
            Exact::Round(b) => b,
            Exact::Triangle(sides) => {
                let mut least: Option<T> = None;
                for &(carrier, window) in pieces {
                    for (o, m) in sides {
                        let (low, _) = piece_along(carrier, window, *o, *m)?;
                        least = Some(least.map_or(low, |l| l.min(low)));
                    }
                }
                return least;
            }
            Exact::Unread => return None,
        };
        let section = self.inside?;
        let (first, (t0, _)) = *pieces.first()?;
        let mut least = self.inside_point(first.eval(t0))?;
        for &(carrier, window) in pieces {
            least = least.min(section.outside(carrier, window, self.center)?);
            for (rim, rim_window) in &b.rims {
                least = least.min(apart((carrier, window), (rim, *rim_window), b.axis)?);
            }
        }
        Some(least)
    }

    /// **The region a cut-off removes from its end face**, read off the
    /// source before any mutation. `rims` pairs each rim with its foot,
    /// `face_a`'s first. `reach` and the floors are read over the rim
    /// pieces alone (the type's doc says why the end curve adds none);
    /// the end curves differ in `center`, `inside` and `exact`, and a
    /// round end's window from `face_a`'s foot to `face_b`'s is read
    /// only to check that it runs forward.
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
            .ok_or_else(|| unresolved(EntityId::Vertex(vertex), "a cut-off's old vertex"))?;
        let [(rim_a, foot_a), (rim_b, foot_b)] = rims;
        // The rims' pieces from their feet to the old vertex.
        let pieces = [
            rim_piece(body, rim_a, vertex, foot_a)?,
            rim_piece(body, rim_b, vertex, foot_b)?,
        ];
        // The end curve's centre, and on a round end the section's frame.
        let (center, round) = match curve {
            EndCurve::Chord => (foot_a + (foot_b - foot_a) * T::from_f64(0.5), None),
            EndCurve::Round(arc) => {
                // The frame with the major semi-axis first
                // (`Curve3::major_frame`).
                let (center, axis, major, minor, u) = match *arc {
                    Curve3::Circle {
                        center,
                        axis,
                        radius,
                        u_ref,
                    } => (center, axis, radius, radius, u_ref),
                    ref ellipse @ Curve3::Ellipse { center, axis, .. } => {
                        let Some(f) = ellipse.major_frame() else {
                            unreachable!("an Ellipse carrier has a major frame")
                        };
                        (center, axis, f.major.abs(), f.minor.abs(), f.major_dir)
                    }
                    Curve3::Line { .. } | Curve3::Spiric { .. } | Curve3::Nurbs(_) => {
                        return Err(unbuilt_geometry(
                            EntityId::Vertex(vertex),
                            "a cut-off's round end curve is neither a circle nor an ellipse",
                        ));
                    }
                };
                let backward = || {
                    disagrees(
                        EntityId::Vertex(vertex),
                        "a cut-off's round end curve does not run forward from one foot to \
                         the other",
                    )
                };
                let (ta, tb) = arc_window(arc, foot_a, foot_b).ok_or_else(backward)?;
                if (tb - ta).lo() <= 0.0 {
                    return Err(backward());
                }
                let frame = SectionFrame {
                    u,
                    w: axis.cross(u),
                    major,
                    minor,
                };
                (center, Some((frame, axis)))
            }
        };
        let toward = (pv - center).normalize();
        let mut directions = vec![toward];
        if let Some((frame, _)) = round {
            directions.extend([frame.u, -frame.u, frame.w, -frame.w]);
        }
        // `reach` and the floors over the rim pieces alone: `A` adds no
        // extreme of its own.
        let extent = |i: usize| {
            let (carrier, window) = &pieces[i];
            let unsupported = || {
                unbuilt_geometry(
                    EntityId::Edge(rims[i].0),
                    "an end face's rim is neither a line, a circle nor an ellipse, the only \
                     rims the sliver's extent is closed-form over",
                )
            };
            let (_, far) = piece_distance(carrier, *window, center).ok_or_else(unsupported)?;
            let lows = directions
                .iter()
                .map(|&d| Some(piece_along(carrier, *window, center, d)?.0))
                .collect::<Option<Vec<T>>>()
                .ok_or_else(unsupported)?;
            Ok::<_, BlendError>((far, lows))
        };
        let ((far_a, lows_a), (far_b, lows_b)) = (extent(0)?, extent(1)?);
        let reach = far_a.max(far_b);
        let floors = directions
            .iter()
            .zip(lows_a.into_iter().zip(lows_b))
            .map(|(&d, (a, b))| (d, a.min(b)))
            .collect();
        // A chord end whose rims are lines cuts off the triangle
        // `V, foot_a, foot_b`.
        let lines = pieces.iter().all(|(c, _)| matches!(c, Curve3::Line { .. }));
        let exact = match round {
            Some((_, axis)) => Exact::Round(SliverBoundary {
                feet: [foot_a, foot_b],
                axis,
                rims: pieces.map(|(carrier, window)| (carrier.clone(), window)),
            }),
            None if lines => Exact::triangle(pv, foot_a, foot_b),
            None => Exact::Unread,
        };
        Ok(Self {
            cap,
            rims: [rim_a, rim_b],
            center,
            inside: round.map(|(frame, _)| frame),
            reach,
            floors,
            exact,
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
        let (carrier, _) = stored_piece(body, rim)?;
        let rate = match *carrier {
            Curve3::Line { .. } => InfSpeed::new(T::one()),
            Curve3::Circle { radius, .. } => InfSpeed::new(radius),
            // An ellipse's speed is never below its minor semi-axis.
            Curve3::Ellipse { semi_u, semi_v, .. } => {
                InfSpeed::new(Curve3::semi_axes_by_size(semi_u, semi_v).1)
            }
            Curve3::Spiric { .. } | Curve3::Nurbs(_) => {
                return Err(unbuilt_geometry(
                    EntityId::Edge(rim),
                    "a rim two cut-offs share is neither a line, a circle nor an ellipse",
                ));
            }
        };
        let he_plus = body
            .get_edge(rim)
            .ok_or_else(|| unresolved(EntityId::Edge(rim), "a shared rim"))?
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
/// [`BlendError::UnsupportedRunOut`] from the foot's parameter
/// ([`foot_param`]); [`BlendError::BodyNotIntact`] when the rim does not
/// resolve, carries no certified carrier, or does not end at `vertex`.
fn rim_piece<T: Decide + Bounds>(
    body: &Body<T>,
    rim: EdgeKey,
    vertex: VertexKey,
    foot: Point3<T>,
) -> Result<Piece<'_, T>, BlendError> {
    let he_plus = body
        .get_edge(rim)
        .ok_or_else(|| unresolved(EntityId::Edge(rim), "an end face's rim"))?
        .he_plus;
    let (carrier, (t0, t1)) = stored_piece(body, rim)?;
    let t = foot_param(body, rim, vertex, foot)?;
    if body.get_half_edge(he_plus).map(|h| h.start) == Some(vertex) {
        Ok((carrier, (t0, t)))
    } else if body.half_edge_end(he_plus) == Some(vertex) {
        Ok((carrier, (t, t1)))
    } else {
        Err(disagrees(
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
        disagrees(
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
        return Err(disagrees(
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
        return Err(disagrees(
            EntityId::Vertex(v),
            "an end face's cycle around the old vertex is not flanked by the two feet just split",
        ));
    }
    let (px, py) = (
        point_of(body, x).ok_or_else(|| unresolved(EntityId::Vertex(x), "a foot"))?,
        point_of(body, y).ok_or_else(|| unresolved(EntityId::Vertex(y), "a foot"))?,
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
        .ok_or_else(|| unresolved(EntityId::Edge(rims.a.near), "a split rim's near piece"))?;
    // The sliver is on whichever side of the near piece is NOT the band.
    let dying = if face_of_half(body, ahp) == Some(band) {
        ahm
    } else {
        ahp
    };
    sources.kef_minted(body, dying, "end sliver kef", tol)?;
    let (bhp, bhm) = halves_of(body, rims.b.near)
        .ok_or_else(|| unresolved(EntityId::Edge(rims.b.near), "a split rim's near piece"))?;
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
    use super::{CapSliver, CycleReading, Exact, SectionFrame, SliverBoundary};
    use geom::Curve3;
    use geom_core::{Band, Bounds, Interval, Point3, Real, Tol, Vec3};
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
                exact: Exact::Unread,
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

    /// The least of `G` over `a → b`: the least of a dense run of samples,
    /// refined by ternary search about it. Never below the true least.
    fn sampled_least(region: &Region, a: (f64, f64), b: (f64, f64)) -> f64 {
        let at = |t: f64| region.g((a.0 + (b.0 - a.0) * t, a.1 + (b.1 - a.1) * t));
        const N: u32 = 2000;
        let (best, n) = (0..=N)
            .map(|n| (at(f64::from(n) / f64::from(N)), n))
            .fold((f64::INFINITY, 0), |l, r| if r.0 < l.0 { r } else { l });
        let (mut lo, mut hi) = (
            f64::from(n.saturating_sub(1)) / f64::from(N),
            f64::from((n + 1).min(N)) / f64::from(N),
        );
        for _ in 0..100 {
            let (m1, m2) = (lo + (hi - lo) / 3.0, hi - (hi - lo) / 3.0);
            if at(m1) < at(m2) {
                hi = m2;
            } else {
                lo = m1;
            }
        }
        best.min(at(0.5 * (lo + hi)))
    }

    /// **The meter's two claims on `a → b`**: at `f64` it reads the least
    /// `G` to the samples' slack; at `Interval` its enclosure's low end lies
    /// at or below that least and within rounding of the `f64` reading, the
    /// enclosure carrying the gain rather than the whole segment's spread.
    pub(super) fn check(region: &Region, a: (f64, f64), b: (f64, f64), case: impl Fn() -> String) {
        let least = sampled_least(region, a, b);
        // `G` is 1-Lipschitz, so the samples' least is within half a
        // spacing of the true one; rounding grows with the distance from
        // the centre.
        let scale = (b.0 - a.0).hypot(b.1 - a.1);
        let slack = scale / 4000.0 + 1e-14 * (1.0 + a.0.hypot(a.1) + scale);
        let (m, iv) = region.meter(a, b);
        let what = || {
            format!(
                "{}: {a:?} → {b:?}, reach {}, section {:?}, floors {:?}",
                case(),
                region.reach,
                region.section,
                region.floors,
            )
        };
        assert!(
            m <= least + slack && m >= least - slack,
            "f64 reads {m}, the least G is {least}: {}",
            what()
        );
        assert!(
            iv.lo() <= least + slack && iv.lo() >= m - 1e-12 * (1.0 + a.0.hypot(a.1) + scale),
            "Interval reads {iv:?}, f64 {m}, the least G is {least}: {}",
            what()
        );
    }

    /// **A segment `~5·10³` from the centre at its ends, run along one
    /// floor's line, whose least is that floor's flat term**, met near the
    /// foot by a second floor crossing the minor disc's term where the two
    /// differ by little (`a′² ≈ d2`): that crossing's quotient of two
    /// differences reads at `Interval` as a run `10⁻⁷` long. Found by the
    /// line-meter fuzzer at `CAD_FUZZ_SEED=0xa9b25b00fc22acb2`, case 80.
    #[test]
    fn a_far_segment_reads_a_floor_crossing_the_minor_disc_to_rounding() {
        let (a, b) = (
            (2778.682165969168, -3803.6929153369833),
            (-3120.380308781379, 4271.024536003959),
        );
        let region = Region {
            section: Some((0.0, 0.3739873428335117, 0.3739873428335117)),
            reach: 1.168384052714115,
            floors: vec![
                (
                    (0.8074717451340941, 0.5899062474750547),
                    -0.11487650578226152,
                ),
                (
                    (-0.9665909248993477, 0.256323982300181),
                    0.33182995179051883,
                ),
            ],
        };
        check(&region, a, b, || "the far crossing".into());
    }

    /// **Two opposite floors whose terms run all but parallel along the
    /// segment**: their crossing is ill-conditioned (`~10⁻¹¹` here) where
    /// `G` is flat, so only a reading that spreads a term over the
    /// candidate's whole box, rather than along the segment, loses it.
    /// Found by the line-meter fuzzer at `CAD_FUZZ_SEED=0x1281277f8dfa215d`,
    /// case 30.
    #[test]
    fn two_floors_parallel_along_a_segment_read_their_crossing_to_rounding() {
        let region = Region {
            section: Some((0.0, 0.25299113746373153, 0.25299113746373153)),
            reach: 0.7804898663095933,
            floors: vec![
                (
                    (0.7972795443729409, -0.6036102452116562),
                    -0.3595462882329724,
                ),
                (
                    (-0.7972795443729409, 0.6036102452116562),
                    0.3588416431553506,
                ),
                (
                    (0.9816386005106127, 0.19075025029489656),
                    0.11024723871746345,
                ),
                (
                    (-0.9005322168633404, 0.434789289646373),
                    0.021858290386058923,
                ),
                ((0.818640532000699, 0.5743062592081098), -0.0035775011289499),
            ],
        };
        check(
            &region,
            (-0.8131924792185901, -0.4784468841887551),
            (0.9976382564163786, 1.9133917489300676),
            || "the parallel floors".into(),
        );
    }

    /// **The quarter sliver**: a right dihedral's supports `y = −1` and
    /// `x = −1` in the end plane `z = 0`, the unit section about the
    /// origin, so `S` is the square `[−1, 0]²` less the unit disc. The
    /// arc runs a quarter turn about `−z` from the foot `(0, −1)` to
    /// `(−1, 0)`, and the rims from the feet to `V = (−1, −1)`. No
    /// floors: `Ω` is the annulus out to `‖V‖`, which contains `S`.
    fn quarter() -> CapSliver<f64> {
        let p = |x: f64, y: f64| Point3::new(x, y, 0.0);
        let v = |x: f64, y: f64| Vec3::new(x, y, 0.0);
        let rim = |o: Point3<f64>, d: Vec3<f64>| (Curve3::Line { origin: o, dir: d }, (0.0, 1.0));
        CapSliver {
            cap: FaceKey::default(),
            rims: [EdgeKey::default(); 2],
            center: p(0.0, 0.0),
            inside: Some(SectionFrame {
                u: v(1.0, 0.0),
                w: v(0.0, 1.0),
                major: 1.0,
                minor: 1.0,
            }),
            reach: 2f64.sqrt(),
            floors: Vec::new(),
            exact: Exact::Round(SliverBoundary {
                feet: [p(0.0, -1.0), p(-1.0, 0.0)],
                axis: Vec3::new(0.0, 0.0, -1.0),
                rims: [
                    rim(p(0.0, -1.0), v(-1.0, 0.0)),
                    rim(p(-1.0, 0.0), v(0.0, -1.0)),
                ],
            }),
        }
    }

    /// A whole circle of radius `r` about `(x, y)` in the end plane.
    fn ring(x: f64, y: f64, r: f64) -> Curve3<f64> {
        Curve3::Circle {
            center: Point3::new(x, y, 0.0),
            axis: Vec3::new(0.0, 0.0, 1.0),
            u_ref: Vec3::new(1.0, 0.0, 0.0),
            radius: r,
        }
    }

    #[allow(clippy::expect_used)]
    fn band() -> Band {
        Band::linear(Tol::witness()).expect("the witness band")
    }

    /// **A point is read inside `S` only inside the wedge between the
    /// feet**: `(−0.9, −0.9)` is inside, `0.1` from both rims (the
    /// segment from the centre clears each by that much); `(0.5, −0.95)`
    /// lies past the foot `(0, −1)`, outside the section and clear of
    /// both rims along its segment from the centre, and only the wedge's
    /// line through that foot reads it out (at `−0.5`, its `x`).
    #[test]
    fn a_point_past_a_foot_is_read_out_by_the_wedge_alone() {
        let s = quarter();
        let inside = s.inside_point(Point3::new(-0.9, -0.9, 0.0));
        assert!(
            inside.is_some_and(|x| (x - 0.1).abs() < 1e-12),
            "(−0.9, −0.9) reads 0.1 inside: {inside:?}"
        );
        let past = s.inside_point(Point3::new(0.5, -0.95, 0.0));
        assert!(
            past.is_some_and(|x| (x + 0.5).abs() < 1e-12),
            "(0.5, −0.95) reads out at −0.5, the wedge's: {past:?}"
        );
    }

    /// **The sliver's three readings of a cycle, with their depths.**
    /// - A ring of radius `0.05` about `(−0.85, −0.85)` is wholly inside,
    ///   `0.1` from both rims: that, not its first point's `0.15` from
    ///   the flat nor its `0.152` from the arc, is its depth.
    /// - A ring of radius `0.3` about `(−0.6, −0.6)` crosses the arc,
    ///   with witnesses on both sides.
    /// - A ring of radius `0.05` touching the arc from outside the
    ///   section has witnesses inside `S` and none certainly outside it:
    ///   a bound, not a crossing.
    /// - The cut cycle has points outside by construction: one inside
    ///   witness makes it a crossing, and it is never read inside.
    #[test]
    fn a_cycle_reads_inside_crossing_or_bound_with_its_depth() {
        let s = quarter();
        let whole = (0.0, std::f64::consts::TAU);
        let inside = ring(-0.85, -0.85, 0.05);
        let got = s.read_cycle(&[(&inside, whole)], false, band());
        assert!(
            matches!(got, CycleReading::Inside(d) if (d + 0.1).abs() < 1e-12),
            "a ring 0.1 from the rims is inside at −0.1: {got:?}"
        );
        let across = ring(-0.6, -0.6, 0.3);
        let got = s.read_cycle(&[(&across, whole)], false, band());
        assert!(
            matches!(got, CycleReading::Crossing(d) if d < 0.0),
            "a ring across the arc crosses: {got:?}"
        );
        let k = 1.05 / 2f64.sqrt();
        let touching = ring(-k, -k, 0.05);
        let got = s.read_cycle(&[(&touching, whole)], false, band());
        assert_eq!(
            got,
            CycleReading::Bound,
            "a ring touching the arc from inside S has no outside witness"
        );
        let got = s.read_cycle(&[(&inside, whole)], true, band());
        assert!(
            matches!(got, CycleReading::Crossing(d) if d < -0.1),
            "the cut cycle crosses on one inside witness: {got:?}"
        );
    }

    /// **The chord end's triangle**: the same dihedral cut off by the
    /// chord between the feet `(0, −1)` and `(−1, 0)`, so `S` is the
    /// triangle with the old vertex `V = (−1, −1)`, read through its
    /// three sides. `Ω` is the disc about the chord's midpoint out to
    /// `V`, on `V`'s side of the chord.
    fn chord_triangle() -> CapSliver<f64> {
        let p = |x: f64, y: f64| Point3::new(x, y, 0.0);
        let toward = Vec3::new(-1.0, -1.0, 0.0) / 2f64.sqrt();
        CapSliver {
            cap: FaceKey::default(),
            rims: [EdgeKey::default(); 2],
            center: p(-0.5, -0.5),
            inside: None,
            reach: 0.5 * 2f64.sqrt(),
            floors: vec![(toward, 0.0)],
            exact: Exact::triangle(p(-1.0, -1.0), p(0.0, -1.0), p(-1.0, 0.0)),
        }
    }

    /// **A chord end reads a cycle three ways through its triangle**: a
    /// ring of radius `0.05` about `(−0.85, −0.85)` is inside at exactly
    /// its distance from the two rims, `0.1`; one of radius `0.2` about
    /// the chord's midpoint crosses it, witnesses on both sides; and a
    /// point's reading is its least distance inside the three sides,
    /// negative past one.
    #[test]
    fn a_chord_end_reads_a_cycle_through_its_triangle() {
        let s = chord_triangle();
        let whole = (0.0, std::f64::consts::TAU);
        let inside = ring(-0.85, -0.85, 0.05);
        let got = s.read_cycle(&[(&inside, whole)], false, band());
        assert!(
            matches!(got, CycleReading::Inside(d) if (d + 0.1).abs() < 1e-12),
            "a ring 0.1 from the rims is inside at −0.1: {got:?}"
        );
        let across = ring(-0.5, -0.5, 0.2);
        let got = s.read_cycle(&[(&across, whole)], false, band());
        assert!(
            matches!(got, CycleReading::Crossing(d) if d < 0.0),
            "a ring across the chord crosses: {got:?}"
        );
        let deep = s.inside_point(Point3::new(-0.9, -0.8, 0.0));
        assert!(
            deep.is_some_and(|x| (x - 0.1).abs() < 1e-12),
            "(−0.9, −0.8) reads 0.1 inside, its distance from x = −1: {deep:?}"
        );
        let past = s.inside_point(Point3::new(-0.3, -0.3, 0.0));
        assert!(
            past.is_some_and(|x| (x + 0.4 / 2f64.sqrt()).abs() < 1e-12),
            "(−0.3, −0.3) reads out by its distance past the chord: {past:?}"
        );
    }
}
