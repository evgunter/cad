//! Deterministic scheduling of the recipe DAG (spec D2): the
//! evaluation `order` is a pure function of the document — Kahn's
//! algorithm with a min-heap of ready nodes, ties broken by position in
//! [`Doc::order`], the recipe's presentation order.
//! The parallel path (spec D6) additionally groups nodes into
//! longest-path LEVELS: every node's inputs live in strictly earlier
//! levels, so a level's nodes are pairwise independent and may run as
//! one rayon idiom-1 indexed map. `order` is data, not schedule — the
//! two paths produce the same `Evaluation`.

use std::cmp::Reverse;
use std::collections::{BTreeMap, BinaryHeap};

use crate::doc::Doc;
use crate::node::RecipeNodeId;

/// The schedule: the D2 order, the D6 levels, and any unschedulable
/// leftover (cycle members and their descendants — unreachable through
/// `apply`, which only ever inserts back-references, but the evaluator
/// refuses them TYPED rather than trusting the door).
pub(crate) struct Schedule {
    /// Topological order, ties to the earlier node in [`Doc::order`].
    pub order: Vec<RecipeNodeId>,
    /// Longest-path levels; concatenated they are ALSO a topological
    /// order (level ascending, [`Doc::order`] within a level).
    pub levels: Vec<Vec<RecipeNodeId>>,
    /// Nodes Kahn never released (in-cycle or downstream of one), in
    /// [`Doc::order`].
    pub unschedulable: Vec<RecipeNodeId>,
}

pub(crate) fn schedule<P: crate::ProfilePayload>(doc: &Doc<P>) -> Schedule {
    // In-degree counts edges from inputs that EXIST in the doc; a
    // dangling input (unreachable through `apply`) simply contributes
    // no edge and the node fails later at operand lookup.
    let mut indegree: BTreeMap<RecipeNodeId, usize> = BTreeMap::new();
    let mut dependents: BTreeMap<RecipeNodeId, Vec<RecipeNodeId>> = BTreeMap::new();
    let position: BTreeMap<RecipeNodeId, usize> =
        doc.order().iter().enumerate().map(|(at, &id)| (id, at)).collect();
    for &id in doc.order() {
        let Some(node) = doc.node(id) else {
            continue; // doc.order() lists live nodes; defensive skip
        };
        let live_inputs: Vec<RecipeNodeId> = node
            .inputs()
            .into_iter()
            .filter(|i| doc.node(*i).is_some())
            .collect();
        indegree.insert(id, live_inputs.len());
        for input in live_inputs {
            dependents.entry(input).or_default().push(id);
        }
    }

    // Every key of `indegree` came from `doc.order()`, so every node
    // the heap sees has a position.
    let at = |id: RecipeNodeId| Reverse((position[&id], id));
    let mut ready: BinaryHeap<Reverse<(usize, RecipeNodeId)>> = indegree
        .iter()
        .filter(|&(_, &d)| d == 0)
        .map(|(&id, _)| at(id))
        .collect();
    let mut order = Vec::with_capacity(indegree.len());
    let mut level: BTreeMap<RecipeNodeId, usize> = BTreeMap::new();
    while let Some(Reverse((_, id))) = ready.pop() {
        order.push(id);
        let lvl = doc
            .node(id)
            .map(|node| {
                node.inputs()
                    .into_iter()
                    .filter_map(|i| level.get(&i).copied())
                    .max()
                    .map_or(0, |m| m + 1)
            })
            .unwrap_or(0);
        level.insert(id, lvl);
        for &dep in dependents.get(&id).into_iter().flatten() {
            if let Some(d) = indegree.get_mut(&dep) {
                *d -= 1;
                if *d == 0 {
                    ready.push(at(dep));
                }
            }
        }
    }

    let unschedulable: Vec<RecipeNodeId> = doc
        .order()
        .iter()
        .filter(|id| indegree.contains_key(id) && !level.contains_key(id))
        .copied()
        .collect();

    let mut levels: Vec<Vec<RecipeNodeId>> = Vec::new();
    for &id in doc.order() {
        if let Some(&lvl) = level.get(&id) {
            if levels.len() <= lvl {
                levels.resize_with(lvl + 1, Vec::new);
            }
            levels[lvl].push(id);
        }
    }

    Schedule {
        order,
        levels,
        unschedulable,
    }
}
