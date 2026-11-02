//! **Which side of a face one bound of a vertex's sector leaves on**:
//! the one reading of a bound's side, taken by the boolean's sector
//! passes (`boolean::sectors`) and the split's neighbourhood walk
//! ([`crate::splitting::entry_side`]) alike.
//!
//! A bound is a ray out of a vertex with what stands behind it
//! ([`Reach`]): a line edge's far vertex, a curved edge's departure
//! tangent and bend, a probe's straight ray, or a subdivision bisector,
//! which bends as its sector's face does. The face is read through its
//! datum at the vertex ([`FaceDatum`]): the normal that names its sides,
//! how it bends ([`FaceBend`]), and the lever its curvature is charged
//! at.
//!
//! The reading is first order, then one order down where first order
//! ties ([`read`], [`Tie::descend`]):
//!
//! - **First order** (`d̂·n̂` at the bound's reach, a line edge's far
//!   vertex in metres), charged for the face's curvature where the
//!   datum carries a lever: decided off the band, the bound is
//!   [`FirstOrder::Off`] the face on that side.
//! - **A first-order tie** is a [`Tie`], which [`Tie::descend`] reads
//!   one order down, the bound's bend past the face's normal section
//!   along it, on the first order's own ladder; within the zero band,
//!   it rides the face, [`Descent::Rides`]. A parting off the zero band
//!   stands on its witness, the bound's own point at the reach the
//!   parting was read at, against the face's carrier by the carrier's
//!   residual: decided on the parting's side (on either, where the
//!   parting lies in the gap), the bound leaves the face tangent to it,
//!   [`Descent::Leaves`]; within the zero band, it rides it; in the gap
//!   between the zero band and the escalation band, or where the bend is
//!   not certified (a spline face, a spiric or spline edge against a
//!   curved face), [`Descent::Unresolved`], which its consumers read as
//!   the first order's tie; a witness decided against the parting, none,
//!   or a poisoned reading refuses.
//!
//! A consumer that keys on the first-order tie reads [`read`] and keeps
//! the [`Tie`] whole; one that wants the side reads
//! [`Tie::descend`] too.

use geom_brep::recourse::Refused;
use geom_brep::{ConicBend, EntersMaterial, LeverEscalation, OutwardNormal, ReferenceNormal};
use geom_core::{Band, Decide, Decided, Indeterminate, Margin, Mat3, Point3, Sign, Vec3};

/// What stands behind a sector bound, which decides how its side of a
/// face is read ([`read`]). A bound is its edge's departure from the
/// base vertex: a line's chord, a curved edge's tangent there.
#[derive(Clone, Copy, Debug)]
pub(crate) enum Reach<T: geom_core::Real> {
    /// A line edge: its far vertex, read in metres off the base vertex.
    Chord {
        /// The sector's base vertex.
        base: Point3<T>,
        /// The edge's other end.
        far: Point3<T>,
    },
    /// A probe direction with no edge behind it, levered at a stated
    /// arm: a hand-stated row's probe, a straight ray.
    #[cfg_attr(not(test), expect(dead_code))]
    Extent(T),
    /// A curved edge — a conic, a spiric or a spline: its departure
    /// tangent at the base vertex, levered at the edge's own extent,
    /// and how it bends off that tangent ([`ConicBend`]), which
    /// [`Tie::descend`] reads where the tangent ties with a face.
    Conic {
        /// The edge's extent from below ([`geom_brep::edge_extent_inf`]):
        /// no longer than its arc, so a point at that arc length lies on
        /// the edge.
        extent: T,
        /// The edge's bend at the base vertex.
        bend: ConicBend<T>,
        /// The edge's arc out of the base vertex, where a witness point
        /// of it is read ([`Tie::descend`]): a circle's or an ellipse's;
        /// `None` for a spiric or a spline, whose bend is not certified,
        /// and for a conic only ever read against a plane, whose lift
        /// [`geom_brep::conic_bend_side`] certifies.
        arc: Option<ConicArc<T>>,
    },
    /// A subdivision bisector: a direction with no point behind it,
    /// levered at its sector's arm, and how its sector's face bends
    /// along it: the face's normal section's curvature vector
    /// ([`FaceBend::section`] along the face's normal), `None` on a
    /// spline face, whose bend is not certified.
    Bisector {
        /// The sector's arm.
        arm: T,
        /// The face's normal section's curvature vector along the
        /// bisector, `None` on a spline face.
        bend: Option<Vec3<T>>,
        /// Where the bisector stands on its sector's face: `None` for a
        /// hand-stated bisector with no face behind it.
        on: Option<OnFace<T>>,
    },
}

impl<T: geom_core::Real> Reach<T> {
    /// The length the bound reaches from its base vertex, in metres:
    /// a line edge's chord, a curved edge's extent, a bisector's arm.
    pub(crate) fn length(self) -> T {
        match self {
            Self::Chord { base, far } => (far - base).norm(),
            Self::Extent(l) | Self::Conic { extent: l, .. } | Self::Bisector { arm: l, .. } => l,
        }
    }

    /// The point that fixes a bound leaving `base` along `dir`: a line
    /// edge's far vertex, and any other bound's direction out to its
    /// reach ([`Self::length`]) — a curved edge's departure tangent at
    /// its extent, a probe's or a bisector's direction at its arm. A
    /// gathered cone's rays and own edges are read at it
    /// ([`crate::link::Cone::gathered`]).
    pub(crate) fn point(self, base: Point3<T>, dir: Vec3<T>) -> Point3<T> {
        match self {
            Self::Chord { far, .. } => far,
            Self::Extent(_) | Self::Conic { .. } | Self::Bisector { .. } => {
                base + dir.normalize() * self.length()
            }
        }
    }
}

impl<T: Decide> Reach<T> {
    /// A bisector of a sector on `face` (outward unit normal `n`) along
    /// the unit `dir`, levered at `arm`: it bends as the face's normal
    /// section along it.
    pub(crate) fn bisector(
        arm: T,
        dir: Vec3<T>,
        n: Vec3<T>,
        face: FaceBend<T>,
        base: Point3<T>,
    ) -> Self {
        Self::Bisector {
            arm,
            bend: face.section(dir, n).map(|a| n * a),
            on: Some(OnFace { base, face }),
        }
    }

    /// A bisector of a straight section, levered at `arm`: a ruling of
    /// a face at `base`, which lies in the face.
    pub(crate) fn ruling(arm: T, base: Point3<T>) -> Self {
        Self::Bisector {
            arm,
            bend: Some(Vec3::zero()),
            on: Some(OnFace {
                base,
                face: FaceBend::Plane,
            }),
        }
    }

    /// A hand-stated straight bisector levered at `arm`, with no face
    /// behind it.
    #[cfg(test)]
    pub(crate) fn straight_bisector(arm: T) -> Self {
        Self::Bisector {
            arm,
            bend: Some(Vec3::zero()),
            on: None,
        }
    }

    /// The bound's base vertex, where it has a point behind it
    /// ([`Self::witness`]).
    fn base(self) -> Option<Point3<T>> {
        match self {
            Self::Chord { base, .. } => Some(base),
            Self::Extent(_) => None,
            Self::Conic { arc, .. } => arc.map(|arc| arc.at(T::zero())),
            Self::Bisector { on, .. } => on.map(|on| on.base),
        }
    }

    /// **The bound's point at `l` from its base vertex**, `l` no longer
    /// than [`Self::length`]: a point of the edge, or of a bisector's
    /// sector face, that a witness reads ([`Tie::descend`]). A line
    /// edge's point on its chord; a conic's at an arc no longer than
    /// `l` ([`ConicArc::at`]); a bisector's direction carried to its
    /// face's carrier ([`Carrier::foot`]). `None` where the bound has no
    /// point behind it: a probe, a hand-stated bisector, a spiric or a
    /// spline edge, or a foot that does not read.
    pub(crate) fn witness(self, dir: Vec3<T>, l: T) -> Option<Point3<T>> {
        match self {
            Self::Chord { base, far } => Some(base + (far - base) * (l / (far - base).norm())),
            Self::Extent(_) => None,
            Self::Conic { arc, .. } => arc.map(|arc| arc.at(l)),
            Self::Bisector { on, .. } => on.and_then(|on| {
                let straight = on.base + dir.normalize() * l;
                match on.face {
                    FaceBend::Plane => Some(straight),
                    FaceBend::Implicit { carrier, .. } => carrier.foot(straight),
                    FaceBend::Fitted => None,
                }
            }),
        }
    }

    /// The bound's signed first-order departure from the plane through
    /// its base vertex with unit normal `n`, as [`read`] reads it: a
    /// line edge's far vertex, in metres; any other bound's direction
    /// `dir` levered at its reach ([`rise`]). The vertex-on-face pass
    /// quotes it for a sector whose bounds tie but whose tilt it cannot
    /// decide (`bool_sector_coplanar`): the steeper bound's first-order
    /// departure is what a smaller tolerance reads off the plane. A
    /// conic's bend is [`Tie::descend`]'s.
    pub(crate) fn departure(self, dir: Vec3<T>, n: Vec3<T>) -> T {
        match self {
            Self::Chord { base, far } => crate::sector_shape::plane_offset(base, n, far),
            Self::Extent(l) | Self::Conic { extent: l, .. } | Self::Bisector { arm: l, .. } => {
                rise(dir, n, l)
            }
        }
    }
}

/// A direction's first-order rise off the plane with unit normal `n`,
/// levered at `arm`: `d̂·n̂·arm`, in metres.
pub(crate) fn rise<T: Decide>(dir: Vec3<T>, n: Vec3<T>, arm: T) -> T {
    geom_brep::rise(dir, n, arm).value()
}

/// **Whether a face is a plane** ([`FaceBend::shape`]): on a plane a
/// bound in it lies in the face, while a bound in a curved face's
/// tangent plane rides it to first order only.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum FaceShape {
    /// The face lies on a plane.
    Plane,
    /// The face lies on a curved surface.
    Curved,
}

impl FaceShape {
    /// The shape of a face on a surface of `kind`.
    pub(crate) fn of_kind(kind: geom::SurfaceKind) -> Self {
        match kind {
            geom::SurfaceKind::Plane => Self::Plane,
            _ => Self::Curved,
        }
    }
}

/// **How a face bends at the vertex**, as a bound tied with its tangent
/// plane is read against it one order down ([`Tie::descend`]).
#[derive(Clone, Copy, Debug)]
pub(crate) enum FaceBend<T: geom_core::Real> {
    /// A plane: it does not bend.
    Plane,
    /// A carrier with an implicit form `F`: its gradient and Hessian at
    /// the vertex, which give its normal section along any tangent, and
    /// its smallest radius of curvature there, which the descent reads
    /// no further than.
    Implicit {
        /// `∇F` at the vertex.
        gradient: Vec3<T>,
        /// `∇²F` at the vertex ([`geom_brep::implicit_hessian`]).
        hessian: Mat3<T>,
        /// `1/κ_max` at the vertex
        /// ([`geom_brep::implicit_max_normal_curvature`]).
        radius: T,
        /// The carrier itself, which a witness point's offset is read
        /// against.
        carrier: Carrier<T>,
    },
    /// A spline face: no implicit form, so no certified bend.
    Fitted,
}

impl<T: geom_core::Real> FaceBend<T> {
    /// How `surface` bends at its point `p`.
    pub(crate) fn at(surface: &geom::Surface<T>, p: Point3<T>) -> Self {
        if surface.kind() == geom::SurfaceKind::Plane {
            return Self::Plane;
        }
        match Carrier::of(surface) {
            None => Self::Fitted,
            Some(carrier) => Self::Implicit {
                gradient: geom_brep::implicit_gradient(surface, p),
                hessian: geom_brep::implicit_hessian(surface, p),
                radius: T::one() / geom_brep::implicit_max_normal_curvature(surface, p),
                carrier,
            },
        }
    }

    /// Whether the face is a plane.
    pub(crate) fn shape(self) -> FaceShape {
        match self {
            Self::Plane => FaceShape::Plane,
            Self::Implicit { .. } | Self::Fitted => FaceShape::Curved,
        }
    }

    /// The face's normal section's acceleration along the unit tangent
    /// `d`, toward the unit normal `n`, per arc length squared
    /// ([`geom_brep::graph_acceleration`]): zero on a plane, `None` on a
    /// spline face.
    pub(crate) fn section(self, d: Vec3<T>, n: Vec3<T>) -> Option<T> {
        match self {
            Self::Plane => Some(T::zero()),
            Self::Implicit {
                gradient, hessian, ..
            } => Some(geom_brep::graph_acceleration(
                d.dot(hessian * d),
                gradient,
                n,
            )),
            Self::Fitted => None,
        }
    }

    /// The length the face's bend is read over: its smallest radius of
    /// curvature, `None` on a plane or a spline face.
    fn radius(self) -> Option<T> {
        match self {
            Self::Implicit { radius, .. } => Some(radius),
            Self::Plane | Self::Fitted => None,
        }
    }
}

#[cfg(test)]
impl FaceBend<f64> {
    /// A sphere of `radius` whose implicit gradient at the vertex is the
    /// unit `gradient`: read against an outward normal along the
    /// gradient, a ball, which bends off its tangent plane against the
    /// normal; against it, a hollow, which bends toward it.
    pub(crate) fn ball(gradient: Vec3<f64>, radius: f64) -> Self {
        let k = 1.0 / radius;
        Self::Implicit {
            carrier: Carrier::Sphere {
                center: Point3::origin() - gradient * radius,
                radius,
                axis: Vec3::new(0.0, 0.0, 1.0),
                u_ref: Vec3::new(1.0, 0.0, 0.0),
            },
            gradient,
            hessian: Mat3::from_cols(
                Vec3::new(k, 0.0, 0.0),
                Vec3::new(0.0, k, 0.0),
                Vec3::new(0.0, 0.0, k),
            ),
            radius,
        }
    }

    /// A cylinder of `radius` about the unit `axis` whose implicit
    /// gradient at the vertex is the unit `gradient`, read as
    /// [`Self::ball`] is: it holds a line along `axis`.
    pub(crate) fn cylinder(gradient: Vec3<f64>, axis: Vec3<f64>, radius: f64) -> Self {
        let k = 1.0 / radius;
        let col = |e: Vec3<f64>| (e - axis * axis.dot(e)) * k;
        Self::Implicit {
            carrier: Carrier::Cylinder {
                origin: Point3::origin() - gradient * radius,
                axis,
                radius,
                u_ref: gradient,
            },
            gradient,
            hessian: Mat3::from_cols(
                col(Vec3::new(1.0, 0.0, 0.0)),
                col(Vec3::new(0.0, 1.0, 0.0)),
                col(Vec3::new(0.0, 0.0, 1.0)),
            ),
            radius,
        }
    }
}

/// **A face's carrier with an implicit form**, kept beside its bend so
/// that a witness point's offset from it is the carrier's own residual
/// ([`geom_brep::implicit_residual`]), the one the root doors read.
#[derive(Clone, Copy, Debug)]
pub(crate) enum Carrier<T: geom_core::Real> {
    /// [`geom::Surface::Cylinder`].
    Cylinder {
        origin: Point3<T>,
        axis: Vec3<T>,
        radius: T,
        u_ref: Vec3<T>,
    },
    /// [`geom::Surface::Cone`].
    Cone {
        apex: Point3<T>,
        axis: Vec3<T>,
        half_angle: T,
        u_ref: Vec3<T>,
    },
    /// [`geom::Surface::Sphere`].
    Sphere {
        center: Point3<T>,
        radius: T,
        axis: Vec3<T>,
        u_ref: Vec3<T>,
    },
    /// [`geom::Surface::Torus`].
    Torus {
        center: Point3<T>,
        axis: Vec3<T>,
        major_radius: T,
        minor_radius: T,
        u_ref: Vec3<T>,
    },
}

impl<T: geom_core::Real> Carrier<T> {
    /// A curved carrier with an implicit form; `None` for a plane, a
    /// spline or a fitted surface.
    fn of(surface: &geom::Surface<T>) -> Option<Self> {
        match *surface {
            geom::Surface::Cylinder {
                origin,
                axis,
                radius,
                u_ref,
            } => Some(Self::Cylinder {
                origin,
                axis,
                radius,
                u_ref,
            }),
            geom::Surface::Cone {
                apex,
                axis,
                half_angle,
                u_ref,
            } => Some(Self::Cone {
                apex,
                axis,
                half_angle,
                u_ref,
            }),
            geom::Surface::Sphere {
                center,
                radius,
                axis,
                u_ref,
            } => Some(Self::Sphere {
                center,
                radius,
                axis,
                u_ref,
            }),
            geom::Surface::Torus {
                center,
                axis,
                major_radius,
                minor_radius,
                u_ref,
            } => Some(Self::Torus {
                center,
                axis,
                major_radius,
                minor_radius,
                u_ref,
            }),
            _ => None,
        }
    }

    fn surface(self) -> geom::Surface<T> {
        match self {
            Self::Cylinder {
                origin,
                axis,
                radius,
                u_ref,
            } => geom::Surface::Cylinder {
                origin,
                axis,
                radius,
                u_ref,
            },
            Self::Cone {
                apex,
                axis,
                half_angle,
                u_ref,
            } => geom::Surface::Cone {
                apex,
                axis,
                half_angle,
                u_ref,
            },
            Self::Sphere {
                center,
                radius,
                axis,
                u_ref,
            } => geom::Surface::Sphere {
                center,
                radius,
                axis,
                u_ref,
            },
            Self::Torus {
                center,
                axis,
                major_radius,
                minor_radius,
                u_ref,
            } => geom::Surface::Torus {
                center,
                axis,
                major_radius,
                minor_radius,
                u_ref,
            },
        }
    }

    /// The carrier's residual at `p` ([`geom_brep::implicit_residual`]):
    /// positive along its gradient, in metres.
    fn residual(self, p: Point3<T>) -> T {
        geom_brep::implicit_residual(&self.surface(), p)
    }

    /// **A bound on [`Self::residual`]'s own rounding at `p`**, in
    /// metres: its squared terms (`ρ² − R²` and the like), each rounded
    /// at their own size, over its divisor, plus the coordinates'
    /// subtraction carried through them. A residual far smaller than
    /// the carrier's own size cancels, and this is what it may have
    /// lost.
    fn rounding(self, p: Point3<T>) -> T {
        let eps = T::from_f64(4.0 * f64::EPSILON);
        let at = |q: Point3<T>| (q - Point3::origin()).norm();
        let split = |anchor: Point3<T>, axis: Vec3<T>| {
            let q = p - anchor;
            let h = q.dot(axis);
            ((q - axis * h).norm(), h.abs(), at(p) + at(anchor))
        };
        let two = T::from_f64(2.0);
        match self {
            Self::Sphere { center, radius, .. } => {
                let (d, coords) = ((p - center).norm(), at(p) + at(center));
                eps * (d.powi(2) + radius.powi(2) + two * d * coords) / (two * radius)
            }
            Self::Cylinder {
                origin,
                axis,
                radius,
                ..
            } => {
                let (rho, _, coords) = split(origin, axis);
                eps * (rho.powi(2) + radius.powi(2) + two * rho * coords) / (two * radius)
            }
            Self::Torus {
                center,
                axis,
                major_radius,
                minor_radius,
                ..
            } => {
                let (rho, h, coords) = split(center, axis);
                let d = (rho - major_radius).abs();
                eps * (d.powi(2)
                    + h.powi(2)
                    + minor_radius.powi(2)
                    + two * (d + h) * (coords + rho + major_radius))
                    / (two * minor_radius)
            }
            Self::Cone { apex, .. } => eps * ((p - apex).norm() + at(p) + at(apex)),
        }
    }

    /// **The point of the carrier nearest `p`**: the closed-form foot
    /// the boolean's carrier distance reads
    /// (`boolean::carrier_touch::distance`) on a sphere, a cylinder or a
    /// torus; on a cone, `p` less its residual along the residual's unit
    /// gradient, exact where the foot lies on the nappe (the residual is
    /// the distance there, [`geom_brep::cone_elevation`]). `None` where
    /// the foot is poison (`p` on an axis, a cone's apex).
    fn foot(self, p: Point3<T>) -> Option<Point3<T>> {
        let surface = self.surface();
        let foot = match self {
            Self::Cone { .. } => {
                p - geom_brep::implicit_gradient(&surface, p)
                    * geom_brep::implicit_residual(&surface, p)
            }
            Self::Cylinder { .. } | Self::Sphere { .. } | Self::Torus { .. } => {
                crate::boolean::carrier_touch::distance(&surface, p)?.1
            }
        };
        geom_core::is_finite_length(foot.x + foot.y + foot.z).then_some(foot)
    }
}

/// **A conic edge's arc out of its vertex**, as [`Reach::witness`]
/// reads a point of it: the circle or ellipse, and its parameters at
/// the vertex and the far end.
#[derive(Clone, Copy, Debug)]
pub(crate) struct ConicArc<T: geom_core::Real> {
    center: Point3<T>,
    axis: Vec3<T>,
    semi: (T, T),
    u_ref: Vec3<T>,
    circle: bool,
    from: T,
    to: T,
}

impl<T: geom_core::Real> ConicArc<T> {
    /// The arc of `curve` its walk leaves the vertex along (`he_plus`
    /// walks `t₀ → t₁`): `None` for a carrier that is not a circle or
    /// an ellipse.
    pub(crate) fn of(curve: &geom_brep::EdgeCurve<T>, he_plus: bool) -> Option<Self> {
        let (t0, t1) = curve.params();
        let (from, to) = if he_plus { (t0, t1) } else { (t1, t0) };
        Self::on(curve.carrier(), from, to)
    }

    /// The arc of `carrier` from its parameter `from` to `to`: `None`
    /// for a carrier that is not a circle or an ellipse.
    pub(crate) fn on(carrier: &geom::Curve3<T>, from: T, to: T) -> Option<Self> {
        let (center, axis, semi, u_ref, circle) = match *carrier {
            geom::Curve3::Circle {
                center,
                axis,
                radius,
                u_ref,
            } => (center, axis, (radius, radius), u_ref, true),
            geom::Curve3::Ellipse {
                center,
                axis,
                semi_u,
                semi_v,
                u_ref,
            } => (center, axis, (semi_u, semi_v), u_ref, false),
            _ => return None,
        };
        Some(Self {
            center,
            axis,
            semi,
            u_ref,
            circle,
            from,
            to,
        })
    }
}

impl<T: Decide> ConicArc<T> {
    /// The arc's point at most `l` along it from the vertex: its
    /// parameter advanced by `l` over the carrier's greatest speed (a
    /// circle's radius, an ellipse's larger semi-axis), no further than
    /// the far end, so the point lies on the edge.
    fn at(self, l: T) -> Point3<T> {
        let (a, b) = self.semi;
        let speed = a.abs().max(b.abs());
        let span = self.to - self.from;
        let t = self.from + span * (l / (speed * span.abs())).min(T::one());
        let carrier = if self.circle {
            geom::Curve3::Circle {
                center: self.center,
                axis: self.axis,
                radius: a,
                u_ref: self.u_ref,
            }
        } else {
            geom::Curve3::Ellipse {
                center: self.center,
                axis: self.axis,
                semi_u: a,
                semi_v: b,
                u_ref: self.u_ref,
            }
        };
        carrier.eval(t)
    }
}

/// **Where a subdivision bisector stands**: its sector's base vertex
/// and the face it lies in.
#[derive(Clone, Copy, Debug)]
pub(crate) struct OnFace<T: geom_core::Real> {
    /// The sector's base vertex.
    pub base: Point3<T>,
    /// The sector's face at the vertex.
    pub face: FaceBend<T>,
}

/// **A face at the vertex, as a bound's side is read against it**: the
/// normal naming its sides, how it bends, and the lever its curvature
/// is charged at (`None`: a splitting plane, which has no curvature and
/// takes no charge).
#[derive(Clone, Copy, Debug)]
pub(crate) struct FaceDatum<T: geom_core::Real> {
    /// The normal: against it is [`Side::Below`] (into a face's
    /// material), along it [`Side::Above`].
    pub reference: ReferenceNormal<T>,
    /// How the face bends at the vertex.
    pub bend: FaceBend<T>,
    /// The face's smallest radius of curvature within the bound's reach
    /// (`boolean::sectors::PierceLever`), where a definite first-order
    /// verdict is charged for it.
    pub lever: Option<T>,
}

impl<T: geom_core::Real> FaceDatum<T> {
    /// A face's datum: its outward normal at the vertex, its bend, and
    /// its curvature lever.
    pub(crate) fn of_face(normal: OutwardNormal<T>, bend: FaceBend<T>, lever: T) -> Self {
        Self {
            reference: ReferenceNormal::of_face_outward(normal),
            bend,
            lever: Some(lever),
        }
    }

    /// A plane's datum with no curvature charge: a splitting plane, or a
    /// face's plane a section is read against.
    pub(crate) fn of_plane(reference: ReferenceNormal<T>) -> Self {
        Self {
            reference,
            bend: FaceBend::Plane,
            lever: None,
        }
    }
}

/// A side of a face: against its datum's normal, or along it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Side {
    /// Against the normal: into a face's material.
    Below,
    /// Along the normal: out of a face's material.
    Above,
}

impl Side {
    fn of(verdict: EntersMaterial) -> Option<Self> {
        match verdict {
            EntersMaterial::Enters => Some(Self::Below),
            EntersMaterial::Exits => Some(Self::Above),
            EntersMaterial::Tangent => None,
        }
    }
}

/// **A bound's side of a face, to the order it is read.**
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum BoundSide {
    /// Decided at first order.
    Off(Side),
    /// Tied at first order, and read one order down ([`Tie::descend`]).
    Tied(Descent),
}

/// **A first-order tie read one order down** ([`Tie::descend`]).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Descent {
    /// Decided one order down: the bound leaves the face tangent to it,
    /// toward this side. (A first-order tie, which
    /// [`EntersMaterial::Tangent`] names, is undecided; this is decided.)
    Leaves(Side),
    /// Within the zero band one order down too: the bound rides the face
    /// to the orders read (a line edge in a plane, a conic whose whole
    /// edge lies within the band of a plane).
    Rides,
    /// Neither decided nor riding, for a reason no smaller reading here
    /// settles. Its consumers read it as the first order's tie, `On`, as
    /// they did before the descent.
    Unresolved(Unresolved),
}

/// Why a tie's descent is [`Descent::Unresolved`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Unresolved {
    /// The bound's witness at the reach its parting was read at lies
    /// between the zero band and the escalation band, on one side: the
    /// bound stands that far off the face there, which neither rides it
    /// nor leaves it at this band.
    Gap,
    /// The bend is not certified: a spline face, a bisector of one, or a
    /// spiric or spline edge against a curved face.
    Uncertified,
}

/// The first order of [`read`]: a side, or a tie [`Tie::descend`] reads
/// one order down.
#[derive(Clone, Copy, Debug)]
pub(crate) enum FirstOrder<T: geom_core::Real> {
    /// Decided at first order, and charged for the face's curvature.
    Off(Side),
    /// Tied at first order.
    Tied(Tie<T>),
}

/// **A bound tied at first order with a face**, and what the tie was
/// decided on.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Tie<T: geom_core::Real> {
    dir: Vec3<T>,
    reach: Reach<T>,
    face: FaceDatum<T>,
    /// The predicate that decided the Zero.
    pub predicate: &'static str,
    /// The Zero, with the margin it was decided on.
    pub decided: Decided,
}

/// Why a reading with no curvature charge read no side: the first
/// order ([`first_order`]) or the descent ([`Tie::descend`]). Every arm
/// carries the indeterminate it was read on.
#[derive(Clone, Copy, Debug)]
pub(crate) enum ReadFault<T: geom_core::Real> {
    /// A line edge's far vertex lies in the band of the plane
    /// (`bool_chord_side`).
    Chord(Indeterminate),
    /// The direction's arm or its first-order departure lies in the
    /// band, with the arm and the departure it read.
    Departure {
        /// The arm gate's or the reading's escalation.
        escalation: LeverEscalation,
        /// The bound's reach.
        arm: T,
        /// Its first-order departure over that reach, in metres.
        departure: T,
    },
    /// A conic's lift off a plane lies in the band of its first-order
    /// residue, or is not certified (a spiric or a spline).
    Bend(Indeterminate),
    /// The bound's parting from the face it is tied with is not read:
    /// its arm in the band, or the parting poisoned or straddling the
    /// zero band (no gap of [`Unresolved::Gap`]'s).
    Parting(LeverEscalation),
    /// The witness of a parting off the zero band does not stand it
    /// ([`Tie::descend`]): decided against the parting's side, poisoned,
    /// straddling the zero band, or no point to read.
    Witness(Indeterminate),
}

impl<T: geom_core::Real> ReadFault<T> {
    /// The indeterminate the fault was read on.
    pub(crate) fn diag(self) -> Indeterminate {
        match self {
            Self::Chord(diag) | Self::Bend(diag) | Self::Witness(diag) => diag,
            Self::Departure { escalation, .. } | Self::Parting(escalation) => escalation.diag(),
        }
    }
}

/// Why [`read`] or `read_side` read no side.
#[derive(Clone, Copy, Debug)]
pub(crate) enum BoundSideFault<T: geom_core::Real> {
    /// The reading before any charge.
    Read(ReadFault<T>),
    /// The curvature charge did not clear the band: the side is second
    /// order there.
    Charge(Refused),
    /// The curvature charge lies in the band.
    ChargeEscalated(Indeterminate),
}

/// [`first_order`]'s reading: a side with the bound's displacement off
/// the face's tangent plane, in metres, or a tie.
type Settled<T> = Result<(Side, T), Tie<T>>;

/// The first order of [`read`] before its charge: a side with the
/// bound's displacement off the face's tangent plane in metres, or a tie.
fn first_order<T: Decide>(
    dir: Vec3<T>,
    reach: Reach<T>,
    face: FaceDatum<T>,
    band: Band,
) -> Result<Settled<T>, ReadFault<T>> {
    let n = face.reference.vec();
    let length = reach.length();
    let tie = |predicate, decided| {
        Ok(Err(Tie {
            dir,
            reach,
            face,
            predicate,
            decided,
        }))
    };
    Ok(Ok(match reach {
        Reach::Chord { base, far } => {
            let offset = crate::sector_shape::plane_offset(base, n, far);
            match crate::validate::decide_reported("bool_chord_side", Margin::of(offset), band) {
                Ok(Decided {
                    sign: Sign::Negative,
                    ..
                }) => (Side::Below, offset.abs()),
                Ok(Decided {
                    sign: Sign::Positive,
                    ..
                }) => (Side::Above, offset.abs()),
                Ok(zero) => return tie("bool_chord_side", zero),
                Err(diag) => return Err(ReadFault::Chord(diag)),
            }
        }
        Reach::Extent(_) | Reach::Conic { .. } | Reach::Bisector { .. } => {
            match geom_brep::departure_side_reported(dir, face.reference, length, band) {
                Ok((verdict, margin)) => match Side::of(verdict) {
                    Some(side) => (side, rise(dir, n, length).abs()),
                    None => {
                        return tie(
                            "enters_material",
                            Decided {
                                sign: Sign::Zero,
                                margin,
                            },
                        );
                    }
                },
                Err(escalation) => {
                    return Err(ReadFault::Departure {
                        escalation,
                        arm: length,
                        departure: rise(dir, n, length),
                    });
                }
            }
        }
    }))
}

/// **A bound's side of a face at first order**, read as its [`Reach`]
/// allows, and charged for the face's curvature where the datum carries
/// a lever.
///
/// # The reading, per reach
///
/// A `Zero` here is a [`Tie`], which a consumer may key on as "this
/// bound lies in the plane". So a Zero has to be a distance, not a
/// direction times a lever that is not the bound's own length.
/// - [`Reach::Chord`] (a line edge): its far vertex's signed distance
///   from the plane through the base vertex, in metres
///   ([`crate::sector_shape::plane_offset`]). The edge is its chord, so
///   every point of it lies between the base's distance (`≈ 0`, the
///   vertex is On) and the far vertex's: a Zero puts the whole edge
///   within the band of the plane.
/// - [`Reach::Conic`] (a curved edge): the departure tangent levered at
///   the edge's own extent. A definite sign is exact (the sign of
///   `t̂·n̂`); a Zero is a first-order tangency.
/// - [`Reach::Extent`] (a probe): the direction levered at its arm.
/// - [`Reach::Bisector`]: a subdivision direction has no point behind
///   it, so it is levered at its sector's arm.
///
/// # Why a definite first-order verdict is charged on a curved face
///
/// The verdict is the bound's side NEAR THE VERTEX: a bound whose
/// first-order departure `s = |d̂·n̂|` from the face's tangent plane
/// is nonzero lies on that side of the face for every small enough
/// distance, whatever the face's curvature and whatever the bound's
/// own. What the charge asks is whether that side is RESOLVED at the
/// band.
///
/// It reads the bound's tangent ray, not points of the bound. At
/// distance `l` along the ray, the face departs its tangent plane by
/// at most the sagitta `l²/lever`, so the ray sits at least
/// `g(l) = s·l − l²/lever` off the face on the verdict's side. That
/// lower bound grows faster than the first-order term, and far enough
/// out it flips sign. The witness is a hole wall (`r = 1`, material
/// outside, so the outward normal points at the axis): a direction
/// with `s ≈ 0.0995` is a definite `Exits`, and it does leave the
/// material, but by `l = 0.5` it has crossed back, and the point there
/// sits at `ρ = 1.0726`, INSIDE the material. So the charge is read
/// where `g` peaks, `l* = s·lever/2`, giving `s²·lever/4`, and capped
/// at the reach's own length: a separation witnessed only beyond the
/// bound's own extent is a direction levered at a lever that is not
/// the bound's length. The verdict stands iff that separation clears
/// the band. What refuses is a bound leaving the face within about
/// `2·sqrt(band/lever)` radians of tangent (or a reach too short to
/// witness its slope), whose side is second order.
///
/// The charge is direction-agnostic on purpose, which is sound for
/// either bend. A plane's lever is infinite, the sagitta underflows to
/// zero, and the charge reduces to the reading just decided definite.
/// A tie takes no charge: it is not a side.
pub(crate) fn read<T: Decide>(
    dir: Vec3<T>,
    reach: Reach<T>,
    face: FaceDatum<T>,
    band: Band,
) -> Result<FirstOrder<T>, BoundSideFault<T>> {
    let length = reach.length();
    let (side, displacement) = match first_order(dir, reach, face, band) {
        Ok(Ok(off)) => off,
        Ok(Err(tie)) => return Ok(FirstOrder::Tied(tie)),
        Err(fault) => return Err(BoundSideFault::Read(fault)),
    };
    let Some(lever) = face.lever else {
        return Ok(FirstOrder::Off(side));
    };
    // **The sagitta bound, and why the textbook ½ is not in it.** At
    // lateral offset `l` a circle of radius `R` departs its tangent
    // line by exactly `R − sqrt(R² − l²) = l²/(R + sqrt(R² − l²))`. The
    // familiar `l²/2R` is that expression's small-`l` LIMIT and is a
    // LOWER bound on it, so charging `l²/2R` would under-charge exactly
    // where the distance is a large fraction of the radius. Dropping
    // the ½ gives `l²/R`, an upper bound unconditionally (`l* ≤ R/2`
    // here, inside the circle's reach).
    let slope = displacement / length;
    match crate::validate::decide_reported(
        "bool_pierce_sector_side_curved",
        Margin::of(sagitta_charge(slope, lever, length)),
        band,
    ) {
        Ok(decided) => match Refused::of(decided, band) {
            None => Ok(FirstOrder::Off(side)),
            Some(refused) => Err(BoundSideFault::Charge(refused)),
        },
        Err(diag) => Err(BoundSideFault::ChargeEscalated(diag)),
    }
}

/// [`read`], descended where first order ties ([`Tie::descend`]).
#[cfg(test)]
pub(crate) fn read_side<T: Decide>(
    dir: Vec3<T>,
    reach: Reach<T>,
    face: FaceDatum<T>,
    band: Band,
) -> Result<BoundSide, BoundSideFault<T>> {
    match read(dir, reach, face, band)? {
        FirstOrder::Off(side) => Ok(BoundSide::Off(side)),
        FirstOrder::Tied(tie) => tie
            .descend(band)
            .map(BoundSide::Tied)
            .map_err(BoundSideFault::Read),
    }
}

/// **A bound's side of a plane with no curvature charge** (a splitting
/// plane's): `read_side` with no charge to refuse, where a bound whose
/// reach the band does not read as a length is tied rather than refused
/// ([`Tie::of_unread_reach`]): such an edge lies within the band of the
/// plane wherever its bend leaves it, and the descent reads it so.
pub(crate) fn read_plane_side<T: Decide>(
    dir: Vec3<T>,
    reach: Reach<T>,
    reference: ReferenceNormal<T>,
    band: Band,
) -> Result<BoundSide, ReadFault<T>> {
    let face = FaceDatum::of_plane(reference);
    let tie = match first_order(dir, reach, face, band) {
        Ok(Ok((side, _))) => return Ok(BoundSide::Off(side)),
        Ok(Err(tie)) => tie,
        Err(fault) => Tie::of_unread_reach(dir, reach, face, fault).ok_or(fault)?,
    };
    tie.descend(band).map(BoundSide::Tied)
}

/// **The curvature charge**: the least separation from a face that a
/// ray leaving it at `slope` (a sine) keeps past a bend of radius
/// `lever`, `slope·l − l²/lever`, read at `l = min(slope·lever/2,
/// reach)`, where it peaks within the reach. [`read`] carries the
/// argument and the witness; `vtxfac::faces_holding_pierce` charges a
/// held pierce's run with it.
pub(crate) fn sagitta_charge<T: Decide>(slope: T, lever: T, reach: T) -> T {
    let l = (slope * T::from_f64(0.5) * lever).min(reach);
    slope * l - l.powi(2) / lever
}

impl<T: Decide> Tie<T> {
    /// **The witness's offset from the face at `l`**, toward the datum's
    /// normal, in metres: the bound's point at `l`
    /// ([`Reach::witness`]) against a curved face's carrier by the
    /// carrier's own residual ([`Carrier::residual`]), against a plane
    /// by its height over the plane through the base vertex; and a
    /// bound on that reading's own rounding ([`Carrier::rounding`]).
    /// `None` where the bound has no point behind it or the face no
    /// carrier.
    fn witness_offset(self, l: T) -> Option<(T, T)> {
        let n = self.face.reference.vec();
        let w = self.reach.witness(self.dir, l)?;
        match self.face.bend {
            FaceBend::Plane => {
                let base = self.reach.base()?;
                let at = |q: Point3<T>| (q - Point3::origin()).norm();
                Some((
                    (w - base).dot(n),
                    T::from_f64(4.0 * f64::EPSILON) * (at(w) + at(base)),
                ))
            }
            FaceBend::Implicit {
                gradient, carrier, ..
            } => {
                let along = gradient.dot(n);
                Some((
                    carrier.residual(w) * along / along.abs(),
                    carrier.rounding(w),
                ))
            }
            FaceBend::Fitted => None,
        }
    }

    /// **A bound whose reach the band does not read as a length**, tied:
    /// `fault` is [`read`]'s, on the arm gate's rung (the reach is within
    /// the band, or decided zero there). A consumer that reads such a
    /// bound as lying within the band of the plane (the split walk: a
    /// conic edge whose whole extent is in the band) ties it here and
    /// descends it as any tie; the Zero quoted is the arm's. `None` for
    /// any other fault.
    pub(crate) fn of_unread_reach(
        dir: Vec3<T>,
        reach: Reach<T>,
        face: FaceDatum<T>,
        fault: ReadFault<T>,
    ) -> Option<Self> {
        match fault {
            ReadFault::Departure { escalation, .. }
                if escalation.rung() == geom_brep::LeverRung::Arm =>
            {
                Some(Self {
                    dir,
                    reach,
                    face,
                    predicate: "enters_material_arm",
                    decided: Decided {
                        sign: Sign::Zero,
                        margin: escalation.diag().margin,
                    },
                })
            }
            _ => None,
        }
    }

    /// **The tied bound's side one order down**: the sign of
    /// `(κ_b·n̂ − II_F(d̂, d̂))·l²/2` ([`geom_brep::parting_side`]), the
    /// bound's bend past the face's normal section along it, at the
    /// shortest of the bound's reach, the face's charge lever and its
    /// smallest radius of curvature, on the first order's own ladder:
    /// within the zero band, [`Descent::Rides`]; decided, or finite and
    /// in the gap between the zero band and the escalation band on one
    /// side, the verdict is its witness's ([`Self::witnessed`]). The
    /// bound's bend is a conic's curvature, a bisector's sector face's
    /// normal section along it, and nothing for a line edge or a probe,
    /// which are straight; a plane's section is nothing. A bend no reading
    /// certifies is [`Unresolved::Uncertified`].
    ///
    /// A conic against a plane is read by its certified lift
    /// ([`geom_brep::conic_bend_side`]), whose `Tangent` is the whole
    /// edge within the band of the plane; a line edge or a probe against
    /// a plane rides it.
    ///
    /// # Errors
    ///
    /// [`ReadFault::Bend`]: a conic's lift against a plane in the band of
    /// its first-order residue, or a spiric's or a spline's, which
    /// certify no bend. [`ReadFault::Parting`]: the parting's arm in the
    /// band, or the parting poisoned (a singular point's jet) or
    /// straddling the zero band. [`ReadFault::Witness`]: as
    /// [`Self::witnessed`].
    pub(crate) fn descend(self, band: Band) -> Result<Descent, ReadFault<T>> {
        let (face, reach) = (self.face, self.reach);
        let uncertified = Ok(Descent::Unresolved(Unresolved::Uncertified));
        let own = match (face.bend, reach) {
            (FaceBend::Fitted, _) | (_, Reach::Bisector { bend: None, .. }) => return uncertified,
            (FaceBend::Plane, Reach::Conic { extent, bend, .. }) => {
                return match geom_brep::conic_bend_side(
                    self.dir,
                    extent,
                    bend,
                    face.reference,
                    band,
                ) {
                    Ok(verdict) => Ok(Side::of(verdict).map_or(Descent::Rides, Descent::Leaves)),
                    Err(diag) => Err(ReadFault::Bend(diag)),
                };
            }
            (FaceBend::Plane, Reach::Chord { .. } | Reach::Extent(_)) => {
                return Ok(Descent::Rides);
            }
            (
                _,
                Reach::Bisector {
                    bend: Some(bend), ..
                },
            ) => bend,
            (FaceBend::Implicit { .. }, Reach::Chord { .. } | Reach::Extent(_)) => Vec3::zero(),
            (FaceBend::Implicit { .. }, Reach::Conic { bend, .. }) => match bend.bounds {
                geom_brep::BendBounds::Planar { .. } => bend.curvature,
                geom_brep::BendBounds::Unbounded => return uncertified,
            },
        };
        let n = face.reference.vec();
        let Some(section) = face.bend.section(self.dir.normalize(), n) else {
            return uncertified;
        };
        let arm = [face.lever, face.bend.radius()]
            .into_iter()
            .flatten()
            .fold(reach.length(), T::min);
        let probe = geom_brep::parting_side(own - n * section, face.reference, arm, band);
        let leading = match probe {
            Ok(verdict) => match Side::of(verdict) {
                Some(side) => Some(side),
                None => return Ok(Descent::Rides),
            },
            Err(escalation)
                if escalation.rung() == geom_brep::LeverRung::Reading
                    && in_one_gap(escalation.diag()) =>
            {
                None
            }
            Err(escalation) => return Err(ReadFault::Parting(escalation)),
        };
        self.witnessed(leading, arm, band)
    }

    /// **The witness's verdict on a parting off the zero band** (the
    /// design's certification of a `Leaves`): the bound's point at the
    /// parting's arm ([`Self::witness_offset`]), read on the first
    /// order's ladder under `"tangent_sector_witness"`. Where the parting
    /// was decided toward `leading`, a witness decided on that side is
    /// [`Descent::Leaves`], one within the zero band [`Descent::Rides`]
    /// (the leading term overstated a bound that stays within the band
    /// of the face at its reach), one in the gap
    /// [`Unresolved::Gap`]. Where the parting lay in the gap
    /// (`leading` `None`), the witness decides alone on the same ladder:
    /// the leading term there is too small to carry a side.
    ///
    /// # Errors
    ///
    /// [`ReadFault::Witness`]: a witness decided against the parting's
    /// side, poisoned or straddling the zero band, one whose own rounding
    /// is past the zero band's share ([`Carrier::rounding`]), or none (a
    /// bound with no point behind it, a foot that does not read).
    fn witnessed(self, leading: Option<Side>, arm: T, band: Band) -> Result<Descent, ReadFault<T>> {
        let nan = T::from_f64(f64::NAN);
        let (offset, rounding) = self.witness_offset(arm).unwrap_or((nan, nan));
        if let Err(margin) = crate::validate::resolved(WITNESS_ROUNDING, rounding, band) {
            return Err(ReadFault::Witness(Indeterminate {
                margin,
                band,
                predicate: Some(WITNESS_ROUNDING),
                terminal_sliver: false,
            }));
        }
        let decided = match crate::validate::decide_reported(
            "tangent_sector_witness",
            Margin::of(offset),
            band,
        ) {
            Ok(decided) => decided,
            Err(diag) if in_one_gap(diag) => return Ok(Descent::Unresolved(Unresolved::Gap)),
            Err(diag) => return Err(ReadFault::Witness(diag)),
        };
        let side = match decided.sign {
            Sign::Zero => return Ok(Descent::Rides),
            Sign::Positive => Side::Above,
            Sign::Negative => Side::Below,
        };
        match leading {
            Some(lead) if lead != side => {
                Err(ReadFault::Witness(geom_core::k_stats::gate_rejection(
                    "tangent_sector_witness",
                    decided,
                    band,
                    match lead {
                        Side::Above => geom_core::SizedPass::Positive,
                        Side::Below => geom_core::SizedPass::Negative,
                    },
                )))
            }
            _ => Ok(Descent::Leaves(side)),
        }
    }
}

/// The witness's own rounding, decided within the zero band's share of
/// noise ([`crate::validate::resolved`]): past it, the witness's offset
/// is not the input's, and the reading refuses.
const WITNESS_ROUNDING: &str = "tangent_sector_witness_rounding";

/// Whether the classifier's escalation is the gap: a point margin it
/// read between the zero band and the escalation band, or an enclosure
/// it found wholly inside one of those slivers
/// ([`Indeterminate::terminal_sliver`]), which no smaller reading of the
/// same term decides either. A poisoned margin, and an enclosure that
/// straddles the zero band, are not the gap.
fn in_one_gap(diag: Indeterminate) -> bool {
    match diag.margin.kind() {
        geom_core::MarginKind::Value => true,
        geom_core::MarginKind::Enclosure => diag.terminal_sliver,
        geom_core::MarginKind::Invalid => false,
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;
    use geom_brep::ConicBend;
    use geom_core::Tol;

    fn band() -> Band {
        Band::linear(Tol::witness()).unwrap()
    }

    /// A quarter of the unit circle leaving the origin along `t` (in
    /// the `xz` plane, `tilt` off `+x` toward `+z`), bending toward
    /// `bend` turned by the same `tilt` about `y` (so square to `t`).
    fn quarter_arc(bend: Vec3<f64>, tilt: f64) -> (Vec3<f64>, Reach<f64>) {
        let span = core::f64::consts::FRAC_PI_2;
        let (c, s) = (tilt.cos(), tilt.sin());
        let t = Vec3::new(c, 0.0, s);
        let bend = Vec3::new(c * bend.x - s * bend.z, bend.y, s * bend.x + c * bend.z);
        assert!(
            t.dot(bend).abs() < 1e-15,
            "the bend is square to the tangent"
        );
        let carrier = geom::Curve3::circle(Point3::origin() + bend, t.cross(bend), 1.0, -bend);
        let extent = geom_brep::edge_extent_inf(&carrier, 0.0, span, 2.0 * (span / 2.0).sin());
        let tangent = carrier.deriv(0.0);
        let bend = ConicBend::of(&carrier, tangent, carrier.deriv2(0.0), span);
        (
            tangent.normalize() * extent,
            Reach::Conic {
                extent,
                bend,
                arc: ConicArc::on(&carrier, 0.0, span),
            },
        )
    }

    /// **A tie is kept whole at first order and descended on asking**: a
    /// line in a plane rides it, and against a curved face a bound reads
    /// its bend past the face's normal section along it: a line leaves a
    /// ball outward and a hollow inward, an arc bending down leaves a
    /// plane and a ball inward and rides a ball of its own radius. A
    /// spline face's bend is not certified: unresolved.
    #[test]
    fn a_tie_descends_by_the_face_and_the_bound() {
        let b = band();
        let z = Vec3::new(0.0, 0.0, 1.0);
        let line = Reach::Chord {
            base: Point3::origin(),
            far: Point3::new(1.0, 0.0, 0.0),
        };
        let x = Vec3::new(1.0, 0.0, 0.0);
        let datum = |bend| FaceDatum::of_face(OutwardNormal::from_chart(z, true), bend, 2.0);
        for (bend, want) in [
            (FaceBend::Plane, Descent::Rides),
            (FaceBend::ball(z, 2.0), Descent::Leaves(Side::Above)),
            (FaceBend::ball(-z, 2.0), Descent::Leaves(Side::Below)),
            (
                FaceBend::Fitted,
                Descent::Unresolved(Unresolved::Uncertified),
            ),
        ] {
            let FirstOrder::Tied(tie) = read(x, line, datum(bend), b).unwrap() else {
                panic!("a line in the tangent plane ties at first order");
            };
            assert_eq!(tie.predicate, "bool_chord_side", "the chord's tie");
            assert_eq!(tie.descend(b).unwrap(), want, "{bend:?}");
        }
        let (dir, arc) = quarter_arc(Vec3::new(0.0, 0.0, -1.0), 0.0);
        for (bend, want) in [
            (FaceBend::Plane, Descent::Leaves(Side::Below)),
            (FaceBend::ball(z, 2.0), Descent::Leaves(Side::Below)),
            (FaceBend::ball(z, 1.0), Descent::Rides),
            (FaceBend::ball(z, 0.5), Descent::Leaves(Side::Above)),
        ] {
            assert_eq!(
                read_side(dir, arc, datum(bend), b).unwrap(),
                BoundSide::Tied(want),
                "a unit arc bending down against {bend:?}"
            );
        }
    }

    /// **A tie's second order reads the first order's own ladder, and a
    /// poisoned reading refuses**: a line in a unit ball's tangent plane
    /// parts from it by `l²/2` at its length, so the length sets the
    /// parting. Within the zero band it rides the ball; in the gap
    /// between the zero band and the escalation band, where its witness
    /// at `l = 1` stands too, it is unresolved; past the escalation band
    /// it leaves. A unit line on a ball so large that its witness's
    /// residual cancels in the ball's own size leaves, or refuses on its
    /// rounding: it never reads within the band. A poisoned jet (a NaN
    /// Hessian, and a cone's own apex, where it has no gradient) is no
    /// gap: it refuses.
    #[test]
    fn a_ties_parting_reads_zero_gap_and_decided_as_first_order_does() {
        let b = band();
        let z = Vec3::new(0.0, 0.0, 1.0);
        let x = Vec3::new(1.0, 0.0, 0.0);
        let o = Point3::origin();
        let line = |d: Vec3<f64>| Reach::Chord {
            base: o,
            far: o + d,
        };
        let descent = |dir: Vec3<f64>, face| {
            let FirstOrder::Tied(tie) = read(dir, line(dir), face, b).unwrap() else {
                panic!("a line in the tangent plane ties at first order");
            };
            tie.descend(b)
        };
        let ball = |radius: f64| {
            FaceDatum::of_face(
                OutwardNormal::from_chart(z, true),
                FaceBend::ball(z, radius),
                radius,
            )
        };
        let parting_at = |margin: f64| descent(x * (2.0 * margin).sqrt(), ball(1.0));
        let (zero, escalate) = (b.zero(), b.escalate());
        assert_eq!(
            parting_at(0.5 * zero).unwrap(),
            Descent::Rides,
            "within the zero band"
        );
        assert_eq!(
            parting_at(0.5 * (zero + escalate)).unwrap(),
            Descent::Unresolved(Unresolved::Gap),
            "in the gap"
        );
        assert_eq!(
            parting_at(10.0 * escalate).unwrap(),
            Descent::Leaves(Side::Above),
            "past the escalation band"
        );
        let huge = descent(x, ball(0.05 / escalate));
        assert!(
            matches!(
                huge,
                Ok(Descent::Leaves(Side::Above))
                    | Err(ReadFault::Witness(Indeterminate {
                        predicate: Some(WITNESS_ROUNDING),
                        ..
                    }))
            ),
            "a ball whose residual at the witness cancels in its own size: {huge:?}"
        );
        let nan = f64::NAN;
        let poisoned = FaceBend::Implicit {
            gradient: z,
            hessian: Mat3::from_cols(
                Vec3::new(nan, 0.0, 0.0),
                Vec3::new(0.0, nan, 0.0),
                Vec3::new(0.0, 0.0, nan),
            ),
            radius: 1.0,
            carrier: Carrier::Sphere {
                center: Point3::new(0.0, 0.0, -1.0),
                radius: 1.0,
                axis: x,
                u_ref: z,
            },
        };
        let face = FaceDatum::of_face(OutwardNormal::from_chart(z, true), poisoned, f64::MAX);
        assert!(
            matches!(descent(x, face), Err(ReadFault::Parting(_))),
            "a poisoned Hessian refuses"
        );
        let cone = geom::Surface::Cone {
            apex: o,
            axis: z,
            half_angle: 0.5,
            u_ref: x,
        };
        let apex = FaceDatum::of_face(
            OutwardNormal::from_chart(Vec3::new(0.5f64.cos(), 0.0, -0.5f64.sin()), true),
            FaceBend::at(&cone, o),
            f64::MAX,
        );
        assert!(
            matches!(
                descent(Vec3::new(0.0, 1.0, 0.0), apex),
                Err(ReadFault::Parting(_))
            ),
            "a cone's apex has no jet: it refuses"
        );
    }

    /// The curved faces a tie is read against (a plane's descent is
    /// [`every_face_kind_reads_the_closed_form_side`]'s).
    fn surfaces() -> Vec<(&'static str, geom::Surface<f64>)> {
        let o = Point3::new(0.1, -0.2, 0.3);
        let a = Vec3::new(0.2, 0.3, 1.0).normalize();
        let u = a.cross(Vec3::new(1.0, 0.0, 0.0)).normalize();
        vec![
            (
                "cylinder",
                geom::Surface::Cylinder {
                    origin: o,
                    axis: a,
                    radius: 0.7,
                    u_ref: u,
                },
            ),
            (
                "cone",
                geom::Surface::Cone {
                    apex: o,
                    axis: a,
                    half_angle: 0.5,
                    u_ref: u,
                },
            ),
            (
                "sphere",
                geom::Surface::Sphere {
                    center: o,
                    radius: 0.9,
                    axis: a,
                    u_ref: u,
                },
            ),
            (
                "torus",
                geom::Surface::Torus {
                    center: o,
                    axis: a,
                    major_radius: 2.0,
                    minor_radius: 0.6,
                    u_ref: u,
                },
            ),
        ]
    }

    /// **The descent agrees with the side a point of the bound stands
    /// on, sampled per face kind, sense and bound kind**: a line, a
    /// circular arc bending toward a random normal of its tangent, and
    /// a bisector of a sphere tangent to the face, each tangent to the
    /// face at a point of it. A `Leaves` verdict names the side a point
    /// of the bound at a small arc length stands on; a `Rides` on a
    /// curved face is a bound the face holds; a witness's refusal is a
    /// bound a point of which within its reach stands on the other side.
    /// A fixed seed: a fixture,
    /// not a search (it reads more than a thousand decided verdicts).
    #[test]
    fn the_descent_matches_a_sampled_side_per_face_kind() {
        /// The point of a bound at an arc length.
        type PointAt = Box<dyn Fn(f64) -> Point3<f64>>;
        fn rng(state: &mut u64) -> f64 {
            *state ^= *state << 13;
            *state ^= *state >> 7;
            *state ^= *state << 17;
            (*state >> 11) as f64 / (1u64 << 53) as f64
        }
        /// A point on `s` near `q`: Newton along the gradient.
        fn foot(s: &geom::Surface<f64>, mut q: Point3<f64>) -> Point3<f64> {
            for _ in 0..60 {
                let f = geom_brep::implicit_residual(s, q);
                let g = geom_brep::implicit_gradient(s, q);
                q = q - g * (f / g.norm_squared());
            }
            q
        }
        let b = band();
        let mut st = 0x9e37_79b9_7f4a_7c15u64;
        let (mut checked, mut crossed, mut wrong) = (0, 0, Vec::new());
        for (name, s) in surfaces() {
            for trial in 0..400 {
                let seed = Point3::new(
                    (rng(&mut st) * 2.0 - 1.0) * 1.5,
                    (rng(&mut st) * 2.0 - 1.0) * 1.5,
                    (rng(&mut st) * 2.0 - 1.0) * 1.5,
                );
                let seed = match name {
                    "torus" => seed + Vec3::new(2.0, 0.0, 0.0),
                    _ => seed,
                };
                let p = foot(&s, seed);
                let n_out = geom_brep::implicit_outward_normal(&s, trial % 2 == 0, p);
                let n = n_out.vec();
                if !n.x.is_finite() || geom_brep::implicit_residual(&s, p).abs() > 1e-13 {
                    continue;
                }
                let r = Vec3::new(rng(&mut st) - 0.5, rng(&mut st) - 0.5, rng(&mut st) - 0.5);
                let d = (r - n * r.dot(n)).normalize();
                let datum = FaceDatum::of_face(n_out, FaceBend::at(&s, p), f64::MAX);
                let kind = trial % 3;
                let (reach, dir, at): (Reach<f64>, Vec3<f64>, PointAt) = match kind {
                    0 => (
                        Reach::Chord {
                            base: p,
                            far: p + d * 0.5,
                        },
                        d,
                        Box::new(move |l| p + d * l),
                    ),
                    1 => {
                        let w = n.cross(d);
                        let phi = rng(&mut st) * std::f64::consts::TAU;
                        let toward = (n * phi.cos() + w * phi.sin()).normalize();
                        let k = 0.2 + 3.0 * rng(&mut st);
                        let radius = 1.0 / k;
                        let center = p + toward * radius;
                        let u_ref = (p - center).normalize();
                        let carrier =
                            geom::Curve3::circle(center, u_ref.cross(d).normalize(), radius, u_ref);
                        let span = 0.5 * k;
                        let chord = 2.0 * radius * (span / 2.0).sin();
                        let extent = geom_brep::edge_extent_inf(&carrier, 0.0, span, chord);
                        let tangent = carrier.deriv(0.0);
                        let bend = ConicBend::of(&carrier, tangent, carrier.deriv2(0.0), span);
                        (
                            Reach::Conic {
                                extent,
                                bend,
                                arc: ConicArc::on(&carrier, 0.0, span),
                            },
                            tangent.normalize() * extent,
                            Box::new(move |l| {
                                center + (u_ref * (k * l).cos() + d * (k * l).sin()) * radius
                            }),
                        )
                    }
                    _ => {
                        let rad = 0.3 + 3.0 * rng(&mut st);
                        let side = if rng(&mut st) < 0.5 { 1.0 } else { -1.0 };
                        let c = p + n * (side * rad);
                        let own = geom::Surface::Sphere {
                            center: c,
                            radius: rad,
                            axis: n.cross(d).normalize(),
                            u_ref: d,
                        };
                        let own_n = (p - c).normalize();
                        (
                            Reach::bisector(0.5, d, own_n, FaceBend::at(&own, p), p),
                            d,
                            Box::new(move |l| c + ((p + d * l) - c).normalize() * rad),
                        )
                    }
                };
                let FirstOrder::Tied(tie) = read(dir, reach, datum, b).unwrap() else {
                    panic!("{name}: a tangent bound ties");
                };
                let side = |q: Point3<f64>| {
                    let v = geom_brep::implicit_residual(&s, q)
                        * geom_brep::implicit_gradient(&s, p).dot(n).signum();
                    (v.abs() >= 1e-13).then_some(if v > 0.0 { Side::Above } else { Side::Below })
                };
                match (tie.descend(b), side(at(2e-3))) {
                    (Ok(Descent::Leaves(got)), Some(truth)) => {
                        checked += 1;
                        if got != truth {
                            wrong.push(format!("{name} kind {kind}: {got:?}, the point {truth:?}"));
                        }
                    }
                    (Ok(Descent::Rides), Some(_)) if kind != 2 => {
                        let off = geom_brep::implicit_residual(&s, at(0.4)).abs();
                        if off > 1e-8 {
                            wrong.push(format!("{name} kind {kind}: rides, {off:e} off at 0.4"));
                        }
                    }
                    (Ok(Descent::Rides | Descent::Unresolved(_)), _) => {}
                    // The witness at the arm refuses a bound that comes
                    // back across the face within its reach.
                    (Err(ReadFault::Witness(_)), Some(truth))
                        if (1..=64).any(|i| {
                            side(at(reach.length() * f64::from(i) / 64.0))
                                .is_some_and(|there| there != truth)
                        }) =>
                    {
                        crossed += 1;
                    }
                    (got, truth) => wrong.push(format!("{name} kind {kind}: {got:?}, {truth:?}")),
                }
            }
        }
        assert!(
            wrong.is_empty(),
            "{} wrong:\n{}",
            wrong.len(),
            wrong.join("\n")
        );
        assert!(checked > 1000, "{checked} decided verdicts checked");
        assert!(crossed > 0, "no bound came back across its face");
    }

    /// **Each face kind reads the side a bound leaves it to in closed
    /// form**: lines and arcs against a plane, a cylinder and a cone
    /// (rulings ride, a circle of the face's own rides, tighter and wider
    /// arcs part), a sphere, and a torus at its outer, inner (saddle) and
    /// top (parabolic) circles, in either sense; and a bisector of a
    /// ball's or a hollow's sector under its tangent plane.
    #[test]
    fn every_face_kind_reads_the_closed_form_side() {
        let q = core::f64::consts::FRAC_PI_4;
        let v = Vec3::new;
        let pt = Point3::new;
        let (x, y, z) = (v(1.0, 0.0, 0.0), v(0.0, 1.0, 0.0), v(0.0, 0.0, 1.0));
        let line = |p: Point3<f64>, t: Vec3<f64>, l: f64| {
            (
                t * l,
                Reach::Chord {
                    base: p,
                    far: p + t * l,
                },
            )
        };
        // An arc of `radius` leaving `p` along unit `t`, its centre
        // toward unit `c`, over `span` radians.
        let arc = |p: Point3<f64>, t: Vec3<f64>, c: Vec3<f64>, radius: f64, span: f64| {
            let u_ref = -c;
            let carrier = geom::Curve3::circle(p + c * radius, u_ref.cross(t), radius, u_ref);
            let chord = 2.0 * radius * (span / 2.0).sin();
            let extent = geom_brep::edge_extent_inf(&carrier, 0.0, span, chord);
            let tangent = carrier.deriv(0.0);
            let bend = ConicBend::of(&carrier, tangent, carrier.deriv2(0.0), span);
            (
                tangent.normalize() * extent,
                Reach::Conic {
                    extent,
                    bend,
                    arc: ConicArc::on(&carrier, 0.0, span),
                },
            )
        };
        let side =
            |s: &geom::Surface<f64>, p, outward: Vec3<f64>, (d, r): (Vec3<f64>, Reach<f64>)| {
                let face = FaceDatum::of_face(
                    OutwardNormal::from_chart(outward, true),
                    FaceBend::at(s, p),
                    f64::MAX,
                );
                match read_side(d, r, face, band()).unwrap() {
                    BoundSide::Tied(descent) => descent,
                    off @ BoundSide::Off(_) => panic!("a tangent bound ties: {off:?}"),
                }
            };
        let (a, b, rides) = (
            Descent::Leaves(Side::Above),
            Descent::Leaves(Side::Below),
            Descent::Rides,
        );
        let o = pt(0.0, 0.0, 0.0);
        let plane = |(d, r): (Vec3<f64>, Reach<f64>)| {
            let face = FaceDatum::of_face(
                OutwardNormal::from_chart(z, true),
                FaceBend::Plane,
                f64::MAX,
            );
            read_side(d, r, face, band()).unwrap()
        };
        assert_eq!(plane(line(o, x, 1.0)), BoundSide::Tied(rides), "plane line");
        assert_eq!(
            plane(arc(o, x, z, 1.0, q)),
            BoundSide::Tied(a),
            "plane arc up"
        );
        assert_eq!(
            plane(arc(o, x, -z, 1.0, q)),
            BoundSide::Tied(b),
            "plane arc down"
        );
        let cyl = geom::Surface::Cylinder {
            origin: o,
            axis: z,
            radius: 1.0,
            u_ref: x,
        };
        let p = pt(1.0, 0.0, 0.0);
        for (outward, across, wide, tight) in [(x, a, a, b), (-x, b, b, a)] {
            assert_eq!(
                side(&cyl, p, outward, line(p, y, 0.5)),
                across,
                "cyl ⟂ {outward:?}"
            );
            assert_eq!(side(&cyl, p, outward, line(p, z, 0.5)), rides, "cyl ruling");
            assert_eq!(
                side(&cyl, p, outward, arc(p, y, -x, 2.0, q)),
                wide,
                "cyl wide arc"
            );
            assert_eq!(
                side(&cyl, p, outward, arc(p, y, -x, 0.5, q)),
                tight,
                "cyl tight arc"
            );
            assert_eq!(
                side(&cyl, p, outward, arc(p, y, -x, 1.0, q)),
                rides,
                "cyl circle"
            );
        }
        let sph = geom::Surface::Sphere {
            center: o,
            radius: 1.0,
            axis: z,
            u_ref: x,
        };
        let p = pt(0.0, 0.0, 1.0);
        assert_eq!(side(&sph, p, z, line(p, x, 0.3)), a, "ball line");
        assert_eq!(side(&sph, p, -z, line(p, x, 0.3)), b, "hollow line");
        assert_eq!(side(&sph, p, z, arc(p, x, -z, 0.5, q)), b, "ball tight arc");
        assert_eq!(side(&sph, p, z, arc(p, x, -z, 3.0, q)), a, "ball wide arc");
        assert_eq!(
            side(&sph, p, z, arc(p, y, -z, 1.0, q)),
            rides,
            "great circle"
        );
        let cone = geom::Surface::Cone {
            apex: o,
            axis: z,
            half_angle: q,
            u_ref: x,
        };
        let p = pt(1.0, 0.0, 1.0);
        let n = v(1.0, 0.0, -1.0).normalize();
        assert_eq!(side(&cone, p, n, line(p, y, 0.3)), a, "cone line");
        assert_eq!(side(&cone, p, -n, line(p, y, 0.3)), b, "cone hollow");
        let ruling = v(1.0, 0.0, 1.0).normalize();
        assert_eq!(
            side(&cone, p, n, line(p, ruling, 0.3)),
            rides,
            "cone ruling"
        );
        assert_eq!(
            side(&cone, p, n, arc(p, y, -x, 0.5, q)),
            b,
            "cone tight arc"
        );
        assert_eq!(
            side(&cone, p, n, arc(p, y, -x, 1.0, q)),
            rides,
            "cone parallel"
        );
        assert_eq!(side(&cone, p, n, arc(p, y, -x, 2.0, q)), a, "cone wide arc");
        let tor = |minor| geom::Surface::Torus {
            center: o,
            axis: z,
            major_radius: 2.0,
            minor_radius: minor,
            u_ref: x,
        };
        let p = pt(3.0, 0.0, 0.0);
        assert_eq!(side(&tor(1.0), p, x, line(p, y, 0.3)), a, "outer parallel");
        assert_eq!(side(&tor(1.0), p, x, line(p, z, 0.3)), a, "outer meridian");
        assert_eq!(
            side(&tor(1.0), p, x, arc(p, z, -x, 1.0, q)),
            rides,
            "outer meridian circle"
        );
        assert_eq!(
            side(&tor(1.0), p, x, arc(p, z, -x, 0.5, q)),
            b,
            "tighter than the meridian"
        );
        let p = pt(1.0, 0.0, 0.0);
        assert_eq!(
            side(&tor(1.0), p, -x, line(p, y, 0.3)),
            b,
            "inner parallel (saddle)"
        );
        assert_eq!(side(&tor(1.0), p, -x, line(p, z, 0.3)), a, "inner meridian");
        assert_eq!(
            side(&tor(1.0), p, -x, arc(p, z, x, 1.0, q)),
            rides,
            "inner meridian circle"
        );
        let p = pt(2.0, 0.0, 0.5);
        assert_eq!(
            side(&tor(0.5), p, z, line(p, y, 0.3)),
            rides,
            "top circle (parabolic)"
        );
        // A bisector of a ball's sector, and of a hollow's: the same
        // surface bends down, below its tangent plane.
        let p = pt(0.0, 0.0, 1.0);
        let face = FaceDatum::of_plane(geom_brep::ReferenceNormal::of_face_outward(
            OutwardNormal::from_chart(z, true),
        ));
        for own in [z, -z] {
            let bisector = Reach::bisector(0.5, x, own, FaceBend::at(&sph, p), p);
            let FirstOrder::Tied(tie) = read(x, bisector, face, band()).unwrap() else {
                panic!("a bisector in the tangent plane ties");
            };
            assert_eq!(
                tie.descend(band()).unwrap(),
                b,
                "bisector, own normal {own:?}"
            );
        }
    }
}
