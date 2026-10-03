//! **The ellipse root door**: the certified crossings of an ELLIPSE
//! carrier with a sphere or a cylinder wall. Like the circle doors it
//! owns no root machinery: it reads the residual's harmonics from their
//! one home and hands them to one of the shared root cores
//! ([`super::circle_roots`]), whose answer it gives.
//!
//! # The residual is a degree-2 trigonometric polynomial
//!
//! With `C(θ) = C₀ + a·û cos θ + b·v̂ sin θ` (`θ` the eccentric anomaly)
//! and either surface written as the quadric `(|⊥(p − o)|² − r²)/2r`
//! (`⊥` the identity on a sphere, the projection off the axis on a
//! wall), `⊥(C(θ) − o)` is a first harmonic in `θ`, so the linearized
//! residual is EXACTLY `c₀ + c₁ cos θ + s₁ sin θ + c₂ cos 2θ + s₂ sin 2θ`
//! metres (`geom_brep::ConicHarmonics`). The noise meter's floor on `|F|`
//! per metre of residual is therefore `1`, and there are at most four
//! crossings per turn. An in-band sign escalates as the circle doors'
//! own decision on that surface ([`BooleanDecision::ArcSphereRoots`],
//! [`BooleanDecision::ArcCylinderRoots`]): the question decided is the
//! same one, whether an arc crosses the surface.
//!
//! # Two arms, by the second harmonic
//!
//! `bool_ellipse_second_harmonic` decides `A₂ = |(c₂, s₂)|`, the second
//! harmonic's amplitude in metres.
//!
//! - **The first-harmonic arm — `A₂` in the zero band**: the residual is
//!   a first harmonic to within `A₂`, which is charged to its noise, and
//!   the shared first-harmonic door decides it on its exact extremes
//!   under the `bool_ellipse_first_*` rows. This is the ellipse whose
//!   projection off the surface's axis is a circle — on a wall, the
//!   tilted section of that wall or of a coaxial one, whose residual is
//!   CONSTANT (on the wall, or a definite miss). As on the circle doors,
//!   the arm is required: the ladder cannot read a constant residual,
//!   and a tangency must reach the extremes' zero band, not the ladder's
//!   discriminant
//!   (`work/germ/the-half-angle-ladder-certifies-in-band-configurations.md`).
//!   Against a sphere `A₂ = |a² − b²|/4r`, in the zero band only for
//!   semi-axes within about `4r·zero/(|a| + |b|)` of each other — a near
//!   circle, which then takes this arm as a circle would, its `A₂`
//!   charged.
//! - **The ladder — `A₂` definite, or in the band's gap**: the five
//!   harmonics go to the shared half-angle ladder under the
//!   `bool_ellipse_*` rows. The carrier's speed `|C′|` lies in `[b, a]`,
//!   which the ladder's frame carries as such, so its root variable is
//!   `τ = 2b·tan(φ/2)`; its lever is that variable's own scale `2b`, as
//!   a circle's is `2ρ` on the cylinder door, and for the same reason it
//!   is not clamped by the sphere's size: the roots spread along the
//!   carrier, not across the surface.

use geom_core::{Band, Decide, Margin, Sign};

use super::circle_roots::{
    CircleRoots, FirstHarmonic, FirstHarmonicRows, HalfAngleFrame, HalfAngleRows, Harmonics,
    SubdivisionRows, first_harmonic_roots, half_angle_roots, rounding_charge,
};
use super::solid_contain::QuarticRows;
use super::{BooleanDecision, BooleanError};
use crate::validate::decide;

/// The first-harmonic arm's rows, per surface decision.
const fn first_rows(decision: BooleanDecision) -> FirstHarmonicRows {
    FirstHarmonicRows {
        noise: "bool_ellipse_first_noise",
        coaxial: "bool_ellipse_first_constant",
        extreme: "bool_ellipse_first_extreme",
        root_slack: "bool_ellipse_first_root_slack",
        decision,
    }
}

/// The ladder's rows, per surface decision.
const fn ladder_rows(decision: BooleanDecision) -> HalfAngleRows {
    HalfAngleRows {
        pole: "bool_ellipse_pole",
        conditioning: "bool_ellipse_pole_conditioning",
        noise: "bool_ellipse_ladder_noise",
        quartic: QuarticRows {
            disc: "bool_ellipse_disc",
            shape: "bool_ellipse_shape",
            depth: "bool_ellipse_depth",
            odd: "bool_ellipse_odd",
            split: "bool_ellipse_split",
            split_lead: "bool_ellipse_split_lead",
        },
        verify: SubdivisionRows {
            clear: "bool_ellipse_sub_clear",
            monotone: "bool_ellipse_sub_monotone",
            side: "bool_ellipse_sub_side",
            width: "bool_ellipse_sub_width",
        },
        decision,
    }
}

/// The certified crossings of the `carrier` ellipse with `surface`, a
/// sphere or a cylinder wall, reported within `π` of the midpoint of
/// `[t0, t1]` (module docs).
///
/// # Errors
///
/// [`BooleanError::ClassificationInvariant`] when `carrier` is not an
/// ellipse or `surface` neither a sphere nor a cylinder — the caller
/// dispatched on those kinds, so a mismatch is a desync, never an
/// answer. An escalation as the surface's arc decision for an in-band
/// classifying sign: an extreme or constant residual of the
/// first-harmonic arm, or a rung of the ladder. A second harmonic in the
/// band's GAP is not an error: it takes the ladder.
pub(super) fn ellipse_roots<T: Decide>(
    carrier: &geom::Curve3<T>,
    t0: T,
    t1: T,
    surface: &geom::Surface<T>,
    band: Band,
) -> Result<CircleRoots<T>, BooleanError> {
    let desync = || BooleanError::ClassificationInvariant {
        what: "the ellipse root door was handed a carrier that is not an ellipse or a surface \
               that is neither a sphere nor a cylinder",
    };
    let conic = match carrier {
        geom::Curve3::Ellipse { .. } => geom_brep::Conic::of(carrier),
        _ => None,
    }
    .ok_or_else(desync)?;
    let (h, r, decision) = match *surface {
        geom::Surface::Sphere { center, radius, .. } => (
            geom_brep::conic_sphere_harmonics(&conic, center, radius),
            radius,
            BooleanDecision::ArcSphereRoots,
        ),
        geom::Surface::Cylinder {
            origin,
            axis,
            radius,
            ..
        } => (
            geom_brep::conic_cylinder_harmonics(&conic, origin, axis, radius),
            radius,
            BooleanDecision::ArcCylinderRoots,
        ),
        _ => return Err(desync()),
    };
    let two = T::from_f64(2.0);
    // The harmonics' rounding, in residual metres: the term bound is in
    // m², before the `2r` division.
    let noise = rounding_charge(h.terms) / (two * r);
    let hypot = |x: T, y: T| (x.powi(2) + y.powi(2)).sqrt();
    let second = hypot(h.c2, h.s2);
    if let Ok(Sign::Zero) = decide("bool_ellipse_second_harmonic", Margin::of(second), band) {
        // The dropped second harmonic is charged to both extremes' noise.
        let (a1, noise) = (hypot(h.c1, h.s1), noise + second);
        let first = FirstHarmonic {
            lo: h.c0 - a1,
            hi: h.c0 + a1,
            cos_part: h.c1,
            sin_part: h.s1,
            lo_noise: noise,
            hi_noise: noise,
            phase_noise: T::zero(),
        };
        return first_harmonic_roots(
            &first,
            conic.speed_hi(),
            t0,
            t1,
            &first_rows(decision),
            band,
        );
    }
    half_angle_roots(
        &Harmonics {
            c0: h.c0,
            c1: h.c1,
            s1: h.s1,
            c2: h.c2,
            s2: h.s2,
        },
        |theta| geom_brep::implicit_residual(surface, conic.point(theta)),
        HalfAngleFrame {
            t0,
            t1,
            speed_lo: conic.speed_lo(),
            speed_hi: conic.speed_hi(),
            lever: two * conic.speed_lo(),
            noise,
            f_per_metre: T::one(),
        },
        &ladder_rows(decision),
        band,
    )
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    //! Each pose is checked against the geometry, not the door's own
    //! algebra: a certified root must put the carrier ON the surface, and
    //! its count must be the count of sign changes of the true distance
    //! along the carrier, each bisected to the bit.

    use core::f64::consts::{PI, TAU};

    use super::*;
    use crate::boolean::conic_oracle::distance;
    use geom_core::{Point3, Tol, Vec3};

    fn band() -> Band {
        Band::linear(Tol::witness()).unwrap()
    }

    /// The ellipse `c + a·û cos θ + b·v̂ sin θ` on the unit normal `n`.
    fn ellipse(c: [f64; 3], n: [f64; 3], u: [f64; 3], a: f64, b: f64) -> geom::Curve3<f64> {
        let n = Vec3::from_array(n).normalize();
        let u = Vec3::from_array(u);
        geom::Curve3::Ellipse {
            center: Point3::from_array(c),
            axis: n,
            major: a,
            minor: b,
            u_ref: (u - n * u.dot(n)).normalize(),
        }
    }

    fn sphere(c: [f64; 3], r: f64) -> geom::Surface<f64> {
        geom::Surface::Sphere {
            center: Point3::from_array(c),
            radius: r,
            axis: Vec3::new(0.0, 0.0, 1.0),
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        }
    }

    fn wall(o: [f64; 3], axis: [f64; 3], r: f64) -> geom::Surface<f64> {
        let axis = Vec3::from_array(axis).normalize();
        let x = Vec3::new(1.0, 0.0, 0.0);
        geom::Surface::Cylinder {
            origin: Point3::from_array(o),
            axis,
            radius: r,
            u_ref: (x - axis * x.dot(axis)).normalize(),
        }
    }

    /// The sign changes of the true distance on `[t0, t1]`.
    fn oracle(e: &geom::Curve3<f64>, s: &geom::Surface<f64>, t0: f64, t1: f64) -> Vec<f64> {
        let f = |t: f64| distance(s, e.eval(t));
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

    fn door(e: &geom::Curve3<f64>, s: &geom::Surface<f64>, t0: f64, t1: f64) -> CircleRoots<f64> {
        ellipse_roots(e, t0, t1, s, band()).unwrap()
    }

    /// Certified roots matching the oracle in number and place, every
    /// one on the surface and within `π` of the arc's midpoint.
    fn assert_matches_oracle(
        label: &str,
        e: &geom::Curve3<f64>,
        s: &geom::Surface<f64>,
        t0: f64,
        want: usize,
    ) {
        let t1 = t0 + 1.0;
        let CircleRoots::Certified { count, thetas } = door(e, s, t0, t1) else {
            panic!("{label}: certified roots, got {:?}", door(e, s, t0, t1));
        };
        assert_eq!(count, want, "{label}: the certified count");
        let mid = (t0 + t1) / 2.0;
        let mut got = thetas[..count].to_vec();
        for &t in &got {
            assert!((t - mid).abs() <= PI, "{label}: {t} within π of {mid}");
            let off = distance(s, e.eval(t)).abs();
            assert!(off < 1e-12, "{label}: root {t} lies {off} off the surface");
        }
        got.sort_by(f64::total_cmp);
        let truth = oracle(e, s, mid - PI, mid + PI);
        assert_eq!(got.len(), truth.len(), "{label}: {got:?} vs {truth:?}");
        for (a, b) in got.iter().zip(&truth) {
            assert!(
                (a - b).abs() < 1e-9,
                "{label}: root {a} vs the oracle's {b}"
            );
        }
    }

    /// Crossings of a sphere and of a wall, two and four per turn, at
    /// several arcs (the roots are reported about each arc's midpoint).
    #[test]
    fn crossings_match_the_true_distance() {
        let thin = ellipse([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], [1.0, 0.0, 0.0], 1.0, 0.3);
        let tilted = ellipse([0.1, 0.2, 0.0], [0.3, -0.2, 1.0], [1.0, 1.0, 0.0], 0.8, 0.5);
        for t0 in [0.0, 1.7, -2.9] {
            // A sphere through both ends of the major axis' span: the
            // ellipse leaves it near each vertex and comes back.
            assert_matches_oracle("thin × sphere", &thin, &sphere([0.0, 0.0, 0.1], 0.6), t0, 4);
            assert_matches_oracle(
                "thin × off-centre sphere",
                &thin,
                &sphere([0.9, 0.0, 0.0], 0.4),
                t0,
                2,
            );
            assert_matches_oracle(
                "tilted × sphere",
                &tilted,
                &sphere([0.0, 0.0, 0.3], 0.7),
                t0,
                2,
            );
            assert_matches_oracle(
                "thin × upright wall",
                &thin,
                &wall([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], 0.6),
                t0,
                4,
            );
            assert_matches_oracle(
                "tilted × leaning wall",
                &tilted,
                &wall([0.5, 0.0, 0.0], [0.2, 1.0, 0.4], 0.3),
                t0,
                2,
            );
        }
    }

    #[test]
    fn a_clear_carrier_is_a_miss() {
        let thin = ellipse([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], [1.0, 0.0, 0.0], 1.0, 0.3);
        for (label, s) in [
            ("inside a sphere", sphere([0.0, 0.0, 0.0], 1.4)),
            ("beside a sphere", sphere([0.0, 2.0, 0.0], 0.5)),
            ("inside a wall", wall([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], 1.2)),
        ] {
            assert!(
                matches!(door(&thin, &s, 0.0, TAU), CircleRoots::Miss),
                "{label}: got {:?}",
                door(&thin, &s, 0.0, TAU)
            );
            assert!(
                oracle(&thin, &s, 0.0, TAU).is_empty(),
                "{label}: the oracle agrees"
            );
        }
    }

    /// **The tilted section of a wall lies on it.** The plane through the
    /// wall's axis point at tilt `φ` cuts the wall of radius `r` in the
    /// ellipse of semi-axes `r/cos φ` and `r`, whose residual against
    /// that wall is identically zero: the first-harmonic arm reads it
    /// `OnSurface` (the ladder could not read a constant). Against a
    /// coaxial wall of another radius the residual is a definite constant,
    /// a `Miss`.
    #[test]
    fn a_wall_s_own_tilted_section_is_on_it() {
        let phi = 0.3_f64;
        let section = ellipse(
            [0.0, 0.0, 0.5],
            [phi.sin(), 0.0, phi.cos()],
            [1.0, 0.0, 0.0],
            0.5 / phi.cos(),
            0.5,
        );
        let on = wall([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], 0.5);
        assert!(
            matches!(door(&section, &on, 0.0, PI), CircleRoots::OnSurface),
            "its own wall: got {:?}",
            door(&section, &on, 0.0, PI)
        );
        let wider = wall([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], 0.7);
        assert!(
            matches!(door(&section, &wider, 0.0, PI), CircleRoots::Miss),
            "a coaxial wider wall: got {:?}",
            door(&section, &wider, 0.0, PI)
        );
    }

    /// **A graze is read by its depth, in the band's own metres.** An
    /// ellipse whose `y` semi-axis is the wall's radius plus `depth`, its
    /// plane tilted `tilt` about `x` (the semi-axis lengthened to match),
    /// crosses a wall about `z` near `θ = ±π/2` by `depth` (four roots)
    /// or, at a depth inside the band, grazes it. At the default band: a
    /// graze in the band is no certified answer either way, a definite
    /// depth is four certified crossings, each on the wall. The
    /// half-angle ladder alone certified a `Miss` at −1e-9 m on a 5 m
    /// wall and answered `CountDisagrees` at 1e-8 m on a 50 m wall and at
    /// 2e-8 m on a 500 m one.
    #[test]
    fn a_graze_is_read_by_its_depth() {
        let band = Band::new(1e-9, 1e-8).unwrap();
        for (r, depth, tilt) in [
            (5.0, -1e-9, 0.0_f64),
            (50.0, 1e-8, 0.0),
            (500.0, 2e-8, 0.0),
            (50.0, 2e-8, 0.3),
            (5.0, 5e-6, 0.3),
        ] {
            let label = format!("wall r {r}, depth {depth}, tilt {tilt}");
            let e = ellipse(
                [0.0; 3],
                [0.0, -tilt.sin(), tilt.cos()],
                [1.0, 0.0, 0.0],
                0.6 * r,
                (r + depth) / tilt.cos(),
            );
            let w = wall([0.0; 3], [0.0, 0.0, 1.0], r);
            let got = ellipse_roots(&e, 0.0, PI, &w, band);
            if f64::abs(depth) <= band.zero() * 10.0 {
                assert!(
                    matches!(got, Ok(CircleRoots::Uncertain) | Err(_)),
                    "{label}: a graze in the band, got {got:?}"
                );
                continue;
            }
            let Ok(CircleRoots::Certified { count, thetas }) = got else {
                panic!("{label}: four certified crossings, got {got:?}");
            };
            assert_eq!(count, 4, "{label}");
            for &t in &thetas[..count] {
                let p = e.eval(t);
                let off = (p.x.hypot(p.y) - r).abs();
                assert!(
                    off <= band.zero(),
                    "{label}: root {t} lies {off} off the wall"
                );
            }
        }
    }

    /// **The subdivision reads its pieces at the carrier's top speed.** A
    /// millimetre ellipse (semi-axes 1.16 mm and 0.18 mm) against a
    /// sphere of radius 0.21 mm, at ε = 1e-6: two crossings, certified,
    /// each on the sphere. Read at the semi-minor axis, the subdivision
    /// takes its pieces' arc lengths for six times shorter than they are,
    /// calls a piece down to the band before it has isolated the roots,
    /// and declines — the pose the counterexample search found to tell the
    /// two apart.
    #[test]
    fn the_subdivision_reads_its_pieces_at_the_top_speed() {
        let band = Band::new(1e-6, 1e-5).expect("a band");
        let e = geom::Curve3::Ellipse {
            center: Point3::new(0.0, 0.0, 0.0),
            axis: Vec3::new(
                -0.02690652087259189,
                -0.6904757134714763,
                0.7228549842399846,
            ),
            major: 0.0011565902894188694,
            minor: 0.00017995811133866398,
            u_ref: Vec3::new(-0.8791819779574158, 0.360475481131658, 0.3116030762649587),
        };
        let center = Point3::new(
            -0.0007969769311573957,
            9.88085673337834e-6,
            0.00019172775206061993,
        );
        let radius = 0.00021405666180469222;
        let s = geom::Surface::Sphere {
            center,
            radius,
            axis: Vec3::new(
                -0.018144408265784995,
                0.9651190040905697,
                -0.2611820981459323,
            ),
            u_ref: Vec3::new(
                0.9998353766739225,
                0.017514396513495407,
                -0.004739774897982686,
            ),
        };
        let got = ellipse_roots(&e, 3.511343352233997, 7.298353179068743, &s, band);
        let Ok(CircleRoots::Certified { count, thetas }) = got else {
            panic!("certified crossings, got {got:?}");
        };
        assert!(count > 0, "crossings");
        for &t in &thetas[..count] {
            let off = ((e.eval(t) - center).norm() - radius).abs();
            assert!(off <= band.zero(), "root {t} lies {off} off the sphere");
        }
    }

    /// **The first-harmonic arm places its roots within the band along
    /// the arc.** The section of a unit wall by a plane tilted
    /// `acos(1/20)` (semi-axes 20 and 1), its `y` semi-axis lengthened by
    /// `8e-11` so the second harmonic against the test wall is about
    /// 2e-11 m (in the zero band: this arm drops it and charges it to the
    /// noise), crosses a wall of radius `2 − h` centred at `(0, −1)` by
    /// `h`, its roots nearest `θ = π/2` where the carrier runs at
    /// 20 m/rad. Over depths from grazing to deep, every certified root
    /// lies within the escalation threshold, as arc length, of the TRUE
    /// crossing. The dropped harmonic moves a root by `A₂/|R′|` radians,
    /// so the arm's root slack must charge it and read it at the
    /// carrier's top speed: uncharged, or read at the semi-minor axis
    /// (`a/b` past the band's `escalate/zero`), it certifies shallow roots
    /// up to twice the threshold off.
    #[test]
    fn the_first_harmonic_arm_places_its_roots_along_the_arc() {
        let band = Band::new(1e-9, 1e-8).expect("a band");
        let phi = (1.0_f64 / 20.0).acos();
        let section = ellipse(
            [0.0; 3],
            [phi.sin(), 0.0, phi.cos()],
            [1.0, 0.0, 0.0],
            1.0 / phi.cos(),
            1.0 + 8e-11,
        );
        let mut certified = 0;
        for k in 0..=80 {
            let h = 10f64.powf(-6.0 + 5.7 * f64::from(k) / 80.0);
            let w = wall([0.0, -1.0, 0.0], [0.0, 0.0, 1.0], 2.0 - h);
            let (t0, t1) = (-0.2, PI + 0.2);
            let Ok(CircleRoots::Certified { count, thetas }) =
                ellipse_roots(&section, t0, t1, &w, band)
            else {
                continue;
            };
            certified += 1;
            let truth = oracle(&section, &w, t0, t1);
            for &t in &thetas[..count] {
                let nearest = truth
                    .iter()
                    .copied()
                    .min_by(|a, b| (a - t).abs().total_cmp(&(b - t).abs()))
                    .expect("a true crossing on the arc");
                let speed = {
                    let (s, c) = nearest.sin_cos();
                    ((s / phi.cos()).powi(2) + c.powi(2)).sqrt()
                };
                let off = (t - nearest).abs() * speed;
                assert!(
                    off <= band.escalate(),
                    "depth {h}: root {t} is {off} m of arc from the crossing {nearest}"
                );
            }
        }
        assert!(
            certified > 5,
            "the sweep certifies its deep crossings: {certified}"
        );
    }

    /// A carrier that is not an ellipse, or a surface the door has no
    /// harmonics for, is a dispatch desync.
    #[test]
    fn a_wrong_kind_is_a_desync() {
        let circle = geom::Curve3::Circle {
            center: Point3::new(0.0, 0.0, 0.0),
            axis: Vec3::new(0.0, 0.0, 1.0),
            radius: 1.0,
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        };
        let s = sphere([0.0, 0.0, 0.0], 0.5);
        assert!(matches!(
            ellipse_roots(&circle, 0.0, 1.0, &s, band()),
            Err(BooleanError::ClassificationInvariant { .. })
        ));
        let e = ellipse([0.0; 3], [0.0, 0.0, 1.0], [1.0, 0.0, 0.0], 1.0, 0.5);
        let torus = geom::Surface::Torus {
            center: Point3::new(0.0, 0.0, 0.0),
            axis: Vec3::new(0.0, 0.0, 1.0),
            major_radius: 1.0,
            minor_radius: 0.2,
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        };
        assert!(matches!(
            ellipse_roots(&e, 0.0, 1.0, &torus, band()),
            Err(BooleanError::ClassificationInvariant { .. })
        ));
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod fuzz_rows {
    //! A counterexample search at millimetre scale and high eccentricity,
    //! the configurations where the ladder's root variable distorts arc
    //! length most, checked against the TRUE distance, at three bands.

    use core::f64::consts::{PI, TAU};

    use super::*;
    use crate::boolean::conic_oracle::{distance, sign_changes, unit};
    use geom_core::{Point3, Vec3};
    use test_utils::fuzz;

    /// **Every certified answer is true of the geometry.** For random
    /// ellipses (semi-axes 0.5–5 mm, eccentricity up to 25, any stored
    /// order and sign) against random walls and spheres posed to cross
    /// them:
    ///
    /// - every certified root lies on the surface, its TRUE distance
    ///   inside the zero band;
    /// - a certified count is never below the true sign changes, which a
    ///   dense sampling of the true distance finds;
    /// - a `Miss` is never certified for a carrier that comes within the
    ///   zero band of the surface, or crosses it.
    ///
    /// At ε = 1e-6 the ladder alone certified roots up to 1.9e-5 m of arc
    /// off, 15 zero bands off the wall. The `Uncertain` count is printed.
    #[test]
    fn certified_answers_hold_against_the_true_distance() {
        hold_against_the_true_distance(&mut fuzz::start("ellipse_roots::certified_answers_hold"));
    }

    /// **The pinned counterexample.** This seed reproduces the ladder's
    /// certified roots off the surface at ε = 1e-6 (an eccentric
    /// millimetre ellipse, its root several zero bands off the wall), the
    /// defect [`certified_answers_hold_against_the_true_distance`] found;
    /// the draws are too many to write out as a literal.
    #[test]
    fn the_ladders_off_surface_roots_stay_answered_truly() {
        hold_against_the_true_distance(&mut fuzz::pinned(
            "ellipse_roots::the_ladders_off_surface_roots",
            0xdd46_6c66_b273_5249,
        ));
    }

    fn hold_against_the_true_distance(rng: &mut fuzz::Rng) {
        let dense = 20_000;
        for eps in [1e-6, 1e-9, 1e-12] {
            let band = Band::new(eps, 10.0 * eps).unwrap();
            let (mut certified, mut misses, mut uncertain, mut escalated) = (0, 0, 0, 0);
            for i in 0..fuzz::scaled(600) {
                let n = unit(rng);
                let u = unit(rng);
                let big = rng.range(5e-4, 5e-3);
                let small = big / rng.range(1.0, 25.0);
                let (major, minor) = if rng.below(2) == 0 {
                    (big, small)
                } else {
                    (small, big)
                };
                // Either semi-axis stored negative, on a cycle of the case
                // number rather than a draw, so the pinned seed's draws
                // are the ones it was pinned on.
                let major = if i % 3 == 1 { -major } else { major };
                let minor = if i % 5 >= 3 { -minor } else { minor };
                let e = geom::Curve3::Ellipse {
                    center: Point3::new(0.0, 0.0, 0.0),
                    axis: n,
                    major,
                    minor,
                    u_ref: (u - n * u.dot(n)).normalize(),
                };
                // A surface through a random point of the carrier, so the
                // pose crosses or grazes it more often than not.
                let through = e.eval(rng.range(0.0, TAU));
                let r = rng.range(2e-4, 3e-3);
                let w = unit(rng);
                let x = Vec3::new(1.0, 0.0, 0.0);
                let s = if i % 2 == 0 {
                    geom::Surface::Cylinder {
                        origin: through + unit(rng).cross(w).normalize() * r,
                        axis: w,
                        radius: r,
                        u_ref: (x - w * x.dot(w)).normalize(),
                    }
                } else {
                    geom::Surface::Sphere {
                        center: through + unit(rng) * r,
                        radius: r,
                        axis: w,
                        u_ref: (x - w * x.dot(w)).normalize(),
                    }
                };
                let t0 = rng.range(0.0, TAU);
                let t1 = t0 + rng.range(0.1, TAU);
                let mid = (t0 + t1) / 2.0;
                let Ok(found) = ellipse_roots(&e, t0, t1, &s, band) else {
                    escalated += 1;
                    continue;
                };
                let samples: Vec<f64> = (0..=dense)
                    .map(|k| distance(&s, e.eval(mid - PI + TAU * f64::from(k) / f64::from(dense))))
                    .collect();
                let changes = sign_changes(&samples);
                let closest = samples.iter().fold(f64::INFINITY, |m, d| m.min(d.abs()));
                let label = format!(
                    "ε {eps}, case {i}: {e:?} against {s:?} on [{t0}, {t1}] — {}",
                    fuzz::replay()
                );
                match found {
                    CircleRoots::Certified { count, thetas } => {
                        certified += 1;
                        for &t in &thetas[..count] {
                            let off = distance(&s, e.eval(t)).abs();
                            assert!(off <= eps, "{label}: root {t} lies {off} off the surface");
                        }
                        assert!(
                            count >= changes,
                            "{label}: {count} certified roots, {changes} sign changes"
                        );
                    }
                    CircleRoots::Miss => {
                        misses += 1;
                        assert!(
                            changes == 0 && closest > eps,
                            "{label}: a Miss {closest} from the surface, {changes} sign changes"
                        );
                    }
                    CircleRoots::Uncertain | CircleRoots::OnSurface => uncertain += 1,
                    CircleRoots::CountDisagrees => panic!("{label}: CountDisagrees"),
                }
            }
            println!(
                "ε {eps}: {certified} certified, {misses} misses, {uncertain} uncertain, \
                 {escalated} escalated"
            );
        }
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod graze_rows {
    //! Grazes at a vertex, by depths of a few bands either side of zero —
    //! the poses where a certified `Miss` is wrong if it is ever wrong —
    //! on circles and ellipses in every stored order and sign, through the
    //! degree-2 doors' certified subdivision.

    use core::f64::consts::FRAC_PI_2;

    use super::*;
    use crate::boolean::circle_cylinder::circle_cylinder_roots;
    use crate::boolean::conic_oracle::{distance, unit};
    use geom_core::{Point3, Vec3};
    use test_utils::fuzz;

    /// **A negative `major` is charged at its magnitude, at a metre and a
    /// kilometre out.** The graze fuzz's counterexample on the signed
    /// charge, pinned: an ellipse of `major = −3.41 m`, `minor = −0.18 m`
    /// crossing a 17 µm ball by 1.8e-11 m at ε = 1e-12, and the same pose
    /// carried 1 km out. The harmonics' rounding charged at the signed
    /// `major` came to `(|C₀ − o| − 3.41)²` — near zero at the metre pose,
    /// ~180× short at the kilometre one — and both read a certified
    /// `Miss`.
    #[test]
    fn a_negative_major_is_charged_its_magnitude() {
        let band = Band::new(1e-12, 1e-11).unwrap();
        let axis = Vec3::new(
            0.784_321_624_695_285_9,
            -0.311_122_040_118_748_94,
            0.536_696_064_069_501_7,
        );
        let u_ref = Vec3::new(
            0.620_298_357_327_510_8,
            0.404_949_218_494_292_5,
            -0.671_748_523_137_679_4,
        );
        let center = Point3::new(
            0.617_664_175_002_875_5,
            -0.050_650_563_273_673_79,
            0.736_877_021_230_139_7,
        );
        let hub = Point3::new(
            2.735_196_927_577_549,
            1.331_737_803_810_837_2,
            -1.556_292_842_004_988_2,
        );
        for out in [Vec3::new(0.0, 0.0, 0.0), Vec3::new(1000.0, -700.0, 400.0)] {
            let e = geom::Curve3::Ellipse {
                center: center + out,
                axis,
                major: -3.413_716_003_510_488,
                minor: -0.175_755_441_781_259_68,
                u_ref,
            };
            let s = geom::Surface::Sphere {
                center: hub + out,
                radius: 1.664_235_542_121_884e-5,
                axis,
                u_ref: Vec3::new(
                    0.620_354_405_993_338,
                    0.393_355_381_418_971_8,
                    -0.678_551_364_948_437_9,
                ),
            };
            let vertex = (0..4)
                .map(|k| FRAC_PI_2 * f64::from(k))
                .min_by(|&a, &b| {
                    let d = |t: f64| {
                        ((e.eval(t) - (hub + out)).norm() - 1.664_235_542_121_884e-5).abs()
                    };
                    d(a).total_cmp(&d(b))
                })
                .unwrap();
            let got = ellipse_roots(&e, vertex - 0.5, vertex + 0.5, &s, band);
            assert!(
                !matches!(got, Ok(CircleRoots::Miss)),
                "{out:?} out: a certified Miss for a crossing 1.8e-11 m deep"
            );
        }
    }

    /// **No certified `Miss` on a graze within the band, or across it.** A
    /// carrier (a circle, or an ellipse of semi-axes 0.5–5 m and
    /// eccentricity up to 40, stored in either order and either sign, its
    /// centre up to 1 km out) meets a small sphere or wall (radius
    /// 1–100 µm) at one of its vertices, the surface set off along the
    /// carrier's outward normal there by `gap` — drawn from −40 to 40
    /// bands — so the carrier's least distance from the surface IS `gap`
    /// (the carrier bends away from it on both sides). The wall's axis
    /// lies along the carrier's tangent at the vertex, so the door takes
    /// its subdivision, not a first-harmonic arm. Every certified root
    /// must read ON the surface (its true distance inside the zero band),
    /// and a `Miss` needs `gap` past the zero band.
    ///
    /// Charging the harmonics' rounding at the SIGNED `major` (a negative
    /// one under-charged ~180× at a kilometre) certified misses on
    /// crossings 1e-10 m deep at ε = 1e-12; dropping the subdivision's
    /// Taylor remainder certified misses on grazes at ε = 1e-9.
    #[test]
    fn no_certified_miss_on_a_graze() {
        let mut rng = fuzz::start("ellipse_roots::no_certified_miss_on_a_graze");
        for eps in [1e-12, 1e-9] {
            let band = Band::new(eps, 10.0 * eps).unwrap();
            let (mut certified, mut misses, mut declined) = (0, 0, 0);
            for i in 0..fuzz::scaled(800) {
                let n = unit(&mut rng);
                let u = unit(&mut rng);
                let u_ref = (u - n * u.dot(n)).normalize();
                let big = rng.range(0.5, 5.0);
                let circle = i % 4 == 0;
                let small = if circle {
                    big
                } else {
                    big / rng.range(1.0, 40.0)
                };
                let (mut major, mut minor) = if rng.below(2) == 0 {
                    (big, small)
                } else {
                    (small, big)
                };
                if !circle && rng.below(2) == 0 {
                    major = -major;
                }
                if !circle && rng.below(3) == 0 {
                    minor = -minor;
                }
                let far = if rng.below(3) == 0 { 1000.0 } else { 1.0 };
                let center = Point3::new(
                    rng.range(-far, far),
                    rng.range(-far, far),
                    rng.range(-far, far),
                );
                let e = if circle {
                    geom::Curve3::Circle {
                        center,
                        axis: n,
                        radius: major,
                        u_ref,
                    }
                } else {
                    geom::Curve3::Ellipse {
                        center,
                        axis: n,
                        major,
                        minor,
                        u_ref,
                    }
                };
                // A vertex: the ends of the stored axes.
                let k = rng.below(4);
                let vertex = FRAC_PI_2 * f64::from(u32::try_from(k).unwrap());
                let vertex_on_major = k.is_multiple_of(2);
                let p = e.eval(vertex);
                let outward = (p - center).normalize();
                let tangent = n.cross(outward);
                let gap = eps * rng.range(-40.0, 40.0);
                // Either a small sphere or wall just outside the vertex,
                // the carrier's distance from it least there; or the
                // vertex's OSCULATING sphere or wall, which meets the
                // carrier to fourth order (`F″ = 0` at the vertex), held
                // off by `gap` — the carrier's distance from it is
                // extreme at the vertex, least at the sharp vertex (the
                // osculating circle inside the carrier) and greatest at
                // the flat one.
                let osculating = !circle && rng.below(2) == 0;
                let (hub, r, wall_axis, least) = if osculating {
                    let this = if vertex_on_major {
                        major.abs()
                    } else {
                        minor.abs()
                    };
                    let other = if vertex_on_major {
                        minor.abs()
                    } else {
                        major.abs()
                    };
                    let rho = other * other / this;
                    let sharp = this >= other;
                    let shift = if sharp { gap } else { -gap };
                    (p - outward * (rho - shift), rho, n, sharp)
                } else {
                    let r = 10f64.powf(rng.range(-6.0, -4.0));
                    (p + outward * (r + gap), r, tangent, true)
                };
                let x = Vec3::new(1.0, 0.0, 0.0);
                let s = if rng.below(2) == 0 {
                    geom::Surface::Sphere {
                        center: hub,
                        radius: r,
                        axis: n,
                        u_ref: (x - n * x.dot(n)).normalize(),
                    }
                } else {
                    geom::Surface::Cylinder {
                        origin: hub,
                        axis: wall_axis,
                        radius: r,
                        u_ref: outward,
                    }
                };
                let (t0, t1) = (vertex - 0.5, vertex + 0.5);
                let got = if circle {
                    match s {
                        geom::Surface::Cylinder { .. } => {
                            circle_cylinder_roots(&e, t0, t1, &s, band)
                        }
                        _ => continue,
                    }
                } else {
                    ellipse_roots(&e, t0, t1, &s, band)
                };
                let distance = |q: Point3<f64>| distance(&s, q);
                let label = format!(
                    "ε {eps}, case {i}: gap {gap:e}, osculating {osculating}, {e:?} against {s:?} — {}",
                    fuzz::replay()
                );
                match got {
                    Ok(CircleRoots::Miss) => {
                        misses += 1;
                        // Clear only when the extreme distance, at the
                        // vertex, is past the band on the far side.
                        let at = distance(p);
                        let clear = if least { at > eps } else { at < -eps };
                        assert!(clear, "{label}: a certified Miss ({at:e} at the vertex)");
                    }
                    Ok(CircleRoots::Certified { count, thetas }) => {
                        certified += 1;
                        for &t in &thetas[..count] {
                            let off = distance(e.eval(t)).abs();
                            assert!(off <= eps, "{label}: root {t} lies {off} off");
                        }
                    }
                    Ok(CircleRoots::CountDisagrees) => panic!("{label}: CountDisagrees"),
                    _ => declined += 1,
                }
            }
            println!("ε {eps}: {certified} certified, {misses} misses, {declined} declined");
        }
    }
}
