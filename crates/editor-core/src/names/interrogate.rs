//! **Name → geometry** (LIB-U5): the forward twin of hit-testing.
//!
//! `resolve/hit.rs` inverts an arena key to a [`StableName`]. This
//! module runs the other way — a name a caller STORED, plus the
//! evaluation it should be read against, to the geometry that name
//! denotes — and it is the door that lets a library consumer ask
//! "where is the face I selected?" without ever holding an arena key.
//!
//! # Why this module has to exist (G1)
//!
//! `EntityRef`/`EntityKey` wrap `topo` arena keys, which are
//! body-lineage-scoped and meaningful only against the evaluation
//! that built the table — `names::table`'s own rule says they never
//! leave editor-core. Before this module the only route from a name
//! to a coordinate was to leak them anyway: index `Evaluation.nodes`,
//! match the payload, index the output body, unwrap the key, then
//! reach into the body's arenas. That is precisely the laundering
//! LIBRARY-DESIGN §L3 says a consumer must never do, and it was the
//! only route on offer. These doors close it: names in, values out,
//! keys confined.
//!
//! # What comes back
//!
//! `topo::readback`'s [`Pose`] — the carrier's own stored frame,
//! copied out. The rules that module states hold verbatim here:
//! values never verdicts (no door answers "is this face planar"),
//! definitional re-reads carry no pad, and no convention is invented
//! where the geometry fixes none. This layer adds only the name
//! resolution and the typed refusals that go with it.

use geom_core::Decide;
use topo::Body;
use topo::readback::{self, Pose, ReadbackError};

use crate::eval::{BooleanValue, Evaluation, NodeResult, SplitSide, ValuePayload};
use crate::names::{EntityKey, EntityKind, Entry, StableName};
use crate::node::RecipeNodeId;

/// **What a name denotes**, without the keys it denotes — the
/// N2 tie made visible at the library surface.
///
/// A tie is a naming success and a REFERENCING failure: the name is
/// well-formed and several entities answer to it equally, so a door
/// that must pick one refuses with [`InterrogateError::Ambiguous`].
/// This type is how a caller finds that out before asking, and it
/// deliberately carries a COUNT rather than the candidates: the
/// candidates are arena keys, and those do not leave this crate.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Denotation {
    /// Exactly one entity answers to this name.
    Unique,
    /// The recorded tie: this many equally-admissible candidates.
    Tied {
        /// How many entities answer to the name.
        candidates: usize,
    },
}

/// Typed refusal of a name→geometry read (closed enum, D4 ¶3).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InterrogateError {
    /// The node has no result in this evaluation (canceled suffix or
    /// a foreign node id).
    NodeNotEvaluated {
        /// The node.
        node: RecipeNodeId,
    },
    /// The node failed; there is no table to read.
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
    /// The node evaluated, and nothing in it answers to this name —
    /// a stale selection (the upstream edit removed what it named) or
    /// a name from another node.
    NoSuchName,
    /// The N2 tie: the name is well-formed and several entities
    /// answer to it, so there is no single geometry to report.
    Ambiguous {
        /// How many entities answer.
        candidates: usize,
    },
    /// The name denotes an entity of a different kind than the door
    /// asked for (a face door handed an edge name).
    WrongKind {
        /// What the door reads.
        wanted: EntityKind,
        /// What the name denotes.
        found: EntityKind,
    },
    /// The name denotes a WHOLE BODY, which has no single frame — ask
    /// about one of its faces, edges, or vertices instead.
    WholeBody,
    /// The node's value carries no bodies at all (a datum, a profile,
    /// a declaration list, or a boolean whose regularized result was
    /// empty), so there is no geometry to read.
    NoBodies {
        /// The payload family, for the message.
        payload: &'static str,
    },
    /// The name's output-body index is not present in this node's
    /// value (an emission/value disagreement, surfaced loudly).
    NoSuchBody {
        /// The index the name carries.
        index: u32,
    },
    /// The geometry read itself refused — see [`ReadbackError`].
    Readback(
        /// The kernel-side refusal, unaltered.
        ReadbackError,
    ),
}

// The human-readable rendering (LIB-DOORS F6 shape): each arm states
// the PROBLEM in the name-door's own vocabulary — the node, the name,
// the kind — plus the recourse where a caller has one. The three
// node-state arms say what the hit-test door's identical arms say,
// because it is the same fact about the same evaluation reached
// through a different door. Kinds render through `EntityKind::noun`,
// never `Debug`, and the `Readback` arm forwards the kernel's own
// words rather than paraphrasing a layer it does not own.
impl core::fmt::Display for InterrogateError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::NodeNotEvaluated { node } => write!(
                f,
                "interrogate: node {} has no result in this evaluation — the name is against a \
                 node this run did not produce (a canceled suffix, or an id from another \
                 document)",
                node.0
            ),
            Self::NodeFailed { node } => write!(
                f,
                "interrogate: node {} failed, so it has no name table to read — fix the node's \
                 own failure before asking about its names",
                node.0
            ),
            Self::NodePoisoned { node, through } => write!(
                f,
                "interrogate: node {} is poisoned by the failure at node {}, so it has no name \
                 table to read — the repair is upstream, at node {}",
                node.0, through.0, through.0
            ),
            Self::NoSuchName => f.write_str(
                "interrogate: nothing in this node answers to that name — the selection is \
                 stale (an upstream edit removed what it named) or the name belongs to another \
                 node",
            ),
            Self::Ambiguous { candidates } => write!(
                f,
                "interrogate: {candidates} entities answer to that name equally well, so there \
                 is no single geometry to report — a tie is recorded, never broken silently"
            ),
            Self::WrongKind { wanted, found } => write!(
                f,
                "interrogate: kind mismatch — this door reads {}, and the name denotes {}; \
                 ask the door for the kind the name actually names",
                wanted.noun(),
                found.noun()
            ),
            Self::WholeBody => f.write_str(
                "interrogate: the name denotes a whole body, which has no single frame — ask \
                 about one of its faces, edges, or vertices instead",
            ),
            Self::NoBodies { payload } => write!(
                f,
                "interrogate: this node's value is a {payload} and carries no bodies at all, so \
                 there is no geometry to read"
            ),
            Self::NoSuchBody { index } => write!(
                f,
                "interrogate: the name carries output-body index {index}, which this node's \
                 value does not have — the emission and the value disagree, so this is a kernel \
                 bug"
            ),
            Self::Readback(error) => write!(f, "interrogate: {error}"),
        }
    }
}

impl core::error::Error for InterrogateError {}

impl From<ReadbackError> for InterrogateError {
    fn from(e: ReadbackError) -> Self {
        Self::Readback(e)
    }
}

/// **How does this name resolve — uniquely, or as a tie?** The
/// referencing question, answered without exposing what it resolves
/// to.
///
/// # Errors
///
/// The node ladder ([`InterrogateError::NodeNotEvaluated`],
/// [`InterrogateError::NodeFailed`], [`InterrogateError::NodePoisoned`])
/// and [`InterrogateError::NoSuchName`].
pub fn denotation<T: Decide>(
    ev: &Evaluation<T>,
    node: RecipeNodeId,
    name: &StableName,
) -> Result<Denotation, InterrogateError> {
    match value_of(ev, node)?.name_table.lookup(name) {
        None => Err(InterrogateError::NoSuchName),
        Some(Entry::Unique(_)) => Ok(Denotation::Unique),
        Some(Entry::Tied(candidates)) => Ok(Denotation::Tied {
            candidates: candidates.len(),
        }),
    }
}

/// **Where is the face I selected?** — the named face's carrier
/// frame, as of THIS evaluation.
///
/// Analytic carriers answer from their stored origin and axes (a
/// definitional re-read: no pad); a NURBS face has no canonical frame
/// and says so.
///
/// # Errors
///
/// Every [`InterrogateError`]: the node ladder, `NoSuchName`,
/// `Ambiguous` for an N2 tie, `WrongKind` for a non-face name, and
/// the wrapped [`ReadbackError`].
pub fn face_frame<T: Decide>(
    ev: &Evaluation<T>,
    node: RecipeNodeId,
    name: &StableName,
) -> Result<Pose<T>, InterrogateError> {
    let (body, key) = entity_of(ev, node, name)?;
    match key {
        EntityKey::Face(f) => Ok(readback::face_pose(body, f)?),
        other => Err(kind_mismatch(EntityKind::Face, other)),
    }
}

/// **Where is the edge I selected?** — the named edge's certified
/// carrier frame, as of THIS evaluation. A straight edge answers with
/// no reference perpendicular (see [`Pose::u_ref`]).
///
/// # Errors
///
/// As [`face_frame`], with `WrongKind` for a non-edge name.
pub fn edge_frame<T: Decide>(
    ev: &Evaluation<T>,
    node: RecipeNodeId,
    name: &StableName,
) -> Result<Pose<T>, InterrogateError> {
    let (body, key) = entity_of(ev, node, name)?;
    match key {
        EntityKey::Edge(e) => Ok(readback::edge_pose(body, e)?),
        other => Err(kind_mismatch(EntityKind::Edge, other)),
    }
}

/// **Where is the vertex I selected?** — the named vertex's stored
/// position, as of THIS evaluation.
///
/// # Errors
///
/// As [`face_frame`], with `WrongKind` for a non-vertex name.
pub fn vertex_position<T: Decide>(
    ev: &Evaluation<T>,
    node: RecipeNodeId,
    name: &StableName,
) -> Result<geom_core::Point3<T>, InterrogateError> {
    let (body, key) = entity_of(ev, node, name)?;
    match key {
        EntityKey::Vertex(v) => Ok(readback::vertex_point(body, v)?),
        other => Err(kind_mismatch(EntityKind::Vertex, other)),
    }
}

/// **Where an entity IS**, in ONE point, for any entity kind — the
/// position the geometric selector's decided
/// [`DatumDistance`](super::GeomPred::DatumDistance) atom measures.
///
/// It nominates nothing new: a vertex answers with its stored
/// position, an edge and a face with their carrier frame ORIGIN —
/// the same points [`vertex_position`], [`edge_frame`] and
/// [`face_frame`] already hand out. The U5 refusals travel with them
/// (a NURBS face has no canonical frame and refuses; a whole-body
/// name has no point at all), which is what keeps an unreadable
/// candidate a typed refusal instead of a silent drop from a result
/// set.
///
/// Takes the resolved `(body, key)` rather than a name, because the
/// tied-name rule (GS-Q4) must measure EVERY candidate of a tie —
/// which the name-level doors deliberately refuse to do.
///
/// # Errors
///
/// [`InterrogateError::WholeBody`] for a body key; the wrapped
/// [`ReadbackError`] for an uncertified or frameless carrier.
pub(crate) fn entity_point<T: Decide>(
    body: &Body<T>,
    key: EntityKey,
) -> Result<geom_core::Point3<T>, InterrogateError> {
    match key {
        EntityKey::Vertex(v) => Ok(readback::vertex_point(body, v)?),
        EntityKey::Edge(e) => Ok(readback::edge_pose(body, e)?.origin),
        EntityKey::Face(f) => Ok(readback::face_pose(body, f)?.origin),
        EntityKey::Body => Err(InterrogateError::WholeBody),
    }
}

/// The kind refusal, with `Body` broken out: a whole body has no
/// frame at all, which is a different fact from "wrong kind of
/// entity".
fn kind_mismatch(wanted: EntityKind, found: EntityKey) -> InterrogateError {
    match found {
        EntityKey::Body => InterrogateError::WholeBody,
        other => InterrogateError::WrongKind {
            wanted,
            found: other.kind(),
        },
    }
}

/// The node's value, or the typed rung of the ladder it failed at
/// (the same ladder `resolve::hit` walks, one direction over).
pub(crate) fn value_of<T: Decide>(
    ev: &Evaluation<T>,
    node: RecipeNodeId,
) -> Result<&crate::eval::NodeValue<T>, InterrogateError> {
    match ev.nodes.get(&node) {
        Some(NodeResult::Ok(v)) => Ok(v),
        Some(NodeResult::Failed(_)) => Err(InterrogateError::NodeFailed { node }),
        Some(NodeResult::Poisoned { through }) => Err(InterrogateError::NodePoisoned {
            node,
            through: *through,
        }),
        None => Err(InterrogateError::NodeNotEvaluated { node }),
    }
}

/// Name → (the body it lives in, the entity within it). The arena key
/// is produced and consumed inside this crate — that confinement is
/// the whole point of the module.
fn entity_of<'a, T: Decide>(
    ev: &'a Evaluation<T>,
    node: RecipeNodeId,
    name: &StableName,
) -> Result<(&'a Body<T>, EntityKey), InterrogateError> {
    let value = value_of(ev, node)?;
    let ent = match value.name_table.lookup(name) {
        None => return Err(InterrogateError::NoSuchName),
        Some(Entry::Unique(e)) => *e,
        Some(Entry::Tied(candidates)) => {
            return Err(InterrogateError::Ambiguous {
                candidates: candidates.len(),
            });
        }
    };
    Ok((output_body(&value.payload, ent.body)?, ent.key))
}

/// The node's output body at `index` — the same body ordering the
/// naming emission used (single-body ops: 0; split: 0 above, 1 below;
/// pattern: the instance index).
pub(crate) fn output_body<T: Decide>(
    payload: &ValuePayload<T>,
    index: u32,
) -> Result<&Body<T>, InterrogateError> {
    let missing = || InterrogateError::NoSuchBody { index };
    let none = |payload| Err(InterrogateError::NoBodies { payload });
    match payload {
        ValuePayload::Body(b) => {
            if index == 0 {
                Ok(b)
            } else {
                Err(missing())
            }
        }
        ValuePayload::Boolean(BooleanValue::Body { body, .. }) => {
            if index == 0 {
                Ok(body)
            } else {
                Err(missing())
            }
        }
        ValuePayload::Boolean(BooleanValue::Empty) => none("empty boolean"),
        ValuePayload::Split { above, below } => match (index, above, below) {
            (0, SplitSide::Body(b), _) | (1, _, SplitSide::Body(b)) => Ok(b),
            (0 | 1, _, _) => Err(missing()),
            _ => Err(missing()),
        },
        ValuePayload::Instances(v) => v.get(index as usize).map(AsRef::as_ref).ok_or_else(missing),
        ValuePayload::Datum(_) => none("datum"),
        ValuePayload::Profile(_) => none("profile"),
        ValuePayload::Declarations(_) => none("declarations"),
        // A12: a mate denotes no body. Interrogating one for geometry
        // is the same category error as interrogating a declaration —
        // and so is interrogating a measurement or its verdict.
        ValuePayload::Mate(_) => none("mate"),
        ValuePayload::Measure { .. } => none("measure"),
        ValuePayload::Assertion(_) => none("assertion"),
    }
}
