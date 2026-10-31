//! **The torus pairs' poses** ([`super`]'s module docs): a ring torus
//! against a sphere, a cylinder or another ring torus, each decided once
//! — the operands guarded, the partner's symmetry read, the critical gap
//! decided ([`super::gap`]) — and the section's components built over the
//! decided pose.
//!
//! # The symmetry reading
//!
//! A sphere is read against the torus axis by its centre's distance from
//! it (`st_coaxial`); a cylinder's or a torus's axis by its tilt to the
//! torus axis and its offset, the section certificate's lever and pivot
//! margins ([`axis_pose`]), and a cylinder not parallel by its tilt from
//! square ([`super::torus_arms::square_pose`]). The lever is read over
//! the torus's own ball, `|x − c| ≤ R + r`, which holds the whole torus
//! and so the whole section (the smaller of the two balls for a torus
//! pair): the value is keyed by surface pair, so no face's extent can
//! enter it, and a ball holding the section is the reach the lever needs.
//!
//! A coaxial partner is a coincidence (D10), recorded; its spine distance
//! is constant, so its gap is the closed-form tube rows, read across the
//! slack the coaxial reading admitted. A sphere off the axis has a
//! closed-form gap too, its centre's two meridian distances from the core
//! ([`SphereTorusPose`]). Every other pose's gap is the sweep, on the
//! operands as they stand, two tori's on either core ([`TorusTorusPose`]).
//! Two tori decided one surface ([`super::torus_coincidence`]) are refused
//! as a coincidence before any other question.
//!
//! # The components
//!
//! Over a transverse gap the symmetric poses count their components in
//! closed form ([`super::torus_arms`]): coaxial parallels as circles, the
//! rest classified with the general rung's carrier
//! ([`ClassCarrier::General`]), which [`super::PairSection::carry`]
//! marches over the pose's ball. A general sphere pose is the sphere
//! arm's; a general cylinder or torus pose has no closed-form count of
//! its components, so its classification is marched
//! ([`Components::Marched`]): its components are the march's, listed
//! when it is carried. Over a tangent gap the components are one tangent
//! component, a point where the pose is a touch.

use geom::{Curve3, Surface};
use geom_core::k_stats::decide_reported;
use geom_core::{
    Band, Circles, Decide, Decided, Indeterminate, Margin, MarginDiag, Point2, Point3, Real, Sign,
    Vec2, Vec3, rounding_charge,
};

use super::gap::{Core, Gap, GapRows, Spine, critical_gap};
use super::torus_arms::{self, ArmRefusal, Pinch, SphereMeridian, SphereParts, SquareWall};
use super::torus_coincidence::{TorusCoincidence, TorusData, tori_coincide};
use super::{
    ClassCarrier, Coincident, ComponentShape, Components, PairPose, PairSection, SectionComponent,
    SectionDecision, TANGENT_COMPONENT, ZeroVerdict, line_tilt, square_part,
};
use crate::ExtentBall;
use crate::intersect::{SectionError, SectionRadius, decide_across_reported};
use crate::torus_convention::TorusRefusal;

// ---------------------------------------------------------------------
// The axis pose
// ---------------------------------------------------------------------

/// **How a partner's axis stands to the torus axis** ([`axis_pose`]).
#[derive(Clone, Copy, Debug)]
pub enum AxisPose<T: Real> {
    /// One axis line: the coaxial reading stands at most `slack` off the
    /// partner anywhere the section can matter; `margin` the offset's
    /// `Zero` verdict.
    Coaxial {
        /// The reading's slack.
        slack: T,
        /// The offset's `Zero` margin.
        margin: MarginDiag,
    },
    /// Parallel, at offset `e` read at the pivot, with `toward` the unit
    /// direction from the torus axis to the partner's: the parallel
    /// reading stands at most `slack` off the partner anywhere the
    /// section can matter.
    Parallel {
        /// The offset.
        e: T,
        /// The unit direction from the torus axis to the partner's.
        toward: Vec3<T>,
        /// The reading's slack.
        slack: T,
    },
    /// The tilt routes the axes parallel, and the offset reads neither
    /// coaxial nor parallel across the reach.
    Undecided(Indeterminate),
    /// Tilted, or the tilt undecided.
    Other,
}

/// The lever of a tilt read about the pivot `at` over `ball`: the
/// ball's farthest distance from it, doubled (the section certificate's
/// module docs, the lever and its pivot).
pub fn lever<T: Real>(at: Point3<T>, ball: ExtentBall<T>) -> T {
    ball.lever_from(at) * T::from_f64(2.0)
}

/// Where the turn onto the torus axis is read: the pivot, the axes' tilt
/// (the sine between them), and the partner axis's offset at the pivot.
#[derive(Clone, Copy, Debug)]
pub struct Turn<T: Real> {
    /// The pivot.
    pub at: Point3<T>,
    /// The sine between the axes.
    pub tilt: T,
    /// The partner axis's offset at the pivot.
    pub offset: T,
}

/// **Decides the axis pose** of the partner axis `(o, d)` against the
/// torus `(c, a)`, over `ball` (the section certificate's module docs,
/// the lever and its pivot, the sum). The tilt, levered from the pivot
/// `at`, only routes. Below it the partner is read as its image under the
/// turn about `at` that takes its axis onto `a`'s direction, which moves
/// it by at most the chord between the axes ([`line_tilt`]) times the
/// ball's farthest distance from `at` anywhere the section can matter:
/// the swing, plus `turn`, how far the arm's own reading moves with the
/// image where it reads a point other than the pivot (zero for a reading
/// that is the image). The offset at the pivot is decided across the
/// reach beside that swing: coaxial on the far row, parallel on the
/// floor.
pub fn axis_pose<T: Decide>(
    (c, a): (Point3<T>, Vec3<T>),
    (o, d): (Point3<T>, Vec3<T>),
    turn: impl FnOnce(&Turn<T>) -> T,
    ball: ExtentBall<T>,
    band: Band,
) -> AxisPose<T> {
    let at = ball.foot_on(o, d);
    let tilt = a.cross(d).norm();
    if !matches!(
        geom_core::k_stats::decide(
            "section_axes_tilt",
            Margin::levered(tilt, lever(at, ball)),
            band
        ),
        Ok(Sign::Zero)
    ) {
        return AxisPose::Other;
    }
    let (perp, e) = square_part(at - c, a);
    let swing = line_tilt(a, d) * ball.lever_from(at)
        + turn(&Turn {
            at,
            tilt,
            offset: e,
        });
    const OFFSET: &str = "section_axes_offset";
    match decide_across_reported([OFFSET, "section_axes_offset_floor"], e, swing, band) {
        Ok((Sign::Zero, margin)) => AxisPose::Coaxial {
            slack: e + swing,
            margin,
        },
        Ok((Sign::Positive, _)) => AxisPose::Parallel {
            e,
            toward: perp / e,
            slack: swing,
        },
        Ok((Sign::Negative, _)) => unreachable!(
            "`{OFFSET}` read a distance's floor Negative: the offset is a norm, and its floor \
             is shrunk toward zero, never past it"
        ),
        Err(diag) => AxisPose::Undecided(diag),
    }
}

// ---------------------------------------------------------------------
// The refusals and the shared data
// ---------------------------------------------------------------------

/// **Which question of a torus pose was left undecided.**
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TorusQuestion {
    /// The symmetry reading: a coaxiality, a tilt or an offset, or
    /// whether two tori are one surface.
    Symmetry,
    /// The critical gap, or a coaxial pose's tube rows.
    Gap,
}

/// **Why a torus pose is not decided.**
#[derive(Clone, Debug, PartialEq)]
pub enum TorusPoseRefusal {
    /// The partner's radius guard: `Ok` a decided `Zero` or `Negative`
    /// radius, `Err` an undecided one.
    Radius {
        /// Whose radius.
        radius: SectionRadius,
        /// The guard's reading.
        verdict: Result<Sign, Indeterminate>,
    },
    /// A torus not shown inside its ring convention.
    Torus(TorusRefusal),
    /// Two tori decided one surface: a coincidence to declare, not a
    /// section.
    Coincident,
    /// A question in the band or poisoned.
    Undecided {
        /// Which question.
        question: TorusQuestion,
        /// Its diagnostics.
        diag: Indeterminate,
    },
}

impl From<TorusPoseRefusal> for SectionError {
    fn from(refusal: TorusPoseRefusal) -> Self {
        match refusal {
            TorusPoseRefusal::Radius {
                radius,
                verdict: Err(diag),
            } => Self::RadiusEscalated { radius, diag },
            TorusPoseRefusal::Radius { radius, .. } => Self::DegenerateOperand {
                what: match radius {
                    SectionRadius::Cylinder => "the cylinder's radius is not definitely positive",
                    SectionRadius::Sphere => "the sphere's radius is not definitely positive",
                },
            },
            TorusPoseRefusal::Torus(t) => Self::DegenerateTorus(t),
            TorusPoseRefusal::Coincident => Self::CoincidentSurfaces,
            TorusPoseRefusal::Undecided { diag, .. } => Self::Escalated(diag),
        }
    }
}

/// A ring torus's data, its axis unit.
#[derive(Clone, Copy, Debug)]
pub struct Ring<T: Real> {
    center: Point3<T>,
    axis: Vec3<T>,
    u_ref: Vec3<T>,
    big_r: T,
    r: T,
}

impl<T: Decide> Ring<T> {
    /// The torus `s` shown inside its ring convention; `None` for another
    /// kind.
    fn of(s: &Surface<T>, band: Band) -> Option<Result<Self, TorusPoseRefusal>> {
        let Surface::Torus {
            center,
            axis,
            major_radius,
            minor_radius,
            u_ref,
        } = *s
        else {
            return None;
        };
        Some(
            TorusRefusal::require(major_radius, minor_radius, band)
                .map_err(TorusPoseRefusal::Torus)
                .map(|()| Self {
                    center,
                    axis: axis / axis.norm(),
                    u_ref,
                    big_r: major_radius,
                    r: minor_radius,
                }),
        )
    }
}

impl<T: Real> Ring<T> {
    /// The torus centre.
    pub fn center(&self) -> Point3<T> {
        self.center
    }

    /// The unit axis.
    pub fn axis(&self) -> Vec3<T> {
        self.axis
    }

    /// `[R, r]`.
    pub fn radii(&self) -> [T; 2] {
        [self.big_r, self.r]
    }

    fn data(self) -> TorusData<T> {
        TorusData {
            center: self.center,
            axis: self.axis,
            major_radius: self.big_r,
            minor_radius: self.r,
        }
    }

    fn core(self) -> Core<T> {
        Core::of(self.center, self.axis, self.u_ref, self.big_r)
    }

    /// The ball holding the torus, and so every section of it.
    pub fn ball(self) -> ExtentBall<T> {
        ExtentBall::of_torus(self.center, self.big_r, self.r)
    }

    /// The size of the torus's coordinates: its centre's distance from
    /// the origin and its reach from it, what a reading's rounding is a
    /// charge of.
    fn size(self) -> T {
        (self.center - Point3::origin()).norm() + self.big_r + self.r
    }

    /// The parallel of radius `rho` at height `z`.
    fn parallel(self, rho: T, z: T) -> Curve3<T> {
        Curve3::Circle {
            center: self.center + self.axis * z,
            axis: self.axis,
            u_ref: geom::square_frame(self.axis, self.u_ref).u_ref,
            radius: rho,
        }
    }
}

/// A partner's radius, guarded definitely positive.
fn radius_guard<T: Decide>(
    name: &'static str,
    radius: SectionRadius,
    value: T,
    band: Band,
) -> Result<(), TorusPoseRefusal> {
    match decide_reported(name, Margin::of(value), band) {
        Ok(Decided {
            sign: Sign::Positive,
            ..
        }) => Ok(()),
        verdict => Err(TorusPoseRefusal::Radius {
            radius,
            verdict: verdict.map(|d| d.sign),
        }),
    }
}

/// The cylinder × torus gap's rows.
pub(super) const CT_GAP: GapRows = GapRows {
    gap: "ct_critical_gap",
    clear: "ct_critical_gap_clear",
    monotone: "ct_critical_gap_monotone",
    smooth: "ct_critical_gap_smooth",
    turn: "ct_critical_gap_turn",
    floor: "ct_critical_gap_floor",
    straddle: "ct_critical_gap_straddle",
};
/// The torus × torus gap's rows.
pub(super) const TT_GAP: GapRows = GapRows {
    gap: "tt_critical_gap",
    clear: "tt_critical_gap_clear",
    monotone: "tt_critical_gap_monotone",
    smooth: "tt_critical_gap_smooth",
    turn: "tt_critical_gap_turn",
    floor: "tt_critical_gap_floor",
    straddle: "tt_critical_gap_straddle",
};
/// The sphere × torus gap's row: the closed form's, read across its
/// rounding.
const ST_GAP: [&str; 2] = ["st_critical_gap", "st_critical_gap_floor"];

/// The sweep's verdict as the pose holds it.
fn swept<T: Decide>(
    ring: Ring<T>,
    spine: Spine<T>,
    radii: [T; 2],
    rows: &GapRows,
    band: Band,
) -> Result<TorusGap<T>, TorusPoseRefusal> {
    match critical_gap(ring.core(), spine, radii, rows, band) {
        Ok(Gap::Transverse) => Ok(TorusGap::Transverse),
        Ok(Gap::Tangent { margin, at }) => Ok(TorusGap::Tangent {
            row: rows.gap,
            margin,
            at: Some(at),
        }),
        Err(diag) => Err(TorusPoseRefusal::Undecided {
            question: TorusQuestion::Gap,
            diag,
        }),
    }
}

/// **The critical gap, decided** (module docs).
#[derive(Clone, Copy, Debug)]
pub enum TorusGap<T: Real> {
    /// Every critical value stands past the band from both levels.
    Transverse,
    /// The surfaces are tangent: the row that decided it, its margin,
    /// and the core point the sweep read it at (none for a coaxial
    /// pose's closed form).
    Tangent {
        /// The deciding row.
        row: &'static str,
        /// Its `Zero` margin.
        margin: MarginDiag,
        /// The core point, where the sweep decided it.
        at: Option<Point3<T>>,
    },
}

impl<T: Real> TorusGap<T> {
    fn tangency(self) -> Option<ZeroVerdict> {
        match self {
            Self::Transverse => None,
            Self::Tangent { row, margin, .. } => Some(ZeroVerdict {
                decision: SectionDecision::Tangency(row),
                margin,
            }),
        }
    }
}

/// A closed-form gap: a row decided across its slack, `Zero` a tangency
/// (at `at`, where one point reads it, else along a parallel).
fn closed_gap<T: Real>(
    row: &'static str,
    (sign, margin): (Sign, MarginDiag),
    at: Option<Point3<T>>,
) -> TorusGap<T> {
    match sign {
        Sign::Zero => TorusGap::Tangent { row, margin, at },
        Sign::Positive | Sign::Negative => TorusGap::Transverse,
    }
}

fn undecided_gap(diag: Indeterminate) -> TorusPoseRefusal {
    TorusPoseRefusal::Undecided {
        question: TorusQuestion::Gap,
        diag,
    }
}

fn undecided_symmetry(diag: Indeterminate) -> TorusPoseRefusal {
    TorusPoseRefusal::Undecided {
        question: TorusQuestion::Symmetry,
        diag,
    }
}

/// An arm's refusal as the value's.
fn arm_refused(refusal: ArmRefusal) -> SectionError {
    match refusal {
        ArmRefusal::Pinch(p) => pinched(p),
        ArmRefusal::Unordered(predicate) => SectionError::Unordered { predicate },
    }
}

fn pinched(p: Pinch) -> SectionError {
    match p.undecided {
        Some(diag) => SectionError::Escalated(diag),
        None => SectionError::Pinched { predicate: p.name },
    }
}

/// Two parallels through the crossings of the tube circle `(R, 0)` of
/// radius `r` with the circle `(x2, z2)` of radius `r2` in the meridian
/// half-plane.
fn crossing_parallels<T: Real>(
    ring: Ring<T>,
    (x2, z2, r2): (T, T, T),
    winds: [bool; 2],
) -> Vec<SectionComponent<T>> {
    let (c1, c2) = (Point2::new(ring.big_r, T::zero()), Point2::new(x2, z2));
    let toward = c2 - c1;
    Circles::new(toward.norm(), ring.r, r2)
        .at(c1, c2)
        .crossings()
        .into_iter()
        .map(|p| SectionComponent {
            shape: ComponentShape::Loop,
            winds,
            witness: None,
            carrier: ClassCarrier::Closed(ring.parallel(p.x, p.y)),
        })
        .collect()
}

/// The one component a tangent pose lists: a point where the arm reads a
/// touch there, else a tangent loop.
fn tangent_component<T: Real>(touch: Option<Point3<T>>, winds: [bool; 2]) -> SectionComponent<T> {
    SectionComponent {
        shape: if touch.is_some() {
            ComponentShape::Point
        } else {
            ComponentShape::Loop
        },
        winds: if touch.is_some() {
            [false, false]
        } else {
            winds
        },
        witness: touch,
        carrier: ClassCarrier::Unminted(TANGENT_COMPONENT),
    }
}

/// An arm's loops, listed in (torus, partner) order, in the order asked.
fn in_order<T: Real>(
    mut loops: Vec<SectionComponent<T>>,
    torus_first: bool,
) -> Vec<SectionComponent<T>> {
    for c in &mut loops {
        c.winds = order(torus_first, c.winds);
    }
    loops
}

/// `[on_torus, on_partner]` in the order asked.
fn order(torus_first: bool, [torus, partner]: [bool; 2]) -> [bool; 2] {
    if torus_first {
        [torus, partner]
    } else {
        [partner, torus]
    }
}

// ---------------------------------------------------------------------
// Sphere × torus
// ---------------------------------------------------------------------

/// **How the sphere stands to the torus**, decided.
#[derive(Clone, Copy, Debug)]
pub enum StStance {
    /// The sphere's centre on the torus axis (`st_coaxial` decided
    /// `Zero`): two parallels, a tangent one, or nothing, by the tube row.
    Coaxial {
        /// The coaxiality's margin: a coincidence its consumer records.
        coaxial: MarginDiag,
        /// `st_coaxial_tube`: the sphere cuts the tube circle.
        tube: (Sign, MarginDiag),
    },
    /// The centre definitely off the axis.
    Apart,
}

/// **The sphere × torus pose, decided once**: the sphere's radius and the
/// torus's ring convention guarded, the sphere's centre read against the
/// torus axis, and the critical gap decided (module docs).
#[derive(Clone, Debug)]
pub struct SphereTorusPose<T: Real> {
    sphere: Surface<T>,
    torus: Surface<T>,
    sphere_first: bool,
    ring: Ring<T>,
    center: Point3<T>,
    radius: T,
    stance: StStance,
    gap: TorusGap<T>,
}

impl<T: Decide> SphereTorusPose<T> {
    /// **Decides the pose of `a` and `b`**, a sphere and a torus in either
    /// order. In order: `st_sphere_radius` and the ring convention;
    /// `st_coaxial`, the centre's distance from the torus axis; then the
    /// coaxial tube row (`st_coaxial_tube`) or the closed-form critical
    /// gap (`st_critical_gap`).
    ///
    /// # Errors
    ///
    /// [`TorusPoseRefusal`]; a pair that is not a sphere and a torus is
    /// `None`.
    pub fn decide(
        a: &Surface<T>,
        b: &Surface<T>,
        band: Band,
    ) -> Option<Result<Self, TorusPoseRefusal>> {
        let (sphere, torus, sphere_first) = match (a, b) {
            (Surface::Sphere { .. }, Surface::Torus { .. }) => (a, b, true),
            (Surface::Torus { .. }, Surface::Sphere { .. }) => (b, a, false),
            _ => return None,
        };
        let &Surface::Sphere { center, radius, .. } = sphere else {
            return None;
        };
        let ring = Ring::of(torus, band)?;
        Some((|| {
            radius_guard("st_sphere_radius", SectionRadius::Sphere, radius, band)?;
            let ring = ring?;
            let q = center - ring.center;
            let (perp, s) = square_part(q, ring.axis);
            let coaxial =
                decide_reported("st_coaxial", Margin::of(s), band).map_err(undecided_symmetry)?;
            let h = q.dot(ring.axis);
            let (stance, gap) = if coaxial.sign == Sign::Zero {
                // The centre's distance from every core point as the
                // coaxial reading has it, and the sphere against the tube
                // circle there, read across the offset `s` the reading
                // dropped: each core point's true distance is within `s`
                // of it.
                let d = Vec2::new(ring.big_r, h).norm();
                let cut = (d + ring.r - radius).min(radius - (d - ring.r).abs());
                let tube = decide_across_reported(
                    ["st_coaxial_tube", "st_coaxial_tube_floor"],
                    cut,
                    s + rounding_charge(ring.size() + radius + d),
                    band,
                )
                .map_err(undecided_gap)?;
                (
                    StStance::Coaxial {
                        coaxial: coaxial.margin,
                        tube,
                    },
                    closed_gap("st_coaxial_tube", tube, None),
                )
            } else {
                (
                    StStance::Apart,
                    sphere_gap(ring, (perp, s, h), radius, band)?,
                )
            };
            Ok(Self {
                sphere: sphere.clone(),
                torus: torus.clone(),
                sphere_first,
                ring,
                center,
                radius,
                stance,
                gap,
            })
        })())
    }

    /// The two surfaces in the order the pose was asked of.
    pub fn operands(&self) -> (&Surface<T>, &Surface<T>) {
        if self.sphere_first {
            (&self.sphere, &self.torus)
        } else {
            (&self.torus, &self.sphere)
        }
    }

    /// The sphere, as given.
    pub fn sphere(&self) -> &Surface<T> {
        &self.sphere
    }

    /// The torus, as given.
    pub fn torus(&self) -> &Surface<T> {
        &self.torus
    }

    /// The torus, guarded.
    pub fn ring(&self) -> Ring<T> {
        self.ring
    }

    /// The sphere's own ball, which holds the section.
    pub fn ball(&self) -> ExtentBall<T> {
        ExtentBall::new(self.center, self.radius)
    }

    /// How the sphere stands.
    pub fn stance(&self) -> StStance {
        self.stance
    }

    /// The critical gap.
    pub fn gap(&self) -> TorusGap<T> {
        self.gap
    }

    /// **The tangency the pose decided**, if any: classification data at
    /// every reader, never a carrier.
    pub fn tangency(&self) -> Option<ZeroVerdict> {
        self.gap.tangency()
    }
}

/// **The sphere × torus gap in closed form**: the sphere's centre stands
/// `ρ = s` from the torus axis and `h` along it, so the core circle's
/// critical distances from it are the two meridian ones, `√((ρ ∓ R)² +
/// h²)`, to the core points in its half-plane and opposite (the sweep's
/// one-variable `D` has its critical points there and nowhere else). The
/// gap is the least of their distances from the levels `r + r_s` and
/// `|r − r_s|`, read across its rounding (the coordinates' size). A
/// `Zero` is read at the core point whose distance it was.
fn sphere_gap<T: Decide>(
    ring: Ring<T>,
    (perp, rho, h): (Vec3<T>, T, T),
    radius: T,
    band: Band,
) -> Result<TorusGap<T>, TorusPoseRefusal> {
    let levels = [ring.r + radius, (ring.r - radius).abs()];
    let of = |d: T| (d - levels[0]).abs().min((d - levels[1]).abs());
    let (near, far) = (
        of(Vec2::new(rho - ring.big_r, h).norm()),
        of(Vec2::new(rho + ring.big_r, h).norm()),
    );
    let gap = near.min(far);
    let size = ring.size() + radius + rho + h.abs();
    let read =
        decide_across_reported(ST_GAP, gap, rounding_charge(size), band).map_err(undecided_gap)?;
    let side = (near - far).select_le_zero(T::one(), T::zero() - T::one());
    let at = ring.center + perp / rho * (ring.big_r * side);
    Ok(closed_gap(ST_GAP[0], read, Some(at)))
}

/// The sphere × torus section over its decided pose.
pub(super) fn sphere_torus<T: Decide>(
    pose: SphereTorusPose<T>,
    band: Band,
) -> Result<PairSection<T>, SectionError> {
    let ring = pose.ring;
    let torus_first = !pose.sphere_first;
    let mut decided = Vec::new();
    let components = match pose.stance {
        StStance::Coaxial { coaxial, tube } => {
            decided.push(ZeroVerdict {
                decision: SectionDecision::Coincidence(Coincident::SphereTorus),
                margin: coaxial,
            });
            let winds = order(torus_first, [true, false]);
            match tube.0 {
                Sign::Positive => {
                    let ca = (pose.center - ring.center).dot(ring.axis);
                    crossing_parallels(ring, (T::zero(), ca, pose.radius), winds)
                }
                Sign::Zero => vec![tangent_component(None, winds)],
                Sign::Negative => Vec::new(),
            }
        }
        StStance::Apart => {
            let meridian = SphereMeridian::of(ring.center, ring.axis, ring.big_r, pose.center);
            let radii = ring.radii();
            match pose.gap {
                TorusGap::Transverse => {
                    let essential = || {
                        let c = SectionComponent {
                            shape: ComponentShape::Loop,
                            winds: order(torus_first, [true, false]),
                            witness: None,
                            carrier: ClassCarrier::General,
                        };
                        vec![c.clone(), c]
                    };
                    match torus_arms::sphere_arm(radii, &meridian, pose.radius, band)
                        .map_err(pinched)?
                    {
                        SphereParts::AroundAxis | SphereParts::AroundTube => essential(),
                        SphereParts::Parts(parts) => in_order(parts, torus_first),
                    }
                }
                TorusGap::Tangent { .. } => {
                    let touch = match torus_arms::sphere_arm(radii, &meridian, pose.radius, band) {
                        Err(tan) if tan.undecided.is_none() => {
                            torus_arms::sphere_touch(tan, radii, &meridian, pose.radius, band)
                        }
                        _ => None,
                    };
                    vec![tangent_component(touch, order(torus_first, [false, false]))]
                }
            }
        }
    };
    decided.extend(pose.tangency());
    Ok(PairSection {
        lever: ring.r,
        pose: PairPose::SphereTorus(pose),
        decided,
        components: Components::Counted(components),
        nodes: Vec::new(),
    })
}

// ---------------------------------------------------------------------
// Cylinder × torus
// ---------------------------------------------------------------------

/// **How the cylinder stands to the torus**, decided.
#[derive(Clone, Copy, Debug)]
pub enum CtStance<T: Real> {
    /// Coaxial: two parallels, a tangent one, or nothing, by the wall row
    /// (`section_torus_coaxial_wall`).
    Coaxial {
        /// The coaxiality's margin: a coincidence its consumer records.
        coaxial: MarginDiag,
        /// The wall row.
        wall: (Sign, MarginDiag),
    },
    /// Parallel axes ([`AxisPose::Parallel`]).
    Parallel {
        /// The offset at the pivot.
        e: T,
        /// The unit direction from the torus axis to the wall's.
        toward: Vec3<T>,
        /// The reading's slack.
        slack: T,
    },
    /// The wall's axis square to the torus axis.
    Square(SquareWall<T>),
    /// Every other pose.
    General,
}

/// **The cylinder × torus pose, decided once** (module docs).
#[derive(Clone, Debug)]
pub struct CylinderTorusPose<T: Real> {
    cylinder: Surface<T>,
    torus: Surface<T>,
    cylinder_first: bool,
    ring: Ring<T>,
    radius: T,
    stance: CtStance<T>,
    gap: TorusGap<T>,
}

impl<T: Decide> CylinderTorusPose<T> {
    /// **Decides the pose of `a` and `b`**, a cylinder and a torus in
    /// either order: `ct_cylinder_radius` and the ring convention; the
    /// axis pose over the torus's ball, then square; then the coaxial wall
    /// row or the critical gap's sweep.
    ///
    /// # Errors
    ///
    /// [`TorusPoseRefusal`]; another pair is `None`.
    pub fn decide(
        a: &Surface<T>,
        b: &Surface<T>,
        band: Band,
    ) -> Option<Result<Self, TorusPoseRefusal>> {
        let (cylinder, torus, cylinder_first) = match (a, b) {
            (Surface::Cylinder { .. }, Surface::Torus { .. }) => (a, b, true),
            (Surface::Torus { .. }, Surface::Cylinder { .. }) => (b, a, false),
            _ => return None,
        };
        let &Surface::Cylinder {
            origin,
            axis,
            radius,
            ..
        } = cylinder
        else {
            return None;
        };
        let ring = Ring::of(torus, band)?;
        Some((|| {
            radius_guard("ct_cylinder_radius", SectionRadius::Cylinder, radius, band)?;
            let ring = ring?;
            let d = axis / axis.norm();
            let line = Spine::Line { origin, axis: d };
            let sweep = || swept(ring, line, [ring.r, radius], &CT_GAP, band);
            let (stance, gap) = match axis_pose(
                (ring.center, ring.axis),
                (origin, d),
                |_| T::zero(),
                ring.ball(),
                band,
            ) {
                AxisPose::Coaxial { slack, margin } => {
                    const WALL: &str = "section_torus_coaxial_wall";
                    let wall = decide_across_reported(
                        [WALL, "section_torus_coaxial_wall_floor"],
                        ring.r - (radius - ring.big_r).abs(),
                        slack,
                        band,
                    )
                    .map_err(undecided_gap)?;
                    (
                        CtStance::Coaxial {
                            coaxial: margin,
                            wall,
                        },
                        closed_gap(WALL, wall, None),
                    )
                }
                AxisPose::Parallel { e, toward, slack } => {
                    (CtStance::Parallel { e, toward, slack }, sweep()?)
                }
                AxisPose::Undecided(diag) => return Err(undecided_symmetry(diag)),
                AxisPose::Other => {
                    match torus_arms::square_pose(
                        ring.center,
                        ring.axis,
                        (origin, d),
                        ring.ball(),
                        band,
                    ) {
                        Some(wall) => (CtStance::Square(wall), sweep()?),
                        None => (CtStance::General, sweep()?),
                    }
                }
            };
            Ok(Self {
                cylinder: cylinder.clone(),
                torus: torus.clone(),
                cylinder_first,
                ring,
                radius,
                stance,
                gap,
            })
        })())
    }

    /// The two surfaces in the order the pose was asked of.
    pub fn operands(&self) -> (&Surface<T>, &Surface<T>) {
        if self.cylinder_first {
            (&self.cylinder, &self.torus)
        } else {
            (&self.torus, &self.cylinder)
        }
    }

    /// The cylinder, as given.
    pub fn cylinder(&self) -> &Surface<T> {
        &self.cylinder
    }

    /// The torus, as given.
    pub fn torus(&self) -> &Surface<T> {
        &self.torus
    }

    /// The torus, guarded.
    pub fn ring(&self) -> Ring<T> {
        self.ring
    }

    /// The torus's ball, which holds the section.
    pub fn ball(&self) -> ExtentBall<T> {
        self.ring.ball()
    }

    /// The cylinder's radius.
    pub fn radius(&self) -> T {
        self.radius
    }

    /// How the cylinder stands.
    pub fn stance(&self) -> CtStance<T> {
        self.stance
    }

    /// The critical gap.
    pub fn gap(&self) -> TorusGap<T> {
        self.gap
    }

    /// The tangency the pose decided, if any.
    pub fn tangency(&self) -> Option<ZeroVerdict> {
        self.gap.tangency()
    }
}

/// The cylinder × torus section over its decided pose.
pub(super) fn cylinder_torus<T: Decide>(
    pose: CylinderTorusPose<T>,
    band: Band,
) -> Result<PairSection<T>, SectionError> {
    let ring = pose.ring;
    let torus_first = !pose.cylinder_first;
    let (c, a, radii, rc) = (ring.center, ring.axis, ring.radii(), pose.radius);
    let mut decided = Vec::new();
    let tangent = |touch| {
        Components::Counted(vec![tangent_component(
            touch,
            order(torus_first, [false, false]),
        )])
    };
    let components = match (pose.stance, pose.gap) {
        (CtStance::Coaxial { coaxial, wall }, _) => {
            decided.push(ZeroVerdict {
                decision: SectionDecision::Coincidence(Coincident::CylinderTorus),
                margin: coaxial,
            });
            let winds = order(torus_first, [true, true]);
            match wall.0 {
                Sign::Positive => {
                    let dr = rc - ring.big_r;
                    let t = ((ring.r - dr.abs()) * (ring.r + dr.abs())).sqrt();
                    Components::Counted(
                        [t, T::zero() - t]
                            .into_iter()
                            .map(|z| SectionComponent {
                                shape: ComponentShape::Loop,
                                winds,
                                witness: None,
                                carrier: ClassCarrier::Closed(ring.parallel(rc, z)),
                            })
                            .collect(),
                    )
                }
                Sign::Zero => Components::Counted(vec![tangent_component(None, winds)]),
                Sign::Negative => Components::Counted(Vec::new()),
            }
        }
        (CtStance::Parallel { e, toward, slack }, TorusGap::Transverse) => {
            let parts =
                torus_arms::parallel_cylinder_arm(c, a, radii, (e, slack), toward, rc, band)
                    .map_err(pinched)?;
            Components::Counted(in_order(parts, torus_first))
        }
        (CtStance::Parallel { e, toward, slack }, TorusGap::Tangent { .. }) => {
            let touch = match torus_arms::parallel_cylinder_arm(
                c,
                a,
                radii,
                (e, slack),
                toward,
                rc,
                band,
            ) {
                Err(tan) => torus_arms::parallel_cylinder_touch(tan, c, (e, toward), rc),
                Ok(_) => None,
            };
            tangent(touch)
        }
        (CtStance::Square(wall), TorusGap::Transverse) => {
            let parts = torus_arms::square_cylinder_arm(c, a, radii, &wall, rc, band)
                .map_err(arm_refused)?;
            Components::Counted(in_order(parts, torus_first))
        }
        (CtStance::Square(_) | CtStance::General, TorusGap::Tangent { .. }) => tangent(None),
        (CtStance::General, TorusGap::Transverse) => Components::Marched,
    };
    decided.extend(pose.tangency());
    Ok(PairSection {
        lever: ring.r,
        pose: PairPose::CylinderTorus(pose),
        decided,
        components,
        nodes: Vec::new(),
    })
}

// ---------------------------------------------------------------------
// Torus × torus
// ---------------------------------------------------------------------

/// **How the second torus stands to the first**, decided.
#[derive(Clone, Copy, Debug)]
pub enum TtStance<T: Real> {
    /// Coaxial and not coincident: two parallels, a tangent one, or
    /// nothing, by the tube rows.
    Coaxial {
        /// The coaxiality's margin: a coincidence its consumer records.
        coaxial: MarginDiag,
        /// The tube rows, the first `Zero` or both definite.
        tube: (Sign, MarginDiag),
    },
    /// Parallel axes ([`AxisPose::Parallel`]).
    Parallel {
        /// The offset at the pivot.
        e: T,
        /// The unit direction from the first axis to the second.
        toward: Vec3<T>,
        /// The reading's slack.
        slack: T,
    },
    /// Every other pose.
    General,
}

/// **The torus × torus pose, decided once** (module docs).
#[derive(Clone, Debug)]
pub struct TorusTorusPose<T: Real> {
    first: Surface<T>,
    second: Surface<T>,
    rings: [Ring<T>; 2],
    ball: ExtentBall<T>,
    stance: TtStance<T>,
    gap: TorusGap<T>,
}

/// The coaxial tube rows: the tubes reach each other, and neither nests
/// in the other.
const TUBE_REACH: &str = "section_torus_coaxial_tube_reach";
const TUBE_NEST: &str = "section_torus_coaxial_tube_nest";

impl<T: Decide> TorusTorusPose<T> {
    /// **Decides the pose of two tori**: both ring conventions; whether
    /// they are one surface ([`tori_coincide`]); the axis pose of the
    /// second over the smaller ball; a coaxial pair's tube rows, or the
    /// critical gap's sweep on either core.
    ///
    /// # Errors
    ///
    /// [`TorusPoseRefusal`]; another pair is `None`.
    pub fn decide(
        a: &Surface<T>,
        b: &Surface<T>,
        band: Band,
    ) -> Option<Result<Self, TorusPoseRefusal>> {
        let (ra, rb) = (Ring::of(a, band)?, Ring::of(b, band)?);
        Some((|| {
            let (r1, r2) = (ra?, rb?);
            match tori_coincide(r1.data(), r2.data(), band) {
                TorusCoincidence::One(_) => return Err(TorusPoseRefusal::Coincident),
                TorusCoincidence::Undecided(diag) => return Err(undecided_symmetry(diag)),
                TorusCoincidence::OffRing(t) => return Err(TorusPoseRefusal::Torus(t)),
                TorusCoincidence::Apart => {}
            }
            // Both balls hold the section: the smaller is the tighter
            // reach, and where the comparison is not decided either
            // serves.
            let ball = match geom_core::k_stats::decide(
                "tt_smaller_ball",
                Margin::of((r2.big_r + r2.r) - (r1.big_r + r1.r)),
                band,
            ) {
                Ok(Sign::Negative) => r2.ball(),
                Ok(Sign::Positive | Sign::Zero) | Err(_) => r1.ball(),
            };
            let (c2, a2) = (r2.center, r2.axis);
            // The centre's height is read off `c2` itself, which the turn
            // onto the torus axis lifts by its station from the pivot
            // times `1 − cos`, at most `sin²`.
            let turn = |t: &Turn<T>| (c2 - t.at).dot(a2).abs() * t.tilt.powi(2);
            let (stance, gap) = match axis_pose((r1.center, r1.axis), (c2, a2), turn, ball, band) {
                AxisPose::Coaxial { slack, margin } => {
                    let z2 = (c2 - r1.center).dot(r1.axis);
                    let tubes = Circles::new(Vec2::new(r2.big_r - r1.big_r, z2).norm(), r1.r, r2.r);
                    let mut tube = None;
                    for (row, m) in [(TUBE_REACH, tubes.reach()), (TUBE_NEST, tubes.internal())] {
                        let read = decide_across_reported([row, floor_of(row)], m, slack, band)
                            .map_err(undecided_gap)?;
                        match (read.0, tube) {
                            (Sign::Zero, _) => {
                                tube = Some((row, read));
                                break;
                            }
                            (Sign::Negative, _) => tube = Some((row, read)),
                            (Sign::Positive, None) => tube = Some((row, read)),
                            (Sign::Positive, Some(_)) => {}
                        }
                    }
                    let (row, tube) = tube.unwrap_or_else(|| unreachable!("two rows were read"));
                    (
                        TtStance::Coaxial {
                            coaxial: margin,
                            tube,
                        },
                        closed_gap(row, tube, None),
                    )
                }
                AxisPose::Parallel { e, toward, slack } => (
                    TtStance::Parallel { e, toward, slack },
                    torus_gap([r1, r2], band)?,
                ),
                AxisPose::Undecided(diag) => return Err(undecided_symmetry(diag)),
                AxisPose::Other => (TtStance::General, torus_gap([r1, r2], band)?),
            };
            Ok(Self {
                first: a.clone(),
                second: b.clone(),
                rings: [r1, r2],
                ball,
                stance,
                gap,
            })
        })())
    }

    /// The two surfaces in the order the pose was asked of.
    pub fn operands(&self) -> (&Surface<T>, &Surface<T>) {
        (&self.first, &self.second)
    }

    /// Both tori, guarded, in order.
    pub fn rings(&self) -> [Ring<T>; 2] {
        self.rings
    }

    /// The smaller torus's ball, which holds the section: the one the
    /// axis pose was read over.
    pub fn ball(&self) -> ExtentBall<T> {
        self.ball
    }

    /// How the second stands to the first.
    pub fn stance(&self) -> TtStance<T> {
        self.stance
    }

    /// The critical gap.
    pub fn gap(&self) -> TorusGap<T> {
        self.gap
    }

    /// The tangency the pose decided, if any.
    pub fn tangency(&self) -> Option<ZeroVerdict> {
        self.gap.tangency()
    }
}

/// A coaxial tube row's floor row.
fn floor_of(row: &'static str) -> &'static str {
    match row {
        TUBE_REACH => "section_torus_coaxial_tube_reach_floor",
        _ => "section_torus_coaxial_tube_nest_floor",
    }
}

/// **The torus × torus gap, read on either core**: the critical values
/// of the distance between two core circles are the same whichever is
/// swept, so each sweep's verdict is a certificate of the pair's, and
/// the two never disagree. The first core is swept against the second;
/// where that escalates, the second against the first, and only both
/// escalating escalates (on the first's reading). The verdict therefore
/// does not depend on the order the pair was asked in: an order whose
/// first sweep decides finds what the other order's sweeps find, one of
/// them deciding.
fn torus_gap<T: Decide>([a, b]: [Ring<T>; 2], band: Band) -> Result<TorusGap<T>, TorusPoseRefusal> {
    swept(a, circle(b), [a.r, b.r], &TT_GAP, band)
        .or_else(|first| swept(b, circle(a), [b.r, a.r], &TT_GAP, band).map_err(|_| first))
}

fn circle<T: Real>(ring: Ring<T>) -> Spine<T> {
    Spine::Circle {
        center: ring.center,
        axis: ring.axis,
        radius: ring.big_r,
    }
}

/// The torus × torus section over its decided pose.
pub(super) fn torus_torus<T: Decide>(
    pose: TorusTorusPose<T>,
    band: Band,
) -> Result<PairSection<T>, SectionError> {
    let [r1, r2] = pose.rings;
    let mut decided = Vec::new();
    let components = match (pose.stance, pose.gap) {
        (TtStance::Coaxial { coaxial, tube }, _) => {
            decided.push(ZeroVerdict {
                decision: SectionDecision::Coincidence(Coincident::TorusTorus),
                margin: coaxial,
            });
            match tube.0 {
                Sign::Positive => {
                    let z2 = (r2.center - r1.center).dot(r1.axis);
                    Components::Counted(crossing_parallels(r1, (r2.big_r, z2, r2.r), [true, true]))
                }
                Sign::Zero => Components::Counted(vec![tangent_component(None, [true, true])]),
                Sign::Negative => Components::Counted(Vec::new()),
            }
        }
        (TtStance::Parallel { e, toward, slack }, TorusGap::Transverse) => {
            let h = (r2.center - r1.center).dot(r1.axis);
            let parts = torus_arms::parallel_torus_arm(
                r1.center,
                r1.axis,
                [r1.radii(), r2.radii()],
                h,
                (e, slack),
                toward,
                band,
            )
            .map_err(pinched)?;
            Components::Counted(parts)
        }
        (TtStance::Parallel { .. } | TtStance::General, TorusGap::Tangent { .. }) => {
            Components::Counted(vec![tangent_component(None, [false, false])])
        }
        (TtStance::General, TorusGap::Transverse) => Components::Marched,
    };
    decided.extend(pose.tangency());
    Ok(PairSection {
        lever: r1.r.min(r2.r),
        pose: PairPose::TorusTorus(pose),
        decided,
        components,
        nodes: Vec::new(),
    })
}
