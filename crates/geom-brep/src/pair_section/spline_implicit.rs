//! **A spline wall and an implicit partner: the pair classified.** The
//! partner is a surface with an implicit residual in metres, a plane, a
//! cylinder, a sphere or a torus, and the wall a NURBS surface (an
//! approximating surface read as its fit). The section is the zero set of
//! `φ = f∘S` over the wall's knot rectangle (C3's residual × chart trace
//! shape), and the pair decides its tangency and coincidence once, in its
//! classification on `φ`, before any march (C5), in three stages run in
//! this order:
//!
//! 1. **the hull**, the bound on `φ` its net gives: a net decided on one
//!    side of the partner is apart; one within the zero band of it lies
//!    on it ([`SectionError::CoincidentSurfaces`]), and one in band
//!    escalates. Then the door: a side of the rectangle collapsed to a
//!    point ([`crate::chart_singularity::spline_collapsed_row`]) not
//!    decided clear of the partner refuses, named. On the plane `φ` is
//!    linear, the net's control heights bound it, and both read at every
//!    scalar and with no march door (`SplineImplicitPose::hull`); a
//!    curved partner's `f` is not, and its hull is the Bernstein hull of
//!    the tensor composite `f∘S`, formed in certification arithmetic
//!    through the march door ([`curved`]);
//! 2. **the boundary pass**, through the march door, each side of the
//!    rectangle against the partner (the plane's [`crate::PlaneSection`]
//!    pass, or a curved partner's [`crate::ssi::boundary`] pass on its
//!    form, run once): a graze or a side held tangent refuses by name,
//!    and its crossings and `Side` and `Corner` regions are where the
//!    section ends;
//! 3. **the regularity sweep** ([`sweep`]): where `∇φ` is bounded off
//!    zero, past the band's exclusions, no component closes in the
//!    wall's chart. That verdict is the section, no component where the
//!    pass found no end and otherwise [`Components::Reaching`] on the
//!    sides it found them on; the interior-loop guard's loop-free stage
//!    is it. Where the sweep declines, a plane's pose is read whole on
//!    the same pass, the pass carried on the pose; a curved partner's
//!    whole reading over the rectangle is not built, and its pose refuses
//!    as the general rung's ([`SectionError::RoutesToGeneralRung`]).
//!
//! The order is the proof's: the sweep's loop-free verdict says nothing
//! of a wall within the band of the partner everywhere, which the hull
//! decides, nor of a component touching a side without crossing it,
//! which the pass refuses as a graze.
//!
//! Read whole, the section of a plane and a NURBS wall (an approximating
//! surface read as its fit) is [`crate::PlaneSection::all`] over the
//! wall's whole knot rectangle: every branch with its three-limb certificate, every
//! boundary region, and the accounting that they are all of them. The
//! plane is read at its pivot, its origin moved to its foot nearest the
//! wall ([`SplineImplicitPose::anchored`]), so where its origin is stored
//! changes nothing. The foot is found in `f64`, so the plane read may
//! stand off the stored one; that shift is certified and charged: the
//! reading's band is padded by it, and the witness's bracket reads the
//! stored plane's height. Each piece of the section is a component:
//!
//! - a closed branch is a [`ComponentShape::Loop`], with its witness;
//! - an open branch, which runs between two crossings of the wall's
//!   rectangle, and a `Side` or `Corner` region, where the section runs
//!   along a side or leaves at a corner within the band, are
//!   [`ComponentShape::Open`], reaching the sides of the wall's chart
//!   they end on ([`ChartReach`]); a branch that ends anywhere else is
//!   refused, never read as reaching.
//!
//! **A branch's witness** is the wall's point `q = S(P(mid))` at the
//! middle of the branch's pcurve on the wall (an enclosure's centre),
//! and its pad a metre bound on how far `q` stands from a point of that
//! branch's own arc: a segment of the chart through `P(mid)` along the
//! plane height's gradient, inside the window of the branch's certified
//! chart tube that holds `P(mid)`, whose ends the stored plane's height
//! is certified to straddle, halved about the sign change while the
//! sign stays certified ([`bracket`]). The segment holds a point of the
//! section, and inside the window limb 3 proves the section one arc,
//! the branch's; the image's length from `P(mid)` to it is bounded by
//! the chart extents and the wall's certified chart speeds over the
//! segment's own box, so the pad shrinks with the band. A branch no
//! bracket fits carries no witness.
//!
//! The fitted carriers the reading certifies are not kept: no reader
//! carries a plane × spline section ([`UNCARRIED`]), and a carried
//! component's pad is a closeness bound a chart tube does not give.
//!
//! A refusal maps by name: a tangency the reading reports (the march's
//! transversality, a graze of a side, a side held tangent) is a pinch
//! named by its decision, an escalation is the value's escalation, and
//! any other refusal, an accounting that did not close among them, is
//! [`SectionError::Unaccounted`], carrying its cause.
//!
//! The reading is a march, so the arm is the march door's
//! ([`super::SsiLane::marched`], at `f64` alone).

use geom::{NurbsSurface, Surface};
use geom_core::k_stats::{Magnitude, decide, decide_magnitude};
use geom_core::{Band, Bounds, Decide, Interval, Margin, Point3, Real, Sign, Vec3};

use super::{
    ChartReach, ClassCarrier, ComponentEnds, ComponentShape, Components, PairPose, PairSection,
    SectionComponent, SsiLane,
};
use crate::ExtentBall;
use crate::intersect::SectionError;
use crate::ssi::boundary::BoundaryPass;
use crate::ssi::enclose::{Box3, NurbsBoxes, UvWindow};
use crate::ssi::exhaust::UvRect;
use crate::ssi::{BranchEnd, SsiBoundaryContact, SsiBranch, SsiDomain, SsiTube};

mod curved;
mod sweep;

/// **A spline wall and an implicit partner**, in the order the section
/// was asked of.
#[derive(Clone, Debug)]
pub struct SplineImplicitPose<T: Real> {
    partner: Surface<T>,
    wall: Surface<T>,
    /// The wall's net: a NURBS wall's own, an approximating surface's
    /// fit's.
    net: geom::NurbsSurface<T>,
    partner_first: bool,
    /// The boundary pass the classification ran (stage 2), carried to
    /// the whole reading so it runs once.
    pass: Option<std::sync::Arc<BoundaryPass>>,
}

impl<T: Real> SplineImplicitPose<T> {
    /// The pose of `a` and `b`, one a plane, a cylinder, a sphere or a
    /// torus and the other a NURBS or an approximating surface; `None`
    /// for any other pair.
    #[must_use]
    pub fn of(a: &Surface<T>, b: &Surface<T>) -> Option<Self> {
        let implicit = |s: &Surface<T>| {
            matches!(
                s,
                Surface::Plane { .. }
                    | Surface::Cylinder { .. }
                    | Surface::Sphere { .. }
                    | Surface::Torus { .. }
            )
        };
        let (partner, wall, partner_first) = if implicit(a) {
            (a, b, true)
        } else if implicit(b) {
            (b, a, false)
        } else {
            return None;
        };
        Some(Self {
            partner: partner.clone(),
            wall: wall.clone(),
            net: wall.spline_chart()?.clone(),
            partner_first,
            pass: None,
        })
    }

    /// The two surfaces in the order the pose was asked of.
    pub fn operands(&self) -> (&Surface<T>, &Surface<T>) {
        if self.partner_first {
            (&self.partner, &self.wall)
        } else {
            (&self.wall, &self.partner)
        }
    }

    /// The implicit partner.
    pub fn partner(&self) -> &Surface<T> {
        &self.partner
    }

    /// Whether the partner is a plane, whose `φ` is linear.
    pub fn is_plane(&self) -> bool {
        matches!(self.partner, Surface::Plane { .. })
    }

    /// The wall's net: a NURBS wall's own, an approximating surface's
    /// fit's.
    pub fn net(&self) -> &geom::NurbsSurface<T> {
        &self.net
    }

    /// `[on the partner, on the wall]` in the order the pose was asked
    /// of.
    pub(super) fn order<V>(&self, on_partner: V, on_wall: V) -> [V; 2] {
        if self.partner_first {
            [on_partner, on_wall]
        } else {
            [on_wall, on_partner]
        }
    }

    /// **A ball holding the whole section**: the wall lies in its net's
    /// convex hull (its weights are positive), so in the ball about the
    /// net's control points: about the middle of their box, reaching the
    /// farthest.
    pub fn ball(&self) -> ExtentBall<T> {
        let control = self.net.control();
        let Some(&first) = control.first() else {
            return ExtentBall::new(Point3::origin(), T::zero());
        };
        let (lo, hi) = control.iter().fold((first, first), |(lo, hi), &p| {
            (
                Point3::new(lo.x.min(p.x), lo.y.min(p.y), lo.z.min(p.z)),
                Point3::new(hi.x.max(p.x), hi.y.max(p.y), hi.z.max(p.z)),
            )
        });
        let center = lo + (hi - lo) * T::from_f64(0.5);
        let radius = control
            .iter()
            .fold(T::zero(), |r, &p| r.max((p - center).norm()));
        ExtentBall::new(center, radius)
    }
}

impl<T: Real> SplineImplicitPose<T> {
    /// The pose of a pair the arm serves.
    ///
    /// # Errors
    ///
    /// [`SectionError::WrongLane`] for any other pair.
    fn served(a: &Surface<T>, b: &Surface<T>) -> Result<Self, SectionError> {
        Self::of(a, b).ok_or(SectionError::WrongLane {
            expected: "a plane, cylinder, sphere or torus and a NURBS or approximating surface",
        })
    }

    /// The pair's section with these components, nothing decided Zero.
    fn section(self, components: Components<T>) -> PairSection<T> {
        let lever = self.ball().radius();
        PairSection {
            pose: PairPose::SplineImplicit(Box::new(self)),
            decided: Vec::new(),
            components,
            nodes: Vec::new(),
            lever,
        }
    }
}

/// What stage 1 decides of a pose it does not refuse.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Hull {
    /// The net lies strictly on one side of the plane.
    Apart,
    /// The net reaches past the band of the plane.
    Cuts,
}

impl<T: Decide> SplineImplicitPose<T> {
    /// **Stage 1, the hull**: the wall lies in its net's convex hull (its
    /// weights are positive), so the control points' heights over the
    /// plane bound the wall's. A net whose every height is decided of one
    /// sign meets the plane nowhere; one whose largest height is decided
    /// within the zero band lies on it. The heights are formed in the
    /// scalar's own arithmetic, as every pose decision is.
    ///
    /// # Errors
    ///
    /// [`SectionError::DegenerateOperand`] for a net whose height is
    /// poison, at every scalar and with or without a march door;
    /// [`SectionError::CoincidentSurfaces`] for a net within the zero
    /// band of the plane everywhere, a coincidence its consumer keeps
    /// (D10); and [`SectionError::Escalated`] where its largest height
    /// is in the band.
    fn hull(&self, band: Band) -> Result<Hull, SectionError> {
        let heights: Vec<T> = self.net.control().iter().map(|&c| self.height(c)).collect();
        if heights.iter().any(|h| h.is_poison()) {
            return Err(SectionError::DegenerateOperand {
                what: "the spline wall's net has a control point whose height over the plane \
                       is poison",
            });
        }
        let sides: Vec<_> = heights
            .iter()
            .map(|&h| decide("section_nurbs_plane_hull", Margin::of(h), band).ok())
            .collect();
        let all = |s| !sides.is_empty() && sides.iter().all(|&t| t == Some(s));
        if all(Sign::Positive) || all(Sign::Negative) {
            return Ok(Hull::Apart);
        }
        let reach = heights.iter().fold(T::zero(), |m, &h| m.max(h.abs()));
        match decide_magnitude("section_nurbs_plane_coincident", Margin::of(reach), band) {
            Ok(Magnitude::Zero) => Err(SectionError::CoincidentSurfaces),
            Ok(Magnitude::Positive) => Ok(Hull::Cuts),
            Err(diag) => Err(SectionError::Escalated(diag)),
        }
    }

    /// **The door's collapsed sides**: a side of the wall's rectangle
    /// collapsed to a point
    /// ([`crate::chart_singularity::spline_collapsed_row`]) is a pole,
    /// and one whose row is not decided clear of the plane, every point
    /// of it on one side, is refused before any sweep, named.
    ///
    /// # Errors
    ///
    /// [`SectionError::Unaccounted`], naming the side.
    fn door(&self, band: Band) -> Result<(), SectionError> {
        let control = self.net.control();
        for side in crate::ssi::boundary::SIDES {
            let Some(row) = crate::chart_singularity::spline_collapsed_row(&self.net, side, band)
            else {
                continue;
            };
            let signs: Vec<_> = row
                .iter()
                .filter_map(|&i| control.get(i))
                .map(|&p| decide("section_nurbs_plane_pole", Margin::of(self.height(p)), band).ok())
                .collect();
            let one = |s| !signs.is_empty() && signs.iter().all(|&t| t == Some(s));
            if !(one(Sign::Positive) || one(Sign::Negative)) {
                return Err(SectionError::Unaccounted {
                    cause: format!(
                        "the wall's {side} collapses to a point not decided clear of the plane"
                    ),
                });
            }
        }
        Ok(())
    }

    /// The height of `p` over the plane, in metres.
    fn height(&self, p: Point3<T>) -> T {
        let Surface::Plane { origin, normal, .. } = self.partner else {
            unreachable!("the pose holds a plane")
        };
        (p - origin).dot(normal / normal.norm())
    }
}

impl SplineImplicitPose<f64> {
    /// **The plane read at its pivot**: the same carrier, its origin
    /// moved to the foot on it of the wall's ball's centre, so the
    /// reading's window stands where the wall is wherever the plane's
    /// stored origin lies (as an angular datum is read at its pivot,
    /// `section_cert`'s lever and pivot). The foot is found in `f64`,
    /// so the plane read stands off the stored one by up to
    /// [`Pivot::shift`], which the reading's band and the witness's
    /// bracket are charged.
    #[must_use]
    pub fn anchored(&self) -> Surface<f64> {
        self.pivot().plane
    }

    /// The plane read at its pivot, with what moving its origin cost.
    fn pivot(&self) -> Pivot {
        let Surface::Plane {
            origin,
            normal,
            u_ref,
        } = self.partner
        else {
            unreachable!("the pose holds a plane")
        };
        let n = normal / normal.norm();
        let c = self.ball().center();
        let foot = c - n * (c - origin).dot(n);
        let (a, o, m) = (
            foot.map(Interval::from_certified),
            origin.map(Interval::from_certified),
            normal.map(Interval::from_certified),
        );
        let lift = (a.x - o.x) * m.x + (a.y - o.y) * m.y + (a.z - o.z) * m.z;
        let norm = (m.x.powi(2) + m.y.powi(2) + m.z.powi(2)).sqrt();
        Pivot {
            plane: Surface::Plane {
                origin: foot,
                normal,
                u_ref,
            },
            heights: Heights {
                origin: foot,
                normal,
                lift,
            },
            shift: (lift.abs() / norm).hi(),
        }
    }

    /// **The reading's domain**: against a plane, the window of the plane
    /// read at its pivot ([`Self::anchored`]), reaching twice the farthest
    /// control point from the pivot plus the feature extent, so it holds
    /// the wall and a march ends only on the wall's rectangle; against a
    /// curved partner, the cube about the net's ball. The feature extent
    /// is the net ball's diameter ([`super::domain_about`]).
    #[must_use]
    pub fn domain(&self) -> SsiDomain {
        if !self.is_plane() {
            let ball = self.ball();
            return super::domain_about(ball.center(), ball.radius() * 1.01, ball.radius() * 2.0);
        }
        let Surface::Plane { origin, .. } = self.anchored() else {
            unreachable!("the pose holds a plane")
        };
        let reach = self
            .net()
            .control()
            .iter()
            .map(|p| p.distance(origin))
            .fold(0.0, f64::max);
        let extent = self.ball().radius() * 2.0;
        super::domain_about(origin, reach * 2.0 + extent, extent)
    }
}

/// **The plane read at its pivot** ([`SplineImplicitPose::anchored`]).
struct Pivot {
    /// The plane, its origin at the foot.
    plane: Surface<f64>,
    /// The stored plane's heights, read from the foot.
    heights: Heights,
    /// How far the plane read may stand off the stored one, in metres:
    /// the foot's certified height over the stored plane. Every distance
    /// to the plane moves by at most this, so the reading's band is
    /// padded by it.
    shift: f64,
}

impl Pivot {
    /// The plane read at its pivot, as the sweep reads it.
    fn phi(&self) -> sweep::Phi<'static> {
        let Surface::Plane { origin, normal, .. } = self.plane else {
            unreachable!("the pivot holds a plane")
        };
        sweep::Phi::Plane(origin, normal)
    }

    /// The reading's band: `band` padded by the shift.
    ///
    /// # Errors
    ///
    /// [`SectionError::Unaccounted`] where the shift pads no band.
    fn band(&self, band: Band) -> Result<Band, SectionError> {
        band.padded(self.shift)
            .map_err(|e| SectionError::Unaccounted {
                cause: format!(
                    "the plane read at its pivot stands {:e} m off the stored one, which pads \
                     no band: {e}",
                    self.shift
                ),
            })
    }
}

/// **The stored plane's height over a point**, read from the foot:
/// `(S − origin)·normal + lift`, where `origin` is the foot and `lift`
/// the foot's certified height over the stored plane, so the height is
/// the stored plane's however far its origin lies.
#[derive(Clone, Copy)]
struct Heights {
    /// The foot.
    origin: Point3<f64>,
    /// The plane's normal, as stored.
    normal: Vec3<f64>,
    /// The foot's height over the stored plane, `(foot − origin)·normal`.
    lift: Interval,
}

impl Heights {
    /// The height over `s`, an enclosure of a point.
    fn over(self, s: Box3) -> Interval {
        let (o, m) = (
            self.origin.map(Interval::from_certified),
            self.normal.map(Interval::from_certified),
        );
        (s.x - o.x) * m.x + (s.y - o.y) * m.y + (s.z - o.z) * m.z + self.lift
    }
}

/// **Why a plane × spline component has no carrier to carry**: the
/// section is read whole for the section certificate alone, the glue
/// door's table never holds it, and no seat certifies against a wall's
/// chart.
pub(super) const UNCARRIED: &str = "a plane × spline section is read whole for the section \
    certificate alone, which carries none of its components";

/// The note a region's component carries.
const REGION: &str = "a Side or Corner region: the section runs along a side of the wall's \
                      rectangle, or leaves it at a corner, within the band, and no carrier is \
                      minted for it";

/// **A branch's witness and how far it may stand from its arc**: the
/// wall's point `q = S(P(mid))` at the middle of the branch's pcurve on
/// the wall, bracketed on the section within the tube window of the
/// branch's certificate that holds `P(mid)` ([`bracket`]). `None` where
/// the certificate is not a chart tube, the branch carries no wall
/// pcurve, or no bracket fits its window: a component with no witness.
fn witness(b: &SsiBranch, net: &NurbsSurface<f64>, heights: Heights) -> Option<(Point3<f64>, f64)> {
    let (SsiTube::Chart { pad_u, pad_v, .. }, Some(pcurve)) =
        (b.certificate.tube, b.pcurve_b.as_ref())
    else {
        return None;
    };
    let mid = 0.5 * (b.params.0 + b.params.1);
    let window = crate::ssi::certify::chart_tube_windows(pcurve, (pad_u, pad_v))?
        .into_iter()
        .find(|w| w.ends.0 <= mid && mid <= w.ends.1)?;
    let at = pcurve.eval(mid);
    bracket(net, heights, (at.x, at.y), window.rect)
}

/// The most doublings [`bracket`] takes. It is not what ends a search:
/// each doubling's ends are checked inside the window and the wall's
/// rectangle first, and a segment that leaves either is no bracket, so
/// the doublings stop once the segment outgrows the window. The cap
/// only bounds the loop should a step stay finite and inside forever.
const BRACKET_DOUBLINGS: u32 = 64;

/// The most halvings [`bracket`] takes to close in on the zero: an
/// `f64` parameter has no more bits to halve.
const BRACKET_HALVINGS: u32 = 64;

/// **The wall's point at `at`, bracketed on the section inside `window`**:
/// `q`, an enclosure's centre of `S(at)`, and a metre bound on its
/// distance from a point of the section on the wall, or `None`.
///
/// The segment through `at` along the plane height's chart gradient
/// is doubled from the height's own step until the stored plane's
/// height ([`Heights`]) is certified negative at one end and positive
/// at the other (the wall's certified box at a point), then halved
/// about the sign change for as long as the height's sign at the middle
/// is certified: a zero of the height, a point of the section, lies
/// between the ends kept. Its distance from `S(at)` is at most the image
/// of the chart segment from `at` to it, bounded by that segment's
/// chart extents times the wall's certified chart speeds over the box
/// spanning `at` and the ends kept; `q` stands within its enclosure's
/// half-widths of `S(at)`, which the bound adds. The segment must lie in
/// `window`, a window of the branch's certified chart tube, where limb
/// 3 proves the section one arc: so the zero is the branch's. A segment
/// that leaves the window or the wall's rectangle, or a gradient that
/// is not a direction, is no bracket.
fn bracket(
    net: &NurbsSurface<f64>,
    heights: Heights,
    at: (f64, f64),
    window: UvRect,
) -> Option<(Point3<f64>, f64)> {
    let jet = net.ders(at.0, at.1);
    let m = heights.normal;
    let (gu, gv) = (m.dot(jet.du), m.dot(jet.dv));
    let g = gu.hypot(gv);
    if !(g.is_finite() && g > 0.0) {
        return None;
    }
    let (du, dv) = (gu / g, gv / g);
    let point = |t: f64| (at.0 + du * t, at.1 + dv * t);
    let phi = (jet.point - heights.origin).dot(m);
    let base = (phi.abs() + heights.lift.abs().hi())
        .max(f64::EPSILON * (jet.point - heights.origin).norm() * m.norm())
        / g;
    let (u_domain, v_domain) = (net.knot_values_u().domain(), net.knot_values_v().domain());
    let inside = |(u, v): (f64, f64)| {
        window.holds((u, v))
            && u_domain.0 <= u
            && u <= u_domain.1
            && v_domain.0 <= v
            && v <= v_domain.1
    };
    let boxes = NurbsBoxes::new(net);
    let at_box = |(u, v): (f64, f64)| UvWindow::new(u, u, v, v).map(|w| boxes.rect_box(w));
    // The height's certified sign at `t`: `Some(true)` above the plane,
    // `Some(false)` below, `None` undecided.
    let sign = |t: f64| {
        let h = heights.over(at_box(point(t))?);
        if !h.is_certified() {
            None
        } else if h.lo() > 0.0 {
            Some(true)
        } else if h.hi() < 0.0 {
            Some(false)
        } else {
            None
        }
    };
    let mut step = base;
    let (mut low, mut high) = (0.0, 0.0);
    for _ in 0..=BRACKET_DOUBLINGS {
        step *= 2.0;
        if !(inside(point(-step)) && inside(point(step))) {
            return None;
        }
        if sign(-step) == Some(false) && sign(step) == Some(true) {
            (low, high) = (-step, step);
            break;
        }
    }
    if low == high {
        return None;
    }
    for _ in 0..BRACKET_HALVINGS {
        let middle = 0.5 * (low + high);
        if middle <= low || middle >= high {
            break;
        }
        match sign(middle) {
            Some(false) => low = middle,
            Some(true) => high = middle,
            None => break,
        }
    }
    let (near, far) = (point(low.min(0.0)), point(high.max(0.0)));
    let span = UvWindow::new(
        near.0.min(far.0),
        near.0.max(far.0),
        near.1.min(far.1),
        near.1.max(far.1),
    )?;
    let reach = (-low).max(high).max(0.0)
        * (du.abs() * boxes.speed_sup(span, true) + dv.abs() * boxes.speed_sup(span, false));
    let (q, half) = at_box(at)?.centre_and_reach()?;
    let pad = (reach + half).next_up();
    pad.is_finite().then_some((q, pad))
}

/// **The pair's classification before any lane is read**: against a
/// plane, stage 1, the hull, and the door's collapsed sides, at every
/// scalar; past them, and for a curved partner from the hull on, the
/// march door's arm ([`section`]).
///
/// # Errors
///
/// [`SectionError::WrongLane`] for a pair that is not an implicit
/// surface and a spline; the plane's hull's and door's;
/// [`SectionError::NoMarch`] for a pose they do not decide, asked with no
/// `lane`; and [`section`]'s.
pub(super) fn classified<T: Decide>(
    a: &Surface<T>,
    b: &Surface<T>,
    lane: Option<SsiLane<T>>,
    band: Band,
) -> Result<PairSection<T>, SectionError> {
    let pose = SplineImplicitPose::served(a, b)?;
    if pose.is_plane() {
        if pose.hull(band)? == Hull::Apart {
            return Ok(pose.section(Components::Counted(Vec::new())));
        }
        pose.door(band)?;
    }
    let lane = lane.ok_or(SectionError::NoMarch { scalar: T::NAME })?;
    (lane.spline)(pose, band)
}

/// **The pair's classification, the march door's arm**: for a curved
/// partner, stages 1 to 3 ([`curved`]); for a plane past its door, stage
/// 2, the boundary pass ([`crate::PlaneSection`]'s, run once), whose
/// grazes and side tangencies refuse by name, then stage 3, the
/// regularity sweep ([`sweep`]). Its loop-free verdict is the section:
/// no component where the pass found no end, else
/// [`Components::Reaching`] on the sides the pass's crossings and
/// regions lie on, the pass carried on the pose. Where the sweep
/// declines, the plane's reading lists the section whole on the same
/// pass.
///
/// # Errors
///
/// The pass's refusal and the whole reading's, mapped (module docs), and
/// [`curved::section`]'s.
pub(super) fn section(
    mut pose: SplineImplicitPose<f64>,
    band: Band,
) -> Result<PairSection<f64>, SectionError> {
    if !pose.is_plane() {
        return curved::section(pose, band);
    }
    let pivot = pose.pivot();
    let read_band = pivot.band(band)?;
    let reading = crate::PlaneSection::of(&pivot.plane, pose.net(), pose.domain(), read_band)
        .map_err(super::whole_reading_refused)?;
    let (reach, pass) = {
        let pass = reading.pass().map_err(super::whole_reading_refused)?;
        let reach = pass
            .crossings
            .iter()
            .map(|c| ChartReach::side(c.at.side))
            .chain(pass.contacts.iter().map(contact_reach))
            .fold(ChartReach::default(), ChartReach::or);
        (reach, std::sync::Arc::new(pass.clone()))
    };
    if let sweep::Sweep::Declined(_) = sweep::sweep(
        reading.boxes(),
        pivot.phi(),
        reading.account_floor(),
        read_band,
        pose.domain().extent,
    ) {
        let out = reading.all().map_err(super::whole_reading_refused)?;
        return listed(pose, &pivot, out);
    }
    drop(reading);
    let Some(reached) = super::Reached::new(pose.order(ChartReach::default(), reach)) else {
        return Ok(pose.section(Components::Counted(Vec::new())));
    };
    pose.pass = Some(pass);
    Ok(pose.section(Components::Reaching(reached)))
}

/// **The section read whole** (module docs): C3's reading, every
/// component listed, on the boundary pass the classification ran where
/// it carried one.
///
/// # Errors
///
/// The whole reading's refusal, mapped (module docs), and
/// [`SectionError::Unaccounted`] for a branch that ends off the wall's
/// rectangle, which the plane lane's reading never marches; a curved
/// partner's whole reading is not built
/// ([`SectionError::RoutesToGeneralRung`]).
pub(super) fn whole(
    pose: &SplineImplicitPose<f64>,
    band: Band,
) -> Result<PairSection<f64>, SectionError> {
    if !pose.is_plane() {
        return Err(SectionError::RoutesToGeneralRung {
            pair: "a NURBS wall and a cylinder, sphere or torus",
            why: "the whole reading over the wall's knot rectangle is not built for a curved \
                  partner",
        });
    }
    let pivot = pose.pivot();
    let read_band = pivot.band(band)?;
    let reading = crate::PlaneSection::of(&pivot.plane, pose.net(), pose.domain(), read_band)
        .map_err(super::whole_reading_refused)?;
    let reading = match &pose.pass {
        Some(pass) => reading.with_pass(BoundaryPass::clone(pass)),
        None => reading,
    };
    listed(
        pose.clone(),
        &pivot,
        reading.all().map_err(super::whole_reading_refused)?,
    )
}

/// The sides a boundary region reaches.
fn contact_reach(region: &SsiBoundaryContact) -> ChartReach {
    match *region {
        SsiBoundaryContact::Side { side, .. } => ChartReach::side(side),
        SsiBoundaryContact::Corner { corner, .. } => ChartReach::corner(corner),
    }
}

/// **The whole reading's outcome as the section's components** (module
/// docs).
///
/// # Errors
///
/// [`SectionError::Unaccounted`] for a branch that ends off the wall's
/// rectangle.
fn listed(
    pose: SplineImplicitPose<f64>,
    pivot: &Pivot,
    out: crate::ssi::SsiOutcome,
) -> Result<PairSection<f64>, SectionError> {
    let open = |on_wall: ChartReach| ComponentShape::Open {
        ends: ComponentEnds {
            rims: pose.order(ChartReach::default(), on_wall),
            ..ComponentEnds::default()
        },
    };
    let mut components = Vec::with_capacity(out.branches.len() + out.boundary.len());
    for b in out.branches {
        let shape = match b.end {
            BranchEnd::Closed => ComponentShape::Loop,
            BranchEnd::Rim { from, to } => {
                open(ChartReach::side(from.side).or(ChartReach::side(to.side)))
            }
            BranchEnd::Cut => {
                return Err(SectionError::Unaccounted {
                    cause: "a branch of the plane × spline reading ended off the wall's \
                            rectangle, where no side of the wall's chart holds it"
                        .to_owned(),
                });
            }
        };
        let witness = witness(&b, pose.net(), pivot.heights);
        components.push(SectionComponent {
            shape,
            winds: [false, false],
            witness: witness.map(|(q, _)| q),
            pad: witness.map_or(0.0, |(_, pad)| pad),
            carrier: ClassCarrier::Unminted(UNCARRIED),
        });
    }
    components.extend(out.boundary.iter().map(|region| SectionComponent {
        shape: open(contact_reach(region)),
        winds: [false, false],
        witness: None,
        pad: 0.0,
        carrier: ClassCarrier::Unminted(REGION),
    }));
    Ok(pose.section(Components::Counted(components)))
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use geom::NurbsSurface;
    use geom_core::spline::KnotVector;
    use geom_core::{Point3, Vec3};

    use geom_core::Interval;

    use super::{Heights, bracket};
    use crate::ssi::exhaust::UvRect;

    /// The plane through `origin` with `normal`, stored where it is read.
    fn plane((origin, normal): (Point3<f64>, Vec3<f64>)) -> Heights {
        Heights {
            origin,
            normal,
            lift: Interval::from_certified(0.0),
        }
    }

    /// The ramp `z = x/2` over `u ∈ [0, 1]`, `y = v ∈ [0, 1]`: its `u`
    /// row runs `x` 0, 0.25, 0.5 over the first knot span and leaps to
    /// `x = 100` over the second, so the net's chart speed is some four
    /// hundred times the first span's.
    fn stretched_ramp() -> NurbsSurface<f64> {
        let ku = KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.5, 1.0, 1.0, 1.0], 2).unwrap();
        let kv = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
        let control = [0.0, 0.25, 0.5, 100.0]
            .iter()
            .flat_map(|&x| [Point3::new(x, 0.0, 0.5 * x), Point3::new(x, 1.0, 0.5 * x)])
            .collect();
        NurbsSurface::new(ku, kv, control, vec![1.0; 8]).unwrap()
    }

    /// The whole chart.
    fn everywhere() -> UvRect {
        UvRect {
            u: (0.0, 1.0),
            v: (0.0, 1.0),
        }
    }

    /// The plane `dz` below the ramp's point at `(0.25, ½)`.
    fn below(dz: f64) -> (Point3<f64>, Vec3<f64>) {
        let q = stretched_ramp().eval(0.25, 0.5);
        (Point3::new(0.0, 0.0, q.z - dz), Vec3::new(0.0, 0.0, 1.0))
    }

    /// **The bracket's reach is the segment's, read over its own box,
    /// and closed in on the zero.** The plane `10⁻⁶` below the wall's
    /// point meets the ramp `2·10⁻⁶` back in `x`, `√1.25·2·10⁻⁶` from it
    /// along the wall: the pad bounds that distance and stays within a
    /// quarter over it. Red against the chart speed read over the whole
    /// net (some four hundred times the first span's), against the pad
    /// halved, and against the reach dropped.
    #[test]
    fn the_brackets_reach_is_the_segments_over_its_own_box() {
        let dz = 1e-6;
        let (q, pad) = bracket(
            &stretched_ramp(),
            plane(below(dz)),
            (0.25, 0.5),
            everywhere(),
        )
        .expect("the plane crosses the wall beside the point");
        let off = 2.0 * dz * 1.25_f64.sqrt();
        assert!(
            q.distance(stretched_ramp().eval(0.25, 0.5)) <= 1e-15,
            "the witness is the wall's point: {q:?}"
        );
        assert!(
            off <= pad && pad <= 1.25 * off,
            "the pad {pad:e} against the distance {off:e}"
        );
    }

    /// The ramp `z = x/2` with `x = u²` over `u ∈ [0, 1]`, `y = v`: its
    /// chart speed in `u` grows with `u`.
    fn quickening_ramp() -> NurbsSurface<f64> {
        let ku = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
        let kv = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
        let control = [0.0, 0.0, 1.0]
            .iter()
            .flat_map(|&x| [Point3::new(x, 0.0, 0.5 * x), Point3::new(x, 1.0, 0.5 * x)])
            .collect();
        NurbsSurface::new(ku, kv, control, vec![1.0; 6]).unwrap()
    }

    /// **The bracket reads the wall's speed over its segment, not at its
    /// point.** On the quickening ramp the wall's point at `u = 0.3`
    /// (`x = 0.09`) and the plane `z = 0.08` (`x = 0.16`, `u = 0.4`):
    /// the section stands `0.07·√1.25` along the wall, and the chart
    /// speed at the point, `0.6·√1.25`, carries the `0.1` of chart to
    /// only `0.06·√1.25`. Red against the speeds read at the point.
    #[test]
    fn the_brackets_speed_is_the_segments_not_the_points() {
        let at = Point3::new(0.0, 0.0, 0.08);
        let (_, pad) = bracket(
            &quickening_ramp(),
            plane((at, Vec3::new(0.0, 0.0, 1.0))),
            (0.3, 0.5),
            everywhere(),
        )
        .expect("the plane crosses the wall beside the point");
        let off = 0.07 * 1.25_f64.sqrt();
        assert!(
            off <= pad && pad <= 1.25 * off,
            "the pad {pad:e} against the distance {off:e}"
        );
    }

    /// **The stored plane's height is read through the lift.** The same
    /// cut as the quickening ramp's, read from a foot `0.01` below the
    /// plane, its lift `−0.01`: the pad is the stored plane's. Red
    /// against the lift dropped (the foot's plane, at `x = 0.14`, is
    /// nearer).
    #[test]
    fn the_bracket_reads_the_stored_plane_through_its_lift() {
        let foot = Heights {
            origin: Point3::new(0.0, 0.0, 0.07),
            normal: Vec3::new(0.0, 0.0, 1.0),
            lift: Interval::from_certified(-0.01),
        };
        let (_, pad) = bracket(&quickening_ramp(), foot, (0.3, 0.5), everywhere())
            .expect("the plane crosses the wall beside the point");
        let off = 0.07 * 1.25_f64.sqrt();
        assert!(
            off <= pad && pad <= 1.25 * off,
            "the pad {pad:e} against the distance {off:e}"
        );
    }

    /// **A bracket that leaves its tube window is none.** The same cut,
    /// its window a hair round the point: the segment the bracket needs
    /// leaves it, where the tube proves nothing of the section, so no
    /// witness is had. Red against the window's containment dropped.
    #[test]
    fn a_bracket_that_leaves_its_window_is_none() {
        let hair = UvRect {
            u: (0.25 - 1e-9, 0.25 + 1e-9),
            v: (0.5 - 1e-9, 0.5 + 1e-9),
        };
        assert!(bracket(&stretched_ramp(), plane(below(1e-6)), (0.25, 0.5), hair).is_none());
    }

    /// The dome `z = −(x − ½)² − (y − ½)²` over `[0, 1]²`, its top `z = 0`
    /// at the centre.
    fn dome() -> NurbsSurface<f64> {
        let k = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
        let g = [-0.25, 0.25, -0.25];
        let control = (0..9)
            .map(|n| {
                let (i, j) = (n / 3, n % 3);
                #[allow(clippy::cast_precision_loss)]
                Point3::new(0.5 * i as f64, 0.5 * j as f64, g[i] + g[j])
            })
            .collect();
        NurbsSurface::new(k.clone(), k, control, vec![1.0; 9]).unwrap()
    }

    /// **A height that never changes sign is no bracket.** A plane just
    /// above the dome's top meets it nowhere: from a point beside the top
    /// the segment doubles inside the chart with its ends below the plane
    /// at every step, until it leaves the chart. Red against the sign
    /// check dropped (the first segment is taken).
    #[test]
    fn a_height_of_one_sign_is_no_bracket() {
        let above = (Point3::new(0.5, 0.5, 1e-3), Vec3::new(0.0, 0.0, 1.0));
        assert!(bracket(&dome(), plane(above), (0.45, 0.5), everywhere()).is_none());
    }
}
