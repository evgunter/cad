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
//! request and evaluates the witness once; [`RangeSearch::step`] does
//! ONE replay; [`RangeSearch::answer`] can be read after any step. The
//! answer is a pure function of the inputs and the number of steps
//! taken. No clock is read, and the one piece of state the replays
//! share, the drive's plain-form memo, is transparent: it hands back the
//! form a replay would have built (`geom_core::sym::DriveMemo`). So
//! stopping keeps what was proven, and a later step continues it.
//!
//! # The schedule
//!
//! Each side of the current value is a row of intervals tiling it,
//! nearest first; at the start, one per side, the side whole. A replay
//! leaves an interval PROVEN (it certifies), DIFFERENT (decided
//! definitely otherwise, with something named that flipped, on an
//! interval that does not hold the current value), or undecided; a flip
//! with no such ground is undecided for good ([`Unproven::FlipUnfounded`]); an undecided interval at the
//! depth bound or the `f64` grid, or one the driver refused for good,
//! is at its FLOOR. A side's answer is read off its FRONTIER, the first
//! interval not proven, and its nearest DIFFERENT interval; the
//! APPROACH is the interval just short of that difference.
//!
//! Nothing past the nearest difference can change the side's answer:
//! the proof cannot pass it, and a difference found there is not the
//! nearest. The search works only where the answer can still move:
//!
//! 1. An interval not yet replayed, short of the nearest difference, is
//!    replayed.
//! 2. The frontier is halved and both halves are replayed, the nearer
//!    first. The near half extends the proof. The far half is how a
//!    difference past undecided ground is seen.
//! 3. The approach is halved and both halves are replayed, which moves
//!    the difference nearer.
//! 4. Neither end is halved more than [`STALL_LEVELS`](crate::range::STALL_LEVELS) times past the
//!    deepest interval whose replay last moved it (the proof's end for
//!    the frontier, the difference for the approach). Halvings that
//!    decide nothing at four successive widths are in ground undecided
//!    at every width: a coincidence or failure zone, or the ε band at a
//!    boundary. That end is then settled ([`Unproven::Stalled`]):
//!    what it gives up lies inside an interval a sixteenth the width of
//!    the one that last moved it. The frontier of a side that has
//!    proven nothing yet has no such bound, because the first proof can
//!    sit many halvings down, and halves to [`RangeBudget::max_depth`].
//! 5. While a side has no difference proven and its frontier is
//!    settled (at its floor, or stalled), the undecided intervals past
//!    the frontier are halved, widest first, down to [`STALL_LEVELS`](crate::range::STALL_LEVELS)
//!    halvings of the side: a difference past undecided ground is looked
//!    for down to a sixteenth of the side's width. That is how a
//!    boundary the frontier's far halves only touch (one sitting on a
//!    halving's cut point) is still seen past.
//!
//! Any other undecided interval, between the frontier and the approach,
//! is left alone. It could move the answer only by holding a difference
//! nearer than the one proven, and halving it is the tiling of
//! undecided ground the rules above exist to stop. So the point proven
//! different is the nearest the search PROVED, not the nearest that
//! exists.
//!
//! Across the two sides the next work is the one on the WIDER interval,
//! then the one nearer the current value, then the low side's. The two
//! sides refine level by level together, and neither starves the
//! other. The search has [`Step::Finished`](crate::range::Step::Finished) when neither side has work
//! left, or the replay cap is spent.
//!
//! The far half is replayed and not skipped, because that is the only
//! way a difference is ever PROVEN: a certified interval and one proven
//! different cannot be neighbours (a margin is definite and of one sign
//! over the first, definite and of another over the second, and both
//! enclose the shared endpoint), so every boundary has undecided ground
//! on its near side and a schedule that only grew the proven interval
//! outward would stop at that ground without ever seeing past it.
//!
//! # What it costs
//!
//! The FIRST proof of width `w` on a side of width `W` costs about
//! `2·log2(W/w)` replays, two a halving down the frontier, where a
//! level-by-level subdivision of the seed costs `W/w` leaves. After it
//! the proof grows by about the width that certifies for every two
//! replays, so a field that certifies only near `ε` never finishes a
//! wide seed: the caller's stop or the replay cap ends it. An end that
//! stalls costs at most `2·STALL_LEVELS` replays past its last move,
//! and looking past a settled frontier (the schedule, 5) at most
//! `2^(STALL_LEVELS+1)` replays a side.
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
//! These are a measurement of the leaf protocol on this hardware, so
//! they move whenever the protocol or the symbolic tier does: re-take
//! them after either changes
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
//! [`RangeBudget`] is the search's own: `max_replays` replays in all,
//! and `max_depth` halvings of one side; its leaf dials are the
//! drive's. The depth bounds a side only until its first proof (the
//! schedule, 4), so it is sized to reach the width a field certifies
//! at: near `ε/4` on curved geometry, which a side of width `W` reaches
//! in about `log2(4W/ε)` halvings. [`RangeBudget::for_seed`] sizes it
//! for a seed. A deeper bound costs two replays a level on a side that
//! never proves anything, and nothing on a side that does. The `f64`
//! grid is no bound near the current value, where it is far finer than
//! any depth. Ground the search still had work on when it stopped is
//! [`Unproven::Unfinished`], under the replays taken.
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

use std::collections::BTreeSet;
use std::sync::Arc;

use crate::Said;
use crate::say;
use geom_core::Tol;

use crate::analysis::{AnalysisPolicy, BoxAxis, ParamBox, analyzed_box};
use crate::distribution::Distribution;
use crate::doc::{Doc, FreeVar};
use crate::drive::{
    BudgetKind, DriveConfig, DriveRefusal, FlipEvidence, LeafVerdict, RefusalReason, SymbolicDials,
    VerdictVector, VerdictVectorKey, certifying_vector, classify, does_not_build, drive_memo,
    witness_build,
};
use crate::edit::{DocEdit, EditError, apply};
use crate::eval::Evaluation;
use crate::ident::Mispaired;
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
    /// That `1` is in the kernel's units of the field's dimension,
    /// whatever the dimension: a length's unit, and one radian for an
    /// angle.
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
    reason: Unproven,
}

/// **Why a side is not decided** at its frontier ([`NotDecided::reason`]).
#[derive(Debug, Clone, PartialEq)]
pub enum Unproven {
    /// The schedule still had work on this side when the search
    /// stopped: the caller stopped stepping, or
    /// [`RangeBudget::max_replays`] was spent.
    Unfinished {
        /// The replays the search had taken: budget spent, not a cap.
        replays: usize,
    },
    /// The frontier was halved [`STALL_LEVELS`] times past the interval
    /// whose proof last moved it and nothing decided (module docs, the
    /// schedule): ground undecided at every width the search tried.
    Stalled {
        /// The width of the interval that last moved the proof's end.
        width: f64,
    },
    /// The driver refused the frontier interval for good: a terminal
    /// sliver, an interval at [`RangeBudget::max_depth`] or the `f64`
    /// grid ([`BudgetKind::Depth`], [`BudgetKind::Resolution`]), or a
    /// node that does not read the ranged variable
    /// ([`RefusalReason::ReadsNoVariable`]), which no budget moves: its
    /// recourse is the node's own.
    Floor(RefusalReason),
    /// The driver refused the frontier interval as a flip that has no
    /// ground: it names nothing that flipped (no verdict and no
    /// structure differs), or the interval holds the current value,
    /// whose build IS the witness's. Not a difference, and not ground
    /// more replays decide: a defect of the leaf protocol to name.
    FlipUnfounded,
}

impl NotDecided {
    /// The offset the undecided ground starts at: the proven frontier.
    #[must_use]
    pub fn past(&self) -> f64 {
        self.past
    }

    /// Why the side is undecided at its frontier.
    #[must_use]
    pub fn reason(&self) -> &Unproven {
        &self.reason
    }

    /// Whether more replays can change this side's answer: the schedule
    /// still had work on it ([`Unproven::Unfinished`]). A stall and a
    /// floor are where the schedule stops; a deeper
    /// [`RangeBudget::max_depth`] can move a depth floor, and more
    /// replays cannot.
    #[must_use]
    pub fn more_time_helps(&self) -> bool {
        matches!(self.reason, Unproven::Unfinished { .. })
    }
}

/// How a side ends, where the search can say.
#[derive(Debug, Clone, PartialEq)]
pub enum SideEnd {
    /// Everything from the current value to the seed's end is proven.
    /// Never a claim about values outside the seed, which were not
    /// replayed.
    Seed,
    /// The nearest point the search proved different (module docs, the
    /// schedule: not necessarily the nearest that exists).
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

    /// An offset read as an absolute field value, `nominal + offset`
    /// rounded to nearest at `f64`. A reader's convenience: the PROOF is
    /// over the offsets, which the replayed axis carries exactly, and
    /// this sum can land a rounding past the offset it reads. An END of
    /// the proof is read through [`Self::proven_interval`], which rounds
    /// inward.
    #[must_use]
    pub fn absolute(&self, offset: f64) -> f64 {
        self.nominal + offset
    }

    /// The proven interval as offsets, derived from the two sides:
    /// `(lo.proven_to(), hi.proven_to())`. `(0, 0)` when nothing is
    /// proven ([`Self::nothing_proven`]).
    ///
    /// **What "proven" means** is the E6 leaf protocol's certificate,
    /// with [`Self::pinned`] at their nominals: at every value in it the
    /// same nodes stand as at the witness and every decision the witness
    /// recorded goes the same way. The protocol replays the witness's
    /// REPRESENTATION choices as recorded rather than deciding them
    /// afresh (a `canonical_order_*` verdict, which picks where a loop
    /// starts), so a plain `f64` build at a proven value can record
    /// such a verdict differently from the witness, and build the same
    /// shape stated from another start
    /// (`work/certread/the-leaf-protocol-pins-representation-choices-the-witness-recorded`).
    #[must_use]
    pub fn proven_offsets(&self) -> (f64, f64) {
        (self.lo.proven_to, self.hi.proven_to)
    }

    /// The proven interval in absolute field values, each end
    /// [`Self::absolute`] of [`Self::proven_offsets`] stepped toward the
    /// nominal where the sum rounded outward, so neither end lies
    /// outside what was proven.
    #[must_use]
    pub fn proven_interval(&self) -> (f64, f64) {
        let (lo, hi) = self.proven_offsets();
        (self.inward(lo), self.inward(hi))
    }

    /// `nominal + offset`, rounded toward the nominal: the exact sum's
    /// rounding error (Knuth's two-sum) says which way it went.
    fn inward(&self, offset: f64) -> f64 {
        let sum = self.nominal + offset;
        let back = sum - self.nominal;
        let error = (self.nominal - (sum - back)) + (offset - back);
        if offset > 0.0 && error < 0.0 {
            sum.next_down()
        } else if offset < 0.0 && error > 0.0 {
            sum.next_up()
        } else {
            sum
        }
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
    /// The build handed to [`admit`] is an evaluation of another
    /// document (ASSEMBLY A2a, the pairing doors).
    EvaluationOfAnotherDocument(Mispaired),
    /// The witness does not build, or its build was stopped: the
    /// driver's refusal, verbatim.
    Drive(Box<DriveRefusal>),
}

impl crate::Say for RangeRefusal {
    fn say(&self, f: &mut crate::Sentence, by: crate::Speaker<'_>) -> core::fmt::Result {
        match self {
            Self::SeedNotABracket { lo, hi } => write!(
                f,
                "a seed brackets the field's current value: offsets [{lo}, {hi}] must be finite \
                 with lo <= 0 <= hi and some width"
            ),
            Self::NotAContinuousParam { param } => say!(
                f,
                by.var(param),
                " is not a continuous parameter of this document — there is no axis to \
                 certify over"
            ),
            Self::UnknownNode { node } => {
                say!(f, "this document has no ", by.mention(node))
            }
            Self::UnknownSlot { node, slot } => {
                say!(
                    f,
                    by.mention(node),
                    format_args!(" carries no {} slot", slot.label())
                )
            }
            Self::StructuralSlot { node, slot } => say!(
                f,
                format_args!("the {} slot of ", slot.label()),
                by.mention(node),
                " is structural (Count) — a structural slot selects \
                 between shapes and has no interval to certify over"
            ),
            Self::SlotIsDefined { node, slot } => say!(
                f,
                format_args!("the {} slot of ", slot.label()),
                by.mention(node),
                format_args!(
                    " reads a variable defined by a formula, which has no \
                 interval of its own. {}",
                    crate::sentence::Recourse("certify the variables that formula reads instead")
                )
            ),
            // Every derived edit re-writes a value the document already
            // holds, or declares one over a validated seed, so the edit
            // door's recourse — about an edit nobody made — is not this
            // door's.
            Self::Derivation(e) => say!(
                f,
                "the derived document was refused: ",
                Said(&e.problem(), by),
                format_args!(". {}", geom_core::KERNEL_DEFECT_ENDING)
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
            Self::EvaluationOfAnotherDocument(m) => write!(
                f,
                "the build handed to the range's admission is of document {}, not of the \
                 document {} asked about",
                m.found, m.expected
            ),
            Self::Drive(e) => say!(f, Said(&e, by)),
        }
    }
}

/// Its sentence, each node said by tag ([`crate::Speaker::TAG`]).
impl core::fmt::Display for RangeRefusal {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        crate::spoken::display_said(self, crate::Speaker::TAG, f)
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
/// a build of another document refuses
/// ([`RangeRefusal::EvaluationOfAnotherDocument`]), and a node that is
/// not usable there refuses as the witness build would
/// ([`DriveRefusal::WitnessDoesNotBuild`]), without building it again.
///
/// The pairing is by identity, as at every pairing door: an earlier
/// build of the same document, or one at another tolerance or through
/// another resolver, is admitted. It answers what the caller's own build
/// shows, which is what a consumer greys its control from, and the
/// search does not trust it: [`RangeSearch::new`] builds its own witness
/// and refuses on that.
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
    if let Some(m) = built.and_then(|built| crate::ident::mispaired(doc.id(), built.document)) {
        return Err(RangeRefusal::EvaluationOfAnotherDocument(m));
    }
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
    budget: &RangeBudget,
    resolver: Option<Arc<dyn PartResolver>>,
    stop: &crate::Stop,
    tol: Tol,
) -> Result<CertifiedRange, RangeRefusal> {
    let mut search = RangeSearch::new(doc, field, seed, budget, resolver, stop, tol)?;
    while search.step(stop) == Step::Advanced {}
    Ok(search.answer())
}

/// How many halvings past the interval that last moved an end of a
/// side's answer the search spends before it calls that end settled
/// (module docs, the schedule).
pub const STALL_LEVELS: u32 = 4;

/// How far below `ε` [`RangeBudget::for_seed`] sizes the depth to
/// reach: a curved field certifies near `ε/4` (module docs), and this
/// leaves two halvings below that.
const DEPTH_REACHES_BELOW_EPS: f64 = 16.0;

/// **The search's budget.**
///
/// Not a [`DriveConfig`]: a step is one replay, so the drive's leaf
/// count, its schedule switch and its depth bound per axis have no
/// meaning here. The leaf protocol's own dials are the drive's, and
/// mean what they mean there.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RangeBudget {
    /// How many intervals the search may replay in all.
    pub max_replays: usize,
    /// How many times one side may be halved. It bounds a side only
    /// until its first proof (module docs, the budget).
    pub max_depth: u32,
    /// The leaf protocol's symbolic tier ([`DriveConfig::symbolic`]).
    pub symbolic: SymbolicDials,
    /// Whether the replays share one plain-form memo
    /// ([`DriveConfig::plain_memo`]).
    pub plain_memo: bool,
}

impl RangeBudget {
    /// The two counts stated, the leaf dials the drive's defaults.
    #[must_use]
    pub fn new(max_depth: u32, max_replays: usize) -> Self {
        let leaf = DriveConfig::default();
        Self {
            max_replays,
            max_depth,
            symbolic: leaf.symbolic,
            plain_memo: leaf.plain_memo,
        }
    }

    /// **The depth a seed needs**: enough halvings that the wider side
    /// reaches `ε/16`, where a field on curved geometry has certified
    /// if it is going to (module docs, the budget), and `max_replays`
    /// as stated.
    #[must_use]
    pub fn for_seed(seed: RangeSeed, tol: Tol, max_replays: usize) -> Self {
        let side = seed.hi.max(-seed.lo);
        let levels = (side * DEPTH_REACHES_BELOW_EPS / tol.eps()).log2().ceil();
        // A seed narrower than the target needs no halving; a non-finite
        // one is refused at admission, and the clamp keeps the cast total.
        let max_depth = if levels.is_finite() && levels > 0.0 {
            levels.min(f64::from(u16::MAX)) as u32
        } else {
            0
        };
        Self::new(max_depth, max_replays)
    }
}

/// What one [`RangeSearch::step`] did.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[must_use]
pub enum Step {
    /// One interval was replayed, or halved and its near half
    /// replayed; there may be more to do.
    Advanced,
    /// Nothing was replayed: the schedule has no work left on either
    /// side, or [`RangeBudget::max_replays`] is spent.
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
    /// Replayed and undecided: the two halves it splits into, nearer
    /// first, cut when it was replayed.
    Undecided(Box<[Cell; 2]>),
    /// Replayed and proven the witness's build.
    Proven,
    /// Replayed and proven not the witness's build.
    Different(Box<FlipEvidence>),
    /// Undecided for good: a terminal sliver, or an interval at the
    /// depth bound or the `f64` grid.
    Floor(RefusalReason),
    /// Refused as a flip with no ground ([`Unproven::FlipUnfounded`]):
    /// undecided for good, and never a difference.
    Unfounded,
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
}

/// One piece of the schedule's work on a side.
#[derive(Debug, Clone, PartialEq)]
enum Work {
    /// Replay the interval at this index.
    Replay(usize),
    /// Put these halves in place of the interval at this index and
    /// replay the nearer.
    Halve(usize, Box<[Cell; 2]>),
}

impl Work {
    fn at(&self) -> usize {
        match self {
            Self::Replay(at) | Self::Halve(at, _) => *at,
        }
    }
}

/// One side of the search: its intervals, nearest first and tiling it,
/// and where each end of its answer last moved.
#[derive(Debug, Clone, PartialEq, Default)]
struct Side {
    cells: Vec<Cell>,
    /// The depth and width of the deepest interval whose replay moved
    /// the proof's end.
    proof_moved: Option<(u32, f64)>,
    /// The depth of the deepest interval whose replay moved the nearest
    /// difference.
    difference_moved: Option<u32>,
}

impl Side {
    fn whole(far: f64) -> Self {
        let cells = if far == 0.0 {
            Vec::new()
        } else {
            vec![Cell {
                near: 0.0,
                far,
                depth: 0,
                ground: Ground::Unreached,
            }]
        };
        Self {
            cells,
            ..Self::default()
        }
    }

    /// The first interval not proven: the frontier.
    fn frontier(&self) -> usize {
        self.cells
            .iter()
            .position(|c| c.ground != Ground::Proven)
            .unwrap_or(self.cells.len())
    }

    /// The nearest interval proven different, or the row's length.
    fn nearest_different(&self) -> usize {
        self.cells
            .iter()
            .position(|c| matches!(c.ground, Ground::Different(_)))
            .unwrap_or(self.cells.len())
    }

    fn proven_to(&self) -> f64 {
        self.frontier()
            .checked_sub(1)
            .map_or(0.0, |last| self.cells[last].far)
    }

    fn different_at(&self) -> Option<f64> {
        self.cells.get(self.nearest_different()).map(|c| c.near)
    }

    /// **The schedule's next work on this side** (module docs): an
    /// unreplayed interval short of the nearest difference, or a halving
    /// of the frontier or the approach while it has not stalled; the
    /// widest first, then the nearest. `None` when this side's answer
    /// cannot move.
    fn work(&self) -> Option<Work> {
        let frontier = self.frontier();
        let different = self.nearest_different();
        let approach = different
            .checked_sub(1)
            .filter(|_| different < self.cells.len());
        let open = |at: usize, depth: u32| {
            let proof = at == frontier
                && self
                    .proof_moved
                    .is_none_or(|(moved, _)| depth < moved + STALL_LEVELS);
            let difference = Some(at) == approach
                && self
                    .difference_moved
                    .is_some_and(|moved| depth < moved + STALL_LEVELS);
            proof || difference
        };
        // With no difference found and the frontier settled, undecided
        // ground past it is looked into down to `STALL_LEVELS` halvings
        // of the side.
        let scouting = different == self.cells.len()
            && self.cells.get(frontier).is_some_and(|c| match &c.ground {
                Ground::Undecided(_) => !open(frontier, c.depth),
                Ground::Floor(_) | Ground::Unfounded => true,
                Ground::Unreached | Ground::Proven | Ground::Different(_) => false,
            });
        let scout = |at: usize, depth: u32| scouting && at > frontier && depth < STALL_LEVELS;
        let mut best: Option<(Work, &Cell)> = None;
        for (at, cell) in self.cells.iter().enumerate().take(different).skip(frontier) {
            let work = match &cell.ground {
                Ground::Unreached => Work::Replay(at),
                Ground::Undecided(halves) if open(at, cell.depth) || scout(at, cell.depth) => {
                    Work::Halve(at, halves.clone())
                }
                _ => continue,
            };
            if best.as_ref().is_none_or(|(_, b)| before(cell, b)) {
                best = Some((work, cell));
            }
        }
        best.map(|(work, _)| work)
    }

    /// Writes a replay's ground and records which end of the answer it
    /// moved.
    fn settle(&mut self, at: usize, ground: Ground) {
        let (proven_to, different_at) = (self.proven_to(), self.different_at());
        self.cells[at].ground = ground;
        let (depth, width) = (self.cells[at].depth, self.cells[at].width());
        if self.proven_to() != proven_to && self.proof_moved.is_none_or(|(d, _)| depth >= d) {
            self.proof_moved = Some((depth, width));
        }
        if self.different_at() != different_at {
            self.difference_moved = Some(self.difference_moved.map_or(depth, |d| d.max(depth)));
        }
    }

    /// This side's answer. `replays` prices ground the schedule still
    /// has work on.
    fn answer(&self, replays: usize) -> RangeSide {
        let proven_to = self.proven_to();
        let Some(frontier) = self.cells.get(self.frontier()) else {
            return RangeSide {
                proven_to,
                end: Some(SideEnd::Seed),
                not_decided: None,
            };
        };
        let end = self
            .cells
            .get(self.nearest_different())
            .and_then(|c| match &c.ground {
                Ground::Different(evidence) => Some(SideEnd::Different(ProvenDifferent {
                    at: c.near,
                    evidence: evidence.clone(),
                })),
                _ => None,
            });
        let reason = if self.work().is_some() {
            Some(Unproven::Unfinished { replays })
        } else {
            match &frontier.ground {
                Ground::Different(_) | Ground::Proven => None,
                Ground::Floor(reason) => Some(Unproven::Floor(reason.clone())),
                Ground::Unfounded => Some(Unproven::FlipUnfounded),
                // An interval short of the nearest difference that is not
                // replayed is work, so an unreached frontier with none
                // left does not occur; it is priced as one that has.
                Ground::Unreached => Some(Unproven::Unfinished { replays }),
                Ground::Undecided(_) => Some(Unproven::Stalled {
                    width: self
                        .proof_moved
                        .map_or(frontier.width(), |(_, width)| width),
                }),
            }
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
}

/// **The order work is taken in**, across and within sides: the wider
/// interval first, so the two sides refine level by level together,
/// then the one nearer the current value. A full tie keeps the earlier
/// (the low side, across sides).
fn before(a: &Cell, b: &Cell) -> bool {
    a.width() > b.width() || (a.width() == b.width() && a.near.abs() < b.near.abs())
}

/// **The resumable search** (module docs).
#[derive(Debug)]
pub struct RangeSearch {
    field: RangeField,
    seed: RangeSeed,
    derived: DerivedRange,
    root: ParamBox,
    budget: RangeBudget,
    resolver: Option<Arc<dyn PartResolver>>,
    tol: Tol,
    witness: Evaluation<f64>,
    witness_vector: VerdictVector,
    witness_key: VerdictVectorKey,
    memo: Arc<geom_core::sym::DriveMemo>,
    /// Whether each node reads the varying axis ([`Doc::free_reads`]).
    reads_axis: BTreeSet<RecipeNodeId>,
    /// The low side, then the high side.
    sides: [Side; 2],
    steps: usize,
}

impl RangeSearch {
    /// Admits the request ([`admit`]) and evaluates the witness, once.
    /// No interval is replayed.
    ///
    /// `resolver` is how a document that instantiates parts reaches
    /// them, at the witness and at every replay. `stop` stops the
    /// witness build; each [`Self::step`] takes its own.
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
        budget: &RangeBudget,
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
        Ok(Self {
            field: field.clone(),
            seed,
            root: ParamBox::of(&analyzed),
            budget: *budget,
            resolver,
            tol,
            witness_key: witness_vector.key(),
            witness_vector,
            witness,
            memo: drive_memo(budget.plain_memo, budget.symbolic),
            reads_axis: derived
                .doc
                .free_reads()
                .into_iter()
                .filter(|(_, reads)| reads.contains(&derived.axis))
                .map(|(node, _)| node)
                .collect(),
            sides: [Side::whole(seed.lo), Side::whole(seed.hi)],
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
            lo: self.sides[0].answer(self.steps),
            hi: self.sides[1].answer(self.steps),
        }
    }

    /// **One replay** (module docs, the schedule). A stopped replay
    /// leaves the search as it stood, a halving included.
    pub fn step(&mut self, stop: &crate::Stop) -> Step {
        if self.steps >= self.budget.max_replays {
            return Step::Finished;
        }
        let Some((side, work)) = next(&self.sides) else {
            return Step::Finished;
        };
        let at = work.at();
        let cell = match &work {
            Work::Replay(_) => &self.sides[side].cells[at],
            Work::Halve(_, halves) => &halves[0],
        };
        match self.replay(cell, stop) {
            Ok(ground) => {
                if let Work::Halve(_, halves) = work {
                    self.sides[side].cells.splice(at..=at, *halves);
                }
                self.sides[side].settle(at, ground);
                self.steps += 1;
                Step::Advanced
            }
            Err(stopped) => Step::Stopped(stopped),
        }
    }

    /// Replays one interval under the E6 leaf protocol and reads what
    /// it decided.
    fn replay(&self, cell: &Cell, stop: &crate::Stop) -> Result<Ground, crate::Stopped> {
        let (lo, hi) = cell.span();
        let axis = BoxAxis::Varying { lo, hi };
        let mut axes = self.root.axes().clone();
        axes.insert(self.derived.axis, axis);
        let box_ = ParamBox::from_axes_in(axes, self.root.order());
        let (verdict, _) = classify(
            &self.derived.doc,
            &box_,
            &self.witness,
            &self.witness_vector,
            self.witness_key,
            self.budget.symbolic,
            &self.memo,
            self.resolver.as_ref(),
            stop,
            self.tol,
        )?;
        Ok(match verdict {
            LeafVerdict::Certified(_) => Ground::Proven,
            LeafVerdict::Refused(RefusalReason::FlipCrossing { flipped }) => flip(cell, flipped),
            LeafVerdict::Refused(reason) => Ground::Floor(reason),
            // A deciding node that reads nothing of the axis is not
            // settled by halving it, at any depth (as in `drive`).
            LeafVerdict::Bisect {
                deciding,
                unsettled,
            } if !self.reads_axis.contains(&deciding) => {
                Ground::Floor(RefusalReason::ReadsNoVariable {
                    node: deciding,
                    unsettled,
                })
            }
            LeafVerdict::Bisect { .. } => halves(cell, axis, self.budget.max_depth),
        })
    }
}

/// The schedule's next work, across both sides ([`before`]), with the
/// side it is on.
fn next(sides: &[Side; 2]) -> Option<(usize, Work)> {
    let mut best: Option<(usize, Work)> = None;
    for (side, row) in sides.iter().enumerate() {
        let Some(work) = row.work() else {
            continue;
        };
        let better = best
            .as_ref()
            .is_none_or(|(s, w)| before(&row.cells[work.at()], &sides[*s].cells[w.at()]));
        if better {
            best = Some((side, work));
        }
    }
    best
}

/// **What a refusal the driver calls a flip makes of an interval.** A
/// difference only where the flip has ground: evidence that names
/// something, on an interval that does not hold the current value. An
/// interval holding offset zero holds the witness's own value, so a
/// named flip there is a defect upstream: it fails loud in a debug build
/// and is undecided ground in any build, never a difference shown as
/// fact.
fn flip(cell: &Cell, flipped: Box<FlipEvidence>) -> Ground {
    let named = !flipped.verdicts.nodes.is_empty() || !flipped.structure.is_empty();
    debug_assert!(
        !named || cell.near != 0.0,
        "an interval holding the current value was decided different: {flipped:?}"
    );
    if named && cell.near != 0.0 {
        Ground::Different(flipped)
    } else {
        Ground::Unfounded
    }
}

/// What an undecided interval becomes: its halves, nearer first, or the
/// floor that forbids halving it.
fn halves(cell: &Cell, axis: BoxAxis, max_depth: u32) -> Ground {
    if cell.depth >= max_depth {
        return Ground::Floor(RefusalReason::Budget(BudgetKind::Depth { max_depth }));
    }
    let Some([(lo, mid), (_, hi)]) = axis.halves() else {
        return Ground::Floor(RefusalReason::Budget(BudgetKind::Resolution));
    };
    let half = |near, far| Cell {
        near,
        far,
        depth: cell.depth + 1,
        ground: Ground::Unreached,
    };
    // The low side's near end is its upper one.
    let pair = if cell.near <= cell.far {
        [half(lo, mid), half(mid, hi)]
    } else {
        [half(hi, mid), half(mid, lo)]
    };
    Ground::Undecided(Box::new(pair))
}

// A search is stepped off the thread that made it.
const _: fn() = || {
    fn sendable<T: Send>() {}
    sendable::<RangeSearch>();
};

/// **The answer and the schedule as functions of a side's intervals**,
/// over rows no fixture is needed to reach.
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

    fn at_depth(depth: u32, near: f64, far: f64, ground: Ground) -> Cell {
        Cell {
            depth,
            ..cell(near, far, ground)
        }
    }

    /// An undecided interval, with the halves it would split into.
    fn undecided(depth: u32, near: f64, far: f64) -> Cell {
        let c = at_depth(depth, near, far, Ground::Unreached);
        let (lo, hi) = c.span();
        let ground = halves(&c, BoxAxis::Varying { lo, hi }, 64);
        Cell { ground, ..c }
    }

    fn depth_floor() -> Ground {
        Ground::Floor(RefusalReason::Budget(BudgetKind::Depth { max_depth: 3 }))
    }

    fn row(cells: Vec<Cell>) -> Side {
        Side {
            cells,
            ..Side::default()
        }
    }

    fn different() -> Ground {
        Ground::Different(evidence(RunStatus::Ok, RunStatus::Ok))
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
            let side = row(vec![
                cell(0.0, 0.5, Ground::Proven),
                cell(0.5, 0.75, depth_floor()),
                cell(0.75, 1.0, Ground::Different(evidence(old, new))),
            ]);
            let different = side
                .answer(3)
                .different()
                .cloned()
                .expect("a difference is proven");
            assert_eq!(different.kind(), expect, "{old:?} -> {new:?}");
            assert_eq!(different.at(), 0.75);
        }
    }

    /// **A difference beyond undecided ground is reported**, at the
    /// NEAREST such interval's near edge, with the ground between stated
    /// undecided under the frontier's own reason once no work is left.
    #[test]
    fn a_difference_past_undecided_ground_is_reported_at_its_near_edge() {
        let mut side = row(vec![
            cell(0.0, 1.0, Ground::Proven),
            cell(1.0, 1.5, depth_floor()),
            at_depth(1, 1.5, 2.0, depth_floor()),
            cell(2.0, 4.0, different()),
            cell(
                4.0,
                8.0,
                Ground::Different(evidence(RunStatus::Ok, RunStatus::Failed)),
            ),
        ]);
        side.difference_moved = Some(0);
        let answer = side.answer(9);
        assert_eq!(answer.proven_to(), 1.0);
        let different = answer
            .different()
            .expect("the far interval is a difference");
        assert_eq!(different.at(), 2.0);
        assert_eq!(different.kind(), DifferenceKind::BuildsDifferently);
        let gap = answer.not_decided().expect("undecided ground lies between");
        assert_eq!(gap.past(), 1.0);
        assert_eq!(
            *gap.reason(),
            Unproven::Floor(RefusalReason::Budget(BudgetKind::Depth { max_depth: 3 }))
        );
        assert!(!gap.more_time_helps(), "nothing is left to do on this side");
    }

    /// **A floor at the frontier does not settle a side the schedule
    /// still has work on**: undecided ground short of the difference
    /// that can still move it prices as more time.
    #[test]
    fn a_floor_at_the_frontier_with_work_beyond_it_is_unfinished() {
        for beyond in [Ground::Unreached, undecided(1, 1.5, 2.0).ground] {
            let mut side = row(vec![
                cell(0.0, 1.0, Ground::Proven),
                cell(1.0, 1.5, depth_floor()),
                at_depth(1, 1.5, 2.0, beyond),
                cell(2.0, 4.0, different()),
            ]);
            side.difference_moved = Some(0);
            let gap = side
                .answer(7)
                .not_decided()
                .cloned()
                .expect("undecided ground");
            assert_eq!(gap.reason, Unproven::Unfinished { replays: 7 });
            assert!(gap.more_time_helps());
        }
    }

    /// **A proven interval beyond an undecided one does not extend the
    /// proof**, and ground the schedule still has work on is priced as
    /// the replays taken.
    #[test]
    fn the_proof_is_contiguous_and_unfinished_ground_is_priced_by_the_replays_taken() {
        for unfinished in [Ground::Unreached, undecided(0, -1.0, -2.0).ground] {
            let side = row(vec![
                cell(0.0, -1.0, Ground::Proven),
                cell(-1.0, -2.0, unfinished),
                cell(-2.0, -3.0, Ground::Proven),
            ]);
            let answer = side.answer(5);
            assert_eq!(answer.proven_to(), -1.0);
            assert!(
                answer.end().is_none(),
                "neither the seed's end nor a difference"
            );
            let gap = answer.not_decided().expect("the frontier is undecided");
            assert_eq!(gap.past(), -1.0);
            assert_eq!(*gap.reason(), Unproven::Unfinished { replays: 5 });
            assert!(gap.more_time_helps());
        }
    }

    /// A side proven throughout reaches the seed's end; so does a side
    /// with no width.
    #[test]
    fn a_side_proven_throughout_reaches_the_seeds_end() {
        let side = row(vec![
            cell(0.0, 1.0, Ground::Proven),
            cell(1.0, 2.0, Ground::Proven),
        ])
        .answer(3);
        assert!(side.reached_seed_end());
        assert_eq!(side.proven_to(), 2.0);
        assert!(side.not_decided().is_none());
        let empty = Side::whole(0.0).answer(0);
        assert!(empty.reached_seed_end());
        assert_eq!(empty.proven_to(), 0.0);
    }

    /// Halving stops at the depth bound and at the `f64` grid, each
    /// under its own name, and puts the nearer half first on each side.
    #[test]
    fn an_interval_halves_nearer_first_until_its_depth_or_the_grid() {
        let low = undecided(0, 0.0, -1.0);
        let Ground::Undecided(pair) = &low.ground else {
            panic!("depth 0 halves: {:?}", low.ground)
        };
        let [near, far] = &**pair;
        assert_eq!(
            (near.near, near.far, far.near, far.far),
            (0.0, -0.5, -0.5, -1.0)
        );
        assert_eq!(near.depth, 1);
        let axis = BoxAxis::Varying { lo: -0.5, hi: 0.0 };
        assert_eq!(
            halves(near, axis, 1),
            Ground::Floor(RefusalReason::Budget(BudgetKind::Depth { max_depth: 1 }))
        );
        let next = 1.0 + f64::EPSILON;
        let tiny = cell(1.0, next, Ground::Unreached);
        assert_eq!(
            halves(&tiny, BoxAxis::Varying { lo: 1.0, hi: next }, 64),
            Ground::Floor(RefusalReason::Budget(BudgetKind::Resolution))
        );
    }

    /// **Nothing past the nearest difference is worked on**, and an
    /// undecided interval between the frontier and the approach is left
    /// alone: neither can move the answer's ends.
    #[test]
    fn the_schedule_works_only_where_the_answer_can_move() {
        let mut side = row(vec![
            cell(0.0, 1.0, Ground::Proven),
            at_depth(2, 1.0, 1.25, depth_floor()),
            undecided(2, 1.25, 1.5),
            undecided(1, 1.5, 2.0),
            cell(2.0, 4.0, different()),
            cell(4.0, 6.0, Ground::Unreached),
            undecided(0, 6.0, 8.0),
        ]);
        side.difference_moved = Some(0);
        assert_eq!(
            side.work().map(|w| w.at()),
            Some(3),
            "only the approach moves"
        );
        side.cells[3].ground = depth_floor();
        assert_eq!(
            side.work(),
            None,
            "the ground at 1.25 cannot move either end"
        );
    }

    /// **An end stalls [`STALL_LEVELS`] halvings past the interval that
    /// last moved it**, and a frontier with no proof behind it halves to
    /// the depth bound.
    #[test]
    fn an_end_stalls_four_halvings_past_its_last_move() {
        let deep = STALL_LEVELS + 2;
        let mut side = row(vec![
            cell(0.0, 0.5, Ground::Proven),
            undecided(deep, 0.5, 0.75),
        ]);
        assert!(side.work().is_some(), "no proof yet bounds the frontier");
        side.proof_moved = Some((2, 0.5));
        assert_eq!(
            side.work(),
            None,
            "halving at depth {deep} is past the stall"
        );
        let answer = side.answer(11);
        assert_eq!(
            answer.not_decided().map(NotDecided::reason),
            Some(&Unproven::Stalled { width: 0.5 })
        );
        side.proof_moved = Some((3, 0.5));
        assert!(
            side.work().is_some(),
            "{deep} is short of 3 + {STALL_LEVELS}"
        );

        let mut approach = row(vec![
            cell(0.0, 0.5, depth_floor()),
            undecided(deep, 0.5, 0.75),
            cell(0.75, 1.0, different()),
        ]);
        approach.difference_moved = Some(2);
        assert_eq!(approach.work(), None, "both ends have stalled");
        approach.difference_moved = Some(3);
        assert_eq!(
            approach.work().map(|w| w.at()),
            Some(1),
            "the approach moves"
        );
    }

    /// **Past a settled frontier with no difference proven, undecided
    /// ground is looked into**, down to [`STALL_LEVELS`] halvings of the
    /// side and no further; not while the frontier still has work, and
    /// not once a difference is proven.
    #[test]
    fn a_settled_frontier_with_no_difference_looks_past_itself() {
        let mut side = row(vec![
            cell(0.0, 0.5, Ground::Proven),
            at_depth(9, 0.5, 0.5 + 2f64.powi(-10), depth_floor()),
            undecided(STALL_LEVELS - 1, 0.5 + 2f64.powi(-10), 0.75),
            undecided(1, 0.75, 1.0),
        ]);
        side.proof_moved = Some((9, 0.5));
        assert_eq!(
            side.work().map(|w| w.at()),
            Some(3),
            "the widest past the frontier"
        );
        side.cells[3] = undecided(STALL_LEVELS, 0.75, 1.0);
        assert_eq!(
            side.work().map(|w| w.at()),
            Some(2),
            "{STALL_LEVELS} halvings down is the last"
        );
        side.cells[2] = undecided(STALL_LEVELS, 0.5 + 2f64.powi(-10), 0.75);
        assert_eq!(side.work(), None, "and no deeper");
        let mut open = side.clone();
        open.cells[1] = undecided(9, 0.5, 0.5 + 2f64.powi(-10));
        open.cells[2] = undecided(1, 0.5 + 2f64.powi(-10), 0.75);
        assert_eq!(
            open.work().map(|w| w.at()),
            Some(1),
            "the frontier has work, so it goes alone"
        );
        side.cells[2] = undecided(1, 0.5 + 2f64.powi(-10), 0.75);
        side.cells.push(cell(1.0, 2.0, different()));
        side.difference_moved = Some(1);
        assert_eq!(
            side.work().map(|w| w.at()),
            Some(3),
            "a difference is proven: only its approach"
        );
    }

    /// **A flip that names nothing is not a difference**: the side
    /// reads it as undecided ground under its own reason, which more
    /// replays do not move, and proves nothing past it.
    #[test]
    fn a_flip_that_names_nothing_is_undecided_and_never_a_difference() {
        let empty = Box::new(FlipEvidence {
            verdicts: FlipSet {
                nodes: BTreeMap::new(),
            },
            structure: Vec::new(),
        });
        for c in [
            cell(0.0, 0.5, Ground::Unreached),
            cell(0.5, 1.0, Ground::Unreached),
        ] {
            assert_eq!(flip(&c, empty.clone()), Ground::Unfounded, "at {}", c.near);
        }
        let named = evidence(RunStatus::Ok, RunStatus::Ok);
        assert!(matches!(
            flip(&cell(0.5, 1.0, Ground::Unreached), named),
            Ground::Different(_)
        ));
        let side = row(vec![cell(0.0, 0.5, Ground::Unfounded)]).answer(4);
        assert!(
            side.end().is_none(),
            "no difference, no seed's end: {side:?}"
        );
        assert_eq!(side.proven_to(), 0.0);
        let gap = side.not_decided().expect("undecided");
        assert_eq!(*gap.reason(), Unproven::FlipUnfounded);
        assert!(!gap.more_time_helps());
    }

    /// **A named flip on an interval holding the current value fails
    /// loud**: that interval holds the witness's own value, so the flip
    /// is a defect upstream, never an answer.
    #[test]
    #[cfg(debug_assertions)]
    #[should_panic(expected = "an interval holding the current value was decided different")]
    fn a_named_flip_at_the_current_value_fails_loud() {
        let _ = flip(
            &cell(0.0, 0.5, Ground::Unreached),
            evidence(RunStatus::Ok, RunStatus::Failed),
        );
    }

    /// **The order across sides**: the wider interval first, then the
    /// nearer, then the low side.
    #[test]
    fn the_wider_interval_goes_first_then_the_nearer_then_the_low_side() {
        let wide_far = row(vec![
            cell(0.0, -0.5, Ground::Proven),
            undecided(1, -0.5, -1.0),
        ]);
        let narrow_near = row(vec![undecided(2, 0.0, 0.25)]);
        let side = |sides: [Side; 2]| next(&sides).map(|(side, _)| side);
        assert_eq!(
            side([wide_far.clone(), narrow_near.clone()]),
            Some(0),
            "the wider"
        );
        assert_eq!(
            side([narrow_near.clone(), wide_far.clone()]),
            Some(1),
            "the wider"
        );
        let near = row(vec![undecided(1, 0.0, 0.5)]);
        let far = row(vec![
            cell(0.0, -0.5, Ground::Proven),
            undecided(1, -0.5, -1.0),
        ]);
        assert_eq!(
            side([far.clone(), near.clone()]),
            Some(1),
            "the nearer, at one width"
        );
        assert_eq!(
            side([near.clone(), far]),
            Some(0),
            "the nearer, at one width"
        );
        let mirror = row(vec![undecided(1, 0.0, -0.5)]);
        assert_eq!(side([mirror, near]), Some(0), "a full tie goes low");
    }

    /// **The proven interval's absolute ends round toward the nominal**,
    /// so neither lies outside the offsets proven: `1 + 0.1` rounds up at
    /// `f64`, past the proven end, and is stepped back.
    #[test]
    fn the_absolute_ends_round_inward() {
        let side = |to: f64| RangeSide {
            proven_to: to,
            end: Some(SideEnd::Seed),
            not_decided: None,
        };
        let range = |nominal: f64, lo: f64, hi: f64| CertifiedRange {
            field: RangeField::Param(VarId::new(1, 0)),
            nominal,
            seed: RangeSeed { lo, hi },
            pinned: Vec::new(),
            steps: 2,
            lo: side(lo),
            hi: side(hi),
        };
        assert!(
            exact_offset(1.0 + 0.1, 1.0) > 0.1,
            "the nearest sum lies past the proven end"
        );
        let (_, hi) = range(1.0, -0.1, 0.1).proven_interval();
        assert_eq!(hi, (1.0f64 + 0.1).next_down());
        for (nominal, offset) in [
            (1.0f64, 0.1f64),
            (3.0, -0.7),
            (1e6, 1e-7),
            (-2.5, 0.3),
            (0.1, -0.2),
        ] {
            let r = range(nominal, offset.min(0.0), offset.max(0.0));
            let (lo, hi) = r.proven_interval();
            for end in [lo, hi] {
                let off = exact_offset(end, nominal);
                assert!(
                    r.seed.lo <= off && off <= r.seed.hi,
                    "{nominal} + {offset}: the end {end} lies {off} from the nominal"
                );
            }
        }
    }

    /// `value − nominal` exactly, where it is representable (the cases
    /// above are chosen so).
    fn exact_offset(value: f64, nominal: f64) -> f64 {
        let d = value - nominal;
        assert_eq!(d + nominal, value, "the difference is exact");
        d
    }
}
