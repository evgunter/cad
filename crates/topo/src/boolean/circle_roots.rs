//! **The circle root cores**: the two root finders every circle × surface
//! door hands its residual to. A door ([`super::circle_sphere`],
//! [`super::circle_cylinder`], [`super::circle_torus`]) computes the
//! residual's harmonics from the geometry, names its rows and its
//! decision, and answers [`CircleRoots`]; the root finding lives here
//! once. Every door has one shape: `(carrier, t0, t1, surface, band)`,
//! a wrong kind is [`BooleanError::ClassificationInvariant`], and an
//! in-band classifying sign escalates as the door's own decision,
//! wrapped here.
//!
//! # The first-harmonic door
//!
//! A residual `R(θ) = c₀ + A₁ cos(θ − φ)` — a circle against a sphere,
//! or against a cylinder wall it is square to — has the EXACT range
//! `[c₀ − A₁, c₀ + A₁]`, and its roots are `θ = φ ± acos(−c₀/A₁)`.
//! [`first_harmonic_roots`] decides on that range, all in residual
//! metres, under the caller's rows:
//!
//! - **noise** — the harmonics' evaluation error ([`NOISE_ULPS`]
//!   half-ulps of the terms' magnitudes, in residual metres) definitely
//!   past the escalation threshold, or not readable at all, refuses: the
//!   representation cannot resolve what the band asks of it. A rounding
//!   estimate on `f64`, run on every scalar: the `Interval` lane carries
//!   its own enclosure and needs no meter to be sound, but the meter
//!   still reads there and can refuse a pose the enclosures alone would
//!   answer.
//! - **coaxial** — the swing `A₁` in the zero band: the residual is
//!   constant to within `A₁` plus the harmonics' `noise`, and
//!   [`constant_residual_roots`] decides it.
//! - **extreme**, on `c₀ − A₁` and `c₀ + A₁` — definitely one-signed is a
//!   miss, definitely straddling is two roots, and either extreme in the
//!   zero band is a tangency, which is not a crossing at any order this
//!   lane sees and answers [`CircleRoots::Uncertain`].
//! - **root slack** — the roots are read off the extremes `lo`, `hi` and
//!   the phase `φ`, each with the error bound its door supplies. At
//!   either root the extremes' errors move the residual by
//!   `δR = (hi·δlo − lo·δhi)/(hi − lo)` — the NEAR extreme's error, plus
//!   only a share `|near|/(hi − lo)` of the far one's — so the root moves
//!   by `δR / |R′|`, `|R′| = √(−lo·hi)` at both roots; the phase's error
//!   moves it by itself, and the angle arithmetic rounds by
//!   [`NOISE_ULPS`] half-ulps of a turn. That arc length must be
//!   definitely inside the band, or the span and trim decisions the
//!   caller makes on the point are made on the wrong point. An
//!   unreadable reading refuses here too. A door whose only account is
//!   `noise` charges it to both extremes and nothing to the phase, and
//!   the slack is then `noise / |R′|`.
//!
//! The half-chord is measured from the extreme nearer zero,
//! `2·asin(√(|near|/(hi − lo)))` past it, which reads the near extreme
//! to its own relative precision: `acos(−c₀/A₁)` near a tangency
//! amplifies the ratio's rounding by `1/√(1 − (c₀/A₁)²)`.
//!
//! Two DISTINCT certified roots therefore certify that the carrier does
//! not lie on the surface — the fact the reduction's `(Zero, Zero)`
//! chord arm leans on to separate a chord from an on-carrier edge.
//!
//! The half-angle ladder below is not used for a first harmonic, for two
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
//! # The half-angle ladder
//!
//! A residual that is a trigonometric polynomial `F(θ)` of degree TWO —
//! a circle against a torus's implicit, or against a cylinder wall it is
//! tilted to — becomes, under the tangent half-angle `t = tan(φ/2)`, the
//! polynomial `F·(1 + t²)²` of degree FOUR in `t`, which
//! [`half_angle_roots`] hands to the ray lane's certified quartic ladder.
//!
//! **Where the pole goes, and why it must be well conditioned.**
//! `φ = θ − θₐ` is measured from an anchor `θₐ`, and the map covers every
//! `θ` except its pole `θₐ + π`. The quartic's leading coefficient is `F`
//! at the pole. The arc's midpoint is tried first, so the pole is the
//! arc's antipode; then anchors a `τ/32` step at a time either side of
//! it, nearest first. A pole may land inside the arc: the one parameter
//! the map cannot reach is then certified not a root by the first
//! decision below, and the roots are reported within `π` of the arc's
//! midpoint whatever the anchor.
//!
//! Non-zero is not enough. A pole definitely off the surface makes the
//! division that normalizes the quartic a division by a non-zero — but a
//! pole a milliradian from a root the arc does not hold puts a root near
//! `t = ∞`, the monic coefficients grow like the inverse of that
//! distance, and at `f64` the ladder then certified wrong counts: in-arc
//! roots dropped, phantoms invented (review of PR 3375, executed at `δ`
//! from 1e-3 to 1e-7). So an anchor is used only when its pole passes
//! TWO decisions:
//!
//! - **pole** — the linearized residual at the pole is definitely
//!   non-zero (it shares `F`'s sign). This is what refuses a carrier
//!   lying ON the surface, whose `F` is identically zero and whose
//!   coefficients are rounding noise that no ratio can be read off.
//! - **conditioning** — `|F(pole)| ≥ κ·A` with
//!   `A = |c₀| + |(c₁, s₁)| + |(c₂, s₂)|`, which bounds `|F|` everywhere
//!   and `|F′|` by `2A`. Every root is then at least `κ/2` radians from
//!   the pole, so none is near `t = ∞`; and each coefficient of
//!   `F·(1 + t²)²` is at most `6A` in size, so the monic coefficients are
//!   at most `6/κ`: the ladder sees a quartic whose coefficients, roots
//!   and rounding are all bounded by a fixed multiple of the circle's
//!   own scale. The margin is metered as the arc length
//!   `ρ·(|F(pole)| − κA)/A` that bound guarantees. `κ = 1/16`.
//!
//! **Some anchor always passes, unless `F ≡ 0`.** By Parseval,
//! `max|F|² ≥ mean F² = c₀² + (A₁² + A₂²)/2`, and by Cauchy–Schwarz
//! `A² = (c₀ + A₁ + A₂)² ≤ (1 + 2 + 2)(c₀² + A₁²/2 + A₂²/2)`, so `|F|`
//! reaches `A/√5 ≈ 0.45A` somewhere, and within `π/32` of there — where
//! some candidate lies — it is still at least `0.45A − 2A·π/32 ≈ 0.25A`,
//! above `κA`. So `Uncertain` from the anchor search means `F` is
//! (numerically) identically zero, or the band could not separate the
//! margins.
//!
//! **Units.** The root variable handed to the ladder is the LENGTH
//! `τ = 2ρ·t`, arc length to first order about the anchor. The ladder's
//! lever is the length its roots spread over, which is where its margins
//! become lengths; each door supplies its own.
//!
//! # The ladder's noise meter
//!
//! `F`'s harmonics are sums of terms much larger than `F` where the
//! crossings live, so at `f64` their rounding is an error in `F` of about
//! `u·T` (`T` a bound on the terms' magnitudes, [`NOISE_ULPS`] of them
//! charged), which is a residual error of `u·T / f_per_metre` metres on
//! the carrier, `f_per_metre` the door's floor on `|F|` per metre of
//! residual. The ladder refuses when that error is DEFINITELY past the
//! band's escalation threshold. The same error moves each root by
//! `error/|F′|` radians; that arc length is held to the same threshold
//! (the root-slack row), or a caller's span and trim decisions would be
//! made on the wrong point.
//!
//! **Its posture on a reading in the band's gap is to pass it**, where
//! the first-harmonic door refuses one. That is a known defect, held open
//! with what refusing would cost
//! (`work/germ/circle-torus-meters-accept-an-unreadable-reading.md`): the
//! term bound is coarse enough that at `ε = 1e-12` ordinary unit-scale
//! poses read in the gap.
//!
//! **What the meter covers, and what it does not.** It bounds ONE stage:
//! the evaluation of the harmonics from the geometry. Everything
//! downstream — the anchor rotation, the division by `F(pole)`, the monic
//! `(2ρ)^k` rescale, the depression and the discriminant — rounds again,
//! and those errors are covered only by the ladder's own band decisions,
//! as they are for the line lane. The conditioning guard keeps that
//! amplification a fixed multiple of the circle's scale, but no bound on
//! it is computed here. So the premise is: **the downstream stages'
//! rounding stays within what the ladder's band margins absorb** — the
//! premise every `f64` ladder in the kernel rests on. What supports it is
//! measurement, not proof (the torus door's module docs say which). The
//! `Interval` lane needs no premise: every stage there is an enclosure.
//!
//! **What the ladder does with a configuration INSIDE the band is not
//! the trilean's escalation.** Deciding on a discriminant rather than on
//! the residual, it can certify a miss for a carrier crossing the
//! surface by less than the zero band, and read an exact tangency as a
//! miss once the carrier is tilted past the band — measured on the
//! cylinder wall, filed on GERM
//! (`work/germ/the-half-angle-ladder-certifies-in-band-configurations.md`).

use geom_core::{Band, Decide, Indeterminate, Margin, Sign};

use super::solid_contain::{QuarticRows, TorusRoots, depressed_quartic_roots};
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
    /// A certified count (2 or 4) and the carrier parameters `θ` of those
    /// roots, unordered, in `thetas[..count]`. Every `θ` lies within `π`
    /// of the arc's midpoint, so it compares with the arc `[t₀, t₁]` the
    /// caller passed without wrapping.
    Certified { count: usize, thetas: [T; 4] },
    /// The quartic's constructed roots disagree in number with its
    /// certified count ([`TorusRoots::CountDisagrees`]).
    CountDisagrees,
}

/// A degree-2 trigonometric polynomial
/// `c₀ + c₁ cos θ + s₁ sin θ + c₂ cos 2θ + s₂ sin 2θ`.
pub(super) struct Harmonics<T> {
    pub(super) c0: T,
    pub(super) c1: T,
    pub(super) s1: T,
    pub(super) c2: T,
    pub(super) s2: T,
}

/// The predicate rows one caller of [`half_angle_roots`] meters under.
pub(super) struct HalfAngleRows {
    /// The pole is definitely off the surface (a residual, metres).
    pub(super) pole: &'static str,
    /// The pole is well conditioned (module docs; metres).
    pub(super) conditioning: &'static str,
    /// The harmonics' rounding, as a residual, is inside the band (the
    /// noise meter; metres).
    pub(super) noise: &'static str,
    /// Each root's position uncertainty, as arc length, is inside the
    /// band (metres).
    pub(super) root_slack: &'static str,
    /// The quartic ladder's own rows.
    pub(super) quartic: QuarticRows,
    /// The decision an in-band ladder sign escalates as.
    pub(super) decision: BooleanDecision,
}

/// Where [`half_angle_roots`] works: the arc `[t0, t1]` of a circle of
/// `radius`, the ladder's `lever` (a length), and the noise meter's two
/// inputs: `noise`, a bound on the `f64` evaluation error of `F` from
/// its harmonics (in `F`'s units), and `f_per_metre`, `F`'s units per
/// metre of the residual `residual` reads — a floor on
/// `|F| / |residual|` near the surface, which turns `noise` into
/// metres. A door whose `F` IS that residual (its harmonics already in
/// metres) passes exactly `1`.
pub(super) struct HalfAngleFrame<T> {
    pub(super) t0: T,
    pub(super) t1: T,
    pub(super) radius: T,
    pub(super) lever: T,
    pub(super) noise: T,
    pub(super) f_per_metre: T,
}

/// How many units in the last place of the term bound the harmonics'
/// evaluation error is charged (module docs, "The ladder's noise meter"). Each
/// harmonic is a short chain from the inputs — a squared norm, a
/// product, a sum of four terms — whose every rounding is half an ulp
/// of a quantity the term bound dominates; sixteen is that chain's
/// count with room. It is a ROUNDING estimate, the `f64` lane's
/// contract, not an enclosure: the `Interval` lane carries the
/// enclosure itself through every coefficient and the ladder decides on
/// it, so it needs no meter to be sound. The first-harmonic door charges the same count: its
/// chains are shorter, so the count holds there with more room.
pub(super) const NOISE_ULPS: f64 = 16.0;

/// The rounding charged against a term bound `terms`: [`NOISE_ULPS`]
/// half-ulps of it — the meters' one spelling of the charge.
pub(super) fn rounding_charge<T: geom_core::Real>(terms: T) -> T {
    T::from_f64(NOISE_ULPS * geom_core::UNIT_ROUNDOFF) * terms
}

/// The conditioning floor `κ` (module docs): the pole's `|F|` must be at
/// least this share of `F`'s amplitude bound.
const POLE_CONDITIONING: f64 = 1.0 / 16.0;

/// How many evenly spaced poles round the carrier the anchor search may
/// try (module docs: 32 is what makes some candidate always pass
/// [`POLE_CONDITIONING`]).
const POLE_CANDIDATES: u32 = 32;

/// **The certified real roots of a degree-2 trigonometric polynomial
/// `F(θ)` along a circle, by the tangent half-angle** — a quartic in
/// `t = tan((θ − θₐ)/2)`, solved by the ray lane's certified ladder.
/// General over the surface: `residual` is the surface's linearized
/// residual (metres) along the carrier, which must share `F`'s sign.
///
/// An anchor `θₐ` is USED only when its pole `θₐ + π` passes two
/// decisions (module docs, "The half-angle ladder"): the residual there is
/// definitely non-zero, and `|F(pole)|` clears `κ·A` with
/// `A = |c₀| + |(c₁, s₁)| + |(c₂, s₂)|`, metered as the arc length
/// `ρ·(|F(pole)| − κA)/A` that bound guarantees between the pole and
/// every root. The antipode of the arc is tried first and poles off the
/// arc are preferred, but a pole may land inside it (certified not a
/// root); if no candidate passes the answer is `Uncertain`.
///
/// Before any of that, the noise meter (module docs, "The ladder's noise meter")
/// must put the harmonics' rounding inside the band, and after it each
/// root's position uncertainty must be inside the band too.
pub(super) fn half_angle_roots<T: Decide>(
    f: &Harmonics<T>,
    residual: impl Fn(T) -> T,
    frame: HalfAngleFrame<T>,
    rows: &HalfAngleRows,
    band: Band,
) -> Result<CircleRoots<T>, BooleanError> {
    let HalfAngleFrame {
        t0,
        t1,
        radius,
        lever,
        noise,
        f_per_metre,
    } = frame;
    // **The noise meter** (module docs, "The ladder's noise meter"): it bounds the harmonics'
    // evaluation error, `noise`, a residual error of up to
    // `noise / f_per_metre` metres everywhere on the carrier, and refuses
    // when that is definitely past the band's escalation threshold. The
    // stages after it (rotation, pole division, rescale, depression,
    // discriminant) round again; those are left to the ladder's own band
    // decisions, which is the premise the module docs state.
    match decide(rows.noise, Margin::of(noise / f_per_metre), band) {
        Ok(Sign::Positive) => return Ok(CircleRoots::Uncertain),
        Ok(Sign::Zero | Sign::Negative) | Err(_) => {}
    }
    let two = T::from_f64(2.0);
    let four = T::from_f64(4.0);
    let six = T::from_f64(6.0);
    let hypot = |x: T, y: T| (x.powi(2) + y.powi(2)).sqrt();
    let amplitude = f.c0.abs() + hypot(f.c1, f.s1) + hypot(f.c2, f.s2);
    let mid = (t0 + t1) / two;
    let scale = two * radius;
    // The candidate anchors: the arc's midpoint first (its pole is the
    // antipode), then every `τ/32` step either side of it, nearest
    // first — so a pole off the arc is preferred and a pole inside it
    // (certified not a root by the residual decision) is used only when
    // none off it is well conditioned. Thirty-two steps are what make
    // the list complete: some candidate always passes the conditioning
    // floor unless `F` is identically zero (module docs).
    let step = T::tau() / T::from_f64(f64::from(POLE_CANDIDATES));
    let shifts = (0..=POLE_CANDIDATES / 2).flat_map(|k| {
        let k = T::from_f64(f64::from(k));
        [step * k, T::zero() - step * k]
    });
    // `k = 0` yields the zero shift twice; the second copy is dropped.
    for shift in shifts.skip(1) {
        let anchor = mid + shift;
        match decide(rows.pole, Margin::of(residual(anchor + T::pi())), band) {
            Ok(Sign::Positive | Sign::Negative) => {}
            Ok(Sign::Zero) | Err(_) => continue,
        }
        // The harmonics about the anchor: the first rotates by `θₐ`, the
        // second by `2θₐ`.
        let (sa, ca) = anchor.sin_cos();
        let (s2a, c2a) = (two * anchor).sin_cos();
        let c1 = f.c1 * ca + f.s1 * sa;
        let s1 = f.s1 * ca - f.c1 * sa;
        let c2 = f.c2 * c2a + f.s2 * s2a;
        let s2 = f.s2 * c2a - f.c2 * s2a;
        // `F·(1 + t²)²` in `t = tan(φ/2)`, low coefficient first.
        let a = [
            f.c0 + c1 + c2,
            two * s1 + four * s2,
            two * f.c0 - six * c2,
            two * s1 - four * s2,
            f.c0 - c1 + c2,
        ];
        let lead = a[4];
        match decide(
            rows.conditioning,
            Margin::over_lever(
                radius * (lead.abs() - T::from_f64(POLE_CONDITIONING) * amplitude),
                amplitude,
            ),
            band,
        ) {
            Ok(Sign::Positive) => {}
            Ok(Sign::Zero | Sign::Negative) | Err(_) => continue,
        }
        // Monic in `τ = 2ρ·t` (a length): `τ⁴ + B τ³ + C τ² + D τ + E`.
        let b3 = scale * a[3] / lead;
        let c2m = scale.powi(2) * a[2] / lead;
        let d1 = scale.powi(3) * a[1] / lead;
        let e0 = scale.powi(4) * a[0] / lead;
        // Depress by `τ = y − B/4`.
        let shift_y = b3 / four;
        let p = c2m - T::from_f64(3.0) * b3.powi(2) / T::from_f64(8.0);
        let q = d1 - b3 * c2m / two + b3.powi(3) / T::from_f64(8.0);
        let s = e0 - b3 * d1 / four + b3.powi(2) * c2m / T::from_f64(16.0)
            - T::from_f64(3.0) * b3.powi(4) / T::from_f64(256.0);
        return Ok(
            match depressed_quartic_roots(p, q, s, lever, &rows.quartic, band).map_err(|diag| {
                BooleanError::Escalated {
                    decision: rows.decision,
                    diag,
                }
            })? {
                TorusRoots::Miss => CircleRoots::Miss,
                TorusRoots::Uncertain => CircleRoots::Uncertain,
                TorusRoots::CountDisagrees => CircleRoots::CountDisagrees,
                TorusRoots::Certified { count, ts: ys } => {
                    let mut thetas = [T::zero(); 4];
                    for (theta, y) in thetas.iter_mut().zip(ys).take(count) {
                        // Reported within `π` of the arc's midpoint, so a
                        // caller compares it with `[t0, t1]` directly.
                        let raw = anchor + two * ((y - shift_y) / scale).atan();
                        *theta = mid + (raw - mid).reduce_periodic_centred(T::tau());
                        // **Where the root is, not just that it is.** The
                        // true root lies within `noise / |F′|` radians of
                        // the computed one; that arc length must be inside
                        // the band, or the span and trim decisions a
                        // caller makes on the point are on the wrong point.
                        let (s1t, c1t) = theta.sin_cos();
                        let (s2t, c2t) = (two * *theta).sin_cos();
                        let slope = f.s1 * c1t - f.c1 * s1t + two * (f.s2 * c2t - f.c2 * s2t);
                        match decide(
                            rows.root_slack,
                            Margin::of(radius * noise / slope.abs()),
                            band,
                        ) {
                            Ok(Sign::Positive) => return Ok(CircleRoots::Uncertain),
                            Ok(Sign::Zero | Sign::Negative) | Err(_) => {}
                        }
                    }
                    CircleRoots::Certified { count, thetas }
                }
            },
        );
    }
    Ok(CircleRoots::Uncertain)
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
) -> Result<CircleRoots<T>, Indeterminate> {
    let lo = decide(row, Margin::of(value - spread), band)?;
    let hi = decide(row, Margin::of(value + spread), band)?;
    Ok(match (lo, hi) {
        (Sign::Positive, _) | (_, Sign::Negative) => CircleRoots::Miss,
        (Sign::Zero, Sign::Zero) => CircleRoots::OnSurface,
        _ => CircleRoots::Uncertain,
    })
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

/// **The certified roots of a first-harmonic residual along a circle of
/// `radius`**, reported within `π` of the midpoint of `[t0, t1]`: the
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
    radius: T,
    t0: T,
    t1: T,
    rows: &FirstHarmonicRows,
    band: Band,
) -> Result<CircleRoots<T>, BooleanError> {
    let decide = |row, m, band| {
        decide(row, m, band).map_err(|diag| BooleanError::Escalated {
            decision: rows.decision,
            diag,
        })
    };
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
        Ok(Sign::Positive) | Err(_) => return Ok(CircleRoots::Uncertain),
    }
    let (c0, a1) = ((lo_value + hi_value) / two, (hi_value - lo_value) / two);
    if let Ok(Sign::Zero) = decide(rows.coaxial, Margin::of(a1), band) {
        return constant_residual_roots(c0, a1 + noise, rows.extreme, band).map_err(|diag| {
            BooleanError::Escalated {
                decision: rows.decision,
                diag,
            }
        });
    }
    let lo = decide(rows.extreme, Margin::of(lo_value), band)?;
    if lo == Sign::Positive {
        return Ok(CircleRoots::Miss);
    }
    let hi = decide(rows.extreme, Margin::of(hi_value), band)?;
    if hi == Sign::Negative {
        return Ok(CircleRoots::Miss);
    }
    if (lo, hi) != (Sign::Negative, Sign::Positive) {
        return Ok(CircleRoots::Uncertain);
    }
    // At either root `R = lo·(1 − cos ψ)/2 + hi·(1 + cos ψ)/2` with
    // `(1 + cos ψ)/2 = −lo/(hi − lo)`, so the extremes' errors move the
    // residual there by `(hi·δlo − lo·δhi)/(hi − lo)`, and the phase's
    // moves the root itself; `|R′| = √(−lo·hi)` at both roots.
    let swing = hi_value - lo_value;
    let slope = ((T::zero() - lo_value) * hi_value).max(T::zero()).sqrt();
    let at_root = (hi_value * lo_noise - lo_value * hi_noise) / swing;
    let phase = phase_noise / (cos_part.powi(2) + sin_part.powi(2)).sqrt();
    let slack = radius * (at_root / slope + phase + rounding_charge(T::tau()));
    match decide(rows.root_slack, Margin::of(slack), band) {
        Ok(Sign::Zero | Sign::Negative) => {}
        Ok(Sign::Positive) | Err(_) => return Ok(CircleRoots::Uncertain),
    }
    let phi = sin_part.atan2(cos_part);
    // The half-chord `acos(−c₀/A₁)`, measured from the extreme nearer
    // zero (module docs).
    let past = |near: T| two * (near.abs() / swing).sqrt().asin();
    let half_chord = (lo_value + hi_value).select_le_zero(past(hi_value), T::pi() - past(lo_value));
    let mid = (t0 + t1) / two;
    let near_mid = |raw: T| mid + (raw - mid).reduce_periodic_centred(T::tau());
    Ok(CircleRoots::Certified {
        count: 2,
        thetas: [
            near_mid(phi - half_chord),
            near_mid(phi + half_chord),
            T::zero(),
            T::zero(),
        ],
    })
}
