//! **An operand is a read** (D10, "reading is the only dependency"):
//! every operand field of a node holds the [`VarId`] of the variable it
//! reads — an operation's output ([`crate::VarDef::Output`]) — and is
//! addressed by the field it is ([`OperandSlot`]), typed by the kinds
//! the field admits ([`OperandKind`]).
//!
//! What a caller writes is an [`Operand`]: a node, which is sugar for
//! that node's output in the seat (`Operand::Node`), a port spelled out,
//! or a variable by id or by name. The edit door lowers it to the id
//! the document stores.

use crate::doc::VarName;
use crate::node::RecipeNodeId;
use crate::var::{VarId, VarKind};

/// **An operand as an author writes it**: what the edit door lowers to
/// the read a node stores.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum Operand {
    /// A node, read through its output in this seat: its one output, or
    /// the one output of the seat's kind. A node with several outputs a
    /// seat could read refuses ([`crate::EditError::AmbiguousOutput`]).
    Node(RecipeNodeId),
    /// Port `port` of `node`'s signature.
    Output {
        /// The operation.
        node: RecipeNodeId,
        /// The port, an index into its signature.
        port: u8,
    },
    /// A variable, by id.
    Var(VarId),
    /// A variable, by name.
    Name(VarName),
}

impl From<RecipeNodeId> for Operand {
    fn from(node: RecipeNodeId) -> Self {
        Self::Node(node)
    }
}

impl From<VarId> for Operand {
    fn from(var: VarId) -> Self {
        Self::Var(var)
    }
}

impl From<&RecipeNodeId> for Operand {
    fn from(node: &RecipeNodeId) -> Self {
        Self::Node(*node)
    }
}

impl Operand {
    /// Port `port` of `node`.
    #[must_use]
    pub fn output(node: RecipeNodeId, port: u8) -> Self {
        Self::Output { node, port }
    }
}

impl core::fmt::Display for Operand {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Node(node) => write!(f, "node {node}"),
            Self::Output { node, port } => write!(f, "port {port} of node {node}"),
            Self::Var(var) => write!(f, "{var}"),
            Self::Name(name) => write!(f, "{name}"),
        }
    }
}

/// **An operand field's address** (spec Q7: named by field, never by
/// position), the operand half of a node's slot vocabulary beside
/// [`crate::SlotId`].
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
pub enum OperandSlot {
    /// The profile an extrude, revolve or sweep reads.
    Profile,
    /// Section `i` of a loft.
    Section(u32),
    /// A sweep's path profile.
    Path,
    /// The axis a revolve turns about, or a circular rule's.
    Axis,
    /// The frame a tube is built in.
    Frame,
    /// The body a blend, a shell or a split reshapes.
    Target,
    /// A split's plane.
    Tool,
    /// A boolean's first operand.
    A,
    /// A boolean's second operand.
    B,
    /// Member `i` of a union.
    Member(u32),
    /// What a transform, a pattern or a placed union places.
    Input,
    /// What a part projection picks from.
    Of,
    /// The measure an assertion bounds.
    Measure,
    /// The frame a profile is drawn on, or an in-plane axis is written
    /// in.
    Plane,
    /// The body a face frame reads its face out of.
    At,
    /// The body a world placement places.
    Body,
}

impl OperandSlot {
    /// The field as a reader says it.
    #[must_use]
    pub fn label(self) -> String {
        match self {
            Self::Profile => "profile".to_owned(),
            Self::Section(i) => format!("section {}", u64::from(i) + 1),
            Self::Path => "path".to_owned(),
            Self::Axis => "axis".to_owned(),
            Self::Frame => "frame".to_owned(),
            Self::Target => "target".to_owned(),
            Self::Tool => "tool".to_owned(),
            Self::A => "first operand".to_owned(),
            Self::B => "second operand".to_owned(),
            Self::Member(i) => format!("member {}", u64::from(i) + 1),
            Self::Input => "input".to_owned(),
            Self::Of => "source".to_owned(),
            Self::Measure => "measure".to_owned(),
            Self::Plane => "plane".to_owned(),
            Self::At | Self::Body => "body".to_owned(),
        }
    }

    /// The kinds this field admits.
    #[must_use]
    pub fn kind(self) -> OperandKind {
        match self {
            Self::Profile | Self::Section(_) | Self::Path => OperandKind::Is(VarKind::Profile),
            Self::Axis => OperandKind::Is(VarKind::Axis),
            Self::Frame | Self::Plane => OperandKind::Is(VarKind::Frame),
            Self::Tool => OperandKind::Is(VarKind::Plane),
            Self::Target | Self::A | Self::B | Self::Member(_) | Self::At | Self::Body => {
                OperandKind::Is(VarKind::Body)
            }
            Self::Input | Self::Of => OperandKind::Placeable,
            Self::Measure => OperandKind::Measured,
        }
    }
}

impl core::fmt::Display for OperandSlot {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(&self.label())
    }
}

/// **The kinds an operand field admits.** A seat holds its own kind;
/// a placer places one body or a list of them, and an assertion bounds
/// a measured scalar.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum OperandKind {
    /// Exactly this kind.
    Is(VarKind),
    /// A `Body` or a `Bodies`.
    Placeable,
    /// A scalar an operation defines: a measure's value.
    Measured,
}

impl OperandKind {
    /// Whether `var` may sit here: its kind, and for a measured seat
    /// that an operation defines it.
    #[must_use]
    pub fn admits(self, var: &crate::Var) -> bool {
        let kind = var.kind();
        match self {
            Self::Is(is) => kind == is,
            Self::Placeable => matches!(kind, VarKind::Body | VarKind::Bodies),
            Self::Measured => kind.dimension().is_some() && var.def().output().is_some(),
        }
    }
}

impl core::fmt::Display for OperandKind {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Is(kind) => write!(f, "{} {kind}", crate::sentence::article(&kind.to_string())),
            Self::Placeable => f.write_str("a body or a list of bodies"),
            Self::Measured => f.write_str("a measured value"),
        }
    }
}
