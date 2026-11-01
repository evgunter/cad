//! **A document variable** (`crates/editor-core/VARIABLES.md`, VR1–VR3): an
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
    /// **The world's space** (D10 `[placement.world]`): the one output
    /// of [`crate::Node::World`], which every document holds from its
    /// creation. Ordinal zero, which the mint never draws, so no minted
    /// variable can carry it.
    pub const WORLD: Self = Self::new(0, 0);

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
/// The scalars are read by expressions at their dimension. The poses,
/// the shapes and the selections are reference kinds: no expression
/// reads one, and none has a free arm, a unit or a distribution. Only an
/// operation defines a shape ([`VarDef::Output`]); a pose is an
/// operation's output or a pose definition ([`VarDef::Pose`]), never
/// free; a selection is defined by naming entities of one body
/// ([`VarDef::Select`]).
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
    /// One face of a body ([`VarDef::Select`]).
    Face,
    /// One edge of a body ([`VarDef::Select`]).
    Edge,
    /// One vertex of a body ([`VarDef::Select`]): a measure's point.
    Vertex,
    /// A set of faces of one body ([`VarDef::Select`]).
    Faces,
    /// A set of edges of one body ([`VarDef::Select`]).
    Edges,
    /// **A space** (D10 `[variables.kinds]`, `[placement.spaces]`): a
    /// coordinate system, holding nothing. Only minted, as an output:
    /// the world's ([`crate::Node::World`]), a `new_space()`'s
    /// ([`crate::Node::Space`]) or a construction's own. A placement's
    /// target is the one slot that reads one.
    Space,
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
            | Self::Profile
            | Self::Face
            | Self::Edge
            | Self::Vertex
            | Self::Faces
            | Self::Edges
            | Self::Space => return None,
        })
    }

    /// **The entity kind a selection of this kind names**, and how
    /// many names it holds ([`Select`]): `None` for a kind that is not
    /// a selection.
    #[must_use]
    pub fn selection(self) -> Option<(crate::names::EntityKind, SelectionCount)> {
        use crate::names::EntityKind;
        use SelectionCount::{Any, AtLeastOne, One};
        match self {
            Self::Face => Some((EntityKind::Face, One)),
            Self::Edge => Some((EntityKind::Edge, One)),
            Self::Vertex => Some((EntityKind::Vertex, One)),
            Self::Faces => Some((EntityKind::Face, Any)),
            Self::Edges => Some((EntityKind::Edge, AtLeastOne)),
            Self::Length
            | Self::Angle
            | Self::Scalar
            | Self::Count
            | Self::Point
            | Self::Direction
            | Self::Axis
            | Self::Plane
            | Self::Frame
            | Self::Body
            | Self::Bodies
            | Self::Profile
            | Self::Space => None,
        }
    }

    /// **A pose kind's symmetry** (D10, A11 (1)): the family of the
    /// subgroup of rigid motions a value of this kind is a frame known
    /// up to, the one the mates fold. `None` for a kind that is not a
    /// pose. A pose value states the subgroup itself
    /// ([`crate::PoseValue`]'s [`crate::PoseSymmetry`]), and the pose
    /// evaluator asserts the two agree for every value it binds.
    #[must_use]
    pub fn symmetry(self) -> Option<crate::mate::SubgroupFamily> {
        use crate::mate::SubgroupFamily;
        match self {
            Self::Frame => Some(SubgroupFamily::Trivial),
            Self::Plane => Some(SubgroupFamily::Planar),
            Self::Axis => Some(SubgroupFamily::Cylindrical),
            Self::Direction => Some(SubgroupFamily::Parallel),
            Self::Point => Some(SubgroupFamily::Spherical),
            Self::Length
            | Self::Angle
            | Self::Scalar
            | Self::Count
            | Self::Body
            | Self::Bodies
            | Self::Profile
            | Self::Face
            | Self::Edge
            | Self::Vertex
            | Self::Faces
            | Self::Edges
            | Self::Space => None,
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
                Self::Face => "face",
                Self::Edge => "edge",
                Self::Vertex => "vertex",
                Self::Faces => "set of faces",
                Self::Edges => "set of edges",
                Self::Space => "space",
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
    /// **A selection** (D10): entities of the body `body` reads, named
    /// by `StableName`. The kind is the variable's (a `Face`, an
    /// `Edge`, or a set of either); a singleton holds one name.
    Select(Select),
    /// **A pose definition** (D10): a pose read off geometry, written
    /// in a frame, or constructed from other poses
    /// ([`crate::pose::PoseDef`]). The kind is the definition's, or for
    /// a flip its read's.
    Pose(crate::pose::PoseDef),
    /// **A pick** (D10 `[placement.placement]`): one piece of a copy,
    /// a `Body`.
    Pick(Pick),
    /// **An index** (D10 `[repetition]`): a `Count` over `0..count`. A
    /// definition or operation whose reads reach it is evaluated once
    /// per value, and its outputs are families.
    Index(Index),
}

/// **What an index ranges over** ([`VarDef::Index`]): `0..count`, the
/// count any `Count` expression, and the index it is declared within,
/// if any. One that reads another index in its count is within it
/// whether or not `within` says so ([`crate::Doc::index_within`]).
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Index {
    /// How many values it takes.
    pub count: Expr,
    /// The index it is declared within.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub within: Option<VarId>,
}

impl Index {
    /// Bit-semantic equality.
    #[must_use]
    pub fn bit_eq(&self, other: &Self) -> bool {
        self.within == other.within && self.count.bit_eq(&other.count)
    }
}

/// **What a pick names** ([`VarDef::Pick`]): one piece of a placement's
/// copy, by the piece's variable in the source, through the
/// placements inside the source down to the first variable it reaches.
///
/// `placement` is the copy read (a placement's or an instance's one
/// output). `of` walks the copy's source: each step is one operand of
/// the `⊔` the walk reaches through the placements before it, so a
/// pick of the bolt in a copy of `bolt ⊔ plate` is `of: [bolt]`, and
/// in a copy of a copy of `(bolt ⊔ plate) ⊔ frame` is `of: [s, bolt]`,
/// `s` the inner `⊔`'s output. The variables are the source's, which
/// for an instance are its part's. Like a selection, a pick authored
/// at two sites is two variables, and a pick whose piece the source no
/// longer has is unresolved at evaluation, never re-pointed.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Pick {
    /// The copy read.
    pub placement: VarId,
    /// The path to the piece, outermost first; never empty.
    pub of: Vec<VarId>,
}

/// **What a selection names** ([`VarDef::Select`]): one body read, and
/// the names of its entities, frozen at authoring and resolved through
/// that body's name table under the N5 ladder at evaluation, the one
/// place a name in a slot is resolved.
///
/// The names are stored in the order the kind reads them: an `Edges`
/// set sorted and deduplicated, so two selections of the same edges are
/// equal; a `Faces` set in designation order, first occurrence kept
/// (a shell's rim inherits its first designated face, so the order is
/// authored data); a singleton holds exactly one. An `Edges` set holds
/// at least one name, since no reader gives a blend of no edges a
/// meaning; a `Faces` set holds any number, and empty is the sealed
/// hollow.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Select {
    /// The body the names are read in.
    pub body: VarId,
    /// The entities, by name.
    pub names: Vec<crate::names::StableName>,
}

impl Select {
    /// **Why `names` is not a stored selection of `kind`**, or `None`:
    /// a kind that is not a selection, a count of names the kind does
    /// not hold ([`SelectionCount`]), a name of another entity kind
    /// than the selection's, or a set out of its stored order
    /// ([`Select`]).
    ///
    /// What a name denotes is its own `kind`: the name table's seating
    /// doors refuse a row whose entity is of another kind
    /// ([`crate::names::SeatingRefusal::KindMismatch`]), so a selection
    /// this passes resolves only to entities of its kind.
    #[must_use]
    pub fn fault(kind: VarKind, names: &[crate::names::StableName]) -> Option<SelectionFault> {
        let Some((entity, holds)) = kind.selection() else {
            return Some(SelectionFault::NotASelection { kind });
        };
        if !holds.admits(names.len()) {
            return Some(SelectionFault::Count {
                kind,
                count: names.len(),
            });
        }
        if let Some((at, name)) = names.iter().enumerate().find(|(_, n)| n.kind != entity) {
            return Some(SelectionFault::Kind {
                at,
                found: name.kind,
                expected: entity,
            });
        }
        if entity == crate::names::EntityKind::Edge {
            names
                .windows(2)
                .position(|w| w[0] >= w[1])
                .map(|at| SelectionFault::NotCanonical { at })
        } else {
            names.iter().enumerate().find_map(|(again, name)| {
                names[..again]
                    .iter()
                    .position(|n| n == name)
                    .map(|first| SelectionFault::Repeated { first, again })
            })
        }
    }

    /// `names` in the stored order of a `kind` selection
    /// ([`Select`]): sorted and deduplicated for edges, first
    /// occurrence kept for faces.
    #[must_use]
    pub fn canonical(
        kind: VarKind,
        mut names: Vec<crate::names::StableName>,
    ) -> Vec<crate::names::StableName> {
        if kind
            .selection()
            .is_some_and(|(e, _)| e == crate::names::EntityKind::Edge)
        {
            names.sort();
            names.dedup();
        } else {
            let mut kept: Vec<crate::names::StableName> = Vec::with_capacity(names.len());
            for n in names {
                if !kept.contains(&n) {
                    kept.push(n);
                }
            }
            names = kept;
        }
        names
    }
}

/// **How many names a selection of a kind holds**
/// ([`VarKind::selection`], [`Select`]).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SelectionCount {
    /// A singleton: exactly one.
    One,
    /// An `Edges` set: at least one.
    AtLeastOne,
    /// A `Faces` set: any number, none included.
    Any,
}

impl SelectionCount {
    /// Whether a selection of this count holds `count` names.
    #[must_use]
    pub fn admits(self, count: usize) -> bool {
        match self {
            Self::One => count == 1,
            Self::AtLeastOne => count >= 1,
            Self::Any => true,
        }
    }
}

/// The count as prose reads it: "exactly one", "at least one".
impl core::fmt::Display for SelectionCount {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(match self {
            Self::One => "exactly one",
            Self::AtLeastOne => "at least one",
            Self::Any => "any number",
        })
    }
}

/// **Why a list of names is not a stored selection** ([`Select::fault`]).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SelectionFault {
    /// The kind is not a selection kind.
    NotASelection {
        /// The kind.
        kind: VarKind,
    },
    /// The seat reads no selection of this entity kind, at a seat that
    /// admits several and takes the kind from the names (a pose read
    /// off geometry admitting only some kinds; a whole body is read as
    /// the body). At a seat of one selection kind (a blend's edges, a
    /// shell's faces) a name of another kind is that name's own refusal,
    /// [`Self::Kind`], which the edit door speaks as
    /// [`crate::EditError::SelectionNameKind`].
    ///
    /// This arm and [`Self::OtherBody`] are the edit door's, which
    /// knows the seat and the read; [`Select::fault`] answers the
    /// rest, of a stored selection's form.
    Seat {
        /// The entity kind the names name.
        entity: crate::names::EntityKind,
    },
    /// A selection holding a count of names its kind does not hold
    /// ([`VarKind::selection`]).
    Count {
        /// The selection's kind.
        kind: VarKind,
        /// How many names it holds.
        count: usize,
    },
    /// An edge set not strictly increasing at `at`: a swap or a repeat
    /// (sorted and deduplicated is one rule).
    NotCanonical {
        /// The first index of the out-of-order pair.
        at: usize,
    },
    /// A name denoting another entity kind than the selection holds (a
    /// face in an edge set).
    Kind {
        /// The name's place.
        at: usize,
        /// What it denotes.
        found: crate::names::EntityKind,
        /// What the selection holds.
        expected: crate::names::EntityKind,
    },
    /// A face set naming one face twice.
    Repeated {
        /// The first occurrence.
        first: usize,
        /// The repeat.
        again: usize,
    },
    /// A whole body named at a measure seat whose read is not that
    /// body: the name's node is neither the read's operation nor
    /// upstream of it.
    OtherBody,
}

impl SelectionFault {
    /// **What a count fault's kind holds**: its entity kind and how many
    /// names, `None` for a fault that is not about the count.
    #[must_use]
    pub fn count_bound(&self) -> Option<(crate::names::EntityKind, SelectionCount)> {
        match self {
            Self::Count { kind, .. } => kind.selection(),
            Self::NotASelection { .. }
            | Self::Seat { .. }
            | Self::NotCanonical { .. }
            | Self::Kind { .. }
            | Self::Repeated { .. }
            | Self::OtherBody => None,
        }
    }
}

impl core::fmt::Display for SelectionFault {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::NotASelection { kind } => write!(f, "a {kind} is not a selection"),
            Self::Seat { entity } => {
                write!(
                    f,
                    "the seat reads no selection of {} {}",
                    entity.article(),
                    entity.noun()
                )
            }
            Self::Count { kind, count } => match kind.selection() {
                // A singleton's kind is its entity's noun ("an edge"); a
                // set's is "set of …".
                Some((entity, holds @ SelectionCount::One)) => write!(
                    f,
                    "{} {kind} holds {holds} name, not {count}",
                    entity.article()
                ),
                Some((_, holds)) => write!(f, "a {kind} holds {holds} name, not {count}"),
                None => write!(f, "a {kind} is not a selection"),
            },
            Self::OtherBody => f.write_str("the body named is not the body read"),
            Self::NotCanonical { at } => write!(
                f,
                "the edges are not sorted and deduplicated (names {at} and {} are out of order)",
                at + 1
            ),
            Self::Kind {
                at,
                found,
                expected,
            } => write!(
                f,
                "name {at} is {} {}, not {} {}",
                found.article(),
                found.noun(),
                expected.article(),
                expected.noun()
            ),
            Self::Repeated { first, again } => write!(f, "name {again} repeats name {first}"),
        }
    }
}

impl VarDef {
    /// The kind this definition holds, `None` for an output, whose kind
    /// is its port's.
    #[must_use]
    pub fn kind(&self) -> Option<VarKind> {
        match self {
            Self::Free(free) => Some(VarKind::from(free.dim())),
            Self::Defined(expr) => Some(VarKind::from(expr.dim())),
            Self::Pose(def) => def.kind(),
            Self::Output { .. } | Self::Select(_) => None,
            Self::Pick(_) => Some(VarKind::Body),
            Self::Index(_) => Some(VarKind::Count),
        }
    }

    /// The free variable, when the definition is one.
    #[must_use]
    pub fn free(&self) -> Option<&FreeVar> {
        match self {
            Self::Free(free) => Some(free),
            _ => None,
        }
    }

    /// The defining expression, when the definition is one.
    #[must_use]
    pub fn defined(&self) -> Option<&Expr> {
        match self {
            Self::Defined(expr) => Some(expr),
            _ => None,
        }
    }

    /// The selection, when the definition is one.
    #[must_use]
    pub fn select(&self) -> Option<&Select> {
        match self {
            Self::Select(select) => Some(select),
            _ => None,
        }
    }

    /// The pose definition, when the definition is one.
    #[must_use]
    pub fn pose(&self) -> Option<&crate::pose::PoseDef> {
        match self {
            Self::Pose(def) => Some(def),
            _ => None,
        }
    }

    /// The index, when the definition is one.
    #[must_use]
    pub fn index(&self) -> Option<&Index> {
        match self {
            Self::Index(index) => Some(index),
            _ => None,
        }
    }

    /// The pick, when the definition is one.
    #[must_use]
    pub fn pick(&self) -> Option<&Pick> {
        match self {
            Self::Pick(pick) => Some(pick),
            _ => None,
        }
    }

    /// The operation and port, when the definition is an output.
    #[must_use]
    pub fn output(&self) -> Option<(crate::RecipeNodeId, u8)> {
        match *self {
            Self::Output { node, port } => Some((node, port)),
            Self::Free(_)
            | Self::Defined(_)
            | Self::Select(_)
            | Self::Pose(_)
            | Self::Pick(_)
            | Self::Index(_) => None,
        }
    }

    /// Bit-semantic equality: every float by its bits.
    #[must_use]
    pub fn bit_eq(&self, other: &VarDef) -> bool {
        match (self, other) {
            (Self::Free(a), Self::Free(b)) => a.bit_eq(b),
            (Self::Defined(a), Self::Defined(b)) => a.bit_eq(b),
            (Self::Index(a), Self::Index(b)) => a.bit_eq(b),
            (Self::Output { .. }, Self::Output { .. })
            | (Self::Select(_), Self::Select(_))
            | (Self::Pose(_), Self::Pose(_))
            | (Self::Pick(_), Self::Pick(_)) => self == other,
            (
                Self::Free(_)
                | Self::Defined(_)
                | Self::Output { .. }
                | Self::Select(_)
                | Self::Pose(_)
                | Self::Pick(_)
                | Self::Index(_),
                _,
            ) => false,
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
    /// A selection of the stated kind.
    Select(VarKind, Select),
    /// A pose definition of the stated kind (a flip's is its read's).
    Pose(VarKind, crate::pose::PoseDef),
    /// A pick, a `Body`.
    Pick(Pick),
    /// An index, a `Count`.
    Index(Index),
}

impl WrittenDef {
    /// The kind this definition holds.
    #[must_use]
    pub fn kind(&self) -> VarKind {
        match self {
            Self::Free(free) => VarKind::from(free.dim()),
            Self::Defined(expr) => VarKind::from(expr.dim()),
            Self::Select(kind, _) | Self::Pose(kind, _) => *kind,
            Self::Pick(_) => VarKind::Body,
            Self::Index(_) => VarKind::Count,
        }
    }
}

impl From<WrittenDef> for VarDef {
    fn from(def: WrittenDef) -> Self {
        match def {
            WrittenDef::Free(free) => Self::Free(free),
            WrittenDef::Defined(expr) => Self::Defined(expr),
            WrittenDef::Select(_, select) => Self::Select(select),
            WrittenDef::Pose(_, def) => Self::Pose(def),
            WrittenDef::Pick(pick) => Self::Pick(pick),
            WrittenDef::Index(index) => Self::Index(index),
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
    /// **An index** over `0..count` ([`VarDef::Index`]), `within` the
    /// index it is declared within.
    Index {
        /// How many values it takes, a `Count`.
        count: crate::Formula,
        /// The index it is declared within, read as a lone variable.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        within: Option<crate::Formula>,
    },
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
            Self::Index { .. } => Dimension::Count,
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
    /// lowers to itself. `None` for an output, which no edit declares,
    /// and a selection, which a seat authors.
    #[must_use]
    pub fn authored(def: VarDef) -> Option<Self> {
        match def {
            VarDef::Free(free) => Some(Self::Free(free)),
            VarDef::Defined(expr) => Some(Self::Defined(crate::Formula::from(expr))),
            VarDef::Index(Index { count, within }) => Some(Self::Index {
                count: crate::Formula::from(count),
                within: within.map(|within| crate::Formula::var(within, Dimension::Count)),
            }),
            VarDef::Output { .. } | VarDef::Select(_) | VarDef::Pose(_) | VarDef::Pick(_) => None,
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

    /// **The world's space** ([`VarId::WORLD`]): the one output of the
    /// world, which every document holds from its creation.
    #[must_use]
    pub fn world() -> Self {
        Self::output(VarKind::Space, crate::RecipeNodeId::WORLD, 0)
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

    /// The selection, writable, when the definition is one: its kind
    /// stays.
    pub(crate) fn select_mut(&mut self) -> Option<&mut Select> {
        match &mut self.def {
            VarDef::Select(select) => Some(select),
            VarDef::Free(_)
            | VarDef::Defined(_)
            | VarDef::Output { .. }
            | VarDef::Pose(_)
            | VarDef::Pick(_)
            | VarDef::Index(_) => None,
        }
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
