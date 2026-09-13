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
//! **A certified range is a subset of every locally-valid range.** The
//! probe asks "does anything NEW fail"; a certificate asks "does
//! anything DECIDE differently", which is strictly more. A value the
//! query cannot certify may still be perfectly valid — a recorded
//! predicate flipping with every node still building is the case, and
//! it has its own arm ([`RangeSide::DecisionFlip`]) precisely so that it is
//! never read as a failure. The two answer different questions, so a
//! consumer shows both or names which one it is showing.
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
//! own answer, asked for directly.
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
pub struct Seed {
    /// Lower offset (`<= 0`).
    pub lo: f64,
    /// Upper offset (`>= 0`).
    pub hi: f64,
}

impl Seed {
    /// A symmetric seed of half-width `w`.
    #[must_use]
    pub fn symmetric(w: f64) -> Self {
        Self { lo: -w, hi: w }
    }
}

/// **One side's verdict, four arms and no other.**
///
/// A leaf's own refusal class decides its arm; this query invents no
/// decision of its own and folds no arm into another.
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
    /// The first leaf outward the driver classified DEFINITELY
    /// otherwise is a [`RefusalReason::FlipCrossing`] whose evidence
    /// shows a node's STANDING change — a node that was `Ok` at the
    /// witness is not, or the reverse. The boundary the probe looks
    /// for is inside `within`, and `[nominal, certified_to]` is
    /// proven.
    NewFailure {
        /// The proven frontier, as an offset (zero when nothing this
        /// side of the nominal certified).
        certified_to: f64,
        /// The bracket the boundary lies in, as offsets: from
        /// `certified_to` — proven the witness's build — to the near
        /// edge of the flipped leaf, proven not. Undecided ground in
        /// between is exactly what makes this a bracket rather than a
        /// number.
        within: (f64, f64),
        /// The driver's evidence, verbatim.
        evidence: Box<FlipEvidence>,
    },
    /// The first leaf outward the driver classified DEFINITELY
    /// otherwise is a [`RefusalReason::FlipCrossing`] with NO standing
    /// change: a recorded predicate decides differently inside
    /// `within` while every node still builds.
    ///
    /// **This is the boundary of the CERTIFICATE, not necessarily of
    /// validity.** The probe would call such values valid and would
    /// be right to; the query says only that it cannot prove them to
    /// be the same build, and names what differs. Never folded into
    /// [`RangeSide::NewFailure`].
    DecisionFlip {
        /// The proven frontier, as an offset.
        certified_to: f64,
        /// The bracket the boundary lies in, as offsets
        /// ([`RangeSide::NewFailure`]'s `within`).
        within: (f64, f64),
        /// The driver's evidence, verbatim.
        evidence: Box<FlipEvidence>,
    },
    /// NO leaf outward was classified definitely otherwise, and the
    /// first uncertified one is [`RefusalReason::Budget`],
    /// [`RefusalReason::SliverTerminal`],
    /// [`RefusalReason::Bifurcation`], [`RefusalReason::Infeasible`]
    /// or [`RefusalReason::MeasureRefused`]: the driver could not
    /// DECIDE `within`, and found nothing beyond it either.
    ///
    /// **Not a failure and not a boundary**, and never to be rendered
    /// as one. The recourse is the reason's own — more budget, a
    /// coarser seed, a document repair.
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
#[derive(Debug, Clone, PartialEq)]
pub struct CertifiedRange {
    field: RangeField,
    nominal: f64,
    seed: Seed,
    lo: RangeSide,
    hi: RangeSide,
}

impl CertifiedRange {
    /// The field this range is about.
    pub fn field(&self) -> &RangeField {
        &self.field
    }

    /// The field's value in the input document — the point the
    /// witness build is taken at, and the origin every offset here is
    /// measured from.
    pub fn nominal(&self) -> f64 {
        self.nominal
    }

    /// The seed the caller chose.
    pub fn seed(&self) -> Seed {
        self.seed
    }

    /// The downward side.
    pub fn lo(&self) -> &RangeSide {
        &self.lo
    }

    /// The upward side.
    pub fn hi(&self) -> &RangeSide {
        &self.hi
    }

    /// An offset read as an absolute field value, `nominal + offset`
    /// at `f64`. A reader's convenience: the PROOF is over the
    /// offsets, which the driver's axis carries exactly, and this sum
    /// rounds.
    pub fn absolute(&self, offset: f64) -> f64 {
        self.nominal + offset
    }

    /// The proven interval as offsets: `(lo.certified_to(),
    /// hi.certified_to())`. Every value in it builds exactly what the
    /// witness built.
    pub fn certified_offsets(&self) -> (f64, f64) {
        (self.lo.certified_to(), self.hi.certified_to())
    }

    /// The proven interval in absolute field values
    /// ([`Self::absolute`] of [`Self::certified_offsets`]).
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
    /// The seed is not a bracket of the nominal: a field is finite,
    /// `lo <= 0 <= hi` fails, or the seed has no width.
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
    /// axis the analysis derived is not the interval that was asked
    /// for, or some other axis of the derived document still varies.
    /// The query refuses rather than certifying a box it did not mean.
    SeedIsNotTheAnalyzedAxis {
        /// The axis the analysis produced, as offsets.
        analyzed: (f64, f64),
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
                "{:?} is not a continuous parameter of this document — there is no axis to \
                 certify over",
                param.0
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
                "the query's synthetic parameter name {:?} is already declared by this document",
                param.0
            ),
            Self::Derivation(e) => write!(f, "the derived document was refused: {e}"),
            Self::SeedIsNotTheAnalyzedAxis { analyzed } => write!(
                f,
                "the analysis derived the axis [{}, {}], which is not the seed that was asked \
                 for — or another axis of the derived document still varies",
                analyzed.0, analyzed.1
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
pub struct Derived {
    /// The document the drive runs on — the input verbatim for a
    /// parameter field, and the input plus one synthetic parameter
    /// with the slot rewritten to name it for a slot field.
    pub doc: Doc<ProfileProgram>,
    /// The parameter whose axis IS the seed.
    pub axis: ParamName,
    /// The field's value in the input document, bit for bit.
    pub nominal: f64,
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
    seed: Seed,
    tol: Tol,
) -> Result<Derived, RangeRefusal> {
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
            if slot.is_structural() {
                return Err(RangeRefusal::StructuralSlot {
                    node: *node,
                    slot: *slot,
                });
            }
            let Some(expr) = n.expr(*slot) else {
                return Err(RangeRefusal::UnknownSlot {
                    node: *node,
                    slot: *slot,
                });
            };
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
    for (name, value) in annotated {
        derived = edit(&derived, &DocEdit::SetDocParam { name, value }, tol)?;
    }
    Ok(Derived {
        doc: derived,
        axis,
        nominal,
    })
}

/// One edit, applied purely, with the door's refusal carried.
fn edit(
    doc: &Doc<ProfileProgram>,
    e: &DocEdit<ProfileProgram>,
    tol: Tol,
) -> Result<Doc<ProfileProgram>, RangeRefusal> {
    apply(doc, e, tol)
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
    seed: Seed,
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
    if derived_axis != asked || analyzed.varying().count() != 1 {
        return Err(RangeRefusal::SeedIsNotTheAnalyzedAxis {
            analyzed: derived_axis,
        });
    }
    let verdict = drive(&derived.doc, &analyzed, config, tol)
        .map_err(|e| RangeRefusal::Drive(Box::new(e)))?;
    let leaves = walkable_leaves(&verdict, &derived.axis, seed)?;
    Ok(CertifiedRange {
        field: field.clone(),
        nominal: derived.nominal,
        seed,
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
fn walkable_leaves<'a>(
    verdict: &'a ParamBoxVerdict,
    axis: &ParamName,
    seed: Seed,
) -> Result<Vec<Leaf<'a>>, RangeRefusal> {
    let span = |box_: &ParamBox| -> Result<(f64, f64), RangeRefusal> {
        let mut found: Option<(f64, f64)> = None;
        for (name, lo, hi) in box_.varying() {
            if name != axis {
                return Err(RangeRefusal::LeavesAreNotAPartition { at: (lo, hi) });
            }
            found = Some((lo, hi));
        }
        // A leaf degenerate on the field's own axis is the `f64` grid
        // showing through, not a partition failure: it reads as the
        // nominal point of the axis it was split on.
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

/// **The walk**: outward from the nominal, one leaf at a time.
///
/// Two questions are answered on one pass and they are not the same
/// question. The PROOF stops at the first leaf that is not certified —
/// that leaf's near edge is `certified_to`, and nothing beyond it is
/// claimed. The BOUNDARY is the first leaf outward the driver
/// classified DEFINITELY otherwise, a
/// [`RefusalReason::FlipCrossing`]; the ground between the two is
/// ground the driver could not decide, so the boundary is reported as
/// a bracket `within` whose near end is proven the witness's build and
/// whose far end is proven not.
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
/// Nothing here classifies geometry. A flip crossing is a boundary of
/// the CERTIFICATE; whether it is also a boundary of VALIDITY is read
/// off the evidence's node standings — a node that built at the
/// witness and does not in the leaf (or the reverse) is
/// [`RangeSide::NewFailure`], and anything else is
/// [`RangeSide::DecisionFlip`]. A side with no flip crossing outward
/// at all is [`RangeSide::Indeterminate`] under the first uncertified
/// leaf's own reason, which is not a boundary and is never reported as
/// one.
fn walk(leaves: &[Leaf<'_>], direction: Direction, seed: Seed) -> RangeSide {
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
    let mut certified_to = 0.0_f64;
    let mut first: Option<((f64, f64), &RefusalReason)> = None;
    for leaf in outward {
        match leaf.refusal {
            None => {
                if first.is_none() {
                    certified_to = far(leaf);
                }
            }
            Some(RefusalReason::FlipCrossing { flipped }) => {
                let standing_changed = flipped
                    .verdicts
                    .nodes
                    .values()
                    .any(|d| d.old_status != d.new_status);
                let within = (certified_to.min(near(leaf)), certified_to.max(near(leaf)));
                let evidence = flipped.clone();
                return if standing_changed {
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
                first.get_or_insert(((leaf.lo, leaf.hi), reason));
            }
        }
    }
    match first {
        None => RangeSide::Certified { to: edge },
        Some((within, reason)) => RangeSide::Indeterminate {
            certified_to,
            within,
            reason: Box::new(reason.clone()),
        },
    }
}
