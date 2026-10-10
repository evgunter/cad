//! **The world, as the viewer reads it** (A10): which bodies are
//! placed, by which placements, and what a pick on a copy is a pick on.
//!
//! The product is the copies the document's world placements
//! (`Node::PlaceInWorld`) define, in their document order, and the
//! viewport draws exactly those copies. A copy's names are its body's,
//! each wrapped in one `Placed` qualifier headed at the placement
//! (`StableName::in_copy`), so a pick on the picture names the copy; a
//! gesture that authors against a body reads the placement's body and
//! the body's own name ([`on_body`]).
//!
//! Module kind: **vocabulary** — it names no driver type and no
//! `app`-only crate (`crates/viewer/README.md`, Module boundaries).

use pncad::document::{Doc, Node, ProfileProgram, RecipeNodeId, VarId};
use pncad::prelude::StableName;

/// **The body `placement` places**, or `None` for a node that is not a
/// live world placement.
pub fn placed_var(doc: &Doc<ProfileProgram>, placement: RecipeNodeId) -> Option<VarId> {
    match doc.node(placement)? {
        Node::PlaceInWorld { body, .. } => Some(body.read),
        _ => None,
    }
}

/// **The world placements reading `var`**, in document order: the
/// copies of that body the product holds.
pub fn placements_of_var(doc: &Doc<ProfileProgram>, var: VarId) -> Vec<RecipeNodeId> {
    doc.placements()
        .into_iter()
        .filter(|&placement| placed_var(doc, placement) == Some(var))
        .collect()
}

/// **The world placements reading any output of `node`**, in document
/// order.
pub fn placements_of(doc: &Doc<ProfileProgram>, node: RecipeNodeId) -> Vec<RecipeNodeId> {
    let outputs = doc.outputs(node);
    doc.placements()
        .into_iter()
        .filter(|&placement| placed_var(doc, placement).is_some_and(|var| outputs.contains(&var)))
        .collect()
}

/// **Whether some world placement reads an output of `node`** — what
/// the feature tree's world badge marks.
pub fn is_placed(doc: &Doc<ProfileProgram>, node: RecipeNodeId) -> bool {
    !placements_of(doc, node).is_empty()
}

/// **A pick on a copy, read on its body**: the operation defining the
/// body `node` places, that body's index within the operation's value,
/// and the body's own name for the entity `name` names on the copy.
///
/// A pick whose node is not a world placement, or whose name is not
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

/// **The body a drawn copy is**: for a world placement's copy, the
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
/// body a world placement places, so a tool authors against the body
/// and not against its copy; any other node as it is.
pub fn seat_of(doc: &Doc<ProfileProgram>, node: RecipeNodeId) -> RecipeNodeId {
    placed_var(doc, node)
        .and_then(|var| doc.operation_of(var))
        .unwrap_or(node)
}

/// **What a feature gesture's result takes over**: the read `target`
/// denotes in a body seat, and the world placements reading it. Empty
/// for a target nothing places, which places nothing.
pub fn placements_of_target(doc: &Doc<ProfileProgram>, target: RecipeNodeId) -> Vec<RecipeNodeId> {
    doc.read_of_node(target)
        .map(|var| placements_of_var(doc, var))
        .unwrap_or_default()
}

/// **The node a creation made**, out of the ids its action minted: the
/// last one that is not a world placement — a creation gesture places
/// what it made after making it, and the placement is the world's, not
/// the thing a label or a form is about.
pub fn made(doc: &Doc<ProfileProgram>, minted: &[RecipeNodeId]) -> Option<RecipeNodeId> {
    minted
        .iter()
        .rev()
        .copied()
        .find(|&node| !matches!(doc.node(node), Some(Node::PlaceInWorld { .. })))
}
