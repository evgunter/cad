//! **The section of two distinct surfaces, as one value** (C5): one
//! constructor that is the only dispatch on kind pairs for the pairs it
//! serves, a pose decided once per pair, and the components every reader
//! of the pair takes.
//!
//! # The value
//!
//! [`pair_section_with`] reads two surfaces, with the march door where
//! the caller holds one, and answers either the section
//! ([`SectionRead::Built`]) or that the kind pair is served elsewhere
//! ([`SectionRead::Elsewhere`], with [`crate::route`]'s arm): its match on
//! kinds is exhaustive with no wildcard arm, so a kind added to
//! [`SurfaceKind`] is a compile-time visit here as it is at `route`.
//! [`pair_section`] is that read with no march door, which refuses a
//! plane and a spline ([`SectionError::NoMarch`]).
//!
//! A [`PairSection`] holds:
//!
//! - **`decided`**, every `Zero` verdict that chose the arm, with its
//!   margin ([`ZeroVerdict`]): a coincidence (the sphere's centre on the
//!   cylinder's axis) is a record its consumer keeps (D10), and a
//!   tangency is classification data that refuses toward C7 wherever a
//!   carrier is wanted;
//! - **its components**, each a loop, a point, or a curve or region that
//!   reaches a surface's chart boundary ([`ComponentShape`]), whether it
//!   winds about each surface's carrier, a witness point with how far it
//!   may stand from the section, and its carrier at classification
//!   ([`ClassCarrier`]): a closed form, the general rung (marched on
//!   demand by [`PairSection::carry`]), one fitted when the section was
//!   read, or none (a tangency or a boundary region, classification
//!   only).
//!
//! # Classified, then carried
//!
//! The value is built in two states. [`PairSection`] is the
//! classification, which costs no march but on a plane and a spline,
//! whose classification is C3's whole reading ([`plane_spline`]): their
//! components are fitted and certified as they are read.
//! [`PairSection::carry`] turns it
//! into a [`CarriedSection`], every component holding a carrier a
//! reader can take a foot on: a closed form as classified, or the
//! general rung's fitted curve with its C2 certificate
//! ([`crate::ssi::SsiCertificate`]). Only `carry` builds a
//! `CarriedSection`, and it refuses where a component has no carrier, so
//! no reader takes a foot on a carrier that does not exist. The march is
//! written at `f64`, and [`SsiLane`] is the door to it: holding one is
//! the statement that the scalar may march (`SsiLane::marched`, at
//! `f64` alone).
//!
//! # The pairs it serves
//!
//! Cylinder × sphere and the three torus pairs (a ring torus against a
//! sphere, a cylinder or another ring torus, [`torus`]), and, through the
//! march door, a plane against a NURBS or approximating surface
//! ([`plane_spline`], [`PlaneSplinePose`], read by the section
//! certificate alone). Each analytic pair's pose
//! ([`CylinderSpherePose`], [`SphereTorusPose`], [`CylinderTorusPose`],
//! [`TorusTorusPose`]) is decided once, and every reader of the pair
//! reads it: the C5 table's pose question ([`crate::route_pose`]), the
//! ℝ³ march ([`crate::ssi::cylinder_sphere_ssi`],
//! [`crate::ssi::sphere_torus_ssi`], [`crate::ssi::cylinder_torus_ssi`],
//! [`crate::ssi::torus_torus_ssi`]), and through this value the
//! Boolean's section table and the section certificate. A torus pair's
//! tangency is its critical gap's ([`gap`]), the one place it is
//! decided.

use geom::{Curve3, Surface, SurfaceKind};
use geom_core::{
    Band, Decide, Decided, Indeterminate, Margin, MarginDiag, Point3, Real, Sign, Vec3,
};

use crate::ExtentBall;
use crate::intersect::{PairRoute, SectionError, SectionRadius, route};
use crate::ssi::{BranchEnd, SsiCertificate, SsiDomain, SsiError};

pub mod gap;
pub mod plane_spline;
pub mod torus;
mod torus_arms;
pub mod torus_coincidence;

pub use gap::Gap;
pub use plane_spline::PlaneSplinePose;
pub use torus::{
    AxisPose, CtStance, CylinderTorusPose, Ring, SphereTorusPose, StStance, TorusGap,
    TorusPoseRefusal, TorusQuestion, TorusTorusPose, TtStance, Turn, axis_pose,
};
pub use torus_arms::{Pinch, SquareWall, signs, signs_across};
pub use torus_coincidence::{
    TT_COINCIDENT, TT_COINCIDENT_FLOOR, TorusCoincidence, TorusData, TorusDatum, TorusDisplacement,
    tori_coincide,
};

/// **`v`'s part square to the unit `a`, and its norm**, taken as
/// `a × (v × a)`, which stands square to `a` to the rounding of its own
/// size: `v − a·(a·v)` keeps a component along `a` of `v`'s size times
/// the rounding, which dividing by a small norm (a centre or a plane
/// near the axis) turns into a direction off the plane square to `a`.
pub fn square_part<T: Real>(v: Vec3<T>, a: Vec3<T>) -> (Vec3<T>, T) {
    let perp = a.cross(v.cross(a));
    (perp, perp.norm())
}

/// The angle between two axis LINES as a chord, `|a₁ − σ·a₂|` for the
/// sign σ that brings them nearest: `2·sin(θ/2)`, which bounds how far
/// a unit offset turns between them. Comparison-free (`min`).
pub fn line_tilt<T: Real>(a1: Vec3<T>, a2: Vec3<T>) -> T {
    (a1 - a2).norm().min((a1 + a2).norm())
}

// ---------------------------------------------------------------------
// The cylinder × sphere pose
// ---------------------------------------------------------------------

/// **The questions the cylinder × sphere pose decides**, in the order
/// they are decided, each on one margin in metres.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CsDecision {
    /// `cs_coaxial`: the sphere's centre's distance `d` from the
    /// cylinder's axis.
    Coaxial,
    /// `cs_wall_reach`, coaxial: `R − r`, how far the sphere reaches past
    /// the wall.
    WallReach,
    /// `cs_near_reach`, off the axis: `R − |d − r|`, how far the sphere
    /// reaches past the wall's nearest ruling.
    NearReach,
    /// `cs_far_reach`, off the axis: `R − (d + r)`, how far it reaches
    /// past the wall's farthest ruling.
    FarReach,
}

impl CsDecision {
    /// The decision's K-funnel name.
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::Coaxial => "cs_coaxial",
            Self::WallReach => "cs_wall_reach",
            Self::NearReach => "cs_near_reach",
            Self::FarReach => "cs_far_reach",
        }
    }
}

/// **Why a cylinder × sphere pose is not decided**: an operand's radius
/// not definitely positive, or a pose question in the band.
///
/// Two doors read it, each in its own refusal vocabulary: the section
/// value as a [`SectionError`] (`From`), and the march as an
/// [`SsiError`] ([`SsiError::of_pose`]), whose refusals carry the
/// SSI door's endings (D4 ¶1) that a `SectionError` does not.
#[derive(Clone, Debug, PartialEq)]
pub enum PoseRefusal {
    /// The radius guard: `Ok` is a decided `Zero` or `Negative` radius,
    /// `Err` an undecided one.
    Radius {
        /// Whose radius.
        radius: SectionRadius,
        /// The guard's reading.
        verdict: Result<Sign, Indeterminate>,
    },
    /// A pose question in the band or poisoned.
    Undecided {
        /// Which question.
        decision: CsDecision,
        /// Its diagnostics.
        diag: Indeterminate,
    },
}

impl From<PoseRefusal> for SectionError {
    fn from(refusal: PoseRefusal) -> Self {
        match refusal {
            PoseRefusal::Radius {
                radius,
                verdict: Err(diag),
            } => Self::RadiusEscalated { radius, diag },
            PoseRefusal::Radius { radius, .. } => Self::DegenerateOperand {
                what: match radius {
                    SectionRadius::Cylinder => "the cylinder's radius is not definitely positive",
                    SectionRadius::Sphere => "the sphere's radius is not definitely positive",
                },
            },
            PoseRefusal::Undecided { diag, .. } => Self::Escalated(diag),
        }
    }
}

/// **How the sphere stands to the cylinder**, decided.
#[derive(Clone, Copy, Debug)]
pub enum CsStance<T: Real> {
    /// The sphere's centre on the axis (`cs_coaxial` decided `Zero`):
    /// two circles, the tangent circle, or nothing, by the wall reach.
    Coaxial {
        /// The coaxiality's margin: a coincidence its consumer records.
        coaxial: MarginDiag,
        /// `cs_wall_reach`.
        wall: Decided,
    },
    /// The centre definitely off the axis.
    Apart {
        /// The unit direction from the axis toward the centre.
        toward: Vec3<T>,
        /// `cs_near_reach`: `Negative` misses the wall, `Zero` touches it
        /// at one point, `Positive` crosses it.
        near: Decided,
        /// `cs_far_reach`, decided only where the sphere crosses the
        /// wall: `Negative` one loop, `Zero` the figure-eight's node,
        /// `Positive` two loops.
        far: Option<Decided>,
    },
}

/// **The cylinder × sphere pose, decided once**: the radii guarded, the
/// sphere's centre placed against the cylinder's axis, and the reach
/// past the wall read. Every number a reader of the pair takes is here,
/// so readers agree by construction; its one constructor is
/// [`Self::decide`], so a pose a reader holds is a decided one.
///
/// In the cylinder's frame put the sphere's centre at offset `d` from
/// the axis, the cylinder's radius `r` and the sphere's `R`. A wall point
/// at azimuth `θ` from the centre's side lies on the sphere at heights
/// `±h(θ)` over the centre's foot, `h² = R² − r² − d² + 2rd·cos θ`: the
/// sphere misses the wall when `R < |d − r|`, crosses it in one loop
/// when `|d − r| < R < d + r` and in two, each the graph `±h` over the
/// whole circle, when `R > d + r`. The equalities are tangencies: at the
/// nearest ruling a touch, at the farthest the loop's node.
#[derive(Clone, Debug)]
pub struct CylinderSpherePose<T: Real> {
    cylinder: Surface<T>,
    sphere: Surface<T>,
    cylinder_first: bool,
    axis: Vec3<T>,
    u_ref: Vec3<T>,
    foot: Point3<T>,
    center: Point3<T>,
    r: T,
    big_r: T,
    offset: T,
    stance: CsStance<T>,
}

impl<T: Decide> CylinderSpherePose<T> {
    /// **Decides the pose of `a` and `b`**, a cylinder and a sphere in
    /// either order (the order is kept for the readers that march in
    /// it). In order, one margin per question:
    ///
    /// 1. `cs_cylinder_radius` and `cs_sphere_radius`, `r` and `R`: each
    ///    definitely positive. The full convention, not a relation
    ///    between the two: a stored radius that is not definitely
    ///    positive is refused on every reading of the pair, as the
    ///    section table refuses it.
    /// 2. `cs_coaxial`, `d`: `Zero` is the coaxial pose, a coincidence
    ///    (D10); definite is off the axis; in band escalates.
    /// 3. Coaxial: `cs_wall_reach`, `R − r`. The exactly degenerate
    ///    formula is the one read under a `Zero` offset.
    /// 4. Off the axis: `cs_near_reach`, `R − |d − r|`, then where it is
    ///    positive `cs_far_reach`, `R − (d + r)`.
    ///
    /// # Errors
    ///
    /// [`PoseRefusal`]; a pair that is not a cylinder and a sphere is
    /// `None`.
    pub fn decide(a: &Surface<T>, b: &Surface<T>, band: Band) -> Option<Result<Self, PoseRefusal>> {
        let (cylinder, sphere, cylinder_first) = match (a, b) {
            (Surface::Cylinder { .. }, Surface::Sphere { .. }) => (a, b, true),
            (Surface::Sphere { .. }, Surface::Cylinder { .. }) => (b, a, false),
            _ => return None,
        };
        let (
            &Surface::Cylinder {
                origin,
                axis,
                radius: r,
                u_ref,
            },
            &Surface::Sphere {
                center,
                radius: big_r,
                ..
            },
        ) = (cylinder, sphere)
        else {
            return None;
        };
        Some(
            Self::decide_stance(origin, axis, center, r, big_r, band).map(
                |(foot, offset, stance)| Self {
                    cylinder: cylinder.clone(),
                    sphere: sphere.clone(),
                    cylinder_first,
                    axis,
                    u_ref,
                    foot,
                    center,
                    r,
                    big_r,
                    offset,
                    stance,
                },
            ),
        )
    }

    #[allow(clippy::type_complexity)] // (foot, offset, stance) — one pose
    fn decide_stance(
        origin: Point3<T>,
        axis: Vec3<T>,
        center: Point3<T>,
        r: T,
        big_r: T,
        band: Band,
    ) -> Result<(Point3<T>, T, CsStance<T>), PoseRefusal> {
        let read = |name: &'static str, value: T| {
            geom_core::k_stats::decide_reported(name, Margin::of(value), band)
        };
        for (name, value, radius) in [
            ("cs_cylinder_radius", r, SectionRadius::Cylinder),
            ("cs_sphere_radius", big_r, SectionRadius::Sphere),
        ] {
            match read(name, value) {
                Ok(Decided {
                    sign: Sign::Positive,
                    ..
                }) => {}
                verdict => {
                    return Err(PoseRefusal::Radius {
                        radius,
                        verdict: verdict.map(|d| d.sign),
                    });
                }
            }
        }
        let pose = |decision: CsDecision, value: T| {
            read(decision.name(), value).map_err(|diag| PoseRefusal::Undecided { decision, diag })
        };
        // `axis` is unit by the surface's own invariant.
        let (off, d) = square_part(center - origin, axis);
        let foot = center - off;
        let coaxial = pose(CsDecision::Coaxial, d)?;
        let stance = if coaxial.sign == Sign::Zero {
            CsStance::Coaxial {
                coaxial: coaxial.margin,
                wall: pose(CsDecision::WallReach, big_r - r)?,
            }
        } else {
            let near = pose(CsDecision::NearReach, big_r - (d - r).abs())?;
            let far = match near.sign {
                Sign::Positive => Some(pose(CsDecision::FarReach, big_r - (d + r))?),
                Sign::Zero | Sign::Negative => None,
            };
            CsStance::Apart {
                toward: off * (T::one() / d),
                near,
                far,
            }
        };
        Ok((foot, d, stance))
    }

    /// The cylinder, as given.
    pub fn cylinder(&self) -> &Surface<T> {
        &self.cylinder
    }

    /// The sphere, as given.
    pub fn sphere(&self) -> &Surface<T> {
        &self.sphere
    }

    /// The two surfaces in the order the pose was asked of.
    pub fn operands(&self) -> (&Surface<T>, &Surface<T>) {
        if self.cylinder_first {
            (&self.cylinder, &self.sphere)
        } else {
            (&self.sphere, &self.cylinder)
        }
    }

    /// The cylinder's axis, unit by the surface's invariant.
    pub fn axis(&self) -> Vec3<T> {
        self.axis
    }

    /// The sphere centre's foot on the axis.
    pub fn foot(&self) -> Point3<T> {
        self.foot
    }

    /// The sphere's centre.
    pub fn center(&self) -> Point3<T> {
        self.center
    }

    /// The sphere's own ball, which holds the section.
    pub fn ball(&self) -> ExtentBall<T> {
        ExtentBall::new(self.center, self.big_r)
    }

    /// The cylinder's radius `r` and the sphere's `R`, both decided
    /// positive.
    pub fn radii(&self) -> (T, T) {
        (self.r, self.big_r)
    }

    /// The centre's distance `d` from the axis.
    pub fn offset(&self) -> T {
        self.offset
    }

    /// How the sphere stands.
    pub fn stance(&self) -> CsStance<T> {
        self.stance
    }

    /// **The tangency the pose decided**, if any: the wall reach of a
    /// coaxial pose, or the near or far reach off the axis, decided
    /// `Zero`. Classification data at every reader, never a carrier.
    pub fn tangency(&self) -> Option<ZeroVerdict> {
        let tangent = |decision: CsDecision, d: Decided| {
            (d.sign == Sign::Zero).then_some(ZeroVerdict {
                decision: SectionDecision::Tangency(decision.name()),
                margin: d.margin,
            })
        };
        match self.stance {
            CsStance::Coaxial { wall, .. } => tangent(CsDecision::WallReach, wall),
            CsStance::Apart { near, far, .. } => tangent(CsDecision::NearReach, near)
                .or_else(|| far.and_then(|f| tangent(CsDecision::FarReach, f))),
        }
    }

    /// The point where an off-axis sphere stands nearest the wall: on the
    /// ruling nearest the centre, at the centre's height.
    fn nearest_ruling(&self, toward: Vec3<T>) -> Point3<T> {
        self.foot + toward * self.r
    }
}

// ---------------------------------------------------------------------
// The section value
// ---------------------------------------------------------------------

/// What a `Zero` verdict that chose the arm means.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ZeroKind {
    /// The surfaces stand in a coincident pose (D10): the consumer keeps
    /// it as a record.
    Coincidence,
    /// The surfaces are tangent along a component or at a point (C7):
    /// classification data, no carrier.
    Tangency,
}

/// **A coincident pose a section decided** (D10), by its site: the
/// record its consumer keeps.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Coincident {
    /// `cs_coaxial`: the sphere's centre on the cylinder's axis.
    CylinderSphere,
    /// `st_coaxial`: the sphere's centre on the torus's axis.
    SphereTorus,
    /// `section_axes_offset`: the cylinder's axis on the torus's.
    CylinderTorus,
    /// `section_axes_offset`: the second torus's axis on the first's.
    TorusTorus,
}

impl Coincident {
    /// The decision's K-funnel name.
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::CylinderSphere => CsDecision::Coaxial.name(),
            Self::SphereTorus => "st_coaxial",
            Self::CylinderTorus | Self::TorusTorus => "section_axes_offset",
        }
    }
}

/// **Which decision a `Zero` verdict was decided on**, by what it means.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SectionDecision {
    /// A coincident pose.
    Coincidence(Coincident),
    /// A tangency, by the row that decided it: a cylinder × sphere
    /// reach, a torus pair's critical gap, or a coaxial pose's
    /// closed-form tube row.
    Tangency(&'static str),
}

impl SectionDecision {
    /// The decision's K-funnel name.
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::Coincidence(c) => c.name(),
            Self::Tangency(row) => row,
        }
    }

    /// What the decision means.
    #[must_use]
    pub const fn kind(self) -> ZeroKind {
        match self {
            Self::Coincidence(_) => ZeroKind::Coincidence,
            Self::Tangency(_) => ZeroKind::Tangency,
        }
    }
}

/// One `Zero` verdict that chose the section's arm.
#[derive(Clone, Copy, Debug)]
pub struct ZeroVerdict {
    /// The decision, and what it means.
    pub decision: SectionDecision,
    /// Its margin.
    pub margin: MarginDiag,
}

/// Whether a component is a curve or one point.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ComponentShape {
    /// A closed curve.
    Loop,
    /// One point, where the surfaces touch.
    Point,
    /// A curve or a band-thin region that reaches the boundary of a
    /// surface's parameter rectangle: an open branch of a spline
    /// section, or a `Side` or `Corner` region of one.
    Open {
        /// The rectangle sides it reaches on each surface, in the order
        /// the section was asked of.
        reaches: [ChartReach; 2],
    },
}

/// **The sides of a surface's parameter rectangle a component reaches**,
/// by the parameter each holds fixed: whether a side is the surface's
/// boundary or a seam of a closed channel is the reader's question.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ChartReach {
    /// It reaches a side on which `u` is held fixed.
    pub u_side: bool,
    /// It reaches a side on which `v` is held fixed.
    pub v_side: bool,
}

impl ChartReach {
    /// The side `side` alone.
    #[must_use]
    pub fn side(side: crate::ssi::ChartSide) -> Self {
        let u_side = side.fixed == crate::ssi::ChartAxis::U;
        Self {
            u_side,
            v_side: !u_side,
        }
    }

    /// The sides either reaches.
    #[must_use]
    pub fn or(self, other: Self) -> Self {
        Self {
            u_side: self.u_side || other.u_side,
            v_side: self.v_side || other.v_side,
        }
    }

    /// It reaches some side.
    #[must_use]
    pub fn any(self) -> bool {
        self.u_side || self.v_side
    }
}

/// A component's carrier as classified.
#[derive(Clone, Debug)]
pub enum ClassCarrier<T: Real> {
    /// A closed form.
    Closed(Curve3<T>),
    /// The general rung's: marched and fitted by [`PairSection::carry`].
    General,
    /// Marched, fitted and certified when the section was read (a
    /// spline section's whole reading).
    Fitted(CarriedComponent<T>),
    /// None mints: the component is classification data, for the reason
    /// given.
    Unminted(&'static str),
}

/// One connected component of a section, as classified.
#[derive(Clone, Debug)]
pub struct SectionComponent<T: Real> {
    /// A loop or a point.
    pub shape: ComponentShape,
    /// Whether it winds about each surface's carrier (it is essential
    /// there), in the order the section was asked of.
    pub winds: [bool; 2],
    /// One point of it, where the arm names one: in closed form, or a
    /// spline section's point bracketed on the section.
    pub witness: Option<Point3<T>>,
    /// How far, in metres past rounding, the witness may stand from the
    /// section: zero for a closed form's.
    pub residual: f64,
    /// Its carrier.
    pub carrier: ClassCarrier<T>,
}

impl<T: Real> SectionComponent<T> {
    /// A component a closed form reads: its witness, where it has one,
    /// within rounding of the section.
    pub(crate) fn closed_form(
        shape: ComponentShape,
        winds: [bool; 2],
        witness: Option<Point3<T>>,
        carrier: ClassCarrier<T>,
    ) -> Self {
        Self {
            shape,
            winds,
            witness,
            residual: 0.0,
            carrier,
        }
    }
}

/// **The section of two distinct surfaces, classified** (module docs).
#[derive(Clone, Debug)]
pub struct PairSection<T: Real> {
    pose: PairPose<T>,
    decided: Vec<ZeroVerdict>,
    components: Vec<SectionComponent<T>>,
    sides: Sides<T>,
    lever: T,
}

/// **The pose a section was built over**, one per pair it serves.
#[derive(Clone, Debug)]
pub enum PairPose<T: Real> {
    /// Cylinder × sphere.
    CylinderSphere(CylinderSpherePose<T>),
    /// Sphere × torus.
    SphereTorus(SphereTorusPose<T>),
    /// Cylinder × torus.
    CylinderTorus(CylinderTorusPose<T>),
    /// Torus × torus.
    TorusTorus(TorusTorusPose<T>),
    /// A plane and a spline wall.
    PlaneSpline(Box<PlaneSplinePose<T>>),
}

impl<T: Real> PairPose<T> {
    /// The cylinder × sphere pose, where it is one.
    pub fn cylinder_sphere(&self) -> Option<&CylinderSpherePose<T>> {
        match self {
            Self::CylinderSphere(p) => Some(p),
            Self::SphereTorus(_)
            | Self::CylinderTorus(_)
            | Self::TorusTorus(_)
            | Self::PlaneSpline(_) => None,
        }
    }
}

impl<T: Decide> PairPose<T> {
    /// The two surfaces in the order the pose was asked of.
    pub fn operands(&self) -> (&Surface<T>, &Surface<T>) {
        match self {
            Self::CylinderSphere(p) => p.operands(),
            Self::SphereTorus(p) => p.operands(),
            Self::CylinderTorus(p) => p.operands(),
            Self::TorusTorus(p) => p.operands(),
            Self::PlaneSpline(p) => p.operands(),
        }
    }

    /// **A ball holding the whole section**, the march's domain
    /// ([`SsiLane`]): every point of the section lies on both surfaces,
    /// so inside the ball of either that is bounded. A sphere is its own
    /// ball, and is taken where the pair has one; a ring torus lies in
    /// `|x − c| ≤ R + r` (each of its points is `r` from a core point,
    /// which is `R` from the centre), the smaller of two tori's as the
    /// pose decided it ([`TorusTorusPose::ball`]). A cylinder is
    /// unbounded along its axis, so it is never the ball, nor is a plane;
    /// a spline wall lies in its net's ([`PlaneSplinePose::ball`]).
    pub fn ball(&self) -> ExtentBall<T> {
        match self {
            Self::CylinderSphere(p) => p.ball(),
            Self::SphereTorus(p) => p.ball(),
            Self::CylinderTorus(p) => p.ball(),
            Self::TorusTorus(p) => p.ball(),
            Self::PlaneSpline(p) => p.ball(),
        }
    }
}

impl PairPose<f64> {
    /// **The march's domain**: the cube about [`Self::ball`], padded by
    /// a hundredth of its radius so the slab's faces stand off the ball,
    /// which the bounding surface reaches (a sphere everywhere, a torus
    /// round its outer equator); the feature extent the ball's diameter,
    /// the standard accounting floor. A plane and a spline wall are read
    /// over the plane's window ([`PlaneSplinePose::domain`]).
    #[must_use]
    pub fn domain(&self) -> SsiDomain {
        if let Self::PlaneSpline(p) = self {
            return p.domain();
        }
        let ball = self.ball();
        SsiDomain {
            center: ball.center(),
            half_extent: ball.radius() * 1.01,
            extent: ball.radius() * 2.0,
            floor_scale: 1.0,
        }
    }
}

/// **How a site's component is told apart** among a section's two
/// ([`PairSection::component_of`]); the first component is the one on
/// the positive side.
#[derive(Clone, Copy, Debug)]
pub(crate) enum Sides<T: Real> {
    /// One component or none: nothing to tell apart.
    One,
    /// The side of the plane through `origin` normal to `normal`.
    Plane {
        origin: Point3<T>,
        normal: Vec3<T>,
        row: &'static str,
    },
    /// The side, in the meridian half-plane of `(center, axis)` through
    /// the site, of the line through `through` along `along`, each
    /// `(distance from the axis, height)`.
    Meridian {
        center: Point3<T>,
        axis: Vec3<T>,
        through: (T, T),
        along: (T, T),
        row: &'static str,
    },
    /// No reading of the classification tells them apart.
    Untold(&'static str),
}

/// What [`pair_section`] answers.
#[derive(Clone, Debug)]
pub enum SectionRead<T: Real> {
    /// The pair's section.
    Built(PairSection<T>),
    /// The kind pair's section is not built by this value: its readers
    /// read it where they read it before ([`crate::route`]'s arm, cited).
    Elsewhere(PairRoute),
}

/// **The section of `a` and `b`** (module docs): the one dispatch on
/// kind pairs this value has, exhaustive with no wildcard arm.
///
/// # Errors
///
/// The arm's [`SectionError`]: an operand's radius not definitely
/// positive ([`SectionError::DegenerateOperand`], or
/// [`SectionError::RadiusEscalated`] in band), and a pose question in
/// the band ([`SectionError::Escalated`]); a torus outside its ring
/// convention ([`SectionError::DegenerateTorus`]), two tori decided one
/// ([`SectionError::CoincidentSurfaces`]), a closed-form count refused
/// ([`SectionError::Pinched`], [`SectionError::Unordered`]), and a
/// general cylinder × torus or torus × torus pose, whose components the
/// classification does not list ([`SectionError::RoutesToGeneralRung`]);
/// a plane and a spline, whose section is a march
/// ([`SectionError::NoMarch`]; [`pair_section_with`] reads it).
pub fn pair_section<T: Decide>(
    a: &Surface<T>,
    b: &Surface<T>,
    band: Band,
) -> Result<SectionRead<T>, SectionError> {
    pair_section_with(a, b, None, band)
}

/// **[`pair_section`] with the march door `lane`**, which a plane and a
/// spline wall (an approximating surface read as its fit) are read
/// through: their section is C3's whole reading, a march
/// ([`plane_spline`]).
///
/// # Errors
///
/// [`pair_section`]'s; for a plane and a spline with no `lane`,
/// [`SectionError::NoMarch`], and with one, the whole reading's refusal,
/// mapped by name ([`plane_spline`]).
pub fn pair_section_with<T: Decide>(
    a: &Surface<T>,
    b: &Surface<T>,
    lane: Option<SsiLane<T>>,
    band: Band,
) -> Result<SectionRead<T>, SectionError> {
    use SurfaceKind::{Approx, Cone, Cylinder, Nurbs, Plane, Sphere, Torus};
    let (ka, kb) = (a.kind(), b.kind());
    match (ka, kb) {
        (Cylinder, Sphere) | (Sphere, Cylinder) => match CylinderSpherePose::decide(a, b, band) {
            Some(pose) => Ok(SectionRead::Built(cylinder_sphere(pose?))),
            None => Err(SectionError::WrongLane {
                expected: "a cylinder and a sphere",
            }),
        },
        (Sphere, Torus) | (Torus, Sphere) => match SphereTorusPose::decide(a, b, band) {
            Some(pose) => torus::sphere_torus(pose?, band).map(SectionRead::Built),
            None => Err(SectionError::WrongLane {
                expected: "a sphere and a torus",
            }),
        },
        (Cylinder, Torus) | (Torus, Cylinder) => match CylinderTorusPose::decide(a, b, band) {
            Some(pose) => torus::cylinder_torus(pose?, band).map(SectionRead::Built),
            None => Err(SectionError::WrongLane {
                expected: "a cylinder and a torus",
            }),
        },
        (Torus, Torus) => match TorusTorusPose::decide(a, b, band) {
            Some(pose) => torus::torus_torus(pose?, band).map(SectionRead::Built),
            None => Err(SectionError::WrongLane {
                expected: "two tori",
            }),
        },
        (Plane, Nurbs | Approx) | (Nurbs | Approx, Plane) => match lane {
            Some(lane) => (lane.spline)(a, b, band).map(SectionRead::Built),
            None => Err(SectionError::NoMarch { scalar: T::NAME }),
        },
        (Plane, Plane | Cylinder | Cone | Sphere | Torus)
        | (Cylinder, Plane | Cylinder | Cone | Nurbs | Approx)
        | (Cone, Plane | Cylinder | Cone | Sphere | Torus | Nurbs | Approx)
        | (Sphere, Plane | Cone | Sphere | Nurbs | Approx)
        | (Torus, Plane | Cone | Nurbs | Approx)
        | (Nurbs, Cylinder | Cone | Sphere | Torus | Nurbs | Approx)
        | (Approx, Cylinder | Cone | Sphere | Torus | Nurbs | Approx) => {
            Ok(SectionRead::Elsewhere(route(ka, kb)))
        }
    }
}

/// **Whether the value builds, by classification, the section of some
/// kind pair `kind` is a side of** — the filter a reader scanning many
/// pairs applies before it asks [`pair_section`]. Exhaustive, as
/// `pair_section`'s match is, and a second spelling of it
/// (`work/sectrung/the-c5-kind-dispatch-is-spelled-twice.md`). A plane and a
/// spline are not among them: their section is a whole reading, read
/// per pair through [`pair_section_with`] by the reader that asks it.
#[must_use]
pub fn serves(kind: SurfaceKind) -> bool {
    use SurfaceKind::{Approx, Cone, Cylinder, Nurbs, Plane, Sphere, Torus};
    match kind {
        Cylinder | Sphere | Torus => true,
        Plane | Cone | Nurbs | Approx => false,
    }
}

/// The C7 note a tangent component carries.
const TANGENT_COMPONENT: &str =
    "the surfaces are tangent along it or at it, a C7 construction the section does not mint";

/// The cylinder × sphere arm over its decided pose.
fn cylinder_sphere<T: Decide>(pose: CylinderSpherePose<T>) -> PairSection<T> {
    // `winds` is read in the order the pose was asked of.
    let order = |on_cylinder: bool, on_sphere: bool| {
        if pose.cylinder_first {
            [on_cylinder, on_sphere]
        } else {
            [on_sphere, on_cylinder]
        }
    };
    let mut decided = Vec::new();
    let circle = |station: T| Curve3::Circle {
        center: pose.center + pose.axis * station,
        axis: pose.axis,
        u_ref: geom::square_frame(pose.axis, pose.u_ref).u_ref,
        radius: pose.r,
    };
    let ring = |carrier| {
        SectionComponent::closed_form(ComponentShape::Loop, order(true, false), None, carrier)
    };
    let components = match pose.stance {
        CsStance::Coaxial { coaxial, wall } => {
            decided.push(ZeroVerdict {
                decision: SectionDecision::Coincidence(Coincident::CylinderSphere),
                margin: coaxial,
            });
            match wall.sign {
                // Both factors are definitely positive: `R − r` by the
                // reach, `R + r` as the sum of two guarded radii. The
                // factored form keeps an interval evaluation tight.
                Sign::Positive => {
                    let station = ((pose.big_r - pose.r) * (pose.big_r + pose.r)).sqrt();
                    vec![
                        ring(ClassCarrier::Closed(circle(station))),
                        ring(ClassCarrier::Closed(circle(T::zero() - station))),
                    ]
                }
                Sign::Zero => vec![ring(ClassCarrier::Unminted(TANGENT_COMPONENT))],
                Sign::Negative => Vec::new(),
            }
        }
        CsStance::Apart { toward, near, far } => match (near.sign, far.map(|f| f.sign)) {
            (Sign::Negative, _) => Vec::new(),
            (Sign::Zero, _) => vec![SectionComponent::closed_form(
                ComponentShape::Point,
                [false, false],
                Some(pose.nearest_ruling(toward)),
                ClassCarrier::Unminted(TANGENT_COMPONENT),
            )],
            // One loop, witnessed on the ruling nearest the centre,
            // `√((R − |d − r|)(R + |d − r|))` above the foot.
            (Sign::Positive, Some(Sign::Negative | Sign::Zero) | None) => {
                let q = (pose.offset - pose.r).abs();
                let t = ((pose.big_r - q) * (pose.big_r + q)).sqrt();
                let carrier = match far.map(|f| f.sign) {
                    Some(Sign::Negative) => ClassCarrier::General,
                    _ => ClassCarrier::Unminted(TANGENT_COMPONENT),
                };
                vec![SectionComponent::closed_form(
                    ComponentShape::Loop,
                    [false, false],
                    Some(pose.nearest_ruling(toward) + pose.axis * t),
                    carrier,
                )]
            }
            (Sign::Positive, Some(Sign::Positive)) => {
                vec![ring(ClassCarrier::General), ring(ClassCarrier::General)]
            }
        },
    };
    decided.extend(pose.tangency());
    PairSection {
        sides: Sides::Plane {
            origin: pose.foot,
            normal: pose.axis,
            row: "cs_component_side",
        },
        lever: pose.r,
        pose: PairPose::CylinderSphere(pose),
        decided,
        components,
    }
}

/// **Why a site has no component of its section**, or more than one.
#[derive(Clone, Debug, PartialEq)]
pub enum SiteRefusal {
    /// The reading in the band or poisoned.
    Escalated(Indeterminate),
    /// The site lies on no component, or on two: the site was not read
    /// from this section.
    Desync(&'static str),
    /// The classification does not tell the section's components apart
    /// at a site, for the reason given.
    Untold(&'static str),
}

impl<T: Decide> PairSection<T> {
    /// The pose the arm decided.
    pub fn pose(&self) -> &PairPose<T> {
        &self.pose
    }

    /// Every `Zero` verdict that chose the arm.
    pub fn decided(&self) -> &[ZeroVerdict] {
        &self.decided
    }

    /// The components.
    pub fn components(&self) -> &[SectionComponent<T>] {
        &self.components
    }

    /// The coincidence among [`Self::decided`], where the arm stands on
    /// one, with its margin: the record its consumer keeps.
    pub fn coincidence(&self) -> Option<(Coincident, MarginDiag)> {
        self.decided.iter().find_map(|z| match z.decision {
            SectionDecision::Coincidence(c) => Some((c, z.margin)),
            SectionDecision::Tangency(_) => None,
        })
    }

    /// The tangency among [`Self::decided`], where the arm stands on one.
    pub fn tangency(&self) -> Option<ZeroVerdict> {
        self.decided
            .iter()
            .copied()
            .find(|z| z.decision.kind() == ZeroKind::Tangency)
    }

    /// **The component the site `p` lies on**, by the classification
    /// alone, so no march is owed for it: a section of one component
    /// answers it; two are told apart by the side of a plane or of a
    /// meridian line the arm names (for cylinder × sphere the plane
    /// through the sphere centre's foot square to the cylinder's axis,
    /// `cs_component_side`), the first component on its positive side.
    /// Both components keep a definite side, so a `Zero` there is a site
    /// on neither, a desync. More than two, or two the arm cannot tell
    /// apart, are untold. Whether the site lies within the band of the
    /// component is the carried section's question
    /// ([`CarriedSection::foot`]).
    ///
    /// # Errors
    ///
    /// [`SiteRefusal`]; a section with no component is a desync.
    pub fn component_of(&self, p: Point3<T>, band: Band) -> Result<usize, SiteRefusal> {
        let side =
            |row: &'static str, m: T| match geom_core::k_stats::decide(row, Margin::of(m), band) {
                Ok(Sign::Positive) => Ok(0),
                Ok(Sign::Negative) => Ok(1),
                Ok(Sign::Zero) => Err(SiteRefusal::Desync(
                    "a site lies between the two components of its section",
                )),
                Err(diag) => Err(SiteRefusal::Escalated(diag)),
            };
        match (self.components.len(), self.sides) {
            (0, _) => Err(SiteRefusal::Desync("a site on a section with no component")),
            (1, _) => Ok(0),
            (
                2,
                Sides::Plane {
                    origin,
                    normal,
                    row,
                },
            ) => side(row, (p - origin).dot(normal)),
            (
                2,
                Sides::Meridian {
                    center,
                    axis,
                    through: (x0, z0),
                    along: (dx, dz),
                    row,
                },
            ) => {
                let w = p - center;
                let (_, rho) = square_part(w, axis);
                let (x, z) = (rho - x0, w.dot(axis) - z0);
                side(row, (dx * z - dz * x) / (dx.powi(2) + dz.powi(2)).sqrt())
            }
            (2, Sides::Untold(why)) => Err(SiteRefusal::Untold(why)),
            (2, Sides::One) => Err(SiteRefusal::Untold(
                "the section's two components have no reading that tells them apart",
            )),
            _ => Err(SiteRefusal::Untold(
                "a section of more than two components, which the classification does not \
                 tell apart at a site",
            )),
        }
    }

    /// **The section carried** (module docs): every component's carrier,
    /// a closed form as classified or the general rung's, marched through
    /// `lane` and fitted with its certificate. The march finds each
    /// component as one closed branch, assigned to the component its
    /// witness lies on ([`Self::component_of`]).
    ///
    /// # Errors
    ///
    /// [`CarryError`].
    pub fn carry(
        &self,
        lane: Option<SsiLane<T>>,
        band: Band,
    ) -> Result<CarriedSection<T>, CarryError> {
        for c in &self.components {
            match c.carrier {
                ClassCarrier::Unminted(why) => return Err(CarryError::Unminted(why)),
                ClassCarrier::Closed(_) | ClassCarrier::General | ClassCarrier::Fitted(_) => {}
            }
        }
        let mut fitted = if self
            .components
            .iter()
            .any(|c| matches!(c.carrier, ClassCarrier::General))
        {
            let lane = lane.ok_or(CarryError::NoLane { scalar: T::NAME })?;
            let branches = (lane.march)(&self.pose, band).map_err(CarryError::March)?;
            let mut slots: Vec<Option<FittedCarrier<T>>> = vec![None; self.components.len()];
            for branch in branches {
                if !branch.closed {
                    return Err(CarryError::Desync(
                        "the march left the ball's box, which holds the whole section",
                    ));
                }
                let k = self
                    .component_of(branch.witness, band)
                    .map_err(CarryError::Site)?;
                match slots.get_mut(k) {
                    Some(slot @ None) => *slot = Some(branch),
                    _ => {
                        return Err(CarryError::Desync(
                            "the march found two branches on one component",
                        ));
                    }
                }
            }
            slots
        } else {
            vec![None; self.components.len()]
        };
        let components = self
            .components
            .iter()
            .zip(fitted.iter_mut())
            .map(|(c, slot)| match (&c.carrier, slot.take()) {
                (ClassCarrier::Closed(curve), _) => Ok(CarriedComponent {
                    carrier: curve.clone(),
                    span: (T::zero(), T::from_f64(core::f64::consts::TAU)),
                    closed: true,
                    certificate: None,
                }),
                (ClassCarrier::General, Some(f)) => Ok(CarriedComponent {
                    carrier: f.carrier,
                    span: f.span,
                    closed: f.closed,
                    certificate: Some(f.certificate),
                }),
                (ClassCarrier::Fitted(carried), _) => Ok(carried.clone()),

                (ClassCarrier::General, None) => Err(CarryError::Desync(
                    "the march found no branch on a component the classification names",
                )),
                (ClassCarrier::Unminted(why), _) => Err(CarryError::Unminted(why)),
            })
            .collect::<Result<_, _>>()?;
        Ok(CarriedSection {
            components,
            lever: self.lever,
        })
    }

    /// **A closed fitted component, opened at the parameter `at`** of
    /// its carrier, so that no chord cut from it straddles the carrier's
    /// parameter seam: the carrier is split at `at` (knot insertion to
    /// full multiplicity), its two pieces are joined the other way round
    /// (the piece after `at` first, the piece before it shifted by the
    /// carrier's period), and the joined carrier is certified again
    /// against the pose's operands through `lane`, since the shifted
    /// knots are rounded. The seated carrier runs from `at`'s point
    /// round to it. A component that is open, or whose carrier is a
    /// closed form, is returned as it is: a closed form's parameter is an
    /// angle, which a chord reads on any branch.
    ///
    /// # Errors
    ///
    /// [`CarryError::NoLane`] off the march's scalar,
    /// [`CarryError::Seat`] where the knot edit refuses, and the
    /// certificate's refusal as [`CarryError::March`].
    pub fn seat(
        &self,
        lane: Option<SsiLane<T>>,
        component: &CarriedComponent<T>,
        at: T,
        band: Band,
    ) -> Result<CarriedComponent<T>, CarryError> {
        if !component.closed || component.certificate.is_none() {
            return Ok(component.clone());
        }
        let lane = lane.ok_or(CarryError::NoLane { scalar: T::NAME })?;
        (lane.seat)(&self.pose, component, at, band)
    }
}

// ---------------------------------------------------------------------
// The carried section
// ---------------------------------------------------------------------

/// One branch the general rung fitted ([`SsiLane`]).
#[derive(Clone, Debug)]
pub struct FittedCarrier<T: Real> {
    /// The fitted carrier.
    pub carrier: Curve3<T>,
    /// Its parameter span.
    pub span: (T, T),
    /// Whether the branch closed on itself.
    pub closed: bool,
    /// The three-limb certificate.
    pub certificate: SsiCertificate<T>,
    /// A point of it (`carrier(mid)`).
    pub witness: Point3<T>,
}

/// **The general rung's door** (module docs): the march of a decided
/// pose over its ball, and a plane and a spline's whole reading, written
/// at `f64`. Holding one is the statement that the scalar may march;
/// [`SsiLane::marched`], at `f64`, is its one constructor.
#[derive(Clone, Copy, Debug)]
pub struct SsiLane<T: Real> {
    /// One march of a decided pose.
    march: fn(&PairPose<T>, Band) -> Marched<T>,
    /// One closed fitted component seated at a parameter, re-certified
    /// against the pose's operands ([`PairSection::seat`]).
    seat: fn(&PairPose<T>, &CarriedComponent<T>, T, Band) -> Seated<T>,
    /// The section of a plane and a spline wall ([`plane_spline`]).
    spline: SplineSection<T>,
}

/// What reads a plane and a spline's section ([`plane_spline`]).
type SplineSection<T> = fn(&Surface<T>, &Surface<T>, Band) -> Result<PairSection<T>, SectionError>;

/// What one seating answers: the component opened at the parameter
/// ([`PairSection::seat`]).
pub type Seated<T> = Result<CarriedComponent<T>, CarryError>;

/// What one march of a pose answers: its fitted branches.
type Marched<T> = Result<Vec<FittedCarrier<T>>, SsiError>;

impl SsiLane<f64> {
    /// The march.
    #[must_use]
    pub fn marched() -> Self {
        Self {
            march: march_over_the_ball,
            seat: seat_at,
            spline: plane_spline::section,
        }
    }
}

/// **The ℝ³ march of `pose` over its ball's box** ([`PairPose::domain`]),
/// which holds every component of the section. No branch reaches the
/// box's faces, so every branch the march finds closes.
fn march_over_the_ball(
    pose: &PairPose<f64>,
    band: Band,
) -> Result<Vec<FittedCarrier<f64>>, SsiError> {
    let domain = pose.domain();
    let out = match pose {
        PairPose::CylinderSphere(p) => crate::ssi::march_cylinder_sphere(p, domain, band)?,
        PairPose::SphereTorus(p) => crate::ssi::march_sphere_torus(p, domain, band)?,
        PairPose::CylinderTorus(p) => crate::ssi::march_cylinder_torus(p, domain, band)?,
        PairPose::TorusTorus(p) => crate::ssi::march_torus_torus(p, domain, band)?,
        // Its components are carried when the section is read; none is
        // left for the ball's march.
        PairPose::PlaneSpline(_) => {
            return Err(SsiError::WrongLane {
                expected: "a pose whose loops the ball's march carries",
            });
        }
    };
    Ok(out
        .branches
        .into_iter()
        .map(|b| FittedCarrier {
            carrier: b.carrier,
            span: b.params,
            closed: matches!(b.end, BranchEnd::Closed),
            certificate: b.certificate,
            witness: b.witness,
        })
        .collect())
}

/// **[`PairSection::seat`] at `f64`.** `at` at either end of the
/// carrier's domain is a seat already: the carrier starts there.
fn seat_at(
    pose: &PairPose<f64>,
    component: &CarriedComponent<f64>,
    at: f64,
    band: Band,
) -> Result<CarriedComponent<f64>, CarryError> {
    if let PairPose::PlaneSpline(_) = pose {
        return Err(CarryError::Unminted(
            "a plane × spline loop is seated against its wall's chart, which no seat \
             certifies yet",
        ));
    }
    let Curve3::Nurbs(net) = &component.carrier else {
        return Err(CarryError::Desync(
            "a fitted component's carrier is not a spline",
        ));
    };
    let (lo, hi) = net.domain();
    if at <= lo || at >= hi {
        return Ok(component.clone());
    }
    // A seat so near the carrier's start (its end, the same point) that
    // the piece before it has no knot room of its own is the start
    // already, where it is decided within the band of it
    // ([`at_the_start`]); one decided past the band is a knot edit that
    // refused, and one undecided escalates.
    let seated = match net
        .split_at(at)
        .and_then(|(before, after)| rotated(&after, &before, hi - lo))
    {
        Ok(seated) => seated,
        Err(e) => {
            let ends = [net.eval(lo), net.eval(hi)];
            return match at_the_start(net.eval(at), ends, band) {
                Ok(true) => Ok(component.clone()),
                Ok(false) => Err(CarryError::Seat(e)),
                Err(diag) => Err(CarryError::Site(SiteRefusal::Escalated(diag))),
            };
        }
    };
    // The feature extent the march read.
    let (a, b) = pose.operands();
    let extent = pose.domain().extent;
    let certificate = crate::ssi::certify_rung3(
        &seated,
        None,
        &crate::ssi::SsiOperand::Analytic(a),
        &crate::ssi::SsiOperand::Analytic(b),
        extent,
        band,
    )
    .map_err(CarryError::March)?;
    let span = seated.domain();
    Ok(CarriedComponent {
        carrier: Curve3::Nurbs(std::sync::Arc::new(seated)),
        span,
        closed: true,
        certificate: Some(certificate),
    })
}

/// **Whether the seat `p` is the loop's start**, one of `ends` (the
/// start and the end, one point), decided within the band of it
/// (**`ssi_seat_at_the_start`**): `true` where it is decided at one,
/// `false` where it is decided off both, and the escalation where either
/// reading is undecided, which the seat refuses rather than keep the
/// loop at its old seat.
fn at_the_start(p: Point3<f64>, ends: [Point3<f64>; 2], band: Band) -> Result<bool, Indeterminate> {
    let mut near = false;
    for end in ends {
        let read = geom_core::k_stats::decide_reported(
            "ssi_seat_at_the_start",
            Margin::norm3(p - end),
            band,
        )?;
        near |= read.sign == Sign::Zero;
    }
    Ok(near)
}

/// **`first` followed by `second` shifted by `period`**: two clamped
/// pieces of one closed carrier, `first` ending where `second` starts
/// (the carrier's own start and end, one point). The join keeps
/// `first`'s last control point and drops `second`'s first, its knot
/// at the join of multiplicity the degree, and scales `second`'s weights
/// to meet `first`'s there, so the joined carrier is continuous.
fn rotated(
    first: &geom::NurbsCurve3<f64>,
    second: &geom::NurbsCurve3<f64>,
    period: f64,
) -> Result<geom::NurbsCurve3<f64>, geom_core::spline::KnotAlgebraError> {
    use core::num::NonZeroUsize;
    use geom_core::spline::{KnotAlgebraError, Knots};
    let p = first.degree();
    let runs =
        |c: &geom::NurbsCurve3<f64>| -> Vec<(f64, usize)> { c.knot_values().runs().collect() };
    let (r1, r2) = (runs(first), runs(second));
    let mut joined: Vec<(f64, NonZeroUsize)> = Vec::with_capacity(r1.len() + r2.len());
    let mut push = |v: f64, m: usize| {
        if let Some(m) = NonZeroUsize::new(m) {
            joined.push((v, m));
        }
    };
    let last = r1.len() - 1;
    for &(v, m) in &r1[..last] {
        push(v, m);
    }
    push(r1[last].0, p);
    for &(v, m) in &r2[1..] {
        push(v + period, m);
    }
    let knots = Knots::new(joined, p).map_err(KnotAlgebraError::Structure)?;
    let (w1, w2) = (first.weights(), second.weights());
    let scale = w1[w1.len() - 1] / w2[0];
    let control: Vec<Point3<f64>> = first
        .control()
        .iter()
        .chain(&second.control()[1..])
        .copied()
        .collect();
    let weights: Vec<f64> = w1
        .iter()
        .copied()
        .chain(w2[1..].iter().map(|w| w * scale))
        .collect();
    geom::NurbsCurve3::new(knots, control, weights).map_err(KnotAlgebraError::Structure)
}

/// **Why a section was not carried.**
#[derive(Clone, Debug)]
pub enum CarryError {
    /// A component has no carrier: a tangency, classification only (C7).
    Unminted(&'static str),
    /// The general rung is not marched at this scalar.
    NoLane {
        /// The scalar, by name.
        scalar: &'static str,
    },
    /// The march refused.
    March(SsiError),
    /// A branch's witness has no component.
    Site(SiteRefusal),
    /// The march and the classification disagree.
    Desync(&'static str),
    /// A closed fitted component could not be opened at its first site
    /// ([`PairSection::seat`]): the knot edit refused.
    Seat(geom_core::spline::KnotAlgebraError),
}

/// One component of a carried section.
#[derive(Clone, Debug)]
pub struct CarriedComponent<T: Real> {
    /// Its carrier.
    pub carrier: Curve3<T>,
    /// The carrier's parameter span over the component.
    pub span: (T, T),
    /// Whether the component is closed: its span's ends are one point.
    pub closed: bool,
    /// The general rung's certificate, where it was fitted.
    pub certificate: Option<SsiCertificate<T>>,
}

/// **A section whose every component holds a carrier** (module docs),
/// made only by [`PairSection::carry`].
#[derive(Clone, Debug)]
pub struct CarriedSection<T: Real> {
    components: Vec<CarriedComponent<T>>,
    lever: T,
}

/// A site's place on its component: the component, the foot parameter,
/// and the carrier's tangent there.
#[derive(Clone, Copy, Debug)]
pub struct SiteFoot<T: Real> {
    /// The component's index.
    pub component: usize,
    /// The foot parameter.
    pub t: T,
    /// `C′(t)`.
    pub tangent: Vec3<T>,
}

impl<T: Decide> CarriedSection<T> {
    /// The components.
    pub fn components(&self) -> &[CarriedComponent<T>] {
        &self.components
    }

    /// **The length a direction's angle to a component is levered by**:
    /// the scale every component of the section turns on, the cylinder's
    /// radius for cylinder × sphere and a torus's minor radius (the
    /// smaller of two) for a torus pair.
    pub fn lever(&self) -> T {
        self.lever
    }

    /// **The site `p`'s foot**: the one component it lies within the
    /// band of ([`Curve3::on_carrier`]), the foot parameter there, and
    /// the carrier's tangent. A site on none, or on two, is a desync.
    ///
    /// # Errors
    ///
    /// [`SiteRefusal`].
    pub fn foot(&self, p: Point3<T>, band: Band) -> Result<SiteFoot<T>, SiteRefusal> {
        use geom::{CarrierRefusal, OnCarrier};
        let mut found = None;
        for (k, c) in self.components.iter().enumerate() {
            let foot = c.carrier.foot(p, c.span.0);
            match foot.on_carrier(band) {
                Ok(OnCarrier::On { .. }) => {
                    if found.is_some() {
                        return Err(SiteRefusal::Desync(
                            "a site lies within the band of two components of its section",
                        ));
                    }
                    let t = foot.t();
                    found = Some(SiteFoot {
                        component: k,
                        t,
                        tangent: c.carrier.ders1(t).1,
                    });
                }
                Ok(OnCarrier::Off) => {}
                Err(CarrierRefusal::Escalated(diag) | CarrierRefusal::Straddle(diag)) => {
                    return Err(SiteRefusal::Escalated(diag));
                }
                Err(CarrierRefusal::Exhausted | CarrierRefusal::Unbounded) => {
                    return Err(SiteRefusal::Desync(
                        "a site's distance from a fitted component could not be read",
                    ));
                }
            }
        }
        found.ok_or(SiteRefusal::Desync(
            "a site lies on no component of its section",
        ))
    }
}

#[cfg(test)]
mod wiring {
    use super::{SsiLane, march_over_the_ball, seat_at};

    /// `Ok(())` when the lane holds its march; otherwise the field that
    /// does not.
    fn holds_the_ssi_lane() -> Result<(), &'static str> {
        if !std::ptr::fn_addr_eq(
            SsiLane::marched().march,
            march_over_the_ball as fn(_, _) -> _,
        ) {
            return Err("march is not `march_over_the_ball`");
        }
        if !std::ptr::fn_addr_eq(SsiLane::marched().seat, seat_at as fn(_, _, _, _) -> _) {
            return Err("seat is not `seat_at`");
        }
        Ok(())
    }

    /// **An undecided nearness to the start refuses**: a seat one band
    /// from the loop's start, between the band's zero and its escalation
    /// edges, escalates rather than read as the start; one well inside
    /// the band is the start, and one far past it is not.
    #[test]
    #[allow(clippy::unwrap_used)]
    fn a_seat_undecided_at_the_start_escalates() {
        use geom_core::{Band, Point3, Tol};
        let band = Band::linear(Tol::witness()).unwrap();
        let start = Point3::new(0.3, -0.2, 0.1);
        let ends = [start, start];
        let at = |d: f64| super::at_the_start(Point3::new(0.3 + d, -0.2, 0.1), ends, band);
        assert_eq!(
            at(0.1 * band.zero()),
            Ok(true),
            "inside the band: the start"
        );
        assert_eq!(at(1e-3), Ok(false), "far past the band: not the start");
        assert!(
            at(3.0 * band.zero()).is_err(),
            "between the band's edges: an escalation, never the start: {:?}",
            at(3.0 * band.zero())
        );
    }

    #[test]
    fn f64_is_wired_to_the_ssi_lane() {
        assert_eq!(
            holds_the_ssi_lane(),
            Ok(()),
            "`SsiLane::marched()` holds something other than its march"
        );
    }
}
