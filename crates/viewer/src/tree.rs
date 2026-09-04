//! The feature tree: the GQ2 result DAG, as rows a panel can draw.
//!
//! # Failures are values, and this module invents none of them
//!
//! GQ2's ratified codomain is a per-node result — `Ok`, `Failed(e)`,
//! `Poisoned { through }` — and the ratified error rule is that a
//! failure is a typed value the GUI renders, never a string invented
//! at the interaction layer. So a failing row's message is
//! `NodeError`'s own `Display`, and nothing here composes a sentence
//! about what went wrong. The one sentence this module does write is
//! a DOWNSTREAM row's ([`downstream_wording`]), and it says only
//! WHERE the failure is.
//!
//! Because that is the other thing this module owns: the *shape* —
//! which rows exist, in which order, at what indentation, which of
//! them the selection is on, and which row a failure sends the eye to.
//!
//! # A mate refusal poisons across the placement graph, not the DAG
//!
//! Mates and instances are DAG LEAVES — a mate's references are names,
//! not edges — so the placement solve is one shared computation the
//! result DAG has no edges for. When a cluster refuses, the kernel
//! records the SAME typed fault against every instance in that cluster
//! and every mate holding it together, and each of those nodes reports
//! it as its own `Failed`. Read verbatim that draws four identical
//! FAILED badges and sends the eye nowhere.
//!
//! The fault itself resolves that: every `MateFault` arm but
//! [`MateFault::Band`] names its subject, and the subject is a mate
//! node (`blamed_mates` holds that carve-out). So a row whose id the
//! fault NAMES is the cause and stays `Failed`; a row the same fault
//! merely reached is [`RowStatus::Poisoned`] through the mate that is
//! named — the only thing read being which node the kernel's own
//! words point at.
//!
//! # Order and depth
//!
//! Rows follow `Evaluation::order` — the evaluation's own
//! deterministic topological order, which is a pure function of the
//! DAG — so the tree a user reads is the order the kernel evaluated
//! in.
//!
//! Depth is the number of BRANCHES a node sits under, not the length
//! of its input chain. A node continues the line of its PRIMARY input
//! — the first entry of `Node::inputs()`, which is the operand the
//! kernel accumulates into: a boolean's `a`, a fillet's `target`, a
//! transform's `input`. Every other input is a branch that indents:
//!
//! ```text
//! depth(n) = 0                                     if n has no inputs
//! depth(n) = max(depth(primary),
//!                max over the other inputs s of depth(s) + 1)
//! ```
//!
//! So a chain of twenty booleans cutting features out of one solid
//! draws as one column with its tools one level in, rather than a
//! staircase twenty levels wide.

use std::collections::BTreeMap;

use pncad::document::{
    Datum, Doc, Evaluation, MateFault, Node, NodeError, NodeErrorKind, NodeResult, ProfileProgram,
    RecipeNodeId,
};

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
    /// The failure this row shows is not its own: it is downstream of
    /// a failure at `through`.
    ///
    /// Two things arrive here: a DAG descendant of a failed node, which
    /// the evaluation itself reports as poisoned; and a node the
    /// placement solve left without a pose because some OTHER mate in
    /// its cluster refused, which the evaluation reports as its own
    /// `Failed` (the module header's second section).
    Poisoned {
        /// The row to go and read: a node THIS TREE badges `Failed`,
        /// so the walk is one hop as DRAWN and not merely as evaluated
        /// (`poisoned_through` pays for the difference).
        through: RecipeNodeId,
        /// The pointer at `through` — [`downstream_wording`], never
        /// the cause's own text. `None` only if the chain does not end
        /// at a failure at all: a broken invariant reported as absence
        /// rather than papered over with an invented cause.
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

    /// The line drawn under the row, when it has one: the typed
    /// payload's own words on a failing row, and on a downstream row
    /// the pointer at the row that has them.
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
///
/// **The datum FLAVOURS are named apart** (`Datum plane`, not
/// `Datum`), which is the same rule one level down: a plane and a
/// frame are the same surface differing only in whether the spin about
/// the normal is pinned, so a tree that called both "Datum" would ask
/// a reader to tell them apart by clicking. The whole name is the
/// vocabulary's, not a prose gloss — this string is also what the
/// delete confirmation and the kind census say.
pub fn node_kind(node: &Node<ProfileProgram>) -> &'static str {
    match node {
        Node::Profile(_) => "Profile",
        Node::Extrude { .. } => "Extrude",
        Node::Revolve { .. } => "Revolve",
        Node::Transform { .. } => "Transform",
        Node::Boolean { .. } => "Boolean",
        Node::Union { .. } => "Union",
        Node::Split { .. } => "Split",
        Node::Pattern { .. } => "Pattern",
        Node::PlacedUnion { .. } => "PlacedUnion",
        Node::Datum(Datum::Plane { .. }) => "Datum plane",
        Node::Datum(Datum::Frame { .. }) => "Datum frame",
        Node::Datum(Datum::AxisInPlane { .. }) => "Datum axis (in sketch)",
        Node::Datum(Datum::Axis { .. }) => "Datum axis",
        Node::Datum(Datum::Point { .. }) => "Datum point",
        Node::Declare { .. } => "Declare",
        Node::Fillet { .. } => "Fillet",
        Node::Chamfer { .. } => "Chamfer",
        Node::Tube { .. } => "Tube",
        Node::HollowTube { .. } => "HollowTube",
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
        let depth = depth_of(&node.inputs(), &depths);
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

/// A node's indentation, under the module's branch rule: the primary
/// input's own depth, and one level deeper than every other input.
///
/// The order is topological, so every input already has its depth; an
/// input that does not (a node the order omits) reads as depth 0,
/// which under-indents rather than inventing a parent.
fn depth_of(inputs: &[RecipeNodeId], depths: &BTreeMap<RecipeNodeId, usize>) -> usize {
    let Some((primary, branches)) = inputs.split_first() else {
        return 0;
    };
    let on_the_line = depths.get(primary).copied().unwrap_or(0);
    branches
        .iter()
        .filter_map(|input| depths.get(input))
        .map(|d| d + 1)
        .fold(on_the_line, usize::max)
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
        Some(NodeResult::Failed(error)) => {
            downstream_of_mate(id, error, ev).unwrap_or_else(|| RowStatus::Failed {
                message: error.to_string(),
            })
        }
        Some(NodeResult::Poisoned { through }) => poisoned_through(*through, ev),
    }
}

/// What a downstream row says: WHERE the failure is, never what it
/// was.
///
/// Named rather than composed inside a render pass, so the wording has
/// one home and can be asserted on (`app::indeterminate_wording`'s
/// rule). Honest for BOTH of [`RowStatus::Poisoned`]'s producers
/// because both point at a row this same tree badges `Failed`, where
/// the payload's own words are read once instead of once per row the
/// failure reached.
pub fn downstream_wording(through: RecipeNodeId) -> String {
    format!(
        "upstream failure at node {} — that row carries the cause",
        through.0
    )
}

/// The status of a row the EVALUATION poisoned, given the nearest
/// failed ancestor it named.
///
/// That ancestor is `Failed` in the run, but the tree may redraw its
/// row as downstream itself — reachably: a boolean over two instances
/// of a cluster that then refuses is poisoned through an instance
/// whose own row now points at the mate. Two hops, one of them a row
/// with nothing to act on, so this carries the same cause that row
/// does. One step settles it: a mate the fault names keeps its own
/// `Failed`.
fn poisoned_through(through: RecipeNodeId, ev: &Evaluation<f64>) -> RowStatus {
    let Some(error) = ev.result(through).and_then(NodeResult::error) else {
        // The chain does not end at a failure: report the absence.
        return RowStatus::Poisoned {
            through,
            message: None,
        };
    };
    downstream_of_mate(through, error, ev).unwrap_or(RowStatus::Poisoned {
        through,
        message: Some(downstream_wording(through)),
    })
}

/// The mates a solve refusal BLAMES — the nodes the fault's own words
/// point the user at.
///
/// Exhaustive on purpose: a fault arm the kernel grows must decide
/// here whether it names a mate, rather than falling into a wildcard
/// and silently drawing every reached row as downstream of nothing.
/// [`MateFault::Band`] names none — no band, no decisions, so no mate
/// is more at fault than any other — and every row it reached keeps
/// its own `Failed`.
fn blamed_mates(fault: &MateFault) -> Vec<RecipeNodeId> {
    match fault {
        MateFault::Frame { mate, .. }
        | MateFault::ClassNotAdmitted { mate }
        | MateFault::TableLacks { mate, .. }
        | MateFault::Indeterminate { mate, .. }
        | MateFault::Under { mate, .. }
        | MateFault::DanglingHead { mate, .. }
        | MateFault::SelfMate { mate, .. }
        // The refusal is about THIS mate's own datum, so this mate is
        // the row at fault and no other.
        | MateFault::Unleverable { mate, .. } => vec![*mate],
        MateFault::Band { .. } => Vec::new(),
        // A contradiction is a claim about a PAIR of mates: neither is
        // the wrong one on the fault's own telling, so both read as
        // causes and the user picks which to relax.
        MateFault::Contradictory { held, added, .. } => {
            if held == added {
                vec![*held]
            } else {
                vec![*held, *added]
            }
        }
    }
}

/// The downstream reading of a node's own `Failed`, when a mate
/// refusal reached it without naming it.
///
/// `None` — the row keeps its own `Failed` — on every guard below,
/// of which the last is the load-bearing one: no mate the fault names
/// is failing in THIS evaluation.
///
/// That guard keeps [`RowStatus::Poisoned`]'s walkable-in-one-hop
/// invariant true here rather than assumed. What it catches is a named
/// mate reading `Ok` — the only other reading available, since mates
/// are DAG leaves and so are never `Poisoned` — which happens in the
/// very evaluation carrying the fault, because a mate's memo key does
/// not carry the solve and a cluster that breaks around an unedited
/// mate does not re-run it. Pointing at a green row would send the
/// user nowhere and drop the message they can act on.
fn downstream_of_mate(
    id: RecipeNodeId,
    error: &NodeError,
    ev: &Evaluation<f64>,
) -> Option<RowStatus> {
    let NodeErrorKind::Mate(fault) = &error.kind else {
        return None;
    };
    let blamed = blamed_mates(fault);
    if blamed.contains(&id) {
        return None;
    }
    // The first named mate the run agrees is failing, in the fault's
    // own order.
    let through = blamed
        .into_iter()
        .find(|mate| ev.result(*mate).and_then(NodeResult::error).is_some())?;
    Some(RowStatus::Poisoned {
        through,
        message: Some(downstream_wording(through)),
    })
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
