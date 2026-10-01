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
//!   axis-parallel line is on a wall. A circle lying ON the sphere is
//!   always this case — its axis passes through the sphere's centre —
//!   with the constant zero.
//! - `bool_circle_sphere_extreme`, on `c₀ − A₁` and `c₀ + A₁` — the
//!   carrier's minimum and maximum residual. Definitely one-signed is a
//!   [`CircleSphereRoots::Miss`]; definitely straddling is two roots;
//!   either extreme in the zero band is a tangency, which is not a
//!   crossing at any order this lane sees and answers
//!   [`CircleSphereRoots::Uncertain`].
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
    let half_chord = (T::zero() - c0 / a1)
        .max(T::zero() - T::one())
        .min(T::one())
        .acos();
    let mid = (t0 + t1) / two;
    let near_mid = |raw: T| mid + (raw - mid).reduce_periodic_centred(T::tau());
    Ok(CircleSphereRoots::Two([
        near_mid(phi - half_chord),
        near_mid(phi + half_chord),
    ]))
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod tests {
    //! Each pose is one of the door's classifications, checked against
    //! the geometry rather than against the door's own algebra: a root
    //! must put the carrier ON the sphere, and a miss must leave every
    //! sample of the carrier on one side of it.

    use super::*;
    use geom_core::Tol;

    fn band() -> Band {
        Band::linear(Tol::witness()).unwrap()
    }

    /// The unit circle in the `xy` plane about the origin, `û = x̂`.
    fn roots(s_center: Point3<f64>, s_radius: f64, t0: f64, t1: f64) -> CircleSphereRoots<f64> {
        circle_sphere_roots(
            Point3::origin(),
            Vec3::new(0.0, 0.0, 1.0),
            1.0,
            Vec3::new(1.0, 0.0, 0.0),
            t0,
            t1,
            s_center,
            s_radius,
            band(),
        )
        .unwrap()
    }

    fn on_circle(theta: f64) -> Point3<f64> {
        Point3::new(theta.cos(), theta.sin(), 0.0)
    }

    #[test]
    fn a_crossing_sphere_meets_the_carrier_at_two_points_on_it() {
        let (c, r) = (Point3::new(0.8, 0.6, 0.3), 0.5);
        let (t0, t1) = (-1.0, 4.0);
        let CircleSphereRoots::Two(ts) = roots(c, r, t0, t1) else {
            panic!("a sphere through the circle crosses it twice");
        };
        let mid = (t0 + t1) / 2.0;
        for t in ts {
            let off = ((on_circle(t) - c).norm() - r).abs();
            assert!(off < 1e-12, "root {t} lies {off} off the sphere");
            assert!(
                (t - mid).abs() <= core::f64::consts::PI,
                "root {t} is within π of {mid}"
            );
        }
        assert!(
            (ts[0] - ts[1]).abs() > 1e-3,
            "the two roots are distinct: {ts:?}"
        );
    }

    #[test]
    fn a_sphere_clear_of_the_carrier_is_a_miss_on_either_side() {
        // Wholly outside: centre far off. Wholly inside: a big sphere
        // about the origin swallows the circle.
        for (c, r) in [
            (Point3::new(3.0, 0.0, 0.0), 0.5),
            (Point3::new(0.1, 0.0, 0.2), 2.0),
        ] {
            assert!(
                matches!(roots(c, r, 0.0, 6.0), CircleSphereRoots::Miss),
                "sphere {c:?} r {r} misses the circle"
            );
            let side = |t: f64| (on_circle(t) - c).norm() > r;
            let first = side(0.0);
            assert!(
                (0..64).all(|k| side(f64::from(k) * 0.1) == first),
                "the oracle agrees the carrier is one-sided against {c:?} r {r}"
            );
        }
    }

    #[test]
    fn a_sphere_centred_on_the_carrier_axis_is_coaxial() {
        assert!(matches!(
            roots(Point3::new(0.0, 0.0, 0.7), 1.2, 0.0, 6.0),
            CircleSphereRoots::Coaxial
        ));
    }

    #[test]
    fn a_tangent_sphere_is_uncertain_and_a_carrier_on_the_sphere_is_coaxial() {
        // Externally tangent at (1, 0, 0): a touch, not a crossing.
        assert!(matches!(
            roots(Point3::new(1.5, 0.0, 0.0), 0.5, 0.0, 6.0),
            CircleSphereRoots::Uncertain
        ));
        // A sphere the circle lies on, off its equator: every such
        // sphere is centred on the carrier's axis, so the residual is the
        // constant zero, never two roots.
        assert!(matches!(
            roots(Point3::new(0.0, 0.0, 0.5), 1.25_f64.sqrt(), 0.0, 6.0),
            CircleSphereRoots::Coaxial
        ));
    }

    /// The line sibling, through the same door the ray lane uses.
    #[test]
    fn a_line_through_a_sphere_has_two_roots_on_it_and_a_clear_one_misses() {
        use super::super::solid_contain::{WallRoots, line_sphere_roots};
        let (c, r) = (Point3::new(0.0, 0.0, -0.5), 0.3);
        let d = Vec3::new(0.0, 0.0, 1.0);
        let q = Point3::new(0.06, 0.0, -0.92);
        let WallRoots::Two(ts) = line_sphere_roots(q, d, c, r, band()).unwrap() else {
            panic!("the line crosses the sphere");
        };
        for t in ts {
            assert!(
                (((q + d * t) - c).norm() - r).abs() < 1e-12,
                "root {t} on the sphere"
            );
        }
        assert!(matches!(
            line_sphere_roots(Point3::new(0.5, 0.0, 0.0), d, c, r, band()).unwrap(),
            WallRoots::Miss
        ));
        assert!(matches!(
            line_sphere_roots(Point3::new(0.3, 0.0, 0.0), d, c, r, band()).unwrap(),
            WallRoots::Tangent
        ));
    }
}
