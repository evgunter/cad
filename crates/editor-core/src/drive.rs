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
//! A leaf replays the recipe at `geom_core::Sym<`[`geom_core::Interval`]`>`
//! over the leaf's own parameter environment (E8: the committed witness
//! verbatim; the profile lift GUIDED, so profile geometry is a function
//! of the leaf's parameters and every consumed structure decision is
//! re-verified there). It certifies when, and only when:
//!
//! 1. **every predicate was definite** — no `k_stats` escalation, no
//!    guided typed abort, no node that failed for a reason this driver
//!    cannot prove definite; and
//! 2. **the leaf's CERTIFYING verdict vector equals the witness
//!    build's, EXACTLY** — same nodes, same outcomes, same predicates,
//!    same signs, in order. "Certifying" is one exclusion and it is
//!    named at [`crate::drive::certifying_vector`]: an `Assertion` node
//!    reports and gates nothing (E10 v1), and certification is a gate,
//!    so its rows are not in the comparison.
//!
//! **What the wrapper adds, and what it does not** (ERROR-DESIGN E12).
//! The numeric channel is `Interval`'s, verbatim and bit for bit —
//! `Sym<T>` computes every operation at `T` and mints a DAG node beside
//! it. One thing changes: a margin whose expression is identically zero
//! in the document's parameters decides `Zero` before any enclosure is
//! consulted, at any box width. That is what lets a MACROSCOPIC box
//! certify at all; before it, the certification identities' enclosures
//! widened as `[0, c·w]` and a leaf went definite only below a fraction
//! of ε. `DriveConfig::symbolic` is the dial, and
//! [`crate::drive::SymbolicDials::off`] replays at plain `Interval` —
//! the pre-E12 driver, reproduced rather than approximated. (The path is
//! spelled in full because these module docs are merged with the OUTER
//! doc comment on `pub mod drive;` in `lib.rs`, so rustdoc resolves
//! their links in the crate root's scope rather than this module's.)
//!
//! **The f64 witness pass is untouched**, deliberately: a point residual
//! is tight, so it still catches a constructor that does not build what
//! it claims — the failure an expression that is identically zero on
//! paper could otherwise hide.
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
//! [`RefusalReason::FlipCrossing`], and the flipped predicates are
//! NAMED by `resolve::vdiff` — the verdict-diff engine this tree
//! declares built once — rather than by a second diff of this
//! module's own (no-flips v1: no branch enumeration, no analysis of
//! the far side). [`FlipEvidence`] carries the argument.
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
//! [`drive()`] takes `&Doc` and returns a value. There is no `&mut` in
//! its signature, no interior mutability on the path, and no
//! re-witnessing however clean a certificate comes out: a document
//! write is not something this API can express, which is a stronger
//! statement than one it does not perform.

use std::collections::BTreeMap;
use std::sync::Arc;

use geom_core::interval::Interval;
use geom_core::{MarginDiag, Sym, SymCounts, Tol};

#[cfg(feature = "probe")]
use crate::analysis::BoxAxis;
use crate::analysis::{AnalyzedBox, MeasureUnavailable, ParamBox};
use crate::doc::{Doc, ParamName};
use crate::eval::{
    CancelToken, ContentKey, EvalOptions, Evaluation, KeyHasher, NodeErrorKind, NodeResult,
    ProfileLift, evaluate,
};
use crate::node::{Node, RecipeNodeId};
use crate::program::ProfileProgram;
use crate::resolve::{FlipSet, diff_verdicts};
// The two derived verdict forms live in one module (`resolve::vdiff`);
// this driver is the strict form's certifying consumer, and names it at
// `drive::` because that is where every consumer already reaches for it.
pub use crate::resolve::{VerdictRow, VerdictVector, VerdictVectorKey};
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
/// ENFORCED AT ADMISSION, so it is a bound and not a target: a split
/// that would commit the drive past this number is refused instead of
/// taken, and `certified + refused` never exceeds it (except at
/// `max_leaves = 0`, where the root alone is one leaf and refusing it
/// is still one leaf — a subdivision has no empty answer).
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
    /// The symbolic identity tier (E12): whether the leaf replay carries
    /// parameter expressions, and how large a normal form may grow
    /// before it freezes.
    pub symbolic: SymbolicDials,
}

/// **The symbolic tier's dials** (E12, `geom_core::sym`).
///
/// # The defaults, and the argument for them
///
/// `enabled` is ON, because a certifier that can only certify boxes
/// narrower than its own ε is the state M10-3 pinned and E12 exists to
/// leave; the tier off is the comparison lane, not the shipped one.
///
/// [`DEFAULT_SYM_MAX_TERMS`] = 4096 and [`DEFAULT_SYM_MAX_DEGREE`] = 128
/// are the FREEZING budget, and both were MEASURED on this kernel's own
/// fixtures rather than picked.
///
/// **Degree is the binding dial; terms are not.** A certification
/// identity on an analytic carrier looks low-degree written down — the
/// plate's widest form is a squared distance in three symbols — but the
/// form the tier actually builds is not the written one. It is a
/// QUOTIENT of polynomials reached by repeated cross-multiplication, and
/// a metered extrusion contributes `‖w‖` and its reciprocal to every
/// term it touches, so the degree that matters is the degree AFTER
/// those denominators have been carried up the DAG. Measured on the
/// M10-3 slab, the endpoint identity the tier's headline row depends on
/// needs **degree ≥ 32** to cancel; at 16 it freezes and the row does
/// not move. 128 is the next power of two with real headroom above the
/// measurement, and it freezes NOTHING on that fixture. Terms never
/// bound anything measured: 64 sufficed at every degree tried, and 4096
/// is headroom against a NURBS-heavy product rather than a number any
/// fixture approached.
///
/// What the budget defends against is exactly that pathological product,
/// where a form grows multiplicatively with no identity at the end of
/// it; there the freeze costs a cancellation that was never going to
/// happen and saves the replay. The evidence for the numbers is the
/// FROZEN COUNT on the verdict: a corpus that freezes nothing has budget
/// to spare, and a corpus that freezes often is telling you to look at
/// the forms rather than to raise the dial. On curved geometry it
/// freezes for a different reason — `i128` coefficient overflow, not the
/// dials — and the unit's D6 carries that measurement.
///
/// `SymbolicDials::off()` reproduces the numeric-only replay bit for
/// bit: no session is installed, no node is minted, and the verdict's
/// serialization carries no symbolic line at all.
///
/// # What the tier COSTS, measured
///
/// `enabled` is on by default, so every `DriveConfig::default()` drive
/// pays this. The bill, release profile, one machine, from
/// `editor-core/tests/m10_7_probe_interval.rs::m10_7_what_the_tier_costs`
/// (run it rather than trusting the numbers — the ratios are what
/// travels):
///
/// | fixture | tier ON | tier OFF | |
/// | --- | --- | --- | --- |
/// | slab, macroscopic box, 32 leaves | 17.1 ms, **certifies** | 5.4 ms, certifies nothing | 3.2x |
/// | slab, macroscopic box, 256 leaves | 9.8 ms, **certifies** | 36.1 ms, certifies nothing | **0.27x** |
/// | filleted bracket (CURVED), 32 leaves | 43.1 ms, certifies nothing | 2.5 ms, certifies nothing | **17x** |
///
/// Three different answers, and the middle one is not a typo. Where the
/// tier CERTIFIES, it certifies in one leaf and the numeric lane
/// subdivides to its budget and fails, so a larger leaf budget makes the
/// tier the FASTER lane — the work it saves is the subdivision it makes
/// unnecessary. Where it cannot certify, it is pure overhead, and the
/// worst measured case is curved geometry: 17x for nothing, because the
/// arc family it cannot discharge (`work/m10/M10-8.md`) means the box
/// refuses either way.
///
/// Two things keep that bill down and both are measured rather than
/// argued. A margin the numeric channel has already proved NON-ZERO
/// never has its form built at all (`geom_core::sym`'s `Decide` impl —
/// a certified enclosure excluding zero is a proof no normal form can
/// contradict), which is most margins on most documents. And
/// `Poly::mul` refuses on pre-bounds instead of building a product and
/// discarding it, so an over-budget multiplication costs its two
/// operands' sizes rather than their product.
///
/// The residual worry is the curved case, and the honest statement is
/// that it is a real 17x paid for nothing on documents the tier cannot
/// help — which is an argument for M10-8, not for a dial.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SymbolicDials {
    /// Whether the leaf replay runs at the symbolic tier.
    pub enabled: bool,
    /// The most terms one normal form may hold before it freezes.
    pub max_terms: usize,
    /// The largest total degree one normal form may reach.
    pub max_degree: u32,
}

/// The shipped term budget ([`SymbolicDials`]).
pub const DEFAULT_SYM_MAX_TERMS: usize = 4096;

/// The shipped degree budget ([`SymbolicDials`]).
pub const DEFAULT_SYM_MAX_DEGREE: u32 = 128;

impl SymbolicDials {
    /// The tier off — the numeric-only replay, bit for bit.
    #[must_use]
    pub fn off() -> Self {
        Self {
            enabled: false,
            ..Self::default()
        }
    }

    /// The budget as the scalar's own type.
    fn budget(self) -> geom_core::SymBudget {
        geom_core::SymBudget {
            max_terms: self.max_terms,
            max_degree: self.max_degree,
        }
    }

    /// The replay lane these dials name — the currency every
    /// certified-leaf consumer takes ([`crate::eval::LeafLane`]).
    pub(crate) fn lane(self) -> crate::eval::LeafLane {
        if self.enabled {
            crate::eval::LeafLane::Symbolic(self.budget())
        } else {
            crate::eval::LeafLane::Numeric
        }
    }
}

impl Default for SymbolicDials {
    fn default() -> Self {
        Self {
            enabled: true,
            max_terms: DEFAULT_SYM_MAX_TERMS,
            max_degree: DEFAULT_SYM_MAX_DEGREE,
        }
    }
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
    ///
    /// **This is NARROWER than E6's sentence, which is "every
    /// driver-path predicate sample lands in the k_stats funnel", and
    /// the gap is disclosed rather than closed here.** Three
    /// narrowings, each with its reason:
    ///
    /// - **Certified leaves only.** A REFUSED leaf's margins are the
    ///   ones nearest a flip, so they are the more interesting half of
    ///   the population — but a refused leaf has no single parameter
    ///   point that represents it (its midpoint is a point the driver
    ///   proved nothing about, and for an indeterminate leaf the whole
    ///   question is that no point speaks for the box). Sampling one
    ///   anyway would put margins in K's distribution under a claim
    ///   the driver did not make. Widening this — a rule for which
    ///   point of a refused leaf to sample, argued rather than
    ///   assumed — is banked.
    /// - **A replay, not the drive itself.** The drive runs at
    ///   `Interval`; `Probe` is an `f64` scalar, so the driver's own
    ///   interval decisions cannot be recorded by it at all. What
    ///   reaches K is the f64 decision at a point the interval pass
    ///   certified around.
    /// - **Off by default, and only in a `probe` build.** The sink it
    ///   writes to does not exist otherwise, and the replay doubles a
    ///   certified leaf's evaluation cost.
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
            symbolic: SymbolicDials::default(),
        }
    }
}

/// **The CERTIFYING vector**: [`VerdictVector::of`] with the rows of
/// nodes that only report left out.
///
/// A free function and not a method on the strict form: the form is
/// `resolve::vdiff`'s, beside the population form it is the counterpart
/// of, and WHICH ROWS A GATE EXCLUDES is this driver's policy. One node
/// kind qualifies and it is [`Node::Assertion`], whose contract is that
/// nothing downstream reads its verdict — "no gate consults it" (E10
/// v1: assertions report; a gating mode is additive policy nobody has
/// ratified), and certification IS a gate. The measure node itself is
/// NOT dropped: a leaf where the measurement could not be taken is not
/// the witness build, and that difference stays in the comparison.
///
/// No row this returns carries [`crate::resolve::RunStatus::Absent`].
/// Every `evaluate` in this module runs on a fresh [`CancelToken`], so
/// no run here has a canceled prefix, and [`drive`] refuses
/// [`DriveRefusal::WitnessDoesNotBuild`] unless every node of the
/// witness's `order` is `Ok`. A consumer that takes
/// [`VerdictVector::of`] over some other evaluation can see `Absent`.
pub fn certifying_vector<T: geom_core::Decide, P>(
    doc: &Doc<P>,
    ev: &Evaluation<T>,
) -> VerdictVector {
    let mut vector = VerdictVector::of(ev);
    vector
        .rows
        .retain(|row| !matches!(doc.node(row.node), Some(Node::Assertion { .. })));
    vector
}

/// **The flip evidence a [`RefusalReason::FlipCrossing`] carries.**
///
/// The verdict half is [`diff_verdicts`]' answer VERBATIM — the engine
/// `resolve::vdiff` declares built ONCE, whose signature is already
/// generic over two different scalars (`Evaluation<T>` against
/// `Evaluation<U>`), which is exactly the f64-witness-against-
/// interval-leaf comparison this driver makes. Nothing here re-diffs.
///
/// **Why not a positional pairing**, which is what this driver first
/// shipped: `vdiff`'s own module docs rule it out, and the argument
/// applies here unchanged. Construction order inside an op is itself
/// steered by recorded exact-order predicates, so a legitimate flip can
/// permute the entire remaining decision sequence — and a positional
/// pairing then reports pure permutation noise as flips and misses the
/// real ones. The engine compares per-predicate sign POPULATIONS, which
/// are permutation-invariant; the residual is the net verdict change.
/// Its documented blind spot rides along with it: two instances of one
/// predicate trading opposite signs inside one node cancel.
///
/// **The blind spot cannot weaken a certificate**, because certification
/// does not consult this at all. A leaf certifies on EXACT
/// [`VerdictVector`] equality (that type's own docs say why the two
/// questions want two shapes); this evidence only NAMES a divergence
/// the strict comparison already found. The strict test gates; the
/// permutation-invariant engine explains.
#[derive(Debug, Clone, PartialEq)]
pub struct FlipEvidence {
    /// The verdict-diff engine's answer: per node, the net sign flips
    /// and the structurally-diverged predicates, plus each node's
    /// standing in the two runs.
    pub verdicts: FlipSet,
    /// Guided structure decisions the lane classified DEFINITELY
    /// otherwise. Not verdict rows — a consumed elaboration decision is
    /// not a `k_stats` verdict — so they are not the engine's business
    /// and ride beside its answer.
    pub structure: Vec<StructureFlip>,
}

impl FlipEvidence {
    /// Whether this evidence names anything at all.
    ///
    /// An empty evidence set is possible and is honest rather than a
    /// bug: the leaf's verdict VECTOR differs from the witness's (that
    /// is why it refused at all), and the population engine's blind
    /// spot — a pure sign exchange within one node — nets to nothing.
    /// The receipt prose says so rather than implying that a named
    /// cause always exists.
    pub fn is_empty(&self) -> bool {
        self.verdicts.is_empty() && self.structure.is_empty()
    }
}

/// A GUIDED structure decision the lane classified definitely
/// otherwise: this binding provably leaves the nominal elaboration's
/// structure. Carried unaltered, so the refusal names the decision the
/// profile lift itself named.
#[derive(Debug, Clone, PartialEq)]
pub struct StructureFlip {
    /// The profile node.
    pub node: RecipeNodeId,
    /// The lift's refusal, verbatim.
    pub refusal: Box<profile::StructureRefusal>,
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
        flipped: Box<FlipEvidence>,
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
    /// **A measure could not be taken, for a reason the BOX cannot
    /// change** (M10-6, R1's MINOR-6): a selection that names the wrong
    /// kind of entity, a pairing the wedge rule empties, a carrier the
    /// engine does not support.
    ///
    /// These are facts about the DOCUMENT, not about the parameter
    /// values, so every sub-box inherits them exactly. Before this
    /// class they fell to the catch-all `Bisect` and the driver split
    /// its way through the whole leaf budget re-deriving the same
    /// refusal — a measured >60 s on a `NoAdmittedPair` fixture — and
    /// then priced the mass as `Budget`, which named the symptom
    /// instead of the cause. Refusing terminally is both faster and
    /// truer: the mass is refused under its own class, and the class
    /// carries the engine's own name for the refusal.
    MeasureRefused {
        /// The measure node that could not be taken.
        node: RecipeNodeId,
        /// The engine's or the wiring's own class name for it.
        class: &'static str,
    },
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
            Self::MeasureRefused { .. } => ReasonClass::MeasureRefused,
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
    /// [`RefusalReason::MeasureRefused`].
    MeasureRefused,
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
            Self::MeasureRefused => "measure_refused",
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
    /// How this leaf's decisions were answered — the E12 receipt
    /// ([`SymbolicDials`]). All zero when the tier is off, because no
    /// session exists to count in.
    pub decisions: SymCounts,
}

/// A leaf the driver refused, and why.
#[derive(Debug, Clone, PartialEq)]
pub struct RefusedLeaf {
    /// The leaf's parameter box.
    pub box_: ParamBox,
    /// The typed reason.
    pub reason: RefusalReason,
    /// How this leaf's decisions were answered before it refused — the
    /// E12 receipt ([`SymbolicDials`]).
    pub decisions: SymCounts,
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
    decisions: SymCounts,
    symbolic: SymbolicDials,
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

    /// **The E12 receipt**: how this drive's predicate decisions were
    /// answered, summed over every leaf — `symbolic_zero` against
    /// `numeric`, with `frozen` beside them.
    ///
    /// All zero when the symbolic tier is off ([`SymbolicDials::off`]),
    /// which is not a claim that nothing decided: with no session
    /// installed there is nothing counting, and the verdict says so by
    /// omitting the line rather than by reporting zeros as data.
    pub fn decisions(&self) -> SymCounts {
        self.decisions
    }

    /// The witness build's CERTIFYING verdict vector — the thing every
    /// certified leaf's vector was compared against, shipped once.
    ///
    /// [`certifying_vector`]'s, not [`VerdictVector::of`]'s: it
    /// carries no `Assertion` row, because a report node's verdict is
    /// not part of what certification means. A consumer who wants the
    /// whole vector of an evaluation takes `of` over that evaluation.
    pub fn witness_vector(&self) -> &VerdictVector {
        &self.witness_vector
    }

    /// The box that was driven.
    pub fn root(&self) -> &ParamBox {
        &self.root
    }

    /// **The tier this drive ran at** ([`SymbolicDials`]).
    ///
    /// Shipped on the verdict because every consumer that REPLAYS a
    /// certified leaf — a stackup's content tie, a measure hull, a
    /// histogram row, an assertion read-back — has to replay it the way
    /// the driver certified it. A leaf certified with the symbolic tier
    /// on can carry a node that a numeric-only replay refuses, and a
    /// consumer that replayed at the wrong tier would report the
    /// verdict as "not of this build" rather than reading it.
    pub fn symbolic(&self) -> SymbolicDials {
        self.symbolic
    }

    /// The replay lane [`Self::symbolic`] names — the currency
    /// `crate::eval::replay_leaf` takes.
    pub(crate) fn lane(&self) -> crate::eval::LeafLane {
        self.symbolic.lane()
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
        // The symbolic tier's receipt, and ONLY when there is one: a
        // drive with the tier off installs no session, counts nothing,
        // and serializes exactly the text it serialized before E12 —
        // which is what makes the tier-off differential a byte
        // comparison rather than a filtered one.
        if self.decisions != SymCounts::default() {
            let _ = writeln!(
                s,
                "decisions symbolic_zero={} numeric={} frozen={}",
                self.decisions.symbolic_zero, self.decisions.numeric, self.decisions.frozen
            );
        }
        let _ = write!(s, "{}", self.accounting.serialize());
        s
    }

    /// **The human form** (M10-6 §2), beside the goldening one and
    /// never instead of it: leaf counts as counts, masses as
    /// percentages, the tail on its own line, and the
    /// priced-or-forced basis stated rather than assumed.
    ///
    /// It takes the analyzed box because the BASIS is a property of
    /// the box's distributions, not of the verdict: a drive over a
    /// band-only box produces exactly the same masses as one over a
    /// uniform box, and only the box knows that none of them is a
    /// probability ([`crate::report::MassBasis`]).
    pub fn render(&self, analyzed: &AnalyzedBox) -> String {
        use core::fmt::Write as _;
        let mut s = String::new();
        let r = self.receipt;
        let _ = writeln!(
            s,
            "drive over {} axis/axes: {} certified, {} refused ({} splits; receipt {})",
            self.root.axes().len(),
            r.certified,
            r.refused,
            r.splits,
            if r.holds() { "holds" } else { "BROKEN" }
        );
        let mut classes: BTreeMap<&'static str, usize> = BTreeMap::new();
        for leaf in &self.refused {
            *classes.entry(leaf.reason.class().name()).or_default() += 1;
        }
        for (class, n) in &classes {
            let _ = writeln!(s, "  {n} leaf/leaves refused {class}");
        }
        let d = self.decisions;
        if d != SymCounts::default() {
            let total = d.decisions();
            let share = if total == 0 {
                0.0
            } else {
                100.0 * (d.symbolic_zero as f64) / (total as f64)
            };
            let _ = writeln!(
                s,
                "  {} of {total} decisions were symbolic identities ({share:.1}%); \
                 {} form(s) frozen",
                d.symbolic_zero, d.frozen
            );
        }
        let _ = write!(
            s,
            "{}",
            crate::report::MassBudget::of(&self.accounting, analyzed).render()
        );
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
///
/// Bare public fields, where [`ParamBoxVerdict`] keeps its own behind
/// accessors, and the difference is deliberate: a verdict has an
/// INVARIANT its accessors protect (the receipt identity ties its two
/// leaf lists together, so handing out `&mut` to one of them would let
/// a caller break it), while this is a record of four independently
/// meaningful numbers with no relation to enforce between them. A
/// consumer builds one to ask [`Self::total`] what it composes to —
/// which is exactly what the accounting probe does — and no accessor
/// would add a guarantee.
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
    /// **Every column here is an UNCONDITIONAL mass** — a share of the
    /// whole measure, not a share of the analyzed box.
    ///
    /// A leaf is priced by [`crate::analysis::box_mass`], which answers
    /// "the mass this distribution puts INSIDE `sub`" under the
    /// parameter's own law; nothing divides by the analyzed box's mass
    /// anywhere on the path. So the leaves already sum to
    /// `∏(1 - tᵢ) = 1 - tail`, and the tail is E2's "explicit ADDITIVE
    /// term" rather than a factor. Composition is therefore addition,
    /// and the two doors below are the only place it happens.
    ///
    /// This sentence is load-bearing and was wrong once: the columns
    /// were composed as `t·(1 - tail) + tail`, which is the right
    /// arithmetic for CONDITIONAL columns and understates a normal
    /// axis's total by `tail·(1 - tail)` — 0.27% at the ±3σ default
    /// policy, and it understated the unresolved budget by the whole
    /// tail, which is the unsafe direction for E10's honesty gate.
    ///
    /// Certified + every refused column + tail. On a drive whose
    /// columns all price, this is 1 up to the `f64` error of summing
    /// the leaf masses. Refuses when any column refuses — a total over
    /// an unpriceable column is not a total.
    ///
    /// # Errors
    ///
    /// The first [`MeasureUnavailable`] among the columns.
    pub fn total(&self) -> Result<f64, MeasureUnavailable> {
        let mut t = self.certified.clone()?;
        for m in self.refused.values() {
            t += m.clone()?;
        }
        Ok(t + self.unanalyzed.clone()?)
    }

    /// The **unresolved-mass budget** (E2/E10's single honesty gate):
    /// refused mass plus tail, both unconditional (see [`Self::total`]).
    /// `containment` says whether it is exact or conservative.
    ///
    /// # Errors
    ///
    /// The first [`MeasureUnavailable`] among the refused columns or
    /// the tail.
    pub fn unresolved(&self) -> Result<f64, MeasureUnavailable> {
        let mut unresolved = self.unanalyzed.clone()?;
        for m in self.refused.values() {
            unresolved += m.clone()?;
        }
        Ok(unresolved)
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
    /// **The symbolic tier and the clearance engine do not compose**
    /// (E12's unit, deviation D3; issue `symbolic-tier-and-clearance-engine`).
    ///
    /// `min_clearance` answers with an enclosure computed by
    /// [`crate::clearance`]'s engine, which is written at
    /// [`geom_core::Interval`] concretely — it borrows a
    /// `&Body<Interval>` — so it cannot be handed the leaf replay's
    /// `Body<Sym<Interval>>`, and no scalar remap of a body exists to
    /// strip one. The lane therefore has no clearance answer at all.
    ///
    /// Refused up front rather than degraded: the trait's honest `None`
    /// reads downstream as the typed absence
    /// [`crate::eval::ValuePayload::MeasureUnavailable`], which is a
    /// VALUE, so the leaf would certify with the clearance measure
    /// silently missing from it. That is precisely the quiet degradation
    /// this kernel refuses, so the drive says so instead. The recourse
    /// is in the message: drive this document with
    /// [`SymbolicDials::off`].
    SymbolicClearanceUnsupported {
        /// The measure node whose primitive has no lane.
        node: RecipeNodeId,
    },
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
            Self::SymbolicClearanceUnsupported { node } => write!(
                f,
                "node {} measures a `min_clearance`, whose engine has no lane at the symbolic \
                 identity tier — drive with `DriveConfig {{ symbolic: SymbolicDials::off(), .. }}` \
                 to get the numeric-only answer, or measure a closed form",
                node.0
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
    if config.symbolic.enabled
        && let Some(node) = clearance_measure(doc)
    {
        return Err(DriveRefusal::SymbolicClearanceUnsupported { node });
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
    let witness_vector = Arc::new(certifying_vector(doc, &witness));
    let witness_key = witness_vector.key();
    // The witness EVALUATION stays alive for the whole drive, not just
    // long enough to take its vector: `diff_verdicts` names a leaf's
    // flips against it, and the engine takes the two evaluations rather
    // than two digests of them. One build's worth of geometry, held
    // once per drive.

    let mut certified: Vec<CertifiedLeaf> = Vec::new();
    let mut refused: Vec<RefusedLeaf> = Vec::new();
    let mut splits = 0usize;
    let mut decisions = SymCounts::default();

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
                // A box the budget refuses before it is replayed decided
                // nothing, so its receipt is empty rather than absent.
                refused.push(RefusedLeaf {
                    box_: b.box_,
                    reason: RefusalReason::Budget(BudgetKind::Leaves {
                        max_leaves: config.max_leaves,
                    }),
                    decisions: SymCounts::default(),
                });
            }
            break;
        }
        let leaf = |b: &Box_| {
            classify(
                doc,
                &b.box_,
                &witness,
                &witness_vector,
                witness_key,
                config.symbolic,
                tol,
            )
        };
        let verdicts: Vec<(LeafVerdict, SymCounts)> = if config.parallel {
            use rayon::prelude::*;
            frontier.par_iter().map(leaf).collect()
        } else {
            frontier.iter().map(leaf).collect()
        };
        let mut next = Vec::new();
        let level = frontier.len();
        for (i, (b, (v, counts))) in frontier.drain(..).zip(verdicts).enumerate() {
            decisions.absorb(counts);
            match v {
                LeafVerdict::Certified(leaf) => certified.push(leaf),
                LeafVerdict::Refused(reason) => refused.push(RefusedLeaf {
                    box_: b.box_,
                    reason,
                    decisions: counts,
                }),
                // The budget is enforced AT ADMISSION, not after the
                // fact: splitting turns one box into two, so the split
                // is refused unless the leaf count it commits to still
                // fits. The lower bound counts what is already final
                // (`certified + refused`), what this level has left to
                // fold (each of which becomes at least one leaf), what
                // is already queued for the next level, and the two
                // halves this split would add.
                //
                // Checking after the level instead admitted a frontier
                // of exactly `max_leaves` and then split every box in
                // it, overshooting the bound by up to 2x. The receipt
                // held either way — nothing was ever dropped — but
                // "how many leaves one drive may produce" has to be
                // the number it says.
                LeafVerdict::Bisect => {
                    let pending = level - i - 1;
                    let committed = certified.len() + refused.len() + pending + next.len();
                    if committed + 2 > config.max_leaves {
                        refused.push(RefusedLeaf {
                            box_: b.box_,
                            reason: RefusalReason::Budget(BudgetKind::Leaves {
                                max_leaves: config.max_leaves,
                            }),
                            decisions: counts,
                        });
                    } else {
                        match bisect(&b, &root, config.max_depth) {
                            Ok((a, c)) => {
                                splits += 1;
                                next.push(a);
                                next.push(c);
                            }
                            Err(kind) => refused.push(RefusedLeaf {
                                box_: b.box_,
                                reason: RefusalReason::Budget(kind),
                                decisions: counts,
                            }),
                        }
                    }
                }
            }
        }
        frontier = next;
    }

    #[cfg(feature = "probe")]
    if config.k_probe == KProbe::CertifiedMidpoints {
        for leaf in &certified {
            probe_midpoint(doc, &leaf.box_, config.symbolic, tol);
        }
    }

    let receipt = Receipt {
        certified: certified.len(),
        refused: refused.len(),
        splits,
    };
    // The receipt identity, checked on EVERY drive — D9 row 5's
    // tripwire class, and a tripwire rather than a proof by the same
    // rule's clause (i): nothing typed rides on it, and the identity
    // itself is shipped on the verdict, so a consumer re-checks it
    // without trusting this module ([`Receipt::holds`]).
    //
    // Calibration (clause iii): the population is every box the loop
    // above pops, and the margin is zero — this is an exact integer
    // identity over a binary tree, not an estimated ceiling. Each box
    // is certified, refused, or split into exactly two, so a firing
    // assertion means a box escaped its bucket, which is a bug in this
    // module and cannot be caused by any document. Every drive in
    // `m10_3_driver_interval.rs` asserts it on the shipped value too,
    // which is what keeps it checked in a release build.
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
        decisions,
        symbolic: config.symbolic,
        witness_vector,
        root,
    })
}

/// The first `Measure` node reading a `min_clearance` primitive, if
/// the document has one ([`DriveRefusal::SymbolicClearanceUnsupported`]).
fn clearance_measure(doc: &Doc<ProfileProgram>) -> Option<RecipeNodeId> {
    doc.order().iter().copied().find(|&id| {
        let Some(Node::Measure { expr, .. }) = doc.node(id) else {
            return false;
        };
        let mut prims = Vec::new();
        expr.primitives(&mut prims);
        prims
            .iter()
            .any(|p| matches!(p, crate::measure::MeasurePrimitive::MinClearance { .. }))
    })
}

/// The evaluation options every pass of a drive uses: the profile lift
/// ON (so profile geometry is a function of the leaf's parameters and
/// every structure decision is re-verified at the lane), sequential
/// (leaf-level parallelism is the driver's, and nesting a rayon scope
/// per leaf inside one buys nothing), and no memo.
///
/// **Crate-visible because the clearance engine replays INSIDE a leaf**
/// ([`crate::clearance`]) and must replay it the way this driver
/// certified it. Two copies of these options are two lanes: a clearance
/// query whose lift setting drifted from the driver's would be
/// certifying a body other than the one the leaf's verdict vector is
/// about.
///
/// The SCALAR is the other half of "the way this driver certified it",
/// and it does not ride here: it rides on the verdict
/// ([`ParamBoxVerdict::symbolic`]) and reaches a consumer through
/// `crate::eval::replay_leaf`. The clearance engine is the one consumer
/// that cannot take it — its selection type is written at `Interval`
/// concretely — which is why a document carrying a `min_clearance`
/// measure refuses the symbolic tier up front
/// ([`DriveRefusal::SymbolicClearanceUnsupported`]) rather than
/// certifying leaves the engine could not then read.
pub(crate) fn lane_opts() -> EvalOptions {
    // `EvalOptions::default()` already mints an epoch; minting a second
    // one to overwrite it burnt a process-global counter per leaf, and
    // a drive is tens of thousands of leaves. The epoch is a
    // stale-result discrimination token for a caller holding several
    // in-flight evaluations, which a drive is not: each leaf's result
    // is consumed before the next is asked for.
    EvalOptions {
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

/// The leaf protocol: replay this box at the configured lane scalar and
/// classify, answering the verdict beside the leaf's own E12 receipt.
///
/// **Which scalar, and why the choice is here rather than inside**
/// (E12). With the symbolic tier on, the replay runs at
/// `Sym<Interval>` inside a fresh [`geom_core::sym::with_session`]: the
/// numeric channel is `Interval`'s, verbatim and bit for bit, and the
/// tier adds one thing — a margin whose expression is identically zero
/// in the document's parameters decides `Zero` without consulting its
/// enclosure. With the tier off the replay is the plain `Interval` one,
/// which is why `SymbolicDials::off()` is a byte-exact reproduction of
/// the pre-E12 driver rather than an approximation of it.
///
/// The session is per-CALL, so its hash-consing table is per leaf and is
/// dropped with the leaf; under the parallel schedule each leaf runs
/// wholly on one rayon thread, and the session is thread-local, so no
/// table is ever shared. Node ids are content hashes, so the two
/// schedules build identical DAGs anyway (D9).
///
/// **The f64 witness pass is untouched.** A point residual is tight, so
/// the witness still catches a constructor that does not build what it
/// claims — which is exactly the failure the symbolic tier could
/// otherwise hide, since an expression that is identically zero on
/// paper says nothing about whether the code computed it.
fn classify(
    doc: &Doc<ProfileProgram>,
    box_: &ParamBox,
    witness: &Evaluation<f64>,
    witness_vector: &VerdictVector,
    witness_key: VerdictVectorKey,
    symbolic: SymbolicDials,
    tol: Tol,
) -> (LeafVerdict, SymCounts) {
    let opts = EvalOptions {
        param_box: Some(Arc::new(box_.clone())),
        ..lane_opts()
    };
    if symbolic.enabled {
        let (leaf, counts) = geom_core::sym::with_session(symbolic.budget(), || {
            let leaf: Evaluation<Sym<Interval>> =
                evaluate(doc, None, &CancelToken::new(), &opts, tol);
            leaf
        });
        return (
            classify_replay(
                doc,
                box_,
                &leaf,
                witness,
                witness_vector,
                witness_key,
                counts,
            ),
            counts,
        );
    }
    let leaf: Evaluation<Interval> = evaluate(doc, None, &CancelToken::new(), &opts, tol);
    let counts = SymCounts::default();
    (
        classify_replay(
            doc,
            box_,
            &leaf,
            witness,
            witness_vector,
            witness_key,
            counts,
        ),
        counts,
    )
}

/// The classification itself, over an already-replayed leaf — generic
/// in the lane scalar at `Decide`, the weakest bound its reads need,
/// because the two lanes above differ only in which scalar produced the
/// evaluation and in nothing this function looks at.
fn classify_replay<T: geom_core::Decide>(
    doc: &Doc<ProfileProgram>,
    box_: &ParamBox,
    leaf: &Evaluation<T>,
    witness: &Evaluation<f64>,
    witness_vector: &VerdictVector,
    witness_key: VerdictVectorKey,
    decisions: SymCounts,
) -> LeafVerdict {
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
    //
    // HOW IT ASKS, AND IN WHAT ORDER. Three reads per node, and the
    // order is a rule: (1) a DEFINITE box-independent refusal is
    // terminal whatever else the op recorded — bisection cannot change
    // a fact about the document — so it is read first; (2) the node's
    // escalation log decides among the box-DEPENDENT outcomes: every
    // indeterminate outcome the FUNNEL produced while the op ran is on
    // it — on the value if the op built anyway, on the error if it did
    // not — so an escalation an op wrapped in its own error enum (a
    // sweep's `ExtrusionEscalated`, ~40 such variants across five
    // crates) is seen without matching on any of them, and the FIRST
    // escalation in decision order speaks: a sliver, or the cue to
    // bisect (a later sliver behind a refinable escalation does not
    // argue — refinement may never reach it on the branch a definite
    // first decision takes — so that order is the conservative one);
    // (3) the error-enum arms. Those arms are LOAD-BEARING, not a
    // fallback: the log carries the funnel's escalations only, and a
    // predicate that asks the funnel, gets a definite sign, and then
    // mints an `Indeterminate` of its own (`geom_brep::enters`,
    // `dihedral`, `pcurve_cache`, `certify`, `edge_nurbs`, `ssi::march`
    // — eight sites) reaches this loop only through the enum it was
    // wrapped in; the whole-document mate solve's escalations likewise
    // arrive only as `NodeErrorKind::Mate`, since no node's bracket is
    // open when it runs. The gap and the unit that closes it:
    // `work/props/escalation-channel-misses-op-minted-indeterminates.md`.
    // ITERATION ORDER IS NODE ID, and where a leaf carries several
    // refusing nodes that decides which one speaks: the FIRST
    // indeterminacy in node-id order settles the leaf as a sliver or a
    // bisect, and a definite structure flip at a later node never gets
    // to argue. Deterministic (a `BTreeMap` walk, same in both
    // schedules) but arbitrary — node id is a minting counter, not a
    // ranking of causes. It is stated rather than defended because
    // nothing downstream depends on which cause wins: every arm here
    // is refused mass either way, and the receipt does not change.
    let mut structure_flips = Vec::new();
    for (&node, result) in &leaf.nodes {
        let (escalations, failure) = match result {
            NodeResult::Ok(v) => (&v.escalations, None),
            NodeResult::Failed(e) => (&e.escalations, Some(e)),
            NodeResult::Poisoned { .. } => continue,
        };
        // (1) **Box-independent measure refusals are TERMINAL** (M10-6,
        // R1's MINOR-6). A selection naming the wrong kind of entity,
        // or a pairing the wedge rule empties, is a fact about the
        // document: every sub-box inherits it exactly, so refining
        // re-derives the same refusal until the budget runs out and
        // then prices the mass `Budget`, naming the symptom. The
        // classes listed in `box_independent_measure_class` are the
        // ones that provably cannot move under refinement; a clearance
        // refusal that CAN (a cell budget, a poisoned enclosure, an
        // unverified witness) still bisects, because for those a
        // smaller box is exactly the remedy. Read BEFORE the log: an
        // escalation the same op also recorded changes nothing about
        // a refusal no box can move.
        if let Some(err) = failure
            && let Some(class) = box_independent_measure_class(&err.kind)
        {
            return LeafVerdict::Refused(RefusalReason::MeasureRefused { node, class });
        }
        // (2) The escalation log.
        if let Some(first) = escalations.first() {
            return indeterminate(&first.source);
        }
        // (3) The error-enum arms.
        let Some(err) = failure else {
            continue;
        };
        match &err.kind {
            NodeErrorKind::Escalated { source, .. } => return indeterminate(source),
            NodeErrorKind::ProfileLaneReplay {
                structure: Some(refusal),
                ..
            } => match &refusal.kind {
                profile::StructureRefusalKind::Indeterminate(source) => {
                    return indeterminate(source);
                }
                profile::StructureRefusalKind::Flipped { .. } => {
                    structure_flips.push(StructureFlip {
                        node,
                        refusal: Box::new(refusal.clone()),
                    });
                }
            },
            _ => return LeafVerdict::Bisect,
        }
    }

    // (ii) The comparison. EXACT, on the CERTIFYING verdict vector —
    // never a width, and never a report node
    // ([`certifying_vector`]).
    let vector = certifying_vector(doc, leaf);
    if structure_flips.is_empty() && vector == *witness_vector {
        return LeafVerdict::Certified(CertifiedLeaf {
            box_: box_.clone(),
            verdict_vector_key: witness_key,
            decisions,
            results: LeafResults {
                node_keys: leaf
                    .order
                    .iter()
                    .filter_map(|&id| leaf.value(id).map(|v| (id, v.content_key)))
                    .collect(),
            },
        });
    }
    // NAMED by the built-once engine, not by a second diff of our own
    // (see `FlipEvidence`): the two evaluations go in and its
    // `FlipSet` comes out. What this module then does is DROP THE
    // NODES the comparison above does not read.
    //
    // **Why the evidence has to be filtered the same way** (M10-6,
    // both reviews). `certifying_vector` retains no `Assertion` row, so an
    // assertion's `assert_bound` flip cannot be why this leaf refused
    // — the comparison never looked at it. Left in, the evidence names
    // a predicate that did not cause the refusal, on exactly the
    // documents E10 exists for: a `min_clearance` assertion is
    // `Unevaluated` at every f64 witness and definite over a certified
    // leaf, so it flips on EVERY leaf and would head the flip list of
    // every unrelated refusal.
    //
    // It is a projection of the engine's answer onto the nodes the
    // question is about, not a second diff: the engine still runs over
    // the whole evaluation, its per-node deltas come back unaltered,
    // and the filter is the SAME predicate `certifying_vector` uses, spelled
    // once here so the two cannot drift.
    let mut verdicts = diff_verdicts(witness, leaf);
    verdicts
        .nodes
        .retain(|id, _| !matches!(doc.node(*id), Some(Node::Assertion { .. })));
    LeafVerdict::Refused(RefusalReason::FlipCrossing {
        flipped: Box::new(FlipEvidence {
            verdicts,
            structure: structure_flips,
        }),
    })
}

/// What one escalation makes of a leaf: a terminal sliver when its
/// enclosure sits wholly inside the band ([`sliver`]), otherwise the
/// cue to bisect. One spelling for the three reads of `classify_replay`.
fn indeterminate(source: &geom_core::Indeterminate) -> LeafVerdict {
    sliver(source).map_or(LeafVerdict::Bisect, |predicate| {
        LeafVerdict::Refused(RefusalReason::SliverTerminal { predicate })
    })
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
/// **Crate-visible because the clearance engine's inner subdivision
/// refuses by the same rule** ([`crate::clearance`]): a cell pair whose
/// separation margin sits wholly inside the band is terminal for
/// exactly this reason — interval enclosures shrink monotonically under
/// subdivision, so a sub-cell's enclosure stays inside the band its
/// parent's was inside. One home, so the two subdivisions cannot drift
/// apart on what a sliver is.
pub(crate) fn sliver(source: &geom_core::Indeterminate) -> Option<&'static str> {
    let MarginDiag::Enclosure { lo, hi } = source.margin else {
        // A point margin (an `f64` lane) or an invalid one says nothing
        // about a box.
        return None;
    };
    let (zero, escalate) = (source.band.zero(), source.band.escalate());
    let inside = (zero < lo && hi < escalate) || (-escalate < lo && hi < -zero);
    inside.then_some(source.predicate.unwrap_or("<unnamed>"))
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
///
/// **ONE parameter, not all of them**, where E2 says a report over a
/// band refuses "naming the Band params". The narrowing is in the
/// carried type, not here: [`MeasureUnavailable`] is a single-parameter
/// refusal, and widening it to a set is a change to the analysis lane's
/// vocabulary that M10-1 owns. What a consumer loses is a list; what it
/// keeps is a true statement and a name to act on — and every band
/// parameter is visible without this door at all, by walking
/// [`crate::analysis::AnalyzedBox::params`] for
/// [`crate::Distribution::Band`]. Recorded as a deviation on the unit
/// rather than papered over with a first-of-several presented as the
/// whole set.
fn add_mass(column: &mut Result<f64, MeasureUnavailable>, m: Result<f64, MeasureUnavailable>) {
    if let Ok(acc) = column {
        match m {
            Ok(v) => *acc += v,
            Err(e) => *column = Err(e),
        }
    }
}

/// The mass OUTSIDE the analyzed box under the product measure —
/// UNCONDITIONAL, like every other column ([`MeasureAccounting::total`]
/// states why that matters). The leaves inside the box sum to
/// `∏(1 - tᵢ)` and this is exactly its complement, so the two ADD to 1.
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
fn probe_midpoint(doc: &Doc<ProfileProgram>, box_: &ParamBox, symbolic: SymbolicDials, tol: Tol) {
    // THE SAME MIDPOINT THE SPLIT RULE USES, through the same door
    // (`BoxAxis::midpoint`). Writing `0.5 * (lo + hi)` here as well
    // would let a change to the split rule silently detach the K
    // population from the leaves it is supposed to describe: these are
    // the points the driver certified AROUND, and "around" is defined
    // by where it split.
    let mid: BTreeMap<ParamName, BoxAxis> = box_
        .axes()
        .iter()
        .map(|(n, a)| {
            let m = a.midpoint();
            (n.clone(), BoxAxis::Varying { lo: m, hi: m })
        })
        .collect();
    let opts = EvalOptions {
        param_box: Some(Arc::new(ParamBox::from_axes(mid))),
        ..lane_opts()
    };
    // The replay runs at the SAME TIER the drive did (E12): with the
    // tier on the recording scalar is `Sym<Probe>`, so a margin that is
    // an identity lands in the funnel as `SampleOutcome::SymbolicZero`
    // and the hosted K row reports the symbolic/numeric split of the
    // driver's own population. Running it at bare `Probe` would report a
    // population the driver did not produce.
    if symbolic.enabled {
        let _ = geom_core::sym::with_session(symbolic.budget(), || {
            let ev: Evaluation<Sym<geom_core::Probe>> =
                evaluate(doc, None, &CancelToken::new(), &opts, tol);
            ev
        });
        return;
    }
    let _: Evaluation<geom_core::Probe> = evaluate(doc, None, &CancelToken::new(), &opts, tol);
}

/// A box's goldening rendering: `name=[lo_bits,hi_bits]` per axis, in
/// name order, floats as exact bits.
pub(crate) fn render_box(b: &ParamBox) -> String {
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
        RefusalReason::MeasureRefused { node, class } => {
            format!("measure_refused {} {class}", node.0)
        }
        RefusalReason::FlipCrossing { flipped } => {
            let mut s = String::from("flip_crossing");
            // Deterministic by construction: `FlipSet` is a `BTreeMap`
            // by node id, and each node's flips and divergences are
            // already sorted by the engine.
            for (node, delta) in &flipped.verdicts.nodes {
                if delta.old_status != delta.new_status {
                    let _ = write!(
                        s,
                        " {}:status:{:?}->{:?}",
                        node.0, delta.old_status, delta.new_status
                    );
                }
                for f in &delta.flips {
                    let _ = write!(
                        s,
                        " {}:{}:{:?}->{:?}x{}",
                        node.0, f.predicate, f.from, f.to, f.count
                    );
                }
                for d in &delta.diverged {
                    let _ = write!(
                        s,
                        " {}:{}:count {}->{}",
                        node.0, d.predicate, d.old_count, d.new_count
                    );
                }
            }
            for f in &flipped.structure {
                let _ = write!(s, " {}:structure:{:?}", f.node.0, f.refusal.decision);
            }
            if flipped.is_empty() {
                // The population engine's blind spot netted to nothing.
                // Said out loud in the goldening form, because a bare
                // `flip_crossing` would read as "no evidence gathered"
                // rather than "the evidence cancelled".
                s.push_str(" unnamed");
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

/// **The recorded requirement's verdict over one certified leaf** —
/// the number E10 says a CI row gates on, through a door rather than
/// through a hand-assembled evaluation.
///
/// A consumer with a `ParamBoxVerdict` in hand has the leaves; what it
/// did not have until M10-6's review was any way to ask what the
/// assertion says over one, short of rebuilding `EvalOptions`, picking
/// the interval scalar and matching on `ValuePayload` — three steps in
/// which the easy mistake is to skip the node entirely and compare
/// `worst_case.lo` against the bound by hand. That comparison is an
/// `f64` `<` over quantities that may differ by less than the run's own
/// coincidence threshold, and it manufactures exactly the certainty the
/// band exists to deny. The tour's tolerance cell made it; this door is
/// what it should have called.
///
/// `None` when `assertion` is not an `Assertion` node, or the
/// evaluation over `box_` did not produce a verdict for it.
///
/// The scalar is the interval one, not a choice: a verdict over a BOX
/// is only meaningful from an enclosure, and the point scalars cannot
/// produce one.
pub fn assertion_at(
    doc: &Doc<ProfileProgram>,
    assertion: RecipeNodeId,
    box_: &ParamBox,
    symbolic: SymbolicDials,
    tol: Tol,
) -> Option<crate::measure::AssertionVerdict<geom_core::Interval>> {
    if !matches!(doc.node(assertion), Some(Node::Assertion { .. })) {
        return None;
    }
    // The SAME LANE the leaf was certified on ([`ParamBoxVerdict::symbolic`]).
    crate::eval::replay_leaf(
        doc,
        &EvalOptions {
            param_box: Some(Arc::new(box_.clone())),
            ..lane_opts()
        },
        symbolic.lane(),
        &crate::eval::LeafPrior::None,
        crate::eval::LeafRequest {
            assertion: Some(assertion),
            ..crate::eval::LeafRequest::default()
        },
        tol,
    )
    .assertion
}

/// The measure-refusal classes a smaller box cannot change
/// ([`RefusalReason::MeasureRefused`]).
///
/// Conservative by construction: a class is here only when refinement
/// PROVABLY cannot alter it, and everything else keeps bisecting. The
/// cost of being wrong in this direction is a leaf refused early
/// (visible, priced under its own name); the cost of being wrong the
/// other way is a leaf that could have certified and did not, which is
/// why the list is enumerated rather than defaulted.
fn box_independent_measure_class(kind: &NodeErrorKind) -> Option<&'static str> {
    match kind {
        // The selection resolved to the wrong KIND of entity. Document
        // structure; no parameter value moves it.
        NodeErrorKind::MeasureSelectionKind { .. } => Some("selection_kind"),
        NodeErrorKind::MeasureClearanceRefused(r) => match r.class {
            // Which faces are admitted, whether the two scopes pair at
            // all, and whether the carrier has an implementation: all
            // decided by the document's own topology and the engine's
            // support table, not by the box.
            c @ ("no_admitted_pair" | "unsupported" | "selection" | "empty_scope"
            | "not_a_distance") => Some(c),
            // `budget`, `sliver`, `poison_enclosure`, `witness_unverified`,
            // `nothing_certified`, `tolerance_has_no_band`: every one of
            // these can differ over a smaller box, so refinement is the
            // right answer and the catch-all keeps it.
            _ => None,
        },
        _ => None,
    }
}
