//! **The curve × face pose** ([`super::pose()`]): how a span sits against a
//! surface before any root is found, decided once, here, for every door
//! (C5's curve × face paragraph, `crates/geom-brep/README.md`).
//!
//! A door's pose rungs decide the CLASS of the pair: the span lies on
//! the surface ([`Pose::Lies`]), is apart from it ([`Pose::Apart`]),
//! touches it at one nondegenerate double root ([`Pose::Tangent`]),
//! passes through a singular point of it ([`Pose::Through`]), or meets
//! it at isolated roots its door's one method places
//! ([`Pose::Isolated`]). Every rung is a `curve_face_pose_*` row decided
//! in this module, and no door decides one: a door keeps only its
//! method, which reads the [`Form`] the pose computed and re-decides
//! nothing of it.
//!
//! - **Not `Lies` before `Isolated`.** An arm answers `Isolated` only once
//!   a rung has excluded `Lies`: a definite discriminant, a definite
//!   swing, a definite second harmonic, or, where no rung does, one
//!   carrier point whose residual is decided past the band
//!   ([`PoseRung::NotLies`]).
//! - **`Tangent` is decided on the relative curvature** across the double
//!   root ([`PoseRung::Kappa`]): definitely positive is a nondegenerate
//!   double root, and in the zero band is a higher-order contact, which
//!   escalates ([`PoseRefusal::InBand`], the gate's rejection of the
//!   zero reading, [`zero_band`]). It is read only where it decides: a
//!   double root past the span whose reach stands clear of it leaves the
//!   span `Apart`. Its arc ([`Touch::arc`]) encloses every parameter on
//!   the span, or between it and the touch, whose point is within the
//!   band of the surface, and never understates it there.
//! - **A singular point is not a meeting.** A cone's apex on the span is
//!   [`Pose::Through`]: the pose's datum, which a consumer reads before
//!   any meeting.

use geom::Conic;
use geom_core::k_stats::{decide, decide_nonzero, straddle};
use geom_core::{Band, Decide, Indeterminate, Margin, NonzeroSign, Point3, Real, Rounded, Sign};

use super::{Door, Route, Undecided, route};
use crate::curve_pair::Span;

/// **How a span sits against a surface** ([`super::pose()`]; module docs).
#[derive(Clone, Copy, Debug)]
pub enum Pose<T: Real> {
    /// The span lies on the surface over its whole length: a reading of
    /// values only.
    Lies,
    /// The span and the surface do not meet.
    Apart,
    /// The span touches the surface at one nondegenerate double root
    /// ([`Touch`]), the pose's decided special case of an isolated
    /// meeting.
    Tangent(Touch<T>),
    /// The span passes through a singular point of the surface, a cone's
    /// apex ([`Singular`]).
    Through(Singular<T>),
    /// The span meets the surface at isolated roots, which the door's
    /// method finds from what the pose computed ([`Form`]).
    Isolated(Form<T>),
}

/// **A double root the pose decided** ([`Pose::Tangent`]).
#[derive(Clone, Copy, Debug)]
pub struct Touch<T: Real> {
    /// The root's parameter on the carrier, within half a turn of the
    /// span's middle on a conic.
    pub at: T,
    /// Its point.
    pub point: Point3<T>,
    /// An arc of carrier parameters about `at` that holds every
    /// parameter on the span, or between the span and `at`, whose point
    /// is within the band of the surface: it never understates that arc
    /// there. Past both (a line × cone's arc is read through the run's
    /// reach from the apex) it claims nothing.
    pub arc: (T, T),
    /// The same arc at the band's escalation `Kε`: every parameter there
    /// whose point is within `Kε` of the surface. A touch past the span
    /// is not `Apart` only where this reach covers part of the span, and
    /// that part is all it says anything of.
    pub reach: (T, T),
}

/// **A singular point of the surface on the span** ([`Pose::Through`]).
#[derive(Clone, Copy, Debug)]
pub struct Singular<T: Real> {
    /// The point's parameter on the carrier.
    pub at: T,
    /// The singular point itself (a cone's apex).
    pub point: Point3<T>,
    /// The carrier parameters about `at` within which every root of the
    /// residual near the point lies.
    pub spread: (T, T),
}

/// **What the pose computed for the door's method** ([`Pose::Isolated`]):
/// private to the arms, so the method reads it without re-deciding it.
#[derive(Clone, Copy, Debug)]
pub struct Form<T: Real>(pub(super) Formed<T>);

/// The arms' forms ([`Form`]).
#[derive(Clone, Copy, Debug)]
pub(super) enum Formed<T: Real> {
    /// A line across a plane: its one root.
    PlaneLine { root: T },
    /// A conic across a plane: its two roots, each within half a turn of
    /// the span's middle.
    PlaneConic { roots: [T; 2] },
    /// A line across a wall or a sphere: its two roots and a bound on
    /// either's rounding in the line's parameter.
    LineQuadratic { roots: [T; 2] },
    /// A line against a cone, off the apex: crossing it, or, `parallel`,
    /// parallel to a generator, which the method cannot read.
    LineCone { parallel: bool },
    /// A line against a torus, which it never lies on.
    LineTorus,
    /// A conic on the first-harmonic lane, crossing: its extremes read
    /// definitely on either side.
    FirstHarmonic(super::circle_roots::FirstHarmonic<T>),
    /// A conic on the ladder lane, `Lies` excluded.
    Ladder,
    /// A circle against a torus whose axis it is parallel to, off the
    /// axis, its extremes placed.
    TorusParallel {
        far: super::circle_torus::Band3,
        near: super::circle_torus::Band3,
    },
    /// A conic against a torus on the general walk, `Lies` excluded, and
    /// the ring's floor on `|F|` per metre.
    TorusWalk { floor: T },
    /// A spiric across a plane not parallel to its own: the line where
    /// the two planes meet, a unit direction from its point nearest the
    /// spiric's foot ([`super::spiric_plane`]), `across`, the unit in
    /// the spiric's plane square to the line, and `sine`, the planes' tilt
    /// sine as the line's own form reads it, `√(α² + β²)`: a move `d`
    /// along `across` is a height `d·sine` off the plane. It is the
    /// `|û × n̂|` the parallel rung decides to a rounding, not that value.
    SpiricPlane {
        line: geom::Line<T>,
        across: geom_core::Vec3<T>,
        sine: T,
    },
}

/// **Which pose rung read in the band** ([`PoseRefusal`]).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PoseRung {
    /// Whether a conic's plane is parallel to the plane, and its offset.
    PlaneParallel,
    /// Whether a conic reaches a plane, grazes it or misses it.
    BellyGraze,
    /// Whether a line runs parallel to a wall's axis or to a plane over
    /// the span, and its offset.
    AxisParallel,
    /// Whether a line crosses a wall or a sphere, grazes it or misses it.
    Discriminant,
    /// A first harmonic's noise meter.
    Noise,
    /// Whether a residual is constant along the carrier, and its value.
    Coaxial,
    /// A first harmonic's extremes.
    Extreme,
    /// The relative curvature across a double root.
    Kappa,
    /// Whether the span reaches a cone's apex.
    Apex,
    /// The spread of the residual's roots about a cone's apex the span
    /// reaches.
    ApexSpread,
    /// Whether a line turns off a cone's generator over the span, and,
    /// along one, its run's whole reach through the cone's floor
    /// (`curve_face_pose_cone_on_surface`), whose gap refuses here.
    ConeLead,
    /// Whether a circle is a torus's meridian.
    Meridian,
    /// A torus's contour reached by a circle parallel to its axis.
    Contour,
    /// One carrier point's residual decided past the band.
    NotLies,
    /// Whether a spiric span is in its two-oval convention,
    /// `0 < |offset| < R − r`.
    SpiricRegime,
}

/// **Why [`super::pose()`] gave no class**, the why of its window
/// ([`Undecided`]). A graze rung in band ([`PoseRung::BellyGraze`],
/// [`PoseRung::Discriminant`] on a wall or a sphere, [`PoseRung::Extreme`],
/// [`PoseRung::Kappa`]) is a window over the graze's reach at the band's
/// escalation `Kε` about its extreme, clipped to the span; a line's
/// [`PoseRung::Apex`] or [`PoseRung::ApexSpread`] in band is a window over
/// the roots' spread about the apex, widened by `Kε`. Every other refusal,
/// and a conic's apex rungs (whose residual has roots the spread does not
/// hold), is one window over the whole span.
#[derive(Clone, Copy, Debug)]
pub enum PoseRefusal {
    /// A rung's reading fell in the band: in its escalation gap, or in
    /// its zero band where the class needs it definite (a higher-order
    /// contact, a carrier no point of which reads off the surface), whose
    /// `diag` is the gate's rejection of the zero reading
    /// ([`PoseRefusal::zero_band`]).
    InBand {
        /// The rung.
        rung: PoseRung,
        /// Its diagnostics.
        diag: Indeterminate,
    },
    /// A meter the rung reads refused: the reading's own error is not
    /// definitely inside the band. No margin says how far it was.
    Meter {
        /// The rung.
        rung: PoseRung,
        /// The meter's row.
        row: &'static str,
    },
    /// A spline arm: no closed pose, its enclosures are the method.
    Enclosed,
    /// An arm the table has not built ([`Route::implemented`]).
    Unbuilt(Route),
    /// A torus outside the ring convention, read before any pose.
    Torus(crate::TorusRefusal),
    /// A carrier or surface the arm does not read, or readings that
    /// contradict: a kernel fault.
    Fault(&'static str),
}

impl PoseRefusal {
    /// A rung's escalation.
    pub(super) const fn in_band(rung: PoseRung) -> impl Fn(Indeterminate) -> Self {
        move |diag| Self::InBand { rung, diag }
    }

    /// Whether a rung read in its zero band where the class needs it
    /// definite: its refusal is the gate's rejection of a zero reading,
    /// no margin of a gap.
    #[must_use]
    pub fn zero_band(&self) -> bool {
        matches!(self, Self::InBand { diag, .. } if diag.margin.rejected_sign() == Some(Sign::Zero))
    }

    /// **What a zero-band refusal is, in the root doors' shape** (the
    /// sweep's one map from `CrossWhy`, and the door rows' test readers): a span no point of which reads off the surface
    /// ([`PoseRung::NotLies`]) is [`crate::roots::Unsettled::Degenerate`],
    /// its residual read zero along it with nothing deciding it lies; any
    /// other rung's zero reading is a graze the class cannot call,
    /// [`crate::roots::Unsettled::Tangency`]. `None` for a refusal that
    /// is not a zero-band one.
    #[must_use]
    pub fn unsettled(&self) -> Option<crate::roots::Unsettled> {
        match self {
            Self::InBand {
                rung: PoseRung::NotLies,
                ..
            } if self.zero_band() => Some(crate::roots::Unsettled::Degenerate),
            _ if self.zero_band() => Some(crate::roots::Unsettled::Tangency),
            _ => None,
        }
    }
}

/// **A rung read in its zero band where the class needs it definite**:
/// the gate's rejection of the zero reading `decided`
/// ([`geom_core::k_stats::gate_rejection`]), passing only a definite
/// sign.
fn zero_band(
    rung: PoseRung,
    name: &'static str,
    decided: geom_core::Decided,
    band: Band,
) -> PoseRefusal {
    PoseRefusal::InBand {
        rung,
        diag: geom_core::k_stats::gate_rejection(
            name,
            decided,
            band,
            geom_core::SizedPass::NonZero,
        ),
    }
}

/// **An arm's refusal, and the stretch of the span it confines**: `None`
/// is the whole span ([`PoseRefusal`]'s rung list).
pub(super) struct Refused<T> {
    pub(super) why: PoseRefusal,
    window: Option<(T, T)>,
}

impl<T> From<PoseRefusal> for Refused<T> {
    fn from(why: PoseRefusal) -> Self {
        Self { why, window: None }
    }
}

/// **The pose of `span` against `surface`** ([`super::pose()`]): the arm
/// [`route`] names, read by its rungs.
///
/// # Errors
///
/// [`Undecided`]: the stretch of the span not read, ascending, and its
/// [`PoseRefusal`].
pub fn pose<T: Decide>(
    span: &Span<'_, T>,
    surface: &geom::Surface<T>,
    band: Band,
) -> Result<Pose<T>, Undecided<T, PoseRefusal>> {
    let carrier = span.carrier();
    let way = route(carrier.kind(), surface.kind());
    let read = if way.implemented {
        match way.door {
            Door::PlaneCrossing => plane(span, surface, band),
            Door::LineRoots => line(span, surface, band),
            Door::ConicQuadric => conic_quadric(span, surface, band),
            Door::ConicTorus => conic_torus(span, surface, band).map_err(Refused::from),
            Door::SpiricPlane => spiric_plane(span, surface, band).map_err(Refused::from),
            Door::SplineComposite | Door::SplineFace => Err(PoseRefusal::Enclosed.into()),
            Door::Spiric | Door::Sheet => Err(PoseRefusal::Unbuilt(way).into()),
        }
    } else {
        Err(PoseRefusal::Unbuilt(way).into())
    };
    read.map_err(|Refused { why, window }| Undecided {
        window: window.unwrap_or_else(|| super::ascending(span.params())),
        why,
    })
}

// ------------------------------------------------------------- the cull

/// **`Apart`'s first stage on the conic arms: the ball cull**
/// (`curve_face_pose_clearance`). `true` where the span is decided
/// apart from `surface` past the band; `false` hands on to the arm's
/// rungs, in band and short of clear alike, never refusing alone.
///
/// The span lies in the ball about its middle point `m` whose radius `ℓ`
/// is the span's metered reach ([`geom_core::bisect::reach`]), capped at
/// the conic's diameter (any two of its points lie within twice its
/// larger semi-axis). Each kind has a signed function `g`, 1-Lipschitz
/// in space up to the stated constant `L`, whose sign is the residual's
/// ([`crate::implicit_residual`]), so over the ball `g` keeps the sign of
/// `g(m)` wherever `|g(m)| > L·ℓ`, and the span meets no point of the
/// surface. The decision reads `|g(m)| − L·ℓ`, in metres:
///
/// - **sphere**: `|p − S| − R`, the distance to the centre less a
///   constant: 1-Lipschitz, and the residual `(|p − S|² − R²)/2R` is it
///   times the positive `(|p − S| + R)/2R`.
/// - **cylinder**: `ρ − R`, `ρ` the distance to the axis line: 1-Lipschitz
///   as a distance to a set, `n` unit by type (`UnitVec3`), and the
///   residual `(ρ² − R²)/2R` is it times a positive factor.
/// - **cone**: `ρ cos α − |h| sin α`, the residual itself
///   ([`crate::cone_elevation`] on the double cone): the smaller of the
///   two forms `ρ cos α ∓ h sin α`, each with gradient
///   `cos α ∇ρ ∓ sin α ∇h`, `∇ρ ⊥ ∇h` unit off the axis, so of norm
///   `L = √(sin²α + cos²α)` at the stored angle's computed pair; a
///   minimum of `L`-Lipschitz forms is `L`-Lipschitz, both nappes and the
///   axis included (`ρ` and `|h|` are 1-Lipschitz there too).
/// - **torus**: `√((ρ − R)² + h²) − r`, the distance to the spine circle
///   less the tube radius: 1-Lipschitz as a distance to a set, and the
///   residual `((ρ − R)² + h² − r²)/2r` is it times the positive
///   `(√(·) + r)/2r`. Nothing reads `R > r`, so a spindle (`r ≥ R`,
///   whose zero set is the same `√(·) = r`) is culled the same way.
///
/// **Re-posed** (standing claim 1): `m` is the conic's running point at
/// the span's middle ([`Conic::running_point`]), never its centre, and
/// every quantity is carried from `m − origin`, the surface's stored
/// origin taken exact, as running [`Rounded`] values, with no `c₀ ± A`
/// extremes and no charge: a far-centred pair keeps the digits of its
/// gap that the stored data has, and the reading's own rounding is the
/// spread the band reads (`spread_readings`), so a gap inside it hands
/// on.
fn culled<T: Decide>(
    span: &Span<'_, T>,
    conic: &dyn Conic<T>,
    surface: &geom::Surface<T>,
    band: Band,
) -> bool {
    use geom_core::running::{cross, dot, exact_vec};
    let (t0, t1) = span.params();
    let speed = geom::carrier_speed::speed_range(span.carrier()).1;
    let (a, b) = conic.semi_axes();
    let diameter = T::from_f64(2.0) * a.abs().max(b.abs());
    let reach = geom_core::bisect::reach(speed, (t0, t1)).min(diameter);
    let m = conic.running_point(geom::mid_param(t0, t1));
    let from = |origin: Point3<T>| {
        let o = exact_vec(geom_core::Vec3::new(origin.x, origin.y, origin.z));
        [m[0] - o[0], m[1] - o[1], m[2] - o[2]]
    };
    let length = |v: [Rounded<T>; 3]| v[0].hypot(v[1]).hypot(v[2]);
    // `m`'s height along the unit axis `n` and its distance from the axis
    // line, `|e × n|`: no `e − (e·n)n`, which cancels far along the axis.
    let split = |origin: Point3<T>, n: geom_core::UnitVec3<T>| {
        let e = from(origin);
        let n = exact_vec(n.get());
        (dot(e, n), length(cross(e, n)))
    };
    let magnitude = |v: Rounded<T>| Rounded::select_le_zero(v.value, -v, v);
    let exact = Rounded::exact;
    let one = exact(T::one());
    let (g, lipschitz) = match *surface {
        geom::Surface::Sphere { frame, radius, .. } => {
            (length(from(frame.origin())) - exact(radius), one)
        }
        geom::Surface::Cylinder { frame, radius, .. } => {
            let (_, rho) = split(frame.origin(), frame.axis_unit());
            (rho - exact(radius), one)
        }
        geom::Surface::Cone {
            frame, half_angle, ..
        } => {
            let (h, rho) = split(frame.origin(), frame.axis_unit());
            let (sin, cos) = half_angle.sin_cos();
            (
                rho * exact(cos) - magnitude(h) * exact(sin),
                exact(sin).hypot(exact(cos)),
            )
        }
        geom::Surface::Torus {
            frame,
            major_radius,
            minor_radius,
            ..
        }
        | geom::Surface::Spindle {
            frame,
            major_radius,
            minor_radius,
            ..
        } => {
            let (h, rho) = split(frame.origin(), frame.axis_unit());
            (
                (rho - exact(major_radius)).hypot(h) - exact(minor_radius),
                one,
            )
        }
        _ => return false,
    };
    let margin = magnitude(g) - exact(reach) * lipschitz;
    matches!(
        crate::roots::spread_readings("curve_face_pose_clearance", margin.value, margin.error, band),
        Ok((below, _)) if below.sign == Sign::Positive
    )
}

// ------------------------------------------------------------- shared

/// **The axis-parallel rung's drift** (`curve_face_pose_axis_parallel`):
/// how far a line's residual moves over the span, a length. `true` in the
/// zero band, where the residual reads as one value ([`axis_offset`]);
/// `false` past the band, where the line is not parallel and the
/// caller's next rung reads it.
///
/// # Errors
///
/// The rung's diagnostics in band ([`PoseRung::AxisParallel`]).
pub(super) fn axis_drift<T: Decide>(drift: Margin<T>, band: Band) -> Result<bool, Indeterminate> {
    Ok(
        match decide("curve_face_pose_axis_parallel", drift, band)? {
            Sign::Zero => true,
            Sign::Positive | Sign::Negative => false,
        },
    )
}

/// **The axis-parallel rung's offset** (C5's "decided like
/// `curve_pair_pose_parallel`", `curve_face_pose_axis_offset`): a line
/// whose drift read zero, its residual `at_mid` at the span's middle to
/// within `half_drift` everywhere on the span. `Lies` where that is in
/// the zero band all round, `Apart` where it is definitely one-signed.
///
/// # Errors
///
/// [`PoseRung::AxisParallel`] in band, the two ends' straddle.
fn axis_offset<T: Decide>(at_mid: T, half_drift: T, band: Band) -> Result<Pose<T>, PoseRefusal> {
    constant_residual_as(
        "curve_face_pose_axis_offset",
        PoseRung::AxisParallel,
        (at_mid, half_drift),
        band,
    )?
    .pose(PoseRung::AxisParallel)
}

/// **A residual read as one value** on `name`: `value` to within `spread`
/// everywhere on the span ([`crate::roots::spread_readings`]).
#[derive(Clone, Copy, Debug)]
enum Constant {
    /// Definitely one-signed: the span and the surface do not meet.
    Apart,
    /// In the zero band all round: the span lies on the surface.
    Lies,
    /// Any other reading, a straddle, with its diagnostics.
    Straddle(Indeterminate),
}

/// The one reading of a constant residual ([`Constant`]) decided as
/// `name`, the spread's gap refusing on `rung`.
fn constant_residual_as<T: Decide>(
    name: &'static str,
    rung: PoseRung,
    (value, spread): (T, T),
    band: Band,
) -> Result<Constant, PoseRefusal> {
    let (below, above) = crate::roots::spread_readings(name, value, spread, band)
        .map_err(PoseRefusal::in_band(rung))?;
    Ok(match (below.sign, above.sign) {
        (Sign::Positive, _) | (_, Sign::Negative) => Constant::Apart,
        (Sign::Zero, Sign::Zero) => Constant::Lies,
        _ => Constant::Straddle(straddle(name, below, above, band)),
    })
}

impl Constant {
    /// The class, a straddle refusing on `rung`.
    fn pose<T: Real>(self, rung: PoseRung) -> Result<Pose<T>, PoseRefusal> {
        match self {
            Self::Apart => Ok(Pose::Apart),
            Self::Lies => Ok(Pose::Lies),
            Self::Straddle(diag) => Err(PoseRefusal::InBand { rung, diag }),
        }
    }
}

/// **A double root an arm read** in the zero band ([`kappa`]).
#[derive(Clone, Copy, Debug)]
struct Graze<T: Real> {
    /// Its parameter on the carrier, within half a turn of the span's
    /// middle on a conic.
    at: T,
    /// The arm's form there (in band), and the bound on its error.
    value: T,
    noise: T,
    /// The magnitude of the form's second derivative in the carrier's
    /// parameter there.
    bend: T,
    /// The carrier's speed there.
    speed: T,
    /// The carrier's own scale at the touch, the lever the curvature is
    /// read over: a conic's diameter there (twice its speed, over which
    /// the sagitta is the residual's whole swing `2A₁` above the touch),
    /// a wall's or a sphere's radius, the touch's distance from a cone's
    /// apex.
    scale: T,
    /// Whether the form is exactly `value + bend·(1 − cos(t − at))` in the
    /// carrier's angle, a first harmonic with no error term (a conic's
    /// height above a plane): its in-band arc is read in closed form
    /// ([`arc`]).
    harmonic: bool,
    /// Whether the carrier's parameter is an angle (a conic's), so no arc
    /// about the root holds more than half a turn either way.
    turn: bool,
    /// Whether the root is the residual's, where the arm's form is not
    /// the residual (a cone's `F`, read through its floor). Read only
    /// once the root is known to reach the span ([`Reaching::confirmed`]):
    /// past it a definite form, whose sign is the residual's, is read
    /// alone.
    confirm: Result<(), PoseRefusal>,
}

/// A [`Graze`] whose reach covers part of the span ([`reaches`]): the
/// only way to its confirmation.
struct Reaching<T: Real>(Graze<T>, (T, T));

/// A [`Reaching`] graze the arm confirmed the residual's: the only thing
/// the curvature is read on ([`curvature`]).
struct Confirmed<T: Real>(Graze<T>, (T, T));

impl<T: Real> Reaching<T> {
    /// The arm's confirmation, read now that the root reaches the span.
    fn confirmed(self) -> Result<Confirmed<T>, PoseRefusal> {
        self.0.confirm?;
        Ok(Confirmed(self.0, self.1))
    }
}

/// The arc about `g.at` holding every parameter whose form is within
/// `width` of zero: `(π/2)·√(2z/bend)`, `z` that width plus the extreme's
/// own reading. A form that falls from its extreme no faster than a first
/// harmonic of that curvature leaves the band no nearer than that
/// (`1 − cos ψ ≥ 2ψ²/π²`), and a quadratic's exactly there. A form that
/// is exactly that first harmonic ([`Graze::harmonic`]) leaves the band at
/// `2·asin(√(z/2·bend))`, read in closed form with no subtraction. Only an
/// angle's arc is held to half a turn ([`Graze::turn`]): a line's runs as
/// far as its quadratic does.
fn arc<T: Real>(g: &Graze<T>, width: T) -> T {
    let z = width + g.value.abs() + g.noise;
    if g.harmonic {
        let two = T::from_f64(2.0);
        return two * (z / (two * g.bend.abs())).min(T::one()).sqrt().asin();
    }
    let quadratic =
        T::from_f64(core::f64::consts::FRAC_PI_2) * (T::from_f64(2.0) * z / g.bend.abs()).sqrt();
    if g.turn {
        quadratic.min(T::pi())
    } else {
        quadratic
    }
}

/// **The relative-curvature rung** at a double root `g` the arm read in
/// the zero band: where it is ([`reaches`]), the arm's confirmation
/// ([`Reaching::confirmed`]), then its curvature ([`curvature`]).
///
/// # Errors
///
/// [`PoseRung::Kappa`] in band, from [`reaches`] or [`curvature`]; the
/// arm's own refusal where it could not confirm the root.
/// Every refusal is a window over the reach ([`graze_reach`]).
fn kappa<T: Decide>(span: &Span<'_, T>, g: Graze<T>, band: Band) -> Result<Pose<T>, Refused<T>> {
    let local = |why| Refused {
        why,
        window: local(span, graze_reach(&g, band), band),
    };
    match reaches(span, g, band).map_err(local)? {
        Ok(reaching) => reaching
            .confirmed()
            .and_then(|confirmed| curvature(span, confirmed, band))
            .map_err(local),
        Err(apart) => Ok(apart),
    }
}

/// **A refusal's window**, where it meets the span ([`super::on_span`])
/// in one piece: one that meets it in two, across the turn's seam, or
/// that the reading has no piece of, is the whole span's.
fn local<T: Decide>(span: &Span<'_, T>, window: (T, T), band: Band) -> Option<(T, T)> {
    match super::on_span(span, window, band)[..] {
        [piece] => Some(piece),
        _ => None,
    }
}

/// **A graze's reach**: `arc(Kε)` about it. A rung that read `g` has
/// excluded `Lies`, so outside the reach its form, and the residual, is
/// past `Kε`: no meeting lies there.
fn graze_reach<T: Real>(g: &Graze<T>, band: Band) -> (T, T) {
    let r = arc(g, T::from_f64(band.escalate()));
    (g.at - r, g.at + r)
}

/// **A graze rung read in its gap** at the extreme `g`, refusing `why`:
/// `Apart` where the reach definitely misses the span ([`reaches`]),
/// otherwise a window over the reach, clipped to the span.
fn graze_window<T: Decide>(
    span: &Span<'_, T>,
    g: Graze<T>,
    why: PoseRefusal,
    band: Band,
) -> Result<Pose<T>, Refused<T>> {
    match reaches(span, g, band) {
        Ok(Err(apart)) => Ok(apart),
        Ok(Ok(_)) | Err(_) => Err(Refused {
            why,
            window: local(span, graze_reach(&g, band), band),
        }),
    }
}

/// **Where the root is**: its reach `arc(Kε)`, the one arc the touch is
/// read by against the span. Inside the span or at an end
/// (`Span::place_param`; an endless span has none), it reaches the span.
/// Past it, the reach's overlap with the span, `min(t₁, at + r) −
/// max(t₀, at − r)` metered at the span's least speed
/// (`curve_face_pose_graze_reach`): definitely negative, every span point
/// stands past `Kε` and the span is `Apart` (`Err`), the curvature
/// deciding nothing; definitely positive, the touch reaches that part of
/// the span; in the zero band, the overlap is the band's to call and
/// refuses.
///
/// # Errors
///
/// [`PoseRung::Kappa`] in band: the root's place against an end, or the
/// overlap, in the gap or in the zero band.
fn reaches<T: Decide>(
    span: &Span<'_, T>,
    g: Graze<T>,
    band: Band,
) -> Result<Result<Reaching<T>, Pose<T>>, PoseRefusal> {
    let in_band = PoseRefusal::in_band(PoseRung::Kappa);
    let reach = graze_reach(&g, band);
    if span.place_param(g.at, band).map_err(&in_band)? == crate::curve_pair::Place::Past {
        // The reach's overlap with the span, `h + r − |at − m|` where it
        // is partial (`m`, `h` the span's middle and half-length): largest
        // at the copy of the reach nearest the middle, which is where a
        // conic's graze is reported (`Period::near`), so no other turn of
        // it meets the span where this one misses.
        let (t0, t1) = super::ascending(span.params());
        let overlap = t1.min(reach.1) - t0.max(reach.0);
        return match geom_core::k_stats::decide_reported(
            "curve_face_pose_graze_reach",
            Margin::metered(overlap, span.speeds().0),
            band,
        ) {
            Ok(read) if read.sign == Sign::Positive => Ok(Ok(Reaching(g, reach))),
            Ok(read) if read.sign == Sign::Zero => Err(zero_band(
                PoseRung::Kappa,
                "curve_face_pose_graze_reach",
                read,
                band,
            )),
            Ok(_) => Ok(Err(Pose::Apart)),
            Err(diag) => Err(in_band(diag)),
        };
    }
    Ok(Ok(Reaching(g, reach)))
}

/// **`κ = bend/speed²`** levered by the carrier's own scale at the touch
/// (`Graze::scale`), a sagitta (`Margin::sagitta`): how far the carrier
/// departs from the surface's tangent plane over its own scale, the
/// span's length no part of it. Definitely positive is [`Pose::Tangent`],
/// its arc `arc(ε)` and its reach `arc(Kε)`.
///
/// # Errors
///
/// [`PoseRung::Kappa`] in band: in the gap, or in the zero band (a
/// higher-order contact).
fn curvature<T: Decide>(
    span: &Span<'_, T>,
    Confirmed(g, reach): Confirmed<T>,
    band: Band,
) -> Result<Pose<T>, PoseRefusal> {
    let decided = geom_core::k_stats::decide_reported(
        "curve_face_pose_kappa",
        // Unsourced lever: the carrier's own scale at the touch
        // (`Graze::scale`), each arm's stated where it builds the graze.
        Margin::sagitta(
            g.bend / g.speed.powi(2),
            geom_core::Lever::unsourced(g.scale),
        ),
        band,
    )
    .map_err(PoseRefusal::in_band(PoseRung::Kappa))?;
    if decided.sign == Sign::Zero {
        return Err(zero_band(
            PoseRung::Kappa,
            "curve_face_pose_kappa",
            decided,
            band,
        ));
    }
    let half = arc(&g, T::from_f64(band.zero()));
    Ok(Pose::Tangent(Touch {
        at: g.at,
        point: span.carrier().eval(g.at),
        arc: (g.at - half, g.at + half),
        reach,
    }))
}

// ------------------------------------------------------------- plane

/// **The plane arm** ([`Door::PlaneCrossing`]).
fn plane<T: Decide>(
    span: &Span<'_, T>,
    surface: &geom::Surface<T>,
    band: Band,
) -> Result<Pose<T>, Refused<T>> {
    let geom::Surface::Plane { frame, .. } = *surface else {
        return Err(
            PoseRefusal::Fault("the plane arm was handed a surface that is not a plane").into(),
        );
    };
    let (origin, normal) = (frame.origin(), frame.axis());
    let carrier = span.carrier();
    let (t0, t1) = span.params();
    match (carrier.conic(), carrier) {
        (Some(conic), _) => plane_conic(span, &conic, (origin, normal), band),
        (None, &geom::Curve3::Line(line)) => {
            Ok(plane_line(line, (t0, t1), (origin, normal), band)?)
        }
        (None, _) => Err(PoseRefusal::Fault(
            "the plane arm was handed a carrier that is not a line or a conic",
        )
        .into()),
    }
}

/// **A line against a plane** over the run `(t0, t1)`: the axis-parallel
/// rung on its drift `|d·n̂|·run` ([`axis_drift`]), then its offset at the
/// run's middle ([`axis_offset`]: lies or apart); past it, its one root.
/// A drift in its gap answers `Apart` where the offset reads the run
/// clear of the plane all along it, and escalates otherwise.
pub(super) fn plane_line<T: Decide>(
    line: geom::Line<T>,
    (t0, t1): (T, T),
    (origin, normal): (Point3<T>, geom_core::Vec3<T>),
    band: Band,
) -> Result<Pose<T>, PoseRefusal> {
    let lead = line.dir.get().dot(normal);
    let mid = geom::mid_param(t0, t1);
    let at_mid = geom::plane_offset(origin, normal, line.origin + line.dir.get() * mid);
    let run = (t1 - t0).abs();
    let half_drift = lead.abs() * run / T::from_f64(2.0);
    // A drift in its gap does not part a line from a plane it stands
    // clear of all along the run: the offset rung reads it apart first.
    let parallel = match axis_drift(
        Margin::metered(run, geom_core::InfSpeed::new(lead.abs())),
        band,
    ) {
        Ok(parallel) => parallel,
        Err(diag) => {
            return match axis_offset(at_mid, half_drift, band) {
                Ok(Pose::Apart) => Ok(Pose::Apart),
                _ => Err(PoseRefusal::in_band(PoseRung::AxisParallel)(diag)),
            };
        }
    };
    if parallel {
        return axis_offset(at_mid, half_drift, band);
    }
    Ok(Pose::Isolated(Form(Formed::PlaneLine {
        root: mid - at_mid / lead,
    })))
}

/// **A conic against a plane**: its height above the plane,
/// `d₀ + R cos(θ − φ)` (`Conic::along`), read on its rungs: the belly
/// (`R − |d₀|`, read to its own digits by
/// [`geom::Conic::belly_along`]: negative, apart), the parallel gate (`R` in the zero
/// band: the offset `d₀ ± R` decides lies or apart), then the belly's
/// two roots or its graze, whose side `d₀`'s sign decides and whose
/// curvature [`kappa`] reads. The roots stand the azimuth of
/// `(√(h·(R + |d₀|)), −d₀)` either side of `φ`, `h = R − |d₀|` the belly's depth: the value
/// of `acos(−d₀/R)` with no subtraction from one near a graze. A belly in
/// its gap is a window over the graze's reach ([`graze_window`]).
pub(super) fn plane_conic<T: Decide>(
    span: &Span<'_, T>,
    conic: &dyn geom::Conic<T>,
    (origin, normal): (Point3<T>, geom_core::Vec3<T>),
    band: Band,
) -> Result<Pose<T>, Refused<T>> {
    let (t0, t1) = span.params();
    let near = geom::mid_param(t0, t1);
    let (d0, a, b) = crate::curve_pair::pose::height(conic, origin, normal);
    // powi, never a*a: both amplitudes straddle zero on near-parallel
    // frames, and an interval product's spurious negative low end makes
    // the sqrt refuse.
    let r = (a.powi(2) + b.powi(2)).sqrt();
    // `r − |d₀|`, read to its own digits rather than as the difference of
    // two terms of the conic's size.
    let depth = conic.belly_along(origin, normal);
    // The miss first: a conic wholly on one side never meets the plane,
    // whatever its tilt reads.
    let belly = decide("curve_face_pose_belly_graze", Margin::of(depth), band);
    if let Ok(Sign::Negative) = belly {
        return Ok(Pose::Apart);
    }
    match decide("curve_face_pose_plane_parallel", Margin::of(r), band)
        .map_err(PoseRefusal::in_band(PoseRung::PlaneParallel))?
    {
        Sign::Zero => {
            return Ok(constant_residual_as(
                "curve_face_pose_plane_offset",
                PoseRung::PlaneParallel,
                (d0, r),
                band,
            )?
            .pose(PoseRung::PlaneParallel)?);
        }
        Sign::Positive | Sign::Negative => {}
    }
    let phi = geom::Azimuth::of(b, a).near(near);
    // The extreme nearest the plane: φ with the centre below it, φ + π
    // above.
    let extreme = || -> Result<T, PoseRefusal> {
        let at = match decide_nonzero("curve_face_pose_graze_side", Margin::of(d0), band)
            .map_err(PoseRefusal::in_band(PoseRung::BellyGraze))?
        {
            NonzeroSign::Negative => phi,
            NonzeroSign::Positive => phi + T::pi(),
        };
        Ok(geom_core::period::Period::angle().near(at, near))
    };
    let graze = |at: T| Graze {
        at,
        value: depth,
        noise: T::zero(),
        bend: r,
        speed: conic.speed_at(at),
        // The conic's diameter at the touch.
        scale: T::from_f64(2.0) * conic.speed_at(at),
        harmonic: true,
        turn: true,
        confirm: Ok(()),
    };
    match belly {
        Ok(Sign::Negative) => Ok(Pose::Apart),
        Ok(Sign::Positive) => {
            // `cos(θ − φ) = −d₀/R`: the half-chord's sine
            // `√(R² − d₀²) = √(h·(R + |d₀|))`, `h = R − |d₀|` read from the
            // same `R` and `d₀` the azimuth below reads.
            let sine = ((r - d0.abs()).max(T::zero()) * (r + d0.abs())).sqrt();
            let delta = geom::Azimuth::of(sine, T::zero() - d0)
                .near(T::from_f64(core::f64::consts::FRAC_PI_2));
            let turn = geom_core::period::Period::angle();
            Ok(Pose::Isolated(Form(Formed::PlaneConic {
                roots: [turn.near(phi + delta, near), turn.near(phi - delta, near)],
            })))
        }
        Ok(Sign::Zero) => kappa(span, graze(extreme()?), band),
        Err(diag) => graze_window(
            span,
            graze(extreme()?),
            PoseRefusal::in_band(PoseRung::BellyGraze)(diag),
            band,
        ),
    }
}

// ------------------------------------------------------------ spiric

/// **The spiric × plane arm** ([`Door::SpiricPlane`]). The span's
/// convention first (`curve_face_pose_spiric_*`, read at no swing: the
/// spiric's own data): outside its two-oval regime, or in its band, the
/// arm refuses on [`PoseRung::SpiricRegime`] with the regime's margin, as
/// for a span spelled past the mint. Then whether the two planes are parallel, by
/// their tilt `|û × n̂|` levered by the radius of the span's cover ball
/// (`curve_face_pose_plane_parallel`): parallel, the spiric's plane's
/// height off the plane at that ball's centre, to within that levered
/// tilt, decides `Lies` or `Apart` (`curve_face_pose_plane_offset`); a
/// tilt in its gap is `Apart` where that height clears it, and escalates
/// otherwise. Past the
/// band, the planes meet in a line, and the span meets the plane where
/// that line meets the oval: `Lies` is excluded, and the line is the
/// method's ([`super::spiric_plane`]).
fn spiric_plane<T: Decide>(
    span: &Span<'_, T>,
    surface: &geom::Surface<T>,
    band: Band,
) -> Result<Pose<T>, PoseRefusal> {
    let (
        &geom::Curve3::Spiric {
            frame: spiric_frame,
            offset,
            ..
        },
        &geom::Surface::Plane {
            frame: plane_frame, ..
        },
    ) = (span.carrier(), surface)
    else {
        return Err(PoseRefusal::Fault(
            "the spiric × plane arm was handed a pair that is not a spiric and a plane",
        ));
    };
    let ring = span
        .carrier()
        .spiric_torus()
        .ok_or(PoseRefusal::Fault("a spiric carrier that names no torus"))?;
    match geom::spiric_regime(
        ring,
        offset,
        T::zero(),
        geom::SpiricReader::CurveFace.sites(),
        band,
    )
    .map_err(PoseRefusal::in_band(PoseRung::SpiricRegime))?
    {
        geom::SpiricRegime::TwoOvals => {}
        regime @ (geom::SpiricRegime::ThroughAxis | geom::SpiricRegime::PastInnerEquator) => {
            // Data a span spelled past the mint can carry, so a refusal,
            // not a fault: the regime's margin read as its rejection.
            let in_band = PoseRefusal::in_band(PoseRung::SpiricRegime);
            let (name, read) = match regime {
                geom::SpiricRegime::ThroughAxis => (
                    "curve_face_pose_spiric_off_axis",
                    geom_core::k_stats::decide_reported(
                        "curve_face_pose_spiric_off_axis",
                        Margin::of(offset.abs()),
                        band,
                    )
                    .map_err(in_band)?,
                ),
                _ => (
                    "curve_face_pose_spiric_two_ovals",
                    geom_core::k_stats::decide_reported(
                        "curve_face_pose_spiric_two_ovals",
                        Margin::of((ring.major_radius() - ring.minor_radius()) - offset.abs()),
                        band,
                    )
                    .map_err(in_band)?,
                ),
            };
            return Err(PoseRefusal::InBand {
                rung: PoseRung::SpiricRegime,
                diag: geom_core::k_stats::gate_rejection(
                    name,
                    read,
                    band,
                    geom_core::SizedPass::Positive,
                ),
            });
        }
    }
    let (center, u) = (spiric_frame.origin(), spiric_frame.u_ref());
    let (origin, n) = (plane_frame.origin(), plane_frame.axis());
    let foot = center + u * offset;
    let Some((height, arm)) = span_in_plane(span, (foot, u), (origin, n)) else {
        return Err(PoseRefusal::Fault(
            "a spiric span in its convention with no cover",
        ));
    };
    let levered = u.cross(n).norm() * arm;
    let parallel = match decide("curve_face_pose_plane_parallel", Margin::of(levered), band) {
        Ok(sign) => sign == Sign::Zero,
        Err(diag) => {
            return match constant_residual_as(
                "curve_face_pose_plane_offset",
                PoseRung::PlaneParallel,
                (height, levered),
                band,
            ) {
                Ok(Constant::Apart) => Ok(Pose::Apart),
                _ => Err(PoseRefusal::in_band(PoseRung::PlaneParallel)(diag)),
            };
        }
    };
    if parallel {
        return constant_residual_as(
            "curve_face_pose_plane_offset",
            PoseRung::PlaneParallel,
            (height, levered),
            band,
        )?
        .pose(PoseRung::PlaneParallel);
    }
    // The line `P ∩ Q`, read in `P`'s own frame (`m = â × û`, `â`): a
    // point `foot + m·x + â·z` of `P` is on `Q` where `α·x + β·z + γ = 0`,
    // `α = m·n̂`, `β = â·n̂`, `γ` the foot's height off `Q`. Its point
    // nearest the foot is `foot − across·γ/s`, `s = √(α² + β²)` the
    // planes' tilt and `across = (α·m + β·â)/s`, and it runs along
    // `(α·â − β·m)/s`. Built from `P`'s own data, it lies in `P` to a
    // rounding of the foot's scale at any tilt. A change `δ` in `α`, `β`
    // or `γ` moves it, at a point of `P` a distance `d` from the foot, by
    // `(|δγ| + d·|δ(α, β)|)/s` to first order: the two planes' own
    // conditioning, `1/s`, and no worse, where the closest-point form
    // `((o − h·k)·û + (h − o·k)·n̂)/(1 − k²)`, `k = û·n̂`, cancels to
    // `1/s²` in both `1 − k²` and its numerators. A meeting's height off
    // `Q` is read along the oval to a rounding of its scale, and its place
    // to that over the crossing's slope.
    let axis = spiric_frame.axis();
    let m = axis.cross(u);
    let (alpha, beta) = (m.dot(n), axis.dot(n));
    let s = (alpha.powi(2) + beta.powi(2)).sqrt();
    let gamma = (foot - origin).dot(n);
    let across = (m * alpha + axis * beta) / s;
    let along = (axis * alpha - m * beta) / s;
    let to_line = gamma / s;
    Ok(Pose::Isolated(Form(Formed::SpiricPlane {
        line: geom::Line {
            origin: foot - across * to_line,
            // `(a·α − m·β)/s` over the orthonormal `a`, `m` with
            // `s = √(α² + β²)`: unit by its construction, checked.
            dir: geom_core::UnitVec3::checked(along),
        },
        across,
        sine: s,
    })))
}

/// **A planar span's height off a plane `(origin, n)`, and the radius it
/// holds to**: the span's cover ball, its centre read onto the span's own
/// plane `(through, u)`. Every point of the span lies in that plane within
/// the ball's radius of the centre's foot there, so its height off the
/// other plane is the foot's to within that radius times the planes' tilt.
/// `None` where the span has no cover.
fn span_in_plane<T: Decide>(
    span: &Span<'_, T>,
    (through, u): (Point3<T>, geom_core::Vec3<T>),
    (origin, n): (Point3<T>, geom_core::Vec3<T>),
) -> Option<(T, T)> {
    let (t0, t1) = span.params();
    let cover = span
        .carrier()
        .cover_over(geom::cover::ParamWindow::new(t0, t1))
        .ok()?;
    let (ball, radius) = cover.ball();
    let at = ball - u * (ball - through).dot(u);
    Some(((at - origin).dot(n), radius))
}

// -------------------------------------------------------------- line

/// **The line arm** ([`Door::LineRoots`]).
fn line<T: Decide>(
    span: &Span<'_, T>,
    surface: &geom::Surface<T>,
    band: Band,
) -> Result<Pose<T>, Refused<T>> {
    let &geom::Curve3::Line(line) = span.carrier() else {
        return Err(
            PoseRefusal::Fault("the line arm was handed a carrier that is not a line").into(),
        );
    };
    match *surface {
        geom::Surface::Cylinder { frame, radius, .. } => {
            let (origin, axis) = (frame.origin(), frame.axis_unit());
            line_wall(span, line, (geom::Line { origin, dir: axis }, radius), band)
        }
        geom::Surface::Sphere { frame, radius, .. } => {
            let center = frame.origin();
            line_sphere(span, line, (center, radius), band)
        }
        geom::Surface::Cone {
            frame, half_angle, ..
        } => {
            let (apex, axis) = (frame.origin(), frame.axis_unit());
            line_cone(
                span,
                line,
                (
                    geom::Line {
                        origin: apex,
                        dir: axis,
                    },
                    half_angle,
                ),
                band,
            )
        }
        // A line never lies on a torus: no ruling of a ring torus is a
        // line.
        geom::Surface::Torus { .. } => Ok(Pose::Isolated(Form(Formed::LineTorus))),
        _ => Err(PoseRefusal::Fault(
            "the line arm was handed a surface that is not a wall, a sphere, a cone or a torus",
        )
        .into()),
    }
}

/// **What the wall's and the sphere's rungs read of a line** ([`line_wall`],
/// [`line_sphere`]), before a graze's curvature: the span-free half the
/// ray lane's wrappers read too.
#[derive(Clone, Copy, Debug)]
pub(super) enum LineQuadric<T: Real> {
    /// The axis-parallel rung's drift in the zero band: the residual at
    /// the run's middle, to within the half drift everywhere on it.
    Parallel {
        /// The residual at the run's middle.
        at_mid: T,
        /// Half the drift.
        half_drift: T,
    },
    /// A definitely negative discriminant.
    Apart,
    /// Two roots, ascending, and a bound on either's rounding in the
    /// line's parameter.
    Two([T; 2], T),
    /// A discriminant in its gap: the graze it would be, a crossing or a
    /// miss the band cannot call.
    Undecided {
        /// The discriminant's diagnostics.
        diag: Indeterminate,
        /// The vertex, as [`LineQuadric::Graze`] reads it.
        at: T,
        /// The residual there.
        value: T,
        /// `|F″|` in the line's parameter.
        bend: T,
        /// The wall's or the sphere's radius.
        radius: T,
    },
    /// A discriminant in the zero band: the vertex, the residual there,
    /// and the residual's second derivative in the line's parameter.
    Graze {
        /// The vertex's parameter.
        at: T,
        /// The residual there.
        value: T,
        /// `|F″|` in the line's parameter.
        bend: T,
        /// The wall's or the sphere's radius.
        radius: T,
    },
}

impl<T: Decide> LineQuadric<T> {
    /// The pose, a graze's curvature read on `span`, a discriminant in its
    /// gap a window over the graze's reach ([`graze_window`]).
    fn pose(self, span: &Span<'_, T>, band: Band) -> Result<Pose<T>, Refused<T>> {
        let graze = |at, value, bend, radius| Graze {
            at,
            value,
            noise: T::zero(),
            bend,
            speed: span.speeds().1.get(),
            // The wall's or the sphere's radius.
            scale: radius,
            harmonic: false,
            turn: false,
            confirm: Ok(()),
        };
        match self {
            Self::Parallel { at_mid, half_drift } => Ok(axis_offset(at_mid, half_drift, band)?),
            Self::Apart => Ok(Pose::Apart),
            Self::Two(roots, _) => Ok(Pose::Isolated(Form(Formed::LineQuadratic { roots }))),
            Self::Graze {
                at,
                value,
                bend,
                radius,
            } => kappa(span, graze(at, value, bend, radius), band),
            Self::Undecided {
                diag,
                at,
                value,
                bend,
                radius,
            } => graze_window(
                span,
                graze(at, value, bend, radius),
                PoseRefusal::in_band(PoseRung::Discriminant)(diag),
                band,
            ),
        }
    }
}

/// **A line against a cylinder wall**: the axis-parallel rung
/// ([`axis_drift`], the line's drift off its distance from the axis
/// over the span, `|d⊥|·(t₁ − t₀)`, about that distance less the radius
/// at the span's middle), then the discriminant ([`discriminant`]).
fn line_wall<T: Decide>(
    span: &Span<'_, T>,
    line: geom::Line<T>,
    wall: (geom::Line<T>, T),
    band: Band,
) -> Result<Pose<T>, Refused<T>> {
    line_wall_read(
        line,
        wall,
        span.params(),
        super::line_quadric::About::Start,
        band,
    )
    .map_err(PoseRefusal::in_band(PoseRung::AxisParallel))?
    .pose(span, band)
}

/// [`line_wall`]'s span-free half, over the line's run `(t0, t1)`.
///
/// # Errors
///
/// The axis-parallel rung's diagnostics in band
/// ([`PoseRung::AxisParallel`]).
pub(super) fn line_wall_read<T: Decide>(
    line: geom::Line<T>,
    (wall, radius): (geom::Line<T>, T),
    (t0, t1): (T, T),
    about: super::line_quadric::About,
    band: Band,
) -> Result<LineQuadric<T>, Indeterminate> {
    let d = line.dir;
    let dp = d.get() - wall.dir.get() * d.get().dot(wall.dir.get());
    let run = (t1 - t0).abs();
    let mid = geom::mid_param(t0, t1);
    let at_mid = wall.distance(line.origin + d.get() * mid) - radius;
    let speed = dp.norm();
    if axis_drift(Margin::metered(run, geom_core::InfSpeed::new(speed)), band)? {
        return Ok(LineQuadric::Parallel {
            at_mid,
            half_drift: speed * run / T::from_f64(2.0),
        });
    }
    let centred = super::line_quadric::wall_quadratic(line, (wall, radius), run, about);
    Ok(discriminant(
        &centred,
        (dp.norm_squared(), radius),
        about,
        band,
    ))
}

/// **A line against a sphere**: the discriminant alone ([`discriminant`]):
/// a line never lies on a sphere.
fn line_sphere<T: Decide>(
    span: &Span<'_, T>,
    line: geom::Line<T>,
    sphere: (Point3<T>, T),
    band: Band,
) -> Result<Pose<T>, Refused<T>> {
    line_sphere_read(line, sphere, super::line_quadric::About::Start, band).pose(span, band)
}

/// [`line_sphere`]'s span-free half.
pub(super) fn line_sphere_read<T: Decide>(
    line: geom::Line<T>,
    (center, radius): (Point3<T>, T),
    about: super::line_quadric::About,
    band: Band,
) -> LineQuadric<T> {
    let centred = super::line_quadric::sphere_quadratic(line, (center, radius), about);
    discriminant(
        &centred,
        (line.dir.get().norm_squared(), radius),
        about,
        band,
    )
}

/// **The wall's and the sphere's discriminant** (`curve_face_pose_disc`):
/// `disc/a` over `2r`, the depth the line reaches inside the circle
/// across the axis or the sphere, a length. Positive: two roots; Zero: a
/// graze at the vertex, its residual `(|w⊥|² − r²)/2r` exactly quadratic
/// along the line with second derivative `a/r`; Negative: apart; in the
/// gap, that graze undecided.
fn discriminant<T: Decide>(
    centred: &super::line_quadric::Centred<T>,
    (a2, radius): (T, T),
    about: super::line_quadric::About,
    band: Band,
) -> LineQuadric<T> {
    let two_r = T::from_f64(2.0) * radius;
    let disc = centred.disc();
    let [a, b, c] = centred.coeffs;
    let (at, value, bend) = (
        (T::zero() - b) / a + centred.shift,
        (c - b.powi(2) / a) / two_r,
        a / radius,
    );
    match decide(
        "curve_face_pose_disc",
        Margin::over_lever(centred.charged(disc, about) / a2, two_r),
        band,
    ) {
        Ok(Sign::Positive) => {
            let (roots, slack) = centred.roots(disc);
            LineQuadric::Two(roots, slack)
        }
        Ok(Sign::Negative) => LineQuadric::Apart,
        Ok(Sign::Zero) => LineQuadric::Graze {
            at,
            value,
            bend,
            radius,
        },
        Err(diag) => LineQuadric::Undecided {
            diag,
            at,
            value,
            bend,
            radius,
        },
    }
}

/// **What the cone's rungs read of a line's run** ([`line_cone_read`]),
/// before a graze's curvature: the span-free half the ray lane's
/// wrapper reads too.
#[derive(Clone, Copy, Debug)]
pub(super) enum ConeRead<T: Real> {
    /// A generator through the apex: it lies on the cone.
    Lies,
    /// Off the apex, parallel to a generator: the method's own
    /// degenerate form.
    Parallel,
    /// Through the apex at `at`, the line's roots within `reach` of it
    /// along the line's parameter.
    Through {
        /// The apex's parameter: the end it is decided at, else the
        /// line's foot from it.
        at: T,
        /// The roots' spread about it, in the line's parameter.
        reach: T,
    },
    /// The run may reach the apex, a reading of the apex or of the roots'
    /// spread in band, `Lies` excluded: its roots lie within `reach` of
    /// `at` along the line's parameter, the quadratic's own two
    /// ([`super::line_quadric`]'s `apex_root_spread`), and `why` is the
    /// rung that read in band.
    ApexUndecided {
        /// The point the spread is read about: the end the apex is decided
        /// at, else the line's foot from it.
        at: T,
        /// The roots' spread about it.
        reach: T,
        /// The rung in band.
        why: PoseRefusal,
    },
    /// A definitely negative depth.
    Apart,
    /// A definitely positive depth: two roots, the method's.
    Two,
    /// A depth in the zero band: the vertex, a lower bound on the
    /// residual's magnitude there, and on its second derivative.
    Graze {
        /// The vertex's parameter.
        at: T,
        /// The residual there.
        value: T,
        /// `|F″|` in the line's parameter.
        bend: T,
        /// The vertex's distance from the apex.
        scale: T,
        /// The vertex's residual read through the cone's floor in the
        /// zero band, or the depth's zero reading rejected
        /// (`curve_face_pose_cone_graze`).
        confirm: Result<(), PoseRefusal>,
    },
}

/// **A line against a cone**: its quadratic read about its closest
/// approach to the apex held to the run (`super::line_quadric`'s
/// `line_cone_quadratic`), on its rungs, each a length:
///
/// - **the apex** (`curve_face_pose_cone_apex`), the run's distance from
///   it: not definitely positive, the run reaches the apex, at the end
///   the apex is decided at, else at its foot. With the lead in the zero
///   band and the run's whole reach through the cone's floor
///   (`curve_face_pose_cone_on_surface`) in it too, the line lies on the
///   cone; otherwise, `Lies` excluded on the residual itself
///   (`curve_face_pose_not_lies`), it passes through the apex, its roots
///   within the spread (`curve_face_pose_cone_apex_spread`, which must be
///   in the zero band: past it, a meter that refused);
/// - **the lead** off the apex, `A/|d|²` levered by the run: Zero, a
///   generator's segment off the apex lies on the cone (its reach through
///   the floor in the zero band), and otherwise, `Lies` excluded, the
///   line runs parallel to a generator, the method's degenerate form;
/// - **the depth** (`curve_face_pose_cone_depth`), `disc/(|A|·R)`, `R`
///   at least the residual's factor over the run, `disc` held to the
///   shift's rounding: positive, two roots; negative, apart; Zero, a
///   graze at the vertex once the vertex's residual read through the
///   floor is in the zero band too (`curve_face_pose_cone_graze`).
///
/// No class is decided on a zero reading of `F` alone: `|F| ≤ |res|`, so
/// a definite sign of `F` is the residual's, but `F` in the zero band
/// bounds the residual only through the floor `m = min(sin α, cos α)`.
///
/// # Errors
///
/// [`PoseRung::Apex`], [`PoseRung::ApexSpread`], [`PoseRung::ConeLead`],
/// [`PoseRung::NotLies`] or [`PoseRung::Discriminant`] in band (a depth in the zero band whose
/// residual through the floor is past it, the depth's zero reading
/// rejected); [`PoseRefusal::Meter`] for a spread definitely past the
/// band.
pub(super) fn line_cone_read<T: Decide>(
    line: geom::Line<T>,
    (cone, half_angle): (geom::Line<T>, T),
    (t0, t1): (T, T),
    band: Band,
) -> Result<ConeRead<T>, PoseRefusal> {
    use super::line_quadric::{About, apex_root_spread, line_cone_quadratic};
    let geom::Line { origin, dir } = line;
    let geom::Line {
        origin: apex,
        dir: axis,
    } = cone;
    let w0 = origin - apex;
    let dd = dir.get().norm_squared();
    let at = |t: T| w0 + dir.get() * t;
    let (lo, hi) = super::ascending((t0, t1));
    let foot = line.param_within(apex, (lo, hi));
    let centred = line_cone_quadratic(line, cone, half_angle, (lo, hi));
    let [a2, b2, c2] = centred.coeffs;
    let length = crate::edge_extent_inf(&geom::Curve3::Line(line), lo, hi);
    let lead = || {
        decide(
            "curve_face_pose_cone_lead",
            Margin::levered(a2 / dd, length),
            band,
        )
        .map_err(PoseRefusal::in_band(PoseRung::ConeLead))
    };
    let residual = |t: T| crate::cone_elevation(cone, half_angle, None, line.at(t));
    // `Lies` on the run's whole reach through the cone's floor
    // (`curve_face_pose_cone_on_surface`), never on the apex's and the
    // lead's readings apart (each `F`'s, not the residual's): the residual
    // `ρ cos α − |h| sin α` is 1-Lipschitz, so against the line through
    // the apex parallel to this one it moves at most `e`, the line's
    // distance from the apex; on that line `Q = A·s²` and
    // `Q = res·(ρ cos α + |h| sin α)`, the factor at least `m·|q|`,
    // `m = min(sin α, cos α)`, so `|res| ≤ |A|·|s|/(m·|d|)`. The run's
    // reach is `e + (|A|/|d|²)·L/m`, `L` the farther end's distance
    // from the line's foot. In the zero band the run lies on the cone;
    // past it nothing is decided, and the caller excludes `Lies` by a
    // reading of the residual itself ([`not_lies_over`]).
    let on_surface = || -> Result<bool, PoseRefusal> {
        let star = line.param(apex);
        let e = at(star).norm();
        let far = (lo - star).abs().max((hi - star).abs()) * dir.get().norm();
        let (sin, cos) = half_angle.sin_cos();
        let reach = e + a2.abs() / dd * far / sin.min(cos);
        match decide("curve_face_pose_cone_on_surface", Margin::of(reach), band)
            .map_err(PoseRefusal::in_band(PoseRung::ConeLead))?
        {
            Sign::Zero => Ok(true),
            Sign::Positive | Sign::Negative => Ok(false),
        }
    };
    // The apex's rung in its gap: the run may reach it, which only a
    // reading past `Lies` may answer locally ([`ConeRead::ApexUndecided`]).
    let mut apex_gap = None;
    let near_apex = match decide("curve_face_pose_cone_apex", Margin::norm3(at(foot)), band) {
        Ok(Sign::Positive) => false,
        Ok(Sign::Zero | Sign::Negative) => true,
        Err(diag) => {
            apex_gap = Some(PoseRefusal::in_band(PoseRung::Apex)(diag));
            true
        }
    };
    if near_apex {
        // The end the apex is decided at, else the foot: WHICH, not a
        // search a later end may answer past, an end in band ahead
        // refusing.
        let mut at_apex = foot;
        if apex_gap.is_none() {
            for end in [t0, t1] {
                match decide("curve_face_pose_cone_apex", Margin::norm3(at(end)), band) {
                    Ok(Sign::Zero) => {
                        at_apex = end;
                        break;
                    }
                    Ok(Sign::Positive | Sign::Negative) => {}
                    Err(diag) => {
                        apex_gap = Some(PoseRefusal::in_band(PoseRung::Apex)(diag));
                        break;
                    }
                }
            }
        }
        if lead()? == Sign::Zero && on_surface()? {
            return Ok(ConeRead::Lies);
        }
        // `Lies` unexcluded, the apex's gap refuses the whole run.
        if let Err(not_lies) = not_lies_over((t0, t1), residual, band) {
            return Err(apex_gap.unwrap_or(not_lies));
        }
        let spread = apex_root_spread(at(at_apex), dir.get(), (axis.get(), half_angle));
        let reach = spread / dir.get().norm();
        return match decide("curve_face_pose_cone_apex_spread", Margin::of(spread), band) {
            Ok(Sign::Zero) => Ok(apex_gap.map_or(
                ConeRead::Through { at: at_apex, reach },
                |why| ConeRead::ApexUndecided {
                    at: at_apex,
                    reach,
                    why,
                },
            )),
            Err(diag) => Ok(ConeRead::ApexUndecided {
                at: at_apex,
                reach,
                why: apex_gap.unwrap_or(PoseRefusal::in_band(PoseRung::ApexSpread)(diag)),
            }),
            Ok(Sign::Positive | Sign::Negative) => Err(PoseRefusal::Meter {
                rung: PoseRung::ApexSpread,
                row: "curve_face_pose_cone_apex_spread",
            }),
        };
    }
    if lead()? == Sign::Zero {
        // A segment of a generator off the apex lies on the cone too.
        if on_surface()? {
            return Ok(ConeRead::Lies);
        }
        not_lies_over((t0, t1), residual, band)?;
        return Ok(ConeRead::Parallel);
    }
    let disc = centred.disc();
    let vertex = centred.shift - b2 / a2;
    let reach = at(lo).norm().max(at(hi).norm()).max(at(vertex).norm());
    let depth = geom_core::k_stats::decide_reported(
        "curve_face_pose_cone_depth",
        Margin::of(centred.charged(disc, About::Nearest) / (a2.abs() * reach)),
        band,
    )
    .map_err(PoseRefusal::in_band(PoseRung::Discriminant))?;
    match depth.sign {
        Sign::Positive => Ok(ConeRead::Two),
        Sign::Negative => Ok(ConeRead::Apart),
        // `|res| ≥ |G|/R`, `G` the quadratic, so the depth in the zero
        // band bounds the residual only from below: its graze is read
        // through the floor (`curve_face_pose_cone_graze`), `|res| ≤
        // |G|/(m·|q|)` at the vertex (`on_surface` above), past which the
        // vertex's residual is undecided, a crossing or a miss, and the
        // depth's zero reading is rejected. Read in the band, its value
        // and curvature are `G`'s over `R`.
        Sign::Zero => {
            // `|res(v)| ≤ |G(v)|/(m·|q(v)|)`, and `≤ |q(v)|` (the residual
            // is 1-Lipschitz and zero at the apex), which holds at the
            // apex itself.
            let (sin, cos) = half_angle.sin_cos();
            let qv = at(vertex).norm();
            let above =
                qv.min(centred.disc_bound(disc, About::Nearest) / (a2.abs() * sin.min(cos) * qv));
            let confirm = match decide("curve_face_pose_cone_graze", Margin::of(above), band) {
                Ok(Sign::Zero) => Ok(()),
                Ok(Sign::Positive | Sign::Negative) => Err(zero_band(
                    PoseRung::Discriminant,
                    "curve_face_pose_cone_depth",
                    depth,
                    band,
                )),
                Err(diag) => Err(PoseRefusal::InBand {
                    rung: PoseRung::Discriminant,
                    diag,
                }),
            };
            Ok(ConeRead::Graze {
                at: vertex,
                value: (c2 - b2.powi(2) / a2) / reach,
                bend: a2.abs() / reach,
                scale: qv,
                confirm,
            })
        }
    }
}

/// **A line against a cone** ([`line_cone_read`]), a graze's curvature
/// read on `span`. A reading of the apex in band, once `Lies` is excluded,
/// is a window over the line's two roots' spread about the apex
/// ([`ConeRead::ApexUndecided`], the read's own datum, so no meeting lies
/// past it), widened by `Kε`.
fn line_cone<T: Decide>(
    span: &Span<'_, T>,
    line: geom::Line<T>,
    cone: (geom::Line<T>, T),
    band: Band,
) -> Result<Pose<T>, Refused<T>> {
    let read = line_cone_read(line, cone, span.params(), band)?;
    Ok(match read {
        ConeRead::Lies => Pose::Lies,
        ConeRead::Parallel => Pose::Isolated(Form(Formed::LineCone { parallel: true })),
        ConeRead::Through { at, reach } => Pose::Through(Singular {
            at,
            point: cone.0.origin,
            spread: (at - reach, at + reach),
        }),
        ConeRead::ApexUndecided { at, reach, why } => {
            // A line along a generator has no quadratic: its spread is no
            // length, and its window the whole span.
            let reach = reach + super::escalation_reach(span, band);
            let window = geom_core::is_finite_length(reach)
                .then(|| local(span, (at - reach, at + reach), band))
                .flatten();
            return Err(Refused { why, window });
        }
        ConeRead::Apart => Pose::Apart,
        ConeRead::Two => Pose::Isolated(Form(Formed::LineCone { parallel: false })),
        ConeRead::Graze {
            at,
            value,
            bend,
            scale,
            confirm,
        } => {
            let graze = Graze {
                at,
                value,
                noise: T::zero(),
                bend,
                speed: line.dir.get().norm(),
                // The vertex's distance from the apex.
                scale,
                harmonic: false,
                turn: false,
                confirm,
            };
            return kappa(span, graze, band);
        }
    })
}

// ------------------------------------------------- conic × quadric

/// **The conic × quadric arm** ([`Door::ConicQuadric`]): its lane choice
/// and the zero lane's rungs (`super::conic_quadric`'s module docs).
/// A circle against a sphere has `A₂ ≡ 0` by kind and takes the zero lane
/// with no decision; a cone reads its apex first, whole.
fn conic_quadric<T: Decide>(
    span: &Span<'_, T>,
    surface: &geom::Surface<T>,
    band: Band,
) -> Result<Pose<T>, Refused<T>> {
    use super::conic_quadric as door;
    let carrier = span.carrier();
    let conic = carrier.conic().ok_or(PoseRefusal::Fault(door::DESYNC))?;
    if culled(span, conic, surface, band) {
        return Ok(Pose::Apart);
    }
    match (carrier, surface) {
        (
            &geom::Curve3::Circle(circle),
            &geom::Surface::Sphere {
                frame: sphere_frame,
                radius: s_radius,
                ..
            },
        ) => {
            let s_center = sphere_frame.origin();
            let h = crate::circle_sphere_harmonic(&circle, s_center, s_radius);
            let first = super::circle_roots::FirstHarmonic {
                lo: h.lo.with_charge(h.frame_error),
                hi: h.hi.with_charge(h.frame_error),
                cos_part: h.e_u,
                sin_part: h.e_v,
                phase_noise: h.phase_error,
            };
            first_harmonic(span, &conic, first, None, band)
        }
        (
            _,
            &geom::Surface::Cone {
                frame: cone_frame,
                half_angle,
                ..
            },
        ) => {
            let (apex, axis) = (cone_frame.origin(), cone_frame.axis_unit());
            let h = door::cone_harmonics(&conic, (apex, axis, half_angle));
            let noise = door::cone_noise(&h);
            if let Some(through) = conic_apex(span, &conic, (apex, axis, half_angle), &h, band)? {
                return Ok(through);
            }
            lane(span, &conic, surface, (&h, noise), Some(h.floor), band)
        }
        _ => {
            let ladder = door::quadric_ladder(&conic, surface, span.params())
                .ok_or(PoseRefusal::Fault(door::DESYNC))?;
            lane(
                span,
                &conic,
                surface,
                (&ladder.h, ladder.frame.walk.noise),
                None,
                band,
            )
        }
    }
}

/// **The lane choice** (`[verdicts.consumers]`, C5's conic × quadric
/// rule): `A₂ = |(c₂, s₂)|` in the zero band takes the first-harmonic
/// lane, with `A₂` charged to the extremes' noise; definite takes the
/// ladder, `Lies` excluded by it; in the gap takes the ladder once
/// [`not_lies`] has excluded `Lies`. It records nothing: either lane is
/// sound on either reading (the forced-ladder row,
/// `conic_quadric::ladder_lane_fuzz`). `cone` is the cone's floor on
/// `|res|/|F|` where `F` is a cone's form.
fn lane<T: Decide>(
    span: &Span<'_, T>,
    conic: &dyn geom::Conic<T>,
    surface: &geom::Surface<T>,
    (h, noise): (&crate::ConicHarmonics<T>, T),
    cone: Option<T>,
    band: Band,
) -> Result<Pose<T>, Refused<T>> {
    let hypot = |x: T, y: T| (x.powi(2) + y.powi(2)).sqrt();
    let second = hypot(h.c2, h.s2);
    match decide("curve_face_pose_second_harmonic", Margin::of(second), band) {
        Ok(Sign::Zero) if !ladder_lane::forced() => {
            let (a1, noise) = (hypot(h.c1, h.s1), noise + second);
            let extreme = |value| Rounded {
                value,
                error: noise,
            };
            let first = super::circle_roots::FirstHarmonic {
                lo: extreme(h.c0 - a1),
                hi: extreme(h.c0 + a1),
                cos_part: h.c1,
                sin_part: h.s1,
                phase_noise: T::zero(),
            };
            first_harmonic(span, conic, first, cone, band)
        }
        Ok(Sign::Zero) => Ok(Pose::Isolated(Form(Formed::Ladder))),
        Ok(Sign::Positive | Sign::Negative) => Ok(Pose::Isolated(Form(Formed::Ladder))),
        Err(_) => {
            not_lies(span, surface, band)?;
            Ok(Pose::Isolated(Form(Formed::Ladder)))
        }
    }
}

/// **The first-harmonic lane's rungs** on a residual
/// `c₀ + A₁ cos(θ − φ)` given by its extremes (`super::circle_roots`'
/// module docs): the noise meter, then the coaxial stage, then the
/// extremes, then a graze's curvature.
///
/// - **coaxial** (`curve_face_pose_coaxial`, `A₁` in the zero band): the
///   residual is `c₀` to within `A₁` plus the noise
///   (`curve_face_pose_constant`): definitely one-signed is `Apart`, in
///   the zero band all round is `Lies` (on a cone, once its whole reach
///   read through the floor is in band too,
///   `curve_face_pose_cone_on_surface`), and any other reading falls
///   through to the extremes, which decide the same carrier;
/// - **extremes** (`curve_face_pose_extreme`), each the interval its
///   noise spans: one definitely past zero on the far side is `Apart`,
///   definitely straddling is `Isolated`, and one in the zero band is a
///   graze at that extreme, whose curvature `A₁` [`kappa`] reads; on a
///   cone, whose `F` bounds the residual only through its floor, once
///   that extreme read through the floor is in the zero band too
///   (`curve_face_pose_cone_graze`), its zero reading rejected
///   otherwise.
fn first_harmonic<T: Decide>(
    span: &Span<'_, T>,
    conic: &dyn geom::Conic<T>,
    h: super::circle_roots::FirstHarmonic<T>,
    cone: Option<T>,
    band: Band,
) -> Result<Pose<T>, Refused<T>> {
    let two = T::from_f64(2.0);
    let noise = h.lo.error.max(h.hi.error);
    match decide("curve_face_pose_noise", Margin::of(noise), band) {
        Ok(Sign::Zero | Sign::Negative) => {}
        Ok(Sign::Positive) | Err(_) => {
            return Err(PoseRefusal::Meter {
                rung: PoseRung::Noise,
                row: "curve_face_pose_noise",
            }
            .into());
        }
    }
    let (c0, a1) = (
        (h.lo.value + h.hi.value) / two,
        (h.hi.value - h.lo.value) / two,
    );
    if let Ok(Sign::Zero) = decide("curve_face_pose_coaxial", Margin::of(a1), band) {
        match constant_residual_as(
            "curve_face_pose_constant",
            PoseRung::Coaxial,
            (c0, a1 + noise),
            band,
        )? {
            Constant::Apart => return Ok(Pose::Apart),
            Constant::Lies => {
                let Some(floor) = cone else {
                    return Ok(Pose::Lies);
                };
                // `|res| ≤ |F| / floor`: a constant `F` in the band is the
                // carrier on the cone once its reach, read through the
                // floor, is in the band too.
                let reach = h.most() / floor;
                return match decide("curve_face_pose_cone_on_surface", Margin::of(reach), band) {
                    Ok(Sign::Zero) => Ok(Pose::Lies),
                    Ok(Sign::Positive | Sign::Negative) | Err(_) => Err(PoseRefusal::Meter {
                        rung: PoseRung::Coaxial,
                        row: "curve_face_pose_cone_on_surface",
                    }
                    .into()),
                };
            }
            Constant::Straddle(_) => {}
        }
    }
    // Each extreme read as the interval its noise spans: ends that
    // disagree read zero, and the zero end is kept for a refusal.
    let extreme = |read: Rounded<T>| {
        let (below, above) =
            crate::roots::spread_readings("curve_face_pose_extreme", read.value, read.error, band)
                .map_err(PoseRefusal::in_band(PoseRung::Extreme))?;
        Ok::<_, PoseRefusal>(if below.sign == above.sign {
            (below.sign, below)
        } else if above.sign == Sign::Zero {
            (Sign::Zero, above)
        } else {
            (Sign::Zero, below)
        })
    };
    let near = geom::mid_param(span.params().0, span.params().1);
    let phi = geom::Azimuth::of(h.sin_part, h.cos_part).near(near);
    let turn = geom_core::period::Period::angle();
    let at_extreme = |at: T, extreme: Rounded<T>, confirm| {
        let at = turn.near(at, near);
        Graze {
            at,
            value: extreme.value,
            noise: extreme.error,
            bend: a1,
            speed: conic.speed_at(at),
            // The conic's diameter at the touch.
            scale: T::from_f64(2.0) * conic.speed_at(at),
            harmonic: false,
            turn: true,
            confirm,
        }
    };
    // An extreme in its gap, the other read definitely on the far side of
    // zero: the residual's roots, if any, lie in that extreme's reach
    // ([`graze_window`]). On a cone `|F| ≤ |res|` (the implicit's docs), so
    // wherever the residual is within `Kε` so is `F`: `F`'s reach holds
    // the residual's `Kε` reach, its roots with it (`window_rows`' cone
    // row).
    let (lo, lo_read) = match extreme(h.lo) {
        Ok(read) => read,
        Err(why) => {
            return match extreme(h.hi)? {
                (Sign::Negative, _) => Ok(Pose::Apart),
                (Sign::Positive, _) => {
                    graze_window(span, at_extreme(phi + T::pi(), h.lo, Ok(())), why, band)
                }
                (Sign::Zero, _) => Err(why.into()),
            };
        }
    };
    if lo == Sign::Positive {
        return Ok(Pose::Apart);
    }
    let (hi, hi_read) = match extreme(h.hi) {
        Ok(read) => read,
        Err(why) if lo == Sign::Negative => {
            return graze_window(span, at_extreme(phi, h.hi, Ok(())), why, band);
        }
        Err(why) => return Err(why.into()),
    };
    if hi == Sign::Negative {
        return Ok(Pose::Apart);
    }
    let graze = |at: T, extreme: Rounded<T>, read: geom_core::Decided| {
        // On a cone the extreme is `F`'s, `|res| ≤ |F| / floor`: an `F`
        // in the zero band is the residual's graze only once it reads in
        // the zero band through the floor too
        // (`curve_face_pose_cone_graze`). Past it the residual's extreme
        // is undecided, a crossing or a miss: the extreme's zero reading
        // is rejected, as any extreme the class needs definite.
        let confirm = cone.map_or(Ok(()), |floor| {
            match decide(
                "curve_face_pose_cone_graze",
                Margin::of(extreme.magnitude() / floor),
                band,
            ) {
                Ok(Sign::Zero) => Ok(()),
                Ok(Sign::Positive | Sign::Negative) => Err(zero_band(
                    PoseRung::Extreme,
                    "curve_face_pose_extreme",
                    read,
                    band,
                )),
                Err(diag) => Err(PoseRefusal::InBand {
                    rung: PoseRung::Extreme,
                    diag,
                }),
            }
        });
        let g = at_extreme(at, extreme, confirm);
        kappa(span, g, band)
    };
    match (lo, hi) {
        (Sign::Negative, Sign::Positive) => Ok(Pose::Isolated(Form(Formed::FirstHarmonic(h)))),
        (Sign::Zero, Sign::Positive) => graze(phi + T::pi(), h.lo, lo_read),
        (Sign::Negative, Sign::Zero) => graze(phi, h.hi, hi_read),
        _ => Err(zero_band(PoseRung::Extreme, "curve_face_pose_extreme", lo_read, band).into()),
    }
}

/// **A conic through a cone's apex** (`super::conic_quadric`'s module
/// docs, "Through the apex"): the apex on the carrier at the anomaly of
/// its own in-plane coordinates (`curve_face_pose_conic_apex`), then the
/// spread of the residual's roots about it
/// (`curve_face_pose_conic_apex_spread`, a bound). Through the apex is
/// [`Pose::Through`]; definitely off it is `None`, the lane's to read.
fn conic_apex<T: Decide>(
    span: &Span<'_, T>,
    conic: &dyn geom::Conic<T>,
    cone: (Point3<T>, geom_core::UnitVec3<T>, T),
    h: &crate::ConicHarmonics<T>,
    band: Band,
) -> Result<Option<Pose<T>>, PoseRefusal> {
    let mid = geom::mid_param(span.params().0, span.params().1);
    let theta_a = conic.azimuth(cone.0).near(mid);
    let off_apex = conic.point_at(theta_a) - cone.0;
    match decide("curve_face_pose_conic_apex", Margin::norm3(off_apex), band)
        .map_err(PoseRefusal::in_band(PoseRung::Apex))?
    {
        Sign::Positive => return Ok(None),
        Sign::Zero | Sign::Negative => {}
    }
    let spread_phi = super::conic_quadric::apex_spread(conic, cone, h, theta_a);
    match decide(
        "curve_face_pose_conic_apex_spread",
        Margin::of(spread_phi * conic.speed_hi()),
        band,
    ) {
        Ok(Sign::Zero) => Ok(Some(Pose::Through(Singular {
            at: theta_a,
            point: cone.0,
            spread: (theta_a - spread_phi, theta_a + spread_phi),
        }))),
        Err(diag) => Err(PoseRefusal::InBand {
            rung: PoseRung::ApexSpread,
            diag,
        }),
        Ok(Sign::Positive | Sign::Negative) => Err(PoseRefusal::Meter {
            rung: PoseRung::ApexSpread,
            row: "curve_face_pose_conic_apex_spread",
        }),
    }
}

/// **The general not-`Lies` rung** (`curve_face_pose_not_lies`): an arm
/// with no rung that excludes `Lies` reads the span's points at its ends,
/// quarters and middle until one's residual is decided past the band.
/// Found, the span does not lie on the surface.
///
/// # Errors
///
/// None found: [`PoseRung::NotLies`] in band, at the first reading in the
/// gap, else at the first in the zero band.
fn not_lies<T: Decide>(
    span: &Span<'_, T>,
    surface: &geom::Surface<T>,
    band: Band,
) -> Result<(), PoseRefusal> {
    not_lies_over(
        span.params(),
        |t| crate::implicit_residual(surface, span.carrier().eval(t)),
        band,
    )
}

/// [`not_lies`] over the run `(t0, t1)` of any carrier whose residual at
/// `t` is `residual(t)`: the span-free half the ray lane reads too.
fn not_lies_over<T: Decide>(
    (t0, t1): (T, T),
    residual: impl Fn(T) -> T,
    band: Band,
) -> Result<(), PoseRefusal> {
    let (mut gap, mut zero) = (None, None);
    for k in [2.0, 0.0, 4.0, 1.0, 3.0] {
        let t = t0 + (t1 - t0) * T::from_f64(k / 4.0);
        match geom_core::k_stats::decide_reported(
            "curve_face_pose_not_lies",
            Margin::of(residual(t)),
            band,
        ) {
            Ok(read) if read.sign != Sign::Zero => return Ok(()),
            Ok(read) => {
                zero.get_or_insert(read);
            }
            Err(diag) => {
                gap.get_or_insert(diag);
            }
        }
    }
    Err(match (gap, zero) {
        (Some(diag), _) => PoseRefusal::in_band(PoseRung::NotLies)(diag),
        (None, Some(read)) => zero_band(PoseRung::NotLies, "curve_face_pose_not_lies", read, band),
        (None, None) => unreachable!("five readings were taken"),
    })
}

/// The lane choice's test-only switch: forced, a second harmonic in the
/// zero band takes the ladder, `Lies` unexcluded, so a row can judge the
/// ladder's certificate where the choice would not send it
/// (`conic_quadric::ladder_lane_fuzz`).
pub(super) mod ladder_lane {
    #[cfg(test)]
    std::thread_local! {
        static FORCED: core::cell::Cell<bool> = const { core::cell::Cell::new(false) };
    }

    /// Whether the ladder is forced on this thread: never outside a
    /// test.
    #[cfg(not(test))]
    pub(in crate::curve_face) const fn forced() -> bool {
        false
    }

    /// Whether the ladder is forced on this thread.
    #[cfg(test)]
    pub(in crate::curve_face) fn forced() -> bool {
        FORCED.with(core::cell::Cell::get)
    }

    /// `body` with the ladder forced.
    #[cfg(test)]
    pub(in crate::curve_face) fn forcing<R>(body: impl FnOnce() -> R) -> R {
        FORCED.with(|f| f.set(true));
        let out = body();
        FORCED.with(|f| f.set(false));
        out
    }
}

// ----------------------------------------------------- conic × torus

/// **The conic × torus arm** ([`Door::ConicTorus`]): the ring
/// convention first, then on a circle the coaxial rungs (its tilt to the
/// axis and its centre's offset: a coaxial circle's constant residual is
/// `Lies`, a rim, or `Apart`), the parallel arm's contour placements, and
/// the meridian (`Lies`); past them, and on an ellipse, [`not_lies`].
/// The contour placements decide the class (a graze at an extreme is
/// neither a crossing nor a miss), so they are this module's rungs; the
/// roots they bracket are the method's.
fn conic_torus<T: Decide>(
    span: &Span<'_, T>,
    surface: &geom::Surface<T>,
    band: Band,
) -> Result<Pose<T>, PoseRefusal> {
    use super::circle_torus as door;
    let geom::Surface::Torus {
        frame,
        major_radius,
        minor_radius,
        ..
    } = *surface
    else {
        return Err(PoseRefusal::Fault(
            "the conic × torus arm was handed a surface that is not a torus",
        ));
    };
    let conic = span.carrier().conic().ok_or(PoseRefusal::Fault(
        "the conic × torus arm was handed a carrier that is not a conic",
    ))?;
    if culled(span, conic, surface, band) {
        return Ok(Pose::Apart);
    }
    let (t_center, t_axis) = (frame.origin(), frame.axis_unit());
    // The doors read only a ring, asked of the convention past the cull,
    // before any other pose: their harmonics take the witness, whose floor on `|F|` per
    // metre and whose meridian bound lean on `R − r > 0`.
    let ring = crate::TorusRefusal::require(major_radius, minor_radius, band)
        .map_err(PoseRefusal::Torus)?;
    let floor = crate::ring_floor(ring);
    let walk = || -> Result<Pose<T>, PoseRefusal> {
        not_lies(span, surface, band)?;
        Ok(Pose::Isolated(Form(Formed::TorusWalk { floor })))
    };
    let &geom::Curve3::Circle(circle) = span.carrier() else {
        return walk();
    };
    let axes = door::axes(&circle, (t_center, t_axis));
    if let Ok(Sign::Zero) = decide("curve_face_pose_torus_tilt", Margin::of(axes.tilt), band) {
        match decide(
            "curve_face_pose_torus_offset",
            Margin::of(axes.offset),
            band,
        )
        .map_err(PoseRefusal::in_band(PoseRung::Coaxial))?
        {
            Sign::Zero => {
                return constant_residual_as(
                    "curve_face_pose_torus_constant",
                    PoseRung::Coaxial,
                    door::coaxial(&circle, surface, minor_radius, &axes),
                    band,
                )?
                .pose(PoseRung::Coaxial);
            }
            Sign::Positive => {
                return torus_parallel(
                    span,
                    &circle,
                    surface,
                    (major_radius, minor_radius),
                    &axes,
                    band,
                );
            }
            Sign::Negative => {
                return Err(PoseRefusal::Fault("a norm read a negative offset"));
            }
        }
    }
    // A tilt in the gap is no coaxial pose: the general rungs read it.
    let deviation = door::meridian(
        &circle,
        (t_center, t_axis.get(), major_radius, minor_radius),
        &axes,
    );
    match decide("curve_face_pose_meridian", Margin::of(deviation), band)
        .map_err(PoseRefusal::in_band(PoseRung::Meridian))?
    {
        Sign::Zero => Ok(Pose::Lies),
        Sign::Positive => walk(),
        Sign::Negative => Err(PoseRefusal::Fault(
            "a meridian deviation, a sum of lengths, read negative",
        )),
    }
}

/// **The parallel arm's placements** (`super::circle_torus::Contours`):
/// the plane's depth into the tube (`curve_face_pose_torus_plane_height`,
/// a screen: definitely negative is apart, a graze defers to the
/// extremes), then each extreme placed by its own residual
/// (`curve_face_pose_contour_residual`) and, outside the tube, by its
/// side of the centre circle (`curve_face_pose_contour_side`). At a
/// graze both extremes outside the tube on one side keep the whole
/// carrier there: apart. Otherwise a crossed contour is `Isolated`, none
/// `Apart`, and an extreme in band refuses.
fn torus_parallel<T: Decide>(
    span: &Span<'_, T>,
    circle: &geom::Circle<T>,
    surface: &geom::Surface<T>,
    (major_radius, minor_radius): (T, T),
    axes: &super::circle_torus::Axes<T>,
    band: Band,
) -> Result<Pose<T>, PoseRefusal> {
    use super::circle_torus::Band3;
    let c =
        super::circle_torus::contours(circle, (major_radius, minor_radius), axes, span.params());
    let in_band = PoseRefusal::in_band(PoseRung::Contour);
    let height = "curve_face_pose_torus_plane_height";
    let graze = match crate::roots::spread_readings(
        "curve_face_pose_torus_plane_height",
        c.depth,
        c.charge_h,
        band,
    ) {
        Ok((below, _)) if below.sign == Sign::Positive => None,
        Ok((_, above)) if above.sign == Sign::Negative => return Ok(Pose::Apart),
        // A reading in the zero band is its gate's rejection; a definite
        // straddle is a straddle, no zero reading.
        Ok((below, _)) if below.sign == Sign::Zero => {
            Some(zero_band(PoseRung::Contour, height, below, band))
        }
        Ok((_, above)) if above.sign == Sign::Zero => {
            Some(zero_band(PoseRung::Contour, height, above, band))
        }
        Ok((below, above)) => Some(in_band(straddle(
            "curve_face_pose_torus_plane_height",
            below,
            above,
            band,
        ))),
        Err(diag) => Some(in_band(diag)),
    };
    let residual = "curve_face_pose_contour_residual";
    let side = "curve_face_pose_contour_side";
    let place = |theta: T, rho: T| -> Result<Band3, PoseRefusal> {
        let res = crate::implicit_residual(surface, circle.point_at(theta));
        let (below, above) =
            crate::roots::spread_readings("curve_face_pose_contour_residual", res, c.charge, band)
                .map_err(&in_band)?;
        match (below.sign, above.sign) {
            (_, Sign::Negative) => return Ok(Band3::Tube),
            (Sign::Positive, _) => {}
            (Sign::Zero, _) => return Err(zero_band(PoseRung::Contour, residual, below, band)),
            (_, Sign::Zero) => return Err(zero_band(PoseRung::Contour, residual, above, band)),
            // A definite straddle of the extreme's value and its charge.
            (Sign::Negative, Sign::Positive) => {
                return Err(in_band(straddle(
                    "curve_face_pose_contour_residual",
                    below,
                    above,
                    band,
                )));
            }
        }
        let read = geom_core::k_stats::decide_reported(
            "curve_face_pose_contour_side",
            Margin::of(rho - major_radius),
            band,
        )
        .map_err(&in_band)?;
        match read.sign {
            Sign::Positive => Ok(Band3::Beyond),
            Sign::Negative => Ok(Band3::Hole),
            Sign::Zero => Err(zero_band(PoseRung::Contour, side, read, band)),
        }
    };
    let far = place(c.theta0, c.rho.1)?;
    let near = place(c.theta0 + T::pi(), c.rho.0)?;
    if let Some(refusal) = graze {
        return if far == near && far != Band3::Tube {
            Ok(Pose::Apart)
        } else {
            Err(refusal)
        };
    }
    let crosses = (near == Band3::Hole && far != Band3::Hole)
        || (far == Band3::Beyond && near != Band3::Beyond);
    Ok(if crosses {
        Pose::Isolated(Form(Formed::TorusParallel { far, near }))
    } else {
        Pose::Apart
    })
}
