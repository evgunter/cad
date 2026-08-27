//! Undo as a TREE, walked linearly.
//!
//! # The invariant this module exists for
//!
//! **Nothing is ever destroyed.** An edit made after an undo mints a
//! SIBLING of the state redo would have reached; the abandoned branch
//! stays in the arena, reachable by id, with its own edits intact. The
//! chrome above exposes only undo/redo along the current branch, which
//! is the degenerate walk of that tree — the branch picker is future
//! work, and this shape is what makes it additive rather than a
//! rewrite.
//!
//! # What "the current branch" means
//!
//! Every entry remembers which child redo reaches
//! ([`Entry::active_child`]). Undo sets the parent's active child to
//! the entry it left, so undo-then-redo returns where it was; a commit
//! makes the new entry active, so undo-edit-undo-redo returns to the
//! NEW work rather than to the branch the edit walked away from. Both
//! branches remain.
//!
//! # Documents are values, so the tree is free
//!
//! Each entry retains the `Doc` the edit produced and the `DocEdit`
//! that produced it. `apply` is pure and never mutates its input, so
//! keeping the old value costs a clone of a document, not a
//! reconstruction — which is why the tree is parent pointers over
//! states a linear stack already had to hold.

use pncad::document::{Doc, DocEdit, EditError, ProfileProgram, apply};
use pncad::geom_core::Tol;

/// An entry's identity in the history arena.
///
/// An index, deliberately opaque: entries are never removed, so an id
/// stays valid for the session's life.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct HistoryId(usize);

impl HistoryId {
    /// The arena position, for a caller rendering the tree.
    pub fn index(self) -> usize {
        self.0
    }
}

/// One document state, and how it was reached.
#[derive(Debug)]
pub struct Entry {
    doc: Doc<ProfileProgram>,
    parent: Option<HistoryId>,
    edit: Option<DocEdit<ProfileProgram>>,
    children: Vec<HistoryId>,
    active_child: Option<HistoryId>,
}

impl Entry {
    /// The document this state holds.
    pub fn doc(&self) -> &Doc<ProfileProgram> {
        &self.doc
    }

    /// The state this one was edited from; `None` for the root.
    pub fn parent(&self) -> Option<HistoryId> {
        self.parent
    }

    /// The edit that produced this state from its parent; `None` for
    /// the root.
    pub fn edit(&self) -> Option<&DocEdit<ProfileProgram>> {
        self.edit.as_ref()
    }

    /// Every state edited from this one, in minting order.
    pub fn children(&self) -> &[HistoryId] {
        &self.children
    }

    /// The child redo reaches from here.
    pub fn active_child(&self) -> Option<HistoryId> {
        self.active_child
    }
}

/// The document's edit history: an arena of states plus a cursor.
#[derive(Debug)]
pub struct History {
    entries: Vec<Entry>,
    current: HistoryId,
}

/// A history could not be seeded from a saved log.
#[derive(Debug)]
pub enum ReplayError {
    /// The log's `index`-th edit was refused by `apply`.
    Refused {
        /// Which entry of the log.
        index: usize,
        /// The typed refusal.
        error: EditError,
    },
}

impl History {
    /// A history holding one state and no edits.
    pub fn new(doc: Doc<ProfileProgram>) -> Self {
        Self {
            entries: vec![Entry {
                doc,
                parent: None,
                edit: None,
                children: Vec::new(),
                active_child: None,
            }],
            current: HistoryId(0),
        }
    }

    /// Seed a history from a saved snapshot and its edit log: the
    /// snapshot is the root and each logged edit is a commit, so the
    /// current path IS the file's log and a save with no further edits
    /// writes the same bytes back.
    ///
    /// # Errors
    ///
    /// [`ReplayError::Refused`] naming the log position `apply`
    /// refused. `save` verifies the same replay before writing, so a
    /// file that reaches here replays — the arm exists because this
    /// module will not assume another crate's postcondition.
    pub fn replayed(
        snapshot: Doc<ProfileProgram>,
        edits: &[DocEdit<ProfileProgram>],
        tol: Tol,
    ) -> Result<Self, ReplayError> {
        let mut history = Self::new(snapshot);
        for (index, edit) in edits.iter().enumerate() {
            let applied = apply(history.doc(), edit, tol)
                .map_err(|error| ReplayError::Refused { index, error })?;
            history.commit(edit.clone(), applied.doc);
        }
        Ok(history)
    }

    /// The state the cursor is on.
    pub fn current(&self) -> HistoryId {
        self.current
    }

    /// The document the cursor is on.
    pub fn doc(&self) -> &Doc<ProfileProgram> {
        &self.entry(self.current).doc
    }

    /// The root state — the snapshot a save writes.
    pub fn root(&self) -> HistoryId {
        HistoryId(0)
    }

    /// One entry, by id.
    ///
    /// Ids are arena positions and entries are append-only, so every
    /// id this history minted stays in range for its life; an id from
    /// a different history is a programming error, and indexing says
    /// so at the site rather than answering `None` for a state that
    /// exists somewhere else.
    pub fn entry(&self, id: HistoryId) -> &Entry {
        &self.entries[id.0]
    }

    /// How many states the history retains — the count that must NOT
    /// go down when a sibling is minted.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Whether the history holds only its root.
    pub fn is_empty(&self) -> bool {
        self.entries.len() <= 1
    }

    /// Record an applied edit as a new state, and move the cursor onto
    /// it.
    ///
    /// When the cursor already has children this MINTS A SIBLING: the
    /// existing children keep their subtrees, and only the parent's
    /// active child moves. The caller supplies the document `apply`
    /// produced, so this function never re-runs an edit.
    pub fn commit(&mut self, edit: DocEdit<ProfileProgram>, doc: Doc<ProfileProgram>) -> HistoryId {
        let id = HistoryId(self.entries.len());
        let parent = self.current;
        self.entries.push(Entry {
            doc,
            parent: Some(parent),
            edit: Some(edit),
            children: Vec::new(),
            active_child: None,
        });
        let parent_entry = &mut self.entries[parent.0];
        parent_entry.children.push(id);
        parent_entry.active_child = Some(id);
        self.current = id;
        id
    }

    /// Whether a parent exists to undo to.
    pub fn can_undo(&self) -> bool {
        self.entry(self.current).parent.is_some()
    }

    /// Whether the current state has a branch to redo along.
    pub fn can_redo(&self) -> bool {
        self.entry(self.current).active_child.is_some()
    }

    /// Move the cursor to the parent, remembering the branch left so
    /// redo returns to it. `None` at the root.
    pub fn undo(&mut self) -> Option<HistoryId> {
        let leaving = self.current;
        let parent = self.entry(leaving).parent?;
        self.entries[parent.0].active_child = Some(leaving);
        self.current = parent;
        Some(parent)
    }

    /// Move the cursor along the current branch. `None` at a leaf.
    pub fn redo(&mut self) -> Option<HistoryId> {
        let child = self.entry(self.current).active_child?;
        self.current = child;
        Some(child)
    }

    /// The states from the root to the cursor, root first.
    pub fn path(&self) -> Vec<HistoryId> {
        let mut path = Vec::new();
        let mut at = Some(self.current);
        while let Some(id) = at {
            path.push(id);
            at = self.entry(id).parent;
        }
        path.reverse();
        path
    }

    /// The edits along the current path, root first — the linear log a
    /// save writes beside the root snapshot.
    pub fn path_edits(&self) -> Vec<DocEdit<ProfileProgram>> {
        self.path()
            .into_iter()
            .filter_map(|id| self.entry(id).edit.clone())
            .collect()
    }
}
