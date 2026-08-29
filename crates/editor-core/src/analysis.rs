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
        (self.nominal + self.offsets.lo, self.nominal + self.offsets.hi)
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
    Ok(1.0 - interval_mass(param, dist, (analyzed.lo, analyzed.hi))?)
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
            let Some((a, b)) = (OffsetInterval { lo: tlo, hi: thi })
                .overlap(&OffsetInterval { lo, hi })
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

/// `P(lo <= X <= hi)` for `X ~ N(0, sigma²)`, via `erf`.
fn normal_mass(sigma: f64, lo: f64, hi: f64) -> f64 {
    let scale = sigma * core::f64::consts::SQRT_2;
    0.5 * (libm::erf(hi / scale) - libm::erf(lo / scale))
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
/// Monotone bisection on `libm::erf` — deterministic, dependency-free,
/// and with no tolerance in sight. `erf` is strictly increasing, so
/// the bracket `[0, hi]` is found by doubling and the interval halves
/// to the `f64` grid: the loop runs until the bracket can no longer be
/// split, so the result is the best `f64` the predicate admits rather
/// than an ε-decided approximation.
///
/// `mass` is a policy value already checked into `(0, 1)`.
fn quantile_z(mass: f64) -> f64 {
    let f = |z: f64| libm::erf(z / core::f64::consts::SQRT_2);
    let mut hi = 1.0;
    // `erf` saturates to 1.0 in f64 near z ≈ 5.9σ, so this terminates
    // for every representable mass < 1.
    while f(hi) < mass && hi < 1e3 {
        hi *= 2.0;
    }
    let mut lo = 0.0;
    loop {
        let mid = 0.5 * (lo + hi);
        if mid <= lo || mid >= hi {
            return hi;
        }
        if f(mid) < mass {
            lo = mid;
        } else {
            hi = mid;
        }
    }
}
