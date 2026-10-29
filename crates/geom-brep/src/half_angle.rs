//! **The half-angle ladder, and the subdivision that answers**: the
//! certified real roots, round a whole turn, of a residual along a conic
//! that is a trigonometric polynomial in the conic's own angle. The
//! caller (a "door") computes the residual's harmonics and names their
//! rows; what this answers is a value or a typed refusal
//! ([`RootRefusal`]), and the caller's own decision and error
//! vocabulary wrap it.
//!
//! A residual that is a trigonometric polynomial `F(θ)` of degree TWO —
//! a circle against a torus's implicit, or against a cylinder wall it is
//! tilted to, an ellipse against a sphere or a wall — has at most four
//! roots round the turn. [`half_angle_roots`] answers it with
//! [`certified_subdivision`], decided on the residual itself in the
//! band's own metres (its docs: Taylor bounds on pieces, each piece
//! clear or monotone, each root bisected and read ON the surface, a
//! tangency refused typed, [`crate::roots::RootRefusal`]).
//!
//! The subdivision is not bound to degree two: a residual of degree `N`
//! (at most [`MAX_DEGREE`]) has at most `2N` roots, and its Taylor and
//! noise bounds read every harmonic up to `N`. An ellipse against a
//! torus's implicit is of degree four, an octic in the half-angle with no
//! ladder, and it goes to the subdivision alone with a root-slack meter
//! ([`RootSlack`]).
//!
//! **The ladder runs first, for its escalations only.** Under the
//! tangent half-angle `t = tan(φ/2)`, `F·(1 + t²)²` is a quartic in `t`,
//! which the certified quartic ladder ([`crate::quartic`]) decides; an in-band
//! sign it meets escalates ([`RootRefusal::Escalated`]), and that `Err` is
//! all [`half_angle_roots`] keeps. Its answers are dropped: they are
//! decided in its root variable `τ`, arc length only to first order
//! about the anchor (and along an ellipse only up to the ratio of its
//! semi-axes), and measured, it placed roots fifteen zero bands off the
//! wall, certified a `Miss` for definite crossings and for grazes inside
//! the band, and answered `CountDisagrees` across definite crossings
//! (`work/germ/the-half-angle-ladder-certifies-in-band-configurations.md`).
//! That its escalations are still read in that metric is filed
//! (`work/hone/half-angle-ladder-escalates-in-its-own-metric.md`).
//!
//! **The pole, and why it must be well conditioned.** `φ = θ − θₐ` is
//! measured from an anchor `θₐ`, and the map covers every `θ` except its
//! pole `θₐ + π`; the quartic's leading coefficient is `F` there. A pole
//! near a root puts a root near `t = ∞` and the monic coefficients grow
//! like the inverse of that distance (review of PR 3375), which would
//! make the ladder's escalations spurious. So an anchor is used only
//! when its pole passes two decisions:
//!
//! - **pole** — the residual there is definitely non-zero (it shares
//!   `F`'s sign);
//! - **conditioning** — `|F(pole)| ≥ κ·A` with
//!   `A = |c₀| + |(c₁, s₁)| + |(c₂, s₂)|`, which bounds `|F|` everywhere
//!   and `|F′|` by `2A`: every root is then at least `κ/2` radians from
//!   the pole, and the monic coefficients are at most `6/κ`. Metered as
//!   the arc length `ρ·(|F(pole)| − κA)/A`, `ρ` the carrier's least
//!   speed; `κ = 1/16`.
//!
//! Some anchor among 32 evenly spaced ones always passes unless `F ≡ 0`:
//! by Parseval and Cauchy–Schwarz `|F|` reaches `A/√5 ≈ 0.45A`
//! somewhere, and within `π/32` of there it is still at least
//! `0.45A − 2A·π/32 ≈ 0.25A > κA`.
//!
//! **Units.** The ladder's root variable is the LENGTH `τ = 2ρ·t`; its
//! lever, the length its roots spread over, is each door's own.
//!
//! # The harmonics' noise
//!
//! `F`'s harmonics are sums of terms much larger than `F` where the
//! crossings live, so at `f64` their rounding is an error in `F` of about
//! `u·T` (`T` a bound on the terms' magnitudes, [`geom_core::SHORT_CHAIN_ULPS`] of them
//! charged): `noise`, a residual error of `noise / f_per_metre` metres,
//! `f_per_metre` the door's floor on `|F|` per metre of residual. The
//! subdivision charges it to every Taylor term it reads (its docs), so a
//! pose whose noise is past the band refuses there, unless the door
//! hands it a jet ([`SubdivisionFrame`]): `F` and its derivatives read
//! at the carrier point, each charged its own rounding, with `noise` left
//! only in the one global bound on `|F⁗|`. A
//! rounding estimate on `f64`, run on every scalar; the `Interval` lane
//! carries its own enclosure.
//!
//! The ladder meters the same noise before it runs: a reading definitely
//! past the band keeps it from running, and any other — one in the band's
//! gap, or none — lets it run, because all it can add is an escalation.
//! The rounding of its own later stages (the anchor rotation, the pole
//! division, the rescale, the depression) is left to its band decisions.

use geom_core::bisect::{Step, bisect};
use geom_core::k_stats::decide;
use geom_core::{Band, Decide, Indeterminate, Margin, Sign};

use crate::quartic::{QuarticRows, depressed_quartic_roots};
use crate::roots::{RootFault, RootRefusal, Roots, Unsettled};

/// The most harmonics [`certified_subdivision`] reads: a conic against a
/// torus ([`crate::ConicTorusHarmonics`]).
pub const MAX_DEGREE: usize = 4;

/// What [`certified_subdivision`] (and so [`half_angle_roots`]) answers
/// about a residual round the whole turn: [`Roots::Miss`], or a
/// certified count (even, at most twice the residual's degree) of
/// carrier parameters `θ`, each within `π` of the arc's midpoint. A
/// trigonometric residual is never read as a zero constant here, so it
/// has no [`Roots::On`] arm.
pub type TrigRoots<T> = Roots<T, { 2 * MAX_DEGREE }>;

/// A trigonometric polynomial `Σₖ cos[k]·cos kθ + sin[k]·sin kθ` of
/// degree `degree ≤ MAX_DEGREE`; coefficients past `degree`, and
/// `sin[0]`, are not read.
pub struct TrigPoly<T> {
    /// The cosine coefficients, by harmonic.
    pub cos: [T; MAX_DEGREE + 1],
    /// The sine coefficients, by harmonic.
    pub sin: [T; MAX_DEGREE + 1],
    /// The highest harmonic read.
    pub degree: u8,
}

/// A trigonometric polynomial of degree two,
/// `c₀ + c₁ cos θ + s₁ sin θ + c₂ cos 2θ + s₂ sin 2θ`: what
/// [`half_angle_roots`] takes, since its quartic in the tangent half-angle
/// describes no higher degree.
#[derive(Clone, Copy)]
pub struct DegreeTwo<T> {
    /// The constant.
    pub c0: T,
    /// The first harmonic's cosine coefficient.
    pub c1: T,
    /// The first harmonic's sine coefficient.
    pub s1: T,
    /// The second harmonic's cosine coefficient.
    pub c2: T,
    /// The second harmonic's sine coefficient.
    pub s2: T,
}

impl<T: geom_core::Real> DegreeTwo<T> {
    /// `c₀ + c₁ cos θ + s₁ sin θ + c₂ cos 2θ + s₂ sin 2θ`.
    pub fn new(c0: T, c1: T, s1: T, c2: T, s2: T) -> Self {
        Self { c0, c1, s1, c2, s2 }
    }

    /// The same polynomial as a [`TrigPoly`].
    #[must_use]
    pub fn poly(&self) -> TrigPoly<T> {
        let z = T::zero();
        TrigPoly {
            cos: [self.c0, self.c1, self.c2, z, z],
            sin: [z, self.s1, self.s2, z, z],
            degree: 2,
        }
    }
}

/// Where [`certified_subdivision`] works: the arc `[t0, t1]` its roots
/// are reported about, the carrier's top speed `|C′|` (metres per
/// radian), `noise`, a bound on the `f64` evaluation error of `F` from
/// its harmonics (in `F`'s units), and `F`'s units per metre of the
/// residual `residual` reads: `f_per_metre`, a FLOOR on
/// `|F| / |residual|` (which turns `noise` into metres), and
/// `f_per_metre_hi`, a CEILING on it wherever `|residual|` is below
/// `residual_reach`, so that `|residual| ≥ min(residual_reach,
/// |F| / f_per_metre_hi)` everywhere — the lower bound a clear piece is
/// read in. A door whose `F` IS that residual passes exactly `1` for
/// both ratios and no reach (`None`: the ceiling holds everywhere).
///
/// `jet`, where a door has one, reads `F` and its first three
/// derivatives at a parameter from the carrier point, each with a bound
/// on its own rounding ([`Jet`]). The walk then reads its Taylor terms
/// there and charges each that bound, and `noise` enters only the one
/// global bound on `|F⁗|` the harmonics give.
pub struct SubdivisionFrame<'a, T> {
    /// The arc's start.
    pub t0: T,
    /// The arc's end.
    pub t1: T,
    /// The carrier's top speed, metres per radian.
    pub speed_hi: T,
    /// The bound on `F`'s `f64` evaluation error, in `F`'s units.
    pub noise: T,
    /// The floor on `|F| / |residual|`.
    pub f_per_metre: T,
    /// The ceiling on `|F| / |residual|` below `residual_reach`.
    pub f_per_metre_hi: T,
    /// Where the ceiling stops holding; `None`, everywhere.
    pub residual_reach: Option<T>,
    /// `F`, `F′`, `F″` and `F‴` read at the point, if the door has them.
    pub jet: Option<&'a Jet<'a, T>>,
}

/// `F` and its first three derivatives at a carrier parameter, each
/// with a bound on its rounding: a reading of the true `F` that does not
/// pass through the harmonics ([`SubdivisionFrame`]).
pub type Jet<'a, T> = dyn Fn(T) -> [geom_core::Rounded<T>; 4] + 'a;

/// **The root-slack meter** a caller of [`certified_subdivision`] may
/// hand it: `residual`, the surface's residual (metres) at a carrier
/// parameter carried with a running bound on its own rounding, and
/// `f_per_metre_hi`, a CEILING on `|F| / |residual|` near the surface
/// (where a root the band reads ON it lies). A caller may instead read
/// `F` itself (in `F`'s units, the circle × torus door's quartic in m⁴)
/// and pass a ceiling of `1`: the reading then bounds the true `|F|` at
/// the root directly, and every "metres of residual" below reads as
/// `F`'s units.
///
/// A root located on its monotone piece is off the true one by at most
/// `|residual(θ)| + error` metres of residual — the reading's own
/// magnitude plus the bound on its rounding — over the residual's least
/// slope where the true root can be, which is at least `F`'s least slope
/// there over `f_per_metre_hi`. That slope is read on a window about the
/// root twice the slack it gives (Taylor about the root; the window is
/// used only when that slack fits inside it), else on the whole piece,
/// which holds the true root: a piece can pass the monotone test with a
/// least slope barely above zero. At the carrier's top speed that is an
/// arc length,
/// and it must be definitely inside the band under `row`, or the span
/// and trim decisions the caller makes on the root are made on the wrong
/// point; an unreadable reading refuses too. A first-order bound, as the
/// running bound is.
pub struct RootSlack<'a, T> {
    /// The row the slack is decided under.
    pub row: &'static str,
    /// The surface's residual, with a running bound on its rounding.
    pub residual: &'a dyn Fn(T) -> geom_core::Rounded<T>,
    /// The ceiling on `|F| / |residual|` near the surface.
    pub f_per_metre_hi: T,
}

/// The predicate rows one caller of [`half_angle_roots`] meters under.
pub struct HalfAngleRows {
    /// The pole is definitely off the surface (a residual, metres).
    pub pole: &'static str,
    /// The pole is well conditioned (module docs; metres).
    pub conditioning: &'static str,
    /// The harmonics' rounding, as a residual, is past the band, and the
    /// ladder does not run (the noise meter; metres).
    pub noise: &'static str,
    /// The quartic ladder's own rows.
    pub quartic: QuarticRows,
    /// The certified subdivision's rows ([`certified_subdivision`]).
    pub verify: SubdivisionRows,
}

/// The predicate rows one caller of [`certified_subdivision`] meters
/// under, every one a residual in metres.
pub struct SubdivisionRows {
    /// A piece is definitely root-free: `|F(m)|` exceeds the most `F` can
    /// fall over the half-width.
    pub clear: &'static str,
    /// A piece is definitely monotone: `|F′(m)|` exceeds the most `F′`
    /// can fall over the half-width, read as the rise it certifies.
    pub monotone: &'static str,
    /// The residual's sign at a monotone piece's ends, and ON the surface
    /// at a located root.
    pub side: &'static str,
    /// A piece is still wider, as arc length, than the zero band: below
    /// it a piece that is neither clear nor monotone holds a double root
    /// the band cannot resolve.
    pub width: &'static str,
}

/// Where [`half_angle_roots`] works: the subdivision's frame (`walk`),
/// the carrier's least speed `|C′|` (a circle's radius; an ellipse's
/// semi-minor axis; the top speed is `walk`'s), and the ladder's `lever`
/// (a length).
pub struct HalfAngleFrame<'a, T> {
    /// The subdivision's frame.
    pub walk: SubdivisionFrame<'a, T>,
    /// The carrier's least speed.
    pub speed_lo: T,
    /// The length the ladder's roots spread over.
    pub lever: T,
}

impl<T: Decide> HalfAngleFrame<'_, T> {
    /// The ladder's noise meter's reading: the harmonics' evaluation
    /// error, `noise / f_per_metre`, in metres of residual.
    pub fn noise_reading(&self) -> T {
        self.walk.noise / self.walk.f_per_metre
    }
}

/// The conditioning floor `κ` (module docs): the pole's `|F|` must be at
/// least this share of `F`'s amplitude bound.
const POLE_CONDITIONING: f64 = 1.0 / 16.0;

/// How many evenly spaced poles round the carrier the anchor search may
/// try (module docs: 32 is what makes some candidate always pass
/// [`POLE_CONDITIONING`]).
const POLE_CANDIDATES: u32 = 32;

/// **The certified real roots of a degree-2 trigonometric polynomial
/// `F(θ)` along a conic** (module docs). General over the surface: `residual` is
/// the surface's residual (metres) along the carrier, which must share
/// `F`'s sign. The answer is [`certified_subdivision`]'s; the half-angle
/// ladder runs first and only its escalations are kept.
///
/// `slack` is the subdivision's root-slack meter: an answer rests on
/// the roots it reports.
///
/// # Errors
///
/// [`RootRefusal::Escalated`], the ladder's in-band sign, and the
/// subdivision's [`RootRefusal`]: a touch, a limit or a fault.
pub fn half_angle_roots<T: Decide>(
    f: &DegreeTwo<T>,
    residual: impl Fn(T) -> T,
    frame: HalfAngleFrame<'_, T>,
    rows: &HalfAngleRows,
    slack: &RootSlack<'_, T>,
    band: Band,
) -> Result<TrigRoots<T>, RootRefusal> {
    ladder_roots(f, &residual, &frame, rows, band)?;
    certified_subdivision(
        &f.poly(),
        &residual,
        &frame.walk,
        &rows.verify,
        Some(slack),
        band,
    )
}

/// [`half_angle_roots`]'s ladder: the quartic in the tangent half-angle
/// (module docs). It answers nothing but an escalation (the `Err`): its
/// quartic's count and roots are not read, and [`certified_subdivision`]
/// decides the roots in the residual's own metres.
#[allow(clippy::too_many_lines)] // the anchor search and the quartic, one walk
fn ladder_roots<T: Decide>(
    f: &DegreeTwo<T>,
    residual: &impl Fn(T) -> T,
    frame: &HalfAngleFrame<'_, T>,
    rows: &HalfAngleRows,
    band: Band,
) -> Result<(), Indeterminate> {
    let HalfAngleFrame {
        walk: SubdivisionFrame { t0, t1, .. },
        speed_lo,
        lever,
    } = *frame;
    let DegreeTwo { c0, c1, s1, c2, s2 } = *f;
    // **The noise meter** (module docs, "The harmonics' noise"): where
    // the harmonics' evaluation error, `noise / f_per_metre` metres, is
    // definitely past the band's escalation threshold, the ladder does
    // not run, so it cannot escalate on its own rounding; the
    // subdivision, which charges that noise, answers alone. A reading in
    // the gap, or none, runs it: it answers nothing, so running it can
    // only refuse. The stages after it (rotation, pole division, rescale,
    // depression, discriminant) round again; those are left to the
    // ladder's own band decisions, which is the premise the module docs
    // state.
    match decide(rows.noise, Margin::of(frame.noise_reading()), band) {
        Ok(Sign::Positive) => return Ok(()),
        Ok(Sign::Zero | Sign::Negative) | Err(_) => {}
    }
    let two = T::from_f64(2.0);
    let four = T::from_f64(4.0);
    let six = T::from_f64(6.0);
    let hypot = |x: T, y: T| (x.powi(2) + y.powi(2)).sqrt();
    let amplitude = c0.abs() + hypot(c1, s1) + hypot(c2, s2);
    let mid = (t0 + t1) / two;
    // An arc length that must be a LOWER bound (the root variable, the
    // conditioning margin) is metered at `speed_lo`; one that must be an
    // UPPER bound (a root's slack) at `speed_hi`.
    let scale = two * speed_lo;
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
        let (c1, s1) = (c1 * ca + s1 * sa, s1 * ca - c1 * sa);
        let (c2, s2) = (c2 * c2a + s2 * s2a, s2 * c2a - c2 * s2a);
        // `F·(1 + t²)²` in `t = tan(φ/2)`, low coefficient first.
        let a = [
            c0 + c1 + c2,
            two * s1 + four * s2,
            two * c0 - six * c2,
            two * s1 - four * s2,
            c0 - c1 + c2,
        ];
        let lead = a[4];
        match decide(
            rows.conditioning,
            Margin::over_lever(
                speed_lo * (lead.abs() - T::from_f64(POLE_CONDITIONING) * amplitude),
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
        let p = c2m - T::from_f64(3.0) * b3.powi(2) / T::from_f64(8.0);
        let q = d1 - b3 * c2m / two + b3.powi(3) / T::from_f64(8.0);
        let s = e0 - b3 * d1 / four + b3.powi(2) * c2m / T::from_f64(16.0)
            - T::from_f64(3.0) * b3.powi(4) / T::from_f64(256.0);
        // Its answer is not read (the subdivision answers); an in-band
        // sign it meets is the door's escalation. A repeated root it reads
        // is dropped with its count, as is its `CountDisagrees`, unlike
        // the line × torus door's, where the quartic's coefficients are
        // the residual's own and a disagreement is a classification
        // invariant: here they come
        // through the anchor rotation, pole division and rescale, whose
        // rounding nothing meters (module docs), and the module docs
        // record it disagreeing across definite crossings.
        return match depressed_quartic_roots(p, q, s, lever, &rows.quartic, band) {
            Err(RootRefusal::Escalated(diag)) => Err(diag),
            Ok(_)
            | Err(RootRefusal::Touch(_) | RootRefusal::Unsettled(_) | RootRefusal::Fault(_)) => {
                Ok(())
            }
        };
    }
    Ok(())
}

/// The pieces the whole turn is cut into before [`certified_subdivision`]
/// starts.
const SUBDIVISION_START: u32 = 16;

/// Where a piece may be split, as shares of its width, tried in order
/// until the residual's sign there is definite. The same shares, of one
/// starting piece, are the offsets tried for the turn's first cut.
///
/// No answer depends on their values: every end is a point whose sign
/// was DECIDED, so any share in `(0, 1)` keeps the walk sound. The values
/// are chosen for what they avoid and for what they bound:
///
/// - **None is `1/2`, and the first is no short fraction.** A pose
///   symmetric about the arc — a graze at a vertex, a wall square to an
///   axis — puts its root or its tangency at the arc's midpoint, or a
///   short dyadic share `j/2ᵐ` of the turn from it: exactly where
///   sixteenths from `mid − π` and halving splits would land, and where
///   a sign reads in the band. The first share is the `f64` nearest
///   `2√5 − 4 = 2/φ³ ≈ 0.4721`. That number is irrational; the stored
///   value is a dyadic rational like every `f64`, but one with a 53-bit
///   denominator, so the points it places are no short dyadic share of
///   the turn from the midpoint. The rest are
///   fallbacks, distinct and spread across the piece, so that a share
///   whose point reads in the band is followed by one away from it.
/// - **Each lies in `[0.12, 0.88]`**, so a split leaves each part at most
///   `0.88` of its piece: `k` levels down, a piece is at most `0.88ᵏ` of
///   a sixteenth of the turn, and the width row is reached in a number
///   of levels logarithmic in the band.
pub(crate) const SPLITS: [f64; 9] = [
    0.472_135_954_999_579_4,
    0.3,
    0.618,
    0.2,
    0.8,
    0.4,
    0.55,
    0.12,
    0.88,
];

/// The bisection steps that locate a root in its monotone piece: enough
/// to reach the scalar's resolution from a sixteenth of a turn.
const BISECTIONS: u32 = 64;

/// **The certified real roots of a trigonometric polynomial `F(θ)` of
/// degree `N ≤ MAX_DEGREE` over the whole turn about the arc's midpoint,
/// by subdivision decided in the RESIDUAL's metres.** It is
/// [`half_angle_roots`]' answer at `N = 2`, the ladder before it keeping
/// only its escalations, and the whole answer at `N = 4` (e.g. an ellipse
/// against a torus).
///
/// Why it is needed: the ladder decides on a quartic in a root variable
/// whose length is arc length only to first order about the anchor, and
/// only on a circle — along an ellipse an arc is up to `a/b` times its
/// `τ`-length. Its count and its roots were therefore certified in a
/// metric the band does not speak, and both went wrong: certified roots
/// fifteen zero bands off the wall (an eccentric ellipse at ε = 1e-6),
/// and a `Miss` for a carrier crossing a wall by a definite depth (an
/// ellipse or a circle grazing into a wall of radius 5–500 m).
///
/// What it decides instead is Taylor's theorem about the piece's
/// midpoint `m`, with the derivatives there to third order and the one
/// global bound the harmonics give exactly, `|F⁗| ≤ M₄ = Σ k⁴Aₖ`. The
/// turn is cut into pieces whose every end has a DEFINITE residual sign
/// (a cut that would land in the band is moved along its piece, and a
/// starting sixteenth with no definite share is left inside its
/// neighbours' piece). On a piece of half-width `w`:
///
/// - **clear** — `|F(m)| − (|F′(m)|w + |F″(m)|w²/2 + |F‴(m)|w³/6 +
///   M₄w⁴/24)` (less the noise) definitely positive, in metres: `F`
///   cannot reach zero on the piece;
/// - **monotone** — `|F′(m)| − (|F″(m)|w + |F‴(m)|w²/2 + M₄w³/6)`, the
///   least `|F′|` on the piece, definitely positive once read as the arc
///   length over which it provably stays so — that least slope over the
///   most `|F″|` can be on the piece, at the carrier's top speed: the
///   piece holds at most one root, and holds one exactly when its ends'
///   signs differ; that root is bisected on the residual itself to the
///   scalar's resolution and must then read ON the surface (`side`,
///   `Zero`): in the gap it is a touch, and definitely off it refuses
///   [`Unsettled::Meter`], the residual's step between two adjacent
///   scalars being past the band — and, where the caller hands
///   a [`RootSlack`] meter, its slack must be definitely inside the band
///   too, or it refuses [`Unsettled::Meter`]. The bisection closes on two adjacent scalars, and the root is
///   the one whose residual is nearer zero;
/// - otherwise the piece is split, until its arc length is inside the
///   band (`width`): a piece that small, neither clear nor monotone,
///   holds a double root — a tangency — and the walk refuses it on its
///   clear margin: a touch where that margin lies in the band's gap,
///   [`Unsettled::Tangency`] where it is decided, never a certified
///   `Miss`.
///
/// A piece none of whose split shares reads a definite sign, and a turn
/// with no definite share at all, refuse the same way: a touch on the
/// first share read in the gap, or [`Unsettled::Tangency`] where every
/// share read zero. The walk has cut wherever it can.
///
/// # Errors
///
/// [`RootRefusal::Touch`], [`Unsettled::Tangency`] and
/// [`Unsettled::Meter`] as above,
/// [`Unsettled::Exhausted`] past the piece budget, and
/// [`RootFault::ImpossibleCount`] for an odd count round the turn or more
/// roots than the degree allows.
///
/// The same Taylor bound, over a piece of a harmonic, is
/// `curve_pair::pose::window`'s.
///
/// `F` shares the residual's sign. The clear margin is read in metres
/// through the CEILING `f_per_metre_hi` (capped at `residual_reach`), so
/// it is a lower bound on the residual's least magnitude over the piece —
/// read through a floor it would overstate it, and a carrier within the
/// band could read clear.
/// The harmonics are rounded: `F` read from them is the
/// true one to within `noise`, so its `k`-th derivative to within
/// `Nᵏ·noise` (Bernstein's inequality for a trigonometric polynomial of
/// degree `N`), and each Taylor term above is charged its own share. A
/// frame's `jet` replaces those readings and their charges with its own;
/// the harmonics then give `M₄` alone.
///
/// **Without a meter** (`slack` `None`) a root is only a scalar whose
/// residual, as evaluated, reads ON beside a sign change of it: within
/// one scalar step of a true root of that evaluation, not within the
/// band of the true root, and a pair inside the band of a true graze
/// can stand for none. No answer rests on such roots: every door that
/// answers meters its walk ([`half_angle_roots`] takes its meter by
/// type; `ellipse_torus`'s `torus_walk` takes an `Option` that only its
/// own rows pass as `None`), and the meterless walks are the ruling
/// charts' uncharged reading, which only chooses a refusal's row, and
/// the rows.
#[allow(clippy::too_many_lines)] // one walk: the cut, the pieces, the bisection
pub fn certified_subdivision<T: Decide>(
    f: &TrigPoly<T>,
    residual: &impl Fn(T) -> T,
    frame: &SubdivisionFrame<'_, T>,
    rows: &SubdivisionRows,
    slack: Option<&RootSlack<'_, T>>,
    band: Band,
) -> Result<TrigRoots<T>, RootRefusal> {
    let SubdivisionFrame {
        t0,
        t1,
        speed_hi,
        noise,
        f_per_metre_hi,
        residual_reach,
        jet,
        ..
    } = *frame;
    let two = T::from_f64(2.0);
    // The degree, and the harmonics `1..=N` with each `k` as a scalar.
    let top = f.degree.min(4);
    let n_deg = usize::from(top);
    let harmonics = || (1..=top).map(|k| (usize::from(k), T::from_f64(f64::from(k))));
    let deg = T::from_f64(f64::from(top));
    let hypot = |x: T, y: T| (x.powi(2) + y.powi(2)).sqrt();
    // `|F⁗| ≤ Σ k⁴Aₖ`, for the true `F` once its harmonics' error is
    // charged at `N⁴·noise` (carried on the top harmonic's term).
    let fourth_hi = harmonics().fold(T::zero(), |acc, (k, kf)| {
        let amp = hypot(f.cos[k], f.sin[k]);
        let amp = if k == n_deg { amp + noise } else { amp };
        acc + kf.powi(4) * amp
    });
    // `(sin kt, cos kt)` for `k = 1..=N`.
    let turns = |t: T| -> [(T, T); MAX_DEGREE + 1] {
        let mut out = [(T::zero(), T::one()); MAX_DEGREE + 1];
        for (k, kf) in harmonics() {
            out[k] = (kf * t).sin_cos();
        }
        out
    };
    let value = |t: T| {
        let sc = turns(t);
        harmonics().fold(f.cos[0], |acc, (k, _)| {
            acc + f.cos[k] * sc[k].1 + f.sin[k] * sc[k].0
        })
    };
    // The `j`-th derivative's harmonic `k` is `kʲ` times the harmonic
    // turned by `jπ/2`: the four turns repeat with period four.
    let derivative = |t: T, j: u8| {
        let sc = turns(t);
        harmonics().fold(T::zero(), |acc, (k, kf)| {
            let (sk, ck) = sc[k];
            let (c, s) = (f.cos[k], f.sin[k]);
            let lever = kf.powi(i32::from(j));
            match j % 4 {
                0 => acc + lever * (c * ck + s * sk),
                1 => acc + lever * (s * ck - c * sk),
                2 => acc - lever * (c * ck + s * sk),
                _ => acc + lever * (c * sk - s * ck),
            }
        })
    };
    // `F` and its first three derivatives at `t`, each with a bound on
    // its distance from the true one: the door's jet where it has one,
    // else the harmonics with `k` derivatives of an error bounded by
    // `noise` bounded by `Nᵏ·noise` (Bernstein's inequality, degree `N`).
    let taylor = |t: T| -> [(T, T); 4] {
        match jet {
            Some(jet) => jet(t).map(|r| (r.value, r.error)),
            None => [
                (value(t), noise),
                (derivative(t, 1), deg * noise),
                (derivative(t, 2), deg.powi(2) * noise),
                (derivative(t, 3), deg.powi(3) * noise),
            ],
        }
    };
    let (six, twenty_four) = (T::from_f64(6.0), T::from_f64(24.0));
    let definitely = |row, m: T| matches!(decide(row, Margin::of(m), band), Ok(Sign::Positive));
    // The first reading in the band's gap at a cut the walk tried: what a
    // walk that can cut nowhere definite refuses on.
    let gap = core::cell::Cell::new(None);
    let side = |t: T| match decide(rows.side, Margin::of(residual(t)), band) {
        Ok(s @ (Sign::Positive | Sign::Negative)) => Some(s),
        Ok(Sign::Zero) => None,
        Err(diag) => {
            if gap.get().is_none() {
                gap.set(Some(diag));
            }
            None
        }
    };
    // Every cut tried reads in band: a touch on the gap's reading where
    // one read there, and a touch decided in the zero band where none did.
    let uncut = || {
        gap.get()
            .map_or(Unsettled::Tangency.into(), RootRefusal::Touch)
    };
    // The first cut: one end in each sixteenth of the turn, each moved
    // along its sixteenth until its sign is definite; a sixteenth with no
    // such share is re-cut by leaving it inside its neighbours' piece, so
    // a stretch in band at the walk's own cuts is walked, not refused. The
    // last end is the first a turn on.
    let n = SUBDIVISION_START;
    let piece = T::tau() / T::from_f64(f64::from(n));
    let base = (t0 + t1) / two - T::pi();
    let mut ends: Vec<(T, Sign)> = (0..n)
        .filter_map(|k| {
            let from = base + piece * T::from_f64(f64::from(k));
            SPLITS.iter().find_map(|&share| {
                let t = from + piece * T::from_f64(share);
                side(t).map(|s| (t, s))
            })
        })
        .collect();
    let Some(&(first, first_side)) = ends.first() else {
        return Err(uncut());
    };
    ends.push((first + T::tau(), first_side));
    // The pieces are walked left to right ([`geom_core::bisect`]), so the
    // bisection's raw midpoints come out in order; the reported roots do
    // not, since one in the walk's tail past `mid + π` re-centres to the
    // front, and no reader relies on the order (the type is documented
    // unordered). A walk past the core's piece budget answers
    // [`Unsettled::Exhausted`].
    let pieces: Vec<((T, Sign), (T, Sign))> = ends.windows(2).map(|w| (w[0], w[1])).collect();
    let mut roots: Vec<T> = Vec::new();
    let walk = bisect(pieces, |((l, sl), (r, sr)): ((T, Sign), (T, Sign))| {
        let half = (r - l) / two;
        let m = l + half;
        // The true `F`'s derivatives at `m`, each widened by its bound.
        let [(f0, e0), (f1, e1), (f2, e2), (f3, e3)] = taylor(m);
        let (d1_lo, d1_hi) = (f1.abs() - e1, f1.abs() + e1);
        let d2_hi = f2.abs() + e2;
        let d3_hi = f3.abs() + e3;
        let fall = d1_hi * half
            + d2_hi * half.powi(2) / two
            + d3_hi * half.powi(3) / six
            + fourth_hi * half.powi(4) / twenty_four;
        let clear = (f0.abs() - fall - e0) / f_per_metre_hi;
        let clear = residual_reach.map_or(clear, |reach| clear.min(reach));
        let clear_read = decide(rows.clear, Margin::of(clear), band);
        if matches!(clear_read, Ok(Sign::Positive)) {
            return Step::Settled;
        }
        let most_bend = d2_hi + d3_hi * half + fourth_hi * half.powi(2) / two;
        let least_slope =
            d1_lo - d2_hi * half - d3_hi * half.powi(2) / two - fourth_hi * half.powi(3) / six;
        if definitely(rows.monotone, speed_hi * least_slope / most_bend) {
            if sl == sr {
                return Step::Settled;
            }
            // Bisected on the residual evaluated at the point, not on `F`
            // from its harmonics: the two share their zero set, and near a
            // graze `|F′|` is small enough that the harmonics' rounding
            // moves `F`'s sign change along the arc by more than the band.
            let close = |l: T, r: T| {
                let (mut lo, mut hi) = (l, r);
                let at_lo = residual(l);
                for _ in 0..BISECTIONS {
                    let c = (lo + hi) / two;
                    let product = residual(c) * at_lo;
                    lo = product.select_le_zero(lo, c);
                    hi = product.select_le_zero(c, hi);
                }
                (lo, hi)
            };
            // Reported within `π` of the arc's midpoint, so a caller
            // compares it with `[t0, t1]` directly: the first cut's ends
            // are moved along their pieces, so the last piece runs past
            // the half-turn. Of the two adjacent scalars the bracket
            // closes on, a steep residual can read ON at one only, and
            // their midpoint rounds onto either. Sound at every scalar by
            // `ring_path::branch`'s window re-centring argument.
            // `rows.side` reads the residual, which is periodic. The
            // readers read an end of a whole-turn span alike: the wall
            // crossing's `at_end`, and the ruling's fixed `(−π, π)`
            // window, which re-sorts its roots. None reads the root as an
            // identity.
            let mid = (t0 + t1) / two;
            let reported = |t: T| mid + (t - mid).reduce_periodic_centred(T::tau());
            let nearer = |(lo, hi): (T, T)| {
                let (at_lo, at_hi) = (reported(lo), reported(hi));
                (residual(at_lo).abs() - residual(at_hi).abs()).select_le_zero(at_lo, at_hi)
            };
            let on = |t: T| {
                matches!(
                    decide(rows.side, Margin::of(residual(t)), band),
                    Ok(Sign::Zero)
                )
            };
            let (lo, hi) = close(l, r);
            let mut root = nearer((lo, hi));
            if !on(root) {
                // Moved by the turn it is reported at, which is `2π` only to
                // rounding, the root stands on a line read with rounding of
                // its own: the bracket is closed again on the piece moved
                // by that turn, on the scalars the root is reported at.
                let turn = lo - reported(lo);
                root = nearer(close(l - turn, r - turn));
                match decide(rows.side, Margin::of(residual(root)), band) {
                    Ok(Sign::Zero) => {}
                    Err(diag) => return Step::Stop(RootRefusal::Touch(diag)),
                    // Definitely off beside a sign change: the residual's
                    // step between adjacent scalars is past the band.
                    Ok(Sign::Positive | Sign::Negative) => {
                        return Step::Stop(Unsettled::Meter.into());
                    }
                }
            }
            if let Some(meter) = slack {
                // `|F|` at the root, true to within `reach`: the true root
                // lies within `reach / L` of it wherever `|F′| ≥ L` holds
                // that far. `L` is read on a window about the root twice
                // that width at the root's own slope — valid when the
                // slack it gives fits inside it — and otherwise on the
                // whole piece, which holds the true root.
                let reach = meter.f_per_metre_hi * (meter.residual)(root).magnitude();
                let [_, (g1, e1), (g2, e2), (g3, e3)] = taylor(root);
                let near = g1.abs() - e1;
                let w = (two * reach / near).min(half).max(T::zero());
                let window = near
                    - (g2.abs() + e2) * w
                    - (g3.abs() + e3) * w.powi(2) / two
                    - fourth_hi * w.powi(3) / six;
                let lever =
                    (reach - window * w).select_le_zero(window.max(least_slope), least_slope);
                let arc = speed_hi * reach / lever;
                match decide(meter.row, Margin::of(arc), band) {
                    Ok(Sign::Zero | Sign::Negative) => {}
                    Ok(Sign::Positive) | Err(_) => return Step::Stop(Unsettled::Meter.into()),
                }
            }
            roots.push(root);
            return Step::Settled;
        }
        if !definitely(rows.width, speed_hi * (r - l)) {
            return Step::Stop(match clear_read {
                Err(diag) => RootRefusal::Touch(diag),
                Ok(_) => Unsettled::Tangency.into(),
            });
        }
        gap.set(None);
        let Some((c, sc)) = SPLITS.iter().find_map(|&share| {
            let c = l + (r - l) * T::from_f64(share);
            side(c).map(|s| (c, s))
        }) else {
            return Step::Stop(uncut());
        };
        Step::Halve([((l, sl), (c, sc)), ((c, sc), (r, sr))])
    });
    match walk {
        Ok(None) => {}
        Ok(Some(why)) => return Err(why),
        Err(_) => return Err(Unsettled::Exhausted.into()),
    }
    // Round the closed turn the ends' signs change an even number of
    // times, and every change on a monotone piece was bisected; an odd
    // count therefore means a change on a piece read CLEAR — the residual
    // and its harmonics disagreeing by more than `noise` — and more than
    // `2N` is more than a degree-`N` polynomial has. Either way a root was
    // lost or invented, and nothing is certified.
    if roots.len() > 2 * n_deg || roots.len() % 2 == 1 {
        return Err(RootFault::ImpossibleCount.into());
    }
    Ok(Roots::of(&roots).ok_or(RootFault::ImpossibleCount)?)
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod subdivision_guard_rows {
    //! [`certified_subdivision`]'s guards, each on a residual built to sit
    //! where that guard is the only thing between the walk and a wrong
    //! answer. The harmonics and the residual are given separately — the
    //! harmonics standing in for rounded ones, the residual for the true
    //! one. Physical poses do put the rounding near the band (metre-scale
    //! carriers against micrometre balls at ε = 1e-12, which is why the
    //! noise is charged at all), but none the graze fuzz drew made one of
    //! these guards the ONLY thing deciding: there the charged noise
    //! refuses first, or the residual's sign changes are clean. These
    //! rows are where each guard is exercised alone.

    use core::f64::consts::PI;

    use super::*;

    const ROWS: SubdivisionRows = SubdivisionRows {
        clear: "bool_conic_quadric_sub_clear",
        monotone: "bool_conic_quadric_sub_monotone",
        side: "bool_conic_quadric_sub_side",
        width: "bool_conic_quadric_sub_width",
    };

    fn walk(
        f: &TrigPoly<f64>,
        residual: &impl Fn(f64) -> f64,
        noise: f64,
        eps: f64,
    ) -> Result<TrigRoots<f64>, RootRefusal> {
        let frame = SubdivisionFrame {
            t0: -0.5,
            t1: 0.5,
            speed_hi: 1.0,
            noise,
            f_per_metre: 1.0,
            f_per_metre_hi: 1.0,
            residual_reach: None,
            jet: None,
        };
        certified_subdivision(
            f,
            residual,
            &frame,
            &ROWS,
            None,
            Band::new(eps, 10.0 * eps).unwrap(),
        )
    }

    /// The residual's sign changes round the turn, read on a grid fine
    /// enough for the residuals below.
    fn sign_changes(residual: &impl Fn(f64) -> f64) -> usize {
        let n = 200_000_u32;
        let samples: Vec<f64> = (0..=n)
            .map(|k| residual(-PI + 2.0 * PI * f64::from(k) / f64::from(n)))
            .collect();
        samples
            .windows(2)
            .filter(|w| (w[0] < 0.0) != (w[1] < 0.0))
            .count()
    }

    fn sine(s1: f64) -> TrigPoly<f64> {
        DegreeTwo::new(0.0, 0.0, s1, 0.0, 0.0).poly()
    }

    /// **The monotone test charges `F′` its share of the noise.** `F` read
    /// from the harmonics is `10⁻³·sin θ`; the true residual differs from
    /// it by at most `0.7·10⁻³` — inside the declared `noise` of
    /// `0.8·10⁻³` — as a wiggle about `θ = 0` that crosses zero several
    /// times. `F′` there is `10⁻³`, less than the `2·noise` its rounding
    /// may move it by, so the piece is not certified monotone and the
    /// walk refuses. Charged nothing, the piece reads monotone,
    /// one crossing is bisected, and two certified roots stand for a
    /// residual that crosses zero more often.
    #[test]
    fn a_slope_inside_its_noise_is_not_monotone() {
        let f = sine(1e-3);
        let residual = |t: f64| {
            let t = (t + PI).rem_euclid(2.0 * PI) - PI;
            1e-3 * t.sin() - 0.7e-3 * (t / 0.01).sin() * (-(t / 0.03).powi(2)).exp()
        };
        let truth = sign_changes(&residual);
        assert!(truth > 2, "the wiggle crosses zero, read {truth} changes");
        match walk(&f, &residual, 0.8e-3, 1e-9) {
            Ok(Roots::Certified(found)) => {
                let count = found.count();
                assert_eq!(count, truth, "certified {count} roots of {truth}");
            }
            Err(_) => {}
            other => panic!("{other:?} for a residual crossing {truth} times"),
        }
    }

    /// **The Taylor remainder is charged.** `F = c₀ + 4 cos θ − cos 2θ`
    /// has `F′ = F″ = F‴ = 0` at `θ = 0` and `F⁗ = −12`: a fourth-order
    /// graze, `F ≈ F(0) − θ⁴/2`. With `F(0) = 3·10⁻⁴` it crosses at
    /// `θ ≈ ±0.156`, inside the first cut's piece about zero, whose ends
    /// read the same sign. The derivatives to third order at that piece's
    /// midpoint say `F` barely moves; only `M₄ = A₁ + 16A₂` says it can
    /// fall `1.2·10⁻³` over the half-width, so the piece is split and the
    /// two roots found. Without it the piece reads clear, and the turn a
    /// certified `Miss`.
    #[test]
    fn a_fourth_order_graze_is_not_read_clear() {
        let f = DegreeTwo::new(-3.0 + 3e-4, 4.0, 0.0, -1.0, 0.0).poly();
        let value = |t: f64| f.cos[0] + f.cos[1] * t.cos() + f.cos[2] * (2.0 * t).cos();
        assert_eq!(sign_changes(&value), 2);
        match walk(&f, &value, 0.0, 1e-9) {
            Ok(Roots::Certified(found)) => {
                let count = found.count();
                assert_eq!(count, 2, "certified {count} roots of 2");
                for &t in found.roots() {
                    assert!(value(t).abs() <= 1e-9, "root {t} reads {:e}", value(t));
                }
            }
            Err(_) => {}
            other => panic!("{other:?} for a residual crossing twice"),
        }
    }

    /// **The remainder reads every harmonic up to the degree.**
    /// `F = −7 + 3·10⁻⁴ + 16 cos 3φ − 9 cos 4φ`, `φ = θ − θg`, has no first
    /// or second harmonic, so `A₁ + 16A₂` is zero, and at `φ = 0` its
    /// derivatives to third order vanish (`9·16 = 16·9`) while
    /// `F⁗ = 81·16 − 256·9 = −1008`: a fourth-order graze,
    /// `F ≈ 3·10⁻⁴ − 42φ⁴`, crossing at `φ ≈ ±0.052`. `θg` is the midpoint
    /// of the first cut's piece about zero, whose ends read the same sign,
    /// so the derivatives the walk reads there say `F` barely moves. Only
    /// `M₄ = Σ k⁴Aₖ` over all four harmonics says it can fall `0.2` over
    /// the piece; read at degree two the piece is clear, the pair is lost,
    /// and the count is certified short by two.
    #[test]
    fn a_fourth_harmonic_graze_is_not_read_clear() {
        let piece = core::f64::consts::TAU / f64::from(SUBDIVISION_START);
        let (l, r) = (
            (-PI + piece * 7.0) + piece * SPLITS[0],
            (-PI + piece * 8.0) + piece * SPLITS[0],
        );
        let at = l + (r - l) / 2.0;
        let (s3, c3) = (3.0 * at).sin_cos();
        let (s4, c4) = (4.0 * at).sin_cos();
        let f = TrigPoly {
            cos: [-7.0 + 3e-4, 0.0, 0.0, 16.0 * c3, -9.0 * c4],
            sin: [0.0, 0.0, 0.0, 16.0 * s3, -9.0 * s4],
            degree: 4,
        };
        let value =
            |t: f64| f.cos[0] + 16.0 * (3.0 * (t - at)).cos() - 9.0 * (4.0 * (t - at)).cos();
        let truth = sign_changes(&value);
        assert!(truth > 2, "the graze pair and the rest, read {truth}");
        let frame = SubdivisionFrame {
            t0: -0.5,
            t1: 0.5,
            speed_hi: 1.0,
            noise: 0.0,
            f_per_metre: 1.0,
            f_per_metre_hi: 1.0,
            residual_reach: None,
            jet: None,
        };
        let band = Band::new(1e-9, 1e-8).unwrap();
        match certified_subdivision(&f, &value, &frame, &ROWS, None, band) {
            Ok(Roots::Certified(found)) => {
                let count = found.count();
                assert_eq!(count, truth, "certified {count} roots of {truth}");
            }
            Err(_) => {}
            other => panic!("{other:?} for a residual crossing {truth} times"),
        }
    }

    /// **A located root must read ON the surface.** The residual is
    /// `sin θ` resolved only to steps of `20ε` — never zero, its sign
    /// change a jump from `−10ε` to `+10ε` — the harmonics exact to within
    /// that step (declared as the noise). Bisection closes on the jump,
    /// where the residual reads `10ε` off, so the walk refuses;
    /// not checked, the jump is certified a root.
    #[test]
    fn a_root_that_reads_off_the_surface_is_not_certified() {
        let eps = 1e-9;
        let step = 20.0 * eps;
        let residual = |t: f64| step * ((t.sin() / step).floor() + 0.5);
        let got = walk(&sine(1.0), &residual, step, eps);
        if let Ok(Roots::Certified(found)) = got {
            for &t in found.roots() {
                let off = residual(t).abs();
                assert!(off <= eps, "root {t} reads {off:e} off the surface");
            }
        }
    }

    /// **A bisected root is the bracket end nearer the surface.** The
    /// residual is `sin(θ − a)` steepened `8.1·10⁶`-fold about `a` and
    /// shifted `0.3` of the band's zero down there, so between `a` and the
    /// next scalar `b` it steps from `−0.3` to `+1.5` zeros: the bracket
    /// closes on them, `a` reads ON and `b` does not. `a`'s last bit is
    /// odd, so their midpoint rounds onto `b`; reported there the root
    /// reads off the surface and the walk refuses.
    #[test]
    fn a_steep_root_is_reported_at_the_bracket_end_on_the_surface() {
        let eps = 1e-9;
        let a = f64::from_bits(1.0f64.to_bits() + 3);
        let b = f64::from_bits(a.to_bits() + 1);
        assert_eq!((a + b) / 2.0, b, "the midpoint rounds onto b");
        let (steep, shift) = (1.8 * eps / (b - a), 0.3 * eps);
        let residual = |t: f64| {
            let near = (-((t - a) / 0.1).powi(2)).exp();
            (t - a).sin() * (1.0 + (steep - 1.0) * near) - shift * near
        };
        assert!(residual(a).abs() < eps && residual(b) > eps, "a ON, b off");
        let (s, c) = a.sin_cos();
        let f = DegreeTwo::new(0.0, -s, c, 0.0, 0.0).poly();
        match walk(&f, &residual, 0.0, eps) {
            Ok(Roots::Certified(found)) if found.count() == 2 => {
                assert!(
                    found.roots().contains(&a),
                    "the roots {:?} miss a",
                    found.roots()
                );
            }
            other => panic!("{other:?} for a residual crossing twice, once at a"),
        }
    }

    /// **An odd count is not certified.** The harmonics read `sin θ`; the
    /// residual is `−1` from `θ = −0.05` to `0.4` and `sin θ` elsewhere —
    /// a disagreement no declared noise covers — so its one sign change
    /// near the arc falls on a piece the harmonics read clear (the first
    /// cut's piece from `0.185` to `0.578`) and is lost, while the change
    /// at `π` is bisected. Round a closed turn that leaves an odd count,
    /// which the walk refuses; not checked, one root is certified for a
    /// residual that crosses twice.
    #[test]
    fn an_odd_count_is_not_certified() {
        let residual = |t: f64| {
            let t = (t + PI).rem_euclid(2.0 * PI) - PI;
            if t > -0.05 && t < 0.4 { -1.0 } else { t.sin() }
        };
        assert_eq!(sign_changes(&residual), 2);
        match walk(&sine(1.0), &residual, 0.0, 1e-9) {
            Ok(Roots::Certified(found)) => {
                let count = found.count();
                assert_eq!(count % 2, 0, "certified an odd count, {count}");
            }
            Err(_) => {}
            other => panic!("{other:?} for a residual crossing twice"),
        }
    }

    /// **Review R1 (PR 5993): the re-cut never loses or merges a root.**
    /// `F = A(1 − cos φ)² − c`, `φ = θ − θ₀`, is flat to fourth order at
    /// `θ₀` (`F ≈ Aφ⁴/4 − c`), so with `A = 10⁻⁶` a whole sixteenth about
    /// `θ₀` reads in band at every share and the first cut leaves it
    /// inside its neighbours' piece. Swept over `θ₀` (so the flat stretch
    /// lands on a cut, across two sixteenths, and inside one) and over
    /// `c` (no root, a double root, two roots within the band of each
    /// other, two roots in adjacent sixteenths, two roots well apart):
    /// every certified answer has the true count with each root on the
    /// residual, and every refusal is a touch or a limit, never a fault
    /// and never a certified miss of a residual that crosses.
    #[test]
    fn review_r1_the_recut_keeps_every_root() {
        let a = 1e-6;
        let (mut certified, mut touched, mut limited) = (0, 0, 0);
        // Every third of a sixteenth, so the flat stretch sits on a cut,
        // across one, and inside a sixteenth.
        for i in 0..48 {
            let t0 = -PI + 2.0 * PI * f64::from(i) / 48.0 + 1e-3;
            for c in [-1e-11, 0.0, 1e-11, 1e-10, 4e-9, 3e-8, 1e-7, 1e-6] {
                let f = DegreeTwo::new(
                    1.5 * a - c,
                    -2.0 * a * t0.cos(),
                    -2.0 * a * t0.sin(),
                    0.5 * a * (2.0 * t0).cos(),
                    0.5 * a * (2.0 * t0).sin(),
                )
                .poly();
                let value = |t: f64| a * (1.0 - (t - t0).cos()).powi(2) - c;
                // The nearest roots stand `0.16` apart, so a coarser grid
                // than `sign_changes`' reads them.
                let truth = (0..=20_000_u32)
                    .map(|k| value(-PI + 2.0 * PI * f64::from(k) / 20_000.0))
                    .collect::<Vec<_>>()
                    .windows(2)
                    .filter(|w| (w[0] < 0.0) != (w[1] < 0.0))
                    .count();
                match walk(&f, &value, 0.0, 1e-9) {
                    Ok(Roots::Certified(found)) => {
                        certified += 1;
                        assert_eq!(found.count(), truth, "θ₀ {t0}, c {c:e}: {found:?}");
                        for &t in found.roots() {
                            assert!(value(t).abs() <= 1e-8, "θ₀ {t0}, c {c:e}: root {t}");
                        }
                    }
                    Ok(Roots::Miss) => {
                        assert_eq!(truth, 0, "θ₀ {t0}, c {c:e}: a miss of {truth} crossings");
                        certified += 1;
                    }
                    Err(RootRefusal::Touch(_)) => touched += 1,
                    Err(RootRefusal::Unsettled(_)) => limited += 1,
                    other => panic!("θ₀ {t0}, c {c:e} ({truth} crossings): {other:?}"),
                }
            }
        }
        assert!(
            touched > 0 && certified > 0,
            "certified {certified}, touched {touched}, limited {limited}"
        );
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod review_5993_recut_rows {
    //! Review rows (PR 5993, R2): the first cut's re-cut of a sixteenth
    //! with no definite share. Each residual is read against a dense
    //! sampling of its sign changes; the walk may refuse, but a certified
    //! count must be the true one.

    use core::f64::consts::PI;

    use super::*;

    const ROWS: SubdivisionRows = SubdivisionRows {
        clear: "bool_conic_quadric_sub_clear",
        monotone: "bool_conic_quadric_sub_monotone",
        side: "bool_conic_quadric_sub_side",
        width: "bool_conic_quadric_sub_width",
    };

    fn walk(f: &TrigPoly<f64>, eps: f64) -> Result<TrigRoots<f64>, RootRefusal> {
        let frame = SubdivisionFrame {
            t0: -0.5,
            t1: 0.5,
            speed_hi: 1.0,
            noise: 0.0,
            f_per_metre: 1.0,
            f_per_metre_hi: 1.0,
            residual_reach: None,
            jet: None,
        };
        let value = |t: f64| {
            f.cos[0]
                + f.cos[1] * t.cos()
                + f.sin[1] * t.sin()
                + f.cos[2] * (2.0 * t).cos()
                + f.sin[2] * (2.0 * t).sin()
        };
        certified_subdivision(
            f,
            &value,
            &frame,
            &ROWS,
            None,
            Band::new(eps, 10.0 * eps).unwrap(),
        )
    }

    fn sign_changes(f: &TrigPoly<f64>) -> usize {
        let value = |t: f64| {
            f.cos[0]
                + f.cos[1] * t.cos()
                + f.sin[1] * t.sin()
                + f.cos[2] * (2.0 * t).cos()
                + f.sin[2] * (2.0 * t).sin()
        };
        let n = 400_000_u32;
        let s: Vec<f64> = (0..=n)
            .map(|k| value(-PI + 2.0 * PI * f64::from(k) / f64::from(n)))
            .collect();
        s.windows(2)
            .filter(|w| (w[0] < 0.0) != (w[1] < 0.0))
            .count()
    }

    /// How many sixteenths of the first cut have no definite share.
    fn dropped(f: &TrigPoly<f64>, eps: f64) -> usize {
        let band = Band::new(eps, 10.0 * eps).unwrap();
        let value = |t: f64| {
            f.cos[0]
                + f.cos[1] * t.cos()
                + f.sin[1] * t.sin()
                + f.cos[2] * (2.0 * t).cos()
                + f.sin[2] * (2.0 * t).sin()
        };
        let piece = core::f64::consts::TAU / f64::from(SUBDIVISION_START);
        let base = 0.0 - PI;
        (0..SUBDIVISION_START)
            .filter(|&k| {
                let from = base + piece * f64::from(k);
                !SPLITS.iter().any(|&s| {
                    matches!(
                        decide(ROWS.side, Margin::of(value(from + piece * s)), band),
                        Ok(Sign::Positive | Sign::Negative)
                    )
                })
            })
            .count()
    }

    #[test]
    fn review_5993_recut_never_certifies_a_wrong_count() {
        let eps = 1e-9;
        // A small-amplitude second harmonic over a near-cancelling first:
        // wide in-band stretches at the first cut's shares.
        let mut seed = 0x9e37_79b9_7f4a_7c15_u64;
        let mut rnd = || {
            seed ^= seed << 13;
            seed ^= seed >> 7;
            seed ^= seed << 17;
            (seed >> 11) as f64 / (1u64 << 53) as f64
        };
        let (mut recut, mut certified, mut touched, mut other) = (0, 0, 0, 0);
        for _ in 0..4000 {
            let amp = 10f64.powf(-9.0 + 3.0 * rnd()); // 1e-9 .. 1e-6
            let c0 = amp * (2.0 * rnd() - 1.0);
            let f = DegreeTwo::new(
                c0,
                amp * (2.0 * rnd() - 1.0),
                amp * (2.0 * rnd() - 1.0),
                amp * (2.0 * rnd() - 1.0),
                amp * (2.0 * rnd() - 1.0),
            )
            .poly();
            let drops = dropped(&f, eps);
            let got = walk(&f, eps);
            if drops == 0 {
                continue;
            }
            recut += 1;
            match got {
                Ok(Roots::Certified(found)) => {
                    certified += 1;
                    let truth = sign_changes(&f);
                    assert_eq!(
                        found.count(),
                        truth,
                        "{:?}: dropped {drops}",
                        (f.cos, f.sin)
                    );
                }
                Ok(Roots::Miss) => {
                    certified += 1;
                    assert_eq!(
                        sign_changes(&f),
                        0,
                        "{:?}: a miss, dropped {drops}",
                        (f.cos, f.sin)
                    );
                }
                Err(RootRefusal::Touch(_)) => touched += 1,
                _ => other += 1,
            }
        }
        println!("recut {recut}: certified {certified}, touch {touched}, other {other}");
        assert!(
            certified > 0,
            "the re-cut certified nothing in {recut} draws"
        );
    }

    /// Two transverse roots in adjacent sixteenths about a stretch in band.
    #[test]
    fn review_5993_two_roots_beside_a_dropped_sixteenth() {
        let eps = 1e-9;
        for &(a, d) in &[
            (1e-7, 5e-10),
            (1e-7, 2e-9),
            (1e-7, 4e-9),
            (3e-8, 1e-9),
            (1e-6, 2e-8),
        ] {
            // a·(1 − cos t) − d: roots at ±acos(1 − d/a).
            let f = DegreeTwo::new(a - d, -a, 0.0, 0.0, 0.0).poly();
            let got = walk(&f, eps);
            println!(
                "a {a:e} d {d:e} dropped {} truth {} -> {got:?}",
                dropped(&f, eps),
                sign_changes(&f)
            );
            if let Ok(Roots::Certified(found)) = got {
                assert_eq!(found.count(), sign_changes(&f));
            }
            assert!(!matches!(got, Ok(Roots::Miss)), "a miss of two roots");
        }
    }

    /// An exact double root, and a root exactly at a cut's first share.
    #[test]
    fn review_5993_double_root_and_root_at_a_cut() {
        let eps = 1e-9;
        let f = DegreeTwo::new(1e-7, -1e-7, 0.0, 0.0, 0.0).poly();
        let got = walk(&f, eps);
        println!("double: dropped {} -> {got:?}", dropped(&f, eps));
        assert!(got.is_err(), "a double root certified: {got:?}");
        let piece = core::f64::consts::TAU / f64::from(SUBDIVISION_START);
        let at = -PI + piece * 9.0 + piece * SPLITS[0];
        // sin(t − at) · 1e-3: roots at `at` and `at + π`.
        let (s, c) = at.sin_cos();
        let f = DegreeTwo::new(0.0, -1e-3 * s, 1e-3 * c, 0.0, 0.0).poly();
        let got = walk(&f, eps);
        println!("root at a cut -> {got:?}");
        match got {
            Ok(Roots::Certified(found)) => assert_eq!(found.count(), 2),
            other => panic!("{other:?}"),
        }
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod review3_fault_rows {
    //! Review 3 (PR 5993): `RootFault::ImpossibleCount` from legal input.
    //! The residual is the harmonics summed in another order, and the
    //! frame charges that evaluation's rounding honestly: a door that does
    //! this is legal input, so a fault here is reachable.

    use super::*;

    const ROWS: SubdivisionRows = SubdivisionRows {
        clear: "bool_conic_quadric_sub_clear",
        monotone: "bool_conic_quadric_sub_monotone",
        side: "bool_conic_quadric_sub_side",
        width: "bool_conic_quadric_sub_width",
    };

    #[test]
    #[ignore = "review probe"]
    fn review3_no_legal_residual_reaches_impossible_count() {
        let mut seed = 0x2545_f491_4f6c_dd1d_u64;
        let mut rnd = || {
            seed ^= seed << 13;
            seed ^= seed >> 7;
            seed ^= seed << 17;
            (seed >> 11) as f64 / (1u64 << 53) as f64
        };
        let mut tally = std::collections::BTreeMap::<String, usize>::new();
        let mut faults = Vec::new();
        for eps in [1e-6, 1e-9, 1e-12] {
            let band = Band::new(eps, 10.0 * eps).unwrap();
            for i in 0..30_000 {
                let scale = 10f64.powf(-3.0 * rnd());
                let phi = core::f64::consts::TAU * rnd();
                // A near-double root (family 0), a near-quadruple one
                // (1), or a random pair of harmonics (2).
                let d = eps * 10f64.powf(3.0 * rnd() - 1.0) * (2.0 * rnd() - 1.0);
                let tiny = eps * 10f64.powf(2.0 * rnd() - 1.0);
                let (c, s) = match i % 3 {
                    0 => (
                        [scale - d, -scale * phi.cos(), tiny * (2.0 * rnd() - 1.0)],
                        [0.0, -scale * phi.sin(), tiny * (2.0 * rnd() - 1.0)],
                    ),
                    1 => (
                        // (1 − cos(t−φ))² = 3/2 − 2cos(t−φ) + cos(2(t−φ))/2
                        [
                            scale * 1.5 - d,
                            -2.0 * scale * phi.cos(),
                            0.5 * scale * (2.0 * phi).cos(),
                        ],
                        [
                            0.0,
                            -2.0 * scale * phi.sin(),
                            0.5 * scale * (2.0 * phi).sin(),
                        ],
                    ),
                    _ => (
                        [
                            scale * (2.0 * rnd() - 1.0),
                            scale * (2.0 * rnd() - 1.0),
                            scale * (2.0 * rnd() - 1.0),
                        ],
                        [
                            0.0,
                            scale * (2.0 * rnd() - 1.0),
                            scale * (2.0 * rnd() - 1.0),
                        ],
                    ),
                };
                let f = DegreeTwo::new(c[0], c[1], s[1], c[2], s[2]).poly();
                let sum: f64 = c.iter().chain(&s).map(|x| x.abs()).sum();
                let frame = SubdivisionFrame {
                    t0: -0.5,
                    t1: 0.5,
                    speed_hi: 1.0,
                    noise: 16.0 * f64::EPSILON * sum,
                    f_per_metre: 1.0,
                    f_per_metre_hi: 1.0,
                    residual_reach: None,
                    jet: None,
                };
                let residual = |t: f64| {
                    ((s[2] * (2.0 * t).sin() + c[2] * (2.0 * t).cos()) + s[1] * t.sin())
                        + (c[1] * t.cos() + c[0])
                };
                let got = certified_subdivision(&f, &residual, &frame, &ROWS, None, band);
                let key = match &got {
                    Ok(Roots::Certified(found)) => format!("certified {}", found.count()),
                    Ok(other) => format!("{other:?}").chars().take(12).collect(),
                    Err(RootRefusal::Touch(_)) => "touch".into(),
                    Err(RootRefusal::Escalated(_)) => "escalated".into(),
                    Err(RootRefusal::Unsettled(u)) => format!("{u:?}"),
                    Err(RootRefusal::Fault(fault)) => {
                        faults.push((eps, i % 3, c, s));
                        format!("FAULT {fault:?}")
                    }
                };
                *tally.entry(format!("{eps:e} {key}")).or_default() += 1;
            }
        }
        println!("{tally:#?}");
        assert!(faults.is_empty(), "legal residuals fault: {faults:?}");
    }
}
