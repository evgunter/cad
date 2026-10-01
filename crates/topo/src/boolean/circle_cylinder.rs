//! **The circle × cylinder root door**: the certified crossings of a
//! CIRCLE carrier with a cylinder wall, beside the circle × sphere
//! ([`super::circle_sphere`]) and circle × torus
//! ([`super::circle_torus`]) doors. It owns no root machinery of its
//! own: it computes the wall's harmonics and hands them to one of those
//! doors' shared halves.
//!
//! # The residual is a degree-2 trigonometric polynomial
//!
//! With the carrier `C(θ) = C₀ + ρ(û cos θ + v̂ sin θ)`, `v̂ = n̂ × û`,
//! the wall `(o, â, r)`, `⊥x = x − â(â·x)`, `e = ⊥(C₀ − o)` and
//! `w(θ) = e + ρ(⊥û cos θ + ⊥v̂ sin θ)`, the wall's implicit
//! `F = |w|² − r²` is
//!
//! `F(θ) = |e|² + ρ²(|⊥û|² + |⊥v̂|²)/2 − r²
//!        + 2ρ(e·⊥û cos θ + e·⊥v̂ sin θ)
//!        + ρ²((|⊥û|² − |⊥v̂|²)/2 cos 2θ + ⊥û·⊥v̂ sin 2θ)`,
//!
//! and the linearized residual is EXACTLY `F / 2r` everywhere — the
//! noise meter's floor on `|F|` per metre of residual, `2r`, is an
//! identity rather than a neighbourhood bound. At most four crossings
//! per turn.
//!
//! # Two arms, by the carrier's tilt
//!
//! `bool_circle_cylinder_tilt` decides `ρ·|n̂ × â|` (metres: the
//! amplitude of the carrier's height along the wall's axis).
//!
//! - **Zero — the circle is square to the axis.** `⊥û`, `⊥v̂` are then
//!   orthonormal, the second harmonic vanishes, and `F / 2r` is the
//!   residual against the wall's cross-section circle — the circle ×
//!   sphere door's first harmonic, decided by its exact extremes
//!   [`super::circle_sphere::first_harmonic_roots`] under this door's
//!   rows. The dropped second harmonic (`≤ tilt²/4r` inside the zero
//!   band) is charged to its noise. A constant residual is this arm's
//!   to answer: `Coaxial` when it is zero (a rim circle of the same
//!   wall), a miss when it is definite. The quartic would read that
//!   pose as a tangency, and a carrier passing just clear of the wall
//!   too (the sphere door's module docs say why).
//! - **Otherwise** (definite, or in the band's gap) the five harmonics
//!   go to the surface-generic half-angle ladder
//!   [`super::circle_torus::half_angle_roots`], with the wall's
//!   `f_per_metre = 2r` and the carrier's own `2ρ` as its lever.
//!
//! The meters keep their doors' postures. The parallel arm's REFUSE an
//! unreadable reading (an escalation in the band's gap, or a NaN
//! slope), the sphere door's. The ladder's refuse only a reading
//! definitely past the band, and pass one in the gap — the posture
//! `work/germ/circle-torus-meters-accept-an-unreadable-reading.md` holds
//! open, measured there against what refusing costs at `ε = 1e-12`.
//!
//! The noise meter's term bound is `(|C₀ − o| + ρ)² + r²`, which
//! dominates every term the harmonics are built from (`|e| ≤ |C₀ − o|`,
//! `|⊥û|, |⊥v̂| ≤ 1`), with the rounding of the projection itself, which
//! is relative to the unprojected offset.

use geom_core::{Band, Decide, Margin, Sign};

use super::circle_sphere::{FirstHarmonic, FirstHarmonicRoots, FirstHarmonicRows};
use super::circle_torus::{CircleRoots, HalfAngleFrame, HalfAngleRows, Harmonics, rounding_charge};
use super::solid_contain::QuarticRows;
use super::{BooleanDecision, BooleanError};
use crate::validate::decide;

/// The parallel arm's rows ([`super::circle_sphere::first_harmonic_roots`]).
const CIRCLE_CYLINDER_FIRST_ROWS: FirstHarmonicRows = FirstHarmonicRows {
    noise: "bool_circle_cylinder_noise",
    coaxial: "bool_circle_cylinder_coaxial",
    extreme: "bool_circle_cylinder_extreme",
    root_slack: "bool_circle_cylinder_root_slack",
    decision: BooleanDecision::ArcCylinderRoots,
};

/// The general arm's rows ([`super::circle_torus::half_angle_roots`]).
const CIRCLE_CYLINDER_ROWS: HalfAngleRows = HalfAngleRows {
    pole: "bool_circle_cylinder_pole",
    conditioning: "bool_circle_cylinder_pole_conditioning",
    noise: "bool_circle_cylinder_noise",
    root_slack: "bool_circle_cylinder_root_slack",
    quartic: QuarticRows {
        disc: "bool_circle_cylinder_disc",
        shape: "bool_circle_cylinder_shape",
        depth: "bool_circle_cylinder_depth",
        odd: "bool_circle_cylinder_odd",
        split: "bool_circle_cylinder_split",
        split_lead: "bool_circle_cylinder_split_lead",
    },
};

/// The certified crossings of the `carrier` circle with the `wall`,
/// reported within `π` of the midpoint of `[t0, t1]` (module docs).
///
/// # Errors
///
/// [`BooleanError::ClassificationInvariant`] when `carrier` is not a
/// circle or `wall` not a cylinder — the caller dispatched on those
/// kinds, so a mismatch is a desync, never an answer. An escalation as
/// [`BooleanDecision::ArcCylinderRoots`] for an in-band classifying sign:
/// an extreme or constant residual of the parallel arm, or a rung of
/// the ladder. An in-band tilt is not an error: it takes the general
/// arm, which needs no tilt to be definite.
pub(super) fn circle_cylinder_roots<T: Decide>(
    carrier: &geom::Curve3<T>,
    t0: T,
    t1: T,
    wall: &geom::Surface<T>,
    band: Band,
) -> Result<CircleRoots<T>, BooleanError> {
    let (
        &geom::Curve3::Circle {
            center,
            axis,
            radius,
            u_ref,
        },
        &geom::Surface::Cylinder {
            origin,
            axis: w_axis,
            radius: w_radius,
            ..
        },
    ) = (carrier, wall)
    else {
        return Err(BooleanError::ClassificationInvariant {
            what: "the circle × cylinder root door was handed a carrier that is not a circle \
                   or a surface that is not a cylinder",
        });
    };
    let two = T::from_f64(2.0);
    let half = T::from_f64(0.5);
    let perp = |x: geom_core::Vec3<T>| {
        let along = w_axis.dot(x);
        x - w_axis * along
    };
    let v_ref = axis.cross(u_ref);
    let d = center - origin;
    let e = perp(d);
    let (up, vp) = (perp(u_ref), perp(v_ref));
    let (e_u, e_v) = (e.dot(up), e.dot(vp));
    let (uu, vv) = (up.norm_squared(), vp.norm_squared());
    let rho2 = radius.powi(2);
    let f = Harmonics {
        c0: e.norm_squared() + rho2 * (uu + vv) * half - w_radius.powi(2),
        c1: two * radius * e_u,
        s1: two * radius * e_v,
        c2: rho2 * (uu - vv) * half,
        s2: rho2 * up.dot(vp),
    };
    let noise = rounding_charge((d.norm() + radius).powi(2) + w_radius.powi(2));
    let f_per_metre = two * w_radius;
    let tilt = radius * axis.cross(w_axis).norm();
    if let Ok(Sign::Zero) = decide("bool_circle_cylinder_tilt", Margin::of(tilt), band) {
        let hypot = |x: T, y: T| (x.powi(2) + y.powi(2)).sqrt();
        let first = FirstHarmonic {
            c0: f.c0 / f_per_metre,
            a1: hypot(f.c1, f.s1) / f_per_metre,
            e_u,
            e_v,
            noise: (noise + hypot(f.c2, f.s2)) / f_per_metre,
        };
        return Ok(
            match super::circle_sphere::first_harmonic_roots(
                &first,
                radius,
                t0,
                t1,
                &CIRCLE_CYLINDER_FIRST_ROWS,
                band,
            )? {
                FirstHarmonicRoots::Coaxial => CircleRoots::Coaxial,
                FirstHarmonicRoots::Miss => CircleRoots::Miss,
                FirstHarmonicRoots::Uncertain => CircleRoots::Uncertain,
                FirstHarmonicRoots::Two([a, b]) => CircleRoots::Certified {
                    count: 2,
                    thetas: [a, b, T::zero(), T::zero()],
                },
            },
        );
    }
    let point_at = |theta: T| {
        let (s, c) = theta.sin_cos();
        center + u_ref * (radius * c) + v_ref * (radius * s)
    };
    super::circle_torus::half_angle_roots(
        &f,
        |theta| geom_brep::implicit_residual(wall, point_at(theta)),
        HalfAngleFrame {
            t0,
            t1,
            radius,
            lever: two * radius,
            noise,
            f_per_metre,
        },
        &CIRCLE_CYLINDER_ROWS,
        band,
    )
    .map(CircleRoots::from)
    .map_err(|diag| BooleanError::Escalated {
        decision: BooleanDecision::ArcCylinderRoots,
        diag,
    })
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod tests {
    //! Each pose is checked against the geometry, not the door's own
    //! algebra: a certified root must put the carrier ON the wall, and
    //! its count must be the count of sign changes of the direct
    //! residual along the carrier.

    use super::*;
    use geom_core::{Bounds, Interval, Point3, Real, Tol, Vec3};

    fn band() -> Band {
        Band::linear(Tol::witness()).unwrap()
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

    /// The wall of radius `r` about the `z` axis through `(x, y, 0)`.
    fn wall<T: Real>(x: f64, y: f64, r: f64) -> geom::Surface<T> {
        geom::Surface::Cylinder {
            origin: Point3::new(T::from_f64(x), T::from_f64(y), T::zero()),
            axis: Vec3::new(T::zero(), T::zero(), T::one()),
            radius: T::from_f64(r),
            u_ref: Vec3::new(T::one(), T::zero(), T::zero()),
        }
    }

    /// A circle of radius `rho` about `c`, its axis `n` (unit) and its
    /// `û` the unit `u` (orthogonal to `n`).
    #[derive(Clone, Copy)]
    struct Pose {
        c: [f64; 3],
        n: [f64; 3],
        u: [f64; 3],
        rho: f64,
    }

    impl Pose {
        fn carrier<T: Real>(self) -> geom::Curve3<T> {
            let v =
                |a: [f64; 3]| Vec3::new(T::from_f64(a[0]), T::from_f64(a[1]), T::from_f64(a[2]));
            geom::Curve3::Circle {
                center: Point3::new(
                    T::from_f64(self.c[0]),
                    T::from_f64(self.c[1]),
                    T::from_f64(self.c[2]),
                ),
                axis: v(self.n),
                radius: T::from_f64(self.rho),
                u_ref: v(self.u),
            }
        }

        fn at(self, theta: f64) -> Point3<f64> {
            let n = Vec3::from_array(self.n);
            let u = Vec3::from_array(self.u);
            let (s, c) = theta.sin_cos();
            Point3::from_array(self.c) + (u * c + n.cross(u) * s) * self.rho
        }
    }

    /// Square to the axis: a circle in the plane `z = h`.
    fn flat(x: f64, y: f64, h: f64, rho: f64) -> Pose {
        Pose {
            c: [x, y, h],
            n: [0.0, 0.0, 1.0],
            u: [1.0, 0.0, 0.0],
            rho,
        }
    }

    /// The direct residual of the carrier against the wall `(x, y, r)`.
    fn off_wall(pose: Pose, theta: f64, x: f64, y: f64, r: f64) -> f64 {
        let p = pose.at(theta);
        (p.x - x).hypot(p.y - y) - r
    }

    /// The oracle: the sign changes of the direct residual on `[t0, t1]`,
    /// each bisected to the bit.
    fn oracle(pose: Pose, t0: f64, t1: f64, w: [f64; 3]) -> Vec<f64> {
        let f = |t: f64| off_wall(pose, t, w[0], w[1], w[2]);
        let steps = 20_000;
        let mut out = Vec::new();
        for k in 0..steps {
            let at = |k: u32| t0 + (t1 - t0) * f64::from(k) / f64::from(steps);
            let (mut a, mut b) = (at(k), at(k + 1));
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
            out.push((a + b) / 2.0);
        }
        out
    }

    fn door(pose: Pose, t0: f64, t1: f64, w: [f64; 3]) -> CircleRoots<f64> {
        circle_cylinder_roots(&pose.carrier(), t0, t1, &wall(w[0], w[1], w[2]), band()).unwrap()
    }

    /// The door's roots on `[t0, t1]` match the oracle's, in number and
    /// to well inside the band, and every one lies on the wall.
    fn assert_matches_oracle(label: &str, pose: Pose, t0: f64, t1: f64, w: [f64; 3], want: usize) {
        let CircleRoots::Certified { count, thetas } = door(pose, t0, t1, w) else {
            panic!(
                "{label}: expected certified roots, got {:?}",
                door(pose, t0, t1, w)
            );
        };
        assert_eq!(count, want, "{label}: the certified count");
        let mid = (t0 + t1) / 2.0;
        let mut got: Vec<f64> = thetas[..count].to_vec();
        for &t in &got {
            assert!(
                (t - mid).abs() <= core::f64::consts::PI,
                "{label}: {t} within π of {mid}"
            );
            let off = off_wall(pose, t, w[0], w[1], w[2]).abs();
            assert!(off < 1e-12, "{label}: root {t} lies {off} off the wall");
        }
        got.sort_by(f64::total_cmp);
        let truth = oracle(
            pose,
            mid - core::f64::consts::PI,
            mid + core::f64::consts::PI,
            w,
        );
        assert_eq!(
            got.len(),
            truth.len(),
            "{label}: {got:?} vs the oracle {truth:?}"
        );
        for (a, b) in got.iter().zip(&truth) {
            assert!(
                (a - b).abs() < 1e-9,
                "{label}: root {a} vs the oracle's {b}"
            );
        }
    }

    /// **The parallel arm**: a circle square to the axis crossing the
    /// wall twice — the parallel equal-radius cylinders' rim, at the
    /// pose of #347 — with the arc past the branch cut.
    #[test]
    fn a_circle_square_to_the_axis_crosses_the_wall_twice() {
        let w = [1.2, 0.0, 1.0];
        assert_matches_oracle("rim", flat(0.0, 0.0, 2.0, 1.0), -1.0, 4.0, w, 2);
        assert_matches_oracle("wrapped", flat(0.0, 0.0, 2.0, 1.0), 5.0, 7.5, w, 2);
    }

    /// **The general arm**: tilted circles crossing the wall twice and
    /// four times. A circle in a plane through the axis, centred on it,
    /// meets the wall at four points (`|cos θ| = r/ρ`).
    #[test]
    fn a_tilted_circle_matches_the_oracle_at_two_and_four_roots() {
        let s = core::f64::consts::FRAC_1_SQRT_2;
        let meridian = Pose {
            c: [0.0, 0.0, 0.3],
            n: [0.0, 1.0, 0.0],
            u: [1.0, 0.0, 0.0],
            rho: 1.0,
        };
        assert_matches_oracle("meridian", meridian, 0.0, 6.0, [0.0, 0.0, 0.5], 4);
        let tilted = Pose {
            c: [0.1, -0.2, 0.0],
            n: [0.0, s, s],
            u: [1.0, 0.0, 0.0],
            rho: 1.0,
        };
        assert_matches_oracle("tilted", tilted, -2.0, 3.0, [1.2, 0.0, 1.0], 2);
    }

    /// Clear of the wall on either side, in both arms. The parallel arm
    /// decides a near miss — 1e-4 m clear — on its exact extremes.
    #[test]
    fn a_circle_clear_of_the_wall_is_a_miss_in_either_arm() {
        let s = core::f64::consts::FRAC_1_SQRT_2;
        for (label, pose, w) in [
            ("outside", flat(0.0, 0.0, 0.0, 1.0), [3.0, 0.0, 1.0]),
            ("inside", flat(0.1, 0.0, 0.0, 0.5), [0.0, 0.0, 1.0]),
            ("just clear", flat(0.0, 0.0, 0.0, 1.0), [2.0001, 0.0, 1.0]),
            (
                "tilted outside",
                Pose {
                    c: [0.0, 0.0, 0.0],
                    n: [0.0, s, s],
                    u: [1.0, 0.0, 0.0],
                    rho: 1.0,
                },
                [3.0, 0.0, 1.0],
            ),
        ] {
            assert!(
                matches!(door(pose, 0.0, 6.0, w), CircleRoots::Miss),
                "{label}: {:?}",
                door(pose, 0.0, 6.0, w)
            );
        }
    }

    /// **Coaxial**: a rim circle of the wall itself has a zero constant
    /// residual, and a coaxial circle of another radius a definite one.
    #[test]
    fn a_coaxial_circle_is_coaxial_on_the_wall_and_a_miss_off_it() {
        let w = [0.3, -0.4, 1.0];
        assert!(matches!(
            door(flat(0.3, -0.4, 5.0, 1.0), 0.0, 6.0, w),
            CircleRoots::Coaxial
        ));
        assert!(matches!(
            door(flat(0.3, -0.4, 5.0, 0.6), 0.0, 6.0, w),
            CircleRoots::Miss
        ));
    }

    /// A tangency is not a crossing at any order this lane sees, in
    /// either arm: a circle square to the axis touching the wall from
    /// outside, and a circle in a plane through the axis touching it at
    /// its top (`ρ = r`, centred on the axis' perpendicular through it).
    #[test]
    fn a_tangent_circle_is_uncertain_in_either_arm() {
        assert!(matches!(
            door(flat(0.0, 0.0, 0.0, 1.0), 0.0, 6.0, [2.0, 0.0, 1.0]),
            CircleRoots::Uncertain
        ));
        let grazing = Pose {
            c: [0.0, 0.0, 0.0],
            n: [0.0, 1.0, 0.0],
            u: [1.0, 0.0, 0.0],
            rho: 1.0,
        };
        // The wall through x = ±2 does not reach it; x = ±1 touches it at
        // θ = 0 and θ = π, where the residual has double roots.
        assert!(matches!(
            door(grazing, -1.0, 4.0, [0.0, 0.0, 1.0]),
            CircleRoots::Uncertain
        ));
    }

    /// **The parallel arm's meter refuses an unreadable reading.** A
    /// unit circle square to a unit wall 2000 m off: a definite miss
    /// whose harmonics are built from terms of order 2000², whose
    /// rounding lies in the default band's escalation gap.
    #[test]
    fn the_parallel_noise_meter_refuses_a_reading_in_the_band_gap() {
        if !default_band() {
            return;
        }
        let got = door(flat(0.0, 0.0, 0.0, 1.0), -1.0, 1.0, [2001.0, 0.0, 1.0]);
        assert!(
            matches!(got, CircleRoots::Uncertain),
            "the noise meter refuses, got {got:?}"
        );
    }

    /// **A desynced caller is a kernel defect, loudly.**
    #[test]
    fn a_non_circle_or_non_cylinder_is_a_classification_invariant() {
        let line = geom::Curve3::Line {
            origin: Point3::origin(),
            dir: Vec3::new(1.0, 0.0, 0.0),
        };
        let plane = geom::Surface::Plane {
            origin: Point3::origin(),
            normal: Vec3::new(0.0, 0.0, 1.0),
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        };
        let circle = flat(0.0, 0.0, 0.0, 1.0).carrier::<f64>();
        let cylinder = wall::<f64>(0.0, 0.0, 1.0);
        for (carrier, surface) in [(&line, &cylinder), (&circle, &plane)] {
            let got = circle_cylinder_roots(carrier, 0.0, 1.0, surface, band());
            assert!(
                matches!(got, Err(BooleanError::ClassificationInvariant { .. })),
                "a wrong kind refuses as a kernel invariant, got {got:?}"
            );
        }
    }

    /// The interval lane, in both arms: every certified root enclosure
    /// is tight and contains the oracle's root.
    #[test]
    fn the_interval_lane_encloses_the_oracle_roots() {
        let s = core::f64::consts::FRAC_1_SQRT_2;
        let tilted = Pose {
            c: [0.1, -0.2, 0.0],
            n: [0.0, s, s],
            u: [1.0, 0.0, 0.0],
            rho: 1.0,
        };
        let w = [1.2, 0.0, 1.0];
        for (label, pose, t0, t1) in [
            ("square", flat(0.0, 0.0, 2.0, 1.0), -1.0, 4.0),
            ("tilted", tilted, -2.0, 3.0),
        ] {
            let got = circle_cylinder_roots::<Interval>(
                &pose.carrier(),
                Interval::from_f64(t0),
                Interval::from_f64(t1),
                &wall(w[0], w[1], w[2]),
                band(),
            )
            .unwrap();
            let CircleRoots::Certified { count, thetas } = got else {
                panic!("{label}: expected certified roots, got {got:?}");
            };
            let mid = (t0 + t1) / 2.0;
            let want = oracle(
                pose,
                mid - core::f64::consts::PI,
                mid + core::f64::consts::PI,
                w,
            );
            assert_eq!(count, want.len(), "{label}: the count");
            for w in want {
                assert!(
                    thetas[..count].iter().any(|t| {
                        t.lo() - 1e-12 <= w && w <= t.hi() + 1e-12 && t.hi() - t.lo() < 1e-9
                    }),
                    "{label}: oracle root {w} in no tight enclosure {thetas:?}"
                );
            }
        }
    }
}
