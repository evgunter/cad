//! **A document variable** (`docs/VARIABLES-DESIGN.md`, VR1–VR3): an
//! identity minted from the document's mint chain, a kind fixed at
//! minting, and a definition.
//!
//! The identity is a [`VarId`], minted by `DeclareVar` and never
//! reused. A variable's name ([`crate::VarName`]) is held beside it in
//! the document (`Doc::var_name`), unique within the document, and
//! is not part of the identity: two declares of one definition mint
//! two ids, and nothing that identifies a variable is text.

use crate::doc::FreeVar;
use crate::expr::{Dimension, Expr};

/// **A variable's identity** (VR1): minted from the document's mint
/// chain ([`crate::Mint`]) by `DeclareVar`, never reused (a deleted
/// variable's id stays in the mint log), never positional.
///
/// Its `Display` is `#` and every bit (`#3fa9c1d2a0b1c3d4`), the text
/// a nameless reader unparses to; [`VarId::full`] gives the bits alone.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
pub struct VarId(pub u64);

/// **What a variable holds** (VR3), fixed at minting: a new kind is a
/// new variable.
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
}

impl VarKind {
    /// The expression dimension a reader of this kind reads at.
    #[must_use]
    pub fn dimension(self) -> Dimension {
        match self {
            Self::Length => Dimension::Length,
            Self::Angle => Dimension::Angle,
            Self::Scalar => Dimension::Scalar,
            Self::Count => Dimension::Count,
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

/// The kind as prose reads it: the dimension's word.
impl core::fmt::Display for VarKind {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.dimension())
    }
}

/// **A variable's definition** (VR3): free — a value, its written
/// unit and optionally a distribution — or defined by an expression
/// over other variables, whose dimension is the variable's kind.
///
/// The stored form: a definition's expression reads variables by id
/// alone, as the edit door wrote it. [`VarDecl`] is the authored twin.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum VarDef {
    /// A free variable.
    Free(FreeVar),
    /// A variable defined by an expression over other variables. It
    /// holds no distribution: its uncertainty is the pushforward of
    /// its inputs'.
    Defined(Expr),
}

impl VarDef {
    /// The kind this definition holds.
    #[must_use]
    pub fn kind(&self) -> VarKind {
        match self {
            Self::Free(free) => VarKind::from(free.dim()),
            Self::Defined(expr) => VarKind::from(expr.dim()),
        }
    }

    /// The free variable, when the definition is one.
    #[must_use]
    pub fn free(&self) -> Option<&FreeVar> {
        match self {
            Self::Free(free) => Some(free),
            Self::Defined(_) => None,
        }
    }

    /// The defining expression, when the definition is one.
    #[must_use]
    pub fn defined(&self) -> Option<&Expr> {
        match self {
            Self::Free(_) => None,
            Self::Defined(expr) => Some(expr),
        }
    }

    /// Bit-semantic equality: every float by its bits.
    #[must_use]
    pub fn bit_eq(&self, other: &VarDef) -> bool {
        match (self, other) {
            (Self::Free(a), Self::Free(b)) => a.bit_eq(b),
            (Self::Defined(a), Self::Defined(b)) => a.bit_eq(b),
            (Self::Free(_) | Self::Defined(_), _) => false,
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
    Defined(Expr),
}

impl VarDecl {
    /// A variable defined by `expr`.
    #[must_use]
    pub fn defined(expr: Expr) -> Self {
        Self::Defined(expr)
    }

    /// The kind this definition holds.
    #[must_use]
    pub fn kind(&self) -> VarKind {
        match self {
            Self::Free(free) => VarKind::from(free.dim()),
            Self::Defined(expr) => VarKind::from(expr.dim()),
        }
    }

    /// The stored definition this declares. A name leaf the door did
    /// not lower stays, and the door refuses it.
    #[must_use]
    pub fn stored(&self) -> VarDef {
        match self {
            Self::Free(free) => VarDef::Free(free.clone()),
            Self::Defined(expr) => VarDef::Defined(expr.clone()),
        }
    }

    /// The defining expression, when the declaration is one.
    pub(crate) fn defined_mut(&mut self) -> Option<&mut Expr> {
        match self {
            Self::Free(_) => None,
            Self::Defined(expr) => Some(expr),
        }
    }
}

impl From<FreeVar> for VarDecl {
    fn from(free: FreeVar) -> Self {
        Self::Free(free)
    }
}

impl From<VarDef> for VarDecl {
    /// A stored definition re-authored: what the door stored, which
    /// lowers to itself.
    fn from(def: VarDef) -> Self {
        match def {
            VarDef::Free(free) => Self::Free(free),
            VarDef::Defined(expr) => Self::Defined(expr),
        }
    }
}

/// **A variable**: its kind and its definition. The definition's kind
/// is the variable's ([`Var::new`] is the one way to pair them, and the
/// load door re-checks a file's pairing).
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Var {
    kind: VarKind,
    def: VarDef,
}

impl Var {
    /// The variable `def` defines, of `def`'s own kind.
    #[must_use]
    pub fn new(def: VarDef) -> Self {
        Self {
            kind: def.kind(),
            def,
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
    /// break and no door can.
    pub(crate) fn kind_holds(&self) -> bool {
        self.kind == self.def.kind()
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
            assert_eq!(VarKind::from(dim).dimension(), dim, "{dim}");
        }
        assert_eq!(
            VarDef::Free(FreeVar::Count { value: 3 }).kind(),
            VarKind::Count
        );
    }
}
