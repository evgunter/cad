//! The evaluation service (M4 PR 2; ratified F2 shape, spec D2–D6):
//! `evaluate(doc) → Evaluation<T>` — the result DAG, memoized
//! incremental recompute on content keys, cooperative cancelation with
//! epochs, and D9-addendum idiom-1 parallelism over independent nodes.
//!
//! Layering (spec D1): this module is where editor-core joins the
//! kernel op crates (`profile`, `sweep`, `topo`) — editor-core sits
//! ABOVE the kernel (G1); the kernel crates gain no editor-core
//! dependency.
//!
//! Failure semantics (spec D2, GQ2 ratified): a node failure poisons
//! its DESCENDANTS ONLY; independent subgraphs complete. Kernel errors
//! are wrapped UNALTERED (no stringification) with (node, slot)
//! context — including PR 1's banked `NonFiniteResult` obligation:
//! every expression evaluated during node evaluation carries the node
//! and slot it came from. One kernel error is generic over the lane
//! scalar, the shell's, and it crosses through a TOTAL fold to `f64`
//! (`NodeErrorKind::Shell`): every arm and every number kept, each
//! number at the bracket end the lane declares — never rendered, never
//! dropped, and at `f64` the identity.

mod anchor;
mod class;
pub mod measure;
mod memo;
pub(crate) mod parts;

pub use parts::PartFault;
pub(crate) mod schedule;
pub(crate) mod slots;
mod wire;
pub(crate) use wire::{
    DECISION_WORDS, EVAL_DIRECTION_NORM, PATTERN_SPACING, PATTERN_STEP, PATTERN_STEP_TURN,
};

pub(crate) use wire::unit as unit_direction;

pub(crate) use anchor::derive_naming;
pub use anchor::{
    CanonicalSegment, LoopAnchor, PiecesFault, ProfileNaming, ProfilePieces, ProfileValue,
};
pub use class::NodeErrorClass;
pub use memo::{ContentBits, ContentKey, KeyHasher, NamingKey};
pub use wire::DirectionRefusal;

use std::collections::BTreeMap;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};

use geom_core::{Decide, Indeterminate};
use profile::ProfileError;
use sweep::{ExtrudeError, RevolveError, SkinError, TubeError};
use topo::splitting::SplitError;
use topo::transform::TransformError;
use topo::{Body, BooleanError, BooleanResultKind, ContactRecords};

use crate::appearance::{self, AppearanceResolution};
use crate::doc::Doc;
use crate::expr::EvalError;
use crate::ident::Mispaired;
use crate::names::{NameTable, NamingError, SegTag};
use crate::node::{PartSelect, RecipeNodeId, SlotId, StableName};
use crate::program::ProfileProgram;
use geom_core::Tol;

/// The result DAG (F2 verbatim, spec D2): a deterministic order plus a
/// per-node result map, with the run's epoch, the id of the document
/// it is OF, its outcome, and the D4-acceptance recompute counters.
#[derive(Debug)]
pub struct Evaluation<T: Decide> {
    /// This evaluation's identity token (spec D5) — the caller's
    /// stale-result discrimination hook.
    pub epoch: Epoch,
    /// **Which document this is an evaluation OF** (DI3): the id
    /// [`Doc::id`] answered when the run started. Identity only, no
    /// pin: within one document the per-node content keys decide
    /// reuse, so the version half would cost a canonicalization per
    /// run for a check the keys already make.
    ///
    /// The pairing doors read this field to refuse a mispairing typed,
    /// before reading anything of the value; the memo reads it too and
    /// refuses differently, below. Node ids alone could not decide any
    /// of it: they are minted from the edits that inserted them, so two
    /// documents built from one recipe carry the SAME ids for the same
    /// nodes, and every lookup would hit.
    ///
    /// Which doors those are, and which doors taking such a pair do
    /// not check it yet, is `crates/editor-core/ASSEMBLY.md`'s A2a —
    /// one place for a set that grows as each door is built.
    ///
    /// The field is `pub` like every other field of this struct, so a
    /// caller CAN restamp it. That is a deliberate act, not a slip, and
    /// nothing downstream re-derives it.
    pub document: crate::ident::DocumentId,
    /// **A prior of another document, refused** (DI3): `Some` when
    /// `evaluate` was handed a `prior` whose [`Evaluation::document`]
    /// is not this document's, in which case the run had NO memo —
    /// every node recomputed and [`Evaluation::reused`] is zero. The
    /// payload is the same [`Mispaired`] the two erroring doors carry,
    /// because the question is the same one.
    ///
    /// `evaluate` is total (it returns no `Result`) and [`EvalOutcome`]
    /// is completed-or-canceled, so the refusal is recorded on the
    /// value the caller already reads rather than pushed into an
    /// outcome arm that would mean something else. `None` is both "no
    /// prior" and "a prior of this document".
    pub prior_refused: Option<Mispaired>,
    /// Deterministic topological order of the live nodes (spec D2:
    /// a pure function of the document; Kahn's algorithm, ties to the
    /// lesser id, the node inserted first). Always the FULL order, even when
    /// canceled — order is data, not schedule.
    pub order: Vec<RecipeNodeId>,
    /// Per-node results. On cancelation this holds the completed
    /// prefix only.
    pub nodes: BTreeMap<RecipeNodeId, NodeResult<T>>,
    /// Completed or canceled (spec D5's typed partial result).
    pub outcome: EvalOutcome,
    /// How many nodes actually ran their op this evaluation (spec D4's
    /// acceptance counter: the downstream cone of an edit).
    pub recomputed: usize,
    /// How many nodes were reused from the prior evaluation by
    /// content-key match.
    pub reused: usize,
    /// **Every node whose value lives in a loose copy's space** (D10,
    /// Spaces), with the loose copy's placement. A node absent here
    /// lives in the world's space, or denotes no geometry.
    /// [`crate::product::product`] gathers the world's alone, and the
    /// at-rest census checks each space by itself.
    pub loose: BTreeMap<RecipeNodeId, RecipeNodeId>,
    /// How many REFERENCED documents this evaluation actually
    /// evaluated across the document seam (ASM-2A D-3's sharing
    /// evidence). N instances of one part contribute 1; a part the
    /// prior evaluation carried contributes 0, however its instances
    /// changed; and a nested crossing — a part that instantiates a
    /// part — contributes to THIS count too, so the number is the whole
    /// run's seam traffic rather than one level's.
    pub part_evaluations: usize,
    /// The document's appearance store resolved against THIS
    /// evaluation's name tables (M4 PR 7): per node, entity →
    /// attributes, so a renderer/exporter consumes appearance without
    /// touching tables; unresolved entries are LOUD typed losses.
    /// Appearance never enters content keys (presentation metadata):
    /// an appearance-only edit re-resolves this field and recomputes
    /// zero nodes.
    pub appearance: AppearanceResolution,
    /// The lane environment this run read its slots from: what
    /// [`Self::reading`] reads a variable's free inputs from.
    pub env: crate::expr::VarEnv<T>,
    /// Each output variable of the evaluated document and the port that
    /// defines it: what [`Self::table`] reads a variable through.
    reads: Arc<BTreeMap<crate::VarId, (RecipeNodeId, u8)>>,
    /// The parts this run entered and evaluated, carried for the run
    /// that names it as its prior.
    parts: parts::Carried<T>,
}

impl<T: Decide> Evaluation<T> {
    /// **The name table of the variable `var`** (NAMES N1, the scope):
    /// the names of the bodies that variable holds, a split's half its
    /// own. The one door to a table: a name is scoped by the variable
    /// that holds its body, so nothing reads a node's names as one set.
    /// `None` for a variable no output defines, one whose node has no
    /// value in this run, and a port that carries no names (a revolve's
    /// axis).
    pub fn table(&self, var: crate::VarId) -> Option<&NameTable> {
        self.output_names(var).map(OutputNames::table)
    }

    /// [`Self::table_at`], shared: for a reader that keeps the table
    /// beside the value it read (the product's gather).
    pub(crate) fn shared_table_at(&self, at: impl Into<At>) -> Option<Arc<NameTable>> {
        self.output_names(self.at_var(at)?)
            .map(OutputNames::shared_table)
    }

    /// The fragment groups minted beside [`Self::table`]'s table.
    pub fn groups(&self, var: crate::VarId) -> Option<&crate::names::FragmentGroups> {
        self.output_names(var).map(OutputNames::groups)
    }

    /// The output and the port `var` reads, as the evaluated document
    /// defined it.
    pub fn read_port(&self, var: crate::VarId) -> Option<(RecipeNodeId, u8)> {
        self.reads.get(&var).copied()
    }

    /// The variable `node`'s port `port` defines, in the evaluated
    /// document.
    pub fn output_var(&self, node: RecipeNodeId, port: u8) -> Option<crate::VarId> {
        self.reads
            .iter()
            .find(|(_, at)| **at == (node, port))
            .map(|(&var, _)| var)
    }

    /// **The variables of `node` that carry names**, in port order: one
    /// for a node with one body output, two for a split.
    pub fn named_outputs(&self, node: RecipeNodeId) -> Vec<crate::VarId> {
        let Some(value) = self.value(node) else {
            return Vec::new();
        };
        let mut ports: Vec<(u8, crate::VarId)> = self
            .reads
            .iter()
            .filter(|(_, (at, port))| *at == node && value.output(*port).is_some())
            .map(|(&var, &(_, port))| (port, var))
            .collect();
        ports.sort_unstable();
        ports.into_iter().map(|(_, var)| var).collect()
    }

    /// **The variable `at` names**: a read as it is; a node named alone
    /// its one output that carries names (`None` for a split, whose
    /// halves are two, and for a node with no value).
    pub fn at_var(&self, at: impl Into<At>) -> Option<crate::VarId> {
        match at.into() {
            At::Read(var) => Some(var),
            At::Node(node) => match self.named_outputs(node).as_slice() {
                [one] => Some(*one),
                _ => None,
            },
        }
    }

    /// [`Self::table`] of the variable `at` names ([`Self::at_var`]).
    pub fn table_at(&self, at: impl Into<At>) -> Option<&NameTable> {
        self.table(self.at_var(at)?)
    }

    /// **The body entity `index` of `var`'s table is in**: a split's half
    /// at its port (its table's one body), any other output's body at
    /// `index` (a family's member at its place).
    ///
    /// # Errors
    ///
    /// The node's standing when it has no value; a
    /// [`crate::InterrogateError`] when the value holds no such body.
    pub fn read_body(
        &self,
        var: crate::VarId,
        index: u32,
    ) -> Result<Result<&Body<T>, crate::InterrogateError>, NodeStanding> {
        let Some((node, port)) = self.read_port(var) else {
            return Ok(Err(crate::InterrogateError::NoSuchBody { index }));
        };
        let value = self.usable(node)?;
        Ok(match &value.payload {
            ValuePayload::Split { .. } if index == 0 => value.payload.output_body(u32::from(port)),
            ValuePayload::Split { .. } => Err(crate::InterrogateError::NoSuchBody { index }),
            payload => payload.output_body(index),
        })
    }

    /// **The read and the table entity an entity of `node`'s value
    /// is** — [`Self::read_body`]'s inverse: a split's body `i` is its
    /// half at port `i`, body 0 of that half's table; any other node's
    /// entity is its one named output's as it stands.
    pub fn entity_read(
        &self,
        node: RecipeNodeId,
        entity: crate::names::EntityRef,
    ) -> Option<(crate::VarId, crate::names::EntityRef)> {
        let (var, body) = self.body_read_of(node, entity.body)?;
        Some((
            var,
            crate::names::EntityRef {
                body,
                key: entity.key,
            },
        ))
    }

    /// **The read body `body` of `node`'s value is in, and its index
    /// in that read's table**: a split's body `i` is its half at port
    /// `i`, body 0 there; any other node's body is its one named
    /// output's at the same index.
    pub fn body_read_of(&self, node: RecipeNodeId, body: u32) -> Option<(crate::VarId, u32)> {
        match &self.value(node)?.payload {
            ValuePayload::Split { .. } => {
                Some((self.output_var(node, u8::try_from(body).ok()?)?, 0))
            }
            _ => Some((self.at_var(node)?, body)),
        }
    }

    fn output_names(&self, var: crate::VarId) -> Option<&OutputNames> {
        let (node, port) = self.read_port(var)?;
        self.value(node)?.output(port)
    }

    /// **The node's value, or its standing** — the one read every door
    /// that needs a node's value makes, so the three ways to have none
    /// are spelled once, here, and a door's refusal carries the
    /// [`NodeStanding`] rather than re-spelling it.
    ///
    /// # Errors
    ///
    /// The node's [`NodeStanding`]: no result in this evaluation,
    /// failed, or poisoned through its nearest failed ancestor.
    pub fn usable(&self, id: RecipeNodeId) -> Result<&NodeValue<T>, NodeStanding> {
        usable_in(&self.nodes, id, || {
            if self.order.contains(&id) {
                NodeStanding::NotEvaluated { node: id }
            } else {
                NodeStanding::NotInDocument { node: id }
            }
        })
    }

    /// **Whether two nodes' values live in different spaces** (D10,
    /// Spaces) — the one predicate every door comparing two nodes'
    /// geometry asks: `Some` naming the two spaces, `None` when both
    /// live in one.
    pub fn across_spaces(
        &self,
        a: RecipeNodeId,
        b: RecipeNodeId,
    ) -> Option<(crate::mate::Space, crate::mate::Space)> {
        let (sa, sb) = (self.space(a), self.space(b));
        (sa != sb).then_some((sa, sb))
    }

    /// **The space `node`'s value lives in** (D10, Spaces), read off
    /// [`Evaluation::loose`]: the world's for a node absent there.
    pub fn space(&self, node: RecipeNodeId) -> crate::mate::Space {
        self.loose
            .get(&node)
            .map_or(crate::mate::Space::World, |&root| {
                crate::mate::Space::Loose { root }
            })
    }

    /// **Every loose copy**, by its placement, in id order: a loose
    /// copy lives in its own space, so each is its own row of
    /// [`Evaluation::loose`].
    pub fn loose_roots(&self) -> Vec<RecipeNodeId> {
        self.loose
            .iter()
            .filter(|&(node, root)| node == root)
            .map(|(_, &root)| root)
            .collect()
    }

    /// The node's successful value, if it has one — [`Evaluation::usable`]
    /// for a reader to whom every standing is the same answer.
    pub fn value(&self, id: RecipeNodeId) -> Option<&NodeValue<T>> {
        self.usable(id).ok()
    }

    /// The node's result as typed data, for a reader that needs what
    /// [`NodeStanding`] does not carry — a failed node's own
    /// [`NodeError`] beside its value's absence (LIB-DOORS F3). `None`
    /// means the id has no entry at all. A door that refuses on a node
    /// with no value reads [`Evaluation::usable`] instead, which tells
    /// "the run stopped before it" from "not in this document".
    pub fn result(&self, id: RecipeNodeId) -> Option<&NodeResult<T>> {
        self.nodes.get(&id)
    }

    /// **A scalar variable's value in this run** (D10): at this run's
    /// scalar, each measured value under it the one its measure computed
    /// here, every other input the lane environment's
    /// ([`measure::Observed`]). A variable no measure is under is that
    /// environment's reading of it.
    ///
    /// # Errors
    ///
    /// [`measure::ObservedRefusal`]: a measure under it with no value in
    /// this run, or an evaluation that refuses — a variable of no scalar
    /// kind, or one `doc` does not hold, among them.
    pub fn reading<P: crate::ProfilePayload>(
        &self,
        doc: &Doc<P>,
        var: crate::VarId,
    ) -> Result<measure::Observed<T>, measure::ObservedRefusal> {
        let dim = doc.var(var).and_then(|v| v.kind().dimension()).ok_or(
            measure::ObservedRefusal::Expr(EvalError::UnresolvedVar { var }),
        )?;
        measure::observe(doc, &self.env, &self.nodes, var, dim)
    }

    /// **`formula` read in this evaluation**, measured values bound:
    /// [`Self::reading`] for a formula no variable holds, such as a
    /// recorded measurement's `value` (`Recording::measure`).
    ///
    /// # Errors
    ///
    /// As [`Self::reading`].
    pub fn reading_formula<P: crate::ProfilePayload>(
        &self,
        doc: &Doc<P>,
        formula: &crate::Formula,
    ) -> Result<measure::Observed<T>, measure::ObservedRefusal> {
        measure::observe_formula(doc, &self.env, &self.nodes, formula)
    }

    /// [`Self::reading`], its refusal rendered with the node it came
    /// from: a failed measure's own error spoken from `doc`, a poisoned
    /// one's failed ancestor's, and otherwise the standing; an
    /// evaluation's refusal is said at the first measure under `var`,
    /// or none when no measure is. The one ladder every reader of a
    /// value outside an assertion takes.
    pub(crate) fn reading_spoken<P: crate::ProfilePayload>(
        &self,
        doc: &Doc<P>,
        var: crate::VarId,
    ) -> Result<measure::Observed<T>, (Option<RecipeNodeId>, String)>
    where
        T: Copy,
    {
        self.reading(doc, var).map_err(|refusal| match refusal {
            measure::ObservedRefusal::Measure(standing) => {
                self.node_error(standing.node()).map_or_else(
                    || (Some(standing.node()), standing.to_string()),
                    |e| (Some(e.node), e.kind_spoken(doc)),
                )
            }
            measure::ObservedRefusal::Expr(source) => (
                doc.observed_outputs(var)
                    .first()
                    .and_then(|out| doc.operation_of(*out)),
                source.to_string(),
            ),
        })
    }

    /// The typed root cause behind a node that produced no value:
    /// `Failed` answers its own error; `Poisoned` answers the nearest
    /// failed ancestor's (one `through` hop — see
    /// [`NodeResult::Poisoned`]'s invariant). `None` for a node that
    /// succeeded or has no entry.
    pub fn node_error(&self, id: RecipeNodeId) -> Option<&NodeError> {
        match self.usable(id) {
            Ok(_) | Err(NodeStanding::NotEvaluated { .. } | NodeStanding::NotInDocument { .. }) => {
                None
            }
            // Every `through` names a `Failed` entry (the poison
            // propagation writes nothing else there); answering `None`
            // on a broken invariant is fail-honest — the caller sees
            // "no root cause", not a wrong one.
            Err(standing) => self.nodes.get(&standing.failed_node()?)?.error(),
        }
    }
}

/// **Why a node has no value in an evaluation** — its standing, the
/// one vocabulary every door that reads a node's value refuses in.
///
/// Answered by [`Evaluation::usable`]. A door carries it as its
/// refusal's payload under its own subject; its `Display` names the
/// standing — which node, what state, where the repair is — and no
/// door.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NodeStanding {
    /// The node is in this evaluation's order but has no result: the
    /// run was canceled before it reached the node.
    NotEvaluated {
        /// The node.
        node: RecipeNodeId,
    },
    /// The id is not a node of the document this evaluation ran over.
    NotInDocument {
        /// The id asked about.
        node: RecipeNodeId,
    },
    /// The node itself failed ([`Evaluation::node_error`] answers the
    /// typed cause).
    Failed {
        /// The failed node.
        node: RecipeNodeId,
    },
    /// An ancestor failed, so the node never ran.
    Poisoned {
        /// The node.
        node: RecipeNodeId,
        /// Its nearest failed ancestor ([`NodeResult::Poisoned`]).
        through: RecipeNodeId,
    },
}

impl NodeStanding {
    /// The node the standing is of.
    #[must_use]
    pub fn node(self) -> RecipeNodeId {
        match self {
            Self::NotEvaluated { node }
            | Self::NotInDocument { node }
            | Self::Failed { node }
            | Self::Poisoned { node, .. } => node,
        }
    }

    /// The nearest failed ancestor, if the node was poisoned.
    #[must_use]
    pub fn through(self) -> Option<RecipeNodeId> {
        match self {
            Self::Poisoned { through, .. } => Some(through),
            Self::NotEvaluated { .. } | Self::NotInDocument { .. } | Self::Failed { .. } => None,
        }
    }

    /// The node whose failure explains the standing, which is where
    /// the repair is: the node itself when it failed, its nearest
    /// failed ancestor when it was poisoned, and `None` when the node
    /// has no entry.
    #[must_use]
    pub fn failed_node(self) -> Option<RecipeNodeId> {
        match self {
            Self::Failed { node } | Self::Poisoned { through: node, .. } => Some(node),
            Self::NotEvaluated { .. } | Self::NotInDocument { .. } => None,
        }
    }

    /// The standing of a document ROOT, as a door whose subject is the
    /// document's roots states it under its own stage word: `root`,
    /// then the standing.
    pub(crate) fn of_placement(self) -> PlacementStanding {
        PlacementStanding(self)
    }
}

/// [`NodeStanding::of_placement`]'s rendering: the one sentence for a
/// placement with no value.
pub(crate) struct PlacementStanding(NodeStanding);

impl crate::spoken::Say for PlacementStanding {
    fn say(
        &self,
        f: &mut core::fmt::Formatter<'_>,
        by: crate::spoken::Speaker<'_>,
    ) -> core::fmt::Result {
        write!(f, "placement {}", crate::spoken::Said(&self.0, by))
    }
}

impl core::fmt::Display for PlacementStanding {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        crate::spoken::Say::say(self, f, crate::spoken::Speaker::TAG)
    }
}

impl crate::spoken::Say for NodeStanding {
    fn say(
        &self,
        f: &mut core::fmt::Formatter<'_>,
        by: crate::spoken::Speaker<'_>,
    ) -> core::fmt::Result {
        match *self {
            Self::NotEvaluated { node } => write!(
                f,
                "{} has no result in this evaluation: the run was canceled before it \
                 reached the node — re-evaluate the document to completion",
                by.node(node)
            ),
            Self::NotInDocument { node } => write!(
                f,
                "{} is not a node of the document this evaluation ran over — ask about \
                 one of that document's nodes, or evaluate the document the node is in",
                by.node(node)
            ),
            Self::Failed { node } => write!(
                f,
                "{} failed, so it has no value — fix the node's own failure",
                by.node(node)
            ),
            Self::Poisoned { node, through } => {
                let through = by.node(through);
                write!(
                    f,
                    "{} is poisoned by the failure at {through}, so it has no value — the \
                     repair is upstream, at {through}",
                    by.node(node)
                )
            }
        }
    }
}

/// The standing where no document is at hand: each node by its tag.
impl core::fmt::Display for NodeStanding {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        crate::spoken::Say::say(self, f, crate::spoken::Speaker::TAG)
    }
}

impl NodeStanding {
    /// **The standing as the frame holding the evaluated document says
    /// it**: each node as `doc` holds it now ([`Doc::spoken`]). A
    /// standing is answered by an evaluation and carried inside values
    /// the evaluation memo reuses, so it holds ids, never a label.
    #[must_use]
    pub fn spoken<P: crate::ProfilePayload>(&self, doc: &Doc<P>) -> String {
        crate::spoken::spoken_by(self, doc)
    }
}

impl core::error::Error for NodeStanding {}

/// [`Evaluation::usable`]'s ladder over a result map, for the readers
/// that hold the map before the [`Evaluation`] exists: the op wiring's
/// input read and the appearance pass. `absent` answers an id the map
/// has no entry for, because only the caller knows whether that id is
/// in the run's order.
pub(crate) fn usable_in<T: Decide>(
    nodes: &BTreeMap<RecipeNodeId, NodeResult<T>>,
    node: RecipeNodeId,
    absent: impl FnOnce() -> NodeStanding,
) -> Result<&NodeValue<T>, NodeStanding> {
    match nodes.get(&node) {
        Some(NodeResult::Ok(value)) => Ok(value),
        Some(NodeResult::Failed(_)) => Err(NodeStanding::Failed { node }),
        Some(NodeResult::Poisoned { through }) => Err(NodeStanding::Poisoned {
            node,
            through: *through,
        }),
        None => Err(absent()),
    }
}

/// **An assertion's evaluated payload** ([`crate::node::payload_exprs`]):
/// its value and its bound, in that order, every expression reading a
/// measure with no value at this scalar left out and its position kept
/// in `unavailable`.
#[derive(Debug)]
pub(crate) struct Payload<T> {
    /// The values, in payload order, the unavailable ones left out.
    pub(crate) values: Vec<T>,
    /// The first expression with no value at this scalar: its position
    /// in payload order, and why.
    pub(crate) unavailable: Option<(usize, crate::measure::MeasureUnavailableAt)>,
}

/// Whether the evaluation ran to completion (spec D5).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EvalOutcome {
    /// Every scheduled node has a result.
    Completed,
    /// The cancel token was observed set at a yield point; `nodes`
    /// holds the completed prefix.
    Canceled,
}

/// One node's outcome (F2 verbatim, spec D2).
#[derive(Debug)]
pub enum NodeResult<T: Decide> {
    /// The node evaluated to a value.
    Ok(NodeValue<T>),
    /// The node itself failed — the typed root cause.
    Failed(NodeError),
    /// An ancestor failed; this node never ran (GQ2: descendants
    /// only). `through` names the NEAREST FAILED ancestor — not
    /// necessarily the root cause of a longer chain, but every
    /// `through` points at a `Failed` entry, so the chain is walkable
    /// in one hop per poisoned run.
    Poisoned {
        /// The nearest failed ancestor.
        through: RecipeNodeId,
    },
}

impl<T: Decide> NodeResult<T> {
    /// The successful value, if this result is `Ok`.
    pub fn value(&self) -> Option<&NodeValue<T>> {
        match self {
            Self::Ok(v) => Some(v),
            Self::Failed(_) | Self::Poisoned { .. } => None,
        }
    }

    /// The typed failure, if this node ITSELF failed. A poisoned
    /// node answers `None` — its own entry carries no error; the root
    /// cause lives at [`NodeResult::poisoned_through`]'s target (or
    /// ask [`Evaluation::node_error`], which walks the hop).
    pub fn error(&self) -> Option<&NodeError> {
        match self {
            Self::Failed(e) => Some(e),
            Self::Ok(_) | Self::Poisoned { .. } => None,
        }
    }

    /// The nearest failed ancestor, if this node was poisoned.
    pub fn poisoned_through(&self) -> Option<RecipeNodeId> {
        match self {
            Self::Poisoned { through } => Some(*through),
            Self::Ok(_) | Self::Failed(_) => None,
        }
    }
}

/// **Where a name table is read** (NAMES N1, the scope): a read of an
/// output variable, or a node named alone, which is its one output
/// that carries names.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum At {
    /// A node named alone: its one named output.
    Node(RecipeNodeId),
    /// An output variable.
    Read(crate::VarId),
}

impl From<RecipeNodeId> for At {
    fn from(node: RecipeNodeId) -> Self {
        Self::Node(node)
    }
}

impl From<crate::VarId> for At {
    fn from(var: crate::VarId) -> Self {
        Self::Read(var)
    }
}

/// **The names one output variable carries** (N1, N4): its name
/// table — every boundary entity of the output's bodies,
/// `StableName ↔` (body, arena key) — and the fragment groups the
/// emitter formed beside it ([`crate::names::FragmentGroups`], the
/// membership the diagnosis ladder's group-size rung counts; not
/// persisted, like the verdict log).
#[derive(Debug, Clone, Default)]
pub struct OutputNames {
    table: Arc<NameTable>,
    groups: Arc<crate::names::FragmentGroups>,
}

impl OutputNames {
    /// An output's names and groups.
    pub(crate) fn new(table: Arc<NameTable>, groups: Arc<crate::names::FragmentGroups>) -> Self {
        Self { table, groups }
    }

    /// The output's name table.
    pub fn table(&self) -> &NameTable {
        &self.table
    }

    /// The fragment groups minted beside it.
    pub fn groups(&self) -> &crate::names::FragmentGroups {
        &self.groups
    }

    /// The table, shared.
    pub(crate) fn shared_table(&self) -> Arc<NameTable> {
        Arc::clone(&self.table)
    }
}

/// A successful node value (spec D2): the op-appropriate payload plus
/// the reserved witness slot and the node's content key.
#[derive(Debug, Clone)]
pub struct NodeValue<T: Decide> {
    /// What the node's op produced.
    pub payload: ValuePayload<T>,
    /// **The names each output carries** (NAMES N1, the scope), by
    /// port: a name is scoped by the variable that holds its body, so
    /// a split's two halves are two tables and an operation with one
    /// body output has one. Reached only through
    /// [`Evaluation::table`] (and, in the evaluator, the read it
    /// resolves to): a table is the variable's, never the node's.
    /// Rides the value, so memo reuse transfers names with geometry
    /// (the content key is the proof).
    outputs: Arc<[OutputNames]>,
    /// The DECLARED CONTACT RECORDS the node's output body 0 carries
    /// (ASM-R2b D-1's contacts channel; [`crate::eval::wire::OpOut`]
    /// states the invariant). Empty for every op but instantiate — a
    /// boolean's records ride its payload, and
    /// `product::sources_of` is where the two homes
    /// reconcile. Rides the value, so memo reuse transfers
    /// declarations with the geometry they are keyed into.
    pub contacts: Arc<topo::ContactRecords>,
    /// The MATE BOOKKEEPING those records travel with (`ASSEMBLY.md`
    /// A5): which mate of which document below authored each of them,
    /// and which mates of those documents could not be minted at all.
    /// Keyed in the same arena as `contacts` and filled at the same
    /// one op, so the gather re-keys both through the graft's own
    /// descendant map. Rides the value, so memo reuse transfers mate
    /// identity with the geometry it is keyed into.
    pub carried: Arc<crate::assembly::CarriedDeclarations>,
    /// **The coincidences the node's op decided from values**
    /// ([`crate::coincide`]), in decision order, each cell named in the
    /// table of the input the decision read, so a row survives a later
    /// merge or split of its cells. A function of the evaluation, as
    /// the verdicts are, so no content key reads it; it rides the value
    /// with the names it is spelled in.
    pub coincidences: Arc<[crate::coincide::NamedCoincidence]>,
    /// **What each input its contact records cite is**: a record's
    /// [`topo::Backing::Carried`] names an input by position, and this
    /// is that position's input, by read ([`crate::coincide::CitedInput`]).
    /// A [`topo::Backing::Decided`] names a row of
    /// [`Self::coincidences`].
    pub cited_inputs: Arc<[crate::coincide::CitedInput]>,
    /// How many parts each of this value's output bodies is
    /// (`crates/editor-core/ASSEMBLY.md`, A2 and A10): a document's
    /// product is its roots, so an instantiation's bodies are as many
    /// parts as the referenced document's root outputs, each counted at
    /// its own `parts` — a sub-assembly's parts count through. Two root
    /// outputs that are a split's two halves are two parts. The placers
    /// (`Transform`, `Pattern`) and `Part` carry the count through; every
    /// other op builds a body of its own and counts 1, an explicit union
    /// (the fuse) included. More than one makes the value a PRODUCT,
    /// which every op that fuses or reshapes a single body operand
    /// refuses ([`NodeErrorKind::ProductOperand`]); a reader of its
    /// geometry (a datum's face frame) reads it as it is.
    pub parts: usize,
    /// **A placement's solved motion** (D10): where its copies sit
    /// relative to the shapes they copy, `None` for the bit-exact
    /// identity and for every value that is no placement's. A carried
    /// pose reads it ([`crate::pose::PoseDef::Carried`]).
    pub motion: Option<geom_core::RigidMotion<T>>,
    /// **The space the value lives in** (D10, Spaces): a placement's
    /// copy its bundle's, every other value its inputs', `None` for a
    /// value that lives in none (a measure's, an assertion's).
    pub space: Option<crate::mate::Space>,
    /// The node's verdict log (M4 PR 4, N5): every definite predicate
    /// decision made evaluating the node — those made before its
    /// content key (a profile's f64 precompute: the plane read, the
    /// replay, the validation) and the op's alike — in the order made,
    /// recorded through the one
    /// `k_stats` funnel. Scalar-independent data —
    /// same verdicts at f64 and Interval — and the diff-engine
    /// substrate ("both evaluations' verdict logs exist"). Rides the
    /// value, so memo reuse transfers the log with the geometry it
    /// certified (same content key ⇒ same decisions, D9).
    ///
    /// **This log is not persisted, and neither is the strict form
    /// derived from it.** The one persisted projection is
    /// [`crate::resolve::VerdictSummary`] — per-predicate sign
    /// populations, written by [`crate::resolve::verdict_summary`] and
    /// carried by the strict codecs in `crate::persist::strict` — so a
    /// consumer that needs a run's decisions across a process boundary
    /// summarizes; one that needs them in hand reads this.
    pub verdicts: Arc<VerdictLog>,
    /// The node's escalation log: every INDETERMINATE predicate outcome
    /// met evaluating the node, in decision order, recorded in the same
    /// `k_stats` frame as the verdicts. Non-empty on a value only when
    /// the node absorbed an escalation and built anyway — a leaf the
    /// subdivision driver must not certify (E6: every predicate
    /// definite), which is what it reads this for. Rides the value with
    /// the verdicts, for the same reason.
    ///
    /// **Not persisted**, like the verdicts and unlike their summary:
    /// an escalation is a fact about one run at one box, read in hand.
    pub escalations: Arc<EscalationLog>,
    /// RESERVED empty slot: the solved witness assignment, filled by
    /// the sketch solver `docs/DESIGN.md` `[open.sketch-solver]` adopts
    /// when constraint-driven sketches have a consumer.
    pub witness: WitnessSlot,
    /// The node's input-content hash (spec D4) — the memo currency.
    pub content_key: ContentKey,
    /// The node's recursive NAMING key (issue #95 disposition 2, M4
    /// PR 5): memo reuse of the whole value requires BOTH keys to
    /// match — content certifies the geometry, naming certifies the
    /// node-id-embedding names half (see [`NamingKey`]).
    pub naming_key: NamingKey,
}

impl<T: Decide> NodeValue<T> {
    /// **The names output `port` carries**: the one indexer of
    /// [`NodeValue::outputs`], private to the evaluator. Everything
    /// outside it reads a table by the variable that holds its body
    /// ([`Evaluation::table`]).
    fn output(&self, port: u8) -> Option<&OutputNames> {
        self.outputs.get(usize::from(port))
    }
}

impl<T: Decide> NodeValue<T> {
    /// **A value read for one pose and never kept**: an op's output with
    /// no log, no key and no space, which a placement's fold reads its
    /// own copy before placement through and drops.
    pub(crate) fn scratch(out: wire::OpOut<T>) -> Self {
        Self {
            payload: out.payload,
            outputs: out.outputs.into(),
            contacts: out.contacts,
            carried: out.carried,
            coincidences: out.coincidences,
            cited_inputs: out.cited_inputs,
            parts: out.parts,
            verdicts: Arc::new(Vec::new()),
            escalations: Arc::new(Vec::new()),
            motion: None,
            space: None,
            witness: WitnessSlot::default(),
            content_key: ContentKey(0),
            naming_key: NamingKey(0),
        }
    }
}

/// **Hand-built values and evaluations** (`test-support`): for rows that
/// probe a door on a table no evaluation mints. Each node's names are
/// its one output's, read through a variable minted from its id.
#[cfg(any(test, feature = "test-support"))]
impl NodeValue<f64> {
    /// An empty boolean's value whose one output carries `table` and
    /// `groups`, with `verdicts` as its log.
    #[must_use]
    pub fn hand_built(
        table: NameTable,
        groups: crate::names::FragmentGroups,
        verdicts: VerdictLog,
    ) -> Self {
        Self {
            payload: ValuePayload::Boolean(BooleanValue::Empty),
            outputs: vec![OutputNames::new(Arc::new(table), Arc::new(groups))].into(),
            contacts: Arc::new(topo::ContactRecords::default()),
            carried: Arc::new(crate::assembly::CarriedDeclarations::default()),
            coincidences: Arc::new([]),
            cited_inputs: Arc::new([]),
            parts: 1,
            verdicts: Arc::new(verdicts),
            escalations: Arc::new(vec![]),
            placement: None,
            motion: None,
            space: None,
            witness: WitnessSlot::default(),
            content_key: ContentKey(0),
            naming_key: NamingKey(0),
        }
    }

    /// The value with output `port`'s table replaced by `table`.
    ///
    /// # Panics
    ///
    /// If the value has no output `port`.
    pub fn replace_table(&mut self, port: u8, table: NameTable) {
        let mut outputs = self.outputs.to_vec();
        let at = &mut outputs[usize::from(port)];
        *at = OutputNames::new(Arc::new(table), Arc::clone(&at.groups));
        self.outputs = outputs.into();
    }
}

#[cfg(any(test, feature = "test-support"))]
impl Evaluation<f64> {
    /// A completed evaluation of `document` holding `nodes` in `order`,
    /// each node's one output read through the variable [`Self::hand_read`]
    /// mints for it.
    #[must_use]
    pub fn hand_built(
        document: crate::DocumentId,
        order: Vec<RecipeNodeId>,
        nodes: BTreeMap<RecipeNodeId, NodeResult<f64>>,
    ) -> Self {
        let reads = nodes
            .keys()
            .map(|&node| (Self::hand_read(node), (node, 0)))
            .collect();
        Self {
            epoch: crate::Epoch::mint(),
            loose: BTreeMap::new(),
            document,
            prior_refused: None,
            recomputed: order.len(),
            order,
            nodes,
            outcome: EvalOutcome::Completed,
            reused: 0,
            part_evaluations: 0,
            appearance: AppearanceResolution::default(),
            env: crate::expr::VarEnv::default(),
            reads: Arc::new(reads),
            parts: parts::Carried::none(),
        }
    }

    /// The variable a hand-built evaluation reads `node`'s output by.
    #[must_use]
    pub fn hand_read(node: RecipeNodeId) -> crate::VarId {
        crate::VarId(node.0)
    }
}

/// One evaluation's per-node verdict vector (the [`NodeValue::verdicts`]
/// payload): [`geom_core::k_stats::Verdict`]s in decision order.
pub type VerdictLog = Vec<geom_core::k_stats::Verdict>;

/// One node's escalations (the [`NodeValue::escalations`] and
/// [`NodeError::escalations`] payload):
/// [`geom_core::k_stats::Escalation`]s in decision order.
pub type EscalationLog = Vec<geom_core::k_stats::Escalation>;

/// The sketch solver's solved-assignment slot, as a type stub (spec
/// D2): no solver is in the tree, and `docs/DESIGN.md`
/// `[open.sketch-solver]` adopts one when constraint-driven sketches
/// have a consumer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct WitnessSlot {}

/// The op-appropriate value a node evaluates to (spec D2's
/// "`Vec<Body<T>>`-shaped output (op-appropriate)", made typed per op
/// family; spec D3 fixes each node's semantics).
#[derive(Debug, Clone)]
pub enum ValuePayload<T: Decide> {
    /// A datum evaluated to geometry values (D3: frames/axes as
    /// values; directions normalized, degenerate refused).
    Datum(PoseValue<T>),
    /// A validated profile (D3: replayed from the node's program
    /// through the driver, then the profile crate's validation door),
    /// its naming anchor, and the radius expression each
    /// of its edges is authored at ([`ProfileValue`]).
    Profile(Arc<ProfileValue<T>>),
    /// A single body: every one-body op (extrude, revolve, the tubes,
    /// loft, sweep, blends, shell, union, placed union, instantiate,
    /// `Part`) and a `Transform` of one body — a transform's value has
    /// its input's shape, so a transform of instances is
    /// [`Self::Instances`].
    Body(Arc<Body<T>>),
    /// A boolean's result: a body with its contact records, or the
    /// typed empty success (F8; D3).
    Boolean(BooleanValue<T>),
    /// Split's BOTH parts, role-tagged by field (D3).
    Split {
        /// Material on the tool plane's normal side.
        above: SplitSide<T>,
        /// Material on the opposite side.
        below: SplitSide<T>,
    },
    /// A pattern's instances AS DATA (D3: patterns do not implicitly
    /// union; index `i` is the A8/N1 `Instance(i)` substrate — for a
    /// nested pattern the flat index `j·M + i`, `wire_pattern`'s
    /// layout).
    ///
    /// **Who takes it, and who refuses.** The placers take it WHOLE
    /// and are shape-preserving over it: `Transform` yields the same
    /// instances under one rigid map, `Pattern` yields their `N·M`
    /// placements, both as `Instances`, because a rigid map of N
    /// bodies is N rigid maps and needs no guess. `Part` takes one
    /// instance out of it by index, and the product gather takes
    /// every instance in order. Every other consumer of a body
    /// operand — the set is `wire::read_body`'s callers (a datum's
    /// face frame) and `wire::body_operand`'s (a blend's and a shell's
    /// body, a split's target, a boolean's and a union's members, a
    /// placed union's prototype) —
    /// takes ONE body and refuses this value typed (`WrongOperand`):
    /// a boolean of N bodies is N booleans or one union of them, a
    /// blend of N bodies is N blends, and the recipe does not guess
    /// which (D3), so the asymmetry between the placers and the rest
    /// is the decision, not an omission.
    Instances(Vec<Arc<Body<T>>>),
    /// **A placement's copies, keyed** ([`Copies`]): a `Place`'s over
    /// several shapes, keyed by read, and an instance's, the part's
    /// world copies keyed by their identity in the part. A `Place` of
    /// one shape has that shape's value ([`ValuePayload::Body`] or
    /// [`ValuePayload::Instances`]).
    Copies(Arc<Copies<T>>),
    /// A `Measure` node's typed F1 quantity (E3): the measured value
    /// in kernel units with the dimension it was measured in. Not
    /// body-denoting — the product gather skips it exactly as it skips
    /// a declaration.
    Measure {
        /// The measured value, in canonical kernel units.
        value: T,
        /// Its F1 dimension, carried FROM the expression rather than
        /// re-derived: the quantity kind rides the expression (E3), so
        /// a reader never has to reconstruct it from the number.
        dim: crate::expr::Dimension,
    },
    /// **A `Measure` node that has no value AT THIS SCALAR** — a typed
    /// absence, and the honest answer where the alternative would be a
    /// number nothing backs.
    ///
    /// One primitive produces it today
    /// ([`crate::MeasurePrimitive::MinClearance`], whose answer is an
    /// ENCLOSURE the interval lane computes and a point scalar has no
    /// channel for), and the reason it is a value rather than a node
    /// error is the E10 shape: an assertion over it must report
    /// [`crate::AssertionVerdict::Unevaluated`] — the third state,
    /// used for exactly what it exists for — where a node error would
    /// poison the assertion into having no verdict at all (F2), and
    /// would additionally refuse every f64 build of a document
    /// carrying such a measure, `drive`'s witness build included.
    ///
    /// It carries the dimension for the same reason
    /// [`Self::Measure`] does: a reader knows WHAT was not measured.
    MeasureUnavailable {
        /// Why there is no value, in the measure layer's own words.
        reason: crate::measure::MeasureUnavailableAt,
        /// The F1 dimension the measure would have had.
        dim: crate::expr::Dimension,
    },
    /// An `Assertion` node's verdict (E10). REPORT-ONLY: no op in the
    /// vocabulary takes a verdict as an operand, so this payload is
    /// consumed by reports and by nothing else.
    Assertion(crate::measure::AssertionVerdict<T>),
}

/// **One family word, as a literal** — so [`concat!`] can compose a
/// phrase out of it at compile time, which a `const` cannot be fed
/// to. [`family`]'s consts are defined FROM this macro and
/// [`phrase`]'s are composed from it, so each word is spelled once in
/// the tree and a composed phrase cannot drift from the `found:` word
/// that answers beside it.
// OPERAND-VOCABULARY BEGIN — the region
// `every_family_word_has_exactly_one_const` reads. An arm with no
// const, or a const with no arm, reds that row.
macro_rules! family_word {
    (datum) => {
        "datum"
    };
    (profile) => {
        "profile"
    };
    (body) => {
        "body"
    };
    (boolean) => {
        "boolean"
    };
    (split) => {
        "split"
    };
    (instances) => {
        "instances"
    };
    (copies) => {
        "copies"
    };
    (measure) => {
        "measure"
    };
    (assertion) => {
        "assertion"
    };
}

/// **The family words** — the vocabulary a typed operand mismatch
/// speaks ([`NodeErrorKind::WrongOperand`]'s `found` and `expected`),
/// written once. [`ValuePayload::kind_name`] says them over a value,
/// and `eval::wire`'s operand door says them in the refusals it builds.
pub(crate) mod family {
    pub(crate) const DATUM: &str = family_word!(datum);
    pub(crate) const PROFILE: &str = family_word!(profile);
    pub(crate) const BODY: &str = family_word!(body);
    pub(crate) const BOOLEAN: &str = family_word!(boolean);
    pub(crate) const SPLIT: &str = family_word!(split);
    pub(crate) const INSTANCES: &str = family_word!(instances);
    pub(crate) const COPIES: &str = family_word!(copies);
    pub(crate) const MEASURE: &str = family_word!(measure);
    pub(crate) const ASSERTION: &str = family_word!(assertion);
}
// OPERAND-VOCABULARY END

/// **The composed phrases** — every `expected:` a refusal names that is
/// not exactly one family word.
///
/// # The rule
///
/// An `expected:` names what to author, and it comes from a const:
/// [`family`] when it is exactly a value family, this module when it is
/// anything else. **No `expected:` is a literal written at a call
/// site.** The reason is not that two copies of a two-word phrase are
/// expensive to keep in step — they are not — it is that the phrases a
/// document author has to learn are then enumerable in one screen,
/// instead of being the set you get by grepping every refusal that
/// speaks one.
///
/// `found:` never appears here. The door computes it from the value it
/// was handed ([`ValuePayload::kind_name`]), so no site can answer it with the negation of
/// its own `expected:` and leave a reader told twice what the input is
/// not and never what it is.
///
/// # The three shapes, and how each is composed
///
/// - **Narrower than a family** ([`phrase::DATUM_FRAME`], [`phrase::DATUM_AXIS`],
///   [`phrase::DATUM_PLANE`]): a variant WITHIN a family. The family word is
///   still in the phrase — and is exactly the word `found:` answers
///   beside it — so it is composed from `family_word!` rather than
///   respelled.
/// - **Wider than a family** ([`phrase::BODY_OR_INSTANCES`]): two families and
///   the conjunction between them, and nothing else; both words are
///   composed.
///
/// # What this module is NOT, and where the neighbouring words live
///
/// The rule above governs `expected:` and nothing else. A second
/// user-visible vocabulary sits beside it — the DIRECTION-ROLE words a
/// [`NodeErrorKind::DegenerateDirection`] or
/// [`NodeErrorKind::NonFiniteDirection`] refusal carries, which name the SLOT
/// whose vector would not normalize rather than the kind an operand
/// had to be. They keep their own home beside the arithmetic that
/// raises them (`eval::wire`'s `*_ROLE` consts, `pub(crate)` because
/// the mate solve re-derives the same refusals), and every one of them
/// is a named const rather than a literal at its call site — that half
/// of the rule they do follow.
///
/// **What they do NOT do is compose: they RESPELL.** Four of them open
/// with a phrase declared here and write it out again as a literal —
/// `"datum frame x axis"`, `"datum frame y axis"`, `"datum plane
/// normal"`, `"datum axis direction"`. That is not a boundary and not
/// a choice: `concat!` takes literals and a `const` is not one, so
/// composing them needs this module's macro layer extended from the
/// WORDS to the PHRASES, which is a design step rather than a rename.
/// It is a residue, and it has a row —
/// `work/wire/direction-role-words-respell-the-operand-phrases.md`.
/// `TRANSFORM_AXIS_ROLE` and `PATTERN_DIRECTION_ROLE` share no family
/// word with anything here, so they are not that row.
pub(crate) mod phrase {
    /// A frame datum: [`crate::node::Datum::Frame`] or
    /// [`crate::node::Datum::FaceFrame`], the two nodes that carry a
    /// [`super::PoseValue::Frame`].
    pub(crate) const DATUM_FRAME: &str = concat!(family_word!(datum), " frame");
    /// A 3-D axis datum ([`crate::node::Datum::Axis`]).
    pub(crate) const DATUM_AXIS: &str = concat!(family_word!(datum), " axis");
    /// A plane datum ([`crate::node::Datum::Plane`]).
    pub(crate) const DATUM_PLANE: &str = concat!(family_word!(datum), " plane");
    /// What a placer places: one body, or a list of placed ones.
    pub(crate) const BODY_OR_INSTANCES: &str =
        concat!(family_word!(body), " or ", family_word!(instances));
}

impl<T: Decide> ValuePayload<T> {
    /// **The body at output `index`**: a one-body value's at 0, a
    /// split's half at its port ([`crate::SplitHalf::output_body`]), a
    /// family's member at its place. The one answer to "which body does
    /// a read of this output hold".
    ///
    /// # Errors
    ///
    /// [`crate::InterrogateError::NoSuchBody`] for an index the value
    /// has no body at (an empty half among them),
    /// [`crate::InterrogateError::NoBodies`] for a value that holds none.
    pub fn output_body(&self, index: u32) -> Result<&Body<T>, crate::InterrogateError> {
        crate::names::interrogate::output_body(self, index)
    }

    /// The payload family, for typed operand mismatches: the `family`
    /// word the value lands in.
    pub fn kind_name(&self) -> &'static str {
        match self {
            Self::Datum(_) => family::DATUM,
            Self::Profile(_) => family::PROFILE,
            Self::Body(_) => family::BODY,
            Self::Boolean(_) => family::BOOLEAN,
            Self::Split { .. } => family::SPLIT,
            Self::Instances(_) => family::INSTANCES,
            Self::Copies(_) => family::COPIES,
            Self::Measure { .. } => family::MEASURE,
            // The SAME family name as a measure that has a value: the
            // node kind is what a typed operand mismatch is about, and
            // "measure" is what this node is either way. Which of the
            // two a reader is holding is a question about the value,
            // and the two variants are how it is asked.
            Self::MeasureUnavailable { .. } => family::MEASURE,
            Self::Assertion(_) => family::ASSERTION,
        }
    }
}

/// A boolean node's typed result (F8: ∅ is a value, not an error).
#[derive(Debug, Clone)]
pub enum BooleanValue<T: Decide> {
    /// The regularized result is empty — typed success.
    Empty,
    /// A real result body.
    Body {
        /// The result body.
        body: Arc<Body<T>>,
        /// How the kernel produced it.
        kind: BooleanResultKind,
        /// Contacts, declared or glued on a decided Zero, surviving into
        /// the result (the `BooleanBody` contract, spec D2).
        contacts: Arc<ContactRecords>,
    },
}

/// **A placement's copies, keyed** ([`ValuePayload::Copies`]).
///
/// `bodies` is what the node's name table indexes and the product
/// gather grafts: a `Place`'s copies flat in key order, or an
/// instance's part product as ONE body, whose arena keys the part's
/// declared contact records. `members` is the same material copy by
/// copy, each with its own table, for a reader of one copy
/// ([`crate::BodyRead::key`]).
#[derive(Debug, Clone)]
pub struct Copies<T: Decide> {
    /// The bodies the node's table indexes, in output-body order.
    pub bodies: Vec<Arc<Body<T>>>,
    /// Each copy by its key, in key order of its placement.
    pub members: Vec<KeyedCopy<T>>,
}

/// **One copy of a placement's copies, by its key**: a body, or a
/// family delivered flat under one key (a part's copy placed by a
/// pattern, until `work/refs/an-indexed-read-of-rank-two-has-no-family`).
#[derive(Debug, Clone)]
pub struct KeyedCopy<T: Decide> {
    /// The copy's key.
    pub key: crate::CopyKey,
    /// Its bodies: one for a body, the family's otherwise.
    pub bodies: Vec<Arc<Body<T>>>,
    /// Whether it is a family, whatever its count.
    pub family: bool,
    /// The parts it delivers ([`NodeValue::parts`]): what a reader of
    /// this one copy reads, not its placement's sum.
    pub parts: usize,
    /// Its names, indexing `bodies` by position.
    pub table: Arc<crate::names::NameTable>,
    /// **The motion a reader moves `bodies` by**: an instance's copy
    /// holds its part's bodies where the part put them, and is placed
    /// when a reader reads that one copy, so an instance that nothing
    /// reads copy by copy places its part once. `None` for bodies that
    /// stand where they are read, and for the bit-exact identity.
    pub motion: Option<geom_core::RigidMotion<T>>,
}

impl<T: Decide> Copies<T> {
    /// The copy keyed `key`, if any.
    #[must_use]
    pub fn member(&self, key: &crate::CopyKey) -> Option<&KeyedCopy<T>> {
        self.members.iter().find(|m| &m.key == key)
    }
}

/// One side of a split's output (both parts are the node's output,
/// role-tagged — spec D3).
#[derive(Debug, Clone)]
pub enum SplitSide<T: Decide> {
    /// No material on this side.
    Empty,
    /// This side's material as an independent body.
    Body(Arc<Body<T>>),
}

// An evaluated pose lives in the pose module (`crate::pose`), beside
// its symmetry; it is re-exported here, where a value payload is read.
pub use crate::pose::PoseValue;

// `NodeErrorKind::VerbArity` carries the kernel's verb name and
// declared-arity types in a pub payload, so both cross with it — the
// discriminant-crosses-with-the-refusal rule the pncad prelude writes
// at its `BlendKind` row. Without these a consumer could match the
// variant but never name what it caught.
pub use verbs::{Arity, VerbKind};

/// A node's typed failure: the wrapped cause plus the node it happened
/// at (spec D2's context contract; slot context lives in
/// [`NodeErrorKind::Expr`] — the PR 1 `NonFiniteResult` obligation).
#[derive(Debug)]
pub struct NodeError {
    /// The node that failed.
    pub node: RecipeNodeId,
    /// The typed cause.
    pub kind: NodeErrorKind,
    /// Every indeterminate predicate outcome met evaluating the node
    /// before it failed, in decision order (the same frame
    /// [`NodeValue::escalations`] reads on success), whether the
    /// failure came from the op or from a pass before its content key.
    /// This is how a consumer learns that a failure IS an escalation —
    /// and on which margin — without matching on whichever op error
    /// enum `kind` wrapped it in: a kernel refusal that carries an
    /// `Indeterminate` inside its own variant is recorded here too,
    /// because the funnel saw it first. Empty when nothing had decided
    /// yet (an input, a cycle, a slot that does not evaluate).
    ///
    /// **Not persisted**, like [`NodeValue::escalations`]: a failure's
    /// escalations are a fact about one run at one box, read in hand
    /// by the subdivision driver. The verdicts the op recorded before
    /// failing are NOT carried here — a failed node has no verdict
    /// vector to certify against — which is why the frame's two
    /// channels are two `Arc`s at this seam rather than one record:
    /// the value carries both, the error only this one.
    pub escalations: Arc<EscalationLog>,
}

/// **An evaluation refusal, carried into a document-layer
/// vocabulary** ([`crate::Refusal`]) — [`MateFault::PlacerRefused`](crate::MateFault),
/// [`EditError::PlacementAxis`](crate::EditError),
/// [`PartFault::PartRootFailed`](crate::PartFault) and
/// [`PartFault::PartRootPoisoned`](crate::PartFault) hold one.
pub type NodeRefusal = crate::refusal::Refusal<NodeErrorKind>;

impl NodeRefusal {
    /// The refusal, as the evaluation layer typed it.
    #[must_use]
    pub fn kind(&self) -> &NodeErrorKind {
        self.get()
    }

    /// The refusal as `node`'s own [`NodeError`] renders it, said by
    /// `by`: the line a surface draws for a carried refusal
    /// ([`NodeErrorKind::carried_chain`]), the same words that node's own
    /// tree draws.
    #[must_use]
    pub fn line_at(&self, node: RecipeNodeId, by: crate::spoken::Speaker<'_>) -> String {
        failed_line(node, self.get(), by)
    }
}

/// **The one space `node`'s inputs live in**, read off their values:
/// a `Place`'s shapes (it copies them as one), no input of an
/// instance's (it copies a part), every other node's inputs — a
/// constraint reads across spaces by design, so its reads are no input
/// here. `None` for a node reading no value that lives in a space.
///
/// # Errors
///
/// The first two spaces, where the inputs lie in two.
fn input_space<T: Decide>(
    doc: &Doc<ProfileProgram>,
    id: RecipeNodeId,
    node: &crate::node::Node<ProfileProgram>,
    results: &BTreeMap<RecipeNodeId, NodeResult<T>>,
) -> Result<Option<crate::mate::Space>, (crate::mate::Space, crate::mate::Space)> {
    use crate::node::Node;
    let inputs = match node {
        Node::InstantiatePart { .. } => Vec::new(),
        Node::Place { shapes, .. } => shapes
            .iter()
            .filter_map(|shape| doc.operation_of(shape.read))
            .collect(),
        _ => doc.upstream_of(id, node),
    };
    let mut first: Option<crate::mate::Space> = None;
    for input in inputs {
        let Some(NodeResult::Ok(value)) = results.get(&input) else {
            continue;
        };
        match (first, value.space) {
            (_, None) => {}
            (None, Some(space)) => first = Some(space),
            (Some(held), Some(space)) if held != space => return Err((held, space)),
            (Some(_), Some(_)) => {}
        }
    }
    Ok(first)
}

/// A node's failure as one line: the node, then its kind's prose, each
/// node said by `by`, and the failing node named once. The one
/// spelling [`NodeError`]'s renderings and [`NodeRefusal::line_at`]
/// share.
fn failed_line(node: RecipeNodeId, kind: &NodeErrorKind, by: crate::spoken::Speaker<'_>) -> String {
    format!(
        "{} failed: {}",
        by.node(node),
        crate::spoken::Said(kind, by.about(node))
    )
}

/// How the copies of a circular pattern whose step is a turn or more
/// would land ([`NodeErrorKind::FullRangeStep`]).
#[derive(Debug, Clone, PartialEq)]
pub enum StepTurns {
    /// The step is a whole number of turns at tolerance, so every
    /// copy lands on the master.
    Whole,
    /// The angle within one turn that lands every copy where the step
    /// does, up to rounding: the authored step less its whole turns,
    /// as a formula the speaker says ([`crate::spoken::Speaker::formula`]).
    Within(crate::Formula),
    /// The step holds this many whole turns, and the formula less them
    /// is one the expression bound refuses (it would nest too deep).
    Over(u64),
    /// The step holds more whole turns than its value resolves.
    Unresolved,
}

/// The closed set of node-evaluation failures. Kernel errors are
/// carried UNALTERED (spec D2: no stringification).
#[derive(Debug)]
pub enum NodeErrorKind {
    /// An expression slot failed to evaluate — (node, slot) context
    /// around [`EvalError`], including `NonFiniteResult` (the PR 1
    /// banked obligation).
    Expr {
        /// The slot whose expression failed.
        slot: SlotId,
        /// The expression evaluator's refusal, unaltered.
        source: EvalError,
    },
    /// Profile validation refused the replayed loops.
    Profile(ProfileError),
    /// A profile loop's program refused at REPLAY (LIB-SWITCH §4b) —
    /// the driver's typed refusal carried unaltered, with the loop
    /// coordinate ([`profile::ReplayError`] carries the step). The
    /// geometry class is V1 class 2: legal at rest, refused under this
    /// binding.
    ProfileReplay {
        /// The refusing loop (program order).
        loop_: u32,
        /// The driver's refusal, unaltered.
        error: profile::ReplayError<f64>,
    },
    /// **The lift's second pass**: one profile loop's program refused
    /// its GUIDED elaboration at the lane scalar.
    ///
    /// `structure` is the load-bearing half and names the consumed
    /// decision that could not be honoured — indeterminate here (the
    /// cue to narrow the parameter box) or classified definitely
    /// otherwise (this binding leaves the elaborated structure).
    /// `None` means the lane's own geometry refused at this binding;
    /// that refusal's payloads are scalar-valued and stay in the lane,
    /// the f64 pass being the one whose refusals carry numbers.
    ProfileLaneReplay {
        /// The refusing loop (program order).
        loop_: u32,
        /// The step it refused at.
        step: usize,
        /// The consumed decision, when the wall was the record.
        structure: Option<profile::StructureRefusal>,
    },
    /// The naming-anchor derivation failed to match a canonical loop
    /// back to a program loop — an internal invariant break
    /// (validate's canonical form is an exact reindexing of its
    /// input), surfaced typed rather than panicking. It names no loop:
    /// the derivation reports only that it failed.
    ProfileAnchor,
    /// The naming anchor, the replay record and the program's minted
    /// step ids do not describe one program, so the profile's pieces
    /// have no names — an internal invariant break, surfaced typed.
    ProfilePieces {
        /// Where the three disagree.
        fault: PiecesFault,
    },
    /// The profile op found no record of the f64 pass it builds on —
    /// no precompute at all, or no replay record for a program loop
    /// the guided lift replays — an internal invariant break, surfaced
    /// typed.
    ProfileRecordMissing {
        /// The program loop with no replay record; `None` when the
        /// whole precompute is absent.
        loop_: Option<u32>,
    },
    /// The extrude op refused.
    Extrude(ExtrudeError),
    /// The revolve op refused.
    Revolve(RevolveError),
    /// A TUBE door refused — the solid door's or the hollow door's.
    ///
    /// ONE arm for both kinds, because the kernel's two doors carry
    /// one error type and that type already says which door it came
    /// from: the three wall arms are reachable only through
    /// `tube_along_arc_hollow`, and its own `Display` names the door
    /// on every arm. Splitting this into `Tube`/`HollowTube` here
    /// would put the door's identity in a payload for the arms that
    /// do not depend on it, and then state it twice for the arms that
    /// do — the shape [`NodeErrorKind::Blend`] needs a `verb` for
    /// precisely because `FilletError` does NOT carry one.
    ///
    /// Carried UNALTERED like every other kernel refusal; the node
    /// never substitutes a body for one that did not build.
    Tube(Box<TubeError>),
    /// The split op refused.
    Split(SplitError),
    /// A BLEND op refused — the fillet's (M5 PR 12) or the chamfer's:
    /// a structural precondition on the requested chain, one of the
    /// numbered predicates, a corner or spine class the in-place
    /// surgery has not been built for, or an escalation. Which door
    /// refused, and what it refused about, is stated on
    /// [`sweep::blend::BlendError`]'s own variants and rendered by
    /// its `Display` — this doc names no predicate of its own, so it
    /// cannot drift from one.
    ///
    /// `verb` is which blend asked, because the two share one kernel
    /// error type and a reader must not be told a chamfer's refusal
    /// was a fillet's.
    ///
    /// Carried UNALTERED like every other kernel refusal; the node
    /// never passes its input body through.
    Blend {
        /// Which blend the refusing node is.
        verb: sweep::blend::BlendKind,
        /// The kernel's own refusal.
        error: sweep::blend::BlendError,
    },
    /// The boolean op refused.
    Boolean(BooleanError),
    /// The rigid-transform op refused.
    Transform(TransformError),
    /// A placement's rigid motion was not minted (a part of it is not
    /// a finite number) or not admitted (a literal frame, or a map the
    /// mate solve composed, that the decided door refuses).
    Motion(geom_core::RigidMotionError),
    /// The §10.3/§10.4 construction refused (M5 PR 10 §2's typed
    /// compatibility door): section shapes that do not correspond,
    /// open/closed mixing, degenerate sections, an unusable v-degree,
    /// a path whose frame does not exist.
    Skin(SkinError),
    /// The loft BODY assembly refused (M6-3): every door named on
    /// [`sweep::LoftError`] — geometry, section-profile validation,
    /// Euler assembly, pcurve mint, stacking orientation.
    Loft(sweep::LoftError),
    /// A curved-solid NODE lane whose front door does not exist yet,
    /// named precisely. Since M6-3 the LOFT body assembles and this
    /// variant no longer fires for it; what remains is the SWEEP
    /// node's joined-path composition lane (a recipe path operand is a
    /// closed profile LOOP, and §10.4 needs ONE curve — banked past
    /// M6), pinned in `sweep/tests/m5_pr10_frontier.rs`'s flipped
    /// rows' successor and `editor-core`'s node suites.
    ///
    /// A named sub-frontier, never a laundered catch-all.
    CurvedSolidFrontier {
        /// The precise missing door.
        what: &'static str,
    },
    /// An input id names no live node (a dangling reference —
    /// unreachable through `apply`, refused typed anyway).
    MissingInput {
        /// The dangling id.
        input: RecipeNodeId,
    },
    /// **An operand reads a variable no live operation defines** (D10:
    /// deleting an operation leaves its readers unresolved, typed, never
    /// re-pointed). The delete reported it
    /// ([`crate::Maintenance::StrandedRead`]); the repair is an edit
    /// naming a new read for the slot.
    UnresolvedRead {
        /// The operand.
        slot: crate::OperandSlot,
        /// The read.
        var: crate::VarId,
    },
    /// **A name of a selection the node reads failed to resolve** (D10:
    /// the N5 ladder runs in the selection, [`crate::VarDef::Select`]).
    /// A selection is a commitment, so a name that no longer resolves
    /// refuses every reader loudly instead of silently shrinking the
    /// set. The repair is `Rebind`.
    SelectResolve {
        /// The operand reading the selection.
        slot: crate::OperandSlot,
        /// The selection.
        var: crate::VarId,
        /// Which of its names failed, by place.
        reference: usize,
        /// The resolution failure (N5's closed trio).
        error: Box<crate::resolve::ResolveError>,
    },
    /// The document's recorded ε disagrees with the process's
    /// committed ambient ε (M4 PR 6 spec D4: one process = one ε —
    /// the kernel's every predicate reads the committed tolerance, so
    /// evaluating this document here would silently use the wrong ε;
    /// refused loudly on EVERY node instead). The repair is
    /// persist-grade: save, and replay in a process whose ε matches
    /// (load commits the recorded ε when the process is fresh).
    ToleranceConflict {
        /// The document's recorded ε.
        document_eps: f64,
        /// The process's committed ε.
        process_eps: f64,
    },
    /// The evaluation's parameter box could not bind an environment at
    /// this scalar (E6's leaf-replay door): the box names a parameter
    /// the document does not carry, or the scalar cannot represent a
    /// widened axis. Refused on EVERY node, like the ε conflict above
    /// and for the same reason — the alternative is answering a
    /// different question in the same shape.
    ParamBox {
        /// The door's refusal, unaltered.
        source: crate::analysis::ParamBoxError,
    },
    /// The evaluation's E4 seed could not bind (the seed door, the
    /// [`NodeErrorKind::ParamBox`] shape): the seed names a parameter
    /// the document does not carry, names a structural `Count`
    /// parameter, or this scalar has no tangent channel to carry it.
    /// Refused on EVERY node, before any node runs, and for the same
    /// reason as the box arm — a dropped seed is a different question
    /// answered in the same shape.
    Seed {
        /// The door's refusal, unaltered.
        source: crate::analysis::SeedError,
    },
    /// An input's value family does not fit this operand (e.g. a
    /// boolean fed a split's two-part value — selecting a part needs
    /// PR 3's naming layer).
    WrongOperand {
        /// The offending input node.
        input: RecipeNodeId,
        /// What the operand needed.
        expected: &'static str,
        /// What the input's value is.
        found: &'static str,
    },
    /// A body operand evaluated to the typed empty value — the kernel
    /// ops take real bodies only (v1; ∅-absorbing semantics would be
    /// invented behavior, spec D3's "wire, don't invent").
    EmptyOperand {
        /// The empty input node.
        input: RecipeNodeId,
    },
    /// A body operand is a PRODUCT — several parts a document gathered
    /// ([`NodeValue::parts`]; `crates/editor-core/ASSEMBLY.md`, A2) —
    /// which no op that fuses or reshapes one body accepts
    /// (`docs/DESIGN.md`, "A solid is one piece of material"): which
    /// solids are one part is recipe structure, and an explicit union is
    /// what makes the parts one body. The placers carry a product
    /// through instead, and a reader of geometry (a datum's face frame)
    /// consumes no material and reads it as it is.
    ProductOperand {
        /// The operand node.
        input: RecipeNodeId,
        /// How many parts its document gathered.
        parts: usize,
    },
    /// A body operand is not a finished body: the at-rest gate
    /// ([`topo::AtRestPolicy::gate_at_rest_kept`], tier 3) refuses the
    /// body its input node built, so a door that takes finished bodies
    /// (the Boolean, the split, the shell, the blends) cannot take it.
    /// Every door owes a finished body (`docs/DESIGN.md`, tier 3), and
    /// not every door gates its own result (the blends do not), so the
    /// defect is the input's door, not anything an author set on either
    /// node.
    UnfinishedOperand {
        /// The operand node.
        input: RecipeNodeId,
        /// The validator's findings, each naming its entity.
        errors: Vec<topo::ValidationError>,
    },
    /// A read of a split's port found its half holding no
    /// material — the tool plane missed the target on that side. Its
    /// own arm rather than [`NodeErrorKind::EmptyOperand`]: that
    /// one's prose is a boolean's, and what is empty here is a side
    /// of a cut, which has a name.
    EmptyHalf {
        /// The split node whose value was read.
        input: RecipeNodeId,
        /// The empty half.
        half: crate::names::SplitHalf,
    },
    /// A [`crate::Node::Part`] indexed a pattern's instances outside
    /// `0..count`. A negative index lands here too — the index is
    /// neither wrapped nor clamped, because either would be a body the
    /// author did not name.
    InstanceOutOfRange {
        /// The pattern node whose value was read.
        input: RecipeNodeId,
        /// The index as authored (resolved through its slot).
        index: i64,
        /// How many instances the value holds.
        count: usize,
    },
    /// A read of one copy ([`crate::BodyRead::key`]) whose key names no
    /// copy of the placement's copies: the part no longer places it, or
    /// the key was never one of these copies. Refused, never re-pointed.
    NoSuchCopy {
        /// The placement whose copies were read.
        input: RecipeNodeId,
        /// The key as written.
        key: Box<crate::CopyKey>,
    },
    /// A `Place` over several shapes reading one variable twice, so two
    /// of its copies key alike (FORK-DM4: an indexed read keys by its
    /// family): copy the family whole, or place each member apart.
    CopiesShareAKey {
        /// The key both copies would carry.
        key: Box<crate::CopyKey>,
    },
    /// A read of one copy at the other shape: an index into a copy that
    /// is one body, or no index into one that is a family.
    CopyShape {
        /// The placement whose copies were read.
        input: RecipeNodeId,
        /// The key as written.
        key: Box<crate::CopyKey>,
        /// Whether the copy is a family.
        family: bool,
    },
    /// A direction-valued vector decided to zero length. Which
    /// vectors those are is the ROLE constants' to say, not this
    /// doc's — every `*_ROLE` const in `wire` and in `placement`, as a
    /// CLASS rather than as a list, because a list here is a second
    /// copy of a set those modules already hold and it went stale the
    /// first time one of them was added.
    DegenerateDirection {
        /// Which vector, by role.
        role: &'static str,
    },
    /// A direction-valued vector whose LENGTH is not a finite number:
    /// components large enough to overflow the norm, or a poisoned
    /// one. A separate fact from a zero length and a separate
    /// recourse — the model is outside the range its own arithmetic
    /// can measure, and the fix is scale, not direction.
    NonFiniteDirection {
        /// Which vector, by role.
        role: &'static str,
    },
    /// A direction-valued vector whose LENGTH underflowed to zero:
    /// components small enough (`≲1e-162` at `f64`) that their
    /// squares are not representable, so the vector has a direction
    /// and no measurable length. A separate fact from a zero length
    /// and the same recourse as
    /// [`NodeErrorKind::NonFiniteDirection`] — the model is outside
    /// the range its own arithmetic can measure, and the fix is
    /// scale, not direction. Which vectors those are is the ROLE
    /// constants' to say, as for the two arms above.
    UnderflowedDirection {
        /// Which vector, by role.
        role: &'static str,
    },
    /// The ambient tolerance could not form a classification band.
    Band(geom_core::BandError),
    /// A slot the wiring expected was absent from the node — a wiring
    /// bug surfaced typed (unreachable while `Node::slots` and the
    /// wire agree; no panic paths in this crate).
    MissingSlot {
        /// The absent slot.
        slot: SlotId,
    },
    /// A verb was run through a door it does not declare —
    /// [`NodeErrorKind::MissingSlot`]'s class: a wiring bug surfaced
    /// typed (unreachable while the per-verb correspondences and the
    /// run doors agree; no panic paths in this crate).
    VerbArity {
        /// The verb whose door refused.
        verb: verbs::VerbKind,
        /// The door the verb was handed to; the declared one is
        /// `verb.arity()`.
        given: verbs::Arity,
    },
    /// A decided predicate escalated (in-band indeterminacy).
    Escalated {
        /// The named predicate.
        predicate: &'static str,
        /// The escalation, unaltered.
        source: Indeterminate,
    },
    /// **A measurement's parallelism read escalated** (D10's Measure
    /// ground): the measure door's own sentence, which [`Self::Escalated`]'s
    /// does not speak for. Its class is [`NodeErrorClass::Escalated`].
    MeasureEscalated {
        /// The named predicate.
        predicate: &'static str,
        /// The escalation, unaltered.
        source: Indeterminate,
    },
    /// **A placement frame's rotation axis escalated** (D10's placement
    /// ground): the sentence `EditError::PlacementAxis` carries, which
    /// [`Self::Escalated`]'s does not speak for. Its class is
    /// [`NodeErrorClass::Escalated`].
    PlacementAxisEscalated {
        /// The named predicate.
        predicate: &'static str,
        /// The escalation, unaltered.
        source: Indeterminate,
    },
    /// **A pose's or datum's direction escalated** (D10's pose ground):
    /// a datum's axis, normal or frame axis, or a pose construction's
    /// direction, in its own sentence,
    /// which [`Self::Escalated`]'s does not speak for. Its class is
    /// [`NodeErrorClass::Escalated`].
    PoseDirectionEscalated {
        /// The named predicate.
        predicate: &'static str,
        /// The escalation, unaltered.
        source: Indeterminate,
    },
    /// **A pose read off geometry found none of its kind** (D10): a
    /// face read as a plane on a curved carrier (DM1b), an axis read
    /// off a carrier that has no axis (a plane, a sphere), a point read
    /// off one that has no centre (a cylinder, a straight edge), or an
    /// entity its body could not read back. A tag read, never a
    /// predicate.
    PoseRead {
        /// The pose kind read.
        pose: crate::VarKind,
        /// Why there is none.
        fault: crate::pose::PoseReadFault,
    },
    /// **A pose construction met its degenerate case** (FORK-1b), as
    /// the decided length that names it found zero: a frame through a
    /// point on its axis, the meeting line of parallel planes. Refused,
    /// never joined by an arbitrary choice.
    PoseDegenerate {
        /// The construction.
        construction: crate::pose::PoseConstruction,
    },
    /// **A scalar a pose definition reads did not evaluate**: a slide's
    /// length or a spin's angle.
    PoseScalar {
        /// The scalar variable.
        var: crate::VarId,
        /// Why.
        source: crate::expr::EvalError,
    },
    /// A `Count` slot evaluated outside the range it admits: below its
    /// least value, or past the largest value the integer it is read
    /// into holds.
    CountOutOfRange {
        /// The slot read.
        slot: SlotId,
        /// The evaluated count.
        count: i64,
        /// The values the slot admits, both ends included.
        admits: core::ops::RangeInclusive<i64>,
    },
    /// A linear pattern's spacing evaluated definitely below zero. A
    /// spacing is a size; which way the copies step is the direction's
    /// to say.
    NegativeSpacing {
        /// The spacing as the classifier saw it, in metres, for the
        /// sentence only.
        spacing: geom_core::MarginDiag,
        /// The authored direction negated, each component a formula
        /// the speaker says ([`crate::spoken::Speaker::formula`]): with
        /// the spacing made positive, it builds the same copies. `None`
        /// for a component whose negation the expression bound refuses
        /// (it would nest too deep).
        reversed: [Option<crate::Formula>; 3],
    },
    /// A linear pattern's spacing is zero at tolerance, so every copy
    /// would land on the master.
    DegenerateSpacing,
    /// A circular pattern's step is zero at tolerance, so every copy
    /// would land on the master.
    DegenerateStep,
    /// A circular pattern's step reaches a full turn at tolerance, or
    /// passes it.
    FullRangeStep {
        /// The step as authored, said as `NegativeSpacing::reversed`
        /// is.
        step: crate::Formula,
        /// What the step evaluated to, in radians, when it is not a
        /// literal (a literal's text already says), for the sentence
        /// only.
        evaluated: Option<geom_core::MarginDiag>,
        /// How the copies would land.
        turns: StepTurns,
    },
    /// A [`crate::node::Node::PlacedUnion`]'s placements could not be
    /// CERTIFIED disjoint (GROUP-BOOLEAN-DESIGN, ratified A′): the two
    /// named copies' conservative boxes meet.
    ///
    /// The certificate is sufficient-not-necessary, so this refusal
    /// covers two situations and does not distinguish them: placements
    /// that genuinely interfere, and placements that are genuinely
    /// disjoint but too close for a box test. Budget-class, refinable
    /// by a sharper predicate later — never a silent maybe, because
    /// the graft door the union lowers through asserts nothing about
    /// its operands (#382).
    PlacementsUncertified {
        /// The lower placement index.
        i: usize,
        /// The higher placement index.
        j: usize,
    },
    /// A placement-rule node's rule is unusable — the count spelled two
    /// ways, an EMPTY explicit placement list, or a non-finite /
    /// improper frame.
    ///
    /// Unreachable through `apply` (the edit door refuses all four
    /// there, with the better diagnostics) and through `load` (the
    /// snapshot check re-refuses them); kept as a typed evaluation
    /// refusal so a hand-built document fails loudly and BY NAME —
    /// an empty list must not denote an empty body, and a poisoned
    /// frame must not read as a separation failure.
    PlacementRule(crate::node::PlacementRuleFault),
    /// The node is in (or downstream of) a dependency cycle — Kahn
    /// never released it (unreachable through `apply`, refused typed).
    UnschedulableCycle,
    /// Name emission failed (M4 PR 3, spec D4's loud door): an
    /// emission bug, a kernel-emission gap, or an in-band N2
    /// discriminator escalation — carried unaltered.
    Naming(NamingError),
    /// **A step of a union's fold refused** (DM4): the step folding
    /// `member` into the accumulation of the members listed before it,
    /// with the step's own refusal. The pairwise judgement decides every
    /// pair of members before the fold in every member order; a refusal
    /// only a fold step raises names the step, so it is never read as a
    /// pair's.
    UnionFoldStep {
        /// The position, in the node's member list, of the member the
        /// refusing step folds in: members of one family share a read.
        member: u32,
        /// That member's read.
        read: crate::VarId,
        /// The step's refusal.
        refusal: Box<NodeErrorKind>,
    },
    /// A blend node's selection is EMPTY. A blend of nothing is not
    /// the identity — it is an unfinished recipe, refused rather than
    /// passed through (the fail-loud voice: no op silently returns its
    /// input).
    BlendSelectionEmpty {
        /// Which blend the refusing node is.
        verb: sweep::blend::BlendKind,
    },
    /// **The shell op refused** — the thickness gate, the wall-clearance
    /// gate, a face's inward offset, a designation gate (a chart named
    /// in part, every face of a shell named, a designation that cuts
    /// the boundary in two), the rim surgery, or a result that does not
    /// validate. Which, and about what, is stated on
    /// [`topo::ShellError`]'s own variants and rendered by its
    /// `Display`; this doc names no predicate of its own.
    ///
    /// Carried at its `f64` WITNESS rather than at the lane scalar:
    /// this enum is scalar-free by construction, and the kernel's
    /// refusal is generic, so it crosses through a total fold
    /// (`crate::verbs::shell::fold_shell_error`) that keeps every arm
    /// and every number — at `f64` the fold is the identity, and at a
    /// bracket scalar each number is the infimum the kernel's own gates
    /// meter. The node never passes its input body through.
    Shell(Box<topo::ShellError<f64>>),
    /// **This evaluation scalar cannot form the shell door's call.**
    /// The door validates what it built with a certified claim, so it
    /// is formed only at a scalar with certification rights; a dual
    /// does not certify (the DL3 ruling), and rather than hollow a
    /// body it cannot validate the node refuses, naming the scalar. The
    /// base-scalar evaluation beside this one is where the shell is
    /// built and validated.
    ///
    /// **The NAME names the refusal, not a trait.** What the scalar
    /// has no door for is read off `topo::AtRestPolicy::shell_door`,
    /// the per-scalar policy seam; there is no `ShellLane` and the
    /// variant is not renamed for the mechanism behind it — the
    /// spelling crosses the Python boundary as the
    /// `shell_lane_unsupported` tag.
    ShellLaneUnsupported {
        /// The scalar that has no door ([`geom_core::Real::NAME`]).
        scalar: &'static str,
    },
    /// A derived frame's face is not planar (DM1b): a sketch frame
    /// needs a plane, and the carrier found is named so a headless
    /// author gets the same answer the chrome pre-empts. A tag read,
    /// not a predicate — the carrier's own kind, copied out.
    FaceFrameNotPlanar {
        /// The carrier kind the face actually has.
        carrier: geom::SurfaceKind,
    },
    /// A derived frame's face resolved to a key its own body could not
    /// read back — an evaluation-internal inconsistency between the
    /// emitted table and the body, surfaced typed rather than guessed
    /// around.
    FaceFrameReadback {
        /// The kernel-side refusal, unaltered.
        error: topo::readback::ReadbackError,
    },
    /// A sketch node's branch selection refused (SOLVER-DESIGN W3;
    /// M4 PR 4 pins the document semantics — a per-node failure
    /// poisoning descendants only, GQ2/W5). NEVER constructed while no
    /// sketch solver is in the tree (`docs/DESIGN.md`
    /// `[open.sketch-solver]`, which waits on constraint-driven
    /// sketches having a consumer): the arm exists so the solver lands
    /// as logic, not a schema change.
    WitnessBifurcation(crate::witness::WitnessBifurcation),
    /// An `InstantiatePart` node could not produce its part's placed
    /// body (ASM-2A D-3). The reference names WHICH document was being
    /// crossed to; the fault says what stopped it — including A4's pin
    /// gate and A2's ε seam, observed here at evaluation rather than
    /// assumed at authoring.
    Part {
        /// The reference that was being resolved.
        doc_ref: crate::ident::DocRef,
        /// Why it did not yield a part body.
        fault: parts::PartFault,
    },
    /// The mate solve refused for this node (ASM-R2a D-4): the mate
    /// itself, or an instance whose group the refusal left without a
    /// pose. The fault names its own subject — the pair, the residual
    /// subgroup, the failed predicate and its measured clash.
    Mate(Box<crate::mate::MateFault>),
    /// **A read across two spaces** (D10, Spaces): a node whose reads
    /// lie in two kinds — a loose copy's and the world's, or two loose
    /// copies' — would relate two raw coordinate systems, which only a
    /// mate may do.
    SpaceMismatch {
        /// One space the node reads.
        a: crate::mate::Space,
        /// Another.
        b: crate::mate::Space,
    },
    /// A crossing declaration on this instance no longer resolves
    /// against the pinned part (ASM-R2b D-4/D-5; A4's "does it
    /// actually fit", A13 clause 4). The seam asserted a contact at a
    /// part entity the pinned document's product does not name — the
    /// swap is refused, never accepted with the declaration quietly
    /// dropped.
    CrossingUnverified {
        /// The instance carrying the record.
        instance: RecipeNodeId,
        /// WHICH crossing: its remainder-side reference, the one the
        /// remainder's mate keeps. A record is keyed by nothing else
        /// — a crossing carries no provenance — and this is a name in
        /// the reader's own document, which a mate id never was.
        outer: Box<crate::names::FaceName>,
        /// The part-side reference that did not resolve.
        name: Box<crate::names::StableName>,
    },
    /// A `Node::Measure` reference resolved into a value that carries
    /// no bodies, or into an output body its value does not have — the
    /// naming emission and the value disagree, or the reference names
    /// a datum. Carries the interrogation layer's own words.
    MeasureRefUnreadable {
        /// The reference.
        slot: crate::OperandSlot,
        /// What it reads.
        var: crate::VarId,
        /// Why it could not be read back.
        error: crate::names::InterrogateError,
    },
    /// A measured expression evaluated to a NON-FINITE value — the
    /// same ruled door `expr::eval` applies to a document expression,
    /// applied to the measurement sublanguage's own arithmetic.
    ///
    /// It is its own arm rather than a reuse of
    /// [`NodeErrorKind::Expr`] because there is no SLOT to name: a
    /// measured expression has no slot vocabulary, and the honest
    /// address is the node.
    MeasureNonFinite {
        /// The expression evaluator's refusal, unaltered.
        source: EvalError,
    },
    /// A measured pair IS in the v1 table, but the arm needs its two
    /// carriers PARALLEL and they are decidedly not. Distinct from
    /// [`NodeErrorKind::MeasureUnsupported`] on purpose: telling an
    /// author that two cylinder walls are an unsupported pair is
    /// false, and sends them looking for a missing feature instead of
    /// at their tilt.
    MeasureNotParallel {
        /// Which primitive was asked.
        verb: &'static str,
        /// The first operand's carrier class.
        a: &'static str,
        /// The second operand's carrier class.
        b: &'static str,
        /// The funnel predicate that decided them non-parallel.
        predicate: &'static str,
    },
    /// The measured expression has no closed form for the carrier pair
    /// it was asked about (E3's honest v1 scope).
    MeasureUnsupported(measure::MeasureUnsupported),
    /// A node's PAYLOAD expression failed to evaluate — an assertion's
    /// value or its bound.
    ///
    /// The address is the expression's position in `payload_exprs`
    /// order, because an assertion has no slot vocabulary to name. The
    /// rendering says WHICH one it is.
    PayloadExpr {
        /// Which node kind's payload this is, for the message.
        what: &'static str,
        /// The expression's index in `payload_exprs` order.
        index: usize,
        /// The expression evaluator's refusal, unaltered.
        source: EvalError,
    },
    /// The clearance engine refused a `min_clearance` measurement: its
    /// own typed refusal (E7's refusal vocabulary), carried unaltered.
    MeasureClearanceRefused(crate::clearance::ClearanceRefusal),
    /// An `Assertion`'s bound is dimensioned differently from the
    /// measure it constrains — comparing metres with radians is a
    /// document fault, refused rather than compared.
    AssertionDimension {
        /// The measure's dimension.
        measured: crate::expr::Dimension,
        /// The bound's.
        bound: crate::expr::Dimension,
    },
}

// LIB-DOORS F6 (reopened on review): the human-readable rendering the
// bindings' exception messages consume. Each arm names the failing op
// and then FORWARDS its payload's own `Display` — the kernel refusal
// still rides the variant unaltered (D2) for callers who match, but
// this string is the only channel a caller who cannot match has, and
// the bindings' `kind` tag carries the discriminant alone. An arm that
// re-states a payload it holds in its own words invents a second
// vocabulary for a refusal that already has one.
//
// Owning a recourse the payload cannot spell buys an arm PROSE, never
// the right to drop the payload.
//
// Every payload-holding arm forwards its payload's own `Display` —
// `EvalError`, `resolve::ResolveError`, `WitnessBifurcation` and
// `PlacementRuleFault` (D54's four) all carry one, and
// `PlacementRuleFault`'s is that fault set's ONE prose vocabulary (the
// edit door's rule arms forward the same impl).
//
// The one exception is a CARRIED refusal: another node's refusal, with
// its own recourse, held as a `NodeRefusal` (`PartFault::PartRootFailed`
// and `PartRootPoisoned`, `MateFault::PlacerRefused`). It is not this refusal's payload, so it is
// never rendered inside this sentence, which names that node and points
// at it; it is drawn as its own line, read off
// [`NodeErrorKind::carried_chain`].
//
// Each node the sentence names is said by the speaker the frame handing
// the refusal out passes ([`crate::spoken::Say`]): the kind lives in the
// evaluation memo, so it holds ids and never a label its key does not
// fix. A part's fault is numbered in the part and holds the part's nodes
// as its pin fixes them, so it says them itself ([`PartFault::held`]).
impl crate::spoken::Say for NodeErrorKind {
    #[allow(clippy::too_many_lines)] // one arm per variant, each short
    fn say(
        &self,
        f: &mut core::fmt::Formatter<'_>,
        by: crate::spoken::Speaker<'_>,
    ) -> core::fmt::Result {
        use crate::spoken::Said;
        match self {
            Self::Expr { slot, source } => {
                write!(
                    f,
                    "the expression at slot {} failed to evaluate: {source}",
                    slot.label()
                )
            }
            Self::Profile(e) => write!(f, "the replayed profile failed validation: {e}"),
            Self::ProfileLaneReplay {
                loop_,
                step,
                structure,
            } => match structure {
                Some(r) => write!(
                    f,
                    "profile loop {loop_}'s lane elaboration refused at step {step}: {r}"
                ),
                None => write!(
                    f,
                    "profile loop {loop_}'s lane elaboration refused at step {step} on its \
                     own geometry under this binding (the f64 pass is the one whose \
                     refusals carry numbers)"
                ),
            },
            Self::ProfileReplay { loop_, error } => {
                write!(f, "profile loop {loop_} refused at {error}")
            }
            Self::ProfileAnchor => write!(
                f,
                "a loop of the validated profile did not match back to a loop of the program. {}",
                geom_core::KERNEL_DEFECT_ENDING
            ),
            Self::ProfilePieces { fault } => {
                write!(f, "the profile's pieces have no names: {fault}")
            }
            Self::ProfileRecordMissing { loop_: None } => write!(
                f,
                "the profile's f64 pass left no record for its op to build on. {}",
                geom_core::KERNEL_DEFECT_ENDING
            ),
            Self::ProfileRecordMissing { loop_: Some(loop_) } => write!(
                f,
                "profile loop {loop_} has no replay record from the f64 pass to guide its \
                 lift. {}",
                geom_core::KERNEL_DEFECT_ENDING
            ),
            Self::Mate(fault) => write!(f, "the mate solve refused: {}", Said(&**fault, by)),
            Self::SpaceMismatch { a, b } => {
                let space = |s: &crate::mate::Space| match s.loose() {
                    None => "the world's space".to_owned(),
                    Some(root) => format!("the space of the loose copy {}", by.node(root)),
                };
                write!(
                    f,
                    "this reads {} and {}, and only a mate relates two spaces. {}",
                    space(a),
                    space(b),
                    crate::sentence::Recourse(
                        "place one copy relative to the other, or read only one space"
                    )
                )
            }
            Self::CrossingUnverified {
                instance,
                outer,
                name,
            } => write!(
                f,
                "{}'s seam declaration crosses at {} on the remainder and claims {} in \
                 the part, which this version of the part does not name, so the crossing does \
                 not re-verify",
                by.node_as(*instance, "instance"),
                by.name(outer),
                // The part's own nodes and steps are another document's
                // ids, so its name is said by tag.
                crate::spoken::Speaker::TAG.name(name),
            ),
            Self::Extrude(e) => write!(f, "the extrude op refused: {e}"),
            Self::Revolve(e) => write!(f, "the revolve op refused: {e}"),
            // The kernel error says which tube it is (solid or
            // hollow), so this line names neither: a wrapper that did
            // would read a hollow refusal as a solid one half the time.
            Self::Tube(e) => write!(f, "the tube op refused: {e}"),
            Self::Split(e) => write!(f, "the split op refused: {e}"),
            Self::Blend { verb, error } => write!(f, "the {verb} op refused: {error}"),
            // No cause is guessed here: the kernel's refusal names its
            // own, and a coincidence refusal carries its own recourse.
            Self::Boolean(e) => write!(f, "the Boolean op refused: {e}"),
            Self::Transform(e) => write!(f, "the transform op refused: {e}"),
            Self::Motion(e) => write!(f, "the placement's motion refused: {e}"),
            Self::Skin(e) => write!(f, "the skin construction refused: {e}"),
            Self::Loft(e) => write!(f, "the loft assembly refused: {e}"),
            Self::CurvedSolidFrontier { what } => write!(f, "not yet buildable: {what}"),
            Self::MissingInput { input } => {
                write!(f, "{} names no live node", by.node_as(*input, "input"))
            }
            Self::UnresolvedRead { slot, var } => write!(
                f,
                "its {slot} reads {var}, which no live operation defines: the operation it \
                 read was deleted"
            ),
            Self::SelectResolve {
                slot,
                var,
                reference,
                error,
            } => match by.selection_reference(*slot, *var, *reference, error.name()) {
                Some(at) => write!(
                    f,
                    "{}",
                    crate::spoken::Said(&crate::resolve::AboutReference(error, at), by)
                ),
                None => write!(
                    f,
                    "a selected name failed to resolve: {}",
                    crate::spoken::Said(error.as_ref(), by)
                ),
            },
            Self::ToleranceConflict {
                document_eps,
                process_eps,
            } => write!(
                f,
                "document ε {document_eps:e} conflicts with the process ε {process_eps:e} \
                 (one process, one ε)"
            ),
            Self::ParamBox { source } => write!(f, "{source}"),
            Self::Seed { source } => write!(f, "{source}"),
            Self::WrongOperand {
                input,
                expected,
                found,
            } => write!(
                f,
                "{} carries kind {found}; the operand needs kind {expected}",
                by.node_as(*input, "input")
            ),
            Self::EmptyOperand { input } => write!(
                f,
                "{} is the empty value — the body ops take real bodies",
                by.node_as(*input, "input")
            ),
            Self::ProductOperand { input, parts } => write!(
                f,
                "{} gathers {parts} parts, and this op takes one body. Recourse: union the \
                 parts explicitly in their document and use that union",
                by.node_as(*input, "input")
            ),
            Self::UnfinishedOperand { input, errors } => write!(
                f,
                "{} shipped a body that does not finish: {}",
                by.node_as(*input, "input"),
                errors
                    .first()
                    .map_or_else(|| "no finding".to_string(), ToString::to_string)
            ),
            Self::EmptyHalf { input, half } => write!(
                f,
                "the split's {} half ({}) holds no material",
                match half {
                    crate::names::SplitHalf::Above => "above",
                    crate::names::SplitHalf::Below => "below",
                },
                by.node(*input)
            ),
            Self::NoSuchCopy { input, key } => write!(
                f,
                "{} delivers no copy keyed {key}. {}",
                by.node(*input),
                crate::sentence::Recourse("read a copy it delivers")
            ),
            Self::CopiesShareAKey { key } => write!(
                f,
                "two of the placement's shapes read {key}, so two copies would be keyed alike. {}",
                crate::sentence::Recourse(
                    "copy the family whole, or place each member by its own placement"
                )
            ),
            Self::CopyShape { input, key, family } => {
                if *family {
                    write!(
                        f,
                        "the copy keyed {key} of {} is a family. {}",
                        by.node(*input),
                        crate::sentence::Recourse("read one member of it by index")
                    )
                } else {
                    write!(
                        f,
                        "the copy keyed {key} of {} is one body. {}",
                        by.node(*input),
                        crate::sentence::Recourse("read it without an index")
                    )
                }
            }
            Self::InstanceOutOfRange {
                input,
                index,
                count,
            } => write!(
                f,
                "instance index {index} is outside the pattern's {count} instances ({}; \
                 the admitted indices are 0 to {})",
                by.node(*input),
                count.saturating_sub(1)
            ),
            // Every role word is already a complete noun phrase for
            // the vector (the `*_ROLE` constants are where they are
            // written), so the sentence names the role and nothing
            // after it.
            Self::DegenerateDirection { role } => {
                write!(f, "the {role} has zero length")
            }
            Self::UnderflowedDirection { role } => write!(
                f,
                "the {role} underflowed to zero length, though it still names a \
                 direction. Recourse: {}",
                geom_core::RANGE_RECOURSE
            ),
            Self::NonFiniteDirection { role } => write!(
                f,
                "the {role} has no finite length (a component overflows the norm or is \
                 not a number). Recourse: {}",
                geom_core::RANGE_RECOURSE
            ),
            Self::Band(e) => write!(
                f,
                "the ambient tolerance could not form a classification band: {e}"
            ),
            Self::MissingSlot { slot } => {
                write!(
                    f,
                    "the node's wiring expected its {} input, which is absent. {}",
                    slot.label(),
                    geom_core::KERNEL_DEFECT_ENDING
                )
            }
            // The sentence is single-homed at the run doors' own
            // refusal (`verbs::VerbError::Arity`); this arm re-wraps
            // the same fields and forwards its Display, so the two
            // layers cannot drift apart.
            Self::VerbArity { verb, given } => {
                // The scalar is immaterial: this arm of the kernel's
                // refusal carries none, and its sentence is a function
                // of the two names it holds. One is named so the type
                // is complete.
                let refusal = verbs::VerbError::<f64>::Arity {
                    verb: *verb,
                    given: *given,
                };
                write!(f, "{refusal}. {}", geom_core::KERNEL_DEFECT_ENDING)
            }
            Self::Escalated { predicate, source } => {
                // What the decision was deciding, in words; the name is
                // routing and rides `Debug`.
                let what = crate::decision::words(predicate).unwrap_or(geom_core::UNNAMED_DECISION);
                write!(
                    f,
                    "{}",
                    source.undecided(what, crate::decision::ending(predicate, source))
                )
            }
            Self::MeasureEscalated { predicate, source }
            | Self::PlacementAxisEscalated { predicate, source }
            | Self::PoseDirectionEscalated { predicate, source } => {
                let what = crate::decision::words(predicate).unwrap_or(geom_core::UNNAMED_DECISION);
                write!(f, "{what} is too close to call: {source}")
            }
            Self::PoseRead { pose, fault } => write!(
                f,
                "{} {pose} read off geometry found none: {fault}. Recourse: read it off \
                 geometry whose carrier has one",
                crate::sentence::article(&pose.to_string())
            ),
            Self::PoseDegenerate { construction } => write!(f, "{construction}"),
            Self::PoseScalar { var: _, source } => write!(
                f,
                "a scalar the pose reads did not evaluate: {source}. Recourse: define it from \
                 values the document holds"
            ),
            Self::CountOutOfRange {
                slot,
                count,
                admits,
            } => {
                let (least, most) = (*admits.start(), *admits.end());
                write!(
                    f,
                    "the {} input evaluated to {count}, and it admits ",
                    slot.label()
                )?;
                if most == i64::MAX {
                    write!(f, "at least {least}")?;
                } else {
                    write!(f, "{least} to {most}")?;
                }
                f.write_str(". Recourse: make it evaluate within that range")
            }
            Self::NegativeSpacing { spacing, reversed } => {
                write!(
                    f,
                    "the pattern spacing evaluated to {spacing} m, below zero, and a spacing is \
                     a size: which way the copies step is the direction's to say, not a sign's. \
                     Recourse: make the spacing evaluate positive (its sign comes from whatever \
                     drives it) and point the direction the other way"
                )?;
                match reversed {
                    [Some(x), Some(y), Some(z)] => {
                        let [x, y, z] = [x, y, z].map(|e| by.formula(e));
                        write!(f, ", ({x}, {y}, {z})")
                    }
                    _ => Ok(()),
                }
            }
            Self::DegenerateSpacing => f.write_str(
                "the pattern spacing is zero at tolerance, so every copy would land on the \
                 master. Recourse: make the spacing a length the tolerance tells from zero, \
                 or lower the tolerance",
            ),
            Self::DegenerateStep => f.write_str(
                "the pattern step is zero at tolerance, so every copy would land on the \
                 master. Recourse: make the step an angle the tolerance tells from zero \
                 (360 deg over the count closes a ring), or lower the tolerance",
            ),
            Self::FullRangeStep {
                step,
                evaluated,
                turns,
            } => {
                write!(f, "the pattern step {}", by.formula(step))?;
                if let Some(radians) = evaluated {
                    write!(f, ", which evaluated to {radians} rad,")?;
                }
                // A literal is rewritten; anything else is made to
                // evaluate to the angle.
                let make = if evaluated.is_some() {
                    "make it evaluate to"
                } else {
                    "make the step"
                };
                match turns {
                    StepTurns::Whole => write!(
                        f,
                        " is a whole number of turns, so every copy would land on the master. \
                         Recourse: {make} a nonzero angle within one turn (360 deg over the \
                         count closes a ring)"
                    ),
                    StepTurns::Within(within) if evaluated.is_some() => write!(
                        f,
                        " is past a full turn, and a step is an angle within one. Recourse: \
                         make it evaluate within one turn; {} does, and places every copy \
                         where this does, up to rounding",
                        by.formula(within)
                    ),
                    StepTurns::Within(within) => write!(
                        f,
                        " is past a full turn, and a step is an angle within one. Recourse: \
                         write it as {}, which places every copy where this does, up to \
                         rounding",
                        by.formula(within)
                    ),
                    StepTurns::Over(held) => write!(
                        f,
                        " is past a full turn, and a step is an angle within one. Recourse: \
                         make it {} deg nearer zero, which places every copy where this does, \
                         up to rounding",
                        360 * u128::from(*held)
                    ),
                    StepTurns::Unresolved => write!(
                        f,
                        " holds more whole turns than its value resolves, and a step is an \
                         angle within one. Recourse: {make} an angle within one turn (360 deg \
                         over the count closes a ring)"
                    ),
                }
            }
            Self::PlacementsUncertified { i, j } => write!(
                f,
                "placements {i} and {j} are not certified disjoint — their conservative boxes meet, \
                 so the group union cannot be lowered through the disjoint-graft door"
            ),
            Self::PlacementRule(fault) => {
                write!(f, "the node's placement rule is unusable: {fault}")
            }
            Self::UnschedulableCycle => {
                f.write_str("the node is in, or downstream of, a dependency cycle")
            }
            // #380: the payload is editor-core's OWN diagnostic, not a
            // kernel refusal riding the variant — it has no other route
            // to a human, so it is carried through rather than dropped.
            Self::Naming(e) => write!(f, "name emission failed: {}", Said(e, by)),
            Self::UnionFoldStep {
                member,
                read,
                refusal,
            } => write!(
                f,
                "the fold refused at the step that folds in member {member} ({}): {}",
                by.read(*read),
                Said(refusal.as_ref(), by)
            ),
            Self::BlendSelectionEmpty { verb } => write!(
                f,
                "the {verb} selection is empty — an unfinished recipe, not the identity"
            ),
            Self::Shell(e) => write!(f, "the shell op refused: {e}"),
            Self::ShellLaneUnsupported { scalar } => write!(
                f,
                "the shell door has no lane at the {scalar} scalar: hollowing validates what it \
                 built with a certified claim, and this scalar does not certify — the \
                 base-scalar evaluation beside this one is where the shell is built"
            ),
            Self::FaceFrameNotPlanar { carrier } => write!(
                f,
                "the derived frame's face lies on {} {} carrier, not a plane — a sketch frame \
                 needs a planar face",
                crate::sentence::article(carrier.name()),
                carrier.name()
            ),
            Self::FaceFrameReadback { error } => write!(
                f,
                "the derived frame's face resolved to a key its body could not read back: {error}"
            ),
            Self::MeasureRefUnreadable {
                slot,
                var: _,
                error,
            } => {
                write!(f, "the measure's {slot} could not be read back: {error}")
            }
            Self::MeasureUnsupported(refusal) => write!(f, "{refusal}"),
            Self::MeasureNotParallel {
                verb,
                a,
                b,
                predicate,
            } => write!(
                f,
                "`{verb}` of {a} against {b} needs them parallel, and {predicate} decided they \
                 are not — this pair IS in the v1 table, so the tilt is what to fix"
            ),
            Self::MeasureNonFinite { source } => write!(
                f,
                "the measured expression did not evaluate to a finite value: {source}"
            ),
            Self::PayloadExpr {
                what,
                index,
                source,
            } => write!(f, "{what} {index} failed to evaluate: {source}"),
            Self::MeasureClearanceRefused(refusal) => {
                write!(f, "the clearance engine refused `{}`", refusal.name())?;
                let payload = refusal.said_payload(by);
                if !payload.is_empty() {
                    write!(f, " ({payload})")?;
                }
                Ok(())
            }
            Self::AssertionDimension { measured, bound } => write!(
                f,
                "the assertion's bound is {} {bound} and the measure it constrains is \
                 {} {measured} — an assertion compares like with like or not at all",
                bound.article(),
                measured.article()
            ),
            Self::WitnessBifurcation(refusal) => {
                write!(f, "{}", crate::witness::BranchSelectionRefused(refusal))
            }
            // The reference is data, not prose: the typed arm keeps it,
            // and a surface names the part by what it knows it as.
            Self::Part { fault, .. } => write!(f, "instantiating the part: {fault}"),
        }
    }
}

/// The kind where no document is at hand: each node by its tag.
impl core::fmt::Display for NodeErrorKind {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        crate::spoken::Say::say(self, f, crate::spoken::Speaker::TAG)
    }
}

impl NodeErrorKind {
    /// **The refusal this one carries, when it carries one**: the node
    /// that raised it and the refusal itself — the exception written
    /// over this type's `Display`, as a value.
    ///
    /// Two arms carry one. A part whose root failed carries that root's
    /// refusal, and one whose root was poisoned carries the refusal of
    /// the node that poisoned it, in the REFERENCED document's id space; a mate whose
    /// placer's own row cannot state its refusal carries the placer's,
    /// in the mate's document. One step only: a surface reads the whole
    /// chain through [`NodeErrorKind::carried_chain`].
    ///
    /// Every other arm holds no [`NodeRefusal`]; the two that forward a
    /// fault enum ask that enum, whose own reading is exhaustive.
    #[must_use]
    pub fn carried(&self) -> Option<(RecipeNodeId, &NodeRefusal)> {
        match self {
            Self::Part { fault, .. } => fault.carried(),
            Self::Mate(fault) => fault.carried(),
            _ => None,
        }
    }

    /// **Every refusal this one carries, outermost first**, each with
    /// the document its node is in: the one reading every surface draws
    /// a traceback from. A part inside a part yields one level per
    /// document, and the last level is the node that refused.
    pub fn carried_chain(&self) -> CarriedChain<'_> {
        CarriedChain {
            next: CarriedChain::step(self, CarriedIn::ThisDocument),
        }
    }
}

/// **Which document a carried refusal's node is in.** A node number
/// means nothing without it: a part's root is numbered in the part.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CarriedIn<'a> {
    /// The document whose evaluation raised the outermost refusal.
    ThisDocument,
    /// The part `doc_ref` names, whose nodes the level names are `held`
    /// as the version it pins holds them ([`PartFault::held`]).
    Part {
        /// The reference crossed into the part.
        doc_ref: &'a crate::ident::DocRef,
        /// The part's nodes, as its fault keeps them.
        held: &'a crate::spoken::HeldNodes,
    },
}

impl<'a> CarriedIn<'a> {
    /// The reference crossed into the part the level is in; `None` for
    /// the outermost document.
    #[must_use]
    pub fn doc_ref(&self) -> Option<&'a crate::ident::DocRef> {
        match self {
            CarriedIn::ThisDocument => None,
            CarriedIn::Part { doc_ref, .. } => Some(doc_ref),
        }
    }
}

/// **One level of a carried chain**: the node that refused, the
/// document it is in, and its refusal.
#[derive(Clone, Copy, Debug)]
pub struct CarriedLevel<'a> {
    /// The document [`CarriedLevel::node`] is numbered in.
    pub document: CarriedIn<'a>,
    /// The node that raised the refusal.
    pub node: RecipeNodeId,
    /// Its refusal.
    pub refusal: &'a NodeRefusal,
}

impl CarriedLevel<'_> {
    /// The level as its node's own tree draws it
    /// ([`NodeRefusal::line_at`]), its nodes spoken from `here`, the
    /// document the outermost refusal was raised in, when the level is
    /// in it. A level in a part says its nodes as the part's fault holds
    /// them, never from `here`: ids are not document-scoped, so `here`
    /// may hold the same id as another node.
    #[must_use]
    pub fn line_in<P: crate::ProfilePayload>(&self, here: &Doc<P>) -> String {
        let by = match self.document {
            CarriedIn::ThisDocument => crate::spoken::Speaker::of(here),
            CarriedIn::Part { held, .. } => crate::spoken::Speaker::held(held),
        };
        self.refusal.line_at(self.node, by)
    }

    /// The level where no document is at hand: a node of the outermost
    /// document by its tag, a part's as its fault holds it.
    #[must_use]
    pub fn line(&self) -> String {
        let by = match self.document {
            CarriedIn::ThisDocument => crate::spoken::Speaker::TAG,
            CarriedIn::Part { held, .. } => crate::spoken::Speaker::held(held),
        };
        self.refusal.line_at(self.node, by)
    }
}

/// The iterator [`NodeErrorKind::carried_chain`] and
/// [`crate::MateFault::carried_chain`] answer.
#[derive(Clone, Debug)]
pub struct CarriedChain<'a> {
    next: Option<CarriedLevel<'a>>,
}

impl<'a> CarriedChain<'a> {
    /// The chain whose first level is `first`, in `document`.
    pub(crate) fn from_first(
        first: Option<(RecipeNodeId, &'a NodeRefusal)>,
        document: CarriedIn<'a>,
    ) -> Self {
        Self {
            next: first.map(|(node, refusal)| CarriedLevel {
                document,
                node,
                refusal,
            }),
        }
    }

    /// The level `kind` carries, when it carries one. A part's level is
    /// in the part; a mate's is in `outer`, the document `kind` itself
    /// was raised in.
    fn step(kind: &'a NodeErrorKind, outer: CarriedIn<'a>) -> Option<CarriedLevel<'a>> {
        let (node, refusal, document) = match kind {
            NodeErrorKind::Part { doc_ref, fault } => {
                let (node, refusal, held) = fault.carried_held()?;
                (node, refusal, CarriedIn::Part { doc_ref, held })
            }
            _ => {
                let (node, refusal) = kind.carried()?;
                (node, refusal, outer)
            }
        };
        Some(CarriedLevel {
            document,
            node,
            refusal,
        })
    }
}

impl<'a> Iterator for CarriedChain<'a> {
    type Item = CarriedLevel<'a>;

    fn next(&mut self) -> Option<CarriedLevel<'a>> {
        let level = self.next.take()?;
        self.next = Self::step(level.refusal.kind(), level.document);
        Some(level)
    }
}

impl NodeError {
    /// **The failure as the frame that owns the node's document speaks
    /// it**: the node as `doc` holds it now ([`Doc::spoken`]), then its
    /// kind's prose, each name it forwards within the table
    /// `evaluation`, the evaluation that raised it, holds it in
    /// ([`crate::Speaker::within`]). The error lives in the
    /// [`Evaluation`], which a frame holds across edits (and a label edit
    /// recomputes nothing), so it holds the id and never a label a rename
    /// could leave stale.
    #[must_use]
    pub fn spoken<P: crate::ProfilePayload>(
        &self,
        doc: &Doc<P>,
        evaluation: &dyn crate::NameTables,
    ) -> String {
        failed_line(
            self.node,
            &self.kind,
            crate::spoken::Speaker::of(doc).within(evaluation),
        )
    }

    /// **The kind's prose alone, spoken from `doc`**: each node and
    /// each formula's reader as `doc` holds them now, and the failing
    /// node as `this <noun>` where the prose names one of its seats.
    /// What a consumer that names the node itself quotes as the cause,
    /// in place of the kind's documentless `Display`, which writes a
    /// reader `#<16 hex>`.
    #[must_use]
    pub fn kind_spoken<P: crate::ProfilePayload>(&self, doc: &Doc<P>) -> String {
        crate::spoken::Said(&self.kind, crate::spoken::Speaker::of(doc).about(self.node))
            .to_string()
    }
}

/// The [`NodeError`] rendering where no document is at hand: each node
/// by its tag, then its kind's prose.
impl core::fmt::Display for NodeError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(&failed_line(
            self.node,
            &self.kind,
            crate::spoken::Speaker::TAG,
        ))
    }
}

impl core::error::Error for NodeError {}

/// An evaluation's identity token (spec D5, GQ2): callers tag each
/// launched evaluation and discriminate stale results by comparing the
/// epoch a result carries against the newest one they minted.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Epoch(pub u64);

static EPOCH_COUNTER: AtomicU64 = AtomicU64::new(1);

impl Epoch {
    /// Mints a process-unique epoch (monotone).
    pub fn mint() -> Self {
        Self(EPOCH_COUNTER.fetch_add(1, Ordering::Relaxed))
    }
}

/// The cooperative cancel token (spec D5): checked BETWEEN nodes on
/// the serial walk, and between levels on the level schedule — node
/// granularity in v1; tokens are never threaded into kernel ops. A
/// `parallel` request that runs as the serial walk (a symbolic session
/// or shape report installed, [`EvalOptions::parallel`]) is checked
/// between nodes.
#[derive(Debug, Clone, Default)]
pub struct CancelToken(Arc<AtomicBool>);

impl CancelToken {
    /// A fresh, un-canceled token.
    pub fn new() -> Self {
        Self::default()
    }

    /// Requests cancelation (any thread, any time).
    pub fn cancel(&self) {
        self.0.store(true, Ordering::Relaxed);
    }

    /// Whether cancelation has been requested.
    pub fn is_canceled(&self) -> bool {
        self.0.load(Ordering::Relaxed)
    }
}

/// What a scalar must satisfy to be evaluated: decided predicates, the
/// memo's content bits, the certification brackets the props lane
/// needs, the scalar's at-rest gate policy (`topo::AtRestPolicy`,
/// which answers the four injected doors — the offset fit's, the
/// fitted pcurves', the plane × NURBS lane's and the shell verb's — and
/// names the scalar; the
/// part seam gathers a referenced document's product, so evaluation
/// owns a gate policy per scalar), the two per-scalar
/// analysis capabilities
/// (`crate::analysis::AxisScalar` for the parameter box,
/// `crate::analysis::SeedScalar` for the E4 seed — both scalar-free
/// options whose capability lives at the scalar), and `Send + Sync`
/// for the rayon schedule.
/// ONE name for the set, stated at the evaluation-service seam that
/// owns it — so the modules below this one (`parts`) name the
/// requirement rather than restate it, and the compound `Bounds` bound
/// stays inside the seam the 2026-07-29 Bounds scope rule ratified.
pub trait EvalScalar:
    Decide
    + ContentBits
    + geom_core::Bounds
    + Send
    + Sync
    + topo::AtRestPolicy
    + crate::analysis::AxisScalar
    + crate::analysis::SeedScalar
    + crate::measure::MinClearanceLane
    + crate::mate::SolveScalar
{
}

impl<T> EvalScalar for T where
    T: Decide
        + ContentBits
        + geom_core::Bounds
        + Send
        + Sync
        + topo::AtRestPolicy
        + crate::analysis::AxisScalar
        + crate::analysis::SeedScalar
        + crate::measure::MinClearanceLane
        + crate::mate::SolveScalar
{
}

/// **The certified-leaf replay door** (ERROR-DESIGN E12): a leaf
/// replays at the certified scalar. It lives inside `eval/mod.rs` because it names
/// `EvalScalar`, which the evaluation-service seam confines to this file
/// and `parts.rs`.
pub(crate) mod leaf {
    use super::{
        CancelToken, ContentKey, EvalOptions, EvalScalar, Evaluation, ValuePayload, evaluate,
    };

    // ------------------------------------------- the certified-leaf replay
    //
    // A leaf replays at the certified scalar, the one `crate::drive`
    // certified it at.

    /// **Which lane a certified leaf is replayed on** (ERROR-DESIGN E12).
    ///
    /// A leaf certified with the symbolic tier on can carry a node a
    /// numeric-only replay refuses — that is the whole point of the tier —
    /// so every consumer that replays a leaf has to replay it the way the
    /// driver certified it. The lane rides `ParamBoxVerdict::symbolic` from
    /// the drive to the consumer; this is the type it arrives as.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub(crate) enum LeafLane {
        /// Plain `Interval`, the pre-E12 replay.
        Numeric,
        /// `Sym<Interval>` inside a fresh session at this budget, with
        /// these atom-algebra rules and this retry ladder
        /// (`geom_core::SymRetry`) — the three dials
        /// `drive::SymbolicDials` carries, so a leaf read back here
        /// runs the tier the drive certified at and not a narrower one.
        Symbolic(
            geom_core::SymBudget,
            geom_core::SymRules,
            geom_core::SymRetry,
        ),
    }

    /// A shared memo prior over the nominal box, for the numeric lane.
    ///
    /// **The symbolic lane has none, deliberately.** A `Sym` value carries
    /// a DAG node id, its hash-consing table is per session and thread-local
    /// (`geom_core::sym::with_session`), and a memo built in one session and
    /// served into another hands the second session ids it has no nodes for.
    /// The forms would freeze rather than cancel — sound, but it would
    /// silently switch the tier off for exactly the subgraph the prior was
    /// built to save. A per-leaf session with no prior costs evaluation time
    /// and claims nothing false.
    pub(crate) enum LeafPrior {
        /// No prior — every leaf evaluates standalone.
        None,
        /// The numeric lane's shared prior.
        Numeric(Box<Evaluation<geom_core::Interval>>),
    }

    impl LeafPrior {
        /// The prior for `lane` over `nominal_box`.
        pub(crate) fn of(
            doc: &crate::doc::Doc<crate::program::ProfileProgram>,
            opts: &EvalOptions,
            lane: LeafLane,
            tol: geom_core::Tol,
        ) -> Self {
            match lane {
                LeafLane::Numeric => Self::Numeric(Box::new(evaluate(
                    doc,
                    None,
                    &CancelToken::new(),
                    opts,
                    tol,
                ))),
                LeafLane::Symbolic(..) => Self::None,
            }
        }
    }

    /// What a consumer wants read off a leaf's replay.
    #[derive(Debug, Clone, Copy, Default)]
    pub(crate) struct LeafRequest {
        /// Read the per-node content keys (the content tie's currency).
        pub keys: bool,
        /// Read this observed variable's value.
        pub measure: Option<crate::VarId>,
        /// Read this assertion node's verdict.
        pub assertion: Option<crate::node::RecipeNodeId>,
    }

    /// What a measure node's replay came to: the certified bracket, `None`
    /// inside the `Ok` when the value carries a domain violation, and the
    /// node with its reason when there was no measured value at all.
    ///
    /// A named type because the nesting is three deep and each layer means
    /// something different — a consumer reading it should meet the three
    /// states by name rather than by unwrapping.
    pub(crate) type MeasureRead =
        Result<Option<(f64, f64)>, (Option<crate::node::RecipeNodeId>, String)>;

    /// What came back.
    #[derive(Debug, Clone, Default)]
    pub(crate) struct LeafReadback {
        /// Per-node content keys, in evaluation order (empty unless asked).
        pub keys: Vec<(crate::node::RecipeNodeId, ContentKey)>,
        /// The measure node's CERTIFIED bracket — `Ok(None)` when the value
        /// carries a domain violation, `Err` when the node has no measured
        /// value at all (with the node and its reason).
        pub measure: Option<MeasureRead>,
        /// The measure node's STORED bracket, for a report that tabulates
        /// enclosures rather than certifying them.
        pub measure_bracket: Option<(f64, f64)>,
        /// The assertion node's verdict, projected onto the numeric channel.
        pub assertion: Option<crate::measure::AssertionVerdict<geom_core::Interval>>,
    }

    /// **Replays one leaf on `lane` and reads back what `want` asks for** —
    /// the ONE door every certified-leaf consumer goes through
    /// ([`LeafLane`]).
    ///
    /// It lives here rather than at each consumer because the lane is the
    /// evaluation service's own fact and because the two arms below are the
    /// only place in the tree that names both leaf scalars. A consumer that
    /// picked its own scalar would be deciding, per file, which build it is
    /// reading — which is exactly the drift that made a perfectly good
    /// verdict report as "not of this build".
    pub(crate) fn replay_leaf(
        doc: &crate::doc::Doc<crate::program::ProfileProgram>,
        opts: &EvalOptions,
        lane: LeafLane,
        prior: &LeafPrior,
        want: LeafRequest,
        tol: geom_core::Tol,
    ) -> LeafReadback {
        match lane {
            LeafLane::Numeric => {
                let prior = match prior {
                    LeafPrior::Numeric(p) => Some(&**p),
                    LeafPrior::None => None,
                };
                let ev: Evaluation<geom_core::Interval> =
                    evaluate(doc, prior, &CancelToken::new(), opts, tol);
                read_leaf(doc, &ev, want, |v| v)
            }
            LeafLane::Symbolic(budget, rules, retry) => {
                let (out, _) = geom_core::sym::with_session_retry(budget, rules, retry, || {
                    let ev: Evaluation<geom_core::Sym<geom_core::Interval>> =
                        evaluate(doc, None, &CancelToken::new(), opts, tol);
                    read_leaf(doc, &ev, want, |v: geom_core::Sym<geom_core::Interval>| {
                        v.value
                    })
                });
                out
            }
        }
    }

    /// The reads themselves, at whatever scalar the lane ran — `project`
    /// takes the lane scalar down to the numeric channel, which is where
    /// every number a consumer sees is quoted from.
    fn read_leaf<T: EvalScalar + geom_core::CertifiedEnclosure>(
        doc: &crate::doc::Doc<crate::program::ProfileProgram>,
        ev: &Evaluation<T>,
        want: LeafRequest,
        project: impl Fn(T) -> geom_core::Interval,
    ) -> LeafReadback {
        let mut out = LeafReadback::default();
        if want.keys {
            out.keys = ev
                .order
                .iter()
                .filter_map(|&id| ev.value(id).map(|v| (id, v.content_key)))
                .collect();
        }
        if let Some(var) = want.measure {
            out.measure = Some(match ev.reading_spoken(doc, var) {
                Ok(super::measure::Observed::Value(value)) => {
                    out.measure_bracket = Some((value.lo(), value.hi()));
                    Ok(geom_core::CertifiedEnclosure::certified_bracket(value))
                }
                Ok(super::measure::Observed::Unavailable(reason)) => Err((
                    Some(super::measure::unavailable_at(doc, var)),
                    format!("{reason}"),
                )),
                Err(refusal) => Err(refusal),
            });
        }
        if let Some(id) = want.assertion {
            out.assertion = match ev.value(id).map(|v| &v.payload) {
                Some(ValuePayload::Assertion(a)) => Some(a.clone().map(&project)),
                _ => None,
            };
        }
        out
    }
}

pub(crate) use leaf::{LeafLane, LeafPrior, LeafRequest, replay_leaf};

/// Evaluation options (spec D5/D6).
#[derive(Debug, Clone)]
pub struct EvalOptions {
    /// The identity token to carry (spec D5). [`EvalOptions::default`]
    /// mints a fresh one.
    pub epoch: Epoch,
    /// Whether independent nodes may run under rayon idiom 1 (spec
    /// D6). A RUNTIME switch so the D9 determinism cross-check can
    /// compare both schedules in one test run; results land by node
    /// id either way — order is data, not schedule.
    ///
    /// The `k_stats` recordings (verdicts, escalations, `probe`
    /// samples) are the serial walk's at any thread count, in the
    /// serial walk's order.
    ///
    /// A request, not a guarantee: while the calling thread has a
    /// symbolic session or shape report installed
    /// (`geom_core::sym::decisions_are_thread_portable`) the run is the
    /// serial walk.
    pub parallel: bool,
    /// Which candidate-generation path boolean nodes run (M5 PR 8) —
    /// a runtime switch in the `parallel` mold so the BVH differential
    /// suite can compare a tree-backed and a brute-force evaluation of
    /// the SAME document in one test run. Results are bit-identical
    /// either way (the tree prunes, predicates decide — the suite pins
    /// it), which is also why the strategy is deliberately NOT part of
    /// any content key. The one strategy-dependent OBSERVABLE is the
    /// verdict LOG (`NodeValue::verdicts`): pruning runs strictly
    /// fewer predicates, so logs are only comparable within one
    /// strategy (the diff engine always compares production runs).
    /// Production default: `Realized`.
    pub boolean_sweep: topo::SweepStrategy,
    /// The document seam (ASM-2A D-3): how an `InstantiatePart` node
    /// reaches the document it pins. `None` — the default — crosses no
    /// seam: a part the prior evaluation carried is served from it (the
    /// pin says which content, A4), and every other instantiate node
    /// refuses typed rather than pretending a part is empty.
    pub resolver: Option<Arc<dyn crate::part::PartResolver>>,
    /// The PARAMETER BOX this evaluation runs over (E6's leaf replay):
    /// each named axis binds `nominal + [lo, hi]` instead of the
    /// nominal. `None` — the default — is the nominal build, and is
    /// what every build-path evaluation passes.
    ///
    /// Scalar-free by construction: the box is offsets, and the
    /// evaluation's own scalar decides whether it can carry them
    /// ([`crate::analysis::AxisScalar`]). A scalar that cannot refuses
    /// the whole evaluation, node by node, rather than quietly
    /// evaluating at the nominals.
    pub param_box: Option<Arc<crate::analysis::ParamBox>>,
    /// The E4 SEED: which parameter's lift carries tangent `1.0` in
    /// this evaluation — the `param_box` seam's twin. `None` — the
    /// default — is every build-path evaluation, bit for bit: the
    /// environment is left exactly as the box door (or the nominal
    /// door) built it.
    ///
    /// Exactly one parameter per evaluation (E4: n parameters ⇒ n
    /// independent passes; a multi-seed vector mode is E11.4's door,
    /// deliberately not this field's). Scalar-free like the box: the
    /// seed is a variable, and the evaluation's scalar decides whether it
    /// can carry a tangent ([`crate::analysis::SeedScalar`]) — a
    /// tangentless scalar refuses every node typed rather than silently
    /// dropping the seed, and an unknown or `Count` variable refuses at env
    /// construction, before any node runs. Seeding composes with
    /// `param_box` exactly where both capabilities meet
    /// (`Dual<Interval>`: value channel the box, tangent the seed).
    pub seed: Option<crate::var::VarId>,
    /// **Every continuous free variable a symbol**
    /// ([`crate::analysis::var_env_symbolic`]): the environment the
    /// coincidence door replays a document in. `false` — the default —
    /// is every other evaluation. Exclusive with `param_box`, which
    /// widens named variables over a box instead: an evaluation asked
    /// for both refuses every node. So does one that also carries a
    /// `resolver` ([`crate::analysis::ParamBoxError::SymbolsAcrossAPartSeam`]).
    pub symbols: bool,
}

impl EvalOptions {
    /// **What the options say about the profile lift** — half of the
    /// rule [`profile_lift_over`](Self::profile_lift_over) reads, the
    /// half that needs no environment: guided where the
    /// [`param_box`](Self::param_box) varies an axis (a degenerate
    /// `Varying { lo: 0, hi: 0 }` included, the nominal asked for as an
    /// analysis — `ParamBox::at_nominal`), where it carries a
    /// [`seed`](Self::seed), or where it binds every variable a
    /// [`symbol`](Self::symbols); pinned otherwise. A box of only
    /// [`Fixed`](crate::analysis::BoxAxis::Fixed) axes, or none, holds
    /// every variable it names at its nominal, so an unused box seam
    /// changes no decision. `Guided` here is the evaluation's lift
    /// whatever the environment; `Pinned` here is the evaluation's lift
    /// only where the environment binds no symbol.
    #[must_use]
    pub fn profile_lift(&self) -> ProfileLift {
        let varies = self.param_box.as_ref().is_some_and(|b| {
            b.axes()
                .values()
                .any(|axis| matches!(axis, crate::analysis::BoxAxis::Varying { .. }))
        });
        if varies || self.seed.is_some() || self.symbols {
            ProfileLift::Guided
        } else {
            ProfileLift::Pinned
        }
    }

    /// **Where this evaluation's profile geometry comes from**, read off
    /// what it binds rather than chosen: an ANALYSIS — one whose
    /// [`options`](Self::profile_lift) vary an axis, seed or ask for
    /// symbols, or whose environment `env` binds a variable as a
    /// parameter symbol ([`crate::expr::VarEnv::binds_symbols`]: at
    /// `Sym`, a toleranced variable under any box, named or not, as VR8
    /// has it) — elaborates its profiles at its own scalar
    /// ([`ProfileLift::Guided`]); every other evaluation binds each
    /// variable to its nominal as a constant and is the build, at any
    /// scalar ([`ProfileLift::Pinned`]). The pinned lift reads the
    /// profile at the nominal, so in an analysis it would enter profile
    /// geometry as constants of a function the document does not
    /// compute; with every variable a constant at its nominal, the two
    /// lifts read the same values and the pinned one is the build's own
    /// bits. The evaluator takes exactly this lift.
    #[must_use]
    pub fn profile_lift_over<T>(&self, env: &crate::expr::VarEnv<T>) -> ProfileLift {
        if env.binds_symbols {
            ProfileLift::Guided
        } else {
            self.profile_lift()
        }
    }
}

/// Where profile geometry comes from at a non-`f64` scalar
/// ([`EvalOptions::profile_lift_over`] reads which, off what the
/// evaluation binds).
///
/// The document's parameters feed two kinds of slot, and until the
/// lift they behaved differently: a node's MAGNITUDE argument (an
/// extrude distance) is lane-live, so the interval lane sees a
/// genuinely interval-evaluated expression, while profile geometry is
/// f64-pinned — the lanes consume the f64 elaboration's bits through
/// `from_f64`. The asymmetry is inherited from the substrate and is
/// correct for building: structure must be selected once, identically
/// for every lane.
///
/// It is wrong for ANALYSIS. A `Dual` seed on a profile dimension
/// propagates no tangent through a constant, and an interval profile
/// parameter does not widen anything downstream of it — silent zeros
/// and points exactly where a sensitivity or an enclosure needs
/// signal. The second pass exists for those lanes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProfileLift {
    /// Profile geometry is the f64 elaboration, embedded through
    /// `from_f64` — the build path, bit for bit.
    Pinned,
    /// Profile geometry is elaborated at THIS evaluation's scalar,
    /// GUIDED by the f64 pass's structure record: every discrete
    /// decision is consumed and re-verified here, and none is remade.
    /// A decision this scalar cannot confirm refuses the node typed
    /// rather than quietly keeping the nominal structure.
    Guided,
}

/// The interval lane's solve: a refusal quotes an enclosure's midpoint,
/// the one `f64` read the lane makes of what it measured (the bracket
/// door, at the evaluation-service seam), and the angle is the
/// certified enclosure.
impl crate::mate::SolveScalar for geom_core::Interval {
    fn quoted(self) -> f64 {
        use geom_core::Bounds;
        0.5 * self.lo() + 0.5 * self.hi()
    }

    fn upper(self) -> f64 {
        geom_core::Bounds::hi(self)
    }

    fn is_stored_identity(_map: &geom_core::Affine3<Self>) -> bool {
        false
    }

    /// Across the negative `x` axis the principal branch's cut widens
    /// the enclosure to the whole circle, so there the angle is read on
    /// the branch about `π`: the solve reads an angle only through its
    /// sine and cosine, which every branch encloses alike.
    fn solve_atan2(y: Self, x: Self) -> Self {
        use geom_core::{Bounds, Real};
        if x.hi() < 0.0 && y.lo() <= 0.0 && 0.0 <= y.hi() {
            Self::pi() + (-y).atan2(-x)
        } else {
            y.atan2(x)
        }
    }

    fn carries_no_variation(_map: &geom_core::Affine3<Self>) -> bool {
        true
    }

    fn identity_varying_as(_map: &geom_core::Affine3<Self>) -> Option<geom_core::Affine3<Self>> {
        None
    }

    /// Each entry's hull with the identity's: the enclosure of every map
    /// the box spans, at rest or not.
    fn with_rest(map: &geom_core::Affine3<Self>) -> Option<geom_core::Affine3<Self>> {
        use geom_core::SpanLocate;
        use geom_core::linalg::Vec3;
        let (one, zero) = (
            <Self as geom_core::Real>::one(),
            <Self as geom_core::Real>::zero(),
        );
        let hull = |v: Vec3<Self>, w: Vec3<Self>| {
            Vec3::new(
                v.x.enclosure_hull(w.x),
                v.y.enclosure_hull(w.y),
                v.z.enclosure_hull(w.z),
            )
        };
        let mut out = *map;
        out.linear.c0 = hull(map.linear.c0, Vec3::new(one, zero, zero));
        out.linear.c1 = hull(map.linear.c1, Vec3::new(zero, one, zero));
        out.linear.c2 = hull(map.linear.c2, Vec3::new(zero, zero, one));
        out.translation = hull(map.translation, Vec3::new(zero, zero, zero));
        Some(out)
    }
}

/// The seed lane's solve: a refusal quotes the value channel — the
/// `f64` lane's own number — and the angle's value channel is the
/// `f64` lane's `atan2` with the dual's own tangent beside it.
impl<T: crate::mate::SolveScalar> crate::mate::SolveScalar for geom_core::Dual<T>
where
    geom_core::Dual<T>: geom_core::Decide,
{
    fn quoted(self) -> f64 {
        self.value.quoted()
    }

    fn upper(self) -> f64 {
        self.value.upper()
    }

    fn is_stored_identity(_map: &geom_core::Affine3<Self>) -> bool {
        false
    }

    fn solve_atan2(y: Self, x: Self) -> Self {
        let angle = geom_core::Real::atan2(y, x);
        Self {
            value: T::solve_atan2(y.value, x.value),
            deriv: angle.deriv,
        }
    }

    /// Every entry's tangent is exactly zero: a pose the seed's
    /// parameter moves keeps its derivative.
    fn carries_no_variation(map: &geom_core::Affine3<Self>) -> bool {
        let q = map.linear;
        let t = map.translation;
        [q.c0, q.c1, q.c2, t].iter().all(|v| {
            [v.x, v.y, v.z]
                .iter()
                .all(|e| e.deriv.quoted() == 0.0 && e.deriv.upper() == 0.0)
        })
    }

    /// The value channel the identity's, every tangent `map`'s.
    fn identity_varying_as(map: &geom_core::Affine3<Self>) -> Option<geom_core::Affine3<Self>> {
        use geom_core::linalg::Vec3;
        let entry = |at: Self, one: bool| Self {
            value: if one { T::one() } else { T::zero() },
            deriv: at.deriv,
        };
        let column = |c: Vec3<Self>, i: usize| {
            Vec3::new(entry(c.x, i == 0), entry(c.y, i == 1), entry(c.z, i == 2))
        };
        let q = map.linear;
        let t = map.translation;
        let mut out = *map;
        out.linear.c0 = column(q.c0, 0);
        out.linear.c1 = column(q.c1, 1);
        out.linear.c2 = column(q.c2, 2);
        out.translation = column(t, 3);
        Some(out)
    }

    fn with_rest(_map: &geom_core::Affine3<Self>) -> Option<geom_core::Affine3<Self>> {
        None
    }
}

/// The symbolic tier's solve: its lane scalar's quote, and the tier's
/// own `atan2`, which records the expression. Its value channel is
/// therefore `Real::atan2` (`libm`'s at `f64`), not the platform one the
/// `f64` and dual lanes keep, and may differ from the `f64` lane's by an
/// ulp: `Sym` mints its value and its recorded node together inside
/// `geom-core`, which has no door to set one beside the other.
impl<T: crate::mate::SolveScalar> crate::mate::SolveScalar for geom_core::Sym<T>
where
    geom_core::Sym<T>: geom_core::Decide,
{
    fn quoted(self) -> f64 {
        self.value.quoted()
    }

    fn upper(self) -> f64 {
        self.value.upper()
    }

    fn is_stored_identity(_map: &geom_core::Affine3<Self>) -> bool {
        false
    }

    fn solve_atan2(y: Self, x: Self) -> Self {
        geom_core::Real::atan2(y, x)
    }

    /// A symbolic map records its expression, which the identity would
    /// lose: it never settles.
    fn carries_no_variation(_map: &geom_core::Affine3<Self>) -> bool {
        false
    }

    fn identity_varying_as(_map: &geom_core::Affine3<Self>) -> Option<geom_core::Affine3<Self>> {
        None
    }

    fn with_rest(_map: &geom_core::Affine3<Self>) -> Option<geom_core::Affine3<Self>> {
        None
    }
}

impl Default for EvalOptions {
    fn default() -> Self {
        Self {
            epoch: Epoch::mint(),
            parallel: false,
            boolean_sweep: topo::SweepStrategy::Realized,
            resolver: None,
            param_box: None,
            seed: None,
            symbols: false,
        }
    }
}

/// **A part's reach over its row** — a part's extent, answered from
/// the parts an evaluation resolves.
///
/// Every caller outside a run — the viewer's mate tool, the Python
/// door, the demos, the test fixtures — builds a store through
/// [`mate_reach`] over the options it would evaluate with, so there is
/// one implementation of "a part's extent" and it is the evaluation's.
fn reach_of<T: EvalScalar>(
    row: Result<parts::PartValue<T>, parts::PartFault>,
) -> Result<f64, crate::mate::ReachRefusal> {
    let value = row.map_err(|fault| crate::mate::ReachRefusal::PartUnresolved { fault })?;
    // The reach is an UPPER BOUND by definition, and the bracket's `hi`
    // is that bound at the run's scalar (`EvalScalar` gathers the
    // ratified compound; on `f64` the bracket is the value). It scales
    // a margin in the refusal-safe direction and decides no topology:
    // the solve still decides through `Decide`. A bracket that reads
    // back non-finite is poison, not a bound.
    let hi = crate::mate::part_reach(&value.body)?.hi();
    if !hi.is_finite() {
        return Err(crate::mate::ReachRefusal::NoFiniteBound);
    }
    Ok(hi)
}

/// **A mate solve's reach, built outside an evaluation** — what
/// [`mate_reach`] answers. Owns a part store over `opts`' resolver
/// that evaluates each part on its first ask, so a caller that solves
/// a document and then evaluates it resolves each mated part once here
/// and once there; the evaluation's own solve reads its run's table
/// instead. Bound to no document: the edit door threads one reach
/// through a group of edits, each applied to its predecessor's output.
///
/// Not `Sync`: the store fills by `&mut` behind a `RefCell`, so one
/// thread asks at a time and no ask waits on another's evaluation. It
/// can still be sent:
///
/// ```compile_fail,E0277
/// fn shared<R: Sync>() {}
/// shared::<editor_core::PartReach<'static, f64>>();
/// ```
///
/// ```
/// fn sent<R: Send>() {}
/// sent::<editor_core::PartReach<'static, f64>>();
/// ```
pub struct PartReach<'a, T: EvalScalar> {
    parts: core::cell::RefCell<parts::PartStore<'a, T>>,
    tol: Tol,
}

impl<'a, T: EvalScalar> PartReach<'a, T> {
    /// **A reach from a resolver alone** — for the doors that are not
    /// a run: the edit door, a solve read outside an evaluation, a
    /// refactoring. Such a door has no evaluation options of its own,
    /// so its parts evaluate under the defaults (`EvalOptions`'s sweep
    /// strategy), spelled here once; a door that IS
    /// a run, or holds a run's options, goes through [`mate_reach`].
    /// `None` is the refusing reach: every part is the typed
    /// no-resolver fault.
    pub fn with_resolver(
        resolver: Option<&'a Arc<dyn crate::part::PartResolver>>,
        tol: Tol,
    ) -> Self {
        let defaults = EvalOptions::default();
        Self::over(resolver, defaults.boolean_sweep, tol)
    }

    /// A reach over a part store built from these options' parts.
    fn over(
        resolver: Option<&'a Arc<dyn crate::part::PartResolver>>,
        boolean_sweep: topo::SweepStrategy,
        tol: Tol,
    ) -> Self {
        Self {
            parts: core::cell::RefCell::new(parts::PartStore::<T>::new(
                resolver,
                boolean_sweep,
                tol,
            )),
            tol,
        }
    }

    /// The row `part` has, evaluated on its first ask. Outside any run,
    /// so nothing cancels it.
    fn row(&self, part: &crate::ident::DocRef) -> Result<parts::PartValue<T>, parts::PartFault> {
        self.parts
            .borrow_mut()
            .row(part, self.tol, &CancelToken::new())
    }
}

impl<T: EvalScalar> crate::mate::MateReach<T> for PartReach<'_, T> {
    fn reach(&self, part: &crate::ident::DocRef) -> Result<f64, crate::mate::ReachRefusal> {
        reach_of(self.row(part))
    }
}

/// **The public door to the evaluation's reach** for
/// [`crate::mate::solve_document`] and [`crate::edit::apply`]: each
/// mated part's extent, resolved through `opts`' resolver the way an
/// evaluation over `opts` would resolve it (same seam, same sweep
/// strategy), at the top of the descent. A caller
/// with no resolver gets a reach whose every answer is the typed
/// no-resolver fault, so the solve refuses each mate in the
/// resolver's own voice rather than levering over nothing.
pub fn mate_reach<'a, T: EvalScalar>(opts: &'a EvalOptions, tol: Tol) -> PartReach<'a, T> {
    PartReach::over(opts.resolver.as_ref(), opts.boolean_sweep, tol)
}

/// **A pose variable bound against an evaluation**, as a reader at
/// `slot` binds it ([`wire::pose`]): a definition at the evaluation's
/// lane scalar from the results it reads, an operation's output from
/// that operation's value. Pose values are bound at their readers and
/// held nowhere, so this is how a test reads one without a reader.
///
/// # Errors
///
/// The refusal the reader at `slot` would carry.
#[doc(hidden)]
pub fn bound_pose<T: EvalScalar>(
    doc: &crate::ProfileDoc,
    ev: &Evaluation<T>,
    slot: crate::OperandSlot,
    read: impl Into<crate::pose::RoleRead<crate::VarId>>,
    tol: Tol,
) -> Result<PoseValue<T>, NodeErrorKind> {
    wire::pose::eval_pose(doc, &ev.nodes, slot, read.into(), &ev.env, tol)
}

/// **Every node whose value lives in a loose copy's space**, in
/// schedule order: a value's own space where it built; for a node that
/// did not, the space its recipe gives it (D10: kinds are decided from
/// the recipe) — a placement with no mate is a loose copy of its own,
/// one whose mates read the world is the world's, one whose mates read
/// poses joins the space of what they read; any other node joins the
/// first loose space among its inputs.
fn spaces_of<T: Decide>(
    doc: &Doc<ProfileProgram>,
    order: &[RecipeNodeId],
    nodes: &BTreeMap<RecipeNodeId, NodeResult<T>>,
) -> BTreeMap<RecipeNodeId, RecipeNodeId> {
    use crate::mate::{Space, Target};
    let mut loose: BTreeMap<RecipeNodeId, RecipeNodeId> = BTreeMap::new();
    for &id in order {
        let space = match nodes.get(&id) {
            Some(NodeResult::Ok(value)) => value.space,
            Some(_) => {
                let Some(node) = doc.node(id) else { continue };
                let read_space =
                    |var: crate::VarId, loose: &BTreeMap<RecipeNodeId, RecipeNodeId>| {
                        doc.read_ports(var)
                            .iter()
                            .find_map(|(at, _)| loose.get(at).copied())
                            .map_or(Space::World, |root| Space::Loose { root })
                    };
                match crate::mate::solve::bundle_of(node) {
                    Some(bundle) => {
                        let mut mates = bundle.iter().filter_map(crate::mate::Constraint::mate);
                        Some(match mates.next() {
                            None => Space::Loose { root: id },
                            Some(mate) => match &mate.to {
                                Target::World(_) => Space::World,
                                Target::Pose(var) => read_space(*var, &loose),
                            },
                        })
                    }
                    None => doc
                        .upstream_of(id, node)
                        .iter()
                        .find_map(|at| loose.get(at).copied())
                        .map(|root| Space::Loose { root }),
                }
            }
            None => None,
        };
        if let Some(Space::Loose { root }) = space {
            loose.insert(id, root);
        }
    }
    loose
}

/// **[`crate::mate::solve_document`] at a run's own scalar** — a box
/// run's poses enclosing each copy's over the box, a seed run's
/// carrying their tangents — under `opts`, whose box or seed the run
/// reads. Here, at the evaluation seam, because it reads an evaluation
/// at a run's scalar.
pub fn solve_document_at<T>(
    doc: &Doc<ProfileProgram>,
    opts: &EvalOptions,
    tol: Tol,
) -> crate::mate::SolvedPoses<T>
where
    T: crate::mate::SolveScalar + EvalScalar,
{
    let evaluation = evaluate::<T>(doc, None, &crate::CancelToken::new(), opts, tol);
    crate::mate::SolvedPoses::of(doc, &evaluation)
}

impl<T: crate::mate::SolveScalar + EvalScalar> crate::mate::SolvedPoses<T> {
    /// **Every placement's answer in `evaluation`**, the evaluation of
    /// `doc`: a placement that evaluated, its space and motion; one
    /// that refused its bundle, the fault.
    pub fn of(doc: &Doc<ProfileProgram>, evaluation: &Evaluation<T>) -> Self {
        let mut placed = BTreeMap::new();
        for id in doc.ids() {
            if doc
                .node(id)
                .and_then(crate::mate::solve::bundle_of)
                .is_none()
            {
                continue;
            }
            match evaluation.result(id) {
                Some(NodeResult::Ok(value)) => {
                    let space = value.space.unwrap_or(crate::mate::Space::World);
                    placed.insert(id, Ok((space, value.motion.map(|m| m.to_affine()))));
                }
                Some(NodeResult::Failed(error)) => {
                    if let NodeErrorKind::Mate(fault) = &error.kind {
                        placed.insert(id, Err((**fault).clone()));
                    }
                }
                _ => {}
            }
        }
        Self {
            document: doc.id(),
            placed,
        }
    }
}

/// **A pose read bound against an evaluation, with no reader seat**
/// — a report's read of a pose variable ([`crate::mate::unobservable`]).
pub(crate) fn read_pose_in<T: EvalScalar>(
    doc: &crate::ProfileDoc,
    ev: &Evaluation<T>,
    read: crate::pose::RoleRead<crate::VarId>,
    tol: Tol,
) -> Result<PoseValue<T>, NodeErrorKind> {
    wire::pose::read_pose(doc, &ev.nodes, read, &ev.env, tol)
}

/// **A mate's world read bound at an evaluation's parameters**
/// ([`read_pose_in`]'s twin for the world).
pub(crate) fn world_pose_in<T: EvalScalar>(
    world: &crate::mate::WorldRead,
    ev: &Evaluation<T>,
    tol: Tol,
) -> Result<PoseValue<T>, NodeErrorKind> {
    wire::pose::eval_world(world, &ev.env, wire::band(tol)?)
}

/// Evaluates the document (spec D2–D6): a TOTAL function — every
/// failure is a per-node typed result, never a top-level error or a
/// panic.
///
/// `prior` is the memo (spec D4): nodes whose content key matches the
/// prior evaluation reuse the prior value without re-running the op;
/// only the downstream cone of changed keys re-evaluates. A prior of
/// ANOTHER document is not a memo for this one (DI3): it is refused
/// whole, recorded on [`Evaluation::prior_refused`], and the run
/// recomputes every node.
///
/// `cancel` is checked between nodes on the serial walk and between
/// levels on the level schedule — spec D5's cooperative yield points at
/// node granularity; a canceled run returns the completed prefix with
/// [`EvalOutcome::Canceled`]. A `parallel` request runs as the serial
/// walk, and is checked between nodes, while a symbolic session or
/// shape report is installed ([`EvalOptions::parallel`]).
pub fn evaluate<T>(
    doc: &Doc<ProfileProgram>,
    prior: Option<&Evaluation<T>>,
    cancel: &CancelToken,
    opts: &EvalOptions,
    tol: Tol,
) -> Evaluation<T>
where
    T: EvalScalar,
{
    evaluate_at_descent(doc, prior, cancel, opts, None, tol)
}

/// An instantiated document's own evaluation (ASM-2A D-3), one level
/// deeper than its instantiator's. `reached` holds the parts the
/// document instantiates, already evaluated by the descent (see
/// [`parts`] on why it runs bottom-up). No prior: a referenced document
/// is resolved fresh, and the memo that keeps THAT from costing
/// anything is the top's part rows, carried from its prior.
pub(crate) fn evaluate_nested<T>(
    doc: &Doc<ProfileProgram>,
    cancel: &CancelToken,
    opts: &EvalOptions,
    reached: parts::Reached<T>,
    tol: Tol,
) -> Evaluation<T>
where
    T: EvalScalar,
{
    evaluate_at_descent(doc, None, cancel, opts, Some(reached), tol)
}

/// Whether `doc` was recorded at the process's ε — the D4 door below.
/// A document that was not refuses every node before any of them runs,
/// so its evaluation reaches none of its parts.
pub(crate) fn recorded_at_process_eps<P>(doc: &Doc<P>, tol: Tol) -> bool {
    doc.epsilon().to_bits() == tol.eps().to_bits()
}

fn evaluate_at_descent<T>(
    doc: &Doc<ProfileProgram>,
    prior: Option<&Evaluation<T>>,
    cancel: &CancelToken,
    opts: &EvalOptions,
    reached: Option<parts::Reached<T>>,
    tol: Tol,
) -> Evaluation<T>
where
    T: EvalScalar,
{
    // The memo's own door (DI3): a prior is a prior OF a document, and
    // node ids are minted per document, so an evaluation of another
    // document can collide on them and be mined for hits that are
    // about other geometry. Refused HERE, once, before the schedule is
    // even built — which is what makes the per-node lookup below
    // (`prior.and_then(|p| p.nodes.get(&id))`) a lookup that cannot
    // reach a foreign node, rather than a check repeated per node.
    let (prior, prior_refused) =
        match prior.and_then(|p| crate::ident::mispaired(doc.id(), p.document).map(|m| (p, m))) {
            Some((_, refused)) => (None, Some(refused)),
            None => (prior, None),
        };
    let sched = schedule::schedule(doc);
    // D4 door (M4 PR 6): the recorded ε must BE the committed process
    // ε — otherwise every predicate below would silently decide at
    // the wrong tolerance. Refuse loudly, per node, staying total.
    if !recorded_at_process_eps(doc, tol) {
        return refuse_tolerance_conflict(doc, sched, opts, prior_refused, tol.eps());
    }
    // The lane environment, built ONCE and shared by every reader
    // below (slot evaluation, the lift's second pass, the two profile
    // ladders): one environment per evaluation is what makes "this run
    // evaluated over that box" a fact about the run rather than about
    // each call site.
    let env = match (opts.param_box.as_deref(), opts.symbols) {
        (None, false) => Ok(doc.var_env::<T>()),
        (None, true) if opts.resolver.is_some() => {
            Err(crate::analysis::ParamBoxError::SymbolsAcrossAPartSeam)
        }
        (None, true) => crate::analysis::var_env_symbolic::<T, _>(doc),
        (Some(b), false) => crate::analysis::var_env_over::<T, _>(doc, b),
        (Some(_), true) => Err(crate::analysis::ParamBoxError::SymbolsOverABox),
    };
    let env = match env {
        Ok(env) => env,
        Err(source) => return refuse_param_box(doc, sched, opts, prior_refused, source),
    };
    // The E4 seed rides the SAME environment (a seed is a separate act
    // on the environment the box door built, never a property of the
    // box — `AxisScalar`'s dual impl states the boundary): exactly one
    // binding gains tangent 1.0, checked here, before any node runs.
    let env = match opts.seed {
        None => env,
        Some(var) => match crate::analysis::seed_env(doc, env, var) {
            Ok(env) => env,
            Err(source) => return refuse_seed(doc, sched, opts, prior_refused, source),
        },
    };
    // The NOMINAL environment, built beside the lane one and carried
    // with it as `wire::LaneEnv::nominal` — what it is and who reads
    // it is stated there; why the key owes it, at `tag::slot`.
    let nominal_env = doc.var_env::<f64>();
    // The parts, entered before anything below reads one: handed down
    // by the descent below the top, and entered here at the top, in
    // one serial pass that reuses what the prior carried (`parts`
    // module docs).
    let entry = match reached {
        Some(reached) => parts::Entry {
            reached,
            carried: parts::Carried::none(),
            evaluations: 0,
        },
        None => parts::PartStore::<T>::new(opts.resolver.as_ref(), opts.boolean_sweep, tol)
            .with_symbols(opts.symbols)
            .enter_census(doc, prior.map(|p| &p.parts), cancel, tol),
    };
    // A placement's bundle is folded where the placement evaluates
    // (INTENT stage 3 C): every pose a mate reads is then a value of
    // this run, at its scalar over the lane environment (A11 (5)), so a
    // seed or a box that binds what a mate reads moves the copy with it.
    let op_env = wire::OpEnv {
        boolean_sweep: opts.boolean_sweep,
        parts: &entry.reached,
        lane: wire::LaneEnv {
            lift: opts.profile_lift_over(&env),
            params: &env,
            nominal: &nominal_env,
        },
    };
    let mut nodes: BTreeMap<RecipeNodeId, NodeResult<T>> = BTreeMap::new();
    let mut recomputed = 0usize;
    let mut reused = 0usize;
    let mut outcome = EvalOutcome::Completed;

    // The level schedule runs only where a decision is thread-portable
    // (`geom_core::sym::decisions_are_thread_portable`): under an
    // installed symbolic session or shape report a node decided on a
    // worker would decide, count and report differently from one
    // decided here, so the walk is the serial one below, exactly.
    if opts.parallel && geom_core::sym::decisions_are_thread_portable() {
        // What each node recorded outside its own bracket — its `probe`
        // samples, and anything its bracket did not take — held until
        // the walk ends and then spliced in `sched.order`, which is the
        // order the serial walk records in. Levels are not that order
        // (a later level's node can precede an earlier level's in it),
        // so splicing level by level would give the same population in
        // a schedule-shaped sequence.
        let mut recordings: BTreeMap<RecipeNodeId, geom_core::k_stats::Detached> = BTreeMap::new();
        for level in &sched.levels {
            if cancel.is_canceled() {
                outcome = EvalOutcome::Canceled;
                break;
            }
            // D6 / D9-addendum idiom 1: an INDEXED parallel map into
            // per-node slots — results land keyed by node id, the
            // combination is positional (one map entry per node),
            // never arithmetic, so the schedule cannot leak into bits.
            // Each node runs under a K-funnel frame of its own; its
            // bracket in `eval_node` nests inside that frame.
            let results = geom_core::k_stats::map_detached(level, |&id| {
                eval_node(doc, &env, id, &nodes, prior, &op_env, tol)
            });
            for (&id, (step, recording)) in level.iter().zip(results) {
                bookkeep(&step, &mut recomputed, &mut reused);
                nodes.insert(id, step.result);
                recordings.insert(id, recording);
            }
        }
        for id in &sched.order {
            if let Some(recording) = recordings.remove(id) {
                geom_core::k_stats::splice(recording);
            }
        }
    } else {
        for &id in &sched.order {
            if cancel.is_canceled() {
                outcome = EvalOutcome::Canceled;
                break;
            }
            let step = eval_node(doc, &env, id, &nodes, prior, &op_env, tol);
            bookkeep(&step, &mut recomputed, &mut reused);
            nodes.insert(id, step.result);
        }
    }

    // Cycle leftovers (unreachable through `apply`): typed refusals,
    // appended after the schedulable order so `order` still covers
    // every live node.
    let mut order = sched.order;
    for &id in &sched.unschedulable {
        order.push(id);
        if outcome == EvalOutcome::Completed {
            nodes.insert(
                id,
                NodeResult::Failed(NodeError {
                    node: id,
                    kind: NodeErrorKind::UnschedulableCycle,
                    escalations: Arc::new(Vec::new()),
                }),
            );
        }
    }

    // Appearance resolution (M4 PR 7): a total post-pass over the
    // result DAG — canceled prefixes resolve what completed and report
    // the rest as typed losses carrying their standing.
    let resolved_appearance = resolve_appearance(doc, &order, &nodes);
    let loose = spaces_of(doc, &order, &nodes);

    Evaluation {
        epoch: opts.epoch,
        document: doc.id(),
        prior_refused,
        order,
        nodes,
        outcome,
        recomputed,
        reused,
        loose,
        part_evaluations: entry.evaluations,
        appearance: resolved_appearance,
        env,
        reads: Arc::new(doc.output_ports()),
        parts: entry.carried,
    }
}

/// **The operation an operand reads** — the node whose output `var`
/// is — or the reader's refusal: a read no live operation defines is
/// unresolved (D10), typed at `slot`.
pub(crate) fn read_at<P>(
    doc: &Doc<P>,
    slot: crate::OperandSlot,
    var: crate::VarId,
) -> Result<RecipeNodeId, NodeErrorKind> {
    doc.operation_of(var)
        .ok_or(NodeErrorKind::UnresolvedRead { slot, var })
}

/// The all-nodes ToleranceConflict refusal (spec D4 door).
fn refuse_tolerance_conflict<T>(
    doc: &Doc<ProfileProgram>,
    sched: schedule::Schedule,
    opts: &EvalOptions,
    prior_refused: Option<Mispaired>,
    process_eps: f64,
) -> Evaluation<T>
where
    T: Decide + ContentBits + geom_core::Bounds + Send + Sync,
{
    let document_eps = doc.epsilon();
    refuse_every_node(doc, sched, opts, prior_refused, move || {
        NodeErrorKind::ToleranceConflict {
            document_eps,
            process_eps,
        }
    })
}

/// The all-nodes parameter-box refusal: the box named a parameter the
/// document does not have, or this evaluation's scalar cannot carry a
/// widened axis. Loud on every node rather than narrowed to the
/// nominals — an `f64` run that silently ignored its box would report
/// the nominal build's answer for a question about a box.
fn refuse_param_box<T>(
    doc: &Doc<ProfileProgram>,
    sched: schedule::Schedule,
    opts: &EvalOptions,
    prior_refused: Option<Mispaired>,
    source: crate::analysis::ParamBoxError,
) -> Evaluation<T>
where
    T: Decide + ContentBits + geom_core::Bounds + Send + Sync,
{
    refuse_every_node(doc, sched, opts, prior_refused, move || {
        NodeErrorKind::ParamBox {
            source: source.clone(),
        }
    })
}

/// The all-nodes seed refusal: the seed named a parameter the document
/// does not have (or a `Count` one), or this evaluation's scalar
/// carries no tangent channel. Loud on every node for the
/// [`refuse_param_box`] reason — an evaluation that silently dropped
/// its seed would report a build's answer for a sensitivity question.
fn refuse_seed<T>(
    doc: &Doc<ProfileProgram>,
    sched: schedule::Schedule,
    opts: &EvalOptions,
    prior_refused: Option<Mispaired>,
    source: crate::analysis::SeedError,
) -> Evaluation<T>
where
    T: Decide + ContentBits + geom_core::Bounds + Send + Sync,
{
    refuse_every_node(doc, sched, opts, prior_refused, move || {
        NodeErrorKind::Seed {
            source: source.clone(),
        }
    })
}

/// A TOTAL evaluation in which every live node fails typed with the
/// same document-level cause, and the appearance store resolves against
/// all-failed states (typed losses, nothing silent).
fn refuse_every_node<T>(
    doc: &Doc<ProfileProgram>,
    sched: schedule::Schedule,
    opts: &EvalOptions,
    prior_refused: Option<Mispaired>,
    kind: impl Fn() -> NodeErrorKind,
) -> Evaluation<T>
where
    T: Decide + ContentBits + geom_core::Bounds + Send + Sync,
{
    let mut order = sched.order;
    order.extend(sched.unschedulable.iter().copied());
    let nodes: BTreeMap<RecipeNodeId, NodeResult<T>> = order
        .iter()
        .map(|&id| {
            (
                id,
                NodeResult::Failed(NodeError {
                    node: id,
                    kind: kind(),
                    escalations: Arc::new(Vec::new()),
                }),
            )
        })
        .collect();
    let resolved_appearance = resolve_appearance(doc, &order, &nodes);
    Evaluation {
        epoch: opts.epoch,
        document: doc.id(),
        prior_refused,
        order,
        nodes,
        outcome: EvalOutcome::Completed,
        recomputed: 0,
        reused: 0,
        loose: BTreeMap::new(),
        part_evaluations: 0,
        appearance: resolved_appearance,
        env: doc.var_env::<T>(),
        reads: Arc::new(doc.output_ports()),
        parts: parts::Carried::none(),
    }
}

/// The document's appearance store against one evaluation's results:
/// every node of `order` — which covers every live node — answers its
/// name table or its standing through [`usable_in`], so a node the
/// order does not hold is one the document does not have.
fn resolve_appearance<T: Decide>(
    doc: &Doc<ProfileProgram>,
    order: &[RecipeNodeId],
    nodes: &BTreeMap<RecipeNodeId, NodeResult<T>>,
) -> AppearanceResolution {
    let states: BTreeMap<RecipeNodeId, appearance::NodeState<'_>> = order
        .iter()
        .map(|&id| {
            let state = usable_in(nodes, id, || NodeStanding::NotEvaluated { node: id });
            (
                id,
                state.map(|v| {
                    (0u8..)
                        .map_while(|port| v.output(port).map(|out| (u32::from(port), out.table())))
                        .collect()
                }),
            )
        })
        .collect();
    let reads = appearance::Reads {
        ports: doc.output_ports(),
        feeds: doc.feeds(),
    };
    appearance::resolve(doc.appearance(), order, &states, &reads)
}

/// One node's evaluation step: the result plus whether it was a memo
/// reuse (for the D4 counters).
struct NodeStep<T: Decide> {
    result: NodeResult<T>,
    reused: bool,
}

fn bookkeep<T: Decide>(step: &NodeStep<T>, recomputed: &mut usize, reused: &mut usize) {
    match &step.result {
        NodeResult::Ok(_) | NodeResult::Failed(_) => {
            if step.reused {
                *reused += 1;
            } else {
                *recomputed += 1;
            }
        }
        NodeResult::Poisoned { .. } => {}
    }
}

/// Evaluates one node against the results so far: poison propagation,
/// slot evaluation with (node, slot) context, content key, memo
/// lookup, and the op wiring.
fn eval_node<T>(
    doc: &Doc<ProfileProgram>,
    env: &crate::expr::VarEnv<T>,
    id: RecipeNodeId,
    results: &BTreeMap<RecipeNodeId, NodeResult<T>>,
    prior: Option<&Evaluation<T>>,
    op_env: &wire::OpEnv<'_, T>,
    tol: Tol,
) -> NodeStep<T>
where
    T: EvalScalar,
{
    // The verdict bracket (N5): one frame per node, open from before
    // the first thing that can decide on its behalf to after the op,
    // so the node's log is every decision made evaluating THIS node,
    // pre-key and op alike, in the order made, through the one
    // `k_stats` funnel. Before the key, only a Profile node decides,
    // and only in its replay and f64 validation (`prepare_profile`);
    // slot and program-expression evaluation reach the funnel through
    // `check_unlogged`, which lands in no frame. The guard is `!Send`, so the
    // frame closes on the worker that opened it (idiom-1 parallelism
    // runs whole nodes on one worker each); an op that evaluates
    // another document (an instantiated part) has that document's
    // nodes open and close their own frames ABOVE this one, so theirs
    // land on their own nodes and this frame receives only this node's.
    let bracket = geom_core::k_stats::Bracket::open();
    // A failure carries what the frame recorded on the way to it,
    // whether or not the op ran.
    let fail = |bracket: geom_core::k_stats::Bracket, kind: NodeErrorKind| NodeStep {
        result: NodeResult::Failed(NodeError {
            node: id,
            kind,
            escalations: Arc::new(bracket.finish().escalations),
        }),
        reused: false,
    };
    let Some(node) = doc.node(id) else {
        // Unreachable: the schedule only lists live nodes.
        return fail(bracket, NodeErrorKind::MissingInput { input: id });
    };
    // Poison propagation (spec D2, GQ2): first blocking input in the
    // node's deterministic input order; `through` always names a
    // FAILED node (propagated through poisoned intermediaries).
    // A read no live operation defines is the reader's own refusal
    // (D10): a deleted operation leaves its readers unresolved, and a
    // selection's or a pose definition's readers when it was what they
    // read.
    if let Some((slot, var)) = node
        .operand_rows()
        .into_iter()
        .find_map(|(slot, var)| doc.unresolved_read(var).map(|at| (slot, at)))
    {
        return fail(bracket, NodeErrorKind::UnresolvedRead { slot, var });
    }
    let mut keys: BTreeMap<RecipeNodeId, (ContentKey, NamingKey)> = BTreeMap::new();
    for input in doc.upstream_of(id, node) {
        // Every input the document has precedes this node in the
        // order and so has its result: an absent one is not in it.
        match usable_in(results, input, || NodeStanding::NotInDocument {
            node: input,
        }) {
            Ok(v) => {
                keys.insert(input, (v.content_key, v.naming_key));
            }
            Err(NodeStanding::NotEvaluated { .. } | NodeStanding::NotInDocument { .. }) => {
                return fail(bracket, NodeErrorKind::MissingInput { input });
            }
            Err(
                NodeStanding::Failed { node: through } | NodeStanding::Poisoned { through, .. },
            ) => {
                return NodeStep {
                    result: NodeResult::Poisoned { through },
                    reused: false,
                };
            }
        }
    }
    // What the keys read of upstream: one entry per READ, in field
    // order, as the operation it reads and the port (D10; DR-59's
    // class: two outputs of one operation are two inputs, and a key
    // that saw only the operation would serve one half's body for the
    // other); a selection's read is its body's. Every operation here
    // has its result (above). A placement's constraint side reading its
    // own copy is no upstream read (`Doc::upstream_of`'s one exemption,
    // the same predicate): the node's own key is what is being formed,
    // and the side's selection names still key it.
    let reads: Vec<(RecipeNodeId, u8)> = node
        .operand_rows()
        .into_iter()
        .filter(|&(seat, var)| !crate::mate::solve::own_copy_side(doc, id, seat, var))
        .flat_map(|(_, var)| doc.read_ports(var))
        .collect();
    let key_of = |at: &RecipeNodeId| {
        *keys
            .get(at)
            .unwrap_or_else(|| unreachable!("every operation a node reads is in its upstream"))
    };
    let upstream_keys: Vec<UpstreamRead<ContentKey>> = reads
        .iter()
        .map(|(at, port)| UpstreamRead {
            at: *at,
            port: *port,
            key: key_of(at).0,
        })
        .collect();
    let upstream_naming: Vec<UpstreamRead<NamingKey>> = reads
        .iter()
        .map(|(at, port)| UpstreamRead {
            at: *at,
            port: *port,
            key: key_of(at).1,
        })
        .collect();

    // Slot evaluation — the (node, slot) context door (PR 1's banked
    // NonFiniteResult obligation lands here). TWICE, through the one
    // door: at the evaluation scalar, which is what the op runs on,
    // and at the document's nominal, which is what the f64-pinned
    // readers decide from and therefore what the content key owes
    // them (`tag::slot`). A nominal that refuses refuses the NODE, in
    // the shape a lane refusal already has: no reader of that nominal
    // could have read it either.
    let slot_values = match slots::eval_slots(node, env) {
        Ok(v) => v,
        Err((slot, source)) => return fail(bracket, NodeErrorKind::Expr { slot, source }),
    };
    let nominal_values = match slots::eval_slots(node, op_env.lane.nominal) {
        Ok(v) => v,
        Err((slot, source)) => return fail(bracket, NodeErrorKind::Expr { slot, source }),
    };
    let pose_words = match wire::pose::key_words(doc, node, env, op_env.lane.nominal) {
        Ok(words) => words,
        Err(kind) => return fail(bracket, kind),
    };
    // A node whose reads lie in two spaces relates two raw coordinate
    // systems, which only a mate may do (D10), and refuses before it
    // reads them. Every input it reads has its value (above), and
    // carries its space.
    let inputs_space = match input_space(doc, id, node, results) {
        Ok(space) => space,
        Err((a, b)) => return fail(bracket, NodeErrorKind::SpaceMismatch { a, b }),
    };

    // Profile-program resolution (LIB-SWITCH §4b): program Exprs
    // resolve at f64 because they feed C6 structure selection, which
    // must be lane-identical (the verified asymmetry: node magnitude
    // slots stay lane-live, profile geometry is f64-pinned). Resolved
    // ONCE here; the same values feed the content key (resolved-value
    // convention, §4e) and the op.
    //
    // "and never at `T`" was true until M10-P and is not any more, so
    // it is qualified rather than dropped: under `ProfileLift::Guided`
    // the SAME program is additionally resolved at `T` below. What is
    // still f64-only is the resolution that STRUCTURE is selected from
    // — this one — which is the part the C6 sentence was ever about.
    let resolved_program = match node {
        crate::node::Node::Profile(program) => match program.resolve(op_env.lane.nominal) {
            Ok(r) => Some(r),
            Err((slot, source)) => return fail(bracket, NodeErrorKind::Expr { slot, source }),
        },
        _ => None,
    };

    // The profile F64 PRECOMPUTE (replay + f64 validation + the naming
    // anchor): C6 structure selection — the successor of the stored
    // f64 bits — made on this node's behalf, so its decisions are the
    // first entries of the node's log: the whole of it under the
    // pinned lift, and ahead of the op's own replay and validation
    // under the guided lift.
    let profile_pre = match (node, &resolved_program) {
        (crate::node::Node::Profile(program), Some(resolved)) => {
            match wire::prepare_profile(resolved, &program.ids, tol) {
                Ok(pre) => Some(pre),
                Err(kind) => return fail(bracket, kind),
            }
        }
        _ => None,
    };

    // PAYLOAD EXPRESSIONS (E10): an assertion's value and bound are
    // not slots. They are evaluated exactly once here, under the
    // discipline `slots` states and for the same reason: these values
    // feed BOTH the content key and the op, so a parameter edit under a
    // bound moves the key rather than serving a stale memo. The value
    // reads what the measures under it computed in this run
    // (`measure::observe`): it is the one read of a measured value, and
    // the environment every construction reads holds none.
    let payload = match node.payload_reads(doc) {
        None => None,
        Some(reads) => {
            let mut payload = Payload {
                values: Vec::with_capacity(reads.len()),
                unavailable: None,
            };
            for (index, (var, dim)) in reads.into_iter().enumerate() {
                match measure::observe(doc, env, results, var, dim) {
                    Ok(measure::Observed::Value(v)) => payload.values.push(v),
                    Ok(measure::Observed::Unavailable(reason)) => {
                        payload.unavailable.get_or_insert((index, reason));
                    }
                    Err(measure::ObservedRefusal::Expr(source)) => {
                        let what = if index == 0 {
                            "the assertion's value,"
                        } else {
                            "the assertion's bound,"
                        };
                        return fail(
                            bracket,
                            NodeErrorKind::PayloadExpr {
                                what,
                                index,
                                source,
                            },
                        );
                    }
                    // Every measure under the value is upstream of this
                    // node, and a failed one poisoned it above.
                    Err(measure::ObservedRefusal::Measure(standing)) => {
                        return fail(
                            bracket,
                            NodeErrorKind::MissingInput {
                                input: standing.node(),
                            },
                        );
                    }
                }
            }
            Some(payload)
        }
    };
    let payload_values = payload.as_ref();

    // The lift's second pass resolves the SAME program at the lane
    // scalar (M10-P PP5). It feeds the content key so a seeded or
    // widened profile parameter moves the key the way a magnitude slot
    // already does — otherwise two evaluations differing only in a
    // profile seed would hit each other's memo. Not computed at all
    // when the lift is off, which is what keeps the f64 build path
    // exactly where it was.
    //
    // THE OP RESOLVES IT AGAIN, and that is a considered decline rather
    // than an oversight. Threading this value into `run_op` would tie
    // the key's inputs and the op's inputs together through one more
    // parameter on a function that already carries eight, and buy
    // nothing about correctness: resolution is a pure function of
    // (program, environment) evaluated by libm-pure `expr::eval`, so
    // D9 makes the second call bit-identical to this one by the same
    // argument that makes the memo's re-runs sound. What it costs is a
    // few dozen expression evaluations per profile node, in analysis
    // mode only. If that ever shows up in a profile, the fix is to pass
    // the value through — not to cache it somewhere both readers reach.
    let lane_program = match (op_env.lane.lift, node, &resolved_program) {
        (ProfileLift::Guided, crate::node::Node::Profile(program), Some(_)) => {
            match program.resolve(env) {
                Ok(r) => Some(r),
                Err((slot, source)) => return fail(bracket, NodeErrorKind::Expr { slot, source }),
            }
        }
        _ => None,
    };

    let content_key = content_key(
        node,
        &crate::param_source::definitions_of(doc),
        &|var| doc.selection(var),
        &slot_values,
        &nominal_values,
        payload_values,
        pose_words.as_deref(),
        resolved_program.as_deref(),
        lane_program.as_deref(),
        &upstream_keys,
        doc.witness(id),
        tol,
    );
    let naming_key = naming_key(content_key, &upstream_naming);

    // Memo (spec D4 + #95 disposition 2): a content-key match
    // certifies the GEOMETRY; whole-value reuse additionally requires
    // the naming key (names embed node ids the content key excludes).
    // On a content hit with a naming miss the op RE-RUNS whole: the
    // emission handoffs are dropped after naming, so the naming half
    // is not separably re-derivable — and D9 makes the re-run's
    // geometry bit-identical, so re-running IS "reuse the geometry,
    // re-derive the names", spelled honestly.
    //
    // `prior` is an evaluation of THIS document: `evaluate_at_descent`
    // drops a foreign one before the schedule is built (DI3), so this
    // lookup cannot serve a coincidental id collision from another
    // document.
    if let Some(v) = prior.and_then(|p| p.value(id))
        && v.content_key == content_key
        && v.naming_key == naming_key
    {
        // The profile's f64 precompute ran inside this frame, and the
        // reused value's log opens with the same decisions. A
        // `Verdict` is (predicate, sign), and the inputs the content
        // key fixes — the resolved program, the placement the frame's
        // own key fixes (`NodeValue::placement`), the tolerance —
        // are exactly what the precompute decides from, so D9 makes
        // the two sequences equal. It holds at EVERY scalar, and
        // `tag::slot` is why: the frame's key fixes its slots at the
        // nominal as well as at the lane, and the placement is a pure
        // function of the nominal ones.
        // The reused value IS the record, so the fresh frame is
        // finished and dropped rather than spliced in, and the prefix
        // identity is asserted in every profile: one compare of a few
        // dozen verdicts per hit, beside a precompute that just
        // replayed and validated the profile.
        let fresh = bracket.finish();
        assert!(
            v.verdicts.starts_with(&fresh.verdicts)
                && v.escalations.starts_with(&fresh.escalations),
            "a memo hit's pre-key decisions differ from the reused log's prefix"
        );
        return NodeStep {
            result: NodeResult::Ok(v.clone()),
            reused: true,
        };
    }

    // The op's decisions — kernel predicates and N2 discriminators
    // alike, definite or escalated — follow the pre-key ones in the
    // frame opened above, which closes once the op returns.
    let op = wire::run_op(
        id,
        node,
        doc,
        results,
        &slot_values,
        payload.as_ref(),
        profile_pre.as_ref(),
        op_env,
        tol,
    );
    let recorded = bracket.finish();
    let escalations = Arc::new(recorded.escalations);
    match op {
        Ok(out) => NodeStep {
            result: NodeResult::Ok(NodeValue {
                payload: out.payload,
                outputs: out.outputs.into(),
                contacts: out.contacts,
                carried: out.carried,
                coincidences: out.coincidences,
                cited_inputs: out.cited_inputs,
                parts: out.parts,
                motion: out.motion,
                space: match node {
                    // A measure and an assertion read geometry and
                    // answer a number, which lives in no space.
                    crate::node::Node::Measure { .. } | crate::node::Node::Assertion { .. } => None,
                    _ => out
                        .space
                        .or(inputs_space)
                        .or(Some(crate::mate::Space::World)),
                },
                verdicts: Arc::new(recorded.verdicts),
                escalations,
                witness: WitnessSlot {},
                content_key,
                naming_key,
            }),
            reused: false,
        },
        // The failure carries what the op escalated on its way to it:
        // the frame is the node's whether or not the op built.
        Err(kind) => NodeStep {
            result: NodeResult::Failed(NodeError {
                node: id,
                kind,
                escalations,
            }),
            reused: false,
        },
    }
}

/// **The content key's structural tags, declared by vocabulary.**
///
/// A content key is a stream of `u64` words, and the memo serves an
/// entry to a node exactly when the two streams agree. A tag separates
/// alternatives only at the grammar position where it is read, so what
/// keeps a memo from serving another node's geometry is injectivity
/// WITHIN each vocabulary read at one position — never one number
/// space for the whole key. Node tag `5` (an extrude) and target tag
/// `5` (a point) are different words of different vocabularies; `43`
/// is retired in the target vocabulary and live in the blend payload's.
///
/// **The rule, stated once:** within a vocabulary an existing number
/// never changes meaning and a retired number is never reused; across
/// vocabularies numbers are unrelated. Keys are process-internal and
/// never persisted, so a number costs a memo invalidation and no
/// schema — the rule guards against a collision inside one run, never
/// a stored value.
///
/// Every vocabulary has one of three declared shapes, each with a
/// census that iterates its whole row set (`tag_vocabulary_tests`):
///
/// - a group below — named consts and an `ALL`, one per grammar
///   position, both projected from one `tag_groups!` declaration so
///   neither can fall behind the other, censused together over
///   `GROUPS`;
/// - a function of a closed enum — [`verb_content_tag`] over
///   `VerbKind::ALL`, [`verb_tag`] over `profile::Verb::ALL`,
///   [`arc_mode_tag`] over `ArcMode::ALL`, [`seg_content_tag`] over
///   `SegTag::ALL`,
///   `ContactClass::content_tag` over `ContactClass::ALL`, and
///   [`winding_tag`], [`side_tag`], [`target_tag`] over a local closed
///   list that an exhaustive match forces to name every variant (their
///   enums are `crates/profile`'s and carry no `ALL`) — each with its
///   own injectivity row;
/// - the node-kind match in [`content_key`], a function of `Node` with
///   no payload-free mirror to iterate, censused by source text between
///   its `NODE-KIND-VOCABULARY` sentinels
///   (`node_kind_vocabulary_is_injective`), and covering that match
///   alone.
///
/// **Disposition of the inline matches.** A few small vocabularies are
/// written as an exhaustive `match` that maps an enum's variants to
/// literals at the site — a constraint's kind, a value's coordinate and
/// a world read's arm (`tag::constraint_kind`, `tag::value_coord`,
/// `tag::world_read`), a name's entity kind, a side verdict, a qualifier, a cap end, a meridian end and a
/// rim support (`names/role.rs`), a tube's window (`node.rs`), an
/// assertion's relation and a measure primitive (`measure.rs`), a
/// dimension (`expr.rs`). Each is total by exhaustiveness — a variant
/// the enum gains breaks the compile at the site — and its two-to-four
/// arms are read at one position under a word this file does census,
/// so a collision would have to be typed into the arms themselves. They
/// stay matches because their enums are DOCM's, M10's or unowned and
/// carry no `ALL` to iterate; the day one gains an `ALL`, its map
/// becomes a function with a row here, as `content_tag` has.
mod tag {
    /// Declares each group's words and its retired numbers, and
    /// projects the group's `ALL`, its `RETIRED` and the census roster
    /// `GROUPS` from the same declaration, so a word declared is a word
    /// censused and a number retired is a number no group can reclaim.
    macro_rules! tag_groups {
        ($( $(#[$gm:meta])* $group:ident $(retires [ $($ret:literal),* $(,)? ])? {
            $( $(#[$cm:meta])* $name:ident = $val:literal ),* $(,)?
        } )*) => {
            $(
                $(#[$gm])*
                pub(crate) mod $group {
                    $( $(#[$cm])* pub(crate) const $name: u8 = $val; )*
                    /// Every word of this group, in declaration order —
                    /// the row set its census iterates.
                    #[allow(dead_code, reason = "the census's row set, read by the test module")]
                    pub(crate) const ALL: &[u8] = &[$( $name ),*];
                    /// Every number this group has RETIRED: written by
                    /// some earlier version of the key with a meaning
                    /// this group no longer has, and therefore dead
                    /// here for good. Declared beside the live words so
                    /// the census reads both from one place.
                    #[allow(dead_code, reason = "the census's row set, read by the test module")]
                    pub(crate) const RETIRED: &[u8] = &[$( $( $ret ),* )?];
                }
            )*
            /// Every group, by name, with its live and its retired
            /// words, for the census — projected from the declarations,
            /// so a group declared is a group censused.
            #[allow(dead_code, reason = "the census's roster, read by the test module")]
            pub(crate) const GROUPS: &[(&str, &[u8], &[u8])] =
                &[$( (stringify!($group), $group::ALL, $group::RETIRED) ),*];
        };
    }

    tag_groups! {
        /// The first word of every content key: the key format
        /// version. It bumps whenever an existing node's stream would
        /// otherwise keep its bytes while its meaning moved — a channel
        /// an existing node kind writes into, or a change that could
        /// let two different nodes share a stream — so no memo entry
        /// from a pre-bump process is reused by a post-bump one. A
        /// channel no existing node reaches is additive and exempt.
        /// Keys are process-internal and never persisted, so a bump
        /// costs one whole-memo invalidation and no migration.
        format {
            /// v15: an extrude no longer writes a side, and a tube
            /// reads no frame. (v14: a mate writes its class and its target's word, a
            /// world read its arms and roles; the solved pose and the
            /// parts a face frame resolves against are no longer keyed,
            /// the fold being a function of the keyed reads. v13: a
            /// pose read writes the role it reads through, a node's
            /// read of an output through one included. v12: a
            /// blend, a shell, a face frame and a measure write
            /// the names of the selections they read, the measure each
            /// reference's names (none for a whole body) where it wrote
            /// its sites' places. v11: a measure writes one primitive
            /// and the place of each reference's site among its
            /// upstream keys; an assertion's payload is its value and
            /// its bound. v10:
            /// each read feeds the port it reads beside its operation's
            /// key (D10), one entry per read rather than one per
            /// operation. v9: an extrude writes its side. v8: a mate
            /// frame writes its arm word before its payload, and a mate
            /// writes the parts its face frames resolve against.)
            VERSION = 15,
        }
        /// The first word of every naming key: the naming-key domain,
        /// which keeps a naming key's stream apart from a content key's.
        naming {
            DOMAIN = 3,
        }
        /// The word before an optional datum — a solved placement, a
        /// witness, an alignment length, a clocking angle, a profile's
        /// resolved program — so that a `None` cannot alias a `Some`
        /// whose payload happens to be empty (a witness with no bytes,
        /// a loop list of length zero): the payload's presence is a
        /// word of its own, read before the payload.
        presence {
            ABSENT = 0,
            PRESENT = 1,
        }
        /// **The words of the pose definitions a node reads** (D10),
        /// written after its payload expressions and only by a node
        /// reading one, so no other node's key moves. Per such operand:
        /// `READ` and the operand's place among the node's reads, the
        /// role it reads through ([`pose_role`]), then the read's tree
        /// — `OUTPUT` for an operation's output (whose key and port the
        /// upstream list carries), `SELECTION` and its names, or
        /// `DEFINITION`, the arm ([`pose_arm`]), each read's role and
        /// tree in field order, then each scalar's value and its
        /// nominal under [`slot`]'s `NOMINAL`.
        pose {
            READ = 1,
            OUTPUT = 2,
            SELECTION = 3,
            DEFINITION = 4,
        }
        /// A pose definition's arm ([`crate::pose::PoseDef`]).
        pose_arm retires [4, 8, 9] {
            PLANE = 1,
            AXIS = 2,
            POINT = 3,
            THROUGH = 5,
            MEET = 6,
            FLIP = 7,
            CARRIED = 10,
            SLIDE = 11,
            SPIN = 12,
        }
        /// The role a pose read reads through
        /// ([`crate::pose::PoseRole`]), a plain read's included.
        pose_role {
            PLAIN = 1,
            X = 2,
            Y = 3,
            Z = 4,
            ORIGIN = 5,
            XY = 6,
            YZ = 7,
            ZX = 8,
            NORMAL = 9,
            DIRECTION = 10,
        }
        /// A union's or an intersect's argument form: a family read
        /// whole, or reads spelled.
        bodies {
            FAMILY = 1,
            SPELLED = 2,
        }
        /// The word before a node's indexed reads ([`crate::BodyRead`]),
        /// written only by a node holding one, so a node of plain reads
        /// keeps its stream; then each indexed read's place among the
        /// node's body seats and its number of indices. The index
        /// values are slots, fed with the slot values.
        index {
            READS = 1,
        }
        /// The word before a node's keyed reads ([`crate::BodyRead::key`]),
        /// written only by a node holding one; then each keyed read's
        /// place among the node's body seats and its key, segment by
        /// segment: a part's placement, then the read that ends it.
        copy_key {
            READS = 1,
            PART = 2,
            READ = 3,
        }
        /// What a constraint states: a mate or a value
        /// ([`crate::mate::ConstraintKind`]).
        constraint_kind {
            MATE = 1,
            VALUE = 2,
            // 3 was the retired world pin's.
        }
        /// Which freedom a value sets ([`crate::mate::Coord`]).
        value_coord {
            SLIDE_X = 1,
            SLIDE_Y = 2,
            SLIDE_Z = 3,
            SPIN = 4,
        }
        /// What a mate's `to` reads: a pose variable of the target
        /// space, or the world read inside the mate.
        mate_target {
            POSE = 1,
            WORLD = 2,
        }
        /// A world read's arms, in pre-order ([`crate::WorldRead`]); a
        /// role arm writes its role word after its own.
        world_read {
            FRAME = 1,
            ROLE = 2,
            SLIDE = 3,
            SPIN = 4,
            FLIP = 5,
        }
        /// **The word before a slot's NOMINAL — the one home of why a
        /// key holds one, and of every exception.** It is written
        /// after that slot's bits at the evaluation scalar, and it
        /// carries the slot's expression evaluated at the evaluation's
        /// nominal environment — [`super::wire::LaneEnv::nominal`],
        /// whose doc says what that environment is.
        ///
        /// **Why the key owes it.** The nominal's readers are listed
        /// at [`super::wire::LaneEnv::nominal`], and each decides what
        /// a lane value IS: the frame read is what the pinned lift
        /// embeds whole into the lane profile's placement, and the
        /// program resolutions are what the key already holds as a
        /// stream. A key that fixed only the lane bits would leave the
        /// frame read's input unfixed, and the memo would serve a
        /// profile placed on another nominal's plane.
        ///
        /// **What the word means per lane.** At f64 under no box, or
        /// under a box whose axes are all zero-width and zero-offset,
        /// it repeats the lane bits; under a degenerate box at a
        /// NON-zero offset — the one f64 boxes admit — the lane value
        /// is `nominal + offset` and this word is the only thing that
        /// keys the nominal apart. At `Dual64` it repeats the value
        /// half, the seed riding the tangent. At `Interval` it is the
        /// input the bounds do not determine at all: a nominal edit
        /// under a compensating box moves the nominal while leaving
        /// `(lo, hi, dec)` equal.
        ///
        /// **The exceptions, all of them.**
        ///
        /// * A COUNT slot writes no nominal word. [`crate::expr::eval_count`]
        ///   reads the document's exact `Count` binding at every
        ///   scalar and [`crate::analysis::var_env_over`] widens
        ///   only `Continuous` parameters, so a count's lane word IS
        ///   its nominal.
        /// * A `Profile` node has no slots here at all — its program
        ///   expressions resolve in their own stage
        ///   ([`super::slots::eval_slots`] exempts the node) — and
        ///   needs none: the resolved stream it feeds is resolved in
        ///   this same environment, so the nominal is already in its
        ///   key.
        /// * The measurement vocabulary's PAYLOAD expressions
        ///   (`Measure`, `Assertion`) write no nominal word. They are
        ///   read only by their own node's op at the evaluation
        ///   scalar, so no f64-pinned reader has an input for the key
        ///   to owe. The day a pre-pass reads a measured bound at
        ///   f64, they join this rule.
        ///
        /// **There is no word for a refused nominal.** A slot whose
        /// expression does not evaluate at the nominal refuses the
        /// NODE, typed, at the same door its lane refusal goes
        /// through — no reader of that nominal could have read it
        /// either, so a value keyed on "the nominal is unreadable"
        /// would be a value no reader can produce.
        slot {
            NOMINAL = 1,
        }
        /// The words that structure a profile node's program payload,
        /// read where a loop or a stream may end. **Every LOOP list
        /// and every STEP list is length-prefixed**: the resolved
        /// stream is its presence word ([`presence`]) and, when
        /// present, the loop count, then per loop `LOOP_START`, the
        /// loop's step count and its steps; the lane stream, when the
        /// lift's second pass ran, is `LANE`, the loop count, then per
        /// loop `LOOP_START`, the loop's scalar count and its scalars,
        /// each under `LANE_SCALAR`; then, per authored radius of every
        /// loop in program order while the profile edge's radius is
        /// flow-bearing, `CARRIER_RADIUS` and the radius expression.
        ///
        /// The counts are what make a loop boundary a SINGLE
        /// vocabulary: a reader that has consumed a loop's declared
        /// step count is at a loop boundary or at the end of the list
        /// it counted, never at a word it must tell apart from the
        /// verb vocabulary ([`super::verb_tag`]). Two words of this
        /// group collide with that one — `LANE` and the `Cusp` verb
        /// are both `41`, `LANE_SCALAR` and the `ContinueTo` verb are
        /// both `42` — which is the class the counts cover, and it has
        /// two members rather than the one it is easy to see. It is
        /// single-vocabulary a second way too, independently of the
        /// counts: a `Cusp` cannot end a loop, its tip being
        /// directed-incoming with no closing verb accepting it, so the
        /// word after a `Cusp` is always a verb tag and never
        /// `LOOP_START`.
        ///
        /// **The carrier-radius entries are a list with no count**, and
        /// they are the exception the sentence above is worded around.
        /// Each is `CARRIER_RADIUS` and then a lowered expression whose
        /// own bytes are length-prefixed
        /// ([`crate::param_source::feed_content_key`]), so a reader
        /// finishing one entry is at the next `CARRIER_RADIUS` or at
        /// the end of the payload — the same END position the optional
        /// `LANE` section stops at, where the next word is the
        /// upstream-key count, `1` for a profile since its plane is its
        /// one input, and neither `41` nor `45`.
        ///
        /// `0` is RETIRED here: it was this group's absent-program
        /// marker, and the resolved stream's presence is
        /// [`presence`]'s word now, so a loop count of `0` cannot
        /// alias it.
        program retires [0] {
            LOOP_START = 1,
            LANE = 41,
            LANE_SCALAR = 42,
            CARRIER_RADIUS = 45,
        }
        /// The word before one payload token inside a resolved step: a
        /// continuous argument's f64 bits under `FLOAT`, a structural
        /// count under `COUNT` — (tag, payload) throughout, so
        /// structure can never alias float data.
        step {
            FLOAT = 2,
            COUNT = 3,
        }
        /// A step's target, where a verb carries one: the entry
        /// vertex, an authored point (followed by its coordinates), or
        /// the entry vertex with the seam's tangent joint declared.
        /// `43` held a retired second declaration and stays dead in
        /// this vocabulary (its `retired` list).
        target retires [43] {
            START = 4,
            POINT = 5,
            START_ARRIVING = 44,
        }
        /// An arc's winding, where an arc mode carries one; read
        /// through [`super::winding_tag`].
        winding {
            CCW = 6,
            CW = 7,
        }
        /// An arc's side, where an arc mode carries one; read through
        /// [`super::side_tag`].
        side {
            LEFT = 36,
            RIGHT = 37,
        }
        /// What may follow a one-scalar verb's name list (a blend's
        /// canonical selection, a shell's open faces): the lowered
        /// expression of its flow-bearing size slot, or nothing. One
        /// word for every verb `feed_scalar_join` feeds.
        scalar_join {
            FLOW_EXPR = 43,
        }
    }
}

/// **The content-key tag of a MIGRATED verb** — the memoization
/// commitment, held where memoization lives, matching on the kernel's
/// own name for the verb.
///
/// The kernel says nothing about content keys; this crate says nothing
/// about what a verb IS. What connects them is one exhaustive match
/// over [`verbs::VerbKind`], so a variant added to the vocabulary
/// breaks this file at compile rather than defaulting to a tag that
/// already means something else.
///
/// **`Option`, because a verb in the vocabulary need not be a verb the
/// document can author.** A content key is a function of a `Node`, so a
/// kernel-only verb has no tag — and the honest shape for that is a
/// declared `None` rather than a missing arm, which is what lets the
/// tag censuses stay exhaustive over the vocabulary while measuring
/// only the rows that are really in the node-kind vocabulary.
///
/// **The numbers are the ones that were already here** and they do not
/// move: they are the tags [`content_key`]'s match wrote inline before
/// the vocabulary had a home, and `verb_content_tags_are_the_committed
/// _numbers` pins each of them. Keys are process-internal and never
/// persist, so a tag costs a memo invalidation and no schema — but an
/// EXISTING tag must never be reused for a new meaning, which is the
/// rule every vocabulary in the key runs on ([`tag`]).
///
/// It takes the payload-free [`verbs::VerbKind`] rather than a
/// `&verbs::Verb<T>` because a content key is computed BEFORE the node's
/// selection has resolved to arena keys or its slot to a scalar: at
/// this point in evaluation there is no verb value to match on, only
/// the verb's name — which is exactly what the tag is a function of.
/// The boolean's NAME carries its op (`VerbKind::Boolean(op)`): union,
/// intersect and subtract are three operations sharing one payload
/// shape, and the node-kind vocabulary has kept them apart since v1, so the
/// three rows below are three names — not payload leaking into the
/// tag, and not a new structural word in the key (the op feeds nothing
/// elsewhere, exactly as before).
fn verb_content_tag(kind: verbs::VerbKind) -> Option<u8> {
    match kind {
        verbs::VerbKind::Fillet => Some(17),
        verbs::VerbKind::Chamfer => Some(24),
        verbs::VerbKind::Extrude => Some(5),
        verbs::VerbKind::Revolve => Some(6),
        verbs::VerbKind::Boolean(topo::BooleanOp::Union) => Some(8),
        verbs::VerbKind::Boolean(topo::BooleanOp::Intersect) => Some(9),
        verbs::VerbKind::Boolean(topo::BooleanOp::Subtract) => Some(10),
        verbs::VerbKind::Split => Some(7),
        // Appended, never a reused tag: the shell took the next free
        // number after the projection node's pair (33/34) when its
        // document node landed. The `Option` stays although every verb
        // now declares a number: "no tag" remains an answer the
        // vocabulary can give for a kernel-only verb, and the censuses
        // below stay total over `VerbKind::ALL` by reading this row
        // rather than a list of their own.
        verbs::VerbKind::Shell => Some(35),
    }
}

/// **[`verb_content_tag`] for a verb a `Node` really builds.**
///
/// Every call site is an arm of the node match below, and each names a
/// verb the document layer authors — all of which declare a tag. The
/// answer this cannot use is the kernel-only one, which is a WIRING
/// mistake rather than a value: a kernel-only verb given a node arm
/// must be loud here rather than keyed under a fallback number,
/// because a silently wrong content tag is how a memo serves another
/// node's geometry. An existing tag never gains a meaning and a
/// retired one is never reused — the rule every vocabulary in the key
/// runs on.
// The narrow `expect`: the alternative is a fallback tag, which is the
// one failure this space cannot survive.
#[allow(clippy::expect_used)]
fn document_verb_tag(kind: verbs::VerbKind) -> u8 {
    verb_content_tag(kind).expect("a verb a Node builds declares a content tag")
}

/// The content key (spec D4): op kind, structural params, evaluated
/// expression values AS BITS **and each of them again at the
/// document's nominal f64** (the rule at [`tag::slot`]), upstream keys
/// — plus the ambient
/// tolerance (ε, k), which parameterizes every decision the kernel
/// ops make, and a leading format version. M4 PR 4: the node's
/// recorded witness datum (if any) is an input too — `solution
/// (constraints, params, witness)` is pure in exactly those three
/// (W5), so a witness change must move the key (v1 evaluation does
/// not read the witness, so the recompute reproduces identical
/// results — W4's "semantically invisible", honestly re-derived
/// rather than assumed).
// The arguments past the node are all INPUTS to the key, which is the
// one thing a content key is allowed to grow: the environment every
// same slots at the document's nominal, the lift's lane-resolved
// program, and an assertion's payload-expression values — the same
// input the slot values are, arriving by a second route because an
// assertion's value and bound are not slots.
#[allow(clippy::too_many_arguments)]
fn content_key<'d, T>(
    node: &crate::node::Node<ProfileProgram>,
    defs: crate::param_source::Definitions<'_, '_>,
    selections: &dyn Fn(crate::VarId) -> Option<&'d crate::var::Select>,
    slot_values: &slots::SlotValues<T>,
    nominal_values: &slots::SlotValues<f64>,
    payload_values: Option<&Payload<T>>,
    pose_words: Option<&[wire::pose::PoseWord<T>]>,
    resolved_program: Option<&[Vec<profile::Step<f64>>]>,
    lane_program: Option<&[Vec<profile::Step<T>>]>,
    upstream_keys: &[UpstreamRead<ContentKey>],
    witness: Option<&crate::witness::WitnessDatum>,
    tol: Tol,
) -> ContentKey
where
    T: Decide + ContentBits,
{
    use crate::node::{Node, PatternKind};
    let mut h = KeyHasher::new();
    // The format version first — the rule for bumping it is at
    // `tag::format::VERSION`. It is written before the node
    // discriminant, so a bump moves every key in the document, which is
    // what a format version is for.
    h.write_tag(tag::format::VERSION);
    let tol = tol.get();
    h.write_f64_bits(tol.eps);
    h.write_f64_bits(tol.k);
    // NODE-KIND-VOCABULARY BEGIN — the sentinels
    // `node_kind_vocabulary_is_injective` reads. Every number between here and
    // the END sentinel is a word of the node-kind vocabulary, whether it
    // is written inline or comes back from `verb_content_tag`, and that
    // census covers this match and nothing else: every other vocabulary
    // in the key is declared in `tag` or as an enum function with its
    // own census (`tag`'s doc), so a tag written outside these lines is
    // a different vocabulary, not a gap. Do not move a node-kind tag out
    // of these lines without teaching that test where it went.
    let kind = match node {
        // 1, 2 and 3 are retired (`RETIRED_NODE_KIND_TAGS`).
        Node::Profile(_) => 4,
        // The numbers are not written here either, and they are the
        // ones that were: a migrated verb's tag is a function of the
        // KERNEL's name for it, so 5, 6 and 7 move to
        // `verb_content_tag` unchanged. A tag that MOVED would
        // invalidate nothing on disk (keys are process-internal) and
        // would still be wrong — an existing tag never gains a new
        // meaning, and never loses its old one either.
        Node::Extrude { .. } => document_verb_tag(verbs::VerbKind::Extrude),
        Node::Revolve { .. } => document_verb_tag(verbs::VerbKind::Revolve),
        Node::Split { .. } => document_verb_tag(verbs::VerbKind::Split),
        // The number is not written here: a migrated verb's tag is a
        // function of the KERNEL's name for it, the regularized
        // difference (`VerbKind::Boolean(Subtract)`).
        Node::Subtract { .. } => {
            document_verb_tag(verbs::VerbKind::Boolean(topo::BooleanOp::Subtract))
        }
        // 11 is retired (`RETIRED_NODE_KIND_TAGS`).
        Node::Pattern { kind, .. } => match kind {
            PatternKind::Linear { .. } => 12,
            PatternKind::Circular { .. } => 13,
            // A fresh word of the node-kind vocabulary: an EXISTING tag
            // never gains a new meaning.
            PatternKind::Explicit(_) => 19,
        },
        // 14 is retired (`RETIRED_NODE_KIND_TAGS`).
        // New node kinds take fresh words — keys are process-internal
        // (never persisted), so growth is free, but an EXISTING tag
        // must never be reused for a new meaning.
        Node::Loft { .. } => 15,
        Node::Sweep { .. } => 16,
        // The number is not written here: a migrated verb's tag is a
        // function of the KERNEL's name for it, so it comes out of
        // `verb_content_tag`.
        Node::Fillet { .. } => document_verb_tag(verbs::VerbKind::Fillet),
        Node::InstantiatePart { .. } => 18,
        // The group boolean: one tag per placement rule, so a rule
        // change moves the key even when every slot value holds.
        Node::PlacedUnion { kind, .. } => match kind {
            PatternKind::Linear { .. } => 20,
            PatternKind::Circular { .. } => 21,
            PatternKind::Explicit(_) => 22,
        },
        // A chamfer and a fillet of the same size on the same edges are
        // different geometry, so they must not share a key. Same home
        // as the fillet's, for the same reason.
        Node::Chamfer { .. } => document_verb_tag(verbs::VerbKind::Chamfer),
        // Fresh words — an existing one must never be reused for a
        // new meaning, and 24 is the chamfer's.
        Node::Measure { .. } => 25,
        Node::Assertion { .. } => 26,
        // 27 is retired (`RETIRED_NODE_KIND_TAGS`).
        // Two tags, not one: a solid tube and a hollow tube of the same
        // radii are different artifacts, and sharing a tag would let
        // one's memo serve the other.
        Node::Tube { .. } => 28,
        Node::HollowTube { .. } => 29,
        // 30 was the in-plane axis datum's, which a revolve's own
        // axis line replaced: a retired tag is never reused.
        // The union's tag, and the intersect's fresh word. The member
        // list itself is not written here — members are input EDGES,
        // and the inputs' own keys carry them in list order below,
        // which is the rule `Loft`'s profiles already run on; whether
        // the argument is a family or spelled is payload, fed below.
        Node::Union { .. } => 31,
        Node::Intersect { .. } => 38,
        // 32 is retired (`RETIRED_NODE_KIND_TAGS`).
        // The projection node. 33 is retired with the split-half
        // selection it tagged, and not reused.
        Node::Part { .. } => 34,
        // The shell's tag is the seat's, read off the verb vocabulary
        // as the blends' are. A shell and its target of the same slot
        // values are different bodies, so they must not share a key. A
        // new node kind is additive — it writes into no channel an
        // existing node kind reaches — so the format version does not
        // bump for it.
        Node::Shell { .. } => document_verb_tag(verbs::VerbKind::Shell),
        // Fresh words: a placement's key is its shapes' upstream keys,
        // its bundle and the pose its bundle solved to; the world's is
        // its tag alone.
        Node::Place { .. } => 40,
        Node::World => 39,
    };
    // NODE-KIND-VOCABULARY END
    h.write_tag(kind);
    // Structural payloads beyond the tag — everything a node carries
    // that its SLOTS do not express, and that two nodes of one tag can
    // differ in. The match is EXHAUSTIVE on purpose: a future variant
    // whose recipe payload lives outside its slots must be fed here or
    // the compile breaks. It cannot default to "tag plus slots" and
    // hash identically to a node that differs in that payload — a memo
    // hit would then serve another node's geometry, which is not
    // hypothetical (see S4: two steps once shared a content-key tag,
    // and a reviewer caught it rather than a type).
    // Every arm NAMES its variant's fields, with `_` for each one that
    // is a slot (fed below) or an input edge (carried by the upstream
    // keys), and no arm writes `{ .. }`: a rest pattern would let a
    // field added to a variant compile unfed, which is how an extrude's
    // side once stayed out of this key while the match still read as
    // exhaustive.
    // The tag match above is exhaustive for the same reason; the two
    // halves of one key had different answers to that until now.
    match node {
        Node::Profile(program) => {
            // LIB-SWITCH §4e: the program's structural payload feeds
            // as (tag, payload) tokens — per loop a LoopStart tag and
            // per RESOLVED step the verb
            // tag + structural tags + the resolved-at-f64 bit pattern
            // of each continuous arg (the same resolved-value
            // convention node slots use). Derived segment floats LEFT
            // the key (V3); display units never enter it (D7). Any
            // edit that can change segments changes the key: structure
            // via tags, Exprs and params via resolved bits, ε above.
            // The plane's twelve placement floats LEFT this key when
            // the plane became a node: it is an input now, so its own
            // content key is already folded in above with every other
            // upstream key, and writing it here too would be the same
            // fact hashed twice — with the two copies free to disagree
            // the day a frame's own key changes shape.
            // Present by eval_node's stage order (profiles resolve
            // before keying); written defensively — no panic paths in
            // this crate — and the presence word keeps an (impossible)
            // absent-program key distinct from any real program's key
            // rather than aliasing an empty loop list, which the loop
            // count alone could not do. The structural words are the
            // profile-payload vocabulary's (`tag::program`), where the
            // length-prefixing rule is stated.
            match resolved_program {
                Some(resolved) => {
                    h.write_tag(tag::presence::PRESENT);
                    h.write_u64(resolved.len() as u64);
                    for steps in resolved {
                        h.write_tag(tag::program::LOOP_START);
                        h.write_u64(steps.len() as u64);
                        for step in steps {
                            feed_step(&mut h, step);
                        }
                    }
                }
                None => h.write_tag(tag::presence::ABSENT),
            }
            // The steps' minted ids: what every name a sweep over this
            // profile publishes spells (N1), so two programs that
            // replay alike but carry different ids — one step
            // re-authored in place of another — are two keys, and a
            // memo hit cannot serve one's names for the other's.
            h.write_u64(program.ids.len() as u64);
            for ids in &program.ids {
                h.write_u64(ids.len() as u64);
                for id in ids {
                    h.write_id(id.0);
                }
            }
            // The f64 stream above IS the structure identity and stays
            // in the key unconditionally, lane-independent as ever.
            // What follows is the lane's own geometry, and only when
            // the second pass computed any: `LANE` opens it, so a
            // pinned evaluation's key is that one word away from what
            // it has always been and cannot alias a lifted one's.
            if let Some(lane) = lane_program {
                h.write_tag(tag::program::LANE);
                h.write_u64(lane.len() as u64);
                for steps in lane {
                    h.write_tag(tag::program::LOOP_START);
                    h.write_u64(steps.len() as u64);
                    for step in steps {
                        feed_lane_step(&mut h, step);
                    }
                }
            }
            // A profile edge's RADIUS EXPRESSION, when a migrated verb
            // declares that operand-carried scalar into a stored field
            // (SEAT-7, key format v5). The stream above carries the
            // radius's VALUE, at f64 bits, which is what the geometry
            // is a function of; what it cannot carry is the spelling,
            // and the spelling is now an input to the BODY a downstream
            // sweep mints — the wall's field source is the lowered
            // expression. So a value-preserving re-spelling (`r` for
            // `0.125`) must move this key, or the sweep's memo would
            // serve a body whose token names an expression the document
            // no longer holds. Exactly the blend's rule (`feed_scalar_join`,
            // v4) at the node that HOLDS the expression rather than the
            // node that attaches it: the sweep's key folds this one in
            // as an upstream key already, so writing it here covers
            // every consumer at once.
            //
            // The rule is read off the declaration, never per node: the
            // moment no verb declares the profile edge's radius into a
            // field, nothing is written and the keys are the v4 ones.
            //
            // **What is fed is the PROGRAM's answer, and the attach's
            // is a subset of it.** This feed has no record of the
            // evaluation in hand — it runs before one exists — so it
            // asks `LoopProgram::step_radii`, which reads the program
            // alone and answers EVERY radius argument it authors. The
            // attach asks `ProfileProgram::segment_radii`, which
            // additionally reads the replay's record of which segment
            // each of those radii drew, and answers only where an arc
            // was drawn. That inclusion is the whole
            // guard, and it is the direction that cannot go stale: a
            // spelling can be keyed and not attached, which costs a
            // memo hit and nothing else, and cannot be attached without
            // having been keyed, which is what would serve a wall whose
            // token names an expression the document no longer holds.
            // It stays true of a THIRD per-edge scalar someone adds
            // later only while that scalar's two doors keep the same
            // relation, which is why each says so at its own end.
            //
            // **How wide this is, stated rather than implied.** Three
            // separate breadths, none of which moves a VALUE — keys are
            // process-internal and never persisted (spec D3), so what
            // widens here is memo hit rate and nothing else.
            //
            // 1. The predicate is GLOBAL. `operand_flow_bearing` asks
            //    the whole vocabulary, not this document, so the word is
            //    written for every profile that authors a radius
            //    anywhere in every document — one that no sweep ever
            //    consumes included.
            // 2. Keys FOLD upstream keys, so a radius
            //    re-spelled invalidates the whole downstream subtree,
            //    not only its sweeps: a loft, a section or a boolean
            //    over that profile re-runs too, and none of them
            //    attaches anything.
            // 3. The format-version bump moves every key in the
            //    document, sweeps or not, once.
            //
            // A narrower guard is constructible — per document (does
            // any node consume this profile through a sweep?) or per
            // consumer (write the tag at the sweep instead of the
            // profile) — and is not worth its cost. Per consumer is the
            // one that would fix (2), and `content_key` has no document
            // handle with which to reach a sweep's operand payload, so
            // it means threading one through the whole key surface for
            // a hit-rate gain on a re-spelling edit, which is rare by
            // construction (a value-preserving edit is a deliberate
            // rewrite, not a drag). Per document costs a reachability
            // walk per profile node per key. Both trade a real
            // invariant — the rule read off the declaration alone — for
            // an unmeasured saving.
            if crate::param_source::operand_flow_bearing(verbs::FlowSource::ProfileEdge(
                verbs::EdgeScalar::Radius,
            )) {
                for lp in &program.loops {
                    // Every radius the loop authors, in program-step
                    // order: a carrier form's one, a chain's per
                    // radius-bearing ARGUMENT — a fused step's two or
                    // three all enter. The loop shapes are not
                    // distinguished here
                    // because the question is not per loop — it is
                    // "which spellings of this program can reach a
                    // stored field", and a chain's arc radii reach the
                    // walls its arcs sweep exactly as a carrier's does.
                    for (_, &var) in lp.step_radii() {
                        // Opened by its word in the profile-payload
                        // vocabulary (`tag::program`).
                        h.write_tag(tag::program::CARRIER_RADIUS);
                        crate::param_source::feed_var(&mut h, defs, var);
                    }
                }
            }
        }
        // ASM-2A D-1/D-2: WHICH document (id + pin — the pin IS the
        // referenced content, so nothing about the part needs hashing
        // here) and WHERE it sits. Where it sits is read off other
        // nodes — its constraints' targets, the parameters its
        // constraints read — which is exactly why it must feed the key:
        // a parameter driving a target moves this node's value and
        // nothing else about the node changes.
        // The INTERFACE RECORD feeds the
        // key too (ASM-R2b D-4 discharging ASM-4's hook obligation):
        // it is inhabited now, it is on-wire data, and evaluation
        // re-verifies the crossings it holds — so a crossing edit
        // must move this node's value rather than hit the memo on the
        // pre-edit answer.
        Node::InstantiatePart {
            doc_ref,
            interface,
            constraints,
        } => {
            feed_doc_ref(&mut h, doc_ref);
            // Its bundle: the fold is a function of what this feeds and
            // of the reads its upstream keys carry, so a constraint
            // edit, or a parameter a pose reads, moves the key.
            feed_bundle(&mut h, constraints);
            h.write_u64(interface.crossings.len() as u64);
            for crossing in &interface.crossings {
                let crate::node::InterfaceCrossing::Mate {
                    class,
                    outer,
                    inner,
                } = crossing;
                h.write_u64(class.content_tag());
                feed_stable_name(&mut h, outer);
                feed_stable_name(&mut h, inner);
            }
        }
        // A placement's shapes are input edges; their count and its
        // bundle feed here.
        Node::Place {
            shapes,
            constraints,
        } => {
            h.write_u64(shapes.len() as u64);
            feed_bundle(&mut h, constraints);
        }
        Node::World => {}
        // The operands and the members are input edges.
        Node::Subtract { from: _, tool: _ } => {}
        Node::Union { members } | Node::Intersect { members } => {
            h.write_tag(match members {
                crate::Bodies::Family(_) => tag::bodies::FAMILY,
                crate::Bodies::Spelled(_) => tag::bodies::SPELLED,
            });
        }
        // LIB-PLACEDUNION: an `Explicit` rule's FRAMES are recipe
        // payload, not slots (the list is the count, D8-structural),
        // so they must feed the key by hand or an edited placement
        // would recompute nothing. Bits, in placement order (D9) —
        // `0.0` and `-0.0` are different placements to this key,
        // exactly as they are to `bit_eq`.
        Node::Pattern {
            input: _,
            count: _,
            kind,
        }
        | Node::PlacedUnion {
            input: _,
            count: _,
            kind,
        } => {
            if let Some(frames) = kind.placements() {
                h.write_u64(frames.len() as u64);
                for x in frames
                    .iter()
                    .flat_map(|f| f.columns.iter().flatten().chain(f.translation.iter()))
                {
                    h.write_f64_bits(*x);
                }
            }
        }
        // A blend's SELECTION is recipe payload, not a slot: two
        // blends of the same size on different edges are different
        // nodes (M6-5). Canonical order (the construction doors) is
        // what makes this a set hash rather than an order hash.
        //
        // Its size slot's EXPRESSION is an input too, when the verb's
        // declared flow lands that parameter in a stored field: the
        // minted body then carries the expression's lowered identity
        // in its field rows, so two spellings of one value are two
        // different bodies and must not serve each other from the
        // memo (SEAT-6, key format v4). The fillet's radius is
        // flow-bearing; the chamfer's setback reaches no field and
        // feeds nothing — the rule is read off the declaration rather
        // than written per verb.
        Node::Fillet {
            radius: _,
            selection,
        } => {
            feed_scalar_join(
                &mut h,
                node,
                defs,
                selected(selections, *selection),
                crate::verbs::blend::FILLET_SLOTS,
            );
        }
        Node::Chamfer {
            distance: _,
            selection,
        } => {
            feed_scalar_join(
                &mut h,
                node,
                defs,
                selected(selections, *selection),
                crate::verbs::blend::CHAMFER_SLOTS,
            );
        }
        // The open list feeds IN ORDER, because the order is meaning:
        // the first designated face of a chart carries the rim, so two
        // shells naming the same faces in different orders mint
        // different names and must not serve each other from the memo.
        // The order feeds across DISTINCT charts too, where it carries
        // no rim — a harmless over-discrimination (two memo entries for
        // one body), accepted over a feed that would have to know
        // which faces share a chart. The thickness slot's expression
        // feeds only if the verb's declared flow lands it in a stored
        // field — read off the declaration, exactly as the blends'.
        Node::Shell { thickness: _, open } => {
            feed_scalar_join(
                &mut h,
                node,
                defs,
                selected(selections, *open),
                crate::verbs::shell::SHELL_SLOTS,
            );
        }
        // A measure's PRIMITIVE is payload; its two references are
        // reads, whose bodies the upstream keys carry in argument order,
        // and a reference that is a selection feeds its name, as a
        // blend's selection does. No body is fed by id (D8): two
        // measures differing only in which of two equal bodies they read
        // are one content and two names, which the naming key tells
        // apart.
        Node::Measure { primitive } => {
            use crate::measure::MeasurePrimitive as P;
            h.write_tag(match primitive {
                P::Distance { .. } => 1,
                P::Angle { .. } => 2,
                P::Gap { .. } => 3,
                // APPENDED, never reused: a content-key tag is an
                // identity, so the day a primitive is retired its tag
                // retires with it.
                P::MinClearance { .. } => 4,
            });
            for &r in primitive.refs() {
                let names = selected(selections, r);
                h.write_u64(names.len() as u64);
                for name in names {
                    feed_stable_name(&mut h, name);
                }
            }
        }
        // The RELATION is payload. The value and the bound are payload
        // expressions, whose evaluated values are fed with the others
        // below; the measures under the value are upstream, so their
        // own keys carry them.
        Node::Assertion {
            value: _,
            bound: _,
            relation,
        } => h.write_tag(match relation {
            crate::measure::AssertionRelation::AtLeast => 1,
            crate::measure::AssertionRelation::AtMost => 2,
            crate::measure::AssertionRelation::Equal => 3,
        }),
        // A tube's WINDOW VARIANT is recipe payload, not a slot: which
        // variant it is decides whether the node has window slots at
        // all, so re-authoring a full ring into an arc must move the
        // key by more than the arrival of two slot values. Fed as a
        // tag for both kinds — the two share this payload exactly as
        // they share the slots it governs.
        Node::Tube {
            major_radius: _,
            window,
            minor_radius: _,
        }
        | Node::HollowTube {
            major_radius: _,
            window,
            minor_radius: _,
            wall: _,
        } => {
            h.write_tag(match window {
                crate::node::TubeWindow::Full => 0,
                crate::node::TubeWindow::Arc { t0: _, t1: _ } => 1,
            });
        }
        // Fully expressed by tag plus slots: their whole recipe payload
        // is either an input edge (excluded from the key by design — the
        // inputs' own keys carry it) or a slot expression, fed below.
        Node::Extrude {
            profile: _,
            distance: _,
        }
        | Node::Revolve {
            profile: _,
            axis_through: _,
            axis_heading: _,
            angle: _,
        }
        | Node::Loft {
            profiles: _,
            v_degree: _,
        }
        | Node::Sweep {
            profile: _,
            path: _,
            stations: _,
            v_degree: _,
        }
        | Node::Split { target: _, tool: _ }
        // The index is a slot, fed with the resolved slots below.
        | Node::Part {
            of: _,
            select: PartSelect::Instance(_),
        } => {}
    }
    let indexed: Vec<(usize, usize)> = node
        .body_reads()
        .into_iter()
        .enumerate()
        .filter(|(_, (_, read))| read.is_indexed())
        .map(|(place, (_, read))| (place, read.at.len()))
        .collect();
    if !indexed.is_empty() {
        h.write_tag(tag::index::READS);
        h.write_u64(indexed.len() as u64);
        for (place, rank) in indexed {
            h.write_u64(place as u64);
            h.write_u64(rank as u64);
        }
    }
    let keyed: Vec<(usize, &crate::CopyKey)> = node
        .body_reads()
        .into_iter()
        .enumerate()
        .filter_map(|(place, (_, read))| read.key.as_ref().map(|key| (place, key)))
        .collect();
    if !keyed.is_empty() {
        h.write_tag(tag::copy_key::READS);
        h.write_u64(keyed.len() as u64);
        for (place, mut key) in keyed {
            h.write_u64(place as u64);
            loop {
                match key {
                    crate::CopyKey::Part { placement, within } => {
                        h.write_tag(tag::copy_key::PART);
                        h.write_id(placement.0);
                        key = within;
                    }
                    crate::CopyKey::Read(var) => {
                        h.write_tag(tag::copy_key::READ);
                        h.write_id(var.0);
                        break;
                    }
                }
            }
        }
    }
    // Evaluated slot values, in the node's deterministic slot order,
    // each followed by its NOMINAL — the rule, its exceptions and why
    // the key owes it are at [`tag::slot`]. The two lists come from
    // ONE door ([`slots::eval_slots`]) at two environments, so they
    // are the same slots in the same order and zip pairwise; the
    // slot id is carried into the loop so the pairing is asserted
    // rather than assumed.
    for (i, ((slot, val), (nominal_slot, nominal))) in
        slot_values.iter().zip(nominal_values.iter()).enumerate()
    {
        debug_assert_eq!(slot, nominal_slot, "the two slot lists are one order");
        h.write_u64(i as u64);
        match (val, nominal) {
            (slots::SlotVal::Scalar(v), slots::SlotVal::Scalar(n)) => {
                v.feed(&mut h);
                h.write_tag(tag::slot::NOMINAL);
                h.write_f64_bits(*n);
            }
            // A count feeds no nominal ([`tag::slot`]). The mixed arms
            // are unreachable — one door decided both lists' shapes
            // off the same `SlotId::is_structural` — and are written
            // rather than panicked: the lane value is the node's own
            // answer, and dropping the nominal word from one slot can
            // only ever merge keys the mixed shape already made
            // impossible.
            (slots::SlotVal::Count(c), _) => h.write_i64(*c),
            (slots::SlotVal::Scalar(v), slots::SlotVal::Count(_)) => v.feed(&mut h),
        }
    }
    // The node's evaluated PAYLOAD expressions, in `payload_exprs`
    // order — resolved values, as the slots above are, so a parameter
    // under a measured bound moves the key exactly as one under an
    // extrude's distance does. No nominal rides them, and [`tag::slot`]
    // carries why. Absent (not empty) for every node kind that carries
    // none, so no existing document's key moves by a byte.
    // An expression with no value at this scalar is left out, and its
    // position is a word of its own, so a value missing and a bound
    // missing do not key alike.
    if let Some(payload) = payload_values {
        h.write_u64(payload.values.len() as u64);
        for v in &payload.values {
            v.feed(&mut h);
        }
        h.write_u64(
            payload
                .unavailable
                .map_or(u64::MAX, |(index, _)| index as u64),
        );
    }
    // The pose definitions the node reads ([`tag::pose`]): absent for
    // a node reading none, so no other node's key moves by a byte.
    if let Some(words) = pose_words {
        h.write_u64(words.len() as u64);
        for word in words {
            match word {
                wire::pose::PoseWord::Tag(t) => h.write_tag(*t),
                wire::pose::PoseWord::Count(n) => h.write_u64(*n),
                wire::pose::PoseWord::Name(name) => feed_stable_name(&mut h, name),
                wire::pose::PoseWord::Value(v, n) => {
                    v.feed(&mut h);
                    h.write_tag(tag::slot::NOMINAL);
                    h.write_f64_bits(*n);
                }
            }
        }
    }
    // Upstream identity, by CONTENT (never by id — ids are stable
    // labels, content keys are the Merkle links): each read as its
    // operation's key and the port it reads.
    h.write_u64(upstream_keys.len() as u64);
    for read in upstream_keys {
        h.write_key(read.key);
        feed_port(&mut h, read.port);
    }
    // The recorded witness datum. Its presence is a word of its own so
    // `None` cannot alias a `Some` with empty bytes; a cleared witness
    // and a never-recorded one are both `None` and key alike, which is
    // right — neither is an input.
    match witness {
        None => h.write_tag(tag::presence::ABSENT),
        Some(w) => {
            h.write_tag(tag::presence::PRESENT);
            h.write_u64(u64::from(w.schema));
            h.write_bytes(&w.bytes);
        }
    }
    h.finish()
}

/// **One read a node's keys feed** (D10): the operation `at` defining
/// the variable read, the port read, and that operation's key.
#[derive(Clone, Copy)]
struct UpstreamRead<K> {
    at: RecipeNodeId,
    port: u8,
    key: K,
}

/// A read's port, as the keys feed it: a presence tag, then the port.
/// The tag stays in the byte stream so the keys of every stored
/// document stay what they are.
fn feed_port(h: &mut KeyHasher, port: u8) {
    h.write_tag(tag::presence::PRESENT);
    h.write_u64(u64::from(port));
}

/// The recursive naming key (issue #95 disposition 2; see
/// [`NamingKey`]): the node's own content key plus every read's
/// (operation id, port, naming key), in read order — ids INCLUDED,
/// which is exactly what the content key omits by design (D8).
fn naming_key(content: ContentKey, upstream: &[UpstreamRead<NamingKey>]) -> NamingKey {
    let mut h = KeyHasher::new();
    h.write_tag(tag::naming::DOMAIN);
    h.write_key(content);
    h.write_u64(upstream.len() as u64);
    for read in upstream {
        h.write_id(read.at.0);
        feed_port(&mut h, read.port);
        h.write_u64(read.key.0 as u64);
        h.write_u64((read.key.0 >> 64) as u64);
    }
    NamingKey(h.finish().0)
}

/// The content-key tag of an authoring verb — the ONE place a verb's
/// key identity is chosen, keyed on [`profile::Verb`] rather than on a
/// [`profile::Step`] arm so the choice is a total function of the
/// transition table's own verb census and can be checked as one.
///
/// Two properties, and only one of them is load-bearing. **No two live
/// verbs may share a tag**: verb identity is structure, and a shared
/// tag aliases two programs' digests within a run. **Retired numbers
/// stay dead** — a cheap way to make a re-used number impossible
/// rather than merely unlikely, not a compatibility requirement: keys
/// are process-internal and never persist, so no stored value depends
/// on one. `verb_tags_are_injective` checks both over
/// [`profile::Verb::ALL`].
fn verb_tag(verb: profile::Verb) -> u8 {
    use profile::Verb as V;
    match verb {
        V::At => 10,
        V::Angle => 12,
        V::Toward => 13,
        V::Tangent => 14,
        V::Cusp => 41,
        V::Turn => 15,
        V::Line => 16,
        V::LineTo => 17,
        // 42 rather than 18: within this vocabulary a number is never
        // reassigned (retired numbers stay dead, above), so a verb the
        // table gained after its first numbering takes a fresh number
        // past every one in use — `Cusp`'s 41 was the previous one —
        // rather than the gap, which would re-key every program using
        // the verbs in between.
        V::ContinueTo => 42,
        V::ArcTo => 18,
        V::TangentArcTo => 21,
        V::Fillet => 22,
        V::FarEndTo => 23,
        V::CloseTo => 24,
        V::Circle => 26,
        V::CircleSplit => 27,
        V::FilletArc => 38,
        V::ArcFillet => 39,
        V::ArcFilletArc => 40,
    }
}

/// The tag numbers [`verb_tag`] may not use: retired by the §2c
/// re-spell along with the verbs that held them, and dead for good.
#[cfg(test)]
const RETIRED_VERB_TAGS: &[(u8, &str)] = &[
    (11, "AtOn"),
    (19, "ArcVia"),
    (20, "ArcCenter"),
    (25, "CloseToOn"),
    (28, "ArcContinue"),
    (29, "AtToward"),
];

/// The tag numbers [`content_key`]'s node-kind match may not use:
/// retired with the node kinds that held them, and dead for good.
#[cfg(test)]
const RETIRED_NODE_KIND_TAGS: &[(u8, &str)] = &[
    (1, "Datum plane"),
    (2, "Datum axis"),
    (3, "Datum point"),
    (11, "Transform"),
    (14, "Declare"),
    (23, "Mate"),
    (27, "Datum frame"),
    (32, "Datum face frame"),
    (36, "Gauge"),
    (37, "PlaceInWorld"),
];

/// The content-key tag of an arc mode — the ONE place a mode's key
/// identity is chosen, keyed on [`profile::ArcMode`] rather than on an
/// [`profile::ArcData`] arm so the choice is a total function of the
/// mode set's own declaration and `arc_mode_tags_are_injective` can
/// check it over [`profile::ArcMode::ALL`]. No two modes may share a
/// tag: two arcs of one radius under different modes are different
/// geometry, and a shared word would alias their programs' digests.
fn arc_mode_tag(mode: profile::ArcMode) -> u8 {
    use profile::ArcMode as M;
    match mode {
        M::Radius => 30,
        M::Bulge => 31,
        M::Via => 32,
        M::Center => 33,
        M::Sweep => 34,
        M::ArcLen => 35,
    }
}

/// The content-key tag of a step's target KIND — the payload-free
/// projection of [`profile::Target`], the one place the choice is
/// made so `target_tags_are_injective` can check it over
/// `profile::TargetKind::ALL`, the form list projected from the same
/// declaration as the variants.
fn target_tag(t: &profile::Target<f64>) -> u8 {
    use profile::Target;
    match t {
        Target::Start => tag::target::START,
        Target::StartArriving => tag::target::START_ARRIVING,
        Target::Point(_) => tag::target::POINT,
    }
}

/// The content-key tag of an arc's winding, checked by
/// `winding_tags_are_injective` over a local closed list.
fn winding_tag(w: profile::ArcSweep) -> u8 {
    use profile::ArcSweep;
    match w {
        ArcSweep::Ccw => tag::winding::CCW,
        ArcSweep::Cw => tag::winding::CW,
    }
}

/// The content-key tag of an arc's side, checked by
/// `side_tags_are_injective` over a local closed list.
fn side_tag(s: profile::ArcSide) -> u8 {
    use profile::ArcSide;
    match s {
        ArcSide::Left => tag::side::LEFT,
        ArcSide::Right => tag::side::RIGHT,
    }
}

/// Feeds one RESOLVED program step into the content key (LIB-SWITCH
/// §4e): verb tag, structural tags (target kind, winding, the
/// `circle_split` count), and each continuous arg's resolved-f64 bits
/// under the float tag — (tag, payload) throughout, so structure can
/// never alias float data (the retired token stream's rule, kept).
///
/// The verb tag is written HERE, from [`verb_tag`]; the match below
/// feeds payloads only. Both are exhaustive over the transition
/// table's vocabulary, so a verb the table gains breaks this file at
/// compile — the loud half of the projection. That a verb the table
/// gains also reaches the DOCUMENT vocabulary is not compile-checked
/// and is a census: `tests/switch_program_vocabulary.rs`.
fn feed_step(h: &mut KeyHasher, step: &profile::Step<f64>) {
    use profile::{ArcData, ArcSide, ArcSweep, Step, Target};
    fn f(h: &mut KeyHasher, v: f64) {
        h.write_tag(tag::step::FLOAT);
        h.write_u64(v.to_bits());
    }
    fn target(h: &mut KeyHasher, t: &Target<f64>) {
        h.write_tag(target_tag(t));
        if let Target::Point(p) = t {
            f(h, p.x);
            f(h, p.y);
        }
    }
    fn winding(h: &mut KeyHasher, w: ArcSweep) {
        h.write_tag(winding_tag(w));
    }
    fn side(h: &mut KeyHasher, s: ArcSide) {
        h.write_tag(side_tag(s));
    }
    // The mode's word comes from `arc_mode_tag`; the match below feeds
    // payloads only, in the mode's field order.
    fn spec(h: &mut KeyHasher, s: &ArcData<f64>) {
        h.write_tag(arc_mode_tag(s.mode()));
        match s {
            ArcData::Radius { r, side: sd } => {
                f(h, *r);
                side(h, *sd);
            }
            ArcData::Bulge { target: t, b } => {
                target(h, t);
                f(h, *b);
            }
            ArcData::Via { q, target: t } => {
                f(h, q.x);
                f(h, q.y);
                target(h, t);
            }
            ArcData::Center {
                c,
                winding: w,
                target: t,
            } => {
                f(h, c.x);
                f(h, c.y);
                winding(h, *w);
                target(h, t);
            }
            ArcData::Sweep { r, side: sd, angle } => {
                f(h, *r);
                side(h, *sd);
                f(h, *angle);
            }
            ArcData::ArcLen { r, side: sd, len } => {
                f(h, *r);
                side(h, *sd);
                f(h, *len);
            }
        }
    }
    h.write_tag(verb_tag(step.verb()));
    match step {
        Step::At(p) | Step::FarEndTo(p) => {
            f(h, p.x);
            f(h, p.y);
        }
        Step::Angle(theta) => f(h, *theta),
        Step::Toward { dx, dy } => {
            f(h, *dx);
            f(h, *dy);
        }
        Step::Tangent | Step::Cusp | Step::CloseTo => {}
        Step::Turn(delta) => f(h, *delta),
        Step::Line(len) => f(h, *len),
        Step::LineTo(t) | Step::ContinueTo(t) | Step::TangentArcTo(t) => target(h, t),
        Step::ArcTo(data) => spec(h, data),
        Step::Fillet { radius } => f(h, *radius),
        Step::FilletArc { radius, spec: sp } => {
            f(h, *radius);
            spec(h, sp);
        }
        Step::ArcFillet { spec: sp, radius } => {
            spec(h, sp);
            f(h, *radius);
        }
        Step::ArcFilletArc {
            spec: sp,
            radius,
            spec2,
        } => {
            spec(h, sp);
            f(h, *radius);
            spec(h, spec2);
        }
        Step::Circle { centre, radius } => {
            f(h, centre.x);
            f(h, centre.y);
            f(h, *radius);
        }
        Step::CircleSplit {
            centre,
            radius,
            n,
            phase,
        } => {
            f(h, centre.x);
            f(h, centre.y);
            f(h, *radius);
            // A structural count under its own word — the (tag,
            // payload) discipline holds for every token.
            h.write_tag(tag::step::COUNT);
            h.write_u64(*n as u64);
            f(h, *phase);
        }
    }
}

/// Feeds one LANE-resolved program step's continuous arguments into
/// the content key, through [`ContentBits`] (M10-P PP5).
///
/// Structural tags are deliberately absent: the f64 stream that ran
/// just before this one already carries every verb tag, target kind and
/// winding, and structure is lane-independent by construction — so
/// repeating it here would hash the same facts twice and say nothing.
/// What this adds is exactly what the lane sees and the f64 pass does
/// not: the seed riding a `Dual`'s tangent channel, the width of an
/// `Interval` box. Both channels of a dual feed, which is what makes a
/// seeded memo entry sound rather than aliasing the unseeded one.
///
/// Exhaustive over the step vocabulary for the same reason
/// [`feed_step`] is: a verb the transition table gains must break this
/// file at compile rather than fall silently out of the key.
fn feed_lane_step<T: ContentBits>(h: &mut KeyHasher, step: &profile::Step<T>) {
    use profile::{ArcData, Step, Target};
    fn f<T: ContentBits>(h: &mut KeyHasher, v: &T) {
        h.write_tag(tag::program::LANE_SCALAR);
        v.feed(h);
    }
    fn pt<T: ContentBits>(h: &mut KeyHasher, p: &geom_core::Point2<T>) {
        f(h, &p.x);
        f(h, &p.y);
    }
    fn target<T: ContentBits>(h: &mut KeyHasher, t: &Target<T>) {
        match t {
            // The Start/Point distinction is structural and rides the
            // f64 stream; only a Point's coordinates are lane data. A
            // declared arrival is structural for the same reason.
            Target::Start | Target::StartArriving => {}
            Target::Point(p) => pt(h, p),
        }
    }
    fn spec<T: ContentBits>(h: &mut KeyHasher, s: &ArcData<T>) {
        match s {
            ArcData::Radius { r, .. } => f(h, r),
            ArcData::Bulge { target: t, b } => {
                target(h, t);
                f(h, b);
            }
            ArcData::Via { q, target: t } => {
                pt(h, q);
                target(h, t);
            }
            ArcData::Center { c, target: t, .. } => {
                pt(h, c);
                target(h, t);
            }
            ArcData::Sweep { r, angle, .. } => {
                f(h, r);
                f(h, angle);
            }
            ArcData::ArcLen { r, len, .. } => {
                f(h, r);
                f(h, len);
            }
        }
    }
    match step {
        Step::At(p) | Step::FarEndTo(p) => pt(h, p),
        Step::Angle(v) | Step::Turn(v) | Step::Line(v) => f(h, v),
        Step::Toward { dx, dy } => {
            f(h, dx);
            f(h, dy);
        }
        Step::Tangent | Step::Cusp | Step::CloseTo => {}
        Step::LineTo(t) | Step::ContinueTo(t) | Step::TangentArcTo(t) => target(h, t),
        Step::ArcTo(s) => spec(h, s),
        Step::Fillet { radius } => f(h, radius),
        Step::FilletArc { radius, spec: s } => {
            f(h, radius);
            spec(h, s);
        }
        Step::ArcFillet { spec: s, radius } => {
            spec(h, s);
            f(h, radius);
        }
        Step::ArcFilletArc {
            spec: s,
            radius,
            spec2,
        } => {
            spec(h, s);
            f(h, radius);
            spec(h, spec2);
        }
        Step::Circle { centre, radius } => {
            pt(h, centre);
            f(h, radius);
        }
        // `n` is a structural count and rides the f64 stream.
        Step::CircleSplit {
            centre,
            radius,
            phase,
            ..
        } => {
            pt(h, centre);
            f(h, radius);
            f(h, phase);
        }
    }
}

/// Feeds a document reference: the id's two words, then the content
/// pin's four — what an instantiate node keys its part by, and what a
/// mate's face-based side keys the part it resolves against by.
fn feed_doc_ref(h: &mut KeyHasher, doc_ref: &crate::ident::DocRef) {
    h.write_u64((doc_ref.id.0 >> 64) as u64);
    h.write_u64(doc_ref.id.0 as u64);
    for chunk in doc_ref.pin.0.chunks(8) {
        let mut byte8 = [0u8; 8];
        byte8[..chunk.len()].copy_from_slice(chunk);
        h.write_u64(u64::from_be_bytes(byte8));
    }
}

/// **A bundle's key**: each constraint's id and what it says that no
/// other channel keys — its class, and its target's word, a world read
/// by its arms and roles. The poses it reads are reads, keyed by the
/// node's pose words and its upstream keys, and a world read's scalars
/// are slots, fed with every slot.
fn feed_bundle(h: &mut KeyHasher, constraints: &[crate::mate::Constraint]) {
    h.write_u64(constraints.len() as u64);
    for c in constraints {
        h.write_u64(u64::from(c.id.0.ordinal()));
        h.write_u64(c.id.0.digest());
        let mate = match &c.kind {
            crate::mate::ConstraintKind::Mate(mate) => mate,
            crate::mate::ConstraintKind::Value(value) => {
                use crate::mate::{Coord, SlideAxis};
                h.write_tag(tag::constraint_kind::VALUE);
                h.write_u64(u64::from(value.mate.0.ordinal()));
                h.write_u64(value.mate.0.digest());
                h.write_tag(match value.coord {
                    Coord::Slide(SlideAxis::X) => tag::value_coord::SLIDE_X,
                    Coord::Slide(SlideAxis::Y) => tag::value_coord::SLIDE_Y,
                    Coord::Slide(SlideAxis::Z) => tag::value_coord::SLIDE_Z,
                    Coord::Spin => tag::value_coord::SPIN,
                });
                continue;
            }
        };
        h.write_tag(tag::constraint_kind::MATE);
        // The class's word is `ContactClass::content_tag` — the one
        // spelling the crossing record and the constraint both write.
        h.write_u64(mate.class.content_tag());
        match &mate.to {
            crate::mate::Target::Pose(_) => h.write_tag(tag::mate_target::POSE),
            crate::mate::Target::World(world) => {
                h.write_tag(tag::mate_target::WORLD);
                feed_world_read(h, world);
            }
        }
    }
}

/// A world read's arms in pre-order, a role arm with its role's word.
fn feed_world_read(h: &mut KeyHasher, world: &crate::mate::WorldRead) {
    use crate::mate::WorldRead as W;
    match world {
        W::Frame => h.write_tag(tag::world_read::FRAME),
        W::Role { pose, role } => {
            h.write_tag(tag::world_read::ROLE);
            h.write_tag(wire::pose::role_word(Some(*role)));
            feed_world_read(h, pose);
        }
        W::Slide { pose, along, .. } => {
            h.write_tag(tag::world_read::SLIDE);
            feed_world_read(h, pose);
            feed_world_read(h, along);
        }
        W::Spin { pose, about, .. } => {
            h.write_tag(tag::world_read::SPIN);
            feed_world_read(h, pose);
            feed_world_read(h, about);
        }
        W::Flip { pose } => {
            h.write_tag(tag::world_read::FLIP);
            feed_world_read(h, pose);
        }
    }
}

/// The names the selection `var` holds, as a content key feeds them:
/// none for a read that is no selection.
fn selected<'d>(
    selections: &dyn Fn(crate::VarId) -> Option<&'d crate::var::Select>,
    var: crate::VarId,
) -> &'d [StableName] {
    selections(var).map_or(&[], |select| select.names.as_slice())
}

/// **A one-scalar verb's selection and its flow-bearing slot**, fed
/// as the blends and the shell all feed them: the names in the order
/// the selection holds them (sorted for edges, designation order for
/// faces), then the slot's EXPRESSION under `FLOW_EXPR` when the
/// verb's declared flow lands it in a stored field (`content_key`'s
/// arms).
fn feed_scalar_join(
    h: &mut KeyHasher,
    node: &crate::node::Node<ProfileProgram>,
    defs: crate::param_source::Definitions<'_, '_>,
    names: &[StableName],
    join: crate::verbs::SlotJoin,
) {
    h.write_u64(names.len() as u64);
    for n in names {
        feed_stable_name(h, n);
    }
    if crate::param_source::flow_bearing(join.size_param)
        && let Some(&var) = node.expr(join.size_slot)
    {
        h.write_tag(tag::scalar_join::FLOW_EXPR);
        crate::param_source::feed_var(h, defs, var);
    }
}

/// Feeds one stable name: its entity kind as a tag, its minting node,
/// then its role path segment by segment. Every field participates —
/// a name is an identity, and two names differing anywhere are two
/// different recipe payloads. Names are float-free by construction
/// (pure tags and integers), so nothing here is eps-dependent.
///
/// A name holds names as deep as its derivation runs, so the feed keeps
/// its own stack: each level's segments are fed into a [`SegFeed`],
/// which holds a name it meets in place, and the stream is then fed in
/// order, each held name fed where it stands.
fn feed_stable_name(h: &mut KeyHasher, name: &StableName) {
    use crate::names::EntityKind;
    let mut fed = vec![Fed::Name(name)];
    while let Some(item) = fed.pop() {
        match item {
            Fed::Tag(tag) => h.write_tag(tag),
            Fed::U64(x) => h.write_u64(x),
            Fed::Name(name) => {
                h.write_tag(match name.kind {
                    EntityKind::Body => 1,
                    EntityKind::Face => 2,
                    EntityKind::Edge => 3,
                    EntityKind::Vertex => 4,
                });
                h.write_id(name.node.0);
                h.write_u64(name.path.len() as u64);
                let mut level = SegFeed(Vec::new());
                for seg in &name.path {
                    feed_role_seg(&mut level, seg);
                }
                fed.extend(level.0.into_iter().rev());
            }
        }
    }
}

/// One item of a name's content-key stream.
enum Fed<'a> {
    Tag(u8),
    U64(u64),
    /// A held name, fed whole where it stands.
    Name(&'a StableName),
}

/// One level of a name's content-key stream, in order.
struct SegFeed<'a>(Vec<Fed<'a>>);

impl<'a> SegFeed<'a> {
    fn write_tag(&mut self, tag: u8) {
        self.0.push(Fed::Tag(tag));
    }

    fn write_u64(&mut self, x: u64) {
        self.0.push(Fed::U64(x));
    }

    fn write_id(&mut self, id: crate::MintId) {
        self.write_u64(u64::from(id.ordinal()));
        self.write_u64(id.digest());
    }

    fn name(&mut self, name: &'a StableName) {
        self.0.push(Fed::Name(name));
    }
}


/// A crossing sense's key tag: a run line with no sense (a seam) is 0.
fn sense_tag(sense: Option<crate::names::Sense>) -> u8 {
    use crate::names::Sense;
    match sense {
        None => 0,
        Some(Sense::Enters) => 1,
        Some(Sense::Leaves) => 2,
        Some(Sense::Outside) => 3,
        Some(Sense::Inside) => 4,
    }
}

/// The content-key tag of a role segment — the ONE place a segment's
/// key identity is chosen, keyed on the payload-free [`SegTag`] so the
/// choice is a total function of the role vocabulary's own mirror and
/// `seg_content_tags_are_injective` can check it over [`SegTag::ALL`].
/// No two segments may share a tag: two role segments hashing alike
/// make two different names hash alike, and a content key that collides
/// serves one node's cached geometry for another's.
fn seg_content_tag(tag: SegTag) -> u8 {
    use SegTag as S;
    match tag {
        S::OutputBody => 1,
        S::Cap => 2,
        S::Lateral => 3,
        S::RimEdge => 4,
        S::LateralEdge => 5,
        S::CapVertex => 6,
        S::LoftWall => 45,
        S::LoftSeam => 46,
        S::Band => 7,
        S::BandRim => 8,
        S::BandRimPi => 9,
        S::BandPi => 10,
        S::Meridian => 11,
        S::MeridianVertex => 12,
        S::RevolveCap => 13,
        S::Pole => 14,
        S::AxisEdge => 15,
        S::From => 52,
        S::Seam => 18,
        S::Crossing => 47,
        S::EdgeCrossing => 48,
        S::Merged => 19,
        S::Fragment => 20,
        S::SplitBody => 21,
        S::SectionFace => 22,
        S::SectionEdge => 23,
        S::SplitFragment => 24,
        S::CrossingVertex => 25,
        S::OnToolVertex => 27,
        S::BlendFace => 29,
        S::CornerFace => 30,
        S::TrimEdge => 31,
        S::FootVertex => 32,
        S::EndArc => 33,
        S::Mitre => 49,
        S::TurnFoot => 50,
        S::BandFace => 34,
        S::BandTrim => 35,
        S::BandFoot => 36,
        S::BandCross => 37,
        S::BandCut => 38,
        S::BandSlit => 39,
        S::Inner => 42,
        S::Rim => 43,
        S::HoleRim => 44,
        S::Instance => 26,
        S::InPart => 40,
        S::Placed => 51,
    }
}

/// Feeds one role segment: its word from [`seg_content_tag`], then the
/// payload its variant carries.
fn feed_role_seg<'a>(h: &mut SegFeed<'a>, seg: &'a crate::names::RoleSeg) {
    use crate::names::{CapEnd, MeridianEnd, Qualifier, RoleSeg};
    let cap = |c: CapEnd| match c {
        CapEnd::End => 1u8,
        CapEnd::Start => 2,
    };
    let mer = |m: MeridianEnd| match m {
        MeridianEnd::Start => 1u8,
        MeridianEnd::End => 2,
        MeridianEnd::Seam => 3,
        MeridianEnd::Pi => 4,
    };
    let rim = |s: crate::names::RimSupport| match s {
        crate::names::RimSupport::Host => 1u8,
        crate::names::RimSupport::Mate => 2,
    };
    // A locator: its form's word, then the step's minted id or the
    // section's circle, then the role (its word, and a piece's index).
    let role = |h: &mut SegFeed<'a>, r: crate::names::PieceRole| {
        use crate::names::PieceRole;
        match r {
            PieceRole::Leg => h.write_tag(1),
            PieceRole::RunIn => h.write_tag(2),
            PieceRole::Arc => h.write_tag(3),
            PieceRole::RunOut => h.write_tag(4),
            PieceRole::Piece(k) => {
                h.write_tag(5);
                h.write_u64(u64::from(k));
            }
        }
    };
    let circle = |c: crate::names::SectionCircle| match c {
        crate::names::SectionCircle::Outer => 1u8,
        crate::names::SectionCircle::Bore => 2,
    };
    let pe = |h: &mut SegFeed<'a>, e: crate::names::ProfileEdgeRef| match e {
        crate::names::ProfileEdgeRef::Piece { step, role: r } => {
            h.write_tag(1);
            h.write_id(step.0);
            role(h, r);
        }
        crate::names::ProfileEdgeRef::Section { circle: c, role: r } => {
            h.write_tag(2);
            h.write_tag(circle(c));
            role(h, r);
        }
    };
    // A run of pieces: one piece feeds as that piece, so a one-piece
    // run digests as the bare locator did; several feed under a tag no
    // locator starts with, then the count and the pieces.
    let run = |h: &mut SegFeed<'a>, r: &crate::names::PieceRun| match r.single() {
        Some(e) => pe(h, e),
        None => {
            h.write_tag(3);
            h.write_u64(r.pieces().len() as u64);
            for e in r.pieces() {
                pe(h, *e);
            }
        }
    };
    let pv = |h: &mut SegFeed<'a>, v: crate::names::ProfileVertexRef| match v {
        crate::names::ProfileVertexRef::Piece { step, role: r } => {
            h.write_tag(1);
            h.write_id(step.0);
            role(h, r);
        }
        crate::names::ProfileVertexRef::Section { circle: c, role: r } => {
            h.write_tag(2);
            h.write_tag(circle(c));
            role(h, r);
        }
    };
    let qual = |h: &mut SegFeed<'a>, q: &'a Qualifier| {
        h.write_tag(match q {
            Qualifier::OrderAlong { .. } => 2,
            Qualifier::Borders(..) => 3,
            Qualifier::Keeps(..) => 4,
            Qualifier::Ends(..) => 5,
        });
        match q {
            Qualifier::Borders(walls) | Qualifier::Keeps(walls) | Qualifier::Ends(walls) => {
                h.write_u64(walls.len() as u64);
                for name in walls {
                    h.name(name);
                }
            }
            Qualifier::OrderAlong { rank, of } => {
                h.write_u64(u64::from(*rank));
                h.write_u64(u64::from(*of));
            }
        }
    };
    // The segment's word first, from `seg_content_tag`; the match
    // below feeds payloads only. The closures above (qualifier, cap
    // end, meridian end, split half, rim support) are
    // vocabularies of their own, each read under a segment word.
    h.write_tag(seg_content_tag(SegTag::of(seg)));
    match seg {
        RoleSeg::OutputBody => {}
        RoleSeg::Cap(c) => {
            h.write_tag(cap(*c));
        }
        RoleSeg::Lateral(r) => {
            run(h, r);
        }
        RoleSeg::RimEdge(c, r) => {
            h.write_tag(cap(*c));
            run(h, r);
        }
        RoleSeg::LateralEdge(v) => {
            pv(h, *v);
        }
        RoleSeg::CapVertex(c, v) => {
            h.write_tag(cap(*c));
            pv(h, *v);
        }
        RoleSeg::LoftWall(es) => {
            h.write_u64(es.len() as u64);
            for e in es {
                pe(h, *e);
            }
        }
        RoleSeg::LoftSeam(vs) => {
            h.write_u64(vs.len() as u64);
            for v in vs {
                pv(h, *v);
            }
        }
        RoleSeg::Band(r) => {
            run(h, r);
        }
        RoleSeg::BandRim(v) => {
            pv(h, *v);
        }
        RoleSeg::BandRimPi(v) => {
            pv(h, *v);
        }
        RoleSeg::BandPi(r) => {
            run(h, r);
        }
        RoleSeg::Meridian(m, r) => {
            h.write_tag(mer(*m));
            run(h, r);
        }
        RoleSeg::MeridianVertex(m, v) => {
            h.write_tag(mer(*m));
            pv(h, *v);
        }
        RoleSeg::RevolveCap(m) => {
            h.write_tag(mer(*m));
        }
        RoleSeg::Pole(v) => {
            pv(h, *v);
        }
        RoleSeg::AxisEdge(r) => {
            run(h, r);
        }
        // An entity carried in: BOTH halves feed. Two members of one
        // union can be placements of ONE body and then carry the same
        // inner name, so a key without the read would give their
        // entities one key — the memo hazard the segment vocabulary
        // exists to prevent.
        RoleSeg::From { read, of } => {
            h.write_id(read.0);
            h.name(of);
        }
        RoleSeg::Seam { a, b } => {
            h.name(a);
            h.name(b);
        }
        RoleSeg::Crossing { edge, face, sense } => {
            h.name(edge);
            h.name(face);
            h.write_tag(sense_tag(Some(*sense)));
        }
        RoleSeg::EdgeCrossing(run) => {
            h.write_u64(run.len() as u64);
            for l in run {
                h.name(&l.line);
                h.write_tag(sense_tag(l.sense));
            }
        }
        RoleSeg::Merged(names) => {
            h.write_u64(names.len() as u64);
            for n in names {
                h.name(n);
            }
        }
        RoleSeg::Fragment(q) => {
            qual(h, q);
        }
        RoleSeg::SplitBody => {}
        RoleSeg::SectionFace { section } => {
            h.write_u64(u64::from(*section));
        }
        RoleSeg::SectionEdge { face } => {
            h.name(face);
        }
        RoleSeg::SplitFragment { parent } => {
            h.name(parent);
        }
        RoleSeg::CrossingVertex { edge, sense } => {
            h.name(edge);
            h.write_tag(sense_tag(Some(*sense)));
        }
        RoleSeg::OnToolVertex { of } => {
            h.name(of);
        }
        RoleSeg::InPart { of } | RoleSeg::Placed { of } => {
            h.name(of);
        }
        RoleSeg::Instance { i, of } => {
            h.write_u64(u64::from(*i));
            h.name(of);
        }
        RoleSeg::BlendFace(n) => {
            h.name(n);
        }
        RoleSeg::CornerFace(n) => {
            h.name(n);
        }
        RoleSeg::TrimEdge { edge, support } => {
            h.name(edge);
            h.name(support);
        }
        RoleSeg::FootVertex { vertex, support } => {
            h.name(vertex);
            h.name(support);
        }
        RoleSeg::EndArc { vertex, edge } => {
            h.name(vertex);
            h.name(edge);
        }
        RoleSeg::Mitre { vertex } | RoleSeg::TurnFoot { vertex } => {
            h.name(vertex);
        }
        RoleSeg::BandFace(names) => {
            h.write_u64(names.len() as u64);
            for n in names {
                h.name(n);
            }
        }
        RoleSeg::BandTrim { edge, support } => {
            h.name(edge);
            h.write_tag(rim(*support));
        }
        RoleSeg::BandFoot(n) => {
            h.name(n);
        }
        RoleSeg::BandCut(n) => {
            h.name(n);
        }
        RoleSeg::BandCross { edge, band } | RoleSeg::BandSlit { edge, band } => {
            h.name(edge);
            h.write_u64(band.len() as u64);
            for n in band {
                h.name(n);
            }
        }
        // The shell's three roles. Each wraps one source name; the hole
        // rim carries its pairing index beside it, the way `Instance`
        // carries `i`.
        RoleSeg::Inner(n) => h.name(n),
        RoleSeg::Rim(n) => h.name(n),
        RoleSeg::HoleRim { of, hole } => {
            h.write_u64(u64::from(*hole));
            h.name(of);
        }
    }
}

#[cfg(test)]
#[allow(clippy::panic, clippy::expect_used)]
mod tag_vocabulary_tests {
    //! Every vocabulary of the content key is injective over its whole
    //! row set — the groups over `tag::GROUPS`, each enum function over
    //! its enum's `ALL` or a local closed list, the node-kind match by
    //! its source census — and the committed numbers are pinned where
    //! a swapped arm would otherwise stay green.

    use super::{
        KeyHasher, RETIRED_NODE_KIND_TAGS, RETIRED_VERB_TAGS, arc_mode_tag,
        feed_lane_step, feed_step, seg_content_tag, side_tag, tag, target_tag, verb_content_tag,
        verb_tag, winding_tag,
    };
    use crate::names::SegTag;

    /// **The seam-arrival target tag is reachable and distinct.**
    /// `START_ARRIVING` is the third word of the target vocabulary
    /// (`tag::target`), beside `START` and `POINT`; `43`, the retired
    /// second declaration, stays dead there.
    ///
    /// This EXECUTES it. Both hashers are otherwise reached only by
    /// evaluating a document node that carries the new target, which
    /// nothing in the tree does, so without this row the arms are
    /// compiled and never run — and a declared seam sharing `Start`'s
    /// key would alias two different loops into one content key.
    #[test]
    fn the_seam_arrival_target_tag_is_distinct_and_executed() {
        use profile::{Step, Target};
        let key = |t: Target<f64>| {
            let mut h = KeyHasher::new();
            feed_step(&mut h, &Step::LineTo(t));
            h.finish()
        };
        let start = key(Target::Start);
        let declared = key(Target::StartArriving);
        assert_ne!(start, declared, "Start and a declared tangent seam joint");
        // The LANE hasher treats all three as structural — no lane data
        // rides them — so its keys agree, which is the property that
        // makes a bisecting lane's memo sound.
        let lane = |t: Target<f64>| {
            let mut h = KeyHasher::new();
            feed_lane_step(&mut h, &Step::LineTo(t));
            h.finish()
        };
        assert_eq!(lane(Target::Start), lane(Target::StartArriving));
    }

    /// [`verb_tag`]'s two properties, computed over the transition
    /// table's own census rather than reviewed: every live verb gets a
    /// distinct tag, and none re-uses a retired number. Anchored on
    /// [`profile::Verb::ALL`], so a verb the table gains is measured
    /// here the moment `verb_tag` grows an arm for it.
    #[test]
    fn verb_tags_are_injective() {
        let mut seen: Vec<(profile::Verb, u8)> = Vec::new();
        for verb in profile::Verb::ALL {
            let tag = verb_tag(*verb);
            if let Some((_, held_by)) = RETIRED_VERB_TAGS.iter().find(|(t, _)| *t == tag) {
                panic!("{verb:?} re-uses tag {tag}, retired with {held_by}");
            }
            if let Some((other, _)) = seen.iter().find(|(_, t)| *t == tag) {
                panic!("{verb:?} and {other:?} share content-key tag {tag}");
            }
            seen.push((*verb, tag));
        }
        assert_eq!(seen.len(), profile::Verb::ALL.len());
    }

    /// **The migrated verbs' content tags are the numbers the inline
    /// match already wrote**, pinned digit by digit.
    ///
    /// This is the load-bearing row of moving the match: a memo key is
    /// a function of the tag, so a tag that moved would silently
    /// re-key every document carrying that node — every blend
    /// document in the corpus recomputing where it used to hit, and,
    /// worse, no red anywhere to say so. The numbers below are read
    /// off the pre-change source, not off the function.
    #[test]
    fn verb_content_tags_are_the_committed_numbers() {
        assert_eq!(verb_content_tag(verbs::VerbKind::Fillet), Some(17));
        assert_eq!(verb_content_tag(verbs::VerbKind::Chamfer), Some(24));
        // The sweeps' two, read off the pre-change source the same
        // way: 5 and 6 were the tag match's inline numbers for the
        // extrude and the revolve, and moving the match must not move
        // them — a moved tag re-keys every document in the registry
        // that carries a sweep, which is nearly all of them, with no
        // red anywhere to say so.
        assert_eq!(verb_content_tag(verbs::VerbKind::Extrude), Some(5));
        assert_eq!(verb_content_tag(verbs::VerbKind::Revolve), Some(6));
        // The split's, read the same way: 7 was the tag match's inline
        // number for `Node::Split`, and every split-carrying document
        // in the registry keys on it.
        assert_eq!(verb_content_tag(verbs::VerbKind::Split), Some(7));
        assert_eq!(
            verb_content_tag(verbs::VerbKind::Boolean(topo::BooleanOp::Union)),
            Some(8)
        );
        assert_eq!(
            verb_content_tag(verbs::VerbKind::Boolean(topo::BooleanOp::Intersect)),
            Some(9)
        );
        assert_eq!(
            verb_content_tag(verbs::VerbKind::Boolean(topo::BooleanOp::Subtract)),
            Some(10)
        );
        // The shell's, read off the pre-change source the same way: 35
        // was the next free number in the node-tag space when
        // `Node::Shell` landed, and every shell-carrying document keys
        // on it.
        assert_eq!(verb_content_tag(verbs::VerbKind::Shell), Some(35));
    }

    /// No two verbs share a tag — the property `verb_tag`'s injectivity
    /// row asserts for the profile vocabulary, here for the node one,
    /// computed over `VerbKind::ALL` so a verb the vocabulary gains is
    /// measured the moment it has an arm.
    #[test]
    fn verb_content_tags_are_injective() {
        let mut seen: Vec<(verbs::VerbKind, u8)> = Vec::new();
        let mut kernel_only: Vec<verbs::VerbKind> = Vec::new();
        for kind in verbs::VerbKind::ALL {
            // A kernel-only verb is not in the vocabulary and is counted
            // rather than skipped: the two buckets together must be the
            // whole vocabulary, so a verb that fell out of both — the
            // failure a plain `continue` would hide — reds on the sum.
            let Some(tag) = verb_content_tag(*kind) else {
                kernel_only.push(*kind);
                continue;
            };
            assert!(
                !seen.iter().any(|(_, t)| *t == tag),
                "{kind:?} shares content tag {tag} with {:?}",
                seen.iter().find(|(_, t)| *t == tag).map(|(k, _)| *k)
            );
            seen.push((*kind, tag));
        }
        assert_eq!(
            seen.len() + kernel_only.len(),
            verbs::VerbKind::ALL.len(),
            "the tagged and the kernel-only rows are not the whole vocabulary"
        );
    }

    /// **The node-kind vocabulary is injective** — the migrated verbs'
    /// tags and every tag still written inline in `content_key`'s node
    /// match, checked as the one vocabulary they are — and none re-uses
    /// a number in [`RETIRED_NODE_KIND_TAGS`].
    ///
    /// The row above is not this row. It says no two VERBS collide, and
    /// it would stay green while a new inline node claimed 17 or 24 —
    /// which is precisely the accident that moving two tags out of the
    /// match created the room for, and precisely the accident the S4
    /// lesson (two steps sharing one content-key tag, caught by a
    /// reviewer rather than a type) says costs a memo hit serving
    /// another node's geometry.
    ///
    /// **It is a source census, and that is the honest shape here.** The
    /// tags live in a match over `&Node`, so enumerating them by calling
    /// `content_key` would mean constructing one of every node variant —
    /// a fixture larger than the property, and one that would go stale
    /// silently. Instead the sentinels bracketing that match delimit the
    /// text, every `=> <number>` inside it is read as an inline tag, and
    /// every `document_verb_tag(VerbKind::X)` is read as a migrated one
    /// and resolved through the real function. Nothing is hand-listed,
    /// so nothing drifts: a tag added inside the sentinels is measured
    /// the moment it is typed.
    ///
    /// A KERNEL-ONLY verb contributes nothing to the space and is
    /// exempted by its own declaration (`verb_content_tag` answering
    /// `None`), never by a list here — so a verb that later gains a
    /// node arm is measured the moment its declaration gains a number.
    ///
    /// **What it covers, exactly: the node-kind match and nothing
    /// else.** Every other vocabulary in the key — the structural
    /// groups in `tag`, the verb, arc-mode and segment functions — is
    /// declared elsewhere and censused by its own row (`tag`'s doc
    /// lists them), so a tag written outside the sentinels is a
    /// different vocabulary read at a different position, never a gap
    /// in this one. What it cannot see, stated: a node-kind tag whose
    /// arm computes rather than names a number. None exists.
    #[test]
    fn node_kind_vocabulary_is_injective() {
        const SOURCE: &str = include_str!("mod.rs");
        // The sentinel walk is `test_utils::source`'s, not this row's:
        // three sites had written it themselves, and the third was
        // nearly line-for-line the second.
        let region = &SOURCE[test_utils::source::sentinel_region(
            SOURCE,
            "eval/mod.rs",
            "NODE-KIND-VOCABULARY BEGIN",
            "NODE-KIND-VOCABULARY END",
        )];
        // Comments inside the region discuss tag numbers in prose ("24
        // is the chamfer's"), which are not arms — blanked through the
        // SHARED Rust reader rather than a `split("//")` this test rolled
        // itself, which would have mis-read a `//` inside a string.
        let code_only = test_utils::source::code_and_literals(region);
        let mut tags: Vec<(u8, String)> = Vec::new();
        for (n, code) in code_only.lines().enumerate() {
            if let Some(rest) = code.split_once("=> ") {
                let token: String = rest.1.chars().take_while(char::is_ascii_digit).collect();
                if let Ok(tag) = token.parse::<u8>() {
                    tags.push((tag, format!("inline arm at region line {n}")));
                    continue;
                }
            }
            // A vocabulary row is matched by its VARIANT token (the
            // name up to any payload parenthesis): the boolean's arm
            // is written `VerbKind::Boolean(*op)` and covers all three
            // op rows, so each of them is resolved through the real
            // function for that one source line.
            for kind in verbs::VerbKind::ALL {
                let name = format!("{kind:?}");
                let token = name.split('(').next().expect("split yields a first piece");
                // A kernel-only verb has no tag, so a node arm naming
                // one contributes nothing to the space — and cannot
                // exist, since the arms go through `document_verb_tag`.
                if code.contains(&format!("VerbKind::{token}"))
                    && let Some(tag) = verb_content_tag(*kind)
                {
                    tags.push((tag, name));
                }
            }
        }
        // A census that read nothing would pass vacuously.
        assert!(
            tags.len() >= 20,
            "the tag census found only {} tags — the sentinels or the scan have drifted from the \
             match they are supposed to read",
            tags.len()
        );
        for kind in verbs::VerbKind::ALL {
            // The kernel-only rows are exempt BY DECLARATION, read off
            // the same function the space is read off: a verb with no
            // tag has no node arm to be reachable from, and stating it
            // this way means the exemption follows the declaration
            // rather than a list here.
            let Some(want) = verb_content_tag(*kind) else {
                continue;
            };
            assert!(
                tags.iter().any(|(t, _)| *t == want),
                "{kind:?}'s migrated tag is not reachable from the node match — the census is \
                 measuring the wrong region"
            );
        }
        let mut seen: Vec<(u8, String)> = Vec::new();
        for (tag, who) in tags {
            if let Some((_, held_by)) = RETIRED_NODE_KIND_TAGS.iter().find(|(t, _)| *t == tag) {
                panic!("{who} re-uses node-kind tag {tag}, retired with {held_by}");
            }
            assert!(
                !seen.iter().any(|(t, _)| *t == tag),
                "content tag {tag} is claimed twice: by {who} and by {}",
                seen.iter()
                    .find(|(t, _)| *t == tag)
                    .map(|(_, w)| w.clone())
                    .unwrap_or_default()
            );
            seen.push((tag, who));
        }
    }

    /// Within every structural group, no two names share a number,
    /// and a retired number is not reused. Iterates `tag::GROUPS`,
    /// which `tag_groups!` projects from the declarations themselves,
    /// so a word declared is a word read here: a duplicate typed into
    /// any group reds, naming the group and both positions, and so
    /// does a live word that some earlier version of the key wrote
    /// with another meaning.
    ///
    /// The retired lists are per GROUP rather than one list beside the
    /// module, because a number is retired FROM a vocabulary and means
    /// nothing outside it — `43` is dead in `target` and live in
    /// `scalar_join`, `0` is dead in `program` and live in `presence`.
    /// Declaring them in the group they are dead in is what lets the
    /// roster carry both lists and the row read them together.
    #[test]
    fn structural_tag_groups_are_injective() {
        for (group, all, retired) in tag::GROUPS {
            assert!(!all.is_empty(), "group `{group}` declares no tag");
            for (i, a) in all.iter().enumerate() {
                if let Some(j) = all[..i].iter().position(|b| b == a) {
                    panic!("group `{group}` claims tag {a} twice: at positions {j} and {i}");
                }
            }
            for (i, r) in retired.iter().enumerate() {
                if let Some(j) = retired[..i].iter().position(|b| b == r) {
                    panic!("group `{group}` retires tag {r} twice: at positions {j} and {i}");
                }
                assert!(
                    !all.contains(r),
                    "group `{group}` claims tag {r}, which it retired"
                );
            }
        }
    }

    /// No two arc modes share a tag, computed over
    /// [`profile::ArcMode::ALL`] so a mode the set gains is measured the
    /// moment `arc_mode_tag` grows an arm for it.
    #[test]
    fn arc_mode_tags_are_injective() {
        let mut seen: Vec<(profile::ArcMode, u8)> = Vec::new();
        for mode in profile::ArcMode::ALL {
            let tag = arc_mode_tag(*mode);
            if let Some((other, _)) = seen.iter().find(|(_, t)| *t == tag) {
                panic!("{mode:?} and {other:?} share content-key tag {tag}");
            }
            seen.push((*mode, tag));
        }
        assert_eq!(seen.len(), profile::ArcMode::ALL.len());
    }

    /// No two role segments share a tag, computed over
    /// [`SegTag::ALL`]. It matters as a memo hazard of the widest
    /// class: `RoleSeg` is the widest enum in the crate, two segments
    /// sharing a tag make two different names hash alike, and a content
    /// key that collides serves one node's cached geometry for
    /// another's. The nested vocabularies a segment carries (qualifier,
    /// cap end, meridian end, split half, rim support) are
    /// each read under a segment word this row holds unique.
    #[test]
    fn seg_content_tags_are_injective() {
        let mut seen: Vec<(SegTag, u8)> = Vec::new();
        for seg in SegTag::ALL {
            let tag = seg_content_tag(*seg);
            if let Some((other, _)) = seen.iter().find(|(_, t)| *t == tag) {
                panic!("{seg:?} and {other:?} share content-key tag {tag}");
            }
            seen.push((*seg, tag));
        }
        assert_eq!(seen.len(), SegTag::ALL.len());
        // The newest words, pinned: the carried-in read key and the
        // shell's three roles, read off the source the numbers were
        // committed in.
        for (seg, want) in [
            (SegTag::From, 52),
            (SegTag::Inner, 42),
            (SegTag::Rim, 43),
            (SegTag::HoleRim, 44),
        ] {
            assert_eq!(seg_content_tag(seg), want, "{seg:?}'s committed number");
        }
    }

    /// Builds the closed list of a payload-free `profile` enum's
    /// variants from the arms of an exhaustive `match`: the arms ARE
    /// the list, so a variant the enum gains has no arm, fails to
    /// compile here, and is in the list the moment it has one.
    macro_rules! closed_list {
        ($ty:ty; $($v:ident),+ $(,)?) => {{
            let _exhaustive = |x: $ty| match x {
                $(<$ty>::$v => ()),+
            };
            [$(<$ty>::$v),+]
        }};
    }

    /// Asserts a tag function injective over `all` and equal to the
    /// committed numbers in `pins` — so a swapped arm (`Cw => CCW`)
    /// reds on the pin and a duplicated one on the census.
    fn injective_and_pinned<V: core::fmt::Debug + Copy>(
        what: &str,
        all: &[V],
        tag: impl Fn(V) -> u8,
        pins: &[(V, u8)],
        same: impl Fn(V, V) -> bool,
    ) {
        let mut seen: Vec<(V, u8)> = Vec::new();
        for v in all {
            let t = tag(*v);
            if let Some((other, _)) = seen.iter().find(|(_, u)| *u == t) {
                panic!("{what}: {v:?} and {other:?} share content-key tag {t}");
            }
            seen.push((*v, t));
        }
        assert_eq!(seen.len(), all.len());
        assert_eq!(pins.len(), all.len(), "{what}: every variant is pinned");
        for (v, want) in pins {
            assert!(
                all.iter().any(|a| same(*a, *v)),
                "{what}: pinned {v:?} is in the list"
            );
            assert_eq!(tag(*v), *want, "{what}: {v:?}'s committed number");
        }
    }

    /// An arc's winding: two words, injective and pinned to the
    /// committed numbers, over the closed list the match forces.
    #[test]
    fn winding_tags_are_injective() {
        use profile::ArcSweep;
        let all = closed_list!(ArcSweep; Ccw, Cw);
        injective_and_pinned(
            "winding",
            &all,
            winding_tag,
            &[(ArcSweep::Ccw, 6), (ArcSweep::Cw, 7)],
            |a, b| {
                matches!(
                    (a, b),
                    (ArcSweep::Ccw, ArcSweep::Ccw) | (ArcSweep::Cw, ArcSweep::Cw)
                )
            },
        );
    }

    /// An arc's side: two words, injective and pinned, over the closed
    /// list the match forces.
    #[test]
    fn side_tags_are_injective() {
        use profile::ArcSide;
        let all = closed_list!(ArcSide; Left, Right);
        injective_and_pinned(
            "side",
            &all,
            side_tag,
            &[(ArcSide::Left, 36), (ArcSide::Right, 37)],
            |a, b| {
                matches!(
                    (a, b),
                    (ArcSide::Left, ArcSide::Left) | (ArcSide::Right, ArcSide::Right)
                )
            },
        );
    }

    /// A step's target KIND: three words, injective and pinned, over
    /// the form list `profile` projects from its own declaration.
    ///
    /// `closed_list!` cannot build the list here — `Target` carries a
    /// payload, so its members are values rather than words — but
    /// `TargetKind::ALL` is that list, and the witness below is a match
    /// on the tag: a form the vocabulary gains has no arm, so it fails
    /// to compile rather than quietly going untagged, and
    /// [`injective_and_pinned`]'s pin-count clause then refuses it a
    /// missing committed number.
    #[test]
    fn target_tags_are_injective() {
        use profile::{Target, TargetKind};
        let origin = geom_core::Point2 { x: 0.0, y: 0.0 };
        let witness = |kind: TargetKind| match kind {
            TargetKind::Start => Target::Start,
            TargetKind::StartArriving => Target::StartArriving,
            TargetKind::Point => Target::Point(origin),
        };
        injective_and_pinned(
            "target",
            TargetKind::ALL,
            |kind| target_tag(&witness(kind)),
            &[
                (TargetKind::Start, 4),
                (TargetKind::StartArriving, 44),
                (TargetKind::Point, 5),
            ],
            |a, b| a == b,
        );
        for retired in tag::target::RETIRED {
            assert!(
                TargetKind::ALL
                    .iter()
                    .all(|kind| target_tag(&witness(*kind)) != *retired)
            );
        }
    }

    /// A contact class's tag — `topo::ContactClass::content_tag`, the
    /// one spelling the mate, the crossing record and the declaration
    /// write — is injective and non-zero over `ContactClass::ALL`: a
    /// class that keyed as `0` would be a class with no word.
    #[test]
    fn contact_class_tags_are_injective_and_nonzero() {
        let mut seen: Vec<(topo::ContactClass, u64)> = Vec::new();
        for class in topo::ContactClass::ALL {
            let t = class.content_tag();
            assert_ne!(t, 0, "{class:?} keys as 0");
            if let Some((other, _)) = seen.iter().find(|(_, u)| *u == t) {
                panic!("{class:?} and {other:?} share content tag {t}");
            }
            seen.push((*class, t));
        }
        assert_eq!(seen.len(), topo::ContactClass::ALL.len());
    }
}

#[cfg(test)]
mod bundle_key {
    //! A bundle's shape reaches the content key: what each constraint
    //! states, a mate's target word and its world read's arms, and a
    //! value's mate and freedom. Its numbers are slots, keyed with the
    //! slots. Dropping any of these from the feed would let two
    //! documents differing only there share a memo entry, and one be
    //! served the other's placement.

    use super::{KeyHasher, feed_bundle};
    use crate::mate::{Constraint, Coord, Mate, SlideAxis, Target, Value, WorldRead};
    use crate::node::ConstraintId;
    use crate::pose::PoseRole;

    const ON: crate::VarId = crate::VarId::new(1, 1);
    const TO: crate::VarId = crate::VarId::new(2, 2);
    const BY: crate::VarId = crate::VarId::new(3, 3);

    fn id(k: u32) -> ConstraintId {
        ConstraintId(crate::MintId::new(k, u64::from(k)))
    }

    fn mate(to: Target) -> Constraint {
        Constraint::of_mate(
            id(10),
            Mate {
                on: ON,
                to,
                class: crate::mate::ContactClass::Rest,
            },
        )
    }

    fn value(mate: ConstraintId, coord: Coord) -> Constraint {
        Constraint::of_value(
            id(11),
            Value {
                mate,
                coord,
                by: BY,
            },
        )
    }

    fn key(bundle: &[Constraint]) -> crate::eval::ContentKey {
        let mut h = KeyHasher::new();
        feed_bundle(&mut h, bundle);
        h.finish()
    }

    #[test]
    fn a_mates_target_word_and_world_arms_move_the_key() {
        let keys = [
            key(&[mate(Target::Pose(TO))]),
            key(&[mate(Target::World(WorldRead::Frame))]),
            key(&[mate(Target::World(WorldRead::through(PoseRole::Xy)))]),
            key(&[mate(Target::World(WorldRead::through(PoseRole::Xy).flip()))]),
        ];
        for (i, a) in keys.iter().enumerate() {
            for b in &keys[i + 1..] {
                assert_ne!(a, b, "each target shape keys distinctly");
            }
        }
    }

    #[test]
    fn a_values_mate_and_freedom_move_the_key() {
        let with = |v: Constraint| key(&[mate(Target::Pose(TO)), v]);
        let keys = [
            key(&[mate(Target::Pose(TO))]),
            with(value(id(10), Coord::Spin)),
            with(value(id(10), Coord::Slide(SlideAxis::X))),
            with(value(id(10), Coord::Slide(SlideAxis::Y))),
            with(value(id(12), Coord::Slide(SlideAxis::Y))),
        ];
        for (i, a) in keys.iter().enumerate() {
            for b in &keys[i + 1..] {
                assert_ne!(a, b, "each value keys distinctly");
            }
        }
    }
}

/// **A derived report's content key**: its tag, then its goldening
/// form verbatim.
///
/// One helper rather than one hasher per report, so every key in the
/// reporting lane is taken the same way over the same substrate — the
/// text that IS the report.
///
/// It lives HERE, beside [`KeyHasher`], rather than in
/// [`crate::report`] where its callers are, because
/// [`crate::mc`] needs it too, and that lane is the pure-`f64` advisory
/// estimator rather than a part of the certified reporting layer.
pub fn key_of(tag: u8, serialized: &str) -> ContentKey {
    let mut h = KeyHasher::new();
    h.write_tag(tag);
    h.write_str(serialized);
    h.finish()
}

#[cfg(test)]
#[allow(clippy::expect_used)]
mod name_feed_tests {
    //! A name's content-key feed keeps its own stack, and feeds the
    //! stream a descent level by level would.

    use super::memo::ContentKey;
    use super::{Fed, KeyHasher, SegFeed, feed_role_seg, feed_stable_name};
    use crate::names::{CapEnd, EntityKind, NameRef, Qualifier, RoleSeg, StableName};
    use crate::node::RecipeNodeId;

    fn key(name: &StableName) -> ContentKey {
        let mut h = KeyHasher::new();
        feed_stable_name(&mut h, name);
        h.finish()
    }

    /// The feed as a descent: each held name fed, whole, where its
    /// holder's segment meets it.
    fn descending(h: &mut KeyHasher, name: &StableName) {
        h.write_tag(match name.kind {
            EntityKind::Body => 1,
            EntityKind::Face => 2,
            EntityKind::Edge => 3,
            EntityKind::Vertex => 4,
        });
        h.write_id(name.node.0);
        h.write_u64(name.path.len() as u64);
        for seg in &name.path {
            let mut level = SegFeed(Vec::new());
            feed_role_seg(&mut level, seg);
            for item in level.0 {
                match item {
                    Fed::Tag(tag) => h.write_tag(tag),
                    Fed::U64(x) => h.write_u64(x),
                    Fed::Name(n) => descending(h, n),
                }
            }
        }
    }

    fn leaf(node: u64) -> StableName {
        StableName {
            kind: EntityKind::Face,
            node: RecipeNodeId::new(0, node),
            path: vec![RoleSeg::Cap(CapEnd::End)],
        }
    }

    /// `name` one level deeper, held in turn by each shape a segment
    /// holds a name in, beside payload the feed writes around it.
    fn wrap(name: StableName, level: usize) -> StableName {
        let r = NameRef::new(name.clone());
        let seg = match level % 5 {
            0 => RoleSeg::Instance { i: 3, of: r },
            1 => RoleSeg::Fragment(Qualifier::Borders(vec![leaf(7), name])),
            2 => RoleSeg::BandCross {
                edge: NameRef::new(leaf(8)),
                band: vec![name],
            },
            3 => RoleSeg::SectionEdge { face: r },
            _ => RoleSeg::Seam {
                a: r,
                b: NameRef::new(leaf(9)),
            },
        };
        StableName {
            kind: EntityKind::Edge,
            node: RecipeNodeId::new(0, level as u64 + 10),
            path: vec![
                seg,
                RoleSeg::Fragment(Qualifier::OrderAlong { rank: 1, of: 2 }),
            ],
        }
    }

    #[test]
    fn a_name_feeds_the_stream_a_descent_feeds() {
        let mut name = leaf(1);
        for level in 0..12 {
            name = wrap(name, level);
            let mut h = KeyHasher::new();
            descending(&mut h, &name);
            assert_eq!(key(&name), h.finish(), "at depth {level}");
        }
    }

    #[test]
    fn a_name_nested_past_every_stack_keys_on_the_smallest_stack() {
        test_utils::own_thread::on_the_smallest_stack(|| {
            let deep = |bottom| (0..20_000).fold(leaf(bottom), wrap);
            let (a, again, b) = (deep(1), deep(1), deep(2));
            assert_eq!(key(&a), key(&again), "one name, one key");
            assert_ne!(key(&a), key(&b), "a difference at the bottom moves the key");
        });
    }
}
