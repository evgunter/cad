//! The persistence doors' ONE shared validator (spec D2/D6.3; DESIGN
//! engineering convention 2, discharged M5 S4):
//!
//! [`validate_document`] holds every direction-independent document
//! check and is invoked by BOTH doors — at save on the in-memory
//! document before a byte is written, at load on the parsed document
//! before replay. A document that would refuse to load is therefore
//! impossible to save by construction: not two mirrored door sets
//! kept in sync by a sweep, but code that is literally the same and
//! cannot drift. Its walks:
//!
//! - [`first_non_finite`] — every float the format would write,
//!   checked finite, with a typed site name. Expression literals are
//!   finite BY CONSTRUCTION (`Expr::literal` refuses non-finite —
//!   ruled door 1; the load side re-runs the same constructors), so
//!   the walk covers the float carriers outside that door: profile
//!   PLANE PLACEMENTS (program args are Exprs, already doored),
//!   continuous doc params, the recorded ε, and D7 metadata
//!   trees (in the snapshot AND in the edit log). A NEW float-carrying
//!   field must join this walk — the D2 round-trip property tests are
//!   the tripwire. (Post-parse this walk cannot fire — JSON has no
//!   non-finite tokens — which is the asymmetry being BYTE-level, not
//!   a reason to fork the validator.)
//! - [`first_distribution_fault`] — the E2 invariants of every doc
//!   param's distribution beyond finiteness, by the same
//!   `Distribution::check` the edit door runs. It walks the SNAPSHOT
//!   only: a `SetDocParam` in the log carries its distribution through
//!   `apply` on replay, which is the same door and the same check, so
//!   a second walk over the log would ask a question `apply` has
//!   already answered. Every snapshot-only walk below holds for this
//!   reason and no other.
//! - [`first_display_unit_fault`] — every document parameter's
//!   authored display unit measures the dimension it was declared
//!   with. A literal needs no twin walk (`Expr::literal_with_unit`
//!   makes the pairing at construction and the load side re-runs it);
//!   a `DocParam` does, because its payload is `pub` and its dimension
//!   is data. Snapshot only, for the reason above.
//! - [`first_slot_fault`] — every node's SLOT expressions carry the
//!   dimension their addresses fix (spec D6), by the same
//!   `Node::slot_dimension_fault` the edit doors ask. EVERY node kind:
//!   a walk that asked profile programs alone admitted a retyped
//!   extrude distance the edit door refuses. Snapshot only, for the
//!   reason above.
//! - [`first_slot_param_ref_fault`] — every slot expression's
//!   document-parameter references against the param table, by the
//!   same `Doc::param_ref_fault` the edit doors ask. An undeclared
//!   name and a dimension the declaration contradicts are facts about
//!   the document; a reference that merely fails to EVALUATE is V1
//!   class 2 and passes. Snapshot only, for the reason above.
//! - [`first_program_fault`] — profile PROGRAM structure: a REPLAY
//!   PROBE under the document's params whose LATTICE violations refuse
//!   (the corrupt-file class — no authoring surface produces them);
//!   resolve failures and geometry refusals PASS this door (V1
//!   class 2: refusing programs may exist at rest — they surface as
//!   typed node errors at evaluation). A step argument's dimension is
//!   the slot walk's above, not a second spelling here. The retired
//!   stored-joint walk died with stored joints: programs persist no
//!   derived values.
//! - [`validate_snapshot`] — the document invariants `apply`
//!   maintains, re-checked structurally (a parsed snapshot is not
//!   trusted; an in-memory one can be corrupted through the `pub`
//!   payload or an in-crate bug). Every rule an edit door also decides
//!   is DELEGATED to the one predicate both doors ask, and this walk
//!   only names the answer in the load door's vocabulary:
//!   `doc::epsilon_admissible`, `doc::witness_site_fault`,
//!   `doc::placement_fault`,
//!   `Node::placement_rule_fault`, `Node::input_fault`,
//!   `Node::measure_fault`, `Node::bad_declare_input`,
//!   `Node::assertion_bound_fault`, `Node::has_non_finite_alignment`,
//!   `MetaValue::require_versioned` and `roots::check`. What is stated
//!   HERE and nowhere else is what only a FILE can be wrong about —
//!   `order` against the node map, ids past the mint counter, a
//!   forward input, and the placement registry's gauge — plus the two
//!   liveness walks whose rule is the node map's own lookup. Each site
//!   says which it is.
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
use crate::doc::{DocParam, DocParamField, ParamName, PlacementFault, WitnessSiteFault};
use crate::edit::DocEdit;
use crate::meta::MetaVersionError;
use crate::names::StableName;
use crate::node::SlotId;
use crate::node::{AssertionBoundFault, Node, RecipeNodeId, SlotDimensionFault};
use crate::program::{ProfileDoc, ProfileProgram, ProgramRefusal};
use crate::resolve::derivation_nodes;
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
    /// A float inside an appearance record's metadata (snapshot).
    Metadata {
        /// The attributed name.
        name: StableName,
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
            } => write!(f, "document parameter {:?}", name.0),
            Self::DocParam { name, field } => {
                write!(f, "document parameter {:?}, {field}", name.0)
            }
            Self::Metadata { name, key, path } => {
                write!(f, "metadata {key:?} on the {name}, at {path}")
            }
            Self::Edit { index, inner } => write!(f, "edit {index}, {inner}"),
        }
    }
}

/// The shared validator (module docs): every direction-independent
/// document check, in one place, invoked by both doors. Check order
/// is float walk → distribution walk → display-unit walk → SLOT walks
/// (dimension, then param refs) → program walk → structural
/// invariants (the save door's historical precedence, pinned by the
/// refusal suite).
///
/// The slot walk runs before the program walk because the program
/// walk PROBES the replay: a step whose argument is an angle where the
/// role fixes a length is not a walk worth probing, and "your loop is
/// not a legal lattice walk" is the wrong sentence for it.
pub(crate) fn validate_document(
    snapshot: &ProfileDoc,
    edits: &[DocEdit<ProfileProgram>],
    tol: Tol,
) -> Result<(), super::PersistError> {
    if let Some(site) = first_non_finite(snapshot, edits) {
        return Err(super::PersistError::NonFinite { site });
    }
    if let Some((name, fault)) = first_distribution_fault(snapshot) {
        return Err(super::PersistError::Distribution { name, fault });
    }
    if let Some((name, unit, declared)) = first_display_unit_fault(snapshot) {
        return Err(super::PersistError::DisplayUnit {
            name,
            unit,
            declared,
        });
    }
    if let Some((node, fault)) = first_slot_fault(snapshot) {
        return Err(super::PersistError::Snapshot(match fault {
            SlotDimensionFault::MissingExpression { slot } => {
                SnapshotError::SlotExpressionMissing { node, slot }
            }
            SlotDimensionFault::Mismatch {
                slot,
                expected,
                found,
            } => SnapshotError::SlotDimension {
                node,
                slot,
                expected,
                found,
            },
        }));
    }
    if let Some((node, slot, fault)) = first_slot_param_ref_fault(snapshot) {
        return Err(super::PersistError::Snapshot(match fault {
            crate::doc::ParamRefFault::Unknown { name } => {
                SnapshotError::SlotUnknownDocParam { node, slot, name }
            }
            crate::doc::ParamRefFault::Dimension {
                name,
                declared,
                referenced,
            } => SnapshotError::SlotDocParamDimension {
                node,
                slot,
                name,
                declared,
                referenced,
            },
        }));
    }
    if let Some((node, fault)) = first_program_fault(snapshot, tol) {
        return Err(super::PersistError::ProfileProgram { node, fault });
    }
    validate_snapshot(snapshot).map_err(super::PersistError::Snapshot)
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

/// The first non-finite float in ε, the document params, the profile
/// nodes, the appearance records or the edit log, reported as a
/// [`NonFiniteSite`], or `None`.
///
/// NOT every float the format writes: placement frames and mate
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
                name: name.clone(),
                key,
                path,
            });
        }
    }
    for (index, edit) in edits.iter().enumerate() {
        if let Some(inner) = edit_non_finite(edit) {
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
/// is DATA — it has not necessarily been applied by this process).
fn edit_non_finite(edit: &DocEdit<ProfileProgram>) -> Option<NonFiniteSite> {
    match edit {
        DocEdit::SetDocParam { name, value } => param_site(name, value),
        // The value door carries no distribution of its own — the
        // declaration it writes into supplies that — but its
        // continuous arm IS a raw float the format writes.
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
                    name: name.clone(),
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
        // - `Node::Mate`'s `alignment` and `SetPlacement`'s `frame` are
        //   RAW `f64`, not `Expr`s. They are refused on replay by
        //   `apply` (`EditError::NonFiniteAlignment`,
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
        | DocEdit::SetTolerance { .. }
        | DocEdit::DeleteNode { .. }
        | DocEdit::SetParam { .. }
        | DocEdit::SetStructuralParam { .. }
        | DocEdit::SetExpression { .. }
        | DocEdit::Rebind { .. }
        | DocEdit::ReWitness { .. }
        | DocEdit::ReWitnessBulk { .. }
        | DocEdit::SetAppearance { .. }
        | DocEdit::ClearAppearance { .. }
        | DocEdit::ClearAppearanceMeta { .. }
        | DocEdit::SetRoots { .. }
        | DocEdit::SetPlacement { .. }
        | DocEdit::UpdateReference { .. } => None,
    }
}

/// A structural invariant violation in a parsed snapshot (load door).
#[derive(Debug, Clone, PartialEq)]
pub enum SnapshotError {
    /// `order` and the node map disagree (missing, extra, or
    /// duplicated ids).
    OrderMismatch,
    /// An id at or beyond the mint counter appears in the document.
    IdBeyondCounter {
        /// The offending id.
        id: RecipeNodeId,
        /// The counter.
        next_id: u64,
    },
    /// A node's input ref does not name a live node.
    DanglingInput {
        /// The referring node.
        node: RecipeNodeId,
        /// The missing input.
        input: RecipeNodeId,
    },
    /// A node's input ref does not precede it in `order` (insertion
    /// order is topological by construction — a forward ref means a
    /// tampered file, and possibly a cycle).
    ForwardInput {
        /// The referring node.
        node: RecipeNodeId,
        /// The forward input.
        input: RecipeNodeId,
    },
    /// A node's `declare` input names a node that is not a
    /// `Node::Declare` — the edit door's rule, asked of file data
    /// (`Node::bad_declare_input`, one predicate, both doors).
    DeclareInput {
        /// The consuming node.
        node: RecipeNodeId,
        /// What its `declare` input names.
        input: RecipeNodeId,
    },
    /// A witness attached to a node that bears no sketch. Its own arm
    /// rather than [`SnapshotError::WitnessOnMissingNode`]: the edit
    /// door names the two apart, and the repairs differ — one moves
    /// the witness, the other has no node to move it to.
    WitnessSite {
        /// The offending node id.
        node: RecipeNodeId,
    },
    /// A witness attached to a node id that names nothing live.
    WitnessOnMissingNode {
        /// The offending node id.
        node: RecipeNodeId,
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
    /// A placement row keyed by a node that is not a live
    /// `InstantiatePart` (A11: only an instance's cluster has a frame).
    PlacementSite {
        /// The offending key.
        node: RecipeNodeId,
    },
    /// A placement frame carrying a non-finite coordinate. The edit
    /// door refuses it, so a file holding one is corrupt — refused,
    /// never repaired.
    PlacementNonFinite {
        /// The offending key.
        node: RecipeNodeId,
    },
    /// An IMPROPER placement frame — determinant ≤ 0, the A6 mirror
    /// case R4 gates. Its own arm rather than
    /// [`SnapshotError::PlacementNonFinite`]: a mirror is authored data
    /// this build declines to admit, a non-finite coordinate is data no
    /// predicate can read, and the repairs differ.
    PlacementImproper {
        /// The offending key.
        node: RecipeNodeId,
        /// The linear part's determinant.
        determinant: f64,
    },
    /// A placement row keyed by an instance that is NOT its cluster's
    /// gauge (ASM-R2a D-3). A11 puts the frame on the cluster, and the
    /// cluster's key is its document-order-first instance; any other
    /// key would place a member instead of the cluster, which is the
    /// multi-anchor state A11 makes unrepresentable.
    PlacementNotGauge {
        /// The offending key.
        node: RecipeNodeId,
        /// The gauge that should have carried the row.
        gauge: RecipeNodeId,
    },
    /// A mate's alignment datum carries a non-finite coordinate. The
    /// edit door refuses it, so a file holding one is corrupt: no
    /// predicate could decide anything about it.
    MateAlignment {
        /// The offending mate.
        node: RecipeNodeId,
    },
    /// A placement-rule node carrying a rule the edit door would have
    /// refused (GROUP-BOOLEAN-DESIGN): the count spelled two ways, an
    /// empty explicit list, or a non-finite / improper frame.
    PlacementRule {
        /// The offending node.
        node: RecipeNodeId,
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
        node: RecipeNodeId,
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
        node: RecipeNodeId,
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
        node: RecipeNodeId,
        /// The slot whose expression reads it.
        slot: SlotId,
        /// The name it reads.
        name: ParamName,
        /// The dimension the declaration carries.
        declared: crate::expr::Dimension,
        /// The dimension the expression reads it at.
        referenced: crate::expr::Dimension,
    },
    /// A node that names a slot it cannot answer for. `Node::slots`
    /// and `Node::expr` agree by construction, so this is a vocabulary
    /// bug in the node layer rather than a property of the file —
    /// surfaced rather than skipped, at this door as at the edit door
    /// ([`crate::EditError::UnknownSlot`]), because a slot that cannot
    /// be read is a slot no rule above can decide.
    SlotExpressionMissing {
        /// The offending node.
        node: RecipeNodeId,
        /// The slot with no expression behind it.
        slot: SlotId,
    },
    /// A measure node whose expression reads a reference the node does
    /// not carry (E3). The expression indexes the reference list
    /// positionally, so this is a corrupt file, not a stale reference:
    /// `Rebind` cannot repair an index.
    MeasureRefs {
        /// The offending node.
        node: RecipeNodeId,
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
        node: RecipeNodeId,
        /// What is wrong with it.
        fault: crate::node::InputFault,
    },
    /// An assertion whose reference is not a measure at all (E10):
    /// there is no measured dimension for the bound to agree with. The
    /// edit door refuses it, so a file carrying one is data the edit
    /// door would never have produced.
    AssertionTarget {
        /// The offending assertion.
        node: RecipeNodeId,
        /// What it references.
        measure: RecipeNodeId,
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
        node: RecipeNodeId,
        /// The measure it constrains.
        measure: RecipeNodeId,
        /// What that measure yields.
        measured: crate::expr::Dimension,
        /// The bound's dimension.
        bound: crate::expr::Dimension,
    },
    /// An appearance metadata value violating the D7 producer
    /// convention (map with an integer `"v"`).
    MetadataUnversioned {
        /// The attributed name.
        name: StableName,
        /// The metadata key.
        key: String,
        /// The typed shape refusal.
        error: MetaVersionError,
    },
}

// The document layer's prose for a corrupt snapshot: each arm states
// WHAT is wrong and WHERE, and forwards the payload's own `Display`
// wherever the payload has one (`RootFault`, `PlacementRuleFault`,
// `MetaVersionError`) — a site that re-states a payload it holds
// invents a second vocabulary for a refusal that already has one. A
// `StableName` renders through its own `Display` (kind plus minting
// node), never a hand-rolled respelling.
impl core::fmt::Display for SnapshotError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::OrderMismatch => f.write_str(
                "the `order` list and the node map disagree — an id is missing, extra or \
                 duplicated",
            ),
            Self::IdBeyondCounter { id, next_id } => write!(
                f,
                "node id {} is at or beyond the mint counter {next_id} — replay would \
                 re-mint a referenced id",
                id.0
            ),
            Self::DanglingInput { node, input } => write!(
                f,
                "node {} takes input from node {}, which is not live",
                node.0, input.0
            ),
            Self::ForwardInput { node, input } => write!(
                f,
                "node {} takes input from node {}, which does not precede it in `order`",
                node.0, input.0
            ),
            Self::DeclareInput { node, input } => write!(
                f,
                "node {}'s declare input names node {}, which is not a declaration",
                node.0, input.0
            ),
            Self::WitnessSite { node } => write!(
                f,
                "a witness is attached to node {}, which bears no sketch",
                node.0
            ),
            Self::WitnessOnMissingNode { node } => write!(
                f,
                "a witness is attached to node {}, which is not live",
                node.0
            ),
            Self::EpsilonInvalid { value } => write!(
                f,
                "the recorded ε {value:e} is not finite and strictly positive"
            ),
            Self::Roots(fault) => write!(f, "{fault}"),
            Self::PlacementSite { node } => write!(
                f,
                "a placement is keyed by node {}, which does not instantiate a part",
                node.0
            ),
            // The frame clause is the frame rule's own
            // (`crate::placement::FrameFault`); these arms supply only
            // the subject, so a reader sees one sentence about a frame
            // wherever a frame was refused.
            Self::PlacementNonFinite { node } => write!(
                f,
                "the placement frame on node {} {}",
                node.0,
                crate::placement::FrameFault::NonFinite
            ),
            Self::PlacementImproper { node, determinant } => write!(
                f,
                "the placement frame on node {} {}",
                node.0,
                crate::placement::FrameFault::Improper {
                    determinant: *determinant
                }
            ),
            Self::PlacementNotGauge { node, gauge } => write!(
                f,
                "the placement keyed by node {} belongs on its cluster's gauge, node {}",
                node.0, gauge.0
            ),
            Self::MateAlignment { node } => write!(
                f,
                "mate node {}'s alignment datum carries a non-finite coordinate",
                node.0
            ),
            Self::PlacementRule { node, fault } => {
                write!(f, "placement-rule node {}: {fault}", node.0)
            }
            // A PROGRAM slot's address is spelled out rather than
            // dumped: `SlotId::Profile`'s three fields are the
            // location, and a derived rendering would put the struct's
            // own braces in a user's message.
            Self::SlotDimension {
                node,
                slot: SlotId::Profile { loop_, step, arg },
                expected,
                found,
            } => write!(
                f,
                "node {}: loop {loop_} step {step}'s {} argument needs {} {expected} \
                 expression, got {} {found}",
                node.0,
                arg.label(),
                expected.article(),
                found.article()
            ),
            Self::SlotDimension {
                node,
                slot,
                expected,
                found,
            } => write!(
                f,
                "node {}: slot {} needs {} {expected} expression, got {} {found}",
                node.0,
                slot.label(),
                expected.article(),
                found.article()
            ),
            Self::SlotUnknownDocParam { node, slot, name } => write!(
                f,
                "node {}: slot {} reads the parameter {:?}, which the document does not declare",
                node.0,
                slot.label(),
                name.0
            ),
            Self::SlotDocParamDimension {
                node,
                slot,
                name,
                declared,
                referenced,
            } => write!(
                f,
                "node {}: slot {} reads the parameter {:?} as {} {referenced}, and it is \
                 declared {declared}",
                node.0,
                slot.label(),
                name.0,
                referenced.article()
            ),
            Self::SlotExpressionMissing { node, slot } => write!(
                f,
                "node {} names the slot {} and carries no expression for it",
                node.0,
                slot.label()
            ),
            Self::MeasureRefs { node, fault } => {
                write!(f, "measure node {}: {fault}", node.0)
            }
            Self::InputList { node, fault } => write!(f, "node {}: {fault}", node.0),
            Self::AssertionBound {
                node,
                measure,
                measured,
                bound,
            } => write!(
                f,
                "assertion node {} bounds {} {measured} measure (node {}) with {} \
                 {bound} expression",
                node.0,
                measured.article(),
                measure.0,
                bound.article()
            ),
            Self::AssertionTarget {
                node,
                measure,
                bound,
            } => write!(
                f,
                "assertion node {} carries {} {bound} bound against node {}, which is not a \
                 measure",
                node.0,
                bound.article(),
                measure.0
            ),
            Self::MetadataUnversioned { name, key, error } => write!(
                f,
                "metadata {key:?} on the {name} does not carry the D7 integer \
                 \"v\" version field: {error}"
            ),
        }
    }
}

/// Re-checks the document invariants `apply` maintains — on a parsed
/// snapshot (load) and on the in-memory snapshot (save) alike.
fn validate_snapshot(doc: &ProfileDoc) -> Result<(), SnapshotError> {
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
    // Every id in the document stays below the mint counter — replay
    // after load must never re-mint a referenced id.
    let check_id = |id: RecipeNodeId| -> Result<(), SnapshotError> {
        if id.0 >= doc.next_id {
            Err(SnapshotError::IdBeyondCounter {
                id,
                next_id: doc.next_id,
            })
        } else {
            Ok(())
        }
    };
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
                return Err(SnapshotError::DanglingInput { node: id, input });
            }
            if position.get(&input) >= position.get(&id) {
                return Err(SnapshotError::ForwardInput { node: id, input });
            }
        }
        // Every name-carrying payload's references, by the one list of
        // which payloads those are: an id past the counter inside a
        // mate head or a fillet selection is as corrupt as one inside a
        // Declare pair, and unrepairable by `Rebind` (whose source door
        // refuses a never-minted id) if it loads.
        for name in node.payload_names() {
            for n in derivation_nodes(name) {
                check_id(n)?;
            }
        }
        // And every node a reference is READ AT that is not also an
        // input (`Node::payload_read_sites` — a mate's two operands):
        // an id past the counter inside an operand is as corrupt as
        // one inside the name beside it, and as unrepairable.
        for at in node.payload_read_sites() {
            check_id(at)?;
        }
        // The placement RULE (GROUP-BOOLEAN-DESIGN), re-checked for the
        // same reason the A11 registry is below: a saved file is DATA,
        // and every rule on the wire must be one the edit door would
        // have accepted — one spelling of the count, at least one
        // placement, and frames that are finite and proper.
        if let Some(fault) = node.placement_rule_fault() {
            return Err(SnapshotError::PlacementRule { node: id, fault });
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
            return Err(SnapshotError::InputList { node: id, fault });
        }
        // The measurement vocabulary's two structural re-checks, for
        // the same reason the placement rule has one: a saved file is
        // DATA, and both of these are refused at the edit door.
        if let Some(fault) = node.measure_fault() {
            return Err(SnapshotError::MeasureRefs { node: id, fault });
        }
        // The declaration edge's kind rule
        // (`Node::bad_declare_input`, the same answer the edit door
        // asks of), for the reason above it: the edit door refuses a
        // `declare` input that is not a `Declare`, and a snapshot is
        // the one way a node reaches a document without passing that
        // door.
        if let Some(input) = node.bad_declare_input(doc) {
            return Err(SnapshotError::DeclareInput { node: id, input });
        }
        // An assertion's bound against the measure it constrains
        // (E10), by the same `Node::assertion_bound_fault` the edit
        // door asks: the predicate needs the DOCUMENT, so it takes one,
        // and this door only names its two answers.
        if let Some(fault) = node.assertion_bound_fault(doc) {
            return Err(match fault {
                AssertionBoundFault::TargetNotMeasure { measure, bound } => {
                    SnapshotError::AssertionTarget {
                        node: id,
                        measure,
                        bound,
                    }
                }
                AssertionBoundFault::DimensionMismatch {
                    measure,
                    measured,
                    bound,
                } => SnapshotError::AssertionBound {
                    node: id,
                    measure,
                    measured,
                    bound,
                },
            });
        }
        // ASM-R2a D-1: a mate's alignment is authored numbers a
        // predicate must be able to decide on. Asked in THIS walk, of
        // the same `Node::has_non_finite_alignment` the edit door asks
        // — a second pass over the nodes would be a second place to
        // forget the question.
        if node.has_non_finite_alignment() {
            return Err(SnapshotError::MateAlignment { node: id });
        }
    }
    for name in doc.appearance.keys() {
        for n in derivation_nodes(name) {
            check_id(n)?;
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
                WitnessSiteFault::NoSuchNode => SnapshotError::WitnessOnMissingNode { node },
                WitnessSiteFault::NotSketchBearing => SnapshotError::WitnessSite { node },
            });
        }
    }
    // The A11 placement registry (ASM-2A D-6): every key names a live
    // instantiate node, and every frame is one the edit door would
    // have accepted.
    for (&node, frame) in &doc.placements {
        check_id(node)?;
        if let Some(fault) = crate::doc::placement_fault(doc, node, frame) {
            return Err(match fault {
                PlacementFault::NotAnInstance => SnapshotError::PlacementSite { node },
                PlacementFault::NonFiniteFrame => SnapshotError::PlacementNonFinite { node },
                PlacementFault::ImproperFrame { determinant } => {
                    SnapshotError::PlacementImproper { node, determinant }
                }
            });
        }
        // The GAUGE rule is this door's alone, and that is the
        // invariant rather than a gap: `SetPlacement` KEYS a row on the
        // cluster's gauge instead of refusing a non-gauge key, and the
        // cluster maintenance re-keys the registry whenever the mate
        // graph moves, so a non-gauge row exists only in a file.
        let gauge = crate::mate::gauge_of(doc, node);
        if gauge != node {
            return Err(SnapshotError::PlacementNotGauge { node, gauge });
        }
    }
    // The A10 root invariants (ASM-ROOTS D-2), run AFTER the node
    // walk so a file with dangling inputs is diagnosed as such rather
    // than as an incidental coverage failure.
    crate::roots::check(doc).map_err(SnapshotError::Roots)?;
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
                    name: name.clone(),
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
    /// Geometry refusals and resolve failures deliberately PASS this
    /// door: they are V1 class 2, legal at rest, surfaced as typed
    /// node errors at evaluation.
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

    use crate::node::{Node, RecipeNodeId};
    use crate::persist::{PersistError, SnapshotError, save};
    use crate::program::ProfileDoc;
    use geom_core::Tol;

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
        label: &str,
        n: usize,
    ) -> (ProfileDoc, Vec<RecipeNodeId>) {
        let doc_ref = crate::ident::DocRef {
            id: crate::ident::DocumentId::derive("check-part"),
            pin: crate::ident::ContentPin::of_bytes(b"check-part"),
        };
        let mut doc = ProfileDoc::empty_derived(label, Tol::witness());
        let mut ids = Vec::new();
        for _ in 0..n {
            let applied = crate::edit::apply(
                &doc,
                &crate::edit::DocEdit::InsertNode {
                    node: Node::instantiate_part(doc_ref),
                },
                Tol::witness(),
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
        let mate = Node::Mate {
            a: crate::SitedRef::at_mint(name(ids[0])),
            b: crate::SitedRef::at_mint(name(ids[1])),
            class: topo::ContactClass::Rest,
            alignment: crate::mate::Alignment {
                a: crate::mate::MateFrame {
                    origin: [0.0; 3],
                    axis: [0.0, 0.0, 1.0],
                    reference: [1.0, 0.0, 0.0],
                },
                b: crate::mate::MateFrame {
                    origin: [0.0; 3],
                    axis: [0.0, 0.0, 1.0],
                    reference: [1.0, 0.0, 0.0],
                },
                primitive: crate::mate::MatePrimitive::FrameCoincidence,
                sense: crate::mate::AxisSense::Aligned,
                clocking: None,
            },
        };
        let applied = crate::edit::apply(
            &doc,
            &crate::edit::DocEdit::InsertNode { node: mate },
            Tol::witness(),
        )
        .expect("a finite alignment inserts");
        let mate_id = applied.record.minted.expect("the insert minted an id");
        doc = applied.doc;
        // Finite, the document saves — so the refusal below is the
        // poked coordinate's.
        save(&doc, &[], Tol::witness()).expect("the mated assembly saves");
        match doc.nodes.get_mut(&mate_id) {
            Some(Node::Mate { alignment, .. }) => alignment.a.origin[0] = f64::NAN,
            other => panic!("the fixture's mate is a mate, got {other:?}"),
        }
        match save(&doc, &[], Tol::witness()) {
            Err(PersistError::Snapshot(SnapshotError::MateAlignment { node })) => {
                assert_eq!(node, mate_id);
            }
            other => panic!("a non-finite alignment must refuse at save, got {other:?}"),
        }

        let (mut doc, ids) = instances_of_an_unresolved_reference("check-place", 1);
        doc.placements.insert(
            ids[0],
            crate::placement::Frame {
                columns: [[f64::NAN, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]],
                translation: [0.0; 3],
            },
        );
        match save(&doc, &[], Tol::witness()) {
            Err(PersistError::Snapshot(SnapshotError::PlacementNonFinite { node })) => {
                assert_eq!(node, ids[0]);
            }
            other => panic!("a non-finite placement must refuse at save, got {other:?}"),
        }
    }
}
