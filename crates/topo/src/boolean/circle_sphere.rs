//! **The circle × sphere root door**: the certified crossings of a
//! CIRCLE carrier with a sphere, beside the line × sphere quadratic
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
//! sphere meet it in at most two points.
//!
//! The door decides it with the shared first-harmonic door
//! ([`super::circle_roots::first_harmonic_roots`]) under its own rows,
//! `bool_circle_sphere_noise`, `_coaxial`, `_extreme` and `_root_slack`,
//! escalating as [`BooleanDecision::ArcSphereRoots`]. Its extremes are
//! the harmonic's factored ones, `(D∓ − r)(D∓ + r)/2r` with `D∓` the
//! distances from the sphere's centre to the circle's nearest and
//! farthest points, each with a running bound on its rounding, so a
//! shallow crossing is placed as well as its near extreme is evaluated
//! rather than as well as the harmonics' m² terms are. A frame that is
//! not orthonormal moves the circle it evaluates off that form, and its
//! defect is charged to both extremes too
//! (`geom_brep::CircleSphereHarmonic::frame_error`). Its constant-
//! residual answer is the coaxial circle's: every circle whose axis
//! passes through the sphere's centre has one, and lies ON the sphere
//! when it is zero.

use geom_core::{Band, Decide};

use super::circle_roots::{CircleRoots, FirstHarmonic, FirstHarmonicRows, first_harmonic_roots};
use super::{BooleanDecision, BooleanError};

const CIRCLE_SPHERE_ROWS: FirstHarmonicRows = FirstHarmonicRows {
    noise: "bool_circle_sphere_noise",
    coaxial: "bool_circle_sphere_coaxial",
    extreme: "bool_circle_sphere_extreme",
    root_slack: "bool_circle_sphere_root_slack",
    decision: BooleanDecision::ArcSphereRoots,
};

/// The certified crossings of the `carrier` circle with the `sphere`,
/// reported within `π` of the midpoint of `[t0, t1]` (module docs).
///
/// # Errors
///
/// [`BooleanError::ClassificationInvariant`] when `carrier` is not a
/// circle or `sphere` not a sphere — the caller dispatched on those kinds,
/// so a mismatch is a desync, never an answer. Otherwise as
/// [`first_harmonic_roots`].
pub(super) fn circle_sphere_roots<T: Decide>(
    carrier: &geom::Curve3<T>,
    t0: T,
    t1: T,
    sphere: &geom::Surface<T>,
    band: Band,
) -> Result<CircleRoots<T>, BooleanError> {
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
    let h = geom_brep::circle_sphere_harmonic(center, axis, radius, u_ref, s_center, s_radius);
    first_harmonic_roots(
        &FirstHarmonic {
            lo: h.lo,
            hi: h.hi,
            cos_part: h.e_u,
            sin_part: h.e_v,
            lo_noise: h.lo_error + h.frame_error,
            hi_noise: h.hi_error + h.frame_error,
            phase_noise: h.phase_error,
        },
        radius,
        t0,
        t1,
        &CIRCLE_SPHERE_ROWS,
        band,
    )
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

    fn roots(c: [f64; 3], r: f64, t0: f64, t1: f64) -> CircleRoots<f64> {
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
        let CircleRoots::Certified {
            count: 2,
            thetas: [ts @ .., _, _],
        } = roots(c, r, t0, t1)
        else {
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
        let CircleRoots::Certified {
            count: 2,
            thetas: [ts @ .., _, _],
        } = roots(c, r, t0, t1)
        else {
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
                matches!(roots(c, r, 0.0, 6.0), CircleRoots::Miss),
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
    /// sphere is a miss, on it is `OnSurface`.
    #[test]
    fn a_coaxial_carrier_is_a_miss_off_the_sphere_and_coaxial_on_it() {
        assert!(matches!(
            roots([0.0, 0.0, 0.7], 1.6, 0.0, 6.0),
            CircleRoots::Miss
        ));
        assert!(matches!(
            roots([0.0, 0.0, 0.7], 0.5, 0.0, 6.0),
            CircleRoots::Miss
        ));
        assert!(matches!(
            roots([0.0, 0.0, 0.5], 1.25_f64.sqrt(), 0.0, 6.0),
            CircleRoots::OnSurface
        ));
    }

    /// **A near-coaxial carrier that comes within the zero band is not a
    /// miss**, at any admissible `K`. The sphere's centre is `0.99·zero`
    /// off the carrier's axis, so the swing `A₁` is in the zero band, and
    /// its mean residual `−1.6e-9` is past the escalation threshold at
    /// `K = 1.5`; the swing carries the residual's top to `−7e-10`, inside
    /// the zero band, where the extremes read a tangency.
    #[test]
    fn a_near_coaxial_carrier_reaching_the_zero_band_is_not_a_miss() {
        let zero = 1e-9;
        let r = (1.25_f64 + 3.2e-9 * 1.25_f64.sqrt()).sqrt();
        let c = [0.99 * zero, 0.0, 0.5];
        let top = off_sphere(core::f64::consts::PI, c, r);
        let bottom = off_sphere(0.0, c, r);
        assert!(
            top > -zero && bottom < -1.5 * zero,
            "the pose's premise: the residual spans {bottom} .. {top}"
        );
        for k in [1.5, 1.2, 2.0, 3.0, 10.0] {
            let got = circle_sphere_roots(
                &circle(1.0),
                0.0,
                6.0,
                &sphere(c, r),
                Band::new(zero, k * zero).unwrap(),
            );
            assert!(
                !matches!(got, Ok(CircleRoots::Miss)),
                "K = {k}: a residual reaching the zero band certified a miss"
            );
        }
    }

    #[test]
    fn a_tangent_sphere_is_uncertain() {
        // Externally tangent at (1, 0, 0): a touch, not a crossing.
        assert!(matches!(
            roots([1.5, 0.0, 0.0], 0.5, 0.0, 6.0),
            CircleRoots::Uncertain
        ));
    }

    /// **The noise meter, isolated.** On a crossing the slack reads at
    /// least the noise (`|R′| ≤ ρ` there, so `ρ·noise/|R′| ≥ noise`), so a
    /// crossing pose cannot tell the two meters apart. A MISS can: it
    /// returns before the slack is read. A unit circle against a unit
    /// sphere far away is a definite miss whose far extreme is evaluated
    /// as a product of lengths of order `|e|` over `2r`: at 2·10⁵ m its
    /// rounding is definitely past the band (`Positive`), at 2000 m it
    /// lies in the band's escalation gap (`Err`). Either reading refuses; without its arm, each pose
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
                matches!(got, CircleRoots::Uncertain),
                "a sphere {far} m off: the noise meter refuses, got {got:?}"
            );
        }
    }

    /// The finest band the suite runs at, named here so the near-tangent
    /// rows read the same at every ε row.
    fn fine_band() -> Band {
        Band::new(1e-12, 1e-11).unwrap()
    }

    /// The near-tangent pose: the unit circle and a sphere of radius
    /// `R` centred at `(1 + R − δ, 0, 0)`, its nearest point `δ` inside
    /// the sphere. Every number is dyadic, so `δ` is the pose's own and
    /// not its rounding.
    const R: f64 = 0.75;

    fn near_tangent(delta: f64, band: Band) -> CircleRoots<f64> {
        circle_sphere_roots(
            &circle(1.0),
            -1.0,
            1.0,
            &sphere([1.0 + R - delta, 0.0, 0.0], R),
            band,
        )
        .unwrap()
    }

    /// **A shallow crossing is placed to within the finest band.** The
    /// residual's slope at the roots shrinks as `√δ`, so the roots' arc
    /// slack is the extremes' rounding over it. Down to `δ = 2⁻²⁰`
    /// (`≈ 9.5e-7`) both roots are certified, and each lies within the
    /// band of the closed-form crossing of the circle with the sphere's
    /// great circle in its plane: `1 − cos θ = δ(2R − δ)/2dρ`, every
    /// factor free of cancellation.
    #[test]
    fn a_near_tangent_crossing_is_placed_within_the_finest_band() {
        for delta in [2f64.powi(-14), 2f64.powi(-17), 2f64.powi(-20)] {
            let d = 1.0 + R - delta;
            let want = 2.0 * (delta * (2.0 * R - delta) / (4.0 * d)).sqrt().asin();
            let CircleRoots::Certified {
                count: 2,
                thetas: [ts @ .., _, _],
            } = near_tangent(delta, fine_band())
            else {
                panic!("δ = {delta:e}: the extremes straddle, so two roots");
            };
            let mut got = ts.map(f64::abs);
            got.sort_by(f64::total_cmp);
            for t in got {
                let off = (t - want).abs();
                assert!(
                    off <= 1e-12,
                    "δ = {delta:e}: root ±{t} is {off:e} off the closed form {want}"
                );
            }
            assert!(
                ts[0] * ts[1] < 0.0,
                "δ = {delta:e}: the roots straddle θ = 0, got {ts:?}"
            );
        }
    }

    /// **The root-slack meter refuses a root it cannot place.** The same
    /// pose at `δ = 2⁻³³` (`≈ 1.2e-10`): both extremes are definite, so
    /// the roots exist, but the slope at them is so shallow that the
    /// extremes' rounding moves each by more than the band's escalation
    /// threshold. Without the meter this pose answers two roots.
    #[test]
    fn the_root_slack_meter_refuses_a_shallow_crossing() {
        let got = near_tangent(2f64.powi(-33), fine_band());
        assert!(
            matches!(got, CircleRoots::Uncertain),
            "the root-slack meter refuses: {got:?}"
        );
    }

    /// **The root-slack meter refuses an unreadable reading — and that
    /// is where the near-tangent family stops at the finest band.** At
    /// `δ = 2⁻²³` (`≈ 1.2e-7`) the slack, about twice the band, lies in
    /// its escalation gap while the noise is in its zero band: the
    /// `f64` evaluation of the near extreme cannot place the root to
    /// within `1e-12`
    /// (`work/reach/f64-cannot-place-a-shallow-crossing-within-the-finest-band.md`).
    /// Without the `Err` arm this pose answers two roots.
    #[test]
    fn the_root_slack_meter_refuses_a_reading_in_the_band_gap() {
        let got = near_tangent(2f64.powi(-23), fine_band());
        assert!(
            matches!(got, CircleRoots::Uncertain),
            "the root-slack meter refuses: {got:?}"
        );
    }

    /// **The half-chord is read from the extreme nearer zero.** A sphere
    /// of radius 2 dipping `δ` from `2.3e-6` to `2.9e-5` into a circle of
    /// radius 4096, on a band of `1e-7`: the far extreme is `≈ 1.7e7` m
    /// of residual against a near one of `≈ −δ`, so `hi/(hi − lo)` and
    /// `c₀/A₁` sit within `≈ 1e-12` of `1`, where `asin` and `acos`
    /// amplify their argument's rounding by `≈ 1e6`: read off either,
    /// a half-chord is up to `≈ 1e-6` m of arc out. (The depths are not
    /// dyadic, since on a dyadic pose those ratios can come out exact
    /// and hide it; `δ = (ρ + r) − d` is still exact, by Sterbenz.)
    /// Read from the near extreme, both roots lie within the band of
    /// the closed form `1 − cos θ = δ(2r − δ)/2dρ`.
    #[test]
    fn a_small_sphere_grazing_a_large_circle_is_placed_from_its_near_extreme() {
        let (rho, r) = (4096.0, 2.0);
        let band = Band::new(1e-7, 1e-6).unwrap();
        for nominal in [2.3e-6, 3.7e-6, 5.1e-6, 7.9e-6, 1.3e-5, 2.9e-5] {
            let d = rho + r - nominal;
            let delta = (rho + r) - d;
            let want = 2.0 * (delta * (2.0 * r - delta) / (4.0 * d * rho)).sqrt().asin();
            let CircleRoots::Certified {
                count: 2,
                thetas: [ts @ .., _, _],
            } = circle_sphere_roots(&circle(rho), -1.0, 1.0, &sphere([d, 0.0, 0.0], r), band)
                .unwrap()
            else {
                panic!("δ = {delta:e}: the small sphere crosses the circle twice");
            };
            for t in ts {
                let off = rho * (t.abs() - want).abs();
                assert!(
                    off <= band.zero(),
                    "δ = {delta:e}: root ±{t} is {off:e} of arc off the closed form ±{want}"
                );
            }
        }
    }

    /// **A frame that is not orthonormal is charged for it.** The
    /// carrier's `û` is stretched by `2⁻²⁰`, so the circle the frame
    /// evaluates has radius `1 + 2⁻²⁰` and its nearest point dips
    /// `2⁻²¹` into the sphere — a crossing. The factored form, which
    /// assumes the frame orthonormal, puts that point `≈ 2.1e-6` OUTSIDE
    /// and would answer a definite miss. The frame's defect is charged
    /// to both extremes (`CircleSphereHarmonic::frame_error`), and the
    /// door then refuses rather than answer.
    #[test]
    fn a_frame_that_is_not_orthonormal_is_charged_for_it() {
        let stretch = 2f64.powi(-20);
        let d = 1.0 + R + 2f64.powi(-21);
        let truth = d - (1.0 + stretch);
        assert!(
            truth < R,
            "the evaluated circle's nearest point is inside the sphere"
        );
        let got = circle_sphere_roots(
            &framed([0.0; 3], [0.0, 0.0, 1.0], [1.0 + stretch, 0.0, 0.0], 1.0),
            -1.0,
            1.0,
            &sphere([d, 0.0, 0.0], R),
            band(),
        )
        .unwrap();
        assert!(
            matches!(got, CircleRoots::Uncertain),
            "a crossing the factored form misreads is refused, got {got:?}"
        );
    }

    /// A circle of radius `rho` about `c` in the frame `(n, u)`, as
    /// stored — orthonormal or not.
    fn framed(c: [f64; 3], n: [f64; 3], u: [f64; 3], rho: f64) -> geom::Curve3<f64> {
        geom::Curve3::Circle {
            center: Point3::from_array(c),
            axis: Vec3::new(n[0], n[1], n[2]),
            radius: rho,
            u_ref: Vec3::new(u[0], u[1], u[2]),
        }
    }

    /// The root slack's terms as the door charges them (module docs of
    /// `circle_roots`, "root slack"), each in metres of arc: the near
    /// extreme's error over the slope, the far extreme's share, the
    /// phase's error, the angle arithmetic's. Restated here only to pose
    /// the pin rows below, each of which sets its band between the slack
    /// with and without one term.
    struct SlackTerms {
        near: f64,
        far_share: f64,
        phase: f64,
        angle: f64,
        noise: f64,
        lo: f64,
        hi: f64,
    }

    impl SlackTerms {
        fn of(c: [f64; 3], n: [f64; 3], u: [f64; 3], rho: f64, sc: [f64; 3], r: f64) -> Self {
            let h = geom_brep::circle_sphere_harmonic(
                Point3::from_array(c),
                Vec3::new(n[0], n[1], n[2]),
                rho,
                Vec3::new(u[0], u[1], u[2]),
                Point3::from_array(sc),
                r,
            );
            let (lo_n, hi_n) = (h.lo_error + h.frame_error, h.hi_error + h.frame_error);
            let (swing, slope) = (h.hi - h.lo, (-h.lo * h.hi).max(0.0).sqrt());
            let (near, far) = if h.lo.abs() <= h.hi.abs() {
                (h.hi * lo_n, -h.lo * hi_n)
            } else {
                (-h.lo * hi_n, h.hi * lo_n)
            };
            Self {
                near: rho * near / swing / slope,
                far_share: rho * far / swing / slope,
                phase: rho * h.phase_error / h.e_u.hypot(h.e_v),
                angle: rho * super::super::circle_roots::rounding_charge(core::f64::consts::TAU),
                noise: lo_n.max(hi_n),
                lo: h.lo,
                hi: h.hi,
            }
        }

        fn full(&self) -> f64 {
            self.near + self.far_share + self.phase + self.angle
        }
    }

    /// **Each term of the root slack decides a pose.** For each term,
    /// a pose where it is a real share of the slack, and a band whose
    /// zero sits just above the slack WITHOUT it: the full slack then
    /// lies in the band's gap and the door refuses, where a slack that
    /// dropped the term would read zero and certify. Each term is a
    /// first-order error the root really carries (`circle_roots` module
    /// docs): the far extreme's share of the residual's error at the
    /// root, the phase's error, and the angle arithmetic's own rounding,
    /// which a review measured exceeding the rest of the slack on one
    /// generic pose in three thousand. The premises are asserted, so a
    /// pose that stops isolating its term fails as itself.
    #[test]
    fn each_term_of_the_root_slack_decides_a_pose() {
        let tilted = Vec3::new(1.0, 2.0, 2.0).normalize();
        let tilted_u = tilted.cross(Vec3::new(1.0, 0.0, 0.0)).normalize();
        let (tn, tu) = (
            [tilted.x, tilted.y, tilted.z],
            [tilted_u.x, tilted_u.y, tilted_u.z],
        );
        let at = |o: [f64; 3], s: f64, a: [f64; 3], b: f64, c: [f64; 3]| -> [f64; 3] {
            core::array::from_fn(|i| o[i] + s * a[i] + b * c[i])
        };
        let v = tilted.cross(tilted_u);
        let tv = [v.x, v.y, v.z];
        type Pose = ([f64; 3], [f64; 3], [f64; 3], f64, [f64; 3], f64);
        type Term = fn(&SlackTerms) -> f64;
        let poses: [(&str, Pose, Term); 3] = [
            (
                "the angle arithmetic",
                (
                    [0.0; 3],
                    [0.0, 0.0, 1.0],
                    [1.0, 0.0, 0.0],
                    1.0,
                    [1.0 + R - 2f64.powi(-10), 0.0, 0.0],
                    R,
                ),
                |t| t.angle,
            ),
            (
                "the far extreme's share",
                (
                    [0.2, -0.1, 0.3],
                    tn,
                    tu,
                    1.0,
                    at([0.2, -0.1, 0.3], -1.2, tu, 0.3, tn),
                    1.59,
                ),
                |t| t.far_share,
            ),
            (
                "the phase",
                (
                    [0.2, -0.1, 0.3],
                    tn,
                    tu,
                    10.0,
                    at(at([0.2, -0.1, 0.3], -0.4, tu, 0.0, tv), 0.0, tu, -5.0, tn),
                    11.0,
                ),
                |t| t.phase,
            ),
        ];
        for (label, (c, n, u, rho, sc, r), term) in poses {
            let t = SlackTerms::of(c, n, u, rho, sc, r);
            let (full, share) = (t.full(), term(&t));
            let zero = (full - share) * (1.0 + 1e-9);
            assert!(
                share > 1e-3 * full && full <= 10.0 * zero,
                "{label}: the term is a share of the slack: {share:e} of {full:e}"
            );
            assert!(
                t.noise <= zero && t.lo.abs() >= 10.0 * zero && t.hi.abs() >= 10.0 * zero,
                "{label}: the pose reaches the slack (noise {:e}, extremes {:e}, {:e}, zero {zero:e})",
                t.noise,
                t.lo,
                t.hi
            );
            let got = circle_sphere_roots(
                &framed(c, n, u, rho),
                -4.0,
                4.0,
                &sphere(sc, r),
                Band::new(zero, 10.0 * zero).unwrap(),
            )
            .unwrap();
            assert!(
                matches!(got, CircleRoots::Uncertain),
                "{label}: the slack with this term lies in the gap, so the door refuses, got {got:?}"
            );
        }
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
        let CircleRoots::Certified {
            count: 2,
            thetas: [ts @ .., _, _],
        } = got
        else {
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
