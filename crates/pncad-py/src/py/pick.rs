//! **Picking: the fourth door onto a name** (LIB-B-PICKING).
//!
//! `crate::select`'s first three doors answer "which entities match
//! this shape" (`select`), "where is this named entity"
//! (`face_frame` and friends) and "how many does it denote"
//! (`denotation`). This one answers **"what is under this ray"** —
//! and it answers in the same currency all of them speak, an opaque
//! `StableName` text, so a pick feeds `Node.fillet` unread exactly as
//! a selection does.
//!
//! # The pairing, and why there is no raw target here
//!
//! Arena keys collide numerically across sibling nodes, so a pick
//! target whose `(node, body)` is not the pair its mesh was
//! tessellated from does not error: it inverts the hit triangle's
//! face key against the WRONG node's table and answers a **plausible,
//! confidently wrong name** (issue #1098). The kernel closes that lane
//! with a type — `NodePick` fetches the body from the evaluation
//! payload itself, tessellates and indexes in one call, so the pairing
//! is true by construction — and leaves raw `PickTarget` assembly for
//! consumers that already hold a mesh index.
//!
//! **Python has no such consumer, by a decision already taken.**
//! `MeshPick` and `MeshPickError` are DECIDED absent from the façade
//! (CUR3; `crates/pncad/src/select.rs`), and `PickTarget::pick` is a
//! `&MeshPick` — so through `pncad` a raw target has no constructor at
//! all. The Python door therefore takes `NodePick`s directly and makes
//! their targets itself: the type that cannot be mis-assembled is not
//! merely the one to prefer here, it is the only one that exists.
//!
//! # Dimensioned
//!
//! A ray's `origin` is a POSITION and crosses as `Length`; its
//! `direction` is a direction, which is dimensionless, and crosses as
//! bare floats — the `Frame` rule (`py/place.rs`), unchanged. The hit
//! parameter `t` is neither: it is in units of `|direction|`, so it is
//! a bare float and a caller who wants a distance normalizes the ray
//! or reads `PickHit.point`, which IS dimensioned.
//!
//! Nothing here pre-checks the ray. A non-finite origin or direction
//! is legal input and fail-safe kernel-side (it can only lose
//! constraints in the slab test), so a poisoned ray answers the typed
//! miss rather than a refusal — the kernel's rule, quoted, not
//! restated.

use std::sync::{Arc, OnceLock};

use pyo3::prelude::*;
use pyo3::types::{PyAny, PyString};

use crate::errors::ErrorClass;
use crate::py::doc::{NodeId, name_text};
use crate::py::mesh::Mesh;
use crate::py::quantity::Length;
use crate::py::select::entity_kind;
use crate::py::typed_err;
use crate::py::value::{Evaluation, lengths};
use crate::tags::{hit_test_error_tag, node_pick_error_tag};
use pncad::select as s;

/// A direction as the bare triple it is — dimensionless.
fn direction(v: pncad::geom_core::Vec3<f64>) -> (f64, f64, f64) {
    (v.x, v.y, v.z)
}

/// A dimensioned triple as the kernel's bare metres.
fn meters(v: (Length, Length, Length)) -> (f64, f64, f64) {
    (v.0.0.meters(), v.1.0.meters(), v.2.0.meters())
}

/// The four fields a [`s::HitTestError`] projects, in order: the node,
/// the failure it was poisoned through, and — on the BUG arm alone —
/// the unnamed entity's kind and body index.
///
/// The arm's payload is an `EntityRef`, an arena key beside a body
/// index. The KEY does not cross (G1, and the façade does not name its
/// type at all); the kind and the body index do, and together they are
/// the whole of what a bug report can act on: "node 7 evaluated and
/// left a face of output body 0 unnamed".
///
/// Exhaustive, no wildcard: an arm added kernel-side arrives here as a
/// compile error rather than as a silently unprojected payload.
fn hit_test_fields(py: Python<'_>, err: &s::HitTestError) -> [Py<PyAny>; 4] {
    let none = || py.None();
    // A field whose own construction failed degrades to `None` rather
    // than replacing the kernel's refusal with a boundary one — the
    // `py/readback.rs` rule: the caller asked why the pick refused,
    // and that answer must survive a failure to build one attribute.
    let obj = |v: PyResult<Py<PyAny>>| v.unwrap_or_else(|_| py.None());
    let node = |n: pncad::document::RecipeNodeId| obj(Py::new(py, NodeId(n)).map(|v| v.into_any()));
    let kind = |k: s::EntityKind| obj(Py::new(py, entity_kind(k)).map(|v| v.into_any()));
    // `u32`'s conversion is INFALLIBLE (its error type is
    // `Infallible`), so this one degrades nowhere: the match is total.
    let int = |n: u32| -> Py<PyAny> {
        match n.into_pyobject(py) {
            Ok(value) => value.into_any().unbind(),
        }
    };

    match err {
        s::HitTestError::NodeNotEvaluated { node: n } | s::HitTestError::NodeFailed { node: n } => {
            [node(*n), none(), none(), none()]
        }
        s::HitTestError::NodePoisoned { node: n, through } => {
            [node(*n), node(*through), none(), none()]
        }
        s::HitTestError::Unnamed { node: n, entity } => [
            node(*n),
            none(),
            kind(entity.key.kind()),
            int(entity.body),
        ],
    }
}

/// Raise `HitTestError` carrying the refusal's stable tag and payload.
///
/// The message is the kernel's own `Display`; the machine payload is
/// `variant` plus the four fields, each present on every arm and
/// `None` where that arm does not carry it.
fn hit_test_err(py: Python<'_>, err: &s::HitTestError) -> PyErr {
    let [node, through, kind, body] = hit_test_fields(py, err);
    typed_err(
        py,
        ErrorClass::HitTest,
        err.to_string(),
        &[
            (
                "variant",
                PyString::new(py, hit_test_error_tag(err))
                    .unbind()
                    .into_any(),
            ),
            ("node", node),
            ("through", through),
            ("kind", kind),
            ("body", body),
        ],
    )
}

/// The `HitTestError` EXCEPTION VALUE rather than a raise — what a
/// per-slot answer carries in the slot that has no name.
///
/// [`NodePick::patch_names`] is total per patch: one naming-emission
/// bug must not cost a consumer the names of every other patch it is
/// drawing, so the refusal rides IN the slot. A Python exception is a
/// value, which is what makes that shape spellable here at all.
fn hit_test_value(py: Python<'_>, err: &s::HitTestError) -> Py<PyAny> {
    hit_test_err(py, err).value(py).clone().into_any().unbind()
}

/// Raise `NodePickError` carrying the refusal's stable tag and payload.
///
/// Two arms FORWARD their payload's own tag and prose rather than
/// wrapping them (`crate::tags::node_pick_error_tag`): the standing
/// ladder is literally a `HitTestError`, and a tessellation refusal is
/// the tessellator's. What they do NOT forward is the inner refusal's
/// own extra ATTRIBUTES — a tessellation refusal's numbers stay on
/// `TessellateError`, which is where a caller who tessellates directly
/// reads them. That is the `AssemblyError` precedent (a gather refusal
/// arrives there under the gather's own tag, without the gather's
/// `node`), and this class's docstring says so.
fn node_pick_err(py: Python<'_>, err: &s::NodePickError) -> PyErr {
    let none = || py.None();
    let obj = |v: PyResult<Py<PyAny>>| v.unwrap_or_else(|_| py.None());
    let node = |n: pncad::document::RecipeNodeId| obj(Py::new(py, NodeId(n)).map(|v| v.into_any()));
    let int = |n: u32| -> Py<PyAny> {
        match n.into_pyobject(py) {
            Ok(value) => value.into_any().unbind(),
        }
    };

    // Exhaustive on purpose: an arm added kernel-side is a compile
    // error here, not a silently unprojected payload.
    let [which, through, kind, body] = match err {
        s::NodePickError::Standing(inner) => hit_test_fields(py, inner),
        s::NodePickError::NotABody { node: n } => [node(*n), none(), none(), none()],
        s::NodePickError::NoSuchBody { node: n, body } => {
            [node(*n), none(), none(), int(*body)]
        }
        s::NodePickError::Tessellate(_) | s::NodePickError::Index(_) => {
            [none(), none(), none(), none()]
        }
    };
    typed_err(
        py,
        ErrorClass::NodePick,
        err.to_string(),
        &[
            (
                "variant",
                PyString::new(py, node_pick_error_tag(err))
                    .unbind()
                    .into_any(),
            ),
            ("node", which),
            ("through", through),
            ("kind", kind),
            ("body", body),
        ],
    )
}

/// **A ray to pick along**: an origin and a direction, over
/// `t >= 0`.
///
/// `direction` need not be unit length, and no door here silently
/// normalizes it: the hit parameter `t` is in units of
/// `|direction|`, so every `t` produced from ONE ray is comparable
/// with every other, and rescaling the direction rescales them all by
/// the same factor.
///
/// A non-finite component is legal input and fail-safe: it can only
/// LOSE constraints in the kernel's conservative slab test, so a
/// poisoned ray prunes nothing and the pick answers the typed miss
/// rather than silently skipping geometry.
#[pyclass(frozen, module = "pncad", from_py_object)]
#[derive(Clone, Copy)]
pub(crate) struct Ray(pub(crate) s::Ray);

#[pymethods]
impl Ray {
    /// The ray from `origin` along `direction`.
    #[new]
    fn new(origin: (Length, Length, Length), direction: (f64, f64, f64)) -> Self {
        let (ox, oy, oz) = meters(origin);
        Self(s::Ray {
            origin: pncad::authoring::p3(ox, oy, oz),
            dir: pncad::authoring::v3(direction.0, direction.1, direction.2),
        })
    }

    /// The ray origin — the point at `t == 0`, dimensioned.
    #[getter]
    fn origin(&self) -> (Length, Length, Length) {
        lengths(self.0.origin)
    }

    /// The ray direction, dimensionless and not normalized.
    #[getter]
    fn direction(&self) -> (f64, f64, f64) {
        direction(self.0.dir)
    }

    fn __repr__(&self) -> String {
        let o = self.0.origin;
        let d = self.0.dir;
        format!(
            "Ray(origin=({}, {}, {}) m, direction=({}, {}, {}))",
            o.x, o.y, o.z, d.x, d.y, d.z
        )
    }
}

/// **A successful face pick**: the stable name, plus where and what
/// was hit.
///
/// No arena key: the NAME is the reference a selection holds (G1), and
/// it is the same opaque text `Evaluation.all_faces` and `select`
/// answer with — hand it to `Node.fillet` unread.
#[pyclass(frozen, module = "pncad")]
pub(crate) struct PickHit {
    name: String,
    node: NodeId,
    body: u32,
    t: f64,
    point: pncad::geom_core::Point3<f64>,
}

#[pymethods]
impl PickHit {
    /// The picked face's stable name — an opaque identifier.
    #[getter]
    fn name(&self) -> &str {
        &self.name
    }

    /// The node whose body was hit.
    #[getter]
    fn node(&self) -> NodeId {
        self.node
    }

    /// The output-body index within that node's value.
    #[getter]
    fn body(&self) -> u32 {
        self.body
    }

    /// The ray parameter of the hit, in units of the ray's own
    /// `|direction|` — a bare float, not a distance, unless the ray
    /// was given a unit direction.
    #[getter]
    fn t(&self) -> f64 {
        self.t
    }

    /// The hit point, `origin + t * direction` — dimensioned, and the
    /// door to read when a distance is what was wanted.
    #[getter]
    fn point(&self) -> (Length, Length, Length) {
        lengths(self.point)
    }

    fn __repr__(&self) -> String {
        format!(
            "PickHit(node={}, body={}, t={})",
            self.node.0.0, self.body, self.t
        )
    }
}

/// **A pick index whose `(node, body)` ↔ mesh pairing is TRUE BY
/// CONSTRUCTION** — and, through this surface, the only pick target
/// there is.
///
/// [`Self::build`] fetches the body from the evaluation's own payload,
/// through the same output-body indexing the name tables key by, then
/// tessellates and indexes it in one call. There is no other
/// constructor, so an index cannot assert a pairing it does not have —
/// which is what stops a pick answering a plausible, confidently wrong
/// name.
///
/// The tessellation rides along ([`Self::mesh`]) so a viewer displays
/// exactly what it picks against: one tessellation, one source of
/// truth. Cache one per displayed `(node, body)` and drop it when the
/// evaluation moves — a new `Evaluation` means new meshes, and an
/// index built against the old one is stale.
#[pyclass(frozen, module = "pncad")]
pub(crate) struct NodePick {
    pub(crate) inner: s::NodePick,
    /// The mesh handle, materialized once on first ask. The kernel's
    /// `NodePick` owns its `Mesh` by value and Python's `Mesh` is a
    /// handle over an `Arc`, so the crossing costs one copy — taken
    /// lazily and kept, rather than per access.
    mesh: OnceLock<Arc<pncad::mesh::Mesh>>,
}

#[pymethods]
impl NodePick {
    /// **Tessellate and index output body `body` of `node`** at the
    /// chordal budget `chordal`, against `evaluation`'s own payload.
    ///
    /// `chordal` is δ, a DISTANCE, and it is `Body.tessellate`'s
    /// budget verbatim: it says how coarsely a view of the model may
    /// approximate it, never what the model IS. Picking against a
    /// coarse index picks against the coarse triangles, which is
    /// exactly right — they are the ones on screen.
    ///
    /// Raises `NodePickError`, typed: `not_a_body` for a node whose
    /// value never draws (a datum, profile, declaration or mate),
    /// `no_such_body` for an index this node's value does not have,
    /// the standing ladder (`node_not_evaluated`, `node_failed`,
    /// `node_poisoned`) for a node this evaluation has no `Ok` value
    /// for, and the tessellator's own tags where tessellation refused.
    #[staticmethod]
    fn build(
        py: Python<'_>,
        evaluation: &Evaluation,
        node: &NodeId,
        body: u32,
        chordal: &Length,
    ) -> PyResult<Self> {
        let tol = pncad::tolerance::Tol::witness();
        s::NodePick::build(&evaluation.inner, node.0, body, chordal.0.meters(), tol)
            .map(Self::wrap)
            .map_err(|err| node_pick_err(py, &err))
    }

    /// **Every output body of `node`, each tessellated and indexed** —
    /// [`Self::build`]'s enumerating form, and the one to reach for
    /// when offering a whole node to a pick.
    ///
    /// A caller cannot ask a node how many bodies it has, and the
    /// indices are not a dense range: a split with one empty half
    /// occupies index 1 and not index 0. Probing `build` at 0, 1, 2, …
    /// until it refuses is precisely the by-hand pairing this type
    /// exists to remove, so the enumeration is taken kernel-side, from
    /// the same gather the name tables key by.
    ///
    /// **An empty list is a legal answer**, and it means something
    /// narrower than it looks: the node's value IS body-denoting but
    /// currently denotes none — an annihilated boolean, a split whose
    /// sides are both empty. A node whose value never draws raises
    /// `not_a_body` instead, because "this kind of node never draws"
    /// and "this node draws nothing today" are different states and
    /// only the second changes under an edit.
    ///
    /// Raises `NodePickError` as [`Self::build`] does; the first body
    /// that refuses stops the whole enumeration, because a partial
    /// answer would be a partial picture.
    #[staticmethod]
    fn build_all(
        py: Python<'_>,
        evaluation: &Evaluation,
        node: &NodeId,
        chordal: &Length,
    ) -> PyResult<Vec<Self>> {
        let tol = pncad::tolerance::Tol::witness();
        s::NodePick::build_all(&evaluation.inner, node.0, chordal.0.meters(), tol)
            .map(|built| built.into_iter().map(Self::wrap).collect())
            .map_err(|err| node_pick_err(py, &err))
    }

    /// The node this index answers for.
    #[getter]
    fn node(&self) -> NodeId {
        NodeId(self.inner.node())
    }

    /// The output-body index this index answers for.
    #[getter]
    fn body(&self) -> u32 {
        self.inner.body()
    }

    /// **The tessellation this index was built from** — the mesh to
    /// display, so that what is drawn is what is picked.
    #[getter]
    fn mesh(&self) -> Mesh {
        Mesh {
            inner: Arc::clone(
                self.mesh
                    .get_or_init(|| Arc::new(self.inner.mesh().clone())),
            ),
        }
    }

    /// **The stable name of every face patch of [`Self::mesh`]**, in
    /// patch order — one entry per patch, so entry `i` names the face
    /// `Mesh.patch(i)` draws.
    ///
    /// This is the inversion a DISPLAY consumer needs, and until this
    /// door Python had no way to ask it: a patch's identity in the mesh
    /// is an arena key, and G1 forbids the key crossing, so a viewer
    /// could address a patch by index and could not learn its name.
    /// Here the key never leaves — the index goes in, the name comes
    /// out.
    ///
    /// **Total, per patch, and each slot is `str` OR a
    /// `HitTestError`.** An evaluated-but-unnamed face is a
    /// naming-emission bug (spec D4), and it is surfaced in ITS OWN
    /// SLOT rather than as a refusal of the whole call, because one
    /// such bug must not cost a consumer the names of every other patch
    /// it is drawing. Branch with `isinstance(entry, str)`; the
    /// exception in a slot is a value, not something raised.
    fn patch_names(&self, py: Python<'_>, evaluation: &Evaluation) -> PyResult<Vec<Py<PyAny>>> {
        self.inner
            .patch_names(&evaluation.inner)
            .iter()
            .map(|slot| slot_name(py, slot))
            .collect()
    }

    /// **The stable name of every boundary polyline of
    /// [`Self::mesh`]**, in polyline order — [`Self::patch_names`]'
    /// edge twin, same contract and same per-slot loud arm.
    ///
    /// The polylines themselves are not bound (their content beside
    /// indices is arena keys), so what this is FOR is a consumer that
    /// hit-tests against drawn edges by POSITION — a display
    /// coordinate valid for one tessellation — and reads the name out
    /// of here.
    fn boundary_names(&self, py: Python<'_>, evaluation: &Evaluation) -> PyResult<Vec<Py<PyAny>>> {
        self.inner
            .boundary_names(&evaluation.inner)
            .iter()
            .map(|slot| slot_name(py, slot))
            .collect()
    }

    fn __repr__(&self) -> String {
        format!(
            "NodePick(node={}, body={}, {} patches)",
            self.inner.node().0,
            self.inner.body(),
            self.inner.mesh().patches.len()
        )
    }
}

impl NodePick {
    /// The kernel's index, with its mesh handle not yet materialized.
    fn wrap(inner: s::NodePick) -> Self {
        Self {
            inner,
            mesh: OnceLock::new(),
        }
    }
}

/// One slot of a per-slot name answer: the opaque text, or the
/// hit-test refusal as a VALUE.
fn slot_name(
    py: Python<'_>,
    slot: &Result<pncad::prelude::StableName, s::HitTestError>,
) -> PyResult<Py<PyAny>> {
    match slot {
        Ok(name) => Ok(PyString::new(py, &name_text(py, name)?).unbind().into_any()),
        Err(err) => Ok(hit_test_value(py, err)),
    }
}

/// **What is under this ray** — the nearest face hit across `targets`,
/// resolved to a stable name.
///
/// The free `pick_face` as a method on the evaluation it answers as
/// of, the posture every selector door on `Evaluation` already takes.
/// `targets` are `NodePick`s: through this surface a pick target has
/// no other spelling, and that is the point (module docs).
pub(crate) fn pick_face(
    py: Python<'_>,
    evaluation: &Evaluation,
    targets: Vec<PyRef<'_, NodePick>>,
    ray: &Ray,
) -> PyResult<Option<PickHit>> {
    let borrowed: Vec<s::PickTarget<'_>> = targets.iter().map(|t| t.inner.target()).collect();
    match s::pick_face(&evaluation.inner, &borrowed, &ray.0) {
        Ok(None) => Ok(None),
        Ok(Some(hit)) => Ok(Some(PickHit {
            name: name_text(py, &hit.name)?,
            node: NodeId(hit.node),
            body: hit.body,
            t: hit.t,
            point: hit.point,
        })),
        Err(err) => Err(hit_test_err(py, &err)),
    }
}

/// Register the picking vocabulary on the module.
pub(crate) fn register(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Ray>()?;
    m.add_class::<PickHit>()?;
    m.add_class::<NodePick>()?;
    Ok(())
}
