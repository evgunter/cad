//! **The member vocabulary** — what a mate side resolves to, and the
//! static offset the resolution derives (A11's member rule).
//!
//! One walk answers both questions. It runs from the body a side's
//! selection reads down the reads to its name's head, through the nodes that
//! place a body without renaming it and the unions whose member the
//! name says, and it yields the MEMBER (the
//! identity the solve keys pairs by) together with the CHAIN of
//! pose-bearing nodes whose maps compose onto the minting instance's
//! pose.
//!
//! Structural throughout: no expression is evaluated in the walk, so
//! the member a side resolves to never depends on a slot value. The offset —
//! which is arithmetic, not admission — does evaluate, in the solve's
//! one environment at its scalar ([`super::solve::solve_document`]).

use crate::doc::Doc;
use crate::names::{RoleSeg, VerbatimEdge};
use crate::node::{Node, PartSelect, RecipeNodeId};

/// **The member a mate reference resolves to** (A11's member
/// vocabulary): a live `InstantiatePart`, reached from the
/// reference's operand through the nodes that place a body without
/// renaming it — any number of `Transform`s and `Part` instance
/// selections, any number of `Pattern` levels, each of which the name
/// qualifies `Instance(i)`, and any number of `Union`s, each of which
/// the name qualifies `From`, keyed by the member's read.
///
/// A member is its instance and the PLACEMENT the walk passed on the
/// way down: two references that reach one instance through different
/// placings relate the same pair of instances through different static
/// offsets, so they are different members, and a bundle holding a
/// mate on each refuses (`MateFault::TargetsDiffer`) instead of
/// folding them as one pair. Two references through ONE placement spelled two
/// ways — read at a transform, and at a union above it naming that
/// member; read at a `Part` selecting a copy, and at the pattern naming
/// that copy — are one member, and their mates fold.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Member {
    /// The instance this member stands on: the head instance itself,
    /// or — for a pattern-placed member — the innermost pattern's
    /// INPUT instance, whose pose every copy at every level is derived
    /// from.
    pub instance: RecipeNodeId,
    /// **The placing nodes the walk passed**, OUTERMOST first: each
    /// pattern with the copy the name says, and each transform. Empty
    /// for a reference read at its instance, or through nothing but
    /// `Part` selections and unions, which move nothing.
    pub chain: Vec<Placing>,
}

/// One placing node on a member's chain.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Placing {
    /// A pattern, at the structural index the reference's name says.
    Copy {
        /// The pattern node.
        pattern: RecipeNodeId,
        /// The copy index.
        index: u32,
    },
    /// A transform.
    Transform(RecipeNodeId),
}

impl Placing {
    /// The recipe node this placing is.
    #[must_use]
    pub fn node(self) -> RecipeNodeId {
        match self {
            Self::Copy { pattern, .. } => pattern,
            Self::Transform(node) => node,
        }
    }
}

impl Member {
    /// **Which copy this member is, at every level**: one
    /// `(pattern node, structural index)` per pattern on the chain,
    /// OUTERMOST first, empty for a member no pattern places.
    #[must_use]
    pub fn copy(&self) -> Vec<(RecipeNodeId, u32)> {
        self.chain
            .iter()
            .filter_map(|p| match *p {
                Placing::Copy { pattern, index } => Some((pattern, index)),
                Placing::Transform(_) => None,
            })
            .collect()
    }
}

/// **The member key, written out.** Its order, every node compared by
/// id (which is document order: an id's mint ordinal leads it), is
/// stated rather than derived, so whatever orders members orders them
/// by recipe structure: `(instance, copy, chain)` — the instance, then the copy
/// chain (patterns only, outermost first), then the whole placing
/// chain, each compared lexicographically. The copies lead so that a
/// copy chain alone decides between two members whenever it differs.
impl Ord for Member {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        (self.instance, self.copy(), &self.chain).cmp(&(other.instance, other.copy(), &other.chain))
    }
}

impl PartialOrd for Member {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

/// One pose-bearing node the member walk passed, with what it
/// contributes to the reference's static offset.
#[derive(Debug, Clone, Copy)]
enum Placer {
    /// A pattern, at the structural index the reference named.
    Pattern {
        /// The pattern node.
        node: RecipeNodeId,
        /// The copy index the NAME says.
        i: u32,
    },
    /// A transform: the one production node that moves a body and
    /// mints no name.
    Transform(RecipeNodeId),
}

/// The member's view of a placer: which copy or which transform, with
/// the walk's own bookkeeping (the `Part` a check reads) left behind.
impl From<Placer> for Placing {
    fn from(p: Placer) -> Self {
        match p {
            Placer::Pattern { node, i } => Self::Copy {
                pattern: node,
                index: i,
            },
            Placer::Transform(node) => Self::Transform(node),
        }
    }
}

/// What the member walk yields: the member, the pose-bearing nodes
/// between the operand and the minting instance, OUTERMOST first (the
/// order the geometry composes them in, so the fold below reads left
/// to right), and the reference's name as the member's instance names
/// it.
///
/// The member's copy chain is DERIVED from `chain` — it is the
/// patterns in it, in the same order — so the two never disagree
/// about which copy the reference names; and `placed` is the name the
/// same descent reached, one qualifier off per pattern in `chain`.
#[derive(Debug, Clone)]
pub(super) struct Walk {
    pub(super) member: Member,
    /// The name inside every `Instance(i)` qualifier the walk took
    /// off, headed at the member's instance: a copy's MASTER's name.
    placed: crate::names::StableName,
}

impl Walk {
    /// **The face of the member's PART the reference names** — the
    /// one `InPart` the member's instance puts round its part's names,
    /// taken off the name the walk reached there
    /// ([`crate::FaceName::part_local`]), in the part's own
    /// coordinates; a copy reads its master's face.
    ///
    /// `None` when the name at the instance is not one `InPart` round
    /// a face: a name an instance mints always is, so only a document
    /// written some other way answers `None`.
    pub(super) fn part_face(&self) -> Option<crate::FaceName> {
        crate::FaceName::part_local(&self.placed, self.member.instance)
    }
}

/// **The walk from a reference's operand down to its name's head** —
/// the single traversal admission and the static offset share.
///
/// Structural only: no expression is evaluated here (the vocabulary
/// is decided on node kinds and name segments alone), so the member a
/// reference resolves to never depends on a slot value.
///
/// # Errors
///
/// The node the walk STOPPED at: the one that is neither a
/// pass-through it can continue through nor a live instance it can
/// stand a member on. That is the node a refusal names, and it is not
/// in general the reference's own head — a stranded operand stops the
/// walk before the head is ever reached.
pub(super) fn walk<P>(doc: &Doc<P>, r: &crate::node::SitedFace) -> Result<Walk, RecipeNodeId> {
    let mut at = r.at;
    let mut name: &crate::names::StableName = &r.name;
    let mut chain: Vec<Placer> = Vec::new();
    loop {
        if at != name.node {
            // Not the head yet: only a name-carrying edge
            // ([`crate::names::verbatim_edge`]) that places or projects
            // this body may stand between an operand and the material
            // it speaks about. A whole edge moves the body and
            // contributes no `RolePath` segment; a `Part` selecting an
            // instance moves nothing at all and carries every name
            // VERBATIM. Anything else — a boolean, a split's intact
            // pass-through — is a different body, not this one placed. (A union is the
            // head of every name it carries, so the walk meets it
            // below, at the head.)
            match doc.node(at).and_then(crate::names::verbatim_edge) {
                // A whole edge is shape-preserving over the value —
                // body `k` in, body `k` out — so a `Part` above it
                // still selects body `k` of whatever stands below,
                // and is carried down to the pattern it checks against.
                Some(VerbatimEdge::Whole { input }) => {
                    chain.push(Placer::Transform(at));
                    at = doc.operation_of(input).ok_or(at)?;
                }
                Some(VerbatimEdge::Selected {
                    of,
                    select: PartSelect::Instance(_),
                }) => {
                    // Pose-neutral and identity-transparent: no
                    // placer, no name segment, and — because the
                    // index it carries is an EXPRESSION — nothing
                    // evaluated. The name is the authority on which
                    // copy.
                    at = doc.operation_of(of).ok_or(at)?;
                }
                Some(VerbatimEdge::Intact) | None => return Err(at),
            }
            continue;
        }
        match doc.node(at) {
            // A placement's copy, an instance's or a `Place`'s: the
            // member the solve poses.
            Some(Node::InstantiatePart { .. } | Node::Place { .. }) => {
                return Ok(Walk {
                    member: Member {
                        instance: at,
                        chain: chain.iter().copied().map(Placing::from).collect(),
                    },
                    placed: name.clone(),
                });
            }
            // A pattern's copy: the name must SAY which copy, and the
            // walk continues at the pattern's input under the name
            // inside the qualifier. Any number of levels, because a
            // member's identity carries the whole chain of copies.
            Some(Node::Pattern { input, .. }) => {
                let Some(RoleSeg::Instance { i, of }) = name.path.first() else {
                    return Err(at);
                };
                chain.push(Placer::Pattern { node: at, i: *i });
                name = of;
                at = doc.operation_of(*input).ok_or(at)?;
            }
            // A union's member: the name must SAY which member, and the
            // walk continues at that member under the name inside the
            // qualifier. A union moves nothing, so it is no placer: a
            // reference read here and one read at the member it names
            // stand on one member.
            Some(Node::Union { members, .. }) => {
                let [RoleSeg::From { read, of }] = name.path.as_slice() else {
                    return Err(at);
                };
                if !members.reads().any(|m| m.read == *read) {
                    return Err(at);
                }
                name = of;
                at = doc.operation_of(*read).ok_or(at)?;
            }
            // A construction of the document's one kind (the named
            // interim of INTENT stage 3 B): the world's kind, at the
            // identity, so a mate reads it as a body of the space its
            // copy joins.
            Some(node)
                if !matches!(
                    node,
                    Node::World | Node::Measure { .. } | Node::Assertion { .. }
                ) =>
            {
                return Ok(Walk {
                    member: Member {
                        instance: at,
                        chain: chain.iter().copied().map(Placing::from).collect(),
                    },
                    placed: name.clone(),
                });
            }
            _ => return Err(at),
        }
    }
}

/// **The member a mate side's head resolves to**, or `None` for a head
/// outside A11's member vocabulary. A side the document holds is read
/// to its head by [`head_of`]; an authoring door asks of the head it is
/// about to write.
///
/// This is the vocabulary's one home — the admission rule the solve
/// reads and any authoring door must gate on, so a door cannot admit
/// a reference the solve will refuse (or refuse one it would place).
///
/// The rule is a WALK, from the operation defining the selection's
/// body down its reads to the selected name's head, which must be a
/// live `InstantiatePart`.
/// Between them it admits exactly the nodes that place a body without
/// renaming it: any number of `Transform`s, any number of `Part`
/// nodes selecting an `Instance`, any number of `Pattern` levels,
/// each of which must carry its `Instance(i)` qualifier in the name,
/// and any number of `Union`s, each of which must carry a `From`
/// qualifier keyed by one of its member reads. A nested copy is
/// a member like any other; its identity carries the whole chain of
/// placings ([`Member::chain`]).
///
/// Structural only — no expression is evaluated here, so the member
/// never depends on a slot value. That includes a `Part`'s
/// own index, which is an expression in a structural slot: admission
/// reads the NAME, and the offset checks the node against it.
///
/// Outside the vocabulary: a non-instance head; a pattern whose name
/// carries no `Instance(i)` qualifier; a union whose name carries no
/// `From` qualifier keyed by one of its member reads; anything the walk
/// meets that places no body of its own — a boolean, a split, a `Part`
/// naming a split HALF (which is a different body, not this one
/// placed), a head the walk never reaches at all; and a side that is no
/// live one-face selection ([`head_of`]).
///
/// [`crate::refactor::split`]'s interface-crossing collector is one of
/// those gates: a collector admitting a reference the member walk
/// does not admit would mint a record for a mate that never solved,
/// which is what AQ8 option (b) SKIP refuses (`ASSEMBLY.md`'s AQ8
/// clause).
pub fn member_of<P>(doc: &Doc<P>, head: &crate::node::SitedFace) -> Option<Member> {
    walk(doc, head).ok().map(|w| w.member)
}

/// **The member a side resolves to, and the side's name as that
/// member's instance names it** — [`member_of`]'s walk, with the name
/// it reached at the instance: inside one `Instance(i)` qualifier per
/// pattern level the walk consumed ([`Member::copy`]) and one `From`
/// per union, so a copy answers its MASTER's name. That name is a row
/// of the instance's own product table.
///
/// `None` exactly where [`member_of`] answers `None`.
pub fn member_reading<P>(
    doc: &Doc<P>,
    head: &crate::node::SitedFace,
) -> Option<(Member, crate::names::StableName)> {
    walk(doc, head).ok().map(|w| (w.member, w.placed))
}

/// **The face of the member's PART a side names** — the side walked
/// to its member, then the instance's `InPart` taken off the name the
/// walk reached there ([`crate::FaceName::part_local`]), in the part's
/// own coordinates; a copy reads its MASTER's face.
///
/// `None` when the side is outside A11's member vocabulary
/// ([`member_of`] answers `None`), or when the name at the member's
/// instance is not one `InPart` round a face, which a name an
/// instance mints always is.
pub fn head_face<P>(doc: &Doc<P>, head: &crate::node::SitedFace) -> Option<crate::FaceName> {
    walk(doc, head).ok()?.part_face()
}

/// **A mate side's head, read off its selection** (D10): the node whose
/// output the selection's body read is, and the one face name the
/// selection holds — where the member walk starts.
///
/// `None` when `side` is no live selection of one face name in a body
/// a live operation defines: the selection, or the body it is read
/// in, was deleted, or it holds a name of another kind. Each is the
/// mate's evaluation's refusal in its own words (the read's
/// `UnresolvedRead`, the selection's `SelectKind`).
pub fn head_of<P>(doc: &Doc<P>, side: crate::VarId) -> Option<crate::node::SitedFace> {
    let select = doc.selection(side)?;
    let [name] = select.names.as_slice() else {
        return None;
    };
    let name = crate::FaceName::new(name.clone()).ok()?;
    Some(crate::node::SitedFace::new(
        doc.operation_of(select.body)?,
        name,
    ))
}
