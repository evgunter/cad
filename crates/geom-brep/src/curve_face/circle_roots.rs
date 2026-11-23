//! **The circle root cores**: the two root finders every circle × surface
//! door hands its residual to. A door ([`super::conic_quadric`],
//! [`super::circle_torus`], [`super::ellipse_torus`]) computes the
//! residual's harmonics from the geometry, names its rows, and answers
//! [`CircleRoots`]; the first harmonic's root finding lives here, and a
//! degree-two or higher residual's in [`crate::half_angle`], read past
//! what it refuses, and the wrappers read a method's answer in the door's
//! vocabulary ([`door_shape`]). Every door has one shape: `(carrier, t0, t1, surface,
//! band)`. A torus door refuses in [`DoorRefusal`]: a wrong kind is
//! [`DoorRefusal::Invariant`], and an in-band classifying sign is
//! [`DoorRefusal::Escalated`]. The conic × quadric door refuses in its
//! own [`super::conic_quadric::ConicQuadricRefusal`], naming the
//! surface's arm. The caller names the decision an escalation is.
//!
//! # The first-harmonic core
//!
//! A residual `R(θ) = c₀ + A₁ cos(θ − φ)` — a circle against a sphere,
//! or against a cylinder wall it is square to — has the EXACT range
//! `[lo, hi] = [c₀ − A₁, c₀ + A₁]`, and its roots are
//! `θ = φ ± acos(−c₀/A₁)`. Its class is decided on that range by the
//! pose (`super::pose`, the zero lane's rungs, all in residual metres):
//!
//! - **noise** (`curve_face_pose_noise`) — the larger of the extremes'
//!   error bounds definitely past the escalation threshold, or not
//!   readable at all, refuses: every decision below reads the extremes,
//!   and the representation cannot resolve what the band asks of them.
//! - **coaxial** (`curve_face_pose_coaxial`) — the swing `A₁` in the zero
//!   band: the residual is `c₀` to within `A₁` plus that error, a
//!   constant whose ends (`curve_face_pose_constant`) definitely
//!   one-signed are apart, both in the zero band lie on the surface, and
//!   otherwise fall through to the extremes. Read at `c₀` alone, an
//!   in-band spread can cross zero while `c₀` sits past the escalation
//!   threshold whenever `K` is near 1.
//! - **extreme** (`curve_face_pose_extreme`), on `lo` and `hi`, each read
//!   as the interval its error bound spans — definitely one-signed is
//!   apart, definitely straddling is two roots, an end in the gap
//!   escalates, and an extreme in the zero band is a graze, whose
//!   curvature the pose reads. The noise meter bounds the error by the
//!   zero band, not by the gap's far edge, so an extreme read bare could
//!   clear `Kε` while its true value sits in the gap.
//!
//! The roots of a crossing the pose read are this core's
//! ([`harmonic_roots_as`]):
//!
//! - **root slack** — at either root the extremes' errors move the
//!   residual by `δR = (hi·δlo − lo·δhi)/(hi − lo)` — the NEAR extreme's
//!   error, plus only a share `|near|/(hi − lo)` of the far one's — so
//!   the root moves by `δR / |R′|`, `|R′| = √(−lo·hi)` at both roots;
//!   the phase's error moves it by itself, and the angle arithmetic
//!   rounds by [`geom_core::SHORT_CHAIN_ULPS`] half-ulps of a turn. That
//!   arc length must be definitely inside the band, or the span and trim
//!   decisions the caller makes on the point are made on the wrong
//!   point. An unreadable reading refuses here too. A door whose only
//!   account is one uniform `noise` charges it to both extremes and
//!   nothing to the phase, and the slack is then `noise / |R′|` plus the
//!   angle charge.
//!
//! The half-chord is measured from the extreme nearer zero,
//! `2·asin(√(|near|/(hi − lo)))` past it, which reads the near extreme
//! to its own relative precision: `acos(−c₀/A₁)`, or the same `asin`
//! read from the far extreme, near a tangency amplifies its argument's
//! rounding by `1/√(1 − (c₀/A₁)²)`.
//!
//! The half-angle ladder ([`crate::half_angle`]) is not used for a
//! first harmonic, for two reasons that are properties of the ladder
//! rather than of its cost. A CONSTANT residual (a coaxial carrier) makes
//! the quartic `F·(1 + t²)²`, a repeated complex pair the ladder cannot
//! answer, where the extremes say on-surface or miss. And a TANGENCY must
//! escalate: the extremes put it in the zero band, where the ladder,
//! deciding on a discriminant rather than on the range, can certify a
//! miss (`work/germ/the-half-angle-ladder-certifies-in-band-configurations.md`).

use crate::half_angle::MAX_DEGREE;
use crate::roots::{OnCarrier, Roots, Unsettled};
use geom_core::period::Period;
use geom_core::{Band, Decide, Margin, Rounded, Sign};

use geom_core::Indeterminate;
use geom_core::k_stats::decide;

/// **Why a conic root door refused**, in the door's own words: the
/// caller names the decision an escalation is. `X` is what a torus door
/// refuses its torus with ([`Self::Torus`]): a door that reads no torus
/// leaves it [`core::convert::Infallible`], so the arm cannot be built, and
/// a reader of that door's refusal need not, and cannot, end a torus.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DoorRefusal<X = core::convert::Infallible> {
    /// A classifying sign the door needed, or a touch it read, fell in
    /// the band's escalation gap.
    Escalated(Indeterminate),
    /// The door's readings contradict, or it was handed a carrier or a
    /// surface of a kind it does not read: a kernel defect.
    Invariant(&'static str),
    /// **A torus outside the ring convention**, the convention's refusal
    /// carried whole ([`crate::TorusRefusal`]), as the plane × torus
    /// section carries it. A torus door reads only a ring: its harmonics
    /// take the convention's witness ([`crate::conic_torus_harmonics`]),
    /// and the door asks for it before any pose. The standing is the
    /// convention's own ([`crate::TorusRefusal::standing`]): an undecided
    /// half is ranked by its margin, and a torus decided outside it is
    /// invalid input.
    Torus(X),
}

impl<X> DoorRefusal<X> {
    /// **This refusal with its torus arm split off**: the torus's own
    /// refusal, or the refusal of a door that reads no torus, which every
    /// other arm is. A reader of a torus door ends the first its own way
    /// and hands the second to the reader of every door.
    ///
    /// # Errors
    ///
    /// The torus arm's `X`.
    pub fn without_torus(self) -> Result<DoorRefusal, X> {
        match self {
            Self::Escalated(diag) => Ok(DoorRefusal::Escalated(diag)),
            Self::Invariant(what) => Ok(DoorRefusal::Invariant(what)),
            Self::Torus(torus) => Err(torus),
        }
    }
}

/// What the certified roots of a circle against a surface say about the
/// whole carrier — the one answer every circle root door gives, in the
/// root finders' one shape ([`crate::roots`]): a miss, the carrier ON
/// the surface (its residual a zero constant; a constant definite
/// residual is a miss), or a certified count (even, at most twice the
/// residual's degree) of carrier parameters `θ`, each within `π` of the
/// arc's midpoint ([`Period::near`]), so it
/// compares with the arc `[t₀, t₁]` the caller passed without wrapping. A door that certifies none of these refuses
/// with an [`Unsettled`] beside its [`DoorRefusal`].
pub type CircleRoots<T> = Roots<T, { 2 * MAX_DEGREE }, OnCarrier>;

/// **A method's answer in the root doors' shape** (their wrappers): its
/// roots where no window meets the span, else the first window that does
/// ([`super::cross::Walked::on_span`]), refused as the door's escalation.
/// A window off the arc refuses nothing.
///
/// # Errors
///
/// [`DoorRefusal::Escalated`] for a sign in the band's gap, and
/// [`DoorRefusal::Invariant`] for a fault or a count past the degree.
pub(super) fn door_shape<T: Decide, X>(
    span: &crate::curve_pair::Span<'_, T>,
    walked: &super::cross::Walked<T>,
    band: Band,
) -> Result<Result<CircleRoots<T>, Unsettled>, DoorRefusal<X>> {
    use crate::curve_pair::MeetFault;
    match walked.on_span(span, band).first() {
        None => Roots::of(&walked.roots)
            .map(Ok)
            .ok_or(DoorRefusal::Invariant(
                "a method read more roots than its degree allows",
            )),
        Some(&(_, fault)) => match fault {
            MeetFault::Escalated(diag) | MeetFault::Touch(diag) => {
                Err(DoorRefusal::Escalated(diag))
            }
            MeetFault::Unsettled(why) => Ok(Err(why)),
            MeetFault::Contradiction { readings } => Err(DoorRefusal::Invariant(readings)),
            MeetFault::Desync | MeetFault::Unwalked(_) | MeetFault::Unsupported { .. } => Err(
                DoorRefusal::Invariant("a root door's method answered a window of no root door"),
            ),
        },
    }
}

/// What [`harmonic_roots_as`] answers: [`CircleRoots`]' shape with room for
/// two roots, the most a first harmonic has.
pub type FirstHarmonicRoots<T> = Roots<T, 2, OnCarrier>;

/// A residual `c₀ + A₁ cos(θ − φ)` along a circle, `φ = atan2(sin_part, cos_part)`,
/// in metres of residual, given by its extremes `lo = c₀ − A₁` and
/// `hi = c₀ + A₁` — the one spelling of the range the door decides on,
/// `c₀` and `A₁` read off it — and the phase's two components.
///
/// Each comes with a bound on its error as the door evaluated it:
/// the extremes' in metres of residual (their rounding, and
/// any term the caller dropped to reach this form, which must bound the
/// residual's error at every `θ` where it is not specific to one
/// extreme), and `phase_noise` in the units of `(cos_part, sin_part)`.
/// A door whose only account is one uniform `noise` passes it as both
/// extremes' and no phase charge: a residual error bounded at every `θ`
/// already moves the roots by no more than that over `|R′|`, the
/// phase's share included.
#[derive(Clone, Copy, Debug)]
pub struct FirstHarmonic<T> {
    /// The least residual, `c₀ − A₁`, with the bound on its error.
    pub lo: Rounded<T>,
    /// The greatest residual, `c₀ + A₁`, with the bound on its error.
    pub hi: Rounded<T>,
    /// The phase's cosine component.
    pub cos_part: T,
    /// The phase's sine component.
    pub sin_part: T,
    /// The bound on the phase components' error, in their units.
    pub phase_noise: T,
}

impl<T: Decide> FirstHarmonic<T> {
    /// An upper bound on `|R|` over the turn: the larger extreme's
    /// magnitude, its bound included.
    pub fn most(&self) -> T {
        self.lo.magnitude().max(self.hi.magnitude())
    }
}

/// **The two roots of a first harmonic the pose read crossing**
/// (`super::pose`: its extremes definitely on either side of zero, its
/// noise metered), along a carrier whose speed `|C′(θ)|` is at most
/// `speed`, reported within `π` of the midpoint of `(t0, t1)`
/// ([`Period::near`]): the root slack (module docs, "root slack"),
/// decided under `root_slack`, then the half-chord from the extreme
/// nearer zero.
///
/// # Errors
///
/// [`SlackRefused`] where the root slack is not definitely inside the
/// band, or not readable: the two roots, and the slack each lies within.
pub fn harmonic_roots_as<T: Decide>(
    h: &FirstHarmonic<T>,
    speed: T,
    (t0, t1): (T, T),
    root_slack: &'static str,
    band: Band,
) -> Result<FirstHarmonicRoots<T>, SlackRefused<T>> {
    let FirstHarmonic {
        lo,
        hi,
        cos_part,
        sin_part,
        phase_noise,
    } = *h;
    let (lo_value, hi_value) = (lo.value, hi.value);
    // At either root `R = lo·(1 − cos ψ)/2 + hi·(1 + cos ψ)/2` with
    // `(1 + cos ψ)/2 = −lo/(hi − lo)`, so the extremes' errors move the
    // residual there by `(hi·δlo − lo·δhi)/(hi − lo)`, and the phase's
    // moves the root itself; `|R′| = √(−lo·hi)` at both roots.
    let swing = hi_value - lo_value;
    let slope = ((T::zero() - lo_value) * hi_value).max(T::zero()).sqrt();
    let at_root = (hi_value * lo.error - lo_value * hi.error) / swing;
    let phase = phase_noise / (cos_part.powi(2) + sin_part.powi(2)).sqrt();
    let angle = at_root / slope + phase + geom_core::rounding_charge(T::tau());
    let held = matches!(
        decide(root_slack, Margin::of(speed * angle), band),
        Ok(Sign::Zero | Sign::Negative)
    );
    // The half-chord `acos(−c₀/A₁)`, measured from the extreme nearer
    // zero (module docs).
    let past = |near: T| crate::roots::arc_past(near.abs() / swing);
    let half_chord = (lo_value + hi_value).select_le_zero(past(hi_value), T::pi() - past(lo_value));
    // The roots within half a turn of the span's middle, by
    // `Period::near`'s span argument. Its readers read an end of a
    // whole-turn span alike.
    let middle = geom::mid_param(t0, t1);
    let phi = geom::Azimuth::of(sin_part, cos_part).near(middle);
    let (a, b) = (phi - half_chord, phi + half_chord);
    let turn = Period::angle();
    let roots = [turn.near(a, middle), turn.near(b, middle)];
    if held {
        Ok(Roots::all(roots))
    } else {
        Err(SlackRefused {
            row: root_slack,
            roots,
            slack: angle,
        })
    }
}

/// **A first harmonic's roots whose slack the meter refused**
/// ([`harmonic_roots_as`]): each root, and the slack in the carrier's
/// parameter that the true root lies within.
#[derive(Clone, Copy, Debug)]
pub struct SlackRefused<T> {
    /// The meter's row.
    pub row: &'static str,
    /// The two roots, as placed.
    pub roots: [T; 2],
    /// The slack about each.
    pub slack: T,
}

/// A root door's answer for a row: its roots, or its refusal, a limit
/// (`Some`) or `None` for its decision's escalation.
///
/// # Panics
///
/// On any other refusal.
#[cfg(test)]
#[allow(clippy::panic)]
pub fn door_answer<T: geom_core::Real, X: core::fmt::Debug>(
    got: Result<Result<CircleRoots<T>, Unsettled>, DoorRefusal<X>>,
) -> Result<CircleRoots<T>, Option<Unsettled>> {
    match got {
        Ok(found) => found.map_err(Some),
        Err(DoorRefusal::Escalated(_)) => Err(None),
        Err(other) => panic!("the door refused outside its decision: {other:?}"),
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    /// **The most residual charges the larger extreme's bound.** `−0.5`
    /// within `0.25` and `0.6` within `0.1`: the most is `0.75`, the
    /// lower extreme's magnitude, which its value alone would put below
    /// the upper's.
    #[test]
    fn the_most_residual_charges_the_extremes_bound() {
        let h = FirstHarmonic {
            lo: Rounded {
                value: -0.5,
                error: 0.25,
            },
            hi: Rounded {
                value: 0.6,
                error: 0.1,
            },
            cos_part: 1.0,
            sin_part: 0.0,
            phase_noise: 0.0,
        };
        assert_eq!(h.most(), 0.75, "the larger magnitude, bound included");
    }
}
