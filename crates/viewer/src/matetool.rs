//! **The mate tool**: a modal layer-3 tool that turns two face picks
//! into one committed mate and the values it names (GUI-4; the plan's
//! ruled addition to G3).
//!
//! # Shape
//!
//! The tool holds **two sequential face picks in tool state** — the
//! GUI-2 single-select vocabulary ([`FaceSelection`]) consumed twice —
//! then a choice of kind, flip and values ([`MateChoice`]), then
//! ONE committed action: the bundle of the copy it places written whole
//! with the mate and its values, the copy re-targeted into the space the
//! `to` pick is in where it was placed elsewhere. Everything
//! before the commit is tool state: it never enters the document, never
//! enters any history, and dies with the session (G1's transient-state
//! rule).
//!
//! # What the picks become
//!
//! A mate is two poses of one kind (D10, FORK-S3M), and the kind is
//! the primitive: two planes rest, two axes are coaxial. The first pick
//! is `on`, read off the copy the mate places; the second is `to`, read
//! where the person picked it, in the space the copy joins. Each is the
//! picked face read as a pose of the chosen kind — its plane with its
//! outward normal, or its carrier's axis ([`MATE_KINDS`]) — and the
//! choice's flip turns `to` round: a plain plane mate equates outward
//! normals, so two faces resting against each other carry the flip.
//!
//! A pick on a `Place`'s copy authors `on` as the face of the shape the
//! placement copies ([`crate::world::body_of`]); a pick on an instance
//! authors it as a selection of the instance's own output; a pick
//! anywhere else is read through A11's member walk
//! ([`pncad::document::member_of`]) to the instance it places. The
//! kernel's edit door decides what is left: an `on` that does not read
//! the copy (`MateFault::NotOnTheCopy`), two kinds that differ. Whether
//! a face has the plane or the axis the kind reads is the placement's
//! own refusal where it evaluates, never a pre-check here.
//!
//! # What the mate leaves
//!
//! Values set the freedoms the mate's kind leaves, on the target
//! space's axes carried onto the mated direction by the least turn from
//! where the mate's orientation puts the copy's: a plane mate leaves two
//! slides and the spin about the target's own direction, an axis mate one slide
//! and the spin. A value the kind does not leave is refused here
//! ([`MateToolError::NotAFreedom`], [`MateFreedom::offered_beside`]), before
//! any edit exists. The mate goes in through the bundle door, which
//! rewrites the placement's bundle whole; the gesture then writes the
//! orientation nearest where the copy stands and the values that stand
//! it where the snap puts it, so a freshly mated copy moves only by the
//! snap, and the values the person named over them.
//!
//! # Survival
//!
//! Tool state survives a picked reference vanishing (GQ7's recorded
//! constraint, the GUI-2 semantics): [`MateTool::reconcile`] re-reads
//! each held pick against the landed pair and **degrades one step per
//! lost pick**, typed — a lost second pick returns the tool to its
//! one-pick step with the first held; a lost first pick with a live
//! second keeps the second as the held pick, which is then the `on`
//! pick. No crash, no silent clear: every drop is a [`MateToolEvent`]
//! the chrome renders.
//!
//! This tool is the one modal tool NOT built on [`crate::seats`]: its
//! picks are faces, and its survivor is promoted to the first pick,
//! where a seated tool's seats are roles a node is never moved between.
//!
//! **`reconcile` is the consumer's obligation.** The tool is a value
//! beside the session, so nothing drives the survival step
//! automatically — the application calls it once per frame
//! (`sync_scene`); a headless consumer that forgets sees a stale pick
//! refused at the edit door, typed but late.
//!
//! Module kind: **vocabulary** — it names no driver type and no
//! `app`-only crate (`crates/viewer/README.md`, Module boundaries).

use pncad::document::{
    Coord, Doc, Evaluation, Formula, Mate, MateSide, Node, Operand, ProfileProgram, RecipeNodeId,
    Said, Say, SitedRef, SlotKind, Speaker, SpokenNode, TargetAxis, VarKind, member_of_ref, pose,
};
use pncad::prelude::StableName;
use pncad::select::{EntityKind, Resolution};

use crate::pickindex::PickKinds;
use crate::session::select::resolves;
use crate::session::{EdgeSelection, FaceSelection, SessionOp, VertexSelection};
use crate::tree::{DrawnBody, HeldPick, PickedEntity};
use crate::vocab::{partial_mirror, vocabulary};

/// **The kinds a mate's picks are read as**, with their labels — each
/// read off ONE pick per side ([`MateReading::of`]): a plane off a
/// face, an axis off a face's carrier or an edge's line, a point off a
/// vertex or a carrier's or a circle's centre, a direction off a face's
/// normal or an edge's line. The kind is the mate's primitive — two
/// planes rest, two axes are coaxial, two points are a ball, two
/// directions are parallel.
///
/// **Deliberately partial**: the `partial_mirror!` invocation below
/// forces every [`VarKind`] variant to be offered at a seat of this
/// list or named as deliberately absent, with the reason.
pub const MATE_KINDS: [(VarKind, &str); 4] = [
    (VarKind::Plane, "plane"),
    (VarKind::Axis, "axis"),
    (VarKind::Point, "point"),
    (VarKind::Direction, "direction"),
];

/// **What the cursor may pick while the mate tool reads `kind`**: the
/// faces alone for a plane, which reads nothing else (so a narrow
/// shelf or a small hole's wall stays pickable however near its own
/// boundary the cursor is), vertices beating edges beating faces for a
/// point, and the bare cursor's rule for the rest. A pick already held
/// that the kind chosen since does not read is refused at the commit,
/// typed ([`MateToolError::KindDoesNotRead`]).
#[must_use]
pub fn pick_kinds(kind: VarKind) -> PickKinds {
    match kind {
        VarKind::Plane => PickKinds::FacesOnly,
        VarKind::Point => PickKinds::WithVertices,
        _ => PickKinds::Any,
    }
}

/// **A mate tool pick**: the face, edge or vertex picked in the
/// viewport, by the selection vocabulary it was picked in. What it
/// reads as is its name's entity kind ([`MatePick::entity`]).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MatePick {
    /// A face pick.
    Face(FaceSelection),
    /// An edge pick.
    Edge(EdgeSelection),
    /// A vertex pick.
    Vertex(VertexSelection),
}

impl MatePick {
    /// The entity kind the pick's name names — what the pose is read
    /// as, whichever vocabulary carried it.
    #[must_use]
    pub fn entity(&self) -> EntityKind {
        self.name().kind
    }

    /// The node whose drawn body was picked.
    #[must_use]
    pub fn node(&self) -> RecipeNodeId {
        self.on().node()
    }

    /// The picked face, when the pick is one — what the viewport marks
    /// as a held face.
    #[must_use]
    pub fn face(&self) -> Option<&FaceSelection> {
        match self {
            Self::Face(face) => Some(face),
            Self::Edge(_) | Self::Vertex(_) => None,
        }
    }
}

impl PickedEntity for MatePick {
    fn name(&self) -> &StableName {
        match self {
            Self::Face(face) => &face.name,
            Self::Edge(edge) => &edge.name,
            Self::Vertex(vertex) => &vertex.name,
        }
    }

    fn on(&self) -> DrawnBody {
        match self {
            Self::Face(face) => face.on(),
            Self::Edge(edge) => edge.on(),
            Self::Vertex(vertex) => vertex.on(),
        }
    }
}

impl From<FaceSelection> for MatePick {
    fn from(face: FaceSelection) -> Self {
        Self::Face(face)
    }
}

impl From<EdgeSelection> for MatePick {
    fn from(edge: EdgeSelection) -> Self {
        Self::Edge(edge)
    }
}

impl From<VertexSelection> for MatePick {
    fn from(vertex: VertexSelection) -> Self {
        Self::Vertex(vertex)
    }
}

/// **How a pick of one entity kind is read as a pose of a mate's
/// kind**: the pose definition its one read sits in, and the role a
/// direction is read through. A plane, an axis and a point are the
/// pose itself; a direction is a face's plane read through its
/// normal, or an edge's axis read through its direction — the kernel's
/// own spelling of a direction mate's side.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MateReading {
    /// The pose kind the pick is read as: [`VarKind::Plane`],
    /// [`VarKind::Axis`] or [`VarKind::Point`].
    pub pose: VarKind,
    /// The role a direction is read through, `None` for the pose
    /// itself.
    pub role: Option<pose::PoseRole>,
}

impl MateReading {
    /// **The reading of a pick of `entity` for a mate of `kind`**,
    /// `None` where none reads it. Whether a pose's read admits the
    /// entity is the kernel's seat ([`SlotKind::selection_kind`] of the
    /// pose's own read), never a table here.
    #[must_use]
    pub fn of(kind: VarKind, entity: EntityKind) -> Option<Self> {
        let reading = |pose, role| Self { pose, role };
        let candidates: &[Self] = match kind {
            VarKind::Plane | VarKind::Axis | VarKind::Point => &[reading(kind, None)],
            VarKind::Direction => &[
                reading(VarKind::Plane, Some(pose::PoseRole::Normal)),
                reading(VarKind::Axis, Some(pose::PoseRole::Direction)),
            ],
            _ => &[],
        };
        candidates
            .iter()
            .copied()
            .find(|reading| reading.seat().selection_kind(entity).is_some())
    }

    /// The pose definition this reading puts `read` in.
    fn def(self, read: Operand) -> pose::PoseDef<Operand, Formula> {
        match self.pose {
            VarKind::Plane => pose::PoseDef::Plane { face: read },
            VarKind::Axis => pose::PoseDef::Axis { of: read },
            _ => pose::PoseDef::Point { of: read },
        }
    }

    /// The seat the pose's one read sits at, as the kernel states it.
    fn seat(self) -> SlotKind {
        match self
            .def(Operand::Var(pncad::document::VarId::WORLD))
            .reads()
            .first()
        {
            Some(&(_, _, _, seat)) => seat,
            None => unreachable!("a plane, an axis and a point each read one entity"),
        }
    }

    /// **`read` as a mate's side**, turned round where `flip`: the pose
    /// itself (its [`pose::PoseDef::Flip`] where flipped), or a
    /// direction read through its role. A direction's side is the
    /// flip of the role read — the kernel's spelling, since no pose
    /// reads a role plainly — so both sides carry one flip, and its
    /// sense is turned by flipping the pose it is read off.
    fn side(self, read: Operand, flip: bool) -> Operand {
        let turned = |pose: Operand| {
            Operand::Pose(Box::new(pose::PoseDef::Flip {
                pose: pose::RoleRead::plain(pose),
            }))
        };
        let pose = Operand::Pose(Box::new(self.def(read)));
        let pose = if flip { turned(pose) } else { pose };
        match self.role {
            None => pose,
            Some(role) => Operand::Pose(Box::new(pose::PoseDef::Flip {
                pose: pose::RoleRead::through(pose, role),
            })),
        }
    }
}

/// **The entity kinds a mate of `kind` reads**, in the order a reader
/// lists them ([`MateReading::of`]).
#[must_use]
pub fn reads(kind: VarKind) -> Vec<EntityKind> {
    [EntityKind::Face, EntityKind::Edge, EntityKind::Vertex]
        .into_iter()
        .filter(|&entity| MateReading::of(kind, entity).is_some())
        .collect()
}

/// **Whether a mate of `kind` has a sense to flip**: the kernel's
/// sensed poses ([`pose::PoseAdmits::Sensed`]). A point has none.
#[must_use]
pub fn sensed(kind: VarKind) -> bool {
    pose::PoseAdmits::Sensed.kinds().contains(&kind)
}

/// The entity kinds `kinds`, said as a list: `a face, an edge or a
/// vertex`.
pub(crate) fn said_entities(kinds: &[EntityKind]) -> String {
    let word = |kind: &EntityKind| match kind {
        EntityKind::Edge => "an edge",
        EntityKind::Vertex => "a vertex",
        EntityKind::Body => "a body",
        EntityKind::Face => "a face",
    };
    match kinds {
        [] => "nothing".to_owned(),
        [one] => word(one).to_owned(),
        [init @ .., last] => format!(
            "{} or {}",
            init.iter().map(word).collect::<Vec<_>>().join(", "),
            word(last)
        ),
    }
}

vocabulary! {
    /// **A freedom a mate's value can set**, as the mate panel offers
    /// it: a slide along the chart's (carried) `x`, `y` or `z`, the spin
    /// about the target's own direction beside a mate fixing one, or a
    /// link of the chain of spins about the target space's own axes
    /// beside a mate fixing none (a point). Which of them a mate leaves
    /// is the kernel's ([`Self::left_by`]), and the panel offers those
    /// alone. It mirrors the kernel's [`Coord`] both ways, by two
    /// exhaustive matches ([`Self::coord`], [`Self::of`]), so a freedom
    /// the kernel grows does not build here until the panel offers it.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum MateFreedom {
        /// A slide along the chart's `x`.
        SlideX,
        /// A slide along the chart's `y`.
        SlideY,
        /// A slide along the chart's `z`.
        SlideZ,
        /// The spin about the target's own direction.
        Spin,
        /// A spin about the target space's `x`, a link of a chain.
        SpinX,
        /// A spin about the target space's `y`, a link of a chain.
        SpinY,
        /// A spin about the target space's `z`, a link of a chain.
        SpinZ,
    }

    /// Every freedom, in the order the mate panel draws them.
    pub const ALL;
}

impl MateFreedom {
    /// The kernel's freedom.
    #[must_use]
    pub fn coord(self) -> Coord {
        match self {
            Self::SlideX => Coord::Slide(TargetAxis::X),
            Self::SlideY => Coord::Slide(TargetAxis::Y),
            Self::SlideZ => Coord::Slide(TargetAxis::Z),
            Self::Spin => Coord::SPIN,
            Self::SpinX => Coord::Spin(Some(TargetAxis::X)),
            Self::SpinY => Coord::Spin(Some(TargetAxis::Y)),
            Self::SpinZ => Coord::Spin(Some(TargetAxis::Z)),
        }
    }

    /// The panel's freedom for the kernel's.
    #[must_use]
    pub fn of(coord: Coord) -> Self {
        match coord {
            Coord::Slide(TargetAxis::X) => Self::SlideX,
            Coord::Slide(TargetAxis::Y) => Self::SlideY,
            Coord::Slide(TargetAxis::Z) => Self::SlideZ,
            Coord::Spin(None) => Self::Spin,
            Coord::Spin(Some(TargetAxis::X)) => Self::SpinX,
            Coord::Spin(Some(TargetAxis::Y)) => Self::SpinY,
            Coord::Spin(Some(TargetAxis::Z)) => Self::SpinZ,
        }
    }

    /// **Whether a lone mate of `kind` leaves this freedom**: the
    /// kernel's answer for the kind's own residual
    /// ([`pncad::document::SubgroupFamily::leaves`]). A plane, an axis
    /// and a direction leave slides and the spin; a point leaves the
    /// chain of spins. Which slides is the copy's geometry's to say (the
    /// slide the least turn carries onto the mated direction is the one
    /// a plane fixes and an axis leaves), so each is offered, and the
    /// evaluation refuses slides that do not span what the mate leaves.
    #[must_use]
    pub fn left_by(self, kind: VarKind) -> bool {
        kind.symmetry()
            .is_some_and(|family| family.leaves(self.coord()))
    }

    /// **How many slides a mate of `kind` leaves**: two beside a plane,
    /// one beside an axis, three beside a direction, none beside a
    /// point.
    #[must_use]
    pub fn slides_left_by(kind: VarKind) -> usize {
        kind.symmetry()
            .map_or(0, pncad::document::SubgroupFamily::slides)
    }
}

partial_mirror! {
    VarKind, labelled MATE_KINDS,
    offered [Plane, Axis, Point, Direction],
    absent [
        Length => "a number, never a pose",
        Angle => "a number, never a pose",
        Scalar => "a number, never a pose",
        Count => "a number, never a pose",
        Frame => "a frame is read through an axis and a point, two picks a \
                  side, which the one-pick-a-side tool does not make",
        Body => "a value, never a pose",
        Bodies => "a value, never a pose",
        Profile => "a value, never a pose",
        Face => "a pick, which the pose is read off",
        Edge => "a pick, which the pose is read off",
        Vertex => "a pick, which the pose is read off",
        Faces => "a reference, never a pose",
        Edges => "a reference, never a pose",
        Space => "a placement's target, never a pose: a mate reads geometry",
    ],
}

/// A typed mate-tool refusal (closed enum, D4 ¶3).
#[derive(Debug)]
pub enum MateToolError {
    /// The tool does not hold two picks yet.
    NotTwoPicks,
    /// The pick names an entity no pose of the chosen kind is read
    /// off ([`MateReading::of`]): an edge or a vertex held for a plane,
    /// a vertex for an axis or a direction — a pick taken under one
    /// kind and committed under another — or a name of no picked
    /// entity at all.
    KindDoesNotRead {
        /// Which pick.
        side: MateSide,
        /// The kind chosen.
        kind: VarKind,
        /// The entity the pick names.
        entity: EntityKind,
    },
    /// The choice turns round a mate whose kind has no sense
    /// ([`sensed`]): a point.
    NoSense {
        /// The kind chosen.
        kind: VarKind,
    },
    /// A pick is on a node the document no longer holds: the tool was
    /// asked without the survival step ([`MateTool::reconcile`]) having
    /// dropped it.
    PickGone {
        /// Which pick.
        side: MateSide,
        /// The node, by the label the tool kept.
        node: SpokenNode,
    },
    /// The `on` pick is on no placement's copy: neither a `Place` nor an
    /// instance, nor anything A11's member walk reads down to an
    /// instance ([`pncad::document::member_of`]) — a pair boolean's
    /// body, a split half. A mate places a copy, so there is no bundle
    /// to add it to.
    NotACopyPick {
        /// The node the pick's body belongs to, as the document held it.
        node: SpokenNode,
    },
    /// Both picks are on the copy the mate places: a mate relates the
    /// copy to what it joins, never to itself.
    SamePick {
        /// The placement both picks read, as the document held it.
        head: SpokenNode,
    },
    /// The chosen kind is none the tool reads off one pick a side
    /// ([`MATE_KINDS`]).
    KindNotOffered {
        /// The kind asked for.
        kind: VarKind,
    },
    /// A value sets a freedom the mate's kind does not leave, or a slide
    /// past as many as it leaves ([`MateFreedom::left_by`],
    /// [`MateFreedom::slides_left_by`]).
    NotAFreedom {
        /// The freedom.
        coord: Coord,
        /// The mate's kind.
        kind: VarKind,
    },
}

impl core::fmt::Display for MateToolError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::NotTwoPicks => write!(f, "needs two picks"),
            Self::KindDoesNotRead { side, kind, entity } => {
                let word = kind.to_string();
                let article = if word.starts_with(['a', 'e', 'i', 'o', 'u']) {
                    "an"
                } else {
                    "a"
                };
                write!(
                    f,
                    "the {} pick is {}, and {article} {word} mate reads {}",
                    side.name(),
                    said_entities(&[*entity]),
                    said_entities(&reads(*kind)),
                )
            }
            Self::NoSense { kind } => write!(f, "a {kind} has no sense to turn round"),
            Self::PickGone { side, node } => write!(
                f,
                "the {} pick is on {node}, which the document no longer holds",
                side.name()
            ),
            Self::NotACopyPick { node } => write!(
                f,
                "the on pick is on {node}, which is no placement's copy; a mate places a copy"
            ),
            Self::SamePick { head } => write!(
                f,
                "both picks are on {head}, the copy the mate places; a mate relates it to \
                 something else"
            ),
            Self::KindNotOffered { kind } => write!(
                f,
                "the tool reads a {} off one pick a side, not a {kind}",
                MATE_KINDS
                    .iter()
                    .map(|(_, label)| *label)
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            Self::NotAFreedom { coord, kind } => {
                write!(f, "a {} mate leaves no {} to set", kind, coord.name())
            }
        }
    }
}

impl core::error::Error for MateToolError {}

impl MateToolError {
    /// This refusal with its nodes spoken from `doc`, a later version of
    /// the document the tool read ([`crate::session::Refusal::respoken`]):
    /// the panel's line ([`MateTool::line`]) speaks the landed document
    /// ([`crate::frame::landed_speaker`]), as the whole chrome does, and
    /// a refusal drawn beside it speaks the same one.
    #[must_use]
    pub fn respoken(self, doc: &Doc<ProfileProgram>) -> Self {
        match self {
            Self::PickGone { side, node } => Self::PickGone {
                side,
                node: node.respoken(doc),
            },
            Self::NotACopyPick { node } => Self::NotACopyPick {
                node: node.respoken(doc),
            },
            Self::SamePick { head } => Self::SamePick {
                head: head.respoken(doc),
            },
            unspoken @ (Self::NotTwoPicks
            | Self::KindDoesNotRead { .. }
            | Self::NoSense { .. }
            | Self::KindNotOffered { .. }
            | Self::NotAFreedom { .. }) => unspoken,
        }
    }
}

/// What the tool holds: none, one, or two picks — the two sequential
/// picks of the ruling, as a value.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum MateToolState {
    /// No pick yet.
    #[default]
    Idle,
    /// The first pick, held: the copy's.
    One(HeldPick<MatePick>),
    /// Both picks, held; the choice is offered.
    Two {
        /// The `on` pick: a face, edge or vertex of the copy the mate
        /// places.
        on: HeldPick<MatePick>,
        /// The `to` pick: a face, edge or vertex of what the copy joins.
        to: HeldPick<MatePick>,
    },
}

impl MateToolState {
    /// The held picks, `on` then `to`, `None` for a side not yet picked
    /// — what the panel's line says and the viewport marks
    /// ([`HeldPick::pick`]).
    pub fn picks(&self) -> [Option<&HeldPick<MatePick>>; 2] {
        match self {
            Self::Idle => [None, None],
            Self::One(on) => [Some(on), None],
            Self::Two { on, to } => [Some(on), Some(to)],
        }
    }
}

/// **The sentence a dropped pick is said in**: its role, and the
/// pick as its panel line said it ([`HeldPick`]) — the mate tool's drop
/// and the mirror tool's (`crate::combine::MirrorEvent`).
pub(crate) fn face_lost<P: PickedEntity>(
    f: &mut core::fmt::Formatter<'_>,
    role: impl core::fmt::Display,
    pick: &HeldPick<P>,
    by: Speaker<'_>,
) -> core::fmt::Result {
    write!(
        f,
        "the {role} pick ({}) no longer resolves; the tool dropped it",
        Said(pick, by)
    )
}

/// A typed tool event the chrome renders — every state change that
/// was not the direct echo of an op.
#[derive(Debug)]
pub enum MateToolEvent {
    /// A held pick's reference no longer resolves against the landed
    /// evaluation; the tool degraded one step, dropping it.
    PickLost {
        /// Which pick was dropped.
        side: MateSide,
        /// The pick that was held, with the body it was said on and the
        /// last labels the tool kept for its nodes ([`MateTool::respeak`]).
        pick: HeldPick<MatePick>,
        /// The resolution machinery's own verdict, read as the feature
        /// tree reads it ([`crate::tree::resolution_as_drawn`]); boxed
        /// for the same width reason `Standing` boxes it.
        resolution: Box<Resolution>,
    },
}

/// A sentence for the status line — the payload stays typed and full
/// in the value; this is what a person reads. A node `by` does not hold
/// is said as the tool kept it.
impl Say for MateToolEvent {
    fn say(&self, f: &mut core::fmt::Formatter<'_>, by: Speaker<'_>) -> core::fmt::Result {
        match self {
            Self::PickLost { side, pick, .. } => face_lost(f, side.name(), pick, by),
        }
    }
}

/// The sentence where no document is at hand: each node as the tool
/// kept it.
impl core::fmt::Display for MateToolEvent {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::PickLost { pick, .. } => self.say(f, Speaker::held(pick.said())),
        }
    }
}

/// The person's choice — what the chrome's controls select between
/// picks and commit.
#[derive(Debug, Clone, PartialEq)]
pub struct MateChoice {
    /// The kind both picks are read as ([`MATE_KINDS`]): the mate's
    /// primitive.
    pub kind: VarKind,
    /// Whether `to` is read in its opposite sense: two faces resting
    /// against each other face opposite ways, so a rest carries it. A
    /// kind with no sense ([`sensed`]) refuses it.
    pub flip: bool,
    /// The freedoms the mate leaves that the person sets, each to its
    /// number: a `Length` for a slide, an `Angle` for the spin.
    pub values: Vec<(Coord, Formula)>,
}

/// The derived, ready-to-commit mate: the placement it joins, the two
/// sides, the mate and its values.
#[derive(Debug, Clone)]
pub struct MateProposal {
    /// The placement whose bundle the mate joins: the `on` pick's copy.
    pub placement: RecipeNodeId,
    /// The `on` entity: read off the copy's shapes.
    pub on: SitedRef,
    /// The `to` entity: read where it was picked.
    pub to: SitedRef,
    /// The mate.
    pub mate: Mate<Formula>,
    /// Its values, written naming the mate in the same bundle.
    pub values: Vec<(Coord, Formula)>,
}

impl MateProposal {
    /// **The one committed action**: the session op that writes the
    /// mate and its values through the ordinary commit door. A mate
    /// whose `to` reads outside the copy's target space is refused there,
    /// naming both spaces ([`Self::op_into_target_space`] moves it).
    pub fn op(&self) -> SessionOp {
        self.committed(false)
    }

    /// [`Self::op`], the copy moved by name into the space the mate's
    /// `to` reads where that is not its target — the chrome's explicit
    /// "move the copy" choice, never a side effect of the mate.
    pub fn op_into_target_space(&self) -> SessionOp {
        self.committed(true)
    }

    fn committed(&self, into_target_space: bool) -> SessionOp {
        SessionOp::AddMate {
            placement: self.placement,
            mate: Box::new(self.mate.clone()),
            values: self.values.clone(),
            into_target_space,
        }
    }
}

/// **A face read as its plane**: `face`, a one-name face selection,
/// read plainly as the plane of that face — a plane mate's side, and
/// the plane a mirror reflects across (`crate::combine::mirror_node`).
pub(crate) fn plane_of(face: pncad::document::Operand) -> pncad::document::Operand {
    pncad::document::Operand::Pose(Box::new(pose::PoseDef::Plane { face }))
}

/// The placement whose copy a pick at `at` is on: `at` itself when it
/// is a `Place` or an instance, the instance A11's member walk reaches
/// otherwise.
fn placement_of(
    doc: &Doc<ProfileProgram>,
    at: RecipeNodeId,
    read: &SitedRef,
) -> Option<RecipeNodeId> {
    match doc.node(at) {
        Some(Node::Place { .. } | Node::InstantiatePart { .. }) => Some(at),
        _ => member_of_ref(doc, read).map(|member| member.instance),
    }
}

/// The modal mate tool. A value: the chrome holds one while the tool
/// is active, a test constructs one and drives the same methods.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct MateTool {
    /// The held picks, each with the nodes its words name as the
    /// document last spoke them ([`MateTool::respeak`]) — what a drop
    /// notice, a refusal or the panel line says once the document no
    /// longer holds a node.
    state: MateToolState,
}

impl MateTool {
    /// A tool holding nothing.
    pub fn new() -> Self {
        Self::default()
    }

    /// The held picks.
    pub fn state(&self) -> &MateToolState {
        &self.state
    }

    /// Feed one pick — a face, an edge or a vertex in the GUI-2
    /// selection vocabulary, consumed into tool state. The first pick
    /// fills `on`, the second `to`; a third REPLACES `to` (the choice
    /// step is still open, and re-picking the second is how a person
    /// corrects it). The pick is held as every pick is
    /// ([`HeldPick::new`]); what it reads as is the commit's question
    /// ([`MateTool::proposal`]).
    pub fn pick(&mut self, doc: &Doc<ProfileProgram>, pick: impl Into<MatePick>) {
        let held = HeldPick::new(doc, pick.into());
        self.state = match std::mem::take(&mut self.state) {
            MateToolState::Idle => MateToolState::One(held),
            MateToolState::One(on) | MateToolState::Two { on, .. } => {
                MateToolState::Two { on, to: held }
            }
        };
    }

    /// **The held picks' nodes, spoken again from `doc`**, the shown
    /// document after an operation ([`SpokenNode::respoken`]'s rule): a
    /// node `doc` holds takes its label now, and one it no longer holds
    /// keeps the last it had.
    pub fn respeak(&mut self, doc: &Doc<ProfileProgram>) {
        let again = |held: &mut HeldPick<MatePick>| *held = held.respoken(doc);
        match &mut self.state {
            MateToolState::Idle => {}
            MateToolState::One(on) => again(on),
            MateToolState::Two { on, to } => {
                again(on);
                again(to);
            }
        }
    }

    /// **The line the mate panel shows for its held picks** — the
    /// seated tools' line (`seats::picks_line`), with the mate's two
    /// sides as its roles and each pick said as the chrome says one
    /// ([`HeldPick`]) by `by`, a node `by` does not hold as the tool
    /// kept it.
    pub fn line(&self, by: Speaker<'_>) -> String {
        let [on, to] = self.state.picks();
        crate::seats::picks_line(
            [(MateSide::On, on), (MateSide::To, to)]
                .map(|(side, pick)| (side.name(), pick.map(|pick| Said(pick, by).to_string()))),
        )
    }

    /// **A held pick's node as a refusal says it**: by the label the
    /// tool kept, re-spoken from the shown document after every
    /// operation; as `doc` holds it only when nothing named was kept.
    fn spoken(
        doc: &Doc<ProfileProgram>,
        pick: &HeldPick<MatePick>,
        node: RecipeNodeId,
    ) -> SpokenNode {
        match pick.said().spoken(node) {
            kept if kept.kind().is_some() => kept,
            _ => doc.spoken(node),
        }
    }

    /// Re-read the held picks against the landed pair, degrading one
    /// step per pick whose reference no longer resolves (module docs:
    /// the survival semantics). Returns the typed drops.
    pub fn reconcile(
        &mut self,
        doc: &Doc<ProfileProgram>,
        eval: &Evaluation<f64>,
    ) -> Vec<MateToolEvent> {
        let mut events = Vec::new();
        let mut lost = |side: MateSide, pick: &HeldPick<MatePick>| -> bool {
            let held = pick.pick();
            let on = held.on();
            let verdict =
                crate::tree::pick_resolution(doc, eval, on.node(), on.body(), held.name());
            if resolves(&verdict) {
                false
            } else {
                events.push(MateToolEvent::PickLost {
                    side,
                    pick: pick.clone(),
                    resolution: Box::new(verdict),
                });
                true
            }
        };
        self.state = match std::mem::take(&mut self.state) {
            MateToolState::Idle => MateToolState::Idle,
            MateToolState::One(on) => {
                if lost(MateSide::On, &on) {
                    MateToolState::Idle
                } else {
                    MateToolState::One(on)
                }
            }
            MateToolState::Two { on, to } => {
                match (lost(MateSide::On, &on), lost(MateSide::To, &to)) {
                    (false, false) => MateToolState::Two { on, to },
                    (false, true) => MateToolState::One(on),
                    (true, false) => MateToolState::One(to),
                    (true, true) => MateToolState::Idle,
                }
            }
        };
        events
    }

    /// Derive the committed action from the two held picks and the
    /// person's choice: `on` read off the first pick's copy, `to` where
    /// the second was picked, each as a pose of the chosen kind
    /// ([`MateReading`]), `to` turned round as chosen, and the values
    /// the kind leaves.
    ///
    /// `doc` is the document the picks were taken on (the session's
    /// landed one): the copy each pick is on is read from it, and a node
    /// is said by the label the tool kept ([`MateTool::respeak`]).
    ///
    /// # Errors
    ///
    /// Every arm of [`MateToolError`] — see each arm's docs.
    pub fn proposal(
        &self,
        doc: &Doc<ProfileProgram>,
        choice: MateChoice,
    ) -> Result<MateProposal, MateToolError> {
        let MateToolState::Two {
            on: held_on,
            to: held_to,
        } = &self.state
        else {
            return Err(MateToolError::NotTwoPicks);
        };
        // The choice's doors FIRST, before any pick is read: the kind
        // among those the tool reads, its sense, each value against the
        // freedoms the kind leaves.
        if !MATE_KINDS.iter().any(|(kind, _)| *kind == choice.kind) {
            return Err(MateToolError::KindNotOffered { kind: choice.kind });
        }
        if choice.flip && !sensed(choice.kind) {
            return Err(MateToolError::NoSense { kind: choice.kind });
        }
        let mut slides = 0;
        for (coord, _) in &choice.values {
            if matches!(coord, Coord::Slide(_)) {
                slides += 1;
            }
            if !MateFreedom::of(*coord).left_by(choice.kind)
                || slides > MateFreedom::slides_left_by(choice.kind)
            {
                return Err(MateToolError::NotAFreedom {
                    coord: *coord,
                    kind: choice.kind,
                });
            }
        }
        let reading = |side: MateSide, held: &HeldPick<MatePick>| {
            let entity = held.pick().entity();
            MateReading::of(choice.kind, entity).ok_or(MateToolError::KindDoesNotRead {
                side,
                kind: choice.kind,
                entity,
            })
        };
        let read_on = reading(MateSide::On, held_on)?;
        let read_to = reading(MateSide::To, held_to)?;
        let (on, to) = (held_on.pick(), held_to.pick());
        for (side, held) in [(MateSide::On, held_on), (MateSide::To, held_to)] {
            let node = held.pick().node();
            if doc.node(node).is_none() {
                return Err(MateToolError::PickGone {
                    side,
                    node: Self::spoken(doc, held, node),
                });
            }
        }
        let picked_on = SitedRef::new(on.node(), on.name().clone());
        let picked_to = SitedRef::new(to.node(), to.name().clone());
        let placement = placement_of(doc, on.node(), &picked_on).ok_or_else(|| {
            MateToolError::NotACopyPick {
                node: Self::spoken(doc, held_on, on.node()),
            }
        })?;
        if placement_of(doc, to.node(), &picked_to) == Some(placement) {
            let head = if placement == on.node() {
                Self::spoken(doc, held_on, on.node())
            } else {
                doc.spoken(placement)
            };
            return Err(MateToolError::SamePick { head });
        }
        // A `Place`'s copy is read off the shape it copies, by the
        // shape's own name for the entity (a copy carries its body's
        // names), on the output it copies where the shape's operation
        // defines several (a split's halves).
        let (on_read, on_operand) = match doc.node(on.node()) {
            Some(Node::Place { .. }) => {
                let (at, port) = crate::world::body_of(doc, on.node(), on.on().body());
                let body = match crate::world::seat_read(doc, at, Some(port)) {
                    (at, Some(port)) => Operand::output(at, port),
                    (at, None) => Operand::Node(at),
                };
                let read = Operand::select(body, vec![on.name().clone()]);
                (SitedRef::new(at, on.name().clone()), read)
            }
            _ => (picked_on.clone(), picked_on.into()),
        };
        Ok(MateProposal {
            placement,
            mate: Mate {
                on: read_on.side(on_operand, false),
                to: read_to.side(picked_to.clone().into(), choice.flip),
                // The commit writes the orientation nearest where the
                // copy stands ([`SessionOp::AddMate`]).
                orient: pncad::document::Orientation::IDENTITY,
            },
            on: on_read,
            to: picked_to,
            values: choice.values,
        })
    }
}
