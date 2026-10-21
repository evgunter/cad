//! **The authoring specs and their lowering to nodes**: what one
//! creation form authors, in the vocabulary the form fills in.
//!
//! A VOCABULARY. These hold no session state at all — a spec is a
//! payload the panels build and the session lowers.
//!
//! Module kind: **vocabulary** — it names no driver type and no
//! `app`-only crate (`crates/viewer/README.md`, Module boundaries).

use pncad::document::AuthoredNode;
use pncad::document::{Bodies, Formula, Node, Operand};
use pncad::topo::BooleanOp;

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
/// form authors.
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
        direction: [Formula; 3],
        /// Distance between instances (`Length`).
        spacing: Formula,
    },
    /// Stepped around an axis (`PatternKind::Circular`): the line
    /// through `through` along `direction`, in the patterned body's
    /// own coordinates, read off its geometry at the numbers' nominal
    /// values.
    Circular {
        /// A point on the axis (`Length`).
        through: [Formula; 3],
        /// The axis direction (`Scalar`).
        direction: [Formula; 3],
        /// Angular step between instances (`Angle`).
        step: Formula,
    },
}

/// **Which copy of a pattern one part form selects** — the
/// authoring counterpart of [`pncad::document::PartSelect`], beside
/// [`PatternRuleSpec`] for its reason: it names what a form authors,
/// in the numbers a form holds.
///
/// **The index is an `i64`, not an `Expr`**, which is the whole
/// difference from the node's own enum and the reason this type
/// exists. `SlotId::Instance` is Count-typed and STRUCTURAL (spec D3),
/// exactly as `SlotId::Count` is: it is edited afterwards through
/// `SetStructuralParam` and never through the continuous door, so an
/// authoring door that carried an `Expr` would be the one place a
/// structural slot could be written continuously. The session mints
/// the `Formula::count` literal ([`crate::combine::part_node`]), which is
/// the same division of labour [`super::SessionOp::AddPattern`]'s
/// count already takes.
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PartSelectSpec {
    /// The `i`-th instance of a `Node::Pattern` value.
    Instance(i64),
}

/// **What one boolean authors**: the operation and its operands, in
/// the shape the node it lowers to takes ([`BooleanSpec::node`]).
///
/// Union and intersect take a LIST — any count is a document the door
/// accepts (one member builds that body, none the typed empty body,
/// and a node spelled twice glues) — while subtract takes exactly two
/// operands that are not interchangeable: `from` is the body KEPT and
/// `tool` the body REMOVED. Three arms rather than an operation beside
/// a list, so a subtraction of the wrong count has no spelling.
#[derive(Clone, Debug, PartialEq)]
pub enum BooleanSpec {
    /// The material in any member, members in fold order.
    Union(Vec<Operand>),
    /// The material in every member, members in fold order.
    Intersect(Vec<Operand>),
    /// `from` with `tool` cut away.
    Subtract {
        /// The body kept.
        from: Operand,
        /// The body removed.
        tool: Operand,
    },
}

impl BooleanSpec {
    /// The operation — the kernel's enum, the boolean form's choice.
    pub fn op(&self) -> BooleanOp {
        match self {
            Self::Union(_) => BooleanOp::Union,
            Self::Intersect(_) => BooleanOp::Intersect,
            Self::Subtract { .. } => BooleanOp::Subtract,
        }
    }

    /// Every operand, in the node's order: the members, or `from`
    /// then `tool`.
    pub fn operands(&self) -> Vec<Operand> {
        match self {
            Self::Union(members) | Self::Intersect(members) => members.clone(),
            Self::Subtract { from, tool } => vec![from.clone(), tool.clone()],
        }
    }

    /// **The same operation over `operands`**, in the node's order —
    /// [`Self::operands`]' inverse, for a door that reads an operand
    /// through a placement.
    ///
    /// # Panics
    ///
    /// A subtraction given other than two operands: the list is always
    /// one [`Self::operands`] answered, re-read.
    #[must_use]
    pub fn with_operands(&self, operands: Vec<Operand>) -> Self {
        match self {
            Self::Union(_) => Self::Union(operands),
            Self::Intersect(_) => Self::Intersect(operands),
            Self::Subtract { .. } => match <[Operand; 2]>::try_from(operands) {
                Ok([from, tool]) => Self::Subtract { from, tool },
                Err(_) => unreachable!("a subtraction's operands are two"),
            },
        }
    }

    /// Lower to the node.
    pub fn node(&self) -> AuthoredNode {
        let spelled = |members: &[Operand]| {
            Bodies::Spelled(members.iter().cloned().map(Into::into).collect())
        };
        match self {
            Self::Union(members) => Node::Union {
                members: spelled(members),
            },
            Self::Intersect(members) => Node::Intersect {
                members: spelled(members),
            },
            Self::Subtract { from, tool } => Node::Subtract {
                from: from.clone().into(),
                tool: tool.clone().into(),
            },
        }
    }
}
