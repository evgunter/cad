//! The document surface: `Doc`, `DocEdit`, `Node`, `evaluate`.
//!
//! Python speaks Doc/DocEdit/evaluate/persist and **never an arena
//! key**. The only identifier that crosses is [`NodeId`], a
//! wrapper over `RecipeNodeId` — a recipe-level id, which is precisely
//! the document layer's own public vocabulary, not a slotmap key.

use pyo3::prelude::*;
use pyo3::types::{PyDict, PyString};

use crate::errors::ErrorClass;
use crate::py::typed_err;
use crate::tags::{
    edit_error_tag, expr_dimension_error_tag, persist_error_tag, workspace_error_tag,
};
use pncad::document as d;
use pncad::tolerance::Tol;

/// Raise `EditError` carrying the refusal's stable tag.
fn edit_err(py: Python<'_>, err: &d::EditError) -> PyErr {
    let tag = edit_error_tag(err);
    typed_err(
        py,
        ErrorClass::Edit,
        // `EditError` implements `Display`: the human message is real
        // prose; the machine payload is the `variant` tag (see
        // `crate::tags`).
        err.to_string(),
        &[("variant", PyString::new(py, tag).unbind().into_any())],
    )
}

/// Raise `EditError` for a declare-sugar refusal.
///
/// `DeclareError` implements `Display`: the human message is the
/// door's own prose (its `Edit` arm forwards the document layer's
/// message, so one refusal keeps one voice), and the machine payload
/// is the stable tag (`crate::tags::declare_error_tag` — the `Edit`
/// arm carries the document layer's own tag through).
fn declare_err(py: Python<'_>, err: &pncad::select::DeclareError) -> PyErr {
    typed_err(
        py,
        ErrorClass::Edit,
        err.to_string(),
        &[(
            "variant",
            PyString::new(py, crate::tags::declare_error_tag(err))
                .unbind()
                .into_any(),
        )],
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
/// predicted: the binding carries no pre-check of its own, so it
/// cannot drift from what the kernel refuses. The exception carries
/// `kind` (the stable tag) AND `value`, the offending number — the
/// kernel error deliberately
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

/// A stable name as Python holds it: the name's OWN serde text.
///
/// # The text is OPAQUE BY CONTRACT
///
/// It is a stable IDENTIFIER — carried from a materializer to a
/// selection unread, exactly as the Rust surface carries the value —
/// and its internal structure is **not API**. Parsing it is
/// representation-dependence: the encoding may change without notice,
/// and code that reads inside a name is code this crate will break.
/// The supported operations are equality, ordering, storage, and
/// handing it back to [`Node::fillet`]. Narrowing a set of names is a
/// SELECTOR's job — `Evaluation.select` / `select_where`,
/// which answer in this same alphabet; the binding is the one
/// licensed reader of the text.
///
/// # Why this encoding
///
/// `StableName` has exactly one serialization and the binding reuses
/// it rather than minting a second spelling, so a name is one
/// vocabulary across Rust, Python and the file format. The relation
/// to a saved document is VALUE equality, not byte equality: `save`
/// pretty-prints and this writes compact, so the two texts differ in
/// whitespace and parse to the same JSON value — and a name taken
/// from either round-trips through the other.
pub(crate) fn name_text(py: Python<'_>, name: &pncad::prelude::StableName) -> PyResult<String> {
    serde_json::to_string(name).map_err(|err| {
        typed_err(
            py,
            ErrorClass::Edit,
            format!("a stable name failed to serialize: {err}"),
            &[(
                "variant",
                PyString::new(py, "name_serialize").unbind().into_any(),
            )],
        )
    })
}

/// Read a stable name back from [`name_text`]'s output.
///
/// Text that is not a name at all is a boundary `ValueError` — the
/// same class of refusal as a string where a `SketchPlane` belongs,
/// with no kernel refusal to forward. A WELL-FORMED name that denotes
/// nothing in this document refuses at the kernel's own door
/// (`fillet_selection_resolve`), which is where that belongs.
pub(crate) fn name_from_text(text: &str) -> PyResult<pncad::prelude::StableName> {
    serde_json::from_str(text).map_err(|err| {
        pyo3::exceptions::PyValueError::new_err(format!(
            "not a stable name: {text:?} ({err}) — names come from \
             `Evaluation.all_edges` and its siblings"
        ))
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
        let _tol = Tol::witness();
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
    /// What the LAST accepted edit did to the placement registry.
    ///
    /// The Rust `apply` returns this beside the new document; the
    /// Python wrapper owns the document and swaps it, so the record
    /// is held here for the same span the document it describes is.
    pub(crate) maintenance: Vec<d::ClusterMaintenance>,
}

#[pymethods]
impl Doc {
    /// An empty document.
    ///
    /// A document's id answers WHICH PART, and a workspace refuses to
    /// hold two files claiming one — so `Doc()` mints a FRESH random
    /// identity and two documents authored here are two parts.
    /// `Doc(label)` derives the id from the label instead: same
    /// label, same id, on every platform, which is the spelling a
    /// caller whose saves must reproduce byte for byte wants — and
    /// which therefore makes two same-label documents the SAME part,
    /// deliberately.
    ///
    /// Raises `IdentityError` if the OS entropy source refuses.
    #[new]
    #[pyo3(signature = (label = None))]
    fn new(py: Python<'_>, label: Option<&str>) -> PyResult<Self> {
        let tol = Tol::witness();
        let inner = match label {
            Some(label) => crate::identity::derived(label, tol),
            // The tag is the store's own, through `crate::tags`, not a
            // literal chosen here: `interactive` refuses with the whole
            // `WorkspaceError` vocabulary, and which of its arms a
            // caller sees is that enum's fact to state, not this raise
            // site's to assume. Only `randomness_unavailable` is
            // reachable through this door today — `random_document_id`
            // has one failure arm — but that is a fact about ANOTHER
            // crate, and a literal here would go on being written
            // confidently over a second arm the day one appears.
            None => crate::identity::interactive(tol).map_err(|err| {
                typed_err(
                    py,
                    ErrorClass::Identity,
                    err.to_string(),
                    &[(
                        "variant",
                        PyString::new(py, workspace_error_tag(&err))
                            .unbind()
                            .into_any(),
                    )],
                )
            })?,
        };
        Ok(Self {
            inner,
            maintenance: Vec::new(),
        })
    }

    /// This document's identity as 32 lowercase hex digits — the
    /// canonical text form, the same one the save file's `id:` header
    /// carries and the workspace store keys on.
    ///
    /// Identity survives every edit; it is not a content hash.
    #[getter]
    fn id(&self) -> String {
        self.inner.id().hex()
    }

    /// Apply an edit, returning the id it minted (if any).
    ///
    /// On refusal the document is unchanged and a typed `EditError` is
    /// raised.
    ///
    /// An accepted edit may also have performed **cluster-record
    /// maintenance** — joins, splits, gauge rewrites and drops the
    /// mate graph's motion forced on the placement registry. That
    /// rides the edit rather than being a second edit, so it is read
    /// off `last_maintenance` instead of returned here: the common
    /// case is an empty list, and widening every caller's return type
    /// for it would be paying for mates in documents that have none.
    fn apply(&mut self, py: Python<'_>, edit: &DocEdit) -> PyResult<Option<NodeId>> {
        let tol = Tol::witness();
        let applied = d::apply(&self.inner, &edit.inner, tol).map_err(|err| edit_err(py, &err))?;
        self.inner = applied.doc;
        self.maintenance = applied.maintenance;
        Ok(applied.record.minted.map(NodeId))
    }

    /// The cluster-record maintenance the LAST accepted edit
    /// performed, in the order it was performed.
    ///
    /// Empty after any edit that moved no mate graph, and empty on a
    /// fresh document — a document that has never applied an edit has
    /// no last edit to report about. A REFUSED edit leaves this
    /// untouched, exactly as it leaves the document untouched.
    ///
    /// Undo is keeping the prior document value, which restores every
    /// one of these exactly; what the record adds is VISIBILITY — an
    /// absorbed cluster's frame is consumed here, where a caller can
    /// read what was consumed.
    #[getter]
    fn last_maintenance(&self) -> Vec<super::mate::ClusterMaintenance> {
        self.maintenance
            .iter()
            .cloned()
            .map(super::mate::ClusterMaintenance)
            .collect()
    }

    /// The document's ordered **product roots** — what `product` and
    /// `assemble` gather, in this order.
    ///
    /// Set through `DocEdit.set_roots`. Maintained automatically by
    /// every other edit (inserting a node that consumes a root
    /// transfers it), so a document always states its product rather
    /// than leaving it to be inferred.
    #[getter]
    fn roots(&self) -> Vec<NodeId> {
        self.inner.roots().iter().copied().map(NodeId).collect()
    }

    /// An instance's **cluster frame**: the placement recorded for the
    /// cluster this node belongs to, or the identity when nothing was
    /// recorded.
    ///
    /// Total — a node with no recorded row answers the identity, which
    /// is what an unplaced instance's placement IS. To know whether a
    /// row exists, compare against `Frame.translation((0*m, 0*m,
    /// 0*m))`; to know which node the registry is keyed by, ask
    /// `gauge_of`.
    ///
    /// This is the AUTHORED frame, not the solved one: a mated
    /// instance's world pose is its cluster frame composed with the
    /// solve's relative pose, which is `SolvedPoses.placement`.
    fn placement(&self, node: &NodeId) -> super::place::Frame {
        super::place::Frame(self.inner.placement(node.0))
    }

    /// The placement **registry itself**: every node with a recorded
    /// cluster frame, as node → frame.
    ///
    /// `placement` is total and answers the identity for a node with
    /// no row, which is what an unplaced instance's placement IS — so
    /// this is the door that distinguishes "placed at the identity"
    /// from "carries no frame of its own". A mated instance that is
    /// not its cluster's gauge is ABSENT here however it is posed:
    /// placement lives on the cluster, and its pose is solved.
    fn placements(&self, py: Python<'_>) -> PyResult<Py<PyDict>> {
        let out = PyDict::new(py);
        for (node, frame) in self.inner.placements() {
            out.set_item(NodeId(*node), super::place::Frame(*frame))?;
        }
        Ok(out.unbind())
    }

    /// The `(id, pin)` reference an instantiate node carries, or
    /// `None` for any other node.
    ///
    /// The read side of `Node.instantiate_part`: what a document says
    /// about which part it references, at which version — the value
    /// `mixed_pins` reports over and `update_reference` moves.
    fn reference(&self, node: &NodeId) -> Option<super::store::DocRef> {
        match self.inner.node(node.0) {
            Some(d::Node::InstantiatePart { doc_ref, .. }) => Some(super::store::DocRef(*doc_ref)),
            _ => None,
        }
    }

    /// The **interface record** an instantiate node carries, or `None`
    /// for any other node.
    ///
    /// Empty for a directly-authored instance — an authored instance
    /// crosses nothing. A non-empty record is minted only by the
    /// `split` that observed declarations crossing its cut.
    fn interface(&self, node: &NodeId) -> Option<super::refactor::InterfaceRecord> {
        match self.inner.node(node.0) {
            Some(d::Node::InstantiatePart { interface, .. }) => {
                Some(super::refactor::InterfaceRecord(interface.clone()))
            }
            _ => None,
        }
    }

    /// Insert a node and return its minted id — the common case,
    /// spelled without the intermediate `DocEdit`.
    fn insert(&mut self, py: Python<'_>, node: &Node) -> PyResult<NodeId> {
        let tol = Tol::witness();
        let edit = d::DocEdit::InsertNode {
            node: node.inner.clone(),
        };
        let applied = d::apply(&self.inner, &edit, tol).map_err(|err| edit_err(py, &err))?;
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

    /// Declare ONE inspected finding: insert a `Declare` node with
    /// its pair and return the node's id, for `Node.boolean`'s
    /// `declare=` input — the detect/declare protocol's declare arm
    /// (SELECT-DESIGN §3). Sugar over the same vocabulary
    /// `Node.declare` constructs; nothing here detects — findings
    /// reach this door as VALUES the caller already inspected (the
    /// ruled no-fusion boundary).
    fn declare(
        &mut self,
        py: Python<'_>,
        finding: &super::flush::FlushFinding,
    ) -> PyResult<NodeId> {
        let tol = Tol::witness();
        let (doc, id) = pncad::select::declare(&self.inner, &finding.0, tol)
            .map_err(|err| declare_err(py, &err))?;
        self.inner = doc;
        Ok(NodeId(id))
    }

    /// Declare a SET of inspected findings in one `Declare` node —
    /// the many-pair case (the boundary is fusion, not arity). Same
    /// contract as `declare`; an EMPTY list refuses (`no_findings`)
    /// rather than inserting a pretend-declaration.
    fn declare_all(
        &mut self,
        py: Python<'_>,
        findings: Vec<super::flush::FlushFinding>,
    ) -> PyResult<NodeId> {
        let tol = Tol::witness();
        let kernel: Vec<pncad::select::FlushFinding> = findings.into_iter().map(|f| f.0).collect();
        let (doc, id) = pncad::select::declare_all(&self.inner, &kernel, tol)
            .map_err(|err| declare_err(py, &err))?;
        self.inner = doc;
        Ok(NodeId(id))
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
    /// (the schema-v4 doors, via the curated façade).
    ///
    /// The Python wrapper holds only the CURRENT document — its edit
    /// history lives in the Rust values it discarded — so the file is
    /// a snapshot with an empty edit log. That is a complete, loadable
    /// document; the GUI's edit-log-bearing files load through the
    /// same `load` door.
    fn save(&self, py: Python<'_>) -> PyResult<String> {
        let tol = Tol::witness();
        d::save(&self.inner, &[], tol).map_err(|err| persist_err(py, &err))
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
/// Rust has ONE `BooleanOp` — the kernel enum the recipe node carries
/// — and this is its binding. The mirror exists because `#[pyclass]`
/// cannot be attached to a type from another crate, so a python-side
/// copy is forced; the obligation it owes the kernel is that every
/// kernel operation has a member here, which
/// [`_binds_every_kernel_operation`] is what enforces.
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

/// Every kernel operation has a member on the python mirror.
///
/// The direction is the load-bearing one. `to_document` matches on
/// `Self` — a closed local enum — so it says nothing about the kernel
/// growing; an operation added there would leave the python surface
/// silently short of it. This match is over the KERNEL enum, so that
/// addition breaks this build and the binding must be written.
///
/// It is never called: a type-checked match is the whole product, and
/// the leading underscore is what says so.
const fn _binds_every_kernel_operation(kernel: d::BooleanOp) -> BooleanOp {
    match kernel {
        d::BooleanOp::Union => BooleanOp::Union,
        d::BooleanOp::Intersect => BooleanOp::Intersect,
        d::BooleanOp::Subtract => BooleanOp::Subtract,
    }
}

/// The rigid placement of a sketch plane in 3-space — the kernel's
/// `profile::SketchPlane`, crossing as a VALUE.
///
/// Sketch (x, y) maps to world `origin + x·u + y·v`, and the plane's
/// NORMAL is `u × v` — which is the direction `Node.extrude` runs, so
/// the plane is what chooses an extrusion's axis.
///
/// The three named planes are the cyclic frames x→y→z→x: `xy` (normal
/// +z), `yz` (u = ŷ, v = ẑ, normal +x), `zx` (u = ẑ, v = x̂, normal
/// +y) — the same convention the demo tour's letterform captions
/// speak ("a yz sketch extruded +x").
///
/// RIGIDITY IS AN UNCHECKED CONVENTION, exactly as in Rust: nothing
/// verifies that `u` and `v` are unit and perpendicular. A non-rigid
/// frame yields a well-defined SKEWED sketch, not poison — the
/// kernel's tier-3 geometric validation is what certifies a body at
/// rest. The binding deliberately adds no orthogonality predicate of
/// its own: one semantics, two host languages.
#[pyclass(frozen, module = "pncad", from_py_object)]
#[derive(Clone, Copy)]
pub(crate) struct SketchPlane(pub(crate) pncad::profile::SketchPlane<f64>);

#[pymethods]
impl SketchPlane {
    /// The world xy-plane: u = x̂, v = ŷ, normal = +ẑ.
    #[staticmethod]
    fn xy() -> Self {
        Self(pncad::profile::SketchPlane::xy())
    }

    /// The world yz-plane: u = ŷ, v = ẑ, normal = +x̂.
    #[staticmethod]
    fn yz() -> Self {
        Self(pncad::profile::SketchPlane::yz())
    }

    /// The world zx-plane: u = ẑ, v = x̂, normal = +ŷ.
    #[staticmethod]
    fn zx() -> Self {
        Self(pncad::profile::SketchPlane::zx())
    }

    /// The plane through `origin` spanned by `u` and `v`.
    ///
    /// `origin` is dimensioned (`Length`s); `u` and `v` are
    /// dimensionless direction triples. Rigidity is the caller's
    /// unchecked convention — see the class docs.
    #[staticmethod]
    fn from_frame(
        origin: (
            super::quantity::Length,
            super::quantity::Length,
            super::quantity::Length,
        ),
        u: (f64, f64, f64),
        v: (f64, f64, f64),
    ) -> Self {
        Self(pncad::profile::SketchPlane::from_frame(
            pncad::authoring::p3::<f64>(
                origin.0.0.meters(),
                origin.1.0.meters(),
                origin.2.0.meters(),
            ),
            pncad::authoring::v3(u.0, u.1, u.2),
            pncad::authoring::v3(v.0, v.1, v.2),
        ))
    }

    /// The plane's origin — sketch (0, 0) in world space.
    ///
    /// The four accessors READ the frame back, they never recompute
    /// it: `from_frame(o, u, v)` round-trips through them exactly, and
    /// `normal` is the third placement column `from_frame` filled with
    /// u × v. Same four doors as Rust's `SketchPlane` (one
    /// vocabulary).
    #[getter]
    fn origin(
        &self,
    ) -> (
        super::quantity::Length,
        super::quantity::Length,
        super::quantity::Length,
    ) {
        let o = self.0.origin();
        let len = |v: f64| super::quantity::Length(pncad::quantity::Length::from_meters(v));
        (len(o.x), len(o.y), len(o.z))
    }

    /// The world direction sketch +x runs — a dimensionless triple.
    #[getter]
    fn u(&self) -> (f64, f64, f64) {
        let u = self.0.u();
        (u.x, u.y, u.z)
    }

    /// The world direction sketch +y runs — a dimensionless triple.
    #[getter]
    fn v(&self) -> (f64, f64, f64) {
        let v = self.0.v();
        (v.x, v.y, v.z)
    }

    /// The plane's normal, u × v — the direction `Node.extrude` runs.
    #[getter]
    fn normal(&self) -> (f64, f64, f64) {
        let n = self.0.normal();
        (n.x, n.y, n.z)
    }

    /// BIT-exact frame equality — Rust's `SketchPlane::bit_eq`,
    /// crossing unchanged (the `Doc.bit_eq` precedent, spec D7):
    /// `0.0` and `-0.0` are different planes here.
    ///
    /// Bit equality is the only equality this value can honestly
    /// offer: a sketch plane carries no ε, and "the same plane up to
    /// tolerance" is a geometric question the kernel answers about
    /// BODIES at tier 3. Two planes equal here place every sketch
    /// point identically.
    fn __eq__(&self, other: &Self) -> bool {
        self.0.bit_eq(&other.0)
    }

    /// Consistent with [`Self::__eq__`] BY CONSTRUCTION: it hashes the
    /// same twelve bit patterns the comparison reads, so bit-equal
    /// planes hash equal and `-0.0` keeps its own bucket.
    fn __hash__(&self) -> u64 {
        use std::hash::{Hash, Hasher};
        let o = self.0.origin();
        let (u, v, n) = (self.0.u(), self.0.v(), self.0.normal());
        let mut h = std::hash::DefaultHasher::new();
        for c in [o.x, o.y, o.z, u.x, u.y, u.z, v.x, v.y, v.z, n.x, n.y, n.z] {
            c.to_bits().hash(&mut h);
        }
        h.finish()
    }

    fn __repr__(&self) -> String {
        let o = self.0.placement.translation;
        let (u, v) = (self.0.placement.linear.c0, self.0.placement.linear.c1);
        format!(
            "SketchPlane(origin=({}, {}, {}), u=({}, {}, {}), v=({}, {}, {}))",
            o.x, o.y, o.z, u.x, u.y, u.z, v.x, v.y, v.z
        )
    }
}

/// The plane a sketch node sits on, from the mutually exclusive
/// `plane=` / `elevation=` pair.
///
/// ONE lowering for both spellings: `elevation` is the xy sugar it has
/// always been — the xy-plane translated up z — so there is a single
/// place where a sketch plane is constructed, and no way for the two
/// spellings to drift. Passing both is a boundary `TypeError`: the
/// author asked for two different planes and the kernel is fail-loud,
/// so nothing is silently preferred.
fn sketch_plane(
    plane: Option<SketchPlane>,
    elevation: Option<super::quantity::Length>,
) -> PyResult<pncad::profile::SketchPlane<f64>> {
    match (plane, elevation) {
        (Some(_), Some(_)) => Err(pyo3::exceptions::PyTypeError::new_err(
            "plane= and elevation= name the sketch plane two different ways; \
             pass exactly one (elevation is the xy-plane sugar)",
        )),
        (Some(p), None) => Ok(p.0),
        (None, elevation) => {
            let z = elevation.map_or(0.0, |e| e.0.meters());
            Ok(pncad::profile::SketchPlane::from_frame(
                pncad::authoring::p3::<f64>(0.0, 0.0, z),
                pncad::authoring::v3(1.0, 0.0, 0.0),
                pncad::authoring::v3(0.0, 1.0, 0.0),
            ))
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
    /// A closed polygonal sketch on a sketch plane.
    ///
    /// The plane is named either way, never both: `plane=` is a
    /// `SketchPlane` (any rigid frame, including the named `yz`/`zx`),
    /// `elevation=` is the xy sugar — the world xy-plane, that far up
    /// z. The default is the xy-plane itself.
    ///
    /// Coordinates arrive as typed `Length`s, so a bare number
    /// is a boundary refusal rather than an ambiguous unit; they are
    /// the sketch's own (x, y), which `plane` maps into the world.
    ///
    /// `elevation` earns its keep because the kernel is fail-loud
    /// about coincidence: it never INFERS that two faces are the same
    /// face, so two solids merely touching on a shared plane are
    /// refused (the `undeclared_contact` menu) until the author
    /// declares the contact. Authoring a genuine Boolean therefore
    /// needs solids that interpenetrate, which needs sketches at
    /// different heights — or the detect/declare protocol
    /// (`Evaluation.find_flush_candidates` → `Doc.declare_all`).
    #[staticmethod]
    #[pyo3(signature = (points, elevation=None, plane=None))]
    fn polygon(
        py: Python<'_>,
        points: Vec<(super::quantity::Length, super::quantity::Length)>,
        elevation: Option<super::quantity::Length>,
        plane: Option<SketchPlane>,
    ) -> PyResult<Self> {
        let plane = sketch_plane(plane, elevation)?;
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

    /// The profile a PATHS loop authors, on a sketch plane.
    ///
    /// `plane=` / `elevation=` mean exactly what they mean on
    /// [`Node::polygon`], through the same one lowering, and are
    /// mutually exclusive for the same reason.
    ///
    /// The node is built from the loop's RECORDED program — the same
    /// verbs, with the same authored arguments, that the chain wrote
    /// — so what Python authored and what the document replays are
    /// one program, not two spellings of one shape.
    ///
    /// `outline` is ONE loop or a LIST of them — a plate with holes is
    /// `[outer, hole, hole]`, in that order (`ProfileProgram.loops` is
    /// a `Vec`, and this is the whole of it).
    ///
    /// Nothing about the loop SET is pre-checked. Which loop is outer,
    /// whether the holes nest, whether two loops cross — all of that
    /// is `Profile::validate`'s work, and it fires as the kernel's own
    /// typed refusal at `evaluate`. The binding's only job is that the
    /// loops arrive in the order they were written.
    #[staticmethod]
    #[pyo3(signature = (outline, elevation=None, plane=None))]
    fn profile(
        py: Python<'_>,
        outline: &Bound<'_, PyAny>,
        elevation: Option<super::quantity::Length>,
        plane: Option<SketchPlane>,
    ) -> PyResult<Self> {
        let plane = sketch_plane(plane, elevation)?;
        // ONE loop or a sequence of them, and nothing else: a value
        // that is neither is `extract`'s own `TypeError`, so a
        // stringly-typed or numeric argument still refuses at the
        // boundary rather than being iterated into nonsense.
        let loops = match outline.cast::<super::path::ClosedLoop>() {
            Ok(one) => vec![super::path::loop_program(py, &one.borrow())?],
            Err(_) => {
                let many: Vec<PyRef<'_, super::path::ClosedLoop>> = outline.extract()?;
                many.iter()
                    .map(|l| super::path::loop_program(py, l))
                    .collect::<PyResult<Vec<_>>>()?
            }
        };
        Ok(Self {
            inner: d::Node::Profile(d::ProfileProgram { plane, loops }),
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

    /// **A solid tube** — a ring torus, or an elbow of one, from its
    /// INTENT parameters.
    ///
    /// `spine` is a `Node.datum_axis`: its origin is the tube's
    /// centre and its direction is the spine axis, both used, both
    /// stored EXACTLY. `u_ref` is the reference direction the
    /// window's angles are measured from — a dimensionless triple,
    /// matching `SlotId::Direction`, exactly as `Node.datum_axis`
    /// takes its own direction.
    ///
    /// # This is not `Node.revolve` of a circle
    ///
    /// It would build a different body. A revolve reconstructs its
    /// minor radius through the profile's bulge arithmetic; this door
    /// stores the number you gave it, so `minor_radius` comes back
    /// out of the body bit for bit.
    ///
    /// # Nothing is pre-checked
    ///
    /// A non-unit axis or reference direction, a pair that is not
    /// perpendicular, a degenerate or reversed window, a window
    /// reaching one full period (say `TubeWindow.full()` instead),
    /// and the ring-torus convention `R > r > 0` — every one of those
    /// is the kernel's own typed refusal at `evaluate`, tagged
    /// `tube`. There is no wall argument: a tube with a wall is
    /// `Node.hollow_tube`, a different node kind.
    #[staticmethod]
    fn tube(
        py: Python<'_>,
        spine: &NodeId,
        u_ref: (f64, f64, f64),
        major_radius: &super::quantity::Length,
        window: &TubeWindow,
        minor_radius: &super::quantity::Length,
    ) -> PyResult<Self> {
        Ok(Self {
            inner: d::Node::Tube {
                spine: spine.0,
                u_ref: u_ref_expr(py, u_ref)?,
                major_radius: literal(py, major_radius.0.meters(), d::Dimension::Length)?,
                window: window.inner.clone(),
                minor_radius: literal(py, minor_radius.0.meters(), d::Dimension::Length)?,
            },
        })
    }

    /// **A hollow tube** — `Node.tube`'s sibling with a WALL.
    ///
    /// `minor_radius` is the OUTER cross-sectional radius and `wall`
    /// the thickness; the inner wall stores `minor_radius - wall`,
    /// one subtraction of your own two numbers, so you recover it by
    /// writing the same subtraction.
    ///
    /// **`wall` is REQUIRED and has no default.** Hollowness is
    /// spelled by which door you call, not by whether you passed an
    /// argument: a solid tube and a hollow one are different
    /// artifacts — a full disc against an annulus — and there is no
    /// `wall=None` that quietly turns this into `Node.tube`.
    ///
    /// A `TubeWindow.full()` builds a torus SHELL, whose cavity is a
    /// void; an arc builds an open elbow of annular section.
    ///
    /// # Nothing is pre-checked, and the wall least of all
    ///
    /// Everything `Node.tube` refuses, this refuses identically, plus
    /// three verdicts only this door can raise: the thickness is not
    /// positive at tolerance, `minor_radius - wall` is not a bore,
    /// and — the one neither of the others can see — the gap between
    /// the two radii the body would STORE collapses, because at a
    /// large outer radius the subtraction rounds the inner radius
    /// back onto the outer. All three are the kernel's, decided
    /// before anything is minted, and all three arrive tagged `tube`.
    #[staticmethod]
    fn hollow_tube(
        py: Python<'_>,
        spine: &NodeId,
        u_ref: (f64, f64, f64),
        major_radius: &super::quantity::Length,
        window: &TubeWindow,
        minor_radius: &super::quantity::Length,
        wall: &super::quantity::Length,
    ) -> PyResult<Self> {
        Ok(Self {
            inner: d::Node::HollowTube {
                spine: spine.0,
                u_ref: u_ref_expr(py, u_ref)?,
                major_radius: literal(py, major_radius.0.meters(), d::Dimension::Length)?,
                window: window.inner.clone(),
                minor_radius: literal(py, minor_radius.0.meters(), d::Dimension::Length)?,
                wall: literal(py, wall.0.meters(), d::Dimension::Length)?,
            },
        })
    }

    /// A skinned solid through two or more section profiles.
    ///
    /// `profiles` are upstream profile nodes IN SKIN ORDER — order is
    /// data, and reversing it reverses the produced surface's
    /// v-direction. `v_degree` is the v-direction interpolation
    /// degree, a COUNT (structural material, D3), so it crosses as
    /// `Expr::count` and not as a continuous literal.
    ///
    /// There is no placement argument, and that is the document
    /// design rather than a missing one: each section rides its OWN
    /// profile's sketch plane, so a stack is authored by giving the
    /// sections different planes (`elevation=`, or `plane=` for
    /// anything else).
    ///
    /// Nothing is pre-checked here. An empty or one-element list, a
    /// degree outside `1 ≤ d ≤ len − 1`, a section that is not a
    /// profile, sections whose loops do not correspond — every one of
    /// those is the kernel's own typed refusal, arriving from `insert`
    /// or from `evaluate` exactly where the Rust surface raises it.
    #[staticmethod]
    fn loft(profiles: Vec<NodeId>, v_degree: i64) -> Self {
        Self {
            inner: d::Node::Loft {
                profiles: profiles.iter().map(|p| p.0).collect(),
                v_degree: d::Expr::count(v_degree),
            },
        }
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

    /// A datum plane: a point and a normal.
    ///
    /// The origin is dimensioned (`Length`); the normal is a
    /// dimensionless triple, matching `SlotId::Normal`'s `Scalar`, and
    /// is UNNORMALIZED — the evaluator normalizes it, or refuses a
    /// degenerate one loudly. This is the tool a `Node.split` cuts
    /// with.
    #[staticmethod]
    fn datum_plane(
        py: Python<'_>,
        origin: (
            super::quantity::Length,
            super::quantity::Length,
            super::quantity::Length,
        ),
        normal: (f64, f64, f64),
    ) -> PyResult<Self> {
        let origin = [
            literal(py, origin.0.0.meters(), d::Dimension::Length)?,
            literal(py, origin.1.0.meters(), d::Dimension::Length)?,
            literal(py, origin.2.0.meters(), d::Dimension::Length)?,
        ];
        let normal = [
            literal(py, normal.0, d::Dimension::Scalar)?,
            literal(py, normal.1, d::Dimension::Scalar)?,
            literal(py, normal.2, d::Dimension::Scalar)?,
        ];
        Ok(Self {
            inner: d::Node::Datum(d::Datum::Plane { origin, normal }),
        })
    }

    /// Constant-radius rolling-ball blends on a SELECTION of
    /// `target`'s edges.
    ///
    /// `selection` is edge names as text — the strings
    /// `Evaluation.all_edges` answers with. A name is CARRIED, not
    /// composed and not read: the text is an opaque identifier whose
    /// internal structure is not API (see [`name_text`]), so there is
    /// no name-building vocabulary in Python and no supported way to
    /// filter a materialized set. There is deliberately no "every
    /// edge" spelling either.
    ///
    /// THE SELECTION FREEZES, exactly as in Rust: it is a commitment
    /// as of the evaluation you read it from, and an upstream edit
    /// that adds edges does NOT extend it. Materialize, store, and
    /// the recipe means what it said.
    ///
    /// Nothing is pre-checked beyond the text being a name at all. An
    /// EMPTY selection (`fillet_selection_empty`), a name that
    /// resolves to nothing (`fillet_selection_resolve`), a name of the
    /// wrong kind (`fillet_selection_kind`), a tangential edge the
    /// roller cannot enter (`fillet`) — every one of those is the
    /// kernel's own typed refusal at `evaluate`.
    ///
    /// The node is built through Rust's `Node::fillet`, the one
    /// construction door, so the stored set is canonical (sorted,
    /// deduplicated) and two recipes that select the same edges are
    /// bit-identical whatever order Python listed them in.
    #[staticmethod]
    fn fillet(
        py: Python<'_>,
        target: &NodeId,
        radius: &super::quantity::Length,
        selection: Vec<String>,
    ) -> PyResult<Self> {
        let radius = literal(py, radius.0.meters(), d::Dimension::Length)?;
        let selection = selection
            .iter()
            .map(|text| name_from_text(text))
            .collect::<PyResult<Vec<_>>>()?;
        Ok(Self {
            inner: d::Node::fillet(target.0, radius, selection),
        })
    }

    /// Equal-setback flat chamfers on a SELECTION of `target`'s
    /// edges — `Node.fillet`'s twin.
    ///
    /// `selection` is edge names as text, exactly as `Node.fillet`
    /// takes them: the strings `Evaluation.all_edges` answers with,
    /// CARRIED and never composed, with no "every edge" spelling and
    /// no way to filter a materialized set. THE SELECTION FREEZES, in
    /// the same sense and for the same reason — read `Node.fillet`.
    ///
    /// `distance` is the SETBACK along each support from the edge,
    /// not a radius. That is the one thing this door does not share
    /// with its twin.
    ///
    /// Nothing is pre-checked beyond the text being a name at all. An
    /// EMPTY selection (`chamfer_selection_empty`), a name that
    /// resolves to nothing (`chamfer_selection_resolve`), a name of
    /// the wrong kind (`chamfer_selection_kind`), an edge whose two
    /// supports are not both planes (`chamfer`) — every one of those
    /// is the kernel's own typed refusal at `evaluate`.
    ///
    /// The node is built through Rust's `Node::chamfer`, the one
    /// construction door, so the stored set is canonical.
    #[staticmethod]
    fn chamfer(
        py: Python<'_>,
        target: &NodeId,
        distance: &super::quantity::Length,
        selection: Vec<String>,
    ) -> PyResult<Self> {
        let distance = literal(py, distance.0.meters(), d::Dimension::Length)?;
        let selection = selection
            .iter()
            .map(|text| name_from_text(text))
            .collect::<PyResult<Vec<_>>>()?;
        Ok(Self {
            inner: d::Node::chamfer(target.0, distance, selection),
        })
    }

    /// Split a target body by a tool — today a `Node.datum_plane`.
    ///
    /// The value is a SPLIT, not a body: read it with `Value.split()`,
    /// which answers `(above, below)` with `None` for an empty side.
    /// A tool that is not a splitting surface, or a cut that produces
    /// nothing, refuses typed at `evaluate`.
    #[staticmethod]
    fn split(target: &NodeId, tool: &NodeId) -> Self {
        Self {
            inner: d::Node::Split {
                target: target.0,
                tool: tool.0,
            },
        }
    }

    /// A rigid placement of an upstream body.
    ///
    /// The convention is the kernel's, unchanged: rotate about
    /// `rotation_axis` THROUGH THE WORLD ORIGIN by `rotation_angle`,
    /// THEN translate. A pure translation is therefore written with
    /// any non-degenerate axis and a zero angle — the axis is
    /// normalized by the evaluator, and a zero-length one refuses
    /// (`degenerate_direction`) rather than being read as "no
    /// rotation".
    ///
    /// `translation` is dimensioned (`Length`s); the axis is a
    /// dimensionless triple, matching `SlotId::RotationAxis`'s
    /// `Scalar`; the angle is an `Angle`.
    #[staticmethod]
    fn transform(
        py: Python<'_>,
        input: &NodeId,
        translation: (
            super::quantity::Length,
            super::quantity::Length,
            super::quantity::Length,
        ),
        rotation_axis: (f64, f64, f64),
        rotation_angle: &super::quantity::Angle,
    ) -> PyResult<Self> {
        let translation = [
            literal(py, translation.0.0.meters(), d::Dimension::Length)?,
            literal(py, translation.1.0.meters(), d::Dimension::Length)?,
            literal(py, translation.2.0.meters(), d::Dimension::Length)?,
        ];
        let rotation_axis = [
            literal(py, rotation_axis.0, d::Dimension::Scalar)?,
            literal(py, rotation_axis.1, d::Dimension::Scalar)?,
            literal(py, rotation_axis.2, d::Dimension::Scalar)?,
        ];
        let rotation_angle = literal(py, rotation_angle.0.radians(), d::Dimension::Angle)?;
        Ok(Self {
            inner: d::Node::Transform {
                input: input.0,
                translation,
                rotation_axis,
                rotation_angle,
            },
        })
    }

    /// A Boolean of two upstream solids.
    ///
    /// `declare` names a `Declare` node whose coincidence pairs this
    /// boolean consumes — the DATA door for a declared contact.
    /// Without it the kernel never infers that two faces are the same
    /// face, so operands that merely touch refuse, and that refusal is
    /// the typed MENU: an
    /// `EvaluationError` with `kind == "undeclared_contact"` whose
    /// `finding` attribute carries the candidate declaration. The
    /// protocol that fills this argument is
    /// `Evaluation.find_flush_candidates` → inspect → `Node.declare`
    /// (or the `Doc.declare`/`Doc.declare_all` sugar) → this
    /// `declare=`.
    #[staticmethod]
    #[pyo3(signature = (op, a, b, declare=None))]
    fn boolean(op: BooleanOp, a: &NodeId, b: &NodeId, declare: Option<NodeId>) -> Self {
        Self {
            inner: d::Node::Boolean {
                op: op.to_document(),
                a: a.0,
                b: b.0,
                declare: declare.map(|d| d.0),
            },
        }
    }

    /// The `Declare` node built from INSPECTED findings — the
    /// detect/declare protocol's declare arm as a node constructor;
    /// its inserted id feeds `Node.boolean`'s `declare=` input.
    /// `Doc.declare`/`Doc.declare_all` are the insert-and-return-id
    /// sugar over this same vocabulary. Nothing here detects
    /// (SELECT-DESIGN §3's ruled no-fusion boundary: findings pass
    /// through your hands as values), and an EMPTY list refuses
    /// (`no_findings`) — an empty Declare records no intent.
    #[staticmethod]
    fn declare(py: Python<'_>, findings: Vec<super::flush::FlushFinding>) -> PyResult<Self> {
        let kernel: Vec<pncad::select::FlushFinding> = findings.into_iter().map(|f| f.0).collect();
        let node = pncad::select::declare_node(&kernel).map_err(|err| declare_err(py, &err))?;
        Ok(Self { inner: node })
    }

    /// **The group boolean** over a PARAMETRIC rule: one prototype,
    /// `count` placements stepped by `kind`, ONE body out — the union
    /// of the prototype at each placement.
    ///
    /// The value is an ordinary body, so every downstream door
    /// consumes it with no new arms; that is exactly what a `Pattern`
    /// node's plural `Instances` payload cannot do. Per-instance
    /// naming is the `Instance(i)` qualifier, ONE segment deep
    /// whatever the count.
    ///
    /// `count` crosses as a plain `int`, the structural-slot exception
    /// to the typed quantities (`Node.loft`'s `v_degree` precedent):
    /// a Count is an integer in the kernel's own expression language,
    /// not a dimensioned measurement, and there is no `Count` quantity
    /// to wrap it in.
    ///
    /// Disjointness is CERTIFIED, never declared: overlapping
    /// placements refuse typed at `evaluate` (`placements_uncertified`,
    /// naming the pair), and the certificate is
    /// sufficient-not-necessary — a touching-but-genuinely-disjoint
    /// arrangement refuses honestly rather than passing on a guess.
    ///
    /// An `explicit` rule brings its OWN placements, so pairing it
    /// with a count is the two-sources-of-truth state: it refuses here
    /// (`EditError`, `placement_rule_mismatch`) and
    /// `Node.placed_union_at` is its door.
    #[staticmethod]
    fn placed_union(
        py: Python<'_>,
        input: &NodeId,
        count: i64,
        kind: &super::place::PatternKind,
    ) -> PyResult<Self> {
        let node = d::Node::placed_union(input.0, d::Expr::count(count), kind.0.clone())
            .ok_or_else(|| {
                typed_err(
                    py,
                    ErrorClass::Edit,
                    "an explicit placement rule carries its own placements, so it has no \
                     count slot: use Node.placed_union_at",
                    &[(
                        "variant",
                        PyString::new(
                            py,
                            crate::tags::placement_rule_fault_tag(
                                &d::PlacementRuleFault::CountSpelling,
                            ),
                        )
                        .unbind()
                        .into_any(),
                    )],
                )
            })?;
        Ok(Self { inner: node })
    }

    /// **The group boolean** over LISTED absolute frames: one
    /// prototype placed at each of `frames`, unioned into ONE body.
    ///
    /// There is no count argument because the list IS the count — one
    /// number, one spelling. An EMPTY list is this rule's `count < 1`
    /// and refuses typed at `Doc.insert` (`empty_placement_list`), as
    /// does a non-finite (`non_finite_placement`) or improper
    /// (`improper_placement`) frame.
    #[staticmethod]
    fn placed_union_at(input: &NodeId, frames: Vec<super::place::Frame>) -> Self {
        Self {
            inner: d::Node::placed_union_at(input.0, frames.into_iter().map(|f| f.0).collect()),
        }
    }

    /// An **instance of another document's product**: a leaf whose
    /// material crosses the document seam rather than arriving from an
    /// upstream node.
    ///
    /// `reference` is the `(id, pin)` pair — which part, at which
    /// version. Cargo.lock semantics: an edit to the referenced
    /// document never retargets this reference, and moving the pin is
    /// its own recorded edit (`DocEdit.update_reference`, or
    /// `update_references` for every site at once).
    ///
    /// **No frame argument.** Placement lives on the CLUSTER, and the
    /// registry holding it is document data — an instance carries no
    /// frame of its own, which is what makes zero-anchor and
    /// multi-anchor states unrepresentable rather than merely refused.
    /// `DocEdit.set_placement` is the door that places one.
    ///
    /// The instance also carries no interface record: an AUTHORED
    /// instance crosses nothing, and a non-empty record is mintable
    /// only by the `split` that observed declarations crossing its
    /// cut. Read one back with `Doc.interface`.
    ///
    /// Evaluating this node needs a resolver — `evaluate(doc,
    /// resolver=workspace)`. Without one it refuses typed
    /// (`EvaluationError`, `kind == "part_no_resolver"`) rather than
    /// pretending the part is empty.
    #[staticmethod]
    fn instantiate_part(reference: &super::store::DocRef) -> Self {
        Self {
            inner: d::Node::instantiate_part(reference.0),
        }
    }

    /// A **mate** between two instances: ONE node carrying both the
    /// placement constraint and the contact declaration, so there is
    /// no second vocabulary to keep synced.
    ///
    /// `a` and `b` are instance-qualified stable references — an
    /// entity of one instance's product and an entity of the other's.
    /// They are name REFERENCES, not recipe edges: inserting a mate
    /// transfers no root, and under consuming edges a mate is an
    /// ordinary non-body root, denoting no body and ignored by the
    /// gather.
    ///
    /// `class_` is the declared contact class (trailing underscore:
    /// `class` is a Python keyword). How far each class gets is
    /// `class_admission` — ask it BEFORE authoring, because a class
    /// the solve folds may still mint nothing at the at-rest gate.
    ///
    /// `alignment` is the authored datum: which frames coincide, at
    /// which axis sense, with which clocking. It is AUTHORED data, not
    /// geometry read back — nothing checks it against the faces `a`
    /// and `b` name (issue #944), so a mate can solve cleanly and
    /// still be refuted at the gate.
    ///
    /// A dangling reference head is not refused here: the solve
    /// refuses typed naming it (`MateFault`, `mate_dangling_head`),
    /// which is the ratified dangling-reference semantics.
    #[staticmethod]
    fn mate(
        py: Python<'_>,
        a: &str,
        b: &str,
        class_: super::flush::ContactClass,
        alignment: &super::mate::Alignment,
    ) -> PyResult<Self> {
        Ok(Self {
            inner: d::Node::Mate {
                a: name_from_text(a)?,
                b: name_from_text(b)?,
                class: class_.to_kernel(py)?,
                alignment: alignment.0,
            },
        })
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

/// `-0.0` folded to `0.0`, every other value untouched — the
/// normalization a hash must apply wherever the equality it mirrors is
/// IEEE (`-0.0 == 0.0`).
fn fold_zero(v: f64) -> f64 {
    if v == 0.0 { 0.0 } else { v }
}

/// A named parameter's declared dimension and exact stored value
/// (guide §3.2): what `DocEdit.set_doc_param` writes.
///
/// Continuous values arrive as typed quantities, so the
/// dimension is carried by the constructor rather than guessed from a
/// bare float. A non-finite value is NOT pre-checked here — the edit
/// door refuses it typed (`non_finite_doc_param`), fail-loud where
/// the kernel refuses.
///
/// The constructors here author UNANNOTATED parameters: a parameter's
/// optional distribution (ERROR-DESIGN E1/E2) has no Python spelling
/// yet, so a document authored from Python declares none. One read
/// back from a `.pncad` file keeps whatever it carries — equality,
/// hashing and `repr` all see the annotation.
#[pyclass(frozen, module = "pncad", from_py_object)]
#[derive(Clone)]
pub(crate) struct DocParam(pub(crate) d::DocParam);

#[pymethods]
impl DocParam {
    /// A continuous Length parameter.
    #[staticmethod]
    fn length(value: &super::quantity::Length) -> Self {
        Self(d::DocParam::continuous(
            d::Dimension::Length,
            value.0.meters(),
        ))
    }

    /// A continuous Angle parameter.
    #[staticmethod]
    fn angle(value: &super::quantity::Angle) -> Self {
        Self(d::DocParam::continuous(
            d::Dimension::Angle,
            value.0.radians(),
        ))
    }

    /// A continuous dimensionless parameter.
    #[staticmethod]
    fn scalar(value: f64) -> Self {
        Self(d::DocParam::continuous(d::Dimension::Scalar, value))
    }

    /// An integer Count parameter (structural material, spec D3).
    #[staticmethod]
    fn count(value: i64) -> Self {
        Self(d::DocParam::Count { value })
    }

    /// Rust's `PartialEq`, mirrored — which is IEEE comparison of the
    /// stored value, NOT the bit comparison `DocParam::bit_eq` makes.
    /// Two spellings of zero are therefore the same parameter here
    /// and different parameters to `bit_eq`, exactly as in Rust; a
    /// NaN value (which the edit door refuses, so it never reaches a
    /// document) equals nothing, itself included.
    ///
    /// Dimension is part of the value: a Length 1 and a Scalar 1 are
    /// different parameters.
    fn __eq__(&self, other: &Self) -> bool {
        self.0 == other.0
    }

    /// Consistent with [`Self::__eq__`]: `-0.0` is folded to `0.0`
    /// before hashing, because the equality that fold mirrors says
    /// they are the same parameter, and a hash that split them would
    /// break the invariant Python dicts rely on. NaN hashes like any
    /// other bit pattern and simply never compares equal.
    fn __hash__(&self) -> u64 {
        use std::hash::{Hash, Hasher};
        let mut h = std::hash::DefaultHasher::new();
        match &self.0 {
            d::DocParam::Continuous {
                dim,
                value,
                distribution,
            } => {
                0u8.hash(&mut h);
                format!("{dim:?}").hash(&mut h);
                fold_zero(*value).to_bits().hash(&mut h);
                // The distribution is part of the parameter, so it is
                // part of the equality this hash mirrors — and it gets
                // the SAME `-0.0` fold the nominal gets, per field,
                // before its debug spelling is hashed. Without the
                // fold, `Band { lo: -0.0, .. }` and `Band { lo: 0.0,
                // .. }` compare EQUAL (Rust's `PartialEq` on `f64` is
                // IEEE) and hash APART, which is the one thing a
                // Python dict may not survive.
                format!("{:?}", distribution.map(d::Distribution::fold_signed_zeros)).hash(&mut h);
            }
            d::DocParam::Count { value } => {
                1u8.hash(&mut h);
                value.hash(&mut h);
            }
        }
        h.finish()
    }

    fn __repr__(&self) -> String {
        match &self.0 {
            d::DocParam::Continuous {
                dim,
                value,
                distribution: None,
            } => format!("DocParam({dim:?} {value})"),
            d::DocParam::Continuous {
                dim,
                value,
                distribution: Some(d),
            } => format!("DocParam({dim:?} {value} {d:?})"),
            d::DocParam::Count { value } => format!("DocParam(Count {value})"),
        }
    }
}

/// The VALUE half of a document parameter: what
/// `DocEdit.set_doc_param_value` writes into an ALREADY-DECLARED
/// parameter.
///
/// This is the safe "just change the number" spelling. `set_doc_param`
/// is create-or-replace and takes a whole `DocParam`, so using it to
/// move a value rebuilds the declaration from parts — and a parameter
/// read back from a file carrying a distribution (ERROR-DESIGN E1/E2)
/// loses it, silently, because Python has no way to spell the
/// annotation it would need to copy across. The value door carries the
/// declaration forward instead: the dimension and the annotation stay
/// exactly as the document has them.
///
/// Continuous values arrive as typed quantities for the same reason
/// `DocParam`'s do — except that here the dimension is NOT being
/// declared, it is being matched: the quantity says which unit the
/// number is in, and the parameter's own declaration is what rules.
#[pyclass(frozen, module = "pncad", from_py_object)]
#[derive(Clone)]
pub(crate) struct DocParamValue(pub(crate) d::DocParamValue);

#[pymethods]
impl DocParamValue {
    /// A continuous value in Length units.
    #[staticmethod]
    fn length(value: &super::quantity::Length) -> Self {
        Self(d::DocParamValue::Continuous(value.0.meters()))
    }

    /// A continuous value in Angle units.
    #[staticmethod]
    fn angle(value: &super::quantity::Angle) -> Self {
        Self(d::DocParamValue::Continuous(value.0.radians()))
    }

    /// A dimensionless continuous value.
    #[staticmethod]
    fn scalar(value: f64) -> Self {
        Self(d::DocParamValue::Continuous(value))
    }

    /// An exact integer, for a `Count` parameter.
    #[staticmethod]
    fn count(value: i64) -> Self {
        Self(d::DocParamValue::Count(value))
    }

    /// Rust's `PartialEq`, mirrored — IEEE on the stored number, so
    /// the two spellings of zero are the same value.
    fn __eq__(&self, other: &Self) -> bool {
        self.0 == other.0
    }

    /// Consistent with [`Self::__eq__`]: `-0.0` folds to `0.0`.
    fn __hash__(&self) -> u64 {
        use std::hash::{Hash, Hasher};
        let mut h = std::hash::DefaultHasher::new();
        match self.0 {
            d::DocParamValue::Continuous(v) => {
                0u8.hash(&mut h);
                fold_zero(v).to_bits().hash(&mut h);
            }
            d::DocParamValue::Count(v) => {
                1u8.hash(&mut h);
                v.hash(&mut h);
            }
        }
        h.finish()
    }

    fn __repr__(&self) -> String {
        format!("DocParamValue({})", self.0)
    }
}

/// A single edit to a document — the ONE API surface shared by the
/// GUI, the bindings, macro recording and headless tests.
///
/// Five edits are exposed today: `insert_node`, `delete_node`,
/// `set_tolerance`, `set_doc_param` and
/// `bind_count_param`, the structural-slot edit narrowed to the Count
/// slot and a parameter reference. The remaining variants (continuous
/// slot edits, re-witnessing, appearance, rebinds, expression paths)
/// are mechanical additions once the surface they need is curated —
/// each waits on an expression vocabulary, which is the reason the
/// count edit crosses in this narrowed form rather than as the
/// general door. Tracked as named gaps in
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

    /// Write a new VALUE into an already-declared document parameter,
    /// keeping its declaration — its dimension and, if a file gave it
    /// one, its distribution (ERROR-DESIGN E1/E2).
    ///
    /// **Prefer this over `set_doc_param` whenever the parameter
    /// already exists.** `set_doc_param` is create-or-replace: handed
    /// a `DocParam` rebuilt from a dimension and a number — the only
    /// shape Python can spell — it REPLACES the declaration, and any
    /// annotation the parameter carried is gone with no refusal and no
    /// diagnostic. This door cannot do that, because it never names a
    /// declaration at all.
    ///
    /// Refuses typed on a name the document does not declare
    /// (`doc_param_not_declared`) and on a kind mismatch — a count for
    /// a continuous parameter or the reverse
    /// (`doc_param_value_kind_mismatch`), which is a redeclaration and
    /// belongs to the other door.
    #[staticmethod]
    fn set_doc_param_value(name: &ParamName, value: &DocParamValue) -> Self {
        Self {
            inner: d::DocEdit::SetDocParamValue {
                name: name.0.clone(),
                value: value.0,
            },
        }
    }

    /// Bind `node`'s STRUCTURAL count slot to the document parameter
    /// `name` — the edit that makes a pattern's or a group's
    /// replication count a named, editable number.
    ///
    /// With the binding in place, one `set_doc_param` re-counts the
    /// placements and recomputes exactly the nodes downstream of the
    /// count; without it a count is a literal and each new number is a
    /// new document.
    ///
    /// # Why the door is this narrow
    ///
    /// The underlying edit replaces a slot's EXPRESSION, and its two
    /// remaining degrees of freedom are the slot and the expression
    /// tree. Both stay closed here: the slot is `Count` — the only
    /// structural slot there is — and the expression is a parameter
    /// reference at `Count` dimension, so no expression algebra
    /// crosses and there is no way to aim this edit at a continuous
    /// slot (the refusal the general door would need). What DOES stay
    /// live is every refusal the edit itself carries: a node with no
    /// count slot, an unknown parameter, a parameter of the wrong
    /// dimension — each arrives as its own typed `EditError`.
    #[staticmethod]
    fn bind_count_param(node: &NodeId, name: &ParamName) -> Self {
        Self {
            inner: d::DocEdit::SetStructuralParam {
                node: node.0,
                slot: d::SlotId::Count,
                expr: d::Expr::param(name.0.clone(), d::Dimension::Count),
            },
        }
    }

    /// Set the document's ordered **product roots** outright.
    ///
    /// THE designate/undesignate door: one TOTAL edit rather than
    /// partial add/remove arms, so the product's solid order is always
    /// stated rather than inferred from an edit sequence. What the
    /// roots name is what `product` and `assemble` gather, in this
    /// order.
    ///
    /// Validator-checked like any other apply. The four root
    /// invariants refuse under their own tags — `root_not_live`,
    /// `root_duplicate`, `root_ancestor` (one root upstream of
    /// another would gather its material twice), `root_uncovered` (a
    /// live node reaching no root is a silently dead subgraph) — on
    /// `EditError`, because which invariant broke is what a caller
    /// branches on.
    #[staticmethod]
    fn set_roots(roots: Vec<NodeId>) -> Self {
        Self {
            inner: d::DocEdit::SetRoots {
                roots: roots.iter().map(|n| n.0).collect(),
            },
        }
    }

    /// Place an instance's **cluster**.
    ///
    /// The target is the instantiate node whose cluster moves, and the
    /// frame REPLACES whatever was recorded (the identity, if nothing
    /// was). Placement is per-cluster, not per-instance: an instance
    /// coupled to others by mates shares their frame, and setting it
    /// through any member places the whole cluster — `gauge_of` says
    /// which node the registry is actually keyed by.
    ///
    /// Refuses typed on `EditError`: `placement_on_non_instance`,
    /// `non_finite_placement`, `improper_placement` (determinant ≤ 0
    /// — a mirror is not a placement).
    #[staticmethod]
    fn set_placement(node: &NodeId, frame: &super::place::Frame) -> Self {
        Self {
            inner: d::DocEdit::SetPlacement {
                node: node.0,
                frame: frame.0,
            },
        }
    }

    /// Move ONE instance's pin to a new version of the same document.
    ///
    /// The id does not move: identity and version stay distinct, and
    /// this edit touches only the version half. `update_references` is
    /// the whole-document elaboration over this primitive, and
    /// `Workspace.update_to_store` the one that computes the new pin
    /// from disk.
    ///
    /// **The new pin is RECIPE DATA, not a resolution.** `apply` does
    /// not reach across the document seam — it has no resolver and no
    /// store — so a pin naming content that does not exist is accepted
    /// HERE and refused at EVALUATION, through the seam vocabulary
    /// that names both pins (`part_pin_mismatch`, `part_unresolved`).
    /// Checking at the edit door would make the edit's meaning depend
    /// on which store was mounted when it was recorded, which is
    /// exactly what a recorded, replayable log must not carry.
    ///
    /// Refuses typed: `update_on_non_instance`, and `pin_unchanged`
    /// when the site already names that version — an update that
    /// records nothing is never reported as a success.
    #[staticmethod]
    fn update_reference(node: &NodeId, new_pin: &super::store::ContentPin) -> Self {
        Self {
            inner: d::DocEdit::UpdateReference {
                node: node.0,
                new_pin: new_pin.0,
            },
        }
    }
}

/// A loaded document: what the persistence door
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
            maintenance: Vec::new(),
        }
    }

    /// The snapshot exactly as saved, before replay.
    #[getter]
    fn snapshot(&self) -> Doc {
        Doc {
            inner: self.snapshot.clone(),
            maintenance: Vec::new(),
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

/// Parse, validate, and replay a saved document.
///
/// Every refusal is a typed `PersistError` carrying the arm's stable
/// `variant` tag — bad header, unknown schema, unparseable body, a
/// snapshot or edit log the shared validator rejects, an ε conflict.
#[pyfunction]
pub(crate) fn load(py: Python<'_>, text: &str) -> PyResult<Loaded> {
    let tol = Tol::witness();
    let loaded = d::load(text, tol).map_err(|err| persist_err(py, &err))?;
    Ok(Loaded {
        snapshot: loaded.snapshot,
        doc: loaded.doc,
        edit_count: loaded.edits.len(),
    })
}

/// A reference direction's three components as Scalar literals — the
/// spelling `Node.datum_axis` gives its own direction, shared so the
/// two cannot drift.
fn u_ref_expr(py: Python<'_>, u: (f64, f64, f64)) -> PyResult<[d::Expr; 3]> {
    Ok([
        literal(py, u.0, d::Dimension::Scalar)?,
        literal(py, u.1, d::Dimension::Scalar)?,
        literal(py, u.2, d::Dimension::Scalar)?,
    ])
}

/// **A tube's traversed window** — the kernel's `TubeWindow`, crossing
/// as a VALUE with two spellings and no third.
///
/// A class rather than an optional pair of angles, and that is the
/// design rather than ceremony: `window=None` would make "the full
/// ring" the shape you get by not saying anything, when it is one of
/// two things a caller must actually choose between. `Full` is a
/// spelling, not an omission, and the kernel refuses an arc that
/// reaches one full period precisely so the two never blur.
#[pyclass(module = "pncad", frozen)]
pub(crate) struct TubeWindow {
    inner: d::TubeWindow,
}

#[pymethods]
impl TubeWindow {
    /// The full ring — the donut.
    #[staticmethod]
    fn full() -> Self {
        Self {
            inner: d::TubeWindow::Full,
        }
    }

    /// The arc from `t0` to `t1` about the spine axis, measured from
    /// the reference direction, right-handed. Wedge caps close the
    /// ends.
    ///
    /// Both angles cross as `Angle`, the same dimensioned quantity
    /// `Node.revolve` takes its sweep angle as — an angle is never a
    /// bare float on this surface.
    ///
    /// Nothing is checked here. A reversed or degenerate span, and a
    /// span reaching one full period (which must say `full()`), are
    /// the kernel's own typed refusals at `evaluate`.
    #[staticmethod]
    fn arc(
        py: Python<'_>,
        t0: &super::quantity::Angle,
        t1: &super::quantity::Angle,
    ) -> PyResult<Self> {
        Ok(Self {
            inner: d::TubeWindow::Arc {
                t0: literal(py, t0.0.radians(), d::Dimension::Angle)?,
                t1: literal(py, t1.0.radians(), d::Dimension::Angle)?,
            },
        })
    }

    fn __repr__(&self) -> String {
        match &self.inner {
            d::TubeWindow::Full => "TubeWindow.full()".to_owned(),
            d::TubeWindow::Arc { .. } => "TubeWindow.arc(...)".to_owned(),
        }
    }
}

/// Every kernel window spelling has a constructor on the python
/// mirror, enforced in the load-bearing direction (the `BooleanOp`
/// mirror's argument, verbatim): the match is over the KERNEL enum, so
/// a variant added there breaks this build rather than leaving the
/// python surface silently short of it. Never called.
const fn _binds_every_kernel_window(kernel: &d::TubeWindow) -> &'static str {
    match kernel {
        d::TubeWindow::Full => "full",
        d::TubeWindow::Arc { .. } => "arc",
    }
}

/// Register the document surface on the module.
pub(crate) fn register(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<NodeId>()?;
    m.add_class::<Doc>()?;
    m.add_class::<DocEdit>()?;
    m.add_class::<ParamName>()?;
    m.add_class::<DocParam>()?;
    m.add_class::<DocParamValue>()?;
    m.add_class::<Node>()?;
    m.add_class::<SketchPlane>()?;
    m.add_class::<BooleanOp>()?;
    m.add_class::<TubeWindow>()?;
    m.add_class::<Loaded>()?;
    m.add_function(wrap_pyfunction!(load, m)?)?;
    Ok(())
}
