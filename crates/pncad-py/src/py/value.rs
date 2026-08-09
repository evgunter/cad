//! Per-node evaluation results: `Evaluation`, `Value`, `Body`.
//!
//! # The ValuePayload exposure inventory (a reported FORK)
//!
//! `ValuePayload` has seven variants. This scaffold projects them as:
//!
//! | variant        | exposure |
//! |----------------|----------|
//! | `Body`         | full — opaque handle + `mass_properties` / `validate` doors |
//! | `Boolean`      | full — unwraps to a `Body`, or `None` when empty |
//! | `Split`        | full — `above` / `below` as optional bodies |
//! | `Instances`    | full — a list of bodies |
//! | `Datum`        | full — typed plane / axis / point with `Length` coordinates |
//! | `Profile`      | KIND ONLY — the curated surface exposes no read-back door on `ValidatedProfile` |
//! | `Declarations` | KIND ONLY — the pairs are `StableName`s, whose curated projection is U5/SEL1 territory |
//!
//! The two kind-only rows are the honest edge of the curated surface,
//! not an omission of convenience: binding them would mean either
//! inventing an accessor (an edit, out of fence) or leaking a `Debug`
//! rendering as data (a string payload, which §L4 forbids).

use std::sync::Arc;

use pyo3::prelude::*;
use pyo3::types::PyString;

use crate::errors::ErrorClass;
use crate::py::quantity::Length;
use crate::py::{doc::NodeId, typed_err};
use pncad::document as d;
use pncad::topo;

/// Raise `EvaluationError` with a stable `reason` tag.
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
        ],
    )
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
    /// # The typed-failure gap (a recorded FINDING)
    ///
    /// §L4 wants a node's FAILURE to arrive as a typed exception
    /// carrying its `NodeError`. The curated surface cannot yet
    /// deliver that: `Evaluation::value` collapses "failed" and
    /// "poisoned" into `None`, and the enum that distinguishes them
    /// (`NodeResult`) is neither re-exported by the façade nor
    /// equipped with accessor methods — so although `NodeError` and
    /// `NodeErrorKind` ARE curated, no curated path leads to one.
    /// (`crate::tags::node_error_tag` is written and compiles against
    /// the real enum, ready for the day an accessor lands.)
    ///
    /// Until then this raises with the most specific reason the
    /// surface can actually justify: `unknown_node` when the id is not
    /// in the evaluated order at all, `no_value` otherwise.
    fn value(&self, py: Python<'_>, node: &NodeId) -> PyResult<Value> {
        match self.inner.value(node.0) {
            Some(node_value) => Ok(Value {
                payload: node_value.payload.clone(),
                node: *node,
            }),
            None if !self.inner.order.contains(&node.0) => Err(eval_err(
                py,
                "no such node in the evaluated document",
                "unknown_node",
                *node,
            )),
            None => Err(eval_err(
                py,
                "the node produced no value (it failed, or an upstream \
                 node did); the typed cause is not reachable through \
                 the curated surface yet",
                "no_value",
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

    fn __repr__(&self) -> String {
        format!("Evaluation({} nodes)", self.inner.order.len())
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
    Ok(())
}
