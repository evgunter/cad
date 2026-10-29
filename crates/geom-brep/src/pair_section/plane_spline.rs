//! **Plane × spline: the section value over C3's whole reading.** The
//! section of a plane and a NURBS wall (an approximating surface read
//! as its fit) is [`crate::PlaneSection::all`] over the wall's whole
//! knot rectangle: every branch with its three-limb certificate, every
//! boundary region, and the accounting that they are all of them. The
//! plane is read at its pivot, its origin moved to its foot nearest the
//! wall ([`PlaneSplinePose::anchored`]), so where its origin is stored
//! changes nothing. Each piece of the section is a component:
//!
//! - a closed branch is a [`ComponentShape::Loop`], with its fitted and
//!   certified carrier and its witness;
//! - an open branch, which runs between two crossings of the wall's
//!   rectangle, and a `Side` or `Corner` region, where the section runs
//!   along a side or leaves at a corner within the band, are
//!   [`ComponentShape::Open`], reaching the sides of the wall's chart
//!   they end on ([`ChartReach`]); a branch that ends anywhere else is
//!   refused, never read as reaching.
//!
//! **A branch's witness** is the wall's point `q = S(P(mid))` at the
//! middle of the branch's pcurve on the wall, and its residual a metre
//! bound on how far `q` stands from a point of that branch's own arc:
//! a segment of the chart through `P(mid)` along the plane height's
//! gradient, inside the window of the branch's certified chart tube
//! that holds `P(mid)`, whose ends the height is certified to straddle
//! ([`bracket`]). The segment's image holds a point of the section, and
//! inside the window limb 3 proves the section one arc, the branch's;
//! the image's length is bounded by the segment's chart extents and the
//! wall's certified chart speeds over the segment's own box, so the
//! residual shrinks with the band. A branch no bracket fits carries no
//! witness.
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
    CarriedComponent, ChartReach, ClassCarrier, ComponentShape, PairPose, PairSection,
    SectionComponent, Sides,
};
use crate::ExtentBall;
use crate::intersect::SectionError;
use crate::ssi::enclose::{NurbsBoxes, UvWindow};
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
    /// `section_cert`'s lever and pivot).
    #[must_use]
    pub fn anchored(&self) -> Surface<f64> {
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
        Surface::Plane {
            origin: c - n * (c - origin).dot(n),
            normal,
            u_ref,
        }
    }

    /// **The reading's domain**: the window of the plane read at its
    /// pivot ([`Self::anchored`]), reaching twice the farthest control
    /// point from the pivot plus the feature extent, so it holds the wall
    /// and a march ends only on the wall's rectangle; the feature extent
    /// the net ball's diameter, the standard accounting floor.
    #[must_use]
    pub fn domain(&self) -> SsiDomain {
        let Surface::Plane { origin, .. } = self.anchored() else {
            unreachable!("the pose holds a plane")
        };
        let extent = self.ball().radius() * 2.0;
        let reach = self
            .net()
            .control()
            .iter()
            .map(|p| p.distance(origin))
            .fold(0.0, f64::max);
        SsiDomain {
            center: origin,
            half_extent: reach * 2.0 + extent,
            extent,
            floor_scale: 1.0,
        }
    }
}

/// The note a region's component carries.
const REGION: &str = "a Side or Corner region: the section runs along a side of the wall's \
                      rectangle, or leaves it at a corner, within the band, and no carrier is \
                      minted for it";

/// **A branch's witness and how far it may stand from its arc**: the
/// wall's point `q = S(P(mid))` at the middle of the branch's pcurve on
/// the wall, bracketed on the section within the tube window of the
/// branch's certificate that holds `P(mid)` ([`bracket`]). `(None, 0)`
/// where the certificate is not a chart tube, the branch carries no wall
/// pcurve, or no bracket fits its window: a component with no witness.
fn witness(
    b: &SsiBranch,
    net: &NurbsSurface<f64>,
    plane: &Surface<f64>,
) -> (Option<Point3<f64>>, f64) {
    let none = (None, 0.0);
    let (SsiTube::Chart { pad_u, pad_v, .. }, Some(pcurve), Surface::Plane { origin, normal, .. }) =
        (b.certificate.tube, b.pcurve_b.as_ref(), plane)
    else {
        return none;
    };
    let mid = 0.5 * (b.params.0 + b.params.1);
    let Some(window) = crate::ssi::certify::chart_tube_windows(pcurve, (pad_u, pad_v))
        .and_then(|ws| ws.into_iter().find(|w| w.ends.0 <= mid && mid <= w.ends.1))
    else {
        return none;
    };
    let at = pcurve.eval(mid);
    bracket(net, (*origin, *normal), (at.x, at.y), window.rect).map_or(none, |(q, r)| (Some(q), r))
}

/// The most doublings [`bracket`] takes before it gives up: past them
/// the segment has left any window a certified tube holds.
const BRACKET_DOUBLINGS: u32 = 64;

/// **The wall's point at `at`, bracketed on the section inside `window`**:
/// `q = S(at)` and a metre bound on its distance from a point of the
/// section on the wall, or `None`.
///
/// The segment through `at` along the plane height's chart gradient
/// is doubled from the height's own step until the height
/// `φ = (S − origin)·normal` is certified negative at one end and
/// positive at the other (the wall's certified box at a point),
/// so the segment's image on the wall holds a zero of `φ`: a point of
/// the section. Its distance from `q` is at most the image's length,
/// bounded by the segment's chart extents times the wall's certified
/// chart speeds over the segment's own box. The segment must lie in
/// `window`, a window of the branch's certified chart tube, where limb
/// 3 proves the section one arc: so the zero is the branch's. A segment
/// that leaves the window or the wall's rectangle, or a gradient that
/// is not a direction, is no bracket.
fn bracket(
    net: &NurbsSurface<f64>,
    (origin, normal): (Point3<f64>, Vec3<f64>),
    at: (f64, f64),
    window: UvRect,
) -> Option<(Point3<f64>, f64)> {
    let jet = net.ders(at.0, at.1);
    let n = normal / normal.norm();
    let (gu, gv) = (n.dot(jet.du), n.dot(jet.dv));
    let g = gu.hypot(gv);
    if !(g.is_finite() && g > 0.0) {
        return None;
    }
    let (du, dv) = (gu / g, gv / g);
    let phi = (jet.point - origin).dot(n);
    let base = phi.abs().max(f64::EPSILON * (jet.point - origin).norm()) / g;
    let (u_domain, v_domain) = (net.knot_values_u().domain(), net.knot_values_v().domain());
    let inside = |(u, v): (f64, f64)| {
        window.holds((u, v))
            && u_domain.0 <= u
            && u <= u_domain.1
            && v_domain.0 <= v
            && v <= v_domain.1
    };
    let boxes = NurbsBoxes::new(net);
    let (o, m) = (
        origin.map(Interval::from_certified),
        normal.map(Interval::from_certified),
    );
    let height = |(u, v): (f64, f64)| {
        let s = UvWindow::new(u, u, v, v).map(|w| boxes.rect_box(w))?;
        Some((s.x - o.x) * m.x + (s.y - o.y) * m.y + (s.z - o.z) * m.z)
    };
    let mut step = base;
    for _ in 0..BRACKET_DOUBLINGS {
        step *= 2.0;
        let (low, high) = (
            (at.0 - du * step, at.1 - dv * step),
            (at.0 + du * step, at.1 + dv * step),
        );
        if !(inside(low) && inside(high)) {
            return None;
        }
        let (Some(below), Some(above)) = (height(low), height(high)) else {
            return None;
        };
        if !(below.is_certified() && above.is_certified() && below.hi() < 0.0 && above.lo() > 0.0) {
            continue;
        }
        let span = UvWindow::new(
            low.0.min(high.0),
            low.0.max(high.0),
            low.1.min(high.1),
            low.1.max(high.1),
        )?;
        let reach = (high.0 - low.0).abs() * boxes.speed_sup(span, true)
            + (high.1 - low.1).abs() * boxes.speed_sup(span, false);
        return reach.is_finite().then_some((jet.point, reach));
    }
    None
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
    let anchored = pose.anchored();
    let out =
        crate::ssi::plane_nurbs_ssi(&anchored, pose.net(), pose.domain(), band).map_err(refused)?;
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
        let (witness, residual) = witness(&b, pose.net(), &anchored);
        components.push(SectionComponent {
            shape,
            winds: [false, false],
            witness,
            residual,
            carrier: ClassCarrier::Fitted(CarriedComponent {
                carrier: b.carrier,
                span: b.params,
                closed: shape == ComponentShape::Loop,
                certificate: Some(b.certificate),
            }),
        });
    }
    components.extend(out.boundary.iter().map(|region| {
        let reach = match *region {
            SsiBoundaryContact::Side { side, .. } => ChartReach::side(side),
            SsiBoundaryContact::Corner { .. } => ChartReach {
                u_side: true,
                v_side: true,
            },
        };
        SectionComponent {
            shape: open(reach),
            winds: [false, false],
            witness: None,
            residual: 0.0,
            carrier: ClassCarrier::Unminted(REGION),
        }
    }));
    let lever = pose.ball().radius();
    Ok(PairSection {
        pose: PairPose::PlaneSpline(Box::new(pose)),
        decided: Vec::new(),
        components,
        sides: Sides::Untold(
            "a spline section's components have no closed-form reading that tells them apart",
        ),
        lever,
    })
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use geom::NurbsSurface;
    use geom_core::spline::KnotVector;
    use geom_core::{Point3, Vec3};

    use super::bracket;
    use crate::ssi::exhaust::UvRect;

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

    /// **The bracket's reach is the segment's, read over its own box.**
    /// The plane `10⁻⁶` below the wall's point meets the ramp `2·10⁻⁶`
    /// back in `x`, about `2.2·10⁻⁶` from it along the wall: the
    /// residual bounds that distance and stays within a few times it. Red
    /// against the chart speed read over the whole net (some four hundred
    /// times the first span's), and against the reach dropped.
    #[test]
    fn the_brackets_reach_is_the_segments_over_its_own_box() {
        let dz = 1e-6;
        let (q, reach) = bracket(&stretched_ramp(), below(dz), (0.25, 0.5), everywhere())
            .expect("the plane crosses the wall beside the point");
        let off = 2.0 * dz * 1.25_f64.sqrt();
        assert!(
            q.distance(stretched_ramp().eval(0.25, 0.5)) == 0.0,
            "the witness is the wall's point: {q:?}"
        );
        assert!(
            off <= reach && reach <= 20.0 * off,
            "the reach {reach:e} against the distance {off:e}"
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
        assert!(bracket(&stretched_ramp(), below(1e-6), (0.25, 0.5), hair).is_none());
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
        assert!(bracket(&dome(), above, (0.45, 0.5), everywhere()).is_none());
    }
}
