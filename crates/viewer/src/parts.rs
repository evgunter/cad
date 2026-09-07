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
//! own scan of that one directory, taken through the resolver that
//! owns it ([`DirResolver::workspace`]) — and a session with no
//! backing file has no catalogue at all, which is the same fact the
//! resolver states about resolution.
//!
//! # The scan is taken once, not per frame
//!
//! Opening a workspace reads every `.pncad` file's header, so the
//! catalogue is a SNAPSHOT: taken when the chooser opens, held as a
//! value, and re-taken only when asked for ([`PartChooser::rescan`]).
//! A directory that changed under a chooser left open is exactly what
//! the re-scan is for. What a stale entry does when picked follows
//! from the pin being minted at the COMMIT, from the store's current
//! content, never from anything this value remembers: an entry whose
//! file is gone refuses (the store has no such id), and an entry whose
//! file merely CHANGED succeeds — against the new content, which is
//! the version the author is looking at and the one A4 says a fresh
//! reference should carry.
//!
//! Module kind: **vocabulary** (`crates/viewer/README.md`, Module
//! boundaries). It names no driver type and no `app`-only crate: what
//! the chooser needs from a session arrives as [`PartCensus`], which
//! the session mints.

use std::path::{Path, PathBuf};

use pncad::document::DocumentId;
use pncad::workspace::WorkspaceError;

use crate::docio::DirResolver;
use crate::session::Refusal;

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
    /// Why this entry cannot be picked, or `None` when it can — the
    /// SAME refusal the op answers a click on it with, so the disabled
    /// reason and the refused action cannot say different things. The
    /// chrome renders this rather than minting a refusal of its own.
    pub fn refusal(&self) -> Option<Refusal> {
        self.open_document
            .then_some(Refusal::SelfInstance { id: self.id })
    }

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

/// The documents the resolver's directory offers as parts, with the
/// open document's own entry marked.
///
/// **Ordered by FILE NAME**, with the id as the tie-break. The
/// workspace answers a `BTreeMap<DocumentId, _>`, so its own iteration
/// order is by identity — and an id is a 32-digit hash, which sorts a
/// chooser into an order no reader can predict or scan. The name is
/// what a person picks by, so it is what the listing is sorted on; the
/// id tie-break keeps two same-named files (different directories
/// cannot arise here, but a rename race can) in a deterministic order.
///
/// # Errors
///
/// The scan's own refusal, verbatim — [`WorkspaceError::DuplicateId`]
/// naming both claimants, [`WorkspaceError::Header`] naming an
/// unreadable sibling, [`WorkspaceError::Io`] naming the directory.
/// The catalogue never partially succeeds: a directory that is not a
/// healthy store cannot answer "which parts are here" honestly.
pub fn catalogue(
    resolver: &DirResolver,
    open: DocumentId,
) -> Result<Vec<PartEntry>, WorkspaceError> {
    let workspace = resolver.workspace()?;
    let mut entries: Vec<PartEntry> = workspace
        .documents()
        .iter()
        .map(|(&id, path)| PartEntry {
            id,
            path: path.clone(),
            // The rule's one home, so the entry the chooser disables
            // and the id the op refuses are decided by one predicate.
            open_document: Refusal::self_instance(open, id).is_some(),
        })
        .collect();
    entries.sort_by(|a, b| a.file_name().cmp(&b.file_name()).then(a.id.cmp(&b.id)));
    Ok(entries)
}

/// The `Add part…` chooser's held state: the catalogue as of its
/// scan, and the directory it was taken in.
///
/// **One scan of the document's directory**, as the session hands it
/// out ([`crate::session::DocSession::part_census`]).
///
/// The two halves are about ONE moment and travel as one value: the
/// directory that was read, and what reading it answered. A chooser
/// holding one and a listing from the other would show a path that did
/// not produce the entries under it, which is why
/// [`PartCensus::taken`] is the only way to build one.
///
/// The session mints it so that this module names no driver;
/// `crates/viewer/README.md`'s *What a vocabulary reads, it is handed*
/// carries the argument for hoisting rather than widening the rule.
#[derive(Debug)]
pub struct PartCensus {
    dir: Option<PathBuf>,
    offered: Result<Vec<PartEntry>, Refusal>,
}

impl PartCensus {
    /// One scan, as taken: the directory and its answer, minted
    /// together so the pair cannot be assembled from two moments.
    #[must_use]
    pub fn taken(dir: Option<PathBuf>, offered: Result<Vec<PartEntry>, Refusal>) -> Self {
        Self { dir, offered }
    }
}

/// Layer-3 state and nothing else — it never enters the document,
/// never enters the history, and dies with the chooser (G1's
/// transient-state rule, the mate tool's posture one size down).
#[derive(Debug)]
pub struct PartChooser {
    /// The scan this chooser was opened over. Held as the value it
    /// arrived as rather than unpacked into two fields, which would be
    /// a second copy of [`PartCensus`] thirty lines from the first.
    census: PartCensus,
}

impl PartChooser {
    /// Open a chooser over a census the session has already taken.
    #[must_use]
    pub fn opened(census: PartCensus) -> Self {
        Self { census }
    }

    /// Replace the scan, in place: the answer to a directory that
    /// changed while the chooser was open.
    pub fn rescan(&mut self, census: PartCensus) {
        self.census = census;
    }

    /// The directory the entries came from, for the chooser's header.
    /// `None` for a session with no backing file — the case
    /// [`Self::offered`] refuses.
    pub fn dir(&self) -> Option<&Path> {
        self.census.dir.as_deref()
    }

    /// The scan's answer: the parts on offer, or the refusal to show
    /// instead of a list.
    pub fn offered(&self) -> Result<&[PartEntry], &Refusal> {
        match &self.census.offered {
            Ok(entries) => Ok(entries.as_slice()),
            Err(refusal) => Err(refusal),
        }
    }
}
