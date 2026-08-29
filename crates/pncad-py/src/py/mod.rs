//! The PyO3 surface. Compiled only under the `python` feature.

mod assembly;
mod checks;
mod doc;
mod flush;
mod mate;
mod mesh;
mod path;
mod place;
mod quantity;
mod refactor;
mod select;
mod store;
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
     typed exceptions carrying the structured error, never \
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
    "An operator applied to two QUANTITIES whose dimensions do not \
     admit it — `1 * m + 1 * rad`. Carries `op`, `left`, `right`: \
     the operator and the two dimension tags.\n\n\
     This is the quantity boundary only, and it is not the only \
     dimension check in the library. The document layer has its own \
     refusal type, which reaches Python two ways: through literal \
     construction, raising `LiteralError`; and through `load`, where \
     a save file's ill-dimensioned expression — a genuine mismatch — \
     arrives as `PersistError` with `variant == \"parse\"` rather \
     than as any dimension class (issue #694). So `DimensionError` \
     does not intercept an expression-layer mismatch, and nothing \
     else does either yet."
);
pyo3::create_exception!(
    pncad,
    LiteralError,
    PncadError,
    "A value the expression layer refused: non-finite, or a count \
     written as a continuous literal. Carries `kind`, the stable tag \
     of the refusing arm.\n\n\
     Not `DimensionError`: that one is the quantity boundary's \
     operator check. The expression layer's refusal type has \
     dimension-mismatch arms too, and `load` DOES reach them from a \
     hand-edited save file — but they arrive as `PersistError` with \
     `variant == \"parse\"`, not here (issue #694). Every `kind` \
     raised on this class is a literal-value refusal."
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
    TessellateError,
    PncadError,
    "The tessellator refused a body. Carries `variant`, the stable tag \
     of the refusing arm, plus the arm's numbers as attributes \
     (`value`, `bound`, `requested`, `note`; `None` where \
     inapplicable).\n\n\
     The offending face or edge is an arena KEY and does not cross — \
     the curation exists to keep those unnameable — so a refusal names \
     WHICH arm fired and, where the arm carries one, the number that \
     makes it actionable. The message is the tessellator's own prose; \
     the tag is the branchable part."
);
pyo3::create_exception!(
    pncad,
    StlError,
    PncadError,
    "An STL export refused. Carries `variant`, the stable tag of the \
     refusing arm.\n\n\
     Three Rust refusals share this class because they refuse the same \
     CALL: the writers' own `StlError` (`degenerate_triangle`, \
     `index_out_of_range`, `too_many_triangles`, `io`), and the two \
     validated option newtypes, which are keyword arguments here — \
     `solid_name_unrepresentable`, `binary_header_too_long`, \
     `binary_header_sniffs_ascii`. The tags share one namespace, so \
     which of the three refused is readable off `variant`."
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
pyo3::create_exception!(
    pncad,
    IdentityError,
    PncadError,
    "A document identity could not be minted: the OS entropy source \
     refused. Identity is never defaulted — two documents sharing an \
     id are the same part, and a workspace refuses to hold both — so \
     the refusal is surfaced. Carries `variant`."
);
pyo3::create_exception!(
    pncad,
    WorkspaceError,
    PncadError,
    "The workspace store refused. Carries `variant`, the stable tag \
     of the refusing arm, and the arm's payload as attributes — \
     `path`, `id`, `first`, `second`, `wanted`, `found` — each \
     present on every arm and `None` where that arm does not carry \
     it.\n\n\
     The arm the store exists to make loud is `pin_mismatch`: a \
     reference names a VERSION, so a document that changed under one \
     refuses with `wanted` and `found` rather than resolving to the \
     new content. `pncad.PIN_MISMATCH_RECOURSE` is the recourse \
     sentence its message ends on."
);
pyo3::create_exception!(
    pncad,
    MateError,
    PncadError,
    "The mate solve could not place an instance. Carries `variant`, \
     the stable tag of the refusing arm, and `fault` — the \
     `MateFault` VALUE, which carries the arm's payload.\n\n\
     The solve itself is TOTAL and never raises: a refusing cluster \
     must not fail an unrelated one, so `solve_document` records the \
     fault per node and `SolvedPoses.fault` hands back the same value \
     this exception carries. This class is raised only where an \
     answer is a pose or nothing — `SolvedPoses.placement`. One \
     payload vocabulary, so the value and the exception cannot \
     disagree."
);
pyo3::create_exception!(
    pncad,
    AssemblyError,
    PncadError,
    "The at-rest assembly gate refused. Carries `variant`, the stable \
     tag of the refusing arm, plus the arm's payload as attributes \
     (`mate`, `side`, `name`, `why`, `class_`, `findings`), each \
     present on every arm and `None` where that arm does not carry \
     it.\n\n\
     **The two verdict arms are NOT interchangeable.** `at_rest` is a \
     finding AGAINST the document — a refuted declaration or an \
     undeclared contact. `uncertified` is the declared direction's \
     FRONTIER: nothing was refuted and nothing was undeclared, the \
     census simply has no certifier lane for the faces a declaration \
     names, so nothing was decided about the geometry either way. A \
     caller who catches this class must say which of the two they \
     mean.\n\n\
     A gather refusal arrives here under the gather's OWN tag \
     (`no_body_roots`, `root_failed`, ...), not a wrapper tag: which \
     invariant broke is what a caller branches on."
);
pyo3::create_exception!(
    pncad,
    ProductError,
    PncadError,
    "The whole-document gather refused. Carries `variant`, the stable \
     tag of the refusing arm, plus `node`, `through` and `name` \
     (`None` where the arm does not carry them).\n\n\
     A product is all of the roots or none of them — there are no \
     partial products."
);
pyo3::create_exception!(
    pncad,
    SplitError,
    PncadError,
    "The `split` refactoring refused. Carries `variant`, the stable \
     tag of the refusing arm, plus its payload as attributes \
     (`node`, `consumer`, `input`, `gauge`, `instance`, `param`, \
     `name`, `id`), `None` where inapplicable."
);
pyo3::create_exception!(
    pncad,
    InlineError,
    PncadError,
    "The `inline` refactoring refused. Carries `variant`, the stable \
     tag of the refusing arm, plus its payload as attributes \
     (`node`, `by`, `name`, `param`, `key`, `root`, `host_epsilon`, \
     `part_epsilon`), `None` where inapplicable.\n\n\
     Inline crosses the SAME document seam evaluation does, so a \
     reference that will not resolve refuses under the seam's own \
     tags — `part_pin_mismatch`, `part_epsilon_seam`, \
     `part_unresolved` — the ones `EvaluationError.kind` already \
     speaks. A stale pin is refused here, never silently retargeted."
);
pyo3::create_exception!(
    pncad,
    UpdateError,
    PncadError,
    "A whole-document pin update produced no edit list. Carries \
     `variant` (`no_such_reference` or `already_pinned`), `id` — the \
     document id, which both arms name because \"which part did you \
     mean\" is the only question an author can act on here — and \
     `pin`, the pin every site already names, on `already_pinned` \
     alone."
);
pyo3::create_exception!(
    pncad,
    ChecksError,
    PncadError,
    "The advisory-check registry could not RUN. Carries `variant`, the \
     stable tag of the refusing arm (`root_without_value`, `band`, \
     `product_unavailable`), and `node` — the root without a value, \
     `None` on the other arms.\n\n\
     NOT a finding. A check that ran and disagreed is a value in the \
     report; this class means nothing was checked."
);
pyo3::create_exception!(
    pncad,
    CheckRefusal,
    PncadError,
    "`enforce_checks` refused: the report carries findings whose check \
     the CALLER configured at `Severity.Error`. Carries `findings`, \
     every refusing `CheckFinding` in report order.\n\n\
     The registry's one refusing path, and it refuses on nothing the \
     caller did not ask to be refused on — no default severity is \
     `Error`, and the separation resident cannot be set to it at all."
);
pyo3::create_exception!(
    pncad,
    FrameError,
    PncadError,
    "A frame constructor refused its inputs — the same typed refusal \
     the Rust door returns: a direction that was not DEFINITELY \
     usable (coincident eye and target, a roll reference along the \
     aim, a zero mirror normal), or a tolerance yielding no usable \
     band. Carries `variant`, the stable tag of the refusing arm."
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
        ErrorClass::Tessellate => TessellateError::new_err(message),
        ErrorClass::StlExport => StlError::new_err(message),
        ErrorClass::StepImport => StepImportError::new_err(message),
        ErrorClass::Path => PathError::new_err(message),
        ErrorClass::Select => SelectRefusal::new_err(message),
        ErrorClass::Frame => FrameError::new_err(message),
        ErrorClass::Identity => IdentityError::new_err(message),
        ErrorClass::Workspace => WorkspaceError::new_err(message),
        ErrorClass::Mate => MateError::new_err(message),
        ErrorClass::Assembly => AssemblyError::new_err(message),
        ErrorClass::Product => ProductError::new_err(message),
        ErrorClass::Split => SplitError::new_err(message),
        ErrorClass::Inline => InlineError::new_err(message),
        ErrorClass::Update => UpdateError::new_err(message),
        ErrorClass::Checks => ChecksError::new_err(message),
        ErrorClass::Enforce => CheckRefusal::new_err(message),
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
    m.add("TessellateError", py.get_type::<TessellateError>())?;
    m.add("StlError", py.get_type::<StlError>())?;
    m.add("StepImportError", py.get_type::<StepImportError>())?;
    m.add("PathError", py.get_type::<PathError>())?;
    m.add("SelectRefusal", py.get_type::<SelectRefusal>())?;
    m.add("FrameError", py.get_type::<FrameError>())?;
    m.add("IdentityError", py.get_type::<IdentityError>())?;
    m.add("WorkspaceError", py.get_type::<WorkspaceError>())?;
    m.add("MateError", py.get_type::<MateError>())?;
    m.add("AssemblyError", py.get_type::<AssemblyError>())?;
    m.add("ProductError", py.get_type::<ProductError>())?;
    m.add("SplitError", py.get_type::<SplitError>())?;
    m.add("InlineError", py.get_type::<InlineError>())?;
    m.add("UpdateError", py.get_type::<UpdateError>())?;
    m.add("ChecksError", py.get_type::<ChecksError>())?;
    m.add("CheckRefusal", py.get_type::<CheckRefusal>())?;

    quantity::register(m)?;
    path::register(m)?;
    place::register(m)?;
    doc::register(m)?;
    select::register(m)?;
    store::register(m)?;
    mate::register(m)?;
    assembly::register(m)?;
    refactor::register(m)?;
    flush::register(m)?;
    checks::register(m)?;
    mesh::register(m)?;
    value::register(m)?;

    // Schema/provenance surface: the version the persistence doors
    // speak, behind `Doc.save`/`load`.
    let meta = PyDict::new(py);
    meta.set_item("f64_only", true)?;
    meta.set_item("abi3", "py38")?;
    meta.set_item("schema_version", pncad::document::SCHEMA_VERSION)?;
    m.add("__build_info__", meta)?;

    Ok(())
}
