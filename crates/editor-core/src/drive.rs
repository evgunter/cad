//! **The E6 subdivision driver**: `drive(doc, box, config)` →
//! [`ParamBoxVerdict`].
//!
//! The analysis lane's one answer to "does this document still build,
//! and build the SAME thing, over this parameter box". It subdivides
//! the box into leaves and puts every leaf in exactly one bucket:
//! **certified** (the leaf provably shares the witness build's
//! structure) or **refused**, typed and priced. Nothing is dropped and
//! nothing is partial — the [`Receipt`] identity below is the theorem.
//!
//! # The leaf protocol, and what "certified" is allowed to mean
//!
//! A leaf replays the recipe at [`Interval`] over the leaf's own
//! parameter environment (E8: the committed witness data verbatim; the
//! profile lift GUIDED, so profile geometry is a function of the
//! leaf's parameters and every consumed structure decision is
//! re-verified there). It certifies when, and only when:
//!
//! 1. **every predicate was definite** — no `k_stats` escalation, no
//!    guided typed abort, no node that failed for a reason this driver
//!    cannot prove definite; and
//! 2. **the leaf's verdict vector equals the witness build's, EXACTLY**
//!    — same nodes, same outcomes, same predicates, same signs, in
//!    order.
//!
//! Clause 2 is an equality of [`geom_core::k_stats::Verdict`] rows,
//! which are float-free and scalar-independent by construction: the
//! same decisions at `f64` and at `Interval` produce the same rows.
//! **It is never a width heuristic.** An enclosure being narrow is not
//! evidence about topology, and a driver that certified on width would
//! certify precisely the leaves whose geometry it had failed to
//! resolve. The comparison is the certificate.
//!
//! A leaf that is fully definite on a DIFFERENT vector is not a
//! failure of the analysis — it is the finding. It refuses
//! [`RefusalReason::FlipCrossing`] with the flipped predicates named
//! from the vector diff (no-flips v1: no branch enumeration, no
//! analysis of the far side).
//!
//! # The receipt identity
//!
//! The subdivision is a binary tree: every box popped from the
//! frontier is certified, refused, or split into exactly two, so
//!
//! ```text
//! certified + refused == splits + 1
//! ```
//!
//! for every drive, including the ones that exhaust a budget — budget
//! exhaustion refuses the boxes it did not examine rather than
//! forgetting them. That identity is what "the certified and refused
//! leaves cover the box exactly" means arithmetically, it is checked
//! on every drive ([`Receipt::holds`], asserted before the verdict is
//! returned), and it rides the shipped verdict so a consumer can
//! re-check it without trusting this module.
//!
//! # Read-only (E8)
//!
//! [`drive`] takes `&Doc` and returns a value. There is no `&mut` in
//! its signature, no interior mutability on the path, and no
//! re-witnessing however clean a certificate comes out: a document
//! write is not something this API can express, which is a stronger
//! statement than one it does not perform.

use std::collections::BTreeMap;
use std::sync::Arc;

use geom_core::interval::Interval;
use geom_core::k_stats::Verdict;
use geom_core::{MarginDiag, Tol};

#[cfg(feature = "probe")]
use crate::analysis::BoxAxis;
use crate::analysis::{AnalyzedBox, MeasureUnavailable, ParamBox};
use crate::doc::{Doc, ParamName};
use crate::eval::{
    CancelToken, ContentKey, Epoch, EvalOptions, Evaluation, KeyHasher, NodeErrorKind, NodeResult,
    ProfileLift, evaluate,
};
use crate::node::RecipeNodeId;
use crate::program::ProfileProgram;
use crate::witness::WitnessBifurcation;

/// The per-axis split budget: how many times ONE axis of the box may be
/// bisected before the driver refuses the remaining mass rather than
/// refining it further.
///
/// A recorded run dial (E6: "run config like K"), not a constant of
/// nature. 24 halvings take an axis to `2^-24` of its analyzed width —
/// about seven decimal digits, well past where any tolerance study's
/// input is meaningful, and short of the `f64` grid where bisection
/// stops being able to split at all.
pub const DEFAULT_MAX_DEPTH: u32 = 24;

/// The whole-drive leaf budget: how many leaves one drive may produce
/// before the rest of the frontier refuses [`RefusalReason::Budget`].
///
/// A recorded run dial. `65_536` is `2^16` — sixteen bisections' worth
/// of frontier — which bounds a drive's cost at the same order as its
/// per-axis depth bound and keeps the verdict a thing a report can
/// hold.
pub const DEFAULT_MAX_LEAVES: usize = 65_536;

/// How one drive is run: the two budgets, and the schedule.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DriveConfig {
    /// Per-axis bisection budget ([`DEFAULT_MAX_DEPTH`]).
    pub max_depth: u32,
    /// Whole-drive leaf budget ([`DEFAULT_MAX_LEAVES`]).
    pub max_leaves: usize,
    /// Whether independent leaves run under rayon (D9 idiom 1). A
    /// RUNTIME switch in the evaluation service's own mould, so the
    /// determinism cross-check compares both schedules in one test
    /// run; the verdict is bit-identical either way, which is what the
    /// differential row pins.
    pub parallel: bool,
    /// Whether certified leaves are additionally replayed at the
    /// K-telemetry recording scalar, so driver-path predicate margins
    /// reach the `k_stats` funnel (E6's T6 obligation). Off by default:
    /// it doubles the certified leaves' evaluation cost and the sink it
    /// writes to only exists in a probe build.
    #[cfg(feature = "probe")]
    pub k_probe: KProbe,
}

/// How driver-path predicate samples reach the `k_stats` funnel.
///
/// The funnel is the existing one — [`geom_core::k_stats::Probe`] and
/// the thread-local sink `start_recording`/`take_samples` install —
/// and this dial only decides whether the driver feeds it. No second
/// recording channel exists, and inventing one is exactly what E6's
/// obligation forbids: the K distribution has to be the SAME
/// distribution, gathered from more of the kernel's work.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg(feature = "probe")]
pub enum KProbe {
    /// Certified leaves are not replayed; the funnel sees only what an
    /// ordinary evaluation puts in it.
    #[default]
    Off,
    /// Each certified leaf is replayed at `Probe` over the DEGENERATE
    /// box at the leaf's own midpoint — a concrete parameter point
    /// inside a leaf the driver proved definite, which is exactly the
    /// population E6 wants K to see: margins driven toward zero by
    /// refinement, sampled where refinement stopped.
    CertifiedMidpoints,
}

impl Default for DriveConfig {
    fn default() -> Self {
        Self {
            max_depth: DEFAULT_MAX_DEPTH,
            max_leaves: DEFAULT_MAX_LEAVES,
            parallel: false,
            #[cfg(feature = "probe")]
            k_probe: KProbe::Off,
        }
    }
}

/// How one node's replay came out, as scalar-independent data.
///
/// The verdict rows alone do not separate "this node built and decided
/// nothing" from "this node failed before deciding anything", and a
/// certificate that could not tell those apart would certify a leaf
/// whose build refused. The tag closes that.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReplayOutcome {
    /// The node produced a value.
    Built,
    /// The node itself refused.
    Refused,
    /// An ancestor refused; this node never ran.
    Poisoned,
}

/// One node's row of a [`VerdictVector`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VerdictRow {
    /// The node.
    pub node: RecipeNodeId,
    /// How its replay came out.
    pub outcome: ReplayOutcome,
    /// Every definite decision it made, in decision order.
    pub verdicts: Vec<Verdict>,
}

/// An evaluation's verdict vector: one [`VerdictRow`] per node, in the
/// evaluation's deterministic order.
///
/// Float-free, so equality is exact and means what it says. This is the
/// object leaf certification compares, and the ONLY thing it compares.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct VerdictVector {
    /// The rows, in evaluation order.
    pub rows: Vec<VerdictRow>,
}

impl VerdictVector {
    /// The vector of an evaluation, in its own `order`.
    pub fn of<T: geom_core::Decide>(ev: &Evaluation<T>) -> Self {
        let rows = ev
            .order
            .iter()
            .map(|&node| {
                let (outcome, verdicts) = match ev.nodes.get(&node) {
                    Some(NodeResult::Ok(v)) => (ReplayOutcome::Built, v.verdicts.to_vec()),
                    Some(NodeResult::Failed(_)) => (ReplayOutcome::Refused, Vec::new()),
                    // A node with no entry at all (a canceled prefix)
                    // has run nothing, which is what `Poisoned` says
                    // about the rows below it too.
                    Some(NodeResult::Poisoned { .. }) | None => {
                        (ReplayOutcome::Poisoned, Vec::new())
                    }
                };
                VerdictRow {
                    node,
                    outcome,
                    verdicts,
                }
            })
            .collect();
        Self { rows }
    }

    /// The vector's content key — the `verdict_vector_key` a certified
    /// leaf carries. Derived, never persisted (E10).
    pub fn key(&self) -> VerdictVectorKey {
        let mut h = KeyHasher::new();
        h.write_tag(0xE6);
        h.write_u64(self.rows.len() as u64);
        for row in &self.rows {
            h.write_u64(row.node.0);
            h.write_tag(match row.outcome {
                ReplayOutcome::Built => 1,
                ReplayOutcome::Refused => 2,
                ReplayOutcome::Poisoned => 3,
            });
            h.write_u64(row.verdicts.len() as u64);
            for v in &row.verdicts {
                h.write_str(v.predicate);
                h.write_tag(sign_tag(v.sign));
            }
        }
        VerdictVectorKey(h.finish().0)
    }
}

/// The identity of a [`VerdictVector`] — what a certified leaf carries
/// instead of a copy of the vector, since every certified leaf's vector
/// is the witness's by definition.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VerdictVectorKey(pub u128);

fn sign_tag(s: geom_core::Sign) -> u8 {
    match s {
        geom_core::Sign::Negative => 1,
        geom_core::Sign::Zero => 2,
        geom_core::Sign::Positive => 3,
    }
}

/// One flipped decision, named — the evidence a
/// [`RefusalReason::FlipCrossing`] carries.
#[derive(Debug, Clone, PartialEq)]
pub enum Flip {
    /// A predicate both builds decided, decided differently.
    Predicate {
        /// The node that decided it.
        node: RecipeNodeId,
        /// The funnel's name for the predicate.
        predicate: &'static str,
        /// The sign the witness build decided.
        witness: geom_core::Sign,
        /// The sign this leaf decided.
        leaf: geom_core::Sign,
    },
    /// A decision one build made and the other did not — the two
    /// builds' decision SEQUENCES diverged at this node, which is a
    /// structural difference and not a sign difference.
    Sequence {
        /// The node whose decision sequence diverged.
        node: RecipeNodeId,
        /// How many decisions the witness build made there.
        witness: usize,
        /// How many this leaf made.
        leaf: usize,
    },
    /// A node that built in one and refused in the other.
    Outcome {
        /// The node.
        node: RecipeNodeId,
        /// The witness build's outcome.
        witness: ReplayOutcome,
        /// This leaf's outcome.
        leaf: ReplayOutcome,
    },
    /// A GUIDED structure decision the lane classified definitely
    /// otherwise: this binding provably leaves the nominal
    /// elaboration's structure. Carried unaltered, so the refusal names
    /// the decision the profile lift itself named.
    Structure {
        /// The profile node.
        node: RecipeNodeId,
        /// The lift's refusal, verbatim.
        refusal: Box<profile::StructureRefusal>,
    },
}

/// Why a leaf is refused mass.
#[derive(Debug, Clone, PartialEq)]
pub enum RefusalReason {
    /// A genuine semantic sliver (the ratified PR-7 semantics): the
    /// deciding enclosure sits WHOLLY inside the ambiguity band
    /// `(ε, Kε)`, so the quantity being decided is itself in the band
    /// and refinement cannot move it out. Refused, never refined.
    SliverTerminal {
        /// The predicate that could not be decided.
        predicate: &'static str,
    },
    /// The leaf is fully definite on a DIFFERENT verdict vector than
    /// the witness build's: it provably leaves the nominal's branch.
    /// No-flips v1 — the far side is refused mass, never analyzed.
    FlipCrossing {
        /// The decisions that differ, named.
        flipped: Vec<Flip>,
    },
    /// **Unreachable in v1**, and pinned so at the type: no machinery
    /// in this module constructs it. The variant exists because E6's
    /// ratified vocabulary names it and E8 says what it will mean — a
    /// box reaching across a solver fold/branch wall, refused with the
    /// W3 payload — and a vocabulary that grew a variant later would
    /// silently reclassify existing reports. The M6 solver is what
    /// makes it reachable.
    Bifurcation(Box<WitnessBifurcation>),
    /// **Unreachable in v1**, for the same reason as
    /// [`RefusalReason::Bifurcation`]: no real solution over part of
    /// the box (E8's elbow past straightening). Its mass is a
    /// product-level finding once a solver can prove it; nothing here
    /// invents a way to reach it.
    Infeasible,
    /// A budget was exhausted. Typed and priced — never a silent
    /// partial answer, and never a leaf quietly dropped from the
    /// receipt.
    Budget(BudgetKind),
}

impl RefusalReason {
    /// The accounting class this reason prices under.
    pub fn class(&self) -> ReasonClass {
        match self {
            Self::SliverTerminal { .. } => ReasonClass::SliverTerminal,
            Self::FlipCrossing { .. } => ReasonClass::FlipCrossing,
            Self::Bifurcation(_) => ReasonClass::Bifurcation,
            Self::Infeasible => ReasonClass::Infeasible,
            Self::Budget(_) => ReasonClass::Budget,
        }
    }
}

/// Which bound stopped the refinement.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BudgetKind {
    /// The split axis had already been bisected
    /// [`DriveConfig::max_depth`] times.
    Depth {
        /// The bound.
        max_depth: u32,
    },
    /// The drive had reached [`DriveConfig::max_leaves`]; this box was
    /// still on the frontier and is refused rather than forgotten.
    Leaves {
        /// The bound.
        max_leaves: usize,
    },
    /// The box could not be bisected: its midpoint on the split axis
    /// lands on an endpoint, so the `f64` grid itself is the bound.
    /// A work bound like the other two, and typed separately because
    /// widening a budget does not move it.
    Resolution,
}

/// The reasons a mass column can be attributed to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ReasonClass {
    /// [`RefusalReason::SliverTerminal`].
    SliverTerminal,
    /// [`RefusalReason::FlipCrossing`].
    FlipCrossing,
    /// [`RefusalReason::Bifurcation`].
    Bifurcation,
    /// [`RefusalReason::Infeasible`].
    Infeasible,
    /// [`RefusalReason::Budget`].
    Budget,
}

impl ReasonClass {
    /// The class's stable name, for reports and the goldening form.
    pub fn name(self) -> &'static str {
        match self {
            Self::SliverTerminal => "sliver_terminal",
            Self::FlipCrossing => "flip_crossing",
            Self::Bifurcation => "bifurcation",
            Self::Infeasible => "infeasible",
            Self::Budget => "budget",
        }
    }
}

/// What a certified leaf's replay produced, node by node.
///
/// The per-node content key (E10's derived identity) rather than the
/// geometry: a drive can certify tens of thousands of leaves, and a
/// report is not a place to keep tens of thousands of bodies. The keys
/// are what the goldening form compares and what a consumer re-derives
/// geometry from, by evaluating the leaf again.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct LeafResults {
    /// Per node, in evaluation order.
    pub node_keys: Vec<(RecipeNodeId, ContentKey)>,
}

/// A leaf the driver certified.
#[derive(Debug, Clone, PartialEq)]
pub struct CertifiedLeaf {
    /// The leaf's parameter box.
    pub box_: ParamBox,
    /// The identity of its verdict vector — equal to the witness
    /// build's, which is what certification MEANS.
    pub verdict_vector_key: VerdictVectorKey,
    /// What its replay produced.
    pub results: LeafResults,
}

/// A leaf the driver refused, and why.
#[derive(Debug, Clone, PartialEq)]
pub struct RefusedLeaf {
    /// The leaf's parameter box.
    pub box_: ParamBox,
    /// The typed reason.
    pub reason: RefusalReason,
}

/// The counting receipt of one drive.
///
/// The subdivision is a binary tree over the analyzed box: each box is
/// certified, refused, or split in two. [`Receipt::holds`] is the
/// arithmetic that follows, and it is the shipped form of "the
/// certified and refused leaves cover the box exactly".
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Receipt {
    /// Leaves certified.
    pub certified: usize,
    /// Leaves refused.
    pub refused: usize,
    /// Interior nodes of the subdivision tree — boxes that were split.
    pub splits: usize,
}

impl Receipt {
    /// The receipt identity: a binary tree with `splits` interior nodes
    /// has exactly `splits + 1` leaves, and every leaf is certified or
    /// refused.
    pub fn holds(&self) -> bool {
        self.certified + self.refused == self.splits + 1
    }
}

/// The E6 verdict over a parameter box.
#[derive(Debug, Clone, PartialEq)]
pub struct ParamBoxVerdict {
    certified: Vec<CertifiedLeaf>,
    refused: Vec<RefusedLeaf>,
    accounting: MeasureAccounting,
    receipt: Receipt,
    witness_vector: Arc<VerdictVector>,
    root: ParamBox,
}

impl ParamBoxVerdict {
    /// The certified leaves.
    pub fn certified(&self) -> &[CertifiedLeaf] {
        &self.certified
    }

    /// The refused leaves.
    pub fn refused(&self) -> &[RefusedLeaf] {
        &self.refused
    }

    /// The measure accounting (E2).
    pub fn accounting(&self) -> &MeasureAccounting {
        &self.accounting
    }

    /// The counting receipt.
    pub fn receipt(&self) -> Receipt {
        self.receipt
    }

    /// The witness build's verdict vector — the thing every certified
    /// leaf's vector was compared against, shipped once.
    pub fn witness_vector(&self) -> &VerdictVector {
        &self.witness_vector
    }

    /// The box that was driven.
    pub fn root(&self) -> &ParamBox {
        &self.root
    }

    /// The verdict's goldening form: a deterministic, float-exact text
    /// rendering (the form M10-6 consumes).
    ///
    /// Every float is written as its exact bits, so the text is a
    /// faithful image of the verdict rather than a rounded picture of
    /// it, and two runs whose verdicts differ anywhere produce
    /// different text.
    pub fn serialize(&self) -> String {
        use core::fmt::Write as _;
        let mut s = String::new();
        let _ = writeln!(
            s,
            "receipt certified={} refused={} splits={} holds={}",
            self.receipt.certified,
            self.receipt.refused,
            self.receipt.splits,
            self.receipt.holds()
        );
        let _ = writeln!(s, "witness_vector {:032x}", self.witness_vector.key().0);
        for leaf in &self.certified {
            let _ = writeln!(
                s,
                "certified {} key={:032x}",
                render_box(&leaf.box_),
                leaf.verdict_vector_key.0
            );
        }
        for leaf in &self.refused {
            let _ = writeln!(
                s,
                "refused {} {}",
                render_box(&leaf.box_),
                render_reason(&leaf.reason)
            );
        }
        let _ = write!(s, "{}", self.accounting.serialize());
        s
    }

    /// The verdict's content key: the identity of everything
    /// [`Self::serialize`] renders.
    ///
    /// The verdict is a pure function of (the recipe slice, the box,
    /// ε, K, the config), so this key is one too — derived on demand,
    /// never persisted (E10).
    pub fn content_key(&self) -> ContentKey {
        let mut h = KeyHasher::new();
        h.write_tag(0xE7);
        h.write_str(&self.serialize());
        h.finish()
    }
}

/// The E2 accounting: where the box's mass went.
#[derive(Debug, Clone, PartialEq)]
pub struct MeasureAccounting {
    /// Mass on certified leaves.
    pub certified: Result<f64, MeasureUnavailable>,
    /// Mass on refused leaves, per reason. Only the classes that
    /// actually occurred appear.
    pub refused: BTreeMap<ReasonClass, Result<f64, MeasureUnavailable>>,
    /// The mass OUTSIDE the analyzed box (E2's tail): reported, never
    /// dropped. Zero for a box that is its own support.
    pub unanalyzed: Result<f64, MeasureUnavailable>,
    /// **Chamber containment** (E2's amendment): every leaf touching
    /// the analyzed box's boundary is `FlipCrossing`-refused, so the
    /// witness chamber is contained in the box and ALL tail mass is
    /// provably off-branch rather than merely unexamined. The
    /// unresolved budget below is then EXACT rather than conservative.
    pub containment: bool,
}

impl MeasureAccounting {
    /// Certified + every refused column + tail, which is 1 up to the
    /// `f64` error of summing the leaf masses. Refuses when any column
    /// refuses — a total over an unpriceable column is not a total.
    ///
    /// # Errors
    ///
    /// The first [`MeasureUnavailable`] among the columns.
    pub fn total(&self) -> Result<f64, MeasureUnavailable> {
        let mut t = self.certified.clone()?;
        for m in self.refused.values() {
            t += m.clone()?;
        }
        // The analyzed columns sum to the mass INSIDE the box; the tail
        // is the rest, and the two are complements by construction of
        // the analysis module's mass pair.
        t = t * (1.0 - self.unanalyzed.clone()?) + self.unanalyzed.clone()?;
        Ok(t)
    }

    /// The **unresolved-mass budget** (E2/E10's single honesty gate):
    /// refused mass plus tail. `containment` says whether it is exact
    /// or conservative.
    ///
    /// # Errors
    ///
    /// The first [`MeasureUnavailable`] among the refused columns or
    /// the tail.
    pub fn unresolved(&self) -> Result<f64, MeasureUnavailable> {
        let mut inside = 0.0;
        for m in self.refused.values() {
            inside += m.clone()?;
        }
        let tail = self.unanalyzed.clone()?;
        // `inside` is conditional on being in the box; compose with the
        // tail without ever forming `1 - small`.
        Ok(inside * (1.0 - tail) + tail)
    }

    /// The goldening form's accounting block.
    fn serialize(&self) -> String {
        use core::fmt::Write as _;
        let mut s = String::new();
        let _ = writeln!(s, "mass certified {}", render_mass(&self.certified));
        for (class, m) in &self.refused {
            let _ = writeln!(s, "mass {} {}", class.name(), render_mass(m));
        }
        let _ = writeln!(s, "mass unanalyzed {}", render_mass(&self.unanalyzed));
        let _ = writeln!(s, "containment {}", self.containment);
        s
    }
}

/// A drive that could not start.
///
/// Not a leaf class: E6's refusal vocabulary describes what a leaf of a
/// subdivision proved, and these two describe a document that has no
/// subdivision to run. Keeping them out of [`RefusalReason`] is what
/// keeps `Infeasible` genuinely unreachable in v1 instead of becoming
/// the drawer everything awkward goes in.
#[derive(Debug, Clone, PartialEq)]
pub enum DriveRefusal {
    /// The witness build itself does not build: there is no branch for
    /// leaves to be certified against. `cause` renders the node's typed
    /// error; evaluating the document at `f64` hands back the error
    /// itself.
    WitnessDoesNotBuild {
        /// The first node, in evaluation order, that did not build.
        node: RecipeNodeId,
        /// The node error's rendering.
        cause: String,
    },
    /// The box has no varying axis: nothing is declared to vary, so
    /// there is no parameter box to subdivide. The nominal build
    /// answers this document completely, and saying so is more useful
    /// than returning a one-leaf verdict that looks like an analysis.
    NothingVaries,
}

impl core::fmt::Display for DriveRefusal {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::WitnessDoesNotBuild { node, cause } => write!(
                f,
                "the witness build refuses at node {}: {cause} — there is no branch to certify \
                 leaves against until the nominal document builds",
                node.0
            ),
            Self::NothingVaries => f.write_str(
                "no parameter of this document declares a distribution, so the analyzed box has \
                 no varying axis — the nominal build is the whole answer",
            ),
        }
    }
}

impl core::error::Error for DriveRefusal {}

/// **Drives the document over the analyzed box** (E6).
///
/// Read-only: `doc` is shared, the result is a value, and nothing on
/// this path can write, re-witness, or edit the document.
///
/// The verdict is a pure function of (the recipe slice, `analyzed`,
/// `tol`'s ε and K, `config`) — the same inputs give the same verdict
/// bit for bit, in either schedule (D9).
///
/// # Errors
///
/// [`DriveRefusal`] when the document has no branch to certify against
/// or no axis to subdivide.
pub fn drive(
    doc: &Doc<ProfileProgram>,
    analyzed: &AnalyzedBox,
    config: &DriveConfig,
    tol: Tol,
) -> Result<ParamBoxVerdict, DriveRefusal> {
    let root = ParamBox::of(analyzed);
    if root.varying().next().is_none() {
        return Err(DriveRefusal::NothingVaries);
    }

    // The WITNESS build: the document at its nominals, at f64, with the
    // profile lift on. The lift is on rather than off because the leaf
    // replay runs guided and the two vectors must be comparable —
    // guided elaboration at f64 IS plain elaboration at f64, bit for
    // bit (the lift's own differential pin), so this changes the build
    // it produces in no way and makes the two passes the same code.
    let witness: Evaluation<f64> = evaluate(doc, None, &CancelToken::new(), &lane_opts(), tol);
    if let Some(&node) = witness
        .order
        .iter()
        .find(|id| !matches!(witness.nodes.get(id), Some(NodeResult::Ok(_))))
    {
        let cause = witness
            .node_error(node)
            .map_or_else(|| "not evaluated".to_owned(), |e| e.kind.to_string());
        return Err(DriveRefusal::WitnessDoesNotBuild { node, cause });
    }
    let witness_vector = Arc::new(VerdictVector::of(&witness));
    let witness_key = witness_vector.key();
    drop(witness);

    let mut certified: Vec<CertifiedLeaf> = Vec::new();
    let mut refused: Vec<RefusedLeaf> = Vec::new();
    let mut splits = 0usize;

    // A level-synchronous frontier, so the sequential and the parallel
    // schedule visit the same boxes in the same order and combine them
    // positionally (D9 idiom 1: an indexed map, never an accumulation).
    let mut frontier: Vec<Box_> = vec![Box_ {
        box_: root.clone(),
        depths: BTreeMap::new(),
    }];
    while !frontier.is_empty() {
        // The leaf budget is checked against what the frontier ALREADY
        // guarantees: every box on it becomes at least one leaf. When
        // it cannot fit, the whole frontier refuses — the boxes are
        // priced and reported, never dropped.
        if certified.len() + refused.len() + frontier.len() > config.max_leaves {
            for b in frontier.drain(..) {
                refused.push(RefusedLeaf {
                    box_: b.box_,
                    reason: RefusalReason::Budget(BudgetKind::Leaves {
                        max_leaves: config.max_leaves,
                    }),
                });
            }
            break;
        }
        let verdicts: Vec<LeafVerdict> = if config.parallel {
            use rayon::prelude::*;
            frontier
                .par_iter()
                .map(|b| classify(doc, &b.box_, &witness_vector, witness_key, tol))
                .collect()
        } else {
            frontier
                .iter()
                .map(|b| classify(doc, &b.box_, &witness_vector, witness_key, tol))
                .collect()
        };
        let mut next = Vec::new();
        for (b, v) in frontier.drain(..).zip(verdicts) {
            match v {
                LeafVerdict::Certified(leaf) => certified.push(leaf),
                LeafVerdict::Refused(reason) => refused.push(RefusedLeaf {
                    box_: b.box_,
                    reason,
                }),
                LeafVerdict::Bisect => match bisect(&b, &root, config.max_depth) {
                    Ok((a, c)) => {
                        splits += 1;
                        next.push(a);
                        next.push(c);
                    }
                    Err(kind) => refused.push(RefusedLeaf {
                        box_: b.box_,
                        reason: RefusalReason::Budget(kind),
                    }),
                },
            }
        }
        frontier = next;
    }

    #[cfg(feature = "probe")]
    if config.k_probe == KProbe::CertifiedMidpoints {
        for leaf in &certified {
            probe_midpoint(doc, &leaf.box_, tol);
        }
    }

    let receipt = Receipt {
        certified: certified.len(),
        refused: refused.len(),
        splits,
    };
    // The receipt identity, checked on EVERY drive. It is a theorem
    // about the loop above — each box is certified, refused, or split
    // in two — so a violation is a bug in this module and not a
    // property of any document; the assertion is where that claim
    // stops being a comment.
    debug_assert!(
        receipt.holds(),
        "receipt identity broken: {receipt:?} — a box escaped its bucket"
    );
    let accounting = account(analyzed, &root, &certified, &refused);
    Ok(ParamBoxVerdict {
        certified,
        refused,
        accounting,
        receipt,
        witness_vector,
        root,
    })
}

/// The evaluation options every pass of a drive uses: the profile lift
/// ON (so profile geometry is a function of the leaf's parameters and
/// every structure decision is re-verified at the lane), sequential
/// (leaf-level parallelism is the driver's, and nesting a rayon scope
/// per leaf inside one buys nothing), and no memo.
fn lane_opts() -> EvalOptions {
    EvalOptions {
        epoch: Epoch::mint(),
        profile_lift: ProfileLift::Guided,
        ..EvalOptions::default()
    }
}

/// A frontier entry: a box and how many times each axis has been split
/// to reach it (the per-axis depth budget's currency).
struct Box_ {
    box_: ParamBox,
    depths: BTreeMap<ParamName, u32>,
}

/// What one leaf's replay decided.
enum LeafVerdict {
    Certified(CertifiedLeaf),
    Refused(RefusalReason),
    Bisect,
}

/// The leaf protocol: replay at `Interval` over this box and classify.
fn classify(
    doc: &Doc<ProfileProgram>,
    box_: &ParamBox,
    witness_vector: &VerdictVector,
    witness_key: VerdictVectorKey,
    tol: Tol,
) -> LeafVerdict {
    let opts = EvalOptions {
        param_box: Some(Arc::new(box_.clone())),
        ..lane_opts()
    };
    let leaf: Evaluation<Interval> = evaluate(doc, None, &CancelToken::new(), &opts, tol);

    // (i) Definiteness. An indeterminacy anywhere is the cue to
    // bisect — unless the enclosure that could not be classified sits
    // wholly inside the band, in which case refinement provably cannot
    // help and the leaf is a terminal sliver.
    //
    // A node that refused for a reason this driver cannot PROVE
    // definite is treated as indeterminate too, and that is the
    // conservative direction on purpose: it costs refinement and, at
    // the floor, a `Budget` refusal, where the opposite mistake would
    // be a `FlipCrossing` claim about a leaf whose predicates were
    // never decided. The two arms it does prove — the funnel's own
    // escalation and the profile lift's guided abort — are the two E6
    // names as the cue to bisect.
    let mut structure_flips = Vec::new();
    for (&node, result) in &leaf.nodes {
        let NodeResult::Failed(err) = result else {
            continue;
        };
        match &err.kind {
            NodeErrorKind::Escalated { source, .. } => {
                if let Some(p) = sliver(source) {
                    return LeafVerdict::Refused(RefusalReason::SliverTerminal { predicate: p });
                }
                return LeafVerdict::Bisect;
            }
            NodeErrorKind::ProfileLaneReplay {
                structure: Some(refusal),
                ..
            } => match &refusal.kind {
                profile::StructureRefusalKind::Indeterminate(source) => {
                    if let Some(p) = sliver(source) {
                        return LeafVerdict::Refused(RefusalReason::SliverTerminal {
                            predicate: p,
                        });
                    }
                    return LeafVerdict::Bisect;
                }
                profile::StructureRefusalKind::Flipped { .. } => {
                    structure_flips.push(Flip::Structure {
                        node,
                        refusal: Box::new(refusal.clone()),
                    });
                }
            },
            _ => return LeafVerdict::Bisect,
        }
    }

    // (ii) The comparison. EXACT, on the verdict vector — never a
    // width.
    let vector = VerdictVector::of(&leaf);
    if structure_flips.is_empty() && vector == *witness_vector {
        return LeafVerdict::Certified(CertifiedLeaf {
            box_: box_.clone(),
            verdict_vector_key: witness_key,
            results: LeafResults {
                node_keys: leaf
                    .order
                    .iter()
                    .filter_map(|&id| leaf.value(id).map(|v| (id, v.content_key)))
                    .collect(),
            },
        });
    }
    let mut flipped = structure_flips;
    flipped.extend(diff(witness_vector, &vector));
    LeafVerdict::Refused(RefusalReason::FlipCrossing { flipped })
}

/// The predicate name of an escalation whose enclosure sits WHOLLY
/// inside the ambiguity band `(ε, Kε)` — the ratified terminal-sliver
/// test — or `None` when refinement could still decide it.
///
/// The test is on the enclosure, both ends: an enclosure that reaches
/// the coincidence threshold might enclose a genuine coincidence, and
/// one that reaches past `escalate` might enclose a definite sign, so
/// either way there is something narrowing could still resolve. Only an
/// enclosure strictly between the two thresholds, on one side of zero,
/// describes a quantity that IS in the band.
fn sliver(source: &geom_core::Indeterminate) -> Option<&'static str> {
    let MarginDiag::Enclosure { lo, hi } = source.margin else {
        // A point margin (an `f64` lane) or an invalid one says nothing
        // about a box.
        return None;
    };
    let (zero, escalate) = (source.band.zero(), source.band.escalate());
    let inside = (zero < lo && hi < escalate) || (-escalate < lo && hi < -zero);
    inside.then_some(source.predicate.unwrap_or("<unnamed>"))
}

/// The flipped decisions between two verdict vectors, named.
fn diff(witness: &VerdictVector, leaf: &VerdictVector) -> Vec<Flip> {
    let mut out = Vec::new();
    let by_node: BTreeMap<RecipeNodeId, &VerdictRow> =
        leaf.rows.iter().map(|r| (r.node, r)).collect();
    for w in &witness.rows {
        let Some(l) = by_node.get(&w.node) else {
            out.push(Flip::Outcome {
                node: w.node,
                witness: w.outcome,
                leaf: ReplayOutcome::Poisoned,
            });
            continue;
        };
        if w.outcome != l.outcome {
            out.push(Flip::Outcome {
                node: w.node,
                witness: w.outcome,
                leaf: l.outcome,
            });
        }
        for (a, b) in w.verdicts.iter().zip(&l.verdicts) {
            if a.predicate != b.predicate {
                out.push(Flip::Sequence {
                    node: w.node,
                    witness: w.verdicts.len(),
                    leaf: l.verdicts.len(),
                });
                break;
            }
            if a.sign != b.sign {
                out.push(Flip::Predicate {
                    node: w.node,
                    predicate: a.predicate,
                    witness: a.sign,
                    leaf: b.sign,
                });
            }
        }
        if w.verdicts.len() != l.verdicts.len() {
            out.push(Flip::Sequence {
                node: w.node,
                witness: w.verdicts.len(),
                leaf: l.verdicts.len(),
            });
        }
    }
    out
}

/// The D9 split: the axis of greatest relative width, ties to the
/// lowest axis index, bisected at its midpoint.
///
/// # Errors
///
/// The budget that stopped it: the per-axis depth bound, or the `f64`
/// grid.
fn bisect(b: &Box_, root: &ParamBox, max_depth: u32) -> Result<(Box_, Box_), BudgetKind> {
    let Some(axis) = b.box_.split_axis(root) else {
        return Err(BudgetKind::Resolution);
    };
    let depth = b.depths.get(&axis).copied().unwrap_or(0);
    if depth >= max_depth {
        return Err(BudgetKind::Depth { max_depth });
    }
    let Some((lo, hi)) = b.box_.split(&axis) else {
        return Err(BudgetKind::Resolution);
    };
    let mut depths = b.depths.clone();
    depths.insert(axis, depth + 1);
    Ok((
        Box_ {
            box_: lo,
            depths: depths.clone(),
        },
        Box_ { box_: hi, depths },
    ))
}

/// The E2 accounting over a finished leaf set.
fn account(
    analyzed: &AnalyzedBox,
    root: &ParamBox,
    certified: &[CertifiedLeaf],
    refused: &[RefusedLeaf],
) -> MeasureAccounting {
    let mut cert = Ok(0.0);
    for leaf in certified {
        add_mass(&mut cert, leaf.box_.mass(analyzed));
    }
    let mut by_reason: BTreeMap<ReasonClass, Result<f64, MeasureUnavailable>> = BTreeMap::new();
    for leaf in refused {
        let entry = by_reason.entry(leaf.reason.class()).or_insert(Ok(0.0));
        add_mass(entry, leaf.box_.mass(analyzed));
    }
    MeasureAccounting {
        certified: cert,
        refused: by_reason,
        unanalyzed: tail(analyzed),
        containment: contained(root, certified, refused),
    }
}

/// Folds one leaf's mass into a column, keeping the FIRST refusal: a
/// column that cannot be priced names the parameter that stopped it,
/// and a later priceable leaf does not un-refuse it.
fn add_mass(column: &mut Result<f64, MeasureUnavailable>, m: Result<f64, MeasureUnavailable>) {
    if let Ok(acc) = column {
        match m {
            Ok(v) => *acc += v,
            Err(e) => *column = Err(e),
        }
    }
}

/// The mass OUTSIDE the analyzed box under the product measure.
///
/// `1 - ∏(1 - tᵢ)` folded as `out ← out + t - out·t`, which never forms
/// `1 - x` for a small `x` — the same discipline the per-axis tail
/// door is written to (a ±9σ box's `~2e-19` of excluded mass survives
/// the fold instead of rounding to a bit-exact zero).
fn tail(analyzed: &AnalyzedBox) -> Result<f64, MeasureUnavailable> {
    let mut out = 0.0;
    for name in analyzed.params().keys() {
        let t = match analyzed.axis_tail_mass(name) {
            Some(r) => r?,
            None => 0.0,
        };
        out = out + t - out * t;
    }
    Ok(out)
}

/// E2's chamber containment: every leaf touching the analyzed box's
/// boundary is `FlipCrossing`-refused.
///
/// A free predicate on the leaf set — no extra evaluation, no extra
/// geometry. False when no leaf touches the boundary at all, because
/// containment is a claim ABOUT the boundary and a leaf set that never
/// reached it has not made one.
fn contained(root: &ParamBox, certified: &[CertifiedLeaf], refused: &[RefusedLeaf]) -> bool {
    if certified.iter().any(|l| l.box_.touches_boundary_of(root)) {
        return false;
    }
    let mut saw = false;
    for leaf in refused {
        if leaf.box_.touches_boundary_of(root) {
            saw = true;
            if !matches!(leaf.reason, RefusalReason::FlipCrossing { .. }) {
                return false;
            }
        }
    }
    saw
}

/// One certified leaf's midpoint, replayed at the K-telemetry
/// recording scalar so its margins land in whatever sink the caller
/// installed (`k_stats::start_recording`). The existing funnel, the
/// existing scalar, the existing sink — the driver only decides WHICH
/// parameter points get sampled.
#[cfg(feature = "probe")]
fn probe_midpoint(doc: &Doc<ProfileProgram>, box_: &ParamBox, tol: Tol) {
    let mid: BTreeMap<ParamName, BoxAxis> = box_
        .axes()
        .iter()
        .map(|(n, a)| {
            let (lo, hi) = a.span();
            let m = 0.5 * (lo + hi);
            (n.clone(), BoxAxis::Varying { lo: m, hi: m })
        })
        .collect();
    let opts = EvalOptions {
        param_box: Some(Arc::new(ParamBox::from_axes(mid))),
        ..lane_opts()
    };
    let _: Evaluation<geom_core::Probe> = evaluate(doc, None, &CancelToken::new(), &opts, tol);
}

/// A box's goldening rendering: `name=[lo_bits,hi_bits]` per axis, in
/// name order, floats as exact bits.
fn render_box(b: &ParamBox) -> String {
    use core::fmt::Write as _;
    let mut s = String::new();
    for (name, axis) in b.axes() {
        let (lo, hi) = axis.span();
        let _ = write!(
            s,
            "{}=[{:016x},{:016x}] ",
            name.0,
            lo.to_bits(),
            hi.to_bits()
        );
    }
    s.pop();
    s
}

/// A refusal's goldening rendering: the class, and the evidence that
/// names it.
fn render_reason(r: &RefusalReason) -> String {
    use core::fmt::Write as _;
    match r {
        RefusalReason::SliverTerminal { predicate } => format!("sliver_terminal {predicate}"),
        RefusalReason::FlipCrossing { flipped } => {
            let mut s = String::from("flip_crossing");
            for f in flipped {
                let _ = match f {
                    Flip::Predicate {
                        node,
                        predicate,
                        witness,
                        leaf,
                    } => write!(s, " {}:{predicate}:{witness:?}->{leaf:?}", node.0),
                    Flip::Sequence {
                        node,
                        witness,
                        leaf,
                    } => write!(s, " {}:seq:{witness}->{leaf}", node.0),
                    Flip::Outcome {
                        node,
                        witness,
                        leaf,
                    } => write!(s, " {}:out:{witness:?}->{leaf:?}", node.0),
                    Flip::Structure { node, refusal } => {
                        write!(s, " {}:structure:{:?}", node.0, refusal.decision)
                    }
                };
            }
            s
        }
        RefusalReason::Bifurcation(_) => "bifurcation".to_owned(),
        RefusalReason::Infeasible => "infeasible".to_owned(),
        RefusalReason::Budget(k) => match k {
            BudgetKind::Depth { max_depth } => format!("budget depth={max_depth}"),
            BudgetKind::Leaves { max_leaves } => format!("budget leaves={max_leaves}"),
            BudgetKind::Resolution => "budget resolution".to_owned(),
        },
    }
}

/// A mass column's goldening rendering: exact bits, or the refusal.
fn render_mass(m: &Result<f64, MeasureUnavailable>) -> String {
    match m {
        Ok(v) => format!("{:016x}", v.to_bits()),
        Err(MeasureUnavailable::BandHasNoMeasure { param }) => {
            format!("refused band:{}", param.0)
        }
    }
}
