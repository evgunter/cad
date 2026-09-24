//! **The drive-scoped plain memo**: the one piece of the tier's state
//! that outlives a leaf.
//!
//! # Why it is sound — THE argument, said once
//!
//! A node's id is the content hash of `(op, payload, children ids)`
//! (D9) — a hash of SYNTAX: a `Lit`'s bits, a `Param`'s symbol, an
//! `Opaque`'s sequence number, an atom's arguments' digests. And the
//! PLAIN walk reads no value: every atom is opaque, rule A0 is the only
//! rule, and no bracket, no registry and no enclosure reaches it. So a
//! node's plain form is a function of its id, the session's
//! [`SymBudget`] and the two [`SymRules`] dials the plain walk consults
//! — and of nothing else, the leaf included. Two leaves that build a
//! node with one id build the same plain form for it.
//!
//! That is the argument the tier already makes for two OCCURRENCES of a
//! node inside one leaf (which is what `Session::forms` is); this memo
//! applies it across the leaves of one drive.
//!
//! **The opaque sequence governs the HIT RATE, not soundness.** An
//! `Opaque` node's payload is the sequence number the leaf minted it at
//! (`OPAQUE_SEQ`), which is the one part of an id that is not a hash of
//! anything the expression says. What follows if two leaves hand one
//! sequence number to two different reals is NOT an unsound answer: a
//! plain form is a syntactic normal form of a syntactic id, so a `Zero`
//! it reaches is an identity in whatever unknowns the syntax names —
//! `opaque(3) − opaque(3)` is zero whichever real either leaf bound —
//! and that holds leaf by leaf. What a leaf-varying sequence costs is
//! MISSES: the second leaf builds different ids and finds nothing here.
//! Demonstrated by execution rather than argued —
//! `geom-core`'s `sym_drive_memo` plants a leaf-varying mint and shows
//! every decision unmoved (R2), and `editor-core`'s
//! `m10_sym_drive_memo_interval` reports each leaf's `Opaque` set.
//!
//! The drive-wide hash-collision assumption is the per-leaf one widened:
//! two distinct expressions colliding on a 128-bit content hash would be
//! a soundness break, and the population the assumption is made over is
//! now a drive's nodes rather than a leaf's.
//!
//! **An unrecorded node is neither read nor written.** `form_in` freezes
//! a node absent from the leaf's own hash-consing table, and that freeze
//! stays in the leaf: publishing it under the node's content id would
//! hand an indeterminate to a leaf that DID record the node and would
//! have built a real form for it — a decision moved, and an
//! order-dependent one. The guard is on the TAINT, not on that node
//! alone (`Session::plain_tainted`): the recorded PARENT of an
//! unrecorded node has the same id in both leaves and a different form,
//! so the refusal has to follow the taint up the walk.
//!
//! The other direction of that asymmetry is sound but not
//! order-independent: a leaf that did NOT record some node can still
//! take a recorded parent's published form and reach a theorem its own
//! freeze would have cost it — which is a stronger answer, not a wrong
//! one, and which depends on whether the publishing leaf ran first. **A
//! receipt is schedule-independent only while no leaf of the drive
//! reaches that branch at all**, and none does: `editor-core`'s
//! `no_leaf_of_a_drive_freezes_a_node_its_session_never_recorded`
//! counts `FreezeCause::Unrecorded` over both documents and pins it at
//! zero. Both directions are rows in `geom-core`'s `sym_drive_memo`.
//!
//! # A LEAF's NEED, and the one reading that is still the schedule's
//!
//! A leaf receipt's `frozen` column is that leaf's NEED: the frozen
//! nodes its own reasoning rested on ([`super::SymCounts::frozen`] says
//! what the column means on each receipt). `super::leaf_need` reads it
//! as a union of three sets, all inside the closure of the leaf's
//! plain-walk roots — this memo's frozen ids, the ids the leaf's own
//! table does not hold, and the freezes the leaf made and could not
//! publish.
//!
//! **The leaf's side is its box.** Its hash-consing table is the DAG
//! its replay built and its roots are the decisions it asked a plain
//! form of; a drive-memo hit changes how much of that closure the walk
//! WALKS but not what is in it, which is why the count is taken over
//! the table and not over `Session::forms`.
//!
//! **The drive's side is the drive's.** By the argument this header
//! opens with, a node's plain form — and therefore the budget's
//! verdict on it — is a function of its id, so every leaf that
//! computes a node freezes it or none does and the set says nothing
//! about who got there first.
//!
//! **The set is complete when the leaf reads it.** Take a frozen node
//! in the closure. Either this leaf computed it, froze it and
//! published — the count is taken after the leaf's own publish — or it
//! took a form for that node or for an ancestor, and the leaf that
//! published that form published its freezes in the same call:
//! [`DriveMemo::publish`] takes ONE write lock for the whole
//! publication, so no reader sees a form without the freezes that came
//! with it, and the induction carries through a publisher that was
//! itself served by an earlier one. (The order inside that lock is a
//! separate promise, for poison recovery; [`DriveMemo::read`] is where
//! it is argued.)
//!
//! **What the leaf cannot resolve is counted from its TABLE, not from
//! its walk.** A node absent from the table freezes when the walk
//! reaches it and that freeze never leaves the leaf — but whether the
//! walk reaches it is this memo's business, because a hit at a
//! recorded ancestor skips the subtree under it. Counting the freezes
//! made would therefore read 1 when the leaf ran first and 0 when it
//! ran after a leaf that recorded the node, with every decision
//! standing still; counting the ids the table does not hold
//! (`Session::foreign`) reads the same in both orders, because the
//! table is the leaf's own. The freezes it could not publish are
//! unioned in beside them, so a node another leaf froze and published
//! is counted once however the two ran.
//!
//! **What is left is one reading, and it is the schedule's.** A freeze
//! the TAINT caused — a recorded node whose form the leaf built out of
//! an unrecorded one, which fits the budget for a leaf that records
//! that node and does not for this one — is in no drive's set and is
//! not in the leaf's table either, so it is counted exactly where the
//! walk made it, which a hit above it can take away. Both readings are
//! true of what the leaf did: in the second order its reasoning really
//! is the stronger one this header calls sound but not
//! order-independent. The row that pins the reading is `geom-core`'s
//! `sym_drive_memo::a_taint_induced_freeze_under_a_hit_is_read_by_order`
//! and the residue is
//! `work/sym/a-taint-induced-freeze-under-a-hit-still-reads-by-order`;
//! the branch itself is pinned at zero over every drive measured
//! (`editor-core`'s
//! `no_leaf_of_a_drive_freezes_a_node_its_session_never_recorded`, five
//! drives), because a drive mints every node inside its own session.
//!
//! # What it holds, and what it does not
//!
//! The plain forms, the `AtomInfo`s the plain walk minted for them, and
//! the set of ids that FROZE. Not the hash-consing table (`Session::nodes`
//! stays per leaf), not the early or door memos (they consult the leaf's
//! registry and its parameter brackets, which a value-dependent refusal
//! can make differ between leaves), not `params`, not the registry.
//!
//! # The lock discipline: one write per LEAF
//!
//! A read-mostly map behind one `RwLock`, shared across the drive's
//! rayon workers so that what the receipt reports is a function of the
//! drive and not of the schedule. A leaf takes a read lock per node it
//! misses on, a second on each HIT (`seed_atoms`, for the atoms that
//! form's indeterminates stand for), and ONE write lock at its end,
//! publishing everything it computed in one pass. One write per node would put every worker's
//! every node through a single exclusive lock — the plain walk is half
//! of a leaf's replay, so that is the whole drive serialized — and buys
//! nothing, because a form another worker is computing concurrently is a
//! form this worker has already started and will finish either way. The
//! cost of the choice is duplicated work on the leaves running
//! concurrently with the first one; the cost of the alternative is the
//! parallel schedule.
//!
//! The forms are `Arc<Form>`, so a hit hands back the allocation the
//! first leaf built rather than a copy of it: a deep clone per hit would
//! be most of what the memo is meant to save.

use std::sync::{Arc, RwLock};

use super::form::Form;
use super::{AtomInfo, IdMap, IdSet, IndetMap, SymBudget, SymId, SymRules};

/// The drive's shared plain-form memo — one per `drive(...)`, dropped
/// with it, holding nothing of the next one.
///
/// **Valid for ONE `(budget, rules)` pair.** A plain form is a function
/// of the node id and those two, so a memo consulted by a leaf under a
/// different budget or different dials would hand back a form that leaf
/// would not have built. [`DriveMemo::accepts`] is the check, and the
/// door that installs a session refuses the mismatch rather than
/// silently serving it.
pub struct DriveMemo {
    budget: SymBudget,
    rules: SymRules,
    /// Whether the memo hands FORMS back. Off is the differential dial
    /// (`DriveConfig::plain_memo`): the frozen ids are still collected,
    /// so the drive's `frozen` column means the same thing with the dial
    /// either way and the differential compares like with like.
    serves_forms: bool,
    inner: RwLock<Inner>,
}

#[derive(Default)]
struct Inner {
    forms: IdMap<Arc<Form>>,
    atoms: IndetMap<AtomInfo>,
    /// The DISTINCT nodes frozen over the drive — a set, so it is the
    /// same under every schedule, and the drive's `frozen` column.
    frozen: IdSet,
}

/// What one drive's memo came to, for the growth guard and the receipt.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct MemoSize {
    /// Plain forms held.
    pub forms: usize,
    /// `AtomInfo`s held.
    pub atoms: usize,
    /// Distinct nodes frozen over the drive — the same kind of quantity
    /// as the two above and therefore the same width; the receipt's own
    /// column is [`DriveMemo::frozen`].
    pub frozen: usize,
    /// **An ESTIMATE of the heap the forms and atoms occupy**, and the
    /// name says so because it is not a measurement: the map slots, each
    /// `Form`'s two term vectors and each term's monomial are counted;
    /// a map's SPARE CAPACITY is not, and an atom argument that is also
    /// a memoized form is counted TWICE (the `Arc` is one allocation,
    /// this walk reaches it by two paths). It is the same shape every
    /// run, which is all a growth guard needs — and the guard's ceiling
    /// is stated as a ceiling on this estimate, not on the heap.
    pub bytes_estimate: usize,
}

impl DriveMemo {
    /// A memo that serves plain forms for `budget` and `rules`.
    #[must_use]
    pub fn new(budget: SymBudget, rules: SymRules) -> Self {
        Self {
            budget,
            rules,
            serves_forms: true,
            inner: RwLock::new(Inner::default()),
        }
    }

    /// A memo that collects the drive's distinct freezes and serves NO
    /// form — `DriveConfig::plain_memo` off. The dial is a performance
    /// dial and nothing else, so the column it leaves on the receipt is
    /// the same column the memo-on run reports.
    #[must_use]
    pub fn counting_only(budget: SymBudget, rules: SymRules) -> Self {
        Self {
            serves_forms: false,
            ..Self::new(budget, rules)
        }
    }

    /// Whether a leaf under `budget` and `rules` may consult this memo.
    ///
    /// The plain walk reads exactly two of the dials — `const_fold` and
    /// `early` (`combine`'s `a0`) — so those two and the budget are what
    /// a plain form's identity actually depends on. The guard compares
    /// the WHOLE [`SymRules`] anyway, CONSERVATIVELY: refusing a memo
    /// over a dial the plain walk ignores costs a drive its hits and
    /// nothing else, while a guard that tracked the two by name would
    /// have to be re-derived every time a rule moves into or out of the
    /// plain walk. The rows pin both halves — each of the two refused,
    /// and a third the walk does not read refused too.
    #[must_use]
    pub fn accepts(&self, budget: SymBudget, rules: SymRules) -> bool {
        self.budget == budget && self.rules == rules
    }

    /// **The drive's `frozen` column**: how many DISTINCT nodes froze
    /// over it — a set, so it is identical under every schedule.
    /// [`super::SymCounts::frozen`] argues the column.
    #[must_use]
    pub fn frozen(&self) -> u64 {
        self.read().frozen.len() as u64
    }

    /// Whether the drive has frozen nothing at all — the door a leaf
    /// asks before it walks its own DAG, because a leaf's NEED is zero
    /// over an empty frozen set whatever the leaf reached.
    pub(super) fn frozen_is_empty(&self) -> bool {
        self.read().frozen.is_empty()
    }

    /// **A leaf's NEED**: the DISTINCT nodes of the drive's frozen set
    /// that lie in `reached` — the plain closure of the leaf's own walk
    /// roots — TOGETHER WITH the freezes the leaf could not publish
    /// (`unpublished`, which the closure contains). The header argues
    /// why that number is the same under every schedule;
    /// `super::leaf_need` builds both sets.
    ///
    /// A union, counted without materialising one: the drive's side is
    /// walked from the FROZEN end, which is the smaller of the two on
    /// every document measured (1,044 ids against a 17,624-node closure
    /// on the plate), and the leaf's own side counts only what the
    /// drive's does not already hold. That choice is a rounding error
    /// in the column's cost and is made because it is also the simpler
    /// code: measured over a 48-leaf plate drive, the whole column
    /// costs 222 ms and this counting is 1.2 ms of it — what a leaf's
    /// NEED actually pays for is `Session::closure`, which materialises
    /// the walk it counts over.
    ///
    /// Both counts under ONE read lock, so the two halves are read
    /// against one state of the memo rather than two.
    pub(super) fn need(&self, reached: &IdSet, unpublished: &IdSet) -> u64 {
        let inner = self.read();
        let shared = inner
            .frozen
            .keys()
            .filter(|id| reached.contains_key(id))
            .count();
        let own = unpublished
            .keys()
            .filter(|id| !inner.frozen.contains_key(id))
            .count();
        (shared + own) as u64
    }

    /// What the memo came to at the drive's end.
    #[must_use]
    pub fn size(&self) -> MemoSize {
        inner_size(&self.read())
    }

    /// A poisoned lock is RECOVERED rather than re-panicked, the same
    /// choice `k_stats` makes at its own: a poisoned memo is a leaf that
    /// panicked mid-publish, and what that leaves behind is FEWER
    /// entries, never a wrong one — each insert is a whole `(id, form)`
    /// pair of a form that was already built. The panic is the leaf's
    /// own and propagates there; re-panicking here would turn it into
    /// every other worker's too.
    fn read(&self) -> std::sync::RwLockReadGuard<'_, Inner> {
        read_inner(&self.inner)
    }

    /// The plain form of `id`, if another leaf of this drive has already
    /// built it; `None` when the memo serves no forms.
    pub(super) fn form(&self, id: SymId) -> Option<Arc<Form>> {
        if !self.serves_forms {
            return None;
        }
        self.read().forms.get(&id).cloned()
    }

    /// Copies into `into` every `AtomInfo` the atoms of `form` need,
    /// transitively through the atoms' own argument forms.
    ///
    /// A hit skips the walk that would have MINTED those atoms, and the
    /// top-residual reduce and the per-node reduction both look an
    /// atom's argument back up by id (`algebra::find_square`). Seeding
    /// the closure of the form actually handed back — rather than the
    /// whole map — keeps what the leaf holds to what it would have held
    /// had it built the form itself.
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

    /// **One write lock per leaf**: everything the leaf's plain walk
    /// computed, in one pass.
    ///
    /// `or_insert`, never overwrite — a form already here was built for
    /// the same id under the same budget and rules, so it IS this one,
    /// and keeping the first keeps the `Arc` every earlier hit handed
    /// out. That premise is checked rather than assumed: `Form` is `Eq`,
    /// so the occupied arm asserts it at debug cost.
    ///
    /// **The write ORDER is frozen, then atoms, then forms**, and it is
    /// what makes [`Self::read`]'s poison recovery true: a panic partway
    /// through leaves a memo with fewer entries than the leaf offered,
    /// never a form whose atoms are missing, because every atom a
    /// published form can reference is already in by the time any form
    /// is.
    pub(super) fn publish(
        &self,
        forms: impl Iterator<Item = (SymId, Arc<Form>)>,
        atoms: impl Iterator<Item = (u128, AtomInfo)>,
        frozen: impl Iterator<Item = SymId>,
    ) {
        let mut inner = self
            .inner
            .write()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        for id in frozen {
            inner.frozen.insert(id, ());
        }
        if !self.serves_forms {
            return;
        }
        for (id, info) in atoms {
            inner.atoms.entry(id).or_insert(info);
        }
        for (id, f) in forms {
            match inner.forms.entry(id) {
                std::collections::hash_map::Entry::Occupied(e) => {
                    // Two workers built one id concurrently. The header's
                    // whole argument is that they built the same form;
                    // this is that argument, checked.
                    debug_assert_eq!(
                        **e.get(),
                        *f,
                        "two leaves of one drive built different plain forms for one node id"
                    );
                }
                std::collections::hash_map::Entry::Vacant(e) => {
                    e.insert(f);
                }
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
        + inner.frozen.len() * core::mem::size_of::<SymId>();
    MemoSize {
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
/// `budget` and `rules` are the two left out — they are the drive's
/// configuration, not its contents — so this still ends in
/// `finish_non_exhaustive` rather than claiming every field is shown.
impl core::fmt::Debug for DriveMemo {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let Self {
            budget: _,
            rules: _,
            serves_forms,
            inner,
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
