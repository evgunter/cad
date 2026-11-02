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
//! instance's copy carries what it reads: a selection each of whose
//! names one cut world placement's copy holds, re-authored on the
//! instance's copy; a read of one copy of a cut world placement, or of
//! a cut `Place`'s one copy, or of a body one cut world `Place` copies
//! at the identity, each re-pointed to a pick of the instance's copy:
//! the member of the part's product `⊔` that copy is, or the copy whole
//! where the product is that one copy ([`crate::VarDef::Pick`]). Any
//! other remainder read of
//! the cut is severed too.
//!
//! # Placements cross the seam (A4)
//!
//! The remainder receives ONE `InstantiatePart` for the whole cut,
//! pinned to the world at the identity: each instance materializes the
//! ENTIRE new document's product, so per-placement instances of one
//! pinned document would duplicate every other copy's material N times.
//! The part's product is its one world copy, or the `⊔` of its world
//! copies in document order, so the instance's copy stands where the
//! cut's copies did, and every pose is the one the cut solved to. A
//! kept constraint reading a cut copy reads it in the instance
//! (FORK-S3M: a constraint reads its target across the cut): its pose
//! is written afresh over the instance's copy, each selection in it
//! re-authored as a crossing selection is. The instance stands where
//! its part's product stands ([`crate::mate::stand`]). Neither door
//! computes a frame.
//!
//! [`inline`] is the inverse, by one rule: the part's nodes land in a
//! `new_space()` verbatim, and the instance becomes a placement of the
//! spliced product body into its target under its own bundle,
//! verbatim. A part with no product refuses
//! ([`InlineError::UnplaceablePart`]). A host read of the instance's
//! copy reads the new placement's, a pick of it a pick of that copy,
//! and a host mate reading it is written afresh over the spliced copy
//! its names land on. [`flatten`] folds such a placement, at the
//! identity, back into its target.
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
//! A pose is such a variable, read by the slot that reads it, and so is
//! each pose and number it is defined over; a selection under a pose is
//! not, and crosses with its body. No door declares a pose, so one
//! crosses as a selection does: written again at its first carried
//! reader, its faces re-selected on the carried bodies and its numbers
//! read as the target holds them, and named there as it was named. A
//! named number a pose on one side and a node on the other read refuses
//! [`SplitError::UncutVarReference`]. A named pose read on both sides
//! never gets that far: a pose reads the faces of the bodies it is
//! defined over, so a kept reader in their space reads the cut and
//! refuses as a severed read ([`SplitError::SeveredEdge`]), and the
//! edit door admits no reader in another space
//! ([`EditError::SpaceMismatch`]).
//!
//! A space is read as a variable is. Every read of a space is an edge
//! to the node that defines it, though it reaches no operation
//! ([`Doc::read_ports`]). A `new_space()` goes where its readers go: one
//! whose readers are all cut joins the cut, whether the selection names
//! it or not; one read on both sides refuses
//! ([`SplitError::SpaceStraddlesCut`]); an unread one stays unless the
//! selection names it. A construction's own space is its output and
//! goes with it, so a placement into it on the other side of the cut
//! is severed ([`SplitError::SeveredEdge`]). The world is no side's.
//!
//! So `inline(split(d))` is `d` up to minted ids — node, step,
//! variable and constraint — and order, on every cut split admits
//! (A4): split appends its instance to the remainder and inline appends
//! what it splices, so a cut whose placements a kept one separates
//! comes back after it. One cut split admits does not come back: with
//! the product reading one of several cut copies, split widens the
//! part's product to the `⊔` of its world copies and inline then finds
//! no preimage of the copy the product read
//! (`split-widens-a-product-reading-one-of-several-cut-copies`, open).
//! `split(inline(h))` is not promised: an unread
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
//! name to the name the part's product spells it by, which the new
//! instance's copy carries verbatim: the name the cut world placement's
//! copy carries, keyed by that copy's read (`From { read, of }` at the
//! product's `⊔`) where the part has several world copies (a recorded
//! [`DocEdit::Rebind`] per name), which is exactly how "every stable
//! name that resolved before resolves after" (D-4) is kept. Inline
//! applies the inverse rewrite: the part's names at the inlined
//! instance re-anchor to the spliced local names — the body's own where
//! its copy stands where the body did, the carried copy's otherwise.
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
    StableName,
};
use crate::node::{AuthoredNode, InterfaceCrossing, InterfaceRecord, Node, RecipeNodeId, StepId};
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
        let RemapError::Unmapped(missing) = missing;
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
        let RemapError::Unmapped(missing) = missing;
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
        let RemapError::Unmapped(missing) = missing;
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
/// that keeps id order carries each name-ordered position in the order
/// it had.
///
/// Across two profiles the order cannot always be kept: the door mints
/// each profile's ids when the profile is inserted, in document order,
/// and a `SetProgram` that keeps a step keeps it within its own profile
/// (`StepIdFault::NotThisProfiles`). So where a `SetProgram` re-minted
/// a step of an earlier profile after a later one was inserted, the
/// earlier profile's step maps below the later profile's. Nothing in a
/// name is read along a name order, so a position that comes out
/// reordered is put back in canonical form and changes no value.
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
/// profiles they are not always, and a name that crosses reordered is
/// put back in canonical form. A node's label follows it in the edit
/// after its insert, so a refusal raised while a later node is carried
/// speaks the carried ones with their labels.
///
/// **A `new_space()` is dealt before every other node**: it reads
/// nothing, and a placement into it may precede it in document order
/// (a space reads no operation, so nothing orders its readers after
/// it).
///
/// **A placement is carried under the identity bundle, and its own
/// bundle written whole once every node is in**
/// ([`DocEdit::SetConstraints`], placements in document order): a
/// constraint reads the copy's side through the placement's own output
/// (`Doc::upstream`'s one exemption), and may read a copy inserted after
/// its placement, so it waits for both, and every bundle pins its copy,
/// so the placement is inserted pinned. A constraint changes no output,
/// so no node inserted meanwhile reads it. Every bundle crosses
/// verbatim. A read of the source's world reads `world` in the target:
/// the target's world for a split, the space inline mints for the part.
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
    world: VarId,
    edit: impl Fn(EditError) -> E,
    miss: impl Fn(RecipeNodeId, RemapMiss) -> E,
) -> Result<(NodeMap, StepMap, ReadMap), E> {
    let mut node_map = NodeMap::new();
    // Each carried operation's outputs, onto the ones its insert
    // minted at the same ports: what a carried read reads.
    let mut out_map = ReadMap::new();
    // The source's world, as the caller reads it in the target.
    out_map.insert(VarId::WORLD, world);
    let mut step_map = StepMap::new();
    let mut bundles: Vec<(RecipeNodeId, RecipeNodeId, Vec<crate::mate::Constraint>)> = Vec::new();
    let is_space = |id: &RecipeNodeId| matches!(source.node(*id), Some(Node::Space));
    let olds: Vec<RecipeNodeId> = olds
        .iter()
        .copied()
        .filter(is_space)
        .chain(olds.iter().copied().filter(|id| !is_space(id)))
        .collect();
    for (k, &old) in olds.iter().enumerate() {
        let Some(node) = source.node(old) else {
            continue;
        };
        let later = &olds[k..];
        let forward = |missing: &RemapError| match *missing {
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
        let (mut authored, fresh) = vars.author(&carried, &selects);
        if let Node::Place { constraints, .. } | Node::InstantiatePart { constraints, .. } =
            &mut authored
        {
            *constraints = crate::mate::pin::Placed::identity().bundle();
        }
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
        // A family's placement takes its bundle at once where every read
        // in it is carried, since its values are what make its copy a
        // family for the member reads carried after it.
        let family_now = !bundle.is_empty()
            && !source.node_reach(old).is_none()
            && carry_bundle(
                source,
                target,
                vars,
                (&node_map, &step_map, &out_map),
                new,
                &bundle,
            )
            .map_or_else(
                |error| match error {
                    BundleMiss::Remap(_) => Ok(false),
                    BundleMiss::Edit(error) => Err(edit(error)),
                },
                |()| Ok(true),
            )?;
        if !bundle.is_empty() && !family_now {
            bundles.push((old, new, bundle));
        }
    }
    // The bundles, now every node is in, each as one write, verbatim.
    for (old, new, bundle) in bundles {
        carry_bundle(
            source,
            target,
            vars,
            (&node_map, &step_map, &out_map),
            new,
            &bundle,
        )
        .map_err(|error| match error {
            BundleMiss::Remap(error) => miss(old, error),
            BundleMiss::Edit(error) => edit(error),
        })?;
    }
    Ok((node_map, step_map, out_map))
}

/// Why [`carry_bundle`] could not carry a bundle.
enum BundleMiss {
    /// A read the maps do not hold yet, or cannot.
    Remap(RemapMiss),
    /// The target's door refused the write.
    Edit(EditError),
}

/// **`bundle`, the constraints of a source placement, carried onto
/// `new`** as one write, each mate's selections named as their
/// sources are.
fn carry_bundle(
    source: &ProfileDoc,
    target: &mut Recording<'_, ProfileProgram>,
    vars: &mut VarCarry<'_>,
    maps: (&NodeMap, &StepMap, &ReadMap),
    new: RecipeNodeId,
    bundle: &[crate::mate::Constraint],
) -> Result<(), BundleMiss> {
    let whole = Node::Place {
        source: crate::BodyRead::plain(VarId::WORLD),
        target: VarId::WORLD,
        constraints: bundle.to_vec(),
    };
    let (carried, selects) =
        remap_with_selects(source, &whole, vars, maps).map_err(BundleMiss::Remap)?;
    let (authored, fresh) = vars.author(&carried, &selects);
    let Node::Place {
        constraints: authored,
        ..
    } = authored
    else {
        unreachable!("a carried placement carrier stays one")
    };
    let record = target
        .apply_recorded(DocEdit::SetConstraints {
            placement: new,
            constraints: authored,
            fresh: fresh.fresh.clone(),
        })
        .map_err(BundleMiss::Edit)?;
    vars.minted(fresh, &record.fresh);
    let minted: Vec<crate::mate::Mate> = target
        .doc()
        .node(new)
        .and_then(crate::mate::solve::bundle_of)
        .map(|bundle| {
            bundle
                .iter()
                .filter_map(crate::mate::Constraint::mate)
                .cloned()
                .collect()
        })
        .unwrap_or_default();
    let reads = |mate: &crate::mate::Mate| vec![mate.on, mate.to];
    for (source_mate, minted) in bundle
        .iter()
        .filter_map(crate::mate::Constraint::mate)
        .zip(&minted)
    {
        name_selections(source, target, vars, reads(source_mate), reads(minted))
            .map_err(BundleMiss::Edit)?;
    }
    Ok(())
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
    // A selection read crosses with its reader when its body does, and
    // a pose read with its reader: the remapped node keeps the source
    // id, and the insert reads it as the variable authored afresh in
    // the target (or as the one an earlier reader carried).
    let pose = |var: VarId| pose_def(source, var).is_some();
    let rd = |slot: crate::OperandSlot, var: VarId| match source.selection(var) {
        Some(select) => rd_output(slot, select.body).map(|_| var),
        None if pose(var) => Ok(var),
        None => rd_output(slot, var),
    };
    let carried = remap_node(node, &rd)?;
    // A selection's name, remapped — or, where it is a part's name read
    // in a body that reaches an instance (spelled in ids the source
    // never minted), carried verbatim: the carried instance's copy
    // carries it as the source's did.
    // A name's `Member` indices are declared variables the carry has
    // mapped, read beside the outputs.
    let reads_and_indices = vars.with_indices(out_map);
    let carried_name = |n: &StableName, body: VarId| -> Result<StableName, RemapMiss> {
        remap_name_keeping(
            n,
            node_map,
            step_map,
            &reads_and_indices,
            (source, source.reaches_instance(body)),
        )
        .map_err(|missing| RemapMiss::Name {
            name: Box::new(n.clone()),
            missing,
        })
    };
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
            .map(|n| carried_name(n, select.body))
            .collect::<Result<Vec<_>, _>>()?;
        Ok(crate::Operand::select(
            crate::Operand::Var(body),
            crate::var::Select::canonical(selection_kind(source, var), names),
        ))
    };
    for (slot, var) in node.operand_rows() {
        if pose(var) {
            // A pose crosses as the one an earlier reader carried, or as
            // its definition, each read in it carried as the read itself
            // would be, a pose not yet carried written again in place,
            // and each number re-pointed at the variables the target
            // holds.
            let mut leaf = |leaf: VarId| -> Result<Option<crate::Operand>, RemapMiss> {
                if let Some(&held) = vars.map.get(&leaf) {
                    return Ok(Some(crate::Operand::Var(held)));
                }
                if source.selection(leaf).is_some() {
                    return reselected(slot, leaf).map(Some);
                }
                if pose(leaf) {
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
                rewritten_pose_with(source, var, &mut leaf, &mut scalar, true)?,
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
                    .map(|n| carried_name(n, select.body))
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

/// **Each selection and pose a carried edit minted stands for its
/// source**, in read order, a pose's own reads with it, and a named one
/// takes its name across: `from` the source's reads, `to` the target's
/// at the same places. The walk is preorder, a pose before the reads
/// under it, so the renames are recorded in the order the source's
/// definitions are written.
fn name_selections(
    source: &ProfileDoc,
    target: &mut Recording<'_, ProfileProgram>,
    vars: &mut VarCarry<'_>,
    from: impl IntoIterator<Item = VarId>,
    to: impl IntoIterator<Item = VarId>,
) -> Result<(), EditError> {
    let mut pairs: Vec<(VarId, VarId)> = from.into_iter().zip(to).collect();
    pairs.reverse();
    while let Some((from, to)) = pairs.pop() {
        if vars.map.contains_key(&from) {
            continue;
        }
        match (pose_def(source, from), pose_def(target.doc(), to).cloned()) {
            (Some(from_def), Some(to_def)) => {
                let to_reads = to_def.reads();
                let inner: Vec<(VarId, VarId)> = from_def
                    .reads()
                    .into_iter()
                    .filter_map(|(at, &r, _, _)| {
                        to_reads
                            .iter()
                            .find(|(there, ..)| *there == at)
                            .map(|&(_, &t, _, _)| (r, t))
                    })
                    .collect();
                pairs.extend(inner.into_iter().rev());
            }
            _ if source.selection(from).is_some() => {}
            _ => continue,
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
    /// A `new_space()` is read by a cut placement and by a kept one: it
    /// cannot go with both — refused naming one reader on each side.
    SpaceStraddlesCut {
        /// The shared space's node, boxed so the refusal stays a small
        /// `Err`.
        space: Box<SpokenNode>,
        /// A cut node placed into it.
        cut_node: SpokenNode,
        /// A kept node placed into it.
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
    /// A remainder-side BODY name crosses the cut. A document's
    /// product name table deliberately carries no body rows (the
    /// product is the document's own body, not any copy's), so the
    /// rewrite of a body name through the instance could never resolve —
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
            Self::SpaceStraddlesCut {
                space,
                cut_node,
                kept_node,
            } => write!(
                f,
                "split: {cut_node}, which is cut, and {kept_node}, which is kept, are both \
                 placed into {space}, and one space cannot go with both documents. {}",
                Recourse(&format!(
                    "cut {kept_node} too, or re-target one of them into a new_space() of its \
                     own"
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
/// the instance, a name on the instance's copy, and the part copy a
/// mate-placed instance's bundle would move onto.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Uncarried {
    /// It places this many bodies, not one.
    Bodies {
        /// How many.
        count: usize,
    },
    /// Its one placement holds a pose of its own, so what stood where
    /// the read did is that placement's copy, moved.
    Posed {
        /// The placement, spoken from the document that holds it.
        placement: SpokenNode,
    },
    /// The copy is in a space other than the world, which a part does
    /// not deliver.
    OtherSpace {
        /// The placement, spoken from the document that holds it.
        placement: SpokenNode,
    },
    /// The read picks a piece by a path the part's product holds no
    /// piece at.
    NoSuchPiece {
        /// The path.
        of: Vec<VarId>,
    },
}

impl core::fmt::Display for Uncarried {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Bodies { count } => write!(
                f,
                "the world in question places {count} copies, so no one copy stands where it did"
            ),
            Self::Posed { placement } => write!(
                f,
                "{placement} places it at a pose of its own, so what stands where it did is a \
                 copy moved"
            ),
            Self::OtherSpace { placement } => write!(
                f,
                "{placement}'s copy is in a space other than the world, and a part delivers only \
                 its world"
            ),
            Self::NoSuchPiece { of } => write!(
                f,
                "the part's product holds no piece at the path {}",
                of.iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
                    .join(" / ")
            ),
        }
    }
}

/// Why [`inline`] refused (spec D-3). Typed and specific.
///
/// **Which document a node is spoken from** ([`SpokenNode`]): the
/// instance, its consumer, and a host name that derives from the
/// instance ([`Self::InstanceBodyNameReferenced`]) are the host's,
/// spoken from it. A
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
    /// **The part has no product** (A4): its product slot reads
    /// nothing, or a body that is gone, so there is no body for the new
    /// placement to copy.
    UnplaceablePart {
        /// The instance.
        instance: SpokenNode,
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
    /// The referenced document holds a named pose nothing in it reads.
    /// No door declares a pose: one crosses with the first spliced edit
    /// that reads it, and this one has none, so inline refuses rather
    /// than drop it.
    UnreadPose {
        /// The pose, spoken from the referenced document.
        var: SpokenVar,
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
    /// A host reference names the instance's own OUTPUT BODY. The
    /// spliced recipe has no single node whose body corresponds to the
    /// instance's placed product, so the reference cannot re-anchor.
    InstanceBodyNameReferenced {
        /// The instance-body name.
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
            Self::UnplaceablePart { instance } => write!(
                f,
                "inline: {instance}'s part has no product: its slot reads nothing, or a body \
                 that is gone. {}",
                Recourse(&format!(
                    "point {instance} (UpdateReference) at a version whose product slot reads \
                     a body"
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
            Self::VarNameConflict { name } => write!(
                f,
                "inline: both documents hold a variable named {name}, and the referenced \
                 document's crosses as a variable of its own. {}",
                Recourse(&format!(
                    "rename the {name} of one of the two documents (RenameVar), then inline"
                ))
            ),
            Self::UnreadPose { var } => write!(
                f,
                "inline: the referenced document holds the pose {var}, which nothing in it \
                 reads, and a pose crosses only with a reader. {}",
                Recourse(&format!(
                    "in the referenced document, read {var} or delete it (DeleteVar), point this \
                     instance at that version (UpdateReference), then inline"
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
            Self::InstanceBodyNameReferenced { name } => write!(
                f,
                "inline: {name} names the instance's own output body, which no single \
                 spliced node corresponds to. {}",
                Recourse(
                    "clear what this document sets on that name (ClearAppearance), or rebind it \
                     to another node's body (Rebind), then inline"
                )
            ),
            Self::NameOnDroppedStep { name, step } => write!(
                f,
                "inline: {name} spells a piece of the profile step {step}, which no profile \
                 of the referenced document draws any more. {}",
                Recourse(STRANDED_IN_THE_PART)
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
            | EditError::PickPath { .. }
            | EditError::ProductRead { .. }
            | EditError::SelectionNameKind { .. }
            | EditError::PoseShape { .. }
            | EditError::IndexedRead { .. }
            | EditError::SetMembersOnNonList { .. }
            | EditError::SetLoftSectionsOnNonLoft { .. }
            | EditError::LoftShape { .. }
            | EditError::SetProgramOnNonProfile { .. }
            | EditError::StepIdsRefused { .. }
            | EditError::LoftSectionsSpelled { .. }
            | EditError::LoftSectionCount { .. }
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
            | EditError::Index { .. }
            | EditError::IndicesUnrelated { .. }
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
            | EditError::SpaceMismatch { .. }
            | EditError::ConstraintOnNonPlacement { .. }
            | EditError::UnknownConstraint { .. }
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
    /// The placement the instance became: the spliced copies of the
    /// part's world placements, placed into the instance's target under
    /// its bundle.
    pub placement: RecipeNodeId,
    /// The space the part's world became.
    pub space: VarId,
}

// ---- Name and node remapping ----

/// Rewrites every LOCAL node id in `name` through `map` — the minting
/// node, embedded operand names, and discriminator partners: exactly
/// the id set [`derivation_nodes`] reads, because that is the set that
/// belongs to THIS document's id space. A name an instance's copy
/// carries is spelled in its part's ids, which this walk cannot tell
/// from the host's ([`remap_name_keeping`] keeps those). The path is walked by `StableName::rewrite_path`, which
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
/// [`RemapError::Unmapped`] at the first local id the maps lack.
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

/// **[`remap_name`], keeping a part's ids as they are**: a copy
/// carries its source's names, so a name read in a copy of an instance
/// (`parted`) holds its part's node and step ids, which `source` never
/// minted and no map of its holds; those cross verbatim while every
/// other id still maps.
///
/// # Errors
///
/// The first id the maps lack that `source` minted.
pub(crate) fn remap_name_keeping<P: crate::ProfilePayload>(
    name: &StableName,
    map: &NodeMap,
    steps: &StepMap,
    reads: &ReadMap,
    (source, parted): (&Doc<P>, bool),
) -> Result<StableName, RemapError> {
    let mut nodes = std::borrow::Cow::Borrowed(map);
    let mut kept_steps = std::borrow::Cow::Borrowed(steps);
    loop {
        match remap_name(name, &nodes, &kept_steps, reads) {
            Err(RemapError::Unmapped(Unmapped::Node(id)))
                if parted && !source.has_minted(id) && !nodes.contains_key(&id) =>
            {
                nodes.to_mut().insert(id, id);
            }
            Err(RemapError::Unmapped(Unmapped::Step(step)))
                if parted && !source.mint().has_step(step) && !kept_steps.contains_key(&step) =>
            {
                kept_steps.to_mut().insert(step, step);
            }
            done => return done,
        }
    }
}

/// The half of [`remap_name`] that a name's KIND is not part of: the
/// minting node and the role path, rewritten through `map` and
/// `steps`. The kind is read, never written: the path's canonical form
/// depends on it (a junction is a vertex's).
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

    // An index is declared, not an output; the carry maps it beside
    // the reads ([`remap_with_selects`]).
    fn index(&mut self, index: VarId) -> Result<VarId, Self::Error> {
        Ok(self.2.get(&index).copied().ok_or(Unmapped::Read(index))?)
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
    /// they lack. The two are not
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
    // which cross as every slot does.
    let br = |seat: crate::OperandSlot, read: &crate::BodyRead<VarId>| {
        Ok::<_, RemapMiss>(crate::BodyRead {
            read: rd(seat, read.read)?,
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
        // A free profile reads nothing; a drawn one's frame and traced
        // faces cross as every read does. Its step ids do NOT cross: the
        // other document's insert door mints its own, and the names that
        // spell them cross through the step map read off that minting
        // ([`carry`]).
        Node::Profile(p) => Node::Profile(ProfileProgram {
            loops: p.loops.clone(),
            ids: Vec::new(),
            on: p
                .on
                .as_ref()
                .map(|on| on.try_map(&mut |seat, &var| rd(seat, var)))
                .transpose()?,
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
        Node::Loft {
            first,
            then,
            v_degree,
        } => Node::Loft {
            first: rd(crate::OperandSlot::Section(0), *first)?,
            then: (1u32..)
                .zip(then)
                .map(|(i, section)| {
                    section.try_map(|&p| rd(crate::OperandSlot::Section(i), p), &mut |&v| Ok(v))
                })
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
        Node::Mirror { body, plane } => Node::Mirror {
            body: br(crate::OperandSlot::Body, body)?,
            plane: plane.try_map(|plane| rd(crate::OperandSlot::Plane, *plane))?,
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
        Node::Assemble { members } => Node::Assemble {
            members: members.try_map(&br)?,
        },
        Node::Place {
            source,
            target,
            constraints,
        } => Node::Place {
            source: br(crate::OperandSlot::Source, source)?,
            target: rd(crate::OperandSlot::Space, *target)?,
            constraints: bundle(constraints)?,
        },
        // The world is every document's own, never carried; a
        // constraint reading it names no id.
        Node::World => Node::World,
        Node::Space => Node::Space,
        // The reference crosses verbatim (the function's docs say why),
        // and its bundle as a placement's does.
        Node::InstantiatePart {
            doc_ref,
            interface,
            target,
            constraints,
        } => Node::InstantiatePart {
            doc_ref: *doc_ref,
            interface: interface.clone(),
            target: rd(crate::OperandSlot::Space, *target)?,
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
        Node::Assertion(crate::measure::Claim::Value {
                    value,
                    bound,
                    relation,
                }) => Node::Assertion(crate::measure::Claim::Value {
                value: match rd(crate::OperandSlot::Input, *value) {
                    Err(RemapMiss::Read { output: false, .. }) => *value,
                    Err(RemapMiss::Read { var, .. }) => return Err(RemapMiss::PayloadRead { var }),
                    read => read?,
                },
                bound: *bound,
                relation: *relation,
            }),
        // The seats re-point as a measure's references do; the bounds
        // are slot variables, carried as a value claim's bound is.
        Node::Assertion(crate::measure::Claim::Clearance { a, b, bounds }) => Node::Assertion(crate::measure::Claim::Clearance {
                a: rd(crate::OperandSlot::Cleared(0), *a)?,
                b: rd(crate::OperandSlot::Cleared(1), *b)?,
                bounds: bounds.clone(),
            }),
    })
}

/// The variables a node's slots and payload read. The expressions no
/// slot addresses count too: a measured bound reading a variable is
/// exactly as much a reason to carry that variable into a split part
/// as an extrude's distance is. A pose the node reads is a variable it
/// reads, and so is each pose and number that pose is defined over;
/// the selections and outputs under it cross with their bodies.
fn node_var_reads(doc: &ProfileDoc, node: &Node<ProfileProgram>) -> BTreeSet<VarId> {
    let is_pose = |var: VarId| pose_def(doc, var).is_some();
    // A reader of a defined variable reads what its definition reads.
    let mut through: BTreeSet<VarId> = BTreeSet::new();
    let mut frontier: Vec<VarId> = node.exprs().into_iter().copied().collect();
    frontier.extend(
        node.operand_rows()
            .into_iter()
            .map(|(_, var)| var)
            .filter(|&var| is_pose(var)),
    );
    while let Some(var) = frontier.pop() {
        if !through.insert(var) {
            continue;
        }
        match pose_def(doc, var) {
            Some(def) => {
                frontier.extend(
                    def.reads()
                        .into_iter()
                        .map(|(_, &r, _, _)| r)
                        .filter(|&r| is_pose(r)),
                );
                frontier.extend(def.scalars().into_iter().map(|(_, &s)| s));
            }
            None => frontier.extend(doc.definition_reads(var)),
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

impl VarCarry<'_> {
    /// `reads` with the carried index variables beside them: a name's
    /// `Member` indices are declared variables, mapped by the carry
    /// rather than read at a port.
    fn with_indices(&self, reads: &ReadMap) -> ReadMap {
        reads
            .iter()
            .chain(self.map.iter().filter(|&(&var, _)| {
                self.source
                    .var(var)
                    .is_some_and(|held| held.def().index().is_some())
            }))
            .map(|(&from, &to)| (from, to))
            .collect()
    }
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
            VarDef::Index(index) => {
                let read = |var: VarId| self.reader(anonymous, var, Dimension::Count);
                let count = Formula::from(&index.count)
                    .substitute_vars(&mut |var| {
                        Some(self.reader(anonymous, var, kind_of(self.source, var)?))
                    })
                    .unwrap_or_else(|fault| {
                        unreachable!(
                            "a reader re-pointed at its own kind keeps every dimension: {fault}"
                        )
                    });
                VarDecl::Index {
                    count,
                    within: index.within.map(read),
                }
            }
            VarDef::Output { .. } => {
                unreachable!("an output crosses with its node, minted by the target's insert")
            }
            VarDef::Select(_) => {
                unreachable!("a selection crosses with its reader, minted by the target's insert")
            }
            VarDef::Pick(_) => {
                unreachable!("a pick crosses with its reader, minted by the target's insert")
            }
            VarDef::Pose(_) => {
                unreachable!("a pose crosses with its reader, minted by the target's insert")
            }
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

/// **What a remainder read of the cut is re-pointed to**: one piece of
/// the instance's copy, by the cut world placement whose copy it is,
/// and the path inside that copy a pick of it read.
#[derive(Debug, Clone)]
struct SourceKey {
    /// The cut world placement.
    placement: RecipeNodeId,
    /// The path inside its copy a pick read, empty for the copy whole.
    within: Vec<VarId>,
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
    // Each node's space reads, the world's left out: the world is no
    // cut's.
    let node_spaces: BTreeMap<RecipeNodeId, Vec<(crate::OperandSlot, RecipeNodeId)>> = doc
        .ids()
        .into_iter()
        .filter_map(|id| {
            let reads: Vec<_> = doc
                .spaces_read(doc.node(id)?)
                .into_iter()
                .filter(|&(_, at)| at != RecipeNodeId::WORLD)
                .collect();
            (!reads.is_empty()).then_some((id, reads))
        })
        .collect();
    // A `new_space()` goes where its readers go (module docs): one every
    // reader of which is cut joins the cut, named or not.
    let mut space_readers: BTreeMap<RecipeNodeId, Vec<RecipeNodeId>> = BTreeMap::new();
    for (&id, reads) in &node_spaces {
        for &(_, space) in reads {
            if matches!(doc.node(space), Some(Node::Space)) {
                space_readers.entry(space).or_default().push(id);
            }
        }
    }
    // One read on both sides is refused beside the variables, once the
    // closure has spoken.
    let mut joined = cut.clone();
    let mut shared: BTreeMap<RecipeNodeId, (RecipeNodeId, RecipeNodeId)> = BTreeMap::new();
    for (&space, readers) in &space_readers {
        let cut_reader = readers.iter().find(|r| cut.contains(r));
        let kept_reader = readers.iter().find(|r| !cut.contains(r));
        match (cut_reader, kept_reader) {
            (Some(&cut_node), Some(&kept_node)) => {
                shared.insert(space, (cut_node, kept_node));
            }
            (Some(_), None) => {
                joined.insert(space);
            }
            _ => {}
        }
    }
    // A joined space is no node the author chose, so the cut's first
    // node a refusal names is the selection's.
    let selected = cut;
    let cut = &joined;
    // Which copies are the world's: a part delivers only its world
    // (A10), so a cut copy in another space crosses nowhere.
    let opts = crate::eval::EvalOptions {
        resolver: resolver.cloned(),
        ..crate::eval::EvalOptions::default()
    };
    // A placement's space is its target, read off the recipe (D10:
    // spaces are decided from the recipe, never from a value).
    let evaluation =
        crate::eval::evaluate::<f64>(doc, None, &crate::CancelToken::new(), &opts, tol);
    let poses = crate::mate::SolvedPoses::of(doc, &evaluation);
    let world = |placement: RecipeNodeId| {
        doc.node(placement)
            .and_then(crate::mate::solve::bundle_of)
            .is_some()
            && doc.node_space(placement) == Some(VarId::WORLD)
    };
    // Whether a copy stands where its body did: its bundle is the
    // identity bundle, read off the recipe.
    let places_at_identity = |placement: RecipeNodeId| poses.at_identity(placement);
    // A cut name re-anchors as the part's product spells it: a name a
    // cut world placement mints (its copy's own, or an instance's) as
    // it is, and a name of a body a cut world `Place` copies, which
    // that placement's copy carries verbatim. The copy holds the names its
    // body minted, those a transform below it carried whole (N1's
    // `Whole` edge), and a family member's, by the member its `Member`
    // segment names, which no value decides. A split half's pick is not
    // followed, since which half holds a split's name is the geometry's
    // answer. A name of material no cut world placement places, or two
    // do, names nothing the instance carries.
    // With `body`, the copy is the one a cut placement makes of that body
    // (a selection's): the placement copies it (the gate's copy rule,
    // `assembly::copy_face`), and the name heads there. The
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
        let holds = |placed: Option<RecipeNodeId>, body: Option<RecipeNodeId>| -> bool {
            let at = match (placed, body) {
                (placed, None) => placed,
                (Some(node), Some(body)) if node == body => Some(node),
                (Some(_) | None, Some(_)) => None,
            };
            at == Some(name.node)
        };
        // A name read at a cut placement's own copy is its source's, as
        // the copy carries it: walked from that source.
        let mut placing = cut.iter().copied().filter(|&p| {
            world(p)
                && matches!(doc.node(p), Some(Node::Place { source, .. })
                if !source.is_member()
                    && holds(
                        doc.operation_of(source.read),
                        body.filter(|&at| at != p),
                    ))
        });
        match (placing.next(), placing.next()) {
            (Some(placement), None) => Some((placement, true)),
            _ => None,
        }
    };
    // The instance whose part's names `body` carries verbatim: its own
    // copy, or a placement's copy of it (a copy's names are its
    // source's). A name read there is spelled in the part's ids, never
    // this document's, so the cut is decided by that instance.
    let part_names_at = |mut at: RecipeNodeId| -> Option<RecipeNodeId> {
        loop {
            match doc.node(at)? {
                Node::InstantiatePart { .. } => return Some(at),
                Node::Place { source, .. } => at = doc.operation_of(source.read)?,
                _ => return None,
            }
        }
    };
    let part_names = |body: VarId| part_names_at(doc.operation_of(body)?);
    // A copy's own body row is minted at the copying node, so a name
    // of that node is this document's.
    let carrier_part = |carrier: NameCarrier<'_>| -> Option<RecipeNodeId> {
        let body = match carrier {
            NameCarrier::Store { body, .. } => body,
            NameCarrier::Select { var, .. } => doc.selection(var)?.body,
            NameCarrier::Payload { .. } => return None,
        };
        (doc.operation_of(body) != Some(carrier.name().node))
            .then(|| part_names(body))
            .flatten()
    };
    // A name `at`'s copy holds: one a cut world placement's copy
    // carries ([`home`]), or a part's name a world instance's copy
    // carries verbatim.
    let holds = |name: &StableName, at: RecipeNodeId| {
        home(name, Some(at)).is_some()
            || (name.node != at
                && world(at)
                && matches!(doc.node(at), Some(Node::InstantiatePart { .. })))
    };
    // **The piece a remainder whole read of the cut re-points to**: at a
    // body seat, a read of a cut world placement's copy or of a pick of
    // it, or a read of a body one cut world `Place` copies at the
    // identity, where its copy stands. `None` for a read that does not
    // cross, which the closure rule refuses.
    let heir_of = |consumer: &Node<ProfileProgram>,
                   slot: crate::OperandSlot,
                   var: VarId|
     -> Result<Option<SourceKey>, Uncarried> {
        let Some(read) = consumer.body_read(slot) else {
            return Ok(None);
        };
        let Some(at) = doc.operation_of(var).filter(|at| cut.contains(at)) else {
            return Ok(None);
        };
        match doc.node(at) {
            Some(Node::Place { .. } | Node::InstantiatePart { .. }) => {
                let within = match doc.pick(var) {
                    Some(pick) => pick.of.clone(),
                    None if !read.is_indexed()
                        && doc
                            .var(var)
                            .is_some_and(|v| v.kind() == crate::VarKind::Body) =>
                    {
                        Vec::new()
                    }
                    None => return Ok(None),
                };
                if !world(at) {
                    return Err(Uncarried::OtherSpace {
                        placement: doc.spoken(at),
                    });
                }
                Ok(Some(SourceKey {
                    placement: at,
                    within,
                }))
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
                            && matches!(doc.node(p), Some(Node::Place { source, .. })
                                if !source.is_member() && source.read == var)
                    })
                    .collect();
                match copies.as_slice() {
                    [] => Ok(None),
                    [one] if places_at_identity(*one) => Ok(Some(SourceKey {
                        placement: *one,
                        within: Vec::new(),
                    })),
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
    // instance's copy, those selections re-pointed (`crossing_leaves`).
    let mut crossing_poses: Vec<(RecipeNodeId, crate::OperandSlot, VarId)> = Vec::new();
    let mut crossing_leaves: Vec<(RecipeNodeId, VarId)> = Vec::new();
    let mut heirs: BTreeMap<(RecipeNodeId, crate::OperandSlot), SourceKey> = BTreeMap::new();
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
                                    select.names.iter().all(|name| holds(name, at))
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
                            cut.contains(&at) && select.names.iter().all(|name| holds(name, at))
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
        // A space read crosses nowhere: no copy carries a space.
        let reads = doc
            .upstream_of(consumer, node)
            .into_iter()
            .filter(|input| !carried.contains(input))
            .map(|input| (None, input))
            .chain(
                node_spaces
                    .get(&consumer)
                    .into_iter()
                    .flatten()
                    .filter(|(_, at)| !shared.contains_key(at))
                    .map(|&(slot, input)| (Some(slot), input)),
            );
        for (space_slot, input) in reads {
            if consumer_is_cut != cut.contains(&input) {
                let slot = space_slot.or_else(|| {
                    node.operand_rows().into_iter().find_map(|(slot, var)| {
                        doc.read_ports(var)
                            .iter()
                            .any(|&(at, _)| at == input)
                            .then_some(slot)
                    })
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
    if let Some((&space, &(cut_node, kept_node))) = shared.first_key_value() {
        return Err(SplitError::SpaceStraddlesCut {
            space: Box::new(doc.spoken(space)),
            cut_node: doc.spoken(cut_node),
            kept_node: doc.spoken(kept_node),
        });
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
                let outside = match carrier_part(carrier) {
                    Some(instance) => (!cut.contains(&instance)).then_some(instance),
                    None => derivation_nodes(name)
                        .into_iter()
                        .filter(|id| !cut.contains(id))
                        .min_by_key(|id| (doc.node(*id).is_none(), *id)),
                };
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
        // A part's ids in the name are no node of either side: a copy
        // carries its source's names, so they cross as they are.
        let ids: BTreeSet<RecipeNodeId> = derivation_nodes(name)
            .into_iter()
            .filter(|&id| doc.has_minted(id))
            .collect();
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
        // A part's name moves with the instance whose copy carries it,
        // re-attached below where that instance is cut.
        if carrier_part(carrier).is_some() {
            continue;
        }
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
    let mut part = Recording::start(&empty, tol);
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
    // anonymous one, and a pose, crosses with the first carried edit
    // that reads it (`VarCarry`).
    let mut vars = VarCarry::new(doc);
    for id in doc
        .definition_order()
        .into_iter()
        .filter(|id| moving.contains(id) && pose_def(doc, *id).is_none())
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
        VarId::WORLD,
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
                    read: Box::new(crate::Operand::Var(var)),
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
    }) && let Some(&first) = doc.ids().iter().find(|id| selected.contains(id))
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
    // The part's product (D10 `[operations]`): its one world copy, or
    // the `⊔` of its world copies in document order, each one arrival.
    let world_copies: Vec<VarId> = in_order
        .iter()
        .filter(|&&id| {
            matches!(
                doc.node(id),
                Some(Node::Place { .. } | Node::InstantiatePart { .. })
            ) && world(id)
        })
        .map(|id| {
            let Some(&carried) = node_map.get(id) else {
                unreachable!("a cut placement is carried")
            };
            part.doc()
                .output(carried, 0)
                .unwrap_or_else(|| unreachable!("a placement defines its copy"))
        })
        .collect();
    let assembled: Option<RecipeNodeId> = match world_copies.as_slice() {
        [] => None,
        [one] => {
            part_apply(
                &mut part,
                DocEdit::SetProduct {
                    product: Some(crate::Operand::Var(*one)),
                },
            )?;
            None
        }
        several => {
            let node = part
                .insert(Node::Assemble {
                    members: crate::Bodies::Spelled(
                        several
                            .iter()
                            .map(|&copy| crate::BodyRead::plain(crate::Operand::Var(copy)))
                            .collect(),
                    ),
                })
                .map_err(part_refused)?;
            part_apply(
                &mut part,
                DocEdit::SetProduct {
                    product: Some(crate::Operand::Node(node)),
                },
            )?;
            Some(node)
        }
    };
    let pin = content_pin(part.doc(), tol).map_err(|error| SplitError::Pin {
        error: Box::new(error),
    })?;
    // The copy, in the part, of the cut world placement `placement`.
    let part_copy = |placement: RecipeNodeId| -> VarId {
        let Some(&carried) = node_map.get(&placement) else {
            unreachable!("a cut placement is carried")
        };
        part.doc()
            .output(carried, 0)
            .unwrap_or_else(|| unreachable!("a placement defines its copy"))
    };
    let names_map = vars.with_indices(&read_map);
    // A cut name as the part's product spells it ([`home`] above): the
    // name its copy carries, keyed by that copy's read where the
    // product is a `⊔`.
    let in_world =
        |name: &StableName, body: Option<RecipeNodeId>| -> Result<StableName, SplitError> {
            let outside = || SplitError::NameOutsidePartWorld {
                name: doc.spoken_name(name),
            };
            // A cut instance's copy carries its part's names verbatim,
            // and so does this part's copy of it.
            let foreign = body.filter(|&at| name.node != at && part_names_at(at).is_some());
            let (of, placement) = match foreign {
                Some(at) => (name.clone(), world(at).then_some(at).ok_or_else(outside)?),
                // A part's ids inside the name (a pattern copy of an
                // instance's face) cross as they are.
                None => (
                    remap_name_keeping(name, &node_map, &step_map, &names_map, (doc, true))
                        .map_err(|missing| SplitError::straddles(doc, name, missing))?,
                    home(name, body).ok_or_else(outside)?.0,
                ),
            };
            let Some(assembled) = assembled else {
                return Ok(of);
            };
            Ok(StableName {
                kind: name.kind,
                node: assembled,
                path: vec![RoleSeg::From {
                    read: part_copy(placement),
                    of: NameRef::new(of),
                }],
            })
        };
    // The pick path, in the part's product, of the piece a crossing
    // whole read reads: empty for the product whole.
    let part_path = |key: &SourceKey| -> Vec<VarId> {
        let within = key.within.iter().map(|var| match doc.node(key.placement) {
            // A cut `Place`'s source is carried; an instance's path is
            // its part's, which crosses whole.
            Some(Node::Place { .. }) => read_map.get(var).copied().unwrap_or(*var),
            _ => *var,
        });
        match assembled {
            Some(_) => core::iter::once(part_copy(key.placement))
                .chain(within)
                .collect(),
            None => within.collect(),
        }
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
            // A part's name is inside where the copy carrying it is.
            let inside = |head: &crate::node::SitedFace| {
                if head.name.node != head.at && part_names_at(head.at).is_some() {
                    cut.contains(&head.at)
                } else {
                    derivation_nodes(&head.name).is_subset(cut)
                }
            };
            let (outer, inner) = match (inside(&a), inside(&b)) {
                (false, true) => (&a, &b),
                (true, false) => (&b, &a),
                _ => continue,
            };
            // An `outer` is a name of this document's: a part's name, read
            // in a kept copy of an instance, is spelled in the part's ids
            // and has no spelling a record can hold (issue
            // `a-part-name-in-the-host-is-spelled-in-the-parts-ids`), so
            // that crossing goes unrecorded.
            if outer.name.node != outer.at && part_names_at(outer.at).is_some() {
                continue;
            }
            // The part-side reference is stored in the PART's own names:
            // that is what the part's product answers to, and what
            // re-verification resolves against. `classify` above already
            // refused a name that straddles, so the remap is total here —
            // and it refuses typed rather than assuming so.
            let Ok(inner) = FaceName::new(in_world(&inner.name, Some(inner.at))?) else {
                unreachable!("a remap keeps a face a face")
            };
            crossings.push(InterfaceCrossing::Mate {
                outer: outer.name.clone(),
                inner,
            });
        }
    }

    // ---- The remainder, as recorded edits from the input ----
    // The remainder inserts one instance pinned to the world, rebinds,
    // re-points and deletes: no edit of it levers a pair.
    let mut remainder = Recording::start(doc, tol);
    let rem_refused = |error| SplitError::RemainderEdit {
        error: Box::new(error),
    };
    let rem_apply = |remainder: &mut Recording<'_, ProfileProgram>,
                     edit: DocEdit<ProfileProgram>|
     -> Result<(), SplitError> {
        remainder.apply(edit).map(|_| ()).map_err(rem_refused)
    };
    // The instance is a placement (D10), standing where its part's
    // world stands by the identity bundle ([`crate::mate::stand`]),
    // which reads nothing of the part, so the part's world copies stand
    // where the cut's did.
    let doc_ref = DocRef { id: part_id, pin };
    let instance = remainder
        .insert(Node::instantiate_part_with(
            doc_ref,
            InterfaceRecord { crossings },
            crate::Operand::Var(VarId::WORLD),
            crate::mate::stand().bundle(),
        ))
        .map_err(rem_refused)?;
    let copies = remainder
        .doc()
        .output(instance, 0)
        .unwrap_or_else(|| unreachable!("an instance defines its copy"));
    // A product the cut took is the instance's copy: the part's product
    // is the cut's world.
    if let Some(var) = doc.product()
        && doc.operation_of(var).is_some_and(|at| cut.contains(&at))
    {
        rem_apply(
            &mut remainder,
            DocEdit::SetProduct {
                product: Some(crate::Operand::Var(copies)),
            },
        )?;
    }
    // The instance's copy carries its part's product names verbatim.
    let in_part = |from: &StableName, body: Option<RecipeNodeId>| in_world(from, body);
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
                let Some(key) = heirs.get(&(node, slot)) else {
                    unreachable!("a crossing whole read has its heir")
                };
                let path = part_path(key);
                crate::SlotValue::Read(if path.is_empty() {
                    crate::Operand::Var(copies)
                } else {
                    crate::Operand::pick(crate::Operand::Var(copies), path)
                })
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
    // that reads the cut authored in the instance's copy as a crossing
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
        // A copy carries its source's names, so a name the instance's
        // copy carries as the cut did needs no repair.
        if &to == from {
            continue;
        }
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

/// **`var` as it was written, with some reads replaced**: each read
/// `leaf` answers for is that answer; each pose written in place is
/// written again over its reads; any other read as
/// [`Doc::written_operand`] spells it.
fn rewritten_pose<P, E>(
    doc: &Doc<P>,
    var: VarId,
    leaf: &mut impl FnMut(VarId) -> Result<Option<crate::Operand>, E>,
) -> Result<crate::Operand, E> {
    rewritten_pose_with(doc, var, leaf, &mut |formula| Ok(formula), false)
}

/// [`rewritten_pose`] with each scalar's written formula mapped by
/// `scalar` — a carry's, re-pointing the named variables it reads —
/// and, where `named`, each named pose `leaf` does not answer for
/// written in place too: a carry's, whose target holds no such pose
/// yet.
fn rewritten_pose_with<P, E>(
    doc: &Doc<P>,
    var: VarId,
    leaf: &mut impl FnMut(VarId) -> Result<Option<crate::Operand>, E>,
    scalar: &mut impl FnMut(Formula) -> Result<Formula, E>,
    named: bool,
) -> Result<crate::Operand, E> {
    if let Some(operand) = leaf(var)? {
        return Ok(operand);
    }
    match doc.var(var).map(crate::Var::def) {
        Some(crate::var::VarDef::Pose(def)) if named || doc.var_name(var).is_none() => {
            // `try_map` calls the two closures one after the other, never
            // together, so the one `scalar` is lent to each in turn.
            let scalar = std::cell::RefCell::new(scalar);
            let mut read = |_, &r: &VarId| {
                rewritten_pose_with(doc, r, leaf, &mut **scalar.borrow_mut(), named)
            };
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

/// `var`'s definition, where `doc` holds it as a pose.
fn pose_def<P>(doc: &Doc<P>, var: VarId) -> Option<&crate::pose::PoseDef> {
    doc.var(var).and_then(|held| held.def().pose())
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

/// Splices the document `instance` references into `doc` and rewrites
/// the instance as a placement — the inverse of [`split`], by one rule
/// (ASSEMBLY A4). It mints a `new_space()`, copies the part's nodes into
/// the host verbatim with the part's world read as that space, and
/// writes one [`Node::Place`] of the spliced copies of the part's world
/// placements, as a list, into the instance's target under the
/// instance's own bundle, verbatim; the instance is deleted. No motion
/// is composed, every variable keeps driving its copy, and no landing
/// refuses for its numbers. The pin resolves through `resolver` (a
/// stale pin is the resolver's typed `PinMismatch`, never a silent
/// retarget); ids remap into fresh host mints in the part's document
/// order. Every host read of the instance's copy reads the new
/// placement's, a selection of them authored afresh there with each
/// part name re-anchored onto the placement's copy, and a
/// side of the instance's own bundle reads the spliced copy it stood
/// on. Pure — `doc` is untouched; undo is keeping it.
///
/// The instance's INTERFACE RECORD dissolves here (ASM-R2b D-4, the
/// inverse of split's populate): each crossing's part-side reference
/// re-anchors onto the spliced copy, which is exactly what the
/// wrapped-name rebind below does to the constraint that declared it —
/// so once every crossing's inner name is confirmed to land locally,
/// the record has no remaining content and goes with the deleted
/// instance. Confirmed, not assumed: a crossing that does not re-anchor
/// refuses [`InlineError::StrandedPartName`] rather than being dropped
/// with the node.
///
/// # Errors
///
/// Every arm of [`InlineError`].
#[allow(clippy::too_many_lines)] // one linear pass per A4 rule, each short
pub fn inline(
    doc: &ProfileDoc,
    instance: RecipeNodeId,
    resolver: &std::sync::Arc<dyn PartResolver>,
    tol: Tol,
) -> Result<InlineOutcome, InlineError> {
    let Some(node) = doc.node(instance) else {
        return Err(InlineError::UnknownNode {
            id: SpokenNode::absent(instance),
        });
    };
    let Node::InstantiatePart {
        doc_ref,
        interface,
        target,
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
    // nothing to be re-pointed at, and refuses here, at this door.
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
    // The part's product: what the instance copies, and what the new
    // placement copies.
    let Some(product) = part
        .product()
        .filter(|&read| part.operation_of(read).is_some())
    else {
        return Err(InlineError::UnplaceablePart {
            instance: doc.spoken(instance),
        });
    };
    let copies = doc
        .output(instance, 0)
        .unwrap_or_else(|| unreachable!("an instance defines its copy"));
    // The picks of the instance's copy, which re-point to picks of the
    // new placement's.
    let picks: BTreeSet<VarId> = doc
        .definition_order()
        .into_iter()
        .filter(|&var| doc.pick(var).is_some_and(|pick| pick.placement == copies))
        .collect();
    let reads_instance = |body: VarId| body == copies || picks.contains(&body);
    // The instance's readers: every operand of the host that reads its
    // copy, whole, a piece of it by a pick, or a selection of either;
    // and every pose read at a seat (a mate's side) reaching a
    // selection of them, written afresh over the new placement's copy.
    let mut readers: Vec<(RecipeNodeId, crate::OperandSlot, VarId)> = Vec::new();
    let mut pose_readers: Vec<(RecipeNodeId, crate::OperandSlot, VarId)> = Vec::new();
    for id in doc.ids() {
        if id == instance {
            continue;
        }
        let Some(reader) = doc.node(id) else { continue };
        for (slot, var) in reader.operand_rows() {
            if let Some(leaves) = pose_leaves(doc, var) {
                if leaves
                    .iter()
                    .any(|&leaf| doc.selection(leaf).is_some_and(|s| reads_instance(s.body)))
                {
                    pose_readers.push((id, slot, var));
                }
                continue;
            }
            let body = doc.selection(var).map_or(var, |select| select.body);
            if reads_instance(body) {
                readers.push((id, slot, var));
            }
        }
    }
    // The instance's own body name has no local correspondent.
    for carrier in doc.name_carriers() {
        let name = carrier.name();
        if name.node == instance && name.path == vec![RoleSeg::OutputBody] {
            return Err(InlineError::InstanceBodyNameReferenced {
                name: doc.spoken_name(name),
            });
        }
    }
    let mut current = Recording::start(doc, tol);
    let refused = |error| InlineError::Edit {
        error: Box::new(error),
    };
    let step = |current: &mut Recording<'_, ProfileProgram>,
                edit: DocEdit<ProfileProgram>|
     -> Result<(), InlineError> { current.apply(edit).map(|_| ()).map_err(refused) };
    // The part's world, minted afresh in the host.
    let space = current
        .apply_recorded(DocEdit::InsertNode {
            node: Box::new(Node::Space),
            fresh: Vec::new(),
        })
        .map_err(refused)?
        .outputs
        .first()
        .copied()
        .unwrap_or_else(|| unreachable!("a space defines its space"));
    // Every part variable crosses as an id the host mints, never one
    // it holds: a named one declared here, refusing a name the host
    // holds whatever either's value; an anonymous one, and a pose,
    // with the first spliced edit that reads it (`VarCarry`).
    let mut vars = VarCarry::new(&part);
    let read: BTreeSet<VarId> = part
        .ids()
        .iter()
        .filter_map(|&id| part.node(id))
        .flat_map(|node| node_var_reads(&part, node))
        .collect();
    for id in part.definition_order() {
        let Some(name) = part.var_name(id) else {
            continue;
        };
        if doc.var_named(name.as_str()).is_some() {
            return Err(InlineError::VarNameConflict { name: name.clone() });
        }
        // An output crosses with its node (`carry`), and a pose with
        // its first reader.
        if part.var(id).is_some_and(|var| var.def().output().is_some()) {
            continue;
        }
        if pose_def(&part, id).is_some() {
            if !read.contains(&id) {
                return Err(InlineError::UnreadPose {
                    var: part.spoken_var(id),
                });
            }
            continue;
        }
        vars.declare(&mut current, id, name.clone())
            .map_err(refused)?;
    }
    // The part's nodes in its document order, each under the id the
    // host's insert door mints for it, the world aside: the part's world
    // is the space just minted.
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
        space,
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
                    read: Box::new(crate::Operand::Var(var)),
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
    let Some(&spliced) = read_map.get(&product) else {
        unreachable!("the part's live product {product:?} is spliced with its operation")
    };
    // A name the instance's copy carries is its part's, verbatim: it
    // lands on the spliced name, every id of the part read through the
    // maps and every id of the host kept.
    let mut host_nodes = node_map.clone();
    for id in doc.ids() {
        host_nodes.entry(id).or_insert(id);
    }
    let mut host_steps = step_map.clone();
    for id in doc.ids() {
        if let Some(Node::Profile(program)) = doc.node(id) {
            for &step in program.ids.iter().flatten() {
                host_steps.entry(step).or_insert(step);
            }
        }
    }
    // A name's `Member` indices are the part's declared variables, mapped
    // by the carry beside its outputs.
    let names_map = vars.with_indices(&read_map);
    let mut host_reads = names_map.clone();
    for var in doc.definition_order() {
        host_reads.entry(var).or_insert(var);
    }
    // A name the part's own copy of an instance carries is that
    // instance's part's, spelled in ids the part never minted: it
    // crosses verbatim, as it entered the part.
    // Only those ids cross as they are: every id the part minted is
    // still read through the maps, so a part's `⊔` naming another
    // part's face lands on the spliced `⊔` and read.
    let parted = part.reaches_instance(product);
    let to_host = |of: &StableName| -> Result<StableName, InlineError> {
        remap_name_keeping(of, &host_nodes, &host_steps, &host_reads, (&part, parted))
            .map_err(|missing| InlineError::stranded(&part, of, missing))
    };
    // A pick's path lands on the spliced variables.
    let host_path = |of: &[VarId]| -> Vec<VarId> {
        of.iter()
            .map(|var| read_map.get(var).copied().unwrap_or(*var))
            .collect()
    };
    // The instance's bundle, its own side read off the spliced product
    // it copies: each selection of the instance's copy authored afresh
    // over that body. Each other side is written again as it was, so
    // the new placement reads no unnamed variable the instance reads.
    let host_vars = VarCarry::new(doc);
    let mut selects: BTreeMap<VarId, crate::Operand> = BTreeMap::new();
    for side in bundle
        .iter()
        .filter_map(crate::mate::Constraint::mate)
        .flat_map(|mate| [mate.on, mate.to])
        .filter(|&side| doc.var_name(side).is_none())
    {
        let written = rewritten_pose(doc, side, &mut |_| Ok::<_, InlineError>(None))?;
        selects.insert(side, written);
    }
    for side in bundle
        .iter()
        .filter_map(crate::mate::Constraint::mate)
        .map(|mate| mate.on)
    {
        if doc.var_name(side).is_some() || pose_leaves(doc, side).is_none() {
            continue;
        }
        let mut leaf = |leaf: VarId| -> Result<Option<crate::Operand>, InlineError> {
            let Some(select) = doc.selection(leaf).filter(|s| s.body == copies) else {
                return Ok(None);
            };
            let names = select
                .names
                .iter()
                .map(&to_host)
                .collect::<Result<Vec<_>, _>>()?;
            Ok(Some(crate::Operand::select(
                crate::Operand::Var(spliced),
                crate::var::Select::canonical(selection_kind(doc, leaf), names),
            )))
        };
        selects.insert(side, rewritten_pose(doc, side, &mut leaf)?);
    }
    let whole = Node::Place {
        source: crate::BodyRead::plain(spliced),
        target: *target,
        constraints: bundle.clone(),
    };
    let (authored, fresh) = host_vars.author(&whole, &selects);
    let placement = current
        .apply_recorded(DocEdit::InsertNode {
            node: Box::new(authored),
            fresh: fresh.fresh,
        })
        .map_err(refused)?
        .minted
        .unwrap_or_else(|| unreachable!("an accepted insert mints its node"));
    let placed = current
        .doc()
        .output(placement, 0)
        .unwrap_or_else(|| unreachable!("a placement defines its copy"));
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
        let key = remap_name(name, &node_map, &step_map, &names_map)
            .map_err(|missing| InlineError::stranded(&part, name, missing))?;
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
    // Every host name the instance's copy carried lands on its spliced
    // name: the selections of the copy are authored afresh below; every
    // other carrier is repaired here, a selection by its body.
    // A name read in a body that reaches another instance only is that
    // instance's part's, spelled in ids the splice's maps may also hold:
    // it is not this instance's, and stays.
    let reaches_this = |body: VarId| {
        doc.read_operation(body).is_some_and(|at| {
            at == instance || crate::doc::strict_ancestors(doc, at).contains(&instance)
        })
    };
    let others_only = |body: VarId| doc.reaches_instance(body) && !reaches_this(body);
    let mut moved: BTreeMap<StableName, StableName> = BTreeMap::new();
    for carrier in doc.name_carriers() {
        let read_in = match carrier {
            NameCarrier::Select { var, .. } => doc.selection(var).map(|select| select.body),
            NameCarrier::Store { body, .. } => Some(body),
            NameCarrier::Payload { .. } => None,
        };
        if read_in.is_some_and(|body| reads_instance(body) || others_only(body)) {
            continue;
        }
        let from = carrier.name();
        let to = to_host(from)?;
        if &to != from {
            moved.insert(from.clone(), to);
        }
    }
    for (from, to) in &moved {
        let mut bodies: BTreeSet<VarId> = BTreeSet::new();
        let mut unbodied = false;
        for carrier in current.doc().name_carriers() {
            if carrier.name() != from {
                continue;
            }
            match carrier {
                NameCarrier::Select { var, .. } => {
                    if let Some(select) = current.doc().selection(var)
                        && !reads_instance(select.body)
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
                    to: to.clone(),
                },
            )?;
        }
    }
    // The record dissolves (ASM-R2b D-4, and the function docs): every
    // crossing's part-side reference must land on a spliced local
    // name, CHECKED.
    for crossing in &interface.crossings {
        let InterfaceCrossing::Mate { inner, .. } = crossing;
        remap_name(inner, &node_map, &step_map, &names_map)
            .map_err(|missing| InlineError::stranded(&part, inner, missing))?;
    }
    // A body read of the instance's copy, as the new placement's: the
    // copy whole, or a pick of it by the spliced path.
    let host_body = |body: VarId| -> crate::Operand {
        match doc.pick(body) {
            Some(pick) if picks.contains(&body) => {
                crate::Operand::pick(crate::Operand::Var(placed), host_path(&pick.of))
            }
            _ => crate::Operand::Var(placed),
        }
    };
    // A selection of the instance's copy authored afresh on the new
    // placement's copy, its names landed.
    let reselect = |var: VarId| -> Result<crate::Operand, InlineError> {
        let Some(select) = doc.selection(var) else {
            unreachable!("a selection reader reads a selection")
        };
        Ok(crate::Operand::select(
            host_body(select.body),
            crate::var::Select::canonical(
                selection_kind(doc, var),
                select
                    .names
                    .iter()
                    .map(&to_host)
                    .collect::<Result<Vec<_>, _>>()?,
            ),
        ))
    };
    {
        // Once per selection, as split authors them: a named one keeps
        // its name on the new variable, which every other reader reads.
        let mut reauthored: BTreeMap<VarId, VarId> = BTreeMap::new();
        for &(node, slot, var) in &readers {
            let value = match (doc.selection(var), reauthored.get(&var)) {
                (Some(_), Some(&fresh)) => crate::SlotValue::Read(crate::Operand::Var(fresh)),
                (Some(_), None) => crate::SlotValue::Read(reselect(var)?),
                (None, _) => crate::SlotValue::Read(host_body(var)),
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
                    Some(select) if reads_instance(select.body) => reselect(leaf).map(Some),
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
    // The product reads the new placement's copy where it read the
    // instance's.
    if doc.product().is_some_and(reads_instance) {
        step(
            &mut current,
            DocEdit::SetProduct {
                product: Some(crate::Operand::Var(placed)),
            },
        )?;
    }
    // A host appearance key read in the instance's copy is re-attached
    // in the new placement's: a Rebind keeps a key's read, and the
    // inline takes that read away.
    for (key, record) in current.doc().appearance().clone() {
        if key.at != copies {
            continue;
        }
        let name = to_host(&key.name)?;
        reattach_appearance(
            &mut |edit| step(&mut current, edit),
            &key,
            &record,
            &crate::AppearanceKey::new(placed, name),
        )?;
    }
    step(&mut current, DocEdit::DeleteNode { id: instance })?;
    // A name on the instance's copy moves onto the new placement's.
    if let Some(name) = doc.var_name(copies) {
        step(
            &mut current,
            DocEdit::RenameVar {
                var: placed.into(),
                name: Some(name.clone()),
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
        placement,
        space,
    })
}
// ---- Flatten ----

/// **What [`flatten`] answers**: the document with the placement and
/// its space gone, and the recorded edits producing it.
#[derive(Debug, Clone)]
pub struct FlattenOutcome {
    /// The document after the flatten.
    pub doc: ProfileDoc,
    /// The recorded edits producing `doc` from the input.
    pub edits: Vec<DocEdit<ProfileProgram>>,
    /// The maintenance `edits` reported, net, in edit order.
    pub maintenance: Vec<Maintenance>,
}

/// **Why a placement does not flatten** ([`flatten`]).
#[derive(Debug)]
pub enum FlattenError {
    /// The id is no live node.
    UnknownNode {
        /// The missing id.
        id: SpokenNode,
    },
    /// The node is no `Place`.
    NotAPlacement {
        /// The node.
        node: SpokenNode,
    },
    /// Its bundle is not the identity as written, so its copy is not
    /// its source where the source stands.
    NotTheIdentity {
        /// The placement.
        placement: SpokenNode,
    },
    /// Its source is not in one `new_space()`: a construction's own
    /// space, or the world, is not one a flatten removes.
    NotANewSpace {
        /// The placement.
        placement: SpokenNode,
    },
    /// Another placement copies a source out of the source's space into
    /// a third, which would then read the target's coordinates.
    SpaceShared {
        /// The placement.
        placement: SpokenNode,
        /// The other placement reading out of the space.
        other: SpokenNode,
    },
    /// A read of the copy names no one shape it copies.
    ReadUncarried {
        /// The reader.
        reader: SpokenNode,
    },
    /// The copy and the one shape it copies are both named, and a
    /// variable has one name (VR2).
    BothNamed {
        /// The copy's name.
        name: crate::doc::VarName,
        /// The shape's.
        held: crate::doc::VarName,
    },
    /// Replaying the constructed edits refused, typed.
    Edit {
        /// The refusing edit's own diagnosis.
        error: Box<EditError>,
    },
}

impl core::fmt::Display for FlattenError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::UnknownNode { id } => write!(
                f,
                "flatten: {id} is not live. {}",
                Recourse("name a live placement")
            ),
            Self::NotAPlacement { node } => write!(
                f,
                "flatten: {node} is no placement. {}",
                Recourse("name a placement whose bundle is the identity")
            ),
            Self::NotTheIdentity { placement } => write!(
                f,
                "flatten: {placement}'s bundle is not the identity as written, so its copy \
                 is not its source where the source stands. {}",
                Recourse(&format!("write {placement}'s bundle as the identity first"))
            ),
            Self::NotANewSpace { placement } => write!(
                f,
                "flatten: {placement}'s source is not in one new_space(), and only such a \
                 space is flattened into its target. {}",
                Recourse("keep the placement")
            ),
            Self::SpaceShared { placement, other } => write!(
                f,
                "flatten: {other} also copies a source out of {placement}'s source's space, \
                 and would read the target's coordinates once it is flattened. {}",
                Recourse(&format!("delete or re-point {other} first"))
            ),
            Self::ReadUncarried { reader } => write!(
                f,
                "flatten: {reader} reads the copy whole or by a name of no one shape it \
                 copies. {}",
                Recourse(&format!(
                    "re-point {reader} at one copy (SetParam), then flatten"
                ))
            ),
            Self::BothNamed { name, held } => write!(
                f,
                "flatten: the copy is named {name} and the shape it copies {held}. {}",
                Recourse(&format!("clear the name {name} (RenameVar), then flatten"))
            ),
            Self::Edit { error } => write!(f, "flatten: {error}"),
        }
    }
}

impl core::error::Error for FlattenError {}

/// **Flattens an identity placement into its target** (ASSEMBLY A4):
/// a `Place` whose bundle is the identity as written, and whose source
/// is in a `new_space()` no other placement copies a source out of. Every
/// placement into that space is re-targeted to the placement's target,
/// every read of the copy re-pointed to the shape it copies (a name of
/// the copy re-anchored on the shape's), and the placement and the
/// space deleted. Anything else refuses, typed. Inline-of-split, then
/// this, returns the document split was given, up to minted ids and
/// order. Pure — `doc` is untouched; it computes no frame.
///
/// # Errors
///
/// Every arm of [`FlattenError`].
#[allow(clippy::too_many_lines)] // one linear pass per read kind, each short
pub fn flatten(
    doc: &ProfileDoc,
    placement: RecipeNodeId,
    tol: Tol,
) -> Result<FlattenOutcome, FlattenError> {
    let Some(node) = doc.node(placement) else {
        return Err(FlattenError::UnknownNode {
            id: SpokenNode::absent(placement),
        });
    };
    let Node::Place {
        source,
        target,
        constraints,
    } = node
    else {
        return Err(FlattenError::NotAPlacement {
            node: doc.spoken(placement),
        });
    };
    if !crate::mate::solve::identity_bundle(doc, constraints) {
        return Err(FlattenError::NotTheIdentity {
            placement: doc.spoken(placement),
        });
    }
    let space = match doc.space_of(source.read) {
        Some(space)
            if doc
                .operation_of(space)
                .is_some_and(|at| matches!(doc.node(at), Some(Node::Space))) =>
        {
            space
        }
        _ => {
            return Err(FlattenError::NotANewSpace {
                placement: doc.spoken(placement),
            });
        }
    };
    for other in doc.placements() {
        if other == placement {
            continue;
        }
        // A placement within the space is re-targeted with it below; one
        // copying out of it into another space would share the source.
        if let Some(Node::Place { source, .. }) = doc.node(other)
            && doc.node_space(other) != Some(space)
            && doc.space_of(source.read) == Some(space)
        {
            return Err(FlattenError::SpaceShared {
                placement: doc.spoken(placement),
                other: doc.spoken(other),
            });
        }
    }
    let copy = doc
        .output(placement, 0)
        .unwrap_or_else(|| unreachable!("a placement defines its copy"));
    let source = source.read;
    // A name of the copy is its source's, verbatim, the copy's own body
    // name aside.
    let shape_of = |name: &StableName| -> Option<(VarId, StableName)> {
        (name.node != placement || name.path != vec![RoleSeg::OutputBody])
            .then(|| (source, name.clone()))
    };
    // A pick of the copy, read in the source's space: a pick of the
    // source by the same path where the source is a copy itself, else
    // the `⊔` operand the path starts at, picked further by the rest.
    let source_is_copy = matches!(
        doc.operation_of(source).and_then(|at| doc.node(at)),
        Some(Node::Place { .. } | Node::InstantiatePart { .. })
    );
    let unpicked = |of: &[VarId]| -> crate::Operand {
        match of.split_first() {
            _ if source_is_copy => crate::Operand::pick(crate::Operand::Var(source), of.to_vec()),
            Some((&first, [])) => crate::Operand::Var(first),
            Some((&first, rest)) => crate::Operand::pick(crate::Operand::Var(first), rest.to_vec()),
            None => crate::Operand::Var(source),
        }
    };
    let picks: BTreeSet<VarId> = doc
        .definition_order()
        .into_iter()
        .filter(|&var| doc.pick(var).is_some_and(|pick| pick.placement == copy))
        .collect();
    let in_source = |body: VarId| -> crate::Operand {
        match doc.pick(body) {
            Some(pick) if picks.contains(&body) => unpicked(&pick.of),
            _ => crate::Operand::Var(source),
        }
    };
    let reads_copy = |body: VarId| body == copy || picks.contains(&body);
    let mut current = Recording::start(doc, tol);
    let refused = |error| FlattenError::Edit {
        error: Box::new(error),
    };
    let step =
        |current: &mut Recording<'_, ProfileProgram>,
         edit: DocEdit<ProfileProgram>|
         -> Result<(), FlattenError> { current.apply(edit).map(|_| ()).map_err(refused) };
    // The placements into the space take its target.
    for other in doc.placements() {
        if other != placement && doc.node_space(other) == Some(space) {
            step(
                &mut current,
                DocEdit::SetParam {
                    node: other,
                    slot: crate::SlotId::Operand(crate::OperandSlot::Space),
                    value: crate::SlotValue::Read(crate::Operand::Var(*target)),
                    fresh: Vec::new(),
                },
            )?;
        }
    }
    // A selection of the copy authored afresh on the shape its names
    // are of.
    let reselect = |var: VarId, reader: RecipeNodeId| -> Result<crate::Operand, FlattenError> {
        let Some(select) = doc.selection(var) else {
            unreachable!("a selection reader reads a selection")
        };
        let names = select
            .names
            .iter()
            .map(|name| {
                shape_of(name)
                    .map(|(_, of)| of)
                    .ok_or_else(|| FlattenError::ReadUncarried {
                        reader: doc.spoken(reader),
                    })
            })
            .collect::<Result<Vec<_>, _>>()?;
        // Names a `⊔` source keys by one member's read are that member's
        // own, read in the member: the `⊔` only grouped the copy.
        let group = doc
            .operation_of(source)
            .filter(|&at| matches!(doc.node(at), Some(Node::Assemble { .. })));
        let member = |name: &StableName| match name.path.as_slice() {
            [RoleSeg::From { read, of }] if Some(name.node) == group => {
                Some((*read, (**of).clone()))
            }
            _ => None,
        };
        if select.body == copy
            && let Some(members) = names.iter().map(member).collect::<Option<Vec<_>>>()
            && let Some(&(read, _)) = members.first()
            && members.iter().all(|(r, _)| *r == read)
        {
            return Ok(crate::Operand::select(
                crate::Operand::Var(read),
                crate::var::Select::canonical(
                    selection_kind(doc, var),
                    members.into_iter().map(|(_, of)| of).collect(),
                ),
            ));
        }
        Ok(crate::Operand::select(
            in_source(select.body),
            crate::var::Select::canonical(selection_kind(doc, var), names),
        ))
    };
    let mut reauthored: BTreeMap<VarId, VarId> = BTreeMap::new();
    for id in doc.ids() {
        if id == placement {
            continue;
        }
        let Some(reader) = doc.node(id) else { continue };
        for (slot, var) in reader.operand_rows() {
            let value = if let Some(leaves) = pose_leaves(doc, var) {
                if !leaves
                    .iter()
                    .any(|&leaf| doc.selection(leaf).is_some_and(|s| reads_copy(s.body)))
                {
                    continue;
                }
                let mut leaf = |leaf: VarId| -> Result<Option<crate::Operand>, FlattenError> {
                    match doc.selection(leaf) {
                        Some(select) if reads_copy(select.body) => reselect(leaf, id).map(Some),
                        _ => Ok(None),
                    }
                };
                crate::SlotValue::Read(rewritten_pose(doc, var, &mut leaf)?)
            } else if doc.selection(var).is_some_and(|s| reads_copy(s.body)) {
                match reauthored.get(&var) {
                    Some(&fresh) => crate::SlotValue::Read(crate::Operand::Var(fresh)),
                    None => crate::SlotValue::Read(reselect(var, id)?),
                }
            } else if reads_copy(var) {
                crate::SlotValue::Read(in_source(var))
            } else {
                continue;
            };
            step(
                &mut current,
                DocEdit::SetParam {
                    node: id,
                    slot: crate::SlotId::Operand(slot),
                    value,
                    fresh: Vec::new(),
                },
            )?;
            if doc.selection(var).is_some_and(|s| reads_copy(s.body))
                && !reauthored.contains_key(&var)
            {
                let fresh = read_at(current.doc(), id, slot);
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
    }
    // An appearance key read in the copy re-attaches in its shape,
    // before the rebinding below would rename it where it is held.
    for (key, record) in current.doc().appearance().clone() {
        if key.at != copy {
            continue;
        }
        let Some((shape, name)) = shape_of(&key.name) else {
            return Err(FlattenError::ReadUncarried {
                reader: doc.spoken(placement),
            });
        };
        reattach_appearance(
            &mut |edit| step(&mut current, edit),
            &key,
            &record,
            &crate::AppearanceKey::new(shape, name),
        )?;
    }
    // Every other carrier of a name of the copy holds its source's name
    // already: a copy names its entities as its source does.
    if doc.product() == Some(copy) {
        step(
            &mut current,
            DocEdit::SetProduct {
                product: Some(crate::Operand::Var(source)),
            },
        )?;
    }
    let renamed = match doc.var_name(copy) {
        None => None,
        Some(name) => match doc.var_name(source) {
            Some(held) => {
                return Err(FlattenError::BothNamed {
                    name: name.clone(),
                    held: held.clone(),
                });
            }
            None => Some((name.clone(), source)),
        },
    };
    step(&mut current, DocEdit::DeleteNode { id: placement })?;
    let space_node = doc
        .operation_of(space)
        .unwrap_or_else(|| unreachable!("a new space is a node's output"));
    step(&mut current, DocEdit::DeleteNode { id: space_node })?;
    // A `⊔` the source is — the product an inline spliced — read by
    // nothing once its copy's readers read its operands, goes with the
    // placement: the copy it grouped is gone.
    let read = |doc: &ProfileDoc, var: VarId| {
        doc.product() == Some(var)
            || doc.ids().into_iter().any(|id| {
                doc.node(id).is_some_and(|node| {
                    node.operand_rows().into_iter().any(|(_, read)| {
                        let reads = |read: VarId| {
                            read == var
                                || doc.selection(read).is_some_and(|select| select.body == var)
                                || doc.pick(read).is_some_and(|pick| pick.placement == var)
                        };
                        // A pose reads its leaves.
                        reads(read)
                            || pose_leaves(doc, read)
                                .is_some_and(|leaves| leaves.into_iter().any(reads))
                    })
                })
            })
    };
    if let Some(group) = doc
        .operation_of(source)
        .filter(|&at| matches!(doc.node(at), Some(Node::Assemble { .. })))
        && !read(current.doc(), source)
    {
        step(&mut current, DocEdit::DeleteNode { id: group })?;
    }
    if let Some((name, var)) = renamed {
        step(
            &mut current,
            DocEdit::RenameVar {
                var: var.into(),
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
    Ok(FlattenOutcome {
        doc,
        edits,
        maintenance,
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
/// order — the form the emitters would mint for the same entity under
/// the new ids — and no value moves: a crossing of a union seam whose
/// sides swap still names the side it enters along.
#[cfg(test)]
#[allow(clippy::expect_used)]
mod a_remap_that_reorders_ids_republishes_the_canonical_form {
    use super::{NodeMap, ReadMap, StepMap, remap_name};
    use crate::names::{NameRef, Qualifier, RoleSeg, Sense, StableName};
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
    fn a_crossing_of_a_union_seam_keeps_the_side_it_enters_along() {
        let vertex = |u: u64, edge: StableName, m3: u64, entering: u64| {
            name(
                EntityKind::Vertex,
                u,
                vec![RoleSeg::Crossing {
                    edge: NameRef::new(edge),
                    face: NameRef::new(member(EntityKind::Face, u, m3)),
                    sense: Sense::Through {
                        entering: vec![NameRef::new(member(EntityKind::Face, u, entering))],
                    },
                }],
            )
        };
        let was = vertex(9, union_edge(9, (1, 2)), 3, 1);
        let out = remap_name(&was, &map(), &StepMap::new(), &mirrored_reads()).expect("covered");
        assert_eq!(
            out,
            vertex(20, union_edge(20, (31, 30)), 32, 31),
            "the seam's pair re-sorted, the sense still naming member 1's side"
        );
    }

    #[test]
    fn a_pair_booleans_sets_borders_junction_and_seams_are_resorted() {
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
            "the run re-sorted, each line in name order"
        );
        // A pair boolean's seam vertex is in name order as a union's is.
        let vertex = name(EntityKind::Vertex, 5, vec![seam(face(2), face(1))]);
        let out = remap_name(&vertex, &map(), &StepMap::new(), &mirrored_reads()).expect("covered");
        let (x, y) = (face(31), face(30));
        let (x, y) = if x < y { (x, y) } else { (y, x) };
        assert_eq!(out.path, vec![seam(x, y)], "the pair in name order");
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

/// **An edge-crossing run whose lines a caller's maps reorder is put
/// back in name order, each sense with its line.** A caller's own maps
/// need not keep id order, so two lines can swap places; nothing in the
/// name is read along that order, so the run is re-sorted and carried.
#[cfg(test)]
#[allow(clippy::expect_used, clippy::panic)]
mod remap_a_run {
    use super::{NodeMap, ReadMap, StepMap, remap_name};
    use crate::names::{NameRef, RoleSeg, RunLine, Sense, StableName};
    use crate::node::RecipeNodeId;
    use crate::{CapEnd, EntityKind};

    fn edge(node: u64) -> NameRef {
        NameRef::new(StableName {
            kind: EntityKind::Edge,
            node: RecipeNodeId::new(0, node),
            path: vec![RoleSeg::Cap(CapEnd::Start)],
        })
    }

    /// A run minted by `minter` over the lines `lines`, the first
    /// entering and the second leaving.
    fn run(minter: u64, lines: [(u64, Sense); 2]) -> StableName {
        StableName {
            kind: EntityKind::Vertex,
            node: RecipeNodeId::new(0, minter),
            path: vec![RoleSeg::EdgeCrossing(
                lines
                    .into_iter()
                    .map(|(n, sense)| RunLine {
                        line: edge(n),
                        sense,
                    })
                    .collect(),
            )],
        }
    }

    fn map(pairs: [(u64, u64); 3]) -> NodeMap {
        pairs
            .into_iter()
            .map(|(a, b)| (RecipeNodeId::new(0, a), RecipeNodeId::new(0, b)))
            .collect()
    }

    #[test]
    fn a_remap_that_swaps_a_runs_lines_re_sorts_it_each_sense_with_its_line() {
        let was = run(5, [(1, Sense::Enters), (2, Sense::Leaves)]);
        assert_eq!(
            remap_name(
                &was,
                &map([(1, 20), (2, 10), (5, 15)]),
                &StepMap::new(),
                &ReadMap::new(),
            ),
            Ok(run(15, [(10, Sense::Leaves), (20, Sense::Enters)])),
            "line 1 is now 20, after 10, and keeps its sense"
        );
        assert_eq!(
            remap_name(
                &was,
                &map([(1, 10), (2, 20), (5, 15)]),
                &StepMap::new(),
                &ReadMap::new()
            ),
            Ok(run(15, [(10, Sense::Enters), (20, Sense::Leaves)])),
            "a map that keeps the lines' order carries the run as it was"
        );
    }
}

/// **Every [`RemapError`] arm reads as a refusal**: within the budget,
/// one recourse, and ending on it, on a representative payload.
#[cfg(test)]
#[allow(clippy::expect_used, clippy::panic)]
mod remap_error_reads {
    use super::{RemapError, Unmapped};
    use crate::node::{RecipeNodeId, StepId};
    use crate::var::VarId;
    use test_utils::refusal::{problems, recourse_markers, tagged};

    #[test]
    fn every_remap_refusal_fits_the_budget_and_ends_on_one_recourse() {
        let arms = [
            RemapError::Unmapped(Unmapped::Node(RecipeNodeId::new(0, tagged(4)))),
            RemapError::Unmapped(Unmapped::Step(StepId::new(0, tagged(4)))),
            RemapError::Unmapped(Unmapped::Read(VarId::new(0, tagged(4)))),
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
