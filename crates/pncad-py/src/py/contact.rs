//! The contact vocabulary crossing Rust → Python: `ContactClass`, the
//! class a mate asserts and an interface crossing reports.

use pyo3::prelude::*;

use pncad::select as s;

/// The contact class a mate asserts between two faces.
#[pyclass(eq, eq_int, frozen, hash, module = "pncad", from_py_object)]
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[allow(
    missing_docs,
    reason = "each variant mirrors the documented `editor_core::ContactClass` variant of the same name"
)]
pub(crate) enum ContactClass {
    Rest,
    Tangent,
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
        //
        // The word is the query door's own, from one map
        // (`crate::tags::unmirrored_select_tag`): a caller reads the
        // same `reason` off either crossing because it learns the same
        // fact from both. The attributes are the whole class's, from
        // `crate::py::select::refusal_fields`, so this path answers
        // `None` where the others answer a payload rather than raising
        // `AttributeError` on the one door that hand-built its list.
        other => Err(crate::py::typed_err(
            py,
            crate::errors::ErrorClass::Select,
            format!("a contact class this binding predates: {other:?}"),
            &crate::py::select::refusal_fields(
                py,
                crate::tags::unmirrored_select_tag(crate::errors::UnmirroredSelect::ContactClass),
            ),
        )),
    }
}

/// Register the contact vocabulary on the module.
pub(crate) fn register(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<ContactClass>()?;
    Ok(())
}
