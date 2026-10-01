//! **Split and inline** — the first-class recorded refactorings
//! (ASM-4 D-2/D-3; ASSEMBLY-DESIGN A4, A10, A11).
//!
//! [`split`] cuts a closed set of nodes out of a document into a new
//! part document and leaves an [`Node::InstantiatePart`] of it behind;
//! [`inline`] is the inverse — it splices a referenced document's
//! recipe into the host and deletes the instance. Both are PURE
//! functions returning new document values, the ordinary recorded
//! [`DocEdit`]s that produce them, and the [`crate::Maintenance`]
//! those edits reported — the payload names a departing cut node
//! stranded, and the offset a spliced mate's door cleared. The input
//! documents are untouched,
//! so undo is this layer's undo everywhere else: keeping the prior
//! value. There is no compound edit arm; atomicity is purity (no
//! partially-refactored document is ever observable).
//!
//! # The cut rule (D-2)
//!
//! The cut set must be closed under the recipe DAG in BOTH directions:
//! an edge with exactly one endpoint in the cut is a severed consuming
//! edge, refused typed naming the edge. (Closure under inputs is A4's
//! "ancestor-closed"; closure under consumers is what makes every cut
//! sink a document sink, i.e. an A10 root.)
//!
//! # Gauges and offsets cross the seam (A4)
//!
//! The cut must be a union of WHOLE placement groups — a torn group
//! refuses [`SplitError::TornGroup`] — so a placing mate never crosses
//! it. Every gauge reference leaving the cut lands on ONE anchor, a
//! kept gauge or the world ([`SplitError::TwoAnchors`]), and the
//! instance left behind names it. The cut's one placed thing gives
//! that instance its offset: a cut that is exactly one placed group —
//! its instances and the mates holding them, nothing else — HOISTS its
//! root's offset onto the instance and lands the root at the empty
//! chain in the part, the shape a reusable part wants; any other cut
//! moves verbatim and the instance sits at the empty offset. Neither
//! computes a frame: an offset is moved as the chain it is.
//!
//! [`inline`] is the inverse. At the empty offset the part's content
//! lands on the instance's gauge verbatim; a part that is one group
//! rooted at the empty chain on its world has its root take the
//! instance's offset. A mate side crosses the seam only where its
//! coordinates do not change — the instance it reads is its group's
//! root, on the part's world, at the empty chain — and the placing
//! mates of one pair must still read one pair (A4's frame and fold
//! rules). The shapes A4 builds with a minted gauge — a cut holding a
//! gauge, an inline at an offset over any other part — refuse typed
//! until that is built ([`SplitError::CutHoldsGauge`],
//! [`InlineError::NeedsAGauge`]).
//!
//! The remainder receives ONE `InstantiatePart` for the whole cut
//! (the D-2 amendment, adjudicated at review ordinal 40): each
//! remainder instance materializes the ENTIRE new document's product,
//! so per-group instances of one pinned document would duplicate
//! every other group's material N times. The single instance carries
//! every cut group where it sat. Consequence (amendment
//! rider i): the cut roots COLLAPSE onto the instance's root-list
//! position, so `inline(split(d))` restores the root SET and the
//! spliced block's relative order but NOT the original interleaving of
//! non-adjacent cut roots with kept roots — inline never sees the
//! interleaving, which lives only in the pre-split list. That is
//! within D-4's ratified identity (census, bit-equal volumes, name
//! re-resolution; root order is unnamed there), and it is pinned by
//! test rather than left implicit.
//!
//! # Names re-anchor across the seam (the bridge, both directions)
//!
//! Split rewrites every remainder-side reference to a cut entity —
//! Declare pairs, fillet selections, appearance keys — from its local
//! name to the `InPart`-wrapped name at the new instance (a recorded
//! [`DocEdit::Rebind`] per name), which is exactly how "every stable
//! name that resolved before resolves after, through the instance
//! qualifier" (D-4) is kept. Inline applies the inverse rewrite:
//! `InPart`-wrapped names at the inlined instance re-anchor to the
//! spliced local names. Appearance records ride these rewrites — they
//! stay with the document that referenced the entity, so the split
//! remainder keeps its presentation and the round trip restores it.
//! A reference that DERIVES from both sides of the cut can re-anchor
//! to neither and refuses typed, as does a top-level BODY name
//! crossing the cut (a product's name table deliberately carries no
//! root body rows, so the wrapped body name could never resolve).
//!
//! # Interface records
//!
//! A mate EDGE whose two ends land on opposite sides of the cut is one
//! [`crate::InterfaceCrossing::Mate`] entry in the remainder
//! instance's [`crate::InterfaceRecord`] (ASM-R2b D-4; AQ8). A PLACING
//! mate never crosses — it welds its members into one group and
//! `TornGroup` refuses to tear one — so every crossing is a declaring
//! mate, its ends on different gauges. A mate that is not an edge — a
//! dangling or nested-pattern head — contributes nothing however its
//! names fall: the gate is what skips it.
//!
//! # Determinism (D6/D9)
//!
//! Both refactorings are pure functions of their inputs (the part
//! document's identity is CALLER-supplied, never ambient), every
//! iteration below runs over ordered structures, and part-node ids
//! remap in document order — so two split runs, in any two processes,
//! produce byte-identical documents.

use std::collections::{BTreeMap, BTreeSet};

use crate::doc::{Doc, NameCarrier};
use crate::edit::Maintenance;
use crate::edit::{DocEdit, EditError, apply};
use crate::ident::{DocRef, DocumentId};
use crate::names::{
    Carry, EntityKind, FaceName, NameRef, ProfileEdgeRef, ProfileVertexRef, RoleSeg, SegRewrite,
    StableName,
};
use crate::node::{InterfaceCrossing, InterfaceRecord, Node, PatternKind, RecipeNodeId, StepId};
use crate::part::{PartResolver, ResolveFailure};
use crate::persist::{PersistError, content_pin};
use crate::program::{ProfileDoc, ProfilePayload as _, ProfileProgram};
use crate::resolve::derivation_nodes;
use crate::sentence::Recourse;
use crate::step_mint::StepMint;
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
}

impl SplitError {
    /// A cut node's reference the part-side rewrite could not map.
    fn reaches(node: RecipeNodeId, name: Box<StableName>, missing: Unmapped) -> Self {
        match missing {
            Unmapped::Node(missing) => Self::PartNameReachesRemainder {
                node,
                name,
                missing,
            },
            Unmapped::Step(step) => Self::NameOnDroppedStep { name, step },
        }
    }

    /// A remainder-side name the part-side rewrite could not map.
    fn straddles(name: Box<StableName>, missing: Unmapped) -> Self {
        match missing {
            Unmapped::Node(missing) => Self::NameStraddlesCut {
                name,
                missing: Some(missing),
            },
            Unmapped::Step(step) => Self::NameOnDroppedStep { name, step },
        }
    }
}

impl InlineError {
    /// A name to be spliced the host-side rewrite could not map.
    fn stranded(name: Box<StableName>, missing: Unmapped) -> Self {
        match missing {
            Unmapped::Node(missing) => Self::StrandedPartName { name, missing },
            Unmapped::Step(step) => Self::NameOnDroppedStep { name, step },
        }
    }
}

/// **A step the insert door minted other than as precomputed**: the
/// source document's step, the id the refactoring predicted for it,
/// and the id the carried profile holds after its insert (`None` where
/// it holds none there).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StepMapDivergence {
    /// The step, in the source document.
    pub step: StepId,
    /// The id the step map holds for it.
    pub precomputed: Option<StepId>,
    /// The id the insert minted for it.
    pub minted: Option<StepId>,
}

impl core::fmt::Display for StepMapDivergence {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let id = |s: Option<StepId>| s.map_or_else(|| "none".to_owned(), |s| format!("#{}", s.0));
        write!(
            f,
            "the profile step minted #{} was predicted to be re-minted as {} and was minted as \
             {}, which is a kernel bug",
            self.step.0,
            id(self.precomputed),
            id(self.minted)
        )
    }
}

/// **The step map checked against what the inserts minted**: every
/// profile of `source` that `node_map` carries into `target`, step by
/// step in loop then step order, holds the id `step_map` predicted.
/// The first step that does not is the divergence.
fn step_map_check(
    source: &ProfileDoc,
    node_map: &NodeMap,
    step_map: &StepMap,
    target: &ProfileDoc,
) -> Result<(), StepMapDivergence> {
    for (&old, &new) in node_map {
        let Some(Node::Profile(carried)) = source.node(old) else {
            continue;
        };
        let minted: Vec<StepId> = match target.node(new) {
            Some(Node::Profile(p)) => p.ids.iter().flatten().copied().collect(),
            _ => Vec::new(),
        };
        for (k, &step) in carried.ids.iter().flatten().enumerate() {
            let precomputed = step_map.get(&step).copied();
            let got = minted.get(k).copied();
            if precomputed.is_none() || precomputed != got {
                return Err(StepMapDivergence {
                    step,
                    precomputed,
                    minted: got,
                });
            }
        }
    }
    Ok(())
}

/// The step map a refactoring's inserts will mint, precomputed: every
/// profile among `nodes`, in the order they are inserted and under the
/// id `node_map` gives it, has its steps minted from `mint` by the insert
/// door's own minting ([`crate::ProfilePayload::mint_step_ids`]) — the
/// same canonical bytes, so the same ids — which [`step_map_check`]
/// confirms once the inserts are done. A profile whose plane or node
/// `node_map` does not carry ends the map there: its insert refuses
/// under its own name.
///
/// # Errors
///
/// The mint's refusal, as the insert would report it.
fn step_map_of<'a>(
    nodes: impl Iterator<Item = (RecipeNodeId, &'a Node<ProfileProgram>)>,
    node_map: &NodeMap,
    mint: &StepMint,
) -> Result<StepMap, EditError> {
    let mut mint = mint.clone();
    let mut map = StepMap::new();
    for (old, node) in nodes {
        let Node::Profile(p) = node else { continue };
        let (Some(&id), Some(&plane)) = (node_map.get(&old), node_map.get(&p.plane)) else {
            break;
        };
        let mut carried = ProfileProgram {
            plane,
            loops: p.loops.clone(),
            ids: Vec::new(),
        };
        carried
            .mint_step_ids(id, &mut mint)
            .map_err(|fault| EditError::StepIdsRefused { node: id, fault })?;
        map.extend(
            p.ids
                .iter()
                .flatten()
                .copied()
                .zip(carried.ids.iter().flatten().copied()),
        );
    }
    Ok(map)
}

/// Why [`split`] refused. Typed and specific (spec D-2): every arm
/// names the offending edge, parameter, or name.
#[derive(Debug)]
pub enum SplitError {
    /// The cut set is empty — there is nothing to split out.
    EmptyCut,
    /// A cut entry does not name a live node.
    UnknownCutNode {
        /// The entry with no live node.
        id: RecipeNodeId,
    },
    /// The new document's identity collides with the document being
    /// split or with a document the cut nodes reference — the new id
    /// must be fresh, or the produced pair could not both exist in one
    /// store (and a self-reference would be an evaluation cycle).
    PartIdCollides {
        /// The colliding identity.
        id: DocumentId,
    },
    /// A recipe edge crosses the cut: its consumer is on one side and
    /// its input on the other, so the cut would sever it (D-2 — the
    /// cut must be ancestor- and consumer-closed).
    SeveredEdge {
        /// The consuming node.
        consumer: RecipeNodeId,
        /// The input it consumes.
        input: RecipeNodeId,
        /// Whether the CONSUMER is the cut-side endpoint.
        consumer_is_cut: bool,
    },
    /// A mate and the OPERAND one of its references is read at land
    /// on opposite sides of the cut — the reading-edge twin of
    /// [`SplitError::SeveredEdge`], and refused for the same reason.
    ///
    /// An operand is not a consuming edge, so D-2's closure rule does
    /// not reach it; but a mate whose operand is on the far side of a
    /// cut can be carried by neither document. Kept, its operand names
    /// a node the remainder no longer has, and nothing downstream
    /// notices until the solve refuses a dangling reference. Cut, the
    /// part document has no node to remap the operand onto. Both are
    /// refused HERE, at the door, naming the mate, the side and the
    /// operand — the repair is to widen the cut, or to re-author the
    /// mate at a node on the side it is staying.
    ///
    /// A mate that WELDS a group meets `TornGroup` first, because
    /// the cut also splits the group its two members share. This
    /// arm is what catches the rest: a mate whose reference resolves
    /// to no member welds nothing, and its operand still crosses.
    OperandSeveredFromMate {
        /// The mate whose reference is severed.
        mate: RecipeNodeId,
        /// Which of its two references.
        side: crate::mate::MateSide,
        /// The operand node on the far side of the cut.
        operand: RecipeNodeId,
        /// Whether the MATE is the cut-side endpoint.
        mate_is_cut: bool,
    },
    /// The cut TEARS a placement group: some of the group's
    /// instances are cut and some are kept (ASM-R2a; review MAJOR-2).
    ///
    /// A11 puts the frame on the GROUP, so a torn group has one
    /// frame and two homes; splitting it would have to invent which
    /// side keeps it and re-mint the other from a relative pose that
    /// now crosses a document seam — machinery no ratified rule
    /// supplies. The cut must be a union of WHOLE groups, which is
    /// what this module's docs have promised since ASM-4 and what
    /// mates made checkable. Refused naming the group and the
    /// instance on the far side of the tear; the repair is to widen
    /// the cut to the whole group, or to delete the mates that hold
    /// it together first.
    TornGroup {
        /// The group's root.
        root: RecipeNodeId,
        /// The first member, in document order, on the opposite side
        /// of the cut from the root.
        instance: RecipeNodeId,
        /// Whether the ROOT is the cut-side endpoint.
        root_is_cut: bool,
    },
    /// **The cut holds a gauge.** Splitting one out — the gauge hoist,
    /// and the anchor a kept instance on it would need — is not built
    /// yet; a cut that leaves the gauge behind anchors on it.
    CutHoldsGauge {
        /// The gauge in the cut.
        gauge: RecipeNodeId,
    },
    /// **The gauge references leaving the cut land on two anchors**
    /// (A4): the instance the split leaves behind names ONE gauge, so
    /// every cut instance must sit on that one — a kept gauge, or the
    /// world.
    TwoAnchors {
        /// The cut instance whose gauge disagrees with the first.
        instance: RecipeNodeId,
        /// The anchor the earlier cut instances name, `None` the world.
        first: Option<RecipeNodeId>,
        /// The one this instance names, `None` the world.
        second: Option<RecipeNodeId>,
    },
    /// **A cut group's gauge chain names a deleted gauge** (A4): a dead
    /// reference unplaces the group, and the split has no anchor to
    /// give the instance it leaves behind.
    DeadGaugeReference {
        /// The cut instance whose chain is dead.
        instance: RecipeNodeId,
        /// The deleted gauge it names.
        gauge: RecipeNodeId,
    },
    /// **The cut is unplaced material alone** (A4): every geometric
    /// node in it lives in an unplaced group's own space, so the
    /// instance the split would leave behind has nothing to be placed
    /// as.
    UnplacedAlone {
        /// The unplaced group the cut holds first, by its root.
        group: RecipeNodeId,
    },
    /// **A mate would start placing** (A4): it reads a kept instance on
    /// one side and the cut on the other — a cut instance on another
    /// gauge, which it declares against, or cut material that is no
    /// instance — and the instance the split leaves behind sits on the
    /// anchor, the kept instance's gauge, so the mate would place what
    /// it did not.
    WouldStartPlacing {
        /// The mate.
        mate: RecipeNodeId,
    },
    /// **A mate side would change coordinates across the seam** (A4's
    /// frame rule): a kept mate reads a cut instance that, in the part,
    /// is not its group's root on the part's world at the empty chain,
    /// so the frame it is authored in would mean another place once it
    /// reads the instance the split leaves behind.
    MateFrameCrosses {
        /// The mate.
        mate: RecipeNodeId,
        /// Which of its sides crosses.
        side: crate::mate::MateSide,
    },
    /// **A `FromFace` side would cross the seam**: a kept mate's side
    /// that reads a cut instance names its frame as a face of that
    /// instance's part, in the part's own spelling, and once the side
    /// reads the instance the split leaves behind the name is not a
    /// row of the new part's table. The face would be in the new
    /// part, under the name the inner instance wraps it in; the
    /// re-spelling is not built, so the split refuses rather than
    /// leave a frame naming nothing.
    MateFaceFrameCrosses {
        /// The mate.
        mate: RecipeNodeId,
        /// Which of its sides crosses.
        side: crate::mate::MateSide,
    },
    /// **A hoisted group's member carries a further offset**: the hoist
    /// lands the root at the empty chain, so a member's offset, stated
    /// against the root's old place, would no longer hold.
    HoistedMemberOffset {
        /// The member.
        instance: RecipeNodeId,
    },
    /// A cut node references a document parameter that a kept node
    /// also references. The parameter can move or stay, but it cannot
    /// silently become two parameters with one name (D-2's "no silent
    /// sharing") — refused naming one referencing node on each side.
    UncutParamReference {
        /// The shared parameter.
        param: crate::doc::ParamName,
        /// A cut node referencing it.
        cut_node: RecipeNodeId,
        /// A kept node referencing it.
        kept_node: RecipeNodeId,
    },
    /// A name inside a CUT node's payload derives from a node that is
    /// not itself cut — the part document could not express the
    /// reference (a part has no name for its consumer's entities, and
    /// a stranded reference has no node to remap at all).
    PartNameReachesRemainder {
        /// The cut node carrying the reference.
        node: RecipeNodeId,
        /// The name that reaches outside the cut.
        name: Box<StableName>,
        /// A node the name derives from that the part document has
        /// no copy of — the id the part-side rewrite could not map,
        /// or, from the precondition below, the lowest-numbered
        /// derivation node outside the cut. For a nested name it is a
        /// node inside one of `name`'s path segments, not `name`'s
        /// own minting node, so `name` alone does not say which node
        /// reaches out.
        missing: RecipeNodeId,
    },
    /// A remainder-side name derives from BOTH sides of the cut, so it
    /// can re-anchor to neither document alone.
    NameStraddlesCut {
        /// The straddling name.
        name: Box<StableName>,
        /// The node the part-side rewrite could not map, when the
        /// refusal came from a rewrite. For a nested name that is a
        /// node inside one of `name`'s path segments, not `name`'s
        /// own minting node, so the name alone does not say which
        /// node failed. `None` when the refusal came instead from the
        /// straddle classification, which weighs the whole derivation
        /// set at once and singles out no one node.
        missing: Option<RecipeNodeId>,
    },
    /// A name the split carries spells a piece of a profile step that
    /// no profile of this document holds — a step a `SetProgram`
    /// dropped, whose names were reported stranded (DM7). Its id is
    /// never minted again here, and the part document would mint it
    /// afresh for a step of its own, so the name cannot cross; repair
    /// the stranded reference first.
    NameOnDroppedStep {
        /// The name.
        name: Box<StableName>,
        /// The dropped step it spells.
        step: StepId,
    },
    /// A remainder-side BODY name crosses the cut. A document's
    /// product name table deliberately carries no root body rows (the
    /// product is the document's own body, not any root's), so the
    /// `InPart`-wrapped rewrite of a body name could never resolve —
    /// refused rather than silently breaking a resolving name.
    BodyNameCrossesCut {
        /// The crossing body name.
        name: Box<StableName>,
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
    /// vocabulary cannot re-author (e.g. a Declare rebound to a node
    /// inserted after it, which no insertion order can satisfy).
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
    /// The part document's insert door minted a cut profile's step
    /// under an id other than the one the split precomputed for it —
    /// a construction bug in this module, surfaced typed: every name
    /// the split rewrote through its step map would spell the wrong
    /// step.
    StepMapDiverged(StepMapDivergence),
}

impl core::fmt::Display for SplitError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::EmptyCut => f.write_str("split: the cut set is empty"),
            Self::UnknownCutNode { id } => {
                write!(f, "split: cut entry {} is not a live node", id.0)
            }
            Self::TornGroup {
                root,
                instance,
                root_is_cut,
            } => {
                let (cut, kept) = if *root_is_cut {
                    (root.0, instance.0)
                } else {
                    (instance.0, root.0)
                };
                write!(
                    f,
                    "split: the cut tears the placement group rooted at node {} (node {cut} is \
                     cut, node {kept} is kept) — the frame lives on the GROUP, so the cut \
                     must be a union of WHOLE groups; widen the cut, or delete the mates \
                     holding the group together first",
                    root.0
                )
            }
            Self::CutHoldsGauge { gauge } => write!(
                f,
                "split: the cut holds gauge {g}, and splitting a gauge out is not built yet. {}",
                Recourse(&format!("leave gauge {g} out of the cut", g = gauge.0)),
                g = gauge.0
            ),
            Self::TwoAnchors {
                instance,
                first,
                second,
            } => write!(
                f,
                "split: the cut's instances sit on two gauges ({} and {}, at instance {}), and \
                 the instance the split leaves behind sits on one. {}",
                anchor_name(*first),
                anchor_name(*second),
                instance.0,
                Recourse(&format!(
                    "set instance {}'s gauge to {} (SetGauge), or leave it out of the cut",
                    instance.0,
                    anchor_name(*first)
                ))
            ),
            Self::DeadGaugeReference { instance, gauge } => write!(
                f,
                "split: instance {}'s gauge chain names node {}, which was deleted, so its group \
                 has no anchor. {}",
                instance.0,
                gauge.0,
                Recourse(&format!(
                    "set instance {}'s gauge to a live one, or the world (SetGauge)",
                    instance.0
                ))
            ),
            Self::UnplacedAlone { group } => write!(
                f,
                "split: everything the cut holds lives in the own space of the group rooted at \
                 node {}, which nothing places. {}",
                group.0,
                Recourse(crate::mate::UNPLACED_RECOURSE)
            ),
            Self::WouldStartPlacing { mate } => write!(
                f,
                "split: mate {m} reads the cut from an instance on the gauge the split's \
                 instance would sit on, so it would start placing. {}",
                Recourse(&format!("delete mate {m}, then split", m = mate.0)),
                m = mate.0
            ),
            Self::MateFrameCrosses { mate, side } => write!(
                f,
                "split: mate {m}'s {} side reads a cut instance that is not, in the new part, \
                 its group's root at the empty offset on the part's world, so its frame would \
                 mean another place. {}",
                side.name(),
                Recourse(&format!("delete mate {m}, then split", m = mate.0)),
                m = mate.0
            ),
            Self::MateFaceFrameCrosses { mate, side } => write!(
                f,
                "split: mate {m}'s {} side reads a cut instance and its frame names a face of \
                 that instance's part, a name the new part's table does not carry, so the frame \
                 would name nothing. {}",
                side.name(),
                Recourse(&format!(
                    "author mate {m}'s {} frame as numbers, or delete mate {m}, then split",
                    side.name(),
                    m = mate.0
                )),
                m = mate.0
            ),
            Self::HoistedMemberOffset { instance } => write!(
                f,
                "split: instance {i} carries an offset beside its group's root, which the \
                 split lands at the empty offset, so the statement would not hold. {}",
                Recourse(&format!(
                    "clear instance {i}'s offset (SetOffset), then split",
                    i = instance.0
                )),
                i = instance.0
            ),
            Self::PartIdCollides { id } => write!(
                f,
                "split: the new document id {id} collides with the split document or a \
                 document the cut references — supply a fresh identity"
            ),
            Self::OperandSeveredFromMate {
                mate,
                side,
                operand,
                mate_is_cut,
            } => {
                let (cut, kept) = if *mate_is_cut {
                    (mate.0, operand.0)
                } else {
                    (operand.0, mate.0)
                };
                write!(
                    f,
                    "split: the cut severs mate {}'s {} reference from the node it is read at \
                     (node {} — node {cut} is cut, node {kept} is kept); widen the cut, or \
                     re-author the mate at a node on its own side",
                    mate.0,
                    side.name(),
                    operand.0
                )
            }
            Self::SeveredEdge {
                consumer,
                input,
                consumer_is_cut,
            } => {
                let (cut, kept) = if *consumer_is_cut {
                    (consumer.0, input.0)
                } else {
                    (input.0, consumer.0)
                };
                write!(
                    f,
                    "split: the cut severs the edge from node {} to node {} (node {cut} is cut, \
                     node {kept} is kept) — a cut must be closed under inputs and consumers",
                    consumer.0, input.0
                )
            }
            Self::UncutParamReference {
                param,
                cut_node,
                kept_node,
            } => write!(
                f,
                "split: parameter {param} is referenced by cut node {} and kept node {} — one \
                 parameter cannot silently become two documents' parameters",
                cut_node.0, kept_node.0
            ),
            Self::PartNameReachesRemainder {
                node,
                name,
                missing,
            } => write!(
                f,
                "split: cut node {}'s reference (the {name}) derives from node {}, which is \
                 outside the cut — the new document could not express it",
                node.0, missing.0
            ),
            Self::NameStraddlesCut { name, missing } => {
                write!(
                    f,
                    "split: the {name} derives from both sides of the cut and can re-anchor to \
                     neither document"
                )?;
                match missing {
                    // The rewrite stopped at ONE node, which for a
                    // nested name is not the name's own mint.
                    Some(id) => write!(f, " — the rewrite stopped at node {}", id.0),
                    None => Ok(()),
                }
            }
            Self::NameOnDroppedStep { name, step } => write!(
                f,
                "split: the {name} spells a piece of the profile step minted #{}, which no profile of this \
                 document draws any more — repair the stranded reference before splitting",
                step.0
            ),
            Self::BodyNameCrossesCut { name } => write!(
                f,
                "split: the {name} crosses the cut — a product's name table carries no \
                 root body rows, so the instance-qualified rewrite could never resolve"
            ),
            Self::Pin { error } => {
                write!(
                    f,
                    "split: the new document's pin would not compute: {error}"
                )
            }
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
            Self::StepMapDiverged(d) => write!(f, "split: {d}"),
        }
    }
}

impl core::error::Error for SplitError {}

/// A gauge reference as a sentence names it.
fn anchor_name(gauge: Option<RecipeNodeId>) -> String {
    match gauge {
        None => "the world".to_owned(),
        Some(g) => format!("gauge {}", g.0),
    }
}

/// Why [`inline`] refused (spec D-3). Typed and specific.
#[derive(Debug)]
pub enum InlineError {
    /// The target id is not a live node.
    UnknownNode {
        /// The missing id.
        id: RecipeNodeId,
    },
    /// The target node does not instantiate a part.
    NotAnInstance {
        /// The non-instance target.
        node: RecipeNodeId,
    },
    /// The instance is consumed by another node. Splicing would have
    /// to rewire that consumer onto the part's product, which the
    /// recipe cannot express for a placed or multi-root product —
    /// refused typed in v1, naming the consumer.
    InstanceConsumed {
        /// The instance.
        node: RecipeNodeId,
        /// A node consuming it.
        by: RecipeNodeId,
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
    /// The referenced document declares a parameter the host also
    /// declares, with a different value — inlining would silently pick
    /// one meaning for the shared name.
    ParamConflict {
        /// The conflicting parameter.
        param: crate::doc::ParamName,
    },
    /// The instance sits off the world's origin — on a gauge, or at
    /// an offset — and the referenced document has a root that is not
    /// itself an instance: plain recipe geometry sits on no gauge, so
    /// the instance's frame is not something the spliced recipe can
    /// express locally (D-3's "refuses typed when the referenced
    /// product is not what the recipe can express").
    UnplaceableFrame {
        /// The plain-geometry root.
        root: RecipeNodeId,
    },
    /// **The instance carries no offset of its own**: its mates place
    /// it (or it is a checked member of a group another instance
    /// roots), so where it sits is a solve result the splice cannot
    /// state.
    MatePlaced {
        /// The instance.
        instance: RecipeNodeId,
    },
    /// **The instance is unplaced** (A11 (2)): nothing places its
    /// group, so there is no frame to splice its part in.
    Unplaced {
        /// The instance.
        instance: RecipeNodeId,
        /// Why nothing places it.
        cause: crate::mate::Unplaced,
    },
    /// **Inlining would mint a gauge** for the part's content at the
    /// instance's offset (A4), which is not built yet. Built are the
    /// empty offset, where the content lands on the instance's gauge
    /// verbatim, and a part that is one group rooted at the empty
    /// offset on its world with no further offset, whose root takes
    /// the instance's offset.
    NeedsAGauge {
        /// The instance.
        instance: RecipeNodeId,
    },
    /// **The referenced document holds a gauge reference to a deleted
    /// gauge**, which has no node in the host to land on.
    PartDeadGauge {
        /// The referenced document's node holding it.
        node: RecipeNodeId,
    },
    /// **A mate side would change coordinates across the seam** (A4's
    /// frame rule): a host mate reads the instance through a face of
    /// an inner instance that is not its part group's root at the
    /// empty offset on the part's world — or of no instance at all —
    /// so the frame it is authored in would mean another place once it
    /// reads the spliced node.
    MateFrameCrosses {
        /// The host mate.
        mate: RecipeNodeId,
        /// Which of its sides.
        side: crate::mate::MateSide,
    },
    /// **A `FromFace` side would cross the seam**: a host mate's side
    /// that reads the instance names its frame as a face of the
    /// referenced document, in that document's spelling, and once the
    /// side reads the spliced inner node the name is not a row of the
    /// inner instance's part. The re-spelling is not built, so inline
    /// refuses rather than leave a frame naming nothing.
    MateFaceFrameCrosses {
        /// The host mate.
        mate: RecipeNodeId,
        /// Which of its sides.
        side: crate::mate::MateSide,
    },
    /// **Two placing mates of one pair would read two pairs** (A4's
    /// fold rule): they relate the instance to one host member, and
    /// re-anchored they would relate two inner instances to it, which
    /// the solve would fold as two pairs rather than one.
    MatePairSplits {
        /// The earlier mate.
        first: RecipeNodeId,
        /// The mate that reads another inner instance.
        second: RecipeNodeId,
    },
    /// A host reference names the instance's own OUTPUT BODY. The
    /// spliced recipe has no single node whose body corresponds to the
    /// instance's placed product, so the reference cannot re-anchor.
    InstanceBodyNameReferenced {
        /// The instance-body name.
        name: Box<StableName>,
    },
    /// A host reference derives from the instance but is not the
    /// bridge's own `InPart`-wrapped form, so this door does not know
    /// what local name corresponds to it.
    ForeignInstanceName {
        /// The unrecognized name.
        name: Box<StableName>,
    },
    /// A name the inline carries spells a piece of a profile step no
    /// profile of the referenced document holds — a step a
    /// `SetProgram` dropped there (DM7). The host would mint the id
    /// afresh for a step of its own, so the name cannot cross.
    NameOnDroppedStep {
        /// The name.
        name: Box<StableName>,
        /// The dropped step it spells.
        step: StepId,
    },
    /// A name to be spliced derives from a node the referenced
    /// document no longer has (an N5-stranded reference) — there is no
    /// node to remap it onto; repair it in the part document first.
    StrandedPartName {
        /// The stranded name.
        name: Box<StableName>,
        /// The node the referenced document no longer has. For a
        /// nested name it is a node inside one of `name`'s path
        /// segments, not `name`'s own minting node.
        missing: RecipeNodeId,
    },
    /// Replaying the constructed edits refused — a construction bug in
    /// this module or a host/part state the edit vocabulary cannot
    /// re-author. Surfaced typed, never absorbed.
    Edit {
        /// The refusing edit's own diagnosis.
        error: Box<EditError>,
    },
    /// The host's insert door minted a spliced profile's step under an
    /// id other than the one the inline precomputed for it — the same
    /// construction bug as [`SplitError::StepMapDiverged`].
    StepMapDiverged(StepMapDivergence),
}

impl core::fmt::Display for InlineError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::UnknownNode { id } => write!(f, "inline: node {} is not live", id.0),
            Self::NotAnInstance { node } => {
                write!(f, "inline: node {} does not instantiate a part", node.0)
            }
            Self::InstanceConsumed { node, by } => write!(
                f,
                "inline: instance {} is consumed by node {} — the recipe cannot rewire a \
                 consumer onto a spliced product",
                node.0, by.0
            ),
            Self::Unresolved { failure } => {
                write!(f, "inline: the reference did not resolve: {failure}")
            }
            Self::EpsilonSeam { host_eps, part_eps } => write!(
                f,
                "inline: the referenced document records tolerance {part_eps:e} but the host \
                 records {host_eps:e} — one document, one ε"
            ),
            Self::PartCarriesMetadata { key } => write!(
                f,
                "inline: the referenced document carries metadata ({key:?}) the edit vocabulary \
                 cannot splice — refused rather than dropped"
            ),
            Self::ParamConflict { param } => write!(
                f,
                "inline: parameter {param} is declared by both documents with different values"
            ),
            Self::UnplaceableFrame { root } => write!(
                f,
                "inline: the instance sits off the world's origin, but part root {} is plain \
                 recipe geometry, which sits on no gauge — the frame is not expressible \
                 locally",
                root.0
            ),
            Self::MatePlaced { instance } => write!(
                f,
                "inline: instance {i} carries no offset of its own, so where it sits is a \
                 solve result the splice cannot state. {}",
                Recourse(&format!(
                    "give instance {i} an offset (SetOffset), then inline",
                    i = instance.0
                )),
                i = instance.0
            ),
            Self::Unplaced { instance, cause } => write!(
                f,
                "inline: instance {} is unplaced, because {cause}. {}",
                instance.0,
                Recourse(crate::mate::UNPLACED_RECOURSE)
            ),
            Self::NeedsAGauge { instance } => write!(
                f,
                "inline: splicing instance {i}'s part at its offset needs a gauge to hold the \
                 offset, which inline does not mint yet. {}",
                Recourse(&format!(
                    "set instance {i}'s offset to the empty chain (SetOffset), then inline",
                    i = instance.0
                )),
                i = instance.0
            ),
            Self::PartDeadGauge { node } => write!(
                f,
                "inline: the referenced document's node {} names a deleted gauge, which has no \
                 node in the host — repair it in the referenced document before inlining",
                node.0
            ),
            Self::MateFrameCrosses { mate, side } => write!(
                f,
                "inline: mate {m}'s {} side would read an inner node that is not its part \
                 group's root at the empty offset on the part's world, so its frame would mean \
                 another place. {}",
                side.name(),
                Recourse(&format!("delete mate {m}, then inline", m = mate.0)),
                m = mate.0
            ),
            Self::MateFaceFrameCrosses { mate, side } => write!(
                f,
                "inline: mate {m}'s {} side reads the instance and its frame names a face of the \
                 referenced document, a name the inner instance's part does not carry, so the \
                 frame would name nothing. {}",
                side.name(),
                Recourse(&format!(
                    "author mate {m}'s {} frame as numbers, or delete mate {m}, then inline",
                    side.name(),
                    m = mate.0
                )),
                m = mate.0
            ),
            Self::MatePairSplits { first, second } => write!(
                f,
                "inline: mates {} and {} place one pair, and re-anchored they would read two. {}",
                first.0,
                second.0,
                Recourse(&format!("delete mate {}, then inline", second.0))
            ),
            Self::InstanceBodyNameReferenced { name } => write!(
                f,
                "inline: the {name} names the instance's own output body, which no single \
                 spliced node corresponds to"
            ),
            Self::ForeignInstanceName { name } => write!(
                f,
                "inline: the {name} derives from the instance but is not an instance-qualified \
                 (`InPart`) name — it cannot re-anchor"
            ),
            Self::NameOnDroppedStep { name, step } => write!(
                f,
                "inline: the {name} spells a piece of the profile step minted #{}, which no profile of the \
                 referenced document draws any more — repair the stranded reference before \
                 inlining",
                step.0
            ),
            Self::StrandedPartName { name, missing } => write!(
                f,
                "inline: the {name} derives from node {}, which the referenced document no \
                 longer has — repair the stranded reference before inlining",
                missing.0
            ),
            Self::Edit { error } => write!(
                f,
                "inline: an edit refused: {}{}",
                error.problem(),
                ReplayTail(error, Replay::Inline)
            ),
            Self::StepMapDiverged(d) => write!(f, "inline: {d}"),
        }
    }
}

impl core::error::Error for InlineError {}

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
            // it — a Declare or a blend selection rebound forward. The
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
            | EditError::DuplicateInput { .. }
            | EditError::RepeatedDesignation { .. }
            | EditError::SelectionNotCanonical { .. }
            | EditError::SetMembersOnNonList { .. }
            | EditError::SetProgramOnNonProfile { .. }
            | EditError::StepIdsRefused { .. }
            | EditError::TooFewMembers { .. }
            | EditError::DeleteWouldDangle { .. }
            | EditError::UnknownSlot { .. }
            | EditError::SlotDimensionMismatch { .. }
            | EditError::StructuralSlotNeedsStructuralEdit { .. }
            | EditError::NotStructuralSlot { .. }
            | EditError::SlotUnknownDocParam { .. }
            | EditError::SlotDocParamDimension { .. }
            | EditError::PayloadUnknownDocParam { .. }
            | EditError::PayloadDocParamDimension { .. }
            | EditError::MeasureMalformed { .. }
            | EditError::AssertionTarget { .. }
            | EditError::DeclareInputNotDeclare { .. }
            | EditError::AssertionDimension { .. }
            | EditError::ContinuousParamCannotBeCount { .. }
            | EditError::DocParamNotDeclared { .. }
            | EditError::DocParamCountHasNoUnit { .. }
            | EditError::DocParamCountHasNoDistribution { .. }
            | EditError::DocParamUnitMismatch { .. }
            | EditError::DocParamValueKindMismatch { .. }
            | EditError::PathOffTree { .. }
            | EditError::Dimension(_)
            | EditError::NameStepNeverMinted { .. }
            | EditError::ReadSiteMissingNode { .. }
            | EditError::NonFiniteDocParam { .. }
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
            | EditError::AppearanceNotSet { .. }
            | EditError::InvalidTolerance { .. }
            | EditError::MetaUnversioned { .. }
            | EditError::MetaNonFinite { .. }
            | EditError::MetaNotSet { .. }
            | EditError::Roots(_)
            | EditError::OffsetOnNonInstance { .. }
            | EditError::GaugeOnNonPlaced { .. }
            | EditError::GaugeNotLive { .. }
            | EditError::NotAGauge { .. }
            | EditError::GaugeCycle { .. }
            | EditError::PlacementRuleMismatch { .. }
            | EditError::EmptyPlacementList { .. }
            | EditError::ImproperPlacement { .. }
            | EditError::NonFinitePlacement { .. }
            | EditError::NonRigidPlacement { .. }
            | EditError::PlacementAxis { .. }
            | EditError::NonFiniteAlignment { .. }
            | EditError::UpdateOnNonInstance { .. }
            | EditError::PinUnchanged { .. } => defect(f),
        }
    }
}

/// What [`split`] produced: the two documents, the recorded edits
/// that produce each (the part's from the empty document under the
/// caller's id, the remainder's from the input document), and the
/// [`crate::Maintenance`] each edit list performed. Undo of the
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
    /// The maintenance `remainder_edits` reported, in edit order
    /// ([`Maintenance`]): every payload name a departing cut node
    /// stranded behind it (DM7). An accepted edit travels whole, so the
    /// outcome carries what its edits DID beside what they produced: a
    /// caller holding a document with the maintenance of its last
    /// accepted edit swaps `remainder` and this in together.
    pub remainder_maintenance: Vec<Maintenance>,
    /// The recorded edits producing `part` from
    /// `Doc::empty(part_id)`.
    pub part_edits: Vec<DocEdit<ProfileProgram>>,
    /// The maintenance `part_edits` reported, in edit order
    /// ([`Maintenance`]): a cut mate that joins two groups as it lands
    /// clears its first operand's root offset, as at every mate insert.
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
/// that produce it and the maintenance they performed.
/// Undo is the caller keeping the input value.
#[derive(Debug, Clone)]
pub struct InlineOutcome {
    /// The host document after the splice.
    pub doc: ProfileDoc,
    /// The recorded edits producing `doc` from the input.
    pub edits: Vec<DocEdit<ProfileProgram>>,
    /// The maintenance `edits` reported, in edit order
    /// ([`Maintenance`]): a spliced mate that joins two groups as it
    /// lands clears its first operand's root offset, as at every mate
    /// insert.
    /// An accepted edit travels whole; a caller holding
    /// a document with the maintenance of its last accepted edit swaps
    /// `doc` and this in together.
    pub maintenance: Vec<Maintenance>,
    /// Part-document node ids → their host ids (minted in the part's
    /// document order).
    pub node_map: NodeMap,
    /// The part's profile step ids → the ids the host minted for them,
    /// in the same order.
    pub step_map: StepMap,
}

/// A document under reconstruction by recorded edits: the value so
/// far, the edits that produce it, and the maintenance those edits
/// performed. The ONE place a refactoring takes an
/// accepted edit up, which is what keeps each [`apply`] result's
/// document and maintenance together — the record's minted id goes
/// back to the caller, and its `structural` bit is a fact of the edit
/// already in the list — so an outcome built from one reports what
/// its edits did, never only what they produced.
struct Recording {
    doc: ProfileDoc,
    edits: Vec<DocEdit<ProfileProgram>>,
    maintenance: Vec<Maintenance>,
}

impl Recording {
    fn start(doc: ProfileDoc) -> Self {
        Self {
            doc,
            edits: Vec::new(),
            maintenance: Vec::new(),
        }
    }

    /// Apply one edit and record it: the new document replaces the
    /// held one, the edit joins the list, and the maintenance the edit
    /// performed is appended in edit order. Returns the id the edit
    /// minted, if any.
    ///
    /// # Errors
    ///
    /// The edit's own refusal; nothing is recorded on that arm.
    fn apply(
        &mut self,
        edit: DocEdit<ProfileProgram>,
        tol: Tol,
        reach: &dyn crate::mate::MateReach,
    ) -> Result<Option<RecipeNodeId>, EditError> {
        let applied = apply(&self.doc, &edit, tol, reach)?;
        self.doc = applied.doc;
        self.maintenance.extend(applied.maintenance);
        self.edits.push(edit);
        Ok(applied.record.minted)
    }
}

// ---- Name and node remapping ----

/// Rewrites every LOCAL node id in `name` through `map` — the minting
/// node, embedded operand names, and discriminator partners: exactly
/// the id set [`derivation_nodes`] reads, because that is the set that
/// belongs to THIS document's id space. `InPart` arguments cross
/// VERBATIM (they name another document's nodes — the walk_names seam
/// rule). The path is walked by `StableName::rewrite_path`, which
/// puts it back in canonical form: the map need not preserve id order
/// (it follows document order, and a loaded document's order is not
/// its id order), so every name-ordered position may come out
/// reordered.
///
/// Public because a name held outside the document — a caller's own
/// reference into a split or inlined part — crosses the same map the
/// split and the inline apply to the names they carry, and has to come
/// out spelled the way theirs do.
///
/// # Errors
///
/// The first local id the map lacks.
pub fn remap_name(
    name: &StableName,
    map: &NodeMap,
    steps: &StepMap,
) -> Result<StableName, Unmapped> {
    let (node, path) = remap_derivation(name.kind, name.node, &name.path, map, steps)?;
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
/// Split out because that is exactly what a face name may have
/// rewritten — `FaceName::map_derivation` hands this function the
/// derivation and keeps the kind itself, which is what makes
/// [`remap_face`] total without an arm for a kind change.
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
) -> Result<(RecipeNodeId, crate::names::RolePath), Unmapped> {
    let to = *map.get(&node).ok_or(Unmapped::Node(node))?;
    let rewritten = StableName {
        kind,
        node,
        path: path.to_vec(),
    }
    .rewrite_path(&mut Remapping(map, steps))?;
    Ok((to, rewritten.into_path()))
}

/// **The split re-map as a [`SegRewrite`]**: every carried name is
/// rewritten through [`remap_name`] — its minting node through the
/// map, then its own path through this same rewriter, so the descent
/// is [`remap_name`]'s and not the walk's — a member edge, which is a
/// local node id like the minting one, is mapped too, and so is every
/// profile locator's step, which the other document re-minted. A
/// kernel-built section's locator has no step and crosses as it is.
/// The walk over [`RoleSeg`]'s shape is [`RoleSeg::rewrite`]'s.
struct Remapping<'a>(&'a NodeMap, &'a StepMap);

impl Remapping<'_> {
    fn step(&self, step: StepId) -> Result<StepId, Unmapped> {
        self.1.get(&step).copied().ok_or(Unmapped::Step(step))
    }
}

impl SegRewrite for Remapping<'_> {
    type Error = Unmapped;

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
        self.member(n.node)?;
        Ok(Carry::Descend)
    }

    fn descended(
        &mut self,
        n: &StableName,
        mut walked: StableName,
    ) -> Result<Option<StableName>, Self::Error> {
        walked.node = self.member(n.node)?;
        Ok(Some(walked))
    }

    fn member(&mut self, m: RecipeNodeId) -> Result<RecipeNodeId, Self::Error> {
        self.0.get(&m).copied().ok_or(Unmapped::Node(m))
    }
}

/// [`remap_name`] for a FACE name — the ONE answer this crate gives to
/// "remap a face name across the split", and the only in-crate place a
/// [`FaceName`] is re-made from a rewritten one.
///
/// The kind is not rewritten and cannot be: `FaceName::map_derivation`
/// is handed the derivation alone and keeps the kind itself, so the
/// only thing that can go wrong is the thing [`remap_name`]'s own
/// errors are about — a local id the map lacks. It reports that id,
/// exactly as [`remap_name`] does, so the two rewrites answer in one
/// vocabulary and neither drops WHICH node was missing on the way to a
/// caller's name-shaped refusal. [`remap_node`] is the one place the
/// id is paired with the face it came from, as [`RemapMiss::Name`].
///
/// # Errors
///
/// The first local id the map lacks.
fn remap_face(name: &FaceName, map: &NodeMap, steps: &StepMap) -> Result<FaceName, Unmapped> {
    name.map_derivation(|node, path| remap_derivation(EntityKind::Face, node, path, map, steps))
}

/// What a payload rewrite could not map: a DAG input (unreachable
/// after the severed-edge check — surfaced as the edit layer's own
/// unresolved-input refusal rather than assumed away) or a referenced
/// name.
enum RemapMiss {
    /// An unmapped DAG input.
    Input(RecipeNodeId),
    /// A name whose local ids the map lacks, and the FIRST such id.
    /// The two are not redundant: a [`StableName`] embeds other names
    /// in its path, so the id the rewrite stopped at may belong to a
    /// name several segments down rather than to `name` itself.
    Name {
        /// The name that could not be rewritten.
        name: Box<StableName>,
        /// The local id the maps lack.
        missing: Unmapped,
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
    id: &impl Fn(RecipeNodeId) -> Result<RecipeNodeId, RemapMiss>,
) -> Result<PatternKind, RemapMiss> {
    Ok(match kind {
        PatternKind::Linear { .. } | PatternKind::Explicit(_) => kind.clone(),
        PatternKind::Circular { axis, step } => PatternKind::Circular {
            axis: id(*axis)?,
            step: step.clone(),
        },
    })
}

/// Rewrites a node payload's id references — DAG inputs AND
/// name-reference payloads — through `map`, and gauge references
/// through `regauge`, for insertion into the other document.
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
    map: &NodeMap,
    steps: &StepMap,
    regauge: &dyn Fn(Option<RecipeNodeId>) -> Result<Option<RecipeNodeId>, RemapMiss>,
) -> Result<Node<ProfileProgram>, RemapMiss> {
    let id = |n: RecipeNodeId| -> Result<RecipeNodeId, RemapMiss> {
        map.get(&n).copied().ok_or(RemapMiss::Input(n))
    };
    let nm = |n: &StableName| {
        remap_name(n, map, steps).map_err(|missing| RemapMiss::Name {
            name: Box::new(n.clone()),
            missing,
        })
    };
    // A mate head across the cut, through the one face remap
    // (`remap_face`): its derivation is rewritten and its kind is the
    // type's, so the only miss is the miss `nm` reports for a bare
    // name.
    let face = |n: &FaceName| {
        remap_face(n, map, steps).map_err(|missing| RemapMiss::Name {
            name: Box::new((**n).clone()),
            missing,
        })
    };
    Ok(match node {
        // **An in-plane axis is not a leaf**: its frame is an input,
        // and a clone would carry the OTHER document's node number
        // across the cut — exactly the trap the profile's plane hit
        // one rung down, where this arm cloned because a profile
        // referenced nothing.
        Node::Datum(crate::Datum::AxisInPlane {
            plane,
            origin,
            direction,
        }) => Node::Datum(crate::Datum::AxisInPlane {
            plane: id(*plane)?,
            origin: origin.clone(),
            direction: direction.clone(),
        }),
        // A derived frame is not a leaf either: its body is an input
        // and its face is a frozen name, and both cross the cut or
        // the remap misses loudly — exactly a blend's target and
        // selection.
        Node::Datum(crate::Datum::FaceFrame { at, face, spin }) => {
            Node::Datum(crate::Datum::FaceFrame {
                at: id(*at)?,
                face: nm(face)?,
                spin: spin.clone(),
            })
        }
        Node::Datum(
            crate::Datum::Plane { .. }
            | crate::Datum::Axis { .. }
            | crate::Datum::Point { .. }
            | crate::Datum::Frame { .. },
        ) => node.clone(),
        // A profile's PLANE is an input like any other: it crosses the
        // cut with the profile or the remap misses loudly. (Before the
        // sketch frame became a node this arm cloned, because a
        // profile referenced nothing.)
        //
        // Its step ids do NOT cross: the other document's insert door
        // mints its own, and the names that spell them cross through
        // the step map precomputed from that minting (`step_map_of`).
        Node::Profile(p) => Node::Profile(ProfileProgram {
            plane: id(p.plane)?,
            loops: p.loops.clone(),
            ids: Vec::new(),
        }),
        Node::Extrude { profile, distance } => Node::Extrude {
            profile: id(*profile)?,
            distance: distance.clone(),
        },
        Node::Revolve {
            profile,
            axis,
            angle,
        } => Node::Revolve {
            profile: id(*profile)?,
            axis: id(*axis)?,
            angle: angle.clone(),
        },
        // The two tube kinds remap the same way — one spine edge, every
        // other field carried — and are written apart rather than
        // through a helper, so which fields each kind has stays
        // readable at the site that has to name them all.
        Node::Tube {
            spine,
            u_ref,
            major_radius,
            window,
            minor_radius,
        } => Node::Tube {
            spine: id(*spine)?,
            u_ref: u_ref.clone(),
            major_radius: major_radius.clone(),
            window: window.clone(),
            minor_radius: minor_radius.clone(),
        },
        Node::HollowTube {
            spine,
            u_ref,
            major_radius,
            window,
            minor_radius,
            wall,
        } => Node::HollowTube {
            spine: id(*spine)?,
            u_ref: u_ref.clone(),
            major_radius: major_radius.clone(),
            window: window.clone(),
            minor_radius: minor_radius.clone(),
            wall: wall.clone(),
        },
        Node::Loft { profiles, v_degree } => Node::Loft {
            profiles: profiles.iter().map(|&p| id(p)).collect::<Result<_, _>>()?,
            v_degree: v_degree.clone(),
        },
        Node::Sweep {
            profile,
            path,
            stations,
            v_degree,
        } => Node::Sweep {
            profile: id(*profile)?,
            path: id(*path)?,
            stations: stations.clone(),
            v_degree: v_degree.clone(),
        },
        Node::Fillet {
            target,
            radius,
            selection,
        } => Node::fillet(
            id(*target)?,
            radius.clone(),
            selection.iter().map(nm).collect::<Result<_, _>>()?,
        ),
        Node::Chamfer {
            target,
            distance,
            selection,
        } => Node::chamfer(
            id(*target)?,
            distance.clone(),
            selection.iter().map(nm).collect::<Result<_, _>>()?,
        ),
        // Through the construction door, which keeps the designation
        // order and drops only a repeat: a remap never re-sorts an
        // ordered payload.
        Node::Shell {
            target,
            thickness,
            open,
        } => Node::shell(
            id(*target)?,
            thickness.clone(),
            open.iter().map(nm).collect::<Result<_, _>>()?,
        ),
        Node::Split { target, tool } => Node::Split {
            target: id(*target)?,
            tool: id(*tool)?,
        },
        Node::Boolean { op, a, b, declare } => Node::Boolean {
            op: *op,
            a: id(*a)?,
            b: id(*b)?,
            declare: declare.map(id).transpose()?,
        },
        Node::Union { members, declare } => Node::Union {
            members: members.iter().map(|&m| id(m)).collect::<Result<_, _>>()?,
            declare: declare.map(id).transpose()?,
        },
        Node::Transform { input, placement } => Node::Transform {
            input: id(*input)?,
            placement: placement.clone(),
        },
        Node::Pattern { input, count, kind } => Node::Pattern {
            input: id(*input)?,
            count: count.clone(),
            kind: remap_rule(kind, &id)?,
        },
        // The selector is payload with no id in it (a half, or an
        // index expression); only the edge remaps.
        Node::Part { of, select } => Node::Part {
            of: id(*of)?,
            select: select.clone(),
        },
        Node::PlacedUnion { input, count, kind } => Node::PlacedUnion {
            input: id(*input)?,
            count: count.clone(),
            kind: remap_rule(kind, &id)?,
        },
        // A declared pair's two halves remap like a mate's: the
        // NAME through the name door and the SITE through the id
        // door, because a site is a node id. Either one the cut
        // severed makes the remap MISS loudly.
        Node::Declare { pairs } => Node::Declare {
            pairs: pairs
                .iter()
                .map(|((a, b), class)| {
                    Ok((
                        (
                            crate::node::SitedRef::new(id(a.at)?, nm(&a.name)?),
                            crate::node::SitedRef::new(id(b.at)?, nm(&b.name)?),
                        ),
                        *class,
                    ))
                })
                .collect::<Result<_, RemapMiss>>()?,
        },
        // The reference crosses verbatim (the function's docs say why);
        // the gauge it sits on is the one id it holds, and the door
        // says where a gauge reference lands across the seam.
        Node::InstantiatePart {
            doc_ref,
            interface,
            gauge,
            offset,
        } => Node::InstantiatePart {
            doc_ref: *doc_ref,
            interface: interface.clone(),
            gauge: regauge(*gauge)?,
            offset: offset.clone(),
        },
        Node::Gauge { parent, placement } => Node::Gauge {
            parent: regauge(*parent)?,
            placement: placement.clone(),
        },
        // A mate's references cross the cut like any other name
        // reference, and BOTH halves of each remap: the NAME through
        // the name door, and the OPERAND through the id door, because
        // an operand is a node id. Either one the cut severed makes
        // the remap MISS loudly.
        Node::Mate {
            a,
            b,
            class,
            alignment,
        } => Node::Mate {
            a: crate::node::SitedFace {
                at: id(a.at)?,
                name: face(&a.name)?,
            },
            b: crate::node::SitedFace {
                at: id(b.at)?,
                name: face(&b.name)?,
            },
            class: *class,
            // The datum crosses verbatim: its vectors are numbers, and
            // a `FromFace` side's name is a row of the PART's table,
            // in the part's own id space, which no cut of this
            // document moves — it crosses as the instance's own
            // reference does.
            alignment: alignment.clone(),
        },
        // A measure's references are BOTH names and edges, so they
        // remap through the name door exactly once — `nm` rewrites the
        // embedded minting node id, which is what the edge is derived
        // from.
        // Both halves remap: the NAME through the name door, and the
        // reading SITE through the id door, because a measure's site
        // is an ordinary input edge.
        Node::Measure { expr, refs } => Node::Measure {
            expr: expr.clone(),
            refs: refs
                .iter()
                .map(|r| {
                    Ok(crate::node::SitedRef {
                        at: id(r.at)?,
                        name: nm(&r.name)?,
                    })
                })
                .collect::<Result<_, RemapMiss>>()?,
        },
        Node::Assertion {
            measure,
            bound,
            dir,
        } => Node::Assertion {
            measure: id(*measure)?,
            bound: bound.clone(),
            dir: *dir,
        },
    })
}

/// The document parameters a node's expressions reference, by name.
fn node_param_refs(node: &Node<ProfileProgram>) -> BTreeSet<crate::doc::ParamName> {
    let mut refs = Vec::new();
    for slot in node.slots() {
        if let Some(expr) = node.expr(slot) {
            expr.param_refs(&mut refs);
        }
    }
    // The expressions no slot addresses count too: a measured bound
    // referencing a parameter is exactly as much a reason to copy that
    // parameter into a split part as an extrude's distance is.
    for expr in crate::node::payload_exprs(node).into_iter().flatten() {
        expr.param_refs(&mut refs);
    }
    refs.into_iter().map(|(name, _)| name).collect()
}

// ---- Split ----

/// Cuts `cut` out of `doc` into a new document under `part_id` (the
/// caller's identity — [`DocumentId::derive`] for deterministic
/// callers, the document layer's `random_document_id` for interactive
/// authoring), leaving one [`Node::InstantiatePart`] of it behind.
/// Semantics: module docs. Pure — `doc` is untouched; undo is keeping
/// it.
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
        if doc.node(id).is_none() {
            return Err(SplitError::UnknownCutNode { id });
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
    // D-2's closure rule: no recipe edge crosses the cut, in either
    // direction (module docs).
    for &consumer in doc.order() {
        let Some(node) = doc.node(consumer) else {
            continue;
        };
        for input in node.inputs() {
            if cut.contains(&consumer) != cut.contains(&input) {
                return Err(SplitError::SeveredEdge {
                    consumer,
                    input,
                    consumer_is_cut: cut.contains(&consumer),
                });
            }
        }
    }
    // A11's group precondition, checked FOR REAL now that mates can
    // make a group multi-node (this module's docs have promised the
    // re-check since ASM-4; review MAJOR-2 found it missing). Run
    // beside the severed-edge check, before anything moves: a torn
    // group is a refusal, not a case the hoist below silently
    // declines to handle.
    let groups = crate::mate::groups(doc);
    for members in &groups {
        let (root, _) = crate::mate::solve::root_and_cause(doc, members);
        let root_is_cut = cut.contains(&root);
        if let Some(&instance) = members.iter().find(|id| cut.contains(id) != root_is_cut) {
            return Err(SplitError::TornGroup {
                root,
                instance,
                root_is_cut,
            });
        }
    }
    // The READING edge's own closure rule (A12). An operand is not an
    // input, so the severed-edge loop never sees it — and a mate separated
    // from the node its reference is read at is expressible in
    // neither document: the remainder would keep an id it no longer
    // has, and the part has no node to remap onto. Checked in BOTH
    // directions.
    //
    // The exception is the interface crossing (below): a kept mate
    // whose name re-anchors carries its at-mint operand with it.
    //
    // AFTER the group precondition on purpose: a mate that WELDS
    // its two members is the case `TornGroup` already speaks to,
    // and it is the more informative refusal — it names the group
    // the cut tears rather than one of its edges. This arm catches
    // what is left: a mate whose reference resolves to no member
    // welds nothing, and its operand still crosses.
    for &mate in doc.order() {
        let Some(Node::Mate { a, b, .. }) = doc.node(mate) else {
            continue;
        };
        let mate_is_cut = cut.contains(&mate);
        for (side, r) in [(crate::mate::MateSide::A, a), (crate::mate::MateSide::B, b)] {
            if cut.contains(&r.at) == mate_is_cut {
                continue;
            }
            // THE ONE CROSSING THAT IS ALREADY CARRIED, and it is the
            // interface crossing this module is built around: a KEPT
            // mate whose reference is read AT ITS OWN MINT and whose
            // NAME lies wholly inside the cut. That name re-anchors
            // through the minted instance's `InPart` wrapper below,
            // and `Rebind` moves an at-mint operand with the name it
            // rewrites — so the operand arrives at the new instance
            // with everything else. Nothing here to refuse.
            let carried_by_the_rebind =
                !mate_is_cut && r.at == r.name.node && derivation_nodes(&r.name).is_subset(cut);
            if carried_by_the_rebind {
                continue;
            }
            return Err(SplitError::OperandSeveredFromMate {
                mate,
                side,
                operand: r.at,
                mate_is_cut,
            });
        }
    }
    // A4's gauge rules. A gauge in the cut is the gauge hoist's and
    // the severed-gauge rule's, neither built yet; every cut instance
    // then names a gauge outside the cut, and those references must
    // land on ONE anchor, which the instance left behind names.
    let cut_instances: Vec<RecipeNodeId> = doc
        .order()
        .iter()
        .copied()
        .filter(|id| cut.contains(id))
        .filter(|&id| matches!(doc.node(id), Some(Node::InstantiatePart { .. })))
        .collect();
    if let Some(&gauge) = doc
        .order()
        .iter()
        .find(|id| cut.contains(id) && matches!(doc.node(**id), Some(Node::Gauge { .. })))
    {
        return Err(SplitError::CutHoldsGauge { gauge });
    }
    let mut anchor: Option<Option<RecipeNodeId>> = None;
    for &instance in &cut_instances {
        let gauge = doc.node(instance).and_then(Node::gauge_ref);
        match anchor {
            None => anchor = Some(gauge),
            Some(first) if first == gauge => {}
            Some(first) => {
                return Err(SplitError::TwoAnchors {
                    instance,
                    first,
                    second: gauge,
                });
            }
        }
    }
    let anchor = anchor.flatten();
    // The cut groups, each with its root and why it is unplaced.
    let cut_groups: Vec<(
        &Vec<RecipeNodeId>,
        RecipeNodeId,
        Option<crate::mate::Unplaced>,
    )> = groups
        .iter()
        .filter(|members| members.iter().any(|m| cut.contains(m)))
        .map(|members| {
            let (root, cause) = crate::mate::solve::root_and_cause(doc, members);
            (members, root, cause)
        })
        .collect();
    for &(_, root, cause) in &cut_groups {
        if let Some(crate::mate::Unplaced::DeadGauge { gauge }) = cause {
            return Err(SplitError::DeadGaugeReference {
                instance: root,
                gauge,
            });
        }
    }
    // Unplaced material alone: every node the cut holds that lives in a
    // space lives in an unplaced group's own.
    let spaces = crate::mate::solve::spaces_with(doc, |instance| {
        cut_groups
            .iter()
            .find(|(members, _, _)| members.contains(&instance))
            .and_then(|&(_, root, cause)| Some((root, cause?)))
    });
    let mut in_world = false;
    let mut first_own = None;
    for &id in cut {
        match spaces.space.get(&id) {
            Some(None) => in_world = true,
            Some(Some((group, _))) => {
                first_own.get_or_insert(*group);
            }
            None => {}
        }
    }
    if let (false, Some(group)) = (in_world, first_own) {
        return Err(SplitError::UnplacedAlone { group });
    }
    // The hoisted-frame case: the cut is exactly one placed GROUP — its
    // instances and the mates holding it together, nothing else — and
    // the instance left behind takes the root's offset while the part
    // lands the root at the empty chain (A4). Every other cut moves
    // verbatim, and the instance left behind sits at the empty offset.
    //
    // WHOLENESS is not a condition here — the torn-group precondition
    // above already refused every partial group, so a group reached by
    // the cut is entirely inside it.
    let hoisted = {
        let only_group_and_its_mates = cut.iter().all(|&id| {
            cut_instances.contains(&id) || matches!(doc.node(id), Some(Node::Mate { .. }))
        });
        match cut_groups.as_slice() {
            [(members, root, None)] if only_group_and_its_mates => Some((*members, *root)),
            _ => None,
        }
    };
    let offset_of = |id: RecipeNodeId| match doc.node(id) {
        Some(Node::InstantiatePart { offset, .. }) => offset.clone(),
        _ => None,
    };
    if let Some((members, root)) = hoisted
        && let Some(&instance) = members
            .iter()
            .find(|&&m| m != root && offset_of(m).is_some())
    {
        return Err(SplitError::HoistedMemberOffset { instance });
    }
    // The mates that cross the cut — a kept mate one of whose sides
    // reads the cut, the other a kept instance. A4 holds each to two
    // rules: it must not start placing once its cut side is read at the
    // instance left behind, which sits on the anchor (a placing one
    // tears its group, and a declaring one, or one whose cut side read
    // no instance, would start); and its cut side's coordinates must
    // not change, so an instance it reads must be, in the part, its
    // group's root at the empty chain on the part's world. A cut side
    // whose frame is `FromFace` refuses besides: its name is a row of
    // the cut instance's part, which the new part carries only wrapped
    // at the inner instance, and that re-spelling is not built.
    let root_lands_empty = |instance: RecipeNodeId| match hoisted {
        Some((_, root)) => root == instance,
        None => cut_groups.iter().any(|&(_, root, cause)| {
            root == instance
                && cause.is_none()
                && offset_of(root).is_some_and(|o| o.steps.is_empty())
        }),
    };
    for &mate in doc.order() {
        if cut.contains(&mate) {
            continue;
        }
        let Some(Node::Mate {
            a, b, alignment, ..
        }) = doc.node(mate)
        else {
            continue;
        };
        for (side, inner, outer, frame) in [
            (crate::mate::MateSide::A, a, b, &alignment.a),
            (crate::mate::MateSide::B, b, a, &alignment.b),
        ] {
            let Some(kept) = crate::mate::member_of(doc, outer) else {
                continue;
            };
            if !derivation_nodes(&inner.name).is_subset(cut) || cut.contains(&kept.instance) {
                continue;
            }
            if doc.node(kept.instance).and_then(Node::gauge_ref) == anchor {
                return Err(SplitError::WouldStartPlacing { mate });
            }
            if let Some(read) = crate::mate::member_of(doc, inner)
                && (!read.copy.is_empty()
                    || read.at != read.instance
                    || !root_lands_empty(read.instance))
            {
                return Err(SplitError::MateFrameCrosses { mate, side });
            }
            if frame.face().is_some() {
                return Err(SplitError::MateFaceFrameCrosses { mate, side });
            }
        }
    }
    // Parameters: referenced by cut nodes → copied into the part;
    // referenced by BOTH sides → refused (no silent sharing). The
    // remainder keeps its table either way — the edit vocabulary has
    // no parameter-removal arm, and an unreferenced parameter is legal
    // document state.
    let mut cut_refs: BTreeMap<crate::doc::ParamName, RecipeNodeId> = BTreeMap::new();
    let mut kept_refs: BTreeMap<crate::doc::ParamName, RecipeNodeId> = BTreeMap::new();
    for &id in doc.order() {
        let Some(node) = doc.node(id) else { continue };
        let into = if cut.contains(&id) {
            &mut cut_refs
        } else {
            &mut kept_refs
        };
        for name in node_param_refs(node) {
            into.entry(name).or_insert(id);
        }
    }
    for (param, &cut_node) in &cut_refs {
        if let Some(&kept_node) = kept_refs.get(param) {
            return Err(SplitError::UncutParamReference {
                param: param.clone(),
                cut_node,
                kept_node,
            });
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
                    .find(|id| !cut.contains(id));
                if let Some(missing) = outside {
                    return Err(SplitError::PartNameReachesRemainder {
                        node,
                        name: Box::new(name.clone()),
                        missing,
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
                name: Box::new(name.clone()),
                // The classification weighs the whole derivation set:
                // it is the SPLIT of that set across the cut that
                // refuses, so no one node is the culprit.
                missing: None,
            });
        }
        if name.kind == crate::names::EntityKind::Body {
            return Err(SplitError::BodyNameCrossesCut {
                name: Box::new(name.clone()),
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
            NameCarrier::Store { name } => classify(name)?,
        }
    }
    // The deterministic id remap: cut nodes in document order mint
    // part ids 0, 1, 2, … (D9 — two runs agree byte for byte).
    let node_map: NodeMap = doc
        .order()
        .iter()
        .filter(|id| cut.contains(id))
        .enumerate()
        .map(|(i, &old)| (old, RecipeNodeId(i as u64)))
        .collect();
    // The same for the cut profiles' steps: the part's insert door
    // mints them from the empty document's mint, in insertion order.
    let step_map = step_map_of(
        doc.order()
            .iter()
            .filter(|id| cut.contains(id))
            .filter_map(|id| doc.node(*id).map(|node| (*id, node))),
        &node_map,
        &StepMint::empty(),
    )
    .map_err(|error| SplitError::PartEdit {
        error: Box::new(error),
    })?;

    // ---- The part document, as recorded edits from empty ----
    // The part side's edits are inserts into a document being built —
    // a Join at most, never a moved root — so they lever through the
    // caller's own seam; the remainder side, below, needs more.
    let part_reach = crate::eval::PartReach::<f64>::with_resolver(resolver, tol);
    let mut part = Recording::start(Doc::empty(part_id, tol));
    let part_apply =
        |part: &mut Recording, edit: DocEdit<ProfileProgram>| -> Result<(), SplitError> {
            part.apply(edit, tol, &part_reach)
                .map(|_| ())
                .map_err(|error| SplitError::PartEdit {
                    error: Box::new(error),
                })
        };
    // The recorded ε carries over iff it differs from what the empty
    // document adopts (the committed process ε — the only value a
    // document this process can evaluate records anyway).
    if doc.epsilon().to_bits() != part.doc.epsilon().to_bits() {
        part_apply(&mut part, DocEdit::SetTolerance { eps: doc.epsilon() })?;
    }
    for param in cut_refs.keys() {
        // The reference was validated against this table, so the
        // declaration exists; a miss would refuse at the insert below.
        if let Some(value) = doc.params().get(param) {
            part_apply(
                &mut part,
                DocEdit::SetDocParam {
                    name: param.clone(),
                    value: value.clone(),
                },
            )?;
        }
    }
    // Every gauge reference leaves the cut for the anchor, which is the
    // part's world.
    let to_part_world = |_: Option<RecipeNodeId>| Ok(None);
    for &old in doc.order().iter().filter(|id| cut.contains(id)) {
        let Some(node) = doc.node(old) else { continue };
        let mut node =
            remap_node(node, &node_map, &step_map, &to_part_world).map_err(|miss| match miss {
                RemapMiss::Input(input) => SplitError::PartEdit {
                    error: Box::new(EditError::UnresolvedInput { input }),
                },
                RemapMiss::Name { name, missing } => SplitError::reaches(old, name, missing),
            })?;
        // The hoist lands the root at the empty chain.
        if let (Some((_, root)), Node::InstantiatePart { offset, .. }) = (hoisted, &mut node)
            && root == old
        {
            *offset = Some(crate::placement::Placement::IDENTITY);
        }
        part_apply(&mut part, DocEdit::InsertNode { node })?;
    }
    step_map_check(doc, &node_map, &step_map, &part.doc).map_err(SplitError::StepMapDiverged)?;
    // Witness DATA copies VERBATIM while node ids remap: sound because
    // a witness datum is sketch-self-relative — it selects among the
    // owning profile's own solution branches and embeds no other
    // node's identity, so there is no cross-id-space reference for the
    // remap to miss. A future witness vocabulary that embeds foreign
    // stable names must remap here or refuse.
    for (&old, &new) in &node_map {
        if let Some(witness) = doc.witness(old) {
            part_apply(
                &mut part,
                DocEdit::ReWitness {
                    node: new,
                    witness: witness.clone(),
                },
            )?;
        }
    }
    // A10: the cut roots keep their ROOT-LIST order (which insertion
    // order need not reproduce — the list may have been reordered).
    let part_roots: Vec<RecipeNodeId> = doc
        .roots()
        .iter()
        .filter_map(|r| node_map.get(r).copied())
        .collect();
    if part.doc.roots() != part_roots {
        part_apply(&mut part, DocEdit::SetRoots { roots: part_roots })?;
    }
    let pin = content_pin(&part.doc, tol).map_err(|error| SplitError::Pin {
        error: Box::new(error),
    })?;

    // ---- The interface record (ASM-R2b D-4; A4's seam) ----
    //
    // INVARIANT: a mate CROSSES iff its two references land on
    // opposite sides of the cut. A mate with both ends inside is
    // part-internal (its names rebind wholesale and it declares
    // nothing about the seam); a mate with both ends outside never
    // touched the cut. Collected in the pre-split document's node
    // order, which is what makes the record D9-deterministic.
    //
    // **Only a mate EDGE can cross.** A4 says "every mate EDGE
    // crossing the cut", and an A12 reading edge exists exactly when
    // both heads resolve to live MEMBERS of A11's vocabulary — a live
    // instance, or a pattern-placed instance (`Pattern` node +
    // `Instance(i)`). The gate is `crate::mate::member_of`
    // ITSELF, not a re-spelling of it: this collector, A12's reading
    // edges and A11's groups ask ONE predicate.
    //
    // # Which mates cross
    //
    // A crossing is a DECLARING mate: a placing one welds its members
    // into one group, and `TornGroup` above refused any cut that is not
    // a union of whole groups. Three facts make the name reading below
    // agree with that instance reading:
    //
    // 1. `Node::Mate::payload_names()` is exactly `[a, b]`, so a kept
    //    mate's two references are classified above: each is wholly
    //    inside the cut or wholly disjoint from it, never straddling
    //    (`NameStraddlesCut` refuses the third case). So `!inside`
    //    here means DISJOINT, not merely "not contained".
    // 2. `Node::Pattern::inputs()` includes `input`, so D-2's closure
    //    check refuses any cut with the pattern on one side and its
    //    input instance on the other: `pattern ∈ cut` iff
    //    `pattern.input ∈ cut`. A pattern-placed head's derivation
    //    nodes and the MEMBER it resolves to therefore always land on
    //    the same side. For a plain head the two are the same node.
    // 3. A placing edge's two members are welded into one group, which
    //    lies wholly on one side.
    //
    // So a mate this loop records is one whose ends sit on different
    // gauges — exactly the declarations AQ8 says the record carries.
    // `rev_fix_xsplit_unreachable.rs` exhausts the placing half over
    // every subset of two recipes.
    //
    // A mate with a DANGLING reference — one resolving to no member at
    // all — is not an edge and contributes NO crossing, however its
    // names fall across the cut. Such a mate never solved, so a record
    // minted from it would be trusted-at-rest state. Unlike (1)-(3),
    // this arm is NOT forced by the cut rules: a nested-pattern head
    // welds no group, so its mate's ends do reach opposite sides of
    // an accepted cut, and the gate is the only thing that skips it.
    // That is AQ8 option (b), SKIP — its home is
    // `crates/editor-core/ASSEMBLY.md`'s AQ8 clause, and `row5_d` in
    // `asm_r2b_assembly.rs` pins it. The mate itself stays in the
    // document (N5) and its names rebind like any other; it simply
    // says nothing about the seam.
    let is_mate_edge_end = |r: &crate::node::SitedFace| crate::mate::member_of(doc, r).is_some();
    let mut crossings: Vec<InterfaceCrossing> = Vec::new();
    for &id in doc.order() {
        if cut.contains(&id) {
            continue;
        }
        let Some(Node::Mate { a, b, class, .. }) = doc.node(id) else {
            continue;
        };
        // The edge gate, before the sides are even looked at.
        if !(is_mate_edge_end(a) && is_mate_edge_end(b)) {
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
        let inner = remap_face(inner, &node_map, &step_map)
            .map_err(|missing| SplitError::straddles(Box::new((**inner).clone()), missing))?;
        // The heads' own face names go through: the record carries
        // what the mate carries, so the split neither unwraps a head
        // nor re-asks the question its type already answered.
        crossings.push(InterfaceCrossing::Mate {
            class: *class,
            outer: outer.clone(),
            inner,
        });
    }

    // ---- The remainder, as recorded edits from the input ----
    let mut remainder = Recording::start(doc.clone());
    // The remainder inserts no mate — an instance, rebinds, deletes and
    // the root list — so none of its edits asks a reach.
    let rem_reach = crate::mate::RefusingReach;
    let rem_apply = |remainder: &mut Recording,
                     edit: DocEdit<ProfileProgram>|
     -> Result<Option<RecipeNodeId>, SplitError> {
        remainder
            .apply(edit, tol, &rem_reach)
            .map_err(|error| SplitError::RemainderEdit {
                error: Box::new(error),
            })
    };
    // The instance names the anchor, and the cut's one placed thing
    // gives it its offset: the hoisted root's, or the empty chain.
    let offset = match hoisted {
        Some((_, root)) => offset_of(root),
        None => Some(crate::placement::Placement::IDENTITY),
    };
    let minted = rem_apply(
        &mut remainder,
        DocEdit::InsertNode {
            node: Node::instantiate_part_with(
                DocRef { id: part_id, pin },
                InterfaceRecord { crossings },
                anchor,
                offset,
            ),
        },
    )?;
    let Some(instance) = minted else {
        // InsertNode always mints; surfaced typed rather than assumed.
        return Err(SplitError::RemainderEdit {
            error: Box::new(EditError::UnknownNode {
                id: RecipeNodeId(0),
            }),
        });
    };
    for from in &rebinds {
        let of = remap_name(from, &node_map, &step_map)
            .map_err(|missing| SplitError::straddles(Box::new(from.clone()), missing))?;
        let to = StableName {
            kind: from.kind,
            node: instance,
            path: vec![RoleSeg::InPart {
                of: NameRef::new(of),
            }],
        };
        rem_apply(
            &mut remainder,
            DocEdit::Rebind {
                from: from.clone(),
                to,
            },
        )?;
    }
    // Reverse document order deletes consumers before their inputs, so
    // no delete dangles a live reference.
    for &old in doc.order().iter().rev() {
        if cut.contains(&old) {
            rem_apply(&mut remainder, DocEdit::DeleteNode { id: old })?;
        }
    }
    // A10 on the remainder: the instance takes the FIRST cut root's
    // list position (the cut material's product order collapses onto
    // the instance); A10's automatic root-list bookkeeping appended it
    // instead — the list's own move, which the outcome's `maintenance`
    // fields do not report.
    let mut desired: Vec<RecipeNodeId> = Vec::new();
    let mut placed = false;
    for &r in doc.roots() {
        if cut.contains(&r) {
            if !placed {
                desired.push(instance);
                placed = true;
            }
        } else {
            desired.push(r);
        }
    }
    if remainder.doc.roots() != desired {
        rem_apply(&mut remainder, DocEdit::SetRoots { roots: desired })?;
    }
    Ok(SplitOutcome {
        remainder: remainder.doc,
        part: part.doc,
        remainder_edits: remainder.edits,
        remainder_maintenance: remainder.maintenance,
        part_edits: part.edits,
        part_maintenance: part.maintenance,
        instance,
        node_map,
        step_map,
    })
}

// ---- Inline ----

/// Splices the document `instance` references into `doc` and deletes
/// the instance — the inverse of [`split`] (D-3). The pin resolves
/// through `resolver` (a stale pin is the resolver's typed
/// `PinMismatch`, never a silent retarget); ids remap into fresh host
/// mints in the part's document order; `InPart`-wrapped names at the
/// instance re-anchor to the spliced local names; spliced instances'
/// placements are the host instance's frame COMPOSED onto the part's
/// own ([`crate::Frame::compose`]). Pure — `doc` is untouched; undo is
/// keeping it.
///
/// The instance's INTERFACE RECORD dissolves here (ASM-R2b D-4, the
/// inverse of split's populate): each crossing's part-side reference
/// is re-anchored to the local name the splice minted, which is
/// exactly what the wrapped-name rebind below does to the mate that
/// declared it — so once every crossing's inner name is confirmed to
/// land locally, the record has no remaining content and goes with the
/// deleted instance. Confirmed, not assumed: a crossing that does not
/// re-anchor refuses [`InlineError::StrandedPartName`] rather than
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
    // A spliced mate's clocking rider levers through the parts the
    // same resolver holds.
    let reach = crate::eval::PartReach::<f64>::with_resolver(Some(resolver), tol);
    let Some(node) = doc.node(instance) else {
        return Err(InlineError::UnknownNode { id: instance });
    };
    let Node::InstantiatePart {
        doc_ref,
        interface,
        gauge: host_gauge,
        ..
    } = node
    else {
        return Err(InlineError::NotAnInstance { node: instance });
    };
    // Who reads this node is `roots`' question, asked here for the
    // witness the refusal names.
    if let Some(by) = crate::roots::consumer(doc, instance) {
        return Err(InlineError::InstanceConsumed { node: instance, by });
    }
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
    // Where the instance sits: on its gauge `g`, at the offset `o` of
    // its own, as the root of its group (A11 (2)). An instance its
    // mates place, or an unplaced one, has no frame to splice at.
    let host_groups = crate::mate::groups(doc);
    let Some(group) = host_groups.iter().find(|g| g.contains(&instance)) else {
        unreachable!("a live instance is in one of its document's groups")
    };
    let offset = match crate::mate::solve::root_and_cause(doc, group) {
        (_, Some(cause)) => return Err(InlineError::Unplaced { instance, cause }),
        (root, None) if root != instance => return Err(InlineError::MatePlaced { instance }),
        (_, None) => match node {
            Node::InstantiatePart {
                offset: Some(offset),
                ..
            } => offset.clone(),
            _ => unreachable!("a placed group's root carries an offset"),
        },
    };
    // The part's content lands on the instance's gauge. A gauge
    // reference the part holds to one it deleted has no node here.
    for &id in part.order() {
        if let Some(g) = part.node(id).and_then(Node::gauge_ref)
            && part.node(g).is_none()
        {
            return Err(InlineError::PartDeadGauge { node: id });
        }
    }
    // Plain recipe geometry sits on no gauge, so it splices in place
    // only where the instance sits at the world's origin.
    if host_gauge.is_some() || !offset.steps.is_empty() {
        for &root in part.roots() {
            if !matches!(
                part.node(root),
                Some(Node::InstantiatePart { .. } | Node::Mate { .. } | Node::Gauge { .. })
            ) {
                return Err(InlineError::UnplaceableFrame { root });
            }
        }
    }
    // The part's groups, each with its root, read once for the shape
    // and for the frame rule below.
    let part_groups: Vec<(
        Vec<RecipeNodeId>,
        RecipeNodeId,
        Option<crate::mate::Unplaced>,
    )> = crate::mate::groups(&part)
        .into_iter()
        .map(|members| {
            let (root, cause) = crate::mate::solve::root_and_cause(&part, &members);
            (members, root, cause)
        })
        .collect();
    let at_the_empty_chain = |id: RecipeNodeId| {
        matches!(
            part.node(id),
            Some(Node::InstantiatePart {
                gauge: None,
                offset: Some(o),
                ..
            }) if o.steps.is_empty()
        )
    };
    let part_root_at_empty = |id: RecipeNodeId| {
        part_groups
            .iter()
            .any(|(_, root, cause)| *root == id && cause.is_none())
            && at_the_empty_chain(id)
    };
    // The two shapes built (A4): at the empty offset the content lands
    // on `g` verbatim; at any other, a part that is one group rooted at
    // the empty chain on its world, with no further offset, has its
    // root take `o` on `g`. Every other inline mints a gauge, and
    // refuses until that is built.
    let sugar_root = if offset.steps.is_empty() {
        None
    } else {
        let has_gauge = part
            .order()
            .iter()
            .any(|&id| matches!(part.node(id), Some(Node::Gauge { .. })));
        match part_groups.as_slice() {
            [(members, root, None)]
                if !has_gauge
                    && part_root_at_empty(*root)
                    && members.iter().all(|&m| {
                        m == *root
                            || matches!(
                                part.node(m),
                                Some(Node::InstantiatePart { offset: None, .. })
                            )
                    }) =>
            {
                Some(*root)
            }
            _ => return Err(InlineError::NeedsAGauge { instance }),
        }
    };
    // The frame rule and the fold rule (A4), over every host mate side
    // that reads the instance — its name wrapped at it, which the
    // rebind below re-anchors onto the inner name. The inner instance
    // must be its part group's root at the empty chain on the part's
    // world, so the frame means what it meant; and the placing mates
    // of one pair must still read one pair. A side whose frame is
    // `FromFace` refuses besides: its name is a row of the referenced
    // document, not of the inner instance's part, and the re-spelling
    // is not built.
    let mut pair_reads: Vec<(crate::mate::Member, RecipeNodeId, RecipeNodeId)> = Vec::new();
    for &mate in doc.order() {
        let Some(Node::Mate {
            a, b, alignment, ..
        }) = doc.node(mate)
        else {
            continue;
        };
        for (side, here, there, frame) in [
            (crate::mate::MateSide::A, a, b, &alignment.a),
            (crate::mate::MateSide::B, b, a, &alignment.b),
        ] {
            let [RoleSeg::InPart { of }] = &here.name.path[..] else {
                continue;
            };
            if here.name.node != instance {
                continue;
            }
            let inner = FaceName::new((**of).clone()).ok().and_then(|face| {
                crate::mate::member_of(&part, &crate::node::SitedFace::at_mint(face))
            });
            let Some(inner) = inner.filter(|m| {
                m.copy.is_empty() && m.at == m.instance && part_root_at_empty(m.instance)
            }) else {
                return Err(InlineError::MateFrameCrosses { mate, side });
            };
            if frame.face().is_some() {
                return Err(InlineError::MateFaceFrameCrosses { mate, side });
            }
            if let Some(other) = crate::mate::member_of(doc, there)
                && crate::mate::places(doc, instance, other.instance)
            {
                if let Some(&(_, first, _)) = pair_reads
                    .iter()
                    .find(|(m, _, read)| *m == other && *read != inner.instance)
                {
                    return Err(InlineError::MatePairSplits {
                        first,
                        second: mate,
                    });
                }
                pair_reads.push((other, mate, inner.instance));
            }
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
                name: Box::new(name.clone()),
            });
        }
        Err(InlineError::ForeignInstanceName {
            name: Box::new(name.clone()),
        })
    };
    // Every name the host document holds, in both carriers — the
    // classification is the same for each, so this asks the
    // enumeration for the names and nothing else.
    for carrier in doc.name_carriers() {
        classify(carrier.name())?;
    }
    wrapped.sort();
    wrapped.dedup();
    // The id remap is precomputed from the mint counter: parameter
    // edits mint nothing, so the part's nodes land on consecutive ids
    // starting at the host's next mint, in part document order — which
    // is what lets payloads with FORWARD name references (a rebound
    // Declare) remap before their targets are inserted.
    let node_map: NodeMap = part
        .order()
        .iter()
        .enumerate()
        .map(|(i, &old)| (old, RecipeNodeId(doc.next_id + i as u64)))
        .collect();
    // The part's profile steps are minted from the host's mint, which
    // inserts of the part's nodes alone extend.
    let step_map = step_map_of(
        part.order()
            .iter()
            .filter_map(|id| part.node(*id).map(|node| (*id, node))),
        &node_map,
        doc.step_mint(),
    )
    .map_err(|error| InlineError::Edit {
        error: Box::new(error),
    })?;

    let mut current = Recording::start(doc.clone());
    let step =
        |current: &mut Recording, edit: DocEdit<ProfileProgram>| -> Result<(), InlineError> {
            current
                .apply(edit, tol, &reach)
                .map(|_| ())
                .map_err(|error| InlineError::Edit {
                    error: Box::new(error),
                })
        };
    // Parameters merge only when they already agree bit for bit; a
    // disagreeing shared name refuses (no silent pick).
    for (name, value) in part.params() {
        match doc.params().get(name) {
            Some(existing) if existing.bit_eq(value) => {}
            Some(_) => {
                return Err(InlineError::ParamConflict {
                    param: name.clone(),
                });
            }
            None => step(
                &mut current,
                DocEdit::SetDocParam {
                    name: name.clone(),
                    value: value.clone(),
                },
            )?,
        }
    }
    // The part's world is the instance's gauge.
    let regauge = |g: Option<RecipeNodeId>| match g {
        None => Ok(*host_gauge),
        Some(g) => node_map
            .get(&g)
            .copied()
            .map(Some)
            .ok_or(RemapMiss::Input(g)),
    };
    for &old in part.order() {
        let Some(node) = part.node(old) else { continue };
        let mut node =
            remap_node(node, &node_map, &step_map, &regauge).map_err(|miss| match miss {
                RemapMiss::Input(input) => InlineError::Edit {
                    error: Box::new(EditError::UnresolvedInput { input }),
                },
                RemapMiss::Name { name, missing } => InlineError::stranded(name, missing),
            })?;
        // The sugar: the part's root takes the instance's offset.
        if let (Some(root), Node::InstantiatePart { offset: held, .. }) = (sugar_root, &mut node)
            && root == old
        {
            *held = Some(offset.clone());
        }
        step(&mut current, DocEdit::InsertNode { node })?;
    }
    step_map_check(&part, &node_map, &step_map, &current.doc)
        .map_err(InlineError::StepMapDiverged)?;
    // Witness data copies VERBATIM while ids remap — the same
    // invariant as split's copy: a witness datum is sketch-self-
    // relative and embeds no other node's identity (see split's
    // witness loop).
    for (&old, &new) in &node_map {
        if let Some(witness) = part.witness(old) {
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
    for (name, record) in part.appearance().iter() {
        let key = remap_name(name, &node_map, &step_map)
            .map_err(|missing| InlineError::stranded(Box::new(name.clone()), missing))?;
        for attr in record.attrs.values() {
            step(
                &mut current,
                DocEdit::SetAppearance {
                    name: key.clone(),
                    attr: attr.clone(),
                },
            )?;
        }
        for (meta_key, value) in &record.metadata {
            step(
                &mut current,
                DocEdit::SetAppearanceMeta {
                    name: key.clone(),
                    key: meta_key.clone(),
                    value: value.clone(),
                },
            )?;
        }
    }
    // The inverse of the bridge: wrapped names re-anchor to local.
    for from in &wrapped {
        // The collection admitted exactly this shape; a non-match here
        // would be a collection bug, and skipping it would strand the
        // name silently — so the shape is re-destructured, not assumed.
        let [RoleSeg::InPart { of }] = &from.path[..] else {
            return Err(InlineError::ForeignInstanceName {
                name: Box::new(from.clone()),
            });
        };
        let to = remap_name(of, &node_map, &step_map)
            .map_err(|missing| InlineError::stranded(Box::new((**of).clone()), missing))?;
        step(
            &mut current,
            DocEdit::Rebind {
                from: from.clone(),
                to,
            },
        )?;
    }
    // The record dissolves (ASM-R2b D-4, and the function docs): every
    // crossing's part-side reference must land on a spliced local
    // name. The declaration itself survives in the mate node, which is
    // in the host and whose wrapped reference the loop above just
    // re-anchored — so the record's job ends here, CHECKED.
    for crossing in &interface.crossings {
        let InterfaceCrossing::Mate { inner, .. } = crossing;
        remap_face(inner, &node_map, &step_map)
            .map_err(|missing| InlineError::stranded(Box::new((**inner).clone()), missing))?;
    }
    step(&mut current, DocEdit::DeleteNode { id: instance })?;
    // A10: the spliced roots take the instance's list position, in the
    // part's own root order.
    let mut desired: Vec<RecipeNodeId> = Vec::new();
    for &r in doc.roots() {
        if r == instance {
            desired.extend(part.roots().iter().filter_map(|p| node_map.get(p).copied()));
        } else {
            desired.push(r);
        }
    }
    if current.doc.roots() != desired {
        step(&mut current, DocEdit::SetRoots { roots: desired })?;
    }
    Ok(InlineOutcome {
        doc: current.doc,
        edits: current.edits,
        maintenance: current.maintenance,
        node_map,
        step_map,
    })
}

/// **A remap never changes a KIND.**
///
/// [`remap_name`] rewrites a name's derivation — its minting node and
/// the node ids embedded in its role path — and copies the kind
/// through untouched, for every [`crate::EntityKind`] and through a
/// nested name-bearing segment. That is the fact [`remap_face`] rests
/// on: it hands `FaceName::map_derivation` the derivation alone, so
/// the kind is the type's rather than the rewrite's, and the two
/// answers agree by construction.
#[cfg(test)]
#[allow(clippy::expect_used, clippy::panic)]
mod remap_keeps_the_kind {
    use super::{NodeMap, StepMap, Unmapped, remap_face, remap_name};
    use crate::names::{FaceName, NameRef, RoleSeg, StableName};
    use crate::node::RecipeNodeId;
    use crate::{CapEnd, EntityKind};

    fn map() -> NodeMap {
        [
            (RecipeNodeId(0), RecipeNodeId(10)),
            (RecipeNodeId(1), RecipeNodeId(11)),
        ]
        .into_iter()
        .collect()
    }

    fn name(kind: EntityKind) -> StableName {
        StableName {
            kind,
            node: RecipeNodeId(0),
            path: vec![
                RoleSeg::Cap(CapEnd::End),
                RoleSeg::FromA(NameRef::new(StableName {
                    kind: EntityKind::Edge,
                    node: RecipeNodeId(1),
                    path: vec![RoleSeg::Cap(CapEnd::Start)],
                })),
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
            let out =
                remap_name(&name(kind), &map(), &StepMap::new()).expect("the map covers both ids");
            assert_eq!(out.kind, kind, "remap_name must carry the kind through");
            assert_eq!(out.node, RecipeNodeId(10), "and renumber the mint");
            assert_eq!(
                FaceName::new(out).is_ok(),
                kind == EntityKind::Face,
                "so what a rewritten name denotes is decided by the INPUT kind alone"
            );
        }
    }

    /// The face rewrite agrees with it: same derivation, same kind,
    /// and no way to ask for a different one.
    #[test]
    fn a_face_remap_agrees_with_it_on_a_covered_map() {
        let face = FaceName::new(name(EntityKind::Face)).expect("a face");
        let out = remap_face(&face, &map(), &StepMap::new())
            .unwrap_or_else(|_| panic!("the map covers both ids"));
        assert_eq!(out.node, RecipeNodeId(10));
        assert_eq!(out.kind, EntityKind::Face);
        assert_eq!(
            *out,
            remap_name(&name(EntityKind::Face), &map(), &StepMap::new())
                .expect("the map covers both ids"),
            "the two rewrites are one rewrite"
        );
    }

    /// Its ONE miss is the id miss — never a panic and never a kind
    /// refusal.
    #[test]
    fn its_one_miss_is_the_id_the_map_lacks() {
        let face = FaceName::new(name(EntityKind::Face)).expect("a face");
        let empty = NodeMap::new();
        match remap_face(&face, &empty, &StepMap::new()) {
            Err(missing) => assert_eq!(
                missing,
                Unmapped::Node(RecipeNodeId(0)),
                "an empty map lacks the mint first, so that is the id reported"
            ),
            Ok(out) => panic!("an empty map covers no id, got {out}"),
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
    use super::{NodeMap, RemapMiss, StepMap, Unmapped, remap_face, remap_name, remap_node};
    use crate::names::{FaceName, NameRef, RoleSeg, StableName};
    use crate::node::{Node, RecipeNodeId, SitedRef};
    use crate::{CapEnd, EntityKind};

    const OUTER: RecipeNodeId = RecipeNodeId(0);
    const INNER: RecipeNodeId = RecipeNodeId(1);

    /// A map that carries the OUTER mint and NOT the nested one: the
    /// case where the name a refusal names and the node that failed
    /// come apart.
    fn map() -> NodeMap {
        [(OUTER, RecipeNodeId(10))].into_iter().collect()
    }

    /// A name whose own mint is [`OUTER`] and whose path embeds a name
    /// minted at [`INNER`].
    fn nested(kind: EntityKind) -> StableName {
        StableName {
            kind,
            node: OUTER,
            path: vec![
                RoleSeg::Cap(CapEnd::End),
                RoleSeg::FromA(NameRef::new(StableName {
                    kind: EntityKind::Edge,
                    node: INNER,
                    path: vec![RoleSeg::Cap(CapEnd::Start)],
                })),
            ],
        }
    }

    /// The premise: the rewrite itself reports the nested node, which
    /// is not the name's mint.
    #[test]
    fn the_rewrite_reports_the_nested_node() {
        let name = nested(EntityKind::Edge);
        let missing = remap_name(&name, &map(), &StepMap::new()).expect_err("INNER is unmapped");
        assert_eq!(missing, Unmapped::Node(INNER));
        assert_ne!(
            missing,
            Unmapped::Node(name.node),
            "the node that failed is not the name's own mint"
        );
    }

    /// And the face rewrite agrees, rather than collapsing the id into
    /// the face it was asked about.
    #[test]
    fn the_face_rewrite_reports_it_too() {
        let face = FaceName::new(nested(EntityKind::Face)).expect("a face");
        let missing = remap_face(&face, &map(), &StepMap::new()).expect_err("INNER is unmapped");
        assert_eq!(missing, Unmapped::Node(INNER));
        assert_ne!(missing, Unmapped::Node(face.node));
    }

    /// The payload rewrite pairs them: the name it could not rewrite
    /// AND the node it stopped at, which a caller raising a
    /// name-shaped refusal passes on.
    #[test]
    fn a_payload_miss_carries_both_the_name_and_the_node() {
        let name = nested(EntityKind::Edge);
        let node = Node::declare_rest(vec![(
            SitedRef::new(OUTER, name.clone()),
            SitedRef::at_mint(name.clone()),
        )]);
        match remap_node(&node, &map(), &StepMap::new(), &|g| Ok(g)) {
            Err(RemapMiss::Name {
                name: reported,
                missing,
            }) => {
                assert_eq!(*reported, name, "the refusal names the outer name");
                assert_eq!(
                    missing,
                    Unmapped::Node(INNER),
                    "and carries the node that actually failed"
                );
                assert_ne!(
                    missing,
                    Unmapped::Node(reported.node),
                    "which the outer name does not say: they are different nodes"
                );
            }
            Err(RemapMiss::Input(id)) => panic!("a name miss is not an input miss, got {id:?}"),
            Ok(out) => panic!("INNER is unmapped, got {out:?}"),
        }
    }
}

/// **A remap that reorders ids republishes the canonical form.**
///
/// The split's node map follows document order, which a loaded
/// document need not keep in id order, so two ids can come out in the
/// other order. Every name-ordered position then has to be put back in
/// order, and a crossing ranked along a union seam whose sides swap
/// reads its rank from the other end — the form the emitters would mint
/// for the same entity under the new ids.
#[cfg(test)]
#[allow(clippy::expect_used)]
mod a_remap_that_reorders_ids_republishes_the_canonical_form {
    use super::{NodeMap, StepMap, remap_name};
    use crate::names::{NameRef, Qualifier, RoleSeg, StableName};
    use crate::node::RecipeNodeId;
    use crate::{CapEnd, EntityKind};

    /// Union 9 → 20; members 1 and 2 swap order (→ 31, 30); 3 → 32.
    fn map() -> NodeMap {
        [(9, 20), (1, 31), (2, 30), (3, 32), (5, 25)]
            .into_iter()
            .map(|(a, b)| (RecipeNodeId(a), RecipeNodeId(b)))
            .collect()
    }

    fn cap(kind: EntityKind, node: u64) -> StableName {
        StableName {
            kind,
            node: RecipeNodeId(node),
            path: vec![RoleSeg::Cap(CapEnd::Start)],
        }
    }

    /// Member `m`'s start cap in union `u`'s space.
    fn member(kind: EntityKind, u: u64, m: u64) -> StableName {
        StableName {
            kind,
            node: RecipeNodeId(u),
            path: vec![RoleSeg::FromMember {
                member: RecipeNodeId(m),
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
            node: RecipeNodeId(node),
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
        let out = remap_name(&union_piece(9, (1, 2)), &map(), &StepMap::new()).expect("covered");
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
        let out = remap_name(&was, &map(), &StepMap::new()).expect("covered");
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
        let out = remap_name(&was, &map(), &StepMap::new()).expect("covered");
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
        let out = remap_name(&junction, &map(), &StepMap::new()).expect("covered");
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
        let out = remap_name(&sided, &map(), &StepMap::new()).expect("covered");
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
    use super::{NodeMap, StepMap, Unmapped, remap_name};
    use crate::EntityKind;
    use crate::names::{PieceRole, ProfileEdgeRef, RoleSeg, SectionCircle, StableName};
    use crate::node::{RecipeNodeId, StepId};

    fn wall(node: u64, e: ProfileEdgeRef) -> StableName {
        StableName {
            kind: EntityKind::Face,
            node: RecipeNodeId(node),
            path: vec![RoleSeg::Lateral(e)],
        }
    }

    fn piece(step: u64) -> ProfileEdgeRef {
        ProfileEdgeRef::Piece {
            step: StepId(step),
            role: PieceRole::Leg,
        }
    }

    #[test]
    fn a_piece_crosses_on_the_step_map_and_a_section_crosses_as_it_is() {
        let nodes: NodeMap = [(RecipeNodeId(2), RecipeNodeId(0))].into_iter().collect();
        let steps: StepMap = [(StepId(7), StepId(1))].into_iter().collect();
        assert_eq!(
            remap_name(&wall(2, piece(7)), &nodes, &steps).expect("covered"),
            wall(0, piece(1))
        );
        let section = ProfileEdgeRef::Section {
            circle: SectionCircle::Outer,
            role: PieceRole::Piece(0),
        };
        assert_eq!(
            remap_name(&wall(2, section), &nodes, &steps).expect("covered"),
            wall(0, section)
        );
        assert_eq!(
            remap_name(&wall(2, piece(8)), &nodes, &steps),
            Err(Unmapped::Step(StepId(8))),
            "a step no carried profile holds misses on that step"
        );
    }
}
