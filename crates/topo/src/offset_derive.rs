//! **The per-chart offset door's derivations**: what an edge and a
//! corner become when one chart moves and its neighbours hold.
//!
//! An edge between the moved surface and a distinct held surface is
//! their **section** ([`section_closed`] through the C5 table's closed
//! forms, [`SectionLane`] for a plane against a spline wall), seeded by
//! the old carrier for branch and sense. A moved corner is a **root**
//! of a surface meeting it along an edge meeting it ([`analytic_root`],
//! [`SectionLane`]'s spline root): the corner lies on every surface
//! around it, and each edge there carries all but at most two of them.
//!
//! **Transport is a decided shortcut, not a default.** A rigid image of
//! a moved edge is the section exactly where the held neighbour is
//! carried onto itself by the move ([`holds_the_move`]): a plane cap
//! moved along its normal beside a wall that contains that normal, a
//! cylinder's radius change beside a cap across its axis. There the
//! door transports, at every scalar; everywhere else it derives.

use geom::{Curve3, NurbsCurve3, NurbsSurface, Surface};
use geom_brep::ssi::{BoundarySection, SsiDomain, SsiError};
use geom_core::k_stats::decide;
use geom_core::{Band, Decide, Indeterminate, Margin, Point3, Real, Sign, SupSpeed, Vec3};

/// Why an edge between the moved surface and a held one has no section
/// this door can state.
#[derive(Clone, Debug)]
pub enum SectionVerdict {
    /// A closed-form section arm refused, verbatim.
    Closed(geom_brep::SectionError),
    /// The plane × spline-wall march refused, verbatim.
    Ssi(SsiError),
    /// The section has no branch near the edge: the moved surface no
    /// longer meets the neighbour there.
    NoBranch,
    /// The section near the edge is a tangency, which is classification
    /// data rather than a curve an edge can carry.
    Tangent,
    /// The door does not derive this section, in its own words.
    Unsupported {
        /// What the door could not do.
        what: &'static str,
    },
}

impl core::fmt::Display for SectionVerdict {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Closed(e) => write!(f, "the closed-form section refused: {e}"),
            Self::Ssi(e) => write!(f, "the spline-wall section refused: {e}"),
            Self::NoBranch => f.write_str("the moved surface no longer meets the neighbour there"),
            Self::Tangent => f.write_str("the moved surface only touches the neighbour there"),
            Self::Unsupported { what } => f.write_str(what),
        }
    }
}

/// Why a moved corner is not a root of a surface along an edge.
#[derive(Clone, Debug)]
pub enum CornerVerdict<T: Real> {
    /// The spline root's section refused, verbatim — a graze among
    /// them ([`SsiError::BoundaryGraze`]).
    Ssi(SsiError),
    /// The surface meets the edge's carrier at a vanishing angle: the
    /// sine between the carrier and the surface is decided zero.
    Graze {
        /// The sine at the root, dimensionless.
        sine: T,
    },
    /// The surface does not meet the carrier near the corner.
    NoRoot,
    /// The door does not solve this root, in its own words.
    Unsupported {
        /// What the door could not do.
        what: &'static str,
    },
}

impl<T: Real> core::fmt::Display for CornerVerdict<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Ssi(e) => write!(f, "the corner's section refused: {e}"),
            Self::Graze { sine } => write!(
                f,
                "the moved surface grazes the edge (the sine between them is {sine:?})"
            ),
            Self::NoRoot => f.write_str("the moved surface does not meet the edge near the corner"),
            Self::Unsupported { what } => f.write_str(what),
        }
    }
}

/// A spline root's answer: the parameter, or where the carrier stops
/// short of the surface.
#[derive(Clone, Copy, Debug)]
pub enum SplineRoot {
    /// The surface meets the carrier at this parameter.
    At(f64),
    /// No root on the carrier's domain. `near_gap` is the distance from
    /// the carrier's end nearer the seed to the surface, and `near` is
    /// whether that end is the closer of the two to it — the surface
    /// lies past the near end — or the far end is, the move having
    /// passed through the whole edge.
    Short {
        /// The near end's parameter.
        end: f64,
        /// The near end's distance to the surface, in metres.
        near_gap: f64,
        /// The near end is closer to the surface than the far end.
        near: bool,
    },
}

/// The plane × spline-wall section and the plane's root along a spline
/// carrier — the per-chart door's spline lane, at the one scalar its
/// march is written in (`geom_brep::plane_nurbs_ssi` is an `f64`
/// module). Where a scalar holds none
/// ([`crate::AtRestPolicy::section_lane`]), a slanted spline edge
/// refuses by name rather than transporting.
#[derive(Clone, Copy)]
#[allow(clippy::type_complexity)]
pub struct SectionLane<T: Real> {
    section: fn(
        &Surface<T>,
        &NurbsSurface<T>,
        &Curve3<T>,
        (T, T),
        T,
        Band,
    ) -> Result<Result<NurbsCurve3<T>, SectionVerdict>, Indeterminate>,
    root: fn(&Surface<T>, &NurbsCurve3<T>, T, T, Band) -> Result<SplineRoot, CornerVerdict<T>>,
}

impl<T: Real> core::fmt::Debug for SectionLane<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str("SectionLane")
    }
}

impl SectionLane<f64> {
    /// The lane: [`plane_wall_section`] and [`plane_spline_root`].
    #[must_use]
    pub fn f64() -> Self {
        Self {
            section: plane_wall_section,
            root: plane_spline_root,
        }
    }
}

impl<T: Real> SectionLane<T> {
    /// The section of `plane` with the spline `wall` nearest `old` on
    /// `[t0, t1]`, running with it: the wall's exact row where the
    /// section is one, the march's branch otherwise.
    pub(crate) fn section(
        self,
        plane: &Surface<T>,
        wall: &NurbsSurface<T>,
        old: &Curve3<T>,
        span: (T, T),
        extent: T,
        band: Band,
    ) -> Result<Result<NurbsCurve3<T>, SectionVerdict>, Indeterminate> {
        (self.section)(plane, wall, old, span, extent, band)
    }

    /// The root of `plane` along the spline `carrier` nearest `seed`.
    pub(crate) fn root(
        self,
        plane: &Surface<T>,
        carrier: &NurbsCurve3<T>,
        seed: T,
        extent: T,
        band: Band,
    ) -> Result<SplineRoot, CornerVerdict<T>> {
        (self.root)(plane, carrier, seed, extent, band)
    }
}

/// `value` metres decided zero at `band`.
fn zero<T: Decide>(name: &'static str, value: T, band: Band) -> Result<bool, Indeterminate> {
    Ok(matches!(decide(name, Margin::of(value), band)?, Sign::Zero))
}

/// **Whether the held surface is carried onto itself by the move** of
/// `moved` — the decided shortcut under which the moved surface's
/// rigid image of an edge between them is their section, and a moved
/// corner whose every held surface holds is the transported point.
/// Each angle is levered by the move's own length `|d|`: what a wrong
/// shortcut costs is the transported edge standing `|d|·θ` off the held
/// surface, so the shortcut is taken exactly where that is within the
/// band. One lever, so the edge and its corners decide alike.
///
/// A hold is a route, not a claim about the body: where it is not
/// decided (in the band, which a move of about the band's own size puts
/// every levered angle in), the edge takes the section route, which is
/// exact whatever the hold would have said.
///
/// Per kind of move: a plane translates along its normal, held by a
/// plane containing that normal, a cylinder along it, or a spline wall
/// that is a translation surface along it; a cylinder's radius change
/// moves each point radially, held by a plane across the axis or one
/// containing it; a sphere's is a homothety about its centre, held by a
/// plane through the centre; a cone's and a torus's move each point in
/// its meridian plane, held by a plane containing the axis, and the
/// torus's also by its equatorial plane. Nothing else holds.
pub(crate) fn holds_the_move<T: Decide>(
    moved: &Surface<T>,
    held: &Surface<T>,
    d: T,
    band: Band,
) -> bool {
    let zero = |name, value| held_zero(name, value, band);
    let reach = d.abs();
    let across = |a: Vec3<T>, b: Vec3<T>| a.cross(b).norm() * reach;
    let through = |q: Point3<T>, m: Vec3<T>, p: Point3<T>| (q - p).dot(m);
    match (moved, held) {
        (Surface::Plane { normal, .. }, Surface::Plane { normal: m, .. }) => {
            zero("offset_holds_plane_plane", normal.dot(*m) * reach)
        }
        (Surface::Plane { normal, .. }, Surface::Cylinder { axis, .. }) => {
            zero("offset_holds_plane_cylinder", across(*normal, *axis))
        }
        (Surface::Plane { normal, .. }, Surface::Nurbs(wall)) => {
            translates_along(wall, *normal, reach, band)
        }
        (
            Surface::Cylinder { origin, axis, .. },
            Surface::Plane {
                origin: q,
                normal: m,
                ..
            },
        ) => {
            zero("offset_holds_cylinder_across", across(*m, *axis))
                || (zero("offset_holds_cylinder_along", m.dot(*axis) * reach)
                    && zero("offset_holds_cylinder_axis", through(*q, *m, *origin)))
        }
        (
            Surface::Sphere { center, .. },
            Surface::Plane {
                origin: q,
                normal: m,
                ..
            },
        ) => zero("offset_holds_sphere_centre", through(*q, *m, *center)),
        (
            Surface::Cone { apex, axis, .. },
            Surface::Plane {
                origin: q,
                normal: m,
                ..
            },
        ) => {
            zero("offset_holds_cone_along", m.dot(*axis) * reach)
                && zero("offset_holds_cone_axis", through(*q, *m, *apex))
        }
        (
            Surface::Torus { center, axis, .. },
            Surface::Plane {
                origin: q,
                normal: m,
                ..
            },
        ) => {
            zero("offset_holds_torus_centre", through(*q, *m, *center))
                && (zero("offset_holds_torus_along", m.dot(*axis) * reach)
                    || zero("offset_holds_torus_across", across(*m, *axis)))
        }
        _ => false,
    }
}

/// Whether `wall` is a translation surface along `n`:
/// `S(u, v) = C(u) + f(v)·n` (or the same with the parameters
/// exchanged), read off its net — every column's offsets from its first
/// point are one list, parallel to `n`, and the weights do not vary
/// along it.
fn translates_along<T: Decide>(wall: &NurbsSurface<T>, n: Vec3<T>, reach: T, band: Band) -> bool {
    let zero = |name, value| held_zero(name, value, band);
    let (nu, nv) = wall.control_counts();
    let (ctl, w) = (wall.control(), wall.weights());
    // `at(i, j)` reads the net with `j` the axis the translation runs.
    let along = |at: &dyn Fn(usize, usize) -> usize, (ni, nj): (usize, usize)| -> bool {
        for i in 0..ni {
            // `j = 0` is each column's own origin: no step to read.
            for j in 1..nj {
                if w[at(i, j)] != w[at(i, 0)] {
                    return false;
                }
                let step = ctl[at(i, j)] - ctl[at(i, 0)];
                let first = ctl[at(0, j)] - ctl[at(0, 0)];
                // The step's direction off `n`, levered by the move.
                let off = step.cross(n).norm() * reach / step.norm();
                if !zero("offset_holds_wall_step", (step - first).norm())
                    || !zero("offset_holds_wall_direction", off)
                {
                    return false;
                }
            }
        }
        true
    };
    along(&|i, j| i * nv + j, (nu, nv)) || along(&|i, j| j * nv + i, (nv, nu))
}

/// `value` metres decided zero at `band`, for a hold: anything short
/// of a decided zero, an undecided margin included, does not hold
/// ([`holds_the_move`]).
fn held_zero<T: Decide>(name: &'static str, value: T, band: Band) -> bool {
    matches!(decide(name, Margin::of(value), band), Ok(Sign::Zero))
}

/// `candidate` run with `old`, or reversed: the section's sense is the
/// old carrier's at its middle, read where `candidate` passes nearest.
fn running_with<T: Decide>(
    candidate: Curve3<T>,
    old: &Curve3<T>,
    (t0, t1): (T, T),
    band: Band,
) -> Result<Result<Curve3<T>, SectionVerdict>, Indeterminate> {
    let half = (t0 + t1) * T::from_f64(0.5);
    let (mid, tangent) = old.ders1(half);
    let Some(at) = candidate.param_near(mid, T::zero()) else {
        return Ok(Err(SectionVerdict::Unsupported {
            what: "a closed-form section whose carrier has no closed-form parameter",
        }));
    };
    let along = candidate.deriv(at);
    let cosine = along.dot(tangent) / (along.norm() * tangent.norm());
    Ok(
        match decide("offset_section_sense", Margin::of(cosine), band)? {
            Sign::Positive => Ok(candidate),
            Sign::Negative => Ok(candidate.reversed().unwrap_or_else(|| {
                unreachable!("every closed-form section carrier is analytic and reverses")
            })),
            Sign::Zero => Err(SectionVerdict::Unsupported {
                what: "the section crosses the old edge rather than running along it",
            }),
        },
    )
}

/// Of `candidates`, the one passing nearest the old carrier's middle;
/// a tie inside the band is refused rather than picked.
fn nearest<T: Decide>(
    candidates: Vec<Curve3<T>>,
    old: &Curve3<T>,
    (t0, t1): (T, T),
    band: Band,
) -> Result<Result<Curve3<T>, SectionVerdict>, Indeterminate> {
    let mid = old.mid_point(t0, t1);
    let reach = |c: &Curve3<T>| {
        c.param_near(mid, T::zero())
            .map(|t| c.eval(t).distance(mid))
    };
    let mut best: Option<(Curve3<T>, T)> = None;
    for c in candidates {
        let Some(r) = reach(&c) else {
            return Ok(Err(SectionVerdict::Unsupported {
                what: "a closed-form section whose carrier has no closed-form parameter",
            }));
        };
        best = match best {
            None => Some((c, r)),
            Some((b, rb)) => match decide("offset_section_branch", Margin::of(rb - r), band)? {
                Sign::Positive => Some((c, r)),
                Sign::Negative => Some((b, rb)),
                Sign::Zero => {
                    return Ok(Err(SectionVerdict::Unsupported {
                        what: "two branches of the section lie equally near the edge",
                    }));
                }
            },
        };
    }
    Ok(best.map(|(c, _)| c).ok_or(SectionVerdict::NoBranch))
}

/// **The closed-form section** of the moved analytic surface `new` and
/// the held analytic surface `held`, nearest the old carrier on
/// `[t0, t1]` and running with it. Plane × plane is the line both
/// planes hold; a plane against a cylinder, cone, sphere or torus, and
/// a cone against a coaxial cylinder, is that C5 arm's carrier. Any
/// other pair is not derived here.
pub(crate) fn section_closed<T: Decide>(
    new: &Surface<T>,
    held: &Surface<T>,
    old: &Curve3<T>,
    span: (T, T),
    extent: T,
    band: Band,
) -> Result<Result<Curve3<T>, SectionVerdict>, Indeterminate> {
    use geom_brep::intersect as c5;
    let closed = |e: geom_brep::SectionError| match e {
        geom_brep::SectionError::Escalated(source)
        | geom_brep::SectionError::RadiusEscalated { diag: source, .. } => Err(source),
        e => Ok(Err(SectionVerdict::Closed(e))),
    };
    let (plane, other) = match (new, held) {
        (Surface::Plane { .. }, _) => (new, held),
        (_, Surface::Plane { .. }) => (held, new),
        (Surface::Cone { .. }, Surface::Cylinder { .. })
        | (Surface::Cylinder { .. }, Surface::Cone { .. }) => {
            let (cone, cyl) = if matches!(new, Surface::Cone { .. }) {
                (new, held)
            } else {
                (held, new)
            };
            let candidates = match c5::cone_cylinder_section(cone, cyl, extent, band) {
                Ok(c5::ConeCylinderSection::CoaxialCircles { c1, c2 }) => vec![c1, c2],
                Err(e) => return closed(e),
            };
            let chosen = match nearest(candidates, old, span, band)? {
                Ok(c) => c,
                Err(v) => return Ok(Err(v)),
            };
            return running_with(chosen, old, span, band);
        }
        _ => {
            return Ok(Err(SectionVerdict::Unsupported {
                what: "the per-chart door derives a section through a plane, or of a cone \
                       and a coaxial cylinder, only",
            }));
        }
    };
    let candidates: Vec<Curve3<T>> = match other {
        Surface::Plane { .. } => match plane_plane(plane, other, old, span, extent, band)? {
            Ok(line) => vec![line],
            Err(v) => return Ok(Err(v)),
        },
        Surface::Cylinder { .. } => {
            let reach = geom_brep::Reach::Span {
                carrier: old.clone(),
                t0: span.0,
                t1: span.1,
            };
            match c5::plane_cylinder_section(plane, other, &reach, band) {
                Ok(
                    c5::PlaneCylinderSection::TiltedEllipse(c) | c5::PlaneCylinderSection::Rim(c),
                ) => {
                    vec![c]
                }
                Ok(c5::PlaneCylinderSection::ParallelLines { l1, l2 }) => vec![l1, l2],
                Ok(c5::PlaneCylinderSection::TangentLine(_)) => {
                    return Ok(Err(SectionVerdict::Tangent));
                }
                Ok(c5::PlaneCylinderSection::Empty) => return Ok(Err(SectionVerdict::NoBranch)),
                Err(e) => return closed(e),
            }
        }
        Surface::Cone { .. } => match c5::plane_cone_section(plane, other, extent, band) {
            Ok(c5::PlaneConeSection::ApexLinePair { l1, l2 }) => vec![l1, l2],
            Ok(
                c5::PlaneConeSection::AxisNormalCircle(c) | c5::PlaneConeSection::TiltedEllipse(c),
            ) => vec![c],
            Ok(c5::PlaneConeSection::ApexTangentLine(_)) => {
                return Ok(Err(SectionVerdict::Tangent));
            }
            Ok(c5::PlaneConeSection::ApexPoint(_)) => return Ok(Err(SectionVerdict::NoBranch)),
            Err(e) => return closed(e),
        },
        Surface::Sphere { .. } => match c5::plane_sphere_section(plane, other, band) {
            Ok(c5::PlaneSphereSection::Circle(c)) => vec![c],
            Ok(c5::PlaneSphereSection::TangentPoint(_)) => return Ok(Err(SectionVerdict::Tangent)),
            Ok(c5::PlaneSphereSection::Empty) => return Ok(Err(SectionVerdict::NoBranch)),
            Err(e) => return closed(e),
        },
        Surface::Torus { .. } => match c5::plane_torus_section(plane, other, extent, band) {
            Ok(
                c5::PlaneTorusSection::MeridianCircles { c1, c2 }
                | c5::PlaneTorusSection::ConcentricCircles { c1, c2 }
                | c5::PlaneTorusSection::SpiricOvals { s1: c1, s2: c2 },
            ) => vec![c1, c2],
            Ok(c5::PlaneTorusSection::TangentCircle(_)) => {
                return Ok(Err(SectionVerdict::Tangent));
            }
            Ok(c5::PlaneTorusSection::Empty) => return Ok(Err(SectionVerdict::NoBranch)),
            Err(e) => return closed(e),
        },
        Surface::Nurbs(_) | Surface::Approx(_) => {
            return Ok(Err(SectionVerdict::Unsupported {
                what: "a spline wall's section is the spline lane's, not a closed form",
            }));
        }
    };
    let chosen = match nearest(candidates, old, span, band)? {
        Ok(c) => c,
        Err(v) => return Ok(Err(v)),
    };
    running_with(chosen, old, span, band)
}

/// The line two planes hold, through the foot of the old carrier's
/// middle on it.
fn plane_plane<T: Decide>(
    a: &Surface<T>,
    b: &Surface<T>,
    old: &Curve3<T>,
    (t0, t1): (T, T),
    extent: T,
    band: Band,
) -> Result<Result<Curve3<T>, SectionVerdict>, Indeterminate> {
    let (
        Surface::Plane {
            origin: q1,
            normal: n1,
            ..
        },
        Surface::Plane {
            origin: q2,
            normal: n2,
            ..
        },
    ) = (a, b)
    else {
        unreachable!("plane_plane is handed two planes")
    };
    let dir = n1.cross(*n2);
    if zero("offset_section_planes_parallel", dir.norm() * extent, band)? {
        return Ok(Err(SectionVerdict::Unsupported {
            what: "the moved plane is parallel to the held one",
        }));
    }
    let dir = dir.normalize();
    let mid = old.mid_point(t0, t1);
    // Three planes: the two given, and the one across `dir` through the
    // old middle.
    let o = Point3::origin();
    let (d1, d2, d3) = (n1.dot(*q1 - o), n2.dot(*q2 - o), dir.dot(mid - o));
    let det = n1.dot(n2.cross(dir));
    let x = (n2.cross(dir) * d1 + dir.cross(*n1) * d2 + n1.cross(*n2) * d3) / det;
    Ok(Ok(Curve3::Line { origin: o + x, dir }))
}

/// **The root of an analytic surface along a carrier**, nearest `seed`:
/// the line × plane closed form, Newton on the surface's implicit form
/// along any other analytic carrier. A carrier meeting the surface at a
/// decided-zero angle grazes, and a Newton that does not land on the
/// surface has no root there.
pub(crate) fn analytic_root<T: Decide>(
    surface: &Surface<T>,
    carrier: &Curve3<T>,
    seed: T,
    extent: T,
    band: Band,
) -> Result<Result<T, CornerVerdict<T>>, Indeterminate> {
    let graze = |sine: T| -> Result<bool, Indeterminate> {
        Ok(matches!(
            decide("offset_corner_graze", Margin::levered(sine, extent), band)?,
            Sign::Zero
        ))
    };
    if let (
        Surface::Plane {
            origin: q,
            normal: m,
            ..
        },
        Curve3::Line { origin, dir },
    ) = (surface, carrier)
    {
        let sine = dir.dot(*m);
        if graze(sine.abs())? {
            return Ok(Err(CornerVerdict::Graze { sine }));
        }
        return Ok(Ok((*q - *origin).dot(*m) / sine));
    }
    if matches!(carrier, Curve3::Nurbs(_)) {
        return Ok(Err(CornerVerdict::Unsupported {
            what: "a moved surface's root along a spline edge is solved for a plane only",
        }));
    }
    let mut t = seed;
    for _ in 0..NEWTON_STEPS {
        let (p, d) = carrier.ders1(t);
        let f = geom_brep::implicit_residual(surface, p);
        let g = geom_brep::implicit_gradient(surface, p);
        t = t - f / g.dot(d);
    }
    let (p, d) = carrier.ders1(t);
    let g = geom_brep::implicit_gradient(surface, p);
    let gap = geom_brep::implicit_residual(surface, p).abs() / g.norm();
    if !zero("offset_corner_on_surface", gap, band)? {
        return Ok(Err(CornerVerdict::NoRoot));
    }
    let sine = g.dot(d) / (g.norm() * d.norm());
    if graze(sine.abs())? {
        return Ok(Err(CornerVerdict::Graze { sine }));
    }
    Ok(Ok(t))
}

/// Newton's fixed step count on an analytic root: quadratic from a seed
/// one offset away, so far more than the digits need.
const NEWTON_STEPS: usize = 24;

/// **The plane × spline-wall section** nearest `old`: the wall's exact
/// row where the plane cuts the wall along one (every control row level
/// in the plane's normal, the case of a loft between parallel
/// sections), the march's certified branch otherwise.
fn plane_wall_section(
    plane: &Surface<f64>,
    wall: &NurbsSurface<f64>,
    old: &Curve3<f64>,
    (t0, t1): (f64, f64),
    extent: f64,
    band: Band,
) -> Result<Result<NurbsCurve3<f64>, SectionVerdict>, Indeterminate> {
    let Surface::Plane { origin, normal, .. } = *plane else {
        unreachable!("the spline lane is handed a plane")
    };
    let mid = old.mid_point(t0, t1);
    let tangent = old.deriv(0.5 * (t0 + t1));
    let carrier = match level_row(wall, origin, normal, extent, band)? {
        Some(Ok(row)) => row,
        Some(Err(v)) => return Ok(Err(v)),
        None => {
            // The window holds the wall: the plane's chart reaches every
            // control point of it from the plane's origin.
            let half = wall
                .control()
                .iter()
                .map(|p| p.distance(origin))
                .fold(0.0, f64::max)
                * 2.0
                + extent;
            let domain = SsiDomain {
                center: origin,
                half_extent: half,
                extent,
                floor_scale: 1.0,
            };
            let outcome = match geom_brep::plane_nurbs_ssi(plane, wall, domain, band) {
                Ok(o) => o,
                Err(e) => return Ok(Err(SectionVerdict::Ssi(e))),
            };
            let mut best: Option<(NurbsCurve3<f64>, f64)> = None;
            for branch in outcome.branches {
                let Curve3::Nurbs(c) = branch.carrier else {
                    continue;
                };
                let (_, r) = nearest_sample(&c, mid);
                if best.as_ref().is_none_or(|(_, rb)| r < *rb) {
                    best = Some(((*c).clone(), r));
                }
            }
            match best {
                Some((c, _)) => c,
                None => return Ok(Err(SectionVerdict::NoBranch)),
            }
        }
    };
    // Its sense: the old carrier's, read where the section passes
    // nearest the old middle.
    let (at, _) = nearest_sample(&carrier, mid);
    let along = carrier.deriv(at);
    let cosine = along.dot(tangent) / (along.norm() * tangent.norm());
    Ok(
        match decide("offset_section_sense", Margin::of(cosine), band)? {
            Sign::Positive => Ok(carrier),
            Sign::Negative => match geom_brep::reversed_column(&carrier) {
                Ok(c) => Ok(c),
                Err(_) => Err(SectionVerdict::Unsupported {
                    what: "the section's knot vector does not mirror",
                }),
            },
            Sign::Zero => Err(SectionVerdict::Unsupported {
                what: "the section crosses the old edge rather than running along it",
            }),
        },
    )
}

/// The parameter of `c`'s sample nearest `p` on a fixed schedule, and
/// its distance: which branch an old edge's middle is beside, and the
/// sense there, are coarse questions a schedule answers without a
/// projection that may not settle.
fn nearest_sample(c: &NurbsCurve3<f64>, p: Point3<f64>) -> (f64, f64) {
    const SAMPLES: u32 = 64;
    let (lo, hi) = c.domain();
    (0..=SAMPLES)
        .map(|i| {
            let t = lo + (hi - lo) * f64::from(i) / f64::from(SAMPLES);
            (t, c.eval(t).distance(p))
        })
        .fold((lo, f64::INFINITY), |a, b| if b.1 < a.1 { b } else { a })
}

/// The wall's row the plane holds, where the wall's control rows are
/// each level in the plane's normal (`None` where they are not): the
/// row at the plane's one root along a column, extracted exactly.
fn level_row(
    wall: &NurbsSurface<f64>,
    origin: Point3<f64>,
    normal: Vec3<f64>,
    extent: f64,
    band: Band,
) -> Result<Option<Result<NurbsCurve3<f64>, SectionVerdict>>, Indeterminate> {
    // Rows of the transpose are columns here: `transposed()` puts the
    // row's own axis first, so `interior_iso_u` of it is the row.
    for (net, cross) in [
        (wall.clone(), wall.transposed()),
        (wall.transposed(), wall.clone()),
    ] {
        let (nu, nv) = net.control_counts();
        let ctl = net.control();
        let mut level = true;
        for j in 0..nv {
            let h0 = (ctl[j] - origin).dot(normal);
            for i in 1..nu {
                if !zero(
                    "offset_section_level_row",
                    (ctl[i * nv + j] - origin).dot(normal) - h0,
                    band,
                )? {
                    level = false;
                    break;
                }
            }
            if !level {
                break;
            }
        }
        if !level {
            continue;
        }
        // `net`'s height depends on its `v` alone, so the plane's root
        // along one of its columns is the row the plane holds.
        let (u0, _) = net.knots_u().domain();
        let column = match geom_brep::interior_iso_u(&net, u0) {
            Ok(c) => c,
            Err(_) => continue,
        };
        let Some(speed) = polynomial_speed(&column) else {
            return Ok(Some(Err(SectionVerdict::Unsupported {
                what: "a rational wall column's speed has no bound the section is read at",
            })));
        };
        let roots = match geom_brep::boundary_section(
            &column,
            origin,
            normal,
            SupSpeed::new(speed),
            band.zero(),
            extent,
            band,
        ) {
            Ok(BoundarySection::Roots {
                interior,
                at_start,
                at_end,
            }) => at_start
                .into_iter()
                .chain(interior)
                .chain(at_end)
                .collect::<Vec<_>>(),
            Ok(BoundarySection::On { .. }) => {
                return Ok(Some(Err(SectionVerdict::Unsupported {
                    what: "the moved plane holds a whole column of the wall",
                })));
            }
            Err(e) => return Ok(Some(Err(SectionVerdict::Ssi(e)))),
        };
        let [root] = roots[..] else {
            return Ok(Some(Err(if roots.is_empty() {
                SectionVerdict::NoBranch
            } else {
                SectionVerdict::Unsupported {
                    what: "the moved plane cuts the wall in more than one row",
                }
            })));
        };
        let v = bisect(&column, origin, normal, root.bracket);
        // A row whose weights vary along both parameters is not exact
        // structure; the march states the section instead.
        return Ok(geom_brep::interior_iso_u(&cross, v).ok().map(Ok));
    }
    Ok(None)
}

/// The plane distance's root inside `bracket`, to the last bit.
fn bisect(
    c: &NurbsCurve3<f64>,
    origin: Point3<f64>,
    normal: Vec3<f64>,
    bracket: (f64, f64),
) -> f64 {
    let phi = |t: f64| (c.eval(t) - origin).dot(normal);
    let (mut a, mut b) = bracket;
    let fa = phi(a);
    for _ in 0..64 {
        let m = 0.5 * (a + b);
        if m <= a || m >= b {
            break;
        }
        if (phi(m) < 0.0) == (fa < 0.0) {
            a = m;
        } else {
            b = m;
        }
    }
    if phi(a).abs() <= phi(b).abs() { a } else { b }
}

/// A certified sup of a polynomial spline's speed: its derivative's
/// control net, through the knot differences, each rounded outward.
/// `None` for a rational carrier.
fn polynomial_speed(c: &NurbsCurve3<f64>) -> Option<f64> {
    if c.weights().iter().any(|w| *w != 1.0) {
        return None;
    }
    let (p, k, ctl) = (c.degree(), c.knots().knots(), c.control());
    let mut sup = 0.0f64;
    #[allow(clippy::cast_precision_loss)]
    for i in 0..ctl.len().saturating_sub(1) {
        let dt = k[i + p + 1] - k[i + 1];
        if dt > 0.0 {
            sup = sup.max((ctl[i + 1] - ctl[i]).norm() * (p as f64) / dt);
        }
    }
    Some((sup * (1.0 + 1e-12)).next_up())
}

/// **The plane's root along a spline carrier** nearest `seed`: the
/// boundary section's isolated root, refined in its bracket, or where
/// the carrier stops short of the plane. A graze refuses by the
/// section's own name ([`SsiError::BoundaryGraze`]).
fn plane_spline_root(
    plane: &Surface<f64>,
    carrier: &NurbsCurve3<f64>,
    seed: f64,
    extent: f64,
    band: Band,
) -> Result<SplineRoot, CornerVerdict<f64>> {
    let Surface::Plane { origin, normal, .. } = *plane else {
        return Err(CornerVerdict::Unsupported {
            what: "a moved surface's root along a spline edge is solved for a plane only",
        });
    };
    let Some(speed) = polynomial_speed(carrier) else {
        return Err(CornerVerdict::Unsupported {
            what: "a rational spline edge's speed has no bound the corner is read at",
        });
    };
    let section = geom_brep::boundary_section(
        carrier,
        origin,
        normal,
        SupSpeed::new(speed),
        band.zero(),
        extent,
        band,
    )
    .map_err(CornerVerdict::Ssi)?;
    let roots = match section {
        BoundarySection::Roots {
            interior,
            at_start,
            at_end,
        } => at_start
            .into_iter()
            .chain(interior)
            .chain(at_end)
            .collect::<Vec<_>>(),
        BoundarySection::On { .. } => {
            return Err(CornerVerdict::Unsupported {
                what: "the edge lies in the moved plane",
            });
        }
    };
    let near = |r: &&geom_brep::ssi::SectionRoot| {
        let (a, b) = r.bracket;
        (0.5 * (a + b) - seed).abs()
    };
    if let Some(root) = roots.iter().min_by(|a, b| near(a).total_cmp(&near(b))) {
        return Ok(SplineRoot::At(bisect(
            carrier,
            origin,
            normal,
            root.bracket,
        )));
    }
    let (lo, hi) = carrier.domain();
    let (near_end, far_end) = if (seed - lo).abs() <= (seed - hi).abs() {
        (lo, hi)
    } else {
        (hi, lo)
    };
    let phi = |t: f64| (carrier.eval(t) - origin).dot(normal).abs();
    Ok(SplineRoot::Short {
        end: near_end,
        near_gap: phi(near_end),
        near: phi(near_end) <= phi(far_end),
    })
}

#[cfg(test)]
mod wiring {
    use super::{SectionLane, plane_spline_root, plane_wall_section};

    /// `Ok(())` when every field holds its routine; otherwise the name
    /// of the first field that does not.
    fn holds_the_section_lane() -> Result<(), &'static str> {
        let lane = SectionLane::f64();
        if !std::ptr::fn_addr_eq(
            lane.section,
            plane_wall_section as fn(_, _, _, _, _, _) -> _,
        ) {
            return Err("section is not `plane_wall_section`");
        }
        if !std::ptr::fn_addr_eq(lane.root, plane_spline_root as fn(_, _, _, _, _) -> _) {
            return Err("root is not `plane_spline_root`");
        }
        Ok(())
    }

    #[test]
    fn f64_is_wired_to_the_section_lane() {
        assert_eq!(
            holds_the_section_lane(),
            Ok(()),
            "`SectionLane::f64()` holds something other than its two routines"
        );
    }
}
