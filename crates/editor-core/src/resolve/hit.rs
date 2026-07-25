//! Hit-testing inversion (M4 PR 4 spec D4): arena key → [`StableName`]
//! over the M2 PR 6 mesh back-references.
//!
//! The picking chain: the GUI reads a mesh's back-refs
//! (`FacePatch::face`, `BoundaryPolyline::edge`/`start_vertex`/
//! `end_vertex`) to an arena key, then editor-core inverts key → name
//! against the SAME evaluation the mesh came from — the N4 table read
//! backwards. The GUI never sees an arena key: this module's answers
//! are names (plus typed errors), and the inversion is TOTAL for
//! every entity the evaluation exposes — an unnamed entity is an
//! emission bug surfaced loudly as [`HitTestError::Unnamed`], never
//! an `Option::None` to swallow.

use geom_core::Decide;
use topo::{EdgeKey, FaceKey, VertexKey};

use crate::eval::{Evaluation, NodeResult};
use crate::names::{EntityKey, EntityRef, StableName};
use crate::node::RecipeNodeId;

/// Typed hit-test failure (closed; no silent lanes).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HitTestError {
    /// The node has no result in this evaluation (canceled suffix or
    /// a foreign node id).
    NodeNotEvaluated {
        /// The node.
        node: RecipeNodeId,
    },
    /// The node failed; there is no table to invert.
    NodeFailed {
        /// The failed node.
        node: RecipeNodeId,
    },
    /// The node was poisoned by an upstream failure.
    NodePoisoned {
        /// The queried node.
        node: RecipeNodeId,
        /// The nearest failed ancestor.
        through: RecipeNodeId,
    },
    /// THE BUG (spec D4): the node evaluated, but the entity has no
    /// name in its table — a naming-emission totality violation,
    /// surfaced loudly.
    Unnamed {
        /// The queried node.
        node: RecipeNodeId,
        /// The unnamed entity.
        entity: EntityRef,
    },
}

/// Inverts one entity of one node's value to its stable name — the
/// bidirectional table read (N4), total for every key the evaluation
/// exposes.
///
/// # Errors
///
/// [`HitTestError`]: no result / failed / poisoned nodes are typed
/// refusals; an evaluated-but-unnamed entity is the loud
/// [`HitTestError::Unnamed`] bug report.
pub fn entity_name<T: Decide>(
    eval: &Evaluation<T>,
    node: RecipeNodeId,
    entity: EntityRef,
) -> Result<&StableName, HitTestError> {
    let value = match eval.nodes.get(&node) {
        Some(NodeResult::Ok(v)) => v,
        Some(NodeResult::Failed(_)) => return Err(HitTestError::NodeFailed { node }),
        Some(NodeResult::Poisoned { through }) => {
            return Err(HitTestError::NodePoisoned {
                node,
                through: *through,
            });
        }
        None => return Err(HitTestError::NodeNotEvaluated { node }),
    };
    value
        .name_table
        .name_of(&entity)
        .ok_or(HitTestError::Unnamed { node, entity })
}

/// [`entity_name`] for a face patch's back-reference.
///
/// # Errors
///
/// See [`entity_name`].
pub fn face_name<T: Decide>(
    eval: &Evaluation<T>,
    node: RecipeNodeId,
    body: u32,
    face: FaceKey,
) -> Result<&StableName, HitTestError> {
    entity_name(
        eval,
        node,
        EntityRef {
            body,
            key: EntityKey::Face(face),
        },
    )
}

/// [`entity_name`] for a boundary polyline's edge back-reference.
///
/// # Errors
///
/// See [`entity_name`].
pub fn edge_name<T: Decide>(
    eval: &Evaluation<T>,
    node: RecipeNodeId,
    body: u32,
    edge: EdgeKey,
) -> Result<&StableName, HitTestError> {
    entity_name(
        eval,
        node,
        EntityRef {
            body,
            key: EntityKey::Edge(edge),
        },
    )
}

/// [`entity_name`] for a polyline-endpoint vertex back-reference.
///
/// # Errors
///
/// See [`entity_name`].
pub fn vertex_name<T: Decide>(
    eval: &Evaluation<T>,
    node: RecipeNodeId,
    body: u32,
    vertex: VertexKey,
) -> Result<&StableName, HitTestError> {
    entity_name(
        eval,
        node,
        EntityRef {
            body,
            key: EntityKey::Vertex(vertex),
        },
    )
}

/// [`entity_name`] for a whole output body.
///
/// # Errors
///
/// See [`entity_name`].
pub fn body_name<T: Decide>(
    eval: &Evaluation<T>,
    node: RecipeNodeId,
    body: u32,
) -> Result<&StableName, HitTestError> {
    entity_name(
        eval,
        node,
        EntityRef {
            body,
            key: EntityKey::Body,
        },
    )
}
