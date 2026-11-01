//! **The world, as the viewer reads it** (A10): which bodies are
//! placed, by which placements, and what a pick on a copy is a pick on.
//!
//! The viewport draws the copies the document's placements (`Node::Place`,
//! and an instance, which is a placement) define, in their document
//! order; the product is what the document's product slot reads, a body
//! of the world. Each placement names the space it copies
//! into (its `target`), and the viewport draws a set of spaces (DI5):
//! the world and every space no placement reads a source body of
//! ([`shown_spaces`]). A placement into a body's own space — a body
//! moved against its own geometry, an operand placed against a combine's
//! first — is drawn through the copy that places that space's body, not
//! on its own. A copy carries its body's names as they are, so a pick
//! on the picture names the body's entity; a gesture that authors
//! against a body reads the placement's body under that name
//! ([`on_body`]).
//!
//! Module kind: **vocabulary** — it names no driver type and no
//! `app`-only crate (`crates/viewer/README.md`, Module boundaries).

use std::collections::BTreeSet;

use pncad::document::{Doc, Node, Operand, ProfileProgram, RecipeNodeId, VarId};
use pncad::prelude::StableName;

/// **The body the drawn copy `placement` places**, or `None` for a node
/// that is not a live drawn copy ([`is_drawn_copy`]) of one shape. An
/// instance places its part's world and reads no body of this
/// document, so it answers `None` too; so does a placement that joins
/// a body's space, which is itself the body a seat reads.
pub fn placed_var(doc: &Doc<ProfileProgram>, placement: RecipeNodeId) -> Option<VarId> {
    if !is_drawn_copy(doc, placement) {
        return None;
    }
    match doc.node(placement)? {
        Node::Place { source, .. } => Some(source.read),
        _ => None,
    }
}

/// **The body `placement` copies**: a `Place`'s source's read; empty
/// for any other node.
fn placed_vars(doc: &Doc<ProfileProgram>, placement: RecipeNodeId) -> Vec<VarId> {
    match doc.node(placement) {
        Some(Node::Place { source, .. }) => vec![source.read],
        _ => Vec::new(),
    }
}

/// **The spaces the picture shows** (DI5; D10 `[placement.spaces]`):
/// the world, and every `new_space()` no placement reads a source body
/// of — a space whose bodies some placement copies is shown through
/// that copy, wherever it lands. Read off the recipe
/// ([`Doc::node_space`]).
///
/// A construction's own space is not shown on its own: the picture
/// draws placements' copies, and a construction is no placement, so a
/// copy placed against it would be drawn without the body it stands
/// on. It is drawn through the placement that copies the construction
/// (an interim: `work/intent/the-viewer-does-not-show-a-constructions-own-space.md`,
/// which also holds the author's show and hide the spec adds to this
/// default set).
pub fn shown_spaces(doc: &Doc<ProfileProgram>) -> BTreeSet<VarId> {
    let placements = doc.placements();
    let read: BTreeSet<VarId> = placements
        .iter()
        .flat_map(|&placement| placed_vars(doc, placement))
        .filter_map(|var| doc.space_of(var))
        .collect();
    placements
        .iter()
        .filter_map(|&placement| doc.node_space(placement))
        .chain([VarId::WORLD])
        .filter(|&space| {
            space == VarId::WORLD
                || (!read.contains(&space)
                    && doc
                        .defined_by(space)
                        .is_some_and(|(node, _)| matches!(doc.node(node), Some(Node::Space))))
        })
        .collect()
}

/// **The construction whose own space `node`'s value lives in**, read
/// off its recipe (D10 `[placement.spaces]`): `None` for a value in the
/// world's space, in a `new_space()`, or in no space. A placement whose
/// target is a construction's space is a relation to that
/// construction's body; a construction answers itself.
pub fn construction_of_space(
    doc: &Doc<ProfileProgram>,
    node: RecipeNodeId,
) -> Option<RecipeNodeId> {
    let space = doc.node_space(node)?;
    let (construction, _) = doc.defined_by(space)?;
    (!matches!(doc.node(construction)?, Node::World | Node::Space)).then_some(construction)
}

/// **Whether `placement` is a copy the picture draws**: a placement
/// whose copy lands in a shown space ([`shown_spaces`]) — the world's,
/// or one no placement reads on. A placement into a construction's
/// space that another placement then copies is a relation between two
/// bodies, drawn through that copy.
pub fn is_drawn_copy(doc: &Doc<ProfileProgram>, placement: RecipeNodeId) -> bool {
    doc.node_space(placement)
        .is_some_and(|space| shown_spaces(doc).contains(&space))
}

/// **The drawn copies**, in document order: [`is_drawn_copy`] over
/// the document's placements, the shown spaces read once.
pub fn drawn_copies(doc: &Doc<ProfileProgram>) -> Vec<RecipeNodeId> {
    let shown = shown_spaces(doc);
    doc.placements()
        .into_iter()
        .filter(|&placement| {
            doc.node_space(placement)
                .is_some_and(|s| shown.contains(&s))
        })
        .collect()
}

/// **The drawn copies of `var`**, in document order: the copies of
/// that body the shown spaces hold.
pub fn placements_of_var(doc: &Doc<ProfileProgram>, var: VarId) -> Vec<RecipeNodeId> {
    drawn_copies(doc)
        .into_iter()
        .filter(|&placement| placed_vars(doc, placement).contains(&var))
        .collect()
}

/// **The drawn copies of any output of `node`**, in document order.
pub fn placements_of(doc: &Doc<ProfileProgram>, node: RecipeNodeId) -> Vec<RecipeNodeId> {
    let outputs = doc.outputs(node);
    drawn_copies(doc)
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
/// and `name`, which a copy carries as its body does.
///
/// A pick whose node is not a drawn copy is already on a body and
/// comes back as it was. A placement whose body is gone keeps its own
/// node: there is no body to read the pick on.
pub fn on_body(
    doc: &Doc<ProfileProgram>,
    node: RecipeNodeId,
    body: u32,
    name: &StableName,
) -> (RecipeNodeId, u32, StableName) {
    let (operation, body) = body_of(doc, node, body);
    (operation, body, name.clone())
}

/// **The body a drawn copy is**: for a placement's copy, the
/// operation defining the body it places and that body's index in the
/// operation's value; any other drawn body as it is.
pub fn body_of(doc: &Doc<ProfileProgram>, node: RecipeNodeId, body: u32) -> (RecipeNodeId, u32) {
    placed_var(doc, node)
        .and_then(|var| read_body(doc, var))
        .map_or((node, body), |on| (on.node, on.body))
}

/// **The drawn body a read is**: the operation defining `var`, and its
/// port as the body's index in that operation's value — a split's
/// second half is its node's body 1. `None` for a variable no operation
/// defines: a placement's piece ([`Doc::pick`]) is a piece of a copy,
/// whose index among the bodies the copy draws the read does not say.
pub fn read_body(doc: &Doc<ProfileProgram>, var: VarId) -> Option<crate::tree::DrawnBody> {
    let (operation, port) = doc.defined_by(var)?;
    Some(crate::tree::DrawnBody::new(operation, u32::from(port)))
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
    // A port is named only where the node holds several outputs: the
    // document's own rule for a read naming a node alone
    // (`Doc::read_of_node`), under which a construction's space is no
    // second output.
    let several = doc.read_of_node(node).is_none() && !doc.outputs(node).is_empty();
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
        Operand::Select { .. } | Operand::Pose(_) | Operand::Pick { .. } => None,
    }
}

/// **The placements reading the body `read` names**: a node
/// named alone reads its one output, a port the output it names.
pub fn placements_of_read(doc: &Doc<ProfileProgram>, read: &Operand) -> Vec<RecipeNodeId> {
    read_var(doc, read)
        .map(|var| placements_of_var(doc, var))
        .unwrap_or_default()
}

/// **The body variable `read` names**: a node named alone reads its
/// one output, a port the output it names. `None` for a selection, a
/// pose or a pick, and for a read the document does not define.
pub fn read_var(doc: &Doc<ProfileProgram>, read: &Operand) -> Option<VarId> {
    match read {
        Operand::Node(node) => doc.read_of_node(*node),
        Operand::Output { node, port } => doc.output(*node, *port),
        Operand::Var(var) => Some(*var),
        Operand::Name(name) => doc.var_named(name.as_str()),
        Operand::Select { .. } | Operand::Pose(_) | Operand::Pick { .. } => None,
    }
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

/// **The bodies the product reads** (D10 `[operations]`): the operands
/// of the `⊔` the product slot reads, in order, or the one body it
/// reads; empty where it reads nothing or a body that is gone.
pub fn product_reads(doc: &Doc<ProfileProgram>) -> Vec<VarId> {
    let Some(var) = doc.product().filter(|&var| doc.operation_of(var).is_some()) else {
        return Vec::new();
    };
    match doc.operation_of(var).and_then(|at| doc.node(at)) {
        Some(Node::Assemble { members }) => members
            .rows()
            .into_iter()
            .map(|(_, read)| read.read)
            .collect(),
        _ => vec![var],
    }
}

/// **The copy a world placement defines**, the body a product can read:
/// its output where `node` is a `Place` or an instance standing in the
/// world; `None` for any other node.
pub fn world_copy(doc: &Doc<ProfileProgram>, node: RecipeNodeId) -> Option<VarId> {
    if !matches!(
        doc.node(node),
        Some(Node::Place { .. } | Node::InstantiatePart { .. })
    ) {
        return None;
    }
    doc.output(node, 0)
        .filter(|&copy| doc.space_of(copy) == Some(VarId::WORLD))
}

/// **The product with `copy` in it**: what it reads, then `copy`, as
/// the bodies a [`crate::session::SessionOp::SetProduct`] says.
pub fn product_with(doc: &Doc<ProfileProgram>, copy: VarId) -> Vec<Operand> {
    product_reads(doc)
        .into_iter()
        .filter(|&read| read != copy)
        .chain([copy])
        .map(Operand::Var)
        .collect()
}

/// **The product with `copy` taken out**: what it reads, less `copy`.
pub fn product_without(doc: &Doc<ProfileProgram>, copy: VarId) -> Vec<Operand> {
    product_reads(doc)
        .into_iter()
        .filter(|&read| read != copy)
        .map(Operand::Var)
        .collect()
}
