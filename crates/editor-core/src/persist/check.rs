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
//! this, said once instead of at each: a `DeclareVar` in the log
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
use crate::doc::{
    DocParamField, ExpansionFault, FreeVar, GaugeRefFault, VarName, VarReadFault, WitnessSiteFault,
};
use crate::edit::DocEdit;
use crate::expr::Expr;
use crate::meta::MetaVersionError;
use crate::node::SlotId;
use crate::node::{AssertionBoundFault, Node, RecipeNodeId};
use crate::placement::{FrameFault, FrameSite};
use crate::program::{ProfileDoc, ProfileProgram, ProgramRefusal};
use crate::resolve::derivation_nodes;
use crate::spoken::SpokenVar;
use crate::spoken::{SpokenName, SpokenNode};
use crate::var::{VarId, VarKind, VarRef};
use geom_core::Tol;

/// Where a non-finite float sits (the D2 refusal's typed site).
#[derive(Debug, Clone, PartialEq)]
pub enum NonFiniteSite {
    /// The recorded ε.
    Epsilon,
    /// A continuous free variable: one the snapshot holds, by its name
    /// where it has one, or one a logged edit addresses, as the edit
    /// addresses it.
    DocParam {
        /// The variable.
        var: VarRef,
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
                var,
                field: DocParamField::Nominal,
            } => write!(f, "{}", SiteVar(var)),
            Self::DocParam { var, field } => {
                write!(f, "{}, {field}", SiteVar(var))
            }
            Self::Metadata { name, key, path } => {
                write!(f, "metadata {key:?} on {name}, at {path}")
            }
            Self::Edit { index, inner } => write!(f, "edit {index}, {inner}"),
        }
    }
}

/// A site's variable as prose: `variable w`, or the bare address's own
/// `variable <tag>`.
struct SiteVar<'a>(&'a VarRef);

impl core::fmt::Display for SiteVar<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self.0 {
            VarRef::Name(name) => write!(f, "variable {name}"),
            VarRef::Id(_) => write!(f, "{}", self.0),
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
    /// Expression literals are finite BY CONSTRUCTION (`Formula::literal`
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
    /// [`first_distribution_fault`] over the variable table: the E2
    /// invariants of every doc param's distribution beyond finiteness,
    /// by the same `Distribution::check` the edit door runs. Snapshot
    /// only.
    Distribution,
    /// [`first_display_unit_fault`] over the variable table: every
    /// document parameter's authored display unit measures the
    /// dimension it was declared with. A literal needs no twin walk
    /// (`Formula::literal_with_unit` makes the pairing at construction and
    /// the load side re-runs it); a `FreeVar` does, because its
    /// payload is `pub` and its dimension is data. Snapshot only.
    DisplayUnit,
    /// [`first_var_fault`] over the variable table (VARIABLES-DESIGN
    /// VR1–VR3): the mint log it asks counts up from one, every
    /// variable's stored kind is its definition's,
    /// every id is logged in the mint as a variable's, every name sits
    /// on a live variable, and no name is held twice. Snapshot only.
    Vars,
    /// [`first_output_fault`] over the variable table and the node map
    /// (D10): every output row names a live node, a port of its
    /// signature and that port's kind, no port has two rows, and every
    /// live node has a row for each of its ports. Snapshot only.
    OutputSignature,
    /// [`first_definition_read_fault`] over every defined variable's
    /// definition: it holds no name leaf, and every variable it reads
    /// is one the mint log holds, read at its kind when live — the
    /// predicate the edit doors ask (`Doc::var_read_faults`). A
    /// definition reading a deleted variable is legal, as a slot's
    /// reader is (VR7). Snapshot only.
    DefinitionRead,
    /// [`first_definition_cycle`] over the variable table (VR3): no
    /// definition reads its own variable back, and no variable's
    /// expansion outgrows [`crate::edit::DEFINITION_NODE_BOUND`].
    /// After the read walk, so a cycle is a cycle of live variables.
    /// Snapshot only.
    DefinitionCycle,
    /// [`first_slot_read_fault`] over every slot expression's readers,
    /// by the same `Doc::var_read_faults` the edit doors ask: a reader
    /// names a minted variable, and a live one at its kind. A reader of
    /// a variable minted and since deleted is legal — it is that
    /// variable's unresolved reader (VR7), refused at evaluation.
    /// Snapshot only.
    SlotRead,
    /// [`first_payload_read_fault`] over the readers of every
    /// expression no slot addresses, asking that same one predicate.
    ///
    /// **Why this is a second walk rather than a wider domain for
    /// [`Walk::SlotRead`]**: the two answer at different ADDRESSES — a
    /// slot fault names the slot, a payload fault has only the node to
    /// name — so folding them would mint an address that is sometimes
    /// absent, and would erase the order between them, which
    /// [`validate_document`] holds as a contract.
    PayloadRead,
    /// [`first_operand_read_fault`] over every operand's read (D10): it
    /// names a minted variable, and a live one of a kind its seat
    /// admits that is no world placement's copy. A read of an output
    /// deleted since is legal — its reader is
    /// stranded, refused at evaluation. Snapshot only.
    OperandRead,
    /// [`first_unnamed_reader_fault`] over the variable table: a
    /// variable with no name has exactly one reader (VR2, VR9).
    /// Snapshot only.
    UnnamedReader,
    /// [`first_program_fault`] over the profile programs' replay: a
    /// REPLAY PROBE under the document's params whose LATTICE
    /// violations refuse (the corrupt-file class — no authoring surface
    /// produces them). Resolve failures and geometry refusals PASS this
    /// door (V1 class 2: refusing programs may exist at rest — they
    /// surface as typed node errors at evaluation). A step argument's
    /// kind is [`Walk::SlotRead`]'s, not a second spelling here.
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
    pub(crate) const ORDER: [Walk; 13] = [
        Walk::NonFinite,
        Walk::Distribution,
        Walk::DisplayUnit,
        Walk::Vars,
        Walk::OutputSignature,
        Walk::DefinitionRead,
        Walk::DefinitionCycle,
        Walk::SlotRead,
        Walk::PayloadRead,
        Walk::OperandRead,
        Walk::Program,
        Walk::Snapshot,
        Walk::UnnamedReader,
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
                .map(|(var, fault)| super::PersistError::Distribution { var, fault }),
            Walk::DisplayUnit => first_display_unit_fault(snapshot).map(|(var, unit, declared)| {
                super::PersistError::DisplayUnit {
                    var,
                    unit,
                    declared,
                }
            }),
            Walk::Vars => first_var_fault(snapshot).map(super::PersistError::Snapshot),
            Walk::OutputSignature => {
                first_output_fault(snapshot).map(super::PersistError::Snapshot)
            }
            Walk::DefinitionRead => {
                first_definition_read_fault(snapshot).map(super::PersistError::Snapshot)
            }
            Walk::DefinitionCycle => {
                first_definition_cycle(snapshot).map(super::PersistError::Snapshot)
            }
            Walk::SlotRead => first_slot_read_fault(snapshot).map(|(node, slot, fault)| {
                read_refusal(
                    snapshot,
                    snapshot.spoken(node),
                    ReadAddress::Slot(slot),
                    fault,
                )
            }),
            Walk::PayloadRead => first_payload_read_fault(snapshot).map(|(node, fault)| {
                read_refusal(snapshot, snapshot.spoken(node), ReadAddress::Payload, fault)
            }),
            Walk::OperandRead => {
                first_operand_read_fault(snapshot).map(super::PersistError::Snapshot)
            }
            Walk::UnnamedReader => {
                first_unnamed_reader_fault(snapshot).map(super::PersistError::Snapshot)
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
/// that document. These are, each with the row that says so:
///
/// - [`Walk::SlotRead`] before [`Walk::PayloadRead`]
///   (`load_door_payload_param_ref::a_document_broken_in_a_slot_and_in_a_payload_reads_the_slot_refusal`).
/// - Both read walks before [`Walk::Snapshot`]
///   (`…::an_assertion_bound_on_a_non_measure_reads_the_payload_refusal`,
///   and `rv_onepred3_probes::rv_the_slot_walk_shadows_a_structural_refusal_it_did_not_shadow_before`
///   for the slot half): a node can carry a broken param reference AND
///   a structurally invalid shape, and the param-table answer names the
///   parameter while the structural one does not.
///
/// [`Walk::UnnamedReader`] comes last: how many readers a variable has
/// is a question about the readers, so it is asked once every reader is
/// known to be well-formed — a slot a structurally broken node no
/// longer addresses leaves its variable unread, and the structural
/// fault is the answer
/// (`rv_payloadrefs_probes::rv_an_expression_no_walk_reads_is_refused_structurally_not_as_a_param_ref`).
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

/// **Where a read fault was found** — the ONE thing the two read walks
/// differ in, and therefore the only argument [`read_refusal`] needs
/// beside the fault itself.
enum ReadAddress {
    /// A SLOT expression, at the slot that addresses it
    /// ([`Walk::SlotRead`]).
    Slot(SlotId),
    /// An expression no slot addresses ([`crate::node::payload_exprs`],
    /// [`Walk::PayloadRead`]): the node is the whole address.
    Payload,
}

/// **The read walks' answer, in the load door's vocabulary** — one
/// mapper for both walks, because the FAULT is one vocabulary
/// ([`VarReadFault`], the one predicate both doors ask) and only the
/// address differs. The walks hand it only the faults this door
/// refuses: an unminted id, or a live variable read at another kind.
fn read_refusal(
    snapshot: &ProfileDoc,
    node: SpokenNode,
    address: ReadAddress,
    fault: VarReadFault,
) -> super::PersistError {
    super::PersistError::Snapshot(match (address, fault) {
        (_, VarReadFault::Unminted { var }) => SnapshotError::ReaderOfUnmintedVar { node, var },
        (
            ReadAddress::Slot(slot),
            VarReadFault::Kind {
                var,
                declared,
                referenced,
            },
        ) => SnapshotError::SlotVarKind {
            node,
            slot,
            var: Box::new(snapshot.spoken_var(var)),
            found: declared,
            expected: crate::SlotKind::Is(VarKind::from(referenced)),
        },
        (
            ReadAddress::Payload,
            VarReadFault::Kind {
                var,
                declared,
                referenced,
            },
        ) => SnapshotError::PayloadVarKind {
            node,
            var: snapshot.spoken_var(var),
            declared,
            referenced,
        },
        (_, VarReadFault::Dead { .. }) => {
            unreachable!("the read walks pass on a dead reader")
        }
    })
}

/// The first fault of `expr`'s readers this door refuses: a dead
/// reader is legal.
fn refused_read(snapshot: &ProfileDoc, expr: &Expr) -> Option<VarReadFault> {
    snapshot.var_read_faults(expr).into_iter().find(|fault| {
        matches!(
            fault,
            VarReadFault::Unminted { .. } | VarReadFault::Kind { .. }
        )
    })
}

/// The first document parameter whose authored display unit does not
/// MEASURE its declared dimension, as `(name, what the unit measures,
/// what was declared)`, or `None`.
///
/// The SNAPSHOT only, for the reason `first_distribution_fault` walks
/// it alone: a `DeclareVar` in the log carries its definition through
/// `apply` on replay, and a replayed document is a snapshot this same
/// validator sees.
///
/// Expression literals need no twin walk — `Formula::literal_with_unit`
/// checks the pairing at construction and the load side re-runs that
/// same constructor, so a literal cannot reach a document mismatched.
/// A `FreeVar` has no such door to make total: its payload is `pub`
/// and its dimension is data, which is exactly the asymmetry this walk
/// covers.
fn first_display_unit_fault(
    snapshot: &ProfileDoc,
) -> Option<(SpokenVar, crate::expr::Dimension, crate::expr::Dimension)> {
    free_vars(snapshot).find_map(|(id, p)| match p {
        FreeVar::Continuous {
            dim, display_unit, ..
        } => {
            // The SAME reading the edit door and the literal
            // constructor make (`UnitSym::measures`): what a unit
            // measures is one fact, stated once.
            let measured = display_unit.measures();
            (measured != *dim).then(|| (snapshot.spoken_var(id), measured, *dim))
        }
        FreeVar::Count { .. } => None,
    })
}

/// The snapshot's free variables, by id.
fn free_vars(snapshot: &ProfileDoc) -> impl Iterator<Item = (VarId, &FreeVar)> {
    snapshot
        .vars
        .iter()
        .filter_map(|(&id, var)| Some((id, var.free()?)))
}

/// **The first output row that disagrees with its operation's
/// signature**, or the first port of a live node with no row (D10):
/// the rows in id order, then the live nodes in id order, each by port.
/// A placer whose operand chain is not live has no kind to check its
/// port against, and the structural walk, which runs after, refuses
/// the chain: [`SnapshotError::OperandUnminted`] for a read of a
/// variable never minted. A read a delete stranded loads, and refuses
/// at evaluation (`NodeErrorKind::UnresolvedRead`).
fn first_output_fault(snapshot: &ProfileDoc) -> Option<SnapshotError> {
    let mut rows: std::collections::BTreeMap<(RecipeNodeId, u8), VarId> =
        std::collections::BTreeMap::new();
    for (&id, var) in &snapshot.vars {
        let Some((node, port)) = var.def().output() else {
            continue;
        };
        let fault = |fault| SnapshotError::OutputSignature {
            node: snapshot.spoken(node),
            fault: Box::new(fault),
        };
        let Some(live) = snapshot.nodes.get(&node) else {
            return Some(SnapshotError::OutputSignature {
                node: crate::spoken::SpokenNode::absent(node),
                fault: Box::new(OutputFault::NodeAbsent {
                    var: snapshot.spoken_var(id),
                }),
            });
        };
        let ports = live.outputs();
        let Some(signed) = ports.get(usize::from(port)) else {
            return Some(fault(OutputFault::PortOutside {
                var: snapshot.spoken_var(id),
                port,
                ports: ports.len(),
            }));
        };
        if let Some(kind) = snapshot
            .signature(node)
            .and_then(|sig| sig.get(usize::from(port)).map(|&(_, kind)| kind))
            && kind != var.kind()
        {
            return Some(fault(OutputFault::Kind {
                var: snapshot.spoken_var(id),
                port: signed.name,
                stored: var.kind(),
                signature: kind,
            }));
        }
        if let Some(&first) = rows.get(&(node, port)) {
            return Some(fault(OutputFault::Twice {
                port: signed.name,
                first: snapshot.spoken_var(first),
                second: snapshot.spoken_var(id),
            }));
        }
        rows.insert((node, port), id);
    }
    for (&node, live) in &snapshot.nodes {
        for (port, signed) in (0u8..).zip(live.outputs()) {
            if !rows.contains_key(&(node, port)) {
                return Some(SnapshotError::OutputSignature {
                    node: snapshot.spoken(node),
                    fault: Box::new(OutputFault::Missing { port: signed.name }),
                });
            }
        }
    }
    None
}

/// The first fault of the variable table itself, in the order the
/// variable is built: its kind against its definition, its id against
/// the mint log, then the names — each on a live variable, none held
/// twice. The names are walked by id, so the pair a twice-held name
/// reports is the two lowest ids holding it.
fn first_var_fault(snapshot: &ProfileDoc) -> Option<SnapshotError> {
    // The mint log first, since `has_var` below asks it: its ordinals
    // count up from one, the only log a mint writes. The structural walk asks
    // it again for a snapshot checked alone (`validate_snapshot`).
    if let Some(entry) = snapshot.mint.out_of_order() {
        return Some(SnapshotError::MintLogOrder { entry });
    }
    for (&id, var) in &snapshot.vars {
        if let Some(def) = var.def().kind().filter(|_| !var.kind_holds()) {
            return Some(SnapshotError::VarKind {
                var: snapshot.spoken_var(id),
                kind: var.kind(),
                def,
            });
        }
        if !snapshot.mint.has_var(id) {
            return Some(SnapshotError::VarNotMinted {
                var: snapshot.spoken_var(id),
            });
        }
    }
    let mut held: std::collections::BTreeMap<&VarName, VarId> = std::collections::BTreeMap::new();
    for (&id, name) in &snapshot.var_names {
        if !snapshot.vars.contains_key(&id) {
            return Some(SnapshotError::NameOnMissingVar {
                var: id,
                name: name.clone(),
            });
        }
        if let Some(&a) = held.get(name) {
            return Some(SnapshotError::VarNameTwice {
                name: name.clone(),
                a,
                b: id,
            });
        }
        held.insert(name, id);
    }
    None
}

/// The first defined variable, in declaration order, whose definition
/// reads what no door could have written ([`Walk::DefinitionRead`]).
fn first_definition_read_fault(snapshot: &ProfileDoc) -> Option<SnapshotError> {
    snapshot.vars.iter().find_map(|(&id, var)| {
        let expr = var.def().defined()?;
        let var = snapshot.spoken_var(id);
        snapshot
            .var_read_faults(expr)
            .into_iter()
            .find_map(|fault| match fault {
                VarReadFault::Unminted { var: read } => {
                    Some(SnapshotError::DefinitionReadsUnmintedVar {
                        var: var.clone(),
                        read,
                    })
                }
                VarReadFault::Kind {
                    var: read,
                    declared,
                    referenced,
                } => Some(SnapshotError::DefinitionVarKind {
                    var: var.clone(),
                    read: snapshot.spoken_var(read),
                    declared,
                    referenced,
                }),
                // Answered above, and a deleted variable's reader is
                // legal (VR7).
                VarReadFault::Dead { .. } => None,
            })
    })
}

/// The first variable, in declaration order, whose definition reads it
/// back, or whose expansion outgrows the bound
/// ([`Walk::DefinitionCycle`]): the search the edit door asks
/// ([`crate::Doc::expansion_fault`]).
fn first_definition_cycle(snapshot: &ProfileDoc) -> Option<SnapshotError> {
    Some(match snapshot.expansion_fault()? {
        ExpansionFault::Cycle { var, through } => SnapshotError::DefinitionCycle {
            var: snapshot.spoken_var(var),
            through: through
                .into_iter()
                .map(|v| snapshot.spoken_var(v))
                .collect(),
        },
        ExpansionFault::TooLarge { var, nodes } => SnapshotError::DefinitionTooLarge {
            var: snapshot.spoken_var(var),
            nodes,
        },
    })
}

/// The first slot expression with a reader this door refuses, by the
/// ONE predicate the edit doors ask ([`crate::Doc::var_read_faults`]).
///
/// Runs after the dimension walk, so a slot broken both ways is
/// diagnosed at its own address first. A reader that does not RESOLVE
/// — its variable deleted — is not the same class as one the table
/// refuses: it is legal at rest and refuses at evaluation (VR7), while
/// an unminted id and a kind the variable does not have are facts about
/// the document itself.
fn first_slot_read_fault(snapshot: &ProfileDoc) -> Option<(RecipeNodeId, SlotId, VarReadFault)> {
    snapshot.nodes.iter().find_map(|(&id, node)| {
        node.rows().into_iter().find_map(|(slot, &var)| {
            let reader = Expr::var(var, slot.expr_dimension());
            Some((id, slot, refused_read(snapshot, &reader)?))
        })
    })
}

/// **The first operand read this door refuses**, in document order and
/// within a node in field order: a read of a variable the mint log
/// does not hold, or a live read [`crate::Doc::read_fault`] — the edit
/// doors' rule, stated once — refuses: a kind its seat does not admit,
/// or a world placement's copy. A placer's read
/// keeping its output's shape is [`Walk::OutputSignature`]'s.
fn first_operand_read_fault(snapshot: &ProfileDoc) -> Option<SnapshotError> {
    snapshot.nodes.iter().find_map(|(&id, node)| {
        node.operand_rows()
            .into_iter()
            .find_map(|(slot, var)| {
                if !snapshot.has_minted_var(var) {
                    return Some(SnapshotError::OperandUnminted {
                        node: snapshot.spoken(id),
                        slot,
                        var: snapshot.spoken_var(var),
                    });
                }
                // A minted read no live operation defines is a strand the
                // file keeps (DM7), the reader's refusal at evaluation.
                let held = snapshot.var(var)?;
                Some(match snapshot.read_fault(held, slot.kind())? {
                    crate::doc::ReadFault::Kind { found } => SnapshotError::SlotVarKind {
                        node: snapshot.spoken(id),
                        slot: SlotId::Operand(slot),
                        var: Box::new(snapshot.spoken_var(var)),
                        found,
                        expected: slot.kind(),
                    },
                    crate::doc::ReadFault::WorldCopy { placement } => {
                        SnapshotError::ReadsWorldCopy {
                            node: snapshot.spoken(id),
                            slot: SlotId::Operand(slot),
                            placement: snapshot.spoken(placement),
                        }
                    }
                })
            })
            .or_else(|| {
                let placement = node
                    .measure_sites()
                    .into_iter()
                    .find_map(|at| crate::edit::world_copy_site(snapshot, at))?;
                Some(SnapshotError::MeasuresWorldCopy {
                    node: snapshot.spoken(id),
                    placement: snapshot.spoken(placement),
                })
            })
    })
}

/// The first PAYLOAD expression with a reader this door refuses, as
/// `(node, fault)`, by the same predicate.
///
/// The expressions no slot addresses ([`crate::node::payload_exprs`]):
/// a [`crate::Node::Measure`]'s measured expression leaves and a
/// [`crate::Node::Assertion`]'s bound. The address reported is the
/// NODE, because that is the address the expression has. Runs after the
/// slot read walk, so a document broken in a slot AND in a payload is
/// diagnosed at the slot, which is the address that carries more.
///
/// **The domain's edge, stated because it is not empty.** A node's rows
/// and [`crate::node::payload_exprs`] together do NOT reach every
/// `Expr` a node can hold: a `Node::Pattern`'s or `Node::PlacedUnion`'s
/// COUNT expression under an `Explicit` rule is addressed by no slot
/// and is no payload either. Such a file is still refused — by
/// [`Walk::Snapshot`], as `PlacementRule` with
/// `PlacementRuleFault::CountSpelling` — so no document reaches memory
/// carrying an unchecked reader; what it does not get is this walk's
/// sentence.
/// `rv_payloadrefs_probes::rv_an_expression_no_walk_reads_is_refused_structurally_not_as_a_param_ref`
/// is the row that pins that.
fn first_payload_read_fault(snapshot: &ProfileDoc) -> Option<(RecipeNodeId, VarReadFault)> {
    snapshot.nodes.iter().find_map(|(&id, node)| {
        node.payload_reads(snapshot)
            .into_iter()
            .flatten()
            .find_map(|(var, dim)| Some((id, refused_read(snapshot, &Expr::var(var, dim))?)))
    })
}

/// The first variable, in declaration order, that has no name and that
/// nothing reads, then the first with no name and more than one reader
/// ([`Walk::UnnamedReader`]).
fn first_unnamed_reader_fault(snapshot: &ProfileDoc) -> Option<SnapshotError> {
    if let Some(&var) = snapshot.unread_anonymous_vars().first() {
        return Some(SnapshotError::AnonymousVarUnread {
            var: snapshot.spoken_var(var),
        });
    }
    let var = *snapshot.shared_unnamed_vars().first()?;
    Some(SnapshotError::SharedVarNeedsName {
        var: snapshot.spoken_var(var),
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
    for (id, p) in free_vars(snapshot) {
        let var = snapshot
            .var_name(id)
            .map_or(VarRef::Id(id), |name| VarRef::Name(name.clone()));
        if let Some(site) = param_site(var, p) {
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
/// predicate `FreeVar::first_non_finite` decides, rendered as the
/// site vocabulary a document author reads. The distribution's offsets
/// belong to this walk rather than to a second spelling of the same
/// defect — the shape invariants are `first_distribution_fault`'s.
fn param_site(var: VarRef, p: &FreeVar) -> Option<NonFiniteSite> {
    Some(NonFiniteSite::DocParam {
        var,
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
fn first_distribution_fault(snapshot: &ProfileDoc) -> Option<(SpokenVar, DistributionFault)> {
    free_vars(snapshot).find_map(|(id, p)| match p.distribution()?.check() {
        Ok(()) => None,
        Err(fault) => Some((snapshot.spoken_var(id), fault)),
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
        DocEdit::DeclareVar {
            name,
            def: crate::var::VarDecl::Free(value),
        } => param_site(VarRef::Name(name.clone()), value),
        DocEdit::DefineVar {
            var,
            def: crate::var::VarDecl::Free(value),
            ..
        } => param_site(var.clone(), value),
        // The value door carries no distribution of its own — the
        // declaration it writes into supplies that — but its
        // continuous arm IS a raw float the format writes.
        //
        // It is the one float site here that cannot delegate to
        // `FreeVar::first_non_finite`: the payload is a bare `f64`,
        // not a `FreeVar`, so there is no parameter for the shared
        // predicate to read. What it shares with the walk over the
        // table is the SITE vocabulary, which is what a reader
        // comparing the two refusals sees.
        DocEdit::SetVarValue {
            var,
            value: crate::doc::FreeValue::Continuous(v),
        } if !v.is_finite() => Some(NonFiniteSite::DocParam {
            var: var.clone(),
            field: DocParamField::Nominal,
        }),
        // The annotation door's whole payload is a distribution, so
        // its offsets are floats the format writes and they belong to
        // THIS walk — the same site vocabulary a definition goes
        // through, offending field and all.
        DocEdit::SetVarDistribution {
            var,
            distribution: Some(d),
        } => Some(NonFiniteSite::DocParam {
            var: var.clone(),
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
        DocEdit::SetVarValue { .. }
        // A definition's floats are its expression's literals, finite
        // by the construction door.
        | DocEdit::DeclareVar {
            def: crate::var::VarDecl::Defined(_),
            ..
        }
        | DocEdit::DefineVar {
            def: crate::var::VarDecl::Defined(_),
            ..
        }
        // A notation is a table code, not a float.
        | DocEdit::SetVarUnit { .. }
        // The partial arm above, completed: a CLEARED annotation
        // carries no float at all.
        | DocEdit::SetVarDistribution { .. }
        | DocEdit::RenameVar { .. }
        | DocEdit::DeleteVar { .. }
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
        | DocEdit::SetOffset { .. }
        // A gauge reference is a node id, and so is what a promote or a
        // fold names.
        | DocEdit::SetGauge { .. }
        | DocEdit::Promote { .. }
        | DocEdit::Fold { .. }
        // A label is text.
        | DocEdit::SetLabel { .. }
        | DocEdit::UpdateReference { .. } => None,
    }
}

/// **How a file's variable table disagrees with an operation's output
/// signature** ([`SnapshotError::OutputSignature`]).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OutputFault {
    /// An output row names a node that is not live.
    NodeAbsent {
        /// The row.
        var: SpokenVar,
    },
    /// An output row names a port past the end of the signature.
    PortOutside {
        /// The row.
        var: SpokenVar,
        /// The port it names.
        port: u8,
        /// How many ports the signature has.
        ports: usize,
    },
    /// An output row's kind is not its port's.
    Kind {
        /// The row.
        var: SpokenVar,
        /// The port's name.
        port: &'static str,
        /// The kind stored.
        stored: VarKind,
        /// The kind the signature gives the port.
        signature: VarKind,
    },
    /// Two rows name one port.
    Twice {
        /// The port's name.
        port: &'static str,
        /// The lower id.
        first: SpokenVar,
        /// The higher id.
        second: SpokenVar,
    },
    /// A live node's port has no row.
    Missing {
        /// The port's name.
        port: &'static str,
    },
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
    /// The mint's log does not count up from one: an entry's ordinal
    /// is not its place in the log (a repeat, a step down or a gap) —
    /// a log no mint wrote.
    MintLogOrder {
        /// The first entry whose ordinal is not its place in the log.
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
    /// A Boolean's or a Union's declared name is minted by the node
    /// itself — the edit doors' [`crate::EditError::DeclaredNameNotUpstream`],
    /// by the same rule, as far as a stored document is held to it: a
    /// re-point may move a minter out of reach, which strands the name
    /// and loads, and no edit makes a node declare its own name.
    DeclaredNameNotUpstream {
        /// The declaring node.
        node: SpokenNode,
        /// The name.
        name: SpokenName,
    },
    /// An operand reads a variable the mint log does not hold — one
    /// the document never minted. A read of a variable minted and since
    /// deleted is legal: it is that output's stranded reader (D10),
    /// refused at evaluation.
    OperandUnminted {
        /// The reading node.
        node: SpokenNode,
        /// The operand.
        slot: crate::OperandSlot,
        /// The read.
        var: SpokenVar,
    },
    /// An operand reads a world placement's copy — the edit doors'
    /// [`crate::EditError::ReadsWorldCopy`].
    ReadsWorldCopy {
        /// The reading node.
        node: SpokenNode,
        /// The slot.
        slot: SlotId,
        /// The placement whose copy it reads.
        placement: SpokenNode,
    },
    /// A measure is sited at a world placement — the edit doors'
    /// [`crate::EditError::MeasuresWorldCopy`].
    MeasuresWorldCopy {
        /// The measure.
        node: SpokenNode,
        /// The placement it is sited at.
        placement: SpokenNode,
    },
    /// The nodes' reads close a loop ([`crate::Doc::upstream`]): no edit
    /// leaves a document so, since every door that writes a read asks
    /// acyclicity ([`crate::EditError::WouldCycle`]).
    ReadCycle {
        /// A node on the loop.
        at: SpokenNode,
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
    /// A variable whose stored kind is not its definition's — the
    /// pairing `Var::new` makes and no door can break.
    VarKind {
        /// The variable.
        var: SpokenVar,
        /// The kind stored beside the definition.
        kind: VarKind,
        /// The kind the definition holds.
        def: VarKind,
    },
    /// The variable table disagrees with an operation's output
    /// signature (D10, [`crate::Node::outputs`]): what the insert door
    /// mints and the delete door removes, so only a file can be wrong
    /// about it.
    OutputSignature {
        /// The operation, absent where the row names no live node.
        node: SpokenNode,
        /// How the table disagrees.
        fault: Box<OutputFault>,
    },
    /// A variable id the mint log does not hold as a variable's — one
    /// the document never minted (VR1).
    VarNotMinted {
        /// The variable.
        var: SpokenVar,
    },
    /// A name attached to a variable id that names nothing live.
    NameOnMissingVar {
        /// The id the name is attached to.
        var: VarId,
        /// The name.
        name: VarName,
    },
    /// One name held by two variables (VR2: a name is unique within a
    /// document).
    VarNameTwice {
        /// The name.
        name: VarName,
        /// The lower id holding it.
        a: VarId,
        /// The higher id holding it.
        b: VarId,
    },
    /// The recorded ε is not finite and strictly positive.
    EpsilonInvalid {
        /// The recorded value.
        value: f64,
    },
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
    /// A node reading a variable id this document never minted. A
    /// deleted variable's reader is legal (its id is in the mint log);
    /// an id the log never held names nothing that ever existed.
    ReaderOfUnmintedVar {
        /// The offending node.
        node: SpokenNode,
        /// The id it reads.
        var: VarId,
    },
    /// A node's slot reads a live variable of a kind it does not
    /// admit: a SLOT expression reads one at another dimension than its
    /// kind, or an operand reads one its field does not take. The edit
    /// doors refuse both through the same predicates
    /// (`Doc::var_read_faults`, [`crate::SlotKind::admits`]), so a file
    /// carrying one is data the edit doors could not have produced.
    SlotVarKind {
        /// The offending node.
        node: SpokenNode,
        /// The slot that reads it.
        slot: SlotId,
        /// The variable it reads, boxed so the refusal stays a small
        /// `Err`.
        var: Box<SpokenVar>,
        /// The kind of the variable read.
        found: VarKind,
        /// What the read there takes: at an operand, the slot's kind;
        /// at an expression, the kind of the dimension its leaf reads
        /// the variable at — the slot's own at the formula's root, and
        /// the leaf's inside a function (`sin(w)` at a length slot
        /// reads `w` as an angle).
        expected: crate::SlotKind,
    },
    /// A node whose PAYLOAD expression — a measured expression's value
    /// leaf or an assertion's bound, the expressions no slot addresses
    /// ([`crate::node::payload_exprs`]) — reads a live variable at
    /// another dimension than its kind. The address is the NODE: there
    /// is no slot to name.
    PayloadVarKind {
        /// The offending node.
        node: SpokenNode,
        /// The variable its payload reads.
        var: SpokenVar,
        /// The dimension the variable's kind reads at.
        declared: VarKind,
        /// The dimension the expression reads it at.
        referenced: crate::expr::Dimension,
    },
    /// A variable with no name that nothing reads (VR7: an anonymous
    /// variable goes with its reader).
    AnonymousVarUnread {
        /// The variable.
        var: SpokenVar,
    },
    /// A variable with no name that more than one reader reads (VR2:
    /// an unnamed variable has exactly one reader). A file written
    /// before the doors refused a share holds one.
    SharedVarNeedsName {
        /// The variable.
        var: SpokenVar,
    },
    /// A definition reading a variable id this document never minted.
    DefinitionReadsUnmintedVar {
        /// The defined variable.
        var: SpokenVar,
        /// The id it reads.
        read: VarId,
    },
    /// A definition reading a live variable at another dimension than
    /// its kind, which the edit doors refuse through the same
    /// predicate.
    DefinitionVarKind {
        /// The defined variable.
        var: SpokenVar,
        /// The variable it reads.
        read: SpokenVar,
        /// The dimension the read variable's kind reads at.
        declared: VarKind,
        /// The dimension the definition reads it at.
        referenced: crate::expr::Dimension,
    },
    /// A definition reading its own variable back (VR3), directly or
    /// through other definitions.
    DefinitionCycle {
        /// The variable defined.
        var: SpokenVar,
        /// The cycle, from `var` on: each variable's definition reads
        /// the next, and the last one's reads `var`.
        through: Vec<SpokenVar>,
    },
    /// A variable whose expansion through the definitions it reads
    /// outgrows [`crate::edit::DEFINITION_NODE_BOUND`].
    DefinitionTooLarge {
        /// The variable.
        var: SpokenVar,
        /// Its expansion's node count, saturating at one past the
        /// bound.
        nodes: usize,
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
            Self::NodeNotMinted { id } => write!(
                f,
                "{id} is not in the document's mint log — the document never minted it",
            ),
            Self::StepIds { node, fault } => write!(f, "{node}'s step ids: {fault}"),
            Self::MintLogOrder { entry } => write!(
                f,
                "the mint's log does not count up from one at {entry} — its ordinal is not its \
                 place in the log (a repeat, a step down or a gap), which no mint writes. {}",
                geom_core::KERNEL_OR_FILE_DEFECT_ENDING
            ),
            Self::NameStepNotMinted { name, step } => write!(
                f,
                "{name} spells the profile step id {step}, which the document's mint log \
                 does not hold — the document never minted it",
            ),
            Self::DeclaredNameNotUpstream { node, name } => write!(
                f,
                "the declaration names {name}, which {node} mints itself — no edit writes such a \
                 pair. {}",
                geom_core::KERNEL_OR_FILE_DEFECT_ENDING
            ),
            // A document written before operands were reads holds a
            // node id in every operand, which the log holds as a node.
            Self::OperandUnminted { node, slot, var } => write!(
                f,
                "{node}'s {slot} reads {var}, which the document's mint log does not hold as a \
                 variable — the document never minted it, or was written before an operand \
                 was a read. {}",
                crate::sentence::Recourse(super::REGENERATE_RECOURSE)
            ),
            Self::MeasuresWorldCopy { node, placement } => write!(
                f,
                "{node} is sited at {placement}, whose copy only the product and export read"
            ),
            Self::ReadsWorldCopy {
                node,
                slot,
                placement,
            } => write!(
                f,
                "{node}'s {slot} reads the world copy {placement} makes, and construction never \
                 reads the world"
            ),
            Self::ReadCycle { at } => write!(
                f,
                "the nodes' reads close a loop through {at} — no edit writes one. {}",
                geom_core::KERNEL_OR_FILE_DEFECT_ENDING
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
            Self::VarKind { var, kind, def } => write!(
                f,
                "{var} is stored as kind {kind} and its definition is of kind {def}"
            ),
            Self::VarNotMinted { var } => write!(
                f,
                "{var} is not in the document's mint log — the document never \
                 minted it"
            ),
            Self::OutputSignature { node, fault } => match &**fault {
                OutputFault::NodeAbsent { var } => {
                    write!(
                        f,
                        "{var} is stored as an output of {node}, which is not live"
                    )
                }
                OutputFault::PortOutside { var, port, ports } => write!(
                    f,
                    "{var} is stored as port {port} of {node}, whose signature has {ports} \
                     port(s)"
                ),
                OutputFault::Kind {
                    var,
                    port,
                    stored,
                    signature,
                } => write!(
                    f,
                    "{var} is stored as {} {stored}, and the {port} port of {node} defines \
                     {} {signature}",
                    crate::sentence::article(&stored.to_string()),
                    crate::sentence::article(&signature.to_string())
                ),
                OutputFault::Twice {
                    port,
                    first,
                    second,
                } => write!(
                    f,
                    "{first} and {second} are both stored as the {port} port of {node}"
                ),
                // A file written before operations defined variables lacks
                // every output, and only a current build can mint them.
                OutputFault::Missing { port } => write!(
                    f,
                    "{node} defines a {port} port and no variable is stored for it. {}",
                    crate::sentence::Recourse(super::REGENERATE_RECOURSE)
                ),
            },
            Self::AnonymousVarUnread { var } => write!(
                f,
                "{var} has no name and nothing reads it, and a variable with no name is one \
                 something reads"
            ),
            Self::SharedVarNeedsName { var } => write!(
                f,
                "{var} has no name and more than one reader, and a variable with no name has \
                 exactly one. {}",
                crate::sentence::Recourse(super::REGENERATE_RECOURSE)
            ),
            Self::DefinitionReadsUnmintedVar { var, read } => write!(
                f,
                "the definition of {var} reads variable {read}, which the document never \
                 minted"
            ),
            Self::DefinitionVarKind {
                var,
                read,
                declared,
                referenced,
            } => write!(
                f,
                "{}",
                crate::edit::DefinitionVarKindSentence {
                    var,
                    read,
                    declared: *declared,
                    referenced: *referenced,
                }
            ),
            Self::DefinitionCycle { var, through } => write!(
                f,
                "{}",
                crate::edit::DefinitionCycleSentence { var, through }
            ),
            Self::DefinitionTooLarge { var, nodes } => write!(
                f,
                "{}",
                crate::edit::DefinitionTooLargeSentence { var, nodes: *nodes }
            ),
            Self::NameOnMissingVar { var, name } => write!(
                f,
                "the name {name} is attached to variable {var}, which is not live"
            ),
            Self::VarNameTwice { name, a, b } => write!(
                f,
                "the name {name} is held by two variables, {a} and {b}, and a name is unique \
                 within a document"
            ),
            Self::EpsilonInvalid { value } => write!(
                f,
                "the recorded ε {value:e} is not finite and strictly positive"
            ),
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
            Self::ReaderOfUnmintedVar { node, var } => write!(
                f,
                "{node} reads variable {var}, which this document never minted"
            ),
            Self::SlotVarKind {
                node,
                slot,
                var,
                found,
                expected,
            } => match expected {
                crate::SlotKind::Is(kind) if let Some(referenced) = kind.dimension() => write!(
                    f,
                    "{node}: slot {} reads {var} as {} {referenced}, and it is declared {found}",
                    slot.label(),
                    referenced.article()
                ),
                _ => write!(
                    f,
                    "{node}'s {slot} reads {var}, which is {} {found}, where it takes {expected}",
                    crate::sentence::article(&found.to_string()),
                ),
            },
            Self::PayloadVarKind {
                node,
                var,
                declared,
                referenced,
            } => write!(
                f,
                "{node}: its payload expression reads {var} as {} {referenced}, and it is \
                 declared {declared}",
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
                "metadata {key:?} on {name} does not carry an integer \
                 \"v\" version field: {error}"
            ),
        }
    }
}

/// Re-checks the document invariants `apply` maintains — on a parsed
/// snapshot (load) and on the in-memory snapshot (save) alike.
fn validate_snapshot(doc: &ProfileDoc, tol: Tol) -> Result<(), SnapshotError> {
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
    // The mint log first, since every check below asks it: its ordinals
    // count up from one, the only log a mint writes.
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
        // A measure's sites are names read at a node, which a delete
        // strands; what only a file can be wrong about is an id the
        // document never minted.
        for at in node.measure_sites() {
            check_id(at)?;
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
        if let Some((at, fault)) = node.placement_frame_fault(tol) {
            return Err(SnapshotError::placement_frame(doc.spoken(id), at, fault));
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
                    input: doc
                        .defined_by(input)
                        .map_or_else(|| SpokenNode::absent(id), |(at, _)| doc.spoken(at)),
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
        // forget the question. A face-based side authors nothing: its
        // numbers are the face its head names in the part, and whether
        // the part has that face is the solve's at evaluation
        // (`MateFault::FaceUnresolved`), never this door's.
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
    // Acyclicity over the reads (`Doc::upstream`), which every door
    // that writes a read asks: `order` is presentation order, since a
    // re-point may read a node inserted after its reader, so a loop is
    // the one thing a file can be wrong about here.
    if let Some(&at) = crate::eval::schedule::schedule(doc).unschedulable.first() {
        return Err(SnapshotError::ReadCycle { at: doc.spoken(at) });
    }
    // Every declared pair, by the rule its edit doors ask
    // (`node::declared_side_fault`), as far as a stored document can
    // still be held to it: a re-point (`SetParam` at an operand, `SetMembers`)
    // strands a site or a name it moved out of reach, and that loads.
    // What no edit leaves is a node declaring a name it mints itself.
    for (&id, node) in &doc.nodes {
        match crate::node::declared_side_fault(
            crate::node::declared_sides(node.declared_pairs()),
            None,
            |n| n != id,
        ) {
            None => {}
            Some((_, crate::node::DeclaredSideFault::SiteNotAnOperand)) => {
                unreachable!("the load door judges no declared site")
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
/// A wrong-kind ARGUMENT is not here. A program slot is a slot like
/// any other, so it is decided by the document-wide slot read walk
/// ([`SnapshotError::SlotVarKind`]) that the edit doors ask too, rather
/// than by a second spelling that reached profile nodes alone.
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
    let env = snapshot.var_env::<f64>();
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
    use crate::doc::VarName;
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
            Vars,
            OutputSignature,
            DefinitionRead,
            DefinitionCycle,
            SlotRead,
            PayloadRead,
            OperandRead,
            UnnamedReader,
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
            Walk::Vars
            | Walk::OutputSignature
            | Walk::DefinitionRead
            | Walk::DefinitionCycle
            | Walk::SlotRead
            | Walk::PayloadRead
            | Walk::OperandRead
            | Walk::UnnamedReader
            | Walk::Snapshot => true,
        }
    }

    test_utils::f6_variants! {
        /// **The load door's snapshot vocabulary**, welded to the enum
        /// by the match the macro writes — the roster half of the
        /// census below. See [`test_utils::f6::VariantCensus`] for what
        /// that weld does and does not buy.
        const SNAPSHOT_ERROR: SnapshotError = [
            NodeNotMinted,
            StepIds,
            MintLogOrder,
            NameStepNotMinted,
            DeclaredNameNotUpstream,
            OperandUnminted,
            ReadsWorldCopy,
            MeasuresWorldCopy,
            ReadCycle,
            WitnessSite,
            WitnessOnMissingNode,
            LabelOnMissingNode,
            VarKind,
            OutputSignature,
            VarNotMinted,
            NameOnMissingVar,
            VarNameTwice,
            ReaderOfUnmintedVar,
            SlotVarKind,
            PayloadVarKind,
            AnonymousVarUnread,
            SharedVarNeedsName,
            DefinitionReadsUnmintedVar,
            DefinitionVarKind,
            DefinitionCycle,
            DefinitionTooLarge,
            EpsilonInvalid,
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
            // `MintLogOrder`: both this walk and the structural one
            // raise it; this one runs first.
            SnapshotError::VarKind { .. }
            | SnapshotError::MintLogOrder { .. }
            | SnapshotError::VarNotMinted { .. }
            | SnapshotError::NameOnMissingVar { .. }
            | SnapshotError::VarNameTwice { .. } => Walk::Vars,
            SnapshotError::OutputSignature { .. } => Walk::OutputSignature,
            // An operand's kind is the operand walk's; the expression
            // walks raise the rest, and the slot walk runs first.
            SnapshotError::SlotVarKind {
                slot: SlotId::Operand(_),
                ..
            } => Walk::OperandRead,
            SnapshotError::ReaderOfUnmintedVar { .. } | SnapshotError::SlotVarKind { .. } => {
                Walk::SlotRead
            }
            SnapshotError::PayloadVarKind { .. } => Walk::PayloadRead,
            SnapshotError::OperandUnminted { .. }
            | SnapshotError::ReadsWorldCopy { .. }
            | SnapshotError::MeasuresWorldCopy { .. } => Walk::OperandRead,
            SnapshotError::AnonymousVarUnread { .. } | SnapshotError::SharedVarNeedsName { .. } => {
                Walk::UnnamedReader
            }
            SnapshotError::DefinitionReadsUnmintedVar { .. }
            | SnapshotError::DefinitionVarKind { .. } => Walk::DefinitionRead,
            SnapshotError::DefinitionCycle { .. } | SnapshotError::DefinitionTooLarge { .. } => {
                Walk::DefinitionCycle
            }
            // `validate_snapshot`, which is where the rest live.
            SnapshotError::NodeNotMinted { .. }
            | SnapshotError::StepIds { .. }
            | SnapshotError::NameStepNotMinted { .. }
            | SnapshotError::DeclaredNameNotUpstream { .. }
            | SnapshotError::ReadCycle { .. }
            | SnapshotError::WitnessSite { .. }
            | SnapshotError::WitnessOnMissingNode { .. }
            | SnapshotError::LabelOnMissingNode { .. }
            | SnapshotError::EpsilonInvalid { .. }
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
        let at = |bits| crate::SpokenNode::absent(RecipeNodeId::new(0, bits));
        let node = || at(5);
        let face = || crate::names::StableName {
            kind: crate::names::EntityKind::Face,
            node: RecipeNodeId::new(0, 5),
            path: Vec::new(),
        };
        // One value per arm, welded to the roster below: a case list
        // that fell behind the enum would name fewer identifiers than
        // the roster holds and red in the comparison.
        let cases = [
            SnapshotError::NodeNotMinted { id: node() },
            SnapshotError::StepIds {
                node: node(),
                fault: crate::program::StepIdFault::Repeated {
                    step: crate::node::StepId::new(0, 2),
                },
            },
            SnapshotError::MintLogOrder {
                entry: crate::Minted::Step(crate::node::StepId::new(0, 3)),
            },
            SnapshotError::NameStepNotMinted {
                name: crate::SpokenName::absent(face()),
                step: crate::node::StepId::new(0, 9),
            },
            SnapshotError::DeclaredNameNotUpstream {
                node: node(),
                name: crate::SpokenName::absent(face()),
            },
            SnapshotError::OperandUnminted {
                node: node(),
                slot: crate::OperandSlot::Target,
                var: crate::SpokenVar::new(crate::VarId::new(0, 7), None),
            },
            SnapshotError::SlotVarKind {
                node: node(),
                slot: SlotId::Operand(crate::OperandSlot::Target),
                var: Box::new(crate::SpokenVar::new(crate::VarId::new(0, 7), None)),
                found: crate::VarKind::Profile,
                expected: crate::SlotKind::Is(crate::VarKind::Body),
            },
            SnapshotError::ReadsWorldCopy {
                node: node(),
                slot: SlotId::Operand(crate::OperandSlot::A),
                placement: node(),
            },
            SnapshotError::MeasuresWorldCopy {
                node: node(),
                placement: node(),
            },
            SnapshotError::ReadCycle { at: at(9) },
            SnapshotError::WitnessSite { node: node() },
            SnapshotError::WitnessOnMissingNode { node: node() },
            SnapshotError::LabelOnMissingNode { node: node() },
            SnapshotError::VarKind {
                var: crate::SpokenVar::new(crate::VarId::new(0, 7), None),
                kind: crate::VarKind::Length,
                def: crate::VarKind::Angle,
            },
            SnapshotError::OutputSignature {
                node: node(),
                fault: Box::new(super::OutputFault::Missing { port: "body" }),
            },
            SnapshotError::VarNotMinted {
                var: crate::SpokenVar::new(crate::VarId::new(0, 7), None),
            },
            SnapshotError::NameOnMissingVar {
                var: crate::VarId::new(0, 7),
                name: VarName::from_static("w"),
            },
            SnapshotError::VarNameTwice {
                name: VarName::from_static("w"),
                a: crate::VarId::new(0, 7),
                b: crate::VarId::new(0, 8),
            },
            SnapshotError::ReaderOfUnmintedVar {
                node: node(),
                var: crate::VarId::new(0, 7),
            },
            SnapshotError::SlotVarKind {
                node: node(),
                slot: SlotId::Distance,
                var: Box::new(crate::SpokenVar::new(
                    crate::VarId::new(0, 7),
                    Some(VarName::from_static("depth")),
                )),
                found: crate::VarKind::Angle,
                expected: crate::SlotKind::Is(crate::VarKind::Length),
            },
            SnapshotError::PayloadVarKind {
                node: node(),
                var: crate::SpokenVar::new(
                    crate::VarId::new(0, 7),
                    Some(VarName::from_static("depth")),
                ),
                declared: crate::VarKind::Angle,
                referenced: Dimension::Length,
            },
            SnapshotError::AnonymousVarUnread {
                var: crate::SpokenVar::new(crate::VarId::new(0, 7), None),
            },
            SnapshotError::SharedVarNeedsName {
                var: crate::SpokenVar::new(crate::VarId::new(0, 7), None),
            },
            SnapshotError::DefinitionReadsUnmintedVar {
                var: crate::SpokenVar::new(crate::VarId::new(0, 7), None),
                read: crate::VarId::new(0, 8),
            },
            SnapshotError::DefinitionVarKind {
                var: crate::SpokenVar::new(crate::VarId::new(0, 7), None),
                read: crate::SpokenVar::new(crate::VarId::new(0, 8), None),
                declared: crate::VarKind::Angle,
                referenced: Dimension::Length,
            },
            SnapshotError::DefinitionCycle {
                var: crate::SpokenVar::new(crate::VarId::new(0, 7), None),
                through: vec![crate::SpokenVar::new(crate::VarId::new(0, 7), None)],
            },
            SnapshotError::DefinitionTooLarge {
                var: crate::SpokenVar::new(crate::VarId::new(0, 7), None),
                nodes: 4097,
            },
            SnapshotError::EpsilonInvalid { value: 0.0 },
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
                    fresh: Vec::new(),
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
                a: crate::mate::MateFrame::on_part(crate::placement::Placement::IDENTITY),
                b: crate::mate::MateFrame::on_part(crate::placement::Placement::IDENTITY),
                primitive: crate::mate::MatePrimitive::FrameCoincidence,
                sense: crate::mate::AxisSense::Aligned,
                clocking: None,
            },
        };
        let applied = crate::edit::apply(
            &doc,
            &crate::edit::DocEdit::InsertNode {
                node: Box::new(mate),
                fresh: Vec::new(),
            },
            Tol::witness(),
            &crate::mate::RefusingReach,
        )
        .expect("a finite alignment inserts");
        let mate_id = applied.record.minted.expect("the insert minted an id");
        doc = applied.doc;
        // Finite, the document saves — so the refusals below are the
        // poked coordinates'.
        save(&doc, &[], Tol::witness()).expect("the mated assembly saves");
        let poked = |alignment: fn(&mut crate::mate::Alignment)| {
            let mut doc = doc.clone();
            match doc.nodes.get_mut(&mate_id) {
                Some(Node::Mate { alignment: a, .. }) => alignment(a),
                other => panic!("the fixture's mate is a mate, got {other:?}"),
            }
            let refused = save(&doc, &[], Tol::witness());
            (doc, refused)
        };
        let (doc_rider, rider) = poked(|a| a.clocking = Some(f64::NAN));
        match rider {
            Err(PersistError::Snapshot(SnapshotError::MateAlignment { node })) => {
                assert_eq!(node, doc_rider.spoken(mate_id));
            }
            other => panic!("a non-finite rider must refuse at save, got {other:?}"),
        }
        // A frame offset's literal step is a placement's, held by the
        // frame rule as a gauge's is, and named by its side.
        let (doc_frame, frame) = poked(|a| {
            a.b = crate::mate::MateFrame::on_part(crate::placement::Placement::literal(
                &crate::placement::Frame::translation([f64::NAN, 0.0, 0.0]),
            ));
        });
        match frame {
            Err(PersistError::Snapshot(SnapshotError::PlacementNonFinite { node, at })) => {
                assert_eq!(node, doc_frame.spoken(mate_id));
                assert_eq!(
                    at,
                    FrameSite::MateStep {
                        side: crate::mate::MateSide::B,
                        index: 0
                    }
                );
            }
            other => panic!("a non-finite frame offset must refuse at save, got {other:?}"),
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
                    &crate::DocEdit::InsertNode {
                        node,
                        fresh: Vec::new(),
                    },
                    tol,
                    &crate::RefusingReach,
                )
                .expect("the node inserts")
                .doc;
            let id = *doc.ids().last().expect("the inserted node");
            (doc, id)
        };
        let doc = ProfileDoc::empty_derived("check-speak", tol);
        let (doc, plane) = insert(doc, Box::new(crate::test_support::xy_frame()));
        let triangle = crate::program::LoopProgram::polygon([(0.0, 0.0), (1.0, 0.0), (0.5, 1.0)])
            .expect("finite corners");
        let (doc, profile) = insert(
            doc,
            Box::new(Node::Profile(crate::program::ProfileProgram {
                frame: plane.into(),
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
                        "metadata \"swatch\" on the face of Profile \"outline\" \
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
            node: RecipeNodeId::new(0, node),
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
                assert_eq!(id, crate::SpokenNode::absent(RecipeNodeId::new(0, 7)));
            }
            other => panic!("an appearance key the mint never minted must refuse, got {other:?}"),
        }
    }
}
