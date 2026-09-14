//! **The drive-scoped plain memo**: the one piece of the tier's state
//! that outlives a leaf.
//!
//! # Why it is sound
//!
//! A node's id is the content hash of `(op, payload, children ids)`
//! (D9), and the PLAIN walk reads no value: every atom is opaque, rule
//! A0 is the only rule, and no bracket, no registry and no enclosure
//! reaches it. So a node's plain form is a function of its id, the
//! session's [`SymBudget`] and the two [`SymRules`] dials the plain
//! walk consults — and of nothing else, the leaf included. Two leaves
//! that build a node with one id build the same plain form for it.
//!
//! That is the argument the tier already makes for two OCCURRENCES of a
//! node inside one leaf (which is what `Session::forms` is); this memo
//! applies it across the leaves of one drive.
//!
//! It rests on one premise that is not a content hash. An `Opaque`
//! node's payload is the SEQUENCE NUMBER the leaf minted it at
//! (`OPAQUE_SEQ`), so leaf-invariance there is a property of the
//! evaluation service's fixed per-leaf walk rather than of the hash.
//! That premise is pinned by execution, not assumed:
//! `editor-core`'s `m10_sym_drive_memo_interval` drives each document
//! and asserts every leaf mints the same set of `Opaque` ids. It is the
//! first row that reds if a lane ever mints an opaque under a
//! value-dependent branch.
//!
//! The drive-wide hash-collision assumption is the per-leaf one widened:
//! two distinct expressions colliding on a 128-bit content hash would be
//! a soundness break, and the population the assumption is made over is
//! now a drive's nodes rather than a leaf's.
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
//! misses on and ONE write lock at its end, publishing everything it
//! computed in one pass. One write per node would put every worker's
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
use super::{AtomInfo, IdMap, IndetMap, SymBudget, SymId, SymRules};

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
    frozen: IdMap<()>,
}

/// What one drive's memo came to, for the growth guard and the receipt.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct MemoSize {
    /// Plain forms held.
    pub forms: usize,
    /// `AtomInfo`s held.
    pub atoms: usize,
    /// Distinct nodes frozen over the drive.
    pub frozen: u64,
    /// The heap the forms and atoms occupy, counted term by term: the
    /// map slots, each `Form`'s two term vectors, and each term's
    /// monomial. An estimate of the same shape every run, so it is a
    /// number a guard can be written against.
    pub bytes: usize,
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
    #[must_use]
    pub fn accepts(&self, budget: SymBudget, rules: SymRules) -> bool {
        self.budget == budget && self.rules == rules
    }

    /// **The drive's `frozen` column**: how many DISTINCT nodes froze
    /// over it. A set, so it is identical under every schedule — which
    /// a sum of the leaves' own counts is not once a leaf can inherit a
    /// form another leaf froze.
    #[must_use]
    pub fn frozen(&self) -> u64 {
        self.read().frozen.len() as u64
    }

    /// What the memo came to at the drive's end.
    #[must_use]
    pub fn size(&self) -> MemoSize {
        let inner = self.read();
        let bytes = inner.forms.values().map(|f| form_bytes(f)).sum::<usize>()
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
            frozen: inner.frozen.len() as u64,
            bytes,
        }
    }

    /// A poisoned lock is a panic in another leaf's publish, which has
    /// left the map half-written; serving from it would make a receipt
    /// that reports a drive nobody ran.
    fn read(&self) -> std::sync::RwLockReadGuard<'_, Inner> {
        self.inner
            .read()
            .expect("the drive memo's lock is poisoned: a leaf panicked mid-publish")
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
    /// the same id under the same budget and rules, so it is this one,
    /// and keeping the first keeps the `Arc` every earlier hit handed
    /// out.
    pub(super) fn publish(
        &self,
        forms: impl Iterator<Item = (SymId, Arc<Form>)>,
        atoms: impl Iterator<Item = (u128, AtomInfo)>,
        frozen: impl Iterator<Item = SymId>,
    ) {
        let mut inner = self
            .inner
            .write()
            .expect("the drive memo's lock is poisoned: a leaf panicked mid-publish");
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
            inner.forms.entry(id).or_insert(f);
        }
    }
}

impl core::fmt::Debug for DriveMemo {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("DriveMemo")
            .field("serves_forms", &self.serves_forms)
            .field("size", &self.size())
            .finish()
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
        p.terms().len() * core::mem::size_of::<(super::form::Mono, super::Rat)>()
            + p.monos()
                .map(|m| m.len() * core::mem::size_of::<(u128, u32)>())
                .sum::<usize>()
    };
    core::mem::size_of::<Form>() + poly(&form.num) + poly(&form.den)
}
