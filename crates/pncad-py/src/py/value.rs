//! Per-node evaluation results: `Evaluation`, `Value`, `Body`.
//!
//! # The ValuePayload exposure inventory (a reported FORK)
//!
//! The bindings project `ValuePayload`'s geometry variants as:
//!
//! | variant        | exposure |
//! |----------------|----------|
//! | `Body`         | full — opaque handle + `mass_properties` / `validate` doors, and `validate_geometric_measured`, the pair that pays once |
//! | `Boolean`      | full — unwraps to a `Body`, or `None` when empty |
//! | `Split`        | full — `above` / `below` as optional bodies |
//! | `Instances`    | full — a list of bodies |
//! | `Profile`      | KIND ONLY — sketch geometry does not ship to Python before the v2 switch |
//!
//! The kind-only row is a SCOPE decision, not a capability limit.
//! `ValidatedProfile::plane()`/`loops()` DO exist and `profile` is
//! wholesale re-exported — this very module's sibling uses
//! `pncad::profile` to build sketches. Projecting a profile back to
//! Python is therefore perfectly possible; it is **ruled out**: Python
//! never ships the opaque-profile intermediate state. Sketch read-back
//! belongs with the v2 program representation.

use std::sync::Arc;

use pyo3::prelude::*;
use pyo3::types::PyString;

use crate::errors::{ErrorClass, EvalReason, ValidationRefusal, measurement_dimension_tag};
use crate::py::quantity::Length;
use crate::py::{
    doc::{NodeId, Var},
    typed_err,
};
use crate::tags::{
    export_error_tag, node_error_tag, node_inner_kind_tag, normalization_kind_tag,
    promoted_curve_kind_tag, promoted_kind_tag, singularity_found_tag, step_import_error_tag,
};
use crate::validation;
use pncad::document as d;
use pncad::tolerance::Tol;
use pncad::topo;

/// The refusing ARM's word, as the exception carries it: the kernel
/// refusal's own discriminant beside the carrier's, `None` where the
/// refusal has no arms.
fn inner_kind(py: Python<'_>, kind: &d::NodeErrorKind) -> Py<PyAny> {
    match node_inner_kind_tag(kind) {
        Some(tag) => PyString::new(py, tag).unbind().into_any(),
        None => py.None().into_any(),
    }
}

/// Raise `EvaluationError` with a stable `reason` tag.
///
/// The reason is an [`EvalReason`], not a `&str`, and it rides on the
/// CLASS ([`ErrorClass::Evaluation`]) rather than in the payload: the
/// word a Python caller branches on is minted once, by
/// [`crate::tags::eval_reason_tag`], on a page the tag-table guard
/// reads. That holds for this door's other two raises below and for
/// any future one, because no raise of this class can be written
/// without naming a variant of the enum.
///
/// `kind`, `inner_kind`, `through` and `document` are ALWAYS present on
/// the exception — `None` where the reason has no failing kind, no arm
/// under that kind, no poisoning ancestor, or the node is the evaluated
/// document's own — so
/// stub-guided code can read them without an `AttributeError` trap — a
/// stub that over-promises is worse than one that says `None`.
fn eval_err(py: Python<'_>, message: impl Into<String>, reason: EvalReason, node: NodeId) -> PyErr {
    let node = match node.into_pyobject(py) {
        Ok(bound) => bound.unbind().into_any(),
        // A `#[pyclass]` conversion fails as a `PyErr` already —
        // surface it as the raise rather than losing it.
        Err(failed) => return failed,
    };
    typed_err(
        py,
        ErrorClass::Evaluation(reason),
        message,
        &[
            ("node", node),
            ("kind", py.None().into_any()),
            ("inner_kind", py.None().into_any()),
            ("through", py.None().into_any()),
            ("document", py.None().into_any()),
        ],
    )
}

/// Raise `EvaluationError` for a node that ITSELF failed: the payload
/// is the `NodeErrorKind`'s stable tag plus the node id; the message
/// is the kernel error's own `Display` prose — never a `Debug` dump.
///
/// `kind` is the CARRIER's word — which door refused — and `inner_kind`
/// is the arm of the kernel refusal that door holds, `None` where that
/// refusal has no arms. Two enums, two discriminants, each projected
/// where it lives.
///
/// A refusal that CARRIES another node's (a part whose product reads a
/// node that failed or was poisoned, a mate whose placer's row cannot state its refusal)
/// never quotes it in its message; the carried refusal crosses typed, as this exception's
/// `__cause__` ([`with_carried`]).
///
/// The message speaks the node from `doc`, the document the evaluation
/// is OF ([`Evaluation`]'s captured `doc`), so a label set after
/// `evaluate` shows on the next evaluation, and each name within its
/// table in `evaluation`, the one that raised it; `node` crosses as
/// the id.
fn node_failure(
    py: Python<'_>,
    doc: &d::ProfileDoc,
    evaluation: &d::Evaluation<f64>,
    node: NodeId,
    error: &d::NodeError,
) -> PyErr {
    let err = refused(py, node, &error.kind, error.spoken(doc, evaluation), None);
    with_carried(py, err, error.kind.carried_chain(), Some(doc))
}

/// **`node`'s standing in `evaluation` raised**, with the REAL typed
/// cause: a failed node's own error, a poisoned one's standing followed
/// by its nearest failed ancestor's, and a node with no entry as its
/// standing alone. Asked only of a node with no value.
pub(crate) fn standing_err(
    py: Python<'_>,
    doc: &d::ProfileDoc,
    evaluation: &d::Evaluation<f64>,
    node: NodeId,
) -> PyErr {
    match evaluation.usable(node.0) {
        Ok(_) => unreachable!("a standing is asked of a node with no value"),
        Err(standing) => standing_raise(py, doc, evaluation, node, standing),
    }
}

/// [`standing_err`] over a standing already read.
fn standing_raise(
    py: Python<'_>,
    doc: &d::ProfileDoc,
    evaluation: &d::Evaluation<f64>,
    node: NodeId,
    standing: d::NodeStanding,
) -> PyErr {
    // `node_error` answers a failed node's own error and a poisoned
    // one's nearest failed ancestor's.
    let root = evaluation.node_error(node.0);
    match (standing, root) {
        (d::NodeStanding::Failed { .. }, Some(error)) => {
            node_failure(py, doc, evaluation, node, error)
        }
        (d::NodeStanding::Poisoned { node, through }, root) => {
            poisoning(py, doc, evaluation, node, through, root)
        }
        (
            d::NodeStanding::Failed { .. }
            | d::NodeStanding::NotEvaluated { .. }
            | d::NodeStanding::NotInDocument { .. },
            _,
        ) => eval_err(
            py,
            standing.spoken(doc),
            EvalReason::Standing(standing),
            node,
        ),
    }
}

/// [`node_failure`]'s one exception, over a kind, its rendering and the
/// document its node is in (`None` for the evaluated document's own),
/// with no cause: what one level of a carried chain is.
pub(crate) fn refused(
    py: Python<'_>,
    node: NodeId,
    kind: &d::NodeErrorKind,
    message: String,
    document: Option<&d::DocRef>,
) -> PyErr {
    let node_obj = match node.into_pyobject(py) {
        Ok(bound) => bound.unbind().into_any(),
        Err(failed) => return failed,
    };
    let document = match document {
        Some(doc_ref) => match super::store::DocRef(*doc_ref).into_pyobject(py) {
            Ok(bound) => bound.unbind().into_any(),
            Err(failed) => return failed,
        },
        None => py.None().into_any(),
    };
    typed_err(
        py,
        ErrorClass::Evaluation(EvalReason::Standing(d::NodeStanding::Failed {
            node: node.0,
        })),
        message,
        &[
            ("node", node_obj),
            (
                "kind",
                PyString::new(py, node_error_tag(kind.class()))
                    .unbind()
                    .into_any(),
            ),
            ("inner_kind", inner_kind(py, kind)),
            ("through", py.None().into_any()),
            ("document", document),
        ],
    )
}

/// **`chain` as `err`'s `__cause__`** ([`carried_cause`]).
pub(crate) fn with_carried(
    py: Python<'_>,
    err: PyErr,
    chain: d::CarriedChain<'_>,
    here: Option<&d::ProfileDoc>,
) -> PyErr {
    if let Some(cause) = carried_cause(py, chain, here) {
        err.set_cause(py, Some(cause));
    }
    err
}

/// The most `EvaluationError`s a carried chain links as `__cause__`s.
///
/// CPython 3.11's default excepthook prints a cause chain by recursing
/// once per link and gives up at the recursion limit (1000 by default):
/// it prints `lost sys.stderr` and no traceback at all, from 999
/// linked exceptions up. A part refusal carries up to 1024 levels (the
/// kernel's nesting bound), so the binding links at most this many and
/// folds the rest into the last one (see [`carried_cause`]), leaving
/// the recursion limit ample room however deep the frame that prints.
pub(crate) const LINKED_LEVELS: usize = 256;

/// **A carried chain, typed**: one `EvaluationError` per level of the
/// kernel's chain ([`d::NodeErrorKind::carried_chain`]), each raised
/// for its node as [`node_failure`] raises one and each the `__cause__`
/// of the level above, so a part inside a part is a chain of causes,
/// one per document. A level's `node` is in the id space of its
/// `document`, the part's `DocRef`, or `None` for the evaluated
/// document's own. `None` for a chain with no level.
///
/// A chain deeper than [`LINKED_LEVELS`] links that many: the last is
/// raised for the deepest level, the node that refused, and its message
/// is every level it stands for, one line each, deepest first — the
/// order CPython prints a cause chain in. So the printed traceback
/// still has one line per document level.
pub(crate) fn carried_cause(
    py: Python<'_>,
    chain: d::CarriedChain<'_>,
    here: Option<&d::ProfileDoc>,
) -> Option<PyErr> {
    let levels: Vec<d::CarriedLevel<'_>> = chain.collect();
    let line = |level: &d::CarriedLevel<'_>| match here {
        Some(doc) => level.line_in(doc),
        None => level.line(),
    };
    let (linked, folded) = levels.split_at(levels.len().min(LINKED_LEVELS).saturating_sub(1));
    let raise = |level: &d::CarriedLevel<'_>, message: String| {
        refused(
            py,
            NodeId(level.node),
            level.refusal.kind(),
            message,
            level.document.doc_ref(),
        )
    };
    let deepest = folded.last()?;
    let lines: Vec<String> = folded.iter().rev().map(line).collect();
    let innermost = raise(deepest, lines.join("\n"));
    Some(linked.iter().rev().fold(innermost, |inner, level| {
        let err = raise(level, line(level));
        err.set_cause(py, Some(inner));
        err
    }))
}

/// Raise `EvaluationError` for a POISONED node: `through` names the
/// nearest failed ancestor, `kind` tags its root cause (present
/// whenever the evaluation's own invariant holds — fail-honest, so a
/// broken hop yields no `kind` rather than a wrong one). A root cause
/// that carries a refusal hands it on as `__cause__`, as
/// [`node_failure`] does.
fn poisoning(
    py: Python<'_>,
    doc: &d::ProfileDoc,
    evaluation: &d::Evaluation<f64>,
    node: d::RecipeNodeId,
    through: d::RecipeNodeId,
    root: Option<&d::NodeError>,
) -> PyErr {
    let standing = d::NodeStanding::Poisoned { node, through };
    let objs = (
        NodeId(node).into_pyobject(py),
        NodeId(through).into_pyobject(py),
    );
    let (node_obj, through_obj) = match objs {
        (Ok(n), Ok(t)) => (n.unbind().into_any(), t.unbind().into_any()),
        (Err(failed), _) | (_, Err(failed)) => return failed,
    };
    let mut fields: Vec<(&str, Py<PyAny>)> = vec![
        ("node", node_obj),
        ("through", through_obj),
        ("document", py.None().into_any()),
    ];
    // The node never ran, so the standing's sentence is followed by
    // the ancestor's own problem.
    let message = match root {
        Some(error) => {
            fields.push((
                "kind",
                PyString::new(py, node_error_tag(error.kind.class()))
                    .unbind()
                    .into_any(),
            ));
            fields.push(("inner_kind", inner_kind(py, &error.kind)));
            format!(
                "{}; the failure there: {}",
                standing.spoken(doc),
                error.spoken(doc, evaluation)
            )
        }
        None => {
            fields.push(("kind", py.None().into_any()));
            fields.push(("inner_kind", py.None().into_any()));
            standing.spoken(doc)
        }
    };
    let err = typed_err(
        py,
        ErrorClass::Evaluation(EvalReason::Standing(standing)),
        message,
        &fields,
    );
    match root {
        Some(error) => with_carried(py, err, error.kind.carried_chain(), Some(doc)),
        None => err,
    }
}

/// Bulk mass properties of a body, in canonical units.
///
/// Volume and area are `m³` and `m²` — dimensions OUTSIDE D6's closed
/// `{Length, Angle, Count}` set, so they cross as plain floats in
/// canonical units rather than as invented quantity types.
#[pyclass(frozen, module = "pncad", skip_from_py_object)]
#[derive(Clone, Copy)]
pub(crate) struct MassProperties {
    /// Signed enclosed volume, m³.
    #[pyo3(get)]
    volume: f64,
    /// Total surface area, m².
    #[pyo3(get)]
    surface_area: f64,
    /// Certified half-width of the volume enclosure.
    #[pyo3(get)]
    volume_pad: f64,
    /// Certified half-width of the area enclosure.
    #[pyo3(get)]
    area_pad: f64,
}

impl From<topo::MassProperties<f64>> for MassProperties {
    /// The kernel's four fields, unchanged and unrounded — one
    /// crossing point, so the two measurement doors cannot answer
    /// differently shaped values.
    fn from(props: topo::MassProperties<f64>) -> Self {
        Self {
            volume: props.volume,
            surface_area: props.surface_area,
            volume_pad: props.volume_pad,
            area_pad: props.area_pad,
        }
    }
}

/// The refusal a MEASUREMENT door raises: the kernel's own message
/// under the `reason` a caller dispatches on, and which refusal it is
/// as `measure_refusal` ([`crate::tags::mass_props_error_tag`]): a face
/// with no enclosure, a predicate whose margin could not be read, a
/// resolution finer than ε, or a deciding predicate still undecided
/// where no face can refine (`"undecided"`). That last one carries its
/// bracket as data beside the word, as the kernel's refusal does:
/// `volume`, the enclosure's ends `(lo, hi)` read outward, and `stop`,
/// why no face could refine further. An unmet resolution is no refusal:
/// it is a [`MassMeasurement`] whose `stop` says so.
fn measurement_err(py: Python<'_>, err: &topo::MassPropsError) -> PyErr {
    let word = |w: &str| PyString::new(py, w).unbind().into_any();
    let mut fields = vec![(
        "measure_refusal",
        word(crate::tags::mass_props_error_tag(err)),
    )];
    if let topo::MassPropsError::Undecided { volume, stop } = err {
        fields.push((
            "volume",
            pyo3::types::PyTuple::new(py, [volume.0, volume.1])
                .map_or_else(|_| py.None(), |t| t.unbind().into_any()),
        ));
        fields.push(("stop", word(stop.word())));
    }
    typed_err(
        py,
        ErrorClass::Validation(ValidationRefusal::MassProperties),
        err.to_string(),
        &fields,
    )
}

#[pymethods]
impl MassProperties {
    fn __repr__(&self) -> String {
        format!(
            "MassProperties(volume={} m^3, surface_area={} m^2)",
            self.volume, self.surface_area
        )
    }
}

/// **What a measurement is asked** (`[tolerance.measure]`): a sign, a
/// relation against a volume bound, a containment, or a resolution.
/// There is no default: every measurement door takes one. Each deciding
/// constructor's decisions are recorded under the funnel name its
/// docstring gives.
#[pyclass(frozen, module = "pncad", from_py_object)]
#[derive(Clone, Copy)]
pub(crate) struct Settle(topo::Settle<f64>);

#[pymethods]
impl Settle {
    /// The volume's sign: `verdict` is `"outer"` for a positive volume
    /// and `"void"` for a negative one. Recorded under
    /// `py_measure_sign` (the walk's sums) and `py_measure_sign_exact`
    /// (the interval re-derivation).
    #[staticmethod]
    fn sign() -> Self {
        Self(topo::Settle::Sign(topo::SignNames {
            walk: topo::RoleNames::one("py_measure_sign"),
            certified: topo::RoleNames::one("py_measure_sign_exact"),
        }))
    }

    /// The volume at most `bound` (m³): `verdict` is whether it holds.
    /// Recorded under `py_measure_at_most`.
    #[staticmethod]
    fn at_most(bound: f64) -> Self {
        Self(topo::Settle::AtMost {
            bound,
            name: "py_measure_at_most",
        })
    }

    /// The volume at least `bound` (m³): `verdict` is whether it holds.
    /// Recorded under `py_measure_at_least`.
    #[staticmethod]
    fn at_least(bound: f64) -> Self {
        Self(topo::Settle::AtLeast {
            bound,
            name: "py_measure_at_least",
        })
    }

    /// The volume within `[lo, hi]` (m³): `verdict` is whether it holds.
    /// Recorded under `py_measure_contains`. A `lo` definitely above `hi`
    /// names no volume and is refused (`measure_refusal`
    /// `"empty_contains"`).
    #[staticmethod]
    fn contains(lo: f64, hi: f64) -> Self {
        Self(topo::Settle::Contains {
            lo,
            hi,
            name: "py_measure_contains",
        })
    }

    /// Each face refined until its enclosure spans at most `length` of
    /// mean boundary displacement: the predicate of a caller that wants
    /// the number. Decides nothing, and is recorded nowhere. A length
    /// finer than the tolerance is refused (`measure_refusal`
    /// `"resolution_finer_than_eps"`).
    #[staticmethod]
    fn resolution(length: &Length) -> Self {
        Self(topo::Settle::Resolution(length.0))
    }

    fn __repr__(&self) -> String {
        match self.0 {
            topo::Settle::Sign(_) => "Settle.sign()".to_owned(),
            topo::Settle::AtMost { bound, .. } => format!("Settle.at_most({bound})"),
            topo::Settle::AtLeast { bound, .. } => format!("Settle.at_least({bound})"),
            topo::Settle::Contains { lo, hi, .. } => format!("Settle.contains({lo}, {hi})"),
            topo::Settle::Resolution(len) => format!("Settle.resolution({} m)", len.meters()),
        }
    }
}

/// **What a measurement reached**: the narrowest enclosure it held, why
/// it stopped, and what a deciding predicate decided.
///
/// `stop` is `"settled"` (a deciding predicate settled), `"met"` (every
/// face met the resolution), `"floor"` (refinement stopped paying short
/// of the question), `"wall"` (a face short of the question reached the
/// work wall) or `"band"` (the enclosure lies wholly in the predicate's
/// band). `verdict` is `"outer"`/`"void"` for `Settle.sign()` settled,
/// a bool for a relation or containment settled, and `None` otherwise.
#[pyclass(frozen, module = "pncad", skip_from_py_object)]
#[derive(Clone)]
pub(crate) struct MassMeasurement {
    /// The volume and area enclosures.
    #[pyo3(get)]
    enclosure: MassProperties,
    stop: topo::Stop,
}

impl From<topo::MassMeasurement<f64>> for MassMeasurement {
    fn from(measured: topo::MassMeasurement<f64>) -> Self {
        Self {
            enclosure: measured.enclosure.into(),
            stop: measured.stop,
        }
    }
}

#[pymethods]
impl MassMeasurement {
    /// Why the measurement stopped, as `topo::Stop::word` spells it.
    #[getter]
    fn stop(&self) -> &'static str {
        self.stop.word()
    }

    /// What a deciding predicate decided, a `SettleVerdict` spelled as
    /// its words: `"outer"` or `"void"` for a sign (`SettleVerdict::Sign`),
    /// a bool for a relation or a containment (`SettleVerdict::Holds`),
    /// `None` where none settled.
    #[getter]
    fn verdict(&self, py: Python<'_>) -> Py<PyAny> {
        match self.stop {
            topo::Stop::Settled(topo::SettleVerdict::Sign(role)) => {
                PyString::new(py, role.word()).unbind().into_any()
            }
            topo::Stop::Settled(topo::SettleVerdict::Holds(holds)) => {
                pyo3::types::PyBool::new(py, holds)
                    .to_owned()
                    .unbind()
                    .into_any()
            }
            _ => py.None(),
        }
    }

    /// The enclosure as a number, where the measurement met its
    /// resolution (`stop == "met"`): `topo::MassMeasurement::met`.
    ///
    /// # Errors
    ///
    /// `ValidationError` with `measure_refusal == "unmet"` and `stop`,
    /// the stop the measurement reached instead.
    fn met(&self, py: Python<'_>) -> PyResult<MassProperties> {
        if self.stop == topo::Stop::Met {
            return Ok(self.enclosure);
        }
        let word = |w: &str| PyString::new(py, w).unbind().into_any();
        Err(typed_err(
            py,
            ErrorClass::Validation(ValidationRefusal::MassProperties),
            topo::Unmet { stop: self.stop }.to_string(),
            &[
                ("measure_refusal", word(crate::tags::MEASUREMENT_UNMET)),
                ("stop", word(self.stop.word())),
            ],
        ))
    }

    fn __repr__(&self) -> String {
        format!(
            "MassMeasurement(stop={}, volume={} ± {} m^3)",
            self.stop.word(),
            self.enclosure.volume,
            self.enclosure.volume_pad
        )
    }
}

/// A solid body — an OPAQUE handle.
///
/// Arena keys never cross; a body crosses as a handle whose
/// interior is reachable only through curated doors.
///
/// # The declared contacts ride WITH the body
///
/// `contacts` is the tier-3′ second argument (M3 PR 6a): the
/// declarations the op that produced this body minted for it. It is
/// CAPTURED here rather than crossing as a value, which is the
/// carrier-projection rule at a door whose Rust signature takes two
/// things a Python caller can only hold one of. `ContactRecords` has
/// no Python constructor and is curated `INTERIOR` — so a
/// `validate_pseudomanifold(contacts)` door would be uncallable — and
/// handing one body ANOTHER body's declarations is exactly the
/// mis-pairing the F1 contract exists to refuse (the validator never
/// blesses discovered contacts). The demo tour states the same rule
/// from the other side: its `SceneBody` carries `contacts` beside the
/// body and runs 3′ "with the op's OWN declared contacts".
///
/// Empty is the honest default, not a hole. With no declarations the
/// kernel's own contract is that 3′ ≡ tier 3 plus the census actually
/// run, which is the STRICTEST rung of the ladder — so a door that
/// mints a body without records can only make this gate refuse
/// (`UnrecordedContact`), never falsely pass.
#[pyclass(frozen, module = "pncad", from_py_object)]
#[derive(Clone)]
pub(crate) struct Body {
    pub(crate) inner: Arc<topo::Body<f64>>,
    contacts: Arc<topo::ContactRecords>,
}

impl Body {
    /// A body with no contact records — every door but those that have
    /// records to carry (an evaluated value, an import).
    pub(crate) fn plain(inner: Arc<topo::Body<f64>>) -> Self {
        Self {
            inner,
            contacts: Arc::new(topo::ContactRecords::default()),
        }
    }

    /// A body with the contact records its producer carries, each citing
    /// the decision that backs it.
    pub(crate) fn recorded(
        inner: Arc<topo::Body<f64>>,
        contacts: Arc<topo::ContactRecords>,
    ) -> Self {
        Self { inner, contacts }
    }
}

#[pymethods]
impl Body {
    /// Volume and area enclosures under `settle`, and why the
    /// measurement stopped.
    ///
    /// **The measurement reads a finished body** (Ev's decision on PR
    /// 4822), so this is [`Self::validate_geometric_measured`]: tier 3
    /// gates the body, and its own certified quadrature, continued under
    /// `settle`, is the measurement — one quadrature. A body the gate
    /// refuses raises the gate's `ValidationError` (door
    /// `validate_geometric`) rather than being measured.
    fn mass_properties(&self, py: Python<'_>, settle: Settle) -> PyResult<MassMeasurement> {
        self.validate_geometric_measured(py, settle)
    }

    /// Full validation. Raises `ValidationError` listing the failures.
    fn validate(&self, py: Python<'_>) -> PyResult<()> {
        self.run_validator(py, ValidationRefusal::Validate, topo::validate(&self.inner))
    }

    /// Closure validation only.
    fn validate_closed(&self, py: Python<'_>) -> PyResult<()> {
        self.run_validator(
            py,
            ValidationRefusal::Closed,
            topo::validate_closed(&self.inner),
        )
    }

    /// Tessellate at a chordal budget — the ladder's step 4.
    ///
    /// `chordal` is a DISTANCE: the maximum the piecewise-linear mesh
    /// may sag from the exact surface. It is deliberately not the
    /// kernel's ε, which `DocEdit.set_tolerance` sets and which
    /// decides what the model IS; this decides how coarsely a view of
    /// it may approximate it. Two budgets see the same body.
    fn tessellate(&self, py: Python<'_>, chordal: &Length) -> PyResult<super::mesh::Mesh> {
        super::mesh::tessellate(py, &self.inner, chordal)
    }

    /// Geometric validation only — tier 3, the verdict and nothing
    /// else.
    ///
    /// Its +V check runs a certified quadrature and this door drops
    /// it. A caller that wants the volume as well should take
    /// [`Self::validate_geometric_measured`], which is this gate with
    /// the measurement it already computed carried out of it.
    fn validate_geometric(&self, py: Python<'_>) -> PyResult<()> {
        let tol = Tol::witness();
        self.run_validator(
            py,
            ValidationRefusal::Geometric,
            topo::validate_geometric(&self.inner, tol),
        )
    }

    /// **Tier 3 and a measurement, in ONE certified quadrature** — the
    /// spelling that pays once.
    ///
    /// Tier 3's +V check IS a certified quadrature, and a measurement
    /// after it would start another from round 0. This door keeps the
    /// certificate the gate decided the sign on and continues THAT
    /// quadrature under `settle`, so the rounds the gate already paid for
    /// are not paid again.
    ///
    /// **Why a door and not a certificate.** The Rust certificate
    /// BORROWS the body it was derived from, so it cannot be handed
    /// out and asked for a number later — a lifetime does not survive
    /// the FFI boundary. Gating and measuring in one call is the shape
    /// that does cross.
    ///
    /// A resolution the quadrature cannot reach is no refusal: the
    /// `MassMeasurement`'s `stop` is `"floor"` or `"wall"`, and its enclosure
    /// is the bracket it reached.
    fn validate_geometric_measured(
        &self,
        py: Python<'_>,
        settle: Settle,
    ) -> PyResult<MassMeasurement> {
        let tol = Tol::witness();
        let certificate = match topo::validate_geometric_certificate(&self.inner, tol) {
            Ok(certificate) => certificate,
            // The gate that spoke is `validate_geometric`, and that is
            // the tag its refusal carries: a caller reading `door`
            // learns which gate refused, not which door it called.
            Err(failures) => {
                return Err(Self::validator_err(
                    py,
                    ValidationRefusal::Geometric,
                    &failures,
                )?);
            }
        };
        certificate
            .measure(settle.0)
            .map(MassMeasurement::from)
            .map_err(|err| measurement_err(py, &err))
    }

    /// **Tier 3′** — the ladder's fourth rung: tier 3's whole local
    /// battery PLUS the global coincidence census tier 3 defers,
    /// certified against THIS body's own declared contacts.
    ///
    /// The census is a diff in both directions. Every coincidence it
    /// finds must be backed by a declaration (`UnrecordedContact`
    /// otherwise — no scan-to-bless) and every declaration must be
    /// geometrically confirmed (`StaleContactDeclaration` otherwise).
    /// There is no contacts argument because a body already carries
    /// its own; see the type's docs for why that is the only pairing
    /// this door can honestly offer.
    ///
    /// So the verdict depends on which door minted the body, and that
    /// is the point rather than a wrinkle: a boolean result and an
    /// assembled product arrive with their declarations and pass over
    /// their seams; the SAME solids gathered by `product`, which
    /// declares nothing, arrive without and this gate reports the
    /// seam it finds. Raises `ValidationError` listing the failures,
    /// with `door`, `failure_count` and `findings` as on the other
    /// three rungs.
    fn validate_pseudomanifold(&self, py: Python<'_>) -> PyResult<()> {
        let tol = Tol::witness();
        self.run_validator(
            py,
            ValidationRefusal::Pseudomanifold,
            topo::validate_pseudomanifold(&self.inner, &self.contacts, tol),
        )
    }
}

impl Body {
    /// Shared shape for the validator doors, which all return
    /// `Result<(), Vec<ValidationError>>`.
    ///
    /// The exception carries `door`, the failure COUNT, and one
    /// [`ValidationFinding`] per failure, in the kernel's own
    /// deterministic report order. The human message is unchanged: each
    /// finding through the enum's own `Display`, one prose sentence with
    /// recourse, joined because a `Vec` has no rendering of its own —
    /// so the words are a branch a caller can take and the sentence is
    /// still the diagnosis they read.
    ///
    /// ONE raise per call, whatever the count. `failure_count` is what
    /// says the door found several, and splitting the raise would
    /// report one of N failures where the join reports all of them.
    ///
    /// It raises through [`typed_err`] like every other door: the
    /// kernel words each tier-3′ census finding through `Display`, so
    /// the joined message is prose the assertion accepts.
    fn run_validator(
        &self,
        py: Python<'_>,
        door: ValidationRefusal,
        outcome: Result<(), Vec<topo::ValidationError>>,
    ) -> PyResult<()> {
        let Err(failures) = outcome else {
            return Ok(());
        };
        Err(Self::validator_err(py, door, &failures)?)
    }

    /// The refusal a validate door raises over the verdict vector it
    /// collected: `door`, `failure_count` and `findings`, the three
    /// attributes every rung of the ladder carries.
    ///
    /// Split out of [`Self::run_validator`] because the gate half of
    /// [`Self::validate_geometric_measured`] collects the SAME vector
    /// and owes the same refusal, but carries a certificate rather
    /// than `()` when it passes — so it cannot be spelled through a
    /// `Result<(), _>`.
    fn validator_err(
        py: Python<'_>,
        door: ValidationRefusal,
        failures: &[topo::ValidationError],
    ) -> PyResult<PyErr> {
        // The word for the human sentence is the one the class mints
        // for `door`, read from the same map rather than restated.
        let door_word = crate::tags::validation_refusal_tag(door);
        let count = failures.len().into_pyobject(py)?.unbind().into_any();
        let findings: Vec<ValidationFinding> = failures
            .iter()
            .map(|failure| ValidationFinding(validation::project(failure)))
            .collect();
        let findings = findings.into_pyobject(py)?.unbind().into_any();
        Ok(typed_err(
            py,
            ErrorClass::Validation(door),
            format!(
                "{door_word} reported {} failure(s): {}",
                failures.len(),
                failures
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
                    .join("; ")
            ),
            &[("failure_count", count), ("findings", findings)],
        ))
    }
}

/// **ONE failure a validator found**, as words a caller branches on.
///
/// The value class behind `ValidationError.findings`, and **the single
/// place on this surface where a refusal's discriminant crosses in a
/// SEQUENCE rather than as a scalar attribute**. That is argued by this
/// door's own shape and by nothing else: `validate*` is the one door
/// that reports MANY refusals at once — `failure_count` has said so
/// since it was bound — so one `variant` string could only name one of
/// them. Everywhere else a refusal reports a single fault and its word
/// is a plain attribute; read this as the exception it is, not as a
/// second convention.
///
/// Frozen, constructed only by the binding, and compared by value: two
/// findings that say the same thing are `==`.
///
/// `variant` is which `ValidationError` arm refused. The other five
/// are its payload, `None` on an arm that carries none, so `getattr`
/// never raises and a caller never has to branch on `variant` first:
///
/// * `subject_kind` — what a census refusal is ABOUT: `"entity"` (one
///   carrier outside the certifiable inventory) or `"face_pair"` (a
///   candidate contact). The two are two different repairs.
/// * `entity_kind` — that entity's kind (`"face"`, `"edge"`,
///   `"vertex"`, …); `None` for a pair, whose sides are faces.
/// * `contact_kind` — which coincidence the tier-3′ census found
///   (`"vertex_on_face"`, `"edge_edge_cross"`, …). The branch that
///   matters: an `"edge_face_pierce"` is interpenetration and cannot
///   be declared, while an `"edge_edge_overlap"` can be.
/// * `crossing_side` — an `"edge_edge_cross"`'s side verdict:
///   `"same_side"` is interpenetration, `"opposite_sides"` a contact,
///   `"undecided"` neither; `None` on every other finding.
/// * `stale_kind` — which contact record the census could not confirm
///   (`"vertex_vertex"`, `"vertex_on_face"`, `"vertex_on_edge"`,
///   `"edge_edge"`, `"edge_face"`, `"patch"`). A record a declaration
///   made is withdrawn or re-seated at that granularity; withdrawing
///   another one leaves the refusal standing. `"vertex_on_edge"` and
///   `"edge_edge"` are records an op wrote, never a declaration: a stale
///   one is the op's defect, with nothing to withdraw.
/// * `ring_contact_kind` — how a ring meets its face's own outer loop,
///   or another ring of that face (`"vertex_vertex"`, `"vertex_on_edge"`, `"vertex_on_ring_edge"`,
///   `"edge_along_edge"`, `"edge_edge_point"`, `"circle_circle"`).
///   The word says where the ring has to move: a shared position one
///   vertex clears, a shared arc no single move separates, or a
///   crossing or touching point no vertex carries.
///
/// **No arena key crosses**, here as everywhere: a `Body` is an opaque
/// handle, so WHICH face or vertex a finding names stays in the
/// kernel's own prose on the message, and these words are what a
/// caller acts on.
#[pyclass(frozen, module = "pncad", skip_from_py_object)]
#[derive(Clone, Copy)]
pub(crate) struct ValidationFinding(validation::Finding);

#[pymethods]
impl ValidationFinding {
    /// Which `ValidationError` arm refused.
    #[getter]
    fn variant(&self) -> &'static str {
        self.0.variant
    }

    /// What a census refusal is about: `"entity"` or `"face_pair"`.
    #[getter]
    fn subject_kind(&self) -> Option<&'static str> {
        self.0.subject_kind
    }

    /// The entity kind of an `"entity"` subject.
    #[getter]
    fn entity_kind(&self) -> Option<&'static str> {
        self.0.entity_kind
    }

    /// Which coincidence the census found.
    #[getter]
    fn contact_kind(&self) -> Option<&'static str> {
        self.0.contact_kind
    }

    /// An in-plane crossing's side verdict.
    #[getter]
    fn crossing_side(&self) -> Option<&'static str> {
        self.0.crossing_side
    }

    /// Which declared record lost its witness.
    #[getter]
    fn stale_kind(&self) -> Option<&'static str> {
        self.0.stale_kind
    }

    /// How a ring meets its face's own outer loop, or another ring of
    /// that face.
    #[getter]
    fn ring_contact_kind(&self) -> Option<&'static str> {
        self.0.ring_contact_kind
    }

    /// A `volume_sign_unresolved` finding's volume bracket `(lo, hi)`
    /// in m³, read outward: the true volume lies between them.
    #[getter]
    fn volume(&self) -> Option<(f64, f64)> {
        self.0
            .volume
            .map(|[lo, hi]| (f64::from_bits(lo), f64::from_bits(hi)))
    }

    fn __repr__(&self) -> String {
        // Python's own spelling of an absent word, not Rust's: a repr
        // a reader can paste back is the whole point of one.
        fn word(value: Option<&str>) -> String {
            value.map_or_else(|| "None".to_owned(), |word| format!("'{word}'"))
        }
        format!(
            "ValidationFinding(variant='{}', subject_kind={}, \
             entity_kind={}, contact_kind={}, crossing_side={}, \
             stale_kind={}, ring_contact_kind={})",
            self.0.variant,
            word(self.0.subject_kind),
            word(self.0.entity_kind),
            word(self.0.contact_kind),
            word(self.0.crossing_side),
            word(self.0.stale_kind),
            word(self.0.ring_contact_kind)
        )
    }

    fn __eq__(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

/// **A measured quantity** (ERROR-DESIGN E3): the value a `Measure`
/// node evaluated to, with the F1 dimension it was measured in.
///
/// The dimension rides the measure rather than the reader inferring it
/// — that is E3's claim, and a Python consumer gets it as data. The
/// value is reported in canonical kernel units (metres for a `Length`,
/// radians for an `Angle`); a `Length` measure additionally answers
/// through the typed quantity, which is the spelling the rest of this
/// surface uses.
#[pyclass(frozen, module = "pncad")]
pub(crate) struct Measurement {
    /// The dimension measured in: `"Length"`, `"Angle"`, `"Count"`
    /// or `"Scalar"`, capitalized where every other door's dimension
    /// word is not. Spelled out because this docstring is the only
    /// place a Python caller can read the four — `pncad.pyi` names
    /// the attribute and not its words.
    // The map is `crate::errors::measurement_dimension_tag`, held to
    // `dimension_tag`'s lower-case four by
    // `the_two_dimension_alphabets_are_one_list_in_two_cases`.
    #[pyo3(get)]
    dimension: &'static str,
    /// The measured value in canonical kernel units.
    #[pyo3(get)]
    value: f64,
    /// The value as a typed `Length`; `None` for every other
    /// dimension, because a `Length` is what a length is.
    #[pyo3(get)]
    length: Option<Length>,
}

#[pymethods]
impl Measurement {
    fn __repr__(&self) -> String {
        format!("Measurement({} {})", self.value, self.dimension)
    }
}

/// **An assertion's verdict** (ERROR-DESIGN E10), REPORT-ONLY.
///
/// Three states, kept three: `holds` is `True`, `False`, or `None`
/// where the run's tolerance could not separate the measurement from
/// the bound. Nothing collapses the third into a silent pass — that is
/// the whole reason the state exists.
///
/// `measured` and `bound` are present for a decided verdict and `None`
/// for an undecided one. Reading a verdict changes nothing: a failing
/// assertion gates no build and moves no product (D10).
#[pyclass(frozen, module = "pncad")]
pub(crate) struct Verdict {
    /// `"Holds"`, `"Violated"` or `"Unevaluated"`.
    #[pyo3(get)]
    status: &'static str,
    /// `True`/`False` for a decided verdict, `None` for an undecided
    /// one.
    #[pyo3(get)]
    holds: Option<bool>,
    /// What the measure evaluated to, in canonical kernel units.
    #[pyo3(get)]
    measured: Option<f64>,
    /// What the bound evaluated to, in canonical kernel units.
    #[pyo3(get)]
    bound: Option<f64>,
    /// Why there is no verdict; `None` when there is one.
    #[pyo3(get)]
    reason: Option<String>,
}

#[pymethods]
impl Verdict {
    fn __repr__(&self) -> String {
        match (self.measured, self.bound) {
            (Some(m), Some(b)) => format!("Verdict({}: {m} vs {b})", self.status),
            _ => format!("Verdict({})", self.status),
        }
    }
}

/// Project a canonical-metre point into typed `Length`s.
pub(crate) fn lengths(p: pncad::geom_core::Point3<f64>) -> (Length, Length, Length) {
    (
        Length(pncad::quantity::Length::from_meters(p.x)),
        Length(pncad::quantity::Length::from_meters(p.y)),
        Length(pncad::quantity::Length::from_meters(p.z)),
    )
}

/// Project a materializer's names into their canonical text — the one
/// alphabet `Node.fillet` reads (see `doc::name_text`).
fn names(py: Python<'_>, found: Vec<pncad::prelude::StableName>) -> PyResult<Vec<String>> {
    found.iter().map(|n| super::doc::name_text(py, n)).collect()
}

/// A node's successful value.
#[pyclass(frozen, module = "pncad", skip_from_py_object)]
#[derive(Clone)]
pub(crate) struct Value {
    payload: d::ValuePayload<f64>,
    /// The value's contact records (ASM-R2b D-1), as its carry holds
    /// them: a boolean's surviving contacts, or what a copy carries
    /// from its source. [`Value::recorded_body`] reads them as
    /// `editor_core::product::sources_of` does, because a Python caller
    /// reading a body off a value and the gather reading the same body
    /// must not disagree about the contacts it carries.
    contacts: Arc<topo::ContactRecords>,
    node: NodeId,
    /// A family's members (D10 `[repetition]`), in index order: each
    /// member's index values, outermost first, and its value. Empty for
    /// a node evaluated once.
    members: Vec<(Vec<u32>, Value)>,
}

impl Value {
    /// `node`'s value as the evaluation holds it, its members with it.
    pub(crate) fn of(node_value: &d::NodeValue<f64>, node: NodeId) -> Self {
        Self {
            payload: node_value.payload.clone(),
            contacts: Arc::clone(&node_value.carry.contacts),
            node,
            members: node_value.members.as_ref().map_or_else(Vec::new, |family| {
                family
                    .members
                    .iter()
                    .map(|(at, member)| (at.clone(), Self::of(member, node)))
                    .collect()
            }),
        }
    }

    /// One of this value's bodies with the contact records that body
    /// carries — `sources_of`'s rule, restated at the boundary.
    ///
    /// Multi-output payloads carry no records (the `OpOut` invariant:
    /// "output body 0" names nothing there), so `Instances` and
    /// `Split` halves are plain — no home to read from, and none is
    /// invented.
    ///
    /// The body is read as it stands in its space, through the façade's
    /// read-out door ([`pncad::export::body_in_space`]); its records
    /// cite it there as they cite its own coordinates, the rigid map
    /// keeping every key.
    fn recorded_body(&self, py: Python<'_>, body: &d::Framed<f64>) -> PyResult<Body> {
        let body = pncad::export::body_in_space(body, Tol::witness())
            .map_err(|err| compose_err(py, self.node, &err))?;
        Ok(match &self.payload {
            d::ValuePayload::Body(_) | d::ValuePayload::Boolean(d::BooleanValue::Body { .. }) => {
                Body::recorded(body, Arc::clone(&self.contacts))
            }
            _ => Body::plain(body),
        })
    }
}

#[pymethods]
impl Value {
    /// The payload's kind tag, straight from the kernel's own
    /// `ValuePayload::kind_name` — so the Python tag set cannot drift
    /// from the document layer's.
    #[getter]
    fn kind(&self) -> &'static str {
        self.payload.kind_name()
    }

    /// **A family's members** (D10 `[repetition]`), in index order: each
    /// member's index values (one per index the family reaches,
    /// outermost first) and its value. Empty for a node evaluated once.
    /// A family of bodies also answers them flat (`bodies`); a family
    /// of anything else (`kind == "family"`) answers them here only.
    fn members(&self) -> Vec<(Vec<u32>, Value)> {
        self.members.clone()
    }

    /// The single body this value denotes.
    ///
    /// Accepts `Body` (a placement's or an instance's copy and a `⊔`
    /// among them) and a non-empty `Boolean`; raises for every other
    /// kind, and for an empty Boolean.
    fn body(&self, py: Python<'_>) -> PyResult<Body> {
        match &self.payload {
            d::ValuePayload::Body(body) => self.recorded_body(py, body),
            d::ValuePayload::Boolean(d::BooleanValue::Body { body, .. }) => {
                self.recorded_body(py, body)
            }
            d::ValuePayload::Boolean(d::BooleanValue::Empty) => Err(eval_err(
                py,
                "the Boolean produced an empty result",
                EvalReason::EmptyBoolean,
                self.node,
            )),
            other => Err(eval_err(
                py,
                format!("a `{}` value is not a body", other.kind_name()),
                EvalReason::WrongKind,
                self.node,
            )),
        }
    }

    /// Every body this value denotes: one for `Body`/`Boolean`, the
    /// whole list for `Instances`, both sides for a
    /// `Split`.
    fn bodies(&self, py: Python<'_>) -> PyResult<Vec<Body>> {
        let wrap = |body: &d::Framed<f64>| self.recorded_body(py, body);
        let side = |s: &d::SplitSide<f64>| match s {
            d::SplitSide::Body(body) => Some(wrap(body)),
            d::SplitSide::Empty => None,
        };
        match &self.payload {
            d::ValuePayload::Body(body) => Ok(vec![wrap(body)?]),
            d::ValuePayload::Boolean(d::BooleanValue::Body { body, .. }) => Ok(vec![wrap(body)?]),
            d::ValuePayload::Instances(bodies) => bodies.iter().map(wrap).collect(),
            d::ValuePayload::Split { above, below } => {
                [side(above), side(below)].into_iter().flatten().collect()
            }
            _ => Ok(Vec::new()),
        }
    }

    /// A split's two sides, `(above, below)`; `None` where empty.
    fn split(&self, py: Python<'_>) -> PyResult<(Option<Body>, Option<Body>)> {
        let side = |s: &d::SplitSide<f64>| match s {
            d::SplitSide::Body(body) => pncad::export::body_in_space(body, Tol::witness())
                .map(|body| Some(Body::plain(body)))
                .map_err(|err| compose_err(py, self.node, &err)),
            d::SplitSide::Empty => Ok(None),
        };
        match &self.payload {
            d::ValuePayload::Split { above, below } => Ok((side(above)?, side(below)?)),
            other => Err(eval_err(
                py,
                format!("a `{}` value is not a split", other.kind_name()),
                EvalReason::WrongKind,
                self.node,
            )),
        }
    }

    /// The quantity a `Measure` node evaluated to (E3).
    ///
    /// **A measure with no value at this scalar refuses by NAME**
    /// (M10-6, R2's MINOR-4). `min_clearance` is answered by an engine
    /// that needs an enclosure, so at the `f64` scalar a Python caller
    /// drives it has no value — a typed ABSENCE, not a failure, which
    /// is why the node evaluates at all. That absence shares
    /// `kind_name()` with a taken measurement ("measure"), so the
    /// generic arm below produced the nonsense "a `measure` value is
    /// not a measure" and told a caller nothing about what to do.
    /// `Value.assertion` already carried its reason through; this door
    /// now does the same.
    ///
    /// The absence raises `MeasureUnavailableAt` and not
    /// `EvaluationError`: it is the kernel's own typed reason, and it
    /// carries the verb, the scalar this build ran at and the DOOR
    /// that can answer, so the recourse is in the refusal rather than
    /// in a reader's memory. `EvaluationError` is for a node that
    /// FAILED, and this one did not.
    fn measure(&self, py: Python<'_>) -> PyResult<Measurement> {
        match &self.payload {
            d::ValuePayload::Measure { value, dim } => Ok(Measurement {
                dimension: measurement_dimension_tag(*dim),
                value: *value,
                length: (*dim == d::Dimension::Length)
                    .then(|| Length(pncad::quantity::Length::from_meters(*value))),
            }),
            d::ValuePayload::MeasureUnavailable { reason, .. } => {
                Err(super::measure::measure_unavailable_at_err(py, reason))
            }
            other => Err(eval_err(
                py,
                format!("a `{}` value is not a measure", other.kind_name()),
                EvalReason::WrongKind,
                self.node,
            )),
        }
    }

    /// The verdict an `Assertion` node evaluated to (E10) — report
    /// only: reading it changes nothing about the document.
    fn assertion(&self, py: Python<'_>) -> PyResult<Verdict> {
        let d::ValuePayload::Assertion(verdict) = &self.payload else {
            return Err(eval_err(
                py,
                format!("a `{}` value is not an assertion", self.payload.kind_name()),
                EvalReason::WrongKind,
                self.node,
            ));
        };
        let (measured, bound, reason) = match verdict {
            d::AssertionVerdict::Holds { measured, bound }
            | d::AssertionVerdict::Violated { measured, bound } => {
                (Some(*measured), Some(*bound), None)
            }
            d::AssertionVerdict::Unevaluated { reason } => (None, None, Some(reason.to_string())),
        };
        Ok(Verdict {
            status: verdict.label(),
            holds: verdict.holds(),
            measured,
            bound,
            reason,
        })
    }

    fn __repr__(&self) -> String {
        format!("Value({})", self.payload.kind_name())
    }
}

/// The result of evaluating a document: the per-node result DAG.
#[pyclass(frozen, module = "pncad")]
pub(crate) struct Evaluation {
    pub(crate) inner: d::Evaluation<f64>,
    /// The evaluated document's parameter bindings, captured at
    /// `evaluate` — `select_where`'s decided atoms state their value
    /// as an `Expr`, which cannot be evaluated without them. Captured
    /// HERE because the answer must be as of the same document the
    /// evaluation is of; threading the doc back in per query would
    /// let the two drift.
    params: d::VarEnv<f64>,
    /// The document the evaluation ran on, captured at `evaluate` for
    /// the same reason [`Self::params`] is — and this is the whole of
    /// what the kernel's `RunCtx` is: a run is a (document,
    /// evaluation) PAIR, and resolution needs both halves (the
    /// document decides whether a stored name's minting node is still
    /// there at all, which no evaluation can answer).
    ///
    /// **Captured rather than taken per call**, which is the same
    /// argument `NodePick` makes about its `(node, body)` pairing:
    /// Python's `Doc` is mutable and `accept` swaps the document
    /// under the handle, so a `resolve(doc, name)` door would let a
    /// caller ask this evaluation about a document it is not of, and
    /// answer confidently against the wrong recipe. Pairing the two
    /// here makes that unspellable.
    ///
    /// Shared, so a report taken of the pair keeps the document it
    /// speaks from without a copy of it.
    doc: Arc<d::ProfileDoc>,
    /// The document's gathered product, materialized on the first ask
    /// and kept for every later one
    /// ([`crate::product_memo`], which holds the whole of the reasoning).
    ///
    /// It lives HERE because a product is a pure function of the pair
    /// above and the run's tolerance, and this object is that pair: it
    /// is frozen, so nothing can move under the memo, and the doors
    /// that want a product — `run_checks`, `product`, `product_named` —
    /// share one gather.
    product: crate::product_memo::ProductMemo,
}

impl Evaluation {
    /// The document this evaluation is of: the one a refusal raised
    /// from it speaks its nodes from.
    pub(crate) fn doc(&self) -> &d::ProfileDoc {
        &self.doc
    }

    /// [`Self::doc`], shared: what a report taken of this pair keeps to
    /// speak from.
    pub(crate) fn doc_shared(&self) -> Arc<d::ProfileDoc> {
        Arc::clone(&self.doc)
    }

    /// The (document, evaluation) pair and the memo over it, as the
    /// arguments [`crate::product_memo`]'s doors take.
    pub(crate) fn gathered<T>(
        &self,
        f: impl FnOnce(&crate::product_memo::ProductMemo, &d::ProfileDoc, &d::Evaluation<f64>) -> T,
    ) -> T {
        f(&self.product, &self.doc, &self.inner)
    }

    /// The DI3 pairing gate: `doc` must be the document this
    /// evaluation is of. `Err` carries the two ids the caller's own
    /// refusal arm names.
    pub(crate) fn paired_with(&self, doc: &super::doc::Doc) -> Result<(), d::Mispaired> {
        crate::product_memo::ProductMemo::paired(&self.inner, doc.inner.id())
    }
}

#[pymethods]
impl Evaluation {
    /// **A scalar's value in this evaluation, measured values bound**
    /// (D10): a variable — a measure's output, or a definition over
    /// outputs — or a formula, such as `Doc.measure`'s `value`. The
    /// one reader of an observed value outside an assertion.
    ///
    /// A measure under it that did not land raises as `value` of that
    /// measure does (`EvaluationError`, `MeasureUnavailableAt` for a
    /// `min_clearance` at this point scalar); an evaluation over the
    /// bound values that refuses raises `ExprError`.
    fn reading(&self, py: Python<'_>, value: super::doc::Evaluand) -> PyResult<Measurement> {
        let formula = match value {
            super::doc::Evaluand::Formula(formula) => self
                .doc
                .resolve(&formula.0)
                .map_err(|fault| super::expr::lower_fault_err(py, &fault))?,
            super::doc::Evaluand::Var(var) => {
                let Some(dim) = self.doc.var(var.0).and_then(|held| held.kind().dimension()) else {
                    return Err(super::expr::eval_err(
                        py,
                        &d::EvalError::UnresolvedVar { var: var.0 },
                        Some(&self.doc),
                    ));
                };
                d::Formula::var(var.0, dim)
            }
        };
        let dim = formula.dim();
        match self.inner.reading_formula(&self.doc, &formula) {
            Ok(d::Observed::Value(value)) => Ok(Measurement {
                dimension: measurement_dimension_tag(dim),
                value,
                length: (dim == d::Dimension::Length)
                    .then(|| Length(pncad::quantity::Length::from_meters(value))),
            }),
            Ok(d::Observed::Unavailable(reason)) => {
                Err(super::measure::measure_unavailable_at_err(py, &reason))
            }
            Err(d::ObservedRefusal::Measure(standing)) => {
                let node = NodeId(standing.node());
                match self.value(py, &node) {
                    Err(raised) => Err(raised),
                    Ok(_) => Err(eval_err(
                        py,
                        standing.to_string(),
                        EvalReason::Standing(standing),
                        node,
                    )),
                }
            }
            Err(d::ObservedRefusal::Expr(err)) => {
                Err(super::expr::eval_err(py, &err, Some(&self.doc)))
            }
        }
    }

    /// The node's successful value.
    ///
    /// A node that produced NO value raises under its standing, with
    /// the REAL typed cause — never a placeholder: `reason` is
    /// `"node_failed"` or `"poisoned"`, `kind` is the `NodeErrorKind`'s
    /// stable tag, a poisoning carries `through`, and the message is
    /// the kernel's own `NodeError` (a failed node) or the standing's
    /// sentence followed by it (a poisoned one).
    ///
    /// A node with no ENTRY at all is two states, kept apart:
    /// `unknown_node` for an id this document does not have, and
    /// `node_not_evaluated` for a node a canceled run never reached.
    fn value(&self, py: Python<'_>, node: &NodeId) -> PyResult<Value> {
        let standing = match self.inner.usable(node.0) {
            Ok(node_value) => {
                return Ok(Value::of(node_value, *node));
            }
            Err(standing) => standing,
        };
        Err(standing_raise(py, &self.doc, &self.inner, *node, standing))
    }

    /// **Whether this run was CANCELED** — the Python shape of the
    /// kernel's `EvalOutcome`, whose two variants are a yes/no and
    /// cross as one.
    ///
    /// `True` means the token passed as [`super::value::evaluate`]'s
    /// `cancel=` was observed set at a yield point, so
    /// [`Self::order`] is still the FULL deterministic order (order is
    /// data, not schedule) while the results hold the completed
    /// PREFIX only. Every node past that prefix answers `False` from
    /// [`Self::succeeded`] and raises `node_not_evaluated` from
    /// [`Self::value`] — a canceled run is a PARTIAL ANSWER, never a
    /// failed one, and no node in it is marked failed by the
    /// cancelation.
    ///
    /// `False` without a `cancel=` token is not a claim worth
    /// doubting: `evaluate` mints a fresh token nobody can reach, so
    /// a run with no token cannot be canceled.
    #[getter]
    fn canceled(&self) -> bool {
        self.inner.outcome == d::EvalOutcome::Canceled
    }

    /// **The coincidences `node` decided from values** (D10), in
    /// decision order, each with what the coincidence door decided
    /// about it. A node with no value raises as [`Self::value`] does.
    fn coincidences(&self, py: Python<'_>, node: &NodeId) -> PyResult<Vec<Coincidence>> {
        self.value(py, node)?;
        let Some(value) = self.inner.value(node.0) else {
            return Ok(Vec::new());
        };
        let Ok(proofs) = d::coincide::prove(
            &self.doc,
            &[(node.0, &*value.coincidences)],
            self.inner.resolver(),
            Tol::witness(),
            &d::CancelToken::new(),
        ) else {
            unreachable!("a token no one else holds is never canceled")
        };
        value
            .coincidences
            .iter()
            .zip(proofs.into_iter().flatten())
            .map(|(row, proof)| Coincidence::new(py, row, &proof, &self.doc))
            .collect()
    }

    /// Whether the node produced a value.
    fn succeeded(&self, node: &NodeId) -> bool {
        self.inner.value(node.0).is_some()
    }

    /// The evaluation order: every node the evaluation ran or would
    /// run, the world (`Doc.world`) among them.
    fn order(&self) -> Vec<NodeId> {
        self.inner.order.iter().copied().map(NodeId).collect()
    }

    /// **Every edge name of `node`'s output body, as of THIS
    /// evaluation** — the name materializer, crossing as text.
    ///
    /// This is the door a `Node.fillet` selection comes through, and
    /// it MATERIALIZES rather than queries: it answers for the
    /// evaluation in hand, the caller stores the answer, and from that
    /// moment the selection is frozen like any other. A recipe holds
    /// no live "all edges", because a stored one would silently grow
    /// under an upstream edit — the staleness the freeze prevents.
    ///
    /// The answer is the WHOLE kind, and each string is an OPAQUE
    /// identifier: its internal structure is not API (see
    /// `doc::name_text`), so narrowing the set is a SELECTOR's job —
    /// [`Self::select`] and [`Self::select_where`], which
    /// answer in the same alphabet.
    ///
    /// Empty when the node has no value, no name table, or no edges.
    /// The fillet's insert is what refuses an EMPTY selection, so the
    /// emptiness surfaces there rather than here.
    fn all_edges(&self, py: Python<'_>, node: super::doc::AtArg) -> PyResult<Vec<String>> {
        names(py, pncad::select::all_edges(&self.inner, node.at()))
    }

    /// Every FACE name of `node`'s output, [`Self::all_edges`]'s
    /// sibling — same contract, same alphabet.
    fn all_faces(&self, py: Python<'_>, node: super::doc::AtArg) -> PyResult<Vec<String>> {
        names(py, pncad::select::all_faces(&self.inner, node.at()))
    }

    /// Every VERTEX name of `node`'s output, same contract.
    fn all_vertices(&self, py: Python<'_>, node: super::doc::AtArg) -> PyResult<Vec<String>> {
        names(py, pncad::select::all_vertices(&self.inner, node.at()))
    }

    /// Every BODY name `node`'s evaluation carries, same contract.
    /// Usually one row; a family's members are the plural case. A
    /// split's halves are two reads (`Doc.output(split, 0 | 1)`) with a
    /// row each.
    fn all_bodies(&self, py: Python<'_>, node: super::doc::AtArg) -> PyResult<Vec<String>> {
        names(py, pncad::select::all_bodies(&self.inner, node.at()))
    }

    /// **Materialize a STRUCTURAL selection**: every name of `node`'s
    /// output matching `selector`'s role-path shape, as of THIS
    /// evaluation — Rust's `select`, same contract as `all_edges`
    /// (canonical order, the caller stores it, frozen thereafter).
    ///
    /// The answer is the same opaque texts the whole-kind
    /// materializers speak, ready for `Node.fillet` unread: narrowing
    /// happens through this door, never by parsing a name (name text
    /// is an identifier, not a value).
    ///
    /// Infallible like `select`: empty when `node` has no value, no
    /// name table, or nothing matches.
    fn select(
        &self,
        py: Python<'_>,
        node: super::doc::AtArg,
        selector: &super::select::Selector,
    ) -> PyResult<Vec<String>> {
        names(
            py,
            pncad::select::select(&self.inner, node.at(), &selector.0),
        )
    }

    /// **Materialize a selection narrowed by GEOMETRY**: `selector`
    /// narrows by name shape, then each survivor is resolved to its
    /// entity in THIS evaluation and tested against the CONJUNCTION
    /// `geom` — Rust's `select_where`, verb for verb. An empty `geom`
    /// makes this exactly [`Self::select`]; run it twice and
    /// concatenate for a geometric union.
    ///
    /// Same materializer contract and same opaque-text alphabet as
    /// [`Self::select`]: the binding narrows, your code never reads
    /// inside a name.
    ///
    /// Raises `SelectRefusal`, typed, where the Rust door refuses:
    /// exact atoms are total and cannot refuse, but a DECIDED atom's
    /// in-band margin (`reason="in_band"`), a tied name whose
    /// candidates disagree (`"tied_disagrees"`), an unreadable
    /// candidate (`"unreadable"`), a distance measured from a face that
    /// is not planar (`"not_a_plane"`) or that the body does not hold
    /// (`"face_unresolved"`) all refuse rather than silently including or
    /// dropping a candidate. The band this query classifies against is
    /// built before any candidate is read, so an ambient tolerance that
    /// admits none refuses too, under the band constructor's OWN word —
    /// `reason="empty"` or `"invalid_value"`, never a flat `"band"`,
    /// because a collapsed band and an overflowed one want opposite
    /// repairs.
    fn select_where(
        &self,
        py: Python<'_>,
        node: super::doc::AtArg,
        selector: &super::select::Selector,
        geom: Vec<super::select::GeomPred>,
    ) -> PyResult<Vec<String>> {
        let tol = Tol::witness();
        let atoms: Vec<pncad::select::GeomPred> = geom.into_iter().map(|g| g.0).collect();
        match pncad::select::select_where(
            &self.inner,
            node.at(),
            &selector.0,
            &atoms,
            &self.params,
            tol,
        ) {
            Ok(found) => names(py, found),
            Err(refusal) => Err(super::select::select_refusal(
                py,
                &refusal,
                &self.doc,
                &self.inner,
            )),
        }
    }

    /// **Where is the face I selected?** — the named face's carrier
    /// frame, as of THIS evaluation.
    ///
    /// The forward twin of the materializers: they hand out names,
    /// and this asks one where it SITS. `name` is one of the opaque
    /// texts `all_faces` / `select` / `select_where` answered with,
    /// handed back unread.
    ///
    /// The answer is the CARRIER's frame, copied out of stored
    /// geometry — a definitional re-read, so no pad and no
    /// measurement. It is not a verdict: no door here says whether a
    /// face is planar or where it is relative to anything, which is
    /// `select_where`'s decided half.
    ///
    /// Raises `ReadbackError`, typed: `no_such_name` for a stale
    /// selection, `ambiguous` for an N2 tie (ask
    /// [`Self::denotation`] first), `wrong_kind` for an edge or
    /// vertex name, `no_canonical_frame` for a NURBS carrier, and the
    /// node ladder for a node this evaluation did not produce.
    fn face_frame(
        &self,
        py: Python<'_>,
        node: super::doc::AtArg,
        name: &str,
    ) -> PyResult<super::readback::Pose> {
        let name = super::doc::name_from_text(name)?;
        pncad::select::face_frame(&self.inner, node.at(), &name)
            .map(super::readback::Pose)
            .map_err(|err| super::readback::readback_err(py, &err, &self.doc))
    }

    /// **The frame a sketch tracing a face stands on** — `Through` the
    /// traced face's own axis and a vertex of the plane face `plane`,
    /// square to it, its origin that vertex's foot on the axis
    /// (`pncad::trace::traced_frame`). Each is a `(body, name)` of this
    /// evaluation's document; pass the answer as `Node.drawn_profile`'s
    /// `frame`. A boss swept up the axis from a piece leaving the traced
    /// wall's trace along its tangent is tangent to the wall by
    /// construction.
    ///
    /// Raises `ValueError` naming why no frame reads off the two: the
    /// traced face has no axis, the plane face is no plane square to it,
    /// or has no vertex off the axis.
    fn traced_frame(
        &self,
        plane: (super::doc::OperandArg, String),
        traced: (super::doc::OperandArg, String),
    ) -> PyResult<super::pose::PoseDef> {
        let face = |(body, name): (super::doc::OperandArg, String)| {
            Ok::<_, PyErr>(d::Operand::select(
                body.read(),
                vec![super::doc::name_from_text(&name)?],
            ))
        };
        pncad::trace::traced_frame(
            &self.doc,
            &self.inner,
            &face(plane)?,
            &face(traced)?,
            Tol::witness(),
        )
        .map(|inner| super::pose::PoseDef { inner })
        .map_err(|fault| pyo3::exceptions::PyValueError::new_err(fault.to_string()))
    }

    /// **Where is the edge I selected?** — the named edge's certified
    /// carrier frame, [`Self::face_frame`]'s sibling.
    ///
    /// A straight edge answers with `u_ref is None`: a line has a
    /// direction and no distinguished perpendicular, and the door
    /// says so rather than inventing one.
    ///
    /// Raises `ReadbackError` as [`Self::face_frame`] does, with
    /// `wrong_kind` for a non-edge name and `no_carrier` for an edge
    /// still carrying null-edge scaffolding.
    fn edge_frame(
        &self,
        py: Python<'_>,
        node: super::doc::AtArg,
        name: &str,
    ) -> PyResult<super::readback::Pose> {
        let name = super::doc::name_from_text(name)?;
        pncad::select::edge_frame(&self.inner, node.at(), &name)
            .map(super::readback::Pose)
            .map_err(|err| super::readback::readback_err(py, &err, &self.doc))
    }

    /// **Where is the vertex I selected?** — the named vertex's
    /// stored position, dimensioned.
    ///
    /// Raises `ReadbackError` as [`Self::face_frame`] does, with
    /// `wrong_kind` for a non-vertex name.
    fn vertex_position(
        &self,
        py: Python<'_>,
        node: super::doc::AtArg,
        name: &str,
    ) -> PyResult<(Length, Length, Length)> {
        let name = super::doc::name_from_text(name)?;
        pncad::select::vertex_position(&self.inner, node.at(), &name)
            .map(lengths)
            .map_err(|err| super::readback::readback_err(py, &err, &self.doc))
    }

    /// **What KIND of surface carries the face I selected?** — the
    /// face's stored carrier tag, copied out.
    ///
    /// A tag READ, never a verdict: `SurfaceKind.Plane` comes back
    /// because the body RECORDS a plane there, and "is this face
    /// planar" is a comparison the caller makes against the answer.
    /// No tolerance enters and nothing is decided — the same split
    /// [`Self::face_frame`] keeps between a value and a predicate.
    ///
    /// It is the door [`Self::face_frame`] cannot be: a NURBS carrier
    /// has no canonical frame and the frame door refuses it
    /// (`no_canonical_frame`), while its kind is still readable here.
    /// It is also how a caller checks a face BEFORE reading a plane off
    /// it (`PoseDef.plane`, `GeomPred.plane_distance`), which refuses a
    /// non-planar carrier.
    ///
    /// Raises `ReadbackError` as [`Self::face_frame`] does, with
    /// `wrong_kind` for an edge or vertex name.
    fn face_carrier_kind(
        &self,
        py: Python<'_>,
        node: super::doc::AtArg,
        name: &str,
    ) -> PyResult<super::select::SurfaceKind> {
        let name = super::doc::name_from_text(name)?;
        pncad::select::face_carrier_kind(&self.inner, node.at(), &name)
            .map(super::select::surface_kind)
            .map_err(|err| super::readback::readback_err(py, &err, &self.doc))
    }

    /// **How does this name resolve — uniquely, or as a tie?** The
    /// referencing question, answered without exposing what it
    /// resolves to.
    ///
    /// This is the door to ask BEFORE a frame: the three frame doors
    /// refuse an N2 tie (`ambiguous`) rather than picking a
    /// candidate, and this says whether one is coming. It answers a
    /// COUNT, never the candidates — those are arena keys, which do
    /// not cross.
    ///
    /// Raises `ReadbackError` for the node ladder and `no_such_name`.
    fn denotation(
        &self,
        py: Python<'_>,
        node: super::doc::AtArg,
        name: &str,
    ) -> PyResult<super::readback::Denotation> {
        let name = super::doc::name_from_text(name)?;
        pncad::select::denotation(&self.inner, node.at(), &name)
            .map(super::readback::Denotation)
            .map_err(|err| super::readback::readback_err(py, &err, &self.doc))
    }

    /// **Does this STORED name still denote, in THIS evaluation?** —
    /// the question every consumer that keeps names must ask on every
    /// run, and the one Python's whole store-then-reuse story runs on.
    ///
    /// Answers a `Resolution`: "this name is gone" is a verdict, not
    /// an error, and the three states are kept three
    /// because their repairs differ. `resolved` carries `node`, `body`
    /// and `kind`; `failed` means rebind (with `offers` as the
    /// kernel's suggestions, never its substitutions); `indeterminate`
    /// means the name is fine and the RUN is not — the minting node
    /// failed, was poisoned or never evaluated, so the repair is
    /// upstream and the name resolves again when that node does.
    ///
    /// **In one read**, as every name door is (NAMES N1: a name tells
    /// apart only what its variable holds): `at` is a `Var` — an
    /// output, `Doc.output(split, port)` for a split's half — or a node,
    /// read at its one body output. A `Var` the document no longer
    /// holds answers `failed` / `read_gone`: re-site the reference. A
    /// node the document no longer holds answers `node_gone` for a
    /// name it minted, and otherwise raises `SelectRefusal`
    /// `node_not_held`: the node took its read with it, and a node
    /// handle cannot say which read that was. A `Var` that is no body
    /// output (a scalar, a selection, a pose) raises `SelectRefusal`
    /// `read_not_an_output`: there is no table to ask. A node with
    /// several body outputs is a `ValueError`, since which one is meant
    /// is the caller's to say.
    ///
    /// The other raise is the boundary one every name-taking door
    /// shares: text that is not a name at all is a `ValueError`. A
    /// well-formed name that denotes nothing is the `failed` verdict,
    /// which is the whole point of the door.
    fn resolve(
        &self,
        py: Python<'_>,
        at: super::doc::AtArg,
        name: &str,
    ) -> PyResult<super::resolve::Resolution> {
        let name = super::doc::name_from_text(name)?;
        let read = match at {
            super::doc::AtArg::Var(var) => var.0,
            // A node named alone is its one named output, as in every
            // name door (`Evaluation::at_var`): a revolve's body beside
            // its axis; a split's two halves are no one read. A node with
            // no value this run is read at its one output, if it has one,
            // and answers its standing.
            super::doc::AtArg::Node(node) => match self
                .inner
                .at_var(node.0)
                .or_else(|| self.doc.read_of_node(node.0))
            {
                Some(read) => read,
                None => {
                    match pncad::select::stranded_at(&self.doc, node.0, &name) {
                        Some(Ok(gone)) => {
                            return super::resolve::resolution(py, &gone, &self.doc, &self.inner);
                        }
                        Some(Err(refusal)) => {
                            return Err(super::select::resolve_refusal(py, &refusal, &self.doc));
                        }
                        None => {}
                    }
                    return Err(pyo3::exceptions::PyValueError::new_err(format!(
                        "{} defines several outputs; resolve in one of them, by `Doc.output`",
                        self.doc.spoken(node.0)
                    )));
                }
            },
        };
        let resolution = pncad::select::resolve(
            pncad::select::RunCtx {
                doc: &self.doc,
                eval: &self.inner,
            },
            read,
            &name,
        )
        .map_err(|refusal| super::select::resolve_refusal(py, &refusal, &self.doc))?;
        super::resolve::resolution(py, &resolution, &self.doc, &self.inner)
    }

    /// **What is under this ray?** — the nearest face hit across
    /// `targets`, resolved to a stable name, as of THIS evaluation.
    ///
    /// The fourth door onto a name, and it answers in the same opaque
    /// alphabet the other three speak: `PickHit.name` is a text
    /// `Node.fillet` takes unread, exactly as `select`'s answers are.
    ///
    /// `targets` are `NodePick`s — build them with `NodePick.build` or
    /// `NodePick.build_all`. There is no other spelling of a pick
    /// target here, and that is deliberate: a target whose
    /// `(node, body)` is not the pair its mesh was tessellated from
    /// answers a plausible, confidently WRONG name rather than an
    /// error, and a `NodePick` cannot be built that way.
    ///
    /// **A miss is `None`, and it is typed.** The ray hitting no
    /// offered triangle is not a failure, and a failure is never
    /// flattened into it. Ties are broken totally and documented
    /// kernel-side: the winner minimizes `(t, position in targets,
    /// triangle position)`, so a ray down a shared edge answers the
    /// same face every time.
    ///
    /// Raises `HitTestError`, typed: the pairing refusal
    /// (`evaluation_of_another_document`) for a target built from an
    /// evaluation of another document, checked before any target's
    /// standing because a twin recipe mints the same node ids; then
    /// the standing ladder for a target whose node this evaluation has
    /// no value for (`node_not_evaluated`, `node_failed`,
    /// `node_poisoned`); then the loud `unnamed` bug arm if the
    /// winning face inverts to no name.
    fn pick_face(
        &self,
        py: Python<'_>,
        targets: Vec<PyRef<'_, super::pick::NodePick>>,
        ray: &super::pick::Ray,
    ) -> PyResult<Option<super::pick::PickHit>> {
        super::pick::pick_face(py, self, targets, ray)
    }

    /// How many nodes ran their op this evaluation.
    ///
    /// With no `prior=`, this is every node that ran. With one, it is
    /// the changed cone — and `recomputed + reused` is what makes the
    /// two numbers EVIDENCE of reuse rather than a hint about it.
    ///
    /// The sum is the nodes that RAN OR WERE REUSED, which is the live
    /// node count only when every node produced a result. A POISONED
    /// node — one that never ran because an ancestor failed — is
    /// counted by neither, so on any refusal path the sum undershoots
    /// `len(order())` by exactly the number of poisonings. A node that
    /// ran and FAILED is counted here, in `recomputed`: it ran.
    #[getter]
    fn recomputed(&self) -> usize {
        self.inner.recomputed
    }

    /// How many nodes were served from `evaluate`'s `prior=` memo
    /// without re-running their op — zero when no prior was passed.
    #[getter]
    fn reused(&self) -> usize {
        self.inner.reused
    }

    /// How many REFERENCED documents this evaluation actually crossed
    /// the seam to evaluate.
    ///
    /// The sharing evidence for `evaluate`'s `resolver=`: N instances
    /// of one part count 1, because the part is evaluated once and its
    /// body reused; a part that instantiates a part counts here too, so
    /// the number is the whole run's seam traffic and not one level's.
    /// Zero without a resolver — nothing crosses.
    #[getter]
    fn part_evaluations(&self) -> usize {
        self.inner.part_evaluations
    }

    /// Export the single body `node` denotes as a STEP (AP214 Part 21)
    /// exchange-file string (the document-layer export door,
    /// `pncad::export::step_for_node` — one construction site
    /// for "which body does this node denote", shared with Rust).
    ///
    /// Accepts a `body` or non-empty `boolean` value; everything else
    /// raises a typed `ExportError`. The body is gated at rest first,
    /// so one the gate refuses raises `ExportError` tagged
    /// `body_invalid` rather than reaching the writer.
    ///
    /// Every `StepOptions` field is a keyword here, and each defaults
    /// to `None` meaning the Rust default — so the door narrows
    /// nothing and an omitted keyword is the same file a Rust caller
    /// gets from `StepOptions::default()`. Its options are a literal
    /// naming every field, held to `StepOptions` by the surface
    /// census's options roster, which is where that device is argued.
    ///
    /// `uncertainty` is the exported
    /// `UNCERTAINTY_MEASURE_WITH_UNIT` length; omitted, the writer
    /// reads the run's ambient tolerance, which is the ε the body was
    /// built under. A value that is not finite and strictly positive
    /// is the writer's refusal, not a check restated here.
    #[pyo3(signature = (
        node,
        product_name = None,
        timestamp = None,
        author = None,
        organization = None,
        originating_system = None,
        uncertainty = None,
    ))]
    #[allow(clippy::too_many_arguments)]
    fn step_string(
        &self,
        py: Python<'_>,
        node: &NodeId,
        product_name: Option<String>,
        timestamp: Option<String>,
        author: Option<String>,
        organization: Option<String>,
        originating_system: Option<String>,
        uncertainty: Option<Length>,
    ) -> PyResult<String> {
        let tol = Tol::witness();
        let defaults = pncad::step_export::StepOptions::default();
        let options = pncad::step_export::StepOptions {
            product_name: product_name.unwrap_or(defaults.product_name),
            timestamp: timestamp.unwrap_or(defaults.timestamp),
            author: author.unwrap_or(defaults.author),
            organization: organization.unwrap_or(defaults.organization),
            originating_system: originating_system.unwrap_or(defaults.originating_system),
            uncertainty_m: uncertainty.map(|u| u.0.meters()).or(defaults.uncertainty_m),
        };
        pncad::export::step_for_node(&self.inner, node.0, &options, tol)
            .map_err(|err| export_err(py, *node, &err, &self.doc))
    }

    fn __repr__(&self) -> String {
        format!("Evaluation({} nodes)", self.inner.order.len())
    }
}

/// Raise `ExportError` with variant `compose_refused` for a read-out
/// whose body the kernel would not compose into its space
/// ([`pncad::export::body_in_space`]): the same arm `export_step` raises
/// for the same refusal, `node` the value's.
pub(crate) fn compose_err(py: Python<'_>, node: NodeId, err: &topo::TransformError) -> PyErr {
    let node_obj = match node.into_pyobject(py) {
        Ok(bound) => bound.unbind().into_any(),
        Err(failed) => return failed,
    };
    let fields: Vec<(&str, Py<PyAny>)> = vec![
        (
            "variant",
            PyString::new(py, "compose_refused").unbind().into_any(),
        ),
        ("node", node_obj),
        ("through", py.None().into_any()),
        ("kind", py.None().into_any()),
    ];
    typed_err(
        py,
        ErrorClass::Export,
        format!("the body could not be composed into its space: {err}"),
        &fields,
    )
}

/// Raise `ExportError` mirroring the Rust door's refusal: `variant`
/// is the arm's stable tag, `node` rides along, a poisoning adds
/// `through`, and a wrong-kind value adds `kind`. The message is the
/// door's own
/// sentence, its nodes spoken from the evaluated document.
fn export_err(
    py: Python<'_>,
    node: NodeId,
    err: &pncad::export::ExportError,
    doc: &d::ProfileDoc,
) -> PyErr {
    use pncad::export::ExportError as E;
    let node_obj = match node.into_pyobject(py) {
        Ok(bound) => bound.unbind().into_any(),
        Err(failed) => return failed,
    };
    // `through`/`kind` are ALWAYS present (`None` where inapplicable)
    // so stub-guided reads cannot `AttributeError`.
    let mut fields: Vec<(&str, Py<PyAny>)> = vec![
        (
            "variant",
            PyString::new(py, export_error_tag(err)).unbind().into_any(),
        ),
        ("node", node_obj),
        ("through", py.None().into_any()),
        ("kind", py.None().into_any()),
    ];
    match err {
        E::Standing(standing) => {
            let [_, through] = super::standing_fields(py, *standing);
            fields[2] = ("through", through);
        }
        E::NotABody { kind, .. } => {
            fields[3] = ("kind", PyString::new(py, kind).unbind().into_any());
        }
        // `Product` is the WHOLE-DOCUMENT door's refusal: it names
        // the world's placements, not this call's node, so it adds no field
        // here. The arm is spelled out because the match
        // is exhaustive on purpose — the tripwire, not a wildcard.
        E::EmptyBoolean { .. }
        | E::BodyInvalid { .. }
        | E::Compose(_)
        | E::Step(_)
        | E::Product(_) => {}
    }
    typed_err(py, ErrorClass::Export, err.spoken(doc), &fields)
}

/// **A boundary-graph census**: what one region contributes to the
/// body, in faces, edges and vertices.
///
/// Two of these ride every [`StructureNormalization`] — the counts the
/// file states for the region, and the counts the mint left it with —
/// and their difference is the mapping a reader needs to reconcile the
/// file's numbers with the imported body's.
#[pyclass(frozen, module = "pncad", skip_from_py_object)]
#[derive(Clone, Copy)]
pub(crate) struct FaceCensus {
    /// Faces.
    #[pyo3(get)]
    faces: usize,
    /// Edges.
    #[pyo3(get)]
    edges: usize,
    /// Vertices.
    #[pyo3(get)]
    vertices: usize,
}

#[pymethods]
impl FaceCensus {
    fn __eq__(&self, other: &Self) -> bool {
        (self.faces, self.edges, self.vertices) == (other.faces, other.edges, other.vertices)
    }

    fn __repr__(&self) -> String {
        format!(
            "FaceCensus(faces={}, edges={}, vertices={})",
            self.faces, self.edges, self.vertices
        )
    }
}

/// **One re-minted boundary graph**, reported as data: the file's
/// locus was fully explained and adopted, but its tessellation is not
/// representable, so the kernel cut the same surface into its own
/// faces, edges and vertices and says which.
///
/// Volume and validity are exact either way; what changed is only the
/// cut. `file_census` and `kernel_census` are that change, counted.
///
/// `kind` is the stable word for what was re-minted, and the payload
/// of each arm that carries one arrives beside it, `None` on the
/// others: `promoted_to` is which analytic surface kind certified and
/// `residual` is the certified deviation that let it
/// (`surface_promotion`), and `singularity` is where the file put the
/// pole or apex import made a vertex (`chart_singularity_vertex`).
/// Present on every row, so a read never raises and a caller never has
/// to branch on `kind` first.
#[pyclass(frozen, module = "pncad", skip_from_py_object)]
#[derive(Clone, Copy)]
pub(crate) struct StructureNormalization(pncad::step_import::StructureNormalization);

#[pymethods]
impl StructureNormalization {
    /// The `ADVANCED_FACE` entity instance the file states.
    #[getter]
    fn face(&self) -> u64 {
        self.0.face
    }

    /// What was re-minted, as a stable word.
    #[getter]
    fn kind(&self) -> &'static str {
        normalization_kind_tag(&self.0.kind)
    }

    /// Which analytic surface kind certified — on `surface_promotion`
    /// alone.
    #[getter]
    fn promoted_to(&self) -> Option<&'static str> {
        match &self.0.kind {
            pncad::step_import::NormalizationKind::SurfacePromotion { to, .. } => {
                Some(promoted_kind_tag(to))
            }
            _ => None,
        }
    }

    /// The certified residual sup (metres): the patch's worst
    /// deviation from the surface it was promoted to. On
    /// `surface_promotion` alone.
    #[getter]
    fn residual(&self) -> Option<f64> {
        match &self.0.kind {
            pncad::step_import::NormalizationKind::SurfacePromotion { residual, .. } => {
                Some(*residual)
            }
            _ => None,
        }
    }

    /// Where the file put the chart singularity import made a vertex —
    /// on `chart_singularity_vertex` alone.
    #[getter]
    fn singularity(&self) -> Option<&'static str> {
        match &self.0.kind {
            pncad::step_import::NormalizationKind::ChartSingularityVertex { found } => {
                Some(singularity_found_tag(found))
            }
            _ => None,
        }
    }

    /// The census the file states for the region.
    #[getter]
    fn file_census(&self) -> FaceCensus {
        census(self.0.file_census)
    }

    /// The census as THIS mint left the region — a mint-event value,
    /// not an at-rest one: a later normalization may split an edge the
    /// region shares, and this record is not revised. The records'
    /// deltas sum to the body's totals; per-face at-rest counts are the
    /// body's to answer.
    #[getter]
    fn kernel_census(&self) -> FaceCensus {
        census(self.0.kernel_census)
    }

    fn __repr__(&self) -> String {
        format!(
            "StructureNormalization(face={}, kind={})",
            self.0.face,
            normalization_kind_tag(&self.0.kind)
        )
    }
}

/// **One promoted curve carrier**: the file stated a NURBS carrier
/// whose deviation from an analytic curve certified at the import's
/// tolerance, so the edge adopted on the analytic carrier.
///
/// It re-mints nothing — the boundary graph is untouched — which is
/// why it carries no census where a [`StructureNormalization`] carries
/// two, and why its key is a curve entity rather than a face.
#[pyclass(frozen, module = "pncad", skip_from_py_object)]
#[derive(Clone, Copy)]
pub(crate) struct CurvePromotion(pncad::step_import::CurvePromotion);

#[pymethods]
impl CurvePromotion {
    /// The curve entity instance the file states — the `EDGE_CURVE`'s
    /// carrier, not an edge that uses it: one carrier may serve
    /// several edges and the promotion is the carrier's.
    #[getter]
    fn curve(&self) -> u64 {
        self.0.curve
    }

    /// The analytic kind that certified, as a stable word.
    #[getter]
    fn kind(&self) -> &'static str {
        promoted_curve_kind_tag(&self.0.kind)
    }

    /// The certified residual sup (metres): the carrier's worst
    /// deviation from the promoted curve over its whole domain.
    #[getter]
    fn residual(&self) -> f64 {
        self.0.residual
    }

    fn __repr__(&self) -> String {
        format!(
            "CurvePromotion(curve={}, kind={})",
            self.0.curve,
            promoted_curve_kind_tag(&self.0.kind)
        )
    }
}

/// **One materialized assembly instance** — what the file said about
/// one solid of the imported body.
///
/// An assembly states N occurrences of M component representations,
/// and import materializes each occurrence as its own solid; one of
/// these travels with each, in `body.solids()` order. Flattening is
/// the right evaluation product and it is not forgetting: this is the
/// association a later import-as-assembly door rebuilds.
///
/// Every entity field names a real record in the file, or is `None`
/// because the file states no assembly — a file with no assembly
/// vocabulary still gets one row per solid, so a caller never has to
/// ask whether the record exists.
#[pyclass(frozen, module = "pncad", skip_from_py_object)]
#[derive(Clone, Copy)]
pub(crate) struct PlacedInstance(pncad::step_import::PlacedInstance);

#[pymethods]
impl PlacedInstance {
    /// Which solid of the imported body this row describes, in
    /// `Body`'s own solid order.
    #[getter]
    fn index(&self) -> usize {
        self.0.index
    }

    /// The `MANIFOLD_SOLID_BREP` this instance is a copy of. Repeats
    /// across one component's several occurrences: that repetition IS
    /// the instancing.
    #[getter]
    fn solid(&self) -> u64 {
        self.0.solid
    }

    /// The shape representation that names `solid`.
    #[getter]
    fn component(&self) -> u64 {
        self.0.component
    }

    /// The `NEXT_ASSEMBLY_USAGE_OCCURRENCE` this instance is, where
    /// the file links one.
    #[getter]
    fn occurrence(&self) -> Option<u64> {
        self.0.occurrence
    }

    /// The `REPRESENTATION_RELATIONSHIP` complex stating the placement.
    #[getter]
    fn relationship(&self) -> Option<u64> {
        self.0.relationship
    }

    /// The `ITEM_DEFINED_TRANSFORMATION` the placement was read from.
    #[getter]
    fn transform(&self) -> Option<u64> {
        self.0.transform
    }

    /// The rigid map actually APPLIED to this copy, as the placement
    /// value the rest of this surface uses.
    ///
    /// `None` is the identity — the file stated a placement that is
    /// the identity at the import's tolerance, or stated none. It is
    /// left as `None` rather than materialized: the record says
    /// nothing moved, and an identity `Frame` here would read as a map
    /// the file chose.
    #[getter]
    fn placement(&self) -> Option<super::place::Frame> {
        self.0
            .placement
            .map(|a| super::place::Frame(d::Frame::from_affine(a)))
    }

    fn __repr__(&self) -> String {
        format!(
            "PlacedInstance(index={}, solid={}, component={})",
            self.0.index, self.0.solid, self.0.component
        )
    }
}

fn census(c: pncad::step_import::FaceCensus) -> FaceCensus {
    FaceCensus {
        faces: c.faces,
        edges: c.edges,
        vertices: c.vertices,
    }
}

/// **What a successful STEP import produced** — the body, the gate's
/// own certificate of it, and the record of what the adoption changed
/// about the file.
///
/// The body is a `Body` handle like any other and gains nothing from
/// arriving this way; the report is what is new. `measure(settle)`
/// measures the body through the certificate the import's own at-rest
/// gate decided the body's orientation invariant on — **not a second
/// computation**: it continues the gate's rounds, where
/// `report.body.mass_properties(settle)` gates the body again and runs
/// its certified quadrature from round 0.
///
/// The three record lists are the adoption's own report, "as data,
/// never silently": `normalizations` is every boundary graph the
/// kernel re-minted, `promotions` every NURBS curve carrier adopted as
/// an analytic one, and `instances` the assembly record — one row per
/// solid, kept whether or not the file states an assembly.
#[pyclass(frozen, module = "pncad")]
pub(crate) struct ImportReport {
    /// The adopted body — Euler-built, certified, and tier-valid at
    /// rest, checked before it was handed out.
    #[pyo3(get)]
    body: Body,
    /// The body held with the at-rest gate's own certificate of it
    /// (`topo::CertifiedBody`): what `measure` measures through. `body`
    /// is its body, handed out as a handle like any other.
    certified: topo::CertifiedBody,
    /// The import's input tolerance in metres: the file's declared
    /// uncertainty, which is a separate quantity from the kernel's ε.
    #[pyo3(get)]
    eps_in: f64,
    normalizations: Vec<StructureNormalization>,
    promotions: Vec<CurvePromotion>,
    instances: Vec<PlacedInstance>,
}

#[pymethods]
impl ImportReport {
    /// The body measured under `settle` through the at-rest gate's own
    /// certificate of it, continuing the rounds the gate took.
    ///
    /// # Errors
    ///
    /// The measurement refusal `validate_geometric_measured` raises.
    fn measure(&self, py: Python<'_>, settle: Settle) -> PyResult<MassMeasurement> {
        self.certified
            .measure(settle.0)
            .map(MassMeasurement::from)
            .map_err(|err| measurement_err(py, &err))
    }

    /// Every boundary graph the adoption re-minted, in resolution
    /// order — empty for a file the kernel represents as stated.
    #[getter]
    fn normalizations(&self) -> Vec<StructureNormalization> {
        self.normalizations.clone()
    }

    /// Every NURBS curve carrier adopted as an analytic curve, by
    /// ascending curve entity id — empty for a file whose carriers are
    /// all stated analytically or all stay NURBS.
    #[getter]
    fn promotions(&self) -> Vec<CurvePromotion> {
        self.promotions.clone()
    }

    /// The assembly record: one row per solid of `body`, in its solid
    /// order.
    #[getter]
    fn instances(&self) -> Vec<PlacedInstance> {
        self.instances.clone()
    }

    fn __repr__(&self) -> String {
        format!(
            "ImportReport({} normalization(s), {} promotion(s), {} instance(s))",
            self.normalizations.len(),
            self.promotions.len(),
            self.instances.len()
        )
    }
}

/// Parse a STEP text with the kernel's own importer and adopt its
/// solid, answering the whole [`ImportReport`]: the body, the gate's
/// own certificate of it, and the record of what the adoption changed.
///
/// The report rather than the body alone is what makes the natural
/// journey — import a file, then ask what it encloses — measure the
/// solid once. The importer's gate has already run the certified
/// quadrature to decide the body's orientation invariant, so
/// `report.measure(settle)` continues that quadrature, where
/// `body.mass_properties(settle)` gates and measures the body over again.
///
/// `eps_in` overrides the file's declared
/// `UNCERTAINTY_MEASURE_WITH_UNIT` as the import's input tolerance —
/// the reading end of the ε `Evaluation.step_string`'s `uncertainty`
/// writes. Omitted, the file's own declaration is read, which is what
/// the Rust default says; a value that is not finite and strictly
/// positive is the importer's own `invalid_eps_override` refusal, not
/// a check restated here.
///
/// Its options are a literal naming every field, held to
/// `ImportOptions` by the surface census's options roster.
#[pyfunction]
#[pyo3(signature = (text, *, eps_in = None))]
pub(crate) fn import_step(
    py: Python<'_>,
    text: &str,
    eps_in: Option<Length>,
) -> PyResult<ImportReport> {
    let tol = Tol::witness();
    let defaults = pncad::step_import::ImportOptions::default();
    let options = pncad::step_import::ImportOptions {
        eps_in: eps_in.map(|e| e.0.meters()).or(defaults.eps_in),
        // This door asks for no chart-coherence examination, because
        // `ImportReport` has no field to report one on: asking would
        // measure the body and drop the measurement. Written as the
        // literal rather than read off `defaults`, so this line
        // ENFORCES the sentence above it — a kernel-side default that
        // flipped one crate away would otherwise turn the examination
        // on here silently, and the comment would go quietly false.
        // The surface census carries the decision as a `NotBound` row.
        examine_chart_coherence: false,
    };
    match pncad::step_import::import_step(text, &options, tol) {
        Ok(pncad::step_import::StepImport::Solid {
            body,
            eps_in,
            normalizations,
            curve_promotions,
            instances,
            // `None` by construction: the options above ask for no
            // examination, and `None` means NOT ASKED — which is the
            // one thing an empty report would not say.
            coherence: _,
            contacts,
            // The rows `contacts` cite: the body carries its records,
            // and the rows are the import's own decisions, which no
            // document reads through this door.
            coincidences: _,
        }) => Ok(ImportReport {
            body: Body::recorded(Arc::new(body.body().clone()), Arc::new(contacts)),
            certified: body,
            eps_in,
            normalizations: normalizations
                .into_iter()
                .map(StructureNormalization)
                .collect(),
            promotions: curve_promotions.into_iter().map(CurvePromotion).collect(),
            instances: instances.into_iter().map(PlacedInstance).collect(),
        }),
        // Not a refusal variant: the import SUCCEEDED and produced
        // the other arm of `StepImport`, which this door does not
        // adopt. Its tag is the arm's name and shares the namespace
        // with `step_import_error_tag`'s, which does not mint it; the
        // word is `crate::tags::STEP_IMPORT_WIREFRAME`, where the tag
        // inventory reads it. A THIRD arm of the kernel's success enum
        // stops this match compiling, and owes a word in that file.
        Ok(pncad::step_import::StepImport::Wireframe { .. }) => Err(typed_err(
            py,
            ErrorClass::StepImport,
            "the file parsed to a wireframe, not a solid",
            &[
                (
                    "variant",
                    PyString::new(py, crate::tags::STEP_IMPORT_WIREFRAME)
                        .unbind()
                        .into_any(),
                ),
                ("promoted_kind", py.None()),
            ],
        )),
        // The tag is the importer's own, through `crate::tags`. Every
        // arm of `StepImportError` is reachable here, and the entity
        // id and line that would tell them apart live in the message
        // prose — so one literal for every arm would make them
        // indistinguishable to a caller.
        //
        // `promoted_kind` is the one arm's payload discriminant,
        // beside the tag rather than in place of it: the word
        // `recognition_ambiguous` names the condition, and which
        // analytic kind's estimator declined is the second question,
        // with its own recourse. `None` on every other arm, which is
        // this surface's every-attribute-always-present rule.
        Err(err) => Err(typed_err(
            py,
            ErrorClass::StepImport,
            err.to_string(),
            &[
                (
                    "variant",
                    PyString::new(py, step_import_error_tag(&err))
                        .unbind()
                        .into_any(),
                ),
                (
                    "promoted_kind",
                    match &err {
                        pncad::step_import::StepImportError::RecognitionAmbiguous {
                            kind, ..
                        } => PyString::new(py, promoted_kind_tag(kind))
                            .unbind()
                            .into_any(),
                        _ => py.None(),
                    },
                ),
            ],
        )),
    }
}

/// **The cooperative cancel token (spec D5)** — what a caller holds
/// to stop an evaluation it has already launched.
///
/// Shared, not copied: the token is a handle onto one flag, so the
/// thread that calls [`Self::cancel`] and the thread inside
/// [`super::value::evaluate`] are looking at the same bit. That is
/// why this class is FROZEN and `cancel` takes no mutable borrow —
/// setting the flag is not a mutation of the Python object, and a
/// token that had to be borrowed mutably could not be held by a
/// running evaluation at all.
///
/// **Cooperative, between nodes and between a family's members.** The
/// kernel reads the flag between nodes (or between levels on the
/// parallel schedule) and before each member of a family, and never
/// inside a kernel op, so `cancel()` does not abort the boolean or the
/// tessellation already in flight; it stops the run before the next
/// node or member starts. The token is one-way — there is no `reset`,
/// because the flag a canceled run observed cannot be un-observed and
/// a reusable token would let two runs disagree about what it meant.
/// Mint a new one per launch, which is what `evaluate` does when no
/// `cancel=` is passed.
///
/// What a canceled run ANSWERS is `Evaluation.canceled`, and the
/// answer is a partial result rather than a raise: see that property.
#[pyclass(frozen, module = "pncad")]
#[derive(Default)]
pub(crate) struct CancelToken {
    inner: d::CancelToken,
}

#[pymethods]
impl CancelToken {
    /// A fresh, un-canceled token.
    #[new]
    fn new() -> Self {
        Self::default()
    }

    /// **Request cancelation** — any thread, any time, idempotent.
    ///
    /// Setting the flag before the run starts is legal and is the
    /// deterministic case: the evaluation checks the token before its
    /// first node, so a pre-canceled token yields a run with the full
    /// `order()`, no results at all, and `canceled` true.
    fn cancel(&self) {
        self.inner.cancel();
    }

    /// Whether cancelation has been requested on THIS token.
    ///
    /// A property of the token, not of any run: it says the flag is
    /// set, never that an evaluation observed it. `Evaluation.canceled`
    /// is the run's own answer, and the two differ exactly when a run
    /// finished before the flag was set.
    #[getter]
    fn canceled(&self) -> bool {
        self.inner.is_canceled()
    }

    fn __repr__(&self) -> String {
        format!(
            "CancelToken(canceled={})",
            if self.canceled() { "True" } else { "False" }
        )
    }
}

impl CancelToken {
    /// The kernel token this handle carries — cloned, which shares
    /// the flag rather than copying it.
    pub(crate) fn token(&self) -> d::CancelToken {
        self.inner.clone()
    }
}

/// Evaluate a document, producing its per-node result DAG.
///
/// Total: evaluation never raises. Individual nodes may still have
/// failed — ask the returned object.
///
/// `resolver` is the DOCUMENT SEAM: what an `InstantiatePart` node
/// reaches the document it pins through. A `Workspace` IS a resolver
/// (`pncad::workspace`'s own impl), so the store is passed as itself.
/// `None` — the default — is a kernel-only evaluation, in which every
/// instantiate node refuses typed (`EvaluationError`, `kind ==
/// "part_no_resolver"`) rather than pretending a part is empty. The
/// parameter carries the kernel's ROLE name: resolving a reference is
/// the capability evaluation needs, and a workspace is today's only
/// thing that has it.
///
/// `prior` is the MEMO: a node whose content and naming keys match
/// its result in `prior` reuses that value instead of re-running its
/// op, so only the changed cone costs anything. Reuse is not a claim
/// the caller has to take on trust — `Evaluation.reused` and
/// `Evaluation.recomputed` count it, node for node.
///
/// The memo is PER DOCUMENT and node-id-keyed: the lookup finds
/// `prior`'s result for the SAME node id and then certifies it by
/// content. An evaluation of a different document is a legal prior
/// that reuses nothing — not because the ids miss, but because the
/// memo REFUSES it: an evaluation carries the id of the document it
/// was run on, and a prior of another document is dropped whole
/// before the schedule is built. Ids alone would not decide it: two
/// documents built from one recipe carry the SAME ids for the same
/// nodes, so every lookup would hit. The run stays total — every node
/// recomputed — and `Evaluation.reused` is 0. Use the prior
/// evaluation of THIS document.
///
/// **A memo hit is served WITHOUT re-running the seam's gates.** A
/// reused `InstantiatePart` node never asks the resolver, so the
/// AVAILABILITY refusals — `part_pin_mismatch`, `part_unresolved`,
/// and `part_no_resolver` with them — are raised only for nodes that
/// actually re-resolve. What the memo serves is what the document's
/// own `DocRef` PINS, certified by content key: it is never a
/// different part, and it is not re-checked against the store. Two
/// consequences, and both are real:
///
/// * Edit a part on disk and re-evaluate with a prior, and the run
///   succeeds serving the previously pinned body where a run without
///   the prior refuses `part_pin_mismatch`. Relative to the STORE
///   that is a stale answer; relative to the DOCUMENT it is the
///   pinned one.
/// * "A pin that moved refuses, and is never silently retargeted"
///   therefore holds for evaluations that cross the seam. It is not
///   weakened for the ones that do — nothing is retargeted either
///   way — but it is not RE-ASSERTED by a run that never asks.
///
/// Pass no prior when the question is "does this document still
/// resolve against the store as it stands".
///
/// `cancel` is the COOPERATIVE STOP: a [`CancelToken`] the caller
/// keeps a handle on, checked between nodes and a family's members,
/// so a run already under way can be abandoned. Omitted, this door
/// mints a fresh token nobody holds — which is exactly what it did
/// before the keyword existed, and is why an evaluation without one is
/// never canceled.
/// A canceled run is a PARTIAL ANSWER and not a raise: it returns
/// normally with `Evaluation.canceled` true, the full `order()`, and
/// results for the completed prefix only.
///
/// **The GIL is released for the kernel run**, and that is what makes
/// the token reachable rather than decorative: the flag is set by
/// ANOTHER Python thread while this one is inside the evaluation, and
/// a thread that cannot run cannot set it. `doc` is borrowed for the
/// whole call, so a concurrent MUTATION of the same document —
/// `Doc.insert`, `Doc.apply`, any door needing `&mut` — raises
/// pyo3's own `RuntimeError("Already borrowed")` instead of editing
/// a recipe out from under a running evaluation. Measured, not
/// reasoned: `tests/test_cancellation.py` executes it.
///
/// `resolver` is the one `EvalOptions` field this door takes. The
/// other six are not one kind of thing and the surface census's
/// options roster says which each is — three are switches the kernel
/// documents as answer-preserving, three change answers and are
/// unreachable at this door's scalar rather than harmless. Its
/// options are a literal naming every field, no `..default()` tail,
/// which is what holds the door to that roster.
#[pyfunction]
#[pyo3(signature = (doc, *, resolver=None, prior=None, cancel=None))]
pub(crate) fn evaluate(
    py: Python<'_>,
    doc: &super::doc::Doc,
    resolver: Option<&super::store::Workspace>,
    prior: Option<&Evaluation>,
    cancel: Option<&CancelToken>,
) -> Evaluation {
    let tol = Tol::witness();
    let defaults = d::EvalOptions::default();
    let opts = d::EvalOptions {
        epoch: defaults.epoch,
        parallel: defaults.parallel,
        boolean_sweep: defaults.boolean_sweep,
        resolver: resolver
            .map(super::store::Workspace::resolver)
            .or(defaults.resolver),
        param_box: defaults.param_box,
        seed: defaults.seed,
        symbols: defaults.symbols,
        pin: defaults.pin,
        computing_twist: defaults.computing_twist,
    };
    let token = cancel.map_or_else(d::CancelToken::new, CancelToken::token);
    let recipe = &doc.inner;
    let memo = prior.map(|p| &p.inner);
    let inner = py.detach(|| d::evaluate::<f64>(recipe, memo, &token, &opts, tol));
    Evaluation {
        inner,
        params: doc.inner.var_env::<f64>(),
        doc: Arc::new(doc.inner.clone()),
        product: crate::product_memo::ProductMemo::default(),
    }
}

/// **One coincidence a node decided from values** (D10), with what the
/// coincidence door decided about it.
///
/// Its cells cross as pairs, two of them, or two or more that share
/// one point for a `meet`: an entity cell as `(read, name)`,
/// the `Var` the deciding node took the entity in through, whose table
/// names it, and the name there, the same opaque text the materializers
/// answer with; a tool cell (the plane a split cuts with) has no name
/// and crosses as `(read, None)`; and a profile's own piece crosses as
/// `(profile, piece)`, the profile's `NodeId` and the piece's text as a
/// step's pieces spell it. `origin` says which of the node's
/// decisions it is: `None` for the node's own operation, `("judged",
/// p, q)` for a union's or an intersect's judgement of the members at
/// places `p < q` in its list, one of its order-free rows, and
/// `("fold", k)` for the fold step that folds in the member at place
/// `k`. `rung` names the door's
/// rung that proved the row structural, `None` where none did, and then
/// `residual` says what separates the two constructions, or why a
/// cell's could not be read (`coincide::Unwalked`, said by its sentence
/// rather than crossed as a type: nothing a Python caller does branches
/// on which node the walk stopped at).
#[pyclass(frozen, module = "pncad", from_py_object)]
#[derive(Clone)]
pub(crate) struct Coincidence {
    cells: Vec<(Cell, Option<String>)>,
    origin: Option<Origin>,
    relation: &'static str,
    site: &'static str,
    rung: Option<&'static str>,
    residual: Option<String>,
}

impl Coincidence {
    /// The row and its proof, crossed.
    pub(crate) fn new(
        py: Python<'_>,
        row: &d::NamedCoincidence,
        proof: &d::Proof,
        doc: &d::ProfileDoc,
    ) -> PyResult<Self> {
        let cells = row
            .cells
            .iter()
            .map(|cell| match cell {
                d::NamedCell::Entity { input, name } => Ok((
                    Cell::Read(Var::of(doc, *input)),
                    Some(super::doc::name_text(py, name)?),
                )),
                d::NamedCell::Tool { input, .. } => Ok((Cell::Read(Var::of(doc, *input)), None)),
                d::NamedCell::Piece { profile, piece } => Ok((
                    Cell::Node(NodeId(*profile)),
                    Some(super::doc::piece_text(piece)?),
                )),
            })
            .collect::<PyResult<_>>()?;
        let (rung, residual) = match proof {
            d::Proof::Structural(rung) => (Some(crate::tags::coincidence_rung_tag(*rung)), None),
            d::Proof::Unproven { residual, .. } => (None, Some(residual.to_string())),
        };
        let origin = match row.origin {
            d::RowOrigin::Node => None,
            d::RowOrigin::Judged { pair: [p, q] } => Some(Origin::Judged("judged", p, q)),
            d::RowOrigin::Fold { step } => Some(Origin::Fold("fold", step)),
        };
        Ok(Self {
            cells,
            origin,
            relation: crate::tags::coincidence_relation_tag(row.relation),
            site: crate::tags::decision_site_tag(row.site),
            rung,
            residual,
        })
    }
}

/// Which of a node's decisions a [`Coincidence`] is, as it crosses.
#[derive(Clone, IntoPyObject)]
pub(crate) enum Origin {
    /// `("judged", p, q)`: the judgement of the members at `p < q`.
    Judged(&'static str, u32, u32),
    /// `("fold", k)`: the fold step folding in the member at `k`.
    Fold(&'static str, u32),
}

/// One cell of a [`Coincidence`] as it crosses: the read an entity or
/// a tool plane came in through, or the profile node a piece is of.
#[derive(Clone, IntoPyObject)]
pub(crate) enum Cell {
    /// The read whose table names the cell.
    Read(Var),
    /// The profile whose piece the cell is.
    Node(NodeId),
}

#[pymethods]
impl Coincidence {
    /// The cells decided: two, or for a `meet` the two or more that
    /// share one point, sorted. `(read, name)` for an entity, the
    /// `Var` whose table names it; `(read, None)` for a tool plane;
    /// `(profile, piece)` for a profile's own piece.
    #[getter]
    fn cells(&self) -> Vec<(Cell, Option<String>)> {
        self.cells.clone()
    }

    /// Which of the node's decisions it is: `None` for the node's own
    /// operation, `("judged", p, q)` for a union's or an intersect's
    /// judgement of its members at places `p < q`, and `("fold", k)`
    /// for the fold step folding in the member at place `k`.
    #[getter]
    fn origin(&self) -> Option<Origin> {
        self.origin.clone()
    }

    /// What was decided between them: `same_oriented`,
    /// `same_opposite`, `on_carrier`, `common_foot`, `tangent`, `cusp`,
    /// `surface_tangent`, `coaxial`, `same_radius`, `axes_meet`,
    /// `co_ruled`, or `meet`, the cells sharing one point. A
    /// `profile_junction` row is `tangent` or `cusp` between two carriers
    /// and `same_oriented` where its pieces continue one; a
    /// `tangent_witness` row is `surface_tangent`, claiming nothing of
    /// the materials.
    #[getter]
    fn relation(&self) -> &'static str {
        self.relation
    }

    /// Where it was decided: `plane_ladder`, `carrier_ladder`,
    /// `tangent_witness`, `coaxial_sphere`, `sphere_on_torus_axis`,
    /// `coaxial_cylinder_torus`, `coaxial_tori`, `crossing_cylinders`,
    /// `split_on`, `battery_turn`, `battery_joint`, `battery_support_axis`,
    /// `profile_junction`, `vertex_fusion` or `census_at_rest`.
    #[getter]
    fn site(&self) -> &'static str {
        self.site
    }

    /// The door's rung that proved it structural (`same_construction`,
    /// `carrier_verdict` or `margin_identity`), or `None`: the door does
    /// not prove it structural.
    #[getter]
    fn rung(&self) -> Option<&'static str> {
        self.rung
    }

    /// What separates the two constructions, where no rung proved it.
    #[getter]
    fn residual(&self) -> Option<String> {
        self.residual.clone()
    }

    fn __repr__(&self) -> String {
        format!(
            "Coincidence({}, {}, {})",
            self.relation,
            self.site,
            self.rung.unwrap_or("unproven")
        )
    }
}

/// Register the value surface on the module.
pub(crate) fn register(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<CancelToken>()?;
    m.add_class::<Evaluation>()?;
    m.add_class::<Coincidence>()?;
    m.add_class::<Value>()?;
    m.add_class::<Body>()?;
    m.add_class::<MassProperties>()?;
    m.add_class::<Settle>()?;
    m.add_class::<MassMeasurement>()?;
    m.add_class::<ImportReport>()?;
    m.add_class::<StructureNormalization>()?;
    m.add_class::<CurvePromotion>()?;
    m.add_class::<PlacedInstance>()?;
    m.add_class::<FaceCensus>()?;
    m.add_class::<ValidationFinding>()?;
    m.add_class::<Measurement>()?;
    m.add_class::<Verdict>()?;
    m.add_function(wrap_pyfunction!(evaluate, m)?)?;
    m.add_function(wrap_pyfunction!(import_step, m)?)?;
    Ok(())
}
