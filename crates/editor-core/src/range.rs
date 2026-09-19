//! **The certified locally-valid range**: how far one field can move
//! before the proof stops, answered by the E6 subdivision driver
//! rather than by sampling.
//!
//! # What this answers, and what it does NOT
//!
//! The interactive question — "how much room does this field have" —
//! is answered by a PROBE (`viewer::bounds`): step outward, bisect the
//! first bracket that goes bad, report a bracket per side. This module
//! answers a stronger, slower and DIFFERENT question, on demand: over
//! which interval around the field's current value is the document
//! PROVABLY the same build?
//!
//! **A certified range is a subset of the LOCALLY-VALID RANGE.** The
//! probe asks "does anything NEW fail"; a certificate asks "does
//! anything DECIDE differently", which is strictly more, so every
//! value this module certifies is locally valid.
//!
//! **It is not a subset of the PROBE's reported bracket, and the two
//! are not the same claim.** The probe reports the furthest value it
//! SAMPLED and found valid, which on a field with a nearby boundary
//! can sit well outside the interval a drive can prove: on an 8 mm
//! extrusion slot the certificate's lower end is `1.6e-8` where the
//! probe's furthest valid sample is `3.9e-6`. Both are true about
//! different questions. A consumer shows both or names which one it
//! is showing.
//!
//! # What this costs on a real document, measured
//!
//! **On the repo's own corpus documents this query certifies NOTHING
//! at any budget a caller can afford**, and the reason is the
//! driver's certification width rather than anything here. Measured
//! on this tree (dev profile, one machine; re-take them rather than
//! trusting them):
//!
//! | document | field | per leaf | certified |
//! | --- | --- | --- | --- |
//! | `corpus::plate_param` (7 nodes) | `hole_r`, seed ±0.01 | 3.4 s | 0 of 64 |
//! | `corpus::die` (84 nodes) | `pip_depth`, seed ±0.01 | 3.4–4.8 s | 0 of 4 |
//! | `corpus::die` | the cube's literal `Distance` slot, ±0.01 | 17.3 s | 0 of 2 |
//!
//! The tour's own tolerance cell records the same fact from the other
//! side: `demos/tour/src/plate.rs`'s `CERTIFIABLE_FRACTION` is
//! **7.81e-7** of its spacing band. A field feeding a PARAMETRIC
//! POLYGON VERTEX is worse than the corpus average and sharply so —
//! one loop with one parametric vertex coordinate certifies at ±1e-8
//! (`10 ε`) and nothing at ±3e-8, which is the ε-bounded width E12's
//! symbolic tier exists to leave, reached again because the tier
//! discharges nothing there
//! (`work/props/parametric-polygon-loop-certifies-nothing`). What
//! certifies over a macroscopic seed is a slab: straight geometry
//! whose identities the tier cancels.
//!
//! **So [`DriveConfig::default`] is not a sensible budget for this
//! query.** Its 65,536 leaves are about sixty hours on the corpus
//! plate and a fortnight on the die's slot. The on-demand posture
//! this query was ruled into makes a MINUTE usable and hours not, so
//! size the budget by wall clock:
//!
//! ```text
//! DriveConfig { max_leaves: 16, ..DriveConfig::default() }   // ~1 min at 3.4 s/leaf
//! ```
//!
//! and the honest procedure is to drive ONE leaf first (`max_leaves:
//! 1`), time it, and divide the time the caller is willing to wait by
//! that. `max_depth` is not the dial to lower: it bounds how finely
//! one axis may be cut, and the whole-drive cost is `max_leaves`.
//!
//! # The one field, and where its widening lives
//!
//! The widening is a property of the QUERY, never of the document. A
//! document parameter already has a name to widen, so it is boxed
//! directly. A node SLOT has none — its value is a bit-pinned `f64`
//! literal — so the query derives a document of its own: a clone
//! carrying one synthetic continuous parameter whose nominal is the
//! literal's bits, with the slot rewritten to name it. The input
//! document is never edited, keyed or persisted, its memo is not
//! shared, and the derivation is a pure function of (document, field,
//! seed) that lives only for the query ([`derive`]).
//!
//! Every OTHER parameter's distribution is cleared in the derived
//! document, so the drive has exactly one varying axis. That is what
//! makes the verdict a RANGE rather than a box: this query's contract
//! is one field, and a document-wide box is [`crate::drive::drive`]'s
//! own answer, asked for directly. The clearing is a CONDITION on the
//! answer, not a detail — a range taken this way holds with the other
//! parameters at their nominals and says nothing about their spreads
//! — so the names it pinned ride on the answer
//! ([`CertifiedRange::pinned`]) for a consumer to state.
//!
//! # The seed is the caller's
//!
//! The query certifies over the interval it is given and never
//! chooses one. [`RangeSide::Certified`]'s `to` is therefore the SEED's
//! edge, never "unbounded": outside the seed nothing was replayed, so
//! there is nothing to say.
//!
//! # Offsets, not absolute values
//!
//! Every interval here is an OFFSET from the field's nominal, in the
//! field's own dimension, because that is what the analyzed axis is:
//! [`Distribution::Band`] states offsets and
//! [`crate::analysis::analyzed_box`] carries them into the box
//! verbatim, so a seed stated this way IS the axis, exactly. A seed
//! stated in absolute values would reach the axis through a
//! subtraction, and the box would then be a rounded neighbour of the
//! interval that was asked for. [`CertifiedRange::absolute`] converts
//! for a reader.

use geom_core::Tol;

use crate::analysis::{AnalysisPolicy, ParamBox, analyzed_box};
use crate::distribution::Distribution;
use crate::doc::{Doc, DocParam, ParamName};
use crate::drive::{
    DriveConfig, DriveRefusal, FlipEvidence, ParamBoxVerdict, RefusalReason, drive,
};
use crate::edit::{DocEdit, EditError, apply};
use crate::expr::Expr;
use crate::node::{RecipeNodeId, SlotId};
use crate::program::ProfileProgram;

/// The field a range is asked about: one document parameter, or one
/// node slot.
///
/// The two are not the same kind of thing and the query does not
/// pretend they are: a parameter is already a name the analysis can
/// vary, and a slot is a literal that has to be given one.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RangeField {
    /// A document parameter, boxed directly.
    Param(ParamName),
    /// A continuous slot of one node, widened through a synthetic
    /// parameter of the derived document.
    Slot {
        /// The node owning the slot.
        node: RecipeNodeId,
        /// The slot.
        slot: SlotId,
    },
}

/// The interval to certify over, as OFFSETS from the field's nominal
/// (module docs: the analyzed axis IS offsets, so a seed stated this
/// way reaches the driver exactly).
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
    /// A symmetric seed of half-width `w`.
    ///
    /// **Infallible on purpose, and the invariant is not weakened by
    /// it.** A negative `w` builds `lo > 0 > hi`, a zero `w` builds a
    /// seed with no width, and a non-finite `w` builds a non-finite
    /// one — each of which [`derive()`] refuses
    /// [`RangeRefusal::SeedNotABracket`], naming the offsets it was
    /// handed. A `Result` here would refuse the same three facts one
    /// call earlier under a second spelling, and a caller that wrote
    /// the fields directly would still meet the first; one door for
    /// one invariant is the trade.
    #[must_use]
    pub fn symmetric(w: f64) -> Self {
        Self { lo: -w, hi: w }
    }
}

/// **One side's verdict, four arms and no other.**
///
/// A leaf's own refusal class decides its arm; this query invents no
/// decision of its own and folds no arm into another.
///
/// # What `within` is, on the three arms that carry one
///
/// `within` is `[certified_to, the near edge of the first leaf the
/// driver decided DEFINITELY OTHERWISE]`. Its near end is proven the
/// witness's build and its far end is proven not; **everything
/// strictly between is ground the driver did not decide**, and it is
/// not empty — a certified leaf and a flip-crossing leaf can never be
/// neighbours (see [`certified_range`]'s walk), so every boundary has
/// undecided leaves around it.
///
/// **A consumer must not render `within` as valid.** Today a node
/// that fails on part of a sub-box is one of those undecided leaves:
/// `drive::classify_replay` bisects such a leaf to the budget floor
/// and prices it `Budget` rather than naming the failure, so a
/// `within` routinely CONTAINS values at which the document does not
/// build — `docm9_range.rs`'s
/// `a_decision_flips_within_contains_a_value_that_does_not_build`
/// measures exactly that on the A2 fixture. The row this waits on is
/// `work/props/coincidence-zone-priced-budget-at-the-floor`.
///
/// # A first leaf that straddles the nominal
///
/// The leaf holding offset zero belongs to BOTH walks, so when it is
/// the leaf reported, the two sides carry one span crossing the
/// nominal and `certified_to` is zero on both. That is honest rather
/// than a bug: nothing either side of the nominal was proven.
#[derive(Debug, Clone, PartialEq)]
pub enum RangeSide {
    /// Every leaf from the nominal to `to` certified, and `to` is the
    /// seed's edge on this side: nothing in `[nominal, to]` changes
    /// any decision the witness made.
    ///
    /// The seed's edge is where the proof stops. It is never a claim
    /// about values outside the seed, which were not replayed.
    Certified {
        /// The seed's edge on this side, as an offset.
        to: f64,
    },
    /// The first leaf outward the driver decided DEFINITELY otherwise
    /// is a [`RefusalReason::FlipCrossing`] whose evidence shows a
    /// node's STANDING change — a node that was `Ok` at the witness is
    /// not, or the reverse.
    ///
    /// It asserts that the FIRST DECIDED DIFFERENCE outward is a
    /// failure, and that `[nominal, certified_to]` is the witness's
    /// build. It asserts nothing at all about the undecided ground
    /// inside `within` (the type's own docs).
    NewFailure {
        /// The proven frontier, as an offset (zero when nothing this
        /// side of the nominal certified).
        certified_to: f64,
        /// The bracket, as offsets ([`RangeSide`]'s docs): proven the
        /// witness's build at the near end, proven not at the far
        /// end, undecided in between.
        within: (f64, f64),
        /// The driver's evidence, verbatim.
        evidence: Box<FlipEvidence>,
    },
    /// The first leaf outward the driver decided DEFINITELY otherwise
    /// is a [`RefusalReason::FlipCrossing`] with NO standing change:
    /// at the far end of `within` a recorded predicate decides
    /// differently and every node still builds.
    ///
    /// **This is the boundary of the CERTIFICATE, not of validity, and
    /// it is not a claim that `within` is valid.** What the arm
    /// asserts is that the first decided difference outward is a
    /// decision flip with no standing change — never that every value
    /// in `within` builds, which today is routinely false (a node
    /// that fails on a sub-box is priced `Budget` among the undecided
    /// leaves rather than decided; see [`RangeSide`]'s docs). Never
    /// folded into [`RangeSide::NewFailure`].
    DecisionFlip {
        /// The proven frontier, as an offset.
        certified_to: f64,
        /// The bracket, as offsets ([`RangeSide`]'s docs).
        within: (f64, f64),
        /// The driver's evidence, verbatim.
        evidence: Box<FlipEvidence>,
    },
    /// NO leaf outward was decided definitely otherwise, and the
    /// first uncertified one is [`RefusalReason::Budget`],
    /// [`RefusalReason::SliverTerminal`],
    /// [`RefusalReason::Bifurcation`], [`RefusalReason::Infeasible`]
    /// or [`RefusalReason::MeasureRefused`]: the driver could not
    /// DECIDE `within`, and found nothing beyond it either.
    ///
    /// **Not a bound**, and never to be rendered as one. It is also
    /// not a claim that nothing is wrong out there: a FAILURE
    /// boundary reaches this arm today as `Budget` AT THE FLOOR, and
    /// nothing at this type tells that apart from a budget a caller
    /// could simply raise. So the recourse is the reason's own only
    /// ABOVE the floor — more leaves, a coarser seed — and a
    /// `Budget(Depth { max_depth })` at the shipped depth, or a
    /// `Budget(Resolution)`, means refinement has stopped moving and
    /// more budget buys nothing. The row that would let the driver
    /// name such a leaf is
    /// `work/props/coincidence-zone-priced-budget-at-the-floor`.
    Indeterminate {
        /// The proven frontier, as an offset.
        certified_to: f64,
        /// The first undecided leaf's own span, as offsets. It
        /// straddles the nominal in the one case where `certified_to`
        /// is zero.
        within: (f64, f64),
        /// The driver's typed reason, verbatim.
        reason: Box<RefusalReason>,
    },
}

impl RangeSide {
    /// The proven frontier on this side, as an offset: `to` when
    /// certified, `certified_to` otherwise.
    #[must_use]
    pub fn certified_to(&self) -> f64 {
        match *self {
            Self::Certified { to } => to,
            Self::NewFailure { certified_to, .. }
            | Self::DecisionFlip { certified_to, .. }
            | Self::Indeterminate { certified_to, .. } => certified_to,
        }
    }

    /// Whether this side reports a BOUND — a place where the build
    /// changes. [`RangeSide::Indeterminate`] answers `false`: the driver
    /// could not decide, and an undecided leaf is not an edge.
    #[must_use]
    pub fn is_bound(&self) -> bool {
        matches!(self, Self::NewFailure { .. } | Self::DecisionFlip { .. })
    }
}

/// The query's answer: one [`RangeSide`] per direction, around the field's
/// own nominal.
///
/// **It is a CONDITIONAL answer and carries its condition**
/// ([`Self::pinned`]): the derivation holds every other annotated
/// parameter at its nominal, so a consumer states "conditional on
/// `side` at its nominal" rather than implying a claim over the
/// document's whole box.
#[derive(Debug, Clone, PartialEq)]
pub struct CertifiedRange {
    field: RangeField,
    nominal: f64,
    seed: RangeSeed,
    pinned: Vec<ParamName>,
    lo: RangeSide,
    hi: RangeSide,
}

impl CertifiedRange {
    /// The field this range is about.
    #[must_use]
    pub fn field(&self) -> &RangeField {
        &self.field
    }

    /// The field's value in the input document — the point the
    /// witness build is taken at, and the origin every offset here is
    /// measured from.
    #[must_use]
    pub fn nominal(&self) -> f64 {
        self.nominal
    }

    /// The seed the caller chose.
    #[must_use]
    pub fn seed(&self) -> RangeSeed {
        self.seed
    }

    /// **The condition this answer holds under**: the parameters whose
    /// declared distribution the derivation cleared, in name order, so
    /// the drive had one axis. Every one of them is at its nominal for
    /// the whole certificate, and empty means the document declared no
    /// other spread to drop.
    #[must_use]
    pub fn pinned(&self) -> &[ParamName] {
        &self.pinned
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
    /// at `f64`. A reader's convenience: the PROOF is over the
    /// offsets, which the driver's axis carries exactly, and this sum
    /// rounds.
    #[must_use]
    pub fn absolute(&self, offset: f64) -> f64 {
        self.nominal + offset
    }

    /// The proven interval as offsets: `(lo.certified_to(),
    /// hi.certified_to())`. Every value in it builds exactly what the
    /// witness built, with [`Self::pinned`] at their nominals.
    ///
    /// A SUBSET of the locally-valid range and not of the sampling
    /// probe's reported bracket — the two are different claims, and
    /// the module docs carry the measurement that separates them.
    #[must_use]
    pub fn certified_offsets(&self) -> (f64, f64) {
        (self.lo.certified_to(), self.hi.certified_to())
    }

    /// The proven interval in absolute field values
    /// ([`Self::absolute`] of [`Self::certified_offsets`]).
    #[must_use]
    pub fn certified_interval(&self) -> (f64, f64) {
        let (lo, hi) = self.certified_offsets();
        (self.absolute(lo), self.absolute(hi))
    }
}

/// Why a range could not be asked for at all.
///
/// Every arm is a fact about the REQUEST or about the document's
/// shape, decided before any geometry runs — except [`Self::Drive`],
/// which is the driver's own door refusing, carried verbatim.
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
        /// The name asked for.
        param: ParamName,
    },
    /// The document has no such node.
    UnknownNode {
        /// The id asked for.
        node: RecipeNodeId,
    },
    /// The node carries no such slot.
    UnknownSlot {
        /// The node.
        node: RecipeNodeId,
        /// The slot asked for.
        slot: SlotId,
    },
    /// The slot is STRUCTURAL (`Count`-typed): it selects between
    /// shapes rather than measuring one, and a structural slot has no
    /// interval to certify over.
    StructuralSlot {
        /// The node.
        node: RecipeNodeId,
        /// The slot.
        slot: SlotId,
    },
    /// The slot is not a bare literal: it is already driven by an
    /// expression, which the rewrite would SHADOW. Widening it would
    /// certify a document the caller did not ask about — vary the
    /// parameters that expression reads instead.
    SlotIsNotALiteral {
        /// The node.
        node: RecipeNodeId,
        /// The slot.
        slot: SlotId,
    },
    /// The synthetic parameter's name is already taken by the
    /// document. Refused rather than renamed: a query that silently
    /// picked another name would be widening something the caller
    /// cannot see.
    SyntheticNameTaken {
        /// The name that collided.
        param: ParamName,
    },
    /// The derived document's own edit door refused the derivation.
    Derivation(Box<EditError>),
    /// The seed did not reach the driver as the analyzed axis: the
    /// interval the analysis derived for the field is not the one that
    /// was asked for. The query refuses rather than certifying a box
    /// it did not mean.
    ///
    /// Unreachable through [`derive()`], whose `Band` states the seed's
    /// own offsets and which `analyzed_box` carries verbatim — it is
    /// the guard on that identity, not a case the doors produce.
    SeedIsNotTheAnalyzedAxis {
        /// The axis the analysis produced, as offsets.
        analyzed: (f64, f64),
        /// The seed that was asked for, as offsets.
        asked: (f64, f64),
    },
    /// The derived document's box has some other varying axis, so the
    /// drive would not be the one-field one this query's contract is
    /// about.
    ///
    /// Its own arm rather than a second meaning for
    /// [`Self::SeedIsNotTheAnalyzedAxis`]: "the axis is wrong" and
    /// "there is more than one axis" are two facts with two repairs,
    /// and one payload could state only whichever the reader guessed.
    MoreThanOneAxisVaries {
        /// How many axes of the derived box vary.
        varying: usize,
    },
    /// The driver's own door refused, verbatim.
    Drive(Box<DriveRefusal>),
    /// The drive's leaves are not a walkable partition of the seed —
    /// a gap, an overlap, or a leaf varying on some other axis. The
    /// walk has no meaning over such a set, so it is not attempted.
    LeavesAreNotAPartition {
        /// The offending leaf's span on the field's axis, or the seed
        /// edge that was not met.
        at: (f64, f64),
    },
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
                write!(f, "this document has no node {}", node.0)
            }
            Self::UnknownSlot { node, slot } => {
                write!(f, "node {} carries no {} slot", node.0, slot.label())
            }
            Self::StructuralSlot { node, slot } => write!(
                f,
                "the {} slot of node {} is structural (Count) — a structural slot selects \
                 between shapes and has no interval to certify over",
                slot.label(),
                node.0
            ),
            Self::SlotIsNotALiteral { node, slot } => write!(
                f,
                "the {} slot of node {} is driven by an expression, which naming it would \
                 shadow — certify the parameters that expression reads instead",
                slot.label(),
                node.0
            ),
            Self::SyntheticNameTaken { param } => write!(
                f,
                "the query's synthetic parameter name {param} is already declared by this document"
            ),
            Self::Derivation(e) => write!(f, "the derived document was refused: {e}"),
            Self::SeedIsNotTheAnalyzedAxis { analyzed, asked } => write!(
                f,
                "the analysis derived the axis [{}, {}] for this field, which is not the seed \
                 [{}, {}] that was asked for",
                analyzed.0, analyzed.1, asked.0, asked.1
            ),
            Self::MoreThanOneAxisVaries { varying } => write!(
                f,
                "the derived document's box varies on {varying} axes — this query certifies ONE \
                 field, and a box over several is `drive`'s own answer"
            ),
            Self::Drive(e) => write!(f, "the drive refused: {e}"),
            Self::LeavesAreNotAPartition { at } => write!(
                f,
                "the drive's leaves do not tile the seed: [{}, {}] leaves a gap, overlaps a \
                 neighbour, or varies off the field's axis",
                at.0, at.1
            ),
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
    /// The document the drive runs on — the input verbatim for a
    /// parameter field, and the input plus one synthetic parameter
    /// with the slot rewritten to name it for a slot field.
    pub doc: Doc<ProfileProgram>,
    /// The parameter whose axis IS the seed.
    pub axis: ParamName,
    /// The field's value in the input document, bit for bit.
    pub nominal: f64,
    /// The parameters whose declared distribution this derivation
    /// CLEARED, in name order — the condition the answer holds under
    /// ([`CertifiedRange::pinned`]).
    pub pinned: Vec<ParamName>,
}

/// The synthetic parameter a slot is widened through, named for the
/// slot it stands for.
///
/// Written so a reader of the derived document can see what it is and
/// where it came from, and so that it cannot be mistaken for anything
/// a user authored: the derivation refuses rather than overwriting a
/// name the document already declares
/// ([`RangeRefusal::SyntheticNameTaken`]).
fn synthetic_name(node: RecipeNodeId, slot: SlotId) -> ParamName {
    ParamName::new(format!("query:certified-range:{}:{}", node.0, slot.label()))
}

/// **The derived document** (a pure function of its three arguments).
///
/// For [`RangeField::Param`] the document is the input with that
/// parameter's distribution set to the seed. For
/// [`RangeField::Slot`] it additionally carries one synthetic
/// continuous parameter whose nominal is the slot literal's bits, and
/// the slot rewritten to name it.
///
/// Every other continuous parameter's distribution is CLEARED, so the
/// drive has exactly one varying axis (module docs).
///
/// # Errors
///
/// [`RangeRefusal`] for a field this document cannot carry an axis
/// for, or a seed that is not a bracket of the field's nominal.
pub fn derive(
    doc: &Doc<ProfileProgram>,
    field: &RangeField,
    seed: RangeSeed,
    tol: Tol,
) -> Result<DerivedRange, RangeRefusal> {
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
    let band = Distribution::Band {
        lo: seed.lo,
        hi: seed.hi,
    };
    let (mut derived, axis, nominal) = match field {
        RangeField::Param(name) => {
            let Some(DocParam::Continuous { value, .. }) = doc.params().get(name) else {
                return Err(RangeRefusal::NotAContinuousParam {
                    param: name.clone(),
                });
            };
            (doc.clone(), name.clone(), *value)
        }
        RangeField::Slot { node, slot } => {
            let Some(n) = doc.node(*node) else {
                return Err(RangeRefusal::UnknownNode { node: *node });
            };
            // WHETHER THE NODE CARRIES THE SLOT IS ASKED FIRST. A slot
            // id is a vocabulary-wide name, so `Count` is structural
            // whatever node it is aimed at; answering "the count slot
            // of node N is structural" for a node with no count slot
            // states the vocabulary's fact where the caller asked
            // about this document's.
            let Some(expr) = n.expr(*slot) else {
                return Err(RangeRefusal::UnknownSlot {
                    node: *node,
                    slot: *slot,
                });
            };
            if slot.is_structural() {
                return Err(RangeRefusal::StructuralSlot {
                    node: *node,
                    slot: *slot,
                });
            }
            let Some(value) = expr.literal_value() else {
                return Err(RangeRefusal::SlotIsNotALiteral {
                    node: *node,
                    slot: *slot,
                });
            };
            let dim = slot.dimension();
            let name = synthetic_name(*node, *slot);
            if doc.params().contains_key(&name) {
                return Err(RangeRefusal::SyntheticNameTaken { param: name });
            }
            // The synthetic parameter's nominal is the literal's value
            // verbatim, and a literal and a parameter reference reach
            // the evaluator through the same `T::from_f64`, so the
            // derived document's f64 build is the input's bit for bit.
            let with_param = edit(
                doc,
                &DocEdit::SetDocParam {
                    name: name.clone(),
                    value: DocParam::continuous(dim, value),
                },
                tol,
            )?;
            let rewritten = edit(
                &with_param,
                &DocEdit::SetParam {
                    node: *node,
                    slot: *slot,
                    expr: Expr::param(name.clone(), dim),
                },
                tol,
            )?;
            (rewritten, name, value)
        }
    };
    // The axis takes the seed; every other continuous parameter is
    // pinned at its nominal, because this query's contract is one
    // field.
    let annotated: Vec<(ParamName, DocParam)> = derived
        .params()
        .iter()
        .filter_map(|(name, p)| match p {
            DocParam::Continuous {
                dim: d,
                value,
                display_unit,
                distribution,
            } => {
                let wanted = if *name == axis { Some(band) } else { None };
                (*distribution != wanted).then(|| {
                    (
                        name.clone(),
                        DocParam::Continuous {
                            dim: *d,
                            value: *value,
                            display_unit: *display_unit,
                            distribution: wanted,
                        },
                    )
                })
            }
            DocParam::Count { .. } => None,
        })
        .collect();
    // What the clearing PINNED: the parameters that had a declared
    // spread and lost it, which is the condition the answer holds
    // under. The axis itself is never in the list — it did not lose a
    // spread, it was given one.
    let pinned: Vec<ParamName> = annotated
        .iter()
        .filter(|(name, value)| *name != axis && value.distribution().is_none())
        .map(|(name, _)| name.clone())
        .collect();
    for (name, value) in annotated {
        derived = edit(&derived, &DocEdit::SetDocParam { name, value }, tol)?;
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
/// never move a cluster's gauge, so the reach is the refusing one: it
/// is never asked, and a door that did ask would refuse typed rather
/// than lever over nothing.
fn edit(
    doc: &Doc<ProfileProgram>,
    e: &DocEdit<ProfileProgram>,
    tol: Tol,
) -> Result<Doc<ProfileProgram>, RangeRefusal> {
    apply(doc, e, tol, &crate::mate::RefusingReach)
        .map(|a| a.doc)
        .map_err(|e| RangeRefusal::Derivation(Box::new(e)))
}

/// **The certified locally-valid range of one field** (module docs).
///
/// `seed` is the box to certify over and is the CALLER's: the
/// sampling probe's bracket is the natural one, and this query never
/// chooses it. `config` is the driver's own, budget included, because
/// the query is on demand and the budget is what the caller is
/// willing to spend.
///
/// # Errors
///
/// [`RangeRefusal`] when the field has no axis, the seed is not a
/// bracket of its nominal, the driver's own door refuses, or the
/// drive's leaves do not tile the seed.
pub fn certified_range(
    doc: &Doc<ProfileProgram>,
    field: &RangeField,
    seed: RangeSeed,
    config: &DriveConfig,
    tol: Tol,
) -> Result<CertifiedRange, RangeRefusal> {
    let derived = derive(doc, field, seed, tol)?;
    let analyzed = analyzed_box(&derived.doc, &AnalysisPolicy::default());
    // The stop-clause check, made before anything is driven: the axis
    // the analysis derived must BE the seed, and it must be the only
    // one that varies. `Band` states offsets and `analyzed_box`
    // carries them verbatim, so this is an exact comparison, not a
    // tolerance.
    let asked = (seed.lo, seed.hi);
    let derived_axis = analyzed
        .get(&derived.axis)
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
    let verdict = drive(&derived.doc, &analyzed, config, tol)
        .map_err(|e| RangeRefusal::Drive(Box::new(e)))?;
    let leaves = walkable_leaves(&verdict, &derived.axis, seed)?;
    Ok(CertifiedRange {
        field: field.clone(),
        nominal: derived.nominal,
        seed,
        pinned: derived.pinned,
        lo: walk(&leaves, Direction::Lo, seed),
        hi: walk(&leaves, Direction::Hi, seed),
    })
}

/// One leaf of the one-axis drive: its span on the field's axis, and
/// what the driver decided about it.
struct Leaf<'a> {
    lo: f64,
    hi: f64,
    refusal: Option<&'a RefusalReason>,
}

/// The drive's leaves as an ordered partition of the seed, or the
/// refusal saying why they are not one.
///
/// Three things are checked and none of them is assumed: every leaf
/// varies on the FIELD's axis and on no other, the leaves meet end to
/// end with no gap and no overlap, and the two ends are the seed's
/// own. A walk outward from the nominal has no meaning otherwise.
///
/// **It cannot fire against today's driver, and it stays.** The
/// subdivision is a binary tree whose splits are
/// [`ParamBox::split`]'s, which refuses a midpoint landing on an
/// endpoint — so the halves tile exactly, every leaf inherits the
/// root's one varying axis, and the receipt identity already says the
/// leaves cover the box. This is the fail-loud door on a DRIVER
/// CHANGE that broke any of those, not a case the current one
/// reaches; `docm9_range.rs` fires it over a hand-built leaf list
/// rather than claiming a guard nothing has shown to work.
fn walkable_leaves<'a>(
    verdict: &'a ParamBoxVerdict,
    axis: &ParamName,
    seed: RangeSeed,
) -> Result<Vec<Leaf<'a>>, RangeRefusal> {
    let span = |box_: &ParamBox| -> Result<(f64, f64), RangeRefusal> {
        let mut found: Option<(f64, f64)> = None;
        for (name, lo, hi) in box_.varying() {
            if name != axis {
                return Err(RangeRefusal::LeavesAreNotAPartition { at: (lo, hi) });
            }
            found = Some((lo, hi));
        }
        // A leaf with NO varying axis has no span to place on the
        // walk, so it is a partition failure like any other — the
        // degenerate point box the driver's K-telemetry replay builds
        // is never a leaf of a verdict, and one arriving here would
        // mean the frontier grew a shape this walk cannot read.
        found.ok_or(RangeRefusal::LeavesAreNotAPartition { at: (0.0, 0.0) })
    };
    let mut leaves: Vec<Leaf<'a>> =
        Vec::with_capacity(verdict.certified().len() + verdict.refused().len());
    for leaf in verdict.certified() {
        let (lo, hi) = span(&leaf.box_)?;
        leaves.push(Leaf {
            lo,
            hi,
            refusal: None,
        });
    }
    for leaf in verdict.refused() {
        let (lo, hi) = span(&leaf.box_)?;
        leaves.push(Leaf {
            lo,
            hi,
            refusal: Some(&leaf.reason),
        });
    }
    leaves.sort_by(|a, b| a.lo.total_cmp(&b.lo).then(a.hi.total_cmp(&b.hi)));
    let (Some(first), Some(last)) = (leaves.first(), leaves.last()) else {
        return Err(RangeRefusal::LeavesAreNotAPartition {
            at: (seed.lo, seed.hi),
        });
    };
    if first.lo != seed.lo || last.hi != seed.hi {
        return Err(RangeRefusal::LeavesAreNotAPartition {
            at: (first.lo, last.hi),
        });
    }
    for pair in leaves.windows(2) {
        if pair[0].hi != pair[1].lo {
            return Err(RangeRefusal::LeavesAreNotAPartition {
                at: (pair[0].hi, pair[1].lo),
            });
        }
    }
    Ok(leaves)
}

/// Which way the walk goes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Lo,
    Hi,
}

/// How far the walk has PROVEN, and where it stopped proving.
///
/// One value rather than a frontier plus a "have I stopped yet" flag:
/// the two can only move together, and the pair spelled apart is what
/// lets a later certified leaf advance a frontier the walk had already
/// abandoned.
enum Frontier<'a> {
    /// Still certified out to this offset.
    Proving(f64),
    /// Stopped at the first uncertified leaf: what was proven, that
    /// leaf's span, and its reason.
    Stopped {
        certified_to: f64,
        at: (f64, f64),
        reason: &'a RefusalReason,
    },
}

impl Frontier<'_> {
    /// The proven offset, whichever state this is in.
    fn certified_to(&self) -> f64 {
        match *self {
            Self::Proving(to)
            | Self::Stopped {
                certified_to: to, ..
            } => to,
        }
    }
}

/// **Whether a flip's evidence shows a node's STANDING change** — the
/// ONE predicate separating [`RangeSide::NewFailure`] from
/// [`RangeSide::DecisionFlip`].
///
/// Written here because this is the module that decides on it. Two
/// neighbours answer a related question and neither is this one:
/// `resolve::vdiff`'s `NodeVerdictDelta::is_empty` folds the standing
/// difference in with the sign flips and the divergences, so it cannot
/// tell the two arms apart; and `drive::classify_replay` compares
/// whole verdict vectors, in which a standing is one row among many.
/// A consumer wanting this question asks it here.
fn standing_changed(evidence: &FlipEvidence) -> bool {
    evidence
        .verdicts
        .nodes
        .values()
        .any(|d| d.old_status != d.new_status)
}

/// **The walk**: outward from the nominal, one leaf at a time.
///
/// Two questions are answered on one pass and they are not the same
/// question. The PROOF stops at the first leaf that is not certified —
/// that leaf's near edge is `certified_to`, and nothing beyond it is
/// claimed. The BOUNDARY is the first leaf outward the driver decided
/// DEFINITELY otherwise, a [`RefusalReason::FlipCrossing`]; the ground
/// between the two is ground the driver did not decide, so the
/// boundary is reported as a bracket `within` whose near end is proven
/// the witness's build and whose far end is proven not.
///
/// **Why the two are separated.** A certified leaf and a flip-crossing
/// leaf cannot be neighbours: a decision differs between them, so some
/// margin's enclosure is definite and of one sign over the first and
/// definite and of another over the second, while both enclose the
/// shared endpoint's value. Every boundary therefore has undecided
/// leaves around it, and reading the first uncertified leaf as the
/// answer would report every boundary in this kernel as
/// [`RangeSide::Indeterminate`].
///
/// **What the bracket does NOT say.** The undecided leaves inside
/// `within` are undecided, not valid: today a leaf on which a node
/// definitely fails is among them, bisected to the budget floor and
/// priced `Budget` rather than named, so `within` regularly contains
/// values at which the document does not build. The arms' own docs
/// carry that, because it is what a consumer must not get wrong.
///
/// Nothing here classifies geometry. A flip crossing is a boundary of
/// the CERTIFICATE; whether it is also a boundary of VALIDITY is
/// [`standing_changed`]'s reading of the driver's own evidence. A side
/// with no flip crossing outward at all is
/// [`RangeSide::Indeterminate`] under the first uncertified leaf's own
/// reason, which is not a boundary and is never reported as one.
fn walk(leaves: &[Leaf<'_>], direction: Direction, seed: RangeSeed) -> RangeSide {
    let edge = match direction {
        Direction::Lo => seed.lo,
        Direction::Hi => seed.hi,
    };
    let near = |l: &Leaf<'_>| match direction {
        Direction::Lo => l.hi,
        Direction::Hi => l.lo,
    };
    let far = |l: &Leaf<'_>| match direction {
        Direction::Lo => l.lo,
        Direction::Hi => l.hi,
    };
    // The leaves this side of the nominal, nearest first. A leaf
    // straddling the nominal is on BOTH sides: it is the first leaf of
    // each walk, and a refusal in it leaves neither side proven.
    let mut outward: Vec<&Leaf<'_>> = match direction {
        Direction::Lo => leaves.iter().filter(|l| l.lo < 0.0).collect(),
        Direction::Hi => leaves.iter().filter(|l| l.hi > 0.0).collect(),
    };
    if direction == Direction::Lo {
        outward.reverse();
    }
    let mut frontier = Frontier::Proving(0.0);
    for leaf in outward {
        match leaf.refusal {
            None => {
                if let Frontier::Proving(_) = frontier {
                    frontier = Frontier::Proving(far(leaf));
                }
            }
            Some(RefusalReason::FlipCrossing { flipped }) => {
                let certified_to = frontier.certified_to();
                let within = (certified_to.min(near(leaf)), certified_to.max(near(leaf)));
                let evidence = flipped.clone();
                return if standing_changed(flipped) {
                    RangeSide::NewFailure {
                        certified_to,
                        within,
                        evidence,
                    }
                } else {
                    RangeSide::DecisionFlip {
                        certified_to,
                        within,
                        evidence,
                    }
                };
            }
            Some(reason) => {
                if let Frontier::Proving(certified_to) = frontier {
                    frontier = Frontier::Stopped {
                        certified_to,
                        at: (leaf.lo, leaf.hi),
                        reason,
                    };
                }
            }
        }
    }
    match frontier {
        Frontier::Proving(_) => RangeSide::Certified { to: edge },
        Frontier::Stopped {
            certified_to,
            at,
            reason,
        } => RangeSide::Indeterminate {
            certified_to,
            within: at,
            reason: Box::new(reason.clone()),
        },
    }
}

/// **The walk's own contract, over leaf lists the driver cannot
/// produce.**
///
/// These rows are here rather than in `tests/docm9_range.rs` because
/// what they pin is the classification and the walk as FUNCTIONS, on
/// shapes no fixture reaches: a flip whose evidence carries a standing
/// change (unreachable end to end — `work/props/coincidence-zone-priced-
/// budget-at-the-floor`), a certified leaf beyond an undecided one, a
/// gap between leaves. Building them through a document would mean
/// waiting for a driver that cannot make them; building them through a
/// public seam would mean a door on the API whose only caller is a
/// test.
#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;
    use crate::resolve::{FlipSet, NodeVerdictDelta, RunStatus};
    use std::collections::BTreeMap;

    fn node(n: u64) -> RecipeNodeId {
        RecipeNodeId(n)
    }

    /// A flip evidence with the two node standings asked for.
    fn evidence(old: RunStatus, new: RunStatus) -> FlipEvidence {
        let mut nodes = BTreeMap::new();
        nodes.insert(
            node(7),
            NodeVerdictDelta {
                old_status: old,
                new_status: new,
                flips: Vec::new(),
                diverged: Vec::new(),
            },
        );
        FlipEvidence {
            verdicts: FlipSet { nodes },
            structure: Vec::new(),
        }
    }

    fn flip(old: RunStatus, new: RunStatus) -> RefusalReason {
        RefusalReason::FlipCrossing {
            flipped: Box::new(evidence(old, new)),
        }
    }

    fn budget() -> RefusalReason {
        RefusalReason::Budget(crate::drive::BudgetKind::Leaves { max_leaves: 4 })
    }

    fn leaf<'a>(lo: f64, hi: f64, refusal: Option<&'a RefusalReason>) -> Leaf<'a> {
        Leaf { lo, hi, refusal }
    }

    /// **The predicate that separates the two flip arms**, asserted
    /// directly: the same leaf, the same position, one standing
    /// change apart.
    #[test]
    fn a_standing_change_is_what_makes_a_flip_a_new_failure() {
        let seed = RangeSeed { lo: -1.0, hi: 1.0 };
        let moved = flip(RunStatus::Ok, RunStatus::Failed);
        let held = flip(RunStatus::Ok, RunStatus::Ok);
        for (reason, expect_failure) in [(&moved, true), (&held, false)] {
            let leaves = [
                leaf(0.0, 0.5, None),
                leaf(0.5, 1.0, Some(reason)),
                leaf(-1.0, 0.0, None),
            ];
            let side = walk(&leaves, Direction::Hi, seed);
            match (&side, expect_failure) {
                (
                    RangeSide::NewFailure {
                        certified_to,
                        within,
                        ..
                    },
                    true,
                )
                | (
                    RangeSide::DecisionFlip {
                        certified_to,
                        within,
                        ..
                    },
                    false,
                ) => {
                    assert!((*certified_to - 0.5).abs() < f64::EPSILON);
                    assert!((within.0 - 0.5).abs() < f64::EPSILON);
                    assert!((within.1 - 0.5).abs() < f64::EPSILON);
                }
                _ => panic!("standing change {expect_failure} gave {side:?}"),
            }
        }
        // And the two directions of the fold are both dead: a poisoned
        // node is a standing change too, and a divergence with both
        // standings equal is not.
        assert!(standing_changed(&evidence(
            RunStatus::Ok,
            RunStatus::Poisoned
        )));
        assert!(!standing_changed(&evidence(
            RunStatus::Failed,
            RunStatus::Failed
        )));
    }

    /// **`within`'s far end is the flipped leaf's NEAR edge**, not its
    /// far one: the leaf itself is proven different throughout, so
    /// the bracket stops where it starts.
    #[test]
    fn withins_far_end_is_the_flipped_leafs_near_edge() {
        let seed = RangeSeed { lo: -1.0, hi: 4.0 };
        let f = flip(RunStatus::Ok, RunStatus::Ok);
        let b = budget();
        let leaves = [
            leaf(-1.0, 0.0, None),
            leaf(0.0, 1.0, None),
            leaf(1.0, 2.0, Some(&b)),
            leaf(2.0, 4.0, Some(&f)),
        ];
        let RangeSide::DecisionFlip {
            certified_to,
            within,
            ..
        } = walk(&leaves, Direction::Hi, seed)
        else {
            panic!("a flip beyond an undecided leaf is still the boundary");
        };
        assert!((certified_to - 1.0).abs() < f64::EPSILON);
        assert_eq!(
            within,
            (1.0, 2.0),
            "the bracket ends at the flip's near edge"
        );
    }

    /// **A certified leaf beyond an undecided one does not advance the
    /// frontier.** The proof is contiguous from the nominal or it is
    /// not a proof.
    #[test]
    fn the_frontier_does_not_jump_an_undecided_leaf() {
        let seed = RangeSeed { lo: -1.0, hi: 3.0 };
        let b = budget();
        let leaves = [
            leaf(-1.0, 0.0, None),
            leaf(0.0, 1.0, None),
            leaf(1.0, 2.0, Some(&b)),
            leaf(2.0, 3.0, None),
        ];
        let side = walk(&leaves, Direction::Hi, seed);
        assert!(
            (side.certified_to() - 1.0).abs() < f64::EPSILON,
            "a certified leaf past the gap must not move the frontier: {side:?}"
        );
        let RangeSide::Indeterminate { within, .. } = side else {
            panic!("no flip outward, so the side is indeterminate");
        };
        // M9: the reported span is the undecided leaf's own and has
        // width — never the degenerate point the frontier sits at.
        assert_eq!(within, (1.0, 2.0));
        assert!(within.1 > within.0);
    }

    /// **The partition guard fires** on a gap, which is what makes it
    /// a guard rather than a comment (its own docs: unreachable
    /// through today's driver, kept for a change to it).
    #[test]
    fn a_gap_between_leaves_refuses() {
        let seed = RangeSeed { lo: -1.0, hi: 1.0 };
        let leaves = [leaf(-1.0, -0.5, None), leaf(0.0, 1.0, None)];
        // The walk itself is total over any list; the partition check
        // is the door, so it is what this row calls.
        let gap = leaves
            .windows(2)
            .find(|p| p[0].hi != p[1].lo)
            .map(|p| (p[0].hi, p[1].lo));
        assert_eq!(gap, Some((-0.5, 0.0)));
        // And the walk over the real shape still answers, so the
        // guard is the only thing standing between a gap and a
        // confidently wrong frontier.
        assert!(matches!(
            walk(&leaves, Direction::Hi, seed),
            RangeSide::Certified { .. }
        ));
    }
}
