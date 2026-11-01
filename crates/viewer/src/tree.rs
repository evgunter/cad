//! The feature tree: the GQ2 result DAG, as rows a panel can draw.
//!
//! # Failures are values, and this module invents none of them
//!
//! GQ2's ratified codomain is a per-node result — `Ok`, `Failed(e)`,
//! `Poisoned { through }` — and G4's ratified error rule is that a
//! failure is a typed value the GUI renders, never a string invented
//! at the interaction layer. So a failing row's message is the
//! kernel's own rendering of the `NodeError`, its node spoken from the
//! document the row is drawn over (`NodeError::spoken`), and nothing
//! here composes a sentence
//! about what went wrong. The two sentences this module writes ABOUT
//! A FAILURE are a downstream row's pointer ([`downstream_wording`])
//! and a failed row's link to the node to repair ([`link_wording`]),
//! and both say only WHERE to go. What it writes about an assertion's
//! verdict is the comparison it decided ([`Asserted::comparison`]),
//! out of the kernel's own relation and two computed values.
//!
//! A failure that CARRIES another node's refusal — a part whose product
//! failed or was poisoned, a mate whose pose did not evaluate — points at the
//! node that failed and never quotes it, so the carried refusal is
//! drawn under the row as a line of its own, and so on down, one line
//! per document level ([`carried_lines`]). Each line is the kernel's
//! own rendering of that node's failure, byte for byte. What this
//! module adds is a label beside it, never inside it: WHICH document
//! the line's node number belongs to, by file name, since the kernel
//! knows a part only by its id.
//!
//! What it does write, and what the rule above does not reach, is what
//! a node IS: the document's spoken node (`Extrude 000000000003`, its
//! kind noun and tag, [`node_label`]), and which part an instance is
//! ([`part_file`]). Those are readings of the node, not verdicts about a run,
//! and they are sited here because the tree and the creation forms'
//! pickers have to name a node the same way.
//!
//! Because that is the other thing this module owns: the *shape* —
//! which rows exist, in which order, which of them the selection is
//! on, and which row a failure sends the eye to.
//!
//! One thing a run said that is no failure rides the row as well: what
//! an `Ok` value says ([`Readout`]) — a measure's value, spelled when
//! drawn as the chrome spells any computed value, or the kernel's typed
//! reason it has none; what an assertion found of it ([`Asserted`]);
//! and that a boolean, or a side of a split, holds no material.
//!
//! A placement's constraints are its children ([`ConstraintRow`]): a
//! constraint is no node, so its row hangs under its placement's.
//!
//! # A mate refusal poisons across the placement graph, not the DAG
//!
//! A constraint's reads say where a copy stands, not what it is made
//! of, so the placement solve is a computation the result DAG has no
//! edges for. When a placement's bundle refuses, the kernel records the
//! SAME typed fault against that placement and against every placement
//! chained to it, and each of those nodes reports it as its own
//! `Failed`. Read verbatim that draws several identical FAILED badges
//! and sends the eye nowhere.
//!
//! The fault itself resolves that wherever it names a constraint
//! (`blamed_mates` is the reading). So the placement holding a
//! constraint the fault BLAMES is the cause and keeps its `Failed`; a
//! row the same fault merely reached is [`RowStatus::Poisoned`] through
//! the blamed placement — the only thing read being which node the
//! kernel's own words point at.
//!
//! **This is the viewer's one answer to which row a node's failure
//! is.** Every other surface that says why a node has no value reads
//! it off [`cause_row`]: a picked face's verdict, a tool's refusal and
//! the pick index's tooltip hold the kernel's `NodeStanding` re-read by
//! [`standing_as_drawn`]. So no panel names a different row from the
//! tree's. Every kernel door under
//! `crates/viewer/src` that hands back a standing is censused by
//! `tree_badges::every_standing_door_in_the_viewer_reads_the_trees_answer`.
//!
//! **The blamed node is the row that carries the fault's words.** This
//! is the one statement of why; [`blamed_mates`] points here. Every arm
//! that names a constraint blames the placement holding it: a mate
//! reads poses, and a pose that does not evaluate is
//! [`MateFault::SideUnevaluated`], which carries the evaluation layer's
//! own refusal — drawn under the placement's row as a carried line
//! ([`carried_lines`]), never as a link to another node to repair.
//!
//! # A failed row outside the solve links where its own error says
//!
//! Every other `NodeErrorKind` is asked the same question
//! (`repair_named`), and a link means one thing: **another node whose
//! own slot refused**. No kind names one: an input the failing node
//! chose, which evaluated to a legal value that choice does not fit,
//! refused nothing and gets no link; its number is in the words, and it
//! is the failing node's own input, so its row is beside this one.
//!
//! # Order and columns
//!
//! Rows follow `Evaluation::order` — the evaluation's own
//! deterministic topological order, which is a pure function of the
//! DAG — so the tree a user reads is the order the kernel evaluated
//! in.
//!
//! **Every node row stands in one column.** In an indented list a row
//! reads as belonging to the less-indented row above it, and in
//! topological order the row above a node is whatever evaluated just
//! before it, which the node need not read. So no node row indents:
//! what does is what belongs to one row — a placement's constraints
//! ([`ConstraintRow`]) and the lines its result writes under it.
//!
//! Module kind: **vocabulary** — it names no driver type and no
//! `app`-only crate (`crates/viewer/README.md`, Module boundaries).

use pncad::document::{
    AssertionRelation, AssertionVerdict, BooleanValue, CarriedIn, Constraint, Doc, Evaluation,
    Label, MateFault, MateRef, MeasureUnavailableAt, Node, NodeError, NodeErrorKind, NodeResult,
    NodeStanding, ProfileProgram, RecipeNodeId, SplitSide, SpokenNode, ValuePayload,
    node_kind_noun,
};
use pncad::select::{
    InterrogateError, Resolution, ResolveIndeterminate, RunCtx, SplitHalf, resolve,
};

use crate::frame::Tone;
use crate::parts::PartFiles;
use crate::props::{Computed, Notation};
use crate::session::VersionOffer;

/// **One level of a failure's traceback**, as the tree draws it: the
/// document the level's node is in, as a label of its own, and the
/// node's refusal.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CarriedLine {
    /// The document, by file name ([`PartFiles::name`]), or
    /// [`THIS_DOCUMENT`] for the tree's own.
    pub document: String,
    /// The node's refusal exactly as its own tree draws it
    /// (`CarriedLevel::line_in`).
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
        /// `NodeError::spoken` over the tree's document.
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
    /// placement solve left without a pose because the bundle of a
    /// placement it is chained to refused, which the evaluation reports
    /// as its own `Failed` (the module header's second section).
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
    /// broken feature has one loud row, not seven. A mate refusal is
    /// actionable on the placements it blames ([`blamed_mates`]).
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
    /// The node as a sentence speaks it: its kind noun, its label when
    /// it has one, and its tag.
    pub spoken: SpokenNode,
    /// **Which one of its kind this node is**, when the node itself can
    /// say — an instance's part by its file name ([`part_file`]).
    /// `None` is a node kind that has no such sentence, not a sentence
    /// that came out empty.
    pub which: Option<String>,
    /// Whether a world placement places a body this node defines — the
    /// world badge (A10). A placement's own row is an ordinary row.
    pub placed: bool,
    /// What the evaluation said about it.
    pub status: RowStatus,
    /// **Another node whose own slot refused**, when a
    /// [`RowStatus::Failed`] row's error names one: whatever
    /// `repair_named` answers for the row's own error, never this row.
    /// That node's own row may read `Ok`.
    ///
    /// `None` on every row that is not `Failed`. A row's other links
    /// carry their own target: a `Poisoned` row's is its `through`,
    /// and an assertion's is its [`Asserted::measure`].
    pub repair_at: Option<SpokenNode>,
    /// **What this row's `Ok` value says** ([`Readout`]); `None` on a
    /// value that says nothing beyond `Ok`, and on every other row,
    /// whose status says why there is none.
    pub readout: Option<Readout>,
    /// **The accept a [`RowStatus::Failed`] instance row offers**, when
    /// its failure is a pin that no longer holds
    /// ([`crate::frame::version_offer`]); `None` on every other row, and
    /// on every row while a newer document's run is outstanding
    /// (`DocSession::tree_rows`).
    pub version_offer: Option<VersionOffer>,
    /// **A placement's constraints, its children** ([`ConstraintRow`]):
    /// a `Place`'s or an instance's bundle, in bundle order. Empty on
    /// every other row. A constraint is no node, so it has no row of its
    /// own in the order; its placement's row carries it.
    pub constraints: Vec<ConstraintRow>,
}

/// **One constraint of a placement, as a child of its row**: its
/// address and what it is.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ConstraintRow {
    /// The constraint's address: its placement and minted id.
    pub mate: MateRef,
    /// What the constraint is, in a few words ([`constraint_said`]).
    pub said: String,
}

/// **What a constraint is, as its child row says it**: its 1-based
/// position in its bundle (`at`), then `mate` for a mate, and the
/// freedom a value sets — `constraint 1: mate`,
/// `constraint 2: spin about z value`. Never its minted id, which is an arena
/// key and no word a person reads.
pub fn constraint_said(at: usize, constraint: &Constraint) -> String {
    match (constraint.mate(), constraint.value()) {
        (Some(_), _) => format!("constraint {at}: mate"),
        (None, Some(value)) => format!("constraint {at}: {} value", value.coord.name()),
        (None, None) => unreachable!("a constraint is a mate or a value"),
    }
}

/// `node`'s constraints as child rows, empty for a node that is no
/// placement.
fn constraint_rows(id: RecipeNodeId, node: &Node<ProfileProgram>) -> Vec<ConstraintRow> {
    match node {
        Node::Place { constraints, .. } | Node::InstantiatePart { constraints, .. } => constraints
            .iter()
            .enumerate()
            .map(|(at, constraint)| ConstraintRow {
                mate: MateRef::new(id, constraint.id),
                said: constraint_said(at + 1, constraint),
            })
            .collect(),
        _ => Vec::new(),
    }
}

impl TreeRow {
    /// **How loud this row is drawn**: its status's tone
    /// ([`RowStatus::tone`]), except on an assertion's row, which is
    /// `Ok` and as loud as its verdict ([`Asserted::tone`]).
    pub fn tone(&self) -> Tone {
        match &self.readout {
            Some(Readout::Asserted(asserted)) => asserted.tone(),
            Some(Readout::Value(_) | Readout::Unavailable(_) | Readout::Empty(_)) | None => {
                self.status.tone()
            }
        }
    }
}

/// **What an `Ok` value says**, on its own row: a measure's value or
/// the kernel's reason it has none, an assertion's verdict over that
/// value, a value that holds no material where the node could have
/// made some, and what a mate did in the solve.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Readout {
    /// The value, as a quantity: spelled when the row is DRAWN
    /// ([`Computed::spelled`]), in the working notation in force then,
    /// so a notation changed after the run re-spells it.
    Value(Computed),
    /// No value at this build's scalar — a value of the node, not a
    /// failure. Its `Display` is the kernel's sentence, which names
    /// the door that can answer.
    Unavailable(MeasureUnavailableAt),
    /// An assertion's verdict over its measure.
    Asserted(Asserted),
    /// A boolean's typed empty result, or a split with a side that
    /// holds no material. A legal value, and the one a consumer
    /// refuses as its input, so the row it came from says so.
    Empty(Emptiness),
}

/// **Which part of a value holds no material**, and its phrase.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Emptiness {
    /// The whole value: a boolean's typed empty result, or a split
    /// with neither side.
    Whole,
    /// One side of a split.
    Half(SplitHalf),
}

impl std::fmt::Display for Emptiness {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Whole => f.write_str("empty"),
            Self::Half(half) => write!(f, "{} half empty", split_half_label(*half)),
        }
    }
}

/// The word for a half of the KERNEL's [`SplitHalf`]: a body seat
/// holding a split draws one button per entry of `SplitHalf::ALL` with
/// it, and an empty half's readout ([`Emptiness`]) names the half with
/// it.
///
/// **A match, not a table**, for the reason `forms::boolean_op_label`
/// is: the enum is declared in `topo`, so no list written here can be
/// projected from its declaration. A third half would arrive with no
/// membership edit here and could not arrive silently, because it has
/// no word until this match gives it one.
///
/// The words are the kernel's own sides — the plane's normal decides
/// which is which, and neither surface paraphrases that.
pub(crate) fn split_half_label(half: SplitHalf) -> &'static str {
    match half {
        SplitHalf::Above => "above",
        SplitHalf::Below => "below",
    }
}

/// **An assertion's verdict, as its row says it**: the kernel's
/// verdict with both numbers carried as the measure's own value is
/// ([`Computed`], in the measure's dimension, spelled when drawn), the
/// relation it states to the bound, and which measure that is.
#[derive(Clone, Debug, PartialEq)]
pub struct Asserted {
    /// The landed verdict.
    pub verdict: AssertionVerdict<Computed>,
    /// How the value must relate to the bound.
    pub relation: AssertionRelation,
    /// The measure under the value the assertion bounds — the
    /// `min_clearance` one when there is one, since that is the measure
    /// a point scalar cannot answer — or `None` for a value no measure
    /// is under.
    pub measure: Option<SpokenNode>,
}

// `AssertionVerdict` derives `PartialEq` alone, for its scalar's sake;
// over bitwise-compared numbers and an `Eq` reason it is an equivalence.
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

    /// **The comparison the verdict decided**, as a report reads it
    /// in `notation` — `0.0125 m >= 0.01 m` — or `None` where there is
    /// no verdict. Both numbers read in the one notation, whatever the
    /// bound was written in, so the two sides of one comparison never
    /// read in two.
    pub fn comparison(&self, notation: Notation) -> Option<String> {
        match &self.verdict {
            AssertionVerdict::Holds { measured, bound }
            | AssertionVerdict::Violated { measured, bound } => Some(format!(
                "{} {} {}",
                measured.spelled(notation),
                self.relation.symbol(),
                bound.spelled(notation)
            )),
            AssertionVerdict::Unevaluated { .. } => None,
        }
    }
}

/// **How the chrome names one node**: the node as the document speaks
/// it ([`SpokenNode`]), and what the node itself says about which one
/// of its kind it is (an instance's part file, [`part_file`]).
///
/// The one spelling a tree row, a picker entry and a properties heading
/// share ([`named`]): the spoken node is what every refusal calls a
/// node by, and the part file is the half that tells two instances
/// apart.
pub fn node_label(doc: &Doc<ProfileProgram>, id: RecipeNodeId, files: &PartFiles) -> String {
    let which = doc.node(id).and_then(|node| part_file(node, files));
    named(&doc.spoken(id), which.as_deref())
}

/// A spoken node and which one of its kind it is, as [`node_label`]
/// draws them: `InstantiatePart 000000000002 — bracket.pncad`, or
/// `InstantiatePart "left" (000000000002) — bracket.pncad` when the
/// node is labelled.
pub fn named(spoken: &SpokenNode, which: Option<&str>) -> String {
    match which {
        Some(which) => format!("{spoken} — {which}"),
        None => spoken.to_string(),
    }
}

/// **A feature-tree row's headline**, in its two voices: `lead` drawn
/// as the row's text, and `muted` beside it, quieter.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Headline {
    /// What the row reads as.
    pub lead: String,
    /// What it says beside that, in a muted voice — `None` when the
    /// lead already says everything.
    pub muted: Option<String>,
}

/// **What a feature-tree row's headline says** (DESIGN.md Band 1,
/// "Node labels"): a labelled node leads with its label, with its kind
/// and tag muted beside it (`base plate` · `Extrude 3fa9c1d2a0b1`); an
/// unlabelled one reads as [`named`] says it, kind, tag and which.
#[must_use]
pub fn headline(spoken: &SpokenNode, which: Option<&str>) -> Headline {
    match (spoken.label(), spoken.kind()) {
        (Some(label), Some(kind)) => Headline {
            lead: label.to_string(),
            muted: Some(format!("{kind} {}", spoken.id())),
        },
        _ => Headline {
            lead: named(spoken, which),
            muted: None,
        },
    }
}

/// **The label a create form proposes** for a new node of kind `noun`
/// ([`node_kind_noun`]'s word): `Kind N`, where N counts the live nodes
/// of that kind at this moment, plus one — three extrudes, labelled or
/// not, propose `Extrude 4` — stepping past any N whose `Kind N` a node
/// of that kind already carries, so the proposal repeats no label of
/// its kind. Only a proposal: the kernel mints no label, and this is
/// stored only if the person commits it. `None` for a `noun` no label
/// can hold (one with a control character).
#[must_use]
pub fn proposed_label(doc: &Doc<ProfileProgram>, noun: &str) -> Option<Label> {
    let of_kind = || {
        doc.ids().into_iter().filter(|&id| {
            doc.node(id)
                .is_some_and(|node| node_kind_noun(node) == noun)
        })
    };
    let taken: Vec<&str> = of_kind()
        .filter_map(|id| doc.label(id))
        .map(Label::as_str)
        .collect();
    let text = (of_kind().count() + 1..)
        .map(|n| format!("{noun} {n}"))
        .find(|text| !taken.contains(&text.as_str()))?;
    Label::new(text).ok()
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
        None => doc.ids().to_vec(),
    };
    let mut rows = Vec::with_capacity(order.len());
    for id in order {
        // The world is no feature: it is in every document, is never
        // deleted, and only a placement's target reads it, so it has no
        // row.
        let Some(node) = doc.node(id).filter(|node| !matches!(node, Node::World)) else {
            continue;
        };
        let status = status_of(doc, id, evaluation, files);
        let (repair_at, readout, version_offer) = match status {
            RowStatus::Failed { .. } => (
                evaluation.and_then(|ev| repair_of(id, ev).map(|at| doc.spoken(at))),
                None,
                evaluation
                    .and_then(|ev| own_error(id, ev))
                    .and_then(|error| crate::frame::version_offer(&error.kind, files)),
            ),
            RowStatus::Ok => (
                None,
                evaluation.and_then(|ev| readout_of(doc, id, node, ev)),
                None,
            ),
            RowStatus::Poisoned { .. } | RowStatus::Unevaluated => (None, None, None),
        };
        rows.push(TreeRow {
            id,
            spoken: doc.spoken(id),
            which: part_file(node, files),
            placed: crate::world::is_placed(doc, id),
            status,
            repair_at,
            readout,
            version_offer,
            constraints: constraint_rows(id, node),
        });
    }
    rows
}

/// **What `id`'s value says in `evaluation`** — asked only of a row
/// [`rows`] has read `Ok`, so this decides which payload, never
/// whether there is one. `None` for a payload whose `Ok` is the whole
/// of it: a body, a profile, a family's members (its count is
/// authored, and the words of any refusal state it), a
/// placement's copy, the world.
fn readout_of(
    doc: &Doc<ProfileProgram>,
    id: RecipeNodeId,
    node: &Node<ProfileProgram>,
    evaluation: &Evaluation<f64>,
) -> Option<Readout> {
    match &evaluation.usable(id).ok()?.payload {
        ValuePayload::Measure { value, dim } => Some(Readout::Value(Computed {
            canonical: *value,
            dimension: *dim,
        })),
        ValuePayload::MeasureUnavailable { reason, .. } => Some(Readout::Unavailable(*reason)),
        ValuePayload::Assertion(verdict) => Some(Readout::Asserted(asserted(doc, node, verdict))),
        ValuePayload::Boolean(BooleanValue::Empty) => Some(Readout::Empty(Emptiness::Whole)),
        ValuePayload::Split { above, below } => match (above, below) {
            (SplitSide::Empty, SplitSide::Empty) => Some(Readout::Empty(Emptiness::Whole)),
            (SplitSide::Empty, SplitSide::Body(_)) => {
                Some(Readout::Empty(Emptiness::Half(SplitHalf::Above)))
            }
            (SplitSide::Body(_), SplitSide::Empty) => {
                Some(Readout::Empty(Emptiness::Half(SplitHalf::Below)))
            }
            (SplitSide::Body(_), SplitSide::Body(_)) => None,
        },
        ValuePayload::Body(_)
        | ValuePayload::Boolean(BooleanValue::Body { .. })
        | ValuePayload::Profile(_)
        | ValuePayload::Instances(_)
        | ValuePayload::Family => None,
    }
}

/// **An assertion's verdict, its numbers carried in its value's
/// dimension.** The value is a scalar variable, and its kind's
/// dimension is the one both numbers are in.
fn asserted(
    doc: &Doc<ProfileProgram>,
    node: &Node<ProfileProgram>,
    verdict: &AssertionVerdict<f64>,
) -> Asserted {
    let Node::Assertion {
        value, relation, ..
    } = node
    else {
        unreachable!("only an assertion node evaluates to a verdict")
    };
    let dim = || match doc.var(*value).and_then(|var| var.kind().dimension()) {
        Some(dim) => dim,
        None => unreachable!("an assertion's value is a scalar variable the document holds"),
    };
    let measures: Vec<RecipeNodeId> = doc
        .observed_outputs(*value)
        .into_iter()
        .filter_map(|output| doc.operation_of(output))
        .collect();
    let clearance = measures.iter().copied().find(|&measure| {
        matches!(
            doc.node(measure),
            Some(Node::Measure {
                primitive: editor_core::MeasurePrimitive::MinClearance { .. }
            })
        )
    });
    Asserted {
        verdict: verdict.clone().map(|number| Computed {
            canonical: number,
            dimension: dim(),
        }),
        relation: *relation,
        measure: clearance
            .or_else(|| measures.first().copied())
            .map(|measure| doc.spoken(measure)),
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
/// Each line is that node's refusal exactly as its own tree draws it,
/// a node of `doc` spoken from it and a part's node as the pinned part
/// holds it ([`pncad::document::CarriedLevel::line_in`]); the document
/// it is in is its label ([`CarriedLine::document`]).
pub fn carried_lines(
    doc: &Doc<ProfileProgram>,
    kind: &NodeErrorKind,
    files: &PartFiles,
) -> Vec<CarriedLine> {
    kind.carried_chain()
        .map(|level| CarriedLine {
            document: match level.document {
                CarriedIn::ThisDocument => THIS_DOCUMENT.to_owned(),
                CarriedIn::Part { doc_ref, .. } => files.name(doc_ref.id).to_owned(),
            },
            line: level.line_in(doc),
        })
        .collect()
}

/// **Where a row stands**, before anything is drawn of it: the
/// structural fact, whose words are drawn only where they are shown
/// ([`status_of`]).
enum Standing<'e> {
    /// No result in the evaluation, or no evaluation.
    Unevaluated,
    /// A usable value.
    Ok,
    /// The row's own failure, and the evaluation that raised it.
    Failed(&'e NodeError, &'e Evaluation<f64>),
    /// Drawn downstream of `through`; `cause_known` is whether the
    /// chain ends at a failure, which is what earns the row its
    /// pointer ([`downstream_wording`]).
    Downstream {
        through: RecipeNodeId,
        cause_known: bool,
    },
}

fn standing(id: RecipeNodeId, evaluation: Option<&Evaluation<f64>>) -> Standing<'_> {
    let Some(ev) = evaluation else {
        return Standing::Unevaluated;
    };
    match ev.result(id) {
        None => Standing::Unevaluated,
        Some(NodeResult::Ok(_)) => Standing::Ok,
        Some(NodeResult::Failed(error)) => {
            downstream_of_mate(id, error).unwrap_or(Standing::Failed(error, ev))
        }
        Some(NodeResult::Poisoned { through }) => poisoned_through(*through, ev),
    }
}

/// One node's status, read out of the result DAG.
fn status_of(
    doc: &Doc<ProfileProgram>,
    id: RecipeNodeId,
    evaluation: Option<&Evaluation<f64>>,
    files: &PartFiles,
) -> RowStatus {
    match standing(id, evaluation) {
        Standing::Unevaluated => RowStatus::Unevaluated,
        Standing::Ok => RowStatus::Ok,
        Standing::Downstream {
            through,
            cause_known,
        } => RowStatus::Poisoned {
            through,
            message: cause_known.then(|| downstream_wording(&doc.spoken(through))),
        },
        Standing::Failed(error, evaluation) => RowStatus::Failed {
            message: error.spoken(doc, evaluation),
            carried: carried_lines(doc, &error.kind, files),
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
/// caller that says "node N, which failed" cannot be handed an `N`
/// the tree does not badge failed. That arm is not expected to be
/// reachable — the evaluation names a failed ancestor as `through` —
/// and it is refused rather than assumed for the same reason the tree
/// reports it as absence.
pub fn cause_row(id: RecipeNodeId, evaluation: &Evaluation<f64>) -> Option<RecipeNodeId> {
    match standing(id, Some(evaluation)) {
        Standing::Failed(..) => Some(id),
        Standing::Downstream {
            through,
            cause_known: true,
        } => Some(through),
        Standing::Downstream {
            cause_known: false, ..
        }
        | Standing::Ok
        | Standing::Unevaluated => None,
    }
}

/// **`id`'s own failure, as this tree reads it**: `Some` exactly when
/// its row is drawn from its own error, off the same [`standing`] as
/// [`status_of`]. A poisoned node answers `None` — its cause is an
/// ancestor's, and so is the row the tree sends a reader to.
pub fn own_error(id: RecipeNodeId, evaluation: &Evaluation<f64>) -> Option<&NodeError> {
    match standing(id, Some(evaluation)) {
        Standing::Failed(error, _) => Some(error),
        Standing::Ok | Standing::Unevaluated | Standing::Downstream { .. } => None,
    }
}

/// **A kernel standing, re-read as this tree draws its node** (the
/// module header's second section).
///
/// The kernel reports a placement a bundle refusal reached as its own
/// `Failed`, and a node poisoned through such a node as poisoned
/// through it; the tree draws both as downstream of the placement the
/// fault blames. Answered off [`cause_row`]: a node whose cause is another
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

/// **A picked name's [`Resolution`], as drawn** ([`resolution_as_drawn`]):
/// read in the read holding the body the pick met (NAMES N1: resolution
/// names the read; `Doc::read_of_body`). A pick whose node the document
/// no longer holds is stranded with it where the name was minted there
/// (`stranded_at`), and is otherwise not in the document; one whose
/// node defines no such read answers the node's standing.
pub fn pick_resolution(
    doc: &pncad::document::Doc<ProfileProgram>,
    evaluation: &Evaluation<f64>,
    node: RecipeNodeId,
    body: u32,
    name: &pncad::prelude::StableName,
) -> Resolution {
    match pncad::select::stranded_at(doc, node, name) {
        Some(Ok(gone)) => return gone,
        // The node took its reads with it, and a pick keeps no read.
        Some(Err(_)) => {
            return Resolution::Indeterminate(ResolveIndeterminate {
                standing: NodeStanding::NotInDocument { node },
            });
        }
        None => {}
    }
    match doc.read_of_body(node, body) {
        Some(read) => resolution_as_drawn(
            resolve(
                RunCtx {
                    doc,
                    eval: evaluation,
                },
                read,
                name,
            )
            .unwrap_or_else(|refusal| {
                unreachable!("a pick met a body, read in a body output: {refusal}")
            }),
            evaluation,
        ),
        None => Resolution::Indeterminate(ResolveIndeterminate {
            standing: standing_as_drawn(
                evaluation
                    .usable(node)
                    .err()
                    .unwrap_or(NodeStanding::NotInDocument { node }),
                evaluation,
            ),
        }),
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
        | InterrogateError::SeveralOutputs { .. }
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
/// The row pointed at is named as the document speaks it
/// ([`SpokenNode`]): this sentence is chrome, drawn in the tree.
pub fn downstream_wording(through: &SpokenNode) -> String {
    format!("upstream failure at {through} — that row carries the cause")
}

/// **What a link to a node says**: the node as the document speaks it,
/// and nothing about why — the why is drawn elsewhere, so this names
/// only WHERE to go.
pub fn link_wording(at: &SpokenNode) -> String {
    format!("see {at}")
}

/// Another node whose own slot refused, when a `Failed` row's error
/// names one.
fn repair_of(id: RecipeNodeId, ev: &Evaluation<f64>) -> Option<RecipeNodeId> {
    repair_named(&ev.result(id)?.error()?.kind).filter(|at| *at != id)
}

/// **Which node an evaluation error names whose own slot refused**,
/// as against a node the words mention as evidence or as the input
/// the failing node chose. No kind names one: each arm says why.
fn repair_named(kind: &NodeErrorKind) -> Option<RecipeNodeId> {
    match kind {
        // A mate refusal is the placement's: the pose that did not
        // evaluate is carried in its own node's words (module header).
        NodeErrorKind::Mate(_) => None,
        // A member's refusal names what that member's own refusal names.
        NodeErrorKind::MemberRefused { source, .. } => repair_named(source),
        // The named input evaluated to a legal value that the failing
        // node's own choice does not fit — a family the operand does
        // not take, an empty body or split side, an index outside the
        // family's members — and nothing on it refused.
        NodeErrorKind::WrongOperand { .. }
        // An operand reads an output its operation no longer defines:
        // the repair is a re-point at the reading node itself.
        | NodeErrorKind::UnresolvedRead { .. }
        | NodeErrorKind::EmptyOperand { .. }
        | NodeErrorKind::EmptyHalf { .. }
        | NodeErrorKind::MemberOutOfRange { .. } => None,
        // Names an id no live node holds, so there is no row to go to.
        NodeErrorKind::MissingInput { .. } => None,
        // The input's door shipped a body that does not finish: a
        // kernel defect, and no author's slot refused.
        NodeErrorKind::UnfinishedOperand { .. } => None,
        // Names the failing instance itself.
        NodeErrorKind::CrossingUnverified { .. } => None,
        // Names the two spaces the operands live in, as evidence: the
        // repair is the reading node's own operands, or a placement's
        // target, and one link would pick for the reader.
        NodeErrorKind::SpaceMismatch { .. } => None,
        // A pick's path that reaches no piece of the copy, or a piece
        // of the wrong shape: the pick's own path is the repair.
        NodeErrorKind::NoSuchPiece { .. } | NodeErrorKind::PieceShape { .. } => None,
        // A payload that names a node does so as evidence: a name's
        // minting node, where the repair is the referring node's own
        // reference; an upstream table the naming pass found missing;
        // or a node in ANOTHER document's id space (`PartFault`), which
        // no row of this tree is.
        NodeErrorKind::Part { .. }
        | NodeErrorKind::SelectResolve { .. }
        | NodeErrorKind::MeasureRefUnreadable { .. }
        | NodeErrorKind::Naming(_) => None,
        // The member names the step that refused, not the node to
        // repair: the refusal is the union's.
        NodeErrorKind::UnionFoldStep { .. } => None,
        // Name no node beside the failing one.
        NodeErrorKind::Expr { .. }
        | NodeErrorKind::PoseRead { .. }
        | NodeErrorKind::PoseDegenerate { .. }
        | NodeErrorKind::PoseScalar { .. }
        | NodeErrorKind::Profile(_)
        | NodeErrorKind::ProfileReplay { .. }
        | NodeErrorKind::ProfileLaneReplay { .. }
        | NodeErrorKind::Trace(_)
        | NodeErrorKind::Extrude(_)
        | NodeErrorKind::Revolve(_)
        | NodeErrorKind::Tube(_)
        | NodeErrorKind::Split(_)
        | NodeErrorKind::Blend { .. }
        | NodeErrorKind::Boolean(_)
        | NodeErrorKind::Transform(_)
        | NodeErrorKind::Mirror(_)
        | NodeErrorKind::Motion(_)
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
        | NodeErrorKind::Escalated { .. }
        | NodeErrorKind::MeasureEscalated { .. }
        | NodeErrorKind::PlacementAxisEscalated { .. }
        | NodeErrorKind::PoseDirectionEscalated { .. }
        | NodeErrorKind::CountOutOfRange { .. }
        | NodeErrorKind::IndicesUnrelated { .. }
        | NodeErrorKind::IndexCount { .. }
        | NodeErrorKind::IndexCountOutOfRange { .. }
        | NodeErrorKind::UnschedulableCycle
        | NodeErrorKind::Shell(_)
        | NodeErrorKind::ShellLaneUnsupported { .. }
        | NodeErrorKind::WitnessBifurcation(_)
        | NodeErrorKind::MeasureNonFinite { .. }
        | NodeErrorKind::MeasureNotParallel { .. }
        | NodeErrorKind::MeasureUnsupported(_)
        | NodeErrorKind::PayloadExpr { .. }
        | NodeErrorKind::MeasureClearanceRefused(_)
        | NodeErrorKind::AssertionDimension { .. } => None,
    }
}

/// The status of a row the EVALUATION poisoned, given the nearest
/// failed ancestor it named.
///
/// That ancestor is `Failed` in the run, but the tree may redraw its
/// row as downstream itself — reachably: a union over the copy of an
/// instance chained to a bundle that then refuses is poisoned through
/// that instance, whose own row now points at the refused placement.
/// Two hops, one of them a row with nothing to act on, so this carries
/// the same cause that row does. One step settles it: the placement of
/// a constraint the fault names keeps its own `Failed`.
fn poisoned_through(through: RecipeNodeId, ev: &Evaluation<f64>) -> Standing<'_> {
    let Some(error) = ev.result(through).and_then(NodeResult::error) else {
        // The chain does not end at a failure: report the absence.
        return Standing::Downstream {
            through,
            cause_known: false,
        };
    };
    downstream_of_mate(through, error).unwrap_or(Standing::Downstream {
        through,
        cause_known: true,
    })
}

/// The placements a solve refusal BLAMES — the placements holding the
/// constraints the fault's own words point the user at. A constraint
/// is no node, so its fault and its row are its placement's.
///
/// Every arm that names a constraint blames its placement, whatever
/// else it names; why that holds for an arm naming a node an author
/// may repair is stated once, in the module header's second section.
///
/// One arm names none: the mispairing, which reaches no row.
fn blamed_mates(fault: &MateFault) -> Vec<RecipeNodeId> {
    match fault {
        MateFault::NotOnTheCopy { mate }
        | MateFault::Unleverable { mate, .. }
        | MateFault::KindsDiffer { mate, .. }
        | MateFault::Backwards { mate, .. }
        | MateFault::Unoriented { mate }
        | MateFault::SideUnevaluated { mate, .. } => vec![mate.placement],
        MateFault::NoChart { value, .. } => vec![value.placement],
        // A claim about one bundle: its placement is the row.
        MateFault::Overconstrained { placement, .. } | MateFault::Unpinned { placement, .. } => {
            vec![*placement]
        }
        // Names no mate and reaches NO row (`MateFault`'s doc says
        // why); the empty answer here is unreachable, not a reading.
        MateFault::PosesOfAnotherDocument { .. } => Vec::new(),
    }
}

/// The downstream reading of a node's own `Failed`, when a mate
/// refusal reached it without naming it.
///
/// `None` — the row keeps its own `Failed` — when the failure is not
/// a mate refusal, when the fault names this very node, and when it
/// names no mate at all.
///
/// **The blame is read directly**, and [`RowStatus::Poisoned`]'s
/// walkable-in-one-hop invariant holds because the kernel's answer is
/// consistent: a placement's content key carries the solve's answer, so
/// the placement of a constraint the fault names is `Failed` in the
/// evaluation carrying that fault — never `Ok` off a stale memo. A row here that pointed at a green row would
/// be that inconsistency surfacing, not a case to absorb.
fn downstream_of_mate(id: RecipeNodeId, error: &NodeError) -> Option<Standing<'static>> {
    let NodeErrorKind::Mate(fault) = &error.kind else {
        return None;
    };
    let blamed = blamed_mates(fault);
    if blamed.contains(&id) {
        return None;
    }
    // The first placement the fault names, in the fault's own order.
    let through = blamed.into_iter().next()?;
    Some(Standing::Downstream {
        through,
        cause_known: true,
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
