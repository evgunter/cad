//! **The placement editor**: a modal layer-3 tool whose draft is one
//! placement's whole bundle — its mates, and the values beneath them on
//! the freedoms they leave — committed as one edit (GUI-4; the plan's
//! ruled addition to G3).
//!
//! # Two doors
//!
//! The editor opens on a body or on a placement ([`PlaceDoor`]):
//!
//! - **Create**: a body not yet placed. Apply inserts one placement of
//!   it holding the whole bundle ([`SessionOp::Place`]).
//! - **Edit**: a placement, opened from its tree row, its constraint
//!   rows, or a pick on its copy. Apply writes its bundle whole
//!   ([`SessionOp::SetBundle`]), keeping the id of every row loaded
//!   from it.
//!
//! Either Apply is one undo step, and nothing enters the document
//! before it (G1's transient-state rule): every pick, kind, sense and
//! number is tool state and dies with the session. A draft over a
//! bundle the document has since changed is stale and reloads, said
//! ([`MateToolEvent::Reloaded`]), as the profile editor's does.
//!
//! # The draft
//!
//! A **mate row** ([`MateRow`]) holds a kind (any of [`MATE_KINDS`]), a
//! sense ([`senses`]) and its seats, and may be struck out. The **value
//! rows** are derived, never held: exactly the freedoms the live mates
//! leave, by the kernel's own rule ([`Placed::freedoms`]), each prefilled
//! where the copy stands snapped onto the mates (`session/snap.rs`), its
//! number read back through the chart that will place it
//! ([`editor_core::mate::read_back`]). A number the person typed on a
//! freedom that survives is kept, and so is a value loaded from the
//! bundle that reads anything but a constant, or that the mate fixing a
//! direction — the chart it is read on — still reads as loaded. Only a
//! constant whose chart changed is measured again, so no value reading
//! `px` is ever re-written as a measured number.
//!
//! "A mate in place of a slide or a spin" is [`MateTool::add_mate`]:
//! the value rows shrink to what the admitted mates leave.
//!
//! # Seats, routed by body
//!
//! A pick on the draft's copy fills an `on` seat of the active row; a
//! pick anywhere else fills a `to` seat. Order carries nothing. Each
//! kind names its seats ([`MateSeat`]): one for a plane, an axis, a point
//! or a direction, read off one entity ([`MateReading::of`]); two for a
//! frame, an axis seat an edge fills and a point seat a vertex fills,
//! written [`pose::PoseDef::Through`]. A pick on a placement's copy
//! authors `on` as the entity of the shape the placement copies
//! ([`crate::world::body_of`]); a pick on an instance as its own
//! output's; a pick on a member of an instance is read through A11's
//! member walk ([`pncad::document::member_of`]).
//!
//! # Survival
//!
//! Tool state survives a picked reference vanishing (GQ7's recorded
//! constraint): [`MateTool::reconcile`] re-reads each held pick against
//! the landed pair and **empties the seat of each lost pick**, typed
//! ([`MateToolEvent::PickLost`]) — the seated tools' rule
//! ([`crate::seats`]). No pick moves between seats.
//!
//! **`reconcile` is the consumer's obligation**: the application calls
//! it once per frame (`sync_scene`); a headless consumer that forgets
//! sees a stale pick refused at Apply, typed but late.
//!
//! # The verdict
//!
//! The editor derives its Apply ([`MateTool::proposal`]); the session
//! judges it on a scratch recording
//! ([`crate::session::DocSession::bundle_verdict`]), so the panel names
//! a refusal — mates repeating an equation, a freedom left unpinned,
//! two spaces, a side off the copy, a pick its kind cannot read —
//! before Apply, and a refused Apply leaves the editor open.
//!
//! Module kind: **vocabulary** — it names no driver type and no
//! `app`-only crate (`crates/viewer/README.md`, Module boundaries).

use pncad::document::{
    Constraint, ConstraintId, ConstraintKind, Coord, Dimension, Doc, Evaluation, Formula, Mate,
    MateRef, MateSide, Node, Operand, Orientation, Placed, ProfileProgram, RecipeNodeId, Said, Say,
    SitedRef, SlotKind, Speaker, SpokenNode, Value, VarId, VarKind, member_of, pose,
};
use pncad::geom_core::Tol;
use pncad::prelude::StableName;
use pncad::select::{EntityKind, Resolution};

use crate::pickindex::PickKinds;
use crate::session::select::resolves;
use crate::session::{EdgeSelection, FaceSelection, SessionOp, VertexSelection};
use crate::tree::{DrawnBody, HeldPick, PickedEntity};
use crate::vocab::partial_mirror;

/// **The kinds a mate's picks are read as**, with their labels: a plane
/// off a face, an axis off a face's carrier or an edge's line, a point
/// off a vertex or a carrier's or a circle's centre, a direction off a
/// face's normal or an edge's line, and a frame through an edge's axis
/// and a vertex. Two planes rest, two axes are coaxial, two points are a
/// ball, two directions are parallel, two frames coincide.
///
/// **Deliberately partial**: the `partial_mirror!` invocation below
/// forces every [`VarKind`] variant to be offered here or named as
/// deliberately absent, with the reason.
pub const MATE_KINDS: [(VarKind, &str); 5] = [
    (VarKind::Plane, "plane"),
    (VarKind::Axis, "axis"),
    (VarKind::Point, "point"),
    (VarKind::Direction, "direction"),
    (VarKind::Frame, "frame"),
];

partial_mirror! {
    VarKind, labelled MATE_KINDS,
    offered [Plane, Axis, Point, Direction, Frame],
    absent [
        Length => "a number, never a pose",
        Angle => "a number, never a pose",
        Scalar => "a number, never a pose",
        Count => "a number, never a pose",
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

/// **What the cursor may pick while the editor's active row reads
/// `kind`**: of every rule ([`PickKinds::ALL`]) admitting each entity
/// the kind reads ([`reads`]), the one admitting fewest — so a kind that
/// reads faces alone picks faces alone, and a narrow shelf or a small
/// hole's wall stays pickable however near its own boundary the cursor
/// is. A pick already held that the kind chosen since does not read is
/// named by the verdict ([`MateToolError::KindDoesNotRead`]).
#[must_use]
pub fn pick_kinds(kind: VarKind) -> PickKinds {
    let read = reads(kind);
    let entities = [EntityKind::Face, EntityKind::Edge, EntityKind::Vertex];
    PickKinds::ALL
        .into_iter()
        .filter(|rule| read.iter().all(|&entity| rule.admits(entity)))
        .min_by_key(|rule| {
            entities
                .iter()
                .filter(|&&entity| rule.admits(entity))
                .count()
        })
        .unwrap_or_default()
}

/// **A mate pick**: the face, edge or vertex picked in the viewport, by
/// the selection vocabulary it was picked in. What it reads as is its
/// name's entity kind ([`MatePick::entity`]).
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
    /// The entity kind the pick's name names.
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

/// **How a pick of one entity kind is read as a pose**: the pose
/// definition its one read sits in, and the role a direction is read
/// through. A plane, an axis and a point are the pose itself; a
/// direction is a face's plane read through its normal, or an edge's
/// axis read through its direction — the kernel's own spelling of a
/// direction mate's side, the flip of the role read, so both sides of
/// a direction carry one flip.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MateReading {
    /// The pose the pick is read as.
    pub pose: ReadPose,
    /// The role a direction is read through, `None` for the pose
    /// itself.
    pub role: Option<pose::PoseRole>,
}

/// **A pose read off one entity**: the three pose definitions whose
/// one read is a pick.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReadPose {
    /// A face's plane ([`pose::PoseDef::Plane`]).
    Plane,
    /// A carrier's or an edge's axis ([`pose::PoseDef::Axis`]).
    Axis,
    /// A vertex, or a carrier's or a circle's centre
    /// ([`pose::PoseDef::Point`]).
    Point,
}

impl MateReading {
    /// **The reading of a pick of `entity` at `seat` of a mate of
    /// `kind`**, `None` where none reads it. Whether a pose's read
    /// admits the entity is the kernel's seat
    /// ([`SlotKind::selection_kind`] of the pose's own read), never a
    /// table here; a frame's axis seat takes an edge and its point seat
    /// a vertex.
    #[must_use]
    pub fn of(kind: VarKind, seat: MateSeat, entity: EntityKind) -> Option<Self> {
        let reading = |pose, role| Self { pose, role };
        let candidates: &[Self] = match (kind, seat) {
            (VarKind::Plane, MateSeat::One) => &[reading(ReadPose::Plane, None)],
            (VarKind::Axis, MateSeat::One) => &[reading(ReadPose::Axis, None)],
            (VarKind::Point, MateSeat::One) => &[reading(ReadPose::Point, None)],
            (VarKind::Direction, MateSeat::One) => &[
                reading(ReadPose::Plane, Some(pose::PoseRole::Normal)),
                reading(ReadPose::Axis, Some(pose::PoseRole::Direction)),
            ],
            (VarKind::Frame, MateSeat::Axis) if entity == EntityKind::Edge => {
                &[reading(ReadPose::Axis, None)]
            }
            (VarKind::Frame, MateSeat::Point) if entity == EntityKind::Vertex => {
                &[reading(ReadPose::Point, None)]
            }
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
            ReadPose::Plane => pose::PoseDef::Plane { face: read },
            ReadPose::Axis => pose::PoseDef::Axis { of: read },
            ReadPose::Point => pose::PoseDef::Point { of: read },
        }
    }

    /// The seat the pose's one read sits at, as the kernel states it.
    fn seat(self) -> SlotKind {
        match self.def(Operand::Var(VarId::WORLD)).reads().first() {
            Some(&(_, _, _, seat)) => seat,
            None => unreachable!("a plane, an axis and a point each read one entity"),
        }
    }

    /// **`read` as this reading's side**, turned round where `flip`: the
    /// pose itself (its flip where flipped), or a direction read through
    /// its role, its sense turned by flipping the pose it is read off.
    fn side(self, read: Operand, flip: bool) -> Operand {
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

/// `pose` turned round: its [`pose::PoseDef::Flip`].
fn turned(pose: Operand) -> Operand {
    Operand::Pose(Box::new(pose::PoseDef::Flip {
        pose: pose::RoleRead::plain(pose),
    }))
}

/// **`side` with its sense turned the other way**: a plain flip taken
/// off, or one put on.
fn toggled(side: Operand) -> Operand {
    match side {
        Operand::Pose(def) => match *def {
            pose::PoseDef::Flip {
                pose: pose::RoleRead { read, role: None },
            } => read,
            other => turned(Operand::Pose(Box::new(other))),
        },
        other => turned(other),
    }
}

/// **The entity kinds a mate of `kind` reads**, at any of its seats, in
/// the order a reader lists them.
#[must_use]
pub fn reads(kind: VarKind) -> Vec<EntityKind> {
    [EntityKind::Face, EntityKind::Edge, EntityKind::Vertex]
        .into_iter()
        .filter(|&entity| {
            MateSeat::of(kind)
                .iter()
                .any(|&seat| MateReading::of(kind, seat, entity).is_some())
        })
        .collect()
}

/// **The two senses a mate of `kind` may be read in**, the default
/// first, each with whether it turns `to` round; `None` for a kind with
/// no sense (a point, [`pose::PoseAdmits::Sensed`]).
///
/// A plane opens `opposed`: a plain plane mate equates outward normals,
/// so two faces resting together carry the flip, and `flush` lays them
/// level, normals the same way. An axis and a direction open `same`,
/// the two running one way, and a frame opens `same`, its axes on the
/// target's: turned, an edge mated to an edge would stand its copy
/// upside down.
#[must_use]
pub fn senses(kind: VarKind) -> Option<[(&'static str, bool); 2]> {
    if !pose::PoseAdmits::Sensed.kinds().contains(&kind) {
        return None;
    }
    Some(match kind {
        VarKind::Plane => [("opposed", true), ("flush", false)],
        VarKind::Frame => [("same", false), ("turned over", true)],
        _ => [("same", false), ("reversed", true)],
    })
}

/// Whether a mate of `kind` opens turned round: its default sense.
fn opens_turned(kind: VarKind) -> bool {
    senses(kind).is_some_and(|[(_, turned), _]| turned)
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

/// **A seat of a mate's side**: the one entity a plane, an axis, a
/// point or a direction is read off, or one of the two a frame is read
/// through.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MateSeat {
    /// The one seat of a one-pick kind.
    One,
    /// A frame's axis: an edge.
    Axis,
    /// A frame's point: a vertex.
    Point,
}

impl MateSeat {
    /// The seats a side of a mate of `kind` has.
    #[must_use]
    pub fn of(kind: VarKind) -> &'static [Self] {
        match kind {
            VarKind::Frame => &[Self::Axis, Self::Point],
            _ => &[Self::One],
        }
    }

    /// The seat a pick of `entity` fills on a side of a mate of `kind`:
    /// a frame's point for a vertex, its axis for anything else; the
    /// one seat otherwise.
    #[must_use]
    pub fn filled_by(kind: VarKind, entity: EntityKind) -> Self {
        match (kind, entity) {
            (VarKind::Frame, EntityKind::Vertex) => Self::Point,
            (VarKind::Frame, _) => Self::Axis,
            _ => Self::One,
        }
    }

    /// The seat's word: `axis`, `point`, or none for the one seat.
    #[must_use]
    pub fn name(self) -> Option<&'static str> {
        match self {
            Self::One => None,
            Self::Axis => Some("axis"),
            Self::Point => Some("point"),
        }
    }
}

/// **One side of a mate row**: the side as the bundle wrote it, while
/// no seat of it has been picked again, and the picks its seats hold.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct MateSideDraft {
    /// The side as loaded from the bundle, in its loaded sense.
    written: Option<Operand>,
    /// The one seat's pick, or a frame's axis pick.
    first: Option<HeldPick<MatePick>>,
    /// A frame's point pick.
    point: Option<HeldPick<MatePick>>,
}

impl MateSideDraft {
    /// The pick a seat holds.
    #[must_use]
    pub fn pick(&self, seat: MateSeat) -> Option<&HeldPick<MatePick>> {
        match seat {
            MateSeat::One | MateSeat::Axis => self.first.as_ref(),
            MateSeat::Point => self.point.as_ref(),
        }
    }

    /// Whether the side is as the bundle wrote it.
    #[must_use]
    pub fn is_written(&self) -> bool {
        self.written.is_some()
    }

    fn seat_mut(&mut self, seat: MateSeat) -> &mut Option<HeldPick<MatePick>> {
        match seat {
            MateSeat::One | MateSeat::Axis => &mut self.first,
            MateSeat::Point => &mut self.point,
        }
    }

    fn picks(&self) -> impl Iterator<Item = &HeldPick<MatePick>> {
        self.first.iter().chain(self.point.iter())
    }
}

/// **One mate of the draft**: its kind, its sense, its two sides, and
/// whether it is struck out; with the constraint it was loaded from.
#[derive(Debug, Clone, PartialEq)]
pub struct MateRow {
    kind: VarKind,
    flip: bool,
    on: MateSideDraft,
    to: MateSideDraft,
    struck: bool,
    loaded: Option<LoadedMate>,
}

/// A loaded row's constraint: its id, its orientation, and the sense it
/// was written in.
#[derive(Debug, Clone, Copy, PartialEq)]
struct LoadedMate {
    id: ConstraintId,
    orient: Orientation,
    flip: bool,
}

impl MateRow {
    /// A fresh row of `kind`, at its default sense, no seat picked.
    fn fresh(kind: VarKind) -> Self {
        Self {
            kind,
            flip: opens_turned(kind),
            on: MateSideDraft::default(),
            to: MateSideDraft::default(),
            struck: false,
            loaded: None,
        }
    }

    /// **A row holding `mate` as written**, read in `doc`: its kind its
    /// `on`'s, its sense whether its `to` reads the opposite sense of
    /// its `on`.
    fn written(doc: &Doc<ProfileProgram>, loaded: Option<ConstraintId>, mate: &Mate<Formula>) -> Self {
        let kind = editor_core::mate::pin::operand_kind(doc, &mate.on).unwrap_or(VarKind::Plane);
        let flip = side_flipped(doc, &mate.to) != side_flipped(doc, &mate.on);
        let written = |side: &Operand| MateSideDraft {
            written: Some(side.clone()),
            ..MateSideDraft::default()
        };
        Self {
            kind,
            flip,
            on: written(&mate.on),
            to: written(&mate.to),
            struck: false,
            loaded: loaded.map(|id| LoadedMate {
                id,
                orient: mate.orient,
                flip,
            }),
        }
    }

    /// The row's kind.
    #[must_use]
    pub fn kind(&self) -> VarKind {
        self.kind
    }

    /// Whether the row turns `to` round.
    #[must_use]
    pub fn flip(&self) -> bool {
        self.flip
    }

    /// Whether the row is struck out.
    #[must_use]
    pub fn struck(&self) -> bool {
        self.struck
    }

    /// The row's side.
    #[must_use]
    pub fn side(&self, side: MateSide) -> &MateSideDraft {
        match side {
            MateSide::On => &self.on,
            MateSide::To => &self.to,
        }
    }

    /// The constraint the row was loaded from, if it was.
    #[must_use]
    pub fn loaded(&self) -> Option<ConstraintId> {
        self.loaded.map(|loaded| loaded.id)
    }

    fn side_mut(&mut self, side: MateSide) -> &mut MateSideDraft {
        match side {
            MateSide::On => &mut self.on,
            MateSide::To => &mut self.to,
        }
    }

    /// **Whether the row reads as it was loaded**: both sides as
    /// written, in the loaded sense — so a chart on it is the bundle's.
    fn as_loaded(&self) -> bool {
        self.loaded.is_some_and(|loaded| loaded.flip == self.flip)
            && self.on.is_written()
            && self.to.is_written()
    }
}

/// **Whether `side` reads the opposite sense of its own direction**: an
/// odd count of flips along the poses it keeps, read plainly or through
/// a role that keeps a sense — the kernel's [`editor_core::mate::flipped`]
/// over a side as authored.
fn side_flipped(doc: &Doc<ProfileProgram>, side: &Operand) -> bool {
    use pose::{PoseDef, PoseRole, RoleRead};
    match side {
        Operand::Var(var) => editor_core::mate::flipped(doc, *var),
        Operand::Pose(def) => {
            let here = matches!(**def, PoseDef::Flip { .. });
            let kept = match def.kept() {
                Some(RoleRead {
                    read,
                    role: None | Some(PoseRole::Normal | PoseRole::Direction),
                }) => side_flipped(doc, read),
                _ => false,
            };
            here != kept
        }
        _ => false,
    }
}

/// **The editor's door** — what Apply writes into.
#[derive(Debug, Clone, PartialEq)]
pub enum PlaceDoor {
    /// A body not yet placed: Apply inserts one placement of it.
    Create {
        /// The body, as a seat reads it.
        body: Operand,
    },
    /// A placement: Apply writes its bundle whole.
    Edit {
        /// The placement.
        placement: RecipeNodeId,
        /// Its bundle as authored when the draft was loaded
        /// ([`crate::session::held_bundle`]) — what tells the editor
        /// the document has moved under it.
        base: Vec<Constraint<Formula>>,
    },
}

/// **A value row**: a freedom the live mates leave, its number, and
/// where the number came from.
#[derive(Debug, Clone, PartialEq)]
pub struct ValueRow {
    /// The freedom.
    pub coord: Coord,
    /// Its number, as Apply writes it.
    pub by: Formula,
    /// Where it came from.
    pub from: ValueFrom,
}

/// Where a value row's number came from.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValueFrom {
    /// The bundle, under this id.
    Loaded(ConstraintId),
    /// The person, in this draft.
    Typed,
    /// Where the copy stands, snapped onto the mates.
    Measured,
}

/// **The person's choice for the active row** — its kind, its sense —
/// and the numbers they type, as one value ([`MateTool::chosen`]).
#[derive(Debug, Clone, PartialEq)]
pub struct MateChoice {
    /// The active row's kind ([`MATE_KINDS`]).
    pub kind: VarKind,
    /// Whether the active row's `to` is read in its opposite sense
    /// ([`senses`]).
    pub flip: bool,
    /// Numbers typed on freedoms: a `Length` for a slide, an `Angle`
    /// for a spin.
    pub values: Vec<(Coord, Formula)>,
}

/// The draft: its door, its mate rows, and what the person set.
#[derive(Debug, Clone, PartialEq)]
struct PlaceDraft {
    door: PlaceDoor,
    rows: Vec<MateRow>,
    typed: Vec<(Coord, Formula)>,
    active: usize,
    into_target_space: bool,
}

/// **The placement editor.** A value: the chrome holds one while the
/// tool is open, a test constructs one and drives the same methods.
#[derive(Debug, Clone, PartialEq)]
pub struct MateTool {
    draft: Option<PlaceDraft>,
    /// The kind a fresh row opens with.
    kind: VarKind,
}

impl Default for MateTool {
    fn default() -> Self {
        Self::new()
    }
}

/// A typed editor refusal (closed enum, D4 ¶3).
#[derive(Debug, Clone)]
pub enum MateToolError {
    /// No pick has opened a door yet.
    NoDoor,
    /// A seat of a live mate row holds no pick.
    SeatEmpty {
        /// The row, 0-based.
        row: usize,
        /// Which side.
        side: MateSide,
        /// Which seat.
        seat: MateSeat,
    },
    /// A pick names an entity its seat does not read
    /// ([`MateReading::of`]) — a pick taken under one kind and applied
    /// under another.
    KindDoesNotRead {
        /// The row, 0-based.
        row: usize,
        /// Which side.
        side: MateSide,
        /// The row's kind.
        kind: VarKind,
        /// The entity the pick names.
        entity: EntityKind,
    },
    /// A pick is on a node the document no longer holds: the survival
    /// step ([`MateTool::reconcile`]) has not emptied its seat yet.
    PickGone {
        /// Which side.
        side: MateSide,
        /// The node, by the label the tool kept.
        node: SpokenNode,
    },
    /// A node the Edit door was asked to open on is no placement.
    NotAPlacement {
        /// The node.
        node: SpokenNode,
    },
    /// The constraint the tree's `remove` asked to strike is no mate of
    /// its placement's bundle.
    NotAMate {
        /// The constraint.
        mate: MateRef,
    },
    /// The chosen kind is none the editor reads ([`MATE_KINDS`]).
    KindNotOffered {
        /// The kind asked for.
        kind: VarKind,
    },
    /// The choice turns round a mate whose kind has no sense
    /// ([`senses`]): a point.
    NoSense {
        /// The kind chosen.
        kind: VarKind,
    },
    /// A number is typed on a freedom the live mates do not leave.
    NotAFreedom {
        /// The freedom.
        coord: Coord,
    },
}

impl core::fmt::Display for MateToolError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::NoDoor => write!(f, "pick the body or the copy to place first"),
            Self::SeatEmpty { row, side, seat } => write!(
                f,
                "mate {}'s {}{} seat is empty",
                row + 1,
                side.name(),
                seat.name().map_or(String::new(), |word| format!(" {word}"))
            ),
            Self::KindDoesNotRead {
                row,
                side,
                kind,
                entity,
            } => {
                let word = kind.to_string();
                let article = if word.starts_with(['a', 'e', 'i', 'o', 'u']) {
                    "an"
                } else {
                    "a"
                };
                write!(
                    f,
                    "mate {}'s {} pick is {}, and {article} {word} mate reads {}",
                    row + 1,
                    side.name(),
                    said_entities(&[*entity]),
                    said_entities(&reads(*kind)),
                )
            }
            Self::PickGone { side, node } => write!(
                f,
                "the {} pick is on {node}, which the document no longer holds",
                side.name()
            ),
            Self::NotAPlacement { node } => write!(f, "{node} is no placement"),
            Self::NotAMate { mate } => write!(
                f,
                "constraint {} of node {} is no mate of its bundle",
                mate.id, mate.placement
            ),
            Self::KindNotOffered { kind } => write!(
                f,
                "the editor reads a {}, not a {kind}",
                MATE_KINDS
                    .iter()
                    .map(|(_, label)| *label)
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            Self::NoSense { kind } => write!(f, "a {kind} has no sense to turn round"),
            Self::NotAFreedom { coord } => {
                write!(f, "the mates leave no {} to set", coord.name())
            }
        }
    }
}

impl core::error::Error for MateToolError {}

impl MateToolError {
    /// This refusal with its nodes spoken from `doc`, a later version of
    /// the document the tool read ([`crate::session::Refusal::respoken`]).
    #[must_use]
    pub fn respoken(self, doc: &Doc<ProfileProgram>) -> Self {
        match self {
            Self::PickGone { side, node } => Self::PickGone {
                side,
                node: node.respoken(doc),
            },
            Self::NotAPlacement { node } => Self::NotAPlacement {
                node: node.respoken(doc),
            },
            unspoken @ (Self::NoDoor
            | Self::SeatEmpty { .. }
            | Self::KindDoesNotRead { .. }
            | Self::NotAMate { .. }
            | Self::KindNotOffered { .. }
            | Self::NoSense { .. }
            | Self::NotAFreedom { .. }) => unspoken,
        }
    }
}

/// **The sentence a dropped pick is said in**: its role, and the pick
/// as its panel line said it ([`HeldPick`]) — the editor's drop and the
/// mirror tool's (`crate::combine::MirrorEvent`).
pub(crate) fn pick_lost<P: PickedEntity>(
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

/// A typed tool event the chrome renders — every state change that was
/// not the direct echo of an op.
#[derive(Debug)]
pub enum MateToolEvent {
    /// A held pick's reference no longer resolves against the landed
    /// evaluation; its seat is empty.
    PickLost {
        /// The row, 0-based.
        row: usize,
        /// Which side.
        side: MateSide,
        /// Which seat.
        seat: MateSeat,
        /// The pick that was held, with the last labels the tool kept
        /// for its nodes ([`MateTool::respeak`]).
        pick: HeldPick<MatePick>,
        /// The resolution machinery's own verdict
        /// ([`crate::tree::resolution_as_drawn`]); boxed for width.
        resolution: Box<Resolution>,
    },
    /// The bundle the Edit door loaded is not the one the placement
    /// holds now (an undo, a redo, an edit from elsewhere): the draft
    /// was loaded again, and what it held is gone.
    Reloaded {
        /// The placement.
        placement: SpokenNode,
    },
    /// The placement the Edit door was on is gone: the editor holds no
    /// draft.
    PlacementGone {
        /// The placement.
        placement: SpokenNode,
    },
    /// A pick opened no door: it is on no copy and no body.
    NotPlaceable {
        /// The node the pick's body belongs to.
        node: SpokenNode,
    },
}

impl Say for MateToolEvent {
    fn say(&self, f: &mut core::fmt::Formatter<'_>, by: Speaker<'_>) -> core::fmt::Result {
        match self {
            Self::PickLost {
                row,
                side,
                seat,
                pick,
                ..
            } => {
                let role = match seat.name() {
                    Some(word) => format!("mate {} {} {word}", row + 1, side.name()),
                    None => format!("mate {} {}", row + 1, side.name()),
                };
                pick_lost(f, role, pick, by)
            }
            Self::Reloaded { placement } => write!(
                f,
                "{placement} changed under the editor; it loaded the bundle again"
            ),
            Self::PlacementGone { placement } => {
                write!(f, "{placement} is gone; the editor holds nothing")
            }
            Self::NotPlaceable { node } => write!(
                f,
                "the pick is on {node}, which is no copy and no body to place"
            ),
        }
    }
}

impl core::fmt::Display for MateToolEvent {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::PickLost { pick, .. } => self.say(f, Speaker::held(pick.said())),
            Self::Reloaded { .. } | Self::PlacementGone { .. } | Self::NotPlaceable { .. } => {
                self.say(f, Speaker::TAG)
            }
        }
    }
}

/// **One mate of an Apply**: the mate, and the entities its seats were
/// picked on.
#[derive(Debug, Clone, PartialEq)]
pub struct ProposedMate {
    /// The mate as Apply writes it.
    pub mate: Mate<Formula>,
    /// The `on` seats' entities, read off the copy's shapes; empty for a
    /// side as the bundle wrote it.
    pub on: Vec<SitedRef>,
    /// The `to` seats' entities, read where they were picked; empty for
    /// a side as the bundle wrote it.
    pub to: Vec<SitedRef>,
}

/// **The editor's Apply, derived**: the door, the live mates, and the
/// bundle with its value rows.
#[derive(Debug, Clone, PartialEq)]
pub struct MateProposal {
    /// The door.
    pub door: PlaceDoor,
    /// The placement an Edit writes; `None` for a Create.
    pub placement: Option<RecipeNodeId>,
    /// The space a Create places its copy into: the space its first
    /// mate's `to` reads, or the body's own with no mate.
    pub target: Option<VarId>,
    /// The live mates, in row order.
    pub mates: Vec<ProposedMate>,
    /// The value rows.
    pub values: Vec<ValueRow>,
    /// The bundle Apply writes: the mates, then the values.
    pub bundle: Vec<Constraint<Formula>>,
}

impl MateProposal {
    /// **The one committed action**: a Create's [`SessionOp::Place`], or
    /// an Edit's [`SessionOp::SetBundle`]. A mate whose `to` reads
    /// outside an Edit's target space is refused there, naming both
    /// spaces ([`Self::op_into_target_space`] moves it).
    #[must_use]
    pub fn op(&self) -> SessionOp {
        self.committed(false)
    }

    /// [`Self::op`], an Edit's copy moved by name into the space its
    /// first mate's `to` reads — the chrome's explicit "move the copy"
    /// choice, never a side effect of a mate.
    #[must_use]
    pub fn op_into_target_space(&self) -> SessionOp {
        self.committed(true)
    }

    fn committed(&self, into_target_space: bool) -> SessionOp {
        match &self.door {
            PlaceDoor::Create { body } => SessionOp::Place {
                body: body.clone(),
                target: self.target.unwrap_or(VarId::WORLD),
                constraints: self.bundle.clone(),
            },
            PlaceDoor::Edit { placement, base } => SessionOp::SetBundle {
                placement: *placement,
                base: base.clone(),
                constraints: self.bundle.clone(),
                into_target_space,
            },
        }
    }
}

/// **A face read as its plane**: `face`, a one-name face selection,
/// read plainly as the plane of that face — a plane mate's side, and
/// the plane a mirror reflects across (`crate::combine::mirror_node`).
pub(crate) fn plane_of(face: Operand) -> Operand {
    Operand::Pose(Box::new(pose::PoseDef::Plane { face }))
}

/// The placement whose copy a pick at `at` is on: `at` itself when it
/// is a `Place` or an instance, the instance A11's member walk reaches
/// otherwise.
fn placement_of(doc: &Doc<ProfileProgram>, at: RecipeNodeId, read: &SitedRef) -> Option<RecipeNodeId> {
    match doc.node(at) {
        Some(Node::Place { .. } | Node::InstantiatePart { .. }) => Some(at),
        _ => member_of(doc, read).map(|member| member.instance),
    }
}

/// The placement whose copy `pick` is on, if any.
fn copy_picked(doc: &Doc<ProfileProgram>, pick: &MatePick) -> Option<RecipeNodeId> {
    let read = SitedRef::new(pick.node(), pick.name().clone());
    placement_of(doc, pick.node(), &read)
}

/// **The entity `pick` names as an `on` side reads it**: off the shape
/// a `Place` copies, by the shape's own name for it (a copy carries its
/// body's names), on the output it copies where the shape's operation
/// defines several (a split's halves); an instance's or a body's own
/// read otherwise.
fn on_read(doc: &Doc<ProfileProgram>, pick: &MatePick) -> (SitedRef, Operand) {
    let picked = SitedRef::new(pick.node(), pick.name().clone());
    match doc.node(pick.node()) {
        Some(Node::Place { .. }) => {
            let (at, port) = crate::world::body_of(doc, pick.node(), pick.on().body());
            let body = match crate::world::seat_read(doc, at, Some(port)) {
                (at, Some(port)) => Operand::output(at, port),
                (at, None) => Operand::Node(at),
            };
            let read = Operand::select(body, vec![pick.name().clone()]);
            (SitedRef::new(at, pick.name().clone()), read)
        }
        Some(Node::InstantiatePart { .. }) => (picked.clone(), picked.into()),
        _ => (picked, crate::session::pick_read(doc, pick)),
    }
}

/// **The entity `pick` names as a `to` side reads it**: on a copy, the
/// copy itself; on a drawn operation, the output picked
/// ([`crate::session::pick_read`]).
fn to_read(doc: &Doc<ProfileProgram>, pick: &MatePick) -> (SitedRef, Operand) {
    let picked = SitedRef::new(pick.node(), pick.name().clone());
    let read = match doc.node(pick.node()) {
        Some(Node::Place { .. } | Node::InstantiatePart { .. }) => picked.clone().into(),
        _ => crate::session::pick_read(doc, pick),
    };
    (picked, read)
}

/// Whether a mate of `kind` fixes one direction — a chart's mate.
fn fixes_a_direction(kind: VarKind) -> bool {
    matches!(kind, VarKind::Plane | VarKind::Axis | VarKind::Direction)
}

/// A length or an angle as the value that writes it: the constant `0`
/// where it is zero.
pub(crate) fn number(by: f64, dim: Dimension) -> Formula {
    if by == 0.0 {
        Formula::zero(dim)
    } else {
        Formula::literal(by, dim).unwrap_or_else(|_| Formula::zero(dim))
    }
}

impl MateTool {
    /// An editor holding nothing: the first pick opens its door.
    #[must_use]
    pub fn new() -> Self {
        Self {
            draft: None,
            kind: VarKind::Plane,
        }
    }

    /// **The Create door on `body`**, a body not yet placed: no mate,
    /// and its copy where the body stands.
    #[must_use]
    pub fn create(body: Operand) -> Self {
        Self {
            draft: Some(PlaceDraft {
                door: PlaceDoor::Create { body },
                rows: Vec::new(),
                typed: Vec::new(),
                active: 0,
                into_target_space: false,
            }),
            kind: VarKind::Plane,
        }
    }

    /// **The Edit door on `placement`**, its bundle loaded as authored:
    /// one row per mate, each as written.
    ///
    /// # Errors
    ///
    /// [`MateToolError::NotAPlacement`] for a node that is none.
    pub fn edit(doc: &Doc<ProfileProgram>, placement: RecipeNodeId) -> Result<Self, MateToolError> {
        let Some(Node::Place { .. } | Node::InstantiatePart { .. }) = doc.node(placement) else {
            return Err(MateToolError::NotAPlacement {
                node: doc.spoken(placement),
            });
        };
        let base = crate::session::held_bundle(doc, placement);
        let rows = base
            .iter()
            .filter_map(|c| c.mate().map(|m| MateRow::written(doc, Some(c.id), m)))
            .collect();
        Ok(Self {
            draft: Some(PlaceDraft {
                door: PlaceDoor::Edit { placement, base },
                rows,
                typed: Vec::new(),
                active: 0,
                into_target_space: false,
            }),
            kind: VarKind::Plane,
        })
    }

    /// **The Edit door on `mate`'s placement, that mate struck out** —
    /// the tree's `remove` on a mate row: the freed values and where the
    /// copy would stand are seen before Apply.
    ///
    /// # Errors
    ///
    /// As [`Self::edit`]'s, and [`MateToolError::NotAMate`] for a
    /// constraint that is no mate of the bundle.
    pub fn edit_striking(doc: &Doc<ProfileProgram>, mate: MateRef) -> Result<Self, MateToolError> {
        let mut tool = Self::edit(doc, mate.placement)?;
        let Some(draft) = tool.draft.as_mut() else {
            unreachable!("the Edit door holds a draft")
        };
        let Some(at) = draft
            .rows
            .iter()
            .position(|row| row.loaded() == Some(mate.id))
        else {
            return Err(MateToolError::NotAMate { mate });
        };
        draft.rows[at].struck = true;
        draft.active = at;
        Ok(tool)
    }

    /// **A row holding `mate` as written**, made the active one — a
    /// mate an API caller states rather than picks
    /// ([`SessionOp::AddMate`]).
    pub fn add_written(&mut self, doc: &Doc<ProfileProgram>, mate: &Mate<Formula>) {
        if let Some(draft) = self.draft.as_mut() {
            draft.rows.push(MateRow::written(doc, None, mate));
            draft.active = draft.rows.len() - 1;
        }
    }

    /// The door the draft is on, `None` before the first pick.
    #[must_use]
    pub fn door(&self) -> Option<&PlaceDoor> {
        self.draft.as_ref().map(|draft| &draft.door)
    }

    /// The mate rows, in order.
    #[must_use]
    pub fn rows(&self) -> &[MateRow] {
        self.draft.as_ref().map_or(&[], |draft| &draft.rows)
    }

    /// The row picks go to.
    #[must_use]
    pub fn active(&self) -> usize {
        self.draft.as_ref().map_or(0, |draft| draft.active)
    }

    /// The kind the active row reads, or the one a fresh row opens with
    /// — what narrows the cursor ([`pick_kinds`]).
    #[must_use]
    pub fn active_kind(&self) -> VarKind {
        self.rows()
            .get(self.active())
            .map_or(self.kind, MateRow::kind)
    }

    /// The numbers the person typed.
    #[must_use]
    pub fn typed(&self) -> &[(Coord, Formula)] {
        self.draft.as_ref().map_or(&[], |draft| &draft.typed)
    }

    /// Whether an Edit's Apply moves its copy into its first mate's `to`
    /// space.
    #[must_use]
    pub fn into_target_space(&self) -> bool {
        self.draft
            .as_ref()
            .is_some_and(|draft| draft.into_target_space)
    }

    /// Set whether an Edit's Apply moves its copy into its first mate's
    /// `to` space.
    pub fn set_into_target_space(&mut self, into: bool) {
        if let Some(draft) = self.draft.as_mut() {
            draft.into_target_space = into;
        }
    }

    /// **Feed one pick**, consumed into tool state. With no door yet the
    /// pick opens one — the Edit door on the placement whose copy it is
    /// on, or the Create door on the body it is on — and fills the `on`
    /// seat of a fresh row. With a door open, a pick on the draft's copy
    /// fills an `on` seat of the active row and any other pick a `to`
    /// seat ([`MateSeat::filled_by`]), replacing what the seat held. A pick
    /// that opens no door is said ([`MateToolEvent::NotPlaceable`]).
    pub fn pick(
        &mut self,
        doc: &Doc<ProfileProgram>,
        pick: impl Into<MatePick>,
    ) -> Option<MateToolEvent> {
        let pick = pick.into();
        let kind = self.kind;
        if self.draft.is_none() {
            let opened = match copy_picked(doc, &pick) {
                Some(placement) => Self::edit(doc, placement).ok(),
                None => doc
                    .node(pick.node())
                    .is_some()
                    .then(|| Self::create(crate::session::picked_body(doc, &pick).0)),
            };
            let Some(opened) = opened else {
                return Some(MateToolEvent::NotPlaceable {
                    node: doc.spoken(pick.node()),
                });
            };
            *self = Self { kind, ..opened };
            self.add_mate(kind);
        }
        let side = if self.is_on_copy(doc, &pick) {
            MateSide::On
        } else {
            MateSide::To
        };
        let Some(draft) = self.draft.as_mut() else {
            unreachable!("a door is open: it was opened just above")
        };
        if draft.rows.is_empty() {
            draft.rows.push(MateRow::fresh(kind));
            draft.active = 0;
        }
        let row = &mut draft.rows[draft.active];
        let seat = MateSeat::filled_by(row.kind, pick.entity());
        let held = HeldPick::new(doc, pick);
        let drafted = row.side_mut(side);
        drafted.written = None;
        *drafted.seat_mut(seat) = Some(held);
        None
    }

    /// Whether `pick` is on the draft's copy: the Edit door's
    /// placement's, or the Create door's body.
    fn is_on_copy(&self, doc: &Doc<ProfileProgram>, pick: &MatePick) -> bool {
        match self.door() {
            Some(PlaceDoor::Edit { placement, .. }) => copy_picked(doc, pick) == Some(*placement),
            Some(PlaceDoor::Create { body }) => {
                copy_picked(doc, pick).is_none()
                    && crate::session::picked_body(doc, pick).0 == *body
            }
            None => false,
        }
    }

    /// **A fresh mate row of `kind`**, made the active one: a mate in
    /// place of the slides and the spin the earlier ones leave.
    pub fn add_mate(&mut self, kind: VarKind) {
        self.kind = kind;
        if let Some(draft) = self.draft.as_mut() {
            draft.rows.push(MateRow::fresh(kind));
            draft.active = draft.rows.len() - 1;
        }
    }

    /// Make row `row` the one picks go to.
    pub fn focus(&mut self, row: usize) {
        if let Some(draft) = self.draft.as_mut()
            && row < draft.rows.len()
        {
            draft.active = row;
        }
    }

    /// **Choose row `row`'s kind**: a side as written reads the old
    /// kind and is emptied; a held pick moves to the seat it fills under
    /// the new kind; the row opens at the kind's own sense.
    pub fn choose_kind(&mut self, row: usize, kind: VarKind) {
        self.kind = kind;
        let Some(row) = self.draft.as_mut().and_then(|d| d.rows.get_mut(row)) else {
            return;
        };
        if row.kind == kind {
            return;
        }
        let mut fresh = MateRow::fresh(kind);
        for side in [MateSide::On, MateSide::To] {
            for pick in row.side(side).picks() {
                let seat = MateSeat::filled_by(kind, pick.pick().entity());
                *fresh.side_mut(side).seat_mut(seat) = Some(pick.clone());
            }
        }
        fresh.loaded = row.loaded;
        fresh.struck = row.struck;
        *row = fresh;
    }

    /// Set row `row`'s sense: whether its `to` is turned round. A kind
    /// with no sense keeps none.
    pub fn set_sense(&mut self, row: usize, flip: bool) {
        if let Some(row) = self.draft.as_mut().and_then(|d| d.rows.get_mut(row)) {
            row.flip = flip && senses(row.kind).is_some();
        }
    }

    /// Strike row `row` out, or take the stroke off.
    pub fn strike(&mut self, row: usize, struck: bool) {
        if let Some(row) = self.draft.as_mut().and_then(|d| d.rows.get_mut(row)) {
            row.struck = struck;
        }
    }

    /// **Type `by` on `coord`**: the number Apply writes on that freedom
    /// while the mates leave it, whatever it held.
    pub fn type_value(&mut self, coord: Coord, by: Formula) {
        if let Some(draft) = self.draft.as_mut() {
            draft.typed.retain(|(c, _)| *c != coord);
            draft.typed.push((coord, by));
        }
    }

    /// **The active row chosen as `choice` says**, and its numbers typed
    /// — the panel's state as one value.
    ///
    /// # Errors
    ///
    /// [`MateToolError::KindNotOffered`] and [`MateToolError::NoSense`].
    pub fn chosen(mut self, choice: MateChoice) -> Result<Self, MateToolError> {
        if !MATE_KINDS.iter().any(|(kind, _)| *kind == choice.kind) {
            return Err(MateToolError::KindNotOffered { kind: choice.kind });
        }
        if choice.flip && senses(choice.kind).is_none() {
            return Err(MateToolError::NoSense { kind: choice.kind });
        }
        let active = self.active();
        self.choose_kind(active, choice.kind);
        self.set_sense(active, choice.flip);
        for (coord, by) in choice.values {
            self.type_value(coord, by);
        }
        Ok(self)
    }

    /// **The held picks' nodes, spoken again from `doc`**
    /// ([`SpokenNode::respoken`]'s rule).
    pub fn respeak(&mut self, doc: &Doc<ProfileProgram>) {
        for row in self.draft.iter_mut().flat_map(|d| d.rows.iter_mut()) {
            for side in [&mut row.on, &mut row.to] {
                for held in [&mut side.first, &mut side.point].into_iter().flatten() {
                    *held = held.respoken(doc);
                }
            }
        }
    }

    /// **The line the panel shows for row `row`'s seats**: the seated
    /// tools' line (`seats::picks_line`), each seat a role, a side as
    /// the bundle wrote it said so.
    #[must_use]
    pub fn line(&self, row: usize, by: Speaker<'_>) -> String {
        let Some(mate) = self.rows().get(row) else {
            return String::new();
        };
        let mut roles = Vec::new();
        for side in [MateSide::On, MateSide::To] {
            let drafted = mate.side(side);
            for &seat in MateSeat::of(mate.kind) {
                let role = match seat.name() {
                    Some(word) => format!("{} {word}", side.name()),
                    None => side.name().to_owned(),
                };
                let said = drafted
                    .pick(seat)
                    .map(|pick| Said(pick, by).to_string())
                    .or_else(|| drafted.is_written().then(|| "as written".to_owned()));
                roles.push((role, said));
            }
        }
        crate::seats::picks_line(roles)
    }

    /// **A held pick's node as a refusal says it**: by the label the
    /// tool kept; as `doc` holds it only when nothing named was kept.
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

    /// **Bring the draft up to the landed pair**: an Edit draft over a
    /// bundle its placement no longer holds is loaded again (or dropped,
    /// with its placement), and each held pick whose reference no longer
    /// resolves empties its seat. Returns the typed events.
    pub fn reconcile(
        &mut self,
        doc: &Doc<ProfileProgram>,
        eval: &Evaluation<f64>,
    ) -> Vec<MateToolEvent> {
        let mut events = Vec::new();
        if let Some(PlaceDoor::Edit { placement, base }) = self.door() {
            let placement = *placement;
            let held = match doc.node(placement) {
                Some(Node::Place { .. } | Node::InstantiatePart { .. }) => {
                    Some(crate::session::held_bundle(doc, placement))
                }
                _ => None,
            };
            match held {
                None => {
                    let kept = self
                        .rows()
                        .iter()
                        .flat_map(|row| row.on.picks().chain(row.to.picks()))
                        .map(|pick| pick.said().spoken(placement))
                        .find(|kept| kept.kind().is_some());
                    self.draft = None;
                    events.push(MateToolEvent::PlacementGone {
                        placement: kept.unwrap_or_else(|| doc.spoken(placement)),
                    });
                    return events;
                }
                Some(held) if held != *base => {
                    if let Ok(reloaded) = Self::edit(doc, placement) {
                        *self = Self {
                            kind: self.kind,
                            ..reloaded
                        };
                    }
                    events.push(MateToolEvent::Reloaded {
                        placement: doc.spoken(placement),
                    });
                }
                Some(_) => {}
            }
        }
        let rows = self.draft.iter_mut().flat_map(|d| d.rows.iter_mut());
        for (at, row) in rows.enumerate() {
            let kind = row.kind;
            for side in [MateSide::On, MateSide::To] {
                for &seat in MateSeat::of(kind) {
                    let held = row.side_mut(side).seat_mut(seat);
                    let lost = held.as_ref().is_some_and(|held| {
                        let pick = held.pick();
                        let on = pick.on();
                        let verdict = crate::tree::pick_resolution(
                            doc,
                            eval,
                            on.node(),
                            on.body(),
                            pick.name(),
                        );
                        !resolves(&verdict)
                    });
                    if !lost {
                        continue;
                    }
                    let Some(pick) = held.take() else {
                        unreachable!("the seat held the lost pick just above")
                    };
                    let on = pick.pick().on();
                    let resolution = crate::tree::pick_resolution(
                        doc,
                        eval,
                        on.node(),
                        on.body(),
                        pick.pick().name(),
                    );
                    events.push(MateToolEvent::PickLost {
                        row: at,
                        side,
                        seat,
                        pick,
                        resolution: Box::new(resolution),
                    });
                }
            }
        }
        events
    }

    /// **Row `at`'s mate**, its sides read off its picks or as the
    /// bundle wrote them, `to` in the row's sense.
    fn row_mate(
        doc: &Doc<ProfileProgram>,
        at: usize,
        row: &MateRow,
    ) -> Result<ProposedMate, MateToolError> {
        let sided = |side: MateSide| -> Result<(Operand, Vec<SitedRef>), MateToolError> {
            let drafted = row.side(side);
            if let Some(written) = &drafted.written {
                let read = match (side, row.loaded) {
                    (MateSide::To, Some(loaded)) if loaded.flip != row.flip => {
                        toggled(written.clone())
                    }
                    _ => written.clone(),
                };
                return Ok((read, Vec::new()));
            }
            let flip = side == MateSide::To && row.flip;
            let mut reads = Vec::new();
            let mut sites = Vec::new();
            for &seat in MateSeat::of(row.kind) {
                let Some(held) = drafted.pick(seat) else {
                    return Err(MateToolError::SeatEmpty {
                        row: at,
                        side,
                        seat,
                    });
                };
                let pick = held.pick();
                if doc.node(pick.node()).is_none() {
                    return Err(MateToolError::PickGone {
                        side,
                        node: Self::spoken(doc, held, pick.node()),
                    });
                }
                let entity = pick.entity();
                let reading = MateReading::of(row.kind, seat, entity).ok_or(
                    MateToolError::KindDoesNotRead {
                        row: at,
                        side,
                        kind: row.kind,
                        entity,
                    },
                )?;
                let (site, read) = match side {
                    MateSide::On => on_read(doc, pick),
                    MateSide::To => to_read(doc, pick),
                };
                sites.push(site);
                reads.push((reading, read));
            }
            let pose = match reads.as_slice() {
                [(axis, of_axis), (point, of_point)] => {
                    let through = Operand::Pose(Box::new(pose::PoseDef::Through {
                        axis: pose::RoleRead::plain(axis.side(of_axis.clone(), false)),
                        point: pose::RoleRead::plain(point.side(of_point.clone(), false)),
                    }));
                    if flip { turned(through) } else { through }
                }
                [(one, read)] => one.side(read.clone(), flip),
                _ => unreachable!("a side reads one pick a seat, of one or two seats"),
            };
            Ok((pose, sites))
        };
        let (on, on_sites) = sided(MateSide::On)?;
        let (to, to_sites) = sided(MateSide::To)?;
        Ok(ProposedMate {
            mate: Mate {
                on,
                to,
                orient: row
                    .loaded
                    .map_or(Orientation::IDENTITY, |loaded| loaded.orient),
            },
            on: on_sites,
            to: to_sites,
        })
    }

    /// **The editor's Apply, derived from the draft** against the landed
    /// pair `doc`, `ev`: each live row's mate, and the value rows on the
    /// freedoms they leave — typed, loaded, or measured where the copy
    /// stands snapped onto the mates (module docs).
    ///
    /// # Errors
    ///
    /// Every arm of [`MateToolError`] a draft can carry — see each.
    pub fn proposal(
        &self,
        doc: &Doc<ProfileProgram>,
        ev: &Evaluation<f64>,
        tol: Tol,
    ) -> Result<MateProposal, MateToolError> {
        let Some(draft) = self.draft.as_ref() else {
            return Err(MateToolError::NoDoor);
        };
        let live: Vec<(usize, &MateRow)> = draft
            .rows
            .iter()
            .enumerate()
            .filter(|(_, row)| !row.struck)
            .collect();
        let mut mates = live
            .iter()
            .map(|&(at, row)| Self::row_mate(doc, at, row))
            .collect::<Result<Vec<_>, _>>()?;
        let base: &[Constraint<Formula>] = match &draft.door {
            PlaceDoor::Edit { base, .. } => base,
            PlaceDoor::Create { .. } => &[],
        };
        // The chart: the first live mate fixing a direction. While it is
        // the bundle's own, as loaded, its orientation and its values
        // keep their meaning; otherwise its orientation is the snap's,
        // and its constants are measured again.
        let chart = live.iter().position(|(_, row)| fixes_a_direction(row.kind));
        let base_chart = base.iter().find_map(|c| {
            let mate = c.mate()?;
            editor_core::mate::pin::operand_kind(doc, &mate.on)
                .filter(|&kind| fixes_a_direction(kind))
                .map(|_| c.id)
        });
        let chart_kept = match chart {
            Some(k) => live[k].1.as_loaded() && live[k].1.loaded() == base_chart,
            None => base_chart.is_none(),
        };
        let (placement, target) = match &draft.door {
            PlaceDoor::Edit { placement, .. } => (Some(*placement), None),
            PlaceDoor::Create { body } => {
                let target = mates
                    .first()
                    .and_then(|m| doc.operand_space(&m.mate.to))
                    .or_else(|| doc.operand_space(body));
                (None, target)
            }
        };
        let written: Vec<Mate<Formula>> = mates.iter().map(|m| m.mate.clone()).collect();
        let snap = crate::session::snap::draft_snap(doc, ev, &draft.door, target, &written, tol);
        if let (Some(k), Some(orient)) = (chart, snap.as_ref().and_then(|s| s.orient))
            && !chart_kept
        {
            mates[k].mate.orient = orient;
        }
        let written: Vec<Mate<Formula>> = mates.iter().map(|m| m.mate.clone()).collect();
        let coords = Placed::freedoms(doc, &written, snap.as_ref().and_then(|s| s.along));
        if let Some((coord, _)) = draft.typed.iter().find(|(c, _)| !coords.contains(c)) {
            return Err(MateToolError::NotAFreedom { coord: *coord });
        }
        let measured = snap.as_ref().map_or_else(
            || vec![None; coords.len()],
            |snap| snap.read(chart.map(|k| written[k].orient), &coords, tol),
        );
        let values: Vec<ValueRow> = coords
            .iter()
            .zip(measured)
            .map(|(&coord, measured)| {
                if let Some((_, by)) = draft.typed.iter().find(|(c, _)| *c == coord) {
                    return ValueRow {
                        coord,
                        by: by.clone(),
                        from: ValueFrom::Typed,
                    };
                }
                let loaded = base
                    .iter()
                    .find_map(|c| c.value().filter(|v| v.coord == coord).map(|v| (c.id, v)));
                match loaded {
                    Some((id, v))
                        if chart_kept || v.by.literal_value().is_none() || measured.is_none() =>
                    {
                        ValueRow {
                            coord,
                            by: v.by.clone(),
                            from: ValueFrom::Loaded(id),
                        }
                    }
                    _ => ValueRow {
                        coord,
                        by: number(measured.unwrap_or(0.0), coord.dimension()),
                        from: ValueFrom::Measured,
                    },
                }
            })
            .collect();
        // A loaded row keeps its id, a value on a freedom the bundle held
        // keeps that value's; every other constraint is minted at Apply.
        let mut minted = 0;
        let mut fresh = || {
            minted += 1;
            ConstraintId::placeholder(minted)
        };
        let mut bundle: Vec<Constraint<Formula>> = live
            .iter()
            .zip(&mates)
            .map(|((_, row), proposed)| Constraint {
                id: row.loaded().unwrap_or_else(&mut fresh),
                kind: ConstraintKind::Mate(proposed.mate.clone()),
            })
            .collect();
        for row in &values {
            let id = base
                .iter()
                .find(|c| c.value().is_some_and(|v| v.coord == row.coord))
                .map_or_else(&mut fresh, |c| c.id);
            bundle.push(Constraint::of_value(
                id,
                Value {
                    coord: row.coord,
                    by: row.by.clone(),
                },
            ));
        }
        Ok(MateProposal {
            door: draft.door.clone(),
            placement,
            target,
            mates,
            values,
            bundle,
        })
    }
}
