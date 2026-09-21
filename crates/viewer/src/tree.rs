//! The feature tree: the GQ2 result DAG, as rows a panel can draw.
//!
//! # Failures are values, and this module invents none of them
//!
//! GQ2's ratified codomain is a per-node result — `Ok`, `Failed(e)`,
//! `Poisoned { through }` — and the ratified error rule is that a
//! failure is a typed value the GUI renders, never a string invented
//! at the interaction layer. So a failing row's message is
//! `NodeError`'s own `Display`, and nothing here composes a sentence
//! about what went wrong. The one sentence this module writes ABOUT A
//! FAILURE is a downstream row's ([`downstream_wording`]), and it says
//! only WHERE the failure is.
//!
//! What it does write, and what the rule above does not reach, is what
//! a node IS: [`node_kind`]'s vocabulary spelling, [`node_number`]'s
//! `feature 3`, and [`frame_pose`]'s statement of which frame a datum
//! frame is. Those are readings of the node, not verdicts about a run,
//! and they are sited here because the tree and the creation forms'
//! pickers have to name a node the same way.
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
//! The fault itself resolves that wherever it names a subject, and
//! that subject is a mate node (`blamed_mates` is the reading). So a
//! row whose id the fault NAMES is the cause and stays `Failed`; a row
//! the same fault merely reached is [`RowStatus::Poisoned`] through
//! the mate that is named — the only thing read being which node the
//! kernel's own words point at.
//!
//! **[`MateFault::Band`] names none, and it is the arm that still
//! reaches rows.** A band is the RUN's tolerance, not a decision about
//! any node: with no band the solve decides nothing, and faults every
//! mate and every instance in the DOCUMENT — across cluster
//! boundaries, and including instances no mate touches — with one
//! shared cause. No row is more at fault than another, so nothing here
//! picks one and every row it reached keeps its own `Failed`. That
//! reading is honest about blame and poor about scope, and improving
//! it wants a status saying "the run, not this row" rather than a
//! culprit invented here
//! (`work/chrome/band-refusal-still-badges-every-row.md`).
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
//!
//! Module kind: **vocabulary** — it names no driver type and no
//! `app`-only crate (`crates/viewer/README.md`, Module boundaries).

use std::collections::BTreeMap;

use pncad::document::{
    Datum, Doc, Evaluation, Expr, MateFault, Node, NodeError, NodeErrorKind, NodeResult,
    ProfileProgram, RecipeNodeId,
};
use pncad::quantity::UnitDef;

use crate::frame::Tone;
use crate::props::{in_written, render_number};

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

    /// **Whether this row is one a reader may need to act on** — the
    /// module header's *which row a failure sends the eye to*, as a
    /// value.
    ///
    /// Only a node whose OWN operation refused is
    /// [`Tone::Actionable`]. A row that was never run has nothing to
    /// act on yet, and a poisoned row shows someone else's failure and
    /// points at the row that owns it, so both stay
    /// [`Tone::Advisory`] and the eye goes to the one row a reader can
    /// do something about — a document with six rows downstream of one
    /// broken feature has one loud row, not seven. There can be more
    /// than one: a `MateFault::Contradictory` naming two different
    /// mates blames both, and both are actionable ([`blamed_mates`]).
    ///
    /// **Total, where [`RowStatus::message`] is not**, because an `Ok`
    /// row still has a [`badge`](RowStatus::badge) — a report, nothing
    /// to act on. Whether a given surface DRAWS that badge is the
    /// surface's own decision and not this axis: the Features pane
    /// stays silent on a healthy row, and the end-to-end walk prints
    /// every row's badge including `ok`.
    pub fn tone(&self) -> Tone {
        match self {
            Self::Ok | Self::Unevaluated | Self::Poisoned { .. } => Tone::Advisory,
            Self::Failed { .. } => Tone::Actionable,
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
    /// **Which one of its kind this node is**, when the node itself can
    /// say — today a datum frame's pose ([`frame_pose`]). `None` is a
    /// node kind that has no such sentence, not a sentence that came
    /// out empty.
    pub pose: Option<String>,
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
        Node::Part { .. } => "Part",
        Node::PlacedUnion { .. } => "PlacedUnion",
        Node::Datum(Datum::Plane { .. }) => "Datum plane",
        Node::Datum(Datum::Frame { .. }) => "Datum frame",
        Node::Datum(Datum::FaceFrame { .. }) => "Datum frame (on face)",
        Node::Datum(Datum::AxisInPlane { .. }) => "Datum axis (in sketch)",
        Node::Datum(Datum::Axis { .. }) => "Datum axis",
        Node::Datum(Datum::Point { .. }) => "Datum point",
        Node::Declare { .. } => "Declare",
        Node::Fillet { .. } => "Fillet",
        Node::Chamfer { .. } => "Chamfer",
        Node::Shell { .. } => "Shell",
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

/// **How the chrome names one node inside a sentence**: its number,
/// and what the node itself says about which one of its kind it is.
///
/// The one home for a picker entry's text. The number is what every
/// refusal in this crate calls a node by, so it stays; what follows it
/// is [`frame_pose`], which is the half that tells two frames apart.
pub fn node_label(node: &Node<ProfileProgram>, id: RecipeNodeId) -> String {
    match frame_pose(node) {
        Some(pose) => format!("{} — {pose}", node_number(id)),
        None => node_number(id),
    }
}

/// **How the chrome names a node when there is nothing more to say**:
/// `feature 3`.
///
/// One home for the phrase, because it is the word a person carries
/// between surfaces — a combo entry, a tree row, a property panel's
/// heading, a refusal's subject — and a surface that spelled it
/// `node 3` would be talking about something a reader has to
/// translate.
pub fn node_number(id: RecipeNodeId) -> String {
    format!("feature {}", id.0)
}

/// **What the NODE says about a datum frame's pose** — the sentence
/// that tells two frames a centimetre apart apart.
///
/// **Read off the node and off nothing else, deliberately.** A pose
/// read from an evaluation would have nothing to say on exactly the
/// rows a person is diagnosing: [`rows`] draws a row for every node in
/// the document, including the [`RowStatus::Unevaluated`] ones before
/// the first run lands and the [`RowStatus::Failed`] ones whose value
/// does not exist. A label sourced there would need this one as its
/// fallback anyway, which is one sentence with two spellings.
///
/// So a component that is not a literal is not evaluated and not
/// guessed: the label says the origin is driven and names no number. A
/// [`Datum::FaceFrame`] says whose face it is read off; it cannot say
/// WHICH face, because a face's identity is its role path and
/// `RoleSeg` has no `Display` (`crate::idpass`'s note says so in as
/// many words).
///
/// `None` is a node with no such sentence — every kind but the two
/// frames.
pub fn frame_pose(node: &Node<ProfileProgram>) -> Option<String> {
    match node {
        Node::Datum(Datum::Frame { origin, u, v }) => {
            Some(match (plane_name(u, v), written_point(origin)) {
                (Some(plane), Some(at)) => format!("{plane} at {at}"),
                (Some(plane), None) => format!("{plane}, origin driven"),
                (None, Some(at)) => format!("at {at}"),
                (None, None) => "origin driven".to_owned(),
            })
        }
        Node::Datum(Datum::FaceFrame { at, .. }) => Some(format!("on {}'s face", node_number(*at))),
        _ => None,
    }
}

/// The two-letter name of the plane a frame's axes span, when they are
/// the world's own and point the positive way (`xy`, `zx`, …).
///
/// `None` is every other pair — a driven component, an oblique frame,
/// or an axis pointing backwards — and it is a label that says LESS
/// rather than one that says something else: the origin still
/// separates two frames, and a spelling for the oblique case would be
/// a matrix, not a name.
fn plane_name(u: &[Expr; 3], v: &[Expr; 3]) -> Option<&'static str> {
    let (u, v) = (axis_name(u)?, axis_name(v)?);
    match (u, v) {
        ('x', 'y') => Some("xy"),
        ('y', 'z') => Some("yz"),
        ('z', 'x') => Some("zx"),
        ('y', 'x') => Some("yx"),
        ('z', 'y') => Some("zy"),
        ('x', 'z') => Some("xz"),
        // Two axes that are the SAME axis span no plane. The datum
        // door refuses such a frame at evaluation; the label declines
        // to name a plane for it rather than printing one.
        _ => None,
    }
}

/// The positive world axis a literal triple IS, exactly — `(1, 0, 0)`
/// is `x` and `(0.999, 0, 0)` is nothing.
///
/// Exact, because the triple is what an author typed and the claim is
/// that they typed the axis. A near-miss is a frame a shade off
/// square, which is the case a person most needs the label not to
/// paper over; evaluation normalizes it and this does not.
fn axis_name(v: &[Expr; 3]) -> Option<char> {
    let mut components = [0.0_f64; 3];
    for (slot, expr) in components.iter_mut().zip(v) {
        *slot = expr.literal_value()?;
    }
    match components {
        [1.0, 0.0, 0.0] => Some('x'),
        [0.0, 1.0, 0.0] => Some('y'),
        [0.0, 0.0, 1.0] => Some('z'),
        _ => None,
    }
}

/// A literal 3-D point as the chrome writes it — the numbers in the
/// unit they were AUTHORED in, which is the unit the property panel
/// shows and edits the same slots in.
///
/// `None` as soon as one component is not a literal: a partial point
/// with a hole in it would read as a position, and the caller says
/// "driven" instead.
///
/// The unit is written once after the triple when all three share it,
/// and against each number when they do not — a frame whose origin was
/// typed in three notations is rare, and printing one of its units for
/// all three would be wrong rather than terse.
fn written_point(origin: &[Expr; 3]) -> Option<String> {
    let mut written: Vec<(f64, UnitDef)> = Vec::with_capacity(origin.len());
    for expr in origin {
        let unit = expr.display_unit()?;
        written.push((in_written(expr.literal_value()?, unit), unit));
    }
    let (_, first) = *written.first()?;
    let shared = written.iter().all(|(_, unit)| *unit == first);
    let numbers = written
        .iter()
        .map(|(value, unit)| {
            let number = render_number(*value);
            if shared {
                number
            } else {
                format!("{number} {}", unit.symbol())
            }
        })
        .collect::<Vec<_>>()
        .join(", ");
    Some(if shared {
        format!("({numbers}) {}", first.symbol())
    } else {
        format!("({numbers})")
    })
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
            pose: frame_pose(node),
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
            downstream_of_mate(id, error).unwrap_or_else(|| RowStatus::Failed {
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
/// one home and can be asserted on ([`crate::app::indeterminate_wording`]'s
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
    downstream_of_mate(through, error).unwrap_or(RowStatus::Poisoned {
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
///
/// Two arms name none, and they get an arm each because they are not
/// the same case: one reaches rows and one cannot reach any.
fn blamed_mates(fault: &MateFault) -> Vec<RecipeNodeId> {
    match fault {
        MateFault::Frame { mate, .. }
        | MateFault::ClassNotAdmitted { mate }
        | MateFault::TableLacks { mate, .. }
        | MateFault::Indeterminate { mate, .. }
        | MateFault::Under { mate, .. }
        | MateFault::DanglingHead { mate, .. }
        | MateFault::PlacerRefused { mate, .. }
        | MateFault::SelfMate { mate, .. }
        | MateFault::PartSelectsAnotherCopy { mate, .. }
        | MateFault::Unleverable { mate, .. } => vec![*mate],
        // Names no mate and reaches EVERY row of the document — the
        // asymmetry with the arm below is stated once, on `MateFault`.
        MateFault::Band { .. } => Vec::new(),
        // Names no mate and reaches NO row (`MateFault`'s doc says
        // why); the empty answer here is unreachable, not a reading.
        MateFault::PosesOfAnotherDocument { .. } => Vec::new(),
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
/// `None` — the row keeps its own `Failed` — when the failure is not
/// a mate refusal, when the fault names this very node, and when it
/// names no mate at all ([`MateFault::Band`], the module header's
/// second section).
///
/// **The blame is read directly**, and [`RowStatus::Poisoned`]'s
/// walkable-in-one-hop invariant holds because the kernel's answer is
/// consistent: a mate's content key carries the solve's answer, so a
/// mate the fault names is `Failed` in the evaluation carrying that
/// fault — never `Ok` off a stale memo, and never `Poisoned`, since
/// mates are DAG leaves. A row here that pointed at a green row would
/// be that inconsistency surfacing, not a case to absorb.
fn downstream_of_mate(id: RecipeNodeId, error: &NodeError) -> Option<RowStatus> {
    let NodeErrorKind::Mate(fault) = &error.kind else {
        return None;
    };
    let blamed = blamed_mates(fault);
    if blamed.contains(&id) {
        return None;
    }
    // The first mate the fault names, in the fault's own order.
    let through = blamed.into_iter().next()?;
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
