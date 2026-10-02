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
//!   which the ladder's frame carries as such; its lever is the
//!   carrier's own `2a` — not clamped by the sphere's size, as the torus
//!   door clamps by its extent, because the roots' variable is arc
//!   length along the carrier, and two crossings a chord `2r` apart can
//!   lie farther apart than that along it.

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
            lever: two * conic.major,
            noise,
            f_per_metre: T::one(),
        },
        &ladder_rows(decision),
        band,
    )
}
