//! The N4 per-node name table: bidirectional `StableName ↔ entity`,
//! covering EVERY boundary entity of a node's output bodies, with N2
//! tie marks. Injectivity is enforced at insertion — a would-be
//! duplicate name outside the tie path is the no-silent-aliasing bug
//! N1 rests on, surfaced as a typed error (this crate has no panic
//! paths).

use std::collections::BTreeMap;

use topo::{EdgeKey, FaceKey, VertexKey};

use super::role::{EntityKind, StableName};

/// One named entity: which output body of the node, and what in it.
/// (`body` indexes the node's output bodies — 0 for single-body ops;
/// split: 0 = above, 1 = below; pattern: the instance index.)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct EntityRef {
    /// Output-body index within the node's value.
    pub body: u32,
    /// The entity within that body.
    pub key: EntityKey,
}

/// An entity key within one body (arena keys are body-lineage-scoped
/// — meaningful only against the evaluation that built this table,
/// N4; they never leave editor-core, G1).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum EntityKey {
    /// The whole body.
    Body,
    /// A face.
    Face(FaceKey),
    /// An edge.
    Edge(EdgeKey),
    /// A vertex.
    Vertex(VertexKey),
}

impl EntityKey {
    /// The entity kind this key denotes.
    pub fn kind(self) -> EntityKind {
        match self {
            Self::Body => EntityKind::Body,
            Self::Face(_) => EntityKind::Face,
            Self::Edge(_) => EntityKind::Edge,
            Self::Vertex(_) => EntityKind::Vertex,
        }
    }
}

/// A forward entry: unique, or the N2 tie (≥ 2 equally-admissible
/// candidates; naming succeeds, REFERENCING is PR 4's
/// `Ambiguous{candidates}`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Entry {
    /// Exactly one entity answers to this name.
    Unique(EntityRef),
    /// The recorded tie: all candidates, in deterministic
    /// (EntityRef) order.
    Tied(Vec<EntityRef>),
}

/// The per-node name table (N4). Part of the node's value: memo reuse
/// transfers it with the geometry (the content key is the proof).
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct NameTable {
    forward: BTreeMap<StableName, Entry>,
    reverse: BTreeMap<EntityRef, StableName>,
}

/// A duplicate-name insertion outside the tie path (the
/// no-silent-aliasing bug, typed).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DuplicateName {
    /// The colliding name.
    pub name: Box<StableName>,
}

impl NameTable {
    /// An empty table (nodes without output bodies).
    pub fn new() -> Self {
        Self::default()
    }

    /// Inserts a unique name ↔ entity row. Kind agreement between the
    /// name and the key is the caller's contract, checked here.
    ///
    /// # Errors
    ///
    /// [`DuplicateName`] if the name is already present (aliasing), or
    /// (same type, same loudness) if the ENTITY is already named or
    /// the kinds disagree — all three are emission bugs, never valid
    /// states.
    pub fn insert(&mut self, name: StableName, ent: EntityRef) -> Result<(), DuplicateName> {
        let dup = || DuplicateName {
            name: Box::new(name.clone()),
        };
        if name.kind != ent.key.kind()
            || self.forward.contains_key(&name)
            || self.reverse.contains_key(&ent)
        {
            return Err(dup());
        }
        self.reverse.insert(ent, name.clone());
        self.forward.insert(name, Entry::Unique(ent));
        Ok(())
    }

    /// Records an N2 tie: one name, ≥ 2 candidates (deduplicated,
    /// sorted). Every candidate gets the name in the reverse map.
    ///
    /// # Errors
    ///
    /// [`DuplicateName`] under the same collisions as
    /// [`NameTable::insert`], or if fewer than 2 candidates remain.
    pub fn insert_tied(
        &mut self,
        name: StableName,
        mut ents: Vec<EntityRef>,
    ) -> Result<(), DuplicateName> {
        let dup = || DuplicateName {
            name: Box::new(name.clone()),
        };
        ents.sort_unstable();
        ents.dedup();
        if ents.len() < 2 || self.forward.contains_key(&name) {
            return Err(dup());
        }
        for e in &ents {
            if name.kind != e.key.kind() || self.reverse.contains_key(e) {
                return Err(dup());
            }
        }
        for e in &ents {
            self.reverse.insert(*e, name.clone());
        }
        self.forward.insert(name, Entry::Tied(ents));
        Ok(())
    }

    /// Resolves a name (PR 4 builds the typed failure ladder on this).
    pub fn lookup(&self, name: &StableName) -> Option<&Entry> {
        self.forward.get(name)
    }

    /// The name of an entity (hit-testing reads this direction).
    pub fn name_of(&self, ent: &EntityRef) -> Option<&StableName> {
        self.reverse.get(ent)
    }

    /// Whether `name` is tie-marked.
    pub fn is_tied(&self, name: &StableName) -> bool {
        matches!(self.forward.get(name), Some(Entry::Tied(_)))
    }

    /// Number of names (ties count once).
    pub fn len(&self) -> usize {
        self.forward.len()
    }

    /// True iff no names.
    pub fn is_empty(&self) -> bool {
        self.forward.is_empty()
    }

    /// Iterates rows in name order (deterministic).
    pub fn iter(&self) -> impl Iterator<Item = (&StableName, &Entry)> {
        self.forward.iter()
    }
}
