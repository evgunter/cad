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
//! one. When those lanes are silent — no prior run, the diff
//! engine's population-cancel blind spot (`vdiff` module docs), or
//! **sweep pruning** (ratified 2026-07-29, N5 as amended): the
//! realized boolean sweep records no verdicts for pairs its candidate
//! generation pruned, so a vanish whose flip evidence lived on a
//! now-pruned pair (an interaction-boundary edit — overlapping ↔
//! disjoint) has no recorded flip to cite — two
//! honest rungs remain, in order: `Cascade` when an embedded operand
//! name itself fails to resolve, and the QUALIFIER-DELTA rung
//! ([`qualifier_delta`]): the N2 discriminator verdicts recorded in
//! the names themselves yield a `PredicateFlip` derived from recorded
//! data when a same-shape sibling differs by exactly one pure-sign
//! `SideOf` entry. If that too finds nothing, the total fallback is
//! `RecipeEdit { NodeChanged(minting node) }`, documented as "the
//! recorded reference disagrees with the recipe as it stands; the
//! cause is not in evidence" — a site, not a claim that an edit
//! happened.

mod hit;
mod pick;
mod vdiff;

pub use hit::{HitTestError, body_name, edge_name, entity_name, face_name, vertex_name};
pub use pick::{MeshPick, MeshPickError, NodePick, NodePickError, PickHit, PickTarget, pick_face};
pub use vdiff::{
    FlipSet, NodeVerdictDelta, NodeVerdicts, PredicateDivergence, RunStatus, SummaryDelta,
    SummaryDivergence, SummaryFlip, SummaryFlipSet, VerdictFlip, VerdictSummary, diff_summaries,
    diff_verdicts, verdict_summary,
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
// the PROBLEM in prose — the name by its kind and minting node (a
// stable name is a derivation path, and the node is the half a user
// can act on), the WHY forwarded from the payload's own rendering.
// Composing layers (`NodeErrorKind`'s two resolve arms) FORWARD this
// rather than re-stating it.
impl core::fmt::Display for ResolveError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Vanished {
                name, diagnosis, ..
            } => write!(
                f,
                "the {} name minted by node {} no longer resolves in this evaluation: \
                 {diagnosis}",
                name.kind.noun(),
                name.node.0
            ),
            Self::Ambiguous { name, tie, .. } => write!(
                f,
                "the {} name minted by node {} is tie-marked: {} equally-admissible \
                 candidates at its recorded site — a tie is never broken by picking; \
                 refine the reference until one candidate remains",
                name.kind.noun(),
                name.node.0,
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

/// Why a name vanished — N5 verbatim plus the reserved
/// `WitnessBifurcation` arm (SOLVER-DESIGN W3; constructed by the M6
/// solver).
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
            } => write!(
                f,
                "predicate {predicate} flipped from {from:?} to {to:?} on the name's \
                 derivation path"
            ),
            Self::StructuralParam { node, param } => write!(
                f,
                "a structural parameter changed on the derivation path (node {}, slot \
                 {param:?})",
                node.0
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
                "the upstream {} name minted by node {} vanished first; its own \
                 resolution failure carries the root cause",
                through.kind.noun(),
                through.node.0
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
) -> Resolution {
    resolve_impl(new, prior, name)
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
) -> Resolution {
    enrich_impl(new, prior, loss)
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
            error: ResolveError::Ambiguous {
                name: loss.name.clone(),
                candidates: vec![loss.name.clone()],
                tie: TieWitness {
                    node: *at,
                    at: loss.name.clone(),
                    width: ents.len() as u32,
                },
            },
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
    fn removal_edit(&self, node: RecipeNodeId) -> Option<RecipeEditRef>;
}

struct NoPrior;

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

    fn removal_edit(&self, _node: RecipeNodeId) -> Option<RecipeEditRef> {
        None
    }
}

impl<U: Decide> PriorCtx for RunCtx<'_, U> {
    /// The with-history diagnosis ladder (deterministic; first honest
    /// evidence wins): path-restricted verdict flips, then structural
    /// parameters on the path, then recipe edits on the path, then
    /// the same three globally (geometry-mediated effects still land
    /// their flips at the deciding node, so the global lanes are the
    /// honesty fallback, not the common case).
    ///
    /// Attribution among several path flips: the `name_frag_*`
    /// discriminator family wins when present — those predicates are
    /// the name's OWN qualifier vocabulary (a discriminator flip is
    /// definitionally the flip that re-qualified the fragment);
    /// otherwise the first flip in deterministic order. This is a
    /// consumer-side attribution choice — the diff engine itself
    /// stays cause-agnostic and unspecialized.
    fn diagnose<T: Decide>(
        &self,
        new: RunCtx<'_, T>,
        _name: &StableName,
        path: &BTreeSet<RecipeNodeId>,
    ) -> Option<Diagnosis> {
        let pick = |flips: &[(RecipeNodeId, VerdictFlip)]| {
            flips
                .iter()
                .find(|(_, f)| f.predicate.starts_with("name_frag_"))
                .or_else(|| flips.first())
                .map(|(_, f)| Diagnosis::PredicateFlip {
                    predicate: f.predicate,
                    from: f.from,
                    to: f.to,
                })
        };
        let flips = diff_verdicts(self.eval, new.eval);
        if let Some(d) = pick(&flips.flips_on_nodes(path)) {
            return Some(d);
        }
        let ddiff = self.doc.diff(new.doc);
        if let Some((node, param)) = structural_param_change(self.doc, new.doc, &ddiff, Some(path))
        {
            return Some(Diagnosis::StructuralParam { node, param });
        }
        if let Some(edit) = recipe_edit_change(self.doc, new.doc, &ddiff, Some(path)) {
            return Some(Diagnosis::RecipeEdit { edit });
        }
        // Global fallbacks (off-path evidence, in the same order).
        if let Some(d) = pick(&flips.report()) {
            return Some(d);
        }
        if let Some((node, param)) = structural_param_change(self.doc, new.doc, &ddiff, None) {
            return Some(Diagnosis::StructuralParam { node, param });
        }
        recipe_edit_change(self.doc, new.doc, &ddiff, None)
            .map(|edit| Diagnosis::RecipeEdit { edit })
    }

    fn tombstone<T: Decide>(&self, _new: RunCtx<'_, T>, name: &StableName) -> Option<Tombstone> {
        let (node, entity) = lookup_unique(self.eval, name)?;
        let table = &self.eval.value(node)?.name_table;
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

    fn removal_edit(&self, node: RecipeNodeId) -> Option<RecipeEditRef> {
        self.doc
            .node(node)
            .is_some()
            .then_some(RecipeEditRef::NodeDeleted { node })
    }
}

fn resolve_impl<T: Decide, P: PriorCtx>(
    new: RunCtx<'_, T>,
    prior: P,
    name: &StableName,
) -> Resolution {
    // 1. NodeGone: the minting node is not live. Ids are never
    //    reused, so an id below the mint counter was deleted; an id
    //    at/above it was never this document's (ForeignNode).
    if new.doc.node(name.node).is_none() {
        let edit = if name.node.0 < new.doc.next_id {
            prior
                .removal_edit(name.node)
                .unwrap_or(RecipeEditRef::NodeDeleted { node: name.node })
        } else {
            RecipeEditRef::ForeignNode { node: name.node }
        };
        return Resolution::Failed(ResolutionFailure {
            error: ResolveError::NodeGone {
                name: name.clone(),
                edit,
            },
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
                error: ResolveError::Ambiguous {
                    name: name.clone(),
                    candidates: vec![name.clone()],
                    tie: TieWitness {
                        node,
                        at: name.clone(),
                        width: ents.len() as u32,
                    },
                },
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
                    error: ResolveError::Ambiguous {
                        name: name.clone(),
                        candidates: vec![base.clone()],
                        tie: TieWitness {
                            node,
                            at: base,
                            width: ents.len() as u32,
                        },
                    },
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
            .unwrap_or(Diagnosis::RecipeEdit {
                // Total fallback, honest about its limits: the
                // recorded reference disagrees with the recipe as it
                // stands and the CAUSE IS NOT IN EVIDENCE (no verdict
                // flip, no doc delta, no recorded qualifier delta —
                // reachable through the population-cancel blind spot,
                // `vdiff` module docs, and through SWEEP PRUNING: the
                // realized sweep records no verdicts for pruned
                // pairs, so interaction-boundary vanishes can land
                // here — ratified 2026-07-29, NAMING-DESIGN N5 as
                // amended; the shadow re-execution recovery rung is
                // banked there). `NodeChanged` names the minting node
                // as the site of the disagreement, not a claim that
                // an edit happened.
                edit: RecipeEditRef::NodeChanged { node: name.node },
            })
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
    for &id in &eval.order {
        let Some(v) = eval.value(id) else { continue };
        for (candidate, _) in v.name_table.iter() {
            if let Some((from, to)) = single_pure_sideof_delta(name, candidate) {
                return Some(Diagnosis::PredicateFlip {
                    predicate: "name_frag_side_of",
                    from,
                    to,
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

/// The first (evaluation-order) Ok table carrying `name`.
fn lookup<'a, T: Decide>(
    eval: &'a Evaluation<T>,
    name: &StableName,
) -> Option<(RecipeNodeId, &'a Entry)> {
    for &id in &eval.order {
        if let Some(v) = eval.value(id)
            && let Some(entry) = v.name_table.lookup(name)
        {
            return Some((id, entry));
        }
    }
    None
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
fn merge_offers<T: Decide>(eval: &Evaluation<T>, name: &StableName) -> Vec<StableName> {
    let mut offers = Vec::new();
    // Unmerge: the name IS a merged name — offer its constituents.
    for seg in &name.path {
        if let RoleSeg::Merged(constituents) = seg {
            offers.extend(constituents.iter().cloned());
        }
    }
    // Merge: a live Merged row lists `name` as a constituent.
    for &id in &eval.order {
        if let Some(v) = eval.value(id) {
            for (candidate, _) in v.name_table.iter() {
                let mut contains = false;
                for seg in &candidate.path {
                    if let RoleSeg::Merged(constituents) = seg
                        && constituents.contains(name)
                    {
                        contains = true;
                    }
                }
                if contains && !offers.contains(candidate) {
                    offers.push(candidate.clone());
                }
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
    for &id in &eval.order {
        if let Some(v) = eval.value(id) {
            for (candidate, _) in v.name_table.iter() {
                if candidate == name || candidate.kind != name.kind || out.contains(candidate) {
                    continue;
                }
                let mut wraps = false;
                walk_names(candidate, Partners::Skip, &mut |inner| {
                    if inner == name {
                        wraps = true;
                    }
                });
                if wraps {
                    out.push(candidate.clone());
                }
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
/// Checked sites: the name-carrying payload of an `InsertNode`
/// ([`crate::node::Node::payload_names`] — Declare pairs, a fillet's
/// selection, a mate's two heads) and `Rebind`'s target. Every other
/// edit validates exactly as [`crate::edit::apply`] — including the
/// four appearance edits, which DO carry a name: theirs resolves at
/// evaluation, into a typed [`crate::appearance::AppearanceLoss`].
/// `Rebind`'s SOURCE is deliberately unchecked too: it is the
/// stranded name being repaired.
///
/// # Errors
///
/// [`crate::edit::EditError::NameUnresolvedInEvaluation`] on a
/// checkable-but-absent name; otherwise whatever [`crate::edit::apply`]
/// returns.
pub fn apply_with_names<T: Decide>(
    doc: &Doc<ProfileProgram>,
    edit: &crate::edit::DocEdit<ProfileProgram>,
    eval: &Evaluation<T>,
    tol: Tol,
) -> Result<crate::edit::Applied<ProfileProgram>, crate::edit::EditError> {
    use crate::edit::{DocEdit, EditError};
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
        | DocEdit::SetParam { .. }
        | DocEdit::SetStructuralParam { .. }
        | DocEdit::SetExpression { .. }
        | DocEdit::SetDocParam { .. }
        | DocEdit::SetDocParamValue { .. }
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
    crate::edit::apply(doc, edit, tol)
}

/// The nodes a name's derivation passes through: its minting node,
/// every embedded operand name's nodes (recursively), and every
/// discriminator partner's nodes — the localization set of N7 ("an
/// edit renames nothing outside derivation paths that actually pass
/// through the edited node").
pub fn derivation_nodes(name: &StableName) -> BTreeSet<RecipeNodeId> {
    let mut nodes = BTreeSet::from([name.node]);
    for_each_inner(name, &mut |inner| {
        nodes.insert(inner.node);
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
/// that answer is written for this and its two sibling matches.
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
            | RoleSeg::BandSlit(n) => visit(n, partners, f),
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
            | RoleSeg::CornerArc {
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
