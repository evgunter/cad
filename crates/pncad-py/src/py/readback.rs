//! The read-back vocabulary: `Pose`, `Denotation`, and the typed
//! refusal the doors that answer in them raise.
//!
//! `crate::select`'s third invariant is **a name answers with values,
//! never keys**, and this module is its Python face. The doors
//! themselves hang off `Evaluation` (`face_frame`, `edge_frame`,
//! `vertex_position`, `denotation`) because a name is only meaningful
//! against the evaluation that minted it; what lives here is the
//! vocabulary they answer in.
//!
//! # Values, never verdicts
//!
//! The kernel's three read-back rules cross verbatim. A door answers
//! "this face's carrier frame is (o, axis, u_ref)" and never "is this
//! face planar" — geometric predicates are `select_where`'s job, on
//! the far side of a margin funnel. A frame read off stored geometry
//! is a definitional re-read and carries no pad. And where the stored
//! geometry fixes no convention the door REFUSES rather than
//! inventing one: a NURBS carrier has no canonical frame, and a
//! straight edge has no distinguished perpendicular, which is why
//! [`Pose::u_ref`] is optional rather than defaulted.
//!
//! # Dimensioned
//!
//! A pose's origin is a POSITION and crosses as `Length`; its `axis`
//! and `u_ref` are directions, which are dimensionless, and cross as
//! bare floats — the `Frame` rule (`py/place.rs`), unchanged.

use pyo3::prelude::*;
use pyo3::types::{PyAny, PyString};

use crate::errors::ErrorClass;
use crate::py::doc::NodeId;
use crate::py::quantity::Length;
use crate::py::select::entity_kind;
use crate::py::typed_err;
use crate::py::value::lengths;
use crate::tags::interrogate_error_tag;
use pncad::select as s;

/// A direction as the bare triple it is — dimensionless.
fn direction(v: pncad::geom_core::Vec3<f64>) -> (f64, f64, f64) {
    (v.x, v.y, v.z)
}

/// **A frame read off stored geometry**: an origin plus the carrier's
/// own reference directions, verbatim.
///
/// `origin` is the CARRIER's distinguished point — a plane's origin,
/// a cylinder's axis point, a circle's centre — not the trimmed
/// face's or edge's, so a plane face's origin need not lie inside the
/// face. `axis` is the carrier's principal direction (a plane's
/// normal, a line's direction), and it is the CHART's direction, NOT
/// corrected by the face's orientation sense: the sense is a separate
/// fact, and folding it in silently would make two questions share
/// one answer.
///
/// `u_ref` is the in-frame reference direction where the carrier's
/// convention fixes one, and `None` where it fixes none — a line has
/// no distinguished perpendicular, and inventing one would be a
/// fabricated convention quoted back as though the model had chosen
/// it. `v_ref` is `axis x u_ref`, and `None` exactly when `u_ref` is.
///
/// **No `==`.** The kernel's `Point3`/`Vec3` deliberately implement
/// no `PartialEq` — comparing coordinates is a tolerance question,
/// and an exact-bit answer to it would be a decided predicate wearing
/// an operator. Read the components and compare them the way your
/// problem requires.
#[pyclass(frozen, module = "pncad", from_py_object)]
#[derive(Clone, Copy)]
pub(crate) struct Pose(pub(crate) s::Pose<f64>);

#[pymethods]
impl Pose {
    /// The carrier's distinguished point.
    #[getter]
    fn origin(&self) -> (Length, Length, Length) {
        lengths(self.0.origin)
    }

    /// The carrier's principal direction, uncorrected by any face
    /// sense.
    #[getter]
    fn axis(&self) -> (f64, f64, f64) {
        direction(self.0.axis)
    }

    /// The in-frame reference direction, or `None` where the carrier
    /// fixes none.
    #[getter]
    fn u_ref(&self) -> Option<(f64, f64, f64)> {
        self.0.u_ref.map(direction)
    }

    /// `axis x u_ref` — the triad's third leg, `None` exactly when
    /// [`Self::u_ref`] is.
    #[getter]
    fn v_ref(&self) -> Option<(f64, f64, f64)> {
        self.0.v_ref().map(direction)
    }

    fn __repr__(&self) -> String {
        let o = self.0.origin;
        let a = self.0.axis;
        format!(
            "Pose(origin=({}, {}, {}) m, axis=({}, {}, {}), u_ref={})",
            o.x,
            o.y,
            o.z,
            a.x,
            a.y,
            a.z,
            match self.0.u_ref {
                Some(u) => format!("({}, {}, {})", u.x, u.y, u.z),
                None => "None".to_string(),
            }
        )
    }
}

/// **What a name denotes** — the referencing question, answered
/// without the entities it resolves to.
///
/// A TIE is a naming success and a referencing failure: the name is
/// well formed and several entities answer to it equally well, so a
/// door that must pick one refuses (`ReadbackError` with `variant ==
/// "ambiguous"`) rather than breaking the tie silently. This value is
/// how a caller finds that out BEFORE asking, and it deliberately
/// carries a COUNT rather than the candidates — the candidates are
/// arena keys, which never cross.
///
/// `tied` is the fact to branch on; `candidates` is how many entities
/// answer, which is `1` exactly when `tied` is `False`.
#[pyclass(frozen, module = "pncad", from_py_object)]
#[derive(Clone, Copy)]
pub(crate) struct Denotation(pub(crate) s::Denotation);

#[pymethods]
impl Denotation {
    /// Whether the name is a recorded tie.
    #[getter]
    fn tied(&self) -> bool {
        matches!(self.0, s::Denotation::Tied { .. })
    }

    /// How many entities answer to the name.
    #[getter]
    fn candidates(&self) -> usize {
        match self.0 {
            s::Denotation::Unique => 1,
            s::Denotation::Tied { candidates } => candidates,
        }
    }

    fn __repr__(&self) -> String {
        match self.0 {
            s::Denotation::Unique => "Denotation(unique)".to_string(),
            s::Denotation::Tied { candidates } => {
                format!("Denotation(tied, candidates={candidates})")
            }
        }
    }

    fn __eq__(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

/// Raise `ReadbackError` carrying the refusal's stable tag and the
/// arm's payload.
///
/// The message is the kernel's own `Display` — the read-back doors
/// and the name doors both have one, and the wrapping arm forwards
/// the kernel's words rather than paraphrasing a layer it does not
/// own — and the machine payload is `variant` plus the fields, each
/// present on every arm and `None` where that arm does not carry it.
pub(crate) fn readback_err(py: Python<'_>, err: &s::InterrogateError) -> PyErr {
    use s::{InterrogateError as E, ReadbackError as R};

    let none = || py.None();
    // A field whose own construction failed degrades to `None` rather
    // than replacing the kernel's refusal with a boundary one: the
    // caller asked why the read-back refused, and that answer must
    // survive a failure to build one of its attributes.
    let obj = |v: PyResult<Py<PyAny>>| v.unwrap_or_else(|_| py.None());
    // `usize`'s conversion is INFALLIBLE (its error type is
    // `Infallible`), so this one degrades nowhere: the match is total.
    let int = |n: usize| -> Py<PyAny> {
        match n.into_pyobject(py) {
            Ok(value) => value.into_any().unbind(),
        }
    };
    let node = |n: pncad::document::RecipeNodeId| obj(Py::new(py, NodeId(n)).map(|v| v.into_any()));
    let kind = |k: s::EntityKind| obj(Py::new(py, entity_kind(k)).map(|v| v.into_any()));
    let text = |s: &str| PyString::new(py, s).unbind().into_any();

    // Every field on every arm, `None` where the arm does not carry
    // it: the `AssemblyError` shape, so `getattr` never raises and a
    // caller reads the payload without first branching on `variant`.
    // The tuple is positional and the match is exhaustive, so an arm
    // added kernel-side arrives here as a compile error rather than
    // as a silently unprojected payload.
    let (which, through, candidates, wanted, found, index, payload, carrier) = match err {
        E::NodeNotEvaluated { node: n } | E::NodeFailed { node: n } => (
            node(*n),
            none(),
            none(),
            none(),
            none(),
            none(),
            none(),
            none(),
        ),
        E::NodePoisoned { node: n, through } => (
            node(*n),
            node(*through),
            none(),
            none(),
            none(),
            none(),
            none(),
            none(),
        ),
        E::Ambiguous { candidates } => (
            none(),
            none(),
            int(*candidates),
            none(),
            none(),
            none(),
            none(),
            none(),
        ),
        E::WrongKind { wanted, found } => (
            none(),
            none(),
            none(),
            kind(*wanted),
            kind(*found),
            none(),
            none(),
            none(),
        ),
        E::NoBodies { payload } => (
            none(),
            none(),
            none(),
            none(),
            none(),
            none(),
            text(payload),
            none(),
        ),
        E::NoSuchBody { index } => (
            none(),
            none(),
            none(),
            none(),
            none(),
            int(*index as usize),
            none(),
            none(),
        ),
        E::Readback(R::NoCanonicalFrame { carrier }) => (
            none(),
            none(),
            none(),
            none(),
            none(),
            none(),
            none(),
            text(carrier),
        ),
        E::NoSuchName | E::WholeBody | E::Readback(_) => (
            none(),
            none(),
            none(),
            none(),
            none(),
            none(),
            none(),
            none(),
        ),
    };
    let fields = [
        ("variant", text(interrogate_error_tag(err))),
        ("node", which),
        ("through", through),
        ("candidates", candidates),
        ("wanted", wanted),
        ("found", found),
        ("index", index),
        ("payload", payload),
        ("carrier", carrier),
    ];
    typed_err(py, ErrorClass::Readback, err.to_string(), &fields)
}

/// Register the read-back vocabulary on the module.
pub(crate) fn register(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Pose>()?;
    m.add_class::<Denotation>()?;
    Ok(())
}
