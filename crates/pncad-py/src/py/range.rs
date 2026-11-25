//! **The certified range of one field**: over which interval around a
//! parameter's or a slot's current value the document is PROVABLY the
//! same build (`pncad::analysis::RangeSearch`).
//!
//! # Why this door is bound and the rest of the certified half is not
//!
//! The search replays intervals at the certified scalar INSIDE the
//! kernel and answers in plain numbers: offsets from the field's
//! nominal, a point proven different, the reason ground is undecided.
//! Nothing the certified scalar evaluates to crosses, so a binding
//! that never evaluates at that scalar itself still carries the whole
//! answer.
//!
//! # What the answer keeps apart
//!
//! Three states per side, and a binding that folded them into a pair
//! of numbers would turn "the search could not decide" into "the edge
//! is here". [`RangeSide::proven_to`] is how far the proof reaches;
//! [`RangeSide::reached_seed_end`] and [`RangeSide::different`] are
//! how the side ENDS, where the search can say; [`RangeSide::not_decided`]
//! is the ground at the frontier it did not decide, which is never a
//! bound. [`CertifiedRange::pinned`] is the condition the whole answer
//! holds under.
//!
//! # Offsets carry the field's dimension
//!
//! Every position is an OFFSET from the field's nominal and crosses in
//! the dimension of the variable the field is boxed through, as a
//! distribution's offsets do: a `Length` field's seed and proven
//! interval are `Length`s. A seed written in another dimension is a
//! `QuantityOpMismatch` at the door.
//!
//! # The budget
//!
//! A replay is a document evaluation at the certified scalar, and how
//! many a caller can afford is not something this door can know.
//! [`RangeSearch`] is stepped, so its caller's loop IS the replay
//! budget, and each step takes its own cancel; the one-shot
//! [`certified_range`] takes `max_replays` as a required keyword rather
//! than choosing one. The depth defaults to what the seed needs
//! (`RangeBudget::for_seed`), which costs nothing past a side's first
//! proof.

use std::sync::{Mutex, PoisonError};

use pyo3::prelude::*;
use pyo3::types::PyString;

use super::analysis::{Offset, agreed, dimension_mismatch, offset, quantity};
use super::doc::{NodeId, Var, VarArg};
use crate::errors::{ErrorClass, dimension_tag};
use crate::py::typed_err;
use crate::tags::{
    budget_kind_tag, difference_kind_tag, range_refusal_tag, range_step_tag, slot_id_tag,
    unproven_tag,
};
use pncad::analysis as a;
use pncad::document as d;
use pncad::tolerance::Tol;

/// Raise `RangeRefusal` with the whole payload projected: `variant`
/// plus every arm's payload, present on every arm and `None` where
/// that arm does not carry one.
fn range_err(py: Python<'_>, refusal: &a::RangeRefusal) -> PyErr {
    use a::RangeRefusal as R;
    let text = |s: &str| PyString::new(py, s).unbind().into_any();
    let object = |value: Option<f64>| -> PyResult<Py<PyAny>> {
        Ok(value.into_pyobject(py)?.into_any().unbind())
    };
    let (lo, hi) = match refusal {
        R::SeedNotABracket { lo, hi }
        | R::SeedIsNotTheAnalyzedAxis {
            asked: (lo, hi), ..
        } => (Some(*lo), Some(*hi)),
        R::NotAContinuousParam { .. }
        | R::UnknownNode { .. }
        | R::UnknownSlot { .. }
        | R::StructuralSlot { .. }
        | R::SlotIsDefined { .. }
        | R::Derivation(_)
        | R::MoreThanOneAxisVaries { .. }
        | R::EvaluationOfAnotherDocument(_)
        | R::Drive(_) => (None, None),
    };
    let analyzed = match refusal {
        R::SeedIsNotTheAnalyzedAxis { analyzed, .. } => Some(*analyzed),
        R::SeedNotABracket { .. }
        | R::NotAContinuousParam { .. }
        | R::UnknownNode { .. }
        | R::UnknownSlot { .. }
        | R::StructuralSlot { .. }
        | R::SlotIsDefined { .. }
        | R::Derivation(_)
        | R::MoreThanOneAxisVaries { .. }
        | R::EvaluationOfAnotherDocument(_)
        | R::Drive(_) => None,
    };
    let param = match refusal {
        R::NotAContinuousParam { param } => text(&d::var_text(param)),
        R::SeedNotABracket { .. }
        | R::UnknownNode { .. }
        | R::UnknownSlot { .. }
        | R::StructuralSlot { .. }
        | R::SlotIsDefined { .. }
        | R::Derivation(_)
        | R::SeedIsNotTheAnalyzedAxis { .. }
        | R::MoreThanOneAxisVaries { .. }
        | R::EvaluationOfAnotherDocument(_)
        | R::Drive(_) => py.None(),
    };
    let node = match refusal {
        R::UnknownNode { node }
        | R::UnknownSlot { node, .. }
        | R::StructuralSlot { node, .. }
        | R::SlotIsDefined { node, .. } => Some(node.id()),
        R::Drive(drive) => match &**drive {
            a::DriveRefusal::WitnessDoesNotBuild { node, .. } => Some(node.id()),
            a::DriveRefusal::NothingVaries | a::DriveRefusal::Stopped(_) => None,
        },
        R::SeedNotABracket { .. }
        | R::NotAContinuousParam { .. }
        | R::Derivation(_)
        | R::SeedIsNotTheAnalyzedAxis { .. }
        | R::EvaluationOfAnotherDocument(_)
        | R::MoreThanOneAxisVaries { .. } => None,
    };
    let slot = match refusal {
        R::UnknownSlot { slot, .. }
        | R::StructuralSlot { slot, .. }
        | R::SlotIsDefined { slot, .. } => text(slot_id_tag(slot)),
        R::SeedNotABracket { .. }
        | R::NotAContinuousParam { .. }
        | R::UnknownNode { .. }
        | R::Derivation(_)
        | R::SeedIsNotTheAnalyzedAxis { .. }
        | R::MoreThanOneAxisVaries { .. }
        | R::EvaluationOfAnotherDocument(_)
        | R::Drive(_) => py.None(),
    };
    let cause = match refusal {
        R::Derivation(edit) => text(&edit.to_string()),
        R::Drive(drive) => match &**drive {
            a::DriveRefusal::WitnessDoesNotBuild { cause, .. } => text(&cause.to_string()),
            a::DriveRefusal::NothingVaries | a::DriveRefusal::Stopped(_) => py.None(),
        },
        R::SeedNotABracket { .. }
        | R::NotAContinuousParam { .. }
        | R::UnknownNode { .. }
        | R::UnknownSlot { .. }
        | R::StructuralSlot { .. }
        | R::SlotIsDefined { .. }
        | R::SeedIsNotTheAnalyzedAxis { .. }
        | R::EvaluationOfAnotherDocument(_)
        | R::MoreThanOneAxisVaries { .. } => py.None(),
    };
    let varying = match refusal {
        R::MoreThanOneAxisVaries { varying } => Some(*varying),
        R::SeedNotABracket { .. }
        | R::NotAContinuousParam { .. }
        | R::UnknownNode { .. }
        | R::UnknownSlot { .. }
        | R::StructuralSlot { .. }
        | R::SlotIsDefined { .. }
        | R::Derivation(_)
        | R::SeedIsNotTheAnalyzedAxis { .. }
        | R::EvaluationOfAnotherDocument(_)
        | R::Drive(_) => None,
    };
    let built = || -> PyResult<[(&'static str, Py<PyAny>); 9]> {
        Ok([
            ("variant", text(range_refusal_tag(refusal))),
            ("lo", object(lo)?),
            ("hi", object(hi)?),
            ("analyzed", analyzed.into_pyobject(py)?.into_any().unbind()),
            ("param", param),
            (
                "node",
                node.map(NodeId).into_pyobject(py)?.into_any().unbind(),
            ),
            ("slot", slot),
            ("cause", cause),
            ("varying", varying.into_pyobject(py)?.into_any().unbind()),
        ])
    };
    match built() {
        Ok(attrs) => typed_err(py, ErrorClass::Range, refusal.to_string(), &attrs),
        Err(failed) => failed,
    }
}

/// A float's bits with the two zeros read as one, so that values that
/// compare equal hash equal.
fn bits(x: f64) -> u64 {
    (x + 0.0).to_bits()
}

/// How a field names its variable or its slot, as Python wrote it.
#[derive(Clone, PartialEq, Eq, Hash)]
enum FieldSpec {
    /// A variable by identity.
    Var(d::VarId),
    /// A variable by the name the document holds for it.
    Name(d::VarName),
    /// A node's slot.
    Slot(d::RecipeNodeId, d::SlotId),
}

/// **The field a range is asked about**: one document parameter
/// (`RangeField.param`), or one slot of one node (`RangeField.slot`).
///
/// A slot is boxed through the free variable it reads, so the range is
/// that variable's — every slot reading it moves with it — reported
/// against the slot it was asked through.
///
/// A value, not a lookup: a parameter written by name is resolved in
/// the document each door is handed, and a name that document does not
/// hold refuses there (`RangeRefusal`, `not_a_continuous_param`).
#[pyclass(frozen, module = "pncad", from_py_object)]
#[derive(Clone, PartialEq, Eq, Hash)]
pub(crate) struct RangeField(FieldSpec);

impl RangeField {
    /// The kernel's field, read in `doc`.
    fn read(&self, doc: &d::ProfileDoc) -> Result<a::RangeField, a::RangeRefusal> {
        match &self.0 {
            FieldSpec::Var(var) => Ok(a::RangeField::Param(*var)),
            // The kernel addresses a parameter by identity and speaks
            // the one it does not hold from that identity; a name the
            // document does not hold has none, so the same refusal is
            // spoken here from the name.
            FieldSpec::Name(name) => doc
                .var_named(name.as_str())
                .map(a::RangeField::Param)
                .ok_or_else(|| a::RangeRefusal::NotAContinuousParam {
                    param: d::SpokenVar::new(d::VarId::new(0, 0), Some(name.clone())),
                }),
            FieldSpec::Slot(node, slot) => Ok(a::RangeField::Slot {
                node: *node,
                slot: *slot,
            }),
        }
    }
}

#[pymethods]
impl RangeField {
    /// A document parameter, by its `Var` or by its `VarName`.
    #[staticmethod]
    fn param(var: VarArg) -> Self {
        Self(match var {
            VarArg::Id(var) => FieldSpec::Var(var.0),
            VarArg::Name(name) => FieldSpec::Name(name.0),
        })
    }

    /// One slot of `node`, by the slot's own word (`"distance"`,
    /// `"radius"`) — the word `Doc.slot` and `DocEdit.set_param` take.
    #[staticmethod]
    fn slot(node: &NodeId, slot: &str) -> PyResult<Self> {
        Ok(Self(FieldSpec::Slot(
            node.0,
            super::doc::slot_from_text(slot)?,
        )))
    }

    fn __eq__(&self, other: &Self) -> bool {
        self == other
    }

    fn __hash__(&self) -> u64 {
        use std::hash::{Hash, Hasher};
        let mut h = std::hash::DefaultHasher::new();
        self.0.hash(&mut h);
        h.finish()
    }

    fn __repr__(&self) -> String {
        match &self.0 {
            FieldSpec::Var(var) => format!("RangeField.param(Var({}))", var.full()),
            FieldSpec::Name(name) => format!("RangeField.param({:?})", name.as_str()),
            FieldSpec::Slot(node, slot) => {
                format!(
                    "RangeField.slot(NodeId({}), {:?})",
                    node.full(),
                    slot_id_tag(slot)
                )
            }
        }
    }
}

/// **How far a search looks**, as OFFSETS from the field's nominal in
/// the field's own dimension: `lo <= 0 <= hi` with some width.
///
/// Constructing one checks nothing but that its two offsets share a
/// dimension. Whether it brackets the field's value is the admission
/// door's one question (`RangeRefusal`, `seed_not_a_bracket`), asked
/// where the seed meets a field.
#[pyclass(frozen, module = "pncad", from_py_object)]
#[derive(Clone, Copy)]
pub(crate) struct RangeSeed {
    inner: a::RangeSeed,
    dim: d::Dimension,
}

#[pymethods]
impl RangeSeed {
    #[new]
    fn new(py: Python<'_>, lo: &Bound<'_, PyAny>, hi: &Bound<'_, PyAny>) -> PyResult<Self> {
        let (lo, hi): (Offset, Offset) = (offset(lo)?, offset(hi)?);
        Ok(Self {
            inner: a::RangeSeed {
                lo: lo.canonical,
                hi: hi.canonical,
            },
            dim: agreed(py, "RangeSeed", &[lo, hi])?,
        })
    }

    /// The symmetric seed `[-w, w]`.
    #[staticmethod]
    fn symmetric(w: &Bound<'_, PyAny>) -> PyResult<Self> {
        let w = offset(w)?;
        Ok(Self {
            inner: a::RangeSeed::symmetric(w.canonical),
            dim: w.dim,
        })
    }

    /// **The seed the kernel chooses for `field`**: from zero to twice
    /// its current value, and `±1` for a field at zero. What every
    /// door here searches when it is handed no seed.
    ///
    /// Raises `RangeRefusal` for a field with no axis.
    #[staticmethod]
    fn of(py: Python<'_>, doc: &super::doc::Doc, field: &RangeField) -> PyResult<Self> {
        admitted(py, &doc.inner, field, None, None).map(|request| Self {
            inner: request.seed,
            dim: request.dim,
        })
    }

    /// The lower offset.
    #[getter]
    fn lo(&self, py: Python<'_>) -> PyResult<Py<PyAny>> {
        quantity(py, self.inner.lo, self.dim)
    }

    /// The upper offset.
    #[getter]
    fn hi(&self, py: Python<'_>) -> PyResult<Py<PyAny>> {
        quantity(py, self.inner.hi, self.dim)
    }

    /// The dimension the offsets are written in.
    #[getter]
    fn dimension(&self) -> &'static str {
        dimension_tag(self.dim)
    }

    fn __eq__(&self, other: &Self) -> bool {
        self.inner == other.inner && self.dim == other.dim
    }

    fn __hash__(&self) -> u64 {
        use std::hash::{Hash, Hasher};
        let mut h = std::hash::DefaultHasher::new();
        (bits(self.inner.lo), bits(self.inner.hi), self.dim).hash(&mut h);
        h.finish()
    }

    fn __repr__(&self) -> String {
        format!(
            "RangeSeed({:?} [{}, {}])",
            self.dim, self.inner.lo, self.inner.hi
        )
    }
}

/// A request the admission door admitted: the kernel's field and seed,
/// and what it resolved them to.
struct Request {
    field: a::RangeField,
    seed: a::RangeSeed,
    admission: a::Admission,
    /// The dimension of the variable the field is boxed through, which
    /// every offset of the answer is written in.
    dim: d::Dimension,
}

/// Admit `field` over `seed` in `doc` — the kernel's own seed where
/// none is handed — and check a handed seed is written in the field's
/// dimension.
fn admitted(
    py: Python<'_>,
    doc: &d::ProfileDoc,
    field: &RangeField,
    seed: Option<&RangeSeed>,
    built: Option<&d::Evaluation<f64>>,
) -> PyResult<Request> {
    let refused = |refusal: a::RangeRefusal| range_err(py, &refusal);
    let kernel_field = field.read(doc).map_err(refused)?;
    let kernel_seed = match seed {
        Some(seed) => seed.inner,
        None => a::RangeSeed::of(doc, &kernel_field).map_err(refused)?,
    };
    let admission = a::range::admit(doc, &kernel_field, kernel_seed, built).map_err(refused)?;
    let Some(d::FreeVar::Continuous { dim, .. }) = doc.free(admission.axis) else {
        unreachable!("an admitted field is boxed through a continuous free variable")
    };
    if let Some(seed) = seed
        && seed.dim != *dim
    {
        return Err(dimension_mismatch(py, "RangeSeed", *dim, seed.dim));
    }
    Ok(Request {
        field: kernel_field,
        seed: kernel_seed,
        admission,
        dim: *dim,
    })
}

/// **What the admission door resolved a request to**: the free
/// variable the field is boxed through, and its value.
#[pyclass(frozen, module = "pncad", skip_from_py_object)]
#[derive(Clone, Copy)]
pub(crate) struct Admission {
    inner: a::Admission,
    axis: Var,
    dim: d::Dimension,
}

#[pymethods]
impl Admission {
    /// The free variable the field is boxed through: the parameter
    /// itself, or the variable the slot reads.
    #[getter]
    fn axis(&self) -> Var {
        self.axis
    }

    /// The field's value in the document.
    #[getter]
    fn nominal(&self, py: Python<'_>) -> PyResult<Py<PyAny>> {
        quantity(py, self.inner.nominal, self.dim)
    }

    fn __eq__(&self, other: &Self) -> bool {
        self.inner == other.inner
    }

    fn __hash__(&self) -> u64 {
        use std::hash::{Hash, Hasher};
        let mut h = std::hash::DefaultHasher::new();
        (self.inner.axis, bits(self.inner.nominal)).hash(&mut h);
        h.finish()
    }

    fn __repr__(&self) -> String {
        format!(
            "Admission(Var({}) {:?} {})",
            self.inner.axis.full(),
            self.dim,
            self.inner.nominal
        )
    }
}

/// **Whether a range can be searched at all**, decided with no
/// geometry run — what a caller asks before offering the query.
///
/// `seed` is the kernel's own (`RangeSeed.of`) where none is handed.
/// `built` is the caller's finished evaluation of this same document,
/// where it holds one: a node that did not build there refuses as the
/// witness build would (`witness_does_not_build`) without building it
/// again. An evaluation of another document is a `ValueError`.
///
/// Raises `RangeRefusal`: a seed that does not bracket the field's
/// value, then a field with no axis — a variable the document does not
/// declare continuous, a node or slot it does not carry, a structural
/// slot, a slot reading a defined variable.
#[pyfunction]
#[pyo3(signature = (doc, field, seed = None, *, built = None))]
fn admit_range(
    py: Python<'_>,
    doc: &super::doc::Doc,
    field: &RangeField,
    seed: Option<&RangeSeed>,
    built: Option<&super::value::Evaluation>,
) -> PyResult<Admission> {
    if let Some(built) = built
        && *built.doc() != doc.inner
    {
        return Err(pyo3::exceptions::PyValueError::new_err(
            "`built` is an evaluation of another document: a range is admitted against the \
             build of the document it is asked about",
        ));
    }
    let request = admitted(py, &doc.inner, field, seed, built.map(|b| &b.inner))?;
    Ok(Admission {
        inner: request.admission,
        axis: Var::of(&doc.inner, request.admission.axis),
        dim: request.dim,
    })
}

/// **A point at which the document is proven NOT the nominal's
/// build**: the near edge of the nearest interval the replay decided
/// definitely otherwise.
#[pyclass(frozen, module = "pncad", skip_from_py_object)]
#[derive(Clone)]
pub(crate) struct ProvenDifferent {
    inner: a::ProvenDifferent,
    dim: d::Dimension,
}

#[pymethods]
impl ProvenDifferent {
    /// The point, as an offset from the field's nominal.
    #[getter]
    fn at(&self, py: Python<'_>) -> PyResult<Py<PyAny>> {
        quantity(py, self.inner.at(), self.dim)
    }

    /// What the difference is: `"standing_change"` — a node that built
    /// does not, or the reverse — or `"builds_differently"`: every node
    /// stands as it did and a recorded decision goes the other way,
    /// which is the boundary of the CERTIFICATE and not of validity.
    #[getter]
    fn kind(&self) -> &'static str {
        difference_kind_tag(&self.inner.kind())
    }

    /// The first node whose standing changed, in id order; `None` for
    /// `"builds_differently"`.
    #[getter]
    fn node(&self) -> Option<NodeId> {
        match self.inner.kind() {
            a::DifferenceKind::StandingChange { node } => Some(NodeId(node)),
            a::DifferenceKind::BuildsDifferently => None,
        }
    }

    fn __repr__(&self) -> String {
        format!(
            "ProvenDifferent({} at {:?} {})",
            self.kind(),
            self.dim,
            self.inner.at()
        )
    }
}

/// **Ground the search did not decide**, outward from `past`.
///
/// Not a bound, and never to be shown as one: nothing here says the
/// document is valid past `past`, or that it is not.
#[pyclass(frozen, module = "pncad", skip_from_py_object)]
#[derive(Clone)]
pub(crate) struct NotDecided {
    inner: a::NotDecided,
    dim: d::Dimension,
}

#[pymethods]
impl NotDecided {
    /// The offset the undecided ground starts at: the proven frontier.
    #[getter]
    fn past(&self, py: Python<'_>) -> PyResult<Py<PyAny>> {
        quantity(py, self.inner.past(), self.dim)
    }

    /// Why the side is undecided at its frontier: `"unfinished"` (the
    /// search still had work here when it stopped), `"stalled"`
    /// (halvings past the proof's last move decided nothing),
    /// `"flip_unfounded"` (the replay refused as a flip that names
    /// nothing, or at the current value: never a difference), or the
    /// driver's own reason for the frontier interval: `"budget"`,
    /// `"sliver_terminal"`, `"reads_no_variable"` (the deciding node
    /// does not read the field, so no halving settles it),
    /// `"bifurcation"` or `"infeasible"`.
    #[getter]
    fn reason(&self) -> &'static str {
        unproven_tag(self.inner.reason())
    }

    /// The replays the search had taken, for `"unfinished"`: budget
    /// spent, not a cap. `None` for another reason.
    #[getter]
    fn replays(&self) -> Option<usize> {
        match self.inner.reason() {
            a::Unproven::Unfinished { replays } => Some(*replays),
            a::Unproven::Stalled { .. } | a::Unproven::Floor(_) | a::Unproven::FlipUnfounded => {
                None
            }
        }
    }

    /// The width of the interval that last moved the proof, for
    /// `"stalled"`, in the field's dimension. `None` for another reason.
    #[getter]
    fn width(&self, py: Python<'_>) -> PyResult<Option<Py<PyAny>>> {
        match self.inner.reason() {
            a::Unproven::Stalled { width } => quantity(py, *width, self.dim).map(Some),
            a::Unproven::Unfinished { .. } | a::Unproven::Floor(_) | a::Unproven::FlipUnfounded => {
                Ok(None)
            }
        }
    }

    /// Which bound a `"budget"` ran out of — `"depth"` (the halvings one
    /// side may take) or `"resolution"` (the `f64` grid) — and `None`
    /// for another reason.
    #[getter]
    fn budget(&self) -> Option<&'static str> {
        match self.inner.reason() {
            a::Unproven::Floor(a::RefusalReason::Budget(kind)) => Some(budget_kind_tag(kind)),
            a::Unproven::Floor(
                a::RefusalReason::SliverTerminal { .. }
                | a::RefusalReason::FlipCrossing { .. }
                | a::RefusalReason::Bifurcation(_)
                | a::RefusalReason::ReadsNoVariable { .. }
                | a::RefusalReason::Infeasible,
            )
            | a::Unproven::Unfinished { .. }
            | a::Unproven::Stalled { .. }
            | a::Unproven::FlipUnfounded => None,
        }
    }

    /// The predicate a `"sliver_terminal"` could not decide, and `None`
    /// for another reason.
    #[getter]
    fn predicate(&self) -> Option<&'static str> {
        match self.inner.reason() {
            a::Unproven::Floor(a::RefusalReason::SliverTerminal { predicate }) => Some(predicate),
            a::Unproven::Floor(
                a::RefusalReason::FlipCrossing { .. }
                | a::RefusalReason::Bifurcation(_)
                | a::RefusalReason::Infeasible
                | a::RefusalReason::ReadsNoVariable { .. }
                | a::RefusalReason::Budget(_),
            )
            | a::Unproven::Unfinished { .. }
            | a::Unproven::Stalled { .. }
            | a::Unproven::FlipUnfounded => None,
        }
    }

    /// Whether more replays can change this side's answer: only
    /// `"unfinished"` answers yes. A stall and a floor are where the
    /// schedule stops; a larger `max_depth` can move a `"depth"`
    /// floor, and more replays cannot.
    #[getter]
    fn more_time_helps(&self) -> bool {
        self.inner.more_time_helps()
    }

    fn __repr__(&self) -> String {
        format!(
            "NotDecided({}{} past {:?} {})",
            self.reason(),
            self.budget().map_or_else(String::new, |b| format!(":{b}")),
            self.dim,
            self.inner.past()
        )
    }
}

/// **One side's answer.**
///
/// `proven_to` is the far end of the proven interval on this side.
/// Between it and a point proven `different` lies ground the search did
/// NOT decide (`not_decided`), which is never valid ground by default.
///
/// `reached_seed_end` and `different` say how the side ends, and both
/// are empty exactly when the side is undecided with nothing proven
/// different beyond; `not_decided` is `None` exactly when the proven
/// interval reaches the seed's end, or runs up to the point proven
/// different.
#[pyclass(frozen, module = "pncad", skip_from_py_object)]
#[derive(Clone)]
pub(crate) struct RangeSide {
    inner: a::RangeSide,
    dim: d::Dimension,
}

#[pymethods]
impl RangeSide {
    /// The far end of the proven interval on this side, as an offset:
    /// zero when nothing is proven.
    #[getter]
    fn proven_to(&self, py: Python<'_>) -> PyResult<Py<PyAny>> {
        quantity(py, self.inner.proven_to(), self.dim)
    }

    /// Whether everything from the current value to the seed's end is
    /// proven. Never a claim about values outside the seed, which were
    /// not replayed.
    #[getter]
    fn reached_seed_end(&self) -> bool {
        self.inner.reached_seed_end()
    }

    /// The nearest point proven different, where there is one.
    #[getter]
    fn different(&self) -> Option<ProvenDifferent> {
        self.inner.different().map(|inner| ProvenDifferent {
            inner: inner.clone(),
            dim: self.dim,
        })
    }

    /// The undecided ground at the proven frontier, where there is any.
    #[getter]
    fn not_decided(&self) -> Option<NotDecided> {
        self.inner.not_decided().map(|inner| NotDecided {
            inner: inner.clone(),
            dim: self.dim,
        })
    }

    fn __repr__(&self) -> String {
        let end = match self.inner.end() {
            Some(a::SideEnd::Seed) => "the seed's end",
            Some(a::SideEnd::Different(_)) => "a point proven different",
            None => "no end decided",
        };
        format!(
            "RangeSide(proven to {:?} {}, {end}{})",
            self.dim,
            self.inner.proven_to(),
            if self.inner.not_decided().is_some() {
                ", undecided at the frontier"
            } else {
                ""
            }
        )
    }
}

/// **The search's answer**: a proven interval around the field's
/// current value, and one `RangeSide` per direction.
///
/// It is a CONDITIONAL answer and carries its condition: `pinned` are
/// the parameters held at their nominals for the whole certificate.
#[pyclass(frozen, module = "pncad", skip_from_py_object)]
#[derive(Clone)]
pub(crate) struct CertifiedRange {
    inner: a::CertifiedRange,
    dim: d::Dimension,
    field: RangeField,
    /// [`a::CertifiedRange::pinned`], read in the document the search
    /// was asked about.
    pinned: Vec<Var>,
}

#[pymethods]
impl CertifiedRange {
    /// The field this range is about, as it was asked.
    #[getter]
    fn field(&self) -> RangeField {
        self.field.clone()
    }

    /// The field's value in the document: the origin every offset here
    /// is measured from.
    #[getter]
    fn nominal(&self, py: Python<'_>) -> PyResult<Py<PyAny>> {
        quantity(py, self.inner.nominal(), self.dim)
    }

    /// How far the search looked.
    #[getter]
    fn seed(&self) -> RangeSeed {
        RangeSeed {
            inner: self.inner.seed(),
            dim: self.dim,
        }
    }

    /// **The condition this answer holds under**: the parameters whose
    /// declared distribution the search cleared, in declaration order.
    /// Every one of them is at its nominal for the whole certificate;
    /// empty means the document declared no other spread.
    ///
    /// By identity, because a parameter carrying a spread need not be
    /// named and the condition names every one; `Doc.var_name` speaks
    /// a handle.
    #[getter]
    fn pinned(&self) -> Vec<Var> {
        self.pinned.clone()
    }

    /// How many replays this answer is the answer after.
    #[getter]
    fn steps(&self) -> usize {
        self.inner.steps()
    }

    /// The downward side.
    #[getter]
    fn lo(&self) -> RangeSide {
        RangeSide {
            inner: self.inner.lo().clone(),
            dim: self.dim,
        }
    }

    /// The upward side.
    #[getter]
    fn hi(&self) -> RangeSide {
        RangeSide {
            inner: self.inner.hi().clone(),
            dim: self.dim,
        }
    }

    /// The proven interval as offsets, `(lo.proven_to, hi.proven_to)`:
    /// every value in it builds exactly what the nominal built, with
    /// `pinned` at their nominals. Both zero when nothing is proven.
    #[getter]
    fn proven_offsets(&self, py: Python<'_>) -> PyResult<(Py<PyAny>, Py<PyAny>)> {
        let (lo, hi) = self.inner.proven_offsets();
        Ok((quantity(py, lo, self.dim)?, quantity(py, hi, self.dim)?))
    }

    /// The proven interval in ABSOLUTE field values. A reader's
    /// convenience: the proof is over the offsets, and this sum rounds.
    #[getter]
    fn proven_interval(&self, py: Python<'_>) -> PyResult<(Py<PyAny>, Py<PyAny>)> {
        let (lo, hi) = self.inner.proven_interval();
        Ok((quantity(py, lo, self.dim)?, quantity(py, hi, self.dim)?))
    }

    /// Whether neither side proved anything around the current value.
    #[getter]
    fn nothing_proven(&self) -> bool {
        self.inner.nothing_proven()
    }

    fn __eq__(&self, other: &Self) -> bool {
        self.inner == other.inner && self.dim == other.dim && self.field == other.field
    }

    fn __hash__(&self) -> u64 {
        use std::hash::{Hash, Hasher};
        let (lo, hi) = self.inner.proven_offsets();
        let mut h = std::hash::DefaultHasher::new();
        (
            &self.field,
            bits(self.inner.nominal()),
            self.inner.steps(),
            bits(lo),
            bits(hi),
        )
            .hash(&mut h);
        h.finish()
    }

    fn __repr__(&self) -> String {
        let (lo, hi) = self.inner.proven_offsets();
        format!(
            "CertifiedRange({:?} proven [{lo}, {hi}] around {} after {} replays, {} pinned)",
            self.dim,
            self.inner.nominal(),
            self.inner.steps(),
            self.pinned.len()
        )
    }
}

/// The search's budget for an admitted seed: the depth that seed needs
/// (`RangeBudget::for_seed`) unless one is stated.
fn budget(seed: a::RangeSeed, max_depth: Option<u32>, max_replays: usize) -> a::RangeBudget {
    let sized = a::RangeBudget::for_seed(seed, Tol::witness(), max_replays);
    a::RangeBudget {
        max_depth: max_depth.unwrap_or(sized.max_depth),
        ..sized
    }
}

/// A stop for one call: the token's, or one nothing cancels.
fn stop_of(cancel: Option<&super::value::CancelToken>) -> d::Stop {
    cancel.map_or_else(d::Stop::default, |c| d::Stop::canceled_by(&c.token()))
}

/// **The stepped search.** Constructing one admits the request and
/// builds the document once, at its nominal; `step` replays ONE
/// interval; `answer` can be read after any step, and is a function of
/// the request and the number of steps taken — no clock is read, so
/// stopping keeps what was proven, and a later `step` continues it.
///
/// The schedule refines toward the current value, works only where the
/// answer can still move, and takes the two sides level by level
/// together (`pncad::analysis::range`'s module docs).
///
/// `seed` is the kernel's own (`RangeSeed.of`) where none is handed.
/// `max_depth` is how many times one side may be halved; it defaults to
/// what the seed needs to reach a width a curved field certifies at
/// (`RangeBudget::for_seed`). `max_replays` bounds how many intervals
/// the search will ever replay, and defaults to no bound: the loop
/// calling `step` is the budget. `resolver` is how a document that
/// instantiates parts reaches them, as `evaluate` takes it. `cancel`
/// stops the nominal build; each `step` takes its own.
///
/// Raises `RangeRefusal`: the request's own refusals before any
/// geometry runs, then a nominal that does not build
/// (`witness_does_not_build`) or was stopped (`stopped`).
#[pyclass(frozen, module = "pncad")]
pub(crate) struct RangeSearch {
    /// Behind a lock so that a step runs with the interpreter released
    /// and two threads stepping one search take turns.
    inner: Mutex<a::RangeSearch>,
    dim: d::Dimension,
    field: RangeField,
    /// The document the search was asked about, which the answer's
    /// pinned parameters are read in.
    doc: d::ProfileDoc,
}

impl RangeSearch {
    #[allow(clippy::too_many_arguments)]
    fn open(
        py: Python<'_>,
        doc: &super::doc::Doc,
        field: &RangeField,
        seed: Option<&RangeSeed>,
        max_depth: Option<u32>,
        max_replays: usize,
        resolver: Option<&super::store::Workspace>,
        stop: &d::Stop,
    ) -> PyResult<Self> {
        let recipe = &doc.inner;
        let request = admitted(py, recipe, field, seed, None)?;
        let config = budget(request.seed, max_depth, max_replays);
        let resolver = super::doc::seam(resolver);
        let tol = Tol::witness();
        // The nominal build is a whole document evaluation, so the
        // interpreter runs while it is paid.
        let opened = py.detach(|| {
            a::RangeSearch::new(
                recipe,
                &request.field,
                request.seed,
                &config,
                resolver,
                stop,
                tol,
            )
        });
        match opened {
            Ok(inner) => Ok(Self {
                inner: Mutex::new(inner),
                dim: request.dim,
                field: field.clone(),
                doc: recipe.clone(),
            }),
            Err(refusal) => Err(range_err(py, &refusal)),
        }
    }

    /// One replay, with the interpreter released.
    fn advance(&self, py: Python<'_>, stop: &d::Stop) -> a::range::Step {
        py.detach(|| {
            self.inner
                .lock()
                .unwrap_or_else(PoisonError::into_inner)
                .step(stop)
        })
    }

    fn read(&self) -> CertifiedRange {
        let inner = self
            .inner
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .answer();
        CertifiedRange {
            pinned: inner
                .pinned()
                .iter()
                .map(|&var| Var::of(&self.doc, var))
                .collect(),
            inner,
            dim: self.dim,
            field: self.field.clone(),
        }
    }
}

#[pymethods]
impl RangeSearch {
    #[new]
    #[pyo3(signature = (
        doc, field, seed = None, *, max_depth = None, max_replays = None, resolver = None,
        cancel = None
    ))]
    #[allow(clippy::too_many_arguments)]
    fn new(
        py: Python<'_>,
        doc: &super::doc::Doc,
        field: &RangeField,
        seed: Option<&RangeSeed>,
        max_depth: Option<u32>,
        max_replays: Option<usize>,
        resolver: Option<&super::store::Workspace>,
        cancel: Option<&super::value::CancelToken>,
    ) -> PyResult<Self> {
        Self::open(
            py,
            doc,
            field,
            seed,
            max_depth,
            max_replays.unwrap_or(usize::MAX),
            resolver,
            &stop_of(cancel),
        )
    }

    /// **One replay.** Answers `"advanced"` when an interval was
    /// replayed, `"finished"` when nothing was — the search has no work
    /// left that could change its answer, or `max_replays` is spent —
    /// and `"stopped"` when the replay stopped short (`cancel`, or the
    /// kernel's member budget) and decided nothing: the search stands
    /// where it stood, and a later `step` continues it.
    #[pyo3(signature = (*, cancel = None))]
    fn step(&self, py: Python<'_>, cancel: Option<&super::value::CancelToken>) -> &'static str {
        range_step_tag(&self.advance(py, &stop_of(cancel)))
    }

    /// How many intervals have been replayed.
    #[getter]
    fn steps(&self) -> usize {
        self.inner
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .steps()
    }

    /// The answer after the steps taken so far.
    fn answer(&self) -> CertifiedRange {
        self.read()
    }

    fn __repr__(&self) -> String {
        format!(
            "RangeSearch({}, steps={})",
            self.field.__repr__(),
            self.steps()
        )
    }
}

/// **The certified range of one field**: a `RangeSearch` stepped until
/// it finishes, `max_replays` is spent, or `cancel` stops it.
///
/// `max_replays` is required: a replay is a document evaluation at the
/// certified scalar, and a field that certifies only near the
/// tolerance never finishes a wide seed, so no count this door could
/// choose is one a caller should wait for. `max_depth` defaults as on
/// `RangeSearch`. A stop after the nominal build keeps what was proven
/// and is not a refusal — the answer's `not_decided` says more time
/// helps.
///
/// Raises `RangeRefusal` as `RangeSearch` does.
#[pyfunction]
#[pyo3(signature = (
    doc, field, seed = None, *, max_replays, max_depth = None, resolver = None, cancel = None
))]
#[allow(clippy::too_many_arguments)]
fn certified_range(
    py: Python<'_>,
    doc: &super::doc::Doc,
    field: &RangeField,
    seed: Option<&RangeSeed>,
    max_replays: usize,
    max_depth: Option<u32>,
    resolver: Option<&super::store::Workspace>,
    cancel: Option<&super::value::CancelToken>,
) -> PyResult<CertifiedRange> {
    let stop = stop_of(cancel);
    let search = RangeSearch::open(
        py,
        doc,
        field,
        seed,
        max_depth,
        max_replays,
        resolver,
        &stop,
    )?;
    while search.advance(py, &stop) == a::range::Step::Advanced {}
    Ok(search.read())
}

/// Register the range vocabulary on the module.
pub(crate) fn register(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<RangeField>()?;
    m.add_class::<RangeSeed>()?;
    m.add_class::<Admission>()?;
    m.add_class::<ProvenDifferent>()?;
    m.add_class::<NotDecided>()?;
    m.add_class::<RangeSide>()?;
    m.add_class::<CertifiedRange>()?;
    m.add_class::<RangeSearch>()?;
    m.add_function(wrap_pyfunction!(admit_range, m)?)?;
    m.add_function(wrap_pyfunction!(certified_range, m)?)?;
    Ok(())
}
