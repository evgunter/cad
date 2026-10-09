//! **An operand is a read** (D10, "reading is the only dependency"):
//! every operand field of a node holds the [`VarId`] of the variable it
//! reads — an operation's output ([`crate::VarDef::Output`]) — and is
//! addressed by the field it is ([`OperandSlot`], one arm of
//! [`crate::SlotId`]), typed by the kinds the field admits
//! ([`SlotKind`]).
//!
//! What a caller writes is an [`Operand`]: a node, which is sugar for
//! that node's one output (`Operand::Node`), a port spelled out, or a
//! variable by id or by name. The slot door
//! ([`crate::DocEdit::SetParam`] with a [`crate::SlotValue::Read`])
//! lowers it to the id the document stores.

use crate::doc::VarName;
use crate::node::RecipeNodeId;
use crate::var::{VarId, VarKind};

/// **An operand as an author writes it**: what the edit door lowers to
/// the read a node stores.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum Operand {
    /// A node, read through its one output (spec Q5). A node with
    /// several outputs — a revolve's body and axis, a split's two halves
    /// — refuses this spelling whatever the seat
    /// ([`crate::EditError::AmbiguousOutput`], naming its ports): the
    /// read names its port ([`Operand::Output`]).
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
/// position): the operand half of a node's slot vocabulary, addressed
/// as [`crate::SlotId::Operand`].
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
    /// The frame a tube is built in, a profile is drawn on, or an
    /// in-plane axis is written in: the one operand kind a field of
    /// each reads, so one slot (its field is `frame` on a tube and
    /// `plane` on the other two).
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
    /// The body a face frame reads its face out of.
    At,
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
            Self::At => "body".to_owned(),
        }
    }

    /// The kinds this field admits.
    #[must_use]
    pub fn kind(self) -> SlotKind {
        match self {
            Self::Profile | Self::Section(_) | Self::Path => SlotKind::Is(VarKind::Profile),
            Self::Axis => SlotKind::Is(VarKind::Axis),
            Self::Frame => SlotKind::Is(VarKind::Frame),
            Self::Tool => SlotKind::Is(VarKind::Plane),
            Self::Target | Self::A | Self::B | Self::Member(_) | Self::At => {
                SlotKind::Is(VarKind::Body)
            }
            Self::Input | Self::Of => SlotKind::Placeable,
            Self::Measure => SlotKind::Measured,
        }
    }
}

impl core::fmt::Display for OperandSlot {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(&self.label())
    }
}

/// **The kinds a slot admits** ([`crate::SlotId::kind`], total over
/// every slot): what a read in the slot may read, and what an
/// expression in a scalar slot lowers to a read of.
///
/// A scalar slot and an operand seat that holds one kind are
/// [`SlotKind::Is`]. Two operand seats admit a set of kinds:
/// [`SlotKind::Placeable`] is exactly `{Body, Bodies}` (a placer places
/// one body or a list of them), and [`SlotKind::Measured`] is the
/// scalar kinds — those with a [`VarKind::dimension`] — of a variable a
/// measure defines (an operation's output), which is what an assertion
/// bounds.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum SlotKind {
    /// Exactly this kind: a seat's own, or a scalar slot's dimension.
    Is(VarKind),
    /// `Body` or `Bodies`.
    Placeable,
    /// A scalar kind, of a variable an operation defines: a measure's
    /// value.
    Measured,
}

impl SlotKind {
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

impl core::fmt::Display for SlotKind {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Is(kind) => write!(f, "{} {kind}", crate::sentence::article(&kind.to_string())),
            Self::Placeable => f.write_str("a body or a list of bodies"),
            Self::Measured => f.write_str("a measured value"),
        }
    }
}
