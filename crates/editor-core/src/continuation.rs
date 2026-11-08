//! **Delete by continuation** (DM6, `crates/editor-core/REFERENCES.md`):
//! the delete gesture hands each reader of a deleted node to the body
//! that node continues, as explicit edits in one action, derived from
//! the recipe and never recorded.
//!
//! A node **continues** a body when it is that body changed in place, in
//! its space, carrying its entities ([`continues`]): a fillet, a chamfer
//! or a shell continues its input; an operator left with one operand
//! continues that operand; a cut whose tool is deleted continues its
//! target. Nothing else continues anything — a placement, a mirror
//! image, a split half, a pattern, a construction — and the readers of
//! such a node go with it, as [`crate::cascade_delete_order`]'s do.
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
//! delete reports it (DM7).

use std::collections::{BTreeMap, BTreeSet};

use crate::edit::{DocEdit, EditError, Recording};
use crate::names::{RoleSeg, StableName};
use crate::node::{Node, RecipeNodeId, SlotId};
use crate::operand::{Bodies, BodyRead, Operand, OperandSlot};
use crate::var::{VarDef, VarId};
use crate::{Doc, Formula, SlotValue};

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
                Bodies::Spelled(reads) => match reads.as_slice() {
                    [one] if !one.is_indexed() => Some(one.read),
                    _ => None,
                },
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

/// **Which body each deleted node continues**, by its own read (the
/// variable its recipe reads), `None` where it continues nothing.
type Dead = BTreeMap<RecipeNodeId, Option<VarId>>;

/// **The live body `var` stands for once `dead` is deleted**: itself, or
/// down the continuations of the deleted operations defining it; `None`
/// where one continues nothing.
fn live<P: crate::ProfilePayload>(doc: &Doc<P>, dead: &Dead, mut var: VarId) -> Option<VarId> {
    let mut seen = BTreeSet::new();
    while let Some(op) = doc.operation_of(var) {
        let Some(&continued) = dead.get(&op) else {
            return Some(var);
        };
        if !seen.insert(op) {
            return None;
        }
        var = continued?;
    }
    Some(var)
}

/// **The name `name` returns to once `dead` is deleted**: while its node
/// is deleted and the name is an entity that node carried in whole from
/// the body it continues (`[From { read, of }]`), the name it carried.
/// Any other name is answered as it is: one a deleted node minted
/// strands.
pub fn carried_back(name: &StableName, continued: impl Fn(RecipeNodeId) -> Option<VarId>) -> StableName {
    let mut name = name.clone();
    let mut steps = 0usize;
    while let Some(read) = continued(name.node) {
        let [RoleSeg::From { read: from, of }] = name.path.as_slice() else {
            break;
        };
        if *from != read || steps > 64 {
            break;
        }
        name = of.name().clone();
        steps += 1;
    }
    name
}

/// [`carried_back`] over a plan's deleted nodes.
fn back(dead: &Dead, name: &StableName) -> StableName {
    carried_back(name, |node| dead.get(&node).copied().flatten())
}

/// **`read` with every body it reads from a deleted operation handed to
/// what that operation continues**, the names of a selection carried
/// back ([`carried_back`]); `None` where a read it holds continues to
/// nothing, or is a named read (shared, so not this reader's to
/// rewrite) that reaches a deleted operation.
fn handed<P: crate::ProfilePayload>(doc: &Doc<P>, dead: &Dead, read: &Operand) -> Option<Operand> {
    let touches = |var: VarId| doc.read_ports(var).iter().any(|(at, _)| dead.contains_key(at));
    let var_read = |var: VarId| -> Option<Operand> {
        match doc.var(var).map(crate::Var::def) {
            Some(VarDef::Output { node, .. }) if dead.contains_key(node) => {
                live(doc, dead, var).map(Operand::Var)
            }
            _ if touches(var) => None,
            _ => Some(Operand::Var(var)),
        }
    };
    match read {
        Operand::Var(var) => var_read(*var),
        Operand::Name(name) => match doc.var_named(name.as_str()) {
            Some(var) if touches(var) => var_read(var),
            _ => Some(read.clone()),
        },
        Operand::Node(node) if dead.contains_key(node) => var_read(doc.read_of_node(*node)?),
        Operand::Output { node, port } if dead.contains_key(node) => {
            var_read(doc.output(*node, *port)?)
        }
        Operand::Node(_) | Operand::Output { .. } => Some(read.clone()),
        Operand::Select { body, names } => {
            let body = handed(doc, dead, body)?;
            Some(Operand::Select {
                body: Box::new(body),
                names: names.iter().map(|name| back(dead, name)).collect(),
            })
        }
        Operand::Pose(def) => {
            let def = def.try_map(
                &mut |_, read| handed(doc, dead, read).ok_or(()),
                &mut |_, scalar: &Formula| Ok(scalar.clone()),
            );
            def.ok().map(|def| Operand::Pose(Box::new(def)))
        }
        Operand::Pick { placement, of } => {
            let placement = handed(doc, dead, placement)?;
            Some(Operand::Pick {
                placement: Box::new(placement),
                of: of.clone(),
            })
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

/// [`Handed`] for `n`, which reads some deleted operation.
fn hand<P: crate::ProfilePayload>(doc: &Doc<P>, dead: &Dead, n: RecipeNodeId, node: &Node<P>) -> Handed {
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
            Some(read) if read.is_indexed() => None,
            Some(read) => live(doc, dead, read.read).map(Operand::Var),
            None => handed(doc, dead, &doc.written_operand(var)),
        };
        match rewritten {
            Some(read) => seats.push((seat, read)),
            None => {
                lost.insert(seat);
            }
        }
    }
    let otherwise = doc.spaces_read(node).iter().any(|(_, at)| dead.contains_key(at))
        || doc
            .upstream(n)
            .into_iter()
            .any(|at| dead.contains_key(&at) && !reached.contains(&at));
    Handed {
        seats,
        lost,
        otherwise,
    }
}

/// **The delete of `id` by continuation, planned over `doc`** (module
/// docs): `None` for an id `doc` does not hold.
///
/// Walking the recipe in evaluation order, each node reading a deleted
/// operation is decided once:
/// - every seat it reads one through is handed to a live body: it stays,
///   those seats rewritten (a list operand through `SetMembers`, any
///   other seat through `SetParam`);
/// - an operator whose lost seats are list operands drops them, and
///   with one operand left it continues that operand and goes, with
///   none it goes;
/// - a cut whose tool is lost continues its target and goes;
/// - anything else goes, continuing nothing.
///
/// The product slot reading a deleted operation is handed the same way.
/// A placement whose source was handed over is followed by a `Rebind`
/// of every name its copy's readers hold that the deleted node carried
/// in, down the copies of that copy.
pub fn splice<P: crate::ProfilePayload + Clone>(
    doc: &Doc<P>,
    id: RecipeNodeId,
) -> Option<Splice<P>> {
    let root = doc.node(id)?;
    let mut dead: Dead = BTreeMap::from([(id, continues(doc, id))]);
    let order = crate::eval::schedule::schedule(doc).order;
    let mut edits: Vec<DocEdit<P>> = Vec::new();
    let mut handed_sources: Vec<RecipeNodeId> = Vec::new();
    for &n in &order {
        if dead.contains_key(&n) {
            continue;
        }
        let Some(node) = doc.node(n) else { continue };
        let depends = doc.upstream(n).iter().any(|at| dead.contains_key(at))
            || doc.spaces_read(node).iter().any(|(_, at)| dead.contains_key(at));
        if !depends {
            continue;
        }
        let Handed {
            seats,
            lost,
            otherwise,
        } = hand(doc, &dead, n, node);
        let rewritten = |seat: OperandSlot| seats.iter().find(|(s, _)| *s == seat).map(|(_, r)| r);
        if otherwise {
            dead.insert(n, None);
            continue;
        }
        let list = match node {
            Node::Union { members } | Node::Intersect { members } | Node::Assemble { members } => {
                match members {
                    Bodies::Spelled(reads) => Some(reads),
                    Bodies::Family(_) => None,
                }
            }
            _ => None,
        };
        if let Some(reads) = list {
            if lost.iter().any(|seat| !matches!(seat, OperandSlot::Member(_))) {
                dead.insert(n, None);
                continue;
            }
            let (Node::Union { members: written }
            | Node::Intersect { members: written }
            | Node::Assemble { members: written }) =
                node.authored(doc)
            else {
                unreachable!("an operator re-authors as itself")
            };
            let Bodies::Spelled(written) = written else {
                unreachable!("a spelled list re-authors spelled")
            };
            let kept: Vec<(VarId, BodyRead<Formula>)> = reads
                .iter()
                .zip(written)
                .enumerate()
                .filter(|(i, _)| !lost.contains(&OperandSlot::Member(*i)))
                .map(|(i, (stored, authored))| match rewritten(OperandSlot::Member(i)) {
                    Some(Operand::Var(var)) => (*var, BodyRead::plain(Operand::Var(*var))),
                    _ => (stored.read, authored),
                })
                .collect();
            match kept.as_slice() {
                [] => {
                    dead.insert(n, None);
                }
                [(one, _)] if !lost.is_empty() => {
                    dead.insert(n, Some(*one));
                }
                _ if seats.is_empty() && lost.is_empty() => {}
                _ => edits.push(DocEdit::SetMembers {
                    node: n,
                    members: Bodies::Spelled(kept.into_iter().map(|(_, read)| read).collect()),
                }),
            }
            continue;
        }
        if let Node::Subtract { from, .. } = node
            && lost.iter().all(|seat| *seat == OperandSlot::Tool)
            && !lost.is_empty()
        {
            let target = match rewritten(OperandSlot::From) {
                Some(Operand::Var(var)) => Some(*var),
                _ => Some(from.read),
            };
            dead.insert(n, target);
            continue;
        }
        if !lost.is_empty() {
            dead.insert(n, None);
            continue;
        }
        for (seat, read) in seats {
            if seat == OperandSlot::Source {
                handed_sources.push(n);
            }
            edits.push(DocEdit::SetParam {
                node: n,
                slot: SlotId::Operand(seat),
                value: SlotValue::Read(read),
                fresh: Vec::new(),
            });
        }
    }
    if let Some(product) = doc.product()
        && doc.operation_of(product).is_some_and(|op| dead.contains_key(&op))
        && let Some(now) = live(doc, &dead, product)
    {
        edits.push(DocEdit::SetProduct {
            product: Some(Operand::Var(now)),
        });
    }
    edits.extend(rebinds(doc, &dead, &handed_sources));
    let unscheduled = root.is_space().then_some(id);
    let deleted = order
        .into_iter()
        .rev()
        .filter(|n| dead.contains_key(n))
        .chain(unscheduled)
        .collect();
    Some(Splice { deleted, edits })
}

/// **The rebinds a hand-over of `placements`' sources owes**: every name
/// a selection of one of their copies holds — down the copies of those
/// copies — that a deleted node carried in, back to the name it carried.
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
            .filter(|&p| doc.copied_read(p).is_some_and(|read| bodies.contains(&read)))
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
        if !bodies.contains(&select.body) {
            continue;
        }
        for name in &select.names {
            let to = back(dead, name);
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

impl<P: crate::ProfilePayload + Clone> Recording<'_, P> {
    /// **Delete `id` by continuation** ([`splice`]), in this action: the
    /// hand-over edits, then the deletes. Answers the nodes deleted.
    ///
    /// # Errors
    ///
    /// The first door that refuses; [`EditError`]'s own refusal of a
    /// delete of a node the document does not hold.
    pub fn delete_by_continuation(&mut self, id: RecipeNodeId) -> Result<Vec<RecipeNodeId>, EditError> {
        let Some(Splice { deleted, edits }) = splice(self.doc(), id) else {
            self.apply(DocEdit::DeleteNode { id })?;
            return Ok(vec![id]);
        };
        for edit in edits {
            self.apply(edit)?;
        }
        for &node in &deleted {
            self.apply(DocEdit::DeleteNode { id: node })?;
        }
        Ok(deleted)
    }
}
