//! **An implicit partner of a spline wall**: a plane, cylinder, sphere
//! or torus read through its canonical form `f` in its own chart frame,
//! the one exact description it has (C3's per-side rule), so the section
//! of it and a wall `S` is the zero set of `φ = f∘S` over the wall's knot
//! rectangle (C5).
//!
//! The forms are the curve × face table's
//! ([`geom_core::spline::compose::CanonicalSurface`]): `z`,
//! `x² + y² − R²`, `x² + y² + z² − r²` and
//! `(x² + y² + z² + R² − r²)² − 4R²(x² + y²)`, in metres, metres²,
//! metres² and metres⁴. The plane's divisor is 1: its form is the
//! distance itself. Every enclosure here is of `f` itself
//! over a box of the frame, and a distance in metres is read from it
//! through the form's divisor ([`Meters`]) in the direction its decision
//! needs:
//!
//! - **a floor** ([`ImplicitForm::floor_metres`]): `f` over an upper bound on
//!   the divisor, whose magnitude bounds the distance from the surface
//!   from below and whose sign is `f`'s. A reading past the band on it is
//!   a box definitely apart from the surface;
//! - **a ceiling** ([`ImplicitForm::ceiling_metres`]): an upper bound on that
//!   distance from an upper bound on `|f|` and a lower bound on the
//!   radial reach, through a lower bound on the divisor. A reading within
//!   the band on it is a box definitely within the band of the surface.
//!   On the torus the divisor's lower bound is `(ρ + R)² − r²`, positive
//!   only where `R > r`, which [`ImplicitForm::of`] decides of the operand
//!   before anything reads it.
//!
//! A slope of `φ` is a sine only over the gradient's length: the sine of
//! the angle between a direction `d` of the wall and the partner's
//! tangent plane is `|∇f·d| / (‖∇f‖·‖d‖)` at a point of the zero set, so
//! every slope read against the band is divided by an upper bound on
//! `‖∇f‖` over the region it is read on ([`ImplicitForm::gradient_sup`]).

use geom::Surface;
use geom_core::interval::certification::Certification;
use geom_core::spline::compose::CanonicalSurface;
use geom_core::{Bounds, Interval, Point3, Vec3};

use super::enclose::Box3;
use crate::pcurve_cache::projected::{Meters, chart_frame};

/// **A plane, cylinder, sphere or torus as a wall's partner** (module
/// docs).
#[derive(Clone, Debug)]
pub(crate) struct ImplicitForm {
    surface: Surface<f64>,
    /// The frame's origin: the plane's origin, the axis point, or the
    /// centre.
    origin: Vec3<Interval>,
    /// The frame, enclosing an exactly orthonormal one about the stored
    /// axis (the plane's normal): `e₀` the reference direction made
    /// orthogonal to the axis, `e₁ = e₂ × e₀`, `e₂` the axis.
    frame: [Vec3<Interval>; 3],
    form: Form,
}

/// **The four forms**, their scalars as certification enclosures: the
/// plane `z`, the cylinder `x² + y² − R²`, the sphere
/// `x² + y² + z² − r²`, the torus
/// `(x² + y² + z² + R² − r²)² − 4R²(x² + y²)`.
#[derive(Clone, Copy, Debug)]
pub(crate) enum Form {
    /// The plane through the frame's origin, normal to its third axis.
    Plane,
    /// The cylinder of `radius` about the frame's third axis.
    Cylinder {
        /// Its radius.
        radius: Interval,
    },
    /// The sphere of `radius` about the frame's origin.
    Sphere {
        /// Its radius.
        radius: Interval,
    },
    /// The ring torus about the frame's third axis.
    Torus {
        /// Its major radius `R`.
        major: Interval,
        /// Its minor radius `r`.
        minor: Interval,
    },
}

impl Form {
    /// The form as the composites take it.
    pub(crate) fn canonical(self) -> CanonicalSurface {
        match self {
            Self::Plane => CanonicalSurface::Plane,
            Self::Cylinder { radius } => CanonicalSurface::Cylinder { radius },
            Self::Sphere { radius } => CanonicalSurface::Sphere { radius },
            Self::Torus { major, minor } => CanonicalSurface::Torus { major, minor },
        }
    }

    /// Its conversion between `|f|` and metres.
    fn meters(self) -> Meters<Interval> {
        match self {
            Self::Plane => Meters::Plane,
            Self::Cylinder { radius } => Meters::Cylinder { radius },
            Self::Sphere { radius } => Meters::Sphere { radius },
            Self::Torus { major, minor } => Meters::Torus { major, minor },
        }
    }
}

/// The plane's kind ([`ImplicitForm::kind`]).
pub(crate) const PLANE: &str = "plane";

/// The refusal of a surface that is no implicit partner.
const NOT_A_PARTNER: &str = concat!("only ", implicit_partners!(), " is an implicit partner here");

impl ImplicitForm {
    /// **The kind of `surface`, where an implicit form is minted from
    /// it**: the one test of the partner kinds (`implicit_partners!`
    /// names them), its name for a refusal's text; `None` for a cone, a
    /// spindle or a spline.
    pub(crate) fn kind(surface: &Surface<f64>) -> Option<&'static str> {
        match surface {
            Surface::Plane { .. } => Some(PLANE),
            Surface::Cylinder { .. } => Some("cylinder"),
            Surface::Sphere { .. } => Some("sphere"),
            Surface::Torus { .. } => Some("torus"),
            Surface::Cone { .. }
            | Surface::Spindle { .. }
            | Surface::Nurbs(_)
            | Surface::Approx(_) => None,
        }
    }

    /// The partner `surface`, or why it is none: a surface of no partner
    /// kind ([`Self::kind`]) is none; a frame that does not certify, a
    /// radius not certified positive, or a torus not certified a ring
    /// torus (`R > r`) refuses.
    pub(crate) fn of(surface: &Surface<f64>) -> Result<Self, &'static str> {
        if Self::kind(surface).is_none() {
            return Err(NOT_A_PARTNER);
        }
        let positive = |x: f64| x.is_finite() && x > 0.0;
        let form = match *surface {
            Surface::Plane { .. } => Form::Plane,
            Surface::Cylinder { radius, .. } if positive(radius) => Form::Cylinder {
                radius: Interval::point(radius),
            },
            Surface::Sphere { radius, .. } if positive(radius) => Form::Sphere {
                radius: Interval::point(radius),
            },
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
                Form::Torus { major, minor }
            }
            Surface::Cylinder { .. } | Surface::Sphere { .. } | Surface::Torus { .. } => {
                return Err("the partner's radius is not certified positive and finite");
            }
            _ => return Err(NOT_A_PARTNER),
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
        })
    }

    /// The form.
    pub(crate) fn form(&self) -> Form {
        self.form
    }

    /// The partner's kind, for a refusal's text.
    pub(crate) fn name(&self) -> &'static str {
        match self.form {
            Form::Plane => PLANE,
            Form::Cylinder { .. } => "cylinder",
            Form::Sphere { .. } => "sphere",
            Form::Torus { .. } => "torus",
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
            Form::Plane => q.z,
            Form::Cylinder { radius } => radial - radius.sqr(),
            Form::Sphere { radius } => radial + q.z.sqr() - radius.sqr(),
            Form::Torus { major, minor } => {
                let shifted = radial + q.z.sqr() + major.sqr() - minor.sqr();
                shifted.sqr() - Interval::point(4.0) * major.sqr() * radial
            }
        }
    }

    /// `∇f` over the frame box `q`, in the frame's coordinates.
    pub(crate) fn frame_gradient(&self, q: Vec3<Interval>) -> Vec3<Interval> {
        let two = Interval::point(2.0);
        match self.form {
            Form::Plane => Vec3::new(
                Interval::point(0.0),
                Interval::point(0.0),
                Interval::point(1.0),
            ),
            Form::Cylinder { .. } => Vec3::new(two * q.x, two * q.y, Interval::point(0.0)),
            Form::Sphere { .. } => Vec3::new(two * q.x, two * q.y, two * q.z),
            Form::Torus { major, minor } => {
                let shifted = q.x.sqr() + q.y.sqr() + q.z.sqr() + major.sqr() - minor.sqr();
                let four = Interval::point(4.0) * shifted;
                let eight = Interval::point(8.0) * major.sqr();
                Vec3::new(
                    four * q.x - eight * q.x,
                    four * q.y - eight * q.y,
                    four * q.z,
                )
            }
        }
    }

    /// A world vector written in the frame.
    pub(crate) fn frame_vector(&self, d: Vec3<Interval>) -> Vec3<Interval> {
        Vec3::new(
            d.dot(self.frame[0]),
            d.dot(self.frame[1]),
            d.dot(self.frame[2]),
        )
    }

    /// `∇²f` over the frame box `q`, in the frame's coordinates, by rows.
    pub(crate) fn frame_hessian(&self, q: Vec3<Interval>) -> [[Interval; 3]; 3] {
        let (o, two) = (Interval::point(0.0), Interval::point(2.0));
        match self.form {
            Form::Plane => [[o; 3]; 3],
            Form::Cylinder { .. } => [[two, o, o], [o, two, o], [o, o, o]],
            Form::Sphere { .. } => [[two, o, o], [o, two, o], [o, o, two]],
            Form::Torus { major, minor } => {
                // f = s² − 4R²(x² + y²), s = |q|² + R² − r²: ∇²f is
                // 4s·I + 8·q qᵀ − 8R²·diag(1, 1, 0).
                let shifted = q.x.sqr() + q.y.sqr() + q.z.sqr() + major.sqr() - minor.sqr();
                let (four, eight) = (Interval::point(4.0) * shifted, Interval::point(8.0));
                let r2 = eight * major.sqr();
                let c = [q.x, q.y, q.z];
                let mut h = [[o; 3]; 3];
                for (i, row) in h.iter_mut().enumerate() {
                    for (j, e) in row.iter_mut().enumerate() {
                        *e = eight * c[i] * c[j];
                        if i == j {
                            *e = *e + four - if i < 2 { r2 } else { o };
                        }
                    }
                }
                h
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
        let divisor = self.form.meters().divisor(q);
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
            Form::Sphere { .. } => q.x.sqr() + q.y.sqr() + q.z.sqr(),
            _ => q.x.sqr() + q.y.sqr(),
        }
        .sqrt();
        if !(reach.is_certified() && mag.is_finite() && mag >= 0.0) {
            return f64::INFINITY;
        }
        let rho = Interval::point(reach.lo().max(0.0));
        let over = self
            .form
            .meters()
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

/// **A wall read against an implicit partner before any march**: its
/// chart speeds, its accounting floor over its knot rectangle and the
/// settling tolerance of its boundary pass and march, minted once, as
/// the NURBS × NURBS lane mints its own ([`super::Section`]), with no
/// plane chart and no ℝ⁴ system: a crossing is settled on the wall
/// alone. The residual a state settles to is read at the wall's point
/// written in the partner's frame (about the plane's origin, or a curved
/// partner's centre or axis), so the tolerance reads
/// the louder of the wall's coordinates and those.
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
    /// ([`super::SsiError::FloorUnresolvable`]) and the boundary pass's
    /// settling tolerance's.
    pub(crate) fn of(
        wall: &'a geom::NurbsSurface<f64>,
        domain: super::SsiDomain,
        band: geom_core::Band,
    ) -> Result<Self, super::SsiError> {
        Self::of_charted((wall, super::ChartedNurbs::mint), domain, band)
    }

    /// [`ImplicitWall::of`] with the wall's chart had from `chart`, called
    /// where [`ImplicitWall::of`] mints it (after the wall and the domain
    /// are checked, so the refusals come in the same order): the mint
    /// itself, or a chart a caller minted once for the operation. **The
    /// chart must be of `wall`**: the door checks that it is the very
    /// surface, in every build, and refuses otherwise.
    ///
    /// # Errors
    ///
    /// As [`ImplicitWall::of`], with the chart's own refusals, and
    /// [`super::SsiError::WrongLane`] where the chart is of another
    /// surface.
    pub(crate) fn of_charted(
        (wall, chart): (
            &'a geom::NurbsSurface<f64>,
            impl FnOnce(
                &'a geom::NurbsSurface<f64>,
            ) -> Result<super::ChartedNurbs<'a, f64>, super::SsiError>,
        ),
        domain: super::SsiDomain,
        band: geom_core::Band,
    ) -> Result<Self, super::SsiError> {
        super::finite_net("NURBS wall", wall)?;
        domain.check()?;
        let charted = chart(wall)?;
        if !core::ptr::eq(charted.surface(), wall) {
            return Err(super::SsiError::WrongLane {
                expected: "the wall paired with a chart minted from it",
            });
        }
        let (ud, vd) = (wall.knot_values_u().domain(), wall.knot_values_v().domain());
        let root = super::exhaust::UvRect { u: ud, v: vd };
        let speed = charted.speeds().max();
        let floor = super::exhaust::SweepFloor::chart(
            root,
            domain.floor(band),
            speed,
            super::exhaust::FloorKind::Accounting,
        )?;
        let tol = super::march::MarchTol::from_band(band, Self::readout(wall, root, speed)?)?;
        Ok(Self {
            charted,
            floor,
            tol,
            domain,
            band,
        })
    }

    /// The wall's control hull, which holds every point of it.
    fn hull(wall: &geom::NurbsSurface<f64>) -> Result<Box3, super::SsiError> {
        wall.control()
            .iter()
            .map(|&p| Box3::between(p, p))
            .reduce(Box3::hull)
            .ok_or(super::SsiError::UnsupportedCertificate {
                what: "a NURBS wall with no control point",
            })
    }

    /// The wall's own readout: its coordinates, which bound the states
    /// (they are the wall's points), and its chart's.
    fn readout(
        wall: &geom::NurbsSurface<f64>,
        root: super::exhaust::UvRect,
        speed: geom_core::SupSpeed<f64>,
    ) -> Result<super::march::Readout, super::SsiError> {
        Ok(
            super::march::Readout::on_operands(Self::hull(wall)?, super::SSI_SPLINE_NOISE_ULPS)
                .or_chart(root, speed, super::SSI_SPLINE_NOISE_ULPS),
        )
    }

    /// **The tolerance a march against `curved` settles to**: the louder
    /// of the wall's own readout and the wall written about the partner's
    /// centre or axis, where the residual a state settles to is read.
    /// Minted only where a march runs, so a pose the classification
    /// decides without one never meets it.
    ///
    /// # Errors
    ///
    /// The tolerance's refusal ([`super::SsiError::SettlingUnresolvable`],
    /// naming the partner's reach where that binds).
    pub(crate) fn march_tol(
        &self,
        form: &ImplicitForm,
    ) -> Result<super::march::MarchTol, super::SsiError> {
        super::march::MarchTol::from_band(self.band, self.march_readout(form)?)
    }

    /// The readout a march against `curved` settles by: the louder of
    /// the wall's own and the wall written about the partner's centre or
    /// axis.
    pub(crate) fn march_readout(
        &self,
        form: &ImplicitForm,
    ) -> Result<super::march::Readout, super::SsiError> {
        let wall = self.charted.surface();
        let root = self.floor.root();
        let framed = form.framed(Self::hull(wall)?);
        let about = super::march::Readout::about_partner(
            Box3 {
                x: framed.x,
                y: framed.y,
                z: framed.z,
            },
            super::SSI_QUADRIC_NOISE_ULPS,
        );
        Ok(Self::readout(wall, root, self.charted.speeds().max())?.louder(about))
    }

    /// The wall's chart enclosures over its own cells.
    pub(crate) fn boxes(&self) -> &super::enclose::NurbsBoxes<'a, f64> {
        self.charted.own_boxes()
    }

    /// The accounting floor over the wall's rectangle.
    pub(crate) fn account_floor(&self) -> super::exhaust::SweepFloor<super::exhaust::UvRect> {
        self.floor
    }

    /// The wall's chart, its speeds minted.
    pub(crate) fn charted(&self) -> &super::ChartedNurbs<'a, f64> {
        &self.charted
    }

    /// The domain the wall is read over.
    pub(crate) fn domain(&self) -> super::SsiDomain {
        self.domain
    }

    /// The run band.
    pub(crate) fn band(&self) -> geom_core::Band {
        self.band
    }

    /// **The boundary pass against `curved`** (C3; [`super::boundary`]),
    /// at the domain's floor and extent.
    ///
    /// # Errors
    ///
    /// As [`super::boundary::Pass::run`].
    pub(crate) fn pass(
        &self,
        form: &ImplicitForm,
    ) -> Result<super::boundary::BoundaryPass<[f64; 2]>, super::SsiError> {
        super::boundary::Pass {
            wall: self.charted.surface(),
            boxes: self.charted.own_boxes(),
            speeds: self.charted.speeds(),
            partner: form,
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

    use super::ImplicitForm;

    fn torus(major: f64, minor: f64) -> Surface<f64> {
        Surface::Torus {
            center: Point3::new(0.0, 0.0, 0.0),
            axis: Vec3::new(0.0, 0.0, 1.0),
            u_ref: Vec3::new(1.0, 0.0, 0.0),
            major_radius: major,
            minor_radius: minor,
        }
    }

    /// **The torus's metres are read only on a ring torus, and bound the
    /// distance both ways.** A spindle torus refuses at the door. On the
    /// ring torus `(1, 0.9)`, a point `1e-6` off its tube on the inner
    /// equator, read over the box that also holds the outer equator's
    /// point: the ceiling at the inner point's `|f|` bounds its distance
    /// from above, at the box's least radial reach, and the floor at the
    /// point bounds it from below. The divisor's upper bound over the box
    /// is the outer equator's, five times the inner's. Red against the
    /// guard dropped, and against the ceiling read through the floor's
    /// divisor (it then reads a fifth of the distance).
    #[test]
    fn a_torus_reads_metres_only_as_a_ring() {
        assert!(
            ImplicitForm::of(&torus(0.2, 0.3)).is_err(),
            "a spindle torus refuses"
        );
        let ring = ImplicitForm::of(&torus(1.0, 0.9)).expect("a ring torus");
        let off = 1e-6;
        let inner = ring.framed_point(Point3::new(0.1 - off, 0.0, 0.0));
        let outer = ring.framed_point(Point3::new(1.9, 0.0, 0.0));
        let hull = |a: geom_core::Interval, b: geom_core::Interval| {
            geom_core::Interval::from_bounds(a.lo().min(b.lo()), a.hi().max(b.hi()))
        };
        let q = Vec3::new(
            hull(inner.x, outer.x),
            hull(inner.y, outer.y),
            hull(inner.z, outer.z),
        );
        let f = ring.value(inner);
        let ceiling = ring.ceiling_metres(f.lo().abs().max(f.hi().abs()), q);
        let floor = ring.floor_metres(f, inner);
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
