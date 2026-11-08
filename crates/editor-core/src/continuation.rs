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
//! **A continuation is a body read whole.** An operator left with one
//! member of a family (`∪[xs[k]]`) keeps that member as its one operand
//! rather than handing its readers the whole family; a reader that
//! reads one member of a deleted node at a seat the slot door writes
//! whole, and a cut whose target is one member, refuse typed
//! ([`SpliceFault`]): no edit hands them the member.
//!
//! The kernel's own [`DocEdit::DeleteNode`] picks no survivor (D10): it
//! leaves its readers unresolved. [`splice`] is the gesture's plan over
//! the doors every edit has — `SetParam`, `SetMembers`, `SetProduct`,
//! `Rebind`, then `DeleteNode` — so a reopened document deletes the same
//! way, and adding a fillet then deleting it leaves the document as it
//! was.
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
use crate::names::{Carry, RoleSeg, SegRewrite, StableName};
use crate::node::{Node, RecipeNodeId, SlotId};
use crate::operand::{Bodies, BodyRead, Operand, OperandSlot};
use crate::var::{VarDef, VarId};
use crate::{Doc, Formula, SlotValue};

/// **The one body a list of operands continues**: its one operand where
/// it holds exactly one, read whole. The census rule both an operator's
/// own continuation and a delete's collapse of one read
/// ([`continues`], [`splice`]).
fn sole_whole(reads: &[BodyRead<VarId>]) -> Option<VarId> {
    match reads {
        [one] if !one.is_indexed() => Some(one.read),
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
    match doc.node(node)? {
        Node::Fillet { selection, .. } | Node::Chamfer { selection, .. } => {
            doc.selection(*selection).map(|select| select.body)
        }
        Node::Shell { open, .. } => doc.selection(*open).map(|select| select.body),
        Node::Union { members } | Node::Intersect { members } | Node::Assemble { members } => {
            match members {
                Bodies::Spelled(reads) => sole_whole(reads),
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
) -> Option<Vec<(BodyRead<VarId>, BodyRead<Formula>)>> {
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

/// **Why a delete cannot splice** ([`splice`]): what it would have to
/// hand over that no edit can.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SpliceFault {
    /// `reader` reads one member of a deleted node at `seat`, which the
    /// slot door writes as a whole read: the member of what the node
    /// continues cannot be handed there.
    MemberAtSeat {
        /// The reader.
        reader: RecipeNodeId,
        /// Its seat.
        seat: OperandSlot,
    },
    /// `cut`'s tool is deleted and its target is one member of a family:
    /// what the cut continues is that member, which its readers cannot
    /// be handed.
    MemberTarget {
        /// The cut.
        cut: RecipeNodeId,
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

/// **Which body each deleted node continues**, by its own read (the
/// variable its recipe reads), `None` where it continues nothing.
type Dead = BTreeMap<RecipeNodeId, Option<VarId>>;

/// **The live body `var` stands for once `dead` is deleted**: itself, or
/// down the continuations of the deleted operations defining it; `None`
/// where one continues nothing.
///
/// # Errors
///
/// [`SpliceFault::ContinuationCycle`] where the continuations come back
/// to a node: each continues one of its own inputs, so in an acyclic
/// recipe they cannot.
fn live<P: crate::ProfilePayload>(
    doc: &Doc<P>,
    dead: &Dead,
    mut var: VarId,
) -> Result<Option<VarId>, SpliceFault> {
    let mut seen = BTreeSet::new();
    while let Some(op) = doc.operation_of(var) {
        let Some(&continued) = dead.get(&op) else {
            return Ok(Some(var));
        };
        if !seen.insert(op) {
            return Err(SpliceFault::ContinuationCycle { node: op });
        }
        let Some(next) = continued else {
            return Ok(None);
        };
        var = next;
    }
    Ok(Some(var))
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
    carried_back(name, |node| dead.get(&node).copied().flatten())
}

/// **The rewrite that carries names back through handed copies**: a
/// `From { read, of }` segment at any depth whose `read` is one of
/// `bodies` (the outputs of copies handed back to what a deleted node
/// continues) carries `of` back ([`carried_back`]). The walk is the one
/// the forward takeover's rebinds take (`names::recarried`), with the
/// map read off the names instead of an evaluation.
struct CarryBack<'a> {
    bodies: &'a BTreeSet<VarId>,
    dead: &'a Dead,
}

impl SegRewrite for CarryBack<'_> {
    type Error = core::convert::Infallible;

    fn section(
        &mut self,
        s: crate::node::SectionId,
    ) -> Result<crate::node::SectionId, Self::Error> {
        Ok(s)
    }

    fn carried(&mut self, read: VarId, of: &StableName) -> Result<Carry, Self::Error> {
        if !self.bodies.contains(&read) {
            return Ok(Carry::Descend);
        }
        let to = back(self.dead, of);
        Ok(if to == *of {
            Carry::Descend
        } else {
            Carry::Replace(to)
        })
    }

    fn name(&mut self, _: &StableName) -> Result<Carry, Self::Error> {
        Ok(Carry::Descend)
    }

    fn descended(
        &mut self,
        n: &StableName,
        walked: StableName,
    ) -> Result<Option<StableName>, Self::Error> {
        Ok((walked != *n).then_some(walked))
    }

    fn index(&mut self, index: VarId) -> Result<VarId, Self::Error> {
        Ok(index)
    }
}

/// **`read` with every body it reads from a deleted operation handed to
/// what that operation continues**, the names of a selection carried
/// back ([`carried_back`]); `None` where a read it holds continues to
/// nothing, or is a named read (shared, so not this reader's to
/// rewrite) that reaches a deleted operation.
fn handed<P: crate::ProfilePayload>(
    doc: &Doc<P>,
    dead: &Dead,
    read: &Operand,
) -> Result<Option<Operand>, SpliceFault> {
    let touches = |var: VarId| {
        doc.read_ports(var)
            .iter()
            .any(|(at, _)| dead.contains_key(at))
    };
    let var_read = |var: VarId| -> Result<Option<Operand>, SpliceFault> {
        Ok(match doc.var(var).map(crate::Var::def) {
            Some(VarDef::Output { node, .. }) if dead.contains_key(node) => {
                live(doc, dead, var)?.map(Operand::Var)
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
            Ok(handed(doc, dead, body)?.map(|body| Operand::Select {
                body: Box::new(body),
                names: names.iter().map(|name| back(dead, name)).collect(),
            }))
        }
        Operand::Pose(def) => {
            let mut fault = None;
            let def = def.try_map(
                &mut |_, read| match handed(doc, dead, read) {
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
                handed(doc, dead, placement)?.map(|placement| Operand::Pick {
                    placement: Box::new(placement),
                    of: of.clone(),
                }),
            )
        }
    }
}

/// **How a surviving reader is handed over**: each seat it reads a
/// deleted operation through, rewritten.
struct Handed {
    /// The seats rewritten, each to its new read.
    seats: Vec<(OperandSlot, Operand)>,
    /// The seats that read something continuing to nothing.
    lost: BTreeSet<OperandSlot>,
    /// Whether the node depends on a deleted operation other than
    /// through its operand seats (a space it stands in, a measured
    /// value its expressions read).
    otherwise: bool,
}

/// [`Handed`] for `n`, which reads some deleted operation. A member of a
/// deleted node read as a list operand is handed as the same member of
/// what the node continues (the list keeps the indices); read at any
/// other seat it refuses ([`SpliceFault::MemberAtSeat`]).
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
        let rewritten = match node.body_read(seat) {
            Some(read) => {
                let now = live(doc, dead, read.read)?;
                if read.is_indexed() && now.is_some() && !matches!(seat, OperandSlot::Member(_)) {
                    return Err(SpliceFault::MemberAtSeat { reader: n, seat });
                }
                now.map(Operand::Var)
            }
            None => handed(doc, dead, &doc.written_operand(var))?,
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
    /// It goes, continuing this body (or nothing).
    Goes(Option<VarId>),
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
    if let Some(members) = spelled(doc, n) {
        if lost
            .iter()
            .any(|seat| !matches!(seat, OperandSlot::Member(_)))
        {
            return Ok(Fate::Goes(None));
        }
        let kept: Vec<(BodyRead<VarId>, BodyRead<Formula>)> = members
            .into_iter()
            .enumerate()
            .filter(|(i, _)| !lost.contains(&OperandSlot::Member(*i)))
            .map(
                |(i, (stored, written))| match rewritten(OperandSlot::Member(i)) {
                    Some(Operand::Var(var)) => (
                        BodyRead {
                            read: *var,
                            at: stored.at,
                        },
                        BodyRead {
                            read: Operand::Var(*var),
                            at: written.at,
                        },
                    ),
                    _ => (stored, written),
                },
            )
            .collect();
        let stored: Vec<BodyRead<VarId>> = kept.iter().map(|(stored, _)| stored.clone()).collect();
        if kept.is_empty() {
            return Ok(Fate::Goes(None));
        }
        if !lost.is_empty()
            && let Some(one) = sole_whole(&stored)
        {
            return Ok(Fate::Goes(Some(one)));
        }
        return Ok(Fate::Stays(vec![DocEdit::SetMembers {
            node: n,
            members: Bodies::Spelled(kept.into_iter().map(|(_, written)| written).collect()),
        }]));
    }
    if let Node::Subtract { from, .. } = node
        && !lost.is_empty()
        && lost.iter().all(|seat| *seat == OperandSlot::Cut)
    {
        if from.is_indexed() {
            return Err(SpliceFault::MemberTarget { cut: n });
        }
        let target = match rewritten(OperandSlot::From) {
            Some(Operand::Var(var)) => *var,
            _ => from.read,
        };
        return Ok(Fate::Goes(Some(target)));
    }
    if !lost.is_empty() {
        return Ok(Fate::Goes(None));
    }
    Ok(Fate::Stays(
        seats
            .into_iter()
            .map(|(seat, read)| DocEdit::SetParam {
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
/// Walking the recipe in evaluation order (`edit::walk_readers`),
/// each node reading a deleted operation is decided once:
/// - every seat it reads one through is handed to a live body: it stays,
///   those seats rewritten (a list operand through `SetMembers`, any
///   other seat through `SetParam`);
/// - an operator whose lost seats are list operands drops them, and
///   with one operand left read whole it continues that operand and
///   goes, with one member left it keeps it, with none it goes;
/// - a cut whose tool is lost continues its target and goes;
/// - anything else goes, continuing nothing.
///
/// The product slot reading a deleted operation is handed the same way.
/// A placement whose source was handed over is followed by a `Rebind`
/// of every name read through its copy that a deleted node carried in,
/// down the copies of that copy.
///
/// # Errors
///
/// [`SpliceFault`]: a member that no edit can hand over.
pub fn splice<P: crate::ProfilePayload + Clone>(
    doc: &Doc<P>,
    id: RecipeNodeId,
) -> Result<Option<Splice<P>>, SpliceFault> {
    if doc.node(id).is_none() {
        return Ok(None);
    }
    let mut dead: Dead = BTreeMap::from([(id, continues(doc, id))]);
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
    if let Some(product) = doc.product()
        && doc
            .operation_of(product)
            .is_some_and(|op| dead.contains_key(&op))
        && let Some(now) = live(doc, &dead, product)?
    {
        edits.push(DocEdit::SetProduct {
            product: Some(Operand::Var(now)),
        });
    }
    edits.extend(rebinds(doc, &dead, &handed_sources));
    Ok(Some(Splice { deleted, edits }))
}

/// **The rebinds a hand-over of `placements`' sources owes**: every name
/// a selection holds that reads one of their copies — down the copies
/// of those copies — and carries a name a deleted node carried in: a
/// name on the copy itself, and a name carried in from the copy at any
/// depth of a later feature's name ([`crate::resolve::derivation_reads`]),
/// each back to the name it carried ([`CarryBack`]).
fn rebinds<P: crate::ProfilePayload>(
    doc: &Doc<P>,
    dead: &Dead,
    placements: &[RecipeNodeId],
) -> Vec<DocEdit<P>> {
    let mut bodies: BTreeSet<VarId> = placements
        .iter()
        .flat_map(|&placement| doc.outputs(placement))
        .collect();
    loop {
        let more: Vec<VarId> = doc
            .placements()
            .into_iter()
            .filter(|&p| !dead.contains_key(&p))
            .filter(|&p| {
                doc.copied_read(p)
                    .is_some_and(|read| bodies.contains(&read))
            })
            .flat_map(|p| doc.outputs(p))
            .filter(|var| !bodies.contains(var))
            .collect();
        if more.is_empty() {
            break;
        }
        bodies.extend(more);
    }
    let mut seen: BTreeSet<(VarId, StableName)> = BTreeSet::new();
    let mut out = Vec::new();
    for held in doc.vars().values() {
        let VarDef::Select(select) = held.def() else {
            continue;
        };
        let on_a_copy = bodies.contains(&select.body);
        for name in &select.names {
            if !on_a_copy
                && !crate::resolve::derivation_reads(name)
                    .iter()
                    .any(|read| bodies.contains(read))
            {
                continue;
            }
            let top = if on_a_copy {
                back(dead, name)
            } else {
                name.clone()
            };
            let Ok(to) = top.rewrite_path(&mut CarryBack {
                bodies: &bodies,
                dead,
            });
            if to != *name && seen.insert((select.body, name.clone())) {
                out.push(DocEdit::Rebind {
                    body: Some(select.body),
                    from: name.clone(),
                    to,
                });
            }
        }
    }
    out
}
