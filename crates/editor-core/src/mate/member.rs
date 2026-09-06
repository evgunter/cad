//! **The member vocabulary** — what a mate reference resolves to, and
//! the static offset the resolution derives (A11's member rule, A12's
//! reading edges).
//!
//! One walk answers both questions. It runs from a reference's OPERAND
//! down the consuming edges to its name's head, through the nodes that
//! place a body without renaming it, and it yields the MEMBER (the
//! identity the solve keys pairs by) together with the CHAIN of
//! pose-bearing nodes whose maps compose onto the minting instance's
//! pose.
//!
//! Structural throughout: no expression is evaluated in the walk, so
//! the cluster partition never depends on a slot value. The offset —
//! which is arithmetic, not admission — does evaluate, at the
//! document's own parameter bindings.

use geom_core::linalg::{Affine3, Point3, Vec3};
use geom_core::predicate::Band;

use super::{MateFault, MateSide};
use crate::doc::Doc;
use crate::eval::SteppedOperands;
use crate::names::RoleSeg;
use crate::node::{Datum, Node, PartSelect, PatternKind, RecipeNodeId};

/// **The member a mate reference resolves to** (A11's member
/// vocabulary): a live `InstantiatePart`, reached from the
/// reference's operand through the nodes that place a body without
/// renaming it — any number of `Transform`s and `Part` instance
/// selections, and any number of `Pattern` levels, each of which the
/// name qualifies `Instance(i)`.
///
/// A member is more than its cluster-graph vertex. Two references
/// that reach one instance through DIFFERENT placings relate the same
/// pair of instances through different static offsets, so what stands
/// between the reference and the instance is part of the member's
/// identity — it is what makes a second such mate close a LOOP
/// (non-tree, declaring) instead of folding into the first mate's
/// pair. Two kinds of placing say so: the pattern COPY CHAIN, and the
/// OPERAND the reference was read at.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Member {
    /// The cluster-graph vertex this member stands on: the head
    /// instance itself, or — for a pattern-placed member — the
    /// innermost pattern's INPUT instance, whose pose every copy at
    /// every level is derived from. This is the edge end A9/A11's
    /// partitions see, and why a mate to `Instance(i)` joins the other
    /// member into the pattern's cluster.
    pub instance: RecipeNodeId,
    /// **Which copy this member is, at every level**: one
    /// `(pattern node, structural index)` per pattern the walk
    /// consumed, OUTERMOST first, empty for a plain instance head.
    ///
    /// A chain rather than one level because the argument that puts a
    /// copy in the member's identity holds at each of them: two mates
    /// onto sibling copies of an INNER pattern under one outer copy
    /// relate different bodies through different static offsets, so
    /// they are different pairs and the second closes a loop.
    pub copy: Vec<(RecipeNodeId, u32)>,
    /// The OPERAND the reference was read at: the node whose geometry
    /// the mate speaks about. Two references to one instance through
    /// two different transforms are two members over one instance, so
    /// they key `by_pair` as different pairs and the second mate
    /// closes a loop rather than folding into the first.
    pub at: RecipeNodeId,
}

/// **The member key, written out.** `Member` is the `BTreeMap` key
/// `by_pair` and `edge_of` are built on and the order the spanning
/// tree picks its edges by, so the ordering is stated rather than
/// derived: `(instance, copy, at)`, with the OPERAND last and the copy
/// chain compared lexicographically.
///
/// Last is load-bearing. In a document whose every reference is read
/// at its own mint the operand is a function of the other two fields,
/// so this order is the pre-operand `(instance, copy)` order refined
/// and no such document's pair set, spanning tree or solve moves. A
/// derive would tie that guarantee to the order the fields happen to
/// be written in, where an edit that reads as cosmetic could change
/// which mate a cluster takes as its tree edge.
///
/// The chain keeps that guarantee one level further out. A document
/// whose members are at most one copy deep has chains of length zero
/// or one, and a lexicographic order over those is exactly the order
/// `Option<(node, index)>` gave: empty before non-empty, and two
/// non-empty chains by their single pair. So no such document's
/// spanning tree moves either.
impl Ord for Member {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        (self.instance, &self.copy, self.at).cmp(&(other.instance, &other.copy, other.at))
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
        /// The `Node::Part` the walk reached this pattern through,
        /// when one stands directly above it — the node whose own
        /// index expression must agree with `i`.
        ///
        /// A `Part` carries no map and is no placer of its own; it is
        /// recorded HERE because the only thing it can disagree with
        /// is the copy this pattern's name segment names, and a check
        /// wants both halves in one place.
        part: Option<RecipeNodeId>,
    },
    /// A transform: the one production node that moves a body and
    /// mints no name.
    Transform(RecipeNodeId),
}

impl Placer {
    /// The recipe node this placer is — what a refusal names.
    fn node(self) -> RecipeNodeId {
        match self {
            Self::Pattern { node, .. } | Self::Transform(node) => node,
        }
    }
}

/// What the member walk yields: the member, and the pose-bearing
/// nodes between the operand and the minting instance, OUTERMOST
/// first (the order the geometry composes them in, so the fold below
/// reads left to right).
///
/// The member's copy chain is DERIVED from `chain` — it is the
/// patterns in it, in the same order — so the two never disagree
/// about which copy the reference names.
#[derive(Debug, Clone)]
pub(super) struct Walk {
    pub(super) member: Member,
    chain: Vec<Placer>,
}

/// **The walk from a reference's operand down to its name's head** —
/// the single traversal admission and the static offset share.
///
/// Structural only: no expression is evaluated here (the vocabulary
/// is decided on node kinds and name segments alone), so the cluster
/// partition never depends on a slot value.
///
/// # Errors
///
/// The node the walk STOPPED at: the one that is neither a
/// pass-through it can continue through nor a live instance it can
/// stand a member on. That is the node a refusal names, and it is not
/// in general the reference's own head — a stranded operand stops the
/// walk before the head is ever reached.
pub(super) fn walk<P>(doc: &Doc<P>, r: &crate::node::SitedRef) -> Result<Walk, RecipeNodeId> {
    let mut at = r.at;
    let mut name = &r.name;
    let mut chain: Vec<Placer> = Vec::new();
    // The `Part` the walk last passed with nothing pose-bearing since
    // — the one standing DIRECTLY above whatever node comes next, and
    // so the only one whose index expression a pattern's name segment
    // can be checked against.
    let mut part: Option<RecipeNodeId> = None;
    loop {
        if at != name.node {
            // Not the head yet: only a node that places or projects
            // this body without renaming it may stand between an
            // operand and the material it speaks about. A transform
            // moves the body and contributes no `RolePath` segment; a
            // `Part` selecting an instance moves nothing at all and
            // carries every name VERBATIM. Anything else — a boolean,
            // a union, a split, a `Part` naming a split HALF — is a
            // different body, not this one placed.
            match doc.node(at) {
                Some(Node::Transform { input, .. }) => {
                    chain.push(Placer::Transform(at));
                    part = None;
                    at = *input;
                }
                Some(Node::Part {
                    of,
                    select: PartSelect::Instance(_),
                }) => {
                    // Pose-neutral and identity-transparent: no
                    // placer, no name segment, and — because the
                    // index it carries is an EXPRESSION — nothing
                    // evaluated. The name is the authority on which
                    // copy; this node is checked against it where the
                    // offset already evaluates ([`derived_offset`]).
                    part = Some(at);
                    at = *of;
                }
                _ => return Err(at),
            }
            continue;
        }
        match doc.node(at) {
            Some(Node::InstantiatePart { .. }) => {
                return Ok(Walk {
                    member: Member {
                        instance: at,
                        copy: chain
                            .iter()
                            .filter_map(|p| match *p {
                                Placer::Pattern { node, i, .. } => Some((node, i)),
                                Placer::Transform(_) => None,
                            })
                            .collect(),
                        at: r.at,
                    },
                    chain,
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
                chain.push(Placer::Pattern {
                    node: at,
                    i: *i,
                    part: part.take(),
                });
                name = of;
                at = *input;
            }
            _ => return Err(at),
        }
    }
}

/// **The member a reference resolves to**, or `None` for a reference
/// outside A11's member vocabulary.
///
/// This is the vocabulary's one home — the admission rule the solve
/// reads and any authoring door must gate on, so a door cannot admit
/// a reference the solve will refuse (or refuse one it would place).
///
/// The rule is a WALK, from the operand `r.at` down the consuming
/// edges to `r.name`'s head, which must be a live `InstantiatePart`.
/// Between them it admits exactly the nodes that place a body without
/// renaming it: any number of `Transform`s, any number of `Part`
/// nodes selecting an `Instance`, and any number of `Pattern` levels,
/// each of which must carry its `Instance(i)` qualifier in the name.
/// A nested copy is a member like any other; its identity carries the
/// whole chain of copies ([`Member::copy`]).
///
/// Structural only — no expression is evaluated here, so the cluster
/// partition never depends on a slot value. That includes a `Part`'s
/// own index, which is an expression in a structural slot: admission
/// reads the NAME, and the offset checks the node against it.
///
/// Outside the vocabulary: a non-instance head; a pattern whose name
/// carries no `Instance(i)` qualifier; anything the walk meets that
/// places no body of its own — a boolean, a union, a split, a `Part`
/// naming a split HALF (which is a different body, not this one
/// placed), a head the walk never reaches at all.
///
/// [`crate::refactor::split`]'s interface-crossing collector is one of
/// those gates: a collector admitting a reference the cluster graph
/// does not weld would mint a record for a mate that never solved,
/// which is what AQ8 option (b) SKIP refuses (`ASSEMBLY.md`'s AQ8
/// clause).
pub fn member_of<P>(doc: &Doc<P>, r: &crate::node::SitedRef) -> Option<Member> {
    walk(doc, r).ok().map(|w| w.member)
}

/// **[`member_of`] for a MATE's reference**: the walk, with the mate
/// and side that attribute its refusal (N5).
///
/// The one door the solve reads a reference through — the member for
/// the pair keying and the partitions, the chain for the offset. A
/// second door answering only the member would be a second name for
/// one walk, and the caller that wanted both would take whichever it
/// remembered.
///
/// # Errors
///
/// [`MateFault::DanglingHead`] naming the node the WALK STOPPED AT,
/// which is where the reference stopped resolving: a stranded
/// operand, or the first node the chain met that no member stands on.
/// Naming the reference's own head instead would attribute the
/// refusal to a node that is often perfectly live and perfectly fine.
pub(super) fn walk_of<P>(
    doc: &Doc<P>,
    mate: RecipeNodeId,
    side: MateSide,
    r: &crate::node::SitedRef,
) -> Result<Walk, MateFault> {
    walk(doc, r).map_err(|head| MateFault::DanglingHead { mate, side, head })
}

/// **The per-reference checks that need a number** — run once per
/// reference of every live mate, at the site the solve walks it.
///
/// The walk is structural and evaluates nothing, so the cluster
/// partition never depends on a slot value. Two questions about a
/// reference are not structural, and both compare the NAME against an
/// evaluated count:
///
/// - the copy the name says must exist — its structural index against
///   the pattern's evaluated count;
/// - a `Part` standing directly above a pattern says which copy the
///   body below it IS, and the name says which copy the mate is
///   about. A document where they disagree would be PLACED by the
///   name and GATHERED by the `Part`, so the solve refuses rather
///   than choosing.
///
/// **They live here, per reference, and not in the offset.** The
/// offset is derived only for a pair the spanning tree takes as an
/// edge, and only for that pair's first mate — so a check sited there
/// runs on some references and not others, and a DECLARING mate whose
/// `Part` names another copy would be silently green with its body
/// somewhere else. Every mate the solve reads gets both checks, and a
/// reference that fails either refuses typed at that reading, exactly
/// as a dangling head does.
///
/// # Errors
///
/// [`MateFault::PartSelectsAnotherCopy`] for the disagreement, naming
/// both indices; [`MateFault::DanglingHead`] at the pattern for a copy
/// index at or beyond the count, and for a count or a `Part` index
/// whose expression does not evaluate at the document's own parameter
/// bindings.
pub(super) fn check_reference<P>(
    doc: &Doc<P>,
    mate: RecipeNodeId,
    side: MateSide,
    w: &Walk,
) -> Result<(), MateFault> {
    let env = doc.param_env::<f64>();
    for placer in &w.chain {
        let Placer::Pattern { node, i, part } = *placer else {
            continue;
        };
        let dangling = || MateFault::DanglingHead {
            mate,
            side,
            head: node,
        };
        let Some(Node::Pattern { count, .. }) = doc.node(node) else {
            return Err(dangling());
        };
        let n = crate::expr::eval_count(count, &env).map_err(|_| dangling())?;
        if i64::from(i) >= n {
            return Err(dangling());
        }
        let Some(part) = part else {
            continue;
        };
        let Some(Node::Part {
            select: PartSelect::Instance(index),
            ..
        }) = doc.node(part)
        else {
            return Err(dangling());
        };
        let selected = crate::expr::eval_count(index, &env).map_err(|_| dangling())?;
        if selected != i64::from(i) {
            return Err(MateFault::PartSelectsAnotherCopy {
                mate,
                side,
                part,
                named: i,
                selected,
            });
        }
    }
    Ok(())
}

/// **The reference's derived offset**: the rigid map every
/// pose-bearing node BETWEEN the operand and the minting instance
/// composes onto that instance's placement, in evaluation order — the
/// map nearest the instance applied first, so a pattern `M(i)` over a
/// transform `T` yields `M(i) ∘ T`.
///
/// The two placers and where their math lives:
///
/// - a pattern contributes [`crate::eval::stepped_rule_map`] at the
///   named index — THE evaluation's own stepped rule, fed the
///   pattern's authored slots evaluated at the document's own
///   parameter bindings;
/// - a transform contributes [`crate::eval::transform_map`] — the
///   same construction `wire_transform` places the body by, fed the
///   node's own expressions at those same bindings.
///
/// A `Part` contributes nothing at all — it selects a body, it does
/// not move one. Whether it agrees with the name, and whether the
/// named copy exists, are [`check_reference`]'s: they are facts about
/// a REFERENCE, and this door sees only the references of the pairs
/// the spanning tree took. Arithmetic here, admission and agreement
/// there.
///
/// Both are document-coordinate maps and both are LEFT-composed, for
/// the reason `wire_transform` and `wire_pattern` compose them
/// outside the placement.
///
/// `None` is the identity: an empty chain, or a chain whose every
/// placer is itself the identity (copy 0's map is the identity by the
/// stepped rule's own construction). Kept as absence, so a document
/// with no transform and no pattern composes nothing and its solve
/// stays bit-for-bit what it was.
///
/// The offset is STATIC: nothing here depends on any solved pose,
/// which is how a mate through a placer can never create per-instance
/// freedom — the placed body rides its instance wherever the solve
/// puts it.
///
/// # Errors
///
/// A chain whose derived pose does not exist refuses
/// [`MateFault::DanglingHead`] — a rule or a transform whose slots do
/// not evaluate, a degenerate or non-finite direction, an
/// explicit-rule pattern (whose count spelling the pattern node
/// itself refuses). This door's job is to refuse rather than guess a
/// pose. An in-band direction-norm decision escalates
/// [`MateFault::Indeterminate`], as every decided predicate here
/// does.
///
/// The copy index is NOT re-checked against the count here.
/// [`check_reference`] has already compared them for every reference
/// of every live mate, so by the time a pose is composed the index is
/// in range by construction — and a second guard would be a branch no
/// document can reach, which is a claim about the code rather than
/// about the kernel.
///
/// **One length decision, two funnel names, ratified.** A circular
/// rule's direction is a DATUM's axis direction, and this derivation
/// re-derives it from the recipe expressions rather than reading the
/// evaluated `DatumValue` whose `UnitVec3` normalized the same
/// triple. So one datum direction is decided under two predicate
/// names depending on which road reaches it — same body, same
/// refusal shape, different name in the K census. That split is
/// RATIFIED, not tolerated, and the argument is SEAT-DN's, not this
/// module's: `docs/DOC-LEDGER.md`'s `work/seat/
/// direction-normalization-two-doors-one-home` entry and the
/// `decide_unit_direction` seat it closed on.
///
/// **The direction refusals say less than they know, and the
/// difference is not recoverable elsewhere.** A rule whose direction
/// has zero or non-finite length is announced as a dangling head for
/// a reference that resolves; the node that could name the length
/// does not, because a mate fault poisons the document and that node
/// evaluates to `Poisoned` rather than to its own
/// `DegenerateDirection`/`NonFiniteDirection`. Carrying the
/// evaluation layer's typed refusal into [`MateFault`] instead is
/// proposed to this module's owner, filed as
/// `mate-dangling-head-is-a-catch-all-that-reports-a-false-cause`.
pub(super) fn derived_offset<P>(
    doc: &Doc<P>,
    mate: RecipeNodeId,
    side: MateSide,
    w: &Walk,
    band: Band,
) -> Result<Option<Affine3<f64>>, Box<MateFault>> {
    let env = doc.param_env::<f64>();
    let mut composed: Option<Affine3<f64>> = None;
    for placer in &w.chain {
        let head = placer.node();
        let dangling = || Box::new(MateFault::DanglingHead { mate, side, head });
        let scalar = |e: &crate::expr::Expr| crate::expr::eval(e, &env).map_err(|_| dangling());
        let triple = |es: &[crate::expr::Expr; 3]| -> Result<Vec3<f64>, Box<MateFault>> {
            Ok(Vec3::new(scalar(&es[0])?, scalar(&es[1])?, scalar(&es[2])?))
        };
        // Every direction here is normalized through the evaluation
        // layer's own decided door — never a raw comparison, never a
        // silent zero direction.
        let unit = |v: Vec3<f64>, role: &'static str| -> Result<Vec3<f64>, Box<MateFault>> {
            crate::eval::unit_direction(v, role, band).map_err(|e| match e {
                crate::eval::NodeErrorKind::Escalated { source, .. } => {
                    Box::new(MateFault::Indeterminate {
                        mate,
                        diag: Box::new(source),
                    })
                }
                _ => dangling(),
            })
        };
        let map = match *placer {
            Placer::Pattern { node, i, .. } => {
                let Some(Node::Pattern { kind, .. }) = doc.node(node) else {
                    return Err(dangling());
                };
                if i == 0 {
                    // Copy 0's map is the identity by the stepped
                    // rule's own construction; composing it would be
                    // a no-op that costs bits.
                    continue;
                }
                let ops = match kind {
                    PatternKind::Linear { direction, spacing } => SteppedOperands::Linear {
                        direction: unit(triple(direction)?, crate::eval::PATTERN_DIRECTION_ROLE)?,
                        spacing: scalar(spacing)?,
                    },
                    PatternKind::Circular { axis, step } => {
                        let Some(Node::Datum(Datum::Axis { origin, direction })) = doc.node(*axis)
                        else {
                            return Err(dangling());
                        };
                        SteppedOperands::Circular {
                            origin: Point3::origin() + triple(origin)?,
                            dir: unit(triple(direction)?, crate::eval::DATUM_AXIS_ROLE)?,
                            step: scalar(step)?,
                        }
                    }
                    // The list-rule pattern's count has two spellings,
                    // which the pattern node itself refuses; no copy
                    // of it has a derived pose to stand a member on.
                    PatternKind::Explicit(_) => return Err(dangling()),
                };
                crate::eval::stepped_rule_map(&ops, i64::from(i))
            }
            Placer::Transform(node) => {
                let Some(Node::Transform {
                    translation,
                    rotation_axis,
                    rotation_angle,
                    ..
                }) = doc.node(node)
                else {
                    return Err(dangling());
                };
                crate::eval::transform_map(
                    triple(translation)?,
                    unit(triple(rotation_axis)?, crate::eval::TRANSFORM_AXIS_ROLE)?,
                    scalar(rotation_angle)?,
                )
            }
        };
        composed = Some(match composed {
            None => map,
            Some(held) => held * map,
        });
    }
    Ok(composed)
}
