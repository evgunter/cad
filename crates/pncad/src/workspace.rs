//! The workspace store: a directory of save files, read and written.
//!
//! A workspace is a directory of `*.pncad` save files. [`Workspace::open`]
//! scans it, reading each file's `id:` header line (never the body —
//! that is the header's whole purpose) into an id → path
//! map; a duplicate id is a typed refusal naming both paths, because
//! identity is the store's uniqueness invariant (construction does
//! not enforce it). [`Workspace::resolve`] takes a
//! [`DocRef`], loads the named document through the full v5 door
//! sequence ([`crate::document::load`]: validation, replay, ambient-ε
//! reconciliation — all exactly as at any other load), recomputes the
//! canonical content pin, and refuses a moved pin typed
//! ([`WorkspaceError::PinMismatch`]) — an assembly is a self-contained
//! reproducible value (Cargo.lock semantics), so
//! an out-of-date pin is surfaced, never silently retargeted.
//!
//! The write side is deliberately MINIMAL — exactly what the split/
//! inline refactorings need, and no general mutation API — [`Workspace::create`] mints a new save
//! file from a `Doc` (the id is the caller's:
//! [`DocumentId::derive`] for deterministic callers,
//! [`random_document_id`] for interactive authoring), and
//! [`Workspace::resave`] rewrites an existing document's file by id.
//! Duplicate-id refusal is unchanged, and there is no general mutation
//! API: split and inline are the only intended writers. Both write the
//! CURRENT state as a snapshot with an empty log (history is not
//! state; the refactoring's own record is its returned edit lists).
//!
//! [`DocumentId::derive`]: crate::document::DocumentId::derive

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use crate::document::{
    ContentPin, DocRef, DocumentId, PartResolver, PersistError, ProfileDoc, ResolveFailure,
    ResolveFault, content_pin, header_document_id, load, save,
};
use geom_core::Tol;

/// Mints a fresh random [`DocumentId`] from OS randomness — the
/// interactive-authoring constructor. It lives HERE, in
/// the document layer, so the kernel stays
/// deterministic-by-construction: corpus and demo regeneration use
/// [`DocumentId::derive`] and never touch ambient randomness.
///
/// # Errors
///
/// [`WorkspaceError::RandomnessUnavailable`] if the OS entropy source
/// refuses — surfaced, never papered over with a weaker source.
pub fn random_document_id() -> Result<DocumentId, WorkspaceError> {
    let mut bytes = [0u8; 16];
    getrandom::fill(&mut bytes).map_err(|e| WorkspaceError::RandomnessUnavailable {
        message: e.to_string(),
    })?;
    Ok(DocumentId(u128::from_be_bytes(bytes)))
}

/// The one recourse sentence a [`WorkspaceError::PinMismatch`] ends
/// on, naming the edit that legitimately moves a pin ("accept updated
/// version" is a recorded `DocEdit` — `DocEdit::UpdateReference`;
/// pins never move silently). Public so
/// callers can assert on it without restating
/// prose.
pub const PIN_MISMATCH_RECOURSE: &str = "the referenced document changed since this reference was pinned; if the new version is \
     intended, record the \"accept updated version\" edit (DocEdit::UpdateReference, or \
     workspace::update_to_store for every site at once) — references are never retargeted \
     silently";

/// Typed workspace refusal (fail loud; no silent best-effort scans
/// or resolves).
#[derive(Debug)]
pub enum WorkspaceError {
    /// A filesystem operation failed, naming the path.
    Io {
        /// The path the operation touched (the directory for the
        /// scan's `read_dir`, the file otherwise).
        path: PathBuf,
        /// The OS error's message.
        message: String,
    },
    /// Two files in the workspace claim the same document id — the
    /// store's uniqueness invariant, refused naming BOTH paths so the
    /// fix is mechanical.
    DuplicateId {
        /// The contested id.
        id: DocumentId,
        /// The path scanned first.
        first: PathBuf,
        /// The path that repeated the id.
        second: PathBuf,
    },
    /// A file's header refused (no/malformed `schema:` or `id:` line,
    /// or a pre-v5 schema), naming the file.
    Header {
        /// The refusing file.
        path: PathBuf,
        /// The typed persistence refusal (boxed: `PersistError` is a
        /// large enum and this error rides many `Result`s).
        error: Box<PersistError>,
    },
    /// The workspace has no document with the requested id.
    UnknownId {
        /// The id no scanned file claims.
        id: DocumentId,
    },
    /// The resolved file refused to load (the full door sequence:
    /// parse, validate, replay, ε reconciliation), naming the file.
    Load {
        /// The refusing file.
        path: PathBuf,
        /// The typed persistence refusal (boxed, as in `Header`).
        error: Box<PersistError>,
    },
    /// The document loaded, but recomputing its content pin refused
    /// (the canonical serializer itself failed — unreachable short of
    /// a serde bug, since the load just validated the same document;
    /// surfaced under its own arm rather than mislabeled as a load
    /// refusal).
    Pin {
        /// The file whose loaded document would not pin.
        path: PathBuf,
        /// The typed persistence refusal (boxed, as in `Header`).
        error: Box<PersistError>,
    },
    /// The document loaded, but its recomputed content pin is not the
    /// pin the reference carries: the referenced document CHANGED
    /// Recourse: [`PIN_MISMATCH_RECOURSE`].
    PinMismatch {
        /// The reference's id.
        id: DocumentId,
        /// The file that resolved.
        path: PathBuf,
        /// The pin the reference expects.
        wanted: ContentPin,
        /// The pin the document actually hashes to.
        found: ContentPin,
    },
    /// The document refused to serialize for a workspace write (the
    /// same shared validator every save runs) — surfaced before any
    /// file is touched, so a refused write leaves the store unchanged.
    Save {
        /// The document that would not save.
        id: DocumentId,
        /// The typed persistence refusal (boxed, as in `Header`).
        error: Box<PersistError>,
    },
    /// The OS entropy source refused ([`random_document_id`]).
    RandomnessUnavailable {
        /// The source's message.
        message: String,
    },
    /// [`update_to_store`] found the store's current pin, and the
    /// document-layer elaboration refused it: the id is
    /// referenced nowhere, or every reference already names that pin.
    /// The store did its part; the refusal is about the ASSEMBLY.
    Update {
        /// The typed elaboration refusal.
        error: crate::document::UpdateError,
    },
}

impl core::fmt::Display for WorkspaceError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Io { path, message } => {
                write!(f, "workspace: io error at {}: {message}", path.display())
            }
            Self::DuplicateId { id, first, second } => write!(
                f,
                "workspace: duplicate document id {id}: {} and {} both claim it — \
                 document ids are unique per workspace",
                first.display(),
                second.display()
            ),
            Self::Header { path, error } => {
                write!(f, "workspace: {} refused: {error}", path.display())
            }
            Self::UnknownId { id } => {
                write!(f, "workspace: no document with id {id}")
            }
            Self::Load { path, error } => {
                write!(f, "workspace: {} refused to load: {error}", path.display())
            }
            Self::Pin { path, error } => write!(
                f,
                "workspace: {} loaded but its content pin would not compute: {error}",
                path.display()
            ),
            Self::PinMismatch {
                id,
                path,
                wanted,
                found,
            } => write!(
                f,
                "workspace: pin mismatch for document {id} at {}: the reference pins \
                 {wanted} but the document hashes to {found} — {PIN_MISMATCH_RECOURSE}",
                path.display()
            ),
            Self::Save { id, error } => {
                write!(f, "workspace: document {id} refused to save: {error}")
            }
            Self::RandomnessUnavailable { message } => {
                write!(f, "workspace: OS randomness unavailable: {message}")
            }
            Self::Update { error } => write!(f, "workspace: {error}"),
        }
    }
}

impl core::error::Error for WorkspaceError {}

/// An opened workspace: the scanned id → path map, plus the two
/// write doors ([`Workspace::create`], [`Workspace::resave`]) the
/// refactorings need. See the module docs.
#[derive(Debug, Clone)]
pub struct Workspace {
    /// The scanned directory.
    root: PathBuf,
    /// Document id → save file, from the scan.
    by_id: BTreeMap<DocumentId, PathBuf>,
}

impl Workspace {
    /// Scans `dir` for `*.pncad` files and builds the id → path map
    /// from their `id:` header lines (bodies stay unparsed until
    /// [`Self::resolve`]). Non-`.pncad` entries and subdirectories
    /// are ignored — they are not documents; everything CLAIMING to
    /// be a document must scan clean, so a `.pncad` file with an
    /// unreadable header refuses the whole open.
    ///
    /// # Errors
    ///
    /// [`WorkspaceError::Io`] naming the failing path,
    /// [`WorkspaceError::Header`] for an unreadable header (including
    /// pre-v5 schemas), [`WorkspaceError::DuplicateId`] naming both
    /// claimants.
    pub fn open(dir: impl AsRef<Path>) -> Result<Self, WorkspaceError> {
        let root = dir.as_ref().to_path_buf();
        let io = |path: &Path| {
            let path = path.to_path_buf();
            move |e: std::io::Error| WorkspaceError::Io {
                path,
                message: e.to_string(),
            }
        };
        let mut paths: Vec<PathBuf> = Vec::new();
        for entry in std::fs::read_dir(&root).map_err(io(&root))? {
            let entry = entry.map_err(io(&root))?;
            let path = entry.path();
            if path.is_file() && path.extension().is_some_and(|ext| ext == "pncad") {
                paths.push(path);
            }
        }
        // Deterministic scan order (D9 posture: no readdir-order
        // dependence), and the order that makes DuplicateId's
        // first/second stable.
        paths.sort();
        let mut by_id: BTreeMap<DocumentId, PathBuf> = BTreeMap::new();
        for path in paths {
            let text = std::fs::read_to_string(&path).map_err(io(&path))?;
            let id = header_document_id(&text).map_err(|error| WorkspaceError::Header {
                path: path.clone(),
                error: Box::new(error),
            })?;
            if let Some(first) = by_id.get(&id) {
                return Err(WorkspaceError::DuplicateId {
                    id,
                    first: first.clone(),
                    second: path,
                });
            }
            by_id.insert(id, path);
        }
        Ok(Self { root, by_id })
    }

    /// The scanned directory.
    pub fn root(&self) -> &Path {
        &self.root
    }

    /// The scanned id → path map (insertion is path-sorted, so
    /// iteration is deterministic).
    pub fn documents(&self) -> &BTreeMap<DocumentId, PathBuf> {
        &self.by_id
    }

    /// Resolves a [`DocRef`]: loads the id's file through the full
    /// door sequence (v5 validation, replay, ambient-ε reconciliation
    /// — exactly as any load), recomputes the canonical content pin,
    /// and hands back the replayed document iff the pin matches.
    ///
    /// # Errors
    ///
    /// [`WorkspaceError::UnknownId`], [`WorkspaceError::Io`],
    /// [`WorkspaceError::Load`] (any [`PersistError`], the ambient-ε
    /// reconciliation refusal included), [`WorkspaceError::Pin`] if
    /// recomputing the pin itself refuses, and
    /// [`WorkspaceError::PinMismatch`] carrying both pins and the
    /// recourse ([`PIN_MISMATCH_RECOURSE`]).
    pub fn resolve(&self, doc_ref: &DocRef, tol: Tol) -> Result<ProfileDoc, WorkspaceError> {
        let (path, doc, found) = self.load_pinned(doc_ref.id, tol)?;
        if found != doc_ref.pin {
            return Err(WorkspaceError::PinMismatch {
                id: doc_ref.id,
                path: path.clone(),
                wanted: doc_ref.pin,
                found,
            });
        }
        Ok(doc)
    }

    /// The pin the store's CURRENT content for `id` hashes to — the
    /// version an update would move a reference onto.
    /// Distinct from [`Workspace::resolve`] by exactly one thing: no
    /// expected pin is supplied, so there is nothing to disagree with.
    ///
    /// # Errors
    ///
    /// [`WorkspaceError::UnknownId`], [`WorkspaceError::Io`],
    /// [`WorkspaceError::Load`], [`WorkspaceError::Pin`] — the read
    /// side's own vocabulary, unchanged.
    pub fn current_pin(&self, id: DocumentId, tol: Tol) -> Result<ContentPin, WorkspaceError> {
        let (_, _, pin) = self.load_pinned(id, tol)?;
        Ok(pin)
    }

    /// The shared read: locate, load through the full door sequence,
    /// and pin the REPLAYED document (the pin is of the
    /// canonical form of current state — never the raw snapshot, never
    /// file bytes; a save carrying a non-empty log must pin its
    /// replayed result).
    fn load_pinned(
        &self,
        id: DocumentId,
        tol: Tol,
    ) -> Result<(&PathBuf, ProfileDoc, ContentPin), WorkspaceError> {
        let path = self
            .by_id
            .get(&id)
            .ok_or(WorkspaceError::UnknownId { id })?;
        let text = std::fs::read_to_string(path).map_err(|e| WorkspaceError::Io {
            path: path.clone(),
            message: e.to_string(),
        })?;
        let loaded = load(&text, tol).map_err(|error| WorkspaceError::Load {
            path: path.clone(),
            error: Box::new(error),
        })?;
        let pin = content_pin(&loaded.doc, tol).map_err(|error| WorkspaceError::Pin {
            path: path.clone(),
            error: Box::new(error),
        })?;
        Ok((path, loaded.doc, pin))
    }

    /// Creates a new save file for `doc` in the workspace (split's
    /// write side) and returns its path. The file is named
    /// `{id}.pncad` — a pure function of the identity, so two split
    /// runs write byte-identical stores (D9). A duplicate id refuses
    /// exactly as the scan does, naming the file that already claims
    /// it; nothing is written on any refusal.
    ///
    /// # Errors
    ///
    /// [`WorkspaceError::DuplicateId`] (the store's uniqueness
    /// invariant — `second` names the path this create would have
    /// written), [`WorkspaceError::Save`] for a document the shared
    /// validator refuses, [`WorkspaceError::Io`] naming the file.
    pub fn create(&mut self, doc: &ProfileDoc, tol: Tol) -> Result<PathBuf, WorkspaceError> {
        let id = doc.id();
        let path = self.root.join(format!("{id}.pncad"));
        if let Some(first) = self.by_id.get(&id) {
            return Err(WorkspaceError::DuplicateId {
                id,
                first: first.clone(),
                second: path,
            });
        }
        let text = save(doc, &[], tol).map_err(|error| WorkspaceError::Save {
            id,
            error: Box::new(error),
        })?;
        std::fs::write(&path, text).map_err(|e| WorkspaceError::Io {
            path: path.clone(),
            message: e.to_string(),
        })?;
        self.by_id.insert(id, path.clone());
        Ok(path)
    }

    /// Rewrites the save file of an EXISTING document with `doc`'s
    /// current state (the remainder's write side). The id
    /// must already be in the store — this door never creates — and
    /// the file keeps its scanned path, so references by id stay
    /// valid while the content (and therefore the pin) moves.
    ///
    /// # Errors
    ///
    /// [`WorkspaceError::UnknownId`] for an id the scan never saw,
    /// [`WorkspaceError::Save`], [`WorkspaceError::Io`].
    pub fn resave(&mut self, doc: &ProfileDoc, tol: Tol) -> Result<PathBuf, WorkspaceError> {
        let id = doc.id();
        let Some(path) = self.by_id.get(&id).cloned() else {
            return Err(WorkspaceError::UnknownId { id });
        };
        let text = save(doc, &[], tol).map_err(|error| WorkspaceError::Save {
            id,
            error: Box::new(error),
        })?;
        std::fs::write(&path, text).map_err(|e| WorkspaceError::Io {
            path: path.clone(),
            message: e.to_string(),
        })?;
        Ok(path)
    }
}

/// The document seam: a workspace
/// IS what an evaluation resolves references through.
///
/// The verdict classification is this layer's because this layer is the
/// one that knows: only the store can tell "the pin does not hold" from
/// "the ε does not reconcile" from "no such document". Every other
/// refusal is `Unresolved` — honestly wide, since the kernel's recourse
/// for all of them is the same.
impl PartResolver for Workspace {
    fn resolve(&self, doc_ref: &DocRef, tol: Tol) -> Result<ProfileDoc, ResolveFailure> {
        Workspace::resolve(self, doc_ref, tol).map_err(|e| ResolveFailure {
            fault: resolve_fault(&e),
            // Not an exception to [`resolve_fault`]'s paragraph
            // below: this match RENDERS rather than classifies, and
            // the wildcard's answer is the enum's own `Display`. A
            // variant added later answers for itself here, and misses
            // only a recourse sentence it does not have.
            message: match &e {
                WorkspaceError::PinMismatch { .. } => format!("{e}; {PIN_MISMATCH_RECOURSE}"),
                _ => e.to_string(),
            },
        })
    }
}

/// Which seam rule a store refusal broke.
///
/// Both this match and [`load_fault`] are EXHAUSTIVE, with no
/// wildcard arm. The kernel's fault vocabulary is deliberately coarse
/// — `Unresolved` is the honest answer wherever the recourse is the
/// same — but coarse has to be a decision taken per arm: a wildcard
/// would classify the next refusal silently, and the two arms that
/// are NOT `Unresolved` exist precisely because the kernel acts
/// differently on them.
fn resolve_fault(e: &WorkspaceError) -> ResolveFault {
    match e {
        WorkspaceError::PinMismatch { .. } => ResolveFault::PinMismatch,
        WorkspaceError::Load { error, .. } => load_fault(error),
        WorkspaceError::Io { .. }
        | WorkspaceError::DuplicateId { .. }
        | WorkspaceError::Header { .. }
        | WorkspaceError::UnknownId { .. }
        | WorkspaceError::Pin { .. }
        | WorkspaceError::Save { .. }
        | WorkspaceError::RandomnessUnavailable { .. }
        | WorkspaceError::Update { .. } => ResolveFault::Unresolved,
    }
}

/// Which seam rule a LOAD refusal broke — the classification by
/// [`PersistError`] variant, matched on the type rather than read out
/// of a rendered message.
fn load_fault(error: &PersistError) -> ResolveFault {
    match error {
        // The ε seam, observed exactly where the load door already
        // refuses it: one process, one ε, so a document recording a
        // different one cannot be evaluated at all.
        PersistError::ToleranceConflict { .. } => ResolveFault::EpsilonSeam,
        PersistError::NonFinite { .. }
        | PersistError::ProfileProgram { .. }
        | PersistError::Serialize { .. }
        | PersistError::Header { .. }
        | PersistError::HeaderId { .. }
        | PersistError::IdMismatch { .. }
        | PersistError::UnknownSchema { .. }
        | PersistError::SchemaTooOld { .. }
        | PersistError::Parse { .. }
        | PersistError::EditReplay { .. }
        | PersistError::Migration(_)
        | PersistError::Snapshot(_)
        | PersistError::ToleranceInvalid { .. } => ResolveFault::Unresolved,
    }
}

/// "Update every reference to `id` in `doc` to whatever the store
/// currently holds" — the workspace-layer convenience over the
/// document layer's [`crate::document::update_references`].
///
/// The ONE thing this adds is the pin: the elaboration is pure and
/// storeless by design, so somebody has to say which version "the new
/// one" is, and only the store knows. That answer is
/// [`Workspace::current_pin`] — the canonical pin of the id's current
/// file, computed through the same load-and-replay door every resolve
/// uses, so "the version on disk" means exactly what it means
/// everywhere else.
///
/// Returns the ordinary edit list, applied to nothing: the caller
/// applies the whole group, exactly as with the storeless door. The
/// document is READ here and never written — persisting the updated
/// assembly is [`Workspace::resave`]'s job, and keeping those separate
/// is what lets an author inspect (or lint) the result before it lands.
///
/// # Errors
///
/// The read side's own vocabulary for a store miss
/// ([`WorkspaceError::UnknownId`], [`WorkspaceError::Io`],
/// [`WorkspaceError::Load`], [`WorkspaceError::Pin`]), and
/// [`WorkspaceError::Update`] carrying the elaboration's typed refusal
/// — an id this document never references, or one every reference
/// already pins.
pub fn update_to_store(
    doc: &ProfileDoc,
    id: DocumentId,
    workspace: &Workspace,
    tol: Tol,
) -> Result<Vec<crate::document::DocEdit<crate::document::ProfileProgram>>, WorkspaceError> {
    let new_pin = workspace.current_pin(id, tol)?;
    crate::document::update_references(doc, id, new_pin)
        .map_err(|error| WorkspaceError::Update { error })
}
