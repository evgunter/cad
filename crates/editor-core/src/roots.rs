//! **Explicit product roots** — the ordered root list's invariants and
//! its automatic maintenance (ASSEMBLY-DESIGN A10; ASM-ROOTS D-2/D-3).
//!
//! The root list is document data, never a DAG node: an ordered,
//! duplicate-free list of live [`RecipeNodeId`]s naming what the
//! document PRODUCES. Two invariants hold at rest and after every
//! edit:
//!
//! - **Coverage** — every live node is an ancestor of, or is, some
//!   root (ancestry over the nodes' own `inputs()` edges). No silently
//!   dead subgraph.
//! - **Ancestor-freedom** — no root is a strict ancestor of another
//!   (listing an extrude and its fillet would gather one body's
//!   material twice).
//!
//! Together they say something sharper than either alone: **the root
//! SET is exactly the DAG's sink set.** (Ancestor-freedom forbids a
//! root with a live consumer, since that consumer's own root would
//! have the first root as a strict ancestor; coverage forces every
//! sink to be listed, since a sink is an ancestor of nothing but
//! itself.) The list therefore adds exactly one thing the graph does
//! not carry — the product's solid ORDER — and that is why the order
//! is semantic and why a reorder moves the document's content pin.
//!
//! That equivalence is also what makes strictness burden-free: the
//! maintenance rules below are the sink set's own bookkeeping, so the
//! invariant-violating states are unreachable through `apply`, and
//! the shared save/load validator is a backstop rather than the
//! mechanism.

use crate::doc::Doc;
use crate::node::RecipeNodeId;

/// A product-root invariant violation. One type, two doors: the edit
/// layer wraps it in [`crate::EditError`], the persistence validator
/// in [`crate::SnapshotError`] — a single implementation of the
/// invariant, so the two doors cannot drift.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RootFault {
    /// A root entry does not name a live node.
    NotLive {
        /// The offending entry.
        root: RecipeNodeId,
    },
    /// A node appears twice in the root list.
    Duplicate {
        /// The repeated entry.
        root: RecipeNodeId,
    },
    /// One root is a strict ancestor of another: the product would
    /// gather the same material twice. Both are named.
    Ancestor {
        /// The root upstream (its material also reaches `descendant`).
        ancestor: RecipeNodeId,
        /// The root downstream of it.
        descendant: RecipeNodeId,
    },
    /// A live node reaches no root — a silently dead subgraph.
    Uncovered {
        /// The first uncovered node, in document order.
        node: RecipeNodeId,
    },
}

impl core::fmt::Display for RootFault {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::NotLive { root } => {
                write!(f, "product root {} is not a live node", root.0)
            }
            Self::Duplicate { root } => {
                write!(f, "product root {} is listed twice", root.0)
            }
            Self::Ancestor {
                ancestor,
                descendant,
            } => write!(
                f,
                "product root {} is an ancestor of product root {} — \
                 the product would gather its material twice",
                ancestor.0, descendant.0
            ),
            Self::Uncovered { node } => write!(
                f,
                "node {} reaches no product root — it would contribute \
                 to nothing",
                node.0
            ),
        }
    }
}

impl core::error::Error for RootFault {}

/// The strict ancestors of `from`, visited depth-first over `inputs()`
/// in deterministic order; `visit` sees each reached node once.
fn walk_strict_ancestors<P: crate::ProfilePayload>(
    doc: &Doc<P>,
    from: RecipeNodeId,
    seen: &mut std::collections::BTreeSet<RecipeNodeId>,
    mut visit: impl FnMut(RecipeNodeId) -> Result<(), RootFault>,
) -> Result<(), RootFault> {
    let mut stack: Vec<RecipeNodeId> = doc.node(from).map(|n| n.inputs()).unwrap_or_default();
    while let Some(id) = stack.pop() {
        if !seen.insert(id) {
            continue;
        }
        visit(id)?;
        if let Some(node) = doc.node(id) {
            stack.extend(node.inputs());
        }
    }
    Ok(())
}

/// Checks both root invariants plus the list's own well-formedness
/// (module docs). Deterministic: faults are reported in list order,
/// then document order.
///
/// # Errors
///
/// The first [`RootFault`] found.
pub(crate) fn check<P: crate::ProfilePayload>(doc: &Doc<P>) -> Result<(), RootFault> {
    let mut listed = std::collections::BTreeSet::new();
    for &root in &doc.roots {
        if doc.node(root).is_none() {
            return Err(RootFault::NotLive { root });
        }
        if !listed.insert(root) {
            return Err(RootFault::Duplicate { root });
        }
    }
    // Ancestor-freedom, per root, over its own strict-ancestor cone.
    // The `seen` set is per root: a node shared by two cones must be
    // re-walked only if it could be a root, and a root in ANY cone is
    // already the fault — so one shared set would be wrong here.
    for &root in &doc.roots {
        let mut seen = std::collections::BTreeSet::new();
        walk_strict_ancestors(doc, root, &mut seen, |id| {
            if listed.contains(&id) {
                Err(RootFault::Ancestor {
                    ancestor: id,
                    descendant: root,
                })
            } else {
                Ok(())
            }
        })?;
    }
    // Coverage: the roots' ancestor-or-self cone must be every node.
    let mut covered = listed.clone();
    for &root in &doc.roots {
        let mut seen = std::collections::BTreeSet::new();
        walk_strict_ancestors(doc, root, &mut seen, |_| Ok(()))?;
        covered.extend(seen);
    }
    if let Some(&node) = doc.order.iter().find(|id| !covered.contains(id)) {
        return Err(RootFault::Uncovered { node });
    }
    Ok(())
}

/// The D-3 maintenance for an accepted `InsertNode`, applied AFTER the
/// node is live: a node consuming no current root APPENDS (it is a
/// fresh sink and nothing else changed); a node consuming current
/// roots REPLACES them at the earliest consumed position (tip
/// transfer), because those roots just stopped being sinks. Consumed
/// non-root inputs change nothing — they were not sinks either way.
pub(crate) fn on_insert<P: crate::ProfilePayload>(
    doc: &mut Doc<P>,
    id: RecipeNodeId,
    inputs: &[RecipeNodeId],
) {
    let Some(at) = doc.roots.iter().position(|r| inputs.contains(r)) else {
        doc.roots.push(id);
        return;
    };
    // Entries before `at` are all retained by construction, so `at` is
    // still the earliest consumed root's slot after the retain.
    doc.roots.retain(|r| !inputs.contains(r));
    doc.roots.insert(at, id);
}

/// The D-3 maintenance for an accepted `SetMembers`, applied AFTER the
/// rewritten node is live.
///
/// The other two maintainers can splice, because an insert and a
/// delete each move the sink set in ONE direction. This edit moves it
/// both ways at once — a member the new list dropped may have become a
/// sink, a member it added may have stopped being one — so the answer
/// is recomputed instead: the root set IS the sink set (D-2's coverage
/// plus ancestor-freedom leave no other set possible, since a sink can
/// be covered only by itself and a non-sink is an ancestor of the sink
/// below it). Existing roots keep their order, and nodes the rewrite
/// orphaned join at the end in document order — the edit vacates no
/// position for them to take.
pub(crate) fn on_set_members<P: crate::ProfilePayload>(doc: &mut Doc<P>) {
    let sinks: Vec<RecipeNodeId> = doc
        .order
        .iter()
        .copied()
        .filter(|x| is_sink(doc, *x))
        .collect();
    let mut kept: Vec<RecipeNodeId> = doc
        .roots
        .iter()
        .copied()
        .filter(|r| sinks.contains(r))
        .collect();
    let fresh: Vec<RecipeNodeId> = sinks.into_iter().filter(|s| !kept.contains(s)).collect();
    kept.extend(fresh);
    doc.roots = kept;
}

/// The D-3 maintenance for an accepted `DeleteNode`, applied AFTER the
/// node is gone: deleting a root re-roots the direct inputs that its
/// departure turned into sinks, in DOCUMENT order, expanding at the
/// deleted root's list position. Deleting a non-root touches nothing
/// (and cannot happen through `apply`: a non-root has a live consumer,
/// which is the dangling refusal).
pub(crate) fn on_delete<P: crate::ProfilePayload>(
    doc: &mut Doc<P>,
    id: RecipeNodeId,
    inputs: &[RecipeNodeId],
) {
    let Some(at) = doc.roots.iter().position(|&r| r == id) else {
        return;
    };
    let orphans: Vec<RecipeNodeId> = doc
        .order
        .iter()
        .copied()
        .filter(|x| inputs.contains(x))
        .filter(|x| is_sink(doc, *x))
        .collect();
    doc.roots.splice(at..=at, orphans);
}

/// **Is this node a sink** — is it an input to nothing live?
///
/// One home for the predicate both maintainers above ask, so "what
/// makes a node a root" is answered in one place: D-2's coverage plus
/// ancestor-freedom make the root set exactly the sink set, and a
/// maintainer that computed sink-hood its own way could drift from
/// that identity without anything noticing.
///
/// Linear in the document per call, so the recomputing maintainer is
/// quadratic in node count. Fine at the sizes this kernel authors
/// (the die, its largest document, is 32 nodes); an incremental
/// consumer index is the fix if it ever is not.
fn is_sink<P: crate::ProfilePayload>(doc: &Doc<P>, id: RecipeNodeId) -> bool {
    !doc.nodes.values().any(|n| n.inputs().contains(&id))
}
