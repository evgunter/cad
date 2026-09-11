//! The N4 per-node name table: bidirectional `NameRef ↔ entity` —
//! one SHARED name per row, indexed both ways — covering EVERY
//! boundary entity of a node's output bodies, with N2 tie marks.
//! Injectivity is enforced at insertion: a would-be duplicate name
//! outside the tie path is the no-silent-aliasing bug N1 rests on,
//! surfaced as a typed error (this crate has no panic paths).
//!
//! Every door comes in two spellings and one body. The public door
//! takes a [`StableName`] and shares it; its `_ref` twin, which the
//! public one delegates to, takes the handle the caller already holds
//! so that a downstream name EMBEDS this table's row rather than a
//! copy of it. An emitter reading an operand wants the `_ref` twin.

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
#[derive(Clone, Default, PartialEq, Eq)]
pub struct NameTable {
    forward: BTreeMap<NameRef, Entry>,
    reverse: BTreeMap<EntityRef, NameRef>,
    sealed: Sealed,
}

// The two maps and nothing else. Whether a table has been sealed is a
// SCHEDULE-dependent bit — which reader reached it first — so it must
// not be printable into a message, a digest or a golden, and this impl
// is what keeps it off every one of them.
impl core::fmt::Debug for NameTable {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("NameTable")
            .field("forward", &self.forward)
            .field("reverse", &self.reverse)
            .finish()
    }
}

/// Whether [`NameTable::seal_order`] has already walked this table.
///
/// Interior mutability because sealing is a CACHE write over a shared
/// table: an operand table is read through `&NameTable` and the stamp
/// it writes changes no answer, only the cost of asking.
#[derive(Debug, Default)]
struct Sealed(core::sync::atomic::AtomicBool);

// A CLONE IS UNSEALED, whatever the original was. The clone shares
// the original's handles, so the rows it starts with keep their
// stamps; but a clone is cloned to be ADDED to, and rows added after
// the copy would be invisible to a flag that already said "walked".
// Starting false means the clone re-seals on its first operand use and
// stamps whatever is then unstamped.
impl Clone for Sealed {
    fn clone(&self) -> Self {
        Self::default()
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
    /// Called by [`super::defer::upstream_name`] — the door every
    /// emitter that reads an operand's names ONE ENTITY AT A TIME goes
    /// through — and by the emitters that instead walk a whole operand
    /// table ([`super::emit_union::member_view`],
    /// [`super::emit::name_pattern`] and its two neighbours), each at
    /// the point it starts that walk. So a table is sealed when it is
    /// first used as an operand and never while it is being built. A
    /// row inserted after a seal is simply unstamped and compares
    /// structurally.
    ///
    /// The flag makes the second call CHEAP, not effective: a second
    /// caller that arrives while the first is mid-walk returns
    /// immediately and reads names that are not stamped yet, so it
    /// compares structurally and gets the same answers a little
    /// slower. Nothing waits and nothing is wrong — which is the whole
    /// licence for writing a cache through `&self`.
    pub(super) fn seal_order(&self) {
        use core::sync::atomic::Ordering::Relaxed;
        if self.sealed.0.swap(true, Relaxed) {
            return;
        }
        // THE INVARIANT THE STAMPED COMPARE RESTS ON, CHECKED WHERE IT
        // IS MINTED. A position only answers for the structural order
        // because this walk IS the structural order; a walk that was
        // not would mint a wrong order silently, and every reader
        // downstream would believe it. The check belongs here and not
        // in `NameRef::cmp`: `[profile.release]` keeps debug
        // assertions ON, so a check in the compare would put the
        // O(depth) walk this unit removed back into every build that
        // has them. Here it is O(n) once per table, and it compares
        // `StableName`s — the structural order itself, never the
        // cached arm that would make the check circular.
        #[cfg(debug_assertions)]
        {
            let mut previous: Option<&StableName> = None;
            for name in self.forward.keys() {
                if let Some(before) = previous {
                    assert!(
                        before < &**name,
                        "a name table's key walk is not its structural order: \
                         {before:?} is not less than {:?}",
                        **name
                    );
                }
                previous = Some(&**name);
            }
        }
        let Some(epoch) = next_epoch() else {
            // Epochs are exhausted and are never reused, so this table
            // stays unstamped and every one of its names compares
            // structurally.
            return;
        };
        for (i, (name, _)) in self.forward.iter().enumerate() {
            // A table with more rows than a `u32` counts cannot be
            // stamped without wrapping two rows onto one position, so
            // the seal stops rather than minting an order that lies.
            let Ok(position) = u32::try_from(i) else {
                return;
            };
            name.stamp(epoch, position);
        }
    }

    /// Rows in key order, as the shared handles — [`NameTable::iter`]'s
    /// twin for an emitter that is about to EMBED each name in a
    /// downstream one.
    pub(super) fn iter_refs(&self) -> impl Iterator<Item = (&NameRef, &Entry)> {
        self.forward.iter()
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
        use std::collections::btree_map::Entry as Slot;
        let dup = || DuplicateName {
            name: Box::new((*name).clone()),
        };
        ents.sort_unstable();
        ents.dedup();
        if ents.len() < 2 {
            return Err(dup());
        }
        for e in &ents {
            if name.kind != e.key.kind() || self.reverse.contains_key(e) {
                return Err(dup());
            }
        }
        // The forward direction is searched once, as `insert_ref`'s is.
        // The reverse direction cannot be: a tie writes SEVERAL rows,
        // and the vacant slots for them cannot be held open at the same
        // time — so those keep the check-then-write pair.
        match self.forward.entry(name) {
            Slot::Occupied(held) => Err(DuplicateName {
                name: Box::new((**held.key()).clone()),
            }),
            Slot::Vacant(slot) => {
                for e in &ents {
                    self.reverse.insert(*e, slot.key().clone());
                }
                slot.insert(Entry::Tied(ents));
                Ok(())
            }
        }
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
    ///
    /// This is the ONE pair that is two bodies rather than one: a bare
    /// name is searched for structurally, a handle
    /// ([`NameTable::entry_of`]) through its order cache, and neither
    /// can be written as the other without minting a handle to throw
    /// away.
    pub fn lookup(&self, name: &StableName) -> Option<&Entry> {
        self.forward.get(name)
    }

    /// The name of an entity (hit-testing reads this direction).
    pub fn name_of(&self, ent: &EntityRef) -> Option<&StableName> {
        self.name_ref_of(ent).map(|n| &**n)
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
        self.iter_refs().map(|(n, e)| (&**n, e))
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

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

    //! **The stamped order IS the structural order** — the invariant
    //! the whole design rests on, and the one a repo-wide
    //! `clippy::mutable_key_type` exception is written against
    //! (`clippy.toml`).
    //!
    //! Every row here compares [`NameRef`]'s answer against
    //! [`StableName`]'s own on real pairs. `seal_order`'s
    //! `debug_assertions` walk covers the MINTING side (a stamped
    //! order that is not the structural one cannot be written); these
    //! cover the READING side, including the four ways a pair can miss
    //! the stamped arm — unstamped, half-stamped, two epochs, and a
    //! bare name probing a sealed map through `Borrow`.

    use super::{EntityKey, EntityRef, NameTable};
    use crate::names::role::{EntityKind, NameRef, RoleSeg, StableName};
    use crate::node::RecipeNodeId;

    /// A name of `depth` descent levels over leaf `leaf`: the chain
    /// shape a boolean step builds, and the shape whose structural
    /// compare walks to the bottom.
    fn chain(depth: usize, leaf: u64) -> StableName {
        let mut n = StableName {
            kind: EntityKind::Body,
            node: RecipeNodeId(leaf),
            path: vec![RoleSeg::OutputBody],
        };
        for _ in 0..depth {
            n = StableName {
                kind: EntityKind::Body,
                node: RecipeNodeId(99),
                path: vec![RoleSeg::FromA(NameRef::new(n))],
            };
        }
        n
    }

    /// A distinct entity per index. `EntityKey::Body` keeps these rows
    /// arena-free: the body index alone separates them, so the table
    /// under test needs no evaluation behind it.
    fn ent(i: u32) -> EntityRef {
        EntityRef {
            body: i,
            key: EntityKey::Body,
        }
    }

    /// Every ordered pair, both arms, against the structural answer.
    fn every_pair_agrees(handles: &[NameRef]) {
        for a in handles {
            for b in handles {
                assert_eq!(
                    a.cmp(b),
                    (**a).cmp(&**b),
                    "the handle's order left the name's order at {:?} vs {:?}",
                    **a,
                    **b
                );
            }
        }
    }

    /// The handles a table holds, in its own key order.
    fn handles(t: &NameTable) -> Vec<NameRef> {
        t.iter_refs().map(|(n, _)| n.clone()).collect()
    }

    fn table_of(names: Vec<StableName>) -> NameTable {
        let mut t = NameTable::new();
        for (i, n) in names.into_iter().enumerate() {
            t.insert(n, ent(u32::try_from(i).expect("a small test table")))
                .expect("distinct names over distinct entities");
        }
        t
    }

    #[test]
    fn a_sealed_tables_stamped_order_is_its_structural_order() {
        let t = table_of((0..12).map(|i| chain(20, i)).collect());
        t.seal_order();
        let rows = handles(&t);
        assert_eq!(rows.len(), 12);
        assert!(
            rows.iter().all(|n| n.stamped_for_tests()),
            "a sealed table stamps every row"
        );
        every_pair_agrees(&rows);
    }

    #[test]
    fn one_handle_in_two_tables_keeps_the_first_seals_positions() {
        // The hazard the stamp-once rule exists for: a name shared by
        // two tables must not carry two tables' positions. Whichever
        // walk claims it, every pair still answers structurally — the
        // others simply fall back.
        let shared: Vec<NameRef> = (0..6).map(|i| NameRef::new(chain(20, i))).collect();
        let mut first = NameTable::new();
        let mut second = NameTable::new();
        for (i, n) in shared.iter().enumerate() {
            let i = u32::try_from(i).expect("a small test table");
            first.insert_ref(n.clone(), ent(i)).expect("fresh row");
            // The second table takes them in the opposite insertion
            // order; a `BTreeMap` sorts either way, so the two walks
            // enumerate the same sequence and only the EPOCH differs.
            let mirrored = shared.len() - 1 - usize::try_from(i).expect("a small index");
            second
                .insert_ref(shared[mirrored].clone(), ent(i))
                .expect("fresh row");
        }
        first.seal_order();
        second.seal_order();
        every_pair_agrees(&shared);
        every_pair_agrees(&handles(&first));
        every_pair_agrees(&handles(&second));
    }

    #[test]
    fn a_row_inserted_after_the_seal_is_unstamped_and_still_ordered() {
        let mut t = table_of((0..6).map(|i| chain(20, i)).collect());
        t.seal_order();
        let late = NameRef::new(chain(20, 100));
        t.insert_ref(late.clone(), ent(6)).expect("a fresh name");
        assert!(
            !late.stamped_for_tests(),
            "a row added after the seal carries no position"
        );
        every_pair_agrees(&handles(&t));
        // And the map still answers for it, by handle and by name.
        assert!(t.entry_of(&late).is_some());
        assert!(t.lookup(&late).is_some());
        assert_eq!(t.name_of(&ent(6)), Some(&*late));
    }

    #[test]
    fn a_bare_name_probes_a_sealed_map_through_borrow() {
        // The `Borrow<StableName>` path: the probe is a value nothing
        // ever stamped, searched through a map whose keys all are.
        let t = table_of((0..8).map(|i| chain(20, i)).collect());
        t.seal_order();
        for i in 0..8 {
            assert!(
                t.lookup(&chain(20, i)).is_some(),
                "a sealed map must answer a bare name it holds"
            );
        }
        assert!(t.lookup(&chain(20, 404)).is_none());
        assert!(t.lookup(&chain(19, 0)).is_none());
    }

    #[test]
    fn two_epochs_fall_back_and_still_agree() {
        // Disjoint names in two tables: every cross pair has two
        // epochs, which is the arm that must NOT read positions.
        let left = table_of((0..8).map(|i| chain(20, i)).collect());
        let right = table_of((8..16).map(|i| chain(20, i)).collect());
        left.seal_order();
        right.seal_order();
        let mut both = handles(&left);
        both.extend(handles(&right));
        every_pair_agrees(&both);
        // Half-stamped pairs too: one side never sealed at all.
        let loose = table_of((16..24).map(|i| chain(20, i)).collect());
        both.extend(handles(&loose));
        every_pair_agrees(&both);
    }

    #[test]
    fn a_deep_chain_answers_the_same_stamped_or_not() {
        // Depth is the thing the stamp exists to stop paying, so the
        // agreement is asserted at a depth where the structural walk
        // is long: the answers are recorded BEFORE sealing and must be
        // identical after.
        let names: Vec<StableName> = (0..10).map(|i| chain(24, i)).collect();
        let t = table_of(names.clone());
        let rows = handles(&t);
        let before: Vec<core::cmp::Ordering> = rows
            .iter()
            .flat_map(|a| rows.iter().map(move |b| a.cmp(b)))
            .collect();
        t.seal_order();
        let after: Vec<core::cmp::Ordering> = rows
            .iter()
            .flat_map(|a| rows.iter().map(move |b| a.cmp(b)))
            .collect();
        assert_eq!(before, after, "sealing changed an answer");
        every_pair_agrees(&rows);
    }

    #[test]
    fn a_clone_reseals_and_stamps_what_it_gained() {
        let t = table_of((0..4).map(|i| chain(20, i)).collect());
        t.seal_order();
        let mut grown = t.clone();
        let late = NameRef::new(chain(20, 50));
        grown
            .insert_ref(late.clone(), ent(4))
            .expect("a fresh name");
        grown.seal_order();
        assert!(
            late.stamped_for_tests(),
            "a clone re-seals, so a row it gained is stamped"
        );
        every_pair_agrees(&handles(&grown));
    }
}
