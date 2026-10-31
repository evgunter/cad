//! Typed `open(path)` / `save(path)` over the shipped persistence.
//!
//! # Where the I/O lives, and why here
//!
//! The kernel's persistence doors are pure: `save` returns a `String`,
//! `load` consumes one, and neither touches a filesystem. Reading the
//! bytes is therefore an interaction-layer act, and writing them is the
//! store's ([`save`]). These are ordinary typed operations callable with
//! no renderer — a file dialog is a way of choosing the `Path` argument,
//! never a different code path — which is why the round trip is
//! exercised headlessly and only the dialog itself escapes.
//!
//! # A file is the document's state (GQ3)
//!
//! A save writes the current document with an empty edit log, and
//! [`open`] answers a history whose root is the loaded document: undo
//! is session state and ends at an open.
//!
//! Module kind: **vocabulary** — it names no driver type and no
//! `app`-only crate (`crates/viewer/README.md`, Module boundaries).

use std::path::{Path, PathBuf};
use std::sync::{Arc, OnceLock};

use pncad::document::{
    DocRef, PartResolver, PersistError, ProfileDoc, Recourse, ResolveFailure, Staged, load,
};
use pncad::geom_core::Tol;
use pncad::workspace::{Scan, Workspace, WorkspaceError};

use crate::history::History;

/// The viewer's document seam: part references resolve against **the
/// opened file's own directory**, and nothing else.
///
/// **The directory rule, stated once.** The assembly gallery's part
/// documents sit beside the assembly that pins them, so the store a
/// session consults is exactly the directory of the file it opened —
/// never a remembered directory from a previous document, never a
/// search path. A session with no backing file resolves through
/// [`NoFile`], which refuses every reference stating the viewer's way
/// through: save the document beside its parts.
///
/// **The scan happens at RESOLUTION time, not at open.** Opening a
/// document must not fail because an unrelated sibling file is
/// corrupt or two junk files collide on an id — a directory is not
/// required to be a healthy store until something actually resolves
/// through it. When a resolution IS attempted and the scan refuses
/// (unreadable header, duplicate id), that refusal arrives typed at
/// the instantiate node, carrying the store's sentence naming the
/// offending files — the tree badge GUI-3 built is where it renders.
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

    /// **The store, opened.** Every read of the session's directory
    /// goes through here — resolution below, and the authoring doors
    /// that list it or mint a pin from it — so the directory rule owns
    /// store access rather than merely describing it, and no second
    /// route can come to consult a different directory.
    ///
    /// The scan happens per call, which is the scan-at-resolution
    /// posture above: this type holds a path, never a cached store.
    ///
    /// # Errors
    ///
    /// The scan's own refusal — [`pncad::workspace::WorkspaceError::Io`],
    /// `Header`, `DuplicateId` — which every caller surfaces typed.
    pub fn workspace(&self) -> Result<Workspace, pncad::workspace::WorkspaceError> {
        Workspace::open(&self.dir)
    }
}

impl PartResolver for DirResolver {
    fn resolve(&self, doc_ref: &DocRef, tol: Tol) -> Result<ProfileDoc, ResolveFailure> {
        // The scan's refusal is the store's sentence, without its stage
        // word as the store's own resolution carries it; the fault
        // classification is `Unresolved` because the reference itself
        // was never reached.
        let workspace = self
            .workspace()
            .map_err(|error| ResolveFailure::unresolved(error.sentence().to_string()))?;
        workspace
            .resolve(doc_ref, tol)
            .map_err(|error| error.resolve_failure(Scan::PerResolution))
    }
}

/// **The resolver of a session with no backing file.** No directory
/// holds its parts, so every reference refuses; saving the document
/// beside its parts gives the session a [`DirResolver`] over them.
#[derive(Debug)]
pub struct NoFile;

impl NoFile {
    /// The one `NoFile` seam every session shares. The evaluation's
    /// memo is keyed by its seam's identity, and this seam answers
    /// every reference the same way, so one identity serves all.
    pub fn seam() -> Arc<dyn PartResolver> {
        static SEAM: OnceLock<Arc<dyn PartResolver>> = OnceLock::new();
        Arc::clone(SEAM.get_or_init(|| Arc::new(NoFile)))
    }
}

impl PartResolver for NoFile {
    fn resolve(&self, _: &DocRef, _: Tol) -> Result<ProfileDoc, ResolveFailure> {
        Err(ResolveFailure::unresolved(format!(
            "this document has no file, so no directory holds its parts. {}",
            Recourse("save it beside its parts")
        )))
    }
}

/// A refusal on the way from a file.
#[derive(Debug)]
pub enum DocIoError {
    /// The file could not be read.
    Read {
        /// What the OS said.
        message: String,
    },
    /// The document layer refused the bytes. Boxed so the refusal
    /// stays a small `Err`.
    Persist(Box<PersistError>),
}

impl core::fmt::Display for DocIoError {
    /// The payload arm delegates to `PersistError`'s own `Display`,
    /// which is the same rule the feature tree's badges follow.
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Read { message } => write!(f, "cannot read the file: {message}"),
            Self::Persist(error) => write!(f, "{error}"),
        }
    }
}

impl core::error::Error for DocIoError {}

/// Open a document file as a history holding one state: the document
/// the file carries.
///
/// # Errors
///
/// [`DocIoError::Read`] for the filesystem, [`DocIoError::Persist`]
/// for a file the document layer refuses.
pub fn open(path: &Path, tol: Tol) -> Result<History, DocIoError> {
    let text = std::fs::read_to_string(path).map_err(|e| DocIoError::Read {
        message: e.to_string(),
    })?;
    let loaded = load(&text, tol).map_err(|e| DocIoError::Persist(Box::new(e)))?;
    Ok(History::new(loaded.doc))
}

/// Write `doc` at `path` through the store of the folder the file
/// lands in ([`file_dir`]): that folder's scan decides whether this is
/// a create, a resave, or a refusal, before anything is written
/// (`pncad::workspace::Workspace::save_at`).
///
/// # Errors
///
/// The scan's own refusal (`Io`, `Header`, `DuplicateId`), then
/// `save_at`'s: `SaveWouldDuplicateId` for an id the folder holds
/// under another file name, `SaveTargetNotInStore` for a target that
/// is not a `*.pncad` file, `Save` and `Io`.
pub fn save(path: &Path, doc: &ProfileDoc, tol: Tol) -> Result<PathBuf, WorkspaceError> {
    Workspace::open(file_dir(path))?.save_at(doc, path, tol)
}

/// The folder a document at `path` lives in — its parent, with a bare
/// file name reading as the current directory. It is both the store a
/// save writes through and the directory the session resolves against.
pub fn file_dir(path: &Path) -> PathBuf {
    match path.parent() {
        Some(parent) if !parent.as_os_str().is_empty() => parent.to_path_buf(),
        _ => PathBuf::from("."),
    }
}
