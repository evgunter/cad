//! The v1 recipe-node vocabulary AS DATA (ratified F4; spec D3). No
//! node here evaluates anything — PR 2's evaluation service interprets
//! this data against the kernel ops.

use crate::expr::{Dimension, Expr};

/// A stable recipe-node identity (spec D3, NAMING-DESIGN N1's
/// substrate): minted from `Doc`'s monotone counter at insertion,
/// never reused (deletion does not free it), never positional. Its
/// stability is a contract, pinned by test.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RecipeNodeId(pub u64);

pub use crate::names::{EntityKind, RoleSeg, StableName};

/// A coordinate axis, naming vector components in slot identities
/// (spec D5: slots are NAMED, never positional indices).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub enum Axis3 {
    /// The x component.
    X,
    /// The y component.
    Y,
    /// The z component.
    Z,
}

/// The regularized boolean operations (F4; kernel semantics in M3's
/// boolean pipeline, interpreted by PR 2).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub enum BooleanOp {
    /// Regularized union.
    Union,
    /// Regularized intersection.
    Intersect,
    /// Regularized difference (a minus b).
    Subtract,
}

/// The NAMED expression-slot identities (spec D5: a per-node-type
/// named enum, never an index). Each variant carries its required
/// dimension ([`SlotId::dimension`]) and structural flag
/// ([`SlotId::is_structural`]).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub enum SlotId {
    /// A datum's origin / a datum point's position component (Length).
    Origin(Axis3),
    /// A datum plane's normal component (Scalar).
    Normal(Axis3),
    /// A datum axis's / linear pattern's direction component (Scalar).
    Direction(Axis3),
    /// An extrude's distance (Length).
    Distance,
    /// A revolve's sweep angle (Angle).
    RevolveAngle,
    /// A transform's translation component (Length).
    Translation(Axis3),
    /// A transform's rotation-axis component (Scalar).
    RotationAxis(Axis3),
    /// A transform's rotation angle (Angle).
    RotationAngle,
    /// A linear pattern's instance spacing (Length).
    Spacing,
    /// A circular pattern's angular step (Angle).
    Step,
    /// A pattern's instance count — the STRUCTURAL slot (spec D3/A8:
    /// Count-typed, edited only via `SetStructuralParam`).
    Count,
}

impl SlotId {
    /// The dimension an expression in this slot must have (checked by
    /// `apply` on insert and on every expression edit, spec D6).
    pub fn dimension(self) -> Dimension {
        match self {
            Self::Origin(_) | Self::Distance | Self::Translation(_) | Self::Spacing => {
                Dimension::Length
            }
            Self::Normal(_) | Self::Direction(_) | Self::RotationAxis(_) => Dimension::Scalar,
            Self::RevolveAngle | Self::RotationAngle | Self::Step => Dimension::Angle,
            Self::Count => Dimension::Count,
        }
    }

    /// Whether this slot is a STRUCTURAL parameter (spec D3: the
    /// structural/continuous distinction is typed, not emergent —
    /// structural slots are exactly the Count-dimensioned ones).
    pub fn is_structural(self) -> bool {
        self.dimension() == Dimension::Count
    }
}

/// A datum construction (F4: plane/axis/point), defined by expression
/// slots — geometry is produced by PR 2's evaluation, never here.
#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub enum Datum {
    /// A plane through `origin` with normal `normal` (unnormalized;
    /// PR 2 normalizes or refuses degenerate loudly).
    Plane {
        /// Origin components, Length ([`SlotId::Origin`]).
        origin: [Expr; 3],
        /// Normal components, Scalar ([`SlotId::Normal`]).
        normal: [Expr; 3],
    },
    /// An axis through `origin` along `direction`.
    Axis {
        /// Origin components, Length ([`SlotId::Origin`]).
        origin: [Expr; 3],
        /// Direction components, Scalar ([`SlotId::Direction`]).
        direction: [Expr; 3],
    },
    /// A point at `position`.
    Point {
        /// Position components, Length ([`SlotId::Origin`]).
        position: [Expr; 3],
    },
}

/// A pattern's replication rule (F4: LinearPattern/CircularPattern;
/// the count lives on [`Node::Pattern`] as the structural slot).
#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub enum PatternKind {
    /// Instances stepped along a direction.
    Linear {
        /// Step direction components, Scalar ([`SlotId::Direction`]).
        direction: [Expr; 3],
        /// Distance between instances, Length ([`SlotId::Spacing`]).
        spacing: Expr,
    },
    /// Instances stepped around a datum axis.
    Circular {
        /// The datum-axis node revolved about (an upstream ref).
        axis: RecipeNodeId,
        /// Angular step between instances ([`SlotId::Step`]).
        step: Expr,
    },
}

/// The v1 feature-node payload (ratified F4; spec D3) — pure data.
///
/// `P` is the OPAQUE profile description: the existing profile crate's
/// type is carried as a value without this crate depending on it
/// (spec D1's geom-core-only boundary + D3's "wrap, don't re-model",
/// reconciled by genericity; PR 2 instantiates `P`).
#[derive(Debug, Clone, PartialEq)]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub enum Node<P> {
    /// A datum construction.
    Datum(Datum),
    /// A programmatic sketch, carried opaquely (F4; never re-modeled).
    Profile(P),
    /// Extrude an upstream profile by a Length distance along its
    /// sketch-plane normal.
    Extrude {
        /// The profile node extruded.
        profile: RecipeNodeId,
        /// Extrusion distance ([`SlotId::Distance`]).
        distance: Expr,
    },
    /// Revolve an upstream profile about a datum axis.
    Revolve {
        /// The profile node revolved.
        profile: RecipeNodeId,
        /// The datum-axis node revolved about.
        axis: RecipeNodeId,
        /// Sweep angle ([`SlotId::RevolveAngle`]).
        angle: Expr,
    },
    /// Split a target body by a tool.
    Split {
        /// The body split.
        target: RecipeNodeId,
        /// The splitting tool.
        tool: RecipeNodeId,
    },
    /// A regularized boolean of two upstream bodies, optionally
    /// consuming a [`Node::Declare`] input (F5: declarations are
    /// recipe data ON the consuming boolean node).
    Boolean {
        /// The operation.
        op: BooleanOp,
        /// Left operand.
        a: RecipeNodeId,
        /// Right operand.
        b: RecipeNodeId,
        /// Optional coincidence-intent input (a `Declare` node).
        declare: Option<RecipeNodeId>,
    },
    /// A rigid placement of an upstream body (F4: Transform).
    Transform {
        /// The body placed.
        input: RecipeNodeId,
        /// Translation components, Length ([`SlotId::Translation`]).
        translation: [Expr; 3],
        /// Rotation-axis components, Scalar ([`SlotId::RotationAxis`]).
        rotation_axis: [Expr; 3],
        /// Rotation angle ([`SlotId::RotationAngle`]).
        rotation_angle: Expr,
    },
    /// A pattern of an upstream body with a STRUCTURAL Count-typed
    /// index expression (spec D3/A8; N1 `Instance(i)` will index it).
    Pattern {
        /// The body replicated.
        input: RecipeNodeId,
        /// Instance count — the structural slot ([`SlotId::Count`]).
        count: Expr,
        /// The replication rule.
        kind: PatternKind,
    },
    /// Coincidence-intent pairs by [`StableName`] (F5; resolution is
    /// PR 3/5 — this crate only carries the data).
    ///
    /// **Name-reference semantics (spec D3 carve-out, ruled at the
    /// PR 1 review)**: the names' `RecipeNodeId`s are REFERENCES, not
    /// DAG edges — [`Node::inputs`] does not include them. `apply`
    /// validates at edit time that every named node EXISTS (a
    /// never-existed id is a typo, refused with a typed error at the
    /// best-diagnostics door), but a later `DeleteNode` MAY strand a
    /// name: that is NAMING-DESIGN N5's ratified dangling-reference
    /// semantics — resolution fails loudly (`NodeGone`) and the
    /// explicit `Rebind` edit (PR 4) is the repair. Blocking the
    /// delete would force cascade-or-pre-repair, worse than the
    /// typed-failure flow.
    Declare {
        /// The declared-coincident name pairs.
        pairs: Vec<(StableName, StableName)>,
    },
}

impl Axis3 {
    /// All three axes, component order (x, y, z).
    pub const ALL: [Axis3; 3] = [Axis3::X, Axis3::Y, Axis3::Z];

    fn index(self) -> usize {
        match self {
            Axis3::X => 0,
            Axis3::Y => 1,
            Axis3::Z => 2,
        }
    }
}

fn comp(v: &[Expr; 3], axis: Axis3) -> &Expr {
    &v[axis.index()]
}

fn comp_mut(v: &mut [Expr; 3], axis: Axis3) -> &mut Expr {
    &mut v[axis.index()]
}

impl<P> Node<P> {
    /// The upstream node references — the recipe DAG's edges (spec
    /// D3). Deterministic order (field order).
    pub fn inputs(&self) -> Vec<RecipeNodeId> {
        match self {
            Node::Datum(_) | Node::Profile(_) | Node::Declare { .. } => Vec::new(),
            Node::Extrude { profile, .. } => vec![*profile],
            Node::Revolve { profile, axis, .. } => vec![*profile, *axis],
            Node::Split { target, tool } => vec![*target, *tool],
            Node::Boolean { a, b, declare, .. } => {
                let mut v = vec![*a, *b];
                v.extend(declare.iter().copied());
                v
            }
            Node::Transform { input, .. } => vec![*input],
            Node::Pattern { input, kind, .. } => {
                let mut v = vec![*input];
                if let PatternKind::Circular { axis, .. } = kind {
                    v.push(*axis);
                }
                v
            }
        }
    }

    /// The expression slots this node actually carries, deterministic
    /// order — the domain of [`Node::expr`].
    pub fn slots(&self) -> Vec<SlotId> {
        let vec3 = |f: fn(Axis3) -> SlotId| Axis3::ALL.map(f);
        match self {
            Node::Datum(Datum::Plane { .. }) => {
                let mut s = vec3(SlotId::Origin).to_vec();
                s.extend(vec3(SlotId::Normal));
                s
            }
            Node::Datum(Datum::Axis { .. }) => {
                let mut s = vec3(SlotId::Origin).to_vec();
                s.extend(vec3(SlotId::Direction));
                s
            }
            Node::Datum(Datum::Point { .. }) => vec3(SlotId::Origin).to_vec(),
            Node::Profile(_) | Node::Split { .. } | Node::Boolean { .. } | Node::Declare { .. } => {
                Vec::new()
            }
            Node::Extrude { .. } => vec![SlotId::Distance],
            Node::Revolve { .. } => vec![SlotId::RevolveAngle],
            Node::Transform { .. } => {
                let mut s = vec3(SlotId::Translation).to_vec();
                s.extend(vec3(SlotId::RotationAxis));
                s.push(SlotId::RotationAngle);
                s
            }
            Node::Pattern { kind, .. } => {
                let mut s = vec![SlotId::Count];
                match kind {
                    PatternKind::Linear { .. } => {
                        s.extend(vec3(SlotId::Direction));
                        s.push(SlotId::Spacing);
                    }
                    PatternKind::Circular { .. } => s.push(SlotId::Step),
                }
                s
            }
        }
    }

    /// The expression in a named slot, `None` if this node type does
    /// not carry that slot (named access only, spec D5).
    pub fn expr(&self, slot: SlotId) -> Option<&Expr> {
        use SlotId as S;
        match (self, slot) {
            (Node::Datum(Datum::Plane { origin, .. }), S::Origin(ax))
            | (Node::Datum(Datum::Axis { origin, .. }), S::Origin(ax))
            | (Node::Datum(Datum::Point { position: origin }), S::Origin(ax)) => {
                Some(comp(origin, ax))
            }
            (Node::Datum(Datum::Plane { normal, .. }), S::Normal(ax)) => Some(comp(normal, ax)),
            (Node::Datum(Datum::Axis { direction, .. }), S::Direction(ax)) => {
                Some(comp(direction, ax))
            }
            (Node::Extrude { distance, .. }, S::Distance) => Some(distance),
            (Node::Revolve { angle, .. }, S::RevolveAngle) => Some(angle),
            (Node::Transform { translation, .. }, S::Translation(ax)) => {
                Some(comp(translation, ax))
            }
            (Node::Transform { rotation_axis, .. }, S::RotationAxis(ax)) => {
                Some(comp(rotation_axis, ax))
            }
            (Node::Transform { rotation_angle, .. }, S::RotationAngle) => Some(rotation_angle),
            (Node::Pattern { count, .. }, S::Count) => Some(count),
            (
                Node::Pattern {
                    kind: PatternKind::Linear { direction, .. },
                    ..
                },
                S::Direction(ax),
            ) => Some(comp(direction, ax)),
            (
                Node::Pattern {
                    kind: PatternKind::Linear { spacing, .. },
                    ..
                },
                S::Spacing,
            ) => Some(spacing),
            (
                Node::Pattern {
                    kind: PatternKind::Circular { step, .. },
                    ..
                },
                S::Step,
            ) => Some(step),
            _ => None,
        }
    }

    /// Mutable access to a named slot's expression (the edit layer's
    /// substrate; all validation lives in `apply`, spec D6).
    pub fn expr_mut(&mut self, slot: SlotId) -> Option<&mut Expr> {
        use SlotId as S;
        match (self, slot) {
            (Node::Datum(Datum::Plane { origin, .. }), S::Origin(ax))
            | (Node::Datum(Datum::Axis { origin, .. }), S::Origin(ax))
            | (Node::Datum(Datum::Point { position: origin }), S::Origin(ax)) => {
                Some(comp_mut(origin, ax))
            }
            (Node::Datum(Datum::Plane { normal, .. }), S::Normal(ax)) => Some(comp_mut(normal, ax)),
            (Node::Datum(Datum::Axis { direction, .. }), S::Direction(ax)) => {
                Some(comp_mut(direction, ax))
            }
            (Node::Extrude { distance, .. }, S::Distance) => Some(distance),
            (Node::Revolve { angle, .. }, S::RevolveAngle) => Some(angle),
            (Node::Transform { translation, .. }, S::Translation(ax)) => {
                Some(comp_mut(translation, ax))
            }
            (Node::Transform { rotation_axis, .. }, S::RotationAxis(ax)) => {
                Some(comp_mut(rotation_axis, ax))
            }
            (Node::Transform { rotation_angle, .. }, S::RotationAngle) => Some(rotation_angle),
            (Node::Pattern { count, .. }, S::Count) => Some(count),
            (
                Node::Pattern {
                    kind: PatternKind::Linear { direction, .. },
                    ..
                },
                S::Direction(ax),
            ) => Some(comp_mut(direction, ax)),
            (
                Node::Pattern {
                    kind: PatternKind::Linear { spacing, .. },
                    ..
                },
                S::Spacing,
            ) => Some(spacing),
            (
                Node::Pattern {
                    kind: PatternKind::Circular { step, .. },
                    ..
                },
                S::Step,
            ) => Some(step),
            _ => None,
        }
    }

    /// The node ids REFERENCED BY NAME from this payload (Declare
    /// pairs) — validated for existence at edit time per the spec D3
    /// carve-out, but NOT DAG edges ([`Node::inputs`] excludes them;
    /// a later delete may strand them, N5 semantics).
    pub fn named_nodes(&self) -> Vec<RecipeNodeId> {
        match self {
            Node::Declare { pairs } => pairs.iter().flat_map(|(a, b)| [a.node, b.node]).collect(),
            _ => Vec::new(),
        }
    }
}

impl<P: PartialEq> Node<P> {
    /// Bit-semantic payload equality (spec D7's comparison substrate):
    /// `PartialEq` for structure plus BIT comparison of every slot
    /// expression's float literals — `0.0` vs `-0.0` differ here. The
    /// opaque profile payload `P` is compared by its own `PartialEq`
    /// (its float semantics are PR 2's contract when `P` is
    /// instantiated).
    pub fn bit_eq(&self, other: &Node<P>) -> bool {
        if self != other {
            return false;
        }
        // Equal payloads have identical slot sets; compare each
        // slot's literal bits (slots() order is deterministic).
        self.slots()
            .into_iter()
            .all(|slot| match (self.expr(slot), other.expr(slot)) {
                (Some(a), Some(b)) => a.bit_eq(b),
                (None, None) => true,
                _ => false,
            })
    }
}
