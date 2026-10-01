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

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use pncad::document::DocumentId;
use pncad::workspace::Workspace;

use crate::docio::DirResolver;
use crate::frame::Tone;
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
        file_name(&self.path)
    }
}

/// A path's file name, as the chrome names a document — the whole path
/// when it names no file.
fn file_name(path: &Path) -> String {
    path.file_name().map_or_else(
        || path.display().to_string(),
        |n| n.to_string_lossy().into_owned(),
    )
}

/// The documents one scan of the session's store offers as parts, with
/// the open document's own entry marked.
///
/// **Ordered by FILE NAME**, with the id as the tie-break. The
/// workspace answers a `BTreeMap<DocumentId, _>`, so its own iteration
/// order is by identity — and an id is a 32-digit hash, which sorts a
/// chooser into an order no reader can predict or scan. The name is
/// what a person picks by, so it is what the listing is sorted on; the
/// id tie-break keeps two same-named files (different directories
/// cannot arise here, but a rename race can) in a deterministic order.
///
/// It takes the store already scanned, so the scan's refusal — a
/// duplicate id, an unreadable sibling, an unreadable directory — is
/// the caller's to say, and the catalogue never partially succeeds: a
/// directory that is not a healthy store answers no listing at all.
pub fn catalogue(workspace: &Workspace, open: DocumentId) -> Vec<PartEntry> {
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
    entries
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

    /// **How loud the chooser's answer is drawn** — read off the scan,
    /// so the arms of [`Self::offered`] are not each given a voice at
    /// the call site.
    ///
    /// A listing with entries in it is [`Tone::Advisory`]: a report of
    /// what is on offer. Every other answer is [`Tone::Actionable`],
    /// because the chooser offers nothing until the reader acts:
    ///
    /// - a refusal — no file to list beside (save the document), or
    ///   the directory's own fault (repair it) — and then a rescan;
    /// - an EMPTY listing, which is worse news than it looks. A saved
    ///   session's own file is in its own directory, so a clean scan
    ///   that finds nothing means that file has gone from under the
    ///   session, and the instances already placed will stop
    ///   resolving.
    #[must_use]
    pub fn tone(&self) -> Tone {
        match &self.census.offered {
            Ok(entries) if !entries.is_empty() => Tone::Advisory,
            Ok(_) | Err(_) => Tone::Actionable,
        }
    }
}

/// **Which file each document in the session's directory is**, by file
/// name: how the feature tree names a part, where the document layer
/// can only name it by its id.
///
/// A snapshot of one scan, like [`PartCensus`], and for the same
/// reason: the tree is drawn every frame and a scan reads every file's
/// header. The session takes it when a run lands, the moment its
/// resolver has just read the same directory, so a file renamed or
/// removed on disk is named as it was until the next landing.
///
/// It says which of three things it knows, and a surface names a part
/// by that, never by its id ([`PartFiles::name`]): no scan yet, a scan
/// that refused, or a scan that found no file for this id.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub enum PartFiles {
    /// No scan has been taken: the session has landed no run of a
    /// document that instantiates a part.
    #[default]
    Unscanned,
    /// The directory could not be scanned. The refusal is not lost: the
    /// same scan refuses every resolution through that directory, typed,
    /// on the instance rows it reaches.
    Refused,
    /// One scan's file names, by document id. A session with no
    /// directory has scanned nothing, so it names no file.
    Scanned(BTreeMap<DocumentId, String>),
}

impl PartFiles {
    /// What a surface names a part before any scan.
    pub const UNSCANNED: &str = "part files not read yet";
    /// What a surface names a part when the scan refused.
    pub const REFUSED: &str = "part files unreadable";
    /// What a surface names a part the scan found no file for.
    pub const NO_FILE: &str = "no file for this part";

    /// The file names one scan of `workspace` found.
    #[must_use]
    pub fn of(workspace: &Workspace) -> Self {
        Self::Scanned(
            workspace
                .documents()
                .iter()
                .map(|(&id, path)| (id, file_name(path)))
                .collect(),
        )
    }

    /// One scan of `resolver`'s directory; with no directory, a scan
    /// that found nothing.
    #[must_use]
    pub fn scanned(resolver: Option<&DirResolver>) -> Self {
        match resolver.map(DirResolver::workspace) {
            None => Self::Scanned(BTreeMap::new()),
            Some(Ok(workspace)) => Self::of(&workspace),
            Some(Err(_)) => Self::Refused,
        }
    }

    /// The file `id` lives in, by name, or which of the three things
    /// this value knows instead.
    #[must_use]
    pub fn name(&self, id: DocumentId) -> &str {
        match self {
            Self::Unscanned => Self::UNSCANNED,
            Self::Refused => Self::REFUSED,
            Self::Scanned(files) => files.get(&id).map_or(Self::NO_FILE, String::as_str),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::PartFiles;
    use crate::docio::DirResolver;
    use pncad::document::DocumentId;

    /// **What a part is named by says which of three things is known**:
    /// no scan yet, a scan that refused, or a scan with no file for the
    /// id — never one of them in the words of another.
    #[test]
    fn a_part_with_no_name_says_why() {
        let id = DocumentId::derive("parts-three-states");
        let missing = std::env::temp_dir().join("parts-three-states-no-such-directory");
        assert!(!missing.exists(), "the fixture path names no directory");
        let refused = PartFiles::scanned(Some(&DirResolver::new(missing)));
        assert_eq!(
            refused,
            PartFiles::Refused,
            "a directory that cannot be read"
        );
        let none = PartFiles::scanned(None);
        let named = [
            PartFiles::Unscanned.name(id),
            refused.name(id),
            none.name(id),
        ];
        assert_eq!(
            named,
            [PartFiles::UNSCANNED, PartFiles::REFUSED, PartFiles::NO_FILE],
            "each state names the part in its own words"
        );
    }
}
