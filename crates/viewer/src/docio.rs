//! Typed `open(path)` / `save(path)` over the shipped snapshot + edit
//! log persistence.
//!
//! # Where the I/O lives, and why here
//!
//! The kernel's persistence doors are pure: `save` returns a `String`,
//! `load` consumes one, and neither touches a filesystem. Reading and
//! writing the bytes is therefore an interaction-layer act, and these
//! two functions are the whole of it. They are ordinary typed
//! operations callable with no renderer — a file dialog is a way of
//! choosing the `Path` argument, never a different code path — which
//! is why the round trip is exercised headlessly and only the dialog
//! itself escapes.
//!
//! # A file becomes a history, not a document
//!
//! `load` answers with the snapshot, the log, and the replayed
//! document. [`open`] keeps all three: the snapshot is the history's
//! root and each logged edit is a commit, so the file's log IS the
//! current path of the undo tree. Saving straight back therefore
//! writes the same snapshot and the same log — persistence is
//! untouched by the tree, which is exactly the plan's undo note.

use std::path::{Path, PathBuf};

use pncad::document::{
    Doc, DocRef, PartResolver, PersistError, ProfileDoc, ProfileProgram, ResolveFailure,
    ResolveFault, load, save,
};
use pncad::geom_core::Tol;
use pncad::workspace::Workspace;

use crate::history::{History, ReplayError};

/// The viewer's document seam: part references resolve against **the
/// opened file's own directory**, and nothing else.
///
/// **The directory rule, stated once.** The assembly gallery's part
/// documents sit beside the assembly that pins them, so the store a
/// session consults is exactly the directory of the file it opened —
/// never a remembered directory from a previous document, never a
/// search path. A session with no backing file carries no resolver at
/// all, and its instantiate nodes refuse with the shipped no-resolver
/// semantics.
///
/// **The scan happens at RESOLUTION time, not at open.** Opening a
/// document must not fail because an unrelated sibling file is
/// corrupt or two junk files collide on an id — a directory is not
/// required to be a healthy store until something actually resolves
/// through it. When a resolution IS attempted and the scan refuses
/// (unreadable header, duplicate id), that refusal arrives typed at
/// the instantiate node, carrying the workspace's own message naming
/// the offending files — the tree badge GUI-3 built is where it
/// renders.
#[derive(Debug)]
pub struct DirResolver {
    dir: PathBuf,
}

impl DirResolver {
    /// A resolver over `dir`.
    pub fn new(dir: PathBuf) -> Self {
        Self { dir }
    }

    /// The directory this resolver consults.
    pub fn dir(&self) -> &Path {
        &self.dir
    }
}

impl PartResolver for DirResolver {
    fn resolve(&self, doc_ref: &DocRef, tol: Tol) -> Result<ProfileDoc, ResolveFailure> {
        let workspace = Workspace::open(&self.dir).map_err(|error| ResolveFailure {
            // The scan's refusal is the store's, verbatim; the fault
            // classification is `Unresolved` because the reference
            // itself was never reached.
            fault: ResolveFault::Unresolved,
            message: error.to_string(),
        })?;
        PartResolver::resolve(&workspace, doc_ref, tol)
    }
}

/// A refusal on the way to or from a file.
#[derive(Debug)]
pub enum DocIoError {
    /// The file could not be read.
    Read {
        /// What the OS said.
        message: String,
    },
    /// The file could not be written.
    Write {
        /// What the OS said.
        message: String,
    },
    /// The document layer refused the bytes (or refused to produce
    /// them).
    Persist(PersistError),
    /// The saved log did not replay through `apply`.
    Replay(ReplayError),
}

impl core::fmt::Display for DocIoError {
    /// The persistence arm delegates to `PersistError`'s own `Display`
    /// — the same rule the feature tree's badges follow. The replay arm
    /// names the log position and delegates to `EditError`.
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Read { message } => write!(f, "cannot read the file: {message}"),
            Self::Write { message } => write!(f, "cannot write the file: {message}"),
            Self::Persist(error) => write!(f, "{error}"),
            Self::Replay(ReplayError::Refused { index, error }) => {
                write!(f, "edit {index} of the saved log was refused: {error}")
            }
        }
    }
}

impl core::error::Error for DocIoError {}

/// Open a document file as an edit history.
///
/// # Errors
///
/// [`DocIoError::Read`] for the filesystem, [`DocIoError::Persist`]
/// for a file the document layer refuses, [`DocIoError::Replay`] for a
/// log that will not replay.
pub fn open(path: &Path, tol: Tol) -> Result<History, DocIoError> {
    let text = std::fs::read_to_string(path).map_err(|e| DocIoError::Read {
        message: e.to_string(),
    })?;
    let loaded = load(&text, tol).map_err(DocIoError::Persist)?;
    History::replayed(loaded.snapshot, &loaded.edits, tol).map_err(DocIoError::Replay)
}

/// Write the history's current path: its root snapshot and the edits
/// along the branch the cursor is on.
///
/// Branches the cursor is not on are NOT written. That is the v1
/// persistence contract — a linear log — and the states those branches
/// hold stay in the session; the separable history sidecar is the
/// future work that would carry them to disk.
///
/// # Errors
///
/// [`DocIoError::Persist`] if the document layer refuses to serialize
/// (which includes its own replay check), [`DocIoError::Write`] for
/// the filesystem.
pub fn save_path(path: &Path, history: &History, tol: Tol) -> Result<(), DocIoError> {
    let root: &Doc<ProfileProgram> = history.entry(history.root()).doc();
    let text = save(root, &history.path_edits(), tol).map_err(DocIoError::Persist)?;
    std::fs::write(path, text).map_err(|e| DocIoError::Write {
        message: e.to_string(),
    })
}
