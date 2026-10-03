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
//! - **extreme**, on `lo` and `hi` — definitely one-signed is a miss,
//!   definitely straddling is two roots, and either extreme in the zero
//!   band is a tangency, which is not a crossing at any order this lane
//!   sees and answers [`CircleRoots::Uncertain`].
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
//! # The half-angle ladder, and the subdivision that answers
//!
//! A residual that is a trigonometric polynomial `F(θ)` of degree TWO —
//! a circle against a torus's implicit, or against a cylinder wall it is
//! tilted to, an ellipse against a sphere or a wall — has at most four
//! roots round the turn. [`half_angle_roots`] answers it with
//! [`certified_subdivision`], decided on the residual itself in the
//! band's own metres (its docs: Taylor bounds on pieces, each piece
//! clear or monotone, each root bisected and read ON the surface, a
//! tangency `Uncertain`).
//!
//! The subdivision is not bound to degree two: a residual of degree `N`
//! (at most [`MAX_DEGREE`]) has at most `2N` roots, and its Taylor and
//! noise bounds read every harmonic up to `N`. An ellipse against a
//! torus's implicit is of degree four, an octic in the half-angle with no
//! ladder, and the ellipse door hands it to the subdivision alone
//! ([`super::ellipse_roots`]) with a root-slack meter ([`RootSlack`]).
//!
//! **The ladder runs first, for its escalations only.** Under the
//! tangent half-angle `t = tan(φ/2)`, `F·(1 + t²)²` is a quartic in `t`,
//! which the ray lane's certified quartic ladder decides; an in-band
//! sign it meets escalates as the door's decision, and that `Err` is
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
//! `u·T` (`T` a bound on the terms' magnitudes, [`geom_brep::HARMONIC_NOISE_ULPS`] of them
//! charged): `noise`, a residual error of `noise / f_per_metre` metres,
//! `f_per_metre` the door's floor on `|F|` per metre of residual. The
//! subdivision charges it to every Taylor term it reads (its docs), so a
//! pose whose noise is past the band answers `Uncertain` there. A
//! rounding estimate on `f64`, run on every scalar; the `Interval` lane
//! carries its own enclosure.
//!
//! The ladder meters the same noise before it runs, and passes a reading
//! in the band's gap (`work/germ/circle-torus-meters-accept-an-unreadable-reading.md`);
//! the rounding of its own later stages (the anchor rotation, the pole
//! division, the rescale, the depression) is left to its band decisions.

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
    /// A certified count (even, at most twice the residual's degree) and
    /// the carrier parameters `θ` of those roots, unordered, in
    /// `thetas[..count]`. Every `θ` lies within `π`
    /// of the arc's midpoint, so it compares with the arc `[t₀, t₁]` the
    /// caller passed without wrapping.
    Certified {
        count: usize,
        thetas: [T; 2 * MAX_DEGREE],
    },
    /// The quartic's constructed roots disagree in number with its
    /// certified count ([`TorusRoots::CountDisagrees`]).
    CountDisagrees,
}

impl<T: geom_core::Real> From<TorusRoots<T>> for CircleRoots<T> {
    /// The quartic ladder's answer in the doors' shape: it has no
    /// on-surface case (no line lies on a torus).
    fn from(roots: TorusRoots<T>) -> Self {
        match roots {
            TorusRoots::Certified { count, ts } => {
                let mut thetas = [T::zero(); 2 * MAX_DEGREE];
                thetas[..ts.len()].copy_from_slice(&ts);
                Self::Certified { count, thetas }
            }
            TorusRoots::Miss => Self::Miss,
            TorusRoots::Uncertain => Self::Uncertain,
            TorusRoots::CountDisagrees => Self::CountDisagrees,
        }
    }
}

/// The most harmonics [`certified_subdivision`] reads: a conic against a
/// torus ([`geom_brep::ConicTorusHarmonics`]).
pub(super) const MAX_DEGREE: usize = 4;

/// A trigonometric polynomial `Σₖ cos[k]·cos kθ + sin[k]·sin kθ` of
/// degree `degree ≤ MAX_DEGREE`; coefficients past `degree`, and
/// `sin[0]`, are not read.
pub(super) struct TrigPoly<T> {
    pub(super) cos: [T; MAX_DEGREE + 1],
    pub(super) sin: [T; MAX_DEGREE + 1],
    pub(super) degree: u8,
}

impl<T: geom_core::Real> TrigPoly<T> {
    /// `c₀ + c₁ cos θ + s₁ sin θ + c₂ cos 2θ + s₂ sin 2θ`.
    pub(super) fn second(c0: T, c1: T, s1: T, c2: T, s2: T) -> Self {
        let z = T::zero();
        Self {
            cos: [c0, c1, c2, z, z],
            sin: [z, s1, s2, z, z],
            degree: 2,
        }
    }
}

/// Where [`certified_subdivision`] works: the arc `[t0, t1]` its roots
/// are reported about, the carrier's top speed `|C′|` (metres per
/// radian), `noise`, a bound on the `f64` evaluation error of `F` from
/// its harmonics (in `F`'s units), and `F`'s units per metre of the
/// residual `residual` reads: `f_per_metre`, a FLOOR on
/// `|F| / |residual|` near the surface (which turns `noise` into metres),
/// and `f_per_metre_hi`, a CEILING on it along the carrier (which turns a
/// lower bound on `|F|` into one on the residual). A door whose `F` IS
/// that residual passes exactly `1` for both.
pub(super) struct SubdivisionFrame<T> {
    pub(super) t0: T,
    pub(super) t1: T,
    pub(super) speed_hi: T,
    pub(super) noise: T,
    pub(super) f_per_metre: T,
    pub(super) f_per_metre_hi: T,
}

/// **The root-slack meter** a caller of [`certified_subdivision`] may
/// hand it: `residual`, the surface's residual (metres) at a carrier
/// parameter carried with a running bound on its own rounding, and
/// `f_per_metre_hi`, a CEILING on `|F| / |residual|` near the surface
/// (where a root the band reads ON it lies).
///
/// A root located on its monotone piece is off the true one by at most
/// `|residual(θ)| + error` metres of residual — the reading's own
/// magnitude plus the bound on its rounding — over the residual's least
/// slope there, which is at least `F`'s least slope on the piece over
/// `f_per_metre_hi`. At the carrier's top speed that is an arc length,
/// and it must be definitely inside the band under `row`, or the span
/// and trim decisions the caller makes on the root are made on the wrong
/// point; an unreadable reading refuses too. A first-order bound, as the
/// running bound is.
pub(super) struct RootSlack<'a, T> {
    pub(super) row: &'static str,
    pub(super) residual: &'a dyn Fn(T) -> geom_core::Rounded<T>,
    pub(super) f_per_metre_hi: T,
}

/// The predicate rows one caller of [`half_angle_roots`] meters under.
pub(super) struct HalfAngleRows {
    /// The pole is definitely off the surface (a residual, metres).
    pub(super) pole: &'static str,
    /// The pole is well conditioned (module docs; metres).
    pub(super) conditioning: &'static str,
    /// The harmonics' rounding, as a residual, is past the band, and the
    /// ladder does not run (the noise meter; metres).
    pub(super) noise: &'static str,
    /// The quartic ladder's own rows.
    pub(super) quartic: QuarticRows,
    /// The certified subdivision's rows ([`certified_subdivision`]).
    pub(super) verify: SubdivisionRows,
    /// The decision an in-band ladder sign escalates as.
    pub(super) decision: BooleanDecision,
}

/// The predicate rows one caller of [`certified_subdivision`] meters
/// under, every one a residual in metres.
pub(super) struct SubdivisionRows {
    /// A piece is definitely root-free: `|F(m)|` exceeds the most `F` can
    /// fall over the half-width.
    pub(super) clear: &'static str,
    /// A piece is definitely monotone: `|F′(m)|` exceeds the most `F′`
    /// can fall over the half-width, read as the rise it certifies.
    pub(super) monotone: &'static str,
    /// The residual's sign at a monotone piece's ends, and ON the surface
    /// at a located root.
    pub(super) side: &'static str,
    /// A piece is still wider, as arc length, than the zero band: below
    /// it a piece that is neither clear nor monotone holds a double root
    /// the band cannot resolve.
    pub(super) width: &'static str,
}

/// Where [`half_angle_roots`] works: the subdivision's frame (`walk`),
/// the carrier's least speed `|C′|` (a circle's radius; an ellipse's
/// semi-minor axis; the top speed is `walk`'s), and the ladder's `lever`
/// (a length).
pub(super) struct HalfAngleFrame<T> {
    pub(super) walk: SubdivisionFrame<T>,
    pub(super) speed_lo: T,
    pub(super) lever: T,
}

/// The rounding charged against a term bound `terms`
/// ([`geom_brep::rounding_charge`], [`geom_brep::HARMONIC_NOISE_ULPS`]
/// half-ulps of it) — the meters' one spelling of the charge. The
/// circle × cylinder square arm charges its first harmonic the same
/// count; the circle × sphere door charges its extremes their own
/// running bounds instead (`geom_brep::CircleSphereHarmonic`).
pub(super) fn rounding_charge<T: geom_core::Real>(terms: T) -> T {
    geom_brep::rounding_charge(terms)
}

/// The conditioning floor `κ` (module docs): the pole's `|F|` must be at
/// least this share of `F`'s amplitude bound.
const POLE_CONDITIONING: f64 = 1.0 / 16.0;

/// How many evenly spaced poles round the carrier the anchor search may
/// try (module docs: 32 is what makes some candidate always pass
/// [`POLE_CONDITIONING`]).
const POLE_CANDIDATES: u32 = 32;

/// **The certified real roots of a degree-2 trigonometric polynomial
/// `F(θ)` along a conic** (module docs, "The half-angle ladder, and the
/// subdivision that answers"). General over the surface: `residual` is
/// the surface's residual (metres) along the carrier, which must share
/// `F`'s sign. The answer is [`certified_subdivision`]'s; the half-angle
/// ladder runs first and only its escalations are kept.
///
/// # Errors
///
/// The ladder's escalation as `rows.decision`, and
/// [`BooleanError::ClassificationInvariant`] for a polynomial of degree
/// past two, which the quartic does not describe.
pub(super) fn half_angle_roots<T: Decide>(
    f: &TrigPoly<T>,
    residual: impl Fn(T) -> T,
    frame: HalfAngleFrame<T>,
    rows: &HalfAngleRows,
    band: Band,
) -> Result<CircleRoots<T>, BooleanError> {
    if f.degree > 2 {
        return Err(BooleanError::ClassificationInvariant {
            what: "the half-angle ladder was handed a residual of degree past two",
        });
    }
    ladder_roots(f, &residual, &frame, rows, band)?;
    certified_subdivision(f, &residual, &frame.walk, &rows.verify, None, band)
}

/// [`half_angle_roots`]'s ladder: the quartic in the tangent half-angle
/// (module docs, "The half-angle ladder, and the subdivision that
/// answers"). It answers nothing but an escalation (the `Err`): its
/// quartic's count and roots are not read, and [`certified_subdivision`]
/// decides the roots in the residual's own metres.
#[allow(clippy::too_many_lines)] // the anchor search and the quartic, one walk
fn ladder_roots<T: Decide>(
    f: &TrigPoly<T>,
    residual: &impl Fn(T) -> T,
    frame: &HalfAngleFrame<T>,
    rows: &HalfAngleRows,
    band: Band,
) -> Result<(), BooleanError> {
    let HalfAngleFrame {
        walk:
            SubdivisionFrame {
                t0,
                t1,
                noise,
                f_per_metre,
                ..
            },
        speed_lo,
        lever,
    } = *frame;
    let (c0, c1, s1, c2, s2) = (f.cos[0], f.cos[1], f.sin[1], f.cos[2], f.sin[2]);
    // **The noise meter** (module docs, "The harmonics' noise"): where
    // the harmonics' evaluation error, `noise / f_per_metre` metres, is
    // definitely past the band's escalation threshold, the ladder does
    // not run, so it cannot escalate on its own rounding; the
    // subdivision, which charges that noise, answers alone. The stages
    // after it (rotation, pole division, rescale, depression,
    // discriminant) round again; those are left to the ladder's own band
    // decisions, which is the premise the module docs state.
    match decide(rows.noise, Margin::of(noise / f_per_metre), band) {
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
        // sign it meets is the door's escalation.
        depressed_quartic_roots(p, q, s, lever, &rows.quartic, band).map_err(|diag| {
            BooleanError::Escalated {
                decision: rows.decision,
                diag,
            }
        })?;
        return Ok(());
    }
    Ok(())
}

/// The pieces the whole turn is cut into before [`certified_subdivision`]
/// starts.
const SUBDIVISION_START: u32 = 16;

/// The most pieces [`certified_subdivision`] examines before it answers
/// `Uncertain`: a bound on the walk (D9), far above what a transverse
/// configuration needs (each simple root is isolated within a few dozen).
const SUBDIVISION_BUDGET: usize = 4096;

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
const SPLITS: [f64; 9] = [
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
/// only its escalations, and the ellipse × torus door's at `N = 4`.
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
/// (a cut that would land in the band is moved along its piece). On a
/// piece of half-width `w`:
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
///   `Zero`), or the answer is `Uncertain` — and, where the caller hands
///   a [`RootSlack`] meter, its slack must be definitely inside the band
///   too;
/// - otherwise the piece is split, until its arc length is inside the
///   band (`width`): a piece that small, neither clear nor monotone,
///   holds a double root — a tangency — and the answer is `Uncertain`,
///   never a certified `Miss`.
///
/// `F` shares the residual's sign. The clear margin is read in metres
/// through the CEILING `f_per_metre_hi`, so it is a lower bound on the
/// residual's least magnitude over the piece — read through a floor it
/// would overstate it, and a carrier within the band could read clear.
/// The harmonics are rounded: `F` read from them is the
/// true one to within `noise`, so its `k`-th derivative to within
/// `Nᵏ·noise` (Bernstein's inequality for a trigonometric polynomial of
/// degree `N`), and each Taylor term above is charged its own share.
#[allow(clippy::too_many_lines)] // one walk: the cut, the pieces, the bisection
pub(super) fn certified_subdivision<T: Decide>(
    f: &TrigPoly<T>,
    residual: &impl Fn(T) -> T,
    frame: &SubdivisionFrame<T>,
    rows: &SubdivisionRows,
    slack: Option<&RootSlack<'_, T>>,
    band: Band,
) -> Result<CircleRoots<T>, BooleanError> {
    let SubdivisionFrame {
        t0,
        t1,
        speed_hi,
        noise,
        f_per_metre_hi,
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
    let slope = |t: T| derivative(t, 1);
    let bend = |t: T| derivative(t, 2);
    let jerk = |t: T| derivative(t, 3);
    let (six, twenty_four) = (T::from_f64(6.0), T::from_f64(24.0));
    let definitely = |row, m: T| matches!(decide(row, Margin::of(m), band), Ok(Sign::Positive));
    let side = |t: T| match decide(rows.side, Margin::of(residual(t)), band) {
        Ok(s @ (Sign::Positive | Sign::Negative)) => Some(s),
        Ok(Sign::Zero) | Err(_) => None,
    };
    // The first cut: one end in each sixteenth of the turn, each moved
    // along its sixteenth until its sign is definite; the last end is the
    // first a turn on.
    let n = SUBDIVISION_START;
    let piece = T::tau() / T::from_f64(f64::from(n));
    let base = (t0 + t1) / two - T::pi();
    let Some(mut ends) = (0..n)
        .map(|k| {
            let from = base + piece * T::from_f64(f64::from(k));
            SPLITS.iter().find_map(|&share| {
                let t = from + piece * T::from_f64(share);
                side(t).map(|s| (t, s))
            })
        })
        .collect::<Option<Vec<_>>>()
    else {
        return Ok(CircleRoots::Uncertain);
    };
    let (first, first_side) = ends[0];
    ends.push((first + T::tau(), first_side));
    // Popped left to right, so the roots come out in order.
    let mut pieces: Vec<((T, Sign), (T, Sign))> =
        ends.windows(2).rev().map(|w| (w[0], w[1])).collect();
    let mut roots: Vec<T> = Vec::new();
    let mut examined = 0usize;
    while let Some(((l, sl), (r, sr))) = pieces.pop() {
        examined += 1;
        if examined > SUBDIVISION_BUDGET {
            return Ok(CircleRoots::Uncertain);
        }
        let half = (r - l) / two;
        let m = l + half;
        // The true `F`'s derivatives at `m`, each read from the rounded
        // harmonics and widened by its own share of their error: `k`
        // derivatives of an error bounded by `noise` are bounded by
        // `Nᵏ·noise` (Bernstein's inequality, degree `N`).
        let (d1, d2, d3) = (slope(m).abs(), bend(m).abs(), jerk(m).abs());
        let (d1_lo, d1_hi) = (d1 - deg * noise, d1 + deg * noise);
        let d2_hi = d2 + deg.powi(2) * noise;
        let d3_hi = d3 + deg.powi(3) * noise;
        let fall = d1_hi * half
            + d2_hi * half.powi(2) / two
            + d3_hi * half.powi(3) / six
            + fourth_hi * half.powi(4) / twenty_four;
        if definitely(rows.clear, (value(m).abs() - fall - noise) / f_per_metre_hi) {
            continue;
        }
        let most_bend = d2_hi + d3_hi * half + fourth_hi * half.powi(2) / two;
        let least_slope =
            d1_lo - d2_hi * half - d3_hi * half.powi(2) / two - fourth_hi * half.powi(3) / six;
        if definitely(rows.monotone, speed_hi * least_slope / most_bend) {
            if sl == sr {
                continue;
            }
            // Bisected on the residual evaluated at the point, not on `F`
            // from its harmonics: the two share their zero set, and near a
            // graze `|F′|` is small enough that the harmonics' rounding
            // moves `F`'s sign change along the arc by more than the band.
            let (mut lo, mut hi) = (l, r);
            let at_lo = residual(l);
            for _ in 0..BISECTIONS {
                let c = (lo + hi) / two;
                let product = residual(c) * at_lo;
                lo = product.select_le_zero(lo, c);
                hi = product.select_le_zero(c, hi);
            }
            // Reported within `π` of the arc's midpoint, so a caller
            // compares it with `[t0, t1]` directly: the first cut's ends
            // are moved along their pieces, so the last piece runs past
            // the half-turn.
            let mid = (t0 + t1) / two;
            let root = mid + ((lo + hi) / two - mid).reduce_periodic_centred(T::tau());
            match decide(rows.side, Margin::of(residual(root)), band) {
                Ok(Sign::Zero) => {}
                _ => return Ok(CircleRoots::Uncertain),
            }
            if let Some(meter) = slack {
                // `|F|` at the root, true to within `reach`: the true root
                // lies within `reach / L` of it wherever `|F′| ≥ L` holds
                // that far. `L` is read on a window about the root twice
                // that width at the root's own slope — valid when the
                // slack it gives fits inside it — and otherwise on the
                // whole piece, which holds the true root.
                let reach = meter.f_per_metre_hi * (meter.residual)(root).magnitude();
                let (e1, e2, e3) = (slope(root).abs(), bend(root).abs(), jerk(root).abs());
                let near = e1 - deg * noise;
                let w = (two * reach / near).min(half).max(T::zero());
                let window = near
                    - (e2 + deg.powi(2) * noise) * w
                    - (e3 + deg.powi(3) * noise) * w.powi(2) / two
                    - fourth_hi * w.powi(3) / six;
                let lever =
                    (reach - window * w).select_le_zero(window.max(least_slope), least_slope);
                let arc = speed_hi * reach / lever;
                match decide(meter.row, Margin::of(arc), band) {
                    Ok(Sign::Zero | Sign::Negative) => {}
                    Ok(Sign::Positive) | Err(_) => return Ok(CircleRoots::Uncertain),
                }
            }
            roots.push(root);
            continue;
        }
        if !definitely(rows.width, speed_hi * (r - l)) {
            return Ok(CircleRoots::Uncertain);
        }
        let Some((c, sc)) = SPLITS.iter().find_map(|&share| {
            let c = l + (r - l) * T::from_f64(share);
            side(c).map(|s| (c, s))
        }) else {
            return Ok(CircleRoots::Uncertain);
        };
        pieces.push(((c, sc), (r, sr)));
        pieces.push(((l, sl), (c, sc)));
    }
    // Round the closed turn the ends' signs change an even number of
    // times, and every change on a monotone piece was bisected; an odd
    // count therefore means a change on a piece read CLEAR — the residual
    // and its harmonics disagreeing by more than `noise` — and more than
    // `2N` is more than a degree-`N` polynomial has. Either way a root was
    // lost or invented, and nothing is certified.
    if roots.len() > 2 * n_deg || roots.len() % 2 == 1 {
        return Ok(CircleRoots::Uncertain);
    }
    let count = roots.len();
    if count == 0 {
        return Ok(CircleRoots::Miss);
    }
    let mut thetas = [T::zero(); 2 * MAX_DEGREE];
    thetas[..count].copy_from_slice(&roots);
    Ok(CircleRoots::Certified { count, thetas })
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

/// **The certified roots of a first-harmonic residual along a carrier
/// whose speed `|C′(θ)|` is at most `speed`** (a circle's radius, an
/// ellipse's semi-major axis), reported within `π` of the midpoint of `[t0, t1]`: the
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
    let slack = speed * (at_root / slope + phase + rounding_charge(T::tau()));
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
        thetas: {
            let mut thetas = [T::zero(); 2 * MAX_DEGREE];
            thetas[0] = near_mid(phi - half_chord);
            thetas[1] = near_mid(phi + half_chord);
            thetas
        },
    })
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
        clear: "bool_ellipse_sub_clear",
        monotone: "bool_ellipse_sub_monotone",
        side: "bool_ellipse_sub_side",
        width: "bool_ellipse_sub_width",
    };

    fn walk(
        f: &TrigPoly<f64>,
        residual: &impl Fn(f64) -> f64,
        noise: f64,
        eps: f64,
    ) -> CircleRoots<f64> {
        let frame = SubdivisionFrame {
            t0: -0.5,
            t1: 0.5,
            speed_hi: 1.0,
            noise,
            f_per_metre: 1.0,
            f_per_metre_hi: 1.0,
        };
        certified_subdivision(
            f,
            residual,
            &frame,
            &ROWS,
            None,
            Band::new(eps, 10.0 * eps).unwrap(),
        )
        .unwrap()
    }

    /// The residual's sign changes round the turn, read on a grid fine
    /// enough for the residuals below.
    fn sign_changes(residual: &impl Fn(f64) -> f64) -> usize {
        let n = 200_000_u32;
        let samples: Vec<f64> = (0..=n)
            .map(|k| residual(-PI + 2.0 * PI * f64::from(k) / f64::from(n)))
            .collect();
        crate::boolean::conic_oracle::sign_changes(&samples)
    }

    fn sine(s1: f64) -> TrigPoly<f64> {
        TrigPoly::second(0.0, 0.0, s1, 0.0, 0.0)
    }

    /// **The monotone test charges `F′` its share of the noise.** `F` read
    /// from the harmonics is `10⁻³·sin θ`; the true residual differs from
    /// it by at most `0.7·10⁻³` — inside the declared `noise` of
    /// `0.8·10⁻³` — as a wiggle about `θ = 0` that crosses zero several
    /// times. `F′` there is `10⁻³`, less than the `2·noise` its rounding
    /// may move it by, so the piece is not certified monotone and the
    /// answer is `Uncertain`. Charged nothing, the piece reads monotone,
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
            CircleRoots::Certified { count, .. } => {
                assert_eq!(count, truth, "certified {count} roots of {truth}");
            }
            CircleRoots::Uncertain => {}
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
        let f = TrigPoly::second(-3.0 + 3e-4, 4.0, 0.0, -1.0, 0.0);
        let value = |t: f64| f.cos[0] + f.cos[1] * t.cos() + f.cos[2] * (2.0 * t).cos();
        assert_eq!(sign_changes(&value), 2);
        match walk(&f, &value, 0.0, 1e-9) {
            CircleRoots::Certified { count, thetas } => {
                assert_eq!(count, 2, "certified {count} roots of 2");
                for &t in &thetas[..count] {
                    assert!(value(t).abs() <= 1e-9, "root {t} reads {:e}", value(t));
                }
            }
            CircleRoots::Uncertain => {}
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
        };
        let band = Band::new(1e-9, 1e-8).unwrap();
        match certified_subdivision(&f, &value, &frame, &ROWS, None, band).unwrap() {
            CircleRoots::Certified { count, .. } => {
                assert_eq!(count, truth, "certified {count} roots of {truth}");
            }
            CircleRoots::Uncertain => {}
            other => panic!("{other:?} for a residual crossing {truth} times"),
        }
    }

    /// **A located root must read ON the surface.** The residual is
    /// `sin θ` resolved only to steps of `20ε` — never zero, its sign
    /// change a jump from `−10ε` to `+10ε` — the harmonics exact to within
    /// that step (declared as the noise). Bisection closes on the jump,
    /// where the residual reads `10ε` off, so the answer is `Uncertain`;
    /// not checked, the jump is certified a root.
    #[test]
    fn a_root_that_reads_off_the_surface_is_not_certified() {
        let eps = 1e-9;
        let step = 20.0 * eps;
        let residual = |t: f64| step * ((t.sin() / step).floor() + 0.5);
        let got = walk(&sine(1.0), &residual, step, eps);
        if let CircleRoots::Certified { count, thetas } = got {
            for &t in &thetas[..count] {
                let off = residual(t).abs();
                assert!(off <= eps, "root {t} reads {off:e} off the surface");
            }
        }
    }

    /// **An odd count is not certified.** The harmonics read `sin θ`; the
    /// residual is `−1` from `θ = −0.05` to `0.4` and `sin θ` elsewhere —
    /// a disagreement no declared noise covers — so its one sign change
    /// near the arc falls on a piece the harmonics read clear (the first
    /// cut's piece from `0.185` to `0.578`) and is lost, while the change
    /// at `π` is bisected. Round a closed turn that leaves an odd count,
    /// which is `Uncertain`; not checked, one root is certified for a
    /// residual that crosses twice.
    #[test]
    fn an_odd_count_is_not_certified() {
        let residual = |t: f64| {
            let t = (t + PI).rem_euclid(2.0 * PI) - PI;
            if t > -0.05 && t < 0.4 { -1.0 } else { t.sin() }
        };
        assert_eq!(sign_changes(&residual), 2);
        match walk(&sine(1.0), &residual, 0.0, 1e-9) {
            CircleRoots::Certified { count, .. } => {
                assert_eq!(count % 2, 0, "certified an odd count, {count}");
            }
            CircleRoots::Uncertain => {}
            other => panic!("{other:?} for a residual crossing twice"),
        }
    }
}
