//! The recipe-node vocabulary AS DATA (ratified F4; spec D3). No node
//! here evaluates anything — the evaluation service interprets this
//! data against the kernel ops.

use crate::expr::{Dimension, Expr};
// The contact vocabulary is the KERNEL's (CONTACT-DESIGN C4, M9-1
// PR-1). Imported, never redefined: the boolean's own refusals must
// carry the same words this node authors, and `crate::names::flush`
// owns the single upward re-export.
use topo::ContactClass;

/// The [`Node`] variants whose payload REFERENCES no [`StableName`], as
/// a PATTERN.
///
/// [`Node::payload_names`] and [`Node::rebind_payload_names`] must name
/// the same variants — the read and the rewrite are one answer read two
/// ways, and a variant one of them treats as nameless while the other
/// rewrites it is a name that survives a `Rebind`.
/// Both matches stay exhaustive: a new [`Node`] variant absent
/// from this list breaks both builds, and adding it to this list is one
/// decision at one site.
macro_rules! name_free_node {
    () => {
        $crate::node::Node::Datum(_)
            | $crate::node::Node::Profile(_)
            | $crate::node::Node::Extrude { .. }
            | $crate::node::Node::Revolve { .. }
            | $crate::node::Node::Loft { .. }
            | $crate::node::Node::Sweep { .. }
            | $crate::node::Node::Split { .. }
            | $crate::node::Node::Boolean { .. }
            | $crate::node::Node::Transform { .. }
            | $crate::node::Node::Pattern { .. }
            | $crate::node::Node::PlacedUnion { .. }
            | $crate::node::Node::InstantiatePart { .. }
            | $crate::node::Node::Assertion { .. }
    };
}

/// A stable recipe-node identity (spec D3, NAMING-DESIGN N1's
/// substrate): minted from `Doc`'s monotone counter at insertion,
/// never reused (deletion does not free it), never positional. Its
/// stability is a contract, pinned by test.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
#[serde(deny_unknown_fields)]
pub struct RecipeNodeId(pub u64);

pub use crate::names::{EntityKind, RoleSeg, StableName};

/// A coordinate axis, naming vector components in slot identities
/// (spec D5: slots are NAMED, never positional indices).
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
#[serde(deny_unknown_fields)]
pub enum Axis3 {
    /// The x component.
    X,
    /// The y component.
    Y,
    /// The z component.
    Z,
}

/// The regularized boolean operations, re-exported from the kernel
/// (F4; ONE enum, defined lowest and re-exported upward, never a
/// parallel enum). A recipe node's operation IS the kernel operation
/// the evaluation service will run, so no conversion stands between
/// authoring it and performing it. Its persisted bytes are this
/// crate's, described by `persist::kernel_wire::boolean_op`.
pub use topo::BooleanOp;

/// A profile-program step's ARGUMENT ROLE — the closed per-verb enum
/// that, with a loop and step index, addresses one expression inside a
/// [`crate::ProfileProgram`] (LIB-SWITCH §4c, VQ3). Roles are named by
/// what the argument IS in the verb's own vocabulary, never by
/// position; [`StepArg::dimension`] carries V2's dimension table.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
#[serde(deny_unknown_fields)]
pub enum StepArg {
    /// An authored on-path point's x (`at`, an arc spec's anchor, the
    /// far-end `to`).
    PointX,
    /// That point's y.
    PointY,
    /// A leg target's x (`line_to`, `arc_to`'s endpoint-full modes,
    /// `tangent_arc_to` — the `Point` target form).
    TargetX,
    /// That target's y.
    TargetY,
    /// A `Via` mode's through-point x.
    ViaX,
    /// That through-point's y.
    ViaY,
    /// A carrier centre's x (the `Center` mode, `circle`,
    /// `circle_split`).
    CenterX,
    /// That centre's y.
    CenterY,
    /// A `toward` director's x component (Scalar — ratio only).
    DirX,
    /// That director's y component.
    DirY,
    /// The `angle(θ)` director.
    AngleVal,
    /// The `turn(δ)` rotation.
    TurnVal,
    /// A `line(len)` length.
    Length,
    /// A radius (`fillet`, `circle`, `circle_split`).
    Radius,
    /// An `arc_to` bulge (authored data, Scalar).
    Bulge,
    /// A `circle_split` first-vertex phase (Angle).
    Phase,
    /// **§2c** an arc spec's carrier radius (`Radius`/`Sweep`/`ArcLen`).
    CarrierRadius,
    /// **§2c** a `Sweep` spec's swept central angle.
    SweepVal,
    /// **§2c** an `ArcLen` spec's arc length.
    ArcLenVal,
    /// **§2c** a fused step's ARRIVAL-spec carrier centre x (the spec₂
    /// role twin — a fused step carries two specs, so the arrival's
    /// roles are distinct).
    Center2X,
    /// That centre's y.
    Center2Y,
    /// The arrival spec's through-point x.
    Via2X,
    /// That through-point's y.
    Via2Y,
    /// The arrival spec's target x.
    Target2X,
    /// That target's y.
    Target2Y,
    /// The arrival spec's carrier radius.
    CarrierRadius2,
}

impl StepArg {
    /// A prose label — the one spelling a user-facing rendering uses,
    /// so a step argument never reaches a reader as `Debug`.
    ///
    /// Named by what the argument IS in the verb's vocabulary, as the
    /// variants are: a coordinate reads as its point plus its axis
    /// (`centre x`), so a panel can put a 2-D point's two roles beside
    /// each other and a reader can see that is what they are. The
    /// arrival-spec twins of a fused step say so rather than carrying a
    /// bare `2`.
    pub fn label(self) -> &'static str {
        match self {
            Self::PointX => "point x",
            Self::PointY => "point y",
            Self::TargetX => "target x",
            Self::TargetY => "target y",
            Self::ViaX => "via x",
            Self::ViaY => "via y",
            Self::CenterX => "centre x",
            Self::CenterY => "centre y",
            Self::DirX => "direction x",
            Self::DirY => "direction y",
            Self::AngleVal => "angle",
            Self::TurnVal => "turn",
            Self::Length => "length",
            Self::Radius => "radius",
            Self::Bulge => "bulge",
            Self::Phase => "phase",
            Self::CarrierRadius => "carrier radius",
            Self::SweepVal => "sweep",
            Self::ArcLenVal => "arc length",
            Self::Center2X => "arrival centre x",
            Self::Center2Y => "arrival centre y",
            Self::Via2X => "arrival via x",
            Self::Via2Y => "arrival via y",
            Self::Target2X => "arrival target x",
            Self::Target2Y => "arrival target y",
            Self::CarrierRadius2 => "arrival carrier radius",
        }
    }

    /// The dimension an expression in this role must have (V2's table:
    /// coordinates/lengths/radii Length; angle/turn/phase Angle;
    /// bulge and director components Scalar — ratio only).
    pub fn dimension(self) -> Dimension {
        match self {
            Self::PointX
            | Self::PointY
            | Self::TargetX
            | Self::TargetY
            | Self::ViaX
            | Self::ViaY
            | Self::CenterX
            | Self::CenterY
            | Self::Length
            | Self::Radius
            | Self::CarrierRadius
            | Self::ArcLenVal
            | Self::Center2X
            | Self::Center2Y
            | Self::Via2X
            | Self::Via2Y
            | Self::Target2X
            | Self::Target2Y
            | Self::CarrierRadius2 => Dimension::Length,
            Self::AngleVal | Self::TurnVal | Self::Phase | Self::SweepVal => Dimension::Angle,
            Self::DirX | Self::DirY | Self::Bulge => Dimension::Scalar,
        }
    }
}

/// The NAMED expression-slot identities (spec D5: a per-node-type
/// named enum, never an index). Each variant carries its required
/// dimension ([`SlotId::dimension`]) and structural flag
/// ([`SlotId::is_structural`]).
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
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
    /// A fillet's constant blend radius (Length).
    Radius,
    /// A chamfer's setback along both supports (Length). Named apart
    /// from [`SlotId::Radius`] because it is a different quantity: a
    /// radius is a rolling ball's, a setback is a distance measured
    /// along each support face from the source edge, and a panel that
    /// spelled both "radius" would be lying about one of them.
    ChamferDistance,
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
    /// A loft's / sweep's v-direction interpolation degree (Book
    /// §10.3) — STRUCTURAL: changing it changes the produced
    /// surface's knot vector, so it is Count-typed like every other
    /// structure-selecting slot (spec D3/A8).
    VDegree,
    /// A sweep's station count: how many rigid copies of the profile
    /// the path is instantiated at before skinning (Book §10.4) —
    /// STRUCTURAL, same rule.
    Stations,
    /// One expression inside a profile PROGRAM (LIB-SWITCH §4c): loop
    /// index, step index, argument role. The LOOP coordinate is a VQ3
    /// sharpening of the design's `(step, arg)` sketch — a profile is
    /// plane + several loops, so the address needs it. Step indices are
    /// stable because program STRUCTURE changes only by re-authoring
    /// (the frozen-selection argument, V2); for the carrier loop forms
    /// (`circle`/`circle_split`) `step` is 0.
    Profile {
        /// The loop's index in the program (description order).
        loop_: u32,
        /// The step's index within the loop's chain (0 for carrier
        /// forms).
        step: u32,
        /// Which of the step's arguments.
        arg: StepArg,
    },
}

/// A slot family whose members are the three COMPONENTS of one
/// 3-vector — the vector-valued half of [`SlotId`], named once here so
/// that a consumer wanting to treat `Origin(X)`, `Origin(Y)` and
/// `Origin(Z)` as one quantity does not have to re-derive which
/// variants those are.
///
/// **The reason this lives in the node vocabulary and not in a panel.**
/// "These three slots are one vector" is a fact about the slot
/// vocabulary (D5), on the same footing as [`SlotId::dimension`] and
/// [`SlotId::is_structural`]: every component of a family shares a
/// dimension, and the family is what an editor, a binding, or a
/// recorded macro means when it says "the origin". A consumer that
/// matched on `SlotId` itself would answer the question correctly
/// today and then silently under-cover the next vector slot added; the
/// exhaustive match in [`SlotId::component`] makes that addition a
/// compile error instead.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum VectorSlot {
    /// A datum's origin / a datum point's position ([`SlotId::Origin`]).
    Origin,
    /// A datum plane's normal ([`SlotId::Normal`]).
    Normal,
    /// A datum axis's / linear pattern's direction
    /// ([`SlotId::Direction`]).
    Direction,
    /// A transform's translation ([`SlotId::Translation`]).
    Translation,
    /// A transform's rotation axis ([`SlotId::RotationAxis`]).
    RotationAxis,
}

impl VectorSlot {
    /// Every vector family, in no significant order — for a consumer
    /// enumerating families rather than reading one off a slot.
    pub const ALL: [VectorSlot; 5] = [
        VectorSlot::Origin,
        VectorSlot::Normal,
        VectorSlot::Direction,
        VectorSlot::Translation,
        VectorSlot::RotationAxis,
    ];

    /// This family's slot for one axis — the inverse of
    /// [`SlotId::component`], and total.
    pub fn slot(self, axis: Axis3) -> SlotId {
        match self {
            Self::Origin => SlotId::Origin(axis),
            Self::Normal => SlotId::Normal(axis),
            Self::Direction => SlotId::Direction(axis),
            Self::Translation => SlotId::Translation(axis),
            Self::RotationAxis => SlotId::RotationAxis(axis),
        }
    }

    /// All three of this family's slots, component order (x, y, z).
    pub fn slots(self) -> [SlotId; 3] {
        Axis3::ALL.map(|axis| self.slot(axis))
    }

    /// The family as a prose noun — the one spelling a user-facing
    /// rendering uses.
    pub fn label(self) -> &'static str {
        match self {
            Self::Origin => "origin",
            Self::Normal => "normal",
            Self::Direction => "direction",
            Self::Translation => "translation",
            Self::RotationAxis => "rotation axis",
        }
    }

    /// The dimension every component of this family carries.
    ///
    /// Answered through [`SlotId::dimension`] rather than restated, so
    /// the family cannot come to disagree with its own slots: the three
    /// components share a dimension by construction (each family maps
    /// to one `SlotId` arm, and that arm's dimension does not depend on
    /// the axis).
    pub fn dimension(self) -> Dimension {
        self.slot(Axis3::X).dimension()
    }
}

impl SlotId {
    /// The dimension an expression in this slot must have (checked by
    /// `apply` on insert and on every expression edit, spec D6).
    pub fn dimension(self) -> Dimension {
        match self {
            Self::Origin(_)
            | Self::Distance
            | Self::Radius
            | Self::ChamferDistance
            | Self::Translation(_)
            | Self::Spacing => Dimension::Length,
            Self::Normal(_) | Self::Direction(_) | Self::RotationAxis(_) => Dimension::Scalar,
            Self::RevolveAngle | Self::RotationAngle | Self::Step => Dimension::Angle,
            Self::Count | Self::VDegree | Self::Stations => Dimension::Count,
            // Profile-program roles carry V2's per-role table; none is
            // Count, so `is_structural` stays false for every StepArg
            // (LIB-SWITCH §4c — program structure is the STEP LIST,
            // changed by re-authoring, never through a slot).
            Self::Profile { arg, .. } => arg.dimension(),
        }
    }

    /// Whether this slot is a STRUCTURAL parameter (spec D3: the
    /// structural/continuous distinction is typed, not emergent —
    /// structural slots are exactly the Count-dimensioned ones).
    pub fn is_structural(self) -> bool {
        self.dimension() == Dimension::Count
    }

    /// A prose label — the one spelling a user-facing rendering uses,
    /// so a slot never reaches a reader as `Debug`.
    ///
    /// It exists for the same reason [`Axis3::label`] and
    /// [`VectorSlot::label`] do, and it is the outermost of the three:
    /// a component reads as its family plus its axis, and a profile
    /// slot as its address plus its role. A panel that spelled these
    /// itself would be a second naming of the vocabulary, drifting from
    /// it silently.
    pub fn label(self) -> String {
        if let Some((family, axis)) = self.component() {
            return format!("{} {}", family.label(), axis.label());
        }
        match self {
            Self::Distance => "distance".to_owned(),
            Self::Radius => "radius".to_owned(),
            Self::ChamferDistance => "chamfer distance".to_owned(),
            Self::RevolveAngle => "revolve angle".to_owned(),
            Self::RotationAngle => "rotation angle".to_owned(),
            Self::Spacing => "spacing".to_owned(),
            Self::Step => "angular step".to_owned(),
            Self::Count => "count".to_owned(),
            Self::VDegree => "v degree".to_owned(),
            Self::Stations => "stations".to_owned(),
            Self::Profile { loop_, step, arg } => {
                format!("loop {loop_} step {step} · {}", arg.label())
            }
            // Every component variant answered above.
            Self::Origin(_)
            | Self::Normal(_)
            | Self::Direction(_)
            | Self::Translation(_)
            | Self::RotationAxis(_) => self.component().map_or_else(
                || String::from("component"),
                |(family, axis)| format!("{} {}", family.label(), axis.label()),
            ),
        }
    }

    /// The 3-vector family this slot is a component of, and which
    /// component — `None` for a scalar slot.
    ///
    /// The match is EXHAUSTIVE on purpose (see [`VectorSlot`]): a slot
    /// variant added to this enum has to answer here, so a new vector
    /// family cannot reach a consumer as three unrelated scalars.
    pub fn component(self) -> Option<(VectorSlot, Axis3)> {
        match self {
            Self::Origin(axis) => Some((VectorSlot::Origin, axis)),
            Self::Normal(axis) => Some((VectorSlot::Normal, axis)),
            Self::Direction(axis) => Some((VectorSlot::Direction, axis)),
            Self::Translation(axis) => Some((VectorSlot::Translation, axis)),
            Self::RotationAxis(axis) => Some((VectorSlot::RotationAxis, axis)),
            Self::Distance
            | Self::Radius
            | Self::ChamferDistance
            | Self::RevolveAngle
            | Self::RotationAngle
            | Self::Spacing
            | Self::Step
            | Self::Count
            | Self::VDegree
            | Self::Stations
            | Self::Profile { .. } => None,
        }
    }
}

/// A datum construction (F4: plane/axis/point), defined by expression
/// slots — geometry is produced by PR 2's evaluation, never here.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
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

/// One declaration crossing a split seam (ASM-4 D-2; ASSEMBLY-DESIGN
/// A4: "the seam is the crossing declarations" — each entry is a
/// (wrapped name, declaration) pair against the pinned document).
///
/// **INHABITED as of ASM-R2b D-4** — the hook ASM-4 named is taken up
/// by its one intended inhabitant, the crossing MATE EDGE. The
/// obligation ASM-4 recorded here is discharged with it: the record
/// now feeds the instantiate node's content key, and the format change
/// rode a schema-version bump (see [`crate::SCHEMA_VERSION`]'s ledger).
///
/// An enum with a single variant, not a struct, for the reason ASM-4
/// gave: a crossing is whatever KIND of edge crossed, and mates are
/// the only kind of edge that can cross today. A second kind extends
/// this enum rather than retrofitting a shape onto the first.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub enum InterfaceCrossing {
    /// A mate whose two ends landed on OPPOSITE SIDES of the split cut
    /// (A4: "every mate edge crossing the cut becomes the interface
    /// record in the remainder").
    ///
    /// `outer` is the reference that stayed in the remainder; `inner`
    /// is the reference that moved into the part, spelled in the
    /// PART's own names — unwrapped, because that is what the part's
    /// product answers to and re-verification resolves against. The
    /// wrapped form (`outer_head / InPart{ inner }`) is what the
    /// remainder's mate now reads, and re-wrapping is the split's
    /// rebind, so storing the wrapper twice would be storing a
    /// derivable fact.
    Mate {
        /// The crossing mate, in the remainder.
        mate: RecipeNodeId,
        /// The class the crossing declares.
        #[serde(with = "crate::persist::kernel_wire::contact_class")]
        class: crate::mate::ContactClass,
        /// The remainder-side reference.
        outer: StableName,
        /// The part-side reference, in the part's own names.
        inner: StableName,
    },
}

/// The interface record of an instantiate seam (ASM-4 D-2): the
/// declarations that crossed the cut when the referenced document was
/// split out. Ordinary node data — recorded by the split that minted
/// the instance, carried by every instantiate node (empty when nothing
/// crossed or the instance was authored directly).
///
/// An ABSENT record on the wire is the empty record (the A11
/// placement-registry precedent: a missing entry is the identity, not
/// a hole), so the empty state costs no bytes and moves no pin.
#[derive(Debug, Clone, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct InterfaceRecord {
    /// The crossing declarations, in the deterministic order the split
    /// collected them (the pre-split document's mate order). Empty for
    /// a directly-authored instance, and for a split that no mate
    /// crossed.
    pub crossings: Vec<InterfaceCrossing>,
}

impl InterfaceRecord {
    /// Whether the record carries no crossings — the wire-presence
    /// test (an empty record serializes as nothing at all).
    pub fn is_empty(&self) -> bool {
        self.crossings.is_empty()
    }
}

/// A pattern's replication rule (F4: LinearPattern/CircularPattern;
/// the count lives on [`Node::Pattern`] as the structural slot).
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
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
    /// Instances at ABSOLUTE frames, listed (GROUP-BOOLEAN-DESIGN,
    /// ratified A′): the rule vocabulary's non-parametric member, for
    /// the placements no linear or circular step generates — the die's
    /// twenty-one pip locations, say.
    ///
    /// **The list IS the count.** Order is data and the index is
    /// D8-structural (it is what `RoleSeg::Instance` indexes), so
    /// appending a placement changes no existing index. A node carrying
    /// this rule has NO [`SlotId::Count`] slot: the number of
    /// placements has exactly one spelling, and the
    /// two-sources-of-truth state is refused at the edit door rather
    /// than reconciled there.
    Explicit(Vec<crate::placement::Frame>),
}

/// **The `Expr`s a node carries OUTSIDE its slots**, in deterministic
/// order — `None` for the nodes that carry none, which is every node
/// but the two the measurement vocabulary adds.
///
/// The slot vocabulary is the ordinary home for a node's expressions,
/// and it stays so: this is the escape hatch for the two expressions
/// whose dimension a slot ADDRESS cannot fix — a measured
/// expression's value leaves (they live inside a `MeasureExpr`, not
/// beside it) and an assertion's bound (its dimension is the measure's).
///
/// One order, three consumers: the evaluator resolves these once, the
/// content key hashes the resolved values, and the op reads the same
/// vector. `None` rather than an empty vector for a slot-only node —
/// the key writes nothing at all for those, so no existing document's
/// content key moves.
pub fn payload_exprs<P>(node: &Node<P>) -> Option<Vec<&Expr>> {
    match node {
        Node::Measure { expr, .. } => {
            let mut leaves = Vec::new();
            expr.value_leaves(&mut leaves);
            Some(leaves)
        }
        Node::Assertion { bound, .. } => Some(vec![bound]),
        Node::Datum(_)
        | Node::Profile(_)
        | Node::Extrude { .. }
        | Node::Revolve { .. }
        | Node::Loft { .. }
        | Node::Sweep { .. }
        | Node::Fillet { .. }
        | Node::Chamfer { .. }
        | Node::Split { .. }
        | Node::Boolean { .. }
        | Node::Transform { .. }
        | Node::Pattern { .. }
        | Node::PlacedUnion { .. }
        | Node::Declare { .. }
        | Node::InstantiatePart { .. }
        | Node::Mate { .. } => None,
    }
}

/// **A measured entity reference: a name, and the node to read it at.**
///
/// Both halves are load-bearing and they are not the same node.
/// `name` says WHICH entity (N1: the name embeds the node that minted
/// it); `at` says which evaluated value to read its carrier out of.
/// They coincide for a reference to a body's own minting node and
/// diverge the moment anything places that body — which is the case
/// this type exists for, because a transform is identity-preserving
/// and mints no name of its own.
///
/// `at` is an ordinary DAG edge ([`Node::inputs`]); `name` resolves
/// against `at`'s table through the same N5 ladder every other
/// authored name takes.
#[derive(
    Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
#[serde(deny_unknown_fields)]
pub struct MeasureRef {
    /// The node whose evaluated value the carrier is read at — the
    /// PLACED geometry, when that node placed it.
    pub at: RecipeNodeId,
    /// The entity's stable name, resolved against `at`'s table.
    pub name: StableName,
}

impl MeasureRef {
    /// A reference read at the node that minted the name — the
    /// degenerate case, and the honest spelling of "as authored".
    pub fn at_mint(name: StableName) -> Self {
        Self {
            at: name.node,
            name,
        }
    }

    /// A reference read at `at`.
    pub fn new(at: RecipeNodeId, name: StableName) -> Self {
        Self { at, name }
    }
}

/// What makes a [`Node::Measure`]'s expression unusable
/// ([`Node::measure_fault`]) — one vocabulary for the construction
/// door and the load door's re-check.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MeasureNodeFault {
    /// A primitive addresses a reference the node does not carry. The
    /// expression indexes `refs` positionally, so an index past its
    /// end names nothing at all — a corrupt recipe, refused rather
    /// than resolved to whatever happens to sit at the last position.
    RefIndexOutOfRange {
        /// The primitive that reads it.
        verb: &'static str,
        /// The out-of-range index.
        index: u32,
        /// How many references the node carries.
        refs: usize,
    },
}

// The ONE prose vocabulary for this fault, forwarded by every door
// that renders it rather than restated.
impl core::fmt::Display for MeasureNodeFault {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::RefIndexOutOfRange { verb, index, refs } => write!(
                f,
                "`{verb}` reads reference {index}, and the measure carries {refs} — the \
                 expression indexes the node's reference list, so this names nothing"
            ),
        }
    }
}

impl core::error::Error for MeasureNodeFault {}

/// What makes a placement-rule node's rule unusable
/// ([`Node::placement_rule_fault`]) — one vocabulary for the edit
/// door, the persist re-check and the evaluation backstop.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PlacementRuleFault {
    /// The rule and the count slot would answer "how many placements"
    /// two different ways: an `Explicit` rule paired with a count, a
    /// stepped rule without one, or an `Explicit` rule on
    /// [`Node::Pattern`] (whose count is a non-optional field).
    CountSpelling,
    /// An `Explicit` rule listing NO placements. The list is the
    /// count, so this is the explicit rule's `count < 1`.
    NoPlacements,
    /// A placement frame with a non-finite coordinate.
    NonFiniteFrame {
        /// Its index in the placement list.
        index: usize,
    },
    /// An IMPROPER placement frame — determinant ≤ 0, i.e. a mirror
    /// (A6). Admitting one is gated on the equivariance audit R4 owns,
    /// exactly as for a cluster placement.
    ImproperFrame {
        /// Its index in the placement list.
        index: usize,
        /// The linear part's determinant.
        determinant: f64,
    },
}

// The ONE prose vocabulary for this fault set — every door that
// renders a `PlacementRuleFault` (the evaluation backstop, the edit
// door's rule arms) FORWARDS this rendering rather than restating the
// fault in its own words.
impl core::fmt::Display for PlacementRuleFault {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::CountSpelling => f.write_str(
                "the placement rule and the count slot disagree about how many placements \
                 there are",
            ),
            Self::NoPlacements => f.write_str(
                "the placement list is empty — a group needs at least one placement, exactly \
                 as a stepped rule needs a count of at least 1",
            ),
            Self::NonFiniteFrame { index } => {
                write!(f, "placement {index} has a non-finite coordinate")
            }
            Self::ImproperFrame { index, determinant } => write!(
                f,
                "placement {index} is improper (mirroring): determinant {determinant}"
            ),
        }
    }
}

impl PatternKind {
    /// The listed placements when this rule carries its own, `None` for
    /// the parametric rules (whose count is the structural slot).
    ///
    /// The one door every count question goes through, so "how many
    /// instances" is never answered two ways.
    pub fn placements(&self) -> Option<&[crate::placement::Frame]> {
        match self {
            PatternKind::Explicit(frames) => Some(frames),
            PatternKind::Linear { .. } | PatternKind::Circular { .. } => None,
        }
    }
}

/// The v1 feature-node payload (ratified F4; spec D3) — pure data.
///
/// `P` is the OPAQUE profile description: the existing profile crate's
/// type is carried as a value without this crate depending on it
/// (spec D1's geom-core-only boundary + D3's "wrap, don't re-model",
/// reconciled by genericity; PR 2 instantiates `P`).
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
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
    /// **Loft** — a skinned solid through two or more section
    /// profiles (The NURBS Book §10.3; C11, M5 PR 10). An ORDINARY op
    /// in this vocabulary: named slots per D5, the
    /// structural/continuous divide preserved, input refs resolving to
    /// existing nodes only.
    ///
    /// # Q8: the produced NURBS **is** the definition
    ///
    /// The walls this node evaluates to are not approximations of some
    /// truer surface implied by the sections — the skin the recipe
    /// selects IS the shape. The recipe (these profiles, this degree)
    /// is PROVENANCE: it records how the surface was chosen and lets an
    /// edit re-choose it. There is NO residual obligation to a
    /// reference locus and NO approximating-surface machinery anywhere
    /// downstream. Only DERIVED items — intersections with these walls,
    /// pcurves of non-iso edges — carry certificates, because only they
    /// claim something about a locus other than themselves.
    Loft {
        /// The section profile nodes, in skin order (≥ 2). Order is
        /// data: reversing it reverses the produced surface's
        /// v-direction.
        profiles: Vec<RecipeNodeId>,
        /// The v-direction interpolation degree ([`SlotId::VDegree`]);
        /// must satisfy `1 ≤ degree ≤ profiles.len() − 1`, checked at
        /// evaluation against the resolved value.
        v_degree: Expr,
    },
    /// **Sweep** — a rigid profile carried along a path (The NURBS
    /// Book §10.4; C11, M5 PR 10), scope-boxed exactly as C11 boxes
    /// it: rigid profile, translational or path-following, **no**
    /// variable sections and **no** scaling laws.
    ///
    /// Evaluated by §10.4's *instantiate and skin*: rigid copies of the
    /// profile are placed at `stations` points along the path and
    /// skinned. Under Q8 (see [`Node::Loft`]) that is not an
    /// approximation of a swept locus — it is the definition of one.
    Sweep {
        /// The profile node swept.
        profile: RecipeNodeId,
        /// The path node: a profile whose FIRST loop's chain is the
        /// trajectory (an open or closed polyline/arc chain).
        path: RecipeNodeId,
        /// How many stations the path is instantiated at
        /// ([`SlotId::Stations`]); must be ≥ 2.
        stations: Expr,
        /// The v-direction interpolation degree ([`SlotId::VDegree`]);
        /// must satisfy `1 ≤ degree ≤ stations − 1`.
        v_degree: Expr,
    },
    /// Constant-radius rolling-ball fillets on a SELECTION of
    /// `target`'s edges (M5 PR 12; the selection is M6-5).
    ///
    /// The op is [`sweep::blend::build::fillet_edges`] over the
    /// resolved selection; anything outside its two assembly front
    /// doors is a typed refusal
    /// ([`crate::eval::NodeErrorKind::Blend`]), never a silent
    /// pass-through of the input body.
    ///
    /// # The selection FREEZES (ruled, #217)
    ///
    /// `selection` is a set of stable names and nothing else — there
    /// is no "every edge" variant. A click-selection is a
    /// **commitment**: an upstream edit that adds edges does NOT
    /// extend it, and an upstream edit that removes a selected edge is
    /// a typed refusal, not a silent shrink. To select everything as
    /// of *now*, call [`all_edges`](crate::all_edges) and store what
    /// it returns; the result is a frozen set with the same
    /// semantics, not a live query.
    ///
    /// ## What moves a stored selection, exactly
    ///
    /// [`crate::DocEdit::Rebind`] rewrites it — that is the REPAIR
    /// path, and the honest description of its reach: a rebind is a
    /// 1:1 `from → to` rewrite followed by re-canonicalization, so it
    /// can SWAP a name or (when `to` is already selected) SHRINK the
    /// set by one. It cannot make the set larger. Adding an edge to a
    /// selection means re-authoring the node with a new set — a
    /// deliberate act, which is the point of freezing.
    ///
    /// Note also that nothing today can quietly widen a selection
    /// even if it wanted to: the kernel's assembly admits only a
    /// fully-requested chain set (`sweep::fillet`'s front door), so a
    /// partially-grown selection refuses typed rather than blending
    /// something the author never picked. Freeze is enforced
    /// structurally, and its breaks are loud.
    ///
    /// # Canonical form
    ///
    /// The set is stored sorted and deduplicated, so two recipes that
    /// select the same edges are bit-identical (the content key reads
    /// the vector in order). [`Node::fillet`] canonicalizes; a loaded
    /// snapshot is ASSERTED canonical rather than repaired
    /// ([`crate::persist`]'s strict door — a non-canonical file is a
    /// corrupt file).
    Fillet {
        /// The body whose edges are blended.
        target: RecipeNodeId,
        /// The constant blend radius ([`SlotId::Radius`]).
        radius: Expr,
        /// The edges to blend, by stable name — canonical (sorted,
        /// deduplicated), frozen at authoring time.
        selection: Vec<StableName>,
    },
    /// Equal-setback flat chamfers on a SELECTION of `target`'s edges
    /// — [`Node::Fillet`]'s twin.
    ///
    /// The op is [`sweep::blend::build::chamfer_edges`], which is
    /// `fillet_edges` modulo the size's meaning: `distance` is the
    /// SETBACK measured along each support from the source edge, not a
    /// rolling ball's radius. Everything else this node says is the
    /// fillet's, and deliberately so — the same two assembly front
    /// doors, the same typed refusal on anything outside them
    /// ([`crate::eval::NodeErrorKind::Blend`] carrying
    /// [`sweep::blend::BlendKind::Chamfer`]), never a silent
    /// pass-through of the input body.
    ///
    /// # The selection FREEZES, and the canonical form
    ///
    /// Both exactly as [`Node::Fillet`] states them: a set of stable
    /// names and nothing else, no "every edge" variant,
    /// [`crate::DocEdit::Rebind`] the one repair, stored sorted and
    /// deduplicated by [`Node::chamfer`], and a non-canonical set on
    /// the wire is a corrupt file. The freeze argument does not depend
    /// on which blend the surgery performs, so it is not restated
    /// here — read it there.
    ///
    /// # Why this is a separate variant and not a flag on `Fillet`
    ///
    /// The two carry different quantities in their size slot
    /// ([`SlotId::Radius`] vs [`SlotId::ChamferDistance`]), and a
    /// stored recipe that changed which one a number meant on a
    /// boolean's value would be a document whose geometry depends on a
    /// field a reader can miss. Separate variants make the size's
    /// meaning readable off the node kind, and make the naming
    /// discrimination structural: the minting node is what tells a
    /// chamfer's blend from a fillet's at every selector
    /// (RECIPE-DOORS D3), so the two must be different nodes.
    Chamfer {
        /// The body whose edges are chamfered.
        target: RecipeNodeId,
        /// The setback along both supports
        /// ([`SlotId::ChamferDistance`]).
        distance: Expr,
        /// The edges to chamfer, by stable name — canonical (sorted,
        /// deduplicated), frozen at authoring time.
        selection: Vec<StableName>,
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
        #[serde(with = "crate::persist::kernel_wire::boolean_op")]
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
    /// **The group boolean** (GROUP-BOOLEAN-DESIGN, ratified A′): ONE
    /// prototype, a placement rule, ONE BODY OUT — the union of the
    /// prototype placed at each placement.
    ///
    /// A Pattern that fuses, and deliberately NOT a [`PatternKind`] of
    /// [`Node::Pattern`]: Pattern's N-bodies-unfused output contract
    /// stays untouched, because forking a node's RESULT TYPE on a
    /// variant is the silent-dispatch trap D3 forbids. What the two
    /// share is the rule vocabulary and the naming: per-instance
    /// discrimination is the ratified `RoleSeg::Instance { i, of }`
    /// (A8/N1) verbatim, so the vocabulary does not grow and
    /// "instance 7's cavity face" is one selector row.
    ///
    /// Disjointness is CERTIFIED, never declared: one
    /// [`topo::Separation`] over the prototype, queried per placement
    /// pair, and the union lowers through the existing
    /// `graft_disjoint_all_keyed` door — no new kernel op, no new
    /// kernel naming record. The certificate is sufficient-not-
    /// necessary, so a BVH-touching-but-genuinely-disjoint arrangement
    /// refuses honestly rather than passing on a guess.
    PlacedUnion {
        /// The prototype placed at every placement.
        input: RecipeNodeId,
        /// Placement count — the structural slot ([`SlotId::Count`]) —
        /// present exactly for the PARAMETRIC rules. `Explicit` carries
        /// its own placements and derives the count from them, so the
        /// slot is ABSENT there rather than inert: one number, one
        /// spelling (the edit door refuses the mismatched states).
        count: Option<Expr>,
        /// The placement rule.
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
        /// The declared contact pairs, each with the CLASS it asserts
        /// (CONTACT-DESIGN C4).
        ///
        /// The class rides every pair rather than a node-level
        /// default: a declaration is "these two faces are in contact,
        /// of THIS kind", and one `Declare` node may carry pairs of
        /// different kinds. A class-less pair is unrepresentable —
        /// there is no constructor that omits it and no default to
        /// fall back to, because defaulting would let a `Tangent`
        /// intent be verified against the conformal table.
        #[serde(with = "crate::persist::kernel_wire::contact_class::pairs")]
        pairs: Vec<((StableName, StableName), ContactClass)>,
    },
    /// An instance of another document's product (ASSEMBLY-DESIGN
    /// A2/A3, ASM-2A D-1): a LEAF — its material crosses the document
    /// seam rather than arriving from an upstream node, so
    /// [`Node::inputs`] is empty and the DAG has nothing to schedule
    /// ahead of it.
    ///
    /// **No frame field.** A11 puts placement on the CLUSTER, and the
    /// registry holding it is document data ([`crate::Doc::placement`])
    /// — an instance carries no frame of its own, which is what makes
    /// zero-anchor and multi-anchor states unrepresentable rather than
    /// merely refused.
    InstantiatePart {
        /// Which document, at which version (A4: the id answers "which
        /// part", the pin "which version of it"). Cargo.lock semantics
        /// — an edit to the referenced document never retargets this
        /// reference; moving the pin is its own recorded edit.
        doc_ref: crate::ident::DocRef,
        /// The split seam's interface record (ASM-4 D-2; inhabited by
        /// ASM-R2b D-4): the declarations that crossed the cut this
        /// instance was minted by. Empty for directly-authored
        /// instances, and absent from the wire while empty — so an
        /// instance that no mate crosses still costs no bytes and
        /// moves no pin. A NON-empty record is on-wire data and feeds
        /// the node's content key.
        #[serde(default, skip_serializing_if = "InterfaceRecord::is_empty")]
        interface: InterfaceRecord,
    },
    /// A **mate** between two instances (ASSEMBLY-DESIGN A3/A12;
    /// ASM-R2a D-1): one node carrying BOTH the placement constraint
    /// and the contact declaration, so there is no second vocabulary
    /// to keep synced.
    ///
    /// **A leaf.** `a`/`b` are instance-qualified stable references,
    /// and name references are not DAG edges — the shipped D3
    /// carve-out `Declare` established — so [`Node::inputs`] is empty
    /// and inserting a mate transfers no root. A12 adds *reading*
    /// edges on top: the instantiate node each reference's head
    /// resolves through, RECOMPUTED at need
    /// ([`crate::mate::reading_edges`]) and never stored. A9's
    /// relative-freedom partition and A11's placement clusters read
    /// consuming ∪ reading edges; A10's invariants, maintenance and
    /// product gather read consuming edges only. Under consuming edges
    /// a mate is an isolated sink, so it is an ordinary NON-BODY root:
    /// listed like any other, denoting no body, ignored by the gather.
    /// A dangling head is N5's ratified semantics — no edge until
    /// `Rebind`, and the solve refuses typed naming it.
    Mate {
        /// The `a` reference: an entity of one instance's product.
        a: StableName,
        /// The `b` reference: an entity of the other's.
        b: StableName,
        /// The declared contact class — the KERNEL vocabulary (M9-1),
        /// re-exported rather than re-minted, so a mate's declaration
        /// is already the currency the boolean wrapper's records
        /// speak. How far each class gets is
        /// [`crate::mate::class_admission`], not a set restated here;
        /// separately, a spelling this build has no name for refuses
        /// typed at the wire door.
        #[serde(with = "crate::persist::kernel_wire::contact_class")]
        class: crate::mate::ContactClass,
        /// Which frames coincide, with which axis senses, at which
        /// clocking (A3's alignment datum).
        alignment: crate::mate::Alignment,
    },
    /// **A measurement sink** (ERROR-DESIGN E3): one dimension-generic
    /// node that denotes NO body and evaluates to a typed F1 quantity.
    ///
    /// There is one `Measure` variant, not one per measured kind: the
    /// quantity's dimension rides the EXPRESSION through the existing
    /// lattice, so `distance` and `angle` are values of one node kind
    /// rather than a parallel type vocabulary beside F1.
    ///
    /// # References
    ///
    /// `refs` is the frozen, canonical entity selection — the
    /// [`Node::Fillet`] `selection` precedent — and the expression
    /// addresses it by INDEX. Unlike a fillet's selection the order is
    /// MEANINGFUL (it is argument order: `gap`'s first reference is the
    /// containing carrier), so the vector is neither sorted nor
    /// deduplicated; what canonicalization buys elsewhere — bit-equal
    /// recipes for equal selections — is bought here by the indices
    /// being part of the expression.
    ///
    /// # These name references ARE edges
    ///
    /// `Declare` and `Mate` carry names that are not DAG edges (the
    /// spec D3 carve-out): they pass their names through as data and
    /// something downstream resolves them. A measure resolves its own,
    /// against values that must ALREADY EXIST when it runs — so the
    /// referenced nodes are exactly its data dependencies, and
    /// [`Node::inputs`] reports them. Nothing else can order the sink
    /// after the geometry it measures: the schedule is edge-driven, so
    /// an edgeless measure would be scheduled at level 0 and resolve
    /// against nothing.
    ///
    /// **The consequence, stated because it departs from the
    /// carve-out**: deleting a referenced node is refused at the
    /// delete door (`DeleteWouldDangle`) exactly as it is for any
    /// consumer's input, where a `Declare` would have let the delete
    /// through and stranded the name. N5's dangling semantics still
    /// govern the case they were written for — a name that stops
    /// resolving in a still-live node's table, which the typed
    /// resolution refusal reports and `Rebind` repairs.
    ///
    /// # What a reference denotes: the carrier AT a named node
    ///
    /// A [`MeasureRef`] is a pair — the entity's [`StableName`], and
    /// the node its carrier is READ AT. The second half is what makes
    /// a measure report placed geometry.
    ///
    /// A name alone cannot do it. N1 names embed their MINTING node,
    /// and a rigid transform is identity-preserving: `wire_transform`
    /// hands the input's table through by `Arc::clone` and contributes
    /// no RolePath segment, so a transformed wall keeps the upstream
    /// name and there is no transform-minted name to reference
    /// instead. Resolving at the minting node therefore measured the
    /// UNMOVED carrier — a box translated 100 m measured 5 where the
    /// placed answer is 95, and said `Ok`.
    ///
    /// So the reference names the node to read at, exactly as the
    /// interrogation doors do (`face_frame(ev, node, name)` — this is
    /// their contract, not a new one). Selecting a wall from a
    /// transform's own selection door and measuring it gives the
    /// placed number, because `at` is that transform.
    Measure {
        /// The measured expression: `Expr` arithmetic over
        /// [`crate::MeasurePrimitive`] leaves that index `refs`.
        expr: crate::measure::MeasureExpr,
        /// The referenced entities, in argument order, frozen at
        /// authoring time.
        refs: Vec<MeasureRef>,
    },
    /// **A recorded tolerance requirement** (ERROR-DESIGN E10): design
    /// intent as document data — "this web is at least 0.5 mm" lives
    /// in the versioned, diffable recipe, not in a script beside it.
    ///
    /// **Report-only, structurally.** The node's value is a verdict
    /// ([`crate::AssertionVerdict`]) and no op in the vocabulary
    /// accepts a verdict as an operand, so a `Violated` assertion
    /// cannot reach any downstream outcome even by mistake: it denotes
    /// no body, the product gather skips it as it skips a
    /// declaration, and `build()` never consults it. E10 v1 rules that
    /// assertions report; a gating mode is additive policy, not a
    /// default this node quietly implements.
    Assertion {
        /// The measure node this constrains — an ordinary DAG edge, so
        /// a failed or poisoned measure poisons its assertions (F2)
        /// rather than producing a verdict about nothing.
        measure: RecipeNodeId,
        /// The bound. Recipe payload rather than a slot: a slot's
        /// address fixes its dimension, and this one's is fixed by the
        /// MEASURE it constrains. It must type-check against that
        /// measure's dimension; a mismatch is a typed document error at
        /// every door, never a silent comparison of radians with
        /// metres.
        bound: Expr,
        /// Which side of the bound the measure must fall on.
        dir: crate::measure::AssertionDir,
    },
}

impl Axis3 {
    /// All three axes, component order (x, y, z).
    pub const ALL: [Axis3; 3] = [Axis3::X, Axis3::Y, Axis3::Z];

    /// The axis as a one-letter label — the one spelling a user-facing
    /// rendering uses, so a component never reaches a reader as
    /// `Debug`.
    pub fn label(self) -> &'static str {
        match self {
            Axis3::X => "x",
            Axis3::Y => "y",
            Axis3::Z => "z",
        }
    }

    /// This axis's position in [`Axis3::ALL`] — the component order
    /// every 3-vector in the recipe is stored and shown in.
    ///
    /// Public because a consumer laying three components out (the
    /// property panel's vector row) needs the same order the recipe
    /// uses, and deriving it by searching `ALL` is both slower and a
    /// second definition of the same fact.
    pub const fn index(self) -> usize {
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

/// A placement-rule node's slot lookup, shared by [`Node::Pattern`] and
/// [`Node::PlacedUnion`] — one rule vocabulary, one slot mapping, so
/// the two nodes can never drift apart on what a slot means.
///
/// `count` is the node's structural count slot when it has one.
/// `Explicit` answers `None` for EVERY slot including `Count`: its
/// placements are the count and carry no expressions, which is exactly
/// what [`Node::slots`] reports for it.
fn rule_expr<'a>(count: Option<&'a Expr>, kind: &'a PatternKind, slot: SlotId) -> Option<&'a Expr> {
    match (kind, slot) {
        (PatternKind::Explicit(_), _) => None,
        (_, SlotId::Count) => count,
        (PatternKind::Linear { direction, .. }, SlotId::Direction(ax)) => Some(comp(direction, ax)),
        (PatternKind::Linear { spacing, .. }, SlotId::Spacing) => Some(spacing),
        (PatternKind::Circular { step, .. }, SlotId::Step) => Some(step),
        _ => None,
    }
}

/// [`rule_expr`]'s mutable twin — same mapping, same `Explicit` rule.
fn rule_expr_mut<'a>(
    count: Option<&'a mut Expr>,
    kind: &'a mut PatternKind,
    slot: SlotId,
) -> Option<&'a mut Expr> {
    match (kind, slot) {
        (PatternKind::Explicit(_), _) => None,
        (_, SlotId::Count) => count,
        (PatternKind::Linear { direction, .. }, SlotId::Direction(ax)) => {
            Some(comp_mut(direction, ax))
        }
        (PatternKind::Linear { spacing, .. }, SlotId::Spacing) => Some(spacing),
        (PatternKind::Circular { step, .. }, SlotId::Step) => Some(step),
        _ => None,
    }
}

impl<P> Node<P> {
    /// The upstream node references — the recipe DAG's edges (spec
    /// D3). Deterministic order (field order).
    pub fn inputs(&self) -> Vec<RecipeNodeId> {
        match self {
            // A leaf whose material crosses the document seam has no
            // DAG edge to offer (A3).
            Node::Datum(_)
            | Node::Profile(_)
            | Node::Declare { .. }
            // A mate is a leaf: its references are NAMES, not edges
            // (A12's reading edges are recomputed, never stored here).
            | Node::Mate { .. }
            | Node::InstantiatePart { .. } => Vec::new(),
            // A measure's references ARE its data dependencies (the
            // variant's docs state why this kind departs from the D3
            // carve-out). The edge is the node each reference is READ
            // AT, not the one that minted the name — reading is what
            // the measure must wait for. Distinct and ascending, so
            // the edge list is a function of the reference SET and two
            // references at one node do not repeat an edge.
            Node::Measure { refs, .. } => {
                let mut v: Vec<RecipeNodeId> = refs.iter().map(|r| r.at).collect();
                v.sort_unstable();
                v.dedup();
                v
            }
            Node::Assertion { measure, .. } => vec![*measure],
            Node::Extrude { profile, .. } => vec![*profile],
            Node::Revolve { profile, axis, .. } => vec![*profile, *axis],
            Node::Loft { profiles, .. } => profiles.clone(),
            Node::Sweep { profile, path, .. } => vec![*profile, *path],
            Node::Fillet { target, .. } | Node::Chamfer { target, .. } => vec![*target],
            Node::Split { target, tool } => vec![*target, *tool],
            Node::Boolean { a, b, declare, .. } => {
                let mut v = vec![*a, *b];
                v.extend(declare.iter().copied());
                v
            }
            Node::Transform { input, .. } => vec![*input],
            // The two placement-rule nodes take the same edges: the
            // body, plus the datum a circular rule turns about.
            Node::Pattern { input, kind, .. } | Node::PlacedUnion { input, kind, .. } => {
                let mut v = vec![*input];
                if let PatternKind::Circular { axis, .. } = kind {
                    v.push(*axis);
                }
                v
            }
        }
    }

    /// The expression slots this node actually carries, deterministic
    /// order — the domain of [`Node::expr`]. Profile nodes enumerate
    /// their PROGRAM's slots (LIB-SWITCH §4c behavior delta 3: the
    /// formerly slot-free payload now carries one slot per continuous
    /// step argument), through the payload's own [`crate::ProfilePayload`]
    /// implementation.
    pub fn slots(&self) -> Vec<SlotId>
    where
        P: crate::ProfilePayload,
    {
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
            Node::Profile(p) => p.slots(),
            // AQ4: an instance takes no arguments in v1 — the
            // referenced document evaluates at its OWN parameters.
            Node::Split { .. }
            | Node::Boolean { .. }
            | Node::Declare { .. }
            // A11: the alignment datum is authored geometry, not a
            // continuous slot — a mate has no expression to drive.
            | Node::Mate { .. }
            | Node::InstantiatePart { .. } => Vec::new(),
            // Neither carries a SLOT. A slot's address fixes its
            // dimension ([`SlotId::dimension`]) — that is the
            // vocabulary's contract, read by the edit door, the load
            // re-check and the GUI alike. A measured expression is not
            // an `Expr` at all, and an assertion's bound takes its
            // dimension from the MEASURE it constrains, which no slot
            // address can state. Both are recipe payload instead, fed
            // to the content key where a fillet's selection is fed and
            // evaluated in their own stage.
            Node::Measure { .. } | Node::Assertion { .. } => Vec::new(),
            Node::Extrude { .. } => vec![SlotId::Distance],
            Node::Fillet { .. } => vec![SlotId::Radius],
            Node::Chamfer { .. } => vec![SlotId::ChamferDistance],
            Node::Revolve { .. } => vec![SlotId::RevolveAngle],
            Node::Loft { .. } => vec![SlotId::VDegree],
            Node::Sweep { .. } => vec![SlotId::Stations, SlotId::VDegree],
            Node::Transform { .. } => {
                let mut s = vec3(SlotId::Translation).to_vec();
                s.extend(vec3(SlotId::RotationAxis));
                s.push(SlotId::RotationAngle);
                s
            }
            Node::Pattern { kind, .. } | Node::PlacedUnion { kind, .. } => match kind {
                PatternKind::Linear { .. } => {
                    let mut s = vec![SlotId::Count];
                    s.extend(vec3(SlotId::Direction));
                    s.push(SlotId::Spacing);
                    s
                }
                PatternKind::Circular { .. } => vec![SlotId::Count, SlotId::Step],
                // The listed placements ARE the rule: no count slot
                // (the list's length is the count) and no expressions
                // (the frames are structural data, D8).
                PatternKind::Explicit(_) => Vec::new(),
            },
        }
    }

    /// The expression in a named slot, `None` if this node type does
    /// not carry that slot (named access only, spec D5).
    pub fn expr(&self, slot: SlotId) -> Option<&Expr>
    where
        P: crate::ProfilePayload,
    {
        use SlotId as S;
        match (self, slot) {
            (Node::Profile(p), S::Profile { .. }) => p.expr(slot),
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
            (Node::Fillet { radius, .. }, S::Radius) => Some(radius),
            (Node::Chamfer { distance, .. }, S::ChamferDistance) => Some(distance),
            (Node::Revolve { angle, .. }, S::RevolveAngle) => Some(angle),
            (Node::Loft { v_degree, .. }, S::VDegree)
            | (Node::Sweep { v_degree, .. }, S::VDegree) => Some(v_degree),
            (Node::Sweep { stations, .. }, S::Stations) => Some(stations),
            (Node::Transform { translation, .. }, S::Translation(ax)) => {
                Some(comp(translation, ax))
            }
            (Node::Transform { rotation_axis, .. }, S::RotationAxis(ax)) => {
                Some(comp(rotation_axis, ax))
            }
            (Node::Transform { rotation_angle, .. }, S::RotationAngle) => Some(rotation_angle),
            (Node::Pattern { count, kind, .. }, s) => rule_expr(Some(count), kind, s),
            (Node::PlacedUnion { count, kind, .. }, s) => rule_expr(count.as_ref(), kind, s),
            // EXHAUSTIVE on the NODE axis, open on the slot axis: a new
            // node kind must be classified here or the compile breaks,
            // while "this node does not carry that slot" stays the
            // honest answer for a slot the listed arms did not claim.
            // `Pattern` and `PlacedUnion` are absent because their arms
            // above already bind every slot.
            (
                Node::Datum(..)
                | Node::Profile(..)
                | Node::Extrude { .. }
                | Node::Revolve { .. }
                | Node::Loft { .. }
                | Node::Sweep { .. }
                | Node::Fillet { .. }
                | Node::Chamfer { .. }
                | Node::Split { .. }
                | Node::Boolean { .. }
                | Node::Transform { .. }
                | Node::Declare { .. }
                | Node::InstantiatePart { .. }
                | Node::Mate { .. }
                | Node::Measure { .. }
                | Node::Assertion { .. },
                _,
            ) => None,
        }
    }

    /// Mutable access to a named slot's expression (the edit layer's
    /// substrate; all validation lives in `apply`, spec D6).
    pub fn expr_mut(&mut self, slot: SlotId) -> Option<&mut Expr>
    where
        P: crate::ProfilePayload,
    {
        use SlotId as S;
        match (self, slot) {
            (Node::Profile(p), S::Profile { .. }) => p.expr_mut(slot),
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
            (Node::Fillet { radius, .. }, S::Radius) => Some(radius),
            (Node::Chamfer { distance, .. }, S::ChamferDistance) => Some(distance),
            (Node::Revolve { angle, .. }, S::RevolveAngle) => Some(angle),
            (Node::Loft { v_degree, .. }, S::VDegree)
            | (Node::Sweep { v_degree, .. }, S::VDegree) => Some(v_degree),
            (Node::Sweep { stations, .. }, S::Stations) => Some(stations),
            (Node::Transform { translation, .. }, S::Translation(ax)) => {
                Some(comp_mut(translation, ax))
            }
            (Node::Transform { rotation_axis, .. }, S::RotationAxis(ax)) => {
                Some(comp_mut(rotation_axis, ax))
            }
            (Node::Transform { rotation_angle, .. }, S::RotationAngle) => Some(rotation_angle),
            (Node::Pattern { count, kind, .. }, s) => rule_expr_mut(Some(count), kind, s),
            (Node::PlacedUnion { count, kind, .. }, s) => rule_expr_mut(count.as_mut(), kind, s),
            // EXHAUSTIVE on the NODE axis, open on the slot axis (the
            // `expr` rule).
            (
                Node::Datum(..)
                | Node::Profile(..)
                | Node::Extrude { .. }
                | Node::Revolve { .. }
                | Node::Loft { .. }
                | Node::Sweep { .. }
                | Node::Fillet { .. }
                | Node::Chamfer { .. }
                | Node::Split { .. }
                | Node::Boolean { .. }
                | Node::Transform { .. }
                | Node::Declare { .. }
                | Node::InstantiatePart { .. }
                | Node::Mate { .. }
                | Node::Measure { .. }
                | Node::Assertion { .. },
                _,
            ) => None,
        }
    }

    /// The [`StableName`]s this payload REFERENCES — `Declare` pairs, a
    /// fillet's selection, a mate's two heads. Document data, never DAG
    /// edges ([`Node::inputs`] excludes them): the edit door checks at
    /// insertion that each one names a live node, and a later delete may
    /// strand it, which is NAMING-DESIGN N5's dangling-reference
    /// semantics with `Rebind` as the one repair.
    ///
    /// The single answer to "which payloads carry a name": every reader
    /// reads this rather than its own copy of the list. The negative
    /// half is [`name_free_node`], shared with the rewriting twin.
    pub fn payload_names(&self) -> Vec<&StableName> {
        match self {
            Node::Declare { pairs } => pairs.iter().flat_map(|((a, b), _)| [a, b]).collect(),
            Node::Fillet { selection, .. } | Node::Chamfer { selection, .. } => {
                selection.iter().collect()
            }
            // A12: a mate's two heads are the instance-qualified
            // references its reading edges are recomputed from.
            Node::Mate { a, b, .. } => vec![a, b],
            // A measure's references are argument-ORDERED, so they are
            // listed in that order rather than a canonical one.
            Node::Measure { refs, .. } => refs.iter().map(|r| &r.name).collect(),
            name_free_node!() => Vec::new(),
        }
    }

    /// Rewrites every payload reference EXACTLY equal to `from` into
    /// `to`, returning how many it rewrote — the substrate of `Rebind`,
    /// N5's one repair. A set-shaped payload re-canonicalizes, because
    /// `to` may sort elsewhere or already be present: a rebind onto an
    /// already-selected edge SHRINKS the set by one rather than
    /// duplicating it.
    ///
    /// The two must name the same variants; [`name_free_node`] is where
    /// that agreement is held.
    pub(crate) fn rebind_payload_names(&mut self, from: &StableName, to: &StableName) -> usize {
        fn rewrite(name: &mut StableName, from: &StableName, to: &StableName) -> usize {
            if name == from {
                *name = to.clone();
                1
            } else {
                0
            }
        }
        let mut hits = 0usize;
        match self {
            Node::Declare { pairs } => {
                for name in pairs.iter_mut().flat_map(|((a, b), _)| [a, b]) {
                    hits += rewrite(name, from, to);
                }
            }
            Node::Fillet { selection, .. } | Node::Chamfer { selection, .. } => {
                for name in selection.iter_mut() {
                    hits += rewrite(name, from, to);
                }
                if hits > 0 {
                    selection.sort();
                    selection.dedup();
                }
            }
            Node::Mate { a, b, .. } => {
                hits += rewrite(a, from, to);
                hits += rewrite(b, from, to);
            }
            // No re-canonicalization: the order IS argument order, and
            // a rebind onto an already-referenced entity must leave two
            // arguments naming one entity rather than shrink the list
            // and renumber every index the expression holds.
            Node::Measure { refs, .. } => {
                for r in refs.iter_mut() {
                    hits += rewrite(&mut r.name, from, to);
                }
            }
            name_free_node!() => {}
        }
        hits
    }

    /// The node ids [`Node::payload_names`] reaches: the heads whose
    /// existence the insert door checks.
    pub fn named_nodes(&self) -> Vec<RecipeNodeId> {
        self.payload_names().iter().map(|name| name.node).collect()
    }

    /// Builds a [`Node::InstantiatePart`] with the EMPTY interface
    /// record — the authoring constructor. A non-empty record is
    /// mintable only by the refactoring that observed declarations
    /// crossing a cut, which reaches it through
    /// [`Node::instantiate_part_with`]: an authored instance crosses
    /// nothing.
    pub fn instantiate_part(doc_ref: crate::ident::DocRef) -> Self {
        Self::instantiate_part_with(doc_ref, InterfaceRecord::default())
    }

    /// Builds a [`Node::InstantiatePart`] carrying a SEAM record
    /// (ASM-R2b D-4): the split's door, since only a split knows what
    /// crossed its cut. Authoring an instance by hand goes through
    /// [`Node::instantiate_part`] — an authored instance crosses
    /// nothing.
    pub fn instantiate_part_with(
        doc_ref: crate::ident::DocRef,
        interface: InterfaceRecord,
    ) -> Self {
        Node::InstantiatePart { doc_ref, interface }
    }

    /// Builds a [`Node::PlacedUnion`] with a PARAMETRIC rule (linear
    /// or circular) and its structural count.
    ///
    /// `None` for an [`PatternKind::Explicit`] rule: that rule brings
    /// its own placements, so pairing it with a count is the
    /// two-sources-of-truth state — [`Node::placed_union_at`] is its
    /// door. (The edit door refuses the same state on a hand-built
    /// value, so this is the convenient refusal, not the only one.)
    pub fn placed_union(input: RecipeNodeId, count: Expr, kind: PatternKind) -> Option<Self> {
        kind.placements().is_none().then_some(Node::PlacedUnion {
            input,
            count: Some(count),
            kind,
        })
    }

    /// Builds a [`Node::PlacedUnion`] over LISTED absolute frames — the
    /// count is the list's length, so there is no count slot to
    /// disagree with it.
    pub fn placed_union_at(input: RecipeNodeId, placements: Vec<crate::placement::Frame>) -> Self {
        Node::PlacedUnion {
            input,
            count: None,
            kind: PatternKind::Explicit(placements),
        }
    }

    /// What is wrong with this node's placement rule, if anything —
    /// the ONE door the edit gate, the persist re-check and the
    /// evaluation backstop all read, so the three can never diverge on
    /// what a usable rule is. `None` for every non-placement node.
    pub fn placement_rule_fault(&self) -> Option<PlacementRuleFault> {
        let (count_present, kind) = match self {
            // Pattern's count is a non-optional field, so it always
            // "has" one — which is why an explicit list there is
            // always a second answer to the same question.
            Node::Pattern { kind, .. } => (true, kind),
            Node::PlacedUnion { count, kind, .. } => (count.is_some(), kind),
            // EXHAUSTIVE on purpose: a future node kind carrying a
            // placement rule must be classified here or the compile
            // breaks, rather than defaulting to "has no rule" and
            // slipping past all three doors this function is the one
            // answer for.
            Node::Datum(..)
            | Node::Profile(..)
            | Node::Extrude { .. }
            | Node::Revolve { .. }
            | Node::Loft { .. }
            | Node::Sweep { .. }
            | Node::Fillet { .. }
            | Node::Chamfer { .. }
            | Node::Split { .. }
            | Node::Boolean { .. }
            | Node::Transform { .. }
            | Node::Declare { .. }
            | Node::InstantiatePart { .. }
            | Node::Mate { .. }
            | Node::Measure { .. }
            | Node::Assertion { .. } => return None,
        };
        let Some(frames) = kind.placements() else {
            // A stepped rule needs its count slot and nothing else.
            return (!count_present).then_some(PlacementRuleFault::CountSpelling);
        };
        if count_present {
            return Some(PlacementRuleFault::CountSpelling);
        }
        // The list IS the count, so an EMPTY list is the explicit
        // rule's `count < 1` — refused for the same reason
        // `NonPositiveCount` refuses a stepped rule's zero, rather
        // than quietly denoting an empty body (LIB-PLACEDUNION review
        // MAJOR-1).
        if frames.is_empty() {
            return Some(PlacementRuleFault::NoPlacements);
        }
        // A11/A6 parity: a placement frame is held to exactly what
        // `SetPlacement` holds a cluster frame to — finite, and proper
        // (det > 0; admitting mirrors is gated on R4's equivariance
        // audit). Checked HERE so the refusal lands at the edit door
        // with the best diagnostics, not at the kernel's rigidity
        // re-check downstream.
        for (index, frame) in frames.iter().enumerate() {
            if !frame.is_finite() {
                return Some(PlacementRuleFault::NonFiniteFrame { index });
            }
            let determinant = frame.determinant();
            if determinant <= 0.0 {
                return Some(PlacementRuleFault::ImproperFrame { index, determinant });
            }
        }
        None
    }

    /// A `Declare` node whose every pair asserts the CONFORMAL class
    /// — the class the class-less payload always meant.
    ///
    /// This NAMES `Rest` at the call site; it does not default it.
    /// The difference matters: a reader of the call sees which of C4's
    /// classes is being claimed, and a pair that means something else
    /// cannot arrive here by omission. Mixed-class nodes build
    /// [`Node::Declare`] directly.
    pub fn declare_rest(pairs: Vec<(StableName, StableName)>) -> Self {
        Node::Declare {
            pairs: pairs.into_iter().map(|p| (p, ContactClass::Rest)).collect(),
        }
    }

    /// Builds a [`Node::Measure`], checking that every primitive's
    /// reference index addresses a reference the node actually carries
    /// — the ONE door, so an expression whose leaf points past the end
    /// of `refs` is unconstructable rather than an evaluation-time
    /// surprise. The load door re-runs the same check on file data
    /// ([`Node::measure_fault`]).
    pub fn measure(
        expr: crate::measure::MeasureExpr,
        refs: Vec<MeasureRef>,
    ) -> Result<Self, MeasureNodeFault> {
        let node = Node::Measure { expr, refs };
        match node.measure_fault() {
            Some(fault) => Err(fault),
            None => Ok(node),
        }
    }

    /// What is wrong with this node's measured expression, if anything
    /// — the one answer the construction door and the persistence
    /// re-check both read, so the two can never disagree about which
    /// trees are well-formed. `None` for every non-measure node.
    pub fn measure_fault(&self) -> Option<MeasureNodeFault> {
        let Node::Measure { expr, refs } = self else {
            return None;
        };
        let mut prims = Vec::new();
        expr.primitives(&mut prims);
        let arity = u32::try_from(refs.len()).unwrap_or(u32::MAX);
        for prim in prims {
            for index in prim.refs() {
                if index >= arity {
                    return Some(MeasureNodeFault::RefIndexOutOfRange {
                        verb: prim.verb(),
                        index,
                        refs: refs.len(),
                    });
                }
            }
        }
        None
    }

    /// Builds a [`Node::Fillet`] with a CANONICAL selection (sorted,
    /// deduplicated) — the one construction door, so a recipe's bits
    /// do not depend on the order a user clicked in.
    pub fn fillet(target: RecipeNodeId, radius: Expr, selection: Vec<StableName>) -> Self {
        let mut selection = selection;
        selection.sort();
        selection.dedup();
        Node::Fillet {
            target,
            radius,
            selection,
        }
    }

    /// Builds a [`Node::Chamfer`] with a CANONICAL selection (sorted,
    /// deduplicated) — the one construction door, for the reason
    /// [`Node::fillet`] is: a recipe's bits must not depend on the
    /// order a user clicked in.
    pub fn chamfer(target: RecipeNodeId, distance: Expr, selection: Vec<StableName>) -> Self {
        let mut selection = selection;
        selection.sort();
        selection.dedup();
        Node::Chamfer {
            target,
            distance,
            selection,
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
    pub fn bit_eq(&self, other: &Node<P>) -> bool
    where
        P: crate::ProfilePayload,
    {
        if self != other {
            return false;
        }
        // The expressions no slot addresses ([`payload_exprs`]) are
        // invisible to the slot walk below, so they are compared here:
        // otherwise a `0.0` and a `-0.0` assertion bound would be one
        // node to every D7 comparator. Equal payloads carry the same
        // payload expressions in the same order (`self != other` has
        // already returned), so the two vectors align.
        match (payload_exprs(self), payload_exprs(other)) {
            (Some(a), Some(b)) => {
                if a.len() != b.len() || !a.iter().zip(&b).all(|(x, y)| x.bit_eq(y)) {
                    return false;
                }
            }
            (None, None) => {}
            _ => return false,
        }
        // A measured expression's own literals live inside the
        // `MeasureExpr`, which `payload_exprs` reaches only the value
        // leaves of — the primitives and the tree shape are compared by
        // `PartialEq` above, and the leaves' bits here.
        if let (Node::Measure { expr: a, .. }, Node::Measure { expr: b, .. }) = (self, other)
            && !a.bit_eq(b)
        {
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
