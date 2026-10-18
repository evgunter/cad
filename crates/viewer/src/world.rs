//! **The world, as the viewer reads it** (A10): which bodies are
//! placed, by which placements, and what a pick on a copy is a pick on.
//!
//! The product is the copies the document's placements (`Node::Place`,
//! and an instance, which is a placement) define wherever their bundle
//! pins them to the world, in their document order, and the viewport
//! draws exactly those copies. A placement whose mates read a body's
//! geometry joins that body's space (D10) — a body moved against its
//! own geometry, an operand placed against a combine's first — and is
//! no world copy: what the world places is the copy that reads it. A copy's names are its body's,
//! each wrapped in one `Placed` qualifier headed at the placement
//! (`StableName::in_copy`), so a pick on the picture names the copy; a
//! gesture that authors against a body reads the placement's body and
//! the body's own name ([`on_body`]).
//!
//! Module kind: **vocabulary** — it names no driver type and no
//! `app`-only crate (`crates/viewer/README.md`, Module boundaries).

use pncad::document::{Doc, Node, Operand, ProfileProgram, RecipeNodeId, Target, VarId};
use pncad::prelude::StableName;

/// **The body the world copy `placement` places**, or `None` for a node
/// that is not a live world copy ([`is_world_copy`]) of one shape. An
/// instance places its part's world and reads no body of this
/// document, so it answers `None` too; so does a placement that joins
/// a body's space, which is itself the body a seat reads.
pub fn placed_var(doc: &Doc<ProfileProgram>, placement: RecipeNodeId) -> Option<VarId> {
    if !is_world_copy(doc, placement) {
        return None;
    }
    match doc.node(placement)? {
        Node::Place { shapes, .. } => match shapes.as_slice() {
            [shape] => Some(shape.read),
            _ => None,
        },
        _ => None,
    }
}

/// **Every body `placement` copies**: a `Place`'s shapes' reads, in the
/// order written; empty for any other node.
fn placed_vars(doc: &Doc<ProfileProgram>, placement: RecipeNodeId) -> Vec<VarId> {
    match doc.node(placement) {
        Some(Node::Place { shapes, .. }) => shapes.iter().map(|shape| shape.read).collect(),
        _ => Vec::new(),
    }
}

/// **The root of the loose space `node`'s value lives in**, read off
/// its recipe (D10, Spaces), or `None` for the world's: a placement
/// with no mate is a loose copy of its own, one whose mates read the
/// world is the world's, one whose mates read a pose joins the space of
/// the body the pose is read off; a construction is the root of its
/// own space, and any other node joins the first loose space among the
/// operands it reads.
pub fn loose_root(doc: &Doc<ProfileProgram>, node: RecipeNodeId) -> Option<RecipeNodeId> {
    let read_space = |var: VarId| {
        doc.read_ports(var)
            .into_iter()
            .find_map(|(at, _)| loose_root(doc, at))
    };
    let held = doc.node(node)?;
    match held {
        Node::Place { constraints, .. } | Node::InstantiatePart { constraints, .. } => {
            match constraints.iter().find_map(|c| c.mate()) {
                None => Some(node),
                Some(mate) => match &mate.to {
                    Target::World(_) => None,
                    Target::Pose(var) => read_space(*var),
                },
            }
        }
        Node::Extrude { .. }
        | Node::Revolve { .. }
        | Node::Tube { .. }
        | Node::HollowTube { .. }
        | Node::Loft { .. }
        | Node::Sweep { .. } => Some(node),
        other => other
            .operand_rows()
            .into_iter()
            .find_map(|(_, read)| read_space(read)),
    }
}

/// **Whether `placement` is a copy of the world's**: a placement whose
/// space is the world, or a loose copy of its own (one no mate pins
/// yet, which the world gesture made and a mate will pin). A placement
/// that joins a body's space is a relation between two bodies, and
/// copies nothing into the world.
pub fn is_world_copy(doc: &Doc<ProfileProgram>, placement: RecipeNodeId) -> bool {
    match loose_root(doc, placement) {
        None => doc.node(placement).is_some(),
        Some(root) => root == placement,
    }
}

/// **The world's copies**, in document order: [`is_world_copy`] over
/// the document's placements.
pub fn world_copies(doc: &Doc<ProfileProgram>) -> Vec<RecipeNodeId> {
    doc.placements()
        .into_iter()
        .filter(|&placement| is_world_copy(doc, placement))
        .collect()
}

/// **The world copies of `var`**, in document order: the copies of
/// that body the world holds.
pub fn placements_of_var(doc: &Doc<ProfileProgram>, var: VarId) -> Vec<RecipeNodeId> {
    world_copies(doc)
        .into_iter()
        .filter(|&placement| placed_vars(doc, placement).contains(&var))
        .collect()
}

/// **The world copies of any output of `node`**, in document order.
pub fn placements_of(doc: &Doc<ProfileProgram>, node: RecipeNodeId) -> Vec<RecipeNodeId> {
    let outputs = doc.outputs(node);
    world_copies(doc)
        .into_iter()
        .filter(|&placement| {
            placed_vars(doc, placement)
                .iter()
                .any(|var| outputs.contains(var))
        })
        .collect()
}

/// **Whether some placement copies an output of `node`** — what the
/// feature tree's placed badge marks.
pub fn is_placed(doc: &Doc<ProfileProgram>, node: RecipeNodeId) -> bool {
    !placements_of(doc, node).is_empty()
}

/// **A pick on a copy, read on its body**: the operation defining the
/// body `node` places, that body's index within the operation's value,
/// and the body's own name for the entity `name` names on the copy.
///
/// A pick whose node is not a placement, or whose name is not
/// the placement's wrap, is already on a body and comes back as it
/// was. A placement whose body is gone keeps its own node: there is no
/// body to read the pick on.
pub fn on_body(
    doc: &Doc<ProfileProgram>,
    node: RecipeNodeId,
    body: u32,
    name: &StableName,
) -> (RecipeNodeId, u32, StableName) {
    match name.copy_of() {
        Some((placement, own)) if placement == node => {
            let (operation, body) = body_of(doc, node, body);
            (operation, body, own.clone())
        }
        _ => (node, body, name.clone()),
    }
}

/// **The body a drawn copy is**: for a placement's copy, the
/// operation defining the body it places and that body's index in the
/// operation's value; any other drawn body as it is.
pub fn body_of(doc: &Doc<ProfileProgram>, node: RecipeNodeId, body: u32) -> (RecipeNodeId, u32) {
    placed_var(doc, node)
        .and_then(|var| doc.defined_by(var))
        .map_or((node, body), |(operation, port)| {
            (operation, u32::from(port))
        })
}

/// **The node a seat takes for a pick on `node`**: the operation whose
/// body a placement places, so a tool authors against the body
/// and not against its copy; any other node as it is.
pub fn seat_of(doc: &Doc<ProfileProgram>, node: RecipeNodeId) -> RecipeNodeId {
    placed_var(doc, node)
        .and_then(|var| doc.operation_of(var))
        .unwrap_or(node)
}

/// **The read a seat takes for a pick on `node`**: the operation and
/// port whose body a placement places, or `node` itself with the
/// output a viewport pick hit (`body`). The port is kept only for an
/// operation that defines several outputs (a split's halves); one with
/// a single output is read by naming it alone.
pub fn seat_read(
    doc: &Doc<ProfileProgram>,
    node: RecipeNodeId,
    body: Option<u32>,
) -> (RecipeNodeId, Option<u8>) {
    let (node, port) = match placed_var(doc, node).and_then(|var| doc.defined_by(var)) {
        Some((operation, port)) => (operation, Some(port)),
        None => (node, body.and_then(|b| u8::try_from(b).ok())),
    };
    let several = doc.outputs(node).len() > 1;
    (node, port.filter(|_| several))
}

/// **What a feature gesture's result takes over**: the read `target`
/// denotes in a body seat, and the placements reading it. Empty
/// for a target nothing places, which places nothing.
pub fn placements_of_target(doc: &Doc<ProfileProgram>, target: RecipeNodeId) -> Vec<RecipeNodeId> {
    doc.read_of_node(target)
        .map(|var| placements_of_var(doc, var))
        .unwrap_or_default()
}

/// **The output `read` names**: its operation, and the port where the
/// read names one. `None` for a selection, which is no output, and for
/// a variable the document does not define.
pub fn read_output(
    doc: &Doc<ProfileProgram>,
    read: &Operand,
) -> Option<(RecipeNodeId, Option<u8>)> {
    let defined = |var: VarId| {
        doc.var(var)?
            .def()
            .output()
            .map(|(node, port)| (node, Some(port)))
    };
    match read {
        Operand::Node(node) => Some((*node, None)),
        Operand::Output { node, port } => Some((*node, Some(*port))),
        Operand::Var(var) => defined(*var),
        Operand::Name(name) => defined(doc.var_named(name.as_str())?),
        Operand::Select { .. } | Operand::Pose(_) => None,
    }
}

/// **The placements reading the body `read` names**: a node
/// named alone reads its one output, a port the output it names.
pub fn placements_of_read(doc: &Doc<ProfileProgram>, read: &Operand) -> Vec<RecipeNodeId> {
    let var = match read {
        Operand::Node(node) => doc.read_of_node(*node),
        Operand::Output { node, port } => doc.output(*node, *port),
        Operand::Var(var) => Some(*var),
        Operand::Name(name) => doc.var_named(name.as_str()),
        Operand::Select { .. } | Operand::Pose(_) => None,
    };
    var.map(|var| placements_of_var(doc, var))
        .unwrap_or_default()
}

/// **The node a creation made**, out of the ids its action minted: the
/// last one that is not a placement — a creation gesture places
/// what it made after making it, and the placement is the world's, not
/// the thing a label or a form is about — or, for a gesture whose
/// creation IS a placement (the transform's), the first it minted.
pub fn made(doc: &Doc<ProfileProgram>, minted: &[RecipeNodeId]) -> Option<RecipeNodeId> {
    minted
        .iter()
        .rev()
        .copied()
        .find(|&node| !matches!(doc.node(node), Some(Node::Place { .. })))
        .or_else(|| minted.first().copied())
}
