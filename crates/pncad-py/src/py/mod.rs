//! The PyO3 surface. Compiled only under the `python` feature.

mod doc;
mod quantity;
mod value;

use pyo3::prelude::*;
use pyo3::types::PyDict;

use crate::errors::ErrorClass;

pyo3::create_exception!(
    pncad,
    PncadError,
    pyo3::exceptions::PyException,
    "Base class for every refusal this module raises.\n\n\
     Every subclass carries its refusal as ATTRIBUTES (LIBRARY-DESIGN \
     §L4: typed exceptions carrying the structured error, never \
     strings). The message is for humans; the attributes are the \
     contract."
);
pyo3::create_exception!(
    pncad,
    EditError,
    PncadError,
    "The document layer refused an edit (unknown node, cycle, slot \
     dimension mismatch, ...)."
);
pyo3::create_exception!(
    pncad,
    EvaluationError,
    PncadError,
    "A node failed to evaluate, or was poisoned by an upstream \
     failure. Carries `node` and, for a poisoning, `through`."
);
pyo3::create_exception!(
    pncad,
    ValidationError,
    PncadError,
    "A body failed a topological or geometric validator. Carries \
     `failures`, the list of validator refusals."
);
pyo3::create_exception!(
    pncad,
    DimensionError,
    PncadError,
    "A dimension mismatch at the quantity boundary. Carries `op`, \
     `left`, `right` — the operator and the two dimension tags."
);
pyo3::create_exception!(
    pncad,
    LiteralError,
    PncadError,
    "A value refused before it reached the kernel: non-finite, or a \
     count written as a continuous literal. Carries `kind` and, when \
     applicable, `value`."
);

/// Raise the exception class [`ErrorClass`] names, with `fields`
/// attached as instance attributes.
///
/// This is the single construction site for every typed refusal, so
/// "the payload is attributes, not prose" is enforced in one place
/// rather than repeated at each raise.
pub(crate) fn typed_err(
    py: Python<'_>,
    class: ErrorClass,
    message: impl Into<String>,
    fields: &[(&str, Py<PyAny>)],
) -> PyErr {
    let message: String = message.into();
    let err = match class {
        ErrorClass::Edit => EditError::new_err(message),
        ErrorClass::Evaluation => EvaluationError::new_err(message),
        ErrorClass::Validation => ValidationError::new_err(message),
        ErrorClass::Dimension => DimensionError::new_err(message),
        ErrorClass::Literal => LiteralError::new_err(message),
    };
    // Attaching attributes needs the instance, which materialises the
    // exception value; a failure here would itself be a Python error,
    // so it replaces the original rather than being swallowed.
    let value = err.value(py);
    for (name, field) in fields {
        if let Err(set_failed) = value.setattr(*name, field.bind(py)) {
            return set_failed;
        }
    }
    PyErr::from_value(value.clone().into_any())
}

/// Python bindings for the `pncad` CAD kernel (LIB-U9S scaffold).
#[pymodule]
#[pyo3(name = "pncad")]
fn pncad_module(m: &Bound<'_, PyModule>) -> PyResult<()> {
    let py = m.py();

    m.add("PncadError", py.get_type::<PncadError>())?;
    m.add("EditError", py.get_type::<EditError>())?;
    m.add("EvaluationError", py.get_type::<EvaluationError>())?;
    m.add("ValidationError", py.get_type::<ValidationError>())?;
    m.add("DimensionError", py.get_type::<DimensionError>())?;
    m.add("LiteralError", py.get_type::<LiteralError>())?;

    quantity::register(m)?;
    doc::register(m)?;
    value::register(m)?;

    // Schema/provenance surface: the version the persistence format
    // would speak. Exposed as data even though save/load are not yet
    // reachable through the façade (see the persistence FINDING).
    let meta = PyDict::new(py);
    meta.set_item("f64_only", true)?;
    meta.set_item("abi3", "py38")?;
    m.add("__build_info__", meta)?;

    Ok(())
}
