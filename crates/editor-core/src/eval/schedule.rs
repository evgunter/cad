//! Deterministic scheduling of the recipe DAG (spec D2): the
//! evaluation `order` is a pure function of the DAG — Kahn's algorithm
//! with a min-heap of ready nodes, tiebreak `RecipeNodeId` ascending.
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
    /// Topological order, Kahn min-id (spec D2's documented tiebreak).
    pub order: Vec<RecipeNodeId>,
    /// Longest-path levels; concatenated they are ALSO a topological
    /// order (level ascending, id ascending within a level).
    pub levels: Vec<Vec<RecipeNodeId>>,
    /// Nodes Kahn never released (in-cycle or downstream of one), id
    /// ascending.
    pub unschedulable: Vec<RecipeNodeId>,
}

pub(crate) fn schedule<P: crate::ProfilePayload>(doc: &Doc<P>) -> Schedule {
    // In-degree counts edges from inputs that EXIST in the doc; a
    // dangling input (unreachable through `apply`) simply contributes
    // no edge and the node fails later at operand lookup.
    let mut indegree: BTreeMap<RecipeNodeId, usize> = BTreeMap::new();
    let mut dependents: BTreeMap<RecipeNodeId, Vec<RecipeNodeId>> = BTreeMap::new();
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

    let mut ready: BinaryHeap<Reverse<RecipeNodeId>> = indegree
        .iter()
        .filter(|&(_, &d)| d == 0)
        .map(|(&id, _)| Reverse(id))
        .collect();
    let mut order = Vec::with_capacity(indegree.len());
    let mut level: BTreeMap<RecipeNodeId, usize> = BTreeMap::new();
    while let Some(Reverse(id)) = ready.pop() {
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
                    ready.push(Reverse(dep));
                }
            }
        }
    }

    let unschedulable: Vec<RecipeNodeId> = indegree
        .keys()
        .filter(|id| !level.contains_key(id))
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
    for lv in &mut levels {
        lv.sort_unstable();
    }

    Schedule {
        order,
        levels,
        unschedulable,
    }
}
