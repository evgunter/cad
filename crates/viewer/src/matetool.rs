//! **The placement editor**: a modal layer-3 tool whose draft is one
//! placement's whole bundle — its mates, and the values beneath them on
//! the freedoms they leave — each change committed live as one edit
//! (GUI-4; `crates/viewer/README.md`, *A create gesture commits the
//! node*).
//!
//! # Place, then edit
//!
//! The editor edits a placement ([`PlaceDoor`]), opened from its tree
//! row, its constraint rows, or a pick on its copy. A pick on a body
//! picked as itself — one nothing places, or one the View pane shows
//! again beside its copies — is **Place** ([`MateTool::take_place`]):
//! [`SessionOp::Place`] inserts a placement of the body where it stands,
//! no mate and every slide and spin at zero, and the editor opens on it
//! ([`MateTool::placed`]). Every mate is then added in the editor.
//!
//! # The target
//!
//! The draft's first row is the placement's target, `into: <space>`
//! ([`MateTool::target`]). A placement's target is its own property and
//! no mate changes it: re-seating the row ([`MateTool::set_target`], a
//! body picked after [`MateTool::pick_target`], the world, a new space,
//! or back where it is) is the one edit that changes it, a live change
//! like any other. A target change keeps every value's number, now on the
//! new target's axes, and every mate as written: a live row whose `to`
//! reads another space than the target ([`MateTool::off_target`]) holds
//! the draft at the door, said on its row, until it is re-picked, struck
//! ([`MateTool::strike_reading`]), or the target is set to its space.
//!
//! # Live commits
//!
//! Each change to the draft — a pick seated, a kind or a sense chosen, a
//! row added or struck, a value typed — is offered to the document's
//! door once it is made ([`MateTool::offer`]), and taken, it is one
//! [`SessionOp::SetBundle`] keeping the id of every row loaded from the
//! bundle: one undo step. A draft the door refuses, or one that writes
//! no bundle yet (a seat still empty), is **held** over the document
//! with its reason ([`MateTool::pending`]) until a later change makes
//! the whole of it pass, or [`MateTool::discard`] drops it. A draft over
//! a bundle the document has since changed (an undo, a redo, an edit
//! from elsewhere) reloads, said ([`MateToolEvent::Reloaded`]), as the
//! profile editor's does.
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
//! bundle that reads a named variable or an expression, or that the mate
//! fixing a direction — the chart it is read on — still reads as loaded,
//! whose slides are then the loaded ones. Only a constant (a number, or
//! the anonymous variable a typed field mints) whose chart changed is
//! measured again. A loaded turn whose freedom goes is carried onto the
//! new freedom's turn only where the kernel states the two one rotation
//! about one line ([`editor_core::mate::spin_as_chain_turn`]), with its
//! sign and the exact quarter turns between their zeros, so `pa` on the
//! spin about a rest's normal is read as `-pa` on the chain's turn about
//! `z`. Every other value the person wrote whose freedom goes is said
//! dropped in the draft ([`MateProposal::dropped`]), with its formula,
//! as it lands, and the constants the new mates fix are said once
//! ([`MateProposal::retired`]) — none is dropped silently, and none is
//! written over with a measured number.
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
//! ([`crate::world::copy_source`]); a pick on an instance as its own
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
//! sees a stale pick refused at its next offer, typed but late.
//!
//! # The door
//!
//! The editor derives its bundle ([`MateTool::proposal`]), and the
//! session judges it as `SetBundle` would land it
//! ([`crate::session::DocSession::bundle_door`]): mates repeating an
//! equation, a freedom left unpinned, two spaces, a side off the copy, a
//! pick its kind cannot read. A refused draft stays held, and the panel
//! says why.
//!
//! Module kind: **vocabulary** — it names no driver type and no
//! `app`-only crate (`crates/viewer/README.md`, Module boundaries).

use pncad::document::{
    Constraint, ConstraintId, ConstraintKind, Dimension, Doc, Evaluation, Formula, Mate, MateRef,
    MateSide, Node, Operand, Orientation, Placed, ProfileProgram, RecipeNodeId, Said, Say,
    SitedRef, SlideAxes, SlotKind, Speaker, SpokenNode, Value, VarId, VarKind, member_of, pose,
};
use pncad::geom_core::Tol;
use pncad::prelude::StableName;
use std::sync::Arc;

/// A freedom as the tool writes it: its read, where it reads one, as
/// authored.
type Coord = pncad::document::Coord<Operand>;
use pncad::select::{EntityKind, Resolution};

use crate::pickindex::{PickKinds, PickedKind};
use crate::session::select::resolves;
use crate::session::{EdgeSelection, FaceSelection, Refusal, SessionOp, VertexSelection};
use crate::session::{PlaceTarget, PlacementBase};
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
/// reads faces alone picks faces alone, no kind is offered a profile
/// (none reads one), and a narrow shelf or a small
/// hole's wall stays pickable however near its own boundary the cursor
/// is. A pick already held that the kind chosen since does not read is
/// named by the verdict ([`MateToolError::KindDoesNotRead`]).
#[must_use]
pub fn pick_kinds(kind: VarKind) -> PickKinds {
    let read: Vec<PickedKind> = reads(kind).into_iter().filter_map(PickedKind::of).collect();
    PickKinds::ALL
        .into_iter()
        .filter(|rule| read.iter().all(|&picked| rule.admits(picked)))
        .min_by_key(|rule| {
            PickedKind::ALL
                .iter()
                .filter(|&&picked| rule.admits(picked))
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

/// A mate pick follows as the selection it was picked in does.
impl crate::session::select::Follows for MatePick {
    fn node(&self) -> RecipeNodeId {
        match self {
            Self::Face(face) => face.node(),
            Self::Edge(edge) => edge.node(),
            Self::Vertex(vertex) => vertex.node(),
        }
    }

    fn follow(&mut self, moved: &pncad::document::Moved, doc: &Doc<ProfileProgram>) {
        match self {
            Self::Face(face) => face.follow(moved, doc),
            Self::Edge(edge) => edge.follow(moved, doc),
            Self::Vertex(vertex) => vertex.follow(moved, doc),
        }
    }
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
/// direction is a face's plane read through its normal, or a round
/// face's or an edge's axis read through its direction — the kernel's own spelling of a
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
        Self::admitted(kind, seat, entity).first().copied()
    }

    /// **The reading of a pick of `entity` at `seat`, sited at `site`
    /// in `ev`**: [`Self::of`], except that a face whose carrier is no
    /// plane takes the first reading that is not its plane where one
    /// admits it — a direction picked on a cylinder's face reads its
    /// carrier's axis. The carrier is the kernel's tag
    /// ([`pncad::select::face_carrier_kind`]); a tag that does not read
    /// leaves [`Self::of`]'s reading, and the placement states its
    /// refusal.
    fn at_site(
        kind: VarKind,
        seat: MateSeat,
        entity: EntityKind,
        ev: &Evaluation<f64>,
        site: &SitedRef,
    ) -> Option<Self> {
        let admitted = Self::admitted(kind, seat, entity);
        let planeless = entity == EntityKind::Face
            && pncad::select::face_carrier_kind(ev, site.at, &site.name)
                .is_ok_and(|carrier| carrier != pncad::geom::SurfaceKind::Plane);
        admitted
            .iter()
            .copied()
            .find(|reading| planeless && reading.pose != ReadPose::Plane)
            .or_else(|| admitted.first().copied())
    }

    /// The readings of `entity` at `seat` of a mate of `kind` whose
    /// seat admits it, in preference order.
    fn admitted(kind: VarKind, seat: MateSeat, entity: EntityKind) -> Vec<Self> {
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
            .filter(|reading| reading.seat().selection_kind(entity).is_some())
            .collect()
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
    unread: bool,
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
            unread: false,
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
    fn written(
        doc: &Doc<ProfileProgram>,
        loaded: Option<ConstraintId>,
        mate: &Mate<Formula>,
    ) -> Self {
        let read = editor_core::mate::pin::operand_kind(doc, &mate.on);
        let flip = editor_core::mate::side_flipped(doc, &mate.to)
            != editor_core::mate::side_flipped(doc, &mate.on);
        let written = |side: &Operand| MateSideDraft {
            written: Some(side.clone()),
            ..MateSideDraft::default()
        };
        Self {
            // A kind that does not resolve is said, never guessed: the
            // row keeps its sides as written and its bundle refuses
            // (`MateToolError::KindUnread`) until it is struck out.
            kind: read.unwrap_or(VarKind::Plane),
            unread: read.is_none(),
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

    /// **The space the row's `to` reads**: its side as written, or the
    /// body its first held pick reads; `None` while no seat is held.
    #[must_use]
    pub fn to_space(&self, doc: &Doc<ProfileProgram>, ev: &Evaluation<f64>) -> Option<VarId> {
        match &self.to.written {
            Some(written) => doc.operand_space(written),
            None => {
                let pick = self.to.picks().next()?;
                let (_, read) = to_read(doc, Some(ev), pick.pick())?;
                doc.operand_space(&read)
            }
        }
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

/// **The placement the editor edits**, and the bundle its draft was
/// loaded from.
#[derive(Debug, Clone, PartialEq)]
pub struct PlaceDoor {
    /// The placement.
    pub placement: RecipeNodeId,
    /// Its target and bundle as authored when the draft was loaded —
    /// what tells the editor the document has moved under it.
    pub base: PlacementBase,
}

/// **A value row**: a freedom the live mates leave, its number, and
/// where the number came from.
#[derive(Debug, Clone, PartialEq)]
pub struct ValueRow {
    /// The freedom.
    pub coord: Coord,
    /// Its number, as the bundle writes it.
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
    /// Nowhere: the mates' poses do not evaluate where the copy stands,
    /// so the freedom reads no number, and the bundle holds the kernel's
    /// fill (the constant `0`, [`Placed::of`]) until a number is typed.
    Unmeasured,
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

/// The draft: its door, its mate rows, what the person set, the rows
/// it was loaded with, and where its last change stands against the
/// document.
#[derive(Debug, Clone, PartialEq)]
struct PlaceDraft {
    door: PlaceDoor,
    rows: Vec<MateRow>,
    /// The rows as the bundle loaded them: what the draft differs from.
    loaded_rows: Vec<MateRow>,
    typed: Vec<(Coord, Formula)>,
    active: usize,
    /// The space the copy is placed into: the loaded target until the
    /// first row is re-seated.
    target: PlaceTarget,
    /// The target the placement held when the draft was loaded.
    loaded_target: PlaceTarget,
    /// The loaded values struck out for reading another space than the
    /// target ([`MateTool::strike_reading`]): their freedoms are measured
    /// afresh.
    struck_values: Vec<ConstraintId>,
    /// What a target change of its own set aside
    /// ([`MateTool::retarget_into`]), made again once it lands and put
    /// back at once on every other answer of the offer.
    held_back: Option<SetAside>,
    offer: Offer,
}

impl PlaceDraft {
    /// Whether the rows, the numbers or the struck values differ from
    /// what was loaded.
    fn differs_in_rows(&self) -> bool {
        self.rows != self.loaded_rows || !self.typed.is_empty() || !self.struck_values.is_empty()
    }
}

/// **What a target change of its own sets aside**: the draft's rows,
/// numbers, struck values and active row, and the changes made after it
/// while it waited on an earlier change landing.
#[derive(Debug, Clone, PartialEq)]
struct SetAside {
    rows: Vec<MateRow>,
    typed: Vec<(Coord, Formula)>,
    struck_values: Vec<ConstraintId>,
    active: usize,
    then: Vec<Change>,
}

/// **What reads a space in the draft**: a live mate row's `to`, or a
/// loaded value along a direction or about an axis.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Reading {
    /// The mate row at this index.
    Mate(usize),
    /// The loaded value of this constraint.
    Value(ConstraintId),
}

/// **Why a draft is held**: a refusal, one value however often it is
/// said — two held drafts are the same state when they hold the same
/// refusal, not two equal ones.
#[derive(Debug, Clone)]
pub struct Held(Arc<Refusal>);

impl PartialEq for Held {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.0, &other.0)
    }
}

/// **One change to the draft**, as the person makes it: made now, or,
/// while the last change lands, made over the bundle it writes
/// ([`crate::offer::Offer::defer`]).
#[derive(Debug, Clone, PartialEq)]
enum Change {
    Pick(MatePick),
    AddMate(VarKind),
    AddWritten(Mate<Formula>),
    ChooseKind(usize, VarKind),
    Sense(usize, bool),
    Strike(usize, bool),
    StrikeReading(Vec<Reading>),
    Type(Coord, Formula),
    Target(PlaceTarget),
    /// `Into <space>` made while an earlier change lands: made over what
    /// it writes as a target change of its own.
    IntoAlone(VarId),
    /// What a target change of its own set aside, put back whole.
    PutBack(Box<SetAside>),
}

/// **Where the draft stands with the document** — the live editors' one
/// state machine ([`crate::offer::Offer`]). A landed change reloads the
/// draft at once ([`MateTool::landed`]), so `Landed` is not held here.
type Offer = crate::offer::Offer<Held, Change, ()>;

/// **The placement editor.** A value: the chrome holds one while the
/// tool is open, a test constructs one and drives the same methods.
#[derive(Debug, Clone, PartialEq)]
pub struct MateTool {
    draft: Option<PlaceDraft>,
    /// The body a pick asked to place and the space it stands in, until
    /// the Place is taken ([`Self::take_place`]).
    placing: Option<(Operand, VarId)>,
    /// The kind a fresh row opens with.
    kind: VarKind,
    /// Whether the next pick names the target's space rather than
    /// seating a mate ([`Self::pick_target`]).
    picking_target: bool,
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
    /// A pick on a drawn family is read in no one member of it: a side
    /// reads one body (REFERENCES DM3), and the landed evaluation finds
    /// no member the pick lies in.
    MemberUnread {
        /// Which side.
        side: MateSide,
        /// The family.
        node: SpokenNode,
    },
    /// A node the editor was asked to open on is no placement.
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
    /// A freedom's measured number is no literal a value can hold.
    Unwritable {
        /// The freedom.
        coord: Coord,
        /// The number.
        by: f64,
    },
    /// A loaded mate row's kind does not resolve.
    KindUnread {
        /// The row, 0-based.
        row: usize,
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
            Self::MemberUnread { side, node } => write!(
                f,
                "the {} pick is on {node}, a family, and a side reads one body: pick on one \
                 drawn member of it, once the document has landed",
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
            Self::Unwritable { coord, by } => {
                write!(
                    f,
                    "the {} measures {by}, which no value holds",
                    coord.name()
                )
            }
            Self::KindUnread { row } => write!(
                f,
                "mate {}'s kind does not resolve; strike it out or pick it again",
                row + 1
            ),
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
            Self::MemberUnread { side, node } => Self::MemberUnread {
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
            | Self::Unwritable { .. }
            | Self::KindUnread { .. }) => unspoken,
        }
    }
}

/// **The sentence a dropped pick is said in**: its role, and the pick
/// as its panel line said it ([`HeldPick`]) — the editor's drop and the
/// mirror tool's (`crate::combine::PlaneEvent`).
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
    /// The bundle the editor loaded is not the one the placement
    /// holds now (an undo, a redo, an edit from elsewhere): the draft
    /// was loaded again, and what it held is gone.
    Reloaded {
        /// The placement.
        placement: SpokenNode,
    },
    /// The placement the editor was on is gone: it holds no
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
    /// The pick is on a split, picked whole: which half is the body.
    HalfUnchosen {
        /// The split.
        split: SpokenNode,
    },
    /// The Transform tool's pick is on a pattern's family, whose values
    /// are its rule: a member moves by the rule, edited on the pattern.
    PatternFamily {
        /// The family's placement.
        family: SpokenNode,
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
            Self::HalfUnchosen { split } => {
                write!(f, "{split} is two bodies; choose the half to move")
            }
            Self::PatternFamily { family } => write!(
                f,
                "{family} is a pattern's members, which its rule moves; edit the pattern"
            ),
        }
    }
}

impl core::fmt::Display for MateToolEvent {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::PickLost { pick, .. } => self.say(f, Speaker::held(pick.said())),
            Self::Reloaded { .. }
            | Self::PlacementGone { .. }
            | Self::NotPlaceable { .. }
            | Self::HalfUnchosen { .. }
            | Self::PatternFamily { .. } => self.say(f, Speaker::TAG),
        }
    }
}

/// **One mate of the draft's bundle**: the mate, and the entities its seats were
/// picked on.
#[derive(Debug, Clone, PartialEq)]
pub struct ProposedMate {
    /// The mate as the bundle writes it.
    pub mate: Mate<Formula>,
    /// The `on` seats' entities, read off the copy's shapes; empty for a
    /// side as the bundle wrote it.
    pub on: Vec<SitedRef>,
    /// The `to` seats' entities, read where they were picked; empty for
    /// a side as the bundle wrote it.
    pub to: Vec<SitedRef>,
}

/// **The editor's bundle, derived**: the door, the live mates, and the
/// bundle with its value rows.
#[derive(Debug, Clone, PartialEq)]
pub struct MateProposal {
    /// The door: the placement the bundle is written into, and the
    /// bundle the draft was loaded from.
    pub door: PlaceDoor,
    /// The live mates, in row order.
    pub mates: Vec<ProposedMate>,
    /// The value rows.
    pub values: Vec<ValueRow>,
    /// **The values the bundle drops that the person wrote**: each
    /// loaded value reading a variable or an expression, and each typed
    /// number, on a freedom the live mates do not leave and carried onto
    /// none, with its formula — said prominently in the draft, never
    /// dropped silently.
    pub dropped: Vec<ValueRow>,
    /// **The constants the bundle drops**: numbers on freedoms the live
    /// mates now fix, said once, quietly.
    pub retired: Vec<ValueRow>,
    /// The target the copy is placed into.
    pub target: PlaceTarget,
    /// The bundle written: the mates, then the values.
    pub bundle: Vec<Constraint<Formula>>,
}

impl MateProposal {
    /// **The one committed action**: the target and the bundle written
    /// whole ([`SessionOp::SetBundle`]). A mate whose `to` reads outside
    /// the target is refused there, naming both spaces.
    #[must_use]
    pub fn op(&self) -> SessionOp {
        SessionOp::SetBundle {
            placement: self.door.placement,
            base: self.door.base.clone(),
            target: self.target.clone(),
            constraints: self.bundle.clone(),
        }
    }
}

/// **A target as the editor's first row says it**: the space by
/// [`Doc::spoken_space`], or "a new space".
#[must_use]
pub fn said_target(doc: &Doc<ProfileProgram>, target: &PlaceTarget) -> String {
    match target {
        PlaceTarget::NewSpace => "a new space".to_owned(),
        PlaceTarget::Space(space) => doc.spoken_space(*space).to_string(),
    }
}

/// **A face read as its plane**: `face`, a one-name face selection,
/// read plainly as the plane of that face — a plane mate's side, and
/// the plane a mirror reflects across (`crate::combine::mirror_node`).
pub(crate) fn plane_of(face: Operand) -> Operand {
    Operand::Pose(Box::new(pose::PoseDef::Plane { face }))
}

/// **What a pick opens the placement editor on**: a placement's
/// editor, or Place on a body.
#[derive(Debug, Clone, PartialEq)]
enum Opening {
    /// The placement whose bundle the editor writes.
    Edit(RecipeNodeId),
    /// The body a Place inserts a placement of, and the space it stands
    /// in.
    Place(Operand, VarId),
}

/// Who asks which door a pick opens.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Purpose {
    /// The Transform tool: it types values, so a pattern's family, whose
    /// values are its rule, is refused rather than opened.
    Moving,
    /// The editor's own first pick.
    Mating,
    /// Which side a pick fills once the editor is open: the same route,
    /// ungated, since a pick that could open nothing is still read as on
    /// the copy or off it.
    Siding,
}

/// **The one rule for what a pick on `node` opens** (`body` the output a
/// viewport pick hit, `name` the entity it named), shared by the
/// editor's first pick, its `on`/`to` routing and the Transform tool:
///
/// - a placement or an instance, or a member A11's walk reaches through
///   `name` ([`member_of`]): its editor — a pick that lands on a copy
///   edits that copy's placement;
/// - a body picked as itself: Place, whether nothing places it yet or
///   the View pane shows it again beside its copies, so a second
///   placement is made the way the first one is;
/// - a split picked whole: [`MateToolEvent::HalfUnchosen`]; anything else
///   that is no body: [`MateToolEvent::NotPlaceable`].
///
/// Moving, a family a pattern writes refuses
/// [`MateToolEvent::PatternFamily`]: its values are the pattern's rule.
fn door_on(
    doc: &Doc<ProfileProgram>,
    node: RecipeNodeId,
    body: Option<u32>,
    name: Option<&StableName>,
    purpose: Purpose,
) -> Result<Opening, Box<MateToolEvent>> {
    let refused = |event| Err(Box::new(event));
    let edit = |placement: RecipeNodeId| {
        if purpose == Purpose::Moving && crate::world::repeats(doc, placement) {
            return refused(MateToolEvent::PatternFamily {
                family: doc.spoken(placement),
            });
        }
        Ok(Opening::Edit(placement))
    };
    let gated = purpose != Purpose::Siding;
    match doc.node(node) {
        None if gated => {
            return refused(MateToolEvent::NotPlaceable {
                node: doc.spoken(node),
            });
        }
        Some(Node::Place { .. } | Node::InstantiatePart { .. }) => return edit(node),
        Some(Node::Split { .. }) if gated && body.is_none() => {
            return refused(MateToolEvent::HalfUnchosen {
                split: doc.spoken(node),
            });
        }
        _ => {}
    }
    // The walk's head may be a construction, which is no placement.
    if let Some(instance) = name
        .and_then(|name| member_of(doc, &SitedRef::new(node, name.clone())))
        .map(|member| member.instance)
        .filter(|&at| {
            matches!(
                doc.node(at),
                Some(Node::Place { .. } | Node::InstantiatePart { .. })
            )
        })
    {
        return edit(instance);
    }
    let read = seated_read(doc, node, body);
    let space = doc.operand_space(&read);
    match space {
        Some(space)
            if !gated
                || crate::session::admits_read(
                    doc,
                    &read,
                    crate::session::NodeKindWanted::Body,
                ) =>
        {
            Ok(Opening::Place(read, space))
        }
        _ => refused(MateToolEvent::NotPlaceable {
            node: doc.spoken(node),
        }),
    }
}

/// **The body a pick on `node` reads** (`body` the output a viewport
/// pick hit): the output the seat reads, as a body — what Place places
/// and whose space the target row's pick names.
fn seated_read(doc: &Doc<ProfileProgram>, node: RecipeNodeId, body: Option<u32>) -> Operand {
    let (at, port) = crate::world::seat_read(doc, node, body);
    crate::session::as_body(doc, crate::seats::Output::named(port).read(at))
}

/// [`door_on`] of an entity pick.
fn door_picked(
    doc: &Doc<ProfileProgram>,
    pick: &MatePick,
    purpose: Purpose,
) -> Result<Opening, Box<MateToolEvent>> {
    door_on(
        doc,
        pick.node(),
        Some(pick.on().body()),
        Some(pick.name()),
        purpose,
    )
}

/// **The entity `pick` names as an `on` side reads it**: off the shape
/// a `Place` copies, by the shape's own name for it (a copy carries its
/// body's names), on the output it copies where the shape's operation
/// defines several (a split's halves) ([`crate::world::copy_source`]);
/// an instance's or a body's own read otherwise, and a copy whose body
/// does not resolve as itself.
fn on_read(
    doc: &Doc<ProfileProgram>,
    ev: Option<&pncad::document::Evaluation<f64>>,
    pick: &MatePick,
) -> Option<(SitedRef, Operand)> {
    let picked = SitedRef::new(pick.node(), pick.name().clone());
    match doc.node(pick.node()) {
        Some(Node::Place { .. }) => {
            let (at, port) = crate::world::copy_source(doc, pick.node(), pick.on().body())
                .map_or((pick.node(), pick.on().body()), |copied| {
                    (copied.operation, copied.body)
                });
            let (at, port) = crate::world::seat_read(doc, at, Some(port));
            let body = crate::seats::Output::named(port).read(at);
            Some((
                SitedRef::new(at, pick.name().clone()),
                picked_read(doc, ev, body, pick)?,
            ))
        }
        Some(Node::InstantiatePart { .. }) => {
            let read = picked_read(doc, ev, Operand::Node(pick.node()), pick)?;
            Some((picked, read))
        }
        _ => Some((picked, crate::session::pick_read(doc, ev, pick)?)),
    }
}

/// **The entity `pick` names as a `to` side reads it**: on a copy, the
/// copy itself; on a drawn operation, the output picked
/// ([`crate::session::pick_read`]). Each reads one body
/// ([`crate::session::picked_selection`]): `None` where the pick is on a
/// family whose member no evaluation finds.
fn to_read(
    doc: &Doc<ProfileProgram>,
    ev: Option<&pncad::document::Evaluation<f64>>,
    pick: &MatePick,
) -> Option<(SitedRef, Operand)> {
    let picked = SitedRef::new(pick.node(), pick.name().clone());
    let read = match doc.node(pick.node()) {
        Some(Node::Place { .. } | Node::InstantiatePart { .. }) => {
            picked_read(doc, ev, Operand::Node(pick.node()), pick)?
        }
        _ => crate::session::pick_read(doc, ev, pick)?,
    };
    Some((picked, read))
}

/// The one-name selection of `pick`'s entity in `body`, read as one
/// body ([`crate::session::picked_selection`]).
fn picked_read(
    doc: &Doc<ProfileProgram>,
    ev: Option<&pncad::document::Evaluation<f64>>,
    body: Operand,
    pick: &MatePick,
) -> Option<Operand> {
    let (body, names) = crate::session::picked_selection(doc, ev, body, vec![pick.name().clone()])?;
    Some(Operand::select(body, names))
}

/// **Whether the value `id` of `placement`'s bundle is a number**: the
/// anonymous variable a typed field mints ([`Doc::is_typed_value`]), or
/// a constant as written (the `0` a bundle door fills in). A value
/// reading a named variable or an expression is the person's, and no
/// snap writes over it.
fn written_constant(doc: &Doc<ProfileProgram>, placement: RecipeNodeId, id: ConstraintId) -> bool {
    let Some(Node::Place { constraints, .. } | Node::InstantiatePart { constraints, .. }) =
        doc.node(placement)
    else {
        return false;
    };
    constraints
        .iter()
        .find(|c| c.id == id)
        .and_then(|c| c.value())
        .is_some_and(|v| {
            let written = doc.written(&pncad::document::Expr::var(v.by, v.coord.dimension()));
            doc.is_typed_value(v.by) || written.is_zero() || written.literal_value().is_some()
        })
}

/// A length or an angle as the value that writes it: the constant `0`
/// where it is zero; `None` for a number no literal holds.
pub(crate) fn number(by: f64, dim: Dimension) -> Option<Formula> {
    if by == 0.0 {
        Some(Formula::zero(dim))
    } else {
        Formula::literal(by, dim).ok()
    }
}

/// **The loaded turn carried onto the turn freedom `coord`**, where the
/// kernel states the two one rotation about one line
/// ([`editor_core::mate::spin_as_chain_turn`]): the spin about a loaded
/// chart's direction onto the new chain's `z`, or a loaded chain's `z`
/// onto the new chart's spin. The value is written `s·v + q·¼`, `s` the
/// sign and `q` the exact quarter turns between the two freedoms' zeros;
/// onto a chart, only where the loaded chain's other turns are the
/// chart's own, exactly, so the copy stands where the snap stands it.
/// `None` where no loaded value carries; a constant never does, it is
/// measured again.
///
/// # Errors
///
/// [`MateToolError::Unwritable`] where the quarter turns write no value.
#[allow(clippy::too_many_arguments)]
fn carried_turn(
    doc: &Doc<ProfileProgram>,
    ev: &Evaluation<f64>,
    tol: Tol,
    coord: Coord,
    coords: &[Coord],
    loaded: &[(ConstraintId, &Value<Formula>)],
    landed: &[ConstraintId],
    snap: Option<&crate::session::snap::DraftSnap>,
    placement: RecipeNodeId,
    base_chart: Option<ConstraintId>,
    orient: Option<Orientation>,
) -> Option<Result<(ConstraintId, Formula), MateToolError>> {
    let free = |id: &ConstraintId, v: &Value<Formula>| {
        !landed.contains(id) && !coords.contains(&v.coord) && !written_constant(doc, placement, *id)
    };
    let (id, v, turn, onto_chart) = match coord {
        // A spin about a loaded chart's direction onto the new chain.
        Coord::Spin(Some(axis)) => {
            let (id, v) = loaded
                .iter()
                .find(|(id, v)| v.coord == Coord::SPIN && free(id, v))?;
            let turn = crate::session::snap::loaded_chain_turn(
                doc,
                ev,
                placement,
                base_chart?,
                coords,
                tol,
            )?;
            (turn.axis == axis).then_some((*id, *v, turn, false))?
        }
        // A loaded chain's last turn onto the new chart's spin.
        Coord::Spin(None) => {
            let chain: Vec<Coord> = loaded.iter().map(|(_, v)| v.coord.clone()).collect();
            let turn = snap?.chain_turn(orient, &chain)?;
            let (id, v) = loaded
                .iter()
                .find(|(id, v)| v.coord == Coord::Spin(Some(turn.axis)) && free(id, v))?;
            // The chain's other turns must be the chart's own, exactly.
            let inner = [
                (TargetAxisX, turn.quarters[0]),
                (TargetAxisY, turn.quarters[1]),
            ];
            let exact = inner.iter().all(|&(axis, q)| {
                loaded
                    .iter()
                    .find(|(_, w)| w.coord == Coord::Spin(Some(axis)))
                    .and_then(|(_, w)| {
                        let held = doc.resolve(&w.by).ok()?;
                        pncad::document::eval::<f64>(&held, &doc.var_env()).ok()
                    })
                    .is_some_and(|at| at == quarter_angle(q))
            });
            exact.then_some((*id, *v, turn, true))?
        }
        Coord::Slide(_) | Coord::SlideAlong(_) | Coord::SpinAbout(_) => return None,
    };
    let signed = if turn.sign > 0 {
        Some(v.by.clone())
    } else {
        Formula::neg(v.by.clone()).ok()
    };
    // Onto the chain: `γ = s·θ + q_z`; onto the chart: `θ = s·(γ − q_z)`.
    let q = if onto_chart {
        -turn.sign * turn.quarters[2]
    } else {
        turn.quarters[2]
    };
    let carried = signed.and_then(|signed| match quarter_turns(q) {
        None => Some(signed),
        Some(offset) => Formula::add(signed, offset.ok()?).ok(),
    });
    Some(carried.map(|by| (id, by)).ok_or(MateToolError::Unwritable {
        coord,
        by: quarter_angle(q),
    }))
}

use pncad::document::TargetAxis::{X as TargetAxisX, Y as TargetAxisY};

/// `q` quarter turns as an angle, exactly as the chain read writes it.
fn quarter_angle(q: i32) -> f64 {
    match q.rem_euclid(4) {
        0 => 0.0,
        1 => core::f64::consts::FRAC_PI_2,
        2 => core::f64::consts::PI,
        _ => -core::f64::consts::FRAC_PI_2,
    }
}

/// **`q` quarter turns as a value**, written exactly (`turn/4`, `turn/2`,
/// `-turn/4`); `None` for none.
fn quarter_turns(q: i32) -> Option<Result<Formula, pncad::document::DimensionError>> {
    let turn = |den| Formula::turn_over(den);
    match q.rem_euclid(4) {
        0 => None,
        1 => Some(turn(4)),
        2 => Some(turn(2)),
        _ => Some(turn(4).and_then(Formula::neg)),
    }
}

/// **A value row's formula as the person reads it**: a named variable
/// by its name, a typed number as the number and unit it was typed in,
/// an expression as written ([`Doc::unparse`]) — never a variable's raw
/// id. What the panel's field shows and a typed edit starts from.
#[must_use]
pub fn value_text(doc: &Doc<ProfileProgram>, by: &Formula) -> String {
    doc.unparse(by)
}

/// **What a person typed in `coord`'s field, as its value**: the text
/// read by the one parser over the document's variables, at the
/// freedom's dimension.
///
/// # Errors
///
/// [`TypedValueError`]: text that does not parse, or a quantity of
/// another dimension.
pub fn typed_value(
    doc: &Doc<ProfileProgram>,
    coord: Coord,
    text: &str,
) -> Result<Formula, TypedValueError> {
    let by = pncad::document::parse_formula(text, &doc.var_scope())
        .map_err(|error| TypedValueError::Parse(Box::new(error)))?;
    let wanted = coord.dimension();
    // A bare `0` is every dimension's zero: the text a zero value's field
    // shows ([`value_text`]) reads back as that zero.
    if by.dim() != wanted && (by == Formula::count(0) || by.literal_value() == Some(0.0)) {
        return Ok(Formula::zero(wanted));
    }
    if by.dim() == wanted {
        Ok(by)
    } else {
        Err(TypedValueError::Dimension {
            coord,
            found: by.dim(),
        })
    }
}

/// Why a typed value field's text is no value (closed enum, D4 ¶3).
#[derive(Debug)]
pub enum TypedValueError {
    /// The text does not parse.
    Parse(Box<pncad::document::ParseError>),
    /// The text is a quantity of another dimension than the freedom's.
    Dimension {
        /// The freedom.
        coord: Coord,
        /// The dimension typed.
        found: Dimension,
    },
}

impl core::fmt::Display for TypedValueError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Parse(error) => write!(f, "{error}"),
            Self::Dimension { coord, found } => write!(
                f,
                "the {} is a {}, and the text is a {found}",
                coord.name(),
                coord.dimension()
            ),
        }
    }
}

impl core::error::Error for TypedValueError {}

/// Whether a mate of `kind` fixes one direction — the chart's mate, by
/// the kernel's family rule ([`pncad::document::SubgroupFamily::fixes_one_direction`]).
fn fixes_one_direction(kind: VarKind) -> bool {
    kind.symmetry()
        .is_some_and(pncad::document::SubgroupFamily::fixes_one_direction)
}

impl MateTool {
    /// An editor holding nothing: the first pick opens it.
    #[must_use]
    pub fn new() -> Self {
        Self {
            draft: None,
            placing: None,
            kind: VarKind::Plane,
            picking_target: false,
        }
    }

    /// **The editor on `placement`**, its bundle loaded as authored:
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
        Ok(Self::loaded(doc, placement))
    }

    /// The editor on `placement`, a node [`Self::edit`] has checked is
    /// one.
    fn loaded(doc: &Doc<ProfileProgram>, placement: RecipeNodeId) -> Self {
        let Some(base) = crate::session::held_placement(doc, placement) else {
            unreachable!("`Self::edit` checked the node is a placement")
        };
        let rows: Vec<MateRow> = base
            .constraints
            .iter()
            .filter_map(|c| c.mate().map(|m| MateRow::written(doc, Some(c.id), m)))
            .collect();
        let Some(space) = doc.node_space(placement) else {
            unreachable!("an admitted placement's target reads a space")
        };
        Self {
            draft: Some(PlaceDraft {
                target: PlaceTarget::Space(space),
                loaded_target: PlaceTarget::Space(space),
                door: PlaceDoor { placement, base },
                loaded_rows: rows.clone(),
                rows,
                typed: Vec::new(),
                active: 0,
                struck_values: Vec::new(),
                held_back: None,
                offer: Offer::Quiet,
            }),
            ..Self::new()
        }
    }

    /// What `opening` names: the editor on a placement, or a Place
    /// waiting to be taken.
    fn opened(doc: &Doc<ProfileProgram>, opening: Opening) -> Self {
        match opening {
            Opening::Edit(placement) => Self::loaded(doc, placement),
            Opening::Place(body, space) => Self {
                placing: Some((body, space)),
                ..Self::new()
            },
        }
    }

    /// **The Transform tool's door on `node`**, a body or a copy picked
    /// in the tree (`body` the output a viewport pick hit), opened with no
    /// mate row: the editor on a copy, and Place on a body picked as
    /// itself ([`door_on`]'s rule).
    ///
    /// # Errors
    ///
    /// The event saying why nothing opens: [`MateToolEvent::NotPlaceable`]
    /// for a node that is no body, [`MateToolEvent::HalfUnchosen`] for a
    /// split picked whole, [`MateToolEvent::PatternFamily`] for a
    /// pattern's family.
    pub fn moving(
        doc: &Doc<ProfileProgram>,
        node: RecipeNodeId,
        body: Option<u32>,
    ) -> Result<Self, Box<MateToolEvent>> {
        door_on(doc, node, body, None, Purpose::Moving).map(|opening| Self::opened(doc, opening))
    }

    /// [`Self::moving`] of a viewport pick, which names the entity it hit,
    /// so a member of an instance reached through a union's name opens
    /// that instance's editor.
    ///
    /// # Errors
    ///
    /// As [`Self::moving`]'s.
    pub fn moving_picked(
        doc: &Doc<ProfileProgram>,
        pick: impl Into<MatePick>,
    ) -> Result<Self, Box<MateToolEvent>> {
        door_picked(doc, &pick.into(), Purpose::Moving).map(|opening| Self::opened(doc, opening))
    }

    /// **The Place a pick asked for**, taken: one placement of the body
    /// into the space it stands in ([`SessionOp::Place`]). The editor
    /// opens on the placement it makes ([`Self::placed`]).
    pub fn take_place(&mut self) -> Option<SessionOp> {
        self.placing.take().map(|(body, space)| SessionOp::Place {
            body,
            target: Operand::Var(space),
        })
    }

    /// **A Place landed**: the editor opens on `placement`, the
    /// placement it made, with no mate row — every mate is added here.
    pub fn placed(&mut self, doc: &Doc<ProfileProgram>, placement: RecipeNodeId) {
        self.reload(doc, placement);
    }

    /// **The editor on `placement` loaded afresh from `doc`**, keeping
    /// the kind a fresh row opens with — every reload's one spelling.
    /// Answers whether `placement` is one; the editor holds nothing when
    /// it is not.
    fn reload(&mut self, doc: &Doc<ProfileProgram>, placement: RecipeNodeId) -> bool {
        let kind = self.kind;
        match Self::edit(doc, placement) {
            Ok(reloaded) => {
                *self = Self { kind, ..reloaded };
                true
            }
            Err(_) => {
                self.draft = None;
                false
            }
        }
    }

    /// **The editor on `mate`'s placement, that mate struck out** —
    /// the tree's `remove` on a mate row, a change offered as every
    /// other is: the freed values are measured where the copy stands.
    ///
    /// # Errors
    ///
    /// As [`Self::edit`]'s, and [`MateToolError::NotAMate`] for a
    /// constraint that is no mate of the bundle.
    pub fn edit_striking(doc: &Doc<ProfileProgram>, mate: MateRef) -> Result<Self, MateToolError> {
        let mut tool = Self::edit(doc, mate.placement)?;
        let Some(draft) = tool.draft.as_mut() else {
            unreachable!("the editor holds a draft")
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
        draft.offer.changed();
        Ok(tool)
    }

    /// **A row holding `mate` as written**, made the active one — a
    /// mate an API caller states rather than picks
    /// ([`SessionOp::AddMate`]).
    pub fn add_written(&mut self, doc: &Doc<ProfileProgram>, mate: &Mate<Formula>) {
        self.make(Some(doc), Change::AddWritten(mate.clone()));
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

    /// **The active row's first seats, `on` then `to`** — what the
    /// viewport marks as held: a frame's axis seats.
    #[must_use]
    pub fn active_picks(&self) -> [Option<&HeldPick<MatePick>>; 2] {
        match self.rows().get(self.active()) {
            Some(row) => [MateSide::On, MateSide::To].map(|side| row.side(side).first.as_ref()),
            None => [None, None],
        }
    }

    /// The numbers the person typed.
    #[must_use]
    pub fn typed(&self) -> &[(Coord, Formula)] {
        self.draft.as_ref().map_or(&[], |draft| &draft.typed)
    }

    /// **The draft's target**: the space the copy is placed into, the
    /// editor's first row.
    #[must_use]
    pub fn target(&self) -> Option<&PlaceTarget> {
        self.draft.as_ref().map(|draft| &draft.target)
    }

    /// **Re-seat the first row**: the copy placed into `target`, one
    /// change offered as every other is. Each value keeps its number and
    /// each mate is kept as written.
    pub fn set_target(&mut self, target: PlaceTarget) {
        self.picking_target = false;
        self.make(None, Change::Target(target));
    }

    /// **`Into <space>`**, offered beside what reads `space`: a target
    /// change of its own. The draft's other changes are set aside while
    /// the target alone is offered, and made again once it lands, so the
    /// target and the held mate are two steps; on any other answer of
    /// the offer (refused, writing no bundle, writing nothing) they are
    /// put back at once and the whole draft is offered over the new
    /// target. Into the loaded target is the target set back, made with
    /// the rest of the draft. Made while an earlier change lands, it
    /// waits on it and is then a step of its own over what it wrote.
    pub fn retarget_into(&mut self, space: VarId) {
        let target = PlaceTarget::space(space);
        let Some(draft) = self.draft.as_mut() else {
            return;
        };
        if draft.offer.landing().is_some() {
            self.picking_target = false;
            self.make(None, Change::IntoAlone(space));
            return;
        }
        if target != draft.target && target != draft.loaded_target && draft.differs_in_rows() {
            draft.held_back = Some(SetAside {
                rows: core::mem::replace(&mut draft.rows, draft.loaded_rows.clone()),
                typed: core::mem::take(&mut draft.typed),
                struck_values: core::mem::take(&mut draft.struck_values),
                active: core::mem::take(&mut draft.active),
                then: Vec::new(),
            });
        }
        self.set_target(target);
    }

    /// **The next pick names the target**: the space of the body it lands
    /// on, rather than a mate's seat ([`Self::pick`]).
    pub fn pick_target(&mut self) {
        self.picking_target = self.draft.is_some();
    }

    /// Whether the next pick names the target ([`Self::pick_target`]).
    #[must_use]
    pub fn picking_target(&self) -> bool {
        self.picking_target
    }

    /// **The space `target` names**: `None` for a new space, which no
    /// read is in yet.
    #[must_use]
    pub fn target_space(target: &PlaceTarget) -> Option<VarId> {
        match target {
            PlaceTarget::Space(space) => Some(*space),
            PlaceTarget::NewSpace => None,
        }
    }

    /// The target the placement held when the draft was loaded.
    #[must_use]
    pub fn loaded_target(&self) -> Option<&PlaceTarget> {
        self.draft.as_ref().map(|draft| &draft.loaded_target)
    }

    /// **What in the draft reads another space than its target**, each
    /// with that space: a live mate row whose `to` does (a `to` picked in
    /// another space, or a mate a target change left reading the old
    /// one), and a loaded value along a direction or about an axis read
    /// off a body there ([`crate::session::read_spaces`]). A mate is a
    /// constraint of one placement and reads its `to` in that placement's
    /// target, and a value reads its direction there too, so each holds
    /// the draft at the door.
    #[must_use]
    pub fn off_target(
        &self,
        doc: &Doc<ProfileProgram>,
        ev: &Evaluation<f64>,
    ) -> Vec<(Reading, VarId)> {
        let Some(draft) = self.draft.as_ref() else {
            return Vec::new();
        };
        let target = Self::target_space(&draft.target);
        let mates = draft
            .rows
            .iter()
            .enumerate()
            .filter(|(_, row)| !row.struck)
            .filter_map(|(at, row)| Some((Reading::Mate(at), row.to_space(doc, ev)?)));
        let values = draft
            .door
            .base
            .constraints
            .iter()
            .filter(|c| c.value().is_some() && !draft.struck_values.contains(&c.id))
            .flat_map(|c| {
                crate::session::read_spaces(doc, c)
                    .into_iter()
                    .map(|space| (Reading::Value(c.id), space))
            });
        mates
            .chain(values)
            .filter(|&(_, space)| Some(space) != target)
            .collect()
    }

    /// **Strike everything in the draft that reads `space`**, as one
    /// change (`strike the N that read <space>`): each mate row struck
    /// out, and each loaded value read off `space` struck so its freedom
    /// is measured afresh; the freed values are measured where the copy
    /// stands, as `remove` measures them.
    pub fn strike_reading(
        &mut self,
        doc: &Doc<ProfileProgram>,
        ev: &Evaluation<f64>,
        space: VarId,
    ) {
        let mut reading: Vec<Reading> = Vec::new();
        for (at, read) in self.off_target(doc, ev) {
            if read == space && !reading.contains(&at) {
                reading.push(at);
            }
        }
        if !reading.is_empty() {
            self.make(None, Change::StrikeReading(reading));
        }
    }

    /// The loaded values struck out for reading another space.
    #[must_use]
    pub fn struck_values(&self) -> &[ConstraintId] {
        self.draft
            .as_ref()
            .map_or(&[], |draft| &draft.struck_values)
    }

    /// **Feed one pick**, consumed into tool state. With nothing open,
    /// a pick on a copy opens the editor on its placement and fills the
    /// `on` seat of a fresh row, and a pick on a body picked as itself
    /// asks for Place ([`Self::take_place`]). With the editor open, a
    /// pick on the draft's copy fills an `on` seat of the active row and
    /// any other pick a `to` seat ([`MateSeat::filled_by`]), replacing
    /// what the seat held. A pick that opens nothing is said
    /// ([`MateToolEvent::NotPlaceable`]).
    pub fn pick(
        &mut self,
        doc: &Doc<ProfileProgram>,
        pick: impl Into<MatePick>,
    ) -> Option<MateToolEvent> {
        let pick = pick.into();
        if self.picking_target && self.draft.is_some() {
            let read = seated_read(doc, pick.node(), Some(pick.on().body()));
            let Some(space) = doc.operand_space(&read) else {
                self.picking_target = false;
                return Some(MateToolEvent::NotPlaceable {
                    node: doc.spoken(pick.node()),
                });
            };
            self.set_target(PlaceTarget::space(space));
            return None;
        }
        if self.draft.is_none() {
            match door_picked(doc, &pick, Purpose::Mating) {
                Ok(Opening::Place(body, space)) => {
                    self.placing = Some((body, space));
                    return None;
                }
                Ok(Opening::Edit(placement)) => {
                    self.reload(doc, placement);
                    self.add_mate(self.kind);
                }
                Err(event) => return Some(*event),
            }
        }
        self.make(Some(doc), Change::Pick(pick));
        None
    }

    /// Whether `pick` is on the draft's copy.
    fn is_on_copy(&self, doc: &Doc<ProfileProgram>, pick: &MatePick) -> bool {
        let Ok(opens) = door_picked(doc, pick, Purpose::Siding) else {
            return false;
        };
        self.door()
            .is_some_and(|door| opens == Opening::Edit(door.placement))
    }

    /// **A fresh mate row of `kind`**, made the active one: a mate in
    /// place of the slides and the spin the earlier ones leave.
    pub fn add_mate(&mut self, kind: VarKind) {
        self.kind = kind;
        self.make(None, Change::AddMate(kind));
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
        self.make(None, Change::ChooseKind(row, kind));
    }

    /// Set row `row`'s sense: whether its `to` is turned round. A kind
    /// with no sense keeps none.
    pub fn set_sense(&mut self, row: usize, flip: bool) {
        self.make(None, Change::Sense(row, flip));
    }

    /// Strike row `row` out, or take the stroke off.
    pub fn strike(&mut self, row: usize, struck: bool) {
        self.make(None, Change::Strike(row, struck));
    }

    /// **Type `by` on `coord`**: the number the bundle writes on that
    /// freedom while the mates leave it, whatever it held.
    pub fn type_value(&mut self, coord: Coord, by: Formula) {
        self.make(None, Change::Type(coord, by));
    }

    /// **`change`, made to the draft**: now, and then waiting to be
    /// offered — or, while the last change lands, held to be made over
    /// the bundle it writes ([`Self::landed`]). `doc` is the document a
    /// pick or a written mate is read in; with none, such a change waits.
    fn make(&mut self, doc: Option<&Doc<ProfileProgram>>, change: Change) {
        let Some(draft) = self.draft.as_mut() else {
            return;
        };
        let Some(change) = draft.offer.defer(change) else {
            return;
        };
        if self.apply(doc, change)
            && let Some(draft) = self.draft.as_mut()
        {
            draft.offer.changed();
        }
    }

    /// `change` made to the open draft; answers whether it moved it.
    fn apply(&mut self, doc: Option<&Doc<ProfileProgram>>, change: Change) -> bool {
        let side = match (&change, doc) {
            (Change::Pick(pick), Some(doc)) => Some(if self.is_on_copy(doc, pick) {
                MateSide::On
            } else {
                MateSide::To
            }),
            _ => None,
        };
        let kind = self.kind;
        let Some(draft) = self.draft.as_mut() else {
            return false;
        };
        match change {
            Change::Pick(pick) => {
                let (Some(doc), Some(side)) = (doc, side) else {
                    return false;
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
            }
            Change::AddMate(kind) => {
                draft.rows.push(MateRow::fresh(kind));
                draft.active = draft.rows.len() - 1;
            }
            Change::AddWritten(mate) => {
                let Some(doc) = doc else {
                    return false;
                };
                draft.rows.push(MateRow::written(doc, None, &mate));
                draft.active = draft.rows.len() - 1;
            }
            Change::ChooseKind(at, kind) => {
                let Some(row) = draft.rows.get_mut(at) else {
                    return false;
                };
                if row.kind == kind {
                    return false;
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
            Change::Sense(at, flip) => {
                let Some(row) = draft.rows.get_mut(at) else {
                    return false;
                };
                row.flip = flip && senses(row.kind).is_some();
            }
            Change::Strike(at, struck) => {
                let Some(row) = draft.rows.get_mut(at) else {
                    return false;
                };
                row.struck = struck;
            }
            Change::StrikeReading(reading) => {
                for at in reading {
                    match at {
                        Reading::Mate(at) => {
                            if let Some(row) = draft.rows.get_mut(at) {
                                row.struck = true;
                            }
                        }
                        Reading::Value(id) if !draft.struck_values.contains(&id) => {
                            draft.struck_values.push(id);
                        }
                        Reading::Value(_) => {}
                    }
                }
            }
            Change::Type(coord, by) => {
                draft.typed.retain(|(c, _)| *c != coord);
                draft.typed.push((coord, by));
            }
            Change::Target(target) => {
                if draft.target == target {
                    return false;
                }
                draft.target = target;
            }
            Change::IntoAlone(_) => {
                unreachable!("made by `Self::make_deferred`, as a target change of its own")
            }
            Change::PutBack(aside) => {
                let SetAside {
                    rows,
                    typed,
                    struck_values,
                    active,
                    then: _,
                } = *aside;
                draft.rows = rows;
                draft.typed = typed;
                draft.struck_values = struck_values;
                draft.active = active.min(draft.rows.len().saturating_sub(1));
            }
        }
        true
    }

    /// **Put back what a target change of its own set aside**, where it
    /// did not land: the rows, numbers and struck values, then each
    /// change made after it. Answers whether anything was set aside.
    fn put_back(&mut self, doc: &Doc<ProfileProgram>) -> bool {
        let Some(aside) = self.draft.as_mut().and_then(|draft| draft.held_back.take()) else {
            return false;
        };
        let then = aside.then.clone();
        self.apply(Some(doc), Change::PutBack(Box::new(aside)));
        for change in then {
            self.apply(Some(doc), change);
        }
        if let Some(draft) = self.draft.as_mut() {
            draft.offer.changed();
        }
        true
    }

    /// This draft with nothing typed — what its mates alone propose.
    #[must_use]
    pub fn untyped(mut self) -> Self {
        if let Some(draft) = self.draft.as_mut() {
            draft.typed.clear();
        }
        self
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

    /// **Whether the draft differs from what it was loaded from**: its
    /// rows, a number typed, a value struck, its target, or what a target
    /// change of its own set aside — what letting go of it would lose
    /// ([`crate::offer::Offer::holds_changes`]).
    #[must_use]
    pub fn differs_from_base(&self) -> bool {
        self.draft.as_ref().is_some_and(|draft| {
            draft.differs_in_rows()
                || draft.target != draft.loaded_target
                || draft.held_back.is_some()
        })
    }

    /// **Whether letting the draft go loses anything** the document does
    /// not hold: a change still landing, changes waiting on it, or a
    /// draft that differs from its bundle — what an undo takes back first
    /// (`Tools::undo_pending`), as the profile editor's does.
    #[must_use]
    pub fn holds_changes(&self) -> bool {
        self.draft
            .as_ref()
            .is_some_and(|draft| draft.offer.holds_changes(self.differs_from_base()))
    }

    /// **Offer the draft to the document**, once a change was made since
    /// the last offer: its bundle ([`Self::proposal`] against the landed
    /// pair `doc`, `ev`) judged by `door`, the document's door
    /// (`DocSession::bundle_door`). Answers the one
    /// [`SessionOp::SetBundle`] that writes the whole draft when the door
    /// takes it, which the draft then waits on ([`Self::landed`]). A
    /// draft that writes no bundle yet, or that the door refuses, is held
    /// with its reason ([`Self::pending`]); one that writes nothing
    /// (`crate::session::bundle_writes_nothing`) answers nothing.
    pub fn offer(
        &mut self,
        doc: &Doc<ProfileProgram>,
        ev: &Evaluation<f64>,
        tol: Tol,
        door: impl FnOnce(&SessionOp) -> Result<(), Refusal>,
    ) -> Option<SessionOp> {
        if !self
            .draft
            .as_ref()
            .is_some_and(|draft| draft.offer.unsettled())
        {
            return None;
        }
        let proposed = self.proposal(doc, ev, tol);
        let placement = self.door().map(|door| door.placement);
        let formed = proposed
            .map_err(|error| Refusal::Draft(Box::new(error)))
            .map(|proposal| {
                let Some(draft) = self.draft.as_ref() else {
                    unreachable!("an unsettled draft is open")
                };
                let nothing = placement.is_some_and(|placement| {
                    crate::session::bundle_writes_nothing(
                        doc,
                        placement,
                        &draft.door.base,
                        &proposal.target,
                        &proposal.bundle,
                    )
                });
                (!nothing).then(|| proposal.op())
            })
            .and_then(|op| match op {
                Some(op) => door(&op).map(|()| Some(op)),
                None => Ok(None),
            });
        // A target change of its own that does not land puts back what it
        // set aside, whatever the offer answered: the whole draft is then
        // offered over the new target.
        if !matches!(formed, Ok(Some(_))) && self.put_back(doc) {
            return None;
        }
        let Some(draft) = self.draft.as_mut() else {
            unreachable!("an unsettled draft is open")
        };
        match formed {
            Ok(Some(op)) => {
                let mut deferred = Vec::new();
                if let Some(mut aside) = draft.held_back.take() {
                    let then = core::mem::take(&mut aside.then);
                    deferred.push(Change::PutBack(Box::new(aside)));
                    deferred.extend(then);
                }
                draft.offer = Offer::Landing {
                    op: Box::new(op.clone()),
                    deferred,
                };
                Some(op)
            }
            Ok(None) => {
                draft.offer = Offer::Quiet;
                None
            }
            Err(refusal) => {
                draft.offer = Offer::Pending(Held(Arc::new(refusal)));
                None
            }
        }
    }

    /// **Why the draft is held over the document**: the refusal of its
    /// last offer, while it still holds what was refused.
    #[must_use]
    pub fn pending(&self) -> Option<&Refusal> {
        self.draft
            .as_ref()
            .and_then(|draft| draft.offer.refused())
            .filter(|_| self.differs_from_base())
            .map(|held| &*held.0)
    }

    /// The op the draft's last change formed, while it is landing.
    #[must_use]
    pub fn landing(&self) -> Option<&SessionOp> {
        self.draft.as_ref().and_then(|draft| draft.offer.landing())
    }

    /// **Drop the held draft**: the editor shows the placement's bundle
    /// as `doc` holds it.
    pub fn discard(&mut self, doc: &Doc<ProfileProgram>) {
        if let Some(placement) = self.door().map(|door| door.placement) {
            self.reload(doc, placement);
        }
    }

    /// **The draft's change landed**: the editor loads the bundle `doc`
    /// now holds, the row picks went to kept, and every change made while
    /// it landed is made over it.
    pub fn landed(&mut self, doc: &Doc<ProfileProgram>) {
        let Some(draft) = self.draft.as_mut() else {
            return;
        };
        let deferred = match core::mem::replace(&mut draft.offer, Offer::Quiet) {
            Offer::Landing { deferred, .. } => deferred,
            _ => Vec::new(),
        };
        let (placement, active) = (draft.door.placement, draft.active);
        if !self.reload(doc, placement) {
            return;
        }
        if let Some(draft) = self.draft.as_mut() {
            draft.active = active.min(draft.rows.len().saturating_sub(1));
            draft.offer = Offer::Deferred(deferred);
        }
        self.make_deferred(doc);
    }

    /// **The draft's change was refused where it landed** (the document
    /// moved under it in the same batch, which the batch says): the
    /// changes made while it landed are made, and the draft waits to be
    /// offered again, so the door says its refusal over the document as
    /// it now is.
    pub fn refused(&mut self, doc: &Doc<ProfileProgram>) {
        let Some(draft) = self.draft.as_mut() else {
            return;
        };
        let deferred = match core::mem::replace(&mut draft.offer, Offer::Quiet) {
            Offer::Landing { deferred, .. } => deferred,
            _ => Vec::new(),
        };
        draft.offer = Offer::Deferred(deferred);
        self.make_deferred(doc);
        if let Some(draft) = self.draft.as_mut() {
            draft.offer.changed();
        }
    }

    /// The changes that waited on a landing, made now over `doc`.
    fn make_deferred(&mut self, doc: &Doc<ProfileProgram>) {
        let deferred = self
            .draft
            .as_mut()
            .map(|draft| draft.offer.take_deferred())
            .unwrap_or_default();
        let mut deferred = deferred.into_iter();
        while let Some(change) = deferred.next() {
            if let Change::IntoAlone(space) = change {
                // "Into <space>" waited on the change that landed: it is a
                // target change of its own over what that wrote, and the
                // changes made after it wait with what it sets aside.
                self.retarget_into(space);
                let rest: Vec<Change> = deferred.by_ref().collect();
                if let Some(draft) = self.draft.as_mut()
                    && !rest.is_empty()
                {
                    let aside = draft.held_back.get_or_insert_with(|| SetAside {
                        rows: draft.rows.clone(),
                        typed: draft.typed.clone(),
                        struck_values: draft.struck_values.clone(),
                        active: draft.active,
                        then: Vec::new(),
                    });
                    aside.then.extend(rest);
                }
                return;
            }
            self.make(Some(doc), change);
        }
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

    /// **The picks the tool holds** ([`crate::session::Holds`]): every
    /// seat's pick, row by row, `on` then `to`.
    pub(crate) fn held(&mut self) -> Vec<crate::session::select::Held<'_>> {
        self.draft
            .iter_mut()
            .flat_map(|draft| draft.rows.iter_mut())
            .flat_map(|row| [&mut row.on, &mut row.to])
            .flat_map(|side| [&mut side.first, &mut side.point])
            .filter_map(|seat| seat.as_mut())
            .map(|pick| crate::session::select::Held::One(pick))
            .collect()
    }

    /// **The line the panel shows for row `row`'s seats**: the seated
    /// tools' line (`seats::picks_line`), each seat a role, a side as
    /// the bundle wrote it said so.
    #[must_use]
    pub fn line(&self, row: usize, by: Speaker<'_>) -> String {
        let Some(mate) = self.rows().get(row) else {
            return crate::seats::picks_line(core::iter::empty::<(String, Option<String>)>());
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

    /// **Bring the draft up to the landed pair**: each held pick whose
    /// reference no longer resolves against the landed pair `doc`,
    /// `eval` empties its seat, and then a draft over a bundle its
    /// placement no longer holds in `committed`, the document the
    /// history is on, is loaded again (or dropped, with its placement):
    /// the landed pair may answer an older document while an edit
    /// evaluates, and the draft's bundle is the committed one's. Returns
    /// the typed events, the drops first.
    pub fn reconcile(
        &mut self,
        committed: &Doc<ProfileProgram>,
        (doc, eval): (&Doc<ProfileProgram>, &Evaluation<f64>),
    ) -> Vec<MateToolEvent> {
        let mut events = Vec::new();
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
        if !events.is_empty()
            && let Some(draft) = self.draft.as_mut()
            && draft.offer.landing().is_none()
        {
            draft.offer.changed();
        }
        // A change landing is judged when it has landed (`Self::landed`).
        if let Some(PlaceDoor { placement, base }) =
            self.door().filter(|_| self.landing().is_none())
        {
            let placement = *placement;
            let doc = committed;
            let held = crate::session::held_placement(doc, placement);
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
                    self.reload(doc, placement);
                    events.push(MateToolEvent::Reloaded {
                        placement: doc.spoken(placement),
                    });
                }
                Some(_) => {}
            }
        }
        events
    }

    /// **Row `at`'s mate**, its sides read off its picks or as the
    /// bundle wrote them, `to` in the row's sense.
    fn row_mate(
        doc: &Doc<ProfileProgram>,
        ev: &Evaluation<f64>,
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
                let (site, read) = match side {
                    MateSide::On => on_read(doc, Some(ev), pick),
                    MateSide::To => to_read(doc, Some(ev), pick),
                }
                .ok_or_else(|| MateToolError::MemberUnread {
                    side,
                    node: Self::spoken(doc, held, pick.node()),
                })?;
                let reading = MateReading::at_site(row.kind, seat, entity, ev, &site).ok_or(
                    MateToolError::KindDoesNotRead {
                        row: at,
                        side,
                        kind: row.kind,
                        entity,
                    },
                )?;
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

    /// **The editor's bundle, derived from the draft** against the landed
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
        if let Some(&(row, _)) = live.iter().find(|(_, row)| row.unread) {
            return Err(MateToolError::KindUnread { row });
        }
        let mut mates = live
            .iter()
            .map(|&(at, row)| Self::row_mate(doc, ev, at, row))
            .collect::<Result<Vec<_>, _>>()?;
        // A loaded value struck for reading another space is no longer
        // loaded: its freedom is measured afresh.
        let base: &[Constraint<Formula>] = &draft
            .door
            .base
            .constraints
            .iter()
            .filter(|c| !draft.struck_values.contains(&c.id))
            .cloned()
            .collect::<Vec<_>>();
        // The chart: the first live mate fixing a direction. While it is
        // the bundle's own, as loaded, its orientation, its slides and its
        // values keep their meaning; otherwise its orientation is the
        // snap's, and its constants are measured again.
        let chart = live
            .iter()
            .position(|(_, row)| fixes_one_direction(row.kind));
        let base_chart = base.iter().find_map(|c| {
            let mate = c.mate()?;
            editor_core::mate::pin::operand_kind(doc, &mate.on)
                .filter(|&kind| fixes_one_direction(kind))
                .map(|_| c.id)
        });
        let chart_kept = match chart {
            Some(k) => live[k].1.as_loaded() && live[k].1.loaded() == base_chart,
            None => base_chart.is_none(),
        };
        let placement = draft.door.placement;
        let written: Vec<Mate<Formula>> = mates.iter().map(|m| m.mate.clone()).collect();
        let snap =
            crate::session::snap::draft_snap(doc, ev, placement, &draft.target, &written, tol);
        if let (Some(k), Some(orient)) = (chart, snap.as_ref().and_then(|s| s.orient))
            && !chart_kept
        {
            mates[k].mate.orient = orient;
        }
        let written: Vec<Mate<Formula>> = mates.iter().map(|m| m.mate.clone()).collect();
        let loaded_slides = || {
            SlideAxes::of(base.iter().filter_map(|c| match c.value()?.coord {
                Coord::Slide(Some(axis)) => Some(axis),
                _ => None,
            }))
        };
        let along = if chart.is_some() && chart_kept {
            Some(loaded_slides())
        } else {
            snap.as_ref().and_then(|s| s.along)
        };
        // A loaded freedom that names its direction stays itself while
        // the mates leave it ([`Placed::freedoms_keeping`]).
        let loaded_coords: Vec<Coord> = base
            .iter()
            .filter_map(|c| c.value().map(|v| v.coord.clone()))
            .collect();
        let coords = Placed::freedoms_keeping(doc, &written, along, &loaded_coords);
        let measured = snap.as_ref().map_or_else(
            || vec![None; coords.len()],
            |snap| snap.read(chart.map(|k| written[k].orient), &coords, tol),
        );
        // Every value the draft held, typed or loaded: each lands on a
        // freedom the mates leave, or is said dropped.
        let loaded: Vec<(ConstraintId, &Value<Formula>)> = base
            .iter()
            .filter_map(|c| c.value().map(|v| (c.id, v)))
            .collect();
        let mut values = Vec::new();
        let mut landed: Vec<ConstraintId> = Vec::new();
        for (coord, measured) in coords.iter().zip(&measured) {
            if let Some((_, by)) = draft.typed.iter().find(|(c, _)| c == coord) {
                values.push(ValueRow {
                    coord: coord.clone(),
                    by: by.clone(),
                    from: ValueFrom::Typed,
                });
                continue;
            }
            let same = loaded
                .iter()
                .find(|(id, v)| v.coord == *coord && !landed.contains(id));
            if let Some(&(id, v)) = same
                && (chart_kept || measured.is_none() || !written_constant(doc, placement, id))
            {
                landed.push(id);
                values.push(ValueRow {
                    coord: coord.clone(),
                    by: v.by.clone(),
                    from: ValueFrom::Loaded(id),
                });
                continue;
            }
            if let Some(same) = same {
                // A constant whose chart changed is measured again in its
                // own place.
                landed.push(same.0);
            }
            let Some(by) = *measured else {
                // The mates' poses do not evaluate where the copy stands,
                // so this freedom reads no number: it is said, written by
                // the kernel's fill rule (the constant `0`,
                // [`Placed::of`]), and the verdict names the mate's fault.
                values.push(ValueRow {
                    coord: coord.clone(),
                    by: Formula::zero(coord.dimension()),
                    from: ValueFrom::Unmeasured,
                });
                continue;
            };
            // A loaded turn whose freedom goes is carried onto this one
            // only where the kernel states the two one rotation about one
            // line ([`editor_core::mate::spin_as_chain_turn`]), with the
            // exact quarter turns between their zeros.
            if let Some((id, carried)) = carried_turn(
                doc,
                ev,
                tol,
                coord.clone(),
                &coords,
                &loaded,
                &landed,
                snap.as_ref(),
                placement,
                base_chart,
                chart.map(|k| written[k].orient),
            )
            .transpose()?
            {
                landed.push(id);
                values.push(ValueRow {
                    coord: coord.clone(),
                    by: carried,
                    from: ValueFrom::Loaded(id),
                });
                continue;
            }
            values.push(ValueRow {
                coord: coord.clone(),
                by: number(by, coord.dimension()).ok_or_else(|| MateToolError::Unwritable {
                    coord: coord.clone(),
                    by,
                })?,
                from: ValueFrom::Measured,
            });
        }
        let gone = loaded
            .iter()
            .filter(|(id, _)| !landed.contains(id))
            .filter(|(_, v)| !coords.contains(&v.coord));
        let (retired, read): (Vec<_>, Vec<_>) =
            gone.partition(|(id, _)| written_constant(doc, placement, *id));
        let row = |&(id, v): &(ConstraintId, &Value<Formula>)| ValueRow {
            coord: v.coord.clone(),
            by: v.by.clone(),
            from: ValueFrom::Loaded(id),
        };
        let retired: Vec<ValueRow> = retired.iter().map(row).collect();
        let typed_gone = draft.typed.iter().filter(|(c, _)| !coords.contains(c));
        let dropped: Vec<ValueRow> = read
            .iter()
            .map(row)
            .chain(typed_gone.map(|(coord, by)| ValueRow {
                coord: coord.clone(),
                by: by.clone(),
                from: ValueFrom::Typed,
            }))
            .collect();
        // A loaded row keeps its id, a value carried or measured on a
        // freedom keeps the id of the value it replaces; every other
        // constraint is minted where the bundle lands.
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
        let mut used: Vec<ConstraintId> = Vec::new();
        for row in &values {
            let id = match row.from {
                ValueFrom::Loaded(id) => Some(id),
                ValueFrom::Typed | ValueFrom::Measured | ValueFrom::Unmeasured => loaded
                    .iter()
                    .find(|(id, v)| v.coord == row.coord && !used.contains(id))
                    .map(|(id, _)| *id),
            }
            .filter(|id| !used.contains(id))
            .unwrap_or_else(&mut fresh);
            used.push(id);
            bundle.push(Constraint::of_value(
                id,
                Value {
                    coord: row.coord.clone(),
                    by: row.by.clone(),
                },
            ));
        }
        Ok(MateProposal {
            door: draft.door.clone(),
            mates,
            values,
            dropped,
            retired,
            target: draft.target.clone(),
            bundle,
        })
    }
}
