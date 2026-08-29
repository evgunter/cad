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
//!   `apply` on replay, which is the same door and the same check
//!   (the shape the alignment and placement notes below already take).
//! - [`first_program_fault`] — profile PROGRAM structure: per-slot
//!   dimension agreement (V2's role table) and a REPLAY PROBE under
//!   the document's params whose LATTICE violations refuse (the
//!   corrupt-file class — no authoring surface produces them);
//!   resolve failures and geometry refusals PASS this door (V1
//!   class 2: refusing programs may exist at rest — they surface as
//!   typed node errors at evaluation). The retired stored-joint walk
//!   died with stored joints: programs persist no derived values.
//! - [`validate_snapshot`] — the document invariants `apply`
//!   maintains, re-checked structurally (a parsed snapshot is not
//!   trusted; an in-memory one can be corrupted through the `pub`
//!   payload or an in-crate bug).
//!
//! The genuinely asymmetric residue stays at its door and is the
//! symmetry sweep's whole remit now: header/parse/position errors and
//! the wire-only canonical-set rule ([`super::wire`]) are load-only by
//! nature; serializer failure is save-only; ε reconciliation is
//! process state, not a document property. Log replayability is
//! shared STRUCTURALLY instead — both doors replay through
//! [`crate::edit::apply`].

use crate::appearance::AppearanceRecord;
use crate::distribution::{DistributionFault, DistributionField};
use crate::doc::{DocParam, ParamName};
use crate::edit::DocEdit;
use crate::expr::Dimension;
use crate::meta::MetaVersionError;
use crate::names::StableName;
use crate::node::SlotId;
use crate::node::{Node, RecipeNodeId};
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
        /// Which distribution offset is not finite, when the defect
        /// is in the ANNOTATION rather than in the nominal; `None`
        /// when it is the nominal itself. The walk has to identify
        /// the field to decide there is a defect at all, so it says
        /// which one rather than discarding the answer.
        field: Option<DistributionField>,
    },
    /// A float of a profile node's PLANE PLACEMENT (snapshot), by
    /// position among the 12 placement floats (columns c0, c1, c2,
    /// translation; x, y, z each). Program arguments are `Expr`s and
    /// carry the literal door's finiteness by construction.
    Profile {
        /// The profile node.
        node: RecipeNodeId,
        /// Index in the canonical float traversal.
        index: usize,
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
    /// A float of a profile payload carried by an `InsertNode` edit
    /// (the node id is minted only at replay, so the site is the
    /// float's traversal index alone).
    InsertedProfile {
        /// Index in the canonical float traversal.
        index: usize,
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
            Self::DocParam { name, field: None } => {
                write!(f, "document parameter {:?}", name.0)
            }
            Self::DocParam {
                name,
                field: Some(field),
            } => write!(
                f,
                "document parameter {:?}, distribution field {field}",
                name.0
            ),
            Self::Profile { node, index } => write!(
                f,
                "float {index} of the sketch-plane placement on profile node {}",
                node.0
            ),
            Self::Metadata { name, key, path } => write!(
                f,
                "metadata {key:?} on the {} named by node {}, at {path}",
                name.kind.noun(),
                name.node.0
            ),
            Self::InsertedProfile { index } => write!(
                f,
                "float {index} of an inserted profile's sketch-plane placement"
            ),
            Self::Edit { index, inner } => write!(f, "edit {index}, {inner}"),
        }
    }
}

/// The shared validator (module docs): every direction-independent
/// document check, in one place, invoked by both doors. Check order
/// is float walk → distribution walk → program walk → structural
/// invariants (the save door's historical precedence, pinned by the
/// refusal suite).
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
    if let Some((node, fault)) = first_program_fault(snapshot, tol) {
        return Err(super::PersistError::ProfileProgram { node, fault });
    }
    validate_snapshot(snapshot).map_err(super::PersistError::Snapshot)
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
    for (&id, node) in &snapshot.nodes {
        if let Node::Profile(desc) = node
            && let Some(index) = profile_non_finite(desc)
        {
            return Some(NonFiniteSite::Profile { node: id, index });
        }
    }
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

fn param_site(name: &ParamName, p: &DocParam) -> Option<NonFiniteSite> {
    let site = |field| NonFiniteSite::DocParam {
        name: name.clone(),
        field,
    };
    match p {
        DocParam::Continuous { value, .. } if !value.is_finite() => Some(site(None)),
        // The distribution's offsets are floats the format writes, so
        // they belong to THIS walk rather than to a second spelling of
        // the same defect; the shape invariants are
        // `first_distribution_fault`'s. The offending field rides
        // along: the walk computes it to answer at all, and a
        // diagnostic that names `sigma` beats one that names only the
        // parameter.
        DocParam::Continuous {
            distribution: Some(d),
            ..
        } if d.first_non_finite().is_some() => Some(site(d.first_non_finite())),
        // EXHAUSTIVE on purpose: a guarded arm does not count towards
        // exhaustiveness, so the finite `Continuous` case is spelled
        // out alongside the float-free ones rather than swept up by a
        // wildcard that would also swallow a future float carrier.
        DocParam::Continuous { .. } | DocParam::Count { .. } => None,
    }
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

/// Walks the program payload's RAW floats — exactly the 12 plane
/// placement values (program arguments are `Expr`s, whose literals
/// are finite by the construction door).
fn profile_non_finite(program: &ProfileProgram) -> Option<usize> {
    let a = &program.plane.placement;
    [a.linear.c0, a.linear.c1, a.linear.c2, a.translation]
        .iter()
        .flat_map(|v| [v.x, v.y, v.z])
        .position(|f| !f.is_finite())
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
        DocEdit::InsertNode {
            node: Node::Profile(program),
        } => profile_non_finite(program).map(|index| NonFiniteSite::InsertedProfile { index }),
        DocEdit::SetDocParam { name, value } => param_site(name, value),
        // The value door carries no distribution of its own — the
        // declaration it writes into supplies that — but its
        // continuous arm IS a raw float the format writes.
        DocEdit::SetDocParamValue {
            name,
            value: crate::doc::DocParamValue::Continuous(v),
        } if !v.is_finite() => Some(NonFiniteSite::DocParam {
            name: name.clone(),
            field: None,
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
        | DocEdit::InsertNode { .. }
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
    /// A `Node::Fillet` selection is not in canonical form (sorted
    /// and deduplicated) — a corrupt file, refused rather than
    /// repaired (M6-5).
    FilletSelectionNotCanonical {
        /// The offending fillet node.
        node: RecipeNodeId,
    },
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
    /// A witness attached to a missing or non-sketch-bearing node.
    WitnessSite {
        /// The offending node id.
        node: RecipeNodeId,
    },
    /// A continuous parameter declared with the Count dimension
    /// (`apply` refuses this; a file must not smuggle it).
    CountContinuous {
        /// The parameter.
        name: ParamName,
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
    /// A placement frame a file must not carry: non-finite, or
    /// improper (determinant ≤ 0, the A6 mirror case R4 gates). The
    /// edit door refuses both, so a file holding one is corrupt —
    /// refused, never repaired.
    PlacementFrame {
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
    /// An assertion whose bound is dimensioned differently from the
    /// measure it constrains, or which references something that is
    /// not a measure at all (E10). The edit door refuses both; a file
    /// carrying one is data the edit door would never have produced.
    AssertionBound {
        /// The offending assertion.
        node: RecipeNodeId,
        /// What it references.
        measure: RecipeNodeId,
        /// The measure's dimension, absent when the reference is not a
        /// measure at all.
        measured: Option<crate::expr::Dimension>,
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
// `StableName` renders as its entity noun plus its minting node,
// which is the document layer's spelling of a name.
impl core::fmt::Display for SnapshotError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::OrderMismatch => f.write_str(
                "the `order` list and the node map disagree — an id is missing, extra or \
                 duplicated",
            ),
            Self::FilletSelectionNotCanonical { node } => write!(
                f,
                "fillet node {}'s selection is not sorted and deduplicated — a corrupt \
                 selection is refused, never repaired",
                node.0
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
            Self::WitnessSite { node } => write!(
                f,
                "a witness is attached to node {}, which is missing or bears no sketch",
                node.0
            ),
            Self::CountContinuous { name } => write!(
                f,
                "continuous parameter {:?} is declared with the Count dimension",
                name.0
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
            Self::PlacementFrame { node, determinant } => write!(
                f,
                "the placement frame on node {} is non-finite or improper (determinant \
                 {determinant})",
                node.0
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
            Self::MeasureRefs { node, fault } => {
                write!(f, "measure node {}: {fault}", node.0)
            }
            Self::AssertionBound {
                node,
                measure,
                measured: Some(measured),
                bound,
            } => write!(
                f,
                "assertion node {} bounds a {measured:?} measure (node {}) with a {bound:?} \
                 expression",
                node.0, measure.0
            ),
            Self::AssertionBound {
                node,
                measure,
                measured: None,
                bound,
            } => write!(
                f,
                "assertion node {} carries a {bound:?} bound against node {}, which is not a \
                 measure",
                node.0, measure.0
            ),
            Self::MetadataUnversioned { name, key, error } => write!(
                f,
                "metadata {key:?} on the {} named by node {} does not carry the D7 integer \
                 \"v\" version field: {error}",
                name.kind.noun(),
                name.node.0
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
    if !(doc.epsilon.is_finite() && doc.epsilon > 0.0) {
        return Err(SnapshotError::EpsilonInvalid { value: doc.epsilon });
    }
    for (name, p) in &doc.params {
        if matches!(
            p,
            DocParam::Continuous {
                dim: Dimension::Count,
                ..
            }
        ) {
            return Err(SnapshotError::CountContinuous { name: name.clone() });
        }
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
        // The fillet selection carries one check of its own (M6-5): the
        // canonical form. `Node::fillet` is the only construction door
        // and it canonicalizes, so a non-canonical selection on the
        // wire is a CORRUPT file — refused, never quietly re-sorted (a
        // repair would change the node's content key behind the
        // caller's back).
        if let Node::Fillet { selection, .. } = node
            && selection.windows(2).any(|w| w[0] >= w[1])
        {
            return Err(SnapshotError::FilletSelectionNotCanonical { node: id });
        }
        // The placement RULE (GROUP-BOOLEAN-DESIGN), re-checked for the
        // same reason the A11 registry is below: a saved file is DATA,
        // and every rule on the wire must be one the edit door would
        // have accepted — one spelling of the count, at least one
        // placement, and frames that are finite and proper.
        if let Some(fault) = node.placement_rule_fault() {
            return Err(SnapshotError::PlacementRule { node: id, fault });
        }
        // The measurement vocabulary's two structural re-checks, for
        // the same reason the placement rule has one: a saved file is
        // DATA, and both of these are refused at the edit door.
        if let Some(fault) = node.measure_fault() {
            return Err(SnapshotError::MeasureRefs { node: id, fault });
        }
        if let Node::Assertion { measure, bound, .. } = node {
            let measured = match doc.nodes.get(measure) {
                Some(Node::Measure { expr, .. }) => Some(expr.dim()),
                _ => None,
            };
            if measured != Some(bound.dim()) {
                return Err(SnapshotError::AssertionBound {
                    node: id,
                    measure: *measure,
                    measured,
                    bound: bound.dim(),
                });
            }
        }
    }
    for name in doc.appearance.keys() {
        for n in derivation_nodes(name) {
            check_id(n)?;
        }
    }
    for &node in doc.witnesses.keys() {
        check_id(node)?;
        if !matches!(doc.nodes.get(&node), Some(Node::Profile(_))) {
            return Err(SnapshotError::WitnessSite { node });
        }
    }
    // The A11 placement registry (ASM-2A D-6): every key names a live
    // instantiate node, and every frame is one the edit door would
    // have accepted.
    for (&node, frame) in &doc.placements {
        check_id(node)?;
        if !matches!(doc.nodes.get(&node), Some(Node::InstantiatePart { .. })) {
            return Err(SnapshotError::PlacementSite { node });
        }
        let determinant = frame.determinant();
        if !frame.is_finite() || determinant <= 0.0 {
            return Err(SnapshotError::PlacementFrame { node, determinant });
        }
        let gauge = crate::mate::gauge_of(doc, node);
        if gauge != node {
            return Err(SnapshotError::PlacementNotGauge { node, gauge });
        }
    }
    // ASM-R2a D-1: a mate's alignment is authored numbers a predicate
    // must be able to decide on.
    for (&node, n) in &doc.nodes {
        if let Node::Mate { alignment, .. } = n
            && !alignment.is_finite()
        {
            return Err(SnapshotError::MateAlignment { node });
        }
    }
    // The A10 root invariants (ASM-ROOTS D-2), run AFTER the node
    // walk so a file with dangling inputs is diagnosed as such rather
    // than as an incidental coverage failure.
    crate::roots::check(doc).map_err(SnapshotError::Roots)?;
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
/// successor at the program layer): a wrong-dimension argument, or a
/// lattice-violating step order. Both are the corrupt-file class — the
/// payload is `pub`, so an in-crate bug can also build one; both doors
/// refuse with the same diagnostics.
#[derive(Debug, Clone, PartialEq)]
pub enum ProgramFault {
    /// A program slot's expression has the wrong dimension for its
    /// role (V2's table, [`crate::StepArg::dimension`]).
    SlotDimension {
        /// The offending slot.
        slot: SlotId,
        /// The role's required dimension.
        expected: crate::expr::Dimension,
        /// The expression's dimension.
        found: crate::expr::Dimension,
    },
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
// could not follow it; the vocabulary tokens are the location, and
// the typed variant remains the machine contract.
impl core::fmt::Display for ProgramFault {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            // A program fault's slot is a program slot, whose address
            // is spelled out rather than dumped: `SlotId::Profile`'s
            // three fields are the location, and a derived rendering
            // would put the struct's own braces in a user's message.
            Self::SlotDimension {
                slot: SlotId::Profile { loop_, step, arg },
                expected,
                found,
            } => write!(
                f,
                "loop {loop_} step {step}'s {arg:?} argument needs a {expected:?} \
                 expression, got {found:?}"
            ),
            Self::SlotDimension {
                slot,
                expected,
                found,
            } => write!(
                f,
                "slot {slot:?} needs a {expected:?} expression, got {found:?}"
            ),
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
                    Some(verb) => write!(f, "a {verb:?} verb at tip state {state:?}"),
                    None => write!(f, "the chain is unclosed at tip state {state:?}"),
                }
            }
        }
    }
}

/// The first program fault in the SNAPSHOT's profile nodes (module
/// docs). The edit log needs no twin: logged edits replay through
/// `apply`, whose own doors (slot dimension checks + the VQ9
/// authoring-time check) refuse the same faults at the same load.
///
/// A program whose Exprs fail to RESOLVE under the document's params
/// (dangling ref, dimension drift) cannot be probed here and PASSES
/// this walk — deliberately: resolution failures are the same
/// binding-dependent class as geometry refusals (V1 class 2) and
/// surface as the node's typed evaluation error; no silent acceptance
/// exists (review NOTE-1).
fn first_program_fault(snapshot: &ProfileDoc, tol: Tol) -> Option<(RecipeNodeId, ProgramFault)> {
    use crate::program::ProfilePayload as _;
    let env = snapshot.param_env::<f64>();
    for (&id, node) in &snapshot.nodes {
        let Node::Profile(program) = node else {
            continue;
        };
        for slot in program.slots() {
            let Some(expr) = crate::program::ProfilePayload::expr(program, slot) else {
                continue;
            };
            if expr.dim() != slot.dimension() {
                return Some((
                    id,
                    ProgramFault::SlotDimension {
                        slot,
                        expected: slot.dimension(),
                        found: expr.dim(),
                    },
                ));
            }
        }
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
    #![allow(clippy::panic)]

    use crate::node::RecipeNodeId;
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
}
