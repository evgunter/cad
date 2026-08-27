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

impl SlotId {
    /// The dimension an expression in this slot must have (checked by
    /// `apply` on insert and on every expression edit, spec D6).
    pub fn dimension(self) -> Dimension {
        match self {
            Self::Origin(_)
            | Self::Distance
            | Self::Radius
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
    /// The op is [`sweep::fillet::build::fillet_edges`] over the
    /// resolved selection; anything outside its two assembly front
    /// doors is a typed refusal
    /// ([`crate::eval::NodeErrorKind::Fillet`]), never a silent
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
            Node::Extrude { profile, .. } => vec![*profile],
            Node::Revolve { profile, axis, .. } => vec![*profile, *axis],
            Node::Loft { profiles, .. } => profiles.clone(),
            Node::Sweep { profile, path, .. } => vec![*profile, *path],
            Node::Fillet { target, .. } => vec![*target],
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
            Node::Extrude { .. } => vec![SlotId::Distance],
            Node::Fillet { .. } => vec![SlotId::Radius],
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
                | Node::Split { .. }
                | Node::Boolean { .. }
                | Node::Transform { .. }
                | Node::Declare { .. }
                | Node::InstantiatePart { .. }
                | Node::Mate { .. },
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
                | Node::Split { .. }
                | Node::Boolean { .. }
                | Node::Transform { .. }
                | Node::Declare { .. }
                | Node::InstantiatePart { .. }
                | Node::Mate { .. },
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
            Node::Fillet { selection, .. } => selection.iter().collect(),
            // A12: a mate's two heads are the instance-qualified
            // references its reading edges are recomputed from.
            Node::Mate { a, b, .. } => vec![a, b],
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
            Node::Fillet { selection, .. } => {
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
            | Node::Split { .. }
            | Node::Boolean { .. }
            | Node::Transform { .. }
            | Node::Declare { .. }
            | Node::InstantiatePart { .. }
            | Node::Mate { .. } => return None,
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
