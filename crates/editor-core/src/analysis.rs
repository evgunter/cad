//! The analysis lane (ERROR-DESIGN E1): the ONE place a
//! [`Distribution`] is read.
//!
//! **The boundary sentence.** The kernel and geometry lanes never see
//! a probability, and no `Real` instantiation carries one. Everything
//! below this module — expression evaluation, the interval channel,
//! the naming and resolve layers — works on the document's nominal
//! values and on boxes. This module projects a distribution to the
//! consumables that live ABOVE that line: the **analyzed box** for
//! interval and driver work, and **mass** for pricing leaves and tail
//! in reports. Nothing here persists; every value is derived.
//!
//! **Independence** (PL6 / E2): one distribution per parameter, and
//! the joint law is the PRODUCT of the marginals. Two slots driven by
//! the same parameter name share one marginal and comove exactly;
//! distinct names are independent; a derived expression comoves
//! through evaluation and carries no distribution of its own. Joint
//! forms are foreclosed in v1 (E11.2).
//!
//! **Distributions are opt-in, and the analysis varies exactly what
//! the user declared variable.** A continuous parameter with NO
//! distribution is FIXED: its analyzed interval has width zero at the
//! nominal and it contributes mass 1. That is a modelling statement,
//! not a fallback — nothing here guesses a spread for a parameter
//! whose author did not state one. `Count` parameters are structural
//! and are not box axes at all (E0's term hygiene).
//!
//! **The analyzed box is the analysis's knob, not the distribution's
//! property** (E2). A [`Normal`](Distribution::Normal) has unbounded
//! support; the box is the symmetric quantile interval for
//! [`AnalysisPolicy::quantile_mass`], and the mass outside it is
//! reported by [`tail_mass`] rather than dropped. Moving the knob
//! moves mass between the analyzed and tail columns; it never moves
//! truth.
//!
//! **No ε here.** The bisection below is reporting-lane `f64`
//! arithmetic: it decides no geometric predicate, funnels nothing,
//! and consults no tolerance. Its convergence error only shifts mass
//! between the analyzed and the tail column.
//!
//! **One standard normal, two spellings, no subtraction of ones.**
//! Every normal quantity here — the mass inside an interval, the mass
//! outside it, and the `z` the quantile box is drawn at — goes through
//! [`std_normal_mass`] and [`std_normal_exterior`], which are written
//! so that neither ever forms `1 - x` for a small `x`: the exterior is
//! the SUM of two `erfc` half-lines and the interior differences
//! `erfc` (not `erf`) whenever the interval lies to one side of the
//! mean. That is what makes "reported, never dropped" true in the deep
//! tail, where `1 - 0.5·(erf(hi) - erf(lo))` cancels to a bit-exact
//! zero over mass that is really there. The box door and the tail
//! column consult the SAME pair, so the analyzed interval always holds
//! exactly the mass the tail complements.

use std::collections::BTreeMap;

use crate::distribution::Distribution;
use crate::doc::{Doc, DocParam, ParamName};
use crate::expr::Dimension;

/// The ±3σ convention: the default share of a parameter's mass the
/// analyzed box is asked to cover, per parameter.
///
/// A recorded policy dial (E2), not a constant of nature — a request
/// may name any mass in `(0, 1)`.
pub const DEFAULT_QUANTILE_MASS: f64 = 0.9973;

/// How a run chooses its analyzed box: request configuration, never a
/// global (E2).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AnalysisPolicy {
    quantile_mass: f64,
}

/// A policy that cannot be honoured.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AnalysisPolicyError {
    /// `quantile_mass` is not a finite number strictly inside
    /// `(0, 1)`: mass 1 asks for an infinite box, mass 0 for an empty
    /// one, and neither is a box.
    QuantileMassOutOfRange {
        /// The requested mass.
        mass: f64,
    },
}

impl core::fmt::Display for AnalysisPolicyError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::QuantileMassOutOfRange { mass } => write!(
                f,
                "analysis quantile mass must be finite and strictly inside (0, 1), got {mass}"
            ),
        }
    }
}

impl Default for AnalysisPolicy {
    fn default() -> Self {
        Self {
            quantile_mass: DEFAULT_QUANTILE_MASS,
        }
    }
}

impl AnalysisPolicy {
    /// A policy asking the box to cover `quantile_mass` of each
    /// unbounded parameter's mass.
    pub fn new(quantile_mass: f64) -> Result<Self, AnalysisPolicyError> {
        if !(quantile_mass.is_finite() && quantile_mass > 0.0 && quantile_mass < 1.0) {
            return Err(AnalysisPolicyError::QuantileMassOutOfRange {
                mass: quantile_mass,
            });
        }
        Ok(Self { quantile_mass })
    }

    /// The requested per-parameter mass.
    pub fn quantile_mass(&self) -> f64 {
        self.quantile_mass
    }
}

/// An offset interval around a parameter's nominal, in the
/// parameter's own dimension. `lo <= 0 <= hi` always.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct OffsetInterval {
    /// Lower offset (`<= 0`).
    pub lo: f64,
    /// Upper offset (`>= 0`).
    pub hi: f64,
}

impl OffsetInterval {
    /// The width-zero interval at the nominal — a FIXED parameter.
    pub const FIXED: Self = Self { lo: 0.0, hi: 0.0 };

    /// Whether this axis is fixed (width zero at the nominal).
    pub fn is_fixed(&self) -> bool {
        self.lo == 0.0 && self.hi == 0.0
    }

    /// `hi - lo`.
    pub fn width(&self) -> f64 {
        self.hi - self.lo
    }

    /// The overlap with `other`, or `None` when they do not meet.
    /// Not itself an [`OffsetInterval`]: an intersection need not
    /// contain the nominal.
    fn overlap(&self, other: &Self) -> Option<(f64, f64)> {
        let lo = self.lo.max(other.lo);
        let hi = self.hi.min(other.hi);
        (lo <= hi).then_some((lo, hi))
    }
}

/// One axis of the analyzed box: the parameter's nominal, the offset
/// interval the analysis varies it over, and the distribution that
/// interval came from (`None` = a fixed parameter).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AnalyzedParam {
    /// The parameter's declared dimension.
    pub dim: Dimension,
    /// The document's nominal value — the single source of truth for
    /// the `f64` build (E2).
    pub nominal: f64,
    /// The analyzed offsets around `nominal`.
    pub offsets: OffsetInterval,
    /// The parameter's distribution, if it declared one.
    pub distribution: Option<Distribution>,
}

impl AnalyzedParam {
    /// The analyzed interval in absolute parameter values.
    pub fn absolute(&self) -> (f64, f64) {
        (
            self.nominal + self.offsets.lo,
            self.nominal + self.offsets.hi,
        )
    }
}

/// The analyzed box: one axis per CONTINUOUS document parameter, in
/// name order. Derived, never stored.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct AnalyzedBox {
    params: BTreeMap<ParamName, AnalyzedParam>,
}

impl AnalyzedBox {
    /// The axes, by parameter name.
    pub fn params(&self) -> &BTreeMap<ParamName, AnalyzedParam> {
        &self.params
    }

    /// One axis, by name.
    pub fn get(&self, name: &ParamName) -> Option<&AnalyzedParam> {
        self.params.get(name)
    }

    /// The axes that actually vary — the box's non-degenerate
    /// dimensions.
    pub fn varying(&self) -> impl Iterator<Item = (&ParamName, &AnalyzedParam)> {
        self.params.iter().filter(|(_, p)| !p.offsets.is_fixed())
    }

    /// The tail mass of ONE named axis: what this box's own interval
    /// for `name` leaves outside. `None` if the document has no such
    /// continuous parameter.
    ///
    /// The reason this exists beside the free [`tail_mass`]: that door
    /// takes a name, a distribution and an interval as three loose
    /// arguments, and nothing stops a caller pairing one parameter's
    /// distribution with another's box — a mistake that produces a
    /// plausible number rather than a refusal. Here the three come
    /// from one axis of one box, so they cannot disagree. The free
    /// doors stay, because a driver pricing a leaf is asking about an
    /// interval that is deliberately NOT the analyzed one.
    ///
    /// An unannotated axis is FIXED and its tail is `0.0` — the
    /// analysis is not leaving anything out, because nothing was
    /// declared to vary.
    pub fn axis_tail_mass(&self, name: &ParamName) -> Option<Result<f64, MeasureUnavailable>> {
        let axis = self.params.get(name)?;
        Some(match axis.distribution {
            Some(dist) => tail_mass(name, &dist, &axis.offsets),
            None => Ok(0.0),
        })
    }

    /// The mass ONE named axis puts inside `sub`, with the axis's own
    /// distribution — the same pairing guarantee
    /// [`Self::axis_tail_mass`] gives, for the leaf-pricing door.
    /// `None` if the document has no such continuous parameter.
    ///
    /// An unannotated axis is a point mass at its nominal, so it
    /// answers `1.0` for any `sub` containing offset zero and `0.0`
    /// otherwise.
    pub fn axis_box_mass(
        &self,
        name: &ParamName,
        sub: (f64, f64),
    ) -> Option<Result<f64, MeasureUnavailable>> {
        let axis = self.params.get(name)?;
        Some(match axis.distribution {
            Some(dist) => box_mass(name, &dist, sub),
            None => Ok(if sub.0 <= 0.0 && 0.0 <= sub.1 {
                1.0
            } else {
                0.0
            }),
        })
    }
}

/// The analyzed box of a document under a policy (E1's first
/// consumable).
///
/// Per continuous parameter: the bounded support for
/// [`Band`](Distribution::Band),
/// [`Uniform`](Distribution::Uniform) and
/// [`TruncatedNormal`](Distribution::TruncatedNormal); the symmetric
/// quantile interval `±z·sigma` for [`Normal`](Distribution::Normal);
/// and [`OffsetInterval::FIXED`] for a parameter with no distribution.
/// `Count` parameters are not axes.
pub fn analyzed_box<P>(doc: &Doc<P>, policy: &AnalysisPolicy) -> AnalyzedBox {
    let z = quantile_z(policy.quantile_mass());
    let params = doc
        .params()
        .iter()
        .filter_map(|(name, p)| match *p {
            DocParam::Continuous {
                dim,
                value,
                distribution,
            } => {
                let offsets = match distribution {
                    None => OffsetInterval::FIXED,
                    // The bounded forms ARE their own analyzed
                    // interval: the box is the support, and no mass
                    // escapes it.
                    Some(
                        Distribution::Band { lo, hi }
                        | Distribution::Uniform { lo, hi }
                        | Distribution::TruncatedNormal { lo, hi, .. },
                    ) => OffsetInterval { lo, hi },
                    // The one unbounded form: the box is the
                    // analysis's choice, and the mass it leaves out is
                    // `tail_mass`'s to report.
                    Some(Distribution::Normal { sigma }) => OffsetInterval {
                        lo: -z * sigma,
                        hi: z * sigma,
                    },
                };
                Some((
                    name.clone(),
                    AnalyzedParam {
                        dim,
                        nominal: value,
                        offsets,
                        distribution,
                    },
                ))
            }
            DocParam::Count { .. } => None,
        })
        .collect();
    AnalyzedBox { params }
}

/// Why a mass could not be computed.
#[derive(Debug, Clone, PartialEq)]
pub enum MeasureUnavailable {
    /// The parameter carries a [`Band`](Distribution::Band): limits
    /// without a shape. "I know the limits but not the shape" is real
    /// information, and no report may quietly promote it to uniform
    /// (E2) — so anything needing a measure over this parameter
    /// refuses, naming it.
    BandHasNoMeasure {
        /// The parameter whose band blocked the pricing.
        param: ParamName,
    },
}

impl core::fmt::Display for MeasureUnavailable {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::BandHasNoMeasure { param } => write!(
                f,
                "parameter {:?} carries a band: worst-case limits with no shape, so it \
                 prices nothing — state a distribution to ask for mass",
                param.0
            ),
        }
    }
}

/// The mass a distribution puts OUTSIDE `analyzed` (E1's third
/// consumable, tail half; E2's tail-mass accounting).
///
/// Exactly `0.0` for a bounded form whose support `analyzed`
/// contains — including a [`Band`](Distribution::Band), where zero is
/// the one answer EVERY measure consistent with the band agrees on,
/// and therefore not a shape claim. A band whose support escapes the
/// interval refuses: how much of it escapes is precisely what a band
/// does not say.
pub fn tail_mass(
    param: &ParamName,
    dist: &Distribution,
    analyzed: &OffsetInterval,
) -> Result<f64, MeasureUnavailable> {
    exterior_mass(param, dist, (analyzed.lo, analyzed.hi))
}

/// The mass a distribution puts INSIDE `sub` — the leaf-pricing door
/// (E1's third consumable, leaf half; E6 consumes it).
///
/// `sub` is any interval of offsets, typically a driver leaf inside
/// the analyzed box. [`Band`](Distribution::Band) refuses, naming the
/// parameter, unless `sub` covers its whole support (mass 1) or misses
/// it entirely (mass 0) — the two answers that hold for every measure
/// on the band.
pub fn box_mass(
    param: &ParamName,
    dist: &Distribution,
    sub: (f64, f64),
) -> Result<f64, MeasureUnavailable> {
    interval_mass(param, dist, sub)
}

/// The shared kernel of both mass doors: `P(offset ∈ [lo, hi])`.
fn interval_mass(
    param: &ParamName,
    dist: &Distribution,
    (lo, hi): (f64, f64),
) -> Result<f64, MeasureUnavailable> {
    // An empty interval holds no mass, whichever measure it is under.
    if lo > hi {
        return Ok(0.0);
    }
    match *dist {
        Distribution::Band { lo: blo, hi: bhi } => {
            if lo <= blo && bhi <= hi {
                // The whole support is inside: every measure on the
                // band answers 1.
                Ok(1.0)
            } else if hi < blo || bhi < lo {
                // Disjoint from the support: every measure answers 0.
                Ok(0.0)
            } else {
                Err(MeasureUnavailable::BandHasNoMeasure {
                    param: param.clone(),
                })
            }
        }
        Distribution::Uniform { lo: ulo, hi: uhi } => {
            let support = OffsetInterval { lo: ulo, hi: uhi };
            let Some((a, b)) = support.overlap(&OffsetInterval { lo, hi }) else {
                return Ok(0.0);
            };
            let width = support.width();
            // A zero-width uniform is a point mass at the nominal;
            // the overlap either contains it or does not.
            Ok(if width > 0.0 {
                clamp_unit((b - a) / width)
            } else {
                1.0
            })
        }
        Distribution::Normal { sigma } => Ok(clamp_unit(normal_mass(sigma, lo, hi))),
        Distribution::TruncatedNormal {
            sigma,
            lo: tlo,
            hi: thi,
        } => {
            let Some((a, b)) =
                (OffsetInterval { lo: tlo, hi: thi }).overlap(&OffsetInterval { lo, hi })
            else {
                return Ok(0.0);
            };
            let total = normal_mass(sigma, tlo, thi);
            // The truncation window carries positive normal mass
            // whenever it has positive width, and `check` forbids
            // `sigma <= 0`; a zero-width window is the point mass at
            // the nominal, which every window containing it holds
            // whole.
            Ok(if total > 0.0 {
                clamp_unit(normal_mass(sigma, a, b) / total)
            } else {
                1.0
            })
        }
    }
}

/// The mass a distribution puts OUTSIDE `[lo, hi]`, computed from the
/// exterior itself rather than as `1 - inside`.
///
/// The two are the same number in exact arithmetic and are NOT the
/// same number in `f64`: `1 - inside` has no relative precision left
/// once `inside` rounds to 1, which for a normal happens while there
/// is still real mass outside the interval. Every arm below therefore
/// sums the pieces that lie outside.
fn exterior_mass(
    param: &ParamName,
    dist: &Distribution,
    (lo, hi): (f64, f64),
) -> Result<f64, MeasureUnavailable> {
    // An empty interval leaves everything outside, whichever measure.
    if lo > hi {
        return Ok(1.0);
    }
    match *dist {
        // The band's two set-theoretic answers, complemented: the
        // partial overlap that `interval_mass` refuses is refused
        // here too, and for the same reason.
        Distribution::Band { .. } => Ok(1.0 - interval_mass(param, dist, (lo, hi))?),
        Distribution::Uniform { lo: ulo, hi: uhi } => {
            let support = OffsetInterval { lo: ulo, hi: uhi };
            let width = support.width();
            if width > 0.0 {
                // The support's own two exterior pieces, measured and
                // added — never one minus the middle.
                let below = (lo.min(uhi).max(ulo) - ulo).max(0.0);
                let above = (uhi - hi.max(ulo).min(uhi)).max(0.0);
                Ok(clamp_unit((below + above) / width))
            } else {
                // A point mass at the nominal: it is inside or it is
                // not.
                Ok(if lo <= ulo && ulo <= hi { 0.0 } else { 1.0 })
            }
        }
        Distribution::Normal { sigma } => {
            Ok(clamp_unit(std_normal_exterior(lo / sigma, hi / sigma)))
        }
        Distribution::TruncatedNormal {
            sigma,
            lo: tlo,
            hi: thi,
        } => {
            let support = OffsetInterval { lo: tlo, hi: thi };
            let Some((a, b)) = support.overlap(&OffsetInterval { lo, hi }) else {
                return Ok(1.0);
            };
            let total = std_normal_mass(tlo / sigma, thi / sigma);
            if total > 0.0 {
                // The truncation window minus the overlap is two
                // sub-windows; each is measured directly, so the
                // numerator never subtracts two near-equal masses.
                let outside = std_normal_mass(tlo / sigma, a / sigma)
                    + std_normal_mass(b / sigma, thi / sigma);
                Ok(clamp_unit(outside / total))
            } else {
                // A zero-width window is the point mass at the
                // nominal, held whole by any window containing it.
                Ok(0.0)
            }
        }
    }
}

/// `P(lo <= X <= hi)` for `X ~ N(0, sigma²)`.
fn normal_mass(sigma: f64, lo: f64, hi: f64) -> f64 {
    std_normal_mass(lo / sigma, hi / sigma)
}

/// `1 / √2`, the argument scale that turns standard deviations into
/// `erf`/`erfc` arguments.
const ERF_SCALE: f64 = core::f64::consts::FRAC_1_SQRT_2;

/// `P(z_lo <= Z <= z_hi)` for the STANDARD normal `Z`, in standard
/// deviations.
///
/// Three branches, one reason: `erf` loses its relative precision
/// where it saturates and `erfc` loses its where it approaches 1, so
/// each half-line is measured by whichever of the two is still
/// resolving there. An interval on one side of the mean differences
/// `erfc` (both terms small, no cancellation); an interval straddling
/// the mean adds two `erf` values (both positive, no cancellation).
fn std_normal_mass(z_lo: f64, z_hi: f64) -> f64 {
    // A degenerate or empty interval holds no mass of a continuous
    // measure.
    if z_hi <= z_lo {
        return 0.0;
    }
    if z_lo >= 0.0 {
        0.5 * (libm::erfc(z_lo * ERF_SCALE) - libm::erfc(z_hi * ERF_SCALE))
    } else if z_hi <= 0.0 {
        0.5 * (libm::erfc(-z_hi * ERF_SCALE) - libm::erfc(-z_lo * ERF_SCALE))
    } else {
        0.5 * (libm::erf(-z_lo * ERF_SCALE) + libm::erf(z_hi * ERF_SCALE))
    }
}

/// `P(Z < z_lo) + P(Z > z_hi)` for the STANDARD normal `Z` — the mass
/// the interval `[z_lo, z_hi]` leaves out, as the SUM of its two
/// exterior half-lines.
///
/// This is the whole point of the pair: `erfc` keeps its relative
/// precision arbitrarily far into the tail, so a `±9σ` box reports the
/// `~2e-19` of mass it excludes instead of the bit-exact zero that
/// `1 - erf(9/√2)` rounds to. The number under-flows to zero only when
/// the mass itself under-flows, at roughly `±38σ`.
fn std_normal_exterior(z_lo: f64, z_hi: f64) -> f64 {
    0.5 * (libm::erfc(-z_lo * ERF_SCALE) + libm::erfc(z_hi * ERF_SCALE))
}

/// Rounding can push a probability a few ulps outside `[0, 1]`; the
/// accounting columns are reported as probabilities, so clamp.
fn clamp_unit(p: f64) -> f64 {
    p.clamp(0.0, 1.0)
}

/// The half-width, in standard deviations, of the symmetric interval
/// holding `mass` of a normal: the `z` solving
/// `erf(z / √2) = mass`.
///
/// Monotone bisection on [`std_normal_mass`] — the SAME measure the
/// mass doors report, so the box this draws holds exactly the mass
/// [`box_mass`] will later say it holds, and [`tail_mass`] complements
/// exactly that. Deterministic, dependency-free, and with no tolerance
/// in sight: the bracket halves to the `f64` grid and the loop runs
/// until it can no longer be split, so the result is the smallest `f64`
/// the predicate admits rather than an ε-decided approximation.
///
/// The bracket is FIXED rather than found by doubling, and that closes
/// the one failure this function could have had. `mass` is a policy
/// value already checked into `(0, 1)`, and `std_normal_mass(-z, z)`
/// reaches exactly `1.0` in `f64` by `z ≈ 8.3`, so [`Z_BRACKET`]
/// satisfies the predicate for EVERY admissible mass: there is no
/// "the bracket ran out" branch to fall out of with a silently wrong
/// `z`.
fn quantile_z(mass: f64) -> f64 {
    let holds = |z: f64| std_normal_mass(-z, z) >= mass;
    debug_assert!(
        holds(Z_BRACKET),
        "the fixed bracket must satisfy every mass a policy admits"
    );
    let (mut lo, mut hi) = (0.0, Z_BRACKET);
    loop {
        let mid = 0.5 * (lo + hi);
        if mid <= lo || mid >= hi {
            return hi;
        }
        if holds(mid) {
            hi = mid;
        } else {
            lo = mid;
        }
    }
}

/// The upper end of [`quantile_z`]'s bracket, in standard deviations.
///
/// Comfortably past the `z ≈ 8.3` where a standard normal's symmetric
/// interval mass rounds to exactly `1.0` in `f64`, so it holds any
/// mass strictly below 1. Widening it costs one bisection step and
/// nothing else.
const Z_BRACKET: f64 = 16.0;
