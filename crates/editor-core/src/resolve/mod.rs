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
//! one. That diff can be silent about the name: the diff engine's
//! population-cancel blind spot (`vdiff` module docs), and **sweep
//! pruning** (ratified 2026-07-29, N5 as amended) — the realized
//! boolean sweep records no verdicts for pairs its candidate
//! generation pruned, so a vanish whose flip evidence lived on a
//! now-pruned pair has no recorded flip to cite.
//!
//! **After the path's flip and doc-diff lanes come up empty**, four
//! honest rungs remain, in order: `Cascade` when an embedded operand
//! name itself fails to resolve; the QUALIFIER-DELTA rung, which for a
//! face piece is the BORDER delta ([`border_delta`]): the divider walls
//! a vanished piece's name records against those its parent's nearest
//! piece borders now, read off the names themselves; with a prior run,
//! the same lanes UPSTREAM of the minting node ([`Diagnosis::Upstream`];
//! the scope rule is at [`upstream_nodes`]); and, with a prior run, the
//! GROUP-SIZE rung ([`group_resized`], whose docs say why a fragment
//! name can vanish with no flip at all) answering
//! [`Diagnosis::GroupResized`]. If that too finds nothing, the total
//! fallback is [`Diagnosis::cause_not_in_evidence`], which carries
//! that reading at the value rather than in prose here.

mod hit;
mod pick;
mod vdiff;

pub use hit::{
    HitTestError, UnnamedEntity, body_name, edge_name, entity_name, face_name, vertex_name,
};
pub use pick::{
    Answer, Crossing, FaceAnswer, MeshPick, MeshPickError, NameLookupError, NodePick,
    NodePickError, PickHit, PickMemo, PickTarget, TSpan, answer_of, crossing, pick_face,
    ray_triangle,
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
use crate::eval::{Evaluation, NodeStanding};
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
            edit: if doc.has_minted(name.node) {
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
        width: usize,
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

/// Why a name vanished.
///
/// N5's arms, including [`Self::GroupResized`] and the border delta
/// ([`Self::BorderDelta`]), plus two additions that are NOT N5's and
/// are marked as such wherever they are read: the reserved
/// `WitnessBifurcation` arm (SOLVER-DESIGN W3, constructed by the M6
/// solver) and [`Self::ConsumedByFold`]. A consumer matching this enum
/// is matching more than N5 wrote.
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
    /// A face piece vanished and its parent is held, at the same node,
    /// as pieces bordering other divider walls ([`border_delta`]): the
    /// walls the vanished piece's name records against those of the
    /// piece nearest it now, read off the names.
    BorderDelta {
        /// The vanished name's minting node.
        node: RecipeNodeId,
        /// The walls the vanished piece bordered and that piece does
        /// not.
        gone: Vec<StableName>,
        /// The walls that piece borders and the vanished piece did not.
        new: Vec<StableName>,
    },
    /// At the vanished name's minting node, the group its emitter formed
    /// from the fragment's parent held `was` entities in the last-good
    /// run and holds `now` in this one, and no rung that names a cause
    /// found one ([`group_resized`], which is where the reading and its
    /// limits are stated).
    ///
    /// A statement about the groups the two runs recorded and the seams
    /// the two tables spell on the group's parent, nothing more: it
    /// states how many entities descend from the parent, and which
    /// seams on it only one run spells, not why either changed.
    GroupResized {
        /// The vanished name's minting node, whose two tables were
        /// counted.
        node: RecipeNodeId,
        /// The group's entity count in the last-good run.
        was: u32,
        /// The group's entity count now; never equal to `was`.
        now: u32,
        /// The seams on the group's parent only one of the two tables
        /// spells, by cutter, or why the tables cannot say
        /// ([`GroupCutters`]).
        cutters: GroupCutters,
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
    /// Evidence UPSTREAM of the vanished name's minting node that is
    /// NOT on its derivation path — a recorded flip, a
    /// structural-parameter change or a recipe edit at a node that fed
    /// the name without deciding it. A candidate cause, stated no more
    /// strongly than that; the scope rule, and what "upstream" means
    /// across the two runs, is stated once at [`upstream_nodes`].
    Upstream {
        /// The vanished name's minting node, which `cause` is upstream
        /// of.
        node: RecipeNodeId,
        /// What was found there.
        cause: UpstreamCause,
    },
    /// An n-ary union's fold consumed the entity a member-space name
    /// denotes, by a composition that leaves no one entity for the
    /// name to denote — NOT N5's: the arm the rule *"a composition that
    /// breaks one name denotes one entity refuses"* adds for the
    /// compositions other than a merge. A merge's own case is looked
    /// through, not refused (`Node::Union`).
    ///
    /// Read off the accumulation's rows at the step the name is fed
    /// to, never by re-measuring the face: which composition consumed
    /// it is the SHAPE of the rows that descend from it
    /// ([`FoldConsumption`]). A refusal carrying this diagnosis offers
    /// no replacement, because none is unique: a split and a
    /// fragmented merge leave several candidates.
    ///
    /// The union is not a field: the name this diagnoses is a
    /// member-space name, minted by that union, so it is the name's own
    /// minting node, and the refusal carrying the name already says so.
    ConsumedByFold {
        /// How the fold consumed it.
        by: FoldConsumption,
    },
}

/// Which composition of a union's fold consumed a member's entity
/// ([`Diagnosis::ConsumedByFold`]) — the structural shape of what the
/// accumulation holds in its place.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FoldConsumption {
    /// A later member SPLIT it: the accumulation holds fragments of
    /// it (the name with `Fragment` qualifiers after it), bare or as
    /// constituents of later merges, and never the name itself as a
    /// face. The step that split it may also have merged part of it:
    /// the name is then a constituent of a bare merged row beside its
    /// own fragment, and that row holds only part of the face.
    Split,
    /// A declared MERGE consumed it and a later member split the
    /// merged face: the accumulation holds fragments of a merged row
    /// whose constituent set covers the name, and no bare merged row
    /// that does.
    FragmentedMerge,
}

// The composition as the clause of [`Diagnosis::ConsumedByFold`]'s
// sentence that says what happened to the entity and why nothing is
// offered in its place.
impl core::fmt::Display for FoldConsumption {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(match self {
            Self::Split => {
                "a later member split it into fragments, some perhaps merged with \
                 another member's face in the same step, and which piece the \
                 reference means is not decidable from the names, so none is offered"
            }
            Self::FragmentedMerge => {
                "a declared merge consumed it and a later member then split the merged \
                 face, and which fragment the reference means is not decidable from the \
                 names, so none is offered"
            }
        })
    }
}

/// The seams on a resized group's parent that only one of the two runs
/// spells, each named by the cutter on its other side, as
/// [`Diagnosis::GroupResized`] reads them off the minting node's two
/// name tables ([`group_cutters`] states the reading and its limits).
///
/// A CUTTER is the operand entity on the other side of a `Seam` row
/// spelled on the group's parent: the seam EDGES a face group's parent
/// meets, the seam VERTICES an edge group's parent meets. Matched on the
/// `Seam { a, b }` pair, never on the row, so a seam that is ranked or
/// tied in one run and single in the other is the same cutter in both.
///
/// A seam is a crossing, not a division: a contact that only trims or
/// notches the parent spells one as well as a contact that divides it,
/// so a cutter listed here is one whose seam with the parent one run
/// spells and the other does not — which says nothing about whether
/// that seam was one of the group's boundaries.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GroupCutters {
    /// Both tables were read in full. `gone` are the cutters whose seam
    /// with the parent the last-good table spells and the current one
    /// does not; `new` the reverse. Each is sorted and holds every such
    /// cutter, so a list of two names two seams' cutters, and both lists
    /// empty says the two tables spell seams on the parent with the same
    /// cutters.
    ///
    /// A cutter is compared by NAME. One an upstream edit RENAMED — the
    /// same operand entity re-qualified — reads as one gone and one new;
    /// this is a statement about spelled seams, not a claim that an
    /// entity was withdrawn.
    Read {
        /// Cutters whose seam with the parent the last-good table spells
        /// and the current one does not.
        gone: Vec<StableName>,
        /// Cutters whose seam with the parent the current table spells
        /// and the last-good one did not.
        new: Vec<StableName>,
    },
    /// The group's emitter does not bound it with `Seam` rows on one
    /// parent name: a split's (face, side) group, whose one cutter is
    /// the tool; a pair boolean's own seam chain or merged face; any
    /// vertex group.
    NotSeamBounded,
    /// Tied parents (N2) share the name the seams are spelled on, in one
    /// run or both, so a seam on that name does not say which parent's
    /// group it bounds.
    TiedParents,
    /// The last-good table spells no seam on the parent, so the tables
    /// hold no cutter to compare against, and an empty comparison would
    /// read as "the same cutters".
    NoSeamOnRecord,
    /// A row of either table spells a seam on the parent in a shape the
    /// reading does not follow (anything but one `Seam` with only
    /// `Fragment`s after it), so a comparison would be over part of the
    /// seams and could name a cutter gone that is merely unread.
    SeamUnread,
}

/// One cutter as [`GroupCutters`]' sentence names it: its name, and the
/// role it has in the entity it denotes, in words — so two walls of one
/// extrude read as two walls rather than as one name twice.
struct Cutter<'a>(&'a StableName);

impl core::fmt::Display for Cutter<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let leaf = descent_leaf(self.0);
        write!(f, "the {} (", self.0)?;
        match leaf.path.first() {
            Some(seg) => role_words(f, seg)?,
            None => write!(f, "no role")?,
        }
        if leaf.node != self.0.node {
            write!(f, ", minted by node {}", leaf.node.0)?;
        }
        write!(f, ")")
    }
}

/// The name a descent chain ends at: `name` with every `FromA`,
/// `FromB`, `FromMember` and `FromTarget` wrapper looked through and
/// every trailing `Fragment` dropped — the entity whose role the
/// cutter's sentence spells.
fn descent_leaf(name: &StableName) -> &StableName {
    let mut n = name;
    loop {
        let head = &n.path[..fragment_tail_start(&n.path)];
        n = match head {
            [RoleSeg::FromA(i) | RoleSeg::FromB(i) | RoleSeg::FromTarget(i)]
            | [RoleSeg::FromMember { of: i, .. }] => i,
            _ => return n,
        };
    }
}

/// A profile piece in words: its role (the role's own `Display`) and
/// the step that drew it, by the id it was minted with — spelled as an
/// id (`#7`), never as a position, since a name holds no position — or,
/// on a kernel-built section, which circle.
fn piece_words(e: &crate::names::ProfileEdgeRef) -> String {
    use crate::names::{ProfileEdgeRef, SectionCircle};
    match e {
        ProfileEdgeRef::Piece { step, role } => {
            format!("the {role} of the profile step minted #{}", step.0)
        }
        ProfileEdgeRef::Section { circle, role } => format!(
            "the {role} of the {} circle",
            match circle {
                SectionCircle::Outer => "outer",
                SectionCircle::Bore => "bore",
            }
        ),
    }
}

/// A role segment in words. Exhaustive, so a new segment is given words
/// here or the compile breaks.
fn role_words(f: &mut core::fmt::Formatter<'_>, seg: &RoleSeg) -> core::fmt::Result {
    use crate::names::{CapEnd, MeridianEnd};
    let cap = |e: &CapEnd| match e {
        CapEnd::Start => "start",
        CapEnd::End => "end",
    };
    let meridian = |e: &MeridianEnd| match e {
        MeridianEnd::Start => "start",
        MeridianEnd::End => "end",
        MeridianEnd::Seam => "seam",
        MeridianEnd::Pi => "half-turn",
    };
    let seg_of = |e: &crate::names::ProfileEdgeRef| piece_words(e);
    let vert_of = |v: &crate::names::ProfileVertexRef| {
        use crate::names::{ProfileEdgeRef, ProfileVertexRef};
        let piece = match *v {
            ProfileVertexRef::Piece { step, role } => ProfileEdgeRef::Piece { step, role },
            ProfileVertexRef::Section { circle, role } => ProfileEdgeRef::Section { circle, role },
        };
        format!("the start of {}", piece_words(&piece))
    };
    match seg {
        RoleSeg::OutputBody => write!(f, "the output body"),
        RoleSeg::Cap(e) => write!(f, "the {} cap", cap(e)),
        RoleSeg::Lateral(e) => write!(f, "the side wall over {}", seg_of(e)),
        RoleSeg::RimEdge(c, e) => write!(f, "the {} rim edge over {}", cap(c), seg_of(e)),
        RoleSeg::LateralEdge(v) => write!(f, "the lateral edge over {}", vert_of(v)),
        RoleSeg::CapVertex(c, v) => write!(f, "the {} cap vertex over {}", cap(c), vert_of(v)),
        RoleSeg::LoftWall(pieces) => write!(
            f,
            "the loft wall over {}",
            pieces
                .iter()
                .map(seg_of)
                .collect::<Vec<_>>()
                .join(", then ")
        ),
        RoleSeg::LoftSeam(vertices) => write!(
            f,
            "the loft seam over {}",
            vertices
                .iter()
                .map(vert_of)
                .collect::<Vec<_>>()
                .join(", then ")
        ),
        RoleSeg::Band(e) => write!(f, "the band face over {}", seg_of(e)),
        RoleSeg::BandRim(v) => write!(f, "the band rim over {}", vert_of(v)),
        RoleSeg::BandRimPi(v) => write!(f, "the second band rim over {}", vert_of(v)),
        RoleSeg::BandPi(e) => write!(f, "the second band face over {}", seg_of(e)),
        RoleSeg::Meridian(m, e) => {
            write!(f, "the {} meridian edge over {}", meridian(m), seg_of(e))
        }
        RoleSeg::MeridianVertex(m, v) => {
            write!(f, "the {} meridian vertex over {}", meridian(m), vert_of(v))
        }
        RoleSeg::RevolveCap(m) => write!(f, "the {} wedge cap", meridian(m)),
        RoleSeg::Pole(v) => write!(f, "the pole over {}", vert_of(v)),
        RoleSeg::AxisEdge(e) => write!(f, "the axis edge over {}", seg_of(e)),
        RoleSeg::FromA(_) | RoleSeg::FromB(_) | RoleSeg::FromTarget(_) => {
            write!(f, "a carried entity")
        }
        RoleSeg::FromMember { .. } => write!(f, "a union member's entity"),
        RoleSeg::Seam { .. } => write!(f, "a boolean seam"),
        RoleSeg::Merged(_) => write!(f, "a merged face"),
        RoleSeg::Fragment(_) => write!(f, "a fragment"),
        RoleSeg::SplitBody(_) => write!(f, "a split body"),
        RoleSeg::SectionFace { .. } => write!(f, "a split's section face"),
        RoleSeg::SectionEdge { .. } => write!(f, "a split's section edge"),
        RoleSeg::SplitFragment { .. } => write!(f, "a split face"),
        RoleSeg::CrossingVertex { .. } => write!(f, "a split's crossing vertex"),
        RoleSeg::OnToolVertex { .. } => write!(f, "a vertex on a split's tool"),
        RoleSeg::BlendFace(_) => write!(f, "a blend face"),
        RoleSeg::CornerFace(_) => write!(f, "a blend corner face"),
        RoleSeg::TrimEdge { .. } => write!(f, "a blend trim edge"),
        RoleSeg::FootVertex { .. } => write!(f, "a blend foot vertex"),
        RoleSeg::EndArc { .. } => write!(f, "a blend end arc"),
        RoleSeg::BandFace(_) => write!(f, "a band face"),
        RoleSeg::BandTrim { .. } => write!(f, "a band trim edge"),
        RoleSeg::BandFoot(_) => write!(f, "a band foot"),
        RoleSeg::BandCross { .. } => write!(f, "a band crossing"),
        RoleSeg::BandCut(_) => write!(f, "a band cut"),
        RoleSeg::BandSlit { .. } => write!(f, "a band slit"),
        RoleSeg::Inner(_) => write!(f, "an inner entity"),
        RoleSeg::Rim(_) => write!(f, "a rim"),
        RoleSeg::HoleRim { hole, .. } => write!(f, "the rim of hole {hole}"),
        RoleSeg::InPart { .. } => write!(f, "an entity of a part"),
        RoleSeg::Instance { i, .. } => write!(f, "an entity of instance {i}"),
    }
}

// The cutter clause of [`Diagnosis::GroupResized`]'s sentence.
impl core::fmt::Display for GroupCutters {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        fn list(f: &mut core::fmt::Formatter<'_>, names: &[StableName]) -> core::fmt::Result {
            match names {
                [one] => write!(f, "{}", Cutter(one)),
                many => {
                    write!(f, "{} cutters (", many.len())?;
                    for (i, n) in many.iter().enumerate() {
                        if i > 0 {
                            write!(f, "; ")?;
                        }
                        write!(f, "{}", Cutter(n))?;
                    }
                    write!(f, ")")
                }
            }
        }
        match self {
            Self::Read { gone, new } if gone.is_empty() && new.is_empty() => {
                write!(f, "the parent's seams name the same cutters in both runs")
            }
            Self::Read { gone, new } => {
                if !gone.is_empty() {
                    write!(f, "the parent's seams with ")?;
                    list(f, gone)?;
                    write!(f, " are gone")?;
                }
                if !new.is_empty() {
                    if !gone.is_empty() {
                        write!(f, ", and ")?;
                    }
                    write!(f, "the parent has new seams with ")?;
                    list(f, new)?;
                }
                Ok(())
            }
            Self::NotSeamBounded => write!(
                f,
                "which cutter changed is not on record, because this group is not \
                 bounded by seams on one parent name"
            ),
            Self::TiedParents => write!(
                f,
                "which cutter changed is not on record, because tied parents share the \
                 name the seams are spelled on"
            ),
            Self::NoSeamOnRecord => write!(
                f,
                "which cutter changed is not on record, because the last-good run spells \
                 no seam on the parent"
            ),
            Self::SeamUnread => write!(
                f,
                "which cutter changed is not on record, because a seam on the parent is \
                 spelled in a shape this reading does not follow"
            ),
        }
    }
}

/// What [`Diagnosis::Upstream`] found upstream of the minting node —
/// the three with-history lanes, each carrying the node it is AT,
/// since that node is by construction not one the name mentions.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UpstreamCause {
    /// A recorded verdict flip (read out of both runs' logs).
    PredicateFlip {
        /// The flipped predicate's k_stats name.
        predicate: &'static str,
        /// The node whose log recorded the flip.
        at: RecipeNodeId,
        /// Its sign in the last-good run.
        from: Sign,
        /// Its sign now.
        to: Sign,
    },
    /// A structural parameter changed.
    StructuralParam {
        /// The node whose structural parameter changed.
        node: RecipeNodeId,
        /// The structural slot.
        param: SlotId,
    },
    /// A recipe edit.
    RecipeEdit {
        /// The edit, by its structural effect.
        edit: RecipeEditRef,
    },
}

/// The subject of a flip report: the signed margin a predicate
/// decides on, named by what it decides (`crate::decision::words`).
/// What flips is that margin's sign, so "from negative to positive"
/// reads as the margin's, not as the decision's. The predicate's name
/// is routing and rides `Debug`; a predicate with no words reads as
/// `geom_core::UNNAMED_DECISION`.
struct FlipSubject<'a>(&'a str);

impl core::fmt::Display for FlipSubject<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match crate::decision::words(self.0) {
            Some(words) => write!(f, "the margin deciding {words}"),
            None => write!(f, "the margin of {}", geom_core::UNNAMED_DECISION),
        }
    }
}

// The CAUSE clause of [`Diagnosis::Upstream`]'s sentence; the arm adds
// where it sits relative to the name.
impl core::fmt::Display for UpstreamCause {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::PredicateFlip {
                predicate,
                at,
                from,
                to,
            } => write!(
                f,
                "{} flipped from {from} to {to} at node {}",
                FlipSubject(predicate),
                at.0
            ),
            Self::StructuralParam { node, param } => write!(
                f,
                "a structural parameter changed at node {} (slot {})",
                node.0,
                param.label()
            ),
            Self::RecipeEdit { edit } => write!(f, "the recipe changed ({edit})"),
        }
    }
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
    /// 2026-07-29, NAMING-DESIGN N5 as amended). For a fragment name,
    /// [`border_delta`] and [`group_resized`] answer part of that case
    /// before this one is reached; a pruned vanish of a name with no
    /// fragment qualifier — an operand corner fused away — still lands
    /// here.
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
            } => write!(
                f,
                "{} flipped from {from} to {to} on the name's derivation path",
                FlipSubject(predicate)
            ),
            Self::BorderDelta { node, gone, new } => write!(
                f,
                "at node {}, the piece of its face nearest it now no longer borders {} \
                 and borders {} it did not",
                node.0,
                Walls(gone),
                Walls(new)
            ),
            Self::GroupResized {
                node,
                was,
                now,
                cutters,
            } => write!(
                f,
                "at node {}, the group this fragment's parent was divided into held \
                 {was} entities in the last-good run and holds {now} now; {cutters}; and \
                 no verdict flip was found that explains the change",
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
            Self::ConsumedByFold { by } => write!(
                f,
                "the union that minted it consumed it before the step its declared pair is \
                 fed to: {by}"
            ),
            Self::Upstream { node, cause } => write!(
                f,
                "{cause}, upstream of node {}, the name's minting node, but not on its \
                 derivation path",
                node.0
            ),
        }
    }
}

/// A list of divider walls as a clause: `the X and the Y`, or
/// `no wall`.
struct Walls<'a>(&'a [StableName]);

impl core::fmt::Display for Walls<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self.0 {
            [] => write!(f, "no wall"),
            [one] => write!(f, "the {one}"),
            many => {
                for (i, w) in many.iter().enumerate() {
                    match i {
                        0 => write!(f, "the {w}")?,
                        _ if i + 1 == many.len() => write!(f, " and the {w}")?,
                        _ => write!(f, ", the {w}")?,
                    }
                }
                Ok(())
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
    pub width: usize,
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
/// when the node evaluates (kept outside [`ResolveError`], which is
/// N5's closed naming-verdict trio).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ResolveIndeterminate {
    /// The minting node's standing.
    pub standing: NodeStanding,
}

// The subject is the reference, and what makes this vocabulary its own
// is that the reference is indeterminate rather than vanished, so the
// recourse is always to restore the node's value, never to rebind; the
// standing supplies which node, what state and where the repair is.
impl core::fmt::Display for ResolveIndeterminate {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "the reference is indeterminate until its minting node evaluates: {}",
            self.standing
        )
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
) -> Resolution {
    resolve_impl(new, Prior { ctx: prior }, name)
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
/// - `Indeterminate` → [`Resolution::Indeterminate`] (indeterminate,
///   not vanished — one standing on both sides of the hook).
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
    enrich_impl(new, Prior { ctx: prior }, loss)
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
            error: ResolveError::ambiguous(&loss.name, loss.name.clone(), *at, ents.len()),
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
        path: &BTreeSet<RecipeNodeId>,
    ) -> Option<Diagnosis>;
    /// The upstream lanes ([`Diagnosis::Upstream`]; the scope rule is
    /// stated at [`upstream_nodes`]).
    fn upstream<T: Decide>(
        &self,
        new: RunCtx<'_, T>,
        name: &StableName,
        path: &BTreeSet<RecipeNodeId>,
    ) -> Option<Diagnosis>;
    fn tombstone<T: Decide>(&self, new: RunCtx<'_, T>, name: &StableName) -> Option<Tombstone>;
    /// The group-size rung ([`group_resized`]): with-prior only, since
    /// a size CHANGE needs a size to change from.
    fn group_resized<T: Decide>(&self, new: RunCtx<'_, T>, name: &StableName) -> Option<Diagnosis>;
    /// Whether the last-good run's tables carry `name` (none without a
    /// prior run).
    fn carried(&self, name: &StableName) -> bool;
}

struct NoPrior;

/// The last-good run — what a with-history diagnosis needs and a
/// single-run resolution does not.
#[derive(Clone, Copy)]
struct Prior<'a, U: Decide> {
    ctx: RunCtx<'a, U>,
}

impl<U: Decide> Prior<'_, U> {
    /// The document the prior run is OF.
    fn doc(&self) -> &Doc<ProfileProgram> {
        self.ctx.doc
    }
}

impl PriorCtx for NoPrior {
    fn carried(&self, _name: &StableName) -> bool {
        false
    }

    fn diagnose<T: Decide>(
        &self,
        _new: RunCtx<'_, T>,
        _path: &BTreeSet<RecipeNodeId>,
    ) -> Option<Diagnosis> {
        None
    }

    fn upstream<T: Decide>(
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

    fn group_resized<T: Decide>(
        &self,
        _new: RunCtx<'_, T>,
        _name: &StableName,
    ) -> Option<Diagnosis> {
        None
    }
}

impl<U: Decide> Prior<'_, U> {
    /// The LANE TABLE: the three with-history lanes over one node set,
    /// in their fixed order — a recorded flip, then a structural
    /// parameter, then a recipe edit — answering the first evidence
    /// found. Both scopes run exactly this; what differs between them
    /// is the node set and the arm the evidence is wrapped in, and
    /// the path scope's extra rungs ahead of it (the name's own
    /// qualifier vocabulary), which this table deliberately does not
    /// know about: a `name_frag_*` flip at an upstream node
    /// re-qualified some OTHER name.
    fn lanes<T: Decide>(
        &self,
        new: RunCtx<'_, T>,
        flips: &FlipSet,
        nodes: &BTreeSet<RecipeNodeId>,
    ) -> Option<Evidence> {
        if let Some((node, f)) = flips.flips_on_nodes(nodes).first() {
            return Some(Evidence::Flip(*node, *f));
        }
        let ddiff = self.doc().diff(new.doc);
        if let Some((node, param)) =
            structural_param_change(self.doc(), new.doc, &ddiff, Some(nodes))
        {
            return Some(Evidence::Param(node, param));
        }
        recipe_edit_change(self.doc(), new.doc, &ddiff, Some(nodes)).map(Evidence::Edit)
    }
}

/// One lane's find ([`Prior::lanes`]), before a scope wraps it in the
/// arm that states where it was found.
enum Evidence {
    Flip(RecipeNodeId, VerdictFlip),
    Param(RecipeNodeId, SlotId),
    Edit(RecipeEditRef),
}

impl<U: Decide> PriorCtx for Prior<'_, U> {
    fn carried(&self, name: &StableName) -> bool {
        lookup(self.ctx.eval, name).is_some()
    }

    /// The PATH scope (the scope rule: [`upstream_nodes`]): the lane
    /// table over [`derivation_nodes`], answering `PredicateFlip`,
    /// `StructuralParam` or `RecipeEdit`, with the name's own
    /// qualifier vocabulary ahead of it:
    ///
    /// 1. **A recorded `name_frag_*` flip on the path wins** — those
    ///    predicates are the name's OWN qualifier vocabulary, so a
    ///    discriminator flip is definitionally the flip that
    ///    re-qualified the fragment.
    /// 2. The lane table ([`Prior::lanes`]) over the path.
    ///
    /// This is a consumer-side attribution choice — the diff engine
    /// itself stays cause-agnostic and unspecialized.
    fn diagnose<T: Decide>(
        &self,
        new: RunCtx<'_, T>,
        path: &BTreeSet<RecipeNodeId>,
    ) -> Option<Diagnosis> {
        let flips = diff_verdicts(self.ctx.eval, new.eval);
        let family = flips
            .flips_on_nodes(path)
            .into_iter()
            .find(|(_, f)| f.predicate.starts_with(crate::names::FAMILY));
        let flip = |f: VerdictFlip| Diagnosis::PredicateFlip {
            predicate: f.predicate,
            from: f.from,
            to: f.to,
        };
        if let Some((_, f)) = family {
            return Some(flip(f));
        }
        Some(match self.lanes(new, &flips, path)? {
            Evidence::Flip(_, f) => flip(f),
            Evidence::Param(node, param) => Diagnosis::StructuralParam { node, param },
            Evidence::Edit(edit) => Diagnosis::RecipeEdit { edit },
        })
    }

    /// The UPSTREAM scope ([`upstream_nodes`]): the lane table over the
    /// minting node's ancestors that are not on the path, each find
    /// wrapped in [`Diagnosis::Upstream`] with the node it is at.
    fn upstream<T: Decide>(
        &self,
        new: RunCtx<'_, T>,
        name: &StableName,
        path: &BTreeSet<RecipeNodeId>,
    ) -> Option<Diagnosis> {
        let nodes = upstream_nodes(self.doc(), new.doc, name.node, path);
        let flips = diff_verdicts(self.ctx.eval, new.eval);
        let cause = match self.lanes(new, &flips, &nodes)? {
            Evidence::Flip(at, f) => UpstreamCause::PredicateFlip {
                predicate: f.predicate,
                at,
                from: f.from,
                to: f.to,
            },
            Evidence::Param(node, param) => UpstreamCause::StructuralParam { node, param },
            Evidence::Edit(edit) => UpstreamCause::RecipeEdit { edit },
        };
        Some(Diagnosis::Upstream {
            node: name.node,
            cause,
        })
    }

    fn group_resized<T: Decide>(&self, new: RunCtx<'_, T>, name: &StableName) -> Option<Diagnosis> {
        group_resized(self.ctx.eval, new.eval, name)
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
                error: ResolveError::ambiguous(name, name.clone(), node, ents.len()),
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
                    error: ResolveError::ambiguous(name, base, node, ents.len()),
                    offers: Vec::new(),
                });
            }
            // The group collapsed to a single fragment: the ranked
            // name is gone; the surviving base is the offer.
            Some((_, Entry::Unique(_))) => offers.push(base),
            None => {}
        }
    } else if let Some(base) = unqualified(name)
        // The same collapse for a face piece: the undivided survivor is
        // offered for an explicit `Rebind`, never bound. There is no
        // over-tie to widen to here — a `Borders` tie is a row of the
        // QUALIFIED name, which step 2 already answered.
        && matches!(lookup(new.eval, &base), Some((_, Entry::Unique(_))))
    {
        offers.push(base);
    }

    // 4. The minting node's standing decides Vanished vs
    //    Indeterminate.
    if let Err(standing) = new.eval.usable(name.node) {
        return Resolution::Indeterminate(ResolveIndeterminate { standing });
    }

    // 5. Vanished. N3 structural offers first (merge/unmerge), then
    //    the diagnosis ladder.
    offers.extend(merge_offers(new.eval, name));

    // Cascade dominates: an embedded operand name that itself fails
    // to resolve carries the root cause (its own diagnosis chains).
    let mut cascade: Option<StableName> = None;
    walk_names(name, Partners::Cascade, &mut |inner| {
        if cascade.is_none() && lookup(new.eval, inner).is_none() {
            cascade = Some(inner.clone());
        }
    });
    let path = derivation_nodes(name);
    let diagnosis = if let Some(through) = cascade {
        Diagnosis::Cascade { through }
    } else {
        prior
            .diagnose(new, &path)
            // The qualifier-delta rung (review Finding 1 ruling), for
            // a face piece the border delta: when the verdict-diff
            // and doc-diff lanes have no evidence — the
            // population-cancel blind spot, or a single-run resolve —
            // the N2 discriminators recorded IN the names themselves
            // are still evidence.
            .or_else(|| border_delta(new.eval, name, |n| prior.carried(n)))
            // The upstream scope ([`upstream_nodes`]): below every
            // rung that names a cause ON the path, qualifier delta
            // included, because a path cause decided the name and an
            // upstream one only fed it.
            .or_else(|| prior.upstream(new, name, &path))
            // The group-size rung (`group_resized`'s docs).
            .or_else(|| prior.group_resized(new, name))
            // Every rung above came up empty: no verdict flip, no doc
            // delta, no recorded qualifier delta, and no group-size
            // change the rung could state (unchanged, or too large to
            // count).
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

/// A fragment name's BASE: the name without its trailing `Fragment`
/// qualifier, of either kind — what its group's rows are spelled from.
/// `None` when the name has no fragment tail, or when popping it
/// would leave an empty path (never emitted: a qualifier always
/// follows a parent-bearing segment, N2).
///
/// The one spelling of the pop. [`unqualified`] and [`widened_base`]
/// are this, restricted to one qualifier kind each, and
/// [`group_resized`] is this unrestricted.
fn fragment_base(name: &StableName) -> Option<StableName> {
    if !matches!(name.path.last(), Some(RoleSeg::Fragment(_))) {
        return None;
    }
    let mut base = name.clone();
    base.path.pop();
    (!base.path.is_empty()).then_some(base)
}

/// A face piece's base ([`fragment_base`]) — what the SAME group is
/// called once it stops being multi-fragment, and therefore the
/// current-run counterpart of a vanished piece.
///
/// Not [`widened_base`]: that one asks which ROW a ranked reference
/// landed on and must refuse a `Borders` tail; this one asks which FACE
/// a piece became and must refuse an `OrderAlong` tail. Same pop,
/// different questions, so different filters.
fn unqualified(name: &StableName) -> Option<StableName> {
    matches!(
        name.path.last(),
        Some(RoleSeg::Fragment(Qualifier::Borders(_)))
    )
    .then(|| fragment_base(name))
    .flatten()
}

/// The BORDER-DELTA rung — N5's qualifier-delta rung for a face piece
/// ([`Diagnosis::BorderDelta`]): a vanished piece's name records the
/// divider walls it bordered (`Qualifier::Borders`), and so does every
/// piece its parent is held as now, so the change is readable off the
/// names even when no log recorded it (the population-cancel blind
/// spot, a pruned pair, or no prior run at all).
///
/// The counterpart is a current piece under the same base that still
/// borders at least one of the vanished piece's walls — a piece sharing
/// none is another piece, not this one changed — and that the last-good
/// run did not already publish (`carried`: an untouched sibling is not
/// a counterpart). Among those, it is the one whose wall set differs
/// least from the vanished one's (the size of the symmetric
/// difference). It answers only when that nearest set is unique, and
/// says which walls the vanished piece bordered and that piece does
/// not, and the reverse; no candidate, or two equally near, and the
/// rung declines rather than pick. A group that stopped being divided
/// has no `Borders` sibling left, and the group-size rung answers it.
fn border_delta<T: Decide>(
    eval: &Evaluation<T>,
    name: &StableName,
    carried: impl Fn(&StableName) -> bool,
) -> Option<Diagnosis> {
    let RoleSeg::Fragment(Qualifier::Borders(was)) = name.path.last()? else {
        return None;
    };
    let base = fragment_base(name)?;
    let was: BTreeSet<&StableName> = was.iter().collect();
    let mut now: BTreeSet<Vec<StableName>> = BTreeSet::new();
    for (_, table) in tables(eval) {
        for (candidate, _) in table.iter() {
            if let Some(RoleSeg::Fragment(Qualifier::Borders(walls))) = candidate.path.last()
                && candidate.kind == name.kind
                && candidate.node == name.node
                && fragment_base(candidate).as_ref() == Some(&base)
                && walls.iter().any(|w| was.contains(w))
                && !carried(candidate)
            {
                now.insert(walls.clone());
            }
        }
    }
    let distance = |walls: &Vec<StableName>| {
        let walls: BTreeSet<&StableName> = walls.iter().collect();
        was.symmetric_difference(&walls).count()
    };
    let nearest = now.iter().map(distance).min()?;
    let mut at = now.iter().filter(|w| distance(w) == nearest);
    let walls = at.next()?;
    if at.next().is_some() {
        return None;
    }
    let walls: BTreeSet<&StableName> = walls.iter().collect();
    Some(Diagnosis::BorderDelta {
        node: name.node,
        gone: was.difference(&walls).map(|&w| w.clone()).collect(),
        new: walls.difference(&was).map(|&w| w.clone()).collect(),
    })
}

/// The GROUP-SIZE rung ([`Diagnosis::GroupResized`]): the vanished
/// name's fragment group, counted in the minting node's table in the
/// last-good run and in this one, changed size.
///
/// # Why a fragment name can vanish with no flip
///
/// This is the one statement of it; the sites that need it point
/// here. A fragment qualifier exists only while its group has two or
/// more members (N2), and `OrderAlong` spells the group's size into
/// the name as `of`. So a fragment name vanishes whenever its group
/// changes size, and that event need not flip any discriminator: a
/// `Borders` group that stops being divided leaves no piece whose walls
/// could be compared — the walls still stand where they stood relative
/// to the survivor — and an `OrderAlong` group ranks its members
/// against EACH OTHER, so a group of one runs no pair. What remains in
/// evidence is the count.
///
/// # What is counted
///
/// The group is the one the minting node's emitter FORMED, read from
/// the record it keeps beside its name table
/// ([`crate::names::FragmentGroups`]) under the name's BASE
/// ([`fragment_base`]): the entities of that node's output that descend
/// from the group's parent by the emitter's own descent, however each
/// is spelled. A member passing
/// through undivided under its upstream name counts, and so does an N3
/// `Merged` face the parent survives in. At a UNION the output is the
/// published body, so a group a fold step formed counts the DISTINCT
/// published entities that descend from its parent within it, the
/// descent followed by entity through every later step: a member a
/// later step swallows counts 0, one it divides counts each piece, and
/// a later merged face holding two pieces counts once. Whichever step
/// spelled the base, the count is that one number. Both records are
/// the MINTING node's own (`name.node`), because the group is what
/// that node's emission divided. A piece a later step re-mints under a
/// seam name of its own (a `Seam` edge zipped along the piece's line)
/// is not the parent's descendant by that descent and is not counted.
///
/// Two TIED parents share one base. Where the emitter groups by parent
/// ENTITY — a face or an edge by the operand entity it descends from,
/// a split face by its face and side — each forms its own group, the
/// tie lane gives their members' rows one set of names, and each group
/// is counted on its own: `was` and `now` are one parent's group, never
/// the tie's sum. Where it groups by parent NAMES — a seam edge or seam
/// vertex by its two faces' names — tied parents' pieces land in one
/// group, no one parent's count is on record, and the rung declines.
///
/// `now == 0` says no entity of the node's output is in that group:
/// none descends from the parent there by the emitter's descent, or — for a split, whose group
/// is a face AND a side — none on that side.
///
/// # When it answers, and when it declines
///
/// It answers when the last-good minting table CARRIED the name — a
/// name the prior run never minted did not vanish by its group
/// changing — and the current count differs. It declines, to the
/// evidence-free fallback, when the name has no fragment tail, when
/// either run has no value at the minting node, when the prior record
/// holds no group under the base, when a group under the base was
/// formed by tied parents' names or the groups a tie shares a base
/// among are not all one size (there is then no one parent's count to
/// state), when a union's record cannot be read in its published space
/// or two of one fold step's bases publish as one, when either count
/// does not fit the diagnosis's `u32` (a saturated size could make two
/// different groups read as one), and when the size did NOT change: a
/// group that re-qualified at the same size is a different event, about
/// which the records say nothing.
///
/// # Why it sits last: cause before effect
///
/// Every rung above it that answers names a CAUSE — a recorded flip
/// names a predicate whose verdict changed, the border delta names the
/// walls that moved, the doc-diff lanes
/// name an edit, and [`Diagnosis::Upstream`] names one of those three
/// at a node that fed the name. This rung states an EFFECT, a structural change whose
/// cause the evidence does not hold. When both are present — the bar
/// slid, a predicate flipped, and the group resized as a consequence —
/// the cause is the answer and the resize is its symptom, so this rung
/// runs only once every cause-naming rung has come up empty, and
/// before the fallback, which states nothing at all. Diagnosis-time
/// only; two record reads and, for the cutters ([`group_cutters`]),
/// two table scans; nothing reaches any log.
fn group_resized<U: Decide, T: Decide>(
    prior: &Evaluation<U>,
    new: &Evaluation<T>,
    name: &StableName,
) -> Option<Diagnosis> {
    let base = fragment_base(name)?;
    let prior_value = prior.value(name.node)?;
    prior_value.name_table.lookup(name)?;
    let was = group_reading(&prior_value.fragment_groups, &base)?;
    if was.size == 0 {
        return None;
    }
    let new_value = new.value(name.node)?;
    let now = group_reading(&new_value.fragment_groups, &base)?;
    (was.size != now.size).then(|| Diagnosis::GroupResized {
        node: name.node,
        was: was.size,
        now: now.size,
        cutters: group_cutters(
            &prior_value.name_table,
            &new_value.name_table,
            &base,
            prior_value.fragment_groups.is_folded(),
            was.parents > 1 || now.parents > 1,
        ),
    })
}

/// One run's reading of the groups under a base: one parent's size, and
/// how many parents' groups share the base (more than one is a tie).
struct GroupReading {
    size: u32,
    parents: usize,
}

/// One parent's group size under `base` in one run's record
/// ([`group_resized`]'s count), with the number of groups that share
/// the base: size `0` when no group is recorded there, `None` when the
/// groups sharing the base (a tie's) differ in size, when the record
/// cannot be read, or when the size does not fit the diagnosis's `u32`
/// — the rung declines rather than report a saturated size two
/// different groups would share.
fn group_reading(groups: &crate::names::FragmentGroups, base: &StableName) -> Option<GroupReading> {
    let sizes = groups.sizes(base)?;
    let size = match sizes.as_slice() {
        [] => 0,
        [one, rest @ ..] => {
            if !rest.iter().all(|s| s == one) {
                return None;
            }
            u32::try_from(*one).ok()?
        }
    };
    Some(GroupReading {
        size,
        parents: sizes.len(),
    })
}

/// [`Diagnosis::GroupResized`]'s cutters: the partners of the `Seam`
/// rows spelled on the group's parent at the minting node, in the
/// last-good table and in the current one, and which of them only one
/// table spells ([`GroupCutters`]). A statement about spelled seams: a
/// cutter listed is one whose seam with the parent only one run spells,
/// whether that seam divided the parent or only trimmed it.
///
/// # Which seams are read
///
/// The emitter spells each crossing of the op by the two operand
/// entities that made it (`names::emit_topo`, `name_boolean_edges` and
/// `name_boolean_vertices`): a seam EDGE by two faces, so a face
/// group's parent meets its cutters along seam edges; a seam VERTEX by
/// an edge and the face, edge or vertex it met, so an edge group's
/// parent meets them at seam vertices. So the rows read are the minting
/// node's own rows of the kind one down from the group's — an edge for
/// a face group, a vertex for an edge group — with a `Seam` segment one
/// side of which is on the parent. Such a row is read when it is one
/// `Seam { a, b }` with only `Fragment`s after it, however many (the
/// pair is matched, not the row). Any other shape on the parent makes
/// the whole reading [`GroupCutters::SeamUnread`]: a comparison that
/// skipped it could name a cutter gone that was only not read. A face
/// group's seam VERTICES (a cutter's edge piercing the face) are of the
/// wrong kind and never read.
///
/// The parent is read off the group's base, per emitter:
/// - a pair boolean's `FromA(p)` / `FromB(p)`: the seam's A side / B
///   side is `p`, spelled in the operand's table, and the cutter is the
///   other side, verbatim — an operand's names are its own published
///   identity, so a cutter the operand re-qualified reads as renamed;
/// - a union's `FromMember { .. }`, carried `Seam { .. }` line or
///   `Merged(..)` face, with any fold-accumulated `Fragment` tail: the
///   union's seams are in name order, so either side may be the parent,
///   and a side that is the base with only `Fragment`s after it (a
///   later fold step cutting one of the group's own pieces) is on it.
///   The cutter is the other side with ITS fold-accumulated `Fragment`
///   tail dropped: a union's trailing fragments are the fold's, not the
///   member's, so a cutter a fold step re-ranked or stopped dividing is
///   the same cutter. A `Seam`-headed base is the pair case's
///   `FromA(Seam ..)` — a seam line an earlier step minted, cut by a
///   later step's seam vertices — read the same way;
/// - any other base — a split's `SplitFragment` (its one cutter is the
///   tool, which is no row), a pair boolean's own seam chain (pieces of
///   one crossing, not pieces a seam divided) or merged face — is
///   [`GroupCutters::NotSeamBounded`].
///
/// # When it names none
///
/// When the groups a tie shares the base among are more than one, in
/// either run ([`GroupCutters::TiedParents`]): the seam lanes spell a
/// crossing on the parent's NAME, which the tie shares, so which
/// parent's group a seam bounds is not on record. When the last-good
/// table spells no seam on the parent at all
/// ([`GroupCutters::NoSeamOnRecord`]): comparing an empty prior read
/// would state "the same cutters" on no evidence. And when a seam row
/// on the parent has a shape the reading does not follow
/// ([`GroupCutters::SeamUnread`]).
///
/// # What it does not see
///
/// A cutter VERTEX the kernel fuses onto the parent edge
/// (`vertex_merges`: a touch, not a crossing) is spelled as the pass-down
/// vertex it fused into, not as a `Seam` row, so a contact that went
/// from a crossing to a touch reads as the cutter gone. Telling the two
/// apart needs the body, which this reading does not consult.
fn group_cutters(
    prior_table: &crate::names::NameTable,
    new_table: &crate::names::NameTable,
    base: &StableName,
    union: bool,
    tied: bool,
) -> GroupCutters {
    let Some(parent) = SeamParent::of(base, union) else {
        return GroupCutters::NotSeamBounded;
    };
    if tied {
        return GroupCutters::TiedParents;
    }
    let Some(before) = parent.cutters(base, prior_table) else {
        return GroupCutters::SeamUnread;
    };
    if before.is_empty() {
        return GroupCutters::NoSeamOnRecord;
    }
    let Some(after) = parent.cutters(base, new_table) else {
        return GroupCutters::SeamUnread;
    };
    GroupCutters::Read {
        gone: before.difference(&after).cloned().collect(),
        new: after.difference(&before).cloned().collect(),
    }
}

/// Where a group's parent sits in the `Seam` rows of its minting node
/// ([`group_cutters`]).
enum SeamParent<'a> {
    /// A pair boolean's operand entity: the A side of every seam on it,
    /// or the B side.
    Pair {
        parent: &'a StableName,
        a_side: bool,
    },
    /// A union's group: either side, the base or a piece of it.
    Union,
}

impl<'a> SeamParent<'a> {
    fn of(base: &'a StableName, union: bool) -> Option<Self> {
        // A face is divided by seam edges, an edge by seam vertices;
        // nothing divides a vertex or a body.
        if !matches!(base.kind, EntityKind::Face | EntityKind::Edge) {
            return None;
        }
        let head = &base.path[..fragment_tail_start(&base.path)];
        match (union, head) {
            (false, [RoleSeg::FromA(p)]) if head.len() == base.path.len() => Some(Self::Pair {
                parent: p,
                a_side: true,
            }),
            (false, [RoleSeg::FromB(p)]) if head.len() == base.path.len() => Some(Self::Pair {
                parent: p,
                a_side: false,
            }),
            (true, [RoleSeg::FromMember { .. } | RoleSeg::Seam { .. } | RoleSeg::Merged(_)]) => {
                Some(Self::Union)
            }
            _ => None,
        }
    }

    /// Whether `side`, one side of a seam, is on the parent of the group
    /// `base` names.
    fn holds(&self, base: &StableName, side: &StableName) -> bool {
        match *self {
            Self::Pair { parent, .. } => side == parent,
            Self::Union => {
                side.node == base.node
                    && side.kind == base.kind
                    && side.path.len() >= base.path.len()
                    && side.path[..base.path.len()] == base.path[..]
                    && fragment_tail_start(&side.path[base.path.len()..]) == 0
            }
        }
    }

    /// The cutter across a seam whose sides are `(a, b)`, or `None` when
    /// neither side is on the parent.
    fn across<'n>(
        &self,
        base: &StableName,
        a: &'n StableName,
        b: &'n StableName,
    ) -> Option<&'n StableName> {
        match *self {
            Self::Pair { a_side: true, .. } => self.holds(base, a).then_some(b),
            Self::Pair { a_side: false, .. } => self.holds(base, b).then_some(a),
            Self::Union if self.holds(base, a) => Some(b),
            Self::Union if self.holds(base, b) => Some(a),
            Self::Union => None,
        }
    }

    /// A cutter as the two runs are compared on it: verbatim in a pair,
    /// its fold-accumulated `Fragment` tail dropped in a union.
    fn normalized(&self, cutter: &StableName) -> StableName {
        match self {
            Self::Pair { .. } => cutter.clone(),
            Self::Union => {
                let mut c = cutter.clone();
                c.path.truncate(fragment_tail_start(&c.path));
                c
            }
        }
    }

    /// The cutters `table` spells a seam with the parent of the group
    /// `base` names against, or `None` when a seam row on the parent has
    /// a shape this reading does not follow.
    fn cutters(
        &self,
        base: &StableName,
        table: &crate::names::NameTable,
    ) -> Option<BTreeSet<StableName>> {
        let seam_kind = match base.kind {
            EntityKind::Face => EntityKind::Edge,
            _ => EntityKind::Vertex,
        };
        let mut out = BTreeSet::new();
        for (row, _) in table.iter() {
            if row.node != base.node || row.kind != seam_kind {
                continue;
            }
            let on_parent = row.path.iter().any(|seg| match seg {
                RoleSeg::Seam { a, b } => self.across(base, a, b).is_some(),
                _ => false,
            });
            if !on_parent {
                continue;
            }
            let (RoleSeg::Seam { a, b }, tail) = row.path.split_first()? else {
                return None;
            };
            if fragment_tail_start(tail) != 0 {
                return None;
            }
            out.insert(self.normalized(self.across(base, a, b)?));
        }
        Some(out)
    }
}

/// Where the trailing run of `Fragment` segments of `path` starts: the
/// length of what they qualify. The one reading of "a name with only
/// fragments after it", for the parent side, the cutter side and a seam
/// row alike ([`group_cutters`]).
fn fragment_tail_start(path: &[RoleSeg]) -> usize {
    path.iter()
        .rposition(|s| !matches!(s, RoleSeg::Fragment(_)))
        .map_or(0, |i| i + 1)
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
    matches!(
        name.path.last(),
        Some(RoleSeg::Fragment(Qualifier::OrderAlong { .. }))
    )
    .then(|| fragment_base(name))
    .flatten()
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
/// `name` as a discriminator PARTNER (a `Borders` wall, a slit's or
/// crossing's band) is not a derivation of
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

/// The UPSTREAM node set of a name minted at `node`, and THE SCOPE
/// RULE of the with-history lanes, stated here once.
///
/// The lanes read two scopes, in order. The PATH is
/// [`derivation_nodes`] (N1: the nodes the name mentions); evidence
/// there is a cause that DECIDED the name, answered as
/// `PredicateFlip`, `StructuralParam` or `RecipeEdit`, whose sentences
/// say "on the derivation path". UPSTREAM is this set: every node
/// that is a strict ancestor of `node` in the last-good document OR
/// in the current one, each walked within its own document, minus the
/// path; evidence there FED the name without deciding it, answered as
/// [`Diagnosis::Upstream`], whose sentence says so. A node in neither
/// set is never read — no edit there reaches this name in either run.
///
/// The two documents are walked separately and then united, never
/// walked together: a chain that crosses from an old edge to a new
/// one reaches nodes that fed the minting node in NEITHER run.
fn upstream_nodes(
    old: &Doc<ProfileProgram>,
    new: &Doc<ProfileProgram>,
    node: RecipeNodeId,
    path: &BTreeSet<RecipeNodeId>,
) -> BTreeSet<RecipeNodeId> {
    let mut nodes = crate::roots::strict_ancestors(old, node);
    nodes.append(&mut crate::roots::strict_ancestors(new, node));
    nodes.retain(|n| !path.contains(n));
    nodes
}

/// Whether a name walk visits discriminator PARTNERS: a `Borders`
/// qualifier's walls, and the `band` of a [`RoleSeg::BandCross`] or
/// [`RoleSeg::BandSlit`]. Partners are discrimination references — an
/// edit at a partner's node can re-qualify the name (N7 localization),
/// but the name is not DERIVED from the partner (suggestions must not
/// offer the other body's fragments for a painted cutter wall — review
/// Finding 2 — nor a band's slit for one of its rim edges).
#[derive(Clone, Copy, PartialEq, Eq)]
enum Partners {
    /// Visit partner names (localization).
    Include,
    /// Visit a band, and not a piece's walls (cascade): a wall is cited
    /// by its parent's name, which no table holds once the wall is
    /// itself cut, so a wall that does not resolve is no vanished
    /// operand; a change of walls is the border delta's to state.
    Cascade,
    /// Skip partner positions (structural embedding only).
    Skip,
}

/// Visits every name embedded in `name`'s role path, at every depth,
/// in path order (operand names, seam pairs, merged constituents,
/// pattern masters — and discriminator partners iff `partners` says
/// so). [`embedded`]'s match is EXHAUSTIVE on purpose: a future [`RoleSeg`] or
/// [`Qualifier`] variant embedding names must be
/// classified here or the compile breaks — or, if it embeds no name,
/// added to [`crate::names::name_free_seg`], which is the one place
/// that answer is written for every match that shares it.
/// (Review Finding 7 — no fail-quiet wildcard.)
///
/// The walk keeps the names still to visit on its own stack: a name
/// nests as deep as its derivation, with no bound.
fn walk_names<'a>(name: &'a StableName, partners: Partners, f: &mut impl FnMut(&'a StableName)) {
    let mut names = Vec::new();
    embedded(name, partners, &mut names);
    names.reverse();
    while let Some(n) = names.pop() {
        f(n);
        let deeper = names.len();
        embedded(n, partners, &mut names);
        if let Some(held) = names.get_mut(deeper..) {
            held.reverse();
        }
    }
}

/// The names [`walk_names`] visits one level down from `name`, in path
/// order.
fn embedded<'a>(name: &'a StableName, partners: Partners, f: &mut Vec<&'a StableName>) {
    fn visit<'a>(n: &'a StableName, _: Partners, f: &mut Vec<&'a StableName>) {
        f.push(n);
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
            | RoleSeg::BandCut(n)
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
            // The source edge is derivation; the band is a
            // DISCRIMINATOR — it says which of the bands on that edge
            // made the entity, and the entity does not replace any of
            // the band's rim edges — so it is a partner position.
            RoleSeg::BandCross { edge, band } | RoleSeg::BandSlit { edge, band } => {
                visit(edge, partners, f);
                if partners != Partners::Skip {
                    for n in band {
                        visit(n, partners, f);
                    }
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
            // Discrimination references, not derivation.
            RoleSeg::Fragment(q) => match q {
                Qualifier::Borders(walls) => {
                    if partners == Partners::Include {
                        for n in walls {
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

/// [`walk_names`] with partners included — the localization walk (N7:
/// a flip at a partner node re-qualifies the name).
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

#[cfg(test)]
mod tests {
    //! [`group_cutters`]'s reading on hand tables: which rows it reads
    //! as a seam on the parent, how it compares a cutter across runs,
    //! and the places it names none.
    #![allow(clippy::unwrap_used, clippy::expect_used)]

    use super::*;
    use crate::names::{CapEnd, FragmentGroups, NameRef, NameTable, ProfileEdgeRef};
    use topo::{EdgeKey, FaceKey, VertexKey};

    const NODE: RecipeNodeId = RecipeNodeId(7);

    fn face(node: u64, seg: u32) -> StableName {
        StableName {
            kind: EntityKind::Face,
            node: RecipeNodeId(node),
            path: vec![RoleSeg::Lateral(ProfileEdgeRef::Piece {
                step: crate::node::StepId(u64::from(seg)),
                role: crate::names::PieceRole::Leg,
            })],
        }
    }

    /// The parent: operand A's top.
    fn top() -> StableName {
        StableName {
            kind: EntityKind::Face,
            node: RecipeNodeId(2),
            path: vec![RoleSeg::Cap(CapEnd::End)],
        }
    }

    /// A pair boolean's group of `top()`'s pieces.
    fn base() -> StableName {
        StableName {
            kind: EntityKind::Face,
            node: NODE,
            path: vec![RoleSeg::FromA(NameRef::new(top()))],
        }
    }

    /// `inner` as union member `member`'s entity, with `tail` after it.
    fn member(member: u64, inner: StableName, tail: &[RoleSeg]) -> StableName {
        StableName {
            kind: inner.kind,
            node: NODE,
            path: core::iter::once(RoleSeg::FromMember {
                member: RecipeNodeId(member),
                of: NameRef::new(inner),
            })
            .chain(tail.iter().cloned())
            .collect(),
        }
    }

    fn seam(kind: EntityKind, a: StableName, b: StableName, tail: &[RoleSeg]) -> StableName {
        StableName {
            kind,
            node: NODE,
            path: core::iter::once(RoleSeg::Seam {
                a: NameRef::new(a),
                b: NameRef::new(b),
            })
            .chain(tail.iter().cloned())
            .collect(),
        }
    }

    fn rank(rank: u32, of: u32) -> RoleSeg {
        RoleSeg::Fragment(Qualifier::OrderAlong { rank, of })
    }

    fn table(rows: Vec<StableName>) -> NameTable {
        let mut t = NameTable::new();
        for (i, row) in rows.into_iter().enumerate() {
            let key = match row.kind {
                EntityKind::Face => EntityKey::Face(FaceKey::default()),
                EntityKind::Edge => EntityKey::Edge(EdgeKey::default()),
                _ => EntityKey::Vertex(VertexKey::default()),
            };
            t.insert(
                row,
                EntityRef {
                    body: u32::try_from(i).unwrap(),
                    key,
                },
            )
            .unwrap();
        }
        t
    }

    /// A pair boolean's reading of `prior` against `now`.
    fn pair(prior: Vec<StableName>, now: Vec<StableName>) -> GroupCutters {
        group_cutters(&table(prior), &table(now), &base(), false, false)
    }

    fn read(gone: Vec<StableName>, new: Vec<StableName>) -> GroupCutters {
        GroupCutters::Read { gone, new }
    }

    /// A cutter is its `Seam` pair: ranked in one run and single in the
    /// other, it is the same cutter. A pair with the parent on the B
    /// side and a seam of the wrong kind (a face group's seam VERTEX) are
    /// not seams on this parent: the current table alone carries them,
    /// so a reading that took either would name its cutter new.
    #[test]
    fn a_cutter_is_its_seam_pair_on_the_parents_side_and_kind() {
        let prior = vec![
            seam(EntityKind::Edge, top(), face(5, 1), &[rank(0, 2)]),
            seam(EntityKind::Edge, top(), face(5, 1), &[rank(1, 2)]),
        ];
        let now = vec![
            seam(EntityKind::Edge, top(), face(5, 1), &[]),
            seam(EntityKind::Edge, top(), face(5, 0), &[]),
            seam(EntityKind::Edge, face(5, 2), top(), &[]),
            seam(EntityKind::Vertex, top(), face(5, 3), &[]),
        ];
        assert_eq!(pair(prior, now), read(vec![], vec![face(5, 0)]));
    }

    /// A seam row with more than one `Fragment` after its pair is read
    /// like any other: a cutter ranked once in one run and twice in the
    /// other is not gone.
    #[test]
    fn a_seam_with_a_deep_fragment_tail_is_read() {
        let prior = vec![seam(EntityKind::Edge, top(), face(5, 1), &[rank(0, 2)])];
        let now = vec![seam(
            EntityKind::Edge,
            top(),
            face(5, 1),
            &[rank(0, 2), rank(1, 2)],
        )];
        assert_eq!(pair(prior, now), read(vec![], vec![]));
    }

    /// A seam row on the parent in a shape the reading does not follow —
    /// here a run of seam lines — makes the whole reading unread, in
    /// either run, rather than a comparison over the rest.
    #[test]
    fn an_unread_seam_on_the_parent_names_none() {
        let mut run = seam(EntityKind::Edge, top(), face(5, 3), &[]);
        run.path.push(RoleSeg::Seam {
            a: NameRef::new(top()),
            b: NameRef::new(face(5, 2)),
        });
        let plain = || seam(EntityKind::Edge, top(), face(5, 1), &[]);
        assert_eq!(
            pair(vec![plain(), run.clone()], vec![plain()]),
            GroupCutters::SeamUnread
        );
        assert_eq!(
            pair(vec![plain()], vec![plain(), run]),
            GroupCutters::SeamUnread
        );
    }

    /// A prior table with no seam on the parent names no cutter, rather
    /// than state that the cutters are the same.
    #[test]
    fn no_seam_on_the_parent_in_the_prior_table_names_none() {
        let now = vec![seam(EntityKind::Edge, top(), face(5, 1), &[])];
        assert_eq!(pair(vec![], now), GroupCutters::NoSeamOnRecord);
    }

    /// Two groups under one base are two tied parents, whichever run
    /// records them, read once per run.
    #[test]
    fn two_groups_under_one_base_read_as_tied_parents() {
        let one = FragmentGroups::from_sizes([(base(), 2)]);
        let tied = FragmentGroups::from_sizes([(base(), 2), (base(), 2)]);
        assert_eq!(group_reading(&one, &base()).unwrap().parents, 1);
        assert_eq!(group_reading(&tied, &base()).unwrap().parents, 2);
        let rows = || table(vec![seam(EntityKind::Edge, top(), face(5, 1), &[])]);
        assert_eq!(
            group_cutters(&rows(), &rows(), &base(), false, true),
            GroupCutters::TiedParents
        );
    }

    /// In a union a seam's sides are in name order, the parent may be
    /// either, and a cutter's fold-accumulated `Fragment` tail is the
    /// fold's: a wall another member divided in one run and not in the
    /// other, or ranked differently, is the same cutter.
    #[test]
    fn a_union_cutter_is_compared_without_its_fold_tail() {
        let base = member(1, top(), &[]);
        let wall = |tail: &[RoleSeg]| member(3, face(5, 1), tail);
        let prior = vec![
            seam(EntityKind::Edge, wall(&[rank(0, 2)]), base.clone(), &[]),
            seam(
                EntityKind::Edge,
                base.clone(),
                member(3, face(5, 3), &[]),
                &[],
            ),
        ];
        let now = vec![seam(
            EntityKind::Edge,
            base.clone(),
            wall(&[rank(0, 3)]),
            &[],
        )];
        assert_eq!(
            group_cutters(&table(prior), &table(now), &base, true, false),
            read(vec![member(3, face(5, 3), &[])], vec![])
        );
    }

    /// A union group whose base is a seam line an earlier fold step
    /// minted is read like the pair's `FromA(Seam ..)`: the later step's
    /// seam vertices on that line, and a piece of it, name its cutters.
    #[test]
    fn a_union_seam_line_group_is_read() {
        let line = seam(
            EntityKind::Edge,
            member(1, top(), &[]),
            member(3, face(5, 1), &[]),
            &[],
        );
        let vertex = |on: StableName, cutter: StableName| seam(EntityKind::Vertex, on, cutter, &[]);
        let mut piece = line.clone();
        piece.path.push(rank(1, 2));
        let cutter = |seg| member(4, face(6, seg), &[]);
        let prior = vec![vertex(line.clone(), cutter(0)), vertex(cutter(1), piece)];
        let now = vec![vertex(line.clone(), cutter(0))];
        assert_eq!(
            group_cutters(&table(prior), &table(now), &line, true, false),
            read(vec![cutter(1)], vec![])
        );
    }
}

#[cfg(test)]
mod walk_tests {
    //! [`walk_names`] visits every embedded name, depth first in path
    //! order, from its own stack.
    #![allow(clippy::expect_used)]

    use super::*;
    use crate::names::{CapEnd, NameRef};

    fn leaf(node: u64) -> StableName {
        StableName {
            kind: EntityKind::Face,
            node: RecipeNodeId(node),
            path: vec![RoleSeg::Cap(CapEnd::End)],
        }
    }

    /// `inner` under a segment holding it and a partner beside it.
    fn over(inner: StableName, node: u64) -> StableName {
        StableName {
            kind: EntityKind::Face,
            node: RecipeNodeId(node),
            path: vec![
                RoleSeg::FromA(NameRef::new(inner)),
                RoleSeg::Fragment(Qualifier::Borders(vec![leaf(node + 1000)])),
            ],
        }
    }

    #[test]
    fn a_walk_visits_depth_first_in_path_order() {
        let name = over(over(leaf(1), 2), 3);
        let mut seen = Vec::new();
        walk_names(&name, Partners::Include, &mut |n| seen.push(n.node.0));
        assert_eq!(seen, [2, 1, 1002, 1003], "partners included");
        seen.clear();
        walk_names(&name, Partners::Skip, &mut |n| seen.push(n.node.0));
        assert_eq!(seen, [2, 1], "partners skipped");
    }

    #[test]
    fn a_walk_over_a_name_nested_past_every_stack_runs_on_the_smallest_stack() {
        const DEEP: u64 = 20_000;
        let seen = test_utils::own_thread::on_the_smallest_stack(|| {
            let name = (2..DEEP + 2).fold(leaf(1), over);
            let mut seen = 0usize;
            walk_names(&name, Partners::Include, &mut |_| seen += 1);
            seen
        });
        assert_eq!(seen, 2 * DEEP as usize, "every level's operand and partner");
    }
}
