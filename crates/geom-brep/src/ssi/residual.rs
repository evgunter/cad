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
use crate::pcurve_cache::projected::Meters;

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
    /// The stored surface's `f` at the frame's origin: zero, except on a
    /// plane read at a pivot ([`ImplicitForm::lifted`]).
    lift: Interval,
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
const NOT_A_PARTNER: &str = concat!(
    "only ",
    implicit_partners!(),
    " is an implicit partner here"
);

/// **A stored `f64` frame as every certified partner read takes it**: its
/// origin, and the columns `u_ref`, `v_ref`, `axis` of the enclosure of
/// its polar factor ([`geom::Frame::map_scalar`] to [`Interval`], decision
/// 1 of PR 7131), the rotation the `Interval` lift reads the carrier on.
/// Never the stored entries lifted as points and squared at the scalar,
/// which is a second carrier a rounding of turn away, and near a graze of
/// a far partner more than the band apart.
pub(crate) fn certified_frame(frame: &geom::Frame<f64>) -> (Vec3<Interval>, [Vec3<Interval>; 3]) {
    let motion = frame.map_scalar(Interval::from_certified).motion();
    let o = motion.origin();
    (
        Vec3::new(o.x, o.y, o.z),
        [motion.u().get(), motion.v().get(), motion.w().get()],
    )
}

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
        let stored = surface
            .frame()
            .ok_or("only an analytic surface has a chart frame")?;
        let (origin, frame) = certified_frame(&stored);
        if !frame
            .iter()
            .all(|v| v.x.is_certified() && v.y.is_certified() && v.z.is_certified())
        {
            return Err("the partner's chart frame does not certify");
        }
        Ok(Self {
            surface: surface.clone(),
            origin,
            frame,
            form,
            lift: Interval::point(0.0),
        })
    }

    /// **This form read through a frame enclosure**: `frame` (`e₀`, `e₁`,
    /// `e₂`) in place of the one squared from the stored surface's axes,
    /// for a partner whose `f64` surface stands on its frame's midpoints
    /// ([`crate::ssi::certify::chart_partner_form`]): an enclosure of the
    /// partner's own rotation, so every value and gradient the form reads
    /// encloses the partner's.
    pub(crate) fn with_frame(self, frame: [Vec3<Interval>; 3]) -> Self {
        Self { frame, ..self }
    }

    /// **The plane read at a pivot, lifted onto its stored plane**: the
    /// same form, with `lift` the stored plane's `f` at this one's
    /// origin, so that `f + lift` is the stored plane's `f`
    /// ([`ImplicitForm::plus_lift`]).
    pub(crate) fn lifted(self, lift: Interval) -> Self {
        Self { lift, ..self }
    }

    /// The stored surface's `f`, from this form's `f`: the value an
    /// exact empty answer reads the sign of. A distance read against the
    /// band reads this form's own `f`, its band padded by the lift.
    pub(crate) fn plus_lift(&self, f: Interval) -> Interval {
        f + self.lift
    }

    /// `lift` ([`ImplicitForm::lifted`]).
    pub(crate) fn lift(&self) -> Interval {
        self.lift
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
                let expanded = shifted.sqr() - Interval::point(4.0) * major.sqr() * radial;
                // The same quartic as `(D² − r²)(D′² − r²)`, `D` and `D′`
                // the distances to the core circle's near and far points
                // in the box's meridian plane. The expanded form cancels
                // two terms of size `4R⁴`, so over a box of width `w` it
                // spreads by about `16R³w` where the quartic varies by
                // `8R²r·w`; the near factor reads its variation at the
                // tube's own scale.
                let rho = radial.sqrt();
                let cross = q.z.sqr() - minor.sqr();
                let near = (rho - major).sqr() + cross;
                let far = (rho + major).sqr() + cross;
                expanded.meet(near * far)
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

    /// `∇f` in world coordinates where it is the same at every point: the
    /// plane's, the frame's third axis (its unit normal). `None` for a
    /// curved form, whose gradient turns with the point.
    pub(crate) fn constant_gradient(&self) -> Option<Vec3<Interval>> {
        match self.form {
            Form::Plane => Some(self.frame[2]),
            Form::Cylinder { .. } | Form::Sphere { .. } | Form::Torus { .. } => None,
        }
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
        let rho = (
            Interval::point(reach.lo().max(0.0)),
            Interval::point(reach.hi()),
        );
        let over = self
            .form
            .meters()
            .over(Interval::point(mag), rho, (0.0, 0.0), |lever, _| {
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

/// **The polar factor's truth, in double-double**, for the rows that pin
/// a certified partner read to the carrier decision 1 names: a stored
/// frame's polar factor `R = M(I − ½G + O(G²))`, `G = MᵀM − I`, its
/// columns in double-double (the `O(G²)` term is about `u²`, far below
/// a double-double's last place at these scales), and a point's height
/// over that plane. Independent of the `Interval` arithmetic it judges.
#[cfg(test)]
pub(crate) mod polar_truth {
    use geom_core::{Interval, Point3};

    /// A double-double `hi + lo`.
    #[derive(Clone, Copy)]
    pub(crate) struct Dd(f64, f64);

    fn two_sum(a: f64, b: f64) -> Dd {
        let s = a + b;
        let v = s - a;
        Dd(s, (a - (s - v)) + (b - v))
    }

    fn add(a: Dd, b: Dd) -> Dd {
        let Dd(s, e) = two_sum(a.0, b.0);
        let Dd(h, l) = two_sum(s, e + a.1 + b.1);
        Dd(h, l)
    }

    fn mul(a: Dd, b: Dd) -> Dd {
        let p = a.0 * b.0;
        let e = a.0.mul_add(b.0, -p) + (a.0 * b.1 + a.1 * b.0);
        let Dd(h, l) = two_sum(p, e);
        Dd(h, l)
    }

    fn of(x: f64) -> Dd {
        Dd(x, 0.0)
    }

    /// The stored motion's columns `u`, `v`, `w`.
    fn columns(frame: &geom::Frame<f64>) -> [[f64; 3]; 3] {
        let m = frame.motion();
        [m.u().get(), m.v().get(), m.w().get()].map(|c| [c.x, c.y, c.z])
    }

    /// The polar factor's column `j`, in double-double.
    pub(crate) fn column(frame: &geom::Frame<f64>, j: usize) -> [Dd; 3] {
        let c = columns(frame);
        let dot = |a: usize, b: usize| {
            (0..3).fold(of(0.0), |acc, k| add(acc, mul(of(c[a][k]), of(c[b][k]))))
        };
        let g = |a: usize, b: usize| {
            let d = dot(a, b);
            if a == b { add(d, of(-1.0)) } else { d }
        };
        let half = of(-0.5);
        std::array::from_fn(|k| {
            (0..3).fold(of(c[j][k]), |acc, i| {
                add(acc, mul(half, mul(of(c[i][k]), g(i, j))))
            })
        })
    }

    /// `p − origin` along the polar factor's column `j`: `p`'s height
    /// over the polar plane at `j = 2`.
    pub(crate) fn coordinate(frame: &geom::Frame<f64>, p: Point3<f64>, j: usize) -> Dd {
        let c = column(frame, j);
        let o = frame.origin();
        let d = [two_sum(p.x, -o.x), two_sum(p.y, -o.y), two_sum(p.z, -o.z)];
        (0..3).fold(of(0.0), |acc, k| add(acc, mul(d[k], c[k])))
    }

    /// `p`'s height over the polar plane through the stored origin.
    pub(crate) fn height(frame: &geom::Frame<f64>, p: Point3<f64>) -> Dd {
        coordinate(frame, p, 2)
    }

    /// Whether the enclosure holds the double-double, decided exactly
    /// on its two parts.
    pub(crate) fn holds(i: Interval, x: Dd) -> bool {
        use geom_core::Bounds;
        let above = |e: f64| {
            let Dd(s, r) = two_sum(x.0, -e);
            s + (r + x.1) >= 0.0
        };
        let below = |e: f64| {
            let Dd(s, r) = two_sum(x.0, -e);
            s + (r + x.1) <= 0.0
        };
        above(i.lo()) && below(i.hi())
    }

    /// `n` tilted frames placed out to `reach` metres and a point on
    /// each a near-graze `h` off the stored plane, `reach` out along its
    /// seam: a seeded LCG, so a red names its draw.
    pub(crate) fn poses(n: usize, reach: f64) -> Vec<(geom::Frame<f64>, Point3<f64>)> {
        use geom_core::Vec3;
        let mut state: u64 = 0x9e37_79b9_7f4a_7c15;
        let mut next = move || {
            state = state
                .wrapping_mul(6_364_136_223_846_793_005)
                .wrapping_add(1_442_695_040_888_963_407);
            (state >> 11) as f64 / (1u64 << 53) as f64 * 2.0 - 1.0
        };
        (0..n)
            .map(|_| {
                let axis = Vec3::new(next(), next(), next() + 2.0);
                let seam = Vec3::new(next() + 2.0, next(), next());
                let origin = Point3::new(next() * reach, next() * reach, next() * reach);
                let frame = geom::Frame::new(origin, axis, seam);
                let (a, b, h) = (reach * (1.0 + next()), reach * next(), 1e-12 * next());
                let p = origin + frame.u_ref() * a + frame.v_ref() * b + frame.axis() * h;
                (frame, p)
            })
            .collect()
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
            frame: geom::Frame::new(
                Point3::new(0.0, 0.0, 0.0),
                Vec3::new(0.0, 0.0, 1.0),
                Vec3::new(1.0, 0.0, 0.0),
            ),
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

    /// **The torus's form over a box holds the quartic at the box's
    /// points, and certifies wherever the expanded quartic alone does.**
    /// Seeded frame boxes on four tori, from a box `1e-12` wide to one
    /// `1e150` wide, many straddling the axis (the radial enclosure's
    /// lower end exactly zero): each box's value meets both spellings'
    /// own point enclosures at its 27 corner, edge and centre points, so
    /// no slack enters. Red against either factor read without its `z²`,
    /// and against a meet that refuses or keeps a `Trv` root at the axis.
    #[test]
    fn a_torus_form_over_a_box_holds_its_points_at_every_scale() {
        use geom_core::Interval;
        use geom_core::interval::certification::Certification;
        let mut state: u64 = 0x2545_f491_4f6c_dd1d;
        let mut next = move || {
            state = state
                .wrapping_mul(6_364_136_223_846_793_005)
                .wrapping_add(1_442_695_040_888_963_407);
            (state >> 11) as f64 / (1u64 << 53) as f64 * 2.0 - 1.0
        };
        let expanded = |major: f64, minor: f64, q: Vec3<Interval>| {
            let (major, minor) = (Interval::point(major), Interval::point(minor));
            let radial = q.x.sqr() + q.y.sqr();
            let s = radial + q.z.sqr() + major.sqr() - minor.sqr();
            s.sqr() - Interval::point(4.0) * major.sqr() * radial
        };
        let overlaps = |a: Interval, b: Interval| a.lo() <= b.hi() && b.lo() <= a.hi();
        let mut axial = 0;
        for (big, small) in [(5.0, 0.06), (1.0, 0.9), (1e-6, 1e-8), (1e3, 1.0)] {
            let form = ImplicitForm::of(&torus(big, small)).expect("a ring torus");
            for scale in [1e-12, 1e-3, 1.0, 1e3, 1e150] {
                for _ in 0..100 {
                    let side = |c: f64, w: f64| {
                        let w = w.abs() * scale;
                        Interval::from_bounds(c - w, c + w)
                    };
                    let reach = big + small;
                    let q = Vec3::new(
                        side(next() * reach * 1.5, next()),
                        side(next() * reach * 1.5, next()),
                        side(next() * small * 2.0, next()),
                    );
                    let f = form.value(q);
                    let e = expanded(big, small, q);
                    assert_eq!(f.is_certified(), e.is_certified(), "{q:?}: {f:?} vs {e:?}");
                    if !f.is_certified() {
                        continue;
                    }
                    if q.x.lo() <= 0.0 && 0.0 <= q.x.hi() && q.y.lo() <= 0.0 && 0.0 <= q.y.hi() {
                        axial += 1;
                    }
                    let ends = |i: Interval| [i.lo(), i.hi(), 0.5 * i.lo() + 0.5 * i.hi()];
                    for x in ends(q.x) {
                        for y in ends(q.y) {
                            for z in ends(q.z) {
                                let p = Vec3::new(
                                    Interval::point(x),
                                    Interval::point(y),
                                    Interval::point(z),
                                );
                                for at in [expanded(big, small, p), form.value(p)] {
                                    assert!(
                                        at.is_certified() && overlaps(f, at),
                                        "({x:e}, {y:e}, {z:e}) reads {at:?} outside {f:?}"
                                    );
                                }
                            }
                        }
                    }
                }
            }
        }
        assert!(
            axial > 100,
            "only {axial} certified boxes straddle the axis"
        );
    }

    /// **A thin torus's form clears a box at the box's own distance.**
    /// Lily wall 8's stem, `R = 5`, `r = 0.06` (`R/r ≈ 83`): a cube of
    /// side `w` centred `d = 1 cm` outside the tube on the outer equator
    /// and one `d` inside it on the inner equator. Each excludes zero
    /// once `w` is a fifth of `d`, where the expanded quartic's spread of
    /// about `16R³w` swallows `8R²r·d` until `w ≲ d/170`. Each box's
    /// enclosure holds the quartic at a 9³ grid of its points. Red under
    /// the expanded quartic alone.
    #[test]
    fn a_thin_torus_form_clears_a_box_at_its_own_distance() {
        let (big, small) = (5.0, 0.06);
        let form = ImplicitForm::of(&torus(big, small)).expect("a ring torus");
        let quartic = |p: Point3<f64>| {
            let radial = p.x * p.x + p.y * p.y;
            let s = radial + p.z * p.z + big * big - small * small;
            s * s - 4.0 * big * big * radial
        };
        let (d, w) = (0.01, 0.01 / 5.0);
        for (centre, sign) in [
            (Point3::new(big + small + d, 0.0, 0.0), 1.0),
            (Point3::new(big - small + d, 0.0, 0.0), -1.0),
        ] {
            let side = |c: f64| geom_core::Interval::from_bounds(c - w / 2.0, c + w / 2.0);
            let b = super::Box3 {
                x: side(centre.x),
                y: side(centre.y),
                z: side(centre.z),
            };
            let f = form.value(form.framed(b));
            assert!(
                f.is_certified() && f.lo() * sign > 0.0 && f.hi() * sign > 0.0,
                "the box {d:e} off the tube at {centre:?} does not exclude zero: {f:?}"
            );
            for i in 0..9 {
                for j in 0..9 {
                    for k in 0..9 {
                        let at = |c: f64, n: usize| c - w / 2.0 + w * n as f64 / 8.0;
                        let p = Point3::new(at(centre.x, i), at(centre.y, j), at(centre.z, k));
                        let truth = quartic(p);
                        let slack = 1e-13 * 4.0 * big.powi(4);
                        assert!(
                            f.lo() - slack <= truth && truth <= f.hi() + slack,
                            "{truth:e} at {p:?} lies outside {f:?}"
                        );
                    }
                }
            }
        }
    }

    /// **A partner form reads a stored frame on its polar factor**
    /// (decision 1, PR 7131): the carrier the `Interval` lift reads, so
    /// one certification never reads two shapes of one partner. 4,096
    /// tilted frames placed out to 10 km, each with a point a near-graze
    /// `≤ 1e-12` off its stored plane 10 to 20 km out along the seam: each
    /// of the point's framed coordinates holds its double-double truth
    /// on the polar factor ([`super::polar_truth`]). Red under the stored
    /// entries lifted as points and squared at the scalar (the old
    /// `ImplicitForm::of`; first at draw 456), and under them read raw,
    /// unsquared (draw 36).
    #[test]
    fn a_partner_form_reads_a_stored_frame_on_its_polar_factor() {
        use super::polar_truth::{coordinate, holds, poses};
        for (n, (frame, p)) in poses(4096, 1e4).into_iter().enumerate() {
            for surface in [
                Surface::Plane { frame },
                Surface::Cylinder { frame, radius: 1e4 },
            ] {
                let form = ImplicitForm::of(&surface).expect("a partner");
                let q = form.framed_point(p);
                for (j, x) in [q.x, q.y, q.z].into_iter().enumerate() {
                    assert!(
                        holds(x, coordinate(&frame, p, j)),
                        "draw {n}, column {j}: {x:?} misses the polar truth"
                    );
                }
            }
        }
    }
}
