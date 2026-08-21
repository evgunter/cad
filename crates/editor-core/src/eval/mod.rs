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
mod memo;
pub(crate) mod parts;

pub use parts::PartFault;
mod schedule;
mod slots;
mod wire;

pub use anchor::{LoopAnchor, ProfileNaming, ProfileValue, embed_profile};
pub use memo::{ContentBits, ContentKey, KeyHasher, NamingKey};

use std::collections::BTreeMap;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};

use geom_core::{Decide, Indeterminate, Point3, Vec3};
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

/// An evaluated datum (spec D3): geometry VALUES, not kernel entities.
/// Normals/directions are normalized at evaluation; a degenerate
/// (decided-zero-length) vector is a typed refusal.
#[derive(Debug, Clone)]
pub enum DatumValue<T: Decide> {
    /// A plane through `origin` with UNIT `normal`.
    Plane {
        /// A point on the plane.
        origin: Point3<T>,
        /// The unit normal.
        normal: Vec3<T>,
    },
    /// An axis through `origin` along UNIT `dir`.
    Axis {
        /// A point on the axis.
        origin: Point3<T>,
        /// The unit direction.
        dir: Vec3<T>,
    },
    /// A point.
    Point {
        /// Its position.
        position: Point3<T>,
    },
}

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
    /// The constant-radius fillet op refused (M5 PR 12): a structural
    /// precondition on the requested chain, one of the numbered
    /// rolling-ball predicates, a corner or spine class the in-place
    /// surgery has not been built for, or an escalation. Which door
    /// refused, and what it refused about, is stated on
    /// [`sweep::fillet::FilletError`]'s own variants and rendered by
    /// its `Display` — this doc names no predicate of its own, so it
    /// cannot drift from one.
    ///
    /// Carried UNALTERED like every other kernel refusal; the node
    /// never passes its input body through.
    Fillet(sweep::fillet::FilletError),
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
    /// The ambient tolerance could not form a classification band.
    Band(geom_core::BandError),
    /// A slot the wiring expected was absent from the node — a wiring
    /// bug surfaced typed (unreachable while `Node::slots` and the
    /// wire agree; no panic paths in this crate).
    MissingSlot {
        /// The absent slot.
        slot: SlotId,
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
    /// A `Node::Fillet` selection name failed to resolve through the
    /// TARGET's name table (M6-5) — the same N5 typed trio as
    /// [`NodeErrorKind::DeclareResolve`], and for the same reason: a
    /// selection is a commitment, so a name that no longer resolves
    /// refuses loudly instead of silently shrinking the set.
    FilletSelectionResolve {
        /// The resolution failure (N5's closed trio).
        error: Box<crate::resolve::ResolveError>,
    },
    /// A `Node::Fillet` selection named something that is not an EDGE
    /// of the target (a face, a vertex, the body). The op blends
    /// edges; a mis-kinded selection is a recipe bug, refused rather
    /// than reinterpreted.
    FilletSelectionKind {
        /// The offending name.
        name: Box<crate::names::StableName>,
        /// What it actually denotes.
        found: crate::names::EntityKind,
    },
    /// A `Node::Fillet` selection is EMPTY. A fillet of nothing is not
    /// the identity — it is an unfinished recipe, refused rather than
    /// passed through (the fail-loud voice: no op silently returns its
    /// input).
    FilletSelectionEmpty,
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
// Five arms render prose over a payload and forward nothing, for ONE
// reason between them — the payload is an editor-core type with no
// `Display` to forward. `Expr` holds an `EvalError`, `DeclareResolve`
// and `FilletSelectionResolve` a `resolve::ResolveError`,
// `WitnessBifurcation` its own type, and `PlacementRule` a
// `PlacementRuleFault` it hand-expands variant by variant — the second
// prose vocabulary that fault set has, `edit.rs`'s four `EditError`
// arms being the first. Four missing `Display` impls, one list; it is
// **D54**, and this list and that row are the same list.
impl core::fmt::Display for NodeErrorKind {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Expr { slot, .. } => {
                write!(f, "the expression at slot {slot:?} failed to evaluate")
            }
            Self::Profile(e) => write!(f, "the replayed profile failed validation: {e}"),
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
                "instance {}'s seam declaration from mate {} names {name:?}, which \
                 the pinned part's product does not name — the crossing does not \
                 re-verify against this version of the part",
                instance.0, mate.0
            ),
            Self::Extrude(e) => write!(f, "the extrude op refused: {e}"),
            Self::Revolve(e) => write!(f, "the revolve op refused: {e}"),
            Self::Split(e) => write!(f, "the split op refused: {e}"),
            Self::Fillet(e) => write!(f, "the fillet op refused: {e}"),
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
            Self::WrongOperand {
                input,
                expected,
                found,
            } => write!(
                f,
                "input {} is a {found}; the operand needs a {expected}",
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
            Self::PlacementRule(fault) => match fault {
                crate::node::PlacementRuleFault::CountSpelling => f.write_str(
                    "the placement rule and the count slot disagree about how many placements \
                     there are",
                ),
                crate::node::PlacementRuleFault::NoPlacements => f.write_str(
                    "the placement list is empty — a group needs at least one placement, exactly \
                     as a stepped rule needs a count of at least 1",
                ),
                crate::node::PlacementRuleFault::NonFiniteFrame { index } => {
                    write!(f, "placement {index} has a non-finite coordinate")
                }
                crate::node::PlacementRuleFault::ImproperFrame { index, determinant } => write!(
                    f,
                    "placement {index} is improper (mirroring): determinant {determinant}"
                ),
            },
            Self::UnschedulableCycle => {
                f.write_str("the node is in, or downstream of, a dependency cycle")
            }
            // #380: the payload is editor-core's OWN diagnostic, not a
            // kernel refusal riding the variant — it has no other route
            // to a human, so it is carried through rather than dropped.
            Self::Naming(e) => write!(f, "name emission failed: {e}"),
            Self::DeclareResolve { .. } => {
                f.write_str("a declared name failed to resolve through the operands' tables")
            }
            Self::DeclareBothOperands { name } => write!(
                f,
                "declared name {name:?} resolves in BOTH operands — the declaration cannot pick a side"
            ),
            Self::DeclareUnsupportedPair { kinds, .. } => write!(
                f,
                "declare pair {kinds:?} is outside the v1 threading vocabulary"
            ),
            // The menu is what this arm owns and the payload cannot
            // spell, so the prose stays — but the prose is an ADDITION
            // to the diagnostic, not a replacement for it: the ladder's
            // own account of what it measured rides out after the two
            // levers, exactly as `Escalated` carries the same type.
            Self::UndeclaredContact { finding, diag } => write!(
                f,
                "the Boolean refused an undeclared contact: a face pair of its operands \
                 is {} without a shared source or declared intent — the refusal carries \
                 the candidate declaration (the pair, by stable name, with its relation); \
                 declare that finding and wire it into the Boolean's declare input, or \
                 move the geometry. The coincidence ladder reports: {diag}",
                match finding.evidence.relation {
                    topo::PlaneRelation::SameOpposite =>
                        "coincident with opposed orientations (resting contact)",
                    topo::PlaneRelation::SameOriented =>
                        "coincident with the same orientation (flush walls)",
                    // Never constructed on a finding; rendered honestly anyway.
                    topo::PlaneRelation::Distinct => "reported coincident",
                }
            ),
            Self::FilletSelectionResolve { .. } => {
                f.write_str("a fillet selection name failed to resolve")
            }
            Self::FilletSelectionKind { name, found } => write!(
                f,
                "fillet selection {name:?} denotes a {found:?}, not an edge"
            ),
            Self::FilletSelectionEmpty => f.write_str(
                "the fillet selection is empty — an unfinished recipe, not the identity",
            ),
            Self::WitnessBifurcation(_) => f.write_str("the sketch's branch selection refused"),
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
/// needs, and `Send + Sync` for the rayon schedule. ONE name for the
/// set, stated at the evaluation-service seam that owns it — so the
/// modules below this one (`parts`) name the requirement rather than
/// restate it, and the compound `Bounds` bound stays inside the seam
/// the 2026-07-29 Bounds scope rule ratified.
pub trait EvalScalar:
    Decide + ContentBits + geom_core::Bounds + Send + Sync + topo::PropsQuadLane
{
}

impl<T> EvalScalar for T where
    T: Decide + ContentBits + geom_core::Bounds + Send + Sync + topo::PropsQuadLane
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
}

impl Default for EvalOptions {
    fn default() -> Self {
        Self {
            epoch: Epoch::mint(),
            parallel: false,
            boolean_sweep: topo::SweepStrategy::Realized,
            resolver: None,
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
) -> Evaluation<T>
where
    T: Decide + ContentBits + geom_core::Bounds + Send + Sync + topo::PropsQuadLane,
{
    evaluate_at_descent(doc, prior, cancel, opts, &[])
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
) -> Evaluation<T>
where
    T: Decide + ContentBits + geom_core::Bounds + Send + Sync + topo::PropsQuadLane,
{
    evaluate_at_descent(doc, None, cancel, opts, chain)
}

fn evaluate_at_descent<T>(
    doc: &Doc<ProfileProgram>,
    prior: Option<&Evaluation<T>>,
    cancel: &CancelToken,
    opts: &EvalOptions,
    chain: &[crate::ident::DocRef],
) -> Evaluation<T>
where
    T: Decide + ContentBits + geom_core::Bounds + Send + Sync + topo::PropsQuadLane,
{
    let sched = schedule::schedule(doc);
    // D4 door (M4 PR 6): the recorded ε must BE the committed process
    // ε — otherwise every predicate below would silently decide at
    // the wrong tolerance. Refuse loudly, per node, staying total.
    let process_eps = geom_core::Tolerance::get().eps;
    if doc.epsilon().to_bits() != process_eps.to_bits() {
        return refuse_tolerance_conflict(doc, sched, opts, process_eps);
    }
    let env = doc.param_env::<T>();
    let parts = parts::PartCache::<T>::new(opts.resolver.as_ref(), chain, opts.boolean_sweep);
    // The mate solve is a WHOLE-DOCUMENT computation over recipe data
    // (A11): one spanning tree per cluster, folded once, read by every
    // instance and every mate below. Running it here rather than per
    // node is not an optimization — a per-node solve would be a second
    // answer to "where does this cluster sit".
    let poses = crate::mate::solve_document(doc);
    let op_env = wire::OpEnv {
        boolean_sweep: opts.boolean_sweep,
        parts: &parts,
        poses: &poses,
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
                .map(|&id| (id, eval_node(doc, &env, id, &nodes, prior, &op_env)))
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
            let step = eval_node(doc, &env, id, &nodes, prior, &op_env);
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

/// The all-nodes ToleranceConflict refusal (spec D4 door): a TOTAL
/// evaluation in which every live node fails typed and the appearance
/// store resolves against all-failed states (typed losses, nothing
/// silent).
fn refuse_tolerance_conflict<T>(
    doc: &Doc<ProfileProgram>,
    sched: schedule::Schedule,
    opts: &EvalOptions,
    process_eps: f64,
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
                    kind: NodeErrorKind::ToleranceConflict {
                        document_eps: doc.epsilon(),
                        process_eps,
                    },
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
) -> NodeStep<T>
where
    T: Decide + ContentBits + geom_core::Bounds + Send + Sync + topo::PropsQuadLane,
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
    // resolve at f64 — never at `T` — because they feed C6 structure
    // selection, which must be lane-identical (the verified asymmetry:
    // node magnitude slots stay lane-live, profile geometry is
    // f64-pinned). Resolved ONCE here; the same values feed the
    // content key (resolved-value convention, §4e) and the op.
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
            match wire::prepare_profile(program, resolved) {
                Ok(pre) => Some(pre),
                Err(kind) => return fail(kind),
            }
        }
        _ => None,
    };

    let content_key = content_key(
        node,
        &slot_values,
        resolved_program.as_deref(),
        &upstream_keys,
        doc.witness(id),
        op_env.poses.placement(doc, id).ok(),
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
        profile_pre.as_ref(),
        op_env,
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
fn content_key<T>(
    node: &crate::node::Node<ProfileProgram>,
    slot_values: &slots::SlotValues<T>,
    resolved_program: Option<&[Vec<profile::Step<f64>>]>,
    upstream_keys: &[ContentKey],
    witness: Option<&crate::witness::WitnessDatum>,
    placement: Option<crate::placement::Frame>,
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
    h.write_tag(2);
    let tol = geom_core::Tolerance::get();
    h.write_f64_bits(tol.eps);
    h.write_f64_bits(tol.k);
    let tag = match node {
        Node::Datum(Datum::Plane { .. }) => 1,
        Node::Datum(Datum::Axis { .. }) => 2,
        Node::Datum(Datum::Point { .. }) => 3,
        Node::Profile(_) => 4,
        Node::Extrude { .. } => 5,
        Node::Revolve { .. } => 6,
        Node::Split { .. } => 7,
        Node::Boolean { op, .. } => match op {
            crate::node::BooleanOp::Union => 8,
            crate::node::BooleanOp::Intersect => 9,
            crate::node::BooleanOp::Subtract => 10,
        },
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
        // M5 PR 12.
        Node::Fillet { .. } => 17,
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
    };
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
        // The fillet SELECTION is recipe payload, not a slot: two
        // fillets of the same radius on different edges are different
        // nodes (M6-5). Canonical order (`Node::fillet`) is what makes
        // this a set hash rather than an order hash.
        Node::Fillet { selection, .. } => {
            h.write_u64(selection.len() as u64);
            for n in selection {
                feed_stable_name(&mut h, n);
            }
        }
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
        V::Turn => 15,
        V::Line => 16,
        V::LineTo => 17,
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
        Step::Tangent | Step::CloseTo => {}
        Step::Turn(delta) => f(h, *delta),
        Step::Line(len) => f(h, *len),
        Step::LineTo(t) | Step::TangentArcTo(t) => target(h, t),
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

/// Feeds a [`StableName`] structurally into the content key (names
/// are float-free by construction — pure tags and integers).
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
        crate::names::RimSupport::Plane => 1u64,
        crate::names::RimSupport::Curved => 2,
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
