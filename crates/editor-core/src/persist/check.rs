//! The persistence doors' ONE shared validator (spec D2/D6.3; DESIGN
//! engineering convention 2, discharged M5 S4):
//!
//! [`validate_document`] holds every direction-independent document
//! check and is invoked by BOTH doors — at save on the in-memory
//! document before a byte is written, at load on the parsed document
//! before replay. A document that would refuse to load is therefore
//! impossible to save by construction: not two mirrored door sets
//! kept in sync by a sweep, but code that is literally the same and
//! cannot drift.
//!
//! **The census of its walks is the code's.** [`Walk`] is the roster,
//! [`Walk::ORDER`] is the order [`validate_document`] iterates, and
//! [`Walk::run`] is the exhaustive map from a walk to the function
//! behind it and to the refusal it raises — none of the three compiles
//! with a walk missing from it. What each walk covers, and why, is on
//! that walk's own variant; a second hand-written list here is exactly
//! what undercounted twice.
//!
//! Two things no roster can carry stay in prose. **The ORDER** — which
//! adjacencies are contracts and which are free — is on
//! [`validate_document`]. **Why most walks read the SNAPSHOT only** is
//! this, said once instead of at each: a `SetDocParam` in the log
//! carries its payload through `apply` on replay, which is the same
//! door and the same check, so a second walk over the log would ask a
//! question `apply` has already answered. Every walk that says
//! "snapshot only" holds for this reason and no other.
//!
//! The genuinely asymmetric residue stays at its door and is the
//! symmetry sweep's whole remit now: header/parse/position errors and
//! the wire-only canonical-set rule ([`super::wire`]) are load-only by
//! nature; serializer failure is save-only; ε reconciliation is
//! process state, not a document property. Log replayability is
//! shared STRUCTURALLY instead — both doors replay through
//! [`crate::edit::apply`].

use crate::appearance::AppearanceRecord;
use crate::distribution::DistributionFault;
use crate::doc::{DocParam, DocParamField, GaugeRefFault, ParamName, WitnessSiteFault};
use crate::edit::DocEdit;
use crate::meta::MetaVersionError;
use crate::node::SlotId;
use crate::node::{AssertionBoundFault, Node, RecipeNodeId, SlotDimensionFault};
use crate::placement::{FrameFault, FrameSite};
use crate::program::{ProfileDoc, ProfileProgram, ProgramRefusal};
use crate::resolve::derivation_nodes;
use crate::spoken::{SpokenName, SpokenNode};
use geom_core::Tol;

/// Where a non-finite float sits (the D2 refusal's typed site).
#[derive(Debug, Clone, PartialEq)]
pub enum NonFiniteSite {
    /// The recorded ε.
    Epsilon,
    /// A continuous document parameter (snapshot).
    DocParam {
        /// The parameter.
        name: ParamName,
        /// Which float of the parameter it is — the nominal, or the
        /// annotation's offset. The walk has to identify the field to
        /// decide there is a defect at all, so it says which one
        /// rather than discarding the answer.
        field: DocParamField,
    },
    /// A float inside an appearance record's metadata (snapshot, or a
    /// logged `SetAppearanceMeta`).
    Metadata {
        /// The attributed name, its minting node spoken from the
        /// snapshot: a name a logged edit carries whose node only a
        /// later edit inserts reads as a node the snapshot does not
        /// hold.
        name: SpokenName,
        /// The metadata key.
        key: String,
        /// Path within the value tree (dot/index notation).
        path: String,
    },
    /// A float carried by an edit in the log; `index` is the edit's
    /// position, `inner` the site within that edit's payload.
    Edit {
        /// The edit's index in the log.
        index: usize,
        /// The site within the edit.
        inner: Box<NonFiniteSite>,
    },
}

// The site prose. Each arm names WHERE the float sits, in the
// vocabulary a document author reads — the recursive `Edit` arm
// forwards the inner site rather than re-stating it.
impl core::fmt::Display for NonFiniteSite {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Epsilon => f.write_str("the recorded ε"),
            Self::DocParam {
                name,
                field: DocParamField::Nominal,
            } => write!(f, "document parameter {name}"),
            Self::DocParam { name, field } => {
                write!(f, "document parameter {name}, {field}")
            }
            Self::Metadata { name, key, path } => {
                write!(f, "metadata {key:?} on the {name}, at {path}")
            }
            Self::Edit { index, inner } => write!(f, "edit {index}, {inner}"),
        }
    }
}

/// **The roster of [`validate_document`]'s walks, in the order it runs
/// them** — the census of this door's refusals, in code rather than in
/// prose beside it.
///
/// A hand-written table of "which walk produces which refusal"
/// undercounted twice in three rounds, once by a whole walk. Nothing
/// here is written twice: [`Walk::ORDER`] is what
/// [`validate_document`] iterates, [`Walk::run`] is the exhaustive map
/// from a walk to the function behind it, and
/// `tests::every_snapshot_error_arm_names_the_walk_that_produces_it`
/// places every [`SnapshotError`] arm in the walk that raises it —
/// each of the three stops compiling when a walk or an arm is added
/// and not placed. What the code cannot know — the EDIT-door twin each
/// refusal has, and whether its predicate is shared — is said on each
/// [`SnapshotError`] arm's own doc; the table in the closed
/// `work/edit/three-door-predicates-are-hand-copied-not-shared` is the
/// record of the round that measured it, not a census kept current.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Walk {
    /// [`first_non_finite`] over ε, the params, the appearance records
    /// and the edit log — every float the format would write, checked
    /// finite, with a typed site name.
    ///
    /// Expression literals are finite BY CONSTRUCTION (`Expr::literal`
    /// refuses non-finite — ruled door 1; the load side re-runs the
    /// same constructors), so this walk covers the float carriers
    /// outside that door: profile PLANE PLACEMENTS (program args are
    /// Exprs, already doored), continuous doc params, the recorded ε,
    /// and D7 metadata trees, in the snapshot AND in the edit log. A
    /// NEW float-carrying field must join it — the D2 round-trip
    /// property tests are the tripwire. Post-parse it cannot fire —
    /// JSON has no non-finite tokens — which is the asymmetry being
    /// BYTE-level, not a reason to fork the validator.
    NonFinite,
    /// [`first_distribution_fault`] over the param table: the E2
    /// invariants of every doc param's distribution beyond finiteness,
    /// by the same `Distribution::check` the edit door runs. Snapshot
    /// only.
    Distribution,
    /// [`first_display_unit_fault`] over the param table: every
    /// document parameter's authored display unit measures the
    /// dimension it was declared with. A literal needs no twin walk
    /// (`Expr::literal_with_unit` makes the pairing at construction and
    /// the load side re-runs it); a `DocParam` does, because its
    /// payload is `pub` and its dimension is data. Snapshot only.
    DisplayUnit,
    /// [`first_slot_fault`] over every node's slots: every node's SLOT
    /// expressions carry the dimension their addresses fix (spec D6),
    /// by the same `Node::slot_dimension_fault` the edit doors ask.
    /// EVERY node kind — a walk that asked profile programs alone
    /// admitted a retyped extrude distance the edit door refuses.
    /// Snapshot only.
    SlotDimension,
    /// [`first_slot_param_ref_fault`] over every slot expression's
    /// document-parameter references, against the param table, by the
    /// same `Doc::param_ref_fault` the edit doors ask. An undeclared
    /// name and a dimension the declaration contradicts are facts about
    /// the document; a reference that merely fails to EVALUATE is V1
    /// class 2 and passes. Snapshot only.
    SlotParamRef,
    /// [`first_payload_param_ref_fault`] over the document-parameter
    /// references of every expression no slot addresses, asking that
    /// same one predicate.
    ///
    /// **Why this is a second walk rather than a wider domain for
    /// [`Walk::SlotParamRef`]**: the two answer at different
    /// ADDRESSES — a slot fault names the slot, a payload fault has
    /// only the node to name — so folding them would mint an address
    /// that is sometimes absent, and would erase the order between
    /// them, which [`validate_document`] holds as a contract.
    PayloadParamRef,
    /// [`first_program_fault`] over the profile programs' replay: a
    /// REPLAY PROBE under the document's params whose LATTICE
    /// violations refuse (the corrupt-file class — no authoring surface
    /// produces them). Resolve failures and geometry refusals PASS this
    /// door (V1 class 2: refusing programs may exist at rest — they
    /// surface as typed node errors at evaluation). A step argument's
    /// dimension is [`Walk::SlotDimension`]'s, not a second spelling
    /// here.
    Program,
    /// [`validate_snapshot`] over the document's structural invariants
    /// — the ones `apply` maintains, re-checked because a parsed
    /// snapshot is not trusted and an in-memory one can be corrupted
    /// through the `pub` payload or an in-crate bug.
    ///
    /// Every rule an edit door also decides is DELEGATED to the one
    /// predicate both doors ask, and this walk only names the answer in
    /// the load door's vocabulary. What is stated THERE and nowhere
    /// else is what only a FILE can be wrong about — `order` against
    /// the node map, ids the mint log does not hold and a forward input
    /// — plus the two liveness walks
    /// whose rule is the node map's own lookup. Each site says which it
    /// is.
    Snapshot,
}

impl Walk {
    /// Every walk, in the order [`validate_document`] runs them —
    /// which it runs them BY, so this is the order rather than a
    /// description of it.
    pub(crate) const ORDER: [Walk; 8] = [
        Walk::NonFinite,
        Walk::Distribution,
        Walk::DisplayUnit,
        Walk::SlotDimension,
        Walk::SlotParamRef,
        Walk::PayloadParamRef,
        Walk::Program,
        Walk::Snapshot,
    ];

    /// **This walk over one document**, or `None` when it finds
    /// nothing — the map from a walk to the function behind it and to
    /// the refusal it raises.
    ///
    /// Exhaustive with no wildcard arm, which is what makes [`Walk`]
    /// the census rather than a comment beside one: a walk added to
    /// the enum does not compile until it says what it runs, and one
    /// missing from [`Walk::ORDER`] never executes — which
    /// `tests::every_snapshot_error_arm_names_the_walk_that_produces_it`
    /// reds on.
    fn run(
        self,
        snapshot: &ProfileDoc,
        edits: &[DocEdit<ProfileProgram>],
        tol: Tol,
    ) -> Option<super::PersistError> {
        match self {
            Walk::NonFinite => first_non_finite(snapshot, edits)
                .map(|site| super::PersistError::NonFinite { site }),
            Walk::Distribution => first_distribution_fault(snapshot)
                .map(|(name, fault)| super::PersistError::Distribution { name, fault }),
            Walk::DisplayUnit => {
                first_display_unit_fault(snapshot).map(|(name, unit, declared)| {
                    super::PersistError::DisplayUnit {
                        name,
                        unit,
                        declared,
                    }
                })
            }
            Walk::SlotDimension => first_slot_fault(snapshot)
                .map(|(node, fault)| slot_refusal(snapshot.spoken(node), fault)),
            Walk::SlotParamRef => {
                first_slot_param_ref_fault(snapshot).map(|(node, slot, fault)| {
                    param_ref_refusal(snapshot.spoken(node), ParamRefAddress::Slot(slot), fault)
                })
            }
            Walk::PayloadParamRef => {
                first_payload_param_ref_fault(snapshot).map(|(node, fault)| {
                    param_ref_refusal(snapshot.spoken(node), ParamRefAddress::Payload, fault)
                })
            }
            Walk::Program => first_program_fault(snapshot, tol).map(|(node, fault)| {
                super::PersistError::ProfileProgram {
                    node: snapshot.spoken(node),
                    fault,
                }
            }),
            Walk::Snapshot => validate_snapshot(snapshot, tol)
                .err()
                .map(super::PersistError::Snapshot),
        }
    }
}

/// The shared validator (module docs): every direction-independent
/// document check, in one place, invoked by both doors. The walks it
/// runs are [`Walk`]'s variants, in [`Walk::ORDER`] — which this
/// function iterates, so the roster IS the order rather than a
/// description of it.
///
/// **The order is a CONTRACT, not an implementation detail**: a
/// document broken in two ways at once is refused by the EARLIER walk,
/// so that is the refusal every caller comparing two doors' answers
/// reads, and moving a walk changes the diagnosis of every file broken
/// both ways. `rv_onepred3_probes::rv_the_slot_walk_shadows_a_structural_refusal_it_did_not_shadow_before`
/// is the row that pins the class.
///
/// **Which adjacencies are contracts, and which are free.** A walk's
/// position is load-bearing exactly when some document is broken in
/// both its subject and its neighbour's, so that moving it re-diagnoses
/// that document. Three are, each with the row that says so:
///
/// - [`Walk::SlotDimension`] before [`Walk::SlotParamRef`] — a slot
///   expression can be retyped AND read an undeclared name, and the
///   slot's own address is the more specific answer.
/// - [`Walk::SlotParamRef`] before [`Walk::PayloadParamRef`]
///   (`load_door_payload_param_ref::a_document_broken_in_a_slot_and_in_a_payload_reads_the_slot_refusal`).
/// - Both param-ref walks before [`Walk::Snapshot`]
///   (`…::an_assertion_bound_on_a_non_measure_reads_the_payload_refusal`,
///   and `rv_onepred3_probes::rv_the_slot_walk_shadows_a_structural_refusal_it_did_not_shadow_before`
///   for the slot half): a node can carry a broken param reference AND
///   a structurally invalid shape, and the param-table answer names the
///   parameter while the structural one does not.
///
/// The slot walks before [`Walk::Program`] is a contract of the same
/// kind with a different reason: the program walk PROBES the replay, so
/// a step whose argument is an angle where the role fixes a length is
/// not a walk worth probing, and "your loop is not a legal lattice
/// walk" is the wrong sentence for it.
///
/// The rest are FREE, and no row pins them: [`Walk::NonFinite`],
/// [`Walk::Distribution`] and [`Walk::DisplayUnit`] read the param
/// table's three independent properties, and a parameter broken in two
/// of them is a file this door refuses either way. Reordering those
/// three changes which sentence such a file gets and breaks no stated
/// contract.
pub(crate) fn validate_document(
    snapshot: &ProfileDoc,
    edits: &[DocEdit<ProfileProgram>],
    tol: Tol,
) -> Result<(), super::PersistError> {
    for walk in Walk::ORDER {
        if let Some(refusal) = walk.run(snapshot, edits, tol) {
            return Err(refusal);
        }
    }
    Ok(())
}

/// The slot walk's answer, in the load door's vocabulary.
fn slot_refusal(node: SpokenNode, fault: SlotDimensionFault) -> super::PersistError {
    let SlotDimensionFault {
        slot,
        expected,
        found,
    } = fault;
    super::PersistError::Snapshot(SnapshotError::SlotDimension {
        node,
        slot,
        expected,
        found,
    })
}

/// **Where a param-ref fault was found** — the ONE thing the two
/// param-ref walks differ in, and therefore the only argument
/// [`param_ref_refusal`] needs beside the fault itself.
enum ParamRefAddress {
    /// A SLOT expression, at the slot that addresses it
    /// ([`Walk::SlotParamRef`]).
    Slot(SlotId),
    /// An expression no slot addresses ([`crate::node::payload_exprs`],
    /// [`Walk::PayloadParamRef`]): the node is the whole address.
    Payload,
}

/// **The param-ref walks' answer, in the load door's vocabulary** —
/// one mapper for both walks, because the FAULT is one vocabulary
/// ([`crate::doc::ParamRefFault`], the one predicate both doors ask)
/// and only the address differs.
///
/// The edit door has the same two destructurings of the same fault
/// (`check_param_refs` and the payload arm of `check_node_slots`) and
/// keeps them apart, because there they feed a different error type
/// with a different subject; the slot-dimension and assertion pairs
/// have that same both-doors shape. The class is filed as
/// `param-ref-refusals-spell-two-facts-four-ways` on EDIT's slate.
fn param_ref_refusal(
    node: SpokenNode,
    address: ParamRefAddress,
    fault: crate::doc::ParamRefFault,
) -> super::PersistError {
    use crate::doc::ParamRefFault;
    super::PersistError::Snapshot(match (address, fault) {
        (ParamRefAddress::Slot(slot), ParamRefFault::Unknown { name }) => {
            SnapshotError::SlotUnknownDocParam { node, slot, name }
        }
        (
            ParamRefAddress::Slot(slot),
            ParamRefFault::Dimension {
                name,
                declared,
                referenced,
            },
        ) => SnapshotError::SlotDocParamDimension {
            node,
            slot,
            name,
            declared,
            referenced,
        },
        (ParamRefAddress::Payload, ParamRefFault::Unknown { name }) => {
            SnapshotError::PayloadUnknownDocParam { node, name }
        }
        (
            ParamRefAddress::Payload,
            ParamRefFault::Dimension {
                name,
                declared,
                referenced,
            },
        ) => SnapshotError::PayloadDocParamDimension {
            node,
            name,
            declared,
            referenced,
        },
    })
}

/// The first document parameter whose authored display unit does not
/// MEASURE its declared dimension, as `(name, what the unit measures,
/// what was declared)`, or `None`.
///
/// The SNAPSHOT only, for the reason `first_distribution_fault` walks
/// it alone: a `SetDocParam` in the log carries its declaration through
/// `apply` on replay, and a replayed document is a snapshot this same
/// validator sees.
///
/// Expression literals need no twin walk — `Expr::literal_with_unit`
/// checks the pairing at construction and the load side re-runs that
/// same constructor, so a literal cannot reach a document mismatched.
/// A `DocParam` has no such door to make total: its payload is `pub`
/// and its dimension is data, which is exactly the asymmetry this walk
/// covers.
fn first_display_unit_fault(
    snapshot: &ProfileDoc,
) -> Option<(ParamName, crate::expr::Dimension, crate::expr::Dimension)> {
    snapshot.params.iter().find_map(|(name, p)| match p {
        DocParam::Continuous {
            dim, display_unit, ..
        } => {
            // The SAME reading the edit door and the literal
            // constructor make (`UnitSym::measures`): what a unit
            // measures is one fact, stated once.
            let measured = display_unit.measures();
            (measured != *dim).then(|| (name.clone(), measured, *dim))
        }
        DocParam::Count { .. } => None,
    })
}

/// The first node whose slots break spec D6's rule, by the ONE
/// predicate the edit doors ask ([`Node::slot_dimension_fault`]) — so
/// a file can carry no slot expression an edit door would have
/// refused, whatever the node kind.
///
/// EVERY node kind, which is the whole point: a walk that asked only
/// profile programs admitted a retyped extrude distance, a fillet
/// radius that counts and a dimensionless datum origin.
fn first_slot_fault(snapshot: &ProfileDoc) -> Option<(RecipeNodeId, SlotDimensionFault)> {
    snapshot
        .nodes
        .iter()
        .find_map(|(&id, node)| Some((id, node.slot_dimension_fault()?)))
}

/// The first slot expression whose document-parameter references the
/// param table cannot answer, by the ONE predicate the edit doors ask
/// ([`crate::Doc::param_ref_fault`]).
///
/// Runs after the dimension walk above, so a slot broken both ways is
/// diagnosed at its own address first. A reference that does not
/// RESOLVE is not the same class as one the table refuses: a legal
/// reference whose value fails to evaluate is V1 class 2 and passes
/// every door here, while an undeclared name and a dimension the
/// declaration contradicts are both facts about the document itself.
fn first_slot_param_ref_fault(
    snapshot: &ProfileDoc,
) -> Option<(RecipeNodeId, SlotId, crate::doc::ParamRefFault)> {
    snapshot.nodes.iter().find_map(|(&id, node)| {
        node.slots().into_iter().find_map(|slot| {
            let fault = snapshot.param_ref_fault(node.expr(slot)?)?;
            Some((id, slot, fault))
        })
    })
}

/// The first PAYLOAD expression whose document-parameter references
/// the param table cannot answer, as `(node, fault)`, by the ONE
/// predicate the edit doors ask ([`crate::Doc::param_ref_fault`]).
///
/// The expressions no slot addresses ([`crate::node::payload_exprs`]):
/// a [`crate::Node::Measure`]'s measured expression leaves and a
/// [`crate::Node::Assertion`]'s bound.
/// The address reported is the NODE, because that is the address the
/// expression has — which is why this is its own pair of refusal arms
/// rather than a wider domain for the slot walk's.
///
/// Runs after the slot param-ref walk, so a document broken in a slot
/// AND in a payload is diagnosed at the slot, which is the address
/// that carries more.
///
/// Their DIMENSIONS are not this walk's subject and are checked
/// nowhere here: a `MeasureExpr` runs the F1 checker at every
/// constructor, and an assertion's bound is checked against its
/// measure's dimension by [`Node::assertion_bound_fault`], whose
/// refusal is [`SnapshotError::AssertionBound`]. What is left for this
/// walk is the param TABLE, exactly as for a slot expression.
///
/// **The domain's edge, stated because it is not empty.** `slots()`
/// and [`crate::node::payload_exprs`] together do NOT reach every
/// `Expr` a node can hold: a `Node::Pattern`'s or `Node::PlacedUnion`'s
/// COUNT expression under an `Explicit` rule is addressed by no slot
/// (`node::rule_rows` gives a count slot only under a STEPPED rule)
/// and is no payload either. Such a file is still refused — by
/// [`Walk::Snapshot`], as `PlacementRule` with
/// `PlacementRuleFault::CountSpelling`, because a count expression and
/// an `Explicit` rule are two spellings of the count that disagree —
/// so no document reaches memory carrying an unchecked parameter
/// reference. What it does NOT get is this walk's sentence: the
/// structural refusal names neither the parameter nor the reference.
/// `rv_payloadrefs_probes::rv_an_expression_no_walk_reads_is_refused_structurally_not_as_a_param_ref`
/// is the row that pins that, and the `Snapshot`/`PlacementRule` row of
/// `work/edit/three-door-predicates-are-hand-copied-not-shared`'s table
/// is where the same fact is recorded for the slot walk.
fn first_payload_param_ref_fault(
    snapshot: &ProfileDoc,
) -> Option<(RecipeNodeId, crate::doc::ParamRefFault)> {
    snapshot.nodes.iter().find_map(|(&id, node)| {
        crate::node::payload_exprs(node)
            .into_iter()
            .flatten()
            .find_map(|expr| Some((id, snapshot.param_ref_fault(expr)?)))
    })
}

/// The first non-finite float in ε, the document params, the profile
/// nodes, the appearance records or the edit log, reported as a
/// [`NonFiniteSite`], or `None`.
///
/// NOT every float the format writes: literal placement frames and mate
/// alignments are checked by [`validate_snapshot`] and reported under
/// [`SnapshotError`], because they are structural state rather than a
/// value the writer is asked to round-trip.
fn first_non_finite(
    snapshot: &ProfileDoc,
    edits: &[DocEdit<ProfileProgram>],
) -> Option<NonFiniteSite> {
    if !snapshot.epsilon.is_finite() {
        return Some(NonFiniteSite::Epsilon);
    }
    for (name, p) in &snapshot.params {
        if let Some(site) = param_site(name, p) {
            return Some(site);
        }
    }
    // A PROFILE is no longer walked here, and the two sites it used to
    // reach are gone with it: its payload carried twelve raw
    // sketch-plane floats, and now carries a frame NODE reference. Its
    // remaining content is `Expr`s, which the expression construction
    // door already refuses non-finite, and the frame's own components
    // are `Expr`s under the same door. There is no float left for this
    // walk to find.
    for (name, rec) in &snapshot.appearance {
        if let Some((key, path)) = record_non_finite(rec) {
            return Some(NonFiniteSite::Metadata {
                name: snapshot.spoken_name(name),
                key,
                path,
            });
        }
    }
    for (index, edit) in edits.iter().enumerate() {
        if let Some(inner) = edit_non_finite(snapshot, edit) {
            return Some(NonFiniteSite::Edit {
                index,
                inner: Box::new(inner),
            });
        }
    }
    None
}

/// The parameter's non-finite float as THIS door names it: the one
/// predicate `DocParam::first_non_finite` decides, rendered as the
/// site vocabulary a document author reads. The distribution's offsets
/// belong to this walk rather than to a second spelling of the same
/// defect — the shape invariants are `first_distribution_fault`'s.
fn param_site(name: &ParamName, p: &DocParam) -> Option<NonFiniteSite> {
    Some(NonFiniteSite::DocParam {
        name: name.clone(),
        field: p.first_non_finite()?,
    })
}

/// The first document parameter whose distribution breaks an E2
/// invariant other than finiteness (`sigma > 0`; bounds containing the
/// nominal), by the SAME [`crate::Distribution::check`] the edit door
/// runs —
/// so a hand-written file with `sigma: -1` refuses at LOAD with the
/// diagnostics SAVE refuses with, and never loads best-effort.
///
/// Runs after the float walk, so a non-finite offset is reported as a
/// non-finite float rather than as a shape fault.
fn first_distribution_fault(snapshot: &ProfileDoc) -> Option<(ParamName, DistributionFault)> {
    snapshot
        .params
        .iter()
        .find_map(|(name, p)| match p.distribution()?.check() {
            Ok(()) => None,
            Err(fault) => Some((name.clone(), fault)),
        })
}

fn record_non_finite(rec: &AppearanceRecord) -> Option<(String, String)> {
    rec.metadata
        .iter()
        .find_map(|(key, value)| value.first_non_finite().map(|path| (key.clone(), path)))
}

/// The float carriers an edit can smuggle past `apply` (a saved log
/// is DATA — it has not necessarily been applied by this process),
/// a name among them spoken from `snapshot`, the document the log
/// starts from.
fn edit_non_finite(snapshot: &ProfileDoc, edit: &DocEdit<ProfileProgram>) -> Option<NonFiniteSite> {
    match edit {
        DocEdit::SetDocParam { name, value } => param_site(name, value),
        // The value door carries no distribution of its own — the
        // declaration it writes into supplies that — but its
        // continuous arm IS a raw float the format writes.
        //
        // It is the one float site here that cannot delegate to
        // `DocParam::first_non_finite`: the payload is a bare `f64`,
        // not a `DocParam`, so there is no parameter for the shared
        // predicate to read. What it shares with the walk over the
        // table is the SITE vocabulary, which is what a reader
        // comparing the two refusals sees.
        DocEdit::SetDocParamValue {
            name,
            value: crate::doc::DocParamValue::Continuous(v),
        } if !v.is_finite() => Some(NonFiniteSite::DocParam {
            name: name.clone(),
            field: DocParamField::Nominal,
        }),
        // The annotation door's whole payload is a distribution, so
        // its offsets are floats the format writes and they belong to
        // THIS walk — the same site vocabulary `SetDocParam`'s
        // declaration goes through, offending field and all.
        DocEdit::SetDocParamDistribution {
            name,
            distribution: Some(d),
        } => Some(NonFiniteSite::DocParam {
            name: name.clone(),
            field: DocParamField::Offset(d.first_non_finite()?),
        }),
        DocEdit::SetAppearanceMeta { name, key, value } => {
            value
                .first_non_finite()
                .map(|path| NonFiniteSite::Metadata {
                    name: snapshot.spoken_name(name),
                    key: key.clone(),
                    path,
                })
        }
        DocEdit::SetTolerance { eps } if !eps.is_finite() => Some(NonFiniteSite::Epsilon),
        // EXHAUSTIVE on purpose: a new `DocEdit` variant carrying a raw
        // float must be classified here or the compile breaks — a
        // wildcard would let it past the load door unchecked. The
        // guarded arms above are repeated without their guards because
        // a guarded arm does not count towards exhaustiveness.
        //
        // Why the classified variants carry nothing for this door,
        // stated per mechanism rather than in one sweeping clause:
        //
        // - Most `InsertNode` node kinds hold their floats in `Expr`
        //   literals, finite by the construction door.
        // - `Node::Mate`'s `alignment` and a placement's literal steps
        //   (an inserted transform's or gauge's, an instance's offset,
        //   `SetOffset`'s) are RAW `f64`, not `Expr`s. They are refused
        //   on replay by `apply` (`EditError::NonFiniteAlignment`,
        //   `NonFinitePlacement`), which `persist::load` runs the log
        //   through — so they are guarded, but by a door this function
        //   deliberately does not rely on for the rest of its list.
        // - The `Node` vocabulary is not closed here: this match is
        //   exhaustive on `DocEdit`, not on `Node`.
        DocEdit::SetDocParamValue { .. }
        // A notation is a table code, not a float.
        | DocEdit::SetDocParamUnit { .. }
        // The partial arm above, completed: a CLEARED annotation
        // carries no float at all.
        | DocEdit::SetDocParamDistribution { .. }
        | DocEdit::InsertNode { .. }
        // A list of node ids carries no float.
        | DocEdit::SetMembers { .. }
        // Sited names and a class tag carry no float.
        | DocEdit::SetDeclare { .. }
        // A program's continuous arguments are `Expr` literals, finite
        // by the construction door like an inserted profile's; its
        // step ids are integers.
        | DocEdit::SetProgram { .. }
        | DocEdit::SetTolerance { .. }
        | DocEdit::DeleteNode { .. }
        | DocEdit::SetParam { .. }
        | DocEdit::SetStructuralParam { .. }
        // A side is one of two words.
        | DocEdit::SetExtrudeSide { .. }
        | DocEdit::SetExpression { .. }
        | DocEdit::Rebind { .. }
        | DocEdit::ReWitness { .. }
        | DocEdit::ReWitnessBulk { .. }
        | DocEdit::SetAppearance { .. }
        | DocEdit::ClearAppearance { .. }
        | DocEdit::ClearAppearanceMeta { .. }
        | DocEdit::SetRoots { .. }
        | DocEdit::SetOffset { .. }
        // A gauge reference is a node id.
        | DocEdit::SetGauge { .. }
        // A label is text.
        | DocEdit::SetLabel { .. }
        | DocEdit::UpdateReference { .. } => None,
    }
}

/// A structural invariant violation in a parsed snapshot (load door).
///
/// Every node and name it holds is spoken from the document being
/// judged, when the refusal is raised ([`crate::Doc::spoken`],
/// [`crate::Doc::spoken_name`]): the save door's document, or the load
/// door's parsed one, whose labels `Label`'s `Deserialize` has already
/// admitted. An id that document does not hold reads as
/// [`SpokenNode::absent`]; a machine channel reads the id off
/// [`SpokenNode::id`].
///
/// Its four document-parameter-reference arms are named under the
/// convention stated once on [`crate::EditError`], whose own four are
/// the same four names.
#[derive(Debug, Clone, PartialEq)]
pub enum SnapshotError {
    /// `order` and the node map disagree (missing, extra, or
    /// duplicated ids).
    OrderMismatch,
    /// A node id the mint log does not hold as a node's appears in
    /// the document — one the document never minted.
    NodeNotMinted {
        /// The offending id.
        id: SpokenNode,
    },
    /// A profile's step ids are not the ones its edit doors would have
    /// minted (`names/README.md`, "N1, the profile pieces"): not one
    /// per authored step, not in the mint log, or one id standing for
    /// two steps anywhere in the document.
    StepIds {
        /// The profile node.
        node: SpokenNode,
        /// What is wrong.
        fault: crate::program::StepIdFault,
    },
    /// The mint's log is not strictly ascending by id: an id logged
    /// twice, or out of order — a log no mint wrote.
    MintLogOrder {
        /// The first entry not greater than the one before it.
        entry: crate::Minted,
    },
    /// A name the document holds spells a profile step its mint log
    /// does not hold — one the document never minted.
    NameStepNotMinted {
        /// The name.
        name: SpokenName,
        /// The step it spells.
        step: crate::node::StepId,
    },
    /// A Boolean's declared pair is read at a node that is not one of
    /// its operands — the edit doors' [`crate::EditError::DeclaredSiteNotAnOperand`],
    /// by the same rule (`node::declared_side_fault`). Asked of a
    /// Boolean only: its operands never change, so no edit leaves it
    /// such a pair. A union's member can be dropped by `SetMembers`
    /// after its pair was written, which is N5's stranded state and
    /// loads; the evaluation refuses it as a vanished name
    /// ([`crate::eval::NodeErrorKind::DeclareResolve`]).
    DeclaredSiteNotAnOperand {
        /// The Boolean.
        node: SpokenNode,
        /// The side's name.
        name: SpokenName,
        /// The node the side is read at.
        site: SpokenNode,
    },
    /// A Boolean's or a Union's declared name is minted by the node
    /// itself or by a live node after it in `order` — the edit doors'
    /// [`crate::EditError::DeclaredNameNotUpstream`], by the same rule.
    /// No edit leaves a document so: the doors refuse it when the pair
    /// is written, and `order` only grows at its end.
    DeclaredNameNotUpstream {
        /// The declaring node.
        node: SpokenNode,
        /// The name.
        name: SpokenName,
    },
    /// A node's input ref does not name a live node.
    DanglingInput {
        /// The referring node.
        node: SpokenNode,
        /// The missing input.
        input: SpokenNode,
    },
    /// A node's input ref does not precede it in `order` (insertion
    /// order is topological by construction — a forward ref means a
    /// tampered file, and possibly a cycle).
    ForwardInput {
        /// The referring node.
        node: SpokenNode,
        /// The forward input.
        input: SpokenNode,
    },
    /// A witness attached to a node that bears no sketch. Its own arm
    /// rather than [`SnapshotError::WitnessOnMissingNode`]: the edit
    /// door names the two apart, and the repairs differ — one moves
    /// the witness, the other has no node to move it to.
    WitnessSite {
        /// The offending node id.
        node: SpokenNode,
    },
    /// A witness attached to a node id that names nothing live.
    WitnessOnMissingNode {
        /// The offending node id.
        node: SpokenNode,
    },
    /// A label attached to a node id that names nothing live — the
    /// state `DeleteNode`, which drops the label, never leaves.
    LabelOnMissingNode {
        /// The offending node id.
        node: SpokenNode,
    },
    /// The recorded ε is not finite and strictly positive.
    EpsilonInvalid {
        /// The recorded value.
        value: f64,
    },
    /// The product-root list violates an A10 invariant (ASM-ROOTS
    /// D-2): the same check `apply` runs, so a file can carry no root
    /// state the edit doors could not have produced.
    Roots(crate::roots::RootFault),
    /// A gauge reference — an instance's gauge or a gauge's parent —
    /// that names a live node that is not a gauge. The edit doors
    /// refuse it through the same predicate (`doc::gauge_ref_fault`).
    NotAGauge {
        /// The instance or gauge holding the reference.
        node: SpokenNode,
        /// The node it names.
        gauge: SpokenNode,
    },
    /// A gauge reference that closes a loop: a gauge sitting on
    /// itself through its own chain.
    GaugeCycle {
        /// The gauge holding the reference.
        node: SpokenNode,
        /// The gauge it names, which sits on `node`.
        gauge: SpokenNode,
    },
    /// A placement frame carrying a non-finite coordinate — a literal
    /// step of a transform's or a gauge's placement, or of an
    /// instance's offset. The edit door refuses it, so a file holding
    /// one is corrupt — refused, never repaired.
    PlacementNonFinite {
        /// The node holding the placement.
        node: SpokenNode,
        /// Which of its frames.
        at: FrameSite,
    },
    /// An IMPROPER placement frame — determinant ≤ 0, the A6 mirror
    /// case R4 gates. Its own arm rather than
    /// [`SnapshotError::PlacementNonFinite`]: a mirror is authored data
    /// this build declines to admit, a non-finite coordinate is data no
    /// predicate can read, and the repairs differ.
    PlacementImproper {
        /// The node holding the placement.
        node: SpokenNode,
        /// Which of its frames.
        at: FrameSite,
        /// The linear part's determinant.
        determinant: f64,
    },
    /// A proper placement frame that is not definitely a rigid motion
    /// at tolerance. The edit door refuses it by the predicate the
    /// evaluation moves a body by.
    PlacementNonRigid {
        /// The node holding the placement.
        node: SpokenNode,
        /// Which of its frames.
        at: FrameSite,
        /// The rigidity check that refused; routing, never rendered by
        /// name.
        check: &'static str,
    },
    /// A mate's alignment datum carries a non-finite coordinate. The
    /// edit door refuses it, so a file holding one is corrupt: no
    /// predicate could decide anything about it.
    MateAlignment {
        /// The offending mate.
        node: SpokenNode,
    },
    /// A placement-rule node carrying a rule the edit door would have
    /// refused (GROUP-BOOLEAN-DESIGN): the count spelled two ways, an
    /// empty explicit list, or a non-finite / improper frame.
    PlacementRule {
        /// The offending node.
        node: SpokenNode,
        /// What is wrong with it.
        fault: crate::node::PlacementRuleFault,
    },
    /// A node whose SLOT expression is of another dimension than the
    /// slot address fixes (spec D6, [`crate::SlotId::dimension`]), for
    /// any node kind — a profile step's argument, an extrude's
    /// distance, a datum's coordinate. The edit doors refuse it
    /// through the same predicate (`Node::slot_dimension_fault`), so a
    /// file carrying one is data the edit doors could not have
    /// produced.
    SlotDimension {
        /// The offending node.
        node: SpokenNode,
        /// The offending slot.
        slot: SlotId,
        /// The dimension the address fixes.
        expected: crate::expr::Dimension,
        /// The expression's dimension.
        found: crate::expr::Dimension,
    },
    /// A node whose SLOT expression reads a document parameter the
    /// document does not declare. The edit door refuses it through the
    /// same predicate (`Doc::param_ref_fault`), and re-asks it of
    /// every slot whenever a declaration lands, so a file carrying one
    /// is data the edit doors could not have produced.
    SlotUnknownDocParam {
        /// The offending node.
        node: SpokenNode,
        /// The slot whose expression reads it.
        slot: SlotId,
        /// The name it reads.
        name: ParamName,
    },
    /// A node whose SLOT expression reads a declared parameter at
    /// another dimension than it was declared with — the pairing a
    /// (re)declaration can break, refused rather than resolved to
    /// whichever of the two the reader happens to trust.
    SlotDocParamDimension {
        /// The offending node.
        node: SpokenNode,
        /// The slot whose expression reads it.
        slot: SlotId,
        /// The name it reads.
        name: ParamName,
        /// The dimension the declaration carries.
        declared: crate::expr::Dimension,
        /// The dimension the expression reads it at.
        referenced: crate::expr::Dimension,
    },
    /// A node whose PAYLOAD expression — a measured expression's value
    /// leaf or an assertion's bound, the expressions no slot addresses
    /// ([`crate::node::payload_exprs`]) — reads a document parameter
    /// the document does not declare. The address is the NODE: there
    /// is no slot to name. The edit door refuses it through the same
    /// predicate (`Doc::param_ref_fault`), so a file carrying one is
    /// data the edit doors could not have produced.
    PayloadUnknownDocParam {
        /// The offending node.
        node: SpokenNode,
        /// The name its payload reads.
        name: ParamName,
    },
    /// A node whose PAYLOAD expression reads a declared parameter at
    /// another dimension than it was declared with — the pairing a
    /// (re)declaration can break, at the address no slot names.
    PayloadDocParamDimension {
        /// The offending node.
        node: SpokenNode,
        /// The name its payload reads.
        name: ParamName,
        /// The dimension the declaration carries.
        declared: crate::expr::Dimension,
        /// The dimension the expression reads it at.
        referenced: crate::expr::Dimension,
    },
    /// A measure node whose expression reads a reference the node does
    /// not carry (E3). The expression indexes the reference list
    /// positionally, so this is a corrupt file, not a stale reference:
    /// `Rebind` cannot repair an index.
    MeasureRefs {
        /// The offending node.
        node: SpokenNode,
        /// What is wrong with it.
        fault: crate::node::MeasureNodeFault,
    },
    /// A node whose structural content is invalid (DM5): inputs that
    /// are not pairwise distinct, a LIST input holding fewer than two
    /// entries, or a name designation outside the canonical form its
    /// construction door establishes — a shell's repeated `open` entry,
    /// a blend's unsorted or repeating `selection`. Both edit doors
    /// refuse them, so a file carrying one is corrupt — refused,
    /// never repaired.
    InputList {
        /// The offending node.
        node: SpokenNode,
        /// What is wrong with it. A repeated input is
        /// [`SnapshotError::DuplicateInput`], not a list fault.
        fault: crate::node::ListFault,
    },
    /// A node reaching one input twice (DM5) — [`Node::input_fault`]'s
    /// `Duplicate` answer, named apart from
    /// [`SnapshotError::InputList`] as the edit door names it
    /// (`EditError::DuplicateInput`), because the input it repeats is a
    /// node this door speaks.
    DuplicateInput {
        /// The node whose input list repeats.
        node: SpokenNode,
        /// The input it reaches twice.
        input: SpokenNode,
    },
    /// An assertion whose reference is not a measure at all (E10):
    /// there is no measured dimension for the bound to agree with. The
    /// edit door refuses it, so a file carrying one is data the edit
    /// door would never have produced.
    AssertionTarget {
        /// The offending assertion.
        node: SpokenNode,
        /// What it references.
        measure: SpokenNode,
        /// The bound's dimension.
        bound: crate::expr::Dimension,
    },
    /// An assertion whose bound is dimensioned differently from the
    /// measure it constrains (E10) — the assertion compares two
    /// different quantities. Its own arm rather than
    /// [`SnapshotError::AssertionTarget`]: a reader should not have to
    /// decode an absent dimension to tell a missing measure from a
    /// mismatched one, and the repairs differ.
    AssertionBound {
        /// The offending assertion.
        node: SpokenNode,
        /// The measure it constrains.
        measure: SpokenNode,
        /// What that measure yields.
        measured: crate::expr::Dimension,
        /// The bound's dimension.
        bound: crate::expr::Dimension,
    },
    /// An appearance metadata value violating the D7 producer
    /// convention (map with an integer `"v"`).
    MetadataUnversioned {
        /// The attributed name.
        name: SpokenName,
        /// The metadata key.
        key: String,
        /// The typed shape refusal.
        error: MetaVersionError,
    },
}

impl SnapshotError {
    /// The arm a frame the admission rule refused is reported under,
    /// at `at` on `node`.
    fn placement_frame(node: SpokenNode, at: FrameSite, fault: FrameFault) -> Self {
        match fault {
            FrameFault::NonFinite => Self::PlacementNonFinite { node, at },
            FrameFault::Improper { determinant } => Self::PlacementImproper {
                node,
                at,
                determinant,
            },
            FrameFault::NotRigid { check } => Self::PlacementNonRigid { node, at, check },
        }
    }
}

/// **A placement frame's refusal at load**: the frame's subject at its
/// site and the frame rule's own clause, then what a user holding the
/// file can do. The edit doors never admitted a non-finite or a
/// mirrored frame, so a file holding one was not written by them; a
/// frame that is not rigid was admitted by earlier builds at the
/// registry and in an explicit rule, and a current build refuses it at
/// the edit door with that frame's own recourse.
fn frame_refusal(
    f: &mut core::fmt::Formatter<'_>,
    node: &SpokenNode,
    at: FrameSite,
    fault: FrameFault,
) -> core::fmt::Result {
    write!(f, "{} {fault}. ", at.subject(node))?;
    match fault {
        FrameFault::NonFinite | FrameFault::Improper { .. } => {
            f.write_str(geom_core::KERNEL_OR_FILE_DEFECT_ENDING)
        }
        FrameFault::NotRigid { .. } => {
            write!(
                f,
                "{}",
                crate::sentence::Recourse(super::REGENERATE_RECOURSE)
            )
        }
    }
}

// The document layer's prose for a corrupt snapshot: each arm states
// WHAT is wrong and WHERE, and forwards the payload's own `Display`
// wherever the payload has one (`RootFault`, `PlacementRuleFault`,
// `MetaVersionError`) — a site that re-states a payload it holds
// invents a second vocabulary for a refusal that already has one. A
// node or a name renders through its spoken `Display` (`SpokenNode`,
// `SpokenName`), never a hand-rolled respelling.
impl core::fmt::Display for SnapshotError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::OrderMismatch => f.write_str(
                "the `order` list and the node map disagree — an id is missing, extra or \
                 duplicated",
            ),
            Self::NodeNotMinted { id } => write!(
                f,
                "{id} is not in the document's mint log — the document never minted it",
            ),
            Self::StepIds { node, fault } => write!(f, "{node}'s step ids: {fault}"),
            Self::MintLogOrder { entry } => write!(
                f,
                "the mint's log is not strictly ascending at {entry} — an id logged twice or out \
                 of order, which no mint writes. {}",
                geom_core::KERNEL_OR_FILE_DEFECT_ENDING
            ),
            Self::NameStepNotMinted { name, step } => write!(
                f,
                "the {name} spells the profile step id {step}, which the document's mint log \
                 does not hold — the document never minted it",
            ),
            Self::DeclaredSiteNotAnOperand { node, name, site } => write!(
                f,
                "the declared {name} is read at {site}, which is not an operand of {node} — no \
                 edit writes such a pair. {}",
                geom_core::KERNEL_OR_FILE_DEFECT_ENDING
            ),
            Self::DeclaredNameNotUpstream { node, name } => write!(
                f,
                "the declared {name} is not minted before {node} in `order` — no edit writes \
                 such a pair. {}",
                geom_core::KERNEL_OR_FILE_DEFECT_ENDING
            ),
            Self::DanglingInput { node, input } => {
                write!(f, "{node} takes input from {input}, which is not live")
            }
            Self::ForwardInput { node, input } => write!(
                f,
                "{node} takes input from {input}, which does not precede it in `order`"
            ),
            Self::DuplicateInput { node, input } => {
                write!(f, "{node}: ")?;
                crate::node::duplicate_input(f, input)
            }
            Self::WitnessSite { node } => {
                write!(f, "a witness is attached to {node}, which bears no sketch")
            }
            Self::WitnessOnMissingNode { node } => {
                write!(f, "a witness is attached to {node}, which is not live")
            }
            Self::LabelOnMissingNode { node } => {
                write!(f, "a label is attached to {node}, which is not live")
            }
            Self::EpsilonInvalid { value } => write!(
                f,
                "the recorded ε {value:e} is not finite and strictly positive"
            ),
            Self::Roots(fault) => write!(f, "{fault}"),
            Self::NotAGauge { node, gauge } => write!(
                f,
                "{node}'s gauge reference names {gauge}, which is not a gauge"
            ),
            Self::GaugeCycle { node, gauge } => {
                write!(f, "{node} sits on {gauge}, which sits on it")
            }
            // The frame clause is the frame rule's own
            // (`crate::placement::FrameFault`); these arms supply only
            // the subject, so a reader sees one sentence about a frame
            // wherever a frame was refused.
            Self::PlacementNonFinite { node, at } => {
                frame_refusal(f, node, *at, FrameFault::NonFinite)
            }
            Self::PlacementImproper {
                node,
                at,
                determinant,
            } => frame_refusal(
                f,
                node,
                *at,
                FrameFault::Improper {
                    determinant: *determinant,
                },
            ),
            Self::PlacementNonRigid { node, at, check } => {
                frame_refusal(f, node, *at, FrameFault::NotRigid { check })
            }
            Self::MateAlignment { node } => write!(
                f,
                "{node}'s alignment datum carries a non-finite coordinate"
            ),
            Self::PlacementRule { node, fault } => write!(f, "{node}: {fault}"),
            // The rule's own clause (`SlotDimensionFault`), forwarded
            // into this door's subject. Every slot address alike,
            // including a program step's: `SlotId::label` is where an
            // address is put into words, so a reader who sees
            // "loop 0 step 2 · centre x" from the edit door sees the
            // same address here.
            Self::SlotDimension {
                node,
                slot,
                expected,
                found,
            } => write!(
                f,
                "{node}: {}",
                crate::node::SlotDimensionFault {
                    slot: *slot,
                    expected: *expected,
                    found: *found
                }
            ),
            Self::SlotUnknownDocParam { node, slot, name } => write!(
                f,
                "{node}: slot {} reads the parameter {name}, which the document does not declare",
                slot.label()
            ),
            Self::SlotDocParamDimension {
                node,
                slot,
                name,
                declared,
                referenced,
            } => write!(
                f,
                "{node}: slot {} reads the parameter {name} as {} {referenced}, and it is \
                 declared {declared}",
                slot.label(),
                referenced.article()
            ),
            Self::PayloadUnknownDocParam { node, name } => write!(
                f,
                "{node}: its payload expression reads the parameter {name}, which the document \
                 does not declare",
            ),
            Self::PayloadDocParamDimension {
                node,
                name,
                declared,
                referenced,
            } => write!(
                f,
                "{node}: its payload expression reads the parameter {name} as {} \
                 {referenced}, and it is declared {declared}",
                referenced.article()
            ),
            Self::MeasureRefs { node, fault } => write!(f, "{node}: {fault}"),
            Self::InputList { node, fault } => write!(f, "{node}: {fault}"),
            Self::AssertionBound {
                node,
                measure,
                measured,
                bound,
            } => write!(
                f,
                "{node} bounds {measure}, which measures {} {measured}, with {} {bound} \
                 expression",
                measured.article(),
                bound.article()
            ),
            Self::AssertionTarget {
                node,
                measure,
                bound,
            } => write!(
                f,
                "{node} carries {} {bound} bound against {measure}, which is not a measure",
                bound.article(),
            ),
            Self::MetadataUnversioned { name, key, error } => write!(
                f,
                "metadata {key:?} on the {name} does not carry an integer \
                 \"v\" version field: {error}"
            ),
        }
    }
}

/// Re-checks the document invariants `apply` maintains — on a parsed
/// snapshot (load) and on the in-memory snapshot (save) alike.
fn validate_snapshot(doc: &ProfileDoc, tol: Tol) -> Result<(), SnapshotError> {
    // order ↔ nodes agreement (and no duplicates: equal lengths plus
    // every order id resolving implies a bijection on a BTreeMap).
    let mut position = std::collections::BTreeMap::new();
    for (i, &id) in doc.order.iter().enumerate() {
        if !doc.nodes.contains_key(&id) || position.insert(id, i).is_some() {
            return Err(SnapshotError::OrderMismatch);
        }
    }
    if position.len() != doc.nodes.len() {
        return Err(SnapshotError::OrderMismatch);
    }
    // The recorded ε, by the same `doc::epsilon_admissible` the edit
    // door asks before it records one.
    if !crate::doc::epsilon_admissible(doc.epsilon) {
        return Err(SnapshotError::EpsilonInvalid { value: doc.epsilon });
    }
    // Every id in the document is one the document has minted — replay
    // after load must never re-mint a referenced id.
    let check_id = |id: RecipeNodeId| -> Result<(), SnapshotError> {
        if !doc.has_minted(id) {
            Err(SnapshotError::NodeNotMinted { id: doc.spoken(id) })
        } else {
            Ok(())
        }
    };
    // The mint log first, since every check below asks it: strictly
    // ascending, the only log a mint writes.
    if let Some(entry) = doc.mint.out_of_order() {
        return Err(SnapshotError::MintLogOrder { entry });
    }
    // Every profile's step ids: one per authored step, each one the
    // mint log holds, and no id standing for two steps anywhere in the
    // document — the three things the edit doors' minting makes true
    // (N1).
    let mut seen_steps = std::collections::BTreeSet::new();
    for (&id, node) in &doc.nodes {
        let Node::Profile(program) = node else {
            continue;
        };
        let fault = |fault| SnapshotError::StepIds {
            node: doc.spoken(id),
            fault,
        };
        program.check_id_shape().map_err(fault)?;
        for ids in &program.ids {
            for &step in ids {
                if !doc.mint.has_step(step) {
                    return Err(fault(crate::program::StepIdFault::NotMinted { step }));
                }
                if !seen_steps.insert(step) {
                    return Err(fault(crate::program::StepIdFault::Repeated { step }));
                }
            }
        }
    }
    for (&id, node) in &doc.nodes {
        check_id(id)?;
        for input in node.inputs() {
            check_id(input)?;
            // The edit doors' liveness test (`EditError::UnresolvedInput`)
            // restated, and irreducibly so: the rule IS the node map's
            // own lookup, so there is no predicate between the two
            // sites to give a home to — only the same `contains_key`,
            // asked of a different subject. The edit doors ask it of
            // ONE incoming reference before it becomes an edge; this
            // walk asks it of every edge a file already claims.
            if !doc.nodes.contains_key(&input) {
                return Err(SnapshotError::DanglingInput {
                    node: doc.spoken(id),
                    input: doc.spoken(input),
                });
            }
            if position.get(&input) >= position.get(&id) {
                return Err(SnapshotError::ForwardInput {
                    node: doc.spoken(id),
                    input: doc.spoken(input),
                });
            }
        }
        // Every node a reference is READ AT that is not also an
        // input (`Node::payload_read_sites` — a mate's two operands):
        // an id the mint log does not hold inside an operand is as corrupt as
        // one inside the name beside it, and as unrepairable.
        for at in node.payload_read_sites() {
            check_id(at)?;
        }
        // The gauge reference (A11 (2)), by the predicate the edit
        // doors ask. A reference to a DELETED gauge is legal state —
        // A11 (2) keeps it so the unplaced group names its cause — and
        // loads; one past the counter, to a live non-gauge, or round a
        // loop is a file no door could have written.
        if let Some(gauge) = node.gauge_ref() {
            check_id(gauge)?;
            match crate::doc::gauge_ref_fault(doc, id, Some(gauge)) {
                None | Some(GaugeRefFault::Deleted) => {}
                // `check_id` above refused an id past the counter.
                Some(GaugeRefFault::NeverMinted) => {
                    unreachable!("node {}'s gauge id was checked minted above", id)
                }
                Some(GaugeRefFault::NotAGauge) => {
                    return Err(SnapshotError::NotAGauge {
                        node: doc.spoken(id),
                        gauge: doc.spoken(gauge),
                    });
                }
                Some(GaugeRefFault::Cycle) => {
                    return Err(SnapshotError::GaugeCycle {
                        node: doc.spoken(id),
                        gauge: doc.spoken(gauge),
                    });
                }
            }
        }
        // The placement RULE (GROUP-BOOLEAN-DESIGN), re-checked because
        // a saved file is DATA,
        // and every rule on the wire must be one the edit door would
        // have accepted — one spelling of the count, at least one
        // placement, and frames that are finite and proper.
        if let Some(fault) = node.placement_rule_fault(tol) {
            return Err(SnapshotError::PlacementRule {
                node: doc.spoken(id),
                fault,
            });
        }
        // A placement's literal frames — a transform's, a gauge's, an
        // instance's offset — held by the predicate the edit door asks,
        // for the rule's reason: the snapshot is the one road
        // to a document that does not pass `apply`.
        if let Some((index, fault)) = node.placement_frame_fault(tol) {
            return Err(SnapshotError::placement_frame(
                doc.spoken(id),
                FrameSite::Step { index },
                fault,
            ));
        }
        // DM5's third caller, for the reason the placement rule above
        // has one: a saved file is DATA, and a SNAPSHOT is the one way
        // a node reaches a document without passing `apply`. The edit
        // log replays through the doors and is covered by them; the
        // snapshot beside it is not, so the rule is asked here, of the
        // same function, in this door's vocabulary.
        //
        // That covers the name designations too — a shell's `open`, a
        // blend's `selection`. A payload has ONE canonical form, held by
        // every door that admits a node, so the question is asked in one
        // place and this door only names the answer.
        if let Some(fault) = node.input_fault() {
            return Err(match fault.list_fault() {
                Err(input) => SnapshotError::DuplicateInput {
                    node: doc.spoken(id),
                    input: doc.spoken(input),
                },
                Ok(fault) => SnapshotError::InputList {
                    node: doc.spoken(id),
                    fault,
                },
            });
        }
        // The measurement vocabulary's two structural re-checks, for
        // the same reason the placement rule has one: a saved file is
        // DATA, and both of these are refused at the edit door.
        if let Some(fault) = node.measure_fault() {
            return Err(SnapshotError::MeasureRefs {
                node: doc.spoken(id),
                fault,
            });
        }
        // An assertion's bound against the measure it constrains
        // (E10), by the same `Node::assertion_bound_fault` the edit
        // door asks: the predicate needs the DOCUMENT, so it takes one,
        // and this door only names its two answers.
        if let Some(fault) = node.assertion_bound_fault(doc) {
            return Err(match fault {
                AssertionBoundFault::TargetNotMeasure { measure, bound } => {
                    SnapshotError::AssertionTarget {
                        node: doc.spoken(id),
                        measure: doc.spoken(measure),
                        bound,
                    }
                }
                AssertionBoundFault::DimensionMismatch {
                    measure,
                    measured,
                    bound,
                } => SnapshotError::AssertionBound {
                    node: doc.spoken(id),
                    measure: doc.spoken(measure),
                    measured,
                    bound,
                },
            });
        }
        // ASM-R2a D-1: a mate's alignment is authored numbers a
        // predicate must be able to decide on. Asked in THIS walk, of
        // the same `Node::has_non_finite_alignment` the edit door asks
        // — a second pass over the nodes would be a second place to
        // forget the question. A `FromFace` side is checked
        // STRUCTURALLY and no further — a face by type, its one key
        // closed at the wire — because its numbers are the part's:
        // whether the name is a row of the part's table is the
        // solve's at evaluation (`MateFault::FaceUnresolved`), never
        // this door's.
        //
        // Nor is its PART-LOCAL spelling checked here, and the gap is
        // real: a face frame spelled as a head is (qualified by an
        // `InPart` under the instance) loads from a snapshot and
        // faults `NoSuchName` at evaluation, where the insert door
        // refuses the same frame at once — the door resolves it, this
        // one cannot. The spelling is not decidable off the bytes: a
        // part that is itself an assembly carries `InPart` rows in its
        // own table, so a qualified name can be exactly the part's own
        // row, and only the part's product says which.
        if node.has_non_finite_alignment() {
            return Err(SnapshotError::MateAlignment {
                node: doc.spoken(id),
            });
        }
    }
    // Every `StableName` the document holds, in ONE pass over the
    // carrier enumeration rather than a payload walk inside the node
    // loop above and a store walk down here, hundreds of lines apart
    // and neither reading as half of one list. An id the mint log lacks
    // inside a mate head, a fillet selection or an appearance key is
    // as corrupt as one inside a declared pair, and as unrepairable
    // by `Rebind` (whose source door refuses a never-minted id) if it
    // loads. A carrier added to `Carrier` is checked here without
    // being remembered into this door.
    for carrier in doc.name_carriers() {
        for n in derivation_nodes(carrier.name()) {
            check_id(n)?;
        }
        // And every profile step it spells, against the mint log: a
        // step a `SetProgram` dropped stays in it.
        if let Some(&step) = carrier
            .name()
            .piece_steps()
            .iter()
            .find(|s| !doc.mint.has_step(**s))
        {
            return Err(SnapshotError::NameStepNotMinted {
                name: doc.spoken_name(carrier.name()),
                step,
            });
        }
    }
    // Every declared pair, by the rule its edit doors ask
    // (`node::declared_side_fault`): a name minted before its node, and
    // — for a Boolean, whose operands never change — a site that is one
    // of its operands. A union's sites are not judged here: a later
    // `SetMembers` may have stranded one, which loads.
    for (index, &id) in doc.order.iter().enumerate() {
        let Some(node) = doc.nodes.get(&id) else {
            continue;
        };
        let operands = node.inputs();
        let sites = matches!(node, Node::Boolean { .. }).then_some(operands.as_slice());
        match crate::node::declared_side_fault(node.declared_pairs(), sites, Some(index), |n| {
            position.get(&n).copied()
        }) {
            None => {}
            Some((side, crate::node::DeclaredSideFault::SiteNotAnOperand)) => {
                return Err(SnapshotError::DeclaredSiteNotAnOperand {
                    node: doc.spoken(id),
                    name: doc.spoken_name(&side.name),
                    site: doc.spoken(side.at),
                });
            }
            Some((side, crate::node::DeclaredSideFault::NameNotUpstream)) => {
                return Err(SnapshotError::DeclaredNameNotUpstream {
                    node: doc.spoken(id),
                    name: doc.spoken_name(&side.name),
                });
            }
        }
    }
    // The witness store's key rule, by the same
    // `doc::witness_site_fault` the witness edit doors ask: a witness
    // is attached to a live node that bears a sketch. This door names
    // the two answers apart, as the edit door does, because a key that
    // names nothing and a key that names the wrong kind of node are
    // repaired differently.
    for &node in doc.witnesses.keys() {
        check_id(node)?;
        if let Some(fault) = crate::doc::witness_site_fault(doc, node) {
            return Err(match fault {
                WitnessSiteFault::NoSuchNode => SnapshotError::WitnessOnMissingNode {
                    node: doc.spoken(node),
                },
                WitnessSiteFault::NotSketchBearing => SnapshotError::WitnessSite {
                    node: doc.spoken(node),
                },
            });
        }
    }
    // The label store's key rule: every key names a live node.
    for &node in doc.labels.keys() {
        check_id(node)?;
        if !doc.nodes.contains_key(&node) {
            return Err(SnapshotError::LabelOnMissingNode {
                node: doc.spoken(node),
            });
        }
    }
    // The A10 root invariants (ASM-ROOTS D-2), run AFTER the node
    // walk so a file with dangling inputs is diagnosed as such rather
    // than as an incidental coverage failure.
    crate::roots::check(doc, |id| doc.spoken(id)).map_err(SnapshotError::Roots)?;
    // D7's producer convention, asked of the whole map. The RULE is
    // already shared — `MetaValue::require_versioned` is the one
    // predicate, and `SetAppearanceMeta` calls it too — and what is not
    // shared is the walk, irreducibly: the edit door holds the one
    // value it is about to write, and this door holds a map that
    // arrived whole. A walk over one value is not a walk.
    for (name, rec) in &doc.appearance {
        for (key, value) in &rec.metadata {
            if let Err(error) = value.require_versioned() {
                return Err(SnapshotError::MetadataUnversioned {
                    name: doc.spoken_name(name),
                    key: key.clone(),
                    error,
                });
            }
        }
    }
    Ok(())
}

/// A profile PROGRAM structure fault (the retired stored-joint walk's
/// successor at the program layer): a lattice-violating step order.
/// It is the corrupt-file class — the payload is `pub`, so an in-crate
/// bug can also build one; both doors refuse with the same
/// diagnostics.
///
/// A wrong-dimension ARGUMENT is not here. A program slot is a slot
/// like any other, so it is decided by the document-wide slot walk
/// ([`SnapshotError::SlotDimension`], `Node::slot_dimension_fault`)
/// that the edit doors ask too, rather than by a second spelling that
/// reached profile nodes alone.
#[derive(Debug, Clone, PartialEq)]
pub enum ProgramFault {
    /// The program is not a legal lattice walk (LIB-SWITCH §4h: the
    /// replay PROBE under the document's params refused with the
    /// Transition class — no authoring surface can record this).
    /// Geometry refusals, resolve failures and validate refusals
    /// ([`crate::ProgramRefusal::Validate`] — a bowtie loop, even one
    /// drawn in literals the insert door refuses) deliberately PASS
    /// this door: they are V1 class 2, legal at rest, surfaced as
    /// typed node errors at evaluation (the validate class as
    /// [`crate::NodeErrorKind::Profile`]).
    Lattice {
        /// The offending loop.
        loop_: u32,
        /// The offending step (one past the end for an unclosed
        /// chain).
        step: u32,
        /// The tip's lattice state.
        state: profile::TipState,
        /// The ill-typed verb (`None` for end-of-program).
        verb: Option<profile::Verb>,
    },
}

// The prose the document layer renders for a program fault. The
// lattice arm states the walk failure in the same words
// [`crate::ProgramRefusal::Transition`] does — that refusal is what
// the probe raised — and then names the tip state and the verb that
// could not follow it, keeping their `Debug` spellings for the reason
// `profile`'s `ReplayError` rendering states: the pair is the
// transition table's coordinate. The typed variant remains the
// machine contract.
impl core::fmt::Display for ProgramFault {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Lattice {
                loop_,
                step,
                state,
                verb,
            } => {
                write!(
                    f,
                    "loop {loop_} step {step} is not a legal chain-lattice walk: "
                )?;
                match verb {
                    Some(verb) => write!(f, "the {verb:?} verb at tip state {state:?}"),
                    None => write!(f, "the chain is unclosed at tip state {state:?}"),
                }
            }
        }
    }
}

/// The first program fault in the SNAPSHOT's profile nodes (module
/// docs). The edit log needs no twin: logged edits replay through
/// `apply`, whose own doors (the shared slot rule + the VQ9
/// authoring-time check) refuse the same faults at the same load.
///
/// A program whose Exprs fail to RESOLVE under the document's params
/// (dangling ref, dimension drift) cannot be probed here and PASSES
/// this walk — deliberately: resolution failures are the same
/// binding-dependent class as geometry refusals (V1 class 2) and
/// surface as the node's typed evaluation error; no silent acceptance
/// exists (review NOTE-1).
fn first_program_fault(snapshot: &ProfileDoc, tol: Tol) -> Option<(RecipeNodeId, ProgramFault)> {
    let env = snapshot.param_env::<f64>();
    for (&id, node) in &snapshot.nodes {
        let Node::Profile(program) = node else {
            continue;
        };
        if let Err(ProgramRefusal::Transition {
            loop_,
            step,
            state,
            verb,
        }) = program.check(&env, tol)
        {
            return Some((
                id,
                ProgramFault::Lattice {
                    loop_,
                    step,
                    state,
                    verb,
                },
            ));
        }
    }
    None
}

#[cfg(test)]
mod tests {
    #![allow(clippy::panic, clippy::expect_used)]

    use super::Walk;
    use crate::doc::ParamName;
    use crate::expr::Dimension;
    use crate::node::{Node, RecipeNodeId, SlotId};
    use crate::persist::{PersistError, SnapshotError, save};
    use crate::placement::FrameSite;
    use crate::program::ProfileDoc;
    use geom_core::Tol;

    test_utils::f6_variants! {
        /// **`validate_document`'s walks**, welded to [`Walk`] by the
        /// match the macro writes: a walk added to the enum leaves it
        /// non-exhaustive, and the census below compares this roster
        /// against [`WALKS_IN_CALL_ORDER`] in both directions.
        const WALK: Walk = [
            NonFinite,
            Distribution,
            DisplayUnit,
            SlotDimension,
            SlotParamRef,
            PayloadParamRef,
            Program,
            Snapshot,
        ];
    }

    /// Whether a walk's refusal is a [`SnapshotError`] — the rest
    /// raise a [`PersistError`](crate::PersistError) arm of their own.
    ///
    /// Exhaustive with no wildcard arm, for the same reason
    /// [`Walk::run`] is: a walk added to the enum is placed here too,
    /// or this does not compile.
    const fn raises_snapshot_error(walk: Walk) -> bool {
        match walk {
            Walk::NonFinite | Walk::Distribution | Walk::DisplayUnit | Walk::Program => false,
            Walk::SlotDimension | Walk::SlotParamRef | Walk::PayloadParamRef | Walk::Snapshot => {
                true
            }
        }
    }

    test_utils::f6_variants! {
        /// **The load door's snapshot vocabulary**, welded to the enum
        /// by the match the macro writes — the roster half of the
        /// census below. See [`test_utils::f6::VariantCensus`] for what
        /// that weld does and does not buy.
        const SNAPSHOT_ERROR: SnapshotError = [
            OrderMismatch,
            NodeNotMinted,
            StepIds,
            MintLogOrder,
            NameStepNotMinted,
            DeclaredSiteNotAnOperand,
            DeclaredNameNotUpstream,
            DanglingInput,
            ForwardInput,
            WitnessSite,
            WitnessOnMissingNode,
            LabelOnMissingNode,
            SlotDimension,
            SlotUnknownDocParam,
            SlotDocParamDimension,
            PayloadUnknownDocParam,
            PayloadDocParamDimension,
            EpsilonInvalid,
            Roots,
            NotAGauge,
            GaugeCycle,
            PlacementNonFinite,
            PlacementImproper,
            PlacementNonRigid,
            MateAlignment,
            PlacementRule,
            MeasureRefs,
            InputList,
            DuplicateInput,
            AssertionTarget,
            AssertionBound,
            MetadataUnversioned,
        ];
    }

    /// **The walk each refusal comes out of**, as an exhaustive match
    /// with no wildcard arm: a [`SnapshotError`] arm added tomorrow
    /// does not compile until it says which walk raises it.
    fn walk_of(err: &SnapshotError) -> Walk {
        match err {
            // `validate_document` itself, from the expression walks it
            // maps into this vocabulary.
            SnapshotError::SlotDimension { .. } => Walk::SlotDimension,
            SnapshotError::SlotUnknownDocParam { .. }
            | SnapshotError::SlotDocParamDimension { .. } => Walk::SlotParamRef,
            SnapshotError::PayloadUnknownDocParam { .. }
            | SnapshotError::PayloadDocParamDimension { .. } => Walk::PayloadParamRef,
            // `validate_snapshot`, which is where the rest live.
            SnapshotError::OrderMismatch
            | SnapshotError::NodeNotMinted { .. }
            | SnapshotError::StepIds { .. }
            | SnapshotError::MintLogOrder { .. }
            | SnapshotError::NameStepNotMinted { .. }
            | SnapshotError::DeclaredSiteNotAnOperand { .. }
            | SnapshotError::DeclaredNameNotUpstream { .. }
            | SnapshotError::DanglingInput { .. }
            | SnapshotError::ForwardInput { .. }
            | SnapshotError::WitnessSite { .. }
            | SnapshotError::WitnessOnMissingNode { .. }
            | SnapshotError::LabelOnMissingNode { .. }
            | SnapshotError::EpsilonInvalid { .. }
            | SnapshotError::Roots(_)
            | SnapshotError::NotAGauge { .. }
            | SnapshotError::GaugeCycle { .. }
            | SnapshotError::PlacementNonFinite { .. }
            | SnapshotError::PlacementImproper { .. }
            | SnapshotError::PlacementNonRigid { .. }
            | SnapshotError::MateAlignment { .. }
            | SnapshotError::PlacementRule { .. }
            | SnapshotError::MeasureRefs { .. }
            | SnapshotError::InputList { .. }
            | SnapshotError::DuplicateInput { .. }
            | SnapshotError::AssertionTarget { .. }
            | SnapshotError::AssertionBound { .. }
            | SnapshotError::MetadataUnversioned { .. } => Walk::Snapshot,
        }
    }

    /// **`validate_document`'s census, in code.** Three rounds of
    /// one-predicate work read this door's refusals off a table
    /// maintained by hand in the tracker, and the table undercounted
    /// twice — the second time by an entire walk and its two refusals.
    /// Nothing here is maintained by hand:
    ///
    /// - a walk added to [`Walk`] does not compile until
    ///   [`Walk::raises_snapshot_error`] places it;
    /// - a [`SnapshotError`] arm added does not compile until
    ///   [`walk_of`] and the [`SNAPSHOT_ERROR`] roster place it;
    /// - and the two sides are compared below, in both directions, so
    ///   an arm that names a walk which refuses in another vocabulary
    ///   — or a snapshot-refusing walk no arm comes out of — is red.
    ///
    /// The column the code cannot know — the EDIT-door twin of each
    /// refusal, and whether the predicate behind it is shared or
    /// hand-copied — is said on each arm's own doc ([`Walk`]'s doc says
    /// where the older table lives).
    #[test]
    fn every_snapshot_error_arm_names_the_walk_that_produces_it() {
        let at = |bits| crate::SpokenNode::absent(RecipeNodeId(bits));
        let node = || at(5);
        let face = || crate::names::StableName {
            kind: crate::names::EntityKind::Face,
            node: RecipeNodeId(5),
            path: Vec::new(),
        };
        // One value per arm, welded to the roster below: a case list
        // that fell behind the enum would name fewer identifiers than
        // the roster holds and red in the comparison.
        let cases = [
            SnapshotError::OrderMismatch,
            SnapshotError::NodeNotMinted { id: node() },
            SnapshotError::StepIds {
                node: node(),
                fault: crate::program::StepIdFault::Repeated {
                    step: crate::node::StepId(2),
                },
            },
            SnapshotError::MintLogOrder {
                entry: crate::Minted::Step(crate::node::StepId(3)),
            },
            SnapshotError::NameStepNotMinted {
                name: crate::SpokenName::absent(face()),
                step: crate::node::StepId(9),
            },
            SnapshotError::DeclaredSiteNotAnOperand {
                node: node(),
                name: crate::SpokenName::absent(face()),
                site: at(9),
            },
            SnapshotError::DeclaredNameNotUpstream {
                node: node(),
                name: crate::SpokenName::absent(face()),
            },
            SnapshotError::DanglingInput {
                node: node(),
                input: at(9),
            },
            SnapshotError::ForwardInput {
                node: node(),
                input: at(9),
            },
            SnapshotError::WitnessSite { node: node() },
            SnapshotError::WitnessOnMissingNode { node: node() },
            SnapshotError::LabelOnMissingNode { node: node() },
            SnapshotError::SlotDimension {
                node: node(),
                slot: SlotId::Distance,
                expected: Dimension::Length,
                found: Dimension::Angle,
            },
            SnapshotError::SlotUnknownDocParam {
                node: node(),
                slot: SlotId::Radius,
                name: ParamName::from_static("fillet"),
            },
            SnapshotError::SlotDocParamDimension {
                node: node(),
                slot: SlotId::Distance,
                name: ParamName::from_static("depth"),
                declared: Dimension::Angle,
                referenced: Dimension::Length,
            },
            SnapshotError::PayloadUnknownDocParam {
                node: node(),
                name: ParamName::from_static("depth"),
            },
            SnapshotError::PayloadDocParamDimension {
                node: node(),
                name: ParamName::from_static("depth"),
                declared: Dimension::Angle,
                referenced: Dimension::Length,
            },
            SnapshotError::EpsilonInvalid { value: 0.0 },
            SnapshotError::Roots(crate::roots::RootFault::Ancestor {
                ancestor: at(1),
                descendant: at(2),
            }),
            SnapshotError::NotAGauge {
                node: node(),
                gauge: at(2),
            },
            SnapshotError::GaugeCycle {
                node: node(),
                gauge: at(2),
            },
            SnapshotError::PlacementNonFinite {
                node: node(),
                at: FrameSite::Step { index: 0 },
            },
            SnapshotError::PlacementImproper {
                node: node(),
                at: FrameSite::Step { index: 1 },
                determinant: -1.0,
            },
            SnapshotError::PlacementNonRigid {
                node: node(),
                at: FrameSite::Step { index: 1 },
                check: "transform_rigid_col0_unit",
            },
            SnapshotError::MateAlignment { node: node() },
            SnapshotError::PlacementRule {
                node: node(),
                fault: crate::node::PlacementRuleFault::NoPlacements,
            },
            SnapshotError::MeasureRefs {
                node: node(),
                fault: crate::node::MeasureNodeFault::RefIndexOutOfRange {
                    verb: "distance",
                    index: 3,
                    refs: 2,
                },
            },
            SnapshotError::InputList {
                node: node(),
                fault: crate::node::ListFault::TooFew { found: 1 },
            },
            SnapshotError::DuplicateInput {
                node: node(),
                input: at(9),
            },
            SnapshotError::AssertionTarget {
                node: node(),
                measure: at(4),
                bound: Dimension::Count,
            },
            SnapshotError::AssertionBound {
                node: node(),
                measure: at(4),
                measured: Dimension::Length,
                bound: Dimension::Angle,
            },
            SnapshotError::MetadataUnversioned {
                name: crate::SpokenName::absent(face()),
                key: "swatch".to_owned(),
                error: crate::meta::MetaVersionError::MissingVersion,
            },
        ];

        let covered: Vec<String> = cases
            .iter()
            .map(test_utils::f6::variant_identifier)
            .collect();
        let covered: Vec<&str> = covered.iter().map(String::as_str).collect();
        if let Some(report) = test_utils::census::set_difference(
            SNAPSHOT_ERROR.identifiers(),
            &covered,
            "the `SnapshotError` roster and the cases this census walks disagree",
            "carried by a case and absent from the roster — add it, spelled as `Debug` \
             renders it",
            "in the roster and carried by no case — give it a case, and place it in `walk_of`",
        ) {
            panic!("{report}");
        }

        let walks: Vec<String> = Walk::ORDER
            .iter()
            .map(test_utils::f6::variant_identifier)
            .collect();
        let walks: Vec<&str> = walks.iter().map(String::as_str).collect();
        if let Some(report) = test_utils::census::set_difference(
            WALK.identifiers(),
            &walks,
            "the `Walk` roster and `Walk::ORDER` disagree",
            "in `Walk::ORDER` and absent from the roster",
            "in the roster and absent from `Walk::ORDER`, which is what `validate_document` \
             runs — so this walk never executes",
        ) {
            panic!("{report}");
        }

        // The two sides against each other: every arm comes out of a
        // walk that refuses in THIS vocabulary, and every such walk is
        // reached by an arm. A walk whose refusals stopped being
        // `SnapshotError`s, or one whose arms were folded into
        // another's, reds here rather than in the tracker.
        for err in &cases {
            let walk = walk_of(err);
            assert!(
                raises_snapshot_error(walk),
                "{err:?} is placed in {walk:?}, a walk that refuses in another vocabulary"
            );
        }
        for walk in Walk::ORDER {
            assert_eq!(
                raises_snapshot_error(walk),
                cases.iter().any(|err| walk_of(err) == walk),
                "{walk:?} and the arms placed in it disagree about whether it refuses with a \
                 `SnapshotError`"
            );
        }
    }

    /// Convention 2's point, pinned at the unit level: a document
    /// that would refuse to load cannot be saved. Both corruptions
    /// need `pub(crate)` access — no API door reaches them, only an
    /// in-crate bug would — and before the consolidation both SAVED
    /// fine, producing a file the load door refuses.
    #[test]
    fn structurally_invalid_documents_refuse_at_save() {
        // ε = 0.0 is finite (past the float walk) but invalid — the
        // load door's EpsilonInvalid, now at save too.
        let mut doc = ProfileDoc::empty_derived("check", Tol::witness());
        doc.epsilon = 0.0;
        match save(&doc, &[], Tol::witness()) {
            Err(PersistError::Snapshot(SnapshotError::EpsilonInvalid { value })) => {
                assert_eq!(value, 0.0);
            }
            other => panic!("non-positive ε must refuse at save, got {other:?}"),
        }
        // order naming a node the map does not hold.
        let mut doc = ProfileDoc::empty_derived("check", Tol::witness());
        doc.order.push(RecipeNodeId(7));
        match save(&doc, &[], Tol::witness()) {
            Err(PersistError::Snapshot(SnapshotError::OrderMismatch)) => {}
            other => panic!("order mismatch must refuse at save, got {other:?}"),
        }
    }

    /// **`n` instances of a part reference nothing resolves**: the
    /// `DocRef` is minted here, over bytes rather than over a part
    /// document, because the validator never resolves a reference — it
    /// reads the document's own structure.
    ///
    /// That is what distinguishes it from the same-shaped fixture in
    /// the `edit_one_predicate` suite, which inserts a real part into a
    /// store because its rows build mate HEADS that name faces inside
    /// the referenced part. The two cannot be one function: an
    /// integration suite cannot reach a `#[cfg(test)]` item in this
    /// library, and this library cannot reach `tests/fixture`.
    ///
    /// Built through the edit door, so every invariant beside the one a
    /// row then breaks is the one `apply` maintains.
    fn instances_of_an_unresolved_reference(
        seed: &str,
        n: usize,
    ) -> (ProfileDoc, Vec<RecipeNodeId>) {
        let doc_ref = crate::ident::DocRef {
            id: crate::ident::DocumentId::derive("check-part"),
            pin: crate::ident::ContentPin::of_bytes(b"check-part"),
        };
        let mut doc = ProfileDoc::empty_derived(seed, Tol::witness());
        let mut ids = Vec::new();
        for _ in 0..n {
            let applied = crate::edit::apply(
                &doc,
                &crate::edit::DocEdit::InsertNode {
                    node: Box::new(Node::instantiate_part(doc_ref)),
                },
                Tol::witness(),
                &crate::mate::RefusingReach,
            )
            .expect("an instance inserts");
            ids.push(applied.record.minted.expect("the insert minted an id"));
            doc = applied.doc;
        }
        (doc, ids)
    }

    /// The two refusals a FILE cannot carry, pinned at the door that
    /// can: JSON has no non-finite token, so a non-finite alignment or
    /// placement coordinate reaches [`validate_document`] only from
    /// memory — through the `pub` payloads or an in-crate bug. Both
    /// documents are built through the edit door and then broken, so
    /// each row's subject is the one coordinate it poked.
    #[test]
    fn non_finite_alignment_and_placement_refuse_at_save() {
        let (mut doc, ids) = instances_of_an_unresolved_reference("check-align", 2);
        let name = |instance: RecipeNodeId| crate::names::StableName {
            kind: crate::names::EntityKind::Face,
            node: instance,
            path: vec![crate::names::RoleSeg::Cap(crate::names::CapEnd::Start)],
        };
        // A mate head is a `SitedFace`, so the fixture's claim that
        // the name it just built is a face is made where it is built.
        let face_head = |name: crate::names::StableName| {
            crate::node::SitedFace::at_mint(
                crate::names::FaceName::new(name).expect("the fixture names a face"),
            )
        };
        let mate = Node::Mate {
            a: face_head(name(ids[0])),
            b: face_head(name(ids[1])),
            class: topo::ContactClass::Rest,
            alignment: crate::mate::Alignment {
                a: crate::mate::MateFrame::authored([0.0; 3], [0.0, 0.0, 1.0], [1.0, 0.0, 0.0]),
                b: crate::mate::MateFrame::authored([0.0; 3], [0.0, 0.0, 1.0], [1.0, 0.0, 0.0]),
                primitive: crate::mate::MatePrimitive::FrameCoincidence,
                sense: crate::mate::AxisSense::Aligned,
                clocking: None,
            },
        };
        let applied = crate::edit::apply(
            &doc,
            &crate::edit::DocEdit::InsertNode {
                node: Box::new(mate),
            },
            Tol::witness(),
            &crate::mate::RefusingReach,
        )
        .expect("a finite alignment inserts");
        let mate_id = applied.record.minted.expect("the insert minted an id");
        doc = applied.doc;
        // Finite, the document saves — so the refusal below is the
        // poked coordinate's.
        save(&doc, &[], Tol::witness()).expect("the mated assembly saves");
        match doc.nodes.get_mut(&mate_id) {
            Some(Node::Mate { alignment, .. }) => {
                alignment.a = crate::mate::MateFrame::authored(
                    [f64::NAN, 0.0, 0.0],
                    [0.0, 0.0, 1.0],
                    [1.0, 0.0, 0.0],
                );
            }
            other => panic!("the fixture's mate is a mate, got {other:?}"),
        }
        match save(&doc, &[], Tol::witness()) {
            Err(PersistError::Snapshot(SnapshotError::MateAlignment { node })) => {
                assert_eq!(node, doc.spoken(mate_id));
            }
            other => panic!("a non-finite alignment must refuse at save, got {other:?}"),
        }

        let (mut doc, ids) = instances_of_an_unresolved_reference("check-place", 1);
        match doc.nodes.get_mut(&ids[0]) {
            Some(Node::InstantiatePart { offset, .. }) => {
                *offset = Some(crate::placement::Placement::literal(
                    &crate::placement::Frame {
                        columns: [[f64::NAN, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]],
                        translation: [0.0; 3],
                    },
                ));
            }
            other => panic!("the fixture's instance is an instance, got {other:?}"),
        }
        match save(&doc, &[], Tol::witness()) {
            Err(PersistError::Snapshot(SnapshotError::PlacementNonFinite { node, .. })) => {
                assert_eq!(node, doc.spoken(ids[0]));
            }
            other => panic!("a non-finite placement must refuse at save, got {other:?}"),
        }
    }

    /// A triangle profile on a frame, the profile labelled `outline`,
    /// as the edit doors build it.
    fn labelled_triangle(tol: Tol) -> (ProfileDoc, RecipeNodeId) {
        let insert = |doc: ProfileDoc, node| {
            let doc = doc
                .apply(
                    &crate::DocEdit::InsertNode { node },
                    tol,
                    &crate::RefusingReach,
                )
                .expect("the node inserts")
                .doc;
            let id = *doc.order().last().expect("the inserted node");
            (doc, id)
        };
        let doc = ProfileDoc::empty_derived("check-speak", tol);
        let (doc, plane) = insert(doc, Box::new(crate::test_support::xy_frame()));
        let triangle = crate::program::LoopProgram::polygon([(0.0, 0.0), (1.0, 0.0), (0.5, 1.0)])
            .expect("finite corners");
        let (doc, profile) = insert(
            doc,
            Box::new(Node::Profile(crate::program::ProfileProgram {
                plane,
                loops: vec![triangle],
                ids: Vec::new(),
            })),
        );
        let doc = doc
            .apply(
                &crate::DocEdit::SetLabel {
                    node: profile,
                    label: Some(crate::Label::new("outline").expect("a label")),
                },
                tol,
                &crate::RefusingReach,
            )
            .expect("the label sets")
            .doc;
        (doc, profile)
    }

    /// The save door speaks a refused program's node from the document
    /// it judges, label and all — not by its tag alone.
    #[test]
    fn the_save_door_speaks_a_refused_programs_node() {
        let tol = Tol::witness();
        let (mut doc, profile) = labelled_triangle(tol);
        let Some(Node::Profile(program)) = doc.nodes.get_mut(&profile) else {
            panic!("the profile is a profile");
        };
        let crate::program::LoopProgram::Chain(steps) = &mut program.loops[0] else {
            panic!("a polygon is a chain");
        };
        steps.pop();
        match save(&doc, &[], tol) {
            Err(e @ PersistError::ProfileProgram { .. }) => {
                let PersistError::ProfileProgram { node, .. } = &e else {
                    unreachable!("matched above")
                };
                assert_eq!(*node, doc.spoken(profile), "spoken from the document");
                assert_eq!(node.label().map(crate::Label::as_str), Some("outline"));
                let said = e.to_string();
                assert!(
                    said.contains(&format!("Profile \"outline\" ({profile})'s program: ")),
                    "{said}"
                );
            }
            other => panic!("an unclosed chain refuses at save, got {other:?}"),
        }
    }

    /// A non-finite metadata float names its record by the name's
    /// minting node, spoken from the document.
    #[test]
    fn the_save_door_speaks_a_non_finite_metadata_sites_name() {
        let tol = Tol::witness();
        let (mut doc, profile) = labelled_triangle(tol);
        let name = crate::names::StableName {
            kind: crate::names::EntityKind::Face,
            node: profile,
            path: Vec::new(),
        };
        let mut record = crate::appearance::AppearanceRecord::default();
        record
            .metadata
            .insert("swatch".to_owned(), crate::meta::MetaValue::Float(f64::NAN));
        doc.appearance.insert(name.clone(), record);
        match save(&doc, &[], tol) {
            Err(e @ PersistError::NonFinite { .. }) => {
                let PersistError::NonFinite {
                    site:
                        super::NonFiniteSite::Metadata {
                            name: said_name, ..
                        },
                } = &e
                else {
                    panic!("the site is the metadata's, got {e:?}");
                };
                assert_eq!(*said_name, doc.spoken_name(&name));
                let said = e.to_string();
                assert!(
                    said.contains(&format!(
                        "metadata \"swatch\" on the face name minted by Profile \"outline\" \
                         ({profile})"
                    )),
                    "{said}"
                );
            }
            other => panic!("a NaN in metadata refuses at save, got {other:?}"),
        }
    }

    // ---- review probes (lane `carriers-rv`, PR #2797) ----

    /// A name minted by `node`, with no role path.
    fn rv_name(node: u64, kind: crate::names::EntityKind) -> crate::names::StableName {
        crate::names::StableName {
            kind,
            node: RecipeNodeId(node),
            path: Vec::new(),
        }
    }

    /// **The store half of the load door's id check**: a document
    /// whose ONLY fault is an appearance key minted by a node the mint
    /// log does not hold refuses typed, with that id named.
    ///
    /// Before this row the store half was held by nothing — dropping
    /// the whole name pass reds two rows, dropping only its `Store`
    /// arm red none. The check is reachable, not dead: no edit door
    /// writes a key the log lacks, but `SetAppearance` is the one
    /// name-carrying edit the insert door deliberately does not check
    /// (`resolve::walk_names`' match says why — an appearance name
    /// resolves at evaluation, where a miss is a typed
    /// `AppearanceLoss`), so a loaded document can hold such a key and
    /// this door is the only one that refuses it. The corruption
    /// needs in-crate reach for the same reason.
    ///
    /// The twin on the payload side is
    /// `asm_r2a_mate_solve::row6i_the_load_check_refuses_a_mate_head_the_mint_never_minted`.
    #[test]
    fn rv_an_appearance_key_the_mint_never_minted_refuses_typed() {
        let mut doc = ProfileDoc::empty_derived("rv-store-id", Tol::witness());
        assert!(
            doc.mint.log().is_empty(),
            "the empty document has minted nothing"
        );
        doc.appearance.insert(
            rv_name(7, crate::names::EntityKind::Face),
            crate::appearance::AppearanceRecord::default(),
        );
        match save(&doc, &[], Tol::witness()) {
            Err(PersistError::Snapshot(SnapshotError::NodeNotMinted { id })) => {
                assert_eq!(id, crate::SpokenNode::absent(RecipeNodeId(7)));
            }
            other => panic!("an appearance key the mint never minted must refuse, got {other:?}"),
        }
    }

    /// **The validator's name pass answers in DOCUMENT order, not id
    /// order.** Two payload names are corrupt at once, and the
    /// document orders their carrying nodes in the REVERSE of their id
    /// order, so a walk over `doc.nodes` and a walk over `doc.order`
    /// name different offending ids. This row says which one this
    /// door does: the pass is `Doc::name_carriers`, which walks
    /// `Doc::order`, so the document's FIRST node answers and the id
    /// is 60.
    ///
    /// **That order is not a contract.** `validate_snapshot` promises
    /// that a corrupt document refuses typed, not WHICH of its faults
    /// it names first; `Walk::ORDER` contracts between walks, not
    /// within one, and a caller cannot repair a doubly corrupt file by
    /// reading the first refusal anyway. What this row is for is that
    /// the answer moved and nothing said so — the pass used to run
    /// inside the per-node loop over `doc.nodes`, a `BTreeMap`, and
    /// this document refused with 50. An unpinned order that changes
    /// silently is how a diagnosis drifts one refactor at a time, so
    /// the row names the order the walk has now: a later change that
    /// moves it again has to say it is moving it.
    #[test]
    fn rv_the_name_pass_refuses_in_document_order() {
        let mut doc = ProfileDoc::empty_derived("rv-name-order", Tol::witness());
        doc.mint = doc
            .mint
            .clone()
            .logged([0, 1, 2, 3].map(|id| crate::Minted::Node(RecipeNodeId(id))));
        for id in [2u64, 3] {
            doc.nodes.insert(
                RecipeNodeId(id),
                Node::Datum(crate::node::Datum::Plane {
                    origin: [0.0; 3].map(crate::test_support::len),
                    normal: [0.0, 0.0, 1.0].map(crate::test_support::scl),
                }),
            );
        }
        for (id, derived) in [(0u64, 50u64), (1, 60)] {
            let sited = || {
                crate::node::SitedRef::new(
                    RecipeNodeId(2),
                    rv_name(derived, crate::names::EntityKind::Face),
                )
            };
            doc.nodes.insert(
                RecipeNodeId(id),
                Node::Boolean {
                    op: topo::BooleanOp::Union,
                    a: RecipeNodeId(2),
                    b: RecipeNodeId(3),
                    declare: vec![((sited(), sited()), topo::BooleanCoincidence::REST)],
                },
            );
        }
        doc.order = vec![
            RecipeNodeId(2),
            RecipeNodeId(3),
            RecipeNodeId(1),
            RecipeNodeId(0),
        ];
        match save(&doc, &[], Tol::witness()) {
            Err(PersistError::Snapshot(SnapshotError::NodeNotMinted { id })) => {
                assert_eq!(
                    id.id(),
                    RecipeNodeId(60),
                    "the name pass walks `Doc::order`, so the document's FIRST node answers"
                );
            }
            other => panic!("a corrupt payload name must refuse, got {other:?}"),
        }
    }
}
