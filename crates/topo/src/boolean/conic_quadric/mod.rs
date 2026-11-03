//! **The conic × quadric root door**: the certified crossings of a
//! CIRCLE or ELLIPSE carrier with a sphere, a cylinder wall or a cone. It
//! reads the residual's harmonics from their one home and hands them to
//! one of the shared root cores ([`super::circle_roots`]), whose answer
//! it gives. Its one root reading of its own is a cone's carrier through
//! the apex ("Through the apex" below), which the cores cannot certify
//! and which it reads in closed form before them.
//!
//! # The residual is a degree-2 trigonometric polynomial
//!
//! With `C(θ) = C₀ + a·û cos θ + b·v̂ sin θ` (`θ` the eccentric anomaly,
//! a circle's angle when `a = b = ρ`) and either surface written as the
//! quadric `(|⊥(p − o)|² − r²)/2r` (`⊥` the identity on a sphere, the
//! projection off the axis on a wall), `⊥(C(θ) − o)` is a first harmonic
//! in `θ`, so the linearized residual is EXACTLY
//! `c₀ + c₁ cos θ + s₁ sin θ + c₂ cos 2θ + s₂ sin 2θ` metres
//! (`geom_brep::ConicHarmonics`). On a sphere or a wall the noise
//! meter's floor on `|F|` per metre of residual is therefore `1`, an
//! identity rather than a neighbourhood bound; a cone's `F` is in its
//! own units, and its floor is read from the geometry ("Against a cone"
//! below). There are at most four crossings per turn.
//! An in-band sign escalates as the surface's arc decision
//! ([`BooleanDecision::ArcSphereRoots`],
//! [`BooleanDecision::ArcCylinderRoots`]).
//!
//! # Two arms, by the second harmonic
//!
//! `bool_conic_quadric_second_harmonic` decides `A₂ = |(c₂, s₂)|`, the
//! second harmonic's amplitude in `F`'s units: metres on a sphere or a
//! wall, where `F` is the residual, and `Q/R` on a cone.
//!
//! - **The first-harmonic arm — `A₂` in the zero band**: the residual is
//!   a first harmonic to within `A₂`, which is charged to both extremes'
//!   noise, and the shared first-harmonic door decides it on its exact
//!   extremes under the `bool_conic_quadric_first_*` rows. The arm is
//!   REQUIRED, for two poses the ladder cannot answer truthfully: a
//!   CONSTANT residual (`F·(1 + t²)²`, which the ladder cannot read; the
//!   extremes answer on-surface or miss), and a TANGENCY, whose extreme
//!   reads in the zero band and refuses `Unsettled::Tangency` (escalating only where
//!   an end of that extreme's error interval lies in the gap), where the
//!   ladder can certify a miss
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
//!   diameter apart whatever `r` is). Each root the subdivision certifies
//!   carries its slack (`bool_conic_quadric_sub_root_slack`): the
//!   residual at the root with its running bound
//!   ([`geom_brep::conic_quadric_residual`]), over `F`'s least slope near
//!   it, at the top speed. At a shallow crossing that slope is small, and
//!   the `f64` residual's sign change sits that reading's rounding over
//!   it from the true root; a root whose slack the band cannot hold is
//!   refused.
//!
//! The two arms' meters keep their cores' postures, which differ on a
//! reading in the band's gap: the first-harmonic arm's refuse it, the
//! ladder's pass it (the circle root cores' module docs, "The ladder's
//! noise meter") — which is why their rows have distinct names.
//!
//! # Against a cone
//!
//! A cone's residual `ρ cos α − |h| sin α` has no harmonic form, but its
//! quadric form `Q = cos²α·ρ² − sin²α·h²` does: `ρ²` is a wall's form and
//! `h` a first harmonic, so `Q` is of degree two along a conic, and its
//! zero set is the DOUBLE cone the residual states. The door reads
//! `F = Q/R`, `R` the carrier's reach from the apex
//! ([`geom_brep::conic_cone_harmonics`]), by the wall's two arms under
//! the same rows and the [`BooleanDecision::ArcConeRoots`] decision.
//! `F` is not the residual, and every reading takes that into account:
//!
//! - **`|F| ≤ |res|`, with the same sign** (on the near nappe
//!   `Q = res·(ρ cos α + |h| sin α)`, and that factor is at most `|q|`).
//!   So a definite sign of `F` is the residual's, a clear margin read
//!   through the ceiling `1` is a lower bound on `|res|`, and a root's
//!   slack is charged its own residual reading
//!   ([`geom_brep::conic_cone_residual`]) under
//!   `bool_conic_cone_root_slack`. Near the apex `F′` vanishes with the
//!   gradient, and the slack with it refuses.
//! - **`|res| ≤ |F| / floor`**, the floor `min(sin α, cos α)·d/R`, `d`
//!   a lower bound on the carrier's distance from the apex. An
//!   `Roots::On` from the first-harmonic arm certifies only that `F` is
//!   in the band; it stands once its whole reach read through the floor
//!   is too (`bool_conic_cone_on_surface`), and refuses `Unsettled::Meter`
//!   otherwise.
//! - **Every root the arms certify, its distance from the apex** is
//!   decided (`bool_conic_cone_apex`): not definitely positive, and the
//!   door refuses [`Unsettled::AtApex`] — the arms read no slope at a
//!   point where the cone has no tangent plane. A carrier through the
//!   apex never reaches them: it is read whole first ("Through the apex"
//!   below), so this rung refuses only a carrier the apex is decided OFF
//!   whose arms still isolate a root beside it.
//!
//! The roots are the double cone's. Which of them a cone FACE holds — the
//! nappe it lies on, then its trim — is the caller's question.
//!
//! # Through the apex
//!
//! A carrier THROUGH the apex is read before the door's arms, whole
//! (`through_the_apex`), and its apex root is answered, not refused. The apex
//! is always a double root of `F`, because the cone's gradient vanishes
//! there. That is why the arms cannot certify it. About it, `φ = θ − θ_a`,
//! the form is exactly `F = −2 sin²(φ/2)·G(φ) + ε₀ + ε₁ sin φ`, with
//! `G = k + R cos(φ − β)` a first harmonic and `ε₀ = F(θ_a)`,
//! `ε₁ = F′(θ_a)`. Each reading is its own row:
//!
//! - **the apex on the carrier** (`bool_conic_cone_apex`), read at the
//!   anomaly of the apex's own in-plane coordinates (`Conic::angle_of`):
//!   on a circle the carrier's nearest point to the apex's projection,
//!   on an ellipse in general not, so the distance read is at least the
//!   carrier's. Conservative both ways: a carrier read clear of the apex
//!   goes to the arms, whose apex rung refuses a root beside it, and the
//!   `δ` the spread is bounded by is that reading's, never smaller than
//!   the true distance. Not decided on it: the arms read the carrier;
//! - **the apex's spread** (`bool_conic_cone_apex_spread`), a bound:
//!   on a window `|φ| ≤ Φ` about the apex, `|G| ≥ |G(0)| − RΦ` and
//!   `2 sin²(φ/2) ≥ (1 − Φ²/12)φ²/2`, so with `λ` their product the roots
//!   there lie within `(|ε₁| + √(ε₁² + 2λ|ε₀|))/λ` of `θ_a`. `Φ` is twice
//!   what `G(0)` alone gives, and a bound past it, or a `λ` not positive,
//!   refuses. The `ε`s are bounded by the geometry, `δ²/R` and
//!   `2δ·speed/R` at that point's distance `δ` from the apex, not by the
//!   rounded coefficients, whose noise the root would amplify. Past the
//!   band: [`Unsettled::AtApex`];
//! - **`G`'s roots** (`bool_conic_cone_deflated_depth`, `|k| − R`):
//!   definitely positive, none, so the apex's double root is the
//!   carrier's only one. In the band, a tangency. Negative: two, in
//!   closed form, each clear of the apex's spread and of each other
//!   (`bool_conic_cone_deflated_clear`), bracketed by a definite change
//!   of the cone's residual (`bool_conic_cone_deflated_side`), and metered
//!   (`bool_conic_cone_root_slack`): `|F|` at the root over a lower bound
//!   on `|F′|` across the whole bracket, `|F′(θ)| − 2·noise − w(ρ₁ + 4ρ₂)` with
//!   `ρₖ` the harmonics' amplitudes, at the carrier's top speed. The
//!   arms' meter (geom_brep's `RootSlack`) decides the same arc under the
//!   same row, its slope read on a window about the root inside the
//!   subdivision; this one reads it on its own bracket, which needs no
//!   subdivision.
//!
//! Outside the spread's window `F`'s roots are `G`'s: `F` has at most four
//! per turn, two of them the apex's.
//!
//! The apex's double root is answered with the others. The crossing
//! layer places it on the face's apex vertex, the carrier's incidence
//! with it, which the vertex pair reads.

use geom_core::{Band, Decide, Margin, Sign};

use super::circle_roots::{
    CircleRoots, FirstHarmonic, FirstHarmonicRows, door_half_angle_roots, first_harmonic_roots,
};
use super::{BooleanDecision, BooleanError};
use crate::validate::decide;
use geom_brep::half_angle::{
    DegreeTwo, HalfAngleFrame, HalfAngleRows, RootSlack, SubdivisionFrame, SubdivisionRows,
};
use geom_brep::quartic::QuarticRows;
use geom_brep::roots::{OnCarrier, Roots, Unsettled};
use geom_core::rounding_charge;

#[cfg(test)]
mod circle_sphere_rows;
#[cfg(test)]
mod circle_wall_rows;
#[cfg(test)]
mod cone_rows;
#[cfg(test)]
mod ellipse_rows;
#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod in_band_sweep;
#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod shallow_sweep;

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

/// The ladder's noise meter's row (module docs, "Two arms").
const LADDER_NOISE: &str = "bool_conic_quadric_ladder_noise";

/// The ladder's rows.
const LADDER_ROWS: HalfAngleRows = HalfAngleRows {
    pole: "bool_conic_quadric_pole",
    conditioning: "bool_conic_quadric_pole_conditioning",
    noise: LADDER_NOISE,
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
};

/// The certified crossings of the `carrier` circle or ellipse with
/// `surface`, a sphere, a cylinder wall or a cone, reported within `π`
/// of the midpoint of `[t0, t1]` (module docs).
///
/// # Errors
///
/// [`BooleanError::ClassificationInvariant`] when `carrier` is neither a
/// circle nor an ellipse or `surface` not a sphere, a cylinder or a cone —
/// the caller dispatched on those kinds, so a mismatch is a desync,
/// never an answer. An escalation as the surface's arc decision for an
/// in-band classifying sign: an extreme or constant residual of the
/// first-harmonic arm, or a rung of the ladder. A second harmonic in the
/// band's GAP is not an error: it takes the ladder. A count the door
/// cannot certify is the inner [`Unsettled`].
pub(super) fn conic_quadric_roots<T: Decide>(
    carrier: &geom::Curve3<T>,
    t0: T,
    t1: T,
    surface: &geom::Surface<T>,
    band: Band,
) -> Result<Result<CircleRoots<T>, Unsettled>, BooleanError> {
    let desync = || BooleanError::ClassificationInvariant {
        what: "the conic × quadric root door was handed a carrier that is not a circle or an \
               ellipse or a surface that is not a sphere, a cylinder or a cone",
    };
    let conic = carrier.conic().ok_or_else(desync)?;
    match (carrier, surface) {
        (
            &geom::Curve3::Circle(geom::Circle {
                frame:
                    geom::Frame {
                        origin: center,
                        axis,
                        u_ref,
                    },
                radius,
            }),
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
            )
            .map(|found| found.map(Roots::widen));
        }
        (
            _,
            &geom::Surface::Cone {
                apex,
                axis,
                half_angle,
                ..
            },
        ) => {
            return cone_roots(&conic, (apex, axis, half_angle), (t0, t1), surface, band);
        }
        _ => {}
    }
    let QuadricLadder {
        h,
        decision,
        quadric,
        frame,
    } = quadric_ladder(&conic, surface, (t0, t1)).ok_or_else(desync)?;
    if let Some(first) = first_harmonic_arm(&h, frame.walk.noise, band) {
        return first_harmonic_roots(
            &first,
            conic.speed_hi(),
            t0,
            t1,
            &first_rows(decision),
            band,
        )
        .map(|found| found.map(Roots::widen));
    }
    door_half_angle_roots(
        &DegreeTwo::new(h.c0, h.c1, h.s1, h.c2, h.s2),
        |theta| geom_brep::implicit_residual(surface, conic.point_at(theta)),
        frame,
        &LADDER_ROWS,
        &RootSlack {
            row: LADDER_ROOT_SLACK,
            residual: &|theta| geom_brep::conic_quadric_residual(&conic, quadric, theta),
            f_per_metre_hi: T::one(),
        },
        decision,
        band,
    )
}

/// A conic against a sphere or a wall, read for the door's two arms: its
/// harmonics, the surface's decision, the quadric as
/// [`geom_brep::conic_quadric_residual`] reads it, and the ladder's frame,
/// which carries the harmonics' noise.
struct QuadricLadder<'a, T: Decide> {
    h: geom_brep::ConicHarmonics<T>,
    decision: BooleanDecision,
    quadric: (geom_core::Point3<T>, Option<geom_core::Vec3<T>>, T),
    frame: HalfAngleFrame<'a, T>,
}

/// [`QuadricLadder`] for `conic` against `surface` over `[t0, t1]`, or
/// `None` when `surface` is neither a sphere nor a cylinder.
fn quadric_ladder<T: Decide>(
    conic: &dyn geom::Conic<T>,
    surface: &geom::Surface<T>,
    (t0, t1): (T, T),
) -> Option<QuadricLadder<'static, T>> {
    let (h, decision, quadric) = match *surface {
        geom::Surface::Sphere { center, radius, .. } => (
            geom_brep::conic_sphere_harmonics(conic, center, radius),
            BooleanDecision::ArcSphereRoots,
            (center, None, radius),
        ),
        geom::Surface::Cylinder {
            origin,
            axis,
            radius,
            ..
        } => (
            geom_brep::conic_cylinder_harmonics(conic, origin, axis, radius),
            BooleanDecision::ArcCylinderRoots,
            (origin, Some(axis), radius),
        ),
        _ => return None,
    };
    // The harmonics' rounding, in residual metres: the term bound is in
    // m², before the `2r` division. It is a charge on the residual's
    // evaluation (`geom_core::SHORT_CHAIN_ULPS`), so it covers the
    // arm's readings together — `c₀ ∓ A₁` and the `A₂` it charges.
    let noise = rounding_charge(h.terms) / h.per;
    let frame = ladder_frame(conic, (t0, t1), noise, h.floor);
    Some(QuadricLadder {
        h,
        decision,
        quadric,
        frame,
    })
}

/// The ladder's root-slack row on a sphere or a wall (module docs, "Two
/// arms"): `F` is the residual, so its ceiling per metre is `1`.
const LADDER_ROOT_SLACK: &str = "bool_conic_quadric_sub_root_slack";

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
    conic: &dyn geom::Conic<T>,
    (t0, t1): (T, T),
    noise: T,
    floor: T,
) -> HalfAngleFrame<'static, T> {
    HalfAngleFrame {
        walk: SubdivisionFrame {
            t0,
            t1,
            speed_hi: conic.speed_hi(),
            noise,
            f_per_metre: floor,
            f_per_metre_hi: T::one(),
            residual_reach: None,
            jet: None,
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
    conic: &dyn geom::Conic<T>,
    (apex, axis, half_angle): (geom_core::Point3<T>, geom_core::Vec3<T>, T),
    (t0, t1): (T, T),
    surface: &geom::Surface<T>,
    band: Band,
) -> Result<Result<CircleRoots<T>, Unsettled>, BooleanError> {
    let h = geom_brep::conic_cone_harmonics(conic, apex, axis, half_angle);
    let noise = rounding_charge(h.terms) / h.per;
    if let ThroughTheApex::Read(found) =
        through_the_apex(conic, (apex, axis, half_angle), &h, noise, (t0, t1), band)
    {
        return Ok(found);
    }
    let found = if let Some(first) = first_harmonic_arm(&h, noise, band) {
        let found = first_harmonic_roots(
            &first,
            conic.speed_hi(),
            t0,
            t1,
            &first_rows(BooleanDecision::ArcConeRoots),
            band,
        )?
        .map(Roots::widen);
        if let Ok(Roots::On(OnCarrier)) = found {
            // `|res| ≤ |F| / floor`: a constant `F` in the band certifies
            // the carrier on the cone only once its whole reach, read
            // through the floor, is in the band too.
            let reach = (first.lo.abs().max(first.hi.abs()) + first.lo_noise) / h.floor;
            return Ok(match decide(CONE_ON_SURFACE, Margin::of(reach), band) {
                Ok(Sign::Zero) => Ok(Roots::On(OnCarrier)),
                Ok(Sign::Positive | Sign::Negative) | Err(_) => Err(Unsettled::Meter),
            });
        }
        found
    } else {
        let placed =
            |theta: T| geom_brep::conic_cone_residual(conic, apex, axis, half_angle, theta);
        let meter = RootSlack {
            row: CONE_ROOT_SLACK,
            residual: &placed,
            f_per_metre_hi: T::one(),
        };
        door_half_angle_roots(
            &DegreeTwo::new(h.c0, h.c1, h.s1, h.c2, h.s2),
            |theta| geom_brep::implicit_residual(surface, conic.point_at(theta)),
            ladder_frame(conic, (t0, t1), noise, h.floor),
            &LADDER_ROWS,
            &meter,
            BooleanDecision::ArcConeRoots,
            band,
        )?
    };
    Ok(found.and_then(|roots| off_the_apex(conic, apex, roots, band)))
}

/// The rows of a conic through the apex (module docs, "Through the
/// apex").
const CONE_APEX_SPREAD: &str = "bool_conic_cone_apex_spread";
const CONE_DEFLATED_DEPTH: &str = "bool_conic_cone_deflated_depth";
const CONE_DEFLATED_SIDE: &str = "bool_conic_cone_deflated_side";
const CONE_DEFLATED_CLEAR: &str = "bool_conic_cone_deflated_clear";

/// What [`through_the_apex`] reads of a conic against the cone.
enum ThroughTheApex<T: geom_core::Real> {
    /// The apex is definitely off the carrier where it is read, at the
    /// apex's own anomaly (on an ellipse not the nearest point, a
    /// distance at least the carrier's): the door's two arms read it, and
    /// their apex rung refuses a root beside the apex.
    Clear,
    /// The carrier passes through the apex, read whole: the apex's
    /// double root and the deflated form's roots, each certified, or a
    /// refusal.
    Read(Result<CircleRoots<T>, Unsettled>),
}

/// **A conic through the cone's apex** (module docs, "Through the
/// apex"): [`ThroughTheApex::Clear`] where the apex is definitely off
/// the carrier; otherwise the carrier read whole. It reads the whole
/// turn, not the caller's arc, as the door's arms do: a refusal stands
/// even where the arc holds neither the apex nor the root it could not
/// read.
fn through_the_apex<T: Decide>(
    conic: &dyn geom::Conic<T>,
    (apex, axis, half_angle): (geom_core::Point3<T>, geom_core::Vec3<T>, T),
    h: &geom_brep::ConicHarmonics<T>,
    noise: T,
    (t0, t1): (T, T),
    band: Band,
) -> ThroughTheApex<T> {
    use ThroughTheApex::{Clear, Read};
    let two = T::from_f64(2.0);
    let mid = (t0 + t1) / two;
    let near_mid = |theta: T| geom_brep::half_angle::near_midpoint(mid, theta);
    // A parameter difference as a turn in `(−π, π]`, the same rule.
    let wrap = |x: T| geom_brep::half_angle::near_midpoint(T::zero(), x);
    // Past every bound: a reading no bound holds refuses.
    let unbounded = T::from_f64(f64::INFINITY);
    // The apex's parameter on the conic, its projection's off the plane.
    let theta_a = near_mid(conic.angle_of(apex));
    let off_apex = conic.point_at(theta_a) - apex;
    match decide(CONE_APEX, Margin::norm3(off_apex), band) {
        Ok(Sign::Positive) => return Clear,
        Ok(Sign::Zero | Sign::Negative) => {}
        Err(_) => return Read(Err(Unsettled::AtApex)),
    }
    let (sa, ca) = theta_a.sin_cos();
    let (s2a, c2a) = (two * theta_a).sin_cos();
    let c1 = h.c1 * ca + h.s1 * sa;
    let (c2, s2) = (h.c2 * c2a + h.s2 * s2a, h.s2 * c2a - h.c2 * s2a);
    let k = c1 + two * c2;
    let (rc, rs) = (two * c2, two * s2);
    let r = (rc.powi(2) + rs.powi(2)).sqrt();
    let g0 = k + rc;
    // The apex's spread (module docs). About the apex
    // `F = −2 sin²(φ/2)·G(φ) + ε₀ + ε₁ sin φ`; the exact `ε₀ = Q(q)/per`
    // and `ε₁ = ∇Q(q)·C′/per` at `q = C(θ_a) − apex`, `δ = |q|`, are read
    // off the geometry, not off the rounded coefficients (whose noise a
    // square root would amplify past any band): `|Q| ≤ |q|²` and
    // `|∇Q| ≤ 2|q|`, the form's weights at most one, so `|ε₀| ≤ δ²/per`
    // and `|ε₁| ≤ 2δ·speed/per`. `δ` carries the point's own rounding.
    let delta = off_apex.norm()
        + geom_core::running::direction_rounding(
            (conic.frame().center - apex).norm() + conic.speed_hi(),
        );
    let (e0, e1) = (
        delta.powi(2) / h.per,
        two * delta * conic.speed_hi() / h.per,
    );
    // A bound, not a model ([`apex_spread_bound`]).
    let spread_phi = apex_spread_bound((e0, e1), g0.abs(), r);
    let spread = spread_phi * conic.speed_hi();
    match decide(CONE_APEX_SPREAD, Margin::of(spread), band) {
        Ok(Sign::Zero) => {}
        Ok(Sign::Positive | Sign::Negative) | Err(_) => return Read(Err(Unsettled::AtApex)),
    }
    // `G`'s own roots: none where `|k| − R` is definite, two where it is
    // definitely negative, a tangency between.
    let depth = k.abs() - r;
    match decide(CONE_DEFLATED_DEPTH, Margin::of(depth - noise), band) {
        Ok(Sign::Positive) => {
            return Read(Ok(Roots::<T, 2>::all([theta_a, theta_a]).widen()));
        }
        Ok(Sign::Negative) => {}
        Ok(Sign::Zero) | Err(_) => return Read(Err(Unsettled::Tangency)),
    }
    let beta = rs.atan2(rc);
    let spin = (T::zero() - k / r).max(-T::one()).min(T::one()).acos();
    // Each root is bracketed by a definite change of the cone's residual
    // over `±w` about it (`w` sixty-four escalation bands of arc), the one
    // root of `F` there by the deflation's count, and metered: the true
    // root lies in the bracket, where `|F′|` is at least its value at the
    // root, less its rounding (`2·noise`, the derivative's coefficients
    // at most twice `F`'s) and `w` times `F″`'s bound (`|F″| ≤ ρ₁ + 4ρ₂`,
    // the harmonics' amplitudes), so it lies within `(|F| + noise)/that`
    // of the root,
    // at the carrier's top speed an arc that must be definitely in the
    // band (`bool_conic_cone_root_slack`, the arms' quantity with a
    // whole-bracket slope bound in place of their window's). A shallow
    // crossing reads neither, and refuses as the tangency it is near.
    let width = T::from_f64(64.0) * T::from_f64(band.escalate()) / conic.speed_lo();
    let residual = |theta: T| geom_brep::conic_cone_residual(conic, apex, axis, half_angle, theta);
    let f = |theta: T| {
        let (s, c) = theta.sin_cos();
        let (s2, c2) = (two * theta).sin_cos();
        h.c0 + h.c1 * c + h.s1 * s + h.c2 * c2 + h.s2 * s2
    };
    let slope = |theta: T| {
        let (s, c) = theta.sin_cos();
        let (s2, c2) = (two * theta).sin_cos();
        h.s1 * c - h.c1 * s + two * (h.s2 * c2 - h.c2 * s2)
    };
    let bend = (h.c1.powi(2) + h.s1.powi(2)).sqrt()
        + T::from_f64(4.0) * (h.c2.powi(2) + h.s2.powi(2)).sqrt();
    let side = |res: geom_core::Rounded<T>| {
        let clear = decide(
            CONE_DEFLATED_SIDE,
            Margin::of(res.value.abs() - res.error),
            band,
        );
        match (
            decide(CONE_DEFLATED_SIDE, Margin::of(res.value), band),
            clear,
        ) {
            (Ok(sign @ (Sign::Positive | Sign::Negative)), Ok(Sign::Positive)) => Some(sign),
            _ => None,
        }
    };
    let mut found = [theta_a; 4];
    for (slot, phi) in [beta + spin, beta - spin].into_iter().enumerate() {
        let theta = near_mid(theta_a + phi);
        let off = wrap(theta - theta_a).abs() - width - spread_phi;
        match decide(
            CONE_DEFLATED_CLEAR,
            Margin::of(off * conic.speed_lo()),
            band,
        ) {
            Ok(Sign::Positive) => {}
            Ok(Sign::Zero | Sign::Negative) | Err(_) => return Read(Err(Unsettled::AtApex)),
        }
        match (side(residual(theta - width)), side(residual(theta + width))) {
            (Some(a), Some(b)) if a != b => {}
            _ => return Read(Err(Unsettled::Tangency)),
        }
        let least = slope(theta).abs() - two * noise - width * bend;
        let slack = least.select_le_zero(
            unbounded,
            conic.speed_hi() * (f(theta).abs() + noise) / least,
        );
        match decide(CONE_ROOT_SLACK, Margin::of(slack), band) {
            Ok(Sign::Zero) => found[2 + slot] = theta,
            Ok(Sign::Positive | Sign::Negative) | Err(_) => return Read(Err(Unsettled::Meter)),
        }
    }
    let apart = wrap(found[2] - found[3]).abs() - two * width;
    match decide(
        CONE_DEFLATED_CLEAR,
        Margin::of(apart * conic.speed_lo()),
        band,
    ) {
        Ok(Sign::Positive) => {}
        Ok(Sign::Zero | Sign::Negative) | Err(_) => return Read(Err(Unsettled::Tangency)),
    }
    Read(Ok(Roots::<T, 4>::all(found).widen()))
}

/// **The apex's spread** (module docs, "Through the apex"), in radians
/// of the carrier's parameter: how far from the apex's own parameter a
/// root of `F` can lie, given `|ε₀|`, `|ε₁|` bounded by the geometry,
/// `|G(0)|` and `G`'s amplitude `R`. On the window `|φ| ≤ Φ`,
/// `|G| ≥ |G(0)| − RΦ` (`|G′| ≤ R`) and `2 sin²(φ/2) ≥ (1 − Φ²/12)·φ²/2`,
/// so a root there has `λφ²/2 ≤ ε₀ + ε₁|φ|`, `λ` their product, and lies
/// within `(ε₁ + √(ε₁² + 2λε₀))/λ`. The window is twice what `G(0)` alone
/// gives. Each factor of `λ` must be positive on its own (two negative
/// factors make a positive product that bounds nothing), and the bound
/// must lie inside the window: otherwise it holds nothing and reads
/// infinite, which refuses. Outside the window `F`'s roots are `G`'s,
/// bracketed by the caller: `F` has at most four per turn, two of them
/// the apex's.
fn apex_spread_bound<T: Decide>((e0, e1): (T, T), g0: T, r: T) -> T {
    let two = T::from_f64(2.0);
    // Past every bound: a reading no bound holds refuses.
    let unbounded = T::from_f64(f64::INFINITY);
    let at_zero = (e1 + (e1.powi(2) + two * g0 * e0).sqrt()) / g0;
    let window = two * at_zero;
    let least_g = g0 - r * window;
    let least_sine = T::one() - window.powi(2) / T::from_f64(12.0);
    let lambda = least_g.max(T::zero()) * least_sine.max(T::zero());
    let bound = (e1 + (e1.powi(2) + two * lambda * e0).sqrt()) / lambda;
    let within = (bound - window).select_le_zero(bound, unbounded);
    least_g.select_le_zero(unbounded, least_sine.select_le_zero(unbounded, within))
}

/// The apex rung (module docs, "Against a cone"): `roots` stand only if
/// every certified root is definitely off the apex, and refuse
/// [`Unsettled::AtApex`] otherwise.
///
/// Through the door it is a second line: a root the subdivision isolates
/// within the escalation band of the apex is refused first by the slack
/// meter, whose lever there is `F`'s slope, at most `2|q|` per unit of
/// speed over `R` — so the rung decides only a root that meter let
/// through, and its row hands it one directly.
fn off_the_apex<T: Decide>(
    conic: &dyn geom::Conic<T>,
    apex: geom_core::Point3<T>,
    roots: CircleRoots<T>,
    band: Band,
) -> Result<CircleRoots<T>, Unsettled> {
    let CircleRoots::Certified(found) = roots else {
        return Ok(roots);
    };
    for &theta in found.roots() {
        match decide(CONE_APEX, Margin::norm3(conic.point_at(theta) - apex), band) {
            Ok(Sign::Positive) => {}
            Ok(Sign::Zero | Sign::Negative) | Err(_) => return Err(Unsettled::AtApex),
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
        let line = geom::Curve3::Line(geom::Line { origin: o, dir: x });
        let circle = geom::Curve3::circle(o, z, 1.0, x);
        let ellipse =
            geom::Curve3::Ellipse(geom::Ellipse::new(geom::Frame::new(o, z, x), 1.0, 0.5));
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
