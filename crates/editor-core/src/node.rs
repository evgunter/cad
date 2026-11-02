//! The recipe-node vocabulary AS DATA (ratified F4; spec D3). No node
//! here evaluates anything — the evaluation service interprets this
//! data against the kernel ops.

use std::collections::BTreeMap;

use crate::expr::{Dimension, Slot};
use crate::mate::BundleRows as _;
use crate::operand::Bodies;
use crate::var::VarId;
// The contact vocabulary is the KERNEL's (CONTACT-DESIGN C4, M9-1
// PR-1). Imported, never redefined: the boolean's own refusals must
// carry the same words this node authors, and `crate::names::flush`
// owns the single upward re-export.

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
        $crate::node::Node::Profile(_)
            | $crate::node::Node::Extrude { .. }
            | $crate::node::Node::Revolve { .. }
            | $crate::node::Node::Tube { .. }
            | $crate::node::Node::HollowTube { .. }
            | $crate::node::Node::Loft { .. }
            | $crate::node::Node::Sweep { .. }
            | $crate::node::Node::Fillet { .. }
            | $crate::node::Node::Chamfer { .. }
            | $crate::node::Node::Shell { .. }
            | $crate::node::Node::Split { .. }
            | $crate::node::Node::Mirror { .. }
            | $crate::node::Node::Subtract { .. }
            | $crate::node::Node::Union { .. }
            | $crate::node::Node::Intersect { .. }
            | $crate::node::Node::Assemble { .. }
            | $crate::node::Node::Place { .. }
            | $crate::node::Node::World
            | $crate::node::Node::Space
            | $crate::node::Node::Measure { .. }
            | $crate::node::Node::Assertion { .. }
    };
}

/// A stable recipe-node identity (spec D3, NAMING-DESIGN N1's
/// substrate): minted from the document's mint chain ([`crate::Mint`])
/// at insertion, never reused (deletion does not free it). Of two
/// nodes one document holds, the one inserted first has the lesser id
/// ([`MintId`]). Its stability is a contract, pinned by test.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
pub struct RecipeNodeId(pub MintId);

/// **A profile program step's identity** (`names/README.md`, "N1, the
/// profile pieces"): minted from the document's mint chain
/// ([`crate::Mint`]) when the step is authored — by `InsertNode`
/// or `SetProgram` — never reused, never positional, and unique across
/// the document, ordered as minted ([`MintId`]). A profile piece's name
/// spells it ([`crate::names::ProfileEdgeRef`]).
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
pub struct StepId(pub MintId);

/// **A constraint's identity** (D10, FORK-S3M): minted from the
/// document's mint chain ([`crate::Mint`]) by the edit that adds it to
/// a placement ([`crate::DocEdit::AddConstraint`]), never reused and
/// never positional. A constraint is addressed by its placement and
/// this id.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
pub struct ConstraintId(pub MintId);

impl ConstraintId {
    /// **The `i`-th id of a bundle as written**: the id an authored
    /// bundle holds until the edit writing it mints its own, so a value
    /// names a mate written beside it by the mate's placeholder
    /// ([`crate::mate::pin::Placed::bundle`]). Digest zero, which no
    /// mint draws.
    #[must_use]
    pub const fn placeholder(i: u32) -> Self {
        Self(MintId::new(i, 0))
    }
}

/// A constraint id is spoken by its ordinal, `#<ordinal>`: the mint
/// never draws one twice, and a sentence always names the placement it
/// is of, so the digest would add length and no information. Where the
/// document is at hand, a constraint is spoken by its position in its
/// bundle instead ([`crate::spoken::Speaker::mate`]).
impl core::fmt::Display for ConstraintId {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "#{}", self.0.ordinal())
    }
}

impl RecipeNodeId {
    /// **The world's id** (D10): every document holds its world here,
    /// from its creation. Ordinal zero, which the mint never draws, so
    /// no inserted node can carry it.
    pub const WORLD: Self = Self::new(0, 0);

    /// The node id with mint ordinal `ordinal` and digest head
    /// `digest` ([`MintId::new`]).
    #[must_use]
    pub const fn new(ordinal: u32, digest: u64) -> Self {
        Self(MintId::new(ordinal, digest))
    }
}

impl StepId {
    /// The step id with mint ordinal `ordinal` and digest head
    /// `digest` ([`MintId::new`]).
    #[must_use]
    pub const fn new(ordinal: u32, digest: u64) -> Self {
        Self(MintId::new(ordinal, digest))
    }
}

pub use crate::mint::MintId;
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
    /// `tangent_arc_to`, `arc_onto` — the `Point` target form).
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
    /// A revolve's axis point component, in its profile's 2-D
    /// coordinates (Length).
    Origin(Axis3),
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
    /// A revolve's axis heading: the angle of its axis line from the
    /// profile's x axis, about the profile's normal (Angle).
    AxisHeading,
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
    /// A slide of a loft section's station ([`StationMove::Slide`]):
    /// the `step`th move of section `section`'s station, sections
    /// counted in skin order from the first, which has none.
    StationSlide {
        /// Which section, in skin order.
        section: usize,
        /// The move's place in the station.
        step: usize,
    },
    /// A spin of a loft section's station ([`StationMove::Spin`]),
    /// addressed as [`SlotId::StationSlide`] is.
    StationSpin {
        /// Which section, in skin order.
        section: usize,
        /// The move's place in the station.
        step: usize,
    },
    /// A slide value's length ([`crate::mate::Value`]).
    ValueSlide {
        /// Which constraint of the placement: the value.
        constraint: ConstraintId,
    },
    /// A spin value's angle ([`crate::mate::Value`]).
    ValueSpin {
        /// Which constraint of the placement: the value.
        constraint: ConstraintId,
    },
    /// A loft's / sweep's v-direction interpolation degree (Book
    /// §10.3) — STRUCTURAL: changing it changes the produced
    /// surface's knot vector, so it is Count-typed like every other
    /// structure-selecting slot (spec D3/A8).
    VDegree,
    /// A sweep's station count: how many rigid copies of the profile
    /// the path is instantiated at before skinning (Book §10.4) —
    /// STRUCTURAL, same rule.
    Stations,
    /// An OPERAND field (D10: an operand is a read): the slot holds a
    /// read of a variable of [`crate::OperandSlot::kind`]'s kind, written
    /// through the same door as an expression slot
    /// ([`crate::DocEdit::SetParam`] with a [`crate::SlotValue::Read`]).
    /// It has a kind and no dimension ([`SlotId::kind`],
    /// [`SlotId::dimension`]).
    Operand(crate::OperandSlot),
    /// Index `k` of the indexed read at body seat `seat`
    /// ([`crate::BodyRead`], REFERENCES DM3): a `Count`, so STRUCTURAL
    /// (which member is read is structure). A plain read carries no
    /// index slot.
    Index {
        /// The body seat whose read it indexes.
        seat: crate::OperandSlot,
        /// Which of the family's indices, from 0.
        k: u8,
    },
    /// One expression inside a profile PROGRAM (LIB-SWITCH §4c): loop
    /// index, step index, argument role. The LOOP coordinate is a VQ3
    /// sharpening of the design's `(step, arg)` sketch — a profile is
    /// plane + several loops, so the address needs it. Step indices are
    /// stable under every slot edit because program STRUCTURE changes
    /// only by [`crate::DocEdit::SetProgram`], which keeps every kept
    /// step's names as they are spelled and reports every name on a
    /// piece the new program does not draw and the old one drew, or
    /// could not be replayed to say (V2, `crates/profile/README.md`);
    /// for the carrier loop forms (`circle`/`circle_split`) `step` is 0.
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
    /// A revolve's axis point ([`SlotId::Origin`]).
    Origin,
}

impl VectorSlot {
    /// This family's slot for one axis — the inverse of
    /// [`SlotId::component`], and total.
    pub fn slot(self, axis: Axis3) -> SlotId {
        match self {
            Self::Origin => SlotId::Origin(axis),
        }
    }

    /// The family as a prose noun — the one spelling a user-facing
    /// rendering uses.
    pub fn label(self) -> String {
        match self {
            Self::Origin => "origin".to_owned(),
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
        self.slot(Axis3::X)
            .dimension()
            .unwrap_or_else(|| unreachable!("a vector family's components are scalar slots"))
    }
}

impl SlotId {
    /// **The kinds a value in this slot may have** — total over the
    /// vocabulary: a scalar slot holds exactly its dimension's kind
    /// ([`SlotKind::Is`]), an operand its field's
    /// ([`crate::OperandSlot::kind`]). Every door that writes a slot
    /// asks this of what the write reads.
    ///
    /// [`SlotKind::Is`]: crate::SlotKind::Is
    pub fn kind(self) -> crate::SlotKind {
        match (self, self.dimension()) {
            (Self::Operand(operand), _) => operand.kind(),
            (_, Some(dim)) => crate::SlotKind::Is(crate::VarKind::from(dim)),
            (_, None) => unreachable!("every slot but an operand has a dimension"),
        }
    }

    /// The dimension an expression in this slot must have (checked by
    /// `apply` on insert and on every expression edit, spec D6), `None`
    /// for an operand, which holds a read and no expression.
    pub fn dimension(self) -> Option<Dimension> {
        Some(match self {
            Self::Origin(_)
            | Self::Distance
            | Self::Radius
            | Self::ChamferDistance
            | Self::ShellThickness
            | Self::TubeMajorRadius
            | Self::TubeMinorRadius
            | Self::TubeWall => Dimension::Length,
            Self::RevolveAngle
            | Self::AxisHeading
            | Self::TubeWindowStart
            | Self::TubeWindowEnd => Dimension::Angle,
            Self::VDegree | Self::Stations | Self::Index { .. } => Dimension::Count,
            Self::StationSlide { .. } => Dimension::Length,
            Self::StationSpin { .. } => Dimension::Angle,
            Self::ValueSlide { .. } => Dimension::Length,
            Self::ValueSpin { .. } => Dimension::Angle,
            // Profile-program roles carry V2's per-role table; none is
            // Count, so `is_structural` stays false for every StepArg:
            // program structure is the STEP LIST, which no slot
            // addresses — it changes by `DocEdit::SetProgram`, which
            // rewrites no name and reports every one whose piece the
            // new program does not draw, unless the old one under the
            // current values did not draw it either (DM7).
            Self::Profile { arg, .. } => arg.dimension(),
            Self::Operand(_) => return None,
        })
    }

    /// The dimension of an EXPRESSION slot — a slot a node holds a
    /// formula in ([`Node::expr`], every slot [`Node::slots`] lists),
    /// which an operand never is. Asked of an operand, it panics: the
    /// caller had a read in hand and took it for an expression.
    pub fn expr_dimension(self) -> Dimension {
        self.dimension()
            .unwrap_or_else(|| unreachable!("{self} is an operand, which holds no expression"))
    }

    /// **Spec D6's slot rule over ONE address and one candidate
    /// expression**: the comparison itself, with nowhere else to write
    /// it down.
    ///
    /// Every door that decides whether an expression may sit in a slot
    /// asks this — [`Node::formula_dimension_fault`] per slot of an
    /// authored node, and `edit`'s `set_slot` of a formula the node
    /// does not hold yet, which is why the subject is a `(slot, expr)`
    /// pair rather than a node.
    pub(crate) fn dimension_fault<L: crate::expr::LeafSet>(
        self,
        expr: &crate::expr::ExprTree<L>,
    ) -> Option<SlotDimensionFault> {
        // A lone `0` stands at every slot's dimension (VR6).
        (self.dimension() != Some(expr.dim()) && !expr.is_zero()).then(|| SlotDimensionFault {
            slot: self,
            expected: self.kind(),
            found: expr.dim(),
        })
    }

    /// Whether this slot is a STRUCTURAL parameter (spec D3: the
    /// structural/continuous distinction is typed, not emergent —
    /// structural slots are exactly the Count-dimensioned ones).
    pub fn is_structural(self) -> bool {
        self.dimension() == Some(Dimension::Count)
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
            Self::AxisHeading => "axis heading".to_owned(),
            Self::TubeMajorRadius => "tube major radius".to_owned(),
            Self::TubeMinorRadius => "tube minor radius".to_owned(),
            Self::TubeWindowStart => "tube window start".to_owned(),
            Self::TubeWindowEnd => "tube window end".to_owned(),
            Self::TubeWall => "tube wall".to_owned(),
            Self::VDegree => "v degree".to_owned(),
            Self::Stations => "stations".to_owned(),
            Self::Operand(operand) => operand.label(),
            Self::Index { seat, k: 0 } => format!("{} index", seat.label()),
            Self::Index { seat, k } => format!("{} index {}", seat.label(), u32::from(k) + 1),
            Self::Profile { loop_, step, arg } => {
                format!("loop {loop_} step {step} · {}", arg.label())
            }
            Self::StationSlide { section, step } => {
                format!("section {} station slide {}", section + 1, step + 1)
            }
            Self::StationSpin { section, step } => {
                format!("section {} station spin {}", section + 1, step + 1)
            }
            Self::ValueSlide { .. } => String::from("slide value"),
            Self::ValueSpin { .. } => String::from("spin value"),
            // Every component variant answered above.
            Self::Origin(_) => self.component().map_or_else(
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
            Self::Distance
            | Self::Radius
            | Self::ChamferDistance
            | Self::ShellThickness
            | Self::RevolveAngle
            | Self::AxisHeading
            | Self::TubeMajorRadius
            | Self::TubeMinorRadius
            | Self::TubeWindowStart
            | Self::TubeWindowEnd
            | Self::TubeWall
            | Self::VDegree
            | Self::Stations
            | Self::Operand(_)
            | Self::Index { .. }
            | Self::Profile { .. }
            | Self::StationSlide { .. }
            | Self::StationSpin { .. }
            | Self::ValueSlide { .. }
            | Self::ValueSpin { .. } => None,
        }
    }

    /// Whether this slot addresses a value's number — an address only a
    /// value carries.
    pub fn is_value(self) -> bool {
        matches!(self, Self::ValueSlide { .. } | Self::ValueSpin { .. })
    }
}

impl core::fmt::Display for SlotId {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(&self.label())
    }
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
/// kind fixed by the type as a mate side's selection is. A crossing is
/// written out of the two sides of a mate (`Face` selections), so the
/// fields are face names by construction, and this is the record SAYING what the
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
///     outer: named(editor_core::EntityKind::Edge),
///     inner: named(editor_core::EntityKind::Edge),
/// };
///
/// fn named(kind: editor_core::EntityKind) -> editor_core::StableName {
///     editor_core::StableName {
///         kind,
///         node: editor_core::RecipeNodeId::new(0, 0),
///         path: Vec::new(),
///     }
/// }
/// ```
///
/// The RUNNING twin below — the same body, differing only in that the
/// two references are made through [`FaceName::new`] — is why that
/// block proves anything.
///
/// ```
/// let face = || {
///     editor_core::FaceName::new(named(editor_core::EntityKind::Face))
///         .expect("a face name is a face")
/// };
///
/// let _ = editor_core::InterfaceCrossing::Mate {
///     outer: face(),
///     inner: face(),
/// };
///
/// fn named(kind: editor_core::EntityKind) -> editor_core::StableName {
///     editor_core::StableName {
///         kind,
///         node: editor_core::RecipeNodeId::new(0, 0),
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
    /// instance's copy carries the part's product names verbatim, so
    /// the remainder's mate reads that same name there, and storing
    /// its host spelling too would be storing a derivable fact.
    ///
    /// **No provenance.** A crossing carries what the seam needs and
    /// nothing about where it came from: the mate the split observed
    /// is not a field, because no door resolves it, nothing
    /// recomputes from it, and an id the record cannot keep honest —
    /// a later delete of the mate is not reported — is a reference
    /// with no reader to protect.
    Mate {
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
/// [`Node::fillet`]/[`Node::chamfer`] sort and dedupe a selection.
/// Every field either tube kind
/// carries is a scalar, a bare direction, a node id, or this
/// two-variant window — nothing set-shaped, nothing with a second
/// spelling to fold — so a door could only re-wrap the struct
/// literal. The one place canonicalization could have applied is the
/// window's angle pair, and the paragraph above is why it must not.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub enum TubeWindow<S = crate::VarId> {
    /// The full ring — the donut.
    Full,
    /// The arc from `t0` to `t1`, radians about the spine axis from the
    /// reference direction, right-handed. Wedge caps close the ends.
    Arc {
        /// The window's start angle ([`SlotId::TubeWindowStart`]).
        t0: S,
        /// The window's end angle ([`SlotId::TubeWindowEnd`]).
        t1: S,
    },
}

/// **One move of a loft section's station**: a slide along, or a spin
/// about, one of the first section's own axes through its origin, by one
/// `Length` or one `Angle` (FORK-MOVE: no slot holds a component).
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub enum StationMove<S = crate::VarId> {
    /// Slid along the axis by a length ([`SlotId::StationSlide`]).
    Slide {
        /// The first section's axis it slides along.
        along: Axis3,
        /// How far (Length).
        by: S,
    },
    /// Spun about the axis by an angle, right-handed
    /// ([`SlotId::StationSpin`]).
    Spin {
        /// The first section's axis it spins about.
        about: Axis3,
        /// How far (Angle).
        by: S,
    },
}

/// **A loft section after the first**: its profile, drawn in its own
/// plane, and its station, the moves that carry that plane from the
/// first section's, applied in the order written.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(bound(
    serialize = "R: serde::Serialize, S: serde::Serialize",
    deserialize = "R: serde::Deserialize<'de>, S: serde::Deserialize<'de>"
))]
pub struct LoftSection<R = crate::VarId, S = crate::VarId> {
    /// The section's profile node.
    pub profile: R,
    /// The moves from the first section's plane to this one's.
    pub station: Vec<StationMove<S>>,
}

macro_rules! station_rows {
    ($section:expr, $station:expr, $out:expr) => {{
        for (step, m) in $station.into_iter().enumerate() {
            match m {
                StationMove::Slide { along: _, by } => {
                    $out.push((
                        SlotId::StationSlide {
                            section: $section,
                            step,
                        },
                        by,
                    ));
                }
                StationMove::Spin { about: _, by } => {
                    $out.push((
                        SlotId::StationSpin {
                            section: $section,
                            step,
                        },
                        by,
                    ));
                }
            }
        }
    }};
}

impl<R, S> LoftSection<R, S> {
    /// The rows of this section's station, as section `section` of its
    /// loft, shared.
    pub(crate) fn rows(&self, section: usize) -> Vec<(SlotId, &S)> {
        let mut out = Vec::new();
        station_rows!(section, &self.station, out);
        out
    }

    /// The rows of this section's station, exclusive.
    pub(crate) fn rows_mut(&mut self, section: usize) -> Vec<(SlotId, &mut S)> {
        let mut out = Vec::new();
        station_rows!(section, &mut self.station, out);
        out
    }

    /// **This section in another read and slot form**
    /// ([`Node::try_map_slots`]'s rule).
    ///
    /// # Errors
    ///
    /// The first refusal of `read` or `f`.
    pub fn try_map<R2, S2, E>(
        &self,
        read: impl FnOnce(&R) -> Result<R2, E>,
        f: &mut impl FnMut(&S) -> Result<S2, E>,
    ) -> Result<LoftSection<R2, S2>, E> {
        Ok(LoftSection {
            profile: read(&self.profile)?,
            station: self
                .station
                .iter()
                .map(|m| {
                    Ok(match m {
                        StationMove::Slide { along, by } => StationMove::Slide {
                            along: *along,
                            by: f(by)?,
                        },
                        StationMove::Spin { about, by } => StationMove::Spin {
                            about: *about,
                            by: f(by)?,
                        },
                    })
                })
                .collect::<Result<_, E>>()?,
        })
    }
}

/// **THE slot table of a tube window: which [`SlotId`] each of its
/// expressions carries, in enumeration order** — none for a full ring,
/// the two angles for an arc. Both tube kinds read it through
/// `tube_rows`, so neither can come to disagree with the other about
/// which angle is which.
macro_rules! window_rows {
    ($window:expr, $out:expr) => {{
        match $window {
            TubeWindow::Full => {}
            TubeWindow::Arc { t0, t1 } => {
                $out.push((SlotId::TubeWindowStart, t0));
                $out.push((SlotId::TubeWindowEnd, t1));
            }
        }
    }};
}

/// **The shared and exclusive readers of one borrow-generic row
/// table**: `rows` and `rows_mut`, each running `$table!` over `self`
/// under its own borrow. The table is one text and match ergonomics
/// bind `&S` or `&mut S` from it, so the two readers list the
/// same rows in the same order. The table is also handed the reader's
/// own name, so a table that holds another value's table reads it
/// through the same borrow.
macro_rules! row_readers {
    ($vis:vis $table:ident -> $key:ty, $slot:ty) => {
        /// The rows of this value's slot table, shared.
        $vis fn rows(&self) -> Vec<($key, &$slot)> {
            let mut out = Vec::new();
            $table!(self, rows, out);
            out
        }

        /// The rows of this value's slot table, exclusive.
        $vis fn rows_mut(&mut self) -> Vec<($key, &mut $slot)> {
            let mut out = Vec::new();
            $table!(self, rows_mut, out);
            out
        }
    };
}
pub(crate) use row_readers;

/// The expression a slot table's rows pair with `key`, `None` when no
/// row names it — the one lookup every slot and role table is read
/// through, for either borrow.
pub(crate) fn find_row<K: PartialEq, E>(rows: Vec<(K, E)>, key: K) -> Option<E> {
    rows.into_iter()
        .find(|(k, _)| *k == key)
        .map(|(_, expr)| expr)
}

/// **The `Expr`s a node carries OUTSIDE its slots**, in deterministic
/// order — `None` for the nodes that carry none, which is every node
/// but [`Node::Assertion`].
///
/// The slot vocabulary is the ordinary home for a node's expressions,
/// and it stays so: this is the escape hatch for the two expressions
/// whose dimension a slot ADDRESS cannot fix — an assertion's value
/// and its bound, whose dimension is whatever the value's is.
///
/// One order, three consumers: the evaluator resolves these once, the
/// content key hashes the resolved values, and the op reads the same
/// vector. `None` rather than an empty vector for a slot-only node —
/// the key writes nothing at all for those, so no existing document's
/// content key moves.
pub fn payload_exprs<P, S: Slot>(node: &Node<P, S>) -> Option<Vec<&S>> {
    expr_table!(node, exprs, Some, _rows => None)
}

/// **THE expression table of a node, borrow-generic**: the payload
/// carrier's expressions ([`payload_exprs`]) each passed through
/// `$wrap`, and every other node bound to `$rest` and answered by
/// `$other` (its slot rows, or nothing). One text, so the
/// readers [`payload_exprs`], [`Node::exprs`] and [`Node::exprs_mut`] cannot
/// come to disagree about which node carries what: a node's slot table
/// lists no row for the payload carrier (`node_rows`' last arm),
/// and this table lists no payload for any other node.
macro_rules! expr_table {
    ($node:expr, $exprs:ident, $wrap:expr, $rest:ident => $other:expr) => {{
        match $node {
            Node::Assertion { claim } => $wrap(claim.$exprs()),
            $rest @ (Node::Profile(_)
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
            | Node::Mirror { .. }
            | Node::Subtract { .. }
            | Node::Union { .. }
            | Node::Intersect { .. }
            | Node::Assemble { .. }
            | Node::Place { .. }
            | Node::World
            | Node::Space
            | Node::InstantiatePart { .. }
            | Node::Measure { .. }) => $other,
        }
    }};
}
use expr_table;

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
/// **A measure is authored in it, and stores a read.** A measure's
/// reference is written as a `SitedRef` and lowers
/// (`impl From<SitedRef> for Operand`) to the read the node stores: a
/// whole body's name to a read of `at`'s output, any other name to a
/// one-name selection of that output (`Operand::Select`), which the
/// door mints as a `Face`, `Edge` or `Vertex` variable. The selection's
/// body read is an ordinary DAG edge ([`crate::Doc::upstream`]) and its
/// name resolves against `at`'s own evaluated name table, through the
/// N5 ladder every other authored name takes. `name` is a bare
/// [`StableName`] because a measure reads a LENGTH between entities of
/// any kind: a face, an edge, a vertex, a whole body.
///
/// A mate's side is authored in its own type, [`SitedFace`], which
/// fixes the name's kind to a face and lowers the same way.
///
/// There is no `Option` on `at`: "as authored" is spelled
/// [`SitedRef::at_mint`].
///
/// **`Rebind` never moves a reference's `at`.** `at` is a DAG edge
/// the author chose, and an edit that rewrote it would be re-pointing
/// a dependency behind the author's back; only the NAME is repaired
/// (through its selection, `DocEdit::Rebind` with the selection's
/// body).
#[derive(
    Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
#[serde(deny_unknown_fields)]
pub struct SitedRef<At = RecipeNodeId> {
    /// Where the carrier is read: a measure's node, whose evaluated
    /// value is the PLACED geometry when that node placed it.
    pub at: At,
    /// The entity's stable name, resolved against `at`'s table.
    pub name: StableName,
}

impl<At> SitedRef<At> {
    /// A reference read at `at`.
    pub fn new(at: At, name: StableName) -> Self {
        Self { at, name }
    }
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
}

/// **A face as an author writes it for a mate: a FACE name, and the
/// node it is read at.**
///
/// [`SitedRef`]'s shape with the name's kind fixed by the type. It
/// lowers (`impl From<SitedFace> for Operand`) to a one-name selection
/// of `at`'s output, which the door mints as a `Face` variable, so the
/// selection's body read is an ordinary DAG edge and the name resolves
/// against `at`'s own evaluated table through the N5 ladder every
/// other authored name takes. A mate reads it inside a pose — the
/// plane of the face ([`crate::pose::PoseDef::Plane`]).
///
/// Where a face name comes from is [`FaceName`]'s own doc: the three
/// boundaries that turn DATA into one, and the single in-crate door
/// that re-derives one without re-asking the kind.
///
/// ```
/// use editor_core::pose::PoseDef;
/// use editor_core::{
///     EntityKind, FaceName, Formula, Mate, Operand, RecipeNodeId, SitedFace,
///     StableName,
/// };
/// let face = FaceName::new(StableName {
///     kind: EntityKind::Face,
///     node: RecipeNodeId::new(0, 0),
///     path: Vec::new(),
/// })
/// .expect("a face name is a face");
/// let plane = |face: FaceName| {
///     Operand::Pose(Box::new(PoseDef::Plane {
///         face: SitedFace::at_mint(face).into(),
///     }))
/// };
/// let flipped = Operand::Pose(Box::new(PoseDef::Flip {
///     pose: plane(face.clone()).into(),
/// }));
/// let _: Mate<Formula> = Mate {
///     on: plane(face),
///     to: flipped,
///     orient: editor_core::Orientation::IDENTITY,
/// };
/// ```
///
/// The constructor's other answer is a typed refusal, never a face
/// name that is not one:
///
/// ```
/// use editor_core::{EntityKind, FaceName, RecipeNodeId, StableName};
/// let edge = StableName {
///     kind: EntityKind::Edge,
///     node: RecipeNodeId::new(0, 0),
///     path: Vec::new(),
/// };
/// assert_eq!(FaceName::new(edge).unwrap_err().found, EntityKind::Edge);
/// ```
#[derive(
    Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
#[serde(deny_unknown_fields)]
pub struct SitedFace {
    /// The node whose output the face is read in — the PLACED
    /// geometry, when that node placed it.
    pub at: RecipeNodeId,
    /// The face's stable name.
    pub name: FaceName,
}

impl SitedFace {
    /// A side read at the node that minted the name — the degenerate
    /// case, and the honest spelling of "as authored".
    pub fn at_mint(name: FaceName) -> Self {
        Self {
            at: name.node,
            name,
        }
    }

    /// A side read at `at`.
    pub fn new(at: RecipeNodeId, name: FaceName) -> Self {
        Self { at, name }
    }
}

/// **What makes a node's structural content invalid**
/// ([`Node::input_fault`]) — one vocabulary for the two edit doors and
/// the load door's re-check, so each rule has one definition and three
/// callers rather than three copies.
///
/// It covers the indexed reads at a node's body seats
/// ([`crate::BodyRead`]): a structural form that only a hand-built
/// variant or a corrupt file can arrive without, refused at every door
/// that ADMITS a node rather than repaired at one. A selection's
/// designation form is the selection's own
/// ([`crate::SelectionFault`]).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputFault {
    /// A union's or an intersect's family argument carries an index
    /// ([`crate::BodyRead`]): the argument reads the family whole, and
    /// one member read by index is a `Body`, spelled in a list.
    IndexedFamily,
    /// An indexed read carries a number of indices other than its
    /// family's rank: a family is keyed by the indices its definer
    /// reaches (D10 `[repetition]`), and `xs[i, j]` writes one value per
    /// index, outer first. A body that is no family has rank zero.
    IndexRank {
        /// The body seat whose read it is.
        seat: crate::OperandSlot,
        /// How many indices it carries.
        found: usize,
        /// How many indices the family it reads is keyed by.
        rank: usize,
    },
}

// The ONE prose vocabulary for this fault, forwarded by every door
// that renders it rather than restated.
impl core::fmt::Display for InputFault {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::IndexedFamily => f.write_str(
                "the family argument carries an index — a family argument reads every member, \
                 and one member read by index is a body, spelled in a list",
            ),
            Self::IndexRank { seat, found, rank } => write!(
                f,
                "the read at its {seat} carries {found} indices, and the family it reads is \
                 keyed by {rank} — a member is read by one value per index, outer first"
            ),
        }
    }
}

/// **An [`InputFault`] as the load door holds it**: each arm is
/// [`InputFault`]'s namesake, and so is the sentence.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ListFault {
    /// [`InputFault::IndexedFamily`].
    IndexedFamily,
    /// [`InputFault::IndexRank`].
    IndexRank {
        /// The body seat whose read it is.
        seat: crate::OperandSlot,
        /// How many indices it carries.
        found: usize,
        /// How many indices the family it reads is keyed by.
        rank: usize,
    },
}

impl InputFault {
    /// This fault as a [`ListFault`].
    #[must_use]
    pub fn list_fault(self) -> ListFault {
        match self {
            Self::IndexedFamily => ListFault::IndexedFamily,
            Self::IndexRank { seat, found, rank } => ListFault::IndexRank { seat, found, rank },
        }
    }
}

impl From<ListFault> for InputFault {
    fn from(fault: ListFault) -> Self {
        match fault {
            ListFault::IndexedFamily => Self::IndexedFamily,
            ListFault::IndexRank { seat, found, rank } => Self::IndexRank { seat, found, rank },
        }
    }
}

// [`InputFault`]'s prose, forwarded rather than restated.
impl core::fmt::Display for ListFault {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", InputFault::from(*self))
    }
}

/// **What makes a formula unusable at a slot**
/// ([`Node::formula_dimension_fault`]; spec D6) — the rule "a slot's
/// formula carries the dimension the slot address fixes", asked by the
/// edit doors before they lower. A stored slot holds a variable, and a
/// variable of another kind is the read walks' fault (VR4).
///
/// The domain is [`Node::slots`], which is EVERY node kind: a profile
/// program's step arguments, an extrude's distance, a value's number
/// and a pattern's count are the same question asked of
/// different addresses.
/// ONE fact, so a struct: the dimensions disagree. A slot
/// [`Node::slots`] names and [`Node::expr`] cannot answer for is not a
/// property of the document at all — it is a disagreement between two
/// matches in this module, which [`Node::formula_dimension_fault`]
/// asserts against at the site rather than routing to a door as a
/// refusal.
///
/// Its [`Display`](core::fmt::Display) is the refusal's one CLAUSE,
/// which each door forwards into its own subject — the shape
/// [`crate::placement::Frame::admission_fault`] carries for the frame
/// rule. The sentence is written here and reaches a reader as
/// "slot radius needs …" from the edit door.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct SlotDimensionFault {
    /// The offending slot.
    pub slot: SlotId,
    /// The kind the address fixes: a scalar slot's dimension, or an
    /// operand's kind, which no expression has.
    pub expected: crate::SlotKind,
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
        match expected {
            crate::SlotKind::Is(kind) if let Some(dim) = kind.dimension() => write!(
                f,
                "slot {} needs {} {dim} expression, got {} {found}",
                slot.label(),
                dim.article(),
                found.article()
            ),
            _ => write!(
                f,
                "slot {} reads {expected}, not {} {found} expression",
                slot.label(),
                found.article()
            ),
        }
    }
}

/// What makes a [`Node::Assertion`]'s bound unusable
/// ([`Node::assertion_bound_fault`]) — one vocabulary for the edit
/// door, the load door's re-check and evaluation's backstop: an
/// assertion compares one quantity, so the bound's dimension is the
/// value's.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct AssertionBoundFault {
    /// The value's dimension.
    pub measured: Dimension,
    /// The bound's dimension.
    pub bound: Dimension,
}

impl AssertionBoundFault {
    /// **E10's agreement itself, stated once.**
    pub(crate) fn against(measured: Dimension, bound: Dimension) -> Option<Self> {
        (measured != bound).then_some(Self { measured, bound })
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
#[serde(bound(
    serialize = "P: serde::Serialize",
    deserialize = "P: serde::Deserialize<'de>"
))]
pub enum Node<P, S: Slot = crate::VarId> {
    /// A programmatic sketch, carried opaquely (F4; never re-modeled).
    Profile(P),
    /// Extrude an upstream profile by a Length depth along its own
    /// normal, in coordinates of its own: the start cap is the
    /// profile's plane and the end cap lies the depth along `+z`.
    /// Which side of a face its material lies on is the placement's
    /// mates' to say.
    Extrude {
        /// The profile node extruded.
        profile: S::Read,
        /// Extrusion depth ([`SlotId::Distance`]): a size, refused
        /// unless definitely positive (`sweep::Extrusion::Distance`).
        distance: S,
    },
    /// Revolve an upstream profile about an axis in its own plane.
    ///
    /// # The axis is written in the profile's own plane
    ///
    /// The axis of revolution is a line in the profile's 2-D axes,
    /// through a point at two lengths and along a heading, and its lift
    /// to 3-D is the node's `axis` output. A revolve's axis has to lie in
    /// the profile's plane, and a 3-D axis would make that a coincidence
    /// the evaluator checks, a tolerance decision on a direction
    /// residual that classifies a tilt in-plane at one model scale and
    /// not another (the audit's F15 row). Two lengths and an angle in
    /// the plane cannot leave it, and a heading cannot be a zero
    /// direction: both errors are unrepresentable rather than checked,
    /// and every axis a revolve could legally take is expressible here.
    Revolve {
        /// The profile node revolved.
        profile: S::Read,
        /// A point on the axis, in the profile's 2-D coordinates —
        /// Length, [`SlotId::Origin`]`(X | Y)`. There is no `Z` slot:
        /// the third coordinate of a point in a plane is not a number
        /// somebody may type.
        axis_through: [S; 2],
        /// The axis's heading from the profile's x axis, about its
        /// normal — Angle, [`SlotId::AxisHeading`]: `0` is along x and
        /// `turn/4` along y.
        axis_heading: S,
        /// Sweep angle ([`SlotId::RevolveAngle`]).
        angle: S,
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
    /// # Its own coordinates
    ///
    /// A tube is built about its own `z` axis, its spine centred on its
    /// own origin and its window's angles measured from its own `x`; it
    /// reads no frame, and a placement says where it stands.
    Tube {
        /// The spine circle's radius ([`SlotId::TubeMajorRadius`]).
        major_radius: S,
        /// The traversed window — a full ring or an arc.
        window: TubeWindow<S>,
        /// The tube's cross-sectional radius
        /// ([`SlotId::TubeMinorRadius`]).
        minor_radius: S,
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
        /// The spine circle's radius ([`SlotId::TubeMajorRadius`]).
        major_radius: S,
        /// The traversed window — a full ring (a torus shell, whose
        /// cavity is a void) or an arc (an open elbow of annular
        /// section).
        window: TubeWindow<S>,
        /// The OUTER cross-sectional radius
        /// ([`SlotId::TubeMinorRadius`]).
        minor_radius: S,
        /// The wall thickness ([`SlotId::TubeWall`]). REQUIRED: the
        /// inner wall stores `minor_radius − wall`, one IEEE
        /// subtraction of the caller's own two numbers.
        wall: S,
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
    ///
    /// Every section is a 2-D profile. The first is the loft's own
    /// plane; each after it stands at its station, slides and spins in
    /// the first section's own axes (D10 `[variables.kinds]`), so the
    /// loft computes in the first section's coordinates and reads no
    /// frame.
    Loft {
        /// The first section's profile node.
        first: S::Read,
        /// The sections after the first, in skin order (≥ 1). Order is
        /// data: reversing it reverses the produced surface's
        /// v-direction.
        then: Vec<LoftSection<S::Read, S>>,
        /// The v-direction interpolation degree ([`SlotId::VDegree`]);
        /// must satisfy `1 ≤ degree ≤ sections − 1`, checked at
        /// evaluation against the resolved value.
        v_degree: S,
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
        profile: S::Read,
        /// The path node: a profile whose FIRST loop's chain is the
        /// trajectory (an open or closed polyline/arc chain).
        path: S::Read,
        /// How many stations the path is instantiated at
        /// ([`SlotId::Stations`]); must be ≥ 2.
        stations: S,
        /// The v-direction interpolation degree ([`SlotId::VDegree`]);
        /// must satisfy `1 ≤ degree ≤ stations − 1`.
        v_degree: S,
    },
    /// Constant-radius rolling-ball fillets on a selection of one body's
    /// edges.
    ///
    /// The op is [`sweep::blend::build::fillet_edges`] over the
    /// resolved selection; anything outside its two assembly front
    /// doors is a typed refusal
    /// ([`crate::eval::NodeErrorKind::Blend`]), never a silent
    /// pass-through of the input body.
    ///
    /// # The selection freezes (ruled, #217)
    ///
    /// `selection` reads one `Edges` variable ([`crate::VarDef::Select`]),
    /// which states the body once and names its edges by stable name and
    /// nothing else — there is no "every edge" variant. A click-selection
    /// is a commitment: an upstream edit that adds edges does not extend
    /// it, and an upstream edit that removes a selected edge is a typed
    /// refusal at the selection, not a silent shrink. To select
    /// everything as of now, call [`all_edges`](crate::all_edges) and
    /// store what it returns.
    ///
    /// [`crate::DocEdit::Rebind`] is the repair: a 1:1 `from → to`
    /// rewrite of the selections of one body, which can swap a name or,
    /// when `to` is already selected, shrink the set by one, and cannot
    /// grow it. Adding an edge means authoring a new selection.
    Fillet {
        /// The constant blend radius ([`SlotId::Radius`]).
        radius: S,
        /// The edges blended: one `Edges` read
        /// ([`crate::OperandSlot::Selection`]).
        selection: S::Read,
    },
    /// Equal-setback flat chamfers on a selection of one body's edges —
    /// [`Node::Fillet`]'s twin.
    ///
    /// The op is [`sweep::blend::build::chamfer_edges`], which is
    /// `fillet_edges` modulo the size's meaning: `distance` is the
    /// setback measured along each support from the source edge, not a
    /// rolling ball's radius. Everything else is the fillet's: the same
    /// two assembly front doors, the same typed refusal on anything
    /// outside them ([`crate::eval::NodeErrorKind::Blend`] carrying
    /// [`sweep::blend::BlendKind::Chamfer`]), and the same frozen
    /// selection.
    ///
    /// # Why this is a separate variant and not a flag on `Fillet`
    ///
    /// The two carry different quantities in their size slot
    /// ([`SlotId::Radius`] vs [`SlotId::ChamferDistance`]); separate
    /// variants make the size's meaning readable off the node kind, and
    /// make the naming discrimination structural: the minting node is
    /// what tells a chamfer's blend from a fillet's at every selector
    /// (RECIPE-DOORS D3).
    Chamfer {
        /// The setback along both supports
        /// ([`SlotId::ChamferDistance`]).
        distance: S,
        /// The edges chamfered: one `Edges` read
        /// ([`crate::OperandSlot::Selection`]).
        selection: S::Read,
    },
    /// **Hollow a body into a thin solid** of wall `thickness`, with the
    /// faces `open` selects re-authored as annular rims.
    ///
    /// The op is the verb seat's `verbs::Shell` over `topo::shell_open`:
    /// every boundary face is replaced by its inward offset and the
    /// offset boundary inserted as a cavity, then each designated chart
    /// is lifted into a rim. Every check is the kernel's, carried
    /// unaltered as [`crate::eval::NodeErrorKind::Shell`]; the node
    /// never passes its input body through.
    ///
    /// `open` reads one `Faces` variable ([`crate::VarDef::Select`]),
    /// whose body is the one hollowed. Its faces are in designation
    /// order, because the kernel's `RimNaming::sources` preserves it and
    /// a chart's rim is its first designated face. An empty selection is
    /// the sealed hollow (`topo::shell`), not a refusal: a shell of
    /// nothing opened is a closed thin solid.
    Shell {
        /// The wall thickness — a magnitude
        /// ([`SlotId::ShellThickness`], Length).
        thickness: S,
        /// The faces opened into rims: one `Faces` read
        /// ([`crate::OperandSlot::Open`]).
        open: S::Read,
    },
    /// Split a target body by a tool.
    Split {
        /// The body split.
        target: crate::BodyRead<S>,
        /// The splitting tool: a `Plane` read, or a frame's coordinate
        /// plane read through its role.
        tool: crate::pose::RoleRead<S::Read>,
    },
    /// **The mirror image of a body** (D10 `[repetition]`,
    /// `docs/MIRROR-DESIGN.md` P4): a construction defining a new `Body`
    /// in its source's space, the source reflected across `plane`
    /// (`topo::mirror_body`). A reflection is not a pose, so no
    /// placement mirrors and every pose is proper. Its plane reads only
    /// geometry of that space, as a split's tool does. Its cells keep
    /// their names in the source, as a copy's do.
    Mirror {
        /// The body reflected.
        body: crate::BodyRead<S>,
        /// The mirror plane: a `Plane` read, or a frame's coordinate
        /// plane read through its role.
        plane: crate::pose::RoleRead<S::Read>,
    },
    /// **The union** (`crates/editor-core/REFERENCES.md` DM4): one
    /// argument of kind `Bodies` ([`Bodies`]), ONE body out, the fold of
    /// the kernel's pair verb in member order.
    ///
    /// A member's entities are named by the read they came through
    /// ([`crate::RoleSeg::From`]), one wrapper whatever the fold's
    /// depth, so a member's names depend on neither its position in the
    /// list nor how many members precede it, and
    /// [`crate::DocEdit::SetMembers`] can drop one without disturbing
    /// the rest. A union of one body builds that body with no boolean
    /// run, and of none the typed empty body.
    ///
    /// # Contact
    ///
    /// Contact is judged pairwise, before the fold: every two members
    /// whose boxes meet are evaluated as the two-member union of just
    /// those two, so two members that touch glue or refuse exactly as a
    /// pair boolean's operands do, in every member order, and for a
    /// contact a third member covers too. The fold reads each carrier
    /// pair's verdict from that judgement, through its faces' member
    /// lineage, and decides none of its own.
    Union {
        /// The members, in fold order (D9: the order is the list's, and
        /// the list is data).
        members: Bodies<crate::BodyRead<S>>,
    },
    /// **The intersect** (DM4): the material in every member, over the
    /// kernel's intersection verb, judged and folded as [`Node::Union`]
    /// is. An empty fold step is the typed empty result. A member that
    /// holds pieces (a [`Node::Assemble`], or a copy of one) is met
    /// piece by piece, and the result holds each piece's meet as a
    /// piece.
    Intersect {
        /// The members, in fold order.
        members: Bodies<crate::BodyRead<S>>,
    },
    /// **The subtract** (DM4): `from` cut by `tool`, the one pair node,
    /// since difference neither commutes nor associates. Several tools
    /// are `Subtract { from, tool: Union([tools…]) }`. A `from` that
    /// holds pieces (a [`Node::Assemble`], or a copy of one) is cut
    /// piece by piece, and the result holds each piece's cut as a
    /// piece; a piece the cut empties drops out.
    Subtract {
        /// The body cut.
        from: crate::BodyRead<S>,
        /// The body cut away.
        tool: crate::BodyRead<S>,
    },
    /// **The assemble** (`⊔`, D10 `[operations.operators]`): its
    /// members side by side as the pieces of one body, each a separate
    /// solid set, none re-decided against another. Its door runs the
    /// at-rest census over the pieces, one arrival per member, carrying
    /// every member's own records, so a meeting inside a member is that
    /// member's; the verdict is kept on its value
    /// ([`crate::product::Census`]), which a copy or a pick of it
    /// carries, and every reader reads that one verdict. Its result is
    /// an ordinary operand: a subtract or an intersect reading it acts
    /// on each piece and keeps what it made as pieces
    /// (`(p ⊔ q) − t = (p − t) ⊔ (q − t)`), and a union fuses its other
    /// members with the pieces they meet, carrying the `⊔`'s records.
    Assemble {
        /// The members, in graft order.
        members: Bodies<crate::BodyRead<S>>,
    },
    /// **A placement** (D10 `[placement.placement]`): one copy of its
    /// `source` under the one rigid motion its constraints pin, defining
    /// one output, the copy, whose names are the source's. Two
    /// placements of one body are two copies.
    ///
    /// Its copy is in its `target`, the space it places into, and its
    /// bundle pins it there: its constraints set every freedom. They
    /// are addressed by their minted ids, never by position. Which copy
    /// a constraint pins is which placement holds it, so a constraint
    /// has no role.
    Place {
        /// The body copied.
        source: crate::BodyRead<S>,
        /// The space the copy is placed into: a `Space` read.
        target: S::Read,
        /// The bundle, in the order written.
        constraints: Vec<crate::mate::Constraint<S>>,
    },
    /// **The world** (D10 `[placement.world]`): one undeletable node
    /// every document holds from its creation, at
    /// [`RecipeNodeId::WORLD`], defining the world's space
    /// ([`VarId::WORLD`]). It is not on the wire: every document has it.
    World,
    /// **A new space** (D10 `[variables.kinds]`, `new_space()`): reads
    /// nothing and defines one `Space`, a coordinate system no
    /// construction is written in, which placements place into.
    Space,
    /// An instance of another document's product (ASSEMBLY-DESIGN
    /// A2/A3, ASM-2A D-1): a LEAF — its material crosses the document
    /// seam rather than arriving from an upstream node, so it reads
    /// no operand and the DAG has nothing to schedule ahead of it.
    ///
    /// **An instance is a placement** (D10): its copy is in its
    /// `target`, where its constraints pin it.
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
        /// The space the copy is placed into: a `Space` read.
        target: S::Read,
        /// The bundle that pins the instance's copy (D10: an
        /// instance of a part is a placement), addressed by minted id,
        /// as a [`Node::Place`]'s is. Its copy side reads a face of the
        /// instance's own output: the copied shape in the part's own
        /// coordinates, which the solve reads off the part, never off
        /// the copy (`mate::solve`'s `copy_side`).
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        constraints: Vec<crate::mate::Constraint<S>>,
    },
    /// **A measurement** (ERROR-DESIGN E3): an operation holding one
    /// closed-form [`crate::MeasurePrimitive`] over two sited
    /// references, defining one scalar output of the primitive's
    /// dimension. That output is **observed** (D10): a function of the
    /// built geometry, read only by an [`Node::Assertion`], directly or
    /// through a definition. Arithmetic over measured values is a
    /// defined variable over the measures' outputs.
    ///
    /// # What a reference denotes: the carrier in the body it reads
    ///
    /// Each reference reads a variable: a `Body` (a whole body, the scope
    /// `min_clearance` takes), or a `Face` or `Edge` selection, whose body
    /// read is what makes a measure report placed geometry: a selection
    /// of a transform's output names the moved carrier, since a rigid
    /// transform hands its input's names through unchanged. Deleting the
    /// body leaves the reference unresolved, reported at the delete
    /// (`Maintenance::Strand`); a name that stops resolving is the
    /// selection's typed refusal, repaired by `Rebind`.
    Measure {
        /// The measurement, over its two references in argument order:
        /// each a `Body`, `Face` or `Edge` read
        /// ([`crate::OperandSlot::Measured`]).
        primitive: crate::measure::MeasurePrimitive<S::Read>,
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
    /// declaration, and `build()` never consults it: an assertion
    /// checks and never places (D10).
    Assertion {
        /// What it claims: a variable against a bound, or a clearance
        /// between two seats within its bounds.
        claim: crate::measure::Claim<S>,
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

/// The rows of a vector of expressions stored in [`Axis3::ALL`] order:
/// component `ax` carries the slot `$slot(ax)`, for any family `$slot`
/// names — a variant, or a closure over a placement step's index. A pair authored in a
/// sketch frame's 2-D coordinates yields `X` and `Y` alone, because a
/// point in a plane has two components and `Z` names none of them.
/// [`axis_arity`] holds the vector to two or three components, so the
/// zip cannot drop one.
macro_rules! axis_rows {
    ($slot:expr, $v:expr, $out:expr) => {{
        let v = $v;
        axis_arity(&*v);
        for (ax, e) in Axis3::ALL.into_iter().zip(v) {
            $out.push(($slot(ax), e));
        }
    }};
}

/// Compiles for an array of two or three components and nothing else:
/// the vectors [`axis_rows`] pairs with [`Axis3::ALL`].
const fn axis_arity<E, const N: usize>(_: &[E; N]) {
    const {
        assert!(
            N == 2 || N == 3,
            "an axis vector has two or three components"
        )
    }
}

/// The rows both tube kinds share: the reference direction, the two
/// radii, then whatever the window carries ([`window_rows`]). The
/// hollow kind appends its wall after them.
macro_rules! tube_rows {
    ($major:expr, $minor:expr, $window:expr, $out:expr) => {{
        $out.push((SlotId::TubeMajorRadius, $major));
        $out.push((SlotId::TubeMinorRadius, $minor));
        window_rows!($window, $out);
    }};
}

/// **THE slot table of a node: which [`SlotId`] each of its
/// expressions carries, in enumeration order** — the node's own field
/// order, deterministic.
///
/// The one declaration [`Node::slots`], [`Node::expr`] and
/// [`Node::expr_mut`] read, through [`row_readers`], so a slot the
/// enumeration lists is a slot the addressing answers for, and the
/// shared and exclusive borrows reach the same field. `$rows` names the
/// reader of the borrow in hand, which the profile arm asks of its
/// payload.
///
/// Exhaustive on the node vocabulary, so a new node kind is classified
/// here or the compile breaks — and on FIELDS: every arm names its
/// variant's fields, with `_` for the ones that are not slots (input
/// edges, recipe payload the content key feeds by hand), so a field
/// added to a node does not compile until it is stated a slot or not.
/// **The index slots of one body seat's read** ([`SlotId::Index`]),
/// in index order: none for a plain read.
macro_rules! index_rows {
    ($seat:expr, $read:expr, $rows:ident, $out:expr) => {{
        let seat = $seat;
        for (k, e) in $read.$rows() {
            $out.push((SlotId::Index { seat, k }, e));
        }
    }};
}

macro_rules! node_rows {
    ($node:expr, $rows:ident, $out:expr) => {{
        use crate::OperandSlot as O;
        use SlotId as S;
        match $node {
            // A profile's slots are its program's. The payload keys its
            // rows by program address, so it answers `S::Profile` and
            // no other slot.
            Node::Profile(p) => {
                for ((loop_, step, arg), e) in p.$rows() {
                    $out.push((S::Profile { loop_, step, arg }, e));
                }
            }
            Node::Extrude {
                profile: _,
                distance,
            } => $out.push((S::Distance, distance)),
            Node::Fillet {
                radius,
                selection: _,
            } => $out.push((S::Radius, radius)),
            Node::Chamfer {
                distance,
                selection: _,
            } => $out.push((S::ChamferDistance, distance)),
            Node::Shell { thickness, open: _ } => $out.push((S::ShellThickness, thickness)),
            // X and Y only: the profile's plane supplies the third
            // coordinate, and a slot for it would be a number nobody
            // may set.
            Node::Revolve {
                profile: _,
                axis_through,
                axis_heading,
                angle,
            } => {
                axis_rows!(S::Origin, axis_through, $out);
                $out.push((S::AxisHeading, axis_heading));
                $out.push((S::RevolveAngle, angle));
            }
            Node::Tube {
                major_radius,
                minor_radius,
                window,
            } => tube_rows!(major_radius, minor_radius, window, $out),
            Node::HollowTube {
                major_radius,
                minor_radius,
                window,
                wall,
            } => {
                tube_rows!(major_radius, minor_radius, window, $out);
                $out.push((S::TubeWall, wall));
            }
            Node::Loft {
                first: _,
                then,
                v_degree,
            } => {
                for (k, section) in then.into_iter().enumerate() {
                    $out.extend(section.$rows(k + 1));
                }
                $out.push((S::VDegree, v_degree));
            }
            Node::Sweep {
                profile: _,
                path: _,
                stations,
                v_degree,
            } => {
                $out.push((S::Stations, stations));
                $out.push((S::VDegree, v_degree));
            }
            // A placement's slots are its shapes' indices, then each
            // constraint's: a world read's scalars and a value's number,
            // addressed by the constraint.
            Node::Place {
                source,
                target: _,
                constraints,
            } => {
                index_rows!(O::Source, source, $rows, $out);
                $out.extend(constraints.$rows());
            }
            // The reference itself takes no arguments (AQ4 — the
            // referenced document evaluates at its OWN parameters).
            Node::InstantiatePart {
                doc_ref: _,
                interface: _,
                target: _,
                constraints,
            } => $out.extend(constraints.$rows()),
            Node::World | Node::Space => {}
            Node::Split { target, tool: _ } => index_rows!(O::Target, target, $rows, $out),
            Node::Mirror { body, plane: _ } => index_rows!(O::Body, body, $rows, $out),
            Node::Subtract { from, tool } => {
                index_rows!(O::From, from, $rows, $out);
                index_rows!(O::Cut, tool, $rows, $out);
            }
            Node::Union { members } | Node::Intersect { members } | Node::Assemble { members } => {
                for (seat, member) in members.$rows() {
                    index_rows!(seat, member, $rows, $out);
                }
            }
            // Neither carries a SLOT. A slot's address fixes its
            // dimension ([`SlotId::dimension`]); a measure reads names,
            // and an assertion's value and bound take their dimension
            // from the variable the value reads, which no slot address
            // can state. Both are payload instead
            // ([`payload_exprs`]), and so are a clearance's bounds.
            Node::Measure { primitive: _ } | Node::Assertion { claim: _ } => {}
        }
    }};
}

impl<P: crate::program::SlotPayload<S>, S: Slot> Node<P, S> {
    row_readers!(pub(crate) node_rows -> SlotId, S);

    /// The expression slots this node actually carries, deterministic
    /// order — the domain of [`Node::expr`]. Profile nodes enumerate
    /// their PROGRAM's slots (LIB-SWITCH §4c behavior delta 3: the
    /// formerly slot-free payload now carries one slot per continuous
    /// step argument), through the payload's own [`crate::ProfilePayload`]
    /// rows.
    pub fn slots(&self) -> Vec<SlotId> {
        self.rows().into_iter().map(|(slot, _)| slot).collect()
    }

    /// The expression in a named slot, `None` if this node type does
    /// not carry that slot (named access only, spec D5).
    pub fn expr(&self, slot: SlotId) -> Option<&S> {
        find_row(self.rows(), slot)
    }

    /// Mutable access to a named slot's expression (the edit layer's
    /// substrate; all validation lives in `apply`, spec D6).
    pub fn expr_mut(&mut self, slot: SlotId) -> Option<&mut S> {
        find_row(self.rows_mut(), slot)
    }

    /// **Every expression this node carries**: its slot rows, or a
    /// payload carrier's expressions ([`payload_exprs`]) — the one walk
    /// the edit door lowers and checks, a document's read queries ask,
    /// and a split or an inline re-points readers through.
    ///
    pub(crate) fn exprs(&self) -> Vec<&S> {
        expr_table!(self, exprs, core::convert::identity, rest => rest
            .rows()
            .into_iter()
            .map(|(_, e)| e)
            .collect())
    }

    /// [`Node::exprs`], exclusive: the same expressions in the same
    /// order.
    pub(crate) fn exprs_mut(&mut self) -> Vec<&mut S> {
        expr_table!(self, exprs_mut, core::convert::identity, rest => rest
            .rows_mut()
            .into_iter()
            .map(|(_, e)| e)
            .collect())
    }
}

impl<P> Node<P> {
    /// **The operands this node reads**, each at its address, in field
    /// order — the reads `crate::Doc::reads` lists first and
    /// [`crate::Doc::upstream`] expands to the operations that define
    /// them, the one dependency relation the schedule, the content key,
    /// acyclicity and the cascade read.
    ///
    /// Every arm names its variant's fields, with `_` for the ones that
    /// are not operands, so a reference added to a node does not
    /// compile until it is stated an operand or not.
    pub fn operand_rows(&self) -> Vec<(crate::OperandSlot, VarId)>
    where
        P: crate::ProfilePayload,
    {
        use crate::OperandSlot as O;
        match self {
            // Each constraint's pose reads, in bundle order, `on` before
            // a pose `to`; a world read reads no variable.
            Node::InstantiatePart {
                doc_ref: _,
                interface: _,
                target,
                constraints,
            } => {
                let mut v = vec![(O::Space, *target)];
                v.extend(side_rows(constraints));
                v
            }
            Node::Place {
                source,
                target,
                constraints,
            } => {
                let mut v: Vec<(O, VarId)> = vec![(O::Source, source.read)];
                v.push((O::Space, *target));
                v.extend(side_rows(constraints));
                v
            }
            | Node::World
            | Node::Space
            // An assertion's value is a payload expression
            // ([`payload_exprs`]), not an operand.
            | Node::Assertion {
                claim: crate::measure::Claim::Value { .. },
            } => Vec::new(),
            // A clearance's two seats are operands; its bounds are
            // payload.
            Node::Assertion {
                claim: crate::measure::Claim::Clearance { a, b, .. },
            } => vec![(O::Cleared(0), *a), (O::Cleared(1), *b)],
            Node::Profile(p) => p.read_rows().into_iter().map(|(o, &r)| (o, r)).collect(),
            Node::Extrude {
                profile,
                distance: _,
            } => vec![(O::Profile, *profile)],
            Node::Revolve {
                profile,
                axis_through: _,
                axis_heading: _,
                angle: _,
            } => vec![(O::Profile, *profile)],
            Node::Tube { .. } | Node::HollowTube { .. } => Vec::new(),
            Node::Loft {
                first,
                then,
                v_degree: _,
            } => core::iter::once(*first)
                .chain(then.iter().map(|section| section.profile))
                .enumerate()
                .map(|(i, r)| (O::Section(u32::try_from(i).unwrap_or(u32::MAX)), r))
                .collect(),
            Node::Sweep {
                profile,
                path,
                stations: _,
                v_degree: _,
            } => vec![(O::Profile, *profile), (O::Path, *path)],
            Node::Fillet {
                radius: _,
                selection,
            }
            | Node::Chamfer {
                distance: _,
                selection,
            } => vec![(O::Selection, *selection)],
            Node::Shell { thickness: _, open } => vec![(O::Open, *open)],
            Node::Measure { primitive } => {
                let verb = primitive.kind();
                let [a, b] = primitive.refs();
                vec![(O::Measured(verb, 0), *a), (O::Measured(verb, 1), *b)]
            }
            Node::Split { target, tool } => vec![(O::Target, target.read), (O::Tool, tool.read)],
            Node::Mirror { body, plane } => vec![(O::Body, body.read), (O::Plane, plane.read)],
            Node::Subtract {
                from,
                tool,
            } => vec![(O::From, from.read), (O::Cut, tool.read)],
            // In LIST ORDER: the order is the fold's (D9).
            Node::Union {
                members,
            }
            | Node::Intersect {
                members,
            }
            | Node::Assemble {
                members,
            } => members.rows().into_iter().map(|(o, r)| (o, r.read)).collect(),
        }
    }

    /// [`Self::operand_rows`], writable, in the same order: the reads a
    /// re-point rewrites.
    pub(crate) fn operand_rows_mut(&mut self) -> Vec<(crate::OperandSlot, &mut VarId)>
    where
        P: crate::ProfilePayload,
    {
        use crate::OperandSlot as O;
        // Every field named, as `operand_rows` names it: a field added
        // to a node does not compile until it is stated here too, so
        // the writable twin cannot fall behind the reading one.
        match self {
            Node::InstantiatePart {
                doc_ref: _,
                interface: _,
                target,
                constraints,
            } => {
                let mut v = vec![(O::Space, target)];
                v.extend(side_rows_mut(constraints));
                v
            }
            Node::Place {
                source,
                target,
                constraints,
            } => {
                let mut v: Vec<(O, &mut VarId)> = vec![(O::Source, &mut source.read)];
                v.push((O::Space, target));
                v.extend(side_rows_mut(constraints));
                v
            }
            Node::World
            | Node::Space
            | Node::Assertion {
                claim: crate::measure::Claim::Value { .. },
            } => Vec::new(),
            Node::Assertion {
                claim: crate::measure::Claim::Clearance { a, b, .. },
            } => vec![(O::Cleared(0), a), (O::Cleared(1), b)],
            Node::Profile(p) => p.read_rows_mut(),
            Node::Extrude {
                profile,
                distance: _,
            } => vec![(O::Profile, profile)],
            Node::Revolve {
                profile,
                axis_through: _,
                axis_heading: _,
                angle: _,
            } => vec![(O::Profile, profile)],
            Node::Tube { .. } | Node::HollowTube { .. } => Vec::new(),
            Node::Loft {
                first,
                then,
                v_degree: _,
            } => core::iter::once(first)
                .chain(then.iter_mut().map(|section| &mut section.profile))
                .enumerate()
                .map(|(i, r)| (O::Section(u32::try_from(i).unwrap_or(u32::MAX)), r))
                .collect(),
            Node::Sweep {
                profile,
                path,
                stations: _,
                v_degree: _,
            } => vec![(O::Profile, profile), (O::Path, path)],
            Node::Fillet {
                radius: _,
                selection,
            }
            | Node::Chamfer {
                distance: _,
                selection,
            } => vec![(O::Selection, selection)],
            Node::Shell { thickness: _, open } => vec![(O::Open, open)],
            Node::Measure { primitive } => {
                let verb = primitive.kind();
                let [a, b] = primitive.refs_mut();
                vec![(O::Measured(verb, 0), a), (O::Measured(verb, 1), b)]
            }
            Node::Split { target, tool } => {
                vec![(O::Target, &mut target.read), (O::Tool, &mut tool.read)]
            }
            Node::Mirror { body, plane } => {
                vec![(O::Body, &mut body.read), (O::Plane, &mut plane.read)]
            }
            Node::Subtract { from, tool } => {
                vec![(O::From, &mut from.read), (O::Cut, &mut tool.read)]
            }
            Node::Union { members } | Node::Intersect { members } | Node::Assemble { members } => {
                members
                    .rows_mut()
                    .into_iter()
                    .map(|(o, r)| (o, &mut r.read))
                    .collect()
            }
        }
    }

    /// **E10, stated once**: whether this assertion's bound is
    /// dimensioned differently from its value. `None` for every node
    /// that is not a [`Node::Assertion`], and for one reading a variable
    /// `doc` does not hold (the read walks' fault).
    ///
    /// The edit door renders the answer as
    /// [`crate::EditError::AssertionDimension`] and the load door as
    /// `SnapshotError::AssertionBound`: a refusal names the door it came
    /// from, and the rule is asked in one place so the two cannot drift.
    pub(crate) fn assertion_bound_fault(
        &self,
        doc: &crate::doc::Doc<P>,
    ) -> Option<AssertionBoundFault> {
        let dim = |var: &VarId| doc.vars.get(var)?.kind().dimension();
        match self {
            Node::Assertion {
                claim: crate::measure::Claim::Value { value, bound, .. },
            } => AssertionBoundFault::against(dim(value)?, dim(bound)?),
            // A clearance is a length, so each bound is one.
            Node::Assertion {
                claim: crate::measure::Claim::Clearance { bounds, .. },
            } => bounds
                .exprs()
                .into_iter()
                .find_map(|b| AssertionBoundFault::against(Dimension::Length, dim(b)?)),
            _ => None,
        }
    }

    /// **What is wrong with this node's indexed reads**, if anything:
    /// an index at a family argument, or a read carrying other than its
    /// family's rank of indices.
    ///
    /// ONE definition with two callers: the edit doors' per-node checks
    /// and the load door's `validate_document`, each rendering it in its
    /// own vocabulary. A repeated read is not a fault (REFERENCES DM5:
    /// the operation answers it), a list's length is evaluation's, and a
    /// selection's designation form is the selection's own
    /// ([`crate::SelectionFault`]).
    pub fn input_fault(&self, doc: &crate::doc::Doc<P>) -> Option<InputFault>
    where
        P: crate::ProfilePayload,
    {
        // An indexed read carries its family's rank of indices, and a
        // family argument none.
        if let Node::Union { members, .. } | Node::Intersect { members, .. } = self
            && let crate::Bodies::Family(read) = members
            && read.is_member()
        {
            return Some(InputFault::IndexedFamily);
        }
        self.body_reads().into_iter().find_map(|(seat, read)| {
            if !read.is_member() {
                return None;
            }
            let rank = doc.var_reach(read.read).chain().map_or(0, <[VarId]>::len);
            (read.at.len() != rank).then_some(InputFault::IndexRank {
                seat,
                found: read.at.len(),
                rank,
            })
        })
    }

    /// Rewrites every payload reference EXACTLY equal to `from` into
    /// `to`, returning how many it rewrote — `Rebind`'s reach into the
    /// names a node still carries (an instance's crossings); a
    /// selection's names are the document's
    /// ([`crate::Doc::rewrite_selection_names`]).
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
        /// A FACE-typed payload name rewritten through `map`, or
        /// `None` where the map leaves it — the re-derivation an
        /// instance's crossing `outer`s use.
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
}

impl<P, L> Node<P, crate::expr::ExprTree<L>>
where
    P: crate::program::SlotPayload<crate::expr::ExprTree<L>>,
    L: crate::expr::LeafSet,
    crate::expr::ExprTree<L>: Slot,
{
    /// **The slot-dimension rule, asked of an authored node**: every
    /// formula the slot table pairs with a slot carries the dimension
    /// [`SlotId::dimension`] fixes for that address — the edit doors'
    /// question before they lower the node.
    pub(crate) fn formula_dimension_fault(&self) -> Option<SlotDimensionFault> {
        self.rows()
            .into_iter()
            .find_map(|(slot, expr)| slot.dimension_fault(expr))
    }
}

/// `f` over each of `a`'s values, in order: the arrays a slot table
/// holds, in another slot form.
pub(crate) fn map_array<S, S2, E, const N: usize>(
    a: &[S; N],
    f: &mut impl FnMut(&S) -> Result<S2, E>,
) -> Result<[S2; N], E> {
    let mut out = Vec::with_capacity(N);
    for value in a {
        out.push(f(value)?);
    }
    match out.try_into() {
        Ok(mapped) => Ok(mapped),
        Err(_) => unreachable!("{N} values in, {N} out"),
    }
}

impl<S> TubeWindow<S> {
    /// **This window in another slot form** ([`Node::try_map_slots`]'s
    /// rule).
    ///
    /// # Errors
    ///
    /// `f`'s first.
    pub fn try_map_slots<S2, E>(
        &self,
        f: &mut impl FnMut(&S) -> Result<S2, E>,
    ) -> Result<TubeWindow<S2>, E> {
        Ok(match self {
            TubeWindow::Full => TubeWindow::Full,
            TubeWindow::Arc { t0, t1 } => TubeWindow::Arc {
                t0: f(t0)?,
                t1: f(t1)?,
            },
        })
    }
}

/// **One port of an operation's output signature** (D10, Operations):
/// the variable the operation defines there ([`crate::VarDef::Output`]),
/// named and typed by the variant.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OutputPort {
    /// The port's name, unique in its signature.
    pub name: &'static str,
    /// What decides its kind.
    pub kind: PortKind,
}

/// **What decides a port's kind**: the variant alone, or, for a placer
/// of its operand's shape, the operand.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PortKind {
    /// This kind.
    Of(crate::VarKind),
    /// A [`Node::Place`]'s value, which has its one shape's shape
    /// (`Bodies` over a `Bodies`, `Body` otherwise), and is `Bodies`
    /// over several (their copies, keyed by read).
    ///
    /// Read off the operand once, at the insert that mints the output;
    /// from then on the output's stored kind is the fact (VR3: a kind
    /// is fixed at minting), and the load door only checks the two
    /// still agree. Re-pointing the operand at the other shape refuses
    /// at the slot door.
    Placed,
}

impl OutputPort {
    const fn of(name: &'static str, kind: crate::VarKind) -> Self {
        Self {
            name,
            kind: PortKind::Of(kind),
        }
    }
}

impl<P, S: Slot> Node<P, S> {
    /// **The operation's output signature** (D10, Operations): the
    /// fixed list of named, typed ports its variant defines, in port
    /// order, possibly none. The insert door mints one variable per
    /// port ([`crate::Doc::output`]).
    ///
    /// A [`Node::Place`]'s one port has its one shape's shape: `Bodies`
    /// over a `Bodies`, `Body` otherwise ([`PortKind::Placed`]). An
    /// instance defines one `copy`, a `Body`: the copy of its part's
    /// product. No port count depends on a list.
    ///
    /// Exhaustive over the variants, so one added does not compile
    /// until it states what it defines.
    #[must_use]
    pub fn outputs(&self) -> Vec<OutputPort> {
        use crate::VarKind;
        const BODY: OutputPort = OutputPort::of("body", VarKind::Body);
        const SPACE: OutputPort = OutputPort::of("space", VarKind::Space);
        match self {
            Self::Profile(_) => vec![OutputPort::of("profile", VarKind::Profile)],
            // A construction's body is born in a space of its own,
            // its last port (D10 `[variables.kinds]`: `a.space`).
            Self::Revolve { .. } => vec![BODY, OutputPort::of("axis", VarKind::Axis), SPACE],
            Self::Extrude { .. }
            | Self::Tube { .. }
            | Self::HollowTube { .. }
            | Self::Loft { .. }
            | Self::Sweep { .. } => vec![BODY, SPACE],
            Self::World | Self::Space => vec![SPACE],
            Self::Split { .. } => vec![
                OutputPort::of("above", VarKind::Body),
                OutputPort::of("below", VarKind::Body),
            ],
            // One copy, of its source's shape.
            Self::Place { .. } => vec![OutputPort {
                name: "copy",
                kind: PortKind::Placed,
            }],
            // One copy of the part's product.
            Self::InstantiatePart { .. } => vec![OutputPort::of("copy", VarKind::Body)],
            Self::Measure { primitive } => {
                vec![OutputPort::of("value", VarKind::from(primitive.dim()))]
            }
            Self::Fillet { .. }
            | Self::Chamfer { .. }
            | Self::Shell { .. }
            | Self::Mirror { .. }
            | Self::Subtract { .. }
            | Self::Union { .. }
            | Self::Intersect { .. }
            | Self::Assemble { .. } => vec![BODY],
            Self::Assertion { .. } => Vec::new(),
        }
    }
}

impl<P, S: Slot> Node<P, S> {
    /// **This node in another slot form** — the one conversion between
    /// the authored and the stored node: every slot and every payload
    /// expression rewritten by `f`, every operand by `read` (handed its
    /// address), the profile's payload by `payload` (handed both for
    /// its own slots and reads), and every other field kept. Lowering is this with the
    /// door's lowerings; re-authoring is this with
    /// [`crate::Formula::from`] and [`crate::Operand::Var`].
    ///
    /// # Errors
    ///
    /// The first refusal of `f`, `read` or `payload`.
    #[allow(clippy::type_complexity)]
    pub fn try_map_slots<P2, S2: Slot, E>(
        &self,
        payload: impl FnOnce(
            &P,
            &mut dyn FnMut(&S) -> Result<S2, E>,
            &mut dyn FnMut(crate::OperandSlot, &S::Read) -> Result<S2::Read, E>,
        ) -> Result<P2, E>,
        f: &mut impl FnMut(&S) -> Result<S2, E>,
        read: &mut impl FnMut(crate::OperandSlot, &S::Read) -> Result<S2::Read, E>,
    ) -> Result<Node<P2, S2>, E> {
        use crate::OperandSlot as O;
        Ok(match self {
            Node::Profile(p) => Node::Profile(payload(p, f, read)?),
            Node::Extrude { profile, distance } => Node::Extrude {
                profile: read(O::Profile, profile)?,
                distance: f(distance)?,
            },
            Node::Revolve {
                profile,
                axis_through,
                axis_heading,
                angle,
            } => Node::Revolve {
                profile: read(O::Profile, profile)?,
                axis_through: map_array(axis_through, f)?,
                axis_heading: f(axis_heading)?,
                angle: f(angle)?,
            },
            Node::Tube {
                major_radius,
                window,
                minor_radius,
            } => Node::Tube {
                major_radius: f(major_radius)?,
                window: window.try_map_slots(f)?,
                minor_radius: f(minor_radius)?,
            },
            Node::HollowTube {
                major_radius,
                window,
                minor_radius,
                wall,
            } => Node::HollowTube {
                major_radius: f(major_radius)?,
                window: window.try_map_slots(f)?,
                minor_radius: f(minor_radius)?,
                wall: f(wall)?,
            },
            Node::Loft {
                first,
                then,
                v_degree,
            } => Node::Loft {
                first: read(O::Section(0), first)?,
                then: then
                    .iter()
                    .enumerate()
                    .map(|(k, section)| {
                        section.try_map(
                            |r| read(O::Section(u32::try_from(k + 1).unwrap_or(u32::MAX)), r),
                            f,
                        )
                    })
                    .collect::<Result<_, E>>()?,
                v_degree: f(v_degree)?,
            },
            Node::Sweep {
                profile,
                path,
                stations,
                v_degree,
            } => Node::Sweep {
                profile: read(O::Profile, profile)?,
                path: read(O::Path, path)?,
                stations: f(stations)?,
                v_degree: f(v_degree)?,
            },
            Node::Fillet { radius, selection } => Node::Fillet {
                radius: f(radius)?,
                selection: read(O::Selection, selection)?,
            },
            Node::Chamfer {
                distance,
                selection,
            } => Node::Chamfer {
                distance: f(distance)?,
                selection: read(O::Selection, selection)?,
            },
            Node::Shell { thickness, open } => Node::Shell {
                thickness: f(thickness)?,
                open: read(O::Open, open)?,
            },
            Node::Split { target, tool } => Node::Split {
                target: target.try_map(O::Target, f, read)?,
                tool: tool.try_map(|tool| read(O::Tool, tool))?,
            },
            Node::Mirror { body, plane } => Node::Mirror {
                body: body.try_map(O::Body, f, read)?,
                plane: plane.try_map(|plane| read(O::Plane, plane))?,
            },
            Node::Subtract { from, tool } => Node::Subtract {
                from: from.try_map(O::From, f, read)?,
                tool: tool.try_map(O::Cut, f, read)?,
            },
            Node::Union { members } => Node::Union {
                members: members.try_map(|o, r| r.try_map(o, f, read))?,
            },
            Node::Intersect { members } => Node::Intersect {
                members: members.try_map(|o, r| r.try_map(o, f, read))?,
            },
            Node::Assemble { members } => Node::Assemble {
                members: members.try_map(|o, r| r.try_map(o, f, read))?,
            },
            Node::Place {
                source,
                target,
                constraints,
            } => Node::Place {
                source: source.try_map(O::Source, f, read)?,
                target: read(O::Space, target)?,
                constraints: map_constraints(constraints, f, read)?,
            },
            Node::World => Node::World,
            Node::Space => Node::Space,
            Node::InstantiatePart {
                doc_ref,
                interface,
                target,
                constraints,
            } => Node::InstantiatePart {
                doc_ref: *doc_ref,
                interface: interface.clone(),
                target: read(O::Space, target)?,
                constraints: map_constraints(constraints, f, read)?,
            },
            Node::Measure { primitive } => {
                let verb = primitive.kind();
                let mut i = 0u8;
                Node::Measure {
                    primitive: primitive.try_map(|r| {
                        let slot = O::Measured(verb, i);
                        i += 1;
                        read(slot, r)
                    })?,
                }
            }
            Node::Assertion { claim } => Node::Assertion {
                claim: claim.try_map(&mut *f, &mut *read)?,
            },
        })
    }
}

impl<P, S: Slot> Node<P, S> {
    /// **The reads at this node's body seats** (REFERENCES DM3), each
    /// at its address, in field order: the seats a read may index
    /// ([`crate::BodyRead`]), and no other operand.
    pub fn body_reads(&self) -> Vec<(crate::OperandSlot, &crate::BodyRead<S>)> {
        use crate::OperandSlot as O;
        match self {
            Node::Split { target, .. } => vec![(O::Target, target)],
            Node::Mirror { body, .. } => vec![(O::Body, body)],
            Node::Subtract { from, tool, .. } => vec![(O::From, from), (O::Cut, tool)],
            Node::Union { members, .. }
            | Node::Intersect { members, .. }
            | Node::Assemble { members, .. } => members.rows(),
            Node::Place { source, .. } => vec![(O::Source, source)],
            Node::Profile(_)
            | Node::Extrude { .. }
            | Node::Revolve { .. }
            | Node::Tube { .. }
            | Node::HollowTube { .. }
            | Node::Loft { .. }
            | Node::Sweep { .. }
            | Node::Fillet { .. }
            | Node::Chamfer { .. }
            | Node::Shell { .. }
            | Node::InstantiatePart { .. }
            | Node::World
            | Node::Space
            | Node::Measure { .. }
            | Node::Assertion { .. } => Vec::new(),
        }
    }

    /// The read at body seat `seat`, `None` where the node has no such
    /// body seat.
    pub fn body_read(&self, seat: crate::OperandSlot) -> Option<&crate::BodyRead<S>> {
        find_row(self.body_reads(), seat)
    }

    /// [`Self::body_read`], exclusive.
    pub fn body_read_mut(&mut self, seat: crate::OperandSlot) -> Option<&mut crate::BodyRead<S>> {
        use crate::OperandSlot as O;
        match (self, seat) {
            (Node::Split { target, .. }, O::Target) => Some(target),
            (Node::Mirror { body, .. }, O::Body) => Some(body),
            (Node::Subtract { from, .. }, O::From) => Some(from),
            (Node::Subtract { tool, .. }, O::Cut) => Some(tool),
            (
                Node::Union { members, .. }
                | Node::Intersect { members, .. }
                | Node::Assemble { members, .. },
                _,
            ) => members
                .rows_mut()
                .into_iter()
                .find_map(|(at, read)| (at == seat).then_some(read)),
            (Node::Place { source, .. }, O::Source) => Some(source),
            _ => None,
        }
    }

    /// **The kinds the variable read at operand `seat` may have**: the
    /// seat's own ([`crate::OperandSlot::kind`]), except at a body seat
    /// whose read is indexed, where the variable read is the family
    /// ([`crate::BodyRead::kind`]). Every door that writes or loads a
    /// read asks this.
    pub fn seat_kind(&self, seat: crate::OperandSlot) -> crate::SlotKind {
        if let Some(role) = self.seat_role(seat) {
            return crate::SlotKind::Is(role.of());
        }
        self.body_read(seat)
            .map_or_else(|| seat.kind(), |read| read.kind(seat))
    }

    /// **The first seat whose read's role yields a kind the seat does
    /// not admit** (a frame's x axis at a split's tool), with the kind it
    /// yields: what both doors refuse as the seat's kind fault.
    #[must_use]
    pub fn seat_role_fault(&self) -> Option<(crate::OperandSlot, crate::VarKind)> {
        self.role_seats().into_iter().find_map(|(seat, role)| {
            let role = role?;
            crate::pose::role_kind(role.of(), role, seat.kind())
                .is_none()
                .then_some((seat, role.kind()))
        })
    }

    /// **The role the read at `seat` reads through** (D10), `None` for
    /// a plain read or a seat that reads no pose: a split's tool or a
    /// ring's axis read off a frame.
    #[must_use]
    pub fn seat_role(&self, seat: crate::OperandSlot) -> Option<crate::pose::PoseRole> {
        self.role_seats()
            .into_iter()
            .find_map(|(at, role)| (at == seat).then_some(role).flatten())
    }

    /// **Every seat whose read carries a role**, with the role it reads
    /// through: the one list both [`Self::seat_role`] and
    /// [`Self::seat_role_fault`] read.
    fn role_seats(&self) -> Vec<(crate::OperandSlot, Option<crate::pose::PoseRole>)> {
        use crate::OperandSlot as O;
        match self {
            Node::Split { tool, .. } => vec![(O::Tool, tool.role)],
            Node::Mirror { plane, .. } => vec![(O::Plane, plane.role)],
            _ => Vec::new(),
        }
    }

    /// The role of the read at `seat`, writable: what a re-point of
    /// that read clears, since the read it writes is replaced whole.
    pub(crate) fn seat_role_mut(
        &mut self,
        seat: crate::OperandSlot,
    ) -> Option<&mut Option<crate::pose::PoseRole>> {
        use crate::OperandSlot as O;
        match (self, seat) {
            (Node::Split { tool, .. }, O::Tool) => Some(&mut tool.role),
            (Node::Mirror { plane, .. }, O::Plane) => Some(&mut plane.role),
            _ => None,
        }
    }
}

/// **A node as an edit carries it**: its slots hold authored formulas,
/// and its profile, if it is one, the authored program. The edit door
/// lowers it to the [`Node`] a document stores.
pub type AuthoredNode<P = crate::ProfileProgram> =
    Node<<P as crate::ProfilePayload>::Authored, crate::Formula>;

impl<P: crate::ProfilePayload> Node<P> {
    /// **This node re-authored**: every slot a formula reading its
    /// variable by id ([`crate::Formula::var`]), at the dimension the
    /// slot's address reads it at, its profile's payload re-authored
    /// ([`crate::ProfilePayload::authored`]). The edit door lowers it
    /// back to this node, bit for bit: a lone variable lowers to
    /// itself, so every unchanged argument keeps its variable and
    /// that variable's distribution.
    ///
    /// `doc` answers what no address fixes: an assertion's value and
    /// bound are read at their variables' dimensions.
    #[must_use]
    pub fn authored(&self, doc: &crate::Doc<P>) -> Node<P::Authored, crate::Formula> {
        self.authored_with(doc, &mut crate::Formula::var, &mut |var| {
            crate::Operand::Var(var)
        })
    }

    /// **This node as it was written**: each slot the formula its
    /// variable was written as ([`crate::Doc::written`]) — an anonymous
    /// variable's value or definition, a named one's reader. Inserting
    /// it mints its anonymous variables afresh, as the insert that
    /// wrote it did. RE-MINTS: where identity must survive, re-author
    /// with [`Self::authored`].
    ///
    /// A document rebuilt by re-inserting its nodes in order is the
    /// document, ids included, ONLY where no anonymous variable is read
    /// more than once or carries a distribution
    /// ([`crate::Doc::written_would_not_reproduce`]) and none was
    /// value-edited after its insert (its mint read the old value). A
    /// caller rebuilding a document checks the first and compares the
    /// ids for the second. An anonymous selection or pose an operand
    /// reads is written as the definition it is
    /// ([`crate::Operand::Select`], [`crate::Operand::Pose`]).
    #[must_use]
    pub fn written(&self, doc: &crate::Doc<P>) -> Node<P::Authored, crate::Formula> {
        self.authored_with(
            doc,
            &mut |var, dim| crate::Formula::from(doc.written(&crate::Expr::var(var, dim))),
            &mut |var| doc.written_operand(var),
        )
    }

    /// [`Self::authored`], each slot's variable written by `reader`
    /// (handed the variable and the dimension the slot reads it at),
    /// and each operand's read by `operand`. A profile's arguments are
    /// written by the same `reader`.
    pub(crate) fn authored_with(
        &self,
        doc: &crate::Doc<P>,
        reader: &mut dyn FnMut(VarId, Dimension) -> crate::Formula,
        operand: &mut dyn FnMut(VarId) -> crate::Operand,
    ) -> Node<P::Authored, crate::Formula> {
        let mut dims: BTreeMap<VarId, Dimension> = self
            .rows()
            .into_iter()
            .map(|(slot, &var)| (var, slot.expr_dimension()))
            .collect();
        dims.extend(self.payload_reads(doc).into_iter().flatten());
        let mut payload = None;
        if let Node::Profile(p) = self {
            payload = Some(p.authored_with(reader, operand));
        }
        let mut read = |var: &VarId| {
            let Some(&dim) = dims.get(var) else {
                unreachable!(
                    "every slot of a stored node has an address or an assertion's \
                     variable to read {var:?} at"
                )
            };
            Ok::<_, core::convert::Infallible>(reader(*var, dim))
        };
        let reauthored = self.try_map_slots(
            |_, _, _| {
                Ok(payload
                    .take()
                    .unwrap_or_else(|| unreachable!("a profile's payload is re-authored once")))
            },
            &mut read,
            &mut |_, var| Ok(operand(*var)),
        );
        let Ok(authored) = reauthored;
        authored
    }
}

impl<P: crate::ProfilePayload> Node<P> {
    /// **The variables the expressions no slot addresses read**, each
    /// with the dimension it is read at ([`payload_exprs`]'s order): an
    /// assertion's value and bound at their variables' kinds — or,
    /// where `doc` no longer holds one, at the other's — and a
    /// clearance's bounds as lengths. `None` for a slot-only node.
    pub(crate) fn payload_reads(&self, doc: &crate::Doc<P>) -> Option<Vec<(VarId, Dimension)>> {
        let Node::Assertion { claim } = self else {
            return None;
        };
        Some(claim_reads(doc, claim))
    }
}

/// [`Node::payload_reads`] of an assertion's claim.
pub(crate) fn claim_reads<P>(
    doc: &crate::Doc<P>,
    claim: &crate::measure::Claim,
) -> Vec<(VarId, Dimension)> {
    match claim {
        crate::measure::Claim::Value { value, bound, .. } => {
            let dim = |var: &VarId| doc.vars.get(var).and_then(|v| v.kind().dimension());
            let (v, b) = (dim(value), dim(bound));
            let either = v.or(b).unwrap_or(Dimension::Scalar);
            vec![(*value, v.unwrap_or(either)), (*bound, b.unwrap_or(either))]
        }
        crate::measure::Claim::Clearance { bounds, .. } => bounds
            .exprs()
            .into_iter()
            .map(|&var| (var, Dimension::Length))
            .collect(),
    }
}

/// The construction doors, in either slot form: a node built from
/// authored formulas is what an edit carries, and the door lowers it.
impl<P, S: Slot> Node<P, S> {
    /// The [`StableName`]s this payload REFERENCES — an instance's
    /// interface crossings' `outer`s; a
    /// name read through a slot is a selection's
    /// ([`crate::VarDef::Select`]), not a payload's. Document data, never DAG
    /// edges ([`crate::Doc::upstream`] excludes them): the edit door checks at
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
            // An instance's interface record: each crossing's `outer`,
            // in record order.
            //
            // An `outer` is a REMAINDER name — it denotes a face in
            // THIS document, on a node this document can delete and
            // under a name this document can rebind — so it is a
            // payload name, and the same FOUR
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

    /// **The nodes a payload's references are READ AT that are not
    /// also DAG inputs** — none of today's payloads has one.
    ///
    /// The insert door checks these are live exactly as it checks a
    /// payload name's head, and for the same reason: a never-existed
    /// id is a typo, and a later delete stranding one is N5's
    /// dangling case, refused at evaluation rather than at the edit.
    pub fn payload_read_sites(&self) -> Vec<RecipeNodeId> {
        match self {
            // EXHAUSTIVE, with no wildcard, so a new [`Node`] variant
            // is classified here or does not compile. The name-free
            // variants reference nothing that could have a read site
            // ([`name_free_node`]), and an instance's interface record
            // holds no node id at all ([`InterfaceCrossing::Mate`]
            // argues why): a crossing's `outer` is a payload name,
            // checked as one by the reading twin's list, and its
            // `inner` is spelled in the part's id space.
            name_free_node!() | Node::InstantiatePart { .. } => Vec::new(),
        }
    }

    /// The node ids [`Node::payload_names`] reaches: the heads whose
    /// existence the insert door checks.
    pub fn named_nodes(&self) -> Vec<RecipeNodeId> {
        self.payload_names().iter().map(|name| name.node).collect()
    }

    /// **Where name `reference` of the selection this node reads at
    /// `slot` sits, as a person reads it**: the node's own noun and the
    /// place — `("fillet", "edge 2")`, `("frame", "face")` — for a
    /// refusal that says a name that stopped resolving by where it sits
    /// rather than by its words. `None` for a seat that reads no
    /// selection.
    pub(crate) fn selection_slot(
        &self,
        slot: crate::OperandSlot,
        reference: usize,
    ) -> Option<(&'static str, String)> {
        use crate::OperandSlot as O;
        let owner = match (self, slot) {
            (Node::Fillet { .. }, O::Selection) => "fillet",
            (Node::Chamfer { .. }, O::Selection) => "chamfer",
            (Node::Shell { .. }, O::Open) => "shell",
            (Node::Measure { .. }, O::Measured(..)) => "measure",
            (Node::Assertion { .. }, O::Cleared(..)) => "clearance",
            (Node::Place { .. } | Node::InstantiatePart { .. }, O::Side { .. }) => "constraint",
            _ => return None,
        };
        Some((owner, slot.selection_entry(reference)?))
    }

    /// **Whether this node is a space and nothing else** (the world, or
    /// a `new_space()`): it holds nothing, so nothing evaluates it.
    #[must_use]
    pub fn is_space(&self) -> bool {
        matches!(self, Node::World | Node::Space)
    }

    /// Builds a [`Node::InstantiatePart`] with the EMPTY interface
    /// record, placed into `target` by `constraints`, which pin it
    /// there ([`crate::mate::pin::Placed::bundle`]). A mate reads poses
    /// off geometry and there is no part frame to read (FORK-S3M), so
    /// no constructor can pin an instance where its part's world is;
    /// the façade's "place part" writes the mates.
    pub fn instantiate_part(
        doc_ref: crate::ident::DocRef,
        target: impl Into<S::Read>,
        constraints: Vec<crate::mate::Constraint<S>>,
    ) -> Self {
        Self::instantiate_part_with(doc_ref, InterfaceRecord::default(), target, constraints)
    }

    /// Builds a [`Node::InstantiatePart`] carrying a SEAM record
    /// (ASM-R2b D-4): the split's door, since only a split knows what
    /// crossed its cut.
    pub fn instantiate_part_with(
        doc_ref: crate::ident::DocRef,
        interface: InterfaceRecord,
        target: impl Into<S::Read>,
        constraints: Vec<crate::mate::Constraint<S>>,
    ) -> Self {
        Node::InstantiatePart {
            doc_ref,
            interface,
            target: target.into(),
            constraints,
        }
    }

    /// Builds a [`Node::Place`] copying `source` into `target` under
    /// `constraints`, the whole bundle: a value naming a mate of it
    /// names the mate's id as written, which the insert mints
    /// ([`crate::mate::pin::Placed::bundle`]).
    pub fn place(
        source: impl Into<crate::BodyRead<S>>,
        target: impl Into<S::Read>,
        constraints: Vec<crate::mate::Constraint<S>>,
    ) -> Self {
        Node::Place {
            source: source.into(),
            target: target.into(),
            constraints,
        }
    }
}

impl<P> Node<P, crate::Formula> {
    /// A [`Node::Measure`] over `primitive`, each reference authored as
    /// the read it lowers to ([`crate::Operand`]: a body, or a sited name
    /// as the selection of it in the body it is read at).
    pub fn measure<R: Clone + Into<crate::Operand>>(
        primitive: &crate::measure::MeasurePrimitive<R>,
    ) -> Self {
        Node::Measure {
            primitive: primitive.map(|r| r.clone().into()),
        }
    }

    /// **An assertion that `clearance(a, b)` lies within `bounds`**, each
    /// seat authored as the read it lowers to: a body (all of its
    /// faces), or a sited face name as the selection of it.
    pub fn clearance(
        a: impl Into<crate::Operand>,
        b: impl Into<crate::Operand>,
        bounds: crate::measure::ClearanceBounds<crate::Formula>,
    ) -> Self {
        Node::Assertion {
            claim: crate::measure::Claim::Clearance {
                a: a.into(),
                b: b.into(),
                bounds,
            },
        }
    }

    /// **The names this authored node's selection seats author**
    /// ([`crate::Operand::Select`]): what the edit door mints selections
    /// of, in seat order.
    pub fn selected_names(&self) -> Vec<&StableName>
    where
        P: crate::program::SlotPayload<crate::Formula>,
    {
        let seats: Vec<&crate::Operand> = match self {
            Node::Fillet { selection, .. } | Node::Chamfer { selection, .. } => vec![selection],
            Node::Shell { open, .. } => vec![open],
            Node::Measure { primitive } => primitive.refs().to_vec(),
            Node::Assertion {
                claim: crate::measure::Claim::Clearance { a, b, .. },
            } => vec![a, b],
            // A drawn profile's traced faces.
            Node::Profile(p) => p.read_rows().into_iter().map(|(_, read)| read).collect(),
            Node::Extrude { .. }
            | Node::Revolve { .. }
            | Node::Tube { .. }
            | Node::HollowTube { .. }
            | Node::Loft { .. }
            | Node::Sweep { .. }
            | Node::Split { .. }
            | Node::Mirror { .. }
            | Node::Subtract { .. }
            | Node::Union { .. }
            | Node::Intersect { .. }
            | Node::Assemble { .. }
            | Node::World
            | Node::Space
            | Node::Place { .. }
            | Node::InstantiatePart { .. }
            | Node::Assertion {
                claim: crate::measure::Claim::Value { .. },
            } => Vec::new(),
        };
        seats
            .into_iter()
            .flat_map(crate::Operand::selected_names)
            .collect()
    }

    /// Builds a [`Node::Fillet`] over the selection of `selection` in
    /// the body `target` reads, the names in their stored order (sorted,
    /// deduplicated, [`crate::var::Select::canonical`]), so a recipe's
    /// bits do not depend on the order a user clicked in. The door mints
    /// the selection.
    pub fn fillet(
        target: impl Into<crate::Operand>,
        radius: crate::Formula,
        selection: Vec<StableName>,
    ) -> Self {
        Node::Fillet {
            radius,
            selection: edges_of(target, selection),
        }
    }

    /// Builds a [`Node::Chamfer`] over the selection of `selection` in
    /// the body `target` reads, as [`Node::fillet`] does.
    pub fn chamfer(
        target: impl Into<crate::Operand>,
        distance: crate::Formula,
        selection: Vec<StableName>,
    ) -> Self {
        Node::Chamfer {
            distance,
            selection: edges_of(target, selection),
        }
    }

    /// Builds a [`Node::Shell`] over the selection of `open` in the body
    /// `target` reads, in designation order with each name's first
    /// occurrence kept ([`crate::var::Select::canonical`]): the first
    /// designated face of a chart carries the rim's identity, so the
    /// order is authored data.
    pub fn shell(
        target: impl Into<crate::Operand>,
        thickness: crate::Formula,
        open: Vec<StableName>,
    ) -> Self {
        Node::Shell {
            thickness,
            open: crate::Operand::select(
                target,
                crate::var::Select::canonical(crate::VarKind::Faces, open),
            ),
        }
    }

    /// Builds a [`Node::Mirror`]: the body `body` reads, reflected across
    /// the plane `plane` reads. A symmetric part is the union of a body
    /// and its mirror across one of its own faces.
    pub fn mirror(
        body: impl Into<crate::Operand>,
        plane: impl Into<crate::pose::RoleRead<crate::Operand>>,
    ) -> Self {
        Node::Mirror {
            body: body.into().into(),
            plane: plane.into(),
        }
    }
}

/// The `Edges` selection of `names` in the body `target` reads, in
/// stored order.
fn edges_of(target: impl Into<crate::Operand>, names: Vec<StableName>) -> crate::Operand {
    crate::Operand::select(
        target,
        crate::var::Select::canonical(crate::VarKind::Edges, names),
    )
}

impl<P, S> Node<P, S>
where
    P: PartialEq + crate::program::SlotPayload<S>,
    S: Slot,
{
    /// Bit-semantic payload equality (spec D7's comparison substrate):
    /// `PartialEq` for structure plus BIT comparison of every slot
    /// expression's float literals — `0.0` vs `-0.0` differ here. The
    /// opaque profile payload `P` is compared by its own `PartialEq`
    /// (its float semantics are PR 2's contract when `P` is
    /// instantiated).
    pub fn bit_eq(&self, other: &Self) -> bool {
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
        // A constraint's own scalars are float payload no slot
        // addresses, and the `PartialEq` above compares them by value:
        // they compare here, by bits.
        if let (
            Node::Place { constraints: a, .. } | Node::InstantiatePart { constraints: a, .. },
            Node::Place { constraints: b, .. } | Node::InstantiatePart { constraints: b, .. },
        ) = (self, other)
            && (a.len() != b.len() || !a.iter().zip(b).all(|(x, y)| x.bit_eq(y)))
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

/// **A bundle's pose reads**, each at its seat
/// ([`crate::OperandSlot::Side`]), in bundle order, `on` before a pose
/// `to` (a world read reads no variable).
fn side_rows<S: Slot>(
    constraints: &[crate::mate::Constraint<S>],
) -> Vec<(crate::OperandSlot, S::Read)>
where
    S::Read: Clone,
{
    let mut out = Vec::new();
    for c in constraints {
        let seat = |side| crate::OperandSlot::Side {
            constraint: c.id,
            side,
        };
        let Some(mate) = c.mate() else {
            continue;
        };
        out.push((seat(crate::mate::MateSide::On), mate.on.clone()));
        out.push((seat(crate::mate::MateSide::To), mate.to.clone()));
    }
    out
}

/// [`side_rows`], writable, in the same order.
fn side_rows_mut<S: Slot>(
    constraints: &mut [crate::mate::Constraint<S>],
) -> Vec<(crate::OperandSlot, &mut S::Read)> {
    let mut out = Vec::new();
    for c in constraints {
        let id = c.id;
        let seat = |side| crate::OperandSlot::Side {
            constraint: id,
            side,
        };
        let Some(crate::mate::Mate { on, to, .. }) = c.mate_mut() else {
            continue;
        };
        out.push((seat(crate::mate::MateSide::On), on));
        out.push((seat(crate::mate::MateSide::To), to));
    }
    out
}

/// **A bundle in another slot form**: each constraint's face reads by
/// `read`, at its seat, and its offsets' steps by `f`, its id kept.
fn map_constraints<S: Slot, S2: Slot, E>(
    constraints: &[crate::mate::Constraint<S>],
    f: &mut impl FnMut(&S) -> Result<S2, E>,
    read: &mut impl FnMut(crate::OperandSlot, &S::Read) -> Result<S2::Read, E>,
) -> Result<Vec<crate::mate::Constraint<S2>>, E> {
    constraints
        .iter()
        .map(|c| {
            let id = c.id;
            c.try_map(
                |side, r| {
                    read(
                        crate::OperandSlot::Side {
                            constraint: id,
                            side,
                        },
                        r,
                    )
                },
                f,
            )
        })
        .collect()
}
