//! The N4 per-node name table: bidirectional `StableName ↔ entity`,
//! covering EVERY boundary entity of a node's output bodies, with N2
//! tie marks. Injectivity is enforced at insertion — a would-be
//! duplicate name outside the tie path is the no-silent-aliasing bug
//! N1 rests on, surfaced as a typed error (this crate has no panic
//! paths).

use std::collections::BTreeMap;

use topo::{EdgeKey, FaceKey, VertexKey};

use super::role::{EntityKind, NameRef, StableName, next_epoch};

/// One named entity: which output body of the node, and what in it.
/// (`body` indexes the node's output bodies — 0 for single-body ops; a
/// split's halves by [`crate::names::SplitHalf::output_body`]; a
/// pattern's instances by instance index.)
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
///
/// Both directions hold the SAME [`NameRef`] per row — one shared
/// name, two indexes into it — so a downstream op that wraps an
/// operand's name wraps the very handle this table sealed rather than
/// a copy of it, and [`NameTable::seal_order`]'s stamp reaches every
/// reader of that name.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct NameTable {
    forward: BTreeMap<NameRef, Entry>,
    reverse: BTreeMap<EntityRef, NameRef>,
    sealed: Sealed,
}

/// Whether [`NameTable::seal_order`] has already walked this table.
///
/// Interior mutability because sealing is a CACHE write over a shared
/// table: an operand table is read through `&NameTable` and the stamp
/// it writes changes no answer, only the cost of asking.
#[derive(Debug, Default)]
struct Sealed(core::sync::atomic::AtomicBool);

impl Clone for Sealed {
    fn clone(&self) -> Self {
        Self(core::sync::atomic::AtomicBool::new(
            self.0.load(core::sync::atomic::Ordering::Relaxed),
        ))
    }
}

// A cache flag is not part of the value: two tables with the same rows
// are the same table whether or not either has been sealed.
impl PartialEq for Sealed {
    fn eq(&self, _: &Self) -> bool {
        true
    }
}

impl Eq for Sealed {}

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

    /// **Caches this table's key order onto its names** — the walk
    /// [`NameRef`]'s stamp is for.
    ///
    /// Every row is stamped with its POSITION in this walk, under one
    /// fresh epoch. Two names of one epoch therefore compare by
    /// position, and the answer is the structural one because the walk
    /// enumerated a structurally-ordered map; a name already stamped by
    /// an earlier walk keeps that stamp and compares structurally
    /// against this table's rows.
    ///
    /// Called by [`super::defer::upstream_name`], the one door every
    /// emitter reads an operand's names through, so a table is sealed
    /// when it is first used as an operand and never while it is being
    /// built. A row inserted after a seal is simply unstamped and
    /// compares structurally.
    pub(super) fn seal_order(&self) {
        use core::sync::atomic::Ordering::Relaxed;
        if self.sealed.0.swap(true, Relaxed) {
            return;
        }
        let epoch = next_epoch();
        for (i, (name, _)) in self.forward.iter().enumerate() {
            // `u32` is the position's width and the table's row count
            // is far below it; a table that somehow exceeded it would
            // wrap two rows onto one position, so the seal simply
            // stops instead.
            let Ok(position) = u32::try_from(i) else {
                return;
            };
            name.stamp(epoch, position);
        }
    }

    /// The shared handle naming `ent` — [`NameTable::name_of`]'s hot
    /// twin, for a caller that is about to EMBED the name in a
    /// downstream one and wants the operand table's own handle rather
    /// than a copy.
    pub(super) fn name_ref_of(&self, ent: &EntityRef) -> Option<&NameRef> {
        self.reverse.get(ent)
    }

    /// [`NameTable::lookup`] by handle: the same answer, reached
    /// through [`NameRef`]'s order cache instead of a structural walk.
    pub(super) fn entry_of(&self, name: &NameRef) -> Option<&Entry> {
        self.forward.get(name)
    }

    /// [`NameTable::insert`] by handle — the door that preserves
    /// sharing: the row keeps the handle it was given rather than
    /// re-sharing an equal name under a second one.
    ///
    /// # Errors
    ///
    /// [`NameTable::insert`]'s own.
    pub(super) fn insert_ref(
        &mut self,
        name: NameRef,
        ent: EntityRef,
    ) -> Result<(), DuplicateName> {
        use std::collections::btree_map::Entry as Slot;
        // Each direction is searched ONCE: the vacant slot the
        // collision check lands on is the slot the row is written into.
        if name.kind != ent.key.kind() {
            return Err(DuplicateName {
                name: Box::new((*name).clone()),
            });
        }
        let Slot::Vacant(rev) = self.reverse.entry(ent) else {
            return Err(DuplicateName {
                name: Box::new((*name).clone()),
            });
        };
        match self.forward.entry(name) {
            Slot::Occupied(held) => Err(DuplicateName {
                name: Box::new((**held.key()).clone()),
            }),
            Slot::Vacant(slot) => {
                rev.insert(slot.key().clone());
                slot.insert(Entry::Unique(ent));
                Ok(())
            }
        }
    }

    /// [`NameTable::insert_tied`] by handle.
    ///
    /// # Errors
    ///
    /// [`NameTable::insert_tied`]'s own.
    pub(super) fn insert_tied_ref(
        &mut self,
        name: NameRef,
        mut ents: Vec<EntityRef>,
    ) -> Result<(), DuplicateName> {
        let dup = || DuplicateName {
            name: Box::new((*name).clone()),
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
        self.insert_ref(NameRef::new(name), ent)
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
        ents: Vec<EntityRef>,
    ) -> Result<(), DuplicateName> {
        self.insert_tied_ref(NameRef::new(name), ents)
    }

    /// Resolves a name (PR 4 builds the typed failure ladder on this).
    pub fn lookup(&self, name: &StableName) -> Option<&Entry> {
        self.forward.get(name)
    }

    /// The name of an entity (hit-testing reads this direction).
    pub fn name_of(&self, ent: &EntityRef) -> Option<&StableName> {
        self.reverse.get(ent).map(|n| &**n)
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
        self.forward.iter().map(|(n, e)| (&**n, e))
    }

    /// **The table of ONE output body, as a single-body table**: the
    /// rows whose [`EntityRef::body`] is `body`, each re-keyed to body
    /// 0 — a `Body` value is body 0 to every reader — with every name
    /// VERBATIM. Nothing is minted and no segment is added, so a name
    /// that resolved in the whole table resolves here to the same
    /// entity, and a name of another body finds no row at all rather
    /// than a congruent one.
    ///
    /// A tie keeps the candidates in the selected body and is dropped
    /// when none is; the survivors narrow exactly as an op's do — one
    /// survivor is `Unique`, several stay `Tied` — through the ONE
    /// narrowing rule the emitter's flush also writes by
    /// (`defer::narrow_into`). A tie CAN straddle two output bodies:
    /// a pass-through entity keeps its upstream name with no side
    /// tag, so a split that separates two tied candidates without
    /// cutting either holds one in each half. The projection is what
    /// separates them, and the split's own table stays `Tied`.
    ///
    /// # Errors
    ///
    /// [`DuplicateName`], the insert doors' own. Not reachable by
    /// construction — the source rows are distinct names over
    /// distinct entities within one body, and re-keying one body's
    /// rows keeps both distinct — but carried as the typed result
    /// rather than unwrapped: this crate has no panic paths.
    pub fn project(&self, body: u32) -> Result<NameTable, DuplicateName> {
        let rekey = |e: &EntityRef| EntityRef {
            body: 0,
            key: e.key,
        };
        let mut out = NameTable::new();
        for (name, entry) in &self.forward {
            match entry {
                Entry::Unique(e) => {
                    if e.body == body {
                        out.insert_ref(name.clone(), rekey(e))?;
                    }
                }
                Entry::Tied(es) => {
                    let kept: Vec<EntityRef> =
                        es.iter().filter(|e| e.body == body).map(rekey).collect();
                    if !kept.is_empty() {
                        super::defer::narrow_into(&mut out, name.clone(), kept)?;
                    }
                }
            }
        }
        Ok(out)
    }
}
