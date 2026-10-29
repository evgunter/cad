//! The N4 per-node name table: bidirectional `NameRef ↔ entity` —
//! one SHARED name per row, indexed both ways — covering EVERY
//! boundary entity of a node's output bodies, with N2 tie marks.
//! Injectivity is enforced at insertion, both ways, as typed errors
//! (this crate has no panic paths): a would-be duplicate name outside
//! the tie path is the no-silent-aliasing bug N1 rests on
//! ([`DuplicateName`]), and an entity offered a second name, or twice
//! in one list, breaks the reverse direction ([`EntityNamed`]).
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
/// family's members by member index.)
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

    /// **The face this key denotes, or `None`**: "not a face", never "no
    /// such face" — the key is an already resolved entity, so the arms
    /// it does not match are the other three kinds, and
    /// [`EntityKey::kind`] is what says which.
    pub(crate) fn face(self) -> Option<FaceKey> {
        match self {
            Self::Face(k) => Some(k),
            _ => None,
        }
    }

    /// **The edge this key denotes, or `None`** ([`EntityKey::face`]'s
    /// twin).
    pub(crate) fn edge(self) -> Option<EdgeKey> {
        match self {
            Self::Edge(k) => Some(k),
            _ => None,
        }
    }

    /// **The vertex this key denotes, or `None`** ([`EntityKey::face`]'s
    /// twin).
    pub(crate) fn vertex(self) -> Option<VertexKey> {
        match self {
            Self::Vertex(k) => Some(k),
            _ => None,
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

impl Entry {
    /// The entities this entry names: one, or the tie's candidates.
    pub(crate) fn entities(&self) -> &[EntityRef] {
        match self {
            Self::Unique(e) => core::slice::from_ref(e),
            Self::Tied(es) => es,
        }
    }
}

/// **Which of its name's candidates an entity is** (N4, "A tie's
/// candidates keep their identity").
///
/// The node that mints a tie numbers its candidates, and `Of(k)` is
/// candidate `k` of that numbering. The pass-through edges of N1 carry
/// it with the name, so it survives [`NameTable::project`]'s re-keying
/// and the product gather's: it is never recomputed from an
/// [`EntityRef`]. A strict name is its own only candidate.
///
/// It is not part of the name: [`NameTable::lookup`], [`NameTable::iter`]
/// and [`Entry`] never show it, so no name digest reads it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum Candidate {
    /// The one entity of a strict name.
    Only,
    /// Candidate `k` of the tie its name's minter recorded.
    Of(u32),
}

/// A forward row: the entry, and the candidate of each of its
/// entities.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct Row {
    entry: Entry,
    /// `None` for a strict row. Otherwise one number per entity of
    /// `entry`, in `entry`'s order: several for a tie, one for a row a
    /// tie narrowed to.
    tie: Option<Box<[u32]>>,
}

impl Row {
    /// Each entity of the row with its candidate, in the entry's order.
    pub(super) fn pairs(&self) -> impl Iterator<Item = (Candidate, EntityRef)> + '_ {
        let ents = self.entry.entities().iter().copied();
        let ks = self.tie.as_deref();
        ents.enumerate().map(move |(i, e)| {
            let k = ks.and_then(|ks| ks.get(i)).copied();
            (k.map_or(Candidate::Only, Candidate::Of), e)
        })
    }
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
    forward: BTreeMap<NameRef, Row>,
    reverse: BTreeMap<EntityRef, NameRef>,
    sealed: Sealed,
    said: Said,
    lines: Lines,
}

// The rows, and nothing else. Whether a table has been sealed is a
// SCHEDULE-dependent bit — which reader reached it first — so it must
// not be printable into a message, a digest or a golden, and this impl
// is what keeps it off every one of them.
//
// `Self` is destructured exhaustively, so a field added to the
// declaration is an E0027 unbound-pattern error rather than a value
// silently absent from every dump; `sealed` binds to `_`, which is what
// makes the omission a decision a reader can see and the compiler still
// forces. `finish_non_exhaustive` is what that `_` arm stands for —
// `finish` would claim the schedule-dependent bit is shown.
impl core::fmt::Debug for NameTable {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let Self {
            forward,
            reverse,
            sealed: _,
            said: _,
            lines: _,
        } = self;
        f.debug_struct("NameTable")
            .field("forward", forward)
            .field("reverse", reverse)
            .finish_non_exhaustive()
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

/// **The detail each name of a table is said at within it**
/// ([`super::words::table_details`]), worked out on the first sentence
/// that asks and kept for every later one: a tree that redraws a failed
/// row each frame says its names at no further search.
///
/// A cache like [`Sealed`]: a clone starts empty (it is cloned to be
/// added to), a write empties it, and it is no part of the value.
#[derive(Default)]
struct Said(std::sync::OnceLock<BTreeMap<NameRef, super::words::Detail>>);

impl Clone for Said {
    fn clone(&self) -> Self {
        Self::default()
    }
}

impl PartialEq for Said {
    fn eq(&self, _: &Self) -> bool {
        true
    }
}

impl Eq for Said {}

/// **Each line this table's edge rows lie on → those rows**, in key
/// order ([`NameTable::on_line`]), and each edge an edge set's line lists
/// → those lines ([`NameTable::sets_listing`]). A cache like [`Said`]: a
/// clone starts empty, a write empties it, and it is no part of the
/// value.
#[derive(Default)]
struct Lines(std::sync::OnceLock<LineIndex>);

/// What [`Lines`] caches.
struct LineIndex {
    by_line: BTreeMap<NameRef, Vec<NameRef>>,
    in_set: BTreeMap<StableName, Vec<NameRef>>,
}

impl Clone for Lines {
    fn clone(&self) -> Self {
        Self::default()
    }
}

impl PartialEq for Lines {
    fn eq(&self, _: &Self) -> bool {
        true
    }
}

impl Eq for Lines {}

/// A name offered to a second row outside the tie path, or a (name,
/// candidate) pair offered twice: the forward map's injectivity, the
/// no-silent-aliasing bug, typed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DuplicateName {
    /// The colliding name.
    pub name: Box<StableName>,
}

/// A row whose entity is not of its name's kind: the invariant every
/// selection's per-name kind rests on (`Select::fault`), found broken
/// at the door that seats it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KindMismatch {
    /// The name the row was to be seated under.
    pub name: Box<StableName>,
    /// The kind of the entity offered for it, which is not
    /// `name.kind`. The kind and not the key: an [`EntityKey`] never
    /// leaves the evaluation that minted it.
    pub found: EntityKind,
}

/// A tie offered with fewer than two distinct candidates: a row that
/// names no tie, refused at the door that would have seated it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShortTie {
    /// The name the tie was to be seated under.
    pub name: Box<StableName>,
    /// How many distinct entities the tie was offered: 0 or 1.
    pub candidates: usize,
}

/// A row offered for an entity that already holds a name: the reverse
/// map's injectivity, one name per entity, found broken at the door
/// that would have seated it. An entity offered twice in one list — to
/// a minted tie, or under two candidates of a carried one — is this
/// too, holding the name it is offered.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EntityNamed {
    /// The name the row was to be seated under.
    pub name: Box<StableName>,
    /// The name the entity already holds.
    pub held: Box<StableName>,
}

/// Why a seating door refused a row. Each is an emission bug, never a
/// valid state.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SeatingRefusal {
    /// The name is already seated, or a candidate list gives one
    /// (name, candidate) pair to two rows.
    Duplicate(DuplicateName),
    /// The entity already holds a name.
    EntityNamed(EntityNamed),
    /// The entity's kind is not the name's.
    KindMismatch(KindMismatch),
    /// A minted tie of fewer than two distinct entities, or a carried
    /// candidate list of none.
    ShortTie(ShortTie),
}

impl From<DuplicateName> for SeatingRefusal {
    fn from(e: DuplicateName) -> Self {
        Self::Duplicate(e)
    }
}

impl SeatingRefusal {
    fn duplicate(name: &StableName) -> Self {
        Self::Duplicate(DuplicateName {
            name: Box::new(name.clone()),
        })
    }

    pub(super) fn entity_named(name: &StableName, held: &StableName) -> Self {
        Self::EntityNamed(EntityNamed {
            name: Box::new(name.clone()),
            held: Box::new(held.clone()),
        })
    }

    /// [`Self::EntityNamed`] when sorted `ents` repeat an entity.
    fn check_distinct(name: &StableName, ents: &[EntityRef]) -> Result<(), Self> {
        if ents.windows(2).any(|w| w[0] == w[1]) {
            Err(Self::entity_named(name, name))
        } else {
            Ok(())
        }
    }

    fn short_tie(name: &StableName, candidates: usize) -> Self {
        Self::ShortTie(ShortTie {
            name: Box::new(name.clone()),
            candidates,
        })
    }

    /// [`Self::KindMismatch`] when `ent` is not of `name`'s kind.
    fn check_kind(name: &StableName, ent: &EntityRef) -> Result<(), Self> {
        if name.kind == ent.key.kind() {
            Ok(())
        } else {
            Err(Self::KindMismatch(KindMismatch {
                name: Box::new(name.clone()),
                found: ent.key.kind(),
            }))
        }
    }
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
    /// [`super::emit::name_family`] and [`super::emit::name_copy`]), each at
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

    /// **The detail `name` is said at within this table**, `None` where
    /// the table does not hold it ([`super::words::table_details`]).
    pub(crate) fn detail(&self, name: &StableName) -> Option<&super::words::Detail> {
        self.said
            .0
            .get_or_init(|| super::words::table_details(self))
            .get(name)
    }

    /// **The rows that lie on `line`** (N5, "A cited line"): this
    /// table's edge rows whose line ([`super::role::edge_line`]) is
    /// `line`, in key order — a row that is the line itself included.
    /// Empty where `line` is no edge's or no row lies on it.
    pub(crate) fn on_line(&self, line: &StableName) -> &[NameRef] {
        self.line_index()
            .by_line
            .get(line)
            .map_or(&[], Vec::as_slice)
    }

    /// **The edge sets that list `edge`**: the lines of this table's
    /// edge rows that are `Merged` sets with `edge` among their
    /// constituents, in key order. Empty where none does.
    pub(crate) fn sets_listing(&self, edge: &StableName) -> &[NameRef] {
        self.line_index()
            .in_set
            .get(edge)
            .map_or(&[], Vec::as_slice)
    }

    fn line_index(&self) -> &LineIndex {
        self.lines.0.get_or_init(|| {
            let mut by_line: BTreeMap<NameRef, Vec<NameRef>> = BTreeMap::new();
            for row in self.forward.keys() {
                if row.kind == super::role::EntityKind::Edge {
                    by_line
                        .entry(super::role::edge_line(row))
                        .or_default()
                        .push(row.clone());
                }
            }
            let mut in_set: BTreeMap<StableName, Vec<NameRef>> = BTreeMap::new();
            for line in by_line.keys() {
                if let [super::role::RoleSeg::Merged(set)] = line.path.as_slice() {
                    for c in set {
                        in_set.entry(c.clone()).or_default().push(line.clone());
                    }
                }
            }
            LineIndex { by_line, in_set }
        })
    }

    /// Rows in key order, as the shared handles — [`NameTable::iter`]'s
    /// twin for an emitter that is about to EMBED each name in a
    /// downstream one.
    pub(super) fn iter_refs(&self) -> impl Iterator<Item = (&NameRef, &Entry)> {
        self.forward.iter().map(|(n, row)| (n, &row.entry))
    }

    /// Rows in key order with their candidates — the walk a VERBATIM
    /// carry reads ([`NameTable::project`], the product gather's
    /// `defer::CarriedRows`), since it keeps each entity's candidate.
    pub(super) fn rows(&self) -> impl Iterator<Item = (&NameRef, &Row)> {
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
        self.forward.get(name).map(|row| &row.entry)
    }

    /// `ent`'s candidate under `name`, or `None` when `name`'s row does
    /// not name `ent`.
    pub(super) fn candidate_of(&self, name: &NameRef, ent: &EntityRef) -> Option<Candidate> {
        self.forward
            .get(name)?
            .pairs()
            .find_map(|(c, e)| (e == *ent).then_some(c))
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
    ) -> Result<(), SeatingRefusal> {
        use std::collections::btree_map::Entry as Slot;
        self.said = Said::default();
        self.lines = Lines::default();
        SeatingRefusal::check_kind(&name, &ent)?;
        // Each direction is searched ONCE: the vacant slot the
        // collision check lands on is the slot the row is written into.
        let rev = match self.reverse.entry(ent) {
            Slot::Occupied(held) => return Err(SeatingRefusal::entity_named(&name, held.get())),
            Slot::Vacant(rev) => rev,
        };
        match self.forward.entry(name) {
            Slot::Occupied(held) => Err(SeatingRefusal::duplicate(held.key())),
            Slot::Vacant(slot) => {
                rev.insert(slot.key().clone());
                slot.insert(Row {
                    entry: Entry::Unique(ent),
                    tie: None,
                });
                Ok(())
            }
        }
    }

    /// [`NameTable::insert_tied`] by handle: the MINTING door, which
    /// numbers the candidates in the order it stores them.
    ///
    /// # Errors
    ///
    /// [`NameTable::insert_tied`]'s own.
    pub(super) fn insert_tied_ref(
        &mut self,
        name: NameRef,
        mut ents: Vec<EntityRef>,
    ) -> Result<(), SeatingRefusal> {
        for e in &ents {
            SeatingRefusal::check_kind(&name, e)?;
        }
        ents.sort_unstable();
        SeatingRefusal::check_distinct(&name, &ents)?;
        if ents.len() < 2 {
            return Err(SeatingRefusal::short_tie(&name, ents.len()));
        }
        // Each candidate is a distinct live entity, so a tie of more
        // than `u32::MAX` of them needs that many entities in memory at
        // once, which no evaluation can hold.
        let ks = (0..ents.len())
            .map(|k| {
                u32::try_from(k)
                    .unwrap_or_else(|_| unreachable!("a tie of more than u32::MAX live candidates"))
            })
            .collect();
        self.write_candidates(name, ents, ks)
    }

    /// **The CARRYING door**: a row whose entities keep the candidates
    /// an upstream tie's minter gave them (N4, "A tie's candidates keep
    /// their identity"). One pair writes a `Unique` row — strict for
    /// [`Candidate::Only`], keeping its candidate for [`Candidate::Of`]
    /// — and several write the tie over exactly those candidates.
    ///
    /// # Errors
    ///
    /// [`SeatingRefusal`] under [`NameTable::insert`]'s collisions;
    /// [`SeatingRefusal::Duplicate`] for a list that repeats a
    /// candidate or holds a strict name's [`Candidate::Only`] beside
    /// anything, as a (name, candidate) pair names one entity;
    /// [`SeatingRefusal::EntityNamed`] for one that offers an entity
    /// under two candidates, as [`NameTable::insert_tied`] refuses an
    /// entity offered twice; and [`SeatingRefusal::ShortTie`] for an
    /// empty list.
    pub(super) fn insert_candidates_ref(
        &mut self,
        name: NameRef,
        mut pairs: Vec<(Candidate, EntityRef)>,
    ) -> Result<(), SeatingRefusal> {
        for (_, e) in &pairs {
            SeatingRefusal::check_kind(&name, e)?;
        }
        let dup = |name: &NameRef| SeatingRefusal::duplicate(name);
        pairs.sort_unstable_by_key(|&(_, e)| e);
        let mut ks = Vec::with_capacity(pairs.len());
        let mut ents = Vec::with_capacity(pairs.len());
        for (c, e) in pairs {
            match (c, ents.as_slice()) {
                (Candidate::Only, []) => ents.push(e),
                (Candidate::Of(k), _) if ks.len() == ents.len() => {
                    ks.push(k);
                    ents.push(e);
                }
                _ => return Err(dup(&name)),
            }
        }
        match ents.as_slice() {
            [] => Err(SeatingRefusal::short_tie(&name, 0)),
            [one] if ks.is_empty() => self.insert_ref(name, *one),
            _ => {
                let mut seen = ks.clone();
                seen.sort_unstable();
                seen.dedup();
                if seen.len() != ks.len() {
                    return Err(dup(&name));
                }
                SeatingRefusal::check_distinct(&name, &ents)?;
                self.write_candidates(name, ents, ks.into_boxed_slice())
            }
        }
    }

    /// Writes a candidate-numbered row: `ents` sorted and distinct,
    /// `ks` one number per entity. One entity is a `Unique` row that
    /// keeps its candidate; several are the tie.
    fn write_candidates(
        &mut self,
        name: NameRef,
        ents: Vec<EntityRef>,
        ks: Box<[u32]>,
    ) -> Result<(), SeatingRefusal> {
        use std::collections::btree_map::Entry as Slot;
        self.said = Said::default();
        self.lines = Lines::default();
        debug_assert_eq!(ents.len(), ks.len(), "one candidate per entity");
        for e in &ents {
            SeatingRefusal::check_kind(&name, e)?;
        }
        if let Some(held) = ents.iter().find_map(|e| self.reverse.get(e)) {
            return Err(SeatingRefusal::entity_named(&name, held));
        }
        // The forward direction is searched once, as `insert_ref`'s is.
        // The reverse direction cannot be: a tie writes SEVERAL rows,
        // and the vacant slots for them cannot be held open at the same
        // time — so those keep the check-then-write pair.
        match self.forward.entry(name) {
            Slot::Occupied(held) => Err(SeatingRefusal::duplicate(held.key())),
            Slot::Vacant(slot) => {
                for e in &ents {
                    self.reverse.insert(*e, slot.key().clone());
                }
                let entry = match ents.as_slice() {
                    [one] => Entry::Unique(*one),
                    _ => Entry::Tied(ents),
                };
                slot.insert(Row {
                    entry,
                    tie: Some(ks),
                });
                Ok(())
            }
        }
    }

    /// Inserts a unique name ↔ entity row. Kind agreement between the
    /// name and the key is the caller's contract, checked here.
    ///
    /// # Errors
    ///
    /// [`SeatingRefusal::Duplicate`] if the name is already present
    /// (aliasing), [`SeatingRefusal::EntityNamed`] if the entity is,
    /// and [`SeatingRefusal::KindMismatch`] if the kinds disagree.
    pub fn insert(&mut self, name: StableName, ent: EntityRef) -> Result<(), SeatingRefusal> {
        self.insert_ref(NameRef::new(name), ent)
    }

    /// Records an N2 tie: one name, ≥ 2 distinct candidates, stored
    /// sorted. Every candidate gets the name in the reverse map.
    ///
    /// # Errors
    ///
    /// [`SeatingRefusal`] under the same collisions as
    /// [`NameTable::insert`], [`SeatingRefusal::EntityNamed`] if the
    /// list repeats an entity, or [`SeatingRefusal::ShortTie`] if it
    /// holds fewer than 2.
    pub fn insert_tied(
        &mut self,
        name: StableName,
        ents: Vec<EntityRef>,
    ) -> Result<(), SeatingRefusal> {
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
        self.forward.get(name).map(|row| &row.entry)
    }

    /// The name of an entity (hit-testing reads this direction).
    pub fn name_of(&self, ent: &EntityRef) -> Option<&StableName> {
        self.name_ref_of(ent).map(|n| &**n)
    }

    /// Whether `name` is tie-marked.
    pub fn is_tied(&self, name: &StableName) -> bool {
        matches!(self.lookup(name), Some(Entry::Tied(_)))
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
    /// when none is; the survivors narrow through the ONE narrowing
    /// rule (`defer::narrow_into`) — one survivor is `Unique`, several
    /// stay `Tied` — and each keeps its [`Candidate`]. A tie CAN
    /// straddle two output bodies: a pass-through entity keeps its
    /// upstream name with no side tag, so a split that separates two
    /// tied candidates without cutting either holds one in each half.
    /// The projection is what separates them, and the split's own
    /// table stays `Tied`.
    ///
    /// # Errors
    ///
    /// [`SeatingRefusal`], the insert doors' own. Not reachable by
    /// construction — the source rows are distinct names over
    /// distinct entities within one body, and re-keying one body's
    /// rows keeps both distinct — but carried as the typed result
    /// rather than unwrapped: this crate has no panic paths.
    pub fn project(&self, body: u32) -> Result<NameTable, SeatingRefusal> {
        self.reindex(|b| (b == body).then_some(0))
    }

    /// The rows over the output bodies `onto` keeps, each re-indexed to
    /// the body it answers: [`Self::project`] over several bodies at
    /// once, so one copy that is a family keeps its members' rows.
    ///
    /// # Errors
    ///
    /// [`Self::project`]'s.
    pub fn reindex(&self, onto: impl Fn(u32) -> Option<u32>) -> Result<NameTable, SeatingRefusal> {
        let mut out = NameTable::new();
        for (name, row) in &self.forward {
            let kept: Vec<(Candidate, EntityRef)> = row
                .pairs()
                .filter_map(|(c, e)| onto(e.body).map(|body| (c, EntityRef { body, key: e.key })))
                .collect();
            if !kept.is_empty() {
                super::defer::narrow_into(&mut out, name.clone(), kept)?;
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
            node: RecipeNodeId::new(0, leaf),
            path: vec![RoleSeg::OutputBody],
        };
        for _ in 0..depth {
            n = StableName {
                kind: EntityKind::Body,
                node: RecipeNodeId::new(0, 99),
                path: vec![RoleSeg::From {
                    read: crate::names::FOLD_A,
                    of: NameRef::new(n),
                }],
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

#[cfg(test)]
mod carrying_door {
    //! **The carrying door's own invariant**: a (name, candidate) pair
    //! names one entity, whichever writer hands the list in. The
    //! product gather refuses a repeat before it reaches here (and names
    //! the root that carried it); this door is what holds the invariant
    //! for every other writer — a split's flush, `project`.
    #![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

    use super::{
        Candidate, EntityKey, EntityNamed, EntityRef, Entry, KindMismatch, NameTable,
        SeatingRefusal, ShortTie,
    };
    use crate::names::role::{EntityKind, NameRef, RoleSeg, StableName};
    use crate::node::RecipeNodeId;
    use Candidate::{Of, Only};

    fn name() -> NameRef {
        NameRef::new(StableName {
            kind: EntityKind::Face,
            node: RecipeNodeId::new(0, 3),
            path: vec![RoleSeg::OutputBody],
        })
    }

    fn face(i: u64) -> EntityRef {
        EntityRef {
            body: 0,
            key: EntityKey::Face(topo::FaceKey::from(slotmap::KeyData::from_ffi(i))),
        }
    }

    fn write(pairs: Vec<(Candidate, EntityRef)>) -> Option<NameTable> {
        let mut t = NameTable::new();
        t.insert_candidates_ref(name(), pairs).ok().map(|()| t)
    }

    #[test]
    fn the_carrying_door_refuses_what_would_name_one_pair_twice() {
        let refused = [
            ("no candidate at all", vec![]),
            (
                "one candidate twice",
                vec![(Of(1), face(1)), (Of(1), face(2))],
            ),
            ("one entity twice", vec![(Of(0), face(1)), (Of(1), face(1))]),
            (
                "a strict name twice",
                vec![(Only, face(1)), (Only, face(2))],
            ),
            (
                "strict, then a candidate",
                vec![(Only, face(1)), (Of(0), face(2))],
            ),
            (
                "a candidate, then strict",
                vec![(Of(0), face(1)), (Only, face(2))],
            ),
            (
                "an edge under a face name",
                vec![(
                    Of(0),
                    EntityRef {
                        body: 0,
                        key: EntityKey::Edge(topo::EdgeKey::from(slotmap::KeyData::from_ffi(1))),
                    },
                )],
            ),
        ];
        for (what, pairs) in refused {
            assert!(write(pairs).is_none(), "{what} must refuse");
        }
    }

    /// Every seating door refuses an entity of another kind than its
    /// name's as that, not as a duplicate, and seats nothing.
    #[test]
    fn a_kind_mismatched_seating_refuses_as_a_kind_mismatch() {
        let edge = |i| EntityRef {
            body: 0,
            key: EntityKey::Edge(topo::EdgeKey::from(slotmap::KeyData::from_ffi(i))),
        };
        let mismatch = SeatingRefusal::KindMismatch(KindMismatch {
            name: Box::new((*name()).clone()),
            found: EntityKind::Edge,
        });
        type Door = fn(&mut NameTable, EntityRef, EntityRef) -> Result<(), SeatingRefusal>;
        let doors: [(&str, Door); 4] = [
            ("insert", |t, e, _| t.insert((*name()).clone(), e)),
            ("insert_tied", |t, e, f| {
                t.insert_tied((*name()).clone(), vec![e, f])
            }),
            ("a carried strict row", |t, e, _| {
                t.insert_candidates_ref(name(), vec![(Only, e)])
            }),
            ("a carried candidate", |t, e, _| {
                t.insert_candidates_ref(name(), vec![(Of(0), e)])
            }),
        ];
        for (door, seat) in doors {
            let mut t = NameTable::new();
            assert_eq!(
                seat(&mut t, edge(1), edge(2)),
                Err(mismatch.clone()),
                "{door}: an edge under a face name"
            );
            assert!(t.iter().next().is_none(), "{door}: nothing is seated");
            // The tied door's second candidate alone mismatched.
            if door == "insert_tied" {
                assert_eq!(
                    seat(&mut t, face(1), edge(2)),
                    Err(mismatch.clone()),
                    "{door}: an edge beside a face"
                );
            }
            // Doubly defective: the name is already held and its first
            // candidate already seated, and the kind disagrees. The kind
            // is checked first, whichever door and entity order.
            let mut held = NameTable::new();
            held.insert((*name()).clone(), face(1)).unwrap();
            assert_eq!(
                seat(&mut held, edge(1), edge(2)),
                Err(mismatch.clone()),
                "{door}: a held name under an edge"
            );
            if door == "insert_tied" {
                assert_eq!(
                    seat(&mut held, face(1), edge(2)),
                    Err(mismatch.clone()),
                    "{door}: a seated face beside an edge"
                );
            }
        }
        // A malformed list and a mismatch: still the mismatch.
        let mut t = NameTable::new();
        assert_eq!(
            t.insert_candidates_ref(name(), vec![(Only, face(1)), (Only, edge(2))]),
            Err(mismatch.clone()),
            "two strict rows, one an edge"
        );
        assert_eq!(
            t.insert_tied((*name()).clone(), vec![edge(1)]),
            Err(mismatch),
            "a one-candidate tie of an edge"
        );
    }

    /// A tie of fewer than two distinct candidates, and a carried list
    /// of none, refuse as a short tie counting what was offered, not as
    /// a duplicate, and seat nothing.
    #[test]
    fn a_short_tie_refuses_as_a_short_tie() {
        let short = |candidates| {
            Err(SeatingRefusal::ShortTie(ShortTie {
                name: Box::new((*name()).clone()),
                candidates,
            }))
        };
        type Door = fn(&mut NameTable) -> Result<(), SeatingRefusal>;
        let doors: [(&str, Door, usize); 3] = [
            (
                "a tie of none",
                |t| t.insert_tied((*name()).clone(), vec![]),
                0,
            ),
            (
                "a tie of one",
                |t| t.insert_tied((*name()).clone(), vec![face(1)]),
                1,
            ),
            (
                "a carried list of none",
                |t| t.insert_candidates_ref(name(), vec![]),
                0,
            ),
        ];
        for (door, seat, candidates) in doors {
            let mut t = NameTable::new();
            assert_eq!(seat(&mut t), short(candidates), "{door}");
            assert!(t.iter().next().is_none(), "{door}: nothing is seated");
        }
    }

    /// Every seating door refuses an entity that already holds a name
    /// as that, carrying the name it holds, and seats nothing; an
    /// entity repeated in one offered list holds the name it is
    /// offered, at the minting and the carrying door alike. A repeated
    /// (name, candidate) pair stays a duplicate name.
    #[test]
    fn an_entity_offered_a_second_name_refuses_as_entity_named() {
        let other = StableName {
            path: vec![RoleSeg::OutputBody, RoleSeg::OutputBody],
            ..(*name()).clone()
        };
        let named = |held: &StableName| {
            Err(SeatingRefusal::EntityNamed(EntityNamed {
                name: Box::new((*name()).clone()),
                held: Box::new(held.clone()),
            }))
        };
        type Door = fn(&mut NameTable) -> Result<(), SeatingRefusal>;
        let held: [(&str, Door); 4] = [
            ("insert", |t| t.insert((*name()).clone(), face(1))),
            ("insert_tied", |t| {
                t.insert_tied((*name()).clone(), vec![face(2), face(1)])
            }),
            ("a carried strict row", |t| {
                t.insert_candidates_ref(name(), vec![(Only, face(1))])
            }),
            ("a carried tie", |t| {
                t.insert_candidates_ref(name(), vec![(Of(0), face(2)), (Of(1), face(1))])
            }),
        ];
        for (door, seat) in held {
            let mut t = NameTable::new();
            t.insert(other.clone(), face(1)).unwrap();
            assert_eq!(
                seat(&mut t),
                named(&other),
                "{door}: face 1 holds another name"
            );
            assert!(t.lookup(&name()).is_none(), "{door}: nothing is seated");
            assert_eq!(t.len(), 1, "{door}: the held row stands alone");
        }
        let repeated: [(&str, Door); 4] = [
            ("a tie of one entity twice", |t| {
                t.insert_tied((*name()).clone(), vec![face(1), face(1)])
            }),
            ("a tie of two, one twice", |t| {
                t.insert_tied((*name()).clone(), vec![face(1), face(2), face(1)])
            }),
            ("one entity as two candidates", |t| {
                t.insert_candidates_ref(name(), vec![(Of(0), face(1)), (Of(1), face(1))])
            }),
            ("one entity as three candidates", |t| {
                t.insert_candidates_ref(
                    name(),
                    vec![(Of(0), face(1)), (Of(1), face(2)), (Of(2), face(1))],
                )
            }),
        ];
        for (door, seat) in repeated {
            let mut t = NameTable::new();
            assert_eq!(seat(&mut t), named(&name()), "{door}");
            assert!(t.iter().next().is_none(), "{door}: nothing is seated");
        }
    }

    #[test]
    fn the_carrying_door_keeps_each_number() {
        let one = write(vec![(Of(4), face(1))]).expect("one carried candidate");
        assert!(matches!(one.entry_of(&name()), Some(Entry::Unique(_))));
        assert_eq!(one.candidate_of(&name(), &face(1)), Some(Of(4)));

        let strict = write(vec![(Only, face(1))]).expect("a strict row");
        assert_eq!(strict.candidate_of(&name(), &face(1)), Some(Only));

        // Handed in out of entity order: the numbers follow their
        // entities, not the stored order.
        let tie = write(vec![(Of(0), face(9)), (Of(1), face(2))]).expect("two candidates");
        assert!(matches!(tie.entry_of(&name()), Some(Entry::Tied(es)) if es.len() == 2));
        assert_eq!(tie.candidate_of(&name(), &face(9)), Some(Of(0)));
        assert_eq!(tie.candidate_of(&name(), &face(2)), Some(Of(1)));
    }
}
