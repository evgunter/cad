//! **The circle × sphere root door**: the certified crossings of a
//! CIRCLE carrier with a sphere, beside the circle × torus quartic
//! ([`super::circle_torus`]) and the line × sphere quadratic
//! ([`super::solid_contain::line_sphere_roots`]).
//!
//! # The residual is a first harmonic, so the roots are closed form
//!
//! With the carrier `C(θ) = C₀ + ρ(û cos θ + v̂ sin θ)`, `v̂ = n̂ × û`,
//! and `e = C₀ − c`, the sphere's linearized residual along it is
//!
//! `R(θ) = (|e|² + ρ² − r²)/2r + (ρ/r)(e·û cos θ + e·v̂ sin θ)
//!       = c₀ + A₁ cos(θ − φ)`,
//!
//! `A₁ = (ρ/r)·|(e·û, e·v̂)|`, `φ = atan2(e·v̂, e·û)` — exactly
//! `geom_brep::circle_residual_extremes`'s harmonic triple, whose range
//! `[c₀ − A₁, c₀ + A₁]` is EXACT, not an enclosure. Two circles on one
//! sphere meet it in at most two points, and they are
//! `θ = φ ± acos(−c₀/A₁)`.
//!
//! The half-angle ladder of [`super::circle_torus::half_angle_roots`] is
//! the wrong instrument here, not merely a heavier one: a first harmonic
//! times `(1 + t²)²` carries the factor `(1 + t²)` twice, a repeated
//! complex pair its discriminant reads as a tangency on every pose.
//!
//! # The decisions, all on residual metres
//!
//! - `bool_circle_sphere_noise` — the harmonics' evaluation error
//!   ([`NOISE_ULPS`] half-ulps of the sum of the terms' magnitudes, over
//!   `2r`) definitely past the escalation threshold refuses: the
//!   representation cannot resolve what the band asks of it. A rounding
//!   estimate on `f64`; the `Interval` lane carries its enclosure through
//!   every stage and needs no meter to be sound.
//! - `bool_circle_sphere_coaxial` — the swing `A₁` in the zero band: the
//!   residual is constant along the carrier (its centre's offset from
//!   the sphere's centre is along its axis), answered
//!   [`CircleSphereRoots::Coaxial`] for the caller to read, as the
//!   axis-parallel line is on a wall.
//! - `bool_circle_sphere_extreme`, on `c₀ − A₁` and `c₀ + A₁` — the
//!   carrier's minimum and maximum residual. Definitely one-signed is a
//!   [`CircleSphereRoots::Miss`]; definitely straddling is two roots;
//!   either extreme in the zero band is a tangency (or a circle lying ON
//!   the sphere, both extremes zero), which is not a crossing at any
//!   order this lane sees and answers [`CircleSphereRoots::Uncertain`].
//! - `bool_circle_sphere_root_slack` — each root moves by
//!   `noise / |R′(θ)|` radians under the harmonics' error, with
//!   `|R′| = √(A₁² − c₀²)` at both roots; that arc length must be inside
//!   the band, or the span and trim decisions the caller makes on the
//!   point are made on the wrong point.
//!
//! Two DISTINCT certified roots therefore certify that the carrier does
//! not lie on the sphere — the fact the reduction's `(Zero, Zero)` chord
//! arm leans on to separate a chord from an on-carrier edge.

use geom_core::{Band, Decide, Indeterminate, Margin, Point3, Sign, Vec3};

use crate::validate::decide;

/// What the certified circle × sphere roots say about a whole carrier.
#[derive(Debug, Clone, Copy)]
pub(super) enum CircleSphereRoots<T> {
    /// The residual is constant along the carrier.
    Coaxial,
    /// The carrier misses the sphere: wholly outside or wholly inside.
    Miss,
    /// No certain answer — a tangency, a carrier on the sphere, or a
    /// representation that cannot resolve the band.
    Uncertain,
    /// Two distinct roots, each within `π` of the arc's midpoint so it
    /// compares with the arc `[t₀, t₁]` the caller passed without
    /// wrapping.
    Two([T; 2]),
}

/// How many half-ulps of the term bound the harmonics' evaluation error
/// is charged: each harmonic is a squared norm or a dot product and a
/// short sum, every rounding of which the term bound dominates; sixteen
/// is that chain's count with room, as for the torus.
const NOISE_ULPS: f64 = 16.0;

/// The certified crossings of the circle carrier
/// `center + radius·(u_ref cos θ + (axis × u_ref) sin θ)` with the
/// sphere `(s_center, s_radius)`, reported within `π` of the midpoint
/// of `[t0, t1]` (module docs).
///
/// # Errors
///
/// [`Indeterminate`] — an extreme residual in the band's escalation
/// gap. An in-band coaxial test or root slack is not an error: the
/// first falls through to the extremes, the second answers `Uncertain`.
#[allow(clippy::too_many_arguments)]
pub(super) fn circle_sphere_roots<T: Decide>(
    center: Point3<T>,
    axis: Vec3<T>,
    radius: T,
    u_ref: Vec3<T>,
    t0: T,
    t1: T,
    s_center: Point3<T>,
    s_radius: T,
    band: Band,
) -> Result<CircleSphereRoots<T>, Indeterminate> {
    let two = T::from_f64(2.0);
    let two_r = two * s_radius;
    let e = center - s_center;
    let (eu, ev) = (e.dot(u_ref), e.dot(axis.cross(u_ref)));
    let offset = (eu.powi(2) + ev.powi(2)).sqrt();
    let c0 = (e.norm_squared() + radius.powi(2) - s_radius.powi(2)) / two_r;
    let a1 = two * radius * offset / two_r;
    let terms = e.norm_squared() + radius.powi(2) + s_radius.powi(2) + two * radius * offset;
    let noise = T::from_f64(NOISE_ULPS * f64::EPSILON * 0.5) * terms / two_r;
    if let Ok(Sign::Positive) = decide("bool_circle_sphere_noise", Margin::of(noise), band) {
        return Ok(CircleSphereRoots::Uncertain);
    }
    if let Ok(Sign::Zero) = decide("bool_circle_sphere_coaxial", Margin::of(a1), band) {
        return Ok(CircleSphereRoots::Coaxial);
    }
    let lo = decide("bool_circle_sphere_extreme", Margin::of(c0 - a1), band)?;
    if lo == Sign::Positive {
        return Ok(CircleSphereRoots::Miss);
    }
    let hi = decide("bool_circle_sphere_extreme", Margin::of(c0 + a1), band)?;
    if hi == Sign::Negative {
        return Ok(CircleSphereRoots::Miss);
    }
    if (lo, hi) != (Sign::Negative, Sign::Positive) {
        return Ok(CircleSphereRoots::Uncertain);
    }
    // |R′| at either root: A₁·|sin(θ − φ)| = √(A₁² − c₀²), factored so
    // that both factors are the definite extremes just decided.
    let slope = ((a1 - c0) * (a1 + c0)).max(T::zero()).sqrt();
    match decide(
        "bool_circle_sphere_root_slack",
        Margin::of(radius * noise / slope),
        band,
    ) {
        Ok(Sign::Positive) => return Ok(CircleSphereRoots::Uncertain),
        Ok(Sign::Zero | Sign::Negative) | Err(_) => {}
    }
    let phi = ev.atan2(eu);
    let half_chord = (T::zero() - c0 / a1).max(T::zero() - T::one()).min(T::one()).acos();
    let mid = (t0 + t1) / two;
    let near_mid = |raw: T| mid + (raw - mid).reduce_periodic_centred(T::tau());
    Ok(CircleSphereRoots::Two([
        near_mid(phi - half_chord),
        near_mid(phi + half_chord),
    ]))
}
