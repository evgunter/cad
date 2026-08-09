//! The document surface: `Doc`, `DocEdit`, `Node`, `evaluate`.
//!
//! §L3: Python speaks Doc/DocEdit/evaluate/persist and **never an
//! arena key**. The only identifier that crosses is [`NodeId`], a
//! wrapper over `RecipeNodeId` — a recipe-level id, which is precisely
//! the document layer's own public vocabulary, not a slotmap key.

use pyo3::prelude::*;
use pyo3::types::PyString;

use crate::errors::{ErrorClass, check_literal};
use crate::py::typed_err;
use crate::tags::edit_error_tag;
use pncad::document as d;

/// Raise `EditError` carrying the refusal's stable tag.
fn edit_err(py: Python<'_>, err: &d::EditError) -> PyErr {
    let tag = edit_error_tag(err);
    typed_err(
        py,
        ErrorClass::Edit,
        // `EditError` implements no `Display`, so the human message is
        // its `Debug` rendering; the machine-readable payload is the
        // `variant` tag attached below (see `crate::tags`).
        format!("{err:?}"),
        &[("variant", PyString::new(py, tag).unbind().into_any())],
    )
}

/// Raise `LiteralError` for a value the boundary refuses.
fn literal_err(py: Python<'_>, refusal: &crate::errors::LiteralRefusal) -> PyErr {
    use crate::errors::LiteralRefusal as R;
    let (kind, value) = match refusal {
        R::NonFinite { value } => ("non_finite", Some(*value)),
        R::CountIsInteger => ("count_is_integer", None),
    };
    let mut fields: Vec<(&str, Py<PyAny>)> =
        vec![("kind", PyString::new(py, kind).unbind().into_any())];
    if let Some(value) = value {
        match value.into_pyobject(py) {
            Ok(bound) => fields.push(("value", bound.unbind().into_any())),
            Err(failed) => return failed.into(),
        }
    }
    typed_err(py, ErrorClass::Literal, refusal.to_string(), &fields)
}

/// Build a dimensioned literal expression, refusing at the boundary.
///
/// `Expr::literal`'s own error type is not re-exported by the façade
/// (a recorded FINDING), so the two conditions it refuses are checked
/// here first; the residual `map_err` never needs to name the type.
pub(crate) fn literal(py: Python<'_>, value: f64, dim: d::Dimension) -> PyResult<d::Expr> {
    let checked = check_literal(value, dim).map_err(|refusal| literal_err(py, &refusal))?;
    d::Expr::literal(checked, dim).map_err(|_| {
        typed_err(
            py,
            ErrorClass::Literal,
            "the document layer refused a literal the boundary admitted",
            &[(
                "kind",
                PyString::new(py, "kernel_refused").unbind().into_any(),
            )],
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
    pub(crate) inner: d::Doc<d::ProfileDesc>,
}

#[pymethods]
impl Doc {
    /// An empty document.
    #[new]
    fn new() -> Self {
        Self {
            inner: d::Doc::empty(),
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
    pub(crate) inner: d::Node<d::ProfileDesc>,
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
        points: Vec<(super::quantity::Length, super::quantity::Length)>,
        elevation: Option<super::quantity::Length>,
    ) -> Self {
        let meters: Vec<(f64, f64)> = points
            .iter()
            .map(|(x, y)| (x.0.meters(), y.0.meters()))
            .collect();
        let z = elevation.map_or(0.0, |e| e.0.meters());
        let plane = pncad::profile::SketchPlane::from_frame(
            pncad::authoring::p3::<f64>(0.0, 0.0, z),
            pncad::authoring::v3(1.0, 0.0, 0.0),
            pncad::authoring::v3(0.0, 1.0, 0.0),
        );
        let loops = vec![pncad::authoring::polygon::<f64>(&meters)];
        let profile = pncad::profile::Profile::new(plane, loops);
        Self {
            inner: d::Node::Profile(d::ProfileDesc(profile)),
        }
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

/// A single edit to a document — the G1 edit vocabulary, which §L3
/// names as the ONE API surface shared by the GUI, the bindings, macro
/// recording and headless tests.
///
/// This scaffold exposes the three edits the smoke journey needs. The
/// remaining variants (re-witnessing, appearance, rebinds, expression
/// paths) are mechanical additions once the surface they need is
/// curated.
#[pyclass(frozen, module = "pncad", from_py_object)]
#[derive(Clone)]
pub(crate) struct DocEdit {
    pub(crate) inner: d::DocEdit<d::ProfileDesc>,
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
}

/// Register the document surface on the module.
pub(crate) fn register(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<NodeId>()?;
    m.add_class::<Doc>()?;
    m.add_class::<DocEdit>()?;
    m.add_class::<Node>()?;
    m.add_class::<BooleanOp>()?;
    Ok(())
}
