//! The feature tree: the GQ2 result DAG, as rows a panel can draw.
//!
//! # Failures are values, and this module invents none of them
//!
//! GQ2's ratified codomain is a per-node result — `Ok`, `Failed(e)`,
//! `Poisoned { through }` — and the ratified error rule is that a
//! failure is a typed value the GUI renders, never a string invented
//! at the interaction layer. So a failing row's message is
//! `NodeError`'s own `Display`, and nothing here composes a sentence
//! about what went wrong. The two sentences this module writes ABOUT
//! A FAILURE are a downstream row's pointer ([`downstream_wording`])
//! and a failed row's link to the node to repair ([`link_wording`]),
//! and both say only WHERE to go. What it writes about an assertion's
//! verdict is the comparison it decided ([`Asserted::comparison`]),
//! out of the kernel's own relation and two computed values.
//!
//! A failure that CARRIES another node's refusal — a part whose root
//! failed or was poisoned, a mate whose placer refused — points at the
//! node that failed and never quotes it, so the carried refusal is
//! drawn under the row as a line of its own, and so on down, one line
//! per document level ([`carried_lines`]). Each line is the kernel's
//! own rendering of that node's failure, byte for byte. What this
//! module adds is a label beside it, never inside it: WHICH document
//! the line's node number belongs to, by file name, since the kernel
//! knows a part only by its id.
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
//! One thing a run said that is no failure rides the row as well: what
//! a measure measured ([`Measured`]) — its value, spelled as the chrome
//! spells any computed value, or the kernel's typed reason it has none
//! — and what an assertion found of it ([`Asserted`]).
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
//! The fault itself resolves that wherever it names a mate
//! (`blamed_mates` is the reading). So a mate the fault BLAMES is the
//! cause and keeps its `Failed`; a row the same fault merely reached
//! is [`RowStatus::Poisoned`] through the blamed mate — the only thing
//! read being which node the kernel's own words point at.
//!
//! **This is the viewer's one answer to which row a node's failure
//! is.** Every other surface that says why a node has no value reads
//! it off [`cause_row`]: a picked face's verdict, a tool's refusal and
//! the pick index's tooltip hold the kernel's `NodeStanding` re-read by
//! [`standing_as_drawn`], and the at-rest badge draws the product
//! gather's refusal in [`product_refusal_wording`]. So no panel names
//! a different row from the tree's. Every kernel door under
//! `crates/viewer/src` that hands back a standing is censused by
//! `tree_badges::every_standing_door_in_the_viewer_reads_the_trees_answer`.
//!
//! **The blamed node is the row that carries the fault's words, and
//! it need not be the node an author edits.** This is the one
//! statement of why; [`blamed_mates`] points here. Several arms name a
//! node beside the mate, and ONE of them is a node the kernel's own
//! doc for the arm calls the thing to repair: `PlacerRefused`'s
//! `placer`, *"the node an author goes and fixes"*. The others are
//! where the refusal was noticed or held, and the repair is the
//! mate's: `DanglingHead`'s `head` *"may be perfectly live"* and its
//! message's recourse, *"rebind it"*, is the mate's reference;
//! `PartSelectsAnotherCopy` is refused *"rather than choosing"*
//! between the name and the `Part`; `Under`'s pair and `SelfMate`'s
//! instance are evidence about where the refusal held. Blame stays on
//! the mate for every one of them, and what decides it is REACH —
//! where the solve records the fault:
//!
//! - **Raised where the solve reads one mate's references**
//!   (`mate::member::check_reference` and the walk): every
//!   [`MateFault::DanglingHead`], every
//!   [`MateFault::PartSelectsAnotherCopy`], and a
//!   [`MateFault::PlacerRefused`] whose placer's own slot does not
//!   evaluate. The fault is recorded against THAT MATE ONLY, so blame
//!   decides a single row: whether the mate reads `Failed` with the
//!   words, or `Poisoned` — a pointer away from the only row that has
//!   them. The named node's row is whatever the evaluation says of it
//!   on its own: `Ok` when it evaluates, `Failed` beside the mate when
//!   it does not (a `Part` indexing past its pattern's count, a pattern
//!   of no copies), and then both rows are loud and neither is
//!   poisoned through the other.
//! - **Raised while a cluster's fold derives an offset**
//!   (`mate::member::derived_offset`): a `PlacerRefused` only. It
//!   reaches every instance and mate of the cluster, and they point at
//!   the mate, whose row carries the placer's refusal verbatim. A
//!   placer on the chain above an instance the fault reached is
//!   poisoned by the evaluation even when its own slots are broken, so
//!   its row carries the pointer at the mate rather than the words —
//!   the arm's doc's *"this fault is the only place that cause
//!   appears"*.
//!
//! **So a `PlacerRefused` mate's row LINKS to the placer**
//! ([`TreeRow::repair_at`]), on either path: blame decides which row
//! is loud and carries the words, and the link is how a reader gets
//! from those words to the node the kernel says to fix. No other mate
//! arm links, for the reason above; `repaired_at` is where an arm
//! answers this. On the fold's path the link lands on a poisoned
//! placer whose own line points back at the mate — the words are on
//! the mate's row, so the link's job is to put the placer under the
//! selection, not to show a second copy of them.
//!
//! **One seat the link inherits is wrong, and it is the kernel's.**
//! `check_reference` evaluates a `Part`'s index expression under the
//! PATTERN below the `Part` (`count_of(level.node, index,
//! SlotId::Instance)`), so a `Part` index that does not evaluate names
//! the pattern as the placer: the link then selects an `Ok` pattern
//! while the `Part` beside it is `Failed` with the real cause
//! (`work/msolve/placer-refused-names-the-pattern-for-a-part-index-that-does-not-evaluate.md`).
//! The tree draws what the fault names; the fix is MSOLVE's.
//!
//! `crates/viewer/tests/msolve3_placer_refused.rs` holds one fixture
//! of each shape above.
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
//! # A failed row outside the solve links where its own error says
//!
//! Every other `NodeErrorKind` is asked the same question
//! (`repair_named`). One links: a profile refused with
//! `FrameDirection` links to the frame, whose own direction slot is
//! what refused — and whose row may read `Ok`, because the frame
//! lands at the lane while its nominal does not.
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

use pncad::document::AssemblyError;
use pncad::document::{
    AssertionDir, AssertionVerdict, CarriedIn, Datum, Doc, Evaluation, Expr, MateFault,
    MeasureUnavailableAt, Node, NodeError, NodeErrorKind, NodeResult, NodeStanding, ProductError,
    ProfileProgram, RecipeNodeId, ValuePayload,
};
use pncad::quantity::UnitDef;
use pncad::select::{InterrogateError, Resolution, ResolveIndeterminate};

use crate::frame::Tone;
use crate::parts::PartFiles;
use crate::props::{computed_text, in_written, render_number};

/// **One level of a failure's traceback**, as the tree draws it: the
/// document the level's node is in, as a label of its own, and the
/// node's refusal.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CarriedLine {
    /// The document, by file name ([`PartFiles::name`]), or
    /// [`THIS_DOCUMENT`] for the tree's own.
    pub document: String,
    /// The node's refusal exactly as its own tree draws it:
    /// `NodeError`'s `Display` for that node and kind.
    pub line: String,
}

/// The label of a carried level whose node is in the document the tree
/// draws.
pub const THIS_DOCUMENT: &str = "this document";

/// A node's status, as the tree draws it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RowStatus {
    /// The node produced a value.
    Ok,
    /// The node's own operation failed. `message` is the typed
    /// error's own rendering, and `carried` the refusals it carries.
    Failed {
        /// `NodeError`'s `Display`.
        message: String,
        /// **The refusals `message` points at and does not quote**, one
        /// per level ([`carried_lines`]): another node's refusal, with
        /// its own recourse, drawn under this row exactly as that
        /// node's own tree draws it. Empty for a failure that carries
        /// none.
        carried: Vec<CarriedLine>,
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
            Self::Failed { message, .. } => Some(message),
            Self::Poisoned { message, .. } => message.as_deref(),
        }
    }

    /// **The row a click on [`message`](RowStatus::message) selects**,
    /// when that line is a pointer rather than words to read.
    ///
    /// The target and the affordance are one answer: a line with
    /// somewhere to go is a link, and a line with nowhere to go is
    /// read. So this is the only thing a surface asks to learn both,
    /// and no second value says which of the two a line is.
    ///
    /// Only a poisoned row's line is a pointer — at `through`, the row
    /// that owns the failure. A failed row's words are its own cause;
    /// the node THEY name as the one to repair is the row's, not the
    /// status's ([`TreeRow::repair_at`]), and gets a line of its own.
    pub fn jump(&self) -> Option<RecipeNodeId> {
        match self {
            Self::Poisoned { through, .. } => Some(*through),
            Self::Ok | Self::Unevaluated | Self::Failed { .. } => None,
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
    /// say — a datum frame's pose ([`frame_pose`]), and an instance's
    /// part by its file name ([`part_file`]). `None` is a node kind
    /// that has no such sentence, not a sentence that came out empty.
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
    /// **The node a [`RowStatus::Failed`] row's words name as the one
    /// to repair, when that is not this node**: whatever
    /// `repair_named` answers for the row's own error.
    ///
    /// `None` on every row that is not `Failed`. A row's other links
    /// carry their own target: a `Poisoned` row's is its `through`,
    /// and an assertion's is its [`Asserted::measure`].
    pub repair_at: Option<RecipeNodeId>,
    /// **What a measure node measured, or an assertion found**, on a
    /// row that is `Ok`; `None` on every other row, whose status says
    /// why there is none.
    pub measured: Option<Measured>,
}

impl TreeRow {
    /// **How loud this row is drawn**: its status's tone
    /// ([`RowStatus::tone`]), except on an assertion's row, which is
    /// `Ok` and as loud as its verdict ([`Asserted::tone`]).
    pub fn tone(&self) -> Tone {
        match &self.measured {
            Some(Measured::Asserted(asserted)) => asserted.tone(),
            Some(Measured::Value(_) | Measured::Unavailable(_)) | None => self.status.tone(),
        }
    }
}

/// **What a measure's or an assertion's row says the run found**: a
/// measure's value or the kernel's reason it has none, and on an
/// assertion's row, its verdict over that value.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Measured {
    /// The value, already spelled ([`computed_text`]: canonical
    /// notation, width-bounded, with its unit's symbol) — which keeps
    /// the row `Eq`, and keeps the notation choice here with the rest
    /// of what the tree writes about a node rather than at each
    /// surface that draws it.
    Value(String),
    /// No value at this build's scalar — a value of the node, not a
    /// failure. Its `Display` is the kernel's sentence, which names
    /// the door that can answer.
    Unavailable(MeasureUnavailableAt),
    /// An assertion's verdict over its measure.
    Asserted(Asserted),
}

/// **An assertion's verdict, as its row says it**: the kernel's
/// verdict with both numbers spelled as the measure's own value is
/// ([`computed_text`], in the measure's dimension), the side of the
/// bound the measure must fall on, and which measure that is.
#[derive(Clone, Debug, PartialEq)]
pub struct Asserted {
    /// The landed verdict.
    pub verdict: AssertionVerdict<String>,
    /// Which side of the bound the measure must fall on.
    pub dir: AssertionDir,
    /// The measure node the assertion constrains.
    pub measure: RecipeNodeId,
}

// `AssertionVerdict` derives `PartialEq` alone, for its scalar's sake;
// over `String` numbers and an `Eq` reason it is an equivalence.
impl Eq for Asserted {}

impl Asserted {
    /// **How loud the verdict is drawn**: a `Violated` requirement is
    /// one the author recorded and the geometry fails, which is a
    /// verdict a reader may need to act on. `Holds` is a report, and
    /// `Unevaluated` is no verdict at all.
    pub fn tone(&self) -> Tone {
        match self.verdict.holds() {
            Some(false) => Tone::Actionable,
            Some(true) | None => Tone::Advisory,
        }
    }

    /// **The comparison the verdict decided**, as a report reads it —
    /// `0.0125 m >= 0.01 m` — or `None` where there is no verdict.
    pub fn comparison(&self) -> Option<String> {
        match &self.verdict {
            AssertionVerdict::Holds { measured, bound }
            | AssertionVerdict::Violated { measured, bound } => {
                Some(format!("{measured} {} {bound}", self.dir.symbol()))
            }
            AssertionVerdict::Unevaluated { .. } => None,
        }
    }
}

/// The kind name of a recipe node — the node vocabulary's own
/// spelling.
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
/// between surfaces — a combo entry, a downstream row's pointer, a
/// property panel's heading, a refusal's subject — and a surface that
/// spelled it
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
        Node::Datum(
            Datum::Plane { .. }
            | Datum::Axis { .. }
            | Datum::Point { .. }
            | Datum::AxisInPlane { .. },
        )
        | Node::Profile(_)
        | Node::Extrude { .. }
        | Node::Revolve { .. }
        | Node::Tube { .. }
        | Node::HollowTube { .. }
        | Node::Loft { .. }
        | Node::Sweep { .. }
        | Node::Fillet { .. }
        | Node::Chamfer { .. }
        | Node::Shell { .. }
        | Node::Split { .. }
        | Node::Boolean { .. }
        | Node::Union { .. }
        | Node::Transform { .. }
        | Node::Pattern { .. }
        | Node::Part { .. }
        | Node::PlacedUnion { .. }
        | Node::Declare { .. }
        | Node::InstantiatePart { .. }
        | Node::Mate { .. }
        | Node::Measure { .. }
        | Node::Assertion { .. } => None,
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
///
/// `files` names the parts the document instantiates: an instance row
/// and a part's carried lines name their document by file name.
pub fn rows(
    doc: &Doc<ProfileProgram>,
    evaluation: Option<&Evaluation<f64>>,
    files: &PartFiles,
) -> Vec<TreeRow> {
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
        let status = status_of(id, evaluation, files);
        let (repair_at, measured) = match status {
            RowStatus::Failed { .. } => (evaluation.and_then(|ev| repair_of(id, ev)), None),
            RowStatus::Ok => (None, evaluation.and_then(|ev| measured_of(id, node, ev))),
            RowStatus::Poisoned { .. } | RowStatus::Unevaluated => (None, None),
        };
        rows.push(TreeRow {
            id,
            kind: node_kind(node),
            pose: frame_pose(node).or_else(|| part_file(node, files)),
            depth,
            root: roots.contains(&id),
            status,
            note: node_note(node),
            repair_at,
            measured,
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
            caveat @ (pncad::document::ClassAdmission::NoAtRestRecord { .. }
            | pncad::document::ClassAdmission::NotAdmitted) => {
                Some(format!("{}: {}", class.name(), caveat.no_record_reason()))
            }
        },
        Node::Datum(_)
        | Node::Profile(_)
        | Node::Extrude { .. }
        | Node::Revolve { .. }
        | Node::Tube { .. }
        | Node::HollowTube { .. }
        | Node::Loft { .. }
        | Node::Sweep { .. }
        | Node::Fillet { .. }
        | Node::Chamfer { .. }
        | Node::Shell { .. }
        | Node::Split { .. }
        | Node::Boolean { .. }
        | Node::Union { .. }
        | Node::Transform { .. }
        | Node::Pattern { .. }
        | Node::Part { .. }
        | Node::PlacedUnion { .. }
        | Node::Declare { .. }
        | Node::InstantiatePart { .. }
        | Node::Measure { .. }
        | Node::Assertion { .. } => None,
    }
}

/// **What `id` measured in `evaluation`** — asked only of a row
/// [`rows`] has read `Ok`, so this decides which payload, never
/// whether there is one. `None` for every payload but a measure's or
/// an assertion's.
fn measured_of(
    id: RecipeNodeId,
    node: &Node<ProfileProgram>,
    evaluation: &Evaluation<f64>,
) -> Option<Measured> {
    match &evaluation.value(id)?.payload {
        ValuePayload::Measure { value, dim } => Some(Measured::Value(computed_text(*dim, *value))),
        ValuePayload::MeasureUnavailable { reason, .. } => Some(Measured::Unavailable(*reason)),
        ValuePayload::Assertion(verdict) => {
            Some(Measured::Asserted(asserted(node, verdict, evaluation)))
        }
        ValuePayload::Body(_)
        | ValuePayload::Boolean(_)
        | ValuePayload::Datum(_)
        | ValuePayload::Profile(_)
        | ValuePayload::Split { .. }
        | ValuePayload::Instances(_)
        | ValuePayload::Declarations(_)
        | ValuePayload::Mate(_) => None,
    }
}

/// **An assertion's verdict, its numbers spelled in its measure's
/// dimension.** A verdict carries numbers only when its measure
/// evaluated to a value, so the dimension is that value's.
fn asserted(
    node: &Node<ProfileProgram>,
    verdict: &AssertionVerdict<f64>,
    evaluation: &Evaluation<f64>,
) -> Asserted {
    let Node::Assertion { measure, dir, .. } = node else {
        unreachable!("only an assertion node evaluates to a verdict")
    };
    let dim = || match evaluation.value(*measure).map(|value| &value.payload) {
        Some(ValuePayload::Measure { dim, .. }) => *dim,
        other => unreachable!(
            "a verdict with numbers compared a measured value, yet its measure holds {:?}",
            other.map(ValuePayload::kind_name)
        ),
    };
    Asserted {
        verdict: verdict.clone().map(|number| computed_text(dim(), number)),
        dir: *dir,
        measure: *measure,
    }
}

/// **Which part an instance row is**: the file its reference names, by
/// file name, or what `files` knows instead ([`PartFiles::name`]) —
/// never the id. `None` for every node that is not an instance.
pub fn part_file(node: &Node<ProfileProgram>, files: &PartFiles) -> Option<String> {
    match node {
        Node::InstantiatePart { doc_ref, .. } => Some(files.name(doc_ref.id).to_owned()),
        _ => None,
    }
}

/// **The levels a failure draws under its own**: the kernel's carried
/// chain ([`NodeErrorKind::carried_chain`]), one level per carried
/// refusal — a part inside a part reads one level per document, and
/// the last is the failing node's own refusal.
///
/// Each line is that node's refusal exactly as its own tree draws it;
/// the document it is in, whose numbering the line's node number is,
/// is its label ([`CarriedLine::document`]).
pub fn carried_lines(kind: &NodeErrorKind, files: &PartFiles) -> Vec<CarriedLine> {
    kind.carried_chain()
        .map(|level| CarriedLine {
            document: match level.document {
                CarriedIn::ThisDocument => THIS_DOCUMENT.to_owned(),
                CarriedIn::Part(doc_ref) => files.name(doc_ref.id).to_owned(),
            },
            line: level.line(),
        })
        .collect()
}

/// **Where a row stands**, before anything is drawn of it: a status
/// read whole, or the row's own failure, whose words are drawn only
/// where they are shown ([`status_of`]).
enum Standing<'e> {
    Status(RowStatus),
    Failed(&'e NodeError),
}

fn standing(id: RecipeNodeId, evaluation: Option<&Evaluation<f64>>) -> Standing<'_> {
    let Some(ev) = evaluation else {
        return Standing::Status(RowStatus::Unevaluated);
    };
    match ev.result(id) {
        None => Standing::Status(RowStatus::Unevaluated),
        Some(NodeResult::Ok(_)) => Standing::Status(RowStatus::Ok),
        Some(NodeResult::Failed(error)) => {
            downstream_of_mate(id, error).map_or(Standing::Failed(error), Standing::Status)
        }
        Some(NodeResult::Poisoned { through }) => Standing::Status(poisoned_through(*through, ev)),
    }
}

/// One node's status, read out of the result DAG.
fn status_of(
    id: RecipeNodeId,
    evaluation: Option<&Evaluation<f64>>,
    files: &PartFiles,
) -> RowStatus {
    match standing(id, evaluation) {
        Standing::Status(status) => status,
        Standing::Failed(error) => RowStatus::Failed {
            message: error.to_string(),
            carried: carried_lines(&error.kind, files),
        },
    }
}

/// **The row this tree sends a reader to for `id`'s failure**: `id`
/// itself when its row is `Failed`, the row it points at when it is
/// `Poisoned`, and `None` when it is `Ok` or never ran.
///
/// Read off the same [`standing`] as [`status_of`], so a surface
/// reporting a CONSEQUENCE of a node's failure names the same row the
/// tree badges `Failed` — blame through a poisoning and through a mate refusal included —
/// rather than re-deriving the blame from the evaluation and drawing
/// it differently.
///
/// **Every `Some` is a row the tree draws `Failed`.** A `Poisoned` row
/// carries its pointer only when its chain ends at a failure
/// ([`poisoned_through`]); the `message: None` arm is the broken
/// invariant reported as absence, and it answers `None` here too, so a
/// caller that says "feature N, which failed" cannot be handed an `N`
/// the tree does not badge failed. That arm is not expected to be
/// reachable — the evaluation names a failed ancestor as `through` —
/// and it is refused rather than assumed for the same reason the tree
/// reports it as absence.
pub fn cause_row(id: RecipeNodeId, evaluation: &Evaluation<f64>) -> Option<RecipeNodeId> {
    match standing(id, Some(evaluation)) {
        Standing::Failed(_) | Standing::Status(RowStatus::Failed { .. }) => Some(id),
        Standing::Status(RowStatus::Poisoned {
            through,
            message: Some(_),
        }) => Some(through),
        Standing::Status(
            RowStatus::Poisoned { message: None, .. } | RowStatus::Ok | RowStatus::Unevaluated,
        ) => None,
    }
}

/// **A kernel standing, re-read as this tree draws its node** (the
/// module header's second section).
///
/// The kernel reports a node a cluster refusal reached as its own
/// `Failed`, and a node poisoned through such a node as poisoned
/// through it; the tree draws both as downstream of the mate the fault
/// blames. Answered off [`cause_row`]: a node whose cause is another
/// row reads `Poisoned` through that row, a node that is its own cause
/// reads `Failed`. A standing [`cause_row`] has no row for — no
/// evaluation entry, not in the document, a chain that ends at no
/// failure — is the kernel's, unchanged.
///
/// **The `Poisoned` this answers is not the kernel's contract.**
/// `NodeStanding::Poisoned`'s `through` is documented as the nearest
/// failed DAG ancestor; here it may be a mate, which is no node's
/// ancestor, and its `Display` then calls the repair "upstream". The
/// kernel question is
/// `work/wire/kernel-standing-names-a-cluster-refused-node-as-its-own-failure`.
pub fn standing_as_drawn(standing: NodeStanding, evaluation: &Evaluation<f64>) -> NodeStanding {
    match standing {
        NodeStanding::Failed { node } | NodeStanding::Poisoned { node, .. } => {
            match cause_row(node, evaluation) {
                Some(cause) if cause == node => NodeStanding::Failed { node },
                Some(through) => NodeStanding::Poisoned { node, through },
                None => standing,
            }
        }
        NodeStanding::NotEvaluated { .. } | NodeStanding::NotInDocument { .. } => standing,
    }
}

/// A name's [`Resolution`], its indeterminate standing re-read by
/// [`standing_as_drawn`]; every other verdict is the resolution
/// machinery's, unchanged.
pub fn resolution_as_drawn(resolution: Resolution, evaluation: &Evaluation<f64>) -> Resolution {
    match resolution {
        Resolution::Indeterminate(ResolveIndeterminate { standing }) => {
            Resolution::Indeterminate(ResolveIndeterminate {
                standing: standing_as_drawn(standing, evaluation),
            })
        }
        Resolution::Resolved(_) | Resolution::Failed(_) => resolution,
    }
}

/// **The words a product-gather refusal is shown in**: the gather's
/// own, with a root's standing re-read by [`standing_as_drawn`], so a
/// root this tree draws downstream of a row the refusal does not name
/// is refused as downstream of that row, in the standing's one sentence.
///
/// Words and not a re-attributed [`ProductError`]: the refusal's own
/// value stays the gather's, and only what is drawn from it is the
/// tree's.
pub fn product_refusal_wording(fault: &ProductError, evaluation: &Evaluation<f64>) -> String {
    match fault {
        ProductError::Root(standing) => AssemblyError::product_refusal(&ProductError::Root(
            standing_as_drawn(*standing, evaluation),
        )),
        ProductError::EvaluationOfAnotherDocument { .. }
        | ProductError::PlacedUnderTwoRoots { .. }
        | ProductError::Naming { .. }
        | ProductError::NoBodyRoots
        | ProductError::Graft { .. }
        | ProductError::RootInvalid { .. }
        | ProductError::ProductInvalid { .. }
        | ProductError::ContactLineage { .. } => AssemblyError::product_refusal(fault),
    }
}

/// An interrogation refusal, its standing re-read by
/// [`standing_as_drawn`]; every other refusal is the door's,
/// unchanged.
pub fn interrogation_as_drawn(
    error: InterrogateError,
    evaluation: &Evaluation<f64>,
) -> InterrogateError {
    match error {
        InterrogateError::Standing(standing) => {
            InterrogateError::Standing(standing_as_drawn(standing, evaluation))
        }
        InterrogateError::NoSuchName
        | InterrogateError::Ambiguous { .. }
        | InterrogateError::WrongKind { .. }
        | InterrogateError::WholeBody
        | InterrogateError::NoBodies { .. }
        | InterrogateError::NoSuchBody { .. }
        | InterrogateError::Readback(_) => error,
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
///
/// The row pointed at is named by [`node_number`], the chrome's one
/// spelling of a node: this sentence is chrome, drawn in the tree.
pub fn downstream_wording(through: RecipeNodeId) -> String {
    format!(
        "upstream failure at {} — that row carries the cause",
        node_number(through)
    )
}

/// **What a link to a node says**: that node's name, as
/// [`node_number`] spells it, and nothing about why — the why is
/// drawn elsewhere, so this names only WHERE to go.
pub fn link_wording(at: RecipeNodeId) -> String {
    format!("see {}", node_number(at))
}

/// The node a `Failed` row's error names as the one to repair, when
/// that is another node.
fn repair_of(id: RecipeNodeId, ev: &Evaluation<f64>) -> Option<RecipeNodeId> {
    repair_named(&ev.result(id)?.error()?.kind).filter(|at| *at != id)
}

/// **Which node an evaluation error names as the one an author
/// repairs**: the named node whose own authored input is what
/// refused, as against a node the words mention as evidence or as
/// the input the failing node misused. The kernel's doc for the arm
/// says which: `PlacerRefused`'s calls it *"the node an author goes
/// and fixes"*; `FrameDirection`'s refusal is the frame's own slot's,
/// carried unaltered to the reader.
fn repair_named(kind: &NodeErrorKind) -> Option<RecipeNodeId> {
    match kind {
        NodeErrorKind::Mate(fault) => repaired_at(fault),
        // The frame's own direction slot refused; the profile only
        // read it, and the frame's row may well read `Ok`.
        NodeErrorKind::FrameDirection { frame, .. } => Some(*frame),
        // An input of the failing node whose value is a family the
        // operand does not take (a split fed to a boolean): the input
        // is a sound node, and choosing it is the failing node's.
        NodeErrorKind::WrongOperand { .. } => None,
        // Either of two nodes may be the repair — the empty input or
        // the failing node's use of it, the pattern's count or the
        // `Part`'s index, either frame — and one link would pick for
        // the reader. Open:
        // `work/chrome/failed-row-repair-links-for-arms-with-two-candidate-repairs`.
        NodeErrorKind::EmptyOperand { .. }
        | NodeErrorKind::EmptyHalf { .. }
        | NodeErrorKind::InstanceOutOfRange { .. }
        | NodeErrorKind::AxisInDifferentPlane { .. } => None,
        // Names an id no live node holds, so there is no row to go to.
        NodeErrorKind::MissingInput { .. } => None,
        // The lane cannot carry what the named nodes hold; neither
        // node is wrong, and the f64 lane builds them.
        NodeErrorKind::SeedPinnedSection { .. } | NodeErrorKind::DerivedFrameSection { .. } => None,
        // Names the site the declaration chose, and the choice is the
        // `Declare`'s, which the error does not name.
        NodeErrorKind::DeclareSiteNotAnOperand { .. } => None,
        // Names the failing instance itself.
        NodeErrorKind::CrossingUnverified { .. } => None,
        // A payload that names a node does so as evidence: a name's
        // minting node, where the repair is the referring node's own
        // reference; an upstream table the naming pass found missing;
        // or a node in ANOTHER document's id space (`PartFault`), which
        // no row of this tree is.
        NodeErrorKind::Part { .. }
        | NodeErrorKind::DeclareResolve { .. }
        | NodeErrorKind::UndeclaredContact { .. }
        | NodeErrorKind::UndeclarableContact { .. }
        | NodeErrorKind::BlendSelectionResolve { .. }
        | NodeErrorKind::BlendSelectionKind { .. }
        | NodeErrorKind::ShellOpenResolve { .. }
        | NodeErrorKind::ShellOpenKind { .. }
        | NodeErrorKind::FaceFrameResolve { .. }
        | NodeErrorKind::FaceFrameKind { .. }
        | NodeErrorKind::MeasureRefResolve { .. }
        | NodeErrorKind::MeasureRefUnreadable { .. }
        | NodeErrorKind::Naming(_) => None,
        // Name no node beside the failing one.
        NodeErrorKind::Expr { .. }
        | NodeErrorKind::Profile(_)
        | NodeErrorKind::ProfileReplay { .. }
        | NodeErrorKind::ProfileLaneReplay { .. }
        | NodeErrorKind::ProfileAnchor { .. }
        | NodeErrorKind::ProfilePieces { .. }
        | NodeErrorKind::Extrude(_)
        | NodeErrorKind::Revolve(_)
        | NodeErrorKind::Tube(_)
        | NodeErrorKind::Split(_)
        | NodeErrorKind::Blend { .. }
        | NodeErrorKind::Boolean(_)
        | NodeErrorKind::Transform(_)
        | NodeErrorKind::Skin(_)
        | NodeErrorKind::Loft(_)
        | NodeErrorKind::CurvedSolidFrontier { .. }
        | NodeErrorKind::ToleranceConflict { .. }
        | NodeErrorKind::ParamBox { .. }
        | NodeErrorKind::Seed { .. }
        | NodeErrorKind::DegenerateDirection { .. }
        | NodeErrorKind::NonFiniteDirection { .. }
        | NodeErrorKind::UnderflowedDirection { .. }
        | NodeErrorKind::Band(_)
        | NodeErrorKind::MissingSlot { .. }
        | NodeErrorKind::VerbArity { .. }
        | NodeErrorKind::Escalated { .. }
        | NodeErrorKind::NonPositiveCount { .. }
        | NodeErrorKind::PlacementsUncertified { .. }
        | NodeErrorKind::PlacementRule(_)
        | NodeErrorKind::UnschedulableCycle
        | NodeErrorKind::ParamSourceAttach(_)
        | NodeErrorKind::DeclareUnsupportedPair { .. }
        | NodeErrorKind::BlendSelectionEmpty { .. }
        | NodeErrorKind::Shell(_)
        | NodeErrorKind::ShellLaneUnsupported { .. }
        | NodeErrorKind::FaceFrameNotPlanar { .. }
        | NodeErrorKind::FaceFrameReadback { .. }
        | NodeErrorKind::WitnessBifurcation(_)
        | NodeErrorKind::MeasureNonFinite { .. }
        | NodeErrorKind::MeasureNotParallel { .. }
        | NodeErrorKind::MeasureUnsupported(_)
        | NodeErrorKind::MeasureMalformed(_)
        | NodeErrorKind::PayloadExpr { .. }
        | NodeErrorKind::MeasureSelectionKind { .. }
        | NodeErrorKind::MeasureClearanceRefused(_)
        | NodeErrorKind::AssertionDimension { .. } => None,
    }
}

/// **Which node a mate refusal names as the one an author repairs,
/// where that is not the blamed mate** — the module header's second
/// section, one arm per fault.
fn repaired_at(fault: &MateFault) -> Option<RecipeNodeId> {
    match fault {
        // The kernel's own doc: "the node an author goes and fixes".
        // For a `Part` index that does not evaluate, the kernel names
        // the pattern below the `Part` here (the module header's
        // kernel-seat paragraph).
        MateFault::PlacerRefused { placer, .. } => Some(*placer),
        // Named beside the mate, and not the repair (module header).
        MateFault::DanglingHead { .. } | MateFault::PartSelectsAnotherCopy { .. } => None,
        // Name no node beside the mate, or name one only as evidence
        // of where the refusal held.
        MateFault::Frame { .. }
        | MateFault::ClassNotAdmitted { .. }
        | MateFault::TableLacks { .. }
        | MateFault::Indeterminate { .. }
        | MateFault::Under { .. }
        | MateFault::SelfMate { .. }
        | MateFault::Unleverable { .. }
        | MateFault::Contradictory { .. }
        | MateFault::Band { .. }
        | MateFault::PosesOfAnotherDocument { .. } => None,
    }
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
/// Every arm that names a mate blames it, whatever else it names; why
/// that holds for an arm naming a node an author may repair is stated
/// once, in the module header's second section.
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

/// Whether any row of a tree reports a failure or a poisoning.
///
/// **A test oracle, not a chrome surface.** No `src/` caller reads
/// this: what a chrome actually shows as "this document is not
/// building" is decided per row by the Features pane's badge draw
/// ([`crate::pane::features`]) and, for the product, by
/// [`crate::frame::product_badge`]. Every caller is an assertion —
/// `!has_faults(…)` is the "this evaluates clean" gate at fifteen of
/// the twenty-two sites, across seven suites and
/// `examples/r1_e2e.rs`. That is what a wrong answer here costs: an
/// oracle silent about a state lets every one of those gates keep
/// passing on documents broken in the new way.
///
/// **Every arm is this chrome's own policy, and none of it is read off
/// the kernel.** A [`RowStatus`] is already a viewer reading of an
/// evaluation, so there is no upstream rule to cite here the way
/// `frame::badge_site` cites `ProductErrorKind::means_no_body`: what a
/// chrome shows as "not building" is decided here, one state at a
/// time.
///
/// - [`RowStatus::Failed`] and [`RowStatus::Poisoned`] are faults. A
///   poisoned row's failure is someone else's, but the document it
///   belongs to is no more building for that.
/// - [`RowStatus::Unevaluated`] is **not** a fault: an absent
///   measurement is not a bad one, and a tree drawn before the first
///   result would otherwise report every document as broken. That is
///   the reading the tests pin, not recovered intent — the function
///   is as old as the crate and its history settles nothing;
///   `review_gui3_r2::a_document_with_no_result_yet_reads_unevaluated_and_reports_no_faults`
///   is the reading held executably.
/// - [`RowStatus::Ok`] is not a fault, and needs no reason beyond
///   that: it is the state the other three are named against.
///
/// **A different axis from [`RowStatus::tone`]**, which is why it is a
/// second reading and not a call to that one: `tone` asks what a
/// reader can ACT on and leaves a poisoned row [`Tone::Advisory`],
/// pointing at the row that owns the failure. This asks whether the
/// document is building, and a poisoned row says it is not. Collapsing
/// the two would make a document whose only fault is downstream report
/// clean.
///
/// **And the opposite reading from [`crate::bounds::Verdict`]**, which
/// excludes poisoning for a reason argued at its own site — *"counting
/// it would make one failure register as many"*. Both readings are
/// right for their question: a verdict is a SET whose size decides
/// whether a value got worse, and a poisoned node inflates it; this is
/// a boolean, which nothing inflates. Where they can actually differ
/// is the one state that has a poisoned row with no failed row behind
/// it ([`RowStatus::Poisoned`] with `message: None`) — this calls that
/// document not building, and a verdict over it is empty. `Verdict`'s
/// own doc names this reading back, so each site states the other.
pub fn has_faults(rows: &[TreeRow]) -> bool {
    rows.iter().any(|row| match row.status {
        RowStatus::Failed { .. } | RowStatus::Poisoned { .. } => true,
        RowStatus::Unevaluated => false,
        RowStatus::Ok => false,
    })
}
