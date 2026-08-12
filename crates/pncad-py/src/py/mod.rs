//! The PyO3 surface. Compiled only under the `python` feature.

mod doc;
mod path;
mod quantity;
mod select;
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
    "A body failed a topological or geometric validator. From the \
     validate doors it carries `door`, the gate that refused, and \
     `failure_count`, how many refusals it collected; from \
     `mass_properties` it carries `reason` instead."
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
    "A value the expression layer refused: non-finite, or a count \
     written as a continuous literal. Carries `kind`, the stable tag \
     of the refusing arm."
);
pyo3::create_exception!(
    pncad,
    PersistError,
    PncadError,
    "A save or load the persistence doors refused (bad header, \
     unknown schema, unparseable body, a snapshot or edit log that \
     fails the shared validator, ...). Carries `variant`, the stable \
     tag of the refusing arm."
);
pyo3::create_exception!(
    pncad,
    ExportError,
    PncadError,
    "The document-layer export door refused. Carries `variant` and \
     `node`; a poisoning adds `through`, a wrong-kind value adds \
     `kind`."
);
pyo3::create_exception!(
    pncad,
    StepImportError,
    PncadError,
    "A STEP text the importer refused, or one that parsed to a \
     non-solid. Carries `variant` (`refused` or `wireframe`); \
     per-variant field projection is deferred with the rest of the \
     read-back surface."
);
pyo3::create_exception!(
    pncad,
    PathError,
    PncadError,
    "The PATHS authoring algebra refused the geometry AT THE CALL \
     SITE (junction check, `NoCornerForFillet`, the tangent-line \
     close, a nonpositive radius, ...) — the same refusal the Rust \
     surface returns, raised where the verb was written. Carries \
     `variant`, the stable tag of the refusing arm."
);
pyo3::create_exception!(
    pncad,
    SelectRefusal,
    PncadError,
    "A selection query `select_where` could not answer — the same \
     typed refusal the Rust door returns (an in-band decided margin, \
     a tied name whose candidates disagree, a non-datum reference, \
     ...). Carries `reason`, the stable tag of the refusing arm, plus \
     the arm's payload as attributes (`None` where inapplicable)."
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
        ErrorClass::Persist => PersistError::new_err(message),
        ErrorClass::Export => ExportError::new_err(message),
        ErrorClass::StepImport => StepImportError::new_err(message),
        ErrorClass::Path => PathError::new_err(message),
        ErrorClass::Select => SelectRefusal::new_err(message),
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

/// Python bindings for the pncad B-rep CAD kernel.
///
/// Author exact solids as a document, evaluate it, measure and export
/// the result. Start with `docs/GUIDE.md` §2.8 or the worked script
/// `crates/pncad-py/examples/bracket.py`; `crates/pncad-py/README.md`
/// covers installation. Refusals are typed exceptions carrying
/// attributes — all of them subclass `PncadError`.
///
/// The name `pncad` is a placeholder until the project is named.
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
    m.add("PersistError", py.get_type::<PersistError>())?;
    m.add("ExportError", py.get_type::<ExportError>())?;
    m.add("StepImportError", py.get_type::<StepImportError>())?;
    m.add("PathError", py.get_type::<PathError>())?;
    m.add("SelectRefusal", py.get_type::<SelectRefusal>())?;

    quantity::register(m)?;
    path::register(m)?;
    doc::register(m)?;
    select::register(m)?;
    value::register(m)?;

    // Schema/provenance surface: the version the persistence doors
    // speak (LIB-DOORS F1 — `Doc.save`/`load` are bound now).
    let meta = PyDict::new(py);
    meta.set_item("f64_only", true)?;
    meta.set_item("abi3", "py38")?;
    meta.set_item("schema_version", pncad::document::SCHEMA_VERSION)?;
    m.add("__build_info__", meta)?;

    Ok(())
}
