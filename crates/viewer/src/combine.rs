//! **Combining bodies**: the modal layer-3 tools that turn body picks
//! into exactly one committed combining edit, and the lowering their
//! ops go through (GAUTH-4, Phase B).
//!
//! # Shape
//!
//! The revolve tool's shape, once per tool here, on the shared seat
//! machinery [`crate::seats`] carries: single-select stays ruled (GQ7), so
//! each tool holds its picks in tool state and consumes the ordinary
//! selection stream — a tree click is a node pick directly, a viewport
//! face or edge pick reaches the node whose DRAWN body the ray met
//! (`Selection::seat_node`). Everything before the commit is tool
//! state; the document transition is one [`SessionOp`], committed
//! through the session's ordinary commit door as one action
//! ([`crate::tools`] says which actions take more than one edit).
//!
//! The seat vocabulary, the pick rule, the survival step and the
//! id-reuse hazard it does not cover (issue #1384) are all
//! [`crate::seats`]'s, and are not restated here.
//!
//! Module kind: **vocabulary** — it names no driver type and no
//! `app`-only crate (`crates/viewer/README.md`, Module boundaries).

use pncad::document::AuthoredNode;
use pncad::document::{
    Doc, Evaluation, Formula, HeldNodes, Node, NodeStanding, Operand, ProfileProgram, RecipeNodeId,
    Said, Speaker, SpokenNode, held_by,
};
use pncad::geom_core::{Tol, Vec3};
use pncad::prelude::StableName;
use pncad::select::SplitHalf;
use pncad::topo::BooleanOp;

use crate::seats::{
    FaceRole, Port, Seat, SeatError, SeatEvent, Seats, pick_said, picks_line, seat_line,
};
use crate::session::refuse::one_body;
use crate::session::{BooleanSpec, FaceSelection, PartSelectSpec, PatternRuleSpec, SessionOp};
use crate::vocab::vocabulary;

/// **The boolean tool**: one operation choice and the body picks it
/// takes — any number for union and intersect, two seats for subtract —
/// committing one [`SessionOp::AddBoolean`].
///
/// **The operation is the tool's own state, not a form draft**, because
/// it decides what a pick DOES: a union or an intersect appends every
/// pick to its member list (a node picked twice is listed twice, which
/// glues), while a subtraction fills two role-typed seats on the
/// [`crate::seats`] rule — the body KEPT, then the body REMOVED, a
/// further pick replacing the removed one. The operand order is DATA
/// for subtract only, so only subtract names its picks by role.
///
/// **Changing the operation re-seats what is held**, in pick order: a
/// member list becomes the subtraction's two seats from its first two
/// members (any further members are let go, and the panel line shows
/// the result), and two seats become a member list of what they hold.
/// Nothing is re-read from the document, so a pick keeps the label it
/// was spoken with until the next respeak.
///
/// **A member drop closes up the list** (a member's place in a union is
/// its fold order, and the members after it keep theirs relative to
/// each other), where a seat drop empties its seat without promoting
/// the survivor ([`crate::seats`]).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BooleanTool {
    op: BooleanOp,
    held: BooleanPicks,
}

/// What the boolean tool holds, by the shape its operation takes. The
/// tool keeps the two in step: [`BooleanPicks::Pair`] exactly when its
/// operation is [`BooleanOp::Subtract`].
#[derive(Debug, Clone, PartialEq, Eq)]
enum BooleanPicks {
    /// A union's or an intersect's members, in pick order, each with
    /// the port it reads.
    Members(Vec<(SpokenNode, Port)>),
    /// A subtraction's two seats.
    Pair(Seats),
}

impl Default for BooleanTool {
    fn default() -> Self {
        Self::new()
    }
}

impl BooleanTool {
    /// A union tool holding nothing.
    pub const fn new() -> Self {
        Self {
            op: BooleanOp::Union,
            held: BooleanPicks::Members(Vec::new()),
        }
    }

    /// The operation the tool commits.
    pub fn operation(&self) -> BooleanOp {
        self.op
    }

    /// **Choose the operation**, re-seating what is held (type docs).
    pub fn set_operation(&mut self, op: BooleanOp) {
        let held = core::mem::replace(&mut self.held, BooleanPicks::Members(Vec::new()));
        self.held = match (held, op) {
            (BooleanPicks::Members(members), BooleanOp::Subtract) => {
                let mut first = members.into_iter();
                // A seat holds what a member held, port and all.
                BooleanPicks::Pair(Seats::holding(
                    // The subtraction's seats, kept then removed.
                    [Seat::SubtractFrom, Seat::SubtractTool],
                    [first.next(), first.next()],
                ))
            }
            (BooleanPicks::Pair(seats), BooleanOp::Union | BooleanOp::Intersect) => {
                BooleanPicks::Members(seats.spoken())
            }
            (held, _) => held,
        };
        self.op = op;
    }

    /// The held picks' nodes, in the node's operand order: the members,
    /// or the subtraction's filled seats (kept, then removed).
    pub fn picks(&self) -> Vec<RecipeNodeId> {
        match &self.held {
            BooleanPicks::Members(members) => members.iter().map(|(node, _)| node.id()).collect(),
            BooleanPicks::Pair(seats) => (0..2).filter_map(|i| seats.held(i)).collect(),
        }
    }

    /// Whether anything is held.
    pub fn is_empty(&self) -> bool {
        match &self.held {
            BooleanPicks::Members(members) => members.is_empty(),
            BooleanPicks::Pair(seats) => seats.is_empty(),
        }
    }

    /// **The panel's held-picks line**: a subtraction's two seats by
    /// role ([`crate::seats::seat_line`]), a member list as `member 1`,
    /// `member 2`, … in pick order — the same composition
    /// ([`crate::seats::picks_line`]), each pick as `doc` speaks it.
    pub fn line(&self, doc: &Doc<ProfileProgram>) -> String {
        match &self.held {
            BooleanPicks::Members(members) => {
                picks_line(members.iter().enumerate().map(|(i, (node, port))| {
                    (
                        format!("{} {}", Seat::Member.name(), i + 1),
                        Some(pick_said(doc, node.id(), *port)),
                    )
                }))
            }
            BooleanPicks::Pair(seats) => seat_line(seats, doc),
        }
    }

    /// Feed one node pick; `doc` routes it, and does not judge it. A
    /// pick on a copy seats the body it places.
    pub fn pick(&mut self, doc: &Doc<ProfileProgram>, node: RecipeNodeId) {
        self.pick_at(doc, node, None);
    }

    /// [`Self::pick`] of the output `body` a viewport pick hit: a
    /// split's half is read by its port, and a split picked in the tree
    /// waits for its half ([`Self::choose_half`]).
    pub fn pick_at(&mut self, doc: &Doc<ProfileProgram>, node: RecipeNodeId, body: Option<u32>) {
        match &mut self.held {
            // A pick on a copy is a pick of the body it places
            // ([`crate::world::seat_read`]), as in the two seats.
            BooleanPicks::Members(members) => {
                let (node, port) = crate::world::seat_read(doc, node, body);
                let port = Port::of_pick(doc, node, port, Seat::Member.wants());
                members.push((doc.spoken(node), port));
            }
            BooleanPicks::Pair(seats) => seats.pick_at(doc, node, body),
        }
    }

    /// **The half pick `i` reads**, when it holds a split picked alone:
    /// member `i` of a union or an intersect, seat `i` of a subtraction.
    pub fn choose_half(&mut self, i: usize, half: SplitHalf) {
        match &mut self.held {
            BooleanPicks::Members(members) => {
                if let Some((_, port)) = members.get_mut(i) {
                    *port = port.chosen(half);
                }
            }
            BooleanPicks::Pair(seats) => seats.choose_half(i, half),
        }
    }

    /// The indices of the picks still waiting for a half, in pick
    /// order.
    pub fn awaiting_half(&self) -> Vec<usize> {
        match &self.held {
            BooleanPicks::Members(members) => members
                .iter()
                .enumerate()
                .filter(|(_, (_, port))| *port == Port::HalfPending)
                .map(|(i, _)| i)
                .collect(),
            BooleanPicks::Pair(seats) => (0..2).filter(|&i| seats.awaits_half(i)).collect(),
        }
    }

    /// Drop every pick, keeping the operation.
    pub fn clear(&mut self) {
        match &mut self.held {
            BooleanPicks::Members(members) => members.clear(),
            BooleanPicks::Pair(seats) => seats.clear(),
        }
    }

    /// The survival step ([`crate::seats`]): a member whose node left
    /// the document leaves the list, a seat's pick empties its seat.
    pub fn reconcile(&mut self, doc: &Doc<ProfileProgram>) -> Vec<SeatEvent> {
        match &mut self.held {
            BooleanPicks::Members(members) => {
                let (kept, lost): (Vec<_>, Vec<_>) = core::mem::take(members)
                    .into_iter()
                    .partition(|(node, _)| doc.node(node.id()).is_some());
                *members = kept;
                lost.into_iter()
                    .map(|(node, _)| SeatEvent::PickLost {
                        seat: Seat::Member,
                        node,
                    })
                    .collect()
            }
            BooleanPicks::Pair(seats) => seats.reconcile(doc),
        }
    }

    /// The held picks spoken again from `doc` ([`Seats::respeak`]).
    pub fn respeak(&mut self, doc: &Doc<ProfileProgram>) {
        match &mut self.held {
            BooleanPicks::Members(members) => {
                for (node, _) in members.iter_mut() {
                    *node = node.respoken(doc);
                }
            }
            BooleanPicks::Pair(seats) => seats.respeak(doc),
        }
    }

    /// **What the tool would commit**: the operation over the held
    /// picks, in their order.
    ///
    /// # Errors
    ///
    /// [`SeatError::Empty`] naming [`Seat::Member`] while a union or an
    /// intersect holds no member, and naming the first empty seat while
    /// a subtraction lacks one; [`SeatError::HalfUnchosen`] while a
    /// pick holds a split whose half is not chosen. Node kinds refuse
    /// at the session door.
    pub fn spec(&self) -> Result<BooleanSpec, SeatError> {
        match &self.held {
            BooleanPicks::Members(members) if members.is_empty() => {
                Err(SeatError::Empty { seat: Seat::Member })
            }
            BooleanPicks::Members(members) => {
                if members.iter().any(|(_, port)| *port == Port::HalfPending) {
                    return Err(SeatError::HalfUnchosen { seat: Seat::Member });
                }
                let members = members
                    .iter()
                    .map(|(node, port)| port.read(node.id()))
                    .collect();
                Ok(if self.op == BooleanOp::Intersect {
                    BooleanSpec::Intersect(members)
                } else {
                    BooleanSpec::Union(members)
                })
            }
            BooleanPicks::Pair(seats) => Ok(BooleanSpec::Subtract {
                from: seats.require_read(0)?,
                tool: seats.require_read(1)?,
            }),
        }
    }

    /// **The one committed edit**: the session op that inserts the
    /// boolean node through the ordinary commit door.
    ///
    /// # Errors
    ///
    /// [`BooleanTool::spec`]'s.
    pub fn op(&self) -> Result<SessionOp, SeatError> {
        Ok(SessionOp::AddBoolean { spec: self.spec()? })
    }
}

/// **The split tool**: a body pick plus the cutting plane's fields,
/// committing one [`SessionOp::AddSplit`]. The plane is a point and a
/// normal in the target's own coordinates, read off its geometry by the
/// session door.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SplitTool {
    seats: Seats,
}

impl Default for SplitTool {
    fn default() -> Self {
        Self::new()
    }
}

impl SplitTool {
    /// A tool holding nothing.
    pub const fn new() -> Self {
        Self {
            seats: Seats::one(Seat::SplitTarget),
        }
    }

    /// The seats, roles and picks together — what the panel's line is
    /// composed from ([`crate::seats::seat_line`]).
    pub fn seats(&self) -> &Seats {
        &self.seats
    }

    /// The held target body.
    pub fn target(&self) -> Option<RecipeNodeId> {
        self.seats.held(0)
    }

    /// Feed one node pick — a second pick REPLACES the first, this tool
    /// having only the one seat. `doc` routes it, and does not judge it.
    /// A pick on a copy seats the body it places.
    pub fn pick(&mut self, doc: &Doc<ProfileProgram>, node: RecipeNodeId) {
        self.seats.pick(doc, node);
    }

    /// [`Self::pick`] of the output `body` a viewport pick hit.
    pub fn pick_at(&mut self, doc: &Doc<ProfileProgram>, node: RecipeNodeId, body: Option<u32>) {
        self.seats.pick_at(doc, node, body);
    }

    /// The half seat `i` reads, when it holds a split picked alone
    /// ([`Seats::choose_half`]).
    pub fn choose_half(&mut self, i: usize, half: SplitHalf) {
        self.seats.choose_half(i, half);
    }

    /// Empty the seat.
    pub fn clear(&mut self) {
        self.seats.clear();
    }

    /// The survival step ([`crate::seats`]).
    pub fn reconcile(&mut self, doc: &Doc<ProfileProgram>) -> Vec<SeatEvent> {
        self.seats.reconcile(doc)
    }

    /// The held picks spoken again from `doc` ([`Seats::respeak`]).
    pub fn respeak(&mut self, doc: &Doc<ProfileProgram>) {
        self.seats.respeak(doc);
    }

    /// **The one committed edit**: the split by the plane through
    /// `origin` with normal `normal`.
    ///
    /// # Errors
    ///
    /// [`SeatError::Empty`] until a body is picked.
    pub fn op(&self, origin: [Formula; 3], normal: [Formula; 3]) -> Result<SessionOp, SeatError> {
        Ok(SessionOp::AddSplit {
            target: self.seats.require_read(0)?,
            origin,
            normal,
        })
    }
}

/// **The mirror tool**: a body seat and the face whose plane the body
/// is reflected across, committing one [`SessionOp::AddMirror`].
///
/// The plane is a FACE pick, held as the mate tool holds its picks and
/// marked as they are (`crate::marks::Held`): a face is a name, so it
/// survives by resolving against the landed evaluation
/// ([`Self::reconcile_plane`]) and is re-seated by a rename, where the
/// body seat survives as every seat does ([`Seats::reconcile`]).
///
/// **The first face pick fills both**: the body it is on and its own
/// plane — a body mirrored across one of its own faces is the
/// symmetric part's case. A later face pick re-picks the plane alone, so
/// a body is mirrored across another body's face by picking that face
/// second; a tree pick re-picks the body.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MirrorTool {
    seats: Seats,
    plane: Option<FaceSelection>,
    /// The plane's node as the document last spoke it
    /// ([`Self::respeak`]).
    plane_said: Option<SpokenNode>,
}

impl Default for MirrorTool {
    fn default() -> Self {
        Self::new()
    }
}

impl MirrorTool {
    /// A tool holding nothing.
    pub const fn new() -> Self {
        Self {
            seats: Seats::one(Seat::MirrorBody),
            plane: None,
            plane_said: None,
        }
    }

    /// The body seat.
    pub fn seats(&self) -> &Seats {
        &self.seats
    }

    /// The held body.
    pub fn input(&self) -> Option<RecipeNodeId> {
        self.seats.held(0)
    }

    /// The held plane face.
    pub fn plane(&self) -> Option<&FaceSelection> {
        self.plane.as_ref()
    }

    /// Feed one node pick: it REPLACES the body.
    pub fn pick(&mut self, doc: &Doc<ProfileProgram>, node: RecipeNodeId) {
        self.seats.pick(doc, node);
    }

    /// **Feed one face pick**: the plane, and the body it is on while
    /// no body is held (the type's docs).
    pub fn pick_plane(&mut self, doc: &Doc<ProfileProgram>, face: FaceSelection) {
        if self.seats.is_empty() {
            self.seats.pick_at(doc, face.node, Some(face.body));
        }
        self.plane_said = Some(doc.spoken(plane_node(doc, &face)));
        self.plane = Some(face);
    }

    /// The half the body seat reads, when it holds a split picked
    /// alone ([`Seats::choose_half`]).
    pub fn choose_half(&mut self, i: usize, half: SplitHalf) {
        self.seats.choose_half(i, half);
    }

    /// Empty the seat and the plane.
    pub fn clear(&mut self) {
        self.seats.clear();
        self.plane = None;
        self.plane_said = None;
    }

    /// The body seat's survival step ([`crate::seats`]).
    pub fn reconcile(&mut self, doc: &Doc<ProfileProgram>) -> Vec<SeatEvent> {
        self.seats.reconcile(doc)
    }

    /// **The plane's survival step**: the held face re-read against the
    /// landed pair, as the mate tool re-reads its picks, and dropped
    /// when it no longer resolves.
    pub fn reconcile_plane(
        &mut self,
        doc: &Doc<ProfileProgram>,
        eval: &Evaluation<f64>,
    ) -> Option<MirrorEvent> {
        let face = self.plane.as_ref()?;
        let verdict = crate::tree::pick_resolution(doc, eval, face.node, face.body, &face.name);
        if crate::session::select::resolves(&verdict) {
            return None;
        }
        let face = self.plane.take()?;
        let node = self
            .plane_said
            .take()
            .unwrap_or_else(|| doc.spoken(plane_node(doc, &face)));
        Some(MirrorEvent::PlaneLost {
            node,
            resolution: Box::new(verdict),
        })
    }

    /// The held picks spoken again from `doc` ([`Seats::respeak`]).
    pub fn respeak(&mut self, doc: &Doc<ProfileProgram>) {
        self.seats.respeak(doc);
        crate::seats::respeak_each(core::slice::from_mut(&mut self.plane_said), doc);
    }

    /// **The panel's held-picks line**: the body seat as [`seat_line`]
    /// says it and the plane as the mate tool says a face pick.
    pub fn line(&self, doc: &Doc<ProfileProgram>) -> String {
        let body = self.seats.said(doc, 0);
        let plane = self
            .plane
            .as_ref()
            .map(|face| crate::matetool::face_of(&doc.spoken(plane_node(doc, face))));
        picks_line([
            (Seat::MirrorBody.name(), body),
            (FaceRole::MirrorPlane.name(), plane),
        ])
    }

    /// **The one committed edit**: the body mirrored across the held
    /// face's plane.
    ///
    /// # Errors
    ///
    /// [`SeatError::Empty`] until a body is picked, and naming the
    /// plane until a face is.
    pub fn op(&self) -> Result<SessionOp, SeatError> {
        let body = self.seats.require_read(0)?;
        let plane = self.plane.clone().ok_or(SeatError::NoFace {
            role: FaceRole::MirrorPlane,
        })?;
        Ok(SessionOp::AddMirror { body, plane })
    }
}

/// **The node a held plane is said by**: the body the face is on, as
/// a body seat says a pick on a copy ([`crate::world::on_body`]), so
/// one click is named by one node in both of the tool's items.
fn plane_node(doc: &Doc<ProfileProgram>, face: &FaceSelection) -> RecipeNodeId {
    crate::world::on_body(doc, face.node, face.body, &face.name).0
}

/// A typed mirror-tool event the chrome renders: the plane's drop. The
/// body seat's drop is a [`SeatEvent`].
#[derive(Debug)]
pub enum MirrorEvent {
    /// The held plane face no longer resolves against the landed
    /// evaluation; the tool dropped it.
    PlaneLost {
        /// The face's node by the last label the tool kept for it.
        node: SpokenNode,
        /// The resolution machinery's own verdict.
        resolution: Box<pncad::select::Resolution>,
    },
}

impl core::fmt::Display for MirrorEvent {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::PlaneLost { node, .. } => {
                crate::matetool::face_lost(f, FaceRole::MirrorPlane.name(), node)
            }
        }
    }
}

/// **A mirror node**: `body` reflected across the plane of the face
/// `face` of the body `on` reads, the face read as a plane mate's is
/// ([`crate::matetool::plane_of`]).
///
/// Total: whether `face` names a face of `on`, and whether that face is
/// planar, are the door's and the evaluation's to refuse.
pub fn mirror_node(body: Operand, on: Operand, face: StableName) -> AuthoredNode {
    Node::mirror(
        body,
        crate::matetool::plane_of(Operand::select(on, vec![face])),
    )
}

/// **The transform tool**: one body pick plus the placement fields,
/// committing one [`SessionOp::AddTransform`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransformTool {
    seats: Seats,
}

impl Default for TransformTool {
    fn default() -> Self {
        Self::new()
    }
}

impl TransformTool {
    /// A tool holding nothing.
    pub const fn new() -> Self {
        Self {
            seats: Seats::one(Seat::TransformBody),
        }
    }

    /// The seats, roles and picks together — what the panel's line is
    /// composed from ([`crate::seats::seat_line`]).
    pub fn seats(&self) -> &Seats {
        &self.seats
    }

    /// The held body.
    pub fn input(&self) -> Option<RecipeNodeId> {
        self.seats.held(0)
    }

    /// Feed one node pick — a second pick REPLACES the first, this tool
    /// having only the one seat. `doc` routes it, and does not judge it.
    pub fn pick(&mut self, doc: &Doc<ProfileProgram>, node: RecipeNodeId) {
        self.seats.pick(doc, node);
    }

    /// [`Self::pick`] of the output `body` a viewport pick hit.
    pub fn pick_at(&mut self, doc: &Doc<ProfileProgram>, node: RecipeNodeId, body: Option<u32>) {
        self.seats.pick_at(doc, node, body);
    }

    /// The half seat `i` reads, when it holds a split picked alone
    /// ([`Seats::choose_half`]).
    pub fn choose_half(&mut self, i: usize, half: SplitHalf) {
        self.seats.choose_half(i, half);
    }

    /// Empty the seat.
    pub fn clear(&mut self) {
        self.seats.clear();
    }

    /// The survival step ([`crate::seats`]).
    pub fn reconcile(&mut self, doc: &Doc<ProfileProgram>) -> Vec<SeatEvent> {
        self.seats.reconcile(doc)
    }

    /// The held picks spoken again from `doc` ([`Seats::respeak`]).
    pub fn respeak(&mut self, doc: &Doc<ProfileProgram>) {
        self.seats.respeak(doc);
    }

    /// **The one committed edit**: the rigid placement.
    ///
    /// # Errors
    ///
    /// [`SeatError::Empty`] until a body is picked.
    pub fn op(
        &self,
        translation: [Formula; 3],
        rotation_axis: [Formula; 3],
        rotation_angle: Formula,
    ) -> Result<SessionOp, SeatError> {
        Ok(SessionOp::AddTransform {
            input: self.seats.require_read(0)?,
            translation,
            rotation_axis,
            rotation_angle,
        })
    }
}

vocabulary! {
    /// **What a pattern's members come out as** — the pattern form's
    /// output choice (D10 `[repetition]`).
    ///
    /// A pattern is one placement whose values read an index: its copy
    /// is a family of members, which stay separate bodies, or which one
    /// union reading the family whole fuses into ONE body. The union's
    /// members are rigid images of one body, so it holds every pair its
    /// separation certificate separates apart unjudged, and judges the
    /// rest (DM4's pairwise pass): the choice is between two honest
    /// answers, not between a strict door and a lax one.
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
    pub enum PatternOutputChoice {
        /// N separate bodies, the family itself.
        #[default]
        Instances = "instances",
        /// ONE body, a union reading the family whole.
        Fused = "fused",
    }

    /// Both choices with their button labels — the chrome's radio row
    /// and a test that sweeps them.
    pub const ALL;
}

/// **The pattern tool**: a body pick, committing one
/// [`SessionOp::AddPattern`] or one [`SessionOp::AddFusedPattern`] (a
/// family, or the union of one); the
/// circular rule's axis is a point and a direction in the body's own
/// coordinates, read off its geometry by the session door.
///
/// **One tool, two ops**: the output choice ([`PatternOutputChoice`])
/// picks which op each door mints, because everything the tool holds is
/// the same either way — the same prototype seat, the same count and
/// the same rule fields.
///
/// **Two commit doors rather than one taking a rule value**: each rule
/// has fields the other lacks. The chrome's rule choice picks the door,
/// which is the same decision it was already making.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PatternTool {
    seats: Seats,
}

impl Default for PatternTool {
    fn default() -> Self {
        Self::new()
    }
}

impl PatternTool {
    /// A tool holding nothing.
    pub const fn new() -> Self {
        Self {
            seats: Seats::one(Seat::PatternBody),
        }
    }

    /// The seats, roles and picks together — what the panel's line is
    /// composed from ([`crate::seats::seat_line`]).
    pub fn seats(&self) -> &Seats {
        &self.seats
    }

    /// The held body.
    pub fn input(&self) -> Option<RecipeNodeId> {
        self.seats.held(0)
    }

    /// Feed one node pick; `doc` routes it, and does not judge it. A
    /// pick on a copy seats the body it places.
    pub fn pick(&mut self, doc: &Doc<ProfileProgram>, node: RecipeNodeId) {
        self.seats.pick(doc, node);
    }

    /// [`Self::pick`] of the output `body` a viewport pick hit.
    pub fn pick_at(&mut self, doc: &Doc<ProfileProgram>, node: RecipeNodeId, body: Option<u32>) {
        self.seats.pick_at(doc, node, body);
    }

    /// The half seat `i` reads, when it holds a split picked alone
    /// ([`Seats::choose_half`]).
    pub fn choose_half(&mut self, i: usize, half: SplitHalf) {
        self.seats.choose_half(i, half);
    }

    /// Empty the seat.
    pub fn clear(&mut self) {
        self.seats.clear();
    }

    /// The survival step ([`crate::seats`]).
    pub fn reconcile(&mut self, doc: &Doc<ProfileProgram>) -> Vec<SeatEvent> {
        self.seats.reconcile(doc)
    }

    /// The held picks spoken again from `doc` ([`Seats::respeak`]).
    pub fn respeak(&mut self, doc: &Doc<ProfileProgram>) {
        self.seats.respeak(doc);
    }

    /// **The one committed edit**, stepping along a direction.
    ///
    /// # Errors
    ///
    /// [`SeatError::Empty`] with no body picked. A count of zero is NOT
    /// judged here: it is the index's, and an index of no value is a
    /// family of no member.
    pub fn linear_op(
        &self,
        output: PatternOutputChoice,
        count: i64,
        direction: [Formula; 3],
        spacing: Formula,
    ) -> Result<SessionOp, SeatError> {
        Ok(pattern_op(
            output,
            self.seats.require_read(0)?,
            count,
            PatternRuleSpec::Linear { direction, spacing },
        ))
    }

    /// **The one committed edit**, stepping around the axis through
    /// `through` along `direction`.
    ///
    /// # Errors
    ///
    /// [`SeatError::Empty`] with no body picked.
    pub fn circular_op(
        &self,
        output: PatternOutputChoice,
        count: i64,
        through: [Formula; 3],
        direction: [Formula; 3],
        step: Formula,
    ) -> Result<SessionOp, SeatError> {
        let input = self.seats.require_read(0)?;
        let rule = PatternRuleSpec::Circular {
            through,
            direction,
            step,
        };
        Ok(pattern_op(output, input, count, rule))
    }
}

/// Which op a filled pattern form commits — the output choice's one
/// consequence, spelled once so the two rule doors cannot disagree
/// about it.
fn pattern_op(
    output: PatternOutputChoice,
    input: Operand,
    count: i64,
    rule: PatternRuleSpec,
) -> SessionOp {
    match output {
        PatternOutputChoice::Instances => SessionOp::AddPattern { input, count, rule },
        PatternOutputChoice::Fused => SessionOp::AddFusedPattern { input, count, rule },
    }
}

/// **A fresh index name for `doc`**: `k`, else the first `k1`, `k2`,
/// … it does not name.
///
/// # Panics
///
/// Never: the names are infinitely many.
pub fn fresh_index(doc: &Doc<ProfileProgram>) -> pncad::document::VarName {
    std::iter::once("k".to_owned())
        .chain((1..).map(|n| format!("k{n}")))
        .find(|name| doc.var_named(name).is_none())
        .and_then(|name| pncad::document::VarName::new(&name).ok())
        .unwrap_or_else(|| unreachable!("an unnamed index"))
}

/// **A row's slides**: each target axis whose component of `direction`
/// is written nonzero, stepped `spacing` times it.
///
/// # Errors
///
/// [`pncad::document::DimensionError`] where a component is no scalar
/// or `spacing` no length — the expression door's answer.
pub fn row_steps(
    direction: &[Formula; 3],
    spacing: &Formula,
) -> Result<Vec<(pncad::document::TargetAxis, Formula)>, pncad::document::DimensionError> {
    use pncad::document::TargetAxis as A;
    [A::X, A::Y, A::Z]
        .into_iter()
        .zip(direction)
        .filter(|(_, d)| !written_zero(d))
        .map(|(axis, d)| Ok((axis, Formula::mul(spacing.clone(), d.clone())?)))
        .collect()
}

/// **The target axis a ring's free slide is written on**: the last
/// of `z`, `y`, `x` whose component of the axis `direction` is written
/// nonzero, which the slide along the axis spans.
pub fn ring_slide_axis(direction: &[Formula; 3]) -> pncad::document::TargetAxis {
    use pncad::document::TargetAxis as A;
    [
        (A::Z, &direction[2]),
        (A::Y, &direction[1]),
        (A::X, &direction[0]),
    ]
    .into_iter()
    .find(|(_, d)| !written_zero(d))
    .map_or(A::Z, |(axis, _)| axis)
}

/// Whether `d` is written zero: the zero formula, or a literal `0`.
fn written_zero(d: &Formula) -> bool {
    d.is_zero() || d.literal_value() == Some(0.0)
}

/// **The part tool**: one pick of a family, committing one
/// [`SessionOp::AddPart`] of one of its members. A split's halves need
/// no tool: each is an output of the split, which a body seat reads.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PartTool {
    seats: Seats,
}

impl Default for PartTool {
    fn default() -> Self {
        Self::new()
    }
}

impl PartTool {
    /// A tool holding nothing.
    pub const fn new() -> Self {
        Self {
            seats: Seats::one(Seat::PartInstance),
        }
    }

    /// The seats, roles and picks together — what the panel's line is
    /// composed from ([`crate::seats::seat_line`]).
    pub fn seats(&self) -> &Seats {
        &self.seats
    }

    /// The held pattern, if one was picked.
    pub fn pattern(&self) -> Option<RecipeNodeId> {
        self.seats.held(0)
    }

    /// Feed one node pick; `doc` routes it, and does not judge it. A
    /// pick on a copy seats the body it places.
    pub fn pick(&mut self, doc: &Doc<ProfileProgram>, node: RecipeNodeId) {
        self.seats.pick(doc, node);
    }

    /// [`Self::pick`] of the output `body` a viewport pick hit.
    pub fn pick_at(&mut self, doc: &Doc<ProfileProgram>, node: RecipeNodeId, body: Option<u32>) {
        self.seats.pick_at(doc, node, body);
    }

    /// Empty the seat.
    pub fn clear(&mut self) {
        self.seats.clear();
    }

    /// The survival step ([`crate::seats`]).
    pub fn reconcile(&mut self, doc: &Doc<ProfileProgram>) -> Vec<SeatEvent> {
        self.seats.reconcile(doc)
    }

    /// The held picks spoken again from `doc` ([`Seats::respeak`]).
    pub fn respeak(&mut self, doc: &Doc<ProfileProgram>) {
        self.seats.respeak(doc);
    }

    /// **The one committed edit**, reading one member of a family.
    ///
    /// # Errors
    ///
    /// [`SeatError::Empty`] until a family is picked. An index outside
    /// the family's members is NOT judged here: it is a fact about the
    /// family's VALUE, and one past the end refuses typed at evaluation
    /// on the reader's own badge (`NodeErrorKind::MemberOutOfRange`).
    pub fn instance_op(&self, index: i64) -> Result<SessionOp, SeatError> {
        Ok(SessionOp::AddPart {
            of: self.seats.require(0)?,
            select: PartSelectSpec::Instance(index),
        })
    }
}

/// **The duplicate tool**: one body pick, committing one
/// [`SessionOp::Duplicate`].
///
/// One seat and no fields: the step after duplicating is to MOVE the
/// copy away, so a form asking where the copy should go first would be
/// the pattern form again under another name. Where the copy lands is
/// [`duplicate_step`]'s rule — along [`STEP_DIRECTION`], clear of the
/// original by at least [`DUPLICATE_GAP`] of its own width — and the
/// step lands in the translation slots of the transform the gesture
/// authors, editable in the property panel the moment the edit lands.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DuplicateTool {
    seats: Seats,
    /// The drawn copy the pick landed on, when it landed on one: the
    /// copy the duplicate is placed beside.
    beside: Option<RecipeNodeId>,
}

impl Default for DuplicateTool {
    fn default() -> Self {
        Self::new()
    }
}

impl DuplicateTool {
    /// A tool holding nothing.
    pub const fn new() -> Self {
        Self {
            seats: Seats::one(Seat::DuplicateBody),
            beside: None,
        }
    }

    /// The seats, roles and picks together — what the panel's line is
    /// composed from ([`crate::seats::seat_line`]).
    pub fn seats(&self) -> &Seats {
        &self.seats
    }

    /// The held body.
    pub fn input(&self) -> Option<RecipeNodeId> {
        self.seats.held(0)
    }

    /// Feed one node pick — a second pick REPLACES the first, this
    /// tool having only the one seat. `doc` routes it, and does not
    /// judge it.
    pub fn pick(&mut self, doc: &Doc<ProfileProgram>, node: RecipeNodeId) {
        self.pick_at(doc, node, None);
    }

    /// [`Self::pick`] of the output `body` a viewport pick hit. A pick
    /// on a drawn copy also holds that copy, which the duplicate is
    /// placed beside.
    pub fn pick_at(&mut self, doc: &Doc<ProfileProgram>, node: RecipeNodeId, body: Option<u32>) {
        self.seats.pick_at(doc, node, body);
        self.beside = crate::world::placed_var(doc, node).map(|_| node);
    }

    /// The drawn copy the held pick landed on, if it landed on one.
    pub fn beside(&self) -> Option<RecipeNodeId> {
        self.beside
    }

    /// The half seat `i` reads, when it holds a split picked alone
    /// ([`Seats::choose_half`]).
    pub fn choose_half(&mut self, i: usize, half: SplitHalf) {
        self.seats.choose_half(i, half);
    }

    /// Empty the seat.
    pub fn clear(&mut self) {
        self.seats.clear();
        self.beside = None;
    }

    /// The survival step ([`crate::seats`]). The held copy survives
    /// while it is still a drawn copy of the held body; otherwise the
    /// duplicate is placed as a pick of the body places it
    /// ([`SessionOp::Duplicate`]).
    pub fn reconcile(&mut self, doc: &Doc<ProfileProgram>) -> Vec<SeatEvent> {
        let events = self.seats.reconcile(doc);
        let held = self.seats.read(0);
        self.beside = self.beside.filter(|copy| {
            held.as_ref()
                .is_some_and(|read| crate::world::placements_of_read(doc, read).contains(copy))
        });
        events
    }

    /// The held picks spoken again from `doc` ([`Seats::respeak`]).
    pub fn respeak(&mut self, doc: &Doc<ProfileProgram>) {
        self.seats.respeak(doc);
    }

    /// **The one committed edit**.
    ///
    /// # Errors
    ///
    /// [`SeatError::Empty`] until a body is picked.
    pub fn op(&self) -> Result<SessionOp, SeatError> {
        Ok(SessionOp::Duplicate {
            input: self.seats.require_read(0)?,
            beside: self.beside,
        })
    }
}

/// **Which way a stepped copy goes by default**: +x of the space the
/// body is in, unnormalized as every direction slot in this vocabulary
/// is. A duplicate is placed at its original copy's pose, so it lands
/// along that copy's own +x.
///
/// ONE home for the two gestures that step a copy along a line: the
/// duplicate tool commits it, and the pattern form's direction field
/// opens on it (`Drafts::default`). The pattern form's SPACING is not
/// shared, because the two answer different questions — a pattern's
/// spacing is a number the person types, a duplicate's is measured off
/// the body ([`duplicate_step`]).
pub const STEP_DIRECTION: [f64; 3] = [1.0, 0.0, 0.0];

/// **How much clear space a duplicate leaves AT LEAST**, as a fraction
/// of the body's own width along [`STEP_DIRECTION`]: the copy's near
/// side lands at least this far past the original's far side, and
/// further for a body thin along the step ([`duplicate_step`] says
/// why).
///
/// A fraction rather than a length, so the gap scales with the part —
/// a millimetre gap beside a metre-long beam is invisible, and beside
/// a millimetre pin it is the whole view.
pub const DUPLICATE_GAP: f64 = 0.25;

/// The measuring tessellation's chord, as a fraction of the body's
/// FLOOR-mesh extent (`crate::scene::SCALE_PROBE_DELTA`'s mesh).
///
/// Fine enough that the conservative inflation it costs — twice the
/// chord, on the width — stays well under [`DUPLICATE_GAP`] for a body
/// about as wide along the step as it is across; coarse enough that
/// measuring a body is one cheap tessellation rather than a picture's
/// worth. For a body thin along the step the inflation is NOT small
/// against the width, and the copy lands further off than the gap
/// alone would put it ([`duplicate_step`]).
///
/// Public for [`crate::scene::SCALE_PROBE_DELTA`]'s reason: a row that
/// asserts a landed step against a body's closed form has to run the
/// rule at this number, and a literal copy in a suite goes stale
/// without the build noticing.
pub const MEASURE_CHORD: f64 = 1.0 / 64.0;

/// **Why a duplicate could not be placed** — every way the landed body
/// can fail to be one thing with a width.
#[derive(Debug)]
pub enum DuplicateFault {
    /// Nothing has landed, so there is no body to measure yet.
    NotLanded,
    /// The picture on screen answers an OLDER document than the one
    /// the duplicate would be committed to — an edit has not landed
    /// yet ([`crate::session::DocSession::busy`]). The landed value of
    /// an edited node is its value BEFORE the edit, so a step measured
    /// off it could be the old width, and after a document replacement
    /// the same id may name a different node altogether (issue #1384).
    Stale,
    /// The picture on screen — which answers the current document —
    /// holds no value for the input, and the input's standing, as the
    /// feature tree draws it ([`crate::tree::standing_as_drawn`]), says
    /// why and where the repair is.
    ///
    /// Its `through` may be a mate, which is not the DAG ancestor
    /// `NodeStanding` documents
    /// (`work/wire/kernel-standing-names-a-cluster-refused-node-as-its-own-failure`).
    NoValue {
        /// The input's standing, as the tree draws it.
        standing: NodeStanding,
        /// The nodes `standing` names, as the landed document held them
        /// ([`held_by`]).
        held: HeldNodes,
    },
    /// The input's VALUE is several bodies, and a duplicate copies one:
    /// a transform of them would be several bodies again, which no
    /// world placement reads.
    ///
    /// The body seat refuses a read of several bodies by its kind, so
    /// this is the value's own answer behind it: the door asks the
    /// value, which is the evaluator's own question.
    NotOneBody {
        /// The node picked, as the landed document held it.
        input: SpokenNode,
    },
    /// The input reads a split's half that holds no material: the
    /// split's plane misses the body on that side.
    EmptyHalf {
        /// The split, as the landed document held it.
        input: SpokenNode,
        /// The half read.
        half: SplitHalf,
    },
    /// The measuring tessellation refused.
    Unmeasured {
        /// The node picked, as the landed document held it.
        input: SpokenNode,
        /// The tessellator's own refusal.
        error: pncad::mesh::TessellateError,
    },
    /// The body's mesh has no extent to step by.
    NoExtent {
        /// The node picked, as the landed document held it.
        input: SpokenNode,
    },
    /// The copy the duplicate is to stand beside is not a drawn copy
    /// of the body duplicated.
    NotACopy {
        /// The copy named, as the landed document held it.
        copy: SpokenNode,
        /// The body duplicated, as the landed document held it.
        input: SpokenNode,
    },
}

impl core::fmt::Display for DuplicateFault {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::NotLanded => f.write_str(
                "the document has not evaluated yet, so there is no body to measure a copy's \
                 step off",
            ),
            Self::Stale => f.write_str(
                "the picture is older than the document — wait for the latest edit to evaluate, \
                 so the copy's step is measured off the body as it now is",
            ),
            Self::NoValue { standing, held } => write!(
                f,
                "there is no body to copy: {}",
                Said(standing, Speaker::held(held))
            ),
            Self::NotOneBody { input } => write!(
                f,
                "{input}'s value is several bodies; a duplicate copies ONE — project the one \
                 you mean first"
            ),
            Self::EmptyHalf { input, half } => write!(
                f,
                "{input}'s {} half holds no material: its plane misses the body on that side",
                half.name()
            ),
            Self::Unmeasured { input, error } => write!(
                f,
                "{input}'s body could not be measured for the copy's step: {error}"
            ),
            Self::NoExtent { input } => {
                write!(f, "{input}'s body has no width to step a copy by")
            }
            Self::NotACopy { copy, input } => write!(
                f,
                "{copy} is not a drawn copy of {input}, so there is no copy to place the \
                 duplicate beside"
            ),
        }
    }
}

impl core::error::Error for DuplicateFault {}

/// **Where a duplicate's copy lands**: the step, in metres along
/// [`STEP_DIRECTION`], that puts the copy's near side AT LEAST
/// [`DUPLICATE_GAP`] of the body's width past the original's far side —
/// so the two never touch, whatever the body's size.
///
/// **Read off `eval`, which the caller must hand in CURRENT** — an
/// evaluation of the document the duplicate will be committed to. The
/// session door refuses [`DuplicateFault::NotLanded`] and
/// [`DuplicateFault::Stale`] before calling this; a stale evaluation
/// would hand back an edited node's OLD value, and nothing here could
/// tell.
///
/// **Conservative, and why.** The width is measured on a tessellation
/// at chord δ. The step needs every point of the exact surfaces to lie
/// within a known distance of the mesh, and `pncad::mesh`'s crate docs
/// state the promise the other way round — each triangle within δ of
/// the surface — with an honest bound of δ + ε (+ rounding), ε being
/// the kernel tolerance its boundary vertices sit within. The direction
/// this needs holds too, and rests on the same facts: the mesh vertices
/// lie on the surfaces (within ε), and each face kind's certificate
/// bounds the gap between the surface and the linear interpolant
/// through them over the triangle's own parameter cell, which runs both
/// ways, while the cells cover the face. So the true width is at most
/// the mesh's plus 2(δ + ε), and the step adds that before the gap;
/// rounding is left to the gap, which exceeds it by many orders.
///
/// **So the gap is a floor, not the step's exact share.** δ is
/// [`MEASURE_CHORD`] of the body's floor-mesh DIAGONAL, which sets the
/// scale before any width is known; for a body thin along
/// [`STEP_DIRECTION`] the 2(δ + ε) margin can be several times the
/// width, and the copy lands further away than a quarter-width. Never
/// nearer.
///
/// # Errors
///
/// [`DuplicateFault::NoValue`], [`DuplicateFault::NotOneBody`],
/// [`DuplicateFault::Unmeasured`], [`DuplicateFault::NoExtent`].
pub fn duplicate_step(
    doc: &Doc<ProfileProgram>,
    eval: &Evaluation<f64>,
    input: &Operand,
    tol: Tol,
) -> Result<f64, DuplicateFault> {
    // The session door's kind gate ran first and refused a read naming
    // no output.
    let (node, port) = crate::world::read_output(doc, input)
        .unwrap_or_else(|| unreachable!("a body read the session admitted names an output"));
    let value = eval.usable(node).map_err(|standing| {
        let standing = crate::tree::standing_as_drawn(standing, eval);
        let held = held_by(&standing, doc);
        DuplicateFault::NoValue { standing, held }
    })?;
    let spoken = || doc.spoken(node);
    let body = match port {
        Some(port) => match value.payload.output_body(u32::from(port)) {
            Ok(body) => Some(body),
            // A port the value holds no body at is a split's empty half.
            Err(_) => match (&value.payload, SplitHalf::of_output_body(u32::from(port))) {
                (pncad::document::ValuePayload::Split { .. }, Some(half)) => {
                    return Err(DuplicateFault::EmptyHalf {
                        input: spoken(),
                        half,
                    });
                }
                _ => None,
            },
        },
        None => one_body(&value.payload),
    }
    .ok_or_else(|| DuplicateFault::NotOneBody { input: spoken() })?;
    let measured = |chord: f64| {
        pncad::mesh::tessellate(body.body(), chord, tol).map_err(|error| {
            DuplicateFault::Unmeasured {
                input: spoken(),
                error,
            }
        })
    };
    let floor = measured(crate::scene::SCALE_PROBE_DELTA)?;
    // The floor mesh's box diagonal: the body's size, read before a
    // chord can be chosen for it.
    let scale = Vec3::from_array(
        [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]]
            .map(|axis| width_along(&floor.positions, axis).unwrap_or(0.0)),
    )
    .norm();
    if !(scale.is_finite() && scale > 0.0) {
        return Err(DuplicateFault::NoExtent { input: spoken() });
    }
    let chord = scale * MEASURE_CHORD;
    let mesh = measured(chord)?;
    // The step is stated in the body's space; the body is measured in
    // its own frame, along that heading read into it.
    let heading = body
        .heading_in_own(Vec3::from_array(STEP_DIRECTION))
        .map_err(|_| DuplicateFault::NoExtent { input: spoken() })?
        .to_array();
    let width = width_along(&mesh.positions, heading)
        .filter(|w| w.is_finite() && *w > 0.0)
        .ok_or_else(|| DuplicateFault::NoExtent { input: spoken() })?;
    Ok((width + 2.0 * (chord + tol.eps())) * (1.0 + DUPLICATE_GAP))
}

impl DuplicateFault {
    /// This fault with its nodes spoken from `doc`, a later version of
    /// the document it was raised in
    /// ([`crate::session::Refusal::respoken`]).
    #[must_use]
    pub fn respoken(self, doc: &Doc<ProfileProgram>) -> Self {
        let again = |node: SpokenNode| node.respoken(doc);
        match self {
            Self::NoValue { standing, held } => Self::NoValue {
                standing,
                held: held.respoken(doc),
            },
            Self::NotOneBody { input } => Self::NotOneBody {
                input: again(input),
            },
            Self::EmptyHalf { input, half } => Self::EmptyHalf {
                input: again(input),
                half,
            },
            Self::Unmeasured { input, error } => Self::Unmeasured {
                input: again(input),
                error,
            },
            Self::NoExtent { input } => Self::NoExtent {
                input: again(input),
            },
            Self::NotACopy { copy, input } => Self::NotACopy {
                copy: again(copy),
                input: again(input),
            },
            unspoken @ (Self::NotLanded | Self::Stale) => unspoken,
        }
    }
}

/// How far `points` spread along `direction` (normalized here): the
/// largest projection less the smallest. `None` for no points or a
/// zero direction.
fn width_along(points: &[pncad::geom_core::Point3<f64>], direction: [f64; 3]) -> Option<f64> {
    let direction = Vec3::from_array(direction);
    let norm = direction.norm();
    if !(norm.is_finite() && norm > 0.0) {
        return None;
    }
    let mut along = points
        .iter()
        .map(|p| Vec3::new(p.x, p.y, p.z).dot(direction) / norm);
    let first = along.next()?;
    let (lo, hi) = along.fold((first, first), |(lo, hi), r| (lo.min(r), hi.max(r)));
    Some(hi - lo)
}

/// **The slides a duplicate's copy is moved by**: [`STEP_DIRECTION`]
/// stepped by `step` metres ([`duplicate_step`]'s answer), on the
/// original's own axes.
///
/// # Errors
///
/// [`pncad::document::DimensionError`] for a non-finite `step` — which
/// [`duplicate_step`] never answers — and it is a `Result` because
/// whether a number is authorable keeps ONE home, the expression door.
pub fn duplicate_moves(
    step: f64,
) -> Result<Vec<(pncad::document::Coord, Formula)>, pncad::document::DimensionError> {
    use pncad::document::{Coord, Dimension, TargetAxis as A};
    [A::X, A::Y, A::Z]
        .into_iter()
        .zip(STEP_DIRECTION)
        .filter(|&(_, d)| d != 0.0)
        .map(|(axis, d)| {
            Ok((
                Coord::Slide(axis),
                Formula::literal(d * step, Dimension::Length)?,
            ))
        })
        .collect()
}

/// **Where the drawn copy `copy` stands in its space**: its
/// placement's motion as `eval` solved it, the identity for a copy the
/// identity bundle places.
///
/// # Errors
///
/// [`DuplicateFault::NoValue`] for a copy `eval` holds no value for.
pub fn copy_motion(
    doc: &Doc<ProfileProgram>,
    eval: &Evaluation<f64>,
    copy: RecipeNodeId,
) -> Result<pncad::document::Frame, DuplicateFault> {
    let value = eval.usable(copy).map_err(|standing| {
        let standing = crate::tree::standing_as_drawn(standing, eval);
        let held = held_by(&standing, doc);
        DuplicateFault::NoValue { standing, held }
    })?;
    Ok(value
        .motion
        .map_or(pncad::document::Frame::IDENTITY, |motion| {
            pncad::document::Frame::from_affine(motion.to_affine())
        }))
}

/// **One member of the family `of`, read alone** (D10 `[repetition]`):
/// a copy of `of[index]` standing in the family's space at the
/// identity, its index the STRUCTURAL count `Formula::count` mints — an
/// exact integer, the structural/continuous split being typed rather
/// than emergent (spec D3).
///
/// Total: whether the index suits the family it reads is evaluation's
/// question, asked of authored and hand-written documents alike.
pub fn part_node(
    doc: &Doc<ProfileProgram>,
    of: RecipeNodeId,
    select: PartSelectSpec,
) -> AuthoredNode {
    let PartSelectSpec::Instance(index) = select;
    let target = doc.node_space(of).unwrap_or(pncad::document::VarId::WORLD);
    Node::place(
        pncad::document::BodyRead::indexed(of, vec![Formula::count(index)]),
        Operand::Var(target),
        pncad::document::stand().bundle(),
    )
}

/// **Whether a node, named alone, reads as a single body** — the
/// question every body seat asks, answered by the edit door's own
/// rule: the read [`Doc::read_of_node`] takes is one a body seat admits
/// (`SlotKind::admits`, the door's kind check).
///
/// A node named alone reads its one output (spec Q5): a split's two
/// sides, or a revolve's body beside its axis, are several, so the read
/// refuses and the seat names a port; a family's members are a list of
/// bodies. A
/// seat filled with one refuses at the door rather than after the edit
/// lands. The recipe's way of saying which of several is meant is
/// a read of one of its outputs (`crates/editor-core/REFERENCES.md`
/// DM3: a split's half by its port), or a read of one member of a
/// family; either is ONE `Body`.
///
/// A node this admits may still refuse downstream — an empty boolean
/// result is a typed success that is not a body, and `wire_sweep`
/// refuses every recipe-expressible sweep today (the curved-solid
/// frontier) — and that refusal is the node's own, arriving by poison
/// propagation. `combine_ops::the_body_seat_is_the_operand_doors_body_slot`
/// offers each kind to the real door and asserts the two agree.
///
/// **Not `product`'s "body-denoting"**, which is a WIDER set: the
/// product gather counts a family's members and a split's sides
/// among the bodies it collects, because collecting several is what it
/// does. This answers the narrower question a single-body operand seat
/// asks.
pub fn denotes_body(doc: &Doc<ProfileProgram>, node: RecipeNodeId) -> bool {
    doc.read_of_node(node)
        .and_then(|read| doc.var(read))
        .is_some_and(|var| {
            pncad::document::SlotKind::Is(pncad::document::VarKind::Body).admits(var)
        })
}
