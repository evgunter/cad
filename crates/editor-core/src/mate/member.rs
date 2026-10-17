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
/// mate on each refuses ([`MateFault::TargetsDiffer`]) instead of
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

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

    use super::*;
    use crate::edit::DocEdit;
    use crate::eval::{CancelToken, EvalOptions, NodeResult, evaluate};

    use crate::ident::DocumentId;
    use crate::program::{LoopProgram, ProfileProgram};
    use crate::test_support::{ang, len, scl, xy_frame};
    use crate::{ProfileDoc, RefusingReach};
    use geom_core::Tol;

    /// The hand-pushed ids are chosen, not minted; the dangling read
    /// names no live output. Their ordinals follow the 60 log entries
    /// `build` mints before them (its three inserts, every slot's
    /// variable and each insert's output): the nodes, then their
    /// outputs.
    const AXIS: RecipeNodeId = RecipeNodeId::new(61, 10);
    const FRAME2: RecipeNodeId = RecipeNodeId::new(62, 11);
    const T1: RecipeNodeId = RecipeNodeId::new(63, 12);
    const T2: RecipeNodeId = RecipeNodeId::new(64, 13);
    const PATTERN: RecipeNodeId = RecipeNodeId::new(65, 14);
    const MATE: crate::mate::MateRef = crate::mate::MateRef::new(
        RecipeNodeId::new(66, 50),
        crate::node::ConstraintId(crate::MintId::new(67, 51)),
    );
    const AXIS_OUT: crate::VarId = crate::VarId::new(67, 20);
    const FRAME2_OUT: crate::VarId = crate::VarId::new(68, 21);
    const T1_OUT: crate::VarId = crate::VarId::new(69, 22);
    const T2_OUT: crate::VarId = crate::VarId::new(70, 23);
    const PATTERN_OUT: crate::VarId = crate::VarId::new(71, 24);
    /// Minted and gone: the output of an operation since deleted.
    const DANGLING: crate::VarId = crate::VarId::new(72, 40);

    fn xf(input: crate::VarId) -> crate::AuthoredNode {
        Node::transform(
            input,
            crate::Step::Rigid {
                translation: [len(0.0), len(0.0), len(0.0)],
                axis: [scl(0.0), scl(0.0), scl(1.0)],
                angle: ang(0.0),
            },
        )
    }
    fn axis_datum_node() -> crate::AuthoredNode {
        Node::Datum(Datum::Axis {
            origin: [len(0.0), len(0.0), len(0.0)],
            direction: [scl(0.0), scl(0.0), scl(1.0)],
        })
    }

    /// Which node an input names.
    #[derive(Clone, Copy, Debug)]
    enum Src {
        Body,
        Axis,
        Frame,
        T1,
        T2,
        Dangling,
    }

    /// Through `apply`: a frame, a profile on it, an extrude — the
    /// BODY. Hand-pushed: `AXIS`, `FRAME2`, `T1 = xf(t1_in)`,
    /// `T2 = xf(T1)`, and `PATTERN`, circular over the body with
    /// `axis_operand` as its rule's axis.
    fn build(seed: &str, t1_in: Src, axis_operand: Src) -> (ProfileDoc, RecipeNodeId) {
        let ins = |doc: ProfileDoc, node: crate::AuthoredNode| {
            let a = crate::apply(
                &doc,
                &DocEdit::InsertNode {
                    node: Box::new(node),
                    fresh: Vec::new(),
                },
                Tol::witness(),
                &RefusingReach,
            )
            .expect("inserts");
            (a.doc, a.record.minted.unwrap())
        };
        let doc = ProfileDoc::empty(DocumentId::derive(seed), Tol::witness());
        let (doc, plane) = ins(doc, xy_frame());
        let (doc, profile) = ins(
            doc,
            Node::Profile(ProfileProgram {
                frame: plane.into(),
                loops: vec![
                    LoopProgram::polygon([(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)]).unwrap(),
                ],
                ids: Vec::new(),
            }),
        );
        let (mut doc, body) = ins(
            doc,
            Node::Extrude {
                profile: profile.into(),
                distance: len(1.0),
                side: crate::ExtrudeSide::Along,
            },
        );
        let body_out = doc.output(body, 0).expect("the body's output");
        let id = |s: Src| match s {
            Src::Body => body_out,
            Src::Axis => AXIS_OUT,
            Src::Frame => FRAME2_OUT,
            Src::T1 => T1_OUT,
            Src::T2 => T2_OUT,
            Src::Dangling => DANGLING,
        };
        // Stored first, so the variables their slots mint are logged
        // before the hand-pushed ids, whose ordinals come next.
        let stored: Vec<_> = [
            (AXIS, axis_datum_node()),
            (FRAME2, xy_frame()),
            (T1, xf(id(t1_in))),
            (T2, xf(T1_OUT)),
            (
                PATTERN,
                Node::Pattern {
                    input: body_out.into(),
                    count: crate::Formula::count(4),
                    kind: PatternKind::Circular {
                        axis: id(axis_operand).into(),
                        step: ang(0.5),
                    },
                },
            ),
        ]
        .into_iter()
        .map(|(id, node)| {
            // Each read is stored as chosen, a dangling one included:
            // the state no door writes is the one these rows need.
            let stored = node.try_map_slots(
                |p, f, r| crate::ProfilePayload::lower(p, f, r),
                &mut |e| crate::edit::lower_slot_into(&mut doc, e),
                &mut |_, read| match read {
                    crate::Operand::Var(var) => Ok(*var),
                    other => unreachable!("every row reads a variable by id, not {other:?}"),
                },
            );
            (id, stored.expect("a node the document can answer lowers"))
        })
        .collect();
        doc.nodes.extend(stored);
        doc.mint = doc.mint.clone().logged(
            [AXIS, FRAME2, T1, T2, PATTERN, MATE.placement]
                .map(crate::Minted::Node)
                .into_iter()
                .chain(
                    [AXIS_OUT, FRAME2_OUT, T1_OUT, T2_OUT, PATTERN_OUT, DANGLING]
                        .map(crate::Minted::Var),
                ),
        );
        for (out, kind, node) in [
            (AXIS_OUT, crate::VarKind::Axis, AXIS),
            (FRAME2_OUT, crate::VarKind::Frame, FRAME2),
            (T1_OUT, crate::VarKind::Body, T1),
            (T2_OUT, crate::VarKind::Body, T2),
            (PATTERN_OUT, crate::VarKind::Bodies, PATTERN),
        ] {
            doc.vars.insert(out, crate::Var::output(kind, node, 0));
        }
        (doc, body)
    }

    /// The walk a reference to copy 1 of the pattern records, `head`
    /// standing for both its name and the name it reached at the
    /// instance (the offset reads neither).
    fn copy_one(instance: RecipeNodeId, head: &crate::FaceName) -> Walk {
        Walk {
            member: Member {
                instance,
                chain: vec![Placing::Copy {
                    pattern: PATTERN,
                    index: 1,
                }],
            },
            chain: vec![Placer::Pattern {
                node: PATTERN,
                i: 1,
                part: None,
            }],
            head: head.clone(),
            placed: (**head).clone(),
        }
    }

    /// **The derivation road's refusal**: the placer it names and the
    /// kind it carries.
    fn derivation(doc: &ProfileDoc, instance: RecipeNodeId) -> (RecipeNodeId, NodeRefusal) {
        let env = doc.var_env::<f64>();
        let band = Band::linear(Tol::witness()).unwrap();
        let head = crate::FaceName::new(crate::names::StableName {
            kind: crate::names::EntityKind::Face,
            node: instance,
            path: Vec::new(),
        })
        .unwrap();
        let fault = derived_offset(
            doc,
            &env,
            MATE,
            MateSide::A,
            &copy_one(instance, &head),
            band,
        )
        .expect_err("the axis operand refuses");
        match *fault {
            MateFault::PlacerRefused {
                mate,
                side,
                placer,
                error,
                placer_row,
            } => {
                assert_eq!((mate, side), (MATE, MateSide::A));
                // The pattern is the chain's placer, poisoned through the
                // instance the fault reaches; any other seat is off the
                // chain and states the refusal on its own row.
                let silent = if placer == PATTERN {
                    PlacerRow::Silent
                } else {
                    PlacerRow::States
                };
                assert_eq!(placer_row, silent, "{placer:?}'s row");
                (placer, error)
            }
            other => panic!("expected PlacerRefused, got {other:?}"),
        }
    }

    /// **The evaluation road**, over the same document.
    fn evaluation(doc: &ProfileDoc) -> crate::eval::Evaluation<f64> {
        evaluate::<f64>(
            doc,
            None,
            &CancelToken::new(),
            &EvalOptions::default(),
            Tol::witness(),
        )
    }

    /// **The two kinds this module seats, compared structurally** —
    /// variant and every field — so a row that names one cannot pass
    /// on a rendering.
    fn same_refusal(a: &NodeErrorKind, b: &NodeErrorKind) -> bool {
        match (a, b) {
            (
                NodeErrorKind::MissingInput { input: x },
                NodeErrorKind::MissingInput { input: y },
            ) => x == y,
            (
                NodeErrorKind::UnresolvedRead { slot: xs, var: xv },
                NodeErrorKind::UnresolvedRead { slot: ys, var: yv },
            ) => xs == ys && xv == yv,
            (
                NodeErrorKind::WrongOperand {
                    input: xi,
                    expected: xe,
                    found: xf,
                },
                NodeErrorKind::WrongOperand {
                    input: yi,
                    expected: ye,
                    found: yf,
                },
            ) => xi == yi && xe == ye && xf == yf,
            _ => false,
        }
    }

    /// The node's own row in the evaluation fails with `kind`.
    fn fails_with(ev: &crate::eval::Evaluation<f64>, node: RecipeNodeId, kind: &NodeErrorKind) {
        match ev.result(node) {
            Some(NodeResult::Failed(err)) => {
                assert_eq!(err.node, node);
                assert!(
                    same_refusal(&err.kind, kind),
                    "{node:?}: the evaluation's own kind {:?} is the derivation's {kind:?}",
                    err.kind
                );
            }
            other => panic!("{node:?} fails on its own: {other:?}"),
        }
    }

    /// The node's row in the evaluation is poisoned through `through`.
    fn poisoned_through(
        ev: &crate::eval::Evaluation<f64>,
        node: RecipeNodeId,
        through: RecipeNodeId,
    ) {
        assert!(
            matches!(ev.result(node), Some(NodeResult::Poisoned { through: t }) if *t == through),
            "{node:?} is poisoned through {through:?}: {:?}",
            ev.result(node)
        );
    }

    fn is_live(ev: &crate::eval::Evaluation<f64>, node: RecipeNodeId) {
        assert!(
            matches!(ev.result(node), Some(NodeResult::Ok(_))),
            "{node:?} is live: {:?}",
            ev.result(node)
        );
    }

    /// **Both roads, one seat**: the derivation refuses at `placer`
    /// with a kind matching `expect`, and the evaluation fails the
    /// same node with the same kind.
    fn both_roads(
        label: &str,
        t1_in: Src,
        axis: Src,
        placer: RecipeNodeId,
        expect: impl FnOnce(&NodeErrorKind) -> bool,
    ) -> crate::eval::Evaluation<f64> {
        let (doc, body) = build(label, t1_in, axis);
        let (at, kind) = derivation(&doc, body);
        let kind = kind.kind();
        assert_eq!(
            at, placer,
            "{label}: seated at {placer:?}, carrying {kind:?}"
        );
        assert!(expect(kind), "{label}: the kind the row names: {kind:?}");
        let ev = evaluation(&doc);
        fails_with(&ev, placer, kind);
        ev
    }

    fn unresolved(slot: crate::OperandSlot, var: crate::VarId) -> impl Fn(&NodeErrorKind) -> bool {
        move |k| matches!(k, NodeErrorKind::UnresolvedRead { slot: s, var: v } if *s == slot && *v == var)
    }

    fn wrong(
        input: RecipeNodeId,
        expected: &'static str,
        found: &'static str,
    ) -> impl Fn(&NodeErrorKind) -> bool {
        move |k| {
            matches!(
                k,
                NodeErrorKind::WrongOperand { input: i, expected: e, found: f }
                    if *i == input && *e == expected && *f == found
            )
        }
    }

    /// **A dangling input of the axis transform is seated at that
    /// transform on both roads**, and the pattern is poisoned through
    /// it rather than refused with it.
    #[test]
    fn a_dangling_input_of_the_axis_transform_seats_at_that_transform_on_both_roads() {
        let ev = both_roads(
            "msolve7-t1-dangling",
            Src::Dangling,
            Src::T1,
            T1,
            unresolved(crate::OperandSlot::Input, DANGLING),
        );
        poisoned_through(&ev, PATTERN, T1);
    }

    /// **Two transforms down, the seat is the deeper one** — the
    /// transform whose input dangles, not the operand the pattern
    /// names.
    #[test]
    fn a_dangling_input_two_transforms_down_seats_at_the_deeper_transform_on_both_roads() {
        let ev = both_roads(
            "msolve7-t2-t1-dangling",
            Src::Dangling,
            Src::T2,
            T1,
            unresolved(crate::OperandSlot::Input, DANGLING),
        );
        poisoned_through(&ev, T2, T1);
        poisoned_through(&ev, PATTERN, T1);
    }

    /// **An axis operand that names no node is the pattern's own
    /// wiring** on both roads.
    #[test]
    fn a_missing_axis_operand_seats_at_the_pattern_on_both_roads() {
        both_roads(
            "msolve7-axis-missing",
            Src::Body,
            Src::Dangling,
            PATTERN,
            unresolved(crate::OperandSlot::Axis, DANGLING),
        );
    }

    /// **A frame datum as the axis is the wrong family, at the
    /// pattern**, in the evaluation's own words.
    #[test]
    fn a_frame_datum_as_the_axis_seats_at_the_pattern_on_both_roads() {
        both_roads(
            "msolve7-axis-frame",
            Src::Body,
            Src::Frame,
            PATTERN,
            wrong(
                FRAME2,
                crate::eval::phrase::DATUM_AXIS,
                crate::eval::family::DATUM,
            ),
        );
    }

    /// **A body as the axis is the wrong family, at the pattern.**
    #[test]
    fn a_body_as_the_axis_seats_at_the_pattern_on_both_roads() {
        let (doc, body) = build("msolve7-axis-body", Src::Body, Src::Body);
        let (at, kind) = derivation(&doc, body);
        let kind = kind.kind();
        assert_eq!(at, PATTERN);
        assert!(
            wrong(
                body,
                crate::eval::phrase::DATUM_AXIS,
                crate::eval::family::BODY
            )(kind),
            "{kind:?}"
        );
        fails_with(&evaluation(&doc), PATTERN, kind);
    }

    /// **A live transform over the body as the axis**: the transform
    /// evaluates, and the wrong family is the pattern's, read through
    /// the transform to the body below.
    #[test]
    fn a_transform_over_the_body_as_the_axis_seats_at_the_pattern_on_both_roads() {
        let ev = both_roads(
            "msolve7-axis-t1-body",
            Src::Body,
            Src::T1,
            PATTERN,
            wrong(
                T1,
                crate::eval::phrase::DATUM_AXIS,
                crate::eval::family::BODY,
            ),
        );
        is_live(&ev, T1);
    }

    /// The same through two live transforms.
    #[test]
    fn two_transforms_over_the_body_as_the_axis_seat_at_the_pattern_on_both_roads() {
        let ev = both_roads(
            "msolve7-axis-t2-t1-body",
            Src::Body,
            Src::T2,
            PATTERN,
            wrong(
                T2,
                crate::eval::phrase::DATUM_AXIS,
                crate::eval::family::BODY,
            ),
        );
        is_live(&ev, T1);
        is_live(&ev, T2);
    }

    /// **A transform over a DATUM as the axis is the TRANSFORM's own
    /// refusal on both roads** — a datum is not placeable, so the
    /// transform fails with `WrongOperand` in the placer's own words
    /// and the pattern is poisoned through it. The doors refuse this
    /// shape by kind (`SlotVarKind`), so the rows build it by hand;
    /// the derivation seats it where the evaluation does by inheriting
    /// the classifier's seat rather than reading the operand's family
    /// for itself.
    #[test]
    fn a_transform_over_a_datum_as_the_axis_seats_at_the_transform_on_both_roads() {
        for (label, src, datum) in [
            ("msolve7-axis-t1-axis", Src::Axis, AXIS),
            ("msolve7-axis-t1-frame", Src::Frame, FRAME2),
        ] {
            let ev = both_roads(
                label,
                src,
                Src::T1,
                T1,
                wrong(
                    datum,
                    crate::eval::phrase::BODY_OR_INSTANCES,
                    crate::eval::family::DATUM,
                ),
            );
            poisoned_through(&ev, T2, T1);
            poisoned_through(&ev, PATTERN, T1);
        }
    }
}
