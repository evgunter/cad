//! **A document variable** (`docs/VARIABLES-DESIGN.md`, VR1–VR3): an
//! identity minted from the document's mint chain, a kind fixed at
//! minting, and a definition.
//!
//! The identity is a [`VarId`], minted by `DeclareVar`, by an edit's
//! lowering for an anonymous variable, or by `InsertNode` for its node's
//! outputs, and never reused. A variable's name ([`crate::VarName`]) is held beside it in
//! the document (`Doc::var_name`), unique within the document, and
//! is not part of the identity: two declares of one definition mint
//! two ids, and nothing that identifies a variable is text.

use crate::doc::FreeVar;
use crate::expr::{Dimension, Expr};

/// **A variable's identity** (VR1): minted from the document's mint
/// chain ([`crate::Mint`]) by `DeclareVar`, a lowering or an insert's
/// outputs ([`crate::Minted::Var`]), never reused (a deleted variable's
/// id stays in the mint log), ordered as minted ([`crate::MintId`]).
///
/// Its `Display` is `#` and the whole id (`#3:3fa9c1d2a0b1c3d4`), the
/// text a nameless reader unparses to; [`VarId::full`] gives the id
/// alone.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
pub struct VarId(pub crate::MintId);

impl VarId {
    /// The variable id with mint ordinal `ordinal` and digest head
    /// `digest` ([`crate::MintId::new`]).
    #[must_use]
    pub const fn new(ordinal: u32, digest: u64) -> Self {
        Self(crate::MintId::new(ordinal, digest))
    }
}

/// **What a variable holds** (VR3; D10's types), fixed at minting: a
/// new kind is a new variable.
///
/// The scalars are read by expressions at their dimension. The poses
/// and the shapes are reference kinds: no expression reads one, none
/// has a free arm, a unit or a distribution, and only an operation
/// defines one ([`VarDef::Output`]).
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
pub enum VarKind {
    /// A length.
    Length,
    /// An angle.
    Angle,
    /// A dimensionless real.
    Scalar,
    /// An exact integer.
    Count,
    /// A point: a frame known up to rotation about it.
    Point,
    /// A direction: a frame known up to translation and spin about it.
    Direction,
    /// An axis: a frame known up to slide and spin along its line.
    Axis,
    /// A plane: a frame known up to in-plane motion.
    Plane,
    /// A frame.
    Frame,
    /// One body.
    Body,
    /// An ordered list of bodies, whose length is a `Count`.
    Bodies,
    /// A profile.
    Profile,
}

impl VarKind {
    /// The expression dimension a reader of this kind reads at, `None`
    /// for a reference kind, which no expression reads.
    #[must_use]
    pub fn dimension(self) -> Option<Dimension> {
        Some(match self {
            Self::Length => Dimension::Length,
            Self::Angle => Dimension::Angle,
            Self::Scalar => Dimension::Scalar,
            Self::Count => Dimension::Count,
            Self::Point
            | Self::Direction
            | Self::Axis
            | Self::Plane
            | Self::Frame
            | Self::Body
            | Self::Bodies
            | Self::Profile => return None,
        })
    }

    /// **A pose kind's symmetry** (D10, A11 (1)): the family of the
    /// subgroup of rigid motions a value of this kind is a frame known
    /// up to, the one the mates fold. `None` for a kind that is not a
    /// pose, and for `Point` and `Direction`, whose subgroups (rotation
    /// about a point; translation with spin about a direction) the
    /// family does not hold.
    #[must_use]
    pub fn symmetry(self) -> Option<crate::mate::SubgroupFamily> {
        use crate::mate::SubgroupFamily;
        match self {
            Self::Frame => Some(SubgroupFamily::Trivial),
            Self::Plane => Some(SubgroupFamily::Planar),
            Self::Axis => Some(SubgroupFamily::Cylindrical),
            Self::Point
            | Self::Direction
            | Self::Length
            | Self::Angle
            | Self::Scalar
            | Self::Count
            | Self::Body
            | Self::Bodies
            | Self::Profile => None,
        }
    }
}

impl From<Dimension> for VarKind {
    fn from(dim: Dimension) -> Self {
        match dim {
            Dimension::Length => Self::Length,
            Dimension::Angle => Self::Angle,
            Dimension::Scalar => Self::Scalar,
            Dimension::Count => Self::Count,
        }
    }
}

/// The kind as prose reads it: a scalar's dimension word, otherwise
/// the kind's own.
impl core::fmt::Display for VarKind {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self.dimension() {
            Some(dim) => write!(f, "{dim}"),
            None => f.write_str(match self {
                Self::Point => "point",
                Self::Direction => "direction",
                Self::Axis => "axis",
                Self::Plane => "plane",
                Self::Frame => "frame",
                Self::Body => "body",
                Self::Bodies => "list of bodies",
                Self::Profile => "profile",
                Self::Length | Self::Angle | Self::Scalar | Self::Count => {
                    unreachable!("a scalar kind reads at a dimension")
                }
            }),
        }
    }
}

/// **A variable's definition** (VR3; D10): free — a value, its
/// written unit and optionally a distribution — defined by an
/// expression over other variables, whose dimension is the variable's
/// kind, or an output of an operation.
///
/// The stored form: a definition's expression reads variables by id
/// alone, as the edit door wrote it. [`VarDecl`] is the authored twin
/// of the first two; an output has none, because it is never declared,
/// only minted by its operation's insert.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum VarDef {
    /// A free variable.
    Free(FreeVar),
    /// A variable defined by an expression over other variables. It
    /// holds no distribution: its uncertainty is the pushforward of
    /// its inputs'.
    Defined(Expr),
    /// **Port `port` of the operation `node`** ([`crate::Node::outputs`]):
    /// the variable lives exactly as long as its node, and its kind is
    /// the port's.
    Output {
        /// The defining operation.
        node: crate::RecipeNodeId,
        /// Its port, the index into its signature.
        port: u8,
    },
}

impl VarDef {
    /// The kind this definition holds, `None` for an output, whose kind
    /// is its port's.
    #[must_use]
    pub fn kind(&self) -> Option<VarKind> {
        match self {
            Self::Free(free) => Some(VarKind::from(free.dim())),
            Self::Defined(expr) => Some(VarKind::from(expr.dim())),
            Self::Output { .. } => None,
        }
    }

    /// The free variable, when the definition is one.
    #[must_use]
    pub fn free(&self) -> Option<&FreeVar> {
        match self {
            Self::Free(free) => Some(free),
            Self::Defined(_) | Self::Output { .. } => None,
        }
    }

    /// The defining expression, when the definition is one.
    #[must_use]
    pub fn defined(&self) -> Option<&Expr> {
        match self {
            Self::Defined(expr) => Some(expr),
            Self::Free(_) | Self::Output { .. } => None,
        }
    }

    /// The operation and port, when the definition is an output.
    #[must_use]
    pub fn output(&self) -> Option<(crate::RecipeNodeId, u8)> {
        match *self {
            Self::Output { node, port } => Some((node, port)),
            Self::Free(_) | Self::Defined(_) => None,
        }
    }

    /// Bit-semantic equality: every float by its bits.
    #[must_use]
    pub fn bit_eq(&self, other: &VarDef) -> bool {
        match (self, other) {
            (Self::Free(a), Self::Free(b)) => a.bit_eq(b),
            (Self::Defined(a), Self::Defined(b)) => a.bit_eq(b),
            (Self::Output { .. }, Self::Output { .. }) => self == other,
            (Self::Free(_) | Self::Defined(_) | Self::Output { .. }, _) => false,
        }
    }
}

/// **A written definition**: free or defined by an expression — what a
/// declaration or an edit's lowering writes, and every variable's
/// definition but an operation's output, whose kind is its port's. Its
/// kind is its own, so [`Var::written`] pairs it with no other.
#[derive(Debug, Clone, PartialEq)]
pub enum WrittenDef {
    /// A free variable.
    Free(FreeVar),
    /// A variable defined by an expression over other variables.
    Defined(Expr),
}

impl WrittenDef {
    /// The kind this definition holds.
    #[must_use]
    pub fn kind(&self) -> VarKind {
        match self {
            Self::Free(free) => VarKind::from(free.dim()),
            Self::Defined(expr) => VarKind::from(expr.dim()),
        }
    }
}

impl From<WrittenDef> for VarDef {
    fn from(def: WrittenDef) -> Self {
        match def {
            WrittenDef::Free(free) => Self::Free(free),
            WrittenDef::Defined(expr) => Self::Defined(expr),
        }
    }
}

/// **A variable's definition as an edit carries it** — the authored
/// twin of [`VarDef`]. A defining expression may read variables by
/// name; the edit door lowers each name to the variable it names and
/// stores the [`VarDef`].
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum VarDecl {
    /// A free variable.
    Free(FreeVar),
    /// A variable defined by an expression over other variables.
    Defined(crate::Formula),
}

impl VarDecl {
    /// A variable defined by `expr`.
    #[must_use]
    pub fn defined(expr: crate::Formula) -> Self {
        Self::Defined(expr)
    }

    /// The kind this definition holds: a scalar, read at [`Self::dim`].
    #[must_use]
    pub fn kind(&self) -> VarKind {
        VarKind::from(self.dim())
    }

    /// The dimension this definition holds.
    #[must_use]
    pub fn dim(&self) -> Dimension {
        match self {
            Self::Free(free) => free.dim(),
            Self::Defined(expr) => expr.dim(),
        }
    }
}

/// **An entry of an edit's fresh table** (VR6): a variable the edit
/// mints for its formulas to read as [`crate::Formula::fresh`], and the
/// name it is minted under, if any. An entry two readers share is named
/// (VR2): an unnamed variable has exactly one reader.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FreshEntry {
    /// The name, unique within the document; none for an entry one
    /// reader reads.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<crate::VarName>,
    /// The definition.
    pub decl: VarDecl,
}

impl FreshEntry {
    /// An entry minted under `name`.
    #[must_use]
    pub fn named(name: crate::VarName, decl: impl Into<VarDecl>) -> Self {
        Self {
            name: Some(name),
            decl: decl.into(),
        }
    }
}

impl From<VarDecl> for FreshEntry {
    fn from(decl: VarDecl) -> Self {
        Self { name: None, decl }
    }
}

impl From<FreeVar> for FreshEntry {
    fn from(free: FreeVar) -> Self {
        VarDecl::Free(free).into()
    }
}

impl From<FreeVar> for VarDecl {
    fn from(free: FreeVar) -> Self {
        Self::Free(free)
    }
}

impl VarDecl {
    /// A stored definition re-authored: what the door stored, which
    /// lowers to itself. `None` for an output, which no edit declares.
    #[must_use]
    pub fn authored(def: VarDef) -> Option<Self> {
        match def {
            VarDef::Free(free) => Some(Self::Free(free)),
            VarDef::Defined(expr) => Some(Self::Defined(crate::Formula::from(expr))),
            VarDef::Output { .. } => None,
        }
    }
}

/// **A variable**: its kind and its definition. A free or defined
/// variable's kind is its definition's ([`Var::written`] pairs them, and
/// the load door re-checks a file's pairing); an output's is its
/// port's ([`Var::output`], checked at load against the operation's
/// signature).
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Var {
    kind: VarKind,
    def: VarDef,
}

impl Var {
    /// The variable `def` defines, of `def`'s own kind.
    #[must_use]
    pub fn written(def: WrittenDef) -> Self {
        Self {
            kind: def.kind(),
            def: def.into(),
        }
    }

    /// **Port `port` of `node`**, of the port's kind `kind`.
    #[must_use]
    pub fn output(kind: VarKind, node: crate::RecipeNodeId, port: u8) -> Self {
        Self {
            kind,
            def: VarDef::Output { node, port },
        }
    }

    /// The kind, fixed at minting.
    #[must_use]
    pub fn kind(&self) -> VarKind {
        self.kind
    }

    /// The definition.
    #[must_use]
    pub fn def(&self) -> &VarDef {
        &self.def
    }

    /// The free variable, when the definition is one.
    #[must_use]
    pub fn free(&self) -> Option<&FreeVar> {
        self.def.free()
    }

    /// Whether the stored kind is the definition's, which a file can
    /// break and no door can. An output's kind is its port's, which the
    /// load door's `OutputSignature` walk checks.
    pub(crate) fn kind_holds(&self) -> bool {
        self.def.kind().is_none_or(|kind| kind == self.kind)
    }

    /// Bit-semantic equality.
    #[must_use]
    pub fn bit_eq(&self, other: &Var) -> bool {
        self.kind == other.kind && self.def.bit_eq(&other.def)
    }
}

/// **A variable as an edit addresses it**: by id, or by the name the
/// document holds for it. The edit door resolves it.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum VarRef {
    /// By identity.
    Id(VarId),
    /// By name.
    Name(crate::VarName),
}

impl From<VarId> for VarRef {
    fn from(id: VarId) -> Self {
        Self::Id(id)
    }
}

impl From<crate::VarName> for VarRef {
    fn from(name: crate::VarName) -> Self {
        Self::Name(name)
    }
}

/// The address as written: the name, or `variable <tag>`.
impl core::fmt::Display for VarRef {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Id(id) => write!(f, "variable {id}"),
            Self::Name(name) => write!(f, "{name}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{VarDef, VarKind};
    use crate::doc::FreeVar;
    use crate::expr::Dimension;

    #[test]
    fn a_kind_reads_at_the_dimension_it_came_from() {
        for dim in [
            Dimension::Length,
            Dimension::Angle,
            Dimension::Scalar,
            Dimension::Count,
        ] {
            assert_eq!(VarKind::from(dim).dimension(), Some(dim), "{dim}");
        }
        assert_eq!(
            VarDef::Free(FreeVar::Count { value: 3 }).kind(),
            Some(VarKind::Count)
        );
    }
}
