//! **The round-trip comparator** (A4: "inline-of-split returns the
//! document split was given, up to node ids").
//!
//! Two documents are the same up to node ids under a node map and a
//! step map when:
//! - **the map is a bijection** of the live nodes: injective, every
//!   live node of the first has a live image, and every live node of
//!   the second a preimage;
//! - **each payload is the first's with its ids replaced** — read field
//!   by field off the node's serialized form, independently of the
//!   remapping the refactorings carry nodes with: every integer that is
//!   a node id of the first must read as its image, every integer that
//!   is a step id as the step map's image, and every other field is
//!   equal. Gauge references, offsets, gauge placements, mate
//!   alignments and heads, and a profile's step ids are all fields;
//! - **the root sets agree** through the map, and the parameters,
//!   labels and ε agree;
//! - **each placement group keeps its document order.** Order is
//!   semantic within a group (its root is its earliest member carrying
//!   an offset), so the images of each group's members read in the
//!   second document's order must follow the first's. Order across
//!   groups and across node kinds is not compared: a split collapses
//!   its cut into a part and inline appends what it splices, and the
//!   carry moves a gauge ahead of what sits on it; none of those moves
//!   a member within its group.
//!
//! **What it does not undo**: a name whose canonical form orders its
//! parts by id (a union's sides) is re-sorted by the remapping, and
//! this reading does not re-sort it, so such a name reads as a
//! mismatch — loudly, never as a false agreement. A literal integer
//! that equals a node or step id of the first would be read as that
//! id; ids are minted far from the small literals a payload holds.
//!
//! A refactoring's round trip composes its two outcomes' maps
//! ([`composed`]).

use std::collections::{BTreeMap, BTreeSet};

use editor_core::{InlineOutcome, Node, NodeMap, ProfileDoc, RecipeNodeId, SplitOutcome, StepMap};
use serde_json::Value;

/// **The split document's ids carried through split then inline**: a
/// kept node keeps its id (the remainder is the document edited), and a
/// cut node goes to the inline's image of its image in the part; the
/// same for the profile steps of each.
///
/// # Panics
///
/// When the inline did not carry a node or step the split carried.
pub fn composed(
    doc: &ProfileDoc,
    split: &SplitOutcome,
    inline: &InlineOutcome,
) -> (NodeMap, StepMap) {
    let mut nodes = NodeMap::new();
    let mut steps = StepMap::new();
    for &id in doc.order() {
        let to = match split.node_map.get(&id) {
            Some(in_part) => *inline
                .node_map
                .get(in_part)
                .unwrap_or_else(|| panic!("the inline carried part node {in_part:?}")),
            None => id,
        };
        nodes.insert(id, to);
        if let Some(Node::Profile(p)) = doc.node(id) {
            for &s in p.ids.iter().flatten() {
                let to = match split.step_map.get(&s) {
                    Some(in_part) => *inline
                        .step_map
                        .get(in_part)
                        .unwrap_or_else(|| panic!("the inline carried part step {in_part:?}")),
                    None => s,
                };
                steps.insert(s, to);
            }
        }
    }
    (nodes, steps)
}

/// **The identity maps of `doc`**: every live node and every profile
/// step onto itself — the maps under which a document is itself.
pub fn identity(doc: &ProfileDoc) -> (NodeMap, StepMap) {
    let mut nodes = NodeMap::new();
    let mut steps = StepMap::new();
    for &id in doc.order() {
        let Some(node) = doc.node(id) else { continue };
        nodes.insert(id, id);
        if let Node::Profile(p) = node {
            steps.extend(p.ids.iter().flatten().map(|&s| (s, s)));
        }
    }
    (nodes, steps)
}

/// Field by field: `b` is `a` with every node id the map holds read as
/// its image and every step id as its step image. Each disagreement is
/// pushed with the path it sits at.
fn same_value(
    a: &Value,
    b: &Value,
    ids: &BTreeMap<u64, u64>,
    steps: &BTreeMap<u64, u64>,
    at: &str,
    out: &mut Vec<String>,
) {
    match (a, b) {
        (Value::Object(x), Value::Object(y)) => {
            let keys = |m: &serde_json::Map<String, Value>| m.keys().cloned().collect::<Vec<_>>();
            if keys(x) != keys(y) {
                out.push(format!("{at}: fields {:?} vs {:?}", keys(x), keys(y)));
                return;
            }
            for (k, v) in x {
                same_value(v, &y[k], ids, steps, &format!("{at}.{k}"), out);
            }
        }
        (Value::Array(x), Value::Array(y)) => {
            if x.len() != y.len() {
                out.push(format!("{at}: {} entries vs {}", x.len(), y.len()));
                return;
            }
            for (i, (v, w)) in x.iter().zip(y).enumerate() {
                same_value(v, w, ids, steps, &format!("{at}[{i}]"), out);
            }
        }
        (Value::Number(x), Value::Number(y)) => {
            let want = match x.as_u64() {
                Some(n) if ids.contains_key(&n) => Value::from(ids[&n]),
                Some(n) if steps.contains_key(&n) => Value::from(steps[&n]),
                _ => a.clone(),
            };
            if &want != b {
                out.push(format!("{at}: want {want}, got {y} (first held {x})"));
            }
        }
        _ => {
            if a != b {
                out.push(format!("{at}: {a} vs {b}"));
            }
        }
    }
}

/// **`a` and `b` are one document up to node ids under `map` and
/// `steps`** (module docs); the error lists every disagreement, one
/// line each, opening with the check that found it.
///
/// # Errors
///
/// The disagreements.
pub fn same_up_to_ids(
    a: &ProfileDoc,
    b: &ProfileDoc,
    map: &NodeMap,
    steps: &StepMap,
) -> Result<(), String> {
    let mut problems = Vec::new();
    let live = |d: &ProfileDoc| -> Vec<RecipeNodeId> {
        d.order()
            .iter()
            .copied()
            .filter(|&id| d.node(id).is_some())
            .collect()
    };
    let mut seen: BTreeMap<RecipeNodeId, RecipeNodeId> = BTreeMap::new();
    for (&from, &to) in map {
        if let Some(other) = seen.insert(to, from) {
            problems.push(format!(
                "injective: {other:?} and {from:?} both map to {to:?}"
            ));
        }
    }
    let ids: BTreeMap<u64, u64> = map.iter().map(|(k, v)| (k.0, v.0)).collect();
    let step_ids: BTreeMap<u64, u64> = steps.iter().map(|(k, v)| (k.0, v.0)).collect();
    let mut covered = BTreeSet::new();
    for id in live(a) {
        let Some(&to) = map.get(&id) else {
            problems.push(format!("image: {id:?} has none"));
            continue;
        };
        covered.insert(to);
        let (Some(x), Some(y)) = (a.node(id), b.node(to)) else {
            problems.push(format!("image: {id:?} -> {to:?}, which is not live"));
            continue;
        };
        let json = |n: &Node<editor_core::ProfileProgram>| {
            serde_json::to_value(n).expect("a node serializes")
        };
        let mut out = Vec::new();
        same_value(&json(x), &json(y), &ids, &step_ids, "", &mut out);
        problems.extend(
            out.into_iter()
                .map(|p| format!("payload: {id:?} -> {to:?}{p}")),
        );
        if a.label(id) != b.label(to) {
            problems.push(format!(
                "label: {id:?} -> {to:?}: {:?} vs {:?}",
                a.label(id),
                b.label(to)
            ));
        }
    }
    for id in live(b) {
        if !covered.contains(&id) {
            problems.push(format!("preimage: {id:?} has none"));
        }
    }
    let roots_a: BTreeSet<Option<RecipeNodeId>> =
        a.roots().iter().map(|r| map.get(r).copied()).collect();
    let roots_b: BTreeSet<Option<RecipeNodeId>> = b.roots().iter().map(|&r| Some(r)).collect();
    if roots_a != roots_b {
        problems.push(format!("roots: {roots_a:?} vs {roots_b:?}"));
    }
    let position: BTreeMap<RecipeNodeId, usize> = b
        .order()
        .iter()
        .enumerate()
        .map(|(i, &id)| (id, i))
        .collect();
    for group in editor_core::groups(a) {
        let at: Vec<Option<usize>> = group
            .iter()
            .map(|m| map.get(m).and_then(|to| position.get(to).copied()))
            .collect();
        if at.windows(2).any(|w| w[0] >= w[1]) {
            problems.push(format!(
                "order: the group {group:?} reads at positions {at:?} in the second"
            ));
        }
    }
    let names = |d: &ProfileDoc| d.params().keys().cloned().collect::<Vec<_>>();
    if names(a) != names(b)
        || a.params()
            .iter()
            .any(|(k, v)| !b.params().get(k).is_some_and(|w| v.bit_eq(w)))
    {
        problems.push(format!("parameters: {:?} vs {:?}", a.params(), b.params()));
    }
    if a.epsilon().to_bits() != b.epsilon().to_bits() {
        problems.push(format!("epsilon: {} vs {}", a.epsilon(), b.epsilon()));
    }
    if problems.is_empty() {
        Ok(())
    } else {
        Err(problems.join("\n"))
    }
}
