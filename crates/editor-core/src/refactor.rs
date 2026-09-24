//! **Split and inline** — the first-class recorded refactorings
//! (ASM-4 D-2/D-3; ASSEMBLY-DESIGN A4, A10, A11).
//!
//! [`split`] cuts a closed set of nodes out of a document into a new
//! part document and leaves an [`Node::InstantiatePart`] of it behind;
//! [`inline`] is the inverse — it splices a referenced document's
//! recipe into the host and deletes the instance. Both are PURE
//! functions returning new document values, the ordinary recorded
//! [`DocEdit`]s that produce them, and the [`crate::Maintenance`]
//! those edits performed — the cluster-record acts the mate graph's
//! motion forced, and the payload names a departing cut node
//! stranded. The input documents are untouched,
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
//! sink a document sink, i.e. an A10 root.) The cut must also be a
//! union of whole placement clusters — vacuously true in the mate-less
//! v1, where every instantiate node is its own singleton cluster
//! (A11), and re-checked for real when mates land.
//!
//! # Placements move with the cut (A11), one frame is hoisted
//!
//! The cut must be a union of WHOLE placement clusters — a torn
//! cluster refuses [`SplitError::TornCluster`], because A11 puts the
//! frame on the cluster and a torn one has one frame and two homes.
//! When the cut is EXACTLY one placement cluster — its instances and
//! the mates holding them together, nothing else (ASM-4 D-2 rider ii,
//! re-keyed by ASM-R2a now that mates make a cluster multi-node) — its
//! frame is HOISTED: the part document holds the copy
//! unplaced (identity) and the remainder's new instance is placed at
//! the cluster's old frame — D-2's "placed at the cluster's old
//! frame", and the shape a reusable part wants (its world pose belongs
//! to the assembly). Every other cut moves its recorded placements
//! into the part document VERBATIM and leaves the remainder instance
//! at identity — the cut material's world pose is baked into its own
//! recipe and placements, so identity IS its old frame. Both shapes
//! make the A4 acceptance identity exact: the identity placement
//! fast-path re-materializes the moved material bit-for-bit, and
//! [`crate::Frame::compose`]'s identity fast-paths give the inline
//! round trip back the original frames with zero arithmetic.
//!
//! The remainder receives ONE `InstantiatePart` for the whole cut
//! (the D-2 amendment, adjudicated at review ordinal 40): each
//! remainder instance materializes the ENTIRE new document's product,
//! so per-cluster instances of one pinned document would duplicate
//! every other cluster's material N times. The single instance carries
//! all cut clusters at their moved placements. Consequence (amendment
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
//! A mate EDGE whose two ends land on opposite sides of the cut would
//! become one [`crate::InterfaceCrossing::Mate`] entry in the
//! remainder instance's [`crate::InterfaceRecord`] (ASM-R2b D-4, the
//! hook ASM-4 left) — but **no accepted cut puts them there, so split
//! always mints the EMPTY record**. The cut rules make it unreachable:
//! an edge welds its two members into one placement cluster and
//! `TornCluster` refuses to tear one. The collector below carries the
//! argument in full; the door that would make a crossing reachable is
//! banked as ASM-XSPLIT. A mate that is not an edge — a dangling or
//! nested-pattern head — contributes nothing however its names fall,
//! and that one IS reachable: the gate is what skips it.
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
use crate::names::{FaceName, NameRef, RoleSeg, SegRewrite, StableName};
use crate::node::{InterfaceCrossing, InterfaceRecord, Node, PatternKind, RecipeNodeId};
use crate::part::{PartResolver, ResolveFailure};
use crate::persist::{PersistError, content_pin};
use crate::program::{ProfileDoc, ProfileProgram};
use crate::resolve::derivation_nodes;
use geom_core::Tol;

/// The old-id → new-id correspondence a refactoring establishes
/// between the two documents' node id spaces.
pub type NodeMap = BTreeMap<RecipeNodeId, RecipeNodeId>;

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
    /// A mate that WELDS a cluster meets `TornCluster` first, because
    /// the cut also splits the cluster its two members share. This
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
    /// The cut TEARS a placement cluster: some of the cluster's
    /// instances are cut and some are kept (ASM-R2a; review MAJOR-2).
    ///
    /// A11 puts the frame on the CLUSTER, so a torn cluster has one
    /// frame and two homes; splitting it would have to invent which
    /// side keeps it and re-mint the other from a relative pose that
    /// now crosses a document seam — machinery no ratified rule
    /// supplies. The cut must be a union of WHOLE clusters, which is
    /// what this module's docs have promised since ASM-4 and what
    /// mates made checkable. Refused naming the cluster and the
    /// instance on the far side of the tear; the repair is to widen
    /// the cut to the whole cluster, or to delete the mates that hold
    /// it together first.
    TornCluster {
        /// The cluster's gauge (its document-order-first instance).
        gauge: RecipeNodeId,
        /// The first member, in document order, on the opposite side
        /// of the cut from the gauge.
        instance: RecipeNodeId,
        /// Whether the GAUGE is the cut-side endpoint.
        gauge_is_cut: bool,
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
}

impl core::fmt::Display for SplitError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::EmptyCut => f.write_str("split: the cut set is empty"),
            Self::UnknownCutNode { id } => {
                write!(f, "split: cut entry {} is not a live node", id.0)
            }
            Self::TornCluster {
                gauge,
                instance,
                gauge_is_cut,
            } => {
                let (cut, kept) = if *gauge_is_cut {
                    (gauge.0, instance.0)
                } else {
                    (instance.0, gauge.0)
                };
                write!(
                    f,
                    "split: the cut tears the placement cluster gauged at node {} (node {cut} is \
                     cut, node {kept} is kept) — the frame lives on the CLUSTER, so the cut \
                     must be a union of WHOLE clusters; widen the cut, or delete the mates \
                     holding the cluster together first",
                    gauge.0
                )
            }
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
            Self::PartEdit { error } => {
                write!(f, "split: a part-side edit refused: {error}")
            }
            Self::RemainderEdit { error } => {
                write!(f, "split: a remainder-side edit refused: {error}")
            }
        }
    }
}

impl core::error::Error for SplitError {}

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
    /// The instance is placed at a non-identity frame and the
    /// referenced document has a root that is not itself an instance:
    /// plain recipe geometry has no placement of its own, so the
    /// instance's frame is not something the spliced recipe can
    /// express locally (D-3's "refuses typed when the referenced
    /// product is not what the recipe can express" — the one case v1
    /// found).
    UnplaceableFrame {
        /// The plain-geometry root.
        root: RecipeNodeId,
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
                "inline: the instance is placed at a non-identity frame, but part root {} is \
                 plain recipe geometry with no placement of its own — the frame is not \
                 expressible locally",
                root.0
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
            Self::StrandedPartName { name, missing } => write!(
                f,
                "inline: the {name} derives from node {}, which the referenced document no \
                 longer has — repair the stranded reference before inlining",
                missing.0
            ),
            Self::Edit { error } => write!(f, "inline: an edit refused: {error}"),
        }
    }
}

impl core::error::Error for InlineError {}

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
    /// The maintenance `remainder_edits` performed, in edit order
    /// ([`Maintenance`]): what the A11 registry did as the cut's names
    /// re-anchored onto the instance (a kept mate that welded nothing
    /// while its far end was a local body welds the instance to its
    /// near end once the name is instance-qualified — a join) and as
    /// the cut nodes left (a cut cluster's mates and members going is
    /// its splits and drops), and every payload name a departing cut
    /// node stranded behind it (DM7). An accepted edit travels whole, so the
    /// outcome carries what its edits DID beside what they produced: a
    /// caller holding a document with the maintenance of its last
    /// accepted edit swaps `remainder` and this in together.
    pub remainder_maintenance: Vec<Maintenance>,
    /// The recorded edits producing `part` from
    /// `Doc::empty(part_id)`.
    pub part_edits: Vec<DocEdit<ProfileProgram>>,
    /// The maintenance `part_edits` performed, in edit order
    /// ([`Maintenance`]). The part is built by inserting the cut nodes, and a cut
    /// mate welds its two members as it lands, so a multi-member
    /// cluster cut whole re-forms in the part as one join per mate
    /// that welded two clusters still separate when it landed. That
    /// insert is the one part-side edit that moves a mate graph: the
    /// tolerance, parameter, witness, placement and root edits
    /// reconcile nothing.
    pub part_maintenance: Vec<Maintenance>,
    /// The remainder's new instantiate node.
    pub instance: RecipeNodeId,
    /// Cut-node ids → their part-document ids (minted in document
    /// order — the D9-deterministic remap).
    pub node_map: NodeMap,
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
    /// The maintenance `edits` performed, in edit order
    /// ([`Maintenance`]): the part's mates weld their spliced members
    /// as they land, a wrapped name's re-anchoring moves what the
    /// instance
    /// welded onto the spliced node (a split, where the spliced node
    /// is no member), and the instance's delete drops or re-keys its
    /// cluster's row. An accepted edit travels whole; a caller holding
    /// a document with the maintenance of its last accepted edit swaps
    /// `doc` and this in together.
    pub maintenance: Vec<Maintenance>,
    /// Part-document node ids → their host ids (minted in the part's
    /// document order).
    pub node_map: NodeMap,
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
/// rule). Set-valued segments and `SideOf` verdict vectors are
/// re-canonicalized after the rewrite (their canonical order is name
/// order, which the rewrite may change).
///
/// # Errors
///
/// The first local id the map lacks.
fn remap_name(name: &StableName, map: &NodeMap) -> Result<StableName, RecipeNodeId> {
    let (node, path) = remap_derivation(name.node, &name.path, map)?;
    Ok(StableName {
        kind: name.kind,
        node,
        path,
    })
}

/// The half of [`remap_name`] that a name's KIND is not part of: the
/// minting node and the role path, rewritten through `map`.
///
/// Split out because that is exactly what a face name may have
/// rewritten — `FaceName::map_derivation` hands this function the
/// derivation and keeps the kind itself, which is what makes
/// [`remap_face`] total without an arm for a kind change.
///
/// # Errors
///
/// The first local id the map lacks.
fn remap_derivation(
    node: RecipeNodeId,
    path: &[RoleSeg],
    map: &NodeMap,
) -> Result<(RecipeNodeId, crate::names::RolePath), RecipeNodeId> {
    let node = *map.get(&node).ok_or(node)?;
    let path = path
        .iter()
        .cloned()
        .map(|seg| seg.rewrite(&mut Remapping(map)))
        .collect::<Result<_, _>>()?;
    Ok((node, path))
}

/// **The split re-map as a [`SegRewrite`]**: every carried name is
/// rewritten through [`remap_name`] — its minting node through the
/// map, then its own path through this same rewriter, so the descent
/// is [`remap_name`]'s and not the walk's — and a member edge, which
/// is a local node id like the minting one, is mapped too: a subgraph
/// copied into another document would otherwise carry a member
/// reference that names a node HERE. A profile locator keeps the
/// trait's identity: nothing about a split moves a segment index. The
/// walk over [`RoleSeg`]'s shape is [`RoleSeg::rewrite`]'s, shared
/// with the anchor rewrite and the whole-program edit.
struct Remapping<'a>(&'a NodeMap);

impl SegRewrite for Remapping<'_> {
    type Error = RecipeNodeId;

    fn name(&mut self, n: &StableName) -> Result<Option<StableName>, Self::Error> {
        remap_name(n, self.0).map(Some)
    }

    fn member(&mut self, m: RecipeNodeId) -> Result<RecipeNodeId, Self::Error> {
        self.0.get(&m).copied().ok_or(m)
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
fn remap_face(name: &FaceName, map: &NodeMap) -> Result<FaceName, RecipeNodeId> {
    name.map_derivation(|node, path| remap_derivation(node, path, map))
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
        /// The local node the map lacks.
        missing: RecipeNodeId,
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
/// name-reference payloads — through `map`, for insertion into the
/// other document. `InstantiatePart` crosses verbatim: its reference
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
) -> Result<Node<ProfileProgram>, RemapMiss> {
    let id = |n: RecipeNodeId| -> Result<RecipeNodeId, RemapMiss> {
        map.get(&n).copied().ok_or(RemapMiss::Input(n))
    };
    let nm = |n: &StableName| {
        remap_name(n, map).map_err(|missing| RemapMiss::Name {
            name: Box::new(n.clone()),
            missing,
        })
    };
    // A mate head across the cut, through the one face remap
    // (`remap_face`): its derivation is rewritten and its kind is the
    // type's, so the only miss is the miss `nm` reports for a bare
    // name.
    let face = |n: &FaceName| {
        remap_face(n, map).map_err(|missing| RemapMiss::Name {
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
        Node::Profile(p) => Node::Profile(ProfileProgram {
            plane: id(p.plane)?,
            loops: p.loops.clone(),
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
        Node::Transform {
            input,
            translation,
            rotation_axis,
            rotation_angle,
        } => Node::Transform {
            input: id(*input)?,
            translation: translation.clone(),
            rotation_axis: rotation_axis.clone(),
            rotation_angle: rotation_angle.clone(),
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
        Node::InstantiatePart { .. } => node.clone(),
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
            alignment: *alignment,
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
    // A11's cluster precondition, checked FOR REAL now that mates can
    // make a cluster multi-node (this module's docs have promised the
    // re-check since ASM-4; review MAJOR-2 found it missing). Run
    // beside the severed-edge check, before anything moves: a torn
    // cluster is a refusal, not a case the hoist below silently
    // declines to handle.
    for members in crate::mate::clusters(doc) {
        let Some(&gauge) = members.first() else {
            continue;
        };
        let gauge_is_cut = cut.contains(&gauge);
        if let Some(&instance) = members.iter().find(|id| cut.contains(id) != gauge_is_cut) {
            return Err(SplitError::TornCluster {
                gauge,
                instance,
                gauge_is_cut,
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
    // AFTER the cluster precondition on purpose: a mate that WELDS
    // its two members is the case `TornCluster` already speaks to,
    // and it is the more informative refusal — it names the cluster
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
    // The hoisted-frame case: the cut is exactly one placement CLUSTER
    // (ASM-4's D-2 amendment, rider ii — re-keyed here now that A12's
    // mates make a cluster multi-node; the pre-mate reading, "exactly
    // one instantiate node", is the singleton case of this one).
    //
    // Two conditions, and each says something the frame move needs:
    // the cut's instances all belong to ONE cluster (else there is no
    // single frame to hoist), and the cut carries nothing but that
    // cluster and the mates holding it together (else the part
    // document owns material the hoisted frame does not place).
    //
    // WHOLENESS is not a third condition here — the torn-cluster
    // precondition above already refused every partial cluster, so a
    // cluster reached by the cut is entirely inside it. That is the
    // load-bearing difference from the shape this predicate had before
    // the review: it used to FILTER torn clusters out of the count,
    // which made a torn cluster look like an absent one and let the
    // hoist proceed while the torn frame was dropped (MAJOR-2).
    let hoisted = {
        let cut_instances: Vec<RecipeNodeId> = cut
            .iter()
            .copied()
            .filter(|&id| matches!(doc.node(id), Some(Node::InstantiatePart { .. })))
            .collect();
        let gauges: BTreeSet<RecipeNodeId> = crate::mate::clusters(doc)
            .into_iter()
            .filter(|members| members.iter().any(|m| cut_instances.contains(m)))
            .filter_map(|members| members.first().copied())
            .collect();
        let only_cluster_and_its_mates = cut.iter().all(|&id| {
            cut_instances.contains(&id) || matches!(doc.node(id), Some(Node::Mate { .. }))
        });
        match gauges.iter().next() {
            Some(&gauge) if gauges.len() == 1 && only_cluster_and_its_mates => Some(gauge),
            _ => None,
        }
    };
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

    // ---- The part document, as recorded edits from empty ----
    // The part side's edits are inserts into a document being built —
    // a Join at most, never a moved gauge — so they lever through the
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
    for &old in doc.order().iter().filter(|id| cut.contains(id)) {
        let Some(node) = doc.node(old) else { continue };
        let node = remap_node(node, &node_map).map_err(|miss| match miss {
            RemapMiss::Input(input) => SplitError::PartEdit {
                error: Box::new(EditError::UnresolvedInput { input }),
            },
            RemapMiss::Name { name, missing } => SplitError::PartNameReachesRemainder {
                node: old,
                name,
                missing,
            },
        })?;
        part_apply(&mut part, DocEdit::InsertNode { node })?;
    }
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
    if hoisted.is_none() {
        // A11: the cut clusters' placements move verbatim (module
        // docs) — every recorded row whose instance is cut, explicit
        // identities included (semantics unchanged; the row's
        // explicitness is not).
        for (&old, frame) in doc.placements() {
            if cut.contains(&old)
                && let Some(&new) = node_map.get(&old)
            {
                part_apply(
                    &mut part,
                    DocEdit::SetPlacement {
                        node: new,
                        frame: *frame,
                    },
                )?;
            }
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
    // edges and A11's clusters ask ONE predicate.
    //
    // # Why no accepted cut reaches this record
    //
    // The record is unreachable, and predicate identity alone does not
    // establish that — the loop below tests NAMES
    // (`derivation_nodes ⊆ cut`) while the cluster precondition tests
    // INSTANCES. Three facts carry the argument, and the second is the
    // one that ties those two readings together:
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
    //    the same side, which is what makes (1)'s name reading agree
    //    with the cluster precondition's instance reading. For a plain
    //    head the two are the same node and this is trivial.
    // 3. An edge's two members are welded into one placement cluster
    //    (`mate::clusters`, on this same predicate), and `TornCluster`
    //    above refuses any cut that is not a union of WHOLE clusters.
    //
    // Together: an edge's two ends are never on opposite sides of an
    // accepted cut, so this loop mints nothing for one and the record
    // is ALWAYS empty. Remove any one of the three and the argument
    // fails. The conversion door that would make a crossing reachable
    // is banked as ASM-XSPLIT. Exhausted over every subset of two
    // recipes in `rev_fix_xsplit_unreachable.rs`.
    //
    // A mate with a DANGLING reference — one resolving to no member at
    // all — is not an edge and contributes NO crossing, however its
    // names fall across the cut. Such a mate never solved, so a record
    // minted from it would be trusted-at-rest state. Unlike (1)-(3),
    // this arm is NOT forced by the cut rules: a nested-pattern head
    // welds no cluster, so its mate's ends do reach opposite sides of
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
        let inner =
            remap_face(inner, &node_map).map_err(|missing| SplitError::NameStraddlesCut {
                name: Box::new((**inner).clone()),
                missing: Some(missing),
            })?;
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
    // **The remainder's reach knows the part this split is minting.**
    // Rebinding a mate's heads onto the new instance one name at a time
    // passes through documents where that mate stands on the new part,
    // and the maintenance that re-keys the clusters those rebinds
    // split solves through the mate's lever — the new part's own
    // extent, which no store holds yet because this call is what
    // creates it. The reach is therefore composed here: the part in
    // hand answers its own reference, the caller's resolver answers
    // every other, and an absent resolver refuses those typed.
    let carving: std::sync::Arc<dyn PartResolver> = std::sync::Arc::new(WithPart {
        doc_ref: DocRef { id: part_id, pin },
        part: part.doc.clone(),
        inner: resolver.cloned(),
    });
    let rem_reach = crate::eval::PartReach::<f64>::with_resolver(Some(&carving), tol);
    let rem_apply = |remainder: &mut Recording,
                     edit: DocEdit<ProfileProgram>|
     -> Result<Option<RecipeNodeId>, SplitError> {
        remainder
            .apply(edit, tol, &rem_reach)
            .map_err(|error| SplitError::RemainderEdit {
                error: Box::new(error),
            })
    };
    let minted = rem_apply(
        &mut remainder,
        DocEdit::InsertNode {
            node: Node::instantiate_part_with(
                DocRef { id: part_id, pin },
                InterfaceRecord { crossings },
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
        let of = remap_name(from, &node_map).map_err(|missing| SplitError::NameStraddlesCut {
            name: Box::new(from.clone()),
            missing: Some(missing),
        })?;
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
    if let Some(old_instance) = hoisted {
        let frame = doc.placement(old_instance);
        if !frame.is_identity_bits() {
            rem_apply(
                &mut remainder,
                DocEdit::SetPlacement {
                    node: instance,
                    frame,
                },
            )?;
        }
    }
    // A10 on the remainder: the instance takes the FIRST cut root's
    // list position (the cut material's product order collapses onto
    // the instance); A10's automatic root-list bookkeeping appended it
    // instead — the list's own move, not one of the A11 cluster-record
    // acts the outcome's `maintenance` fields hold.
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
    // Deleting the instance moves its cluster's gauge, and the
    // maintenance that re-keys the cluster levers through the parts
    // the same resolver holds.
    let reach = crate::eval::PartReach::<f64>::with_resolver(Some(resolver), tol);
    let Some(node) = doc.node(instance) else {
        return Err(InlineError::UnknownNode { id: instance });
    };
    let Node::InstantiatePart {
        doc_ref, interface, ..
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
    let frame = doc.placement(instance);
    if !frame.is_identity_bits() {
        for &root in part.roots() {
            if !matches!(part.node(root), Some(Node::InstantiatePart { .. })) {
                return Err(InlineError::UnplaceableFrame { root });
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
    for &old in part.order() {
        let Some(node) = part.node(old) else { continue };
        let node = remap_node(node, &node_map).map_err(|miss| match miss {
            RemapMiss::Input(input) => InlineError::Edit {
                error: Box::new(EditError::UnresolvedInput { input }),
            },
            RemapMiss::Name { name, missing } => InlineError::StrandedPartName { name, missing },
        })?;
        step(&mut current, DocEdit::InsertNode { node })?;
    }
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
    // D-3's placement rule: the instance's cluster frame composes onto
    // the part's own placements. Every spliced instance is placed at
    // the composition (identity compositions stay unrecorded — a
    // missing row IS the identity).
    for &old in part.order() {
        if !matches!(part.node(old), Some(Node::InstantiatePart { .. })) {
            continue;
        }
        let composed = frame.compose(&part.placement(old));
        if !composed.is_identity_bits()
            && let Some(&new) = node_map.get(&old)
        {
            step(
                &mut current,
                DocEdit::SetPlacement {
                    node: new,
                    frame: composed,
                },
            )?;
        }
    }
    // The part's appearance records land on the spliced local names.
    // A collision with a host record is the Rebind door's own typed
    // refusal below — never an auto-pick.
    for (name, record) in part.appearance().iter() {
        let key = remap_name(name, &node_map).map_err(|missing| InlineError::StrandedPartName {
            name: Box::new(name.clone()),
            missing,
        })?;
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
        let to = remap_name(of, &node_map).map_err(|missing| InlineError::StrandedPartName {
            name: Box::new((**of).clone()),
            missing,
        })?;
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
        remap_face(inner, &node_map).map_err(|missing| InlineError::StrandedPartName {
            name: Box::new((**inner).clone()),
            missing,
        })?;
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
    })
}

/// **The split's own resolver**: the part being minted, answered from
/// the document in hand, and every other reference answered by the
/// caller's resolver — or refused typed when there is none.
#[derive(Debug)]
struct WithPart {
    /// The new part's reference: its id and the pin of the document
    /// this split built.
    doc_ref: DocRef,
    /// That document.
    part: ProfileDoc,
    /// The caller's seam, for every other part.
    inner: Option<std::sync::Arc<dyn PartResolver>>,
}

impl PartResolver for WithPart {
    fn resolve(&self, doc_ref: &DocRef, tol: Tol) -> Result<ProfileDoc, ResolveFailure> {
        if doc_ref.id == self.doc_ref.id {
            if doc_ref.pin != self.doc_ref.pin {
                return Err(ResolveFailure::pin_mismatch(
                    "the reference names another version of the part this split is minting",
                ));
            }
            return Ok(self.part.clone());
        }
        match &self.inner {
            Some(inner) => inner.resolve(doc_ref, tol),
            None => Err(ResolveFailure::unresolved(
                "the split was given no resolver, and the reference is not the part it is \
                 minting",
            )),
        }
    }
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
    use super::{NodeMap, remap_face, remap_name};
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
            let out = remap_name(&name(kind), &map()).expect("the map covers both ids");
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
        let out = remap_face(&face, &map()).unwrap_or_else(|_| panic!("the map covers both ids"));
        assert_eq!(out.node, RecipeNodeId(10));
        assert_eq!(out.kind, EntityKind::Face);
        assert_eq!(
            *out,
            remap_name(&name(EntityKind::Face), &map()).expect("the map covers both ids"),
            "the two rewrites are one rewrite"
        );
    }

    /// Its ONE miss is the id miss — never a panic and never a kind
    /// refusal.
    #[test]
    fn its_one_miss_is_the_id_the_map_lacks() {
        let face = FaceName::new(name(EntityKind::Face)).expect("a face");
        let empty = NodeMap::new();
        match remap_face(&face, &empty) {
            Err(missing) => assert_eq!(
                missing,
                RecipeNodeId(0),
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
    use super::{NodeMap, RemapMiss, remap_face, remap_name, remap_node};
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
        let missing = remap_name(&name, &map()).expect_err("INNER is unmapped");
        assert_eq!(missing, INNER);
        assert_ne!(
            missing, name.node,
            "the node that failed is not the name's own mint"
        );
    }

    /// And the face rewrite agrees, rather than collapsing the id into
    /// the face it was asked about.
    #[test]
    fn the_face_rewrite_reports_it_too() {
        let face = FaceName::new(nested(EntityKind::Face)).expect("a face");
        let missing = remap_face(&face, &map()).expect_err("INNER is unmapped");
        assert_eq!(missing, INNER);
        assert_ne!(missing, face.node);
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
        match remap_node(&node, &map()) {
            Err(RemapMiss::Name {
                name: reported,
                missing,
            }) => {
                assert_eq!(*reported, name, "the refusal names the outer name");
                assert_eq!(missing, INNER, "and carries the node that actually failed");
                assert_ne!(
                    missing, reported.node,
                    "which the outer name does not say: they are different nodes"
                );
            }
            Err(RemapMiss::Input(id)) => panic!("a name miss is not an input miss, got {id:?}"),
            Ok(out) => panic!("INNER is unmapped, got {out:?}"),
        }
    }
}
