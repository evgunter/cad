//! **The section of two distinct surfaces, as one value** (C5): one
//! constructor that is the only dispatch on kind pairs for the pairs it
//! serves, a pose decided once per pair, and the components every reader
//! of the pair takes.
//!
//! # The value
//!
//! [`pair_section`] reads two surfaces, over the operation's region and
//! with the march door where the caller holds them, and answers either
//! the section ([`SectionRead::Built`]) or that the kind pair is served
//! elsewhere ([`SectionRead::Elsewhere`], with [`crate::route`]'s arm):
//! its match on kinds is exhaustive with no wildcard arm, so a kind added
//! to [`SurfaceKind`] is a compile-time visit here as it is at `route`.
//! With no march door it refuses a plane and a spline
//! ([`SectionError::NoMarch`]).
//!
//! A [`PairSection`] holds:
//!
//! - **`decided`**, every `Zero` verdict that chose the arm, with its
//!   margin ([`ZeroVerdict`]): a coincidence (the sphere's centre on the
//!   cylinder's axis) is a record its consumer keeps (D10), and a
//!   tangency is classification data that refuses toward C7 wherever a
//!   carrier is wanted;
//! - **its components, where a closed form counts them**
//!   ([`Components::Counted`]): each a loop, a point, or a curve or region
//!   that reaches a surface's chart boundary ([`ComponentShape`]),
//!   whether it winds about each surface's carrier, a witness point with
//!   how far it may stand from the component, and its carrier at
//!   classification ([`ClassCarrier`]): a closed form, the general rung
//!   (marched on demand by [`PairSection::carry`]), or none (a tangency,
//!   a boundary region or a spline section's piece, classification
//!   only). Where no closed form counts them
//!   ([`Components::Marched`]: a general cylinder × torus or torus ×
//!   torus pose), the components are the march's, listed only once
//!   carried. An empty count is the surfaces disjoint; `Marched` never
//!   reads as that.
//!
//! # Classified, then carried
//!
//! The value is built in two states. [`PairSection`] is the
//! classification: it costs no march and needs no lane, but on a plane
//! and a spline, whose classification is C3's whole reading
//! ([`plane_spline`]), read through the march door.
//! [`PairSection::carry`] turns it into a [`CarriedSection`], which lists
//! the components, every one holding a carrier a reader can take a foot
//! on: a closed form as classified, or the general rung's fitted curve
//! with its C2 certificate ([`crate::ssi::SsiCertificate`]). Where the
//! classification is marched, the carried components are the march's
//! closed branches, each winding as the fitted loop's turns about each
//! surface's axis and core circle, read over the loop's uniqueness tube
//! in ℝ³, say
//! ([`winds`]); where it is counted, the march's distinct branches must
//! match the count, so the closed form is a cross-check of the march's
//! exhaustiveness receipt. A reader that needs a list no closed form
//! counts carries. A site's component is its foot on the carried section
//! ([`CarriedSection::foot`]), the one reading of it. Only `carry`
//! builds a `CarriedSection`, and it refuses where a component has no
//! carrier, so no reader takes a foot on a carrier that does not exist.
//! The march is written at `f64`, and [`SsiLane`] is the door to it:
//! holding one is the statement that the scalar may march
//! (`SsiLane::marched`, at `f64` alone).
//!
//! # The pairs it serves
//!
//! Cylinder × sphere, the three torus pairs (a ring torus against a
//! sphere, a cylinder or another ring torus, [`torus`]) and two
//! cylinders ([`cylinders`]), which have no bounded side and are read
//! over the operation's region; and, through the march door, a plane
//! against a NURBS or approximating surface ([`plane_spline`],
//! [`PlaneSplinePose`], read by the section certificate alone). Each
//! analytic pair's pose
//! ([`CylinderSpherePose`], [`SphereTorusPose`], [`CylinderTorusPose`],
//! [`TorusTorusPose`], [`CylinderCylinderPose`]) is decided once, and
//! every reader of the pair
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

pub mod cylinders;
pub mod gap;
pub mod plane_spline;
pub mod torus;
mod torus_arms;
pub mod torus_coincidence;
pub mod winds;

pub use cylinders::{CcDecision, CcNode, CcStance, CylinderCylinderPose, RegionPlacement};
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
pub use winds::{Winding, WindsRefusal};

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
    /// `cc_radius_equality`: two cylinders of one radius, at the node
    /// pose.
    CylinderRadii,
    /// `cc_axes_coplanar`: two cylinders' axes meeting, at the node pose.
    CylinderAxesMeet,
}

impl Coincident {
    /// The decision's K-funnel name.
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::CylinderSphere => CsDecision::Coaxial.name(),
            Self::SphereTorus => "st_coaxial",
            Self::CylinderTorus | Self::TorusTorus => "section_axes_offset",
            Self::CylinderRadii => CcDecision::RadiusEquality.name(),
            Self::CylinderAxesMeet => CcDecision::AxesMeet.name(),
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

/// Whether a component is a closed curve, one point, or a curve with
/// ends (C3: a component is closed, ends on the domain's boundary, or
/// ends at a node).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ComponentShape {
    /// A closed curve.
    Loop,
    /// One point, where the surfaces touch.
    Point,
    /// A curve with ends, each on the domain's boundary or at a node.
    Open {
        /// Where it ends.
        ends: ComponentEnds,
    },
}

/// **Where an open component ends**: the sides of each surface's
/// parameter rectangle it reaches (rim ends, in the order the section was
/// asked of), whether it reaches the operation's region (a cut end), and
/// the nodes it ends at, by their index in [`PairSection::nodes`].
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ComponentEnds {
    /// The rectangle sides it reaches on each surface.
    pub rims: [ChartReach; 2],
    /// The region cuts it: a marched branch ends on the march domain's
    /// cube, or a closed form runs past the region (it is carried whole,
    /// its carrier exact there).
    pub cut: bool,
    /// The nodes its ends stand at.
    pub nodes: [Option<usize>; 2],
}

impl ComponentEnds {
    /// Both ends on the region's boundary.
    #[must_use]
    pub fn cut() -> Self {
        Self {
            cut: true,
            ..Self::default()
        }
    }

    /// From the node `from` to the node `to`.
    #[must_use]
    pub fn nodes(from: usize, to: usize) -> Self {
        Self {
            nodes: [Some(from), Some(to)],
            ..Self::default()
        }
    }
}

/// **The sides of a surface's parameter rectangle a component reaches**,
/// each named by the parameter it holds fixed and the end it holds it
/// at: whether a side is the surface's boundary, a seam of a closed
/// channel or a collapsed side (a pole) is the reader's question.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ChartReach {
    /// `[u held fixed, v held fixed]`, each `[at the low end, at the
    /// high end]`.
    sides: [[bool; 2]; 2],
}

impl ChartReach {
    /// The side `side` alone.
    #[must_use]
    pub fn side(side: crate::ssi::ChartSide) -> Self {
        let mut sides = [[false; 2]; 2];
        sides[axis_index(side.fixed)][end_index(side.end)] = true;
        Self { sides }
    }

    /// The two sides meeting at `corner`.
    #[must_use]
    pub fn corner(corner: crate::ssi::ChartCorner) -> Self {
        use crate::ssi::{ChartAxis, ChartSide};
        Self::side(ChartSide {
            fixed: ChartAxis::U,
            end: corner.u,
        })
        .or(Self::side(ChartSide {
            fixed: ChartAxis::V,
            end: corner.v,
        }))
    }

    /// The sides either reaches.
    #[must_use]
    pub fn or(self, other: Self) -> Self {
        let mut sides = self.sides;
        for (mine, theirs) in sides.iter_mut().flatten().zip(other.sides.iter().flatten()) {
            *mine |= *theirs;
        }
        Self { sides }
    }

    /// It reaches some side.
    #[must_use]
    pub fn any(self) -> bool {
        self.sides.iter().flatten().any(|&s| s)
    }

    /// The sides it reaches.
    pub fn sides(self) -> impl Iterator<Item = crate::ssi::ChartSide> {
        use crate::ssi::{ChartAxis, ChartEnd, ChartSide};
        [
            (ChartAxis::U, ChartEnd::Low),
            (ChartAxis::U, ChartEnd::High),
            (ChartAxis::V, ChartEnd::Low),
            (ChartAxis::V, ChartEnd::High),
        ]
        .into_iter()
        .filter(move |&(fixed, end)| self.sides[axis_index(fixed)][end_index(end)])
        .map(|(fixed, end)| ChartSide { fixed, end })
    }
}

/// A chart axis's index in [`ChartReach`]'s sides.
fn axis_index(axis: crate::ssi::ChartAxis) -> usize {
    match axis {
        crate::ssi::ChartAxis::U => 0,
        crate::ssi::ChartAxis::V => 1,
    }
}

/// A chart end's index in [`ChartReach`]'s sides.
fn end_index(end: crate::ssi::ChartEnd) -> usize {
    match end {
        crate::ssi::ChartEnd::Low => 0,
        crate::ssi::ChartEnd::High => 1,
    }
}

/// A component's carrier as classified.
#[derive(Clone, Debug)]
pub enum ClassCarrier<T: Real> {
    /// A closed form, the whole of a closed curve.
    Closed(Curve3<T>),
    /// A closed form over a parameter span: an arc between nodes, or a
    /// line cut by the region.
    Span {
        /// The curve.
        curve: Curve3<T>,
        /// The span of it the component is.
        span: (T, T),
    },
    /// The general rung's: marched and fitted by [`PairSection::carry`].
    General,
    /// None mints: the component is classification data, for the reason
    /// given.
    Unminted(&'static str),
}

/// One connected component of a section, as classified.
#[derive(Clone, Debug)]
pub struct SectionComponent<T: Real> {
    /// A loop, a point or a curve with ends.
    pub shape: ComponentShape,
    /// Whether the arm proves it winds about each surface's carrier (it
    /// is essential there), in the order the section was asked of:
    /// `false` claims nothing, since an arm that does not read
    /// essentiality lists none.
    pub winds: [bool; 2],
    /// One point of it, where the arm names one: in closed form, or a
    /// spline section's wall point with a point of the component
    /// bracketed near it ([`plane_spline`]).
    pub witness: Option<Point3<T>>,
    /// **How far the witness may stand from the component**, in metres:
    /// zero for a closed form, the bracket's bound for a spline
    /// section's.
    pub pad: f64,
    /// Its carrier.
    pub carrier: ClassCarrier<T>,
}

impl<T: Real> SectionComponent<T> {
    /// A component a closed form reads: its witness, where it has one,
    /// on the component.
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
            pad: 0.0,
            carrier,
        }
    }
}

/// **A section's components, as classified** (module docs).
#[derive(Clone, Debug)]
pub enum Components<T: Real> {
    /// A closed form counts them: these, and no others. Empty is the
    /// surfaces disjoint.
    Counted(Vec<SectionComponent<T>>),
    /// No closed form counts them: they are the march's closed branches,
    /// listed by [`PairSection::carry`].
    Marched,
}

/// **The section of two distinct surfaces, classified** (module docs).
#[derive(Clone, Debug)]
pub struct PairSection<T: Real> {
    pose: PairPose<T>,
    decided: Vec<ZeroVerdict>,
    components: Components<T>,
    nodes: Vec<Point3<T>>,
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
    /// Cylinder × cylinder.
    CylinderCylinder(CylinderCylinderPose<T>),
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
            | Self::CylinderCylinder(_)
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
            Self::CylinderCylinder(p) => p.operands(),
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
    /// unbounded along its axis, so it is never the ball: two cylinders
    /// are read over their own ball where the region holds it, and over
    /// the region otherwise ([`CylinderCylinderPose::ball`]), which holds
    /// every section point the operation can use.
    pub fn ball(&self) -> ExtentBall<T> {
        match self {
            Self::CylinderSphere(p) => p.ball(),
            Self::SphereTorus(p) => p.ball(),
            Self::CylinderTorus(p) => p.ball(),
            Self::TorusTorus(p) => p.ball(),
            Self::CylinderCylinder(p) => p.ball(),
            Self::PlaneSpline(p) => p.ball(),
        }
    }

    /// Whether the domain cuts the section, so that a marched branch may
    /// end on its boundary: only where the region bounds it.
    pub fn cut(&self) -> bool {
        match self {
            Self::CylinderCylinder(p) => p.cut(),
            Self::CylinderSphere(_)
            | Self::SphereTorus(_)
            | Self::CylinderTorus(_)
            | Self::TorusTorus(_)
            | Self::PlaneSpline(_) => false,
        }
    }
}

impl PairPose<f64> {
    /// **The march's domain**: the cube about [`Self::ball`], padded by
    /// a hundredth of its radius so the slab's faces stand off the ball,
    /// which the bounding surface reaches (a sphere everywhere, a torus
    /// round its outer equator); the feature extent the ball's diameter,
    /// the standard accounting floor. For two cylinders it is the cube of
    /// one ball ([`CylinderCylinderPose::ball`]): their own ball where
    /// the region holds it, else the region's, each a superset of their
    /// ball cut by the region.
    #[must_use]
    pub fn domain(&self) -> SsiDomain {
        if let Self::PlaneSpline(p) = self {
            return p.domain();
        }
        let ball = self.ball();
        domain_about(ball.center(), ball.radius() * 1.01, ball.radius() * 2.0)
    }
}

/// **A march's domain**: the cube of half-width `half_extent` about
/// `center`, its feature extent `extent`, at the standard accounting
/// floor.
fn domain_about(center: Point3<f64>, half_extent: f64, extent: f64) -> SsiDomain {
    SsiDomain {
        center,
        half_extent,
        extent,
        floor_scale: 1.0,
    }
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
/// ([`SectionError::Pinched`], [`SectionError::Unordered`]); a pair with
/// no bounded side asked with no `region` ([`SectionError::NoRegion`]);
/// a plane and a spline asked with no `lane` ([`SectionError::NoMarch`]),
/// and with one, the whole reading's refusal, mapped by name
/// ([`plane_spline`]).
///
/// `region` is the operation's (C5): the ball about the overlap of the
/// two operands' boxes, padded, which holds every section point the
/// operation can use. A pair with no bounded side (two cylinders) is
/// read over it; a pair with a sphere or a torus side is read over that
/// side's own ball and does not read it.
///
/// `lane` is the march door: a plane and a spline wall (an approximating
/// surface read as its fit) are read through it, since their section is
/// C3's whole reading, a march ([`plane_spline`]). No other pair reads it.
pub fn pair_section<T: Decide>(
    a: &Surface<T>,
    b: &Surface<T>,
    region: Option<ExtentBall<T>>,
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
        (Cylinder, Cylinder) => {
            let region = region.ok_or(SectionError::NoRegion {
                pair: "cylinder×cylinder",
            })?;
            match CylinderCylinderPose::decide(a, b, &crate::Reach::Ball(region), band) {
                Some(pose) => cylinders::cylinder_cylinder(pose?, band).map(SectionRead::Built),
                None => Err(SectionError::WrongLane {
                    expected: "two cylinders",
                }),
            }
        }
        (Plane, Nurbs | Approx) | (Nurbs | Approx, Plane) => match lane {
            Some(lane) => (lane.spline)(a, b, band).map(SectionRead::Built),
            None => Err(SectionError::NoMarch { scalar: T::NAME }),
        },
        (Plane, Plane | Cylinder | Cone | Sphere | Torus)
        | (Cylinder, Plane | Cone | Nurbs | Approx)
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
/// (`work/sectrung/the-c5-kind-dispatch-is-spelled-twice.md`). A plane and
/// a spline are not among them: their section is a whole reading, read
/// per pair through [`pair_section`]'s march door by the reader that asks
/// it.
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
        lever: pose.r,
        pose: PairPose::CylinderSphere(pose),
        decided,
        components: Components::Counted(components),
        nodes: Vec::new(),
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

    /// The components, as classified.
    pub fn components(&self) -> &Components<T> {
        &self.components
    }

    /// **The nodes the pose placed** (D2): points where branches of the
    /// section cross and the surfaces are tangent, each decided on the
    /// pose as a `Zero` tangency whose section still has carriers. An
    /// open component names the ones it ends at
    /// ([`ComponentEnds::nodes`]).
    pub fn nodes(&self) -> &[Point3<T>] {
        &self.nodes
    }

    /// The components a closed form counts, where one does.
    pub fn counted(&self) -> Option<&[SectionComponent<T>]> {
        match &self.components {
            Components::Counted(listed) => Some(listed),
            Components::Marched => None,
        }
    }

    /// The coincidence among [`Self::decided`], where the arm stands on
    /// one, with its margin: the record its consumer keeps.
    pub fn coincidence(&self) -> Option<(Coincident, MarginDiag)> {
        self.decided.iter().find_map(|z| match z.decision {
            SectionDecision::Coincidence(c) => Some((c, z.margin)),
            SectionDecision::Tangency(_) => None,
        })
    }

    /// Every coincidence among [`Self::decided`], with its margin: a
    /// node pose of two cylinders stands on two.
    pub fn coincidences(&self) -> impl Iterator<Item = (Coincident, MarginDiag)> + '_ {
        self.decided.iter().filter_map(|z| match z.decision {
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

    /// **The section carried** (module docs): every component holding a
    /// carrier. Marched, the components are the march's closed branches,
    /// in the march's order. Counted, they are the classification's, in
    /// its order: a closed form as classified, and the general rung's
    /// marched, the march's branches matched to them ([`Self::matched`]).
    ///
    /// # Errors
    ///
    /// [`CarryError`].
    pub fn carry(
        &self,
        lane: Option<SsiLane<T>>,
        band: Band,
    ) -> Result<CarriedSection<T>, CarryError> {
        let marched = || {
            let lane = lane.ok_or(CarryError::NoLane { scalar: T::NAME })?;
            (lane.march)(&self.pose, band)
        };
        self.listed(marched, band)
    }

    /// **The components listed from the march's branches** as
    /// [`Self::carry`] lists them, `branches` asked for only where a
    /// component is the general rung's.
    fn listed(
        &self,
        branches: impl FnOnce() -> Marched<T>,
        band: Band,
    ) -> Result<CarriedSection<T>, CarryError> {
        let components = match &self.components {
            Components::Marched => distinct(
                branches()?
                    .into_iter()
                    .map(|b| b.component(self.pose.cut()))
                    .collect::<Result<_, _>>()?,
                band,
            )?,
            Components::Counted(listed) => {
                for c in listed {
                    if let ClassCarrier::Unminted(why) = c.carrier {
                        return Err(CarryError::Unminted(why));
                    }
                }
                let general = listed
                    .iter()
                    .any(|c| matches!(c.carrier, ClassCarrier::General));
                let fitted = if general { branches()? } else { Vec::new() };
                Self::matched(listed, fitted, self.pose.cut(), band)?
            }
        };
        Ok(CarriedSection {
            components,
            lever: self.lever,
        })
    }

    /// **The march's branches matched to the listed components**, the
    /// closed form a cross-check of the march: the march finds exactly as
    /// many distinct branches as the classification lists general
    /// components ([`distinct`]); a listed witness lies on exactly one
    /// branch, which is its component's; and the witnessless components
    /// take the rest, each a branch winding at least as it claims, by a
    /// matching that does not depend on the list's order. Anything else
    /// is a desync.
    fn matched(
        listed: &[SectionComponent<T>],
        branches: Vec<FittedCarrier<T>>,
        cut: bool,
        band: Band,
    ) -> Result<Vec<CarriedComponent<T>>, CarryError> {
        let general: Vec<usize> = (0..listed.len())
            .filter(|&i| matches!(listed[i].carrier, ClassCarrier::General))
            .collect();
        if branches.len() != general.len() {
            return Err(CarryError::Desync(
                "the march found a different number of branches than the classification counts",
            ));
        }
        let branches = distinct(
            branches
                .into_iter()
                .map(|b| b.component(cut))
                .collect::<Result<_, _>>()?,
            band,
        )?;
        let claims = |listed: [bool; 2], marched: [bool; 2]| {
            listed.iter().zip(marched).all(|(&l, m)| !l || m)
        };
        // `owner[j]`: the listed component branch `j` is matched to.
        let mut owner: Vec<Option<usize>> = vec![None; branches.len()];
        for &i in &general {
            let Some(w) = listed[i].witness else {
                continue;
            };
            let mut on = None;
            for (j, b) in branches.iter().enumerate() {
                if b.holds(w, band)? {
                    if on.is_some() {
                        return Err(CarryError::Desync(
                            "a listed component's witness lies on two of the march's branches",
                        ));
                    }
                    on = Some(j);
                }
            }
            let j = on.ok_or(CarryError::Desync(
                "a listed component's witness lies on no branch the march found",
            ))?;
            if owner[j].is_some() {
                return Err(CarryError::Desync(
                    "two listed components' witnesses lie on one branch",
                ));
            }
            if !claims(listed[i].winds, branches[j].winds) {
                return Err(CarryError::Desync(
                    "a branch does not wind as the listed component it holds the witness of",
                ));
            }
            owner[j] = Some(i);
        }
        // The witnessless components against the branches left, by
        // augmenting paths: a matching exists exactly where this finds
        // one, whatever order the components are listed in.
        let free: Vec<usize> = (0..branches.len())
            .filter(|&j| owner[j].is_none())
            .collect();
        let edge = |i: usize, j: usize| claims(listed[i].winds, branches[j].winds);
        fn augment(
            i: usize,
            free: &[usize],
            edge: &dyn Fn(usize, usize) -> bool,
            seen: &mut [bool],
            owner: &mut [Option<usize>],
        ) -> bool {
            for &j in free {
                if seen[j] || !edge(i, j) {
                    continue;
                }
                seen[j] = true;
                if owner[j].is_none_or(|k| augment(k, free, edge, seen, owner)) {
                    owner[j] = Some(i);
                    return true;
                }
            }
            false
        }
        for &i in general.iter().filter(|&&i| listed[i].witness.is_none()) {
            let mut seen = vec![false; branches.len()];
            if !augment(i, &free, &edge, &mut seen, &mut owner) {
                return Err(CarryError::Desync(
                    "the march's branches do not wind as the listed components claim",
                ));
            }
        }
        let mut slot: Vec<Option<CarriedComponent<T>>> = vec![None; listed.len()];
        for (j, b) in branches.into_iter().enumerate() {
            if let Some(i) = owner[j] {
                slot[i] = Some(b);
            }
        }
        listed
            .iter()
            .zip(slot)
            .map(|(c, fitted)| match (&c.carrier, fitted) {
                (ClassCarrier::Closed(curve), _) => Ok(CarriedComponent {
                    carrier: curve.clone(),
                    span: (T::zero(), T::from_f64(core::f64::consts::TAU)),
                    shape: ComponentShape::Loop,
                    certificate: None,
                    winds: c.winds,
                    pad: 0.0,
                    reach: 0.0,
                    witness: curve.eval(T::pi()),
                }),
                (ClassCarrier::Span { curve, span }, _) => Ok(CarriedComponent {
                    carrier: curve.clone(),
                    span: *span,
                    shape: c.shape,
                    certificate: None,
                    winds: c.winds,
                    pad: 0.0,
                    reach: 0.0,
                    witness: curve.eval((span.0 + span.1) * T::from_f64(0.5)),
                }),
                (ClassCarrier::General, Some(f)) => Ok(f),
                (ClassCarrier::General, None) => Err(CarryError::Desync(
                    "the march found no branch on a component the classification names",
                )),
                (ClassCarrier::Unminted(why), _) => Err(CarryError::Unminted(why)),
            })
            .collect()
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
        if !component.closed() || component.certificate.is_none() {
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
    /// How the march ended it: closed on itself, or cut by the domain.
    pub end: BranchEnd,
    /// The three-limb certificate.
    pub certificate: SsiCertificate<T>,
    /// A point of it (`carrier(mid)`).
    pub witness: Point3<T>,
    /// How it winds about each operand, in the order the section was
    /// asked of ([`winds`]).
    pub winding: [Winding; 2],
    /// How far from its carrier a point of the loop is read, in metres
    /// ([`CarriedComponent::pad`]).
    pub pad: f64,
    /// How far the witness may stand from the loop, in metres
    /// ([`CarriedComponent::reach`]).
    pub reach: f64,
}

impl<T: Decide> FittedCarrier<T> {
    /// The branch as a carried component: a closed one where the
    /// march's box holds the whole section, and one cut by the domain
    /// where the domain is `cut` ([`PairPose::cut`]).
    fn component(self, cut: bool) -> Result<CarriedComponent<T>, CarryError> {
        let shape = match (self.end, cut) {
            (BranchEnd::Closed, _) => ComponentShape::Loop,
            (BranchEnd::Cut, true) => ComponentShape::Open {
                ends: ComponentEnds::cut(),
            },
            (BranchEnd::Cut, false) => {
                return Err(CarryError::Desync(
                    "the march left the ball's box, which holds the whole section",
                ));
            }
            (BranchEnd::Rim { .. }, _) => {
                return Err(CarryError::Desync(
                    "an ℝ³ march ended on a chart's side, which it does not trace",
                ));
            }
        };
        Ok(CarriedComponent {
            carrier: self.carrier,
            span: self.span,
            shape,
            certificate: Some(self.certificate),
            winds: self.winding.map(Winding::essential),
            pad: self.pad,
            reach: self.reach,
            witness: self.witness,
        })
    }
}

/// **The general rung's door** (module docs): the ℝ³ march of a
/// decided pose over its ball, and a plane and a spline's whole reading,
/// written at `f64`. Holding one is the statement that the scalar may
/// march; [`SsiLane::marched`], at `f64`, is its one constructor.
#[derive(Clone, Copy, Debug)]
pub struct SsiLane<T: Real> {
    /// One march of a decided pose, each branch's winding read.
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
type Marched<T> = Result<Vec<FittedCarrier<T>>, CarryError>;

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

/// **The ℝ³ march of `pose` over its ball's box** ([`PairPose::domain`]).
/// Where the ball holds every component of the section no branch reaches
/// the box's faces, so every branch the march finds closes; where the
/// region cuts it ([`PairPose::cut`]) a branch may end on them. Each
/// closed branch's winding about each operand is read over its
/// uniqueness tube ([`winds`]).
fn march_over_the_ball(
    pose: &PairPose<f64>,
    band: Band,
) -> Result<Vec<FittedCarrier<f64>>, CarryError> {
    let domain = pose.domain();
    let out = match pose {
        PairPose::CylinderSphere(p) => crate::ssi::march_cylinder_sphere(p, domain, band),
        PairPose::SphereTorus(p) => crate::ssi::march_sphere_torus(p, domain, band),
        PairPose::CylinderTorus(p) => crate::ssi::march_cylinder_torus(p, domain, band),
        PairPose::TorusTorus(p) => crate::ssi::march_torus_torus(p, domain, band),
        PairPose::CylinderCylinder(p) => crate::ssi::march_cylinder_cylinder(p, domain, band),
        // Its components are read whole when the section is built, and
        // none is carried.
        PairPose::PlaneSpline(_) => {
            return Err(CarryError::Unminted(plane_spline::UNCARRIED));
        }
    }
    .map_err(CarryError::March)?;
    let (a, b) = pose.operands();
    out.branches
        .into_iter()
        .map(|br| {
            let closed = matches!(br.end, BranchEnd::Closed);
            let tube = Tube::of(
                &br.carrier,
                &br.certificate,
                0.5 * (br.params.0 + br.params.1),
            )?;
            let winding = if closed {
                let read = |s| winds::winding(s, &br.carrier, br.params, &tube.chain);
                [read(a), read(b)]
                    .into_iter()
                    .collect::<Result<Vec<_>, _>>()
                    .map_err(CarryError::Winds)?
                    .try_into()
                    .map_err(|_| CarryError::Desync("a branch's winding is read per operand"))?
            } else {
                [Winding::default(); 2]
            };
            Ok(FittedCarrier {
                carrier: br.carrier,
                span: br.params,
                end: br.end,
                certificate: br.certificate,
                witness: br.witness,
                winding,
                pad: tube.pad,
                reach: tube.reach,
            })
        })
        .collect()
}

/// **A marched loop's uniqueness tube** (C2 limb 3): its certificate's
/// box chain, each box padded by the tube radius
/// (`ssi::certify::tube_chain`), the cover the march's exhaustiveness
/// accounts the section to; limb 3 proves the section's points in it
/// one arc, a piece of it in every box. Read three ways: as the
/// winding's pieces ([`winds`]), for the pad ([`CarriedComponent::pad`])
/// and for the witness's reach ([`CarriedComponent::reach`]).
struct Tube {
    chain: Vec<winds::Piece>,
    pad: f64,
    reach: f64,
}

impl Tube {
    /// The tube of the loop `carrier` certified by `certificate`, its
    /// witness at `carrier(at)`.
    ///
    /// - **The pad** is the farthest any point of the tube stands from
    ///   the carrier: each box is halved [`PAD_SPLITS`] times
    ///   (`Box3::split`), and a part's every point is no farther from
    ///   the carrier than from the nearest of [`PAD_SAMPLES`] carrier
    ///   points over the box's span ([`farthest`]). The carrier's limb-2
    ///   bound must lie within it too, so the carrier lies within the pad
    ///   of each surface; otherwise it refuses [`CarryError::PadShort`].
    /// - **The reach** bounds the witness's distance from the component:
    ///   a box whose span holds `at` holds the witness and a piece of the
    ///   component, so the farthest the box reaches from the witness
    ///   bounds it; the least over such boxes.
    ///
    /// An empty chain, or a box or carrier point that is refused, reads
    /// no tube ([`CarryError::TubeUnread`]).
    fn of(
        carrier: &Curve3<f64>,
        certificate: &SsiCertificate<f64>,
        at: f64,
    ) -> Result<Self, CarryError> {
        let crate::ssi::SsiTube::Spatial { radius } = certificate.tube else {
            return Err(CarryError::Desync(
                "an ℝ³ march's certificate proved no spatial tube",
            ));
        };
        let Curve3::Nurbs(net) = carrier else {
            return Err(CarryError::Desync(
                "a fitted component's carrier is not a spline",
            ));
        };
        let lift = <geom_core::Interval as Real>::from_computed;
        let lifted = net.map_scalar(lift);
        let chain = crate::ssi::certify::tube_chain(net, radius);
        let pad = (!chain.is_empty()).then_some(()).and_then(|()| {
            chain.iter().try_fold(0.0f64, |acc, &(b, _, (lo, hi))| {
                #[allow(clippy::cast_precision_loss)]
                let samples: Vec<Point3<geom_core::Interval>> = (0..PAD_SAMPLES)
                    .map(|j| {
                        let t = lo + (hi - lo) * j as f64 / (PAD_SAMPLES - 1) as f64;
                        lifted.eval(lift(t))
                    })
                    .collect();
                parts(b).into_iter().try_fold(acc, |acc, part| {
                    let far = samples
                        .iter()
                        .map(|&p| farthest(part, p))
                        .fold(f64::INFINITY, f64::min);
                    far.is_finite().then(|| acc.max(far))
                })
            })
        });
        let Some(pad) = pad else {
            return Err(CarryError::TubeUnread);
        };
        if certificate.hull_sup.partial_cmp(&pad) == Some(core::cmp::Ordering::Greater)
            || certificate.hull_sup.is_nan()
        {
            return Err(CarryError::PadShort {
                pad,
                hull_sup: certificate.hull_sup,
            });
        }
        let witness = lifted.eval(lift(at));
        let reach = chain
            .iter()
            .filter(|(_, _, (lo, hi))| *lo <= at && at <= *hi)
            .map(|&(b, _, _)| farthest(b, witness))
            .filter(|r| r.is_finite())
            .fold(f64::INFINITY, f64::min);
        if !reach.is_finite() {
            return Err(CarryError::TubeUnread);
        }
        Ok(Self { chain, pad, reach })
    }
}

/// How many times [`Tube::of`] halves each tube box for the pad: into
/// `2³` parts.
const PAD_SPLITS: u32 = 3;

/// The carrier points [`Tube::of`] reads each tube box's parts against,
/// evenly over the box's span, its ends among them.
const PAD_SAMPLES: usize = 9;

/// `b` halved [`PAD_SPLITS`] times along its widest axis
/// (`Box3::split`, which keeps a refused axis refused); the parts cover
/// it.
fn parts(b: crate::ssi::enclose::Box3) -> Vec<crate::ssi::enclose::Box3> {
    (0..PAD_SPLITS).fold(vec![b], |boxes, _| {
        boxes
            .into_iter()
            .flat_map(|b| {
                let (l, r) = b.split();
                [l, r]
            })
            .collect()
    })
}

/// An upper bound on the distance from any point of `b` to any point of
/// the enclosure `p`: the diagonal of the box of per-axis far gaps
/// (`carrier_net::diagonal`), rounded up by a relative `1e-12`, far past
/// its few roundings. `NaN` where either side is refused.
fn farthest(b: crate::ssi::enclose::Box3, p: Point3<geom_core::Interval>) -> f64 {
    use geom_core::{Bounds as _, Interval};
    let gap = |s: Interval, q: Interval| {
        if s.is_certified() && q.is_certified() {
            (0.0, (s.hi() - q.lo()).max(q.hi() - s.lo()))
        } else {
            (0.0, f64::NAN)
        }
    };
    let gaps = [gap(b.x, p.x), gap(b.y, p.y), gap(b.z, p.z)];
    crate::carrier_net::diagonal(&gaps) * (1.0 + 1e-12)
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
        return Err(CarryError::Unminted(plane_spline::UNCARRIED));
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
                Err(diag) => Err(CarryError::Escalated(diag)),
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
    let carrier = Curve3::Nurbs(std::sync::Arc::new(seated));
    let mid = 0.5 * (span.0 + span.1);
    let tube = Tube::of(&carrier, &certificate, mid)?;
    Ok(CarriedComponent {
        witness: carrier.eval(mid),
        carrier,
        span,
        shape: ComponentShape::Loop,
        certificate: Some(certificate),
        winds: component.winds,
        pad: tube.pad,
        reach: tube.reach,
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
    /// A branch's winding was not read.
    Winds(WindsRefusal),
    /// A reading in the band or poisoned.
    Escalated(Indeterminate),
    /// A fitted loop's limb-2 bound stands past the pad its points are
    /// read within ([`CarriedComponent::pad`]).
    PadShort {
        /// The pad, in metres.
        pad: f64,
        /// The certificate's limb-2 bound, in metres.
        hull_sup: f64,
    },
    /// A fitted loop's uniqueness tube could not be read: its box chain
    /// is empty, or a box or carrier point of it is refused ([`Tube::of`]).
    TubeUnread,
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
    /// A loop, whose span's ends are one point, or a curve with ends.
    pub shape: ComponentShape,
    /// The general rung's certificate, where it was fitted.
    pub certificate: Option<SsiCertificate<T>>,
    /// Whether it winds about each surface's carrier, in the order the
    /// section was asked of: a closed form's as classified, a fitted
    /// loop's turns about each surface's axis and core read over its
    /// uniqueness tube ([`winds`]).
    pub winds: [bool; 2],
    /// **How far from its carrier a point of this component is read**,
    /// in metres; zero for a closed form. Every reading of a point
    /// against the component pads its band by it ([`Self::reads`]), and
    /// the certificate's W3 rests on it. For
    /// a marched loop it is [`Tube::of`]'s: two facts make it the pad, each
    /// a limb of the loop's C2 certificate.
    ///
    /// - **Closeness**: every point of the component lies within `pad`
    ///   of the carrier. The march's exhaustiveness receipt accounts each
    ///   cell of the domain to a branch's uniqueness tube, the hull boxes
    ///   padded by the tube radius (limb 3 says each holds exactly one
    ///   arc, the branch's own), and the pad bounds every point of every
    ///   padded box's distance from the carrier. And limbs 1 and 2
    ///   put the carrier within `hull_sup` of each surface, which the pad
    ///   is checked to hold ([`CarryError::PadShort`]), so a point of the
    ///   carrier lies within the pad of each surface too.
    /// - **Uniqueness**: limb 3's tube holds this loop's arc alone, so no
    ///   other component crosses it. A point read on two components
    ///   (each within its pad) is not chosen between: it desyncs.
    pub pad: f64,
    /// **How far the witness may stand from the component**, in metres;
    /// zero for a closed form. For a marched loop it is the diameter of
    /// a tube box holding the witness ([`Tube::of`]): limb 3 puts a
    /// piece of the component in every box of the chain. The
    /// certificate's W3 reads the witness within it.
    pub reach: f64,
    /// A point of the carrier, `carrier(mid)`: the witness the component
    /// is read by.
    pub witness: Point3<T>,
}

impl<T: Decide> CarriedComponent<T> {
    /// Whether it is a loop.
    #[must_use]
    pub fn closed(&self) -> bool {
        self.shape == ComponentShape::Loop
    }

    /// **Where the point `p` of the section lies against this
    /// component**: its foot on the carrier where `p` is within the band
    /// padded by [`Self::pad`] ([`Curve3::on_carrier`], [`Band::padded`]),
    /// `None` where it is past it. The one reading of "a point on this
    /// component", by [`CarriedSection::foot`] and the counted section's
    /// cross-check alike.
    ///
    /// # Errors
    ///
    /// [`SiteRefusal`]: an undecided reading escalates.
    pub fn reads(&self, p: Point3<T>, band: Band) -> Result<Option<T>, SiteRefusal> {
        use geom::{CarrierRefusal, OnCarrier};
        let padded = band
            .padded(self.pad)
            .map_err(|_| SiteRefusal::Desync("a component's tube pads no band"))?;
        let foot = self.carrier.foot(p, self.span.0);
        match foot.on_carrier(padded) {
            Ok(OnCarrier::On { .. }) => Ok(Some(foot.t())),
            Ok(OnCarrier::Off) => Ok(None),
            Err(CarrierRefusal::Escalated(diag) | CarrierRefusal::Straddle(diag)) => {
                Err(SiteRefusal::Escalated(diag))
            }
            Err(CarrierRefusal::Exhausted | CarrierRefusal::Unbounded) => Err(SiteRefusal::Desync(
                "a point's distance from a fitted component could not be read",
            )),
        }
    }

    /// [`Self::reads`] as the carry reads it.
    fn holds(&self, w: Point3<T>, band: Band) -> Result<bool, CarryError> {
        self.reads(w, band)
            .map(|t| t.is_some())
            .map_err(|e| match e {
                SiteRefusal::Escalated(diag) => CarryError::Escalated(diag),
                SiteRefusal::Desync(what) => CarryError::Desync(what),
            })
    }
}

/// **The march's branches, each a distinct loop**: each branch's
/// witness, `carrier(mid)`, lies on that branch alone
/// ([`CarriedComponent::reads`]). A march that found one loop twice
/// holds a witness on two branches, which the count cannot see: a desync.
/// The tube's uniqueness is what makes this a test of identity: no point
/// of another loop lies within a branch's tube.
fn distinct<T: Decide>(
    branches: Vec<CarriedComponent<T>>,
    band: Band,
) -> Result<Vec<CarriedComponent<T>>, CarryError> {
    for (i, b) in branches.iter().enumerate() {
        let w = b.witness;
        for (j, other) in branches.iter().enumerate() {
            if i != j && other.holds(w, band)? {
                return Err(CarryError::Desync(
                    "the march found one loop of the section twice",
                ));
            }
        }
    }
    Ok(branches)
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

    /// **The site `p`'s foot**: the one component it lies on
    /// ([`CarriedComponent::reads`]), the foot parameter there, and the
    /// carrier's tangent. A site on none, or on two, is a desync.
    ///
    /// # Errors
    ///
    /// [`SiteRefusal`].
    pub fn foot(&self, p: Point3<T>, band: Band) -> Result<SiteFoot<T>, SiteRefusal> {
        let mut found = None;
        for (k, c) in self.components.iter().enumerate() {
            if let Some(t) = c.reads(p, band)? {
                if found.is_some() {
                    return Err(SiteRefusal::Desync(
                        "a site lies on two components of its section",
                    ));
                }
                found = Some(SiteFoot {
                    component: k,
                    t,
                    tangent: c.carrier.ders1(t).1,
                });
            }
        }
        found.ok_or(SiteRefusal::Desync(
            "a site lies on no component of its section",
        ))
    }
}

#[cfg(test)]
#[allow(clippy::panic, clippy::expect_used, clippy::unwrap_used)]
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

    fn band() -> geom_core::Band {
        geom_core::Band::linear(geom_core::Tol::witness()).expect("the witness band")
    }

    /// The cylinder `radius 0.5` about `z`, and the sphere `radius` at
    /// `center`.
    fn wall_and_ball(
        center: (f64, f64, f64),
        radius: f64,
    ) -> (geom::Surface<f64>, geom::Surface<f64>) {
        use geom_core::{Point3, Vec3};
        (
            geom::Surface::Cylinder {
                origin: Point3::new(0.0, 0.0, -3.0),
                axis: Vec3::unit_z(),
                radius: 0.5,
                u_ref: Vec3::unit_x(),
            },
            geom::Surface::Sphere {
                center: Point3::new(center.0, center.1, center.2),
                radius,
                axis: Vec3::unit_z(),
                u_ref: Vec3::unit_x(),
            },
        )
    }

    /// The germ row's ball (radius `1/2` at the origin) against its donut
    /// (`R = 0.6`, `r = 0.2` about `y`) turned 0.9 rad about `(1, 1, 0)`
    /// and moved to `(0.05, 0.1, 0.02)`: the sphere cuts both tube
    /// circles of its meridian, two loops about the torus axis that no
    /// witness names.
    fn ball_round_the_axis() -> (geom::Surface<f64>, geom::Surface<f64>) {
        use geom_core::{Point3, Vec3};
        let k = Vec3::new(1.0, 1.0, 0.0).normalize();
        let (sn, cs) = 0.9f64.sin_cos();
        let y = Vec3::unit_y();
        let axis = y * cs + k.cross(y) * sn + k * (k.dot(y) * (1.0 - cs));
        let u_ref = axis.orthonormal_basis().0;
        (
            geom::Surface::Sphere {
                center: Point3::origin(),
                radius: 0.5,
                axis: Vec3::unit_z(),
                u_ref: Vec3::unit_x(),
            },
            geom::Surface::Torus {
                center: Point3::new(0.05, 0.1, 0.02),
                axis,
                major_radius: 0.6,
                minor_radius: 0.2,
                u_ref,
            },
        )
    }

    fn built(a: &geom::Surface<f64>, b: &geom::Surface<f64>) -> super::PairSection<f64> {
        match super::pair_section(a, b, None, None, band()) {
            Ok(super::SectionRead::Built(s)) => s,
            other => panic!("the value builds the pair: {other:?}"),
        }
    }

    /// The march with its first branch in its second's place: one loop
    /// found twice, the other dropped.
    fn doubled(
        pose: &super::PairPose<f64>,
        band: geom_core::Band,
    ) -> Result<Vec<super::FittedCarrier<f64>>, super::CarryError> {
        let mut out = march_over_the_ball(pose, band)?;
        assert!(out.len() >= 2, "a section of two loops at least");
        out[1] = out[0].clone();
        Ok(out)
    }

    /// **A doubled ring is caught** (R1): two general rings of a sphere
    /// past a wall's far ruling, and the ball round the donut's axis,
    /// listed from a march that found one loop twice, read counted and
    /// read marched. The count agrees and no witness is listed, so only
    /// the loops' identity can see it: each branch's witness lies on the
    /// other branch. (The lane is the door's own: its fields are written
    /// by its constructor alone, so the doubled march is handed to the
    /// listing `carry` hands its march to.)
    #[test]
    fn a_doubled_ring_is_caught() {
        use super::{CarryError, ClassCarrier, Components, SsiLane};
        for (what, (a, b)) in [
            (
                "two rings round the wall",
                wall_and_ball((0.1, 0.0, 0.2), 0.7),
            ),
            ("the ball round the donut's axis", ball_round_the_axis()),
        ] {
            let section = built(&a, &b);
            let listed = section
                .counted()
                .unwrap_or_else(|| panic!("{what}: counted"));
            assert!(
                listed.len() == 2
                    && listed
                        .iter()
                        .all(|c| c.witness.is_none() && matches!(c.carrier, ClassCarrier::General)),
                "{what}: two witnessless general loops"
            );
            assert!(
                section.carry(Some(SsiLane::marched()), band()).is_ok(),
                "{what}: the march carries"
            );
            let marched = super::PairSection {
                components: Components::Marched,
                ..section.clone()
            };
            for (read, s) in [("counted", &section), ("marched", &marched)] {
                let got = s.listed(|| doubled(s.pose(), band()), band());
                assert!(
                    matches!(got, Err(CarryError::Desync(_))),
                    "{what}, {read}: one loop marched twice: {:?}",
                    got.map(|c| c.components().len())
                );
            }
        }
    }

    /// **A doubled loop among witnessless components is a desync** (R2),
    /// read on the matching itself, in either list order; and a listed
    /// witness on no branch the march found (a lone loop's, against
    /// another pose's march) is one too.
    #[test]
    #[allow(clippy::panic)]
    fn a_doubled_loop_among_witnessless_components_is_a_desync() {
        use super::{CarryError, PairSection};
        let (a, b) = ball_round_the_axis();
        let section = built(&a, &b);
        let mut listed = section.counted().expect("counted").to_vec();
        let fitted = march_over_the_ball(section.pose(), band()).expect("the march");
        assert_eq!(fitted.len(), 2, "two loops");
        for (order, branch) in fitted.iter().enumerate() {
            let twice = vec![branch.clone(), branch.clone()];
            assert!(
                matches!(
                    PairSection::matched(&listed, twice, false, band()),
                    Err(CarryError::Desync(_))
                ),
                "branch {order} twice"
            );
            assert!(
                PairSection::matched(&listed, fitted.clone(), false, band()).is_ok(),
                "the march's own two, listed in order {order}"
            );
            listed.reverse();
        }
        let (wall, here) = wall_and_ball((0.5, 0.0, 0.2), 0.3);
        let (_, there) = wall_and_ball((0.5, 0.0, 1.2), 0.3);
        let lone = built(&wall, &here);
        let elsewhere = built(&wall, &there);
        let listed = lone.counted().expect("counted");
        assert!(listed[0].witness.is_some(), "a lone loop's witness");
        let theirs = march_over_the_ball(elsewhere.pose(), band()).expect("the march");
        assert!(
            matches!(
                PairSection::matched(listed, theirs, false, band()),
                Err(CarryError::Desync(_))
            ),
            "a witness on no branch the march found"
        );
    }

    /// **The witnessless matching is not first fit** (an order that
    /// defeats it): the ball round the donut's axis with its two
    /// components claiming `[true, false]` and `[false, true]` against
    /// branches winding `[true, true]` and `[true, false]`, listed in the
    /// order where first fit gives the first component the `[true, true]`
    /// branch and leaves the second none. The matching takes the
    /// `[true, false]` branch for the first and carries.
    #[test]
    fn the_witnessless_matching_is_not_first_fit() {
        use super::{PairSection, Winding};
        let (a, b) = ball_round_the_axis();
        let section = built(&a, &b);
        let mut listed = section.counted().expect("counted").to_vec();
        listed[0].winds = [true, false];
        listed[1].winds = [false, true];
        let mut fitted = march_over_the_ball(section.pose(), band()).expect("the march");
        let turn = Winding {
            about_axis: 1,
            about_core: 0,
        };
        fitted[0].winding = [turn, turn];
        fitted[1].winding = [turn, Winding::default()];
        let carried = PairSection::matched(&listed, fitted, false, band())
            .unwrap_or_else(|e| panic!("a matching exists: {e:?}"));
        assert_eq!(
            carried.iter().map(|c| c.winds).collect::<Vec<_>>(),
            [[true, false], [true, true]],
            "each component on the branch its claim needs"
        );
    }

    /// **Closeness is checked**: every marched loop's limb-2 bound lies
    /// within its pad, which holds its uniqueness tube's radius; every
    /// corner and centre of every tube box lies within the pad of the
    /// carrier (read against `4096` carrier points, within their
    /// spacing), and the pad stands well inside the hull-diameter bound;
    /// the witness lies in every box whose span holds it, and its reach
    /// is no wider than one; and a certificate whose limb-2 bound stands
    /// past the pad refuses `PadShort`.
    #[test]
    fn a_marched_loops_pad_holds_its_tube_and_its_limb_two_bound() {
        use super::{CarryError, Tube};
        use crate::ssi::SsiTube;
        use geom_core::{Bounds as _, Point3};
        for (what, (a, b)) in [
            (
                "two rings round the wall",
                wall_and_ball((0.1, 0.0, 0.2), 0.7),
            ),
            ("the ball round the donut's axis", ball_round_the_axis()),
        ] {
            let section = built(&a, &b);
            for br in march_over_the_ball(section.pose(), band()).expect("the march") {
                let SsiTube::Spatial { radius } = br.certificate.tube else {
                    panic!("{what}: a spatial tube");
                };
                assert!(
                    br.certificate.hull_sup <= br.pad && radius < br.pad,
                    "{what}: limb 2 {:e}, tube {radius:e}, pad {:e}",
                    br.certificate.hull_sup,
                    br.pad
                );
                let (lo, hi) = br.span;
                let mid = 0.5 * (lo + hi);
                let tube = Tube::of(&br.carrier, &br.certificate, mid).expect("the tube");
                assert_eq!((tube.pad, tube.reach), (br.pad, br.reach), "{what}");
                let dense: Vec<_> = (0..=4096)
                    .map(|i| br.carrier.eval(lo + (hi - lo) * f64::from(i) / 4096.0))
                    .collect();
                let spacing = dense
                    .windows(2)
                    .map(|w| (w[1] - w[0]).norm())
                    .fold(0.0, f64::max);
                let mut worst = 0.0f64;
                for (bx, _, _) in &tube.chain {
                    let [x, y, z] = [bx.x, bx.y, bx.z].map(|s| [s.lo(), s.hi()]);
                    let centre = Point3::new(
                        0.5 * (x[0] + x[1]),
                        0.5 * (y[0] + y[1]),
                        0.5 * (z[0] + z[1]),
                    );
                    let corners =
                        (0..8).map(|k| Point3::new(x[k & 1], y[(k >> 1) & 1], z[(k >> 2) & 1]));
                    for q in corners.chain([centre]) {
                        let d = dense
                            .iter()
                            .map(|&c| (q - c).norm())
                            .fold(f64::INFINITY, f64::min);
                        worst = worst.max(d);
                        assert!(
                            d <= br.pad + spacing,
                            "{what}: a tube point {d:e} from the carrier, pad {:e}",
                            br.pad
                        );
                    }
                }
                let diag = |bx: &crate::ssi::enclose::Box3| {
                    crate::carrier_net::diagonal(&[bx.x, bx.y, bx.z].map(|s| (s.lo(), s.hi())))
                };
                let held: Vec<_> = tube
                    .chain
                    .iter()
                    .filter(|(_, _, (l, h))| *l <= mid && mid <= *h)
                    .collect();
                assert!(
                    !held.is_empty()
                        && held.iter().all(|(bx, _, _)| {
                            let w = br.witness;
                            [(bx.x, w.x), (bx.y, w.y), (bx.z, w.z)]
                                .iter()
                                .all(|(s, v)| s.lo() <= *v && *v <= s.hi())
                        }),
                    "{what}: the witness lies in the boxes whose span holds it"
                );
                assert!(
                    br.reach
                        <= held
                            .iter()
                            .map(|(bx, _, _)| diag(bx))
                            .fold(f64::INFINITY, f64::min),
                    "{what}: the reach {:e} is no wider than a box holding the witness",
                    br.reach
                );
                // Tight as well as sound: within 1.5 times the farthest
                // tube corner or centre read on the dense carrier (the
                // hull-diameter bound it replaces read up to twice that).
                assert!(
                    br.pad <= 1.5 * worst + spacing,
                    "{what}: pad {:e} against the tube's farthest sampled point {worst:e}",
                    br.pad
                );
                let mut loose = br.certificate;
                loose.hull_sup = 2.0 * br.pad;
                assert!(
                    matches!(
                        Tube::of(&br.carrier, &loose, mid),
                        Err(CarryError::PadShort { .. })
                    ),
                    "{what}: a limb-2 bound past the pad refuses"
                );
            }
        }
    }

    /// **Close loops carry**: the wall of radius `1/2` against a sphere at
    /// `(0.1, 0, 0.2)` of radius `0.6 + δ`, its two rings `δ` from
    /// meeting at the far ruling, carry as counted and as marched, two
    /// loops each, from `δ = 3e-2` down to `3e-4`.
    #[test]
    fn close_rings_carry_counted_and_marched() {
        // Pinned red at 1e-6: at δ 3e-4 no rung of the march's tube is a
        // graph, and nothing refines it
        // (`work/ssimarch/an-r3-tube-no-rung-makes-a-graph-is-never-refined.md`).
        test_utils::pinned_red::row(
            geom_core::Tol::witness().eps(),
            &[1e-6],
            "the close rings",
            &[
                "δ 3e-4, counted: March(TubeStraddles { verdict: Zero(Classified { margin: MarginDiag(Value(0.0, None, ZeroWord(0))), band: Band { zero: 1e-6, escalate: 9.999999999999999e-6 } }), boxes: 206 })",
            ],
            close_rings_carry_counted_and_marched_row,
        );
    }

    /// The row [`close_rings_carry_counted_and_marched`] runs, under its pin.
    fn close_rings_carry_counted_and_marched_row() {
        use super::{Components, SsiLane};
        for delta in [3e-2, 1e-2, 3e-3, 1e-3, 3e-4] {
            let (wall, ball) = wall_and_ball((0.1, 0.0, 0.2), 0.6 + delta);
            let section = built(&wall, &ball);
            let marched = super::PairSection {
                components: Components::Marched,
                ..section.clone()
            };
            for (read, s) in [("counted", &section), ("marched", &marched)] {
                let got = s
                    .carry(Some(SsiLane::marched()), band())
                    .unwrap_or_else(|e| panic!("δ {delta:e}, {read}: {e:?}"));
                assert_eq!(got.components().len(), 2, "δ {delta:e}, {read}");
            }
        }
    }

    /// **A count the march disagrees with is a desync**: the two loops of
    /// a sphere reaching past a wall's far ruling, listed as three. As
    /// counted, they carry; read as marched, the march's two.
    #[test]
    #[allow(clippy::unwrap_used)]
    fn a_count_the_march_disagrees_with_refuses() {
        use super::{CarryError, Components, SsiLane};
        let (wall, ball) = wall_and_ball((0.1, 0.0, 0.2), 0.7);
        let section = built(&wall, &ball);
        let lane = Some(SsiLane::marched());
        let listed = section.counted().unwrap().to_vec();
        assert_eq!(listed.len(), 2, "two loops");
        assert!(
            section.carry(lane, band()).is_ok(),
            "as counted, it carries"
        );
        let restated = |components| super::PairSection {
            components,
            ..section.clone()
        };
        let three = restated(Components::Counted(
            listed.iter().chain(&listed[..1]).cloned().collect(),
        ));
        assert!(
            matches!(three.carry(lane, band()), Err(CarryError::Desync(_))),
            "three listed, two marched"
        );
        let marched = restated(Components::Marched).carry(lane, band()).unwrap();
        assert_eq!(marched.components().len(), 2, "marched, the march's two");
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
