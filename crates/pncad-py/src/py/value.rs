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
//! | `Profile`      | KIND ONLY — sketch geometry does not ship to Python before the v2 switch |
//! | `Declarations` | KIND ONLY — the naming projection is deferred, not blocked |
//!
//! The two kind-only rows are SCOPE decisions, not capability limits.
//! Being precise about which, because the distinction is load-bearing:
//!
//! * `ValidatedProfile::plane()`/`loops()` DO exist and `profile` is
//!   wholesale re-exported — this very module's sibling uses
//!   `pncad::profile` to build sketches. Projecting a profile back to
//!   Python is therefore perfectly possible; it is **ruled out**:
//!   Python never ships the opaque-profile intermediate state.
//!   Sketch read-back belongs with the v2 program representation.
//! * `StableName` is likewise prelude-curated with public fields, so
//!   Declarations is reachable too. It is deferred because the
//!   naming/selection projection is a design subject of its own, and
//!   binding a provisional shape here would fork it.

use std::sync::Arc;

use pyo3::prelude::*;
use pyo3::types::PyString;

use crate::errors::ErrorClass;
use crate::py::quantity::Length;
use crate::py::{doc::NodeId, typed_err};
use crate::tags::{export_error_tag, node_error_tag, step_import_error_tag};
use pncad::document as d;
use pncad::tolerance::Tol;
use pncad::topo;

/// Raise `EvaluationError` with a stable `reason` tag.
///
/// `kind`, `through` and `finding` are ALWAYS present on the
/// exception — `None` where the reason has no failing kind, no
/// poisoning ancestor, or no refusal-menu payload — so stub-guided
/// code can read them without an `AttributeError` trap — a stub that
/// over-promises is worse than one that says `None`.
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
            ("finding", py.None().into_any()),
        ],
    )
}

/// Raise `EvaluationError` for a node that ITSELF failed: the payload
/// is the `NodeErrorKind`'s stable tag plus the node id; the message
/// is the kernel error's own `Display` prose — never a `Debug` dump.
fn node_failure(py: Python<'_>, node: NodeId, error: &d::NodeError) -> PyErr {
    let node_obj = match node.into_pyobject(py) {
        Ok(bound) => bound.unbind().into_any(),
        Err(failed) => return failed,
    };
    // The refusal MENU: an undeclared-contact
    // refusal carries its candidate declaration as a typed
    // `FlushFinding` on the exception — the same value shape
    // `Evaluation.find_flush_candidates` answers with, ready for
    // `Node.declare`/`Doc.declare`. `None` on every other kind.
    let finding = match &error.kind {
        d::NodeErrorKind::UndeclaredContact { finding, .. } => {
            match super::flush::FlushFinding((**finding).clone()).into_pyobject(py) {
                Ok(bound) => bound.unbind().into_any(),
                Err(failed) => return failed,
            }
        }
        _ => py.None().into_any(),
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
            ("finding", finding),
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
        ("finding", py.None().into_any()),
    ];
    // The message is the root cause's `Display` prose: the node
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
/// Arena keys never cross; a body crosses as a handle whose
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
        let tol = Tol::witness();
        let props = topo::mass_properties(&self.inner, tol).map_err(|err| {
            typed_err(
                py,
                ErrorClass::Validation,
                err.to_string(),
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

    /// Geometric validation only.
    fn validate_geometric(&self, py: Python<'_>) -> PyResult<()> {
        let tol = Tol::witness();
        self.run_validator(
            py,
            "validate_geometric",
            topo::validate_geometric(&self.inner, tol),
        )
    }
}

impl Body {
    /// Shared shape for the validator doors, which all return
    /// `Result<(), Vec<ValidationError>>`.
    ///
    /// `ValidationError` has no curated tag mapping, so the exception
    /// carries the failure COUNT as structured data and the findings
    /// themselves as the human message — each through the enum's own
    /// `Display`, one prose sentence with recourse per finding, joined
    /// because a `Vec` has no rendering of its own. Per-variant tags
    /// are the same mechanical work `crate::tags` does for edits,
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
                "{door} reported {} failure(s): {}",
                failures.len(),
                failures
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
                    .join("; ")
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
    params: d::ParamEnv<f64>,
}

#[pymethods]
impl Evaluation {
    /// The node's successful value.
    ///
    /// A node that produced NO value raises with the REAL typed cause
    /// — never a placeholder:
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
        node: &NodeId,
        selector: &super::select::Selector,
    ) -> PyResult<Vec<String>> {
        names(py, pncad::select::select(&self.inner, node.0, &selector.0))
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
    /// candidate (`"unreadable"`), a non-datum reference
    /// (`"not_a_datum"`) all refuse rather than silently including or
    /// dropping a candidate.
    fn select_where(
        &self,
        py: Python<'_>,
        node: &NodeId,
        selector: &super::select::Selector,
        geom: Vec<super::select::GeomPred>,
    ) -> PyResult<Vec<String>> {
        let tol = Tol::witness();
        let atoms: Vec<pncad::select::GeomPred> = geom.into_iter().map(|g| g.0).collect();
        match pncad::select::select_where(
            &self.inner,
            node.0,
            &selector.0,
            &atoms,
            &self.params,
            tol,
        ) {
            Ok(found) => names(py, found),
            Err(refusal) => Err(super::select::select_refusal(py, &refusal)),
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
        node: &NodeId,
        name: &str,
    ) -> PyResult<super::readback::Pose> {
        let name = super::doc::name_from_text(name)?;
        pncad::select::face_frame(&self.inner, node.0, &name)
            .map(super::readback::Pose)
            .map_err(|err| super::readback::readback_err(py, &err))
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
        node: &NodeId,
        name: &str,
    ) -> PyResult<super::readback::Pose> {
        let name = super::doc::name_from_text(name)?;
        pncad::select::edge_frame(&self.inner, node.0, &name)
            .map(super::readback::Pose)
            .map_err(|err| super::readback::readback_err(py, &err))
    }

    /// **Where is the vertex I selected?** — the named vertex's
    /// stored position, dimensioned.
    ///
    /// Raises `ReadbackError` as [`Self::face_frame`] does, with
    /// `wrong_kind` for a non-vertex name.
    fn vertex_position(
        &self,
        py: Python<'_>,
        node: &NodeId,
        name: &str,
    ) -> PyResult<(Length, Length, Length)> {
        let name = super::doc::name_from_text(name)?;
        pncad::select::vertex_position(&self.inner, node.0, &name)
            .map(lengths)
            .map_err(|err| super::readback::readback_err(py, &err))
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
        node: &NodeId,
        name: &str,
    ) -> PyResult<super::readback::Denotation> {
        let name = super::doc::name_from_text(name)?;
        pncad::select::denotation(&self.inner, node.0, &name)
            .map(super::readback::Denotation)
            .map_err(|err| super::readback::readback_err(py, &err))
    }

    /// **The cross-body flush-plane candidates between `a`'s and
    /// `b`'s outputs, as of THIS evaluation** — the detect arm of the
    /// detect/declare protocol: the verifier run in
    /// candidate-generation mode, so a
    /// finding can never disagree with the boolean's own
    /// verify-at-use.
    ///
    /// Findings come back in canonical order and are only ever
    /// DEFINITE values — inspect them, then `Node.declare` /
    /// `Doc.declare` / `Doc.declare_all` turn the inspected findings
    /// into the `Declare` node `Node.boolean`'s `declare=` consumes.
    /// Detection and declaration are separate doors ON PURPOSE (the
    /// ruled no-fusion boundary). Like `select`, the query answers
    /// EMPTY if either node has no value in this evaluation.
    ///
    /// Raises `SelectRefusal`, typed, exactly where the Rust door
    /// refuses: a pair whose verify-door margin is inside the
    /// ambiguity band (`reason="pair_in_band"` — neither reported nor
    /// silently dropped), a tied name whose candidates disagree
    /// (`"tied_disagrees"`), an unreadable name-table entry
    /// (`"unreadable"`), a broken ambient tolerance (`"band"`).
    fn find_flush_candidates(
        &self,
        py: Python<'_>,
        a: &NodeId,
        b: &NodeId,
    ) -> PyResult<Vec<super::flush::FlushFinding>> {
        let tol = Tol::witness();
        match pncad::select::find_flush_candidates(&self.inner, a.0, b.0, tol) {
            Ok(findings) => Ok(findings
                .into_iter()
                .map(super::flush::FlushFinding)
                .collect()),
            Err(refusal) => Err(super::select::select_refusal(py, &refusal)),
        }
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
    /// raises a typed `ExportError`.
    #[pyo3(signature = (node, product_name=None))]
    fn step_string(
        &self,
        py: Python<'_>,
        node: &NodeId,
        product_name: Option<String>,
    ) -> PyResult<String> {
        let tol = Tol::witness();
        let mut options = pncad::step_export::StepOptions::default();
        if let Some(name) = product_name {
            options.product_name = name;
        }
        pncad::export::step_for_node(&self.inner, node.0, &options, tol)
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
        // `Product` is the WHOLE-DOCUMENT door's refusal: it names
        // product roots, not this call's node, so it adds no field
        // here. The arm is spelled out because the match
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
/// solid as an opaque `Body` handle — the one-shot journey's round-trip
/// oracle ("the exported file PARSES"), bound so the Python suite can
/// assert it without reaching past the module.
#[pyfunction]
pub(crate) fn import_step(py: Python<'_>, text: &str) -> PyResult<Body> {
    let tol = Tol::witness();
    match pncad::step_import::import_step(text, &pncad::step_import::ImportOptions::default(), tol)
    {
        Ok(pncad::step_import::StepImport::Solid { body, .. }) => Ok(Body {
            inner: Arc::new(body),
        }),
        // Not a refusal variant: the import SUCCEEDED and produced
        // the other arm of `StepImport`, which this door does not
        // adopt. Its tag is the arm's name and shares the namespace
        // with `step_import_error_tag`'s, which contains no
        // `wireframe`.
        Ok(pncad::step_import::StepImport::Wireframe { .. }) => Err(typed_err(
            py,
            ErrorClass::StepImport,
            "the file parsed to a wireframe, not a solid",
            &[(
                "variant",
                PyString::new(py, "wireframe").unbind().into_any(),
            )],
        )),
        // The tag is the importer's own, through `crate::tags`. Every
        // arm of `StepImportError` is reachable here, and the entity
        // id and line that would tell them apart live in the message
        // prose — so one literal for all twenty-one would make them
        // indistinguishable to a caller.
        Err(err) => Err(typed_err(
            py,
            ErrorClass::StepImport,
            err.to_string(),
            &[(
                "variant",
                PyString::new(py, step_import_error_tag(&err))
                    .unbind()
                    .into_any(),
            )],
        )),
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
/// content, so an evaluation of a different document is a legal prior
/// that reuses nothing — node ids are minted per document, and two
/// assemblies over the same parts at the same pins still share none.
/// Use the prior evaluation of THIS document.
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
#[pyfunction]
#[pyo3(signature = (doc, *, resolver=None, prior=None))]
pub(crate) fn evaluate(
    doc: &super::doc::Doc,
    resolver: Option<&super::store::Workspace>,
    prior: Option<&Evaluation>,
) -> Evaluation {
    let tol = Tol::witness();
    let opts = d::EvalOptions {
        resolver: resolver.map(super::store::Workspace::resolver),
        ..d::EvalOptions::default()
    };
    Evaluation {
        inner: d::evaluate::<f64>(
            &doc.inner,
            prior.map(|p| &p.inner),
            &d::CancelToken::new(),
            &opts,
            tol,
        ),
        params: doc.inner.param_env::<f64>(),
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
