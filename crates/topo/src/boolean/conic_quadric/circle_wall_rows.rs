//! **The circle × wall rows** of the one conic × quadric door.
//!
//! Each pose is checked against the geometry, not the door's own
//! algebra: a certified root must put the carrier ON the wall, and
//! its count must be the count of sign changes of the direct
//! residual along the carrier.

#![allow(clippy::unwrap_used, clippy::panic)]

use core::f64::consts::TAU;

use super::*;
use crate::boolean::conic_oracle::{crossings, extremes};
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
        let v = |a: [f64; 3]| Vec3::new(T::from_f64(a[0]), T::from_f64(a[1]), T::from_f64(a[2]));
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

/// The oracle: the crossings of the direct residual on `[t0, t1]`
/// (`conic_oracle::crossings`).
fn oracle(pose: Pose, t0: f64, t1: f64, w: [f64; 3]) -> Vec<f64> {
    crossings(|t| off_wall(pose, t, w[0], w[1], w[2]), t0, t1)
}

fn door(pose: Pose, t0: f64, t1: f64, w: [f64; 3]) -> CircleRoots<f64> {
    conic_quadric_roots(&pose.carrier(), t0, t1, &wall(w[0], w[1], w[2]), band()).unwrap()
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

/// **A graze is read by its depth, in the band's own metres.** A
/// circle of the wall's own radius, its plane tilted 0.3 rad about the
/// `x` axis, lies inside the wall and touches it at `θ = 0` and `π`;
/// moved `depth` along `x`, it crosses the wall by `|depth|` on one
/// side. At the default band: a depth inside the zero band is no
/// certified answer either way, and a definite depth is two certified
/// crossings, each on the wall — or, on the 50 m and 500 m walls,
/// `Uncertain`: there the `f64` residual's rounding places the shallow
/// roots 2e-10 to 4.6e-10 m from the true ones (the exact oracle), and
/// the root slack, which must read inside the zero band, refuses them.
/// The half-angle ladder alone certified a `Miss` at depth 1e-9 m on a
/// 50 m wall and at 1e-8 m on a 500 m wall, and answered
/// `CountDisagrees` at −1e-8 m.
#[test]
fn a_graze_is_read_by_its_depth() {
    let band = Band::new(1e-9, 1e-8).unwrap();
    let tilt = 0.3_f64;
    for (r, depth, placed) in [
        (50.0, 1e-9, false),
        (500.0, 1e-8, false),
        (500.0, -1e-8, false),
        (500.0, 5e-6, false),
        (50.0, 2e-8, false),
        (5.0, -2e-8, true),
    ] {
        let label = format!("wall r {r}, depth {depth}");
        let pose = Pose {
            c: [depth, 0.0, 0.0],
            n: [0.0, -tilt.sin(), tilt.cos()],
            u: [1.0, 0.0, 0.0],
            rho: r,
        };
        let got = conic_quadric_roots(&pose.carrier(), -1.0, 1.0, &wall(0.0, 0.0, r), band);
        if f64::abs(depth) <= band.zero() * 10.0 {
            assert!(
                matches!(got, Ok(CircleRoots::Uncertain) | Err(_)),
                "{label}: a graze in the band, got {got:?}"
            );
            continue;
        }
        if !placed {
            assert!(
                matches!(got, Ok(CircleRoots::Uncertain)),
                "{label}: roots the band cannot place refuse on their slack, got {got:?}"
            );
            continue;
        }
        let Ok(CircleRoots::Certified { count, thetas }) = got else {
            panic!("{label}: two certified crossings, got {got:?}");
        };
        assert_eq!(count, 2, "{label}");
        for &t in &thetas[..count] {
            let off = off_wall(pose, t, 0.0, 0.0, r).abs();
            assert!(
                off <= band.zero(),
                "{label}: root {t} lies {off} off the wall"
            );
        }
    }
}

/// **The first-harmonic arm**: a circle square to the axis crossing the
/// wall twice — the parallel equal-radius cylinders' rim, at the
/// pose of #347 — with the arc past the branch cut.
#[test]
fn a_circle_square_to_the_axis_crosses_the_wall_twice() {
    let w = [1.2, 0.0, 1.0];
    assert_matches_oracle("rim", flat(0.0, 0.0, 2.0, 1.0), -1.0, 4.0, w, 2);
    assert_matches_oracle("wrapped", flat(0.0, 0.0, 2.0, 1.0), 5.0, 7.5, w, 2);
}

/// **The ladder**: tilted circles crossing the wall twice and
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

/// **The arm is decided on the second harmonic, not on the tilt.** On a
/// circle `A₂ = (ρ·sin α)²/4r`, so the second harmonic is in the zero band
/// for every tilt `ρ·sin α` up to `2√(r·zero)`, orders past the zero band
/// a tilt read as a length would allow. A circle of a unit wall's own
/// radius, centred on its axis and tilted by `tilt` — at most
/// `tilt²/2r` inside the wall anywhere — at a tenth of that reach (`A₂` a
/// hundredth of the zero band, the tilt definitely past the escalation
/// threshold) lies on the wall to within the band, and is `OnSurface`;
/// read on its tilt it took the ladder, which cannot read a constant
/// residual and answered `Uncertain`. At ten times that reach (`A₂`
/// definite) it touches the wall from inside at `θ = 0` and `π`: the
/// ladder's, and never `OnSurface` nor a `Miss`.
#[test]
fn the_second_harmonic_band_reaches_past_the_tilt_band() {
    let band = band();
    for r in [1.0, 100.0] {
        let reach = 2.0 * (r * band.zero()).sqrt();
        for (scale, on) in [(0.1, true), (10.0, false)] {
            let tilt = scale * reach;
            let label = format!("wall r {r}, tilt {tilt:e}");
            let pose = Pose {
                c: [0.0; 3],
                n: [0.0, -tilt / r, (1.0 - (tilt / r).powi(2)).sqrt()],
                u: [1.0, 0.0, 0.0],
                rho: r,
            };
            assert!(tilt > band.escalate(), "{label}: the tilt is definite");
            let h = geom_brep::conic_cylinder_harmonics(
                &geom_brep::Conic::of(&pose.carrier::<f64>()).unwrap(),
                Point3::origin(),
                Vec3::new(0.0, 0.0, 1.0),
                r,
            );
            let a2 = h.c2.hypot(h.s2);
            let closed = tilt.powi(2) / (4.0 * r);
            assert!(
                (a2 - closed).abs() <= 1e-3 * closed + rounding_charge(h.terms) / (2.0 * r),
                "{label}: A₂ {a2:e} is (ρ·sin α)²/4r = {closed:e}, to within its rounding"
            );
            let (lo, hi) = extremes(|t| off_wall(pose, t, 0.0, 0.0, r), 0.0, TAU);
            let deepest = lo.abs().max(hi.abs());
            let got = conic_quadric_roots(&pose.carrier(), -0.5, 0.5, &wall(0.0, 0.0, r), band);
            if on {
                assert!(
                    deepest <= band.zero(),
                    "{label}: on the wall, {deepest:e} at most"
                );
                assert!(
                    matches!(got, Ok(CircleRoots::OnSurface)),
                    "{label}: on the wall to within the band, got {got:?}"
                );
            } else {
                assert!(
                    deepest > band.escalate(),
                    "{label}: {deepest:e} inside at most"
                );
                assert!(
                    !matches!(
                        got,
                        Ok(CircleRoots::OnSurface
                            | CircleRoots::Miss
                            | CircleRoots::Certified { .. })
                    ),
                    "{label}: a tangency from inside, got {got:?}"
                );
            }
        }
    }
}

/// **The first-harmonic arm charges the second harmonic it drops.** A
/// circle of the wall's own radius, centred on its axis and tilted, has
/// the residual `−A₂(1 − cos 2θ)`: it touches the wall at `θ = 0` and `π`
/// and lies `2A₂` inside it at `±π/2`. With `A₂` at seven tenths of the
/// zero band the arm takes it, and its true greatest distance from the
/// wall, `1.4·zero`, is past the zero band: no `OnSurface`. Read with
/// the dropped harmonic uncharged, the residual's constant part `−A₂` sits
/// inside the band and the arm answered `OnSurface`.
#[test]
fn a_near_square_circle_inside_the_band_by_its_second_harmonic_is_not_on_the_wall() {
    let band = band();
    for r in [1.0, 100.0] {
        let a2 = 0.7 * band.zero();
        let tilt = (4.0 * r * a2).sqrt();
        let label = format!("wall r {r}, A₂ {a2:e}");
        let pose = Pose {
            c: [0.0; 3],
            n: [0.0, -tilt / r, (1.0 - (tilt / r).powi(2)).sqrt()],
            u: [1.0, 0.0, 0.0],
            rho: r,
        };
        let h = geom_brep::conic_cylinder_harmonics(
            &geom_brep::Conic::of(&pose.carrier::<f64>()).unwrap(),
            Point3::origin(),
            Vec3::new(0.0, 0.0, 1.0),
            r,
        );
        assert!(
            h.c2.hypot(h.s2) < band.zero(),
            "{label}: the second harmonic is in the zero band, so the arm decides"
        );
        let (lo, hi) = extremes(|t| off_wall(pose, t, 0.0, 0.0, r), 0.0, TAU);
        assert!(
            lo < -band.zero() && hi.abs() <= band.zero(),
            "{label}: the true distance runs over [{lo:e}, {hi:e}], `2A₂` deep"
        );
        let got = conic_quadric_roots(&pose.carrier(), -0.5, 0.5, &wall(0.0, 0.0, r), band);
        assert!(
            !matches!(got, Ok(CircleRoots::OnSurface)),
            "{label}: {:e} off the wall, got {got:?}",
            -lo
        );
    }
}

/// Clear of the wall on either side, in both arms. The first-harmonic arm
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

/// **Coaxial, on the first-harmonic arm**: a rim circle of the wall itself
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

/// **The first-harmonic arm escalates a tangency** — a circle square to the
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

/// **The first-harmonic arm's meter refuses an unreadable reading.** A
/// unit circle square to a unit wall 2000 m off: a definite miss
/// whose harmonics are built from terms of order 2000², whose
/// rounding lies in the default band's escalation gap.
#[test]
fn the_first_harmonic_noise_meter_refuses_a_reading_in_the_band_gap() {
    if !default_band() {
        return;
    }
    let got = door(flat(0.0, 0.0, 0.0, 1.0), -1.0, 1.0, [2001.0, 0.0, 1.0]);
    assert!(
        matches!(got, CircleRoots::Uncertain),
        "the noise meter refuses, got {got:?}"
    );
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
        let got = conic_quadric_roots::<Interval>(
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
