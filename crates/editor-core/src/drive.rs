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
//!    checks and never places (D10), so its rows are not in the
//!    comparison. An assertion the leaf could not decide is still an
//!    indeterminacy, and bisects under 1.
//!
//! **What a certified leaf has not had** (Ev on PR 6371, the box
//! scalars' switch at `topo::AtRestPolicy::leaf`): at the box scalars a
//! sweep door's body is tier 2 alone, and tier 3 runs only where a
//! consumer takes it as a finished body (a Boolean's, a split's, a
//! shell's or a blend's operand, the product). So a leaf whose document
//! is sweeps alone has had tier 2, not tier 3, over its box, until the
//! switch's rows land (`work/boxfinish/delete-the-box-scalar-sweep-leaf-switch.md`).
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
//! [`RefusalReason::FlipCrossing`], and the decision that turned is
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

use crate::say;
use std::collections::{BTreeMap, BTreeSet};
use std::sync::Arc;

use geom_core::interval::Interval;
use geom_core::sym;
use geom_core::{Sym, SymCounts, Tol};

#[cfg(feature = "probe")]
use crate::analysis::BoxAxis;
use crate::analysis::{AnalyzedBox, MeasureUnavailable, ParamBox};
use crate::doc::Doc;
use crate::eval::{
    ContentKey, EvalOptions, Evaluation, KeyHasher, NodeErrorKind, NodeResult, evaluate,
};
use crate::node::{Node, RecipeNodeId};
use crate::program::ProfileProgram;
use crate::resolve::{FlipSet, diff_verdicts};
use crate::spoken::SpokenNode;
use crate::var::VarId;
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
    /// **Whether the drive's leaves share one table**
    /// (`geom_core::sym::DriveMemo`): the expression table, the plain
    /// forms and the atom records. On by default, and a PERFORMANCE
    /// dial only.
    ///
    /// Each entry is a function of its key, the budget and the tier's
    /// dials, so what one leaf made is what every other leaf of this
    /// drive would make; the table hands it back instead of having it
    /// rebuilt, and a leaf starts from the caches an earlier leaf left.
    /// Every verdict and every decision count is the same either way,
    /// and so is the `frozen` column on both receipts: the drive's is
    /// the DISTINCT nodes frozen over the drive, which the table counts
    /// whether or not it is serving, and a leaf's is its NEED against
    /// that same set (`geom_core::SymCounts::frozen` argues the column).
    ///
    /// Off is the differential lane the pins compare against, in
    /// `parallel`'s own mould: every leaf builds its own tables and
    /// drops them.
    pub plain_memo: bool,
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
/// arc family it cannot discharge (M10-8) means the box
/// refuses either way.
///
/// Two things keep that bill down. A margin the numeric channel has
/// already proved NON-ZERO asks the tier nothing (`geom_core::sym`'s
/// `Decide` impl — at this lane's EXACT witness a certified enclosure
/// excluding zero is a proof no normal form can contradict), which is
/// most margins on most documents: no form is built for it. And
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
    /// The atom-algebra rules the normal form applies, and the
    /// registered-identity door beside them
    /// ([`geom_core::SymRules`]): the shipped set by default
    /// ([`geom_core::SymRules::shipped`], chosen by measurement); each
    /// switchable alone, and [`geom_core::SymRules::none`] is the
    /// quotient form with every atom opaque — the tier exactly as it
    /// stood before the algebra, so the effect of each rule on a
    /// document is a measurement taken through this dial rather than
    /// an assumption.
    ///
    /// **That premise needs [`Self::retry`] at
    /// [`geom_core::SymRetry::none`]**: a retry ladder is a second
    /// rule set run into each side's refusals — its first attempt shuts
    /// rule G — so a rules differential taken with it on both sides
    /// reads a rule's cost as recovered. The suites' differentials take
    /// their dials from one place that sets it so
    /// (`editor-core/tests/m10_8_harness.rs`, `dials`).
    pub rules: geom_core::SymRules,
    /// **The RETRY LADDER a refused decision may take**
    /// ([`geom_core::SymRetry`]): the measured ladder by default
    /// ([`DEFAULT_SYM_RETRY`] = [`geom_core::SymRetry::kept_atom`], whose
    /// doc carries the cost against the line and the trade), and
    /// [`geom_core::SymRetry::none`] is the tier making one attempt per
    /// rung — exactly as it stood before SYM-9 — so what the ladder buys
    /// a document is a differential taken through this dial.
    ///
    /// It can only ADD: a retry is asked only where every rung of the
    /// first attempt declined, so no dial value here moves a decision
    /// the first attempt answered.
    pub retry: geom_core::SymRetry,
}

/// The shipped term budget ([`SymbolicDials`]).
pub const DEFAULT_SYM_MAX_TERMS: usize = 4096;

/// The shipped degree budget ([`SymbolicDials`]).
pub const DEFAULT_SYM_MAX_DEGREE: u32 = 128;

/// **The shipped RETRY LADDER** ([`SymbolicDials::retry`]):
/// [`geom_core::SymRetry::kept_atom`] — two kept-atom attempts, rule G
/// shut and then rule A's `sqrt(X)² = X` shut, and no wider-ring
/// attempt. It SHIPS ACROSS THE 1.6 s LINE as a disclosed trade, the way
/// rule E did (`geom_core::sym`'s rule-E section).
///
/// **The cost, on the affordability line's own instrument** — one
/// whole-box leaf (`m10_10_leaf_cost_with_and_without_the_algebra`),
/// release, the fastest of three takes on a four-core box, the shipped
/// rules at one attempt per rung against the same rules with a ladder:
///
/// | document (scale) | M10-9 | one attempt | `kept_atom` | rule-G mask alone | rule-A mask alone | what the ladder closes |
/// | --- | --- | --- | --- | --- | --- | --- |
/// | two-hole plate (`1e2·ε`) | 0.09 s | 0.35 s | 0.35 s | 0.38 s¹ | 0.38 s¹ | nothing |
/// | R1's annulus (`1e1·ε`) | 0.06 s | 0.37 s | 0.38 s | 0.40 s¹ | 0.39 s¹ | nothing |
/// | R2's filleted bracket (`1e1·ε`) | 0.28 s | 2.86 s | 3.88 s | 3.53 s | 3.25 s | 6 decisions (rule A's attempt) |
/// | R2's link (`1e1·ε`) | 0.23 s | 17.28 s | 19.71 s | 18.94 s | 17.93 s | 12 decisions (rule G's attempt) |
/// | R2's rounded pad (`1e2·ε`) | 0.75 s | 131.3 s | 147.7 s | 144.0 s¹ | 139.2 s¹ | nothing |
///
/// ¹ One take, not three.
///
/// The table is SYM-9's measurement, taken while the receipt still
/// split its zeros by kind (the six were `registered`, the twelve
/// `symbolic_zero`); every one of them is in `symbolic` now.
///
/// **The trade, on the balance.** The line is 1.6 s. The bracket, the
/// link and the pad are over it at ONE attempt per rung — the shipped
/// rules alone take the bracket to 10× and the link to 75× their M10-9
/// leaf — and the ladder adds 1.0 s on the bracket (36 %), 2.4 s on the
/// link (14 %) and 16 s on the pad (12.5 %). It takes no document from
/// under the line to over it and changes no certification (every leaf
/// above certifies whole with it and without it); with the decision read
/// shut (`SymRules::without_the_reads`) the same columns move by less
/// than the takes' spread, so the read is not what it costs. What it
/// buys is the link's twelve decisions, ten of them exactly the ten
/// rule G costs the `carrier_on_surface_1`/`_2` pair at one attempt per
/// rung
/// (`work/decide/rule-g-trades-sixteen-of-the-links-carrier-on-surface-2`)
/// — so the rule-G attempt pays back what the default rule G takes.
///
/// **The rule-A attempt ships for its reach.** On the only document it
/// acts on it moves six decisions out of `numeric` for about 0.4 s of
/// the bracket's 1.0 s (rule-A mask alone 3.25 s against 2.86), and it
/// ships with the rule-G attempt on the same balance: decisions taken
/// from the numeric channel at the cost named here.
///
/// **The masks' order is a tie**: the other order recovers the same
/// decisions on every measured document (the nominal replay, dev:
/// bracket 1.47× against 1.44×, link 1.22× against 1.21×; the leaf:
/// 3.88 against 3.76 s and 19.71 against 19.51 s, inside the takes'
/// spread), so the order is carried by neither reach nor cost and
/// rule G's attempt goes first as SYM-9 shipped it.
///
/// **One mask would not do**: rule A and rule G shut in ONE attempt
/// keeps the bracket's six and recovers NONE of the link's twelve at
/// the nominal (why is not executed; `geom_core::SymRetry::without`
/// carries the hypothesis and what would confirm it).
///
/// **The wider ring is measured and not in the ladder**, which is the
/// answer SYM-9 owes `work/sym/coefficient-ring-width-is-not-monotone-in-reach`:
/// at 512 bits it recovers, predicate by predicate, no more than the
/// kept-atom attempts do — the bracket's same six and eight of the
/// link's twelve — at 4.50× the nominal replay on the bracket against
/// the kept-atom ladder's 1.47×. At 1024 bits it recovers thirteen more
/// on the bracket than the kept atom does, 19 in all, at 11.57×.
pub const DEFAULT_SYM_RETRY: geom_core::SymRetry = geom_core::SymRetry::kept_atom();

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
            crate::eval::LeafLane::Symbolic(self.budget(), self.rules, self.retry)
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
            // The shipped atom-algebra set ([`geom_core::SymRules::shipped`]).
            rules: geom_core::SymRules::default(),
            // The measured retry ladder ([`DEFAULT_SYM_RETRY`]).
            retry: DEFAULT_SYM_RETRY,
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
            plain_memo: true,
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
/// `resolve::vdiff`'s, beside the first-turn reading it is the
/// counterpart of, and WHICH ROWS A GATE EXCLUDES is this driver's policy. One node
/// kind qualifies and it is [`Node::Assertion`]: an assertion checks
/// and never places (D10), so a leaf whose verdict differs from the
/// witness's is not a different build. Its comparison's escalation is
/// still read by `classify_replay`'s definiteness step, so an
/// assertion the leaf cannot decide bisects. The measure node itself is
/// NOT dropped: a leaf where the measurement could not be taken is not
/// the witness build, and that difference stays in the comparison.
///
/// No row this returns carries [`crate::resolve::RunStatus::Absent`].
/// [`drive`] refuses [`DriveRefusal::Stopped`] for any run of its that
/// stopped short, and [`DriveRefusal::WitnessDoesNotBuild`] unless every
/// node of the witness's `order` is `Ok`, so no row it reads this from
/// is of a prefix. A consumer that takes
/// [`VerdictVector::of`] over some other evaluation can see `Absent`.
pub fn certifying_vector<T: topo::AtRestPolicy + geom_core::Decide, P>(
    doc: &Doc<P>,
    ev: &Evaluation<T>,
) -> VerdictVector {
    let mut vector = VerdictVector::of(ev);
    vector
        .rows
        .retain(|row| !matches!(doc.node(row.node), Some(Node::Assertion(_))));
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
/// **What it names**: per node, the first choice the witness and the
/// leaf take differently, where it is one predicate's sign
/// (`resolve::first_turn`). Up to that choice the two took one path, so
/// whatever follows it, permuted or recounted, is not read; a check is
/// never read, since another sign there could only refuse the leaf. A
/// node whose choices part another way names nothing.
///
/// **What it reads cannot weaken a certificate**, because certification
/// does not consult this at all. A leaf certifies on EXACT
/// [`VerdictVector`] equality, checks included (that type's own docs say
/// why the two questions want two shapes); this evidence only NAMES a
/// divergence the strict comparison already found. The strict test
/// gates; the first turn explains.
#[derive(Debug, Clone, PartialEq)]
pub struct FlipEvidence {
    /// The verdict-diff engine's answer: per node, its first turn and
    /// its standing in the two runs.
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
    /// is why it refused at all), but no node's choices turned on a
    /// sign: they differ only in checks, or part on another predicate.
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
    /// The leaf's deciding node, the first node in evaluation order
    /// that did not settle, reads no variable of the box, so both
    /// halves of any split replay it on the inputs it had here. Not a
    /// budget: no setting changes it.
    ReadsNoVariable {
        /// The deciding node.
        node: RecipeNodeId,
        /// What it did instead of settling.
        unsettled: Unsettled,
    },
}

/// How a leaf's deciding node left the leaf unsettled.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Unsettled {
    /// A decision of its own came back indeterminate.
    Undecided,
    /// It failed typed, for a reason the driver does not prove definite
    /// over the box and so does not read as a flip.
    Failed {
        /// Which failure, without its payload.
        class: crate::NodeErrorClass,
        /// The failure as the document speaks it.
        cause: crate::Sentence,
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
            Self::ReadsNoVariable { .. } => ReasonClass::ReadsNoVariable,
        }
    }
}

/// Which bound stopped the refinement.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BudgetKind {
    /// Every variable the deciding node reads that the `f64` grid can
    /// still split had already been bisected
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
    /// The box could not be bisected: on every variable the deciding
    /// node reads the midpoint lands on an endpoint, so the `f64` grid
    /// itself is the bound.
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
    /// [`RefusalReason::ReadsNoVariable`].
    ReadsNoVariable,
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
            Self::ReadsNoVariable => "reads_no_variable",
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
    /// ([`SymbolicDials`]), `frozen` being this leaf's NEED of the
    /// drive's frozen set rather than the work it happened to do
    /// (`geom_core::SymCounts::frozen`). All zero when the tier is off,
    /// because no session exists to count in.
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
    /// E12 receipt ([`SymbolicDials`]), `frozen` being this leaf's NEED
    /// of the drive's frozen set (`geom_core::SymCounts::frozen`).
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
    memo: sym::MemoSize,
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

    /// **What the drive's table came to** at its end
    /// (`DriveConfig::plain_memo`): the nodes, plain forms and atom
    /// records it held and an estimate of the heap they occupied. The
    /// leaves' warm caches of it are not in the estimate
    /// (`geom_core::sym::MemoSize::bytes_estimate`).
    ///
    /// With the dial OFF the table serves nothing, so `nodes`, `forms`
    /// and `atoms` are zero — but `frozen` is not, and neither is
    /// `bytes_estimate`: the off lane still collects the drive's
    /// distinct freezes, which is what keeps the receipt's `frozen`
    /// column the same column in both lanes (the plate reads
    /// `frozen: 1044, bytes_estimate: 16704` there). The growth guard
    /// is a reading of the lane that holds forms.
    ///
    /// Not part of the receipt: it is the memo's SIZE, a cost, and the
    /// verdict reports it so a guard can be written against it rather
    /// than re-derived by instrumenting the tier.
    pub fn plain_memo(&self) -> sym::MemoSize {
        self.memo
    }

    /// The counting receipt.
    pub fn receipt(&self) -> Receipt {
        self.receipt
    }

    /// **The E12 receipt**: how this drive's predicate decisions were
    /// answered, summed over every leaf — `symbolic` against
    /// `numeric`, with `frozen` beside them.
    ///
    /// **`frozen` is the odd one out**: the decision columns are sums
    /// over the leaves, and `frozen` is a SET — the DISTINCT nodes
    /// frozen over the drive (`geom_core::sym::DriveMemo::frozen`),
    /// which is not the sum of the leaves' own columns because those
    /// are sets over the same nodes and they overlap.
    /// `SymCounts::frozen` argues both receipts' column.
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
        let _ = writeln!(s, "witness_vector {:032x}", self.witness_vector.key());
        for leaf in &self.certified {
            let _ = writeln!(
                s,
                "certified {} key={:032x}",
                render_box(&leaf.box_),
                leaf.verdict_vector_key
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
            let _ = write!(
                s,
                "decisions symbolic={} numeric={} frozen={}",
                self.decisions.symbolic, self.decisions.numeric, self.decisions.frozen
            );
            // The door's REFUSAL column, present only when non-zero: a
            // registration the witness separated. Zero on every
            // document that behaves, so a clean drive serializes what
            // it serialized before.
            if self.decisions.registrations_refused != 0 {
                let _ = write!(
                    s,
                    " registrations_refused={}",
                    self.decisions.registrations_refused
                );
            }
            // **How many of the discharges above a RETRY carried**
            // (`geom_core::SymCounts::retried`, SYM-9), by the same
            // present-only-when-nonzero rule: the ladder is entered
            // only on a refusal, so a drive it closes nothing on
            // serializes the line it serialized before the ladder
            // existed. At the nominal it closes something on two of the
            // five documents SYM-9 measured (R2's link and bracket).
            //
            // It is not a second discharge column: a retry's answer
            // lands in `symbolic`, and this says how many of those the
            // second attempt is holding up.
            if self.decisions.retried != 0 {
                let _ = write!(s, " retried={}", self.decisions.retried);
            }
            let _ = writeln!(s);
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
    /// probability ([`crate::report::MassBasis`]). Each variable a line
    /// names is spoken from `doc`, the document the drive ran over, and
    /// mentioned in it: one sentence per line.
    pub fn render<P: crate::ProfilePayload>(
        &self,
        doc: &crate::doc::Doc<P>,
        analyzed: &AnalyzedBox,
    ) -> crate::Lines {
        let by = crate::Speaker::of(doc);
        let mut s = crate::Sentence::new();
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
            if *class != ReasonClass::ReadsNoVariable.name() {
                continue;
            }
            // Each node once, with what it did.
            let named: BTreeMap<RecipeNodeId, &Unsettled> = self
                .refused
                .iter()
                .filter_map(|leaf| match &leaf.reason {
                    RefusalReason::ReadsNoVariable { node, unsettled } => Some((*node, unsettled)),
                    _ => None,
                })
                .collect();
            for (node, unsettled) in named {
                let said = by.mention(&doc.spoken(node));
                let _ = match unsettled {
                    Unsettled::Undecided => say!(
                        &mut s,
                        "    ",
                        said,
                        " is undecided and reads no variable of the box: no split changes it\n"
                    ),
                    Unsettled::Failed { cause, .. } => say!(
                        &mut s,
                        "    ",
                        said,
                        " fails and reads no variable of the box: no split changes it (",
                        cause,
                        ")\n"
                    ),
                };
            }
        }
        let d = self.decisions;
        if d != SymCounts::default() {
            let total = d.decisions();
            let share = if total == 0 {
                0.0
            } else {
                100.0 * (d.symbolic as f64) / (total as f64)
            };
            let _ = write!(
                s,
                "  {} of {total} decisions were proven zero by the symbolic tier ({share:.1}%)",
                d.symbolic
            );
            // How many of those a retry carried
            // (`geom_core::SymCounts::retried`).
            if d.retried != 0 {
                let _ = write!(
                    s,
                    "; {} of those discharges reached only by a second attempt",
                    d.retried
                );
            }
            // A refusal is louder than a count: it says a constructor
            // stated something this box contradicts.
            if d.registrations_refused != 0 {
                let _ = write!(
                    s,
                    "; {} registration(s) REFUSED by the witness",
                    d.registrations_refused
                );
            }
            let _ = writeln!(s, "; {} form(s) frozen", d.frozen);
        }
        let _ = say!(
            &mut s,
            crate::report::MassBudget::of(&self.accounting, analyzed).render(doc)
        );
        s.lines()
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
        /// The first node, in evaluation order, that did not build,
        /// spoken from the driven document.
        node: SpokenNode,
        /// The node error, said from the driven document.
        cause: crate::Sentence,
    },
    /// The box has no varying axis: nothing is declared to vary, so
    /// there is no parameter box to subdivide. The nominal build
    /// answers this document completely, and saying so is more useful
    /// than returning a one-leaf verdict that looks like an analysis.
    NothingVaries,
    /// An evaluation the drive ran stopped short: its caller canceled,
    /// or it ran out of its member budget ([`crate::Stop`]). A leaf
    /// whose replay holds a prefix decided nothing, so the drive has no
    /// verdict to report.
    Stopped(crate::Stopped),
}

impl crate::Say for DriveRefusal {
    fn say(&self, f: &mut crate::Sentence, by: crate::Speaker<'_>) -> core::fmt::Result {
        match self {
            Self::WitnessDoesNotBuild { node, cause } => say!(
                f,
                "the witness build refuses at ",
                by.mention(node),
                ": ",
                cause,
                " — there is no branch to certify leaves against until the nominal document \
                 builds"
            ),
            Self::NothingVaries => f.write_str(
                "no parameter of this document declares a distribution, so the analyzed box has \
                 no varying axis — the nominal build is the whole answer",
            ),
            Self::Stopped(stopped) => write!(f, "the drive stopped: {stopped}"),
        }
    }
}

/// Its sentence, each node said by tag ([`crate::Speaker::TAG`]).
impl core::fmt::Display for DriveRefusal {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        crate::spoken::display_said(self, crate::Speaker::TAG, f)
    }
}

impl core::error::Error for DriveRefusal {}

/// **The WITNESS build's options**: the document at its nominals, over
/// the analysed box collapsed to the nominal (`ParamBox::at_nominal`),
/// the environment a leaf binds at the box's centre. At `f64` that is
/// the build, and its profile nodes log both passes as a leaf's do
/// (`ProfileLane`), so the two verdict vectors compare. [`drive`]
/// evaluates its witness at f64 under exactly these.
#[must_use]
pub fn witness_options(analyzed: &AnalyzedBox, stop: &crate::Stop) -> EvalOptions {
    EvalOptions {
        param_box: Some(Arc::new(ParamBox::of(analyzed).at_nominal())),
        ..lane_opts(stop)
    }
}

/// **The witness build of a drive**: the document at its nominals, at
/// `f64`, under [`witness_options`] and `resolver`. Its profile nodes
/// log both passes, as a leaf's do (`ProfileLane`), so the two verdict
/// vectors compare.
///
/// # Errors
///
/// [`DriveRefusal::Stopped`] for a run that stopped short, and
/// [`DriveRefusal::WitnessDoesNotBuild`] unless every node is usable.
pub(crate) fn witness_build(
    doc: &Doc<ProfileProgram>,
    analyzed: &AnalyzedBox,
    resolver: Option<&Arc<dyn crate::part::PartResolver>>,
    stop: &crate::Stop,
    tol: Tol,
) -> Result<Evaluation<f64>, DriveRefusal> {
    let opts = EvalOptions {
        resolver: resolver.cloned(),
        ..witness_options(analyzed, stop)
    };
    let witness: Evaluation<f64> = evaluate(doc, None, &stop.cancel, &opts, tol);
    if let Some(stopped) = crate::Stopped::of(witness.outcome) {
        return Err(DriveRefusal::Stopped(stopped));
    }
    match does_not_build(doc, &witness) {
        Some(refusal) => Err(refusal),
        None => Ok(witness),
    }
}

/// [`DriveRefusal::WitnessDoesNotBuild`] for a finished `f64` build
/// with a node that is not usable, naming the first in evaluation
/// order; `None` when every node is.
pub(crate) fn does_not_build(
    doc: &Doc<ProfileProgram>,
    built: &Evaluation<f64>,
) -> Option<DriveRefusal> {
    first_unusable(doc, built)
        .map(|(node, cause)| DriveRefusal::WitnessDoesNotBuild { node, cause })
}

/// **The first node of a finished `f64` build that is not usable**, in
/// evaluation order, spoken from `doc`, and why: its own error, or the
/// standing, said from `doc` with each node it names a mention there.
/// `None` when every node is usable. The drive's witness and the
/// Monte-Carlo nominal refuse on it alike.
pub(crate) fn first_unusable(
    doc: &Doc<ProfileProgram>,
    built: &Evaluation<f64>,
) -> Option<(SpokenNode, crate::Sentence)> {
    let standing = built.order.iter().find_map(|&id| built.usable(id).err())?;
    let node = standing.node();
    let cause = built
        .node_error(node)
        .map_or_else(|| standing.spoken(doc), |e| e.kind_spoken(doc));
    Some((doc.spoken(node), cause))
}

/// The plain-form memo one drive's leaves share
/// ([`DriveConfig::plain_memo`]): serving forms, or only counting the
/// freezes with the dial off.
pub(crate) fn drive_memo(plain_memo: bool, symbolic: SymbolicDials) -> Arc<sym::DriveMemo> {
    Arc::new(if plain_memo {
        sym::DriveMemo::new(symbolic.budget(), symbolic.rules)
    } else {
        sym::DriveMemo::counting_only(symbolic.budget(), symbolic.rules)
    })
}

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
    stop: &crate::Stop,
    tol: Tol,
) -> Result<ParamBoxVerdict, DriveRefusal> {
    let root = ParamBox::of(analyzed);
    if root.varying().next().is_none() {
        return Err(DriveRefusal::NothingVaries);
    }

    let witness = witness_build(doc, analyzed, None, stop, tol)?;
    geom_core::k_stats::probe_tally::dump("witness");
    let witness_vector = Arc::new(certifying_vector(doc, &witness));
    let witness_key = witness_vector.key();
    // What each node reads is the document's, so it is taken once.
    let reads = doc.free_reads();
    // The witness EVALUATION stays alive for the whole drive, not just
    // long enough to take its vector: `diff_verdicts` names a leaf's
    // turns against it, and the engine takes the two evaluations rather
    // than two digests of them. One build's worth of geometry, held
    // once per drive.

    let mut certified: Vec<CertifiedLeaf> = Vec::new();
    let mut refused: Vec<RefusedLeaf> = Vec::new();
    let mut splits = 0usize;
    let mut decisions = SymCounts::default();
    // **The drive's table**, created before the level loop and dropped
    // after it, so it holds nothing of the next drive. One for the
    // drive and not one per worker: the `frozen` column it answers is
    // the distinct nodes frozen over the DRIVE, and a value one leaf
    // built is read in another through it. With the dial off it serves
    // nothing and only counts the freezes, so the column is the same
    // column in both lanes.
    //
    // Built even with the tier OFF or at a zero-term budget, where no
    // session is installed and nothing reaches it: the alternative is
    // an `Option` every call site below would open for a case
    // `symbolic.enabled` already answers.
    let memo = drive_memo(config.plain_memo, config.symbolic);

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
                &memo,
                None,
                stop,
                tol,
            )
        };
        let verdicts: Result<Vec<(LeafVerdict, SymCounts)>, crate::Stopped> = if config.parallel {
            // Every leaf is collected before a stop is chosen, so the one
            // reported is the first in frontier order whatever the
            // schedule, as the stackup's and the MC lane's maps do.
            geom_core::k_stats::par_map(&frontier, leaf)
                .into_iter()
                .collect()
        } else {
            frontier.iter().map(leaf).collect()
        };
        let verdicts = verdicts.map_err(DriveRefusal::Stopped)?;
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
                //
                // A deciding node that reads nothing of the box is asked
                // first: no budget is why that leaf stops.
                LeafVerdict::Bisect {
                    deciding,
                    unsettled,
                } => {
                    let pending = level - i - 1;
                    let committed = certified.len() + refused.len() + pending + next.len();
                    // Every node an evaluation orders is a node of the
                    // document, and `free_reads` holds every one.
                    debug_assert!(
                        reads.contains_key(&deciding),
                        "the deciding node {deciding:?} is in no read set"
                    );
                    let split = bisect(
                        &b,
                        &root,
                        reads.get(&deciding).unwrap_or(&BTreeSet::new()),
                        config.max_depth,
                    );
                    let reason = match split {
                        Err(Unsplit::Unread) => RefusalReason::ReadsNoVariable {
                            node: deciding,
                            unsettled,
                        },
                        _ if committed + 2 > config.max_leaves => {
                            RefusalReason::Budget(BudgetKind::Leaves {
                                max_leaves: config.max_leaves,
                            })
                        }
                        Err(Unsplit::Budget(kind)) => RefusalReason::Budget(kind),
                        Ok((a, c)) => {
                            splits += 1;
                            next.push(a);
                            next.push(c);
                            continue;
                        }
                    };
                    refused.push(RefusedLeaf {
                        box_: b.box_,
                        reason,
                        decisions: counts,
                    });
                }
            }
        }
        frontier = next;
    }

    #[cfg(feature = "probe")]
    if config.k_probe == KProbe::CertifiedMidpoints {
        for leaf in &certified {
            probe_midpoint(doc, &leaf.box_, config.symbolic, stop, tol);
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
    // **The drive's `frozen` column**, and the one place it is written:
    // the DISTINCT nodes frozen over the drive (`SymCounts::frozen`
    // argues the column; `SymCounts::absorb` is why it is not a sum).
    if config.symbolic.enabled {
        decisions.frozen = memo.frozen();
    }
    if geom_core::k_stats::probe_tally::on() {
        eprintln!(
            "[bf-drive] certified={} refused={} splits={} decisions sym={} num={} frozen={}",
            certified.len(),
            refused.len(),
            splits,
            decisions.symbolic,
            decisions.numeric,
            decisions.frozen
        );
        geom_core::k_stats::probe_tally::dump("drive");
    }
    let accounting = account(analyzed, &root, &certified, &refused);
    Ok(ParamBoxVerdict {
        certified,
        refused,
        accounting,
        receipt,
        decisions,
        symbolic: config.symbolic,
        memo: memo.size(),
        witness_vector,
        root,
    })
}

/// The evaluation options every pass of a drive uses: sequential
/// (leaf-level parallelism is the driver's, and nesting a rayon scope
/// per leaf inside one buys nothing), and no memo.
///
/// **Crate-visible because the clearance engine replays INSIDE a leaf**
/// ([`crate::clearance`]) and must replay it the way this driver
/// certified it. Two copies of these options are two lanes: a clearance
/// query whose options drifted from the driver's would be
/// certifying a body other than the one the leaf's verdict vector is
/// about.
///
/// The SCALAR is the other half of "the way this driver certified it",
/// and it does not ride here: it rides on the verdict
/// ([`ParamBoxVerdict::symbolic`]) and reaches a consumer through
/// `crate::eval::replay_leaf`.
pub(crate) fn lane_opts(stop: &crate::Stop) -> EvalOptions {
    // `EvalOptions::default()` already mints an epoch; minting a second
    // one to overwrite it burnt a process-global counter per leaf, and
    // a drive is tens of thousands of leaves. The epoch is a
    // stale-result discrimination token for a caller holding several
    // in-flight evaluations, which a drive is not: each leaf's result
    // is consumed before the next is asked for.
    stop.options(EvalOptions::default())
}

/// A frontier entry: a box and how many times each axis has been split
/// to reach it (the per-axis depth budget's currency).
struct Box_ {
    box_: ParamBox,
    depths: BTreeMap<VarId, u32>,
}

/// What one leaf's replay decided.
pub(crate) enum LeafVerdict {
    Certified(CertifiedLeaf),
    Refused(RefusalReason),
    /// Unsettled at `deciding`, the first node in evaluation order
    /// that did not settle: the node whose reads choose the split
    /// ([`bisect`]).
    Bisect {
        deciding: RecipeNodeId,
        unsettled: Unsettled,
    },
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
/// The session is per-CALL and thread-local, and under the parallel
/// schedule each leaf runs wholly on one rayon thread. What a leaf
/// holds of its own is its box's brackets, the caches valid for that
/// box, and each node's state while the node runs
/// (`geom_core::sym::MintScope::node`); the expression table, the
/// plain forms and the atom records are the drive's (`memo`,
/// `DriveConfig::plain_memo`, `geom_core::sym::DriveMemo`), shared
/// across the workers and the same whichever leaf wrote them.
///
/// **The f64 witness pass is untouched.** A point residual is tight, so
/// the witness still catches a constructor that does not build what it
/// claims — which is exactly the failure the symbolic tier could
/// otherwise hide, since an expression that is identically zero on
/// paper says nothing about whether the code computed it.
// One parameter per named input the leaf replay needs: the document,
// the box, the witness build and its vector, the tier's dials, the
// drive's plain memo and the part resolver (`None` from [`drive`]).
// Nothing here is state to thread — every one is read by the leaf and
// none is written — so a bundle would be a struct that exists to
// satisfy a count.
#[allow(clippy::too_many_arguments)]
pub(crate) fn classify(
    doc: &Doc<ProfileProgram>,
    box_: &ParamBox,
    witness: &Evaluation<f64>,
    witness_vector: &VerdictVector,
    witness_key: VerdictVectorKey,
    symbolic: SymbolicDials,
    memo: &Arc<sym::DriveMemo>,
    resolver: Option<&Arc<dyn crate::part::PartResolver>>,
    stop: &crate::Stop,
    tol: Tol,
) -> Result<(LeafVerdict, SymCounts), crate::Stopped> {
    let opts = EvalOptions {
        param_box: Some(Arc::new(box_.clone())),
        resolver: resolver.cloned(),
        ..lane_opts(stop)
    };
    if symbolic.enabled {
        // PROBE (boxfinish/probe-switch-off): one line per leaf.
        let probe = geom_core::k_stats::probe_tally::on();
        let started = std::time::Instant::now();
        let finishes = topo::finishes_on_this_thread();
        let evals = std::cell::Cell::new(0u32);
        let (leaf, counts) = sym::with_session_memo_retry(
            symbolic.budget(),
            symbolic.rules,
            symbolic.retry,
            memo,
            || {
                evals.set(evals.get() + 1);
                if geom_core::k_stats::probe_tally::m8() {
                    geom_core::k_stats::probe_tally::nodes_take();
                    geom_core::k_stats::probe_tally::nodes9_take();
                }
                let m9_eval = std::time::Instant::now();
                let leaf: Evaluation<Sym<Interval>> = evaluate(doc, None, &stop.cancel, &opts, tol);
                if geom_core::k_stats::probe_tally::m11() {
                    static LEAVES: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
                    let n = LEAVES.fetch_add(1, std::sync::atomic::Ordering::Relaxed) + 1;
                    let (nodes, forms, atoms) = geom_core::sym::probe_session_sizes();
                    let spans: Vec<String> = box_
                        .order()
                        .iter()
                        .filter_map(|n| Some((n, box_.get(*n)?)))
                        .map(|(n, axis)| {
                            let (lo, hi) = axis.span();
                            format!("{}=[{lo:e},{hi:e}]", n.full().to_string().chars().take(4).collect::<String>())
                        })
                        .collect();
                    let first = leaf.order.iter().find_map(|id| match leaf.result(*id) {
                        Some(NodeResult::Failed(e)) => Some(format!("node {} {}", id.0, format!("{:?}", e.kind).chars().take(500).collect::<String>())),
                        _ => None,
                    });
                    eprintln!(
                        "[m11-mem] leaf={n} rss_kb={} eval_ms={:.0} session_nodes={nodes} session_forms={forms} session_atoms={atoms} drive_memo={:?} box={} first_failure={}",
                        geom_core::k_stats::probe_tally::rss_kb(),
                        m9_eval.elapsed().as_secs_f64() * 1e3,
                        memo.size(),
                        spans.join(";"),
                        first.unwrap_or_else(|| "none".to_owned()),
                    );
                }
                let m9_eval_ms = m9_eval.elapsed().as_secs_f64() * 1e3;
                let m9_probe = std::time::Instant::now();
                // PROBE (M8): one line per replay and per node, inside
                // the session so the symbolic ids can be classified.
                if geom_core::k_stats::probe_tally::m8() {
                    use geom_core::k_stats::probe_tally as pt;
                    let spans: Vec<String> = box_
                        .order()
                        .iter()
                        .filter_map(|n| Some((n, box_.get(*n)?)))
                        .map(|(n, axis)| {
                            let (lo, hi) = axis.span();
                            format!("{}=[{lo:e},{hi:e}]", n.full())
                        })
                        .collect();
                    let spans = spans.join(";");
                    let rid = pt::hash_ids(&spans.bytes().map(u128::from).collect::<Vec<_>>());
                    let timed = pt::nodes_take();
                    let timed9 = pt::nodes9_take();
                    eprintln!("[m9-replay] rid={rid:016x} eval_ms={m9_eval_ms:.2}");
                    let deciding = leaf.order.iter().find(|id| match leaf.result(**id) {
                        Some(NodeResult::Ok(v)) => !v.escalations.is_empty(),
                        Some(NodeResult::Failed(_)) => true,
                        _ => false,
                    });
                    eprintln!(
                        "[m8-replay] rid={rid:016x} attempt={} deciding={} box={spans}",
                        evals.get(),
                        deciding.map_or("none".to_owned(), |d| format!("{}", d.0)),
                    );
                    for id in &leaf.order {
                        let name = format!("{}", id.0);
                        let t = timed.iter().find(|t| t.0 == name);
                        let (state, key, ids) = match leaf.result(*id) {
                            Some(NodeResult::Ok(v)) => (
                                "ok",
                                Some(v.content_key.0),
                                pt::sym_ids(&format!("{:?}", v.payload)),
                            ),
                            Some(NodeResult::Failed(_)) => ("failed", t.map(|t| t.1), Vec::new()),
                            Some(NodeResult::Poisoned { .. }) => ("poisoned", None, Vec::new()),
                            None => ("absent", None, Vec::new()),
                        };
                        // PROBE (M9): the node's recorded side as the drive
                        // receives it, and where it differs under one key.
                        let t9 = timed9.iter().find(|t| t.0 == name);
                        let counts = t9.map_or("none", |t| t.3.as_str());
                        let recorded = match leaf.result(*id) {
                            Some(NodeResult::Ok(v)) => Some(format!(
                                "verdicts={:?} escalations={:?} marks={:?}",
                                v.verdicts, v.escalations, v.marks
                            )),
                            Some(NodeResult::Failed(e)) => {
                                Some(format!("kind={:?} escalations={:?}", e.kind, e.escalations))
                            }
                            _ => None,
                        };
                        if let (Some(text), Some(k)) = (&recorded, key) {
                            let stripped = pt::strip_nodes(text);
                            let diff = pt::first_difference(format!("{name}/{k:032x}"), text);
                            let diff_c = pt::first_difference(format!("counts/{name}/{k:032x}"), counts);
                            eprintln!(
                                "[m9-node] rid={rid:016x} node={name} state={state} key={k:032x} pre_ms={:.3} rec={:016x} rec_no_ids={:016x} rec_len={} counts={counts}",
                                t9.map_or(0.0, |t| t.1),
                                pt::hash_text(text),
                                pt::hash_text(&stripped),
                                text.len(),
                            );
                            if let Some(d) = diff {
                                eprintln!("[m9-diff] node={name} key={k:032x} {d}");
                            }
                            if let Some(d) = diff_c {
                                eprintln!("[m9-cdiff] node={name} key={k:032x} {d}");
                            }
                        }
                        let mut distinct = ids.clone();
                        distinct.dedup();
                        let opaque = geom_core::sym::probe_opaque_of(&distinct);
                        eprintln!(
                            "[m8-node] rid={rid:016x} node={name} state={state} key={} total_ms={:.2} finish_ms={:.2} sym_multiset={:016x} sym_n={} sym_distinct={} opaque_n={} opaque_set={:016x}",
                            key.map_or("none".to_owned(), |k| format!("{k:032x}")),
                            t.map_or(0.0, |t| t.2),
                            t.map_or(0.0, |t| t.3),
                            pt::hash_ids(&ids),
                            ids.len(),
                            distinct.len(),
                            opaque.len(),
                            pt::hash_ids(&opaque),
                        );
                    }
                    eprintln!("[m9-probe] rid={rid:016x} probe_ms={:.2}", m9_probe.elapsed().as_secs_f64() * 1e3);
                }
                leaf
            },
        );
        if let Some(stopped) = crate::Stopped::of(leaf.outcome) {
            return Err(stopped);
        }
        let m9_classify = std::time::Instant::now();
        let verdict = classify_replay(
            doc,
            box_,
            &leaf,
            witness,
            witness_vector,
            witness_key,
            counts,
        );
        if geom_core::k_stats::probe_tally::m8() {
            let (setup, teardown) = geom_core::sym::PROBE_SESSION_MS.with(std::cell::Cell::get);
            eprintln!(
                "[m9-leaf] leaf_ms={:.2} session_setup_ms={setup:.3} session_teardown_ms={teardown:.3} classify_ms={:.3} frozen={} sym={} num={}",
                started.elapsed().as_secs_f64() * 1e3,
                m9_classify.elapsed().as_secs_f64() * 1e3,
                counts.frozen, counts.symbolic, counts.numeric,
            );
        }
        if probe {
            let spans: Vec<String> = box_
                .order()
                .iter()
                .filter_map(|n| Some((n, box_.get(*n)?)))
                .map(|(n, axis)| {
                    let (lo, hi) = axis.span();
                    format!("{}=[{lo:e},{hi:e}]", n.full())
                })
                .collect();
            eprintln!(
                "[bf-leaf] box {} evals={} finishes={} dt_ms={:.1} sym={} num={} verdict={}",
                spans.join(" "),
                evals.get(),
                topo::finishes_on_this_thread() - finishes,
                started.elapsed().as_secs_f64() * 1e3,
                counts.symbolic,
                counts.numeric,
                match &verdict {
                    LeafVerdict::Certified(_) => "CERTIFIED".to_owned(),
                    LeafVerdict::Bisect { .. } => "BISECT".to_owned(),
                    LeafVerdict::Refused(r) => format!("REFUSED {}", render_reason(r)),
                }
            );
            for id in &leaf.order {
                if let Some(NodeResult::Failed(e)) = leaf.result(*id) {
                    let text = format!("{:?}", e.kind);
                    eprintln!(
                        "[bf-leaf]   node {} FAILED {}",
                        id.0,
                        text.chars().take(1800).collect::<String>()
                    );
                }
            }
        }
        return Ok((verdict, counts));
    }
    let leaf: Evaluation<Interval> = evaluate(doc, None, &stop.cancel, &opts, tol);
    if let Some(stopped) = crate::Stopped::of(leaf.outcome) {
        return Err(stopped);
    }
    let counts = SymCounts::default();
    Ok((
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
    ))
}

/// The classification itself, over an already-replayed leaf — generic
/// in the lane scalar (any scalar an evaluation is held at),
/// because the two lanes above differ only in which scalar produced the
/// evaluation and in nothing this function looks at.
fn classify_replay<T: topo::AtRestPolicy + geom_core::Decide>(
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
    // HOW IT ASKS, AND IN WHAT ORDER. Two reads per node, and the
    // order is a rule: (1) the node's escalation log decides among the box-DEPENDENT outcomes: every
    // indeterminate outcome the FUNNEL produced while the op ran is on
    // it — on the value if the op built anyway, on the error if it did
    // not — so an escalation an op wrapped in its own error enum (a
    // sweep's `ExtrusionEscalated`, ~40 such variants across five
    // crates) is seen without matching on any of them — which is what
    // the log buys HERE, not a claim that those variants are redundant:
    // the log is a per-bracket side channel, empty for every caller of
    // those ops that opens no bracket, and this loop is one of the few
    // callers that does — and the FIRST
    // escalation in decision order speaks: a sliver, or the cue to
    // bisect (a later sliver behind a refinable escalation does not
    // argue — refinement may never reach it on the branch a definite
    // first decision takes — so that order is the conservative one);
    // (2) the error-enum arms, a CONSERVATIVE read with no live path
    // to them today. Every mint of either kind — `NodeErrorKind::`
    // `Escalated` and the `ProfileLaneReplay` structure refusal — is in
    // `eval::wire`, inside the bracket `eval_node` opens around the op,
    // so the escalation is on the node's log and read (1) answers
    // first. That is a search, not a proof: it says no path reaches
    // these arms, not that none can. Deleting them is a behaviour
    // change of its own — read (1) speaks for the log's FIRST
    // escalation where an arm speaks for the one the error carried, and
    // those differ on a node that recovered from an earlier one — so it
    // wants its own red-first row and its own measurement, which
    // `work/verdict/should-classify-replays-error-enum-arms-be-deleted.md`
    // holds. What HAS been discharged is the precondition the arms were
    // kept for: the log now carries the op-minted escalations too.
    // ITERATION ORDER IS THE LEAF'S EVALUATION ORDER, AND THE SPLIT
    // DEPENDS ON IT: the FIRST node in that order that did not settle
    // decides the leaf as a sliver or a bisect, a definite structure
    // flip at any node never gets to argue against it, and that node is
    // the leaf's DECIDING node, whose reads are the only variables the
    // leaf may be split on ([`bisect`]). Every node before it settled
    // (definitely, on the witness's branch or on another), so its
    // inputs are definite over the leaf and a split of what it reads is
    // the only one that can change what it says. Deterministic (the
    // schedule's order, the same in both schedules).
    let mut structure_flips = Vec::new();
    for (node, result) in leaf
        .order
        .iter()
        .filter_map(|&node| leaf.nodes.get(&node).map(|result| (node, result)))
    {
        let (escalations, failure) = match result {
            NodeResult::Ok(v) => (&v.escalations, None),
            NodeResult::Failed(e) => (&e.escalations, Some(e)),
            NodeResult::Poisoned { .. } => continue,
        };
        // (1) The escalation log.
        if let Some(undecided) = log_read(escalations) {
            return undecided.at(node);
        }
        // (2) The error-enum arms.
        let Some(err) = failure else {
            continue;
        };
        match &err.kind {
            NodeErrorKind::Escalated { source, .. }
            | NodeErrorKind::MeasureEscalated { source, .. }
            | NodeErrorKind::PlacementAxisEscalated { source, .. }
            | NodeErrorKind::PoseDirectionEscalated { source, .. } => {
                return indeterminate(source).at(node);
            }
            NodeErrorKind::ProfileLaneReplay {
                structure: Some(refusal),
                ..
            } => match &refusal.kind {
                profile::StructureRefusalKind::Indeterminate { source, .. } => {
                    return indeterminate(source).at(node);
                }
                profile::StructureRefusalKind::Flipped { .. } => {
                    structure_flips.push(StructureFlip {
                        node,
                        refusal: Box::new(refusal.clone()),
                    });
                }
            },
            _ => {
                return LeafVerdict::Bisect {
                    deciding: node,
                    unsettled: Unsettled::Failed {
                        class: err.kind.class(),
                        cause: err.kind_spoken(doc),
                    },
                };
            }
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
    // documents E10 exists for: a clearance assertion is `Refused`
    // (`needs_certifying_lane`) at every f64 witness and decided over a
    // certified leaf, so it flips on EVERY leaf and would head the flip
    // list of every unrelated refusal.
    //
    // It is a projection of the engine's answer onto the nodes the
    // question is about, not a second diff: the engine still runs over
    // the whole evaluation, its per-node deltas come back unaltered,
    // and the filter is the SAME predicate `certifying_vector` uses, spelled
    // once here so the two cannot drift.
    let mut verdicts = diff_verdicts(witness, leaf);
    verdicts
        .nodes
        .retain(|id, _| !matches!(doc.node(*id), Some(Node::Assertion(_))));
    LeafVerdict::Refused(RefusalReason::FlipCrossing {
        flipped: Box::new(FlipEvidence {
            verdicts,
            structure: structure_flips,
        }),
    })
}

/// What one escalation makes of the node that holds it.
#[derive(Debug, PartialEq, Eq)]
enum Undecided {
    /// Its enclosure sits wholly inside the band: a terminal sliver,
    /// naming the predicate.
    Sliver(&'static str),
    /// The cue to bisect.
    Refinable,
}

impl Undecided {
    /// The leaf's verdict when `node` is the one that holds it.
    fn at(self, node: RecipeNodeId) -> LeafVerdict {
        match self {
            Self::Sliver(predicate) => {
                LeafVerdict::Refused(RefusalReason::SliverTerminal { predicate })
            }
            Self::Refinable => LeafVerdict::Bisect {
                deciding: node,
                unsettled: Unsettled::Undecided,
            },
        }
    }
}

/// Read (1) of [`classify_replay`]: what a node's escalation log makes
/// of the node — its FIRST escalation speaks — or `None` for an empty
/// log, which leaves the node to read (2).
fn log_read(escalations: &[geom_core::k_stats::Escalation]) -> Option<Undecided> {
    escalations
        .first()
        .map(|first| indeterminate(&first.source))
}

/// What one escalation makes of its node: a terminal sliver when its
/// enclosure sits wholly inside the band ([`topo::clearance::sliver`]), otherwise the
/// cue to bisect. One spelling for the three reads of `classify_replay`.
fn indeterminate(source: &geom_core::Indeterminate) -> Undecided {
    topo::clearance::sliver(source).map_or(Undecided::Refinable, Undecided::Sliver)
}

/// Why a box was not split.
enum Unsplit {
    /// The deciding node reads no variable of the box.
    Unread,
    /// Every variable it reads is at a bound.
    Budget(BudgetKind),
}

/// The D9 split: of the variables in `reads` (the deciding node's,
/// [`Doc::free_reads`]) with depth left and room on the `f64` grid, the one
/// of greatest relative width, ties to the earliest-declared variable,
/// bisected at its midpoint. A variable outside `reads` is never split:
/// both halves would replay the deciding node on the inputs it had.
///
/// # Errors
///
/// [`Unsplit::Unread`] when the box varies in nothing the node reads;
/// otherwise the budget that stopped it: the per-axis depth bound where
/// it holds any variable the grid could still split, else the grid.
fn bisect(
    b: &Box_,
    root: &ParamBox,
    reads: &BTreeSet<VarId>,
    max_depth: u32,
) -> Result<(Box_, Box_), Unsplit> {
    let depth = |var: VarId| b.depths.get(&var).copied().unwrap_or(0);
    // Each variable the node reads, with its halves where the grid has
    // room for them.
    let mut halves: BTreeMap<VarId, Option<(ParamBox, ParamBox)>> = b
        .box_
        .varying()
        .filter(|(var, _, _)| reads.contains(var))
        .map(|(var, _, _)| (var, b.box_.split(var)))
        .collect();
    if halves.is_empty() {
        return Err(Unsplit::Unread);
    }
    let open = |var: VarId| depth(var) < max_depth && halves.get(&var).is_some_and(Option::is_some);
    let Some((axis, (lo, hi))) = b
        .box_
        .split_axis(root, open)
        .and_then(|axis| Some((axis, halves.remove(&axis)??)))
    else {
        let at_depth = halves.values().any(Option::is_some);
        return Err(Unsplit::Budget(if at_depth {
            BudgetKind::Depth { max_depth }
        } else {
            BudgetKind::Resolution
        }));
    };
    let mut depths = b.depths.clone();
    depths.insert(axis, depth(axis) + 1);
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
    for (name, _) in analyzed.in_order() {
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
fn probe_midpoint(
    doc: &Doc<ProfileProgram>,
    box_: &ParamBox,
    symbolic: SymbolicDials,
    stop: &crate::Stop,
    tol: Tol,
) {
    // THE SAME MIDPOINT THE SPLIT RULE USES, through the same door
    // (`BoxAxis::midpoint`). Spelling the midpoint here as well
    // would let a change to the split rule silently detach the K
    // population from the leaves it is supposed to describe: these are
    // the points the driver certified AROUND, and "around" is defined
    // by where it split.
    let mid: BTreeMap<VarId, BoxAxis> = box_
        .axes()
        .iter()
        .map(|(&n, a)| {
            let m = a.midpoint();
            (n, BoxAxis::Varying { lo: m, hi: m })
        })
        .collect();
    let opts = EvalOptions {
        param_box: Some(Arc::new(ParamBox::from_axes_in(mid, box_.order()))),
        ..lane_opts(stop)
    };
    // The replay runs at the SAME TIER the drive did (E12): with the
    // tier on the recording scalar is `Sym<Probe>`, so a margin that is
    // an identity lands in the funnel as `SampleOutcome::Symbolic`
    // and the hosted K row reports the symbolic/numeric split of the
    // driver's own population. Running it at bare `Probe` would report a
    // population the driver did not produce.
    if symbolic.enabled {
        let _ = geom_core::sym::with_session_retry(
            symbolic.budget(),
            symbolic.rules,
            symbolic.retry,
            || {
                let ev: Evaluation<Sym<geom_core::Probe>> =
                    evaluate(doc, None, &stop.cancel, &opts, tol);
                ev
            },
        );
        return;
    }
    let _: Evaluation<geom_core::Probe> = evaluate(doc, None, &stop.cancel, &opts, tol);
}

/// A box's goldening rendering: `id=[lo_bits,hi_bits]` per axis, in
/// declaration order, every bit of the id and floats as exact bits.
pub(crate) fn render_box(b: &ParamBox) -> String {
    use core::fmt::Write as _;
    let mut s = String::new();
    for (name, axis) in b.order().iter().filter_map(|n| Some((n, b.get(*n)?))) {
        let (lo, hi) = axis.span();
        let _ = write!(
            s,
            "{}=[{:016x},{:016x}] ",
            name.full(),
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
            // Deterministic by construction: `FlipSet` is a `BTreeMap`
            // by node id, with at most one turn per node.
            for (node, delta) in &flipped.verdicts.nodes {
                if delta.old_status != delta.new_status {
                    let _ = write!(
                        s,
                        " {}:status:{:?}->{:?}",
                        node.full(),
                        delta.old_status,
                        delta.new_status
                    );
                }
                if let Some(f) = delta.flip {
                    let _ = write!(
                        s,
                        " {}:{}:{:?}->{:?}",
                        node.full(),
                        f.predicate,
                        f.from,
                        f.to
                    );
                }
            }
            for f in &flipped.structure {
                let _ = write!(s, " {}:structure:{:?}", f.node.full(), f.refusal.decision);
            }
            if flipped.is_empty() {
                // No node's choices turned on a sign. Said out loud in
                // the goldening form, because a bare `flip_crossing`
                // would read as "no evidence gathered" rather than "no
                // turn found".
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
        RefusalReason::ReadsNoVariable { node, unsettled } => match unsettled {
            Unsettled::Undecided => format!("reads_no_variable {} undecided", node.full()),
            Unsettled::Failed { class, .. } => {
                format!("reads_no_variable {} failed:{class:?}", node.full())
            }
        },
    }
}

/// A mass column's goldening rendering: exact bits, or the refusal.
fn render_mass(m: &Result<f64, MeasureUnavailable>) -> String {
    match m {
        Ok(v) => format!("{:016x}", v.to_bits()),
        Err(MeasureUnavailable::BandHasNoMeasure { param }) => {
            format!("refused band:{}", param.id().full())
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
/// A clearance assertion ([`crate::Claim::Clearance`]) is answered the
/// same way, per leaf: its verdict is the sweep's at its bounds over
/// the leaf's box, each pair verified at the box's midpoint
/// ([`AssertionVerdict::Clearance`](crate::AssertionVerdict::Clearance)).
///
/// `Ok(None)` when `assertion` is not an `Assertion` node, or the
/// evaluation over `box_` did not produce a verdict for it.
///
/// # Errors
///
/// [`crate::Stopped`] when `stop` stopped the replay: its token was
/// canceled, or the run ran out of its member budget.
///
/// The scalar is the interval one, not a choice: a verdict over a BOX
/// is only meaningful from an enclosure, and the point scalars cannot
/// produce one.
pub fn assertion_at(
    doc: &Doc<ProfileProgram>,
    assertion: RecipeNodeId,
    box_: &ParamBox,
    symbolic: SymbolicDials,
    stop: &crate::Stop,
    tol: Tol,
) -> Result<Option<crate::measure::AssertionVerdict<geom_core::Interval>>, crate::Stopped> {
    if !matches!(doc.node(assertion), Some(Node::Assertion(_))) {
        return Ok(None);
    }
    // The SAME LANE the leaf was certified on ([`ParamBoxVerdict::symbolic`]).
    let readback = crate::eval::replay_leaf(
        doc,
        &EvalOptions {
            param_box: Some(Arc::new(box_.clone())),
            ..lane_opts(stop)
        },
        symbolic.lane(),
        &crate::eval::LeafPrior::None,
        &stop.cancel,
        crate::eval::LeafRequest {
            assertion: Some(assertion),
            ..crate::eval::LeafRequest::default()
        },
        tol,
    );
    match readback.stopped {
        Some(stopped) => Err(stopped),
        None => Ok(readback.assertion),
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

    use geom::{Curve3, Surface};
    use geom_brep::{EdgeCurve, EdgeCurveSpec, EdgeDescriptionSpec, SurfaceKey, SurfacePair};
    use geom_core::k_stats::{Bracket, Escalation};
    use geom_core::{Band, Point3, Real, Vec3};

    use super::*;

    fn band() -> Band {
        Band::linear(Tol::witness()).expect("the run's linear band")
    }

    /// A tangency certificate of the line `(x, 0, z)`, `z ∈ [0, 1]`,
    /// between a cylinder of radius `r` about z (as `s1`) and a plane
    /// through the line tilted by `tilt` — at `Interval`, the driver's
    /// lane — and the escalation log a node bracket would have kept.
    /// Re-spells `certify_line_at_interval` and `cylinder_of` in
    /// `geom-brep/tests/m5_pr9_tangent.rs`, whose rows pin these logs.
    fn tangent_log(r: Option<f64>, x: f64, tilt: f64) -> Vec<Escalation> {
        let lift = Interval::from_f64;
        let s1 = r.map_or(
            Surface::Plane {
                frame: geom::Frame::new(
                    Point3::new(x, 0.0, 0.0),
                    Vec3::new(1.0, 0.0, 0.0),
                    Vec3::new(0.0, 0.0, 1.0),
                ),
            },
            |radius| Surface::Cylinder {
                frame: geom::Frame::new(
                    Point3::new(0.0, 0.0, 0.0),
                    Vec3::new(0.0, 0.0, 1.0),
                    Vec3::new(1.0, 0.0, 0.0),
                ),
                radius,
            },
        );
        let (sin, cos) = tilt.sin_cos();
        let s2 = Surface::Plane {
            frame: geom::Frame::new(
                Point3::new(x, 0.0, 0.0),
                Vec3::new(cos, sin, 0.0),
                Vec3::new(0.0, 0.0, 1.0),
            ),
        };
        let mut arena: slotmap::SlotMap<SurfaceKey, Surface<Interval>> =
            slotmap::SlotMap::with_key();
        let k1 = arena.insert(s1.map_scalar(lift));
        let k2 = arena.insert(s2.map_scalar(lift));
        let carrier = Curve3::line(
            Point3::new(x, 0.0, 0.0),
            geom::stated_direction(Vec3::new(0.0, 0.0, 1.0)),
        )
        .map_scalar(lift);
        let (t0, t1) = (lift(0.0), lift(1.0));
        let (p0, p1) = (carrier.eval(t0), carrier.eval(t1));
        let spec = EdgeCurveSpec {
            description: EdgeDescriptionSpec::TangentIntersection {
                pair: SurfacePair::new(k1, k2),
                witness: carrier.eval(lift(0.5)),
            },
            carrier,
            param_start: t0,
            param_end: t1,
        };
        let bracket = Bracket::open();
        let refused = EdgeCurve::certify(
            spec,
            p0,
            p1,
            |k| arena.get(k).cloned().ok_or(geom_brep::SurfaceMiss::Stale),
            geom_core::test_support::fixture_lever(lift(1.0)),
            band(),
            geom_brep::NurbsLane::certified(),
        )
        .is_err();
        assert!(refused, "the configuration is not a tangency");
        bracket.finish().escalations
    }

    /// The sliver radius: its sagitta `R/2` over its own arm is `0.8·Kε`,
    /// wholly in the band. The same constant as `sliver_radius` in
    /// `geom-brep/tests/m5_pr9_tangent.rs`.
    fn sliver_radius() -> f64 {
        1.6 * band().escalate()
    }

    /// **A refusal the certificate renamed to `TangentParallel` reads as
    /// the definite refusal it is, not as an osculating sliver.** The
    /// second-order enclosure is wholly in band and the parallelism
    /// defect at the folded arm is definite (`sin θ = 0.9`): the log is
    /// empty, so read (1) passes the node to read (2), which reads the
    /// error the same way whatever order the certificate's readings took.
    #[test]
    fn a_renamed_tangent_refusal_is_not_a_second_order_sliver() {
        let r = sliver_radius();
        let log = tangent_log(Some(r), r, 0.9f64.asin());
        assert!(
            log_read(&log).is_none(),
            "the log named a cause the error does not: {log:?}"
        );
    }

    /// **A definite second-order refusal is not a parallelism sliver.**
    /// Two planes through one line refuse `NotSecondOrderSeparated`
    /// definitely; the naming reading's in-band defect at the extent
    /// only names, so it is not on the log.
    #[test]
    fn a_definite_second_order_refusal_is_not_a_parallelism_sliver() {
        let b = band();
        let in_band = (b.zero() * b.escalate()).sqrt().asin();
        let log = tangent_log(None, 1.0, in_band);
        assert!(
            log_read(&log).is_none(),
            "the naming reading's escalation spoke for the node: {log:?}"
        );
    }

    /// The control: where the in-band second-order reading IS the
    /// refusal (`sin θ = 0.3`, a defect in band too), the same cylinder
    /// is a terminal sliver named by `tangent_second_order`.
    #[test]
    fn an_in_band_second_order_refusal_is_a_second_order_sliver() {
        let r = sliver_radius();
        let log = tangent_log(Some(r), r, 0.3f64.asin());
        assert!(
            matches!(
                log_read(&log),
                Some(Undecided::Sliver("tangent_second_order"))
            ),
            "the in-band second-order refusal is a sliver: {log:?}"
        );
    }

    /// The slab's witness build at `f64` with its extrude failed for an
    /// empty operand naming its profile: a failure whose cause names
    /// another node.
    fn slab_failing_naming_its_profile() -> (
        crate::test_support::MeasuredSlab,
        AnalyzedBox,
        Evaluation<f64>,
    ) {
        let slab = crate::test_support::measured_slab("drive-cause-mentions", 1.0, Tol::witness());
        let analyzed =
            crate::analysis::analyzed_box(&slab.doc, &crate::analysis::AnalysisPolicy::default());
        let opts = witness_options(&analyzed, &crate::Stop::default());
        let mut built: Evaluation<f64> = evaluate(
            &slab.doc,
            None,
            &crate::CancelToken::new(),
            &opts,
            Tol::witness(),
        );
        built.nodes.insert(
            slab.block,
            NodeResult::Failed(crate::eval::NodeError {
                node: slab.block,
                kind: NodeErrorKind::EmptyOperand {
                    input: slab.profile,
                },
                escalations: Arc::new(Vec::new()),
                sym: std::sync::Arc::default(),
            }),
        );
        (slab, analyzed, built)
    }

    /// Whether `said` mentions `node` in the document `doc`.
    fn mentions(said: &crate::Sentence, node: RecipeNodeId, doc: crate::DocumentId) -> bool {
        said.mentions()
            .any(|m| m.node().map(SpokenNode::id) == Some(node) && m.document() == Some(doc))
    }

    /// **A witness that does not build keeps the nodes its cause names
    /// as mentions**: the extrude's empty operand names the profile, and
    /// the cause, and the refusal said from the document, mention it in
    /// the driven document. The Monte-Carlo nominal refuses on the same
    /// first failure ([`first_unusable`]).
    #[test]
    fn a_witness_refusals_cause_mentions_the_node_it_names() {
        let (slab, _, built) = slab_failing_naming_its_profile();
        let id = slab.doc.id();
        let refusal = does_not_build(&slab.doc, &built).expect("the failed extrude refuses");
        let DriveRefusal::WitnessDoesNotBuild { node, cause } = &refusal else {
            panic!("the failed extrude refuses the witness");
        };
        assert_eq!(node.id(), slab.block, "the first node that did not build");
        assert!(
            mentions(cause, slab.profile, id),
            "the cause mentions the profile in the document: {cause:?}"
        );
        let said = crate::Said(&refusal, crate::Speaker::of(&slab.doc)).sentence();
        assert!(
            mentions(&said, slab.profile, id) && mentions(&said, slab.block, id),
            "the refusal keeps the cause's mention beside its node's: {said:?}"
        );
        assert_eq!(
            first_unusable(&slab.doc, &built).map(|(node, cause)| (node.id(), cause)),
            Some((slab.block, cause.clone())),
            "the Monte-Carlo nominal's refusal reads the same"
        );
    }

    /// **A leaf whose deciding node fails keeps the nodes its cause
    /// names as mentions**: [`Unsettled::Failed`]'s cause mentions the
    /// profile the extrude's empty operand names, in the document.
    #[test]
    fn a_failed_leafs_cause_mentions_the_node_it_names() {
        let (slab, analyzed, leaf) = slab_failing_naming_its_profile();
        let id = slab.doc.id();
        let witness = witness_build(
            &slab.doc,
            &analyzed,
            None,
            &crate::Stop::default(),
            Tol::witness(),
        )
        .expect("the slab builds");
        let vector = certifying_vector(&slab.doc, &witness);
        let verdict = classify_replay(
            &slab.doc,
            &ParamBox::of(&analyzed),
            &leaf,
            &witness,
            &vector,
            vector.key(),
            SymCounts::default(),
        );
        let LeafVerdict::Bisect {
            deciding,
            unsettled: Unsettled::Failed { cause, .. },
        } = &verdict
        else {
            panic!("the failed extrude does not settle the leaf");
        };
        assert_eq!(*deciding, slab.block);
        assert!(
            mentions(cause, slab.profile, id),
            "the leaf's cause mentions the profile in the document: {cause:?}"
        );
    }

    /// **The human form, word for word**: the slab driven over its
    /// depth on a budget of four leaves, the symbolic tier off so no
    /// decision count rides it. Four leaves certify none, so each
    /// refuses on the budget and the mass budget follows, priced and
    /// conservative. Pins the
    /// report's text ([`ParamBoxVerdict::render`] and the
    /// [`crate::report::MassBudget::render`] under it), so a break or a
    /// word moved is red here.
    #[test]
    fn the_drive_report_reads_word_for_word() {
        let slab = crate::test_support::measured_slab("drive-report-golden", 1.0, Tol::witness());
        let analyzed =
            crate::analysis::analyzed_box(&slab.doc, &crate::analysis::AnalysisPolicy::default());
        let config = DriveConfig {
            symbolic: SymbolicDials::off(),
            max_leaves: 4,
            ..DriveConfig::default()
        };
        let verdict = drive(
            &slab.doc,
            &analyzed,
            &config,
            &crate::Stop::default(),
            Tol::witness(),
        )
        .expect("the slab drives");
        let lines = verdict.render(&slab.doc, &analyzed);
        let budget_text = "priced: every varying parameter carries a stated distribution, so these masses are \
         integrals of a stated measure\n  certified   0.0000%\n  refused (budget) 100.0000%\n  \
         tail        0.0000%\n  UNRESOLVED  100.0000% (conservative: the witness chamber may \
         reach past the analyzed box)\n";
        assert_eq!(
            lines.to_string(),
            format!(
                "drive over 1 axis/axes: 0 certified, 4 refused (3 splits; receipt holds)\n  4 \
                 leaf/leaves refused budget\n{budget_text}"
            ),
            "the drive's report"
        );
        let budget =
            crate::report::MassBudget::of(verdict.accounting(), &analyzed).render(&slab.doc);
        assert_eq!(budget.to_string(), budget_text, "the mass budget");
        assert_eq!(budget.len(), 5, "one sentence per line");
        assert_eq!(
            lines
                .iter()
                .filter(|line| line.mentions().next().is_some())
                .count(),
            0,
            "a certified drive over a named depth names no node"
        );
    }
}
