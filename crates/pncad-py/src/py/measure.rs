//! **Authoring a measurement** (ERROR-DESIGN E3/E10, CONTACT-DESIGN
//! C5): the vocabulary a `Node.measure` and a `Node.assertion` are
//! built from, and the two refusals the fourth verb adds.
//!
//! The READING half already ships in `py/value.rs` — `Value.measure`
//! answers a `Measurement` and `Value.assertion` a `Verdict`. This
//! module is the other direction: what a Python caller WRITES so that
//! there is a web and a verdict to read.
//!
//! # Two layers, and only one of them is new here
//!
//! [`MeasureExpr`] is `Expr`'s arithmetic over two leaf kinds — an
//! ordinary document expression, and a [`MeasurePrimitive`] naming a
//! closed-form measurement of entities the NODE references. The
//! ordinary expression enters through `Doc.parse_expr`, the one text
//! door this surface has: there is no second spelling of the grammar
//! here, exactly as `py/expr.rs` rules for the builders it leaves out.
//!
//! # The references live on the node, and cross as pairs
//!
//! A primitive addresses its operands by INDEX into the node's
//! reference list, never by name, so the expression stays a pure
//! value. Each reference is a kernel `SitedRef` — a `StableName` and
//! the node its carrier is READ AT — and it crosses as the
//! `(NodeId, str)` pair `Node.mate` already takes each of its two
//! sides as. The type itself never crosses: a name is opaque text by
//! the ordinal-28 contract, and a read site is a node id, so a class
//! wrapping the two would add ceremony and no reach.
//!
//! **The read site is what makes a measure report PLACED geometry.**
//! Selecting a face from a transform's own selection door and
//! measuring it gives the moved number; naming the minting node gives
//! the authored one. Both are legal and they are different questions.
//!
//! # The F1 lattice is asked, not restated
//!
//! Every arithmetic constructor here calls the kernel's, which builds
//! probe expressions at the operand dimensions and runs `Expr`'s own
//! smart constructors over them. So a mis-dimensioned measurement
//! refuses in the same words a document expression would have earned,
//! and it refuses at the CONSTRUCTOR rather than at the `Doc.apply`
//! three lines later — the timing `Distribution`'s constructors buy,
//! for the same reason.
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
//! The engine's own refusal (`MinClearanceRefusal`) is therefore not
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
use crate::tags::{expr_dimension_error_tag, measure_node_fault_tag, measure_unavailable_at_tag};
use pncad::document as d;

/// Raise `MeasureNodeFault` — the construction door's refusal, with
/// the whole fault projected.
///
/// The projected shape `py/readback.rs` states: `variant` plus every
/// arm's payload, present on every arm, so `getattr` never raises and
/// a caller never has to branch on `variant` before reading a field.
/// The type has one arm today and the shape is written for the second
/// one anyway, exactly as its Rust `match` is.
pub(crate) fn measure_node_fault_err(py: Python<'_>, fault: &d::MeasureNodeFault) -> PyErr {
    let d::MeasureNodeFault::RefIndexOutOfRange { verb, index, refs } = fault;
    let numbers = (index.into_pyobject(py), refs.into_pyobject(py));
    let (index, refs) = match numbers {
        (Ok(index), Ok(refs)) => (index.into_any().unbind(), refs.into_any().unbind()),
        (Err(failed), _) | (_, Err(failed)) => return failed.into(),
    };
    typed_err(
        py,
        ErrorClass::MeasureNode,
        fault.to_string(),
        &[
            (
                "variant",
                PyString::new(py, measure_node_fault_tag(fault))
                    .unbind()
                    .into_any(),
            ),
            ("verb", PyString::new(py, verb).unbind().into_any()),
            ("index", index),
            ("refs", refs),
        ],
    )
}

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

/// The expression layer's own `DimensionError`, refused at a
/// measurement constructor.
///
/// It is `LiteralError`, the class that already IS this kernel type
/// crossing: the measurement sublanguage does not restate the F1
/// table, it builds probe expressions and runs `Expr`'s own
/// constructors, so the refusal a caller gets here is the one a
/// document expression would have earned. `value` is `None` because
/// this door refuses over two operands' DIMENSIONS and has no single
/// number in hand.
fn measure_dimension_err(py: Python<'_>, err: &d::DimensionError) -> PyErr {
    typed_err(
        py,
        ErrorClass::Literal,
        err.to_string(),
        &[
            (
                "kind",
                PyString::new(py, expr_dimension_error_tag(err))
                    .unbind()
                    .into_any(),
            ),
            ("value", py.None()),
        ],
    )
}

/// **Which closed-form measurement a leaf computes**, and over which
/// of the node's references.
///
/// Four verbs and no fifth. `distance` and `angle` are the two
/// geometric readings, `gap` is C5's SIGNED mating gap, and
/// `min_clearance` is the one an engine answers.
///
/// The shape is `PatternKind`'s and `PartSelect`'s: a frozen value
/// class of static constructors, one per kernel arm, spelled in snake
/// case.
///
/// **The arguments are POSITIONS, not measurements.** Each is an index
/// into the reference list `Node.measure` is given, so a plain `int`
/// is the right type — the structural-slot exception `NodePick.build`'s
/// `body` and `PartSelect.instance`'s `index` already ride. An index
/// past the end of the list refuses at `Node.measure` with
/// `MeasureNodeFault`; a negative one is not a `u32` and is a
/// `OverflowError` from the argument conversion, exactly as it is at
/// `NodePick.build`.
#[pyclass(frozen, module = "pncad", from_py_object)]
#[derive(Clone, Copy)]
pub(crate) struct MeasurePrimitive(pub(crate) d::MeasurePrimitive);

#[pymethods]
impl MeasurePrimitive {
    /// The distance between two referenced entities — a `Length`.
    ///
    /// The v1 carrier scope is the kernel's: a pair of carriers the
    /// closed form has no arm for refuses at `evaluate`
    /// (`measure_unsupported`), naming the pair, and never guesses.
    #[staticmethod]
    fn distance(a: u32, b: u32) -> Self {
        Self(d::MeasurePrimitive::Distance { a, b })
    }

    /// The angle between two referenced entities — an `Angle`.
    #[staticmethod]
    fn angle(a: u32, b: u32) -> Self {
        Self(d::MeasurePrimitive::Angle { a, b })
    }

    /// **The minimum clearance between two selections** — a `Length`,
    /// and the one verb whose value is an ENCLOSURE.
    ///
    /// Each reference's entity kind is the selection's face scope: a
    /// BODY reference selects every face of that body, a FACE
    /// reference selects the one. Anything else (an edge, a vertex, a
    /// datum) refuses at `evaluate` with `measure_selection_kind`.
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
    fn min_clearance(a: u32, b: u32) -> Self {
        Self(d::MeasurePrimitive::MinClearance { a, b })
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
    fn gap(outer: u32, inner: u32) -> Self {
        Self(d::MeasurePrimitive::Gap { outer, inner })
    }

    /// The verb, as the one stable word: `"distance"`, `"angle"`,
    /// `"gap"` or `"min_clearance"`.
    ///
    /// The kernel's own `verb()`, which is also what its refusals
    /// name themselves with — so a caller comparing a
    /// `MeasureNodeFault.verb` against a primitive it built is
    /// comparing one vocabulary with itself.
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

    /// The reference indices this primitive reads, in ARGUMENT order.
    ///
    /// The kernel's own `refs()`, so a `gap`'s pair reads
    /// `(outer, inner)` and not a re-sorted one.
    #[getter]
    fn refs(&self) -> (u32, u32) {
        let [a, b] = self.0.refs();
        (a, b)
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
        let (a, b) = self.refs();
        format!("MeasurePrimitive.{}({a}, {b})", self.0.verb())
    }
}

/// Which way an assertion constrains its measure (E10).
///
/// Two directions and no third, and both still gate: a clearance
/// requirement is `AtLeast`, a maximum-gap requirement is `AtMost`.
///
/// Rust has ONE `AssertionDir` — the kernel enum the recipe node
/// carries — and this is its binding. The mirror exists because
/// `#[pyclass]` cannot be attached to a type from another crate; the
/// obligation it owes the kernel is that every kernel direction has a
/// member here, which [`_binds_every_kernel_direction`] enforces.
#[pyclass(eq, eq_int, frozen, hash, module = "pncad", from_py_object)]
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum AssertionDir {
    /// The measured quantity must be at least the bound.
    AtLeast,
    /// The measured quantity must be at most the bound.
    AtMost,
}

#[pymethods]
impl AssertionDir {
    /// The relation as it reads in a report: `">="` or `"<="`.
    ///
    /// The kernel's own `symbol()`, not a second rendering.
    #[getter]
    fn symbol(&self) -> &'static str {
        self.to_kernel().symbol()
    }
}

impl AssertionDir {
    pub(crate) fn to_kernel(self) -> d::AssertionDir {
        match self {
            Self::AtLeast => d::AssertionDir::AtLeast,
            Self::AtMost => d::AssertionDir::AtMost,
        }
    }
}

/// Every kernel direction has a member on the Python mirror.
///
/// The direction is the load-bearing one, exactly as it is for
/// `BooleanOp`: `to_kernel` matches on `Self`, a closed local enum, so
/// it says nothing about the kernel growing. This match is over the
/// KERNEL enum, so a direction added there breaks this build and the
/// binding must be written. Never called; the type-checked match is
/// the whole product.
const fn _binds_every_kernel_direction(kernel: d::AssertionDir) -> AssertionDir {
    match kernel {
        d::AssertionDir::AtLeast => AssertionDir::AtLeast,
        d::AssertionDir::AtMost => AssertionDir::AtMost,
    }
}

/// **A dimension-checked measurement expression**: `Expr`'s arithmetic
/// over a closed-form measurement leaf.
///
/// Private fields and fallible constructors, exactly as the kernel's:
/// an ill-dimensioned tree is unrepresentable, so `dimension` is
/// trustworthy by construction.
///
/// **No `__hash__`**, for `Expr`'s reason: equality is the kernel's
/// own `PartialEq`, an IEEE comparison of the literals inside, so
/// `0.0` and `-0.0` are equal trees whose bit patterns are not, and
/// there is no hash that respects the first without lying about the
/// second.
#[pyclass(frozen, module = "pncad", from_py_object)]
#[derive(Clone)]
pub(crate) struct MeasureExpr(pub(crate) d::MeasureExpr);

#[pymethods]
impl MeasureExpr {
    /// A closed-form measurement leaf. Total — the primitive's
    /// dimension is fixed by its verb, so there is nothing to refuse.
    #[staticmethod]
    fn primitive(p: &MeasurePrimitive) -> Self {
        Self(d::MeasureExpr::primitive(p.0))
    }

    /// An ordinary document expression as a leaf — a literal bound, a
    /// parameter, a whole arithmetic subtree of them.
    ///
    /// `Doc.parse_expr` is where one comes from, and it is the only
    /// door: the checking parser reaches the whole algebra through a
    /// single call, and a second spelling of that grammar is what
    /// `py/expr.rs` already rules out.
    #[staticmethod]
    fn value(e: &super::expr::Expr) -> Self {
        Self(d::MeasureExpr::value(e.0.clone()))
    }

    /// Same-dimension addition.
    #[staticmethod]
    fn add(py: Python<'_>, a: &Self, b: &Self) -> PyResult<Self> {
        d::MeasureExpr::add(a.0.clone(), b.0.clone())
            .map(Self)
            .map_err(|err| measure_dimension_err(py, &err))
    }

    /// Same-dimension subtraction.
    #[staticmethod]
    fn sub(py: Python<'_>, a: &Self, b: &Self) -> PyResult<Self> {
        d::MeasureExpr::sub(a.0.clone(), b.0.clone())
            .map(Self)
            .map_err(|err| measure_dimension_err(py, &err))
    }

    /// Negation — any dimension, and total.
    #[staticmethod]
    fn neg(a: &Self) -> Self {
        Self(d::MeasureExpr::neg(a.0.clone()))
    }

    /// Product; the F1 rule, at least one operand dimensionless.
    #[staticmethod]
    fn mul(py: Python<'_>, a: &Self, b: &Self) -> PyResult<Self> {
        d::MeasureExpr::mul(a.0.clone(), b.0.clone())
            .map(Self)
            .map_err(|err| measure_dimension_err(py, &err))
    }

    /// Quotient; the divisor must be dimensionless (F1).
    #[staticmethod]
    fn div(py: Python<'_>, a: &Self, b: &Self) -> PyResult<Self> {
        d::MeasureExpr::div(a.0.clone(), b.0.clone())
            .map(Self)
            .map_err(|err| measure_dimension_err(py, &err))
    }

    /// Same-dimension lattice minimum.
    #[staticmethod]
    fn min(py: Python<'_>, a: &Self, b: &Self) -> PyResult<Self> {
        d::MeasureExpr::min(a.0.clone(), b.0.clone())
            .map(Self)
            .map_err(|err| measure_dimension_err(py, &err))
    }

    /// Same-dimension lattice maximum.
    #[staticmethod]
    fn max(py: Python<'_>, a: &Self, b: &Self) -> PyResult<Self> {
        d::MeasureExpr::max(a.0.clone(), b.0.clone())
            .map(Self)
            .map_err(|err| measure_dimension_err(py, &err))
    }

    /// What this expression measures: `"length"`, `"angle"`,
    /// `"count"` or `"scalar"`.
    ///
    /// Correct by construction — the F1 checker computed it as the
    /// tree was built — and the fact an assertion's bound has to
    /// match, which the edit door checks (`assertion_dimension`).
    #[getter]
    fn dimension(&self) -> &'static str {
        dimension_tag(self.0.dim())
    }

    /// Every primitive in the tree, in PRE-ORDER — the same order the
    /// node door's bounds check runs over and the evaluation walk
    /// reads them back in.
    #[getter]
    fn primitives(&self) -> Vec<MeasurePrimitive> {
        let mut out = Vec::new();
        self.0.primitives(&mut out);
        out.into_iter().map(MeasurePrimitive).collect()
    }

    fn __eq__(&self, other: &Self) -> bool {
        self.0 == other.0
    }

    fn __repr__(&self) -> String {
        format!("MeasureExpr({})", dimension_tag(self.0.dim()))
    }
}

/// Register the measurement authoring vocabulary on the module.
pub(crate) fn register(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<MeasurePrimitive>()?;
    m.add_class::<AssertionDir>()?;
    m.add_class::<MeasureExpr>()?;
    Ok(())
}
