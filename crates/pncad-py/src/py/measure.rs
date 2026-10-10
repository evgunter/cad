//! **Authoring a measurement** (ERROR-DESIGN E3/E10, CONTACT-DESIGN
//! C5): the vocabulary a `Node.measure` and a `Node.assertion` are
//! built from, and the refusal the fourth verb adds.
//!
//! The READING half ships in `py/value.rs` — `Value.measure` answers a
//! `Measurement`, `Evaluation.reading` a measured value no node holds,
//! and `Value.assertion` a `Verdict`. This module is the other
//! direction: what a Python caller WRITES so that there is a value and
//! a verdict to read.
//!
//! # One measure, one primitive; arithmetic is a formula
//!
//! A [`MeasurePrimitive`] is one closed-form measurement of two
//! references, and `Node.measure` is the node holding it; its value is
//! the node's output (`Doc.output`). Arithmetic over measured values is
//! an ordinary `Formula` over those outputs, which an assertion reads
//! or a definition names: there is no second arithmetic language.
//!
//! # The references cross as pairs
//!
//! Each reference is a kernel `SitedRef` — a `StableName` and the node
//! its carrier is READ AT — and it crosses as the `(NodeId, str)` pair
//! `Node.mate` already takes each of its two sides as. The type itself
//! never crosses: a name is opaque text by the ordinal-28 contract, and
//! a read site is a node id, so a class wrapping the two would add
//! ceremony and no reach.
//!
//! **The read site is what makes a measure report PLACED geometry.**
//! Selecting a face from a transform's own selection door and
//! measuring it gives the moved number; naming the minting node gives
//! the authored one. Both are legal and they are different questions.
//!
//! # The fourth verb, and the scalar it needs
//!
//! `min_clearance` is answered by an engine rather than a closed form,
//! and its value is an ENCLOSURE. Python evaluates at `f64` alone, and
//! a point scalar has nowhere to put one: the measure evaluates fine
//! and has no value, which reaches a caller as
//! [`MeasureUnavailableAt`](crate::errors::ErrorClass::MeasureUnavailableAt)
//! naming the verb, the scalar and the door that could answer. That is
//! a typed ABSENCE and not a failure, which is why an assertion over
//! such a measure reports `Unevaluated` rather than being poisoned.
//!
//! The engine's own refusal (`ClearanceRefusal`) is therefore not
//! reachable from Python at all: the only lane that computes a bracket
//! is the interval one, and the binding does not evaluate there. It
//! would arrive as `EvaluationError` with
//! `kind == "measure_clearance_refused"` the day Python gains a
//! certified scalar, and it should gain its spelling in the unit that
//! brings one.

use pyo3::prelude::*;
use pyo3::types::PyString;

use crate::errors::{ErrorClass, dimension_tag};
use crate::py::typed_err;
use crate::tags::measure_unavailable_at_tag;
use pncad::document as d;

use super::doc::NodeId;

/// Raise `MeasureUnavailableAt` — the typed absence a point scalar
/// answers an enclosure-valued measure with.
///
/// Every field of every arm, and the DOOR among them: the refusal
/// carries where the answer lives rather than a worse answer, which
/// is the whole shape of the kernel type.
pub(crate) fn measure_unavailable_at_err(
    py: Python<'_>,
    reason: &d::MeasureUnavailableAt,
) -> PyErr {
    let d::MeasureUnavailableAt::NeedsEnclosure { verb, scalar, door } = reason;
    let text = |s: &str| PyString::new(py, s).unbind().into_any();
    typed_err(
        py,
        ErrorClass::MeasureUnavailableAt,
        reason.to_string(),
        &[
            ("variant", text(measure_unavailable_at_tag(reason))),
            ("verb", text(verb)),
            ("scalar", text(scalar)),
            ("door", text(door)),
        ],
    )
}

/// **Which closed-form measurement a measure computes**, and of which
/// two references.
///
/// Four verbs and no fifth. `distance` and `angle` are the two
/// geometric readings, `gap` is C5's SIGNED mating gap, and
/// `min_clearance` is the one an engine answers.
///
/// The shape is `PatternKind`'s and `PartSelect`'s: a frozen value
/// class of static constructors, one per kernel arm, spelled in snake
/// case. Each reference is a `(node, name)` pair: the entity's stable
/// name, and the node its carrier is READ AT. A text that is not a name
/// refuses here with `ValueError`.
#[pyclass(frozen, module = "pncad", from_py_object)]
#[derive(Clone)]
pub(crate) struct MeasurePrimitive(pub(crate) d::MeasurePrimitive);

/// A `(node, name)` pair as the kernel's sited reference.
fn sited((at, name): (NodeId, String)) -> PyResult<d::SitedRef> {
    Ok(d::SitedRef::new(at.0, super::doc::name_from_text(&name)?))
}

#[pymethods]
impl MeasurePrimitive {
    /// The distance between two referenced entities — a `Length`.
    ///
    /// The v1 carrier scope is the kernel's: a pair of carriers the
    /// closed form has no arm for refuses at `evaluate`
    /// (`measure_unsupported`), naming the pair, and never guesses.
    #[staticmethod]
    fn distance(a: (NodeId, String), b: (NodeId, String)) -> PyResult<Self> {
        Ok(Self(d::MeasurePrimitive::Distance {
            a: sited(a)?,
            b: sited(b)?,
        }))
    }

    /// The angle between two referenced entities — an `Angle`.
    #[staticmethod]
    fn angle(a: (NodeId, String), b: (NodeId, String)) -> PyResult<Self> {
        Ok(Self(d::MeasurePrimitive::Angle {
            a: sited(a)?,
            b: sited(b)?,
        }))
    }

    /// **The minimum clearance between two selections** — a `Length`,
    /// and the one verb whose value is an ENCLOSURE.
    ///
    /// Each reference's entity kind is the selection's face scope: a
    /// BODY reference selects every face of that body, a FACE
    /// reference selects the one. Anything else (an edge, a vertex)
    /// refuses when the measure is inserted, `slot_var_kind`.
    ///
    /// At the `f64` scalar Python evaluates at, this measure HAS NO
    /// VALUE: a station pair found by a point-scalar search is an
    /// upper bound on the minimum rather than the minimum, and
    /// reporting one would be the degradation E7 forbids by name. So
    /// `Value.measure` raises `MeasureUnavailableAt` naming the door
    /// that could answer, and an assertion over it reports
    /// `Unevaluated` carrying the same reason. The measure node itself
    /// evaluates successfully — the absence is a value, not a failure.
    #[staticmethod]
    fn min_clearance(a: (NodeId, String), b: (NodeId, String)) -> PyResult<Self> {
        Ok(Self(d::MeasurePrimitive::MinClearance {
            a: sited(a)?,
            b: sited(b)?,
        }))
    }

    /// C5's SIGNED gap between a mating pair — a `Length`.
    ///
    /// **Argument order is the mating ROLE, not a symmetry.** `outer`
    /// is the containing carrier (the socket, the bore, the plane the
    /// offset is measured FROM) and `inner` the contained one (the
    /// ball, the pin). C5's formulas are asymmetric in exactly that
    /// way, so the roles are authored rather than inferred from which
    /// radius is larger.
    #[staticmethod]
    fn gap(outer: (NodeId, String), inner: (NodeId, String)) -> PyResult<Self> {
        Ok(Self(d::MeasurePrimitive::Gap {
            outer: sited(outer)?,
            inner: sited(inner)?,
        }))
    }

    /// The verb, as the one stable word: `"distance"`, `"angle"`,
    /// `"gap"` or `"min_clearance"` — the kernel's own `verb()`, which
    /// is also what its refusals name themselves with.
    #[getter]
    fn verb(&self) -> &'static str {
        self.0.verb()
    }

    /// What this primitive measures: `"length"` or `"angle"`. Fixed
    /// per verb — E3's "the quantity kind rides the expression".
    #[getter]
    fn dimension(&self) -> &'static str {
        dimension_tag(self.0.dim())
    }

    /// The two references, in ARGUMENT order, as `(node, name)` pairs:
    /// a `gap`'s pair reads `(outer, inner)` and not a re-sorted one.
    #[getter]
    fn refs(&self, py: Python<'_>) -> PyResult<((NodeId, String), (NodeId, String))> {
        let [a, b] = self.0.refs();
        let pair = |r: &d::SitedRef| -> PyResult<(NodeId, String)> {
            Ok((NodeId(r.at), super::doc::name_text(py, &r.name)?))
        };
        Ok((pair(a)?, pair(b)?))
    }

    fn __eq__(&self, other: &Self) -> bool {
        self.0 == other.0
    }

    fn __hash__(&self) -> u64 {
        use std::hash::{Hash, Hasher};
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        self.0.hash(&mut hasher);
        hasher.finish()
    }

    fn __repr__(&self) -> String {
        format!("MeasurePrimitive.{}(…)", self.0.verb())
    }
}

/// The relation an assertion states between its value and its bound
/// (D10: `≥`, `≤`, `=`).
///
/// Rust has ONE `AssertionRelation` — the kernel enum the recipe node
/// carries — and this is its binding. The mirror exists because
/// `#[pyclass]` cannot be attached to a type from another crate; the
/// obligation it owes the kernel is that every kernel relation has a
/// member here, which [`_binds_every_kernel_relation`] enforces.
#[pyclass(eq, eq_int, frozen, hash, module = "pncad", from_py_object)]
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum AssertionRelation {
    /// The measured quantity must be at least the bound.
    AtLeast,
    /// The measured quantity must be at most the bound.
    AtMost,
    /// The measured quantity must equal the bound, at the document's
    /// tolerance.
    Equal,
}

#[pymethods]
impl AssertionRelation {
    /// The relation as it reads in a report: `">="`, `"<="` or `"="`.
    ///
    /// The kernel's own `symbol()`, not a second rendering.
    #[getter]
    fn symbol(&self) -> &'static str {
        self.to_kernel().symbol()
    }
}

impl AssertionRelation {
    pub(crate) fn to_kernel(self) -> d::AssertionRelation {
        match self {
            Self::AtLeast => d::AssertionRelation::AtLeast,
            Self::AtMost => d::AssertionRelation::AtMost,
            Self::Equal => d::AssertionRelation::Equal,
        }
    }
}

/// Every kernel relation has a member on the Python mirror.
///
/// The direction is the load-bearing one, exactly as it is for
/// `ExtrudeSide`: `to_kernel` matches on `Self`, a closed local enum, so
/// it says nothing about the kernel growing. This match is over the
/// KERNEL enum, so a relation added there breaks this build and the
/// binding must be written. Never called; the type-checked match is
/// the whole product.
const fn _binds_every_kernel_relation(kernel: d::AssertionRelation) -> AssertionRelation {
    match kernel {
        d::AssertionRelation::AtLeast => AssertionRelation::AtLeast,
        d::AssertionRelation::AtMost => AssertionRelation::AtMost,
        d::AssertionRelation::Equal => AssertionRelation::Equal,
    }
}

/// Register the measurement authoring vocabulary on the module.
pub(crate) fn register(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<MeasurePrimitive>()?;
    m.add_class::<AssertionRelation>()?;
    Ok(())
}
