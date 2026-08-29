//! The feature tree: the GQ2 result DAG, as rows a panel can draw.
//!
//! # Failures are values, and this module renders no opinion of its
//! own
//!
//! GQ2's ratified codomain is a per-node result — `Ok`, `Failed(e)`,
//! `Poisoned { through }` — and the ratified error rule is that a
//! failure is a typed value the GUI renders, never a string invented
//! at the interaction layer. So a badge's message is
//! `NodeError`'s own `Display`, and a poisoned row's message is the
//! `Display` of the FAILED ANCESTOR's error, reached through the
//! shipped `Evaluation::node_error`. Nothing here composes a sentence
//! about what went wrong; the payload already knows.
//!
//! What this module does own is the *shape*: which rows exist, in
//! which order, at what indentation, and which of them the selection
//! is on.
//!
//! # Order and depth
//!
//! Rows follow `Evaluation::order` — the evaluation's own
//! deterministic topological order, which is a pure function of the
//! DAG — so the tree a user reads is the order the kernel evaluated
//! in. Depth is the longest input chain, which makes a node sit below
//! everything it consumes.

use std::collections::BTreeMap;

use pncad::document::{Doc, Evaluation, Node, NodeResult, ProfileProgram, RecipeNodeId};

/// A node's status, as the tree draws it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RowStatus {
    /// The node produced a value.
    Ok,
    /// The node's own operation failed. `message` is the typed
    /// error's own rendering.
    Failed {
        /// `NodeError`'s `Display`.
        message: String,
    },
    /// An ancestor failed, so this node was never run.
    Poisoned {
        /// The nearest failed ancestor.
        through: RecipeNodeId,
        /// That ancestor's error, rendered by the payload itself.
        /// `None` only if the evaluation's poison chain does not end
        /// at a failure — a broken invariant this reports as absence
        /// rather than papering over with an invented cause.
        message: Option<String>,
    },
    /// The node has no entry in this evaluation: past a cancelation's
    /// completed prefix, or never scheduled.
    Unevaluated,
}

impl RowStatus {
    /// A short badge label — the status axis alone, without the
    /// payload.
    pub fn badge(&self) -> &'static str {
        match self {
            Self::Ok => "ok",
            Self::Failed { .. } => "FAILED",
            Self::Poisoned { .. } => "POISONED",
            Self::Unevaluated => "—",
        }
    }

    /// The typed payload's message, when the status carries one.
    pub fn message(&self) -> Option<&str> {
        match self {
            Self::Ok | Self::Unevaluated => None,
            Self::Failed { message } => Some(message),
            Self::Poisoned { message, .. } => message.as_deref(),
        }
    }
}

/// One line of the feature tree.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TreeRow {
    /// The recipe node this row is.
    pub id: RecipeNodeId,
    /// The node's kind, as the vocabulary spells it.
    pub kind: &'static str,
    /// How far the node sits below the document's sources.
    pub depth: usize,
    /// Whether this node is one of the document's product roots.
    pub root: bool,
    /// What the evaluation said about it.
    pub status: RowStatus,
    /// A standing caveat about the NODE itself, independent of any
    /// run's status — today: a mate whose declared class carries no
    /// at-rest record ([`pncad::document::ClassAdmission`], the
    /// kernel's own reason). The admission verdict shown at the mate
    /// tool's commit persists here on the node's own row, so a
    /// committed `Tangent` is not a green row indistinguishable from
    /// a certifiable one.
    pub note: Option<String>,
}

/// The kind name of a recipe node — the node vocabulary's own
/// spelling, one arm per variant so a new node type cannot fall into a
/// wildcard and draw as something it is not.
pub fn node_kind(node: &Node<ProfileProgram>) -> &'static str {
    match node {
        Node::Profile(_) => "Profile",
        Node::Extrude { .. } => "Extrude",
        Node::Revolve { .. } => "Revolve",
        Node::Transform { .. } => "Transform",
        Node::Boolean { .. } => "Boolean",
        Node::Split { .. } => "Split",
        Node::Pattern { .. } => "Pattern",
        Node::PlacedUnion { .. } => "PlacedUnion",
        Node::Datum(_) => "Datum",
        Node::Declare { .. } => "Declare",
        Node::Fillet { .. } => "Fillet",
        Node::Chamfer { .. } => "Chamfer",
        Node::Loft { .. } => "Loft",
        Node::Sweep { .. } => "Sweep",
        Node::InstantiatePart { .. } => "InstantiatePart",
        Node::Mate { .. } => "Mate",
        Node::Measure { .. } => "Measure",
        Node::Assertion { .. } => "Assertion",
    }
}

/// The tree's rows for a document under an evaluation.
///
/// `evaluation` is optional because a session shows a tree before its
/// first result lands: every row is then [`RowStatus::Unevaluated`],
/// which is the honest reading rather than an optimistic `ok`.
pub fn rows(doc: &Doc<ProfileProgram>, evaluation: Option<&Evaluation<f64>>) -> Vec<TreeRow> {
    let order: Vec<RecipeNodeId> = match evaluation {
        Some(ev) => ev.order.clone(),
        None => doc.order().to_vec(),
    };
    let mut depths: BTreeMap<RecipeNodeId, usize> = BTreeMap::new();
    let roots = doc.roots();
    let mut rows = Vec::with_capacity(order.len());
    for id in order {
        let Some(node) = doc.node(id) else {
            continue;
        };
        // The order is topological, so every input already has its
        // depth; an input that does not (a node the order omits) reads
        // as depth 0, which under-indents rather than inventing a
        // parent.
        let depth = node
            .inputs()
            .iter()
            .filter_map(|input| depths.get(input))
            .max()
            .map_or(0, |d| d + 1);
        depths.insert(id, depth);
        rows.push(TreeRow {
            id,
            kind: node_kind(node),
            depth,
            root: roots.contains(&id),
            status: status_of(id, evaluation),
            note: node_note(node),
        });
    }
    rows
}

/// The standing caveat for a node, when it has one — the kernel's own
/// words, never a sentence composed here (the same rule the badges
/// follow).
fn node_note(node: &Node<ProfileProgram>) -> Option<String> {
    match node {
        Node::Mate { class, .. } => match pncad::document::class_admission(*class) {
            pncad::document::ClassAdmission::Mints => None,
            other => Some(format!("{}: {}", class.name(), other.no_record_reason())),
        },
        _ => None,
    }
}

/// One node's status, read out of the result DAG.
fn status_of(id: RecipeNodeId, evaluation: Option<&Evaluation<f64>>) -> RowStatus {
    let Some(ev) = evaluation else {
        return RowStatus::Unevaluated;
    };
    match ev.result(id) {
        None => RowStatus::Unevaluated,
        Some(NodeResult::Ok(_)) => RowStatus::Ok,
        Some(NodeResult::Failed(error)) => RowStatus::Failed {
            message: error.to_string(),
        },
        Some(NodeResult::Poisoned { through }) => RowStatus::Poisoned {
            through: *through,
            // `node_error` walks the one `through` hop to the failed
            // ancestor and answers ITS typed error — the whole reason
            // a poisoned row can say what actually broke without this
            // module knowing anything about the failure.
            message: ev.node_error(id).map(ToString::to_string),
        },
    }
}

/// Whether any row reports a failure or a poisoning — what a chrome
/// shows as "this document is not building".
pub fn has_faults(rows: &[TreeRow]) -> bool {
    rows.iter().any(|row| {
        matches!(
            row.status,
            RowStatus::Failed { .. } | RowStatus::Poisoned { .. }
        )
    })
}
