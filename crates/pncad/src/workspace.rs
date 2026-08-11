//! The workspace store — read side only (ASM-1 D-5).
//!
//! A workspace is a directory of `*.pncad` save files. [`Workspace::open`]
//! scans it, reading each file's `id:` header line (never the body —
//! that is the header's whole purpose, spec D-6) into an id → path
//! map; a duplicate id is a typed refusal naming both paths, because
//! identity is the store's uniqueness invariant (construction does
//! not enforce it — spec D-1). [`Workspace::resolve`] takes a
//! [`DocRef`], loads the named document through the full v5 door
//! sequence ([`crate::document::load`]: validation, replay, ambient-ε
//! reconciliation — all exactly as at any other load), recomputes the
//! canonical content pin, and refuses a moved pin typed
//! ([`WorkspaceError::PinMismatch`]) — an assembly is a self-contained
//! reproducible value (ASSEMBLY-DESIGN A4's Cargo.lock semantics), so
//! an out-of-date pin is surfaced, never silently retargeted.
//!
//! No write side lives here: creating files stays
//! [`crate::document::save`] + `std::fs` at the callers.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use crate::document::{
    ContentPin, DocRef, DocumentId, PersistError, ProfileDoc, content_pin, header_document_id, load,
};

/// Mints a fresh random [`DocumentId`] from OS randomness — the
/// interactive-authoring constructor (ASM-1 D-1). It lives HERE, in
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
/// on, naming the edit that legitimately moves a pin (ASSEMBLY-DESIGN
/// A4: "accept updated version" is a recorded `DocEdit`, planned for
/// the split/inline unit — pins never move silently). Public so
/// callers can assert on it without restating prose.
pub const PIN_MISMATCH_RECOURSE: &str = "the referenced document changed since this reference was pinned; if the new version is \
     intended, record an \"accept updated version\" edit (the planned pin-moving DocEdit) — \
     references are never retargeted silently";

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
    /// fix is mechanical (spec D-5).
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
    /// (ASSEMBLY-DESIGN A4). Recourse: [`PIN_MISMATCH_RECOURSE`].
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
    /// The OS entropy source refused ([`random_document_id`]).
    RandomnessUnavailable {
        /// The source's message.
        message: String,
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
            Self::RandomnessUnavailable { message } => {
                write!(f, "workspace: OS randomness unavailable: {message}")
            }
        }
    }
}

impl core::error::Error for WorkspaceError {}

/// An opened workspace: the scanned id → path map (read side only,
/// spec D-5; module docs).
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
    /// reconciliation refusal included), and
    /// [`WorkspaceError::PinMismatch`] carrying both pins and the
    /// recourse ([`PIN_MISMATCH_RECOURSE`]).
    pub fn resolve(&self, doc_ref: &DocRef) -> Result<ProfileDoc, WorkspaceError> {
        let path = self
            .by_id
            .get(&doc_ref.id)
            .ok_or(WorkspaceError::UnknownId { id: doc_ref.id })?;
        let text = std::fs::read_to_string(path).map_err(|e| WorkspaceError::Io {
            path: path.clone(),
            message: e.to_string(),
        })?;
        let loaded = load(&text).map_err(|error| WorkspaceError::Load {
            path: path.clone(),
            error: Box::new(error),
        })?;
        // Pin the REPLAYED document (D-3: the pin is of the canonical
        // form of current state — never the raw snapshot, never file
        // bytes; a save carrying a non-empty log must pin its
        // replayed result).
        let found = content_pin(&loaded.doc).map_err(|error| WorkspaceError::Pin {
            path: path.clone(),
            error: Box::new(error),
        })?;
        if found != doc_ref.pin {
            return Err(WorkspaceError::PinMismatch {
                id: doc_ref.id,
                path: path.clone(),
                wanted: doc_ref.pin,
                found,
            });
        }
        Ok(loaded.doc)
    }
}
