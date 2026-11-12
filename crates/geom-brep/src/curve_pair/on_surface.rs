//! **Two carriers on one surface** (C5's co-surface arm). A curve built on
//! a surface ([`SurfaceCurve`]) and an edge of a face on it ([`OnSurface`])
//! lie on one surface record, so their gap is zero in the model: a 3D
//! curve-pair reading ([`super::meet`]) of the pair decides only the
//! construction's rounding against the band. They meet instead where one
//! crosses the other's plane, through the plane door
//! ([`crate::curve_face::plane_crossing::span_plane_meet`], whose harmonic
//! is [`super::pose::height`], the one [`super::meet`]'s transverse
//! reading is built on), at a root that lies on the curve's own section of
//! its plane ([`Section`]).
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
//! its face's record ([`OnSurface::edge`]), and both readings of the pair,
//! [`OnSurface::across`] and [`OnSurface::under`], refuse a pair on two
//! records ([`AcrossRefusal::OffRecord`]). That the key and the surface
//! a caller hands [`Charted::record`] are one record's stays the
//! caller's: each reads both off one face.

use geom::{Azimuth, ChartAngle, ChartArm, ChartChannels, ChartInverse, Circle, Curve3, Frame};
use geom_core::k_stats::decide;
use geom_core::{Band, Decide, Indeterminate, Margin, Point2, Point3, Real, Sign, Vec3};

use crate::curve_face::plane_crossing::{
    ConicRootFault, SpanEnds, SpanPlaneMeet, SpanRoot, span_plane_meet,
};

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
/// edges of faces on that record ([`OnSurface::across`],
/// [`OnSurface::under`]).
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
                origin,
                axis,
                radius,
                ..
            } => geom::Surface::Cylinder {
                origin,
                axis,
                radius,
                u_ref,
            },
            geom::Surface::Cone {
                apex,
                axis,
                half_angle,
                ..
            } => geom::Surface::Cone {
                apex,
                axis,
                half_angle,
                u_ref,
            },
            geom::Surface::Sphere {
                center,
                radius,
                axis,
                ..
            } => geom::Surface::Sphere {
                center,
                radius,
                axis,
                u_ref,
            },
            geom::Surface::Torus {
                center,
                axis,
                major_radius,
                minor_radius,
                ..
            } => geom::Surface::Torus {
                center,
                axis,
                major_radius,
                minor_radius,
                u_ref,
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

    fn circle(
        chart: Placed<T>,
        center: Point3<T>,
        normal: Vec3<T>,
        radius: T,
        start: Point3<T>,
        section: Section<T>,
    ) -> Self {
        let w = start - center;
        Self {
            chart,
            shape: Shape::Circle(Circle::new(
                Frame::new(center, normal, w.normalize()),
                radius,
            )),
            plane: (center, normal),
            section,
        }
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
        let normal = if forward { f.axis } else { -f.axis };
        let start = surface.eval(uv.x, uv.y);
        let (center, radius, section) = match chart.arm() {
            ChartArm::Cylinder { radius } => (f.origin + f.axis * uv.y, radius, Section::Whole),
            ChartArm::Cone { half_angle } => {
                let (sin, cos) = half_angle.sin_cos();
                (
                    f.origin + f.axis * (uv.y * cos),
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
                    f.origin + f.axis * (minor * sin),
                    major + minor * cos,
                    Section::TubeSide {
                        toward: Azimuth::of(T::zero(), side),
                    },
                )
            }
            _ => return Err(Self::refused("parallel", surface)),
        };
        Ok(Self::circle(chart, center, normal, radius, start, section))
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
        let normal = radial.cross(f.axis);
        let normal = if forward { normal } else { -normal };
        let start = surface.eval(uv.x, uv.y);
        let (sin_u, cos_u) = uv.x.sin_cos();
        let section = Section::HalfPlane {
            toward: Azimuth::of(sin_u, cos_u),
            opening: None,
        };
        let (center, radius) = match chart.arm() {
            ChartArm::Torus { major, minor } => (f.origin + radial * major, minor),
            ChartArm::Sphere { radius } => (f.origin, radius),
            _ => return Err(Self::refused("meridian", surface)),
        };
        Ok(Self::circle(chart, center, normal, radius, start, section))
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
            ChartArm::Cylinder { .. } => (f.axis, None),
            ChartArm::Cone { .. } => {
                let g = start - f.origin;
                (g.normalize(), Some(opening))
            }
            _ => return Err(Self::refused("ruling", surface)),
        };
        let dir = if outward { dir } else { -dir };
        Ok(Self::line(
            chart,
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
        let g = surface.eval(toward.x, toward.y) - chart.frame().origin;
        Ok(Self::line(
            chart,
            geom::Line {
                origin: apex,
                dir: g.normalize(),
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
        let (centre, axis) = (c.frame.origin, c.frame.axis);
        let w = p - centre;
        let w = w - axis * w.dot(axis);
        debug_assert!(
            {
                let off = (p - centre)
                    .dot(axis)
                    .abs()
                    .max((w.norm() - c.radius).abs());
                match (off.point_value(), c.radius.point_value()) {
                    (Some(off), Some(r)) => off <= ON_CIRCLE * r.max(1.0),
                    _ => true,
                }
            },
            "a circle's start is a point the caller holds on it"
        );
        Self {
            shape: Shape::Circle(Circle::new(
                Frame::new(centre, axis, w.normalize()),
                c.radius,
            )),
            ..self
        }
    }

    fn line(chart: Placed<T>, line: geom::Line<T>, u: T, opening: Option<bool>) -> Self {
        let f = *chart.frame();
        let (sin, cos) = u.sin_cos();
        let normal = f.axis.cross(f.radial(u));
        Self {
            chart,
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
            shape: Shape::Line(geom::Line {
                origin: start,
                dir: f.u_ref.normalize(),
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
        let center = chart.frame().origin;
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
        Ok(Self::circle(
            chart,
            center,
            normal,
            radius,
            start,
            Section::Whole,
        ))
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
        let c = chart.frame().origin;
        let a = surface.eval(pole.x, pole.y) - c;
        let a = a.normalize();
        let q = surface.eval(through.x, through.y);
        let center = c + a * a.dot(q - c);
        let normal = if forward { a } else { -a };
        Ok(Self::circle(
            chart,
            center,
            normal,
            (q - center).norm(),
            q,
            Section::Whole,
        ))
    }

    /// The key of the record the curve's surface was charted from
    /// ([`Charted`]).
    pub fn record(&self) -> SurfaceKey {
        self.chart.record
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
            Section::Whole => return Ok(Sign::Positive),
            Section::HalfPlane { toward, opening } => (toward, opening),
            Section::TubeSide { toward } => (toward, None),
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

/// The plane door's answer for `carrier` decided once, for both readings
/// ([`OnSurface::across`], [`OnSurface::under`]): a carrier
/// parallel to the plane is in it or apart on `row`'s offset, and each
/// root carries its point.
fn decided<T: Decide>(
    found: SpanPlaneMeet<T>,
    carrier: &Curve3<T>,
    row: &'static str,
    band: Band,
) -> Result<CoSurface<T>, Indeterminate> {
    Ok(match found {
        SpanPlaneMeet::Parallel { offset } => match decide(row, Margin::of(offset), band)? {
            Sign::Zero => CoSurface::InPlane,
            Sign::Positive | Sign::Negative => CoSurface::Roots(Vec::new()),
        },
        SpanPlaneMeet::Touch => CoSurface::Touch,
        SpanPlaneMeet::Roots(roots) => CoSurface::Roots(
            roots
                .into_iter()
                .map(|root| (root, carrier.eval(root.t)))
                .collect(),
        ),
    })
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
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AcrossRefusal {
    /// A rung of the plane door, or the parallel offset, in band.
    InBand(Indeterminate),
    /// The edge's face is on another record than the one the curve was
    /// charted from: the pair is not known to be co-surface, so the arm
    /// does not read it.
    OffRecord {
        /// The edge's face's record.
        edge: SurfaceKey,
        /// The curve's record.
        curve: SurfaceKey,
    },
}

impl From<Indeterminate> for AcrossRefusal {
    fn from(diag: Indeterminate) -> Self {
        Self::InBand(diag)
    }
}

/// **Where an edge on a surface crosses a curve on it**
/// ([`OnSurface::across`]).
#[derive(Debug)]
pub enum CoSurface<T: Real> {
    /// The edge lies in the curve's plane: on the curve's own section or
    /// another curve of the plane, which the caller reads
    /// ([`SurfaceCurve::section_of`]) where it needs to.
    InPlane,
    /// The edge grazes the curve's plane: a double root, no crossing.
    Touch,
    /// The edge's roots on the curve's plane, each with its point: one
    /// is a meeting where it lies on the curve's own section of the plane
    /// ([`SurfaceCurve::section_of`]), which the caller reads, after what
    /// it reads first (a pole, an end).
    Roots(Vec<(SpanRoot<T>, Point3<T>)>),
    /// The edge's carrier has no closed form across a plane (a spiric or
    /// a spline): the caller reads the curve across the edge's plane.
    Unlaned,
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

    /// **The edge across `curve`'s plane** (module docs): the plane door's
    /// roots in the edge's own parameter over its span, each with its
    /// point. An edge parallel to the plane is decided in it or apart on
    /// `row`'s offset.
    ///
    /// # Errors
    ///
    /// [`AcrossRefusal::OffRecord`] where the edge's record is not the
    /// curve's; [`AcrossRefusal::InBand`] where a rung of the plane door or
    /// an offset is in band.
    pub fn across(
        &self,
        curve: &SurfaceCurve<T>,
        row: &'static str,
        band: Band,
    ) -> Result<CoSurface<T>, AcrossRefusal> {
        self.same_record(curve)?;
        let (on, normal) = curve.plane();
        match span_plane_meet(self.carrier, self.span, SpanEnds::Ended, on, normal, band) {
            Ok(Some(found)) => Ok(decided(found, self.carrier, row, band)?),
            Ok(None) => Ok(CoSurface::Unlaned),
            Err(fault) => Err(ConicRootFault::diag(fault).into()),
        }
    }

    /// **The plane the edge's carrier lies in**, a point and a unit
    /// normal: a conic's own, a spiric's cutting plane. A line lies in no
    /// one plane (`None`).
    pub fn plane(&self) -> Option<(Point3<T>, Vec3<T>)> {
        match *self.carrier {
            Curve3::Circle(Circle { frame, .. }) | Curve3::Ellipse(geom::Ellipse { frame, .. }) => {
                Some((frame.origin, frame.axis))
            }
            Curve3::Spiric {
                center,
                u_ref,
                offset,
                ..
            } => Some((center + u_ref * offset, u_ref)),
            _ => None,
        }
    }

    /// **`curve` over `span` across the edge's plane** ([`Self::plane`]):
    /// the other way to read the pair, where the edge's carrier has no
    /// closed form across the curve's plane or that plane is tangent to
    /// the surface along the edge. The plane door's roots are in the
    /// curve's own parameter, each with its point, decided as
    /// [`Self::across`] decides (a curve parallel to the plane in it or
    /// apart on `row`'s offset). An edge in no one plane is
    /// [`CoSurface::Unlaned`].
    ///
    /// # Errors
    ///
    /// [`AcrossRefusal::OffRecord`] where the edge's record is not the
    /// curve's; [`AcrossRefusal::InBand`] where a rung of the plane door or
    /// an offset is in band.
    pub fn under(
        &self,
        curve: &SurfaceCurve<T>,
        span: (T, T),
        ends: SpanEnds,
        row: &'static str,
        band: Band,
    ) -> Result<CoSurface<T>, AcrossRefusal> {
        self.same_record(curve)?;
        let Some((on, normal)) = self.plane() else {
            return Ok(CoSurface::Unlaned);
        };
        let carrier = curve.carrier();
        match span_plane_meet(&carrier, span, ends, on, normal, band) {
            Ok(Some(found)) => Ok(decided(found, &carrier, row, band)?),
            Ok(None) => unreachable!("a surface curve is a circle or a line"),
            Err(fault) => Err(ConicRootFault::diag(fault).into()),
        }
    }

    /// The pair's records agree, or the arm refuses it.
    fn same_record(&self, curve: &SurfaceCurve<T>) -> Result<(), AcrossRefusal> {
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

#[cfg(test)]
#[allow(clippy::unwrap_used)]
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
            origin: Point3::origin(),
            axis: z(),
            radius: 2.0,
            u_ref: x(),
        }
    }

    fn cone() -> geom::Surface<f64> {
        geom::Surface::Cone {
            apex: Point3::origin(),
            axis: z(),
            half_angle: FRAC_PI_4,
            u_ref: x(),
        }
    }

    fn plane() -> geom::Surface<f64> {
        geom::Surface::Plane {
            origin: Point3::new(0.5, -1.0, 2.0),
            normal: z(),
            u_ref: Vec3::new(0.6, 0.8, 0.0),
        }
    }

    fn sphere() -> geom::Surface<f64> {
        geom::Surface::Sphere {
            center: Point3::new(1.0, -2.0, 0.5),
            radius: 3.0,
            axis: z(),
            u_ref: x(),
        }
    }

    fn torus() -> geom::Surface<f64> {
        geom::Surface::Torus {
            center: Point3::origin(),
            axis: z(),
            major_radius: 5.0,
            minor_radius: 1.5,
            u_ref: x(),
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
            .across(&meridian, ROW, b())
            .unwrap()
        else {
            unreachable!("a parallel crosses a meridian's plane")
        };
        assert_eq!(roots.len(), 2);
        let mut sides: Vec<(f64, Sign)> = roots
            .iter()
            .map(|&(_, y)| (y.x, meridian.section_of(y, ROW, b()).unwrap()))
            .collect();
        sides.sort_by(|a, b| a.0.total_cmp(&b.0));
        assert!((sides[0].0 + 6.5).abs() < 1e-12 && (sides[1].0 - 6.5).abs() < 1e-12);
        assert_eq!((sides[0].1, sides[1].1), (Sign::Negative, Sign::Positive));
        let short = OnSurface::edge(key(), &carrier, (0.2, 1.0))
            .across(&meridian, ROW, b())
            .unwrap();
        assert!(
            matches!(short, CoSurface::Roots(ref r) if r.is_empty()),
            "{short:?}"
        );
        let own = meridian.carrier();
        let in_plane = OnSurface::edge(key(), &own, (0.0, 1.0))
            .across(&meridian, ROW, b())
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
        assert_eq!(
            OnSurface::edge(other, &carrier, span)
                .across(&meridian, ROW, b())
                .unwrap_err(),
            AcrossRefusal::OffRecord {
                edge: other,
                curve: mine
            }
        );
        assert!(matches!(
            OnSurface::edge(mine, &carrier, span).across(&meridian, ROW, b()),
            Ok(CoSurface::Roots(ref roots)) if roots.len() == 2
        ));
        // The other way, the meridian across the parallel edge's plane,
        // is refused alike: no reading of the pair skips the check.
        let whole = (0.0, 2.0 * PI);
        assert_eq!(
            OnSurface::edge(other, &carrier, span)
                .under(&meridian, whole, SpanEnds::Endless, ROW, b())
                .unwrap_err(),
            AcrossRefusal::OffRecord {
                edge: other,
                curve: mine
            }
        );
        assert!(matches!(
            OnSurface::edge(mine, &carrier, span)
                .under(&meridian, whole, SpanEnds::Endless, ROW, b()),
            Ok(CoSurface::Roots(ref roots)) if roots.len() == 2
        ));
    }

    #[test]
    fn a_turned_record_is_the_same_surface_on_another_chart() {
        let torus = torus();
        let turned = Charted::record(key(), &torus).turned(Vec3::new(0.0, 1.0, 0.0));
        let geom::Surface::Torus { u_ref, .. } = *turned.surface() else {
            unreachable!()
        };
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
            origin: Point3::origin(),
            normal: z(),
            u_ref: x(),
        };
        let line = SurfaceCurve::u_line(&on(&plane), Point2::new(0.0, 0.0)).unwrap();
        let circle = |cy: f64| geom::Curve3::circle(Point3::new(0.0, cy, 0.0), z(), 1.0, x());
        let near = circle(0.5);
        let CoSurface::Roots(roots) = OnSurface::edge(key(), &near, (-PI, PI))
            .across(&line, ROW, b())
            .unwrap()
        else {
            unreachable!("a circle through the line crosses its plane")
        };
        let mut xs: Vec<f64> = roots.iter().map(|&(_, y)| y.x).collect();
        xs.sort_by(f64::total_cmp);
        assert_eq!(xs.len(), 2);
        assert!((xs[0] + 0.75f64.sqrt()).abs() < 1e-12 && (xs[1] - 0.75f64.sqrt()).abs() < 1e-12);
        for &(_, y) in &roots {
            assert_eq!(line.section_of(y, ROW, b()).unwrap(), Sign::Positive);
        }
        let clear = circle(2.0);
        let apart = OnSurface::edge(key(), &clear, (-PI, PI))
            .across(&line, ROW, b())
            .unwrap();
        assert!(
            matches!(apart, CoSurface::Roots(ref r) if r.is_empty()),
            "{apart:?}"
        );
        let tangent = circle(1.0);
        let touch = OnSurface::edge(key(), &tangent, (-PI, PI))
            .across(&line, ROW, b())
            .unwrap();
        assert!(matches!(touch, CoSurface::Touch), "{touch:?}");
    }
}
