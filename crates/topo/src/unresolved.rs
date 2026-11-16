//! **A key that did not resolve** ([`Unresolved`]): the one statement of
//! that state, which every refusal that meets it carries.

use crate::entity::EntityId;

/// A key an operation was reading did not resolve in the body it read.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Unresolved {
    /// The entity's own record is not in its arena.
    Record(EntityId),
    /// The entity's record resolved, and the geometry it holds did not:
    /// a vertex's point, an edge's curve, a face's surface.
    Geometry(EntityId),
    /// A walk from the entity met a dangling key: its own record, or one
    /// the walk reached from it. The entity is the one a caller can find
    /// in the body it holds.
    Walk(EntityId),
}

impl Unresolved {
    /// The entity the read was of.
    #[must_use]
    pub const fn entity(self) -> EntityId {
        match self {
            Self::Record(entity) | Self::Geometry(entity) | Self::Walk(entity) => entity,
        }
    }
}

impl core::fmt::Display for Unresolved {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Record(entity) => write!(f, "{entity} does not resolve"),
            Self::Geometry(entity) => write!(f, "the geometry of {entity} does not resolve"),
            Self::Walk(entity) => {
                write!(f, "a walk from {entity} met a key that does not resolve")
            }
        }
    }
}
