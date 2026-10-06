//! **The conic × quadric root door**: the certified crossings of a
//! CIRCLE or ELLIPSE carrier with a sphere or a cylinder wall. It owns
//! no root machinery: it reads the residual's harmonics from their one
//! home and hands them to one of the shared root cores
//! ([`super::circle_roots`]), whose answer it gives.
//!
//! # The residual is a degree-2 trigonometric polynomial
//!
//! With `C(θ) = C₀ + a·û cos θ + b·v̂ sin θ` (`θ` the eccentric anomaly,
//! a circle's angle when `a = b = ρ`) and either surface written as the
//! quadric `(|⊥(p − o)|² − r²)/2r` (`⊥` the identity on a sphere, the
//! projection off the axis on a wall), `⊥(C(θ) − o)` is a first harmonic
//! in `θ`, so the linearized residual is EXACTLY
//! `c₀ + c₁ cos θ + s₁ sin θ + c₂ cos 2θ + s₂ sin 2θ` metres
//! (`geom_brep::ConicHarmonics`). The noise meter's floor on `|F|` per
//! metre of residual is therefore `1`, an identity rather than a
//! neighbourhood bound, and there are at most four crossings per turn.
//! An in-band sign escalates as the surface's arc decision
//! ([`BooleanDecision::ArcSphereRoots`],
//! [`BooleanDecision::ArcCylinderRoots`]).
//!
//! # Two arms, by the second harmonic
//!
//! `bool_conic_quadric_second_harmonic` decides `A₂ = |(c₂, s₂)|`, the
//! second harmonic's amplitude in metres.
//!
//! - **The first-harmonic arm — `A₂` in the zero band**: the residual is
//!   a first harmonic to within `A₂`, which is charged to both extremes'
//!   noise, and the shared first-harmonic door decides it on its exact
//!   extremes under the `bool_conic_quadric_first_*` rows. The arm is
//!   REQUIRED, for two poses the ladder cannot answer truthfully: a
//!   CONSTANT residual (`F·(1 + t²)²`, which the ladder cannot read; the
//!   extremes answer on-surface or miss), and a TANGENCY, which the
//!   extremes put in the zero band and escalate, where the ladder can
//!   certify a miss
//!   (`work/germ/the-half-angle-ladder-certifies-in-band-configurations.md`).
//!   Its poses:
//!   - a circle square to a wall's axis, or within `2√(r·zero)` of
//!     square: on a circle `A₂ = (ρ·sin α)²/4r`, `α` its tilt to the
//!     axis; a COAXIAL one has a constant residual (a rim circle of the
//!     same wall, or a miss);
//!   - an ellipse whose projection off a wall's axis is a circle — the
//!     tilted section of that wall or of a coaxial one, whose residual is
//!     constant — or within the band of one;
//!   - against a sphere, an ellipse whose semi-axes are within about
//!     `4r·zero/(|a| + |b|)` of each other (`A₂ = |a² − b²|/4r`);
//!   - **a circle against a sphere, `A₂ ≡ 0`**: no decision is taken.
//!     Its first harmonic is read in its factored form
//!     (`geom_brep::circle_sphere_harmonic`): the extremes
//!     `(D∓ − r)(D∓ + r)/2r`, `D∓` the distances from the sphere's centre
//!     to the circle's nearest and farthest points, each with a running
//!     bound on its rounding, so a shallow crossing is placed as well as
//!     its near extreme is evaluated rather than as well as the
//!     harmonics' m² terms are. A frame that is not orthonormal moves the
//!     circle off that form, and its defect is charged to both extremes
//!     (`geom_brep::CircleSphereHarmonic::frame_error`). Every circle
//!     whose axis passes through the sphere's centre has a constant
//!     residual, and lies ON the sphere when it is zero.
//! - **The ladder — `A₂` definite, or in the band's gap**: the five
//!   harmonics go to the shared half-angle ladder under the
//!   `bool_conic_quadric_*` ladder rows. The carrier's speed `|C′|` lies
//!   in `[b, a]`, which the ladder's frame carries as such, so its root
//!   variable is `τ = 2b·tan(φ/2)`; its lever is that variable's own
//!   scale `2b` (a circle's `2ρ`), and it is not clamped by the surface's
//!   size: the roots spread along the carrier, not across the surface (a
//!   circle in a plane through a wall's axis meets it at points a whole
//!   diameter apart whatever `r` is).
//!
//! The two arms' meters keep their cores' postures, which differ on a
//! reading in the band's gap: the first-harmonic arm's refuse it, the
//! ladder's pass it (the circle root cores' module docs, "The ladder's
//! noise meter") — which is why their rows have distinct names.

use geom_core::{Band, Decide, Margin, Sign};

use super::circle_roots::{
    CircleRoots, FirstHarmonic, FirstHarmonicRows, HalfAngleFrame, HalfAngleRows, RootSlack,
    SubdivisionFrame, SubdivisionRows, TrigPoly, first_harmonic_roots, half_angle_roots,
    rounding_charge,
};
use super::solid_contain::QuarticRows;
use super::{BooleanDecision, BooleanError};
use crate::validate::decide;

#[cfg(test)]
mod circle_sphere_rows;
#[cfg(test)]
mod circle_wall_rows;
#[cfg(test)]
mod ellipse_rows;

/// The arm switch's row (module docs, "Two arms").
const SECOND_HARMONIC: &str = "bool_conic_quadric_second_harmonic";

/// The first-harmonic arm's rows, per surface decision.
const fn first_rows(decision: BooleanDecision) -> FirstHarmonicRows {
    FirstHarmonicRows {
        noise: "bool_conic_quadric_first_noise",
        coaxial: "bool_conic_quadric_first_constant",
        extreme: "bool_conic_quadric_first_extreme",
        root_slack: "bool_conic_quadric_first_root_slack",
        decision,
    }
}

/// The ladder's rows, per surface decision.
const fn ladder_rows(decision: BooleanDecision) -> HalfAngleRows {
    HalfAngleRows {
        pole: "bool_conic_quadric_pole",
        conditioning: "bool_conic_quadric_pole_conditioning",
        noise: "bool_conic_quadric_ladder_noise",
        quartic: QuarticRows {
            disc: "bool_conic_quadric_disc",
            shape: "bool_conic_quadric_shape",
            depth: "bool_conic_quadric_depth",
            odd: "bool_conic_quadric_odd",
            split: "bool_conic_quadric_split",
            split_lead: "bool_conic_quadric_split_lead",
        },
        verify: SubdivisionRows {
            clear: "bool_conic_quadric_sub_clear",
            monotone: "bool_conic_quadric_sub_monotone",
            side: "bool_conic_quadric_sub_side",
            width: "bool_conic_quadric_sub_width",
        },
        decision,
    }
}

/// The certified crossings of the `carrier` circle or ellipse with
/// `surface`, a sphere or a cylinder wall, reported within `π` of the
/// midpoint of `[t0, t1]` (module docs).
///
/// # Errors
///
/// [`BooleanError::ClassificationInvariant`] when `carrier` is neither a
/// circle nor an ellipse or `surface` neither a sphere nor a cylinder —
/// the caller dispatched on those kinds, so a mismatch is a desync,
/// never an answer. An escalation as the surface's arc decision for an
/// in-band classifying sign: an extreme or constant residual of the
/// first-harmonic arm, or a rung of the ladder. A second harmonic in the
/// band's GAP is not an error: it takes the ladder.
pub(super) fn conic_quadric_roots<T: Decide>(
    carrier: &geom::Curve3<T>,
    t0: T,
    t1: T,
    surface: &geom::Surface<T>,
    band: Band,
) -> Result<CircleRoots<T>, BooleanError> {
    let desync = || BooleanError::ClassificationInvariant {
        what: "the conic × quadric root door was handed a carrier that is not a circle or an \
               ellipse or a surface that is not a sphere or a cylinder",
    };
    let conic = geom_brep::Conic::of(carrier).ok_or_else(desync)?;
    let (h, decision) = match (carrier, surface) {
        (
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
        ) => {
            // `A₂ ≡ 0`: the first harmonic, in its factored form.
            let h =
                geom_brep::circle_sphere_harmonic(center, axis, radius, u_ref, s_center, s_radius);
            return first_harmonic_roots(
                &FirstHarmonic {
                    lo: h.lo,
                    hi: h.hi,
                    cos_part: h.e_u,
                    sin_part: h.e_v,
                    lo_noise: h.lo_error + h.frame_error,
                    hi_noise: h.hi_error + h.frame_error,
                    phase_noise: h.phase_error,
                },
                conic.speed_hi(),
                t0,
                t1,
                &first_rows(BooleanDecision::ArcSphereRoots),
                band,
            );
        }
        (_, &geom::Surface::Sphere { center, radius, .. }) => (
            geom_brep::conic_sphere_harmonics(&conic, center, radius),
            BooleanDecision::ArcSphereRoots,
        ),
        (
            _,
            &geom::Surface::Cylinder {
                origin,
                axis,
                radius,
                ..
            },
        ) => (
            geom_brep::conic_cylinder_harmonics(&conic, origin, axis, radius),
            BooleanDecision::ArcCylinderRoots,
        ),
        (
            _,
            &geom::Surface::Cone {
                apex,
                axis,
                half_angle,
                ..
            },
        ) => {
            return cone_roots(
                &conic,
                (apex, axis, half_angle),
                (t0, t1),
                surface,
                band,
            );
        }
        _ => return Err(desync()),
    };
    // The harmonics' rounding, in residual metres: the term bound is in
    // m², before the `2r` division. It is a charge on the residual's
    // evaluation (`geom_brep::HARMONIC_NOISE_ULPS`), so it covers the
    // arm's readings together — `c₀ ∓ A₁` and the `A₂` it charges.
    let noise = rounding_charge(h.terms) / h.per;
    if let Some(first) = first_harmonic_arm(&h, noise, band) {
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
        &TrigPoly::second(h.c0, h.c1, h.s1, h.c2, h.s2),
        |theta| geom_brep::implicit_residual(surface, conic.point(theta)),
        ladder_frame(&conic, (t0, t1), noise, h.floor),
        &ladder_rows(decision),
        None,
        band,
    )
}

/// The first-harmonic arm's reading of `h` (module docs, "Two arms"),
/// or `None` where the second harmonic takes the ladder.
fn first_harmonic_arm<T: Decide>(
    h: &geom_brep::ConicHarmonics<T>,
    noise: T,
    band: Band,
) -> Option<FirstHarmonic<T>> {
    let hypot = |x: T, y: T| (x.powi(2) + y.powi(2)).sqrt();
    let second = hypot(h.c2, h.s2);
    let Ok(Sign::Zero) = decide(SECOND_HARMONIC, Margin::of(second), band) else {
        return None;
    };
    let (a1, noise) = (hypot(h.c1, h.s1), noise + second);
    Some(FirstHarmonic {
        lo: h.c0 - a1,
        hi: h.c0 + a1,
        cos_part: h.c1,
        sin_part: h.s1,
        lo_noise: noise,
        hi_noise: noise,
        phase_noise: T::zero(),
    })
}

/// The ladder's frame for a conic: `F`'s ceiling on `|F|` per metre of
/// residual is `1` on every kind ([`geom_brep::ConicHarmonics::floor`]),
/// its floor the door's `floor`.
fn ladder_frame<T: Decide>(
    conic: &geom_brep::Conic<T>,
    (t0, t1): (T, T),
    noise: T,
    floor: T,
) -> HalfAngleFrame<T> {
    HalfAngleFrame {
        walk: SubdivisionFrame {
            t0,
            t1,
            speed_hi: conic.speed_hi(),
            noise,
            f_per_metre: floor,
            f_per_metre_hi: T::one(),
            residual_reach: None,
        },
        speed_lo: conic.speed_lo(),
        lever: T::from_f64(2.0) * conic.speed_lo(),
    }
}

/// The cone arm's rows: the root slack, the apex, and the on-surface
/// reading of a constant `F` (module docs, "Against a cone").
const CONE_ROOT_SLACK: &str = "bool_conic_cone_root_slack";
const CONE_APEX: &str = "bool_conic_cone_apex";
const CONE_ON_SURFACE: &str = "bool_conic_cone_on_surface";

/// The conic × quadric door against a CONE (module docs, "Against a
/// cone"): the wall's two arms on `F = Q/R`, every root's slack metered
/// and its distance from the apex decided, and an on-surface answer read
/// again through the floor.
fn cone_roots<T: Decide>(
    conic: &geom_brep::Conic<T>,
    (apex, axis, half_angle): (geom_core::Point3<T>, geom_core::Vec3<T>, T),
    (t0, t1): (T, T),
    surface: &geom::Surface<T>,
    band: Band,
) -> Result<CircleRoots<T>, BooleanError> {
    let h = geom_brep::conic_cone_harmonics(conic, apex, axis, half_angle);
    let noise = rounding_charge(h.terms) / h.per;
    let roots = if let Some(first) = first_harmonic_arm(&h, noise, band) {
        let roots = first_harmonic_roots(
            &first,
            conic.speed_hi(),
            t0,
            t1,
            &first_rows(BooleanDecision::ArcConeRoots),
            band,
        )?;
        if let CircleRoots::OnSurface = roots {
            // `|res| ≤ |F| / floor`: a constant `F` in the band certifies
            // the carrier on the cone only once its whole reach, read
            // through the floor, is in the band too.
            let reach = (first.lo.abs().max(first.hi.abs()) + first.lo_noise) / h.floor;
            return Ok(match decide(CONE_ON_SURFACE, Margin::of(reach), band) {
                Ok(Sign::Zero) => CircleRoots::OnSurface,
                Ok(Sign::Positive | Sign::Negative) | Err(_) => CircleRoots::Uncertain,
            });
        }
        roots
    } else {
        let placed = |theta: T| geom_brep::conic_cone_residual(conic, apex, axis, half_angle, theta);
        let meter = RootSlack {
            row: CONE_ROOT_SLACK,
            residual: &placed,
            f_per_metre_hi: T::one(),
        };
        half_angle_roots(
            &TrigPoly::second(h.c0, h.c1, h.s1, h.c2, h.s2),
            |theta| geom_brep::implicit_residual(surface, conic.point(theta)),
            ladder_frame(conic, (t0, t1), noise, h.floor),
            &ladder_rows(BooleanDecision::ArcConeRoots),
            Some(&meter),
            band,
        )?
    };
    let CircleRoots::Certified { count, thetas } = roots else {
        return Ok(roots);
    };
    for &theta in &thetas[..count] {
        match decide(CONE_APEX, Margin::norm3(conic.point(theta) - apex), band) {
            Ok(Sign::Positive) => {}
            Ok(Sign::Zero | Sign::Negative) | Err(_) => return Ok(CircleRoots::AtApex),
        }
    }
    Ok(roots)
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod tests {
    use super::*;
    use geom_core::{Point3, Tol, Vec3};

    fn band() -> Band {
        Band::linear(Tol::witness()).unwrap()
    }

    /// **A desynced caller is a kernel defect, loudly.** The door is
    /// dispatched on a circle or an ellipse against a sphere or a wall;
    /// anything else reaching it is the caller's broken invariant, never
    /// an answer.
    #[test]
    fn a_wrong_kind_is_a_desync() {
        let (o, z, x) = (
            Point3::origin(),
            Vec3::new(0.0, 0.0, 1.0),
            Vec3::new(1.0, 0.0, 0.0),
        );
        let line = geom::Curve3::Line { origin: o, dir: x };
        let circle = geom::Curve3::Circle {
            center: o,
            axis: z,
            radius: 1.0,
            u_ref: x,
        };
        let ellipse = geom::Curve3::Ellipse {
            center: o,
            axis: z,
            major: 1.0,
            minor: 0.5,
            u_ref: x,
        };
        let sphere = geom::Surface::Sphere {
            center: o,
            radius: 1.0,
            axis: z,
            u_ref: x,
        };
        let wall = geom::Surface::Cylinder {
            origin: o,
            axis: z,
            radius: 1.0,
            u_ref: x,
        };
        let plane = geom::Surface::Plane {
            origin: o,
            normal: z,
            u_ref: x,
        };
        let torus = geom::Surface::Torus {
            center: o,
            axis: z,
            major_radius: 1.0,
            minor_radius: 0.25,
            u_ref: x,
        };
        for (carrier, surface) in [
            (&line, &sphere),
            (&line, &wall),
            (&circle, &plane),
            (&ellipse, &plane),
            (&circle, &torus),
            (&ellipse, &torus),
        ] {
            let got = conic_quadric_roots(carrier, 0.0, 1.0, surface, band());
            assert!(
                matches!(got, Err(BooleanError::ClassificationInvariant { .. })),
                "{carrier:?} against {surface:?} refuses as a kernel invariant, got {got:?}"
            );
        }
    }
}
