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
    /// **A selection authored at this seat** (D10): the entities `names`
    /// names of the body `body` reads. The door mints one anonymous
    /// selection variable of the seat's kind ([`crate::VarDef::Select`])
    /// and the seat reads it, so a selection authored at two seats is
    /// two variables; a seat shares one by reading it by id or name.
    Select {
        /// The body read.
        body: Box<Operand>,
        /// The entities, by name.
        names: Vec<crate::names::StableName>,
    },
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

    /// The names this operand authors a selection of, its body's
    /// included.
    #[must_use]
    pub fn selected_names(&self) -> Vec<&crate::names::StableName> {
        match self {
            Self::Select { body, names } => {
                let mut held = body.selected_names();
                held.extend(names);
                held
            }
            Self::Node(_) | Self::Output { .. } | Self::Var(_) | Self::Name(_) => Vec::new(),
        }
    }

    /// The selection of `names` in the body `body` reads.
    #[must_use]
    pub fn select(body: impl Into<Operand>, names: Vec<crate::names::StableName>) -> Self {
        Self::Select {
            body: Box::new(body.into()),
            names,
        }
    }
}

impl core::fmt::Display for Operand {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Node(node) => write!(f, "node {node}"),
            Self::Output { node, port } => write!(f, "port {port} of node {node}"),
            Self::Var(var) => write!(f, "{var}"),
            Self::Name(name) => write!(f, "{name}"),
            Self::Select { body, names } => {
                write!(f, "the selection of {} names in {body}", names.len())
            }
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
    /// The body a split cuts.
    Target,
    /// The edges a fillet or a chamfer blends.
    Selection,
    /// The faces a shell opens into rims.
    Open,
    /// The face a face frame is read off.
    Face,
    /// Reference `i` of a measure, in argument order.
    Measured(u8),
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
            Self::Selection => "selection".to_owned(),
            Self::Open => "open faces".to_owned(),
            Self::Face => "face".to_owned(),
            Self::Measured(i) => format!("reference {}", u16::from(i) + 1),
            Self::Tool => "tool".to_owned(),
            Self::A => "first operand".to_owned(),
            Self::B => "second operand".to_owned(),
            Self::Member(i) => format!("member {}", u64::from(i) + 1),
            Self::Input => "input".to_owned(),
            Self::Of => "source".to_owned(),
            Self::Body => "body".to_owned(),
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
            Self::Target | Self::A | Self::B | Self::Member(_) | Self::Body => {
                SlotKind::Is(VarKind::Body)
            }
            Self::Selection => SlotKind::Is(VarKind::Edges),
            Self::Open => SlotKind::Is(VarKind::Faces),
            Self::Face => SlotKind::Is(VarKind::Face),
            Self::Measured(_) => SlotKind::Measured,
            Self::Input | Self::Of => SlotKind::Placeable,
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
/// [`SlotKind::Is`]. One operand seat admits a set of kinds:
/// [`SlotKind::Placeable`] is exactly `{Body, Bodies}` (a placer places
/// one body or a list of them), and [`SlotKind::Measured`]
/// `{Body, Face, Edge, Vertex}` (a measure reads one entity, or a whole
/// body).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum SlotKind {
    /// Exactly this kind: a seat's own, or a scalar slot's dimension.
    Is(VarKind),
    /// `Body` or `Bodies`.
    Placeable,
    /// `Body`, `Face`, `Edge` or `Vertex`.
    Measured,
}

impl SlotKind {
    /// Whether `var` may sit here, by its kind.
    #[must_use]
    pub fn admits(self, var: &crate::Var) -> bool {
        let kind = var.kind();
        match self {
            Self::Is(is) => kind == is,
            Self::Placeable => matches!(kind, VarKind::Body | VarKind::Bodies),
            Self::Measured => matches!(
                kind,
                VarKind::Body | VarKind::Face | VarKind::Edge | VarKind::Vertex
            ),
        }
    }

    /// **The kind a selection authored at this seat is minted at**
    /// ([`Operand::Select`]), given the entity kind its names name:
    /// the seat's own selection kind, or at a seat admitting several,
    /// the singleton of that entity kind. `None` where the seat holds no
    /// selection of it.
    #[must_use]
    pub fn selection_kind(self, entity: crate::names::EntityKind) -> Option<VarKind> {
        use crate::names::EntityKind as E;
        match self {
            Self::Is(kind) => kind.selection().is_some().then_some(kind),
            Self::Measured => match entity {
                E::Face => Some(VarKind::Face),
                E::Edge => Some(VarKind::Edge),
                E::Vertex => Some(VarKind::Vertex),
                E::Body => None,
            },
            Self::Placeable => None,
        }
    }
}

impl core::fmt::Display for SlotKind {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Is(kind) => write!(f, "{} {kind}", crate::sentence::article(&kind.to_string())),
            Self::Placeable => f.write_str("a body or a list of bodies"),
            Self::Measured => f.write_str("a body, a face, an edge or a vertex"),
        }
    }
}

/// A sited reference as a measure seat reads it: a body name read at
/// `at` is that node's body, any other name the selection of it in
/// `at`'s body.
impl From<crate::SitedRef> for Operand {
    fn from(r: crate::SitedRef) -> Self {
        if r.name.kind == crate::names::EntityKind::Body {
            Self::Node(r.at)
        } else {
            Self::select(r.at, vec![r.name])
        }
    }
}
