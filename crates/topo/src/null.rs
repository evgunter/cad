//! Null-entity scaffolding: typed attributes and the null-edge lane
//! (M3 PR 1, fork F9 of `docs/M3-PLAN.md`).
//!
//! Ch. 14/15's splitting and boolean pipelines manufacture **null
//! entities** — zero-length edges and two-loop "null faces" holding
//! section-polygon copies — as mandatory mid-operation scaffolding. The
//! book encodes which side such an entity faces in half-edge slot
//! position (`he1`/`he2`) and list position (`floops`); both notes flag
//! that encoding as a mirror-bug farm. F9's ratified answer: **which
//! side a null entity faces is DATA** — typed attributes
//! ([`NullEdge`], [`NullFacePair`]), correspondence by explicit keys,
//! never index coincidence.
//!
//! # The null-edge representation (ratified shape)
//!
//! A null edge is a **distinct scaffolding representation, not a
//! relaxed certification**: its curve-arena entry is
//! [`CurveGeom::NullScaffold`] — carrying the F9 attribute and *no
//! carrier at all* — rather than a certified
//! [`EdgeCurve`](geom_brep::EdgeCurve) with a degenerate interval. The
//! forward-span certification gate (M2 PR 3: a certified interval's
//! arc length is definitely positive) is untouched; zero length is
//! representable only *by type*, and the type is transient:
//!
//! - **Tier 1 accepts** null entities (mid-op states are legal, as for
//!   every other scaffolding shape — empty loops, struts).
//! - **Tier 2 refuses** them at rest, by name
//!   ([`crate::ValidationError::NullEdgeAtRest`] /
//!   [`crate::ValidationError::NullFaceAtRest`]): a body carrying null
//!   entities is mid-surgery and never crosses an API boundary at rest.
//! - Every consumer that needs a real carrier meets the sum type
//!   [`CurveGeom`] and must handle the scaffolding variant explicitly
//!   (fail-loud at the type level; there is no accessor that silently
//!   converts a null edge into geometry).
//!
//! # Null faces
//!
//! A null face (the completed section polygon: one face, two coincident
//! loops) is an ordinary [`Face`](crate::Face) — its two loops and its
//! surface slot are real topology — so its null-ness is a typed
//! **annotation**, stored in a side table on the body
//! ([`crate::Body::null_face_pair`]) and maintained by the same
//! kill-op hygiene as provenance records (a record never outlives its
//! face). The asymmetry with edges is deliberate: an edge's null-ness
//! *replaces* its geometry (no carrier exists, by type), while a
//! face's null-ness *annotates* loop roles on an otherwise complete
//! face.
//!
//! # Consumers (one line each, per the M3 doc convention)
//!
//! - [`Body::mev_null`](crate::Body::mev_null) serves `splitclassify` /
//!   `separ1`-`separ2` null-edge insertion (M3 PRs 2 and 4).
//! - [`NullFacePair`] serves `splitconnect`'s completed section
//!   polygons and `setopfinish`'s in/out copies (M3 PRs 3 and 5).

use geom_brep::EdgeCurve;
use geom_core::Real;

use crate::body::Body;
use crate::entity::{EntityId, FaceKey, LoopKey, VertexKey};
use crate::euler::{EulerOpError, MevCreated, MevSite};
use crate::provenance::Provenance;

/// The F9 typed attribute of a null (zero-length) edge: **which side
/// each end faces is data**. The two end vertices are geometrically
/// coincident copies; `below_end` belongs to the below/IN side of the
/// splitting surface, `above_end` to the above/OUT side (the boolean
/// lane reads below ≙ IN, above ≙ OUT — one attribute, two readings,
/// documented at the PR 4 call sites).
///
/// Stored inside the edge's curve-arena entry
/// ([`CurveGeom::NullScaffold`]) — a null edge has no carrier, and the
/// attribute is what it has instead. Coherence contract (deliberately
/// *not* a tier-1 check: Euler surgery on a neighborhood legitimately
/// rewires half-edge starts mid-sequence, and the minting/consuming
/// ops of PRs 2–5 own the attribute's currency): `{below_end,
/// above_end}` name the edge's two end vertices as minted.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NullEdge {
    /// The end vertex on the below (splitting) / IN (boolean) side.
    pub below_end: VertexKey,
    /// The end vertex on the above (splitting) / OUT (boolean) side.
    pub above_end: VertexKey,
}

/// The F9 typed attribute of a null face: which of its two loops plays
/// which role — never derived from `outer`-vs-ring designation or list
/// position.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NullFacePair {
    /// Ch. 14 splitting: the two copies of a section polygon.
    Split {
        /// The loop bounding the above part.
        above_loop: LoopKey,
        /// The loop bounding the below part.
        below_loop: LoopKey,
    },
    /// Ch. 15 booleans: the IN/OUT copies of a seam polygon.
    Boolean {
        /// The loop of the IN component's copy.
        in_copy: LoopKey,
        /// The loop of the OUT component's copy.
        out_copy: LoopKey,
    },
}

impl NullFacePair {
    /// The two role loops in declaration order (above/in first).
    pub fn loops(self) -> [LoopKey; 2] {
        match self {
            Self::Split {
                above_loop,
                below_loop,
            } => [above_loop, below_loop],
            Self::Boolean { in_copy, out_copy } => [in_copy, out_copy],
        }
    }
}

/// Which side the **new** vertex of a [`Body::mev_null`] call faces —
/// the caller's declaration, recorded into the minted [`NullEdge`]
/// attribute (the old vertex takes the other side).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NewVertexSide {
    /// The new vertex is the above/OUT copy; the old vertex is
    /// below/IN.
    Above,
    /// The new vertex is the below/IN copy; the old vertex is
    /// above/OUT.
    Below,
}

/// A curve-arena element: a certified carrier, or the typed null-edge
/// scaffolding state (module docs — the F9/forward-span design point).
///
/// The sum lives at the arena so that *no carrier at all* is
/// representable without weakening [`EdgeCurve`]'s
/// certified-only-constructible invariant: consumers that need real
/// geometry match on this type and handle the scaffolding variant
/// explicitly (typically by refusing with a typed error — tier 2 has
/// already banned null entities from every at-rest body they should
/// legitimately see).
#[derive(Clone, Copy, Debug)]
pub enum CurveGeom<T: Real> {
    /// A certified edge carrier (the only at-rest state; D4 ¶2).
    Certified(EdgeCurve<T>),
    /// M3 null-edge scaffolding: no carrier **by type**; the payload is
    /// the F9 side attribute. Transient — tier 2 refuses it at rest.
    NullScaffold(NullEdge),
}

impl<T: Real> CurveGeom<T> {
    /// The certified carrier, if this entry is one (`None` for null
    /// scaffolding — callers decide loudly what that means for them).
    pub fn certified(&self) -> Option<&EdgeCurve<T>> {
        match self {
            Self::Certified(curve) => Some(curve),
            Self::NullScaffold(_) => None,
        }
    }

    /// The null-edge attribute, if this entry is scaffolding.
    pub fn null_scaffold(&self) -> Option<&NullEdge> {
        match self {
            Self::Certified(_) => None,
            Self::NullScaffold(attr) => Some(attr),
        }
    }
}
