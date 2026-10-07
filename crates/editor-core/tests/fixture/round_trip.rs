//! **The round-trip comparator** (A4: "inline-of-split returns the
//! document split was given, up to minted ids and that one
//! regrouping").
//!
//! Two documents are the same up to minted ids under a node map and a
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
//!   (`roots.rs`) — and the named variables, labels and ε agree, a
//!   named variable matched by its name and a definition read with the
//!   ids it reads as their images (a named variable's by its name, an
//!   anonymous one's by its place in the definition that reads it). By A10's
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

/// **Each variable of `a` matched to its image in `b`**: a named one
/// by its name, and one a matched definition reads — anonymous ones
/// among them — by its place among that definition's reads, followed
/// to a fixed point. The two documents mint their own ids; a name or a
/// definition is what a reader reads.
fn var_images(a: &ProfileDoc, b: &ProfileDoc) -> BTreeMap<MintId, MintId> {
    let mut images: BTreeMap<MintId, MintId> = a
        .var_names()
        .iter()
        .filter_map(|(id, name)| Some((id.0, b.var_named(name.as_str())?.0)))
        .collect();
    let reads = |d: &ProfileDoc, id: MintId| {
        let mut out = Vec::new();
        if let Some(expr) = d
            .var(editor_core::VarId(id))
            .and_then(|v| v.def().defined())
        {
            expr.var_reads(&mut out);
        }
        out.into_iter().map(|(read, _)| read.0).collect::<Vec<_>>()
    };
    loop {
        let mut found = Vec::new();
        for (&x, &y) in &images {
            let (rx, ry) = (reads(a, x), reads(b, y));
            if rx.len() == ry.len() {
                found.extend(
                    rx.into_iter()
                        .zip(ry)
                        .filter(|(rx, _)| !images.contains_key(rx)),
                );
            }
        }
        if found.is_empty() {
            return images;
        }
        for (x, y) in found {
            images.entry(x).or_insert(y);
        }
    }
}

/// `text` with every `RecipeNodeId(id)` the map holds read as its image,
/// every `StepId(id)` the step map holds as its image, and every
/// `VarId(id)` the variable map holds as its image.
fn renamed(
    text: &str,
    ids: &BTreeMap<MintId, MintId>,
    steps: &BTreeMap<MintId, MintId>,
    vars: &BTreeMap<MintId, MintId>,
) -> String {
    const OPEN: &str = "Id(MintId { ordinal: ";
    let mut out = String::with_capacity(text.len());
    let mut rest = text;
    while let Some(at) = rest.find(OPEN) {
        let (head, tail) = rest.split_at(at + OPEN.len());
        let table = if head.ends_with(&format!("RecipeNode{OPEN}")) {
            Some(ids)
        } else if head.ends_with(&format!("Step{OPEN}")) {
            Some(steps)
        } else if head.ends_with(&format!("Var{OPEN}")) {
            Some(vars)
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

/// Field by field: `b` is `a` with its node, step and named-variable ids
/// read as their images, read off the node's derived rendering as
/// written ([`Node::written`]: each anonymous variable its value or
/// definition, so the two documents' own minted ids do not enter),
/// which spells every field (gauge references, offsets, placements,
/// alignments, heads, a profile's step ids); the first disagreement is
/// reported in context.
fn same_payload(
    a: &editor_core::AuthoredNode,
    b: &editor_core::AuthoredNode,
    ids: &BTreeMap<MintId, MintId>,
    steps: &BTreeMap<MintId, MintId>,
    vars: &BTreeMap<MintId, MintId>,
    out: &mut Vec<String>,
) {
    let want = renamed(&format!("{a:?}"), ids, steps, vars);
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
    let var_ids = var_images(a, b);
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
        same_payload(
            &x.written(a),
            &y.written(b),
            &ids,
            &step_ids,
            &var_ids,
            &mut out,
        );
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
    // Variables are compared by name, a definition with the ids it
    // reads read as their images: the two documents mint their own
    // ids, and a name is what a reader reads.
    let vars = |d: &ProfileDoc, through: Option<&BTreeMap<MintId, MintId>>| {
        d.var_names()
            .iter()
            .filter_map(|(id, name)| {
                let shown = format!("{:?}", d.var(*id)?);
                let shown = match through {
                    Some(vars) => renamed(&shown, &ids, &step_ids, vars),
                    None => shown,
                };
                Some((name.clone(), shown))
            })
            .collect::<BTreeMap<_, _>>()
    };
    let (va, vb) = (vars(a, Some(&var_ids)), vars(b, None));
    if va != vb {
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
