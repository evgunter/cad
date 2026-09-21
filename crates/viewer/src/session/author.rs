//! **The authoring specs and their lowering to nodes**: what one
//! creation form authors, in the vocabulary the form fills in.
//!
//! A VOCABULARY. These hold no session state at all — a spec is a
//! payload the panels build and [`datum_node`] lowers, and the lowering
//! is total for the reason its own note gives.
//!
//! Module kind: **vocabulary** — it names no driver type and no
//! `app`-only crate (`crates/viewer/README.md`, Module boundaries).

use pncad::document::{Datum, Dimension, DimensionError, Expr, Node, ProfileProgram, RecipeNodeId};
use pncad::prelude::StableName;
use pncad::profile::SketchPlane;

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
    /// A sketch frame read off a PICKED FACE: its origin and normal
    /// are the face's, and `spin` is the only number an author
    /// chooses.
    ///
    /// Two PICKS and one field — [`Self::AxisInPlane`]'s shape, not
    /// [`Self::Plane`]'s.
    ///
    /// **`at` is the node whose BODY the ray met, never the feature
    /// that minted the face.** The name is read out of that body's own
    /// table ([`crate::session::FaceSelection::node`] is that node;
    /// `FaceSelection::feature` answers the different question), so a
    /// flat swept by an extrude and shrunk by a later fillet is
    /// authored on the FILLET's body — reading it through the extrude
    /// would name a face of a different size. The two sit beside each
    /// other in `FaceSelection` and the wrong one compiles, which is
    /// why it is said here.
    FaceFrame {
        /// The body-denoting node the face is read out of — a PICK,
        /// and a DAG input exactly as [`Self::AxisInPlane`]'s `plane`
        /// is.
        at: RecipeNodeId,
        /// The picked face, frozen — a PICK, resolved through `at`'s
        /// value under the N5 ladder.
        face: StableName,
        /// The rotation of sketch +x about the outward normal
        /// (`Angle`) — the node's only slot, because the origin and
        /// the normal are read off the face.
        spin: Expr,
    },
}

/// **Which frame an add-profile form draws on**: one that already
/// exists, or the world XY frame the same submit mints.
///
/// A CHOICE ON THE PICK, not a second creation op, because the two
/// arms differ in exactly one thing — whether the id the profile
/// names has to be checked. [`Self::Existing`] is a pick like every
/// other and is gated `WrongNodeKind` at the door;
/// [`Self::NewXy`] names an id the same action minted a line earlier,
/// so there is no pick to be wrong and nothing to gate. A second
/// [`super::SessionOp`] would have re-declared `loops` and the whole
/// insert-door refusal contract beside the one that has it, and would
/// have had to answer the three exhaustive matches over the op
/// vocabulary twice.
///
/// **What [`Self::NewXy`] must not become is an implicit frame.** It
/// inserts an ordinary [`pncad::document::Datum::Frame`] node — visible
/// in the tree, editable in the property panel, pickable by the next
/// profile — and the profile that follows it names it by id. The two
/// inserts are ONE committed action and therefore one undo.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ProfilePlane {
    /// A `Datum::Frame` or `Datum::FaceFrame` node the document
    /// already holds.
    Existing(RecipeNodeId),
    /// The world XY frame, minted by the same action: origin `(0, 0,
    /// 0)`, sketch +x along world +x, sketch +y along world +y.
    NewXy,
}

impl ProfilePlane {
    /// The world XY frame's own numbers, as [`Self::NewXy`] authors
    /// them.
    ///
    /// Spelled here rather than at the form, for [`DatumSpec`]'s
    /// reason: the numbers a form authors are the vocabulary's, and
    /// the session mints the literals. `Length` origin, `Scalar`
    /// axes — the dimensions [`Datum::Frame`]'s own slots take.
    ///
    /// # Errors
    ///
    /// Never, in practice: every component is `0.0` or `1.0`, and
    /// [`Expr::literal`] refuses only a non-finite one. It is a
    /// `Result` so that "is this number authorable" keeps ONE home,
    /// the expression door, rather than an `unwrap` here.
    pub fn world_xy() -> Result<DatumSpec, DimensionError> {
        let length = |value: f64| Expr::literal(value, Dimension::Length);
        let scalar = |value: f64| Expr::literal(value, Dimension::Scalar);
        Ok(DatumSpec::Frame {
            origin: [length(0.0)?, length(0.0)?, length(0.0)?],
            u: [scalar(1.0)?, scalar(0.0)?, scalar(0.0)?],
            v: [scalar(0.0)?, scalar(1.0)?, scalar(0.0)?],
        })
    }

    /// **The placement the chrome draws [`Self::NewXy`] on before its
    /// node exists.**
    ///
    /// A preview needs a plane and there is nothing evaluated to read
    /// one off, so this is the one place the choice's geometry is
    /// stated twice — once as the node's numbers above, once as the
    /// placement here. The kernel's own mint, not a hand-built affine,
    /// and `creation_ops`'s `a_new_xy_frame_lands_where_its_preview_drew`
    /// evaluates [`Self::world_xy`]'s node and holds the two together,
    /// so the pair cannot drift in silence.
    pub fn xy_placement() -> SketchPlane<f64> {
        SketchPlane::xy()
    }
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
pub(crate) fn datum_node(spec: DatumSpec) -> Node<ProfileProgram> {
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
        DatumSpec::FaceFrame { at, face, spin } => Datum::FaceFrame { at, face, spin },
    })
}
