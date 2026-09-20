//! Name resolution and diagnosis (M4 PR 4; NAMING-DESIGN N5 verbatim,
//! ratified #74).
//!
//! Resolution is a TABLE LOOKUP (N4), never a search: a name resolves
//! by reading the evaluation the replay just built. What this module
//! adds is the typed failure ladder around that lookup —
//! [`ResolveError`] exactly as N5 wrote it, with the [`Diagnosis`]
//! computed by the verdict-vector diff engine ([`vdiff`]), tombstones
//! for ghost rendering, and the hit-testing inversion ([`hit`]).
//!
//! # The candidate story (spec D1, reported choices)
//!
//! - A [`crate::names::Entry::Tied`] row IS the ambiguity: referencing
//!   it yields [`ResolveError::Ambiguous`] whose `candidates` are the
//!   distinct NAMES the tied set answers to (the tied entities are
//!   combinatorially indistinguishable — that is what a tie means — so
//!   the tied set expressed in names is the tie row itself, and the
//!   [`TieWitness`] carries the multiplicity and site).
//! - The documented `order_along` over-tie widens: a reference to a
//!   RANKED fragment name whose group over-tied resolves `Ambiguous`
//!   with the WIDENED base name as the candidate — never a mis-bind.
//! - N3's offered candidates (a retired constituent's merged name; a
//!   vanished merged name's constituents) ride NEXT TO the N5 error,
//!   not inside it: [`ResolutionFailure::offers`] wraps the verbatim
//!   [`ResolveError`] (spec D9's sanctioned "wrapping" choice — N5's
//!   `Vanished` struct stays byte-verbatim).
//!
//! # The appearance hook (spec D9)
//!
//! PR 7's [`crate::appearance::AppearanceLoss`] rows carry the
//! coarse, evaluation-visible causes; [`enrich_appearance_loss`] (and
//! its with-prior form) maps each row onto this module's full ladder
//! — the per-arm table lives on that function. The banked
//! operand→final repair ergonomics (PR 7 review A1) are served by
//! [`appearance_rebind_suggestions`]: every appearance-carrying name
//! mapped to the derivations wrapping it, loss or no loss.
//!
//! # Low-evidence diagnosis (reported)
//!
//! `Vanished`'s diagnosis diffs the last-good run against the current
//! one. Two things can make that diff silent about the name, and they
//! are answered at different points of the ladder.
//!
//! **Inside the flip stage**, before any fallback: the diff engine's
//! population-cancel blind spot (`vdiff` module docs), and **sweep
//! pruning** (ratified 2026-07-29, N5 as amended) — the realized
//! boolean sweep records no verdicts for pairs its candidate
//! generation pruned, so a vanish whose flip evidence lived on a
//! now-pruned pair has no recorded flip to cite. The SHADOW-EXECUTION
//! rung ([`shadow_exec_flip`], issue 134) sits there: it fires on the
//! empty pair population, re-runs the vanished name's own
//! discriminator pairs against both contexts, and outranks the
//! incidental flips a disjointing edit leaves at the same node. Its
//! answer is marked [`FlipSource::ShadowExec`], so no reader mistakes
//! it for a line of a log. The front door N5's amended text pointed
//! at — the recovery rung that did not exist yet — is that rung.
//!
//! **After the flip and doc-diff lanes come up empty**, two honest
//! rungs remain, in order: `Cascade` when an embedded operand name
//! itself fails to resolve, and the QUALIFIER-DELTA rung
//! ([`qualifier_delta`]): the N2 discriminator verdicts recorded in
//! the names themselves yield a `PredicateFlip` derived from recorded
//! data when a same-shape sibling differs by exactly one pure-sign
//! `SideOf` entry. If that too finds nothing, the total fallback is
//! [`Diagnosis::cause_not_in_evidence`], which carries that reading at
//! the value rather than in prose here.
//!
//! The shadow rung's limits — the cancelling exchange it does not
//! address, the collapse half it cannot, the `OrderAlong` half it has
//! no pair for — are stated once, at [`shadow_exec_flip`].

mod hit;
mod pick;
mod vdiff;

pub use hit::{HitTestError, body_name, edge_name, entity_name, face_name, vertex_name};
pub use pick::{
    Answer, Crossing, FaceAnswer, MeshPick, MeshPickError, NodePick, NodePickError, PickHit,
    PickMemo, PickTarget, TSpan, answer_of, crossing, pick_face, ray_triangle,
};
pub use vdiff::{
    FlipSet, NodeVerdictDelta, NodeVerdicts, PredicateDivergence, RunStatus, SummaryDelta,
    SummaryDivergence, SummaryFlip, SummaryFlipSet, VerdictFlip, VerdictRow, VerdictSummary,
    VerdictVector, VerdictVectorKey, diff_summaries, diff_verdicts, verdict_summary,
};

use std::collections::{BTreeMap, BTreeSet};

use geom_core::{Decide, Sign};

use crate::appearance::{AppearanceLoss, AppearanceLossCause, AppearanceMap};
use crate::diff::NodeChange;
use crate::doc::Doc;
use crate::eval::{Evaluation, NodeResult};
use crate::names::{
    EntityKey, EntityKind, EntityRef, Entry, Qualifier, RoleSeg, StableName, name_free_seg,
};
use crate::node::{RecipeNodeId, SlotId};
use crate::program::ProfileProgram;
use crate::witness::WitnessBifurcation;
use geom_core::Tol;

/// Typed resolution failure — N5 VERBATIM (spec D1: the block is
/// normative, not indicative). Data in results, never a panic and
/// never a string.
#[derive(Debug, Clone, PartialEq)]
pub enum ResolveError {
    /// The name's minting node is live but no table in the evaluation
    /// derives the name any more.
    Vanished {
        /// The unresolvable name.
        name: StableName,
        /// Why — computed from both evaluations' verdict logs, the
        /// structural doc diff, or upstream cascade (N5: "the
        /// diagnosis is computable").
        diagnosis: Diagnosis,
        /// The last-good table entry, when a prior evaluation still
        /// resolved the name (GQ7 ghost-rendering payload).
        last_good: Option<Tombstone>,
    },
    /// The name is tie-marked (N2): the reference cannot pick among
    /// equally-admissible candidates — never auto-picked.
    Ambiguous {
        /// The referenced name.
        name: StableName,
        /// The distinct names the tied set answers to (module docs:
        /// the tie row itself, or the widened base on an
        /// `order_along` over-tie).
        candidates: Vec<StableName>,
        /// The recorded tie's site and width.
        tie: TieWitness,
    },
    /// The name's minting node is no longer in the document (N5
    /// dangling semantics: a later `DeleteNode` may strand a name;
    /// the repair is the explicit `Rebind`).
    NodeGone {
        /// The stranded name.
        name: StableName,
        /// The recipe edit that removed the node (derivable: ids are
        /// never reused, so a live-then-gone node was deleted).
        edit: RecipeEditRef,
    },
}

// The human-readable rendering (LIB-DOORS F6 shape): each arm states
// the PROBLEM in prose — the name through `StableName`'s own
// `Display` (its kind and minting node — the half a user can act on),
// the WHY forwarded from the payload's own rendering. `NodeGone`
// alone re-spells the name, because its sentence interleaves the
// fields ("the name's minting node …"). Composing layers
// (`NodeErrorKind`'s two resolve arms) FORWARD this rather than
// re-stating it.
impl core::fmt::Display for ResolveError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Vanished {
                name, diagnosis, ..
            } => write!(
                f,
                "the {name} no longer resolves in this evaluation: {diagnosis}"
            ),
            Self::Ambiguous { name, tie, .. } => write!(
                f,
                "the {name} is tie-marked: {} equally-admissible \
                 candidates at its recorded site — a tie is never broken by picking; \
                 refine the reference until one candidate remains",
                tie.width
            ),
            Self::NodeGone { name, edit } => write!(
                f,
                "the {} name's minting node {} is no longer in the document ({edit}) — \
                 the repair is an explicit rebind",
                name.kind.noun(),
                name.node.0
            ),
        }
    }
}

impl core::error::Error for ResolveError {}

/// The two refusals every N5 ladder in this crate mints, minted in one
/// place.
///
/// Two ladders answer the same question at different scopes — this
/// module's whole-evaluation one and the mid-evaluation one the
/// evaluation doors resolve through (`eval::wire`'s `ladder`) — and
/// their rungs 1 and 2
/// are the SAME refusal, derived from the same facts. What differs
/// between them is rung order enforcement and the `Vanished` rung, so
/// those stay where they are and these do not: a payload shape is
/// held by one constructor rather than by two sites agreeing.
impl ResolveError {
    /// Rung 1: the minting node must still be in the document, or the
    /// name is stranded. `None` when it is live.
    ///
    /// Ids are never reused, so the two cases are derivable rather
    /// than recorded: an id below the mint counter named a node this
    /// document DELETED, and one at or above it was never this
    /// document's at all.
    ///
    /// Neither case takes a refinement, and neither can: both are
    /// decided by the document in hand, and a prior run of the SAME
    /// document has nothing to add to either — ids are not reused, so
    /// a node the prior run held and this one does not is deleted,
    /// which is what the counter already says.
    pub(crate) fn node_gone(name: &StableName, doc: &Doc<ProfileProgram>) -> Option<Self> {
        if doc.node(name.node).is_some() {
            return None;
        }
        Some(Self::NodeGone {
            name: name.clone(),
            edit: if name.node.0 < doc.next_id {
                RecipeEditRef::NodeDeleted { node: name.node }
            } else {
                RecipeEditRef::ForeignNode { node: name.node }
            },
        })
    }

    /// Rung 2: `name` landed on a tie row — `at`, recorded at `node`,
    /// `width` candidates wide.
    ///
    /// The tie row IS the ambiguity (N5), so the candidates are that
    /// row expressed in names and are derived from the witness here
    /// rather than restated per door. `at` is the referenced name
    /// itself, except on an `order_along` over-tie, where it is the
    /// widened base row the reference actually tied against.
    pub(crate) fn ambiguous(
        name: &StableName,
        at: StableName,
        node: RecipeNodeId,
        width: u32,
    ) -> Self {
        Self::Ambiguous {
            name: name.clone(),
            candidates: vec![at.clone()],
            tie: TieWitness { node, at, width },
        }
    }

    /// Rung 3 for a door with no history: `name` is gone and nothing
    /// can be said about why.
    ///
    /// `last_good` is `None` because there is nothing to look it up
    /// in — a mid-evaluation door has no prior run — which is the same
    /// absence that makes the diagnosis
    /// [`Diagnosis::cause_not_in_evidence`]. The whole-evaluation
    /// ladder does not use this: it reaches the same diagnosis only
    /// after its own rungs come up empty, and it can still bank a
    /// tombstone.
    pub(crate) fn vanished_fallback(name: &StableName) -> Self {
        Self::Vanished {
            name: name.clone(),
            diagnosis: Diagnosis::cause_not_in_evidence(name.node),
            last_good: None,
        }
    }
}

/// Where a [`Diagnosis::PredicateFlip`]'s evidence came from.
///
/// N5's promise is a RECORDED flip, and for every flip the engine
/// reads out of two verdict logs that is what this says. The second
/// arm exists because one honest case has no record to read: the
/// realized sweep prunes candidate pairs and a collapsed fragment
/// group re-runs none of its own probes, so a pair's population can
/// be EMPTY in a run whose geometry moved underneath the name. The
/// recovery rung re-executes that pair at diagnosis time and reports
/// the flip it finds — a true flip of a real predicate, and one no
/// log contains. A reader that treats the two alike would be citing
/// a line of a log that was never written, so the distinction is a
/// field rather than prose.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FlipSource {
    /// Read out of both runs' verdict logs (the population diff).
    VerdictLog,
    /// Recomputed at diagnosis time by re-running ONE of the vanished
    /// name's discriminator pairs — the one named here — against the
    /// prior and the current context (issue 134's rung). Nothing was
    /// written to any log.
    ///
    /// The partner rides the marker rather than sitting beside it
    /// because it is only meaningful for this arm: a flip read out of
    /// a log is attributed to a NODE, and this one is attributed to a
    /// PAIR, which is a strictly finer answer that the type should not
    /// let a reader ask for in the other case.
    ShadowExec {
        /// The discriminator partner whose side verdict changed.
        partner: Box<StableName>,
    },
}

/// Why the shadow-exec rung refused ([`Diagnosis::ShadowExecDeclined`]).
///
/// Both arms are cases where the rung was ELIGIBLE — the name carries
/// a pair and the log has no population for it — and could not finish.
/// A reader who sees one of these knows the evidence exists and what
/// stood between the diagnosis and it, which is a different fact from
/// "no evidence".
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ShadowExecRefusal {
    /// The pair is wider than [`SHADOW_EXEC_MAX_PAIRS`], so
    /// re-executing it is not the bounded diagnosis-time work the rung
    /// is allowed to be.
    PairTooWide {
        /// How many discriminator partners the pair carries.
        pairs: usize,
        /// The ceiling it exceeded.
        ceiling: usize,
    },
    /// A probe refused typed — a dangling face, a vertex without a
    /// point, a non-planar carrier, or an in-band margin the shadow
    /// run met where the recorded run did not. The emission's own
    /// sentence, carried rather than swallowed: these are kernel-gap
    /// and band facts a reader can act on, and turning one into a
    /// silent fall-through is exactly the fail-quiet this kernel
    /// refuses.
    ProbeRefused {
        /// The refusal, rendered through its own `Display`.
        probe: String,
    },
}

// The WHY clause of [`Diagnosis::ShadowExecDeclined`]'s sentence: what
// stood between the diagnosis and evidence that exists.
impl core::fmt::Display for ShadowExecRefusal {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::PairTooWide { pairs, ceiling } => write!(
                f,
                "{pairs} discriminator partners exceeds the {ceiling} this diagnosis \
                 is allowed to re-execute"
            ),
            Self::ProbeRefused { probe } => write!(f, "a probe refused — {probe}"),
        }
    }
}

/// The shadow-exec rung's ceiling: the widest discriminator pair it
/// will re-execute.
///
/// The work is two face probes per partner, so the rung's cost is
/// linear in the `SideOf` vector's length and this is what keeps
/// "diagnosis-time only" a bound rather than a hope. The widest
/// vector the evaluation corpus mints is TWELVE, measured and pinned
/// (`bool7_shadow_exec::the_corpus_widest_pair_is_twelve`); a pair
/// STRICTLY above this number refuses
/// ([`ShadowExecRefusal::PairTooWide`]).
///
/// `pub` because the suite's ceiling fixture authors a pair of
/// `ceiling + 1` partners and cannot name that width otherwise.
pub const SHADOW_EXEC_MAX_PAIRS: usize = 32;

/// Why a name vanished.
///
/// N5's four arms verbatim, plus two additions and one field that are
/// NOT N5's and are marked as such wherever they are read: the
/// reserved `WitnessBifurcation` arm (SOLVER-DESIGN W3, constructed
/// by the M6 solver), [`Self::ShadowExecDeclined`], and
/// [`Self::PredicateFlip`]'s `source`, which says whether the flip was
/// read out of a log or recomputed at diagnosis time. A consumer
/// matching this enum is matching more than N5 wrote, and the
/// difference is where a flip's provenance lives.
#[derive(Debug, Clone, PartialEq)]
pub enum Diagnosis {
    /// A recorded predicate flip on the name's derivation path — the
    /// pillar's promise.
    PredicateFlip {
        /// The flipped predicate's k_stats name.
        predicate: &'static str,
        /// Its sign in the last-good run.
        from: Sign,
        /// Its sign now.
        to: Sign,
        /// WHERE the flip was found — a recorded population, or a
        /// shadow execution that recomputed one the run never wrote
        /// ([`FlipSource`]).
        source: FlipSource,
    },
    /// The shadow-exec rung REFUSED on a vanish it was otherwise
    /// eligible for, and says why ([`ShadowExecRefusal`]) — never a
    /// silent fall-through to a weaker rung that would read as
    /// "cause not in evidence" when the cause was in evidence and
    /// merely unreachable.
    ShadowExecDeclined {
        /// The vanished name's minting node.
        node: RecipeNodeId,
        /// Why the rung refused.
        reason: ShadowExecRefusal,
    },
    /// A structural parameter changed on the derivation path.
    StructuralParam {
        /// The node whose structural parameter changed.
        node: RecipeNodeId,
        /// The structural slot.
        param: SlotId,
    },
    /// A recipe edit touched the derivation path.
    RecipeEdit {
        /// The edit, by its structural effect.
        edit: RecipeEditRef,
    },
    /// An operand name embedded in the derivation vanished upstream;
    /// its own resolution failure carries the root cause.
    Cascade {
        /// The vanished upstream operand name.
        through: StableName,
    },
    /// A stable-name resolution failed through a sketch node whose
    /// branch selection refused (W3's payload verbatim; M6 constructs
    /// this arm).
    WitnessBifurcation(Box<WitnessBifurcation>),
}

impl Diagnosis {
    /// The total fallback, honest about its limits: the recorded
    /// reference disagrees with the recipe as it stands and the CAUSE
    /// IS NOT IN EVIDENCE. `NodeChanged` names the minting node as the
    /// SITE of the disagreement, never a claim that an edit happened.
    ///
    /// Reachable two ways, and the vocabulary is the same for both, so
    /// the value has one home: with no evidence to weigh — a
    /// mid-evaluation door, which has no prior run to diff — and with
    /// evidence that came up empty, through the population-cancel
    /// blind spot (`vdiff` module docs) or through SWEEP PRUNING (the
    /// realized sweep records no verdicts for pruned pairs, so an
    /// interaction-boundary vanish can land here — ratified
    /// 2026-07-29, NAMING-DESIGN N5 as amended). [`shadow_exec_flip`]
    /// answers part of that case before this one is reached; its docs
    /// say which part.
    pub(crate) fn cause_not_in_evidence(node: RecipeNodeId) -> Self {
        Self::RecipeEdit {
            edit: RecipeEditRef::NodeChanged { node },
        }
    }
}

// Rendered as the WHY clause of [`ResolveError::Vanished`]'s message:
// each arm states its cause; payload-holding arms forward the
// payload's own rendering.
impl core::fmt::Display for Diagnosis {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::PredicateFlip {
                predicate,
                from,
                to,
                source: FlipSource::VerdictLog,
            } => write!(
                f,
                "predicate {predicate} flipped from {from} to {to} on the name's \
                 derivation path"
            ),
            // The recovered flip says so: it is a real flip of a real
            // predicate, and it is in no log a reader could go and
            // check (the rung's docs).
            Self::PredicateFlip {
                predicate,
                from,
                to,
                source: FlipSource::ShadowExec { partner },
            } => write!(
                f,
                "predicate {predicate} flipped from {from} to {to} against the {partner} \
                 — recovered by re-running the pair at diagnosis time, because neither \
                 run recorded a verdict for it"
            ),
            Self::ShadowExecDeclined { node, reason } => write!(
                f,
                "a run recorded no verdict for the vanished pair at node {}, and \
                 re-running it was refused: {reason}",
                node.0
            ),
            Self::StructuralParam { node, param } => write!(
                f,
                "a structural parameter changed on the derivation path (node {}, slot \
                 {})",
                node.0,
                param.label()
            ),
            // A SITE of difference, not a claim that an edit happened
            // (module docs: the total fallback arm reaches this on a
            // never-edited document pair).
            Self::RecipeEdit { edit } => write!(
                f,
                "the recorded reference disagrees with the recipe as it stands on the \
                 derivation path ({edit})"
            ),
            Self::Cascade { through } => write!(
                f,
                "the upstream {through} vanished first; its own \
                 resolution failure carries the root cause"
            ),
            Self::WitnessBifurcation(refusal) => {
                write!(f, "{}", crate::witness::BranchSelectionRefused(refusal))
            }
        }
    }
}

/// A recipe edit, referenced by its structural effect on the node it
/// touched (N5's `RecipeEditRef`, concrete shape reported: edits are
/// not logged inside `Doc`, so the reference is derived from the
/// document pair — which is total, because node ids are never
/// reused).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecipeEditRef {
    /// The node was deleted (it once existed: its id is below the
    /// document's mint counter).
    NodeDeleted {
        /// The deleted node.
        node: RecipeNodeId,
    },
    /// The node was inserted.
    NodeInserted {
        /// The inserted node.
        node: RecipeNodeId,
    },
    /// The node's payload changed.
    NodeChanged {
        /// The changed node.
        node: RecipeNodeId,
    },
    /// The id was never minted by this document — no edit of THIS
    /// document produced the reference (a foreign or corrupt name,
    /// surfaced honestly rather than blamed on a delete).
    ForeignNode {
        /// The unknown id.
        node: RecipeNodeId,
    },
}

// Prose for the edit-reference parentheticals in [`ResolveError`]'s
// and [`Diagnosis`]'s messages.
impl core::fmt::Display for RecipeEditRef {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::NodeDeleted { node } => write!(f, "node {} was deleted", node.0),
            Self::NodeInserted { node } => write!(f, "node {} was inserted", node.0),
            // A difference statement, not an edit claim — this arm is
            // the diff fallback's site vocabulary.
            Self::NodeChanged { node } => write!(f, "node {}'s payload differs", node.0),
            Self::ForeignNode { node } => {
                write!(f, "node {} was never minted by this document", node.0)
            }
        }
    }
}

/// The recorded tie a reference ran into (N2's tie mark, as evidence).
#[derive(Debug, Clone, PartialEq)]
pub struct TieWitness {
    /// The node whose table records the tie.
    pub node: RecipeNodeId,
    /// The tied table row (the referenced name itself, or the widened
    /// base name on an `order_along` over-tie).
    pub at: StableName,
    /// How many equally-admissible candidates tie there.
    pub width: u32,
}

/// The last-good table entry of a vanished name (N5: entity kind,
/// owning body name, and the mesh patch key of the last evaluation —
/// GQ7's ghost-rendering payload). Selection state holds name +
/// tombstone, never a bare arena key.
#[derive(Debug, Clone, PartialEq)]
pub struct Tombstone {
    /// The vanished entity's kind.
    pub kind: EntityKind,
    /// The owning body's stable name (in the last-good evaluation).
    pub body: StableName,
    /// Where the entity's render data lives in the last evaluation's
    /// tessellation.
    pub patch: MeshPatchKey,
}

/// Addresses one entity's tessellation patch in a specific
/// evaluation: the node whose value carried the entity, the output
/// body index, and the entity's arena key — exactly the mesh crate's
/// back-reference vocabulary (`FacePatch::face`,
/// `BoundaryPolyline::edge`/vertices), scoped to the evaluation the
/// mesh came from.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MeshPatchKey {
    /// The node whose evaluation exposed the entity.
    pub node: RecipeNodeId,
    /// The output body and entity key within that node's value.
    pub entity: EntityRef,
}

/// A successful resolution: the first (evaluation-order) node whose
/// table carries the name, and the entity it denotes there.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Resolved {
    /// The carrying node.
    pub node: RecipeNodeId,
    /// The denoted entity in that node's value.
    pub entity: EntityRef,
}

/// A resolution failure: the N5 error VERBATIM plus the offered
/// repair candidates riding next to it (module docs — the spec D9
/// "wrapping" choice). Offers are suggestions for an explicit
/// `Rebind` and never an auto-repair (the policy menu is EMPTY).
#[derive(Debug, Clone, PartialEq)]
pub struct ResolutionFailure {
    /// The typed failure.
    pub error: ResolveError,
    /// N3-style structural offers (a constituent's merged name, a
    /// merged name's constituents, an over-tie group's collapse
    /// survivor). Empty when nothing structural offers itself.
    pub offers: Vec<StableName>,
}

/// A name whose minting node has no usable value in this evaluation:
/// the reference is INDETERMINATE, not vanished — it resolves again
/// when the node evaluates (same vocabulary as appearance's loss
/// causes; kept outside [`ResolveError`], which is N5's closed
/// naming-verdict trio).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResolveIndeterminate {
    /// The minting node failed this evaluation.
    TargetFailed {
        /// The failed node.
        node: RecipeNodeId,
    },
    /// The minting node was poisoned by an upstream failure.
    TargetPoisoned {
        /// The nearest failed ancestor.
        through: RecipeNodeId,
    },
    /// The minting node has no result (canceled run's suffix).
    TargetNotEvaluated {
        /// The unevaluated node.
        node: RecipeNodeId,
    },
}

// The human-readable rendering (LIB-DOORS F6 shape): each arm states
// the PROBLEM — the minting node's standing, which is the half a user
// can act on — plus the fact that makes this vocabulary its own: the
// reference is indeterminate, not vanished, so the recourse is always
// to restore the node's value, never to rebind. The three arms say
// what the hit-test and interrogate doors' identical arms say — the
// same fact about the same evaluation reached through a different
// door — and the shared recourse tail ("the repair is upstream, at
// node N") is deliberately word-for-word across the three standing
// renderings, hand-synced: each door's sentence differs in subject
// and consequence, so only the tail is common and it is too small a
// fragment to be worth a shared helper.
impl core::fmt::Display for ResolveIndeterminate {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::TargetFailed { node } => write!(
                f,
                "the name's minting node {} failed this evaluation, so the reference cannot be \
                 answered right now — fix the node's own failure and it resolves again",
                node.0
            ),
            Self::TargetPoisoned { through } => write!(
                f,
                "the name's minting node is poisoned by the failure at node {}, so the \
                 reference cannot be answered right now — the repair is upstream, at node {}",
                through.0, through.0
            ),
            Self::TargetNotEvaluated { node } => write!(
                f,
                "the name's minting node {} has no result in this evaluation (a canceled run's \
                 suffix) — re-evaluate and the reference resolves again",
                node.0
            ),
        }
    }
}

impl core::error::Error for ResolveIndeterminate {}

/// One resolution's outcome (total: every input name gets exactly one
/// of these, never a panic).
#[derive(Debug, Clone, PartialEq)]
pub enum Resolution {
    /// The name denotes exactly one entity.
    Resolved(Resolved),
    /// The typed N5 failure (with offers).
    Failed(ResolutionFailure),
    /// The evaluation cannot answer (target failed/poisoned/missing).
    Indeterminate(ResolveIndeterminate),
}

/// One run's context: the document and its evaluation.
#[derive(Clone, Copy)]
pub struct RunCtx<'a, T: Decide> {
    /// The document the evaluation ran on.
    pub doc: &'a Doc<ProfileProgram>,
    /// The evaluation.
    pub eval: &'a Evaluation<T>,
}

/// Resolves `name` against a single run, with no prior-run history
/// (the diagnosis then rests on current-run evidence only — module
/// docs). Prefer [`resolve_with_prior`] whenever a last-good run
/// exists.
pub fn resolve<T: Decide>(new: RunCtx<'_, T>, name: &StableName) -> Resolution {
    resolve_impl(new, NoPrior, name)
}

/// Resolves `name` against the new run with the last-good run as
/// diagnosis context: `Vanished` carries the verdict-diff diagnosis
/// (N5's promise) and the tombstone when the prior run still resolved
/// the name. The two runs may use different scalars (verdict logs are
/// scalar-independent).
pub fn resolve_with_prior<T: Decide, U: Decide>(
    new: RunCtx<'_, T>,
    prior: RunCtx<'_, U>,
    name: &StableName,
    tol: Tol,
) -> Resolution {
    resolve_impl(new, Prior { ctx: prior, tol }, name)
}

/// Enriches one appearance loss with the full N5 ladder (spec D9's
/// enrichment mapping), single-run form: no prior evaluation, so
/// `Vanished` diagnoses rest on current-run evidence only. Prefer
/// [`enrich_appearance_loss_with_prior`] when a last-good run exists.
///
/// The per-arm mapping ([`AppearanceLossCause`] → [`Resolution`]):
///
/// - `Ambiguous { at, .. }` → [`ResolveError::Ambiguous`] derived by
///   `table.lookup(name)` at node `at` (the loss recorded the first
///   carrying node; the Tied entry there IS the tie — its width and
///   site fill the [`TieWitness`], and the candidates are the tie row
///   expressed in names, module docs).
/// - `NodeGone` → [`ResolveError::NodeGone`] with the derived
///   [`RecipeEditRef`].
/// - `Vanished { candidates }` → [`ResolveError::Vanished`] with the
///   diagnosis ladder's verdict; the coarse structural candidates
///   reappear among [`ResolutionFailure::offers`] (the spec D9
///   wrapping choice: offers ride NEXT TO the byte-verbatim N5 error,
///   never inside it).
/// - `TargetFailed`/`TargetPoisoned`/`TargetNotEvaluated` →
///   [`Resolution::Indeterminate`] (indeterminate, not vanished —
///   same vocabulary on both sides of the hook).
///
/// Total and honest: a loss row whose recorded cause no longer
/// matches the evaluation (stale row against a different run) falls
/// through to the full ladder rather than fabricating the recorded
/// shape.
pub fn enrich_appearance_loss<T: Decide>(new: RunCtx<'_, T>, loss: &AppearanceLoss) -> Resolution {
    enrich_impl(new, NoPrior, loss)
}

/// [`enrich_appearance_loss`] with the last-good run as diagnosis
/// context: `Vanished` gains the verdict-diff diagnosis and the
/// tombstone (GQ7 ghost payload), exactly as [`resolve_with_prior`].
pub fn enrich_appearance_loss_with_prior<T: Decide, U: Decide>(
    new: RunCtx<'_, T>,
    prior: RunCtx<'_, U>,
    loss: &AppearanceLoss,
    tol: Tol,
) -> Resolution {
    enrich_impl(new, Prior { ctx: prior, tol }, loss)
}

fn enrich_impl<T: Decide, P: PriorCtx>(
    new: RunCtx<'_, T>,
    prior: P,
    loss: &AppearanceLoss,
) -> Resolution {
    // The Ambiguous arm maps DIRECTLY off the recorded site (spec D9:
    // candidates and witness by table lookup at `at`); everything
    // else — and any stale row — takes the full ladder below.
    if let AppearanceLossCause::Ambiguous { at, .. } = &loss.cause
        && let Some(v) = new.eval.value(*at)
        && let Some(Entry::Tied(ents)) = v.name_table.lookup(&loss.name)
    {
        return Resolution::Failed(ResolutionFailure {
            error: ResolveError::ambiguous(&loss.name, loss.name.clone(), *at, ents.len() as u32),
            offers: Vec::new(),
        });
    }
    resolve_impl(new, prior, &loss.name)
}

/// Rebind suggestions for every appearance-carrying name (the spec D9
/// BANKED obligation, ruled at PR 7 review A1): the operand→final
/// paint gap means an attribute on an operand-node name resolves on
/// the intermediate body only — the final node's corresponding face
/// is a DIFFERENT derivation (`FromA(x)` ≠ `x`, N1 identity), showing
/// neither paint nor loss. The explicit repair must be ergonomic:
/// this maps EVERY appearance key (resolving or not — the gap is
/// silent by design, so suggestions cannot be gated on a loss) to the
/// evaluation's derivations wrapping it ([`rebind_suggestions`]'s
/// ladder). Total over the store: a name with nothing wrapping it
/// maps to an empty list, never a dropped row. Suggestions feed an
/// explicit [`crate::edit::DocEdit::Rebind`] — which also MOVES the
/// appearance key (the attribute rides the name) — and nothing
/// follows automatically (the ratified EMPTY policy menu).
pub fn appearance_rebind_suggestions<T: Decide>(
    appearance: &AppearanceMap,
    eval: &Evaluation<T>,
) -> BTreeMap<StableName, Vec<StableName>> {
    appearance
        .keys()
        .map(|name| (name.clone(), rebind_suggestions(eval, name)))
        .collect()
}

/// The prior-run capability, monomorphized away: `NoPrior` for the
/// single-run entry, a [`RunCtx`] for the full ladder.
trait PriorCtx {
    fn diagnose<T: Decide>(
        &self,
        new: RunCtx<'_, T>,
        name: &StableName,
        path: &BTreeSet<RecipeNodeId>,
    ) -> Option<Diagnosis>;
    fn tombstone<T: Decide>(&self, new: RunCtx<'_, T>, name: &StableName) -> Option<Tombstone>;
}

struct NoPrior;

/// The last-good run AND the band the ladder decides at — what a
/// with-history diagnosis needs and a single-run resolution does not.
///
/// [`Tol`] is a zero-sized witness that the process committed a
/// tolerance (D4), so it carries no per-run band and cannot: "the
/// prior at ε_a, the current at ε_b" is unrepresentable in one
/// process. It rides here rather than on [`RunCtx`] because only the
/// with-prior ladder re-executes a predicate and so needs it at all.
#[derive(Clone, Copy)]
struct Prior<'a, U: Decide> {
    ctx: RunCtx<'a, U>,
    tol: Tol,
}

impl<U: Decide> Prior<'_, U> {
    /// The document the prior run is OF.
    fn doc(&self) -> &Doc<ProfileProgram> {
        self.ctx.doc
    }
}

impl PriorCtx for NoPrior {
    fn diagnose<T: Decide>(
        &self,
        _new: RunCtx<'_, T>,
        _name: &StableName,
        _path: &BTreeSet<RecipeNodeId>,
    ) -> Option<Diagnosis> {
        None
    }

    fn tombstone<T: Decide>(&self, _new: RunCtx<'_, T>, _name: &StableName) -> Option<Tombstone> {
        None
    }
}

impl<U: Decide> PriorCtx for Prior<'_, U> {
    /// The with-history diagnosis ladder (deterministic; first honest
    /// evidence wins): path-restricted verdict flips, then structural
    /// parameters on the path, then recipe edits on the path, then
    /// the same three globally (geometry-mediated effects still land
    /// their flips at the deciding node, so the global lanes are the
    /// honesty fallback, not the common case).
    ///
    /// Attribution among several path flips, in order:
    ///
    /// 1. **A recorded `name_frag_*` flip wins** — those predicates
    ///    are the name's OWN qualifier vocabulary, so a discriminator
    ///    flip is definitionally the flip that re-qualified the
    ///    fragment.
    /// 2. **The shadow-exec rung** ([`shadow_exec_flip`], issue 134),
    ///    which recovers a discriminator flip the run never recorded.
    ///    It sits HERE, above the generic fallback and not below it,
    ///    for the same reason rung 1 does: a recovered
    ///    `name_frag_side_of` flip is the name's own vocabulary, and
    ///    an incidental `bool_*` flip at the same node — the
    ///    containment walk re-deciding when two operands come apart —
    ///    is not. Ranking the incidental flip first would answer "why
    ///    did this fragment name vanish" with a sentence about the
    ///    boolean's interior.
    /// 3. Otherwise the first flip in deterministic order.
    ///
    /// This is a consumer-side attribution choice — the diff engine
    /// itself stays cause-agnostic and unspecialized.
    fn diagnose<T: Decide>(
        &self,
        new: RunCtx<'_, T>,
        name: &StableName,
        path: &BTreeSet<RecipeNodeId>,
    ) -> Option<Diagnosis> {
        let recorded = |flips: &[(RecipeNodeId, VerdictFlip)], family_only: bool| {
            let mut it = flips.iter();
            let hit = if family_only {
                it.find(|(_, f)| f.predicate.starts_with(crate::names::FAMILY))
            } else {
                it.next()
            };
            hit.map(|(_, f)| Diagnosis::PredicateFlip {
                predicate: f.predicate,
                from: f.from,
                to: f.to,
                source: FlipSource::VerdictLog,
            })
        };
        let flips = diff_verdicts(self.ctx.eval, new.eval);
        let on_path = flips.flips_on_nodes(path);
        if let Some(d) = recorded(&on_path, true) {
            return Some(d);
        }
        if let Some(d) = shadow_exec_flip(new, self.ctx, name, self.tol) {
            return Some(d);
        }
        if let Some(d) = recorded(&on_path, false) {
            return Some(d);
        }
        let ddiff = self.doc().diff(new.doc);
        if let Some((node, param)) =
            structural_param_change(self.doc(), new.doc, &ddiff, Some(path))
        {
            return Some(Diagnosis::StructuralParam { node, param });
        }
        if let Some(edit) = recipe_edit_change(self.doc(), new.doc, &ddiff, Some(path)) {
            return Some(Diagnosis::RecipeEdit { edit });
        }
        // Global fallbacks (off-path evidence, in the same order).
        let global = flips.report();
        if let Some(d) = recorded(&global, true).or_else(|| recorded(&global, false)) {
            return Some(d);
        }
        if let Some((node, param)) = structural_param_change(self.doc(), new.doc, &ddiff, None) {
            return Some(Diagnosis::StructuralParam { node, param });
        }
        recipe_edit_change(self.doc(), new.doc, &ddiff, None)
            .map(|edit| Diagnosis::RecipeEdit { edit })
    }

    fn tombstone<T: Decide>(&self, _new: RunCtx<'_, T>, name: &StableName) -> Option<Tombstone> {
        let (node, entity) = lookup_unique(self.ctx.eval, name)?;
        let table = &self.ctx.eval.value(node)?.name_table;
        let Some(body) = table.name_of(&EntityRef {
            body: entity.body,
            key: EntityKey::Body,
        }) else {
            // An Ok value whose table has no body row is an
            // emission-totality violation — the same event
            // hit-testing screams about (`HitTestError::Unnamed`,
            // spec D4). Scream in debug too (review Finding 4);
            // release degrades to no-tombstone (the ghost payload is
            // cosmetic, the resolution verdict is unaffected).
            debug_assert!(
                false,
                "emission totality violation: no body row for {entity:?} at {node:?}"
            );
            return None;
        };
        Some(Tombstone {
            kind: name.kind,
            body: body.clone(),
            patch: MeshPatchKey { node, entity },
        })
    }
}

fn resolve_impl<T: Decide, P: PriorCtx>(
    new: RunCtx<'_, T>,
    prior: P,
    name: &StableName,
) -> Resolution {
    // 1. NodeGone: the minting node is not live.
    if let Some(error) = ResolveError::node_gone(name, new.doc) {
        return Resolution::Failed(ResolutionFailure {
            error,
            offers: Vec::new(),
        });
    }

    // 2. The table lookup (N4: resolution IS this read). First
    //    carrying node in evaluation order wins (deterministic;
    //    pass-through tables carry the same rows).
    match lookup(new.eval, name) {
        Some((node, Entry::Unique(entity))) => {
            return Resolution::Resolved(Resolved {
                node,
                entity: *entity,
            });
        }
        Some((node, Entry::Tied(ents))) => {
            return Resolution::Failed(ResolutionFailure {
                error: ResolveError::ambiguous(name, name.clone(), node, ents.len() as u32),
                offers: Vec::new(),
            });
        }
        None => {}
    }

    // 3. The order_along over-tie widening (spec D1): a ranked
    //    fragment reference whose group over-tied resolves Ambiguous
    //    against the WIDENED base row — never a mis-bind.
    let mut offers = Vec::new();
    if let Some(base) = widened_base(name) {
        match lookup(new.eval, &base) {
            Some((node, Entry::Tied(ents))) => {
                return Resolution::Failed(ResolutionFailure {
                    error: ResolveError::ambiguous(name, base, node, ents.len() as u32),
                    offers: Vec::new(),
                });
            }
            // The group collapsed to a single fragment: the ranked
            // name is gone; the surviving base is the offer.
            Some((_, Entry::Unique(_))) => offers.push(base),
            None => {}
        }
    }

    // 4. The minting node's standing decides Vanished vs
    //    Indeterminate.
    match new.eval.nodes.get(&name.node) {
        Some(NodeResult::Ok(_)) => {}
        Some(NodeResult::Failed(_)) => {
            return Resolution::Indeterminate(ResolveIndeterminate::TargetFailed {
                node: name.node,
            });
        }
        Some(NodeResult::Poisoned { through }) => {
            return Resolution::Indeterminate(ResolveIndeterminate::TargetPoisoned {
                through: *through,
            });
        }
        None => {
            return Resolution::Indeterminate(ResolveIndeterminate::TargetNotEvaluated {
                node: name.node,
            });
        }
    }

    // 5. Vanished. N3 structural offers first (merge/unmerge), then
    //    the diagnosis ladder.
    offers.extend(merge_offers(new.eval, name));

    // Cascade dominates: an embedded operand name that itself fails
    // to resolve carries the root cause (its own diagnosis chains).
    let mut cascade: Option<StableName> = None;
    for_each_inner(name, &mut |inner| {
        if cascade.is_none() && lookup(new.eval, inner).is_none() {
            cascade = Some(inner.clone());
        }
    });
    let path = derivation_nodes(name);
    let diagnosis = if let Some(through) = cascade {
        Diagnosis::Cascade { through }
    } else {
        prior
            .diagnose(new, name, &path)
            // The qualifier-delta rung (review Finding 1 ruling):
            // when the verdict-diff and doc-diff lanes have no
            // evidence — the population-cancel blind spot, or a
            // single-run resolve — the N2 discriminator verdicts
            // recorded IN the names themselves are still evidence.
            .or_else(|| qualifier_delta(new.eval, name))
            // Every rung above came up empty: no verdict flip, no doc
            // delta, no recorded qualifier delta.
            .unwrap_or_else(|| Diagnosis::cause_not_in_evidence(name.node))
    };
    let last_good = prior.tombstone(new, name);
    Resolution::Failed(ResolutionFailure {
        error: ResolveError::Vanished {
            name: name.clone(),
            diagnosis,
            last_good,
        },
        offers,
    })
}

/// The SHADOW-EXECUTION rung: when the vanished name's own
/// discriminator PAIRS recorded no verdict at all in one of the two
/// runs, re-run them against both contexts and report the first
/// partner whose side verdict changed.
///
/// # Why a rung exists here at all
///
/// `Vanished`'s evidence is a diff of two verdict LOGS, and a log can
/// only be diffed where it has entries. The realized sweep prunes
/// candidate pairs (C10), so exactly the interaction-boundary edits
/// that vanish a discriminated fragment — overlapping to disjoint —
/// are the ones that leave the pair's population EMPTY. The
/// population never existed; there is nothing to cancel and nothing
/// to reconcile. The banked alternative, recording pseudo-verdicts
/// for pruned pairs, stays ruled out: it re-introduces the quadratic
/// in space that pruning removed.
///
/// # What is re-executed, against what
///
/// The pair is written IN the name. A `Fragment(SideOf(v))` qualifier
/// is one entry per SEAM PARTNER, each a recipe-covariant
/// [`StableName`], so the vanished name names its own partners. For
/// each of them, in the qualifier's order:
///
/// - the FACE is the vanished fragment in the prior run and the
///   SURVIVOR — the same name without its trailing qualifier — in the
///   current one. Those are the two faces the qualifier is about.
/// - the PARTNER is resolved at the boolean's OPERAND in each run:
///   the body the minting node actually consumed, found by walking
///   the minting node's recipe inputs to the one whose table carries
///   the partner name ([`operand_face`]). A partner name is minted at
///   the operand's own node and carried unchanged through every
///   name-preserving placer above it, so resolving it at its first
///   carrying node reads the carrier off the UNPLACED body and probes
///   against a wall that is not where the boolean saw it.
/// - the per-vertex sign stream becomes a [`crate::names::SideVerdict`]
///   through the emission's OWN rule
///   ([`crate::names::aggregate_side`]) — one door, so the rung
///   cannot disagree with the emitter about what a fragment's side is.
/// - the prior side is CALIBRATED against the verdict the qualifier
///   recorded: the rung just asked the run's own question of the run's
///   own face, so the answer must be the record or, if the probe read
///   the carrier from the other side of the same plane, its exact
///   negation. Anything else and the rung is not probing the pair the
///   name recorded, and it says nothing. This is what makes the
///   reported `from` the qualifier's own verdict rather than the
///   probe's convention, and what makes two faces with different
///   boundaries comparable at all: a side is not a vertex count.
///
/// Every input is already in hand — the bodies ride the node values,
/// the faces come from the same table lookup resolution itself uses,
/// and the partner's plane is read through the emission's own
/// `face_plane` door. The recipe is read for ONE thing, the minting
/// node's input EDGES; **nothing replays the op**.
///
/// # What it costs, and what it refuses
///
/// Diagnosis-time only, and nothing reaches a log: the probes run
/// detached and their recording is read, never spliced. The work is
/// two face probes per partner, bounded by
/// [`SHADOW_EXEC_MAX_PAIRS`]; above it, and on a typed probe refusal,
/// the rung answers [`Diagnosis::ShadowExecDeclined`] rather than
/// falling through silently.
///
/// It declines TO THE NEXT RUNG, reporting nothing, in five cases,
/// and each is an absence of evidence rather than a refusal: the name
/// carries no `SideOf` qualifier; a run does not hold the face or the
/// partner's operand body; a partner's verdict has no single [`Sign`]
/// on one side (`SideVerdict::Mixed` — definite probes on both sides
/// is not one sign, the R9 honesty pin); the prior side does not
/// calibrate against the record; and **no partner's verdict
/// changed**.
///
/// # The two halves of the SideOf vanish, and only one is recovered
///
/// The last two cases are the COLLAPSE half, and it is a limit, not a
/// gap. A fragment group stops being multi-fragment whenever the
/// partner walls stop CUTTING the face — the bar lands short of the
/// far edge, or withdraws on the side it was already on — and the
/// walls have not crossed the fragment, so every side verdict is what
/// it was and the survivor still satisfies the vanished name's own
/// vector. There is no flip; the rung finds none and the vanish rests
/// on the later rungs. What this rung recovers is the half where a
/// side MOVED: the partner crossed, the sweep pruned the pair, and
/// the verdict that re-qualified the name was never written down.
/// Pruning the pair and re-qualifying the name are different events,
/// and only the second is a flip.
///
/// The `OrderAlong` half of the same issue is not recovered either,
/// for a different reason: `Qualifier::OrderAlong { rank, of }`
/// records an ordinal and a group size and NO partner, so the pair it
/// was ranked against cannot be read back out of the name — and the
/// pruned run has no sibling left to rank against, a single-member
/// group running zero `name_frag_order_along` pairs. Recovering it
/// needs the naming vocabulary to carry the partner, which is the
/// names lane's design surface. This is the one statement of that
/// limit; the sites that need it point here.
///
/// # The trigger is node-granular, which is a narrowing
///
/// A run's log holds per-node POPULATIONS, not per-pair attributions,
/// so "this PAIR recorded nothing" is not a question the log can be
/// asked. The trigger reads the minting node's whole
/// `name_frag_side_of` population instead, in either run. A node
/// carrying a SECOND fragment group whose pair was not pruned
/// therefore keeps the rung out of the pruned one
/// (`bool7r1_probes::a_second_pair_at_the_node_keeps_the_rung_out_of_a_pruned_one`
/// pins it). Making it pair-granular means attributing recorded
/// verdicts to pairs, which is a verdict-LOG format change.
fn shadow_exec_flip<T: Decide, U: Decide>(
    new: RunCtx<'_, T>,
    prior: RunCtx<'_, U>,
    name: &StableName,
    tol: Tol,
) -> Option<Diagnosis> {
    // The pair, read off the name: the trailing SideOf qualifier.
    let RoleSeg::Fragment(Qualifier::SideOf(partners)) = name.path.last()? else {
        return None;
    };
    // The trigger: at least one run recorded no `name_frag_side_of`
    // verdict at the minting node at all. A recorded population is the
    // log's evidence and belongs to the rungs above.
    if !pair_population_is_empty(prior.eval, name.node)
        && !pair_population_is_empty(new.eval, name.node)
    {
        return None;
    }
    if partners.len() > SHADOW_EXEC_MAX_PAIRS {
        return Some(Diagnosis::ShadowExecDeclined {
            node: name.node,
            reason: ShadowExecRefusal::PairTooWide {
                pairs: partners.len(),
                ceiling: SHADOW_EXEC_MAX_PAIRS,
            },
        });
    }
    // The two faces the qualifier is about: the vanished fragment, in
    // the run that still had it, and the SURVIVOR it became — the same
    // name without its trailing qualifier, which is what an
    // un-fragmented group is called.
    let (old_body, old_face) = face_at(prior.eval, name)?;
    let (now_body, now_face) = face_at(new.eval, &unqualified(name)?)?;
    let refused = |e: &crate::names::NamingError| {
        Some(Diagnosis::ShadowExecDeclined {
            node: name.node,
            reason: ShadowExecRefusal::ProbeRefused {
                probe: e.to_string(),
            },
        })
    };
    for (partner, recorded) in partners {
        let (pb_old, pk_old) = operand_face(prior, name.node, partner)?;
        let (pb_new, pk_new) = operand_face(new, name.node, partner)?;
        let old = match crate::names::shadow_side_of(old_body, old_face, pb_old, pk_old, tol) {
            Ok(signs) => signs,
            Err(e) => return refused(&e),
        };
        let now = match crate::names::shadow_side_of(now_body, now_face, pb_new, pk_new, tol) {
            Ok(signs) => signs,
            Err(e) => return refused(&e),
        };
        // The emission's own rule, on both sides (one door).
        let (Some(was), Some(is), Some(from)) = (
            crate::names::aggregate_side(&old)
                .as_ref()
                .and_then(verdict_sign),
            crate::names::aggregate_side(&now)
                .as_ref()
                .and_then(verdict_sign),
            verdict_sign(recorded),
        ) else {
            // `Mixed` on a side: definite probes both ways is not one
            // sign, and this is where the COLLAPSE half lands — the
            // survivor is still cut by the wall, so it has no side.
            continue;
        };
        // THE CALIBRATION, and the D9 replay statement at the pair.
        // The recorded verdict is the same question this rung just
        // asked of the same face, so the prior side must re-execute to
        // it — up to the ONE thing that can differ, the sense of the
        // carrier the probe read. The emission reads the partner off
        // the boolean's OUTPUT body, where a subtract's tool walls face
        // the other way; the rung reads it off the OPERAND, which is
        // the only body both runs hold. A plane's orientation admits
        // exactly two answers, so agreeing with the record or negating
        // it are the only two consistent outcomes, and anything else
        // means the rung is not probing the pair the name recorded —
        // in which case it says nothing rather than guessing.
        let negated = if was == from {
            false
        } else if was == from.flip() {
            true
        } else {
            continue;
        };
        let to = if negated { is.flip() } else { is };
        if from == to {
            // This partner did not re-qualify the fragment.
            continue;
        }
        return Some(Diagnosis::PredicateFlip {
            predicate: crate::names::SIDE_OF,
            from,
            to,
            source: FlipSource::ShadowExec {
                partner: Box::new(partner.clone()),
            },
        });
    }
    None
}

/// A fragment name without its trailing qualifier — what the SAME
/// group is called once it stops being multi-fragment, and therefore
/// the current-run counterpart of a vanished fragment.
///
/// Not [`widened_base`], which pops a trailing `OrderAlong` for a
/// different purpose (the over-tie widening reads the row the
/// reference actually tied against). The operations look alike and the
/// questions are not: that one asks which ROW a ranked reference
/// landed on and must refuse a `SideOf` tail; this one asks which FACE
/// a discriminated fragment became and must refuse an `OrderAlong`
/// tail. Neither can answer the other's question.
fn unqualified(name: &StableName) -> Option<StableName> {
    if !matches!(
        name.path.last(),
        Some(RoleSeg::Fragment(Qualifier::SideOf(_)))
    ) {
        return None;
    }
    let mut base = name.clone();
    base.path.pop();
    (!base.path.is_empty()).then_some(base)
}

/// The body and face key `partner` denotes AT `node`'s operand in
/// `run` — the body the node actually consumed, not the body the name
/// was minted on.
///
/// A discriminator partner is an operand-node name, and the placers
/// above it ([`crate::node::Node::Transform`], the pattern, the
/// part-instance doors) are name-PRESERVING: the same rows ride the
/// placed table. So a plain table scan finds the partner at its
/// minting node and reads the carrier off geometry that has not been
/// placed yet. Walking the minting node's own recipe inputs picks the
/// operand instead, whatever chain of placers sits between them, and
/// that is the body whose walls the boolean cut against.
fn operand_face<'a, T: Decide>(
    run: RunCtx<'a, T>,
    node: RecipeNodeId,
    partner: &StableName,
) -> Option<(&'a topo::Body<T>, topo::FaceKey)> {
    let inputs = run.doc.node(node)?.inputs();
    inputs
        .into_iter()
        .find_map(|input| face_in(run.eval, input, partner))
}

/// The body and face key a face name denotes in ONE node's value.
fn face_in<'a, T: Decide>(
    eval: &'a Evaluation<T>,
    node: RecipeNodeId,
    name: &StableName,
) -> Option<(&'a topo::Body<T>, topo::FaceKey)> {
    let value = eval.value(node)?;
    let Entry::Unique(entity) = value.name_table.lookup(name)? else {
        return None;
    };
    let EntityKey::Face(key) = entity.key else {
        return None;
    };
    let body = crate::names::interrogate::output_body(&value.payload, entity.body).ok()?;
    Some((body, key))
}

/// The body and face key a face name denotes in one run — the same
/// table lookup resolution itself performs, projected onto the
/// geometry the probe reads.
fn face_at<'a, T: Decide>(
    eval: &'a Evaluation<T>,
    name: &StableName,
) -> Option<(&'a topo::Body<T>, topo::FaceKey)> {
    let (node, _) = lookup(eval, name)?;
    face_in(eval, node, name)
}

/// Whether `node` recorded NO `name_frag_side_of` verdict in `eval` —
/// the rung's trigger (the pair population never existed there).
fn pair_population_is_empty<T: Decide>(eval: &Evaluation<T>, node: RecipeNodeId) -> bool {
    eval.value(node).is_none_or(|v| {
        !v.verdicts
            .iter()
            .any(|w| w.predicate == crate::names::SIDE_OF)
    })
}

/// The single [`Sign`] an aggregated side verdict has, if any.
///
/// `On` HAS one: every probe decided `Zero`, so `Zero` is the
/// unanimous per-vertex sign and reporting it states what the probes
/// found. `Mixed` has none, and deriving one would be fabrication
/// (the R9 honesty pin).
///
/// [`pure_sign`] is this filtered further, for a different rung — its
/// docs carry the difference.
fn verdict_sign(v: &crate::names::SideVerdict) -> Option<Sign> {
    use crate::names::SideVerdict;
    match v {
        SideVerdict::Positive => Some(Sign::Positive),
        SideVerdict::Negative => Some(Sign::Negative),
        SideVerdict::On => Some(Sign::Zero),
        SideVerdict::Mixed => None,
    }
}

/// The qualifier-delta diagnosis rung (review Finding 1 ruling): a
/// re-qualified fragment's OLD name carries `(partner, s)` where a
/// same-shape sibling in the new tables carries `(partner, s')` —
/// the N2 discriminator verdicts are recorded in the names, so the
/// flip is derivable from recorded data even when the verdict-diff
/// engine reports nothing (its population-cancel blind spot, `vdiff`
/// module docs) or no prior run exists.
///
/// Fires only on a CLEAN delta (first match in deterministic
/// evaluation/table order): a candidate of the same kind, node, and
/// path shape, equal in every segment except ONE `SideOf` vector,
/// equal in every entry of that vector except ONE partner whose
/// verdicts are unanimous signs on both sides (`Positive` ↔
/// `Negative`). REPORTED boundary: aggregate verdicts (`Mixed`,
/// `On`) have no single-`Sign` reading in N5's `PredicateFlip`
/// payload, and multi-entry deltas have no single flip — deriving a
/// `Sign` for either would be fabrication (the R9 honesty pin), so
/// both fall through to the documented fallback.
fn qualifier_delta<T: Decide>(eval: &Evaluation<T>, name: &StableName) -> Option<Diagnosis> {
    for (_, table) in tables(eval) {
        for (candidate, _) in table.iter() {
            if let Some((from, to)) = single_pure_sideof_delta(name, candidate) {
                return Some(Diagnosis::PredicateFlip {
                    predicate: crate::names::SIDE_OF,
                    from,
                    to,
                    source: FlipSource::VerdictLog,
                });
            }
        }
    }
    None
}

/// The (from, to) sign pair iff `new` differs from `old` by exactly
/// one pure-sign `SideOf` entry ([`qualifier_delta`] docs).
fn single_pure_sideof_delta(old: &StableName, new: &StableName) -> Option<(Sign, Sign)> {
    if old.kind != new.kind || old.node != new.node || old.path.len() != new.path.len() {
        return None;
    }
    let mut delta: Option<(Sign, Sign)> = None;
    for (a, b) in old.path.iter().zip(&new.path) {
        if a == b {
            continue;
        }
        // More than one differing segment: not a single delta.
        if delta.is_some() {
            return None;
        }
        let (RoleSeg::Fragment(Qualifier::SideOf(va)), RoleSeg::Fragment(Qualifier::SideOf(vb))) =
            (a, b)
        else {
            return None;
        };
        if va.len() != vb.len() {
            return None;
        }
        for ((pa, sa), (pb, sb)) in va.iter().zip(vb) {
            if pa != pb {
                return None; // different partner sets: different shape
            }
            if sa == sb {
                continue;
            }
            if delta.is_some() {
                return None; // two entries moved: no single flip
            }
            delta = Some((pure_sign(sa)?, pure_sign(sb)?));
        }
        // A SideOf pair that differs as a whole but entry-wise not at
        // all cannot happen (same partners, same verdicts ⇒ equal);
        // delta is Some here by construction.
    }
    delta
}

/// The unanimous sign of a side verdict, if it has one (`Mixed`/`On`
/// aggregates do not — [`qualifier_delta`]'s reported boundary).
fn pure_sign(v: &crate::names::SideVerdict) -> Option<Sign> {
    match v {
        crate::names::SideVerdict::Positive => Some(Sign::Positive),
        crate::names::SideVerdict::Negative => Some(Sign::Negative),
        crate::names::SideVerdict::Mixed | crate::names::SideVerdict::On => None,
    }
}

/// Every Ok table of an evaluation, with its node, in EVALUATION
/// ORDER — the one scan this module resolves, offers and diagnoses
/// through.
///
/// The order is the determinism every consumer here leans on: "the
/// first carrying node wins", "the first honest evidence wins" and
/// the deterministic order of an offer list are all this iterator's
/// order and not four independent claims. Nodes that failed or were
/// never evaluated carry no table and are skipped — a resolution
/// reads what the run actually built.
fn tables<T: Decide>(
    eval: &Evaluation<T>,
) -> impl Iterator<Item = (RecipeNodeId, &crate::names::NameTable)> {
    eval.order
        .iter()
        .filter_map(|&id| eval.value(id).map(|v| (id, &*v.name_table)))
}

/// The first (evaluation-order) Ok table carrying `name`.
fn lookup<'a, T: Decide>(
    eval: &'a Evaluation<T>,
    name: &StableName,
) -> Option<(RecipeNodeId, &'a Entry)> {
    tables(eval).find_map(|(id, table)| table.lookup(name).map(|entry| (id, entry)))
}

/// [`lookup`], demanding a unique entry.
fn lookup_unique<T: Decide>(
    eval: &Evaluation<T>,
    name: &StableName,
) -> Option<(RecipeNodeId, EntityRef)> {
    match lookup(eval, name)? {
        (node, Entry::Unique(e)) => Some((node, *e)),
        _ => None,
    }
}

/// The widened base of a ranked fragment name: the same name without
/// its trailing `Fragment(OrderAlong)` qualifier (the row the emitter
/// ties when the group over-ties).
fn widened_base(name: &StableName) -> Option<StableName> {
    match name.path.last() {
        Some(RoleSeg::Fragment(Qualifier::OrderAlong { .. })) => {
            let mut base = name.clone();
            base.path.pop();
            Some(base)
        }
        _ => None,
    }
}

/// N3's structural offers: the merged name a retired constituent
/// entered (scan for `Merged` rows containing `name`), or a vanished
/// merged name's own constituents.
///
/// # Why this reads one level and [`rebind_suggestions`] reads all
///
/// An OFFER is a name the user can rebind TO, so it has to be a name
/// that resolves — which is what fixes the depth here, and fixes it
/// differently for the two halves:
///
/// - **Unmerge.** The constituents of `name`'s own top-level `Merged`
///   segment are the names the merge retired; when the merge stops
///   happening they are live again, exactly as spelled. A merge nested
///   DEEPER — inside `FromA(m)`, where `m` is the merged row — is not
///   dropped by this scan, it is RE-POINTED: an embedded operand name
///   that does not itself resolve makes the whole resolution a
///   `Cascade { through: m }`, which names `m` as the root cause, and
///   resolving `m` runs this function with the `Merged` segment at ITS
///   top level. So the deep case is answered where it is answerable,
///   one name up, and answered with names that resolve — which is what
///   a scan through `FromA(m)` could not do, since the offer it would
///   collect is the bare constituent and the name that would be live
///   is `FromA(x)`.
/// - **Merge.** A retired name's merged row is offered from the table
///   that HOLDS it: a live `Merged` row whose flat set COVERS the name
///   (`names::merged::covers`). A constituent is covered outright. A
///   merged face that a WIDER merge consumed — the inner row of a
///   boolean over a boolean, carried into the outer as
///   `FromA(inner:Merged(cs))` and never itself a constituent, since
///   the outer row lists `cs`'s faces re-wrapped — is covered by that
///   outer row, because every face it stood for is in the set. The
///   row is found whole and at its own depth; a candidate that merely
///   embeds it deeper (a seam across it) needs no separate offer, the
///   row itself still resolving at the node whose table minted it.
///
/// [`rebind_suggestions`] answers a different question — every
/// derivation WRAPPING a name, so a paint can follow the entity
/// forward — and every answer it gives is a whole table row, so
/// depth costs it nothing.
fn merge_offers<T: Decide>(eval: &Evaluation<T>, name: &StableName) -> Vec<StableName> {
    let mut offers = Vec::new();
    // Unmerge: the name IS a merged name — offer its constituents.
    for seg in &name.path {
        if let RoleSeg::Merged(constituents) = seg {
            offers.extend(constituents.iter().cloned());
        }
    }
    // Merge: a live Merged row covers `name`.
    for (_, table) in tables(eval) {
        for (candidate, _) in table.iter() {
            let covers = candidate.path.iter().any(|seg| match seg {
                RoleSeg::Merged(constituents) => crate::names::merged::covers(constituents, name),
                _ => false,
            });
            if covers && !offers.contains(candidate) {
                offers.push(candidate.clone());
            }
        }
    }
    offers
}

/// Rebind suggestions for a vanished-or-gapped name (spec D9's
/// suggestion ladder, general form): every SAME-KIND name in the
/// evaluation whose derivation STRUCTURALLY wraps `name` (embeds it
/// as an operand name at any depth — `FromA(x)`, `Instance{of: x}`,
/// seams, fragments of it), deterministically ordered (first carrying
/// node, then name order). These are SUGGESTIONS for an explicit
/// `Rebind` — nothing follows automatically (the ratified EMPTY
/// policy menu).
///
/// Two exclusions (review Finding 2): a name that merely MENTIONS
/// `name` as a `SideOf` discriminator PARTNER is not a derivation of
/// it — partners are the references fragments are classified
/// against, so painting a cutter wall must not suggest the other
/// body's fragments ([`walk_names`] with [`Partners::Skip`]); and
/// cross-kind candidates are excluded because `Rebind` itself
/// refuses them ([`crate::edit::EditError::RebindKindMismatch`]) —
/// offering un-rebindable names is not ergonomics.
pub fn rebind_suggestions<T: Decide>(eval: &Evaluation<T>, name: &StableName) -> Vec<StableName> {
    let mut out: Vec<StableName> = Vec::new();
    for (_, table) in tables(eval) {
        for (candidate, _) in table.iter() {
            if candidate == name || candidate.kind != name.kind || out.contains(candidate) {
                continue;
            }
            let mut wraps = false;
            walk_names(candidate, Partners::Skip, &mut |inner| {
                if inner == name {
                    wraps = true;
                }
            });
            // A merged row wraps every face it lists, and so wraps a
            // merged face those faces came from: the flat set covers
            // it (`names::merged::covers`) though no segment embeds
            // it.
            wraps |= candidate.path.iter().any(|seg| match seg {
                RoleSeg::Merged(constituents) => crate::names::merged::covers(constituents, name),
                _ => false,
            });
            if wraps {
                out.push(candidate.clone());
            }
        }
    }
    out
}

/// Name-level edit-time validation (PR 3's R6 obligation, landed
/// here): [`crate::edit::apply`] upgraded with resolution of the
/// edit's recorded [`StableName`]s against `eval`'s tables WHEN THEY
/// ARE EVALUABLE — a name whose minting node has an Ok value in
/// `eval` must be carried by some table (unique or tied; a tie is
/// recordable intent, refused only at reference-resolution time).
/// The documented carve-out for forward references stands: names
/// whose nodes are unevaluated, failed, or poisoned in `eval` are
/// not checkable here and defer to evaluation-time resolution.
///
/// Checked sites: every payload name an `InsertNode` carries
/// ([`crate::node::Node::payload_names`] is the list) and `Rebind`'s
/// target. Every other
/// edit validates exactly as [`crate::edit::apply`] — including the
/// four appearance edits, which DO carry a name: theirs resolves at
/// evaluation, into a typed [`crate::appearance::AppearanceLoss`].
/// `Rebind`'s SOURCE is deliberately unchecked too: it is the
/// stranded name being repaired.
///
/// `eval` must be an evaluation OF `doc`: the tables this door reads
/// are the evaluation's, and node ids are minted per document, so an
/// evaluation of a twin recipe satisfies the carve-out on every name
/// and answers out of the wrong tables. The pairing goes through
/// [`crate::ident::mispaired`], the one predicate the pair doors
/// share, before any name is read.
///
/// # Errors
///
/// [`crate::edit::EditError::EvaluationOfAnotherDocument`] for a
/// mispaired `eval`;
/// [`crate::edit::EditError::NameUnresolvedInEvaluation`] on a
/// checkable-but-absent name; otherwise whatever [`crate::edit::apply`]
/// returns.
pub fn apply_with_names<T: Decide>(
    doc: &Doc<ProfileProgram>,
    edit: &crate::edit::DocEdit<ProfileProgram>,
    eval: &Evaluation<T>,
    tol: Tol,
    reach: &dyn crate::mate::MateReach,
) -> Result<crate::edit::Applied<ProfileProgram>, crate::edit::EditError> {
    use crate::edit::{DocEdit, EditError};
    // The pairing, before any name is read (why: this fn's docs).
    if let Some(m) = crate::ident::mispaired(doc.id(), eval.document) {
        return Err(m.into());
    }
    let mut names: Vec<&StableName> = Vec::new();
    // EXHAUSTIVE on purpose (the `walk_names` rule): the three groups
    // below are the doc's checked/unchecked split, and a future
    // `DocEdit` variant must join one of them or the compile breaks.
    // A wildcard here would enrol a new name-carrying edit in the
    // unchecked group silently, which is the one outcome the split is
    // there to prevent.
    match edit {
        DocEdit::InsertNode { node } => names.extend(node.payload_names()),
        DocEdit::Rebind { to, .. } => names.push(to),
        // Name-carrying and deliberately unchecked here: an appearance
        // name resolves at evaluation, where a miss is a typed
        // `AppearanceLoss` rather than a silent drop, and clearing is
        // the repair path for a name that no longer resolves at all.
        // (`Rebind`'s `from` is the same carve-out, in the arm above:
        // it is the stranded name being repaired.)
        DocEdit::SetAppearance { .. }
        | DocEdit::ClearAppearance { .. }
        | DocEdit::SetAppearanceMeta { .. }
        | DocEdit::ClearAppearanceMeta { .. } => {}
        // Carry no `StableName` at all.
        DocEdit::DeleteNode { .. }
        // A list of node ids carries no name.
        | DocEdit::SetMembers { .. }
        // A program and its provenance carry no name; the names a
        // reshaping moves are the document's own, rewritten at the
        // door.
        | DocEdit::SetProgram { .. }
        | DocEdit::SetParam { .. }
        | DocEdit::SetStructuralParam { .. }
        | DocEdit::SetExpression { .. }
        | DocEdit::SetDocParam { .. }
        | DocEdit::SetDocParamValue { .. }
        | DocEdit::SetDocParamUnit { .. }
        | DocEdit::SetDocParamDistribution { .. }
        | DocEdit::ReWitness { .. }
        | DocEdit::ReWitnessBulk { .. }
        | DocEdit::SetTolerance { .. }
        | DocEdit::SetRoots { .. }
        | DocEdit::SetPlacement { .. }
        | DocEdit::UpdateReference { .. } => {}
    }
    for name in names {
        // Checkable = the minting node evaluated Ok. (Node existence
        // itself is apply's own door.)
        if eval.value(name.node).is_some() && lookup(eval, name).is_none() {
            return Err(EditError::NameUnresolvedInEvaluation { name: name.clone() });
        }
    }
    crate::edit::apply(doc, edit, tol, reach)
}

/// The nodes a name's derivation passes through: its minting node,
/// every embedded operand name's nodes (recursively), every
/// discriminator partner's nodes, and every node id a SEGMENT carries
/// in its own right — the localization set of N7 ("an edit renames
/// nothing outside derivation paths that actually pass through the
/// edited node").
///
/// The last of those is a union's member edge
/// ([`crate::names::RoleSeg::FromMember`]): the entity derives from
/// that member, and the member is named by an id rather than by a
/// name, so a walk that only visits embedded names would leave it out
/// of the localization set and out of every id check built on this.
pub fn derivation_nodes(name: &StableName) -> BTreeSet<RecipeNodeId> {
    let mut nodes = BTreeSet::from([name.node]);
    nodes.extend(name.path.iter().filter_map(crate::names::member_edge));
    for_each_inner(name, &mut |inner| {
        nodes.insert(inner.node);
        nodes.extend(inner.path.iter().filter_map(crate::names::member_edge));
    });
    nodes
}

/// Whether a name walk visits `SideOf` discriminator PARTNERS.
/// Partners are discrimination references — an edit at a partner's
/// node can re-qualify the name (N7 localization, cascade), but the
/// name is not DERIVED from the partner (suggestions must not offer
/// the other body's fragments for a painted cutter wall — review
/// Finding 2).
#[derive(Clone, Copy, PartialEq, Eq)]
enum Partners {
    /// Visit partner names (localization, cascade).
    Include,
    /// Skip partner positions (structural embedding only).
    Skip,
}

/// Visits every name embedded in `name`'s role path, recursively,
/// in path order (operand names, seam pairs, merged constituents,
/// pattern masters — and discriminator partners iff `partners` says
/// so). The match is EXHAUSTIVE on purpose: a future [`RoleSeg`] or
/// [`Qualifier`] variant embedding names must be
/// classified here or the compile breaks — or, if it embeds no name,
/// added to [`crate::names::name_free_seg`], which is the one place
/// that answer is written for every match that shares it.
/// (Review Finding 7 — no fail-quiet wildcard.)
fn walk_names<'a>(name: &'a StableName, partners: Partners, f: &mut impl FnMut(&'a StableName)) {
    fn visit<'a>(n: &'a StableName, partners: Partners, f: &mut impl FnMut(&'a StableName)) {
        f(n);
        walk_names(n, partners, f);
    }
    for seg in &name.path {
        match seg {
            // Structural embeddings: the entity derives from these.
            RoleSeg::FromA(n)
            | RoleSeg::FromB(n)
            | RoleSeg::FromMember { of: n, .. }
            | RoleSeg::SectionEdge { face: n, .. }
            | RoleSeg::SplitFragment { parent: n, .. }
            | RoleSeg::CrossingVertex { edge: n, .. }
            | RoleSeg::OnToolVertex { of: n, .. }
            | RoleSeg::Instance { of: n, .. }
            // The fillet vocabulary (M6-5): every argument is the
            // SOURCE entity the blend was born for — derivation, not
            // discrimination.
            | RoleSeg::FromTarget(n)
            | RoleSeg::BlendFace(n)
            | RoleSeg::CornerFace(n)
            | RoleSeg::BandTrim { edge: n, .. }
            | RoleSeg::BandFoot(n)
            | RoleSeg::BandCross(n)
            | RoleSeg::BandCut(n)
            | RoleSeg::BandSlit(n)
            // The shell vocabulary: each argument is the SOURCE entity
            // the twin or rim was born for — derivation, not
            // discrimination (a hole rim's index discriminates, and is
            // not a name).
            | RoleSeg::Inner(n)
            | RoleSeg::Rim(n)
            | RoleSeg::HoleRim { of: n, .. } => visit(n, partners, f),
            // ASM-2A: the DOCUMENT SEAM. An `InPart` argument is a name
            // in ANOTHER document's id space — its `RecipeNodeId`s name
            // that document's nodes, not this one's — so no local walk
            // may descend into it. Every consumer of this walk reads
            // `.node` as a local id: the persistence id check, the
            // Rebind rewrite sites, the N7 suggestion machinery. Which
            // document those ids belong to is the instantiate node's
            // `doc_ref`, and a part-document edit that breaks a local
            // name surfaces at the PART's own diagnosis, carried across
            // as this node's typed refusal.
            RoleSeg::InPart { .. } => {}
            RoleSeg::TrimEdge {
                edge: a,
                support: b,
            }
            | RoleSeg::FootVertex {
                vertex: a,
                support: b,
            }
            | RoleSeg::EndArc {
                vertex: a,
                edge: b,
            } => {
                visit(a, partners, f);
                visit(b, partners, f);
            }
            RoleSeg::BandFace(names) => {
                for n in names {
                    visit(n, partners, f);
                }
            }
            RoleSeg::Seam { a, b } => {
                visit(a, partners, f);
                visit(b, partners, f);
            }
            RoleSeg::Merged(names) => {
                for n in names {
                    visit(n, partners, f);
                }
            }
            // Discrimination references: verdicts against partners,
            // not derivation.
            RoleSeg::Fragment(q) => match q {
                Qualifier::SideOf(vec) => {
                    if partners == Partners::Include {
                        for (n, _) in vec {
                            visit(n, partners, f);
                        }
                    }
                }
                Qualifier::OrderAlong { .. } => {}
            },
            name_free_seg!() => {}
        }
    }
}

/// [`walk_names`] with partners included — the localization/cascade
/// walk (N7: a flip at a partner node re-qualifies the name).
fn for_each_inner<'a>(name: &'a StableName, f: &mut impl FnMut(&'a StableName)) {
    walk_names(name, Partners::Include, f);
}

/// The first structural-parameter change, restricted to `path` when
/// given: a path node whose Count-typed slot expression changed
/// bitwise, or whose Count slot references a Count doc-param that
/// changed.
fn structural_param_change(
    old: &Doc<ProfileProgram>,
    new: &Doc<ProfileProgram>,
    ddiff: &crate::diff::DocDiff,
    path: Option<&BTreeSet<RecipeNodeId>>,
) -> Option<(RecipeNodeId, SlotId)> {
    let changed_params: Vec<&crate::doc::ParamName> = ddiff.params.iter().collect();
    let candidates: Vec<RecipeNodeId> = match path {
        Some(p) => p.iter().copied().collect(),
        None => new.order().to_vec(),
    };
    for id in candidates {
        let (Some(a), Some(b)) = (old.node(id), new.node(id)) else {
            continue;
        };
        for slot in b.slots() {
            if !slot.is_structural() {
                continue;
            }
            let (ea, eb) = (a.expr(slot), b.expr(slot));
            let expr_changed = match (ea, eb) {
                (Some(x), Some(y)) => !x.bit_eq(y),
                (None, None) => false,
                _ => true,
            };
            if expr_changed {
                return Some((id, slot));
            }
            // A changed Count doc-param the slot references.
            if let Some(expr) = eb {
                let mut refs = Vec::new();
                expr.param_refs(&mut refs);
                if refs.iter().any(|(name, _)| changed_params.contains(&name)) {
                    return Some((id, slot));
                }
            }
        }
    }
    None
}

/// The first recipe edit touching the (optionally restricted) node
/// set, as a [`RecipeEditRef`]. A `Changed` node whose delta is
/// confined to CONTINUOUS slot expressions is NOT a recipe edit —
/// that is parameter motion, N7's site (i)/(ii) vocabulary (its
/// effects surface as verdict flips or structural-parameter
/// diagnoses), never site (iii).
fn recipe_edit_change(
    old: &Doc<ProfileProgram>,
    new: &Doc<ProfileProgram>,
    ddiff: &crate::diff::DocDiff,
    path: Option<&BTreeSet<RecipeNodeId>>,
) -> Option<RecipeEditRef> {
    for change in &ddiff.nodes {
        let (node, edit) = match *change {
            NodeChange::Added(n) => (n, RecipeEditRef::NodeInserted { node: n }),
            NodeChange::Removed(n) => (n, RecipeEditRef::NodeDeleted { node: n }),
            NodeChange::Changed(n) => {
                if let (Some(a), Some(b)) = (old.node(n), new.node(n))
                    && continuous_only_change(a, b)
                {
                    continue;
                }
                (n, RecipeEditRef::NodeChanged { node: n })
            }
        };
        if path.is_none_or(|p| p.contains(&node)) {
            return Some(edit);
        }
    }
    None
}

/// Whether two payloads differ ONLY in continuous slot expressions
/// (checked by copying the new continuous exprs over the old payload
/// and comparing bitwise — total, no per-variant knowledge).
fn continuous_only_change(
    old: &crate::node::Node<ProfileProgram>,
    new: &crate::node::Node<ProfileProgram>,
) -> bool {
    let mut patched = old.clone();
    for slot in new.slots() {
        if slot.is_structural() {
            continue;
        }
        let (Some(dst), Some(src)) = (patched.expr_mut(slot), new.expr(slot)) else {
            return false; // slot sets disagree: structural change
        };
        *dst = src.clone();
    }
    patched.bit_eq(new)
}
