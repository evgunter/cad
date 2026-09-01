//! The detect/declare protocol's value vocabulary: `FlushFinding` and
//! its evidence enums, crossing Rust → Python.
//!
//! A finding is a REPORT — `Evaluation.find_flush_candidates` answers
//! with these values, the caller INSPECTS them, and `Node.declare` /
//! `Doc.declare` / `Doc.declare_all` turn inspected findings into the
//! shipped `Declare` vocabulary. The no-fusion boundary is kept
//! across the language boundary: no door here both detects and
//! declares. The same value also rides the boolean's
//! refusal MENU: an `EvaluationError` with `kind ==
//! "undeclared_contact"` carries one as its `finding` attribute
//! — the recourse is in the error.
//!
//! The pair's names cross as the SAME opaque texts every other door
//! speaks (`doc::name_text`) — the ordinal-28 contract: a name is an
//! identifier to store and hand back, never parsed. Nothing here
//! requires reading inside one: the finding itself is what
//! `Node.declare` consumes, typed.

use pyo3::prelude::*;

use crate::py::doc::name_text;
use pncad::select as s;
use pncad::topo::PlaneRelation as KPlaneRelation;

/// The verify door's relation verdict a finding's evidence carries.
#[pyclass(eq, eq_int, module = "pncad", from_py_object)]
#[derive(Clone, Copy, PartialEq)]
#[allow(
    missing_docs,
    reason = "each variant mirrors the documented `topo::PlaneRelation` variant of the same name"
)]
pub(crate) enum PlaneRelation {
    SameOriented,
    SameOpposite,
    Distinct,
}

/// The contact class a finding would verify as.
#[pyclass(eq, eq_int, module = "pncad", from_py_object)]
#[derive(Clone, Copy, PartialEq)]
#[allow(
    missing_docs,
    reason = "each variant mirrors the documented `editor_core::ContactClass` variant of the same name"
)]
pub(crate) enum ContactClass {
    Rest,
    Tangent,
}

/// Which rung of the verify ladder decided a finding.
#[pyclass(eq, eq_int, module = "pncad", from_py_object)]
#[derive(Clone, Copy, PartialEq)]
#[allow(
    missing_docs,
    reason = "each variant mirrors the documented `editor_core::FlushRung` variant of the same name"
)]
pub(crate) enum FlushRung {
    SharedSource,
    DecidedCoincident,
}

/// One flush-plane finding: "this face pair would verify as declared
/// contact" — a VALUE to inspect and pass to `Node.declare` /
/// `Doc.declare` / `Doc.declare_all`, never itself a declaration.
///
/// `a` and `b` are the pair's names as opaque text (`a` from the
/// query's first node, `b` from its second); `relation` is the verify
/// door's own verdict (`SameOpposite` = resting contact, opposed
/// outward normals; `SameOriented` = flush walls, the merge-stage
/// flavor); `class_` names the contact class (trailing underscore:
/// `class` is a Python keyword — the `or_` precedent); `rung` says
/// which ladder rung decided (`SharedSource` = syntactic recipe
/// identity, `DecidedCoincident` = the geometric trilean).
#[pyclass(frozen, module = "pncad", from_py_object)]
#[derive(Clone)]
pub(crate) struct FlushFinding(pub(crate) s::FlushFinding);

/// Crossing helper: the kernel relation as the Python mirror. The
/// match is exhaustive over the KERNEL enum with no wildcard arm, so
/// a kernel variant this module does not mirror stops the build.
pub(crate) fn plane_relation(rel: KPlaneRelation) -> PlaneRelation {
    match rel {
        KPlaneRelation::SameOriented => PlaneRelation::SameOriented,
        KPlaneRelation::SameOpposite => PlaneRelation::SameOpposite,
        KPlaneRelation::Distinct => PlaneRelation::Distinct,
    }
}

impl ContactClass {
    /// The kernel class this mirror names.
    ///
    /// Total over the MIRROR, which is the safe direction: every
    /// variant Python can spell has a kernel counterpart, so this
    /// cannot fail today. It takes `py` and returns a `PyResult`
    /// anyway, because the kernel enum is `#[non_exhaustive]` and a
    /// class that lands there without a mirror variant would make
    /// this a partial map — and a door that has to refuse later is
    /// better than a signature that has to change.
    pub(crate) fn to_kernel(self, _py: Python<'_>) -> PyResult<s::ContactClass> {
        match self {
            Self::Rest => Ok(s::ContactClass::Rest),
            Self::Tangent => Ok(s::ContactClass::Tangent),
        }
    }
}

/// Crossing helper: the kernel contact class as the Python mirror.
///
/// `ContactClass` is `#[non_exhaustive]` kernel-side, so unlike the
/// other mirrors the wildcard arm is FORCED on this crate and the
/// compile-time drift alarm is unavailable; a kernel class this
/// binding does not know refuses typed HERE rather than crossing
/// mislabeled. The wildcard is deliberate, not a missed arm — but it
/// is also why growth has to be conscious, so the pin in
/// `src/tests.rs` (`the_contact_class_mirror_matches_the_kernel`)
/// enumerates the vocabulary this mirror claims to speak.
///
/// A wildcarded pin cannot fire on its own when the kernel grows a
/// class, which is the standing cost of the forced wildcard and the
/// reason the pin lists the variants explicitly.
pub(crate) fn contact_class(py: Python<'_>, class: s::ContactClass) -> PyResult<ContactClass> {
    match class {
        s::ContactClass::Rest => Ok(ContactClass::Rest),
        s::ContactClass::Tangent => Ok(ContactClass::Tangent),
        // `Debug` because there is nothing else: the kernel enum has
        // no `Display`, and an unknown variant has no tag either. It
        // holds only while the unknown variant is FIELDLESS — a struct
        // variant renders with the struct fingerprint
        // `crate::errors::reads_as_prose` rejects, and this graceful
        // refusal becomes a panic at the funnel. Whoever adds one
        // decides then: give the kernel enum a `Display`, or render
        // the name alone here.
        other => Err(crate::py::typed_err(
            py,
            crate::errors::ErrorClass::Select,
            format!("a contact class this binding predates: {other:?}"),
            &[(
                "reason",
                pyo3::types::PyString::new(py, "unclassified")
                    .unbind()
                    .into_any(),
            )],
        )),
    }
}

/// Crossing helper: the kernel rung as the Python mirror. Exhaustive
/// over the KERNEL enum, no wildcard arm — kernel growth stops the
/// build here.
pub(crate) fn flush_rung(rung: s::FlushRung) -> FlushRung {
    match rung {
        s::FlushRung::SharedSource => FlushRung::SharedSource,
        s::FlushRung::DecidedCoincident => FlushRung::DecidedCoincident,
    }
}

#[pymethods]
impl FlushFinding {
    /// The pair's first name (the query's `a` node side), as the
    /// opaque name text.
    #[getter]
    fn a(&self, py: Python<'_>) -> PyResult<String> {
        name_text(py, &self.0.pair.0)
    }

    /// The pair's second name (the query's `b` node side).
    #[getter]
    fn b(&self, py: Python<'_>) -> PyResult<String> {
        name_text(py, &self.0.pair.1)
    }

    /// The verify door's relation verdict.
    #[getter]
    fn relation(&self) -> PlaneRelation {
        plane_relation(self.0.evidence.relation)
    }

    /// The contact class the pair would verify as.
    #[getter]
    fn class_(&self, py: Python<'_>) -> PyResult<ContactClass> {
        contact_class(py, self.0.class)
    }

    /// Which ladder rung decided.
    #[getter]
    fn rung(&self) -> FlushRung {
        flush_rung(self.0.evidence.rung)
    }

    fn __repr__(&self) -> String {
        format!("{:?}", self.0)
    }

    fn __eq__(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

/// Register the detect/declare value vocabulary on the module.
pub(crate) fn register(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PlaneRelation>()?;
    m.add_class::<ContactClass>()?;
    m.add_class::<FlushRung>()?;
    m.add_class::<FlushFinding>()?;
    Ok(())
}
