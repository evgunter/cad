//! **The circle root cores**: the two root finders every circle × surface
//! door hands its residual to. A door ([`super::conic_quadric`],
//! [`super::circle_torus`], [`super::ellipse_torus`]) computes the
//! residual's harmonics from the geometry, names its rows and its
//! decision, and answers [`CircleRoots`]; the first harmonic's root
//! finding lives here, and a degree-two or higher residual's in
//! [`geom_brep::half_angle`], which [`door_half_angle_roots`] and
//! [`door_refusal`] read in the door's vocabulary.
//! Every door has one shape: `(carrier, t0, t1, surface, band)`,
//! a wrong kind is [`BooleanError::ClassificationInvariant`], and an
//! in-band classifying sign escalates as the door's own decision,
//! wrapped here.
//!
//! # The first-harmonic door
//!
//! A residual `R(θ) = c₀ + A₁ cos(θ − φ)` — a circle against a sphere,
//! or against a cylinder wall it is square to — has the EXACT range
//! `[lo, hi] = [c₀ − A₁, c₀ + A₁]`, and its roots are
//! `θ = φ ± acos(−c₀/A₁)`. A door hands [`first_harmonic_roots`] the
//! extremes themselves, each with a bound on its error, and the phase's
//! components with theirs; `c₀` and `A₁` are read off the extremes. It
//! decides on that range, all in residual metres, under the caller's
//! rows:
//!
//! - **noise** — the larger of the extremes' error bounds definitely
//!   past the escalation threshold, or not readable at all, refuses:
//!   every decision below reads the extremes, and the representation
//!   cannot resolve what the band asks of them. A rounding estimate on
//!   `f64`, run on every scalar: the `Interval` lane carries its own
//!   enclosure and needs no meter to be sound, but the meter still
//!   reads there and can refuse a pose the enclosures alone would
//!   answer.
//! - **coaxial** — the swing `A₁` in the zero band: the residual is
//!   constant to within `A₁` plus that error, and
//!   [`constant_residual_roots`] decides it.
//! - **extreme**, on `lo` and `hi`, each read as the interval its error
//!   bound spans, both ends decided — definitely one-signed is a miss,
//!   definitely straddling is two roots, an end in the escalation gap
//!   escalates, and an extreme otherwise not definite is a tangency,
//!   which is not a crossing at any order this lane sees and refuses
//!   [`Unsettled::Tangency`]. The noise meter bounds the error by the
//!   zero band, not by the gap's far edge, so an extreme read bare could
//!   clear `Kε` while its true value sits in the gap.
//! - **root slack** — at either root the extremes' errors move the
//!   residual by `δR = (hi·δlo − lo·δhi)/(hi − lo)` — the NEAR extreme's
//!   error, plus only a share `|near|/(hi − lo)` of the far one's — so
//!   the root moves by `δR / |R′|`, `|R′| = √(−lo·hi)` at both roots;
//!   the phase's error moves it by itself, and the angle arithmetic
//!   rounds by [`geom_core::SHORT_CHAIN_ULPS`] half-ulps of a turn. That arc length must
//!   be definitely inside the band, or the span and trim decisions the
//!   caller makes on the point are made on the wrong point. An
//!   unreadable reading refuses here too. A door whose only account is
//!   one uniform `noise` charges it to both extremes and nothing to the
//!   phase, and the slack is then `noise / |R′|` plus the angle charge.
//!
//! The half-chord is measured from the extreme nearer zero,
//! `2·asin(√(|near|/(hi − lo)))` past it, which reads the near extreme
//! to its own relative precision: `acos(−c₀/A₁)`, or the same `asin`
//! read from the far extreme, near a tangency amplifies its argument's
//! rounding by `1/√(1 − (c₀/A₁)²)`.
//!
//! Two DISTINCT certified roots therefore certify that the carrier does
//! not lie on the surface — the fact the reduction's `(Zero, Zero)`
//! chord arm leans on to separate a chord from an on-carrier edge.
//!
//! The half-angle ladder ([`geom_brep::half_angle`]) is not used for a
//! first harmonic, for two
//! reasons that are properties of the ladder rather than of its cost.
//! A CONSTANT residual (a coaxial carrier) makes the quartic
//! `F·(1 + t²)²`, a repeated complex pair the ladder cannot answer, where
//! the extremes say on-surface or miss. And a TANGENCY must escalate: the
//! extremes put it in the zero band, where the ladder, deciding on a
//! discriminant rather than on the range, can certify a miss
//! (`work/germ/the-half-angle-ladder-certifies-in-band-configurations.md`).
//!
//! # A constant residual
//!
//! A coaxial carrier's residual is ONE value `c` only to within the
//! carrier's in-band offset and tilt: in the band, not zero. Its spread
//! `s` about `c` — a bound each door derives from that offset and tilt,
//! never from the band — is charged into the one decision every
//! constant-residual answer takes ([`constant_residual_roots`]): the
//! carrier is a [`Roots::Miss`] when `c − s` is definitely
//! positive or `c + s` definitely negative, [`Roots::On`]
//! when both are in the zero band, and otherwise refused. Read at
//! `c` alone, an in-band spread can cross zero while `c` sits past the
//! escalation threshold whenever `K` is near 1.
//!

use geom_brep::half_angle::{
    DegreeTwo, HalfAngleFrame, HalfAngleRows, MAX_DEGREE, RootSlack, half_angle_roots,
};
use geom_brep::roots::{OnCarrier, RootRefusal, Roots, Unsettled, extreme_sign, spread_signs};
use geom_core::{Band, Decide, Margin, Point3, Sign, Vec3};

use super::{BooleanDecision, BooleanError};
use crate::validate::decide;

/// What the certified roots of a circle against a surface say about the
/// whole carrier — the one answer every circle root door gives, in the
/// root finders' one shape ([`geom_brep::roots`]): a miss, the carrier ON
/// the surface (its residual a zero constant; a constant definite
/// residual is a miss), or a certified count (even, at most twice the
/// residual's degree) of carrier parameters `θ`, each within `π` of the
/// arc's midpoint, so it compares with the arc `[t₀, t₁]` the caller
/// passed without wrapping. A door that certifies none of these refuses
/// with an [`Unsettled`] beside its [`BooleanError`].
pub(super) type CircleRoots<T> = Roots<T, { 2 * MAX_DEGREE }, OnCarrier>;

/// [`half_angle_roots`] as a door answers it: its roots as
/// [`CircleRoots`], its count it could not certify as the inner `Err`,
/// and the ladder's in-band sign escalated as the door's `decision`.
///
/// # Errors
///
/// As above.
pub(super) fn door_half_angle_roots<T: Decide>(
    f: &DegreeTwo<T>,
    residual: impl Fn(T) -> T,
    frame: HalfAngleFrame<T>,
    rows: &HalfAngleRows,
    slack: &RootSlack<'_, T>,
    decision: BooleanDecision,
    band: Band,
) -> Result<Result<CircleRoots<T>, Unsettled>, BooleanError> {
    Ok(door_refusal(
        half_angle_roots(f, residual, frame, rows, slack, band),
        decision,
    )?
    .map(Roots::widen))
}

/// A root finder's answer as a door gives it: a limit as the inner
/// `Err`, a sign in the band's gap (a classifying sign, or a touch)
/// escalated as the door's `decision`, and a fault a kernel defect.
///
/// # Errors
///
/// [`BooleanError::Escalated`], as above, and
/// [`BooleanError::ClassificationInvariant`] for a fault.
pub(super) fn door_refusal<V>(
    found: Result<V, RootRefusal>,
    decision: BooleanDecision,
) -> Result<Result<V, Unsettled>, BooleanError> {
    match found {
        Ok(roots) => Ok(Ok(roots)),
        Err(RootRefusal::Unsettled(why)) => Ok(Err(why)),
        Err(RootRefusal::Escalated(diag) | RootRefusal::Touch(diag)) => {
            Err(BooleanError::Escalated { decision, diag })
        }
        Err(RootRefusal::Fault(fault)) => Err(BooleanError::ClassificationInvariant {
            what: fault.readings(),
        }),
    }
}

/// **The answer for a residual constant along the carrier to within
/// `spread`** about `value` (module docs, "A constant residual"), both in
/// metres, decided under `row`. `spread` must bound the residual's
/// distance from `value` everywhere on the carrier.
///
/// # Errors
///
/// [`RootRefusal::Escalated`] when `value ∓ spread` lies in the band's
/// gap, and [`Unsettled::Tangency`] when one end is in the zero band and
/// the other definite: the carrier is neither on the surface nor clear
/// of it.
pub(super) fn constant_residual_roots<T: Decide>(
    value: T,
    spread: T,
    row: &'static str,
    band: Band,
) -> Result<FirstHarmonicRoots<T>, RootRefusal> {
    match spread_signs(row, value, spread, band)? {
        (Sign::Positive, _) | (_, Sign::Negative) => Ok(Roots::Miss),
        (Sign::Zero, Sign::Zero) => Ok(Roots::On(OnCarrier)),
        _ => Err(Unsettled::Tangency.into()),
    }
}

/// What [`first_harmonic_roots`] and [`constant_residual_roots`] answer:
/// [`CircleRoots`]' shape with room for two roots, the most a first
/// harmonic has.
pub(super) type FirstHarmonicRoots<T> = Roots<T, 2, OnCarrier>;

/// A residual `c₀ + A₁ cos(θ − φ)` along a circle, `φ = atan2(sin_part, cos_part)`,
/// in metres of residual, given by its extremes `lo = c₀ − A₁` and
/// `hi = c₀ + A₁` — the one spelling of the range the door decides on,
/// `c₀` and `A₁` read off it — and the phase's two components.
///
/// Each comes with a bound on its error as the door evaluated it:
/// `lo_noise` and `hi_noise` in metres of residual (their rounding, and
/// any term the caller dropped to reach this form, which must bound the
/// residual's error at every `θ` where it is not specific to one
/// extreme), and `phase_noise` in the units of `(cos_part, sin_part)`.
/// A door whose only account is one uniform `noise` passes it as both
/// extremes' and no phase charge: a residual error bounded at every `θ`
/// already moves the roots by no more than that over `|R′|`, the
/// phase's share included.
pub(super) struct FirstHarmonic<T> {
    pub(super) lo: T,
    pub(super) hi: T,
    pub(super) cos_part: T,
    pub(super) sin_part: T,
    pub(super) lo_noise: T,
    pub(super) hi_noise: T,
    pub(super) phase_noise: T,
}

impl<T: Decide> FirstHarmonic<T> {
    /// **A circle's signed distance off a plane**: the circle
    /// `center + radius·(û cos θ + v̂ sin θ)`, `v̂ = axis × û`, against the
    /// plane through `on` of unit normal `n`, its residual
    /// `n·(center − on) + radius(n·û) cos θ + radius(n·v̂) sin θ` and one
    /// uniform rounding charge on the terms it is formed from. The one
    /// spelling of a circle against a plane: an edge against a ray's or a
    /// leg's plane, a leg against an edge's, a sphere face's arc against a
    /// cut.
    pub(super) fn circle_off_plane(
        (center, axis, radius, u_ref): (Point3<T>, Vec3<T>, T, Vec3<T>),
        plane: (Point3<T>, Vec3<T>),
    ) -> Self {
        Self::conic_off_plane((center, axis, (radius, radius), u_ref), plane)
    }

    /// **A conic's signed distance off a plane**: the ellipse
    /// `center + a·û cos θ + b·v̂ sin θ`, `v̂ = axis × û` (a circle where
    /// `a = b`), against the plane through `on` of unit normal `n` — the
    /// residual `n·(center − on) + a(n·û) cos θ + b(n·v̂) sin θ`, charged
    /// as [`Self::circle_off_plane`] charges a circle, at the larger
    /// semi-axis.
    pub(super) fn conic_off_plane(
        (center, axis, (a, b), u_ref): (Point3<T>, Vec3<T>, (T, T), Vec3<T>),
        (on, n): (Point3<T>, Vec3<T>),
    ) -> Self {
        let offset = center - on;
        let d = n.dot(offset);
        let (cos_part, sin_part) = (a * n.dot(u_ref), b * n.dot(axis.cross(u_ref)));
        let amplitude = (cos_part.powi(2) + sin_part.powi(2)).sqrt();
        let noise = geom_core::rounding_charge(offset.norm() + a.max(b));
        Self {
            lo: d - amplitude,
            hi: d + amplitude,
            cos_part,
            sin_part,
            lo_noise: noise,
            hi_noise: noise,
            phase_noise: T::zero(),
        }
    }
}

/// The predicate rows one caller of [`first_harmonic_roots`] meters
/// under (module docs: noise, coaxial, extreme, root slack; all metres),
/// and the decision an in-band extreme escalates as.
pub(super) struct FirstHarmonicRows {
    pub(super) noise: &'static str,
    pub(super) coaxial: &'static str,
    pub(super) extreme: &'static str,
    pub(super) root_slack: &'static str,
    pub(super) decision: BooleanDecision,
}

/// **The certified roots of a first-harmonic residual along a carrier
/// whose speed `|C′(θ)|` is at most `speed`** (a circle's radius, an
/// ellipse's larger semi-axis), reported within `π` of the midpoint of `[t0, t1]`: the
/// decisions of the module docs ("The first-harmonic door"), under the
/// caller's `rows`.
///
/// # Errors
///
/// A coincidence escalation, as `rows.decision`, when the constant
/// residual of a coaxial carrier, or an extreme residual, lies in the
/// band's escalation gap. An escalated noise or root-slack reading is
/// NOT an escalation: it refuses [`Unsettled::Meter`] in the inner `Err`,
/// as a definitely excessive one does — a meter that cannot be read does
/// not license the roots it meters. An extreme in the zero band refuses
/// [`Unsettled::Tangency`] there. An escalated coaxial test falls through
/// to the extremes, which decide the same carrier.
pub(super) fn first_harmonic_roots<T: Decide>(
    h: &FirstHarmonic<T>,
    speed: T,
    t0: T,
    t1: T,
    rows: &FirstHarmonicRows,
    band: Band,
) -> Result<Result<FirstHarmonicRoots<T>, Unsettled>, BooleanError> {
    door_refusal(
        first_harmonic_roots_in_band(h, speed, t0, t1, rows, band),
        rows.decision,
    )
}

/// [`first_harmonic_roots`] with its refusals left as the root finders'
/// own, for a caller that is not the boolean's and names no
/// [`BooleanDecision`] (`rows.decision` is not read).
///
/// # Errors
///
/// As [`first_harmonic_roots`], as the bare [`RootRefusal`].
pub(super) fn first_harmonic_roots_in_band<T: Decide>(
    h: &FirstHarmonic<T>,
    speed: T,
    t0: T,
    t1: T,
    rows: &FirstHarmonicRows,
    band: Band,
) -> Result<FirstHarmonicRoots<T>, RootRefusal> {
    harmonic_roots(h, speed, Some((t0, t1)), rows, band)
}

/// **[`first_harmonic_roots_in_band`] with the roots unwound onto no
/// branch**: `φ ± acos(−c₀/A₁)` as computed, a representative of each
/// root modulo a turn, for a reader that reads a root only through the
/// point it gives (a leg's own parameter, read as `cos s` and `sin s`).
/// Unwinding onto a span's branch moves a root near the branch's cut by a
/// turn either way, which at an enclosing scalar is a whole turn's hull
/// (`Real::periodic_branch`'s tie): a leg whose start lies on the plane
/// has a root there, at the cut of a whole-turn span's branch.
///
/// # Errors
///
/// As [`first_harmonic_roots_in_band`].
pub(super) fn first_harmonic_roots_unwound<T: Decide>(
    h: &FirstHarmonic<T>,
    speed: T,
    rows: &FirstHarmonicRows,
    band: Band,
) -> Result<FirstHarmonicRoots<T>, RootRefusal> {
    harmonic_roots(h, speed, None, rows, band)
}

/// The one body of [`first_harmonic_roots_in_band`] and
/// [`first_harmonic_roots_unwound`]: the roots on the branch of
/// `window`'s middle, or unwound where there is none.
fn harmonic_roots<T: Decide>(
    h: &FirstHarmonic<T>,
    speed: T,
    window: Option<(T, T)>,
    rows: &FirstHarmonicRows,
    band: Band,
) -> Result<FirstHarmonicRoots<T>, RootRefusal> {
    let FirstHarmonic {
        lo: lo_value,
        hi: hi_value,
        cos_part,
        sin_part,
        lo_noise,
        hi_noise,
        phase_noise,
    } = *h;
    let two = T::from_f64(2.0);
    // The extremes are what every decision below reads, so their error
    // is what the representation must resolve.
    let noise = lo_noise.max(hi_noise);
    match decide(rows.noise, Margin::of(noise), band) {
        Ok(Sign::Zero | Sign::Negative) => {}
        Ok(Sign::Positive) | Err(_) => return Err(Unsettled::Meter.into()),
    }
    let (c0, a1) = ((lo_value + hi_value) / two, (hi_value - lo_value) / two);
    if let Ok(Sign::Zero) = decide(rows.coaxial, Margin::of(a1), band) {
        return constant_residual_roots(c0, a1 + noise, rows.extreme, band);
    }
    let lo = extreme_sign(rows.extreme, lo_value, lo_noise, band)?;
    if lo == Sign::Positive {
        return Ok(Roots::Miss);
    }
    let hi = extreme_sign(rows.extreme, hi_value, hi_noise, band)?;
    if hi == Sign::Negative {
        return Ok(Roots::Miss);
    }
    if (lo, hi) != (Sign::Negative, Sign::Positive) {
        return Err(Unsettled::Tangency.into());
    }
    // At either root `R = lo·(1 − cos ψ)/2 + hi·(1 + cos ψ)/2` with
    // `(1 + cos ψ)/2 = −lo/(hi − lo)`, so the extremes' errors move the
    // residual there by `(hi·δlo − lo·δhi)/(hi − lo)`, and the phase's
    // moves the root itself; `|R′| = √(−lo·hi)` at both roots.
    let swing = hi_value - lo_value;
    let slope = ((T::zero() - lo_value) * hi_value).max(T::zero()).sqrt();
    let at_root = (hi_value * lo_noise - lo_value * hi_noise) / swing;
    let phase = phase_noise / (cos_part.powi(2) + sin_part.powi(2)).sqrt();
    let slack = speed * (at_root / slope + phase + geom_core::rounding_charge(T::tau()));
    match decide(rows.root_slack, Margin::of(slack), band) {
        Ok(Sign::Zero | Sign::Negative) => {}
        Ok(Sign::Positive) | Err(_) => return Err(Unsettled::Meter.into()),
    }
    let phi = sin_part.atan2(cos_part);
    // The half-chord `acos(−c₀/A₁)`, measured from the extreme nearer
    // zero (module docs).
    let past = |near: T| geom_brep::roots::arc_past(near.abs() / swing);
    let half_chord = (lo_value + hi_value).select_le_zero(past(hi_value), T::pi() - past(lo_value));
    // The roots on the branch of the span's middle: sound at every scalar
    // by `ring_path::branch`'s window re-centring argument. Its readers
    // read an end of a whole-turn span alike: the wall crossing's
    // `at_end`, the sphere cut-in's vertex lookup at either end, and
    // `sphere_region`'s and `torus_region`'s readings `sideless` at
    // either end. None reads
    // a root as an identity, which is why the floor stays: a structural
    // pick would decide nothing a reader uses.
    let (a, b) = (phi - half_chord, phi + half_chord);
    Ok(Roots::all(match window {
        Some(span) => [near_span(a, span), near_span(b, span)],
        None => [a, b],
    }))
}

/// **An angle on the branch of a span's middle**: `raw` moved by whole
/// turns to within `π` of the middle of `(t0, t1)`, so it compares with the
/// span's ends without wrapping. The one unwinding of a carrier's angle
/// onto the span it is read against: this door's roots, and an angle a
/// region reads off a point of an edge.
pub(super) fn near_span<T: Decide>(raw: T, (t0, t1): (T, T)) -> T {
    let mid = (t0 + t1) / T::from_f64(2.0);
    mid + (raw - mid).reduce_periodic_centred(T::tau())
}

/// A root door's answer for a row: its roots, or its refusal, a limit
/// (`Some`) or `None` for its decision's escalation.
///
/// # Panics
///
/// On any other refusal.
#[cfg(test)]
#[allow(clippy::panic)]
pub(super) fn door_answer<T: geom_core::Real>(
    got: Result<Result<CircleRoots<T>, Unsettled>, BooleanError>,
) -> Result<CircleRoots<T>, Option<Unsettled>> {
    match got {
        Ok(found) => found.map_err(Some),
        Err(BooleanError::Escalated { .. }) => Err(None),
        Err(other) => panic!("the door refused outside its decision: {other:?}"),
    }
}
