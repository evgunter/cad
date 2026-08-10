//! The document surface: `Doc`, `DocEdit`, `Node`, `evaluate`.
//!
//! §L3: Python speaks Doc/DocEdit/evaluate/persist and **never an
//! arena key**. The only identifier that crosses is [`NodeId`], a
//! wrapper over `RecipeNodeId` — a recipe-level id, which is precisely
//! the document layer's own public vocabulary, not a slotmap key.

use pyo3::prelude::*;
use pyo3::types::PyString;

use crate::errors::ErrorClass;
use crate::py::typed_err;
use crate::tags::{edit_error_tag, expr_dimension_error_tag, persist_error_tag};
use pncad::document as d;

/// Raise `EditError` carrying the refusal's stable tag.
fn edit_err(py: Python<'_>, err: &d::EditError) -> PyErr {
    let tag = edit_error_tag(err);
    typed_err(
        py,
        ErrorClass::Edit,
        // `EditError` implements `Display` (LIB-DOORS F6, reopened on
        // review): the human message is real prose; the machine
        // payload is the `variant` tag (see `crate::tags`).
        err.to_string(),
        &[("variant", PyString::new(py, tag).unbind().into_any())],
    )
}

/// Raise `PersistError` carrying the refusal's stable tag.
fn persist_err(py: Python<'_>, err: &d::PersistError) -> PyErr {
    let tag = persist_error_tag(err);
    typed_err(
        py,
        ErrorClass::Persist,
        // `PersistError` implements `Display`, so the human message is
        // real prose; the machine payload is still the tag.
        err.to_string(),
        &[("variant", PyString::new(py, tag).unbind().into_any())],
    )
}

/// Build a dimensioned literal expression, refusing at the boundary.
///
/// The refusal is `Expr::literal`'s OWN error type, matched — not
/// predicted: the pre-check this function used to carry (LIB-U9S's F5
/// workaround, from before the façade curated `DimensionError`) is
/// gone, so the binding cannot drift from what the kernel refuses.
/// The exception keeps U9S's payload: `kind` (the stable tag) AND
/// `value`, the offending number — the kernel error deliberately
/// carries no float, but the boundary has it in hand.
pub(crate) fn literal(py: Python<'_>, value: f64, dim: d::Dimension) -> PyResult<d::Expr> {
    d::Expr::literal(value, dim).map_err(|err| {
        let tag = expr_dimension_error_tag(&err);
        let value_obj = match value.into_pyobject(py) {
            Ok(bound) => bound.unbind().into_any(),
            Err(failed) => return failed.into(),
        };
        typed_err(
            py,
            ErrorClass::Literal,
            format!("literal {value}: {err}"),
            &[
                ("kind", PyString::new(py, tag).unbind().into_any()),
                ("value", value_obj),
            ],
        )
    })
}

/// A recipe node's identity within a document.
#[pyclass(frozen, module = "pncad", from_py_object)]
#[derive(Clone, Copy)]
pub(crate) struct NodeId(pub(crate) d::RecipeNodeId);

#[pymethods]
impl NodeId {
    fn __repr__(&self) -> String {
        format!("NodeId({})", self.0.0)
    }

    fn __eq__(&self, other: &Self) -> bool {
        self.0 == other.0
    }

    fn __hash__(&self) -> u64 {
        self.0.0
    }
}

/// A parametric document: the recipe, not the geometry.
///
/// Rust's `Doc` is immutable — `apply` returns a NEW document. Python
/// object semantics make an in-place edit far more natural, so this
/// wrapper owns the current document and swaps it on a successful
/// apply. The Rust value semantics are still available: a refused edit
/// leaves the document untouched, exactly as the immutable API
/// guarantees, because the swap only happens on `Ok`.
#[pyclass(module = "pncad")]
pub(crate) struct Doc {
    pub(crate) inner: d::ProfileDoc,
}

#[pymethods]
impl Doc {
    /// An empty document.
    #[new]
    fn new() -> Self {
        Self {
            inner: d::ProfileDoc::empty(),
        }
    }

    /// Apply an edit, returning the id it minted (if any).
    ///
    /// On refusal the document is unchanged and a typed `EditError` is
    /// raised.
    fn apply(&mut self, py: Python<'_>, edit: &DocEdit) -> PyResult<Option<NodeId>> {
        let applied = d::apply(&self.inner, &edit.inner).map_err(|err| edit_err(py, &err))?;
        self.inner = applied.doc;
        Ok(applied.record.minted.map(NodeId))
    }

    /// Insert a node and return its minted id — the common case,
    /// spelled without the intermediate `DocEdit`.
    fn insert(&mut self, py: Python<'_>, node: &Node) -> PyResult<NodeId> {
        let edit = d::DocEdit::InsertNode {
            node: node.inner.clone(),
        };
        let applied = d::apply(&self.inner, &edit).map_err(|err| edit_err(py, &err))?;
        self.inner = applied.doc;
        applied.record.minted.map(NodeId).ok_or_else(|| {
            typed_err(
                py,
                ErrorClass::Edit,
                "an insert minted no node id",
                &[(
                    "variant",
                    PyString::new(py, "no_minted_id").unbind().into_any(),
                )],
            )
        })
    }

    /// How many nodes the document holds.
    #[getter]
    fn node_count(&self) -> usize {
        self.inner.len()
    }

    /// The document's evaluation order.
    fn order(&self) -> Vec<NodeId> {
        self.inner.order().iter().copied().map(NodeId).collect()
    }

    /// The document's tolerance.
    #[getter]
    fn epsilon(&self) -> f64 {
        self.inner.epsilon()
    }

    /// Bit-exact document equality (D9's replay currency).
    fn bit_eq(&self, other: &Self) -> bool {
        self.inner.bit_eq(&other.inner)
    }

    /// Serialize this document to the persistence text format
    /// (LIB-DOORS F1; the schema-v4 doors, via the curated façade).
    ///
    /// The Python wrapper holds only the CURRENT document — its edit
    /// history lives in the Rust values it discarded — so the file is
    /// a snapshot with an empty edit log. That is a complete, loadable
    /// document; the GUI's edit-log-bearing files load through the
    /// same `load` door.
    fn save(&self, py: Python<'_>) -> PyResult<String> {
        d::save(&self.inner, &[]).map_err(|err| persist_err(py, &err))
    }

    fn __len__(&self) -> usize {
        self.inner.len()
    }

    fn __repr__(&self) -> String {
        format!("Doc({} nodes)", self.inner.len())
    }
}

/// Which Boolean the document layer performs.
///
/// This is the DOCUMENT-layer `BooleanOp` (`pncad::document`), not the
/// kernel's identically-named `topo::BooleanOp` that sits in the Rust
/// prelude. LIB-LOG's U9 backlog note left the choice open; bindings
/// speak the document vocabulary throughout (§L3), so the document
/// one is what crosses.
#[pyclass(eq, eq_int, module = "pncad", from_py_object)]
#[derive(Clone, Copy, PartialEq)]
pub(crate) enum BooleanOp {
    /// Fuse the operands.
    Union,
    /// Keep the common volume.
    Intersect,
    /// Remove `b` from `a`.
    Subtract,
}

impl BooleanOp {
    fn to_document(self) -> d::BooleanOp {
        match self {
            Self::Union => d::BooleanOp::Union,
            Self::Intersect => d::BooleanOp::Intersect,
            Self::Subtract => d::BooleanOp::Subtract,
        }
    }
}

/// A recipe node, before it is inserted into a document.
#[pyclass(frozen, module = "pncad", from_py_object)]
#[derive(Clone)]
pub(crate) struct Node {
    pub(crate) inner: d::Node<d::ProfileProgram>,
}

#[pymethods]
impl Node {
    /// A closed polygonal sketch on a plane parallel to the world
    /// xy-plane, `elevation` above it (default: the xy-plane itself).
    ///
    /// Coordinates arrive as typed `Length`s (§L4), so a bare number
    /// is a boundary refusal rather than an ambiguous unit.
    ///
    /// `elevation` exists because the kernel is fail-loud about
    /// coincidence: it never INFERS that two faces are the same face,
    /// so two solids merely touching on a shared plane are refused
    /// (`UndeclaredCoincidence`) until the author declares the
    /// contact. Authoring a genuine Boolean therefore needs solids
    /// that interpenetrate, which needs sketches at different
    /// heights. Fully arbitrary sketch placement still needs an
    /// `Affine3` binding, which is out of this unit's fence.
    #[staticmethod]
    #[pyo3(signature = (points, elevation=None))]
    fn polygon(
        py: Python<'_>,
        points: Vec<(super::quantity::Length, super::quantity::Length)>,
        elevation: Option<super::quantity::Length>,
    ) -> PyResult<Self> {
        let z = elevation.map_or(0.0, |e| e.0.meters());
        let plane = pncad::profile::SketchPlane::from_frame(
            pncad::authoring::p3::<f64>(0.0, 0.0, z),
            pncad::authoring::v3(1.0, 0.0, 0.0),
            pncad::authoring::v3(0.0, 1.0, 0.0),
        );
        // The polygon as a loop PROGRAM (the post-switch v4 payload:
        // the program IS the profile's definition): anchor at the
        // first vertex, one `line_to` per remaining vertex, close by
        // targeting `Start`. Too few points is not pre-checked — the
        // edit door's replay probe refuses it typed, at `insert`.
        let point = |py2: Python<'_>,
                     p: &(super::quantity::Length, super::quantity::Length)|
         -> PyResult<[d::Expr; 2]> {
            Ok([
                literal(py2, p.0.0.meters(), d::Dimension::Length)?,
                literal(py2, p.1.0.meters(), d::Dimension::Length)?,
            ])
        };
        let mut steps = Vec::with_capacity(points.len() + 1);
        let mut vertices = points.iter();
        if let Some(first) = vertices.next() {
            steps.push(d::ProgramStep::At(point(py, first)?));
        }
        for p in vertices {
            steps.push(d::ProgramStep::LineTo(d::ProgramTarget::Point(point(
                py, p,
            )?)));
        }
        steps.push(d::ProgramStep::LineTo(d::ProgramTarget::Start));
        Ok(Self {
            inner: d::Node::Profile(d::ProfileProgram {
                plane,
                loops: vec![d::LoopProgram::Chain(steps)],
            }),
        })
    }

    /// The profile a PATHS loop authors, on a plane parallel to the
    /// world xy-plane, `elevation` above it (default: the xy-plane).
    ///
    /// The node is built from the loop's RECORDED program — the same
    /// verbs, with the same authored arguments, that the chain wrote
    /// — so what Python authored and what the document replays are
    /// one program, not two spellings of one shape.
    ///
    /// Exactly ONE loop: multi-loop profiles (holes) and non-xy
    /// planes are named gaps, not omissions.
    #[staticmethod]
    #[pyo3(signature = (outline, elevation=None))]
    fn profile(
        py: Python<'_>,
        outline: &super::path::ClosedLoop,
        elevation: Option<super::quantity::Length>,
    ) -> PyResult<Self> {
        let z = elevation.map_or(0.0, |e| e.0.meters());
        let plane = pncad::profile::SketchPlane::from_frame(
            pncad::authoring::p3::<f64>(0.0, 0.0, z),
            pncad::authoring::v3(1.0, 0.0, 0.0),
            pncad::authoring::v3(0.0, 1.0, 0.0),
        );
        Ok(Self {
            inner: d::Node::Profile(d::ProfileProgram {
                plane,
                loops: vec![super::path::loop_program(py, outline)?],
            }),
        })
    }

    /// Extrude an upstream profile along its sketch-plane normal.
    #[staticmethod]
    fn extrude(
        py: Python<'_>,
        profile: &NodeId,
        distance: &super::quantity::Length,
    ) -> PyResult<Self> {
        let distance = literal(py, distance.0.meters(), d::Dimension::Length)?;
        Ok(Self {
            inner: d::Node::Extrude {
                profile: profile.0,
                distance,
            },
        })
    }

    /// Revolve an upstream profile about a datum axis.
    #[staticmethod]
    fn revolve(
        py: Python<'_>,
        profile: &NodeId,
        axis: &NodeId,
        angle: &super::quantity::Angle,
    ) -> PyResult<Self> {
        let angle = literal(py, angle.0.radians(), d::Dimension::Angle)?;
        Ok(Self {
            inner: d::Node::Revolve {
                profile: profile.0,
                axis: axis.0,
                angle,
            },
        })
    }

    /// A datum axis: a point and a direction.
    ///
    /// The origin is dimensioned (`Length`); the direction is a
    /// dimensionless triple, matching `SlotId::Direction`'s `Scalar`.
    #[staticmethod]
    fn datum_axis(
        py: Python<'_>,
        origin: (
            super::quantity::Length,
            super::quantity::Length,
            super::quantity::Length,
        ),
        direction: (f64, f64, f64),
    ) -> PyResult<Self> {
        let origin = [
            literal(py, origin.0.0.meters(), d::Dimension::Length)?,
            literal(py, origin.1.0.meters(), d::Dimension::Length)?,
            literal(py, origin.2.0.meters(), d::Dimension::Length)?,
        ];
        let direction = [
            literal(py, direction.0, d::Dimension::Scalar)?,
            literal(py, direction.1, d::Dimension::Scalar)?,
            literal(py, direction.2, d::Dimension::Scalar)?,
        ];
        Ok(Self {
            inner: d::Node::Datum(d::Datum::Axis { origin, direction }),
        })
    }

    /// A Boolean of two upstream solids.
    #[staticmethod]
    fn boolean(op: BooleanOp, a: &NodeId, b: &NodeId) -> Self {
        Self {
            inner: d::Node::Boolean {
                op: op.to_document(),
                a: a.0,
                b: b.0,
                declare: None,
            },
        }
    }
}

/// A document-level parameter name (guide §3.2) — a plain string
/// newtype, the same name the recipe's expressions reference. NOT an
/// arena key: recipe vocabulary, meaningful in any document.
#[pyclass(frozen, module = "pncad", from_py_object)]
#[derive(Clone)]
pub(crate) struct ParamName(pub(crate) d::ParamName);

#[pymethods]
impl ParamName {
    #[new]
    fn new(name: &str) -> Self {
        Self(d::ParamName::new(name))
    }

    /// The name itself.
    #[getter]
    fn name(&self) -> String {
        self.0.0.clone()
    }

    fn __repr__(&self) -> String {
        format!("ParamName({:?})", self.0.0)
    }

    fn __eq__(&self, other: &Self) -> bool {
        self.0 == other.0
    }

    fn __hash__(&self) -> u64 {
        use std::hash::{Hash, Hasher};
        let mut h = std::hash::DefaultHasher::new();
        self.0.hash(&mut h);
        h.finish()
    }
}

/// A named parameter's declared dimension and exact stored value
/// (guide §3.2): what `DocEdit.set_doc_param` writes.
///
/// Continuous values arrive as typed quantities (§L4), so the
/// dimension is carried by the constructor rather than guessed from a
/// bare float. A non-finite value is NOT pre-checked here — the edit
/// door refuses it typed (`non_finite_doc_param`), fail-loud where
/// the kernel refuses.
#[pyclass(frozen, module = "pncad", from_py_object)]
#[derive(Clone)]
pub(crate) struct DocParam(pub(crate) d::DocParam);

#[pymethods]
impl DocParam {
    /// A continuous Length parameter.
    #[staticmethod]
    fn length(value: &super::quantity::Length) -> Self {
        Self(d::DocParam::Continuous {
            dim: d::Dimension::Length,
            value: value.0.meters(),
        })
    }

    /// A continuous Angle parameter.
    #[staticmethod]
    fn angle(value: &super::quantity::Angle) -> Self {
        Self(d::DocParam::Continuous {
            dim: d::Dimension::Angle,
            value: value.0.radians(),
        })
    }

    /// A continuous dimensionless parameter.
    #[staticmethod]
    fn scalar(value: f64) -> Self {
        Self(d::DocParam::Continuous {
            dim: d::Dimension::Scalar,
            value,
        })
    }

    /// An integer Count parameter (structural material, spec D3).
    #[staticmethod]
    fn count(value: i64) -> Self {
        Self(d::DocParam::Count { value })
    }

    fn __repr__(&self) -> String {
        match &self.0 {
            d::DocParam::Continuous { dim, value } => {
                format!("DocParam({dim:?} {value})")
            }
            d::DocParam::Count { value } => format!("DocParam(Count {value})"),
        }
    }
}

/// A single edit to a document — the G1 edit vocabulary, which §L3
/// names as the ONE API surface shared by the GUI, the bindings, macro
/// recording and headless tests.
///
/// Four edits are exposed today: `insert_node`, `delete_node`,
/// `set_tolerance` and `set_doc_param` (R1-PARAMS). The remaining
/// variants (slot edits, re-witnessing, appearance, rebinds,
/// expression paths) are mechanical additions once the surface they
/// need is curated — tracked as named gaps in
/// `docs/guide/north-star-audit.md`.
#[pyclass(frozen, module = "pncad", from_py_object)]
#[derive(Clone)]
pub(crate) struct DocEdit {
    pub(crate) inner: d::DocEdit<d::ProfileProgram>,
}

#[pymethods]
impl DocEdit {
    /// Insert a node.
    #[staticmethod]
    fn insert_node(node: &Node) -> Self {
        Self {
            inner: d::DocEdit::InsertNode {
                node: node.inner.clone(),
            },
        }
    }

    /// Delete a node.
    #[staticmethod]
    fn delete_node(id: &NodeId) -> Self {
        Self {
            inner: d::DocEdit::DeleteNode { id: id.0 },
        }
    }

    /// Set the document tolerance.
    #[staticmethod]
    fn set_tolerance(eps: f64) -> Self {
        Self {
            inner: d::DocEdit::SetTolerance { eps },
        }
    }

    /// Create or replace a document-level named parameter (guide
    /// §3.2). The edit applies cleanly even for a value the geometry
    /// will refuse — a program that refuses under the current binding
    /// is legal AT REST; the refusal belongs to replay.
    #[staticmethod]
    fn set_doc_param(name: &ParamName, value: &DocParam) -> Self {
        Self {
            inner: d::DocEdit::SetDocParam {
                name: name.0.clone(),
                value: value.0.clone(),
            },
        }
    }
}

/// A loaded document (LIB-DOORS F1): what the persistence door
/// answered — the snapshot as saved, the replayed current state, and
/// how many recorded edits the replay ran.
#[pyclass(frozen, module = "pncad")]
pub(crate) struct Loaded {
    snapshot: d::ProfileDoc,
    doc: d::ProfileDoc,
    edit_count: usize,
}

#[pymethods]
impl Loaded {
    /// The current document: the snapshot with every recorded edit
    /// replayed through the `apply` door.
    #[getter]
    fn doc(&self) -> Doc {
        Doc {
            inner: self.doc.clone(),
        }
    }

    /// The snapshot exactly as saved, before replay.
    #[getter]
    fn snapshot(&self) -> Doc {
        Doc {
            inner: self.snapshot.clone(),
        }
    }

    /// How many recorded edits the load replayed.
    #[getter]
    fn edit_count(&self) -> usize {
        self.edit_count
    }

    fn __repr__(&self) -> String {
        format!(
            "Loaded({} nodes, {} replayed edits)",
            self.doc.len(),
            self.edit_count
        )
    }
}

/// Parse, validate, and replay a saved document (LIB-DOORS F1).
///
/// Every refusal is a typed `PersistError` carrying the arm's stable
/// `variant` tag — bad header, unknown schema, unparseable body, a
/// snapshot or edit log the shared validator rejects, an ε conflict.
#[pyfunction]
pub(crate) fn load(py: Python<'_>, text: &str) -> PyResult<Loaded> {
    let loaded = d::load(text).map_err(|err| persist_err(py, &err))?;
    Ok(Loaded {
        snapshot: loaded.snapshot,
        doc: loaded.doc,
        edit_count: loaded.edits.len(),
    })
}

/// Register the document surface on the module.
pub(crate) fn register(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<NodeId>()?;
    m.add_class::<Doc>()?;
    m.add_class::<DocEdit>()?;
    m.add_class::<ParamName>()?;
    m.add_class::<DocParam>()?;
    m.add_class::<Node>()?;
    m.add_class::<BooleanOp>()?;
    m.add_class::<Loaded>()?;
    m.add_function(wrap_pyfunction!(load, m)?)?;
    Ok(())
}
