//! Name resolution and diagnosis (M4 PR 4; NAMING-DESIGN N5 verbatim,
//! ratified #74; spec `docs/M4-PR4-SPEC.md` D1–D4).
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
//! # No-prior diagnosis (reported)
//!
//! `Vanished`'s diagnosis diffs the last-good run against the current
//! one. Without any prior run the honest evidence is current-run only:
//! `Cascade` when an embedded operand name itself fails to resolve;
//! otherwise the discrepancy is between the recorded reference and the
//! recipe as it stands, reported as
//! `RecipeEdit { NodeChanged(minting node) }` — the recipe's current
//! definition does not derive the name, and no verdict evidence exists
//! to blame a flip.

mod hit;
mod vdiff;

pub use hit::{HitTestError, body_name, edge_name, entity_name, face_name, vertex_name};
pub use vdiff::{FlipSet, NodeVerdictDelta, RunStatus, VerdictFlip, diff_verdicts};

use std::collections::BTreeSet;

use geom_core::{Decide, Sign};

use crate::diff::NodeChange;
use crate::doc::Doc;
use crate::eval::{Evaluation, NodeResult};
use crate::names::{EntityKey, EntityKind, EntityRef, Entry, Qualifier, RoleSeg, StableName};
use crate::node::{RecipeNodeId, SlotId};
use crate::profile_desc::ProfileDesc;
use crate::witness::WitnessBifurcation;

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
    pub doc: &'a Doc<ProfileDesc>,
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
    fn diagnose<T: Decide>(
        &self,
        new: RunCtx<'_, T>,
        _name: &StableName,
        path: &BTreeSet<RecipeNodeId>,
    ) -> Option<Diagnosis> {
        let flips = diff_verdicts(self.eval, new.eval);
        if let Some((_, f)) = flips.flips_on_nodes(path).first() {
            return Some(Diagnosis::PredicateFlip {
                predicate: f.predicate,
                from: f.from,
                to: f.to,
            });
        }
        let ddiff = self.doc.diff(new.doc);
        if let Some((node, param)) = structural_param_change(self.doc, new.doc, &ddiff, Some(path))
        {
            return Some(Diagnosis::StructuralParam { node, param });
        }
        if let Some(edit) = recipe_edit_change(&ddiff, Some(path)) {
            return Some(Diagnosis::RecipeEdit { edit });
        }
        // Global fallbacks (off-path evidence, in the same order).
        if let Some(&(_, f)) = flips.report().first() {
            return Some(Diagnosis::PredicateFlip {
                predicate: f.predicate,
                from: f.from,
                to: f.to,
            });
        }
        if let Some((node, param)) = structural_param_change(self.doc, new.doc, &ddiff, None) {
            return Some(Diagnosis::StructuralParam { node, param });
        }
        recipe_edit_change(&ddiff, None).map(|edit| Diagnosis::RecipeEdit { edit })
    }

    fn tombstone<T: Decide>(&self, _new: RunCtx<'_, T>, name: &StableName) -> Option<Tombstone> {
        let (node, entity) = lookup_unique(self.eval, name)?;
        let table = &self.eval.value(node)?.name_table;
        let body = table
            .name_of(&EntityRef {
                body: entity.body,
                key: EntityKey::Body,
            })?
            .clone();
        Some(Tombstone {
            kind: name.kind,
            body,
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
            .unwrap_or(Diagnosis::RecipeEdit {
                // No prior run and no cascade evidence: the recorded
                // reference disagrees with the recipe as it stands
                // (module docs; also the total fallback for the
                // N4-invariant-impossible "nothing differs" state).
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
/// suggestion ladder, general form): every name in the evaluation
/// whose derivation WRAPS `name` (embeds it as an operand name at any
/// depth — `FromA(x)`, `Instance{of: x}`, seams, fragments of it),
/// deterministically ordered (first carrying node, then name order).
/// These are SUGGESTIONS for an explicit `Rebind` — nothing follows
/// automatically (the ratified EMPTY policy menu).
pub fn rebind_suggestions<T: Decide>(eval: &Evaluation<T>, name: &StableName) -> Vec<StableName> {
    let mut out: Vec<StableName> = Vec::new();
    for &id in &eval.order {
        if let Some(v) = eval.value(id) {
            for (candidate, _) in v.name_table.iter() {
                if candidate == name || out.contains(candidate) {
                    continue;
                }
                let mut wraps = false;
                for_each_inner(candidate, &mut |inner| {
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

/// Visits every name embedded in `name`'s role path, recursively,
/// in path order (operand names, seam pairs, merged constituents,
/// discriminator partners, pattern masters).
fn for_each_inner<'a>(name: &'a StableName, f: &mut impl FnMut(&'a StableName)) {
    fn visit<'a>(n: &'a StableName, f: &mut impl FnMut(&'a StableName)) {
        f(n);
        for_each_inner(n, f);
    }
    for seg in &name.path {
        match seg {
            RoleSeg::FromA(n)
            | RoleSeg::FromB(n)
            | RoleSeg::SectionEdge { face: n, .. }
            | RoleSeg::SplitFragment { parent: n, .. }
            | RoleSeg::CrossingVertex { edge: n, .. }
            | RoleSeg::OnToolVertex { of: n, .. }
            | RoleSeg::Instance { of: n, .. } => visit(n, f),
            RoleSeg::Seam { a, b } => {
                visit(a, f);
                visit(b, f);
            }
            RoleSeg::Merged(names) => {
                for n in names {
                    visit(n, f);
                }
            }
            RoleSeg::Fragment(Qualifier::SideOf(vec)) => {
                for (n, _) in vec {
                    visit(n, f);
                }
            }
            _ => {}
        }
    }
}

/// The first structural-parameter change, restricted to `path` when
/// given: a path node whose Count-typed slot expression changed
/// bitwise, or whose Count slot references a Count doc-param that
/// changed.
fn structural_param_change(
    old: &Doc<ProfileDesc>,
    new: &Doc<ProfileDesc>,
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
/// set, as a [`RecipeEditRef`].
fn recipe_edit_change(
    ddiff: &crate::diff::DocDiff,
    path: Option<&BTreeSet<RecipeNodeId>>,
) -> Option<RecipeEditRef> {
    for change in &ddiff.nodes {
        let (node, edit) = match *change {
            NodeChange::Added(n) => (n, RecipeEditRef::NodeInserted { node: n }),
            NodeChange::Removed(n) => (n, RecipeEditRef::NodeDeleted { node: n }),
            NodeChange::Changed(n) => (n, RecipeEditRef::NodeChanged { node: n }),
        };
        if path.is_none_or(|p| p.contains(&node)) {
            return Some(edit);
        }
    }
    None
}
