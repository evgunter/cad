//! **A curved implicit partner of a spline wall**: a cylinder, sphere or
//! torus read through its canonical form `f` in its own chart frame, the
//! one exact description it has (C3's per-side rule), so the section of
//! it and a wall `S` is the zero set of `φ = f∘S` over the wall's knot
//! rectangle (C5).
//!
//! The forms are the curve × face table's
//! ([`geom_core::spline::compose::CanonicalSurface`]): `x² + y² − R²`,
//! `x² + y² + z² − r²` and `(x² + y² + z² + R² − r²)² − 4R²(x² + y²)`, in
//! metres², metres² and metres⁴. Every enclosure here is of `f` itself
//! over a box of the frame, and a distance in metres is read from it
//! through the form's divisor ([`Meters`]) in the direction its decision
//! needs:
//!
//! - **a floor** ([`Curved::floor_metres`]): `f` over an upper bound on
//!   the divisor, whose magnitude bounds the distance from the surface
//!   from below and whose sign is `f`'s. A reading past the band on it is
//!   a box definitely apart from the surface;
//! - **a ceiling** ([`Curved::ceiling_metres`]): an upper bound on that
//!   distance from an upper bound on `|f|` and a lower bound on the
//!   radial reach, through a lower bound on the divisor. A reading within
//!   the band on it is a box definitely within the band of the surface.
//!   On the torus the divisor's lower bound is `(ρ + R)² − r²`, positive
//!   only where `R > r`, which [`Curved::of`] decides of the operand
//!   before anything reads it.
//!
//! A slope of `φ` is a sine only over the gradient's length: the sine of
//! the angle between a direction `d` of the wall and the partner's
//! tangent plane is `|∇f·d| / (‖∇f‖·‖d‖)` at a point of the zero set, so
//! every slope read against the band is divided by an upper bound on
//! `‖∇f‖` over the region it is read on ([`Curved::gradient_sup`]).

use geom::Surface;
use geom_core::interval::certification::Certification;
use geom_core::spline::compose::CanonicalSurface;
use geom_core::{Bounds, Interval, Point3, Vec3};

use super::enclose::Box3;
use crate::pcurve_cache::projected::{Meters, chart_frame};

/// **A cylinder, sphere or torus as a wall's partner** (module docs).
#[derive(Clone, Debug)]
pub(crate) struct Curved {
    surface: Surface<f64>,
    /// The frame's origin: the axis point, or the centre.
    origin: Vec3<Interval>,
    /// The frame, enclosing an exactly orthonormal one about the stored
    /// axis: `e₀` the reference direction made orthogonal to the axis,
    /// `e₁ = e₂ × e₀`, `e₂` the axis.
    frame: [Vec3<Interval>; 3],
    form: CanonicalSurface,
    meters: Meters<Interval>,
}

impl Curved {
    /// The partner `surface`, or why it is none: a plane, a cone or a
    /// spline is no curved partner here; a frame that does not certify,
    /// a radius not certified positive, or a torus not certified a ring
    /// torus (`R > r`) refuses.
    pub(crate) fn of(surface: &Surface<f64>) -> Result<Self, &'static str> {
        let positive = |x: f64| x.is_finite() && x > 0.0;
        let (form, meters) = match *surface {
            Surface::Cylinder { radius, .. } if positive(radius) => {
                let radius = Interval::point(radius);
                (
                    CanonicalSurface::Cylinder { radius },
                    Meters::Cylinder { radius },
                )
            }
            Surface::Sphere { radius, .. } if positive(radius) => {
                let radius = Interval::point(radius);
                (
                    CanonicalSurface::Sphere { radius },
                    Meters::Sphere { radius },
                )
            }
            Surface::Torus {
                major_radius,
                minor_radius,
                ..
            } if positive(minor_radius) && major_radius.is_finite() => {
                let (major, minor) = (Interval::point(major_radius), Interval::point(minor_radius));
                if (major - minor).lo() <= 0.0 {
                    return Err(
                        "the torus is not certified a ring torus (R > r), on which alone \
                                its form's distance divisor has a positive lower bound",
                    );
                }
                (
                    CanonicalSurface::Torus { major, minor },
                    Meters::Torus { major, minor },
                )
            }
            Surface::Cylinder { .. } | Surface::Sphere { .. } | Surface::Torus { .. } => {
                return Err("the partner's radius is not certified positive and finite");
            }
            Surface::Plane { .. }
            | Surface::Cone { .. }
            | Surface::Nurbs(_)
            | Surface::Approx(_) => {
                return Err("only a cylinder, a sphere or a torus is a curved implicit partner");
            }
        };
        let (origin, [u_ref, _, axis]) =
            chart_frame(surface).ok_or("only an analytic surface has a chart frame")?;
        let vector = |v: Vec3<f64>| v.map(Interval::point);
        let (u_ref, axis) = (vector(u_ref), vector(axis));
        let e2 = axis / axis.norm();
        let radial = u_ref - e2 * u_ref.dot(e2);
        let e0 = radial / radial.norm();
        let frame = [e0, e2.cross(e0), e2];
        if !frame
            .iter()
            .all(|v| v.x.is_certified() && v.y.is_certified() && v.z.is_certified())
        {
            return Err("the partner's chart frame does not certify");
        }
        Ok(Self {
            surface: surface.clone(),
            origin: vector(Vec3::new(origin.x, origin.y, origin.z)),
            frame,
            form,
            meters,
        })
    }

    /// The canonical form the composites are built of.
    pub(crate) fn form(&self) -> CanonicalSurface {
        self.form
    }

    /// The partner's kind, for a refusal's text.
    pub(crate) fn name(&self) -> &'static str {
        match self.form {
            CanonicalSurface::Cylinder { .. } => "cylinder",
            CanonicalSurface::Sphere { .. } => "sphere",
            CanonicalSurface::Torus { .. } => "torus",
            CanonicalSurface::Plane | CanonicalSurface::Cone { .. } => {
                unreachable!("a curved partner is a cylinder, a sphere or a torus")
            }
        }
    }

    /// A world box written in the frame.
    pub(crate) fn framed(&self, b: Box3) -> Vec3<Interval> {
        let g = Vec3::new(b.x, b.y, b.z) - self.origin;
        Vec3::new(
            g.dot(self.frame[0]),
            g.dot(self.frame[1]),
            g.dot(self.frame[2]),
        )
    }

    /// A world point written in the frame.
    pub(crate) fn framed_point(&self, p: Point3<f64>) -> Vec3<Interval> {
        let x = |c: f64| Interval::point(c);
        self.framed(Box3 {
            x: x(p.x),
            y: x(p.y),
            z: x(p.z),
        })
    }

    /// `f` over the frame box `q`.
    pub(crate) fn value(&self, q: Vec3<Interval>) -> Interval {
        let radial = q.x.sqr() + q.y.sqr();
        match self.form {
            CanonicalSurface::Cylinder { radius } => radial - radius.sqr(),
            CanonicalSurface::Sphere { radius } => radial + q.z.sqr() - radius.sqr(),
            CanonicalSurface::Torus { major, minor } => {
                let shifted = radial + q.z.sqr() + major.sqr() - minor.sqr();
                shifted.sqr() - Interval::point(4.0) * major.sqr() * radial
            }
            CanonicalSurface::Plane | CanonicalSurface::Cone { .. } => {
                unreachable!("a curved partner is a cylinder, a sphere or a torus")
            }
        }
    }

    /// `∇f` over the frame box `q`, in the frame's coordinates.
    fn frame_gradient(&self, q: Vec3<Interval>) -> Vec3<Interval> {
        let two = Interval::point(2.0);
        match self.form {
            CanonicalSurface::Cylinder { .. } => {
                Vec3::new(two * q.x, two * q.y, Interval::point(0.0))
            }
            CanonicalSurface::Sphere { .. } => Vec3::new(two * q.x, two * q.y, two * q.z),
            CanonicalSurface::Torus { major, minor } => {
                let shifted = q.x.sqr() + q.y.sqr() + q.z.sqr() + major.sqr() - minor.sqr();
                let four = Interval::point(4.0) * shifted;
                let eight = Interval::point(8.0) * major.sqr();
                Vec3::new(
                    four * q.x - eight * q.x,
                    four * q.y - eight * q.y,
                    four * q.z,
                )
            }
            CanonicalSurface::Plane | CanonicalSurface::Cone { .. } => {
                unreachable!("a curved partner is a cylinder, a sphere or a torus")
            }
        }
    }

    /// `∇f` over the frame box `q`, in world coordinates.
    pub(crate) fn gradient(&self, q: Vec3<Interval>) -> Vec3<Interval> {
        let g = self.frame_gradient(q);
        self.frame[0] * g.x + self.frame[1] * g.y + self.frame[2] * g.z
    }

    /// An upper bound on `‖∇f‖` over the frame box `q`; `NaN` where it
    /// does not certify. The frame encloses an exactly orthonormal one,
    /// so the frame's coordinates carry the world length.
    pub(crate) fn gradient_sup(&self, q: Vec3<Interval>) -> f64 {
        let g = self.frame_gradient(q);
        let n = (g.x.sqr() + g.y.sqr() + g.z.sqr()).sqrt();
        if n.is_certified() { n.hi() } else { f64::NAN }
    }

    /// **A floor in metres** (module docs): `f`'s enclosure over the
    /// upper bound of the form's divisor over the frame box `q`, a signed
    /// enclosure whose magnitude bounds the distance of `q`'s points from
    /// the surface from below. Refused where the divisor has no positive
    /// finite upper bound.
    pub(crate) fn floor_metres(&self, f: Interval, q: Vec3<Interval>) -> Interval {
        let divisor = self.meters.divisor(q);
        let sup = divisor.lo().abs().max(divisor.hi().abs());
        if !(divisor.is_certified() && sup > 0.0 && sup.is_finite()) {
            return Interval::refused();
        }
        f / Interval::point(sup)
    }

    /// **A ceiling in metres** (module docs): an upper bound on the
    /// distance from the surface of a point of the frame box `q` where
    /// `|f| ≤ mag`, through [`Meters::over`] at `q`'s least radial reach
    /// (from the axis on the cylinder and the torus, from the centre on
    /// the sphere). `+∞` where it does not certify.
    pub(crate) fn ceiling_metres(&self, mag: f64, q: Vec3<Interval>) -> f64 {
        let reach = match self.form {
            CanonicalSurface::Sphere { .. } => q.x.sqr() + q.y.sqr() + q.z.sqr(),
            _ => q.x.sqr() + q.y.sqr(),
        }
        .sqrt();
        if !(reach.is_certified() && mag.is_finite() && mag >= 0.0) {
            return f64::INFINITY;
        }
        let rho = Interval::point(reach.lo().max(0.0));
        let over = self
            .meters
            .over(Interval::point(mag), rho, (0.0, 0.0), |lever| {
                if lever.is_certified() && lever.lo() > 0.0 {
                    Ok(lever)
                } else {
                    Err(())
                }
            });
        match over {
            Ok(d) if d.is_certified() && d.hi().is_finite() => d.hi().max(0.0),
            _ => f64::INFINITY,
        }
    }

    /// The partner's residual at `p` in metres
    /// ([`crate::implicit_residual`], the kernel's linearized form), the
    /// value a settled point is held to.
    pub(crate) fn residual(&self, p: Point3<f64>) -> f64 {
        crate::implicit_residual(&self.surface, p)
    }

    /// The residual's rate along `d` at `p`.
    pub(crate) fn residual_rate(&self, p: Point3<f64>, d: Vec3<f64>) -> f64 {
        crate::implicit_gradient(&self.surface, p).dot(d)
    }
}

/// **A wall read against a curved partner before any march**: its
/// chart speeds, its accounting floor over its knot rectangle and the
/// settling tolerance of its boundary pass, minted once, as the plane ×
/// NURBS lane mints them ([`super::PlaneSection`]), with no plane chart
/// and no ℝ⁴ system: a crossing is settled on the wall alone.
pub(crate) struct ImplicitWall<'a> {
    charted: super::ChartedNurbs<'a, f64>,
    floor: super::exhaust::SweepFloor<super::exhaust::UvRect>,
    tol: super::march::MarchTol,
    domain: super::SsiDomain,
    band: geom_core::Band,
}

impl<'a> ImplicitWall<'a> {
    /// The wall `wall` over `domain` at `band`.
    ///
    /// # Errors
    ///
    /// [`super::SsiError::OperandNotFinite`] for a wall that is not
    /// finite, [`super::SsiError::DomainUnusable`],
    /// [`super::SsiError::ChartSpeed`], the floor's refusal
    /// ([`super::SsiError::FloorUnresolvable`]) and the march
    /// tolerance's.
    pub(crate) fn of(
        wall: &'a geom::NurbsSurface<f64>,
        domain: super::SsiDomain,
        band: geom_core::Band,
    ) -> Result<Self, super::SsiError> {
        super::finite_net("NURBS wall", wall)?;
        domain.check()?;
        let charted = super::ChartedNurbs::mint(wall)?;
        let (ud, vd) = (wall.knot_values_u().domain(), wall.knot_values_v().domain());
        let root = super::exhaust::UvRect { u: ud, v: vd };
        let speed = charted.speeds().max();
        let floor = super::exhaust::SweepFloor::chart(
            root,
            domain.floor(band),
            speed,
            super::exhaust::FloorKind::Accounting,
        )?;
        let hull = wall
            .control()
            .iter()
            .map(|&p| Box3::between(p, p))
            .reduce(Box3::hull)
            .ok_or(super::SsiError::UnsupportedCertificate {
                what: "a NURBS wall with no control point",
            })?;
        let tol = super::march::MarchTol::from_band(
            band,
            super::spline_readout(hull, wall, root, speed),
        )?;
        Ok(Self {
            charted,
            floor,
            tol,
            domain,
            band,
        })
    }

    /// The wall's chart enclosures over its own cells.
    pub(crate) fn boxes(&self) -> &super::enclose::NurbsBoxes<'a, f64> {
        self.charted.own_boxes()
    }

    /// The accounting floor over the wall's rectangle.
    pub(crate) fn account_floor(&self) -> super::exhaust::SweepFloor<super::exhaust::UvRect> {
        self.floor
    }

    /// **The boundary pass against `curved`** (C3; [`super::boundary`]),
    /// at the domain's floor and extent.
    ///
    /// # Errors
    ///
    /// As [`super::boundary::Pass::run`].
    pub(crate) fn pass(
        &self,
        curved: &Curved,
    ) -> Result<super::boundary::BoundaryPass<[f64; 2]>, super::SsiError> {
        super::boundary::Pass {
            wall: self.charted.surface(),
            boxes: self.charted.own_boxes(),
            speeds: self.charted.speeds(),
            partner: curved,
            tol: self.tol,
            floor: self.domain.floor(self.band),
            extent: self.domain.extent,
            band: self.band,
        }
        .run()
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests {
    use geom::Surface;
    use geom_core::{Bounds, Point3, Vec3};

    use super::Curved;

    fn torus(major: f64, minor: f64) -> Surface<f64> {
        Surface::Torus {
            center: Point3::new(0.0, 0.0, 0.0),
            axis: Vec3::new(0.0, 0.0, 1.0),
            u_ref: Vec3::new(1.0, 0.0, 0.0),
            major_radius: major,
            minor_radius: minor,
        }
    }

    /// **The torus's metres are read only on a ring torus.** A spindle
    /// torus refuses at the door; a ring torus's ceiling at a point
    /// `1e-6` off its tube bounds that distance and stays within twice
    /// it, and its floor stays under it. Red against the guard dropped
    /// (the spindle's lever is not positive and the ceiling would be
    /// `+∞` or wrong), and against a ceiling read from the floor's
    /// divisor.
    #[test]
    fn a_torus_reads_metres_only_as_a_ring() {
        assert!(
            Curved::of(&torus(0.2, 0.3)).is_err(),
            "a spindle torus refuses"
        );
        let ring = Curved::of(&torus(1.0, 0.25)).expect("a ring torus");
        let off = 1e-6;
        let q = ring.framed_point(Point3::new(1.25 + off, 0.0, 0.0));
        let f = ring.value(q);
        let ceiling = ring.ceiling_metres(f.lo().abs().max(f.hi().abs()), q);
        let floor = ring.floor_metres(f, q);
        assert!(
            off * (1.0 - 1e-6) <= ceiling && ceiling <= 2.0 * off,
            "the ceiling {ceiling:e} against the distance {off:e}"
        );
        assert!(
            floor.hi() <= off * (1.0 + 1e-6) && floor.lo() > 0.0,
            "the floor {floor:?} against the distance {off:e}"
        );
    }
}
