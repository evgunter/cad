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
//! `A₁ = (ρ/r)·|(e·û, e·v̂)|`, `φ = atan2(e·v̂, e·û)` —
//! `geom_brep::circle_sphere_harmonic`, the harmonic triple
//! `geom_brep::circle_residual_extremes` reads too, whose range
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
//!   (`NOISE_ULPS` half-ulps of the sum of the terms' magnitudes, over
//!   `2r`) definitely past the escalation threshold, or not readable at
//!   all, refuses: the representation cannot resolve what the band asks
//!   of it. A rounding estimate on `f64`, run on every scalar: the
//!   `Interval` lane carries its own enclosure and needs no meter to be
//!   sound, but the meter still reads there and can refuse a pose the
//!   enclosures alone would answer.
//! - `bool_circle_sphere_coaxial` — the swing `A₁` in the zero band: the
//!   residual is constant along the carrier (its centre's offset from
//!   the sphere's centre is along its axis), so `c₀` alone decides it:
//!   definite is a [`CircleSphereRoots::Miss`], zero is
//!   [`CircleSphereRoots::Coaxial`] — the circle lies ON the sphere,
//!   which every circle on a sphere does with its axis through the
//!   sphere's centre — for the caller to read, as the axis-parallel line
//!   is on a wall.
//! - `bool_circle_sphere_extreme`, on `c₀ − A₁` and `c₀ + A₁` — the
//!   carrier's minimum and maximum residual. Definitely one-signed is a
//!   [`CircleSphereRoots::Miss`]; definitely straddling is two roots;
//!   either extreme in the zero band is a tangency, which is not a
//!   crossing at any order this lane sees and answers
//!   [`CircleSphereRoots::Uncertain`].
//! - `bool_circle_sphere_root_slack` — each root moves by
//!   `noise / |R′(θ)|` radians under the harmonics' error, with
//!   `|R′| = √(A₁² − c₀²)` at both roots; that arc length must be
//!   definitely inside the band, or the span and trim decisions the
//!   caller makes on the point are made on the wrong point.
//!
//! Two DISTINCT certified roots therefore certify that the carrier does
//! not lie on the sphere — the fact the reduction's `(Zero, Zero)` chord
//! arm leans on to separate a chord from an on-carrier edge.

use geom_core::{Band, Decide, Margin, Sign};

use super::circle_torus::rounding_charge;
use super::{BooleanDecision, BooleanError};
use crate::validate::decide;

/// What the certified circle × sphere roots say about a whole carrier.
#[derive(Debug, Clone, Copy)]
pub(super) enum CircleSphereRoots<T> {
    /// The residual is constant along the carrier and ZERO: the circle
    /// lies on the sphere. A constant definite residual is a
    /// [`Self::Miss`].
    Coaxial,
    /// The carrier misses the sphere: wholly outside or wholly inside.
    Miss,
    /// No certain answer — a tangency, or a representation that cannot
    /// resolve the band.
    Uncertain,
    /// Two distinct roots, each within `π` of the arc's midpoint so it
    /// compares with the arc `[t₀, t₁]` the caller passed without
    /// wrapping.
    Two([T; 2]),
}

/// The certified crossings of the `carrier` circle with the `sphere`,
/// reported within `π` of the midpoint of `[t0, t1]` (module docs).
///
/// # Errors
///
/// [`BooleanError::ClassificationInvariant`] when `carrier` is not a
/// circle or `sphere` not a sphere — the caller dispatched on those kinds,
/// so a mismatch is a desync, never an answer. A coincidence escalation
/// when the constant residual of a coaxial carrier, or an extreme
/// residual, lies in the band's escalation gap. An escalated
/// noise or root-slack reading is NOT an error: it answers `Uncertain`,
/// as a definitely excessive one does — a meter that cannot be read
/// does not license the roots it meters. An escalated coaxial test
/// falls through to the extremes, which decide the same carrier.
pub(super) fn circle_sphere_roots<T: Decide>(
    carrier: &geom::Curve3<T>,
    t0: T,
    t1: T,
    sphere: &geom::Surface<T>,
    band: Band,
) -> Result<CircleSphereRoots<T>, BooleanError> {
    let (
        &geom::Curve3::Circle {
            center,
            axis,
            radius,
            u_ref,
        },
        &geom::Surface::Sphere {
            center: s_center,
            radius: s_radius,
            ..
        },
    ) = (carrier, sphere)
    else {
        return Err(BooleanError::ClassificationInvariant {
            what: "the circle × sphere root door was handed a carrier that is not a circle \
                   or a surface that is not a sphere",
        });
    };
    let decide = |row, m, band| {
        decide(row, m, band).map_err(|diag| BooleanError::Escalated {
            decision: BooleanDecision::ArcSphereRoots,
            diag,
        })
    };
    let geom_brep::CircleSphereHarmonic {
        c0,
        a1,
        e_u,
        e_v,
        terms,
    } = geom_brep::circle_sphere_harmonic(center, axis, radius, u_ref, s_center, s_radius);
    let two = T::from_f64(2.0);
    let noise = rounding_charge(terms) / (two * s_radius);
    match decide("bool_circle_sphere_noise", Margin::of(noise), band) {
        Ok(Sign::Zero | Sign::Negative) => {}
        Ok(Sign::Positive) | Err(_) => return Ok(CircleSphereRoots::Uncertain),
    }
    if let Ok(Sign::Zero) = decide("bool_circle_sphere_coaxial", Margin::of(a1), band) {
        // A constant residual: its one value decides the whole carrier.
        return Ok(
            match decide("bool_circle_sphere_extreme", Margin::of(c0), band)? {
                Sign::Zero => CircleSphereRoots::Coaxial,
                Sign::Positive | Sign::Negative => CircleSphereRoots::Miss,
            },
        );
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
        Ok(Sign::Zero | Sign::Negative) => {}
        Ok(Sign::Positive) | Err(_) => return Ok(CircleSphereRoots::Uncertain),
    }
    let phi = e_v.atan2(e_u);
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
    use geom_core::{Bounds, Interval, Point3, Real, Tol, Vec3};

    fn band() -> Band {
        Band::linear(Tol::witness()).unwrap()
    }

    fn sphere<T: Real>(c: [f64; 3], r: f64) -> geom::Surface<T> {
        geom::Surface::Sphere {
            center: Point3::new(T::from_f64(c[0]), T::from_f64(c[1]), T::from_f64(c[2])),
            radius: T::from_f64(r),
            axis: Vec3::new(T::zero(), T::one(), T::zero()),
            u_ref: Vec3::new(T::one(), T::zero(), T::zero()),
        }
    }

    /// The circle of radius `rho` in the `xy` plane about the origin,
    /// `û = x̂`.
    fn circle<T: Real>(rho: f64) -> geom::Curve3<T> {
        geom::Curve3::Circle {
            center: Point3::origin(),
            axis: Vec3::new(T::zero(), T::zero(), T::one()),
            radius: T::from_f64(rho),
            u_ref: Vec3::new(T::one(), T::zero(), T::zero()),
        }
    }

    fn roots(c: [f64; 3], r: f64, t0: f64, t1: f64) -> CircleSphereRoots<f64> {
        circle_sphere_roots(&circle(1.0), t0, t1, &sphere(c, r), band()).unwrap()
    }

    fn on_circle(theta: f64) -> Point3<f64> {
        Point3::new(theta.cos(), theta.sin(), 0.0)
    }

    fn off_sphere(theta: f64, c: [f64; 3], r: f64) -> f64 {
        (on_circle(theta) - Point3::from_array(c)).norm() - r
    }

    /// The default band, for the rows whose poses are sized to it.
    fn default_band() -> bool {
        if Tol::witness().get().eps == 1e-9 {
            return true;
        }
        test_utils::vacuity::stood_down(
            "non-default eps",
            "the meter poses are sized to the default band's escalation threshold",
        );
        false
    }

    #[test]
    fn a_crossing_sphere_meets_the_carrier_at_two_points_on_it() {
        let (c, r) = ([0.8, 0.6, 0.3], 0.5);
        let (t0, t1) = (-1.0, 4.0);
        let CircleSphereRoots::Two(ts) = roots(c, r, t0, t1) else {
            panic!("a sphere through the circle crosses it twice");
        };
        let mid = (t0 + t1) / 2.0;
        for t in ts {
            let off = off_sphere(t, c, r).abs();
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

    /// **The window wraps.** An arc whose span straddles `θ = 0` from
    /// above `π` — `[5, 7.5]` — gets its roots reported inside that span
    /// rather than at their principal angles, so the caller's span test
    /// compares like with like.
    #[test]
    fn roots_are_reported_within_pi_of_an_arc_past_the_branch_cut() {
        let (c, r) = ([1.0, -0.1, 0.0], 0.3);
        let (t0, t1) = (5.0, 7.5);
        let CircleSphereRoots::Two(ts) = roots(c, r, t0, t1) else {
            panic!("the sphere straddles the circle at θ ≈ 0");
        };
        for t in ts {
            assert!(
                t0 < t && t < t1,
                "root {t} reported inside the arc [{t0}, {t1}]"
            );
            assert!(off_sphere(t, c, r).abs() < 1e-12, "root {t} on the sphere");
        }
    }

    #[test]
    fn a_sphere_clear_of_the_carrier_is_a_miss_on_either_side() {
        // Wholly outside: centre far off. Wholly inside: a big sphere
        // about the origin swallows the circle.
        for (c, r) in [([3.0, 0.0, 0.0], 0.5), ([0.1, 0.0, 0.2], 2.0)] {
            assert!(
                matches!(roots(c, r, 0.0, 6.0), CircleSphereRoots::Miss),
                "sphere {c:?} r {r} misses the circle"
            );
            let first = off_sphere(0.0, c, r) > 0.0;
            assert!(
                (0..64).all(|k| (off_sphere(f64::from(k) * 0.1, c, r) > 0.0) == first),
                "the oracle agrees the carrier is one-sided against {c:?} r {r}"
            );
        }
    }

    /// **A coaxial carrier is decided by its one residual.** Centred on
    /// the circle's axis the residual is constant: definitely off the
    /// sphere is a miss, on it is `Coaxial`.
    #[test]
    fn a_coaxial_carrier_is_a_miss_off_the_sphere_and_coaxial_on_it() {
        assert!(matches!(
            roots([0.0, 0.0, 0.7], 1.6, 0.0, 6.0),
            CircleSphereRoots::Miss
        ));
        assert!(matches!(
            roots([0.0, 0.0, 0.7], 0.5, 0.0, 6.0),
            CircleSphereRoots::Miss
        ));
        assert!(matches!(
            roots([0.0, 0.0, 0.5], 1.25_f64.sqrt(), 0.0, 6.0),
            CircleSphereRoots::Coaxial
        ));
    }

    #[test]
    fn a_tangent_sphere_is_uncertain() {
        // Externally tangent at (1, 0, 0): a touch, not a crossing.
        assert!(matches!(
            roots([1.5, 0.0, 0.0], 0.5, 0.0, 6.0),
            CircleSphereRoots::Uncertain
        ));
    }

    /// **The noise meter, isolated.** On a crossing the slack reads at
    /// least the noise (`|R′| ≤ ρ` there, so `ρ·noise/|R′| ≥ noise`), so a
    /// crossing pose cannot tell the two meters apart. A MISS can: it
    /// returns before the slack is read. A unit circle against a unit
    /// sphere far away is a definite miss whose harmonics are built from
    /// terms of order `|e|²`: at 2·10⁵ m their rounding is definitely past
    /// the band (`Positive`), at 2000 m it lies in the band's escalation
    /// gap (`Err`). Either reading refuses; without its arm, each pose
    /// answers `Miss`.
    #[test]
    fn the_noise_meter_refuses_a_definite_and_an_unreadable_reading() {
        if !default_band() {
            return;
        }
        for far in [2e5, 2000.0] {
            let got = circle_sphere_roots(
                &circle(1.0),
                -1.0,
                1.0,
                &sphere([far, 0.0, 0.0], 1.0),
                band(),
            )
            .unwrap();
            assert!(
                matches!(got, CircleSphereRoots::Uncertain),
                "a sphere {far} m off: the noise meter refuses, got {got:?}"
            );
        }
    }

    /// **The root-slack meter refuses a root it cannot place.** A
    /// radius-100 circle dipping `3e-8` into a unit sphere: both
    /// extremes are definite, so the roots exist, but the residual's
    /// slope at them is so shallow that the harmonics' rounding moves
    /// each by more than the band. Without the meter this pose answers
    /// two roots.
    #[test]
    fn the_root_slack_meter_refuses_a_shallow_crossing() {
        if !default_band() {
            return;
        }
        let rho = 100.0;
        let got = circle_sphere_roots(
            &circle(rho),
            -1.0,
            1.0,
            &sphere([rho + 1.0 - 3e-8, 0.0, 0.0], 1.0),
            band(),
        )
        .unwrap();
        assert!(
            matches!(got, CircleSphereRoots::Uncertain),
            "the root-slack meter refuses: {got:?}"
        );
    }

    /// **The root-slack meter refuses an unreadable reading.** The same
    /// shallow crossing dipping `4e-5` instead: the slack (`≈ 4e-9`) lies
    /// in the band's escalation gap while the noise is in its zero band.
    /// Without the `Err` arm this pose answers two roots.
    #[test]
    fn the_root_slack_meter_refuses_a_reading_in_the_band_gap() {
        if !default_band() {
            return;
        }
        let rho = 100.0;
        let got = circle_sphere_roots(
            &circle(rho),
            -1.0,
            1.0,
            &sphere([rho + 1.0 - 4e-5, 0.0, 0.0], 1.0),
            band(),
        )
        .unwrap();
        assert!(
            matches!(got, CircleSphereRoots::Uncertain),
            "the root-slack meter refuses: {got:?}"
        );
    }

    /// **A desynced caller is a kernel defect, loudly.** The door is
    /// dispatched on a circle against a sphere; anything else reaching it
    /// is the caller's broken invariant, never an answer.
    #[test]
    fn a_non_circle_or_non_sphere_is_a_classification_invariant() {
        let line = geom::Curve3::Line {
            origin: Point3::origin(),
            dir: Vec3::new(1.0, 0.0, 0.0),
        };
        let plane = geom::Surface::Plane {
            origin: Point3::origin(),
            normal: Vec3::new(0.0, 0.0, 1.0),
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        };
        for (carrier, surface) in [
            (&line, &sphere::<f64>([0.0; 3], 1.0)),
            (&circle::<f64>(1.0), &plane),
        ] {
            let got = circle_sphere_roots(carrier, 0.0, 1.0, surface, band());
            assert!(
                matches!(got, Err(BooleanError::ClassificationInvariant { .. })),
                "a wrong kind refuses as a kernel invariant, got {got:?}"
            );
        }
    }

    /// The interval lane: every root enclosure contains the oracle's
    /// root, found by bisecting the distance to the sphere.
    #[test]
    fn the_interval_lane_encloses_the_oracle_roots() {
        let (c, r) = ([0.8, 0.6, 0.3], 0.5);
        let got = circle_sphere_roots::<Interval>(
            &circle(1.0),
            Interval::from_f64(-1.0),
            Interval::from_f64(4.0),
            &sphere(c, r),
            band(),
        )
        .unwrap();
        let CircleSphereRoots::Two(ts) = got else {
            panic!("interval lane: expected two roots, got {got:?}");
        };
        // The f64 sign changes over the arc, each bisected to the bit.
        let f = |t: f64| off_sphere(t, c, r);
        let mut want = Vec::new();
        let steps = 4000;
        for k in 0..steps {
            let (mut a, mut b) = (
                -1.0 + 5.0 * f64::from(k) / f64::from(steps),
                -1.0 + 5.0 * f64::from(k + 1) / f64::from(steps),
            );
            if f(a).signum() == f(b).signum() {
                continue;
            }
            for _ in 0..80 {
                let m = (a + b) / 2.0;
                if f(m).signum() == f(a).signum() {
                    a = m;
                } else {
                    b = m;
                }
            }
            want.push((a + b) / 2.0);
        }
        assert_eq!(want.len(), 2, "the oracle finds two roots");
        for w in want {
            assert!(
                ts.iter()
                    .any(|t| t.lo() - 1e-12 <= w && w <= t.hi() + 1e-12 && t.hi() - t.lo() < 1e-9),
                "oracle root {w} in no tight enclosure {ts:?}"
            );
        }
    }

    /// The line sibling, through the same door the ray lane uses — with
    /// a unit and a NON-unit direction, whose roots are in the
    /// direction's own parameter.
    #[test]
    fn a_line_through_a_sphere_has_two_roots_on_it_and_a_clear_one_misses() {
        use super::super::solid_contain::{WallRoots, line_sphere_roots};
        let (c, r) = (Point3::new(0.0, 0.0, -0.5), 0.3);
        let q = Point3::new(0.06, 0.0, -0.92);
        for d in [Vec3::new(0.0, 0.0, 1.0), Vec3::new(0.0, 0.0, 2.5)] {
            let WallRoots::Two(ts) = line_sphere_roots(q, d, c, r, band()).unwrap() else {
                panic!("the line along {d:?} crosses the sphere");
            };
            for t in ts {
                assert!(
                    (((q + d * t) - c).norm() - r).abs() < 1e-12,
                    "root {t} along {d:?} on the sphere"
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
}
