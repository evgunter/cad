//! **One drive's node results** (E6's leaf protocol: "a node whose
//! inputs a leaf leaves unchanged is served from an earlier replay of
//! the drive: result or refusal, and record, whole").
//!
//! A drive replays one document over many boxes. A node's result (its
//! value or its refusal, with the record it left in the symbolic
//! session) is a function of its inputs, so a leaf that reaches a node
//! under a key an earlier leaf holds is served that leaf's result
//! through the evaluator's prior door ([`super::Prior::Store`]) and
//! does not run the op.
//!
//! # The key, and why it is the proof
//!
//! [`StoreKey`]: the node's id, its content key, its naming key, and the
//! leaf box's brackets on the node's READ SET, by bits. The read set is
//! [`Doc::free_reads`]'s, transitive through definitions and through
//! every operation the node reads ([`Doc::upstream_of`], the edges the
//! schedule runs on), and the whole box for a clearance, whose verifier
//! reads the `f64` rebuild at the box's midpoint ([`super::wire::Witnessing`]).
//!
//! Inside one drive the document is fixed, so every value a node can
//! read, through a read [`super::wire::seen_reads`] lists or any other
//! the op makes, is a function of the document and those brackets: two
//! leaves with equal brackets on the read set hold equal inputs at the
//! node whatever the op reads
//! (`work/boxfinish/a-nodes-op-and-its-keys-walk-its-reads-separately`
//! asks the narrower question of the content key alone). The brackets
//! are also what a node's decisions read of the box: the symbolic
//! tier's early walk reads the brackets of the parameters a form
//! reaches, every one of which is a free variable the node reads, and
//! the content key hashes no bracket (`geom_core::sym::NodeRecord`).
//! The content and naming keys are the evaluator's memo key; beside the
//! brackets they add, inside one drive, only the member a family's step
//! is (`members::member_key`), which no bracket names.
//!
//! The key is built here and nowhere else: a lookup is handed the leaf's
//! box and finds the brackets itself.
//!
//! # Each entry computed once
//!
//! A slot is absent, claimed by the leaf computing it, or held. A leaf
//! that finds a slot claimed waits for it; one that finds it absent
//! claims it, computes, and publishes. A claim dropped unpublished (a
//! canceled run, an unwinding one) empties the slot and wakes its
//! waiters, one of which claims it again.
//!
//! **Why a wait cannot close a cycle.** A leaf looks the store up only
//! between nodes of the drive's own document, holding no claim: a
//! claim is held while its node computes, and computing a node looks
//! nothing up (a part's run and a nested document take no prior), so no
//! thread waits while it holds a claim. **That holds only while no
//! thread that can wait is a worker of a pool whose tasks a claim
//! holder can steal**: a holder that nests a parallel map would
//! otherwise run a waiting leaf on its own stack, under the claim it
//! is waiting for.
//!
//! # What is not served
//!
//! - A node that instantiates a part: its keys name the part by
//!   reference alone, and the part's run is entered in each replay.
//! - Anything of a nested document, which takes no prior.
//! - Anything under another expression table than the drive's
//!   ([`super::PriorRefusal::Table`]), a deferring session's included:
//!   it is always on a table of its own.
//! - Anything while the shape report records decisions: a served node
//!   pushes no row (`geom_core::sym::reports_decisions`). The drive
//!   installs no store then.
//! - A value that carries a door's census, or a node that reads one
//!   (`Carry::touches_census`): a census runs once, where its first
//!   reader asks for it, and logs and counts in that reader, so it is
//!   built in every leaf with every reader of it.
//! - A failure before the node's keys: there is nothing to look it up
//!   by, and nothing ran that a store would save.

use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::sync::{Arc, Condvar, Mutex, MutexGuard, PoisonError};

use super::{ContentKey, NamingKey, NodeResult};
use crate::analysis::ParamBox;
use crate::doc::Doc;
use crate::node::Node;
use crate::{ProfileProgram, RecipeNodeId, VarId};
use geom_core::predicate::Decide;

/// **What each node of a document reads of a box**: the free variables
/// it reads, or the whole box. One per drive, shared by the store and
/// by the drive's count of distinct results, so the two key a result
/// alike.
#[derive(Debug)]
pub(crate) struct ReadSets {
    reads: BTreeMap<RecipeNodeId, BTreeSet<VarId>>,
    whole: BTreeSet<RecipeNodeId>,
}

impl ReadSets {
    /// Every node of `doc`: [`Doc::free_reads`], and the whole box for a
    /// clearance assertion.
    pub(crate) fn of(doc: &Doc<ProfileProgram>) -> Self {
        let reads = doc.free_reads();
        let whole = reads
            .keys()
            .copied()
            .filter(|&id| {
                matches!(
                    doc.node(id),
                    Some(Node::Assertion(claim))
                        if matches!(claim, crate::measure::Claim::Clearance { .. })
                )
            })
            .collect();
        Self { reads, whole }
    }

    /// The read set of `node`.
    pub(crate) fn reads(&self, node: RecipeNodeId) -> Option<&BTreeSet<VarId>> {
        self.reads.get(&node)
    }

    /// **The key a result of `node` is held under** at `box_`.
    pub(crate) fn key(
        &self,
        node: RecipeNodeId,
        content: ContentKey,
        naming: NamingKey,
        box_: &ParamBox,
    ) -> StoreKey {
        let span = |var: VarId| {
            box_.get(var).map(|axis| {
                let (lo, hi) = axis.span();
                (var, lo.to_bits(), hi.to_bits())
            })
        };
        let brackets = if self.whole.contains(&node) {
            box_.order().iter().filter_map(|&var| span(var)).collect()
        } else {
            self.reads
                .get(&node)
                .into_iter()
                .flatten()
                .filter_map(|&var| span(var))
                .collect()
        };
        StoreKey {
            node,
            content,
            naming,
            brackets,
        }
    }
}

/// **The key of one node result in a drive** (module docs): built by
/// [`ReadSets::key`] alone.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub(crate) struct StoreKey {
    node: RecipeNodeId,
    content: ContentKey,
    naming: NamingKey,
    /// The box's brackets on the read set, by bits.
    brackets: Box<[(VarId, u64, u64)]>,
}

/// One slot of the store.
enum Slot<T: topo::AtRestPolicy + Decide> {
    /// A leaf is computing it.
    Claimed,
    /// The result, with its record.
    Held(NodeResult<T>),
}

/// **One drive's node results** (module docs).
pub(crate) struct NodeStore<T: topo::AtRestPolicy + Decide> {
    reads: Arc<ReadSets>,
    seat: super::PriorSeat,
    slots: Mutex<HashMap<StoreKey, Slot<T>>>,
    /// Signalled whenever a slot is published or emptied.
    settled: Condvar,
}

/// What a lookup found.
pub(crate) enum Taken<'a, T: topo::AtRestPolicy + Decide> {
    /// An earlier leaf's result.
    Held(NodeResult<T>),
    /// Nothing: this leaf computes it, and publishes it through the
    /// claim.
    Claim(Claim<'a, T>),
}

/// **A slot this leaf is computing.** Dropped unpublished, it empties
/// the slot and wakes the waiters.
pub(crate) struct Claim<'a, T: topo::AtRestPolicy + Decide> {
    store: &'a NodeStore<T>,
    key: Option<StoreKey>,
}

/// What the store came to at the drive's end.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct NodeStoreSize {
    /// Node results held: each computed once.
    pub entries: usize,
    /// Of them, refusals.
    pub refusals: usize,
}

impl<T: topo::AtRestPolicy + Decide> NodeStore<T> {
    /// An empty store for runs at `seat` ([`super::PriorSeat`]), keyed
    /// by `reads`.
    pub(crate) fn new(reads: Arc<ReadSets>, seat: super::PriorSeat) -> Self {
        Self {
            reads,
            seat,
            slots: Mutex::new(HashMap::new()),
            settled: Condvar::new(),
        }
    }

    /// What a run handed this store compares itself to.
    pub(crate) fn seat(&self) -> &super::PriorSeat {
        &self.seat
    }

    fn slots(&self) -> MutexGuard<'_, HashMap<StoreKey, Slot<T>>> {
        // A poisoned lock is a leaf that panicked holding it; every
        // write is one whole slot, so what is left is fewer entries,
        // never a wrong one.
        self.slots.lock().unwrap_or_else(PoisonError::into_inner)
    }

    /// **The result held under `node`'s keys at `box_`**, waiting for a
    /// leaf that is computing it, or a claim to compute it.
    pub(crate) fn take(
        &self,
        node: RecipeNodeId,
        content: ContentKey,
        naming: NamingKey,
        box_: &ParamBox,
    ) -> Taken<'_, T> {
        let key = self.reads.key(node, content, naming, box_);
        let mut slots = self.slots();
        loop {
            match slots.get(&key) {
                Some(Slot::Held(result)) => return Taken::Held(result.clone()),
                Some(Slot::Claimed) => {
                    slots = self
                        .settled
                        .wait(slots)
                        .unwrap_or_else(PoisonError::into_inner);
                }
                None => {
                    slots.insert(key.clone(), Slot::Claimed);
                    return Taken::Claim(Claim {
                        store: self,
                        key: Some(key),
                    });
                }
            }
        }
    }

    /// What the store holds now.
    pub(crate) fn size(&self) -> NodeStoreSize {
        let slots = self.slots();
        let mut size = NodeStoreSize::default();
        for slot in slots.values() {
            if let Slot::Held(result) = slot {
                size.entries += 1;
                size.refusals += usize::from(matches!(result, NodeResult::Failed(_)));
            }
        }
        size
    }
}

impl<T: topo::AtRestPolicy + Decide> Claim<'_, T> {
    /// Publishes the result this leaf computed, with its record.
    pub(crate) fn publish(mut self, result: &NodeResult<T>) {
        let Some(key) = self.key.take() else { return };
        let mut slots = self.store.slots();
        slots.insert(key, Slot::Held(result.clone()));
        drop(slots);
        self.store.settled.notify_all();
    }
}

impl<T: topo::AtRestPolicy + Decide> Drop for Claim<'_, T> {
    fn drop(&mut self) {
        if let Some(key) = self.key.take() {
            let mut slots = self.store.slots();
            slots.remove(&key);
            drop(slots);
            self.store.settled.notify_all();
        }
    }
}
