//! **The round-trip comparator** (A4: "inline-of-split returns the
//! document split was given, up to node ids").
//!
//! Two documents are the same up to node ids when one node map carries
//! the first onto the second: every live node maps to a live node whose
//! payload is the first's rewritten through the map — gauge references,
//! offsets, mate alignments and heads included, by the remapping split
//! and inline carry nodes with (`editor_core::test_support::remap_node`)
//! — the root lists agree as sets, and the parameters, labels and ε
//! agree. A refactoring's round trip composes its two outcomes' maps
//! ([`composed`]).

use std::collections::BTreeSet;

use editor_core::{InlineOutcome, Node, NodeMap, ProfileDoc, RecipeNodeId, SplitOutcome, StepMap};

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

/// **`a` and `b` are one document up to node ids under `map`**; the
/// error lists every disagreement, one line each.
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
    let image = |id: RecipeNodeId| map.get(&id).copied();
    let mut covered = BTreeSet::new();
    for &id in a.order() {
        let Some(node) = a.node(id) else { continue };
        let Some(to) = image(id) else {
            problems.push(format!("{id:?} has no image"));
            continue;
        };
        covered.insert(to);
        match (
            editor_core::test_support::remap_node(node, map, steps),
            b.node(to),
        ) {
            (Ok(want), Some(got)) if &want == got => {}
            (Ok(want), got) => {
                problems.push(format!("{id:?} -> {to:?}: want {want:?}, got {got:?}"));
            }
            (Err(miss), _) => problems.push(format!("{id:?} does not remap: {miss}")),
        }
        if a.label(id) != b.label(to) {
            problems.push(format!(
                "{id:?} -> {to:?}: label {:?} vs {:?}",
                a.label(id),
                b.label(to)
            ));
        }
    }
    for &id in b.order() {
        if b.node(id).is_some() && !covered.contains(&id) {
            problems.push(format!("{id:?} in the second has no preimage"));
        }
    }
    let roots_a: BTreeSet<Option<RecipeNodeId>> = a.roots().iter().map(|&r| image(r)).collect();
    let roots_b: BTreeSet<Option<RecipeNodeId>> = b.roots().iter().map(|&r| Some(r)).collect();
    if roots_a != roots_b {
        problems.push(format!("roots {roots_a:?} vs {roots_b:?}"));
    }
    let names = |d: &ProfileDoc| d.params().keys().cloned().collect::<Vec<_>>();
    if names(a) != names(b)
        || a.params()
            .iter()
            .any(|(k, v)| !b.params().get(k).is_some_and(|w| v.bit_eq(w)))
    {
        problems.push(format!("parameters {:?} vs {:?}", a.params(), b.params()));
    }
    if a.epsilon().to_bits() != b.epsilon().to_bits() {
        problems.push(format!("ε {} vs {}", a.epsilon(), b.epsilon()));
    }
    if problems.is_empty() {
        Ok(())
    } else {
        Err(problems.join("\n"))
    }
}
