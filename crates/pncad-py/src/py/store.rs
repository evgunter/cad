//! The workspace store: a directory of documents, the pins that say
//! WHICH VERSION of one, and the references that pair the two.
//!
//! Three vocabularies, and they are deliberately separate. A
//! [`super::doc::Doc`]'s id answers "which part" and survives every
//! edit. A [`ContentPin`] answers "which version of it" — the SHA-256
//! of the document's canonical semantic bytes, so it moves whenever
//! the content does. A [`DocRef`] pairs them, and that pair is what a
//! cross-document reference carries: Cargo.lock semantics, where
//! editing the referenced document never silently retargets the
//! reference — the store refuses the stale pin instead.
//!
//! Identity crosses as the canonical 32 hex digits (`Doc.id`'s own
//! spelling) rather than as an opaque handle class, so the id a
//! caller reads off a document, a save header, or a store listing is
//! one value in one alphabet. A pin does NOT: it is a class, because
//! a pin is compared far more often than it is read, and a bare
//! 64-character string invites the substring comparisons the type
//! forbids.

use std::path::PathBuf;
use std::sync::Arc;

use pyo3::prelude::*;
use pyo3::types::{PyBytes, PyDict, PyString};

use crate::errors::ErrorClass;
use crate::py::doc::persist_err;
use crate::py::typed_err;
use crate::tags::workspace_error_tag;
use pncad::document as d;
use pncad::tolerance::Tol;
use pncad::workspace as ws;

/// Parse the canonical 32-hex-digit identity spelling.
///
/// A malformed id is a boundary `ValueError`, the same class of
/// refusal as a string where a `SketchPlane` belongs: there is no
/// kernel refusal to forward, because `DocumentId::parse_hex` answers
/// an `Option`. A WELL-FORMED id the store does not hold refuses at
/// the store's own door (`WorkspaceError`, `variant == "unknown_id"`),
/// which is where that belongs.
fn document_id(text: &str) -> PyResult<d::DocumentId> {
    d::DocumentId::parse_hex(text).ok_or_else(|| {
        pyo3::exceptions::PyValueError::new_err(format!(
            "not a document id: {text:?} — an id is exactly 32 lowercase \
             hex digits, the spelling `Doc.id` answers"
        ))
    })
}

/// Raise `WorkspaceError` carrying the refusal's stable tag and the
/// arm's own payload.
///
/// Every attribute is set on every arm, `None` where the arm does not
/// carry it (the `SelectRefusal` posture): a caller reads
/// `err.wanted` without first branching on `err.variant`, and an
/// attribute that is absent rather than `None` would be an
/// `AttributeError` in the middle of error handling.
///
/// The tag comes from [`workspace_error_tag`], which is total over
/// the enum — the map is the store's fact to state, not this raise
/// site's to assume.
fn workspace_err(py: Python<'_>, err: &ws::WorkspaceError) -> PyErr {
    use ws::WorkspaceError as E;
    let path = |p: &PathBuf| -> Py<PyAny> {
        PyString::new(py, &p.display().to_string())
            .unbind()
            .into_any()
    };
    let id = |i: &d::DocumentId| -> Py<PyAny> { PyString::new(py, &i.hex()).unbind().into_any() };
    let pin = |p: &d::ContentPin| -> Py<PyAny> {
        // Materialising the pyclass can only fail if the interpreter
        // is out of memory; a failure there would surface as a
        // different Python error rather than being swallowed.
        Py::new(py, ContentPin(*p))
            .map(|v| v.into_any())
            .unwrap_or_else(|_| py.None())
    };
    let none = || py.None();
    // Field extraction and message rendering are separate: the
    // message is `WorkspaceError`'s own `Display` (real prose,
    // including `PIN_MISMATCH_RECOURSE`), and the payload is these
    // attributes.
    let (p, i, first, second, wanted, found) = match err {
        E::Io { path: at, .. } => (path(at), none(), none(), none(), none(), none()),
        E::DuplicateId {
            id: which,
            first: a,
            second: b,
        } => (none(), id(which), path(a), path(b), none(), none()),
        E::Header { path: at, .. } | E::Load { path: at, .. } | E::Pin { path: at, .. } => {
            (path(at), none(), none(), none(), none(), none())
        }
        E::UnknownId { id: which } | E::Save { id: which, .. } => {
            (none(), id(which), none(), none(), none(), none())
        }
        E::PinMismatch {
            id: which,
            path: at,
            wanted: w,
            found: f,
        } => (path(at), id(which), none(), none(), pin(w), pin(f)),
        E::RandomnessUnavailable { .. } | E::Update { .. } => {
            (none(), none(), none(), none(), none(), none())
        }
    };
    typed_err(
        py,
        ErrorClass::Workspace,
        err.to_string(),
        &[
            (
                "variant",
                PyString::new(py, workspace_error_tag(err))
                    .unbind()
                    .into_any(),
            ),
            ("path", p),
            ("id", i),
            ("first", first),
            ("second", second),
            ("wanted", wanted),
            ("found", found),
        ],
    )
}

/// A document version's content pin: the SHA-256 of its canonical
/// semantic bytes.
#[pyclass(frozen, module = "pncad", from_py_object)]
#[derive(Clone, Copy)]
pub(crate) struct ContentPin(pub(crate) d::ContentPin);

#[pymethods]
impl ContentPin {
    /// Parse the canonical text form — exactly 64 lowercase hex
    /// digits, anything else a boundary `ValueError`. The strict
    /// spelling is what `hex` emits, so a loose one is a foreign or
    /// tampered value, not data.
    #[new]
    fn new(hex: &str) -> PyResult<Self> {
        d::ContentPin::parse_hex(hex).map(Self).ok_or_else(|| {
            pyo3::exceptions::PyValueError::new_err(format!(
                "not a content pin: {hex:?} — a pin is exactly 64 lowercase \
                 hex digits, the spelling `ContentPin.hex` answers"
            ))
        })
    }

    /// The canonical text form: exactly 64 lowercase hex digits.
    #[getter]
    fn hex(&self) -> String {
        self.0.hex()
    }

    fn __eq__(&self, other: &Self) -> bool {
        self.0 == other.0
    }

    fn __hash__(&self) -> u64 {
        // The pin is already a cryptographic digest; its first eight
        // bytes are as good a hash as any function of the whole.
        let b = self.0.0;
        u64::from_be_bytes([b[0], b[1], b[2], b[3], b[4], b[5], b[6], b[7]])
    }

    fn __str__(&self) -> String {
        self.0.hex()
    }

    fn __repr__(&self) -> String {
        format!("ContentPin({:?})", self.0.hex())
    }
}

/// A cross-document reference: which part, and which version of it.
#[pyclass(frozen, module = "pncad", from_py_object)]
#[derive(Clone, Copy)]
pub(crate) struct DocRef(pub(crate) d::DocRef);

#[pymethods]
impl DocRef {
    /// Pair an identity with a pin.
    ///
    /// Nothing here consults a store: a `DocRef` is a VALUE, and
    /// whether any store holds that version is `Workspace.resolve`'s
    /// question. Constructing one that resolves nowhere is legal and
    /// is exactly what a stale reference is.
    #[new]
    fn new(id: &str, pin: &ContentPin) -> PyResult<Self> {
        Ok(Self(d::DocRef {
            id: document_id(id)?,
            pin: pin.0,
        }))
    }

    /// The referenced document's identity, 32 lowercase hex digits.
    #[getter]
    fn id(&self) -> String {
        self.0.id.hex()
    }

    /// The pinned version.
    #[getter]
    fn pin(&self) -> ContentPin {
        ContentPin(self.0.pin)
    }

    fn __eq__(&self, other: &Self) -> bool {
        self.0 == other.0
    }

    fn __hash__(&self) -> u64 {
        let b = self.0.pin.0;
        (self.0.id.0 as u64) ^ u64::from_be_bytes([b[0], b[1], b[2], b[3], b[4], b[5], b[6], b[7]])
    }

    fn __repr__(&self) -> String {
        // Rust's `Display` abbreviates the pin for prose; a repr is
        // read to reconstruct a value, so it carries both in full.
        format!(
            "DocRef({:?}, ContentPin({:?}))",
            self.0.id.hex(),
            self.0.pin.hex()
        )
    }
}

/// A directory of save files, scanned into an id → path map.
#[pyclass(module = "pncad")]
pub(crate) struct Workspace {
    inner: ws::Workspace,
}

impl Workspace {
    /// This store AS the document seam an evaluation crosses — the
    /// `resolver=` argument of [`super::value::evaluate`].
    ///
    /// A store IS a `PartResolver` (`pncad::workspace`'s own impl), so
    /// nothing is adapted here; what this door adds is a SNAPSHOT.
    /// The kernel wants an owned `Arc<dyn PartResolver>` and the
    /// Python object is mutable through `create`/`resave`, so the scan
    /// is copied as of the call: the evaluation resolves against the
    /// store the caller passed, and a `create` made while it runs
    /// cannot change what it already resolved. The copy is the id →
    /// path map, not the documents — bodies are read from disk at
    /// resolve time either way.
    pub(crate) fn resolver(&self) -> Arc<dyn d::PartResolver> {
        Arc::new(self.inner.clone())
    }
}

#[pymethods]
impl Workspace {
    /// Scan `path` for `*.pncad` files and read each one's `id:`
    /// header — never its body, which is the header's whole purpose.
    ///
    /// Everything CLAIMING to be a document must scan clean: a
    /// `.pncad` file with an unreadable header refuses the whole
    /// open, and two files claiming one id refuse naming both
    /// (`variant == "duplicate_id"`, with `first` and `second`).
    /// Non-`.pncad` entries and subdirectories are ignored — they are
    /// not documents.
    ///
    /// Raises `WorkspaceError`, typed.
    #[new]
    fn new(py: Python<'_>, path: &str) -> PyResult<Self> {
        ws::Workspace::open(path)
            .map(|inner| Self { inner })
            .map_err(|err| workspace_err(py, &err))
    }

    /// The scanned directory.
    #[getter]
    fn root(&self) -> String {
        self.inner.root().display().to_string()
    }

    /// The scan, as identity → file path. Iteration order is the
    /// store's own: sorted by id, from a path-sorted scan, so it does
    /// not depend on readdir order.
    fn documents(&self, py: Python<'_>) -> PyResult<Py<PyDict>> {
        let out = PyDict::new(py);
        for (id, path) in self.inner.documents() {
            out.set_item(id.hex(), path.display().to_string())?;
        }
        Ok(out.unbind())
    }

    /// Load the document a `DocRef` names, through the full door
    /// sequence `load` runs — validation, replay, ambient-ε
    /// reconciliation — and hand it back **iff its recomputed pin is
    /// the pin the reference carries**.
    ///
    /// A moved pin is a typed refusal (`variant == "pin_mismatch"`,
    /// carrying `wanted` and `found`), never a silent retarget: a
    /// reference names a VERSION, and the recourse is to record the
    /// acceptance of the new one.
    ///
    /// The load's replay runs below the `Doc` wrapper, so the returned
    /// document reports no `last_maintenance` even where the replayed
    /// history's last edit performed some — that reading starts here.
    ///
    /// Raises `WorkspaceError`, typed.
    fn resolve(&self, py: Python<'_>, reference: &DocRef) -> PyResult<super::doc::Doc> {
        let tol = Tol::witness();
        self.inner
            .resolve(&reference.0, tol)
            .map(|inner| super::doc::Doc {
                inner,
                maintenance: Vec::new(),
            })
            .map_err(|err| workspace_err(py, &err))
    }

    /// The pin the store's CURRENT content for `id` hashes to — the
    /// version a reference would move onto.
    ///
    /// Differs from `resolve` by exactly one thing: no expected pin
    /// is supplied, so there is nothing to disagree with.
    ///
    /// Raises `WorkspaceError`, typed.
    fn current_pin(&self, py: Python<'_>, id: &str) -> PyResult<ContentPin> {
        let tol = Tol::witness();
        self.inner
            .current_pin(document_id(id)?, tol)
            .map(ContentPin)
            .map_err(|err| workspace_err(py, &err))
    }

    /// Write `doc` into the store as a new file, `{id}.pncad`, and
    /// answer its path.
    ///
    /// The name is a pure function of the identity, so two runs write
    /// the same store. An id the store already holds refuses
    /// (`variant == "duplicate_id"`) and nothing is written. The file
    /// is a snapshot with an empty edit log, exactly as `Doc.save`
    /// writes one.
    ///
    /// Raises `WorkspaceError`, typed.
    fn create(&mut self, py: Python<'_>, doc: &super::doc::Doc) -> PyResult<String> {
        let tol = Tol::witness();
        self.inner
            .create(&doc.inner, tol)
            .map(|p| p.display().to_string())
            .map_err(|err| workspace_err(py, &err))
    }

    /// Rewrite an EXISTING document's file with `doc`'s current
    /// state, keeping its path, and answer that path.
    ///
    /// This door never creates: an id the scan never saw refuses
    /// (`variant == "unknown_id"`). The identity is unchanged and the
    /// content is not, so every reference by id stays valid and every
    /// reference by PIN goes stale — which is the point.
    ///
    /// Raises `WorkspaceError`, typed.
    fn resave(&mut self, py: Python<'_>, doc: &super::doc::Doc) -> PyResult<String> {
        let tol = Tol::witness();
        self.inner
            .resave(&doc.inner, tol)
            .map(|p| p.display().to_string())
            .map_err(|err| workspace_err(py, &err))
    }

    /// The edits that move every reference to `id` onto the version
    /// the STORE currently holds — `update_references` with the pin
    /// computed from disk.
    ///
    /// **When this reads the store: once, now.** The current pin is
    /// recomputed from the file on disk at this call, and the returned
    /// edits carry it as a literal. Nothing re-reads later: a resave
    /// between this call and the caller's `apply` leaves the applied
    /// pin naming the older version, because the edits are a snapshot
    /// of the store at this instant and not a subscription to it. Nor
    /// does applying them check that the pin resolves — a pin is
    /// recipe data, and whether it resolves is evaluation's question.
    ///
    /// **What it does NOT read: `doc` from the store.** The document
    /// passed here is the caller's in-memory value, and the edits are
    /// computed against ITS reference sites. Passing a document the
    /// store has never seen is legal and normal — a document being
    /// authored has no file yet.
    ///
    /// Pure: nothing is applied and nothing is written. The caller
    /// applies the whole list or none of it.
    ///
    /// Raises `WorkspaceError`, typed. The store's own arms fire when
    /// the id is unknown or its file will not load; the elaboration's
    /// refusal arrives under `variant == "update"` — the store did its
    /// part and the refusal is about the ASSEMBLY (the id is
    /// referenced nowhere, or every site already names that pin).
    fn update_to_store(
        &self,
        py: Python<'_>,
        doc: &super::doc::Doc,
        id: &str,
    ) -> PyResult<Vec<super::doc::DocEdit>> {
        let tol = Tol::witness();
        ws::update_to_store(&doc.inner, document_id(id)?, &self.inner, tol)
            .map(|edits| {
                edits
                    .into_iter()
                    .map(|inner| super::doc::DocEdit { inner })
                    .collect()
            })
            .map_err(|err| workspace_err(py, &err))
    }

    fn __len__(&self) -> usize {
        self.inner.documents().len()
    }

    fn __repr__(&self) -> String {
        let root = self.inner.root().display().to_string();
        format!(
            "Workspace({root:?}, {} documents)",
            self.inner.documents().len()
        )
    }
}

/// Mint a fresh random document identity from OS randomness — the
/// interactive-authoring constructor, and the same door `Doc()` uses.
///
/// Raises `IdentityError` if the OS entropy source refuses. Identity
/// is never defaulted.
#[pyfunction]
pub(crate) fn random_document_id(py: Python<'_>) -> PyResult<String> {
    ws::random_document_id().map(|id| id.hex()).map_err(|err| {
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
    })
}

/// The document's canonical semantic bytes — what the pin is the
/// SHA-256 of.
///
/// Canonical means what it says: the same document authored two ways
/// serialises to the same bytes, so two callers agree on a pin
/// without agreeing on whitespace. Runs the shared validator first,
/// so an invalid document has no bytes rather than nonsense ones.
///
/// Raises `PersistError`, typed.
#[pyfunction]
pub(crate) fn canonical_bytes(py: Python<'_>, doc: &super::doc::Doc) -> PyResult<Py<PyBytes>> {
    let tol = Tol::witness();
    let bytes = d::canonical_bytes(&doc.inner, tol).map_err(|err| persist_err(py, &err))?;
    Ok(PyBytes::new(py, &bytes).unbind())
}

/// The document's content pin: which VERSION this document is.
///
/// Raises `PersistError`, typed.
#[pyfunction]
pub(crate) fn content_pin(py: Python<'_>, doc: &super::doc::Doc) -> PyResult<ContentPin> {
    let tol = Tol::witness();
    d::content_pin(&doc.inner, tol)
        .map(ContentPin)
        .map_err(|err| persist_err(py, &err))
}

/// Read a save file's identity out of its header alone, without
/// parsing the body — the store's scan door.
///
/// Raises `PersistError`, typed: a missing or malformed header, or a
/// schema too old to carry an id line.
#[pyfunction]
pub(crate) fn header_document_id(py: Python<'_>, text: &str) -> PyResult<String> {
    d::header_document_id(text)
        .map(|id| id.hex())
        .map_err(|err| persist_err(py, &err))
}

/// Register the workspace-store surface on the module.
pub(crate) fn register(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<ContentPin>()?;
    m.add_class::<DocRef>()?;
    m.add_class::<Workspace>()?;
    m.add_function(wrap_pyfunction!(random_document_id, m)?)?;
    m.add_function(wrap_pyfunction!(canonical_bytes, m)?)?;
    m.add_function(wrap_pyfunction!(content_pin, m)?)?;
    m.add_function(wrap_pyfunction!(header_document_id, m)?)?;
    m.add("PIN_MISMATCH_RECOURSE", ws::PIN_MISMATCH_RECOURSE)?;
    Ok(())
}
