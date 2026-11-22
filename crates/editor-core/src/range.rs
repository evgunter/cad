//! **The certified locally-valid range**: how far one field can move
//! before the proof stops, answered by replaying intervals of the field
//! under the E6 leaf protocol rather than by sampling.
//!
//! # What this answers
//!
//! Over which interval around the field's current value is the document
//! PROVABLY the same build? A certificate asks "does anything DECIDE
//! differently", which is strictly more than "does anything new fail",
//! so every value this module proves is locally valid — and a value it
//! does not prove is not thereby invalid.
//!
//! # The search
//!
//! [`RangeSearch`] is a stepped value. [`RangeSearch::new`] admits the
//! request and evaluates the witness once; [`RangeSearch::step`] replays
//! ONE interval; [`RangeSearch::answer`] can be read after any step, and
//! is a pure function of the inputs and the number of steps taken — no
//! clock is read, so stopping keeps what was proven.
//!
//! The schedule refines TOWARD the current value:
//!
//! 1. The seed is split at the current value, and each side is replayed
//!    whole.
//! 2. An interval the replay could not decide is halved and BOTH halves
//!    are replayed, the one nearer the current value first.
//! 3. The interval halved next is the undecided one nearest the current
//!    value, on either side (a tie goes to the wider interval, then to
//!    the low side).
//!
//! The far half is replayed and not skipped, because that is the only
//! way a difference is ever PROVEN: a certified interval and one proven
//! different cannot be neighbours (a margin is definite and of one sign
//! over the first, definite and of another over the second, and both
//! enclose the shared endpoint), so every boundary has undecided ground
//! on its near side and a schedule that only grew the proven interval
//! outward would stop at that ground without ever seeing past it.
//!
//! Proving width `w` on a side of width `W` therefore costs about
//! `2·log2(W/w)` replays where a level-by-level subdivision of the seed
//! costs `W/w` leaves, which is what makes a wide seed affordable.
//!
//! # What it costs, measured
//!
//! A replay's cost is how far it gets before something is undecided,
//! so it spans orders of magnitude on one document. Per replay, dev
//! profile, on a 4-vCPU x86_64 cloud container (Linux 6.18) with a
//! second build sharing its cores:
//!
//! | document | field | an interval that refuses early | undecided | certifies, or nearly |
//! | --- | --- | --- | --- | --- |
//! | `corpus::plate_param` (7 nodes) | `hole_r` | 0.1 s | 1.4–4.5 s | 21–24 s |
//! | `corpus::die` (57 nodes) | `pip_depth` | 0.2 s | 2.6 s | 115–132 s, none certified in 8 |
//! | `corpus::die` | the cube's `Distance` slot | 0.4 s | 1.8–2.1 s | 26–42 s, none certified in 8 |
//!
//! Re-take them rather than trusting them
//! (`docm9_range::the_per_replay_cost_on_the_corpus`, ignored, prints
//! the rows).
//!
//! The width that certifies is the driver's, not this module's. The
//! plate's `hole_r` certifies at about `ε/4` and no wider (an arc rim's
//! endpoint pinning): 47 replays from a seed of `±1e-6`, where a
//! level-by-level subdivision of that seed needs 2¹³ leaves. A field
//! feeding a PARAMETRIC POLYGON VERTEX certifies at about `10 ε`
//! (`work/nurbs/parametric-polygon-loop-certifies-nothing`); what
//! certifies over a macroscopic interval is straight geometry whose
//! identities the symbolic tier cancels.
//!
//! # The budget
//!
//! [`DriveConfig::max_leaves`] is the number of replays the search may
//! take and [`DriveConfig::max_depth`] how many times one side may be
//! halved; `symbolic` and `plain_memo` are the leaf protocol's own.
//! `parallel` is not read: a step is one replay. A depth costs two
//! replays a level here, so it is sized to the seed and not to a leaf
//! count: reaching a width that certifies near `ε` from a side of
//! width `W` takes `log2(W/ε)` halvings, more than
//! [`crate::drive::DEFAULT_MAX_DEPTH`] for a side wider than `2²⁴ ε`,
//! and the `f64` grid bounds it whatever the dial says. Ground the search has
//! not replayed, or has replayed and not yet halved, is priced
//! [`RefusalReason::Budget`] (E6) under the replays taken so far.
//!
//! # The one field, and where its widening lives
//!
//! The widening is a property of the QUERY, never of the document. A
//! node SLOT reads a variable (VR4), so a slot is boxed through the
//! free variable it reads, exactly as a variable named directly is:
//! the query derives a document of its own, a clone with that
//! variable's spread set to the seed. The input document is never
//! edited, keyed or persisted, its memo is not shared, and the
//! derivation is a pure function of (document, field, seed) that lives
//! only for the query ([`derive`]).
//!
//! Every OTHER parameter's distribution is cleared in the derived
//! document, so the replay has exactly one varying axis. The clearing
//! is a CONDITION on the answer — a range taken this way holds with the
//! other parameters at their nominals and says nothing about their
//! spreads — so the names it pinned ride on the answer
//! ([`CertifiedRange::pinned`]) for a consumer to state.
//!
//! # The seed
//!
//! The seed is how far the search looks, and the answer states it
//! ([`CertifiedRange::seed`]): outside it nothing was replayed, so a
//! side that proves all of it has reached the SEED's end and never
//! "unbounded". A caller may state its own; [`RangeSeed::of`] is the
//! one this module chooses.
//!
//! # Offsets, not absolute values
//!
//! Every interval here is an OFFSET from the field's nominal, in the
//! field's own dimension, because that is what the analyzed axis is:
//! [`Distribution::Band`] states offsets and
//! [`crate::analysis::analyzed_box`] carries them into the box
//! verbatim, so a seed stated this way IS the axis, exactly.
//! [`CertifiedRange::absolute`] converts for a reader.

use std::sync::Arc;

use geom_core::Tol;

use crate::analysis::{AnalysisPolicy, BoxAxis, ParamBox, analyzed_box};
use crate::distribution::Distribution;
use crate::doc::{Doc, FreeVar};
use crate::drive::{
    BudgetKind, DriveConfig, DriveRefusal, FlipEvidence, LeafVerdict, RefusalReason, VerdictVector,
    VerdictVectorKey, certifying_vector, classify, does_not_build, drive_memo, witness_build,
};
use crate::edit::{DocEdit, EditError, apply};
use crate::eval::Evaluation;
use crate::node::{RecipeNodeId, SlotId};
use crate::part::PartResolver;
use crate::program::ProfileProgram;
use crate::spoken::SpokenNode;
use crate::spoken::SpokenVar;
use crate::var::VarId;

/// The field a range is asked about: one document parameter, or one
/// node slot.
///
/// A slot is boxed through the free variable it reads; one that reads a
/// defined variable has no interval of its own
/// ([`RangeRefusal::SlotIsDefined`]).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RangeField {
    /// A free variable of the document, boxed directly.
    Param(VarId),
    /// A continuous slot of one node, widened through the free variable
    /// it reads. That variable may be shared — a named one several
    /// slots read (D10: sharing is reading one variable; VR2: a shared
    /// variable is named) — and widening it moves every reader, not
    /// only this slot: the certified range is the variable's, reported
    /// against the slot it was asked through.
    Slot {
        /// The node owning the slot.
        node: RecipeNodeId,
        /// The slot.
        slot: SlotId,
    },
}

/// How far the search looks, as OFFSETS from the field's nominal
/// (module docs: the analyzed axis IS offsets, so a seed stated this
/// way reaches the replay exactly).
///
/// `lo <= 0 <= hi` and `lo < hi`: the seed brackets the value the
/// field has now, which is the value the witness build is taken at. A
/// range over an interval that excludes the nominal is a range of a
/// DIFFERENT document — set the field there and seed around it.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RangeSeed {
    /// Lower offset (`<= 0`).
    pub lo: f64,
    /// Upper offset (`>= 0`).
    pub hi: f64,
}

impl RangeSeed {
    /// A symmetric seed of half-width `w`. Infallible: a negative, zero
    /// or non-finite `w` builds a seed [`admit`] refuses
    /// [`RangeRefusal::SeedNotABracket`], naming the offsets it was
    /// handed — one door for one invariant.
    #[must_use]
    pub fn symmetric(w: f64) -> Self {
        Self { lo: -w, hi: w }
    }

    /// **The seed this module chooses for a field**: from zero to twice
    /// the current value, `±|nominal|`, and `±1` for a field at zero.
    ///
    /// No field of this kernel states an admissible domain — a slot
    /// states a dimension ([`SlotId::dimension`]) and a variable a
    /// value, and a length is signed — so the span is fixed relative to
    /// the current value. Zero is the end it is sized to reach: it is
    /// where a depth, a radius and a width stop building.
    ///
    /// # Errors
    ///
    /// [`RangeRefusal`] for a field [`admit`] refuses.
    pub fn of(doc: &Doc<ProfileProgram>, field: &RangeField) -> Result<Self, RangeRefusal> {
        let nominal = axis_of(doc, field)?.1;
        Ok(Self::symmetric(if nominal == 0.0 {
            1.0
        } else {
            nominal.abs()
        }))
    }
}

/// **What a proven difference is**, read off the driver's evidence
/// ([`ProvenDifferent::kind`]).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DifferenceKind {
    /// A node's STANDING differs from the witness's: one that built
    /// does not, or the reverse.
    StandingChange {
        /// The first such node, in id order.
        node: RecipeNodeId,
    },
    /// Every node stands as it did at the witness and a recorded
    /// decision goes the other way: the document builds, differently.
    /// The boundary of the CERTIFICATE, not of validity.
    BuildsDifferently,
}

/// **A point at which the document is proven NOT the witness's build**:
/// the near edge of the nearest interval the replay decided definitely
/// otherwise ([`RefusalReason::FlipCrossing`]).
#[derive(Debug, Clone, PartialEq)]
pub struct ProvenDifferent {
    at: f64,
    evidence: Box<FlipEvidence>,
}

impl ProvenDifferent {
    /// The point, as an offset.
    #[must_use]
    pub fn at(&self) -> f64 {
        self.at
    }

    /// The driver's evidence, verbatim.
    #[must_use]
    pub fn evidence(&self) -> &FlipEvidence {
        &self.evidence
    }

    /// Whether a node's standing changed, or the document builds on
    /// another branch. `resolve::vdiff`'s `FlipSet` holds a node whose
    /// standing differs beside one that turned, so membership there
    /// cannot tell the two apart; this reads the standings.
    #[must_use]
    pub fn kind(&self) -> DifferenceKind {
        self.evidence
            .verdicts
            .nodes
            .iter()
            .find(|(_, d)| d.old_status != d.new_status)
            .map_or(DifferenceKind::BuildsDifferently, |(node, _)| {
                DifferenceKind::StandingChange { node: *node }
            })
    }
}

/// **Ground the search did not decide**, outward from `past`.
///
/// Not a bound, and never to be rendered as one: nothing here says the
/// document is valid past `past`, or that it is not.
#[derive(Debug, Clone, PartialEq)]
pub struct NotDecided {
    past: f64,
    reason: RefusalReason,
}

impl NotDecided {
    /// The offset the undecided ground starts at: the proven frontier.
    #[must_use]
    pub fn past(&self) -> f64 {
        self.past
    }

    /// Why the interval at the frontier is undecided: the driver's
    /// typed reason, or [`BudgetKind::Leaves`] under the replays taken
    /// for ground the search has not finished with.
    #[must_use]
    pub fn reason(&self) -> &RefusalReason {
        &self.reason
    }

    /// Whether more replays can move the frontier. Only a spent replay
    /// count answers yes: a depth floor, the `f64` grid and a terminal
    /// sliver are where refinement has stopped moving.
    #[must_use]
    pub fn more_time_helps(&self) -> bool {
        matches!(
            self.reason,
            RefusalReason::Budget(BudgetKind::Leaves { .. })
        )
    }
}

/// How a side ends, where the search can say.
#[derive(Debug, Clone, PartialEq)]
pub enum SideEnd {
    /// Everything from the current value to the seed's end is proven.
    /// Never a claim about values outside the seed, which were not
    /// replayed.
    Seed,
    /// The nearest point proven different.
    Different(ProvenDifferent),
}

/// **One side's answer.**
///
/// `proven_to` is the far end of the proven interval on this side (zero
/// when nothing is). Between it and a [`SideEnd::Different`] point lies
/// ground the search did NOT decide — [`Self::not_decided`] — which a
/// consumer never renders as valid: a node that fails on part of an
/// interval is among it (`drive::classify_replay` bisects such an
/// interval to the floor and prices it `Budget`;
/// `work/verdict/coincidence-zone-priced-budget-at-the-floor`).
///
/// [`Self::end`] is `None` exactly when the side is undecided with
/// nothing proven different beyond; [`Self::not_decided`] is `None`
/// exactly when the proven interval reaches the seed's end, or runs up
/// to the point proven different.
#[derive(Debug, Clone, PartialEq)]
pub struct RangeSide {
    proven_to: f64,
    end: Option<SideEnd>,
    not_decided: Option<NotDecided>,
}

impl RangeSide {
    /// The far end of the proven interval on this side, as an offset.
    #[must_use]
    pub fn proven_to(&self) -> f64 {
        self.proven_to
    }

    /// How the side ends, where the search can say.
    #[must_use]
    pub fn end(&self) -> Option<&SideEnd> {
        self.end.as_ref()
    }

    /// The point proven different, where there is one.
    #[must_use]
    pub fn different(&self) -> Option<&ProvenDifferent> {
        match &self.end {
            Some(SideEnd::Different(d)) => Some(d),
            Some(SideEnd::Seed) | None => None,
        }
    }

    /// Whether the proven interval reaches the seed's end.
    #[must_use]
    pub fn reached_seed_end(&self) -> bool {
        matches!(self.end, Some(SideEnd::Seed))
    }

    /// The undecided ground at the proven frontier, where there is any.
    #[must_use]
    pub fn not_decided(&self) -> Option<&NotDecided> {
        self.not_decided.as_ref()
    }
}

/// The search's answer: a proven interval around the field's current
/// value, and one [`RangeSide`] per direction.
///
/// **It is a CONDITIONAL answer and carries its condition**
/// ([`Self::pinned`]): the derivation holds every other annotated
/// parameter at its nominal.
#[derive(Debug, Clone, PartialEq)]
pub struct CertifiedRange {
    field: RangeField,
    nominal: f64,
    seed: RangeSeed,
    pinned: Vec<VarId>,
    steps: usize,
    lo: RangeSide,
    hi: RangeSide,
}

impl CertifiedRange {
    /// The field this range is about.
    #[must_use]
    pub fn field(&self) -> &RangeField {
        &self.field
    }

    /// The field's value in the input document — the point the witness
    /// build is taken at, and the origin every offset here is measured
    /// from.
    #[must_use]
    pub fn nominal(&self) -> f64 {
        self.nominal
    }

    /// How far the search looked, as offsets.
    #[must_use]
    pub fn seed(&self) -> RangeSeed {
        self.seed
    }

    /// **The condition this answer holds under**: the parameters whose
    /// declared distribution the derivation cleared, in declaration
    /// order. Every one of them is at its nominal for the whole
    /// certificate; empty means the document declared no other spread.
    #[must_use]
    pub fn pinned(&self) -> &[VarId] {
        &self.pinned
    }

    /// How many replays this answer is the answer after.
    #[must_use]
    pub fn steps(&self) -> usize {
        self.steps
    }

    /// The downward side.
    #[must_use]
    pub fn lo(&self) -> &RangeSide {
        &self.lo
    }

    /// The upward side.
    #[must_use]
    pub fn hi(&self) -> &RangeSide {
        &self.hi
    }

    /// An offset read as an absolute field value, `nominal + offset` at
    /// `f64`. A reader's convenience: the PROOF is over the offsets,
    /// which the replayed axis carries exactly, and this sum rounds.
    #[must_use]
    pub fn absolute(&self, offset: f64) -> f64 {
        self.nominal + offset
    }

    /// The proven interval as offsets, derived from the two sides:
    /// `(lo.proven_to(), hi.proven_to())`. Every value in it builds
    /// exactly what the witness built, with [`Self::pinned`] at their
    /// nominals. `(0, 0)` when nothing is proven
    /// ([`Self::nothing_proven`]).
    #[must_use]
    pub fn proven_offsets(&self) -> (f64, f64) {
        (self.lo.proven_to, self.hi.proven_to)
    }

    /// The proven interval in absolute field values
    /// ([`Self::absolute`] of [`Self::proven_offsets`]).
    #[must_use]
    pub fn proven_interval(&self) -> (f64, f64) {
        let (lo, hi) = self.proven_offsets();
        (self.absolute(lo), self.absolute(hi))
    }

    /// Whether the proven interval is empty: neither side proved
    /// anything around the current value.
    #[must_use]
    pub fn nothing_proven(&self) -> bool {
        self.lo.proven_to == 0.0 && self.hi.proven_to == 0.0
    }
}

/// Why a range could not be asked for at all.
///
/// Every arm is a fact about the REQUEST or about the document's
/// shape, decided before any geometry runs ([`admit`]) — except
/// [`Self::Drive`], the witness build's refusal, carried verbatim.
#[derive(Debug, Clone, PartialEq)]
pub enum RangeRefusal {
    /// The seed is not a bracket of the nominal: an offset is not
    /// finite, `lo <= 0 <= hi` fails, or the seed has no width.
    SeedNotABracket {
        /// The lower offset asked for.
        lo: f64,
        /// The upper offset asked for.
        hi: f64,
    },
    /// The document declares no such parameter, or declares it
    /// `Count` — a structural parameter is not a box axis.
    NotAContinuousParam {
        /// The variable asked for.
        param: SpokenVar,
    },
    /// The document has no such node.
    UnknownNode {
        /// The id asked for, which the document does not hold
        /// ([`SpokenNode::absent`]).
        node: SpokenNode,
    },
    /// The node carries no such slot.
    UnknownSlot {
        /// The node, spoken from the document asked about.
        node: SpokenNode,
        /// The slot asked for.
        slot: SlotId,
    },
    /// The slot is STRUCTURAL (`Count`-typed): it selects between
    /// shapes rather than measuring one, and a structural slot has no
    /// interval to certify over.
    StructuralSlot {
        /// The node, spoken from the document asked about.
        node: SpokenNode,
        /// The slot.
        slot: SlotId,
    },
    /// The slot reads a DEFINED variable: its value is a function of
    /// other variables, so it has no interval of its own to certify
    /// over — certify the variables its definition reads instead.
    SlotIsDefined {
        /// The node, spoken from the document asked about.
        node: SpokenNode,
        /// The slot.
        slot: SlotId,
    },
    /// The derived document's own edit door refused the derivation.
    Derivation(Box<EditError>),
    /// The seed did not reach the driver as the analyzed axis: the
    /// interval the analysis derived for the field is not the one that
    /// was asked for. The query refuses rather than certifying a box
    /// it did not mean.
    ///
    /// Unreachable through [`derive()`], whose `Band` states the seed's
    /// own offsets and which `analyzed_box` carries verbatim: the guard
    /// on that identity.
    SeedIsNotTheAnalyzedAxis {
        /// The axis the analysis produced, as offsets.
        analyzed: (f64, f64),
        /// The seed that was asked for, as offsets.
        asked: (f64, f64),
    },
    /// The derived document's box has some other varying axis, which
    /// [`derive()`]'s clearing leaves none of: the guard on that.
    MoreThanOneAxisVaries {
        /// How many axes of the derived box vary.
        varying: usize,
    },
    /// The witness does not build, or its build was stopped: the
    /// driver's refusal, verbatim.
    Drive(Box<DriveRefusal>),
}

impl core::fmt::Display for RangeRefusal {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::SeedNotABracket { lo, hi } => write!(
                f,
                "a seed brackets the field's current value: offsets [{lo}, {hi}] must be finite \
                 with lo <= 0 <= hi and some width"
            ),
            Self::NotAContinuousParam { param } => write!(
                f,
                "{param} is not a continuous parameter of this document — there is no axis to \
                 certify over"
            ),
            Self::UnknownNode { node } => {
                write!(f, "this document has no {node}")
            }
            Self::UnknownSlot { node, slot } => {
                write!(f, "{node} carries no {} slot", slot.label())
            }
            Self::StructuralSlot { node, slot } => write!(
                f,
                "the {} slot of {} is structural (Count) — a structural slot selects \
                 between shapes and has no interval to certify over",
                slot.label(),
                node
            ),
            Self::SlotIsDefined { node, slot } => write!(
                f,
                "the {} slot of {} reads a variable defined by a formula, which has no \
                 interval of its own. {}",
                slot.label(),
                node,
                crate::sentence::Recourse("certify the variables that formula reads instead")
            ),
            // Every derived edit re-writes a value the document already
            // holds, or declares one over a validated seed, so the edit
            // door's recourse — about an edit nobody made — is not this
            // door's.
            Self::Derivation(e) => write!(
                f,
                "the derived document was refused: {}. {}",
                e.problem(),
                geom_core::KERNEL_DEFECT_ENDING
            ),
            Self::SeedIsNotTheAnalyzedAxis {
                analyzed: (analyzed_lo, analyzed_hi),
                asked: (asked_lo, asked_hi),
            } => write!(
                f,
                "the analysis derived the axis [{analyzed_lo}, {analyzed_hi}] for this field, \
                 which is not the seed [{asked_lo}, {asked_hi}] that was asked for. {}",
                geom_core::KERNEL_DEFECT_ENDING
            ),
            Self::MoreThanOneAxisVaries { varying } => write!(
                f,
                "the derived document's box varies on {varying} axes, where this query's own \
                 derivation leaves one. {}",
                geom_core::KERNEL_DEFECT_ENDING
            ),
            Self::Drive(e) => write!(f, "{e}"),
        }
    }
}

impl core::error::Error for RangeRefusal {}

/// The document the query actually drives, and the axis it drives it
/// over.
///
/// A pure function of (document, field, seed): nothing here is stored
/// on the input, keyed against it, or shared with its memo.
#[derive(Debug, Clone, PartialEq)]
pub struct DerivedRange {
    /// The document the search replays: the input, its spreads set as
    /// [`derive`](fn@derive) says.
    pub doc: Doc<ProfileProgram>,
    /// The variable whose axis IS the seed.
    pub axis: VarId,
    /// The field's value in the input document, bit for bit.
    pub nominal: f64,
    /// The parameters whose declared distribution this derivation
    /// CLEARED, in declaration order — the condition the answer holds under
    /// ([`CertifiedRange::pinned`]).
    pub pinned: Vec<VarId>,
}

/// What [`admit`] resolved a request to.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Admission {
    /// The free variable the field is boxed through.
    pub axis: VarId,
    /// The field's value in the document, bit for bit.
    pub nominal: f64,
}

/// **The one admission door**: whether this request can be searched at
/// all, decided with NO geometry run — a consumer asks it before
/// offering the query.
///
/// It refuses, in this order, a seed that is not a bracket of the
/// current value, then a field with no axis: a variable the document
/// does not declare continuous, a node or slot it does not carry, a
/// structural slot, a slot reading a defined variable. `built` is the
/// caller's finished `f64` build of this document, where it holds one:
/// a node that is not usable there refuses as the witness build would
/// ([`DriveRefusal::WitnessDoesNotBuild`]), without building it again.
///
/// # Errors
///
/// [`RangeRefusal`], every arm but the two derivation guards.
pub fn admit(
    doc: &Doc<ProfileProgram>,
    field: &RangeField,
    seed: RangeSeed,
    built: Option<&Evaluation<f64>>,
) -> Result<Admission, RangeRefusal> {
    if !seed.lo.is_finite()
        || !seed.hi.is_finite()
        || seed.lo > 0.0
        || seed.hi < 0.0
        || seed.lo >= seed.hi
    {
        return Err(RangeRefusal::SeedNotABracket {
            lo: seed.lo,
            hi: seed.hi,
        });
    }
    let (axis, nominal) = axis_of(doc, field)?;
    if let Some(refusal) = built.and_then(|built| does_not_build(doc, built)) {
        return Err(RangeRefusal::Drive(Box::new(refusal)));
    }
    Ok(Admission { axis, nominal })
}

/// The free variable a field is boxed through, and its value.
fn axis_of(doc: &Doc<ProfileProgram>, field: &RangeField) -> Result<(VarId, f64), RangeRefusal> {
    match field {
        RangeField::Param(var) => {
            let Some(FreeVar::Continuous { value, .. }) = doc.free(*var) else {
                return Err(RangeRefusal::NotAContinuousParam {
                    param: doc.spoken_var(*var),
                });
            };
            Ok((*var, *value))
        }
        RangeField::Slot { node, slot } => {
            let Some(n) = doc.node(*node) else {
                return Err(RangeRefusal::UnknownNode {
                    node: SpokenNode::absent(*node),
                });
            };
            // Whether THIS node carries the slot is asked first: a slot
            // id is a vocabulary-wide name, so `Count` is structural
            // whatever node it is aimed at, and "the count slot of node
            // N is structural" about a node with no count slot states
            // the vocabulary's fact where the caller asked about this
            // document's.
            let Some(var) = n.expr(*slot) else {
                return Err(RangeRefusal::UnknownSlot {
                    node: doc.spoken(*node),
                    slot: *slot,
                });
            };
            if slot.is_structural() {
                return Err(RangeRefusal::StructuralSlot {
                    node: doc.spoken(*node),
                    slot: *slot,
                });
            }
            let Some(FreeVar::Continuous { value, .. }) = doc.free(*var) else {
                return Err(RangeRefusal::SlotIsDefined {
                    node: doc.spoken(*node),
                    slot: *slot,
                });
            };
            Ok((*var, *value))
        }
    }
}

/// **The derived document** (a pure function of its three arguments).
///
/// The document is the input with the field's variable's distribution
/// set to the seed: the variable itself for [`RangeField::Param`], the
/// free variable the slot reads for [`RangeField::Slot`] (VR4), widened
/// in place — nothing declared, nothing named, nothing rewritten.
///
/// Every other continuous parameter's distribution is CLEARED, so the
/// drive has exactly one varying axis (module docs).
///
/// # Errors
///
/// [`RangeRefusal`] for a request [`admit`] refuses.
pub fn derive(
    doc: &Doc<ProfileProgram>,
    field: &RangeField,
    seed: RangeSeed,
    tol: Tol,
) -> Result<DerivedRange, RangeRefusal> {
    let Admission { axis, nominal } = admit(doc, field, seed, None)?;
    let band = Distribution::Band {
        lo: seed.lo,
        hi: seed.hi,
    };
    let mut derived = doc.clone();
    // The axis takes the seed; every other continuous parameter is
    // pinned at its nominal, because this query's contract is one
    // field.
    let annotated: Vec<(VarId, Option<Distribution>)> = derived
        .free_vars()
        .filter_map(|(id, free)| match free {
            FreeVar::Continuous { distribution, .. } => {
                let wanted = if id == axis { Some(band) } else { None };
                (*distribution != wanted).then_some((id, wanted))
            }
            FreeVar::Count { .. } => None,
        })
        .collect();
    // What the clearing PINNED: the parameters that had a declared
    // spread and lost it, which is the condition the answer holds
    // under. The axis itself is never in the list — it did not lose a
    // spread, it was given one.
    let pinned: Vec<VarId> = annotated
        .iter()
        .filter(|(id, wanted)| *id != axis && wanted.is_none())
        .map(|(id, _)| *id)
        .collect();
    for (var, distribution) in annotated {
        derived = edit(
            &derived,
            &DocEdit::SetVarDistribution {
                var: var.into(),
                distribution,
            },
            tol,
        )?;
    }
    Ok(DerivedRange {
        doc: derived,
        axis,
        nominal,
        pinned,
    })
}

/// One edit, applied purely, with the door's refusal carried. The
/// edits this module applies are document-parameter edits, which
/// add no constraint, so the reach is the refusing one: it
/// is never asked, and a door that did ask would refuse typed rather
/// than lever over nothing.
fn edit(
    doc: &Doc<ProfileProgram>,
    e: &DocEdit<ProfileProgram>,
    tol: Tol,
) -> Result<Doc<ProfileProgram>, RangeRefusal> {
    apply(doc, e, tol)
        .map(|a| a.doc)
        .map_err(|e| RangeRefusal::Derivation(Box::new(e)))
}

/// **The certified locally-valid range of one field**: the search
/// ([`RangeSearch`]) run until it finishes or `stop` stops it.
///
/// # Errors
///
/// [`RangeRefusal`] when the request is not admitted ([`admit`]), the
/// witness does not build, or `stop` stops the witness build. A stop
/// after that keeps what was proven and is not a refusal.
pub fn certified_range(
    doc: &Doc<ProfileProgram>,
    field: &RangeField,
    seed: RangeSeed,
    config: &DriveConfig,
    resolver: Option<Arc<dyn PartResolver>>,
    stop: &crate::Stop,
    tol: Tol,
) -> Result<CertifiedRange, RangeRefusal> {
    let mut search = RangeSearch::new(doc, field, seed, config, resolver, stop, tol)?;
    while search.step(stop) == Step::Advanced {}
    Ok(search.answer())
}

/// What one [`RangeSearch::step`] did.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[must_use]
pub enum Step {
    /// One interval was replayed; there may be more to do.
    Advanced,
    /// Nothing was replayed: every interval is decided or at its floor,
    /// or the replay count ([`DriveConfig::max_leaves`]) is spent.
    Finished,
    /// The replay stopped short ([`crate::Stop`]) and decided nothing;
    /// the search stands where it stood.
    Stopped(crate::Stopped),
}

/// What the search knows about one interval of a side.
#[derive(Debug, Clone, PartialEq)]
enum Ground {
    /// Not replayed yet.
    Unreached,
    /// Replayed, undecided, and still halvable.
    Undecided,
    /// Replayed and proven the witness's build.
    Proven,
    /// Replayed and proven not the witness's build.
    Different(Box<FlipEvidence>),
    /// Undecided for good: a terminal sliver, or an interval at the
    /// depth floor or the `f64` grid.
    Floor(RefusalReason),
}

/// One interval of a side: `near` is the end towards the current value.
#[derive(Debug, Clone, PartialEq)]
struct Cell {
    near: f64,
    far: f64,
    /// How many halvings of the side reached this interval.
    depth: u32,
    ground: Ground,
}

impl Cell {
    fn span(&self) -> (f64, f64) {
        (self.near.min(self.far), self.near.max(self.far))
    }

    fn width(&self) -> f64 {
        (self.far - self.near).abs()
    }

    /// The two halves, nearer first, or the bound that forbids halving.
    fn halves(&self, max_depth: u32) -> Result<(Self, Self), BudgetKind> {
        if self.depth >= max_depth {
            return Err(BudgetKind::Depth { max_depth });
        }
        let (lo, hi) = self.span();
        let mid = BoxAxis::Varying { lo, hi }.midpoint();
        if !(lo < mid && mid < hi) {
            return Err(BudgetKind::Resolution);
        }
        let half = |near, far| Self {
            near,
            far,
            depth: self.depth + 1,
            ground: Ground::Unreached,
        };
        Ok((half(self.near, mid), half(mid, self.far)))
    }
}

/// One side's answer, read off its intervals (nearest first, tiling the
/// side). `steps` prices the ground the search has not finished with.
fn side_answer(cells: &[Cell], steps: usize) -> RangeSide {
    let proven = cells
        .iter()
        .take_while(|c| c.ground == Ground::Proven)
        .count();
    let proven_to = proven.checked_sub(1).map_or(0.0, |last| cells[last].far);
    let Some(frontier) = cells.get(proven) else {
        return RangeSide {
            proven_to,
            end: Some(SideEnd::Seed),
            not_decided: None,
        };
    };
    let end = cells[proven..].iter().find_map(|c| match &c.ground {
        Ground::Different(evidence) => Some(SideEnd::Different(ProvenDifferent {
            at: c.near,
            evidence: evidence.clone(),
        })),
        _ => None,
    });
    let reason = match &frontier.ground {
        Ground::Different(_) | Ground::Proven => None,
        Ground::Floor(reason) => Some(reason.clone()),
        Ground::Unreached | Ground::Undecided => Some(RefusalReason::Budget(BudgetKind::Leaves {
            max_leaves: steps,
        })),
    };
    RangeSide {
        proven_to,
        end,
        not_decided: reason.map(|reason| NotDecided {
            past: proven_to,
            reason,
        }),
    }
}

/// **The resumable search** (module docs).
#[derive(Debug)]
pub struct RangeSearch {
    field: RangeField,
    seed: RangeSeed,
    derived: DerivedRange,
    root: ParamBox,
    config: DriveConfig,
    resolver: Option<Arc<dyn PartResolver>>,
    tol: Tol,
    witness: Evaluation<f64>,
    witness_vector: VerdictVector,
    witness_key: VerdictVectorKey,
    memo: Arc<geom_core::sym::DriveMemo>,
    /// The low side's intervals, then the high side's; each nearest
    /// first and tiling its side.
    sides: [Vec<Cell>; 2],
    steps: usize,
}

impl RangeSearch {
    /// Admits the request ([`admit`]) and evaluates the witness, once.
    /// No interval is replayed.
    ///
    /// `resolver` is how a document that instantiates parts reaches
    /// them, at the witness and at every replay.
    ///
    /// # Errors
    ///
    /// [`RangeRefusal`]: the request's own, before any geometry runs,
    /// then [`RangeRefusal::Drive`] for a witness that does not build
    /// or was stopped.
    pub fn new(
        doc: &Doc<ProfileProgram>,
        field: &RangeField,
        seed: RangeSeed,
        config: &DriveConfig,
        resolver: Option<Arc<dyn PartResolver>>,
        stop: &crate::Stop,
        tol: Tol,
    ) -> Result<Self, RangeRefusal> {
        let derived = derive(doc, field, seed, tol)?;
        let analyzed = analyzed_box(&derived.doc, &AnalysisPolicy::default());
        // The axis the analysis derived must BE the seed, and the only
        // one that varies. `Band` states offsets and `analyzed_box`
        // carries them verbatim, so this is an exact comparison.
        let asked = (seed.lo, seed.hi);
        let derived_axis = analyzed
            .get(derived.axis)
            .map_or((0.0, 0.0), |p| (p.offsets.lo, p.offsets.hi));
        if derived_axis != asked {
            return Err(RangeRefusal::SeedIsNotTheAnalyzedAxis {
                analyzed: derived_axis,
                asked,
            });
        }
        let varying = analyzed.varying().count();
        if varying != 1 {
            return Err(RangeRefusal::MoreThanOneAxisVaries { varying });
        }
        let witness = witness_build(&derived.doc, &analyzed, resolver.as_ref(), stop, tol)
            .map_err(|e| RangeRefusal::Drive(Box::new(e)))?;
        let witness_vector = certifying_vector(&derived.doc, &witness);
        let side = |far: f64| {
            if far == 0.0 {
                Vec::new()
            } else {
                vec![Cell {
                    near: 0.0,
                    far,
                    depth: 0,
                    ground: Ground::Unreached,
                }]
            }
        };
        Ok(Self {
            field: field.clone(),
            seed,
            root: ParamBox::of(&analyzed),
            config: config.clone(),
            resolver,
            tol,
            witness_key: witness_vector.key(),
            witness_vector,
            witness,
            memo: drive_memo(config),
            sides: [side(seed.lo), side(seed.hi)],
            steps: 0,
            derived,
        })
    }

    /// How many intervals have been replayed.
    #[must_use]
    pub fn steps(&self) -> usize {
        self.steps
    }

    /// The answer after the steps taken so far: a pure function of the
    /// inputs and [`Self::steps`].
    #[must_use]
    pub fn answer(&self) -> CertifiedRange {
        CertifiedRange {
            field: self.field.clone(),
            nominal: self.derived.nominal,
            seed: self.seed,
            pinned: self.derived.pinned.clone(),
            steps: self.steps,
            lo: side_answer(&self.sides[0], self.steps),
            hi: side_answer(&self.sides[1], self.steps),
        }
    }

    /// **One replay** (module docs, the schedule): the unreached
    /// interval nearest the current value, after halving the nearest
    /// undecided one when none is waiting.
    pub fn step(&mut self, stop: &crate::Stop) -> Step {
        if self.steps >= self.config.max_leaves {
            return Step::Finished;
        }
        let (side, at) = match self.nearest(|g| *g == Ground::Unreached) {
            Some(found) => found,
            None => {
                let Some((side, at)) = self.nearest(|g| *g == Ground::Undecided) else {
                    return Step::Finished;
                };
                // `Undecided` is only ever written for an interval that
                // halves (`Self::replay`), and neither bound moves.
                let Ok((near, far)) = self.sides[side][at].halves(self.config.max_depth) else {
                    unreachable!("an undecided interval was checked to halve when it was replayed")
                };
                self.sides[side].splice(at..=at, [near, far]);
                (side, at)
            }
        };
        match self.replay(&self.sides[side][at], stop) {
            Ok(ground) => {
                self.sides[side][at].ground = ground;
                self.steps += 1;
                Step::Advanced
            }
            Err(stopped) => Step::Stopped(stopped),
        }
    }

    /// The interval whose ground satisfies `wanted` nearest the current
    /// value: least `|near|`, then the wider, then the low side.
    fn nearest(&self, wanted: impl Fn(&Ground) -> bool) -> Option<(usize, usize)> {
        let mut best: Option<(usize, usize)> = None;
        for (side, cells) in self.sides.iter().enumerate() {
            // Within a side the first match is the nearest.
            let Some(at) = cells.iter().position(|c| wanted(&c.ground)) else {
                continue;
            };
            let c = &cells[at];
            let better = best.is_none_or(|(s, a)| {
                let b = &self.sides[s][a];
                c.near.abs() < b.near.abs()
                    || (c.near.abs() == b.near.abs() && c.width() > b.width())
            });
            if better {
                best = Some((side, at));
            }
        }
        best
    }

    /// Replays one interval under the E6 leaf protocol and reads what
    /// it decided.
    fn replay(&self, cell: &Cell, stop: &crate::Stop) -> Result<Ground, crate::Stopped> {
        let (lo, hi) = cell.span();
        let mut axes = self.root.axes().clone();
        axes.insert(self.derived.axis, BoxAxis::Varying { lo, hi });
        let box_ = ParamBox::from_axes_in(axes, self.root.order());
        let (verdict, _) = classify(
            &self.derived.doc,
            &box_,
            &self.witness,
            &self.witness_vector,
            self.witness_key,
            self.config.symbolic,
            &self.memo,
            self.resolver.as_ref(),
            stop,
            self.tol,
        )?;
        Ok(match verdict {
            LeafVerdict::Certified(_) => Ground::Proven,
            LeafVerdict::Refused(RefusalReason::FlipCrossing { flipped }) => {
                Ground::Different(flipped)
            }
            LeafVerdict::Refused(reason) => Ground::Floor(reason),
            LeafVerdict::Bisect => match cell.halves(self.config.max_depth) {
                Ok(_) => Ground::Undecided,
                Err(kind) => Ground::Floor(RefusalReason::Budget(kind)),
            },
        })
    }
}

// A search is stepped off the thread that made it.
const _: fn() = || {
    fn sendable<T: Send>() {}
    sendable::<RangeSearch>();
};

/// **The answer as a function of a side's intervals**, over lists no
/// fixture is needed to reach: a standing change in the evidence, a
/// proven interval beyond an undecided one, a floor at the frontier.
#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;
    use crate::resolve::{FlipSet, NodeVerdictDelta, RunStatus};
    use std::collections::BTreeMap;

    fn node(n: u64) -> RecipeNodeId {
        RecipeNodeId::new(0, n)
    }

    /// A flip evidence with the two node standings asked for.
    fn evidence(old: RunStatus, new: RunStatus) -> Box<FlipEvidence> {
        let mut nodes = BTreeMap::new();
        nodes.insert(
            node(7),
            NodeVerdictDelta {
                old_status: old,
                new_status: new,
                flip: None,
            },
        );
        Box::new(FlipEvidence {
            verdicts: FlipSet { nodes },
            structure: Vec::new(),
        })
    }

    fn cell(near: f64, far: f64, ground: Ground) -> Cell {
        Cell {
            near,
            far,
            depth: 0,
            ground,
        }
    }

    fn depth_floor() -> Ground {
        Ground::Floor(RefusalReason::Budget(BudgetKind::Depth { max_depth: 3 }))
    }

    /// **`kind()` is the evidence's standings**, both ways: a standing
    /// change names its node, and a divergence with equal standings is
    /// a build on another branch.
    #[test]
    fn the_kind_of_a_difference_is_read_off_the_evidences_standings() {
        for (old, new, expect) in [
            (
                RunStatus::Ok,
                RunStatus::Failed,
                DifferenceKind::StandingChange { node: node(7) },
            ),
            (
                RunStatus::Ok,
                RunStatus::Poisoned,
                DifferenceKind::StandingChange { node: node(7) },
            ),
            (
                RunStatus::Ok,
                RunStatus::Ok,
                DifferenceKind::BuildsDifferently,
            ),
            (
                RunStatus::Failed,
                RunStatus::Failed,
                DifferenceKind::BuildsDifferently,
            ),
        ] {
            let cells = [
                cell(0.0, 0.5, Ground::Proven),
                cell(0.5, 1.0, Ground::Different(evidence(old, new))),
            ];
            let side = side_answer(&cells, 2);
            let different = side.different().expect("a difference is proven");
            assert_eq!(different.kind(), expect, "{old:?} -> {new:?}");
            assert_eq!(different.at(), 0.5);
            assert_eq!(side.proven_to(), 0.5);
            assert!(
                side.not_decided().is_none(),
                "the proof runs up to the difference"
            );
        }
    }

    /// **A difference beyond undecided ground is still reported**, at
    /// the NEAREST such interval's near edge, with the ground between
    /// stated undecided under the frontier's own reason.
    #[test]
    fn a_difference_past_undecided_ground_is_reported_at_its_near_edge() {
        let cells = [
            cell(0.0, 1.0, Ground::Proven),
            cell(1.0, 1.5, depth_floor()),
            cell(1.5, 2.0, Ground::Undecided),
            cell(
                2.0,
                4.0,
                Ground::Different(evidence(RunStatus::Ok, RunStatus::Ok)),
            ),
            cell(
                4.0,
                8.0,
                Ground::Different(evidence(RunStatus::Ok, RunStatus::Failed)),
            ),
        ];
        let side = side_answer(&cells, 9);
        assert_eq!(side.proven_to(), 1.0);
        let different = side.different().expect("the far interval is a difference");
        assert_eq!(different.at(), 2.0);
        assert_eq!(different.kind(), DifferenceKind::BuildsDifferently);
        let gap = side.not_decided().expect("undecided ground lies between");
        assert_eq!(gap.past(), 1.0);
        assert_eq!(
            *gap.reason(),
            RefusalReason::Budget(BudgetKind::Depth { max_depth: 3 })
        );
        assert!(!gap.more_time_helps(), "the frontier is at its floor");
    }

    /// **A proven interval beyond an undecided one does not extend the
    /// proof**, and unfinished ground is priced under the steps taken.
    #[test]
    fn the_proof_is_contiguous_and_unfinished_ground_is_priced_as_budget() {
        for unfinished in [Ground::Unreached, Ground::Undecided] {
            let cells = [
                cell(0.0, -1.0, Ground::Proven),
                cell(-1.0, -2.0, unfinished),
                cell(-2.0, -3.0, Ground::Proven),
            ];
            let side = side_answer(&cells, 5);
            assert_eq!(side.proven_to(), -1.0);
            assert!(
                side.end().is_none(),
                "neither the seed's end nor a difference"
            );
            let gap = side.not_decided().expect("the frontier is undecided");
            assert_eq!(gap.past(), -1.0);
            assert_eq!(
                *gap.reason(),
                RefusalReason::Budget(BudgetKind::Leaves { max_leaves: 5 })
            );
            assert!(gap.more_time_helps());
        }
    }

    /// A side proven throughout reaches the seed's end; so does a side
    /// with no width.
    #[test]
    fn a_side_proven_throughout_reaches_the_seeds_end() {
        let cells = [
            cell(0.0, 1.0, Ground::Proven),
            cell(1.0, 2.0, Ground::Proven),
        ];
        let side = side_answer(&cells, 3);
        assert!(side.reached_seed_end());
        assert_eq!(side.proven_to(), 2.0);
        assert!(side.not_decided().is_none());
        let empty = side_answer(&[], 0);
        assert!(empty.reached_seed_end());
        assert_eq!(empty.proven_to(), 0.0);
    }

    /// Halving stops at the depth bound and at the `f64` grid, each
    /// under its own name.
    #[test]
    fn an_interval_halves_until_its_depth_or_the_grid() {
        let c = cell(0.0, -1.0, Ground::Undecided);
        let (near, far) = c.halves(1).expect("depth 0 halves under a bound of 1");
        assert_eq!(
            (near.near, near.far, far.near, far.far),
            (0.0, -0.5, -0.5, -1.0)
        );
        assert_eq!(near.depth, 1);
        assert_eq!(near.halves(1), Err(BudgetKind::Depth { max_depth: 1 }));
        let tiny = cell(1.0, 1.0 + f64::EPSILON, Ground::Undecided);
        assert_eq!(tiny.halves(64), Err(BudgetKind::Resolution));
    }
}
