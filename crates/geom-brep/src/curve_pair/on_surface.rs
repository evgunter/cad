//! **Two carriers on one surface** (C5's co-surface arm). A curve built on
//! a surface ([`SurfaceCurve`]) and an edge of a face on it ([`OnSurface`])
//! lie on one surface record, so their gap is zero in the model: a 3D
//! curve-pair reading ([`super::meet`]) of the pair decides only the
//! construction's rounding against the band. They meet instead where one
//! crosses the other's plane, through the curve × face crossing reading
//! ([`crate::curve_face::cross()`], whose plane arm reads
//! [`super::pose::height`], the one [`super::meet`]'s transverse reading is
//! built on), at a meeting that lies on both curves' own sections of their
//! planes ([`Section`]). Which one crosses which plane is fixed per pair
//! of a curve's family and an edge's kind by the arm's direction table
//! ([`direction`]), on the transversality rule, and read by
//! [`OnSurface::meet`], which places each meeting on the edge's span.
//!
//! A plane through a curve on the surface cuts the surface in that curve
//! and a few others: a plane's square plane through a line of it in that
//! line alone, a cylinder's plane through the axis in two rulings, a
//! cone's in four half-generators, a torus's planes in two circles. Which
//! of them a root lies on is read through the chart's channels
//! ([`geom::ChartChannels::read`]) on the caller's row, so the reading
//! decides it as a length.
//!
//! A [`SurfaceCurve`]'s **locus** is built from a surface record's surface
//! and chart coordinates and from nothing else: its centre, radius and
//! direction are that surface's own at those coordinates, so no curve off
//! it can be built as one (C5, `crates/geom-brep/README.md`: "a curve on a
//! surface is built from chart coordinates on a surface record"). Its
//! parametrisation may start at a point the
//! caller holds on it ([`SurfaceCurve::starting_at`]): a circle's `θ = 0`
//! names where along the locus places are counted from, and moves no
//! point of it. The surface is the record's as stored, or turned about
//! its own axis onto another seam ([`Charted::turned`]: the sphere
//! cut-in's meridian), the same surface on another chart.
//!
//! A curve and an edge are tied by record: a curve carries the key of the
//! surface record it was charted from ([`Charted`]), an edge the key of
//! its face's record ([`OnSurface::edge`]), and every reading of the pair,
//! [`OnSurface::meet`] and [`OnSurface::across`], refuses a pair on two
//! records ([`AcrossRefusal::OffRecord`], [`MeetRefusal::OffRecord`]). That the key and the surface
//! a caller hands [`Charted::record`] are one record's stays the
//! caller's: each reads both off one face.

use geom::{Azimuth, ChartAngle, ChartArm, ChartChannels, ChartInverse, Circle, Curve3, Frame};
use geom_core::UnitVec3;
use geom_core::k_stats::decide;
use geom_core::{
    Band, Decide, Indeterminate, Margin, Point2, Point3, Real, RigidMotion, Sign, Vec3,
};

use crate::curve_face::{CrossRefusal, Meeting, OnSpan, Pose};
use crate::curve_pair::Place;

use crate::keys::SurfaceKey;

/// How far, relative to a radius of at least 1, [`SurfaceCurve::starting_at`]'s
/// point may lie off its circle in a debug build.
const ON_CIRCLE: f64 = 1e-6;

/// **Which of the curves its plane cuts from the surface a curve is**:
/// what a root on that plane is read against ([`SurfaceCurve::section_of`]).
#[derive(Clone, Copy, Debug)]
pub enum Section<T: Real> {
    /// The plane cuts the surface in this curve alone.
    Whole,
    /// The plane holds the axis: the curve is its curve on the half-plane
    /// of chart azimuth `toward`, and on a cone on the nappe `opening`.
    HalfPlane {
        /// The half-plane's azimuth direction.
        toward: Azimuth<T>,
        /// The nappe, on a cone (`true` the opening one).
        opening: Option<bool>,
    },
    /// A plane square to a torus's axis: the parallel on the tube side
    /// `toward`, a unit along the radial component of the tube angle.
    TubeSide {
        /// The tube side's unit direction.
        toward: Azimuth<T>,
    },
    /// An edge's circle, of its plane's section: the one about `centre`
    /// at `radius`, where the plane holds another (a torus's meridian
    /// and its mirror, a Villarceau circle and its mirror, a parallel and
    /// the one across the tube). Read as a distance from the centre, it
    /// never cannot tell: a point where a Villarceau circle crosses its
    /// mirror is on both, so on this one.
    Circle {
        /// The circle's centre.
        centre: Point3<T>,
        /// Its radius.
        radius: T,
    },
    /// An edge's spiric oval: the one on the side `side` (a unit) of the
    /// axis's projection on its plane, through `foot`.
    Oval {
        /// A point of the axis's projection.
        foot: Point3<T>,
        /// The oval's side of it.
        side: Vec3<T>,
    },
}

impl<T: Decide> Section<T> {
    /// **Whether `y`, a point of the section's plane on the surface, lies
    /// on this section's curve**, read from the section's own data, on
    /// `row`: the whole section always; an edge's circle where `y` is at
    /// its radius from its centre, the plane's other curve definitely off
    /// it; an edge's oval on its side of the axis's projection. `Zero`
    /// where an oval's reading cannot tell the two apart.
    ///
    /// # Errors
    ///
    /// A reading in band.
    ///
    /// # Panics
    ///
    /// On a section read through a chart ([`Self::HalfPlane`],
    /// [`Self::TubeSide`]): its curve's ([`SurfaceCurve::section_of`]).
    fn of_its_own(
        &self,
        y: Point3<T>,
        row: &'static str,
        band: Band,
    ) -> Result<Sign, Indeterminate> {
        match *self {
            Self::Whole => Ok(Sign::Positive),
            Self::Circle { centre, radius } => Ok(
                match decide(row, Margin::of((y - centre).norm() - radius), band)? {
                    Sign::Zero => Sign::Positive,
                    Sign::Positive | Sign::Negative => Sign::Negative,
                },
            ),
            Self::Oval { foot, side } => decide(row, Margin::of((y - foot).dot(side)), band),
            Self::HalfPlane { .. } | Self::TubeSide { .. } => {
                unreachable!("a section read through a chart is its curve's")
            }
        }
    }
}

/// **A curve on a surface's family**: what the co-surface arm's direction
/// table reads of the curve built on the surface ([`direction`]).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Family {
    /// A cylinder's or a cone's parallel: its plane square to the axis
    /// meets the surface square all along it.
    Parallel,
    /// A torus's parallel: its plane is square to the axis, tangent to the
    /// torus along the crest parallels (its top and bottom circles) and
    /// all but tangent along the parallels near them.
    TorusParallel,
    /// A torus's or a sphere's meridian: its plane holds the axis and is
    /// normal to the surface everywhere.
    Meridian,
    /// A cylinder's or a cone's ruling through a chart point.
    Ruling,
    /// A cone's generator from its apex.
    Generator,
    /// A plane's `u`-line: its plane is square to the surface.
    ULine,
    /// A sphere's great circle.
    GreatCircle,
    /// A sphere's small circle about a pole.
    SmallCircle,
}

/// **Which curve of a co-surface pair crosses the other's plane**
/// ([`direction`]).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Read {
    /// The edge across the curve's plane ([`OnSurface::across`]).
    EdgeOverLeg,
    /// The curve across the edge's own plane.
    LegOverEdge,
}

/// **The co-surface arm's direction table** (C5): one row per pair of a
/// curve family and an edge's kind, fixed here, on the transversality
/// rule — the plane read is that of a curve whose plane is transversal
/// to the surface all along that curve.
///
/// - A torus's parallel against a planar edge (a circle, an ellipse, a
///   spiric) reads the parallel across the edge's plane: the parallel's
///   own plane is tangent to the torus along the crest parallels and all
///   but tangent along those near them, where an edge across it grazes it
///   or crosses it at a pair of roots the band cannot part. A torus's circle edge is a
///   meridian, whose plane is normal to the torus, or a parallel, whose
///   plane is the leg's or parallel to it, or a Villarceau circle, whose
///   plane is tangent to the torus at two points: that row has no
///   transversal choice, is fixed all the same, and escalates in band
///   there.
/// - Any other curve against a spiric edge reads the curve across the
///   oval's plane, which is parallel to the torus's axis and stands
///   inside the inner equator, so transversal to the torus all along it.
/// - Every other pair reads the edge across the curve's plane: a
///   meridian's, a ruling's, a generator's and a sphere's circles' are
///   transversal to their surfaces all along them, a cylinder's or a
///   cone's parallel's square to it, and a plane's `u`-line's square
///   plane meets it in that line alone (the planar walk's ray, spirics
///   included, through the spiric × plane door).
#[must_use]
pub const fn direction(curve: Family, edge: geom::CurveKind) -> Read {
    use geom::CurveKind::{Circle, Ellipse, Line, Nurbs, Periodic, Spiric};
    match (curve, edge) {
        (Family::TorusParallel, Circle | Ellipse | Spiric) => Read::LegOverEdge,
        (
            Family::Parallel
            | Family::Meridian
            | Family::Ruling
            | Family::Generator
            | Family::GreatCircle
            | Family::SmallCircle,
            Spiric,
        ) => Read::LegOverEdge,
        (Family::ULine, Spiric) => Read::EdgeOverLeg,
        (
            Family::Parallel
            | Family::TorusParallel
            | Family::Meridian
            | Family::Ruling
            | Family::Generator
            | Family::ULine
            | Family::GreatCircle
            | Family::SmallCircle,
            Line | Nurbs | Periodic,
        ) => Read::EdgeOverLeg,
        (
            Family::Parallel
            | Family::Meridian
            | Family::Ruling
            | Family::Generator
            | Family::ULine
            | Family::GreatCircle
            | Family::SmallCircle,
            Circle | Ellipse,
        ) => Read::EdgeOverLeg,
    }
}

/// A curve built on a surface: a circle or a line.
#[derive(Clone, Copy, Debug)]
pub enum Shape<T: Real> {
    /// A circle of the surface.
    Circle(Circle<T>),
    /// A ruling, from the point it was built at.
    Line(geom::Line<T>),
}

/// **A curve on a surface, built from its chart**: the surface's parallel,
/// meridian or ruling through a chart point, or a sphere's great or small
/// circle through one. Its section of its own plane is part of what it is
/// ([`Section`]).
#[derive(Clone, Copy, Debug)]
pub struct SurfaceCurve<T: Real> {
    chart: Placed<T>,
    family: Family,
    shape: Shape<T>,
    plane: (Point3<T>, Vec3<T>),
    section: Section<T>,
}

/// **A surface record's surface and its key**: what a [`SurfaceCurve`]
/// is built on. It is the record's own surface as stored
/// ([`Self::record`]), or that surface turned about its own axis
/// ([`Self::turned`]): the same surface on another chart. A surface a
/// caller builds instead is [`Self::held`], its premise the caller's.
/// The key names the record, so a curve built on it is read only against
/// edges of faces on that record ([`OnSurface::across`]).
#[derive(Clone, Debug)]
pub struct Charted<T: Real> {
    key: SurfaceKey,
    surface: geom::Surface<T>,
}

impl<T: Real> Charted<T> {
    /// The record `key`'s surface, `surface` as the record stores it.
    pub fn record(key: SurfaceKey, surface: &geom::Surface<T>) -> Self {
        Self {
            key,
            surface: surface.clone(),
        }
    }

    /// **A surface the caller builds rather than reads off the record**,
    /// under the record's key: one the caller holds to carry the curves of
    /// the record's face it reads it against, on a premise of its own that
    /// it names where it builds it (the planar loop walk's ray plane).
    pub fn held(key: SurfaceKey, surface: geom::Surface<T>) -> Self {
        Self { key, surface }
    }

    /// **The same surface turned about its own axis**, its seam at
    /// `u_ref`, a unit direction square to the axis (conventional, as a
    /// surface's own `u_ref` is): every other field is the record's. A
    /// cylinder, cone, sphere or torus is its own image under a turn about
    /// its axis, so this is the record's surface on another chart.
    ///
    /// # Panics
    ///
    /// On a plane or a spline surface, which no caller turns: a fault in
    /// the caller.
    #[must_use]
    pub fn turned(self, u_ref: Vec3<T>) -> Self {
        let surface = match self.surface {
            geom::Surface::Cylinder {
                frame: carrier_frame,
                radius,
                ..
            } => geom::Surface::Cylinder {
                frame: turned_to(&carrier_frame, u_ref),
                radius,
            },
            geom::Surface::Cone {
                frame: carrier_frame,
                half_angle,
                ..
            } => geom::Surface::Cone {
                frame: turned_to(&carrier_frame, u_ref),
                half_angle,
            },
            geom::Surface::Sphere {
                frame: carrier_frame,
                radius,
                ..
            } => geom::Surface::Sphere {
                frame: turned_to(&carrier_frame, u_ref),
                radius,
            },
            geom::Surface::Torus {
                frame: carrier_frame,
                major_radius,
                minor_radius,
                ..
            } => geom::Surface::Torus {
                frame: turned_to(&carrier_frame, u_ref),
                major_radius,
                minor_radius,
            },
            other => unreachable!("only a surface of revolution is turned: {:?}", other.kind()),
        };
        Self { surface, ..self }
    }

    /// The record's key.
    pub fn key(&self) -> SurfaceKey {
        self.key
    }

    /// The surface the curve is charted on.
    pub fn surface(&self) -> &geom::Surface<T> {
        &self.surface
    }
}

/// A curve's chart: the surface's inversion and the record it was charted
/// from.
#[derive(Clone, Copy, Debug)]
struct Placed<T: Real> {
    inverse: ChartInverse<T>,
    record: SurfaceKey,
}

impl<T: Real> Placed<T> {
    fn arm(&self) -> ChartArm<T> {
        self.inverse.arm()
    }

    fn frame(&self) -> &Frame<T> {
        self.inverse.frame()
    }
}

/// **`frame` turned about its own axis onto the seam `u_ref`**: the axis
/// kept as the witness the frame holds and `u_ref`'s part square to it
/// normalized ([`RigidMotion::squared_about`]), the origin kept. For a
/// seam the caller holds square to the axis already, so nothing is
/// decided.
fn turned_to<T: Real>(frame: &Frame<T>, u_ref: Vec3<T>) -> Frame<T> {
    Frame::of_motion(&RigidMotion::squared_about(
        frame.origin(),
        frame.axis_unit(),
        u_ref,
    ))
}

/// A surface kind a curve family is not built on.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NoSuchFamily {
    /// The family asked for.
    pub family: &'static str,
    /// The surface kind it was asked of.
    pub kind: geom::SurfaceKind,
}

impl<T: Decide> SurfaceCurve<T> {
    fn chart(on: &Charted<T>, family: &'static str) -> Result<Placed<T>, NoSuchFamily> {
        let inverse = ChartInverse::of(&on.surface).map_err(|_| NoSuchFamily {
            family,
            kind: on.surface.kind(),
        })?;
        Ok(Placed {
            inverse,
            record: on.key,
        })
    }

    fn refused(family: &'static str, surface: &geom::Surface<T>) -> NoSuchFamily {
        NoSuchFamily {
            family,
            kind: surface.kind(),
        }
    }

    /// The circle of the family about `center` in the plane of `normal`,
    /// its seam toward `start`. Its frame is minted decided
    /// ([`RigidMotion::from_axis_and_reference`] at the least band, under
    /// `on_surface_circle_frame`): the frame door's arithmetic step for
    /// step, so its bits are the squared frame's, and `None` where
    /// `start` stands on the axis through `center` (a cone's apex, a
    /// sphere's pole on its own axis), where the circle is a point and
    /// the undecided square poisoned its frame. The caller refuses that
    /// as no such family there.
    #[allow(clippy::too_many_arguments)] // the chart, the family, the circle and its section
    fn circle(
        chart: Placed<T>,
        family: Family,
        center: Point3<T>,
        normal: Vec3<T>,
        radius: T,
        start: Point3<T>,
        section: Section<T>,
    ) -> Option<Self> {
        let w = start - center;
        let frame = RigidMotion::from_axis_and_reference(
            center,
            normal,
            w.normalize(),
            "on_surface_circle_frame",
            Band::least(),
        )
        .ok()?;
        Some(Self {
            chart,
            family,
            shape: Shape::Circle(Circle::new(Frame::of_motion(&frame), radius)),
            plane: (center, normal),
            section,
        })
    }

    /// **The parallel through `uv`** on a cylinder, a cone or a torus,
    /// run `forward` (right-handed about the axis) or back, starting at
    /// the point `uv` names: at the surface's own radius there.
    ///
    /// # Errors
    ///
    /// [`NoSuchFamily`] on any other surface.
    pub fn parallel(on: &Charted<T>, uv: Point2<T>, forward: bool) -> Result<Self, NoSuchFamily> {
        let surface = &on.surface;
        let chart = Self::chart(on, "parallel")?;
        let f = *chart.frame();
        let normal = if forward { f.axis() } else { -f.axis() };
        let start = surface.eval(uv.x, uv.y);
        let (family, center, radius, section) = match chart.arm() {
            ChartArm::Cylinder { radius } => (
                Family::Parallel,
                f.origin() + f.axis() * uv.y,
                radius,
                Section::Whole,
            ),
            ChartArm::Cone { half_angle } => {
                let (sin, cos) = half_angle.sin_cos();
                (
                    Family::Parallel,
                    f.origin() + f.axis() * (uv.y * cos),
                    (uv.y * sin).abs(),
                    Section::Whole,
                )
            }
            ChartArm::Torus { major, minor } => {
                let (sin, cos) = uv.y.sin_cos();
                // The tube side `uv` lies on, as a unit: a root's `ρ − R`
                // is read along it in metres.
                let side = cos.select_le_zero(-T::one(), T::one());
                (
                    Family::TorusParallel,
                    f.origin() + f.axis() * (minor * sin),
                    major + minor * cos,
                    Section::TubeSide {
                        toward: Azimuth::of(T::zero(), side),
                    },
                )
            }
            _ => return Err(Self::refused("parallel", surface)),
        };
        Self::circle(chart, family, center, normal, radius, start, section)
            .ok_or_else(|| Self::refused("parallel", surface))
    }

    /// **A meridian through `uv`**: a torus's tube circle at `uv`'s
    /// azimuth, or a sphere's great circle through its poles there, run
    /// `forward` (the second chart angle increasing) or back.
    ///
    /// # Errors
    ///
    /// [`NoSuchFamily`] on any other surface.
    pub fn meridian(on: &Charted<T>, uv: Point2<T>, forward: bool) -> Result<Self, NoSuchFamily> {
        let surface = &on.surface;
        let chart = Self::chart(on, "meridian")?;
        let f = *chart.frame();
        let radial = f.radial(uv.x);
        let normal = radial.cross(f.axis());
        let normal = if forward { normal } else { -normal };
        let start = surface.eval(uv.x, uv.y);
        let (sin_u, cos_u) = uv.x.sin_cos();
        let section = Section::HalfPlane {
            toward: Azimuth::of(sin_u, cos_u),
            opening: None,
        };
        let (center, radius) = match chart.arm() {
            ChartArm::Torus { major, minor } => (f.origin() + radial * major, minor),
            ChartArm::Sphere { radius } => (f.origin(), radius),
            _ => return Err(Self::refused("meridian", surface)),
        };
        Self::circle(
            chart,
            Family::Meridian,
            center,
            normal,
            radius,
            start,
            section,
        )
        .ok_or_else(|| Self::refused("meridian", surface))
    }

    /// **The ruling through `uv`** on a cylinder or a cone, on the nappe
    /// `opening` (a cylinder has one), run `outward` (away from a cone's
    /// apex; along the axis on a cylinder) or back.
    ///
    /// # Errors
    ///
    /// [`NoSuchFamily`] on any other surface.
    pub fn ruling(
        on: &Charted<T>,
        uv: Point2<T>,
        opening: bool,
        outward: bool,
    ) -> Result<Self, NoSuchFamily> {
        let surface = &on.surface;
        let chart = Self::chart(on, "ruling")?;
        let f = *chart.frame();
        let start = surface.eval(uv.x, uv.y);
        let (dir, opening) = match chart.arm() {
            ChartArm::Cylinder { .. } => (f.axis_unit(), None),
            ChartArm::Cone { .. } => {
                let g = start - f.origin();
                (UnitVec3::squared(g), Some(opening))
            }
            _ => return Err(Self::refused("ruling", surface)),
        };
        let dir = if outward { dir } else { -dir };
        Ok(Self::line(
            chart,
            Family::Ruling,
            geom::Line { origin: start, dir },
            uv.x,
            opening,
        ))
    }

    /// **A cone's generator from its apex** through the point `toward`
    /// names, on the nappe `opening` that point lies on.
    ///
    /// # Errors
    ///
    /// [`NoSuchFamily`] on any surface but a cone.
    pub fn from_apex(
        on: &Charted<T>,
        toward: Point2<T>,
        opening: bool,
    ) -> Result<Self, NoSuchFamily> {
        let surface = &on.surface;
        let chart = Self::chart(on, "generator from the apex")?;
        if !matches!(chart.arm(), ChartArm::Cone { .. }) {
            return Err(Self::refused("generator from the apex", surface));
        }
        let apex = surface.eval(toward.x, T::zero());
        let g = surface.eval(toward.x, toward.y) - chart.frame().origin();
        Ok(Self::line(
            chart,
            Family::Generator,
            geom::Line {
                origin: apex,
                dir: UnitVec3::squared(g),
            },
            toward.x,
            Some(opening),
        ))
    }

    /// **The circle with its parametrisation started at `p`**, a point the
    /// caller holds on it: `θ = 0` is `p`'s offset from the centre with its
    /// component along the axis projected out, so the frame stays square to
    /// the axis. The locus (centre, axis, radius) is the chart's, as built
    /// from chart coordinates on the record (C5); only where places along
    /// it are counted from moves. The cone and torus region doors start
    /// their circle legs at the query's closed-form foot
    /// ([`geom::Surface::foot`]), and the sphere door its great circle and
    /// its circle about the query at the foot and the corner it holds, so a
    /// crossing's place along a leg reads no chart frame in the leg's start,
    /// which `surface.eval(uv)` carries in its last ulps.
    ///
    /// # Panics
    ///
    /// On a line: a line's origin is a point of its locus, so it does not
    /// move, and asking is a caller's defect. In debug builds, where `p`
    /// lies farther from the circle, off its plane or off its radius, than
    /// `1e-6·max(r, 1)`: `1e-6` of the radius `r`, and an absolute `1e-6`
    /// below a radius of 1 (point values only; an enclosure is not checked).
    #[must_use]
    pub fn starting_at(self, p: Point3<T>) -> Self {
        let Shape::Circle(c) = self.shape else {
            unreachable!("only a circle's start moves: a line's origin is its locus")
        };
        let (centre, axis) = (c.frame().origin(), c.frame().axis());
        let w = p - centre;
        let w = w - axis * w.dot(axis);
        debug_assert!(
            {
                let off = (p - centre)
                    .dot(axis)
                    .abs()
                    .max((w.norm() - c.radius()).abs());
                match (off.point_value(), c.radius().point_value()) {
                    (Some(off), Some(r)) => off <= ON_CIRCLE * r.max(1.0),
                    _ => true,
                }
            },
            "a circle's start is a point the caller holds on it"
        );
        // `p` is on the circle (the caller's, asserted above), so `w` is
        // the radius, square to the axis: the circle's own frame turned
        // about its axis onto it, nothing decided.
        Self {
            shape: Shape::Circle(Circle::new(
                turned_to(&c.frame(), w.normalize()),
                c.radius(),
            )),
            ..self
        }
    }

    fn line(
        chart: Placed<T>,
        family: Family,
        line: geom::Line<T>,
        u: T,
        opening: Option<bool>,
    ) -> Self {
        let f = *chart.frame();
        let (sin, cos) = u.sin_cos();
        let normal = f.axis().cross(f.radial(u));
        Self {
            chart,
            family,
            shape: Shape::Line(line),
            plane: (line.origin, normal.normalize()),
            section: Section::HalfPlane {
                toward: Azimuth::of(sin, cos),
                opening,
            },
        }
    }

    /// **A plane's `u`-line through `uv`**: the line from the point `uv`
    /// names along the plane's `u_ref`, normalised as [`Self::ruling`]'s
    /// direction is, so places along it are metres. Its plane is the one
    /// through it square to the surface, of unit normal `v_ref`, which cuts
    /// the plane in this line alone ([`Section::Whole`]).
    ///
    /// # Errors
    ///
    /// [`NoSuchFamily`] on any surface but a plane.
    pub fn u_line(on: &Charted<T>, uv: Point2<T>) -> Result<Self, NoSuchFamily> {
        let surface = &on.surface;
        let chart = Self::chart(on, "u-line")?;
        if !matches!(chart.arm(), ChartArm::Plane) {
            return Err(Self::refused("u-line", surface));
        }
        let f = *chart.frame();
        let start = surface.eval(uv.x, uv.y);
        Ok(Self {
            chart,
            family: Family::ULine,
            shape: Shape::Line(geom::Line {
                origin: start,
                dir: UnitVec3::squared(f.u_ref()),
            }),
            plane: (start, f.v_ref().normalize()),
            section: Section::Whole,
        })
    }

    /// **A sphere's great circle through `uv`** along `t` there: the
    /// great circle in the plane of `t` and the radius to `uv`'s point.
    /// `t` must not be parallel to that radius (a debug assertion), as
    /// the sphere ray's unit tangent never is.
    ///
    /// # Errors
    ///
    /// [`NoSuchFamily`] on any surface but a sphere.
    pub fn great_circle(on: &Charted<T>, uv: Point2<T>, t: Vec3<T>) -> Result<Self, NoSuchFamily> {
        let surface = &on.surface;
        let chart = Self::chart(on, "great circle")?;
        let ChartArm::Sphere { radius } = chart.arm() else {
            return Err(Self::refused("great circle", surface));
        };
        let center = chart.frame().origin();
        let start = surface.eval(uv.x, uv.y);
        let w = start - center;
        // Unit whatever `t`'s tilt off the tangent plane: the circle is
        // the sphere's great circle in the plane of `w` and `t`.
        let normal = (w.normalize()).cross(t);
        let normal = normal.normalize();
        // `t` parallel to the radius spans no plane: the zero normal over
        // its zero length is poison.
        debug_assert!(
            !normal.x.is_poison(),
            "a great circle's direction runs through the sphere's centre"
        );
        Self::circle(
            chart,
            Family::GreatCircle,
            center,
            normal,
            radius,
            start,
            Section::Whole,
        )
        .ok_or_else(|| Self::refused("great circle", surface))
    }

    /// **A sphere's small circle about the axis through `pole`** (chart
    /// coordinates), through the point `through` names, run `forward`
    /// (right-handed about that axis) or back.
    ///
    /// # Errors
    ///
    /// [`NoSuchFamily`] on any surface but a sphere.
    pub fn about(
        on: &Charted<T>,
        pole: Point2<T>,
        through: Point2<T>,
        forward: bool,
    ) -> Result<Self, NoSuchFamily> {
        let surface = &on.surface;
        let chart = Self::chart(on, "circle about a pole")?;
        if !matches!(chart.arm(), ChartArm::Sphere { .. }) {
            return Err(Self::refused("circle about a pole", surface));
        }
        let c = chart.frame().origin();
        let a = surface.eval(pole.x, pole.y) - c;
        let a = a.normalize();
        let q = surface.eval(through.x, through.y);
        let center = c + a * a.dot(q - c);
        let normal = if forward { a } else { -a };
        Self::circle(
            chart,
            Family::SmallCircle,
            center,
            normal,
            (q - center).norm(),
            q,
            Section::Whole,
        )
        .ok_or_else(|| Self::refused("circle about a pole", surface))
    }

    /// The key of the record the curve's surface was charted from
    /// ([`Charted`]).
    pub fn record(&self) -> SurfaceKey {
        self.chart.record
    }

    /// The curve's family ([`direction`]).
    pub fn family(&self) -> Family {
        self.family
    }

    /// The curve's shape.
    pub fn shape(&self) -> Shape<T> {
        self.shape
    }

    /// The curve as a carrier.
    pub fn carrier(&self) -> Curve3<T> {
        match self.shape {
            Shape::Circle(c) => Curve3::Circle(c),
            Shape::Line(l) => Curve3::Line(l),
        }
    }

    /// The plane the curve lies in, a point and a unit normal: a circle's
    /// own, a ruling's through the surface's axis.
    pub fn plane(&self) -> (Point3<T>, Vec3<T>) {
        self.plane
    }

    /// Which curve of its plane's section of the surface it is.
    pub fn section(&self) -> Section<T> {
        self.section
    }

    /// **Whether `y`, a point of this curve's plane on the surface, lies
    /// on this curve's own section of it**, on `row`: `Positive` on it,
    /// `Negative` on another curve of the plane, `Zero` where the reading
    /// cannot tell them apart (the axis, a torus's top or bottom circle).
    /// The point is read through the chart's channels: its azimuth or tube
    /// direction along the curve's, and on a cone its nappe on its height
    /// first. A cone's apex lies on every generator: a point there is the
    /// curve's (`Positive`).
    ///
    /// # Errors
    ///
    /// A reading in band.
    pub fn section_of(
        &self,
        y: Point3<T>,
        row: &'static str,
        band: Band,
    ) -> Result<Sign, Indeterminate> {
        let (toward, opening) = match self.section {
            Section::HalfPlane { toward, opening } => (toward, opening),
            Section::TubeSide { toward } => (toward, None),
            Section::Whole | Section::Circle { .. } | Section::Oval { .. } => {
                return self.section.of_its_own(y, row, band);
            }
        };
        let (x, yy, h) = self.chart.frame().local(y);
        let channels = match self.chart.arm() {
            ChartArm::Cylinder { .. } => ChartChannels::Cylinder,
            ChartArm::Sphere { .. } => ChartChannels::Sphere,
            ChartArm::Torus { major, .. } => ChartChannels::Torus { major },
            ChartArm::Cone { half_angle } => {
                let mirror = match decide(row, Margin::of(h), band)? {
                    Sign::Zero => return Ok(Sign::Positive),
                    Sign::Positive => false,
                    Sign::Negative => true,
                };
                if opening.is_some_and(|opening| opening == mirror) {
                    return Ok(Sign::Negative);
                }
                let (sin, cos) = half_angle.sin_cos();
                ChartChannels::Cone { sin, cos, mirror }
            }
            ChartArm::Plane | ChartArm::Spindle { .. } => {
                unreachable!("no surface curve is built on a plane or a spindle")
            }
        };
        let want = match self.section {
            Section::TubeSide { .. } => ChartAngle::Tube,
            _ => ChartAngle::Azimuth,
        };
        let mut read = None;
        channels.read((x, yy, h), |angle, direction| {
            if angle == want {
                read = Some(direction);
            }
            T::zero()
        });
        let Some(direction) = read else {
            unreachable!("the channels read the angle the section names")
        };
        decide(row, Margin::of(direction.along(toward)), band)
    }
}

/// **An edge on a surface**: a span of a face's boundary edge, on the
/// surface of that face. It is read only against a [`SurfaceCurve`] on
/// the same surface ([`OnSurface::across`]), never as a bare
/// [`super::Span`].
#[derive(Clone, Copy, Debug)]
pub struct OnSurface<'a, T: Real> {
    on: SurfaceKey,
    carrier: &'a Curve3<T>,
    span: (T, T),
}

/// **Why an edge gives no reading against a curve**
/// ([`OnSurface::across`]).
#[derive(Clone, Debug)]
pub enum AcrossRefusal<T: Real> {
    /// The crossing reading left windows: the meetings it decided outside
    /// them, and the stretches it could not read
    /// ([`crate::curve_face::CrossRefusal`]).
    Windows(CrossRefusal<T>),
    /// The edge's face is on another record than the one the curve was
    /// charted from: the pair is not known to be co-surface, so the arm
    /// does not read it.
    OffRecord {
        /// The edge's face's record.
        edge: SurfaceKey,
        /// The curve's record.
        curve: SurfaceKey,
    },
    /// The edge lies in no one plane (a line), so no curve is read across
    /// its plane.
    NoPlane,
    /// The direction table reads the pair the other way ([`direction`]):
    /// the curve across the edge's plane, which [`OnSurface::meet`]
    /// reads.
    AgainstTheTable,
}

/// **A meeting of an edge and a curve on one surface**
/// ([`OnSurface::meet`]): on both curves' own sections of their planes,
/// placed on the edge's span.
#[derive(Clone, Copy, Debug)]
pub struct CoMeeting<T: Real> {
    /// The meeting's point.
    pub point: Point3<T>,
    /// Its place on the edge's span: inside at the edge's parameter, or
    /// at an end.
    pub on_edge: OnSpan<T>,
}

/// **Where an edge on a surface meets a curve on it, read by the direction
/// table** ([`OnSurface::meet`]).
#[derive(Debug)]
pub enum Met<T: Real> {
    /// The two lie in one plane: the caller reads whether the edge is on
    /// the curve's own section of it ([`SurfaceCurve::section_of`]).
    InPlane,
    /// The read curve touches the other's plane: a double root, no
    /// crossing.
    Touch,
    /// The meetings, in the read curve's order.
    Meets(Vec<CoMeeting<T>>),
}

/// **Why [`OnSurface::meet`] gave no reading.**
#[derive(Clone, Debug)]
pub enum MeetRefusal<T: Real> {
    /// The crossing reading left windows, in the parameter of the curve
    /// `read` names as read across the other's plane: the meetings it
    /// decided outside them, on both sections and placed on the edge.
    Windows {
        /// Which curve was read across the other's plane.
        read: Read,
        /// The meetings decided outside every window.
        found: Vec<CoMeeting<T>>,
        /// The stretches not read.
        windows: Vec<crate::curve_face::Undecided<T, crate::curve_face::CrossWhy>>,
    },
    /// A meeting's section of its plane read in band (`Some`), or could
    /// not be told from the plane's other curve (`None`).
    Section(Option<Indeterminate>),
    /// A meeting's place on the edge's span read in band.
    Place(Indeterminate),
    /// The pair is on two records ([`AcrossRefusal::OffRecord`]).
    OffRecord {
        /// The edge's face's record.
        edge: SurfaceKey,
        /// The curve's record.
        curve: SurfaceKey,
    },
}

/// **Where an edge on a surface crosses a curve on it**
/// ([`OnSurface::across`]).
#[derive(Debug)]
pub enum CoSurface<T: Real> {
    /// The edge lies in the curve's plane: on the curve's own section or
    /// another curve of the plane, which the caller reads
    /// ([`SurfaceCurve::section_of`]) where it needs to.
    InPlane,
    /// The edge touches the curve's plane: a double root, no crossing.
    Touch,
    /// The edge's meetings with the curve's plane, ascending along the
    /// edge: one is a meeting of the pair where it lies on the curve's own
    /// section of the plane ([`SurfaceCurve::section_of`]), which the
    /// caller reads, after what it reads first (a pole, an end).
    Roots(Vec<Meeting<T>>),
}

impl<'a, T: Decide> OnSurface<'a, T> {
    /// The span `(t0, t1)` of `carrier`, an edge of a face whose surface
    /// record is `on`. Tier 3 places each edge on its faces' carriers at
    /// rest, so the edge lies on that record's surface, and it is read
    /// only against a curve charted from the same record
    /// ([`Self::across`]).
    pub fn edge(on: SurfaceKey, carrier: &'a Curve3<T>, span: (T, T)) -> Self {
        Self { on, carrier, span }
    }

    /// The edge's carrier.
    pub fn carrier(&self) -> &'a Curve3<T> {
        self.carrier
    }

    /// The edge's span.
    pub fn span(&self) -> (T, T) {
        self.span
    }

    /// **The edge across `curve`'s plane** (module docs), where the
    /// direction table reads the pair that way ([`direction`]): the
    /// crossing reading of the edge's span against the plane
    /// ([`crate::curve_face::cross()`]), its pose deciding an edge in the
    /// plane, touching it, or apart from it; its meetings in the edge's
    /// own parameter.
    ///
    /// # Errors
    ///
    /// [`AcrossRefusal::OffRecord`] where the edge's record is not the
    /// curve's; [`AcrossRefusal::AgainstTheTable`] where the table reads
    /// the curve across the edge's plane; [`AcrossRefusal::Windows`] where
    /// the crossing reading left a stretch of the edge unread.
    pub fn across(
        &self,
        curve: &SurfaceCurve<T>,
        band: Band,
    ) -> Result<CoSurface<T>, AcrossRefusal<T>> {
        self.same_record(curve)?;
        if direction(curve.family(), self.carrier.kind()) != Read::EdgeOverLeg {
            return Err(AcrossRefusal::AgainstTheTable);
        }
        let (origin, normal) = curve.plane();
        // The plane is charted along the curve: a circle's reference, or a
        // line's direction, each in its plane.
        let u_ref = match curve.shape {
            Shape::Circle(c) => c.frame().u_ref(),
            Shape::Line(l) => l.dir.get(),
        };
        // A circle's seam and a line's direction lie in the curve's own
        // plane, square to its normal by the curve's frame: squared about
        // that normal, nothing decided.
        let plane = geom::Surface::Plane {
            frame: Frame::of_motion(&RigidMotion::squared_about(
                origin,
                UnitVec3::squared(normal),
                u_ref,
            )),
        };
        co_surface(&super::Span::new(self.carrier, self.span), &plane, band)
    }

    /// **The plane the edge's carrier lies in**, as a frame whose axis is
    /// its unit normal: a conic's own frame, its axes as stored, a spiric's cutting
    /// plane with the spiric's `u_ref` kept bit for bit as its axis. A line
    /// lies in no one plane (`None`).
    pub fn plane(&self) -> Option<Frame<T>> {
        match *self.carrier {
            Curve3::Circle(_) | Curve3::Ellipse(_) => self.carrier.frame(),
            Curve3::Spiric { frame, offset, .. } => {
                let (center, u_ref) = (frame.origin(), frame.u_ref());
                Some(Frame::of_motion(&geom_core::RigidMotion::squared_about(
                    center + u_ref * offset,
                    frame.u_ref_unit(),
                    frame.axis(),
                )))
            }
            _ => None,
        }
    }

    /// **`curve` over `span` across the edge's plane** ([`Self::plane`]):
    /// the pair read the other way, by the crossing reading of the curve's
    /// span against that plane, in the curve's own parameter. A whole
    /// turn (`endless`) has no ends. [`Self::meet`] places each meeting on
    /// the edge: the plane cuts the surface in the edge's curve and
    /// others.
    ///
    /// # Errors
    ///
    /// [`AcrossRefusal::NoPlane`] for an edge in no one plane;
    /// [`AcrossRefusal::Windows`] where the reading left a stretch of the
    /// curve unread.
    fn under(
        &self,
        curve: &SurfaceCurve<T>,
        span: (T, T),
        endless: bool,
        band: Band,
    ) -> Result<CoSurface<T>, AcrossRefusal<T>> {
        let frame = self.plane().ok_or(AcrossRefusal::NoPlane)?;
        let plane = geom::Surface::Plane { frame };
        let carrier = curve.carrier();
        let span = super::Span::new(&carrier, span);
        let span = if endless { span.endless() } else { span };
        co_surface(&span, &plane, band)
    }

    /// **The edge's own section of its plane** ([`Section`]): a circle's
    /// own of the plane's curves, a spiric's oval on its side of the
    /// axis's projection; the whole section for any other edge.
    pub fn section(&self) -> Section<T> {
        match *self.carrier {
            Curve3::Circle(c) => Section::Circle {
                centre: c.frame().origin(),
                radius: c.radius(),
            },
            Curve3::Spiric { frame, offset, .. } => {
                let (center, axis, u_ref) = (frame.origin(), frame.axis(), frame.u_ref());
                Section::Oval {
                    foot: center + u_ref * offset,
                    side: axis.cross(u_ref),
                }
            }
            Curve3::Line(_) | Curve3::Ellipse(_) | Curve3::Nurbs(_) | Curve3::Periodic(_) => {
                Section::Whole
            }
        }
    }

    /// **Where the edge meets `curve`**, read in the direction the table
    /// fixes for the pair ([`direction`]): one curve across the other's
    /// plane by the crossing reading.
    ///
    /// Each meeting is kept where it lies on both curves' own sections of
    /// their planes: the read curve's by construction, the other's decided
    /// on `row`. It is then placed on the edge's span by the span's own
    /// reading, and a meeting past the span is none. `span` gives the
    /// curve's span and whether it is a whole turn with no ends; it is
    /// asked only where the curve is read across the edge's plane.
    ///
    /// # Errors
    ///
    /// [`MeetRefusal`], as its variants say.
    pub fn meet(
        &self,
        curve: &SurfaceCurve<T>,
        span: impl FnOnce() -> ((T, T), bool),
        row: &'static str,
        band: Band,
    ) -> Result<Met<T>, MeetRefusal<T>> {
        self.same_record(curve)
            .map_err(|_| MeetRefusal::OffRecord {
                edge: self.on,
                curve: curve.record(),
            })?;
        let read = direction(curve.family(), self.carrier.kind());
        let reading = match read {
            Read::EdgeOverLeg => self.across(curve, band),
            Read::LegOverEdge => {
                let (span, endless) = span();
                self.under(curve, span, endless, band)
            }
        };
        let keep = |meetings: Vec<Meeting<T>>| -> Result<Vec<CoMeeting<T>>, MeetRefusal<T>> {
            let mut out = Vec::new();
            for m in meetings {
                let own = match read {
                    Read::EdgeOverLeg => curve.section_of(m.point, row, band),
                    Read::LegOverEdge => self.section().of_its_own(m.point, row, band),
                };
                match own.map_err(|diag| MeetRefusal::Section(Some(diag)))? {
                    Sign::Positive => {}
                    Sign::Negative => continue,
                    Sign::Zero => return Err(MeetRefusal::Section(None)),
                }
                let on_edge = match read {
                    Read::EdgeOverLeg => m.on_span,
                    Read::LegOverEdge => match self.place(m.point, band)? {
                        Some(on_edge) => on_edge,
                        None => continue,
                    },
                };
                out.push(CoMeeting {
                    point: m.point,
                    on_edge,
                });
            }
            Ok(out)
        };
        match reading {
            Ok(CoSurface::InPlane) => Ok(Met::InPlane),
            Ok(CoSurface::Touch) => Ok(Met::Touch),
            Ok(CoSurface::Roots(meetings)) => Ok(Met::Meets(keep(meetings)?)),
            Err(AcrossRefusal::Windows(refusal)) => Err(MeetRefusal::Windows {
                read,
                found: keep(refusal.found)?,
                windows: refusal.windows,
            }),
            Err(AcrossRefusal::OffRecord { edge, curve }) => {
                Err(MeetRefusal::OffRecord { edge, curve })
            }
            Err(AcrossRefusal::NoPlane | AcrossRefusal::AgainstTheTable) => unreachable!(
                "the table reads a curve across an edge's plane only for a circle, an ellipse \
                 or a spiric edge"
            ),
        }
    }

    /// **Where `y`, a point of the edge's curve, lies on its span**: its
    /// parameter within half a turn of the span's middle, the carrier's
    /// own reading ([`Curve3::param_near`]), read against the span by its
    /// own reading
    /// ([`super::Span::place_param`]); `None` past it.
    fn place(&self, y: Point3<T>, band: Band) -> Result<Option<OnSpan<T>>, MeetRefusal<T>> {
        let (t0, t1) = self.span;
        let mid = geom::mid_param(t0, t1);
        let Some(t) = self.carrier.param_near(y, mid) else {
            unreachable!(
                "the table reads a curve across a circle's, an ellipse's or a spiric's plane"
            )
        };
        Ok(
            match super::Span::new(self.carrier, self.span)
                .place_param(t, band)
                .map_err(MeetRefusal::Place)?
            {
                Place::Inside => Some(OnSpan::Inside { t }),
                Place::End(end) => Some(OnSpan::End(end)),
                Place::Past => None,
            },
        )
    }

    /// The pair's records agree, or the arm refuses it.
    fn same_record(&self, curve: &SurfaceCurve<T>) -> Result<(), AcrossRefusal<T>> {
        if self.on == curve.record() {
            Ok(())
        } else {
            Err(AcrossRefusal::OffRecord {
                edge: self.on,
                curve: curve.record(),
            })
        }
    }
}

/// The crossing reading of `span` against `plane`, as the pair's reading
/// ([`CoSurface`]).
fn co_surface<T: Decide>(
    span: &super::Span<'_, T>,
    plane: &geom::Surface<T>,
    band: Band,
) -> Result<CoSurface<T>, AcrossRefusal<T>> {
    let read = crate::curve_face::cross(span, plane, band).map_err(AcrossRefusal::Windows)?;
    Ok(match read.pose {
        Pose::Lies => CoSurface::InPlane,
        Pose::Tangent(_) => CoSurface::Touch,
        Pose::Apart | Pose::Isolated(_) | Pose::Through(_) => CoSurface::Roots(read.meetings),
    })
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod tests {
    use super::*;
    use core::f64::consts::{FRAC_PI_2, FRAC_PI_4, PI};

    fn b() -> Band {
        Band::new(1e-9, 1e-8).unwrap()
    }

    const ROW: &str = "test_on_surface_section";

    /// The one record every fixture surface is charted from.
    fn key() -> SurfaceKey {
        let mut keys = slotmap::SlotMap::<SurfaceKey, ()>::with_key();
        keys.insert(())
    }

    fn on(surface: &geom::Surface<f64>) -> Charted<f64> {
        Charted::record(key(), surface)
    }

    fn z() -> Vec3<f64> {
        Vec3::new(0.0, 0.0, 1.0)
    }

    fn x() -> Vec3<f64> {
        Vec3::new(1.0, 0.0, 0.0)
    }

    fn cylinder() -> geom::Surface<f64> {
        geom::Surface::Cylinder {
            frame: geom::Frame::new(Point3::origin(), z(), x()),
            radius: 2.0,
        }
    }

    /// **A circle through its own centre is no such family, not a
    /// poisoned carrier**: a cone's parallel at the apex (`v = 0`)
    /// starts where it is centred, so its seam has no direction. The
    /// circle's decided mint refuses it as no parallel there, where the
    /// undecided square minted a frame of poison. A parallel off the apex
    /// mints, its seam toward its start. Red with the undecided square in
    /// the circle's mint (the apex parallel then answers `Ok`, its frame
    /// poisoned).
    #[test]
    fn a_parallel_at_the_cones_apex_is_no_such_family() {
        let cone = cone();
        let at_apex = SurfaceCurve::parallel(&on(&cone), Point2::new(0.3, 0.0), true);
        assert!(
            matches!(
                at_apex,
                Err(NoSuchFamily {
                    family: "parallel",
                    ..
                })
            ),
            "the apex holds no parallel: {:?}",
            at_apex.as_ref().map(|c| c.carrier())
        );
        let off = SurfaceCurve::parallel(&on(&cone), Point2::new(0.3, 1.0), true).unwrap();
        let geom::Curve3::Circle(c) = off.carrier() else {
            panic!("a parallel is a circle")
        };
        assert!(
            [c.frame().axis(), c.frame().u_ref()]
                .iter()
                .all(|v| v.x.is_finite() && v.y.is_finite() && v.z.is_finite()),
            "a parallel off the apex mints: {c:?}"
        );
    }

    fn cone() -> geom::Surface<f64> {
        geom::Surface::Cone {
            frame: geom::Frame::new(Point3::origin(), z(), x()),
            half_angle: FRAC_PI_4,
        }
    }

    fn plane() -> geom::Surface<f64> {
        geom::Surface::Plane {
            frame: geom::Frame::new(Point3::new(0.5, -1.0, 2.0), z(), Vec3::new(0.6, 0.8, 0.0)),
        }
    }

    fn sphere() -> geom::Surface<f64> {
        geom::Surface::Sphere {
            frame: geom::Frame::new(Point3::new(1.0, -2.0, 0.5), z(), x()),
            radius: 3.0,
        }
    }

    fn torus() -> geom::Surface<f64> {
        geom::Surface::Torus {
            frame: geom::Frame::new(Point3::origin(), z(), x()),
            major_radius: 5.0,
            minor_radius: 1.5,
        }
    }

    /// The largest distance of a sample of `curve` from `surface`, read
    /// through the surface's chart: the foot's point against the sample.
    fn off_surface(surface: &geom::Surface<f64>, curve: &SurfaceCurve<f64>) -> f64 {
        let carrier = curve.carrier();
        (0..16)
            .map(|i| {
                let p = carrier.eval(f64::from(i) * 0.4 - 3.0);
                let uv = surface.chart_point(p, b()).unwrap().uv;
                (surface.eval(uv.x, uv.y) - p).norm()
            })
            .fold(0.0, f64::max)
    }

    /// **Every family lies on its surface**: the parallels of a cylinder,
    /// a cone and a torus, the meridians of a torus and a sphere, the
    /// rulings of a cylinder and a cone, a cone's generator from its
    /// apex, and a sphere's great and small circles, each built from a
    /// surface record and chart coordinates and read back through the
    /// surface's chart at sixteen points.
    #[test]
    fn every_family_lies_on_its_surface() {
        let uv = Point2::new(0.7, 1.3);
        let built = [
            (
                cylinder(),
                SurfaceCurve::parallel(&on(&cylinder()), uv, true),
            ),
            (cone(), SurfaceCurve::parallel(&on(&cone()), uv, false)),
            (torus(), SurfaceCurve::parallel(&on(&torus()), uv, true)),
            (torus(), SurfaceCurve::meridian(&on(&torus()), uv, false)),
            (sphere(), SurfaceCurve::meridian(&on(&sphere()), uv, true)),
            (
                cylinder(),
                SurfaceCurve::ruling(&on(&cylinder()), uv, true, false),
            ),
            (cone(), SurfaceCurve::ruling(&on(&cone()), uv, true, true)),
            (cone(), SurfaceCurve::from_apex(&on(&cone()), uv, true)),
            (
                sphere(),
                SurfaceCurve::great_circle(&on(&sphere()), Point2::new(0.7, 0.3), z()),
            ),
            (
                sphere(),
                SurfaceCurve::about(&on(&sphere()), Point2::new(0.2, 0.4), uv, true),
            ),
            (plane(), SurfaceCurve::u_line(&on(&plane()), uv)),
        ];
        for (i, (surface, curve)) in built.iter().enumerate() {
            let off = off_surface(surface, curve.as_ref().unwrap());
            assert!(off < 1e-12, "family {i} lies {off:e} off its surface");
        }
    }

    /// **A family is built only on a surface that has it**: no parallel of
    /// a sphere (its circles about a pole are [`SurfaceCurve::about`]), no
    /// meridian of a cylinder, no ruling of a torus.
    #[test]
    fn a_family_is_refused_on_a_surface_without_it() {
        let uv = Point2::new(0.0, 0.5);
        assert!(SurfaceCurve::parallel(&on(&sphere()), uv, true).is_err());
        assert!(SurfaceCurve::meridian(&on(&cylinder()), uv, true).is_err());
        assert!(SurfaceCurve::ruling(&on(&torus()), uv, true, true).is_err());
        assert!(SurfaceCurve::from_apex(&on(&cylinder()), uv, true).is_err());
        assert!(SurfaceCurve::great_circle(&on(&torus()), uv, z()).is_err());
        assert!(SurfaceCurve::u_line(&on(&cylinder()), uv).is_err());
        assert!(SurfaceCurve::parallel(&on(&plane()), uv, true).is_err());
    }

    /// **A cone's generator reads its own half-generator of its plane**:
    /// the plane through the axis at azimuth `0` cuts the cone in four
    /// half-generators; a point on the generator's own is `Positive`, one
    /// on the mirror nappe or the opposite azimuth `Negative`, and the
    /// apex, on every generator, `Positive`.
    #[test]
    fn a_cone_generator_reads_its_own_half_generator() {
        let cone = cone();
        let g = SurfaceCurve::ruling(&on(&cone), Point2::new(0.0, 2.0), true, true).unwrap();
        let read = |p: Point3<f64>| g.section_of(p, ROW, b()).unwrap();
        assert_eq!(read(Point3::new(1.0, 0.0, 1.0)), Sign::Positive, "its own");
        assert_eq!(
            read(Point3::new(-1.0, 0.0, -1.0)),
            Sign::Negative,
            "mirror nappe, through the apex"
        );
        assert_eq!(
            read(Point3::new(-1.0, 0.0, 1.0)),
            Sign::Negative,
            "the opposite azimuth"
        );
        assert_eq!(
            read(Point3::new(1.0, 0.0, -1.0)),
            Sign::Negative,
            "mirror nappe, same side"
        );
        assert_eq!(read(Point3::origin()), Sign::Positive, "the apex");
    }

    /// **A torus's parallel crosses a meridian's plane twice**: a parallel
    /// of the outer equator crosses the meridian plane at azimuth `0` at
    /// `±x`, the root at `+x` on the meridian's own half-plane and the one
    /// at `−x` on its mirror. A parallel arc short of the plane meets it
    /// nowhere, and the meridian's own circle lies in it.
    #[test]
    fn a_torus_parallel_crosses_a_meridians_plane_twice() {
        let torus = torus();
        let meridian = SurfaceCurve::meridian(&on(&torus), Point2::new(0.0, 0.3), true).unwrap();
        let parallel =
            SurfaceCurve::parallel(&on(&torus), Point2::new(FRAC_PI_2, 0.0), true).unwrap();
        let carrier = parallel.carrier();
        let CoSurface::Roots(roots) = OnSurface::edge(key(), &carrier, (-1.0, 2.0 * PI - 1.0))
            .across(&meridian, b())
            .unwrap()
        else {
            unreachable!("a parallel crosses a meridian's plane")
        };
        assert_eq!(roots.len(), 2);
        let mut sides: Vec<(f64, Sign)> = roots
            .iter()
            .map(|m| (m.point.x, meridian.section_of(m.point, ROW, b()).unwrap()))
            .collect();
        sides.sort_by(|a, b| a.0.total_cmp(&b.0));
        assert!((sides[0].0 + 6.5).abs() < 1e-12 && (sides[1].0 - 6.5).abs() < 1e-12);
        assert_eq!((sides[0].1, sides[1].1), (Sign::Negative, Sign::Positive));
        let short = OnSurface::edge(key(), &carrier, (0.2, 1.0))
            .across(&meridian, b())
            .unwrap();
        assert!(
            matches!(short, CoSurface::Roots(ref r) if r.is_empty()),
            "{short:?}"
        );
        let own = meridian.carrier();
        let in_plane = OnSurface::edge(key(), &own, (0.0, 1.0))
            .across(&meridian, b())
            .unwrap();
        assert!(matches!(in_plane, CoSurface::InPlane), "{in_plane:?}");
    }

    /// **An edge of a face on another record is not read**: a parallel
    /// charted from one record against an arc of a face on another
    /// refuses as off its record, whatever the geometry says; on its own
    /// record the same pair reads its crossings.
    #[test]
    fn an_edge_on_another_record_refuses() {
        let mut keys = slotmap::SlotMap::<SurfaceKey, ()>::with_key();
        let (mine, other) = (keys.insert(()), keys.insert(()));
        let torus = torus();
        let meridian =
            SurfaceCurve::meridian(&Charted::record(mine, &torus), Point2::new(0.0, 0.3), true)
                .unwrap();
        let parallel = SurfaceCurve::parallel(
            &Charted::record(mine, &torus),
            Point2::new(FRAC_PI_2, 0.0),
            true,
        )
        .unwrap();
        let carrier = parallel.carrier();
        let span = (-1.0, 2.0 * PI - 1.0);
        assert!(matches!(
            OnSurface::edge(other, &carrier, span)
                .across(&meridian, b())
                .unwrap_err(),
            AcrossRefusal::OffRecord { edge, curve } if edge == other && curve == mine
        ));
        assert!(matches!(
            OnSurface::edge(mine, &carrier, span).across(&meridian, b()),
            Ok(CoSurface::Roots(ref roots)) if roots.len() == 2
        ));
        // The table's reading, the parallel across the meridian edge's
        // plane, is refused alike: no reading of the pair skips the check.
        let edge = meridian.carrier();
        let whole = || ((0.0, 2.0 * PI), true);
        assert!(matches!(
            OnSurface::edge(other, &edge, (-1.0, 1.0))
                .meet(&parallel, whole, ROW, b())
                .unwrap_err(),
            MeetRefusal::OffRecord { edge, curve } if edge == other && curve == mine
        ));
        assert!(matches!(
            OnSurface::edge(mine, &edge, (-1.0, 1.0)).meet(&parallel, whole, ROW, b()),
            Ok(Met::Meets(ref meets)) if meets.len() == 1
        ));
    }

    /// The torus's crest probe: `v = π/2 − d`, `d` from `1e-1` to `1e-6`.
    const CREST: [f64; 6] = [1e-1, 1e-2, 1e-3, 1e-4, 1e-5, 1e-6];

    /// One meeting, at `want`, inside the edge's span.
    fn lands_once(label: &str, got: Result<Met<f64>, MeetRefusal<f64>>, want: Point3<f64>) {
        let Ok(Met::Meets(meets)) = got else {
            panic!("{label}: {got:?}");
        };
        assert_eq!(meets.len(), 1, "{label}: {meets:?}");
        let m = meets[0];
        assert!(
            matches!(m.on_edge, crate::curve_face::OnSpan::Inside { .. }),
            "{label}: {m:?}"
        );
        assert!(
            (m.point - want).norm() < 1e-9,
            "{label}: {m:?}, want {want:?}"
        );
    }

    /// **A meeting past the edge's span is none** ([`OnSurface::meet`]'s
    /// placement): a parallel leg read across a meridian edge's plane
    /// meets the meridian's circle at its tube angle, which lands inside
    /// one span of the edge and past another.
    #[test]
    fn a_meeting_past_the_edges_span_is_none() {
        let (big, minor, v) = (5.0, 1.5, 0.7);
        let torus = torus();
        let on = on(&torus);
        let meridian = SurfaceCurve::meridian(&on, Point2::new(0.0, 0.0), true).unwrap();
        let edge = meridian.carrier();
        let parallel = SurfaceCurve::parallel(&on, Point2::new(0.0, v), true).unwrap();
        let whole = || ((-PI, PI), true);
        lands_once(
            "a span holding the meeting",
            OnSurface::edge(key(), &edge, (0.5, 1.0)).meet(&parallel, whole, ROW, b()),
            Point3::new(big + minor * v.cos(), 0.0, minor * v.sin()),
        );
        let past = OnSurface::edge(key(), &edge, (1.0, 2.5)).meet(&parallel, whole, ROW, b());
        assert!(
            matches!(past, Ok(Met::Meets(ref m)) if m.is_empty()),
            "a span past the meeting: {past:?}"
        );
    }

    /// **The crest probe** ([`direction`]'s torus rows): a parallel leg
    /// near the tube's crest, whose plane is all but tangent to the torus
    /// along the leg, lands once on a meridian edge and once on an oval,
    /// read across the edge's plane; and a meridian leg lands once on a
    /// parallel edge there, the edge read across the meridian's plane.
    /// Read the other way, each pair crosses a plane all but tangent to
    /// the torus at the meeting, which the band cannot read.
    #[test]
    fn the_crest_probe_lands_at_every_depth() {
        let (big, minor) = (5.0, 1.5);
        let torus = torus();
        let on = on(&torus);
        let oval_offset = 2.0;
        let oval =
            Curve3::spiric(Point3::origin(), z(), x(), big, minor, oval_offset, b()).unwrap();
        // The meridian edge at azimuth 0, its parameter counted from the
        // outer equator.
        let meridian = SurfaceCurve::meridian(&on, Point2::new(0.0, 0.0), true).unwrap();
        let meridian_edge = meridian.carrier();
        for d in CREST {
            let v = FRAC_PI_2 - d;
            let (rho, h) = (big + minor * v.cos(), minor * v.sin());
            let parallel = SurfaceCurve::parallel(&on, Point2::new(0.0, v), true).unwrap();
            let whole = || ((-PI, PI), true);
            lands_once(
                &format!("d {d}: a parallel on a meridian"),
                OnSurface::edge(key(), &meridian_edge, (-0.5, 2.5)).meet(
                    &parallel,
                    whole,
                    ROW,
                    b(),
                ),
                Point3::new(rho, 0.0, h),
            );
            lands_once(
                &format!("d {d}: a parallel on an oval"),
                OnSurface::edge(key(), &oval, (-0.5, 2.5)).meet(&parallel, whole, ROW, b()),
                Point3::new(
                    oval_offset,
                    (rho * rho - oval_offset * oval_offset).sqrt(),
                    h,
                ),
            );
            let edge = parallel.carrier();
            lands_once(
                &format!("d {d}: a meridian on a parallel"),
                OnSurface::edge(key(), &edge, (-1.0, 1.0)).meet(&meridian, whole, ROW, b()),
                Point3::new(rho, 0.0, h),
            );
        }
    }

    #[test]
    fn a_turned_record_is_the_same_surface_on_another_chart() {
        let torus = torus();
        let turned = Charted::record(key(), &torus).turned(Vec3::new(0.0, 1.0, 0.0));
        let geom::Surface::Torus {
            frame: carrier_frame,
            ..
        } = *turned.surface()
        else {
            unreachable!()
        };
        let u_ref = carrier_frame.u_ref();
        assert_eq!((u_ref.x, u_ref.y, u_ref.z), (0.0, 1.0, 0.0));
        // A point of the turned chart lies on the record's surface.
        let w = turned.surface().eval(0.4, 0.9) - Point3::origin();
        let rho = (w.x * w.x + w.y * w.y).sqrt();
        assert!(
            ((rho - 5.0).powi(2) + w.z * w.z - 1.5 * 1.5).abs() < 1e-12,
            "{w:?}"
        );
        assert_eq!(turned.key(), key());
    }

    /// **A plane's line crosses a circle of the plane where the circle
    /// crosses the line's square plane**: the `u`-line along `x` through
    /// the origin meets the unit circle about `(0, 0.5)` at `x = ±√0.75`,
    /// both on the line's one section; a circle clear of the line meets
    /// it nowhere, and one tangent to it touches.
    #[test]
    fn a_planes_line_crosses_a_circle_of_the_plane_twice() {
        let plane = geom::Surface::Plane {
            frame: geom::Frame::new(Point3::origin(), z(), x()),
        };
        let line = SurfaceCurve::u_line(&on(&plane), Point2::new(0.0, 0.0)).unwrap();
        let circle = |cy: f64| geom::Curve3::circle(Point3::new(0.0, cy, 0.0), z(), 1.0, x());
        let near = circle(0.5);
        let CoSurface::Roots(roots) = OnSurface::edge(key(), &near, (-PI, PI))
            .across(&line, b())
            .unwrap()
        else {
            unreachable!("a circle through the line crosses its plane")
        };
        let mut xs: Vec<f64> = roots.iter().map(|m| m.point.x).collect();
        xs.sort_by(f64::total_cmp);
        assert_eq!(xs.len(), 2);
        assert!((xs[0] + 0.75f64.sqrt()).abs() < 1e-12 && (xs[1] - 0.75f64.sqrt()).abs() < 1e-12);
        for m in &roots {
            assert_eq!(line.section_of(m.point, ROW, b()).unwrap(), Sign::Positive);
        }
        let clear = circle(2.0);
        let apart = OnSurface::edge(key(), &clear, (-PI, PI))
            .across(&line, b())
            .unwrap();
        assert!(
            matches!(apart, CoSurface::Roots(ref r) if r.is_empty()),
            "{apart:?}"
        );
        let tangent = circle(1.0);
        let touch = OnSurface::edge(key(), &tangent, (-PI, PI))
            .across(&line, b())
            .unwrap();
        assert!(matches!(touch, CoSurface::Touch), "{touch:?}");
    }

    /// **An edge's plane is its carrier's as stored** ([`OnSurface::plane`]):
    /// a circle edge a megametre out, on frames whose axes are unit as
    /// stored, hands its plane over bit for bit. Re-squaring a unit axis
    /// changes its bits about a third of the time, and an ulp of tilt moves
    /// the plane by `1e-10` across a megametre, past the zero band at
    /// ε `1e-12`.
    #[test]
    fn an_edges_plane_is_its_carriers_frame_as_stored() {
        let mut moved = 0;
        for i in 0..64 {
            let t = f64::from(i) * 0.37;
            let axis = Vec3::new(t.cos() * 0.6, t.sin() * 0.6, 0.8 - 0.01 * t);
            let frame = Frame::new(
                Point3::new(1e6 + t, -2e6 + 3.0 * t, 5e5),
                axis,
                Vec3::new(-axis.y, axis.x, 0.0),
            );
            let carrier = Curve3::Circle(Circle::new(frame, 2.0));
            let plane = OnSurface::edge(key(), &carrier, (0.0, 1.0))
                .plane()
                .unwrap();
            let bits = |v: Vec3<f64>| [v.x, v.y, v.z].map(f64::to_bits);
            assert_eq!(
                bits(plane.axis()),
                bits(frame.axis()),
                "frame {i}: the axis"
            );
            assert_eq!(
                bits(plane.origin() - Point3::origin()),
                bits(frame.origin() - Point3::origin()),
                "frame {i}: the origin"
            );
            let squared = Frame::new(frame.origin(), frame.axis(), frame.u_ref());
            if bits(squared.axis()) != bits(frame.axis()) {
                moved += 1;
            }
        }
        // The witness the row guards against: re-squaring moves some axes.
        assert!(moved > 0, "no stored axis moves when squared again");
    }
}
