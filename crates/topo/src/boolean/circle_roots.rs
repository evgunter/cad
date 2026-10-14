//! **The circle root cores**: the two root finders every circle × surface
//! door hands its residual to. A door ([`super::conic_quadric`],
//! [`super::circle_torus`], [`super::ellipse_torus`]) computes the
//! residual's harmonics from the geometry, names its rows and its
//! decision, and answers [`CircleRoots`]; the first harmonic's root
//! finding lives here, and a degree-two or higher residual's in
//! [`geom_brep::half_angle`], which [`door_half_angle_roots`] and
//! [`CircleRoots::from`] read in the door's vocabulary.
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
//!   which is not a crossing at any order this lane sees and answers
//!   [`CircleRoots::Uncertain`]. The noise meter bounds the error by the
//!   zero band, not by the gap's far edge, so an extreme read bare could
//!   clear `Kε` while its true value sits in the gap.
//! - **root slack** — at either root the extremes' errors move the
//!   residual by `δR = (hi·δlo − lo·δhi)/(hi − lo)` — the NEAR extreme's
//!   error, plus only a share `|near|/(hi − lo)` of the far one's — so
//!   the root moves by `δR / |R′|`, `|R′| = √(−lo·hi)` at both roots;
//!   the phase's error moves it by itself, and the angle arithmetic
//!   rounds by [`geom_brep::HARMONIC_NOISE_ULPS`] half-ulps of a turn. That arc length must
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
//! carrier is a [`CircleRoots::Miss`] when `c − s` is definitely
//! positive or `c + s` definitely negative, [`CircleRoots::OnSurface`]
//! when both are in the zero band, and otherwise `Uncertain`. Read at
//! `c` alone, an in-band spread can cross zero while `c` sits past the
//! escalation threshold whenever `K` is near 1.
//!

use geom_brep::half_angle::{
    HalfAngleFrame, HalfAngleRows, LadderRefusal, MAX_DEGREE, RootSlack, TrigPoly, TrigRoots,
    half_angle_roots,
};
use geom_core::{Band, Decide, Indeterminate, Margin, Sign};

use super::{BooleanDecision, BooleanError};
use crate::validate::decide;

/// What the certified roots of a circle against a surface say about the
/// whole carrier — the one answer every circle root door gives.
#[derive(Debug, Clone, Copy)]
pub(super) enum CircleRoots<T> {
    /// The residual is constant along the carrier and ZERO: the circle
    /// lies on the surface. A constant definite residual is a
    /// [`Self::Miss`].
    OnSurface,
    /// A certified count of zero: the carrier misses the surface.
    Miss,
    /// No certain count — a tangency, a carrier on the surface the door
    /// cannot read as one, a meter that refuses, or no anchor whose pole
    /// is definitely off the surface AND well conditioned.
    Uncertain,
    /// A certified count (even, at most twice the residual's degree) and
    /// the carrier parameters `θ` of those roots, unordered, in
    /// `thetas[..count]`. Every `θ` lies within `π`
    /// of the arc's midpoint, so it compares with the arc `[t₀, t₁]` the
    /// caller passed without wrapping.
    Certified {
        count: usize,
        thetas: [T; 2 * MAX_DEGREE],
    },
    /// A root within the band of a cone's apex, where the surface has no
    /// tangent plane and its quadric form's gradient vanishes: no
    /// crossing there can be read, and the door refuses rather than
    /// answer.
    AtApex,
}

impl<T: geom_core::Real> From<TrigRoots<T>> for CircleRoots<T> {
    fn from(roots: TrigRoots<T>) -> Self {
        match roots {
            TrigRoots::Miss => Self::Miss,
            TrigRoots::Uncertain => Self::Uncertain,
            TrigRoots::Certified { count, thetas } => Self::Certified { count, thetas },
        }
    }
}

/// [`half_angle_roots`] as a door answers it: its roots as
/// [`CircleRoots`], the ladder's in-band sign escalated as the door's
/// `decision`, and a residual past degree two the door's
/// [`BooleanError::ClassificationInvariant`].
///
/// # Errors
///
/// As above.
pub(super) fn door_half_angle_roots<T: Decide>(
    f: &TrigPoly<T>,
    residual: impl Fn(T) -> T,
    frame: HalfAngleFrame<T>,
    rows: &HalfAngleRows,
    slack: &RootSlack<'_, T>,
    decision: BooleanDecision,
    band: Band,
) -> Result<CircleRoots<T>, BooleanError> {
    half_angle_roots(f, residual, frame, rows, slack, band)
        .map(CircleRoots::from)
        .map_err(|refusal| match refusal {
            LadderRefusal::Escalated(diag) => BooleanError::Escalated { decision, diag },
            LadderRefusal::PastDegreeTwo => BooleanError::ClassificationInvariant {
                what: "the half-angle ladder was handed a residual of degree past two",
            },
        })
}

/// The rounding charged against a term bound `terms`
/// ([`geom_brep::rounding_charge`], [`geom_brep::HARMONIC_NOISE_ULPS`]
/// half-ulps of it) — the meters' one spelling of the charge. The
/// conic × quadric door's first-harmonic arm charges its first harmonic
/// the same count, except on a circle against a sphere, whose extremes
/// carry their own running bounds (`geom_brep::CircleSphereHarmonic`).
pub(super) fn rounding_charge<T: geom_core::Real>(terms: T) -> T {
    geom_brep::rounding_charge(terms)
}

/// **The answer for a residual constant along the carrier to within
/// `spread`** about `value` (module docs, "A constant residual"), both in
/// metres, decided under `row`. `spread` must bound the residual's
/// distance from `value` everywhere on the carrier.
///
/// # Errors
///
/// The band's escalation when `value ∓ spread` lies in its gap.
pub(super) fn constant_residual_roots<T: Decide>(
    value: T,
    spread: T,
    row: &'static str,
    band: Band,
) -> Result<FirstHarmonicRoots<T>, Indeterminate> {
    Ok(match span_signs(row, value, spread, band)? {
        (Sign::Positive, _) | (_, Sign::Negative) => FirstHarmonicRoots::Miss,
        (Sign::Zero, Sign::Zero) => FirstHarmonicRoots::OnSurface,
        _ => FirstHarmonicRoots::Uncertain,
    })
}

/// What [`first_harmonic_roots`] and [`constant_residual_roots`] answer:
/// [`CircleRoots`] without the arms only another door reaches — a first
/// harmonic has at most two roots and no apex.
#[derive(Debug, Clone, Copy)]
pub(super) enum FirstHarmonicRoots<T> {
    /// [`CircleRoots::OnSurface`].
    OnSurface,
    /// [`CircleRoots::Miss`].
    Miss,
    /// [`CircleRoots::Uncertain`].
    Uncertain,
    /// Two certified roots, each within `π` of the arc's midpoint
    /// ([`CircleRoots::Certified`]).
    Certified([T; 2]),
}

impl<T: geom_core::Real> From<FirstHarmonicRoots<T>> for CircleRoots<T> {
    fn from(roots: FirstHarmonicRoots<T>) -> Self {
        match roots {
            FirstHarmonicRoots::OnSurface => Self::OnSurface,
            FirstHarmonicRoots::Miss => Self::Miss,
            FirstHarmonicRoots::Uncertain => Self::Uncertain,
            FirstHarmonicRoots::Certified(ts) => {
                let mut thetas = [T::zero(); 2 * MAX_DEGREE];
                thetas[..2].copy_from_slice(&ts);
                Self::Certified { count: 2, thetas }
            }
        }
    }
}

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
/// NOT an error: it answers `Uncertain`, as a definitely excessive one
/// does — a meter that cannot be read does not license the roots it
/// meters. An escalated coaxial test falls through to the extremes,
/// which decide the same carrier.
pub(super) fn first_harmonic_roots<T: Decide>(
    h: &FirstHarmonic<T>,
    speed: T,
    t0: T,
    t1: T,
    rows: &FirstHarmonicRows,
    band: Band,
) -> Result<FirstHarmonicRoots<T>, BooleanError> {
    first_harmonic_roots_in_band(h, speed, t0, t1, rows, band).map_err(|diag| {
        BooleanError::Escalated {
            decision: rows.decision,
            diag,
        }
    })
}

/// [`first_harmonic_roots`] with its one refusal left as the in-band
/// reading itself, for a caller that is not the boolean's and names no
/// [`BooleanDecision`] (`rows.decision` is not read).
///
/// # Errors
///
/// As [`first_harmonic_roots`], as the bare [`Indeterminate`].
pub(super) fn first_harmonic_roots_in_band<T: Decide>(
    h: &FirstHarmonic<T>,
    speed: T,
    t0: T,
    t1: T,
    rows: &FirstHarmonicRows,
    band: Band,
) -> Result<FirstHarmonicRoots<T>, Indeterminate> {
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
        Ok(Sign::Positive) | Err(_) => return Ok(FirstHarmonicRoots::Uncertain),
    }
    let (c0, a1) = ((lo_value + hi_value) / two, (hi_value - lo_value) / two);
    if let Ok(Sign::Zero) = decide(rows.coaxial, Margin::of(a1), band) {
        return constant_residual_roots(c0, a1 + noise, rows.extreme, band);
    }
    let lo = extreme_sign(rows.extreme, lo_value, lo_noise, band)?;
    if lo == Sign::Positive {
        return Ok(FirstHarmonicRoots::Miss);
    }
    let hi = extreme_sign(rows.extreme, hi_value, hi_noise, band)?;
    if hi == Sign::Negative {
        return Ok(FirstHarmonicRoots::Miss);
    }
    if (lo, hi) != (Sign::Negative, Sign::Positive) {
        return Ok(FirstHarmonicRoots::Uncertain);
    }
    // At either root `R = lo·(1 − cos ψ)/2 + hi·(1 + cos ψ)/2` with
    // `(1 + cos ψ)/2 = −lo/(hi − lo)`, so the extremes' errors move the
    // residual there by `(hi·δlo − lo·δhi)/(hi − lo)`, and the phase's
    // moves the root itself; `|R′| = √(−lo·hi)` at both roots.
    let swing = hi_value - lo_value;
    let slope = ((T::zero() - lo_value) * hi_value).max(T::zero()).sqrt();
    let at_root = (hi_value * lo_noise - lo_value * hi_noise) / swing;
    let phase = phase_noise / (cos_part.powi(2) + sin_part.powi(2)).sqrt();
    let slack = speed * (at_root / slope + phase + rounding_charge(T::tau()));
    match decide(rows.root_slack, Margin::of(slack), band) {
        Ok(Sign::Zero | Sign::Negative) => {}
        Ok(Sign::Positive) | Err(_) => return Ok(FirstHarmonicRoots::Uncertain),
    }
    let phi = sin_part.atan2(cos_part);
    // The half-chord `acos(−c₀/A₁)`, measured from the extreme nearer
    // zero (module docs).
    let past = |near: T| two * (near.abs() / swing).sqrt().asin();
    let half_chord = (lo_value + hi_value).select_le_zero(past(hi_value), T::pi() - past(lo_value));
    // The roots on the branch of the span's middle: sound at every scalar
    // by `ring_path::branch`'s window re-centring argument. Its readers
    // read an end of a whole-turn span alike: the wall crossing's
    // `at_end`, the sphere cut-in's vertex lookup at either end, and
    // `sphere_region`'s ray reading `sideless` at either end. None reads
    // a root as an identity, which is why the floor stays: a structural
    // pick would decide nothing a reader uses.
    let mid = (t0 + t1) / two;
    let near_mid = |raw: T| mid + (raw - mid).reduce_periodic_centred(T::tau());
    Ok(FirstHarmonicRoots::Certified([
        near_mid(phi - half_chord),
        near_mid(phi + half_chord),
    ]))
}

/// The signs of both ends of `[value − spread, value + spread]`, each
/// decided under `row`: the one reading of a value known only to within
/// `spread`.
///
/// # Errors
///
/// The band's escalation when either end lies in its gap.
pub(super) fn span_signs<T: Decide>(
    row: &'static str,
    value: T,
    spread: T,
    band: Band,
) -> Result<(Sign, Sign), Indeterminate> {
    let below = decide(row, Margin::of(value - spread), band)?;
    let above = decide(row, Margin::of(value + spread), band)?;
    Ok((below, above))
}

/// The sign of an extreme read as `value` with error at most `noise`
/// ([`span_signs`]): ends that disagree read `Zero`. `noise` must bound
/// the whole error, rounding included, and a caller's charge may hold no
/// slack beyond that (the conic × quadric door's true error reaches
/// 0.99999999 of it), so no term of it can be trimmed.
fn extreme_sign<T: Decide>(
    row: &'static str,
    value: T,
    noise: T,
    band: Band,
) -> Result<Sign, Indeterminate> {
    let (below, above) = span_signs(row, value, noise, band)?;
    Ok(if below == above { below } else { Sign::Zero })
}
