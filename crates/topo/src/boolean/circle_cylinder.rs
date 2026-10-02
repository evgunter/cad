//! **The circle × cylinder root door**: the certified crossings of a
//! CIRCLE carrier with a cylinder wall. It owns no root machinery: it
//! reads the wall's harmonics from their one home and hands them to one
//! of the shared root cores ([`super::circle_roots`]).
//!
//! # The residual is a degree-2 trigonometric polynomial
//!
//! With the carrier `C(θ) = C₀ + ρ(û cos θ + v̂ sin θ)`, `v̂ = n̂ × û`,
//! and the wall `(o, â, r)`, the linearized residual is EXACTLY
//! `c₀ + c₁ cos θ + s₁ sin θ + c₂ cos 2θ + s₂ sin 2θ` in metres
//! (`geom_brep::conic_cylinder_harmonics`, which
//! `geom_brep::circle_residual_extremes` reads too), so the noise
//! meter's floor on `|F|` per metre of residual is `1`, an identity
//! rather than a neighbourhood bound. At most four crossings per turn.
//! Every in-band sign escalates as [`BooleanDecision::ArcCylinderRoots`].
//!
//! # Two arms, by the carrier's tilt
//!
//! `bool_circle_cylinder_tilt` decides `ρ·|n̂ × â|` (metres: the
//! amplitude of the carrier's height along the wall's axis).
//!
//! - **The square arm — tilt in the zero band**: the circle is square to
//!   the axis. The second harmonic then vanishes up to `tilt²/4r`, and
//!   the residual is a first harmonic, the wall's cross-section circle
//!   against the carrier. The shared first-harmonic door decides it on
//!   its exact extremes, under the `bool_circle_cylinder_square_*` rows,
//!   with the dropped second harmonic charged to its noise. This arm is
//!   REQUIRED, for two poses the ladder cannot answer truthfully: a
//!   COAXIAL circle, whose residual is constant (`F·(1 + t²)²`, which
//!   the ladder cannot read; the extremes answer on-surface — a rim
//!   circle of the same wall — or miss), and a TANGENCY, which the
//!   extremes put in the zero band and escalate, where the ladder can
//!   certify a miss
//!   (`work/germ/the-half-angle-ladder-certifies-in-band-configurations.md`).
//! - **The ladder — tilt definite, or in the band's gap**: the five
//!   harmonics go to the shared half-angle ladder under the
//!   `bool_circle_cylinder_*` ladder rows, with the carrier's own `2ρ`
//!   as its lever.
//!
//! The two arms' meters keep their cores' postures, which differ on a
//! reading in the band's gap: the square arm's refuse it, the ladder's
//! pass it (the circle root cores' module docs, "The ladder's noise
//! meter") — which is why their rows have distinct names.

use geom_core::{Band, Decide, Margin, Sign};

use super::circle_roots::{
    CircleRoots, FirstHarmonic, FirstHarmonicRows, HalfAngleFrame, HalfAngleRows, Harmonics,
    first_harmonic_roots, half_angle_roots, rounding_charge,
};
use super::solid_contain::QuarticRows;
use super::{BooleanDecision, BooleanError};
use crate::validate::decide;

/// The square arm's rows.
const CIRCLE_CYLINDER_SQUARE_ROWS: FirstHarmonicRows = FirstHarmonicRows {
    noise: "bool_circle_cylinder_square_noise",
    coaxial: "bool_circle_cylinder_square_coaxial",
    extreme: "bool_circle_cylinder_square_extreme",
    root_slack: "bool_circle_cylinder_square_root_slack",
    decision: BooleanDecision::ArcCylinderRoots,
};

/// The ladder's rows.
const CIRCLE_CYLINDER_LADDER_ROWS: HalfAngleRows = HalfAngleRows {
    pole: "bool_circle_cylinder_pole",
    conditioning: "bool_circle_cylinder_pole_conditioning",
    noise: "bool_circle_cylinder_ladder_noise",
    root_slack: "bool_circle_cylinder_ladder_root_slack",
    quartic: QuarticRows {
        disc: "bool_circle_cylinder_disc",
        shape: "bool_circle_cylinder_shape",
        depth: "bool_circle_cylinder_depth",
        odd: "bool_circle_cylinder_odd",
        split: "bool_circle_cylinder_split",
        split_lead: "bool_circle_cylinder_split_lead",
    },
    decision: BooleanDecision::ArcCylinderRoots,
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
/// an extreme or constant residual of the square arm, or a rung of the
/// ladder. A tilt in the band's GAP is not an error: it takes the
/// ladder, which needs no tilt to be definite; a tilt in the zero band
/// takes the square arm.
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
    let h = geom_brep::conic_cylinder_harmonics(
        &geom_brep::Conic::circle(center, axis, radius, u_ref),
        origin,
        w_axis,
        w_radius,
    );
    let two = T::from_f64(2.0);
    // The harmonics' rounding, in residual metres: the term bound is in
    // m², before the `2r` division.
    let noise = rounding_charge(h.terms) / (two * w_radius);
    let hypot = |x: T, y: T| (x.powi(2) + y.powi(2)).sqrt();
    let tilt = radius * axis.cross(w_axis).norm();
    if let Ok(Sign::Zero) = decide("bool_circle_cylinder_tilt", Margin::of(tilt), band) {
        // The dropped second harmonic is charged to the noise. No test
        // can tell this charge from its absence: with the tilt in the
        // zero band it is at most `tilt²/4r ≤ zero²/4r`, which is below
        // a quarter of the zero band for any wall `r ≥ zero`, so it never
        // moves a decision a test can reach. It is kept because it is
        // what makes the first harmonic the residual to within `noise`.
        let first = FirstHarmonic {
            c0: h.c0,
            a1: hypot(h.c1, h.s1),
            cos_part: h.c1,
            sin_part: h.s1,
            noise: noise + hypot(h.c2, h.s2),
        };
        return first_harmonic_roots(&first, radius, t0, t1, &CIRCLE_CYLINDER_SQUARE_ROWS, band);
    }
    let v_ref = axis.cross(u_ref);
    let point_at = |theta: T| {
        let (s, c) = theta.sin_cos();
        center + u_ref * (radius * c) + v_ref * (radius * s)
    };
    half_angle_roots(
        &Harmonics {
            c0: h.c0,
            c1: h.c1,
            s1: h.s1,
            c2: h.c2,
            s2: h.s2,
        },
        |theta| geom_brep::implicit_residual(wall, point_at(theta)),
        HalfAngleFrame {
            t0,
            t1,
            speed_lo: radius,
            speed_hi: radius,
            // Not clamped by the wall's size, as the torus door clamps
            // by its extent: the wall is unbounded along its axis, and a
            // circle in a plane through that axis meets it at points a
            // whole diameter apart whatever `r` is, so `2ρ` is the
            // spread the roots can have.
            lever: two * radius,
            noise,
            f_per_metre: T::one(),
        },
        &CIRCLE_CYLINDER_LADDER_ROWS,
        band,
    )
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

    /// **The square arm**: a circle square to the axis crossing the
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

    /// Clear of the wall on either side, in both arms. The square arm
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

    /// **Coaxial, on the square arm**: a rim circle of the wall itself
    /// has a zero constant residual, and a coaxial circle of another
    /// radius a definite one.
    #[test]
    fn a_coaxial_circle_is_on_the_wall_or_a_miss_off_it() {
        let w = [0.3, -0.4, 1.0];
        assert!(matches!(
            door(flat(0.3, -0.4, 5.0, 1.0), 0.0, 6.0, w),
            CircleRoots::OnSurface
        ));
        assert!(matches!(
            door(flat(0.3, -0.4, 5.0, 0.6), 0.0, 6.0, w),
            CircleRoots::Miss
        ));
    }

    /// **The square arm escalates a tangency** — a circle square to the
    /// axis touching the wall from outside puts an extreme in the zero
    /// band, and that is not a crossing at any order this lane sees. (The
    /// ladder carries no such guarantee: it can read a tangency as a
    /// miss, `work/germ/the-half-angle-ladder-certifies-in-band-configurations.md`.)
    #[test]
    fn a_tangent_circle_square_to_the_axis_is_uncertain() {
        assert!(matches!(
            door(flat(0.0, 0.0, 0.0, 1.0), 0.0, 6.0, [2.0, 0.0, 1.0]),
            CircleRoots::Uncertain
        ));
    }

    /// **The square arm's meter refuses an unreadable reading.** A
    /// unit circle square to a unit wall 2000 m off: a definite miss
    /// whose harmonics are built from terms of order 2000², whose
    /// rounding lies in the default band's escalation gap.
    #[test]
    fn the_square_noise_meter_refuses_a_reading_in_the_band_gap() {
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
