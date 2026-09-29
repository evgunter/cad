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
//! emission bug surfaced loudly as [`UnnamedEntity`], never an
//! `Option::None` to swallow.
//!
//! Two refusals live here because two things can go wrong, at two
//! places. The node's STANDING ([`standing`]) is a fact about the
//! evaluation — no result, failed, poisoned — and settles whether
//! there is a table at all; the LOOKUP ([`lookup`]) reads that table
//! and has one refusal of its own, [`UnnamedEntity`]. The hit-test
//! doors fold both into [`HitTestError`]; a door that only looks names
//! up carries the lookup's refusal by itself, so its type says exactly
//! what can happen there.

use geom_core::Decide;
use topo::{EdgeKey, FaceKey, VertexKey};

use crate::eval::{Evaluation, NodeResult, NodeValue};
use crate::names::{EntityKey, EntityRef, StableName};
use crate::node::RecipeNodeId;

/// **The lookup's one refusal**: the node evaluated, so its table
/// exists, and the table has no name for the entity. Naming emission
/// is total (N4), so this is a kernel bug, reported loudly and never
/// degraded to an `Option::None`.
///
/// It is its own type because it is the whole of what a name LOOKUP
/// can refuse. [`super::pick::NodePick::patch_names`] and
/// [`super::pick::NodePick::boundary_names`] name every drawn entity
/// by a table read, with the node's standing settled once for the
/// call, so their per-entity lane holds exactly this and no hit-test
/// arm; a reader of that lane handles one state, and the sentence it
/// forwards names a lookup, because that is what ran.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UnnamedEntity {
    /// The queried node.
    pub node: RecipeNodeId,
    /// The unnamed entity.
    pub entity: EntityRef,
}

// LIB-DOORS F6: the entity kind renders through `EntityKind::noun`,
// never `Debug` — an arena key is editor-core-private (N4) and means
// nothing to a person — so the sentence names the kind and the body
// index and calls the violation what it is.
impl core::fmt::Display for UnnamedEntity {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "name lookup: node {}'s {} in output body {} evaluated but \
             has no name in its table — naming emission is total, so \
             this is a kernel bug",
            self.node.0,
            self.entity.key.kind().noun(),
            self.entity.body
        )
    }
}

impl core::error::Error for UnnamedEntity {}

/// Typed hit-test failure (closed; no silent lanes).
///
/// **Not `Copy` and not `Eq`**: [`HitTestError::Ambiguous`] carries the
/// tied hits, which is a `Vec` of float geometry. Equality is
/// field-wise, including the parameters and the points
/// ([`super::pick::PickHit`]), because what two refusals being equal
/// means here is that they name the same faces at the same places.
#[derive(Debug, Clone, PartialEq)]
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
    /// The handed evaluation is of another document (DI3, A2a): the
    /// door is a statement about a value of `expected`, and an
    /// evaluation of `found` answers out of another document's name
    /// tables. Node ids are minted per document, so a twin recipe's
    /// evaluation answers every lookup — confidently, about other
    /// geometry — and the refusal comes before any table is read.
    EvaluationOfAnotherDocument {
        /// The document the door is a statement about.
        expected: crate::ident::DocumentId,
        /// The document the handed evaluation is of.
        found: crate::ident::DocumentId,
    },
    /// **The certified tie between FACES**: the survivors of the
    /// interval order ([`super::pick::TSpan::precedes`]) name more
    /// than one face, so the geometry does not say which face the ray
    /// met and no second key invents one ([`super::pick::pick_face`]).
    ///
    /// Every hit here is TRUE — each is its own face's hull interval
    /// and a point of that face — and the list is complete: the
    /// traversal's early-out drops only candidates the order already
    /// dropped, so every face of the tie is present. The list's order
    /// (the caller's target order, then face-arena order) is an order
    /// for a LIST and decides nothing.
    ///
    /// Survivors naming ONE face are not this: a ray across a
    /// triangle diagonal or an in-face shared edge answers that face,
    /// which is what keeps the refusal off the picks whose answer is
    /// clear.
    Ambiguous {
        /// The tied faces' hits, one per face, at least two.
        hits: Vec<super::pick::PickHit>,
    },
    /// THE BUG (spec D4): the node evaluated, but the entity has no
    /// name in its table — the lookup's own refusal, carried whole.
    Unnamed(UnnamedEntity),
}

/// The lookup's refusal, at the door that ran a hit test first.
impl From<UnnamedEntity> for HitTestError {
    fn from(unnamed: UnnamedEntity) -> Self {
        Self::Unnamed(unnamed)
    }
}

/// **The pairing predicate's finding, in this door's vocabulary.**
///
/// A2a's rule is one predicate (`ident::mispaired`) and one arm per
/// error type over it. The projection lives HERE, at the type that
/// owns the arm, so a door that runs the predicate writes `?` or
/// `m.into()` and no site re-spells which field goes where.
impl From<crate::ident::Mispaired> for HitTestError {
    fn from(m: crate::ident::Mispaired) -> Self {
        Self::EvaluationOfAnotherDocument {
            expected: m.expected,
            found: m.found,
        }
    }
}

// LIB-DOORS F6: the human-readable rendering a consumer prints instead
// of composing a sentence about somebody else's refusal. Each arm
// states the PROBLEM in this layer's vocabulary — which node, and what
// about it makes the inversion impossible — plus the recourse where a
// user has one. The `Unnamed` arm forwards the lookup's own sentence
// under this door's prefix: the hit test ran, and its lookup refused.
impl core::fmt::Display for HitTestError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::NodeNotEvaluated { node } => write!(
                f,
                "hit test: node {} has no result in this evaluation — \
                 the pick names a node this run did not produce (a \
                 canceled suffix, or an id from another document)",
                node.0
            ),
            Self::NodeFailed { node } => write!(
                f,
                "hit test: node {} failed, so it has no name table to \
                 invert — fix the node's own failure before picking \
                 against it",
                node.0
            ),
            Self::NodePoisoned { node, through } => write!(
                f,
                "hit test: node {} is poisoned by the failure at node \
                 {}, so it has no name table to invert — the repair is \
                 upstream, at node {}",
                node.0, through.0, through.0
            ),
            Self::EvaluationOfAnotherDocument { expected, found } => write!(
                f,
                "hit test: the evaluation is of document {found}, not \
                 of document {expected} — the index and the tables it \
                 is read against are of two documents"
            ),
            Self::Ambiguous { hits } => {
                write!(
                    f,
                    "hit test: the ray is tied between {} faces the arithmetic cannot order — ",
                    hits.len()
                )?;
                // The ordinal is what ties each phrase to its entry
                // in `hits`, where the role path two faces of one node
                // differ by IS carried.
                for (i, hit) in hits.iter().enumerate() {
                    if i > 0 {
                        f.write_str(", ")?;
                    }
                    write!(f, "({}) {}", i + 1, hit.name)?;
                }
                write!(
                    f,
                    " — so the pick names none of them; aim away from the shared edge, or \
                     choose one of the tied faces, which this refusal lists in full"
                )
            }
            Self::Unnamed(unnamed) => write!(f, "hit test: {unnamed}"),
        }
    }
}

impl core::error::Error for HitTestError {}

/// One node's `Ok` value, or the standing refusal that says why there
/// is none — the ladder every door of this module and of
/// [`super::pick`] climbs, written once: two spellings of one
/// standing vocabulary is how two doors come to disagree about a
/// poisoned node.
///
/// # Errors
///
/// [`HitTestError::NodeNotEvaluated`], [`HitTestError::NodeFailed`]
/// or [`HitTestError::NodePoisoned`], per the node's standing.
pub(super) fn standing<T: Decide>(
    eval: &Evaluation<T>,
    node: RecipeNodeId,
) -> Result<&NodeValue<T>, HitTestError> {
    match eval.nodes.get(&node) {
        Some(NodeResult::Ok(value)) => Ok(value),
        Some(NodeResult::Failed(_)) => Err(HitTestError::NodeFailed { node }),
        Some(NodeResult::Poisoned { through }) => Err(HitTestError::NodePoisoned {
            node,
            through: *through,
        }),
        None => Err(HitTestError::NodeNotEvaluated { node }),
    }
}

/// One entity's name out of one node's table — the lookup itself,
/// with the node's standing already settled by [`standing`].
///
/// # Errors
///
/// [`UnnamedEntity`], the lookup's one refusal.
pub(super) fn lookup<T: Decide>(
    value: &NodeValue<T>,
    node: RecipeNodeId,
    entity: EntityRef,
) -> Result<&StableName, UnnamedEntity> {
    value
        .name_table
        .name_of(&entity)
        .ok_or(UnnamedEntity { node, entity })
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
    Ok(lookup(standing(eval, node)?, node, entity)?)
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
