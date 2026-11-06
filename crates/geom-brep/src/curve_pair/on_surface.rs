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
//! and a few others: a cylinder's plane through the axis in two rulings, a
//! cone's in four half-generators, a torus's planes in two circles. Which
//! of them a root lies on is read through the chart's channels
//! ([`geom::ChartChannels::read`]) on the caller's row, so the reading
//! decides it as a length.
//!
//! A [`SurfaceCurve`] is built from a surface value and chart coordinates
//! and from nothing else: its centre, radius and direction are that
//! surface's own at those coordinates, so no curve off it can be built as
//! one. The surface is a `geom::Surface` value, which a caller may
//! synthesise rather than read off a face (the sphere cut-in charts the
//! face's sphere on its own `u_ref`; a region door rebuilds its surface
//! from its own fields); the type ties the curve to that value, not to a
//! face's record.
//!
//! An [`OnSurface`] edge is read against a curve on the same surface, and
//! nothing in the types ties the two together: that the edge lies on the
//! curve's surface is [`OnSurface::edge`]'s precondition, held by the
//! caller.

use geom::{Azimuth, ChartAngle, ChartArm, ChartChannels, ChartInverse, Circle, Curve3, Frame};
use geom_core::k_stats::decide;
use geom_core::{Band, Decide, Indeterminate, Margin, Point2, Point3, Real, Sign, Vec3};

use crate::curve_face::plane_crossing::{
    ConicRootFault, SpanEnds, SpanPlaneMeet, SpanRoot, span_plane_meet,
};

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
    chart: ChartInverse<T>,
    shape: Shape<T>,
    plane: (Point3<T>, Vec3<T>),
    section: Section<T>,
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
    fn chart(
        surface: &geom::Surface<T>,
        family: &'static str,
    ) -> Result<ChartInverse<T>, NoSuchFamily> {
        ChartInverse::of(surface).map_err(|_| NoSuchFamily {
            family,
            kind: surface.kind(),
        })
    }

    fn refused(family: &'static str, surface: &geom::Surface<T>) -> NoSuchFamily {
        NoSuchFamily {
            family,
            kind: surface.kind(),
        }
    }

    fn circle(
        chart: ChartInverse<T>,
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
    pub fn parallel(
        surface: &geom::Surface<T>,
        uv: Point2<T>,
        forward: bool,
    ) -> Result<Self, NoSuchFamily> {
        let chart = Self::chart(surface, "parallel")?;
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
    pub fn meridian(
        surface: &geom::Surface<T>,
        uv: Point2<T>,
        forward: bool,
    ) -> Result<Self, NoSuchFamily> {
        let chart = Self::chart(surface, "meridian")?;
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
        surface: &geom::Surface<T>,
        uv: Point2<T>,
        opening: bool,
        outward: bool,
    ) -> Result<Self, NoSuchFamily> {
        let chart = Self::chart(surface, "ruling")?;
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
        Ok(Self::line(chart, start, dir, uv.x, opening))
    }

    /// **A cone's generator from its apex** through the point `toward`
    /// names, on the nappe `opening` that point lies on.
    ///
    /// # Errors
    ///
    /// [`NoSuchFamily`] on any surface but a cone.
    pub fn from_apex(
        surface: &geom::Surface<T>,
        toward: Point2<T>,
        opening: bool,
    ) -> Result<Self, NoSuchFamily> {
        let chart = Self::chart(surface, "generator from the apex")?;
        if !matches!(chart.arm(), ChartArm::Cone { .. }) {
            return Err(Self::refused("generator from the apex", surface));
        }
        let apex = surface.eval(toward.x, T::zero());
        let g = surface.eval(toward.x, toward.y) - chart.frame().origin;
        Ok(Self::line(
            chart,
            apex,
            g.normalize(),
            toward.x,
            Some(opening),
        ))
    }

    fn line(
        chart: ChartInverse<T>,
        start: Point3<T>,
        dir: Vec3<T>,
        u: T,
        opening: Option<bool>,
    ) -> Self {
        let f = *chart.frame();
        let (sin, cos) = u.sin_cos();
        let normal = f.axis.cross(f.radial(u));
        Self {
            chart,
            shape: Shape::Line(geom::Line { origin: start, dir }),
            plane: (start, normal.normalize()),
            section: Section::HalfPlane {
                toward: Azimuth::of(sin, cos),
                opening,
            },
        }
    }

    /// **A sphere's great circle through `uv`** along `t` there: the
    /// great circle in the plane of `t` and the radius to `uv`'s point.
    /// `t` must not be parallel to that radius (a debug assertion), as
    /// the sphere ray's unit tangent never is.
    ///
    /// # Errors
    ///
    /// [`NoSuchFamily`] on any surface but a sphere.
    pub fn great_circle(
        surface: &geom::Surface<T>,
        uv: Point2<T>,
        t: Vec3<T>,
    ) -> Result<Self, NoSuchFamily> {
        let chart = Self::chart(surface, "great circle")?;
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
        surface: &geom::Surface<T>,
        pole: Point2<T>,
        through: Point2<T>,
        forward: bool,
    ) -> Result<Self, NoSuchFamily> {
        let chart = Self::chart(surface, "circle about a pole")?;
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

    /// **This curve over `span` across a plane through `on` of unit normal
    /// `normal`**: the plane door's reading in the curve's own parameter
    /// ([`span_plane_meet`]), decided as [`OnSurface::across`] decides an
    /// edge's (a parallel curve in the plane or apart on `row`'s offset).
    ///
    /// # Errors
    ///
    /// A rung or an offset in band.
    pub fn across_plane(
        &self,
        span: (T, T),
        ends: SpanEnds,
        (on, normal): (Point3<T>, Vec3<T>),
        row: &'static str,
        band: Band,
    ) -> Result<CoSurface<T>, Indeterminate> {
        let carrier = self.carrier();
        match span_plane_meet(&carrier, span, ends, on, normal, band) {
            Ok(Some(found)) => decided(found, &carrier, row, band),
            Ok(None) => unreachable!("a surface curve is a circle or a line"),
            Err(fault) => Err(ConicRootFault::diag(fault)),
        }
    }
}

/// The plane door's answer for `carrier` decided once, for both readings
/// ([`OnSurface::across`], [`SurfaceCurve::across_plane`]): a carrier
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
    carrier: &'a Curve3<T>,
    span: (T, T),
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
    /// The span `(t0, t1)` of `carrier`, an edge of a face on the surface
    /// its curves are read on.
    ///
    /// **Precondition**: the edge lies on the surface of every
    /// [`SurfaceCurve`] it is read against. The types do not tie the two
    /// (module docs); the caller holds it, as tier 3 places each edge on
    /// its faces' carriers at rest.
    pub fn edge(carrier: &'a Curve3<T>, span: (T, T)) -> Self {
        Self { carrier, span }
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
    /// A rung of the plane door or an offset in band.
    pub fn across(
        &self,
        curve: &SurfaceCurve<T>,
        row: &'static str,
        band: Band,
    ) -> Result<CoSurface<T>, Indeterminate> {
        let (on, normal) = curve.plane();
        match span_plane_meet(self.carrier, self.span, SpanEnds::Ended, on, normal, band) {
            Ok(Some(found)) => decided(found, self.carrier, row, band),
            Ok(None) => Ok(CoSurface::Unlaned),
            Err(fault) => Err(ConicRootFault::diag(fault)),
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
            (cylinder(), SurfaceCurve::parallel(&cylinder(), uv, true)),
            (cone(), SurfaceCurve::parallel(&cone(), uv, false)),
            (torus(), SurfaceCurve::parallel(&torus(), uv, true)),
            (torus(), SurfaceCurve::meridian(&torus(), uv, false)),
            (sphere(), SurfaceCurve::meridian(&sphere(), uv, true)),
            (
                cylinder(),
                SurfaceCurve::ruling(&cylinder(), uv, true, false),
            ),
            (cone(), SurfaceCurve::ruling(&cone(), uv, true, true)),
            (cone(), SurfaceCurve::from_apex(&cone(), uv, true)),
            (
                sphere(),
                SurfaceCurve::great_circle(&sphere(), Point2::new(0.7, 0.3), z()),
            ),
            (
                sphere(),
                SurfaceCurve::about(&sphere(), Point2::new(0.2, 0.4), uv, true),
            ),
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
        assert!(SurfaceCurve::parallel(&sphere(), uv, true).is_err());
        assert!(SurfaceCurve::meridian(&cylinder(), uv, true).is_err());
        assert!(SurfaceCurve::ruling(&torus(), uv, true, true).is_err());
        assert!(SurfaceCurve::from_apex(&cylinder(), uv, true).is_err());
        assert!(SurfaceCurve::great_circle(&torus(), uv, z()).is_err());
    }

    /// **A cone's generator reads its own half-generator of its plane**:
    /// the plane through the axis at azimuth `0` cuts the cone in four
    /// half-generators; a point on the generator's own is `Positive`, one
    /// on the mirror nappe or the opposite azimuth `Negative`, and the
    /// apex, on every generator, `Positive`.
    #[test]
    fn a_cone_generator_reads_its_own_half_generator() {
        let cone = cone();
        let g = SurfaceCurve::ruling(&cone, Point2::new(0.0, 2.0), true, true).unwrap();
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
        let meridian = SurfaceCurve::meridian(&torus, Point2::new(0.0, 0.3), true).unwrap();
        let parallel = SurfaceCurve::parallel(&torus, Point2::new(FRAC_PI_2, 0.0), true).unwrap();
        let carrier = parallel.carrier();
        let CoSurface::Roots(roots) = OnSurface::edge(&carrier, (-1.0, 2.0 * PI - 1.0))
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
        let short = OnSurface::edge(&carrier, (0.2, 1.0))
            .across(&meridian, ROW, b())
            .unwrap();
        assert!(
            matches!(short, CoSurface::Roots(ref r) if r.is_empty()),
            "{short:?}"
        );
        let own = meridian.carrier();
        let in_plane = OnSurface::edge(&own, (0.0, 1.0))
            .across(&meridian, ROW, b())
            .unwrap();
        assert!(matches!(in_plane, CoSurface::InPlane), "{in_plane:?}");
    }
}
