//! **The round-trip comparator** (A4: "inline-of-split returns the
//! document split was given, up to node ids and that one regrouping").
//!
//! Two documents are the same up to node ids under a node map and a
//! step map when:
//! - **the map is a bijection** of the live nodes: injective, every
//!   live node of the first has a live image, and every live node of
//!   the second a preimage;
//! - **each payload is the first's with its ids replaced** — read field
//!   by field off the node's derived rendering (`Debug`),
//!   independently of the remapping the refactorings carry nodes with:
//!   every `RecipeNodeId(n)` the map holds must read as its image, every
//!   `StepId(n)` the step map holds as its image, and every other field
//!   is equal. Gauge references, offsets, gauge placements, mate
//!   alignments and heads, and a profile's step ids are all fields;
//! - **the root lists agree** through the map, in order — the order is
//!   the product's solid order, semantic and in the content pin
//!   (`roots.rs`) — and the parameters, labels and ε agree. By A10's
//!   replacement rule split's instance goes where the first cut root
//!   was and inline splices the part's roots there, so a round trip
//!   agrees in order exactly when the cut's roots are adjacent in the
//!   list; a cut a kept root separates comes back regrouped, and this
//!   check reports that regrouping as a `roots` line;
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
//! mismatch — loudly, never as a false agreement. An id the maps do
//! not hold reads as itself.
//!
//! A refactoring's round trip composes its two outcomes' maps
//! ([`composed`]).

use std::collections::{BTreeMap, BTreeSet};

use editor_core::{
    InlineOutcome, MintId, Node, NodeMap, ProfileDoc, RecipeNodeId, SplitOutcome, StepMap,
};

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
    for id in doc.ids() {
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
    for id in doc.ids() {
        let Some(node) = doc.node(id) else { continue };
        nodes.insert(id, id);
        if let Node::Profile(p) = node {
            steps.extend(p.ids.iter().flatten().map(|&s| (s, s)));
        }
    }
    (nodes, steps)
}

/// `text` with every `RecipeNodeId(..)` the map holds read as its image
/// and every `StepId(..)` the step map holds as its image, each id as
/// `Debug` spells it.
fn renamed(text: &str, ids: &BTreeMap<MintId, MintId>, steps: &BTreeMap<MintId, MintId>) -> String {
    const OPEN: &str = "Id(MintId { ordinal: ";
    let mut out = String::with_capacity(text.len());
    let mut rest = text;
    while let Some(at) = rest.find(OPEN) {
        let (head, tail) = rest.split_at(at + OPEN.len());
        let table = if head.ends_with(&format!("RecipeNode{OPEN}")) {
            Some(ids)
        } else if head.ends_with(&format!("Step{OPEN}")) {
            Some(steps)
        } else {
            None
        };
        let Some(end) = tail.find(" })") else {
            out.push_str(head);
            rest = tail;
            continue;
        };
        let read = tail[..end]
            .split_once(", digest: ")
            .and_then(|(o, d)| Some(MintId::new(o.parse().ok()?, d.parse().ok()?)));
        match (read, table) {
            (Some(id), Some(table)) => {
                let to = table.get(&id).copied().unwrap_or(id);
                out.push_str(&head[..head.len() - "MintId { ordinal: ".len()]);
                out.push_str(&format!("{to:?}"));
                rest = &tail[end + " }".len()..];
            }
            _ => {
                out.push_str(head);
                rest = tail;
            }
        }
    }
    out.push_str(rest);
    out
}

/// Field by field: `b` is `a` with its node and step ids read as their
/// images, read off the node's derived rendering, which spells every
/// field (gauge references, offsets, placements, alignments, heads,
/// a profile's step ids); the first disagreement is reported in context.
fn same_payload(
    a: &editor_core::Node<editor_core::ProfileProgram>,
    b: &editor_core::Node<editor_core::ProfileProgram>,
    ids: &BTreeMap<MintId, MintId>,
    steps: &BTreeMap<MintId, MintId>,
    out: &mut Vec<String>,
) {
    let want = renamed(&format!("{a:?}"), ids, steps);
    let got = format!("{b:?}");
    if want == got {
        return;
    }
    // The first disagreement, with the field context around it.
    let at = want
        .bytes()
        .zip(got.bytes())
        .position(|(x, y)| x != y)
        .unwrap_or(want.len().min(got.len()));
    let window = |s: &str| {
        let from = s.floor_char_boundary(at.saturating_sub(60));
        let to = s.ceil_char_boundary((at + 60).min(s.len()));
        s[from..to].to_owned()
    };
    out.push(format!(
        ": want …{}…, got …{}…",
        window(&want),
        window(&got)
    ));
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
        d.ids()
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
    let ids: BTreeMap<MintId, MintId> = map.iter().map(|(k, v)| (k.0, v.0)).collect();
    let step_ids: BTreeMap<MintId, MintId> = steps.iter().map(|(k, v)| (k.0, v.0)).collect();
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
        let mut out = Vec::new();
        same_payload(x, y, &ids, &step_ids, &mut out);
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
    let roots_a: Vec<Option<RecipeNodeId>> =
        a.roots().iter().map(|r| map.get(r).copied()).collect();
    let roots_b: Vec<Option<RecipeNodeId>> = b.roots().iter().map(|&r| Some(r)).collect();
    if roots_a != roots_b {
        problems.push(format!("roots: {roots_a:?} vs {roots_b:?}"));
    }
    let position: BTreeMap<RecipeNodeId, usize> =
        b.ids().iter().enumerate().map(|(i, &id)| (id, i)).collect();
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
    // Variables are compared by name: the two documents mint their own
    // ids, and a name is what a reader reads.
    let vars = |d: &ProfileDoc| {
        d.var_names()
            .iter()
            .filter_map(|(id, name)| Some((name.clone(), d.var(*id)?.clone())))
            .collect::<BTreeMap<_, _>>()
    };
    let (va, vb) = (vars(a), vars(b));
    if va.keys().ne(vb.keys())
        || va
            .iter()
            .any(|(k, v)| !vb.get(k).is_some_and(|w| v.bit_eq(w)))
    {
        problems.push(format!("variables: {va:?} vs {vb:?}"));
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
