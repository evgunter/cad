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
//! # An entry is ONE USER ACTION, which may be several edits
//!
//! [`Entry`] holds the edits its action performed, in the order they
//! were applied, and undo steps over the whole group. A cascading
//! delete is one action and one undo; a slider drag is one action and
//! one undo; nothing here privileges the count.
//!
//! # A history is a session's, and ends at an open
//!
//! A save file holds the document's state and no history (GQ3), so an
//! opened file is a history of one state ([`History::new`]) and undo
//! stops there.
//!
//! # Documents are values, so the tree is free
//!
//! Each entry retains the `Doc` its action produced and the edits
//! that produced it. `apply` is pure and never mutates its input, so
//! keeping the old value costs a clone of a document, not a
//! reconstruction — which is why the tree is parent pointers over
//! states a linear stack already had to hold.
//!
//! Module kind: **vocabulary** — it names no driver type and no
//! `app`-only crate (`crates/viewer/README.md`, Module boundaries).

use pncad::document::{Doc, DocEdit, DocumentId, ProfileProgram};

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
    edits: Vec<DocEdit<ProfileProgram>>,
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

    /// The edits that produced this state from its parent, in applied
    /// order — one for most actions, several for a cascade. EMPTY for
    /// the root, which is the one state no action reached.
    pub fn edits(&self) -> &[DocEdit<ProfileProgram>] {
        &self.edits
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

impl History {
    /// A history holding one state and no edits.
    pub fn new(doc: Doc<ProfileProgram>) -> Self {
        Self {
            entries: vec![Entry {
                doc,
                parent: None,
                edits: Vec::new(),
                children: Vec::new(),
                active_child: None,
            }],
            current: HistoryId(0),
        }
    }

    /// **Every state, re-identified under `id`**, as one operation
    /// and no undo step: the fork (DI4). No state keeps the old
    /// identity, so no undo or redo can return the session to it.
    /// Ids are not in the edits, so the edits stay as they were.
    pub fn under_identity(&mut self, id: DocumentId) {
        for entry in &mut self.entries {
            entry.doc = entry.doc.clone().under_identity(id);
        }
    }

    /// The state the cursor is on.
    pub fn current(&self) -> HistoryId {
        self.current
    }

    /// The document the cursor is on.
    pub fn doc(&self) -> &Doc<ProfileProgram> {
        &self.entry(self.current).doc
    }

    /// The root state: the document the session opened or started
    /// with.
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

    /// Whether the history holds no EDIT — one state, the root, and
    /// nothing to undo.
    ///
    /// The name is clippy's (`len_without_is_empty` pairs it with
    /// [`History::len`]) and it fights the arithmetic, because `len`
    /// counts STATES and a history's minimum is one. Read it as "empty
    /// of edits", which is the question a caller actually asks and the
    /// same thing [`History::can_undo`] answers from the cursor.
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
        self.commit_group(vec![edit], doc)
    }

    /// Record a whole ACTION — the edits it applied, in order, and the
    /// document the last of them produced — as ONE state, and move the
    /// cursor onto it.
    ///
    /// [`History::commit`] is this with a group of one; the branching
    /// rule and the caller's obligation (the document is `apply`'s
    /// output, never re-derived here) are identical for both.
    ///
    /// # Panics
    ///
    /// On an empty group. The root is the one state no edit reached,
    /// and a second such state would give undo a step that changes
    /// nothing.
    pub fn commit_group(
        &mut self,
        edits: Vec<DocEdit<ProfileProgram>>,
        doc: Doc<ProfileProgram>,
    ) -> HistoryId {
        assert!(!edits.is_empty(), "a committed action performs an edit");
        let id = HistoryId(self.entries.len());
        let parent = self.current;
        self.entries.push(Entry {
            doc,
            parent: Some(parent),
            edits,
            children: Vec::new(),
            active_child: None,
        });
        let parent_entry = &mut self.entries[parent.0];
        parent_entry.children.push(id);
        parent_entry.active_child = Some(id);
        self.current = id;
        id
    }

    /// **Add one more edit to the action the cursor's state recorded**:
    /// `edit` joins that state's group, and `doc` — `apply`'s output
    /// for it over the state's document — replaces the state's. The
    /// state stays ONE undo step.
    ///
    /// For an action that finishes after its own commit door has
    /// recorded it (a labelled creation's `SetLabel`), so it is only
    /// sound on the state that door just recorded: `at` names it, and
    /// it must be where the cursor is and have no children yet.
    ///
    /// # Panics
    ///
    /// When `at` is not the cursor's state, is the root, or already
    /// has children — any of which would rewrite an action some other
    /// state was edited from.
    pub fn extend_current(
        &mut self,
        at: HistoryId,
        edit: DocEdit<ProfileProgram>,
        doc: Doc<ProfileProgram>,
    ) {
        assert!(
            at == self.current
                && self.entry(at).parent.is_some()
                && self.entry(at).children.is_empty(),
            "only the leaf state an action just recorded can be extended"
        );
        let entry = &mut self.entries[at.0];
        entry.edits.push(edit);
        entry.doc = doc;
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
}
