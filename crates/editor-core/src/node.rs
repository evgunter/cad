//! The recipe-node vocabulary AS DATA (ratified F4; spec D3). No node
//! here evaluates anything — the evaluation service interprets this
//! data against the kernel ops.

use crate::expr::{Dimension, Expr};
use crate::names::SplitHalf;
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
///
/// [`Node::payload_read_sites`] is a THIRD reader, and of a weaker
/// claim: a variant that references no name references nothing to
/// have a read site FOR, so this pattern is the no-site answer's
/// bulk. The variants it does not cover — the named ones whose
/// references are read at their own mints — are spelled beside it
/// there, so that match is exhaustive too.
macro_rules! name_free_node {
    () => {
        $crate::node::Node::Datum(
            $crate::node::Datum::Plane { .. }
                | $crate::node::Datum::Axis { .. }
                | $crate::node::Datum::Point { .. }
                | $crate::node::Datum::Frame { .. }
                | $crate::node::Datum::AxisInPlane { .. },
        ) | $crate::node::Node::Profile(_)
            | $crate::node::Node::Extrude { .. }
            | $crate::node::Node::Revolve { .. }
            | $crate::node::Node::Tube { .. }
            | $crate::node::Node::HollowTube { .. }
            | $crate::node::Node::Loft { .. }
            | $crate::node::Node::Sweep { .. }
            | $crate::node::Node::Split { .. }
            | $crate::node::Node::Boolean { .. }
            | $crate::node::Node::Union { .. }
            | $crate::node::Node::Transform { .. }
            | $crate::node::Node::Pattern { .. }
            | $crate::node::Node::Part { .. }
            | $crate::node::Node::PlacedUnion { .. }
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
pub struct RecipeNodeId(pub u64);

/// **A profile program step's identity** (`names/README.md`, "N1, the
/// profile pieces"): minted from the document's monotone step counter
/// when the step is authored — by `InsertNode` or `SetProgram` — never
/// reused, never positional, and unique across the document. A
/// profile piece's name spells it ([`crate::names::ProfileEdgeRef`]).
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
pub struct StepId(pub u64);

pub use crate::names::{EntityKind, FaceName, RoleSeg, StableName};

/// A coordinate axis, naming vector components in slot identities
/// (spec D5: slots are NAMED, never positional indices).
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
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
    /// The arrival spec's swept central angle.
    ///
    /// **This role and the two below it exist for HAND-BUILT
    /// programs.** No recording surface can put a `Sweep`, `ArcLen` or
    /// `Bulge` in a fused step's arrival position — `profile`'s
    /// `family::ArrivalSpec` is implemented for `Center`, `Via` and
    /// `Radius` alone — and the replay lattice refuses the shape, so
    /// both document doors (`InsertNode`'s VQ9 check and the
    /// persistence snapshot walk) reject a program carrying one.
    /// [`crate::ProgramStep`]'s fields are public data by design (the
    /// node-slot pattern), so such a program is nonetheless
    /// REPRESENTABLE and is a supported construction; slot addressing
    /// is total over the data type, so the arrival spec's argument
    /// gets its own role rather than sharing the incoming spec's.
    SweepVal2,
    /// The arrival spec's arc length.
    ArcLenVal2,
    /// The arrival spec's bulge.
    Bulge2,
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
            Self::SweepVal2 => "arrival sweep",
            Self::ArcLenVal2 => "arrival arc length",
            Self::Bulge2 => "arrival bulge",
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
            | Self::CarrierRadius2
            | Self::ArcLenVal2 => Dimension::Length,
            Self::AngleVal | Self::TurnVal | Self::Phase | Self::SweepVal | Self::SweepVal2 => {
                Dimension::Angle
            }
            Self::DirX | Self::DirY | Self::Bulge | Self::Bulge2 => Dimension::Scalar,
        }
    }

    /// **Whether an expression in this role is a RADIUS** — the length
    /// an arc is drawn at, as against a coordinate or a distance
    /// travelled.
    ///
    /// The distinction is the vocabulary's own and belongs beside
    /// [`StepArg::dimension`], which cannot make it: every radius is a
    /// `Length` and so is every coordinate. A consumer asking "which of
    /// this step's arguments could name the radius of an edge it drew"
    /// asks here rather than keeping a list of variant names, so a role
    /// added to this enum is answered by the author of that role and
    /// not silently missed.
    ///
    /// The match is exhaustive and takes no wildcard arm: a new role
    /// fails to compile until it is dispositioned.
    #[must_use]
    pub fn is_radius(self) -> bool {
        match self {
            Self::Radius | Self::CarrierRadius | Self::CarrierRadius2 => true,
            Self::PointX
            | Self::PointY
            | Self::TargetX
            | Self::TargetY
            | Self::ViaX
            | Self::ViaY
            | Self::CenterX
            | Self::CenterY
            | Self::DirX
            | Self::DirY
            | Self::AngleVal
            | Self::TurnVal
            | Self::Length
            | Self::Bulge
            | Self::Phase
            | Self::SweepVal
            | Self::ArcLenVal
            | Self::Center2X
            | Self::Center2Y
            | Self::Via2X
            | Self::Via2Y
            | Self::Target2X
            | Self::Target2Y
            | Self::SweepVal2
            | Self::ArcLenVal2
            | Self::Bulge2 => false,
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
    /// A datum frame's first in-plane direction component — sketch +x
    /// (Scalar).
    U(Axis3),
    /// A datum frame's second in-plane direction component — sketch +y
    /// (Scalar). Orthogonalized against `u` at evaluation.
    V(Axis3),
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
    /// A shell's wall thickness (Length) — the magnitude every boundary
    /// face is offset inward by. Named apart from [`SlotId::Radius`]
    /// and [`SlotId::ChamferDistance`] for the reason those two are
    /// named apart from each other: a wall thickness is neither a
    /// rolling ball's radius nor a setback along a support, and a
    /// panel that spelled it as either would be lying about it.
    ShellThickness,
    /// A revolve's sweep angle (Angle).
    RevolveAngle,
    /// A derived frame's SPIN — the authored rotation of sketch +x
    /// about the face's outward normal, from the carrier's own
    /// u-reference (Angle). The one continuous slot a
    /// [`Datum::FaceFrame`] carries: its origin and normal are read
    /// off the face, so the spin is the whole of what an author
    /// chooses.
    Spin,
    /// A tube's MAJOR radius — the spine circle's radius, from the
    /// spine centre to the tube's own centreline (Length).
    ///
    /// Both tube kinds carry it: hollowness is spelled by node kind,
    /// so the parameters the two artifacts share have one slot each.
    TubeMajorRadius,
    /// A tube's MINOR radius — the tube's own cross-sectional radius
    /// (Length). On [`crate::Node::HollowTube`] this is the OUTER
    /// minor radius, exactly as the kernel door reads it.
    ///
    /// Named apart from [`SlotId::Radius`] for the reason
    /// [`SlotId::ChamferDistance`] is: a blend radius is a rolling
    /// ball's, and a panel that spelled both "radius" would be lying
    /// about one of them.
    TubeMinorRadius,
    /// A tube window's start angle about the spine axis, measured from
    /// the reference direction (Angle). Present only on an
    /// [`TubeWindow::Arc`] window — a full ring carries no window
    /// slot, because there is no angle to drive.
    TubeWindowStart,
    /// A tube window's end angle, same frame and units
    /// ([`SlotId::TubeWindowStart`]).
    TubeWindowEnd,
    /// A hollow tube's wall thickness (Length) — the one slot
    /// [`crate::Node::Tube`] does not carry, because a solid tube has
    /// no wall to drive.
    TubeWall,
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
    /// A [`crate::Node::Part`]'s INSTANCE INDEX into a pattern's
    /// value — STRUCTURAL (Count-typed, edited only via
    /// `SetStructuralParam`): which body the projection selects is
    /// structure, not a continuous quantity. Its own slot rather than
    /// a reuse of [`SlotId::Count`]: a panel that spelled an index
    /// "count" would be lying about it.
    Instance,
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
    /// stable under every slot edit because program STRUCTURE changes
    /// only by [`crate::DocEdit::SetProgram`], which reports every name
    /// its reshaping strands and rebinds every name it moves (V2,
    /// `crates/profile/README.md`); for the carrier loop forms
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
///
/// **Nothing enumerates the families, and that is the design.** A
/// consumer reaches a family from a slot ([`SlotId::component`]) and
/// its slots back from the family ([`VectorSlot::slot`]); the one site
/// that groups by family — the properties panel's `group_rows`
/// (`crates/viewer/src/props.rs`) — walks the slots a node actually
/// lists, so a family added here is grouped there without an edit. An
/// array of every variant would not be a census of this declaration
/// either: a family added to the enum reds [`VectorSlot::slot`] and
/// [`VectorSlot::label`] and nothing else, so such a list compiles
/// unchanged one family short of the enum it claims to hold.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum VectorSlot {
    /// A datum's origin / a datum point's position ([`SlotId::Origin`]).
    Origin,
    /// A datum plane's normal ([`SlotId::Normal`]).
    Normal,
    /// A datum axis's / linear pattern's direction
    /// ([`SlotId::Direction`]).
    Direction,
    /// A datum frame's first in-plane direction ([`SlotId::U`]).
    U,
    /// A datum frame's second in-plane direction ([`SlotId::V`]).
    V,
    /// A transform's translation ([`SlotId::Translation`]).
    Translation,
    /// A transform's rotation axis ([`SlotId::RotationAxis`]).
    RotationAxis,
}

impl VectorSlot {
    /// This family's slot for one axis — the inverse of
    /// [`SlotId::component`], and total.
    pub fn slot(self, axis: Axis3) -> SlotId {
        match self {
            Self::Origin => SlotId::Origin(axis),
            Self::Normal => SlotId::Normal(axis),
            Self::Direction => SlotId::Direction(axis),
            Self::U => SlotId::U(axis),
            Self::V => SlotId::V(axis),
            Self::Translation => SlotId::Translation(axis),
            Self::RotationAxis => SlotId::RotationAxis(axis),
        }
    }

    /// The family as a prose noun — the one spelling a user-facing
    /// rendering uses.
    pub fn label(self) -> &'static str {
        match self {
            Self::Origin => "origin",
            Self::Normal => "normal",
            Self::Direction => "direction",
            // The SKETCH's names for them, not the vocabulary's: a
            // reader picking a frame's axes is thinking in the 2D
            // coordinates they are about to draw, and "u" alone on a
            // panel says nothing.
            Self::U => "x axis",
            Self::V => "y axis",
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
            | Self::ShellThickness
            | Self::TubeMajorRadius
            | Self::TubeMinorRadius
            | Self::TubeWall
            | Self::Translation(_)
            | Self::Spacing => Dimension::Length,
            Self::Normal(_)
            | Self::Direction(_)
            | Self::U(_)
            | Self::V(_)
            | Self::RotationAxis(_) => Dimension::Scalar,
            Self::RevolveAngle
            | Self::Spin
            | Self::RotationAngle
            | Self::Step
            | Self::TubeWindowStart
            | Self::TubeWindowEnd => Dimension::Angle,
            Self::Count | Self::VDegree | Self::Stations | Self::Instance => Dimension::Count,
            // Profile-program roles carry V2's per-role table; none is
            // Count, so `is_structural` stays false for every StepArg:
            // program structure is the STEP LIST, which no slot
            // addresses — it changes by `DocEdit::SetProgram`, which
            // rebinds every kept name and retires the rest (DM7).
            Self::Profile { arg, .. } => arg.dimension(),
        }
    }

    /// **Spec D6's slot rule over ONE address and one candidate
    /// expression**: the comparison itself, with nowhere else to write
    /// it down.
    ///
    /// Every door that decides whether an expression may sit in a slot
    /// asks this — [`Node::slot_dimension_fault`] per slot of a node
    /// the document already holds, and `edit`'s `set_slot` of an
    /// expression the node does not hold YET, which is why the subject
    /// is a `(slot, expr)` pair rather than a node.
    pub(crate) fn dimension_fault(self, expr: &Expr) -> Option<SlotDimensionFault> {
        (expr.dim() != self.dimension()).then(|| SlotDimensionFault {
            slot: self,
            expected: self.dimension(),
            found: expr.dim(),
        })
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
            Self::ShellThickness => "shell thickness".to_owned(),
            Self::RevolveAngle => "revolve angle".to_owned(),
            Self::Spin => "spin".to_owned(),
            Self::TubeMajorRadius => "tube major radius".to_owned(),
            Self::TubeMinorRadius => "tube minor radius".to_owned(),
            Self::TubeWindowStart => "tube window start".to_owned(),
            Self::TubeWindowEnd => "tube window end".to_owned(),
            Self::TubeWall => "tube wall".to_owned(),
            Self::RotationAngle => "rotation angle".to_owned(),
            Self::Spacing => "spacing".to_owned(),
            Self::Step => "angular step".to_owned(),
            Self::Count => "count".to_owned(),
            Self::Instance => "instance".to_owned(),
            Self::VDegree => "v degree".to_owned(),
            Self::Stations => "stations".to_owned(),
            Self::Profile { loop_, step, arg } => {
                format!("loop {loop_} step {step} · {}", arg.label())
            }
            // Every component variant answered above.
            Self::Origin(_)
            | Self::Normal(_)
            | Self::Direction(_)
            | Self::U(_)
            | Self::V(_)
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
            Self::U(axis) => Some((VectorSlot::U, axis)),
            Self::V(axis) => Some((VectorSlot::V, axis)),
            Self::Translation(axis) => Some((VectorSlot::Translation, axis)),
            Self::RotationAxis(axis) => Some((VectorSlot::RotationAxis, axis)),
            Self::Distance
            | Self::Radius
            | Self::ChamferDistance
            | Self::ShellThickness
            | Self::RevolveAngle
            | Self::Spin
            | Self::TubeMajorRadius
            | Self::TubeMinorRadius
            | Self::TubeWindowStart
            | Self::TubeWindowEnd
            | Self::TubeWall
            | Self::RotationAngle
            | Self::Spacing
            | Self::Step
            | Self::Count
            | Self::Instance
            | Self::VDegree
            | Self::Stations
            | Self::Profile { .. } => None,
        }
    }
}

/// A datum construction (F4: plane/axis/point, plus the two sketch
/// frames), defined by expression slots and — for the derived frame —
/// a DAG edge and a frozen name; geometry is produced by PR 2's
/// evaluation, never here.
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
    /// **An oriented plane**: a plane through `origin` spanned by `u`
    /// and `v`, with normal u × v — the sketch frame a 2D profile is
    /// drawn on.
    ///
    /// [`Datum::Plane`] is origin plus normal, which pins five of a
    /// placement's six rigid degrees of freedom. The sixth — the spin
    /// about the normal — is what a sketch's x and y axes ARE, so a
    /// plane cannot serve as a sketch frame and a frame is not a
    /// dressed-up plane: a section cut wants the surface and would
    /// have to ignore the spin. Both stay, named apart.
    ///
    /// `u` and `v` are authored as arbitrary expressions and
    /// ORTHONORMALIZED at evaluation (PR 2's `wire`), which is also
    /// where a degenerate or parallel pair refuses loudly. Nothing is
    /// checked here — this vocabulary carries expression slots, never
    /// geometry.
    Frame {
        /// Origin components, Length ([`SlotId::Origin`]) — sketch
        /// (0, 0) in world space.
        origin: [Expr; 3],
        /// First in-plane direction, sketch +x, Scalar
        /// ([`SlotId::U`]).
        u: [Expr; 3],
        /// Second in-plane direction, sketch +y, Scalar
        /// ([`SlotId::V`]). Orthogonalized against `u`, so only its
        /// component perpendicular to `u` is read.
        v: [Expr; 3],
    },
    /// **An axis that lives IN a sketch frame**, authored in that
    /// frame's own 2-D coordinates — a revolve's axis of revolution.
    ///
    /// [`Datum::Axis`] is a world-space line, and a revolve's axis has
    /// to lie in the profile's plane. Spelling that axis in 3-D means
    /// authoring six numbers whose legality is a *coincidence* the
    /// evaluator then has to check, and checking it is a tolerance
    /// decision on a direction residual — the audit's F15 row, whose
    /// executed consequence is that a tilt classifies in-plane at
    /// every model scale while the deviation it induces crosses the
    /// band between a millimetre and a ten-metre profile.
    ///
    /// Four numbers in the frame's own coordinates cannot be out of
    /// plane. So this variant does not make the check cheaper — it
    /// makes the error **unrepresentable**, and the residual question
    /// ("is this the SAME plane the profile is drawn on?") is answered
    /// by comparing `plane` against the profile's, an identity of node
    /// ids with no band and no scale.
    ///
    /// Nothing is lost by it: every 3-D axis a revolve could legally
    /// have taken lay in the profile's plane by definition, so it was
    /// always expressible here — and here it is expressible only in
    /// the ways that are legal.
    AxisInPlane {
        /// The [`Datum::Frame`] node this axis lives in. A DAG input,
        /// exactly as a profile's plane is: the frame is the meaning
        /// of the two coordinate pairs below, so an axis without it is
        /// four numbers about nothing.
        plane: RecipeNodeId,
        /// A point on the axis, in the frame's 2-D coordinates —
        /// Length, [`SlotId::Origin`]`(X | Y)`. There is no `Z` slot:
        /// the third coordinate of a point in a plane is not a number
        /// somebody may type.
        origin: [Expr; 2],
        /// The axis direction in the frame's 2-D coordinates — Scalar,
        /// [`SlotId::Direction`]`(X | Y)`. Normalized at evaluation,
        /// where a degenerate pair refuses loudly.
        direction: [Expr; 2],
    },
    /// **A sketch frame DERIVED from a face**
    /// (`crates/editor-core/REFERENCES.md` DM1): a [`Datum::Frame`] whose
    /// pose is computed at evaluation
    /// from a named face of an upstream body — origin the carrier's
    /// own distinguished point, normal the face's OUTWARD normal,
    /// sketch +x the carrier's u-reference rotated by `spin`. It
    /// evaluates to the same value an authored frame does, so every
    /// reader of a frame takes it unchanged.
    ///
    /// It is derived, not frozen: a frame read off a face and written
    /// into nine literals would reintroduce the placement snapshot the
    /// profile-plane migration deleted, one node out, and lie about
    /// why it sits where it sits. As a DAG input the face's body is
    /// upstream, the frame moves when the face moves, and it
    /// participates in the memo and content key like every node.
    ///
    /// The failure mode is the fillet's: a face name that stops
    /// resolving fails the frame typed and poisons the sketch above
    /// it, exactly as a blend's selection does, and the repair is
    /// `Rebind`. It is the first datum with an N5 failure mode.
    ///
    /// The normal is the OUTWARD one: the face's orientation sense
    /// times the carrier's chart axis, both read off the face, so a
    /// sketch on the underside of a plate faces out of the plate.
    FaceFrame {
        /// The body-denoting node the face is read out of — a DAG
        /// input, exactly as [`Datum::AxisInPlane::plane`] is.
        at: RecipeNodeId,
        /// The face, as a frozen name resolved through `at`'s value
        /// under the N5 ladder ([`Node::payload_names`] lists it, so
        /// the insert door's liveness check and `Rebind` reach it).
        face: StableName,
        /// The rotation of sketch +x about the outward normal, from
        /// the carrier's u-reference — Angle, [`SlotId::Spin`].
        spin: Expr,
    },
}

/// One declaration crossing a split seam (ASM-4 D-2; ASSEMBLY-DESIGN
/// A4: "the seam is the crossing declarations" — each entry is a
/// (wrapped name, declaration) pair against the pinned document).
///
/// **INHABITED as of ASM-R2b D-4** — the hook ASM-4 named is taken up
/// by its one intended inhabitant, the crossing MATE EDGE. The
/// obligation ASM-4 recorded here is discharged with it: the record
/// now feeds the instantiate node's content key and is file data.
///
/// An enum with a single variant, not a struct, for the reason ASM-4
/// gave: a crossing is whatever KIND of edge crossed, and mates are
/// the only kind of edge that can cross today. A second kind extends
/// this enum rather than retrofitting a shape onto the first.
///
/// **A crossing's two references are FACE names** ([`FaceName`]), the
/// kind fixed by the type as a mate head's is. A crossing is written
/// out of the two heads of a mate ([`SitedFace`]s), so the fields are
/// face names by construction, and this is the record SAYING what the
/// split guarantees rather than the readers re-asking it. The wire
/// asks the question once, in `FaceName`'s `Deserialize`, so a file
/// whose crossing names an edge refuses at the load door's parse; the
/// split's own re-wrap is one call at this boundary rather than one
/// per reader.
///
/// **A crossing cannot be built from a bare name**, which is the whole
/// claim, pinned where a claim about types belongs:
///
/// ```compile_fail,E0308
/// let _ = editor_core::InterfaceCrossing::Mate {
///     class: editor_core::ContactClass::Rest,
///     outer: named(editor_core::EntityKind::Edge),
///     inner: named(editor_core::EntityKind::Edge),
/// };
///
/// fn named(kind: editor_core::EntityKind) -> editor_core::StableName {
///     editor_core::StableName {
///         kind,
///         node: editor_core::RecipeNodeId(0),
///         path: Vec::new(),
///     }
/// }
/// ```
///
/// The RUNNING twin below — the same body, differing only in that the
/// two references are made through [`FaceName::new`] — is why that
/// block proves anything; [`SitedFace`]'s doc states the rule, for the
/// pair it states it about.
///
/// ```
/// let face = || {
///     editor_core::FaceName::new(named(editor_core::EntityKind::Face))
///         .expect("a face name is a face")
/// };
///
/// let _ = editor_core::InterfaceCrossing::Mate {
///     class: editor_core::ContactClass::Rest,
///     outer: face(),
///     inner: face(),
/// };
///
/// fn named(kind: editor_core::EntityKind) -> editor_core::StableName {
///     editor_core::StableName {
///         kind,
///         node: editor_core::RecipeNodeId(0),
///         path: Vec::new(),
///     }
/// }
/// ```
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
    /// wrapped form (`outer / InPart{ inner }`) is what the
    /// remainder's mate now reads, and re-wrapping is the split's
    /// rebind, so storing the wrapper twice would be storing a
    /// derivable fact.
    ///
    /// **No provenance.** A crossing carries what the seam needs and
    /// nothing about where it came from: the mate the split observed
    /// is not a field, because no door resolves it, nothing
    /// recomputes from it, and an id the record cannot keep honest —
    /// a later delete of the mate is not reported — is a reference
    /// with no reader to protect.
    Mate {
        /// The class the crossing declares.
        #[serde(with = "crate::persist::kernel_wire::contact_class")]
        class: crate::mate::ContactClass,
        /// The remainder-side reference — the one the mate keeps.
        outer: FaceName,
        /// The part-side reference, in the part's own names — the one
        /// that moved, remapped into the part's node numbering.
        inner: FaceName,
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

/// The traversed window of a tube's spine arc, as RECIPE DATA — the
/// document's spelling of [`sweep::TubeWindow`].
///
/// Two spellings, not one with an optional pair: an exactly full ring
/// must SAY [`TubeWindow::Full`], which is the kernel door's own
/// contract (a window reaching one period refuses
/// `FullRangeWindow`). Carrying `Full` as a distinguished variant is
/// what makes that contract expressible in the recipe rather than
/// re-derived from two angles at every reader.
///
/// The variant is STRUCTURAL: it decides whether the node has window
/// slots at all, so it changes by re-authoring the node, never through
/// a slot edit. An `Arc`'s two angles are ordinary continuous slots
/// ([`SlotId::TubeWindowStart`] / [`SlotId::TubeWindowEnd`]).
///
/// **Not canonicalized.** `t1 ≤ t0` is a REVERSED window, which the
/// kernel refuses typed (`DegenerateWindow`); swapping the two here
/// would silently author a different tube than the caller asked for.
///
/// **And so neither tube kind gets a canonicalizing construction
/// door**, which the spec asked for and which is satisfied VACUOUSLY
/// here — measured, not skipped. A canonicalizing door exists where a
/// payload is SET-SHAPED and two spellings denote one artifact:
/// [`Node::fillet`]/[`Node::chamfer`] sort and dedupe a selection,
/// `PlacedUnion` orders its placements. Every field either tube kind
/// carries is a scalar, a bare direction, a node id, or this
/// two-variant window — nothing set-shaped, nothing with a second
/// spelling to fold — so a door could only re-wrap the struct
/// literal. The one place canonicalization could have applied is the
/// window's angle pair, and the paragraph above is why it must not.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub enum TubeWindow {
    /// The full ring — the donut.
    Full,
    /// The arc from `t0` to `t1`, radians about the spine axis from the
    /// reference direction, right-handed. Wedge caps close the ends.
    Arc {
        /// The window's start angle ([`SlotId::TubeWindowStart`]).
        t0: Expr,
        /// The window's end angle ([`SlotId::TubeWindowEnd`]).
        t1: Expr,
    },
}

impl TubeWindow {
    /// This window's slots, deterministic order — empty for a full
    /// ring, the two angles for an arc.
    ///
    /// The one door every "which window slots" question goes through,
    /// so the two node kinds cannot come to disagree about it
    /// ([`PatternKind::placements`] is the same shape for the same
    /// reason).
    pub fn slots(&self) -> Vec<SlotId> {
        match self {
            TubeWindow::Full => Vec::new(),
            TubeWindow::Arc { .. } => vec![SlotId::TubeWindowStart, SlotId::TubeWindowEnd],
        }
    }

    /// The expression in one of this window's slots.
    pub fn expr(&self, slot: SlotId) -> Option<&Expr> {
        match (self, slot) {
            (TubeWindow::Arc { t0, .. }, SlotId::TubeWindowStart) => Some(t0),
            (TubeWindow::Arc { t1, .. }, SlotId::TubeWindowEnd) => Some(t1),
            (TubeWindow::Full | TubeWindow::Arc { .. }, _) => None,
        }
    }

    /// Mutable access to one of this window's slots.
    pub fn expr_mut(&mut self, slot: SlotId) -> Option<&mut Expr> {
        match (self, slot) {
            (TubeWindow::Arc { t0, .. }, SlotId::TubeWindowStart) => Some(t0),
            (TubeWindow::Arc { t1, .. }, SlotId::TubeWindowEnd) => Some(t1),
            (TubeWindow::Full | TubeWindow::Arc { .. }, _) => None,
        }
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

/// **Which body of a multi-body value a [`Node::Part`] selects**
/// (`crates/editor-core/REFERENCES.md` DM3): the named half of a
/// split, or one instance of a pattern by index.
///
/// The two are one enum because the node is one sentence — "this
/// body, out of those" — and the value it reads decides which arm is
/// well-typed: a half against a split, an index against a pattern's
/// instances, and any other pairing refuses at evaluation.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum PartSelect {
    /// The named half of a [`Node::Split`] value.
    SplitHalf(SplitHalf),
    /// The `i`-th instance of a [`Node::Pattern`] value — a
    /// Count-typed STRUCTURAL slot ([`SlotId::Instance`]).
    Instance(Expr),
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
        | Node::Tube { .. }
        | Node::HollowTube { .. }
        | Node::Loft { .. }
        | Node::Sweep { .. }
        | Node::Fillet { .. }
        | Node::Chamfer { .. }
        | Node::Shell { .. }
        | Node::Split { .. }
        | Node::Boolean { .. }
        | Node::Union { .. }
        | Node::Transform { .. }
        | Node::Pattern { .. }
        | Node::Part { .. }
        | Node::PlacedUnion { .. }
        | Node::Declare { .. }
        | Node::InstantiatePart { .. }
        | Node::Mate { .. } => None,
    }
}

/// **An entity reference: a name, and the node it is read at.**
///
/// Both halves are load-bearing and they are not the same node.
/// `name` says WHICH entity (N1: the name embeds the node that minted
/// it); `at` says which node's geometry that entity is being spoken
/// about. They coincide for a reference to a body's own minting node
/// and diverge the moment anything PLACES that body — a transform is
/// identity-preserving, so it mints no name of its own and a name
/// resolved through it still points at the minting node while the
/// geometry has moved.
///
/// **One reader: [`Node::Measure`].** Its reference reads the carrier
/// out of `at`'s evaluated value, so `at` is an ordinary DAG edge
/// ([`Node::inputs`]) and `name` resolves against `at`'s own evaluated
/// name table, through the N5 ladder every other authored name takes —
/// the carrier has to be findable there or the measure has nothing to
/// read. `name` is a bare [`StableName`] because a measure reads a
/// LENGTH between entities of any kind: a face, an edge, a vertex, a
/// whole body.
///
/// A mate's head is the other sited reference in the vocabulary and is
/// its own type, [`SitedFace`] — not this one with a different name in
/// it. What `at` means there is a different fact (an A12 reading edge,
/// never consuming) about a name that resolves somewhere else (the
/// PRODUCT's table, at the at-rest gate), so the two carry their own
/// contracts rather than one doc saying "it depends who holds it".
///
/// There is no `Option` on `at`: "as authored" is spelled
/// [`SitedRef::at_mint`].
///
/// **`Rebind` never moves a measure's `at`.** A measure's `at` is a
/// DAG edge the author chose, and an edit that rewrote it would be
/// re-pointing a dependency behind the author's back; only the NAME
/// is repaired (`Node::rebind_payload_names`). [`SitedFace`]'s doc
/// states the other half of that one repair.
#[derive(
    Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
#[serde(deny_unknown_fields)]
pub struct SitedRef {
    /// The node whose evaluated value the carrier is read at — the
    /// PLACED geometry, when that node placed it.
    pub at: RecipeNodeId,
    /// The entity's stable name, resolved against `at`'s table.
    pub name: StableName,
}

impl SitedRef {
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

/// **A mate head: a FACE name, and the node it is read at.**
///
/// [`SitedRef`]'s shape with the name's kind fixed by the type. A mate
/// declares a FACE-PAIR contact, so a head that names a body, an edge
/// or a vertex is a different statement — and because the kind is data
/// on the name rather than a property of some product, the requirement
/// is expressible where it belongs: in the type of the field. A mate
/// whose head is a bare [`StableName`] does not compile, so no door
/// downstream has a document to refuse.
///
/// Where a face name comes from is [`FaceName`]'s own doc: the three
/// boundaries that turn DATA into one, and the single in-crate door
/// that re-derives one without re-asking the kind.
///
/// **`at` is an A12 READING edge** — never consuming, or the mated
/// bodies would leave A10's root set. It names the OPERAND the mate is
/// authored against, and the solve walks from it down to the name's
/// head, composing every pose-bearing node it passes
/// ([`crate::mate::member_of`]). The name resolves nowhere at the
/// solve: the solve is structural and inspects no geometry, so it
/// reads the name's HEAD and its `Instance(i)` qualifiers as recipe
/// data and nothing more, and the name is resolved later against the
/// PRODUCT's table by the at-rest gate that mints the declaration.
///
/// **`Rebind` moves a head's at-mint operand**, which is the half of
/// that one repair a measure does not have ([`SitedRef`]'s doc states
/// the other): a head read at its own mint is the reference saying
/// "read me where I was minted", so the operand follows the name it
/// was authored to coincide with, while a head read somewhere ELSE
/// keeps its operand — that node is an authored fact the edit knows
/// nothing about.
///
/// **A mate cannot be built from a bare name**, which is the whole
/// claim, pinned where a claim about types belongs:
///
/// ```compile_fail,E0308
/// fn head() -> editor_core::SitedRef {
///     editor_core::SitedRef::at_mint(named(editor_core::EntityKind::Edge))
/// }
///
/// let _: editor_core::Node<editor_core::ProfileProgram> = editor_core::Node::Mate {
///     a: head(),
///     b: head(),
///     class: editor_core::ContactClass::Rest,
///     alignment: alignment(),
/// };
///
/// fn named(kind: editor_core::EntityKind) -> editor_core::StableName {
///     editor_core::StableName {
///         kind,
///         node: editor_core::RecipeNodeId(0),
///         path: Vec::new(),
///     }
/// }
///
/// fn alignment() -> editor_core::Alignment {
///     let frame = editor_core::MateFrame {
///         origin: [0.0, 0.0, 0.0],
///         axis: [0.0, 0.0, 1.0],
///         reference: [1.0, 0.0, 0.0],
///     };
///     editor_core::Alignment {
///         a: frame,
///         b: frame,
///         primitive: editor_core::MatePrimitive::FrameCoincidence,
///         sense: editor_core::AxisSense::Aligned,
///         clocking: None,
///     }
/// }
/// ```
///
/// **What that row proves, and what it does not.** Stable rustdoc
/// checks only that the block FAILS to build; it does not enforce the
/// `,E0308` named beside it, so a row whose body had a typo, a
/// renamed field or a missing import would pass just as well and
/// prove nothing about the head's type. The twin below is the same
/// body with the one difference this claim is about — `head()` returns
/// a [`SitedFace`] made through the constructor instead of a
/// [`SitedRef`] made from a bare name — and it is a RUNNING doctest:
/// every other line above is a line it also compiles, so a defect
/// anywhere but the head reddens here rather than silently satisfying
/// the block above for the wrong reason. (The idiom is
/// `quantity::units`', which states the rule; the code was read off
/// `rustc` on the snippet.)
///
/// ```
/// fn head() -> editor_core::SitedFace {
///     editor_core::SitedFace::at_mint(
///         editor_core::FaceName::new(named(editor_core::EntityKind::Face))
///             .expect("a face name is a face"),
///     )
/// }
///
/// let _: editor_core::Node<editor_core::ProfileProgram> = editor_core::Node::Mate {
///     a: head(),
///     b: head(),
///     class: editor_core::ContactClass::Rest,
///     alignment: alignment(),
/// };
///
/// fn named(kind: editor_core::EntityKind) -> editor_core::StableName {
///     editor_core::StableName {
///         kind,
///         node: editor_core::RecipeNodeId(0),
///         path: Vec::new(),
///     }
/// }
///
/// fn alignment() -> editor_core::Alignment {
///     let frame = editor_core::MateFrame {
///         origin: [0.0, 0.0, 0.0],
///         axis: [0.0, 0.0, 1.0],
///         reference: [1.0, 0.0, 0.0],
///     };
///     editor_core::Alignment {
///         a: frame,
///         b: frame,
///         primitive: editor_core::MatePrimitive::FrameCoincidence,
///         sense: editor_core::AxisSense::Aligned,
///         clocking: None,
///     }
/// }
/// ```
///
/// The constructor's other answer is a typed refusal, never a face
/// name that is not one:
///
/// ```
/// use editor_core::{EntityKind, FaceName, RecipeNodeId, StableName};
/// let edge = StableName {
///     kind: EntityKind::Edge,
///     node: RecipeNodeId(0),
///     path: Vec::new(),
/// };
/// assert_eq!(FaceName::new(edge).unwrap_err().found, EntityKind::Edge);
/// ```
#[derive(
    Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
#[serde(deny_unknown_fields)]
pub struct SitedFace {
    /// The node whose evaluated value the carrier is read at — the
    /// PLACED geometry, when that node placed it.
    pub at: RecipeNodeId,
    /// The face's stable name.
    pub name: FaceName,
}

impl SitedFace {
    /// A head read at the node that minted the name — the degenerate
    /// case, and the honest spelling of "as authored".
    pub fn at_mint(name: FaceName) -> Self {
        Self {
            at: name.node,
            name,
        }
    }

    /// A head read at `at`.
    pub fn new(at: RecipeNodeId, name: FaceName) -> Self {
        Self { at, name }
    }
}

/// **What makes a node's structural content invalid**
/// ([`Node::input_fault`]; DM5) — one vocabulary for the two edit doors
/// and the load door's re-check, so each rule has one definition and
/// three callers rather than three copies.
///
/// It covers the input list (DM5's own subject) and the NAME
/// DESIGNATIONS beside it, because the two are one kind of rule: a
/// structural form that a construction door establishes, and that only
/// a hand-built variant or a corrupt file can arrive without. Which
/// form is the payload's own — ORDERED for a shell's `open`, SORTED for
/// a blend's `selection` — and either way a node that does not hold it
/// is refused at every door that ADMITS a node rather than repaired at
/// one. [`crate::DocEdit::Rebind`] repairs, and that is not an
/// exception: it rewrites through the same canonicalizer the
/// construction doors use, so what it writes is a node these doors
/// accept.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputFault {
    /// One node is reached twice through this node's edges. It covers
    /// a boolean or a split whose two operands coincide and a list
    /// with a repeated entry alike: what "the same body twice" means
    /// does not change with the node kind.
    Duplicate {
        /// The input reached twice.
        input: RecipeNodeId,
    },
    /// A LIST input ([`Node::list_input`]) left with fewer than two
    /// entries. A union of one body is that body and a loft through
    /// one section is not a skin: either is a node whose meaning is
    /// its own input, spelled as an operator.
    TooFew {
        /// How many entries it has.
        found: usize,
    },
    /// An ORDERED designation names one entity twice. A shell's `open`
    /// list is the payload that has one: its order is meaning (the
    /// first designated face of a chart carries the rim), so its
    /// canonical form is "no repeats" rather than "sorted", and the
    /// construction door drops a repeat keeping the first occurrence —
    /// a repeat that reaches a door came from a hand-built variant or
    /// a corrupt file, and is refused rather than repaired.
    RepeatedDesignation {
        /// The position of the entry's first occurrence.
        first: usize,
        /// The position at which it is named again.
        again: usize,
    },
    /// A SORTED designation is not in canonical form. A blend's
    /// `selection` ([`Node::Fillet`], [`Node::Chamfer`]) is the payload
    /// that has one: its order carries no meaning, so it is stored
    /// sorted and deduplicated and two recipes picking the same edges
    /// are bit-identical. That is ONE predicate — the entries strictly
    /// increase — which a repeat and a swap both break, at the position
    /// named here. [`Node::fillet`]/[`Node::chamfer`] are the
    /// construction doors that establish the form; a selection that
    /// reaches a door without it came from a hand-built variant or a
    /// corrupt file and is refused rather than re-sorted, because a
    /// repair would move the node's content key behind the caller's
    /// back.
    SelectionNotCanonical {
        /// The position of the entry that does not sort strictly
        /// before the one after it. ONE index is the whole content:
        /// the break is between this entry and its successor, so the
        /// successor's position is this one plus one. The rendered
        /// sentence spells both out because a reader comparing two
        /// entries wants both numbers in front of them; the payload
        /// carries the one that is data.
        at: usize,
    },
}

// The ONE prose vocabulary for this fault, forwarded by every door
// that renders it rather than restated.
impl core::fmt::Display for InputFault {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Duplicate { input } => write!(
                f,
                "node {} is taken as an input twice — a node's inputs are pairwise distinct",
                input.0
            ),
            Self::TooFew { found } => write!(
                f,
                "a list input takes two or more entries, and this has {found}"
            ),
            Self::RepeatedDesignation { first, again } => write!(
                f,
                "the open-face designation names one face twice (entries {first} and {again}) — \
                 an ordered designation names each face once, the first occurrence carrying the \
                 rim"
            ),
            Self::SelectionNotCanonical { at } => write!(
                f,
                "the blend selection is not canonical (entry {at} does not sort strictly before \
                 entry {}) — a selection is stored sorted and deduplicated, so the same edges \
                 always make the same recipe",
                at + 1
            ),
        }
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

/// **What makes a node's SLOT unusable** ([`Node::slot_dimension_fault`];
/// spec D6) — one vocabulary for the edit doors and the load door, so
/// the rule "a slot's expression carries the dimension the slot
/// address fixes" has one definition rather than one per door.
///
/// The domain is [`Node::slots`], which is EVERY node kind: a profile
/// program's step arguments, an extrude's distance, a datum's
/// coordinates and a pattern's count are the same question asked of
/// different addresses, and a door that asks it of one kind admits
/// files the other doors could not have produced.
/// ONE fact, so a struct: the dimensions disagree. A slot
/// [`Node::slots`] names and [`Node::expr`] cannot answer for is not a
/// property of the document at all — it is a disagreement between two
/// matches in this module, which [`Node::slot_dimension_fault`]
/// asserts against at the site rather than routing to a door as a
/// refusal.
///
/// Its [`Display`](core::fmt::Display) is the refusal's one CLAUSE,
/// which each door forwards into its own subject — the shape
/// [`crate::placement::Frame::admission_fault`] carries for the frame
/// rule. The sentence is written here and reaches a reader as
/// "node 7: slot radius needs …" from the load door and as
/// "slot radius needs …" from the edit door.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct SlotDimensionFault {
    /// The offending slot.
    pub slot: SlotId,
    /// The dimension the address fixes.
    pub expected: Dimension,
    /// The expression's dimension.
    pub found: Dimension,
}

impl core::fmt::Display for SlotDimensionFault {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let Self {
            slot,
            expected,
            found,
        } = self;
        write!(
            f,
            "slot {} needs {} {expected} expression, got {} {found}",
            slot.label(),
            expected.article(),
            found.article()
        )
    }
}

/// What makes a [`Node::Assertion`]'s bound unusable
/// ([`Node::assertion_bound_fault`]) — one vocabulary for the edit
/// door and the load door's re-check.
///
/// Two arms rather than one dimension-or-nothing answer: "the
/// reference is not a measure" and "it is, and it measures something
/// else" are different mistakes with different repairs, and a reader
/// should not have to decode an absent dimension to tell them apart.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum AssertionBoundFault {
    /// The reference names no live measure node — there is no measured
    /// dimension for the bound to agree with.
    TargetNotMeasure {
        /// What the assertion references.
        measure: RecipeNodeId,
        /// The bound's dimension.
        bound: Dimension,
    },
    /// The reference is a measure, and its dimension is not the
    /// bound's: the assertion compares two different quantities.
    DimensionMismatch {
        /// The measure it constrains.
        measure: RecipeNodeId,
        /// What that measure yields.
        measured: Dimension,
        /// The bound's dimension.
        bound: Dimension,
    },
}

impl AssertionBoundFault {
    /// **E10's agreement itself, stated once**: an assertion compares
    /// one quantity, so the bound's declared dimension is the one the
    /// measure yields.
    ///
    /// The entry point for a caller that already HAS the measured
    /// dimension and cannot reach the measure node —
    /// `eval::wire`'s assertion backstop, which reads it off
    /// the evaluated payload, the dimension the measure node's own
    /// expression put there. [`Node::assertion_bound_fault`] is the
    /// entry point for a caller holding the document, and reaches this
    /// one once it has resolved the reference.
    pub(crate) fn against(
        measure: RecipeNodeId,
        measured: Dimension,
        bound: Dimension,
    ) -> Option<Self> {
        (measured != bound).then_some(Self::DimensionMismatch {
            measure,
            measured,
            bound,
        })
    }
}

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
            // The frame clause is the frame rule's own
            // ([`crate::placement::FrameFault`]); this arm supplies
            // only the subject, so the sentence a reader sees about a
            // frame is the same one wherever the frame was refused.
            Self::NonFiniteFrame { index } => {
                write!(
                    f,
                    "placement {index} {}",
                    crate::placement::FrameFault::NonFinite
                )
            }
            Self::ImproperFrame { index, determinant } => write!(
                f,
                "placement {index} {}",
                crate::placement::FrameFault::Improper {
                    determinant: *determinant
                }
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
    /// **A solid tube** — a ring torus, or an elbow of it, from its
    /// INTENT parameters (RECIPE-DOORS D4 as revised): the op is
    /// [`sweep::tube_along_arc`], whose whole reason to exist is that
    /// the numbers the caller gives are the numbers the body stores.
    ///
    /// # Why this is not a revolve of a circle
    ///
    /// It could be authored that way, and the result would be a
    /// different body: a revolve reconstructs its minor radius through
    /// profile→bulge→radius arithmetic, which is where the review
    /// donut's 56 ulps came from. This node reaches the door that
    /// stores the intent verbatim, so a caller recovers
    /// `minor_radius` bit for bit from the body it authored.
    ///
    /// # The anchoring, and why it is spelled this way
    ///
    /// `spine` is a datum-AXIS node, consumed whole: its origin is the
    /// tube's centre and its direction is the spine axis. That is
    /// [`Node::Revolve`]'s precedent verbatim — one datum reference,
    /// both of its parts used.
    ///
    /// `u_ref` is a BARE DIRECTION, which no datum node denotes on its
    /// own, so it is carried as components on
    /// [`SlotId::Direction`] — the spelling `Datum::Plane`'s normal,
    /// `PatternKind::Linear`'s direction and `Node::Transform`'s
    /// rotation axis all use. A second datum-axis reference would
    /// carry an origin nothing reads.
    ///
    /// # The two directions are NOT treated alike, and that asymmetry
    /// is the datum's doing
    ///
    /// `spine` is a datum NODE, and a datum axis normalizes its own
    /// direction when it evaluates ([`mod@crate::eval`]'s `wire_datum`
    /// builds a `UnitVec3`, which is what `Node::Revolve` gets too), so
    /// a spine authored `(0, 0, 2)` is silently the unit z axis and
    /// builds. Only a degenerate or non-finite direction refuses
    /// there, one node upstream, and it refuses as a DATUM fault. The
    /// tube door's own non-unit-axis verdict is therefore unreachable
    /// along the recipe path; it still guards the kernel-direct
    /// caller.
    ///
    /// `u_ref` is a BARE TRIPLE that passes through no datum, so it
    /// reaches the door exactly as written: a non-unit reference, or
    /// one not perpendicular to the axis, refuses TYPED from the door
    /// ([`crate::eval::NodeErrorKind::Tube`]). Nothing normalizes it
    /// here — a silent normalization would be exactly the invention
    /// this door exists to avoid.
    Tube {
        /// The datum-axis node giving the spine's centre (its origin)
        /// and axis (its direction).
        spine: RecipeNodeId,
        /// The reference direction the window's angles are measured
        /// from, components ([`SlotId::Direction`], Scalar).
        u_ref: [Expr; 3],
        /// The spine circle's radius ([`SlotId::TubeMajorRadius`]).
        major_radius: Expr,
        /// The traversed window — a full ring or an arc.
        window: TubeWindow,
        /// The tube's cross-sectional radius
        /// ([`SlotId::TubeMinorRadius`]).
        minor_radius: Expr,
    },
    /// **A hollow tube** — [`Node::Tube`]'s sibling with a WALL: the op
    /// is [`sweep::tube_along_arc_hollow`], and `minor_radius` is the
    /// OUTER minor radius.
    ///
    /// # Why a second kind rather than an optional wall
    ///
    /// A solid tube and a hollow one are different artifacts — a
    /// full-disc cross-section against an annular one — and the
    /// vocabulary says so where callers read (RECIPE-DOORS D4 as
    /// revised by the #1205 ruling). `Option` never appears in the
    /// recipe vocabulary: hollowness is spelled by node kind, which is
    /// the distinction the artifacts already have. The kernel's PUBLIC
    /// DOORS split the same way; that the two share a private
    /// implementation is implementation.
    ///
    /// # The wall is validated KERNEL-SIDE, entirely
    ///
    /// Three separate verdicts stand between a wall and a body — the
    /// thickness is positive, `minor_radius − wall` is a bore, and the
    /// REALIZED gap between the two stored radii is positive — and
    /// none of them is re-derived here. They are decided before
    /// anything is minted and they are what the full ring's cavity
    /// insertion carries as its containment evidence, so a recipe-side
    /// pre-check could only be a second, weaker opinion. Every one of
    /// them crosses as [`crate::eval::NodeErrorKind::Tube`].
    HollowTube {
        /// The datum-axis node giving the spine's centre and axis.
        spine: RecipeNodeId,
        /// The reference direction's components
        /// ([`SlotId::Direction`], Scalar).
        u_ref: [Expr; 3],
        /// The spine circle's radius ([`SlotId::TubeMajorRadius`]).
        major_radius: Expr,
        /// The traversed window — a full ring (a torus shell, whose
        /// cavity is a void) or an arc (an open elbow of annular
        /// section).
        window: TubeWindow,
        /// The OUTER cross-sectional radius
        /// ([`SlotId::TubeMinorRadius`]).
        minor_radius: Expr,
        /// The wall thickness ([`SlotId::TubeWall`]). REQUIRED: the
        /// inner wall stores `minor_radius − wall`, one IEEE
        /// subtraction of the caller's own two numbers.
        wall: Expr,
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
    /// the vector in order). [`Node::fillet`] canonicalizes; every door
    /// that ADMITS a node ASSERTS the form rather than repairing it,
    /// through the one predicate [`Node::input_fault`] states
    /// ([`InputFault::SelectionNotCanonical`]) — so a hand-built
    /// variant at the insert door and a non-canonical file at the load
    /// door are refused alike, and a repair at either would move the
    /// node's content key behind the caller's back.
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
    /// deduplicated by [`Node::chamfer`], and a non-canonical set at
    /// any door refused rather than repaired. The freeze argument does not depend
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
    /// **Hollow `target` into a thin solid** of wall `thickness`, with
    /// the faces in `open` re-authored as annular RIMS.
    ///
    /// The op is the verb seat's `Verb::Shell` over `topo::shell_open`:
    /// every boundary face is replaced by its inward offset and the
    /// offset boundary inserted as a cavity, then each designated
    /// chart is lifted into a rim. Every check — the thickness gate,
    /// the wall-clearance gate, the per-face offset refusals, the
    /// designation gates, the validation of the result — is the
    /// kernel's, carried unaltered as
    /// [`crate::eval::NodeErrorKind::Shell`]; the node never passes
    /// its input body through.
    ///
    /// # `open` is ORDERED, not canonical
    ///
    /// This is the one place the blend selection's canonical form does
    /// not transfer, and the reason is the kernel's own record:
    /// `RimNaming::sources` PRESERVES designation order, and a chart's
    /// rim is its first designated face, so a caller that wants a
    /// particular face to carry the rim's identity names it first.
    /// Sorting would silently change which face the rim inherits.
    /// [`Node::shell`], the one construction door, therefore keeps the
    /// order it is given and DEDUPLICATES keeping the first occurrence;
    /// a repeated name that reaches a door — a hand-built variant at
    /// the insert door, a corrupt file at the load door — is refused
    /// ([`InputFault::RepeatedDesignation`], asked of
    /// [`Node::input_fault`] by both), never quietly repaired.
    ///
    /// # Empty `open` is the sealed hollow
    ///
    /// Legal, and not a refusal: an empty designation IS the sealed
    /// form (`topo::shell`), which has no node of its own because the
    /// seat's own contract says "empty is the sealed hollow". A blend
    /// of nothing is an unfinished recipe; a shell of nothing opened
    /// is a closed thin solid, which is a body.
    ///
    /// # The freeze
    ///
    /// `open` is a set of stable names and nothing else — no "the top
    /// face" spelling and no filter — frozen at authoring time, with
    /// [`crate::DocEdit::Rebind`] the one repair, exactly as
    /// [`Node::Fillet`] states it for its selection. A name resolves
    /// through the target's table to a FACE; anything else refuses
    /// typed ([`crate::eval::NodeErrorKind::ShellOpenKind`]).
    Shell {
        /// The body hollowed.
        target: RecipeNodeId,
        /// The wall thickness — a magnitude
        /// ([`SlotId::ShellThickness`], Length).
        thickness: Expr,
        /// The faces opened into rims, by stable name, IN DESIGNATION
        /// ORDER (first occurrence kept; see the variant docs).
        open: Vec<StableName>,
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
    /// **The n-ary union** (`crates/editor-core/REFERENCES.md` DM4):
    /// two or more
    /// member bodies, ONE body out — the same value shape a pair
    /// union yields, so every consumer of a union is unchanged.
    ///
    /// It sits beside [`Node::Boolean`], which stays for a pair, and
    /// beside [`Node::PlacedUnion`], which fuses instances of one
    /// prototype and is a different sentence.
    ///
    /// # Why the list, and what the list buys
    ///
    /// A pairwise chain records JOIN DEPTH in every name it mints:
    /// boolean naming wraps each operand's names in `FromA`/`FromB`,
    /// so the twentieth member of a chain is twenty segments deep and
    /// removing one link renames every member that joined before it.
    /// A member of this node is named by IDENTITY —
    /// [`crate::RoleSeg::FromMember`] wrapping the member's own name,
    /// one wrapper whatever the fold's depth — so a member's names
    /// depend on neither its position in the list nor on how many
    /// members precede it, and [`crate::DocEdit::SetMembers`] can drop
    /// one without disturbing the rest.
    ///
    /// # The `declare` field, and why it records no position
    ///
    /// Members that touch refuse `UndeclaredContact` exactly as a pair
    /// boolean's operands do, and the recourse is the same one: a
    /// [`Node::Declare`] input. Its pairs name SITED entities
    /// ([`SitedRef`]) — the entity's name in a MEMBER's own table,
    /// with that member beside it. A declaration therefore says "this
    /// face of member `m` meets that face of member `n`" while naming
    /// nothing of this node's own, which is what lets it be authored
    /// BEFORE the union: the `Declare` goes in first and the union
    /// carrying its edge second, in two edits.
    ///
    /// It records no fold position either: the step each pair is fed
    /// at is DERIVED from where its two sites sit in `members`, so
    /// REORDERING the list re-derives the routing rather than
    /// invalidating the declaration. Dropping a declared member is
    /// the other case and is not silent: its site is no longer in the
    /// list, and the next evaluation refuses that pair as a vanished
    /// name (N5), since `SetMembers` leaves `declare` as it was. And
    /// the SITE is the side —
    /// the later member is the joining operand, the earlier is inside
    /// the accumulation — so two members that are transforms of one
    /// body, whose tables are identical (N1), are told apart by the
    /// pair itself.
    ///
    /// Two sites in ONE member are that member's own CARRIED contact,
    /// fed at the step that member joins at — member 0's at the first
    /// step, where it is operand A — which is the pair chain's rule for
    /// a carried contact, on a member instead of an operand.
    ///
    /// A row the FOLD mints (a `Seam`, a `Merged`, a `Fragment`, the
    /// output body) is not a declaration subject at all: it exists
    /// only in this node's own evaluation, after the `Declare` that
    /// would name it, so there is no node to site it at.
    ///
    /// A declared pair resolves at its step through the MERGES the
    /// fold has performed. A declared merge consumes the
    /// two faces it joins and publishes a `Merged` row in their place,
    /// and a member's face that is inside such a row by the time its
    /// pair's step runs resolves TO that row — the one whose flat
    /// constituent set holds it (N3: a merge of a merged face lists
    /// the faces, never the merge, so the row is the same whatever
    /// order the merges happened in). A chain of contacts (`a` to `c`,
    /// `c` to `d`) fuses in every order of the three, with
    /// `Merged({a, c, d})` as the fused cap's row in each.
    ///
    /// Merges are the whole of it. A member face the fold consumed
    /// otherwise — split by a later member, swallowed by containment,
    /// or inside a merged row that was later fragmented — is not
    /// looked through, and a pair naming it resolves only in the
    /// orders that reach it while it is still a row
    /// (`work/wire/member-space-look-through-stops-at-splits-containment-and-fragmented-merges.md`).
    Union {
        /// The member bodies, in fold order (D9: the order is the
        /// list's, and the list is data). Two or more, pairwise
        /// distinct — both held at the edit door
        /// ([`crate::EditError::TooFewMembers`],
        /// [`crate::EditError::DuplicateInput`]).
        members: Vec<RecipeNodeId>,
        /// Optional coincidence-intent input (a `Declare` node), the
        /// same slot [`Node::Boolean`] carries and the same edit-door
        /// check ([`crate::EditError::DeclareInputNotDeclare`]).
        /// [`crate::DocEdit::SetMembers`] leaves it as it was.
        declare: Option<RecipeNodeId>,
    },
    /// **A rigid placement of an upstream value** (F4: Transform):
    /// ONE map, applied to every body the input's value carries, in
    /// that value's own order. Shape-preserving — a body places as a
    /// body, an `Instances` value places as `Instances` — so what
    /// this node takes is a placer's operand and not a body seat
    /// (`eval::wire`'s `placeable_operand`).
    Transform {
        /// The value placed: a body, a boolean's non-empty result, or
        /// an `Instances` value taken whole.
        input: RecipeNodeId,
        /// Translation components, Length ([`SlotId::Translation`]).
        translation: [Expr; 3],
        /// Rotation-axis components, Scalar ([`SlotId::RotationAxis`]).
        rotation_axis: [Expr; 3],
        /// Rotation angle ([`SlotId::RotationAngle`]).
        rotation_angle: Expr,
    },
    /// **A pattern of an upstream value** with a STRUCTURAL
    /// Count-typed index expression (spec D3/A8; N1 `Instance(i)`
    /// will index it): the input is the MASTER, placed WHOLE at every
    /// placement. It takes the same operand [`Node::Transform`] does
    /// (`eval::wire`'s `placeable_operand`), but it is not
    /// shape-preserving the way a transform is — the value is
    /// `Instances` whatever the master was: N bodies for a one-body
    /// master, N·M placement-major for an `Instances` master of M.
    Pattern {
        /// The master replicated: a body, a boolean's non-empty
        /// result, or an `Instances` value placed whole.
        input: RecipeNodeId,
        /// Instance count — the structural slot ([`SlotId::Count`]).
        count: Expr,
        /// The replication rule.
        kind: PatternKind,
    },
    /// **One body out of a multi-body value**
    /// (`crates/editor-core/REFERENCES.md` DM3): the named half of a
    /// [`Node::Split`] value or the `i`-th
    /// instance of a [`Node::Pattern`] value, as a `Body` value every
    /// body-consuming node takes. The recipe's way of saying "union
    /// the upper half of that split into this block" or "subtract
    /// instance 3 of that pattern".
    ///
    /// A projection, not an operation: the selected body is the
    /// half's or the instance's own (the same `Arc`, no clone, no
    /// re-stamp), and the node's name table is the input's table
    /// restricted to that body with every name VERBATIM — a
    /// pass-through in `Transform`'s sense, contributing no role
    /// segment, so every selector already spelled against that half
    /// or that instance resolves here unchanged.
    ///
    /// A node rather than a selector inside every consumer's operand:
    /// one meaning, one node, and every consumer's operand door stays
    /// as it is ([`Node::PlacedUnion`]'s ruling). A bare split or
    /// pattern is still refused at a body seat; this node is how a
    /// user says which body they meant.
    ///
    /// Because it moves nothing and renames nothing, an `Instance`
    /// selection is a pass-through of A11's member walk too
    /// ([`crate::mate::member_of`]): a mate read at one, or below
    /// one, stands on the same member the pattern's copy does.
    ///
    /// A pattern of a pattern does NOT need this projection — a
    /// placer takes an `Instances` value whole — so a `Part` between
    /// two patterns is a user saying WHICH copy to replicate, a
    /// different document from the nest without it.
    Part {
        /// The split or pattern whose value is read.
        of: RecipeNodeId,
        /// Which body of it.
        select: PartSelect,
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
    ///
    /// **A declared entity is SITED** (DM4): each side is a
    /// [`SitedRef`] — the entity's name, and the node it is READ AT,
    /// which is one of the consumer's operands (a member, for a
    /// union). A declaration therefore names only what exists BEFORE
    /// the consumer, and is authored in one pass: the `Declare` is
    /// inserted first and the boolean or union carrying its edge
    /// second. The site is also the SIDE — a name carried by both
    /// operands says which one it means — so nothing about a
    /// declaration depends on the consumer's own name space. A
    /// `Declare` left with no consumer is a legal document — it
    /// evaluates to its own payload and refuses nothing — so what
    /// says the node went inert is the delete that TOOK its last
    /// consumer, as a [`crate::edit::Maintenance::OrphanedDeclare`]
    /// on the accepted edit — the delete door's orphan report,
    /// beside DM7's strands, whose arm carries the transition rule.
    ///
    /// **A union's own fold rows are therefore UNREPRESENTABLE here,
    /// not refused** — a `Seam`, a `Merged`, a `Fragment` or the
    /// output body of the union is minted by the union's evaluation
    /// and has no node it is read at. A pair of bare [`StableName`]s,
    /// which is the only spelling that could have named one, does not
    /// typecheck:
    ///
    /// ```compile_fail,E0308
    /// let _: editor_core::Node<editor_core::ProfileProgram> =
    ///     editor_core::Node::declare_rest(vec![(named(), named())]);
    ///
    /// fn named() -> editor_core::StableName {
    ///     editor_core::StableName {
    ///         kind: editor_core::EntityKind::Face,
    ///         node: editor_core::RecipeNodeId(0),
    ///         path: Vec::new(),
    ///     }
    /// }
    /// ```
    ///
    /// **What that row proves, and what it does not.** Stable rustdoc
    /// checks only that the block FAILS to build; the `,E0308` beside
    /// it is not enforced, so a typo or a missing import would pass it
    /// just as well. The twin below is the same body with the ONE
    /// difference this claim is about — each side wrapped in a
    /// [`SitedRef`] — and it RUNS, so a defect anywhere but the side's
    /// type reddens here instead of satisfying the block above for the
    /// wrong reason. (The idiom is `quantity::units`'; the mate head
    /// above states it too.)
    ///
    /// ```
    /// let _: editor_core::Node<editor_core::ProfileProgram> =
    ///     editor_core::Node::declare_rest(vec![(sited(), sited())]);
    ///
    /// fn sited() -> editor_core::SitedRef {
    ///     editor_core::SitedRef::new(editor_core::RecipeNodeId(0), named())
    /// }
    ///
    /// fn named() -> editor_core::StableName {
    ///     editor_core::StableName {
    ///         kind: editor_core::EntityKind::Face,
    ///         node: editor_core::RecipeNodeId(0),
    ///         path: Vec::new(),
    ///     }
    /// }
    /// ```
    ///
    /// The persisted form is the other door the class could have come
    /// in through, and it refuses there instead of loading: the
    /// `Unreadable` detail is serde's own missing-field message, and
    /// the crate adds no constructor sentence to it — there is no
    /// analogue of the mate head's constructor refusal for a side
    /// whose SHAPE is wrong rather than whose kind is
    /// (`a_declared_pair_side_that_is_a_bare_name_does_not_load`).
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
        pairs: Vec<((SitedRef, SitedRef), ContactClass)>,
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
    /// **A leaf.** `a`/`b` are [`SitedFace`]s — each an
    /// instance-qualified FACE name plus the OPERAND node it is
    /// read at — and neither half is a consuming edge, so
    /// [`Node::inputs`] is empty and inserting a mate transfers no
    /// root. A12 adds *reading* edges on top: the walk from each
    /// operand down to its name's head yields the member the edge
    /// lands on, RECOMPUTED at need ([`crate::mate::reading_edges`])
    /// and never stored. A9's relative-freedom partition and A11's
    /// placement clusters read consuming ∪ reading edges; A10's
    /// invariants, maintenance and product gather read consuming
    /// edges only. Under consuming edges a mate is an isolated sink,
    /// so it is an ordinary NON-BODY root: listed like any other,
    /// denoting no body, ignored by the gather.
    ///
    /// **The operand is why a mate on placed geometry means what it
    /// says.** A transform mints no name (N1), so a reference read at
    /// the transform and one read at the instance carry the same
    /// name; the operand is the only thing that tells them apart, and
    /// the solve composes the map of every pose-bearing node between
    /// the operand and the minting instance
    /// ([`crate::mate::member_of`]). Two mates from one instance
    /// through two different transforms are two MEMBERS. So are two
    /// mates onto two copies of one pattern, at any depth of nesting:
    /// a member's identity is its instance, the chain of copies the
    /// walk consumed, and the operand it was read at.
    ///
    /// The insert door checks both halves against the live document —
    /// a never-existed operand or name node is a typo. A later delete
    /// may strand either, which is N5's ratified semantics: no edge
    /// until the mate is re-authored, and the solve refuses typed
    /// naming the head.
    ///
    /// **A mate's VALUE is the solve's answer for it** — its role when
    /// the solve placed it, a typed refusal when the solve faulted it
    /// — so that answer is one of the node's inputs and its content
    /// key feeds it beside this payload (`eval`'s `SolveAnswer` is the
    /// one home for why).
    Mate {
        /// The `a` reference: an entity of one instance's product,
        /// read at the operand the mate is authored against.
        a: SitedFace,
        /// The `b` reference: an entity of the other's.
        b: SitedFace,
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
    /// A [`SitedRef`] is a pair — the entity's [`StableName`], and
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
        refs: Vec<SitedRef>,
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

/// [`comp`] for a pair authored in a sketch frame's 2-D coordinates:
/// `Z` names no component, because a point in a plane has two.
fn comp2(v: &[Expr; 2], axis: Axis3) -> Option<&Expr> {
    v.get(axis.index())
}

/// [`comp2`]'s mutable twin.
fn comp2_mut(v: &mut [Expr; 2], axis: Axis3) -> Option<&mut Expr> {
    v.get_mut(axis.index())
}

/// A placement-rule node's slot lookup, shared by [`Node::Pattern`] and
/// [`Node::PlacedUnion`] — one rule vocabulary, one slot mapping, so
/// the two nodes can never drift apart on what a slot means.
///
/// `count` is the node's structural count slot when it has one.
/// `Explicit` answers `None` for EVERY slot including `Count`: its
/// placements are the count and carry no expressions, which is exactly
/// what [`Node::slots`] reports for it.
/// A placement-rule node's slot LIST, the domain of [`rule_expr`]
/// above the same two nodes — `has_count` says whether the node holds
/// a count expression at all, which only [`Node::PlacedUnion`] can
/// answer `false` to.
///
/// The two are one mapping read two ways, so a slot listed here is a
/// slot `rule_expr` answers for: `Explicit` carries listed placements
/// rather than a rule, so it has no count slot (the list's length IS
/// the count) and no expressions (the frames are structural data, D8);
/// and a parametric rule with no count is a node
/// [`Node::placement_rule_fault`] refuses, not a node with a count
/// slot nothing can read.
fn rule_slots(has_count: bool, kind: &PatternKind) -> Vec<SlotId> {
    let count = has_count.then_some(SlotId::Count);
    match kind {
        PatternKind::Linear { .. } => count
            .into_iter()
            .chain(Axis3::ALL.map(SlotId::Direction))
            .chain([SlotId::Spacing])
            .collect(),
        PatternKind::Circular { .. } => count.into_iter().chain([SlotId::Step]).collect(),
        PatternKind::Explicit(_) => Vec::new(),
    }
}

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
    ///
    /// The payload bound is the profile's: its plane is a node, and
    /// the reference lives in the payload, so answering this question
    /// means asking the payload for it.
    pub fn inputs(&self) -> Vec<RecipeNodeId>
    where
        P: crate::ProfilePayload,
    {
        match self {
            // **Two datums are not leaves.** An in-plane axis's
            // two coordinate pairs MEAN something only against the
            // frame they are written in, so the frame is an input, not
            // a note. Ahead of the leaf arm below, which is every
            // OTHER datum.
            Node::Datum(Datum::AxisInPlane { plane, .. }) => vec![*plane],
            // The derived frame reads its face out of `at`'s value, so
            // that body is an input for the same reason.
            Node::Datum(Datum::FaceFrame { at, .. }) => vec![*at],
            // A leaf whose material crosses the document seam has no
            // DAG edge to offer (A3).
            Node::Datum(_)
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
            // **A profile is not a leaf any more**: it is drawn ON a
            // frame node, and that is a DAG edge like any other. The
            // reference lives in the payload (where the plane always
            // did), so it is read through the payload trait — a
            // payload with no plane, which is every `Doc<P>` test
            // payload, still answers with no edge.
            Node::Profile(p) => p.plane_input().into_iter().collect(),
            Node::Assertion { measure, .. } => vec![*measure],
            Node::Extrude { profile, .. } => vec![*profile],
            Node::Revolve { profile, axis, .. } => vec![*profile, *axis],
            // A tube has no profile operand at all — its cross-section
            // is the door's own intent parameters — so the spine datum
            // is its only DAG edge.
            Node::Tube { spine, .. } | Node::HollowTube { spine, .. } => vec![*spine],
            Node::Loft { profiles, .. } => profiles.clone(),
            Node::Sweep { profile, path, .. } => vec![*profile, *path],
            Node::Fillet { target, .. } | Node::Chamfer { target, .. } => vec![*target],
            Node::Shell { target, .. } => vec![*target],
            Node::Split { target, tool } => vec![*target, *tool],
            Node::Boolean { a, b, declare, .. } => {
                let mut v = vec![*a, *b];
                v.extend(declare.iter().copied());
                v
            }
            // In LIST ORDER, not sorted: the order is the fold's (D9),
            // so it is what the DAG edge list has to report. The list
            // is pairwise distinct at the edit door, so no edge repeats.
            // The declaration input follows the members, the pair
            // boolean's precedent: this order is the memo's and the
            // content key's.
            Node::Union { members, declare } => {
                let mut v = members.clone();
                v.extend(declare.iter().copied());
                v
            }
            Node::Transform { input, .. } => vec![*input],
            Node::Part { of, .. } => vec![*of],
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

    /// **The node's LIST input**, where it has one — the whole of it,
    /// in order.
    ///
    /// A list input is an input the recipe spells as a sequence rather
    /// than as named slots, so the only edit that can change it is one
    /// that names the WHOLE new sequence
    /// ([`crate::DocEdit::SetMembers`]) — there is no position to
    /// address and no per-entry edit. Two nodes have one: a union's
    /// members and a loft's sections. Everything else answers `None`,
    /// which is what makes `SetMembers` at a boolean or a split a
    /// typed refusal rather than a silent no-op.
    ///
    /// The match is EXHAUSTIVE on purpose: a future node whose inputs
    /// are a list must be classified here or the compile breaks,
    /// rather than defaulting to "has no list" and being unreachable
    /// from the edit that exists for exactly it.
    pub fn list_input(&self) -> Option<&[RecipeNodeId]> {
        match self {
            Node::Union { members, .. } => Some(members),
            Node::Loft { profiles, .. } => Some(profiles),
            Node::Datum(_)
            | Node::Profile(_)
            | Node::Extrude { .. }
            | Node::Revolve { .. }
            | Node::Tube { .. }
            | Node::HollowTube { .. }
            | Node::Sweep { .. }
            | Node::Fillet { .. }
            | Node::Chamfer { .. }
            | Node::Shell { .. }
            | Node::Split { .. }
            | Node::Boolean { .. }
            | Node::Transform { .. }
            | Node::Pattern { .. }
            | Node::Part { .. }
            | Node::PlacedUnion { .. }
            | Node::Declare { .. }
            | Node::InstantiatePart { .. }
            | Node::Mate { .. }
            | Node::Measure { .. }
            | Node::Assertion { .. } => None,
        }
    }

    /// **The node's DECLARATION input**, where it has one — the edge a
    /// [`Node::Declare`] is wired to.
    ///
    /// Two node kinds carry one: the pair boolean and the n-ary union.
    /// Both mean the same thing by it (coincidence intent the verb
    /// verifies) and both are held to the same rule — the node it names
    /// must BE a `Declare` — so the rule is asked of this one answer at
    /// the edit door and at the load door rather than written per kind.
    ///
    /// The match is EXHAUSTIVE on purpose: a future node that consumes
    /// declarations is classified here or the compile breaks, rather
    /// than defaulting to "declares nothing" and slipping past both
    /// doors.
    pub fn declare_input(&self) -> Option<RecipeNodeId> {
        match self {
            Node::Boolean { declare, .. } | Node::Union { declare, .. } => *declare,
            Node::Datum(_)
            | Node::Profile(_)
            | Node::Extrude { .. }
            | Node::Revolve { .. }
            | Node::Tube { .. }
            | Node::HollowTube { .. }
            | Node::Loft { .. }
            | Node::Sweep { .. }
            | Node::Fillet { .. }
            | Node::Chamfer { .. }
            | Node::Shell { .. }
            | Node::Split { .. }
            | Node::Transform { .. }
            | Node::Pattern { .. }
            | Node::Part { .. }
            | Node::PlacedUnion { .. }
            | Node::Declare { .. }
            | Node::InstantiatePart { .. }
            | Node::Mate { .. }
            | Node::Measure { .. }
            | Node::Assertion { .. } => None,
        }
    }

    /// **The declaration edge's KIND rule, stated once**: the node a
    /// `declare` input names must be a [`Node::Declare`]. `Some(input)`
    /// is the offender; `None` is a node whose declare edge is fine or
    /// absent.
    ///
    /// Two doors ask it — [`crate::DocEdit::InsertNode`] and the load
    /// door (`persist::check`) — and each phrases the refusal in its
    /// own vocabulary ([`crate::EditError::DeclareInputNotDeclare`],
    /// `SnapshotError::DeclareInput`). The QUESTION is this one: a door
    /// that admits one node's broken declare edge and refuses
    /// another's is not a door, and two spellings of one predicate is
    /// how that happens.
    ///
    /// The input's LIVENESS is not asked here — a declare edge is a DAG
    /// edge, so each caller's `inputs()` walk has already refused a
    /// dangling one.
    pub(crate) fn bad_declare_input(&self, doc: &crate::doc::Doc<P>) -> Option<RecipeNodeId> {
        self.declare_input()
            .filter(|input| !matches!(doc.nodes.get(input), Some(Node::Declare { .. })))
    }

    /// **E10, stated once**: what is wrong with this assertion's bound
    /// against the node it constrains, if anything — the dimension the
    /// measure yields, or the absence of a measure at that reference.
    /// `None` for every node that is not a [`Node::Assertion`].
    ///
    /// The predicate takes the DOCUMENT because the measured dimension
    /// is another node's property; that is the shape
    /// [`Node::bad_declare_input`] has, for the same reason, and it is
    /// what lets both doors ask ONE question. The edit door renders the
    /// answer as [`crate::EditError::AssertionTarget`] /
    /// [`crate::EditError::AssertionDimension`] and the load door as
    /// `SnapshotError::AssertionTarget` / `SnapshotError::AssertionBound`:
    /// a refusal names the door it came from, and the rule is asked in
    /// one place so the two cannot drift.
    ///
    /// The target's LIVENESS is not asked separately: a reference that
    /// names no live node is not a measure, and reports as such.
    pub(crate) fn assertion_bound_fault(
        &self,
        doc: &crate::doc::Doc<P>,
    ) -> Option<AssertionBoundFault> {
        let Node::Assertion { measure, bound, .. } = self else {
            return None;
        };
        let (measure, bound) = (*measure, bound.dim());
        match doc.nodes.get(&measure) {
            Some(Node::Measure { expr, .. }) => {
                AssertionBoundFault::against(measure, expr.dim(), bound)
            }
            _ => Some(AssertionBoundFault::TargetNotMeasure { measure, bound }),
        }
    }

    /// Whether this node is a mate whose alignment datum carries a
    /// coordinate no predicate can decide on (ASM-R2a D-1).
    ///
    /// [`crate::mate::Alignment::is_finite`] is the rule; this is the
    /// one place a NODE is asked it, so the edit door and the load
    /// door's walk share the destructuring as well as the test.
    /// `false` for every node that is not a [`Node::Mate`].
    pub(crate) fn has_non_finite_alignment(&self) -> bool {
        matches!(self, Node::Mate { alignment, .. } if !alignment.is_finite())
    }

    /// **DM5, stated once**: what is wrong with this node's structural
    /// content, if anything — one node reached twice, a list left under
    /// two, or a name designation outside the canonical form its
    /// construction door establishes.
    ///
    /// Structural rules over the node's own content rather than a rule
    /// per node kind, and ONE definition with three callers:
    /// `InsertNode`,
    /// [`crate::DocEdit::SetMembers`] on the rewritten node, and the
    /// load door's `validate_document`. The two edit doors render it in
    /// [`crate::EditError`]'s vocabulary and the load door in
    /// `SnapshotError`'s, because a refusal names the door it came
    /// from — but the question is asked in exactly one place, which is
    /// what stops the three from drifting.
    ///
    /// The order is deliberate. The list's floor answers first, so a
    /// one-entry list is reported as short rather than as whatever its
    /// single entry happens to collide with; liveness is NOT asked
    /// here at all, because it needs the document and the callers
    /// check it before they call.
    ///
    /// # What the rule covers, and why that is sound
    ///
    /// The two INPUT clauses read [`Node::inputs`], so they apply to
    /// EVERY node kind — not only the union, the list-input kinds and
    /// the boolean. That is wider than DM5's text, and deliberately:
    ///
    /// - The duplicate clause is sound everywhere because no node kind
    ///   in this crate has a meaning for the same input twice. A
    ///   boolean with `a == b` is a self-operation whose result is one
    ///   of its own operands; a `Split` cutting a body by itself is the
    ///   same; a `Mate` between a part and itself has no relative
    ///   frame. The one kind that could plausibly want a repeat is
    ///   [`Node::Measure`], and it does not: its edges come from the
    ///   measurement's own node set, which DEDUPS before `inputs`
    ///   returns, so a measurement over one body twice presents one
    ///   edge here and is untouched by this rule.
    /// - The floor clause only ever fires where [`Node::list_input`]
    ///   answers `Some`, which is [`Node::Union`] and [`Node::Loft`].
    ///   For the loft this is NEW — a one-section loft was accepted
    ///   before this unit and is refused now, at the insert door and at
    ///   the load door alike. A single section has nothing to loft
    ///   between and the sweep refused it downstream anyway; the change
    ///   is that it is refused where it is authored, naming the list,
    ///   instead of at evaluation naming the sweep.
    ///   (`a_one_section_loft_is_refused_at_the_insert_door` and its
    ///   load-door twin pin both.)
    /// - The two DESIGNATION clauses read one payload each — a shell's
    ///   `open`, a blend's `selection` — and are silent about every
    ///   other node kind, because a canonical form is the payload's own
    ///   and there is nothing to generalize. What is general is that
    ///   each is asked HERE, so the form a construction door
    ///   establishes is the form every door admits.
    pub fn input_fault(&self) -> Option<InputFault>
    where
        P: crate::ProfilePayload,
    {
        if let Some(list) = self.list_input()
            && list.len() < 2
        {
            return Some(InputFault::TooFew { found: list.len() });
        }
        let mut seen: std::collections::BTreeSet<RecipeNodeId> = std::collections::BTreeSet::new();
        if let Some(input) = self.inputs().into_iter().find(|input| !seen.insert(*input)) {
            return Some(InputFault::Duplicate { input });
        }
        // The name designations, each against the canonical form its
        // own construction door establishes. Asked here, once, so every
        // door that admits a node refuses the same shapes, and neither
        // form is repaired at any of them. `SetMembers` is a caller but
        // reaches neither clause: it refuses `SetMembersOnNonList`
        // first, since no designation-carrying kind has a list input.
        // The doors a designation fault is REACHABLE at are the insert
        // door and the load door.
        //
        // The ORDERED payload — a shell's `open` — carries only the
        // rule the sorted payloads state by their order: no entry
        // twice.
        if let Node::Shell { open, .. } = self {
            for (again, name) in open.iter().enumerate() {
                if let Some(first) = open[..again].iter().position(|n| n == name) {
                    return Some(InputFault::RepeatedDesignation { first, again });
                }
            }
        }
        // The SORTED payload — a blend's selection — states both rules
        // in one: strictly increasing IS "sorted and deduplicated", so
        // a swap and a repeat are one fault at one position. An empty
        // selection holds it vacuously and is evaluation's refusal to
        // name (`BlendSelectionEmpty`), not this door's.
        if let Node::Fillet { selection, .. } | Node::Chamfer { selection, .. } = self
            && let Some(at) = selection.windows(2).position(|w| w[0] >= w[1])
        {
            return Some(InputFault::SelectionNotCanonical { at });
        }
        None
    }

    /// Writes a whole new list into [`Node::list_input`]'s slot,
    /// answering whether this node has one. The edit door validates
    /// against the REWRITTEN node, so the write happens first and the
    /// checks run on the result — which is what makes `SetMembers`
    /// share `InsertNode`'s checks rather than mirror them.
    pub(crate) fn set_list_input(&mut self, list: Vec<RecipeNodeId>) -> bool {
        match self {
            Node::Union { members, .. } => {
                *members = list;
                true
            }
            Node::Loft { profiles, .. } => {
                *profiles = list;
                true
            }
            // Exhaustive, and it names the same variants
            // [`Node::list_input`] answers `None` for: the read and
            // the write are one answer read two ways, and a variant
            // one of them treats as list-free while the other writes
            // it is a list nothing can read back.
            Node::Datum(_)
            | Node::Profile(_)
            | Node::Extrude { .. }
            | Node::Revolve { .. }
            | Node::Tube { .. }
            | Node::HollowTube { .. }
            | Node::Sweep { .. }
            | Node::Fillet { .. }
            | Node::Chamfer { .. }
            | Node::Shell { .. }
            | Node::Split { .. }
            | Node::Boolean { .. }
            | Node::Transform { .. }
            | Node::Pattern { .. }
            | Node::Part { .. }
            | Node::PlacedUnion { .. }
            | Node::Declare { .. }
            | Node::InstantiatePart { .. }
            | Node::Mate { .. }
            | Node::Measure { .. }
            | Node::Assertion { .. } => false,
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
            // X and Y only: the frame supplies the third coordinate,
            // and a slot for it would be a number nobody may set.
            Node::Datum(Datum::AxisInPlane { .. }) => vec![
                SlotId::Origin(Axis3::X),
                SlotId::Origin(Axis3::Y),
                SlotId::Direction(Axis3::X),
                SlotId::Direction(Axis3::Y),
            ],
            Node::Datum(Datum::Frame { .. }) => {
                let mut s = vec3(SlotId::Origin).to_vec();
                s.extend(vec3(SlotId::U));
                s.extend(vec3(SlotId::V));
                s
            }
            // Origin and normal come off the face; the spin is the
            // one number an author chooses.
            Node::Datum(Datum::FaceFrame { .. }) => vec![SlotId::Spin],
            Node::Profile(p) => p.slots(),
            // AQ4: an instance takes no arguments in v1 — the
            // referenced document evaluates at its OWN parameters.
            Node::Split { .. }
            | Node::Boolean { .. }
            | Node::Union { .. }
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
            Node::Shell { .. } => vec![SlotId::ShellThickness],
            Node::Revolve { .. } => vec![SlotId::RevolveAngle],
            // The two kinds enumerate the SAME shared head — the
            // reference direction, then the two radii, then whatever
            // the window carries — and the hollow kind appends its
            // wall. Written as one arm plus one push, so the shared
            // half cannot drift between them.
            Node::Tube { window, .. } | Node::HollowTube { window, .. } => {
                let mut s = vec3(SlotId::Direction).to_vec();
                s.push(SlotId::TubeMajorRadius);
                s.push(SlotId::TubeMinorRadius);
                s.extend(window.slots());
                if matches!(self, Node::HollowTube { .. }) {
                    s.push(SlotId::TubeWall);
                }
                s
            }
            Node::Loft { .. } => vec![SlotId::VDegree],
            Node::Sweep { .. } => vec![SlotId::Stations, SlotId::VDegree],
            Node::Transform { .. } => {
                let mut s = vec3(SlotId::Translation).to_vec();
                s.extend(vec3(SlotId::RotationAxis));
                s.push(SlotId::RotationAngle);
                s
            }
            // A pattern's count is a field, so it is always there; a
            // placed union's is an `Option`, and a rule missing the
            // count it needs carries no count SLOT either — the
            // mismatch is `PlacementRuleFault::CountSpelling`, refused
            // at both doors, and not a slot address that answers
            // nothing.
            Node::Pattern { kind, .. } => rule_slots(true, kind),
            Node::PlacedUnion { count, kind, .. } => rule_slots(count.is_some(), kind),
            // A half is recipe payload, not a number anyone sets; an
            // index is the one structural slot the projection carries.
            Node::Part { select, .. } => match select {
                PartSelect::SplitHalf(_) => Vec::new(),
                PartSelect::Instance(_) => vec![SlotId::Instance],
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
            | (Node::Datum(Datum::Frame { origin, .. }), S::Origin(ax))
            | (Node::Datum(Datum::Point { position: origin }), S::Origin(ax)) => {
                Some(comp(origin, ax))
            }
            (Node::Datum(Datum::Plane { normal, .. }), S::Normal(ax)) => Some(comp(normal, ax)),
            (Node::Datum(Datum::Axis { direction, .. }), S::Direction(ax)) => {
                Some(comp(direction, ax))
            }
            (Node::Datum(Datum::Frame { u, .. }), S::U(ax)) => Some(comp(u, ax)),
            (Node::Datum(Datum::Frame { v, .. }), S::V(ax)) => Some(comp(v, ax)),
            (Node::Datum(Datum::FaceFrame { spin, .. }), S::Spin) => Some(spin),
            // `comp2` answers None for `Z`, which is the honest
            // "this node does not carry that slot" this match is open
            // on — not a panic and not a silent zero.
            (Node::Datum(Datum::AxisInPlane { origin, .. }), S::Origin(ax)) => comp2(origin, ax),
            (Node::Datum(Datum::AxisInPlane { direction, .. }), S::Direction(ax)) => {
                comp2(direction, ax)
            }
            (Node::Extrude { distance, .. }, S::Distance) => Some(distance),
            (Node::Fillet { radius, .. }, S::Radius) => Some(radius),
            (Node::Chamfer { distance, .. }, S::ChamferDistance) => Some(distance),
            (Node::Shell { thickness, .. }, S::ShellThickness) => Some(thickness),
            (Node::Revolve { angle, .. }, S::RevolveAngle) => Some(angle),
            (Node::Tube { u_ref, .. } | Node::HollowTube { u_ref, .. }, S::Direction(ax)) => {
                Some(comp(u_ref, ax))
            }
            (
                Node::Tube { major_radius, .. } | Node::HollowTube { major_radius, .. },
                S::TubeMajorRadius,
            ) => Some(major_radius),
            (
                Node::Tube { minor_radius, .. } | Node::HollowTube { minor_radius, .. },
                S::TubeMinorRadius,
            ) => Some(minor_radius),
            (Node::HollowTube { wall, .. }, S::TubeWall) => Some(wall),
            // The window answers for its own two slots, so "which
            // angle is which" has one home ([`TubeWindow::expr`]).
            (Node::Tube { window, .. } | Node::HollowTube { window, .. }, s) => window.expr(s),
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
            (
                Node::Part {
                    select: PartSelect::Instance(index),
                    ..
                },
                S::Instance,
            ) => Some(index),
            // EXHAUSTIVE on the NODE axis, open on the slot axis: a new
            // node kind must be classified here or the compile breaks,
            // while "this node does not carry that slot" stays the
            // honest answer for a slot the listed arms did not claim.
            // `Pattern`, `PlacedUnion` and the two tube kinds are
            // absent because their arms above already bind every slot.
            (
                Node::Datum(..)
                | Node::Profile(..)
                | Node::Extrude { .. }
                | Node::Revolve { .. }
                | Node::Loft { .. }
                | Node::Sweep { .. }
                | Node::Fillet { .. }
                | Node::Chamfer { .. }
                | Node::Shell { .. }
                | Node::Split { .. }
                | Node::Boolean { .. }
                | Node::Union { .. }
                | Node::Transform { .. }
                | Node::Part { .. }
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
            | (Node::Datum(Datum::Frame { origin, .. }), S::Origin(ax))
            | (Node::Datum(Datum::Point { position: origin }), S::Origin(ax)) => {
                Some(comp_mut(origin, ax))
            }
            (Node::Datum(Datum::Plane { normal, .. }), S::Normal(ax)) => Some(comp_mut(normal, ax)),
            (Node::Datum(Datum::Axis { direction, .. }), S::Direction(ax)) => {
                Some(comp_mut(direction, ax))
            }
            (Node::Datum(Datum::Frame { u, .. }), S::U(ax)) => Some(comp_mut(u, ax)),
            (Node::Datum(Datum::Frame { v, .. }), S::V(ax)) => Some(comp_mut(v, ax)),
            (Node::Datum(Datum::FaceFrame { spin, .. }), S::Spin) => Some(spin),
            (Node::Datum(Datum::AxisInPlane { origin, .. }), S::Origin(ax)) => {
                comp2_mut(origin, ax)
            }
            (Node::Datum(Datum::AxisInPlane { direction, .. }), S::Direction(ax)) => {
                comp2_mut(direction, ax)
            }
            (Node::Extrude { distance, .. }, S::Distance) => Some(distance),
            (Node::Fillet { radius, .. }, S::Radius) => Some(radius),
            (Node::Chamfer { distance, .. }, S::ChamferDistance) => Some(distance),
            (Node::Shell { thickness, .. }, S::ShellThickness) => Some(thickness),
            (Node::Revolve { angle, .. }, S::RevolveAngle) => Some(angle),
            (Node::Tube { u_ref, .. } | Node::HollowTube { u_ref, .. }, S::Direction(ax)) => {
                Some(comp_mut(u_ref, ax))
            }
            (
                Node::Tube { major_radius, .. } | Node::HollowTube { major_radius, .. },
                S::TubeMajorRadius,
            ) => Some(major_radius),
            (
                Node::Tube { minor_radius, .. } | Node::HollowTube { minor_radius, .. },
                S::TubeMinorRadius,
            ) => Some(minor_radius),
            (Node::HollowTube { wall, .. }, S::TubeWall) => Some(wall),
            (Node::Tube { window, .. } | Node::HollowTube { window, .. }, s) => window.expr_mut(s),
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
            (
                Node::Part {
                    select: PartSelect::Instance(index),
                    ..
                },
                S::Instance,
            ) => Some(index),
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
                | Node::Shell { .. }
                | Node::Split { .. }
                | Node::Boolean { .. }
                | Node::Union { .. }
                | Node::Transform { .. }
                | Node::Part { .. }
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
    /// blend's selection, a shell's open list, a derived frame's face, a
    /// measure's references, a mate's two heads, an instance's interface
    /// crossings' `outer`s. Document data, never DAG
    /// edges ([`Node::inputs`] excludes them): the edit door checks at
    /// insertion that each one names a live node, and a later delete may
    /// strand it, which is NAMING-DESIGN N5's dangling-reference
    /// semantics with `Rebind` as the one repair.
    ///
    /// The single answer to "which payloads carry a name": every reader
    /// reads this rather than its own copy of the list. The negative
    /// half is [`name_free_node`], shared with the rewriting twin.
    ///
    /// **The list has TWO prose homes and no others**: this doc, beside
    /// the match that enforces it, and `REFERENCES.md` §0's `Carriers:`
    /// clause, which a reader without the code reads. A new carrier is
    /// therefore two edits. Every other site says what it DOES with the
    /// list and points here for what is in it, so a site that spells
    /// variant names is a third home to delete rather than maintain.
    ///
    /// The question is asked IN THIS DOCUMENT'S NAME SPACE, which is
    /// the space every reader of the answer reasons in — the insert
    /// door's liveness check, `Rebind`, DM7's strand walk through
    /// `Doc::name_carriers` (which `split`'s
    /// `PartNameReachesRemainder` precondition reads too), the insert
    /// census in `crate::resolve`. A reference a payload holds in
    /// ANOTHER document's id space is therefore not a name here: an
    /// instance's crossing `inner` is the one such reference, and the
    /// arm below is the ONE home for the reason it is out of scope
    /// rather than absent — every other site says "not a name of this
    /// document" and points here.
    pub fn payload_names(&self) -> Vec<&StableName> {
        match self {
            // A declared pair's two NAMES. The sites beside them
            // are node ids, not names, and are listed by
            // [`Node::payload_read_sites`].
            Node::Declare { pairs } => pairs
                .iter()
                .flat_map(|((a, b), _)| [&a.name, &b.name])
                .collect(),
            Node::Fillet { selection, .. } | Node::Chamfer { selection, .. } => {
                selection.iter().collect()
            }
            // Designation order, which is meaning here (the first
            // named face carries the rim), not a click sequence.
            Node::Shell { open, .. } => open.iter().collect(),
            // The derived frame's face is a frozen name like a blend's
            // selection: the insert door checks its node is live, and
            // `Rebind` is its repair.
            Node::Datum(Datum::FaceFrame { face, .. }) => vec![face],
            // A12: a mate's two heads are the instance-qualified
            // names its reading edges are recomputed from. The
            // operands they are read at are node ids, not names, and
            // are listed by [`Node::payload_read_sites`].
            Node::Mate { a, b, .. } => vec![a.name.as_ref(), b.name.as_ref()],
            // A measure's references are argument-ORDERED, so they are
            // listed in that order rather than a canonical one.
            Node::Measure { refs, .. } => refs.iter().map(|r| &r.name).collect(),
            // An instance's interface record: each crossing's `outer`,
            // in record order.
            //
            // An `outer` is a REMAINDER name — it denotes a face in
            // THIS document, on a node this document can delete and
            // under a name this document can rebind — so it is a
            // payload name like a mate's head, and the same FOUR
            // doors reach it: the insert door's liveness check,
            // `DocEdit::Rebind`, DM7's strand report, and `split`'s
            // `PartNameReachesRemainder` precondition, which refuses
            // a cut that TAKES an instance whose record names a kept
            // node — a part cannot name the remainder.
            //
            // An `inner` is NOT listed, and the reason is the id space
            // it is spelled in: the PART's. Its `node` is a part-side
            // id, which this document may not hold or may hold as an
            // unrelated node, so the insert door's liveness check over
            // it would be a wrong check and `Rebind` a wrong repair.
            // Its life is the pinned product's, re-verified at every
            // evaluation ([`crate::eval::NodeErrorKind::CrossingUnverified`],
            // ASSEMBLY A4). That is what makes this list's "single
            // answer" claim TRUE BY SCOPE: it lists the names in THIS
            // document's name space, which is the space every reader
            // of it reasons in.
            Node::InstantiatePart { interface, .. } => interface
                .crossings
                .iter()
                .map(|crossing| {
                    let InterfaceCrossing::Mate { outer, .. } = crossing;
                    outer.as_ref()
                })
                .collect(),
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
    /// [`Node::rewrite_payload_names`] under the one-pair map; that
    /// walk is the one home for which payloads are rewritten and how
    /// each re-canonicalizes.
    pub(crate) fn rebind_payload_names(&mut self, from: &StableName, to: &StableName) -> usize {
        self.rewrite_payload_names(&mut |name| (name == from).then(|| to.clone()))
    }

    /// **Rewrites every payload name through `map`, all at once**,
    /// returning how many it rewrote: `map` answers the name a site
    /// now holds, or `None` to leave it. The substrate of both name
    /// rewrites the edit vocabulary has — [`crate::DocEdit::Rebind`]'s
    /// one pair ([`Node::rebind_payload_names`]) and the whole-program
    /// edit's per-segment map, which moves several names of one
    /// payload in one pass.
    ///
    /// ONE pass rather than one pair at a time, because a pair at a
    /// time is wrong for a permutation: rewriting `1 → 2` and then
    /// `2 → 3` over a selection holding both collapses the first onto
    /// the second at the re-canonicalization between them and then
    /// moves the merged name, so a name is lost. Every site is mapped
    /// from what it held BEFORE the pass, and the payload
    /// re-canonicalizes once, after.
    ///
    /// The variants named here and in [`Node::payload_names`] are the
    /// same variants; [`name_free_node`] is where that agreement is
    /// held.
    pub(crate) fn rewrite_payload_names(
        &mut self,
        map: &mut dyn FnMut(&StableName) -> Option<StableName>,
    ) -> usize {
        fn rewrite(
            name: &mut StableName,
            map: &mut dyn FnMut(&StableName) -> Option<StableName>,
        ) -> usize {
            match map(name) {
                Some(next) => {
                    *name = next;
                    1
                }
                None => 0,
            }
        }
        /// A FACE-typed payload name rewritten through `map`, or
        /// `None` where the map leaves it — the one re-derivation
        /// both face-name payloads use (a mate's heads, an instance's
        /// crossing `outer`s).
        ///
        /// A face name's kind is the TYPE's, not this rewrite's:
        /// [`crate::DocEdit::Rebind`] refuses a cross-kind pair at
        /// its own door and the segment map moves a name's locators
        /// only, so what the mapped name contributes is its
        /// DERIVATION, and `FaceName::map_derivation` is the one
        /// in-crate door for that. It cannot change a kind, so there
        /// is no arm to assert away and `Infallible` is the whole of
        /// what can go wrong.
        fn rewrite_face(
            name: &FaceName,
            map: &mut dyn FnMut(&StableName) -> Option<StableName>,
        ) -> Option<FaceName> {
            let to = map(name.as_ref())?;
            let Ok(next) = name.map_derivation(|_, _| {
                Ok::<_, core::convert::Infallible>((to.node, to.path.clone()))
            });
            Some(next)
        }
        let mut hits = 0usize;
        match self {
            // The NAME rewrites; the SITE stays. A site is the node
            // the author chose to read the entity at — an operand of
            // the consumer — and moving it would re-author which
            // member the declaration is about, which is not a repair
            // for a name whose minting node went away.
            Node::Declare { pairs } => {
                for r in pairs.iter_mut().flat_map(|((a, b), _)| [a, b]) {
                    hits += rewrite(&mut r.name, map);
                }
            }
            // A SORTED payload re-canonicalizes through the same door
            // that established the form: the repair re-establishes it,
            // so what a rebind writes is what `Node::input_fault`
            // accepts and there is no shape a repair can leave behind
            // that an edit door would refuse.
            Node::Fillet { selection, .. } | Node::Chamfer { selection, .. } => {
                for name in selection.iter_mut() {
                    hits += rewrite(name, map);
                }
                if hits > 0 {
                    canonicalize_selection(selection);
                }
            }
            // An ORDERED payload re-canonicalizes to its own form: the
            // order stays, and a rebind onto a face already designated
            // keeps the EARLIER occurrence — the one whose position
            // decides which face carries the rim — and drops the later,
            // so the list shrinks by one rather than naming one face
            // twice (which the load door refuses as corrupt).
            Node::Shell { open, .. } => {
                for name in open.iter_mut() {
                    hits += rewrite(name, map);
                }
                if hits > 0 {
                    dedup_keeping_first(open);
                }
            }
            // A mate's two references: the NAME rewrites like any
            // other, and a reference read AT ITS OWN MINT stays read
            // at its own mint — the operand follows the name it was
            // authored to coincide with. A reference read somewhere
            // ELSE keeps its operand: that node is an authored fact
            // this edit knows nothing about, and re-targeting it is
            // re-authoring the mate.
            Node::Mate { a, b, .. } => {
                for r in [a, b] {
                    let Some(next) = rewrite_face(&r.name, map) else {
                        continue;
                    };
                    let at_mint = r.at == r.name.node;
                    r.name = next;
                    if at_mint {
                        r.at = r.name.node;
                    }
                    hits += 1;
                }
            }
            // One name, no set to re-canonicalize.
            Node::Datum(Datum::FaceFrame { face, .. }) => {
                hits += rewrite(face, map);
            }
            // No re-canonicalization: the order IS argument order, and
            // a rebind onto an already-referenced entity must leave two
            // arguments naming one entity rather than shrink the list
            // and renumber every index the expression holds.
            Node::Measure { refs, .. } => {
                for r in refs.iter_mut() {
                    hits += rewrite(&mut r.name, map);
                }
            }
            // An instance's crossing `outer`s — the reading twin's
            // list, rewritten. No re-canonicalization: a record is
            // ordered by the split's collection order and each
            // crossing is keyed by its own mate, so a rebind can make
            // two `outer`s equal but never two CROSSINGS equal, and
            // there is no set to collapse. An `inner` is not a name in
            // this document (the reading twin says why) and is not
            // rewritten.
            Node::InstantiatePart { interface, .. } => {
                for crossing in interface.crossings.iter_mut() {
                    let InterfaceCrossing::Mate { outer, .. } = crossing;
                    let Some(next) = rewrite_face(outer, map) else {
                        continue;
                    };
                    *outer = next;
                    hits += 1;
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

    /// **The nodes a payload's references are READ AT that are not
    /// also DAG inputs** — a mate's two operands and a declared
    /// pair's two sites.
    ///
    /// The insert door checks these are live exactly as it checks a
    /// payload name's head, and for the same reason: a never-existed
    /// id is a typo, and a later delete stranding one is N5's
    /// dangling case, refused at the solve rather than at the edit.
    ///
    /// A measure's `at` is absent here because it is an ordinary
    /// input ([`Node::inputs`] reports it), and the input check
    /// already covers it. A mate's is not: an operand is an A12
    /// READING edge, and making it consuming would take the mated
    /// bodies out of A10's root set.
    pub fn payload_read_sites(&self) -> Vec<RecipeNodeId> {
        match self {
            Node::Mate { a, b, .. } => vec![a.at, b.at],
            // A declared pair's sites are the consumer's operands, so
            // they are reading edges exactly as a mate's are: this
            // node has no `inputs`, and a site that is not the
            // consumer's operand is the EVALUATION's refusal, not the
            // insert door's.
            Node::Declare { pairs } => pairs.iter().flat_map(|((a, b), _)| [a.at, b.at]).collect(),
            // EXHAUSTIVE, with no wildcard, so a new [`Node`] variant
            // is classified here or does not compile — the promise
            // the twins above already keep. Three groups: the
            // name-free variants, which reference nothing that could
            // have a read site (one home for that list,
            // [`name_free_node`]); the named variants whose
            // references are read at a node the DAG ALREADY CARRIES
            // — a blend's and a shell's at the body they consume, a
            // derived frame's and a measure's at an `at` that
            // [`Node::inputs`] reports — so the input check covers
            // the site and there is nothing extra to name here; and
            // an instance, whose interface record holds NO node id at
            // all ([`InterfaceCrossing::Mate`] argues why). A crossing
            // is a class and two face names: the `outer` is a payload
            // name, checked as one by the reading twin's list, and the
            // `inner` is spelled in the part's id space, which no door
            // here may read.
            name_free_node!()
            | Node::Fillet { .. }
            | Node::Chamfer { .. }
            | Node::Shell { .. }
            | Node::Datum(Datum::FaceFrame { .. })
            | Node::InstantiatePart { .. }
            | Node::Measure { .. } => Vec::new(),
        }
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
            | Node::Tube { .. }
            | Node::HollowTube { .. }
            | Node::Loft { .. }
            | Node::Sweep { .. }
            | Node::Fillet { .. }
            | Node::Chamfer { .. }
            | Node::Shell { .. }
            | Node::Split { .. }
            | Node::Boolean { .. }
            | Node::Union { .. }
            | Node::Transform { .. }
            | Node::Part { .. }
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
        // `SetPlacement` holds a cluster frame to, because it is held
        // to it by the same predicate — `Frame::admission_fault`, whose
        // home is the frame. This arm says only WHICH frame in the list
        // answered. Checked HERE so the refusal lands at the edit door
        // with the best diagnostics, not at the kernel's rigidity
        // re-check downstream.
        frames
            .iter()
            .enumerate()
            .find_map(|(index, frame)| match frame.admission_fault()? {
                crate::placement::FrameFault::NonFinite => {
                    Some(PlacementRuleFault::NonFiniteFrame { index })
                }
                crate::placement::FrameFault::Improper { determinant } => {
                    Some(PlacementRuleFault::ImproperFrame { index, determinant })
                }
            })
    }

    /// A `Declare` node whose every pair asserts the CONFORMAL class
    /// — the class the class-less payload always meant.
    ///
    /// This NAMES `Rest` at the call site; it does not default it.
    /// The difference matters: a reader of the call sees which of C4's
    /// classes is being claimed, and a pair that means something else
    /// cannot arrive here by omission. Mixed-class nodes build
    /// [`Node::Declare`] directly.
    pub fn declare_rest(pairs: Vec<(SitedRef, SitedRef)>) -> Self {
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
        refs: Vec<SitedRef>,
    ) -> Result<Self, MeasureNodeFault> {
        let node = Node::Measure { expr, refs };
        match node.measure_fault() {
            Some(fault) => Err(fault),
            None => Ok(node),
        }
    }

    /// **The slot-dimension rule, asked of this node** (spec D6):
    /// every slot [`Node::slots`] names answers an expression, and
    /// that expression carries the dimension [`SlotId::dimension`]
    /// fixes for the address. `None` when the node carries no slot at
    /// all, which is most of the assembly vocabulary.
    ///
    /// One home for the question, read by the edit doors
    /// (`check_node_slots`) and by the load door's walk, each naming
    /// the answer in its own vocabulary. The `pub` payloads are what
    /// make a violation reachable: a hand-built node and a corrupt
    /// file can both state one, and neither may reach a document the
    /// edit doors could not have produced.
    pub(crate) fn slot_dimension_fault(&self) -> Option<SlotDimensionFault>
    where
        P: crate::ProfilePayload,
    {
        self.slots().into_iter().find_map(|slot| {
            // `slots()` IS `expr()`'s domain — the two matches answer
            // for the same payload — so a slot with no expression is a
            // bug in this module, not a document a door may refuse.
            // Pinned for every node kind by
            // `switch_slots::every_node_kinds_slots_are_all_readable`.
            let Some(expr) = self.expr(slot) else {
                unreachable!(
                    "slot {}: `Node::slots` names it and `Node::expr` does not answer for it — \
                     the two matches in this module disagree",
                    slot.label()
                )
            };
            slot.dimension_fault(expr)
        })
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
        for prim in prims {
            for index in prim.refs() {
                if !usize::try_from(index).is_ok_and(|i| i < refs.len()) {
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
    /// do not depend on the order a user clicked in. The form comes
    /// from [`canonicalize_selection`], which every site that
    /// establishes it shares.
    pub fn fillet(target: RecipeNodeId, radius: Expr, selection: Vec<StableName>) -> Self {
        let mut selection = selection;
        canonicalize_selection(&mut selection);
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
        canonicalize_selection(&mut selection);
        Node::Chamfer {
            target,
            distance,
            selection,
        }
    }

    /// Builds a [`Node::Shell`] with `open` in DESIGNATION ORDER,
    /// deduplicated keeping each name's first occurrence — the one
    /// construction door, and deliberately not [`Node::fillet`]'s
    /// sort: the first designated face of a chart is the one that
    /// carries the rim's identity (the variant docs), so the order is
    /// authored data the kernel reads, and sorting it would silently
    /// move a rim from one face to another.
    pub fn shell(target: RecipeNodeId, thickness: Expr, open: Vec<StableName>) -> Self {
        let mut open = open;
        dedup_keeping_first(&mut open);
        Node::Shell {
            target,
            thickness,
            open,
        }
    }
}

/// Sorts a name designation and drops its repeats — the canonical form
/// of a SORTED designation ([`InputFault::SelectionNotCanonical`]),
/// shared by the two construction doors and the rebind rewrite so the
/// three cannot disagree about it, exactly as
/// [`dedup_keeping_first`] is shared for the ordered twin.
///
/// It is the establisher of the form [`Node::input_fault`] checks:
/// what comes back from here always answers `None` there, which is
/// what makes `Rebind` a repair rather than a second authoring of a
/// shape the doors would refuse.
fn canonicalize_selection(names: &mut Vec<StableName>) {
    names.sort();
    names.dedup();
}

/// Drops every repeat of a name, keeping the FIRST occurrence and the
/// order of what remains — the canonical form of an ordered
/// designation, shared by the construction door and the rebind
/// rewrite so the two cannot disagree about it.
fn dedup_keeping_first(names: &mut Vec<StableName>) {
    let mut seen: Vec<StableName> = Vec::with_capacity(names.len());
    names.retain(|n| {
        if seen.contains(n) {
            false
        } else {
            seen.push(n.clone());
            true
        }
    });
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
