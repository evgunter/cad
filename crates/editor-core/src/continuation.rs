//! **Delete by continuation** (DM6, `crates/editor-core/REFERENCES.md`):
//! the delete gesture hands each reader of a deleted node to the body
//! that node continues, as explicit edits in one action, derived from
//! the recipe and never recorded.
//!
//! A node **continues** a body when it is that body changed in place, in
//! its space, carrying its entities ([`continues`]): a fillet, a chamfer
//! or a shell continues its input; an operator left with one operand
//! read whole continues that operand; a cut whose tool is deleted
//! continues its target. Nothing else continues anything — a placement,
//! a mirror image, a split half, a pattern, a construction — and the
//! readers of such a node go with it, by the walk
//! [`crate::cascade_delete_order`] takes (`edit::walk_readers`).
//!
//! **A member is handed as a member, by one rule.** What a deleted node
//! continues may be one member of a family — a cut of `xs[k]` whose tool
//! goes, a fillet over a family read at `F[k]`. A list operand is handed
//! that member (`SetMembers` keeps indices); an operator left with one
//! member keeps it as its one operand, so its readers still read one
//! body. Every other seat — a placement's source, a selection's or a
//! pose's body, the product slot — is written as a whole read, so a
//! member that would land there refuses typed ([`SpliceFault`]), and
//! nothing is deleted. A cut no one reads hands nothing and refuses
//! nothing.
//!
//! The kernel's own [`DocEdit::DeleteNode`] picks no survivor (D10): it
//! leaves its readers unresolved. [`splice`] is the gesture's plan over
//! the doors every edit has — `SetParam`, `SetMembers`, `SetProduct`,
//! `DefineVar`, `Rebind`, then `DeleteNode` — so a reopened document
//! deletes the same way, and adding a fillet then deleting it leaves the
//! document as it was.
//!
//! **A definition is a reader.** A placement reads its body through a
//! source (VARIABLES VR4); a source reading a deleted operation's body is
//! redefined as a read of what it continues, and every seat reading the
//! deleted operation only through sources — a placement's source, a
//! mate's side selecting the source — keeps its read, its names handed
//! back with the copies'.
//!
//! **Names.** A name the deleted node carried in from the body it
//! continues (`[From { read, of }]`, `read` that body) returns to the
//! name it carried ([`carried_back`]); a name it minted strands, and the
//! delete reports it (DM7). A copy handed back carries the continued
//! body's names again, so every name read through it — on the copy, or
//! carried in from it at any depth of a later feature's name — is
//! rebound back, the reach the forward takeover has
//! (`Recording::take_over`).

use std::collections::{BTreeMap, BTreeSet};

use crate::edit::{DocEdit, EditError, Recording};
use crate::names::{Carries, RoleSeg, StableName};
use crate::node::{Node, RecipeNodeId, SlotId};
use crate::operand::{Bodies, BodyRead, Operand, OperandSlot};
use crate::var::{VarDef, VarId};
use crate::{Doc, Formula, SlotValue};

/// **The one body a list of operands continues**: its one operand where
/// it holds exactly one, read whole. The census rule both an operator's
/// own continuation and a delete's collapse of one read
/// ([`continues`], [`splice`]).
/// Each read is its variable and whether it names members.
fn sole_whole(reads: &[(VarId, bool)]) -> Option<VarId> {
    match reads {
        [(one, false)] => Some(*one),
        _ => None,
    }
}

/// **The body `node` continues, read off its recipe**: a fillet's, a
/// chamfer's or a shell's input (the body its selection is read in), and
/// an operator's one operand where it holds exactly one, read whole.
/// `None` for every other node, and for one `doc` does not hold.
///
/// A cut continues its target only once its tool is deleted, which is
/// the delete's to say ([`splice`]), not the cut's.
pub fn continues<P: crate::ProfilePayload>(doc: &Doc<P>, node: RecipeNodeId) -> Option<VarId> {
    // A member read is no variable a reader could read in its place,
    // except one at its binder's own indices by a node that binder
    // yields: `{k in xs: fillet(xs[k])}` continues the family `xs` whole
    // (D10 `[repetition]`).
    let whole = |body: &crate::BodyRead<VarId>| {
        if !body.is_indexed() {
            return Some(body.read);
        }
        let binder = doc.yielding(body.read)?;
        (doc.binder_indices(binder) == body.at
            && doc
                .outputs(node)
                .iter()
                .any(|&out| doc.yielding(out) == Some(binder)))
        .then_some(body.read)
    };
    match doc.node(node)? {
        Node::Fillet { selection, .. } | Node::Chamfer { selection, .. } => doc
            .selection(*selection)
            .and_then(|select| whole(&select.body)),
        Node::Shell { open, .. } => doc.selection(*open).and_then(|select| whole(&select.body)),
        Node::Union { members } | Node::Intersect { members } | Node::Assemble { members } => {
            match members {
                Bodies::Spelled(reads) => sole_whole(
                    &reads
                        .iter()
                        .map(|read| {
                            (
                                read.read().read,
                                read.is_splice() || read.read().is_indexed(),
                            )
                        })
                        .collect::<Vec<_>>(),
                ),
                Bodies::Family(_) => None,
            }
        }
        Node::Profile(_)
        | Node::Extrude { .. }
        | Node::Revolve { .. }
        | Node::Tube { .. }
        | Node::HollowTube { .. }
        | Node::Loft { .. }
        | Node::Sweep { .. }
        | Node::Split { .. }
        | Node::Mirror { .. }
        | Node::Subtract { .. }
        | Node::Place { .. }
        | Node::Datum { .. }
        | Node::World
        | Node::Space
        | Node::InstantiatePart { .. }
        | Node::Measure { .. }
        | Node::Assertion(_) => None,
    }
}

/// **A spelled operator's operands, stored and as written**, in list
/// order: a `∪`'s, a `∩`'s or a `⊔`'s, each read beside the read the
/// edit doors take back ([`Node::authored`]). `None` for any other node
/// and for an operator reading a family whole. The one re-authoring of
/// a spelled list, for every edit that keeps some members and drops or
/// re-points others.
#[allow(clippy::type_complexity)]
pub fn spelled<P: crate::ProfilePayload>(
    doc: &Doc<P>,
    id: RecipeNodeId,
) -> Option<
    Vec<(
        crate::Spell<BodyRead<VarId>>,
        crate::Spell<BodyRead<Formula>>,
    )>,
> {
    let (Node::Union { members } | Node::Intersect { members } | Node::Assemble { members }) =
        doc.node(id)?
    else {
        return None;
    };
    let Bodies::Spelled(stored) = members else {
        return None;
    };
    let (Node::Union { members: written }
    | Node::Intersect { members: written }
    | Node::Assemble { members: written }) = doc.node(id)?.authored(doc)
    else {
        return None;
    };
    let Bodies::Spelled(written) = written else {
        return None;
    };
    Some(stored.iter().cloned().zip(written).collect())
}

/// **A subtract's spelled tools, stored and as written**, in list
/// order, as [`spelled`] gives an operator's operands. `None` for any
/// other node and for a subtract reading a family whole.
#[allow(clippy::type_complexity)]
fn spelled_tools<P: crate::ProfilePayload>(
    doc: &Doc<P>,
    id: RecipeNodeId,
) -> Option<
    Vec<(
        crate::Spell<BodyRead<VarId>>,
        crate::Spell<BodyRead<Formula>>,
    )>,
> {
    let Node::Subtract {
        tool: Bodies::Spelled(stored),
        ..
    } = doc.node(id)?
    else {
        return None;
    };
    let Node::Subtract {
        tool: Bodies::Spelled(written),
        ..
    } = doc.node(id)?.authored(doc)
    else {
        return None;
    };
    Some(stored.iter().cloned().zip(written).collect())
}

/// **Why a delete cannot splice** ([`splice`]): a member of a family
/// that would land where no edit can write one.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SpliceFault {
    /// `reader` would be handed one member of a family at `seat`, which
    /// the slot door writes as a whole read (a placement's source, a
    /// selection's or a pose's body).
    MemberAtSeat {
        /// The reader.
        reader: RecipeNodeId,
        /// Its seat.
        seat: OperandSlot,
    },
    /// The product slot would be handed one member of a family, and it
    /// reads a whole body.
    MemberAtProduct,
    /// A reader of one member of `node`, deleted, would be handed a
    /// member of what `node` continues, itself one member: a member of a
    /// member, which no read spells.
    MemberOfAMember {
        /// The deleted node.
        node: RecipeNodeId,
    },
    /// The continuations reached `node` twice, which an acyclic recipe
    /// cannot do.
    ContinuationCycle {
        /// Where the walk came back to.
        node: RecipeNodeId,
    },
}

/// **A delete by continuation, planned**: the nodes it deletes,
/// consumers first and the target last, and the edits that hand every
/// surviving reader to what its deleted read continues, applied before
/// the deletes.
#[derive(Debug, Clone)]
pub struct Splice<P: crate::ProfilePayload> {
    /// The nodes deleted, in the order the door accepts them.
    pub deleted: Vec<RecipeNodeId>,
    /// The hand-over edits, in order.
    pub edits: Vec<DocEdit<P>>,
}

impl<P: crate::ProfilePayload + Clone> Splice<P> {
    /// **This plan, applied in `run`**: the hand-over edits, then the
    /// deletes.
    ///
    /// # Errors
    ///
    /// The first door that refuses.
    pub fn apply(self, run: &mut Recording<'_, P>) -> Result<(), EditError> {
        for edit in self.edits {
            run.apply(edit)?;
        }
        for id in self.deleted {
            run.apply(DocEdit::DeleteNode { id })?;
        }
        Ok(())
    }
}

/// **What a deleted node continues**: the variable its recipe reads, and
/// the indices of the member read, as written (empty for a body read
/// whole).
#[derive(Debug, Clone)]
struct Continued {
    read: VarId,
    at: Vec<Formula>,
}

impl Continued {
    fn whole(read: VarId) -> Self {
        Self {
            read,
            at: Vec::new(),
        }
    }

    fn is_member(&self) -> bool {
        !self.at.is_empty()
    }
}

/// **What each deleted node continues**, `None` where it continues
/// nothing.
type Dead = BTreeMap<RecipeNodeId, Option<Continued>>;

/// **What the body `var` reads stands for once `dead` is deleted**:
/// itself, or down the continuations of the deleted operations defining
/// it, a member where one continues a member; `None` where one continues
/// nothing.
///
/// # Errors
///
/// [`SpliceFault::MemberOfAMember`] where a member continues to a
/// member, and [`SpliceFault::ContinuationCycle`] where the
/// continuations come back to a node: each continues one of its own
/// inputs, so in an acyclic recipe they cannot.
fn live<P: crate::ProfilePayload>(
    doc: &Doc<P>,
    dead: &Dead,
    var: VarId,
) -> Result<Option<Continued>, SpliceFault> {
    let mut now = Continued::whole(var);
    let mut seen = BTreeSet::new();
    while let Some(op) = doc.operation_of(now.read) {
        let Some(continued) = dead.get(&op) else {
            return Ok(Some(now));
        };
        if !seen.insert(op) {
            return Err(SpliceFault::ContinuationCycle { node: op });
        }
        let Some(next) = continued else {
            return Ok(None);
        };
        if now.is_member() && next.is_member() {
            return Err(SpliceFault::MemberOfAMember { node: op });
        }
        now = Continued {
            read: next.read,
            at: if now.is_member() {
                now.at
            } else {
                next.at.clone()
            },
        };
    }
    Ok(Some(now))
}

/// [`live`], where only a whole body can be written: `at`, a reader's
/// seat, or the product slot where `None`.
fn live_whole<P: crate::ProfilePayload>(
    doc: &Doc<P>,
    dead: &Dead,
    var: VarId,
    at: Option<(RecipeNodeId, OperandSlot)>,
) -> Result<Option<VarId>, SpliceFault> {
    match live(doc, dead, var)? {
        Some(now) if now.is_member() => Err(match at {
            Some((reader, seat)) => SpliceFault::MemberAtSeat { reader, seat },
            None => SpliceFault::MemberAtProduct,
        }),
        now => Ok(now.map(|now| now.read)),
    }
}

/// **The name `name` returns to once the nodes `continued` answers for
/// are deleted**: while its node is one of them and the name is an
/// entity that node carried in whole from the body it continues
/// (`[From { read, of }]`, `read` that body), the name it carried. Any
/// other name is answered as it is: one a deleted node minted strands.
///
/// The walk ends: each step answers the name `of` the step's segment
/// holds, a strict part of the name it read.
pub fn carried_back(
    name: &StableName,
    continued: impl Fn(RecipeNodeId) -> Option<VarId>,
) -> StableName {
    let mut name = name.clone();
    while let Some(read) = continued(name.node) {
        let [RoleSeg::From { read: from, of }] = name.path.as_slice() else {
            break;
        };
        if *from != read {
            break;
        }
        name = of.name().clone();
    }
    name
}

/// [`carried_back`] over a plan's deleted nodes.
fn back(dead: &Dead, name: &StableName) -> StableName {
    carried_back(name, |node| {
        dead.get(&node).and_then(Option::as_ref).map(|now| now.read)
    })
}

/// **Names carried back through handed copies**: a `From { read, of }`
/// segment whose `read` is one of `bodies` (the outputs of nodes that
/// carry a handed copy's names) carries `of` back ([`carried_back`]).
/// The walk is the forward takeover's (`names::recarried_by`), the
/// carries read off the names instead of an evaluation.
struct CarryBack<'a> {
    bodies: &'a BTreeSet<VarId>,
    dead: &'a Dead,
}

impl Carries for CarryBack<'_> {
    fn carried(&self, read: VarId, of: &StableName) -> Option<StableName> {
        if !self.bodies.contains(&read) {
            return None;
        }
        let to = back(self.dead, of);
        (to != *of).then_some(to)
    }
}

/// **`read` with every body it reads from a deleted operation handed to
/// what that operation continues**, the names of a selection carried
/// back ([`carried_back`]); `None` where a read it holds continues to
/// nothing, or is a named read (shared, so not this reader's to
/// rewrite) that reaches a deleted operation. `at` is the reader and the
/// seat holding `read`, `None` for the product slot, which writes a whole
/// body.
fn handed<P: crate::ProfilePayload>(
    doc: &Doc<P>,
    dead: &Dead,
    read: &Operand,
    at: Option<(RecipeNodeId, OperandSlot)>,
) -> Result<Option<Operand>, SpliceFault> {
    let touches = |var: VarId| {
        doc.read_ports(var)
            .iter()
            .any(|(at, _)| dead.contains_key(at))
    };
    let var_read = |var: VarId| -> Result<Option<Operand>, SpliceFault> {
        Ok(match doc.var(var).map(crate::Var::def) {
            Some(VarDef::Output { node, .. }) if dead.contains_key(node) => {
                live_whole(doc, dead, var, at)?.map(Operand::Var)
            }
            // A source is read as it is: its definition is handed over.
            Some(VarDef::Source(_)) if touches(var) => {
                source_survives(doc, dead, var)?.then_some(Operand::Var(var))
            }
            _ if touches(var) => None,
            _ => Some(Operand::Var(var)),
        })
    };
    match read {
        Operand::Var(var) => var_read(*var),
        Operand::Name(name) => match doc.var_named(name.as_str()) {
            Some(var) if touches(var) => var_read(var),
            _ => Ok(Some(read.clone())),
        },
        Operand::Node(node) if dead.contains_key(node) => match doc.read_of_node(*node) {
            Some(var) => var_read(var),
            None => Ok(None),
        },
        Operand::Output { node, port } if dead.contains_key(node) => {
            match doc.output(*node, *port) {
                Some(var) => var_read(var),
                None => Ok(None),
            }
        }
        Operand::Node(_) | Operand::Output { .. } => Ok(Some(read.clone())),
        Operand::Select { body, names } => {
            Ok(
                handed(doc, dead, &body.read, at)?.map(|read| Operand::Select {
                    body: Box::new(crate::BodyRead {
                        read,
                        at: body.at.clone(),
                    }),
                    names: names.iter().map(|name| back(dead, name)).collect(),
                }),
            )
        }
        Operand::Pose(def) => {
            let mut fault = None;
            let def = def.try_map(
                &mut |_, read| match handed(doc, dead, read, at) {
                    Ok(Some(read)) => Ok(read),
                    Ok(None) => Err(()),
                    Err(error) => {
                        fault = Some(error);
                        Err(())
                    }
                },
                &mut |_, scalar: &Formula| Ok(scalar.clone()),
            );
            if let Some(fault) = fault {
                return Err(fault);
            }
            Ok(def.ok().map(|def| Operand::Pose(Box::new(def))))
        }
        Operand::Pick { placement, of } => {
            Ok(
                handed(doc, dead, placement, at)?.map(|placement| Operand::Pick {
                    placement: Box::new(placement),
                    of: of.clone(),
                }),
            )
        }
        Operand::Source(body) => Ok(handed(doc, dead, &body.read, at)?.map(|read| {
            Operand::Source(Box::new(crate::BodyRead {
                read,
                at: body.at.clone(),
            }))
        })),
    }
}

/// **Whether `var` reaches a deleted operation other than through a
/// source**, the sources it reaches collected into `sources`: an output
/// of one directly, or a selection's body, a pose's reads or a pick's
/// copy that does.
fn reaches_directly<P>(doc: &Doc<P>, dead: &Dead, var: VarId, sources: &mut Vec<VarId>) -> bool {
    match doc.var(var).map(crate::Var::def) {
        Some(VarDef::Output { node, .. }) => dead.contains_key(node),
        Some(VarDef::Source(_)) => {
            sources.push(var);
            false
        }
        Some(VarDef::Select(select)) => reaches_directly(doc, dead, select.body.read, sources),
        Some(VarDef::Pick(pick)) => reaches_directly(doc, dead, pick.placement, sources),
        Some(VarDef::Pose(def)) => {
            let mut direct = false;
            for (_, &read, _, _) in def.reads() {
                direct |= reaches_directly(doc, dead, read, sources);
            }
            direct
        }
        Some(VarDef::Free(_) | VarDef::Defined(_) | VarDef::Index(_)) | None => false,
    }
}

/// **Whether the source `var` still reads a body once `dead` is
/// deleted**: what its holder reads stands for a live body ([`live`]).
///
/// # Errors
///
/// [`live`]'s, and [`SpliceFault::MemberOfAMember`] where a member read
/// would be handed a member.
fn source_survives<P: crate::ProfilePayload>(
    doc: &Doc<P>,
    dead: &Dead,
    var: VarId,
) -> Result<bool, SpliceFault> {
    let Some(body) = doc.source_holder(var) else {
        return Ok(false);
    };
    match live(doc, dead, body.read)? {
        None => Ok(false),
        Some(now) if body.is_indexed() && now.is_member() => Err(SpliceFault::MemberOfAMember {
            node: doc.operation_of(body.read).unwrap_or(RecipeNodeId::WORLD),
        }),
        Some(_) => Ok(true),
    }
}

/// **The source `var` a delete hands over** (DM6: a definition is a
/// reader), where its definition reads a body a deleted operation
/// defines: redefined as a read of what that body stands for ([`live`]),
/// a member where it continues one. A source whose body continues to
/// nothing is left reading it, unresolved (VR7), its readers gone with
/// the walk.
fn source_handover<P: crate::ProfilePayload>(
    doc: &Doc<P>,
    dead: &Dead,
    var: VarId,
    body: &BodyRead<VarId>,
) -> Result<Option<DocEdit<P>>, SpliceFault> {
    if doc.source(body.read).is_some()
        || !doc
            .operation_of(body.read)
            .is_some_and(|op| dead.contains_key(&op))
    {
        return Ok(None);
    }
    let Some(now) = live(doc, dead, body.read)? else {
        return Ok(None);
    };
    let at = match (body.is_indexed(), now.is_member()) {
        (true, true) => {
            return Err(SpliceFault::MemberOfAMember {
                node: doc.operation_of(body.read).unwrap_or(RecipeNodeId::WORLD),
            });
        }
        (true, false) => doc.written_body(body, Operand::Var(now.read)).at,
        (false, _) => now.at,
    };
    Ok(Some(DocEdit::DefineVar {
        var: var.into(),
        def: crate::VarDecl::Source(BodyRead {
            read: Operand::Var(now.read),
            at,
        }),
        fresh: Vec::new(),
    }))
}

/// **One seat handed over**: its new read, and, for a body read, the
/// indices of the member it now reads where the hand-over makes it one
/// (`None` keeps the seat's own).
struct Seat {
    read: Operand,
    at: Option<Vec<Formula>>,
}

/// **How a surviving reader is handed over**: each seat it reads a
/// deleted operation through, rewritten.
struct Handed {
    /// The seats rewritten.
    seats: Vec<(OperandSlot, Seat)>,
    /// The seats that read something continuing to nothing.
    lost: BTreeSet<OperandSlot>,
    /// Whether the node depends on a deleted operation other than
    /// through its operand seats (a space it stands in, a measured
    /// value its expressions read).
    otherwise: bool,
}

/// [`Handed`] for `n`, which reads some deleted operation. A body seat
/// is handed what its read stands for ([`live`]): a member of a deleted
/// family-valued node is the same member of what it continues, and a
/// whole read of a node that continues a member is that member. A
/// member lands only in a list operand; anywhere else it refuses
/// ([`SpliceFault::MemberAtSeat`]).
fn hand<P: crate::ProfilePayload>(
    doc: &Doc<P>,
    dead: &Dead,
    n: RecipeNodeId,
    node: &Node<P>,
) -> Result<Handed, SpliceFault> {
    let mut seats = Vec::new();
    let mut lost = BTreeSet::new();
    let mut reached = BTreeSet::new();
    for (seat, var) in node.operand_rows() {
        let at: Vec<RecipeNodeId> = doc
            .read_ports(var)
            .into_iter()
            .map(|(at, _)| at)
            .filter(|at| dead.contains_key(at))
            .collect();
        if at.is_empty() || crate::mate::solve::own_copy_side(doc, n, seat, var) {
            continue;
        }
        reached.extend(at);
        // A seat reaching a deleted operation only through sources — a
        // placement's source, a mate's side selecting the source — keeps
        // its read: the source's definition is the reader handed over
        // ([`source_handover`]), and the names in it are rebound after.
        let mut sources = Vec::new();
        if !reaches_directly(doc, dead, var, &mut sources) {
            for source in sources {
                if !source_survives(doc, dead, source)? {
                    lost.insert(seat);
                }
            }
            continue;
        }
        let rewritten = match node.body_read(seat) {
            Some(read) => match live(doc, dead, read.read)? {
                None => None,
                Some(now) => {
                    if read.is_indexed() && now.is_member() {
                        return Err(SpliceFault::MemberOfAMember {
                            node: doc.operation_of(read.read).unwrap_or(n),
                        });
                    }
                    let member = read.is_indexed() || now.is_member();
                    if member && !matches!(seat, OperandSlot::Member(_) | OperandSlot::Cut(_)) {
                        return Err(SpliceFault::MemberAtSeat { reader: n, seat });
                    }
                    Some(Seat {
                        read: Operand::Var(now.read),
                        at: now.is_member().then_some(now.at),
                    })
                }
            },
            None => handed(doc, dead, &doc.written_operand(var), Some((n, seat)))?
                .map(|read| Seat { read, at: None }),
        };
        match rewritten {
            Some(read) => seats.push((seat, read)),
            None => {
                lost.insert(seat);
            }
        }
    }
    let otherwise = doc
        .spaces_read(node)
        .iter()
        .any(|(_, at)| dead.contains_key(at))
        || doc
            .upstream(n)
            .into_iter()
            .any(|at| dead.contains_key(&at) && !reached.contains(&at));
    Ok(Handed {
        seats,
        lost,
        otherwise,
    })
}

/// **What the delete does with one reader `n`** of a deleted node.
enum Fate<P: crate::ProfilePayload> {
    /// It stays, these edits handing it over.
    Stays(Vec<DocEdit<P>>),
    /// It goes, continuing this (or nothing).
    Goes(Option<Continued>),
}

/// The [`Fate`] of `n`, under `dead` as the walk has it so far.
fn fate<P: crate::ProfilePayload + Clone>(
    doc: &Doc<P>,
    dead: &Dead,
    n: RecipeNodeId,
    node: &Node<P>,
) -> Result<Fate<P>, SpliceFault> {
    let Handed {
        seats,
        lost,
        otherwise,
    } = hand(doc, dead, n, node)?;
    if otherwise {
        return Ok(Fate::Goes(None));
    }
    let rewritten = |seat: OperandSlot| seats.iter().find(|(s, _)| *s == seat).map(|(_, r)| r);
    // A subtract keeping its `from` and some of its spelled tools keeps
    // cutting by those; one losing every tool continues as its `from`,
    // below.
    let list = match spelled(doc, n) {
        Some(members) => Some((members, crate::operand::Seats::MEMBERS, true)),
        None => spelled_tools(doc, n)
            .filter(|tools| {
                !lost.contains(&OperandSlot::From)
                    && (0..tools.len()).any(|i| !lost.contains(&OperandSlot::Cut(i)))
            })
            .map(|tools| (tools, crate::operand::Seats::CUTS, false)),
    };
    // An operator left one whole operand continues it; a subtract left
    // one tool still cuts.
    if let Some((members, list_seats, collapses)) = list {
        let seat_of = |i: usize| list_seats.of(OperandSlot::Member(i));
        if lost
            .iter()
            .any(|seat| !(0..members.len()).any(|i| seat_of(i) == *seat))
        {
            return Ok(Fate::Goes(None));
        }
        // Each operand kept: its variable, whether it names members,
        // and its read as written.
        let kept: Vec<(VarId, bool, crate::Spell<BodyRead<Formula>>)> = members
            .into_iter()
            .enumerate()
            .filter(|(i, _)| !lost.contains(&seat_of(*i)))
            .map(|(i, (stored, written))| match rewritten(seat_of(i)) {
                Some(Seat {
                    read: Operand::Var(var),
                    at,
                }) => {
                    let splice = written.is_splice();
                    let at = at.clone().unwrap_or(written.read().at.clone());
                    let read = BodyRead {
                        read: Operand::Var(*var),
                        at,
                    };
                    (
                        *var,
                        splice || !read.at.is_empty(),
                        crate::Spell::of(read, splice),
                    )
                }
                _ => (
                    stored.read().read,
                    stored.is_splice() || stored.read().is_indexed(),
                    written,
                ),
            })
            .collect();
        let census: Vec<(VarId, bool)> = kept
            .iter()
            .map(|(var, member, _)| (*var, *member))
            .collect();
        if kept.is_empty() {
            return Ok(Fate::Goes(None));
        }
        if !lost.is_empty()
            && collapses
            && let Some(one) = sole_whole(&census)
        {
            return Ok(Fate::Goes(Some(Continued::whole(one))));
        }
        return Ok(Fate::Stays(vec![DocEdit::SetMembers {
            node: n,
            members: Bodies::Spelled(kept.into_iter().map(|(_, _, written)| written).collect()),
        }]));
    }
    if let Node::Subtract { .. } = node
        && !lost.is_empty()
        && lost
            .iter()
            .all(|seat| matches!(seat, OperandSlot::Cut(_) | OperandSlot::Cuts))
    {
        // As written, so a literal index is minted afresh for its new
        // reader rather than shared unnamed.
        let Node::Subtract { from: written, .. } = node.written(doc) else {
            unreachable!("a cut re-authors as a cut")
        };
        let Node::Subtract { from, .. } = node else {
            unreachable!("matched above")
        };
        let target = match rewritten(OperandSlot::From) {
            Some(Seat {
                read: Operand::Var(var),
                at,
            }) => Continued {
                read: *var,
                at: at.clone().unwrap_or(written.at),
            },
            _ => Continued {
                read: from.read,
                at: written.at,
            },
        };
        return Ok(Fate::Goes(Some(target)));
    }
    if !lost.is_empty() {
        return Ok(Fate::Goes(None));
    }
    Ok(Fate::Stays(
        seats
            .into_iter()
            .map(|(seat, Seat { read, .. })| DocEdit::SetParam {
                node: n,
                slot: SlotId::Operand(seat),
                value: SlotValue::Read(read),
                fresh: Vec::new(),
            })
            .collect(),
    ))
}

/// **The delete of `id` by continuation, planned over `doc`** (module
/// docs): `None` for an id `doc` does not hold.
///
/// Walking the recipe in evaluation order (`edit::walk_readers`), each
/// node reading a deleted operation is decided once:
/// - every seat it reads one through is handed to a live body: it stays,
///   those seats rewritten (a list operand through `SetMembers`, any
///   other seat through `SetParam`);
/// - an operator whose lost seats are list operands drops them, and
///   with one operand left read whole it continues that operand and
///   goes, with one member left it keeps it, with none it goes;
/// - a cut whose tool is lost continues its target, a member where its
///   target is one, and goes;
/// - anything else goes, continuing nothing.
///
/// The product slot reading a deleted operation is handed the same way,
/// and so is every source reading one ([`source_handover`]). A placement
/// whose source was handed over is followed by a `Rebind` of every name
/// read through what carries its copy's names, or selected in the source,
/// that a deleted node carried in.
///
/// # Errors
///
/// [`SpliceFault`]: a member that would land where no edit writes one.
pub fn splice<P: crate::ProfilePayload + Clone>(
    doc: &Doc<P>,
    id: RecipeNodeId,
) -> Result<Option<Splice<P>>, SpliceFault> {
    if doc.node(id).is_none() {
        return Ok(None);
    }
    let mut dead: Dead = BTreeMap::from([(id, continues(doc, id).map(Continued::whole))]);
    let mut edits: Vec<DocEdit<P>> = Vec::new();
    let mut handed_sources: Vec<RecipeNodeId> = Vec::new();
    let mut fault = None;
    let deleted = crate::edit::walk_readers(doc, id, |n, node| {
        if fault.is_some() {
            return true;
        }
        match fate(doc, &dead, n, node) {
            Ok(Fate::Goes(continued)) => {
                dead.insert(n, continued);
                true
            }
            Ok(Fate::Stays(written)) => {
                if written.iter().any(|edit| {
                    matches!(
                        edit,
                        DocEdit::SetParam {
                            slot: SlotId::Operand(OperandSlot::Source),
                            ..
                        }
                    )
                }) {
                    handed_sources.push(n);
                }
                edits.extend(written);
                false
            }
            Err(error) => {
                fault = Some(error);
                true
            }
        }
    });
    if let Some(fault) = fault {
        return Err(fault);
    }
    let mut product = None;
    let mut redefined = Vec::new();
    let mut sources = Vec::new();
    for site in doc.read_sites() {
        match site {
            // Every node reading a deleted operation is decided by the
            // walk above: a node's fate turns on its inputs' fates, so it
            // is decided in evaluation order, not one read at a time.
            crate::doc::ReadSite::Node { .. } => {}
            // A source's definition is its reader, handed over in place
            // ([`source_handover`]); any other definition goes with the
            // seat reading it.
            crate::doc::ReadSite::Definition { var, read } => {
                if let Some(body) = doc.source(var).filter(|body| body.read == read)
                    && let Some(edit) = source_handover(doc, &dead, var, body)?
                {
                    redefined.push(edit);
                    sources.push(var);
                }
            }
            crate::doc::ReadSite::Product { read } => {
                let written = doc.written_operand(read);
                if let Some(read) = handed(doc, &dead, &written, None)?
                    && read != written
                {
                    product = Some(DocEdit::SetProduct {
                        product: Some(read),
                    });
                }
            }
        }
    }
    edits.extend(product);
    edits.extend(redefined);
    let reading: BTreeSet<VarId> = sources
        .iter()
        .flat_map(|&source| crate::take_over::sources_reading(doc, source).into_keys())
        .collect();
    let copies: BTreeSet<RecipeNodeId> = sources
        .iter()
        .flat_map(|&source| doc.plain_copies_of(source))
        .collect();
    handed_sources.extend(
        copies
            .into_iter()
            .filter(|placement| !dead.contains_key(placement)),
    );
    edits.extend(rebinds(doc, &dead, &handed_sources, &reading));
    Ok(Some(Splice { deleted, edits }))
}

/// **The rebinds a hand-over of `placements`' sources owes**: every name
/// a selection holds that reads what carries one of their copies' names
/// as they are — the copy, a copy of it, a mirror or a split of it, over
/// the census the forward takeover walks (`take_over::carrying_chains`)
/// — and carries a name a deleted node carried in: a name on such a
/// body itself, and a name carried in from one at any depth of a later
/// feature's name ([`crate::resolve::derivation_reads`]), each back to
/// the name it carried ([`CarryBack`]).
fn rebinds<P: crate::ProfilePayload>(
    doc: &Doc<P>,
    dead: &Dead,
    placements: &[RecipeNodeId],
    sources: &BTreeSet<VarId>,
) -> Vec<DocEdit<P>> {
    let bodies: BTreeSet<VarId> =
        crate::take_over::carrying_chains(doc, placements.iter().copied())
            .into_keys()
            .filter(|node| !dead.contains_key(node))
            .flat_map(|node| doc.outputs(node))
            .chain(sources.iter().copied())
            .collect();
    let carry = CarryBack {
        bodies: &bodies,
        dead,
    };
    let mut seen: BTreeSet<(VarId, StableName)> = BTreeSet::new();
    let mut out = Vec::new();
    for held in doc.vars().values() {
        let VarDef::Select(select) = held.def() else {
            continue;
        };
        let on_a_carrier = bodies.contains(&select.body.read);
        for name in &select.names {
            if !on_a_carrier
                && !crate::resolve::derivation_reads(name)
                    .iter()
                    .any(|read| bodies.contains(read))
            {
                continue;
            }
            let top = if on_a_carrier {
                back(dead, name)
            } else {
                name.clone()
            };
            let to = crate::names::recarried_by(&top, &carry).unwrap_or(top);
            if to != *name && seen.insert((select.body.read, name.clone())) {
                out.push(DocEdit::Rebind {
                    body: Some(select.body.read),
                    from: name.clone(),
                    to,
                });
            }
        }
    }
    out
}
