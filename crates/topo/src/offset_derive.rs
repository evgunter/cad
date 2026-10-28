//! **The offset door's derivations**: what an edge and a corner become
//! when charts move ([`crate::replace_faces_offset`],
//! [`crate::offset_surfaces_together`]).
//!
//! An edge between a moved surface and a distinct surface, held or
//! moved, is their **section** ([`section_closed`] through the C5 table's closed
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

use core::cell::OnceCell;
use geom::{Curve3, NurbsCurve3, NurbsSurface, Surface};
use std::collections::BTreeMap;

use geom_brep::ChartedNurbs;
use geom_brep::SectionReading;
use geom_brep::ssi::{
    BoundarySection, ChartAxis, ChartEnd, SsiBoundaryContact, SsiDomain, SsiError,
};

use crate::SurfaceKey;
use geom_core::k_stats::decide;
use geom_core::{Band, Decide, Indeterminate, Margin, Point3, Real, Sign, SupSpeed, Vec3};

/// Why an edge between a moved surface and its neighbour has no section
/// this door can state.
#[derive(Clone, Debug)]
pub enum SectionVerdict {
    /// A closed-form section arm refused, verbatim.
    Closed(geom_brep::SectionError),
    /// The plane × spline-wall march refused, verbatim.
    Ssi(SsiError),
    /// The section traced from the old edge's middle refused, verbatim:
    /// the middle is the trace's seed.
    FromMiddle(SsiError),
    /// The section has no branch near the edge: the moved surface no
    /// longer meets the neighbour there.
    NoBranch,
    /// The section near the edge is a tangency, which is classification
    /// data rather than a curve an edge can carry.
    Tangent,
    /// The moved surface meets the neighbour within the band at a corner
    /// of the neighbour's window only: a point, which no edge carries.
    Corner {
        /// The corner.
        corner: geom_brep::ChartCorner,
    },
    /// The door does not derive this section, in its own words.
    Unsupported {
        /// What the door could not do.
        what: &'static str,
    },
    /// The moved pair meets in a plane×cone parabola or hyperbola, whose
    /// arc is a rational-quadratic NURBS carrier (`geom_brep`'s C1) this
    /// door does not mint.
    OpenConic {
        /// The conic.
        conic: geom_brep::OpenConic,
    },
}

impl core::fmt::Display for SectionVerdict {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Closed(e) => write!(f, "the closed-form section refused: {e}"),
            Self::Ssi(e) => write!(f, "the spline-wall section refused: {e}"),
            Self::FromMiddle(e) => write!(
                f,
                "the section traced from the old edge's middle refused (the middle is the \
                 trace's seed): {e}"
            ),
            Self::NoBranch => f.write_str("the moved surface no longer meets the neighbour there"),
            Self::Tangent => f.write_str("the moved surface only touches the neighbour there"),
            Self::Corner { corner } => write!(
                f,
                "the moved surface meets the neighbour only at {corner} of its window, a point \
                 rather than an edge"
            ),
            Self::Unsupported { what } => f.write_str(what),
            Self::OpenConic { conic } => write!(
                f,
                "the moved surfaces meet in {}, an arc this offset cannot yet carry",
                conic.named()
            ),
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
/// carrier — the offset door's spline lane, at the one scalar its
/// march is written in (`geom_brep::PlaneSection` traces in
/// `f64`). Where a scalar holds none
/// ([`crate::AtRestPolicy::section_lane`]), a slanted spline edge
/// refuses by name rather than transporting.
#[derive(Clone, Copy)]
#[allow(clippy::type_complexity)]
pub struct SectionLane<T: Real> {
    section: for<'w, 's> fn(
        &'s Surface<T>,
        Wall<'w, 's, T>,
        &Curve3<T>,
        (T, T),
        T,
        Band,
    ) -> Result<Result<NurbsCurve3<T>, SectionVerdict>, Indeterminate>,
    root: fn(&Surface<T>, &NurbsCurve3<T>, T, T, Band) -> Result<SplineRoot, CornerVerdict<T>>,
    pair: for<'w, 's> fn(
        Wall<'w, 's, T>,
        Wall<'w, 's, T>,
        &Curve3<T>,
        (T, T),
        T,
        Band,
    ) -> Result<Result<NurbsCurve3<T>, SectionVerdict>, Indeterminate>,
}

/// **One offset's charted walls**: a slot per wall its sections may read,
/// each holding that wall's [`ChartedNurbs`] once it is minted. Built and
/// dropped by the operation's own frame, which owns the walls it borrows
/// (a moved chart's new surface, a held neighbour in the body); keyed by
/// structure alone, a moved wall by its chart's place in the operation's
/// charts and a held one by its [`SurfaceKey`], never by what a net holds.
/// So a wall is minted at most once an operation however many of its
/// edges are sectioned, and never across operations: no section reads a
/// chart its operation did not mint.
///
/// The slots hold `f64` charts, the one scalar whose section lane is
/// written ([`SectionLane::f64`]); an operation at any other scalar
/// builds none.
#[derive(Debug, Default)]
pub(crate) struct WallCharts<'s> {
    moved: Vec<OnceCell<ChartedNurbs<'s, f64>>>,
    held: BTreeMap<SurfaceKey, OnceCell<ChartedNurbs<'s, f64>>>,
}

impl<'s> WallCharts<'s> {
    /// Empty slots for `moved` moved charts and for every surface `held`
    /// names.
    pub(crate) fn new(moved: usize, held: impl IntoIterator<Item = SurfaceKey>) -> Self {
        Self {
            moved: (0..moved).map(|_| OnceCell::new()).collect(),
            held: held.into_iter().map(|k| (k, OnceCell::new())).collect(),
        }
    }

    /// The slot of the operation's `i`th moved chart's new surface.
    pub(crate) fn moved(&self, i: usize) -> WallSlot<'_, 's> {
        self.moved.get(i)
    }

    /// The slot of the held surface `key`.
    pub(crate) fn held(&self, key: SurfaceKey) -> WallSlot<'_, 's> {
        self.held.get(&key)
    }
}

/// A wall's chart slot as a section is handed it: the operation's
/// [`WallCharts`] entry, or `None` to mint the chart afresh.
pub(crate) type WallSlot<'w, 's> = Option<&'w OnceCell<ChartedNurbs<'s, f64>>>;

/// **A spline wall as a section reads it**: its net, and the operation's
/// slot for its chart ([`WallCharts`]) where the operation holds one.
/// The slot must be the net's own. The section doors check it in every
/// build (`geom_brep::Section::of_charted`), and since this module
/// builds every pairing, a mispaired one here is a kernel bug: the
/// door's refusal panics at the call site ([`mispaired`]).
#[derive(Clone, Copy)]
pub(crate) struct Wall<'w, 's, T: Real> {
    /// The net.
    pub(crate) net: &'s NurbsSurface<T>,
    /// Its chart's slot, or `None` to mint the chart afresh.
    pub(crate) chart: WallSlot<'w, 's>,
}

/// A `WrongLane` from a charted door, which this module builds every
/// wall/chart pairing for: a kernel bug, not an input the operation
/// can decline (`docs/DESIGN.md` `[loud-failure]`), so it panics here
/// rather than reaching a refusal or a fallback. The `pub` doors keep
/// the typed refusal for callers outside the kernel.
///
/// # Panics
///
/// Always.
#[allow(clippy::panic)]
fn mispaired(door: &str, wall: &str, e: &SsiError) -> ! {
    panic!("kernel invariant: {door} refused {wall} as mispaired with their charts: {e}")
}

/// How a section has `wall`'s chart: from the operation's slot, minting
/// into it the first time, or minted afresh where there is no slot.
fn chart_of<'w, 's>(
    slot: WallSlot<'w, 's>,
) -> impl FnOnce(&'s NurbsSurface<f64>) -> Result<ChartedNurbs<'s, f64>, SsiError> + 'w {
    move |net| match slot {
        None => ChartedNurbs::mint(net),
        Some(cell) => {
            if let Some(chart) = cell.get() {
                return Ok(chart.clone());
            }
            let chart = ChartedNurbs::mint(net)?;
            Ok(cell.get_or_init(|| chart).clone())
        }
    }
}

impl<T: Real> core::fmt::Debug for SectionLane<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str("SectionLane")
    }
}

impl SectionLane<f64> {
    /// The lane: [`plane_wall_section`], [`plane_spline_root`] and
    /// [`pair_section`].
    #[must_use]
    pub fn f64() -> Self {
        Self {
            section: plane_wall_section,
            root: plane_spline_root,
            pair: pair_section,
        }
    }
}

impl<T: Real> SectionLane<T> {
    /// The section of `plane` with the spline `wall` nearest `old` on
    /// `[t0, t1]`, running with it: the wall's exact row where the
    /// section is one, the march's branch otherwise.
    pub(crate) fn section<'w, 's>(
        self,
        plane: &'s Surface<T>,
        wall: Wall<'w, 's, T>,
        old: &Curve3<T>,
        span: (T, T),
        extent: T,
        band: Band,
    ) -> Result<Result<NurbsCurve3<T>, SectionVerdict>, Indeterminate> {
        (self.section)(plane, wall, old, span, extent, band)
    }

    /// The section of two spline charts through `old`'s middle, running
    /// with it.
    pub(crate) fn pair<'w, 's>(
        self,
        a: Wall<'w, 's, T>,
        b: Wall<'w, 's, T>,
        old: &Curve3<T>,
        span: (T, T),
        extent: T,
        band: Band,
    ) -> Result<Result<NurbsCurve3<T>, SectionVerdict>, Indeterminate> {
        (self.pair)(a, b, old, span, extent, band)
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
/// torus's also by its equatorial plane. A spline or fitted chart
/// moves each point along its own normal, so it is held along a
/// boundary `row` of its net by a plane containing that normal at
/// every point of the row ([`normal_in_plane_along`]); with no row
/// named (a corner), or beside anything but a plane, it does not hold.
/// Nothing else holds.
pub(crate) fn holds_the_move<T: Decide>(
    moved: &Surface<T>,
    held: &Surface<T>,
    row: Option<(ChartAxis, T)>,
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
        (Surface::Nurbs(_) | Surface::Approx(_), Surface::Plane { normal: m, .. }) => {
            match (moved.spline_chart(), row) {
                (Some(net), Some((axis, at))) => {
                    normal_in_plane_along(net, axis, at, *m, reach, band)
                }
                _ => false,
            }
        }
        _ => false,
    }
}

/// Whether the plane of normal `m` contains `net`'s normal at every
/// point of its boundary row `axis = at`.
///
/// At a clamped end the cross-row derivative is
/// `S_a = k·Σⱼ Nⱼ·wⱼ·(P₁ⱼ − P₀ⱼ) / W` when the first two rows' weights
/// agree pointwise, so where every step `P₁ⱼ − P₀ⱼ` is along `m` and
/// they all point ONE way along it the derivative is too, and the
/// normal `S_u × S_v`, across it, lies in the plane. Steps pointing
/// both ways can cancel in the sum, leaving whatever their small
/// leans off `m` add up to as the derivative, so a row whose steps'
/// senses disagree, or any one of them is undecided, does not hold.
/// Sufficient, not necessary: a cross-row derivative leaning along the
/// row also leaves the normal in the plane, and answers false here,
/// which routes the edge to its section. Each step's direction off `m`
/// is levered by the move, as every hold is.
fn normal_in_plane_along<T: Decide>(
    net: &NurbsSurface<T>,
    axis: ChartAxis,
    at: T,
    m: Vec3<T>,
    reach: T,
    band: Band,
) -> bool {
    // A `v` column of the chart is a `u` row of its transpose.
    let transposed;
    let net = match axis {
        ChartAxis::U => net,
        ChartAxis::V => {
            transposed = net.transposed();
            &transposed
        }
    };
    let zero = |name, value| held_zero(name, value, band);
    let (nu, nv) = net.control_counts();
    if nu < 2 {
        return false;
    }
    let (a0, a1) = net.knot_values_u().domain();
    // The boundary row and the one next in from it.
    let (first, second) = if zero("offset_holds_row_at_start", at - a0) {
        (0, 1)
    } else if zero("offset_holds_row_at_end", at - a1) {
        (nu - 1, nu - 2)
    } else {
        return false;
    };
    let row = |r: usize| r * nv..(r + 1) * nv;
    let (ctl, w) = (net.control(), net.weights_f64_interim());
    let mut sense = None;
    ctl[row(first)]
        .iter()
        .zip(&ctl[row(second)])
        .zip(w[row(first)].iter().zip(&w[row(second)]))
        .all(|((p0, p1), (w0, w1))| {
            let step = *p1 - *p0;
            let length = step.norm();
            if w0 != w1
                || zero("offset_holds_row_step", length)
                || !zero(
                    "offset_holds_row_direction",
                    step.cross(m).norm() * reach / length,
                )
            {
                return false;
            }
            match decide("offset_holds_row_sense", Margin::of(step.dot(m)), band) {
                Ok(s @ (Sign::Positive | Sign::Negative)) => *sense.get_or_insert(s) == s,
                _ => false,
            }
        })
}

/// Whether `wall` is a translation surface along `n`:
/// `S(u, v) = C(u) + f(v)·n` (or the same with the parameters
/// exchanged), read off its net — every column's offsets from its first
/// point are one list, parallel to `n`, and the weights do not vary
/// along it.
fn translates_along<T: Decide>(wall: &NurbsSurface<T>, n: Vec3<T>, reach: T, band: Band) -> bool {
    let zero = |name, value| held_zero(name, value, band);
    let (nu, nv) = wall.control_counts();
    let (ctl, w) = (wall.control(), wall.weights_f64_interim());
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
    // The distance at the foot ([`Curve3::foot`]): the unit-coordinate
    // angle alone reads an ellipse's point up to `(1 + a/b)` times
    // farther off. A carrier with no closed-form parameter is refused.
    let reach = |c: &Curve3<T>| c.param_near(mid, T::zero()).map(|t| c.foot(mid, t).gap());
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
///
/// `extent` is the old edge's extent from below
/// ([`geom_brep::edge_extent_inf`]), which the plane pair's parallelism
/// reads: its Zero refuses and its Positive certifies the crossing. The
/// torus's axis fork (`pt_axis_in_plane`) acts on its Zero (a meridian
/// or spiric pose) and refuses on a definite tilt, so it reads the old
/// edge's extent from above ([`geom_brep::edge_extent_sup`]). The cone
/// and cylinder are handed both bounds ([`geom_brep::LeverBounds`]):
/// their parallelism and coaxial swing act on a Zero and read the bound
/// from above, their convention guards and station admission act on a
/// Positive and read the bound from below
/// ([`geom_brep::intersect::cone_cylinder_section`]).
pub(crate) fn section_closed<T: Decide>(
    new: &Surface<T>,
    held: &Surface<T>,
    old: &Curve3<T>,
    span: (T, T),
    extent: T,
    band: Band,
) -> Result<Result<Curve3<T>, SectionVerdict>, Indeterminate> {
    use geom_brep::intersect as c5;
    let chord = old.eval(span.0).distance(old.eval(span.1));
    let sup = geom_brep::edge_extent_sup(old, span.0, span.1, chord);
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
            let extent = geom_brep::LeverBounds::Between {
                below: extent,
                above: sup,
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
                what: "the offset door derives a section through a plane, or of a cone \
                       and a coaxial cylinder, only",
            }));
        }
    };
    // The old edge's span, which the plane×cylinder and plane×cone arms
    // read the pose over, as `route_pose` did. A line pair through a
    // cone's apex is read again over the new edge's span once its ends
    // are known (`replace_face`'s `pair_over_its_span`).
    let span_reach = || geom_brep::Reach::Span {
        carrier: old.clone(),
        t0: span.0,
        t1: span.1,
    };
    let candidates: Vec<Curve3<T>> = match other {
        Surface::Plane { .. } => match plane_plane(plane, other, old, span, extent, band)? {
            Ok(line) => vec![line],
            Err(v) => return Ok(Err(v)),
        },
        Surface::Cylinder { .. } => {
            match c5::plane_cylinder_section(plane, other, &span_reach(), band) {
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
        Surface::Cone { .. } => match c5::plane_cone_section(plane, other, &span_reach(), band) {
            Ok(c5::PlaneConeSection::ApexLinePair { l1, l2 }) => vec![l1, l2],
            Ok(
                c5::PlaneConeSection::AxisNormalCircle(c) | c5::PlaneConeSection::TiltedEllipse(c),
            ) => vec![c],
            Ok(c5::PlaneConeSection::ApexTangentLine(_)) => {
                return Ok(Err(SectionVerdict::Tangent));
            }
            Ok(c5::PlaneConeSection::ApexPoint(_)) => return Ok(Err(SectionVerdict::NoBranch)),
            Ok(c5::PlaneConeSection::OpenConic(section)) => {
                return Ok(Err(SectionVerdict::OpenConic {
                    conic: section.conic(),
                }));
            }
            Err(e) => return closed(e),
        },
        Surface::Sphere { .. } => match c5::plane_sphere_section(plane, other, band) {
            Ok(c5::PlaneSphereSection::Circle(c)) => vec![c],
            Ok(c5::PlaneSphereSection::TangentPoint(_)) => return Ok(Err(SectionVerdict::Tangent)),
            Ok(c5::PlaneSphereSection::Empty) => return Ok(Err(SectionVerdict::NoBranch)),
            Err(e) => return closed(e),
        },
        Surface::Torus { .. } => match c5::plane_torus_section(plane, other, sup, band) {
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
            what: "the two planes are parallel",
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
    // The graze is decided at every iterate before the step divides by
    // `g·d`: a carrier tangent to the surface there has no transversal
    // root to step toward, and a step through a vanishing or undecided
    // `g·d` would carry no number at all.
    let sine_at = |p: Point3<T>, d: Vec3<T>| {
        let g = geom_brep::implicit_gradient(surface, p);
        g.dot(d) / (g.norm() * d.norm())
    };
    let mut t = seed;
    for _ in 0..NEWTON_STEPS {
        let (p, d) = carrier.ders1(t);
        let sine = sine_at(p, d);
        if !matches!(
            decide(
                "offset_corner_graze",
                Margin::levered(sine.abs(), extent),
                band
            ),
            Ok(Sign::Positive)
        ) {
            return Ok(Err(CornerVerdict::Graze { sine }));
        }
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

/// **The plane × spline-wall section** through `old`'s middle: the
/// wall's exact row where the plane cuts the wall along one (every
/// control row level in the plane's normal, the case of a loft between
/// parallel sections), otherwise what [`geom_brep::PlaneSection`] reads
/// at that point: the branch whose certified chart tube holds where the
/// middle settles, or the wall's boundary row where a `Side` region
/// holds it (the plane holds that row within the band). A `Corner`
/// region is a point, and refuses.
fn plane_wall_section<'w, 's>(
    plane: &'s Surface<f64>,
    Wall { net: wall, chart }: Wall<'w, 's, f64>,
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
            let mut section = match geom_brep::PlaneSection::of_charted(
                plane,
                (wall, chart_of(chart)),
                domain,
                band,
            ) {
                Ok(s) => s,
                Err(e @ SsiError::WrongLane { .. }) => {
                    mispaired("PlaneSection::of_charted", "the plane's wall", &e)
                }
                Err(e) => return Ok(Err(SectionVerdict::Ssi(e))),
            };
            match section.branch_at(mid) {
                Ok(SectionReading::Branch(branch)) => branch_carrier(branch),
                Ok(SectionReading::Region(SsiBoundaryContact::Side { side, .. })) => {
                    let (lo, hi) = match side.fixed {
                        ChartAxis::U => wall.knots_u_f64_interim().domain(),
                        ChartAxis::V => wall.knots_v_f64_interim().domain(),
                    };
                    let at = if side.end == ChartEnd::High { hi } else { lo };
                    match wall_row(wall, side.fixed, at) {
                        Some(row) => row,
                        None => {
                            return Ok(Err(SectionVerdict::Unsupported {
                                what: "the moved plane holds a boundary row of a wall whose \
                                       weights do not factor, which is no exact row",
                            }));
                        }
                    }
                }
                Ok(SectionReading::Region(SsiBoundaryContact::Corner { corner, .. })) => {
                    return Ok(Err(SectionVerdict::Corner { corner: *corner }));
                }
                Err(e) => return Ok(Err(seeded(e))),
            }
        }
    };
    running_as(carrier, (mid, tangent), band)
}

/// A refusal of the section traced from the old edge's middle, as the
/// door reports it. A middle that settles onto the section outside the
/// neighbour's window is the moved surface no longer meeting the
/// neighbour there, which the exact-row reading reports as
/// [`SectionVerdict::NoBranch`] too; any other refusal is the trace's.
fn seeded(e: SsiError) -> SectionVerdict {
    match e {
        SsiError::SeedOffDomain { .. } => SectionVerdict::NoBranch,
        e => SectionVerdict::FromMiddle(e),
    }
}

/// A certified branch's carrier.
fn branch_carrier(branch: &geom_brep::SsiBranch) -> NurbsCurve3<f64> {
    let Curve3::Nurbs(c) = &branch.carrier else {
        unreachable!("an `SsiBranch`'s carrier is a fitted `Curve3::Nurbs` (C1 rung 3)")
    };
    (**c).clone()
}

/// `carrier` in the old carrier's sense, read where it passes nearest
/// the old middle `mid`, whose tangent is `tangent`.
fn running_as(
    carrier: NurbsCurve3<f64>,
    (mid, tangent): (Point3<f64>, Vec3<f64>),
    band: Band,
) -> Result<Result<NurbsCurve3<f64>, SectionVerdict>, Indeterminate> {
    let (at, _) = nearest_sample(&carrier, mid);
    let along = carrier.deriv(at);
    let cosine = along.dot(tangent) / (along.norm() * tangent.norm());
    Ok(
        match decide("offset_section_sense", Margin::of(cosine), band)? {
            Sign::Positive => Ok(carrier),
            Sign::Negative => Ok(geom_brep::reversed_column(&carrier)),
            Sign::Zero => Err(SectionVerdict::Unsupported {
                what: "the section crosses the old edge rather than running along it",
            }),
        },
    )
}

/// **The section of two spline charts** through the old carrier's
/// middle: [`geom_brep::Section`]'s branch at that point, the one whose
/// certified tube holds where the middle settles, in the old carrier's
/// sense. A pair tangent at the seed refuses by the section's own
/// transversality decision, and a section outside either chart's window
/// there by the seed's.
fn pair_section<'w, 's>(
    a: Wall<'w, 's, f64>,
    b: Wall<'w, 's, f64>,
    old: &Curve3<f64>,
    (t0, t1): (f64, f64),
    extent: f64,
    band: Band,
) -> Result<Result<NurbsCurve3<f64>, SectionVerdict>, Indeterminate> {
    let mid = old.mid_point(t0, t1);
    let tangent = old.deriv(0.5 * (t0 + t1));
    // The rectangles bound the search; the window a plane would need
    // is the extent about the middle, and bounds nothing here.
    let domain = SsiDomain {
        center: mid,
        half_extent: extent,
        extent,
        floor_scale: 1.0,
    };
    let mut section = match geom_brep::Section::of_charted(
        (a.net, chart_of(a.chart)),
        (b.net, chart_of(b.chart)),
        domain,
        band,
    ) {
        Ok(s) => s,
        Err(e @ SsiError::WrongLane { .. }) => {
            mispaired("Section::of_charted", "the pair's walls", &e)
        }
        Err(e) => return Ok(Err(SectionVerdict::Ssi(e))),
    };
    let carrier = match section.branch_at(mid) {
        Ok(branch) => branch_carrier(branch),
        Err(e) => return Ok(Err(seeded(e))),
    };
    running_as(carrier, (mid, tangent), band)
}

/// The parameter of `c`'s sample nearest `p` on a fixed schedule, and
/// its distance: where a section passes beside the old edge's middle,
/// at which its sense is read, is a coarse question a schedule answers
/// without a projection that may not settle.
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
    // A net whose height depends on its `v` alone holds the plane along
    // a row of fixed `v`: the wall's own rows first, then its columns.
    for (net, fixed) in [
        (wall.clone(), ChartAxis::V),
        (wall.transposed(), ChartAxis::U),
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
        let (u0, _) = net.knots_u_f64_interim().domain();
        let column = match geom_brep::interior_iso_u(&net, u0) {
            Ok(c) => c,
            Err(_) => continue,
        };
        let speed = certified_speed(&column);
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
        return Ok(wall_row(wall, fixed, v).map(Ok));
    }
    Ok(None)
}

/// The wall's row where the parameter `fixed` holds `at`, extracted
/// exactly (`None` where the wall's weights do not factor, so no row of
/// it is exact structure).
fn wall_row(wall: &NurbsSurface<f64>, fixed: ChartAxis, at: f64) -> Option<NurbsCurve3<f64>> {
    // `transposed()` puts the `v` axis first, so `interior_iso_u` of it
    // is the row of fixed `v`.
    match fixed {
        ChartAxis::U => geom_brep::interior_iso_u(wall, at),
        ChartAxis::V => geom_brep::interior_iso_u(&wall.transposed(), at),
    }
    .ok()
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

/// A certified sup of a spline's speed, polynomial or rational: the
/// curve's [`NurbsCurve3::speed_upper_bound`] read at `Interval`, whose
/// upper end bounds the exact quotient bound from above.
fn certified_speed(c: &NurbsCurve3<f64>) -> f64 {
    geom_core::Bounds::hi(
        c.map_scalar(<geom_core::Interval as geom_core::Real>::from_f64)
            .speed_upper_bound()
            .get(),
    )
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
    let speed = certified_speed(carrier);
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
    use super::{SectionLane, pair_section, plane_spline_root, plane_wall_section};

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
        if !std::ptr::fn_addr_eq(lane.pair, pair_section as fn(_, _, _, _, _, _) -> _) {
            return Err("pair is not `pair_section`");
        }
        Ok(())
    }

    #[test]
    fn f64_is_wired_to_the_section_lane() {
        assert_eq!(
            holds_the_section_lane(),
            Ok(()),
            "`SectionLane::f64()` holds something other than its three routines"
        );
    }
}

#[cfg(test)]
#[allow(clippy::expect_used)]
mod holds {
    use super::holds_the_move;
    use geom::{NurbsSurface, Surface};
    use geom_brep::ssi::ChartAxis;
    use geom_core::spline::KnotVector;
    use geom_core::{Band, Point3, Tol, Vec3};
    use std::sync::Arc;

    /// The bilinear chart whose boundary row runs `0 → x̂` with the
    /// row next in from it `p10 → p11`, beside the plane `z = 0`: laid
    /// out as the `u = 0` row, and as the `v = 0` column of the same
    /// net transposed, which must answer alike.
    fn holds_beside_z0(p10: Point3<f64>, p11: Point3<f64>) -> bool {
        let kv = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).expect("a clamped linear vector");
        let (o, x) = (Point3::new(0.0, 0.0, 0.0), Point3::new(1.0, 0.0, 0.0));
        let plane = Surface::Plane {
            origin: Point3::origin(),
            normal: Vec3::unit_z(),
            u_ref: Vec3::unit_x(),
        };
        let band = Band::linear(Tol::witness()).expect("the witness band");
        let answer = |axis, control| {
            let chart = Surface::Nurbs(Arc::new(
                NurbsSurface::new(kv.clone(), kv.clone(), control, vec![1.0; 4])
                    .expect("a bilinear chart"),
            ));
            holds_the_move(&chart, &plane, Some((axis, 0.0)), 0.1, band)
        };
        let row = answer(ChartAxis::U, vec![o, x, p10, p11]);
        let column = answer(ChartAxis::V, vec![o, p10, x, p11]);
        assert_eq!(row, column, "the row and its transpose's column disagree");
        row
    }

    /// A row whose cross-row steps all run up `ẑ` holds: the chart's
    /// normal lies in `z = 0` all along it.
    #[test]
    fn steps_running_one_way_along_the_normal_hold() {
        assert!(holds_beside_z0(
            Point3::new(0.0, 0.0, 1.0),
            Point3::new(1.0, 0.0, 2.0)
        ));
    }

    /// **Steps along `±ẑ` that disagree in sense do not hold** (review
    /// of PR 4525, F1). Here `S_u` along the row is `(0, e, 1 − 2v)`:
    /// at `v = ½` its `ẑ` parts cancel and the normal is `−ẑ`, so the
    /// moved row point stands `|d|` off the plane. Each step alone is
    /// along `ẑ` to within its lean `e`, at `e = 0` and below the band.
    #[test]
    fn steps_along_the_normal_that_disagree_in_sense_do_not_hold() {
        for e in [0.0, 1e-13] {
            assert!(
                !holds_beside_z0(Point3::new(0.0, e, 1.0), Point3::new(1.0, e, -1.0)),
                "e = {e}"
            );
        }
    }
}

/// **A held neighbour meets the moved surface square.** Every closed-form
/// pose [`holds_the_move`] admits has the held surface contain the moved
/// surface's normal along the edge, so the two meet at a right angle:
/// the must-carry rule reads the pair `Intersection` there, never
/// under-determined, and the pair routes and serves the pose. So an
/// edge an offset door re-asks beside a held neighbour (`replace_face`'s
/// chart and pair arms) stores the section, which `neighbour_section`'s
/// route question is the precondition of, declared or derived alike.
#[cfg(test)]
#[allow(clippy::expect_used)]
mod square {
    use super::holds_the_move;
    use crate::geometry::SurfaceKey;
    use geom::{Curve3, Surface};
    use geom_core::{Band, Point3, Tol, Vec3};

    /// A pose by name: the moved surface, the held one, and an edge on
    /// both over its span.
    type Pose = (
        &'static str,
        Surface<f64>,
        Surface<f64>,
        Curve3<f64>,
        (f64, f64),
    );

    #[test]
    fn every_held_pose_routes_and_reads_the_section() {
        let band = Band::linear(Tol::witness()).expect("the witness band");
        let mut keys: slotmap::SlotMap<SurfaceKey, ()> = slotmap::SlotMap::with_key();
        let (ka, kb) = (keys.insert(()), keys.insert(()));
        let (x, y, z) = (Vec3::unit_x(), Vec3::unit_y(), Vec3::unit_z());
        let o = Point3::origin();
        let plane = |origin, normal: Vec3<f64>, u_ref| Surface::Plane {
            origin,
            normal,
            u_ref,
        };
        let line = |origin, dir| Curve3::Line { origin, dir };
        let circle = |center, axis, radius, u_ref| Curve3::Circle {
            center,
            axis,
            radius,
            u_ref,
        };
        let cylinder = Surface::Cylinder {
            origin: o,
            axis: z,
            radius: 1.0,
            u_ref: x,
        };
        let torus = Surface::Torus {
            center: o,
            axis: z,
            major_radius: 3.0,
            minor_radius: 1.0,
            u_ref: x,
        };
        let s = std::f64::consts::FRAC_1_SQRT_2;
        let poses: Vec<Pose> = vec![
            (
                "plane beside a plane holding its normal",
                plane(o, z, x),
                plane(o, x, y),
                line(o, y),
                (0.0, 1.0),
            ),
            (
                "plane beside a cylinder along its normal",
                plane(o, z, x),
                cylinder.clone(),
                circle(o, z, 1.0, x),
                (0.0, 1.0),
            ),
            (
                "cylinder beside a plane across its axis",
                cylinder.clone(),
                plane(o, z, x),
                circle(o, z, 1.0, x),
                (0.0, 1.0),
            ),
            (
                "cylinder beside a plane holding its axis",
                cylinder.clone(),
                plane(o, x, y),
                line(Point3::new(0.0, 1.0, 0.0), z),
                (0.0, 1.0),
            ),
            (
                "sphere beside a plane through its centre",
                Surface::Sphere {
                    center: o,
                    radius: 1.0,
                    axis: z,
                    u_ref: x,
                },
                plane(o, z, x),
                circle(o, z, 1.0, x),
                (0.0, 1.0),
            ),
            (
                "cone beside a plane holding its axis",
                Surface::Cone {
                    apex: o,
                    axis: z,
                    half_angle: std::f64::consts::FRAC_PI_4,
                    u_ref: x,
                },
                plane(o, x, y),
                line(o, Vec3::new(0.0, s, s)),
                (0.5, 1.5),
            ),
            (
                "torus beside a meridian plane",
                torus.clone(),
                plane(o, x, y),
                circle(Point3::new(0.0, 3.0, 0.0), x, 1.0, y),
                (0.0, 1.0),
            ),
            (
                "torus beside its equatorial plane",
                torus,
                plane(o, z, x),
                circle(o, z, 4.0, x),
                (0.0, 1.0),
            ),
        ];
        for (name, moved, held, carrier, (t0, t1)) in poses {
            assert!(
                holds_the_move(&moved, &held, None, 0.1, band),
                "{name}: holds"
            );
            let reach = geom_brep::Reach::Span {
                carrier: carrier.clone(),
                t0,
                t1,
            };
            let route = geom_brep::intersect::route_pose(&moved, &held, &reach, band)
                .expect("the pose is decided");
            assert!(route.implemented, "{name}: {}", route.note);
            let read = geom_brep::reask_pair(
                [(ka, &moved), (kb, &held)],
                &carrier,
                (t0, t1),
                1.0,
                None,
                band,
            )
            .expect("the rule reads the pair");
            assert!(
                matches!(read, geom_brep::EdgeDescriptionSpec::Intersection { .. }),
                "{name}: {read:?}"
            );
        }
    }
}

/// The plane × spline-wall door where the old edge's middle settles in a
/// region the boundary pass reports.
#[cfg(test)]
#[allow(clippy::expect_used, clippy::panic)]
mod regions {
    use super::{SectionVerdict, Wall, plane_wall_section, wall_row};
    use geom::{Curve3, NurbsSurface, Surface};
    use geom_brep::ssi::ChartAxis;
    use geom_core::spline::KnotVector;
    use geom_core::{Band, Point3, Tol, Vec3};
    use std::sync::Arc;

    /// A biquadratic over `[0, 1]²` whose height is `z(i, j)` at control
    /// point `(i, j)`.
    fn net(z: &dyn Fn(usize, usize) -> f64) -> NurbsSurface<f64> {
        let k = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).expect("clamped");
        let control = (0..9)
            .map(|n| {
                let (i, j) = (n / 3, n % 3);
                Point3::new(0.5 * i as f64, 0.5 * j as f64, z(i, j))
            })
            .collect();
        NurbsSurface::new(k.clone(), k, control, vec![1.0; 9]).expect("a biquadratic")
    }

    fn ground() -> Surface<f64> {
        Surface::Plane {
            origin: Point3::origin(),
            normal: Vec3::unit_z(),
            u_ref: Vec3::unit_x(),
        }
    }

    fn band() -> Band {
        Band::linear(Tol::witness()).expect("the witness band")
    }

    /// **A plane holding a wall's boundary row is that row.** The wall
    /// rises from `z = 0` along its bottom row and leans along its others,
    /// so no row is level and the march reads the section; the old edge
    /// is that bottom row. Its middle settles in the pass's `Side`
    /// region, and the edge is the wall's exact bottom row, as
    /// `level_row` extracts a row. Red if the region refuses.
    #[test]
    fn a_side_region_is_the_walls_boundary_row() {
        let wall = net(&|i, j| (0.5 + 0.2 * i as f64) * j as f64);
        let row = geom_brep::boundary_iso_v(&wall, false).expect("the bottom row");
        let span = row.domain();
        let old = Curve3::Nurbs(Arc::new(row));
        let carrier = match plane_wall_section(
            &ground(),
            Wall {
                net: &wall,
                chart: None,
            },
            &old,
            span,
            1.0,
            band(),
        ) {
            Ok(Ok(c)) => c,
            other => panic!("the bottom row is the section, got {other:?}"),
        };
        let exact = wall_row(&wall, ChartAxis::V, 0.0).expect("an exact row");
        let xyz = |c: &geom::NurbsCurve3<f64>| -> Vec<[f64; 3]> {
            c.control().iter().map(|p| [p.x, p.y, p.z]).collect()
        };
        assert_eq!(
            xyz(&carrier),
            xyz(&exact),
            "the edge is the wall's exact bottom row"
        );
    }

    /// **A plane meeting a wall at a corner only refuses by the corner.**
    /// The tilted wall `z = x + y` meets `z = 0` at its corner `(0, 0)`
    /// alone, and an old edge through that corner settles in the pass's
    /// `Corner` region: a point, which no edge carries.
    #[test]
    fn a_corner_region_refuses_by_the_corner() {
        let wall = net(&|i, j| 0.5 * (i + j) as f64);
        let old = Curve3::Line {
            origin: Point3::origin(),
            dir: Vec3::new(1.0, -1.0, 0.0) / 2.0_f64.sqrt(),
        };
        let got = plane_wall_section(
            &ground(),
            Wall {
                net: &wall,
                chart: None,
            },
            &old,
            (-0.1, 0.1),
            1.0,
            band(),
        );
        assert!(
            matches!(got, Ok(Err(SectionVerdict::Corner { .. }))),
            "expected the corner's refusal, got {got:?}"
        );
    }
}

/// **The spline lane reaches rational carriers.** Both doors that meter
/// a spline's speed ([`certified_speed`]) answer on a rational one:
/// `level_row`'s column and `plane_spline_root`'s edge each read the
/// quotient bound, where they refused before it.
#[cfg(test)]
#[allow(clippy::expect_used, clippy::panic)]
mod rational_carriers {
    use super::{SplineRoot, Wall, plane_spline_root, plane_wall_section};
    use geom::{Curve3, NurbsCurve3, NurbsSurface, Surface};
    use geom_core::spline::KnotVector;
    use geom_core::{Band, Point3, Tol, Vec3};

    fn band() -> Band {
        Band::linear(Tol::witness()).expect("the witness band")
    }

    fn quad() -> KnotVector {
        KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).expect("clamped")
    }

    /// A biquadratic wall whose rows are level (height `−½, 0, ½` by
    /// row) and whose weights vary along its columns, so `level_row`
    /// meters a RATIONAL column: `z = 0` holds its middle row, which
    /// is the section, every control point on the plane.
    #[test]
    fn a_level_row_of_a_rational_wall_is_the_section() {
        let control = (0..9)
            .map(|n| {
                let (i, j) = (n / 3, n % 3);
                #[allow(clippy::cast_precision_loss)]
                Point3::new(0.5 * i as f64, 0.1 * (i * j) as f64, 0.5 * j as f64 - 0.5)
            })
            .collect();
        let weights = (0..9).map(|n| [1.0, 0.5, 1.0][n % 3]).collect();
        let wall = NurbsSurface::new(quad(), quad(), control, weights).expect("a rational wall");
        let plane = Surface::Plane {
            origin: Point3::origin(),
            normal: Vec3::unit_z(),
            u_ref: Vec3::unit_x(),
        };
        let old = Curve3::Line {
            origin: Point3::new(0.0, 0.05, 0.0),
            dir: Vec3::unit_x(),
        };
        let row = match plane_wall_section(
            &plane,
            Wall {
                net: &wall,
                chart: None,
            },
            &old,
            (0.0, 1.0),
            1.0,
            band(),
        ) {
            Ok(Ok(c)) => c,
            other => panic!("the middle row is the section, got {other:?}"),
        };
        for p in row.control() {
            assert!(
                p.z.abs() <= 1e-12,
                "a row control point off the plane: {p:?}"
            );
        }
    }

    /// A rational quarter arc crossed by the plane `x = ½`: the root is
    /// the arc's point at 60°.
    #[test]
    fn a_rational_edge_has_its_plane_root() {
        let h = 0.5f64.sqrt();
        let arc = NurbsCurve3::new(
            quad(),
            vec![
                Point3::new(1.0, 0.0, 0.0),
                Point3::new(1.0, 1.0, 0.0),
                Point3::new(0.0, 1.0, 0.0),
            ],
            vec![1.0, h, 1.0],
        )
        .expect("a quarter arc");
        let plane = Surface::Plane {
            origin: Point3::new(0.5, 0.0, 0.0),
            normal: Vec3::unit_x(),
            u_ref: Vec3::unit_y(),
        };
        match plane_spline_root(&plane, &arc, 0.6, 1.0, band()) {
            Ok(SplineRoot::At(t)) => {
                let p = arc.eval(t);
                assert!(
                    (p.x - 0.5).abs() <= 1e-9 && (p.y - 0.75f64.sqrt()).abs() <= 1e-9,
                    "the root reads {p:?}"
                );
            }
            Ok(SplineRoot::Short { .. }) => panic!("the arc crosses the plane; no root read"),
            Err(e) => panic!("the rational edge refused: {e:?}"),
        }
    }
}
