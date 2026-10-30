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
//! # The references cross as pairs, or as poses
//!
//! Each reference to a cell is a kernel `SitedRef` — a `StableName` and
//! the node its carrier is READ AT — and it crosses as the `(NodeId,
//! str)` pair `Mate` already takes each of its two face sides as. The
//! type itself never crosses: a name is opaque text by the ordinal-28
//! contract, and a read site is a node id, so a class wrapping the two
//! would add ceremony and no reach. A `distance` or a `gap` reference
//! may instead be a pose (D10 `[assertions.quiet]`): a `PoseDef` read
//! off a cell, or a pose `Var`, such as a split's tool.
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
//! The engine's own refusal (`MinSeparationRefusal`) is therefore not
//! reachable from Python at all: only a certifying scalar holds the
//! engine's door, and the binding does not evaluate at one. It
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
    let (d::MeasureUnavailableAt::NeedsEnclosure { verb, scalar, door }
    | d::MeasureUnavailableAt::NoUpperBound { verb, scalar, door }) = reason;
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
/// The shape is `StationMove`'s: a frozen value class of static
/// constructors, one per kernel arm, spelled in snake case. Each reference is a `(node, name)` pair: the entity's stable
/// name, and the node its carrier is READ AT. A text that is not a name
/// refuses here with `ValueError`.
#[pyclass(frozen, module = "pncad", from_py_object)]
#[derive(Clone)]
pub(crate) struct MeasurePrimitive {
    /// The primitive, each reference the operand it is read as.
    pub(crate) inner: d::MeasurePrimitive<d::Operand>,
    /// The two references as they were passed, in argument order.
    refs: [MeasureRef; 2],
}

/// A `(node, name)` pair as the kernel's sited reference.
fn sited((at, name): (NodeId, String)) -> PyResult<d::SitedRef> {
    Ok(d::SitedRef::new(at.0, super::doc::name_from_text(&name)?))
}

/// **One reference of a measure**: a `(node, name)` pair naming a cell,
/// or, for a `distance` or a `gap`, a pose: one defined where it is read
/// (`PoseDef`, such as `PoseDef.axis(...)` off a face), or a pose `Var`.
#[derive(FromPyObject, Clone)]
pub(crate) enum MeasureRef {
    /// A cell, by the node it is read at and its name there.
    Cell((NodeId, String)),
    /// A pose defined at the seat.
    Pose(super::pose::PoseDef),
    /// A variable, read as itself.
    Var(super::doc::Var),
}

impl MeasureRef {
    /// The operand the kernel reads.
    fn operand(&self) -> PyResult<d::Operand> {
        Ok(match self {
            Self::Cell(pair) => sited(pair.clone())?.into(),
            Self::Pose(pose) => pose.inner.clone(),
            Self::Var(var) => d::Operand::Var(var.0),
        })
    }

    /// The reference as it was passed.
    fn object(&self, py: Python<'_>) -> PyResult<Py<PyAny>> {
        Ok(match self {
            Self::Cell(pair) => pair.clone().into_pyobject(py)?.into_any().unbind(),
            Self::Pose(pose) => Py::new(py, pose.clone())?.into_any(),
            Self::Var(var) => Py::new(py, *var)?.into_any(),
        })
    }
}

impl MeasurePrimitive {
    /// The primitive `make` builds over `a` and `b`.
    fn of(
        a: MeasureRef,
        b: MeasureRef,
        make: fn(d::Operand, d::Operand) -> d::MeasurePrimitive<d::Operand>,
    ) -> PyResult<Self> {
        Ok(Self {
            inner: make(a.operand()?, b.operand()?),
            refs: [a, b],
        })
    }
}

#[pymethods]
impl MeasurePrimitive {
    /// The distance between two referenced entities — a `Length`.
    ///
    /// The v1 carrier scope is the kernel's: a pair of carriers the
    /// closed form has no arm for refuses at `evaluate`
    /// (`measure_unsupported`), naming the pair, and never guesses.
    ///
    /// A reference may be a pose instead: a point, an axis or a plane
    /// read off a cell (`PoseDef.point`, `PoseDef.axis`,
    /// `PoseDef.plane`), or a split's tool. Two axes read as one
    /// scalar, zero where they are one line or meet; an assertion
    /// `= 0` over the poses read off an unproven coincidence's cells
    /// quiets it.
    #[staticmethod]
    fn distance(a: MeasureRef, b: MeasureRef) -> PyResult<Self> {
        Self::of(a, b, |a, b| d::MeasurePrimitive::Distance { a, b })
    }

    /// The angle between two referenced entities — an `Angle`.
    #[staticmethod]
    fn angle(a: (NodeId, String), b: (NodeId, String)) -> PyResult<Self> {
        Self::of(MeasureRef::Cell(a), MeasureRef::Cell(b), |a, b| {
            d::MeasurePrimitive::Angle { a, b }
        })
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
        Self::of(MeasureRef::Cell(a), MeasureRef::Cell(b), |a, b| {
            d::MeasurePrimitive::MinClearance { a, b }
        })
    }

    /// C5's SIGNED gap between a mating pair — a `Length`.
    ///
    /// **Argument order is the mating ROLE, not a symmetry.** `outer`
    /// is the containing carrier (the plane the offset is measured
    /// FROM, the outer of two nested faces) and `inner` the contained
    /// one, authored rather than inferred from which radius is larger.
    /// Two faces are read by both material sides: an opposed pair (a
    /// rod on a plate, two rods side by side, a pin in a bore) reads
    /// the same in either order, an aligned one (a flush pair) negates.
    ///
    /// Each reference names a face, an edge or a vertex, or is a plane
    /// pose (a split's tool) against a vertex. Between a face and an
    /// edge or a vertex, or an edge and a vertex, the
    /// higher-dimensional cell's material reads the other in either
    /// seat; between two edges or two vertices the outer's reads the
    /// inner.
    #[staticmethod]
    fn gap(outer: MeasureRef, inner: MeasureRef) -> PyResult<Self> {
        Self::of(outer, inner, |outer, inner| d::MeasurePrimitive::Gap {
            outer,
            inner,
        })
    }

    /// The verb, as the one stable word: `"distance"`, `"angle"`,
    /// `"gap"` or `"min_clearance"` — the kernel's own `verb()`, which
    /// is also what its refusals name themselves with.
    #[getter]
    fn verb(&self) -> &'static str {
        self.inner.verb()
    }

    /// What this primitive measures: `"length"` or `"angle"`. Fixed
    /// per verb — E3's "the quantity kind rides the expression".
    #[getter]
    fn dimension(&self) -> &'static str {
        dimension_tag(self.inner.dim())
    }

    /// The two references, in ARGUMENT order, as they were passed (a
    /// `(node, name)` pair, a `PoseDef` or a `Var`): a `gap`'s pair
    /// reads `(outer, inner)` and not a re-sorted one.
    #[getter]
    fn refs(&self, py: Python<'_>) -> PyResult<(Py<PyAny>, Py<PyAny>)> {
        let [a, b] = &self.refs;
        Ok((a.object(py)?, b.object(py)?))
    }

    fn __eq__(&self, other: &Self) -> bool {
        self.inner == other.inner
    }

    fn __hash__(&self) -> u64 {
        use std::hash::{Hash, Hasher};
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        format!("{:?}", self.inner).hash(&mut hasher);
        hasher.finish()
    }

    fn __repr__(&self) -> String {
        format!("MeasurePrimitive.{}(…)", self.inner.verb())
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
/// The direction is the load-bearing one: `to_kernel` matches on
/// `Self`, a closed local enum, so it says nothing about the kernel growing. This match is over the
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
