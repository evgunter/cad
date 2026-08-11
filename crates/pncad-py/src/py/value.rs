//! Per-node evaluation results: `Evaluation`, `Value`, `Body`.
//!
//! # The ValuePayload exposure inventory (a reported FORK)
//!
//! `ValuePayload` has seven variants. The bindings project them as:
//!
//! | variant        | exposure |
//! |----------------|----------|
//! | `Body`         | full — opaque handle + `mass_properties` / `validate` doors |
//! | `Boolean`      | full — unwraps to a `Body`, or `None` when empty |
//! | `Split`        | full — `above` / `below` as optional bodies |
//! | `Instances`    | full — a list of bodies |
//! | `Datum`        | full — typed plane / axis / point with `Length` coordinates |
//! | `Profile`      | KIND ONLY — LQ4 forbids shipping sketch geometry to Python before the v2 switch |
//! | `Declarations` | KIND ONLY — SEL1/U5 owns the naming projection; deferred, not blocked |
//!
//! The two kind-only rows are SCOPE decisions, not capability limits.
//! Being precise about which, because the distinction is load-bearing:
//!
//! * `ValidatedProfile::plane()`/`loops()` DO exist and `profile` is
//!   wholesale re-exported — this very module's sibling uses
//!   `pncad::profile` to build sketches. Projecting a profile back to
//!   Python is therefore perfectly possible; it is **ruled out**, by
//!   LQ4's "Python never ships the opaque-profile intermediate
//!   state". Sketch read-back belongs to the unit that binds the v2
//!   program representation, after SWITCH-E.
//! * `StableName` is likewise prelude-curated with public fields, so
//!   Declarations is reachable too. It is deferred because the naming
//!   /selection projection is SEL1's subject, and binding a
//!   provisional shape here would fork it.
//!
//! Neither row is "no accessor exists"; an earlier revision of this
//! comment claimed that, and it was false.

use std::sync::Arc;

use pyo3::prelude::*;
use pyo3::types::PyString;

use crate::errors::ErrorClass;
use crate::py::quantity::Length;
use crate::py::{doc::NodeId, typed_err};
use crate::tags::{export_error_tag, node_error_tag};
use pncad::document as d;
use pncad::topo;

/// Raise `EvaluationError` with a stable `reason` tag.
///
/// `kind` and `through` are ALWAYS present on the exception — `None`
/// where the reason has no failing kind or poisoning ancestor — so
/// stub-guided code can read them without an `AttributeError` trap
/// (the R1/R2 NOTE on over-promising stubs).
fn eval_err(py: Python<'_>, message: impl Into<String>, reason: &str, node: NodeId) -> PyErr {
    let node = match node.into_pyobject(py) {
        Ok(bound) => bound.unbind().into_any(),
        // A `#[pyclass]` conversion fails as a `PyErr` already —
        // surface it as the raise rather than losing it.
        Err(failed) => return failed,
    };
    typed_err(
        py,
        ErrorClass::Evaluation,
        message,
        &[
            ("reason", PyString::new(py, reason).unbind().into_any()),
            ("node", node),
            ("kind", py.None().into_any()),
            ("through", py.None().into_any()),
        ],
    )
}

/// Raise `EvaluationError` for a node that ITSELF failed: the payload
/// is the `NodeErrorKind`'s stable tag plus the node id; the message
/// is the kernel error's own `Display` prose (F6, reopened on
/// review — never a `Debug` dump).
fn node_failure(py: Python<'_>, node: NodeId, error: &d::NodeError) -> PyErr {
    let node_obj = match node.into_pyobject(py) {
        Ok(bound) => bound.unbind().into_any(),
        Err(failed) => return failed,
    };
    typed_err(
        py,
        ErrorClass::Evaluation,
        error.to_string(),
        &[
            (
                "reason",
                PyString::new(py, "node_failed").unbind().into_any(),
            ),
            ("node", node_obj),
            (
                "kind",
                PyString::new(py, node_error_tag(&error.kind))
                    .unbind()
                    .into_any(),
            ),
            ("through", py.None().into_any()),
        ],
    )
}

/// Raise `EvaluationError` for a POISONED node: `through` names the
/// nearest failed ancestor, `kind` tags its root cause (present
/// whenever the evaluation's own invariant holds — fail-honest, so a
/// broken hop yields no `kind` rather than a wrong one).
fn poisoning(py: Python<'_>, node: NodeId, through: NodeId, root: Option<&d::NodeError>) -> PyErr {
    let objs = (node.into_pyobject(py), through.into_pyobject(py));
    let (node_obj, through_obj) = match objs {
        (Ok(n), Ok(t)) => (n.unbind().into_any(), t.unbind().into_any()),
        (Err(failed), _) | (_, Err(failed)) => return failed,
    };
    let mut fields: Vec<(&str, Py<PyAny>)> = vec![
        ("reason", PyString::new(py, "poisoned").unbind().into_any()),
        ("node", node_obj),
        ("through", through_obj),
    ];
    // The message is the root cause's `Display` prose (F6): the node
    // never ran, so the honest sentence names the ancestor's problem.
    let message = match root {
        Some(error) => {
            fields.push((
                "kind",
                PyString::new(py, node_error_tag(&error.kind))
                    .unbind()
                    .into_any(),
            ));
            format!("never ran — poisoned by failed ancestor: {error}")
        }
        None => {
            fields.push(("kind", py.None().into_any()));
            format!("never ran — poisoned through node {}", through.0.0)
        }
    };
    typed_err(py, ErrorClass::Evaluation, message, &fields)
}

/// Bulk mass properties of a body, in canonical units.
///
/// Volume and area are `m³` and `m²` — dimensions OUTSIDE D6's closed
/// `{Length, Angle, Count}` set, so they cross as plain floats in
/// canonical units rather than as invented quantity types.
#[pyclass(frozen, module = "pncad")]
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

#[pymethods]
impl MassProperties {
    fn __repr__(&self) -> String {
        format!(
            "MassProperties(volume={} m^3, surface_area={} m^2)",
            self.volume, self.surface_area
        )
    }
}

/// A solid body — an OPAQUE handle.
///
/// §L3 forbids arena keys crossing; a body crosses as a handle whose
/// interior is reachable only through curated doors.
#[pyclass(frozen, module = "pncad", from_py_object)]
#[derive(Clone)]
pub(crate) struct Body {
    pub(crate) inner: Arc<topo::Body<f64>>,
}

#[pymethods]
impl Body {
    /// Volume, area, and their certified pads.
    fn mass_properties(&self, py: Python<'_>) -> PyResult<MassProperties> {
        let props = topo::mass_properties(&self.inner).map_err(|err| {
            typed_err(
                py,
                ErrorClass::Validation,
                format!("{err:?}"),
                &[(
                    "reason",
                    PyString::new(py, "mass_properties_failed")
                        .unbind()
                        .into_any(),
                )],
            )
        })?;
        Ok(MassProperties {
            volume: props.volume,
            surface_area: props.surface_area,
            volume_pad: props.volume_pad,
            area_pad: props.area_pad,
        })
    }

    /// Full validation. Raises `ValidationError` listing the failures.
    fn validate(&self, py: Python<'_>) -> PyResult<()> {
        self.run_validator(py, "validate", topo::validate(&self.inner))
    }

    /// Closure validation only.
    fn validate_closed(&self, py: Python<'_>) -> PyResult<()> {
        self.run_validator(py, "validate_closed", topo::validate_closed(&self.inner))
    }

    /// Geometric validation only.
    fn validate_geometric(&self, py: Python<'_>) -> PyResult<()> {
        self.run_validator(
            py,
            "validate_geometric",
            topo::validate_geometric(&self.inner),
        )
    }
}

impl Body {
    /// Shared shape for the validator doors, which all return
    /// `Result<(), Vec<ValidationError>>`.
    ///
    /// `ValidationError` has no `Display` and no curated tag mapping,
    /// so the exception carries the failure COUNT as structured data
    /// and the `Debug` rendering as the human message. Per-variant
    /// tags are the same mechanical work `crate::tags` does for edits,
    /// deferred with the rest of the read-back surface.
    fn run_validator(
        &self,
        py: Python<'_>,
        door: &str,
        outcome: Result<(), Vec<topo::ValidationError>>,
    ) -> PyResult<()> {
        let Err(failures) = outcome else {
            return Ok(());
        };
        let count = failures.len().into_pyobject(py)?.unbind().into_any();
        Err(typed_err(
            py,
            ErrorClass::Validation,
            format!(
                "{door} reported {} failure(s): {failures:?}",
                failures.len()
            ),
            &[
                ("door", PyString::new(py, door).unbind().into_any()),
                ("failure_count", count),
            ],
        ))
    }
}

/// A datum: a construction plane, axis, or point.
#[pyclass(frozen, module = "pncad")]
pub(crate) struct Datum {
    /// `"plane"`, `"axis"`, or `"point"`.
    #[pyo3(get)]
    kind: &'static str,
    /// Plane/axis origin, or the point's position, as metres.
    #[pyo3(get)]
    origin: (Length, Length, Length),
    /// Plane normal or axis direction; `None` for a point.
    #[pyo3(get)]
    direction: Option<(f64, f64, f64)>,
}

#[pymethods]
impl Datum {
    fn __repr__(&self) -> String {
        format!("Datum({})", self.kind)
    }
}

/// Project a canonical-metre point into typed `Length`s.
fn lengths(p: pncad::geom_core::Point3<f64>) -> (Length, Length, Length) {
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
#[pyclass(frozen, module = "pncad")]
pub(crate) struct Value {
    payload: d::ValuePayload<f64>,
    node: NodeId,
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

    /// The single body this value denotes.
    ///
    /// Accepts `Body` and a non-empty `Boolean`; raises for every
    /// other kind, and for an empty Boolean.
    fn body(&self, py: Python<'_>) -> PyResult<Body> {
        match &self.payload {
            d::ValuePayload::Body(body) => Ok(Body {
                inner: Arc::clone(body),
            }),
            d::ValuePayload::Boolean(d::BooleanValue::Body { body, .. }) => Ok(Body {
                inner: Arc::clone(body),
            }),
            d::ValuePayload::Boolean(d::BooleanValue::Empty) => Err(eval_err(
                py,
                "the Boolean produced an empty result",
                "empty_boolean",
                self.node,
            )),
            other => Err(eval_err(
                py,
                format!("a `{}` value is not a body", other.kind_name()),
                "wrong_kind",
                self.node,
            )),
        }
    }

    /// Every body this value denotes: one for `Body`/`Boolean`, the
    /// whole list for `Instances`, both sides for a `Split`.
    fn bodies(&self) -> Vec<Body> {
        let wrap = |body: &Arc<topo::Body<f64>>| Body {
            inner: Arc::clone(body),
        };
        let side = |s: &d::SplitSide<f64>| match s {
            d::SplitSide::Body(body) => Some(wrap(body)),
            d::SplitSide::Empty => None,
        };
        match &self.payload {
            d::ValuePayload::Body(body) => vec![wrap(body)],
            d::ValuePayload::Boolean(d::BooleanValue::Body { body, .. }) => vec![wrap(body)],
            d::ValuePayload::Instances(bodies) => bodies.iter().map(wrap).collect(),
            d::ValuePayload::Split { above, below } => {
                [side(above), side(below)].into_iter().flatten().collect()
            }
            _ => Vec::new(),
        }
    }

    /// A split's two sides, `(above, below)`; `None` where empty.
    fn split(&self, py: Python<'_>) -> PyResult<(Option<Body>, Option<Body>)> {
        let side = |s: &d::SplitSide<f64>| match s {
            d::SplitSide::Body(body) => Some(Body {
                inner: Arc::clone(body),
            }),
            d::SplitSide::Empty => None,
        };
        match &self.payload {
            d::ValuePayload::Split { above, below } => Ok((side(above), side(below))),
            other => Err(eval_err(
                py,
                format!("a `{}` value is not a split", other.kind_name()),
                "wrong_kind",
                self.node,
            )),
        }
    }

    /// The datum this value denotes.
    fn datum(&self, py: Python<'_>) -> PyResult<Datum> {
        match &self.payload {
            d::ValuePayload::Datum(d::DatumValue::Plane { origin, normal }) => Ok(Datum {
                kind: "plane",
                origin: lengths(*origin),
                direction: Some((normal.x, normal.y, normal.z)),
            }),
            d::ValuePayload::Datum(d::DatumValue::Axis { origin, dir }) => Ok(Datum {
                kind: "axis",
                origin: lengths(*origin),
                direction: Some((dir.x, dir.y, dir.z)),
            }),
            d::ValuePayload::Datum(d::DatumValue::Point { position }) => Ok(Datum {
                kind: "point",
                origin: lengths(*position),
                direction: None,
            }),
            other => Err(eval_err(
                py,
                format!("a `{}` value is not a datum", other.kind_name()),
                "wrong_kind",
                self.node,
            )),
        }
    }

    fn __repr__(&self) -> String {
        format!("Value({})", self.payload.kind_name())
    }
}

/// The result of evaluating a document: the GQ2 per-node result DAG.
#[pyclass(frozen, module = "pncad")]
pub(crate) struct Evaluation {
    inner: d::Evaluation<f64>,
}

#[pymethods]
impl Evaluation {
    /// The node's successful value.
    ///
    /// A node that produced NO value raises with the REAL typed cause
    /// (LIB-DOORS F3; the U9S `no_value` placeholder is gone):
    /// `reason` is `"node_failed"` or `"poisoned"`, `kind` is the
    /// `NodeErrorKind`'s stable tag, a poisoning carries `through`,
    /// and the message renders the kernel's own `NodeError`.
    fn value(&self, py: Python<'_>, node: &NodeId) -> PyResult<Value> {
        match self.inner.result(node.0) {
            Some(d::NodeResult::Ok(node_value)) => Ok(Value {
                payload: node_value.payload.clone(),
                node: *node,
            }),
            Some(d::NodeResult::Failed(error)) => Err(node_failure(py, *node, error)),
            Some(d::NodeResult::Poisoned { through }) => {
                let root = self.inner.node_error(node.0);
                Err(poisoning(py, *node, NodeId(*through), root))
            }
            None => Err(eval_err(
                py,
                "no such node in the evaluated document",
                "unknown_node",
                *node,
            )),
        }
    }

    /// Whether the node produced a value.
    fn succeeded(&self, node: &NodeId) -> bool {
        self.inner.value(node.0).is_some()
    }

    /// The evaluation order.
    fn order(&self) -> Vec<NodeId> {
        self.inner.order.iter().copied().map(NodeId).collect()
    }

    /// **Every edge name of `node`'s output body, as of THIS
    /// evaluation** — the U7 materializer, crossing as text.
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
    /// `doc::name_text`), so narrowing the set is a SELECTOR's job and
    /// no selector crosses yet — the audit's G13.
    ///
    /// Empty when the node has no value, no name table, or no edges.
    /// The fillet node is what refuses an EMPTY selection, so the
    /// emptiness surfaces there rather than here.
    fn all_edges(&self, py: Python<'_>, node: &NodeId) -> PyResult<Vec<String>> {
        names(py, pncad::select::all_edges(&self.inner, node.0))
    }

    /// Every FACE name of `node`'s output, [`Self::all_edges`]'s
    /// sibling — same contract, same alphabet.
    fn all_faces(&self, py: Python<'_>, node: &NodeId) -> PyResult<Vec<String>> {
        names(py, pncad::select::all_faces(&self.inner, node.0))
    }

    /// Every VERTEX name of `node`'s output, same contract.
    fn all_vertices(&self, py: Python<'_>, node: &NodeId) -> PyResult<Vec<String>> {
        names(py, pncad::select::all_vertices(&self.inner, node.0))
    }

    /// Every BODY name `node`'s evaluation carries, same contract.
    /// Usually one row; a split's two halves are the plural case.
    fn all_bodies(&self, py: Python<'_>, node: &NodeId) -> PyResult<Vec<String>> {
        names(py, pncad::select::all_bodies(&self.inner, node.0))
    }

    /// How many nodes were recomputed rather than reused from the memo.
    #[getter]
    fn recomputed(&self) -> usize {
        self.inner.recomputed
    }

    /// How many nodes were served from the memo.
    #[getter]
    fn reused(&self) -> usize {
        self.inner.reused
    }

    /// Export the single body `node` denotes as a STEP (AP214 Part 21)
    /// exchange-file string (LIB-DOORS F2: the document-layer export
    /// door, `pncad::export::step_for_node` — one construction site
    /// for "which body does this node denote", shared with Rust).
    ///
    /// Accepts a `body` or non-empty `boolean` value; everything else
    /// raises a typed `ExportError`.
    #[pyo3(signature = (node, product_name=None))]
    fn step_string(
        &self,
        py: Python<'_>,
        node: &NodeId,
        product_name: Option<String>,
    ) -> PyResult<String> {
        let mut options = pncad::step_export::StepOptions::default();
        if let Some(name) = product_name {
            options.product_name = name;
        }
        pncad::export::step_for_node(&self.inner, node.0, &options)
            .map_err(|err| export_err(py, *node, &err))
    }

    fn __repr__(&self) -> String {
        format!("Evaluation({} nodes)", self.inner.order.len())
    }
}

/// Raise `ExportError` mirroring the Rust door's refusal: `variant`
/// is the arm's stable tag, `node` rides along, a poisoning adds
/// `through` and a wrong-kind value adds `kind`. The message is the
/// door's own `Display`.
fn export_err(py: Python<'_>, node: NodeId, err: &pncad::export::ExportError) -> PyErr {
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
        E::Poisoned { through, .. } => match NodeId(*through).into_pyobject(py) {
            Ok(bound) => fields[2] = ("through", bound.unbind().into_any()),
            Err(failed) => return failed,
        },
        E::NotABody { kind, .. } => {
            fields[3] = ("kind", PyString::new(py, kind).unbind().into_any());
        }
        // `Product` is the WHOLE-DOCUMENT door's refusal (ASM-ROOTS
        // D-4): it names product roots, not this call's node, so it
        // adds no field here. The arm is spelled out because the match
        // is exhaustive on purpose — the tripwire, not a wildcard.
        E::UnknownNode { .. }
        | E::NodeFailed { .. }
        | E::EmptyBoolean { .. }
        | E::Step(_)
        | E::Product(_) => {}
    }
    typed_err(py, ErrorClass::Export, err.to_string(), &fields)
}

/// Parse a STEP text with the kernel's own importer and adopt its
/// solid as an opaque `Body` handle — the §L3 journey's round-trip
/// oracle ("the exported file PARSES"), bound so the Python suite can
/// assert it without reaching past the module.
#[pyfunction]
pub(crate) fn import_step(py: Python<'_>, text: &str) -> PyResult<Body> {
    match pncad::step_import::import_step(text, &pncad::step_import::ImportOptions::default()) {
        Ok(pncad::step_import::StepImport::Solid { body, .. }) => Ok(Body {
            inner: Arc::new(body),
        }),
        Ok(pncad::step_import::StepImport::Wireframe { .. }) => Err(typed_err(
            py,
            ErrorClass::StepImport,
            "the file parsed to a wireframe, not a solid",
            &[(
                "variant",
                PyString::new(py, "wireframe").unbind().into_any(),
            )],
        )),
        Err(err) => Err(typed_err(
            py,
            ErrorClass::StepImport,
            format!("{err:?}"),
            &[("variant", PyString::new(py, "refused").unbind().into_any())],
        )),
    }
}

/// Evaluate a document, producing its per-node result DAG.
///
/// Total: evaluation never raises. Individual nodes may still have
/// failed — ask the returned object.
#[pyfunction]
pub(crate) fn evaluate(doc: &super::doc::Doc) -> Evaluation {
    Evaluation {
        inner: d::evaluate::<f64>(
            &doc.inner,
            None,
            &d::CancelToken::new(),
            &d::EvalOptions::default(),
        ),
    }
}

/// Register the value surface on the module.
pub(crate) fn register(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Evaluation>()?;
    m.add_class::<Value>()?;
    m.add_class::<Body>()?;
    m.add_class::<MassProperties>()?;
    m.add_class::<Datum>()?;
    m.add_function(wrap_pyfunction!(evaluate, m)?)?;
    m.add_function(wrap_pyfunction!(import_step, m)?)?;
    Ok(())
}
