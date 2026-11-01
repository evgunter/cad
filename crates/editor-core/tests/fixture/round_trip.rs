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
//!   `StepId(n)` the step map holds as its image, every `ConstraintId(n)`
//!   of a mapped placement's bundle as the id at its place in its
//!   image's bundle, and every other field is equal. A placement's
//!   constraints, their alignments and sides, and a profile's step ids
//!   are all fields;
//! - **the named variables, labels and ε agree**, a named variable
//!   matched by its name and a definition — a scalar's or a pose's —
//!   read with the ids it reads as their images (a named variable's by
//!   its name, an anonymous one's by its place in the definition that
//!   reads it);
//! - **the placements keep their document order**: the product's copies
//!   are gathered in it. Order across other node kinds is not compared:
//!   a split collapses its cut into a part and inline appends what it
//!   splices.
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

/// **An inline's result with its placement flattened** ([`editor_core::flatten`]):
/// the document an inline of a split's instance standing at the identity
/// gives back, its new space folded into the instance's target (A4).
///
/// # Panics
///
/// When the placement does not flatten.
pub fn flat(back: &editor_core::InlineOutcome) -> editor_core::ProfileDoc {
    editor_core::flatten(&back.doc, back.placement, geom_core::Tol::witness())
        .unwrap_or_else(|e| panic!("the inline's identity placement flattens: {e:?}"))
        .doc
}

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
/// by its name, an operation's output by its port on the node's image
/// under `map`, and one a matched definition reads — anonymous ones
/// among them — by its place among that definition's reads, followed
/// to a fixed point. The two documents mint their own ids; a name, a
/// port or a definition is what a reader reads.
fn var_images(a: &ProfileDoc, b: &ProfileDoc, map: &NodeMap) -> BTreeMap<MintId, MintId> {
    let mut images: BTreeMap<MintId, MintId> = a
        .var_names()
        .iter()
        .filter_map(|(id, name)| Some((id.0, b.var_named(name.as_str())?.0)))
        .collect();
    for (&from, &to) in map {
        for port in 0..=u8::MAX {
            let (Some(x), Some(y)) = (a.output(from, port), b.output(to, port)) else {
                break;
            };
            images.entry(x.0).or_insert(y.0);
        }
    }
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

/// **Each constraint of a mapped placement onto the one at its place
/// in its image's bundle**: the carry adds a bundle constraint by
/// constraint, in order, and mints each id afresh.
fn constraint_images(a: &ProfileDoc, b: &ProfileDoc, map: &NodeMap) -> BTreeMap<MintId, MintId> {
    let bundle = |d: &ProfileDoc, id: RecipeNodeId| match d.node(id) {
        Some(Node::Place { constraints, .. } | Node::InstantiatePart { constraints, .. }) => {
            constraints.iter().map(|c| c.id.0).collect()
        }
        _ => Vec::new(),
    };
    map.iter()
        .flat_map(|(&from, &to)| bundle(a, from).into_iter().zip(bundle(b, to)))
        .collect()
}

/// `text` with every `RecipeNodeId(id)` the map holds read as its image,
/// every `StepId(id)` the step map holds as its image, every
/// `ConstraintId(id)` the constraint map holds as its image, and every
/// `VarId(id)` the variable map holds as its image.
fn renamed(
    text: &str,
    (ids, constraints): (&BTreeMap<MintId, MintId>, &BTreeMap<MintId, MintId>),
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
        } else if head.ends_with("ConstraintId(MintId { ordinal: ") {
            Some(constraints)
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

/// Field by field: `b` is `a` with its node, constraint, step and
/// named-variable ids read as their images, read off the node's derived
/// rendering as written ([`Node::written`]: each anonymous variable its
/// value or definition, so the two documents' own minted ids do not
/// enter), which spells every field (a placement's constraints, their
/// alignments and sides, a profile's step ids); the first disagreement
/// is reported in context.
fn same_payload(
    a: &editor_core::AuthoredNode,
    b: &editor_core::AuthoredNode,
    ids: (&BTreeMap<MintId, MintId>, &BTreeMap<MintId, MintId>),
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
/// The map is the one the refactorings carried nodes with, and the
/// documents are live. Two SAVED bodies with no map between them are
/// `tests/wire/up_to_ids.rs`'s, which builds its map from the walk.
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
    let var_ids = var_images(a, b, map);
    let constraint_ids = constraint_images(a, b, map);
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
            (&ids, &constraint_ids),
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
    let position: BTreeMap<RecipeNodeId, usize> =
        b.ids().iter().enumerate().map(|(i, &id)| (id, i)).collect();
    // The placements keep their order: the product's copies are
    // gathered in it.
    let placements = a.placements();
    let at: Vec<Option<usize>> = placements
        .iter()
        .map(|m| map.get(m).and_then(|to| position.get(to).copied()))
        .collect();
    if at.windows(2).any(|w| w[0] >= w[1]) {
        problems.push(format!(
            "order: the placements {placements:?} read at positions {at:?} in the second"
        ));
    }
    // Variables are compared by name, a definition as written (each
    // anonymous variable it reads as what it holds) with the ids it
    // reads read as their images: the two documents mint their own
    // ids, and a name is what a reader reads.
    let vars = |d: &ProfileDoc, through: Option<&BTreeMap<MintId, MintId>>| {
        d.var_names()
            .iter()
            .filter_map(|(id, name)| {
                let held = d.var(*id)?;
                let shown = match held.def() {
                    editor_core::var::VarDef::Defined(defined) => {
                        format!("{:?} {:?}", held.kind(), d.written(defined))
                    }
                    editor_core::var::VarDef::Pose(def) => {
                        let written = def.try_map(
                            &mut |_, &read| {
                                Ok::<_, std::convert::Infallible>(d.written_operand(read))
                            },
                            &mut |slot, &scalar| {
                                Ok(editor_core::Formula::from(d.written(
                                    &editor_core::Expr::var(scalar, slot.dimension()),
                                )))
                            },
                        );
                        let Ok(written) = written;
                        format!("{:?} {written:?}", held.kind())
                    }
                    _ => format!("{held:?}"),
                };
                let shown = match through {
                    Some(vars) => renamed(&shown, (&ids, &constraint_ids), &step_ids, vars),
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
