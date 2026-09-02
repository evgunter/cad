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
//! and slot it came from.

mod anchor;
pub mod measure;
mod memo;
pub(crate) mod parts;

pub use parts::PartFault;
mod schedule;
mod slots;
mod wire;

pub(crate) use wire::{SteppedOperands, stepped_rule_map, unit as unit_direction};

pub use anchor::{LoopAnchor, ProfileNaming, ProfileValue, embed_profile};
pub use memo::{ContentBits, ContentKey, KeyHasher, NamingKey};

use std::collections::BTreeMap;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};

use geom_core::{Decide, Indeterminate};
use profile::ProfileError;
use sweep::{ExtrudeError, RevolveError, SkinError};
use topo::splitting::SplitError;
use topo::transform::TransformError;
use topo::{Body, BooleanError, BooleanResultKind, ContactClass, ContactRecords};

use crate::appearance::{self, AppearanceResolution};
use crate::doc::Doc;
use crate::expr::EvalError;
use crate::names::{NameTable, NamingError};
use crate::node::{RecipeNodeId, SlotId, StableName};
use crate::program::ProfileProgram;
use geom_core::Tol;

/// The result DAG (F2 verbatim, spec D2): a deterministic order plus a
/// per-node result map, with the run's epoch, outcome, and the
/// D4-acceptance recompute counters.
#[derive(Debug)]
pub struct Evaluation<T: Decide> {
    /// This evaluation's identity token (spec D5) — the caller's
    /// stale-result discrimination hook.
    pub epoch: Epoch,
    /// Deterministic topological order of the live nodes (spec D2:
    /// a pure function of the DAG; Kahn's algorithm, tiebreak
    /// `RecipeNodeId` ascending). Always the FULL order, even when
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
    /// How many REFERENCED documents this evaluation actually
    /// evaluated across the document seam (ASM-2A D-3's sharing
    /// evidence). N instances of one part contribute 1; a memo-hit
    /// instance contributes 0, because it never asks; and a nested
    /// crossing — a part that instantiates a part — contributes to
    /// THIS count too, so the number is the whole run's seam traffic
    /// rather than one level's.
    pub part_evaluations: usize,
    /// The document's appearance store resolved against THIS
    /// evaluation's name tables (M4 PR 7): per node, entity →
    /// attributes, so a renderer/exporter consumes appearance without
    /// touching tables; unresolved entries are LOUD typed losses.
    /// Appearance never enters content keys (presentation metadata):
    /// an appearance-only edit re-resolves this field and recomputes
    /// zero nodes.
    pub appearance: AppearanceResolution,
}

impl<T: Decide> Evaluation<T> {
    /// The node's successful value, if it has one.
    pub fn value(&self, id: RecipeNodeId) -> Option<&NodeValue<T>> {
        match self.nodes.get(&id) {
            Some(NodeResult::Ok(v)) => Some(v),
            _ => None,
        }
    }

    /// The node's result as typed data — `Ok`/`Failed`/`Poisoned`
    /// distinguished, where [`Evaluation::value`] collapses the last
    /// two into `None` (LIB-DOORS F3: the curated path from an
    /// evaluation to its `NodeError`s). `None` means the id has no
    /// entry at all: never scheduled, or past a cancelation's prefix.
    pub fn result(&self, id: RecipeNodeId) -> Option<&NodeResult<T>> {
        self.nodes.get(&id)
    }

    /// The typed root cause behind a node that produced no value:
    /// `Failed` answers its own error; `Poisoned` answers the nearest
    /// failed ancestor's (one `through` hop — see
    /// [`NodeResult::Poisoned`]'s invariant). `None` for a node that
    /// succeeded or has no entry.
    pub fn node_error(&self, id: RecipeNodeId) -> Option<&NodeError> {
        match self.nodes.get(&id)? {
            NodeResult::Ok(_) => None,
            NodeResult::Failed(e) => Some(e),
            NodeResult::Poisoned { through } => match self.nodes.get(through)? {
                // Every `through` names a `Failed` entry (the poison
                // propagation writes nothing else there); answering
                // `None` on a broken invariant is fail-honest — the
                // caller sees "no root cause", not a wrong one.
                NodeResult::Failed(e) => Some(e),
                NodeResult::Ok(_) | NodeResult::Poisoned { .. } => None,
            },
        }
    }
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

/// A successful node value (spec D2): the op-appropriate payload plus
/// the reserved PR 3 / M6 slots and the node's content key.
#[derive(Debug, Clone)]
pub struct NodeValue<T: Decide> {
    /// What the node's op produced.
    pub payload: ValuePayload<T>,
    /// The node's eagerly-emitted name table (N4, PR 3): every
    /// boundary entity of the node's output bodies, `StableName ↔`
    /// (body, arena key). Rides the value, so memo reuse transfers
    /// names with geometry (the content key is the proof).
    pub name_table: Arc<NameTable>,
    /// The DECLARED CONTACT RECORDS the node's output body 0 carries
    /// (ASM-R2b D-1's contacts channel; [`crate::eval::wire::OpOut`]
    /// states the invariant). Empty for every op but instantiate — a
    /// boolean's records ride its payload, and
    /// `product::sources_of` is where the two homes
    /// reconcile. Rides the value, so memo reuse transfers
    /// declarations with the geometry they are keyed into.
    pub contacts: Arc<topo::ContactRecords>,
    /// The node's verdict log (M4 PR 4, N5): every definite predicate
    /// decision the node's op made, in decision order, recorded
    /// through the one `k_stats` funnel. Scalar-independent data —
    /// same verdicts at f64 and Interval — and the diff-engine
    /// substrate ("both evaluations' verdict logs exist"). Rides the
    /// value, so memo reuse transfers the log with the geometry it
    /// certified (same content key ⇒ same decisions, D9).
    pub verdicts: Arc<VerdictLog>,
    /// RESERVED empty slot: the solved witness assignment (M6 fills).
    pub witness: WitnessSlot,
    /// The node's input-content hash (spec D4) — the memo currency.
    pub content_key: ContentKey,
    /// The node's recursive NAMING key (issue #95 disposition 2, M4
    /// PR 5): memo reuse of the whole value requires BOTH keys to
    /// match — content certifies the geometry, naming certifies the
    /// node-id-embedding names half (see [`NamingKey`]).
    pub naming_key: NamingKey,
}

/// One evaluation's per-node verdict vector (the [`NodeValue::verdicts`]
/// payload): [`geom_core::k_stats::Verdict`]s in decision order.
pub type VerdictLog = Vec<geom_core::k_stats::Verdict>;

/// M6's solved-assignment slot, as a type stub (spec D2).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct WitnessSlot {}

/// The op-appropriate value a node evaluates to (spec D2's
/// "`Vec<Body<T>>`-shaped output (op-appropriate)", made typed per op
/// family; spec D3 fixes each node's semantics).
#[derive(Debug, Clone)]
pub enum ValuePayload<T: Decide> {
    /// A datum evaluated to geometry values (D3: frames/axes as
    /// values; directions normalized, degenerate refused).
    Datum(DatumValue<T>),
    /// A validated profile (D3: replayed from the node's program
    /// through the driver, then the profile crate's validation door)
    /// plus its program-anchor naming map ([`ProfileValue`]).
    Profile(Arc<ProfileValue<T>>),
    /// A single body: Extrude, Revolve, Transform.
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
    /// union; index `i` is the A8/N1 `Instance(i)` substrate).
    Instances(Vec<Arc<Body<T>>>),
    /// A Declare node's pairs with their contact classes, passed
    /// through as data (D3; the boolean consumes them at its
    /// `declare` input). The class travels WITH its pair from
    /// authoring to the kernel door — the one vocabulary end-to-end
    /// (SELECT-DESIGN §3d).
    Declarations(Vec<((StableName, StableName), ContactClass)>),
    /// A Mate node's ROLE in the solve (A11 rule 4; ASM-R2a D-1): a
    /// tree mate determined its child, a non-tree mate declared and
    /// solved nothing. Not body-denoting, so the product gather skips
    /// it exactly as it skips a `Declare` — which is what "an ordinary
    /// non-body root" means in code.
    Mate(crate::mate::MateRole),
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
    /// An `Assertion` node's verdict (E10). REPORT-ONLY: no op in the
    /// vocabulary takes a verdict as an operand, so this payload is
    /// consumed by reports and by nothing else.
    Assertion(crate::measure::AssertionVerdict<T>),
}

impl<T: Decide> ValuePayload<T> {
    /// The payload family, for typed operand mismatches.
    pub fn kind_name(&self) -> &'static str {
        match self {
            Self::Datum(_) => "datum",
            Self::Profile(_) => "profile",
            Self::Body(_) => "body",
            Self::Boolean(_) => "boolean",
            Self::Split { .. } => "split",
            Self::Instances(_) => "instances",
            Self::Declarations(_) => "declarations",
            Self::Mate(_) => "mate",
            Self::Measure { .. } => "measure",
            Self::Assertion(_) => "assertion",
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
        /// Declared contacts surviving into the result (the
        /// `BooleanBody` contract, spec D2).
        contacts: Arc<ContactRecords>,
    },
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

// An evaluated datum (spec D3): geometry VALUES, not kernel entities,
// which is why the type itself lives at the kernel query seat
// (`topo::query`) — it is the resolved comparand the decided distance
// predicate takes, and this layer's evaluation is what mints one. Its
// normals and axis directions are `UnitVec3`, whose constructor is
// where a degenerate, decided-zero-length vector becomes a typed
// refusal; this layer maps that refusal onto its own node error and
// invents nothing. `DatumValue` is re-exported at its historical home,
// so no consumer's path to it moved — but the surface GREW: the two
// `UnitVec3` names are new here, and they are not optional decoration.
// A consumer cannot build a datum, or read a normal back out of one,
// without naming the type that carries the invariant.
pub use topo::query::{DatumValue, UnitVec3, UnitVec3Error};

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
    /// The program-anchor derivation failed to match a canonical loop
    /// back to a program loop — an internal invariant break
    /// (validate's canonical form is an exact reindexing of its
    /// input), surfaced typed rather than panicking.
    ProfileAnchor {
        /// The canonical loop index that failed to match.
        loop_: u32,
    },
    /// The extrude op refused.
    Extrude(ExtrudeError),
    /// The revolve op refused.
    Revolve(RevolveError),
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
    /// A named sub-frontier, never a laundered catch-all (the
    /// `RestZipUnsupported` precedent).
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
    /// A direction-valued vector decided to zero length (datum
    /// normal/direction, transform rotation axis, pattern direction).
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
    /// The ambient tolerance could not form a classification band.
    Band(geom_core::BandError),
    /// A slot the wiring expected was absent from the node — a wiring
    /// bug surfaced typed (unreachable while `Node::slots` and the
    /// wire agree; no panic paths in this crate).
    MissingSlot {
        /// The absent slot.
        slot: SlotId,
    },
    /// A verb run door was handed a different operand count than the
    /// verb declares — [`NodeErrorKind::MissingSlot`]'s class: a
    /// wiring bug surfaced typed (unreachable while the per-verb
    /// correspondences and the run doors agree; no panic paths in this
    /// crate).
    VerbArity {
        /// The verb whose door refused.
        verb: verbs::VerbKind,
        /// The operand count the door was handed; the declared count
        /// is `verb.arity()`.
        given: verbs::Arity,
    },
    /// A decided predicate escalated (in-band indeterminacy).
    Escalated {
        /// The named predicate.
        predicate: &'static str,
        /// The escalation, unaltered.
        source: Indeterminate,
    },
    /// A revolve axis with a decided out-of-plane component (the
    /// kernel's `RevolveAxis` is a sketch-plane datum; an axis not in
    /// the profile's plane cannot be wired, only refused).
    AxisNotInSketchPlane {
        /// The axis datum node.
        axis: RecipeNodeId,
    },
    /// A pattern count that is not at least 1.
    NonPositiveCount {
        /// The evaluated count.
        count: i64,
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
    /// A `Declare` pair failed to resolve through the operands' name
    /// tables (F5, M4 PR 5) — the N5 typed error VERBATIM: a Declare
    /// naming a vanished/ambiguous/deleted name refuses loudly; no
    /// silent drop, no best-effort gluing.
    DeclareResolve {
        /// The resolution failure (N5's closed trio).
        error: Box<crate::resolve::ResolveError>,
    },
    /// A `Declare` name resolves in BOTH operands' tables (the same
    /// body value feeding both sides) — the declaration cannot pick a
    /// side; refused, never guessed.
    DeclareBothOperands {
        /// The ambiguous name.
        name: Box<crate::names::StableName>,
    },
    /// A `Declare` pair outside the v1 threading vocabulary
    /// (supported: cross-operand Face–Face; same-operand
    /// Vertex–Vertex and Vertex–Face).
    DeclareUnsupportedPair {
        /// The pair's entity kinds, declaration order.
        kinds: (crate::names::EntityKind, crate::names::EntityKind),
        /// Whether the names resolved in different operands.
        cross_operand: bool,
    },
    /// The boolean refused an UNDECLARED contact (F6) and the raise
    /// site identified the face pair — the refusal-menu payload
    /// (SELECT-DESIGN §3d, register R3, LIB-PYG5). `finding` is the
    /// SAME value shape the detector answers with: the pair's keys
    /// resolved to StableNames through the OPERANDS' name tables, the
    /// relation the coincidence ladder decided before refusing.
    /// Nothing is re-detected on the error path — the payload is what
    /// the raise site held. So the recourse is IN the error, and the
    /// menu has exactly two arms (the #256 ruling applied to contact,
    /// no absorb arm): declare this finding
    /// ([`crate::names::declare`] / [`crate::node::Node::Declare`] →
    /// the boolean's `declare` input), or move the geometry.
    ///
    /// Raised INSTEAD of wrapping the kernel's
    /// `BooleanError::UndeclaredCoincidence` under
    /// [`NodeErrorKind::Boolean`]; if either key fails to resolve to
    /// a name (an emitter-coverage invariant break, not an authoring
    /// state), the plain `Boolean` wrapping is preserved — the
    /// boolean's refusal is never masked by its own menu.
    UndeclaredContact {
        /// The candidate declaration, in the detector's value shape.
        finding: Box<crate::names::FlushFinding>,
        /// The refusing predicate's diagnostics, unaltered.
        diag: Indeterminate,
    },
    /// A blend node's selection name failed to resolve through the
    /// TARGET's name table (M6-5) — the same N5 typed trio as
    /// [`NodeErrorKind::DeclareResolve`], and for the same reason: a
    /// selection is a commitment, so a name that no longer resolves
    /// refuses loudly instead of silently shrinking the set.
    ///
    /// The edge-selection ladder is ONE door serving both blend nodes,
    /// so its refusals carry `verb` rather than being written twice —
    /// a chamfer's refusal says "chamfer".
    BlendSelectionResolve {
        /// Which blend the refusing node is.
        verb: sweep::blend::BlendKind,
        /// The resolution failure (N5's closed trio).
        error: Box<crate::resolve::ResolveError>,
    },
    /// A blend node's selection named something that is not an EDGE
    /// of the target (a face, a vertex, the body). The op blends
    /// edges; a mis-kinded selection is a recipe bug, refused rather
    /// than reinterpreted.
    BlendSelectionKind {
        /// Which blend the refusing node is.
        verb: sweep::blend::BlendKind,
        /// The offending name.
        name: Box<crate::names::StableName>,
        /// What it actually denotes.
        found: crate::names::EntityKind,
    },
    /// A blend node's selection is EMPTY. A blend of nothing is not
    /// the identity — it is an unfinished recipe, refused rather than
    /// passed through (the fail-loud voice: no op silently returns its
    /// input).
    BlendSelectionEmpty {
        /// Which blend the refusing node is.
        verb: sweep::blend::BlendKind,
    },
    /// A sketch node's branch selection refused (SOLVER-DESIGN W3;
    /// M4 PR 4 pins the document semantics — a per-node failure
    /// poisoning descendants only, GQ2/W5). NEVER constructed before
    /// the M6 solver: the arm exists so the solver lands as logic,
    /// not a schema change.
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
    /// itself, or an instance whose cluster the refusal left without a
    /// pose. The fault names its own subject — the pair, the residual
    /// subgroup, the failed predicate and its measured clash.
    Mate(Box<crate::mate::MateFault>),
    /// A crossing declaration on this instance no longer resolves
    /// against the pinned part (ASM-R2b D-4/D-5; A4's "does it
    /// actually fit", A13 clause 4). The seam asserted a contact at a
    /// part entity the pinned document's product does not name — the
    /// swap is refused, never accepted with the declaration quietly
    /// dropped.
    CrossingUnverified {
        /// The instance carrying the record.
        instance: RecipeNodeId,
        /// The mate whose crossing it is.
        mate: RecipeNodeId,
        /// The part-side reference that did not resolve.
        name: Box<crate::names::StableName>,
    },
    /// A `Node::Measure` reference failed to resolve against the value
    /// of the node it is read AT (E3) — the same N5 typed trio as
    /// [`NodeErrorKind::BlendSelectionResolve`], and for the same
    /// reason: a measurement's references are a commitment, so a name
    /// that stopped resolving refuses loudly rather than measuring
    /// whatever is left.
    MeasureRefResolve {
        /// The resolution failure (N5's closed trio).
        error: Box<crate::resolve::ResolveError>,
    },
    /// A `Node::Measure` reference resolved into a value that carries
    /// no bodies, or into an output body its value does not have — the
    /// naming emission and the value disagree, or the reference names
    /// a datum. Carries the interrogation layer's own words.
    MeasureRefUnreadable {
        /// The reference.
        name: Box<crate::names::StableName>,
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
    /// A `Node::Measure` carries an expression whose primitive reads a
    /// reference the node does not have. Refused at the construction
    /// and load doors; this is the evaluation backstop, so a corrupt
    /// node reaches a typed refusal rather than a panic.
    MeasureMalformed(crate::node::MeasureNodeFault),
    /// A node's PAYLOAD expression failed to evaluate — a measured
    /// expression's value leaf, or an assertion's bound.
    ///
    /// The address is the expression's position in `payload_exprs`
    /// order, because neither carrier has a slot vocabulary to name.
    /// The rendering says WHICH kind it is: calling an assertion's
    /// failed bound "value leaf 0 of the measured expression" named a
    /// thing an assertion does not have.
    PayloadExpr {
        /// Which node kind's payload this is, for the message.
        what: &'static str,
        /// The expression's index in `payload_exprs` order.
        index: usize,
        /// The expression evaluator's refusal, unaltered.
        source: EvalError,
    },
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

/// The undeclared-contact refusal as a document-layer finding
/// ([`crate::finding`]): the refusing op is the subject, the story
/// states the relation and forwards the ladder's own diagnostic, and
/// the recourse is the two-armed menu (SELECT-DESIGN §3d, the #256
/// ruling applied to contact: declare the finding or move the
/// geometry — no absorb arm).
///
/// The subject is SENTENCE-shaped ("the Boolean refused an undeclared
/// contact") rather than a bare attribution: the phrase is pinned
/// across the bindings and predates the sink, so this impl preserves
/// it verbatim rather than bending the pin to the subject style.
struct UndeclaredContactFinding<'a> {
    /// The candidate declaration, in the detector's value shape.
    finding: &'a crate::names::FlushFinding,
    /// The refusing predicate's diagnostics.
    diag: &'a Indeterminate,
}

impl crate::finding::Finding for UndeclaredContactFinding<'_> {
    fn subject(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str("the Boolean refused an undeclared contact")
    }

    fn story(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        // The MARGIN-PAYLOAD view of the diagnostic (name + numbers),
        // not its full `Display`: the funnel's generic three-arm menu
        // ends that rendering, and a contact refusal deliberately has
        // no "lower the tolerance" arm (`topo::CONTACT_RECOURSE`'s doc
        // comment) — the one menu here is this finding's recourse.
        write!(
            f,
            "a face pair of its operands is {} without a shared source or declared \
             intent; the coincidence ladder reports: {}",
            match self.finding.evidence.relation {
                topo::PlaneRelation::SameOpposite =>
                    "coincident with opposed orientations (resting contact)",
                topo::PlaneRelation::SameOriented =>
                    "coincident with the same orientation (flush walls)",
                // Never constructed on a finding; rendered honestly anyway.
                topo::PlaneRelation::Distinct => "reported coincident",
            },
            self.diag.payload()
        )
    }

    fn recourse(&self) -> &str {
        "the refusal carries the candidate declaration (the pair, by stable name, \
         with its relation); declare that finding and wire it into the Boolean's \
         declare input, or move the geometry"
    }
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
// the right to drop the payload: `UndeclaredContact` states its
// two-armed menu (F6) AND renders its diagnostic.
//
// Every payload-holding arm forwards its payload's own `Display`;
// the exception list is EMPTY — `EvalError`, `resolve::ResolveError`,
// `WitnessBifurcation` and `PlacementRuleFault` (D54's four) all
// carry one, and `PlacementRuleFault`'s is that fault set's ONE prose
// vocabulary (the edit door's rule arms forward the same impl).
// `UndeclaredContact` composes through the document layer's finding
// sink ([`crate::finding`]): subject, story, its two-armed recourse.
impl core::fmt::Display for NodeErrorKind {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Expr { slot, source } => {
                write!(
                    f,
                    "the expression at slot {slot:?} failed to evaluate: {source}"
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
                write!(
                    f,
                    "profile loop {loop_}'s program refused at replay: {error}"
                )
            }
            Self::ProfileAnchor { loop_ } => write!(
                f,
                "internal: canonical loop {loop_} failed to match back to a program loop"
            ),
            Self::Mate(fault) => write!(f, "the mate solve refused: {fault}"),
            Self::CrossingUnverified {
                instance,
                mate,
                name,
            } => write!(
                f,
                "instance {}'s seam declaration from mate {} names {} {} of the part \
                 (minted by its node {}), which the pinned part's product does not \
                 name — the crossing does not re-verify against this version of the \
                 part",
                instance.0,
                mate.0,
                name.kind.article(),
                name.kind.noun(),
                name.node.0
            ),
            Self::Extrude(e) => write!(f, "the extrude op refused: {e}"),
            Self::Revolve(e) => write!(f, "the revolve op refused: {e}"),
            Self::Split(e) => write!(f, "the split op refused: {e}"),
            Self::Blend { verb, error } => write!(f, "the {verb} op refused: {error}"),
            Self::Boolean(e) => write!(
                f,
                "the Boolean op refused its operands (undeclared coincidence is the \
                 common case: the kernel never infers that touching faces are the \
                 same face): {e}"
            ),
            Self::Transform(e) => write!(f, "the transform op refused: {e}"),
            Self::Skin(e) => write!(f, "the skin construction refused: {e}"),
            Self::Loft(e) => write!(f, "the loft assembly refused: {e}"),
            Self::CurvedSolidFrontier { what } => write!(f, "not yet buildable: {what}"),
            Self::MissingInput { input } => {
                write!(f, "input {} names no live node", input.0)
            }
            Self::ToleranceConflict {
                document_eps,
                process_eps,
            } => write!(
                f,
                "document ε {document_eps:e} conflicts with the process ε {process_eps:e} \
                 (one process, one ε)"
            ),
            Self::ParamBox { source } => write!(f, "parameter box: {source}"),
            Self::WrongOperand {
                input,
                expected,
                found,
            } => write!(
                f,
                "input {} carries kind {found}; the operand needs kind {expected}",
                input.0
            ),
            Self::EmptyOperand { input } => write!(
                f,
                "input {} is the empty value — the body ops take real bodies",
                input.0
            ),
            Self::DegenerateDirection { role } => {
                write!(f, "the {role} direction has zero length")
            }
            Self::NonFiniteDirection { role } => write!(
                f,
                "the {role} direction has no finite length — its components \
                 overflow the norm, or one of them is not a number; scale \
                 the geometry into the session's range"
            ),
            Self::Band(e) => write!(
                f,
                "the ambient tolerance could not form a classification band: {e}"
            ),
            Self::MissingSlot { slot } => {
                write!(
                    f,
                    "internal: the wiring expected slot {slot:?}, which is absent"
                )
            }
            // The sentence is single-homed at the run doors' own
            // refusal (`verbs::VerbError::Arity`); this arm re-wraps
            // the same fields and forwards its Display, so the two
            // layers cannot drift apart.
            Self::VerbArity { verb, given } => {
                let refusal = verbs::VerbError::Arity {
                    verb: *verb,
                    given: *given,
                };
                write!(f, "internal: {refusal}")
            }
            Self::Escalated { predicate, source } => write!(
                f,
                "predicate {predicate} escalated (in-band indeterminacy): {source}"
            ),
            Self::AxisNotInSketchPlane { axis } => write!(
                f,
                "revolve axis (node {}) does not lie in the profile's sketch plane",
                axis.0
            ),
            Self::NonPositiveCount { count } => {
                write!(f, "pattern count {count} is not at least 1")
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
            Self::Naming(e) => write!(f, "name emission failed: {e}"),
            Self::DeclareResolve { error } => write!(
                f,
                "a declared name failed to resolve through the operands' tables: {error}"
            ),
            // Forwards `StableName`'s `Display` rather than
            // re-spelling the kind-plus-minting-node phrase; the pin
            // builds its expectation from the impl.
            Self::DeclareBothOperands { name } => write!(
                f,
                "the declared {name} resolves in BOTH operands — the declaration cannot \
                 pick a side"
            ),
            Self::DeclareUnsupportedPair { kinds, .. } => write!(
                f,
                "declare pair ({}, {}) is outside the v1 threading vocabulary",
                kinds.0.noun(),
                kinds.1.noun()
            ),
            // The menu is what this arm owns and the payload cannot
            // spell — stated through the finding sink as the arm's
            // recourse, an ADDITION to the diagnostic rather than a
            // replacement for it: the ladder's own account of what it
            // measured rides the story, exactly as `Escalated` carries
            // the same type.
            Self::UndeclaredContact { finding, diag } => {
                crate::finding::compose(f, &UndeclaredContactFinding { finding, diag })
            }
            Self::BlendSelectionResolve { verb, error } => {
                write!(f, "a {verb} selection name failed to resolve: {error}")
            }
            Self::BlendSelectionKind { verb, name, found } => write!(
                f,
                "the {verb} selection name minted by node {} denotes {} {}, not an edge",
                name.node.0,
                found.article(),
                found.noun()
            ),
            Self::BlendSelectionEmpty { verb } => write!(
                f,
                "the {verb} selection is empty — an unfinished recipe, not the identity"
            ),
            Self::MeasureRefResolve { error } => {
                write!(f, "a measure reference failed to resolve: {error}")
            }
            Self::MeasureRefUnreadable { name, error } => write!(
                f,
                "the measure reference minted by node {} could not be read back: {error}",
                name.node.0
            ),
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
            Self::MeasureMalformed(fault) => write!(f, "{fault}"),
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
            Self::Part { doc_ref, fault } => {
                write!(f, "instantiating {doc_ref}: {fault}")
            }
        }
    }
}

/// The [`NodeError`] rendering: the node, then its kind's prose.
impl core::fmt::Display for NodeError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "node {} failed: {}", self.node.0, self.kind)
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

/// The cooperative cancel token (spec D5): checked BETWEEN nodes
/// (sequential path) or between levels (parallel path) — node
/// granularity in v1; tokens are never threaded into kernel ops.
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
/// which carries `topo::PropsQuadLane` as its supertrait — the part
/// seam gathers a referenced document's product, so evaluation owns a
/// gate policy per scalar), and `Send + Sync` for the rayon schedule.
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
{
}

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
    /// reaches the document it pins. `None` — the default — is a
    /// kernel-only evaluation, in which every instantiate node refuses
    /// typed rather than pretending a part is empty.
    pub resolver: Option<Arc<dyn crate::part::PartResolver>>,
    /// Whether profile GEOMETRY is a function of the parameters at
    /// this evaluation's scalar. Default [`ProfileLift::Pinned`] — the
    /// build path, unchanged.
    pub profile_lift: ProfileLift,
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
}

/// Where profile geometry comes from at a non-`f64` scalar.
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ProfileLift {
    /// Profile geometry is the f64 elaboration, embedded through
    /// `from_f64` — the build path, bit for bit.
    #[default]
    Pinned,
    /// Profile geometry is elaborated at THIS evaluation's scalar,
    /// GUIDED by the f64 pass's structure record: every discrete
    /// decision is consumed and re-verified here, and none is remade.
    /// A decision this scalar cannot confirm refuses the node typed
    /// rather than quietly keeping the nominal structure.
    Guided,
}

impl Default for EvalOptions {
    fn default() -> Self {
        Self {
            epoch: Epoch::mint(),
            parallel: false,
            boolean_sweep: topo::SweepStrategy::Realized,
            resolver: None,
            profile_lift: ProfileLift::Pinned,
            param_box: None,
        }
    }
}

/// Evaluates the document (spec D2–D6): a TOTAL function — every
/// failure is a per-node typed result, never a top-level error or a
/// panic.
///
/// `prior` is the memo (spec D4): nodes whose content key matches the
/// prior evaluation reuse the prior value without re-running the op;
/// only the downstream cone of changed keys re-evaluates.
///
/// `cancel` is checked between nodes (sequential) or between levels
/// (parallel) — spec D5's cooperative yield points at node
/// granularity; a canceled run returns the completed prefix with
/// [`EvalOutcome::Canceled`].
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
    evaluate_at_descent(doc, prior, cancel, opts, &[], tol)
}

/// An instantiated document's own evaluation (ASM-2A D-3), one level
/// deeper than its instantiator's. `chain` is the descent — every
/// reference this run was reached through — which is what makes a
/// cycle decidable at the seam. No prior: a referenced document is
/// resolved fresh, and the memo that keeps THAT from costing anything
/// is the part cache, one layer up.
pub(crate) fn evaluate_nested<T>(
    doc: &Doc<ProfileProgram>,
    cancel: &CancelToken,
    opts: &EvalOptions,
    chain: &[crate::ident::DocRef],
    tol: Tol,
) -> Evaluation<T>
where
    T: EvalScalar,
{
    evaluate_at_descent(doc, None, cancel, opts, chain, tol)
}

fn evaluate_at_descent<T>(
    doc: &Doc<ProfileProgram>,
    prior: Option<&Evaluation<T>>,
    cancel: &CancelToken,
    opts: &EvalOptions,
    chain: &[crate::ident::DocRef],
    tol: Tol,
) -> Evaluation<T>
where
    T: EvalScalar,
{
    let sched = schedule::schedule(doc);
    // D4 door (M4 PR 6): the recorded ε must BE the committed process
    // ε — otherwise every predicate below would silently decide at
    // the wrong tolerance. Refuse loudly, per node, staying total.
    let process_eps = tol.eps();
    if doc.epsilon().to_bits() != process_eps.to_bits() {
        return refuse_tolerance_conflict(doc, sched, opts, process_eps);
    }
    // The lane environment, built ONCE and shared by every reader
    // below (slot evaluation, the lift's second pass, the two profile
    // ladders): one environment per evaluation is what makes "this run
    // evaluated over that box" a fact about the run rather than about
    // each call site.
    let env = match opts.param_box.as_deref() {
        None => doc.param_env::<T>(),
        Some(b) => match crate::analysis::param_env_over::<T, _>(doc, b) {
            Ok(env) => env,
            Err(source) => return refuse_param_box(doc, sched, opts, source),
        },
    };
    let parts = parts::PartCache::<T>::new(
        opts.resolver.as_ref(),
        chain,
        opts.boolean_sweep,
        opts.profile_lift,
        tol,
    );
    // The mate solve is a WHOLE-DOCUMENT computation over recipe data
    // (A11): one spanning tree per cluster, folded once, read by every
    // instance and every mate below. Running it here rather than per
    // node is not an optimization — a per-node solve would be a second
    // answer to "where does this cluster sit".
    let poses = crate::mate::solve_document(doc, tol);
    let op_env = wire::OpEnv {
        boolean_sweep: opts.boolean_sweep,
        parts: &parts,
        poses: &poses,
        lane: wire::LaneEnv {
            lift: opts.profile_lift,
            params: &env,
        },
    };
    let mut nodes: BTreeMap<RecipeNodeId, NodeResult<T>> = BTreeMap::new();
    let mut recomputed = 0usize;
    let mut reused = 0usize;
    let mut outcome = EvalOutcome::Completed;

    if opts.parallel {
        for level in &sched.levels {
            if cancel.is_canceled() {
                outcome = EvalOutcome::Canceled;
                break;
            }
            // D6 / D9-addendum idiom 1: an INDEXED parallel map into
            // per-node slots — results land keyed by node id, the
            // combination is positional (one map entry per node),
            // never arithmetic, so the schedule cannot leak into bits.
            use rayon::prelude::*;
            let results: Vec<(RecipeNodeId, NodeStep<T>)> = level
                .par_iter()
                .map(|&id| (id, eval_node(doc, &env, id, &nodes, prior, &op_env, tol)))
                .collect();
            for (id, step) in results {
                bookkeep(&step, &mut recomputed, &mut reused);
                nodes.insert(id, step.result);
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
                }),
            );
        }
    }

    // Appearance resolution (M4 PR 7): a total post-pass over the
    // result DAG — canceled prefixes resolve what completed and report
    // the rest as typed TargetNotEvaluated losses.
    let states: BTreeMap<RecipeNodeId, appearance::NodeState<'_>> = nodes
        .iter()
        .map(|(&id, res)| {
            let s = match res {
                NodeResult::Ok(v) => appearance::NodeState::Ok(&v.name_table),
                NodeResult::Failed(_) => appearance::NodeState::Failed,
                NodeResult::Poisoned { through } => {
                    appearance::NodeState::Poisoned { through: *through }
                }
            };
            (id, s)
        })
        .collect();
    let resolved_appearance =
        appearance::resolve(doc.appearance(), |id| doc.node(id).is_some(), &states);
    drop(states);

    Evaluation {
        epoch: opts.epoch,
        order,
        nodes,
        outcome,
        recomputed,
        reused,
        part_evaluations: parts.evaluations(),
        appearance: resolved_appearance,
    }
}

/// The all-nodes ToleranceConflict refusal (spec D4 door).
fn refuse_tolerance_conflict<T>(
    doc: &Doc<ProfileProgram>,
    sched: schedule::Schedule,
    opts: &EvalOptions,
    process_eps: f64,
) -> Evaluation<T>
where
    T: Decide + ContentBits + geom_core::Bounds + Send + Sync,
{
    let document_eps = doc.epsilon();
    refuse_every_node(doc, sched, opts, move || NodeErrorKind::ToleranceConflict {
        document_eps,
        process_eps,
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
    source: crate::analysis::ParamBoxError,
) -> Evaluation<T>
where
    T: Decide + ContentBits + geom_core::Bounds + Send + Sync,
{
    refuse_every_node(doc, sched, opts, move || NodeErrorKind::ParamBox {
        source: source.clone(),
    })
}

/// A TOTAL evaluation in which every live node fails typed with the
/// same document-level cause, and the appearance store resolves against
/// all-failed states (typed losses, nothing silent).
fn refuse_every_node<T>(
    doc: &Doc<ProfileProgram>,
    sched: schedule::Schedule,
    opts: &EvalOptions,
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
                }),
            )
        })
        .collect();
    let states: BTreeMap<RecipeNodeId, appearance::NodeState<'_>> = nodes
        .keys()
        .map(|&id| (id, appearance::NodeState::Failed))
        .collect();
    let resolved_appearance =
        appearance::resolve(doc.appearance(), |id| doc.node(id).is_some(), &states);
    drop(states);
    Evaluation {
        epoch: opts.epoch,
        order,
        nodes,
        outcome: EvalOutcome::Completed,
        recomputed: 0,
        reused: 0,
        part_evaluations: 0,
        appearance: resolved_appearance,
    }
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
    env: &crate::expr::ParamEnv<T>,
    id: RecipeNodeId,
    results: &BTreeMap<RecipeNodeId, NodeResult<T>>,
    prior: Option<&Evaluation<T>>,
    op_env: &wire::OpEnv<'_, T>,
    tol: Tol,
) -> NodeStep<T>
where
    T: EvalScalar,
{
    let fail = |kind: NodeErrorKind| NodeStep {
        result: NodeResult::Failed(NodeError { node: id, kind }),
        reused: false,
    };
    let Some(node) = doc.node(id) else {
        // Unreachable: the schedule only lists live nodes.
        return fail(NodeErrorKind::MissingInput { input: id });
    };

    // Poison propagation (spec D2, GQ2): first blocking input in the
    // node's deterministic input order; `through` always names a
    // FAILED node (propagated through poisoned intermediaries).
    let mut upstream_keys: Vec<ContentKey> = Vec::new();
    let mut upstream_naming: Vec<(RecipeNodeId, NamingKey)> = Vec::new();
    for input in node.inputs() {
        match results.get(&input) {
            None => return fail(NodeErrorKind::MissingInput { input }),
            Some(NodeResult::Failed(_)) => {
                return NodeStep {
                    result: NodeResult::Poisoned { through: input },
                    reused: false,
                };
            }
            Some(NodeResult::Poisoned { through }) => {
                return NodeStep {
                    result: NodeResult::Poisoned { through: *through },
                    reused: false,
                };
            }
            Some(NodeResult::Ok(v)) => {
                upstream_keys.push(v.content_key);
                upstream_naming.push((input, v.naming_key));
            }
        }
    }

    // Slot evaluation — the (node, slot) context door (PR 1's banked
    // NonFiniteResult obligation lands here).
    let slot_values = match slots::eval_slots(node, env) {
        Ok(v) => v,
        Err((slot, source)) => return fail(NodeErrorKind::Expr { slot, source }),
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
        crate::node::Node::Profile(program) => match program.resolve(&doc.param_env::<f64>()) {
            Ok(r) => Some(r),
            Err((slot, source)) => return fail(NodeErrorKind::Expr { slot, source }),
        },
        _ => None,
    };

    // The profile F64 PRECOMPUTE (replay + f64 validation + the naming
    // anchor) also lives here, OUTSIDE the verdict-log bracket: it is
    // C6 structure selection — the successor of the stored f64 bits —
    // not a per-lane op decision, so the node's logged verdicts stay
    // exactly the lane validation the op runs (the v1 logged surface).
    let profile_pre = match (node, &resolved_program) {
        (crate::node::Node::Profile(program), Some(resolved)) => {
            match wire::prepare_profile(program, resolved, tol) {
                Ok(pre) => Some(pre),
                Err(kind) => return fail(kind),
            }
        }
        _ => None,
    };

    // PAYLOAD EXPRESSIONS (E3/E10): the measurement vocabulary carries
    // `Expr`s that are not slots — a measured expression's value
    // leaves, and an assertion's bound. They are still ordinary
    // document expressions and are evaluated exactly once here, under
    // the discipline `slots` states and for the same reason: these
    // values feed BOTH the content key and the op, so a parameter edit
    // under a bound moves the key rather than serving a stale memo.
    let payload_values = match crate::node::payload_exprs(node) {
        None => None,
        Some(exprs) => {
            let mut values = Vec::with_capacity(exprs.len());
            for leaf in exprs {
                match crate::expr::eval(leaf, env) {
                    Ok(v) => values.push(v),
                    Err(source) => {
                        let what = match node {
                            crate::node::Node::Assertion { .. } => "the assertion's bound,",
                            _ => "value leaf of the measured expression,",
                        };
                        return fail(NodeErrorKind::PayloadExpr {
                            what,
                            index: values.len(),
                            source,
                        });
                    }
                }
            }
            Some(values)
        }
    };

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
                Err((slot, source)) => return fail(NodeErrorKind::Expr { slot, source }),
            }
        }
        _ => None,
    };

    let content_key = content_key(
        node,
        &slot_values,
        payload_values.as_deref(),
        resolved_program.as_deref(),
        lane_program.as_deref(),
        &upstream_keys,
        doc.witness(id),
        op_env.poses.placement(doc, id).ok(),
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
    if let Some(NodeResult::Ok(v)) = prior.and_then(|p| p.nodes.get(&id))
        && v.content_key == content_key
        && v.naming_key == naming_key
    {
        return NodeStep {
            result: NodeResult::Ok(v.clone()),
            reused: true,
        };
    }

    // Verdict-log bracket (M4 PR 4): every definite decision the op
    // makes — kernel predicates and N2 discriminators alike — lands in
    // this node's log through the one `k_stats` funnel. The bracket is
    // per-node and thread-confined (kernel ops are single-threaded;
    // idiom-1 parallelism runs whole nodes on one worker each), so
    // logs never interleave across nodes.
    geom_core::k_stats::start_verdict_log();
    let op = wire::run_op(
        id,
        node,
        doc,
        results,
        &slot_values,
        payload_values.as_deref(),
        profile_pre.as_ref(),
        op_env,
        tol,
    );
    let verdicts = geom_core::k_stats::take_verdict_log();
    match op {
        Ok(out) => NodeStep {
            result: NodeResult::Ok(NodeValue {
                payload: out.payload,
                name_table: out.names,
                contacts: out.contacts,
                verdicts: Arc::new(verdicts),
                witness: WitnessSlot {},
                content_key,
                naming_key,
            }),
            reused: false,
        },
        Err(kind) => fail(kind),
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
/// **The numbers are the ones that were already here** and they do not
/// move: they are the tags [`content_key`]'s match wrote inline before
/// the vocabulary had a home, and `verb_content_tags_are_the_committed
/// _numbers` pins each of them. Keys are process-internal and never
/// persist, so a tag costs a memo invalidation and no schema — but an
/// EXISTING tag must never be reused for a new meaning, which is the
/// rule this whole tag space runs on.
///
/// It takes the payload-free [`verbs::VerbKind`] rather than a
/// `&Verb<T>` because a content key is computed BEFORE the node's
/// selection has resolved to arena keys or its slot to a scalar: at
/// this point in evaluation there is no verb value to match on, only
/// the verb's name — which is exactly what the tag is a function of.
/// The boolean's NAME carries its op (`VerbKind::Boolean(op)`): union,
/// intersect and subtract are three operations sharing one payload
/// shape, and this tag space has kept them apart since v1, so the
/// three rows below are three names — not payload leaking into the
/// tag, and not a new structural word in the key (the op feeds nothing
/// elsewhere, exactly as before).
fn verb_content_tag(kind: verbs::VerbKind) -> u8 {
    match kind {
        verbs::VerbKind::Fillet => 17,
        verbs::VerbKind::Chamfer => 24,
        verbs::VerbKind::Boolean(topo::BooleanOp::Union) => 8,
        verbs::VerbKind::Boolean(topo::BooleanOp::Intersect) => 9,
        verbs::VerbKind::Boolean(topo::BooleanOp::Subtract) => 10,
    }
}

/// The content key (spec D4): op kind, structural params, evaluated
/// expression values AS BITS, upstream keys — plus the ambient
/// tolerance (ε, k), which parameterizes every decision the kernel
/// ops make, and a leading format version. M4 PR 4: the node's
/// recorded witness datum (if any) is an input too — `solution
/// (constraints, params, witness)` is pure in exactly those three
/// (W5), so a witness change must move the key (v1 evaluation does
/// not read the witness, so the recompute reproduces identical
/// results — W4's "semantically invisible", honestly re-derived
/// rather than assumed).
// The two arguments past the seventh are both INPUTS to the key,
// which is the one thing a content key is allowed to grow: the lift's
// lane-resolved program, and the measurement vocabulary's
// payload-expression values — the same input the slot values are,
// arriving by a second route because a `MeasureExpr` is not a slot.
#[allow(clippy::too_many_arguments)]
fn content_key<T>(
    node: &crate::node::Node<ProfileProgram>,
    slot_values: &slots::SlotValues<T>,
    payload_values: Option<&[T]>,
    resolved_program: Option<&[Vec<profile::Step<f64>>]>,
    lane_program: Option<&[Vec<profile::Step<T>>]>,
    upstream_keys: &[ContentKey],
    witness: Option<&crate::witness::WitnessDatum>,
    placement: Option<crate::placement::Frame>,
    tol: Tol,
) -> ContentKey
where
    T: Decide + ContentBits,
{
    use crate::node::{Datum, Node, PatternKind};
    let mut h = KeyHasher::new();
    // Key format v2 (M4 PR 6 spec D5, banking PR 4 review Finding 8):
    // the tag versions the key's INPUT-SET SHAPE, which grew since v1
    // — witness datum (schema + bytes, PR 4/W5) and the naming-key
    // context now feed the hash. Keys remain process-internal (spec
    // D3: never persisted), so no migration machinery exists or is
    // wanted; any future persistence of keys inherits this honest
    // version. Bump AGAIN whenever the hashed input set changes.
    //
    // Key format v3 (M10-P PP5): the input set grew again — a profile
    // node's LANE-resolved program feeds the key when the lift's
    // second pass runs, so that a `Dual` seed or an interval box on a
    // profile dimension moves the key instead of aliasing the nominal
    // evaluation's memo entry.
    //
    // THE BUMP'S RADIUS IS EVERY NODE, not the profile nodes whose
    // input set actually grew: the tag is written once here, before the
    // node discriminant, so every content key in the document moves —
    // and naming keys are derived from content keys, so those move too.
    // That is the intended cost of a format version and not an
    // oversight. It means one thing for a reader: no memo entry from a
    // pre-bump process is reused by a post-bump one, anywhere, which is
    // exactly what a format version is for. Keys are process-internal
    // and never persisted (spec D3), so nothing on disk is affected.
    // M10-2 GREW THE INPUT SET AGAIN AND DID NOT BUMP, which
    // contradicts the rule stated above unless the exception is
    // written down — so here it is.
    //
    // The measurement vocabulary added the payload-expression channel
    // (a measured expression's value leaves, an assertion's bound).
    // The channel writes NOTHING for a node that carries none:
    // `node::payload_exprs` answers `None`, not an empty vector, and
    // the `if let Some` below skips the length word entirely. So every
    // key a document without measures can produce is byte-identical to
    // the pre-M10-2 one.
    //
    // Bumping would have moved all of them — the radius paragraph
    // above says every node — and broken the very claim the unit is
    // measured against: a document that uses no measure is untouched.
    // The rule the comment states is about input sets that could
    // COLLIDE or DRIFT two different nodes onto one key; a channel no
    // existing node reaches is strictly additive and is the case the
    // rule does not cover. A future channel that any existing node
    // writes into gets the bump.
    h.write_tag(3);
    let tol = tol.get();
    h.write_f64_bits(tol.eps);
    h.write_f64_bits(tol.k);
    // NODE-TAG-SPACE BEGIN — the sentinel `node_tag_space_is_injective`
    // reads. Every number between here and the END sentinel is a tag in
    // ONE space, whether it is written inline or comes back from
    // `verb_content_tag`; do not move a tag out of these lines without
    // teaching that test where it went.
    let tag = match node {
        Node::Datum(Datum::Plane { .. }) => 1,
        Node::Datum(Datum::Axis { .. }) => 2,
        Node::Datum(Datum::Point { .. }) => 3,
        Node::Profile(_) => 4,
        Node::Extrude { .. } => 5,
        Node::Revolve { .. } => 6,
        Node::Split { .. } => 7,
        // The numbers are not written here: a migrated verb's tag is a
        // function of the KERNEL's name for it, and the boolean's name
        // carries its op (`VerbKind::Boolean(op)` — the three
        // regularized ops are three names in the vocabulary).
        Node::Boolean { op, .. } => verb_content_tag(verbs::VerbKind::Boolean(*op)),
        Node::Transform { .. } => 11,
        Node::Pattern { kind, .. } => match kind {
            PatternKind::Linear { .. } => 12,
            PatternKind::Circular { .. } => 13,
            // Verified next-free at LIB-PLACEDUNION (the tag-29
            // lesson: an EXISTING tag never gains a new meaning).
            PatternKind::Explicit(_) => 19,
        },
        Node::Declare { .. } => 14,
        // M5 PR 10: new tags append — the key's tag space is
        // process-internal (never persisted), so growth is free, but
        // an EXISTING tag must never be reused for a new meaning.
        Node::Loft { .. } => 15,
        Node::Sweep { .. } => 16,
        // M5 PR 12. The number is not written here: a migrated verb's
        // tag is a function of the KERNEL's name for it, so it comes
        // out of `verb_content_tag`.
        Node::Fillet { .. } => verb_content_tag(verbs::VerbKind::Fillet),
        // ASM-2A.
        Node::InstantiatePart { .. } => 18,
        // LIB-PLACEDUNION (19 is `Pattern`'s explicit rule, above):
        // the group boolean, one tag per placement rule, so a rule
        // change moves the key even when every slot value holds.
        Node::PlacedUnion { kind, .. } => match kind {
            PatternKind::Linear { .. } => 20,
            PatternKind::Circular { .. } => 21,
            PatternKind::Explicit(_) => 22,
        },
        // ASM-R2a. Tags APPEND — an existing one must never be reused
        // for a new meaning (M5 PR 10's rule), so the mate takes the
        // next free number rather than the one its unit first wrote.
        Node::Mate { .. } => 23,
        // LIB-G16. Appended, never a reused tag: a chamfer and a
        // fillet of the same size on the same edges are different
        // geometry, so they must not share a key. Same home as the
        // fillet's, for the same reason.
        Node::Chamfer { .. } => verb_content_tag(verbs::VerbKind::Chamfer),
        // M10-2. Tags APPEND — an existing one must never be reused
        // for a new meaning. Both of these claimed 24 on their own
        // branches; LIB-G16 merged first, so they take the next free
        // numbers rather than the ones this unit first wrote. Keys are
        // process-internal, so the renumber costs nothing on disk.
        Node::Measure { .. } => 25,
        Node::Assertion { .. } => 26,
    };
    // NODE-TAG-SPACE END
    h.write_tag(tag);
    // Structural payloads beyond the tag — everything a node carries
    // that its SLOTS do not express, and that two nodes of one tag can
    // differ in. The match is EXHAUSTIVE on purpose: a future variant
    // whose recipe payload lives outside its slots must be fed here or
    // the compile breaks. It cannot default to "tag plus slots" and
    // hash identically to a node that differs in that payload — a memo
    // hit would then serve another node's geometry, which is not
    // hypothetical (see S4: `Step::AtToward`'s content-key tag collided
    // with `ArcContinue`'s and was caught by a reviewer, not a type).
    // The tag match above is exhaustive for the same reason; the two
    // halves of one key had different answers to that until now.
    match node {
        Node::Profile(program) => {
            // LIB-SWITCH §4e: the program's structural payload feeds
            // as (tag, payload) tokens — plane placement floats, then
            // per loop a LoopStart tag and per RESOLVED step the verb
            // tag + structural tags + the resolved-at-f64 bit pattern
            // of each continuous arg (the same resolved-value
            // convention node slots use). Derived segment floats LEFT
            // the key (V3); display units never enter it (D7). Any
            // edit that can change segments changes the key: structure
            // via tags, Exprs and params via resolved bits, ε above.
            for bits in crate::program::plane_key_bits(&program.plane) {
                h.write_tag(2);
                h.write_u64(bits);
            }
            // Present by eval_node's stage order (profiles resolve
            // before keying); written defensively — no panic paths in
            // this crate — and the write_tag(0) marker keeps an
            // (impossible) absent-program key distinct from any real
            // program's key rather than aliasing an empty one.
            match resolved_program {
                Some(resolved) => {
                    for steps in resolved {
                        h.write_tag(1); // LoopStart
                        for step in steps {
                            feed_step(&mut h, step);
                        }
                    }
                }
                None => h.write_tag(0),
            }
            // The f64 stream above IS the structure identity and stays
            // in the key unconditionally, lane-independent as ever.
            // What follows is the lane's own geometry, and only when
            // the second pass computed any: tag 41 opens it, so a
            // pinned evaluation's key is the v3 tag away from what it
            // has always been and cannot alias a lifted one's.
            if let Some(lane) = lane_program {
                h.write_tag(41);
                for steps in lane {
                    h.write_tag(1); // LoopStart
                    for step in steps {
                        feed_lane_step(&mut h, step);
                    }
                }
            }
        }
        // ASM-2A D-1/D-2: WHICH document (id + pin — the pin IS the
        // referenced content, so nothing about the part needs hashing
        // here) and WHERE its cluster sits. The placement is document
        // data, not node data, which is exactly why it must feed the
        // key: a `SetPlacement` moves this node's value and nothing
        // else about the node changes. The INTERFACE RECORD feeds the
        // key too (ASM-R2b D-4 discharging ASM-4's hook obligation):
        // it is inhabited now, it is on-wire data, and evaluation
        // re-verifies the crossings it holds — so a crossing edit
        // must move this node's value rather than hit the memo on the
        // pre-edit answer.
        Node::InstantiatePart {
            doc_ref, interface, ..
        } => {
            h.write_u64((doc_ref.id.0 >> 64) as u64);
            h.write_u64(doc_ref.id.0 as u64);
            for chunk in doc_ref.pin.0.chunks(8) {
                let mut byte8 = [0u8; 8];
                byte8[..chunk.len()].copy_from_slice(chunk);
                h.write_u64(u64::from_be_bytes(byte8));
            }
            // The SOLVED placement (ASM-R2a D-5): a mate edit that
            // moves this instance's pose moves its key, and a cluster
            // that refuses to solve keys DISTINCTLY from any pose —
            // otherwise a repaired document could hit the memo on a
            // stale success.
            match placement {
                Some(frame) => {
                    h.write_tag(1);
                    for x in frame
                        .columns
                        .iter()
                        .flatten()
                        .chain(frame.translation.iter())
                    {
                        h.write_f64_bits(*x);
                    }
                }
                None => h.write_tag(0),
            }
            h.write_u64(interface.crossings.len() as u64);
            for crossing in &interface.crossings {
                let crate::node::InterfaceCrossing::Mate {
                    mate,
                    class,
                    outer,
                    inner,
                } = crossing;
                h.write_u64(mate.0);
                h.write_tag(contact_class_tag(*class));
                feed_stable_name(&mut h, outer);
                feed_stable_name(&mut h, inner);
            }
        }
        // A mate's own key is its references, its class and its
        // alignment: the recipe payload that decides what it says.
        Node::Mate {
            a,
            b,
            class,
            alignment,
        } => {
            feed_stable_name(&mut h, a);
            feed_stable_name(&mut h, b);
            h.write_tag(contact_class_tag(*class));
            feed_alignment(&mut h, alignment);
        }
        Node::Declare { pairs } => {
            h.write_u64(pairs.len() as u64);
            for ((a, b), class) in pairs {
                feed_stable_name(&mut h, a);
                feed_stable_name(&mut h, b);
                // The CLASS is part of the node's identity: two
                // declarations of the same pair under different
                // classes are different nodes, and a memo keyed
                // without it would serve a `Rest` answer to a
                // `Tangent` question. Keys are process-internal, so
                // this costs a one-time memo invalidation and no
                // schema.
                h.write_u64(class.content_tag());
            }
        }
        // LIB-PLACEDUNION: an `Explicit` rule's FRAMES are recipe
        // payload, not slots (the list is the count, D8-structural),
        // so they must feed the key by hand or an edited placement
        // would recompute nothing. Bits, in placement order (D9) —
        // `0.0` and `-0.0` are different placements to this key,
        // exactly as they are to `bit_eq`.
        Node::Pattern { kind, .. } | Node::PlacedUnion { kind, .. } => {
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
        Node::Fillet { selection, .. } | Node::Chamfer { selection, .. } => {
            h.write_u64(selection.len() as u64);
            for n in selection {
                feed_stable_name(&mut h, n);
            }
        }
        // A measure's REFERENCES and its measured EXPRESSION are both
        // recipe payload rather than slots: two measures with the same
        // tag differ in exactly those. The references feed in argument
        // order (the order is meaning here, not a click sequence), and
        // the expression feeds structurally — its shape, its primitive
        // indices, and its value leaves' literal BITS, so a `-0.0`
        // bound inside a measured expression is not the same node as a
        // `0.0` one.
        Node::Measure { expr, refs } => {
            h.write_u64(refs.len() as u64);
            for r in refs {
                // BOTH halves: the name says which entity, the reading
                // site says which value its carrier comes out of, and
                // two measures differing only in the site are two
                // different measurements (that is the whole point of
                // the site — one reads placed geometry, the other
                // authored). The site is a node ID, which content keys
                // otherwise exclude (D8); it is fed here because it is
                // RECIPE PAYLOAD selecting a reading, not a Merkle link
                // to an input — the input's own key is fed separately
                // through `upstream_keys`.
                h.write_u64(r.at.0);
                feed_stable_name(&mut h, &r.name);
            }
            feed_measure_expr(&mut h, expr);
        }
        // The DIRECTION is payload. The bound is NOT a slot — it is a
        // payload expression, and its evaluated value is fed with the
        // others below; the measure is an input edge, so its own key
        // carries it.
        Node::Assertion { dir, .. } => h.write_tag(match dir {
            crate::measure::AssertionDir::AtLeast => 1,
            crate::measure::AssertionDir::AtMost => 2,
        }),
        // Fully expressed by tag plus slots: their whole recipe payload
        // is either an input edge (excluded from the key by design — the
        // inputs' own keys carry it) or a slot expression, fed below.
        Node::Datum(_)
        | Node::Extrude { .. }
        | Node::Revolve { .. }
        | Node::Loft { .. }
        | Node::Sweep { .. }
        | Node::Split { .. }
        | Node::Boolean { .. }
        | Node::Transform { .. } => {}
    }
    // Evaluated slot values, in the node's deterministic slot order.
    for (i, (_slot, val)) in slot_values.iter().enumerate() {
        h.write_u64(i as u64);
        match val {
            slots::SlotVal::Scalar(v) => v.feed(&mut h),
            slots::SlotVal::Count(n) => h.write_i64(*n),
        }
    }
    // The node's evaluated PAYLOAD expressions, in `payload_exprs`
    // order — the same resolved-value convention the slots above
    // follow, so a parameter under a measured bound moves the key
    // exactly as one under an extrude's distance does. Absent (not
    // empty) for every node kind that carries none, so no existing
    // document's key moves by a byte.
    if let Some(values) = payload_values {
        h.write_u64(values.len() as u64);
        for v in values {
            v.feed(&mut h);
        }
    }
    // Upstream identity, by CONTENT (never by id — ids are stable
    // labels, content keys are the Merkle links).
    h.write_u64(upstream_keys.len() as u64);
    for k in upstream_keys {
        h.write_key(*k);
    }
    // The recorded witness datum (M4 PR 4). The tag is fed in BOTH
    // cases — a datum's absence is content too (recording, then
    // clearing, a witness must not alias the never-recorded key).
    match witness {
        None => h.write_tag(0),
        Some(w) => {
            h.write_tag(1);
            h.write_u64(u64::from(w.schema));
            h.write_bytes(&w.bytes);
        }
    }
    h.finish()
}

/// The recursive naming key (issue #95 disposition 2; see
/// [`NamingKey`]): the node's own content key plus every input's
/// (id, naming key) pair, in input order — ids INCLUDED, which is
/// exactly what the content key omits by design (D8).
fn naming_key(content: ContentKey, upstream: &[(RecipeNodeId, NamingKey)]) -> NamingKey {
    let mut h = KeyHasher::new();
    h.write_tag(3); // naming-key domain, format v1
    h.write_key(content);
    h.write_u64(upstream.len() as u64);
    for (id, nk) in upstream {
        h.write_u64(id.0);
        h.write_u64(nk.0 as u64);
        h.write_u64((nk.0 >> 64) as u64);
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
        // 42 rather than 18: the low numbers were assigned in table
        // order when this map was written, the space is APPEND-ONLY
        // (retired numbers stay dead, above), and 41 — `Cusp`, the
        // previous append — was the high-water mark. Renumbering to
        // close the gap would re-key every program that uses the verbs
        // in between, which is the one thing this map must never do.
        V::ContinueTo => 42,
        V::ArcTo => 18,
        V::TangentArcTo => 21,
        V::Fillet => 22,
        V::FarEndTo => 23,
        V::CloseTo => 24,
        V::Circle => 26,
        V::CircleSplit => 27,
        V::ArcContinue => 28,
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
    (29, "AtToward"),
];

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
        h.write_tag(2);
        h.write_u64(v.to_bits());
    }
    fn target(h: &mut KeyHasher, t: &Target<f64>) {
        match t {
            Target::Start => h.write_tag(4),
            Target::Point(p) => {
                h.write_tag(5);
                f(h, p.x);
                f(h, p.y);
            }
        }
    }
    fn winding(h: &mut KeyHasher, w: ArcSweep) {
        h.write_tag(match w {
            ArcSweep::Ccw => 6,
            ArcSweep::Cw => 7,
        });
    }
    // The arc-spec and structural tags. They share one number space
    // with the verb tags `verb_tag` allocates, and the same
    // append-only rule: 30–40 were appended by the §2c re-spell.
    fn side(h: &mut KeyHasher, s: ArcSide) {
        h.write_tag(match s {
            ArcSide::Left => 36,
            ArcSide::Right => 37,
        });
    }
    fn spec(h: &mut KeyHasher, s: &ArcData<f64>) {
        match s {
            ArcData::Radius { r, side: sd } => {
                h.write_tag(30);
                f(h, *r);
                side(h, *sd);
            }
            ArcData::Bulge { target: t, b } => {
                h.write_tag(31);
                target(h, t);
                f(h, *b);
            }
            ArcData::Via { q, target: t } => {
                h.write_tag(32);
                f(h, q.x);
                f(h, q.y);
                target(h, t);
            }
            ArcData::Center {
                c,
                winding: w,
                target: t,
            } => {
                h.write_tag(33);
                f(h, c.x);
                f(h, c.y);
                winding(h, *w);
                target(h, t);
            }
            ArcData::Sweep { r, side: sd, angle } => {
                h.write_tag(34);
                f(h, *r);
                side(h, *sd);
                f(h, *angle);
            }
            ArcData::ArcLen { r, side: sd, len } => {
                h.write_tag(35);
                f(h, *r);
                side(h, *sd);
                f(h, *len);
            }
        }
    }
    h.write_tag(verb_tag(step.verb()));
    match step {
        Step::At(p) | Step::ArcContinue(p) | Step::FarEndTo(p) => {
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
            // Structural int under its own tag (3) — the (tag,
            // payload) discipline holds for every token, review
            // NOTE-3.
            h.write_tag(3);
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
        h.write_tag(42);
        v.feed(h);
    }
    fn pt<T: ContentBits>(h: &mut KeyHasher, p: &geom_core::Point2<T>) {
        f(h, &p.x);
        f(h, &p.y);
    }
    fn target<T: ContentBits>(h: &mut KeyHasher, t: &Target<T>) {
        match t {
            // The Start/Point distinction is structural and rides the
            // f64 stream; only a Point's coordinates are lane data.
            Target::Start => {}
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
        Step::At(p) | Step::ArcContinue(p) | Step::FarEndTo(p) => pt(h, p),
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

/// Feeds a mate's alignment datum: the structural choices as tags, the
/// authored coordinates as bits — the same (tag, payload) convention
/// every other structural payload uses here.
///
/// The primitive's own lengths come from
/// [`crate::mate::MatePrimitive::authored_lengths`], which is where
/// they are enumerated for the lever arm and the finiteness admission
/// as well. This is the reader where leaving one out is WORST: the tag
/// match below breaks on a new variant, so the variant cannot arrive
/// untagged — but a length it carries would silently not be hashed,
/// and two documents differing only in that length would share a memo
/// entry. Reading the list rather than naming a variant is what makes
/// the tag and its payload arrive together.
fn feed_alignment(h: &mut KeyHasher, a: &crate::mate::Alignment) {
    use crate::mate::{AxisSense, MatePrimitive};
    h.write_tag(match a.primitive {
        MatePrimitive::FrameCoincidence => 1,
        MatePrimitive::Coaxial => 2,
        MatePrimitive::PlanarRest { .. } => 3,
        MatePrimitive::Clocking => 4,
    });
    for length in a.primitive.authored_lengths() {
        match length {
            Some(l) => {
                h.write_tag(1);
                h.write_f64_bits(l);
            }
            None => h.write_tag(0),
        }
    }
    h.write_tag(match a.sense {
        AxisSense::Aligned => 1,
        AxisSense::Opposed => 2,
    });
    match a.clocking {
        Some(theta) => {
            h.write_tag(1);
            h.write_f64_bits(theta);
        }
        None => h.write_tag(0),
    }
    for frame in [&a.a, &a.b] {
        for x in frame
            .origin
            .iter()
            .chain(frame.axis.iter())
            .chain(frame.reference.iter())
        {
            h.write_f64_bits(*x);
        }
    }
}

/// The key tag of a contact class. One function, so a mate's own key
/// and the crossing record that quotes its class cannot drift apart —
/// they must agree, or a crossing edit and the mate edit that caused
/// it would key inconsistently.
///
/// The `_` arm is forced by `ContactClass`'s `#[non_exhaustive]`, and
/// it is a KNOWN sharp edge: a third class landing (`Fit { gap }`) and
/// a fourth would both key as 0 and could collide in the memo. Every
/// class this crate can NAME has its own tag, so the collision needs
/// two unnamed classes to exist at once — but when `Fit` lands, its
/// tag lands here with it rather than riding the wildcard.
fn contact_class_tag(class: topo::ContactClass) -> u8 {
    match class {
        topo::ContactClass::Rest => 1,
        topo::ContactClass::Tangent => 2,
        _ => 0,
    }
}

/// Feeds a measured expression: one tag per AST node, then each
/// node's own payload. The tag space is closed and the match is
/// EXHAUSTIVE, so a new arithmetic arm cannot default to hashing like
/// an existing one — the S4 lesson (a step verb's key tag collided
/// with another's and served the wrong geometry from the memo).
fn feed_measure_expr(h: &mut KeyHasher, expr: &crate::measure::MeasureExpr) {
    use crate::measure::{MeasureKind as K, MeasurePrimitive as P};
    let binary = |h: &mut KeyHasher, tag, a: &crate::measure::MeasureExpr, b: &_| {
        h.write_tag(tag);
        feed_measure_expr(h, a);
        feed_measure_expr(h, b);
    };
    match expr.kind() {
        K::Primitive(p) => {
            h.write_tag(1);
            h.write_tag(match p {
                P::Distance { .. } => 1,
                P::Angle { .. } => 2,
                P::Gap { .. } => 3,
            });
            for index in p.refs() {
                h.write_u64(u64::from(index));
            }
        }
        K::Value(e) => {
            h.write_tag(2);
            // The value leaf's literal BITS and parameter names — the
            // same two facts `Expr::bit_eq` compares, so two leaves
            // that are bit-equal hash equal and no others do.
            let mut bits = Vec::new();
            e.literal_bits(&mut bits);
            h.write_u64(bits.len() as u64);
            for b in bits {
                h.write_u64(b);
            }
            let mut params = Vec::new();
            e.param_refs(&mut params);
            h.write_u64(params.len() as u64);
            for (name, dim) in params {
                h.write_str(&name.0);
                h.write_tag(dimension_tag(dim));
            }
        }
        K::Neg(a) => {
            h.write_tag(3);
            feed_measure_expr(h, a);
        }
        K::Add(a, b) => binary(h, 4, a, b),
        K::Sub(a, b) => binary(h, 5, a, b),
        K::Mul(a, b) => binary(h, 6, a, b),
        K::Div(a, b) => binary(h, 7, a, b),
        K::Min(a, b) => binary(h, 8, a, b),
        K::Max(a, b) => binary(h, 9, a, b),
    }
}

/// The closed dimension lattice as key tags.
fn dimension_tag(dim: crate::expr::Dimension) -> u8 {
    match dim {
        crate::expr::Dimension::Length => 1,
        crate::expr::Dimension::Angle => 2,
        crate::expr::Dimension::Count => 3,
        crate::expr::Dimension::Scalar => 4,
    }
}

/// Feeds one stable name: its entity kind as a tag, its minting node,
/// then its role path segment by segment. Every field participates —
/// a name is an identity, and two names differing anywhere are two
/// different recipe payloads. Names are float-free by construction
/// (pure tags and integers), so nothing here is eps-dependent.
fn feed_stable_name(h: &mut KeyHasher, name: &StableName) {
    use crate::names::EntityKind;
    h.write_tag(match name.kind {
        EntityKind::Body => 1,
        EntityKind::Face => 2,
        EntityKind::Edge => 3,
        EntityKind::Vertex => 4,
    });
    h.write_u64(name.node.0);
    h.write_u64(name.path.len() as u64);
    for seg in &name.path {
        feed_role_seg(h, seg);
    }
}

/// Feeds one role segment (closed enum — every variant tagged; the
/// tags are part of the key format version).
fn feed_role_seg(h: &mut KeyHasher, seg: &crate::names::RoleSeg) {
    use crate::names::{CapEnd, MeridianEnd, Qualifier, RoleSeg, SideVerdict, SplitHalf};
    let cap = |c: CapEnd| match c {
        CapEnd::Top => 1u64,
        CapEnd::Bottom => 2,
    };
    let mer = |m: MeridianEnd| match m {
        MeridianEnd::Start => 1u64,
        MeridianEnd::End => 2,
        MeridianEnd::Seam => 3,
        MeridianEnd::Pi => 4,
    };
    let half = |s: SplitHalf| match s {
        SplitHalf::Above => 1u64,
        SplitHalf::Below => 2,
    };
    let rim = |s: crate::names::RimSupport| match s {
        crate::names::RimSupport::Host => 1u64,
        crate::names::RimSupport::Mate => 2,
    };
    let pe = |h: &mut KeyHasher, e: crate::names::ProfileEdgeRef| {
        h.write_u64(u64::from(e.loop_index));
        h.write_u64(u64::from(e.segment));
    };
    let pv = |h: &mut KeyHasher, v: crate::names::ProfileVertexRef| {
        h.write_u64(u64::from(v.loop_index));
        h.write_u64(u64::from(v.vertex));
    };
    let qual = |h: &mut KeyHasher, q: &Qualifier| match q {
        Qualifier::SideOf(vec) => {
            h.write_tag(1);
            h.write_u64(vec.len() as u64);
            for (name, v) in vec {
                feed_stable_name(h, name);
                h.write_tag(match v {
                    SideVerdict::Positive => 1,
                    SideVerdict::Negative => 2,
                    SideVerdict::Mixed => 3,
                    SideVerdict::On => 4,
                });
            }
        }
        Qualifier::OrderAlong { rank, of } => {
            h.write_tag(2);
            h.write_u64(u64::from(*rank));
            h.write_u64(u64::from(*of));
        }
    };
    match seg {
        RoleSeg::OutputBody => h.write_tag(1),
        RoleSeg::Cap(c) => {
            h.write_tag(2);
            h.write_u64(cap(*c));
        }
        RoleSeg::Lateral(e) => {
            h.write_tag(3);
            pe(h, *e);
        }
        RoleSeg::RimEdge(c, e) => {
            h.write_tag(4);
            h.write_u64(cap(*c));
            pe(h, *e);
        }
        RoleSeg::LateralEdge(v) => {
            h.write_tag(5);
            pv(h, *v);
        }
        RoleSeg::CapVertex(c, v) => {
            h.write_tag(6);
            h.write_u64(cap(*c));
            pv(h, *v);
        }
        RoleSeg::Band(e) => {
            h.write_tag(7);
            pe(h, *e);
        }
        RoleSeg::BandRim(v) => {
            h.write_tag(8);
            pv(h, *v);
        }
        RoleSeg::BandRimPi(v) => {
            h.write_tag(9);
            pv(h, *v);
        }
        RoleSeg::BandPi(e) => {
            h.write_tag(10);
            pe(h, *e);
        }
        RoleSeg::Meridian(m, e) => {
            h.write_tag(11);
            h.write_u64(mer(*m));
            pe(h, *e);
        }
        RoleSeg::MeridianVertex(m, v) => {
            h.write_tag(12);
            h.write_u64(mer(*m));
            pv(h, *v);
        }
        RoleSeg::RevolveCap(m) => {
            h.write_tag(13);
            h.write_u64(mer(*m));
        }
        RoleSeg::Pole(v) => {
            h.write_tag(14);
            pv(h, *v);
        }
        RoleSeg::AxisEdge(e) => {
            h.write_tag(15);
            pe(h, *e);
        }
        RoleSeg::FromA(inner) => {
            h.write_tag(16);
            feed_stable_name(h, inner);
        }
        RoleSeg::FromB(inner) => {
            h.write_tag(17);
            feed_stable_name(h, inner);
        }
        RoleSeg::Seam { a, b } => {
            h.write_tag(18);
            feed_stable_name(h, a);
            feed_stable_name(h, b);
        }
        RoleSeg::Merged(names) => {
            h.write_tag(19);
            h.write_u64(names.len() as u64);
            for n in names {
                feed_stable_name(h, n);
            }
        }
        RoleSeg::Fragment(q) => {
            h.write_tag(20);
            qual(h, q);
        }
        RoleSeg::SplitBody(s) => {
            h.write_tag(21);
            h.write_u64(half(*s));
        }
        RoleSeg::SectionFace { side, section } => {
            h.write_tag(22);
            h.write_u64(half(*side));
            h.write_u64(u64::from(*section));
        }
        RoleSeg::SectionEdge { side, face } => {
            h.write_tag(23);
            h.write_u64(half(*side));
            feed_stable_name(h, face);
        }
        RoleSeg::SplitFragment { side, parent } => {
            h.write_tag(24);
            h.write_u64(half(*side));
            feed_stable_name(h, parent);
        }
        RoleSeg::CrossingVertex { side, edge } => {
            h.write_tag(25);
            h.write_u64(half(*side));
            feed_stable_name(h, edge);
        }
        RoleSeg::OnToolVertex { side, of } => {
            h.write_tag(27);
            h.write_u64(half(*side));
            feed_stable_name(h, of);
        }
        RoleSeg::InPart { of } => {
            h.write_tag(40);
            feed_stable_name(h, of);
        }
        RoleSeg::Instance { i, of } => {
            h.write_tag(26);
            h.write_u64(u64::from(*i));
            feed_stable_name(h, of);
        }
        // The fillet vocabulary (M6-5). Tags continue the one shared
        // sequence; they are part of the key format version.
        RoleSeg::FromTarget(n) => {
            h.write_tag(28);
            feed_stable_name(h, n);
        }
        RoleSeg::BlendFace(n) => {
            h.write_tag(29);
            feed_stable_name(h, n);
        }
        RoleSeg::CornerFace(n) => {
            h.write_tag(30);
            feed_stable_name(h, n);
        }
        RoleSeg::TrimEdge { edge, support } => {
            h.write_tag(31);
            feed_stable_name(h, edge);
            feed_stable_name(h, support);
        }
        RoleSeg::FootVertex { vertex, support } => {
            h.write_tag(32);
            feed_stable_name(h, vertex);
            feed_stable_name(h, support);
        }
        RoleSeg::CornerArc { vertex, edge } => {
            h.write_tag(33);
            feed_stable_name(h, vertex);
            feed_stable_name(h, edge);
        }
        RoleSeg::BandFace(names) => {
            h.write_tag(34);
            h.write_u64(names.len() as u64);
            for n in names {
                feed_stable_name(h, n);
            }
        }
        RoleSeg::BandTrim { edge, support } => {
            h.write_tag(35);
            feed_stable_name(h, edge);
            h.write_u64(rim(*support));
        }
        RoleSeg::BandFoot(n) => {
            h.write_tag(36);
            feed_stable_name(h, n);
        }
        RoleSeg::BandCross(n) => {
            h.write_tag(37);
            feed_stable_name(h, n);
        }
        RoleSeg::BandCut(n) => {
            h.write_tag(38);
            feed_stable_name(h, n);
        }
        RoleSeg::BandSlit(n) => {
            h.write_tag(39);
            feed_stable_name(h, n);
        }
    }
}

#[cfg(test)]
#[allow(clippy::panic)]
mod verb_tag_tests {
    use super::{RETIRED_VERB_TAGS, verb_tag};

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
}

#[cfg(test)]
#[allow(clippy::expect_used)]
mod verb_content_tag_tests {
    use super::verb_content_tag;

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
        assert_eq!(verb_content_tag(verbs::VerbKind::Fillet), 17);
        assert_eq!(verb_content_tag(verbs::VerbKind::Chamfer), 24);
        assert_eq!(
            verb_content_tag(verbs::VerbKind::Boolean(topo::BooleanOp::Union)),
            8
        );
        assert_eq!(
            verb_content_tag(verbs::VerbKind::Boolean(topo::BooleanOp::Intersect)),
            9
        );
        assert_eq!(
            verb_content_tag(verbs::VerbKind::Boolean(topo::BooleanOp::Subtract)),
            10
        );
    }

    /// No two verbs share a tag — the property `verb_tag`'s injectivity
    /// row asserts for the profile vocabulary, here for the node one,
    /// computed over `VerbKind::ALL` so a verb the vocabulary gains is
    /// measured the moment it has an arm.
    #[test]
    fn verb_content_tags_are_injective() {
        let mut seen: Vec<(verbs::VerbKind, u8)> = Vec::new();
        for kind in verbs::VerbKind::ALL {
            let tag = verb_content_tag(*kind);
            assert!(
                !seen.iter().any(|(_, t)| *t == tag),
                "{kind:?} shares content tag {tag} with {:?}",
                seen.iter().find(|(_, t)| *t == tag).map(|(k, _)| *k)
            );
            seen.push((*kind, tag));
        }
        assert_eq!(seen.len(), verbs::VerbKind::ALL.len());
    }

    /// **The COMBINED node-tag space is injective** — the migrated
    /// verbs' tags and every tag still written inline, checked as the
    /// one space they actually are.
    ///
    /// The row above is not this row. It says no two VERBS collide, and
    /// it would stay green while a new inline node claimed 17 or 24 —
    /// which is precisely the accident that moving two tags out of the
    /// match created the room for, and precisely the accident the S4
    /// lesson (`Step::AtToward` colliding with `ArcContinue`, caught by
    /// a reviewer rather than a type) says costs a memo hit serving
    /// another node's geometry.
    ///
    /// **It is a source census, and that is the honest shape here.** The
    /// tags live in a match over `&Node`, so enumerating them by calling
    /// `content_key` would mean constructing one of every node variant —
    /// a fixture larger than the property, and one that would go stale
    /// silently. Instead the sentinels bracketing that match delimit the
    /// text, every `=> <number>` inside it is read as an inline tag, and
    /// every `verb_content_tag(VerbKind::X)` is read as a migrated one
    /// and resolved through the real function. Nothing is hand-listed,
    /// so nothing drifts: a tag added inside the sentinels is measured
    /// the moment it is typed.
    ///
    /// What it cannot see, stated: a tag written OUTSIDE the sentinels
    /// (the sentinel comment says not to), and a tag whose arm computes
    /// rather than names a number. Neither exists today.
    #[test]
    fn node_tag_space_is_injective() {
        const SOURCE: &str = include_str!("mod.rs");
        let region = SOURCE
            .split_once("NODE-TAG-SPACE BEGIN")
            .expect("the tag match carries its opening sentinel")
            .1
            .split_once("NODE-TAG-SPACE END")
            .expect("the tag match carries its closing sentinel")
            .0;
        // Comments inside the region discuss tag numbers in prose ("the
        // tag-29 lesson"), which are not arms — blanked through the
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
                if code.contains(&format!("VerbKind::{token}")) {
                    tags.push((verb_content_tag(*kind), name));
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
            assert!(
                tags.iter().any(|(t, _)| *t == verb_content_tag(*kind)),
                "{kind:?}'s migrated tag is not reachable from the node match — the census is \
                 measuring the wrong region"
            );
        }
        let mut seen: Vec<(u8, String)> = Vec::new();
        for (tag, who) in tags {
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
}

#[cfg(test)]
#[allow(clippy::expect_used)]
mod alignment_key {
    //! The mate datum's authored LENGTHS reach the content key.
    //!
    //! No fixture in the suite authors a `PlanarRest` — every mate in
    //! `asm_r2b_assembly` is `FrameCoincidence`, and a lone planar
    //! rest is under-determined, so no end-to-end row can carry this.
    //! Which is exactly the condition under which a length silently
    //! stops being hashed: two documents differing only in the
    //! standoff would share a memo entry and one would be served the
    //! other's answer. Dropping the length from the key reds this row
    //! and nothing else in the crate.

    use super::{KeyHasher, feed_alignment};
    use crate::mate::{Alignment, AxisSense, MateFrame, MatePrimitive};

    fn datum(primitive: MatePrimitive) -> Alignment {
        let frame = MateFrame {
            origin: [0.0, 0.0, 0.0],
            axis: [0.0, 0.0, 1.0],
            reference: [1.0, 0.0, 0.0],
        };
        Alignment {
            a: frame,
            b: frame,
            primitive,
            sense: AxisSense::Opposed,
            clocking: None,
        }
    }

    fn key(primitive: MatePrimitive) -> crate::eval::ContentKey {
        let mut h = KeyHasher::new();
        feed_alignment(&mut h, &datum(primitive));
        h.finish()
    }

    #[test]
    fn a_planar_rest_standoff_moves_the_key() {
        assert_ne!(
            key(MatePrimitive::PlanarRest { offset: 0.0 }),
            key(MatePrimitive::PlanarRest { offset: 0.25 }),
            "the standoff is authored data and must reach the key"
        );
    }

    /// And the primitives that author no length still separate from
    /// each other and from a rest at zero standoff — the tag is doing
    /// its own job, so the row above is testing the payload.
    #[test]
    fn the_primitive_choice_separates_on_its_own() {
        let keys = [
            key(MatePrimitive::FrameCoincidence),
            key(MatePrimitive::Coaxial),
            key(MatePrimitive::Clocking),
            key(MatePrimitive::PlanarRest { offset: 0.0 }),
        ];
        for (i, a) in keys.iter().enumerate() {
            for b in &keys[i + 1..] {
                assert_ne!(a, b, "each primitive keys distinctly");
            }
        }
    }
}
