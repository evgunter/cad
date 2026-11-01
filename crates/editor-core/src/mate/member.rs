//! **The member vocabulary** — what a mate side resolves to, and the
//! static offset the resolution derives (A11's member rule).
//!
//! One walk answers both questions. It runs from the body a side's
//! selection reads down the reads to its name's head, through the
//! families whose member the name says and the unions whose operand it
//! says, and it yields the MEMBER (the identity the solve keys pairs
//! by) together with the CHAIN of families it passed.
//!
//! Structural throughout: no expression is evaluated in the walk, so
//! the member a side resolves to never depends on a slot value. The offset —
//! which is arithmetic, not admission — does evaluate, in the solve's
//! one environment at its scalar ([`super::solve::solve_document`]).

use crate::doc::Doc;
use crate::names::{RoleSeg, VerbatimEdge};
use crate::node::{Node, RecipeNodeId};

/// **The member a mate reference resolves to** (A11's member
/// vocabulary): a live `InstantiatePart`, `Place` or construction,
/// reached from the reference's operand through the nodes that carry a
/// body to it without renaming it — any number of `Union`s, each of
/// which the name qualifies `From`, keyed by the member's read — and
/// at each family the walk passes, the member of it the name says
/// (`Member { at, of }`, D10 `[repetition]`).
///
/// A member is its instance and the family members the walk passed on
/// the way down: two references to two members of one family stand on
/// two members, and two references through ONE family member spelled
/// two ways — read at a placement, and at a union above it naming that
/// member — are one member, and their mates fold.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Member {
    /// The instance this member stands on.
    pub instance: RecipeNodeId,
    /// **The family members the walk passed**, OUTERMOST first, each
    /// at the index values the name says. Empty for a reference read
    /// through no family.
    pub chain: Vec<Placing>,
}

/// One family member on a member's chain.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Placing {
    /// The family the operation `family` defines, at the index values
    /// the reference's name says.
    Member {
        /// The family's operation.
        family: RecipeNodeId,
        /// The member's index values.
        at: crate::names::MemberAt,
    },
}

impl Placing {
    /// The recipe node this placing is.
    #[must_use]
    pub fn node(&self) -> RecipeNodeId {
        match self {
            Self::Member { family, .. } => *family,
        }
    }
}

/// **The member key, written out.** Its order, every node compared by
/// id (which is document order: an id's mint ordinal leads it), is
/// stated rather than derived, so whatever orders members orders them
/// by recipe structure: `(instance, chain)`, the chain compared
/// lexicographically, outermost first.
impl Ord for Member {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        (self.instance, &self.chain).cmp(&(other.instance, &other.chain))
    }
}

impl PartialOrd for Member {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

/// What the member walk yields: the member, and the reference's name
/// as the member's instance names it, one `Member` qualifier off per
/// family on the member's chain.
#[derive(Debug, Clone)]
pub(super) struct Walk {
    pub(super) member: Member,
    /// The name inside every `Member` qualifier the walk took off,
    /// headed at the member's instance: a copy's SOURCE's name.
    placed: crate::names::StableName,
}

impl Walk {
    /// **The face of the member's source the reference names** — the
    /// name the walk reached at the member, which a copy carries
    /// verbatim from its source, in the source's own coordinates.
    ///
    /// `None` when that name is no face's.
    pub(super) fn part_face(&self) -> Option<crate::FaceName> {
        crate::FaceName::new(self.placed.clone()).ok()
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
pub(super) fn walk<P: crate::ProfilePayload>(
    doc: &Doc<P>,
    r: &crate::node::SitedFace,
) -> Result<Walk, RecipeNodeId> {
    let mut at = r.at;
    let mut name: &crate::names::StableName = &r.name;
    let mut chain: Vec<Placing> = Vec::new();
    loop {
        // A family's member: the name must SAY which member, and the
        // walk continues at the same operation under the name inside
        // the qualifier, which is the member's own.
        if name.node == at
            && let Some(RoleSeg::Member { at: values, of }) = name.path.first()
        {
            // The qualifier keys every index the family is over, in
            // order: one that keys fewer names no member.
            let keys: Vec<crate::VarId> = values.iter().map(|&(index, _)| index).collect();
            if doc.node_reach(at).chain() != Some(keys.as_slice()) {
                return Err(at);
            }
            chain.push(Placing::Member {
                family: at,
                at: values.clone(),
            });
            name = of;
            continue;
        }
        // A placement's copy, an instance's or a `Place`'s, carries its
        // source's names verbatim: the member the solve poses, whoever
        // minted the name.
        if matches!(
            doc.node(at),
            Some(Node::InstantiatePart { .. } | Node::Place { .. })
        ) {
            return Ok(Walk {
                member: Member {
                    instance: at,
                    chain,
                },
                placed: name.clone(),
            });
        }
        if at != name.node {
            // Not the head yet: no name-carrying edge
            // ([`crate::names::verbatim_edge`]) projects this body. A
            // split's intact pass-through or a mirror's image is a
            // different body, not this one placed. (A union is the head
            // of every name it carries, so the walk meets it below, at
            // the head.)
            match doc.node(at).and_then(crate::names::verbatim_edge) {
                Some(
                    VerbatimEdge::Intact { .. }
                    | VerbatimEdge::Reflected { .. }
                    | VerbatimEdge::Copied { .. },
                )
                | None => {
                    return Err(at);
                }
            }
        }
        match doc.node(at) {
            // A union's member: the name must SAY which member, and the
            // walk continues at that member under the name inside the
            // qualifier. A union moves nothing, so it is no placer: a
            // reference read here and one read at the member it names
            // stand on one member.
            Some(Node::Union { members, .. } | Node::Assemble { members, .. }) => {
                let [RoleSeg::From { read, of }] = name.path.as_slice() else {
                    return Err(at);
                };
                if !members.reads().any(|m| m.read == *read) {
                    return Err(at);
                }
                name = of;
                at = doc.operation_of(*read).ok_or(at)?;
            }
            // A construction, the root of its own space (D10): at its
            // own identity, so a mate reads it as a body of the space
            // its copy joins.
            Some(node)
                if !matches!(
                    node,
                    Node::World | Node::Measure { .. } | Node::Assertion { .. }
                ) =>
            {
                return Ok(Walk {
                    member: Member {
                        instance: at,
                        chain,
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
/// live `InstantiatePart`, `Place` or construction. Between them it
/// admits any number of `Union`s, each of which must carry a `From`
/// qualifier keyed by one of its member reads, and at each family, the
/// `Member` qualifier naming which member ([`Member::chain`]).
///
/// Structural only — no expression is evaluated here, so the member
/// never depends on a slot value: admission reads the NAME.
///
/// Outside the vocabulary: a non-instance head; a union whose name
/// carries no `From` qualifier keyed by one of its member reads;
/// anything the walk meets that places no body of its own — a boolean,
/// a split, a head the walk never reaches at all; and a side that is no
/// live one-face selection ([`head_of`]).
///
/// [`crate::refactor::split`]'s interface-crossing collector is one of
/// those gates: a collector admitting a reference the member walk
/// does not admit would mint a record for a mate that never solved,
/// which is what AQ8 option (b) SKIP refuses (`ASSEMBLY.md`'s AQ8
/// clause).
pub fn member_of<P: crate::ProfilePayload>(
    doc: &Doc<P>,
    head: &crate::node::SitedFace,
) -> Option<Member> {
    walk(doc, head).ok().map(|w| w.member)
}

/// **The member a side resolves to, and the side's name as that
/// member's instance names it** — [`member_of`]'s walk, with the name
/// it reached at the instance: inside one `Member` qualifier per family
/// the walk passed ([`Member::chain`]) and one `From` per union, so a
/// copy answers its SOURCE's name. That name is a row of the instance's
/// own product table.
///
/// `None` exactly where [`member_of`] answers `None`.
pub fn member_reading<P: crate::ProfilePayload>(
    doc: &Doc<P>,
    head: &crate::node::SitedFace,
) -> Option<(Member, crate::names::StableName)> {
    walk(doc, head).ok().map(|w| (w.member, w.placed))
}

/// **The face of the member's source a side names** — the side walked
/// to its member, and the name the walk reached there, which a copy
/// carries verbatim, in the source's own coordinates.
///
/// `None` when the side is outside A11's member vocabulary
/// ([`member_of`] answers `None`), or when that name is no face's.
pub fn head_face<P: crate::ProfilePayload>(
    doc: &Doc<P>,
    head: &crate::node::SitedFace,
) -> Option<crate::FaceName> {
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
