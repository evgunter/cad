//! **The drive's table**: the tier's state that belongs to a drive and
//! is the same for every leaf of it. The expression table, the plain
//! forms, the atom records the plain forms stand on, and the set of
//! nodes that froze.
//!
//! # Why a leaf may read what another leaf wrote
//!
//! Every entry is a function of its key, so it is the entry any leaf
//! would write.
//!
//! - **A node** is its id's preimage: the id is the content hash of
//!   `(op, payload, children ids)` (D9).
//! - **A plain form** is a function of its node's id, the
//!   [`SymBudget`] and the [`SymRules`] dials the plain walk consults:
//!   the plain walk reads no value, no bracket and no registry, every
//!   atom is opaque and rule A0 is its only rule. So is whether the
//!   budget froze it, which is why the frozen set says nothing about
//!   which leaf got there first.
//! - **An atom's record** is a function of its id, a hash of the op,
//!   the payload and the argument forms' digests.
//!
//! The early and door forms are not here: they read the leaf's
//! parameter brackets and a node's registry.
//!
//! The hash-collision assumption is over a drive's nodes: two distinct
//! expressions colliding on a 128-bit content hash would be a
//! soundness break.
//!
//! # What a leaf resolves through the table, and what it cannot
//!
//! A session on a drive's table records each node as it mints it, and
//! a leaf resolves an id through its own cache and then through the
//! table, so a value built in one leaf of a drive means in another
//! what it meant where it was built.
//!
//! An id in neither was minted outside every session on the table: a
//! `Sym` is a plain value, and any public door can be handed one. It
//! is an UNKNOWN in that leaf (`Session::unknown`): an indeterminate
//! of its own, which proves nothing the node would not, and its form
//! and every plain form built over it are neither published here nor
//! left in the caches the table keeps warm, because they are that
//! leaf's reading and not a function of the id.
//!
//! **One dependence on the order of leaves is left, and it is this
//! one.** An id is its content's hash, so once any leaf has minted the
//! same content inside a session the table names it, and a leaf handed
//! that content from outside resolves it. Sound either way; stronger
//! after the minting leaf. Only a value from outside every session
//! reaches it, and the evaluation service refuses the one such value
//! it is handed whole, a prior built under another table
//! (`editor_core`'s `prior_under_this_table`).
//!
//! # A leaf's `frozen` column
//!
//! A leaf's NEED: the drive's frozen nodes inside the closure of the
//! roots its plain walk was asked for ([`super::SymCounts::frozen`]).
//! The closure is the leaf's box; the frozen set is the drive's. The
//! set is complete over that closure when the leaf reads it: a form
//! is published with its freeze in one write, children before
//! parents, so a leaf that took a form for a node took it after every
//! freeze under that node was in the set.
//!
//! # One publication per new entry, and a warm cache per leaf
//!
//! A leaf works in caches of its own (the expression table, the plain
//! forms, the atom records) and writes an entry to the table when it
//! makes one its cache does not hold. The caches outlive the leaf:
//! the table keeps them ([`DriveMemo::warm`]) and hands one to the
//! next leaf, which therefore starts with what an earlier leaf made
//! and publishes only what is new to it. Every node and plain form in
//! a cache is in the table; its atom records are more than the
//! table's (the early and retry walks' too), and each is a function of
//! its id. So which cache a leaf is handed changes what it costs and
//! nothing it answers. Nothing caps a cache: it grows to the table's
//! size, one per leaf that runs at once, and
//! [`MemoSize::bytes_estimate`] does not count it. The forms are `Arc<Form>`, so a hit
//! hands back the allocation the first leaf built.
//!
//! With the dial off ([`DriveMemo::counting_only`]) the table keeps
//! the frozen set alone: a leaf starts cold, takes nothing, and the
//! drive's `frozen` column is the same column.

use std::sync::{Arc, Mutex, RwLock};

use super::form::Form;
use super::{AtomInfo, IdMap, IdSet, IndetMap, SymBudget, SymId, SymNode, SymRules};

/// One drive's table (module docs): one per `drive(...)`, dropped with
/// it, holding nothing of the next one.
///
/// **Valid for ONE `(budget, rules)` pair.** A plain form is a function
/// of the node id and those two, so a table consulted by a leaf under a
/// different budget or different dials would hand back a form that leaf
/// would not have built. [`DriveMemo::accepts`] is the check, and the
/// door that installs a session refuses the mismatch.
pub struct DriveMemo {
    budget: SymBudget,
    rules: SymRules,
    /// Whether the table hands entries back. Off is the differential
    /// dial (`DriveConfig::plain_memo`): the frozen ids are still
    /// collected, so the drive's `frozen` column means the same thing
    /// with the dial either way.
    serves_forms: bool,
    inner: RwLock<Inner>,
    /// The caches of the leaves that are done, for the next ones.
    warm: Mutex<Vec<Warm>>,
    /// Which table this is ([`table_token`]).
    token: u64,
}

/// A token no other table of the process has: what a value built under
/// a table is told from one built under another by (`super::table`).
/// Monotone and process-wide, it identifies a table and is read as
/// nothing else.
pub(super) fn table_token() -> u64 {
    static NEXT: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(1);
    NEXT.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
}

#[derive(Default)]
struct Inner {
    nodes: IdMap<SymNode>,
    forms: IdMap<Arc<Form>>,
    atoms: IndetMap<AtomInfo>,
    /// The DISTINCT nodes frozen over the drive: a set, so it is the
    /// same under every schedule, and the drive's `frozen` column.
    frozen: IdSet,
}

/// **A leaf's caches of the drive's table**: the nodes, plain forms and
/// atom records it has made or taken. Every node and plain form in it
/// is in the table, and every atom record is its id's.
#[derive(Default)]
pub(super) struct Warm {
    pub(super) nodes: IdMap<SymNode>,
    pub(super) forms: IdMap<Arc<Form>>,
    pub(super) atoms: IndetMap<AtomInfo>,
}

/// What one drive's table came to, for the growth guard and the receipt.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct MemoSize {
    /// Expression nodes held.
    pub nodes: usize,
    /// Plain forms held.
    pub forms: usize,
    /// `AtomInfo`s held.
    pub atoms: usize,
    /// Distinct nodes frozen over the drive; the receipt's own column
    /// is [`DriveMemo::frozen`].
    pub frozen: usize,
    /// **An ESTIMATE of the heap the table occupies**, not a
    /// measurement: the map slots, each `Form`'s two term vectors and
    /// each term's monomial are counted; a map's SPARE CAPACITY is not,
    /// an atom argument that is also a memoized form is counted TWICE,
    /// and the leaves' warm caches are not counted at all. It is the
    /// same shape every run, which is all a growth guard needs.
    pub bytes_estimate: usize,
}

impl DriveMemo {
    /// A table that serves its entries for `budget` and `rules`.
    #[must_use]
    pub fn new(budget: SymBudget, rules: SymRules) -> Self {
        Self {
            budget,
            rules,
            serves_forms: true,
            inner: RwLock::new(Inner::default()),
            warm: Mutex::new(Vec::new()),
            token: table_token(),
        }
    }

    /// A table that collects the drive's distinct freezes and serves
    /// nothing: `DriveConfig::plain_memo` off. The dial is a
    /// performance dial, so the column it leaves on the receipt is the
    /// column the serving table reports.
    #[must_use]
    pub fn counting_only(budget: SymBudget, rules: SymRules) -> Self {
        Self {
            serves_forms: false,
            ..Self::new(budget, rules)
        }
    }

    /// This table's token.
    pub(super) fn token(&self) -> u64 {
        self.token
    }

    /// Whether a leaf under `budget` and `rules` may consult this
    /// table. The plain walk reads two of the dials (`const_fold` and
    /// `early`); the guard compares the WHOLE [`SymRules`], which costs
    /// a drive its hits over a dial the plain walk ignores and never
    /// has to be re-derived when a rule moves into the plain walk.
    #[must_use]
    pub fn accepts(&self, budget: SymBudget, rules: SymRules) -> bool {
        self.budget == budget && self.rules == rules
    }

    /// **The drive's `frozen` column**: how many DISTINCT nodes froze
    /// over it ([`super::SymCounts::frozen`]).
    #[must_use]
    pub fn frozen(&self) -> u64 {
        self.read().frozen.len() as u64
    }

    /// Whether the drive has frozen nothing: a leaf's NEED is then zero
    /// whatever it reached.
    pub(super) fn frozen_is_empty(&self) -> bool {
        self.read().frozen.is_empty()
    }

    /// **A leaf's NEED**: the drive's frozen nodes that lie in
    /// `reached`, the plain closure of the leaf's walk roots. Walked
    /// from the frozen end, the smaller of the two on every document
    /// measured.
    pub(super) fn need(&self, reached: &IdSet) -> u64 {
        self.read()
            .frozen
            .keys()
            .filter(|id| reached.contains_key(id))
            .count() as u64
    }

    /// What the table came to at the drive's end.
    #[must_use]
    pub fn size(&self) -> MemoSize {
        inner_size(&self.read())
    }

    /// A poisoned lock is RECOVERED rather than re-panicked: a poisoned
    /// table is a leaf that panicked mid-write, and what that leaves is
    /// fewer entries, never a wrong one, since each write is one whole
    /// entry. The panic is the leaf's own and propagates there.
    fn read(&self) -> std::sync::RwLockReadGuard<'_, Inner> {
        read_inner(&self.inner)
    }

    fn write(&self) -> std::sync::RwLockWriteGuard<'_, Inner> {
        self.inner
            .write()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
    }

    /// The caches a leaf starts with: an earlier leaf's, or empty ones
    /// for the first leaves and for a table that serves nothing.
    pub(super) fn warm(&self) -> Warm {
        if !self.serves_forms {
            return Warm::default();
        }
        self.warm
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .pop()
            .unwrap_or_default()
    }

    /// Keeps a finished leaf's caches for the next one.
    pub(super) fn rest(&self, caches: Warm) {
        if self.serves_forms {
            self.warm
                .lock()
                .unwrap_or_else(std::sync::PoisonError::into_inner)
                .push(caches);
        }
    }

    /// The node `id` names, if a leaf of this drive recorded it.
    pub(super) fn node(&self, id: SymId) -> Option<SymNode> {
        if !self.serves_forms {
            return None;
        }
        self.read().nodes.get(&id).copied()
    }

    /// Records a node a leaf minted.
    pub(super) fn record_node(&self, id: SymId, node: SymNode) {
        if self.serves_forms {
            self.write().nodes.entry(id).or_insert(node);
        }
    }

    /// The plain form of `id`, if a leaf of this drive has built it.
    pub(super) fn form(&self, id: SymId) -> Option<Arc<Form>> {
        if !self.serves_forms {
            return None;
        }
        self.read().forms.get(&id).cloned()
    }

    /// Copies into `into` every `AtomInfo` the atoms of `form` need,
    /// transitively through the atoms' own argument forms: a hit skips
    /// the walk that would have recorded them, and the reductions look
    /// an atom's argument up by id (`algebra::find_square`).
    pub(super) fn seed_atoms(&self, form: &Form, into: &mut IndetMap<AtomInfo>) {
        let inner = self.read();
        if inner.atoms.is_empty() {
            return;
        }
        let mut stack: Vec<u128> = Vec::new();
        push_atoms(form, &mut stack);
        while let Some(id) = stack.pop() {
            if into.contains_key(&id) {
                continue;
            }
            let Some(info) = inner.atoms.get(&id) else {
                continue;
            };
            for arg in info.args.iter().flatten() {
                push_atoms(arg, &mut stack);
            }
            into.insert(id, info.clone());
        }
    }

    /// Publishes an atom record the plain walk referenced, before the
    /// form that stands on it.
    pub(super) fn publish_atom(&self, id: u128, info: &AtomInfo) {
        if !self.serves_forms || self.read().atoms.contains_key(&id) {
            return;
        }
        self.write().atoms.entry(id).or_insert_with(|| info.clone());
    }

    /// **Publishes one plain form a leaf built**, with whether the
    /// budget froze it, in one write: no reader sees the form without
    /// its freeze. A form already here was built for the same id under
    /// the same budget and rules, so it is this one, and the first is
    /// kept with the `Arc` every earlier hit handed out.
    pub(super) fn publish_form(&self, id: SymId, form: &Arc<Form>, froze: bool) {
        // A table that serves nothing keeps the freezes alone.
        if !froze && !self.serves_forms {
            return;
        }
        let mut inner = self.write();
        if froze {
            inner.frozen.insert(id, ());
        }
        if !self.serves_forms {
            return;
        }
        match inner.forms.entry(id) {
            std::collections::hash_map::Entry::Occupied(e) => {
                debug_assert_eq!(
                    **e.get(),
                    **form,
                    "two leaves of one drive built different plain forms for one node id"
                );
            }
            std::collections::hash_map::Entry::Vacant(e) => {
                e.insert(Arc::clone(form));
            }
        }
    }
}

/// [`DriveMemo::read`]'s recovery, over the lock rather than over the
/// memo, so the `Debug` below can take the guard from the `inner` its
/// own destructure binds. The argument for recovering rather than
/// re-panicking is [`DriveMemo::read`]'s and is not repeated.
fn read_inner(lock: &RwLock<Inner>) -> std::sync::RwLockReadGuard<'_, Inner> {
    lock.read()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
}

/// What an [`Inner`] holds, as the [`MemoSize`] a receipt reports.
///
/// Named apart from `core::mem::size_of`, which the prelude carries.
///
/// A free function over the guard rather than a method on
/// [`DriveMemo`], so the dump can compute it from the `inner` its own
/// destructure binds instead of reaching back through `self` — a
/// binding read is what holds the dump to the declaration.
fn inner_size(inner: &Inner) -> MemoSize {
    let bytes_estimate = inner.forms.values().map(|f| form_bytes(f)).sum::<usize>()
        + inner.forms.len() * core::mem::size_of::<(SymId, Arc<Form>)>()
        + inner
            .atoms
            .values()
            .map(|a| {
                a.args
                    .iter()
                    .flatten()
                    .map(|f| form_bytes(f))
                    .sum::<usize>()
            })
            .sum::<usize>()
        + inner.atoms.len() * core::mem::size_of::<(u128, AtomInfo)>()
        + inner.frozen.len() * core::mem::size_of::<SymId>()
        + inner.nodes.len() * core::mem::size_of::<(SymId, SymNode)>();
    MemoSize {
        nodes: inner.nodes.len(),
        forms: inner.forms.len(),
        atoms: inner.atoms.len(),
        frozen: inner.frozen.len(),
        bytes_estimate,
    }
}

/// **The dump is held to the declaration**: `Self` is destructured
/// exhaustively, so a field added to [`DriveMemo`] is an E0027
/// unbound-pattern error rather than a value silently absent from every
/// dump. `inner` is carried as its [`MemoSize`] — what the memo HOLDS
/// is what its dump is asked for, where the tables themselves are every
/// form and every atom of the drive — and it is the BINDING that is
/// summarised, not `self` reached through again: a field shown as a
/// summary is a field shown, and it must not wear the `_` spelling that
/// stands for one left out.
///
/// `budget` and `rules` are left out (the drive's configuration, not
/// its contents) and so are the leaves' warm caches, which hold
/// nothing the table does not, so this still ends in
/// `finish_non_exhaustive` rather than claiming every field is shown.
impl core::fmt::Debug for DriveMemo {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let Self {
            budget: _,
            rules: _,
            serves_forms,
            inner,
            warm: _,
            token: _,
        } = self;
        f.debug_struct("DriveMemo")
            .field("serves_forms", serves_forms)
            .field("size", &inner_size(&read_inner(inner)))
            .finish_non_exhaustive()
    }
}

/// Every atom id the form mentions, at the top level.
fn push_atoms(form: &Form, out: &mut Vec<u128>) {
    for poly in [&form.num, &form.den] {
        for mono in poly.monos() {
            out.extend(mono.iter().map(|&(id, _)| id));
        }
    }
}

/// One form's heap, term by term (the map slot is counted by the
/// caller).
fn form_bytes(form: &Form) -> usize {
    let poly = |p: &super::Poly| {
        core::mem::size_of_val(p.terms())
            + p.monos()
                .map(|m| m.len() * core::mem::size_of::<(u128, u32)>())
                .sum::<usize>()
    };
    core::mem::size_of::<Form>() + poly(&form.num) + poly(&form.den)
}
