//! **Split and inline** — the first-class recorded refactorings
//! (ASM-4 D-2/D-3; ASSEMBLY-DESIGN A4, A10, A11).
//!
//! [`split`] cuts a closed set of nodes out of a document into a new
//! part document and leaves an [`Node::InstantiatePart`] of it behind;
//! [`inline`] is the inverse — it splices a referenced document's
//! recipe into the host and deletes the instance. Both are PURE
//! functions returning new document values, the ordinary recorded
//! [`DocEdit`]s that produce them, and the [`crate::Maintenance`]
//! those edits reported, net of what a later edit in the same
//! refactoring took back ([`crate::MaintenanceNet`]): the payload
//! names a departing cut node stranded. The input documents are
//! untouched, so undo is this layer's undo everywhere else: keeping the
//! prior value. There is no compound edit arm; atomicity is purity (no
//! partially-refactored document is ever observable).
//!
//! # The cut rule (D-2)
//!
//! The cut set must be closed under what its nodes read: a cut node
//! reading the remainder is a severed edge, refused typed naming it
//! (A4's "ancestor-closed"). A cut takes whole placements: a
//! placement's constraints are its own, so a cut placement whose bundle
//! reads a kept copy severs that read. A part delivers only its world
//! (A10), so a remainder read of the cut crosses only where the
//! instance's copies carry what it reads: a selection each of whose
//! names one cut world placement's copy holds, re-authored on the
//! instance's copies; a read of one copy of a cut world placement, or of
//! a cut `Place`'s one copy, or of a body one cut world `Place` copies
//! at the identity, each re-pointed to that copy of the instance by its
//! key in the part ([`crate::CopyKey`]). Any other remainder read of
//! the cut is severed too.
//!
//! # Placements cross the seam (A4)
//!
//! The remainder receives ONE `InstantiatePart` for the whole cut,
//! pinned to the world at the identity: each instance materializes the
//! ENTIRE new document's product, so per-placement instances of one
//! pinned document would duplicate every other copy's material N times.
//! The cut's world placements are the part's world, so the instance's
//! copies stand where the cut's did, and every pose is the one the cut
//! solved to. A kept constraint reading a cut copy reads it in the
//! instance (FORK-S3M: a constraint reads its target across the cut):
//! its pose is written afresh over the instance's copies, each selection
//! in it re-authored as a crossing selection is. The instance stands
//! where its part's world stands ([`crate::mate::stand`]). Neither
//! door computes a frame.
//!
//! [`inline`] is the inverse. An instance whose mates all target the
//! world splices its part's placements with every mate on the part's
//! world moved by the motion it solved to (at the identity, verbatim).
//! An instance placed by other constraints is inlined only over a part
//! whose one placement on its world is an instance pinned there at the
//! identity, which takes the instance's bundle in place of its pin
//! ([`InlineError::UnplaceablePart`]). A host mate reading the
//! instance's copies is written afresh over the spliced copy its names
//! land on.
//!
//! # Variables go where their readers go
//!
//! A variable's side is the union of its readers' — a node slot, or
//! the definition of another variable — and one with no reader follows
//! what it reads. Split moves a variable whose side is the cut's: the
//! part declares it, under its name if it has one, and the remainder
//! records its `DeleteVar`. One with readers or reads on both sides
//! refuses
//! ([`SplitError::UncutVarReference`],
//! [`SplitError::DefinitionStraddlesCut`]); any other stays, an unread
//! free variable among them. Inline carries every variable of the part
//! under an id the host mints, and refuses a name the host already
//! holds ([`InlineError::VarNameConflict`]): two variables are never
//! one because their values agree (VR1).
//!
//! So `inline(split(d))` is `d` up to minted ids — node, step,
//! variable and constraint — and order, on every cut split admits
//! (A4): split appends its instance to the remainder and inline appends
//! what it splices, so a cut whose placements a kept one separates
//! comes back after it. `split(inline(h))` is not promised: an unread
//! free named variable the part held has no reader to follow back, and
//! stays in the host.
//!
//! # Labels follow their nodes
//!
//! A node label (DESIGN.md Band 1, "Node labels") is keyed by the node,
//! so it moves with the node: split labels each cut node's copy in the
//! part as the original was, inline labels each spliced node as its
//! part-side original was, and the instance a split leaves behind is a
//! new node with no label.
//!
//! # Names re-anchor across the seam (the bridge, both directions)
//!
//! Split rewrites every remainder-side reference to a cut entity —
//! fillet selections, frame faces, appearance keys — from its local
//! name to the `InPart`-wrapped name at the new instance, around the
//! name the part's product spells it by: a cut world placement's own
//! name as it is, and a body's name under the cut placement that copies
//! it (`Placed`; a recorded [`DocEdit::Rebind`] per name), which is
//! exactly how "every stable name that resolved before resolves after,
//! through the instance qualifier" (D-4) is kept. Inline applies the
//! inverse rewrite: `InPart`-wrapped names at the inlined instance
//! re-anchor to the spliced local names — the body's own where its
//! copy stands where the body did, the carried copy's otherwise.
//! Appearance records ride these rewrites — they stay with the document
//! that referenced the entity, so the split remainder keeps its
//! presentation and the round trip restores it. A reference that
//! DERIVES from both sides of the cut can re-anchor to neither and
//! refuses typed, as does a top-level BODY name crossing the cut (a
//! product's name table deliberately carries no body rows, so the
//! wrapped body name could never resolve), and a name of material the
//! part's world does not hold once.
//!
//! # Interface records
//!
//! A kept constraint whose two face reads land on opposite sides of the
//! cut is one [`crate::InterfaceCrossing::Mate`] entry in the remainder
//! instance's [`crate::InterfaceRecord`] (ASM-R2b D-4; AQ8): a kept
//! placement pinned against the part. A constraint whose side resolves
//! to no member — a dangling or nested-pattern head — contributes
//! nothing however its names fall: the gate is what skips it.
//!
//! # Determinism (D6/D9)
//!
//! Both refactorings are pure functions of their inputs (the part
//! document's identity is CALLER-supplied, never ambient), every
//! iteration below runs over ordered structures, and part-node ids
//! remap in document order — so two split runs, in any two processes,
//! produce byte-identical documents.
use std::collections::{BTreeMap, BTreeSet, btree_map};

use crate::doc::{Doc, FreeVar, NameCarrier, VarName};
use crate::edit::Maintenance;
use crate::edit::{DocEdit, EditError, Recorded, Recording};
use crate::expr::Dimension;
use crate::formula::Formula;
use crate::ident::{DocRef, DocumentId};
use crate::names::{
    Carry, EntityKind, FaceName, NameRef, ProfileEdgeRef, ProfileVertexRef, RoleSeg, SegRewrite,
    StableName, Unrankable,
};
use crate::node::{
    AuthoredNode, InterfaceCrossing, InterfaceRecord, Node, PatternKind, RecipeNodeId, StepId,
};
use crate::part::{PartResolver, ResolveFailure};
use crate::persist::{PersistError, content_pin};
use crate::program::{ProfileDoc, ProfileProgram};
use crate::resolve::derivation_nodes;
use crate::sentence::{Recourse, Staged};
use crate::spoken::{SpokenName, SpokenNode, SpokenVar};
use crate::var::{VarDecl, VarDef, VarId};
use geom_core::Tol;

/// The old-id → new-id correspondence a refactoring establishes
/// between the two documents' node id spaces.
pub type NodeMap = BTreeMap<RecipeNodeId, RecipeNodeId>;

/// The old-id → new-id correspondence a refactoring establishes
/// between the two documents' profile STEP id spaces: every step of
/// every profile it carries across, re-minted by the other document's
/// insert door (`names/README.md`, "N1, the profile pieces").
pub type StepMap = BTreeMap<StepId, StepId>;

/// **The id a name's rewrite could not map** — a node the other
/// document has no copy of, or a profile step no carried profile holds
/// (a step a `SetProgram` dropped, which a stranded name still spells).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Unmapped {
    /// A node id.
    Node(RecipeNodeId),
    /// A profile step id.
    Step(StepId),
    /// A read ([`crate::RoleSeg::From`]) whose operation the maps do not
    /// carry.
    Read(VarId),
}

impl Unmapped {
    /// The id the maps lack, a read spoken as the operation `source`
    /// defines it by where it defines one.
    fn in_source(self, source: &ProfileDoc) -> Self {
        match self {
            Self::Read(var) => source.operation_of(var).map_or(self, Self::Node),
            other => other,
        }
    }
}

/// **Why [`remap_name`] would not carry a name across a caller's maps.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RemapError {
    /// An id the maps lack.
    Unmapped(Unmapped),
    /// A vertex ranked along the least of the edges that cross there
    /// (N2's `EdgeCrossing` run) whose least edge the maps make another
    /// one: the rank was measured along the old least edge, and nothing
    /// says where the vertex lies along the new one. `vertex` is the
    /// ranked name as it was, which may be one the name embeds.
    RankedRunReordered {
        /// The ranked vertex's name, in the maps' source ids.
        vertex: Box<StableName>,
    },
}

impl From<Unmapped> for RemapError {
    fn from(missing: Unmapped) -> Self {
        Self::Unmapped(missing)
    }
}

impl core::fmt::Display for RemapError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        const LACKS: &str = "the name cannot be carried across these maps: they hold no image of";
        match self {
            Self::Unmapped(Unmapped::Node(node)) => write!(
                f,
                "{LACKS} node {node}. Recourse: carry that node too, or drop the name"
            ),
            Self::Unmapped(Unmapped::Step(step)) => write!(
                f,
                "{LACKS} profile step {step}. Recourse: carry the profile that holds it, or \
                 drop the name"
            ),
            Self::Unmapped(Unmapped::Read(read)) => write!(
                f,
                "{LACKS} {}. Recourse: carry the operation that defines it, or drop the name",
                crate::spoken::ReadTag(*read)
            ),
            Self::RankedRunReordered { vertex } => write!(
                f,
                "{} is ranked along an edge these maps no longer put first. Recourse: keep id \
                 order in the maps, or pick it again",
                SpokenName::absent((**vertex).clone())
            ),
        }
    }
}

impl std::error::Error for RemapError {}

/// Each carried operation's outputs, onto the ones its insert minted at
/// the same ports: what a carried read reads.
pub type ReadMap = BTreeMap<VarId, VarId>;

impl SplitError {
    /// A cut node's reference the part-side rewrite could not map,
    /// spoken from `source`, the document being split.
    fn reaches(
        source: &ProfileDoc,
        node: RecipeNodeId,
        name: &StableName,
        missing: RemapError,
    ) -> Self {
        let name = source.spoken_name(name);
        let missing = match missing {
            RemapError::Unmapped(missing) => missing,
            RemapError::RankedRunReordered { .. } => {
                return Self::RankedRunReordered { name };
            }
        };
        match missing.in_source(source) {
            Unmapped::Node(missing) => Self::PartNameReachesRemainder {
                node: source.spoken(node),
                name,
                missing: source.spoken(missing),
            },
            Unmapped::Step(step) => Self::NameOnDroppedStep { name, step },
            Unmapped::Read(_) => Self::NameStraddlesCut {
                name,
                missing: None,
            },
        }
    }

    /// A remainder-side name the part-side rewrite could not map,
    /// spoken from `source`, the document being split.
    fn straddles(source: &ProfileDoc, name: &StableName, missing: RemapError) -> Self {
        let name = source.spoken_name(name);
        let missing = match missing {
            RemapError::Unmapped(missing) => missing,
            RemapError::RankedRunReordered { .. } => {
                return Self::RankedRunReordered { name };
            }
        };
        match missing.in_source(source) {
            Unmapped::Node(missing) => Self::NameStraddlesCut {
                name,
                missing: Some(source.spoken(missing)),
            },
            Unmapped::Step(step) => Self::NameOnDroppedStep { name, step },
            Unmapped::Read(_) => Self::NameStraddlesCut {
                name,
                missing: None,
            },
        }
    }
}

impl InlineError {
    /// A name to be spliced the host-side rewrite could not map, spoken
    /// from `part`, the referenced document whose ids it is spelled in.
    fn stranded(part: &ProfileDoc, name: &StableName, missing: RemapError) -> Self {
        let name = part.spoken_name(name);
        let missing = match missing {
            RemapError::Unmapped(missing) => missing,
            RemapError::RankedRunReordered { .. } => {
                return Self::RankedRunReordered { name };
            }
        };
        match missing.in_source(part) {
            Unmapped::Node(missing) => Self::StrandedPartName {
                name,
                missing: part.spoken(missing),
            },
            Unmapped::Step(step) => Self::NameOnDroppedStep { name, step },
            Unmapped::Read(read) => Self::StrandedPartRead {
                name,
                read: part.spoken_var(read),
            },
        }
    }
}

/// **The ids `minted` dealt out in `old`'s id order**, shaped as `old`:
/// the step whose id is the k-th least of `old` takes the k-th least of
/// `minted`, over every loop of the one profile a node holds. The
/// insert door mints a profile's ids in program order and a
/// `SetProgram` may hold them in another (`edit::settle_step_ids`); a
/// name orders the lines one node mints by their steps, so a step map
/// that did not keep id order would reorder a run that a rank is
/// measured along.
///
/// Across two profiles the order cannot always be kept: the door mints
/// each profile's ids when the profile is inserted, in document order,
/// and a `SetProgram` that keeps a step keeps it within its own profile
/// (`StepIdFault::NotThisProfiles`). So where a `SetProgram` re-minted
/// a step of an earlier profile after a later one was inserted, the
/// earlier profile's step maps below the later profile's. No emitter
/// compares two profiles' steps in one position of a name (a loft's
/// sections differ at the section first); a name that did, holding a
/// ranked run, refuses the pair typed (`RankedRunReordered`) rather
/// than cross unresolvable.
fn in_id_order(old: &[Vec<StepId>], minted: &[Vec<StepId>]) -> Vec<Vec<StepId>> {
    let mut sorted: Vec<StepId> = minted.iter().flatten().copied().collect();
    sorted.sort_unstable();
    let rank: BTreeMap<StepId, usize> = old
        .iter()
        .flatten()
        .copied()
        .collect::<BTreeSet<_>>()
        .into_iter()
        .enumerate()
        .map(|(k, id)| (id, k))
        .collect();
    old.iter()
        .map(|lp| lp.iter().map(|id| sorted[rank[id]]).collect())
        .collect()
}

/// **The carried nodes, inserted one at a time**: each of `olds`, a
/// node of `source`, is remapped through the maps the inserts before it
/// built and inserted into `target`, and maps to the id the insert door
/// minted for it; a profile's steps map to the ids the door minted for
/// them, dealt out in the order of the ids they replace
/// ([`in_id_order`]): where that is not the door's program order, a
/// recorded `SetProgram` keeping every step moves them. The maps are
/// read off the door, never predicted, so they cannot disagree with
/// what it minted.
///
/// **What keeps a carried name in name order.** Node ids follow
/// document order, which is id order. A read
/// is an output, minted at its node's insert in port order
/// (`Mint::outputs_of_insert`), so reads follow node order. Each
/// profile's steps are dealt in id order ([`in_id_order`]); across
/// profiles they are not always, and a name that would cross reordered
/// refuses ([`remap_name`]'s `RankedRunReordered`, spoken by `miss`). A node's label follows it in the edit after its
/// insert, so a refusal raised while a later node is
/// carried speaks the carried ones with their labels.
///
/// **A placement is carried bare, and its bundle constraint by
/// constraint once every node is in** ([`DocEdit::AddConstraint`],
/// placements in document order, each bundle in its order): a
/// constraint reads the copy's side through the placement's own output
/// (`Doc::upstream`'s one exemption), and may read a copy inserted after
/// its placement, so it waits for both. A constraint changes no output,
/// so no node inserted meanwhile reads it. Each constraint, authored,
/// passes `settle` with its
/// placement's old id, which may rewrite it or drop it (`false`): the
/// one rewrite inline makes, a mate to the world moved by the
/// instance's pose, or the part's root taking the instance's bundle in
/// place of its own.
///
/// A name the maps lack whose missing id belongs to a node still to
/// come is a FORWARD reference (a blend selection or a frame's face rebound
/// onto a later node): no order of inserts satisfies it, and it refuses
/// as the insert door would, [`EditError::DeclareNamesMissingNode`],
/// spelled in `source`'s ids and so spoken from `source` (see
/// [`SplitError`] on why the id space, not membership, decides). Every other miss is `miss`'s.
///
/// # Errors
///
/// `edit` over the insert door's refusal; `miss` over a remap miss.
#[allow(clippy::too_many_arguments)]
fn carry<E>(
    source: &ProfileDoc,
    olds: &[RecipeNodeId],
    target: &mut Recording<'_, ProfileProgram>,
    vars: &mut VarCarry<'_>,
    fold: bool,
    settle: impl Fn(RecipeNodeId, &mut crate::mate::Mate<Formula>, Option<&Run>) -> bool,
    edit: impl Fn(EditError) -> E,
    miss: impl Fn(RecipeNodeId, RemapMiss) -> E,
) -> Result<(NodeMap, StepMap, ReadMap), E> {
    let mut node_map = NodeMap::new();
    // Each carried operation's outputs, onto the ones its insert
    // minted at the same ports: what a carried read reads.
    let mut out_map = ReadMap::new();
    let mut step_map = StepMap::new();
    let mut bundles: Vec<(RecipeNodeId, RecipeNodeId, Vec<crate::mate::Constraint>)> = Vec::new();
    for (k, &old) in olds.iter().enumerate() {
        let Some(node) = source.node(old) else {
            continue;
        };
        let later = &olds[k..];
        let forward = |missing: &RemapError| match *missing {
            RemapError::RankedRunReordered { .. } => false,
            RemapError::Unmapped(Unmapped::Node(n)) => later.contains(&n),
            RemapError::Unmapped(Unmapped::Read(var)) => {
                source.operation_of(var).is_some_and(|n| later.contains(&n))
            }
            RemapError::Unmapped(Unmapped::Step(step)) => later.iter().any(|n| {
                matches!(source.node(*n), Some(Node::Profile(p))
                    if p.ids.iter().flatten().any(|s| *s == step))
            }),
        };
        let refuse = |error: RemapMiss| match error {
            RemapMiss::Name { name, missing } if forward(&missing) => {
                edit(EditError::DeclareNamesMissingNode {
                    name: source.spoken_name(&name),
                })
            }
            other => miss(old, other),
        };
        // A placement's bundle is carried once every node is in (the
        // function's docs).
        let bundle: Vec<crate::mate::Constraint> = crate::mate::solve::bundle_of(node)
            .map(<[_]>::to_vec)
            .unwrap_or_default();
        let mut bare = node.clone();
        if let Node::Place { constraints, .. } | Node::InstantiatePart { constraints, .. } =
            &mut bare
        {
            constraints.clear();
        }
        let (carried, selects) =
            remap_with_selects(source, &bare, vars, (&node_map, &step_map, &out_map))
                .map_err(&refuse)?;
        let (authored, fresh) = vars.author(&carried, &selects);
        let record = target
            .apply_recorded(DocEdit::InsertNode {
                node: Box::new(authored),
                fresh: fresh.fresh.clone(),
            })
            .map_err(&edit)?;
        vars.minted(fresh, &record.fresh);
        let Some(new) = record.minted else {
            unreachable!("an accepted insert mints its node")
        };
        node_map.insert(old, new);
        // Each selection the insert minted stands for its source, and a
        // named one takes its name across.
        let minted_reads = target
            .doc()
            .node(new)
            .map(Node::operand_rows)
            .unwrap_or_default();
        name_selections(
            source,
            target,
            vars,
            bare.operand_rows().into_iter().map(|(_, var)| var),
            minted_reads.into_iter().map(|(_, var)| var),
        )
        .map_err(&edit)?;
        if let Some(label) = source.label(old) {
            target
                .apply(DocEdit::SetLabel {
                    node: new,
                    label: Some(label.clone()),
                })
                .map_err(&edit)?;
        }
        // A named output crosses with its node: the insert minted the
        // new one at the same port, and the name moves onto it.
        for (output, &minted) in source.outputs(old).into_iter().zip(&record.outputs) {
            out_map.insert(output, minted);
            if let Some(name) = source.var_name(output) {
                target
                    .apply(DocEdit::RenameVar {
                        var: minted.into(),
                        name: Some(name.clone()),
                    })
                    .map_err(&edit)?;
            }
        }
        if let (Node::Profile(from), Some(Node::Profile(to))) = (node, target.doc().node(new)) {
            let shape = |ids: &[Vec<StepId>]| ids.iter().map(Vec::len).collect::<Vec<_>>();
            if shape(&from.ids) != shape(&to.ids) {
                unreachable!(
                    "the insert door mints one id per authored step, so a carried profile's step \
                     ids have its source's shape: carried {:?}, source {:?}",
                    shape(&to.ids),
                    shape(&from.ids)
                );
            }
            let images = in_id_order(&from.ids, &to.ids);
            if images != to.ids {
                let loops = to.loops.iter().map(crate::LoopProgram::authored).collect();
                target
                    .apply(DocEdit::SetProgram {
                        node: new,
                        loops,
                        ids: images
                            .iter()
                            .map(|lp| lp.iter().copied().map(Some).collect())
                            .collect(),
                        fresh: Vec::new(),
                    })
                    .map_err(&edit)?;
            }
            step_map.extend(
                from.ids
                    .iter()
                    .flatten()
                    .copied()
                    .zip(images.into_iter().flatten()),
            );
        }
        if !bundle.is_empty() {
            bundles.push((old, new, bundle));
        }
    }
    // The bundles, now every node is in. A value names its mate by the
    // id the mate's add minted here; a value of a mate `settle` dropped
    // goes with it.
    for (old, new, bundle) in bundles {
        let mut ids: BTreeMap<crate::node::ConstraintId, crate::node::ConstraintId> =
            BTreeMap::new();
        for mut constraint in bundle {
            // A world read's run of written moves, folded off the
            // source mate before it is carried (`fold`): its numbers
            // are not carried, and `settle` moves the read by them.
            let run = fold
                .then(|| fold_world_run(source, &mut constraint))
                .flatten();
            let one = Node::Place {
                shapes: Vec::new(),
                constraints: vec![constraint.clone()],
            };
            let (carried, selects) =
                remap_with_selects(source, &one, vars, (&node_map, &step_map, &out_map))
                    .map_err(|error| miss(old, error))?;
            let (authored, fresh) = vars.author(&carried, &selects);
            let Node::Place {
                constraints: mut authored,
                ..
            } = authored
            else {
                unreachable!("a carried placement carrier stays one")
            };
            let Some(authored) = authored.pop() else {
                unreachable!("the carrier holds the one constraint")
            };
            let mut mate = match authored.kind {
                crate::mate::ConstraintKind::Mate(mate) => mate,
                crate::mate::ConstraintKind::Value(mut value) => {
                    let Some(&mate) = ids.get(&value.mate) else {
                        continue;
                    };
                    value.mate = mate;
                    let record = target
                        .apply_recorded(DocEdit::AddValue {
                            placement: new,
                            value: Box::new(value),
                            fresh: fresh.fresh.clone(),
                        })
                        .map_err(&edit)?;
                    vars.minted(fresh, &record.fresh);
                    continue;
                }
            };
            if !settle(old, &mut mate, run.as_ref()) {
                continue;
            }
            let record = target
                .apply_recorded(DocEdit::AddConstraint {
                    placement: new,
                    mate: Box::new(mate),
                    fresh: fresh.fresh.clone(),
                })
                .map_err(&edit)?;
            vars.minted(fresh, &record.fresh);
            let minted = record.minted_constraints.first().and_then(|id| {
                ids.insert(constraint.id, *id);
                target
                    .doc()
                    .node(new)
                    .and_then(crate::mate::solve::bundle_of)
                    .and_then(|bundle| bundle.iter().find(|c| c.id == *id))
                    .and_then(|c| c.mate().cloned())
            });
            let Some(minted) = minted else {
                unreachable!("an accepted constraint is in its bundle")
            };
            let Some(source_mate) = constraint.mate() else {
                unreachable!("a value continued above")
            };
            let reads = |mate: &crate::mate::Mate| {
                std::iter::once(mate.on)
                    .chain(mate.to.pose().copied())
                    .collect::<Vec<_>>()
            };
            name_selections(source, target, vars, reads(source_mate), reads(&minted))
                .map_err(&edit)?;
        }
    }
    Ok((node_map, step_map, out_map))
}

/// **`node` remapped for the target, with each selection it reads
/// authored afresh there** — the selection's body re-pointed and its
/// names rewritten through the maps — or read by the id an earlier
/// carried reader minted for it.
fn remap_with_selects(
    source: &ProfileDoc,
    node: &Node<ProfileProgram>,
    vars: &VarCarry<'_>,
    (node_map, step_map, out_map): (&NodeMap, &StepMap, &ReadMap),
) -> Result<(Node<ProfileProgram>, BTreeMap<VarId, crate::Operand>), RemapMiss> {
    let rd_output = |slot: crate::OperandSlot, var: VarId| match out_map.get(&var) {
        Some(&carried) => Ok(carried),
        None => match source.operation_of(var) {
            Some(input) => Err(RemapMiss::Input(input)),
            None => Err(RemapMiss::Read {
                slot,
                var,
                output: source
                    .var(var)
                    .is_some_and(|held| held.def().output().is_some()),
            }),
        },
    };
    // A selection read crosses with its reader when its body does:
    // the remapped node keeps the source id, and the insert reads it as
    // the selection authored afresh in the target (or as the one an
    // earlier reader carried).
    let anonymous_pose = |var: VarId| {
        source.var_name(var).is_none()
            && matches!(
                source.var(var).map(crate::Var::def),
                Some(crate::var::VarDef::Pose(_))
            )
    };
    let rd = |slot: crate::OperandSlot, var: VarId| match source.selection(var) {
        Some(select) => rd_output(slot, select.body).map(|_| var),
        None if anonymous_pose(var) => Ok(var),
        None => rd_output(slot, var),
    };
    let carried = remap_node(node, &rd)?;
    let mut selects: BTreeMap<VarId, crate::Operand> = BTreeMap::new();
    // A selection authored afresh over its carried body, its names
    // remapped.
    let reselected = |slot: crate::OperandSlot, var: VarId| -> Result<crate::Operand, RemapMiss> {
        let Some(select) = source.selection(var) else {
            unreachable!("asked of a selection")
        };
        let body = rd_output(slot, select.body)?;
        let names = select
            .names
            .iter()
            .map(|n| {
                remap_name(n, node_map, step_map, out_map).map_err(|missing| RemapMiss::Name {
                    name: Box::new(n.clone()),
                    missing,
                })
            })
            .collect::<Result<Vec<_>, _>>()?;
        Ok(crate::Operand::select(
            crate::Operand::Var(body),
            crate::var::Select::canonical(selection_kind(source, var), names),
        ))
    };
    for (slot, var) in node.operand_rows() {
        if anonymous_pose(var) {
            // A pose written in place crosses as its definition, each
            // read in it carried as the read itself would be and each
            // number re-pointed at the variables the target holds.
            let mut leaf = |leaf: VarId| -> Result<Option<crate::Operand>, RemapMiss> {
                if let Some(&held) = vars.map.get(&leaf) {
                    return Ok(Some(crate::Operand::Var(held)));
                }
                if source.selection(leaf).is_some() {
                    return reselected(slot, leaf).map(Some);
                }
                if anonymous_pose(leaf) {
                    return Ok(None);
                }
                rd_output(slot, leaf).map(|carried| Some(crate::Operand::Var(carried)))
            };
            let mut scalar = |formula: Formula| -> Result<Formula, RemapMiss> {
                let mut missed = None;
                let mapped = formula.substitute_vars(&mut |read| {
                    let dim = kind_of(source, read)?;
                    match vars.map.get(&read) {
                        Some(&held) => Some(Formula::from(&crate::Expr::var(held, dim))),
                        None => {
                            missed.get_or_insert(read);
                            None
                        }
                    }
                });
                match (mapped, missed) {
                    (Ok(mapped), None) => Ok(mapped),
                    (_, missed) => Err(RemapMiss::Read {
                        slot,
                        var: missed.unwrap_or(var),
                        output: false,
                    }),
                }
            };
            selects.insert(
                var,
                rewritten_pose_with(source, var, &mut leaf, &mut scalar)?,
            );
            continue;
        }
        let Some(select) = source.selection(var) else {
            continue;
        };
        let operand = match vars.map.get(&var) {
            Some(&held) => crate::Operand::Var(held),
            None => {
                let body = rd_output(slot, select.body)?;
                let names = select
                    .names
                    .iter()
                    .map(|n| {
                        remap_name(n, node_map, step_map, out_map).map_err(|missing| {
                            RemapMiss::Name {
                                name: Box::new(n.clone()),
                                missing,
                            }
                        })
                    })
                    .collect::<Result<Vec<_>, _>>()?;
                let kind = selection_kind(source, var);
                crate::Operand::select(
                    crate::Operand::Var(body),
                    crate::var::Select::canonical(kind, names),
                )
            }
        };
        selects.insert(var, operand);
    }
    Ok((carried, selects))
}

/// **Each selection a carried edit minted stands for its source**, in
/// read order, and a named one takes its name across: `from` the
/// source's reads, `to` the target's at the same places.
fn name_selections(
    source: &ProfileDoc,
    target: &mut Recording<'_, ProfileProgram>,
    vars: &mut VarCarry<'_>,
    from: impl IntoIterator<Item = VarId>,
    to: impl IntoIterator<Item = VarId>,
) -> Result<(), EditError> {
    for (from, to) in from.into_iter().zip(to) {
        if source.selection(from).is_none() || vars.map.contains_key(&from) {
            continue;
        }
        vars.map.insert(from, to);
        if let Some(name) = source.var_name(from) {
            target.apply(DocEdit::RenameVar {
                var: to.into(),
                name: Some(name.clone()),
            })?;
        }
    }
    Ok(())
}
/// Why [`split`] refused. Typed and specific (spec D-2): every arm
/// names the offending edge, parameter, or name.
///
/// **Which document a node is spoken from** ([`SpokenNode`]): every
/// node and name an arm of its own names is spelled in the ids of the
/// document being split, and is spoken from it; one it does not hold
/// reads `node <tag>`. A forwarded replay refusal ([`Self::PartEdit`],
/// [`Self::RemainderEdit`]) is the edit door's, which speaks from the
/// document the replay writes — the part being built, or the
/// remainder — because the ids it names are that document's. The one
/// refusal the replay raises before the door sees the edit, a forward
/// reference ([`EditError::DeclareNamesMissingNode`], and its
/// unreachable twin [`EditError::UnresolvedInput`]), names an id the
/// rewrite could not map, spelled in the document being split, so it
/// speaks from there, where its recourse sends the reader. No one
/// sentence names nodes from both documents.
///
/// **An id does not say which document it belongs to.** Every
/// document mints from the same empty chain, so two documents can hold
/// one id for different nodes: a host and a part that both begin with
/// the same frame insert mint the same id for it. A site therefore
/// speaks an id from the document whose ids it is spelled in, known by
/// construction, never from whichever document happens to hold it;
/// speaking it from the other would read that document's node and
/// label, and `node <tag>` is no guard against that.
#[derive(Debug)]
pub enum SplitError {
    /// The cut set is empty — there is nothing to split out.
    EmptyCut,
    /// A cut entry does not name a live node.
    UnknownCutNode {
        /// The entry with no live node.
        id: SpokenNode,
    },
    /// The cut names the world, which every document holds: the part
    /// has its own, and the remainder keeps this one.
    CutsTheWorld,
    /// The instance left in the remainder offers no geometry to stand
    /// it where its part's world stands ([`crate::mate::stand`]): its
    /// part evaluated to no body, or to one with no face to read.
    InstanceUnstood,
    /// The new document's identity collides with the document being
    /// split or with a document the cut nodes reference — the new id
    /// must be fresh, or the produced pair could not both exist in one
    /// store (and a self-reference would be an evaluation cycle).
    PartIdCollides {
        /// The colliding identity.
        id: DocumentId,
    },
    /// A remainder node reads cut material, and no one copy of the
    /// part's world stands where what it read did ([`Uncarried`]).
    RemainderReadUncarried {
        /// The remainder reader.
        reader: SpokenNode,
        /// Why no one copy carries the read.
        why: Uncarried,
    },
    /// A recipe edge crosses the cut: its consumer is on one side and
    /// its input on the other, so the cut would sever it (D-2 — the
    /// cut must be ancestor- and consumer-closed).
    SeveredEdge {
        /// The consuming node.
        consumer: SpokenNode,
        /// The consumer's seat whose read reaches the input, a
        /// selection's through its body; `None` where no one seat's
        /// read is the input's.
        slot: Option<crate::OperandSlot>,
        /// The input it consumes.
        input: SpokenNode,
        /// Whether the CONSUMER is the cut-side endpoint.
        consumer_is_cut: bool,
    },
    /// **The cut places nothing in the world** (A4, A10): it holds no
    /// world placement, and a part delivers only its world, so the
    /// instance the split leaves behind would evaluate to nothing.
    NoMaterial {
        /// The cut's first node, in document order.
        node: SpokenNode,
    },
    /// A cut node reads a variable that a kept node also reads. The
    /// variable can move or stay, but it cannot silently become two
    /// variables (D-2's "no silent sharing") — refused naming one
    /// reading node on each side.
    UncutVarReference {
        /// The shared variable, boxed so the refusal stays a small
        /// `Err`.
        var: Box<SpokenVar>,
        /// A cut node referencing it.
        cut_node: SpokenNode,
        /// A kept node referencing it.
        kept_node: SpokenNode,
    },
    /// A variable no node reads is tied by definitions both to a
    /// variable the cut moves and to one that stays, so it can go with
    /// neither document — refused naming one of each.
    DefinitionStraddlesCut {
        /// The tied variable: one whose definition reads `staying`.
        var: Box<SpokenVar>,
        /// A variable the cut moves that the definitions read.
        moving: SpokenVar,
        /// A variable that stays that the definitions read.
        staying: SpokenVar,
        /// Whether this document holds `staying`: a deleted one (VR7)
        /// stays too, since it moves nowhere.
        staying_held: bool,
    },
    /// A cut node reads a variable this document no longer holds (a
    /// deleted one: VR7 leaves its readers unresolved, which is legal
    /// document state). The part could not hold the reader either way:
    /// it declares the variables the cut reads, and there is none.
    UnresolvedVarCrossesCut {
        /// The variable, by its id: this document holds no name for it.
        var: SpokenVar,
        /// A cut node reading it.
        node: SpokenNode,
    },
    /// A name inside a CUT node's payload derives from a node that is
    /// not itself cut — the part document could not express the
    /// reference (a part has no name for its consumer's entities, and
    /// a stranded reference has no node to remap at all).
    PartNameReachesRemainder {
        /// The cut node carrying the reference.
        node: SpokenNode,
        /// The name that reaches outside the cut.
        name: SpokenName,
        /// A node the name derives from that the part document has
        /// no copy of — the id the part-side rewrite could not map,
        /// or, from the precondition below, the earliest derivation
        /// node outside the cut in document order (a deleted one, which
        /// has no place in it, after every live one). For a nested name
        /// it is a node inside one of `name`'s path segments, not
        /// `name`'s own minting node, so `name` alone does not say
        /// which node reaches out.
        missing: SpokenNode,
    },
    /// A remainder-side name derives from BOTH sides of the cut, so it
    /// can re-anchor to neither document alone.
    NameStraddlesCut {
        /// The straddling name.
        name: SpokenName,
        /// The node the part-side rewrite could not map, when the
        /// refusal came from a rewrite. For a nested name that is a
        /// node inside one of `name`'s path segments, not `name`'s
        /// own minting node, so the name alone does not say which
        /// node failed. `None` when the refusal came instead from the
        /// straddle classification, which weighs the whole derivation
        /// set at once and singles out no one node.
        missing: Option<SpokenNode>,
    },
    /// A name the split carries spells a piece of a profile step that
    /// no profile of this document holds — a step a `SetProgram`
    /// dropped, whose names were reported stranded (DM7). Its id is
    /// never minted again here, and the part document would mint it
    /// afresh for a step of its own, so the name cannot cross; repair
    /// the stranded reference first.
    NameOnDroppedStep {
        /// The name.
        name: SpokenName,
        /// The dropped step it spells.
        step: StepId,
    },
    /// A name the split carries holds a vertex ranked along the least
    /// of the edges crossing there (N2's `EdgeCrossing` run), and the
    /// part's re-minted ids make another of those edges least, so the
    /// rank would no longer say which vertex: the carry keeps the order
    /// of the node ids and of each profile's step ids
    /// ([`in_id_order`]), not of step ids across two profiles, and
    /// carrying the name would leave a reference that no longer
    /// resolves.
    RankedRunReordered {
        /// The name.
        name: SpokenName,
    },
    /// A remainder-side BODY name crosses the cut. A document's
    /// product name table deliberately carries no body rows (the
    /// product is the document's own body, not any copy's), so the
    /// `InPart`-wrapped rewrite of a body name could never resolve —
    /// refused rather than silently breaking a resolving name.
    BodyNameCrossesCut {
        /// The crossing body name.
        name: SpokenName,
    },
    /// **A remainder-side name of cut material the part does not
    /// deliver** (A4, A10): a part delivers only its world, and no cut
    /// world placement's copy holds the name's entity, or two do, so
    /// the instance carries it nowhere or twice.
    NameOutsidePartWorld {
        /// The name.
        name: SpokenName,
    },
    /// The new part document's content pin would not compute (the
    /// shared save validator refused it) — unreachable when the edits
    /// replayed clean, surfaced under its own arm rather than hidden.
    Pin {
        /// The typed persistence refusal.
        error: Box<PersistError>,
    },
    /// Replaying the constructed part-side edits refused — a
    /// construction bug in this module or a document state its edit
    /// vocabulary cannot re-author (e.g. a blend selection rebound to a
    /// node inserted after it, which no insertion order can satisfy).
    /// Surfaced typed, never absorbed.
    PartEdit {
        /// The refusing edit's own diagnosis.
        error: Box<EditError>,
    },
    /// Replaying the constructed remainder-side edits refused — same
    /// posture as [`SplitError::PartEdit`].
    RemainderEdit {
        /// The refusing edit's own diagnosis.
        error: Box<EditError>,
    },
}

impl core::fmt::Display for SplitError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::EmptyCut => write!(
                f,
                "split: the cut set is empty, so there is nothing to split out. {}",
                Recourse("name at least one live node in the cut")
            ),
            Self::InstanceUnstood => write!(
                f,
                "split: the part evaluates to no body with a face to stand its instance on, so \
                 the instance cannot be placed where the cut stood. {}",
                Recourse("cut placements whose copies evaluate to a body")
            ),
            Self::UnknownCutNode { id } => write!(
                f,
                "split: the cut names {id}, which is not live. {}",
                Recourse(&format!("leave {id} out of the cut"))
            ),
            Self::CutsTheWorld => write!(
                f,
                "split: the cut names the world, which every document holds. {}",
                Recourse("leave the world out of the cut")
            ),
            Self::NoMaterial { node } => write!(
                f,
                "split: the cut places nothing in the world (it begins at {node}), and a part \
                 delivers only its world, so the instance the split leaves behind would \
                 evaluate to nothing. {}",
                Recourse("add to the cut the placements of the bodies it moves")
            ),
            Self::PartIdCollides { id } => write!(
                f,
                "split: the new document id {id} collides with the split document or a \
                 document the cut references. {}",
                Recourse("split under a fresh document id")
            ),
            Self::SeveredEdge {
                consumer,
                slot,
                input,
                consumer_is_cut,
            } => {
                let (consumer_side, input_side) = cut_and_kept(*consumer_is_cut);
                let (cut, kept) = if *consumer_is_cut {
                    (consumer, input)
                } else {
                    (input, consumer)
                };
                let reads = slot.map_or_else(String::new, |slot| format!("'s {}", slot.label()));
                write!(
                    f,
                    "split: the cut severs the edge from {consumer}{reads} to its input {input}. The \
                     consumer is {consumer_side} and the input is {input_side}, but a cut must be \
                     closed under inputs and consumers. {}",
                    Recourse(&format!("add {kept} to the cut, or leave {cut} out of it"))
                )
            }
            Self::RemainderReadUncarried { reader, why } => write!(
                f,
                "split: {reader} stays and reads cut material, and {why}. {}",
                Recourse(&format!(
                    "cut {reader} too, or place what it reads by one world placement at the \
                     identity in the cut"
                ))
            ),
            Self::UncutVarReference {
                var,
                cut_node,
                kept_node,
            } => write!(
                f,
                "split: {var} is read by {cut_node}, which is cut, and by {kept_node}, which \
                 is kept, and one variable cannot become two documents'. {}",
                Recourse(&format!(
                    "put {cut_node} and {kept_node} on one side of the cut, or give one of \
                     them a variable of its own (DeclareVar, SetParam)"
                ))
            ),
            Self::DefinitionStraddlesCut {
                var,
                moving,
                staying,
                staying_held: true,
            } => write!(
                f,
                "split: no node reads {var}, and definitions tie it both to {moving}, which \
                 moves with the cut, and to {staying}, which stays, so it can go with neither \
                 document. {}",
                Recourse(&format!(
                    "define {var} over variables of one side (DefineVar), or delete it \
                     (DeleteVar), then split"
                ))
            ),
            Self::DefinitionStraddlesCut {
                var,
                moving,
                staying,
                staying_held: false,
            } => write!(
                f,
                "split: no node reads {var}, and definitions tie it both to {moving}, which \
                 moves with the cut, and to {staying}, which this document no longer holds, so \
                 the part has no variable to read. {}",
                Recourse(&format!(
                    "redefine {var} without {staying} (DefineVar), or delete it (DeleteVar), \
                     then split"
                ))
            ),
            Self::UnresolvedVarCrossesCut { var, node } => write!(
                f,
                "split: {node}, which is cut, reads {var}, which this document no longer \
                 holds, so the part has no variable to read. {}",
                Recourse(&format!(
                    "repoint {node}'s reader at a variable this document holds, or remove the \
                     reader, then split"
                ))
            ),
            Self::PartNameReachesRemainder {
                node,
                name,
                missing,
            } => write!(
                f,
                "split: {node} is cut, but its reference ({name}) derives from {missing}, \
                 which is outside the cut, so the new document could not express it. {}",
                Recourse(&format!(
                    "add {missing} to the cut, or rebind that name to an entity inside the cut \
                     (Rebind)"
                ))
            ),
            Self::NameStraddlesCut { name, missing } => {
                write!(
                    f,
                    "split: {name} derives from both sides of the cut and can re-anchor to \
                     neither document"
                )?;
                // The rewrite stopped at ONE node, which for a nested
                // name is not the name's own mint.
                if let Some(id) = missing {
                    write!(f, ", and the rewrite stopped at {id}")?;
                }
                write!(
                    f,
                    ". {}",
                    Recourse(
                        "put every node it derives from on one side of the cut, or rebind it to \
                         an entity of one side (Rebind)"
                    )
                )
            }
            Self::NameOnDroppedStep { name, step } => write!(
                f,
                "split: {name} spells a piece of the profile step {step}, which no profile \
                 of this document draws any more. {}",
                Recourse("rebind that name to a live entity (Rebind), then split")
            ),
            Self::RankedRunReordered { name } => write!(
                f,
                "split: {name} is ranked along an edge the part's ids no longer put first. {}",
                Recourse("rebind it (Rebind), then split")
            ),
            Self::NameOutsidePartWorld { name } => write!(
                f,
                "split: {name} crosses the cut, but the part delivers only its world, and no \
                 one world placement in the cut places the body it is an entity of. {}",
                Recourse(
                    "add to the cut the one world placement of that body, or rebind the name to \
                     a kept node's entity (Rebind), then split"
                )
            ),
            Self::BodyNameCrossesCut { name } => write!(
                f,
                "split: {name} crosses the cut, and a product's name table carries no \
                 body rows, so the instance-qualified rewrite could never resolve. {}",
                Recourse(
                    "clear what this document sets on that name (ClearAppearance), or rebind it \
                     to a kept node's body (Rebind), then split"
                )
            ),
            // The part replayed clean through the edit doors, which
            // the save validator agrees with, so its pin refusing is
            // this module's defect: the forwarded sentence is evidence
            // for the report, and the stage it names is this one's.
            Self::Pin { error } => write!(
                f,
                "split: the new document's pin would not compute: {}. {}",
                error.sentence(),
                geom_core::KERNEL_DEFECT_ENDING
            ),
            Self::PartEdit { error } => write!(
                f,
                "split: a part-side edit refused: {}{}",
                error.problem(),
                ReplayTail(error, Replay::SplitPart)
            ),
            Self::RemainderEdit { error } => write!(
                f,
                "split: a remainder-side edit refused: {}{}",
                error.problem(),
                ReplayTail(error, Replay::SplitRemainder)
            ),
        }
    }
}

impl core::error::Error for SplitError {}

/// The sides of a severed pair: the first endpoint's, then the
/// second's.
fn cut_and_kept(first_is_cut: bool) -> (&'static str, &'static str) {
    if first_is_cut {
        ("cut", "kept")
    } else {
        ("kept", "cut")
    }
}

/// **Why no one copy stands where a read across the seam read** (A4,
/// A10): split's remainder reader of cut material, inline's reader of
/// the instance, a name on the instance's copies, and the part copy a
/// mate-placed instance's bundle would move onto.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Uncarried {
    /// It places this many bodies, not one.
    Bodies {
        /// How many.
        count: usize,
    },
    /// The one body standing where it did is named already.
    HeirNamed {
        /// That name.
        held: crate::doc::VarName,
    },
    /// Its one placement holds a pose of its own, so what stood where
    /// the read did is that placement's copy, moved.
    Posed {
        /// The placement, spoken from the document that holds it.
        placement: SpokenNode,
    },
    /// The copy lives in a loose space, which a part does not deliver.
    Loose {
        /// The placement, spoken from the document that holds it.
        placement: SpokenNode,
    },
    /// The one placement on the world copies recipe content, which no
    /// bundle of an instance's faces may move onto (the B4 interim,
    /// `work/intent/the-solve-reads-a-face-only-of-an-instance.md`).
    NotAnInstance {
        /// The placement, spoken from the document that holds it.
        placement: SpokenNode,
    },
    /// The read names a copy by a key the part delivers no copy at.
    NoSuchCopy {
        /// The key.
        key: Box<crate::CopyKey>,
    },
}

impl core::fmt::Display for Uncarried {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Bodies { count } => write!(
                f,
                "the world in question places {count} copies, so no one copy stands where it did"
            ),
            Self::HeirNamed { held } => {
                write!(f, "the copy standing where it did is named {held} already")
            }
            Self::Posed { placement } => write!(
                f,
                "{placement} places it at a pose of its own, so what stands where it did is a \
                 copy moved"
            ),
            Self::Loose { placement } => write!(
                f,
                "{placement}'s copy is loose, and a part delivers only its world"
            ),
            Self::NotAnInstance { placement } => write!(
                f,
                "{placement} copies recipe content rather than placing an instance, and a \
                 face of it is not one a constraint reads yet"
            ),
            Self::NoSuchCopy { key } => write!(f, "the part delivers no copy keyed {key}"),
        }
    }
}

/// Why [`inline`] refused (spec D-3). Typed and specific.
///
/// **Which document a node is spoken from** ([`SpokenNode`]): the
/// instance, its consumer, and a host name that derives from the
/// instance ([`Self::InstanceBodyNameReferenced`],
/// [`Self::ForeignInstanceName`]) are the host's, spoken from it. A
/// part placement, and a name being carried out of the part
/// ([`Self::NameOnDroppedStep`], [`Self::StrandedPartName`]), are the
/// referenced document's, spoken from it; a stranded name's missing
/// node is one it does not hold, `node <tag>`. A forwarded replay
/// refusal ([`Self::Edit`]) is the edit door's, spoken from the host
/// the replay writes, except a forward reference the replay raises
/// before the door sees the edit, which names a part id the rewrite
/// could not map and speaks from the part, as its recourse does
/// ([`SplitError`] says the same of a split, and why an id's document
/// is known by construction rather than by asking who holds it).
#[derive(Debug)]
pub enum InlineError {
    /// The target id is not a live node.
    UnknownNode {
        /// The missing id.
        id: SpokenNode,
    },
    /// The target node does not instantiate a part.
    NotAnInstance {
        /// The non-instance target.
        node: SpokenNode,
    },
    /// **The instance is placed by other constraints than one world
    /// pin, and the part is not one instance pinned to its world at the
    /// identity** (A4): only such a part's one placement can take the
    /// instance's bundle and move the part's copies as the instance
    /// moved them.
    UnplaceablePart {
        /// The instance.
        instance: SpokenNode,
        /// What the part's placements on its world are instead.
        why: Uncarried,
    },
    /// **A host node reads the instance's copies, and no one copy of
    /// the part takes the read** (A4): a whole read of the copies, or a
    /// key the part delivers nothing at.
    InstanceReadUncarried {
        /// The reader.
        reader: SpokenNode,
        /// Why no one copy takes its read.
        why: Uncarried,
    },
    /// The reference did not resolve — the resolver's own classified
    /// refusal (A4's pin gate arrives here as
    /// [`crate::ResolveFault::PinMismatch`]: inline of a stale pin is
    /// refused, never silently retargeted).
    Unresolved {
        /// The resolver's classified failure.
        failure: ResolveFailure,
    },
    /// The referenced document's recorded ε disagrees with the host's
    /// (A2's ε seam, re-checked at this door because the spliced nodes
    /// would otherwise adopt the host's ε silently).
    EpsilonSeam {
        /// The host's recorded ε.
        host_eps: f64,
        /// The referenced document's recorded ε.
        part_eps: f64,
    },
    /// The referenced document carries free-form document metadata,
    /// which the edit vocabulary has no arm to splice — refused rather
    /// than silently dropped.
    PartCarriesMetadata {
        /// The first metadata key, in map order.
        key: String,
    },
    /// The referenced document holds a variable under a name the host
    /// also holds. The carried variable is a new one whatever its
    /// value (VR1), and a name is unique (VR2): inline neither merges
    /// the two nor drops or mints a name.
    VarNameConflict {
        /// The name.
        name: crate::doc::VarName,
    },
    /// The instance's copies carry a name (VR2) and the referenced
    /// document has no one copy to carry it onto: inline neither drops
    /// a name nor gives a variable two.
    InstanceOutputUncarried {
        /// The name.
        name: crate::doc::VarName,
        /// Why no copy takes it.
        why: Uncarried,
    },
    /// The referenced document's spliced recipe reads a variable that
    /// document no longer holds (a deleted one, legal there by VR7):
    /// the host has no variable to point the reader at.
    UnresolvedVarCrossesCut {
        /// The variable, by its id, spoken from the referenced document.
        var: SpokenVar,
        /// A node of the referenced document reading it.
        node: SpokenNode,
    },
    /// **The instance sits at a pose of its own and a construction of
    /// the part reads a copy** (D's interim: a construction is of the
    /// document's one kind): the part's copies would move with the pose
    /// and its constructions would not, so the construction would read
    /// the copy moved.
    ConstructionReadsCopy {
        /// The construction, spoken from the referenced document.
        node: SpokenNode,
    },
    /// **The part's world moves under the inline and one of its copies
    /// is pinned against a construction** (D's interim: a construction
    /// is of the document's one kind): at a pose of the instance's own,
    /// or as the root that takes the instance's bundle, the part's
    /// copies move and its constructions do not, so the copy would land
    /// where the construction stays rather than where the instance put
    /// it.
    CopyPinnedToConstruction {
        /// The part placement, spoken from the referenced document.
        placement: SpokenNode,
        /// The construction its bundle pins it against, likewise.
        construction: SpokenNode,
    },
    /// **The instance's pin reads a variable**: the part's world mates
    /// take the motion the pin solved to as numbers, so the variable
    /// would no longer move what it moved.
    PinReadsVariable {
        /// The instance.
        instance: SpokenNode,
        /// The variable its pin reads, boxed so the refusal stays a small
        /// `Err`.
        var: Box<SpokenVar>,
    },
    /// **A part copy's value is charted on the world the inline moves**:
    /// a value on a mate to the part's world measures the copy from the
    /// world's own frame, which stays where it is once the part's world
    /// moves with the instance, so the copy would not move with it.
    ValueOnAMovedWorld {
        /// The part placement, spoken from the referenced document.
        placement: SpokenNode,
        /// The value.
        value: crate::node::ConstraintId,
    },
    /// **The instance's pin to the world did not solve**: the motion
    /// the part's world would move by has no value.
    InstanceUnsolved {
        /// The instance.
        instance: SpokenNode,
    },
    /// A host reference names the instance's own OUTPUT BODY. The
    /// spliced recipe has no single node whose body corresponds to the
    /// instance's placed product, so the reference cannot re-anchor.
    InstanceBodyNameReferenced {
        /// The instance-body name.
        name: SpokenName,
    },
    /// A host reference derives from the instance but is not the
    /// bridge's own `InPart`-wrapped form, so this door does not know
    /// what local name corresponds to it.
    ForeignInstanceName {
        /// The unrecognized name.
        name: SpokenName,
    },
    /// A name the inline carries spells a piece of a profile step no
    /// profile of the referenced document holds — a step a
    /// `SetProgram` dropped there (DM7). The host would mint the id
    /// afresh for a step of its own, so the name cannot cross.
    NameOnDroppedStep {
        /// The name.
        name: SpokenName,
        /// The dropped step it spells.
        step: StepId,
    },
    /// A name the inline carries holds a vertex ranked along the least
    /// of the edges crossing there (N2's `EdgeCrossing` run), and the
    /// part's re-minted ids make another of those edges least, so the
    /// rank would no longer say which vertex: the carry keeps the order
    /// of the node ids and of each profile's step ids
    /// ([`in_id_order`]), not of step ids across two profiles, and
    /// carrying the name would leave a reference that no longer
    /// resolves.
    RankedRunReordered {
        /// The name.
        name: SpokenName,
    },
    /// A name to be spliced derives from a node the referenced
    /// document no longer has (an N5-stranded reference) — there is no
    /// node to remap it onto; repair it in the part document first.
    StrandedPartName {
        /// The stranded name.
        name: SpokenName,
        /// The node the referenced document no longer has. For a
        /// nested name it is a node inside one of `name`'s path
        /// segments, not `name`'s own minting node.
        missing: SpokenNode,
    },
    /// A name to be spliced carries a read no operation of the
    /// referenced document defines any longer (an N5-stranded
    /// reference); repair it in the part document first.
    StrandedPartRead {
        /// The stranded name.
        name: SpokenName,
        /// The read no operation defines.
        read: crate::spoken::SpokenVar,
    },
    /// Replaying the constructed edits refused — a construction bug in
    /// this module or a host/part state the edit vocabulary cannot
    /// re-author. Surfaced typed, never absorbed.
    Edit {
        /// The refusing edit's own diagnosis.
        error: Box<EditError>,
    },
}

impl core::fmt::Display for InlineError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::UnknownNode { id } => write!(
                f,
                "inline: {id} is not live. {}",
                Recourse("name a live instance of a part")
            ),
            Self::NotAnInstance { node } => write!(
                f,
                "inline: {node} does not instantiate a part. {}",
                Recourse("name an instance of a part")
            ),
            Self::UnplaceablePart { instance, why } => write!(
                f,
                "inline: {instance} is placed by constraints other than mates to the world, and \
                 only a part whose world is one instance pinned at the identity takes them, \
                 but {why}. {}",
                Recourse(&format!(
                    "pin {instance} to the world alone, or point it (UpdateReference) at a \
                     version whose world is one instance pinned at the identity"
                ))
            ),
            Self::InstanceReadUncarried { reader, why } => write!(
                f,
                "inline: {reader} reads the instance's copies, and {why}. {}",
                Recourse(&format!(
                    "re-point {reader} at one copy of the instance (SetParam), then inline"
                ))
            ),
            // The store's sentence states the recourse of a pin or a
            // lookup; the ε seam's is the same whatever the store.
            Self::Unresolved { failure } => match failure.fault {
                crate::ResolveFault::EpsilonSeam => write!(
                    f,
                    "inline: the referenced document's recorded tolerance disagrees with this \
                     process's: {failure}. {}",
                    Recourse(crate::part::EPSILON_SEAM_RECOURSE)
                ),
                crate::ResolveFault::PinMismatch | crate::ResolveFault::Unresolved => {
                    write!(f, "inline: the reference did not resolve: {failure}")
                }
            },
            Self::EpsilonSeam { host_eps, part_eps } => write!(
                f,
                "inline: the referenced document records tolerance {part_eps:e} but the host \
                 records {host_eps:e}, and one document holds one ε. {}",
                Recourse(
                    "set this document's tolerance to the referenced document's (SetTolerance), \
                     then inline"
                )
            ),
            Self::PartCarriesMetadata { key } => write!(
                f,
                "inline: the referenced document carries metadata ({key:?}) the edit vocabulary \
                 cannot splice. {}",
                Recourse(&format!(
                    "delete {key:?} from the referenced document's file, point this instance at \
                     that version (UpdateReference), then inline"
                ))
            ),
            Self::InstanceOutputUncarried { name, why } => write!(
                f,
                "inline: the instance's copies are named {name}, and {why}. {}",
                Recourse(&format!("clear the name {name} (RenameVar), then inline"))
            ),
            Self::VarNameConflict { name } => write!(
                f,
                "inline: both documents hold a variable named {name}, and the referenced \
                 document's crosses as a variable of its own. {}",
                Recourse(&format!(
                    "rename the {name} of one of the two documents (RenameVar), then inline"
                ))
            ),
            Self::UnresolvedVarCrossesCut { var, node } => write!(
                f,
                "inline: {node} in the referenced document reads {var}, which that document \
                 no longer holds, so this document has no variable to point the reader at. {}",
                Recourse(&format!(
                    "in the referenced document, repoint {node}'s reader at a variable it holds \
                     or remove the reader, point this instance at that version \
                     (UpdateReference), then inline"
                ))
            ),
            Self::ConstructionReadsCopy { node } => write!(
                f,
                "inline: the instance sits at a pose of its own, and the referenced document's \
                 {node} reads a copy, which would move with that pose while {node} does not. {}",
                Recourse(
                    "pin the instance to the world at the identity, or re-point the \
                     construction at a body, then inline"
                )
            ),
            Self::CopyPinnedToConstruction {
                placement,
                construction,
            } => write!(
                f,
                "inline: the instance does not sit at the identity, and the referenced \
                 document's {placement} is pinned against {construction}, which would stay \
                 where it is while the copy moves with the instance. {}",
                Recourse(
                    "pin the instance to the world at the identity, or pin the placement \
                     against a copy, then inline"
                )
            ),
            Self::PinReadsVariable { instance, var } => write!(
                f,
                "inline: {instance}'s pin to the world reads {var}, and once spliced its part's \
                 world mates take the pin's motion as numbers, so {var} would move nothing. {}",
                Recourse(&format!(
                    "write {instance}'s pin by numbers, or keep the instance"
                ))
            ),
            Self::ValueOnAMovedWorld { placement, value } => write!(
                f,
                "inline: {placement}'s value {value} is charted on its part's world frame, \
                 which stays where it is when the part's world moves with the instance, so \
                 the copy would not move with the rest. {}",
                Recourse("inline the instance where it stands at the identity, or keep it")
            ),
            Self::InstanceUnsolved { instance } => write!(
                f,
                "inline: {instance}'s pin to the world did not solve, so there is no motion \
                 to move its part's world by. {}",
                Recourse(&format!(
                    "evaluate the document and repair {instance}'s pin"
                ))
            ),
            Self::InstanceBodyNameReferenced { name } => write!(
                f,
                "inline: {name} names the instance's own output body, which no single \
                 spliced node corresponds to. {}",
                Recourse(
                    "clear what this document sets on that name (ClearAppearance), or rebind it \
                     to another node's body (Rebind), then inline"
                )
            ),
            Self::ForeignInstanceName { name } => write!(
                f,
                "inline: {name} derives from the instance but is not an instance-qualified \
                 (`InPart`) name, so it cannot re-anchor. {}",
                Recourse(
                    "rebind that name to an `InPart` name of the instance, or to an entity that \
                     does not derive from it (Rebind), then inline"
                )
            ),
            Self::NameOnDroppedStep { name, step } => write!(
                f,
                "inline: {name} spells a piece of the profile step {step}, which no profile \
                 of the referenced document draws any more. {}",
                Recourse(STRANDED_IN_THE_PART)
            ),
            Self::RankedRunReordered { name } => write!(
                f,
                "inline: {name} is ranked along an edge the host's ids no longer put first. {}",
                Recourse("rebind it in the part document (Rebind), then inline")
            ),
            Self::StrandedPartName { name, missing } => write!(
                f,
                "inline: {name} derives from {missing}, which the referenced document no \
                 longer has. {}",
                Recourse(STRANDED_IN_THE_PART)
            ),
            Self::StrandedPartRead { name, read } => write!(
                f,
                "inline: {name} is carried in through {read}, which no operation of the \
                 referenced document defines any longer. {}",
                Recourse(STRANDED_IN_THE_PART)
            ),
            Self::Edit { error } => write!(
                f,
                "inline: an edit refused: {}{}",
                error.problem(),
                ReplayTail(error, Replay::Inline)
            ),
        }
    }
}

impl core::error::Error for InlineError {}

/// What an inline does about a name the referenced document stranded:
/// the name is that document's, so the repair is made there.
const STRANDED_IN_THE_PART: &str = "in the referenced document, rebind that name to a live entity \
     (Rebind), point this instance at that version (UpdateReference), then inline";

/// Which of this module's edit replays refused. The user authored none
/// of those edits, so the edit door's own recourse — written for the
/// person who typed the edit — is not theirs to follow; the replay's
/// door states its own ([`ReplayTail`]).
#[derive(Clone, Copy, PartialEq, Eq)]
enum Replay {
    /// The part document, rebuilt from empty in the cut's document
    /// order.
    SplitPart,
    /// The remainder: the instance inserted, the crossing names rebound
    /// onto it, the cut deleted.
    SplitRemainder,
    /// The part's nodes spliced into the host in the part's document
    /// order, its records carried, the instance's names rebound.
    Inline,
}

/// The ending a replaying door gives a forwarded [`EditError`]: a
/// recourse where the replay cannot re-author a document state the
/// user can change, the kernel-defect ending where the replay only
/// re-writes what a document already holds, or nothing where the
/// forwarded sentence is another layer's whole refusal.
struct ReplayTail<'a>(&'a EditError, Replay);

impl core::fmt::Display for ReplayTail<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let defect =
            |f: &mut core::fmt::Formatter<'_>| write!(f, ". {}", geom_core::KERNEL_DEFECT_ENDING);
        let Self(error, replay) = *self;
        match error {
            // A payload name on a node inserted AFTER the one carrying
            // it — a blend selection or a frame's face rebound forward.
            // The
            // replay inserts in document order, so no order satisfies
            // it. The remainder inserts one instance, whose names the
            // replay wrote itself.
            EditError::DeclareNamesMissingNode { .. } => match replay {
                Replay::SplitPart => write!(
                    f,
                    ". {}",
                    Recourse(
                        "rebind that reference to an entity of a node that comes before the \
                         node carrying it, since the split rebuilds the part in document order"
                    )
                ),
                Replay::Inline => write!(
                    f,
                    ". {}",
                    Recourse(
                        "in the part document, rebind that reference to an entity of a node \
                         that comes before the node carrying it, since the inline splices the \
                         part in its document order"
                    )
                ),
                Replay::SplitRemainder => defect(f),
            },
            // Inline carries the part's appearance records onto the
            // spliced names and THEN rebinds this document's
            // instance-qualified names onto them, so a record on both
            // sides collides. A split's rebind targets are names of an
            // instance it has just minted, which carry nothing.
            EditError::RebindAppearanceCollision { .. }
            | EditError::RebindMetadataCollision { .. } => match replay {
                Replay::Inline => write!(
                    f,
                    ". {}",
                    Recourse(
                        "clear what this document sets on the instance's name, since the part \
                         carries its own"
                    )
                ),
                Replay::SplitPart | Replay::SplitRemainder => defect(f),
            },
            // The forwarded mate refusal's whole sentence, which the
            // mate solve owns. A part the solve could not lever through
            // states its resolver's recourse inside it.
            EditError::MateRefused { .. } => Ok(()),
            // Every other edit re-writes what the source document or
            // the part already holds, each validated at its own door
            // when it was written: a refusal here is this module's
            // construction bug. Listed arm by arm, so a new arm is
            // classified here or does not compile.
            EditError::UnknownNode { .. }
            | EditError::ProfileProgramRefused { .. }
            | EditError::UnresolvedInput { .. }
            | EditError::WouldCycle { .. }
            | EditError::SelectionShape { .. }
            | EditError::SelectionNameKind { .. }
            | EditError::PoseShape { .. }
            | EditError::IndexedRead { .. }
            | EditError::SetMembersOnNonList { .. }
            | EditError::SetProgramOnNonProfile { .. }
            | EditError::StepIdsRefused { .. }
            | EditError::LoftSectionsSpelled { .. }
            | EditError::OperandUnresolved { .. }
            | EditError::AmbiguousOutput { .. }
            | EditError::DefinesNothing { .. }
            | EditError::UnknownSlot { .. }
            | EditError::SlotDimensionMismatch { .. }
            | EditError::StructuralSlotNeedsStructuralEdit { .. }
            | EditError::NotStructuralSlot { .. }
            | EditError::SlotUnknownVarName { .. }
            | EditError::SlotVarKind { .. }
            | EditError::PayloadUnknownVarName { .. }
            | EditError::PayloadVarKind { .. }
            | EditError::SlotUnresolvedVar { .. }
            | EditError::PayloadUnresolvedVar { .. }
            | EditError::VarNameUnchanged { .. }
            | EditError::AnonymousVarUnread { .. }
            | EditError::SharedVarNeedsName { .. }
            | EditError::DeleteAnonymousVar { .. }
            | EditError::AssertionDimension { .. }
            | EditError::ConstructionReadsObserved { .. }
            | EditError::ContinuousVarCannotBeCount { .. }
            | EditError::UnknownVar { .. }
            | EditError::VarNameTaken { .. }
            | EditError::FreshUnheld { .. }
            | EditError::FreshKind { .. }
            | EditError::FreshUnread { .. }
            | EditError::VarKindFixed { .. }
            | EditError::NotAFreeVar { .. }
            | EditError::VarIsAnOutput { .. }
            | EditError::DefinitionCycle { .. }
            | EditError::DefinitionTooLarge { .. }
            | EditError::DefinitionUnknownVarName { .. }
            | EditError::DefinitionUnresolvedVar { .. }
            | EditError::DefinitionVarKind { .. }
            | EditError::VarCountHasNoUnit { .. }
            | EditError::VarCountHasNoDistribution { .. }
            | EditError::VarUnitMismatch { .. }
            | EditError::VarValueKindMismatch { .. }
            | EditError::CopyReadOffBodySeat { .. }
            | EditError::PathOffTree { .. }
            | EditError::Dimension(_)
            | EditError::NameStepNeverMinted { .. }
            | EditError::ReadSiteMissingNode { .. }
            | EditError::NonFiniteVar { .. }
            | EditError::InvalidDistribution { .. }
            | EditError::RebindTargetMissingNode { .. }
            | EditError::RebindUnknownName { .. }
            | EditError::RebindKindMismatch { .. }
            | EditError::RebindIdentity { .. }
            | EditError::RebindNoReferences { .. }
            | EditError::WitnessOnNonSketch { .. }
            | EditError::DuplicateWitnessEntry { .. }
            | EditError::EmptyWitnessBulk
            | EditError::NameUnresolvedInEvaluation { .. }
            | EditError::EvaluationOfAnotherDocument { .. }
            | EditError::AppearanceWrongKind { .. }
            | EditError::AppearanceNamesMissingNode { .. }
            | EditError::AppearanceBodyNotARead { .. }
            | EditError::AppearanceNotSet { .. }
            | EditError::InvalidTolerance { .. }
            | EditError::MetaUnversioned { .. }
            | EditError::MetaNonFinite { .. }
            | EditError::MetaNotSet { .. }
            | EditError::WorldIsUndeletable { .. }
            | EditError::ConstraintOnNonPlacement { .. }
            | EditError::UnknownConstraint { .. }
            | EditError::MateNamedByValue { .. }
            | EditError::PlacementRuleMismatch { .. }
            | EditError::EmptyPlacementList { .. }
            | EditError::ImproperPlacement { .. }
            | EditError::NonFinitePlacement { .. }
            | EditError::NonRigidPlacement { .. }
            | EditError::PlacementAxis { .. }
            | EditError::UpdateOnNonInstance { .. }
            | EditError::PinUnchanged { .. }
            | EditError::LabelUnchanged { .. } => defect(f),
        }
    }
}

/// What [`split`] produced: the two documents, the recorded edits
/// that produce each (the part's from the empty document under the
/// caller's id, the remainder's from the input document), and the
/// [`crate::Maintenance`] each edit list performed, net of what a later
/// edit in the same list took back. Undo of the
/// refactoring is the caller keeping the input value — the input is
/// untouched.
#[derive(Debug, Clone)]
pub struct SplitOutcome {
    /// The input document with the cut nodes replaced by one instance
    /// of the new part document.
    pub remainder: ProfileDoc,
    /// The new part document holding the cut nodes.
    pub part: ProfileDoc,
    /// The recorded edits producing `remainder` from the input.
    pub remainder_edits: Vec<DocEdit<ProfileProgram>>,
    /// The maintenance `remainder_edits` reported, net of what a later
    /// edit in the list took back ([`crate::MaintenanceNet`]), in edit order:
    /// a payload name a departing cut node stranded behind it (DM7)
    /// that `remainder` still carries. A refactoring is one action, so
    /// the outcome carries what the action did beside what it
    /// produced: a caller holding a document with the maintenance of
    /// its last accepted action swaps `remainder` and this in together.
    pub remainder_maintenance: Vec<Maintenance>,
    /// The recorded edits producing `part` from
    /// `Doc::empty(part_id)`.
    pub part_edits: Vec<DocEdit<ProfileProgram>>,
    /// The maintenance `part_edits` reported, net of what a later edit
    /// in the list took back ([`crate::MaintenanceNet`]), in edit order.
    pub part_maintenance: Vec<Maintenance>,
    /// The remainder's new instantiate node.
    pub instance: RecipeNodeId,
    /// Cut-node ids → their part-document ids (minted in document
    /// order — the D9-deterministic remap).
    pub node_map: NodeMap,
    /// The cut profiles' step ids → the ids the part document minted
    /// for them, in the same order.
    pub step_map: StepMap,
}

/// What [`inline`] produced: the host with the referenced document's
/// recipe spliced in and the instance gone, plus the recorded edits
/// that produce it and the maintenance they performed, net of what a
/// later edit in the list took back.
/// Undo is the caller keeping the input value.
#[derive(Debug, Clone)]
pub struct InlineOutcome {
    /// The host document after the splice.
    pub doc: ProfileDoc,
    /// The recorded edits producing `doc` from the input.
    pub edits: Vec<DocEdit<ProfileProgram>>,
    /// The maintenance `edits` reported, net of what a later edit in
    /// the list took back ([`crate::MaintenanceNet`]), in edit order. A
    /// refactoring is one action; a caller holding a
    /// document with the maintenance of its last accepted action swaps
    /// `doc` and this in together.
    pub maintenance: Vec<Maintenance>,
    /// Part-document node ids → their host ids (minted in the part's
    /// document order).
    pub node_map: NodeMap,
    /// The part's profile step ids → the ids the host minted for them,
    /// in the same order.
    pub step_map: StepMap,
}

// ---- Name and node remapping ----

/// Rewrites every LOCAL node id in `name` through `map` — the minting
/// node, embedded operand names, and discriminator partners: exactly
/// the id set [`derivation_nodes`] reads, because that is the set that
/// belongs to THIS document's id space. `InPart` arguments cross
/// VERBATIM (they name another document's nodes — the walk_names seam
/// rule). The path is walked by `StableName::rewrite_path`, which
/// puts it back in canonical form, since a caller's maps need not keep
/// id order and every name-ordered position may come out reordered.
///
/// Public because a name held outside the document — a caller's own
/// reference into a split or inlined part — crosses the same map the
/// split and the inline apply to the names they carry, and has to come
/// out spelled the way theirs do.
///
/// # Errors
///
/// [`RemapError::Unmapped`] at the first local id the maps lack;
/// [`RemapError::RankedRunReordered`] where the maps make another line
/// least in a ranked run the name holds or embeds, which a split's or
/// an inline's maps never do.
pub fn remap_name(
    name: &StableName,
    map: &NodeMap,
    steps: &StepMap,
    reads: &ReadMap,
) -> Result<StableName, RemapError> {
    let (node, path) = remap_derivation(name.kind, name.node, &name.path, map, steps, reads)?;
    Ok(StableName {
        kind: name.kind,
        node,
        path,
    })
}

/// The half of [`remap_name`] that a name's KIND is not part of: the
/// minting node and the role path, rewritten through `map` and
/// `steps`. The kind is read, never written: the path's canonical form
/// depends on it (a junction is a vertex's; a rank's rule is an edge's
/// or a vertex's).
///
/// # Errors
///
/// The first local id the maps lack.
fn remap_derivation(
    kind: EntityKind,
    node: RecipeNodeId,
    path: &[RoleSeg],
    map: &NodeMap,
    steps: &StepMap,
    reads: &ReadMap,
) -> Result<(RecipeNodeId, crate::names::RolePath), RemapError> {
    let to = *map.get(&node).ok_or(Unmapped::Node(node))?;
    let rewritten = StableName {
        kind,
        node,
        path: path.to_vec(),
    }
    .rewrite_path(&mut Remapping(map, steps, reads))?;
    Ok((to, rewritten.into_path()))
}

/// **The split re-map as a [`SegRewrite`]**: every carried name is
/// rewritten through [`remap_name`] — its minting node through the
/// map, then its own path through this same rewriter, so the descent
/// is [`remap_name`]'s and not the walk's — a read, which is a local id
/// like the minting node, is mapped too, and so is every profile
/// locator's step, which the other document re-minted. A kernel-built
/// section's locator has no step and crosses as it is. The walk over
/// [`RoleSeg`]'s shape is [`RoleSeg::rewrite`]'s.
struct Remapping<'a>(&'a NodeMap, &'a StepMap, &'a ReadMap);

impl Remapping<'_> {
    fn step(&self, step: StepId) -> Result<StepId, RemapError> {
        Ok(self.1.get(&step).copied().ok_or(Unmapped::Step(step))?)
    }

    fn node(&self, m: RecipeNodeId) -> Result<RecipeNodeId, RemapError> {
        Ok(self.0.get(&m).copied().ok_or(Unmapped::Node(m))?)
    }
}

impl SegRewrite for Remapping<'_> {
    type Error = RemapError;

    fn edge(&mut self, e: ProfileEdgeRef) -> Result<ProfileEdgeRef, Self::Error> {
        Ok(match e {
            ProfileEdgeRef::Piece { step, role } => ProfileEdgeRef::Piece {
                step: self.step(step)?,
                role,
            },
            ProfileEdgeRef::Section { .. } => e,
        })
    }

    fn vertex(&mut self, v: ProfileVertexRef) -> Result<ProfileVertexRef, Self::Error> {
        Ok(match v {
            ProfileVertexRef::Piece { step, role } => ProfileVertexRef::Piece {
                step: self.step(step)?,
                role,
            },
            ProfileVertexRef::Section { .. } => v,
        })
    }

    // [`remap_name`], one level at a time: the minting node is mapped
    // (and an unmapped one refused) before the path is walked, and the
    // walked path is then put under it.
    fn name(&mut self, n: &StableName) -> Result<Carry, Self::Error> {
        self.node(n.node)?;
        Ok(Carry::Descend)
    }

    fn descended(
        &mut self,
        n: &StableName,
        mut walked: StableName,
    ) -> Result<Option<StableName>, Self::Error> {
        walked.node = self.node(n.node)?;
        Ok(Some(walked))
    }

    fn read(&mut self, r: VarId) -> Result<VarId, Self::Error> {
        Ok(self.2.get(&r).copied().ok_or(Unmapped::Read(r))?)
    }

    // Only the run's reason is refused: the other two are emission-time
    // facts (`names::canonical::rewritten` says why), and are carried.
    fn unrankable(&mut self, was: &StableName, why: Unrankable) -> Result<(), Self::Error> {
        match why {
            Unrankable::RunLeastLineMoved => Err(RemapError::RankedRunReordered {
                vertex: Box::new(was.clone()),
            }),
            Unrankable::SidedVertexRank | Unrankable::RankOutsideCount => Ok(()),
        }
    }
}

/// What a payload rewrite could not map: a DAG input (unreachable
/// after the severed-edge check — surfaced as the edit layer's own
/// unresolved-input refusal rather than assumed away) or a referenced
/// name.
enum RemapMiss {
    /// An unmapped DAG input.
    Input(RecipeNodeId),
    /// An operand whose read no live operation defines: the source
    /// holds the reader stranded (D10), and a stranded read names an
    /// id of the source alone.
    Read {
        /// The reader's operand.
        slot: crate::OperandSlot,
        /// The read.
        var: VarId,
        /// Whether the read is an operation's output (a stranded one),
        /// rather than a variable the carry moves on its own.
        output: bool,
    },
    /// An assertion's value reading an output no live operation
    /// defines: a stranded measured value, which the carry cannot
    /// re-point and must not keep.
    PayloadRead {
        /// The read.
        var: VarId,
    },
    /// A name the maps could not carry, and why: the FIRST local id
    /// they lack, or a ranked run they reorder. The two are not
    /// redundant: a [`StableName`] embeds other names in its path, so
    /// the id the rewrite stopped at may belong to a name several
    /// segments down rather than to `name` itself.
    Name {
        /// The name that could not be rewritten.
        name: Box<StableName>,
        /// Why.
        missing: RemapError,
    },
}

/// Rewrites a placement rule's id references: the circular rule's datum
/// axis is the only one the vocabulary carries (a linear rule is pure
/// expressions, an explicit rule pure frames). Shared by both
/// placement-rule nodes so their seam behavior cannot drift apart.
///
/// # Errors
///
/// The first [`RemapMiss`].
fn remap_rule(
    kind: &PatternKind,
    rd: &dyn Fn(crate::OperandSlot, VarId) -> Result<VarId, RemapMiss>,
) -> Result<PatternKind, RemapMiss> {
    Ok(match kind {
        PatternKind::Linear { .. } | PatternKind::Explicit(_) => kind.clone(),
        PatternKind::Circular { axis, step } => PatternKind::Circular {
            axis: axis.try_map(|axis| rd(crate::OperandSlot::Axis, *axis))?,
            step: *step,
        },
    })
}

/// Rewrites a node payload's reads through `rd`, for insertion into
/// the other document.
/// `InstantiatePart`'s document reference crosses verbatim: it
/// is a document seam, not a local id, and its interface record rides
/// with it BECAUSE the [`SplitError::PartNameReachesRemainder`]
/// precondition has already refused any record whose `outer` names a
/// kept node: a crossing `outer` is a payload name
/// ([`crate::Node::payload_names`]), so [`crate::Doc::name_carriers`]
/// reports it and the precondition sees it. No name of the REMAINDER
/// reaches the part's space through this arm — the row is
/// `edit_instance_crossing_names`'s
/// `a_split_that_takes_an_instance_naming_a_kept_node_is_refused`.
/// The match is exhaustive so a future node kind must be classified
/// here.
///
/// # Errors
///
/// The first [`RemapMiss`].
fn remap_node(
    node: &Node<ProfileProgram>,
    rd: &dyn Fn(crate::OperandSlot, VarId) -> Result<VarId, RemapMiss>,
) -> Result<Node<ProfileProgram>, RemapMiss> {
    // A body seat's read crosses through `rd`; its indices are slots,
    // which cross as every slot does, and its key is the copy's
    // identity in what it reads, which crosses with it.
    let br = |seat: crate::OperandSlot, read: &crate::BodyRead<VarId>| {
        Ok::<_, RemapMiss>(crate::BodyRead {
            read: rd(seat, read.read)?,
            key: read.key.clone(),
            at: read.at.clone(),
        })
    };
    // A constraint's sides are reads, and re-point as every read does: a
    // selection crosses with its body, and one the cut severed makes the
    // remap MISS loudly. Its numbers are slots, which split and inline
    // carry beside it.
    let bundle = |constraints: &[crate::mate::Constraint]| {
        constraints
            .iter()
            .map(|c| {
                c.try_map(
                    |side, &var| {
                        rd(
                            crate::OperandSlot::Side {
                                constraint: c.id,
                                side,
                            },
                            var,
                        )
                    },
                    &mut |&slot| Ok::<_, RemapMiss>(slot),
                )
            })
            .collect::<Result<Vec<_>, _>>()
    };
    Ok(match node {
        // A profile reads nothing. Its step ids do NOT cross: the other document's insert door
        // mints its own, and the names that spell them cross through
        // the step map read off that minting ([`carry`]).
        Node::Profile(p) => Node::Profile(ProfileProgram {
            loops: p.loops.clone(),
            ids: Vec::new(),
        }),
        Node::Extrude { profile, distance } => Node::Extrude {
            profile: rd(crate::OperandSlot::Profile, *profile)?,
            distance: *distance,
        },
        Node::Revolve {
            profile,
            axis_through,
            axis_heading,
            angle,
        } => Node::Revolve {
            profile: rd(crate::OperandSlot::Profile, *profile)?,
            axis_through: *axis_through,
            axis_heading: *axis_heading,
            angle: *angle,
        },
        // A tube reads nothing: every field is carried.
        Node::Tube { .. } | Node::HollowTube { .. } => node.clone(),
        Node::Loft { profiles, v_degree } => Node::Loft {
            profiles: (0u32..)
                .zip(profiles)
                .map(|(i, &p)| rd(crate::OperandSlot::Section(i), p))
                .collect::<Result<_, _>>()?,
            v_degree: *v_degree,
        },
        Node::Sweep {
            profile,
            path,
            stations,
            v_degree,
        } => Node::Sweep {
            profile: rd(crate::OperandSlot::Profile, *profile)?,
            path: rd(crate::OperandSlot::Path, *path)?,
            stations: *stations,
            v_degree: *v_degree,
        },
        Node::Fillet { radius, selection } => Node::Fillet {
            radius: *radius,
            selection: rd(crate::OperandSlot::Selection, *selection)?,
        },
        Node::Chamfer {
            distance,
            selection,
        } => Node::Chamfer {
            distance: *distance,
            selection: rd(crate::OperandSlot::Selection, *selection)?,
        },
        Node::Shell { thickness, open } => Node::Shell {
            thickness: *thickness,
            open: rd(crate::OperandSlot::Open, *open)?,
        },
        Node::Split { target, tool } => Node::Split {
            target: br(crate::OperandSlot::Target, target)?,
            tool: tool.try_map(|tool| rd(crate::OperandSlot::Tool, *tool))?,
        },
        Node::Subtract { from, tool } => Node::Subtract {
            from: br(crate::OperandSlot::From, from)?,
            tool: br(crate::OperandSlot::Cut, tool)?,
        },
        Node::Union { members } => Node::Union {
            members: members.try_map(&br)?,
        },
        Node::Intersect { members } => Node::Intersect {
            members: members.try_map(&br)?,
        },
        Node::Place {
            shapes,
            constraints,
        } => Node::Place {
            shapes: (0u32..)
                .zip(shapes)
                .map(|(i, shape)| br(crate::OperandSlot::Shape(i), shape))
                .collect::<Result<_, _>>()?,
            constraints: bundle(constraints)?,
        },
        // The world is every document's own, never carried; a
        // constraint reading it names no id.
        Node::World => Node::World,
        Node::Pattern { input, count, kind } => Node::Pattern {
            input: rd(crate::OperandSlot::Input, *input)?,
            count: *count,
            kind: remap_rule(kind, rd)?,
        },
        // The selector is payload with no id in it (a half, or an
        // index expression); only the edge remaps.
        Node::Part { of, select } => Node::Part {
            of: rd(crate::OperandSlot::Of, *of)?,
            select: select.clone(),
        },
        Node::PlacedUnion { input, count, kind } => Node::PlacedUnion {
            input: rd(crate::OperandSlot::Input, *input)?,
            count: *count,
            kind: remap_rule(kind, rd)?,
        },
        // The reference crosses verbatim (the function's docs say why),
        // and its bundle as a placement's does.
        Node::InstantiatePart {
            doc_ref,
            interface,
            constraints,
        } => Node::InstantiatePart {
            doc_ref: *doc_ref,
            interface: interface.clone(),
            constraints: bundle(constraints)?,
        },
        Node::Measure { primitive } => {
            let verb = primitive.kind();
            let mut i = 0u8;
            Node::Measure {
                primitive: primitive.try_map(|&r| {
                    let slot = crate::OperandSlot::Measured(verb, i);
                    i += 1;
                    rd(slot, r)
                })?,
            }
        }
        // A value reading a measure's output directly re-points as an
        // operand does, and one reading a stranded output refuses; any
        // other value is a slot variable, carried as the bound is. The
        // value is no operand, so its miss names no slot of its own.
        Node::Assertion {
            value,
            bound,
            relation,
        } => Node::Assertion {
            value: match rd(crate::OperandSlot::Input, *value) {
                Err(RemapMiss::Read { output: false, .. }) => *value,
                Err(RemapMiss::Read { var, .. }) => return Err(RemapMiss::PayloadRead { var }),
                read => read?,
            },
            bound: *bound,
            relation: *relation,
        },
    })
}

/// The variables a node's slots and payload read. The expressions no
/// slot addresses count too: a measured bound reading a variable is
/// exactly as much a reason to carry that variable into a split part
/// as an extrude's distance is.
fn node_var_reads(doc: &ProfileDoc, node: &Node<ProfileProgram>) -> BTreeSet<VarId> {
    // A reader of a defined variable reads what its definition reads.
    let mut through: BTreeSet<VarId> = BTreeSet::new();
    let mut frontier: Vec<VarId> = node.exprs().into_iter().copied().collect();
    while let Some(var) = frontier.pop() {
        if through.insert(var) {
            frontier.extend(doc.definition_reads(var));
        }
    }
    through
}

/// **The variables a carry re-points** (VR4, VR6): each variable of
/// the source document the target already holds, by source id.
///
/// A named variable is declared in the target before any node reads
/// it. An anonymous one crosses as an entry of the fresh table of the
/// carried edit that reads it, directly or through the definitions of
/// other anonymous variables, with its definition bit for bit,
/// distribution included: it has one reader (VR2).
struct VarCarry<'s> {
    source: &'s ProfileDoc,
    map: BTreeMap<VarId, VarId>,
}

/// **A run of world-axis moves, as one rigid motion**: what
/// [`fold_world_run`] takes off a part's world read.
type Run = (geom_core::linalg::Mat3<f64>, geom_core::linalg::Vec3<f64>);

/// **The run of written moves a carried world mate's read ends in,
/// taken off it** ([`crate::mate::pin::written_moves`]): each a turn
/// about or slide along a world axis by a number the part holds as an
/// anonymous, untoleranced free value (`pin_at`'s and inline's own
/// writing). `None` for a constraint that is no world mate, or whose
/// read ends in no such move.
fn fold_world_run(source: &ProfileDoc, constraint: &mut crate::mate::Constraint) -> Option<Run> {
    let crate::mate::ConstraintKind::Mate(mate) = &mut constraint.kind else {
        return None;
    };
    let crate::mate::Target::World(world) = &mut mate.to else {
        return None;
    };
    let number = |&var: &VarId| match source.var(var).map(crate::Var::def) {
        Some(VarDef::Free(crate::doc::FreeVar::Continuous {
            value,
            distribution: None,
            ..
        })) if source.var_name(var).is_none() => Some(*value),
        _ => None,
    };
    let before = world.depth();
    let (base, linear, translation) =
        crate::mate::pin::written_moves(std::mem::replace(world, crate::WorldRead::Frame), number);
    let peeled = base.depth() < before;
    *world = base;
    peeled.then_some((linear, translation))
}

/// One carried edit's variables: the fresh table it carries and the
/// source variable each entry stands for.
struct Carried {
    fresh: Vec<crate::FreshEntry>,
    anonymous: Vec<VarId>,
}

impl<'s> VarCarry<'s> {
    fn new(source: &'s ProfileDoc) -> Self {
        Self {
            source,
            map: BTreeMap::new(),
        }
    }

    /// The anonymous variables `reads` reach, through the definitions
    /// of anonymous variables, that the target does not hold yet, in
    /// the source's definition order: a definition after what it
    /// reads.
    fn unheld(&self, reads: impl IntoIterator<Item = VarId>) -> Vec<VarId> {
        let mut reached: BTreeSet<VarId> = BTreeSet::new();
        let mut frontier: Vec<VarId> = reads.into_iter().collect();
        while let Some(var) = frontier.pop() {
            if self.map.contains_key(&var)
                || self.source.var_name(var).is_some()
                || self.source.var(var).is_none()
                || !reached.insert(var)
            {
                continue;
            }
            frontier.extend(self.source.definition_reads(var));
        }
        self.source
            .definition_order()
            .into_iter()
            .filter(|var| reached.contains(var))
            .collect()
    }

    /// A reader of the source variable `var` at `dim`, as the carried
    /// edit writes it: entry `i` of its fresh table where `var` is
    /// `anonymous[i]`, the target's id where the target holds it, and
    /// `var` itself otherwise — a reader of a variable the source no
    /// longer holds, which the target's door refuses in its words.
    fn reader(&self, anonymous: &[VarId], var: VarId, dim: Dimension) -> Formula {
        match anonymous.iter().position(|&held| held == var) {
            Some(index) => Formula::fresh(
                u16::try_from(index).unwrap_or_else(|_| {
                    unreachable!("one edit's fresh table is a node's own variables")
                }),
                dim,
            ),
            None => Formula::var(self.map.get(&var).copied().unwrap_or(var), dim),
        }
    }

    /// `var`'s definition, as the carried edit declares it.
    fn decl(&self, anonymous: &[VarId], var: VarId) -> VarDecl {
        let Some(held) = self.source.var(var) else {
            unreachable!("the carry declares only variables its source holds")
        };
        match held.def() {
            VarDef::Free(free) => VarDecl::Free(free.clone()),
            VarDef::Defined(expr) => {
                let formula = Formula::from(expr)
                    .substitute_vars(&mut |read| {
                        Some(self.reader(anonymous, read, kind_of(self.source, read)?))
                    })
                    .unwrap_or_else(|fault| {
                        unreachable!(
                            "a reader re-pointed at its own kind keeps every dimension: {fault}"
                        )
                    });
                VarDecl::Defined(formula)
            }
            VarDef::Output { .. } => {
                unreachable!("an output crosses with its node, minted by the target's insert")
            }
            VarDef::Select(_) => {
                unreachable!("a selection crosses with its reader, minted by the target's insert")
            }
            VarDef::Pose(_) => unreachable!(
                "the carry declares variables slots read, and no slot reads a pose: a pose \
                 read crosses no cut (its reader's remap misses, `RemapMiss::Read`)"
            ),
        }
    }

    /// The fresh table an edit reading `reads` carries.
    fn table(&self, reads: impl IntoIterator<Item = VarId>) -> Carried {
        let anonymous = self.unheld(reads);
        let fresh = anonymous
            .iter()
            .map(|&var| self.decl(&anonymous, var).into())
            .collect();
        Carried { fresh, anonymous }
    }

    /// **`node` as the carried insert writes it**: every slot a reader
    /// re-pointed into the target ([`Self::reader`]), and its table.
    fn author(
        &self,
        node: &Node<ProfileProgram>,
        selects: &BTreeMap<VarId, crate::Operand>,
    ) -> (AuthoredNode, Carried) {
        let carried = self.table(node.exprs().into_iter().copied());
        let authored = node.authored_with(
            self.source,
            &mut |var, dim| self.reader(&carried.anonymous, var, dim),
            &mut |var| {
                selects
                    .get(&var)
                    .cloned()
                    .unwrap_or(crate::Operand::Var(var))
            },
        );
        (authored, carried)
    }

    /// The ids the carried edit's fresh table minted, entry by entry.
    fn minted(&mut self, carried: Carried, minted: &[VarId]) {
        assert_eq!(
            carried.anonymous.len(),
            minted.len(),
            "an accepted edit mints one variable per fresh entry"
        );
        self.map
            .extend(carried.anonymous.into_iter().zip(minted.iter().copied()));
    }

    /// **The named variable `var` declared in the target** as `name`:
    /// its definition re-pointed, an anonymous variable it reads
    /// carried in a fresh table — through a free declare of its kind,
    /// then the definition, since a declare carries no table.
    fn declare(
        &mut self,
        target: &mut Recording<'_, ProfileProgram>,
        var: VarId,
        name: VarName,
    ) -> Result<VarId, EditError> {
        let carried = self.table(self.source.definition_reads(var));
        let decl = self.decl(&carried.anonymous, var);
        let minted = if carried.anonymous.is_empty() {
            target.declare(name, decl)?
        } else {
            let minted = target.declare(name, VarDecl::Free(placeholder(decl.dim())))?;
            let record = target.apply_recorded(DocEdit::DefineVar {
                var: minted.into(),
                def: decl,
                fresh: carried.fresh.clone(),
            })?;
            self.minted(carried, &record.fresh);
            minted
        };
        self.map.insert(var, minted);
        Ok(minted)
    }
}

/// The dimension the source variable `var` is read at, where the
/// source holds it.
fn kind_of(source: &ProfileDoc, var: VarId) -> Option<Dimension> {
    source.var(var)?.kind().dimension()
}

/// A free value of `kind`, held only until the definition that
/// replaces it, in the same action.
fn placeholder(dim: Dimension) -> FreeVar {
    match dim {
        Dimension::Count => FreeVar::Count { value: 0 },
        dim => FreeVar::continuous(dim, 0.0),
    }
}

/// **The variables a split moves** (A4, the module doc's rule): each
/// a cut node reads, and each group of variables no node reaches —
/// tied by definition reads, either way — whose reads outside the
/// group all move. A group with no such read stays (an unread free
/// variable, VR7); one with reads on both sides refuses
/// [`SplitError::DefinitionStraddlesCut`]. A read of a variable the
/// document no longer holds stays.
fn moving_vars(
    doc: &ProfileDoc,
    cut_refs: &BTreeMap<VarId, RecipeNodeId>,
    kept_refs: &BTreeMap<VarId, RecipeNodeId>,
) -> Result<BTreeSet<VarId>, SplitError> {
    let order = doc.definition_order();
    let unreached: BTreeSet<VarId> = order
        .iter()
        .copied()
        .filter(|var| !cut_refs.contains_key(var) && !kept_refs.contains_key(var))
        .collect();
    let mut ties: BTreeMap<VarId, Vec<VarId>> = BTreeMap::new();
    for &var in &unreached {
        for read in doc.definition_reads(var) {
            if unreached.contains(&read) {
                ties.entry(var).or_default().push(read);
                ties.entry(read).or_default().push(var);
            }
        }
    }
    // Each unreached variable's group, labelled by its first member in
    // definition order.
    let mut group_of: BTreeMap<VarId, VarId> = BTreeMap::new();
    let mut seeds = Vec::new();
    for &seed in &order {
        if !unreached.contains(&seed) || group_of.contains_key(&seed) {
            continue;
        }
        group_of.insert(seed, seed);
        seeds.push(seed);
        let mut frontier = vec![seed];
        while let Some(var) = frontier.pop() {
            for &tied in ties.get(&var).into_iter().flatten() {
                if let btree_map::Entry::Vacant(slot) = group_of.entry(tied) {
                    slot.insert(seed);
                    frontier.push(tied);
                }
            }
        }
    }
    // One pass in definition order: each group's first read of a
    // moving variable and of a staying one, by the member that reads it.
    type Read = Option<(VarId, VarId)>;
    let mut sides: BTreeMap<VarId, (Read, Read)> = BTreeMap::new();
    for &member in &order {
        let Some(&seed) = group_of.get(&member) else {
            continue;
        };
        let (moves, stays) = sides.entry(seed).or_default();
        for read in doc.definition_reads(member) {
            if group_of.get(&read) == Some(&seed) {
                continue;
            }
            let side = if cut_refs.contains_key(&read) {
                &mut *moves
            } else {
                &mut *stays
            };
            side.get_or_insert((member, read));
        }
    }
    let mut moving: BTreeSet<VarId> = cut_refs.keys().copied().collect();
    let mut joining = BTreeSet::new();
    for seed in seeds {
        match sides.get(&seed).copied().unwrap_or_default() {
            (Some((_, moved)), Some((member, stayed))) => {
                return Err(SplitError::DefinitionStraddlesCut {
                    var: Box::new(doc.spoken_var(member)),
                    moving: doc.spoken_var(moved),
                    staying: doc.spoken_var(stayed),
                    staying_held: doc.var(stayed).is_some(),
                });
            }
            (Some(_), None) => {
                joining.insert(seed);
            }
            (None, _) => {}
        }
    }
    moving.extend(
        group_of
            .iter()
            .filter(|(_, seed)| joining.contains(*seed))
            .map(|(&var, _)| var),
    );
    Ok(moving)
}

// ---- Split ----

/// **What a remainder read of the cut is re-pointed to**: one copy of
/// the instance's copies, by the copy's key in the source document.
#[derive(Debug, Clone)]
enum SourceKey {
    /// The copy a cut `Place` makes of the shape it reads through this
    /// variable.
    Shape(VarId),
    /// A read of one copy of a cut placement's copies, keyed as the
    /// source keys it.
    Keyed(crate::CopyKey),
}

/// **The numbers `placement` is pinned to the world by**, when every
/// mate of its bundle targets the world (the pin
/// [`crate::mate::pin_at`] writes, or a plane mate and its values):
/// each world read's scalars and each value's, as slots. `None` for a
/// bundle with no mate or with a mate on another target.
fn world_pin_of<P>(doc: &Doc<P>, placement: RecipeNodeId) -> Option<Vec<(crate::SlotId, VarId)>> {
    use crate::mate::BundleRows;
    let node = doc.node(placement)?;
    let bundle = crate::mate::solve::bundle_of(node)?;
    let mut mates = bundle
        .iter()
        .filter_map(crate::mate::Constraint::mate)
        .peekable();
    mates.peek()?;
    if !mates.all(|mate| matches!(mate.to, crate::mate::Target::World(_))) {
        return None;
    }
    Some(
        bundle
            .to_vec()
            .rows()
            .into_iter()
            .map(|(slot, &var)| (slot, var))
            .collect(),
    )
}

/// **A resolver that serves one more part**: the part split is about
/// to write, beside whatever the caller's resolver serves.
#[derive(Debug)]
struct WithPart {
    part: (DocRef, ProfileDoc),
    rest: Option<std::sync::Arc<dyn PartResolver>>,
}

impl PartResolver for WithPart {
    fn resolve(&self, doc_ref: &DocRef, tol: Tol) -> Result<ProfileDoc, crate::ResolveFailure> {
        if *doc_ref == self.part.0 {
            return Ok(self.part.1.clone());
        }
        match &self.rest {
            Some(rest) => rest.resolve(doc_ref, tol),
            None => Err(crate::ResolveFailure::new(
                crate::ResolveFault::Unresolved,
                "no resolver serves a part other than the one split writes",
            )),
        }
    }
}

/// Cuts `cut` out of `doc` into a new document under `part_id` (the
/// caller's identity — [`DocumentId::derive`] for deterministic
/// callers, the document layer's `random_document_id` for interactive
/// authoring), leaving one [`Node::InstantiatePart`] of it behind,
/// pinned to the world at the identity. Semantics: module docs. Pure —
/// `doc` is untouched; undo is keeping it.
///
/// # Errors
///
/// Every arm of [`SplitError`] (each names its offending edge,
/// parameter, or name).
#[allow(clippy::too_many_lines)] // one linear pass per D-2 rule, each short
pub fn split(
    doc: &ProfileDoc,
    cut: &BTreeSet<RecipeNodeId>,
    part_id: DocumentId,
    tol: Tol,
    resolver: Option<&std::sync::Arc<dyn PartResolver>>,
) -> Result<SplitOutcome, SplitError> {
    if cut.is_empty() {
        return Err(SplitError::EmptyCut);
    }
    for &id in cut {
        if id == RecipeNodeId::WORLD {
            return Err(SplitError::CutsTheWorld);
        }
        if doc.node(id).is_none() {
            return Err(SplitError::UnknownCutNode {
                id: SpokenNode::absent(id),
            });
        }
    }
    // The new identity must be fresh: not the split document's own,
    // and not any document the cut references (a part pinning its own
    // id would be an evaluation cycle by A4's rule). KEPT-node
    // references to `part_id` pass this door by design: the remainder
    // is not the new document, so no cycle arises, and the collision
    // that does matter — two files claiming one id — is refused typed
    // at `Workspace::create`'s duplicate-id door.
    if part_id == doc.id() {
        return Err(SplitError::PartIdCollides { id: part_id });
    }
    for &id in cut {
        if let Some(Node::InstantiatePart { doc_ref, .. }) = doc.node(id)
            && doc_ref.id == part_id
        {
            return Err(SplitError::PartIdCollides { id: part_id });
        }
    }
    // Which copies are the world's: a part delivers only its world
    // (A10), so a cut copy in a loose space crosses nowhere. The solve
    // levers through the caller's seam, as the part's inserts do.
    let part_reach = crate::eval::PartReach::<f64>::with_resolver(resolver, tol);
    let opts = crate::eval::EvalOptions {
        resolver: resolver.cloned(),
        ..crate::eval::EvalOptions::default()
    };
    // A placement's space is the evaluation's: its value's where it
    // built, its recipe's where it did not (D10: kinds are decided from
    // the recipe, never from a value), so a world mate the solve cannot
    // reach still places its copy in the world.
    let evaluation =
        crate::eval::evaluate::<f64>(doc, None, &crate::CancelToken::new(), &opts, tol);
    let poses = crate::mate::SolvedPoses::of(doc, &evaluation);
    let world = |placement: RecipeNodeId| {
        doc.node(placement)
            .and_then(crate::mate::solve::bundle_of)
            .is_some()
            && evaluation.space(placement) == crate::mate::Space::World
    };
    // Whether a copy stands where its body did: its placement solved to
    // the identity of the world's coordinates, the fold's own verdict.
    let places_at_identity = |placement: RecipeNodeId| poses.at_identity(placement);
    // A cut name re-anchors as the part's product spells it: a name a
    // cut world placement mints (its copy's own, or an instance's) as
    // it is, and a name of a body a cut world `Place` copies under that
    // placement ([`RoleSeg::Placed`]). The copy holds the names its
    // body minted, those a transform below it carried whole (N1's
    // `Whole` edge), and a pattern copy's names where a part picks that
    // copy: the name's own `Instance` index against the pick's, at the
    // document's values, as the member walk judges a pick
    // (`mate::member`). That match is positional and interim: it
    // retires with `[ev]` #4341's `Member` keys. A split half's pick is
    // not followed, since which half holds a split's name is the
    // geometry's answer. A name of material no cut world placement
    // places, or two do, names nothing the instance carries.
    let env = doc.var_env::<f64>();
    // With `body`, the copy is the one a cut placement makes of that body
    // (a selection's): the placement copies it, or a `Part` that picks a
    // copy out of it (the gate's copy rule, `assembly::copy_face`), and
    // the walk follows the name from there to its minting node. The
    // answer is the placement, and whether the part's product wraps the
    // name under it.
    let home = |name: &StableName, body: Option<RecipeNodeId>| -> Option<(RecipeNodeId, bool)> {
        if cut.contains(&name.node)
            && body.is_none_or(|b| b == name.node)
            && matches!(
                doc.node(name.node),
                Some(Node::Place { .. } | Node::InstantiatePart { .. })
            )
        {
            return world(name.node).then_some((name.node, false));
        }
        let copy_of_name = match name.path.first() {
            Some(RoleSeg::Instance { i, .. }) => Some(i64::from(*i)),
            _ => None,
        };
        let holds = |placed: Option<RecipeNodeId>| -> bool {
            let mut picked: Option<i64> = None;
            let mut at = match (placed, body) {
                (placed, None) => placed,
                (Some(node), Some(body)) if node == body => Some(node),
                (Some(node), Some(body)) => {
                    match doc.node(node).and_then(crate::names::verbatim_edge) {
                        Some(crate::names::VerbatimEdge::Selected {
                            of,
                            select: crate::node::PartSelect::Instance(index),
                        }) if doc.operation_of(of) == Some(body) => {
                            picked = crate::expr::eval_var_count(*index, &env).ok();
                            picked.map(|_| body)
                        }
                        _ => None,
                    }
                }
                (None, Some(_)) => None,
            };
            while let Some(node) = at {
                if node == name.node {
                    return picked.is_none_or(|k| copy_of_name == Some(k));
                }
                at = match doc.node(node).and_then(crate::names::verbatim_edge) {
                    Some(crate::names::VerbatimEdge::Selected {
                        of,
                        select: crate::node::PartSelect::Instance(index),
                    }) if picked.is_none() => {
                        picked = crate::expr::eval_var_count(*index, &env).ok();
                        picked.and_then(|_| doc.operation_of(of))
                    }
                    Some(
                        crate::names::VerbatimEdge::Selected { .. }
                        | crate::names::VerbatimEdge::Intact,
                    )
                    | None => None,
                };
            }
            false
        };
        let mut placing = cut.iter().copied().filter(|&p| {
            world(p)
                && matches!(doc.node(p), Some(Node::Place { shapes, .. })
                    if shapes.iter().any(|s| !s.is_member() && holds(doc.operation_of(s.read))))
        });
        match (placing.next(), placing.next()) {
            (Some(placement), None) => Some((placement, true)),
            _ => None,
        }
    };
    // **The copy a remainder whole read of the cut re-points to**: at a
    // body seat, a read of one copy of a cut world placement, a read of
    // a cut `Place`'s one copy, or a read of a body one cut world
    // `Place` copies at the identity, where its copy stands. `None` for
    // a read that does not cross, which the closure rule refuses.
    let heir_of = |consumer: &Node<ProfileProgram>,
                   slot: crate::OperandSlot,
                   var: VarId|
     -> Result<Option<(RecipeNodeId, SourceKey)>, Uncarried> {
        let Some(read) = consumer.body_read(slot) else {
            return Ok(None);
        };
        let Some(at) = doc.operation_of(var).filter(|at| cut.contains(at)) else {
            return Ok(None);
        };
        match doc.node(at) {
            Some(Node::Place { .. } | Node::InstantiatePart { .. }) => {
                let key = match (&read.key, doc.node(at)) {
                    (Some(key), _) => SourceKey::Keyed(key.clone()),
                    (None, Some(Node::Place { shapes, .. }))
                        if !read.is_indexed()
                            && doc
                                .var(var)
                                .is_some_and(|v| v.kind() == crate::VarKind::Body) =>
                    {
                        let [shape] = shapes.as_slice() else {
                            return Ok(None);
                        };
                        SourceKey::Shape(shape.read)
                    }
                    _ => return Ok(None),
                };
                if !world(at) {
                    return Err(Uncarried::Loose {
                        placement: doc.spoken(at),
                    });
                }
                Ok(Some((at, key)))
            }
            _ if read.is_member() => Ok(None),
            _ => {
                let copies: Vec<RecipeNodeId> = doc
                    .ids()
                    .iter()
                    .copied()
                    .filter(|&p| {
                        cut.contains(&p)
                            && world(p)
                            && matches!(doc.node(p), Some(Node::Place { shapes, .. })
                                if shapes.iter().any(|s| !s.is_member() && s.read == var))
                    })
                    .collect();
                match copies.as_slice() {
                    [] => Ok(None),
                    [one] if places_at_identity(*one) => Ok(Some((*one, SourceKey::Shape(var)))),
                    [one] => Err(Uncarried::Posed {
                        placement: doc.spoken(*one),
                    }),
                    many => Err(Uncarried::Bodies { count: many.len() }),
                }
            }
        }
    };
    // D-2's closure rule, narrowed by the world (module docs): no cut
    // node reads the remainder, and a remainder read of the cut crosses
    // only where the part's world carries what it reads: a selection
    // each of whose names one cut world placement's copy holds, or a
    // whole read with its heir. Any other dependency refuses.
    let mut crossing_reads: Vec<(RecipeNodeId, crate::OperandSlot, VarId)> = Vec::new();
    // A pose read at a seat (a mate's side) crosses where every read of
    // it that reaches the cut is a selection written inside it whose
    // names the part's world carries: it is written afresh over the
    // instance's copies, those selections re-pointed (`crossing_leaves`).
    let mut crossing_poses: Vec<(RecipeNodeId, crate::OperandSlot, VarId)> = Vec::new();
    let mut crossing_leaves: Vec<(RecipeNodeId, VarId)> = Vec::new();
    let mut heirs: BTreeMap<(RecipeNodeId, crate::OperandSlot), (RecipeNodeId, SourceKey)> =
        BTreeMap::new();
    for consumer in doc.ids() {
        let Some(node) = doc.node(consumer) else {
            continue;
        };
        let consumer_is_cut = cut.contains(&consumer);
        let mut carried: BTreeSet<RecipeNodeId> = BTreeSet::new();
        if !consumer_is_cut {
            for (slot, var) in node.operand_rows() {
                if let Some(leaves) = pose_leaves(doc, var) {
                    let reaching: Vec<VarId> = leaves
                        .into_iter()
                        .filter(|&leaf| doc.read_ports(leaf).iter().any(|(at, _)| cut.contains(at)))
                        .collect();
                    let carries = |leaf: VarId| {
                        doc.var_name(leaf).is_none()
                            && doc.selection(leaf).is_some_and(|select| {
                                doc.operation_of(select.body).is_some_and(|at| {
                                    select
                                        .names
                                        .iter()
                                        .all(|name| home(name, Some(at)).is_some())
                                })
                            })
                    };
                    if !reaching.is_empty() && reaching.iter().all(|&leaf| carries(leaf)) {
                        crossing_poses.push((consumer, slot, var));
                        for leaf in reaching {
                            carried.extend(doc.read_operation(leaf));
                            crossing_leaves.push((consumer, leaf));
                        }
                    }
                    continue;
                }
                let (body, crosses) = match doc.selection(var) {
                    Some(select) => (
                        select.body,
                        doc.operation_of(select.body).is_some_and(|at| {
                            cut.contains(&at)
                                && select
                                    .names
                                    .iter()
                                    .all(|name| home(name, Some(at)).is_some())
                        }),
                    ),
                    None => match heir_of(node, slot, var) {
                        Ok(Some(heir)) => {
                            heirs.insert((consumer, slot), heir);
                            (var, true)
                        }
                        Ok(None) => (var, false),
                        Err(why) => {
                            return Err(SplitError::RemainderReadUncarried {
                                reader: doc.spoken(consumer),
                                why,
                            });
                        }
                    },
                };
                if crosses && let Some(input) = doc.operation_of(body) {
                    crossing_reads.push((consumer, slot, var));
                    carried.insert(input);
                }
            }
        }
        for input in doc.upstream_of(consumer, node) {
            if consumer_is_cut != cut.contains(&input) && !carried.contains(&input) {
                let slot = node.operand_rows().into_iter().find_map(|(slot, var)| {
                    doc.read_ports(var)
                        .iter()
                        .any(|&(at, _)| at == input)
                        .then_some(slot)
                });
                return Err(SplitError::SeveredEdge {
                    consumer: doc.spoken(consumer),
                    slot,
                    input: doc.spoken(input),
                    consumer_is_cut,
                });
            }
        }
    }
    // A selection names the copy it reads, and a cut copy's face read in
    // the instance stands where it did, the instance being pinned at the
    // identity. A body's face, read through the cut placement that
    // copies it, stands where the body did only where that copy is at
    // the identity.
    let selections = crossing_reads
        .iter()
        .map(|&(reader, _, var)| (reader, var))
        .chain(crossing_leaves.iter().copied());
    for (reader, var) in selections {
        let Some(select) = doc.selection(var) else {
            continue;
        };
        let body = doc.operation_of(select.body);
        for name in &select.names {
            if let Some((placement, true)) = home(name, body)
                && !places_at_identity(placement)
            {
                return Err(SplitError::RemainderReadUncarried {
                    reader: doc.spoken(reader),
                    why: Uncarried::Posed {
                        placement: doc.spoken(placement),
                    },
                });
            }
        }
    }
    // A value is charted on its mate's target body's own frame: a kept
    // mate whose values read a cut copy keeps their meaning across the
    // seam only where that copy stands at the identity, since the
    // instance's frame is the part's world.
    for &(reader, slot, _) in &crossing_poses {
        let crate::OperandSlot::Side {
            constraint,
            side: crate::mate::MateSide::To,
        } = slot
        else {
            continue;
        };
        let valued = doc
            .node(reader)
            .and_then(crate::mate::solve::bundle_of)
            .is_some_and(|bundle| {
                bundle
                    .iter()
                    .any(|c| c.value().is_some_and(|v| v.mate == constraint))
            });
        if !valued {
            continue;
        }
        for &(_, leaf) in crossing_leaves.iter().filter(|&&(at, _)| at == reader) {
            let Some(select) = doc.selection(leaf) else {
                continue;
            };
            let body = doc.operation_of(select.body);
            for name in &select.names {
                if let Some((placement, _)) = home(name, body)
                    && !places_at_identity(placement)
                {
                    return Err(SplitError::RemainderReadUncarried {
                        reader: doc.spoken(reader),
                        why: Uncarried::Posed {
                            placement: doc.spoken(placement),
                        },
                    });
                }
            }
        }
    }
    // Every walk over the cut below reads it in document order, so the
    // node a refusal names is the one the author placed first.
    let in_order: Vec<RecipeNodeId> = doc
        .ids()
        .iter()
        .copied()
        .filter(|id| cut.contains(id))
        .collect();
    // Variables go where their readers go: read by cut nodes → moved
    // into the part; read by BOTH sides → refused (no silent sharing).
    let mut cut_refs: BTreeMap<VarId, RecipeNodeId> = BTreeMap::new();
    let mut kept_refs: BTreeMap<VarId, RecipeNodeId> = BTreeMap::new();
    for id in doc.ids() {
        let Some(node) = doc.node(id) else { continue };
        let into = if cut.contains(&id) {
            &mut cut_refs
        } else {
            &mut kept_refs
        };
        for var in node_var_reads(doc, node) {
            into.entry(var).or_insert(id);
        }
    }
    for (&var, &cut_node) in &cut_refs {
        if let Some(&kept_node) = kept_refs.get(&var) {
            return Err(SplitError::UncutVarReference {
                var: Box::new(doc.spoken_var(var)),
                cut_node: doc.spoken(cut_node),
                kept_node: doc.spoken(kept_node),
            });
        }
    }
    let moving = moving_vars(doc, &cut_refs, &kept_refs)?;
    // Each selection's readers, by node.
    let mut select_readers: BTreeMap<VarId, Vec<RecipeNodeId>> = BTreeMap::new();
    for id in doc.ids() {
        for (_, var) in doc.node(id).map(Node::operand_rows).unwrap_or_default() {
            if doc.selection(var).is_some() {
                select_readers.entry(var).or_default().push(id);
            }
        }
    }
    // Cut-side name references must lie wholly within the cut: the
    // part document cannot name the remainder's entities. Read off
    // the document's name-carrier enumeration, so a carrier added to
    // `Carrier` is walked here without being remembered into this
    // site — only its SIDE has to be decided, which is what the two
    // arms below say.
    for carrier in doc.name_carriers() {
        match carrier {
            NameCarrier::Payload { node, name } => {
                if !cut.contains(&node) {
                    continue;
                }
                let outside = derivation_nodes(name)
                    .into_iter()
                    .filter(|id| !cut.contains(id))
                    .min_by_key(|id| (doc.node(*id).is_none(), *id));
                if let Some(missing) = outside {
                    return Err(SplitError::PartNameReachesRemainder {
                        node: doc.spoken(node),
                        name: doc.spoken_name(name),
                        missing: doc.spoken(missing),
                    });
                }
            }
            // A selection is on the side of its readers: one a cut node
            // reads is the part's, and may not name the remainder.
            NameCarrier::Select { var, name } => {
                let Some(&reader) = select_readers
                    .get(&var)
                    .and_then(|readers| readers.iter().find(|reader| cut.contains(reader)))
                else {
                    continue;
                };
                let outside = derivation_nodes(name)
                    .into_iter()
                    .filter(|id| !cut.contains(id))
                    .min_by_key(|id| (doc.node(*id).is_none(), *id));
                if let Some(missing) = outside {
                    return Err(SplitError::PartNameReachesRemainder {
                        node: doc.spoken(reader),
                        name: doc.spoken_name(name),
                        missing: doc.spoken(missing),
                    });
                }
            }
            // A store key is the document's, not either side's: no
            // node carries it, so there is no cut-side instance of
            // one to refuse. It is classified below instead, where
            // the remainder's references are.
            NameCarrier::Store { .. } => {}
        }
    }
    // Remainder-side references to cut entities re-anchor through the
    // instance qualifier (module docs); collect them, refusing the
    // inexpressible cases typed.
    let mut rebinds: BTreeSet<StableName> = BTreeSet::new();
    let mut classify = |name: &StableName| -> Result<(), SplitError> {
        let ids = derivation_nodes(name);
        if ids.iter().all(|id| !cut.contains(id)) {
            return Ok(());
        }
        if !ids.is_subset(cut) {
            return Err(SplitError::NameStraddlesCut {
                name: doc.spoken_name(name),
                // The classification weighs the whole derivation set:
                // it is the SPLIT of that set across the cut that
                // refuses, so no one node is the culprit.
                missing: None,
            });
        }
        if name.kind == crate::names::EntityKind::Body {
            return Err(SplitError::BodyNameCrossesCut {
                name: doc.spoken_name(name),
            });
        }
        rebinds.insert(name.clone());
        Ok(())
    };
    // Every name the document holds that is not carried by a cut
    // node: the payload names of the kept nodes, then the store's
    // keys, which no node carries and which therefore always
    // classify.
    for carrier in doc.name_carriers() {
        match carrier {
            NameCarrier::Payload { node, name } => {
                if cut.contains(&node) {
                    continue;
                }
                classify(name)?;
            }
            // A remainder selection that crosses is authored afresh on
            // the instance below; any other is classified as a payload
            // name is.
            NameCarrier::Select { var, name } => {
                let remainder_read = select_readers
                    .get(&var)
                    .is_some_and(|readers| readers.iter().any(|r| !cut.contains(r)));
                let crossing = crossing_reads.iter().any(|&(_, _, read)| read == var);
                if remainder_read && !crossing {
                    classify(name)?;
                }
            }
            NameCarrier::Store { name, .. } => classify(name)?,
        }
    }
    // ---- The part document, as recorded edits from empty ----
    let empty = Doc::empty(part_id, tol);
    let mut part = Recording::start(&empty, tol, &part_reach);
    let part_refused = |error: EditError| SplitError::PartEdit {
        error: Box::new(error),
    };
    let part_apply =
        |part: &mut Recording<'_, ProfileProgram>,
         edit: DocEdit<ProfileProgram>|
         -> Result<(), SplitError> { part.apply(edit).map(|_| ()).map_err(part_refused) };
    // The recorded ε carries over iff it differs from what the empty
    // document adopts (the committed process ε — the only value a
    // document this process can evaluate records anyway).
    if doc.epsilon().to_bits() != part.doc().epsilon().to_bits() {
        part_apply(&mut part, DocEdit::SetTolerance { eps: doc.epsilon() })?;
    }
    // A cut reader of a variable the parent no longer holds has
    // nothing to be re-pointed at, and refuses here, at this door.
    if let Some((&id, &node)) = cut_refs.iter().find(|(id, _)| doc.var(**id).is_none()) {
        return Err(SplitError::UnresolvedVarCrossesCut {
            var: doc.spoken_var(id),
            node: doc.spoken(node),
        });
    }
    // The named variables that move, declared in the PARENT's
    // definition order — its declaration order, a definition after what
    // it reads — so the part lists them as the parent's author did. An
    // anonymous one crosses with the first carried edit that reads it
    // (`VarCarry`).
    let mut vars = VarCarry::new(doc);
    for id in doc
        .definition_order()
        .into_iter()
        .filter(|id| moving.contains(id))
    {
        if let Some(name) = doc.var_name(id) {
            vars.declare(&mut part, id, name.clone())
                .map_err(part_refused)?;
        }
    }
    // The cut nodes in document order, each under the id the part's
    // insert door mints for it (D9 — two runs agree byte for byte).
    let (node_map, step_map, read_map) = carry(
        doc,
        &in_order,
        &mut part,
        &mut vars,
        false,
        |_, _, _| true,
        part_refused,
        |old, miss| match miss {
            RemapMiss::Input(input) => SplitError::PartEdit {
                error: Box::new(EditError::UnresolvedInput {
                    input: doc.spoken(input),
                }),
            },
            RemapMiss::Read { slot, var, .. } => SplitError::PartEdit {
                error: Box::new(EditError::OperandUnresolved {
                    node: doc.spoken(old),
                    slot: crate::SlotId::Operand(slot),
                    read: crate::Operand::Var(var),
                }),
            },
            RemapMiss::PayloadRead { var } => SplitError::PartEdit {
                error: Box::new(EditError::PayloadUnresolvedVar {
                    var: doc.spoken_var(var),
                    node: doc.spoken(old),
                }),
            },
            RemapMiss::Name { name, missing } => SplitError::reaches(doc, old, &name, missing),
        },
    )?;
    // A cut that places nothing in the world leaves an instance of
    // nothing, since a part delivers only its world: asked once the cut
    // has carried, so a cut whose names cannot cross is told that first.
    if !in_order.iter().any(|&id| {
        matches!(
            doc.node(id),
            Some(Node::Place { .. } | Node::InstantiatePart { .. })
        ) && world(id)
    }) && let Some(&first) = in_order.first()
    {
        return Err(SplitError::NoMaterial {
            node: doc.spoken(first),
        });
    }
    // Witness DATA copies VERBATIM while node ids remap: sound because
    // a witness datum is sketch-self-relative — it selects among the
    // owning profile's own solution branches and embeds no other
    // node's identity, so there is no cross-id-space reference for the
    // remap to miss. A future witness vocabulary that embeds foreign
    // stable names must remap here or refuse.
    for &old in &in_order {
        if let (Some(&new), Some(witness)) = (node_map.get(&old), doc.witness(old)) {
            part_apply(
                &mut part,
                DocEdit::ReWitness {
                    node: new,
                    witness: witness.clone(),
                },
            )?;
        }
    }
    let pin = content_pin(part.doc(), tol).map_err(|error| SplitError::Pin {
        error: Box::new(error),
    })?;
    // A cut name as the part's product spells it ([`home`] above).
    let in_world =
        |name: &StableName, body: Option<RecipeNodeId>| -> Result<StableName, SplitError> {
            let of = remap_name(name, &node_map, &step_map, &read_map)
                .map_err(|missing| SplitError::straddles(doc, name, missing))?;
            let (placement, wrapped) =
                home(name, body).ok_or_else(|| SplitError::NameOutsidePartWorld {
                    name: doc.spoken_name(name),
                })?;
            if !wrapped {
                return Ok(of);
            }
            let Some(&placement) = node_map.get(&placement) else {
                unreachable!("a cut placement is carried")
            };
            Ok(StableName {
                kind: name.kind,
                node: placement,
                path: vec![RoleSeg::Placed {
                    of: NameRef::new(of),
                }],
            })
        };
    // The key, in the part, of the copy a crossing whole read reads.
    let part_key = |placement: RecipeNodeId, key: &SourceKey| -> crate::CopyKey {
        let Some(&carried) = node_map.get(&placement) else {
            unreachable!("a cut placement is carried")
        };
        let read = |var: VarId| {
            crate::CopyKey::Read(
                read_map
                    .get(&var)
                    .copied()
                    .unwrap_or_else(|| unreachable!("a cut placement's shape is carried")),
            )
        };
        let within = match (key, doc.node(placement)) {
            (SourceKey::Shape(var), _) => read(*var),
            (SourceKey::Keyed(crate::CopyKey::Read(var)), Some(Node::Place { .. })) => read(*var),
            // An instance's keys are its part's, which crosses whole.
            (SourceKey::Keyed(key), _) => key.clone(),
        };
        crate::CopyKey::part(carried, within)
    };

    // ---- The interface record (ASM-R2b D-4; A4's seam) ----
    //
    // INVARIANT: a constraint CROSSES iff its two face reads land on
    // opposite sides of the cut. One with both ends inside is
    // part-internal (its names rebind wholesale and it declares nothing
    // about the seam); one with both ends outside never touched the
    // cut. Collected in the pre-split document's node order, then
    // bundle order, which is what makes the record D9-deterministic.
    //
    // **Only a constraint whose two face reads resolve can cross**
    // (AQ8): both sides resolve to live MEMBERS of A11's vocabulary — a
    // live instance, or a pattern-placed instance. The gate is
    // `crate::mate::member_of` ITSELF, not a re-spelling of it. A kept
    // constraint reading a cut copy is a kept placement pinned against
    // the part (FORK-S3M), and its cut side reads the instance's copy
    // once split. A constraint with a DANGLING side contributes NO
    // crossing, however its names fall across the cut: that is AQ8
    // option (b), SKIP — its home is `crates/editor-core/ASSEMBLY.md`'s
    // AQ8 clause.
    let mut crossings: Vec<InterfaceCrossing> = Vec::new();
    for id in doc.ids() {
        if cut.contains(&id) {
            continue;
        }
        let Some(bundle) = doc.node(id).and_then(crate::mate::solve::bundle_of) else {
            continue;
        };
        for constraint in bundle {
            let Some(mate) = constraint.mate() else {
                continue;
            };
            let class = &mate.class;
            let Some((a, b)) = crate::mate::rested_faces(doc, mate) else {
                continue;
            };
            let (a, b) = (&a, &b);
            // The resolve gate, before the sides are even looked at.
            let (Some(a), Some(b)) = (
                crate::mate::member::head_of(doc, *a),
                crate::mate::member::head_of(doc, *b),
            ) else {
                continue;
            };
            if crate::mate::member_of(doc, &a).is_none()
                || crate::mate::member_of(doc, &b).is_none()
            {
                continue;
            }
            let inside = |name: &StableName| derivation_nodes(name).is_subset(cut);
            let (outer, inner) = match (inside(&a.name), inside(&b.name)) {
                (false, true) => (&a.name, &b.name),
                (true, false) => (&b.name, &a.name),
                _ => continue,
            };
            // The part-side reference is stored in the PART's own names:
            // that is what the part's product answers to, and what
            // re-verification resolves against. `classify` above already
            // refused a name that straddles, so the remap is total here —
            // and it refuses typed rather than assuming so.
            let Ok(inner) = FaceName::new(in_world(inner, None)?) else {
                unreachable!("a remap keeps a face a face")
            };
            crossings.push(InterfaceCrossing::Mate {
                class: *class,
                outer: outer.clone(),
                inner,
            });
        }
    }

    // ---- The remainder, as recorded edits from the input ----
    // The remainder inserts one instance pinned to the world, rebinds,
    // re-points and deletes: no edit of it levers a pair.
    let rem_reach = crate::mate::RefusingReach;
    let mut remainder = Recording::start(doc, tol, &rem_reach);
    let rem_refused = |error| SplitError::RemainderEdit {
        error: Box::new(error),
    };
    let rem_apply = |remainder: &mut Recording<'_, ProfileProgram>,
                     edit: DocEdit<ProfileProgram>|
     -> Result<(), SplitError> {
        remainder.apply(edit).map(|_| ()).map_err(rem_refused)
    };
    // The instance is a placement (D10), standing where its part's
    // world stands ([`crate::mate::stand`]), so the part's world copies
    // stand where the cut's did. The part is read as it is about to be
    // written, beside whatever the caller's resolver serves.
    let doc_ref = DocRef { id: part_id, pin };
    let instance = remainder
        .insert(Node::instantiate_part_with(
            doc_ref,
            InterfaceRecord { crossings },
        ))
        .map_err(rem_refused)?;
    let with_part = crate::eval::EvalOptions {
        resolver: Some(std::sync::Arc::new(WithPart {
            part: (doc_ref, part.doc().clone()),
            rest: resolver.cloned(),
        })),
        ..crate::eval::EvalOptions::default()
    };
    let stood = remainder
        .doc()
        .output(instance, 0)
        .ok_or(SplitError::InstanceUnstood)
        .and_then(|copies| {
            crate::mate::stand(remainder.doc(), &crate::Operand::Var(copies), &with_part, tol)
                .map_err(|_| SplitError::InstanceUnstood)
        })?;
    let mate = remainder
        .apply_recorded(DocEdit::AddConstraint {
            placement: instance,
            mate: Box::new(stood.mate),
            fresh: Vec::new(),
        })
        .map_err(rem_refused)?
        .minted_constraints
        .first()
        .copied()
        .unwrap_or_else(|| unreachable!("a mate mints its id"));
    for (coord, by) in stood.values {
        rem_apply(
            &mut remainder,
            DocEdit::AddValue {
                placement: instance,
                value: Box::new(crate::mate::Value { mate, coord, by }),
                fresh: Vec::new(),
            },
        )?;
    }
    let copies = remainder
        .doc()
        .output(instance, 0)
        .unwrap_or_else(|| unreachable!("an instance defines its copies"));
    let in_part =
        |from: &StableName, body: Option<RecipeNodeId>| -> Result<StableName, SplitError> {
            Ok(StableName {
                kind: from.kind,
                node: instance,
                path: vec![RoleSeg::InPart {
                    of: NameRef::new(in_world(from, body)?),
                }],
            })
        };
    // A selection of cut material is authored afresh in the instance's
    // copies, each name read through it, once per selection: its first
    // reader authors it, and a named one keeps its name on the new
    // variable, which every other reader then reads. A whole read reads
    // the one copy that stands where its body did, by its key in the
    // part.
    let mut reauthored: BTreeMap<VarId, VarId> = BTreeMap::new();
    for &(node, slot, var) in &crossing_reads {
        let value = match (doc.selection(var), reauthored.get(&var)) {
            (Some(_), Some(&fresh)) => crate::SlotValue::Read(crate::Operand::Var(fresh)),
            (Some(select), None) => crate::SlotValue::Read(crate::Operand::select(
                crate::Operand::Var(copies),
                crate::var::Select::canonical(
                    selection_kind(doc, var),
                    select
                        .names
                        .iter()
                        .map(|name| in_part(name, doc.operation_of(select.body)))
                        .collect::<Result<Vec<_>, _>>()?,
                ),
            )),
            (None, _) => {
                let Some((placement, key)) = heirs.get(&(node, slot)) else {
                    unreachable!("a crossing whole read has its heir")
                };
                crate::SlotValue::Copy {
                    of: crate::Operand::Var(copies),
                    key: part_key(*placement, key),
                }
            }
        };
        rem_apply(
            &mut remainder,
            DocEdit::SetParam {
                node,
                slot: crate::SlotId::Operand(slot),
                value,
                fresh: Vec::new(),
            },
        )?;
        if doc.selection(var).is_some() && !reauthored.contains_key(&var) {
            let fresh = read_at(remainder.doc(), node, slot);
            reauthored.insert(var, fresh);
            if let Some(name) = doc.var_name(var) {
                rem_apply(&mut remainder, DocEdit::DeleteVar { var: var.into() })?;
                rem_apply(
                    &mut remainder,
                    DocEdit::RenameVar {
                        var: fresh.into(),
                        name: Some(name.clone()),
                    },
                )?;
            }
        }
    }
    // A pose read that crosses is written afresh, each selection in it
    // that reads the cut authored in the instance's copies as a crossing
    // selection is.
    for &(node, slot, var) in &crossing_poses {
        let mut leaf = |leaf: VarId| -> Result<Option<crate::Operand>, SplitError> {
            if !crossing_leaves
                .iter()
                .any(|&(at, read)| at == node && read == leaf)
            {
                return Ok(None);
            }
            let Some(select) = doc.selection(leaf) else {
                return Ok(None);
            };
            Ok(Some(crate::Operand::select(
                crate::Operand::Var(copies),
                crate::var::Select::canonical(
                    selection_kind(doc, leaf),
                    select
                        .names
                        .iter()
                        .map(|name| in_part(name, doc.operation_of(select.body)))
                        .collect::<Result<Vec<_>, _>>()?,
                ),
            )))
        };
        let value = crate::SlotValue::Read(rewritten_pose(doc, var, &mut leaf)?);
        rem_apply(
            &mut remainder,
            DocEdit::SetParam {
                node,
                slot: crate::SlotId::Operand(slot),
                value,
                fresh: Vec::new(),
            },
        )?;
    }
    // An appearance key read in a cut body is re-attached in the
    // instance's body, its name read through it as a selection's is: a
    // Rebind keeps a key's read, and the cut takes that read with it.
    for (key, record) in remainder.doc().appearance().clone() {
        let Some(at) = remainder
            .doc()
            .operation_of(key.at)
            .filter(|at| cut.contains(at))
        else {
            continue;
        };
        let onto = crate::AppearanceKey::new(copies, in_part(&key.name, Some(at))?);
        reattach_appearance(
            &mut |edit| rem_apply(&mut remainder, edit),
            &key,
            &record,
            &onto,
        )?;
    }
    for from in &rebinds {
        let to = in_part(from, None)?;
        // A repair is addressed by body: one per remainder selection
        // body naming `from`, and one for the carriers no body holds.
        let mut bodies: BTreeSet<VarId> = BTreeSet::new();
        let mut unbodied = false;
        for carrier in remainder.doc().name_carriers() {
            if carrier.name() != from {
                continue;
            }
            match carrier {
                NameCarrier::Select { var, .. } => {
                    if let Some(select) = remainder.doc().selection(var) {
                        bodies.insert(select.body);
                    }
                }
                NameCarrier::Payload { .. } | NameCarrier::Store { .. } => unbodied = true,
            }
        }
        for body in bodies.into_iter().map(Some).chain(unbodied.then_some(None)) {
            rem_apply(
                &mut remainder,
                DocEdit::Rebind {
                    body,
                    from: from.clone(),
                    to: to.clone(),
                },
            )?;
        }
    }
    // Reverse document order deletes consumers before their inputs, so
    // no delete dangles a live reference — except past a union
    // `SetMembers` pointed forward
    // (`work/doors/a-member-set-after-its-union-points-forward-so-save-and-cascade-delete-break.md`).
    for &old in doc.ids().iter().rev() {
        if cut.contains(&old) {
            rem_apply(&mut remainder, DocEdit::DeleteNode { id: old })?;
        }
    }
    // A moved named variable leaves with its readers; an anonymous one
    // already left with its last. The door takes them in any order
    // (VR7 leaves a reader unresolved).
    for &var in doc.definition_order().iter().rev() {
        if moving.contains(&var) && doc.var_name(var).is_some() {
            rem_apply(&mut remainder, DocEdit::DeleteVar { var: var.into() })?;
        }
    }
    let Recorded {
        doc: remainder,
        edits: remainder_edits,
        maintenance: remainder_maintenance,
        ..
    } = remainder.finish().map_err(rem_refused)?;
    let Recorded {
        doc: part,
        edits: part_edits,
        maintenance: part_maintenance,
        ..
    } = part.finish().map_err(part_refused)?;
    Ok(SplitOutcome {
        remainder,
        part,
        remainder_edits,
        remainder_maintenance,
        part_edits,
        part_maintenance,
        instance,
        node_map,
        step_map,
    })
}

/// The kind of the selection `var` holds in `doc`.
///
/// # Panics
///
/// If `doc` holds no variable `var`: every caller asks of a variable it
/// just read as a selection.
/// **The reads a pose read at a seat is written over**: `None` unless
/// `var` is a pose written in place (an anonymous definition); else
/// every read inside it that is not itself such a pose, in read order.
fn pose_leaves<P>(doc: &Doc<P>, var: VarId) -> Option<Vec<VarId>> {
    fn walk<P>(doc: &Doc<P>, var: VarId, out: &mut Vec<VarId>) -> bool {
        if doc.var_name(var).is_some() {
            return false;
        }
        let Some(crate::var::VarDef::Pose(def)) = doc.var(var).map(crate::Var::def) else {
            return false;
        };
        for (_, &read, _, _) in def.reads() {
            if !walk(doc, read, out) {
                out.push(read);
            }
        }
        true
    }
    let mut out = Vec::new();
    walk(doc, var, &mut out).then_some(out)
}

/// **The first construction a placement's target read reads**: a node
/// that is no placement and not the world, read through `var`'s
/// definition other than through a placement's carried copy. `None`
/// where the target reads only copies and the world.
fn construction_read<P>(doc: &Doc<P>, var: VarId) -> Option<RecipeNodeId> {
    let placed = |node: RecipeNodeId| {
        matches!(
            doc.node(node),
            Some(Node::Place { .. } | Node::InstantiatePart { .. } | Node::World)
        )
    };
    match doc.var(var).map(crate::Var::def)? {
        crate::var::VarDef::Pose(crate::pose::PoseDef::Carried { copy, .. }) => {
            doc.read_operation(*copy).filter(|&node| !placed(node))
        }
        crate::var::VarDef::Pose(def) => def
            .reads()
            .into_iter()
            .find_map(|(_, &read, _, _)| construction_read(doc, read)),
        _ => doc.read_operation(var).filter(|&node| !placed(node)),
    }
}

/// **`var` as it was written, with some reads replaced**: each read
/// `leaf` answers for is that answer; each pose written in place is
/// written again over its reads; any other read as
/// [`Doc::written_operand`] spells it.
fn rewritten_pose<P, E>(
    doc: &Doc<P>,
    var: VarId,
    leaf: &mut impl FnMut(VarId) -> Result<Option<crate::Operand>, E>,
) -> Result<crate::Operand, E> {
    rewritten_pose_with(doc, var, leaf, &mut |formula| Ok(formula))
}

/// [`rewritten_pose`] with each scalar's written formula mapped by
/// `scalar` — a carry's, re-pointing the named variables it reads.
fn rewritten_pose_with<P, E>(
    doc: &Doc<P>,
    var: VarId,
    leaf: &mut impl FnMut(VarId) -> Result<Option<crate::Operand>, E>,
    scalar: &mut impl FnMut(Formula) -> Result<Formula, E>,
) -> Result<crate::Operand, E> {
    if let Some(operand) = leaf(var)? {
        return Ok(operand);
    }
    match doc.var(var).map(crate::Var::def) {
        Some(crate::var::VarDef::Pose(def)) if doc.var_name(var).is_none() => {
            // `try_map` calls the two closures one after the other, never
            // together, so the one `scalar` is lent to each in turn.
            let scalar = std::cell::RefCell::new(scalar);
            let mut read =
                |_, &r: &VarId| rewritten_pose_with(doc, r, leaf, &mut **scalar.borrow_mut());
            let written = def.try_map(&mut read, &mut |slot, &s| {
                (scalar.borrow_mut())(Formula::from(
                    doc.written(&crate::Expr::var(s, slot.dimension())),
                ))
            });
            Ok(crate::Operand::Pose(Box::new(written?)))
        }
        _ => Ok(doc.written_operand(var)),
    }
}

fn selection_kind<P>(doc: &Doc<P>, var: VarId) -> crate::VarKind {
    let Some(held) = doc.var(var) else {
        unreachable!("a selection the document holds is a variable it holds")
    };
    held.kind()
}

/// The variable `node` reads at `slot` in `doc`.
///
/// # Panics
///
/// If `node` holds no read at `slot`: every caller asks right after
/// writing it.
fn read_at<P: crate::ProfilePayload>(
    doc: &Doc<P>,
    node: RecipeNodeId,
    slot: crate::OperandSlot,
) -> VarId {
    let Some(var) = doc
        .node(node)
        .and_then(|n| n.operand_rows().into_iter().find(|(at, _)| *at == slot))
        .map(|(_, var)| var)
    else {
        unreachable!("the read was just written")
    };
    var
}
// ---- Inline ----

/// **How an inlined part's placements land in the host** (A4).
enum Landing {
    /// The instance was pinned to the world, solved to this motion
    /// (`None` the identity): each part constraint on the world is moved
    /// by it, and every other lands as it is.
    Pinned(Option<geom_core::linalg::Affine3<f64>>),
    /// The instance was placed by other constraints: the part's one
    /// placement on the world, pinned there at the identity and itself
    /// an instance, takes the instance's bundle in place of its own pin.
    Root(RecipeNodeId),
}

/// Splices the document `instance` references into `doc` and deletes
/// the instance — the inverse of [`split`] (D-3). The pin resolves
/// through `resolver` (a stale pin is the resolver's typed
/// `PinMismatch`, never a silent retarget); ids remap into fresh host
/// mints in the part's document order; `InPart`-wrapped names at the
/// instance re-anchor to the spliced local names; the part's
/// placements land where the instance put them ([`Landing`]), and no
/// frame is computed (A4). Pure — `doc` is untouched; undo is keeping
/// it.
///
/// The instance's INTERFACE RECORD dissolves here (ASM-R2b D-4, the
/// inverse of split's populate): each crossing's part-side reference
/// is re-anchored to the local name the splice minted, which is
/// exactly what the wrapped-name rebind below does to the constraint
/// that declared it — so once every crossing's inner name is confirmed
/// to land locally, the record has no remaining content and goes with
/// the deleted instance. Confirmed, not assumed: a crossing that does
/// not re-anchor refuses [`InlineError::StrandedPartName`] rather than
/// being dropped with the node.
///
/// # Errors
///
/// Every arm of [`InlineError`].
#[allow(clippy::too_many_lines)] // one linear pass per D-3 rule, each short
pub fn inline(
    doc: &ProfileDoc,
    instance: RecipeNodeId,
    resolver: &std::sync::Arc<dyn PartResolver>,
    tol: Tol,
) -> Result<InlineOutcome, InlineError> {
    let reach = crate::eval::PartReach::<f64>::with_resolver(Some(resolver), tol);
    let Some(node) = doc.node(instance) else {
        return Err(InlineError::UnknownNode {
            id: SpokenNode::absent(instance),
        });
    };
    let Node::InstantiatePart {
        doc_ref,
        interface,
        constraints: bundle,
    } = node
    else {
        return Err(InlineError::NotAnInstance {
            node: doc.spoken(instance),
        });
    };
    let part = resolver
        .as_ref()
        .resolve(doc_ref, tol)
        .map_err(|failure| InlineError::Unresolved { failure })?;
    if part.epsilon().to_bits() != doc.epsilon().to_bits() {
        return Err(InlineError::EpsilonSeam {
            host_eps: doc.epsilon(),
            part_eps: part.epsilon(),
        });
    }
    if let Some(key) = part.metadata().keys().next() {
        return Err(InlineError::PartCarriesMetadata { key: key.clone() });
    }
    // A spliced reader of a variable the part no longer holds has
    // nothing to be re-pointed at, and refuses here, at this door, before
    // anything asks the part's evaluation, which it leaves without one.
    for id in part.ids() {
        let Some(node) = part.node(id) else { continue };
        if let Some(var) = node_var_reads(&part, node)
            .into_iter()
            .find(|&var| part.var(var).is_none())
        {
            return Err(InlineError::UnresolvedVarCrossesCut {
                var: part.spoken_var(var),
                node: part.spoken(id),
            });
        }
    }
    // The part's world copies: what the instance's copies are.
    let opts = crate::eval::EvalOptions {
        resolver: Some(std::sync::Arc::clone(resolver)),
        ..crate::eval::EvalOptions::default()
    };
    let part_poses = crate::mate::solve_document(&part, &opts, tol);
    // Whether a part copy stands where its body did: its placement
    // solved to the identity of the part's world, decided at the band.
    let places_at_identity = |placement: RecipeNodeId| part_poses.at_identity(placement);
    let part_world: Vec<RecipeNodeId> = part
        .placements()
        .into_iter()
        .filter(|&p| part_poses.space(p) == Some(crate::mate::Space::World))
        .collect();
    let reads_world = |p: RecipeNodeId| {
        part.node(p)
            .and_then(crate::mate::solve::bundle_of)
            .is_some_and(|bundle| bundle.iter().any(crate::mate::Constraint::reads_world))
    };
    // Where the part's placements land (A4). A pin's numbers are the
    // motion it solved to: one that reads a variable would lose it once
    // the part's world mates carry the motion, and refuses.
    let landing = match world_pin_of(doc, instance) {
        Some(numbers) => {
            for (slot, scalar) in numbers {
                let written = doc.written(&crate::expr::Expr::var(scalar, slot.expr_dimension()));
                let (mut ids, mut names) = (Vec::new(), Vec::new());
                written.var_reads(&mut ids);
                written.named_reads(&mut names);
                if let Some(&(var, _)) = ids.first() {
                    return Err(InlineError::PinReadsVariable {
                        instance: doc.spoken(instance),
                        var: Box::new(doc.spoken_var(var)),
                    });
                }
                debug_assert!(names.is_empty(), "a stored formula reads by id");
            }
            let host = crate::mate::solve_document(doc, &opts, tol);
            if host.at_identity(instance) {
                Landing::Pinned(None)
            } else if host.space(instance) == Some(crate::mate::Space::World)
                && let Some(motion) = host.relative_map(instance)
            {
                Landing::Pinned(Some(motion))
            } else {
                return Err(InlineError::InstanceUnsolved {
                    instance: doc.spoken(instance),
                });
            }
        }
        None => {
            let on_world: Vec<RecipeNodeId> = part
                .placements()
                .into_iter()
                .filter(|&p| reads_world(p))
                .collect();
            match on_world.as_slice() {
                [root]
                    if places_at_identity(*root)
                        && matches!(part.node(*root), Some(Node::InstantiatePart { .. })) =>
                {
                    Landing::Root(*root)
                }
                [root] => {
                    return Err(InlineError::UnplaceablePart {
                        instance: doc.spoken(instance),
                        why: if places_at_identity(*root) {
                            Uncarried::NotAnInstance {
                                placement: part.spoken(*root),
                            }
                        } else {
                            Uncarried::Posed {
                                placement: part.spoken(*root),
                            }
                        },
                    });
                }
                many => {
                    return Err(InlineError::UnplaceablePart {
                        instance: doc.spoken(instance),
                        why: Uncarried::Bodies { count: many.len() },
                    });
                }
            }
        }
    };
    // At a pose of its own, the part's world moves and its constructions
    // do not (D's interim: a construction is of the document's one
    // kind), so a construction reading a copy would read it moved.
    if let Landing::Pinned(Some(_)) = &landing {
        // A value is charted on the world's own frame, which the move
        // leaves where it is: the copy would land where its numbers say
        // in the host's world, not moved with the rest.
        for placement in part.placements() {
            let Some(bundle) = part.node(placement).and_then(crate::mate::solve::bundle_of) else {
                continue;
            };
            let on_world = |id: crate::node::ConstraintId| {
                bundle
                    .iter()
                    .any(|c| c.id == id && c.mate().is_some() && c.reads_world())
            };
            if let Some(value) = bundle
                .iter()
                .find(|c| c.value().is_some_and(|v| on_world(v.mate)))
            {
                return Err(InlineError::ValueOnAMovedWorld {
                    placement: part.spoken(placement),
                    value: value.id,
                });
            }
        }
        for id in part.ids() {
            let Some(node) = part.node(id) else { continue };
            if matches!(
                node,
                Node::Place { .. }
                    | Node::InstantiatePart { .. }
                    | Node::World
                    | Node::Measure { .. }
                    | Node::Assertion { .. }
            ) {
                continue;
            }
            if part.upstream_of(id, node).into_iter().any(|input| {
                matches!(
                    part.node(input),
                    Some(Node::Place { .. } | Node::InstantiatePart { .. })
                )
            }) {
                return Err(InlineError::ConstructionReadsCopy {
                    node: part.spoken(id),
                });
            }
        }
    }
    // Wherever the part's world moves (a pose of the instance's own, or
    // the root taking the instance's bundle), its constructions stay,
    // so a part copy pinned against one would land beside it rather
    // than where the instance put it.
    let world_moves = match &landing {
        Landing::Pinned(motion) => motion.is_some(),
        Landing::Root(_) => true,
    };
    if world_moves {
        for placement in part.placements() {
            let Some(bundle) = part.node(placement).and_then(crate::mate::solve::bundle_of) else {
                continue;
            };
            for constraint in bundle {
                let Some(to) = constraint.mate().and_then(|m| m.to.pose()) else {
                    continue;
                };
                if let Some(construction) = construction_read(&part, *to) {
                    return Err(InlineError::CopyPinnedToConstruction {
                        placement: part.spoken(placement),
                        construction: part.spoken(construction),
                    });
                }
            }
        }
    }
    // Whether a part copy stands where the instance put the body it
    // copies: landing verbatim at the identity, its placement pinned
    // there too, so a reader of the copy reads that body.
    let verbatim = matches!(&landing, Landing::Pinned(None));
    let raw = |placement: RecipeNodeId| {
        verbatim
            && places_at_identity(placement)
            && matches!(part.node(placement), Some(Node::Place { shapes, .. }) if shapes.len() == 1)
    };
    // The instance's readers (A4, A10): every operand of the host that
    // reads its copies, whole, one copy by its key, or a selection of
    // a face of it; and the sides of the host's constraints, which are
    // selections. A whole read has no one body to read once the copies
    // are the part's placements, and refuses.
    let copies = doc
        .output(instance, 0)
        .unwrap_or_else(|| unreachable!("an instance defines its copies"));
    let mut readers: Vec<(RecipeNodeId, crate::OperandSlot, VarId)> = Vec::new();
    // A pose read at a seat (a mate's side) that reads a selection of
    // the copies in it is written afresh over the spliced copies.
    let mut pose_readers: Vec<(RecipeNodeId, crate::OperandSlot, VarId)> = Vec::new();
    for id in doc.ids() {
        if id == instance {
            continue;
        }
        let Some(reader) = doc.node(id) else { continue };
        for (slot, var) in reader.operand_rows() {
            if let Some(leaves) = pose_leaves(doc, var) {
                if leaves.contains(&copies) {
                    return Err(InlineError::InstanceReadUncarried {
                        reader: doc.spoken(id),
                        why: Uncarried::Bodies {
                            count: part_world.len(),
                        },
                    });
                }
                if leaves
                    .iter()
                    .any(|&leaf| doc.selection(leaf).is_some_and(|s| s.body == copies))
                {
                    pose_readers.push((id, slot, var));
                }
                continue;
            }
            let body = doc.selection(var).map_or(var, |select| select.body);
            if body != copies {
                continue;
            }
            if doc.selection(var).is_none()
                && reader.body_read(slot).is_none_or(|read| read.key.is_none())
            {
                return Err(InlineError::InstanceReadUncarried {
                    reader: doc.spoken(id),
                    why: Uncarried::Bodies {
                        count: part_world.len(),
                    },
                });
            }
            readers.push((id, slot, var));
        }
    }
    // Host references deriving from the instance must be exactly the
    // bridge's wrapped form — those re-anchor; anything else has no
    // local correspondent and refuses typed.
    let mut wrapped: Vec<StableName> = Vec::new();
    let mut classify = |name: &StableName| -> Result<(), InlineError> {
        if !derivation_nodes(name).contains(&instance) {
            return Ok(());
        }
        if name.node == instance
            && let [RoleSeg::InPart { .. }] = &name.path[..]
        {
            wrapped.push(name.clone());
            return Ok(());
        }
        if name.node == instance && name.path == vec![RoleSeg::OutputBody] {
            return Err(InlineError::InstanceBodyNameReferenced {
                name: doc.spoken_name(name),
            });
        }
        Err(InlineError::ForeignInstanceName {
            name: doc.spoken_name(name),
        })
    };
    for carrier in doc.name_carriers() {
        classify(carrier.name())?;
    }
    wrapped.sort();
    wrapped.dedup();
    // The part copy a selection of the instance's copies lands on: the
    // one placement all its names are under.
    let landing_copy = |var: VarId| -> Option<RecipeNodeId> {
        let select = doc.selection(var)?;
        let mut placements = select
            .names
            .iter()
            .filter_map(|name| match name.path.as_slice() {
                [RoleSeg::InPart { of }] if name.node == instance => Some(of.node),
                _ => None,
            });
        let first = placements.next()?;
        placements.all(|p| p == first).then_some(first)
    };
    let mut current = Recording::start(doc, tol, &reach);
    let refused = |error| InlineError::Edit {
        error: Box::new(error),
    };
    let step = |current: &mut Recording<'_, ProfileProgram>,
                edit: DocEdit<ProfileProgram>|
     -> Result<(), InlineError> { current.apply(edit).map(|_| ()).map_err(refused) };
    // Every part variable crosses as an id the host mints, never one
    // it holds: a named one declared here, refusing a name the host
    // holds whatever either's value; an anonymous one with the first
    // spliced edit that reads it (`VarCarry`).
    let mut vars = VarCarry::new(&part);
    for id in part.definition_order() {
        let Some(name) = part.var_name(id) else {
            continue;
        };
        if doc.var_named(name.as_str()).is_some() {
            return Err(InlineError::VarNameConflict { name: name.clone() });
        }
        // An output crosses with its node (`carry`).
        if part.var(id).is_some_and(|var| var.def().output().is_some()) {
            continue;
        }
        vars.declare(&mut current, id, name.clone())
            .map_err(refused)?;
    }
    // A name on the instance's copies moves onto the one copy of the
    // part, where the part has one: the body it copies, landing
    // verbatim, else the copy.
    let heir_of_name = match doc.var_name(copies) {
        None => None,
        Some(name) => {
            let [placement] = part_world.as_slice() else {
                return Err(InlineError::InstanceOutputUncarried {
                    name: name.clone(),
                    why: Uncarried::Bodies {
                        count: part_world.len(),
                    },
                });
            };
            let held = match part.node(*placement) {
                Some(Node::Place { shapes, .. }) if raw(*placement) => {
                    shapes.first().and_then(|s| part.var_name(s.read))
                }
                _ => part
                    .output(*placement, 0)
                    .and_then(|copy| part.var_name(copy)),
            };
            if let Some(held) = held {
                return Err(InlineError::InstanceOutputUncarried {
                    name: name.clone(),
                    why: Uncarried::HeirNamed { held: held.clone() },
                });
            }
            Some((name.clone(), *placement))
        }
    };
    // The part's nodes in its document order, each under the id the
    // host's insert door mints for it, the world aside: the host's is
    // the part's.
    let carried: Vec<RecipeNodeId> = part
        .ids()
        .into_iter()
        .filter(|&id| id != RecipeNodeId::WORLD)
        .collect();
    let (node_map, step_map, read_map) = carry(
        &part,
        &carried,
        &mut current,
        &mut vars,
        // Under a moved landing a world read's run of written moves is
        // folded into the motion, so a part inlined into a part inlined
        // in turn holds one run, not one per level.
        matches!(landing, Landing::Pinned(Some(_))),
        |old, mate, run| match &landing {
            // The root's own pin goes: the instance's bundle takes its
            // place below.
            Landing::Root(root) => old != *root,
            // A constraint on the part's world is one on the host's
            // world, moved by the instance's pose, composed after the
            // run its read was folded off.
            Landing::Pinned(motion) => {
                if let (Some(motion), crate::mate::Target::World(world)) = (motion, &mut mate.to) {
                    let (linear, translation) = match run {
                        Some((held, at)) => (
                            motion.linear * *held,
                            motion.linear * *at + motion.translation,
                        ),
                        None => (motion.linear, motion.translation),
                    };
                    *world = crate::mate::pin::moved_by(world.clone(), linear, translation);
                }
                true
            }
        },
        refused,
        |old, miss| match miss {
            RemapMiss::Input(input) => InlineError::Edit {
                error: Box::new(EditError::UnresolvedInput {
                    input: part.spoken(input),
                }),
            },
            RemapMiss::Read { slot, var, .. } => InlineError::Edit {
                error: Box::new(EditError::OperandUnresolved {
                    node: part.spoken(old),
                    slot: crate::SlotId::Operand(slot),
                    read: crate::Operand::Var(var),
                }),
            },
            RemapMiss::PayloadRead { var } => InlineError::Edit {
                error: Box::new(EditError::PayloadUnresolvedVar {
                    var: part.spoken_var(var),
                    node: part.spoken(old),
                }),
            },
            RemapMiss::Name { name, missing } => InlineError::stranded(&part, &name, missing),
        },
    )?;
    // A name of the part's product (a copy's name, under its placement)
    // as the host spells it once the instance is gone: the body's own
    // name where the copy stands where its body did, else the carried
    // copy's.
    let to_host = |of: &StableName| -> Result<StableName, InlineError> {
        if let Some((placement, inner)) = of.copy_of()
            && raw(placement)
        {
            return remap_name(inner, &node_map, &step_map, &read_map)
                .map_err(|missing| InlineError::stranded(&part, inner, missing));
        }
        remap_name(of, &node_map, &step_map, &read_map)
            .map_err(|missing| InlineError::stranded(&part, of, missing))
    };
    // The inverse of the bridge: wrapped names re-anchor to local.
    let rehost = |from: &StableName| -> Result<StableName, InlineError> {
        // The collection admitted exactly this shape; a non-match here
        // would be a collection bug, and skipping it would strand the
        // name silently — so the shape is re-destructured, not assumed.
        let [RoleSeg::InPart { of }] = &from.path[..] else {
            return Err(InlineError::ForeignInstanceName {
                name: doc.spoken_name(from),
            });
        };
        to_host(of)
    };
    // The host variable a part copy is read through once spliced, and
    // the key of the copy within it: the body it copies, landing
    // verbatim; a `Place`'s copies, whole over one shape, keyed by read
    // over several; an instance's copies, keyed as they were.
    let host_copy =
        |host: &ProfileDoc, key: &crate::CopyKey| -> Option<(VarId, Option<crate::CopyKey>)> {
            let crate::CopyKey::Part { placement, within } = key else {
                return None;
            };
            let carried = host.output(*node_map.get(placement)?, 0)?;
            match (part.node(*placement)?, within.as_ref()) {
                (Node::Place { shapes, .. }, crate::CopyKey::Read(var)) => {
                    let read = *read_map.get(var)?;
                    if raw(*placement) {
                        Some((read, None))
                    } else if shapes.len() == 1 {
                        Some((carried, None))
                    } else {
                        Some((carried, Some(crate::CopyKey::Read(read))))
                    }
                }
                (Node::InstantiatePart { .. }, within) => Some((carried, Some(within.clone()))),
                _ => None,
            }
        };
    // A selection's body once spliced: the copy its names land on, read
    // as [`host_copy`] reads it.
    let selection_body = |host: &ProfileDoc, var: VarId| -> Option<VarId> {
        let placement = landing_copy(var)?;
        let carried = host.output(*node_map.get(&placement)?, 0)?;
        match part.node(placement)? {
            Node::Place { shapes, .. } if raw(placement) => {
                read_map.get(&shapes.first()?.read).copied()
            }
            _ => Some(carried),
        }
    };
    // Witness data copies VERBATIM while ids remap — the same
    // invariant as split's copy: a witness datum is sketch-self-
    // relative and embeds no other node's identity (see split's
    // witness loop).
    for old in part.ids() {
        if let (Some(&new), Some(witness)) = (node_map.get(&old), part.witness(old)) {
            step(
                &mut current,
                DocEdit::ReWitness {
                    node: new,
                    witness: witness.clone(),
                },
            )?;
        }
    }
    // The part's appearance records land on the spliced local names.
    // A collision with a host record is the Rebind door's own typed
    // refusal below — never an auto-pick.
    for (at, record) in part.appearance().iter() {
        let name = &at.name;
        let key = remap_name(name, &node_map, &step_map, &read_map)
            .map_err(|missing| InlineError::stranded(&part, name, missing))?;
        // The body read lands on its spliced local read, as a name's
        // carried reads do.
        let body = read_map.get(&at.at).copied().ok_or_else(|| {
            InlineError::stranded(&part, name, RemapError::Unmapped(Unmapped::Read(at.at)))
        })?;
        for attr in record.attrs.values() {
            step(
                &mut current,
                DocEdit::SetAppearance {
                    body,
                    name: key.clone(),
                    attr: attr.clone(),
                },
            )?;
        }
        for (meta_key, value) in &record.metadata {
            step(
                &mut current,
                DocEdit::SetAppearanceMeta {
                    body,
                    name: key.clone(),
                    key: meta_key.clone(),
                    value: value.clone(),
                },
            )?;
        }
    }
    // The selections of the instance's copies are authored afresh on
    // the spliced copy below; every other carrier is repaired here, a
    // selection of a body downstream of the instance by its body.
    for from in &wrapped {
        let mut bodies: BTreeSet<VarId> = BTreeSet::new();
        let mut unbodied = false;
        for carrier in current.doc().name_carriers() {
            if carrier.name() != from {
                continue;
            }
            match carrier {
                NameCarrier::Select { var, .. } => {
                    if let Some(select) = current.doc().selection(var)
                        && select.body != copies
                    {
                        bodies.insert(select.body);
                    }
                }
                NameCarrier::Payload { .. } | NameCarrier::Store { .. } => unbodied = true,
            }
        }
        for body in bodies.into_iter().map(Some).chain(unbodied.then_some(None)) {
            step(
                &mut current,
                DocEdit::Rebind {
                    body,
                    from: from.clone(),
                    to: rehost(from)?,
                },
            )?;
        }
    }
    // The record dissolves (ASM-R2b D-4, and the function docs): every
    // crossing's part-side reference must land on a spliced local
    // name. The declaration itself survives in the constraint, which is
    // in the host and whose wrapped reference the loop above just
    // re-anchored — so the record's job ends here, CHECKED.
    for crossing in &interface.crossings {
        let InterfaceCrossing::Mate { inner, .. } = crossing;
        to_host(inner)?;
    }
    // A selection authored afresh on the spliced copy, its names
    // re-anchored.
    let reselect = |host: &ProfileDoc, var: VarId| -> Result<crate::Operand, InlineError> {
        let reader = || {
            doc.spoken(
                readers
                    .iter()
                    .find(|r| r.2 == var)
                    .map_or(instance, |r| r.0),
            )
        };
        let Some(select) = doc.selection(var) else {
            unreachable!("a selection reader reads a selection")
        };
        let body = selection_body(host, var).ok_or_else(|| InlineError::InstanceReadUncarried {
            reader: reader(),
            why: Uncarried::Bodies {
                count: part_world.len(),
            },
        })?;
        Ok(crate::Operand::select(
            crate::Operand::Var(body),
            crate::var::Select::canonical(
                selection_kind(doc, var),
                select
                    .names
                    .iter()
                    .map(|name| {
                        if wrapped.contains(name) {
                            rehost(name)
                        } else {
                            Ok(name.clone())
                        }
                    })
                    .collect::<Result<Vec<_>, _>>()?,
            ),
        ))
    };
    // The root takes the instance's bundle: each constraint authored
    // afresh on it, its own side's faces re-anchored onto the root's
    // copy, every other read kept.
    if let Landing::Root(root) = &landing {
        let Some(&carried_root) = node_map.get(root) else {
            unreachable!("the root is carried")
        };
        let host_vars = VarCarry::new(doc);
        let mut ids: BTreeMap<crate::node::ConstraintId, crate::node::ConstraintId> =
            BTreeMap::new();
        for constraint in bundle {
            let one = Node::Place {
                shapes: Vec::new(),
                constraints: vec![constraint.clone()],
            };
            // A side on the instance's copies is reselected on the
            // root's; an anonymous selection of another copy is authored
            // afresh, since it has one reader and the instance still
            // reads it until it goes; a named one is read by its name.
            let mut selects: BTreeMap<VarId, crate::Operand> = BTreeMap::new();
            let sides = constraint
                .mate()
                .map(|mate| std::iter::once(mate.on).chain(mate.to.pose().copied()))
                .into_iter()
                .flatten();
            for side in sides {
                if doc.var_name(side).is_some() || pose_leaves(doc, side).is_none() {
                    continue;
                }
                let mut leaf = |leaf: VarId| -> Result<Option<crate::Operand>, InlineError> {
                    match doc.selection(leaf) {
                        Some(select) if select.body == copies => {
                            reselect(current.doc(), leaf).map(Some)
                        }
                        _ => Ok(None),
                    }
                };
                selects.insert(side, rewritten_pose(doc, side, &mut leaf)?);
            }
            let (authored, fresh) = host_vars.author(&one, &selects);
            let Node::Place {
                constraints: mut authored,
                ..
            } = authored
            else {
                unreachable!("a placement carrier stays one")
            };
            let Some(authored) = authored.pop() else {
                unreachable!("the carrier holds the one constraint")
            };
            let edit = match authored.kind {
                crate::mate::ConstraintKind::Mate(mate) => DocEdit::AddConstraint {
                    placement: carried_root,
                    mate: Box::new(mate),
                    fresh: fresh.fresh,
                },
                crate::mate::ConstraintKind::Value(mut value) => {
                    let Some(&mate) = ids.get(&value.mate) else {
                        unreachable!("a value's mate is in its bundle, ahead of it or not")
                    };
                    value.mate = mate;
                    DocEdit::AddValue {
                        placement: carried_root,
                        value: Box::new(value),
                        fresh: fresh.fresh,
                    }
                }
            };
            let record = current.apply_recorded(edit).map_err(refused)?;
            if let Some(&id) = record.minted_constraints.first() {
                ids.insert(constraint.id, id);
            }
        }
    }
    {
        // Once per selection, as split authors them: a named one keeps
        // its name on the new variable, which every other reader reads.
        let mut reauthored: BTreeMap<VarId, VarId> = BTreeMap::new();
        for &(node, slot, var) in &readers {
            if node == instance {
                continue;
            }
            let value = match (doc.selection(var), reauthored.get(&var)) {
                (Some(_), Some(&fresh)) => crate::SlotValue::Read(crate::Operand::Var(fresh)),
                (Some(_), None) => crate::SlotValue::Read(reselect(current.doc(), var)?),
                (None, _) => {
                    let Some(key) = doc
                        .node(node)
                        .and_then(|n| n.body_read(slot))
                        .and_then(|read| read.key.clone())
                    else {
                        unreachable!("a whole reader of the copies refused above")
                    };
                    match host_copy(current.doc(), &key) {
                        Some((read, None)) => crate::SlotValue::Read(crate::Operand::Var(read)),
                        Some((read, Some(key))) => crate::SlotValue::Copy {
                            of: crate::Operand::Var(read),
                            key,
                        },
                        None => {
                            return Err(InlineError::InstanceReadUncarried {
                                reader: doc.spoken(node),
                                why: Uncarried::NoSuchCopy { key: Box::new(key) },
                            });
                        }
                    }
                }
            };
            step(
                &mut current,
                DocEdit::SetParam {
                    node,
                    slot: crate::SlotId::Operand(slot),
                    value,
                    fresh: Vec::new(),
                },
            )?;
            if doc.selection(var).is_some() && !reauthored.contains_key(&var) {
                let fresh = read_at(current.doc(), node, slot);
                reauthored.insert(var, fresh);
                if let Some(name) = doc.var_name(var) {
                    step(&mut current, DocEdit::DeleteVar { var: var.into() })?;
                    step(
                        &mut current,
                        DocEdit::RenameVar {
                            var: fresh.into(),
                            name: Some(name.clone()),
                        },
                    )?;
                }
            }
        }
        for &(node, slot, var) in &pose_readers {
            let mut leaf = |leaf: VarId| -> Result<Option<crate::Operand>, InlineError> {
                match doc.selection(leaf) {
                    Some(select) if select.body == copies => {
                        reselect(current.doc(), leaf).map(Some)
                    }
                    _ => Ok(None),
                }
            };
            let value = crate::SlotValue::Read(rewritten_pose(doc, var, &mut leaf)?);
            step(
                &mut current,
                DocEdit::SetParam {
                    node,
                    slot: crate::SlotId::Operand(slot),
                    value,
                    fresh: Vec::new(),
                },
            )?;
        }
    }
    // A host appearance key read in the instance's copies is re-attached
    // in the part's one copy, where the name on the copies goes too: a
    // Rebind keeps a key's read, and the inline takes that read away. A
    // part of several copies has no one body to hold it, as a whole
    // reader of the copies has none, and refuses.
    for (key, record) in current.doc().appearance().clone() {
        if key.at != copies {
            continue;
        }
        let [placement] = part_world.as_slice() else {
            return Err(InlineError::InstanceReadUncarried {
                reader: doc.spoken(instance),
                why: Uncarried::Bodies {
                    count: part_world.len(),
                },
            });
        };
        let heir = match part.node(*placement) {
            Some(Node::Place { shapes, .. }) if raw(*placement) => {
                shapes.first().and_then(|s| read_map.get(&s.read)).copied()
            }
            _ => node_map
                .get(placement)
                .and_then(|&p| current.doc().output(p, 0)),
        };
        let Some(heir) = heir else {
            unreachable!("the part's one copy is carried, and is defined")
        };
        let name = if wrapped.contains(&key.name) {
            rehost(&key.name)?
        } else {
            key.name.clone()
        };
        reattach_appearance(
            &mut |edit| step(&mut current, edit),
            &key,
            &record,
            &crate::AppearanceKey::new(heir, name),
        )?;
    }
    step(&mut current, DocEdit::DeleteNode { id: instance })?;
    if let Some((name, placement)) = heir_of_name {
        let heir = match part.node(placement) {
            Some(Node::Place { shapes, .. }) if raw(placement) => {
                shapes.first().and_then(|s| read_map.get(&s.read)).copied()
            }
            _ => node_map
                .get(&placement)
                .and_then(|&p| current.doc().output(p, 0)),
        };
        let Some(heir) = heir else {
            unreachable!("the part's one copy is carried, and is defined")
        };
        step(
            &mut current,
            DocEdit::RenameVar {
                var: heir.into(),
                name: Some(name),
            },
        )?;
    }
    let Recorded {
        doc,
        edits,
        maintenance,
        ..
    } = current.finish().map_err(refused)?;
    Ok(InlineOutcome {
        doc,
        edits,
        maintenance,
        node_map,
        step_map,
    })
}
/// **A remap never changes a KIND.**
///
/// [`remap_name`] rewrites a name's derivation — its minting node and
/// the node ids embedded in its role path — and copies the kind
/// through untouched, for every [`crate::EntityKind`] and through a
/// nested name-bearing segment.
/// **One appearance record re-attached from `from` to `onto`**: every
/// attribute and metadata entry set at `onto`, then cleared at `from`. A
/// Rebind never moves a key's read, so a refactor that takes a key's
/// read away re-attaches the key this way, as an author would.
fn reattach_appearance<E>(
    apply: &mut dyn FnMut(DocEdit<ProfileProgram>) -> Result<(), E>,
    from: &crate::AppearanceKey,
    record: &crate::AppearanceRecord,
    onto: &crate::AppearanceKey,
) -> Result<(), E> {
    if from == onto {
        return Ok(());
    }
    for attr in record.attrs.values() {
        apply(DocEdit::SetAppearance {
            body: onto.at,
            name: onto.name.clone(),
            attr: attr.clone(),
        })?;
    }
    for (key, value) in &record.metadata {
        apply(DocEdit::SetAppearanceMeta {
            body: onto.at,
            name: onto.name.clone(),
            key: key.clone(),
            value: value.clone(),
        })?;
    }
    for kind in record.attrs.keys() {
        apply(DocEdit::ClearAppearance {
            body: from.at,
            name: from.name.clone(),
            kind: *kind,
        })?;
    }
    for key in record.metadata.keys() {
        apply(DocEdit::ClearAppearanceMeta {
            body: from.at,
            name: from.name.clone(),
            key: key.clone(),
        })?;
    }
    Ok(())
}

#[cfg(test)]
#[allow(clippy::expect_used, clippy::panic)]
mod remap_keeps_the_kind {
    use super::{NodeMap, ReadMap, StepMap, remap_name};
    use crate::names::{FaceName, NameRef, RoleSeg, StableName};
    use crate::node::RecipeNodeId;
    use crate::{CapEnd, EntityKind};

    /// The reads the names below carry, mapped as [`map`] maps the
    /// member nodes they stand for, and the fold sides onto themselves.
    fn mirrored_reads() -> ReadMap {
        map()
            .into_iter()
            .map(|(a, b)| {
                (
                    crate::VarId::new(1, a.0.digest()),
                    crate::VarId::new(1, b.0.digest()),
                )
            })
            .chain([crate::names::FOLD_A, crate::names::FOLD_B].map(|f| (f, f)))
            .collect()
    }

    fn map() -> NodeMap {
        [
            (RecipeNodeId::new(0, 0), RecipeNodeId::new(0, 10)),
            (RecipeNodeId::new(0, 1), RecipeNodeId::new(0, 11)),
        ]
        .into_iter()
        .collect()
    }

    fn name(kind: EntityKind) -> StableName {
        StableName {
            kind,
            node: RecipeNodeId::new(0, 0),
            path: vec![
                RoleSeg::Cap(CapEnd::End),
                RoleSeg::From {
                    read: crate::names::FOLD_A,
                    of: NameRef::new(StableName {
                        kind: EntityKind::Edge,
                        node: RecipeNodeId::new(0, 1),
                        path: vec![RoleSeg::Cap(CapEnd::Start)],
                    }),
                },
            ],
        }
    }

    /// The bare-name rewrite carries every kind through, and renumbers
    /// the mint while it does.
    #[test]
    fn a_bare_name_remap_carries_every_kind_through() {
        for kind in [
            EntityKind::Body,
            EntityKind::Face,
            EntityKind::Edge,
            EntityKind::Vertex,
        ] {
            let out = remap_name(&name(kind), &map(), &StepMap::new(), &mirrored_reads())
                .expect("the map covers both ids");
            assert_eq!(out.kind, kind, "remap_name must carry the kind through");
            assert_eq!(out.node, RecipeNodeId::new(0, 10), "and renumber the mint");
            assert_eq!(
                FaceName::new(out).is_ok(),
                kind == EntityKind::Face,
                "so what a rewritten name denotes is decided by the INPUT kind alone"
            );
        }
    }
}

/// **A name-shaped refusal carries the node the rewrite stopped at.**
///
/// A [`StableName`] embeds other names in its `path`, so the node a
/// remap could not map may sit inside a path segment rather than be
/// the name's own mint. A refusal raised out of such a miss names the
/// OUTER name, and for a nested name that is a DIFFERENT node from the
/// one that failed — so the id is what a reader has to act on, and
/// every refusal raised out of a remap miss carries it.
#[cfg(test)]
#[allow(clippy::expect_used, clippy::panic)]
mod a_miss_two_segments_down_is_not_the_outer_name {
    use super::{NodeMap, ReadMap, RemapError, StepMap, Unmapped, remap_name};
    use crate::names::{NameRef, RoleSeg, StableName};
    use crate::node::RecipeNodeId;
    use crate::{CapEnd, EntityKind};

    const OUTER: RecipeNodeId = RecipeNodeId::new(0, 0);
    const INNER: RecipeNodeId = RecipeNodeId::new(0, 1);

    /// A map that carries the OUTER mint and NOT the nested one: the
    /// case where the name a refusal names and the node that failed
    /// come apart.
    fn map() -> NodeMap {
        [(OUTER, RecipeNodeId::new(0, 10))].into_iter().collect()
    }

    /// A name whose own mint is [`OUTER`] and whose path embeds a name
    /// minted at [`INNER`].
    fn nested(kind: EntityKind) -> StableName {
        StableName {
            kind,
            node: OUTER,
            path: vec![
                RoleSeg::Cap(CapEnd::End),
                RoleSeg::From {
                    read: crate::names::FOLD_A,
                    of: NameRef::new(StableName {
                        kind: EntityKind::Edge,
                        node: INNER,
                        path: vec![RoleSeg::Cap(CapEnd::Start)],
                    }),
                },
            ],
        }
    }

    /// The reads the names below carry, mapped onto themselves, so a
    /// miss is the node's.
    fn reads() -> ReadMap {
        ReadMap::from([(crate::names::FOLD_A, crate::names::FOLD_A)])
    }

    /// The premise: the rewrite itself reports the nested node, which
    /// is not the name's mint.
    #[test]
    fn the_rewrite_reports_the_nested_node() {
        let name = nested(EntityKind::Edge);
        let missing =
            remap_name(&name, &map(), &StepMap::new(), &reads()).expect_err("INNER is unmapped");
        assert_eq!(missing, RemapError::Unmapped(Unmapped::Node(INNER)));
        assert_ne!(
            missing,
            RemapError::Unmapped(Unmapped::Node(name.node)),
            "the node that failed is not the name's own mint"
        );
    }
}

/// **A remap that reorders ids republishes the canonical form.**
///
/// A map need not keep two ids in their order (it sends each to the id
/// another document's insert door mints for it), so two ids can come
/// out in the other order. Every name-ordered position then has to be put back in
/// order, and a crossing ranked along a union seam whose sides swap
/// reads its rank from the other end — the form the emitters would mint
/// for the same entity under the new ids.
#[cfg(test)]
#[allow(clippy::expect_used)]
mod a_remap_that_reorders_ids_republishes_the_canonical_form {
    use super::{NodeMap, ReadMap, StepMap, remap_name};
    use crate::names::{NameRef, Qualifier, RoleSeg, StableName};
    use crate::node::RecipeNodeId;
    use crate::{CapEnd, EntityKind};

    /// Union 9 → 20; members 1 and 2 swap order (→ 31, 30); 3 → 32.
    /// The reads the names below carry, mapped as [`map`] maps the
    /// member nodes they stand for, and the fold sides onto themselves.
    fn mirrored_reads() -> ReadMap {
        map()
            .into_iter()
            .map(|(a, b)| {
                (
                    crate::VarId::new(1, a.0.digest()),
                    crate::VarId::new(1, b.0.digest()),
                )
            })
            .chain([crate::names::FOLD_A, crate::names::FOLD_B].map(|f| (f, f)))
            .collect()
    }

    fn map() -> NodeMap {
        [(9, 20), (1, 31), (2, 30), (3, 32), (5, 25)]
            .into_iter()
            .map(|(a, b)| (RecipeNodeId::new(0, a), RecipeNodeId::new(0, b)))
            .collect()
    }

    fn cap(kind: EntityKind, node: u64) -> StableName {
        StableName {
            kind,
            node: RecipeNodeId::new(0, node),
            path: vec![RoleSeg::Cap(CapEnd::Start)],
        }
    }

    /// Member `m`'s start cap in union `u`'s space.
    fn member(kind: EntityKind, u: u64, m: u64) -> StableName {
        StableName {
            kind,
            node: RecipeNodeId::new(0, u),
            path: vec![RoleSeg::From {
                read: crate::VarId::new(1, m),
                of: NameRef::new(cap(kind, m)),
            }],
        }
    }

    fn seam(a: StableName, b: StableName) -> RoleSeg {
        RoleSeg::Seam {
            a: NameRef::new(a),
            b: NameRef::new(b),
        }
    }

    fn rank(rank: u32, of: u32) -> RoleSeg {
        RoleSeg::Fragment(Qualifier::OrderAlong { rank, of })
    }

    fn name(kind: EntityKind, node: u64, path: Vec<RoleSeg>) -> StableName {
        StableName {
            kind,
            node: RecipeNodeId::new(0, node),
            path,
        }
    }

    /// A union seam edge between members 1 and 2.
    fn union_edge(u: u64, (m1, m2): (u64, u64)) -> StableName {
        let (x, y) = (
            member(EntityKind::Face, u, m1),
            member(EntityKind::Face, u, m2),
        );
        let (x, y) = if x <= y { (x, y) } else { (y, x) };
        name(EntityKind::Edge, u, vec![seam(x, y)])
    }

    /// A piece of that seam, ending at members 1's and 2's start-cap
    /// vertices.
    fn union_piece(u: u64, (m1, m2): (u64, u64)) -> StableName {
        let mut piece = union_edge(u, (m1, m2));
        let mut ends = vec![
            member(EntityKind::Vertex, u, m1),
            member(EntityKind::Vertex, u, m2),
        ];
        ends.sort();
        piece.path.push(RoleSeg::Fragment(Qualifier::Ends(ends)));
        piece
    }

    #[test]
    fn a_union_seam_edge_piece_swaps_its_sides_and_reorders_its_ends() {
        let out = remap_name(
            &union_piece(9, (1, 2)),
            &map(),
            &StepMap::new(),
            &mirrored_reads(),
        )
        .expect("covered");
        assert_eq!(
            out,
            union_piece(20, (31, 30)),
            "the pair and the ends in name order under the new ids"
        );
    }

    #[test]
    fn a_union_seam_vertex_follows_its_edge_parents_line() {
        let vertex = |u: u64, edge: StableName, m3: u64, r: u32| {
            let face = member(EntityKind::Face, u, m3);
            let (x, y) = if edge <= face {
                (edge, face)
            } else {
                (face, edge)
            };
            name(EntityKind::Vertex, u, vec![seam(x, y), rank(r, 2)])
        };
        let was = vertex(9, union_edge(9, (1, 2)), 3, 0);
        let out = remap_name(&was, &map(), &StepMap::new(), &mirrored_reads()).expect("covered");
        assert_eq!(out, vertex(20, union_edge(20, (31, 30)), 32, 1));
    }

    #[test]
    fn a_pair_booleans_sets_borders_and_junction_are_resorted_and_its_seams_stay_sided() {
        // Boolean 5 over operands 1 and 2, with 3 a third name.
        let face = |n| cap(EntityKind::Face, n);
        let was = name(
            EntityKind::Face,
            5,
            vec![
                RoleSeg::Merged(vec![face(1), face(2)]),
                RoleSeg::Fragment(Qualifier::Borders(vec![face(1), face(2)])),
            ],
        );
        let out = remap_name(&was, &map(), &StepMap::new(), &mirrored_reads()).expect("covered");
        assert_eq!(
            out.path,
            vec![
                RoleSeg::Merged(vec![face(30), face(31)]),
                RoleSeg::Fragment(Qualifier::Borders(vec![face(30), face(31)])),
            ]
        );
        let junction = name(
            EntityKind::Vertex,
            5,
            vec![seam(face(1), face(3)), seam(face(2), face(3))],
        );
        let out =
            remap_name(&junction, &map(), &StepMap::new(), &mirrored_reads()).expect("covered");
        assert_eq!(
            out.path,
            vec![seam(face(30), face(32)), seam(face(31), face(32))],
            "the run re-sorted, each line A-first"
        );
        let sided = name(
            EntityKind::Vertex,
            5,
            vec![seam(cap(EntityKind::Edge, 1), face(2)), rank(0, 3)],
        );
        let out = remap_name(&sided, &map(), &StepMap::new(), &mirrored_reads()).expect("covered");
        assert_eq!(
            out.path,
            vec![seam(cap(EntityKind::Edge, 31), face(30)), rank(0, 3)],
            "a pair boolean's seam is sided: no swap, no reversal"
        );
    }
}

/// **A profile piece's step crosses a refactoring through the step
/// map**, the way a node id crosses through the node map: the other
/// document re-mints every step it inserts, and a name spelling a step
/// no carried profile holds — a dropped step's stranded name — misses
/// on that step, typed.
#[cfg(test)]
#[allow(clippy::expect_used, clippy::panic)]
mod remap_moves_the_step {
    use super::{NodeMap, ReadMap, RemapError, StepMap, Unmapped, remap_name};
    use crate::EntityKind;
    use crate::names::{PieceRole, ProfileEdgeRef, RoleSeg, SectionCircle, StableName};
    use crate::node::{RecipeNodeId, StepId};

    fn wall(node: u64, e: ProfileEdgeRef) -> StableName {
        StableName {
            kind: EntityKind::Face,
            node: RecipeNodeId::new(0, node),
            path: vec![RoleSeg::Lateral(e.into())],
        }
    }

    fn piece(step: u64) -> ProfileEdgeRef {
        ProfileEdgeRef::Piece {
            step: StepId::new(0, step),
            role: PieceRole::Leg,
        }
    }

    #[test]
    fn a_piece_crosses_on_the_step_map_and_a_section_crosses_as_it_is() {
        let nodes: NodeMap = [(RecipeNodeId::new(0, 2), RecipeNodeId::new(0, 0))]
            .into_iter()
            .collect();
        let steps: StepMap = [(StepId::new(0, 7), StepId::new(0, 1))]
            .into_iter()
            .collect();
        assert_eq!(
            remap_name(&wall(2, piece(7)), &nodes, &steps, &ReadMap::new()).expect("covered"),
            wall(0, piece(1))
        );
        let section = ProfileEdgeRef::Section {
            circle: SectionCircle::Outer,
            role: PieceRole::Piece(0),
        };
        assert_eq!(
            remap_name(&wall(2, section), &nodes, &steps, &ReadMap::new()).expect("covered"),
            wall(0, section)
        );
        assert_eq!(
            remap_name(&wall(2, piece(8)), &nodes, &steps, &ReadMap::new()),
            Err(RemapError::Unmapped(Unmapped::Step(StepId::new(0, 8)))),
            "a step no carried profile holds misses on that step"
        );
    }
}

/// **A ranked edge-crossing run whose least line a caller's maps change
/// refuses typed.** A pair boolean ranks several vertices of one run
/// along the run's least line (N2). A caller's own maps need not keep
/// id order, so two lines can swap places; the rank was measured along
/// the line that is no longer least, and no name says where it lies
/// along the other, so `remap_name` refuses rather than carry a name
/// no emitter mints. Maps that keep the order carry it, rank and all.
#[cfg(test)]
#[allow(clippy::expect_used, clippy::panic)]
mod remap_a_ranked_run {
    use super::{NodeMap, ReadMap, RemapError, StepMap, remap_name};
    use crate::names::{NameRef, Qualifier, RoleSeg, RunLine, Sense, StableName};
    use crate::node::RecipeNodeId;
    use crate::{CapEnd, EntityKind};

    fn edge(node: u64) -> NameRef {
        NameRef::new(StableName {
            kind: EntityKind::Edge,
            node: RecipeNodeId::new(0, node),
            path: vec![RoleSeg::Cap(CapEnd::Start)],
        })
    }

    fn ranked(minter: u64, lines: [u64; 2]) -> StableName {
        StableName {
            kind: EntityKind::Vertex,
            node: RecipeNodeId::new(0, minter),
            path: vec![
                RoleSeg::EdgeCrossing(
                    lines
                        .into_iter()
                        .map(|n| RunLine {
                            line: edge(n),
                            sense: Some(Sense::Outside),
                        })
                        .collect(),
                ),
                RoleSeg::Fragment(Qualifier::OrderAlong { rank: 0, of: 2 }),
            ],
        }
    }

    fn map(pairs: [(u64, u64); 3]) -> NodeMap {
        pairs
            .into_iter()
            .map(|(a, b)| (RecipeNodeId::new(0, a), RecipeNodeId::new(0, b)))
            .collect()
    }

    #[test]
    fn a_remap_that_swaps_a_ranked_runs_lines_refuses_and_one_that_keeps_them_carries_the_rank() {
        let was = ranked(5, [1, 2]);
        let swapped = remap_name(
            &was,
            &map([(1, 20), (2, 10), (5, 15)]),
            &StepMap::new(),
            &ReadMap::new(),
        );
        assert_eq!(
            swapped,
            Err(RemapError::RankedRunReordered {
                vertex: Box::new(was.clone())
            }),
            "a map that makes the other line least refuses, naming the ranked vertex"
        );
        let text = swapped.expect_err("refused").to_string();
        assert!(
            text.contains("crossing 0 of 2"),
            "the refusal names the ranked vertex: {text}"
        );
        assert_eq!(
            remap_name(
                &was,
                &map([(1, 10), (2, 20), (5, 15)]),
                &StepMap::new(),
                &ReadMap::new()
            ),
            Ok(ranked(15, [10, 20])),
            "a map that keeps the lines' order carries the run and its rank"
        );
    }

    /// A ranked run embedded in another name refuses the same way, and
    /// names the run it found, not the name that holds it.
    #[test]
    fn a_ranked_run_a_name_embeds_refuses_naming_the_run() {
        let was = ranked(5, [1, 2]);
        let holder = StableName {
            kind: EntityKind::Face,
            node: RecipeNodeId::new(0, 5),
            path: vec![RoleSeg::BlendFace(NameRef::new(was.clone()))],
        };
        assert_eq!(
            remap_name(
                &holder,
                &map([(1, 20), (2, 10), (5, 15)]),
                &StepMap::new(),
                &ReadMap::new()
            ),
            Err(RemapError::RankedRunReordered {
                vertex: Box::new(was)
            })
        );
    }
}

/// **Every [`RemapError`] arm reads as a refusal**: within the budget,
/// one recourse, and ending on it, on a representative payload — the
/// ranked arm on a two-line run, which it names in full.
#[cfg(test)]
#[allow(clippy::expect_used, clippy::panic)]
mod remap_error_reads {
    use super::{RemapError, Unmapped};
    use crate::names::{NameRef, ProfileVertexRef, Qualifier, RoleSeg, RunLine, Sense, StableName};
    use crate::node::{RecipeNodeId, StepId};
    use crate::var::VarId;
    use crate::{EntityKind, PieceRole};
    use test_utils::refusal::{problems, recourse_markers, tagged};

    fn ranked() -> StableName {
        let edge = |step| {
            NameRef::new(StableName {
                kind: EntityKind::Edge,
                node: RecipeNodeId::new(0, tagged(3)),
                path: vec![RoleSeg::LateralEdge(ProfileVertexRef::Piece {
                    step: StepId::new(0, tagged(step)),
                    role: PieceRole::Leg,
                })],
            })
        };
        StableName {
            kind: EntityKind::Vertex,
            node: RecipeNodeId::new(0, tagged(5)),
            path: vec![
                RoleSeg::EdgeCrossing(
                    [1, 2]
                        .into_iter()
                        .map(|step| RunLine {
                            line: edge(step),
                            sense: Some(Sense::Outside),
                        })
                        .collect(),
                ),
                RoleSeg::Fragment(Qualifier::OrderAlong { rank: 0, of: 2 }),
            ],
        }
    }

    #[test]
    fn every_remap_refusal_fits_the_budget_and_ends_on_one_recourse() {
        let arms = [
            RemapError::Unmapped(Unmapped::Node(RecipeNodeId::new(0, tagged(4)))),
            RemapError::Unmapped(Unmapped::Step(StepId::new(0, tagged(4)))),
            RemapError::Unmapped(Unmapped::Read(VarId::new(0, tagged(4)))),
            RemapError::RankedRunReordered {
                vertex: Box::new(ranked()),
            },
        ];
        let mut found = Vec::new();
        for arm in &arms {
            let text = arm.to_string();
            let name = format!("{arm:?}");
            found.extend(problems(&name, &text, &[], false));
            let tail = text.rsplit("Recourse: ").next().unwrap_or_default();
            if recourse_markers(&text) != 1 || tail.contains(". ") || tail.len() == text.len() {
                found.push(format!("does not end on its one recourse: {text}"));
            }
        }
        assert!(found.is_empty(), "{}", found.join("\n"));
    }
}
