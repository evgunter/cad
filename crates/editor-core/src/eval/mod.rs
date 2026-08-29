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

mod memo;
mod schedule;
mod slots;
mod wire;

pub use memo::{ContentBits, ContentKey, KeyHasher};

use std::collections::BTreeMap;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};

use geom_core::{Decide, Indeterminate, Point3, Vec3};
use profile::{ProfileError, ValidatedProfile};
use sweep::{ExtrudeError, RevolveError};
use topo::splitting::SplitError;
use topo::transform::TransformError;
use topo::{Body, BooleanError, BooleanResultKind, ContactRecords};

use crate::appearance::{self, AppearanceResolution};
use crate::doc::Doc;
use crate::expr::EvalError;
use crate::names::{NameTable, NamingError};
use crate::node::{RecipeNodeId, SlotId, StableName};
use crate::profile_desc::ProfileDesc;

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
    /// A validated profile (D3: the wrapped description through the
    /// profile crate's validation door).
    Profile(Arc<ValidatedProfile<T>>),
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
    /// A Declare node's pairs, passed through as data (D3; threading
    /// into booleans is PR 5).
    Declarations(Vec<(StableName, StableName)>),
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
    /// Profile validation refused the description.
    Profile(ProfileError),
    /// The extrude op refused.
    Extrude(ExtrudeError),
    /// The revolve op refused.
    Revolve(RevolveError),
    /// The split op refused.
    Split(SplitError),
    /// The boolean op refused.
    Boolean(BooleanError),
    /// The rigid-transform op refused.
    Transform(TransformError),
    /// An input id names no live node (a dangling reference —
    /// unreachable through `apply`, refused typed anyway).
    MissingInput {
        /// The dangling id.
        input: RecipeNodeId,
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
    /// The node is in (or downstream of) a dependency cycle — Kahn
    /// never released it (unreachable through `apply`, refused typed).
    UnschedulableCycle,
    /// Name emission failed (M4 PR 3, spec D4's loud door): an
    /// emission bug, a kernel-emission gap, or an in-band N2
    /// discriminator escalation — carried unaltered.
    Naming(NamingError),
    /// A sketch node's branch selection refused (SOLVER-DESIGN W3;
    /// M4 PR 4 pins the document semantics — a per-node failure
    /// poisoning descendants only, GQ2/W5). NEVER constructed before
    /// the M6 solver: the arm exists so the solver lands as logic,
    /// not a schema change.
    WitnessBifurcation(crate::witness::WitnessBifurcation),
}

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
}

impl Default for EvalOptions {
    fn default() -> Self {
        Self {
            epoch: Epoch::mint(),
            parallel: false,
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
    doc: &Doc<ProfileDesc>,
    prior: Option<&Evaluation<T>>,
    cancel: &CancelToken,
    opts: &EvalOptions,
) -> Evaluation<T>
where
    T: Decide + ContentBits + Send + Sync,
{
    let sched = schedule::schedule(doc);
    let env = doc.param_env::<T>();
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
                .map(|&id| (id, eval_node(doc, &env, id, &nodes, prior)))
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
            let step = eval_node(doc, &env, id, &nodes, prior);
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
    doc: &Doc<ProfileDesc>,
    env: &crate::expr::ParamEnv<T>,
    id: RecipeNodeId,
    results: &BTreeMap<RecipeNodeId, NodeResult<T>>,
    prior: Option<&Evaluation<T>>,
) -> NodeStep<T>
where
    T: Decide + ContentBits,
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
            Some(NodeResult::Ok(v)) => upstream_keys.push(v.content_key),
        }
    }

    // Slot evaluation — the (node, slot) context door (PR 1's banked
    // NonFiniteResult obligation lands here).
    let slot_values = match slots::eval_slots(node, env) {
        Ok(v) => v,
        Err((slot, source)) => return fail(NodeErrorKind::Expr { slot, source }),
    };

    let content_key = content_key(node, &slot_values, &upstream_keys, doc.witness(id));

    // Memo (spec D4): same key ⇒ same inputs ⇒ (D9) same output.
    if let Some(NodeResult::Ok(v)) = prior.and_then(|p| p.nodes.get(&id))
        && v.content_key == content_key
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
    let op = wire::run_op(id, node, results, &slot_values);
    let verdicts = geom_core::k_stats::take_verdict_log();
    match op {
        Ok((payload, name_table)) => NodeStep {
            result: NodeResult::Ok(NodeValue {
                payload,
                name_table,
                verdicts: Arc::new(verdicts),
                witness: WitnessSlot {},
                content_key,
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
    node: &crate::node::Node<ProfileDesc>,
    slot_values: &slots::SlotValues<T>,
    upstream_keys: &[ContentKey],
    witness: Option<&crate::witness::WitnessDatum>,
) -> ContentKey
where
    T: Decide + ContentBits,
{
    use crate::node::{Datum, Node, PatternKind};
    let mut h = KeyHasher::new();
    h.write_tag(1); // key format v1
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
        },
        Node::Declare { .. } => 14,
    };
    h.write_tag(tag);
    // Structural payloads beyond the tag: profile floats and Declare
    // pairs (StableName is float-free by construction).
    match node {
        Node::Profile(desc) => {
            for bits in desc.float_bits() {
                h.write_u64(bits);
            }
        }
        Node::Declare { pairs } => {
            h.write_u64(pairs.len() as u64);
            for (a, b) in pairs {
                feed_stable_name(&mut h, a);
                feed_stable_name(&mut h, b);
            }
        }
        _ => {}
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
        RoleSeg::Instance { i, of } => {
            h.write_tag(26);
            h.write_u64(u64::from(*i));
            feed_stable_name(h, of);
        }
    }
}
