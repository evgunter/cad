//! **The authoring specs and their lowering to nodes**: what one
//! creation form authors, in the vocabulary the form fills in.
//!
//! A VOCABULARY. These hold no session state at all — a spec is a
//! payload the panels build and [`datum_node`] lowers, and the lowering
//! is total for the reason its own note gives.
//!
//! Module kind: **vocabulary** — it names no driver type and no
//! toolkit type (`crates/viewer/README.md`, Module boundaries).

use pncad::document::{Datum, Expr, Node, ProfileProgram, RecipeNodeId};

/// The literal payload of one add-datum form (GAUTH-1): plain numbers
/// in canonical units. The SESSION mints the `Expr` literals and
/// refuses a non-finite component typed
/// ([`super::Refusal::Dimension`]), so no form ever constructs an
/// expression — chrome deals in numbers, the vocabulary in values.
///
/// Component dimensions follow the [`Datum`] slots they land in:
/// origins and positions are Lengths, normals and directions are
/// Scalars (an unnormalized direction; evaluation normalizes or
/// refuses degenerate loudly).
#[derive(Clone, Debug)]
pub enum DatumSpec {
    /// A plane through `origin` with normal `normal`.
    Plane {
        /// Origin components (`Length`).
        origin: [Expr; 3],
        /// Normal components (`Scalar`).
        normal: [Expr; 3],
    },
    /// An axis through `origin` along `direction`.
    Axis {
        /// Origin components (`Length`).
        origin: [Expr; 3],
        /// Direction components (`Scalar`).
        direction: [Expr; 3],
    },
    /// A point at `position`.
    Point {
        /// Position components (`Length`).
        position: [Expr; 3],
    },
    /// An axis written in a sketch frame — a revolve's axis of
    /// revolution.
    ///
    /// `plane` is a PICK, not a field: it names the `Datum::Frame` the
    /// two coordinate pairs are written against, and the pairs are
    /// that frame's own 2-D coordinates. There is no third component,
    /// which is the whole of why a revolve about one cannot leave the
    /// sketch.
    AxisInPlane {
        /// The frame node the axis lives in.
        plane: RecipeNodeId,
        /// Origin components in the frame's coordinates (`Length`).
        origin: [Expr; 2],
        /// Direction components in the frame's coordinates (`Scalar`),
        /// unnormalized — the kernel's `RevolveAxis` normalizes and
        /// refuses a sliver at its own door.
        direction: [Expr; 2],
    },
    /// A sketch frame through `origin`, spanned by `u` and `v`.
    Frame {
        /// Origin components (`Length`).
        origin: [Expr; 3],
        /// Sketch +x components (`Scalar`).
        u: [Expr; 3],
        /// Sketch +y components (`Scalar`), orthogonalized against
        /// `u` at evaluation.
        v: [Expr; 3],
    },
}

/// **The add-profile door's loop vocabulary**, re-exported from the
/// module that owns it.
///
/// It is named by [`super::SessionOp::AddProfile`], so it has to be
/// reachable beside the op; it is DEFINED in [`crate::sketch`],
/// because the PATHS verb set and its lowering are a vocabulary of
/// their own and this file is the crate's accretion case (#1386).
pub use crate::sketch::ProfileShape;

/// The payload of one pattern form (GAUTH-4), beside the other
/// authoring spec for the reason it is here at all: it names what a
/// form authors, plus the node references that are PICKS rather than
/// fields.
///
/// `Explicit` has no arm by the plan's ruling: a list of absolute
/// frames is not a form's job.
///
/// **The fields are `Expr`, not numbers** — see [`super::SessionOp`]'s note on
/// the authoring vocabulary. Not `Copy` in consequence, which an
/// `Expr` cannot be.
#[derive(Clone, Debug, PartialEq)]
pub enum PatternRuleSpec {
    /// Stepped along a direction (`PatternKind::Linear`).
    Linear {
        /// Step direction components (`Scalar`).
        direction: [Expr; 3],
        /// Distance between instances (`Length`).
        spacing: Expr,
    },
    /// Stepped around a datum axis (`PatternKind::Circular`).
    Circular {
        /// The datum-axis node stepped around.
        axis: RecipeNodeId,
        /// Angular step between instances (`Angle`).
        step: Expr,
    },
}

/// Lower one datum spec to its node.
///
/// Total, for [`crate::combine::pattern_node`]'s reason: the components arrive
/// as `Expr`s that were checked at their own construction, and whether
/// each suits the slot it lands in is the edit door's question.
pub(super) fn datum_node(spec: DatumSpec) -> Node<ProfileProgram> {
    Node::Datum(match spec {
        DatumSpec::Plane { origin, normal } => Datum::Plane { origin, normal },
        DatumSpec::Axis { origin, direction } => Datum::Axis { origin, direction },
        DatumSpec::Point { position } => Datum::Point { position },
        DatumSpec::Frame { origin, u, v } => Datum::Frame { origin, u, v },
        DatumSpec::AxisInPlane {
            plane,
            origin,
            direction,
        } => Datum::AxisInPlane {
            plane,
            origin,
            direction,
        },
    })
}
