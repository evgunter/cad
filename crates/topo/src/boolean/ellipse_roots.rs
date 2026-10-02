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
//!   Against a sphere `A₂ = (a² − b²)/4r`, which a minted ellipse holds
//!   definitely positive at any ordinary scale.
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
    first_harmonic_roots, half_angle_roots, rounding_charge,
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
        root_slack: "bool_ellipse_ladder_root_slack",
        quartic: QuarticRows {
            disc: "bool_ellipse_disc",
            shape: "bool_ellipse_shape",
            depth: "bool_ellipse_depth",
            odd: "bool_ellipse_odd",
            split: "bool_ellipse_split",
            split_lead: "bool_ellipse_split_lead",
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
        let first = FirstHarmonic {
            c0: h.c0,
            a1: hypot(h.c1, h.s1),
            cos_part: h.c1,
            sin_part: h.s1,
            noise: noise + second,
        };
        return first_harmonic_roots(&first, conic.major, t0, t1, &first_rows(decision), band);
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
            speed_lo: conic.minor,
            speed_hi: conic.major,
            lever: two * conic.minor,
            noise,
            f_per_metre: T::one(),
        },
        &ladder_rows(decision),
        band,
    )
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod tests {
    //! Each pose is checked against the geometry, not the door's own
    //! algebra: a certified root must put the carrier ON the surface, and
    //! its count must be the count of sign changes of the true distance
    //! along the carrier, each bisected to the bit.

    use core::f64::consts::{PI, TAU};

    use super::*;
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

    /// The true signed distance of `p` from the surface.
    fn distance(s: &geom::Surface<f64>, p: Point3<f64>) -> f64 {
        match *s {
            geom::Surface::Sphere { center, radius, .. } => (p - center).norm() - radius,
            geom::Surface::Cylinder {
                origin,
                axis,
                radius,
                ..
            } => {
                let w = p - origin;
                (w - axis * w.dot(axis)).norm() - radius
            }
            _ => unreachable!(),
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
