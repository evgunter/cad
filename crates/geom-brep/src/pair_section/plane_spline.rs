//! **Plane × spline: the section value over C3's whole reading.** The
//! section of a plane and a NURBS wall (an approximating surface read
//! as its fit) is [`crate::PlaneSection::all`] over the wall's whole
//! knot rectangle: every branch with its three-limb certificate, every
//! boundary region, and the accounting that they are all of them. The
//! plane is read at its pivot, its origin moved to its foot nearest the
//! wall ([`PlaneSplinePose::anchored`]), so where its origin is stored
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
use geom_core::{Band, Bounds, Interval, Point3, Real, Vec3};

use super::{
    ChartReach, ClassCarrier, ComponentShape, Components, PairPose, PairSection, SectionComponent,
};
use crate::ExtentBall;
use crate::intersect::SectionError;
use crate::ssi::enclose::{Box3, NurbsBoxes, UvWindow};
use crate::ssi::exhaust::UvRect;
use crate::ssi::{BranchEnd, SsiBoundaryContact, SsiBranch, SsiDomain, SsiError, SsiTube};

/// **A plane and a spline wall**, in the order the section was asked
/// of.
#[derive(Clone, Debug)]
pub struct PlaneSplinePose<T: Real> {
    plane: Surface<T>,
    wall: Surface<T>,
    /// The wall's net: a NURBS wall's own, an approximating surface's
    /// fit's.
    net: geom::NurbsSurface<T>,
    plane_first: bool,
}

impl<T: Real> PlaneSplinePose<T> {
    /// The pose of `a` and `b`, one a plane and the other a NURBS or an
    /// approximating surface; `None` for any other pair.
    #[must_use]
    pub fn of(a: &Surface<T>, b: &Surface<T>) -> Option<Self> {
        let (plane, wall, plane_first) = match (a, b) {
            (Surface::Plane { .. }, w) => (a, w, true),
            (w, Surface::Plane { .. }) => (b, w, false),
            _ => return None,
        };
        Some(Self {
            plane: plane.clone(),
            wall: wall.clone(),
            net: wall.spline_chart()?.clone(),
            plane_first,
        })
    }

    /// The two surfaces in the order the pose was asked of.
    pub fn operands(&self) -> (&Surface<T>, &Surface<T>) {
        if self.plane_first {
            (&self.plane, &self.wall)
        } else {
            (&self.wall, &self.plane)
        }
    }

    /// The plane.
    pub fn plane(&self) -> &Surface<T> {
        &self.plane
    }

    /// The wall's net: a NURBS wall's own, an approximating surface's
    /// fit's.
    pub fn net(&self) -> &geom::NurbsSurface<T> {
        &self.net
    }

    /// `[on the plane, on the wall]` in the order the pose was asked of.
    pub(super) fn order<V>(&self, on_plane: V, on_wall: V) -> [V; 2] {
        if self.plane_first {
            [on_plane, on_wall]
        } else {
            [on_wall, on_plane]
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

impl PlaneSplinePose<f64> {
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
        } = self.plane
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

    /// **The reading's domain**: the window of the plane read at its
    /// pivot ([`Self::anchored`]), reaching twice the farthest control
    /// point from the pivot plus the feature extent, so it holds the wall
    /// and a march ends only on the wall's rectangle; the feature extent
    /// the net ball's diameter ([`super::domain_about`]).
    #[must_use]
    pub fn domain(&self) -> SsiDomain {
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

/// **The plane read at its pivot** ([`PlaneSplinePose::anchored`]).
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
    let s = at_box(at)?;
    let centre = |x: Interval| 0.5 * (x.lo() + x.hi());
    let q = Point3::new(centre(s.x), centre(s.y), centre(s.z));
    let half = |x: Interval| (x.hi() - centre(x)).max(centre(x) - x.lo()).next_up();
    let pad = (reach + half(s.x) + half(s.y) + half(s.z)).next_up();
    pad.is_finite().then_some((q, pad))
}

/// The whole reading's refusal as the section value's (module docs): a
/// tangency the reading reports ([`SsiError::tangency`]) is a pinch named
/// by its decision, an escalation is the value's escalation, as the
/// analytic arms' in-band readings are, and any other refusal is
/// [`SectionError::Unaccounted`].
fn refused(e: SsiError) -> SectionError {
    if let Some(predicate) = e.tangency() {
        return SectionError::Pinched { predicate };
    }
    match e {
        SsiError::Escalated { cause, .. } | SsiError::CertificateEscalated { cause, .. } => {
            SectionError::Escalated(cause)
        }
        other => SectionError::Unaccounted {
            cause: other.to_string(),
        },
    }
}

/// **The section of a plane and a spline wall** (module docs), the
/// march door's arm.
///
/// # Errors
///
/// [`SectionError::WrongLane`] for a pair that is not a plane and a
/// spline; the whole reading's refusal, mapped (module docs); and
/// [`SectionError::Unaccounted`] for a branch that ends off the wall's
/// rectangle, which the plane lane's reading never marches.
pub(super) fn section(
    a: &Surface<f64>,
    b: &Surface<f64>,
    band: Band,
) -> Result<PairSection<f64>, SectionError> {
    let pose = PlaneSplinePose::of(a, b).ok_or(SectionError::WrongLane {
        expected: "a plane and a NURBS or approximating surface",
    })?;
    let pivot = pose.pivot();
    let read_band = band
        .padded(pivot.shift)
        .map_err(|e| SectionError::Unaccounted {
            cause: format!(
                "the plane read at its pivot stands {:e} m off the stored one, which pads no \
                 band: {e}",
                pivot.shift
            ),
        })?;
    let out = crate::ssi::plane_nurbs_ssi(&pivot.plane, pose.net(), pose.domain(), read_band)
        .map_err(refused)?;
    let open = |on_wall: ChartReach| ComponentShape::Open {
        reaches: pose.order(ChartReach::default(), on_wall),
    };
    let mut components = Vec::with_capacity(out.branches.len() + out.boundary.len());
    for b in out.branches {
        let shape = match b.end {
            BranchEnd::Closed => ComponentShape::Loop,
            BranchEnd::Crossings { from, to } => {
                open(ChartReach::side(from.side).or(ChartReach::side(to.side)))
            }
            BranchEnd::Slab | BranchEnd::SlabInBand => {
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
    components.extend(out.boundary.iter().map(|region| {
        let reach = match *region {
            SsiBoundaryContact::Side { side, .. } => ChartReach::side(side),
            SsiBoundaryContact::Corner { corner, .. } => ChartReach::corner(corner),
        };
        SectionComponent {
            shape: open(reach),
            winds: [false, false],
            witness: None,
            pad: 0.0,
            carrier: ClassCarrier::Unminted(REGION),
        }
    }));
    let lever = pose.ball().radius();
    Ok(PairSection {
        pose: PairPose::PlaneSpline(Box::new(pose)),
        decided: Vec::new(),
        components: Components::Counted(components),
        lever,
    })
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
