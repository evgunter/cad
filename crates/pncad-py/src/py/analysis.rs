//! **Parameter uncertainty and the analysis lane** (ERROR-DESIGN
//! E1/E2, E11.1): the annotation a continuous parameter carries, the
//! three derived answers a caller reads back off a document, and the
//! advisory estimator that replays the document over draws from it.
//!
//! A [`Distribution`] is inert document metadata. It feeds no
//! evaluation, no content key and no predicate — the kernel and the
//! geometry lanes never see a probability — and `pncad::analysis` is
//! its ONE interpreter. So this module holds both halves: the value a
//! `DocParam` carries, and the doors that read it.
//!
//! # The offsets are typed, and the parameter owns the dimension
//!
//! Every field of a distribution is an OFFSET relative to the
//! parameter's nominal, in the parameter's own dimension (E2: there
//! is no separate dimension field to disagree with the nominal). The
//! kernel stores those offsets as canonical `f64`, exactly as it
//! stores the nominal. Python does not: §L4's typed quantities are
//! what stop `1e-6` meaning microns to a reader and metres to the
//! kernel, and a sigma is a length in every way a nominal is. So each
//! constructor takes `Length`, `Angle` or a bare `float`, the wrapper
//! REMEMBERS which, and every offset in one distribution has to agree
//! — a `Length` low bound with an `Angle` high bound is a
//! `DimensionError` at the constructor, not a plausible number later.
//!
//! A distribution read back OFF a document takes its dimension from
//! the parameter that carried it, which is the same statement from
//! the other side: the parameter is where the dimension lives, and
//! the annotation borrows it.
//!
//! # Construction refuses EARLY, through the kernel's own check
//!
//! `Distribution::check` is the one statement of E2's invariants —
//! the edit door and the persistence validator both run it, so a file
//! that would refuse to load cannot be authored. The constructors
//! here call THAT function rather than restating it, so the binding
//! cannot drift from what the kernel refuses; what they add is
//! timing. A sigma of zero refuses where the zero is written instead
//! of at the `Doc.apply` three lines later, and the refusal is
//! `DistributionFault`, carrying the same fault value the edit door's
//! `invalid_distribution` carries.
//!
//! # The read doors, and the one that is not here
//!
//! `analyzed_box` projects a document to the box the analysis varies
//! its parameters over; [`AnalyzedBox::tail_mass`] reports the mass
//! that box leaves out; [`AnalyzedBox::box_mass`] prices a
//! sub-interval of it. The last two refuse typed on a band — limits
//! with no shape price nothing, and no report may quietly promote
//! them to uniform.
//!
//! Those two arrive here as METHODS ON THE BOX, where the kernel also
//! offers them as free functions taking a name, a distribution and an
//! interval as three loose arguments. That is deliberate and it is
//! the kernel's own reasoning, applied one language out:
//! `AnalyzedBox::axis_tail_mass`'s doc comment says the free doors let
//! a caller pair one parameter's distribution with another's box and
//! get a plausible number rather than a refusal, and Python has no
//! compile step that would catch it. The kernel keeps the free doors
//! for the driver, which prices intervals that are deliberately NOT
//! the analyzed ones; the driver is behind `interval` and has no
//! Python surface, so nothing on this side wants them.
//!
//! # The advisory lane, which is on this side of the gate
//!
//! `monte_carlo` replays the document at `f64` over draws from its own
//! distributions and answers an [`McReport`] — every number an
//! ESTIMATE, with the sample count and the seed that produced it on
//! the report and on every rendered line. It is bound here because it
//! is UNGATED in the kernel, and it is ungated for exactly this
//! caller: a consumer with no certified scalar still gets the labeled
//! estimate, which is the whole point of an advisory lane.
//!
//! [`sample_offset`] is the same lane one rung down — the single draw
//! the replay is built from, crossing as a free function beside it
//! because that is what it is: one law, one quantile, one offset.
//! Both refuse on a BAND, which is the module's standing refusal
//! arriving in a third place; the run refuses as a whole rather than
//! sampling the parameters it can, because a mean over a subset of
//! them is an estimate of a different document.
//!
//! # What is NOT bound, and why it is a measurement rather than a gap
//!
//! `pncad::analysis` carries a second half — the E6 subdivision
//! driver and its `ParamBox`, the E4/E5 sensitivity and stackup, the
//! E10 reporting layer, `assertion_at` — behind
//! `#[cfg(feature = "interval")]`. The wheel is built from the default
//! feature set, so those names do not exist in the crate this module
//! compiles into and binding them would mean shipping a door that is
//! absent from the artifact a user installs. Everything this module
//! DOES bind is on the ungated list, so the whole of it compiles on
//! the default build; the gate is a boundary this module stops at, not
//! one it works around. `crates/pncad/src/analysis.rs` is read by the
//! binding census, so a name crossing that boundary in either
//! direction owes a row there rather than passing unremarked.

use pyo3::prelude::*;
use pyo3::types::PyString;

use crate::errors::{ErrorClass, QuantityOpMismatch, dimension_tag};
use crate::py::typed_err;
use crate::tags::{
    analysis_policy_error_tag, distribution_fault_tag, distribution_field_tag,
    distribution_kind_tag, mc_refusal_tag, measure_unavailable_tag,
};
use pncad::analysis as a;
use pncad::document as d;
use pncad::tolerance::Tol;

/// One offset argument, as it crossed: the canonical `f64` and the
/// dimension the caller wrote it in.
///
/// `Dimension::Count` is unreachable here on purpose — a `Count`
/// parameter cannot be annotated at all (E2: a structural count is
/// fixed under any error analysis), so a `Count` quantity has no
/// offset to be.
#[derive(Clone, Copy)]
struct Offset {
    canonical: f64,
    dim: d::Dimension,
}

/// Read one offset argument: a `Length`, an `Angle` or a bare
/// `float`.
///
/// Anything else is a `TypeError` naming the three, which is the
/// boundary refusal for an argument that is not a quantity at all
/// rather than one whose dimension is wrong.
fn offset(obj: &Bound<'_, PyAny>) -> PyResult<Offset> {
    if let Ok(length) = obj.extract::<PyRef<'_, super::quantity::Length>>() {
        return Ok(Offset {
            canonical: length.0.meters(),
            dim: d::Dimension::Length,
        });
    }
    if let Ok(angle) = obj.extract::<PyRef<'_, super::quantity::Angle>>() {
        return Ok(Offset {
            canonical: angle.0.radians(),
            dim: d::Dimension::Angle,
        });
    }
    if let Ok(scalar) = obj.extract::<f64>() {
        return Ok(Offset {
            canonical: scalar,
            dim: d::Dimension::Scalar,
        });
    }
    Err(pyo3::exceptions::PyTypeError::new_err(
        "a distribution offset is a Length, an Angle or a float — the \
         parameter's own dimension, as an offset from its nominal",
    ))
}

/// The dimension a group of offsets agree on, or the mismatch.
///
/// `door` names the constructor in the raise, because a
/// `DimensionError` from here is about a pair of ARGUMENTS rather
/// than about an operator, and the attribute that says which pair is
/// the only place a caller can read it.
fn agreed(py: Python<'_>, door: &'static str, parts: &[Offset]) -> PyResult<d::Dimension> {
    let Some(first) = parts.first() else {
        // No door calls this with an empty slice; every distribution
        // form carries at least one offset.
        return Ok(d::Dimension::Scalar);
    };
    for part in &parts[1..] {
        if part.dim != first.dim {
            return Err(dimension_mismatch(py, door, first.dim, part.dim));
        }
    }
    Ok(first.dim)
}

/// The quantity boundary's `DimensionError`, raised for a DOOR whose
/// arguments must share a dimension rather than for an operator.
///
/// Shared with the `DocParam` constructors, which ask the same
/// question one rung up: a parameter's declaration and the annotation
/// hung on it must agree about what dimension the offsets are in.
pub(crate) fn dimension_mismatch(
    py: Python<'_>,
    door: &'static str,
    left: d::Dimension,
    right: d::Dimension,
) -> PyErr {
    let text = |s: &str| PyString::new(py, s).unbind().into_any();
    typed_err(
        py,
        ErrorClass::Dimension,
        QuantityOpMismatch::new(door, left, right).to_string(),
        &[
            ("op", text(door)),
            ("left", text(dimension_tag(left))),
            ("right", text(dimension_tag(right))),
        ],
    )
}

/// Raise `DistributionFault` with the whole fault projected.
///
/// The projected shape `py/readback.rs` states: `variant` plus every
/// arm's payload, present on every arm and `None` where that arm does
/// not carry one, so `getattr` never raises and a caller never has to
/// branch on `variant` before reading a field.
fn fault_err(py: Python<'_>, fault: &d::DistributionFault) -> PyErr {
    let field = match fault {
        d::DistributionFault::NonFinite { field } => Some(distribution_field_tag(field)),
        d::DistributionFault::SigmaNotPositive { .. }
        | d::DistributionFault::NominalOutsideSupport { .. } => None,
    };
    let sigma = match *fault {
        d::DistributionFault::SigmaNotPositive { sigma } => Some(sigma),
        d::DistributionFault::NonFinite { .. }
        | d::DistributionFault::NominalOutsideSupport { .. } => None,
    };
    let (lo, hi) = match *fault {
        d::DistributionFault::NominalOutsideSupport { lo, hi } => (Some(lo), Some(hi)),
        d::DistributionFault::NonFinite { .. } | d::DistributionFault::SigmaNotPositive { .. } => {
            (None, None)
        }
    };
    let object = |value: Option<f64>| -> PyResult<Py<PyAny>> {
        Ok(value.into_pyobject(py)?.into_any().unbind())
    };
    let attrs = match (object(sigma), object(lo), object(hi)) {
        (Ok(sigma), Ok(lo), Ok(hi)) => [
            (
                "variant",
                PyString::new(py, distribution_fault_tag(fault))
                    .unbind()
                    .into_any(),
            ),
            (
                "field",
                match field {
                    Some(name) => PyString::new(py, name).unbind().into_any(),
                    None => py.None(),
                },
            ),
            ("sigma", sigma),
            ("lo", lo),
            ("hi", hi),
        ],
        (Err(failed), _, _) | (_, Err(failed), _) | (_, _, Err(failed)) => return failed,
    };
    typed_err(py, ErrorClass::Distribution, fault.to_string(), &attrs)
}

/// Raise `MeasureUnavailable` — the band's typed refusal, with the
/// parameter it names.
fn measure_err(py: Python<'_>, err: &a::MeasureUnavailable) -> PyErr {
    let a::MeasureUnavailable::BandHasNoMeasure { param } = err;
    typed_err(
        py,
        ErrorClass::Measure,
        err.to_string(),
        &[
            (
                "variant",
                PyString::new(py, measure_unavailable_tag(err))
                    .unbind()
                    .into_any(),
            ),
            ("param", PyString::new(py, &param.0).unbind().into_any()),
        ],
    )
}

/// Hand a canonical offset back in the dimension it belongs to.
fn quantity(py: Python<'_>, canonical: f64, dim: d::Dimension) -> PyResult<Py<PyAny>> {
    Ok(match dim {
        d::Dimension::Length => {
            super::quantity::Length(pncad::quantity::Length::from_meters(canonical))
                .into_pyobject(py)?
                .into_any()
                .unbind()
        }
        d::Dimension::Angle => {
            super::quantity::Angle(pncad::quantity::Angle::from_radians(canonical))
                .into_pyobject(py)?
                .into_any()
                .unbind()
        }
        // A `Count` parameter is not a box axis and cannot be
        // annotated, so neither reader reaches this arm with one; a
        // dimensionless offset is the number itself.
        d::Dimension::Scalar | d::Dimension::Count => {
            canonical.into_pyobject(py)?.into_any().unbind()
        }
    })
}

/// **A parameter's uncertainty** (ERROR-DESIGN E1/E2): offsets from
/// its nominal, in its own dimension, in one of four forms.
///
/// The differences between the forms are CLAIMS, not conveniences.
/// `band` states limits and no shape; `uniform` states the same
/// limits and says every value between them is equally likely;
/// `normal` states a spread with unbounded support; `truncated_normal`
/// restricts a normal to a window and renormalizes it. A parameter
/// with NO distribution is FIXED — annotation is opt-in and means
/// something, and the analysis never guesses a spread nobody stated.
///
/// The shape is `PatternKind`'s and `PartSelect`'s: a frozen value
/// class of static constructors, one per kernel arm, spelled in snake
/// case. What it adds over those two is that the arguments are typed
/// quantities and the constructor CHECKS — see the module header for
/// both halves.
#[pyclass(frozen, module = "pncad", from_py_object)]
#[derive(Clone)]
pub(crate) struct Distribution {
    pub(crate) inner: d::Distribution,
    /// The dimension the offsets were written in — the parameter's,
    /// borrowed. Never `Count`.
    pub(crate) dim: d::Dimension,
}

impl Distribution {
    /// The kernel value with the dimension a `DocParam` declares.
    ///
    /// The read direction of the borrow the module header describes:
    /// an annotation off a document has no dimension of its own, and
    /// this is where it takes the parameter's.
    pub(crate) fn borrowed(inner: d::Distribution, dim: d::Dimension) -> Self {
        Self { inner, dim }
    }

    /// Check a freshly built distribution through the kernel's own
    /// invariant statement, and hand back the wrapper.
    fn checked(py: Python<'_>, inner: d::Distribution, dim: d::Dimension) -> PyResult<Self> {
        match inner.check() {
            Ok(()) => Ok(Self { inner, dim }),
            Err(fault) => Err(fault_err(py, &fault)),
        }
    }
}

#[pymethods]
impl Distribution {
    /// Worst-case limits with NO shape claim: `[lo, hi]` bounds the
    /// parameter and prices nothing.
    ///
    /// "I know the extremes, not the distribution" is real
    /// information, and the mass doors refuse rather than promote it
    /// to uniform — except for the two answers every measure on the
    /// band agrees about (an interval covering the whole support
    /// holds mass 1, a disjoint one holds 0), which are set facts
    /// rather than shape claims.
    #[staticmethod]
    fn band(py: Python<'_>, lo: &Bound<'_, PyAny>, hi: &Bound<'_, PyAny>) -> PyResult<Self> {
        let (lo, hi) = (offset(lo)?, offset(hi)?);
        let dim = agreed(py, "Distribution.band", &[lo, hi])?;
        Self::checked(
            py,
            d::Distribution::Band {
                lo: lo.canonical,
                hi: hi.canonical,
            },
            dim,
        )
    }

    /// Uniform over `[lo, hi]` — the same limits a band states, plus
    /// the shape claim a band withholds. It answers exactly where the
    /// band refuses.
    #[staticmethod]
    fn uniform(py: Python<'_>, lo: &Bound<'_, PyAny>, hi: &Bound<'_, PyAny>) -> PyResult<Self> {
        let (lo, hi) = (offset(lo)?, offset(hi)?);
        let dim = agreed(py, "Distribution.uniform", &[lo, hi])?;
        Self::checked(
            py,
            d::Distribution::Uniform {
                lo: lo.canonical,
                hi: hi.canonical,
            },
            dim,
        )
    }

    /// A zero-mean normal with standard deviation `sigma > 0`.
    ///
    /// Unbounded support: the analyzed box is the ANALYSIS's knob,
    /// never a cutoff baked into the model, so what a box leaves out
    /// is reported as tail mass rather than dropped.
    #[staticmethod]
    fn normal(py: Python<'_>, sigma: &Bound<'_, PyAny>) -> PyResult<Self> {
        let sigma = offset(sigma)?;
        Self::checked(
            py,
            d::Distribution::Normal {
                sigma: sigma.canonical,
            },
            sigma.dim,
        )
    }

    /// A normal restricted to `[lo, hi]` and RENORMALIZED — not
    /// clipped. Its own support holds all of its mass, so its tail is
    /// identically zero.
    #[staticmethod]
    fn truncated_normal(
        py: Python<'_>,
        sigma: &Bound<'_, PyAny>,
        lo: &Bound<'_, PyAny>,
        hi: &Bound<'_, PyAny>,
    ) -> PyResult<Self> {
        let (sigma, lo, hi) = (offset(sigma)?, offset(lo)?, offset(hi)?);
        let dim = agreed(py, "Distribution.truncated_normal", &[sigma, lo, hi])?;
        Self::checked(
            py,
            d::Distribution::TruncatedNormal {
                sigma: sigma.canonical,
                lo: lo.canonical,
                hi: hi.canonical,
            },
            dim,
        )
    }

    /// Which of the four forms this is.
    #[getter]
    fn kind(&self) -> &'static str {
        distribution_kind_tag(&self.inner)
    }

    /// The dimension its offsets are in — the parameter's own.
    #[getter]
    fn dimension(&self) -> &'static str {
        dimension_tag(self.dim)
    }

    /// The lower offset, or `None` for the one unbounded form.
    #[getter]
    fn lo(&self, py: Python<'_>) -> PyResult<Py<PyAny>> {
        match self.inner.support() {
            Some((lo, _)) => quantity(py, lo, self.dim),
            None => Ok(py.None()),
        }
    }

    /// The upper offset, or `None` for the one unbounded form.
    #[getter]
    fn hi(&self, py: Python<'_>) -> PyResult<Py<PyAny>> {
        match self.inner.support() {
            Some((_, hi)) => quantity(py, hi, self.dim),
            None => Ok(py.None()),
        }
    }

    /// The standard deviation of the underlying normal, or `None` for
    /// the two forms that state no shape parameter.
    ///
    /// For a `truncated_normal` this is the UNDERLYING normal's sigma,
    /// which is what the form was written with; the truncated law's own
    /// spread is a derived number and a different question.
    #[getter]
    fn sigma(&self, py: Python<'_>) -> PyResult<Py<PyAny>> {
        match self.inner {
            d::Distribution::Normal { sigma } | d::Distribution::TruncatedNormal { sigma, .. } => {
                quantity(py, sigma, self.dim)
            }
            d::Distribution::Band { .. } | d::Distribution::Uniform { .. } => Ok(py.None()),
        }
    }

    /// Rust's `PartialEq`, mirrored — IEEE on the offsets, so the two
    /// spellings of zero are the same offset. The dimension is part of
    /// the value: a Length band and a Scalar band of the same numbers
    /// are different annotations, exactly as a Length 1 and a Scalar 1
    /// are different `DocParam`s.
    fn __eq__(&self, other: &Self) -> bool {
        self.dim == other.dim && self.inner == other.inner
    }

    /// Consistent with [`Self::__eq__`], through the kernel's own
    /// fold: `Distribution::fold_signed_zeros` is the ONE statement of
    /// the `-0.0` normalization a hash must apply wherever the
    /// equality it mirrors is IEEE, and `DocParam.__hash__` folds
    /// through the same door.
    fn __hash__(&self) -> u64 {
        use std::hash::{Hash, Hasher};
        let mut h = std::hash::DefaultHasher::new();
        format!("{:?}", self.dim).hash(&mut h);
        format!("{:?}", self.inner.fold_signed_zeros()).hash(&mut h);
        h.finish()
    }

    fn __repr__(&self) -> String {
        format!(
            "Distribution({} {:?} {})",
            self.kind(),
            self.inner,
            self.dimension()
        )
    }
}

/// **How a run chooses its analyzed box**: request configuration,
/// never a global (E2).
///
/// `quantile_mass` is the share of each unbounded parameter's mass the
/// box is asked to cover, and it defaults to
/// `DEFAULT_QUANTILE_MASS` — the ±3σ convention. Moving it moves mass
/// between the analyzed and the tail columns; it never moves truth,
/// because the tail is reported rather than dropped.
///
/// It is checked into `(0, 1)`: mass 1 asks for an infinite box, mass
/// 0 for an empty one, and neither is a box.
#[pyclass(frozen, module = "pncad", from_py_object)]
#[derive(Clone, Copy)]
pub(crate) struct AnalysisPolicy(a::AnalysisPolicy);

#[pymethods]
impl AnalysisPolicy {
    #[new]
    #[pyo3(signature = (quantile_mass = None))]
    fn new(py: Python<'_>, quantile_mass: Option<f64>) -> PyResult<Self> {
        let Some(mass) = quantile_mass else {
            return Ok(Self(a::AnalysisPolicy::default()));
        };
        match a::AnalysisPolicy::new(mass) {
            Ok(policy) => Ok(Self(policy)),
            Err(err) => {
                let a::AnalysisPolicyError::QuantileMassOutOfRange { mass } = err;
                let refused = mass.into_pyobject(py)?.into_any().unbind();
                Err(typed_err(
                    py,
                    ErrorClass::AnalysisPolicy,
                    err.to_string(),
                    &[
                        (
                            "variant",
                            PyString::new(py, analysis_policy_error_tag(&err))
                                .unbind()
                                .into_any(),
                        ),
                        ("mass", refused),
                    ],
                ))
            }
        }
    }

    /// The requested per-parameter mass.
    #[getter]
    fn quantile_mass(&self) -> f64 {
        self.0.quantile_mass()
    }

    fn __eq__(&self, other: &Self) -> bool {
        self.0 == other.0
    }

    fn __repr__(&self) -> String {
        format!("AnalysisPolicy({})", self.0.quantile_mass())
    }
}

/// **One axis of the analyzed box**: the parameter's nominal, the
/// offset interval the analysis varies it over, and the distribution
/// that interval came from.
///
/// An unannotated continuous parameter is still an axis — a
/// width-zero one at its nominal, with `distribution` `None`. That is
/// the typed spelling of FIXED, and it is why a document says what it
/// varies rather than having it inferred.
#[pyclass(frozen, module = "pncad", skip_from_py_object)]
#[derive(Clone)]
pub(crate) struct AnalyzedParam {
    inner: a::AnalyzedParam,
}

#[pymethods]
impl AnalyzedParam {
    /// The parameter's declared dimension.
    #[getter]
    fn dimension(&self) -> &'static str {
        dimension_tag(self.inner.dim)
    }

    /// The document's nominal value — the single source of truth.
    #[getter]
    fn nominal(&self, py: Python<'_>) -> PyResult<Py<PyAny>> {
        quantity(py, self.inner.nominal, self.inner.dim)
    }

    /// The analyzed offsets around the nominal, as `(lo, hi)`.
    #[getter]
    fn offsets(&self, py: Python<'_>) -> PyResult<(Py<PyAny>, Py<PyAny>)> {
        Ok((
            quantity(py, self.inner.offsets.lo, self.inner.dim)?,
            quantity(py, self.inner.offsets.hi, self.inner.dim)?,
        ))
    }

    /// The analyzed interval in ABSOLUTE parameter values.
    fn absolute(&self, py: Python<'_>) -> PyResult<(Py<PyAny>, Py<PyAny>)> {
        let (lo, hi) = self.inner.absolute();
        Ok((
            quantity(py, lo, self.inner.dim)?,
            quantity(py, hi, self.inner.dim)?,
        ))
    }

    /// `hi - lo`.
    #[getter]
    fn width(&self, py: Python<'_>) -> PyResult<Py<PyAny>> {
        quantity(py, self.inner.offsets.width(), self.inner.dim)
    }

    /// Whether this axis is FIXED: width zero at the nominal.
    #[getter]
    fn is_fixed(&self) -> bool {
        self.inner.offsets.is_fixed()
    }

    /// The distribution this axis came from, or `None` for a fixed
    /// parameter — carrying the parameter's dimension, which is where
    /// an annotation's dimension lives.
    #[getter]
    fn distribution(&self) -> Option<Distribution> {
        self.inner
            .distribution
            .map(|dist| Distribution::borrowed(dist, self.inner.dim))
    }

    fn __repr__(&self) -> String {
        format!(
            "AnalyzedParam({:?} {} {:?})",
            self.inner.dim, self.inner.nominal, self.inner.offsets
        )
    }
}

/// **The analyzed box**: one axis per CONTINUOUS document parameter,
/// in name order. Derived on request from a document and a policy,
/// never stored, and never seen by evaluation.
///
/// `Count` parameters are not axes: a structural count is fixed under
/// any error analysis.
#[pyclass(frozen, module = "pncad")]
pub(crate) struct AnalyzedBox(a::AnalyzedBox);

#[pymethods]
impl AnalyzedBox {
    /// Every axis name, in the box's own order.
    #[getter]
    fn names(&self) -> Vec<super::doc::ParamName> {
        self.0
            .params()
            .keys()
            .map(|name| super::doc::ParamName(name.clone()))
            .collect()
    }

    /// The names of the axes that actually VARY — the box's
    /// non-degenerate dimensions.
    #[getter]
    fn varying(&self) -> Vec<super::doc::ParamName> {
        self.0
            .varying()
            .map(|(name, _)| super::doc::ParamName(name.clone()))
            .collect()
    }

    /// One axis by name, or `None` when the document declares no such
    /// continuous parameter.
    fn get(&self, name: &super::doc::ParamName) -> Option<AnalyzedParam> {
        self.0
            .get(&name.0)
            .map(|inner| AnalyzedParam { inner: *inner })
    }

    /// **The tail mass of one axis**: what this box's own interval for
    /// `name` leaves outside.
    ///
    /// `None` when the document declares no such continuous parameter.
    /// An unannotated axis is FIXED and its tail is `0.0` — nothing was
    /// declared to vary, so the analysis is leaving nothing out.
    ///
    /// Raises `MeasureUnavailable` when the axis carries a band whose
    /// support escapes the interval: how much of it escapes is
    /// precisely what a band does not say.
    fn tail_mass(&self, py: Python<'_>, name: &super::doc::ParamName) -> PyResult<Option<f64>> {
        match self.0.axis_tail_mass(&name.0) {
            None => Ok(None),
            Some(Ok(mass)) => Ok(Some(mass)),
            Some(Err(err)) => Err(measure_err(py, &err)),
        }
    }

    /// **The leaf mass of one axis**: what its distribution puts
    /// INSIDE the offset interval `(lo, hi)`.
    ///
    /// The offsets are quantities in the axis's own dimension, and one
    /// in another dimension is a `DimensionError` — the pairing this
    /// door exists to make impossible, one rung out from the kernel's.
    ///
    /// `None` when the document declares no such continuous parameter.
    /// An unannotated axis is a point mass at its nominal, so it
    /// answers `1.0` for any interval containing offset zero and `0.0`
    /// otherwise. A band raises `MeasureUnavailable` unless the
    /// interval covers its whole support or misses it entirely.
    fn box_mass(
        &self,
        py: Python<'_>,
        name: &super::doc::ParamName,
        lo: &Bound<'_, PyAny>,
        hi: &Bound<'_, PyAny>,
    ) -> PyResult<Option<f64>> {
        let (lo, hi) = (offset(lo)?, offset(hi)?);
        let dim = agreed(py, "AnalyzedBox.box_mass", &[lo, hi])?;
        if let Some(axis) = self.0.get(&name.0)
            && axis.dim != dim
        {
            return Err(dimension_mismatch(
                py,
                "AnalyzedBox.box_mass",
                axis.dim,
                dim,
            ));
        }
        match self.0.axis_box_mass(&name.0, (lo.canonical, hi.canonical)) {
            None => Ok(None),
            Some(Ok(mass)) => Ok(Some(mass)),
            Some(Err(err)) => Err(measure_err(py, &err)),
        }
    }

    fn __len__(&self) -> usize {
        self.0.params().len()
    }

    fn __repr__(&self) -> String {
        format!("AnalyzedBox({} axes)", self.0.params().len())
    }
}

/// **The analyzed box of a document under a policy** (E1's first
/// consumable).
///
/// Per continuous parameter: the bounded support for `band`,
/// `uniform` and `truncated_normal`; the symmetric quantile interval
/// `±z·sigma` for `normal`; and a width-zero interval at the nominal
/// for a parameter with no distribution.
#[pyfunction]
#[pyo3(signature = (doc, policy = None))]
fn analyzed_box(doc: &super::doc::Doc, policy: Option<&AnalysisPolicy>) -> AnalyzedBox {
    let policy = policy.map_or_else(a::AnalysisPolicy::default, |p| p.0);
    AnalyzedBox(a::analyzed_box(&doc.inner, &policy))
}

/// Raise `McRefusal` — the advisory lane's own refusal, with the
/// whole payload projected.
///
/// The projected shape `py/readback.rs` states: `variant` plus every
/// arm's payload, present on every arm and `None` where that arm does
/// not carry one, so `getattr` never raises and a caller never has to
/// branch on `variant` before reading a field.
///
/// The band arm's `param` comes out of the [`a::MeasureUnavailable`]
/// it CARRIES, at the same attribute name that refusal spells it
/// under — one fault, one word, one payload name, whichever door
/// refused.
fn mc_err(py: Python<'_>, refusal: &a::McRefusal) -> PyErr {
    let text = |s: &str| PyString::new(py, s).unbind().into_any();
    let param = match refusal {
        a::McRefusal::BandHasNoMeasure(a::MeasureUnavailable::BandHasNoMeasure { param }) => {
            text(&param.0)
        }
        a::McRefusal::NoSamples | a::McRefusal::NominalDoesNotBuild { .. } => py.None(),
    };
    let (node, cause) = match refusal {
        a::McRefusal::NominalDoesNotBuild { node, cause } => (
            match super::doc::NodeId(*node).into_pyobject(py) {
                Ok(id) => id.into_any().unbind(),
                Err(failed) => return failed,
            },
            text(cause),
        ),
        a::McRefusal::BandHasNoMeasure(_) | a::McRefusal::NoSamples => (py.None(), py.None()),
    };
    typed_err(
        py,
        ErrorClass::Mc,
        refusal.to_string(),
        &[
            ("variant", text(mc_refusal_tag(refusal))),
            ("param", param),
            ("node", node),
            ("cause", cause),
        ],
    )
}

/// **How one Monte-Carlo run is configured** (ERROR-DESIGN E11.1):
/// the sample count, the seed both ride in the report, and which
/// schedule runs them.
///
/// A frozen value with the kernel's own defaults, so
/// `monte_carlo(doc, box)` is the shipped dial and a caller who wants
/// a tail resolved further asks for more samples and pays linearly.
///
/// `parallel` is a RUNTIME switch and not a build one, because the
/// property it exists to let a caller check is that it changes
/// nothing: each sample is seeded from its own index, so the two
/// schedules produce bit-identical reports.
#[pyclass(frozen, module = "pncad", from_py_object)]
#[derive(Clone, Copy)]
pub(crate) struct McConfig(a::McConfig);

#[pymethods]
impl McConfig {
    #[new]
    #[pyo3(signature = (samples = None, seed = None, parallel = None))]
    fn new(samples: Option<usize>, seed: Option<u64>, parallel: Option<bool>) -> Self {
        let base = a::McConfig::default();
        Self(a::McConfig {
            samples: samples.unwrap_or(base.samples),
            seed: seed.unwrap_or(base.seed),
            parallel: parallel.unwrap_or(base.parallel),
        })
    }

    /// How many samples the run draws.
    #[getter]
    fn samples(&self) -> usize {
        self.0.samples
    }

    /// The stream's seed, recorded in the report.
    #[getter]
    fn seed(&self) -> u64 {
        self.0.seed
    }

    /// Whether the samples run under rayon.
    #[getter]
    fn parallel(&self) -> bool {
        self.0.parallel
    }

    fn __eq__(&self, other: &Self) -> bool {
        self.0 == other.0
    }

    fn __hash__(&self) -> u64 {
        use std::hash::{Hash, Hasher};
        let mut h = std::hash::DefaultHasher::new();
        (self.0.samples, self.0.seed, self.0.parallel).hash(&mut h);
        h.finish()
    }

    fn __repr__(&self) -> String {
        format!(
            "McConfig(samples={}, seed={:#018x}, parallel={})",
            self.0.samples, self.0.seed, self.0.parallel
        )
    }
}

/// **One measure node's empirical summary** — ADVISORY, and the label
/// is on the report this row is not reachable without.
///
/// The statistics are over the samples that HAD a value: `unmeasured`
/// counts the rest and is never averaged over. A measure with no
/// `f64` value at any sample (a `min_clearance`, whose answer is an
/// enclosure) is `unmeasured` at every draw, and then the four
/// statistics are `0/0` — which is why the count is beside them
/// rather than behind them.
#[pyclass(frozen, module = "pncad", skip_from_py_object)]
#[derive(Clone)]
pub(crate) struct McMeasure(a::McMeasure);

#[pymethods]
impl McMeasure {
    /// The measure node this row summarizes.
    #[getter]
    fn node(&self) -> super::doc::NodeId {
        super::doc::NodeId(self.0.node)
    }

    /// The sample mean, over the samples where the measure had a
    /// value.
    #[getter]
    fn mean(&self) -> f64 {
        self.0.mean
    }

    /// The sample standard deviation (the `N − 1` form; `0.0` for one
    /// sample, which is the only value a single draw supports).
    #[getter]
    fn sigma(&self) -> f64 {
        self.0.sigma
    }

    /// The least measured value.
    #[getter]
    fn min(&self) -> f64 {
        self.0.min
    }

    /// The greatest.
    #[getter]
    fn max(&self) -> f64 {
        self.0.max
    }

    /// How many samples produced a value.
    #[getter]
    fn measured(&self) -> usize {
        self.0.measured
    }

    /// How many produced none — a refusing node, or a measure with no
    /// value at `f64`. Counted, never averaged over.
    #[getter]
    fn unmeasured(&self) -> usize {
        self.0.unmeasured
    }

    fn __eq__(&self, other: &Self) -> bool {
        self.0 == other.0
    }

    /// Consistent with [`Self::__eq__`], which is the kernel's
    /// `PartialEq` — IEEE on the statistics, so the two spellings of
    /// zero are one value and the hash folds them together. A row
    /// nothing could sample carries `NaN`, which equals nothing
    /// including itself, so no pair the contract is about reaches
    /// this.
    fn __hash__(&self) -> u64 {
        use std::hash::{Hash, Hasher};
        let mut h = std::hash::DefaultHasher::new();
        self.0.node.0.hash(&mut h);
        for v in [self.0.mean, self.0.sigma, self.0.min, self.0.max] {
            super::doc::fold_zero(v).to_bits().hash(&mut h);
        }
        (self.0.measured, self.0.unmeasured).hash(&mut h);
        h.finish()
    }

    fn __repr__(&self) -> String {
        format!(
            "McMeasure(node={}, mean={}, sigma={}, measured={}, unmeasured={})",
            self.0.node.0, self.0.mean, self.0.sigma, self.0.measured, self.0.unmeasured
        )
    }
}

/// **One assertion node's empirical summary.**
///
/// The three counts partition the samples, and the third is kept OUT
/// of [`Self::violation_fraction`]: an undecided sample is not a
/// passing one, and folding it into either side would invent the
/// verdict E10's third state exists to withhold.
#[pyclass(frozen, module = "pncad", skip_from_py_object)]
#[derive(Clone)]
pub(crate) struct McAssertion(a::McAssertion);

#[pymethods]
impl McAssertion {
    /// The assertion node this row summarizes.
    #[getter]
    fn node(&self) -> super::doc::NodeId {
        super::doc::NodeId(self.0.node)
    }

    /// Samples whose verdict was `Holds`.
    #[getter]
    fn holds(&self) -> usize {
        self.0.holds
    }

    /// Samples whose verdict was `Violated`.
    #[getter]
    fn violated(&self) -> usize {
        self.0.violated
    }

    /// Samples with no verdict at all.
    #[getter]
    fn unevaluated(&self) -> usize {
        self.0.unevaluated
    }

    /// The empirical violation fraction over the DECIDED samples, or
    /// `None` when none were decided.
    #[getter]
    fn violation_fraction(&self) -> Option<f64> {
        self.0.violation_fraction()
    }

    fn __eq__(&self, other: &Self) -> bool {
        self.0 == other.0
    }

    /// Consistent with [`Self::__eq__`]: three counts and a node, all
    /// of them exact.
    fn __hash__(&self) -> u64 {
        use std::hash::{Hash, Hasher};
        let mut h = std::hash::DefaultHasher::new();
        (
            self.0.node.0,
            self.0.holds,
            self.0.violated,
            self.0.unevaluated,
        )
            .hash(&mut h);
        h.finish()
    }

    fn __repr__(&self) -> String {
        format!(
            "McAssertion(node={}, holds={}, violated={}, unevaluated={})",
            self.0.node.0, self.0.holds, self.0.violated, self.0.unevaluated
        )
    }
}

/// **The E11.1 advisory report.** Every number in it is an ESTIMATE,
/// and the count and seed that produced it ride at the top.
///
/// It never gates, it is never persisted as an assertion, and it never
/// enters the mass accounting. The certified answer covers the
/// analyzed box; this one draws from the WHOLE distribution, tail
/// included, so it estimates the quantity the certified lane
/// deliberately does not.
#[pyclass(frozen, module = "pncad")]
pub(crate) struct McReport(a::McReport);

#[pymethods]
impl McReport {
    /// How many samples were drawn.
    #[getter]
    fn samples(&self) -> usize {
        self.0.samples
    }

    /// The seed they were drawn from.
    #[getter]
    fn seed(&self) -> u64 {
        self.0.seed
    }

    /// Per measure node, in the document's own node order.
    #[getter]
    fn measures(&self) -> Vec<McMeasure> {
        self.0.measures.iter().cloned().map(McMeasure).collect()
    }

    /// Per assertion node, in the document's own node order.
    #[getter]
    fn assertions(&self) -> Vec<McAssertion> {
        self.0.assertions.iter().cloned().map(McAssertion).collect()
    }

    /// The fraction of samples that landed OUTSIDE the analyzed box —
    /// the empirical twin of E2's tail term, and the one number here a
    /// reader can check against the certified side.
    #[getter]
    fn outside_box(&self) -> f64 {
        self.0.outside_box
    }

    /// **The human form**, with the advisory label and the dials on
    /// every line that carries an estimate.
    ///
    /// The label repeats per line on purpose: a reader who copies one
    /// line out of a report takes it with them, which a single header
    /// line does not survive. It is the kernel's own rendering, so the
    /// discipline E11.1 requires does not stop at the language
    /// boundary.
    fn render(&self) -> String {
        self.0.render()
    }

    fn __repr__(&self) -> String {
        format!(
            "McReport(ADVISORY, samples={}, seed={:#018x}, measures={}, assertions={})",
            self.0.samples,
            self.0.seed,
            self.0.measures.len(),
            self.0.assertions.len()
        )
    }
}

/// **The advisory lane's one door** (ERROR-DESIGN E11.1): replay the
/// document at `f64` over `config.samples` draws from its own
/// distributions, and summarize.
///
/// `analyzed` is the box the run is measured against — it is what
/// decides which parameters VARY and which offsets count as outside,
/// so it is the caller's explicit choice rather than a default hidden
/// inside the run. The box comes from [`analyzed_box`], which is where
/// the policy that shaped it lives.
///
/// Every number in the answer is an estimate and none of it gates.
/// The certified lane is the only gate, and it is absent from this
/// build: the wheel is the default feature set, which is exactly the
/// caller this lane was un-gated for.
///
/// Raises `McRefusal`: a varying parameter carrying a band, a
/// zero-sample request, or a document that does not build at its
/// nominal.
#[pyfunction]
#[pyo3(signature = (doc, analyzed, config = None))]
fn monte_carlo(
    py: Python<'_>,
    doc: &super::doc::Doc,
    analyzed: &AnalyzedBox,
    config: Option<&McConfig>,
) -> PyResult<McReport> {
    let config = config.map_or_else(a::McConfig::default, |c| c.0);
    let tol = Tol::witness();
    let recipe = &doc.inner;
    let box_ = &analyzed.0;
    // One full document evaluation per sample is the lane's honest
    // price, so the interpreter runs while it is paid.
    let answer = py.detach(|| a::monte_carlo(recipe, box_, &config, tol));
    match answer {
        Ok(report) => Ok(McReport(report)),
        Err(refusal) => Err(mc_err(py, &refusal)),
    }
}

/// **The offset a distribution puts at quantile `u`** — inverse-
/// transform sampling's one door, and the advisory lane's only way to
/// draw a parameter value.
///
/// `u` is a uniform draw in `[0, 1)`. The answer is an OFFSET from the
/// nominal in the distribution's own dimension, so a `Length`
/// parameter's offset is a `Length` — the same borrow every other door
/// in this module makes, and the reason `param` is the NAME and the
/// dimension comes off the distribution.
///
/// It draws from the WHOLE law, never from the analyzed box: the tail
/// the box excludes is exactly the region the certified answer does
/// not cover, so an estimator that clipped it would be estimating the
/// same restriction twice.
///
/// Raises `MeasureUnavailable` for a band: limits without a shape
/// cannot be sampled, and promoting one to uniform is the E2 violation
/// this whole module refuses.
#[pyfunction]
fn sample_offset(
    py: Python<'_>,
    param: &super::doc::ParamName,
    dist: &Distribution,
    u: f64,
) -> PyResult<Py<PyAny>> {
    match a::sample_offset(&param.0, &dist.inner, u) {
        Ok(offset) => quantity(py, offset, dist.dim),
        Err(err) => Err(measure_err(py, &err)),
    }
}

/// Register the analysis vocabulary on the module.
pub(crate) fn register(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Distribution>()?;
    m.add_class::<AnalysisPolicy>()?;
    m.add_class::<AnalyzedParam>()?;
    m.add_class::<AnalyzedBox>()?;
    m.add_class::<McConfig>()?;
    m.add_class::<McMeasure>()?;
    m.add_class::<McAssertion>()?;
    m.add_class::<McReport>()?;
    m.add_function(wrap_pyfunction!(analyzed_box, m)?)?;
    m.add_function(wrap_pyfunction!(monte_carlo, m)?)?;
    m.add_function(wrap_pyfunction!(sample_offset, m)?)?;
    m.add("DEFAULT_QUANTILE_MASS", a::DEFAULT_QUANTILE_MASS)?;
    m.add("DEFAULT_SAMPLES", a::DEFAULT_SAMPLES)?;
    m.add("DEFAULT_SEED", a::DEFAULT_SEED)?;
    Ok(())
}
