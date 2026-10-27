//! **The section of two distinct surfaces, as one value** (C5): one
//! constructor that is the only dispatch on kind pairs for the pairs it
//! serves, a pose decided once per pair, and the components every reader
//! of the pair takes.
//!
//! # The value
//!
//! [`pair_section`] reads two surfaces and answers either the section
//! ([`SectionRead::Built`]) or that the kind pair is served elsewhere
//! ([`SectionRead::Elsewhere`], with [`crate::route`]'s arm): its match on
//! kinds is exhaustive with no wildcard arm, so a kind added to
//! [`SurfaceKind`] is a compile-time visit here as it is at `route`.
//!
//! A [`PairSection`] holds:
//!
//! - **`decided`**, every `Zero` verdict that chose the arm, with its
//!   margin ([`ZeroVerdict`]): a coincidence (the sphere's centre on the
//!   cylinder's axis) is a record its consumer keeps (D10), and a
//!   tangency is classification data that refuses toward C7 wherever a
//!   carrier is wanted;
//! - **its components**, each a loop or a point, whether it winds about
//!   each surface's carrier, a witness point, and its carrier at
//!   classification ([`ClassCarrier`]): a closed form, the general rung
//!   (marched on demand by [`PairSection::carry`]), or none (a tangency,
//!   classification only).
//!
//! # Classified, then carried
//!
//! The value is built in two states. [`PairSection`] is the
//! classification and costs no march. [`PairSection::carry`] turns it
//! into a [`CarriedSection`], every component holding a carrier a
//! reader can take a foot on: a closed form as classified, or the
//! general rung's fitted curve with its C2 certificate
//! ([`crate::ssi::SsiCertificate`]). Only `carry` builds a
//! `CarriedSection`, and it refuses where a component has no carrier, so
//! no reader takes a foot on a carrier that does not exist. The march is
//! written at `f64`, and [`SsiLane`] is the door to it: holding one is
//! the statement that the scalar may march ([`SsiLane::marched`]).
//!
//! # Cylinder × sphere
//!
//! The one pair this value serves today. Its pose
//! ([`CylinderSpherePose`]) is decided once, and every reader of the
//! pair reads it: the C5 coaxial arm ([`crate::cylinder_sphere_section`]),
//! the ℝ³ march ([`crate::ssi::cylinder_sphere_ssi`]), and through this
//! value the Boolean's section table.

use geom::{Curve3, Surface, SurfaceKind};
use geom_core::{
    Band, Decide, Decided, Indeterminate, Margin, MarginDiag, Point3, Real, Sign, Vec3,
};

use crate::intersect::{PairRoute, SectionError, SectionRadius, route};
use crate::ssi::{BranchEnd, SsiCertificate, SsiDomain, SsiError};

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
/// so readers agree by construction.
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
    /// The cylinder's axis, unit by the surface's invariant.
    pub axis: Vec3<T>,
    /// The cylinder's reference direction.
    pub u_ref: Vec3<T>,
    /// The sphere centre's foot on the axis.
    pub foot: Point3<T>,
    /// The sphere's centre.
    pub center: Point3<T>,
    /// The cylinder's radius `r`, definitely positive.
    pub r: T,
    /// The sphere's radius `R`, definitely positive.
    pub big_r: T,
    /// The centre's distance `d` from the axis.
    pub offset: T,
    /// How the sphere stands.
    pub stance: CsStance<T>,
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
            Self::stance(origin, axis, center, r, big_r, band).map(|(foot, offset, stance)| Self {
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
            }),
        )
    }

    #[allow(clippy::type_complexity)] // (foot, offset, stance) — one pose
    fn stance(
        origin: Point3<T>,
        axis: Vec3<T>,
        center: Point3<T>,
        r: T,
        big_r: T,
        band: Band,
    ) -> Result<(Point3<T>, T, CsStance<T>), PoseRefusal> {
        for (name, value, radius) in [
            ("cs_cylinder_radius", r, SectionRadius::Cylinder),
            ("cs_sphere_radius", big_r, SectionRadius::Sphere),
        ] {
            match geom_core::k_stats::decide(name, Margin::of(value), band) {
                Ok(Sign::Positive) => {}
                verdict => return Err(PoseRefusal::Radius { radius, verdict }),
            }
        }
        let undecided = |decision| move |diag| PoseRefusal::Undecided { decision, diag };
        let read = |decision: CsDecision, value: T| {
            geom_core::k_stats::decide_reported(decision.name(), Margin::of(value), band)
                .map_err(undecided(decision))
        };
        // The centre's part square to the axis, taken as `a × (w × a)`
        // (`axis` unit by the surface's invariant), which stands square
        // to the axis to the rounding of its own size: `w − a·(a·w)`
        // keeps a component along the axis of `w`'s size times the
        // rounding, which dividing by a small offset turns into a
        // direction off the plane square to the axis.
        let off = axis.cross((center - origin).cross(axis));
        let d = off.norm();
        let foot = center - off;
        let coaxial = read(CsDecision::Coaxial, d)?;
        let stance = if coaxial.sign == Sign::Zero {
            CsStance::Coaxial {
                coaxial: coaxial.margin,
                wall: read(CsDecision::WallReach, big_r - r)?,
            }
        } else {
            let near = read(CsDecision::NearReach, big_r - (d - r).abs())?;
            let far = match near.sign {
                Sign::Positive => Some(read(CsDecision::FarReach, big_r - (d + r))?),
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

    /// **The tangency the pose decided**, if any: the wall reach of a
    /// coaxial pose, or the near or far reach off the axis, decided
    /// `Zero`. Classification data at every reader, never a carrier.
    pub fn tangency(&self) -> Option<ZeroVerdict> {
        let tangent = |decision: CsDecision, d: Decided| {
            (d.sign == Sign::Zero).then_some(ZeroVerdict {
                decision: decision.name(),
                margin: d.margin,
                kind: ZeroKind::Tangency,
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

/// One `Zero` verdict that chose the section's arm.
#[derive(Clone, Copy, Debug)]
pub struct ZeroVerdict {
    /// The decision's K-funnel name.
    pub decision: &'static str,
    /// Its margin.
    pub margin: MarginDiag,
    /// What it means.
    pub kind: ZeroKind,
}

/// Whether a component is a curve or one point.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ComponentShape {
    /// A closed curve.
    Loop,
    /// One point, where the surfaces touch.
    Point,
}

/// A component's carrier as classified.
#[derive(Clone, Debug)]
pub enum ClassCarrier<T: Real> {
    /// A closed form.
    Closed(Curve3<T>),
    /// The general rung's: marched and fitted by [`PairSection::carry`].
    General,
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
    /// One point of it, in closed form, where the arm names one.
    pub witness: Option<Point3<T>>,
    /// Its carrier.
    pub carrier: ClassCarrier<T>,
}

/// What the pair's arm decided its section from.
#[derive(Clone, Debug)]
pub enum SectionPose<T: Real> {
    /// The cylinder × sphere pose.
    CylinderSphere(CylinderSpherePose<T>),
}

/// **The section of two distinct surfaces, classified** (module docs).
#[derive(Clone, Debug)]
pub struct PairSection<T: Real> {
    pose: SectionPose<T>,
    decided: Vec<ZeroVerdict>,
    components: Vec<SectionComponent<T>>,
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
/// the band ([`SectionError::Escalated`]).
pub fn pair_section<T: Decide>(
    a: &Surface<T>,
    b: &Surface<T>,
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
        (Plane, Plane | Cylinder | Cone | Sphere | Torus | Nurbs | Approx)
        | (Cylinder, Plane | Cylinder | Cone | Torus | Nurbs | Approx)
        | (Cone, Plane | Cylinder | Cone | Sphere | Torus | Nurbs | Approx)
        | (Sphere, Plane | Cone | Sphere | Torus | Nurbs | Approx)
        | (Torus, Plane | Cylinder | Cone | Sphere | Torus | Nurbs | Approx)
        | (Nurbs, Plane | Cylinder | Cone | Sphere | Torus | Nurbs | Approx)
        | (Approx, Plane | Cylinder | Cone | Sphere | Torus | Nurbs | Approx) => {
            Ok(SectionRead::Elsewhere(route(ka, kb)))
        }
    }
}

/// **Whether the value builds the section of some kind pair `kind` is a
/// side of** — the filter a reader scanning many pairs applies before it
/// asks [`pair_section`]. Exhaustive, as `pair_section`'s match is.
#[must_use]
pub fn serves(kind: SurfaceKind) -> bool {
    use SurfaceKind::{Approx, Cone, Cylinder, Nurbs, Plane, Sphere, Torus};
    match kind {
        Cylinder | Sphere => true,
        Plane | Cone | Torus | Nurbs | Approx => false,
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
    let ring = |carrier| SectionComponent {
        shape: ComponentShape::Loop,
        winds: order(true, false),
        witness: None,
        carrier,
    };
    let components = match pose.stance {
        CsStance::Coaxial { coaxial, wall } => {
            decided.push(ZeroVerdict {
                decision: CsDecision::Coaxial.name(),
                margin: coaxial,
                kind: ZeroKind::Coincidence,
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
            (Sign::Zero, _) => vec![SectionComponent {
                shape: ComponentShape::Point,
                winds: [false, false],
                witness: Some(pose.nearest_ruling(toward)),
                carrier: ClassCarrier::Unminted(TANGENT_COMPONENT),
            }],
            // One loop, witnessed on the ruling nearest the centre,
            // `√((R − |d − r|)(R + |d − r|))` above the foot.
            (Sign::Positive, Some(Sign::Negative | Sign::Zero) | None) => {
                let q = (pose.offset - pose.r).abs();
                let t = ((pose.big_r - q) * (pose.big_r + q)).sqrt();
                let carrier = match far.map(|f| f.sign) {
                    Some(Sign::Negative) => ClassCarrier::General,
                    _ => ClassCarrier::Unminted(TANGENT_COMPONENT),
                };
                vec![SectionComponent {
                    shape: ComponentShape::Loop,
                    winds: [false, false],
                    witness: Some(pose.nearest_ruling(toward) + pose.axis * t),
                    carrier,
                }]
            }
            (Sign::Positive, Some(Sign::Positive)) => {
                vec![ring(ClassCarrier::General), ring(ClassCarrier::General)]
            }
        },
    };
    decided.extend(pose.tangency());
    PairSection {
        pose: SectionPose::CylinderSphere(pose),
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
}

impl<T: Decide> PairSection<T> {
    /// The pose the arm decided.
    pub fn pose(&self) -> &SectionPose<T> {
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
    /// one: the record its consumer keeps.
    pub fn coincidence(&self) -> Option<ZeroVerdict> {
        self.decided
            .iter()
            .copied()
            .find(|z| z.kind == ZeroKind::Coincidence)
    }

    /// The tangency among [`Self::decided`], where the arm stands on one.
    pub fn tangency(&self) -> Option<ZeroVerdict> {
        self.decided
            .iter()
            .copied()
            .find(|z| z.kind == ZeroKind::Tangency)
    }

    /// **The component the site `p` lies on**, by the classification
    /// alone: a section of one component answers it; two are told apart
    /// by the side of the plane through the sphere centre's foot square
    /// to the cylinder's axis (`cs_component_side`), the first component
    /// on its positive side. Both components of a two-loop section keep
    /// a definite height over that plane, so a `Zero` there is a site on
    /// neither, a desync.
    ///
    /// # Errors
    ///
    /// [`SiteRefusal`]; a section with no component is a desync.
    pub fn component_of(&self, p: Point3<T>, band: Band) -> Result<usize, SiteRefusal> {
        match self.components.len() {
            0 => Err(SiteRefusal::Desync("a site on a section with no component")),
            1 => Ok(0),
            _ => {
                let SectionPose::CylinderSphere(pose) = &self.pose;
                let height = (p - pose.foot).dot(pose.axis);
                match geom_core::k_stats::decide("cs_component_side", Margin::of(height), band) {
                    Ok(Sign::Positive) => Ok(0),
                    Ok(Sign::Negative) => Ok(1),
                    Ok(Sign::Zero) => Err(SiteRefusal::Desync(
                        "a site lies between the two loops of its section",
                    )),
                    Err(diag) => Err(SiteRefusal::Escalated(diag)),
                }
            }
        }
    }

    /// **The section carried** (module docs): every component's carrier,
    /// a closed form as classified or the general rung's, marched through
    /// `lane` over the sphere's own box, which holds every component of a
    /// sphere's section, and fitted with its certificate. The march finds
    /// each component as one closed branch, assigned to the component its
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
        if let Some(why) = self.components.iter().find_map(|c| match c.carrier {
            ClassCarrier::Unminted(why) => Some(why),
            ClassCarrier::Closed(_) | ClassCarrier::General => None,
        }) {
            return Err(CarryError::Unminted(why));
        }
        let mut fitted = if self
            .components
            .iter()
            .any(|c| matches!(c.carrier, ClassCarrier::General))
        {
            let lane = lane.ok_or(CarryError::NoLane { scalar: T::NAME })?;
            let SectionPose::CylinderSphere(pose) = &self.pose;
            let branches = (lane.march)(pose, band).map_err(CarryError::March)?;
            let mut slots: Vec<Option<FittedCarrier<T>>> = vec![None; self.components.len()];
            for branch in branches {
                if !branch.closed {
                    return Err(CarryError::Desync(
                        "the march left the sphere's box, which holds the whole section",
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
                (ClassCarrier::General, None) => Err(CarryError::Desync(
                    "the march found no branch on a component the classification names",
                )),
                (ClassCarrier::Unminted(why), _) => Err(CarryError::Unminted(why)),
            })
            .collect::<Result<_, _>>()?;
        Ok(CarriedSection { components })
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

/// **The general rung's door** (module docs): the ℝ³ march of a
/// cylinder × sphere pose, written at `f64`. Holding one is the
/// statement that the scalar may march; [`SsiLane::marched`] is its one
/// constructor, and it exists at `f64` alone.
#[derive(Clone, Copy, Debug)]
pub struct SsiLane<T: Real> {
    #[allow(clippy::type_complexity)] // one march
    march: fn(&CylinderSpherePose<T>, Band) -> Result<Vec<FittedCarrier<T>>, SsiError>,
}

impl SsiLane<f64> {
    /// The march.
    #[must_use]
    pub fn marched() -> Self {
        Self {
            march: march_over_the_sphere,
        }
    }
}

/// The ℝ³ march of `pose` over the sphere's own box, padded by a hundredth
/// of the radius, with the sphere's diameter as the feature extent.
fn march_over_the_sphere(
    pose: &CylinderSpherePose<f64>,
    band: Band,
) -> Result<Vec<FittedCarrier<f64>>, SsiError> {
    let domain = SsiDomain {
        center: pose.center,
        half_extent: pose.big_r * 1.01,
        extent: pose.big_r * 2.0,
        floor_scale: 1.0,
    };
    let out = crate::ssi::march_cylinder_sphere(pose, domain, band)?;
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
