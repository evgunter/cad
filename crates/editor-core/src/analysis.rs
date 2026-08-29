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

/// **A scalar that can carry a parameter-box axis** — the door INTO the
/// lane, as [`geom_core::Bounds`] is the door out.
///
/// [`Self::axis`] answers the tightest value of `Self` enclosing the
/// OFFSET span `[lo, hi]`, or `None` when this scalar has no such value.
/// Offsets rather than absolute values, because the caller adds the
/// nominal in the scalar's own arithmetic: at the interval scalar
/// `nominal + [lo, hi]` rounds OUTWARD and therefore encloses the true
/// span, where a pre-rounded `[nominal + lo, nominal + hi]` computed in
/// `f64` may not.
///
/// A point scalar answers `None` for a widened axis, and that is the
/// whole content of the trait: an `f64` build over a box with any width
/// in it is not a narrower answer, it is a different question, and the
/// evaluation service refuses it rather than silently evaluating at the
/// nominals.
pub trait AxisScalar: geom_core::Real + Sized {
    /// The tightest enclosure of the offset span `[lo, hi]`, or `None`
    /// when this scalar cannot represent it.
    fn axis(lo: f64, hi: f64) -> Option<Self>;
}

/// A point scalar carries only a degenerate axis, and compares by BITS:
/// `-0.0` and `0.0` are the same offset in every arithmetic sense, and
/// this door is asked once per parameter per evaluation, so the stricter
/// comparison costs nothing and cannot admit a span by rounding.
impl AxisScalar for f64 {
    fn axis(lo: f64, hi: f64) -> Option<Self> {
        (lo.to_bits() == hi.to_bits()).then_some(lo)
    }
}

/// The recording scalar is `f64` with a sink attached (`Decide`
/// delegates and records), so it carries exactly what `f64` carries.
#[cfg(feature = "probe")]
impl AxisScalar for geom_core::Probe {
    fn axis(lo: f64, hi: f64) -> Option<Self> {
        f64::axis(lo, hi).map(geom_core::Probe)
    }
}

/// The interval scalar carries every axis: this is the door
/// [`geom_core::Interval::from_bounds`] exists for.
#[cfg(feature = "interval")]
impl AxisScalar for geom_core::Interval {
    fn axis(lo: f64, hi: f64) -> Option<Self> {
        Some(geom_core::Interval::from_bounds(lo, hi))
    }
}

/// A dual carries whatever its value channel carries, with a ZERO
/// tangent: a box axis is an enclosure, not a seed. Seeding a parameter
/// is a separate act on the same environment and stays that way — a box
/// that silently seeded would make "the derivative with respect to what"
/// a property of the analysis box.
impl<T: AxisScalar> AxisScalar for geom_core::Dual<T>
where
    geom_core::Dual<T>: geom_core::Real,
{
    fn axis(lo: f64, hi: f64) -> Option<Self> {
        T::axis(lo, hi).map(geom_core::Dual::constant)
    }
}

/// One axis of a [`ParamBox`]: the offsets the box spans on that
/// parameter, around the document's nominal.
///
/// **`Fixed` is spelled, never omitted.** A parameter with no
/// distribution contributes mass 1 to every leaf, and that is a
/// modelling statement (E2's opt-in rule) rather than a convention each
/// consumer re-derives from a missing entry — so the box type carries
/// the distinction and [`ParamBox::mass`] reads it off the type.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BoxAxis {
    /// The parameter does not vary: the axis is the nominal point, and
    /// its mass is 1 under every measure.
    Fixed,
    /// The parameter varies over these offsets. Unlike
    /// [`OffsetInterval`] this need NOT contain zero: a leaf of the
    /// subdivision generally sits off the nominal.
    Varying {
        /// Lower offset.
        lo: f64,
        /// Upper offset.
        hi: f64,
    },
}

impl BoxAxis {
    /// The offsets this axis spans (`(0, 0)` when fixed).
    pub fn span(&self) -> (f64, f64) {
        match *self {
            Self::Fixed => (0.0, 0.0),
            Self::Varying { lo, hi } => (lo, hi),
        }
    }

    /// `hi - lo` (`0.0` when fixed).
    pub fn width(&self) -> f64 {
        let (lo, hi) = self.span();
        hi - lo
    }
}

/// A sub-box of the [`AnalyzedBox`]: one [`BoxAxis`] per continuous
/// document parameter, in name order. Derived, never stored.
///
/// The root box is [`ParamBox::of`] an analyzed box; every other box is
/// a [`ParamBox::split`] descendant of one. `Count` parameters are not
/// axes (E0's term hygiene) and never appear here.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ParamBox {
    axes: BTreeMap<ParamName, BoxAxis>,
}

/// A box that cannot be turned into an environment.
#[derive(Debug, Clone, PartialEq)]
pub enum ParamBoxError {
    /// The box names a parameter the document does not carry as a
    /// continuous parameter.
    UnknownParam {
        /// The unmatched name.
        param: ParamName,
    },
    /// The evaluation scalar cannot represent this axis — an `f64` (or
    /// `Probe`) build asked to run over a box with width in it. Refused
    /// rather than narrowed: evaluating at the nominals would answer a
    /// different question in the same shape.
    AxisUnrepresentable {
        /// The axis that does not fit the scalar.
        param: ParamName,
        /// The lower offset asked for.
        lo: f64,
        /// The upper offset asked for.
        hi: f64,
    },
}

impl core::fmt::Display for ParamBoxError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::UnknownParam { param } => write!(
                f,
                "the parameter box names {:?}, which is not a continuous parameter of this document",
                param.0
            ),
            Self::AxisUnrepresentable { param, lo, hi } => write!(
                f,
                "parameter {:?} spans offsets [{lo}, {hi}] and this evaluation scalar carries no \
                 such value — a widened box needs an enclosing scalar",
                param.0
            ),
        }
    }
}

impl ParamBox {
    /// The root box of an analyzed box: every axis at its full analyzed
    /// offsets, and every unannotated parameter [`BoxAxis::Fixed`].
    pub fn of(analyzed: &AnalyzedBox) -> Self {
        let axes = analyzed
            .params()
            .iter()
            .map(|(name, p)| {
                let axis = if p.offsets.is_fixed() {
                    BoxAxis::Fixed
                } else {
                    BoxAxis::Varying {
                        lo: p.offsets.lo,
                        hi: p.offsets.hi,
                    }
                };
                (name.clone(), axis)
            })
            .collect();
        Self { axes }
    }

    /// The axes, by parameter name.
    pub fn axes(&self) -> &BTreeMap<ParamName, BoxAxis> {
        &self.axes
    }

    /// One axis, by name.
    pub fn get(&self, name: &ParamName) -> Option<&BoxAxis> {
        self.axes.get(name)
    }

    /// The axes that vary — the box's non-degenerate dimensions.
    pub fn varying(&self) -> impl Iterator<Item = (&ParamName, f64, f64)> {
        self.axes.iter().filter_map(|(n, a)| match *a {
            BoxAxis::Fixed => None,
            BoxAxis::Varying { lo, hi } => Some((n, lo, hi)),
        })
    }

    /// The DETERMINISTIC split axis (D9): the varying axis of greatest
    /// width RELATIVE to `root`'s width on that axis, ties broken to the
    /// lowest axis index — which is name order, the order every box
    /// iterates in. `None` when nothing varies.
    ///
    /// Relative rather than absolute because axes carry different
    /// dimensions and different spreads: a 10 mm band and a 0.01°
    /// band are not comparable as numbers, and bisecting the numerically
    /// widest would subdivide one axis forever. Relative width is
    /// dimensionless and is 1 on every axis of the root box, so the
    /// recursion refines the box uniformly.
    pub fn split_axis(&self, root: &Self) -> Option<ParamName> {
        let mut best: Option<(ParamName, f64)> = None;
        for (name, lo, hi) in self.varying() {
            let full = root.get(name).map_or(0.0, BoxAxis::width);
            // A varying axis of a box whose root axis is degenerate has
            // no relative width to speak of; rank it by its own width so
            // it is still splittable rather than silently unsplittable.
            let rel = if full > 0.0 { (hi - lo) / full } else { hi - lo };
            // STRICTLY greater keeps the tie on the earlier name, which
            // is the tie-break the rule names.
            if best.as_ref().is_none_or(|(_, b)| rel > *b) {
                best = Some((name.clone(), rel));
            }
        }
        best.map(|(n, _)| n)
    }

    /// The two halves of this box across `name`, bisected at the axis
    /// midpoint. `None` when the axis is absent, fixed, or too narrow to
    /// split (a midpoint that lands on an endpoint — the `f64` grid's
    /// own floor).
    ///
    /// The midpoint is `0.5 * (lo + hi)`, a pure function of the two
    /// endpoints, so the same box splits into the same two halves on
    /// every machine and in every schedule (D9).
    pub fn split(&self, name: &ParamName) -> Option<(Self, Self)> {
        let BoxAxis::Varying { lo, hi } = *self.axes.get(name)? else {
            return None;
        };
        let mid = 0.5 * (lo + hi);
        if !(lo < mid && mid < hi) {
            return None;
        }
        let mut a = self.clone();
        let mut b = self.clone();
        a.axes.insert(name.clone(), BoxAxis::Varying { lo, hi: mid });
        b.axes.insert(name.clone(), BoxAxis::Varying { lo: mid, hi });
        Some((a, b))
    }

    /// This box's mass under the product measure (E2's independence):
    /// the product over axes of each axis's own mass.
    ///
    /// A [`BoxAxis::Fixed`] axis contributes exactly 1 — the typed
    /// spelling of E2's opt-in rule, read off the axis rather than
    /// inferred. A [`Band`](Distribution::Band) axis refuses, naming the
    /// parameter, unless this box covers its whole support or misses it
    /// entirely: pricing is where a band stops, never certification.
    pub fn mass(&self, analyzed: &AnalyzedBox) -> Result<f64, MeasureUnavailable> {
        let mut m = 1.0;
        for (name, axis) in &self.axes {
            m *= match *axis {
                BoxAxis::Fixed => 1.0,
                BoxAxis::Varying { lo, hi } => match analyzed.axis_box_mass(name, (lo, hi)) {
                    Some(r) => r?,
                    // An axis the analyzed box does not carry prices
                    // nothing: it is not this door's job to invent one.
                    None => 0.0,
                },
            };
        }
        Ok(m)
    }

    /// Whether this box touches the boundary of `root` — some axis of it
    /// sits at one of the root axis's own endpoints.
    ///
    /// The free predicate E2's chamber-containment amendment asks for:
    /// containment holds when every boundary-touching leaf is
    /// `FlipCrossing`-refused.
    pub fn touches_boundary_of(&self, root: &Self) -> bool {
        self.varying().any(|(name, lo, hi)| {
            let (rlo, rhi) = root.get(name).map_or((0.0, 0.0), |a| a.span());
            lo <= rlo || hi >= rhi
        })
    }
}

/// The parameter environment of `doc`'s nominals WIDENED by `box_` —
/// the interval parameter door (E6's leaf replay reads through this).
///
/// Each axis binds `nominal + [lo, hi]`, formed in the scalar's own
/// arithmetic so the enclosure rounds outward; `Count` parameters bind
/// exactly as [`Doc::param_env`] binds them (they are structural, never
/// axes). A parameter the box does not name binds its nominal.
///
/// # Errors
///
/// [`ParamBoxError::UnknownParam`] when the box names a parameter the
/// document does not have; [`ParamBoxError::AxisUnrepresentable`] when
/// `T` cannot carry a widened axis.
pub fn param_env_over<T: AxisScalar, P>(
    doc: &Doc<P>,
    box_: &ParamBox,
) -> Result<crate::expr::ParamEnv<T>, ParamBoxError> {
    for name in box_.axes.keys() {
        if !matches!(doc.params().get(name), Some(DocParam::Continuous { .. })) {
            return Err(ParamBoxError::UnknownParam {
                param: name.clone(),
            });
        }
    }
    let mut bindings = BTreeMap::new();
    for (name, p) in doc.params() {
        let v = match *p {
            DocParam::Continuous { dim, value, .. } => {
                let (lo, hi) = box_.get(name).map_or((0.0, 0.0), BoxAxis::span);
                let offset = T::axis(lo, hi).ok_or_else(|| ParamBoxError::AxisUnrepresentable {
                    param: name.clone(),
                    lo,
                    hi,
                })?;
                crate::expr::ParamValue::Continuous {
                    dim,
                    value: T::from_f64(value) + offset,
                }
            }
            DocParam::Count { value } => crate::expr::ParamValue::Count(value),
        };
        bindings.insert(name.clone(), v);
    }
    Ok(crate::expr::ParamEnv { bindings })
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
