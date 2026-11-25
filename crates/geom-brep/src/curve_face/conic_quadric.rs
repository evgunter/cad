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
//! (`crate::ConicHarmonics`). On a sphere or a wall the noise
//! meter's floor on `|F|` per metre of residual is therefore `1`, an
//! identity rather than a neighbourhood bound; a cone's `F` is in its
//! own units, and its floor is read from the geometry ("Against a cone"
//! below). There are at most four crossings per turn.
//! An in-band sign refuses as the surface's arm of
//! [`ConicQuadricRefusal`] ([`ConicQuadricRefusal::Sphere`],
//! [`ConicQuadricRefusal::Cylinder`]).
//!
//! # Two arms, by the second harmonic
//!
//! `curve_face_pose_second_harmonic` decides `A₂ = |(c₂, s₂)|`, the
//! second harmonic's amplitude in `F`'s units: metres on a sphere or a
//! wall, where `F` is the residual, and `Q/R` on a cone.
//!
//! - **The first-harmonic arm — `A₂` in the zero band**: the residual is
//!   a first harmonic to within `A₂`, which is charged to both extremes'
//!   noise, and the pose decides it on its exact extremes (the
//!   `curve_face_pose_*` rows; the roots' slack is the method's
//!   `conic_quadric_first_root_slack`). The arm is
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
//!     (`crate::circle_sphere_harmonic`): the extremes
//!     `(D∓ − r)(D∓ + r)/2r`, `D∓` the distances from the sphere's centre
//!     to the circle's nearest and farthest points, each with a running
//!     bound on its rounding, so a shallow crossing is placed as well as
//!     its near extreme is evaluated rather than as well as the
//!     harmonics' m² terms are. A frame that is not orthonormal moves the
//!     circle off that form, and its defect is charged to both extremes
//!     (`crate::CircleSphereHarmonic::frame_error`). Every circle
//!     whose axis passes through the sphere's centre has a constant
//!     residual, and lies ON the sphere when it is zero.
//! - **The ladder — `A₂` definite, or in the band's gap**: the five
//!   harmonics go to the shared half-angle ladder under the
//!   `conic_quadric_*` ladder rows. The carrier's speed `|C′|` lies
//!   in `[b, a]`, which the ladder's frame carries as such, so its root
//!   variable is `τ = 2b·tan(φ/2)`; its lever is that variable's own
//!   scale `2b` (a circle's `2ρ`), and it is not clamped by the surface's
//!   size: the roots spread along the carrier, not across the surface (a
//!   circle in a plane through a wall's axis meets it at points a whole
//!   diameter apart whatever `r` is). Each root the subdivision certifies
//!   carries its slack (`conic_quadric_sub_root_slack`): the
//!   residual at the root with its running bound
//!   ([`crate::conic_quadric_residual`]), over `F`'s least slope near
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
//! ([`crate::conic_cone_harmonics`]), by the wall's two arms under
//! the same rows and the [`ConicQuadricRefusal::Cone`] arm.
//! `F` is not the residual, and every reading takes that into account:
//!
//! - **`|F| ≤ |res|`, with the same sign** (on the near nappe
//!   `Q = res·(ρ cos α + |h| sin α)`, and that factor is at most `|q|`).
//!   So a definite sign of `F` is the residual's, a clear margin read
//!   through the ceiling `1` is a lower bound on `|res|`, and a root's
//!   slack is charged its own residual reading
//!   ([`crate::conic_cone_residual`]) under
//!   `conic_quadric_cone_root_slack`. Near the apex `F′` vanishes with the
//!   gradient, and the slack with it refuses.
//! - **`|res| ≤ |F| / floor`**, the floor `min(sin α, cos α)·d/R`, `d`
//!   a lower bound on the carrier's distance from the apex. An
//!   `Roots::On` from the first-harmonic arm certifies only that `F` is
//!   in the band; it stands once its whole reach read through the floor
//!   is too (`curve_face_pose_cone_on_surface`), and refuses `Unsettled::Meter`
//!   otherwise; an extreme of `F` in the zero band is the residual's
//!   graze only once it reads in the zero band through the floor too
//!   (`curve_face_pose_cone_graze`), and is the extreme's zero reading
//!   rejected otherwise. No class is decided on `F` in its zero band
//!   alone.
//! - **Every root the arms certify, its distance from the apex** is
//!   decided (`conic_quadric_cone_apex`): not definitely positive, and the
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
//! - **the apex on the carrier** (`conic_quadric_cone_apex`), read at the
//!   anomaly of the apex's own in-plane coordinates (`Conic::azimuth`):
//!   on a circle the carrier's nearest point to the apex's projection,
//!   on an ellipse in general not, so the distance read is at least the
//!   carrier's. Conservative both ways: a carrier read clear of the apex
//!   goes to the arms, whose apex rung refuses a root beside it, and the
//!   `δ` the spread is bounded by is that reading's, never smaller than
//!   the true distance. Not decided on it: the arms read the carrier;
//! - **the apex's spread** (`curve_face_pose_conic_apex_spread`), a bound:
//!   on a window `|φ| ≤ Φ` about the apex, `|G| ≥ |G(0)| − RΦ` and
//!   `2 sin²(φ/2) ≥ (1 − Φ²/12)φ²/2`, so with `λ` their product the roots
//!   there lie within `(|ε₁| + √(ε₁² + 2λ|ε₀|))/λ` of `θ_a`. `Φ` is twice
//!   what `G(0)` alone gives, and a bound past it, or a `λ` not positive,
//!   refuses. The `ε`s are bounded by the geometry, `δ²/R` and
//!   `2δ·speed/R` at that point's distance `δ` from the apex, not by the
//!   rounded coefficients, whose noise the root would amplify. Past the
//!   band: [`Unsettled::AtApex`];
//! - **`G`'s roots** (`conic_quadric_cone_deflated_depth`, `|k| − R`):
//!   definitely positive, none, so the apex's double root is the
//!   carrier's only one. In the band, a tangency. Negative: two, in
//!   closed form, each clear of the apex's spread and of each other
//!   (`conic_quadric_cone_deflated_clear`), bracketed by a definite change
//!   of the cone's residual (`conic_quadric_cone_deflated_side`), and metered
//!   (`conic_quadric_cone_root_slack`): `|F|` at the root over a lower bound
//!   on `|F′|` across the whole bracket, `|F′(θ)| − 2·noise − w(ρ₁ + 4ρ₂)` with
//!   `ρₖ` the harmonics' amplitudes, at the carrier's top speed. The
//!   arms' meter (`RootSlack`) decides the same arc under the
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

use geom_core::{Band, Decide, Margin, Rounded, Sign};

use super::circle_roots::{CircleRoots, DoorRefusal};
use super::cross::Walked;
use crate::curve_pair::MeetFault;
use crate::half_angle::half_angle_roots_past;
use crate::half_angle::{DegreeTwo, HalfAngleFrame, HalfAngleRows, RootSlack, SubdivisionFrame};
#[cfg(test)]
use crate::roots::OnCarrier;
use crate::roots::{Roots, Unsettled};
use geom_core::k_stats::decide;
use geom_core::{Indeterminate, rounding_charge};

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

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod ladder_lane_fuzz;

/// The ladder's rows.
const LADDER_ROWS: HalfAngleRows = crate::rows::CONIC_QUADRIC_LADDER;

/// The refusal of a carrier or a surface of a kind the door does not
/// read.
pub(super) const DESYNC: &str = "the conic × quadric door was handed a carrier that is not a \
     circle or an ellipse or a surface that is not a sphere, a cylinder or a cone";

/// **Why [`conic_quadric_roots`] refused**: the surface's arm and the
/// classifying sign it read in the band's escalation gap, or a kernel
/// defect. The caller names the decision an arm's escalation is.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ConicQuadricRefusal {
    /// Against a sphere.
    Sphere(Indeterminate),
    /// Against a cylinder wall.
    Cylinder(Indeterminate),
    /// Against a cone.
    Cone(Indeterminate),
    /// A carrier or a surface of a kind the door does not read, or
    /// readings that contradict: a kernel defect.
    Invariant(&'static str),
}

impl ConicQuadricRefusal {
    /// The refusal without its arm, for a reader that names no decision
    /// per arm, in any door's refusal shape.
    #[must_use]
    pub const fn door_refusal<X>(self) -> DoorRefusal<X> {
        match self {
            Self::Sphere(diag) | Self::Cylinder(diag) | Self::Cone(diag) => {
                DoorRefusal::Escalated(diag)
            }
            Self::Invariant(what) => DoorRefusal::Invariant(what),
        }
    }
}

/// **The certified crossings of the `carrier` circle or ellipse with
/// `surface`**, a sphere, a cylinder wall or a cone, reported within `π`
/// of the midpoint of `[t0, t1]`, in the root doors' shape: the door
/// rows' reading, a test reader since the sweep reads [`super::cross()`]:
/// the pose ([`super::pose()`]), then the method
/// ([`conic_quadric_method`]), mapped back. `Lies` is
/// [`CircleRoots::On`], `Apart` [`CircleRoots::Miss`], a graze (its
/// curvature in band too) [`Unsettled::Tangency`], a carrier through a
/// cone's apex the apex's double root with the deflated form's roots, an
/// apex in band [`Unsettled::AtApex`], and a pose meter that refused
/// [`Unsettled::Meter`].
///
/// # Errors
///
/// [`ConicQuadricRefusal::Invariant`] for a carrier or a surface of
/// another kind; the surface's arm for an escalation of the pose or of
/// the method.
#[cfg(test)]
pub fn conic_quadric_roots<T: Decide>(
    carrier: &geom::Curve3<T>,
    t0: T,
    t1: T,
    surface: &geom::Surface<T>,
    band: Band,
) -> Result<Result<CircleRoots<T>, Unsettled>, ConicQuadricRefusal> {
    use super::pose::{Pose, PoseRefusal, PoseRung};
    let arm: fn(Indeterminate) -> ConicQuadricRefusal = match surface {
        geom::Surface::Sphere { .. } => ConicQuadricRefusal::Sphere,
        geom::Surface::Cylinder { .. } => ConicQuadricRefusal::Cylinder,
        geom::Surface::Cone { .. } => ConicQuadricRefusal::Cone,
        _ => return Err(ConicQuadricRefusal::Invariant(DESYNC)),
    };
    if carrier.conic().is_none() {
        return Err(ConicQuadricRefusal::Invariant(DESYNC));
    }
    let span = crate::curve_pair::Span::new(carrier, (t0, t1));
    let pose = match super::pose::pose(&span, surface, band).map_err(|u| u.why) {
        Ok(pose) => pose,
        Err(refusal) if let Some(why) = refusal.unsettled() => return Ok(Err(why)),
        // The apex, or its spread, in band is refused typed, never
        // escalated: the arms read no slope at a point with no tangent
        // plane.
        Err(
            PoseRefusal::InBand {
                rung: PoseRung::Apex | PoseRung::ApexSpread,
                ..
            }
            | PoseRefusal::Meter {
                rung: PoseRung::ApexSpread,
                ..
            },
        ) => return Ok(Err(Unsettled::AtApex)),
        Err(PoseRefusal::InBand { diag, .. }) => return Err(arm(diag)),
        Err(PoseRefusal::Meter { row, .. }) => return Ok(Err(Unsettled::Meter { row })),
        Err(PoseRefusal::Fault(what)) => return Err(ConicQuadricRefusal::Invariant(what)),
        Err(PoseRefusal::Enclosed | PoseRefusal::Unbuilt(_) | PoseRefusal::Torus(_)) => {
            return Err(ConicQuadricRefusal::Invariant(DESYNC));
        }
    };
    let refused = |refusal: DoorRefusal| match refusal {
        DoorRefusal::Escalated(diag) => arm(diag),
        DoorRefusal::Invariant(what) => ConicQuadricRefusal::Invariant(what),
    };
    let shaped = |walked: Result<super::cross::Walked<T>, &'static str>| {
        walked
            .map_err(DoorRefusal::Invariant)
            .and_then(|walked| super::circle_roots::door_shape(&span, &walked, band))
            .map_err(refused)
    };
    Ok(match pose {
        Pose::Lies => Ok(CircleRoots::On(OnCarrier)),
        Pose::Apart => Ok(CircleRoots::Miss),
        Pose::Tangent(_) => Err(Unsettled::Tangency),
        Pose::Through(apex) => {
            shaped(through_the_apex(&span, surface, apex, band))?.map(|found| {
                let mut all = vec![apex.at, apex.at];
                if let CircleRoots::Certified(found) = found {
                    all.extend_from_slice(found.roots());
                }
                // The apex's double root and the deflated form's at most
                // two: four, inside the eight a `CircleRoots` holds.
                CircleRoots::of(&all).unwrap_or_else(|| unreachable!("at most four roots"))
            })
        }
        Pose::Isolated(form) => shaped(conic_quadric_method(&span, surface, form.0, band))?,
    })
}

/// **The conic × quadric door's method**: the roots of an isolated
/// conic from what its pose computed. On the first-harmonic lane the
/// core's root slack and placement ([`super::circle_roots::harmonic_roots_as`]),
/// a slack the meter refuses a window over each root ± that slack widened
/// by `Kε`; on the ladder lane the subdivision read past its refusals
/// ([`crate::half_angle::half_angle_roots_past`]). On a cone every root's
/// distance from the apex is decided after ([`off_the_apex`]): a root at
/// the apex refuses the whole span.
///
/// # Errors
///
/// A kernel fault's readings: a form of another arm, a carrier or a
/// surface of another kind, a walk's fault.
pub(super) fn conic_quadric_method<T: Decide>(
    span: &crate::curve_pair::Span<'_, T>,
    surface: &geom::Surface<T>,
    form: super::pose::Formed<T>,
    band: Band,
) -> Result<Walked<T>, &'static str> {
    use super::pose::Formed;
    let carrier = span.carrier();
    let (t0, t1) = span.params();
    let conic = carrier.conic().ok_or(DESYNC)?;
    let walked = match form {
        Formed::FirstHarmonic(h) => match super::circle_roots::harmonic_roots_as(
            &h,
            conic.speed_hi(),
            (t0, t1),
            "conic_quadric_first_root_slack",
            band,
        ) {
            Ok(found) => Walked::found(match found {
                Roots::Certified(found) => found.roots().to_vec(),
                Roots::Miss | Roots::On(_) => Vec::new(),
            }),
            Err(refused) => {
                let slack = refused.slack;
                let fault = MeetFault::Unsettled(Unsettled::Meter { row: refused.row });
                Walked {
                    roots: Vec::new(),
                    windows: refused
                        .roots
                        .iter()
                        .map(|&t| ((t - slack, t + slack), fault))
                        .collect(),
                }
            }
        },
        Formed::Ladder => match *surface {
            geom::Surface::Cone {
                frame, half_angle, ..
            } => {
                let (apex, axis) = (frame.origin(), frame.axis_unit());
                cone_ladder(&conic, (apex, axis, half_angle), (t0, t1), surface, band)?
            }
            _ => {
                let QuadricLadder { h, quadric, frame } =
                    quadric_ladder(&conic, surface, (t0, t1)).ok_or(DESYNC)?;
                Walked::of_walk(
                    (t0, t1),
                    half_angle_roots_past(
                        &DegreeTwo::new(h.c0, h.c1, h.s1, h.c2, h.s2),
                        |theta| crate::implicit_residual(surface, conic.point_at(theta)),
                        frame,
                        &LADDER_ROWS,
                        &RootSlack {
                            row: LADDER_ROOT_SLACK,
                            residual: &|theta| {
                                crate::conic_quadric_residual(&conic, quadric, theta)
                            },
                            f_per_metre_hi: T::one(),
                        },
                        band,
                    ),
                )?
            }
        },
        _ => return Err("the conic × quadric method was handed a form of another arm"),
    };
    Ok(match *surface {
        geom::Surface::Cone { frame, .. } => {
            let roots = CircleRoots::of(&walked.roots)
                .ok_or("a conic read more roots against a cone than its degree allows")?;
            match off_the_apex(&conic, frame.origin(), roots, band) {
                Ok(_) => walked,
                Err(why) => Walked::whole((t0, t1), MeetFault::Unsettled(why)),
            }
        }
        _ => walked,
    })
}

/// A conic against a sphere or a wall, read for the door's two arms: its
/// harmonics, the quadric as
/// [`crate::conic_quadric_residual`] reads it, and the ladder's frame,
/// which carries the harmonics' noise.
pub(super) struct QuadricLadder<'a, T: Decide> {
    pub(super) h: crate::ConicHarmonics<T>,
    quadric: (geom_core::Point3<T>, Option<geom_core::UnitVec3<T>>, T),
    pub(super) frame: HalfAngleFrame<'a, T>,
}

/// [`QuadricLadder`] for `conic` against `surface` over `[t0, t1]`, or
/// `None` when `surface` is neither a sphere nor a cylinder.
pub(super) fn quadric_ladder<T: Decide>(
    conic: &dyn geom::Conic<T>,
    surface: &geom::Surface<T>,
    (t0, t1): (T, T),
) -> Option<QuadricLadder<'static, T>> {
    let (h, quadric) = match *surface {
        geom::Surface::Sphere { frame, radius, .. } => {
            let center = frame.origin();
            (
                crate::conic_sphere_harmonics(conic, center, radius),
                (center, None, radius),
            )
        }
        geom::Surface::Cylinder { frame, radius, .. } => {
            let (origin, axis) = (frame.origin(), frame.axis_unit());
            (
                crate::conic_cylinder_harmonics(conic, geom::Line { origin, dir: axis }, radius),
                (origin, Some(axis), radius),
            )
        }
        _ => return None,
    };
    // The harmonics' rounding, in residual metres: the term bound is in
    // m², before the `2r` division. It is a charge on the residual's
    // evaluation (`geom_core::SHORT_CHAIN_ULPS`), so it covers the
    // arm's readings together — `c₀ ∓ A₁` and the `A₂` it charges.
    let noise = rounding_charge(h.terms) / h.per;
    let frame = ladder_frame(conic, (t0, t1), noise, h.floor);
    Some(QuadricLadder { h, quadric, frame })
}

/// The ladder's root-slack row on a sphere or a wall (module docs, "Two
/// arms"): `F` is the residual, so its ceiling per metre is `1`.
const LADDER_ROOT_SLACK: &str = "conic_quadric_sub_root_slack";

/// The ladder's frame for a conic: `F`'s ceiling on `|F|` per metre of
/// residual is `1` on every kind ([`crate::ConicHarmonics::floor`]),
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
        },
        jet: None,
        speed_lo: conic.speed_lo(),
        lever: T::from_f64(2.0) * conic.speed_lo(),
    }
}

/// The cone arm's rows: the root slack and the apex, read on each root
/// the arms certify (module docs, "Against a cone").
const CONE_ROOT_SLACK: &str = "conic_quadric_cone_root_slack";
const CONE_APEX: &str = "conic_quadric_cone_apex";

/// The cone's quadric form `F = Q/R` along `conic` (module docs,
/// "Against a cone").
pub(super) fn cone_harmonics<T: Decide>(
    conic: &dyn geom::Conic<T>,
    (apex, axis, half_angle): (geom_core::Point3<T>, geom_core::UnitVec3<T>, T),
) -> crate::ConicHarmonics<T> {
    crate::conic_cone_harmonics(
        conic,
        geom::Line {
            origin: apex,
            dir: axis,
        },
        half_angle,
    )
}

/// The rounding of a cone's form `F = Q/R` as the door evaluates it, in
/// `F`'s units: the term bound's charge, over `R`.
pub(super) fn cone_noise<T: Decide>(h: &crate::ConicHarmonics<T>) -> T {
    rounding_charge(h.terms) / h.per
}

/// The ladder against a CONE (module docs, "Against a cone"): the
/// wall's ladder on `F = Q/R`, every root's slack metered on the cone's
/// residual.
fn cone_ladder<T: Decide>(
    conic: &dyn geom::Conic<T>,
    (apex, axis, half_angle): (geom_core::Point3<T>, geom_core::UnitVec3<T>, T),
    (t0, t1): (T, T),
    surface: &geom::Surface<T>,
    band: Band,
) -> Result<Walked<T>, &'static str> {
    let h = cone_harmonics(conic, (apex, axis, half_angle));
    let noise = cone_noise(&h);
    let placed = |theta: T| {
        crate::conic_cone_residual(
            conic,
            geom::Line {
                origin: apex,
                dir: axis,
            },
            half_angle,
            theta,
        )
    };
    Walked::of_walk(
        (t0, t1),
        half_angle_roots_past(
            &DegreeTwo::new(h.c0, h.c1, h.s1, h.c2, h.s2),
            |theta| crate::implicit_residual(surface, conic.point_at(theta)),
            ladder_frame(conic, (t0, t1), noise, h.floor),
            &LADDER_ROWS,
            &RootSlack {
                row: CONE_ROOT_SLACK,
                residual: &placed,
                f_per_metre_hi: T::one(),
            },
            band,
        ),
    )
}

/// The rows of a conic through the apex (module docs, "Through the
/// apex"), past the pose's apex and spread.
const CONE_DEFLATED_DEPTH: &str = "conic_quadric_cone_deflated_depth";
const CONE_DEFLATED_SIDE: &str = "conic_quadric_cone_deflated_side";
const CONE_DEFLATED_CLEAR: &str = "conic_quadric_cone_deflated_clear";

/// The deflated form about the apex's anomaly `θ_a` (module docs,
/// "Through the apex"): `G = k + R cos(φ − β)`, as `(k, R, (R cos β,
/// R sin β), G(0))`.
fn deflated<T: Decide>(h: &crate::ConicHarmonics<T>, theta_a: T) -> (T, T, (T, T), T) {
    let two = T::from_f64(2.0);
    let (sa, ca) = theta_a.sin_cos();
    let (s2a, c2a) = (two * theta_a).sin_cos();
    let c1 = h.c1 * ca + h.s1 * sa;
    let (c2, s2) = (h.c2 * c2a + h.s2 * s2a, h.s2 * c2a - h.c2 * s2a);
    let k = c1 + two * c2;
    let (rc, rs) = (two * c2, two * s2);
    let r = (rc.powi(2) + rs.powi(2)).sqrt();
    (k, r, (rc, rs), k + rc)
}

/// **The apex's spread** (module docs, "Through the apex"), in radians
/// of the carrier's parameter about `θ_a`: the bound
/// ([`apex_spread_bound`]) the pose decides on. The `ε`s are read off the
/// geometry, `δ²/per` and `2δ·speed/per` at the point's distance `δ`
/// from the apex, its own rounding carried, not off the rounded
/// coefficients, whose noise a square root would amplify past any band.
#[expect(
    clippy::disallowed_methods,
    reason = "a conic's centre frame, derived where an ellipse is held on its arc: owed a reading through the conic (work/oneround/one-stored-form-waits-on-the-readers-of-a-conics-centre.md)"
)]
pub(super) fn apex_spread<T: Decide>(
    conic: &dyn geom::Conic<T>,
    (apex, _, _): (geom_core::Point3<T>, geom_core::UnitVec3<T>, T),
    h: &crate::ConicHarmonics<T>,
    theta_a: T,
) -> T {
    let two = T::from_f64(2.0);
    let off_apex = conic.point_at(theta_a) - apex;
    let delta = off_apex.norm()
        + geom_core::running::direction_rounding(
            (conic.frame().origin() - apex).norm() + conic.speed_hi(),
        );
    let (e0, e1) = (
        delta.powi(2) / h.per,
        two * delta * conic.speed_hi() / h.per,
    );
    let (_, r, _, g0) = deflated(h, theta_a);
    apex_spread_bound((e0, e1), g0.abs(), r)
}

/// **A conic through the cone's apex, read whole** (module docs,
/// "Through the apex"): the method of a [`super::pose::Pose::Through`],
/// the deflated form's roots, each certified. The apex's own double root
/// is the pose's datum, not among them. It reads the whole turn, not the
/// caller's arc: a refusal stands even where the arc holds neither the
/// apex nor the root it could not read.
///
/// # Errors
///
/// A surface that is not a cone, or a carrier that is not a conic: a
/// kernel fault.
pub(super) fn through_the_apex<T: Decide>(
    span: &crate::curve_pair::Span<'_, T>,
    surface: &geom::Surface<T>,
    apex: super::pose::Singular<T>,
    band: Band,
) -> Result<Walked<T>, &'static str> {
    let geom::Surface::Cone {
        frame, half_angle, ..
    } = *surface
    else {
        return Err("a singular point of a surface that is not a cone");
    };
    let conic = span.carrier().conic().ok_or(DESYNC)?;
    Ok(
        match deflated_roots(
            &conic,
            (frame.axis_unit(), half_angle),
            span.params(),
            apex,
            band,
        ) {
            Ok(found) => Walked::found(match found {
                Roots::Certified(found) => found.roots().to_vec(),
                Roots::Miss | Roots::On(_) => Vec::new(),
            }),
            Err(why) => Walked::whole(span.params(), MeetFault::Unsettled(why)),
        },
    )
}

/// [`through_the_apex`]'s reading of the deflated form's roots on
/// `conic` against the cone `(axis, half_angle)` about `apex`.
fn deflated_roots<T: Decide>(
    conic: &dyn geom::Conic<T>,
    (axis, half_angle): (geom_core::UnitVec3<T>, T),
    (t0, t1): (T, T),
    apex: super::pose::Singular<T>,
    band: Band,
) -> Result<CircleRoots<T>, Unsettled> {
    let h = cone_harmonics(conic, (apex.point, axis, half_angle));
    let noise = cone_noise(&h);
    let two = T::from_f64(2.0);
    let mid = geom::mid_param(t0, t1);
    let near_mid = |theta: T| geom::conic_period().near(theta, mid);
    let theta_a = apex.at;
    let spread_phi = apex.spread.1 - theta_a;
    let (k, r, (rc, rs), _) = deflated(&h, theta_a);
    let beta = rs.atan2(rc);
    // `G`'s own roots: none where `|k| − R` is definite, two where it is
    // definitely negative, a tangency between.
    let depth = k.abs() - r;
    match decide(CONE_DEFLATED_DEPTH, Margin::of(depth - noise), band) {
        Ok(Sign::Positive) => return Ok(CircleRoots::Miss),
        Ok(Sign::Negative) => {}
        Ok(Sign::Zero) | Err(_) => return Err(Unsettled::Tangency),
    }
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
    // band (`conic_quadric_cone_root_slack`, the arms' quantity with a
    // whole-bracket slope bound in place of their window's). A shallow
    // crossing reads neither, and refuses as the tangency it is near.
    let width = T::from_f64(64.0) * T::from_f64(band.escalate()) / conic.speed_lo();
    let residual = |theta: T| {
        crate::conic_cone_residual(
            conic,
            geom::Line {
                origin: apex.point,
                dir: axis,
            },
            half_angle,
            theta,
        )
    };
    let f = |theta: T| deflated_reading(&h, noise, theta).0;
    let slope = |theta: T| deflated_reading(&h, noise, theta).1;
    let bend = (h.c1.powi(2) + h.s1.powi(2)).sqrt()
        + T::from_f64(4.0) * (h.c2.powi(2) + h.s2.powi(2)).sqrt();
    let mut found = [theta_a; 2];
    for (slot, phi) in [beta + spin, beta - spin].into_iter().enumerate() {
        let theta = near_mid(theta_a + phi);
        let off = geom::separation(theta, theta_a) - width - spread_phi;
        match decide(
            CONE_DEFLATED_CLEAR,
            Margin::of(off * conic.speed_lo()),
            band,
        ) {
            Ok(Sign::Positive) => {}
            Ok(Sign::Zero | Sign::Negative) | Err(_) => return Err(Unsettled::AtApex),
        }
        match (
            side_past_rounding(residual(theta - width), band),
            side_past_rounding(residual(theta + width), band),
        ) {
            (Some(a), Some(b)) if a != b => {}
            _ => return Err(Unsettled::Tangency),
        }
        let slack = cone_root_slack(f(theta), slope(theta), width * bend, conic.speed_hi());
        match decide(CONE_ROOT_SLACK, Margin::of(slack), band) {
            Ok(Sign::Zero) => found[slot] = theta,
            Ok(Sign::Positive | Sign::Negative) | Err(_) => {
                return Err(Unsettled::Meter {
                    row: CONE_ROOT_SLACK,
                });
            }
        }
    }
    let apart = geom::separation(found[0], found[1]) - two * width;
    match decide(
        CONE_DEFLATED_CLEAR,
        Margin::of(apart * conic.speed_lo()),
        band,
    ) {
        Ok(Sign::Positive) => {}
        Ok(Sign::Zero | Sign::Negative) | Err(_) => return Err(Unsettled::Tangency),
    }
    Ok(Roots::<T, 2>::all(found).widen())
}

/// **`F` and `F′` at `theta` off the rounded harmonics**, each with its
/// rounding: `F` within `noise`, the bound on the harmonics' error
/// everywhere, and `F′` within twice it, the derivative's coefficients
/// being at most twice `F`'s (the second harmonic's turn).
fn deflated_reading<T: Decide>(
    h: &crate::ConicHarmonics<T>,
    noise: T,
    theta: T,
) -> (Rounded<T>, Rounded<T>) {
    let two = T::from_f64(2.0);
    let (s, c) = theta.sin_cos();
    let (s2, c2) = (two * theta).sin_cos();
    (
        Rounded {
            value: h.c0 + h.c1 * c + h.s1 * s + h.c2 * c2 + h.s2 * s2,
            error: noise,
        },
        Rounded {
            value: h.s1 * c - h.c1 * s + two * (h.s2 * c2 - h.c2 * s2),
            error: two * noise,
        },
    )
}

/// The side a residual reads past its own rounding
/// (`bool_conic_cone_deflated_side`), `None` where its rounding or the
/// band holds it.
fn side_past_rounding<T: Decide>(res: Rounded<T>, band: Band) -> Option<Sign> {
    match decide(CONE_DEFLATED_SIDE, Margin::of(res.toward_zero()), band) {
        Ok(sign @ (Sign::Positive | Sign::Negative)) => Some(sign),
        _ => None,
    }
}

/// **A deflated root's slack**, in metres at the carrier's top speed
/// `speed`: how far from a root where `F` reads `f` the true root can
/// lie, `|F|` at most `f`'s magnitude and `|F′|` at least the room of its
/// reading `slope` less `drift`, its most change across the bracket.
/// Past every bound where that least slope is not positive.
fn cone_root_slack<T: Decide>(f: Rounded<T>, slope: Rounded<T>, drift: T, speed: T) -> T {
    let least = slope.room() - drift;
    least.select_le_zero(T::from_f64(f64::INFINITY), speed * f.magnitude() / least)
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
    /// dispatched on a circle or an ellipse against a sphere, a wall or a
    /// cone;
    /// anything else reaching it is the caller's broken invariant, never
    /// an answer.
    #[test]
    fn a_wrong_kind_is_a_desync() {
        let (o, z, x) = (
            Point3::origin(),
            Vec3::new(0.0, 0.0, 1.0),
            Vec3::new(1.0, 0.0, 0.0),
        );
        let line = geom::Curve3::Line(geom::Line {
            origin: o,
            dir: geom_core::UnitVec3::checked(x),
        });
        let circle = geom::Curve3::circle(o, z, 1.0, x);
        let ellipse =
            geom::Curve3::Ellipse(geom::Ellipse::new(geom::Frame::new(o, z, x), 1.0, 0.5));
        let sphere = geom::Surface::Sphere {
            frame: geom::Frame::new(o, z, x),
            radius: 1.0,
        };
        let wall = geom::Surface::Cylinder {
            frame: geom::Frame::new(o, z, x),
            radius: 1.0,
        };
        let plane = geom::Surface::Plane {
            frame: geom::Frame::new(o, z, x),
        };
        let torus = geom::Surface::Torus {
            frame: geom::Frame::new(o, z, x),
            major_radius: 1.0,
            minor_radius: 0.25,
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
                matches!(got, Err(ConicQuadricRefusal::Invariant(_))),
                "{carrier:?} against {surface:?} refuses as a kernel invariant, got {got:?}"
            );
        }
    }
}
