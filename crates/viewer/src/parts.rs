//! **The part catalogue**: which documents the open document's own
//! directory offers as instances, and the chooser state the `Add
//! part…` door holds while a user reads them.
//!
//! # The directory rule, consumed rather than restated
//!
//! A reference resolves against the directory of the file the session
//! opened ([`crate::docio::DirResolver`]), so that same directory is
//! the only place an instance can be picked FROM: a catalogue built
//! anywhere else would offer parts the authored reference could not
//! resolve. The listing is therefore [`pncad::workspace::Workspace`]'s
//! own scan of that one directory — id → path, in the store's
//! deterministic order — and a session with no backing file has no
//! catalogue at all, which is the same fact the resolver states about
//! resolution.
//!
//! # The scan is taken once, not per frame
//!
//! Opening a workspace reads every `.pncad` file's header, so the
//! catalogue is a SNAPSHOT: taken when the chooser opens, held as a
//! value, and re-taken only when asked for ([`PartChooser::rescan`]).
//! A directory that changed under a chooser left open is exactly what
//! the re-scan is for, and picking a stale entry refuses at the op —
//! the pin is minted at commit from the store's current content, never
//! from anything this value remembers.

use std::path::{Path, PathBuf};

use pncad::document::DocumentId;
use pncad::workspace::{Workspace, WorkspaceError};

use crate::session::{DocSession, Refusal};

/// One document the catalogue offers, as the chooser shows it: which
/// part it is, which file it lives in, and whether it is the open
/// document itself.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PartEntry {
    /// The document's stable identity — what the authored reference
    /// carries (A4's "which part"; the version is minted at commit).
    pub id: DocumentId,
    /// The file the scan found it in, inside the catalogue's
    /// directory.
    pub path: PathBuf,
    /// Set for the entry that IS the document being edited.
    ///
    /// Kept in the listing rather than filtered out of it, because a
    /// door that cannot open should say so rather than vanish: a user
    /// looking for their own document's name finds it, disabled, with
    /// the reason — where a silently shorter list reads as a missing
    /// file. The op refuses this entry typed
    /// ([`Refusal::SelfInstance`]).
    pub open_document: bool,
}

impl PartEntry {
    /// The file's own name, as the chooser labels it — the whole path
    /// when the path names no file (which the scan cannot produce, and
    /// which is shown rather than hidden if it ever does).
    pub fn file_name(&self) -> String {
        self.path.file_name().map_or_else(
            || self.path.display().to_string(),
            |n| n.to_string_lossy().into_owned(),
        )
    }
}

/// The documents `dir` offers as parts, with the open document's own
/// entry marked.
///
/// The order is the workspace's: its id → path map, which its scan
/// fills in path-sorted, so two runs over one directory list the same
/// parts in the same order.
///
/// # Errors
///
/// The scan's own refusal, verbatim — [`WorkspaceError::DuplicateId`]
/// naming both claimants, [`WorkspaceError::Header`] naming an
/// unreadable sibling, [`WorkspaceError::Io`] naming the directory.
/// The catalogue never partially succeeds: a directory that is not a
/// healthy store cannot answer "which parts are here" honestly.
pub fn catalogue(dir: &Path, open: DocumentId) -> Result<Vec<PartEntry>, WorkspaceError> {
    let workspace = Workspace::open(dir)?;
    Ok(workspace
        .documents()
        .iter()
        .map(|(&id, path)| PartEntry {
            id,
            path: path.clone(),
            open_document: id == open,
        })
        .collect())
}

/// The `Add part…` chooser's held state: the catalogue as of its
/// scan, and the directory it was taken in.
///
/// Layer-3 state and nothing else — it never enters the document,
/// never enters the history, and dies with the chooser (G1's
/// transient-state rule, the mate tool's posture one size down).
#[derive(Debug)]
pub struct PartChooser {
    /// The directory the scan read, when the session had one.
    dir: Option<PathBuf>,
    /// What that scan answered: the parts on offer, or the typed
    /// refusal the chooser renders in place of a list.
    offered: Result<Vec<PartEntry>, Refusal>,
}

impl PartChooser {
    /// Open a chooser over `session`, taking the scan now.
    pub fn opened(session: &DocSession) -> Self {
        Self {
            dir: session.resolve_dir().map(Path::to_path_buf),
            offered: session.part_catalogue(),
        }
    }

    /// Re-take the scan, in place: the answer to a directory that
    /// changed while the chooser was open.
    pub fn rescan(&mut self, session: &DocSession) {
        *self = Self::opened(session);
    }

    /// The directory the entries came from, for the chooser's header.
    /// `None` for a session with no backing file — the case
    /// [`Self::offered`] refuses.
    pub fn dir(&self) -> Option<&Path> {
        self.dir.as_deref()
    }

    /// The scan's answer: the parts on offer, or the refusal to show
    /// instead of a list.
    pub fn offered(&self) -> Result<&[PartEntry], &Refusal> {
        match &self.offered {
            Ok(entries) => Ok(entries.as_slice()),
            Err(refusal) => Err(refusal),
        }
    }
}
