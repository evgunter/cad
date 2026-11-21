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
//!   zero reading, [`zero_band`]). Its arc
//!   ([`Touch::arc`]) encloses every span parameter whose point is within
//!   the band of the surface about the touch, and never understates it.
//! - **A singular point is not a meeting.** A cone's apex on the span is
//!   [`Pose::Through`]: the pose's datum, which a consumer reads before
//!   any meeting.

use geom::Conic;
use geom_core::k_stats::{decide, decide_nonzero, straddle};
use geom_core::{Band, Decide, Indeterminate, Margin, NonzeroSign, Point3, Real, Sign};

use super::{Door, Route, route};
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
    /// parameter near it whose point is within the band of the surface:
    /// it never understates that arc.
    pub arc: (T, T),
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
    /// Whether a line turns off a cone's generator over the span.
    ConeLead,
    /// Whether a circle is a torus's meridian.
    Meridian,
    /// A torus's contour reached by a circle parallel to its axis.
    Contour,
    /// One carrier point's residual decided past the band.
    NotLies,
}

/// **Why [`super::pose()`] gave no class.** Each is one window over the
/// whole span ([`super::cross()`]).
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

/// **The pose of `span` against `surface`** ([`super::pose()`]): the arm
/// [`route`] names, read by its rungs.
///
/// # Errors
///
/// [`PoseRefusal`], as its variants say.
pub fn pose<T: Decide>(
    span: &Span<'_, T>,
    surface: &geom::Surface<T>,
    band: Band,
) -> Result<Pose<T>, PoseRefusal> {
    let carrier = span.carrier();
    let way = route(carrier.kind(), surface.kind());
    if !way.implemented {
        return Err(PoseRefusal::Unbuilt(way));
    }
    match way.door {
        Door::PlaneCrossing => plane(span, surface, band),
        Door::LineRoots => line(span, surface, band),
        Door::ConicQuadric => conic_quadric(span, surface, band),
        Door::ConicTorus => conic_torus(span, surface, band),
        Door::SplineComposite | Door::SplineFace => Err(PoseRefusal::Enclosed),
        Door::Spiric | Door::Sheet => Err(PoseRefusal::Unbuilt(way)),
    }
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
/// [`PoseRung::AxisParallel`] in band.
pub(super) fn axis_drift<T: Decide>(drift: Margin<T>, band: Band) -> Result<bool, PoseRefusal> {
    Ok(
        match decide("curve_face_pose_axis_parallel", drift, band)
            .map_err(PoseRefusal::in_band(PoseRung::AxisParallel))?
        {
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
    let (below, above) =
        crate::roots::spread_readings("curve_face_pose_axis_offset", at_mid, half_drift, band)
            .map_err(PoseRefusal::in_band(PoseRung::AxisParallel))?;
    match (below.sign, above.sign) {
        (Sign::Positive, _) | (_, Sign::Negative) => Ok(Pose::Apart),
        (Sign::Zero, Sign::Zero) => Ok(Pose::Lies),
        _ => Err(PoseRefusal::InBand {
            rung: PoseRung::AxisParallel,
            diag: straddle("curve_face_pose_axis_offset", below, above, band),
        }),
    }
}

/// **The relative-curvature rung** at a double root at `at`, whose
/// residual there is `value` (in band) and whose residual's second
/// derivative in the carrier's parameter is `bend` in magnitude, the
/// carrier running at `speed` there: `κ = bend/speed²` levered by the
/// span's length at its least speed, a sagitta (`Margin::sagitta`): how
/// far the carrier departs from the surface's tangent plane over the span
/// the reading reads, as the axis-parallel rung's drift is. Definitely positive is
/// [`Pose::Tangent`], its arc `(π/2)·√(2z/bend)` about `at` in the
/// parameter, `z` the band's zero plus `|value|` plus `noise`: a residual
/// that falls from its extreme no faster than a first harmonic of that
/// curvature leaves the band no nearer than that (`1 − cos ψ ≥ 2ψ²/π²`).
///
/// # Errors
///
/// [`PoseRung::Kappa`] in band: in the gap, or in the zero band.
fn kappa<T: Decide>(
    span: &Span<'_, T>,
    at: T,
    (value, noise): (T, T),
    bend: T,
    speed: T,
    band: Band,
) -> Result<Pose<T>, PoseRefusal> {
    let (t0, t1) = span.params();
    let length = span.speeds().0.get() * (t1 - t0).abs();
    let decided = geom_core::k_stats::decide_reported(
        "curve_face_pose_kappa",
        // Unsourced lever: the span's own length at its least speed, the
        // run the reading reads, as the axis-parallel rung's drift is.
        Margin::sagitta(bend / speed.powi(2), geom_core::Lever::unsourced(length)),
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
    let z = T::from_f64(band.zero()) + value.abs() + noise;
    let half = (T::from_f64(core::f64::consts::FRAC_PI_2)
        * (T::from_f64(2.0) * z / bend.abs()).sqrt())
    .min(T::pi());
    Ok(Pose::Tangent(Touch {
        at,
        point: span.carrier().eval(at),
        arc: (at - half, at + half),
    }))
}

// ------------------------------------------------------------- plane

/// **The plane arm** ([`Door::PlaneCrossing`]).
fn plane<T: Decide>(
    span: &Span<'_, T>,
    surface: &geom::Surface<T>,
    band: Band,
) -> Result<Pose<T>, PoseRefusal> {
    let geom::Surface::Plane { origin, normal, .. } = *surface else {
        return Err(PoseRefusal::Fault(
            "the plane arm was handed a surface that is not a plane",
        ));
    };
    let carrier = span.carrier();
    let (t0, t1) = span.params();
    match (carrier.conic(), carrier) {
        (Some(conic), _) => plane_conic(span, &conic, (origin, normal), band),
        (None, &geom::Curve3::Line(line)) => {
            let lead = line.dir.dot(normal);
            let mid = geom::mid_param(t0, t1);
            let at_mid = geom::plane_offset(origin, normal, line.origin + line.dir * mid);
            let run = (t1 - t0).abs();
            if axis_drift(
                Margin::metered(run, geom_core::InfSpeed::new(lead.abs())),
                band,
            )? {
                return axis_offset(at_mid, lead.abs() * run / T::from_f64(2.0), band);
            }
            Ok(Pose::Isolated(Form(Formed::PlaneLine {
                root: mid - at_mid / lead,
            })))
        }
        (None, _) => Err(PoseRefusal::Fault(
            "the plane arm was handed a carrier that is not a line or a conic",
        )),
    }
}

/// **A conic against a plane**: its height above the plane,
/// `d₀ + R cos(θ − φ)` (`Conic::along`), read on its rungs: the belly
/// (`R − |d₀|`: negative, apart), the parallel gate (`R` in the zero
/// band: the offset `d₀ ± R` decides lies or apart), then the belly's
/// two roots or its graze, whose side `d₀`'s sign decides and whose
/// curvature [`kappa`] reads.
pub(super) fn plane_conic<T: Decide>(
    span: &Span<'_, T>,
    conic: &dyn geom::Conic<T>,
    (origin, normal): (Point3<T>, geom_core::Vec3<T>),
    band: Band,
) -> Result<Pose<T>, PoseRefusal> {
    let (t0, t1) = span.params();
    let near = geom::mid_param(t0, t1);
    let (d0, a, b) = crate::curve_pair::pose::height(conic, origin, normal);
    // powi, never a*a: both amplitudes straddle zero on near-parallel
    // frames, and an interval product's spurious negative low end makes
    // the sqrt refuse.
    let r = (a.powi(2) + b.powi(2)).sqrt();
    // The miss first: a conic wholly on one side never meets the plane,
    // whatever its tilt reads.
    let belly = decide(
        "curve_face_pose_belly_graze",
        Margin::of(r - d0.abs()),
        band,
    );
    if let Ok(Sign::Negative) = belly {
        return Ok(Pose::Apart);
    }
    match decide("curve_face_pose_plane_parallel", Margin::of(r), band)
        .map_err(PoseRefusal::in_band(PoseRung::PlaneParallel))?
    {
        Sign::Zero => {
            let (below, above) =
                crate::roots::spread_readings("curve_face_pose_plane_offset", d0, r, band)
                    .map_err(PoseRefusal::in_band(PoseRung::PlaneParallel))?;
            return match (below.sign, above.sign) {
                (Sign::Positive, _) | (_, Sign::Negative) => Ok(Pose::Apart),
                (Sign::Zero, Sign::Zero) => Ok(Pose::Lies),
                _ => Err(PoseRefusal::InBand {
                    rung: PoseRung::PlaneParallel,
                    diag: straddle("curve_face_pose_plane_offset", below, above, band),
                }),
            };
        }
        Sign::Positive | Sign::Negative => {}
    }
    let phi = geom::Azimuth::of(b, a).near(near);
    match belly.map_err(PoseRefusal::in_band(PoseRung::BellyGraze))? {
        Sign::Negative => Ok(Pose::Apart),
        Sign::Positive => {
            // The roots solve cos(θ − φ) = −d₀/R, clamped against rounding.
            let delta = ((T::zero() - d0) / r)
                .min(T::one())
                .max(T::zero() - T::one())
                .acos();
            let turn = geom_core::period::Period::angle();
            Ok(Pose::Isolated(Form(Formed::PlaneConic {
                roots: [turn.near(phi + delta, near), turn.near(phi - delta, near)],
            })))
        }
        Sign::Zero => {
            // The graze's one root is the extremum nearest the plane: φ
            // with the centre below it, φ + π above.
            let at = match decide_nonzero("curve_face_pose_graze_side", Margin::of(d0), band)
                .map_err(PoseRefusal::in_band(PoseRung::BellyGraze))?
            {
                NonzeroSign::Negative => phi,
                NonzeroSign::Positive => phi + T::pi(),
            };
            let at = geom_core::period::Period::angle().near(at, near);
            let extreme = r - d0.abs();
            kappa(span, at, (extreme, T::zero()), r, conic.speed_at(at), band)
        }
    }
}

// -------------------------------------------------------------- line

/// **The line arm** ([`Door::LineRoots`]).
fn line<T: Decide>(
    span: &Span<'_, T>,
    surface: &geom::Surface<T>,
    band: Band,
) -> Result<Pose<T>, PoseRefusal> {
    let &geom::Curve3::Line(line) = span.carrier() else {
        return Err(PoseRefusal::Fault(
            "the line arm was handed a carrier that is not a line",
        ));
    };
    match *surface {
        geom::Surface::Cylinder {
            origin,
            axis,
            radius,
            ..
        } => line_wall(span, line, (geom::Line { origin, dir: axis }, radius), band),
        geom::Surface::Sphere { center, radius, .. } => {
            line_sphere(span, line, (center, radius), band)
        }
        geom::Surface::Cone {
            apex,
            axis,
            half_angle,
            ..
        } => line_cone(
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
        ),
        // A line never lies on a torus: no ruling of a ring torus is a
        // line.
        geom::Surface::Torus { .. } => Ok(Pose::Isolated(Form(Formed::LineTorus))),
        _ => Err(PoseRefusal::Fault(
            "the line arm was handed a surface that is not a wall, a sphere, a cone or a torus",
        )),
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
    /// A discriminant in the zero band: the vertex, the residual there,
    /// and the residual's second derivative in the line's parameter.
    Graze {
        /// The vertex's parameter.
        at: T,
        /// The residual there.
        value: T,
        /// `|F″|` in the line's parameter.
        bend: T,
    },
}

impl<T: Decide> LineQuadric<T> {
    /// The pose, a graze's curvature read on `span`.
    fn pose(self, span: &Span<'_, T>, band: Band) -> Result<Pose<T>, PoseRefusal> {
        match self {
            Self::Parallel { at_mid, half_drift } => axis_offset(at_mid, half_drift, band),
            Self::Apart => Ok(Pose::Apart),
            Self::Two(roots, _) => Ok(Pose::Isolated(Form(Formed::LineQuadratic { roots }))),
            Self::Graze { at, value, bend } => kappa(
                span,
                at,
                (value, T::zero()),
                bend,
                span.speeds().1.get(),
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
) -> Result<Pose<T>, PoseRefusal> {
    line_wall_read(
        line,
        wall,
        span.params(),
        super::line_quadric::About::Start,
        band,
    )?
    .pose(span, band)
}

/// [`line_wall`]'s span-free half, over the line's run `(t0, t1)`.
pub(super) fn line_wall_read<T: Decide>(
    line: geom::Line<T>,
    (wall, radius): (geom::Line<T>, T),
    (t0, t1): (T, T),
    about: super::line_quadric::About,
    band: Band,
) -> Result<LineQuadric<T>, PoseRefusal> {
    let d = line.dir;
    let dp = d - wall.dir * d.dot(wall.dir);
    let run = (t1 - t0).abs();
    let mid = geom::mid_param(t0, t1);
    let at_mid = wall.distance(line.origin + d * mid) - radius;
    let speed = dp.norm();
    if axis_drift(Margin::metered(run, geom_core::InfSpeed::new(speed)), band)? {
        return Ok(LineQuadric::Parallel {
            at_mid,
            half_drift: speed * run / T::from_f64(2.0),
        });
    }
    let centred = super::line_quadric::wall_quadratic(line, (wall, radius), run, about);
    discriminant(&centred, (dp.norm_squared(), radius), about, band)
}

/// **A line against a sphere**: the discriminant alone ([`discriminant`]):
/// a line never lies on a sphere.
fn line_sphere<T: Decide>(
    span: &Span<'_, T>,
    line: geom::Line<T>,
    sphere: (Point3<T>, T),
    band: Band,
) -> Result<Pose<T>, PoseRefusal> {
    line_sphere_read(line, sphere, super::line_quadric::About::Start, band)?.pose(span, band)
}

/// [`line_sphere`]'s span-free half.
pub(super) fn line_sphere_read<T: Decide>(
    line: geom::Line<T>,
    (center, radius): (Point3<T>, T),
    about: super::line_quadric::About,
    band: Band,
) -> Result<LineQuadric<T>, PoseRefusal> {
    let centred = super::line_quadric::sphere_quadratic(line, (center, radius), about);
    discriminant(&centred, (line.dir.norm_squared(), radius), about, band)
}

/// **The wall's and the sphere's discriminant** (`curve_face_pose_disc`):
/// `disc/a` over `2r`, the depth the line reaches inside the circle
/// across the axis or the sphere, a length. Positive: two roots; Zero: a
/// graze at the vertex, its residual `(|w⊥|² − r²)/2r` exactly quadratic
/// along the line with second derivative `a/r`; Negative: apart.
fn discriminant<T: Decide>(
    centred: &super::line_quadric::Centred<T>,
    (a2, radius): (T, T),
    about: super::line_quadric::About,
    band: Band,
) -> Result<LineQuadric<T>, PoseRefusal> {
    let two_r = T::from_f64(2.0) * radius;
    let disc = centred.disc();
    match decide(
        "curve_face_pose_disc",
        Margin::over_lever(centred.charged(disc, about) / a2, two_r),
        band,
    )
    .map_err(PoseRefusal::in_band(PoseRung::Discriminant))?
    {
        Sign::Positive => {
            let (roots, slack) = centred.roots(disc);
            Ok(LineQuadric::Two(roots, slack))
        }
        Sign::Negative => Ok(LineQuadric::Apart),
        Sign::Zero => {
            let [a, b, c] = centred.coeffs;
            Ok(LineQuadric::Graze {
                at: (T::zero() - b) / a + centred.shift,
                value: (c - b.powi(2) / a) / two_r,
                bend: a / radius,
            })
        }
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
    },
}

/// **A line against a cone**: its quadratic read about its closest
/// approach to the apex held to the run (`super::line_quadric`'s
/// `line_cone_quadratic`), on its rungs, each a length:
///
/// - **the apex** (`curve_face_pose_cone_apex`), the run's distance from
///   it: not definitely positive, the run reaches the apex, at the end
///   the apex is decided at, else at its foot; a generator there
///   (`curve_face_pose_cone_lead` Zero) lies on the cone, and otherwise
///   the line passes through the apex, its roots within the spread
///   (`curve_face_pose_cone_apex_spread`, which must be in the zero band:
///   past it, a meter that refused);
/// - **the lead** off the apex, `A/|d|²` levered by the run: Zero, the
///   line runs parallel to a generator, the method's degenerate form;
/// - **the depth** (`curve_face_pose_cone_depth`), `disc/(|A|·R)`, `R`
///   at least the residual's factor over the run, `disc` held to the
///   shift's rounding: positive, two roots; negative, apart; Zero, a
///   graze at the vertex.
///
/// # Errors
///
/// [`PoseRung::Apex`], [`PoseRung::ConeLead`] or
/// [`PoseRung::Discriminant`] in band; [`PoseRefusal::Meter`] for a
/// spread definitely past the band.
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
    let dd = dir.norm_squared();
    let at = |t: T| w0 + dir * t;
    let (lo, hi) = (t0.min(t1), t0.max(t1));
    let foot = line.param_within(apex, (lo, hi));
    let centred = line_cone_quadratic(line, cone, half_angle, (lo, hi));
    let [a2, b2, c2] = centred.coeffs;
    // Unsourced lever: a length read off the parameter run, not the
    // run's end points; the end points' distance rounds differently.
    let length = geom_core::Lever::unsourced(dir.norm() * (hi - lo));
    let lead = || {
        decide(
            "curve_face_pose_cone_lead",
            Margin::levered(a2 / dd, length),
            band,
        )
        .map_err(PoseRefusal::in_band(PoseRung::ConeLead))
    };
    match decide("curve_face_pose_cone_apex", Margin::norm3(at(foot)), band)
        .map_err(PoseRefusal::in_band(PoseRung::Apex))?
    {
        Sign::Positive => {}
        Sign::Zero | Sign::Negative => {
            // The end the apex is decided at, else the foot: WHICH, not a
            // search a later end may answer past, an end in band ahead
            // refusing.
            let mut at_apex = foot;
            for end in [t0, t1] {
                if decide("curve_face_pose_cone_apex", Margin::norm3(at(end)), band)
                    .map_err(PoseRefusal::in_band(PoseRung::Apex))?
                    == Sign::Zero
                {
                    at_apex = end;
                    break;
                }
            }
            if lead()? == Sign::Zero {
                return Ok(ConeRead::Lies);
            }
            let spread = apex_root_spread(at(at_apex), dir, (axis, half_angle));
            return match decide("curve_face_pose_cone_apex_spread", Margin::of(spread), band)
                .map_err(PoseRefusal::in_band(PoseRung::Apex))?
            {
                Sign::Zero => Ok(ConeRead::Through {
                    at: at_apex,
                    reach: spread / dir.norm(),
                }),
                Sign::Positive | Sign::Negative => Err(PoseRefusal::Meter {
                    rung: PoseRung::Apex,
                    row: "curve_face_pose_cone_apex_spread",
                }),
            };
        }
    }
    if lead()? == Sign::Zero {
        return Ok(ConeRead::Parallel);
    }
    let disc = centred.disc();
    let vertex = centred.shift - b2 / a2;
    let reach = at(lo).norm().max(at(hi).norm()).max(at(vertex).norm());
    match decide(
        "curve_face_pose_cone_depth",
        Margin::of(centred.charged(disc, About::Nearest) / (a2.abs() * reach)),
        band,
    )
    .map_err(PoseRefusal::in_band(PoseRung::Discriminant))?
    {
        Sign::Positive => Ok(ConeRead::Two),
        Sign::Negative => Ok(ConeRead::Apart),
        // `|res| ≥ |G|/R` on the near nappe, `G` the quadratic: its value
        // and curvature over `R`.
        Sign::Zero => Ok(ConeRead::Graze {
            at: vertex,
            value: (c2 - b2.powi(2) / a2) / reach,
            bend: a2.abs() / reach,
        }),
    }
}

/// **A line against a cone** ([`line_cone_read`]), a graze's curvature
/// read on `span`.
fn line_cone<T: Decide>(
    span: &Span<'_, T>,
    line: geom::Line<T>,
    cone: (geom::Line<T>, T),
    band: Band,
) -> Result<Pose<T>, PoseRefusal> {
    Ok(match line_cone_read(line, cone, span.params(), band)? {
        ConeRead::Lies => Pose::Lies,
        ConeRead::Parallel => Pose::Isolated(Form(Formed::LineCone { parallel: true })),
        ConeRead::Through { at, reach } => Pose::Through(Singular {
            at,
            point: cone.0.origin,
            spread: (at - reach, at + reach),
        }),
        ConeRead::Apart => Pose::Apart,
        ConeRead::Two => Pose::Isolated(Form(Formed::LineCone { parallel: false })),
        ConeRead::Graze { at, value, bend } => {
            return kappa(span, at, (value, T::zero()), bend, line.dir.norm(), band);
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
) -> Result<Pose<T>, PoseRefusal> {
    use super::conic_quadric as door;
    let carrier = span.carrier();
    let conic = carrier.conic().ok_or(PoseRefusal::Fault(door::DESYNC))?;
    match (carrier, surface) {
        (
            &geom::Curve3::Circle(circle),
            &geom::Surface::Sphere {
                center: s_center,
                radius: s_radius,
                ..
            },
        ) => {
            let h = crate::circle_sphere_harmonic(&circle, s_center, s_radius);
            let first = super::circle_roots::FirstHarmonic {
                lo: h.lo,
                hi: h.hi,
                cos_part: h.e_u,
                sin_part: h.e_v,
                lo_noise: h.lo_error + h.frame_error,
                hi_noise: h.hi_error + h.frame_error,
                phase_noise: h.phase_error,
            };
            first_harmonic(span, &conic, first, None, band)
        }
        (
            _,
            &geom::Surface::Cone {
                apex,
                axis,
                half_angle,
                ..
            },
        ) => {
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
) -> Result<Pose<T>, PoseRefusal> {
    let hypot = |x: T, y: T| (x.powi(2) + y.powi(2)).sqrt();
    let second = hypot(h.c2, h.s2);
    match decide("curve_face_pose_second_harmonic", Margin::of(second), band) {
        Ok(Sign::Zero) if !ladder_lane::forced() => {
            let (a1, noise) = (hypot(h.c1, h.s1), noise + second);
            let first = super::circle_roots::FirstHarmonic {
                lo: h.c0 - a1,
                hi: h.c0 + a1,
                cos_part: h.c1,
                sin_part: h.s1,
                lo_noise: noise,
                hi_noise: noise,
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
///   graze at that extreme, whose curvature `A₁` [`kappa`] reads.
fn first_harmonic<T: Decide>(
    span: &Span<'_, T>,
    conic: &dyn geom::Conic<T>,
    h: super::circle_roots::FirstHarmonic<T>,
    cone: Option<T>,
    band: Band,
) -> Result<Pose<T>, PoseRefusal> {
    let two = T::from_f64(2.0);
    let noise = h.lo_noise.max(h.hi_noise);
    match decide("curve_face_pose_noise", Margin::of(noise), band) {
        Ok(Sign::Zero | Sign::Negative) => {}
        Ok(Sign::Positive) | Err(_) => {
            return Err(PoseRefusal::Meter {
                rung: PoseRung::Noise,
                row: "curve_face_pose_noise",
            });
        }
    }
    let (c0, a1) = ((h.lo + h.hi) / two, (h.hi - h.lo) / two);
    if let Ok(Sign::Zero) = decide("curve_face_pose_coaxial", Margin::of(a1), band) {
        let (below, above) =
            crate::roots::spread_readings("curve_face_pose_constant", c0, a1 + noise, band)
                .map_err(PoseRefusal::in_band(PoseRung::Coaxial))?;
        match (below.sign, above.sign) {
            (Sign::Positive, _) | (_, Sign::Negative) => return Ok(Pose::Apart),
            (Sign::Zero, Sign::Zero) => {
                let Some(floor) = cone else {
                    return Ok(Pose::Lies);
                };
                // `|res| ≤ |F| / floor`: a constant `F` in the band is the
                // carrier on the cone once its reach, read through the
                // floor, is in the band too.
                let reach = (h.lo.abs().max(h.hi.abs()) + h.lo_noise) / floor;
                return match decide("curve_face_pose_cone_on_surface", Margin::of(reach), band) {
                    Ok(Sign::Zero) => Ok(Pose::Lies),
                    Ok(Sign::Positive | Sign::Negative) | Err(_) => Err(PoseRefusal::Meter {
                        rung: PoseRung::Coaxial,
                        row: "curve_face_pose_cone_on_surface",
                    }),
                };
            }
            _ => {}
        }
    }
    // Each extreme read as the interval its noise spans: ends that
    // disagree read zero, and the zero end is kept for a refusal.
    let extreme = |value, noise| {
        let (below, above) =
            crate::roots::spread_readings("curve_face_pose_extreme", value, noise, band)
                .map_err(PoseRefusal::in_band(PoseRung::Extreme))?;
        Ok::<_, PoseRefusal>(if below.sign == above.sign {
            (below.sign, below)
        } else if above.sign == Sign::Zero {
            (Sign::Zero, above)
        } else {
            (Sign::Zero, below)
        })
    };
    let (lo, lo_read) = extreme(h.lo, h.lo_noise)?;
    if lo == Sign::Positive {
        return Ok(Pose::Apart);
    }
    let (hi, _) = extreme(h.hi, h.hi_noise)?;
    if hi == Sign::Negative {
        return Ok(Pose::Apart);
    }
    let near = geom::mid_param(span.params().0, span.params().1);
    let phi = geom::Azimuth::of(h.sin_part, h.cos_part).near(near);
    let turn = geom_core::period::Period::angle();
    let graze = |at: T, value: T, noise: T| {
        let at = turn.near(at, near);
        kappa(span, at, (value, noise), a1, conic.speed_at(at), band)
    };
    match (lo, hi) {
        (Sign::Negative, Sign::Positive) => Ok(Pose::Isolated(Form(Formed::FirstHarmonic(h)))),
        (Sign::Zero, Sign::Positive) => graze(phi + T::pi(), h.lo, h.lo_noise),
        (Sign::Negative, Sign::Zero) => graze(phi, h.hi, h.hi_noise),
        _ => Err(zero_band(
            PoseRung::Extreme,
            "curve_face_pose_extreme",
            lo_read,
            band,
        )),
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
    cone: (Point3<T>, geom_core::Vec3<T>, T),
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
            rung: PoseRung::Apex,
            diag,
        }),
        Ok(Sign::Positive | Sign::Negative) => Err(PoseRefusal::Meter {
            rung: PoseRung::Apex,
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
    let (t0, t1) = span.params();
    let (mut gap, mut zero) = (None, None);
    for k in [2.0, 0.0, 4.0, 1.0, 3.0] {
        let t = t0 + (t1 - t0) * T::from_f64(k / 4.0);
        let residual = crate::implicit_residual(surface, span.carrier().eval(t));
        match geom_core::k_stats::decide_reported(
            "curve_face_pose_not_lies",
            Margin::of(residual),
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
        center: t_center,
        axis: t_axis,
        major_radius,
        minor_radius,
        ..
    } = *surface
    else {
        return Err(PoseRefusal::Fault(
            "the conic × torus arm was handed a surface that is not a torus",
        ));
    };
    // The doors read only a ring, asked of the convention before any
    // pose: their harmonics take the witness, whose floor on `|F|` per
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
                let (value, spread) = door::coaxial(&circle, surface, minor_radius, &axes);
                let (below, above) = crate::roots::spread_readings(
                    "curve_face_pose_torus_constant",
                    value,
                    spread,
                    band,
                )
                .map_err(PoseRefusal::in_band(PoseRung::Coaxial))?;
                return match (below.sign, above.sign) {
                    (Sign::Positive, _) | (_, Sign::Negative) => Ok(Pose::Apart),
                    (Sign::Zero, Sign::Zero) => Ok(Pose::Lies),
                    _ => Err(PoseRefusal::InBand {
                        rung: PoseRung::Coaxial,
                        diag: straddle("curve_face_pose_torus_constant", below, above, band),
                    }),
                };
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
        (t_center, t_axis, major_radius, minor_radius),
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
    let graze = match crate::roots::spread_readings(
        "curve_face_pose_torus_plane_height",
        c.depth,
        c.charge_h,
        band,
    ) {
        Ok((below, _)) if below.sign == Sign::Positive => None,
        Ok((_, above)) if above.sign == Sign::Negative => return Ok(Pose::Apart),
        Ok((below, _)) => Some(zero_band(
            PoseRung::Contour,
            "curve_face_pose_torus_plane_height",
            below,
            band,
        )),
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
            (_, Sign::Zero | Sign::Positive) => {
                return Err(zero_band(PoseRung::Contour, residual, above, band));
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
